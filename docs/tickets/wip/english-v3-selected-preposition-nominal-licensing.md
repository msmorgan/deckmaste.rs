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
witness, and preserve every legitimate Reading. Preserve noun
postmodification: `creature with flying` (Hurricane) must keep
its complete nominal structure. Raise Dead
("Return target creature card from your graveyard to your hand.") must keep
parsing with the source Complement in Return's lexical frame optional From
slot. Its nominal `[card from your graveyard]` bracketing is a retired wrong
analysis, accounted for with the from/to/by NP Modifier retirements.

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
  their choice"), and **Library of Leng** (fixed "instead of into your graveyard").

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

## Original licence table (superseded by measured narrowing below)

Functions: **A** = clause/VP Adjunct, **M** = Noun Phrase postmodifier, **VC** =
verb-selected Complement (lexical frame), **NC** = noun-selected Complement,
**AdjC** = adjective complement, **P** = predicative Complement (after
*be*/*become*), **CP** = part of a declared compound/fixed preposition.

Historical draft, approved for measurement on 2026-10-06; its exclusions are
superseded by the narrowing resolution and final effective table below.

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
| against | A, VC, M | Unattested in the supported corpus; linguistic licence retained | no |
| because | A (clausal), CP ("because of") | Unattested in the supported corpus; linguistic licence retained | no |
| within | A, M | Unattested in the supported corpus; linguistic licence retained | no |
| through | A, VC, M | Unattested in supported prose (two tokens inside card names); linguistic licence retained | no |

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

## STOP resolutions

- Raise Dead — orchestrator resolution (2026-10-06; user review pending).
  The earlier nominal-preservation requirement is superseded by the licence
  table. The user's own words (2026-10-06), quoted verbatim:
  "from modifies return in that case doesn't it?" The From PP must survive
  as Return's source Complement, selected by its optional lexical frame slot;
  the nominal attachment is a retired wrong analysis. Hurricane's nominal
  witness stands.
- Unattested prepositions — orchestrator resolution (2026-10-06; user review
  pending). The table's no-licence instruction contradicted the recorded
  September 4 ruling "Attestation is provenance, never a filter". Retain
  general-English licences for against (A, VC, M), because (clausal A, with
  because of as CP), within (A, M), and through (A, VC, M). Their absence in
  supported prose is provenance only. Do not delete these lexemes; deletion
  requires a separate user decision. CGEL's function inventory is Ch. 7 §2.1,
  pp. 604–606.

## Witness substitution (resolved 2026-10-06)

The baseline witness census exits 1: `missing requested card names: Lantern of
Undersight`. The snapshot marks that card Vintage `not_legal` and `set_type: funny`;
it is excluded permanently by the repository scope. Replace it with an
authentic supported fixed-preposition witness; do not expand the support scope. Implementation stopped under the orchestrator's instruction
to stop on every red result except the claimant's own test compile error.
Resolution (user, 2026-10-06): replace the witness with Library of Leng,
"instead of into your graveyard", in the ticket and tests. An out-of-scope
witness is a brief error: substitute an attested in-scope card with the same
Construction and disclose the substitution at landing. The full baseline census is
`/tmp/nominal-licensing-before.json`: 32,828 supported faces, 19,206 no Reading,
6,653 unique, 6,969 multiple, 176,171 Readings, zero issues; corpus wall
39,727,731,126 ns, eight workers, host load [2.02783203125, 4.7421875,
4.98193359375], checked-text thread CPU 158,266 ns/B. Measured on change
`luxpqklz`; baseline covered count 13,622. The successful substituted witness census is
`/tmp/nominal-licensing-witnesses-before.json`. No landing is claimed.


## Corpus STOP (resolved by narrowing, 2026-10-06)

The first implementation census is red under the explicit table acceptance
rule. Against the complete baseline, **1,158 faces lose all Readings** and two
previously uncovered faces gain Readings. Coverage falls from 13,622 to 12,466.
This is an intermediate tree before the Return source-frame and chosen/partitive
witness corrections; it is not a final landing measurement. Both reports are
stamped `luxpqklz`, with the baseline covered count 13,622.

Evidence: `/tmp/nominal-licensing-before.json`,
`/tmp/nominal-licensing-stage.json`, and the complete named lost-face list in
`.superpowers/english-v3-selected-preposition-nominal-licensing/first-census-lost-faces.md`.
The baseline Reading identities and their nominal attachment functions are in
`/tmp/nominal-licensing-before-readings.jsonl`.

A decisive outside-scope loss is **Gravetiller Wurm**:
"This creature enters with four +1/+1 counters on it if a creature died this
turn." Its 24 baseline Readings become zero. Baseline Reading
`5173638a12a98201387d952d6bedf72c347b045773fbca5efb9ed1d281a904ef`
has no NP Modifier attachment at all, hence none of the authorised
from/to/by/during NP Modifier retirements. The loss cannot be disclosed as an
approved nominal retirement. The table excludes With from the Adjunct licence
and requires the selected Complement analysis to be licensed by a lexical
frame; existing selected-frame declarations do not provide the replacement
coverage for this sentence. Other lost faces include Disposal Mummy, Game
Trail, Flycatcher Giraffid, and Marketback Walker.

No licence is widened to bypass this red result. Do not integrate or drop the
workspace. Implementation and evidence remain available for review; this STOP
requires a ruling on the table's effects and the scope of replacement selected
frames. The full landing gate and final Reading-retirement accounting are not
claimed complete.

The substituted Library of Leng witness, Hurricane's modifier witness, and
Animal Magnetism's independently constructed exact two-Reading expected set
pass. The draft also re-spells Seedborn's attachment assertions, renames
AdverbialUse to one declared PrepositionFunctionLicence set, and retains the
unattested general-English licences. No tests are removed or ignored.

Latest focused test run (`/tmp/nominal-licensing-witness-tests-3.log`):
`cargo test -p deckmaste_english_v3 --test preposition_function_licences --test reading_support`
compiles successfully; the first test binary reports four passed and one failed.
Raise Dead's selected optional source Complement now passes. The of-shape test
remains red at `aren't of the chosen type`; subsequent of witnesses and the
reading_support binary were not reached. Tests are retained without weakening.
This unresolved witness is additional unfinished implementation work, not a
coverage gain or a licence-table resolution. The full corpus STOP above already
prevents landing.


## Narrowing resolution

Orchestrator resolution (2026-10-06; **user review pending**): an exclusion
from a Preposition Function Licence takes effect only where its replacement
analysis already exists. The draft table assumed undeclared or unconsumed
selected frames. Attribute each exclusion independently against the complete
baseline; retain its old function when the intended exclusion loses an
unpermitted face or removes a legitimate Reading. Do not add verb frames to
repair that coverage in this ticket. Deferred exclusions return to this ticket
after their prerequisite frames become consumable, with the dependencies
recorded in english-v3-systemic-residuals. The original broad table and open
STOP instructions above are historical evidence superseded by this resolution.

The Return source-frame correction made before this resolution rewrites the
existing optional source/controller frame; its frame count remains two. It
adds no verb frame. No further frame declaration changes are made by the
narrowing. The predicative-of repair consumes BeNegative's existing
PredicativeComplement frame and extends the existing verbal Premodifier
construction to a past-participial transitive head with declared Premodifier
availability; Choose's *chosen* form was
already declared. Seedborn's negative tests the direct PostmodifiedNominal
structure rather than forbidding a Nominal span containing a relative-clause
Adjunct. No test is deleted or ignored.


Spatial predicative availability is preserved: Among, At, Between, Beyond, On,
Under, and Within already entered the copular LocativeComplement construction
through their declared LocativeUse feature. Their existing P availability is
now explicit in PrepositionFunctionLicence, and LocativeComplement checks that
licence as well as its spatial classification. These are retained analyses,
not new functions or exclusions. The broad draft table omitted them; that
omission never removed a Reading in the first implementation. In, Outside,
Over, FaceDown, and FaceUp already declared P. No spatial P analysis is retired.


## Independent exclusion attribution

Measured against baseline `luxpqklz`, covered 13,622, using every baseline
Reading identity in `/tmp/nominal-licensing-before-readings.jsonl`. The harness
keeps the baseline Constructions and frames, adds permissive declared-function
checks, reproduces all 176,171 identities, then removes exactly one function
from exactly one Preposition's feature. It checks declaration admission and
realization under that counterfactual. A Reading with no construction using
that function is unaffected. Admission indexes retain all lexical identities
occurring in baseline input; no source frame or Construction is changed by a
toggle. The 137 toggles include every new removal, plus zero-effect exclusions;
of-A is also listed explicitly because it was already forbidden in baseline.
This is attribution, not a linguistic licence inferred from attestation.

Full named counterfactual evidence is in
`.superpowers/english-v3-selected-preposition-nominal-licensing/nominal-licensing-attribution-table.json`
and `nominal-licensing-attribution.jsonl`; deferred lost-face sentences and their
lexical verbs are in `deferred-exclusion-witnesses.md` in the same directory.
Simultaneous application of the original exclusions loses 1158 baseline
faces; single-exclusion face-loss counts overlap and do not sum to that total.
After the seven independently required deferrals, the union of retained
exclusions loses 0 baseline faces. The final corpus independently checks the
combined grammar and witness additions.

A, M, VC, NC and AdjC have the meanings in the licence table above. Examples
name affected cards; fewer than three means the exclusion affected fewer than
three distinct cards. A dash means no baseline Reading is affected. Zeroes do
not withhold a general-English function or delete an unattested Lexeme.

| Exclusion | Faces lost alone | Readings lost alone | Three example cards | Disposition |
|---|---:|---:|---|---|
| after-AdjC | 0 | 0 | — | no baseline effect |
| after-M | 0 | 0 | — | no baseline effect |
| after-NC | 0 | 0 | — | no baseline effect |
| after-VC | 0 | 0 | — | no baseline effect |
| against-AdjC | 0 | 0 | — | no baseline effect |
| against-NC | 0 | 0 | — | no baseline effect |
| among-AdjC | 0 | 0 | — | no baseline effect |
| among-A | 0 | 1,284 | Strength of Unity; Power Armor; Planar Despair | retained exclusion |
| among-NC | 0 | 0 | — | no baseline effect |
| as-AdjC | 0 | 0 | — | no baseline effect |
| as-M | 0 | 263 | Nef-Crop Entangler; Clockwork Droid; Vizier of the True | retained exclusion |
| as-NC | 0 | 0 | — | no baseline effect |
| as-VC | 0 | 0 | — | no baseline effect |
| at-AdjC | 0 | 0 | — | no baseline effect |
| at-M | 0 | 8,692 | Vizier of Deferment; Kykar, Zephyr Awakener; Eerie Interlude | retained exclusion |
| at-NC | 0 | 0 | — | no baseline effect |
| because-AdjC | 0 | 0 | — | no baseline effect |
| because-M | 0 | 0 | — | no baseline effect |
| because-NC | 0 | 0 | — | no baseline effect |
| because-VC | 0 | 0 | — | no baseline effect |
| before-AdjC | 0 | 0 | — | no baseline effect |
| before-M | 0 | 0 | — | no baseline effect |
| before-NC | 0 | 0 | — | no baseline effect |
| before-VC | 0 | 0 | — | no baseline effect |
| between-AdjC | 0 | 0 | — | no baseline effect |
| between-A | 0 | 0 | — | no baseline effect |
| between-M | 0 | 0 | — | no baseline effect |
| between-VC | 0 | 0 | — | no baseline effect |
| beyond-AdjC | 0 | 0 | — | no baseline effect |
| beyond-A | 0 | 0 | — | no baseline effect |
| beyond-NC | 0 | 0 | — | no baseline effect |
| beyond-VC | 0 | 0 | — | no baseline effect |
| by-AdjC | 0 | 0 | — | no baseline effect |
| by-M | 0 | 36,185 | Avacyn, Guardian Angel; Noctis, Prince of Lucis; Alien Symbiosis | retained exclusion |
| by-NC | 0 | 0 | — | no baseline effect |
| during-AdjC | 0 | 0 | — | no baseline effect |
| during-M | 0 | 3,360 | Sphinx's Insight; Drumbellower; Tangle Kelp | retained exclusion |
| during-NC | 0 | 0 | — | no baseline effect |
| during-VC | 0 | 0 | — | no baseline effect |
| except-AdjC | 0 | 0 | — | no baseline effect |
| except-M | 0 | 0 | — | no baseline effect |
| except-NC | 0 | 0 | — | no baseline effect |
| except-VC | 0 | 0 | — | no baseline effect |
| face down-AdjC | 0 | 0 | — | no baseline effect |
| face down-M | 0 | 1,062 | Moonring Mirror; Necropotence; Suppress | retained exclusion |
| face down-NC | 0 | 0 | — | no baseline effect |
| face down-VC | 0 | 0 | — | no baseline effect |
| face up-AdjC | 0 | 0 | — | no baseline effect |
| face up-M | 0 | 12 | Primordial Mist | retained exclusion |
| face up-NC | 0 | 0 | — | no baseline effect |
| face up-VC | 0 | 0 | — | no baseline effect |
| for-AdjC | 0 | 0 | — | no baseline effect |
| for-M | 1 | 15,529 | Theft of Dreams; Vraska's Scorn; Chain of Plasma | DEFERRED |
| from-AdjC | 0 | 0 | — | no baseline effect |
| from-A | 17 | 11,772 | Disposal Mummy; Crypt Creeper; Game Trail | DEFERRED |
| from-M | 507 | 16,530 | Disposal Mummy; Pharika's Mender; Summon the School | DEFERRED |
| if-AdjC | 0 | 0 | — | no baseline effect |
| if-M | 0 | 1,474 | Vizier of Deferment; Gravetiller Wurm; Tangle Kelp | retained exclusion |
| if-NC | 0 | 0 | — | no baseline effect |
| if-VC | 0 | 0 | — | no baseline effect |
| in-AdjC | 0 | 0 | — | no baseline effect |
| in-A | 1 | 10,644 | Don Andres, the Renegade; Neurok Transmuter; Ghastly Remains | DEFERRED |
| in-NC | 0 | 0 | — | no baseline effect |
| in-VC | 0 | 0 | — | no baseline effect |
| into-AdjC | 0 | 0 | — | no baseline effect |
| into-M | 0 | 331 | Dwell on the Past; Survive; Cathartic Parting | retained exclusion |
| into-NC | 0 | 0 | — | no baseline effect |
| minus-AdjC | 0 | 0 | — | no baseline effect |
| minus-M | 0 | 0 | — | no baseline effect |
| minus-NC | 0 | 0 | — | no baseline effect |
| minus-VC | 0 | 0 | — | no baseline effect |
| of-AdjC | 0 | 0 | — | no baseline effect |
| of-A | 0 | 0 | — | no baseline effect |
| of-VC | 0 | 0 | — | no baseline effect |
| on-AdjC | 0 | 0 | — | no baseline effect |
| on-NC | 0 | 0 | — | no baseline effect |
| onto-AdjC | 0 | 0 | — | no baseline effect |
| onto-M | 0 | 0 | — | no baseline effect |
| onto-NC | 0 | 0 | — | no baseline effect |
| outside-AdjC | 0 | 0 | — | no baseline effect |
| outside-NC | 0 | 0 | — | no baseline effect |
| outside-VC | 0 | 0 | — | no baseline effect |
| over-AdjC | 0 | 0 | — | no baseline effect |
| over-NC | 0 | 0 | — | no baseline effect |
| plus-AdjC | 0 | 0 | — | no baseline effect |
| plus-M | 0 | 0 | — | no baseline effect |
| plus-NC | 0 | 0 | — | no baseline effect |
| plus-VC | 0 | 0 | — | no baseline effect |
| since-AdjC | 0 | 0 | — | no baseline effect |
| since-M | 0 | 28 | Keldon Twilight | retained exclusion |
| since-NC | 0 | 0 | — | no baseline effect |
| since-VC | 0 | 0 | — | no baseline effect |
| than-AdjC | 0 | 0 | — | no baseline effect |
| than-M | 0 | 0 | — | no baseline effect |
| than-NC | 0 | 0 | — | no baseline effect |
| than-VC | 0 | 0 | — | no baseline effect |
| though-AdjC | 0 | 0 | — | no baseline effect |
| though-M | 0 | 0 | — | no baseline effect |
| though-NC | 0 | 0 | — | no baseline effect |
| though-VC | 0 | 0 | — | no baseline effect |
| through-AdjC | 0 | 0 | — | no baseline effect |
| through-NC | 0 | 0 | — | no baseline effect |
| to-M | 83 | 32,467 | Vedalken Heretic; Saltwater Stalwart; Awaken the Sky Tyrant | DEFERRED |
| under-AdjC | 0 | 0 | — | no baseline effect |
| under-NC | 0 | 0 | — | no baseline effect |
| unless-AdjC | 0 | 0 | — | no baseline effect |
| unless-M | 0 | 230 | Grip of Amnesia; Thirst for Identity; Waterspout Djinn | retained exclusion |
| unless-NC | 0 | 0 | — | no baseline effect |
| unless-VC | 0 | 0 | — | no baseline effect |
| until-AdjC | 0 | 0 | — | no baseline effect |
| until-M | 2 | 3,392 | Trapjaw Tyrant; Vault Guardsman; Sauron, the Lidless Eye | DEFERRED |
| until-NC | 0 | 0 | — | no baseline effect |
| until-VC | 0 | 0 | — | no baseline effect |
| up-AdjC | 0 | 0 | — | no baseline effect |
| up-M | 0 | 0 | — | no baseline effect |
| up-NC | 0 | 0 | — | no baseline effect |
| up-VC | 0 | 0 | — | no baseline effect |
| while-AdjC | 0 | 0 | — | no baseline effect |
| while-M | 0 | 257 | Fire Lord Azula; Astral Drift; Social Snub | retained exclusion |
| while-NC | 0 | 0 | — | no baseline effect |
| while-VC | 0 | 0 | — | no baseline effect |
| with-AdjC | 0 | 0 | — | no baseline effect |
| with-A | 147 | 32,450 | High-Rise Sawjack; Kykar, Zephyr Awakener; Metamorphosis Fanatic | DEFERRED |
| with-NC | 0 | 0 | — | no baseline effect |
| within-AdjC | 0 | 0 | — | no baseline effect |
| within-NC | 0 | 0 | — | no baseline effect |
| within-VC | 0 | 0 | — | no baseline effect |
| without-AdjC | 0 | 0 | — | no baseline effect |
| without-NC | 0 | 0 | — | no baseline effect |
| without-VC | 0 | 0 | — | no baseline effect |
| when-AdjC | 0 | 0 | — | no baseline effect |
| when-M | 0 | 300 | Seraph; Tatsumasa, the Dragon's Fang; Flamehold Grappler | retained exclusion |
| when-NC | 0 | 0 | — | no baseline effect |
| when-VC | 0 | 0 | — | no baseline effect |
| whenever-AdjC | 0 | 0 | — | no baseline effect |
| whenever-M | 0 | 136 | Shrine of Limitless Power; Shrine of Boundless Growth; Shrine of Loyal Legions | retained exclusion |
| whenever-NC | 0 | 0 | — | no baseline effect |
| whenever-VC | 0 | 0 | — | no baseline effect |


The draft table's function sets are proposals, not the final declarations.
The seven DEFERRED rows retain their baseline function in the declared set;
all other listed exclusions remain applied. Spatial P availability is retained
as recorded above. No new verb frame is declared by this narrowing.


## Deferred exclusions and prerequisites

Orchestrator resolution (2026-10-06; **user review pending**), applied by
independent measurement. These seven exclusions remain DEFERRED; their old
functions stay licensed. The causal examples below distinguish unavailable
selected frames from composition gaps. A face's inventory of lexical verbs
is provenance and does not establish that every verb selects that PP.
`english-v3-systemic-residuals` owns the prerequisites; this ticket re-applies
each exclusion after the replacement analyses exist and the same attribution
check passes. No prerequisite frame is added in this landing.

- **with-A** (147 faces; 32,450 Readings): consume Enter's declared
  `Preposition(With), Object, Preposition(On), Complement` frame, reconciling
  legacy `Object` and `FrameComplement` with typed categories. Gravetiller
  Wurm, Flycatcher Giraffid and Marketback Walker establish that dependency.
  Exile's declared `[ObjectNounPhrase, Role("ResultativeComplement")]` needs
  its resultative consumer; Return/Put resultative uses need declared and
  consumable Complement slots. Circumstantial With Adjuncts, including those
  in death clauses, must remain legitimate rather than become invented Die
  frames. Re-apply with-A here only after distinguishing those uses.
- **from-M** (507 faces; 16,530 Readings): consume Put's legacy source-bearing
  frames (`Object`/`FrameComplement` reconciliation), and declare consumable
  From source slots for Exile, Discard, Reveal, Choose and Cast where selected.
  Disposal Mummy (Exile), Game Trail (Reveal) and Gix's Caress (Discard) are
  causal witnesses. Return's existing optional source/controller frame was
  corrected before narrowing and is consumable; it does not repair those
  other verbs. Comeuppance/Vazi remain covered because the whole function is
  deferred. Re-apply from-M here after the source-frame prerequisites land.
- **from-A** (17 faces; 11,772 Readings): declare and consume Enter's From
  source (Phyrexian Dragon Engine, Dredging Claw, Triarch Praetorian) and
  Cast's source after a pronominal object (Furnace Dragon, Wakening Sun's
  Avatar). Jason Bright, Glowing Prophet also needs the adjective Different's
  From Complement valence and its consumer; this is not a Be source slot.
  Re-apply from-A here after those selected analyses exist, retaining genuine
  source Adjuncts.
- **to-M** (83 faces; 32,467 Readings): consume Addition's selected To
  Complement composition, including the fixed `in addition to` expression
  (Don Andres, the Renegade; Neurok Transmuter; Blanket of Night). Consume the
  existing Attach `[ObjectNounPhrase, Preposition(To), ObjectNounPhrase]`
  frame in reduced passives (`Equipment attached to ...`), and Deal's existing
  recipient-gap pattern under perfect auxiliaries (The Fallen's `it has dealt
  damage to [gap] this game`). Re-apply to-M here after these consumers land.
- **in-A** (one face; 10,644 Readings): Indigo Faerie's `becomes blue in
  addition to its other colors until end of turn` needs a compositional
  `in addition to` fixed-PP consumer retaining outer Adjunct scope. It does
  not establish a selected In slot for Become. Re-apply in-A here after that
  composition exists and legitimate locative Adjuncts are distinguished.
- **for-M** (one face; 15,529 Readings): Kemba's Legion's `can block an
  additional creature each combat for each Equipment attached to this
  creature` needs distributive For-each scope outside the Object NP and the
  reduced passive Attach-To consumer. Block's existing `[ObjectNounPhrase]`
  frame must compose with those modifiers; this does not establish a
  selected For slot for Block. Re-apply for-M here after that composition.
- **until-M** (two faces; 3,392 Readings): Deputy of Detention and Banishment
  need the duration Adjunct outside Exile's coordinated Object NP, composing
  with `with the same name as ...`. Consume the existing Exile
  `[ObjectNounPhrase]` frame through that coordinated nominal/comparison
  structure; no selected Until slot is implied. Re-apply until-M here after
  that composition lands.

The expected zero attested noun uses of During is confirmed as provenance,
but its baseline grammar nevertheless admitted **3,360 wrong M Readings**.
Removing them loses zero faces. By-M removes **36,185 wrong M Readings** and
loses zero faces, including Avacyn (29,800), Noctis (1,260), and Alien Symbiosis
(260). From-M and To-M depend on frames/composition; With-A is deferred as
predicted. The seven independent lost-face counts overlap: together the
original exclusions lost 1,158 faces, not their arithmetic sum.

## Final effective licence table

This table is generated from the declarations measured on `luxpqklz`, final
covered count 13,716. It replaces the draft's effective licences. P on spatial
prepositions preserves the prior LocativeUse-based analyses. CmpC means
Comparative Complement; an empty set admits no PP function. CP membership does
not imply that every compound expression already has a consumer.

| Preposition | Effective licence | Deferred exclusion |
|---|---|---|
| Than | CmpC | — |
| Minus | empty | — |
| Plus | empty | — |
| Up | CP | — |
| If | A | — |
| Unless | A | — |
| As | CP, A | — |
| Though | A | — |
| While | A | — |
| Until | A, M | M |
| Since | A | — |
| Because | A, CP | — |
| When | A | — |
| Whenever | A | — |
| After | A | — |
| Against | A, VC, M | — |
| Among | VC, M, CP, P | — |
| At | A, VC, CP, P | — |
| Before | A | — |
| Between | NC, P | — |
| Beyond | M, P | — |
| By | VC, A | — |
| During | A | — |
| Except | A | — |
| For | VC, NC, CP, A, M | M |
| From | VC, NC, CP, M, A | A, M |
| In | M, P, CP, A | A |
| Into | VC | — |
| Of | NC, M, P | — |
| On | M, VC, A, P | — |
| Onto | VC | — |
| Outside | A, M, P | — |
| Over | A, M, VC, P | — |
| Through | A, VC, M | — |
| To | VC, AdjC, NC, CP, M | M |
| Under | VC, A, M, P | — |
| With | M, VC, A | A |
| Within | A, M, P | — |
| Without | A, M | — |
| Instead | A, CP | — |
| face down, face up | A, P | — |


### Gate repairs within the ticket

The first compiler-only gate failure was the added zero-input-table rejection
fixture expecting a later diagnostic; it now checks the existing earlier
validation diagnostic. No rejection assertion is removed.

The existing Noctis independent-value test also constructed the retired By-M
attachment to *graveyard*. Its intended outer means Adjunct and complete
`paying 3 life in addition to paying their other costs` Complement retain both
roundtrip laws; the nominal contrast is re-spelled as a negative admission and
parse-membership assertion. This is the deliberate By-M subject retirement,
not a deleted test or a relaxed value comparison.

An initial broad PastParticiple/Transitive Premodifier row admitted the existing
negative `defended player` and split `artifact named Powerstone Shard` into a
wrong nominal instead of retaining its independently authored exact set.
The existing tests caught this before landing. The repair reads declared
`PastParticipialPremodifier` availability as well as form and frame; Choose
records that distribution, and all pre-existing negative/value assertions stay
unchanged. The earlier full corpus on that broad row is superseded by the
corrected final census; its additions are not claimed as gains.


### Measured residual destination composition

Cavalier of Thorns and Genesis Ultimatum remain inherited no-Reading faces,
not losses. Exact probes on final `luxpqklz`, covered 13,716:
`among them` has one PP Reading; `from among them` has zero, as do
`a land card from among them` and
`any number of permanent cards from among them`. All four probes enumerate
completely with zero internal issues. The missing component is From's
PP Complement consumer: the current PrepositionComplement inventory admits
only None/NounPhrase, not a PP argument (CGEL Ch. 7 §2.1, pp. 605–606).
The source-bearing Object NPs therefore cannot compose even though the inner
Among PP is available. This distinct cause is routed to
`english-v3-systemic-residuals`, with exact final probe evidence
`/tmp/nominal-components-final-{0,1,2,3}.json`; it is not repaired by a nominal
exclusion or a new verb frame here.


## Landing record

### PROVE

Baseline and final tree: `luxpqklz`; baseline covered count 13,622, final
covered count 13,716, all 32,828 supported faces enumerated. The lexical
snapshot and support filter are unchanged. No face is lost: Comeuppance and
Vazi, Keen Negotiator remain covered under the deferred From-M function.
The independent per-exclusion table above reproduces all 176,171 baseline
identities and measures every exclusion, including Of-A/Of-VC. Its complete
named lost-face lists, Reading hashes and sentences remain in ignored evidence;
the seven deferrals are recorded above and routed to systemic residuals.

The exact final diff removes **53,109** baseline Reading identities and adds
**27,262**, with **zero unexplained removals**. Removed counts by independently
measured function follow; a Reading can witness more than one exclusion, so
these counts overlap. Added hashes and their source faces are in ignored
`added-readings.jsonl`; additions on previously covered faces include the
corrected optional Return frame alternatives as well as the witness repairs.
Deferred nominal ambiguities are still inherited defects, not gains credited
as corrected analyses.

| Retired function | Final removed Reading witnesses |
|---|---:|
| Among-A | 1,284 |
| As-M | 263 |
| At-M | 8,692 |
| By-M | 36,185 |
| During-M | 3,360 |
| FaceDown-M | 1,062 |
| FaceUp-M | 12 |
| If-M | 1,474 |
| Into-M | 331 |
| Since-M | 28 |
| Unless-M | 230 |
| While-M | 257 |
| When-M | 300 |
| Whenever-M | 136 |

The final Reading identity diff is recorded below. Every removed identity is
named with its complete source sentence(s), retired function and grammatical
witness in the ignored `removed-readings.jsonl` / `removed-readings.md` evidence.
No removed identity lacks an independently measured retained exclusion. No
legitimate Reading is intentionally retired, and no re-coverage obligation or
unexplained loss is created. The existing wrong From/To nominal analyses remain
licensed while their replacements are incomplete; they are deferred defects,
not newly claimed coverage gains.

All 150,324 final Readings pass declaration admission, lexical ownership,
byte-exact realization, construction traversal and lexical leaf traversal
identity against independent materialization traces. Internal failures,
cyclic derivations, duplicate Readings, incomplete enumerations and issues are
zero. Independently authored witnesses prove the converse roundtrip law and
complete-value equality for Animal Magnetism's exact two-Reading set, Hurricane,
Library of Leng and the authored predicative Of values. Raise Dead independently
compares the complete selected optional source value; the other Of witnesses
assert continued parsing and validate every returned Reading's realization.
Seedborn has exactly two attachments plus the direct nominal negative.
Multiple grammatical Readings remain preserved, as the current lexical
analysis decision requires; no destructive selection or zero-tie gate applies.

Forbidden word/card/lexeme-named admission guards added: zero. Function guards
read one declared PrepositionFunctionLicence set, marker guards also retain
the declared lexical marker class, and past-participial Premodifier admission
reads declared availability, form and frame. Lexical source/environment loading
succeeds with zero errors. The active v3 stack has no legacy `environment.rs`
or emitted legacy permitted-licensing-checker total; these are not fabricated
or obtained from the retired coverage pipeline. V3's authority is
`deckmaste_lexical_source::load_workspace` and the declared feature checks.

### DISCLOSE

| Tree / covered | No Reading | Unique | Multiple | Complete Readings |
|---|---:|---:|---:|---:|
| `luxpqklz` baseline / 13,622 | 19,206 | 6,653 | 6,969 | 176,171 |
| `luxpqklz` final / 13,716 | 19,112 | 6,421 | 7,295 | 150,324 |

Specificity-resolved selections: zero before and after. The current parser
preserves all Readings; uniqueness is a census, not a selection rule. Optional
source/controller Return frame alternatives coexist with its direct destination
frame. No construction pair gains a destructive specificity preference.

Every newly covered face identity follows, with its selected constituent
analysis and full-Document Reading count. The 94 additions require the witness
repairs: declared past-participial Premodifier (72 faces), partitive NP (13),
both (4), predicative Of with the participial Premodifier (3), or compound
Instead-of PP (2). The superseded broad-participle census's extra gains are
excluded; the existing independent negative tests caught and prevented them.

| Identity | Card face | Analysis admitted | Readings |
|---|---|---|---:|
| `005ee549-1bf5-478f-bc3f-3e791bd7eecf#card` | Kindred Discovery | declared past-participial Premodifier | 1 |
| `0114f97f-b196-4af1-90aa-eba164cb8f22#card` | Etchings of the Chosen | declared past-participial Premodifier | 2 |
| `0ea6d165-2c30-41a4-8829-6d0510afc03f#card` | And They Shall Know No Fear | declared past-participial Premodifier | 6 |
| `107d5f67-1395-444f-b612-c5cc770df956#card` | Legerdemain | partitive NP | 44 |
| `110d5d45-7174-4d1b-9e9c-e1ccd950ee3b#card` | Gahiji, Honored One | partitive NP | 6 |
| `112322ad-8f66-4cd4-98a1-f425d61a69ce#card` | True-Name Nemesis | declared past-participial Premodifier | 1 |
| `129ee7d6-e3df-40ef-9502-7aad10183031#card` | Prism Ring | declared past-participial Premodifier | 2 |
| `17770eeb-ecbf-4510-af2d-acececfbe146#card` | Multiversal Passage | declared past-participial Premodifier | 1 |
| `17ae63fa-fd63-45e6-81c1-bf30851bf77c#card` | Illusionary Terrain | declared past-participial Premodifier | 8 |
| `1f91297a-ec2b-4ea8-9198-aa1daac20ff8#card` | Metallic Mimic | declared past-participial Premodifier | 147 |
| `240e85d3-e495-4877-8609-4b4056c402f7#card` | Victimize | declared past-participial Premodifier | 15 |
| `24108160-e72b-4451-b1b1-b2b3ac3ede85#card` | Threats Undetected | partitive NP; declared past-participial Premodifier | 52 |
| `2494daad-c81d-4a80-ba5d-e7011af8de46#card` | Maeve, Insidious Singer | partitive NP | 1 |
| `2518a4f2-607f-41bb-9389-2009138d6b09#card` | Metamorphic Alteration | declared past-participial Premodifier | 1 |
| `29ae4471-c28f-4e43-8afb-fee5023e4d08#card` | Diamond Knight | declared past-participial Premodifier | 2 |
| `29ec9b49-fbe5-49ac-8ad6-7159cb88bc66#card` | Roots of Life | declared past-participial Premodifier | 3 |
| `2a64cee5-04e2-4cbf-8fa9-d55c854f9500#card` | Genestealer Locus | partitive NP | 9 |
| `2dc6da5d-c4d1-4f2c-8f46-11a935bc0044#card` | Blackmail | partitive NP | 3 |
| `38503d34-f4f9-4eeb-9b1e-4fec1df921b3#face:0` | An Unexpected Party | declared past-participial Premodifier | 1 |
| `3c889b2e-380a-4bc4-8d56-5fb12bc7c78b#card` | Nyxathid | declared past-participial Premodifier | 7 |
| `3ee68758-9431-4446-8539-f42f545f3bd0#card` | Celestial Regulator | declared past-participial Premodifier | 120 |
| `410b7faf-365c-4b73-a64d-bddbe49cfded#card` | Alloy Golem | declared past-participial Premodifier | 1 |
| `4634fd4e-5b38-4cfb-ab73-bf3b14b3bdee#card` | Shifting Sky | declared past-participial Premodifier | 1 |
| `46e50ee9-91f8-416e-82fa-f1a76e61fb73#card` | Belbe's Portal | declared past-participial Premodifier | 7 |
| `48c127f0-2857-4c36-97bc-1291b6fe4a82#card` | Steely Resolve | declared past-participial Premodifier | 1 |
| `51d6d164-6e5f-4be3-8f09-433e0b8d4c6b#card` | Bloodline Pretender | declared past-participial Premodifier | 1 |
| `51feda69-0ae8-4df3-bb50-a4bb9deb965d#card` | Species Specialist | declared past-participial Premodifier | 1 |
| `53c730c6-2f8c-4af8-b400-b9d573a71e60#card` | Adaptive Automaton | declared past-participial Premodifier | 21 |
| `58aec411-167d-4709-8560-793eaaed62c5#card` | Gifts Ungiven | partitive NP; declared past-participial Premodifier | 19 |
| `5e816472-561d-43d1-ad28-80a829e2a91a#card` | Iona, Shield of Emeria | declared past-participial Premodifier | 1 |
| `60dabe9e-4872-4aec-9e8e-35358b3b7dd5#card` | Riders of Gavony | declared past-participial Premodifier | 4 |
| `6437867b-a658-4a71-a8a5-0c5a782d1f84#card` | Ashes of the Fallen | declared past-participial Premodifier | 22 |
| `649e4969-71d4-434f-85b0-205ec2c6b24f#card` | Collective Inferno | declared past-participial Premodifier | 1 |
| `64d022ab-5222-4a7a-ab8a-89749560f666#card` | Shared Triumph | declared past-participial Premodifier | 1 |
| `66418c65-a15f-489e-8a2d-6cee1b5d977c#card` | Order of the Stars | declared past-participial Premodifier | 1 |
| `685c4665-e04c-4012-aa5b-888a042c2a21#card` | Convincing Mirage | declared past-participial Premodifier | 1 |
| `68f81fcf-2dcf-41b9-ad7c-588149feb12a#card` | Antagonism | partitive NP | 14 |
| `70973a2d-66e6-4662-ab10-99678775dda1#card` | Dauntless Bodyguard | declared past-participial Premodifier | 2 |
| `716632dd-2947-4641-a1db-50e0a2b0b4e8#card` | Hall of Triumph | declared past-participial Premodifier | 1 |
| `7264d3b3-fd46-4ea3-a85d-f0b068c331ad#card` | Calculating Lich | partitive NP | 1 |
| `76642e7a-3c8f-45b8-9cd7-dac8086bea87#card` | Xenograft | declared past-participial Premodifier | 7 |
| `76d07c99-8c6b-4c2c-a0f6-47541999015b#card` | From the Rubble | declared past-participial Premodifier | 224 |
| `776ea0c7-fc5b-44b4-9302-5a3514ff843b#card` | Tyrannical Pitlord | declared past-participial Premodifier | 1 |
| `7c259e83-1d85-4267-a2e6-1a0fbacae3fe#card` | Ward Sliver | declared past-participial Premodifier | 1 |
| `7dcbce46-2973-4a9f-93df-95ac41ce668a#card` | Phantasmal Terrain | declared past-participial Premodifier | 1 |
| `8353f834-678a-4fa1-857a-8bfdfb8d6378#card` | Lurebound Scarecrow | declared past-participial Premodifier | 1 |
| `83b480ea-77c5-4e00-99d0-bea2a60a1bfe#card` | Kabira Evangel | declared past-participial Premodifier | 5 |
| `8b5659bd-a272-44cd-811f-9feec7d9825f#card` | Pack's Disdain | declared past-participial Premodifier | 9 |
| `8b834f83-951b-4b5c-b9f1-3d860baa3c0e#card` | Winnowing | declared past-participial Premodifier | 90 |
| `8be46af9-1b70-4da9-b16d-869c3831dc99#card` | Instruments of War | declared past-participial Premodifier | 1 |
| `8cf38025-5821-45a0-9483-266353b7e82d#card` | Vanquisher's Banner | declared past-participial Premodifier | 4 |
| `8e862a8e-7954-43ad-9bc0-09ce7b9859f0#card` | Bathe in Light | declared past-participial Premodifier | 72 |
| `924ef69f-9977-439c-ace9-353b429e4be6#card` | Gauntlets of Chaos | partitive NP | 148 |
| `9a440122-a015-4af2-b270-37f019884458#card` | Diamond Mare | declared past-participial Premodifier | 2 |
| `9b6d3dcf-aa5a-4516-bd48-a0723e86bfd1#card` | Door of Destinies | declared past-participial Premodifier | 10 |
| `9d73a51a-1c8e-40cd-8124-aeff406a0884#card` | Brave the Elements | declared past-participial Premodifier | 8 |
| `9e73f938-0aa4-4639-a97a-f7e68f645623#card` | Deliver Unto Evil | partitive NP; declared past-participial Premodifier | 56 |
| `9fa75b3c-a512-4318-9c03-e2d958acb6ef#card` | Mazzy, Truesword Paladin | partitive NP | 12 |
| `a3fc6888-0a4b-4e1d-b709-042b8b6b1a04#card` | Akroma's Blessing | declared past-participial Premodifier | 4 |
| `a9c2bf1b-2d5e-42c5-bc76-8429aaae161c#card` | Bloodline Shaman | declared past-participial Premodifier | 4 |
| `ad5e2cb0-00d7-4dad-b20e-23bd41ada398#card` | Prismatic Ward | declared past-participial Premodifier | 28 |
| `aee5dda1-2181-40d5-9846-85d154e1d5da#card` | Prismatic Boon | declared past-participial Premodifier | 4 |
| `b1631e9c-a993-4d44-b009-b2624d79560c#card` | Voice of All | declared past-participial Premodifier | 1 |
| `b34b5b3f-7f17-4292-814e-634408a5d7a5#card` | Sudden Demise | declared past-participial Premodifier | 1 |
| `b5dce42a-a769-4d7d-b29e-8722f79d4092#card` | Kindred Judgment | predicative Of PP; declared past-participial Premodifier | 3 |
| `bad869dd-e844-46ca-be53-73c4bc19d09c#card` | Realmwright | declared past-participial Premodifier | 7 |
| `baf93873-35e6-4bf2-bdcc-78a5206422fb#card` | Painter's Servant | declared past-participial Premodifier | 105 |
| `bdb9e933-f679-4930-b669-0d123db09d16#card` | Bloodline Bidding | declared past-participial Premodifier | 16 |
| `c2531360-61f8-4edc-886b-408811a0639d#card` | Alhammarret, High Arbiter | declared past-participial Premodifier | 24 |
| `c3fdfb94-2d10-4743-864c-a59fdd57d8b7#card` | Reflections of Littjara | declared past-participial Premodifier | 2 |
| `c637fedc-b7b9-4da8-bab4-0488e7c1fac1#card` | Noggin Whack | partitive NP | 3 |
| `c761af28-d7d8-46d3-9bc7-1e44fdf6d0a0#card` | Doom Cannon | declared past-participial Premodifier | 1 |
| `ccaa44f2-96be-44e2-884f-c31baa3908d5#card` | Kindred Dominance | predicative Of PP; declared past-participial Premodifier | 3 |
| `d660f6a0-3642-4445-b877-fda2a2950680#card` | Fell Beast's Shriek | declared past-participial Premodifier | 1 |
| `d71cd08e-3e84-41ff-b9db-9e343c0af6b4#card` | Remand | fixed Instead-of PP | 6 |
| `d78654fc-7d90-40ce-9c36-a211424d1a88#card` | Cover of Darkness | declared past-participial Premodifier | 1 |
| `dafd319e-1d4a-4e08-9167-a82ad8670f6f#card` | Teferi's Moat | declared past-participial Premodifier | 3 |
| `dc386f6c-bd4a-462a-8d42-0e7a552a4f17#card` | Lynde, Cheerful Tormentor | partitive NP | 30 |
| `dc8210b4-8e17-4156-994b-e95e215e3e34#card` | Engineered Plague | declared past-participial Premodifier | 1 |
| `def2087a-5020-41e8-aa7b-d1f0c1f0fdb6#card` | Skyboon Evangelist | partitive NP | 6 |
| `df130616-8efe-42ed-a7f0-b58bef3f11c4#card` | Plague Engineer | declared past-participial Premodifier | 3 |
| `e0d78210-ba85-439f-bf1f-92ef80b6030d#card` | Curse of Wizardry | declared past-participial Premodifier | 1 |
| `e21c8fc6-d4ef-42b6-b11e-d9c931da1387#card` | Realms Uncharted | partitive NP; declared past-participial Premodifier | 26 |
| `e2ef3a24-9e78-47fc-9192-049aa0ddb7a0#card` | Obelisk of Urd | declared past-participial Premodifier | 1 |
| `e7a7ba65-ad14-41d5-889b-9bbeae9ab3f7#card` | Crippling Fear | predicative Of PP; declared past-participial Premodifier | 6 |
| `e949b362-4988-4482-ac51-c1049af9040c#card` | Chronicle of Victory | declared past-participial Premodifier | 4 |
| `ee3d1f44-e0ca-4ce9-be76-4b675a115156#card` | Desertion | fixed Instead-of PP | 36 |
| `eeb07c8a-21d1-43fa-a5f3-ee3fe328a671#card` | Rally the Ranks | declared past-participial Premodifier | 1 |
| `f04441b0-755c-4e7b-a9e4-4e03b8175ab6#card` | Kindred Boon | declared past-participial Premodifier | 5 |
| `f27f91fb-009a-4cbd-858e-15db88b65b9f#card` | Pentarch Paladin | declared past-participial Premodifier | 1 |
| `f2a6e26c-6bd6-4fca-ba9f-d52d01fc03ef#card` | Volrath's Laboratory | declared past-participial Premodifier | 12 |
| `f3d935cd-d582-464a-a13b-bb84a53da4a1#card` | Flameskull | partitive NP | 2 |
| `f557fb2c-581b-4a8d-8766-1e46e92224fa#card` | Shimmer | declared past-participial Premodifier | 1 |
| `f62250c6-0832-46c8-a800-806589504f5f#card` | Titan of Littjara | declared past-participial Premodifier | 1064 |


Deviations and additions:

- The compiler gains generic set-feature containment/intersection operations
  and a declared selected-frame marker-licence policy. This avoids a parallel
  collection of boolean features or guards naming prepositions.
- PredicativePreposition consumes the existing copular PredicativeComplement
  frame; Prepositional is added to PredicativeKind and its coordination rows.
- CompoundPrepositionPhrase and the lexical Instead Preposition declare the
  Library of Leng witness. The existing Instead Adverb owner remains available.
- PartitiveNounPhrase and PartitiveModifier compose Incriminate's fused-head
  cardinal partitive with its choice modifier; no independent test is removed.
- The existing VerbalPremodifier gains declared past-participial availability
  to consume *chosen type*. Existing Name/Defend negatives retain their exact
  assertions; the broad transitive row was repaired before the final census.
- Return's existing optional source/controller frame is corrected to typed
  NounPhrase arguments and marked optional slots; its frame count remains two.
  This correction predates the narrowing resolution; no new verb frame is
  added, and no further frame declaration is changed during narrowing.
- Prior spatial P availability becomes explicit and is checked in
  LocativeComplement, preserving the old LocativeUse-based copular analyses.
- Seven functions remain DEFERRED by measured coverage: with-A, from-A,
  from-M, to-M, in-A, for-M and until-M. Exact prerequisites and re-application
  ownership are recorded above and in systemic residuals.
- Lantern of Undersight is replaced with Vintage-supported Library of Leng,
  as the user authorized. No funny-set card enters the support inventory.
- Existing tests re-spelled: **2** (Seedborn attachment set, Noctis nominal
  contrast). Restored **0**; added **6** (five preposition witness tests and
  one compiler set-feature validation test); ignored **0**; removed **0**.
- The grammar glossary now defines Compound Preposition and Partitive Noun
  Phrase and describes the licensed past-participial Premodifier distribution.
  These were the glossary gaps needed by this landing; CGEL citations accompany
  their definitions. No CR citation is introduced or changed.

STOPs and their resolutions are recorded above: Raise Dead (orchestrator,
2026-10-06, user review pending; the user's verbatim words are quoted),
unattested licences (orchestrator, same date, user review pending), the
out-of-scope Lantern witness (user-approved Library substitution), and the
1,158-face broad-table loss (orchestrator, same date, user review pending;
measured narrowing and seven deferrals). The initial own compiler fixture and
within-ticket test defects are fixed in the retained tests/code, as described
in Gate repairs; they are not waivers. No unresolved ruling contradiction
remains. Residual destination PP composition belongs to systemic residuals.

### REPORT

The active v3 corpus command has no legacy coverage lock. Its measured covered
count is **13,716** (`luxpqklz`), from 13,622 on the baseline of the same change.
The tracked legacy lock is unchanged and supplies no v3 admission authority.
Named grammar declarations: **179 ordinary Constructions + 44 shared schemas
= 223 constructor names**, **135 Categories**, **545 static productions** and
**611 environment-compiled productions**. These are grammar inventory
provenance on final `luxpqklz`, covered 13,716, not corpus-driven licences.

Homographs: the baseline named inventory has 120 surfaces; final has 121,
adding *instead* (`vocab:ReplacementMarker/Instead` and
`vocab:Preposition/Instead`). Full named owners are in ignored
`nominal-licensing-final-inventory.tsv`; all surfaces are listed below.
Form-literal/vocabulary overlaps: **none**, before and after. Multiword lexical
owners stay indivisible; these inventories are provenance, not filters.

Homograph surfaces: `'d`, `'s`, `Adamant`, `Addendum`, `Alliance`, `Battalion`, `Bloodrush`, `Celebration`, `Channel`, `Chroma`, `Cohort`, `Constellation`, `Converge`, `Council's dilemma`, `Coven`, `Delirium`, `Descend 4`, `Descend 8`, `Disappear`, `Domain`, `Eerie`, `Eminence`, `Enrage`, `Fateful hour`, `Fathomless descent`, `Ferocious`, `Flurry`, `Formidable`, `Grandeur`, `Hellbent`, `Heroic`, `I`, `Imprint`, `Infusion`, `Inspired`, `Join forces`, `Kinship`, `Landfall`, `Lieutenant`, `Magecraft`, `Metalcraft`, `Morbid`, `More Than Meets the Eye`, `Opus`, `Pack tactics`, `Paradox`, `Parley`, `Radiance`, `Raid`, `Rally`, `Renew`, `Repartee`, `Revolt`, `Secret council`, `Spell mastery`, `Strive`, `Survival`, `Sweep`, `Tempting offer`, `Threshold`, `Undergrowth`, `Valiant`, `Vivid`, `Void`, `Will of the council`, `control`, `copies`, `copy`, `cost`, `costs`, `counter`, `counters`, `cycling`, `deathtouch`, `decayed`, `die`, `double strike`, `draw`, `draws`, `exalted`, `exile`, `exiles`, `first strike`, `flying`, `goaded`, `harnessed`, `haste`, `her`, `hexproof`, `his`, `if`, `indestructible`, `instead`, `it`, `less`, `lifelink`, `menace`, `name`, `names`, `one`, `reach`, `shadow`, `solved`, `suspected`, `tapped`, `target`, `targets`, `that`, `time`, `to`, `trample`, `turn`, `turns`, `untap`, `untapped`, `up`, `vigilance`, `you`, `’d`, `’s`, `∞`.


Performance advisory and verification results are recorded after the completed
checks below. Evidence lives under ignored
`.superpowers/english-v3-selected-preposition-nominal-licensing/`, and is retained
in the coordinator checkout when the feature workspace is dropped. No corpus
snapshot or task-scoped verifier is embedded in a source crate.


Performance advisory (final `luxpqklz`, covered 13,716): the timed complete
`cargo xtask english-v3 --all --workers 8 --samples-per-face 0` command took
42,765,692,816 ns wall, above the quiet-host ceiling
of 16,260,000,000 ns. Eight workers, host load [2.638671875, 5.53076171875, 5.76220703125]; the
reverse-dependency gate's Lean checks were running concurrently, so this is
not a quiet-host benchmark. Reported setup 5,977,870,075 ns and corpus
35,884,353,255 ns; checked-text thread CPU **133,395 ns/B**.
The independently timed run reproduces every final face's Reading count,
constructor inventory and zero issues. Baseline `luxpqklz`, covered 13,622:
corpus wall 39,727,731,126 ns, eight workers, host load
[2.02783203125, 4.7421875, 4.98193359375], checked-text thread CPU
158,266 ns/B; no independent complete-command timer was captured for that
baseline, so its reported corpus phase is distinguished from final command
wall. Performance remains advisory, owned by english-v3-census-tractability.

Final census source: `/tmp/nominal-licensing-final.json`;
separate complete-command timing/provenance:
`/tmp/nominal-licensing-performance.json` and
`/tmp/nominal-licensing-command-time.json`. Final Reading hashes:
`/tmp/nominal-licensing-final-readings.jsonl`. All counts above refer to
`luxpqklz` / covered 13,716 (or the explicitly stamped baseline), rather than
an unstamped post-integration measurement.


Verification on final `luxpqklz`, covered 13,716:

- `cargo xtask gate --changed --run --clippy` exits 0. Derived reverse-dependency
  test command: `cargo test -p deckmaste_construction_v3_core -p
  deckmaste_lexical_source -p deckmaste_construction_v3 -p
  deckmaste_english_v3 -p xtask`; all 714 tests/doc-tests pass,
  including the real Lean gate checks. No test is removed or newly ignored.
- Derived strict lint command: `cargo clippy -p
  deckmaste_construction_v3_core -p deckmaste_lexical_source -p
  deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets --
  -D warnings`; green.
- Kata `kanban check`: no duplicates, cycles or dangling needs.
- From the feature workspace root, `cargo xtask cite check
  --list-noncompliant`: zero; `cargo xtask cite check`: 16,059 sites, zero
  stale. `jj diff --git | cargo xtask cite audit --diff` audits zero changed
  CR citation sites, consistent with the glossary's CGEL-only additions.
- `cargo fmt --all -- --check`: green. The generated runtime and selected-frame
  templates also match rustfmt under the repository configuration.
- Complete corpus enumerations and exhaustive before/after identities are
  checked as recorded above; full compiler/source reverse dependencies are
  included rather than only the focused witness tests.
