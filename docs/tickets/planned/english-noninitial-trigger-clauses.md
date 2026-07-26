---
needs: []
---
**`When`/`Whenever` trigger clauses fail to parse when they do not open
their ability's effect paragraph.** Diagnosed 2026-07-25 (round nextcast,
measured not assumed — `recovery-harness/out/nextcast-mechanic-report.md`
"Diagnosis correction"): a trigger sentence parses when it is the first
sentence of a standalone effect, but the byte-identical sentence yields
`NoCompleteParse` when preceded by an activation-cost colon (`{T}: …`), a
loyalty-cost header (`[−2]: …`), an ability-word or Saga-chapter header, or
simply another sentence in the same paragraph. Reproduced `next`-independently
with minimal pairs: `"When you next cast a creature spell, copy that
spell."` standalone → clean; `"Draw a card. When you cast an artifact spell
this turn, that spell gains sunburst."` → trigger span fails; same for
loyalty-prefixed. Known blocked faces from the nextcast family alone (15 of
39 rows): Chandra the Firebrand, Cursed Recording, Ether, Ral Storm Conduit,
Rimefire Torque, Solar Array, Sword of Wealth and Power, Howl of the Horde
(2nd ability), Najal, Kumano Faces Kakkazan, Gadwick's First Duel (Saga
chapter III), Summon: G.F. Cerberus ×2 (also catalog-blocked on
Double/Triple). The corpus-wide family is presumably much larger — every
mid-paragraph delayed trigger. Diagnosis entry point: whatever segmentation/
grammar path builds trigger conditions apparently only predicts them at
paragraph-initial position; this smells like the invariant-repair class
(an under-predicted position) rather than a new construction, but that is
unverified. High-value candidate for the next diagnosis round.
