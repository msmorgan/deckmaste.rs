---
needs: []
---
**Publish `ForEachOf`'s group binding after the loop.** Ruling 2026-09-04
(cleanroom review 3, D-Q7).

- `Effect.instrIntro (ForEachOf grp body) = pluralizeDelta (instrDelta body)
  ++ bs` discards the group's own bindings, so after "for each creature you
  control, …" neither the group nor anything the element clause described
  about it is readable. Publish `nounDelta grp`, pluralised, alongside the
  body delta.
- `Effect.ForEachKindOf` carries the identical `KeepsOuter` obligation and was
  left at `instrIntro … = bs` by `workbench-choice-scope-residues`, which
  recorded it as a loop-delta gap rather than a separate shape. Apply the same
  rule there and unblock Celestial Judgment.
- Bench a printed card that reads the group after the loop, and pin a read of
  a body-local binding the loop does not publish.

Size: S. Done when: both loop rows publish the group and the body delta;
Celestial Judgment is benched; the pin probes non-vacuous; build at its module
count. Standard constraints apply, including the RON-shaped constraint.

## As landed

- `Effect.instrProfile (ForEachOf grp body)` now announces
  `pluralizeDelta (instrDelta body) ++ pluralizeDelta (nounDelta grp) ++ bs`:
  the group survives the loop as a plural, beside the body delta. The element
  binding `elemIntro` adds stays body-local.
- `Effect.instrProfile (ForEachKindOf _ dom _ body)` was `sameIntro bs []`; it
  now announces the same shape over the loop's own group,
  `pluralizeDelta (instrDelta body) ++ pluralizeDelta (maybe [] nounDelta dom)
  ++ bs`. That unblocks Celestial Judgment, benched whole as
  `Cards.Choice.celestialJudgment` — "Destroy each creature not chosen this
  way" reads the choices the `ValueAxis Power` pass published.
- Witness `ProofsAnaphora.okLoopGroupSurvives` (Vaevictis Asmadi, the Dire's
  opening loop, "Those players …") and pin
  `ProofsAnaphora.badLoopElementRead` ("That player …", the body-local
  element). Not done: no `Cards/` entry for the group read — see the STOP.
- Re-probed `ProofsZone.badLoopedZoneMoveRead`,
  `ProofsZone.badKindLoopZoneMoveRead` and
  `ProofsAnaphora.badSingletonForEach`; all three still non-vacuous.

## Landing record

Measured on change `xvsnkzzr`, against parent `swqrlxzz`
(`kata: claim workbench-foreach-group-survives`).

**Numbers before → after**

- Idris modules: 46/46 → 46/46.
- `Cards/*.idr` card witnesses: 810 → 811 (Celestial Judgment).
- `Cards/*.idr` bare `Instruction []` witnesses: 293 → 292
  (`celestialJudgmentPass` folded into the card).
- `Proofs*.idr` `Unspellable` pins: 609 → 610.

**Gates**

- `cd idris && ./scripts/build` (after `rm -rf build`): last line
  `46/46: Building Cards (src/Cards.idr)`; 0 `Error`/`Warning` lines; 1m37s.
- `cargo xtask cite check --list-noncompliant`:
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check`:
  `checked 14377 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `jj --no-pager diff --git | cargo xtask cite audit --diff`:
  `audited 0 citation site(s) — nothing selected` (the round adds no citation).
- `cargo xtask cite bless`: not run — no newly cited rule.

**Assurance counts**

- restored 0; re-spelled 1 (`celestialJudgmentPass` → `celestialJudgment`,
  same card, same asserted outcome, now the whole printed card); ignored 0;
  added 2 (`okLoopGroupSurvives`, `badLoopElementRead`); removed 0.
- Non-vacuity: `badLoopElementRead` mis-stated to `ManyOf` gives
  `badLoopElementRead Refl is not a valid impossible case`.
  `badLoopedZoneMoveRead`, `badKindLoopZoneMoveRead` and
  `badSingletonForEach` each give the same message when mis-stated
  (body that only adds; plural group).

**Deviations and additions**

- The ticket's "bench a printed card that reads the group after its loop" is
  met in `ProofsAnaphora`, not in `Cards/` — see the STOP.
- `ForEachKindOf`'s group is its `dom : Maybe (Noun bs Object)`; `Nothing`
  publishes nothing, which is the `kindValueIntro q Nothing = qualityB q :: bs`
  case read back.
- No comment or citation added to `Effect.idr`.

**STOP**

- Corpus sweep of `select(.supported)` for a `ForEachOf` whose group is read
  after the loop returns exactly one card: Vaevictis Asmadi, the Dire ("for
  each player, choose target permanent that player controls. Those players
  sacrifice those permanents."). With this round's change the group read now
  resolves, but the printed next clause is refused one obligation later, at
  `EachStackOk` — a plural agent's deed that moves an object bound outside the
  agent phrase. That is `Effect.Enact`'s agent lane, a sibling round's region
  and the open ledger item recorded by `workbench-loop-delta`
  ("`doesInstrIntro ManyOf` / `distributedDelta` still republish `nomIntro s`
  over a mutated stack"). Resolution: not touched here; the group read is
  benched as `okLoopGroupSurvives` with a readable tail, naming the card, and
  the refusal is left for that lane. Route at integrate: Vaevictis Asmadi is
  blocked on the plural-agent `EachStackOk` obligation, not on loop deltas.
