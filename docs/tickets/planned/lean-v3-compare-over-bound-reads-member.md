---
needs: []
---
# A comparison over a domain lets its bound read the member

Standard constraints apply.

`Predicate.compareOver domain measure comparator bound` steps `measure` with the member of
`domain` in scope (`ownMember`, "itself"), but steps `bound` in a context that drops it (the
`.compareOver` arm of the predicate stepper, `lean/Semantics/Check/Phrase.lean`). So a bound that
reads the member back is refused `.anaphor`: "a creature with power greater than its base power"
cannot be written `comparesOwnStat .power … .greater (basePowerOf ownMember)`.

Found in `lean-v3-rules-and-card-fidelity` (owner ruling 2026-10-25 on base power and toughness):
*Baird, Argivian Recruiter* (`lean/Semantics/Cards/Trigger.lean`) is spelled as
`compareOver … (minus (powerOf ownMember) (basePowerOf ownMember)) .greater 0` only because of this
limitation. *Kutzil, Malamet Exemplar* and *Sovereign Okinec Ahau* print the same comparison.

Change: let the bound read the member as the measure does, keeping a bound that names no member
(a count, a printed number) unchanged, with a pin that "power greater than its base power" checks
written over `ownMember` in the bound. Then respell Baird that way and update
`bairdComparesPowerWithItsOwnBasePower` (`lean/Semantics/Proofs/CardFidelity.lean`).
