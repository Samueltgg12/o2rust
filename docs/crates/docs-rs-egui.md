egui: an easy-to-use GUI in pure Rust!

Try the live web demo: https://www.egui.rs/#demo. Read more about egui at https://github.com/emilk/egui.

egui is in heavy development, with each new version having breaking changes. You need to have rust 1.95.0 or later to use egui.

To quickly get started with egui, you can take a look at eframe_template which uses eframe.

To create a GUI using egui you first need a Context (by convention referred to by ctx). Then you add a Window or a Panel to get a Ui, which is what you’ll be using to add all the buttons and labels that you need.
Feature flags

    bytemuck — bytemuck enables you to cast epaint::Vertex, emath::Vec2 etc to &[u8].

    callstack — Show a debug-ui on hover including the stacktrace to the hovered item. This is very useful in finding the code that creates a part of the UI. Does not work on web.

    cint — cint enables interoperability with other color libraries.

    color-hex — Enable the hex_color macro.

    default_fonts (enabled by default) — If set, egui will use include_bytes! to bundle some fonts. If you plan on specifying your own fonts you may disable this feature.

    mint — mint enables interoperability with other math libraries such as glam and nalgebra.

    persistence — Enable persistence of memory (window positions etc).

    rayon — Enable parallel tessellation using rayon.

    This can help performance for graphics-intense applications.

    serde — Allow serialization using serde.

    unity — Change Vertex layout to be compatible with unity

    _override_unity — Override and disable the unity feature This exists, so that when testing with –all-features, snapshots render correctly.

Optional dependencies

    document-features — Enable this when generating docs.

Using egui

To see what is possible to build with egui you can check out the online demo at https://www.egui.rs/#demo.

If you like the “learning by doing” approach, clone https://github.com/emilk/eframe_template and get started using egui right away.
A simple example

Here is a simple counter that can be incremented and decremented using two buttons:

fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui| {
        if ui.button("−").clicked() {
            *counter -= 1;
        }
        ui.label(counter.to_string());
        if ui.button("+").clicked() {
            *counter += 1;
        }
    });
}

In some GUI frameworks this would require defining multiple types and functions with callbacks or message handlers, but thanks to egui being immediate mode everything is one self-contained function!
Quick start

ui.label("This is a label");
ui.hyperlink("https://github.com/emilk/egui");
ui.text_edit_singleline(&mut my_string);
if ui.button("Click me").clicked() { }
ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0));
ui.add(egui::DragValue::new(&mut my_f32));

ui.checkbox(&mut my_boolean, "Checkbox");

#[derive(PartialEq)]
enum Enum { First, Second, Third }
ui.horizontal(|ui| {
    ui.radio_value(&mut my_enum, Enum::First, "First");
    ui.radio_value(&mut my_enum, Enum::Second, "Second");
    ui.radio_value(&mut my_enum, Enum::Third, "Third");
});

ui.separator();

ui.image((my_image, egui::Vec2::new(640.0, 480.0)));

ui.collapsing("Click to see what is hidden!", |ui| {
    ui.label("Not much, as it turns out");
});

Viewports

Some egui backends support multiple viewports, which is what egui calls the native OS windows it resides in. See crate::viewport for more information.
Coordinate system

The left-top corner of the screen is (0.0, 0.0), with X increasing to the right and Y increasing downwards.

egui uses logical points as its coordinate system. Those related to physical pixels by the pixels_per_point scale factor. For example, a high-dpi screen can have pixels_per_point = 2.0, meaning there are two physical screen pixels for each logical point.

Angles are in radians, and are measured clockwise from the X-axis, which has angle=0.
Integrating with egui

Most likely you are using an existing egui backend/integration such as eframe, bevy_egui, or egui-miniquad, but if you want to integrate egui into a new game engine or graphics backend, this is the section for you.

You need to collect RawInput and handle FullOutput. The basic structure is this:

let mut ctx = egui::Context::default();

// Game loop:
loop {
    let raw_input: egui::RawInput = gather_input();

    let full_output = ctx.run_ui(raw_input, |ui| {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Hello world!");
            if ui.button("Click me").clicked() {
                // take some action here
            }
        });
    });
    handle_platform_output(full_output.platform_output);
    let clipped_primitives = ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
    paint(full_output.textures_delta, clipped_primitives);
}

For a reference OpenGL renderer, see the egui_glow painter.
Debugging your renderer
Things look jagged

    Turn off backface culling.

My text is blurry

    Make sure you set the proper pixels_per_point in the input to egui.
    Make sure the texture sampler is not off by half a pixel. Try nearest-neighbor sampler to check.

