//! A tiny, dependency-free JSON writer.
//!
//! The project is stdlib-only, so we cannot pull in `serde`. This module offers
//! just enough to emit stable, pretty-printed JSON for manifests and pack
//! metadata. Keys are written in insertion order, giving deterministic output.

use std::fmt::Write as _;

/// A JSON value tree that can be rendered deterministically.
#[derive(Debug, Clone)]
pub enum Json {
    Null,
    Bool(bool),
    /// Integer number (rendered without a decimal point).
    Int(i64),
