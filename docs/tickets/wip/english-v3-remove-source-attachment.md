---
needs: []
---
# Read *remove* without its Source, and *combat* as its Source: Add {U} for each storage counter removed this way. / Remove it from combat.

## Why

*remove* has one active frame, `Predicate([ObjectNounPhrase,
Role("SourceComplement")])` (`lexicon/verbs.ron`), and one passive frame with
the Source Complement (`"core-verb:Remove"` in `lexicon/core.ron`). So
"Remove a counter from this land." 1 and "A counter was removed from this
land." 1, but "Remove a counter." 0, "A counter was removed." 0 and NP "each
counter removed this way" 0 (NP "each creature sacrificed this way" 1).
Separately *from combat* reads nowhere: "Remove it from combat." 0, "Remove
it from the game." 1. Both were deferred from
`docs/tickets/done/english-v3-data-batch-2.md` (items 5 and 6, D1 follow-ups
in its landing record, item 4). Blocker report at trunk `uoqyuxtn`: keys
`at:way .` 38 and `at:combat .` 20.

`cargo xtask english-v3 grep`:

- `\bremoved this way\b`: **43** faces, all unread, **37** with that line as
  their only failing line.
- bare transitive (counted in `blockers-wave7.json`): *doesn't remove this
  Aura* 12 faces (11 sole); *remove those counters / them / all of them* 14
  (12 sole).
- `\bfrom combat\b`: **24** faces, all unread, **22** sole; all 24 are
  active *remove <NP> from combat* (`\bremoved from combat\b`: 0 faces).

## Goal

*remove* reads with an Object and no Source, active and bare passive, and
*from combat* reads as the Source Complement of *remove*, with byte-exact
realisation; "Remove a counter from this land."
keeps exactly one Reading.

## Analysis

CGEL Ch. 8 §4.3, p. 687: "Source complements generally have the form of PPs
headed by *from*, *away*, *off*, *out*", and "*He came home from school*"
(a bare noun after *from*). That backs the Source Complement function and a
bare nominal inside the *from* PP. Which attachments Oracle English keeps is
the project's ruling.

Diagnoses carried over from the data-batch-2 landing record:

- A trial bare Object frame made "Remove a counter from this land." go 1 → 4
  "via competing Source attachments": with the Source no longer obligatory,
  the *from* PP also read as Postmodifier of *counter* and as a free Adjunct.
  The bare passive needs the same frame.
- A trial Location licence on *combat* (the `BareNominalComplement:
  Location` route *from* uses in `BareNominalPreposition`) admitted bare
  "combat" as an ordinary NP and broke
  `combat_intervals::combat_damage_keeps_its_premodifier_analysis`
  (`crates/deckmaste_english_v3/tests/suite/combat_intervals.rs`).

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`SourceComplement` (`[phrase: PrepositionPhrase]`, `require
phrase.SourceMarker = Yes`); `frame_category SourceFreeObject =
AccusativePhrase where SourceModifierPresent = No`, already used by *return*'s
frames (glossary: **Source-free Object**); `BareIntervalNominal` and
`BareTemporalPreposition` (`agree head.NominalBareClass =
complement.NominalBareClass`, exports `SourceMarker` through
`PrepositionHeadPermissions`), which read *during combat* because
`lexeme:turn_part/combat` and *during* both declare `NominalBareClass:
Interval`.

Orchestrator rulings 2026-10-10:

- RS1. Declarations first. The bare frame's Object is `SourceFreeObject`,
  not `ObjectNounPhrase`: that removes the Postmodifier twin by an existing
  declared feature. Whether it is a second frame or the Source made optional
  in the one frame is the implementer's choice; record it.
