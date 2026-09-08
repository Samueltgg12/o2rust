//! Graphics subsystem — the CRM chipset.
//!
//! The O2 graphics pipeline consists of four ASICs:
//!
//! - **Microprocessor** — display list / vertex processing + MRE control.
//! - **ICE** (Imaging & Compression Engine) — pixel packaging/unpacking.
//! - **MRE** (Memory & Rendering Engine) — rasterization + texture mapping.
//! - **Display Engine** — analog video generation.
//!
//! Under the UMA, the framebuffer and textures live in main memory (no
//! separate VRAM). The framebuffer is tile-based (GBE).
//!
//! **Status:** milestone M3 (graphics + I/O functional). This module is a
//! placeholder; the full CRM emulation is implemented in later milestones.

/// GBE display engine base address (IRIX `crime.h`).
pub const GBE_BASE: u32 = 0x1600_0000;

/// CRIME render engine base address.
pub const RENDER_BASE: u32 = 0x1500_0000;

/// The graphics subsystem state.
///
/// Placeholder — will hold the CRM chipset state (Microprocessor, ICE, MRE,
/// Display Engine) once milestone M3 begins.
#[derive(Debug, Default)]
pub struct Graphics {
    /// Framebuffer width in pixels.
    pub width: u32,
    /// Framebuffer height in pixels.
    pub height: u32,
    /// Framebuffer pixel format (GBE tile format).
    pub format: u32,
}

impl Graphics {
    /// Create a new graphics subsystem.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the graphics subsystem.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}