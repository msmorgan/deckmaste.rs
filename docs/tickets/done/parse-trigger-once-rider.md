---
needs: []
---
Parse the triggered-ability limit rider: `This ability triggers only once
each turn.` (and the `… only once.` per-game form where it appears). The
`UseLimit` machinery already models per-turn and per-game limits on abilities
(`crates/deckmaste_core/src/ability.rs`); the triggered-ability parser
(`parsers/triggered_ability.rs`) just needs to strip the trailing rider
sentence and set the limit on the parsed trigger frame instead of failing
the whole ability.

Sibling of `parse-activation-restrictions` (same trailing-rider technique on
the activated frame); kept separate so the two parser files can be worked in
parallel.

**~67 of 17,022 one-away cards** (2026-07-16 tally).

Verify: `cargo xtask generate plugins/wizards` graduation delta; one
once-each-turn trigger card round-trips via `cargo xtask fidelity`.
