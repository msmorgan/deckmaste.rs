---
needs: []
---
**Add `BareDet` as a determiner value that introduces a group binding, and
fold the four predicate/noun pairs it exists to work around.** Cleanroom
review 2026-09-03, R2, ruled: a bare plural *is* a determiner value.

`Amount.CountOf (p : Predicate)`, `Aggregate op ax (p : Predicate)`,
`AggregateOver`, `Condition.Exists (p : Predicate)` and `Happened`'s
complement take predicates rather than nouns because "creatures you control"
(no determiner) has no `DetPhrase` value — `DetPhrase = TargetDet | ADet |
EachDet | AllDet | TheDet | CountDet` (`Phrase.idr:1412–1418`). That gap is
why `CountOf p` / `CountOfGroup grp` and `Aggregate` / `AggregateOf` are
pairs.

## The ruling

`BareDet` is a determiner value: plural indefinite, plurality `ManyOf`. It
**introduces a group binding**, exactly as `AllDet` does — "creatures you
control … those creatures" reads back in printed text, so the binding is real
and the read must be available. This follows the settled "the determiner is a
slot" ruling; it is not an exception to it.

Then fold, each to the noun-taking member of the pair: `CountOf` /
`CountOfGroup`, `Aggregate` / `AggregateOf`, and `AggregateOver`. `Exists`
takes a `Noun`, and so does `Happened`'s complement.

Size: M.

Done when: the build is 23/23 with 0 errors and 0 warnings; `BareDet` is a
`DetPhrase` value with `ManyOf` plurality and a group binding in `detDelta`;
`CountOfGroup`, `AggregateOf` and `AggregateOver` are gone and their former
sites read through the surviving noun-taking constructors; `Exists` and
`Happened`'s complement take a `Noun`; a bench witness reads a bare plural
back through `those` in a later sentence; a pin refutes a singular read of a
`BareDet` group, and it is non-vacuous. Standard constraints apply.
