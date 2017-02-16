//! The stamping mill: run the full pipeline and cast a context pack + manifest.
//!
//! A *pack* is the concatenated, delimited content of the selected files, ready
//! to paste into an agent's context window. A *manifest* is a JSON document
//! explaining every include / exclude decision so the pour is auditable.

use crate::budget::{self, Item};
use crate::json::Json;
use crate::scan::{self, ScanConfig, ScanResult};
use crate::score::{self, ScoreParts};
use crate::secrets;
use crate::tokens;
use crate::{Candidate, Decision, VERSION};
use std::io;
use std::path::Path;

/// Options controlling a compile.
#[derive(Debug, Clone)]
pub struct PackOptions {
    /// Hard token budget for the pack.
    pub budget: u64,
    /// Free-text relevance query (may be empty).
    pub query: String,
    /// Minimum grade a file must reach to be eligible (0.0 disables the floor).
    pub min_score: f64,
    /// Scan tuning.
    pub scan: ScanConfig,
    /// A short label recorded in the manifest (e.g. the pack name).
    pub label: String,
}

impl Default for PackOptions {
    fn default() -> Self {
        PackOptions {
            budget: 8_000,
            query: String::new(),
            min_score: 0.0,
            scan: ScanConfig::default(),
            label: "pour".into(),
        }
    }
}

/// A fully evaluated file with its final decision.
#[derive(Debug, Clone)]
pub struct Entry {
    pub candidate: Candidate,
    pub decision: Decision,
}

/// The complete result of a compile.
#[derive(Debug, Clone)]
pub struct Pack {
    pub options_budget: u64,
    pub options_query: String,
    pub options_min_score: f64,
    pub label: String,
    pub entries: Vec<Entry>,
    pub tokens_used: u64,
    pub captured_value: f64,
    pub method: String,
    /// Files rejected during scanning, kept for the audit trail.
    pub scan_rejected: Vec<scan::RejectedFile>,
}

impl Pack {
    /// Included entries in fill order.
    pub fn included(&self) -> Vec<&Entry> {
        let mut inc: Vec<&Entry> = self
            .entries
            .iter()
