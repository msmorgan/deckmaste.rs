---
needs: [english-v2-pp-construction]
---
Fix the preposition-class data and close the reopened overgeneration
(pp-landing-review H1/H2/M2/M3). The general PP mechanism stands; its data
and guards do not.

- H2 first: every `vocab Preposition` member carries an EXPLICIT declared
  class — no member may inherit the vocab default. Assign per the
  2026-09-02 amendment (in, on, under, at, during adjunct-capable; to,
  into, onto, of selected/postmodifier) and MEASURE each assignment
  against the corpus before declaring it; a member whose corpus behaviour
  contradicts the amendment's list is a STOP with the census, not a quiet
  reassignment. Split the conflated enum into two orthogonal features:
  PrepositionAttachment (adjunct-capable | postmodifier-only |
  selected-only) x BareLocativeComplement (yes | no), so `in` can be both
  adjunct-capable and bare-licensing.
- H1: the temporal trigger prefix and every preposed/adjunct site require
  the adjunct-capable attachment feature; "Into your graveyard, draw a
  card.", "Of your library, …", "To target player, …", "Under your
  control, …", "Among them, …" must all reject. §6 probe #24 ("Draw a card
  at the beginning of your end step.") must parse.
- M2: delete the zero-data `NounComplement` feature (a duplicate of the
  live `Relationality` feature). License `of` (and any noun-selected
  preposition) by relaying the head noun's declared complement licence up
  the NP stages to the postmodifier site — the stages already relay
  agreement/number/onset that way — so "Creatures you control of the
  chosen type" and "a nontoken creature of their choice" parse while
  "Draw a card of your library.", "Sacrifice a creature of your hand.",
  "You gain 2 life of your library.", "Destroy target creature on your
  hand." reject. Add those four as permanent negative oracles.
- M3: implement the attachment rule, don't inherit it from specificity —
  "Sacrifice a creature during your upkeep." and "Draw a card for each
  creature you control." each currently yield two readings with nothing
  pinning the survivor; the ruling (nearest licensing constituent) must be
  a derived structural decision, and a genuine two-survivor case is a
  STOP. Record the unique/specificity counts before and after.
Acceptance: coverage >= 15,966 with zero counters, all named oracles in
both directions, landing record with deviations. Standard constraints
apply.

## Landing record (2026-09-02)

The preposition data now enforces two independent facts: where a PP may
attach, and whether its complement head licenses that preposition. The single
general PP construction checks the complement licence while the temporal,
preposed, predicate-adjunct, and nominal-postmodifier sites check the
attachment class. No attachment or complement-licensing construction/helper
names a preposition or noun; the existing selected verb-frame spellings
remain unchanged.

The compiler now supports a vocabulary feature with no default only when
every member explicitly declares it. It emits the same closed feature helpers
for vocabulary members, closed lexemes, and aggregate declaration nouns, so
the head noun's licence relays through the nominal, NP, object, and
prepositional-complement stages just like agreement, number, and onset.

### Preposition census

Counts are case-insensitive whole-word occurrences in the 32,641-unit
supported Oracle snapshot. Every one of the 14 members explicitly declares
all three facts; none inherits a default.

| preposition | occurrences | attachment | bare | general-PP complement rule |
| --- | ---: | --- | --- | --- |
| after | 571 | AdjunctCapable | No | unrestricted temporal sequence |
| among | 1,079 | SelectedOnly | No | selected frame |
| at | 4,707 | AdjunctCapable | No | turn-part head |
| before | 230 | AdjunctCapable | No | unrestricted temporal sequence |
| during | 1,153 | AdjunctCapable | No | turn-part head |
| for | 5,712 | AdjunctCapable | No | unrestricted distribution |
| from | 6,978 | PostmodifierOnly | Yes | unrestricted source |
| in | 3,001 | AdjunctCapable | Yes | in-licensed head |
| into | 2,954 | SelectedOnly | No | selected frame |
| of | 21,380 | PostmodifierOnly | No | of-licensed head |
| on | 8,267 | AdjunctCapable | No | on-licensed head, or library edge |
| onto | 1,286 | SelectedOnly | No | selected frame |
| to | 15,657 | SelectedOnly | No | selected frame |
| under | 954 | SelectedOnly | No | selected frame |

The decisive free-adjunct witnesses are abundant rather than hypothetical:
3,128 supported occurrences contain `at the beginning of`, 1,104 contain a
`during <NP>` sequence, and 309 contain `on <determiner> turn`. In
contrast, `under` has zero preposed-comma witnesses; 885 of its 954
occurrences are the selected `under ... control` family. This is why
`on`, `at`, and `during` remain adjunct-capable while `under` is
`SelectedOnly`.

