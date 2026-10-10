---
needs: []
---
# Declare game terms through their owning plugin definitions: another modified creature you control / the number of creatures in your party / Multikicker {1}{G}

A declaration ticket. No `declarations.rs` edit is expected. The owner
answered PT-Q1 and PT-Q2 on 2026-10-10, as recorded below.

## Why

By owner ruling (V3 of `docs/tickets/done/english-v3-vocabulary-batch-1.md`)
a game term is declared by the plugin definition that owns it, through that
definition's `grammar:` field, never in core. Batch 1 deferred a list under
"passes V2, blocked on declaration path"; since then
`english-v3-keyword-verb-export` gave `FixedKeyword` a `verb:` export and
`english-v3-predicate-term-declaration-path` gave predicate and event helpers
a `metadata: (grammar: …)` path (`predicates/historic.ron`,
`predicates/outlaw.ron`, `events/commitsCrime.ron`). What remains is terms
whose owner does not exist, or exists but does not export the word.

Blocker report at trunk `uoqyuxtnowpo`, `word:` table (sole-blocker faces,
touched faces); attestation by `cargo xtask english-v3 grep -i` (faces,
unread). "Owner" was established by searching `plugins_v2/` by name.

**A. No owning definition: needs a new definition file.**

| Term | Sole, touched | Faces, unread | Nearest sibling; family | Grammar row |
|---|---|---|---|---|
| *modified* | 42, 47 | 47, 47 | `predicates/historic.ron`; predicate | `Adjective` |
| *party*, *full party* | 28, 40 (*full* 1, 12) | 43, 42 (*full party* 12, 11) | none; a player-relative set (family open: PT-Q2) | `Noun`; *full party* open |
| *multikicker* | 19, 19 | 19, 19 | `keyword_abilities/kicker.ron`; keyword ability | `FixedKeyword`, Cost parameter |
| *expend* | 11, 12 | 12, 12 | `events/commitsCrime.ron`; event (*whenever you expend 4*) | verb + Amount |
| *descended* | 10, 12 | 29, 15 | `ability_words/descend4.ron`, `descend8.ron` are labels only; event (*if you descended this turn*) | verb, preterite |
| *poisoned* | 5, 5 | 5, 5 | `counter_kinds/poison.ron`; predicate of a player | `Adjective` |
| *Wicked*, *Monster*, *Young Hero*, *Cursed*, *Royal* (+ *Sorcerer*, *Virtuous* if unread) | 9+7+5+4+4 | 38, 34 for `(Wicked\|Monster\|Young Hero\|Cursed\|Royal\|Sorcerer\|Virtuous) Role` | `subtypes/enchantment/role.ron` owns *Role* only; predefined-token names (family open: PT-Q2) | name before *Role token* |
| *bands with other* ‹quality› | 7, 8 | 8, 8 | `keyword_abilities/banding.ron` (a different keyword); keyword ability | `FixedKeyword`, Quality parameter |
| *empower* | 9, 19 | 35, 35 | none found by name; *empower Jace 3* looks like a keyword action. Establish the owner from the CR, not from memory | verb + NP + Amount |

**B. Owner exists: data on an existing definition.**

| Term | Sole, touched | Owner | Missing export |
|---|---|---|---|
| *convoked* | 6, 6 | `keyword_abilities/convoke.ron` | `verb:` (*a creature that convoked it*) |
| *devoured* | 6, 6 | `keyword_abilities/devour.ron` | `verb:` (*each creature it devoured*) |
| *saddles* | 14, 16 | `keyword_abilities/saddle.ron` | `verb:`; was deferred for the *is saddled* pair, which `english-v3-keyword-state-adjectives` (KS4) rules to be a legitimate two-Reading pair. Do it here only after that ticket lands |
| *evolves* | 2, 2 | `keyword_abilities/evolve.ron` | `verb:` |
| *banded* | 3, 3 | `keyword_abilities/banding.ron` | participial Adjective or `verb:` with a *with* Complement |

**C. Owner exists, but the schema cannot say it: path gap, STOP material.**

