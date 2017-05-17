//! `crucible` — the command-line face of the token foundry.
//!
//! Subcommands:
//!   compile   scan a repo and pour a context pack + manifest
//!   scan      list what the scanner keeps and rejects
//!   compare   weigh two manifests against each other
//!   help / --version
//!
//! The CLI is argument-parsed by hand (stdlib only). All output is
//! deterministic so it can be diffed in CI.

use contextcrucible::compare::{self, Comparison};
use contextcrucible::pack::{self, PackOptions};
use contextcrucible::scan::ScanConfig;
use contextcrucible::VERSION;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("crucible: error: {}", e);
            ExitCode::from(1)
        }
    }
}

fn run(args: &[String]) -> Result<ExitCode, String> {
    let cmd = args.first().map(|s| s.as_str()).unwrap_or("help");
    match cmd {
        "--version" | "-V" => {
            println!("crucible {}", VERSION);
            Ok(ExitCode::SUCCESS)
        }
        "help" | "--help" | "-h" => {
            print_help();
            Ok(ExitCode::SUCCESS)
