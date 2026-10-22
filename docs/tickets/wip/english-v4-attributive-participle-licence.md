---
needs: []
---
# [design] Attributive participles only where Oracle uses them

Owner ruling (cost-audit smaller defect S9): mint this as a defect. A verbal
participle read as an attributive Premodifier of a bare noun yields spurious
Readings.

## The defect

The `Premodification(head: Verb, …, forms: [GerundParticiple, PastParticiple])`
Schema makes every verb's participle an attributive Premodifier of a bare
Nominal. In Oracle text this produces:

- *You may play [lands and cast spells] from that player's hand this turn*:
  *cast spells* is a Noun Phrase ("spells that were cast"), coordinated with
  *lands* as the Object of *play*. The same happens on two other sampled
  Segments:
  - *Until end of turn, you may play lands and cast spells from your
    graveyard*;
  - *You may play lands and cast spells from among cards you own in exile with
    croak counters on them*.
- *instead of [declaring blockers]*: *declaring blockers* is a Noun Phrase
  ("blockers that are declaring") instead of a Gerund-participial Clause.

## Authority

- CGEL, Ch. 5 §14.2, p. 444 admits attributive VPs headed by participles in
  English generally (*the defeated army*, *the gleaming showroom*).
  - This reading is therefore not ungrammatical in open-domain English.
  - It is wrong in the Oracle register.
- `docs/decisions/english-lexical-analysis.md` §2 ("Morphological and valence
  completeness vs controlled-register syntax"): syntactic schemas model the
  controlled register, not open-domain English.
- Corpus facts:
  - No Vintage-playable Oracle text uses attributive *cast* or *declaring*
    after a determinative: 0 matches of *the/a/each/that/those/any/no* +
    *cast*/*declaring* + noun in the Oracle Cards snapshot.
  - Oracle does use many participles attributively. In the evaluation data the
    following appear in every Reading of their Segment, so they must keep
    reading:
    - *chosen* (13 Segments), *attacking* (9), *exiled* (7);
    - *triggered* (4), *activated* (4);
    - *blocking* (3);
    - *tapped*, *starting*, *revealed*, *sacrificed*, *locked* (2 each);
    - *enchanted*, *warped* (1 each).

## Design decision to make at claim time

The fix needs a licence on which participles premodify.

- Candidate:
  - declare the participles with an attested attributive use with their verb
    in `verbs.ron`, or in the catalogs for keyword-action and status
    participles;
  - license `Premodification` only for those.
  - CGEL treats attributive and predicative distribution as a lexical property
    of adjectives (Ch. 6 §4); this would be the participle analogue.
- ADR conflict: `docs/decisions/english-v4.md` §4.2 bans policing flags that
  suppress *benign* ambiguity. Before implementing, confirm with the owner that
  this licence is lexical data and not such a flag. If it is not acceptable,
  STOP.
- Related: *tapped* and *untapped* have both an adjective-lexeme analysis and a
  participle Premodification analysis, which duplicate each other (2 Readings,
  for example *a tapped 1/1 … token*). Resolve that in the same change, so
  that one source supplies each attributive word.

## Acceptance set

All figures are from the cost-audit evaluation data, measured on `loxuknun`.

- **Corpus sample:** the 4 Segments above lose 382 Readings, out of a combined
  Surplus of 794.
- **Judgment set:** no entry depends on the removed analysis.
- **Corpus reach:**
  - 27 ambiguous units contain *and/or cast spells* or *of declaring*, with
    4,878 Readings between them.
  - Expected reduction: about half of those units' Surplus.
- **Must keep:** every face that today reads only through an attributive
  participle from the list above.

## Done when

- *play lands and cast spells* has no Noun Phrase *cast spells*.
- *instead of declaring blockers* has only the Gerund-participial Clause.
- Each has a `+` Judgment.
- No face is lost, and no unit gains a Reading.

## Constraint: no new features in `english.ron`

Owner's ruling (round 2): this defect comes from a rule that is too broad, and
it is fixed by tightening or deleting that rule. The fix adds **zero new
features to `english.ron`**. It uses existing features, lexicon or catalog
frames, or an engine mechanism, and names no word in the grammar. If it
cannot be done that way, STOP and return to the owner.

Caveat 5 (owner-approved): this ticket needs a structural fix or goes back to
the owner. The candidate licence is declared in `verbs.ron` and the catalogs.
If licensing `Premodification` from it needs a new feature in `english.ron`,
STOP.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
