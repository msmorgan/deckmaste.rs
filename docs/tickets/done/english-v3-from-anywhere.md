---
needs: []
---
# Read *from anywhere*: If a card would be put into your graveyard from anywhere, exile it instead.

## Why

*from anywhere* never reads. On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "from anywhere
(source PP)" touches **142** unread faces and is the sole cause on **106**.
Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (`--readings 1`): "Whenever another card is put into a graveyard from
anywhere, exile that card." 0; with *from the battlefield* 1.

## Goal

*from anywhere* reads as the source Complement or Adjunct that *from the
battlefield* already reads as, with byte-exact realisation.

## Analysis

CGEL, Ch. 5 §9.6, p. 423 (page-checked 2026-10-08): *anywhere* is one of the
central compound determinatives ([41iv], with *everywhere*, *somewhere*,
*nowhere*), which occur as fused determiner-head of an NP; "the analysis of
the locatives as heads of NPs is motivated by their use as object in
constructions like *Have you got anywhere to spend the night?*". On that
analysis *from anywhere* is *from* + NP.

Existing route: *anywhere* is `vocab LocativeProform`
(`crates/deckmaste_lexical_source/lexicon/vocabulary.rs:94`), mapped to
Adverb (`crates/deckmaste_lexical_source/src/legacy/core.rs:78`).
`vocab:Preposition/From` (`lexicon/core.ron:220`) takes
`NounPhrase|PrepositionPhrase`. `AdverbComplementPreposition`
(`crates/deckmaste_english_v3/src/declarations.rs:3125`) admits AdvP
Complements only with `AdverbComplementClass = Duration`.

Orchestrator ruling 2026-10-08 (R6a): *anywhere* is re-categorised per CGEL
p. 423 as a compound determinative functioning as NP head, not an Adverb.
Do not widen `AdverbComplementPreposition` with a locative class. Declare
*anywhere* as a Determinative read as fused determiner-head of an NP,
through whatever existing route already lets a Determinative head an NP;
add no Construction for it. If no such route admits a bare Determinative
NP, the fallback is the route that already reads *everything* as an NP head
(`vocab IndefinitePronoun`, `vocabulary.rs:169`, mapped to Pronoun at
`legacy/core.rs:90`); record that fallback as a deviation. *everything*'s
own Pronoun label (CGEL p. 423 lists it with *anywhere* as a compound
determinative) is pre-existing and is not fixed here. Retire the Adverb lexeme on both sides (the
`LocativeProform` inventory, whose only member is *anywhere*, and its
`legacy/core.rs:78` mapping) if `cargo xtask english-v3 grep` finds no
attested adverbial use of *anywhere* in the supported corpus; if it finds
one, keep both and record the attested use.

Mechanism: no existing vocab inventory maps a member to Determinative with
a fused-head use (`legacy/core.rs:81–84` maps only quantifier, demonstrative
and definite-marker inventories), so moving *anywhere* out of `Adverb` takes
a one-line remap in `legacy/core.rs:78` (rename the `LocativeProform`
inventory for its new category and map it to Determinative), not only a
`vocabulary.rs` edit. The ruling covers that edit. Pre-screen disagreement
(2026-10-08): the sizing pre-screen classed this as a reader change, the draft
as a lexicon change; resolved as a lexicon change plus that one mapping line,
size S.

Pre-integrate review ruling (2026-10-09): extend the existing postpositive
comparative modifier to a declared-licensed Fused Determiner-Head NP. On
*from anywhere other than X*, retain both *anywhere*-attached and enclosing
nominal attachments. This supersedes the exclusion of that surface below;
no new Construction or lexeme-naming guard is authorized.

## Witnesses

Each is the face's only failing unit at `qsupksky` (0 Readings), each
control 1 (*from the battlefield*):

- Brinebound Gift: "If this Aura would be put into a graveyard from anywhere,
  exile it instead."
- Yawgmoth's Agenda: "If a card would be put into your graveyard from
  anywhere, exile it instead."
- Planar Void: "Whenever another card is put into a graveyard from anywhere,
  exile that card."
- Rest in Peace: "If a card or token would be put into a graveyard from
  anywhere, exile it instead."
- Generous Soul (`--as-card`): "If Generous Soul would be put into a graveyard
  from anywhere, exile it instead." 

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose raw text contains case-insensitive "anywhere", plus 300 random covered controls
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
   Reading. *anywhere* read both as an NP and as an AdvP Complement of the same *from* (two labels for one constituency) is a defect. A wrong analysis that parses is a defect, not a gain.
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
11. The category of *anywhere* is ruled (R6a in Analysis); do not STOP on it.

## Out of scope

- *anywhere else*.
- Other uses of *anywhere* not attested in the supported corpus.

## Landing record