My windows are too transparent or too dark

    egui uses premultiplied alpha, so make sure your blending function is (ONE, ONE_MINUS_SRC_ALPHA).
    Make sure your texture sampler is clamped (GL_CLAMP_TO_EDGE).
    egui prefers gamma color spaces for all blending so:
        Do NOT use an sRGBA-aware texture (NOT GL_SRGB8_ALPHA8).
        Multiply texture and vertex colors in gamma space
        Turn OFF sRGBA/gamma framebuffer (NO GL_FRAMEBUFFER_SRGB).

Understanding immediate mode

egui is an immediate mode GUI library.

Immediate mode has its roots in gaming, where everything on the screen is painted at the display refresh rate, i.e. at 60+ frames per second. In immediate mode GUIs, the entire interface is laid out and painted at the same high rate. This makes immediate mode GUIs especially well suited for highly interactive applications.

It is useful to fully grok what “immediate mode” implies.

Here is an example to illustrate it:

if ui.button("click me").clicked() {
    take_action()
}

This code is being executed each frame at maybe 60 frames per second. Each frame egui does these things:

    lays out the letters click me in order to figure out the size of the button
    decides where on screen to place the button
    check if the mouse is hovering or clicking that location
    choose button colors based on if it is being hovered or clicked
    add a Shape::Rect and Shape::Text to the list of shapes to be painted later this frame
    return a Response with the clicked member so the user can check for interactions

There is no button being created and stored somewhere. The only output of this call is some colored shapes, and a Response.

Similarly, consider this code:

ui.add(egui::Slider::new(&mut value, 0.0..=100.0).text("My value"));

Here egui will read value (an f32) to display the slider, then look if the mouse is dragging the slider and if so change the value. Note that egui does not store the slider value for you - it only displays the current value, and changes it by how much the slider has been dragged in the previous few milliseconds. This means it is responsibility of the egui user to store the state (value) so that it persists between frames.

It can be useful to read the code for the toggle switch example widget to get a better understanding of how egui works: https://github.com/emilk/egui/blob/main/crates/egui_demo_lib/src/demo/toggle_switch.rs.

Read more about the pros and cons of immediate mode at https://github.com/emilk/egui#why-immediate-mode.
Multi-pass immediate mode

By default, egui usually only does one pass for each rendered frame. However, egui supports multi-pass immediate mode. Another pass can be requested with Context::request_discard.

This is used by some widgets to cover up “first-frame jitters”. For instance, the Grid needs to know the width of all columns before it can properly place the widgets. But it cannot know the width of widgets to come. So it stores the max widths of previous frames and uses that. This means the first time a Grid is shown it will guess the widths of the columns, and will usually guess wrong. This means the contents of the grid will be wrong for one frame, before settling to the correct places. Therefore Grid calls Context::request_discard when it is first shown, so the wrong placement is never visible to the end user.

This is an example of a form of multi-pass immediate mode, where earlier passes are used for sizing, and later passes for layout.

See Context::request_discard and Options::max_passes for more.
Misc
How widgets works

if ui.button("click me").clicked() { take_action() }

is short for

let button = egui::Button::new("click me");
if ui.add(button).clicked() { take_action() }

which is short for

let button = egui::Button::new("click me");
let response = button.ui(ui);
if response.clicked() { take_action() }

Button uses the builder pattern to create the data required to show it. The Button is then discarded.

Button implements trait Widget, which looks like this:

pub trait Widget {
    /// Allocate space, interact, paint, and return a [`Response`].
    fn ui(self, ui: &mut Ui) -> Response;
}

Widget interaction

Each widget has a Sense, which defines whether or not the widget is sensitive to clicking and/or drags.

For instance, a Button only has a Sense::click (by default). This means if you drag a button it will not respond with Response::dragged. Instead, the drag will continue through the button to the first widget behind it that is sensitive to dragging, which for instance could be a ScrollArea. This lets you scroll by dragging a scroll area (important on touch screens), just as long as you don’t drag on a widget that is sensitive to drags (e.g. a Slider).

When widgets overlap it is the last added one that is considered to be on top and which will get input priority.

The widget interaction logic is run at the start of each frame, based on the output from the previous frame. This means that when a new widget shows up you cannot click it in the same frame (i.e. in the same fraction of a second), but unless the user is spider-man, they wouldn’t be fast enough to do so anyways.

