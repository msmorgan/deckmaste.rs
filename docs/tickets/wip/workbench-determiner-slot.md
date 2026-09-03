---
needs: [workbench-gate-dedup, workbench-pronoun-read-index]
---
**Make the determiner a slot: one `Described` noun over a `DetPhrase`, and
delete the six determiner constructors.** Ruling 2026-09-02 on audit item R1.

`Phrase.Noun:1491-1506` has `Each p`, `Indefinite mode p`, `Definite p
{Uniquifying}`, `TargetGroup q p`, `CountedGroup q mode p` and `AllOf p`: six
constructors over the `Determiner` axis, each carrying its own extras
(quantity, choice mode, uniqueness proof). The cost of the split is the 13×6
clause block that `workbench-gate-dedup` papers over with a `nounDet`
projection, plus a `Determiner` type that exists only to fill `Binding.det`.

## The ruling

The determiner is a **slot**, not a constructor. v1 also splits
(`TargetSpec`/`Selection::SelectAll`/`Choose`/`Each`), so the mirror ruling
does not decide this one; the decision is made here on the clause cost.

```idris
data DetPhrase bs = TargetDet (Quantity bs) | ADet (ChoiceMode bs)
                  | EachDet | AllDet | TheDet
                  | CountDet (Quantity bs) (Maybe (ChoiceMode bs))

Described : (d : DetPhrase bs) -> (p : Predicate bs k)
         -> {auto ph : Phrasal k} -> {auto 0 ok : detOk d p} -> Noun bs k
```

The per-determiner extras move into their `DetPhrase` constructors, and the
per-determiner side conditions (`Definite`'s uniqueness proof, `TargetGroup`'s
and `CountedGroup`'s quantity gates, `Indefinite`'s choice mode) become
branches of the single `detOk d p`. `nounDet` from `workbench-gate-dedup`
becomes a field read on `Described`, and the shape predicates derived from it
become one-liners over that read.

Size: M.

Done when: build is 23/23; `Each`, `Indefinite`, `Definite`, `TargetGroup`,
`CountedGroup` and `AllOf` are gone from `Phrase.Noun`; `detOk` is the single
site of every per-determiner side condition; the bench witnesses for each of
the six are re-spelled through their macros and still typecheck; the uniqueness
pin that `Definite` carried refutes through `detOk TheDet` and remains
non-vacuous. Standard constraints apply.

## As landed

- Slot: `Phrase.DetPhrase bs` (`TargetDet q`, `ADet m`, `EachDet`, `AllDet`, `TheDet`, `CountDet q mode`) and one `Described : (d : DetPhrase bs) -> (p : Predicate bs k) -> {auto ph : Phrasal k} -> {auto 0 ok : detOk d p} -> Noun bs k`; `Each`, `Indefinite`, `Definite`, `TargetGroup`, `CountedGroup`, `AllOf` are gone from `Noun` and from every file under `idris/src/Experimental/`.
- `detOk` is the single side-condition site: `TargetDet q` → `(NonZeroQ q, WellFormedQ q, Targetable k)`, `CountDet q _` → `(NonZeroQ q, WellFormedQ q)`, `TheDet` → `Uniquifying p`, the rest `()`. `TargetGroup`'s unerased `tk` became the erased `Targetable k` conjunct; `nounDelta` binds the target row through `ph` where it read `targetablePhrasal tk` (the same `Phrasal` per kind).
- Projections `detOf`, `detPlur`, `detQuant`, and `detDelta` (three clauses: the two quantity-bearing rows, then `bindFor (detOf d) (detPlur d)`); `nounDet (Described d _) = Just (detOf d)`. The nine derived shape predicates are unchanged in text. `detQuant` sits beside `quantPlur` because `Quantity`'s header is declared after `Noun` in the `mutual` first pass.
- `TokenPhrase.CountedTokens`/`OneToken` index `Described (CountDet q m) p` / `Described (ADet m) p` with an erased `dk : detOk …`; `Effect.twoPartiesOk` reads `detQuant d >>= quantExact`; `nounRegime`, `effChoiceDelta` re-spelled.
- Macros: `target` is `Described (TargetDet (exactly 1)) p` with `{auto ph}` and an erased `{auto 0 tk}`; new `targets q p`, `each p`, `allOf p`, `the p`, `counted q p` forward `detOk` as their one auto gate; `a`, `aTheirChoice`, `aYourChoice`, `aAtRandom`, `countedAtRandom` re-pointed; the deontic counterparts and `proliferate` use `allOf`/`counted`.
- Bench and pins re-spelled by script (word-boundary rename, `CountedGroup q Nothing` → `counted q`): Cards.idr `allOf` 334, `each` 118, `targets` 38, `counted` 57, `the` 13; Proofs* 60 sites. Doc-comment hits reverted. No witness removed.
- Pins re-spelled through `detOk`: `badDescendingRange` (`{wf = ok}` → `{ok = (MaxAtLeastOne, ok, ObjectTgt)}`), `badZeroGroup` (`(MaxAtLeastOne, _, _) impossible`), `badBareDefinite` (`Macros.the Macros.creature {ok}`, `Oh impossible`). Probed non-vacuous by mis-stating once, each refused "not a valid impossible case": `badDescendingRange` (ascending range), `badZeroGroup` (`exactly 1`), `badFightGraveyard` (battlefield zone), `badBareDefinite` (superlative predicate), `badChooseDefinite` (`Macros.a`), `badEachOfCountedGroup` (`targets`), `badEachOfDistributive` (`targets`), `badPartitiveOfDescription` (`allOf`).
- Clauses over the six (before → after): `nounEqRef` 6→1, `nounDelta` 6→1 (+`detDelta` 3), `nounDet` 6→1, `moveIntro` 6→1, `nounZone` 6→1, `nounTy` 6→1, `nounHeadTys` 6→1, `nounTys` 6→1, `nounPlur` 6→1, `Effect.nounRegime` 6→1, `Effect.twoPartiesOk` 2→1, `agentIntro` 1→1, `chosenDelta` 2→2, `testSubjectOk` 1→1, `slicePossessorOk` 1→1, `effChoiceDelta` 1→1, `TokenPhrase` 2→2; new `detOf` 6, `detPlur` 6, `detQuant` 3, `detOk` 4.
- Undone: `Words.Determiner` stays — `Binding.det` still needs `PartD`/`SelfD`, which no `DetPhrase` names. The `[CR#608.2]` note moved from `Each` to `EachDet` unchanged.
- Build timings (clean `rm -rf build && ./scripts/build`, sibling builds running): before real 1m34.4s / user 1m21.3s (load 3.6→22); after real 58.4s / user 55.3s (load 9→15). Incremental Phrase-onward rebuild 38s.

## Landing record

- Construction count: `Noun` 49→44 (six constructors → `Described`); `DetPhrase` +6; `Determiner` unchanged.
- Coverage and lock state: bench coverage unchanged, no witnesses removed; `cr-citations.lock` unchanged (`bless` re-registered 1569 rules, no diff).
- Assurance: restored 0; re-spelled 3 pins (holes) plus the scripted renames; ignored 0; added 0; removed 0.
- Positive artifacts: 23/23 Idris build, no Warning lines; `cite check --list-noncompliant` 0; `cite check` 0 stale of 17915; diff audit 1 site (`[CR#608.2]`, moved).
- Deviations and additions: `target` gains `{auto ph : Phrasal k}` (`Described` needs it for every determiner); `Macros.the` shadows Prelude's `the` by name, used qualified from Cards/Proofs; `detQuant` placed in the `Quantity` section rather than beside the other `det*` projections.
- STOPs: none.