1. **Census delta**: claim-parent `qzwzuwwu` source measured at pre-edit
   `uzrzywom` → refreshed fix `sytlvznr`: covered faces 300 → 401; Readings
   11,533 → 37,282; gained 101, lost none, decreases 0, increases 0,
   digest-only 9 (`diff-final.json`). Relative to reviewed `tupvqkuk`,
   the fix gains 4 faces and increases 22; lost/decreases 0, digest-only 11
   (`diff-review.json`). Digest changes are the existing `PostpositiveNominal`
   Reading's added category field when generalized to a shared schema; all
   affected faces retain their counts. Incoming Get frames in `spmtqknq`
   supplied Unstable Amulet's prior 48-Reading gain; it now has 144 Readings.
   Refresh incorporated transitive-turn change `vrwvvslo`; at `ooxwlork`,
   the six-worker bounded recheck preserves all 458 faces, counts and digests
   from `rwxrkzuw` (`diff-review-refresh.json`: all delta categories 0).
   A further refresh brought comparative-property grammar `xrmmlsot`; the
   bounded six-worker check at `sytlvznr` again preserves every count and
   digest (`diff-final-refresh.json`: all delta categories 0).
   Same selector: 158 supported analysed *anywhere* faces plus 300 covered
   controls selected with seed 20261008, 458 identities in `manifest.json`.
   At `sytlvznr`, all 458 completely enumerated: 106 unique, 295 multiple,
   57 unread; limited/undetermined 0. All 37,282 Readings checked for exact
   realization and structural traversal; issues, duplicates and internal
   failures 0. Evidence: `target/english-v3/english-v3-from-anywhere/`.
2. **Witnesses**: the five ticket units at pre-edit `uzrzywom` had 0 Readings;
   implemented `uzrzywom` has Brinebound Gift 12, Yawgmoth's Agenda 12,
   Planar Void 3, Rest in Peace 12, Generous Soul 12 (card-scoped), preserved
   by the review fix's tests. Review witness at pre-fix `rwxrkzuw` (empty
   child of `tupvqkuk`): “You may cast
   spells from anywhere other than your hand.” has 1 enclosing-nominal
   attachment; *anywhere other than your hand* has 0 NP Readings. At
   `rwxrkzuw`, the sentence has 5 Readings: 4 with the anywhere-attached
   comparative Modifier and the original nominal attachment retained; the
   modified NP has exactly 1 Reading. Bare *from anywhere* retains 1.
3. **Spot checks**: ten newly covered, non-*other than* faces re-inspected
   byte-exact at `rwxrkzuw`, each with *from* + fused Determiner-Head NP:
   Abandoned Sarcophagus (84 Readings), Ancestor's Embrace (12), Archive Haunt
   (36), Compost (3), Benevolent Geist (24), Dread (6), Blightsteel Colossus
   (44), Brinebound Gift (96), Case of the Gorgon's Kiss (1,500), Catlike
   Curiosity (36). The reviewed *other than* faces now each have an inspected
   Reading where the Comparative Adjective Phrase postmodifies the
   FusedDeterminerHeadNounPhrase *anywhere* inside the source PP: Avatar's
   Wrath (12), Bilbo, Thief in the Night (3,120), Dimir Strandcatcher (14),
   Drannith Magistrate (5), Flaming Tyrannosaurus (36), Ghostly Pilferer (15),
   Iraxxa, Empress of Mars (12), Keeper of Secrets (30), Memory Worm (12),
   Mizzix, Replica Rider (78), Nico Minoru, Runaway (288), Osgood, Operation
   Double (144), Poxwalkers (12), Sage of the Beyond (16), Shadow of the
   Goblin (32), Sisterhood of Karn (300), Spider-Man 2099 (1,872), The
   Thirteenth Doctor (240), Ultron the Annihilator (12), Ultron's Auxiliary
   (12), Vega, the Watcher (6). Counts measured at `rwxrkzuw`; enclosing
   nominal attachments remain admitted alongside the new scope.
4. **Deviations and STOPs**: initial R6a moves the sole *anywhere* inventory
   and mapping from Adverb to Determinative, without the Pronoun fallback.
   The HIGH pre-integrate finding exposed missing comparative NP scope in
   the original gains; fixed under the user's 2026-10-09 review direction,
   which expands the ticket's prior *other than* exclusion. Generalized the
   existing `PostpositiveNominal` into one shared schema with its unchanged
   nominal instance and one comparative NP instance. No new named
   Construction, frame or word guard. Declared `ComparativePostmodifierUse`
   on *anywhere*, exported it from the fused head, and explicitly supplied
   No on other NP/coordination summaries. Defined Comparative Postmodifier
   Use in the owning glossary; no remaining glossary gap. Added the requested
   attachment/retention test with independent NP roundtrip and unlicensed-head
   checks; re-spelled `grammar::complemented_adjectives_are_postpositive_in_both_directions`
   for the shared Reading's category field, preserving every assertion.
   No STOPs; the retired LocativeProform/Adverb route remains retired.
5. **Tests**: at `rwxrkzuw`, `cargo test -p deckmaste_english_v3` passed
   (616 tests); `cargo test -p deckmaste_lexical_source` passed (27 unit +
   35 integration tests). `cargo clippy -p deckmaste_english_v3 --tests --
   -D warnings` and `cargo clippy -p deckmaste_lexical_source --tests --
   -D warnings` passed. English clippy first caught similar variable names
   in the new test; renamed the binding and reran it successfully. No test
   command needed a retry. The resume instruction (2026-10-09) retains this
   completed test/clippy evidence without rerunning it. After final refresh
   brought grammar `xrmmlsot`, the required affected-crate check at `sytlvznr`,
   `CARGO_INCREMENTAL=0 cargo test -p deckmaste_english_v3`, passed 634 tests
   including the attachment/retention regression; lexical-source and clippy
   evidence was retained. The missing refreshed
   census first failed to link with rust-lld undefined symbols; retried with
   `CARGO_INCREMENTAL=0` as directed. Assurance totals at `rwxrkzuw`: restored 0,
   re-spelled 1, ignored 0, added 8, removed 0. This fix adds 1 and
   re-spells 1; removes none.
6. **Timing**: the bounded final census at `sytlvznr` took 6.459085849 s
   corpus wall time with 6 workers.
