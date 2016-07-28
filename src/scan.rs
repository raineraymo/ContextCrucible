//! Ore extraction: walk a repository and reject slag.
//!
//! A file is rejected (never enters the pipeline) if it is:
//!   * inside a well-known vendor / dependency / build directory,
//!   * matched by a generated-artifact name or extension,
//!   * binary (contains a NUL byte in its leading sample, or is > the text
//!     ratio threshold of non-text bytes),
//!   * larger than `max_bytes`.
//!
//! Everything is walked with a manual, deterministic depth-first traversal that
//! sorts each directory's entries so results never depend on filesystem order.

use crate::Decision;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Directories whose entire subtree is skipped.
pub const VENDOR_DIRS: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "node_modules",
    "vendor",
    "target",
    "dist",
    "build",
    "out",
    ".venv",
    "venv",
    "__pycache__",
    ".mypy_cache",
    ".pytest_cache",
