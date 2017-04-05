//! The assay balance: weigh two pours against each other.
//!
//! Comparison is done over parsed manifest summaries (or live [`Pack`]s). It
//! reports which files entered one pack but not the other, and how the budget
//! utilisation and captured value shifted.

use crate::json::Json;
use crate::pack::{utilization, Pack};
use crate::Decision;
use std::collections::BTreeSet;

/// The difference between a baseline pack (`a`) and a candidate pack (`b`).
#[derive(Debug, Clone)]
pub struct Comparison {
    pub label_a: String,
    pub label_b: String,
    pub tokens_a: u64,
    pub tokens_b: u64,
    pub value_a: f64,
    pub value_b: f64,
    pub util_a: f64,
    pub util_b: f64,
    /// Files included in `b` but not in `a`.
    pub added: Vec<String>,
    /// Files included in `a` but not in `b`.
    pub removed: Vec<String>,
    /// Files included in both.
    pub retained: Vec<String>,
}

impl Comparison {
    /// Change in captured value, `b - a`.
    pub fn value_delta(&self) -> f64 {
        self.value_b - self.value_a
    }

    /// Change in tokens used, `b - a`.
    pub fn token_delta(&self) -> i64 {
        self.tokens_b as i64 - self.tokens_a as i64
    }
