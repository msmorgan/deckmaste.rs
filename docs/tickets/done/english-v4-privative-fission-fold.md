---
needs: []
---
# Fold category fission into privative features

Implements owner-rulings-12b finding C, which R5.1 (english-lexical-analysis §3)
now allows.

## Change

- Replace SubjectInitialClause with a privative one-valued feature on Clause.
  Its five duplicated Clause productions (RelativeSubjectPredicate,
  RelativeCliticSubjectPredicate, RelativeFloatedEach, RelativeFloatedAll,
  RelativeAdjunctGap) collapse into the plain Clause ones.
- Replace NegativeAttribute, NegativeNounPremodifier, NegativeParticiple and
  NegativeAdjectivePhrase with a privative `Polarity: ["Negative"]` value on
  their base categories, and drop the six duplicated premodifier productions.
- No unary wrapper (R3, C-c). Each new value has two or more holders.
- The two formation rules are unchanged and become feature Requires: bare
  relatives are subject-initial (CGEL Ch. 12 §3.5.5 p. 1055), and the chain
  takes only *non-* premodifiers (OSG §6 "Determiners and modifier order").
  Restate their ADR §4.4 kept-gate entries as Requires.
- Rewrite Judgments that name the deleted categories to assert the same
  analysis in the new form.

## Acceptance

- −5 categories and −11 duplicated productions.
- Census identical, or differing only by trees that were duplicates; any
  changed unit is explained. Nothing new admitted; no face or endorsed
  analysis lost.

## Resolution

Owner ruling round 13, item 3: the Negative* half lands alone (−4 Categories,
−6 duplicated Productions); `Polarity` is two-valued, `["Affirmative",
"Negative"]`, since the lexicon already gives auxiliaries `Affirmative`.
SubjectInitialClause and its 5 restated Productions stay: category fission is
the required form here; a head-propagated feature cannot express the listing
without a new engine field.

Standard constraints apply.

1. **Census delta** (`xpqpupwk` → `lkwknrrq`, full corpus; the Negative* half only): faces read 28,801 → 28,801; total Readings 245,781 → 245,781, Surplus 106,817 → 106,817, no unit changed. Lost faces: none.
2. **Judgment set** (`lkwknrrq`): 3,052 enforced, 206 pending; all hold.
