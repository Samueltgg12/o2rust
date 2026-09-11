//! Windowed display: winit + glutin OpenGL front-end.
//!
//! Creates a GL context on a winit window, renders the GBE framebuffer to a
//! textured quad (letterboxed to the window aspect), and pumps keyboard/mouse
//! events into the emulator's PS/2 queues while draining the audio ring.

use std::num::NonZeroU32;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::{ApiPreference, DisplayBuilder, GlWindow};
use glow::HasContext;
use o2rust::system::Emulator;
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::window::{Window, WindowId};

use crate::audio::AudioOut;
use crate::input::{key_to_scan_bytes, HostInput};

/// Instructions executed per redraw (keeps the guest ~real-time at 60 fps).
const STEPS_PER_FRAME: u64 = 1_000_000;

/// Window title.
const WINDOW_TITLE: &str = "O2Rust — SGI O2 (IP32) Workstation";

/// Open the window, create the GL context, and run the event loop.
pub fn run(emulator: Emulator, audio: Option<AudioOut>) -> Result<()> {
    let event_loop = EventLoop::new().context("failed to create winit event loop")?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let window_attributes = Window::default_attributes()
        .with_title(WINDOW_TITLE)
        .with_inner_size(LogicalSize::new(960.0, 600.0));

    let (window, config) = DisplayBuilder::new()
        .with_preference(ApiPreference::PreferEgl)
        .with_window_attributes(Some(window_attributes))
        .build(
            &event_loop,
            ConfigTemplateBuilder::new().with_alpha_size(8),
            |mut configs| configs.next().expect("no OpenGL configs available"),
        )
        .map_err(|e| anyhow!("failed to create OpenGL display/window: {e}"))?;

    let window = Arc::new(match window {
        Some(w) => w,
        None => glutin_winit::finalize_window(
            &event_loop,
            Window::default_attributes().with_title(WINDOW_TITLE),
            &config,
        )
        .context("failed to create window")?,
    });

    // GL context (OpenGL 3.3 core; retry with driver default if rejected).
    let raw_window_handle = window.window_handle().ok().map(|h| h.as_raw());
    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
        .build(raw_window_handle);
    let not_current = match unsafe { config.display().create_context(&config, &context_attributes) }
    {
        Ok(ctx) => ctx,
        Err(_) => unsafe {
            config
                .display()
                .create_context(&config, &ContextAttributesBuilder::new().build(raw_window_handle))
                .context("failed to create GL context")?
        },
    };

    let surface_attributes = window.build_surface_attributes(Default::default())?;
    let surface = unsafe { config.display().create_window_surface(&config, &surface_attributes) }?;
    let context = not_current
        .make_current(&surface)
        .context("failed to make GL context current")?;

    let gl = unsafe {
        glow::Context::from_loader_function_cstr(|s| config.display().get_proc_address(s))
    };

    tracing::info!(
        "OpenGL {} ({}){}; display: {}",
        unsafe { gl.get_parameter_string(glow::VERSION) },
        unsafe { gl.get_parameter_string(glow::RENDERER) },
        unsafe { gl.get_parameter_string(glow::SHADING_LANGUAGE_VERSION) },
        config.display().version_string()
    );

    let renderer = GlRenderer::new(gl).context("failed to initialize OpenGL")?;

    // One-frame vsync where available; errors are non-fatal.
    let _ = surface.set_swap_interval(&context, SwapInterval::Wait(NonZeroU32::new(1).expect("1 is non-zero")));

    let mut app = WinApp {
        window,
        surface,
        context,
        renderer,
        emulator,
        _audio: audio,
        input: HostInput::default(),
        fb_buffer: Vec::new(),
        last_cursor: None,
    };

    event_loop.run_app(&mut app).context("event loop failed")?;
    Ok(())
}

/// Runs the emulator a frame and blits the GBE framebuffer.
struct WinApp {
    window: Arc<Window>,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    renderer: GlRenderer,
    emulator: Emulator,
    /// Held so the cpal stream keeps running; never read.
    _audio: Option<AudioOut>,
    input: HostInput,
    fb_buffer: Vec<u8>,
    last_cursor: Option<winit::dpi::PhysicalPosition<f64>>,
}

