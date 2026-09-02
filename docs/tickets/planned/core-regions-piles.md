---
needs: [core-regions-discourse]
---
**Stage 4 of [Core is explicit regions](../../decisions/core-explicit-regions.md):
piles are registers.** Types, validator, and lowering only; the engine
resolution stays in `engine-piles`, which needs this. Standard constraints
apply.

## Scope

- `SeparatePiles { dests: [DefId], group, by }` defines one pile register
  per pile [CR#700.3a]; `ChoosePile { dest, from: [RefId], by, random }`
  defines the chosen pile [CR#700.3b]. `Selection::PilesOf`,
  `PileSource::Labels`, and `PileSource::Noted` are deleted; a pile is read
  as `Reg`. `Value::Pile` is an object group, not an object.
- Lowering maps the authored pile labels of `plugins/canon/cards/Do or
  Die.ron` onto dests, so core carries no label. Removing the labels from
  the authored surface itself, and the Idris `DivideAndChoose` emit gap,
  stay in `core-do-or-die-divide-and-choose`.
- Validator: a `ChoosePile` reads only pile registers; `Each` over a pile
  register iterates its members.

## Gates

Do or Die lowers with no label in the core term, and the engine still
reaches its `todo!()` pile seam rather than a lowering error. Whims of the
Fates is the N-pile design target once authored.
