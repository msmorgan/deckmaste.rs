---
needs: []
---
# License selected prepositions through declared noun valence

Implement the recorded “preposition classes and noun complement licensing”
amendment (2026-09-02) in [the English rewrite decision](../../decisions/english-v2-rewrite.md#amendment-preposition-classes-and-noun-complement-licensing-2026-09-02):
“Only adjunct-capable PPs enter the free predicate-adjunct / NP-postmodifier
attachment rule.” A “selected-only preposition therefore appears only under a
licensing verb frame or noun valence.” Use declared preposition properties and
noun valence; never use a guard naming a preposition or card.

Pinned witness: Animal Magnetism's exact constituent,
`put that card onto the battlefield and the rest into your graveyard`.
The frame-coordination refinement admits the intended plain and past-participial
segment Readings alongside six pre-existing invalid NP/PP groupings. Assert its
complete independently justified expected set with zero invalid members and
both roundtrip laws. Compare the full supported corpus before/after by identity;
name every removed Reading as a retired wrong analysis with its grammatical
witness, and preserve every legitimate Reading. Preserve adjunct-capable noun
postmodification: `card from your graveyard` (Raise Dead) and
`creature with flying` (Hurricane) must keep parsing with their complete
grammatical structures. Both currently have one nominal Reading; probes are in
`/tmp/frame-coordination-adjunct-{card,creature}-nominal.json`.

Widened scope (2026-10-06): implement the
[Preposition Phrase function amendment](../../decisions/english-lexical-analysis.md#amendment-preposition-phrase-functions-are-licensed-per-preposition-2026-10-06).
Every preposition declares its **Preposition Function Licence** (permitted
functions: Adjunct, NP Modifier, noun Complement, verb-selected Complement,
predicative Complement), read by the free-adjunct and postmodifier rules and by
selected frames. The 2026-09-02 "adjunct-capable / selected-only" split above
is the special case. *during* is Adjunct-only; *of* is NP-internal only.

Additional pinned witnesses:

- **Seedborn Muse**, "Untap all permanents you control during each other
  player's untap step.": the expected attachment set is exactly two (to
  *Untap*, and to *control* inside the relative clause). The nominal
  attachment to *permanents you control* is a negative witness.
- *of* shapes that must keep parsing: **Kindred Judgment** (predicative
  "aren't of the chosen type"), **Stenn, Paranoid Partisan** (discontinuous
  "Spells you cast of the chosen type"), **Incriminate** ("one of them of
  their choice"), and **Lantern of Undersight** (fixed "instead of").

Additional acceptance:

- Re-spell the Seedborn Muse test in
  `crates/deckmaste_english_v3/tests/reading_support.rs` to assert exactly the
  two attachments plus a negative for the nominal attachment.
- Rename the feature `AdverbialUse` to follow the glossary term.
- Full-corpus before/after identity comparison naming every removed Reading as
  a retired wrong analysis with its witness, and every lost face (expected:
  none).
- Route the "At the beginning of combat on your turn" question (319 cards) to
  the user before implementation touches *on*.

Also discriminate the unreadable destination constituents of Cavalier of
Thorns and Genesis Ultimatum: `from among them` and their source-bearing Object
NPs. These are owned causes to diagnose, not assumed noun-licensing failures;
route any distinct missing PP-complement or nominal composition to a measured
owner. Evidence: `/tmp/frame-coordination-components-*.json`.

Orchestrator decision (2026-10-06), **user review pending**: keep this repair
separate from frame coordination. Its six invalid Animal Magnetism groupings
pre-date that ticket (six baseline Readings, none correct); the refinement adds
intended segment structures and introduces no new wrong Reading in that
constituent. Implementing the amendment restricts NP postmodification across
the corpus and requires its own reviewed landing. Standard constraints apply.