By running the interaction code early, egui can actually tell you if a widget is being interacted with before you add it, as long as you know its Id before-hand (e.g. using Ui::next_auto_id), by calling Context::read_response. This can be useful in some circumstances in order to style a widget, or to respond to interactions before adding the widget (perhaps on top of other widgets).
Auto-sizing panels and windows

In egui, all panels and windows auto-shrink to fit the content. If the window or panel is also resizable, this can lead to a weird behavior where you can drag the edge of the panel/window to make it larger, and when you release the panel/window shrinks again. This is an artifact of immediate mode, and here are some alternatives on how to avoid it:

    Turn off resizing with Window::resizable, Panel::resizable.
    Wrap your panel contents in a ScrollArea, or use Window::vscroll and Window::hscroll.
    Use a justified layout:

ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
    ui.button("I am becoming wider as needed");
});

    Fill in extra space with emptiness:

ui.allocate_space(ui.available_size()); // put this LAST in your panel/window code

Sizes

You can control the size of widgets using Ui::add_sized.

ui.add_sized([40.0, 20.0], egui::DragValue::new(&mut my_value));

Code snippets

// Miscellaneous tips and tricks

ui.horizontal_wrapped(|ui| {
    ui.spacing_mut().item_spacing.x = 0.0; // remove spacing between widgets
    // `radio_value` also works for enums, integers, and more.
    ui.radio_value(&mut some_bool, false, "Off");
    ui.radio_value(&mut some_bool, true, "On");
});

ui.group(|ui| {
    ui.label("Within a frame");
    ui.set_min_height(200.0);
});

// A `scope` creates a temporary [`Ui`] in which you can change settings:
ui.scope(|ui| {
    ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
    ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);

    ui.label("This text will be red, monospace, and won't wrap to a new line");
}); // the temporary settings are reverted here

Installing additional fonts

The default egui fonts only support latin and cryllic characters, and some emojis. To use egui with e.g. asian characters you need to install your own font (.ttf or .otf) using Context::set_fonts.
Instrumentation

This crate supports using the profiling crate for instrumentation. You can enable features on the profiling crates in your application to add instrumentation for all crates that support it, including egui. See the profiling crate docs for more information.

[dependencies]
profiling = "1.0"
[features]
profile-with-puffin = ["profiling/profile-with-puffin"]

Custom allocator

egui apps can run significantly (~20%) faster by using a custom allocator, like mimalloc or talc.
Re-exports

pub use self::containers::menu::MenuBar;
pub use self::layers::LayerId;
pub use self::layers::Order;
pub use self::load::SizeHint;
pub use self::plugin::Plugin;
pub use self::response::InnerResponse;
pub use self::response::Response;
pub use self::style::FontSelection;
pub use self::style::Spacing;
pub use self::style::Style;
pub use self::style::TextStyle;
pub use self::style::Visuals;
pub use self::widget_text::RichText;
pub use self::widget_text::WidgetText;
pub use accesskit;
pub use epaint;
pub use epaint::ecolor;
pub use epaint::emath;
pub use self::containers::*;
pub use self::viewport::*;
pub use self::widgets::*;

Modules

cache
    Caches for preventing the same value from being recomputed every frame.
containers
    Containers are pieces of the UI which wraps other pieces of UI. Examples: Window, ScrollArea, Resize, Panel, etc.
debug_text
    This is an example of how to create a plugin for egui.
gui_zoom
    Helpers for zooming the whole GUI of an app (changing Context::pixels_per_point).
introspection
    Showing UI:s for egui/epaint types.
layers
    Handles paint layers, i.e. how things are sometimes painted behind or in front of other things.
load
    Image loading
mutex
    Wrappers around parking_lot locks, with a simple deadlock detection mechanism.
os
output
    All the data egui returns to the backend at the end of each frame.
plugin
response
special_emojis
    The default egui fonts supports around 1216 emojis in total. Here are some of the most useful: ∞⊗⎗⎘⎙⏏⏴⏵⏶⏷ ⏩⏪⏭⏮⏸⏹⏺■▶📾🔀🔁🔃 ☀☁★☆☐☑☜☝☞☟⛃⛶✔ ↺↻⟲⟳⬅➡⬆⬇⬈⬉⬊⬋⬌⬍⮨⮩⮪⮫ ♡ 📅📆 📈📉📊 📋📌📎📤📥🔆 🔈🔉🔊🔍🔎🔗🔘 🕓🖧🖩🖮🖱🖴🖵🖼🗀🗁🗋🗐🗑🗙🚫❓
style
    egui theme (spacing, colors, etc).
text
text_selection
    Helpers regarding text selection for labels and text edit.
