//! Typed errors. A malformed fence names the fence; the renderer never
//! swallows a failure silently.

/// Why a document could not be rendered.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum DocsError {
    /// An `island` fence carried no component name.
    #[error("island fence without a component name: `{0}`")]
    IslandWithoutName(String),
    /// An island fence's body was not a JSON object.
    #[error("island props must be a JSON object, got: `{0}`")]
    InvalidIslandProps(String),
}
