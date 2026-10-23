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

Standard constraints apply.

1. **Census delta**: not yet measured.
2. **Judgment set**: not yet measured.
