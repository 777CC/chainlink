use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringName(Arc<str>);

impl StringName {
    pub fn new(s: impl AsRef<str>) -> Self { Self(Arc::from(s.as_ref())) }
    pub fn as_str(&self) -> &str { &self.0 }
}

impl From<&str> for StringName { fn from(s: &str) -> Self { Self::new(s) } }
impl From<String> for StringName { fn from(s: String) -> Self { Self::new(s) } }

impl std::fmt::Display for StringName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.0.fmt(f) }
}
