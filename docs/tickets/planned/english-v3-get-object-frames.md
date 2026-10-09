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

1. **Census delta**: bounded before → after (covered faces, Readings); lost
   faces classified by name; decreases, increases, digest-only changes.
2. **Witnesses**: each witness's Reading count before and after.
3. **Spot checks**: ten newly covered faces with their selected Reading.
4. **Deviations and STOPs**: constructions or tests added or removed beyond
   the ticket's letter, with justification; every STOP and its resolution;
   any glossary gap.
5. **Tests**: commands and outcomes; restored, re-spelled, ignored with
   blockers, added, removed.
6. **Timing**: one bounded census wall time with worker count.