util
    Miscellaneous tools used by the rest of egui.
viewport
    egui supports multiple viewports, corresponding to multiple native windows.
widget_style
widget_text
widgets
    Widgets are pieces of GUI such as Label, Button, Slider etc.

Macros

generate_loader_id
    Used to get a unique ID when implementing one of the loader traits: BytesLoader::id, ImageLoader::id, and TextureLoader::id.
github_link_file
    Create a Hyperlink to the current file!() on github.
github_link_file_line
    Create a Hyperlink to the current file!() (and line) on Github
hex_color
    Construct a crate::Color32 from a hex RGB or RGBA string literal.
include_image
    Include an image in the binary.

Structs

Align2
    Two-dimension alignment, e.g. Align2::LEFT_TOP.
AllocatedAtomLayout
    Instructions for painting an AtomLayout.
Atom
    A low-level ui building block.
AtomLayout
    Intra-widget layout utility.
AtomLayoutResponse
    Response from a AtomLayout::show or AllocatedAtomLayout::paint.
Atoms
    A list of Atoms.
ClippedPrimitive
    A Mesh or PaintCallback within a clip rectangle.
Color32
    This format is used for space-efficient color representation (32 bits).
ColorImage
    A 2D RGBA color image in RAM.
Context
    Your handle to egui.
CornerRadius
    How rounded the corners of things should be.
CustomCursorImage
    A bitmap cursor pushed to the integration via PlatformOutput::cursor_image.
DragAndDrop
    Plugin for tracking drag-and-drop payload.
EventFilter
    Controls which events that a focused widget will have exclusive access to.
FontData
    A .ttf or .otf file and a font face index.
FontDefinitions
    Describes the font data and the sizes to use.
FontId
    How to select a sized font.
FontTweak
    Extra scale and vertical tweak to apply to all text of a certain font.
FullOutput
    What egui emits each frame from crate::Context::run_ui.
Galley
    Text that has been laid out, ready for painting.
Grid
    A simple grid layout.
HoveredFile
    A file about to be dropped into egui.
Id
    egui tracks widgets frame-to-frame using Ids.
IdSalt
    Uniquely identifies a child widget within a parent widget.
InputOptions
    Options for input state handling.
InputState
    Input state that egui updates each frame.
InteractOptions
    How to handle multiple calls to crate::Response::interact and crate::Ui::interact_opt.
IntoSizedArgs
    Args passed when sizing an super::Atom
IntoSizedResult
    Result returned when sizing an super::Atom
KeyboardShortcut
    A keyboard shortcut, e.g. Ctrl+Alt+W.
Layout
    The layout of a Ui, e.g. “vertical & centered”.
LogicOutput
    What egui emits from crate::Context::run_logic, i.e. from a tick where no ui was shown.
Margin
    A value for all four sides of a rectangle, often used to express padding or spacing.
Memory
    The data that egui persists between frames.
Mesh
    Textured triangles in two dimensions.
ModifierNames
    Names of different modifier keys.
Modifiers
    State of the modifier keys. These must be fed to egui.
MultiTouchInfo
    All you probably need to know about a multi-touch gesture.
OpenUrl
    What URL to open, and how.
Options
    Some global options that you can read and write.
PaintCallback
    If you want to paint some 3D shapes inside an egui region, you can use this.
PaintCallbackInfo
    Information passed along with PaintCallback (Shape::Callback).
Painter
    Helper to paint shapes and text to a specific region on a specific layer.
PlatformOutput
    The non-rendering part of what egui emits each frame.
PointerState
    Mouse or touch state.
Pos2
    A position on screen.
Rangef
    Inclusive range of floats, i.e. min..=max, but more ergonomic than RangeInclusive.
RawInput
    What the integrations provides to egui at the start of each frame.
Rect
    A rectangular region of space.
RectAlign
    Position a child Rect relative to a parent Rect.
RepaintCause
    What called Context::request_repaint or Context::request_discard?
RequestRepaintInfo
    Information given to the backend about when it is time to repaint the ui.
Rgba
    0-1 linear space RGBA color with premultiplied alpha.
SafeAreaInsets
    The ‘safe area’ insets of the screen
Sense
    What sort of interaction is a widget sensitive to?
Shadow
    The color and fuzziness of a fuzzy shape.
SizedAtom
    A crate::Atom which has been sized.
SizedAtomLayout
    A measured AtomLayout, ready to be painted at a Rect.
Stroke
    Describes the width and color of a line.
TextFormat
    Formatting option for a section of text.
TextureHandle
    Used to paint images.
