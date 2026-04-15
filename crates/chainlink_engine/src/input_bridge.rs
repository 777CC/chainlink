//! Translate winit events into `chainlink_input` types.

use chainlink_core::Vec2;
use chainlink_input::{InputEvent, InputServer, Key, MouseButton};
use winit::event::{
    ElementState, KeyEvent, MouseButton as WinitMouseButton,
    MouseScrollDelta, WindowEvent,
};
use winit::keyboard::{KeyCode, PhysicalKey};

/// Convert a winit `WindowEvent` into zero or more `InputEvent`s and feed
/// them to the `InputServer`.
pub fn handle_window_event(server: &mut InputServer, event: &WindowEvent) {
    match event {
        // ── Keyboard ──────────────────────────────────────────────────────────
        WindowEvent::KeyboardInput {
            event: KeyEvent { physical_key, state, repeat, .. },
            ..
        } => {
            if let PhysicalKey::Code(code) = physical_key {
                let key = map_keycode(*code);
                let ev = match state {
                    ElementState::Pressed  => InputEvent::KeyPressed { key, repeat: *repeat },
                    ElementState::Released => InputEvent::KeyReleased { key },
                };
                server.handle_event(ev);
            }
        }

        // ── Mouse button ──────────────────────────────────────────────────────
        WindowEvent::MouseInput { button, state, .. } => {
            let btn = map_mouse_button(*button);
            // We don't track cursor position here (comes separately)
            let pos = server.mouse_position();
            let ev = match state {
                ElementState::Pressed  => InputEvent::MousePressed  { button: btn, position: pos },
                ElementState::Released => InputEvent::MouseReleased { button: btn, position: pos },
            };
            server.handle_event(ev);
        }

        // ── Mouse move ────────────────────────────────────────────────────────
        WindowEvent::CursorMoved { position, .. } => {
            let new_pos = Vec2::new(position.x as f32, position.y as f32);
            let old_pos = server.mouse_position();
            let ev = InputEvent::MouseMoved {
                position: new_pos,
                delta:    new_pos - old_pos,
            };
            server.handle_event(ev);
        }

        // ── Mouse scroll ──────────────────────────────────────────────────────
        WindowEvent::MouseWheel { delta, .. } => {
            let scroll = match delta {
                MouseScrollDelta::LineDelta(x, y) => Vec2::new(*x * 32.0, *y * 32.0),
                MouseScrollDelta::PixelDelta(d)   => Vec2::new(d.x as f32, d.y as f32),
            };
            server.handle_event(InputEvent::MouseScrolled { delta: scroll });
        }

        _ => {}
    }
}

// ── Key mapping ───────────────────────────────────────────────────────────────

fn map_keycode(code: KeyCode) -> Key {
    use KeyCode::*;
    match code {
        KeyA => Key::A, KeyB => Key::B, KeyC => Key::C, KeyD => Key::D,
        KeyE => Key::E, KeyF => Key::F, KeyG => Key::G, KeyH => Key::H,
        KeyI => Key::I, KeyJ => Key::J, KeyK => Key::K, KeyL => Key::L,
        KeyM => Key::M, KeyN => Key::N, KeyO => Key::O, KeyP => Key::P,
        KeyQ => Key::Q, KeyR => Key::R, KeyS => Key::S, KeyT => Key::T,
        KeyU => Key::U, KeyV => Key::V, KeyW => Key::W, KeyX => Key::X,
        KeyY => Key::Y, KeyZ => Key::Z,

        Digit0 => Key::Key0, Digit1 => Key::Key1, Digit2 => Key::Key2,
        Digit3 => Key::Key3, Digit4 => Key::Key4, Digit5 => Key::Key5,
        Digit6 => Key::Key6, Digit7 => Key::Key7, Digit8 => Key::Key8,
        Digit9 => Key::Key9,

        F1 => Key::F1, F2 => Key::F2, F3 => Key::F3, F4 => Key::F4,
        F5 => Key::F5, F6 => Key::F6, F7 => Key::F7, F8 => Key::F8,
        F9 => Key::F9, F10 => Key::F10, F11 => Key::F11, F12 => Key::F12,

        ArrowUp => Key::Up, ArrowDown => Key::Down,
        ArrowLeft => Key::Left, ArrowRight => Key::Right,

        Home => Key::Home, End => Key::End,
        PageUp => Key::PageUp, PageDown => Key::PageDown,

        Backspace => Key::Backspace, Delete => Key::Delete,
        Insert => Key::Insert, Tab => Key::Tab, Enter => Key::Enter,
        NumpadEnter => Key::NumpadEnter,
        Space => Key::Space, Escape => Key::Escape,

        ShiftLeft => Key::Shift, ShiftRight => Key::ShiftRight,
        ControlLeft => Key::Ctrl, ControlRight => Key::CtrlRight,
        AltLeft => Key::Alt, AltRight => Key::AltRight,
        SuperLeft | SuperRight => Key::Super,

        Minus => Key::Minus, Equal => Key::Equal,
        BracketLeft => Key::BracketLeft, BracketRight => Key::BracketRight,
        Backslash => Key::Backslash, Semicolon => Key::Semicolon,
        Quote => Key::Apostrophe, Comma => Key::Comma,
        Period => Key::Period, Slash => Key::Slash,
        Backquote => Key::Grave,

        Numpad0 => Key::Numpad0, Numpad1 => Key::Numpad1,
        Numpad2 => Key::Numpad2, Numpad3 => Key::Numpad3,
        Numpad4 => Key::Numpad4, Numpad5 => Key::Numpad5,
        Numpad6 => Key::Numpad6, Numpad7 => Key::Numpad7,
        Numpad8 => Key::Numpad8, Numpad9 => Key::Numpad9,
        NumpadAdd      => Key::NumpadAdd,
        NumpadSubtract => Key::NumpadSubtract,
        NumpadMultiply => Key::NumpadMultiply,
        NumpadDivide   => Key::NumpadDivide,
        NumpadDecimal  => Key::NumpadDecimal,

        _ => Key::Unknown,
    }
}

fn map_mouse_button(btn: WinitMouseButton) -> MouseButton {
    match btn {
        WinitMouseButton::Left   => MouseButton::Left,
        WinitMouseButton::Right  => MouseButton::Right,
        WinitMouseButton::Middle => MouseButton::Middle,
        WinitMouseButton::Back   => MouseButton::Back,
        WinitMouseButton::Forward => MouseButton::Forward,
        WinitMouseButton::Other(n) => MouseButton::Other(n),
    }
}
