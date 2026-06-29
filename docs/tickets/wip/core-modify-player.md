---
needs: []
---
**Core: `ModifyPlayer` continuous statics over player attributes (extra land
plays, no-maximum hand size).** From the 2026-06-28 idris↔rust model audit.

Today no continuous static modifies a player attribute: `StaticEffect`
(`crates/deckmaste_core/src/continuous.rs`) modifies objects only, and
max-hand-size etc. live as raw engine player state with no data-model static. So
Exploration ("you may play an additional land"), Reliquary Tower ("you have no
maximum hand size"), and hand-size caps are unrepresentable as data.

Idris `StaticEffect.ModifyPlayer (Reference …APlayer) PlayerMod`
(`idris/src/Core.idr`) with `PlayerMod = SetTo | Raise | Lower | NoMax` over a
`PlayerAttr` (`Life | HandSize | HandSizeLimit | LandPlaysPerTurn`). `NoMax` is a
dedicated op (not a `Maybe` value) since the reader is a count.

Adoption: add a `ModifyPlayer` static plus a player-attribute modification
(set/raise/lower, plus a "no maximum" form).

Verdict: **improvement** (common statics with no current home). Effort: **M**.
Related: NONE (the Idris side landed in `idris-effects-costs-and-choices` (done/);
this is the Rust core-type gap).
