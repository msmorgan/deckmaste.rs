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

- RS7 (2026-10-10). Source-free applies to the whole Object, including
  transparent Obliques and Genitive Possessors. Project the declared Source
  class through those extensions; no word tests. The six nominal-extension
  increases must return to their claim-parent counts.
- RS8 (2026-10-10), superseded by RS12. A Source PP following a gerund whose predicate used a
  Source-free Object frame belongs to that predicate. Export a declared
  boundary through the gerund PP and reject Source-class matrix Adjuncts
  across it. Dawnhand Dissident, Falco Spara and Quilled Greatwurm must
  return to their claim-parent counts.
- RS9 (2026-10-10). Stormfront Riders' two removed Source Adjunct Readings
  require one-for-one selected Complement replacements, or must remain.
  Implementation choice: preserve those existing Adjunct Readings by leaving
  Return's prior attachment licence unchanged. Adding a new Return frame
  would broaden this ticket beyond the required Remove construction.
  None of the nine increases is accepted. Verify the same 6,634 identities;
  stop on any surviving increase, and classify gains by intended analysis.

- RS10 (2026-10-10). The 34 Return decreases are not accepted. Preserve
  the head-only Source-free Object licence for every existing Frame;
  the ticket's new bare-Object Frames declare a distinct whole-Object
  licence. Expected: zero decreases.
- RS11 (2026-10-10), superseded by RS12. Forward the RS8 Source boundary through auxiliaries
  instead of resetting it, for this property only. Expected: Dawnhand
  Dissident, Falco Spara and Quilled Greatwurm retain their before counts.
  Verify the same 6,634 identities: zero lost/increases/decreases, 67 gains
  with intended analyses. Stop with the shape if anything else moves.

- RS12 (2026-10-10), superseding RS8/RS11. Remove the gerund-boundary
  property, its gerund/auxiliary forwarding and its trailing-Adjunct row;
  restore that grammar to its claim-parent shape. Retain the new Remove
  frames, combat Source, RS7 scoped by RS10 and RS9. Accept only the three
  gerund increases (Dawnhand Dissident, Falco Spara, Quilled Greatwurm),
  whose following Source PP can also attach to the matrix Predicate.
  This ordinary PP-attachment ambiguity is follow-up work, not a defect
  of this ticket. Expected: zero lost/decreases, 67 intended gains, exactly
  those three increases; otherwise stop with the shape.

- RS13 (2026-10-10). A bare passive Postmodifier followed by a Source PP
  may stack that PP as a second Postmodifier: NP "each counter removed
  from this land" 1 → 2. This is an accepted unattested residual of the
  same family as RS12 (0 covered faces have the shape); no guard is added.

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

1. **Census delta** (`nxxknqvk`; claim-parent content `kntxoopo`). Bounded
   selector `\b(remov(e|es|ed|ing)|from|combat)\b`, case-insensitive:
   6,383 matching faces plus 300 random covered controls, seed 20261008,
   6,634 unique identities. Covered 5,480 → 5,547; Readings
   272,490 → 276,357; gained 67, lost none, decreases 0, digest-only 0,
   issues 0. Exactly three increases, accepted by ruling RS12:
   Dawnhand Dissident 1,154 → 3,273; Falco Spara, Pactweaver 56 → 109;
   Quilled Greatwurm 34 → 71. Added shape: Source PP after the bare-Object
   gerund `by removing N counters` attaches to the matrix Predicate rather
   than being selected by the gerund. All six RS7 faces and the three
   without-paying controls retain their before counts; Stormfront Riders
   retains 5. Built-in `diff --lines`: `diff.json`, under
   `target/english-v3/english-v3-remove-source-attachment/`.
   After refresh (`vppqlzlo`, empty child of `nxxknqvk`), built-in diff
   on the seeded 1,500-face manifest sample: covered 1,500 → 1,500,
   Readings 54,535 → 54,535, lost/gained/decreases/increases 0, issues 0.
   Seven digest-only changes belong to trunk `soxvyspk`
   (`MultiplierNounPhrase` → `PredeterminerNounPhrase`); no removed
   Readings. `after-refresh.json` / `refresh-diff.json`.
   Final integration refresh (`tqryzout`): the same 1,500 identities have
   covered 1,500 → 1,498, Readings 54,535 → 54,523, lost 2, gained 0,
   decreases/increases 0, digest-only 45. Glacierwood Siege 8 → 0 and
   Palace Siege 4 → 0: lost on trunk by pxtwrtvu before this refresh;
   not caused by this change. Their retired shapes are Flavor Word-headed
   standalone named-bullet Document Readings.
   Complete probes confirm zero replacement Document Readings. The 45
   digest-only changes accompany trunk's optional secondary negator
   `wqkrtoxr`; counts are unchanged. Evidence: `after-final-refresh.json`,
   `final-refresh-diff.json` and `final-refresh-lost-*.json`. Integration
   is authorized by the orchestrator ruling of 2026-10-10: these two faces
   and eight other anchor-word faces were lost on trunk; their separate
   repair belongs to the named-modes-regression workspace. Own gains remain 67.
