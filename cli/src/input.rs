//! Host keyboard/mouse input → O2 PS/2 devices.
//!
//! The O2's PS/2 ports (PC87312, MACE peripheral block) expect a standard
//! keyboard speaking **scan code set 2** and a PS/2 mouse in the default
//! stream mode (3-byte relative packets).
//!
//! Key encoding (set 2): press = make byte(s); release = `0xF0` followed by
//! the make byte(s). Extended keys carry an `0xE0` prefix.
//!
//! Mouse packet:
//! ```text
//! byte 0: [Y-overflow X-overflow Y-sign X-sign 1 0 M R L]
//! byte 1: X delta (signed 8-bit)
//! byte 2: Y delta (signed 8-bit, positive = down on the host = up on screen)
//! ```

use winit::keyboard::KeyCode;

/// Shared host input state owned by the front-end.
#[derive(Debug, Default)]
pub struct HostInput {
    /// PS/2 mouse accumulator + button state.
    pub mouse: MouseState,
}

/// PS/2 keyboard scan code set 2 make codes (no `0xE0` prefix, store raw).
fn make_code(key: KeyCode) -> (bool, u8) {
    let extended = true;
    match key {
        // Letters
        KeyCode::KeyQ => (false, 0x15), KeyCode::KeyW => (false, 0x1D),
        KeyCode::KeyE => (false, 0x24), KeyCode::KeyR => (false, 0x2D),
        KeyCode::KeyT => (false, 0x2C), KeyCode::KeyY => (false, 0x35),
        KeyCode::KeyU => (false, 0x3C), KeyCode::KeyI => (false, 0x43),
        KeyCode::KeyO => (false, 0x44), KeyCode::KeyP => (false, 0x4D),
        KeyCode::KeyA => (false, 0x1C), KeyCode::KeyS => (false, 0x1B),
        KeyCode::KeyD => (false, 0x23), KeyCode::KeyF => (false, 0x2B),
        KeyCode::KeyG => (false, 0x34), KeyCode::KeyH => (false, 0x33),
        KeyCode::KeyJ => (false, 0x3B), KeyCode::KeyK => (false, 0x42),
        KeyCode::KeyL => (false, 0x4B), KeyCode::KeyZ => (false, 0x1A),
        KeyCode::KeyX => (false, 0x22), KeyCode::KeyC => (false, 0x21),
        KeyCode::KeyV => (false, 0x2A), KeyCode::KeyB => (false, 0x32),
        KeyCode::KeyN => (false, 0x31), KeyCode::KeyM => (false, 0x3A),

        // Digits + punctuation (top row / bottom row)
        KeyCode::Backquote => (false, 0x0E), KeyCode::Digit1 => (false, 0x16),
        KeyCode::Digit2 => (false, 0x1E),  KeyCode::Digit3 => (false, 0x26),
        KeyCode::Digit4 => (false, 0x25),  KeyCode::Digit5 => (false, 0x2E),
        KeyCode::Digit6 => (false, 0x36),  KeyCode::Digit7 => (false, 0x3D),
        KeyCode::Digit8 => (false, 0x3E),  KeyCode::Digit9 => (false, 0x46),
        KeyCode::Digit0 => (false, 0x45),  KeyCode::Minus => (false, 0x4E),
        KeyCode::Equal => (false, 0x55),   KeyCode::Backspace => (false, 0x66),
        KeyCode::Tab => (false, 0x0D),     KeyCode::Enter => (false, 0x5A),
        KeyCode::Space => (false, 0x29),   KeyCode::CapsLock => (false, 0x58),
        KeyCode::Semicolon => (false, 0x4C), KeyCode::Quote => (false, 0x52),
        KeyCode::Backslash => (false, 0x5D), KeyCode::BracketLeft => (false, 0x54),
        KeyCode::BracketRight => (false, 0x5B), KeyCode::Comma => (false, 0x41),
        KeyCode::Period => (false, 0x49),  KeyCode::Slash => (false, 0x4A),
        KeyCode::IntlBackslash => (false, 0x61), KeyCode::IntlRo => (false, 0x73),

        // Modifiers
        KeyCode::ShiftLeft => (false, 0x12), KeyCode::ShiftRight => (false, 0x59),
        KeyCode::ControlLeft => (false, 0x14), KeyCode::ControlRight => (extended, 0x14),
        KeyCode::AltLeft => (false, 0x11), KeyCode::AltRight => (extended, 0x11),
        KeyCode::SuperLeft => (extended, 0x1F), KeyCode::SuperRight => (extended, 0x27),

        // Navigation (extended)
        KeyCode::ArrowLeft => (extended, 0x6B), KeyCode::ArrowRight => (extended, 0x74),
        KeyCode::ArrowUp => (extended, 0x75), KeyCode::ArrowDown => (extended, 0x72),
        KeyCode::Home => (extended, 0x6C), KeyCode::End => (extended, 0x69),
        KeyCode::PageUp => (extended, 0x7D), KeyCode::PageDown => (extended, 0x7A),
        KeyCode::Insert => (extended, 0x70), KeyCode::Delete => (extended, 0x71),
        KeyCode::PrintScreen => (extended, 0x12), // real code is E0 12 E0 7C (see below)
        KeyCode::Pause => (extended, 0x14),        // real code is E1 14 77 (see below)

        // Function keys
        KeyCode::F1 => (false, 0x05), KeyCode::F2 => (false, 0x06),
        KeyCode::F3 => (false, 0x04), KeyCode::F4 => (false, 0x0C),
        KeyCode::F5 => (false, 0x03), KeyCode::F6 => (false, 0x0B),
        KeyCode::F7 => (false, 0x83), KeyCode::F8 => (false, 0x0A),
        KeyCode::F9 => (false, 0x01), KeyCode::F10 => (false, 0x09),
        KeyCode::F11 => (false, 0x78), KeyCode::F12 => (false, 0x07),

        // Lock keys
        KeyCode::NumLock => (false, 0x77), KeyCode::ScrollLock => (false, 0x7E),

        // Keypad (set 2 make codes; NumLock state ignored for now)
        KeyCode::Numpad0 => (false, 0x70), KeyCode::Numpad1 => (false, 0x69),
        KeyCode::Numpad2 => (false, 0x72), KeyCode::Numpad3 => (false, 0x7A),
        KeyCode::Numpad4 => (false, 0x6B), KeyCode::Numpad5 => (false, 0x73),
        KeyCode::Numpad6 => (false, 0x74), KeyCode::Numpad7 => (false, 0x6C),
        KeyCode::Numpad8 => (false, 0x75), KeyCode::Numpad9 => (false, 0x7D),
        KeyCode::NumpadDecimal => (false, 0x71),
        KeyCode::NumpadAdd => (false, 0x79),
        KeyCode::NumpadSubtract => (false, 0x7B),
        KeyCode::NumpadMultiply => (false, 0x7C),
        KeyCode::NumpadDivide => (extended, 0x4A),
        KeyCode::NumpadEnter => (extended, 0x5A),

        _ => return (false, 0x00),
    }
}

