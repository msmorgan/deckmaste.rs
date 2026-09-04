---
needs: []
---
**One `Delta` for every numeric change: one `Modify` static row, one life
instruction row.** Ruling 2026-09-04 on deltas (cleanroom review 3, finding
G7; D-Q10 and D-Q11 fold in).

- **One sort.** `Words.Delta = Up a | Down a | Set a`, shared by every numeric
  change. `Phrase.PtUp`, `PtDown`, `PtShift`, `Effect.CharOp`, `Effect.Gets`,
  `Phrase.LifeUp` and `LifeDown` fold away. `PtUp (Lit 0)` and `PtDown (Lit 0)`
  stop being two spellings of "-0" (D-Q10 — `Cards.Static` writes both).
- **One characteristic-change row.** `Effect.StaticSpec.Modify (referent)
  (property) (delta)`, named for the layer that applies effects modifying
  power and toughness [CR#613.4c]. A one-shot "gets +1/+1 until end of turn"
  is `Continuously (Modify …) span`; "+1/+1" is two `Modify`s combined by a
  macro. `Effect.Becomes` stays for non-numeric characteristics.
- **G7 and D-Q11.** `Effect.CharOp` is shared by `Gets` and `Becomes` with a
  value dead on `Gets` (`ptOpOk Loses _ _ = False`), and `Macros.hasBasePt`
  spells "has base power and toughness" as `Gets Sets n (PtUp pow) (PtUp tou)`
  — a set written as two rises, policed by `shiftRises`. With `Set` in `Delta`
  and one property per `Modify`, a base set is `Set` and the power-only
  characteristic-defining ability of [CR#208.2a] ("[This creature's] power is
  equal to …", `Cards.Description.archpriestOfIonaPower`) becomes spellable
  without an `Unchanged` slot. `ptOpOk` and `shiftRises` go with the fold.
- **Life is not a continuous effect.** One `Effect.Instruction` row over the
  same `Delta`: "gains 3 life" is `Up`, "loses 3 life" is `Down`, "your life
  total becomes 20" is `Set`. The engine classifies gain, loss and no-op from
  the resulting total; macros never do arithmetic.
- **Counters keep their own lane.** `Effect.PutCounters` and `RemoveCounters`
  are unchanged, loyalty included.
- Re-spell every bench and pin site through the folds, as printed, and report
  the assurance counts.

Size: L. Done when: `grep` finds no `PtUp`, `PtDown`, `PtShift`, `CharOp`,
`LifeUp`, `LifeDown` or `Gets` in the tree; `Modify` is the only
characteristic-change row and one instruction row carries every life change;
Archpriest of Iona's power-only ability is benched; every re-spelled pin is
probed non-vacuous; build at its module count. Standard constraints apply,
including the RON-shaped constraint.
