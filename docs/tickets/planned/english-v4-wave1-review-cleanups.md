---
needs: [english-v4-existential-locative-ambiguity, english-v4-temporal-np-as-direct-object, english-v4-zero-plural-determiner-number]
---
# Follow-up cleanups from the pre-Stratum-7 ambiguity wave

Three small follow-ups from the review of the pre-Stratum-7 ambiguity tickets:

1. **Omnivorous Flytrap existential split (`english-v4-existential-locative-ambiguity`):**
   *As long as there are four or more card types among cards in your graveyard*
   still has two Readings for the existential clause because `in your graveyard`
   attaches either inside the displaced subject (*card types among [cards in
   your graveyard]*) or as `LocatedExistentialBe`'s locative `among`/`in`
   complement (*[card types among cards] [in your graveyard]*).
   `LocatedExistentialBe` (`english.ron:547`) already declares a selected
   `among`/`in` complement (`prepositions: Some(["among", "in"])`), so adding
   `unextended: ["displaced_subject"]` on `LocatedExistentialBe` (and allowing
   `Binder::unextended` in `deckmaste_syntax/src/compile.rs` to close a child
   against a following selected-preposition sibling in the same production)
   retires the split Reading without affecting genuine `ExistentialBe` +
   locative `LocatedExistentialBe` (*there are seven or more cards in your
   graveyard*).
2. **Production/Category name collision (`english-v4-zero-plural-determiner-number`):**
   Rename the Production `TargetingMarker` (`english.ron:1066`) to
   `BareTargetingMarker` so it does not collide with the Category name
   `TargetingMarker` (`english.ron:166`).
3. **Avoid duplicate `PeriodNP` preposition frames (`english-v4-temporal-np-as-direct-object`):**
   Adding `SlotFill::PeriodNP` / `PeriodNounPhrase` required duplicating
   `(complement: PeriodNP, ...)` frames alongside `NP` on `of`, `after`,
   `before` and `during` in `prepositions.ron`. Because prepositions take
   `NounPhrase` while verb direct objects take `ComplementNounPhrase`
   (`NonNumericComplement`, `english.ron:397`), representing deictic and
   distributive period phrases as `NounPhrase` with `NounKind: Period`
   (excluded on `NonNumericComplement`, alongside `SlotFill::PeriodNP` or a
   `Period` object frame on `skip`) lets prepositions take them via their
   existing `NP` frame without duplicate preposition frames.

## Done when

- Omnivorous Flytrap has no split `[card types among cards] [in your graveyard]`
  Reading, covered by a `+` Judgment on real text.
- Production `TargetingMarker` is renamed to `BareTargetingMarker`.
- `of`, `after`, `before` and `during` do not duplicate `NP` and `PeriodNP`
  frames in `prepositions.ron`.
- No face is lost, and no unit gains a Reading.

Standard constraints apply.
