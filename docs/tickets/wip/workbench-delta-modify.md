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

## As landed

- **One sort.** `Words.Delta : Type -> Type` (`Up | Down | Set`) is the single
  numeric-change sort; `Words.Counter.Delta`, `Phrase.LifeOp`
  (`LifeUp`/`LifeDown`/`Set`) and `Effect.PtShift` (`PtUp`/`PtDown`) fold into
  it. `Phrase.deltaIntro`/`deltaDelta` replace `lifeIntro`;
  `Effect.shiftAmount`/`shiftRises`/`shiftDelta`/`shiftIntro`/`sameDirection`
  are gone. D-Q10: `Up (Lit 0)` and `Down (Lit 0)` both stay writable — "+0"
  and "-0" are both printed ("gets -2/-0") — so the fold gives them one sort,
  not one spelling; no zero guard was added.
- **One characteristic-change row.** `Effect.StaticSpec.Modify (n)
  (what : Stat) (d : Delta (Amount …))` [CR#613.4c] replaces `Gets`;
  `ptOpOk`/`ptOpKind` go. `deltaKind` sends `Set` to `BasePtSet` [CR#613.4b]
  and `Up`/`Down` to `PtDelta`; `modifyStatOk` admits `Power` and `Toughness`
  only. A one-shot is `Continuously (getsPt …) span`; "+1/+1" is two `Modify`s
  under one `AndAlso`. `Becomes` is unchanged.
- **G7 and D-Q11.** `Effect.CharOp` is now `QualityOp`, `Becomes`'s operator
  alone — no value is dead on it. Archpriest of Iona's power-only ability is
  benched as `DefinesPt Macros.thisCreature PowerAlone Macros.partySize`
  (`Cards/Description.idr`), not as a base set; see Deviations 3.
- **Life.** `Effect.ChangeLife (who) (d : Delta (Amount (nomIntro who)))` is
  the one instruction row; "gains 3 life" is `Up`, "loses 3 life" is `Down`,
  "your life total becomes 20" is `Set`. `instrProfile` publishes
  `LifeGained` for `Up` and `LifeLost` for `Down` — the printed verb names the
  event [CR#119.3] — and nothing for `Set`, whose gain or loss follows from
  arithmetic on the new total [CR#119.5], which the engine does.
- **Counters keep their own lane.** `Effect.PutCounters`/`RemoveCounters` are
  untouched. `Words.CounterKind.BoostCounter` carries `Delta Nat` with
  `So (counterShift pow && counterShift tou)`: a counter kind adds or
  subtracts, never sets [CR#122.1a].
- **Macros, one per lemma.** `getsPt` (the static row), `gets`
  (`Continuously (getsPt …) span`), `getsBase` (replaces `hasBasePt`),
  `gainsLife`, `losesLife`, `lifeBecomes` (renames `lifeTotalBecomes`),
  `payLife`; `plusOnePlusOne`/`minusOneMinusOne` keep their counter-kind
  meaning. New helpers `sameWindow` and `itsOther` read the subject back for
  the second characteristic without re-announcing it.
- **Pins.** Added `badSetBoostCounter` with twin `okBoostCounterKind`
  (`ProofsCounters`). Re-spelled `badSetBasePtDownward` →
  `badModifyManaValue` and `badLosePtOp` → `badSetLoyalty`
  (`ProofsStatic`, twin `okAddPtUpward`, now
  `Modify Macros.thisCreature Power (Up (Lit 1))`). All three probed
  non-vacuous. Nothing was left undone.

## Landing record

Numbers before → after:

- modules 46 → 46; `Unspellable` pins 647 → 648.
- `Gets`, `PtUp`, `PtDown`, `PtShift`, `CharOp`, `LifeUp`, `LifeDown`,
  `ptOpOk`, `shiftRises`, `shiftDelta`, `shiftIntro`, `ptOpKind`,
  `sameDirection`, `hasBasePt`, `lifeTotalBecomes` in `idris/src/Experimental/`:
  all → 0.
- `Modify` 0 → 131 sites; `Macros.getsPt` 64, `Macros.gets` 102,
  `Macros.getsBase` 4, `Macros.itsOther` 45, `Macros.gainsLife` 83,
  `Macros.losesLife` 60, `Macros.lifeBecomes` 8.
- diff 28 files, +510 / −436.

Gates:

- `cd idris && ./scripts/build` → `46/46: Building Cards (src/Cards.idr)`;
  0 Error, 0 Warning; clean build 1m14.9s.
- `cargo xtask cite check --list-noncompliant` →
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` →
  `checked 14396 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` →
  `audited 10 citation site(s)`; each rule text read against its claim. No
  `cite bless` was needed — every rule cited this round was already in
  `cr-citations.lock`.

Performance advisory: `Experimental/Effect.idr` elaboration 4.33s → 4.15s
(`idris2 --find-ipkg --check src/Experimental/Effect.idr` with dependencies
pre-built; the "before" figure measured on a restored copy of the parent
tree). No movement worth reporting.

Assurance counts: restored 0; re-spelled 29 pin definitions (2 of them with
subjects the ticket retires, renamed), 161 bench witnesses, 23 `Proofs*`
positives and twins, 17 core/macro definitions; ignored with blockers 0;
added 2 definitions (`okBoostCounterKind`, `badSetBoostCounter`); removed 0.

Deviations and additions:

1. `Words.Counter.Delta` folded into `Delta Nat` (the ruling names the P/T and
   life sorts but not the counter payload). Reason: the ruling's "one sort …
   shared by every numeric change", and it is what gives the ticket's "a `Set`
   on a counter kind" pin a subject at all. `PutCounters`/`RemoveCounters`
   and loyalty are untouched.
2. `Effect.CharOp` renamed `QualityOp` — the done-when greps for `CharOp`, and
   the type survives only as `Becomes`'s operator.
3. Archpriest of Iona is benched as `DefinesPt … PowerAlone …`, not as
   `Modify … Power (Set …)`. D-Q11's premise does not hold on the tree:
   `DefinesPt` has always carried `PowerAlone` (`Cards/Turn.idr` already
   benches it), so the power-only ability was never unspellable; only
   `Gets Sets` forced both slots. "…'s power is equal to …" is a
   characteristic-defining ability [CR#208.2a] and applies in layer 7a, while
   a `Set` delta is the base-set sublayer 7b [CR#613.4b], so spelling it as a
   `Set` would file it in the wrong layer. The fold's real gain stands:
   `getsBase` no longer forces an invented toughness onto a power-only
   ability.
4. New macro helpers `sameWindow` and `itsOther`. With one property per
   `Modify`, "+1/+1" is two rows and the second must read the same subject
   without announcing it again; `itsOther` is a window-scoped read over
   exactly what the first row introduced. Where the subject is itself a read
   (`It`, `ownSubject`) the second row repeats that read; where the subject
   announces nothing and is not a read (`AsMarker TokenMarker This`) the noun
   is repeated.
5. `Macros.gets` keeps its name beside the new `getsPt`: `gets` is the
   instruction ("gets +1/+1 until end of turn"), `getsPt` the static row they
   share. The pair mirrors `gains`/`Gains`; it is not an agreement or arity
   pair for one lemma.
6. `Effect.sameDirection` deleted with the fold (a `PtShift` helper with no
   callers). `Effect.writtenZero` kept — it is `Amount`-typed and untouched.
7. Two witnesses expanded out of `Macros.gets` because their subjects cannot
   be read back by a pronoun: `nestingDragonInnerToken` (`AsMarker
   TokenMarker This` introduces no binding, so the noun is repeated) and
   `deadlyDancerPump` (`EachOf (Both …)`, whose second row re-reads the target
   as `It OneOf` rather than announcing it twice).
8. `ProofsStatic.okSingletonCoordination` is now the genuine one-part
   coordination `AndAlso Nothing [Modify (target creature) Power (Up (Lit 1))]`
   ("Target creature gets +1/+0"): under the fold "+1/+1" is two parts and
   would no longer be the singleton that twin exists to admit.

STOP recorded (resolved in-round, flagged for review): Deviation 3 resolves
the ruling's illustrative claim about Archpriest of Iona against the tree,
because the claim's premise is false and its spelling would file a
characteristic-defining ability in the base-set sublayer. The ticket's
done-when ("Archpriest of Iona's power-only ability is benched") is met either
way; if the conductor wants the ruling's literal spelling instead, it is the
one line `Static (Modify Macros.thisCreature Power (Set Macros.partySize))` in
`Cards/Description.idr`.
