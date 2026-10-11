---
needs: []
---
# Declare four missing forms and licences: creatures controlled by the same player / from outside the game / becomes attached to a creature / you may instead create …

A data ticket: four independent declarations that existing routes are
expected to read. No new Construction. Each item lands or is returned on its
own; none blocks the others.

## Why

Blocker report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.txt`),
keys `at:controlled by` 11, `at:put it` 9, `at:attached to` 12, `at:may
instead` 9. Probes show each fails beside a sister that reads, the difference
being one declaration. `cargo xtask english-v3 grep -i` (faces; all unread;
"sole" = that line is the face's only failing line):

| Item | Pattern | Faces | Sole | Reads today |
|---|---|---|---|---|
| A *controlled by* | `\bcontrolled by\b` | 30 | 27 | "That creature is owned by that player." 3; NP "creatures blocked by that creature" 3 |
| B *from outside the game* | `\bfrom outside the game\b` | 18 | 16 | NP "a card you own outside the game" 2; NP "a card from among them" 1 |
| C *become / remain attached to* | `\b(becomes?\|remains?\|already) attached to\b` | 13 | 13 | "As long as this Equipment is attached to a creature, draw a card." 1; "This Equipment becomes blocked by a creature." 2 |
| D *may instead* | `\bmay instead\b` | 9 | 9 | "If you would draw a card, you may draw two cards instead." 2; "… instead you may draw two cards." 1 |

Probed 0: NP "planeswalkers controlled by that opponent", "That creature is
controlled by that player.", "If it's controlled by the chosen player, draw a
card."; NP "a sorcery card from outside the game", "You may reveal a card
from outside the game."; "Whenever this Equipment becomes attached to a
creature, draw a card.", "This Equipment remains attached to it."; "If you
would draw a card, you may instead draw two cards."

## Goal

The four shapes read through the routes their sisters use, with byte-exact
realisation and no change to any covered face.

## Analysis

No CGEL claim is added. C rests on what
`docs/tickets/done/english-v3-become-adjectival-passive.md` already cites for
adjectival passives with a Complement under *become*; the rest are
declarations (project rulings).

Orchestrator rulings 2026-10-10:

- DB1 (A). The lexical analysis of *controlled* returns Preterite readings
  only, where *owned* also returns a PastParticiple (`core-verb:Control` and
  `core-verb:Own` have the same entry shape in `lexicon/verbs.ron`, one
  `Predicate([ObjectNounPhrase])` frame each, and the same features in
  `lexicon/core.ron`). Find the declaration that withholds the form (an
  override, an availability table or the pruning of a once-unattested form)
  and declare the past participle. The passive and the past-participial
  postmodifier with a *by* phrase then follow from the routes that read
  *owned by* / *blocked by* (`PassiveComplement`,
  `ParticipialPostmodifiedNominal`). Do not add a participial Adjective
  *controlled*.
- DB2 (B). *from* takes a PP Complement through the route of
  `docs/tickets/done/english-v3-from-preposition-phrase-complement.md`
  (*from among them*). `vocab:Preposition/Among` declares
  `SourceComplementUse: Yes` and a `Compound` function;
  `vocab:Preposition/Outside` (`Adjunct|Modifier|PredicativeComplement|
  PreposedAdjunct`) does not. Declare on *outside* what that route reads, no
  more. *outside the game* keeps its postmodifier Reading (Spawnsire of
  Ulamog, 50 Readings in the census).
- DB3 (C). *attached* under *become* / *remain* / after *already* is the
  adjectival passive with a *to* Complement. Add `lexeme:keyword_action/attach`
  to the `participial_adjective_classes` of `lexicon/core.ron` (as
  `core-verb:Block` and `lexeme:keyword_action/tap` are) with the marked
  *to* frame, mirroring *blocked*'s *by* frame. The verb stays owned by
  `plugins_v2/builtin/macros/keyword_actions/attach.ron`. Expected shape of
  increases: *is attached to* gains the adjectival Reading beside the verbal
  passive, exactly as *is blocked by* has both today (the become-passive
  landing's two-Reading pair); report the faces and counts. Any other
  increase is a defect.
- DB4 (D). *instead* (`vocab:Preposition/Instead`, an intransitive
  Preposition with `ConnectiveAdjunctUse: Yes`) takes the central position
  between a modal and its verb through the route of
  `docs/tickets/done/english-v3-central-adverb-position.md`
  (`CentralAdverbComplement`, read from `CentralAdjunctLicence` /
  `CentralAdjunctPosition`). Declare that licence on *instead*. If the route
  accepts only the Adverb category, the fix is one form that reads the same
  declared licence from an intransitive Preposition; record it under
  Deviations. A homograph Adverb *instead* is not minted.
- DB5. One analysis per string, per item: *owned by* / *blocked by* keep
  their Readings; *you may draw two cards instead* and *instead you may* keep
  theirs (central *instead* is a third position, not a re-analysis of the
  other two); *from among them* keeps its Readings.
- DB6. If an item's declaration alone does not read its witnesses, record the
  probe evidence and return that item (mint or route it); do not add a
  Construction under this ticket. Newly covered faces are gains.

Owner amendments (2026-10-10, task instructions): DB1 applies to the natural
class of regular transitive verbs, including every other core verb with the
same missing form. DB3's core member-list recipe is superseded: the plugin
must own the participial-Adjective export. If that export cannot carry the
marked *to* frame, return C independently. Standing ruling W9-A preserves
independent ambiguity and requires one route for the new construction.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- A, Barrin's Spite: "Choose two target creatures controlled by the same
  player. Their controller chooses and sacrifices one of them. Return the
  other to its owner's hand."
- A, Run Away Together: "Choose two target creatures controlled by different
  players. Return those creatures to their owners' hands."
- B, Granted: "You may reveal a noncreature card you own from outside the
  game and put it into your hand."
- B, Living Wish: "You may reveal a creature or land card you own from
  outside the game and put it into your hand. Exile Living Wish."
- C, Enormous Energy Blade: "Whenever this Equipment becomes attached to a
  creature, tap that creature."
- C, Bramble Elemental: "Whenever an Aura becomes attached to this creature,
  create two 1/1 green Saproling creature tokens."
- D, Parallel Thoughts: "If you would draw a card, you may instead put the
  top card of the pile you exiled into your hand."
- D, Retriever Phoenix: "As long as this card is in your graveyard, if you
  would learn, you may instead return this card to the battlefield."

Controls that must not change (except DB3's disclosed pair): Lydia Frye
"Lydia Frye can't be blocked by creatures with power 3 or greater." 21
(census); Strong Back "Enchanted creature gets +2/+2 for each Aura and
Equipment attached to it." 3 (census); Silvan Rally "Mill four cards, then
put up to two land cards from among them into your hand." 4 (census);
Pursuit of Knowledge "If you would draw a card, you may put a study counter
on this enchantment instead." 2 (census); Likeness of the Seeker "Whenever
this creature becomes blocked, untap up to three lands you control." 1
(census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    (?i)\bcontrolled\b|\b(owned|blocked) by\b|\boutside\b|\bfrom among\b|\battached\b|\binstead\b

(1,993 faces; DB3 and DB4 touch shared words, so every *attached* and
*instead* face is in) plus 300 random covered controls, `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent.
Zero lost; increases only of DB3's shape. `cargo test -p` and `cargo clippy
-p … --tests -- -D warnings` for `deckmaste_lexical_source` and
`deckmaste_english_v3`, once, at the end; lexicon data loads at runtime (a
rebuild, needed only for DB4's fallback, takes about 2–3 minutes; the six
test binaries about 75 seconds). Tests assert the role per item (the past
participle of *control* in passive and postmodifier function; the PP
Complement of *from*; the adjectival Complement of *become* with its *to*
phrase; *instead* in central position), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- A: lines with another blocker (*the player who controls the fewest*,
  Balance and three others; *the next player in the chosen direction*; *the
  player to their left / right*).
- B: nothing further; the `at:put it` key was this, not coordination ("You
  may reveal a sorcery card you own from your hand and put it into your
  hand." 34).
- C: *for as long as this Equipment remains attached to it, that creature
  becomes a copy of* … if the copy clause blocks.
- D: *you may instead choose land or nonland* (Abundance).
- *if tribute wasn't paid* (11 faces, 11 sole): the keyword *tribute* has no
  noun export (`keyword_abilities/tribute.ron`); add it to
  `english-v3-plugin-term-definitions` section B.

## Landing record

1. **Census delta** (original `txkvmlxzpprs` claim-parent content →
   `mlnortmqxzrl` → refreshed `vtnyzwqyyzwu`): bounded manifest of 2,278
   faces; own change 1,836 → 1,879 covered, 135,314 → 140,643 Readings,
   43 gains. After refresh: 1,884 covered, 140,968 Readings, 48 net gains,
   2,230 unchanged. The five additional gains belong to trunk waterbend
   change `ulrnwvuw`: Avatar Kuruk, Foggy Swamp Vinebender, Secret of
   Bloodbending, Spirit Water Revival and Water Tribe Rallier (325 Readings).
   Both official `english-v3 diff --lines` comparisons report zero lost,
   decreases, increases and digest-only changes. Lost faces: none.
   Selector: the Method's complete surface pattern plus
   `\b(flipped|skipped)\b`, 1,994 matching faces, unioned with 300 random
   covered controls from `diff --manifest --random 300 --seed 20261008`
   (all 300 covered on the claim parent). Evidence:
   `target/english-v3/english-v3-wave9-declaration-batch-b/`:
   `manifest.json`, `before.json`, `after.json`, `after-refresh.json`,
   `diff.json`, `refresh-diff.json`, `final-diff.json`.
2. **Witnesses** (`txkvmlxzpprs` → `mlnortmqxzrl`, complete `probe` enumeration
   of the ticket texts in card context): Barrin's Spite 0 → 6 (owned control
   6); Run Away Together 0 → 6 (6); Granted 0 → 34 (from-hand control 34);
   Living Wish 0 → 91 (91); Enormous Energy Blade 0 → 0 (blocked control 2);
   Bramble Elemental 0 → 0 (2); Parallel Thoughts 0 → 5 (without-central-
   instead control 5); Retriever Phoenix 0 → 1 (1). The W9-A control counts
   are preserved. Lydia Frye 21, Strong Back 3, Silvan Rally 4, Pursuit of
   Knowledge 2, Likeness of the Seeker 1: all unchanged. These are witness
   fragment/document counts; the face census sums its separate Ability lines.
3. **Spot checks** (`mlnortmqxzrl`): Barrin's Spite, Run Away Together,
   Cannibalize and Retribution select the *controlled by* verbal past-
   participial Postmodifier; Granted, Living Wish and Burning Wish select
   *outside the game* as the PP Complement of *from*; Parallel Thoughts,
   Retriever Phoenix and Archmage Ascension select intransitive Preposition
   *instead* as a central Adjunct in the modal's Bare Complement. Each
   inspected Reading realizes byte-exactly, with no issue or unintended
   analysis found. `spot-0.json` … `spot-9.json` under the evidence directory.
4. **Deviations and STOPs** (`mlnortmqxzrl`): DB1's cause is an orthographic
   hole, not a removed form: defaults generate *controled*, *fliped*,
   *skiped*. The core inventory audit found exactly Control, Flip and Skip;
   their existing verb declarations now supply the correct participles.
   The retired default spellings attest zero supported faces (`english-v3
   grep`, `retired-spellings.txt`). No Adjective *controlled* is minted.
   DB2 adds only SourceComplementUse and Compound on *outside*. DB4 uses
   its authorized fallback: one additional form on CentralAdverbComplement
   reads an intransitive Preposition and the existing central licence; no
   Adverb homograph or new Construction. C's item-only STOP is resolved by
   returning it to `english-v3-plugin-participial-adjective-selected-frame`:
   the current export is FixedKeyword-only and cannot carry a selected
   frame. No plugin definition changed. No Construction/frame/test retired;
   no glossary gap. Seven role/roundtrip English tests and one lexical
   spelling test added.
5. **Tests** (`mlnortmqxzrl`, then refreshed `vtnyzwqyyzwu`):
   `cargo test -p deckmaste_english_v3` passed (1,002 before refresh,
   1,006 after); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed after fixing the new fixture's explicit-default style lint;
   `cargo test -p deckmaste_lexical_source` passed (87);
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   All four commands ran in full again after refresh, along with
   `cargo build -p xtask` (passed). Restored 0 / re-spelled 0 / ignored 0 /
   added 8 / removed 0. Citation checks: zero stale, zero noncompliant;
   piped diff audit selects zero citation sites. No new citations to bless.
   No Lean/plugin checks apply because this feature changed no plugin
   definition. Refresh incorporated waterbend's tests and plugin data;
   no change to this feature's selected Readings, and no trunk decreases.
6. **Timing** (`vtnyzwqyyzwu`): bounded post-refresh census under
   `/usr/bin/time -v`, **1m 33.61s**, six workers,
   `--samples-per-face 0`. No `--all` run.