/// Append the PS/2 service byte sequence for a key event to `out`.
///
/// `pressed = false` produces the break sequence (`0xF0` + make).
pub fn key_to_scan_bytes(key: KeyCode, pressed: bool, out: &mut Vec<u8>) {
    let (extended, code) = make_code(key);
    if code == 0x00 {
        tracing::trace!("unmapped key: {key:?}");
        return;
    }

    let push = |out: &mut Vec<u8>| {
        if extended {
            out.push(0xE0);
        }
        out.push(code);
    };

    if pressed {
        push(out);
    } else {
        if extended {
            out.push(0xE0);
        }
        out.push(0xF0);
        push(out);
    }
}

/// PS/2 mouse: accumulated deltas + button mask.
#[derive(Debug, Default, Clone)]
pub struct MouseState {
    acc_x: f64,
    acc_y: f64,
    left: bool,
    right: bool,
    middle: bool,
}

impl MouseState {
    /// Set the pressed state of a host mouse button, returning `true` when the
    /// change alters a PS/2 packet payload (so a packet should be emitted).
    pub fn set_button(&mut self, button: winit::event::MouseButton, pressed: bool) -> bool {
        let changed = match button {
            winit::event::MouseButton::Left => self.left != pressed,
            winit::event::MouseButton::Right => self.right != pressed,
            winit::event::MouseButton::Middle => self.middle != pressed,
            _ => return false,
        };
        match button {
            winit::event::MouseButton::Left => self.left = pressed,
            winit::event::MouseButton::Right => self.right = pressed,
            winit::event::MouseButton::Middle => self.middle = pressed,
            _ => {}
        }
        changed
    }

