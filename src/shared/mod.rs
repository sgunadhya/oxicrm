#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identifier(pub uuid::Uuid);

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
