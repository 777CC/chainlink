//! Keyboard key identifiers.

/// Physical keyboard key — subset of the USB HID usage table, mapped to
/// Godot-compatible names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // ── Alphanumeric ─────────────────────────────────────────────────────────
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    Key0, Key1, Key2, Key3, Key4,
    Key5, Key6, Key7, Key8, Key9,

    // ── Function keys ────────────────────────────────────────────────────────
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

    // ── Navigation ───────────────────────────────────────────────────────────
    Up, Down, Left, Right,
    Home, End, PageUp, PageDown,

    // ── Editing ──────────────────────────────────────────────────────────────
    Backspace, Delete, Insert, Tab, Enter,
    Space, Escape,

    // ── Modifiers ────────────────────────────────────────────────────────────
    Shift, Ctrl, Alt, Super,
    ShiftRight, CtrlRight, AltRight,

    // ── Symbols ──────────────────────────────────────────────────────────────
    Minus, Equal, BracketLeft, BracketRight,
    Backslash, Semicolon, Apostrophe,
    Comma, Period, Slash, Grave,

    // ── Numpad ───────────────────────────────────────────────────────────────
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4,
    Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadAdd, NumpadSubtract, NumpadMultiply, NumpadDivide,
    NumpadEnter, NumpadDecimal,

    /// Unknown / unmapped key.
    Unknown,
}