impl ApplicationHandler for WinApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if id != self.window.id() {
            return;
        }
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(_size) => {
                self.window.resize_surface(&self.surface, &self.context);
                self.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                self.redraw(&self.window.inner_size());
            }
            WindowEvent::CursorMoved { position, .. } => {
                let dx = self
                    .last_cursor
                    .map(|last| position.x - last.x)
                    .unwrap_or(0.0);
                let dy = self
                    .last_cursor
                    .map(|last| position.y - last.y)
                    .unwrap_or(0.0);
                self.last_cursor = Some(position);
                if let Some(pkt) = self.input.mouse.feed_motion(dx * 0.25, dy * 0.25) {
                    self.emulator.push_ms_packet(pkt);
                }
            }
            WindowEvent::CursorLeft { .. } => {
                self.last_cursor = None;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if self.input.mouse.set_button(button, state == ElementState::Pressed) {
                    if let Some(pkt) = self.input.mouse.feed_motion(0.0, 0.0) {
                        self.emulator.push_ms_packet(pkt);
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.key_event(event);
            }
            WindowEvent::Focused(false) => {
                self.emulator.flush_input();
                self.last_cursor = None;
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Continuous animation: request a redraw every poll iteration.
        self.window.request_redraw();
    }
}

impl WinApp {
    fn key_event(&mut self, event: KeyEvent) {
        let code = match event.physical_key {
            PhysicalKey::Code(code) => code,
            _ => return,
        };
        // PS/2 autorepeat: send make again for presses, break on release.
        let mut bytes = Vec::with_capacity(4);
        key_to_scan_bytes(code, event.state == ElementState::Pressed, &mut bytes);
        for b in bytes {
            self.emulator.push_kbd_byte(b);
        }
    }

    fn redraw(&mut self, win_size: &winit::dpi::PhysicalSize<u32>) {
        // Advance guest emulation.
        self.emulator.run(STEPS_PER_FRAME);

        // Render the GBE framebuffer (linear RGBA8, big-endian from the ASIC).
        let fb_w = self.emulator.framebuffer_width();
        let fb_h = self.emulator.framebuffer_height();
        let want = fb_w * fb_h * 4;
        if want > self.fb_buffer.len() {
            self.fb_buffer.resize(want, 0);
        }
        let pixels = self.emulator.render_framebuffer(&mut self.fb_buffer);
        let data = if pixels > 0 {
            Some(&self.fb_buffer[..pixels * 4])
        } else {
            None
        };

        self.renderer.render(
            data,
            fb_w as u32,
            fb_h as u32,
            win_size.width,
            win_size.height,
        );

        if let Err(e) = self.surface.swap_buffers(&self.context) {
            tracing::warn!("swap_buffers failed: {e}");
        }
    }
}

/// Simple textured-quad blitter for the framebuffer.
struct GlRenderer {
    gl: glow::Context,
    program: glow::Program,
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    texture: glow::Texture,
    tex_w: u32,
    tex_h: u32,
}

const VERT_SRC: &str = r#"#version 330 core
layout (location = 0) in vec2 a_pos;
layout (location = 1) in vec2 a_uv;
out vec2 v_uv;
void main() {
    v_uv = a_uv;
    gl_Position = vec4(a_pos, 0.0, 1.0);
}
"#;

const FRAG_SRC: &str = r#"#version 330 core
in vec2 v_uv;
out vec4 o_color;
uniform sampler2D u_tex;
void main() {
    o_color = texture(u_tex, v_uv);
}
"#;

impl GlRenderer {
    fn new(gl: glow::Context) -> Result<Self> {
        let vs = compile_shader(&gl, glow::VERTEX_SHADER, VERT_SRC)?;
        let fs = compile_shader(&gl, glow::FRAGMENT_SHADER, FRAG_SRC)?;

        let program = unsafe { gl.create_program().map_err(anyhow::Error::msg)? };
        unsafe {
            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(anyhow!("program link failed: {}", gl.get_program_info_log(program)));
            }
            gl.delete_shader(vs);
            gl.delete_shader(fs);
        }

        let vao = unsafe { gl.create_vertex_array().map_err(anyhow::Error::msg)? };
        let vbo = unsafe { gl.create_buffer().map_err(anyhow::Error::msg)? };
        let texture = unsafe { gl.create_texture().map_err(anyhow::Error::msg)? };

        unsafe {
            // Static attribute layout: position (2 × f32) + uv (2 × f32).
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 16, 0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 16, 8);

            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        }

        Ok(Self {
            gl,
            program,
            vao,
            vbo,
            texture,
            tex_w: 0,
            tex_h: 0,
        })
    }

    /// Upload `data` (`fb_w × fb_h` RGBA8 pixels) and draw it letterboxed into
    /// a `win_w × win_h` viewport. `data == None` clears to black.
    fn render(
        &mut self,
        data: Option<&[u8]>,
        fb_w: u32,
        fb_h: u32,
        win_w: u32,
        win_h: u32,
    ) {
        let gl = &self.gl;
        unsafe {
            gl.viewport(0, 0, win_w as i32, win_h as i32);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            let Some(data) = data else {
                return; // nothing to display yet (firmware hasn't set a mode)
            };

            // Upload (or resize) the framebuffer texture.
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
            if fb_w != self.tex_w || fb_h != self.tex_h {
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA8 as i32,
                    fb_w as i32,
                    fb_h as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    glow::PixelUnpackData::Slice(Some(data)),
                );
                self.tex_w = fb_w;
                self.tex_h = fb_h;
            } else {
                gl.tex_sub_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    0,
                    0,
                    fb_w as i32,
                    fb_h as i32,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    glow::PixelUnpackData::Slice(Some(data)),
                );
            }

            // Letterbox quad so the video aspect is preserved.
            let (sx, sy) = letterbox_scale(fb_w, fb_h, win_w, win_h);
            let verts: [f32; 16] = [
                -sx, -sy, 0.0, 0.0,
                sx, -sy, 1.0, 0.0,
                sx, sy, 1.0, 1.0,
                -sx, sy, 0.0, 1.0,
            ];
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                u8_slice(&verts),
                glow::DYNAMIC_DRAW,
            );

            gl.use_program(Some(self.program));
            gl.bind_vertex_array(Some(self.vao));
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            gl.bind_vertex_array(None);
            gl.use_program(None);
        }
    }
}

