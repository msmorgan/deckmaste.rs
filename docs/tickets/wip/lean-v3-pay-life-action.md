---
needs: []
---
# "Pays life" is a payment: `payLife` becomes an Action Template over `pay`

Standard constraints apply. Provenance: row A1 of
[lean-v3-cr-structure-principle-audit](../done/lean-v3-cr-structure-principle-audit.md).

## Ruling (owner, 2026-10-22)

`payLife` becomes an Action Template, `pay (perform (loseLife amount)) .once`. There is no new
`Cost` constructor and no `actFacts` entry. The existing `Coe Action Cost := ⟨perform⟩` keeps
cost-slot call sites unchanged. The "pays life" pattern is rewritten to `does player payLife`, and
its two pins are updated. The implementer checks that in a cost slot the coerced form
`perform (pay (perform (loseLife N)))` neither double-counts nor changes the cost-paid readback
[CR#118.1,119.4,119.8].

Paying life is a cost payment that subtracts life [CR#119.4]; a player who can't lose life can't
pay it [CR#119.8].

## Verified (2026-10-22)

Paths are under `lean/Semantics/`.

- `Action.pay (cost : Cost) (times : PayTimes) (outcome : PaymentOutcome) (whose) (keyword)`
  (`Abilities.lean:1227`); Template `pay cost times (outcome := .paid)` (`Templates.lean:1575`).
  `.once` is `PayTimes.once` (`Words.lean:487-491`).
- `loseLife` is a Template, `.changeLife (.down amount)` with `amount : Amount`
  (`Templates.lean:1213-1214`). There is no `loseLife` Action.
- `Coe Action Cost := ⟨perform⟩` is at `Templates.lean:2029`, with `perform` at `:2026`.
- `payLife (amount : Nat) : Cost := .perform (.changeLife (.down (.lit amount)))`
  (`Templates.lean:1779-1781`).
- `actFacts` (`Check/Words.lean:142-212`) is keyed by keyword action and has no pay or perform
  row; `.pay` is already an event head and a deontic head (`Check/Events.lean:41,59`). No entry
  is needed, as ruled.
- The ruled tree already exists: *Font of Agonies* (`Cards/Counters.lean:827-830`) writes
  `does you (pay (loseLife anyAmount) .once)`, and `Proofs/Anaphora.lean:623-625`
  (`youPaidLifeThisTurn`) uses the same `pay` inside `did`.
- The pattern to rewrite is `afterLifePayment` (`Proofs/Trigger.lean:114`), pinned by
  `okLifePaymentPayerReadback` (`:117-118`) and `okLifePaymentThatMuch` (`:130`).
- No Lean site writes "can't pay life" or "life was paid" (*Verrak*, *Angel of Jubilation*) yet.

**Correction to the ruling's spelling.** `payLife` takes a `Nat` today, so `loseLife amount` does
not typecheck and "pays life" (`anyAmount`) has no spelling. The Template takes an `Amount`.
Literal call sites still elaborate through `OfNat`; `payLife 2` gives
`.pay (.perform (.changeLife (.down (.lit 2)))) .once .paid none none`.

## The cost-slot check (evidence from verification)

Evaluated in Lean during verification (not yet pinned):

- `Cost.perform` steps its Action at `.postEvent` (`Check/Phrase.lean:3823`). `Action.pay` steps
  `whose`, then its cost (`:3385-3393`), and pushes `repeatCount` only when `times != .once`
  (`hasRepeatedPayment`, `:1540-1546`). So the coerced tree pushes one `lifeLost` outcome and no
  `repeatCount`. `Cost.localChecks` (`Check/LocalShape.lean:72-75`) and the number slots
  (`Check/NumberSlots.lean:112,197`) add nothing.
- The rewritten `afterLifePayment`, `does (a isPlayer) (payLife anyAmount)` read back as
  "that player", and `gainLife thatMuch` after it all check `[]`. So do `activated (payLife 2)`,
  `compound [tapSymbol, payLife 2]`, `ward (payLife 5)`, `cumulativeUpkeep (payLife 2)`,
  `doUnless … (payLife 1)` and `may (payLife 1) …`.

**Decide in the landing: the doubly wrapped pay.** Where a `Cost` is itself wrapped in `.pay`,
the tree becomes `pay (perform (pay (perform (changeLife …))))`. The checker accepts it, but it
says "pay a payment of life". The sites are `doUnless` (`Templates.lean:1390-1391`; *Carnophage*,
`Cards/Choice.lean:98`), ward's `pay (.inCaller cost)` (`Definitions/Keywords.lean:1647`;
*Prismari*, `Cards/Keyword.lean:181`), cumulative upkeep (`Definitions/Keywords.lean:83-84`;
*Glacial Chasm*, `Cards/Deontic.lean:465`), and the raw `.pay (payLife …)` in the pins below.
Either have `pay` (Template) not re-wrap a cost that is already a payment, or write `loseLife`
at those sites. Do not leave a double payment to be read as two. Pin the choice.

Pre-existing and unchanged: "You may pay 2 life. If you do, gain that much life" refuses
`quantOutcomeInScope 0` in both the old and the new form. It is not this ticket's defect; if the
landing touches it, ticket it.

## Change

1. `Templates.lean:1779`: `payLife (amount : Amount) : Action := pay (perform (loseLife amount))
   .once`, documented with [CR#119.4,119.8].
2. `Proofs/Trigger.lean:114`: `afterLifePayment` becomes `does (actor player) (payLife anyAmount)`.
3. Resolve the doubly wrapped pay (above).
4. Same-meaning rewrites, optional but preferred: *Font of Agonies* (`Cards/Counters.lean:828`) to
   `does you (payLife anyAmount)`; `Proofs/KeywordArguments.lean:61`
   (`costCannotReadOuterChoice`, "pay life" written as `perform (loseLife …)`).
5. ADR `docs/decisions/semantics-v3.md:409` maps `paysLife` to `pay(.life amount, .paid)`, which
   no code carries; rewrite it to the Template. Check the `loseLife` note at `:415`.

Cost-slot call sites keep their source and change only their tree: *Erebos*
(`Cards/Cost.lean:42`), *Force of Will* (`:572`), *Demon of Death's Gate* (`:582`), *Boseiju*
(`Cards/Mana.lean:913`), `Cards/Choice.lean:98,303,603`, *Yawgmoth's Bargain*, *Snuff Out* and
*Glacial Chasm* (`Cards/Deontic.lean:374,449,465`), *Rouse* (`Cards/Static.lean:546`) and
*Prismari* (`Cards/Keyword.lean:181`).

`lean-v3-rules-and-card-fidelity` (its discard/unless-pays bullet) asks to "generalize `payLife`
to `Amount`" so that *Killing Wave*, *Heat Wave* and *Font of Agonies* use it. Step 1 does the
generalisation; leave those three cards to that ticket unless they are trivial here, and say which.

## Tests

- Rewrite `okLifePaymentPayerReadback` and `okLifePaymentThatMuch` over the new
  `afterLifePayment`. They pinned the collapsed tree (losing life), which the ruling replaces.
- Rewrite `okMatchedPayer` (`Proofs/Choice.lean:136-139`), `okActorMatchedPayer`
  (`Proofs/Actor.lean:218-223`) and `okPayLifeCost` (`Proofs/Deontic.lean:77-79`), which write
  `.pay (payLife …)`, to the form the doubly-wrapped-pay decision settles.
- Keep `okOwnPayerCost` (`Proofs/Keyword.lean:293-296`), `okActorOwnPayerCost`
  (`Proofs/Actor.lean:227`) and `ownPayerCostHandoffTwin` (`:370-377`) passing unchanged.
- Add `payLifeIsAPayment`: `payLife N` is `pay (perform (loseLife N)) .once` (`rfl`).
- Add `payLifeInACostSlotCountsOnce`: in an activated ability's cost, `payLife 2` followed by
  "that much" in the effect reads one life amount (one `lifeLost` outcome).
- Add a deontic pin, `cant (a isPlayer) (payLife anyAmount)` checks clean and is distinct from
  `cant … (loseLife …)` (*Angel of Jubilation*'s "can't pay life" must not forbid a
  *Lightning Bolt*'s life loss).

## Other tickets on the same files

- `lean-v3-becomes-blocked-status` also edits `docs/decisions/semantics-v3.md` (rows `:307,411`).
- `lean-v3-cause-flag-fold` touches `Templates.lean`, `Proofs/Trigger.lean` and
  `Proofs/Counters.lean` widely; land one after the other.
- `lean-v3-alternative-cost-unification` edits *Force of Will*'s and *Snuff Out*'s neighbours and
  the keyword bodies; no shared lines.
