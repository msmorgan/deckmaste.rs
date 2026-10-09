---
needs: []
---
# Read a comparative property value: creatures with power less than this creature's power.

## Why

*with power less than X* never reads although *with power 2 or less* and
*its power is less than X* do. On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "with <property>
less/greater than" touches **143** unread faces and is the sole cause on
**114**. Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (`--readings 1`): "Destroy target creature with power less than 3." 0,
"… with power 3 or less." 1; "Its power is less than 3." 1.

## Goal

After a property noun (*power*, *toughness*, *mana value*), a comparative
AdjP (*less than X*, *greater than X*, *less than or equal to X*) reads as
the property value, with byte-exact realisation.

## Analysis

CGEL, Ch. 6 §3.3(c), p. 552, [46ii] (*a leader younger than you*):
postpositive AdjPs follow an ordinary noun head; Ch. 6 §2.1, pp. 528–529,
for the postpositive function. That backs the function only.

Existing route: `MeasuredAttribute`
(`crates/deckmaste_english_v3/src/declarations.rs:4047`,
`[head: Nominal, quantity: PropertyValue]`, category `PropertyValue` at
`declarations.rs:341`) reads *power 2 or less*. The comparative AdjP reads
predicatively through `ComparativeAdjectivePhrase` (`declarations.rs:4141`)
with `NominalComparativeComplement` (`declarations.rs:4137`).

Orchestrator ruling 2026-10-08 (R5a): the comparative enters as a
`PropertyValue`, through the existing property-value route, not as a new
Construction category or a new NP postmodifier construction. `PropertyValue`
is fed today by the `ScalarMeasureValue` schema instance row
`(PropertyValue, ScalarValue, ScalarPropertyValueKind)`
(`declarations.rs:4015–4018`) and the `ScalarValue` constructions after it;
widen that route so `MeasuredAttribute` reads *power less than X* the way it
reads *power 2 or less*. *less than or equal to X* belongs to the same value
slot. *whose power is less than 3* (relative-clause gap) is out of scope.

Distinct from the planned
`english-v3-postpositive-comparative-determinatives` (determinative *more*
after a measured NP); no dependency.

## Witnesses

Each is the face's only failing unit at `qsupksky` (0 Readings), each
control 1; (D) = `--category Document`:

- Howlgeist: "Creatures with power less than this creature's power can't
  block it." (control "Creatures with power 2 or less can't block it.")
- Unliving Psychopath (D): "{B}, {T}: Destroy target creature with power less
  than this creature's power." (control "… with power 2 or less.")
- Ensnaring Bridge: "Creatures with power greater than the number of cards in
  your hand can't attack." (control "Creatures with power 2 or greater can't
  attack.")
- Immobilizer Eldrazi (D): "{2}{C}: Each creature with toughness greater than
  its power can't block this turn." (control "… with toughness 2 or greater
  …")
- Kiora, Sovereign of the Deep: "You may cast a spell with mana value less
  than X from among them without paying its mana cost." (control "… with mana
  value X or less …")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose raw text contains case-insensitive "less than" or "greater than", plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, 12 workers and `--samples-per-face 0`; compare
   it with `cargo xtask english-v3 diff` against the claim parent's census of
   the same manifest. Never run `--all`. Tests: `cargo test -p` and
   `cargo clippy -p … --tests -- -D warnings` for every crate whose code or consumed data changed (`deckmaste_english_v3` always; `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed). No xtask tests, no
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
   Reading. The comparative read as a postmodifier of the whole NP (*creature with power*) rather than as the property value, or *than X* attached outside the AdjP, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *whose power is less than X* (relative-clause gap; its control also
  fails).
- Determinative *more* after a measured NP
  (`english-v3-postpositive-comparative-determinatives`).

## Landing record

1. **Census delta**: bounded `before.json` → `after.json`, both measured on
   `xrmmlsot` (before edits: claim-parent source `tzkmtpuw`; after: final feature
   source): 568 → 670 covered faces; 81,933 → 335,418 Readings. `english-v3 diff`:
   102 gained, 0 lost, 0 decreases, 0 increases on previously covered faces,
   0 digest-only, 0 source changes; 1,041 unchanged. Lost faces: none.
   Manifest: all 846 supported faces matching case-insensitive `" than "` plus
   300 random covered controls (seed 20261008), 1,143 distinct faces; all
   controls remained covered on the measured claim-parent source. Roundtrip,
   lexical-identity and structural-traversal validation: 335,418 exact Readings,
   0 issues on `xrmmlsot`.
2. **Witnesses**: each before count is 0 on `xrmmlsot` before edits
   (claim-parent source `tzkmtpuw`); after counts on `xrmmlsot`: Howlgeist 2;
   Unliving Psychopath 7; Ensnaring Bridge 5; Immobilizer Eldrazi 3;
   Kiora, Sovereign of the Deep 19. Every probe enumerated completely.
   Controls on `xrmmlsot` before → after: “Destroy target creature with power
   3 or less.” 3 → 3; “Its power is less than 3.” 1 → 1.
3. **Spot checks** (`xrmmlsot`; full-face Reading counts): Ancient Lumberknot
   8 — toughness / greater than its power; Arni Metalbrow 12 — mana value /
   less than that creature's mana value; Aura Gnarlid 10 — power / less than
   this creature's power; Baird, Argivian Recruiter 2 — power / greater than
   its base power; Beguiler of Wills 18 — power / less than or equal to the
   number of creatures you control; Broodcaller Scourge 15 — mana value /
   less than or equal to that damage; Catapult Fodder 1 — toughness / greater
   than their power; Earthshaker Khenra 9 — power / less than or equal to
   this creature's power; Ironclaw Curse 8 — power / equal to or greater than
   the enchanted creature's toughness; Legacy's Allure 31 — power / less than
   or equal to the number of treasure counters on this enchantment. Each
   selected tree is `MeasuredAttribute` → `ScalarMeasureValue` →
   `ComparativeScalarValue` → `ComparativeAdjectivePhrase`, with the whole
   comparison Complement inside the AdjP; all roundtrip exactly.
4. **Deviations and STOPs**: one new `ComparativeScalarValue` producer feeds
   the existing R5a route; schema instances cannot repeat a result Category,
   so a second `ScalarMeasureValue` instance row was unavailable. Existing
   `ComparisonMarker` distinguishes ordering and preserves coordinated
   equality; existing `PredicativeKind` confines scalar-location frames to
   nominal values. An initial probe exposed a second scalar-location Reading
   for the predicative control; the frame constraint restores its claim-parent
   count before final verification. No superseded route, lexicon change or
   STOP. Beyond the five witness tests, added coordinated-complement, control
   count and independent-roundtrip tests. Genitive scope alternatives remain
   admitted under the lexical-analysis contract; tests and spot checks also
   require the ordinary full-complement Reading. Updated the existing Scalar
   Value glossary definition for orchestrator ruling R5a; no remaining glossary
   gap. CGEL is cited only for postpositive function.
5. **Tests** (`xrmmlsot`): `cargo test -p deckmaste_english_v3` passed
   (601 passed, 0 failed, 0 ignored); `cargo clippy -p deckmaste_english_v3
   --tests -- -D warnings` passed. `cargo fmt -p deckmaste_english_v3` completed.
   Assurance counts on `xrmmlsot`: restored 0, re-spelled 0, ignored-with-blocker
   0, added 8, removed 0.
6. **Timing**: final bounded census on `xrmmlsot`: 66.25 seconds, 12 workers.