### Complement-head census

The supported corpus's zone-noun inventory and conservative directly
licensed-pattern counts are:

| zone head | surface occurrences | measured licensed witness | declared licence |
| --- | ---: | ---: | --- |
| battlefield | 3,507 | 353 `on <NP> battlefield` | on |
| command zone | 50 | 15 `in <NP> command zone` | in |
| exile | 4,247 | 44 `in <NP> exile` | in |
| graveyard | 4,983 | 776 `in <NP> graveyard` | in |
| hand | 4,750 | 337 `in <NP> hand` | in |
| library | 4,463 | 10 `in <NP> library`; 645 edge uses | in, or on top/bottom of |
| stack | 36 | 26 `on <NP> stack` | on |

`CommandZone` was added to the closed common-noun inventory so the measured
zone set is complete. The closed temporal heads `beginning` (3,193),
`end` (7,626), `phase` (274), `step` (1,969), and `turn` (11,464),
plus every open `TurnPart` declaration, license `during`, `on`, and
`at`; their relational `of` licence is preserved in the same carried
value.

The conjunction is pinned at both levels. `Under your control, draw a
card.` fails because `under` cannot enter an adjunct site.
`Destroy target creature on your hand.` reaches the adjunct-capable
`on` row but fails because `hand` licenses only `in`. Direct unit
assertions pin both causes, independently of the whole-parser negative
oracles.

All nine required negative oracles reject. The required positive families
parse, including `Draw a card at the beginning of your end step.`,
`Creatures you control get +1/+1 on your turn.`, `Destroy creatures you
control of the chosen type.`, and the two M3 witnesses. The latter select
one reading by the existing structural specificity rule; no attachment
preference or member-name guard was added.

### Measurements and gates

| gate | before | after |
| --- | ---: | ---: |
| selected and covered units | 15,966 | 16,174 |
| unique selections | 10,784 | 10,487 |
| specificity-resolved selections | 5,182 | 5,687 |
| unresolved ties | 0 | 0 |
| construction declarations | 378 | 378 |
| coverage-lock identities | 15,966 | 16,174 |

The coverage increase is 208 identities with no previously covered identity
lost. The final coverage gate reports zero selected-uncovered units, ties,
internal failures, exception uses, round-trip mismatches, ownership failures,
gaps, overlaps, synthetic claims, or provenance-plan mismatches. The two
literal/lexicon collisions are the pre-existing recorded baseline. The
coverage-lock SHA-256 is
`aeba28e9d9309c147bda971e35365589e1e6cee8a726cd8bab5cf02bb323ff54`.

Positive gates: `cargo fmt --all -- --check`; all workspace unit,
integration, compile-fail, and doc tests; strict all-target workspace Clippy;
`cargo xtask english_v2 coverage --check`; and
`cargo xtask english_v2 ambiguity --require-resolved --json`.

### STOP and resolution

The first pass stopped because the ticket repeated the earlier intuitive
classification of `under` as adjunct-capable, while the corpus measured it
only in selected control constructions. The owner resolved that STOP in the
rewrite ADR amendment, “preposition classes are corpus-measured; complements
license locatives”: `under` is `SelectedOnly`; `on`, `at`, and
`during` retain their measured free-adjunct behavior; locative/temporal
admissibility also requires the complement head's declared licence. This
landing implements that resolution.

### Deviations and additions

- The superseded ticket prose lists `under` as adjunct-capable. Per the
  resolving amendment and census, it lands as `SelectedOnly`.
- The ticket asked to delete the zero-data `NounComplement` duplicate and
  reuse live noun data. It is deleted. A single sealed
  `LocativeTemporalLicense` carries the compatible `of`, locative, and
  temporal combinations because `Relationality` alone cannot distinguish
  `hand` from `library` or encode the three turn-part prepositions.
- Corpus preservation required ordinary object heads to retain measured
  relational `of` and counter/marking `on` behavior, and headless
  anaphoric complements to retain their measured `of`/`in`/`on`
  behavior. Explicit zone, turn-part, choice, color, and type rows override
  those general values. The final measured values recovered the remaining 161
  temporarily lost identities and admitted 208 additional corpus identities
  without weakening the named negatives.
- Added the missing `command zone` closed noun. No construction was added,
  removed, or duplicated.
- Assurance counts: added 2 unit tests and 1 integration test; removed 0
  tests; re-spelled 0 tests; ignored 0 tests with new blockers. One existing
  `in a coin` probe moved from the positive table to the negative table
  because its complement head is now correctly unlicensed. One existing AST
  assertion was mechanically updated to use the generated private-field
  accessor. The unrelated four-candidate `any one color` specificity pin
  remains unchanged.
