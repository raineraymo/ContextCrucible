//! End-to-end integration tests over the bundled `fixtures/sample-repo`.
//!
//! These exercise the whole foundry pipeline: scan → assay → grade → secret
//! quarantine → budget pour → manifest, plus pack comparison.

use contextcrucible::compare;
use contextcrucible::pack::{self, PackOptions};
use contextcrucible::scan::ScanConfig;
use contextcrucible::Decision;
use std::path::PathBuf;

fn fixture_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("fixtures");
    p.push("sample-repo");
    p