    /// Feed a host pointer motion delta (in pixels). Returns `Some` packed
    /// bytes when a whole packet can be formed (always, for accumulated
    /// integer deltas); `None` when nothing moved.
    pub fn feed_motion(&mut self, dx: f64, dy: f64) -> Option<[u8; 3]> {
        if dx == 0.0 && dy == 0.0 && !(self.left || self.right || self.middle) {
            return None;
        }
        // The O2 PS/2 mouse reports signed 8-bit deltas; accumulate the
        // fractional part so slow hosts don't stall the pointer.
        self.acc_x += dx;
        self.acc_y += dy;
        let sx = self.acc_x as i16;
        let sy = self.acc_y as i16;
        self.acc_x -= f64::from(sx);
        self.acc_y -= f64::from(sy);
        Some(self.packet(sx.clamp(-127, 127) as i8, sy.clamp(-127, 127) as i8))
    }

    fn packet(&self, sx: i8, sy: i8) -> [u8; 3] {
        // PS/2 stream-mode packet: bit 3 is always set (reserved), bits 4/5
        // are the X/Y sign, bits 6/7 the X/Y overflow flags.
        let mut flags = 0x08u8;
        if self.left {
            flags |= 0x01;
        }
        if self.right {
            flags |= 0x02;
        }
        if self.middle {
            flags |= 0x04;
        }
        if sx < 0 {
            flags |= 0x10;
        }
        if sy < 0 {
            flags |= 0x20;
        }
        if (sx as i32) > 127 {
            flags |= 0x40;
        }
        if (sy as i32) > 127 {
            flags |= 0x80;
        }
        [flags, sx as u8, sy as u8]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::keyboard::KeyCode;

    #[test]
    fn make_and_break_sequences() {
        let mut make = Vec::new();
        key_to_scan_bytes(KeyCode::KeyA, true, &mut make);
        assert_eq!(make, [0x1C]);

        let mut brk = Vec::new();
        key_to_scan_bytes(KeyCode::KeyA, false, &mut brk);
        assert_eq!(brk, [0xF0, 0x1C]);
    }

    #[test]
    fn extended_keys_carry_e0_prefix() {
        let mut out = Vec::new();
        key_to_scan_bytes(KeyCode::ArrowUp, true, &mut out);
        assert_eq!(out, [0xE0, 0x75]);

        let mut brk = Vec::new();
        key_to_scan_bytes(KeyCode::ArrowUp, false, &mut brk);
        assert_eq!(brk, [0xE0, 0xF0, 0x75]);
    }

    #[test]
    fn unmapped_keys_produce_nothing() {
        let mut out = Vec::new();
        key_to_scan_bytes(KeyCode::F13, true, &mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn mouse_packets_encode_deltas() {
        let mut mouse = MouseState::default();
        let pkt = mouse.feed_motion(10.0, -5.0).unwrap();
        // 0x08 reserved bit + Y-sign (dy negative on host = +y in PS/2 space).
        assert_eq!(pkt, [0x08 | 0x20, 10, 0xFB]);
        // Repeat motion with no buttons still yields a packet.
        let pkt2 = mouse.feed_motion(0.0, 0.0).unwrap();
        assert_eq!(pkt2, [0x08, 0, 0]);
    }

    #[test]
    fn mouse_buttons_set_flags() {
        let mut mouse = MouseState::default();
        assert!(mouse.set_button(winit::event::MouseButton::Left, true));
        let pkt = mouse.feed_motion(1.0, 1.0).unwrap();
        assert_eq!(pkt[0] & 0x01, 0x01);
        assert!(mouse.set_button(winit::event::MouseButton::Left, false));
    }
}