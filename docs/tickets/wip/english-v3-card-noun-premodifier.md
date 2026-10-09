---
needs: []
---
# Let *card* premodify a noun: the number of card types among cards in your graveyard.

## Why

*card type* and *card name* never read. On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "card as
noun premodifier (*card type* / *card name*)" touches **224** unread faces
and is the sole cause on **183** (Delirium counts, *choose a card name*,
*shares a card type with it*). Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (`--readings 1`): "Choose a card type." 0; "Choose a creature type." 1;
"Choose a card name." 0.

## Goal

*card* functions as a noun Premodifier in *card type(s)* and *card
name(s)*, through the declared `NounPremodifier` feature, with byte-exact
realisation. No Construction is added.

## Analysis

Existing route: `NounPremodifiedNominal` reads the declared
`NounPremodifier: Yes`. `lexeme:CommonNoun/Permanent` has it
(`crates/deckmaste_lexical_source/lexicon/core.ron:113`);
`lexeme:CommonNoun/Card` (`core.ron:112`) does not. The landed
`english-v3-permanent-noun-premodifier` is the precedent and its CGEL citation
(CGEL, Ch. 6 §2.4.1(a), p. 537: nouns as attributive modifiers) applies unchanged.

## Witnesses

Each is the face's only failing unit at `qsupksky` (0 Readings), each
control 1:

- Crop Sigil: "Activate only if there are four or more card types among cards
  in your graveyard." (control "… creature types …")
- Peer Past the Veil: "Then draw X cards, where X is the number of card types
  among cards in your graveyard." (control "… creature types …")
- Deluge of Doom: "All creatures get -X/-X until end of turn, where X is the
  number of card types among cards in your graveyard." (control "… creature
  types …")
- Memoricide: "Choose a nonland card name." (control "Choose a nonland
  permanent name.")
- Curse of Silence: "As this Aura enters, choose a card name." (control "… a
  permanent name.")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose raw text contains case-insensitive "card type" or "card name", plus 300 random covered controls
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
   Reading. *card* read as a premodifier of a noun other than an attested one, or a *card type* string with two Readings that differ only in a duplicated route, is a defect. *nonland card name* bracketed as [*nonland* [*card name*]] and [[*nonland card*] *name*] are both grammatical; keep both. A wrong analysis that parses is a defect, not a gain.
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

- *card* premodifying nouns not attested in the supported corpus.
- Delirium hosts whose controls also fail.

## Rulings (orchestrator, 2026-10-08)

- Method §5's second bracketing of *nonland card name* is withdrawn.
  `[nonland [card name]]` is accepted; compound nominal premodifier, routed
  to recon.
- The nine increases introducing coordinated Premodifiers containing *card*
  over *card* or *planeswalker* are wrong analyses. A distinct declared
  Attribute noun Premodifier use selects heads declaring Attribute Premodifier
  Head permission. Only attested heads declare that permission; coordination
  retains the restricted use. Both sides of admission read declared features.
- The selector is every supported face containing case-insensitive *card*,
  union 300 seeded previously covered controls (seed 20261008).

## Landing record

1. **Census delta** (baseline `mqrlzszkuszn`, claim-parent-equivalent to `qsupksky`; corrected pre-refresh `txvmkwvvnruu`): bounded covered faces 7,663 → 7,828; total Readings 880,610 → 1,016,226. Built-in `english-v3 diff`: gained 165; lost 0; decreases 0; increases 0; digest-only 0; source changes 0. Lost faces: none. Every selected face enumerated completely; every Reading passed exact realization and structural traversal; validation issues, internal failures and duplicates all 0. Evidence: `target/english-v3/english-v3-card-noun-premodifier/`. After refresh at `yspwwwmxnyly`, the same bounded manifest has identical counts and digests; the inherited `wtnuopxm` landwalk changes bring no decreases or other census deltas.
2. **Witnesses** (`mqrlzszkuszn` before → `txvmkwvvnruu` after, ticket units): Crop Sigil 0 → 25; Peer Past the Veil 0 → 12; Deluge of Doom 0 → 24; Memoricide 0 → 1; Curse of Silence 0 → 1. Full-face counts at `txvmkwvvnruu`: respectively 50, 12, 24, 60, 12. Probe enumeration is complete and byte-exact, with no validation issues. Memoricide's single `[nonland [card name]]` analysis is accepted by the orchestrator ruling.
3. **Spot checks** (`txvmkwvvnruu`, full-face counts; each selected Reading checked by probe): Alpine Moon 6, *card* premodifies *name* inside *nonbasic land card name*; Altar of the Goyf 72, *card types* inside the number expression; Amareth, the Lustrous 40, *card type* selected by *shares*; Atraxa, Grand Unifier 6, *each card type*; Blood Oath 23, *choose a card type*; Booby Trap 4, both *card name* occurrences; Brain Pry 3, *nonland* outside *card name*; Curse of Silence 12, *choose a card name* in the preposed clause; Darksteel Mutation 24, *card types* coordinated with abilities and creature types; Memoricide 60, *nonland* outside *card name*. Each *card* is a singular noun Premodifier retaining its common-noun lexical identity.
4. **Deviations and STOPs**: both STOPs resolved by the orchestrator ruling of 2026-10-08: compound nominal premodifier, routed to recon; the second bracketing demand is withdrawn. The unrestricted trial at `txvmkwvvnruu` wrongly increased nine covered faces: Compass Gnome, District Guide, Open the Gates, Prishe's Wanderings, Roots of Wisdom, Sanctum of Shattered Heights, Scampering Surveyor and Szarekh, the Silent King (*card* premodifying *card*); Haven of the Spirit Dragon (*card* premodifying *planeswalker*). Those bracketings are unattested and retired; all nine final counts and digests equal the claim parent. A declared Attribute licence, attested head permissions and propagated coordination constraints refine existing declarations; no Construction or frame added or removed, no loader code or word-naming guard. *Pool* is additionally declared from Deal Broker's attested *card pool*; no *card draw* occurs. Glossary gaps closed: Attribute Noun Premodifier and Attribute Premodifier Head. The three additional test functions cover independent values and the requested negative/positive controls; Baba Lysaga's retired vocabulary-blocker assertion is re-spelled while retaining its authentic clause as a positive witness.
5. **Tests** (before refresh, `txvmkwvvnruu`): `cargo test -p deckmaste_english_v3` passed (588); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed; `cargo test -p deckmaste_lexical_source` passed (26 unit and 35 integration); `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed. English's earlier compile ambiguity was corrected; its obsolete Baba Lysaga blocker expectation was re-spelled after the first executed suite found it. Changed-test accounting: restored 0; re-spelled 1 (`existential_there::contracted_ambiguity_and_preterite_lexical_form`); ignored with blockers 0; added 8; removed 0. Value comparisons, existing lexical-form assertions and both roundtrip laws are preserved. After refresh at `yspwwwmxnyly`, only the two changed-crate test commands were repeated and passed: English 588; lexical source 27 unit and 35 integration (the extra unit test is inherited from `wtnuopxm`).
6. **Timing** (`txvmkwvvnruu`): corrected bounded census wall time 108.816 seconds with 12 workers (setup plus corpus, compilation excluded). The manifest selects 10,510 supported analysed-text faces containing case-insensitive *card*, union 300 seeded previously covered controls (seed 20261008; 90 overlaps), for 10,720 faces; `--samples-per-face 0`.
