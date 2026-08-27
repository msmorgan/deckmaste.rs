---
needs: []
---
# Declare the type-line order in english_v2

Routed from `workbench-card-class-and-command-zone` (close, 2026-08-26),
superseding the follow-up left by `workbench-type-line-order-is-spelling`.
The measured order table now lives at
`crates/deckmaste_english_v2/docs/type-line-order.md` (12 attested
sequences; ranks 9–14 are [CR#300.1]'s stated alphabetical convention), but
the crate still builds no type line: nothing consumes the table. When the
v2 spelling layer renders or parses a type line, this document is its
declaration source — wire it then; do not build a consumer before one is
needed.

## Consumption boundary

`crates/deckmaste_english_v2` only. No workbench files.

## Acceptance

- The type-line order is consumed from the declaration (or the doc is
  converted to the declaration form the crate uses), with tests against the
  attested sequences.

Standard constraints apply.
