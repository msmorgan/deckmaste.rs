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
