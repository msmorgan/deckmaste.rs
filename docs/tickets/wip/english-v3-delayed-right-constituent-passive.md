---
needs: []
---
# Read a delayed right constituent after coordinated passive participles: damage that would be dealt to and dealt by this creature

## Why

*dealt to X* and *dealt by X* each read; coordinated with one shared NP they
do not. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:and dealt` 21 sole-blocker
faces. `cargo xtask english-v3 grep -i 'dealt to and dealt by'` finds **23**
faces, all unread; **22** have that line as their only failing line. The
shared NP is *this creature* / *that creature* / *it* (11 or 12), *enchanted
creature* (4), *target creature you control* / *target permanent an opponent
controls* (4), *creatures you control* / *those creatures* / *the creature*
(3). A wider search for any Preposition + Coordinator + (participle) +
Preposition + NP finds the same 23 faces and no other pair.

## Goal

*dealt to and dealt by NP* reads as a Coordination of two passive participle
heads, each keeping its own selected Preposition, with the NP realized once
as their Shared Complement, with byte-exact realisation.

## Analysis

CGEL Ch. 15 §4.4, p. 1343: in delayed right constituent coordination "the
constituent which in basic coordination would appear as the rightmost element
of the first coordinate is held back until after the final coordinate", [21ii]
"*She knew of but never mentioned my other work.*"; each coordinate must end
"in a way that syntactically allows completion by the delayed element". P.
1344, [22i]: "*I'm interested in but rather apprehensive about their new
proposal*", where "the heads *interested* and *apprehensive* take different
prepositions". Those back a delayed NP completing two coordinates that end
in different Prepositions. Its use with a repeated participle, and the
grouping below, are project rulings.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`): the
selected-head sharing family, `SecondarySelectedHead` /
`SecondarySelectedHeadSeries` and `SelectedPrepositionHead` /
`SelectedPrepositionHeadSeries` (coordination schema rows with
`SharedFrameConcord` / `SharedHeadStatus`), which already realise a delayed
Complement after coordinated heads (*attacks or blocks a creature* reads 2).
The comment above `comparative_marker` records the project convention
"[governor marker][bare Complement] … following SelectedPrepositionHead and
FiniteSelectedHead for shared delayed Complements". Glossary: **Shared
Complement**. Today `secondary-selected-head` "dealt to" reads 0 and
`selected-preposition-head-series` "to and by" reads 0.

Orchestrator rulings 2026-10-10:

- DR1. Refine the existing selected-head sharing route: a past-participle
  head together with its selected Preposition (*dealt to*, *dealt by*) is a
  member whose missing right element is that Preposition's NP. Two such
  members coordinate and the NP is their Shared Complement. No new
  Construction; a new coordination-schema instance row or a second form on
  an existing head Construction is the permitted size.
