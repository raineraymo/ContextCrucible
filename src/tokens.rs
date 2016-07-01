//! Token assay.
//!
//! We do **not** claim to reproduce any particular tokenizer's vocabulary — that
//! would require shipping a model's merge table. Instead we use a transparent,
//! deterministic heuristic calibrated against the rough behaviour of byte-pair
//! encoders on source code: tokens tend to break on word boundaries, runs of
//! whitespace collapse, and punctuation is comparatively dense.
//!
//! The estimator is a pure function of the input bytes, so two runs over the
//! same file always agree.

