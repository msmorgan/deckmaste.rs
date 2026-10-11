---
needs: []
---
# Read *at random* between an Object and the verb's source Complement: Return a creature card at random from your graveyard to your hand.

## Why

*at random* reads at the end of the verb phrase and before a free *from*
phrase ("Discard a card at random." 1; "Target opponent discards a card at
random from their hand." 1; "Return a creature card from your graveyard to
your hand at random." 1). It does not read where the verb selects what
follows: "Return a creature card at random from your graveyard to your
hand." 0, "Return two cards at random from your graveyard to your hand." 0,
"Put a card at random from your graveyard into your hand." 0, "Return a card
from your graveyard at random to your hand." 0, beside "Return a creature
card from your graveyard to your hand." 1. Blocker report at trunk `ymqwzour`
(`target/english-v3/blockers-wave8.txt`): key `at:random from` 11, part of
failing token `from` 30. `cargo xtask english-v3 grep -i '\bat random
from\b'` finds **41** faces: 26 covered (*reveals / exiles / chooses a card
at random from* …) and **15** unread, **12** with that line as their only
failing line. 13 of the 15 are *return* NP *at random from your graveyard to
your hand / to the battlefield*.

## Goal

The manner phrase *at random* reads between the Object of *return* and its
selected *from* … *to* … Complements, with byte-exact realisation and with
the Object, source and goal still filling the verb's frame.

## Analysis

No CGEL claim is relied on for the position; that a manner Adjunct may stand
between an Object and the verb's PP Complements in this register is a
project ruling, measured on the corpus (13 faces for *return*).

Existing routes: `core-verb:Return`'s frame
(`crates/deckmaste_lexical_source/lexicon/verbs.ron`:
`Argument(Object: SourceFreeObject)`, `Role("SourceComplement")`,
`Lex("Preposition", "To")`, `ObjectNounPhrase`, …), whose slots are adjacent;
the selected-complement tails `ObjectPrepositionTail`, `ObjectLocativeTail`
and `ObjectLocativeMannerTail`
(`crates/deckmaste_english_v3/src/declarations.rs`), the last of which
already places a manner PP after a locative Complement;
`InterveningPrepositionAdjunct` (`[" ", head: PrepositionPhrase]`, `require
head.InterveningAdjunctLicence = Yes`, `licence_adjunct(…)`), the optional
Adjunct that `SelectedGapPrepositionPredicate` admits between a verb and its
marker. *at random* is `vocab:Preposition/At` + `vocab:Adjective/Random`
(both declare `AdjectiveComplementClass: Manner`); only
`vocab:Preposition/As` declares `InterveningAdjunctLicence: Yes` today.

Orchestrator rulings 2026-10-10:

- AR1. Refine the selected-frame realisation, no new Construction: the
  optional `InterveningPrepositionAdjunct` is admitted after the Object slot
  and before a source Complement, as it is admitted after the verb in
  `SelectedGapPrepositionPredicate`.
- AR2. The licence is declared: the manner PP carries
  `InterveningAdjunctLicence` (declared on the manner Complement class of
  *at* / on *random*, whichever the PP's export reads), and the position is
  open only in frames whose next slot is a source Complement, read from the
  frame's declared slot role. No verb, noun or preposition is named.
- AR3. One analysis per string. The 26 covered *at random from* faces keep
  exactly their Readings: where *from their hand* is a free Adjunct or an NP
  postmodifier nothing changes, and *reveals a card at random from their
  hand* must not gain a Reading through the new position. *at random* is not
  a postmodifier of the Object NP (*a card at random* is not an NP): do not
  reach the witnesses by licensing that.
- AR4. Expected shape of increases: none. If `as`-phrases (the other
  declarer of the licence) start reading in the new position on covered
  faces, report them with their shape before landing.
- AR5. Newly covered faces are gains. *put* NP *at random from* … *into* …
  is unattested (0 faces); it may read as a consequence and is not blocked.

## Witnesses

Each is the only failing line of its face in `blockers-wave8.json` (census
`ymqwzour`, 0 Readings). Fragment probes ran 2026-10-10 on the `ymqwzour`
binary (`--readings 60`); trunk then moved to `pxmykwnl` and the prebuilt
probe no longer loads plugin data, so (census) controls give the covered
line's `reading_count`. Re-probe on the claim parent first.

- Surreal Memoir: "Return an instant card at random from your graveyard to
  your hand."
- Make a Wish: "Return two cards at random from your graveyard to your hand."
- Ghoulraiser: "When this creature enters, return a Zombie card at random
  from your graveyard to your hand."
- Haunted Fengraf: "{3}, {T}, Sacrifice this land: Return a creature card at
  random from your graveyard to your hand."
- Moldgraf Monstrosity: "When this creature dies, exile it, then return two
  creature cards at random from your graveyard to the battlefield."

Claim-parent probes (2026-10-11, `zkluyryr`; empty working-copy snapshot
`owouoxzv`): all five witnesses have 0 Readings. Their controls with *at random*
removed have 1 / 1 / 1 / 1 / 3 Readings, respectively. The six controls below
have 1 / 1 / 1 / 3 / 6 / 1 Readings. Enumeration completed without issues for
every probe; none of the witnesses was already covered. Evidence:
`target/english-v3/english-v3-at-random-before-source/before-probes.json`.

Controls that must not change: "Return a creature card from your graveyard to
your hand." 1; "Discard a card at random." 1; "Target opponent discards a
card at random from their hand." 1; Merfolk Spy "Whenever this creature deals
combat damage to a player, that player reveals a card at random from their
hand." 3 (census); Fall "Target player reveals two cards at random from
their hand, then discards each nonland card revealed this way." 6 (census);
Ogre Shaman "{2}, Discard a card at random: This creature deals 2 damage to
any target." 1 (census).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `(?i)\bat random\b|\breturns? [^.]* from [^.]* to\b` (184
faces contain *at random*; record the total) plus 300 random covered
controls, `--workers 6 --samples-per-face 0`, compared with `cargo xtask
english-v3 diff` against the claim parent. Zero lost; AR4 for increases. Full
`cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
`deckmaste_lexical_source` and `deckmaste_english_v3`, once, at the end; a
rebuild after a grammar edit takes about 2–3 minutes, the six test binaries
about 75 seconds; lexicon data loads at runtime. Tests assert the role (*at
random* as an Adjunct of the *return* predicate; Object, source and goal in
their frame slots; the AR3 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- Crabomination (*the top card of their library, a card at random from their
  graveyard, and a card at random from their hand*: Object coordination).
- The Deck of Many Things (table rows).
- Kheru Lich Lord (also *anywhere else*, `english-v3-anywhere-else-goal`);
  Fossil Find (*then reorder your graveyard as you choose*); Tomb Tyrant
  (*at least three* … restriction) if those tails block.

## Landing record

1. **Census delta** (`owouoxzv`, claim-parent behavior `zkluyryr`): bounded
   covered faces **3,250 → 3,262**, total Readings **60,332 → 60,373**;
   **12 gains, 0 lost, 0 decreases, 0 increases, 0 digest-only**, 0 issues.
   Lost faces: none. Wrong-analysis gains: none. Digest-only classes and their
   line counts: none (0); the line diff reports 19 lines on gained faces.
   The Method regex matches 1,318 faces; the selector was enlarged to every
   surface occurrence of the affected source-frame verbs:
   `(?i)\bat random\b|\b(?:return(?:s|ed|ing)?|remov(?:e|es|ed|ing))\b`
   (3,294 faces, including all 184 *at random* faces). All 143 covered
   `AdjectiveComplementPreposition` users in the supplied parent census are
   included. Union with 300 random covered controls, selected by the official
   `diff --manifest --random 300 --seed 20261008`, gives **3,567 faces** after
   deduplication. BEFORE was freshly run on the claim parent with this manifest.
   Evidence: `target/english-v3/english-v3-at-random-before-source/{manifest,before,after,diff}.json`;
   comparison used `cargo xtask english-v3 diff ... --lines --json ...`.
2. **Witnesses** (`owouoxzv`): Surreal Memoir **1** (whole face 2), Make a
   Wish **1** (face 1), Ghoulraiser **1** (face 1), Haunted Fengraf **1**
   (face 2), Moldgraf Monstrosity **3** (face 4). Each fragment was 0 on
   `zkluyryr`; its paired control is respectively 1 / 1 / 1 / 1 / 3 before
   and after. The six original controls retain 1 / 1 / 1 / 3 / 6 / 1.
   Every probe enumerated completely without issues. W9-A's independent
   ambiguity is preserved; the new material has one construction route.
3. **Spot checks** (`owouoxzv`): Charmbreaker Devils (2 / paired 2),
   Exalted Flamer of Tzeentch (2 / 2), Fossil Find (4 / 4), Ghoulraiser
   (1 / 1), Haunted Fengraf (1 / 1), Make a Wish (1 / 1), Moldgraf
   Monstrosity (3 / 3), Reap the Past (1 / 1), Surreal Memoir (1 / 1),
   and Tomb Tyrant (5 / 5), using the affected line's counts. Each uses
   `SelectedPredicate`: Object + `FrameValue::Adjunct` containing
   `InterveningPrepositionAdjunct` / `AdjectiveComplementPreposition` +
   `SourceComplement` + selected `To` + goal argument. The other two gains,
   Viconia, Drow Apostate (4 / 4) and Woodland Sleuth (2 / 2), were also
   checked. No gained line uses an unintended analysis; evidence is `spots.json`
   and its per-line probes under the same target directory.
4. **Deviations and STOPs** (`owouoxzv`): no new or retired Construction,
   lexical Verb Frame, or lexeme; no superseded route to retire. AR1–AR2 need
   compiler support because the former frame values represent only selected
   arguments/markers: added the generic `frame_adjunct` slot-category
   declaration and `FrameValue::Adjunct`, admitted only after an Object and
   before the declared Complement role. `random` declares the existing
   licence; the existing adjective-complement PP exports it. Two compiler
   tests supplement the seven English regressions. Slot validation and
   category projection were extracted into helpers to meet clippy's function
   line limit, without changing emitted grammar. No STOPs or glossary gaps.
5. **Tests** (`owouoxzv`): `cargo test -p deckmaste_english_v3` passed
   (1,095 passed, 1 inherited ignore);
   `cargo test -p deckmaste_lexical_source` passed (44);
   `cargo test -p deckmaste_construction_v3_core` passed (41).
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`, and
   `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings`
   all passed. Earlier runs caught three new English fixtures with lowercase
   controls for initially capitalized witnesses, one new compiler fixture
   missing the required Construction, and the two compiler function-length
   lint failures; these were corrected with the value assertions retained.
   Restored **0**, re-spelled **0**, newly ignored **0**, added **9**
   (7 English + 2 compiler), removed **0** (no removals to justify).
   The unchanged inherited ignore is
   `mana_symbol_nominal::cost_notation_is_the_subject_of_the_licensed_locative_small_clause`,
   blocked by `english-v3-mana-symbol-verbless-subject` MS4's governed Mana
   Phrase Subject route. Citation checks: 0 noncompliant, 0 stale;
   no new CR citations to bless.
6. **Timing** (`owouoxzv`): bounded AFTER census **94.26 seconds**, **6
   workers**, measured with `/usr/bin/time -v`; `--samples-per-face 0`.
