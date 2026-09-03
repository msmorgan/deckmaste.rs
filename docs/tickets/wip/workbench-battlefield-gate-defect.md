---
needs: []
---
**Pick one battlefield gate per row, by a recorded decision about whether a
placeless subject is admissible.** Correctness defect, 2026-09-02 workbench
audit (F16). This is the first item in every chain below that touches nouns.

The gate is told two ways and the two disagree:

- `Words.OnBattlefield:2337` — `OnField : OnBattlefield (Just Battlefield)`
  is the only constructor, so it **refuses** a placeless noun. 14 sites in
  `Effect.idr`.
- `ZoneFits (nounZone n) (Just Battlefield)` — `Words.zoneFits:2416` has
  `zoneFits Nothing _ = True`, so it **admits** a placeless noun. 39 sites
  across `Effect`/`Phrase`/`Triggers`.

Neighbouring rows pick different forms with no pin telling them apart:
`SetStatus:1088`, `TurnOver:1091`, `Fights:1080` use `OnBattlefield`;
`Regenerate:1119`, `Gets:236`, `Dies:193` use `ZoneFits` (proofs catalog §1).

## Fix

For each of the 53 sites, decide whether the subject may be the self (`This`,
where `nounZone This = Nothing`) or must be a placed permanent. Then:
`ZoneFits` where placeless is admissible, `OnBattlefield` (or `So (onFieldZone
…)`) where it is not. Audit the 39 `ZoneFits … (Just Battlefield)` sites in
particular for rows that pass today only because the bench subject happens to
be `This` — those are the ones the decision actually moves.

Size: S.

Done when: build is 23/23 with 0 errors and 0 warnings; every `Just
Battlefield` gate site uses one of the two forms by a recorded per-row
decision; each row whose gate tightened carries an `impossible`-clause pin in
`Proofs*` refuting the newly-excluded subject, and each such pin is
non-vacuous — it stops compiling if the gate is loosened back. Standard
constraints apply.

## As landed

- Audited all 53 battlefield gate sites: 16 use `OnBattlefield`, 37 use `ZoneFits`; `Regenerate` and `DoorOf` tightened, while every source-admissible row retained `ZoneFits`.
- Re-spelled `badRegenerateInGraveyard` and `badUnlockDoorOffBattlefield` through `OnField`; the existing `drudgeSkeletons` and `ghostlyKeybearer` witnesses needed no re-spelling.
- Added non-vacuous `badRegenerateBareThis` and `badDoorOfBareThis` pins; no part was left undone.
