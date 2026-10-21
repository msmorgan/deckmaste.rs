---
needs: []
---
# `ExistentialBe` vs. `LocatedExistentialBe` duplication on postmodified displaced subjects

`ExistentialBe` (`there are <NP>`) and `LocatedExistentialBe` (`there are <NP>
<PP>`, `english.ron`) both match an existential clause whose displaced subject
is followed by a `PrepositionPhrase` that can also postmodify a `Nominal` (*in
your graveyard*, *on the battlefield*, *among permanents you control*):

- *As long as there are seven or more cards in your graveyard, this creature
  gets +1/+1.* reads *there are seven or more cards in your graveyard* twice:
  once via `ExistentialBe` with *in your graveyard* postmodifying *cards*, and
  once via `LocatedExistentialBe` with *in your graveyard* as the locative
  `complement` of `LocatedExistentialBe`.
- `english-v4-spurious-ambiguity-multipliers` excluded `ComplementKind = NP`
  from the `ExistentialPredicate` PP-adjunct schema (`english.ron`), but left
  `LocatedExistentialBe` overlapping with `ExistentialBe` whenever the trailing
  PP is a `Nominal`-hosted postmodifier, producing 1,049 surplus Readings
  across 338 Corpus Units (among units with at most 64 derivations alone; only
  *Wail of the Forgotten*, which has a second trailing clause-level PP *as you
  cast this spell*, currently relies on `LocatedExistentialBe` to unlock the
  `ExistentialPredicate` adjunct schema).

## Change

- Eliminate the duplicate analysis between `ExistentialBe` (with nominal
  postmodification on `displaced_subject`) and `LocatedExistentialBe` so an
  existential clause with a locative/partitive PP has one Reading per genuine
  attachment, while keeping *Wail of the Forgotten* (*If there are eight or
  more permanent cards in your graveyard as you cast this spell, …*) read.

## Done when

- *As long as there are seven or more cards in your graveyard, this creature
  gets +1/+1.* has one Reading.
- The 1,049+ duplicate existential Readings are gone.
- No face is lost.

Standard constraints apply.

Census delta (measured on ulnxlpmm): faces read 28,709 → 28,709; total Readings 344,330 → 342,176; lost faces: none.
Judgment set: 2,920 enforced at `COMPLETED_STRATUM` 6, all hold; 192 pending.
