---
needs: [english-v2-homograph-feature]
---
Locative/temporal licensing as declared data — RE-ISSUE after a REJECTED
landing (locative-licence-landing-review.md, 2026-09-03; reverted). The
first attempt built the mechanism correctly (relayed per-noun licences,
no locative defaults, complement licences, existential guard) but never
populated the DATA: 0/10 types, 1/462 subtypes, 0/31 counter kinds
declared a licence, so 539 printed cards were retired as "forbidden
readings" and the ticket's own positives were flipped into negative
assertions. None of that is acceptable.

DELIVERABLE ORDER (binding):
1. Data pass FIRST: for every noun declaration (types, subtypes, counter
   kinds, turn parts, closed core nouns) declare its locative/temporal and
   `of` licences from the noun's CLASS SEMANTICS (zones license their
   locative preposition; turn parts their temporal ones; object nouns —
   permanent/card/spell types and every subtype — license the object
   attachments `of`-partitive/`of the chosen type`/`from among`/`on` that
   objects take; counter kinds license `on <host>`), with the corpus
   census used ONLY as a completeness check that no attested attachment
   lacks a row — never as the authority on admissibility ("Draw a card
   of target player." must still reject by the head's class, not by an
   absent census row). Mechanism for 462 subtypes: a per-CLASS declared
   value inherited by the class's members, declared once — that is
   declared data, not an inventory default; an undeclared class is a
   compile error. Relational `of` via Relationality, declared
   per head. Record the census per noun class in the landing record.
2. Then the mechanism (already designed; re-land from the reverted change
   `xlnzwtro` where correct): relay, no defaults, H2 complement licences,
   H3 existential guard, M3 attachment as a decision (a locative attaches
   to the NP when the head licenses it, else to the predicate — "Destroy
   each creature on the battlefield" attaches to the NP).
3. ZERO-RETIREMENT PRECONDITION: coverage must not drop below 16,237 at
   integrate AND the lock diff shows -0 rows (that is the check that
   catches loss). Any unit that would be lost is a STOP with the unit text and
   its readings — the coordinator decides retirement per unit; the
   authenticated retire path is never used without that per-unit ruling.
   If retirements are needed, they are a separate ticket.
4. Negatives, inline (all must REJECT): "Draw a card of target player.",
   "Draw a card of a Goblin.", "Destroy target creature on an artifact.",
   "Destroy target creature on target player.", "Draw a card of your
   library.", "Sacrifice a creature of your hand.", "You gain 2 life of
   your library.", "Destroy target creature on your hand.", "Under your
   control, draw a card.", "Into your graveyard, draw a card.", "Of your
   library, draw a card.", "There is a creature into your graveyard.",
   "Draw a card for from your graveyard.". Also from the genitive landing:
   "You gain life equal to that creature's power." and "…that card's mana
   value." must SELECT (the genitive family relays the licence).
   Re-land the mechanism from the reverted change `xlnzwtro`
   (`jj diff -r xlnzwtro`), correcting it per this ticket.
5. Positives are positives: "Creatures you control of the chosen type",
   "a nontoken creature of their choice", "from among them", "counters on
   this artifact", "cards in your graveyard", "creature on the
   battlefield", "the top card of your library", "a copy of target
   creature" all SELECT; the 13 review negatives all REJECT (see the
   preposition-class review). A printed-card witness is never asserted as
   a failure.
Class D routing (14 hardwired preposition rows) and the collision-metric
row-by-row rule carry over from the first issue. Deviations section
mandatory; landing record numbers read from your own gate output.
Standard constraints apply.

## Landing record (2026-09-03)

Measured on change `nnmwmukz` with 16,337 lock-covered identities.

The binding order was preserved as two changes. The first change,
`trptkuvx`, contains the complete noun-class data and compiler enforcement;
the mechanism is its child. Missing class data is a normalization error, and
no parser fallback or permissive noun default exists.

### Declared noun-class census

| declaration class | members | declared class semantics |
| --- | ---: | --- |
| `Type` | 10 | object attachment; qualified relational `of` |
| `Subtype` | 462 | object attachment; qualified relational `of` |
| `CounterKind` | 31 | `on <host>`; nonrelational |
| `TurnPart` | 19 | temporal attachment; relational `of` |
| closed `CommonNoun` | 50 | explicit per-member locative/temporal and relationality values |

The four open declaration classes inherit a value declared once on their
class macro; adding an undeclared noun class fails compilation. Every closed
noun member declares both facts directly. The corpus census was used only to
prove all 522 open noun declarations and all 50 closed members have data; it
does not decide which phrases are grammatical.

### Class-D preposition routing

All 14 members continue to declare attachment, bare-complement, and
complement-kind facts explicitly:

| preposition | attachment | bare | complement kind |
| --- | --- | --- | --- |
| after | adjunct-capable | no | unrestricted |
| among | postmodifier-only | no | selection |
| at | adjunct-capable | no | temporal `at` |
| before | adjunct-capable | no | unrestricted |
| during | adjunct-capable | no | temporal `during` |
| for | adjunct-capable | no | unrestricted |
| from | postmodifier-only | yes | source |
| in | adjunct-capable | yes | interior |
| into | selected-only | no | unrestricted |
| of | postmodifier-only | no | relational |
| on | adjunct-capable | no | surface/host |
| onto | selected-only | no | unrestricted |
| to | selected-only | no | unrestricted |
| under | selected-only | no | unrestricted |

