---
needs: []
---
Modern deck legality: banlist via mtgjson legalities (derived data only),
the 4-of rule, and sideboard size validation.

## Deferred — premature until broader card coverage (2026-06-14)
Not worth doing until the engine can represent a much larger slice of cards;
deck/format legality has little value over a tiny card pool. When revisited it also
needs a data step: the derived corpus carries no per-format legalities — only a
vintage-derived `supported` bool (`crates/deckmaste_migrations/src/extract.rs:29`).
That means extending the mtgjson `Legalities` ingest + the derived schema and
regenerating the shared gitignored `data/` fixture, plus pinning a `Format` model.
