# contextcrucible — the token foundry
#
# A thin orchestration layer over cargo and npm. Targets are phony; the real
# work lives in the Rust crate and the TypeScript explorer.

CARGO ?= cargo
NPM   ?= npm
