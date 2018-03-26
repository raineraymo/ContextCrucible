# Contributing to ContextCrucible

ContextCrucible is a budget-aware context compiler for coding agents: scan the
ore body, reject the slag, assay the token mass, grade relevance, and pour a
hard-budget context pack with a stamped manifest.

## Ground rules

- Determinism: same repo + manifest + budget => byte-identical pack and manifest.
- The manifest schema is additive-only; field names are frozen as of 1.0.0.
- The spark-test must never produce false negatives for the planted fixtures.

