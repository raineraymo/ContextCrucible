//! Spark test: detect likely credentials so they never reach an LLM context.
//!
//! Detection is deliberately conservative and rule-based — there are no network
//! calls and no learned models. Each rule reports a confidence in `0.0..=1.0`.
//! Any finding at or above [`QUARANTINE_CONFIDENCE`] excludes the whole file.
//!
//! The rules combine well-known token shapes (AWS access keys, private-key PEM
//! headers, JWTs, Slack / GitHub tokens) with a generic "assignment of a long
//! high-entropy value to a secret-looking name" heuristic.

/// Files with a finding at or above this confidence are excluded outright.
pub const QUARANTINE_CONFIDENCE: f64 = 0.75;

/// A single secret detection within a file.
#[derive(Debug, Clone, PartialEq)]
pub struct Finding {
    /// Human-readable rule name (e.g. `aws-access-key-id`).
    pub rule: String,
    /// 1-based line number where the match began.
    pub line: usize,
    /// Detection confidence in `0.0..=1.0`.
    pub confidence: f64,
    /// A redacted excerpt safe to show in a manifest.
    pub redacted: String,
}

/// Scan file content and return all findings, ordered by line.
pub fn scan(content: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    for (idx, line) in content.lines().enumerate() {
        let line_no = idx + 1;
        detect_line(line, line_no, &mut findings);
