# workbench-bench-witness-gaps

Two corpus-real vocabulary arms with no bench or pin witness, surfaced by the
first `cargo xtask map idris-dead` run (idris-dead-constructor-audit, done
2026-08-27). Each wants a bench card, not a pin — nothing about them is
rules-meaningless.

- `ChapterNumber`'s `ChapterIV`/`ChapterV`/`ChapterVI` — chapter markers occur
  in supported Saga text (35 / 2 / 3 occurrences) with no witness.
- `ConferringWord`'s `StoriedW` and `RenownW` — renown (22 supported cards) and
  the storied designation (11). The type's other three arms are macro-mediated;
  these two are not.

Corpus claims measured via `jq 'select(.supported)'` over
`data/derived/cards.jsonl` at audit time; re-verify at claim.

## Optional tooling follow-up

`map idris-dead` could grow an allowlist. The audit's triage names the two
exemption classes it would have to encode — structurally unnameable type-index
constructors and vocabulary spelled only in the declaring module's fact
tables — build it only if repeat runs prove worth the encoding.
