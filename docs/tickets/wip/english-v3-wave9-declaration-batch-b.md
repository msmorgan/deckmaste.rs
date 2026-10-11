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

To be written by the claimant (six items, `docs/tickets/README.md`).
