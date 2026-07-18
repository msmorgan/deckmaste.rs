---
needs: []
---
`filter_noun`'s controller suffix renders `ControlledBy(OpponentOf(Ref(You)))`
as "an opponent controls", but current oracle wording is "your opponents
control". Systemic: the "your opponents control" mismatch appears ~44× across
the corpus (pump and non-pump) — e.g. Copperhoof Vorrac, Crusading Knight,
Marauding Knight, Scourge of Geier Reach, Bleeding Woods.

The fix lives in the shared `controller_suffix` / `singular_controller_suffix`
in `crates/deckmaste_cards/src/render/fragment.rs` — not for-each-pump specific
(the pump macros only newly EXPOSED it by rendering these selections at all).

Verify: the ~44 "your opponents control" fidelity mismatches clear; no
regression to "you control" / "you don't control" / "an opponent controls"
where those ARE the oracle wording (check both singular and plural forms).