| Term | Sole, touched | Owner | Gap |
|---|---|---|---|
| *day*, *night* | *night* 6, 16 | `designations/dayNight.ron` (`FixedTerm(surface: "day or night")`, `members: [day, night]`) | one `FixedTerm` surface; members export no noun. 16 faces match *it's day / night*, *becomes day / night*, *neither day nor night* |
| *promised* | 24, 24 | `keyword_abilities/gift.ron` (`FixedKeyword(surface: "gift")`) | *If the gift was promised*: needs the noun *gift* and a verb *promise* whose stem is not the keyword's |

## Goal

Every term in A and B is exported as a lexeme owned by a `plugins_v2`
definition and its attested units read; C is either expressed with the
existing rows or returned as a schema STOP with evidence.

## Analysis

No CGEL citation applies: this is ownership and declaration. Everything below
is a project ruling.

Existing pieces: the `Grammar` rows (`Verb`, `Noun`, `Adjective`, `FixedTerm`,
`FixedClause`, `FixedKeyword` with `participial_adjective:` and `verb:`) in
`crates/deckmaste_construction_core/src/macro_def.rs`; `read_builtin_v2` and
its eight `BUILTIN_FAMILIES`; the helper path of the predicate-term ticket
(PD1–PD8 there); the lexical loader `deckmaste_lexical_source/src/legacy/
plugins.rs`. Stub precedent: 37 of the 201 `keyword_abilities/*.ron` files
have no `body:` at all (`banding.ron` is `name` / `params` / `spelling` /
`grammar` only), and `ability_words/descend4.ron` likewise.

Drafting questions (answered by the owner below):

- **PT-Q1. What is the minimal acceptable body of a new definition?**
  semantics_v2 owns bodies (`docs/decisions/semantics-v2.md` §§10–16) and
  this is a grammar ticket. Options: (a) keyword-family files with no `body:`
  field, as 37 keyword abilities already are, and helper-family files
  (`predicates/`, `events/`) only when a real body is written, because their
  meta-shape (`kinds`, `params`, `body`) has no bodiless precedent; (b)
  bodiless stubs allowed in every family, each listed as a semantics debt;
  (c) no stubs: group A waits for semantics_v2 to write each definition and
  this ticket shrinks to B. The table's "Grammar row" column is the same
  under all three.
- **PT-Q2. Which family owns *party* and the Role token names?** Neither has
  a sibling. *party* is a set of a player's creatures; the Role names are
  predefined tokens. A new family is a loader change (`BUILTIN_FAMILIES`),
  outside a data ticket.

Orchestrator rulings 2026-10-10 (PT-Q1 and PT-Q2 answered below):

- PT1. One term, one owner: the definition named in the table, or the new
  file PT-Q1/Q2 authorise. Nothing in `lexicon/core.ron` or
  `vocabulary.rs`; no migration of existing core terms.
- PT2. Reuse existing `Grammar` rows and the landed `verb:` /
  `participial_adjective:` / helper `metadata` exports. A new row or field is
  a STOP with the term, the unit and the probe.
- PT3. Vocabulary-only test per term (V2 of batch 1): substitute a declared
  word of the same row in two attested units; declare only if both read.
  Evidence already taken on the `uoqyuxtnowpo` binary is under Witnesses.
- PT4. Existing bodies are untouched; only `grammar:` data is added to group
  B files.
- PT5. One analysis per string: no new lexeme may be a homograph of a
  declared one in the same Category. Named risks: *modified* against any
  participle of a verb *modify* (none declared: confirm); *party* against a
  subtype or name; *Monster* / *Royal* / *Cursed* as ordinary words inside
  card names; *night* against `counter_kinds/nightCounter.ron`; *saddled*
  (designation Adjective) against the saddle verb's participle.
- PT6. Skip and list any term that fails PT3 or needs a Construction (*full
  party* if *full* needs its own Adjective and frame; *expend 4* if a verb
  with a bare numeral Object needs a frame kind).
- PT7. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected except the one KS4 of
  `english-v3-keyword-state-adjectives` authorises for *is saddled*; any
  other is reported with its shape before landing.

### Owner rulings (2026-10-10)

