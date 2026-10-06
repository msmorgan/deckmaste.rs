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
  a retired wrong analysis with its witness, and every lost face (the only
  permitted losses are Comeuppance and Vazi, Keen Negotiator, as the licence
  table below allows; any other lost face is a STOP).
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

## Licence table (orchestrator draft, 2026-10-06; user approved running it)

Functions: **A** = clause/VP Adjunct, **M** = Noun Phrase postmodifier, **VC** =
verb-selected Complement (lexical frame), **NC** = noun-selected Complement,
**AdjC** = adjective complement, **P** = predicative Complement (after
*be*/*become*), **CP** = part of a declared compound/fixed preposition.

Source: a heuristic corpus survey over 32,875 supported faces on 2026-10-06.
Counts are token-context classifications, not parses.

| Preposition | Licence | Evidence | Consequential? |
|---|---|---|---|
| of | NC, M, P | 17,395 tokens; NC-dominant ("number of", "end of"); P "aren't of the chosen type" (6); never A, never VC | no |
| to | VC, AdjC, NC, CP | 13,309; VC dominant (return/deal/attach … to); AdjC "equal to" (1,674); NC "attached to"; CP "up to", "in addition to"; M excluded (~90 heuristic M-certain hits must be re-examined; expected to be verb-attached) | YES: M excluded |
| from | VC, NC, CP | 5,373; VC source of movement (~3,500: return/put/exile … from); NC "protection from" (353); CP "from among" (472); M excluded. Two attested M uses are expected losses and must be named: Comeuppance "damage from a creature source", Vazi, Keen Negotiator "mana from a Treasure" | YES: M excluded; at most these two faces may lose coverage |
| by | VC (Internalised Complement, Scalar Extent), A (means, "by paying"; "except by") | 675; passive agent dominant (blocked by, dealt by); M excluded: this retires the nominal attachments "damage … this turn by sources" (Avacyn, Guardian Angel), "graveyard by paying 3 life" (Noctis, Prince of Lucis), and Alien Symbiosis's equivalents; these are Reading retirements, not face losses | YES: M excluded |
| with | M, VC | 5,334; M dominant ("Creatures with flying", "token with prowess"); VC "enters with N counters on it" (580), resultative "exiled with a time counter on it"; never A, never P | no |
| on | M, VC, A | 6,308; M "counter on it"; VC "put … on"; A "On your turn," (4) and "combat on your turn" (318) keeps both analyses; no removal | no |
| in | M, P, CP | 2,385; M "card in your hand"; P "is in your graveyard" (80); CP "in addition to" | no |
| at | A, VC, CP | 3,715; A "at the beginning of"; VC "look at" (627); CP "at random" | no |
| for | VC, NC, CP, A | 3,600; VC "search … for"; NC "affinity for"; CP "for each", "for as long as"; A duration | no |
| into | VC | 2,044 | no |
| onto | VC | 1,170 (the Animal Magnetism witness) | no |
| among | VC, M, CP | 1,006; VC "divide … among", M "greatest power among", CP "from among" | no |
| under | VC, A, M | 693; "under its owner's control" (617) keeps all analyses; no removal | no |
| during | A | 909; 0 NP-postmodifier uses; Seedborn Muse nominal attachment retired | YES (already ruled) |
| without | A, M | 463; A "without paying its mana cost" (315); M "Creatures without flying" | no |
| until, if, unless, except, after, while, since, before | A | clausal/temporal adjuncts; "spells cast before that spell" attaches to *cast* | no |
| as | CP, A | "as long as", "as though", "as X as", "as an additional cost" | no |
| than | comparative complement only | 893 | no |
| up | CP ("up to") only | 1,612 | no |
| face down, face up | intransitive; resultative/depictive complement after the verb or its object | 110 / 175 | no |
| plus, minus | arithmetic operators, not PP licence | 205 / 34 | no |
| between, beyond | NC "difference between" / M "each target beyond the first" | 14 / 24 | no |
| against, because, within, through | 0 supported tokens (through: 2, both inside card names); declare no licence; do not delete the lexemes in this ticket, report them | no |

Rules for this table:

- Hard cases where M and VC are both structurally available after a verb's
  object (*with* after create/destroy/search; *on* in "counter on"; *under* in
  "under its owner's control") are NOT resolved by this ticket: both analyses
  remain admitted; Preference owns ranking.
- For the three "M excluded" rows, the full-corpus before/after comparison must
  name every removed Reading as a retired wrong analysis with the sentence, and
  every lost face. If any face other than Comeuppance and Vazi loses coverage,
  or if removals appear on faces not involving *from*/*to*/*by* nominal
  attachment, STOP and report before landing (fix-first rule applies to
  regressions in the implementation, not to a surprise in this table: a
  surprise means the table is wrong and the user decides).
- Implementation reads one declared licence feature per preposition; no guard
  names a word. The existing `AdverbialUse` feature becomes the A licence under
  the new name.
- The *on your turn* question is closed by licensing *on* for A and M both; no
  ruling needed.
