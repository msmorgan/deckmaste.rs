---
needs: [workbench-determiner-slot]
---
**Probe unification first, then make `Words.Kind.(\/)` a normalising join and
fold the four coordination rows to two.** Ruling 2026-09-02 on audit item R2 —
conditional, with the probe as task one.

`Phrase.Noun:1512-1521` carries same-kind and cross-kind pairs for each
connective solely because `Words.Kind.(\/):226` is a free constructor: `Object
\/ Object` is not `Object`, so a same-kind coordination cannot return `Noun bs
Object` through the cross-kind row.

## The ruling

`\/` **may** normalise (idempotent on equal kinds) — **gated on a probe**.

**Task one: the probe.** A function in an index position can stop unification
solving neighbouring implicits (`so-synonym-loses-index-inference` note). Probe
on `Both You n` (`Macros.youAnd:47`): make `\/` normalising and check that the
`youAnd` call sites still elaborate with their implicits solved, without extra
annotations at the bench.

- **Probe passes** → normalise. `Both`/`BothOf` and `EitherOf`/`EitherJoined`
  become two rows (`Both`, `EitherOf`), and `EachOfBoth:1520` folds into
  `EachOf:1509`.
- **Probe fails** → do not touch `\/`. Fall back to folding **only**
  `EachOfBoth` into `EachOf`: their gates (`CoordinatedPair` and
  `GroupMention`) are both "a plural to distribute over". Record the probe's
  failure mode on the declaration so it is not re-attempted blind.

Size: M.

Done when: build is 23/23; the probe result is recorded with the concrete
elaboration evidence either way; `EachOfBoth` is gone in both branches, and on
the passing branch `BothOf` and `EitherJoined` are gone too; the coordination
bench witnesses are re-spelled and still typecheck with no added annotations;
the coordination pins in `Proofs*` are re-spelled and remain non-vacuous.
Standard constraints apply.

## As landed

- Probe, computed join: `Both : … -> Noun bs (joinKind ka kb)` with `joinKind a b = joinKindBy (a == b) a b` (`joinKindBy True a _ = a`, `joinKindBy False a b = a \/ b`) built through `Words`, `Events`, `Phrase`, `Triggers`, `Effect`, `Card` and `Experimental`, then failed in `Macros` with: `Error: scry is not covering. Experimental.Macros:1550:1--1553:59 … Missing cases: scry (Both m m) arg / scry (Both m m) arg` and the same for `surveil`. At a fixed-kind match the checker cannot refute `Both m m = You` under a stuck index, so the `{ay = Refl}` branches of `scry`/`surveil` lose coverage. Reverted; `Words.(\/)` stays a free constructor.
- Probe, relation: `Words.Joins : Kind -> Kind -> Kind -> Type` with `JoinSame : Joins k k k` and `JoinDiff : {auto 0 ne : So (not (a == b))} -> Joins a b (a \/ b)`; `Both` and `EitherOf` take `{auto jk : Joins ka kb k}` and return `Noun bs k`, so the index is a variable and the join is decided by search. Every `Macros.youAnd` site, the `UnionHalf` reach cases and `Macros.thatSplitController` elaborated unchanged; 23/23 with no bench edit. Passed.
- `BothOf` → `Both`: 11 bench sites and `badExchangePluralParty`, `badSharedSubjectTwoInDelta`, `badOwnTwoInDelta`; `Effect.twoPartiesOk` matches `Both`; `nounZone`/`nounTy` of `Both` take the equal-halves rule; `nounTys` splits on `jk` (`JoinSame` → `SoleTy (nounTy n)`, `JoinDiff` → `JoinTy`).
- `EitherJoined` → `EitherOf`: 3 bench sites (Archangel of Tithes, Archon of Absolution, Blood Reckoning) through new `Macros.youOr : Noun bs Object -> Noun bs (Player \/ Object)`; the `ag` plurality-agreement gate is dropped and `nounPlur` uses the `samePlur` rule (no pin covered `ag`; the cross-kind row already admitted mixed plurality).
- `EachOfBoth` → `EachOf`: 3 bench sites (Secret Rendezvous, Mana Clash, Alluring Suitor); `groupMention (Both _ _) = True`; `coordinatedPair`/`CoordinatedPair` deleted.
- Annotation: `nounDelta {k = Object}` in `badSharedSubjectTwoInDelta` — a bare `nounDelta (Both …)` has no expected kind and the `Joins ka kb ?k` search is postponed while `k` is a hole (the same reason the `EitherOf You …` bench sites read through `youOr`, which fixes the kind as `youAnd` does). No bench annotation.
- Pins probed non-vacuous, each refused as "not a valid impossible case" once mis-stated: `badExchangePluralParty` (`target Opponent`), `badSharedSubjectTwoInDelta` and `badOwnTwoInDelta` (single `target creature`), `badEachOfDistributive` (`Both (target creature) (target artifact)`), `badNestedEachOf` (inner `EachOf` dropped), `badEachOfSingular` (`Both …`).
- Undone: nothing.
- Timings (clean `rm -rf build && ./scripts/build`, sibling builds running): before real 1m13.2s / user 1m06.0s; after real 1m23.8s / user 1m09.1s.

## Landing record

- Construction count: `Noun` 32 → 29 (`EitherJoined`, `BothOf`, `EachOfBoth` gone); `Words.Joins` +1 data (2 constructors); `Macros.youOr` +1; `coordinatedPair`/`CoordinatedPair` deleted.
- Coverage and lock state: printed-card bench coverage unchanged, no witnesses removed; `cr-citations.lock` unchanged (`bless` re-registered 1569 rules, no diff).
- Assurance: restored 0; re-spelled 3 pins and 15 bench lines; ignored 0; added 0; removed 0.
- Positive artifacts: 23/23 Idris build, no Warning lines; `cite check --list-noncompliant` 0; `cite check` 0 stale of 17920; diff audit 0 sites.
- Deviations and additions: the join is the `Joins` relation, not a function in the index (the computed form fails coverage, above); `Macros.youOr`; `EitherOf` loses `ag`; one `{k = Object}` in a pin; a cross-kind `Both` now reports a shared zone/type where its halves agree.
- STOPs: none.
