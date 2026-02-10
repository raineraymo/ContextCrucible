# Contributing to ContextCrucible

ContextCrucible is a budget-aware context compiler for coding agents: scan the
ore body, reject the slag, assay the token mass, grade relevance, and pour a
hard-budget context pack with a stamped manifest.

## Ground rules

- Determinism: same repo + manifest + budget => byte-identical pack and manifest.
- The manifest schema is additive-only; field names are frozen as of 1.0.0.
- The spark-test must never produce false negatives for the planted fixtures.

## Workflow

1. Fork, create a topic branch.
2. `cargo fmt --check && cargo clippy --all-targets -- -D warnings`.
3. `cargo test` green; `make` targets from the Makefile where applicable.
4. Explorer changes: `npm ci && npm test` in `explorer/`.
5. Conventional commits (`feat:`, `fix:`, `docs:`, `test:`, `chore:`).

# draft note 20