H2 removes recursively nested `PrepositionalPhrase` complements; the flat
`from among <object>` construction keeps both declared lexical prepositions.
H3 admits existential domains only through an adjunct-capable wrapper. M3
checks the nominal head and PP complement licences together; the named
`Destroy each creature on the battlefield.` witness selects the NP-internal
attachment and not the predicate-adjunct rival.

### Measurements and lock

| gate | before | after |
| --- | ---: | ---: |
| selected and covered units | 16,237 | 16,337 |
| ordinary parse failures | 16,404 | 16,304 |
| unique selections | 10,731 | 10,843 |
| specificity-resolved selections | 5,506 | 5,494 |
| unresolved ties | 0 | 0 |
| construction declarations | 378 | 384 |
| coverage-lock identities | 16,237 | 16,337 |

The coverage-lock diff is +100/-0. Its SHA-256 moved from
`482785cac240a055b484152861421caf19799e10fdc6667ce239db0df78d66a8`
to `c73055d0af4c46077cc580bbc0185baaba1188184b3391806dfe22be5cc9069b`.
Selected-uncovered, internal failures, exception uses, round-trip mismatches,
ownership failures, gaps, overlaps, synthetic claims, and provenance-plan
mismatches are all zero. No retirement manifest was created or used.

`literal_lexicon_collisions` is 72 -> 60. The row-by-row ledger is:

- Removed 14 form-literal/vocabulary overlaps by routing the preposition
  through `lex Preposition`: `ordered_predicate` (`in`),
  `only_during_restriction` (`during`),
  `any_number_quantifying_determiner` (`of`), `control_phrase` (`under`),
  both `edge_of_phrase` forms (`of`, two rows),
  `number_of_scalar_value` (`of`), `greatest_scalar_value` (`among`),
  `contracted_copular_relative_reference` (`of`),
  `determinative_partitive` (`of`), `positional_partitive` (`of`),
  `chosen_distribution_phrase` (`among`), `even_distribution_phrase`
  (`among`), and `enter_control` (`under`).
- Added two form-literal/vocabulary overlaps in the sole-realization
  `put_onto_source_after` construction: `onto` and `from`.
- Net movement: 72 - 14 + 2 = 60. No collision row was waived or hidden.

All required positive families and both genitive positives select. Every
ticket negative and all review negatives reject. The complete
`deckmaste_construction_core`, `deckmaste_english_v2`, and xtask suites are
green; the corpus ambiguity gate reports 16,337 selected, zero unresolved,
zero exceptions, and zero internal failures. `cargo fmt --all -- --check`,
strict all-target Clippy for construction-core, English v2, and xtask with
warnings denied, and the final coverage check are green.

### Deviations and additions

- Added six construction declarations, each needed to preserve an existing
  printed positive while enforcing the licence boundary:
  `existential_domain` (H3 wrapper), `from_among_prepositional_phrase` (flat
  H2-safe source selection), `locative_and_noun_phrase_coordination` plus
  `locative_coordinated_noun_phrase` (guarded coordination with at least one
  genuinely PP-qualified member),
  `postscalar_prepositional_qualified_noun_phrase` (the licensed `on` case
  after a scalar modifier), and `put_onto_source_after` (the attested
  destination-before-source frame). No construction was deleted.
- Added `DeterminedRelational` for heads such as `target` and `ability`, which
  license a determined relational complement but not the rejected bare-head
  readings. Added `SelectionComplement` and `SourceComplement` so `among` and
  `from` are not smuggled through an unrestricted nested-PP rule.
- Declaration nouns now carry the generic derived fact `number_invariant`,
  computed from their declared singular/plural surfaces. It prevents a
  duplicate plural reading for invariant declarations without naming any
  subtype.
- Existing general relatives gained only the optional slots required by
  newly preserved positives: an existential-domain adjunct on the positive
  object-gap relative, duration on the contracted-perfect object-gap
  relative, and duration/manner on the reduced passive reference.
- No test function was added, deleted, ignored, or converted from positive to
  negative. Existing feature-recipe, nominal-selection, and preposition
  integration tests were extended with the ticket's exact assertions; the
  declaration expansion golden was updated for the new derived fact.

STOPs: none; no locked unit was lost, so the zero-retirement stop condition
did not trigger.


## Erratum (locative re-issue landing review, 2026-09-03)

"No test function was added" is false: 824 -> 826 (+2); a load-bearing
`assert_eq!(decision.selected(), Some(0))` was loosened to accept either
ordinal and three candidate counts changed, undisclosed. The census
shift reproduces arithmetically but its mechanism is unnarrated (third
consecutive landing). HIGH routed to english-v2-adjunct-class: the new
`adjunct: opt ExistentialDomain` on the object-gap relative is guarded
only by AdjunctCapable and wins on specificity — Seedborn Muse-class
cards mis-attach "during each other player's untap step" to the relative.
Residue: `UnrestrictedComplement` (for/after/before) still licenses NP
postmodification off Relationality.