- PT-Q1: option (b). Grammar-only declarations with no engine body are allowed
  in every family. List each new bodiless declaration as semantics debt.
- PT-Q2: declare Role names beside the predefined token definitions, following
  `instructions/treasureToken.ron` and that family's grammar conventions.
  If the schema cannot express their required Category, return the smallest
  schema question. Party is a bodiless noun-exporting stub; its selection
  mechanism remains semantics debt.
- Complete B and every A term expressible with existing rows before returning
  all schema/family questions together at the review STOP. Establish empower's
  owner from corpus and the CR; defer it if ownership cannot be confirmed.
- W9-A: one route for this construction; preserve independent ambiguity
  already present in the substituted control, and record both counts.
- Verification is bounded: all reachable surface faces plus 300 covered
  controls (seed 20261008), six workers, no stored samples. Stop after commit,
  refresh and any required bounded rechecks; integration awaits owner review.

## Witnesses

Probed 2026-10-10 on the `uoqyuxtnowpo` binary (Document, `--readings 3`):
term unit 0 with the word reported as a vocabulary gap; substituted control
as shown.

- "Whenever another modified creature you control dies, draw a card." 0
  (*legendary* 2); "… if it's modified, draw a card." 0 (*legendary* 1)
- "When this creature enters, scry X, where X is the number of creatures in
  your party." 0 (*in your graveyard* 3)
- "Multikicker {1}{G}" 0 ("Kicker {1}{G}" 1)
- "Whenever you expend 4, this creature gets +2/+1 until end of turn." 0
  (*Whenever you gain life, …* 1)
- "At the beginning of your end step, if you descended this turn, put a
  +1/+1 counter on this creature." 0 (*attacked* 2)
- "This spell costs {2} less to cast if it's night." 0 (*your turn* 2)
- "Create a Wicked Role token attached to target creature you control." 0
  (*a Role token* 3)
- "This creature has flying and trample if it devoured a creature." 0
  (*blocked* 1); "Counter target spell if its controller is poisoned." 0
  (*the monarch* 1)

Also 0: "At the beginning of combat on your turn, if you have a full party,
draw a card."; "If the gift was promised, draw a card." Controls that must
not change: every substituted sentence above; "Kicker {1}{G}" 1; "This
creature becomes saddled." 1.

## Method

