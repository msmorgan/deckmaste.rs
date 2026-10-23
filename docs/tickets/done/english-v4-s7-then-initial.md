---
needs: []
---
# Stratum 7: Sentence-initial *Then*

Part of `english-v4-stratum-7-discourse-and-long-tail`. Implement in `crates/deckmaste_english_v4` per [`docs/decisions/english-v4.md`](../../decisions/english-v4.md) (§6, Stratum 7).

## Scope

Admit sentence-initial *Then* as a connective Adjunct before an unfronted Clause, imperative or declarative (*Then draw a card.*, *Then that player shuffles.*, *Otherwise, ...* in the same position). Today *Then* reads only before a preposed dependent (*Then if ...*). The analysis is the one the pending judgments already state: `AdverbPhrase "Then"` as an Adjunct of the Clause's head (`then_sequencing.txt:5`). `english-v4-s7-then-coordination` builds on this slice.

## Grounding

- CGEL, Ch. 8 §19, p. 778 [9ii], which lists *then* among the ordering connectives.
- `docs/oracle-style-guide.md` checklist item 10 and §10, "Sequence: and, then, if you do, when you do".

## Witness cards

- Wasp, Shrinking Savior (*Then draw a card for each creature with power less than 0 ...*)
- Chain of Plasma
- Terisiare's Devastation
- Thirst for Identity
- Special Move (`named_modes.txt:13`)
- Lily Bowen, Raging Grandma (*Otherwise*)

## Estimated faces

About 430 faces (an estimate from the 2026-10-23 census, not a measured figure).

## Engine

No. Grammar only.

## Pending judgments this slice makes hold

`base_phrases.txt:15`; `then_sequencing.txt:5`; `superlative_quantifiers.txt:11`; `waterbend_cost_verb.txt:6`; `trailing_adjunct_height.txt:9`; `card_noun_premodifier.txt:3`; `all_but_exception.txt:14`; `starting_with_adjunct.txt:4`; `named_modes.txt:13`; `where_variable_clause.txt:16`.

Confirm each entry against the current judgment files when you start; entries already holding need nothing.

## Constraints

Governance is `docs/decisions/english-v4.md` §4.4, Principles: admit English, suppress nothing grammatical, add the general construction the text needs (any features, values, Categories, Productions or engine capabilities, each justified by a CGEL, OSG or CR citation), and keep the engine free of grammar names. Standard constraints apply.

## Landing note

Per `docs/tickets/README.md`, end the ticket with these two lines, stamped with the change id they were measured on:

1. **Census delta**: faces read and total Readings before and after; lost faces by name, or "none".
2. **Judgment set**: the result of running it.

A front Adverb Phrase without a comma now precedes a Clause (*Then draw a
card*, *Then that player shuffles*). Front dependents sharing one comma stay
siblings in one Production (*Once each turn, ...*, *Then if ...*): the new
Clause feature `FrontPosition`, valued by every comma-set fronting Production
and taken by a Clause coordination from its first conjunct, keeps the
comma-less schema off such a Clause, so *Once [each turn, ...]* is no second
tree. Reading Normal Form now re-derives a moved host's head-inherited
features (fronted dependent outermost). Interrogative *how* is no longer a
front adverb. Remove Enchantments (138 × 68), Muse Vortex (1 × 37 × 48) and
Legion's End (116 × 9) are products of independent per-sentence counts.

1. **Census delta** (`wznlvsyl` → `oktrlqmo`, full corpus): faces read 29,932 → 30,365; total Readings 258,466 → 277,002; Surplus 113,785 → 116,303; 443 units changed (439 newly read, 4 grown). Lost faces: none.
2. **Judgment set** (`oktrlqmo`): 3,081 enforced, 242 pending; all hold.
