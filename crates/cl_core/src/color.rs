//! RGBA color type stored as `f32` components in the `[0, 1]` range.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    // ── Named presets ────────────────────────────────────────────────────────
    pub const WHITE:       Self = Self::rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK:       Self = Self::rgba(0.0, 0.0, 0.0, 1.0);
    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);
    pub const RED:         Self = Self::rgba(1.0, 0.0, 0.0, 1.0);
    pub const GREEN:       Self = Self::rgba(0.0, 0.8, 0.0, 1.0);
    pub const BLUE:        Self = Self::rgba(0.0, 0.4, 1.0, 1.0);
    pub const YELLOW:      Self = Self::rgba(1.0, 1.0, 0.0, 1.0);
    pub const CYAN:        Self = Self::rgba(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA:     Self = Self::rgba(1.0, 0.0, 1.0, 1.0);
    pub const ORANGE:      Self = Self::rgba(1.0, 0.6, 0.0, 1.0);
    pub const GRAY:        Self = Self::rgba(0.5, 0.5, 0.5, 1.0);
    pub const DARK_GRAY:   Self = Self::rgba(0.2, 0.2, 0.2, 1.0);

    #[inline]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Parse a `#RRGGBB` or `#RRGGBBAA` hex string.
    pub fn from_html(s: &str) -> Option<Self> {
        let s = s.trim_start_matches('#');
        let parse = |slice: &str| u8::from_str_radix(slice, 16).ok().map(|v| v as f32 / 255.0);
        match s.len() {
            6 => Some(Self::rgb(parse(&s[0..2])?, parse(&s[2..4])?, parse(&s[4..6])?)),
            8 => Some(Self::rgba(parse(&s[0..2])?, parse(&s[2..4])?, parse(&s[4..6])?, parse(&s[6..8])?)),
            _ => None,
        }
    }

    /// Linear interpolation between two colors.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    #[inline]
    pub fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Default for Color {
    fn default() -> Self { Self::WHITE }
}

impl From<[f32; 4]> for Color {
    fn from(a: [f32; 4]) -> Self { Self::rgba(a[0], a[1], a[2], a[3]) }
}

impl From<[f32; 3]> for Color {
    fn from(a: [f32; 3]) -> Self { Self::rgb(a[0], a[1], a[2]) }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self { c.to_array() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_presets_are_correct() {
        assert_eq!(Color::WHITE.to_array(), [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(Color::BLACK.to_array(), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(Color::TRANSPARENT.a, 0.0);
    }

    #[test]
    fn color_from_html_6_digit() {
        let c = Color::from_html("#ff8000").unwrap();
        assert!((c.r - 1.0).abs() < 1e-3);
        assert!((c.g - 0.502).abs() < 1e-2);
        assert!((c.b - 0.0).abs() < 1e-3);
        assert_eq!(c.a, 1.0);
    }

    #[test]
    fn color_from_html_8_digit_includes_alpha() {
        let c = Color::from_html("#ffffff80").unwrap();
        assert_eq!(c.r, 1.0);
        assert!((c.a - 0.502).abs() < 1e-2);
    }

    #[test]
    fn color_from_html_without_hash() {
        assert!(Color::from_html("ff0000").is_some());
    }

    #[test]
    fn color_from_html_invalid_returns_none() {
        assert!(Color::from_html("#xyz").is_none());
        assert!(Color::from_html("#12345").is_none());
    }

    #[test]
    fn color_lerp_midpoint() {
        let a = Color::BLACK;
        let b = Color::WHITE;
        let mid = a.lerp(b, 0.5);
        assert!((mid.r - 0.5).abs() < 1e-6);
        assert!((mid.g - 0.5).abs() < 1e-6);
        assert!((mid.b - 0.5).abs() < 1e-6);
    }

    #[test]
    fn color_lerp_at_zero_equals_self() {
        let c = Color::RED;
        let lerped = c.lerp(Color::BLUE, 0.0);
        assert_eq!(lerped.r, c.r);
        assert_eq!(lerped.b, c.b);
    }

    #[test]
    fn color_from_array_roundtrip() {
        let arr = [0.1, 0.2, 0.3, 0.4_f32];
        let c = Color::from(arr);
        assert_eq!(<[f32; 4]>::from(c), arr);
    }
}
