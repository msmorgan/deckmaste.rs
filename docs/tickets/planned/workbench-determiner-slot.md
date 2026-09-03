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
