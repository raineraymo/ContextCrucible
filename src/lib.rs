//! contextcrucible — a budget-aware context compiler for coding agents.
//!
//! The library is built entirely on the Rust standard library. It is organised
//! as a small pipeline of pure-ish stages that mirror an industrial foundry:
//!
//! 1. [`scan`]   — walk the ore body (repository) and reject slag
//!    (generated / vendor / binary files).
//! 2. [`tokens`] — assay each nugget for its token mass.