2. **Witnesses** (`nxxknqvk`, complete probes, claim-parent → after):
   Saprazzan Cove 0 → 2; Catti-brie of Mithral Hall 0 → 8; White Ward's
   sentence 0 → 1; Spires of Orazca's ability 0 → 4; Gustcloak Sentinel
   0 → 4; Disharmony's fragment 0 → 2. None already read on the parent.
   All eight must-not-change controls retain their counts: six at 1, combat
   damage at 3, bare NP combat at 0. The two witness-paired Source controls
   increased: "… for each storage counter removed from this land this way."
   2 → 4 and the Catti-brie "removed from Catti-brie this way" control 8 → 18. `probes-before.log` / `probes-rs12.log`.
3. **Spot checks** (`nxxknqvk`). Black Ward, Blue Ward, White Ward — bare
   Object; Saprazzan Cove, Catti-brie, Hex Parasite, Bounty of the Luxa —
   Bare Passive Postmodifier; Spires of Orazca, Gustcloak Sentinel,
   Disharmony — selected combat Source Complement. Current-build role
   inspection finds an intended analysis on all 67 gained faces: 20 bare
   Object, 29 bare passive, 18 combat Source. Unintended-only gains: 0,
   none. Evidence: `gain-role-review.json` and `gain-*.json`; this checks
   existence of an intended analysis, not every alternative's correctness.
4. **Deviations and STOPs** (`nxxknqvk`). Separate bare active/passive
   frames are appended in core.ron, preserving old indices. RS2 adds
   declared Source selection and feature-only predicate/Clause attachment
   guards. RS3 adds From's Interval class alongside Location. RS7 scoped
   by RS10 uses WholeSourceFreeObject only on the new frames; the original
   head-only licence remains unchanged. RS9 preserves both Stormfront
   Riders Source Adjunct Readings. RS12 resolves the three STOP rounds by
   retiring the RS8/RS11 gerund-boundary mechanism entirely; its ordinary
   attachment ambiguity is routed to systemic residuals. Glossary terms
   cite project rulings. The accepted shapes are in
   `rs12-accepted-increases.log`; no word-naming guard or Lean change.
   Refresh conflicts in category declarations and suite_6 registrations
   were resolved preserving both feature sets and all test modules. The
   incoming focused-NP and participial-supplement anchor projections now
   preserve this ticket's declared features (`SourceSelectionAnchor` and
   the existing Whole-Object projection). Two compile errors in the merge
   repair were corrected before the successful rebuild and full checks.
   The final refresh preserved both AuxiliaryProFormLicence and this
   ticket's Source selection in selected-head categories, and registered
   do-so, named-mode and Remove test modules once each. Its new
   ProFormComplement needed SourceComplementSelection = No, like the
   existing elided Complement, to compile with this ticket's required
   export. No RS8/RS11 mechanism was restored. A new STOP records the two
   named-bullet losses brought by `pxtwrtvu`; the orchestrator confirms
   these are trunk losses and authorizes integration with their repair in
   a separate workspace. No unrelated guard is added.
   Shapes accepted under RS12/RS13, all unattested on covered faces
   (census: 0 increases beyond RS12's three): (i) bare passive Postmodifier
   followed by a Source PP: NP "each counter removed from this land"
   1 → 2, 3 in a Clause; `removed from` is 19 faces, all finite passives
   plus Cogwork Grinder. (ii) bare-Object *remove* with the Source PP read
   as Adjunct of a relative clause inside the Object: Reconnaissance
   4 of 8 Readings; False Orders has the same string. (iii) *from combat*
   outside *remove* reads as an ordinary *from* PP ("Draw a card from
   combat." 2); all 24 `from combat` faces are *remove NP from combat*.
5. **Tests** (`nxxknqvk`). `cargo test -p deckmaste_english_v3` passed
   803 tests across all six binaries; `cargo test -p deckmaste_lexical_source`
   passed 82. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`
   passed. Against trunk: 13 added, 3 re-spelled; the RS12 gerund test is new
   against trunk. Restored 0 / ignored 0 / removed 0.
   Re-spelled: `declarations_fill_attested_roles_and_preserve_both_roundtrip_laws`
   (combat checks moved to an added test),
   `sun_droplet_remove_retains_object_and_marked_source_roundtrip` and
   `frames_preserve_objects_selected_complements_and_existing_plugin_frames`
   (complete frame inventories extended). The new RS12 test is
   `gerund_source_retains_selected_and_matrix_attachments_under_rs12`
   (Falco line, selected Source retained, matrix alternative accepted).
   After refresh (`vppqlzlo`), both full commands passed again: 849 English
   tests, 83 lexical-source tests; both clippy commands passed again.
   `cargo build -p xtask` passed before and after refresh. No xtask,
   construction-core or plugin-data change required its test suite.
   Final integration refresh: `cargo build -p xtask` passed after repairing
   the required ProFormComplement export. The full English command passed
   all six binaries (866 tests); full `cargo test -p deckmaste_lexical_source`
   and `cargo test -p deckmaste_lexical` passed. Clippy with `--tests -- -D
   warnings` passed for all three crates, including the incoming lexical
   code. No construction-core or plugin-data changes came in.
   Citation checks: 0 noncompliant, 0 stale, 0 changed citation sites.
6. **Timing** (`nxxknqvk`). Bounded AFTER census **4:49.89** wall under
   `/usr/bin/time -v`, 6 workers, 6,634 faces, `--samples-per-face 0`.
   No full-corpus or `--all` run.
