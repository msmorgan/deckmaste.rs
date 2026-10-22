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

   The Production Cost audit found the same defect in more places. Owner ruling
   (defect #7): it is folded in here, not ticketed separately.
   - The Displaced Subject can also end in the `among` phrase itself:
     *[four or more card types among cards (in your graveyard)]*, with no
     Extension or with *in your graveyard* as the Extension.
   - That reading contradicts the settled analysis in
     `docs/decisions/english-lexical-analysis.md` §2. A locative Preposition
     Phrase right after the Displaced Subject is its Extension, never also its
     postmodifier (CGEL, Ch. 16 §6.2.1, p. 1394).
   - The same `unextended: ["displaced_subject"]` closure covers it.
   - Evidence, measured on `loxuknun`:
     - Delirium: *Whenever this creature enters or attacks, if there are four
       or more card types among cards in your graveyard, …*. 15 of its 60
       Readings make *among cards* a Modifier of *card types*.
     - *Then if there are six or more card types among cards in your
       graveyard, double the number of +1/+1 counters on those creatures.*
       2 of its 6 Readings do the same.
     - Judgment set: 3 entries, 6 of 14 Readings.
     - The intended Reading has *among cards in your graveyard* as the
       Extension and *in your graveyard* as a Modifier of *cards*.
2. **Production/Category name collision (`english-v4-zero-plural-determiner-number`):**
   Rename the Production `TargetingMarker` (`english.ron:1066`) to
   `BareTargetingMarker` so it does not collide with the Category name
   `TargetingMarker` (`english.ron:166`).
3. **Avoid duplicate `PeriodNP` preposition frames:** moved to
   `english-v4-period-np-frame-duplication`. Its proposed fix needs a new
   `NounKind` value, which the owner's rulings forbid, so it waits on an owner
   ruling there.

## Done when

- Omnivorous Flytrap has no split `[card types among cards] [in your graveyard]`
  Reading, covered by a `+` Judgment on real text.
- No existential Displaced Subject ends in an `among` or `in` phrase that
  `LocatedExistentialBe` would take as its Extension. The Delirium text above
  is pinned by a `+` Judgment.
- Production `TargetingMarker` is renamed to `BareTargetingMarker`.
- No face is lost, and no unit gains a Reading.

## Constraint on item 1 (cost-audit defect #7): no new features in `english.ron`

Owner's ruling (round 2): defects 1–8 are fixed by tightening or deleting the
rule that admits them, with zero new features in `english.ron`. Item 1 uses
the existing `unextended` mechanism; if it cannot, STOP and return to the
owner.

Standard constraints apply.

1. **Census delta** (measured on `srporzyt`, items 1 and 2):
   faces read 28,799 → 28,799; total Readings 214,080 → 213,878 (Surplus
   88,371 → 88,248, over 75 units, none rising); lost faces: none.
2. **Judgment set**: 3,032 enforced at `COMPLETED_STRATUM` 6, all hold; 208
   pending.