TextureOptions
    How the texture texels are filtered.
TexturesDelta
    What has been allocated and freed during the last period.
TouchDeviceId
    this is a u64 as values of this kind can always be obtained by hashing
TouchId
    Unique identification of a touch occurrence (finger or pen or …). A Touch ID is valid until the finger is lifted. A new ID is used for the next touch.
Ui
    This is what you use to place widgets.
UiBuilder
    The properties specified when creating a top-level or child Ui.
UiStack
    Information about a crate::Ui and its parents.
UiStackInfo
    Information about a crate::Ui to be included in the corresponding UiStack.
UiStackIterator
    Iterator that walks up a stack of StackFrames.
UiTags
    User-chosen tags.
UserData
    A wrapper around dyn Any, used for passing custom user data to crate::ViewportCommand::Screenshot.
Vec2
    A vector has a direction and length. A Vec2 is often used to represent a size.
Vec2b
    Two bools, one for each axis (X and Y).
ViewportInfo
    Information about the current viewport, given as input each frame.
WidgetInfo
    Describes a widget such as a crate::Button or a crate::TextEdit.
WidgetRect
    Used to store each widget’s Id, Rect and Sense each frame.
WidgetRects
    Stores the WidgetRects of all widgets generated during a single egui update/frame.

Enums

Align
    left/center/right or top/center/bottom alignment for e.g. anchors and layouts.
AtomKind
    The different kinds of crate::Atoms.
CursorIcon
    A mouse cursor icon.
Direction
    A cardinal direction, one of LeftToRight, RightToLeft, TopDown, BottomUp.
Event
    An input event generated by the integration.
FocusDirection
    A direction in which to move the keyboard focus.
FontFamily
    Font of unknown size.
IdSource
    Is this Ui a root or a child of another Ui?
ImageData
    An image stored in RAM.
ImeEvent
    IME event.
Key
    Keyboard keys.
MouseWheelUnit
    The unit associated with the numeric value of a mouse wheel event
OutputCommand
    Commands that the egui integration should execute at the end of a frame.
PointerButton
    Mouse button (or similar for touch input)
Shape
    A paint primitive such as a circle or a piece of text. Coordinates are all screen space points (not physical pixels).
SizedAtomKind
    A sized crate::AtomKind.
StrokeKind
    Describes how the stroke of a shape should be painted.
SurrenderFocusOn
TextWrapMode
    How to wrap and elide text.
TextureFilter
    How the texture texels are filtered.
TextureId
    What texture to use in a Mesh mesh.
TextureWrapMode
    Defines how textures are wrapped around objects when texture coordinates fall outside the [0, 1] range.
Theme
    Dark or Light theme.
ThemePreference
    The user’s theme preference.
TouchPhase
    In what phase a touch event is in.
UiKind
    What kind is this crate::Ui?
UserAttentionType
    Types of attention to request from a user when a native window is not in focus.
ViewportEvent
    An input event from the backend into egui, about a specific viewport.
WidgetType
    The different types of built-in widgets in egui

Constants

NUM_POINTER_BUTTONS
    Number of pointer buttons supported by egui, i.e. the number of possible states of PointerButton.

Traits

AsId
    Types that can be converted to an Id.
AsIdSalt
    Types that can be converted to an IdSalt.
AtomExt
    A trait for conveniently building Atoms.
DroppedFile
    A file dropped into egui.
IntoAtoms
    Trait for turning a tuple of Atoms into Atoms.
NumExt
    Extends f32, Vec2 etc with at_least and at_most as aliases for max and min.

Functions

__run_test_ctx
    For use in tests; especially doctests.
__run_test_ui
    For use in tests; especially doctests.
accesskit_root_id
lerp
    Linear interpolation.
pos2
    pos2(x, y) == Pos2::new(x, y)
remap
    Linearly remap a value from one range to another, so that when x == from.start() returns to.start() and when x == from.end() returns to.end().
remap_clamp
    Like remap, but also clamps the value so that the returned value is always in the to range.
vec2
    vec2(x, y) == Vec2::new(x, y)
warn_if_debug_build
    Helper function that adds a label when compiling with debug assertions enabled.

Type Aliases

AtomClosure
    See AtomKind::Closure
DroppedFileHandle
    A shared reference to a dropped file.
IdMap
    IdMap<V> is a HashMap<Id, V> optimized by knowing that Id has good entropy, and doesn’t need more hashing.
IdSet
    IdSet is a HashSet<Id> optimized by knowing that Id has good entropy, and doesn’t need more hashing.