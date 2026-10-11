---
needs: []
---
# Declare eight plugin-owned terms: modified, poisoned, multikicker, expend, descend, convoke, devour and evolve

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

Owner acceptance (2026-10-10): the eight completed terms are accepted (101
feature gains, no covered-face changes). Move the seven deferred questions to a
new planned ticket, verify against `kruuppqw`, then integrate and drop the
workspace. This supersedes the review STOP for this landing; it does not waive
post-refresh verification.

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

## Landing record

1. **Census delta (`yzlwrssy`, implementation `ztvzruvp`):** bounded identity
   manifest `target/english-v3/english-v3-plugin-term-definitions/manifest.json`,
   1,076 faces: 785 matching
   `modif|part(y|ies)|multikick|expend|descend|poison|wicked|monster|young hero|cursed|royal|sorcerer|virtuous|bands with other|empower|convok|devour|saddl|evolv|banded|day|night|gift|promis`
   anywhere in supported analysed text, unioned with 300 covered controls
   (seed 20261008; all 300 covered on the claim parent). Covered 748 → 857;
   total Readings 20,427 → 21,387. Official `english-v3 diff --lines`:
   109 gains, 0 lost/decreases/increases/source changes, 4 digest-only,
   963 unchanged (`third-after.json`, `third-diff.json`). The accepted
   ticket contributes 101 gains, 0 changes on covered faces. The first incoming
   diff (`accepted-incoming-diff.json`, against `second-refreshed-after.json`)
   attributes three extra gains to trunk: Bjorna, Nightfall Alchemist and
   Peri Brown (`zvoyutom`, phrasal keywords); Wicked Slumber (`qnvlmzuu`,
   determinative-headed partitive). Three unchanged-count digests come from
   `pusrqwml` wrapping Cardinal partitive heads as quantitative Determiners:
   Heroes' Hangout 12, Interdisciplinary Mascot 5, Party Thrasher 82.
   `incoming-gain-probes.json` confirms the intended keyword/partitive shapes.
   Lost faces: none. No unintended-analysis gains found. No bespoke comparison.
   Integration refresh imported `tznkrzpw` (quantified energy) and `ropwqyov`
   (scalar difference). `integration-incoming-diff.json` attributes two more
   gains to scalar difference: Ancient Cellarspawn 8 and Vraska, Betrayal's
   Sting 11. It attributes one unchanged-count digest to quantified energy:
   Janjeet Sentry 2: its Symbol Words now carry declared mass countability
   (`Some(false)`); the repeated-symbol Complement route is preserved, as
   recorded by the energy landing. Five extra gains require incoming trunk
   changes. The later `third-incoming-diff.json` adds Caprichrome 3, Famished
   Worldsire 38 and Feasting Hobbit 3 through devour-quality (`vuyokrvx`,
   `ulkrnzyt`); `third-incoming-probes.json` records their intended shapes.
   Eight extra gains require incoming trunk changes, and all four digest
   changes come from trunk; the ticket's accepted delta remains 101 gains
   with no changes to covered faces. `integration-incoming-probes.json` records the intended shapes.
   The latest refresh imported starting-with (`wxxvsntt`) and devour-quality
   (`vuyokrvx`, `ulkrnzyt` and their supporting stack). The bounded census,
   full suites, clippy and Lean check all passed on `yzlwrssy`.
2. **Witnesses (`ztvzruvp`, after/control):** every implemented witness was
   0 on the claim parent; controls had the counts below before the edit.
   Modified attributive 2/2,
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
4. **Deviations and STOPs:** eight terms completed with existing rows;
   no Construction, loader, core vocabulary or existing semantic body changed.
   Deferred questions: [english-v3-plugin-term-schema-questions](../planned/english-v3-plugin-term-schema-questions.md).
   Removed trial bands-with-other and active-only band exports after probes
   proved the declaration/Construction gaps; no previously admitted route was
   retired. PT-Q1 authorizes all five new bodiless definitions as semantics debt:
   modified and poisoned need predicate evaluation; descend and expend need
   event recognition and rules implementations. These four helper families
   have no facts-row gate; generated Lean lists each as bodyless/not derived.
   Multikicker has its existing Cost-signature facts row in
   `crates/xtask/src/facts.rs`; its engine body remains debt despite its
   generated syntax wrapper. No invented facts rows or bodies. No glossary gap.
   Three tests assert ownership, role and independent roundtrips, registered
   once in suite_6; three existing exhaustive inventory tests were extended
   for Multikicker. Initial assertion conversion and Copy-clone lint defects
   were fixed without weakening assertions. Full xtask found a stale
   Multikicker row-without-stub exemption and stale generated Lean: removed
   only that obsolete exemption and regenerated Declared.lean. No failing test
   was removed or ignored. Refresh test-registration conflicts preserved both
   sides exactly once; documentation divergence was repaired by retaining
   both sets of edits (the discarded successor had the identical final tree).
   Latest refresh preflight caught identical inventory-count edits merging
   as one increment: Multikicker plus devourQuality require 110 parameterized
   entries and 209 ability declarations. Corrected both exhaustive counts
   while retaining both declaration entries and every assertion.
   The upstream six-row STOP from `zvoyutom` was resolved by `kruuppqw`'s
   owner-approved semantics-debt exemptions, regenerated Lean and corrected
   Waterbend assertion. Post-refresh verification below supersedes the STOP.
5. **Tests (`yzlwrssy` after the latest refresh):** all commands exit 0:
   - `cargo build -p xtask` — passed.
   - `cargo test -p xtask` — full suite, 425 passed, 1 inherited ignore.
   - `cargo test -p deckmaste_english_v3` — 1,041 passed.
   - `cargo test -p deckmaste_lexical_source` — 88 passed.
   - `cargo test -p deckmaste_construction_core` — 474 passed.
   - `cargo test -p deckmaste_semantics_v2` — 106 passed.
   - `cargo clippy -p xtask --tests -- -D warnings` — passed.
   - `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` — passed.
   - `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` — passed.
   - `cargo clippy -p deckmaste_construction_core --tests -- -D warnings` — passed.
   - `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings` — passed.
   - `cargo xtask lean check` — 243 cards / 2 plugins passed.
   - `cargo xtask cite check --list-noncompliant` — 0 noncompliant.
   - `cargo xtask cite check` — 0 stale.
   - `jj --no-pager diff --git | cargo xtask cite audit --diff` — one moved
     citation audited; [CR#701.71a] read in full against the ownership claim.
   Earlier `cargo xtask cite bless` registered only that rule. Exact commands,
   outcomes and timings are in `third-verification.json` and adjacent
   `third-check-*.log`; prior rounds remain in `accepted-verification.json`
   and `integration-verification.json` in that directory. Assurance: restored 0 /
   re-spelled 0 / introduced ignores 0 / added 3 / removed 0. No removals to
   justify. The inherited ignored xtask test
   `macros::templates::tests::macro_schema_census_count_matches_21` retains
   its existing reason: “cross-checks the live corpus against the census;
   run on demand”.
6. **Timing (`yzlwrssy`):** final bounded census 86.42 s wall under
   `/usr/bin/time -v`, six workers, `--samples-per-face 0`, identical manifest.
   No full-corpus census was run. Earlier bounded checks remain in the evidence
   directory.