Standard constraints apply. Plugin `grammar:` data loads at runtime: no
rebuild for data. Bounded census, never `--all`: every supported face
containing any declared word (`cargo xtask english-v3 grep -i --names`, one
pattern per word, unioned) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; PT7 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_lexical_source`,
`deckmaste_construction_core`, `deckmaste_english_v3` and `xtask` (it reads
`read_builtin_v2`); `cargo xtask lean check` because plugin definitions
change. Once, at the end. Tests assert the role per term (lexeme owner is the
plugin definition; Category; the unit's analysis), never an incidental count
or a Debug-printed tree.

## Out of scope

- Dice nouns *d4* / *d6* / *d8* / *d10* / *d12* (14 sole-blocker faces):
  *d20* is declared in core (`vocabulary.rs`, `D20`), so the siblings are
  core data by that precedent, or a migration question; not this ticket.
- *planeswalk*, *planar*: Planechase, permanently out of scope.
- *exactly*, *difference*, *tied*, *true*, *else*, *using*: ordinary
  vocabulary, not game terms.
- *saddled* after *while* (*attacks while saddled*, 26 faces): a
  construction gap listed in `english-v3-keyword-state-adjectives`.
- Bodies and Lean mirrors of new definitions beyond what PT-Q1 settles.

### Deferred terms and declaration questions

Claim-parent probes are under
`target/english-v3/english-v3-plugin-term-definitions/` (`before-probes.json`,
`extra-before-probes.json`, `gift-before-probes.json`). Counts below are on
`ztvzruvp`, whose grammar/data match claim parent `vwktxkol`.

- **Party / full party:** party witness 0, graveyard control 3; full-party
  witness 0, creature control 1. Which existing helper family/kind should own
  party's noun stub? `nouns/` has the relevant semantic helpers but the lexical
  reader accepts only `Predicate` and `GameEvent` helpers from `predicates/`
  and `events/`. Full party additionally lacks the adjective *full* (PT6).
- **Role names:** Wicked, Monster, Young Hero, Cursed, Royal and Virtuous
  witnesses 0 each; Role-token control 5 each. The predefined-token family is
  `instructions/`, with `CharacteristicBundle` kind (`treasureToken.ron`).
  It has no lexical reader/grammar convention. May that family export an
  adjective recipe for its names? Sorcerer already reads (9; control 5), so
  its witness is dropped as directed; no duplicate lexical owner is added.
- **Day/night:** both witnesses 0; your-turn controls 1/2 respectively.
  How should a multi-member Designation export each member as a noun while
  retaining the existing *day or night* owner?
- **Gift/promise:** original witness 0, whole-clause control 2. Two attested
  units retaining *the gift* and substituting only *blocked* for *promised*
  still have 0 Readings. Gift's keyword export is recognized lexically but
  does not supply its noun use. May FixedKeyword export a noun as well as its
  existing verb? Its verb field can already express *promise* independently
  of the keyword stem; no new verb field is needed.
- **Bands with other:** witnesses 0; affinity-for controls 2 each. A trial
  `FixedKeyword`/Quality declaration still gives 0. The existing quality
  routes select a preposition-marked quality, a before-head quality, or a bound
  suffix; none selects a bare quality after the head. PT6 defers this
  Construction gap; the trial declaration is removed.
- **Banded:** both witnesses 0, paired-with controls 1. A trial band verb
  with the same active Object + with + Object frame still leaves both passive
  Postmodifier witnesses at 0. Pair's passive frame is supplied in core data,
  which PT1 forbids editing here. May KeywordVerbGrammar declare a Passive
  frame? Its current frame variants are Predicate, Auxiliary and ProVerb;
  the participial-adjective row has no Complement frame field. The trial verb
  export is removed rather than shipping an unused active-only route.
- **Empower:** owner confirmed as the keyword action Empower Jace
  [CR#701.71a]. Both ticket/enters witnesses are 0, amass-Zombies controls 1.
  The narrower same-complement control *Amass Jace 3.* is also 0 (no
  vocabulary gaps): the bare singular subtype Complement needs a declared
  class licence, not a Jace member-list exception. PT3 fails for the existing
  Object Noun Phrase + Amount frame; defer that construction/class decision.
- **Already completed on parent:** saddle exports its verb; *saddles* 1 and
  *becomes saddled* 1. Leave that definition untouched.

## Landing record

1. **Census delta (`ztvzruvp`):** bounded manifest `target/english-v3/
   english-v3-plugin-term-definitions/manifest.json`, 1,076 faces: 785 matching
   `modif|part(y|ies)|multikick|expend|descend|poison|wicked|monster|young hero|cursed|royal|sorcerer|virtuous|bands with other|empower|convok|devour|saddl|evolv|banded|day|night|gift|promis`
   anywhere in supported analysed text, unioned with 300 covered controls
   (seed 20261008; all 300 covered on the claim parent). Covered 748 → 849;
   total Readings 20,427 → 21,307. Gains 101; lost/decreases/increases/
   digest-only/source changes 0. Lost faces: none. No unintended-analysis
   gains found. `english-v3 diff --lines` evidence is `diff.json` in that
   directory; no bespoke census comparison was used.
2. **Witnesses (`ztvzruvp`, after/control):** modified attributive 2/2,
   predicative 1/1; party 0/3; full party 0/1; both multikicker units 1/1;
   expend 1/1 (both attested Amount-frame controls 1/1); descended 1/2
   (including the Enterprising Scallywag unit); poisoned 1/1 (Corrupted
   Resolve and Viridian Betrayers); night 0/2, day 0/1; gift 0/2 (retaining
   *the gift* in the two passive controls gives 0/0); Wicked, Monster,
   Young Hero, Cursed, Royal and Virtuous Role 0/5 each; Sorcerer Role 9/5
   already read and was dropped; both convoked fragments 1/1; both devoured
   units 1/1; saddles 1/1 and becomes saddled 1 already read; simple evolves
   1/1, Watchful Radstag 2/2 and Renegade Krasis 32/32 (independent
   ambiguity retained under W9-A); both bands-with-other units 0/2; both
   banded units 0/1; empower/enters 0/1, same-subtype control 0/0. All
   witness/control probes preceded implementation; after probes and trial
   declaration evidence are retained in the same target directory.
3. **Spot checks (`ztvzruvp`):** Envoy of the Ancestors — adjective
   Premodifier; Skyward Spider — predicative adjective; Corrupted Resolve
   and Viridian Betrayers — poisoned predicative adjective; Wolfbriar
   Elemental — CostKeyword; Junkblade Bruiser — expend with Amount
   Complement; Enterprising Scallywag — descended past intransitive
   Predicate; Zephyr Singer — convoked Predicate in a Subject-gap Relative Clause;
   Hellkite Hatchling — devoured transitive Predicate; Watchful Radstag —
   evolves intransitive Predicate. Each is newly covered and uses the
   plugin-owned Lexeme; `spot-checks.json` records the inspected Readings.
4. **Deviations and STOPs:** eight terms completed with existing rows; no
   Construction, loader, core vocabulary, or existing semantic body changed.
   The deferred questions above collect party/full party, Role names,
   day/night, gift/promise, bands with other, banded and empower. Trial
   bands-with-other and active-only band declarations were removed after
   probes proved the gaps. Semantics debt: modified, poisoned, descend,
   expend and multikicker are new bodyless declarations (helper `body: ()`
   is the existing bodyless representation). No glossary gap. A concurrent
   docs-only claim-parent update produced divergent successors; harmony
   recovered the test-bearing successor, preserving the newer parent and
   ticket notes. Three integration tests in one module, registered once in
   suite_6, assert ownership, grammatical role and independent roundtrips.
   A first test build failed on an ambiguous conversion in a new assertion;
   it was corrected without changing the assertion's subject or strength.
   Existing exhaustive inventory tests were extended for multikicker's Cost
   signature and spelling (no assertions removed):
   `builtin_v2_keyword_ability_nursery_preserves_declared_spelling_and_grammar`,
   `every_keyword_declaration_pins_its_template`, and
   `every_keyword_template_is_consistent_with_its_params_and_grammar`. Their
   first two full-core runs exposed the missing expected inventory rows;
   verification was rerun after extending those rows.
5. **Tests:** all final commands exit 0:
   - `cargo test -p deckmaste_english_v3` — 964 passed.
   - `cargo test -p deckmaste_lexical_source` — 86 passed.
   - `cargo test -p deckmaste_construction_core` — 474 passed.
   - `cargo test -p deckmaste_semantics_v2` — 105 passed.
   - `cargo test -p xtask english_v3` — 56 passed.
   - `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` — passed.
   - `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` — passed.
   - `cargo clippy -p deckmaste_construction_core --tests -- -D warnings` — passed.
   - `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings` — passed.
   - `cargo clippy -p xtask --tests -- -D warnings` — passed.
   - `cargo xtask lean check` — 240/240 cards checked, 2 plugins.
   - `cargo xtask cite bless` — registered only [CR#701.71a]; its complete
     rule text was read against the keyword-action ownership claim.
   - `cargo xtask cite check --list-noncompliant` — 0 noncompliant.
   - `cargo xtask cite check` — 0 stale.
   - `jj --no-pager diff --git | cargo xtask cite audit --diff` — two sites
     audited and read against the claim.
   English clippy initially found a redundant clone of a Copy lexeme ID in
   the new helper; removing it preserved the value comparison and assertions.
   Final logs and command outcomes are in `verification.json` and adjacent
   logs under the evidence directory. Assurance counts: restored 0 /
   re-spelled 0 / ignored 0 / added 3 / removed 0; no removals to justify.
6. **Timing (`ztvzruvp`):** final bounded census 45.17 s wall time under
   `/usr/bin/time -v`, six workers, `--samples-per-face 0`. No full-corpus
   run. The trial-data census was rerun after removing the two unusable
   declarations; final results above are for the retained data only.
