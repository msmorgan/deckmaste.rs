---
needs: []
---
# Two Judgments that endorse wrong analyses, and a duplicate *and* coordination under *each*

Owner ruling (cost-audit defect #8): fix the two Judgments listed below, and
the duplicate Noun Phrase *and*-coordination match on nouns of dual
countability under *each*.

## 1. `clause_final_predicative_as.txt:9` (Aven Shrine)

*… where X is the number of cards in all graveyards with the same name as that
spell.* The entry asserts `PrepositionPhrase "with the same name as that
spell" as Modifier of graveyard`. The phrase modifies *cards*.

- A name is a characteristic of an object [CR#109.3]. A graveyard is a zone,
  not an object [CR#400.1], so it has no name.
- The neighbouring entry for Counterbore (`clause_final_predicative_as.txt:8`)
  makes the same phrase a Modifier of *card*.

Evidence from the evaluation data:

- The entry has 36 Readings.
- It endorses the 6 Readings that attach the phrase to *graveyard*.
- The 2 Readings that attach the phrase to *card* are not endorsed.
- The rest attach it to *be* (6), to *number* (4), or split the phrase (18).

Rewrite the entry to `… as Modifier of card`. Keep its
`!PrepositionPhrase "as that spell" as Adjunct of gain`.

## 2. `coordination.txt:34`

*+NounPhrase creatures with mana value less than or equal to the number of void
counters on it* asserts `NounPhrase "the number of void counters" as Complement
of than`. That span leaves out *on it*. *on it* belongs to *void counters*: the
counters are on the object that *it* refers to (Sphere of Annihilation's own
text).

Evidence from the evaluation data:

- The entry has 14 Readings.
- It endorses 3 of them. In those, *on it* is a Modifier of *creature* (2) or
  of *mana value* (1). Both analyses are wrong.
- None of the 5 Readings with *on it* as a Modifier of *void counters* is
  endorsed.

Rewrite the entry to `NounPhrase "the number of void counters on it"` in the
same function, and add `PrepositionPhrase "on it" as Modifier of "void
counter"`, using the counter's lemma.

## 3. Duplicate Noun Phrase *and* coordination under *each*

`english.ron` declares three Noun Phrase *and* coordinations. Two of them
matter here:

- Plural *and*: excludes a distributive first conjunct.
- Singular *and*: carries a distributive determinative on both conjuncts.
  This is what *each color and each combination of colors* uses.

The sibling study found that both match the same conjunct pair when the nouns
have dual countability. My evaluation data does not reproduce it:

- *your devotion to each color and each combination of colors* has 3
  Readings, and none of them duplicates another.
  - The endorsed Reading is the singular *and* over *each color and each
    combination of colors*.
  - One Reading coordinates *your devotion to each color* with *each
    combination of colors*. It uses the plural *and*. This is a wrong scope,
    not a duplicate.
  - One Reading strands *of colors* onto *devotion*. That is
    `english-v4-omitted-idiom-complement-split`.
- Across the judgment set and the 172-Segment corpus sample, no two Readings
  share every Constituent and Production while using two different Noun Phrase
  *and* coordinations.

So the first step of this item is a witness:

- Find a text in which the two coordinations both admit the same Constituents.
- Add it as a `+` Judgment that asserts the one intended analysis, with
  `readings N` only if the count is the claim.

Then:

- Make the two coordinations' equations disjoint on the distributive feature
  for dual-countability conjuncts, through features and never by naming words.
- Also retire the plural *and* coordination's admission of the wrong scope
  above, if the Countability or Distribution features allow that without a
  word list.

If no witness exists on current text, record that here, with the texts
checked, and close the item.

## Done when

- Both Judgments assert the intended analysis and hold.
- The duplicate coordination has a witness Judgment that holds with one
  analysis, or this ticket records that none exists.
- No face is lost, and no unit gains a Reading.

## Constraint: no new features in `english.ron`

Owner's ruling (round 2): this defect comes from a rule that is too broad, and
it is fixed by tightening or deleting that rule. The fix adds **zero new
features to `english.ron`**. It uses existing features, lexicon or catalog
frames, or an engine mechanism, and names no word in the grammar. If it
cannot be done that way, STOP and return to the owner.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
