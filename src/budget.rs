//! The pour: deterministic hard-budget selection.
//!
//! Given a set of graded candidates each with a token mass and a relevance
//! grade, choose a subset whose total token mass does not exceed a hard budget
//! while maximising captured relevance. This is a 0/1 knapsack.
//!
//! We solve it exactly with dynamic programming when the budget (scaled to a
//! coarse grid) is small enough, and fall back to a deterministic
//! value-density greedy for very large inputs. Both paths are fully
//! deterministic: ties are broken by `(‑score, tokens, rel_path)` so the same
//! inputs always yield the same pour.

/// One item presented to the solver.
#[derive(Debug, Clone)]
pub struct Item {
    /// Stable identity (repo-relative path).
    pub id: String,
    /// Token cost of including the item.
    pub tokens: u64,
    /// Relevance grade (value) in `0.0..=100.0`.
    pub score: f64,
}

/// Result of a solve.
#[derive(Debug, Clone)]
pub struct Solution {
    /// Selected item ids in fill order (highest priority first).
    pub selected: Vec<String>,
    /// Total tokens consumed.
    pub tokens_used: u64,
