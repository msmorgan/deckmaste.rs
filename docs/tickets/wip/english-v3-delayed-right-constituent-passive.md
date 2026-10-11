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

To be written by the claimant: the six items of `docs/tickets/README.md`.