- RS2. The free-Adjunct twin: after a verb whose frame set declares a Source
  Complement, a source-marked PP is that Complement, not an Adjunct. If RS1
  leaves "Remove a counter from this land." above 1, narrow the existing
  trailing-Adjunct route by declared features only (the PP's `SourceMarker`
  and a property the verb's frame set exports). No guard names *remove* or
  *from*. If no feature-only narrowing holds, STOP with the Reading shapes.
- RS3. *from combat* goes through the bare-interval route: *from* declares
  the `NominalBareClass` that `BareTemporalPreposition` agrees on, so *combat*
  needs no new licence and bare "combat" stays unread as an NP. The glossary
  entry **Bare Nominal Complement** is not widened. If a Preposition cannot
  hold both `BareNominalComplement: Location` and this class, STOP.
- RS4. One analysis per string. "Remove it from combat." reads once, with
  the PP as Source Complement (the passive *is removed from combat* is
  unattested: it may follow from the existing passive frame, and is neither
  required nor blocked);
  *combat damage* keeps its Premodifier analysis (the test above is
  unchanged); *from combat* does not become a Postmodifier of a preceding
  noun on any covered face.
- RS5. The bare passive follows from the bare active frame as it does for
  *sacrifice*; if it needs an explicit passive declaration, add that one
  line (as data-batch-2 did for the Source passive).
- RS6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected; 138 covered faces contain *remove*,
  and an increase there is the RS1 / RS2 defect. Report any with its shape
  before landing.

## Witnesses

Each is the face's only failing line at `uoqyuxtn`. Probed 2026-10-10 with
the `uoqyuxtn` binary (`--category document --readings 3`, `--as-card` where
the name occurs): witness 0, control as shown (3+ = enumeration capped).
Re-probe on the claim parent first.

- Saprazzan Cove: "{T}, Remove any number of storage counters from this
  land: Add {U} for each storage counter removed this way." (controls "… for
  each storage counter removed from this land this way." 2; "{T}, Sacrifice
  any number of creatures: Add {U} for each creature sacrificed this way." 2)
- Catti-brie of Mithral Hall: "{1}, Remove all +1/+1 counters from
  Catti-brie: It deals X damage to target attacking or blocking creature an
  opponent controls, where X is the number of counters removed this way."
  (control "… removed from Catti-brie this way." 3+)
- White Ward (sentence): "This effect doesn't remove this Aura." (control
  "This effect doesn't destroy this Aura." 1)
- Spires of Orazca: "{T}: Untap target attacking creature an opponent
  controls and remove it from combat." (control "… from the game." 3+)
- Gustcloak Sentinel: "Whenever this creature becomes blocked, you may untap
  it and remove it from combat." (control "… from the game." 3+)

Also 0 today: "Untap target attacking creature and remove it from combat.
Gain control of that creature until end of turn." (Disharmony). Controls that must not change: "Remove a
counter from this land." 1; "A counter was removed from this land." 1;
"Remove all counters from target permanent." 1; "Remove it from the game."
1; "Draw a card during combat." 1; "This creature deals combat damage to
target creature." 3+; "Return target creature card from your graveyard to
your hand." 1; NP "combat" 0 (stays 0).

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(remov(e|es|ed|ing)|from combat)\b

(626 faces, case-insensitive) plus 300 random covered controls (`cargo xtask
english-v3 diff --manifest … --random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. If RS2 or RS3 touches a shared route, add every face
matching `\b(from|combat)\b` to the selector and say so. Zero lost; RS6 for
increases. Full `cargo test -p` and `cargo clippy -p … --tests -- -D
warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`, once,
at the end: lexicon data loads at runtime; a rebuild after a grammar edit
(RS2 only) takes about 3 minutes and the test binary about 7. Tests assert
the role (Object without Source; bare passive as Postmodifier; *from combat*
as Source Complement; the single Reading of the marked
Source control; the RS4 negatives), never an incidental count or a
Debug-printed tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *remove all but one +1/+1 counter from it* (`at:all but`), *remove all of
  them* where *all of them* itself fails: other causes; note which remain.
- *from combat* lines with a second cause (Balduvian Warlord `word:hadn't`,
  Reconnaissance, Gollum, Scheming Guide).
- *after combat*, *until end of combat* and other interval uses not
  involving a Source.
- *devotion to*, standalone *total*, *basic landcycling* (the other
  data-batch-2 follow-ups).

## Landing record

Not landed.