- DR2. The licence reads the verb's declared frame (*deal*'s recipient *to*
  role and the passive's internalised *by* Complement) and each
  Preposition's `PrepositionFunctionLicence`. No test on *deal*, *to*, *by*
  or *damage*. Each member must independently license the delayed NP (the
  glossary's definition of Shared Complement).
- DR3. One analysis per string. The twins to avoid: (a) *dealt to* read as a
  complete passive with an Object Gap or elided Complement, coordinated with
  the complete *dealt by this creature* (the first coordinate must be
  incomplete, not gapped); (b) a Reading in which *and dealt by this
  creature* coordinates at Clause or relative-clause level; (c) a trailing
  *this turn* attaching inside the shared NP (it keeps the attachments it has
  in "… dealt to that creature this turn").
- DR4. Do not generalise to stranded Prepositions elsewhere: *dealt to* with
  nothing after it stays unread outside this Coordination. Do not add a
  check whose only job is to block other verbs: the frame decides.
- DR5. Newly covered faces are gains. No Reading-count increase on a covered
  face is expected: no covered face contains the pattern. An increase on a
  covered face containing *dealt to* or *dealt by* means DR3(a) leaked;
  report it with its shape before landing.

Orchestrator ruling DR6 (2026-10-11): first choice, no new declaration:
use the optional lexical field support landed in `ooumkosurztr` under SB8 of
`english-v3-supertype-premodified-bound-keyword`. Give `SelectedVerbHead` an
optional selected-Preposition field; its requirements apply only when present.
Its licence reads the verb frame and Preposition Function Licence. Preserve
existing counts, include every claim-parent `SelectedVerbHead` consumer in the
bounded manifest, and classify expected digest-only changes if Readings gain a
field. Second choice, only if that field cannot express the member (explain
why): one passive selected-head Construction feeding the existing coordination
and Shared Complement route, disclosed against the Construction budget.
DR2–DR5 remain in force.

## Witnesses

Each is the face's only failing line. Probed with the `ymqwzour` binary
(`--category sentence --readings 3`): witness 0; controls in parentheses are
the same sentence with one half only (3+ = enumeration capped). Re-probe on
the claim parent first.

- Fog Bank: "Prevent all combat damage that would be dealt to and dealt by
  this creature." (*to* only 3+; *by* only 3+)
- Gaseous Form: "Prevent all combat damage that would be dealt to and dealt
  by enchanted creature." (*to* only 3+)
- Maze of Ith: "{T}: Untap target attacking creature. Prevent all combat
  damage that would be dealt to and dealt by that creature this turn."
  (second sentence, *to* only 3+; *by* only 3+)
- Kiora, the Crashing Wave: "+1: Until your next turn, prevent all damage
  that would be dealt to and dealt by target permanent an opponent
  controls." (sentence without the loyalty cost, *to* only 3+)
- Statecraft: "Prevent all combat damage that would be dealt to and dealt by
  creatures you control."

Controls that must not change: "Prevent all combat damage that would be
dealt to this creature and dealt by this creature." 3+ (basic coordination);
"Whenever this creature attacks or blocks a creature, destroy that
creature." 2; "Whenever this creature blocks or becomes blocked by a
creature, destroy that creature." 3+; "Damage is dealt to this creature." 1.

## Claim-parent probes and DR1 STOP (2026-10-11)

Measured on `mzpnmlpo`, empty child of claim `nqlnoplr` (claim parent
`orwuzrvu`), with `cargo xtask english-v3 probe --category sentence
--readings 100 --json`. Every enumeration completed:

| Witness | Shared NP | To control | By control |
|---|---:|---:|---:|
| Fog Bank | 0 | 5 | 10 |
| Gaseous Form | 0 | 5 | 10 |
| Maze of Ith (second sentence) | 0 | 11 | 28 |
| Kiora, the Crashing Wave (without loyalty cost) | 0 | 4 | 9 |
| Statecraft | 0 | 12 | 22 |

Other controls: basic coordination 14; attacks or blocks 2; blocks or
becomes blocked 3; simple passive 1. Standalone *dealt to*, probed as
`secondary-verb-phrase`, has 0. No witness was already covered.

Bounded BEFORE: 857 faces, 812 covered, 81,312 Readings; 54.84 seconds,
6 workers. Manifest unions the Method's 420 surface faces, 300 covered
controls selected with seed 20261008 (user direction supersedes the Method's
seed), and 144 selected-head consumers identified by Construction in
`baseline-wave8.json`. Evidence is under
`target/english-v3/english-v3-delayed-right-constituent-passive/`.

STOP requiring an amendment to DR1: `SelectedVerbHead` stores only its verb
Word; the proposed incomplete passive member needs both that Word and its
Preposition Word. The v3 compiler requires every form to retain the same named
fields and cardinalities (`crates/deckmaste_construction_v3_core/src/ir.rs`,
form validation); schema instances cannot override schema-owned forms. A new
coordination-schema row alone does not create an incomplete member. Proposed
resolution: permit a passive selected-head declaration retaining both Words,
using the existing Coordination schemas and Shared Complement route. Resolved by DR6's fallback (2026-10-11): SB8's optional lexical support
extracts only direct constant requirements from optional fields
(`optional_word_requirements` in `ir.rs`). A cross-field frame/marker table
call and feature export still fail `resolve_reference` with “optional/repeated
fields have no single feature value”. The member needs that cross-field licence
and a retained distinction between a selected recipient and an internalised
agent. One `PassiveSelectedHead` Construction supplies both, feeding
`SecondarySelectedHead`, the existing Coordination schemas and
`SharedObjectComplement`. No new Category, schema or lexicon entry; ordinary
selected-head Readings retain their fields.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bdealt (to|by)\b` (420 faces) plus 300 random covered
controls (`--random 300 --seed 20261010`), `--workers 6 --samples-per-face
0`, compared with `cargo xtask english-v3 diff` against the claim parent.
Zero lost; DR5 for increases. Full `cargo test -p deckmaste_english_v3` and
`cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`, once, at the
end: a rebuild after a grammar edit takes about 2–3 minutes and the six test
binaries about 75 seconds, so batch edits. Tests assert the roles (two
participle members each with its Preposition, the NP as Shared Complement of
both, the DR3 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Goblin Snowman (1 face): a second failing line.
- Delayed right constituents after other head pairs not attested in the
  corpus (*to and by* with one participle: 0 faces).

## Landing record

1. **Census delta**: bounded selector `(?i)\bdealt (to|by)\b` (420
   faces), 300 random covered controls (seed 20261008), and every
   `SelectedVerbHead` consumer selected by node in `baseline-wave8.json`
   (144 faces; the claim-parent BEFORE also contains those 144): union **857**.
   Manifest: `target/english-v3/english-v3-delayed-right-constituent-passive/manifest.json`.
   On `mzpnmlpo` → `knlqxlok`: covered **812 → 834**, total per-line Readings
   **81,312 → 81,537**; gained **22**, lost **none**, decreases **0**, increases
   **0**, digest-only **0**, issues **0**. Existing selected-head consumer class:
   144 faces, 0 changed lines. The diff's 43 changed lines belong to gained
   faces. The wider declared-marker coordination grep finds 23 faces, all in
   the manifest. Winter's Chill remains unread through another blocker.

2. **Witnesses** (`knlqxlok`, complete sentence probes; to/by control counts in
   parentheses): Fog Bank **2** (5/10); Gaseous Form **2** (5/10); Maze of Ith,
   second sentence **8** (11/28); Kiora, the Crashing Wave, without loyalty
   cost **2** (4/9); Statecraft **5** (12/22). All five were 0 on `mzpnmlpo`;
   none was dropped. W9-A preserves independent attachment ambiguity, while
   each Reading has one route for the new material. Other controls unchanged:
   basic coordination 14, attacks or blocks 2, blocks or becomes blocked 3,
   simple passive 1; standalone *dealt to* remains 0 as Secondary Verb Phrase.

3. **Spot checks** (`knlqxlok`, full-face census counts): Cephalid Illusionist (18); Deftblade Elite (9); Delirium (25); Dovin, Hand of Control (10); Ebony Horse (16); Elvish Scout (16); Energy Arc (8); Fog Bank (4); Foxfire (10); Goblin Snowman (9).
   Each inspected Reading has two `PassiveSelectedHead` members retaining
   separate selected Prepositions, joined by `Coordination` below one
   `SharedObjectComplement` NP, with byte-exact realization and no issues.
   Wrong-analysis gains: 0 observed.

4. **Deviations and STOPs** (`knlqxlok`): the DR1 STOP is resolved by DR6's
   fallback, for the optional-field limitation recorded above. Exactly one
   Construction added, `PassiveSelectedHead`; no Category, schema or lexicon
   entry added or retired. `FrameUse` distinguishes recipient and internalised
   agent completion; existing concord/completion policies preserve ordinary
   signatures. No superseded route, removed Reading or glossary gap.
   Goblin Snowman's gain is incidental: its other line already reads without
   this Construction; no repair beyond the ticket's surface was made. Initial
   refresh brought the semantics-v2 counter-annihilation landing, including
   `plugins_v2/` changes, so the standing rule adds the full xtask, semantics,
   construction-core and Lean gates.

5. **Tests** (`knlqxlok`): `cargo test -p deckmaste_english_v3` passes (1,066
   tests); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passes.
   The first full suite found an overstrict new expectation; the full suite
   was rerun after re-spelling
   `delayed_right_constituent_passive::witnesses_keep_two_incomplete_members_and_one_shared_np`:
   its expected value changed from one fully postmodified NP surface per
   witness to the set of NP surfaces in the corresponding selected-*to*
   control, preserving Kiora/Statecraft's higher relative attachments under
   W9-A, while retaining both member-frame comparisons and the NP-internal
   attachment assertion. Restored 0; re-spelled 0 existing / 1 new; ignored 0;
   added 4; removed 0 (no removals to justify). Registered once in `suite_5`.
   Mandatory gates after the plugin refresh also pass: `cargo build -p xtask`,
   `cargo test -p xtask` (full), `cargo test -p deckmaste_semantics_v2`,
   `cargo test -p deckmaste_construction_core`, and `cargo xtask lean check`.
   Citation checks pass: `cargo xtask cite check --list-noncompliant`,
   `cargo xtask cite check`, and
   `jj --no-pager diff --git | cargo xtask cite audit --diff`; no new CR sites.

6. **Timing**: the bounded AFTER census on `knlqxlok` took **86.36 seconds**
   wall time under `/usr/bin/time -v`, with **6 workers** and
   `--samples-per-face 0`. No full-corpus census was run.
