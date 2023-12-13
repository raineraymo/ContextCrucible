<!-- The Token Foundry :: contextcrucible -->

<p align="center">
  <img src="docs/assets/crucible-hero.svg" alt="contextcrucible pours molten tokens from a crucible into a budget-sized mold" width="100%" />
</p>

<h1 align="center">contextcrucible</h1>

<p align="center">
  <strong>A budget-aware context compiler for coding agents.</strong><br/>
  Scan the ore body. Reject the slag. Assay the token mass. Grade the relevance.<br/>
  Spark-test for secrets. Pour a hard-budget context pack — and stamp a manifest that explains every gram.
</p>

<p align="center">
  <code>rust 1.74+ · stdlib only</code> ·
  <code>typescript 5 explorer</code> ·
  <code>zero runtime dependencies</code> ·
  <code>deterministic output</code>
</p>

---

## Why a foundry?

Feeding a coding agent is metallurgy, not magic. You have a mountain of raw ore
— a repository — and a mold of fixed size — the model's context window. You
cannot pour the whole mountain in. You must **choose**: melt down the highest-
grade files, skim off the vendored slag and the minified dross, and *never* let
a molten credential splash into the pour.

`contextcrucible` is that foundry. It is a single, dependency-free Rust binary
(`crucible`) plus a small TypeScript explorer that visualises the result. Every
stage is transparent and every decision is written to an auditable manifest.
There are **no embeddings, no learned models, and no network calls** — every
point of relevance is traceable to a concrete token match you can read yourself.

<p align="center">
  <img src="docs/assets/budget-smelter.svg" alt="Animated bars showing a token budget poured into the highest-grade files first" width="90%" />
</p>

---

## The seven stages of the pour

```
   ore body                                                        context pack
  ┌────────┐   scan      tokens     score      secrets    budget   ┌──────────┐
  │  repo  │ ────────▶ ────────▶ ────────▶ ────────▶ ────────▶ ──▶ │  pack +  │
  └────────┘  reject   assay      grade      quarantine  pour      │ manifest │
              slag     mass       relevance  credentials           └──────────┘
```

1. **scan** — a deterministic, symlink-safe walk that rejects vendor
   directories (`node_modules`, `target`, `vendor`, …), generated artefacts
   (`*.min.js`, lockfiles, `*.pb.go`, …), binary files (by extension *and* by
   sampling the head for NUL bytes and non-text ratio), and anything over the
   size cap.
2. **tokens** — a transparent token estimator. It counts word segments (long
   identifiers fragment every ~4 chars, matching how BPE splits
   `snake_case_names`), charges one segment per punctuation mark, and collapses
   whitespace runs. It is a *heuristic*, deterministic and honest — it does not
   pretend to be any specific vendor tokenizer.
3. **score** — four additive signals, weighted to 100: **path** (25), **query
   frequency** (35, with diminishing returns), **import graph** (20), and
   **symbol definitions** (20). The breakdown is preserved so the manifest can
   explain each grade.
4. **secrets** — rule-based spark tests for AWS keys, PEM private keys, GitHub
   and Slack tokens, JWTs, and a generic *high-entropy assignment to a secret-
   looking name*. Placeholders like `changeme` or `${VAR}` are deliberately
   ignored. Any finding at or above 0.75 confidence quarantines the whole file.
