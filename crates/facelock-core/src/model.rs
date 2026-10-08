//! Face model types.
//!
//! A [`Model`] is the result of enrolling a face: an embedding vector plus
//! enough metadata to identify and describe it. This module defines the
//! in-memory representation only — persistence lives in the daemon.

use std::time::SystemTime;

/// Opaque identifier for a stored model.
///
/// The string is chosen by whoever creates the model. The store is free to
/// interpret it as a filename, a UUID, a username, or anything else, so
/// callers must not assume a structure. Use [`ModelId::as_str`] for display
/// and logging only.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModelId(String);

impl ModelId {
    /// Create a new [`ModelId`] from a string.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the string representation of the [`ModelId`].
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// An enrolled face: an embedding vector plus identifying metadata.
///
/// The embedding is a fixed-length float vector produced by the embedder
/// (e.g. 512 dimensions for ArcFace). Its length is determined by the model
/// the embedder uses, not by this type — matching two embeddings with
/// different lengths is a caller error.
#[derive(Debug, Clone)]
pub struct Model {
    /// Opaque identifier for this model. The store is free to interpret it as a
    pub id: ModelId,
    /// The embedding vector produced by the embedder. Its length is determined
    pub embedding: Vec<f32>,
    /// The time when this model was created. This is set by the store when the
    pub created_at: SystemTime,
    /// Human-facing metadata attached to this model. This is optional and may
    pub metadata: ModelMetadata,
}

impl Model {
    /// Dimensionality of the embedding. Useful for validating that two models
    /// are comparable before running a matcher over them.
    pub fn dimensions(&self) -> usize {
        self.embedding.len()
    }
}

/// Human-facing metadata attached to a [`Model`].
///
/// Everything here is optional. The matcher never reads it; it exists for
/// the UI and for operators debugging a deployment.
#[derive(Debug, Clone, Default)]
pub struct ModelMetadata {
    /// Free-form label, e.g. `"default"`, `"with glasses"`.
    pub label: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions_matches_embedding_len() {
        let m = Model {
            id: ModelId::new("alice"),
            embedding: vec![0.0; 512],
            created_at: SystemTime::now(),
            metadata: ModelMetadata::default(),
        };
        assert_eq!(m.dimensions(), 512);
    }
}
