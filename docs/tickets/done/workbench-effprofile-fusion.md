---
needs: [workbench-pin-hygiene]
---
**Finish the `Effect` table fusion or remove `EffProfile`.** Residue of
`workbench-table-machinery` (2026-09-03): the round added `Effect.EffProfile`
but the nine per-constructor tables it was meant to replace remain the
authority (the full fusion pushed one module past the five-minute build
watchdog). A record beside the tables is more surface, not less. Either land
the fusion incrementally (one table per build, measuring `Effect`'s check
time each step, stopping at the last step that stays under the watchdog) or
delete `EffProfile` and record why the fusion is not affordable.

Size: M. Done when: no projection of `Effect` exists twice (record and
table), build 23/23 under the watchdog, `Effect` check time reported before
and after. Standard constraints apply.

## As landed

The fusion landed incrementally, one table per build, stopping one step before
the blow-up. `EffProfile` is now the authority for the three tables it carries
and carries nothing else, so no projection of `Effect` exists twice.

| table | fate | `Effect` check |
| --- | --- | --- |
| — | baseline (record beside eight tables) | 2.464 s |
| `preIntro` (85 clauses) | **fused** as `pre` | 7.426 s (with `annIntro`) |
| `annIntro` (85 clauses) | **fused** as `announced` | 7.426 s |
| `deedDelta` (83 clauses) | **fused** as `deed` | 10.9 s |
| `effIntro` (90 clauses) | **kept** — fusing it took the check to 235 s (21x); reverted | — |
| `reflexEncloseUse` (51) | **kept**, field `enclosure` deleted | — |
| `costActionOk` (61) | **kept**, field `agentive` deleted | — |
| `heldUntilOk` (4) | **kept**, field `moved` deleted | — |
| `thisWayOutcomeOk` (4) | **kept**, field `scheduled` deleted | — |

`effIntro` is the stopping point: its pattern list splits `Choose`, `Pay` and
`Enact` where the fused spine does not, so folding it in takes the table to 91
clauses and each index-level reduction of `preIntro`/`annIntro`/`deedDelta`
through the record then drags four fields instead of three. That step was
reverted, not kept under the watchdog, because a 21x step is the blow-up the
ticket says to stop at.

The four predicate tables are kept and their record fields deleted rather than
fused: `%default total` wildcard fallbacks make them 4, 4, 51 and 61 clauses,
and a per-constructor record field would re-expand each to 87 explicit
clauses. That expansion is what pushed `workbench-table-machinery`'s full
fusion past five minutes. Deleting their fields is what removes the second
projection.

## Landing record

- Change: `mnrxkqtl`.
- Build gate: `23/23: Building Cards (src/Cards.idr)`; 23 modules, 0 Error
  lines, 0 Warning lines.
- Clean-build wall time (`rm -rf build && ./scripts/build`): before 39.825 s;
  after 47.981 s (also measured 51.999 s and 50.130 s across the round).
  Watchdog is 300 s.
- `Effect` check time (`rm build/ttc/*/Experimental/Effect.tt[cm] && idris2
  --check --source-dir src src/Experimental/Effect.idr`): before 2.464 s;
  after 10.885 s. Intermediate steps: 6.818 s (`pre`+`announced`, values
  written twice), 7.426 s (same, sharing via `sameIntro`), 10.644 s
  (+`deed`), 235 s (+`introduced`, reverted).
- `Effect.idr`: 2636 lines before, 2492 after.
- Citation gates: `0 non-compliant citation-looking string(s)`; `checked 17798
  citations against cr.txt (eff. 2026-08-07); 0 stale`; `audited 0 citation
  site(s) — nothing selected`.
- Projection equivalence: `idris/scripts/check-effprofile-fusion BEFORE AFTER`
  reads both files, and for each of the 87 fused clauses evaluates each old
  table by first-match on the fused pattern and compares the field value
  textually — `preIntro: 85 old clauses -> pre of 87 fused clauses`,
  `annIntro: 85 -> announced`, `deedDelta: 83 -> deed`, `effProfile: 261
  projection values checked, all equal`. It also refuses if any projection is
  not a lone `f e = field (effProfile e)`, if an old clause is narrower than a
  fused clause that no earlier fused clause already shadows, or if any old
  clause is unreachable. Probed by hand: changing `Fights`' `pre` to
  `nomIntro a` and `Draw`'s `deed` to `[outcomeB LifeGained]` are both caught.
- Pin non-vacuity, re-probed through the fused projections (each perturbs one
  fused clause, then the file is restored): `Create`'s `announced` widened to
  include the token binding makes `badOtherwiseReadsIfArm` and
  `badConditionalArmAntecedent` "not a valid impossible case"; `May`'s
  `announced` widened to `mayIntro body did notd` makes
  `badSimultaneousReadsMayDeed` and `badSimultaneousReadsMayOutcome` possible;
  `Enact (Just s) v (Move …)`'s `announced` forced to `doesAnnIntro OneOf`
  makes `badDistributedAnnouncedDiscardSingular` possible. `pre` and `deed`
  are additionally re-checked positively by the build:
  `okUnlessManaCost` reads `It` in `Unless`' payer slot, typed at `preIntro`,
  and `ProofsAnaphora.otherwiseCtxIsThenBranchOnly` is a `Refl` proof that
  `otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e`.
- Assurance: restored 0; re-spelled 0; ignored 0; added 0; removed 0. No pin,
  twin, table assertion or bench card was touched.
- Deviations and additions: `EffProfile` lost the five fields that duplicated
  the tables kept as tables (`moved`, `scheduled`, `agentive`, `introduced`,
  `enclosure`) — they had no consumer anywhere in `src/` or `crates/`. The
  record declaration moved above the `mutual` block, because a projection
  named before the record's declaration point inside `mutual` reports
  `Undefined name pre`. `sameIntro b d = MkEffProfile b b d` was added so the
  74 constructors where `preIntro` and `annIntro` agree write their value
  once. Two spine clauses split to keep first-match semantics, taking 85
  clauses to 87: `Distribute`, because `deedDelta` splits on the divided verb,
  and `Enact (Just s)`, because `deedDelta` splits on `Move`; the second uses
  an as-pattern so the sub-effect is still passed, not rebuilt.
  `idris/scripts/check-effprofile-fusion` was added beside the previous
  round's `compare-effect-tables`. No citation was added or changed; no file
  outside `idris/` and this ticket was touched.
- STOP: none. The ticket authorised either branch, and the measured blow-up at
  `effIntro` selected the stopping point rather than raising a question.
