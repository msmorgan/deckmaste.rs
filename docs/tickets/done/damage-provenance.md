---
needs: [migrate-damage-sbas-to-rules]
---
Make marked damage source-aware so deathtouch (and future source-based effects)
can be expressed generically. Today `GameObject.damage` is a flat `Uint` and
deathtouch rides a bespoke deal-time `struck_by_deathtouch` bool plus a
`Condition::DamagedByDeathtouch` placeholder. Replace `obj.damage` with a list of
`(source-with-deal-time-abilities, amount)` — the source's deathtouch-ness captured
when the damage is dealt, since the source may later lose it or leave (the current
`LkiSnapshot` carries identity but NOT abilities, so it must be extended or a
purpose-built record used). Add a `Reference::Source` binding over a creature's
damage sources, update the ~6 `.damage` readers (lethal check, cleanup ×2, LKI,
block-availability), then replace `Condition::DamagedByDeathtouch` with
`Is(Ref(Source), Has(Deathtouch))` (already expressible via `Condition::Is` +
`Filter::Has` — no new `HasAbility` condition) and remove the
`struck_by_deathtouch` flag. Sized as a sprawling ~6-runtime-file change.
