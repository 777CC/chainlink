#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NodePath(String);

impl NodePath {
    pub fn new(s: impl Into<String>) -> Self { Self(s.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    /// Iterator over path segments split by '/'.
    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.0.split('/').filter(|s| !s.is_empty())
    }
}

impl From<&str> for NodePath { fn from(s: &str) -> Self { Self::new(s) } }
impl From<String> for NodePath { fn from(s: String) -> Self { Self::new(s) } }
