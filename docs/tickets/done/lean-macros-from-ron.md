---
needs: [plugins-v2-dialect]
---
**Generate `lean/Semantics/Macros.lean` from the `plugins_v2` declarations**
so the Lean macro layer stops being hand-written (`semantics-v2.md` §6
"eventually"). Not needed by the gate, which checks expanded terms. Parked
until the macro-body families land.

## Residue from `semantics-v2-macro-capture-and-plurality` (2026-10-07)

That landing answered capture and plurality in the loader (`semantics-v2.md`
§12.2) and retired the pronoun-baking helpers, but did not port the 21 macros
left in §12.1's "It computes" bucket one by one. Each still decides here: the
nine that build a pronoun window are inventory for
`lean-drt-anaphora-refactor`; the rest (`amass`, `chooseSpree`,
`dealDamageOwnPower`, `joinedHead`, `joinedHeadWhile`, `lookAndSort`,
`lookAndSortInto`, `lookedTop`, `loseCounters`, `modular`, `partyOf`,
`rollRow`, `scaledMana`) either port over the §12.2 devices or are re-spelled.
The RON `lookAndSortSlice` replaces `lookAndSort`/`lookAndSortInto` for the
keyword bodies; the bench still calls the Lean pair.

## Landing record

Numbers measured on `uuqznyso` unless stamped otherwise.

1. **Census delta**: no grammar or corpus change; no English census run. Lean
   gates unchanged: `lean check` 173/173 cards (165 canon, 8 testing),
   `lean definitions` 698/698. Lost: none.
2. **Witnesses**: the ticket letter, "generate `Macros.lean` from the
   `plugins_v2` declarations", is met as `cargo xtask lean macros generate`
   writing the tracked `lean/Semantics/Macros/Declared.lean` (975 declarations, 978 after the refresh onto trunk at `qnzzmvmz`,
   derived, 728 not: 697 registry nodes whose Lean form is `facts`', 9 metas,
   6 bodyless keyword actions, 10 `Type` declarations, and 6 helpers, each
   listed with its reason at the file's foot), `check` rejecting a stale copy
   (unit test `the_committed_module_matches_the_declarations`), and `align`
   proving hand macros equal to their generated namesakes by `rfl`: 372
   namesakes, 324 aligned, 48 drifted (31 loader device, 15 signature, 2 body).
   Residue (the 13 named here), re-read against today's `Macros.lean`, all
   still present: `amass` and `modular` have declarations and drift by
   signature (`Nat`/`String` against `Amount`/`Subtype`); `lookAndSort`,
   `lookAndSortInto` (and `lookedCards`) are superseded by the generated
   `lookAndSortSlice`, which `Proofs/BindingIdentity.lean` now reads instead
   of its hand copy, and retire when `scry`/`fateseal`/`surveil` cut over;
   `lookedTop` builds a checker `Bindings` context, not a phrase, and stays;
   `chooseSpree`, `dealDamageOwnPower`, `joinedHead`, `joinedHeadWhile`,
   `loseCounters`, `partyOf`, `rollRow`, `scaledMana` have no declaration.
   None names a retired node (`librarySlice`, `libraryRun`, `Lookback`,
   `HeaderPossessor`, `PartQuant`, `RankPeriod`, `TokenRider`,
   `SpecialAction`, `Cost.tapForGeneric`). Everything not done here is routed
   to `planned/lean-macros-cutover`.
3. **Spot checks** (generated, read against their RON): `target`,
   `yourHand` (`handOf` inlined), `procedure` (untyped binders typed
   `Procedure`/`PlayPayment`, `resulting : Option Predicate := none`),
   `addSubtype` (hole inside a structure literal), `createToken`
   (`count : Amount := lit 1`), `additionalPhase` (`followed_by` →
   `followedBy`), `doublePower` (`firstMention`/`laterMention` ids kept),
   `exileUntil` (`ownScope`/`inCaller` kept), `foodToken`, `lookAndSortSlice`
   (equal by `rfl` to the hand copy it replaced).
4. **Deviations and STOPs**: no STOP. Deviation: the hand layer is not yet
   replaced by the generated one (concurrent sibling edits to `Macros.lean`,
   and 48 drifts that need per-macro decisions); the generated file lives
   beside it in `Semantics.Macros.Declared` and the cutover is minted as
   `lean-macros-cutover`. Hand macros aligned here: `delay` (+`alternatives`),
   `doIf` (+`otherwise`), `putOntoBattlefieldTappedAttacking` (+`from_`), all
   trailing defaulted binders, so no call site changed. Two samples appended
   to `expansions::SAMPLES` (`ManaSubstitute`); appending is the list's own
   rule, and `expansions`/`facts` outputs are unchanged (`facts check` up to
   date). New untracked lake library `GeneratedMacroAlignment`, ignored.
   Glossary: no new domain term.
5. **Tests**: `cargo test -p xtask` 392 passed, 1 ignored (pre-existing), plus
   4 + 14 in the other targets; `cargo clippy -p xtask --tests -- -D warnings`
   clean; `cargo test -p deckmaste_semantics_v2` all pass (45/7/1/4/43);
   `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings` clean;
   `lake build --wfail` (91 jobs) succeeds; `cargo xtask facts check` up to
   date. Counts: restored 0, re-spelled 1 (`BindingIdentity`'s
   `lookAndSortSlice` fixture now the generated macro, with its hand text kept
   as the new pin `lookAndSortSlice_is_the_hand_expansion`), ignored 0, added
   5 (4 unit tests in `lean_macros`, 1 Lean pin), removed 0.
6. **Timing**: no census. `lean macros generate` 1m36s wall, `align` 2m21s
   wall, both on all cores (rayon / lake defaults).
