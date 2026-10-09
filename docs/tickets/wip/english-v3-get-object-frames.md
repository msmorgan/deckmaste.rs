---
needs: []
---
# Give *get* its Object frames: You get {E}{E}. Each opponent gets a poison counter.

## Why

*get* reads only with a P/T adjustment. On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "player
*get* energy/counters" touches **185** unread faces and is the sole cause on
**144**; 88 of the sole faces have *{E}*. Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (`--readings 1`): "Each opponent gets a poison counter." 0, *gains* 1;
"You get {E}{E}." 0, "You pay {E}{E}." 1, "You gain {E}{E}." 0.

## Goal

*get* takes an Object NP (*a poison counter*, *two rad counters*, *an
experience counter*) and the symbol-sequence Object that *pay* already takes
(*{E}{E}*). *gets +1/+1* keeps its single existing Reading.

## Analysis

Existing route: `crates/deckmaste_lexical_source/lexicon/verbs.ron:78`
declares Get with one frame, `Predicate([Role("PowerToughnessAdjustment")])`.
Gain (`verbs.ron:74`) has `ObjectNounPhrase`; Pay (`verbs.ron:106–108`) has
`ObjectNounPhrase` and `Role("CostSymbols")`. Add both attested frames to Get.

Orchestrator ruling 2026-10-08 (R3a): the symbol Object (*you get {E}{E}*)
reuses Pay's existing `CostSymbols` Role; no new Role. The counter Object
(*gets a poison counter*) goes through the existing counter-object frame,
which is the plain `ObjectNounPhrase` frame Gain uses for *gains a poison
counter* (there is no counter-specific frame). *gets +1/+1* keeps its own
`PowerToughnessAdjustment` frame and must not also read through the Object
frame.
No CGEL citation applies to the symbol Object (Oracle notation); the NP
Object is ordinary monotransitive complementation.

*You get an emblem with "…"* (recon bucket "emblem body", 89 / 63) is hosted
by this frame; record how many of those faces read after the landing, without
working on the quoted bodies.

## Witnesses

Each is the face's only failing unit at `qsupksky` (0 Readings), each
control 1:

- Attune with Aether: "You get {E}{E}." (control "You pay {E}{E}.")
- Phyresis Outbreak: "Each opponent gets a poison counter." (control
  "… gains …")
- Crypt Cobra: "Whenever this creature attacks and isn't blocked, defending
  player gets a poison counter." (control "… gains …")
- Scrapper Champion: "When this creature enters, you get {E}{E}." (control
  "… you pay {E}{E}.")
- Vexing Radgull: "Whenever this creature deals combat damage to a player,
  that player gets two rad counters if they don't have any rad counters."
  (control "… gains two rad counters …")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose raw text contains case-insensitive " get " or " gets ", plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, 12 workers and `--samples-per-face 0`; compare
   it with `cargo xtask english-v3 diff` against the claim parent's census of
   the same manifest. Never run `--all`. Tests: `cargo test -p` and
   `cargo clippy -p … --tests -- -D warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`. No xtask tests, no
   Lean checks, no post-refresh re-census unless refresh touched grammar,
   lexicon or core. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Probe each witness before implementation and record the counts; write each
   witness as a test. Run the tests and clippy once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. A P/T adjustment (*gets +1/+1*) that also reads through the Object frame, or a symbol Object read as anything but the declared Role, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests. Prefer refining an existing Construction: the count is above the
   ADR's 250 ceiling.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- Emblem bodies and other quoted abilities.
- *get* frames not attested in the supported corpus.

## Landing record

1. **Census delta** (`spmtqknq`, before/after): claim-parent `ywrkppnm`-equivalent baseline measured before implementation on its empty child; bounded selector = case-insensitive ` gets? ` over supported analysed text (5,224 faces), plus 300 covered controls from built-in `diff --manifest --random 300 --seed 20261008` (45 overlaps), 5,479 distinct identities. Covered faces 4,382 → 4,592; total Readings 194,450 → 227,664. Built-in `english-v3 diff`: gained 210; lost faces: none; decreases, increases, digest-only and source changes all 0. All enumerations complete; exact realization and structural traversal validated every Reading; issues, duplicate Readings and internal failures all 0. `grep 'get an emblem' -i` attests 83 faces; 51 read afterward, with quoted bodies unchanged. Evidence: `target/english-v3/english-v3-get-object-frames/`.
2. **Witnesses** (`spmtqknq`, complete fragment probes before → after): Attune with Aether 0 → 1; Phyresis Outbreak 0 → 1; Crypt Cobra 0 → 2; Scrapper Champion 0 → 1; Vexing Radgull 0 → 9. All controls read before implementation; complete control enumeration finds 2 for Crypt Cobra and 9 for Vexing Radgull (the ticket's minting probes capped inspection at 1). Finite-predicate fragment *gets +1/+1* retains exactly 1 → 1; Glorious Anthem's *Creatures you control get +1/+1.* also retains 1 → 1.
3. **Spot checks** (`spmtqknq`, whole-face counts; one selected Reading inspected per gain): Attune with Aether (7), energy SymbolSequence in `CostSymbols`; Aether Chaser (1), energy SymbolSequence; Aether Hub (1), single energy SymbolSequence; Aether Meltdown (1), energy SymbolSequence with its separate -4/-0 SlashPair frame preserved; Aether Poisoner (1), energy SymbolSequence; Crypt Cobra (2), AccusativePhrase headed by the declared poison-counter Compound Noun; Phyresis Outbreak (6), poison-counter Object with its separate -1/-1 SlashPair frame; Vexing Radgull (9), quantified rad-counter Object; Daxos the Returned (4), experience-counter Object; Ajani, Adversary of Tyrants (16), emblem Object with quoted *with* postmodifier. All inspected Readings realize exactly and validate traversal; symbols select frame 2, Noun Phrases frame 1 and SlashPairs frame 0.
4. **Deviations and STOPs** (`spmtqknq`): exactly the two declared Get frames added under orchestrator ruling R3a (2026-10-08); no new Construction, Role, guard, superseded route, CGEL claim or glossary gap. The retained P/T frame is not superseded. Added an independent energy-Object test for both roundtrip laws and traversal and a P/T uniqueness test alongside the five required witnesses. No STOPs. The inherited census predates intervening landings, so an accurate bounded claim-parent-equivalent baseline was captured before edits rather than attributing inherited gains to this ticket.
5. **Tests** (`spmtqknq`): `cargo test -p deckmaste_english_v3` passed 587 tests; `cargo test -p deckmaste_lexical_source` passed 62 tests. `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` both passed. Tests written once and run once; no linker retry needed; no xtask tests or Lean checks. Assurance: restored 0; re-spelled 0; ignored with blockers 0; added 7; removed 0. Existing tests and structural assertions retained.
6. **Timing** (`spmtqknq`): landing bounded census wall time 63.901 s with 12 workers (setup plus corpus, compilation excluded).