fn compile_shader(gl: &glow::Context, ty: u32, src: &str) -> Result<glow::Shader> {
    let shader = unsafe { gl.create_shader(ty).map_err(anyhow::Error::msg)? };
    unsafe {
        gl.shader_source(shader, src);
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
            return Err(anyhow!(
                "shader compile failed: {}",
                gl.get_shader_info_log(shader)
            ));
        }
    }
    Ok(shader)
}

/// Scale factors for a letterboxed quad given source and window aspect ratios.
fn letterbox_scale(fb_w: u32, fb_h: u32, win_w: u32, win_h: u32) -> (f32, f32) {
    if fb_w == 0 || fb_h == 0 || win_w == 0 || win_h == 0 {
        return (1.0, 1.0);
    }
    let fb = fb_w as f32 / fb_h as f32;
    let win = win_w as f32 / win_h as f32;
    if win > fb {
        // Window is wider relative to the image → bars on the sides.
        (fb / win, 1.0)
    } else {
        // Window is taller relative to the image → bars top/bottom.
        (1.0, win / fb)
    }
}

/// Reinterpret a `&[f32]` byte slice without copying (for glow uploads).
fn u8_slice(s: &[f32]) -> &[u8] {
    // SAFETY: `[f32]` has no padding bytes; casting preserves the pointer/len.
    unsafe { std::slice::from_raw_parts(s.as_ptr() as *const u8, s.len() * 4) }
}