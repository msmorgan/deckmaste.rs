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

- *anywhere else*, *from anywhere other than your hand*.
- Other uses of *anywhere* not attested in the supported corpus.

## Landing record

1. **Census delta**: bounded pre-edit `uzrzywom` (empty child of claim parent
   `qzwzuwwu`, source-equivalent) → implemented `uzrzywom`: covered faces
   300 → 396; Readings 11,533 → 33,324. Gained 96; lost none; decreases 0,
   increases 0, digest-only 0. Selector: every supported analysed face containing
   case-insensitive *anywhere* (158 faces), plus 300 covered controls selected by
   `english-v3 diff --manifest --random 300 --seed 20261008`: 458 identities in
   `target/english-v3/english-v3-from-anywhere/manifest.json`. `diff.json` compares
   the identical manifest, with no identity or source-hash changes. At
   `uzrzywom`, all 458 faces completely enumerated; 107 unique, 289 multiple,
   62 unread, 0 undetermined or limited. Exact realization and structural
   traversal checked for all 33,324 Readings; issues, duplicates and internal
   failures all 0. The older batch census (`mqrlzszk`, source tree `qsupksky`)
   supplied the seeded controls; a bounded pre-edit census measured the actual
   claim parent, which already includes subsequent grammar landings.
   Post-refresh bounded recheck at `tupvqkuk`: 397 covered / 33,372 Readings;
   107 unique, 290 multiple, 61 unread. All 458 faces completely enumerated;
   issues, duplicates, internal failures and non-exact Readings all 0.
   `diff-refresh.json` attributes the one additional gain, Unstable Amulet
   (48 Readings), to incoming Get frames in `spmtqknq`; losses, decreases,
   increases and digest-only changes remain 0. `diff-final.json` reports
   97 gains in total against the claim-parent baseline.
2. **Witnesses**: at pre-edit `uzrzywom`, all five `english-v3 probe` calls
   returned 0 Readings. At implemented `uzrzywom`, complete witness-unit
   enumeration returned Brinebound Gift 12, Yawgmoth's Agenda 12, Planar Void 3,
   Rest in Peace 12, Generous Soul 12 (card-scoped). Each has the NP-headed
   source PP; independently probed *from anywhere* has exactly 1 Reading.
3. **Spot checks**: ten newly covered faces at `uzrzywom`, each inspected with
   a byte-exact Reading whose *from* PP has the compound Determinative as a
   Fused Determiner-Head NP Complement: Abandoned Sarcophagus (84 Readings),
   Ancestor's Embrace (12), Archive Haunt (36), Avatar's Wrath (2), Benevolent
   Geist (24), Bilbo, Thief in the Night (1,560), Blightsteel Colossus (44),
   Brinebound Gift (96), Case of the Gorgon's Kiss (1,500), Catlike Curiosity
   (36). NP Modifier attachment on the *other than* surfaces uses existing
   grammar; those incidental gains add no route.
4. **Deviations and STOPs**: R6a implemented directly, without the Pronoun
   fallback or any Construction/frame change. Retired the sole-member
   `LocativeProform` inventory and its Adverb mapping; the supported-corpus
   `english-v3 grep` evidence at pre-edit `uzrzywom` shows only *from anywhere*
   and the out-of-scope *putting it/them anywhere else* location Complement,
   no independent Adverb use. No dedicated locative-Adverb Construction
   exists to retire; the general Adverb and duration-Complement routes remain
   used by other lexemes. Fused-head features live in `core.ron` because the
   vocabulary declaration reader rejects them inline; no new loader logic.
   Beyond the five required witness tests, added an independent NP/PP
   roundtrip-and-traversal test and an ownership/Adverb-retirement test.
   No STOPs, guards or glossary gaps.
5. **Tests**: at `uzrzywom`, `cargo test -p deckmaste_english_v3` passed
   (608 tests); `cargo test -p deckmaste_lexical_source` passed (27 unit and
   35 integration tests). `cargo clippy -p deckmaste_english_v3 --tests --
   -D warnings` and `cargo clippy -p deckmaste_lexical_source --tests --
   -D warnings` both passed. The first English test build failed before running
   tests on an ambiguous `contains_key` argument in the new ownership test;
   corrected the argument's type and repeated that command successfully.
   Assurance counts at `uzrzywom`: restored 0, re-spelled 0, ignored 0,
   added 7, removed 0. After the lexicon refresh, at `tupvqkuk`, repeated only
   `cargo test -p deckmaste_english_v3` (615 passed, including the incoming Get
   tests) and `cargo test -p deckmaste_lexical_source` (27 unit + 35 integration
   passed); failures and ignored tests both 0. Prior clippy results retained.
6. **Timing**: the one bounded landing census at `uzrzywom` took 4.848493556 s
   corpus wall time with 12 workers.
