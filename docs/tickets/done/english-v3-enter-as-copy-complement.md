---
needs: []
---
# Read Enter's selected *as a copy of* Complement: enter as a copy of a creature you control

## Why

No *enter as a copy of* sentence reads. On census `xnkuznpo` (docs-only
descendant of `vvxtllnq`; 32,828 supported faces, 23,459 covered, 9,369
unread; wave-4 recon of 2026-10-08), the recon bucket "enter as a copy of"
touches **63** unread faces and is the sole cause on **51**. Of the 94 failing
", except it …" units, 40 sit on this host. It is also one of the two
prerequisites of `english-v3-clause-final-predicative-as` (orchestrator ruling
2026-10-07). Counts are unread faces *touched* / *sole* as defined in the
recon; surface counts, not gain forecasts.

Probes (admitted roots): "You may have this creature enter as a copy of a
creature you control." 0, while "You may have this creature enter tapped." 1;
"Creatures you control enter as a copy of this creature." 0, while
"Creatures you control enter tapped." 1.

## Goal

*enter* selects an *as* + NP predicative Complement (*as a copy of …*), also
after a depictive (*enter tapped as a copy of …*), realised byte-exactly. The
*as* phrase is a Complement of *enter*, never a trailing Adjunct, per the
predicative-*as* orchestrator ruling of 2026-10-06 recorded in
`english-v3-systemic-residuals` ("Enter selected predicative-as Complement").

## Analysis

CGEL, Ch. 4 §5.1, p. 255, [9]: predicative obliques are commonly marked by
*as* (*That counts as excellent*, *She served as treasurer*). That backs an
*as*-marked predicative Complement. Its selection by *enter* is the 2026-10-06
ruling.

Existing route: Enter's frames (`verbs.ron:63–69`) have no predicative
Complement. `vocab:Preposition/As` declares `PrepositionComplement:
PredicativeNounPhrase` (`core.ron:181`), and
`PredicativeComplementPreposition` (`declarations.rs:2689`) builds the PP.
This ticket adds the Enter frame and its consumer, reusing that PP.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Essence of the Wild: "Creatures you control enter as a copy of this
  creature." (control "Creatures you control enter tapped." 1)
- Mirror Image: "You may have this creature enter as a copy of a creature you
  control." (control "You may have this creature enter tapped." 1)
- Vesuva: "You may have this land enter tapped as a copy of any land on the
  battlefield."
- Body Double: "You may have this creature enter as a copy of any creature card
  in a graveyard."
- Masterwork of Ingenuity: "You may have this Equipment enter as a copy of any
  Equipment on the battlefield."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed (`deckmaste_english_v3` always;
   `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed).
   No xtask tests, no Lean checks, no post-refresh re-census unless refresh
   touched grammar, lexicon or core. New tests go under tests/suite/
   registered in tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. An *as a copy of* phrase read as an Adjunct, or as a postmodifier of the Subject, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- The *, except it …* content clause after the copy: count newly covered
  faces such as Quicksilver Gargantuan if they read, but do not change the
  except route here.
- *becomes a copy of*, *create a token that's a copy of* hosts.
- Plural agreement in Hulking Metamorph's "… and its power and toughness are
  equal to this creature's power and toughness"
  (`english-v3-coordinated-subject-agreement`).
- Clause-final predicative *as*: `english-v3-clause-final-predicative-as`.

## Landing record

1. **Census delta.** Bounded claim-parent baseline `mqrlzszk` (grammar at claim `qyyktsrr`): 1,559 covered faces / 189,579 Readings → `voyrmlnk`: 1,577 / 191,741 (+18 / +2,162). `voyrmlnk`: lost faces none; removed Readings none; decreases, increases on previously covered faces, digest-only changes and internal failures all zero. Every counted Reading passed admission, byte-exact realization and node/word traversal validation. Newly covered faces carrying “, except it …”: zero (`voyrmlnk`). After refresh, `vmksqonr` retains 1,577 covered faces and has 142,793 Readings: no lost faces, new gains, increases or digest-only changes relative to `voyrmlnk`; zero validation issues or internal failures. The 17 inherited decreases (48,948 removed Readings) are from name-scope consolidation `zntxxpru`, confirmed by official `trunk-refresh-diff.json`: Aang, A Lot to Learn 225→9; Azula, Cunning Usurper 29106→3234; Captain America, Super-Soldier 1280→20; Cloud, Midgar Mercenary 2268→84; Daxos of Meletis 18432→4608; Elektra, Daughter of the Hand 10→5; Ezio, Brash Novice 48→12; Gimli, Mournful Avenger 432→16; Grist, the Hunger Tide 6336→1584; Iname, Life Aspect 8→4; Killmonger, Scourge of Wakanda 24→6; Lightning, Security Sergeant 72→18; Multani, Yavimaya's Avatar 14→7; Quicksilver, Speedster 16→8; Rat King, Verminister 384→192; Ty Lee, Chi Blocker 48→12; Wolverine, Claws Out 96→32. These are trunk changes, not removals by this feature. Bounded licence fix `znsuzsus`, compared with `vmksqonr` using official `cargo xtask english-v3 diff`: all 3,107 faces unchanged, 1,577 covered / 142,793 Readings before and after; lost/gained faces, decreases/increases, digest-only changes, validation issues and internal failures all zero. Master Biomancer remains 96→96; the negative “This creature enters during your turn and as a Mutant.” probe is 1 (`lkmolrsy` parent grammar) → 0 (`znsuzsus`).
2. **Witnesses.** All five failed their pre-implementation `cargo xtask english-v3 probe` with zero Readings (`mqrlzszk`); after (`voyrmlnk`): Essence of the Wild 1; Mirror Image 2; Vesuva 6; Body Double 7; Masterwork of Ingenuity 12. All probes completed without issues. The same five census counts remain at `vmksqonr`.
3. **Spot checks.** Exhaustive probes at `voyrmlnk` select Enter's nominal predicative PP Complement, with *copy* taking an *of* PP: Essence of the Wild 1 (*this creature*); Mirror Image 2 (*a creature you control*); Vesuva 6 (*any land*, with *tapped* depictive); Body Double 7 (*any creature card in a graveyard*); Masterwork of Ingenuity 12 (*any Equipment*); Clone 6 (*any creature*); Clever Impersonator 7 (*any nonland permanent*); Jwari Shapeshifter 7 (*any Ally creature*); Copy Enchantment 6 (*any enchantment*); Sculpting Steel 6 (*any artifact*). Location PPs retain all licensed attachment readings; the copy PP remains selected in every Reading. All ten probes completed without issues (`voyrmlnk`).
4. **Deviations and STOPs.** The new frame selects nominal predicative PPs generally, as the Goal specifies: attested extra gains Master Biomancer (*as a Mutant*) and Teferi's Time Twist (*as a creature*) were checked. The depictive schema handles Vesuva. The existing nominal-as construction and lexical licence were updated together, retaining preposed use and excluding final Adjunct use; no duplicate replacement route was added. Passing the declared clausal restriction through finite/irrealis PP constructions prevents an invalid finite-as predicative reading; Geth's Summons retains its baseline 48 Readings (`voyrmlnk`). Four initially incorrect test expectations assumed location PPs always belonged inside the copy PP; they were corrected to exact surface sets preserving all licensed attachments. The bounded census follows the binding orchestrator override of 2026-10-08. Refresh's registration conflict retained both this module and the upstream name-scope module. No STOPs in the original landing, glossary gaps, lexeme-naming guards or new CGEL claims. Follow-up review found the unlike-function licensing bypass. The per-coordinate host-frame check STOP was resolved by the owner’s bounded ruling of 2026-10-08: the coordinated predicative slot is gated by the coordinates’ own licences (VerbComplement on the None-restricted coordinate; PredicativeComplement on the nominal-as coordinate), not by Enter’s specific frames. A full per-coordinate frame check needs a coordination mechanism that keeps each coordinate’s frame signature for the shared head, routed to recon as [“unlike-function complement coordination licensing”](../planned/english-v3-systemic-residuals.md#unlike-function-complement-coordination-licensing-2026-10-08). Feature-table rows gained wildcard and set-membership patterns to read those declared licences without lexeme guards; no frame-licensing mechanism was built. Positive Master Biomancer and negative adjunct-only coordination tests were added (`znsuzsus`), with exhaustive issue-free probes confirming 96 and 0 respectively. The previously invalid Reading is retired; no corpus Reading is removed.
5. **Tests.** Initial `cargo test -p deckmaste_english_v3`: 524 passed, four new/re-spelled test expectation failures, subsequently corrected. At `voyrmlnk`, `cargo test -p deckmaste_english_v3 --test main` with the following filters passed: `enter_as_copy_complement::` (8), `focusing_only::` (12), `as_though_content_clauses::` (10), `fixed_cost_phrases::` (10); English doc tests passed. `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings` passed (`voyrmlnk`). Pre-refresh `cargo test -p deckmaste_lexical_source`: 23 passed, inherited `card_names::tests::generated_nicknames_equal_the_streamed_computation` failed, owned by `english-v3-name-lexeme-scope`. After refresh at `vmksqonr`, `cargo test -p deckmaste_english_v3 --test main` passed with filters `enter_as_copy_complement::` (8), `focusing_only::` (12), `name_lexeme_scope::` (4); `cargo test -p deckmaste_lexical_source` passed (59, including the inherited catalog test); the same two-crate clippy command passed. No full English suite was repeated after refresh. Assurance counts at `voyrmlnk`: restored 0, re-spelled 1 (the existing selected/comparative-as control), ignored with blocker 0, added 8, removed 0. No xtask tests or Lean checks. Licence follow-up `znsuzsus`: `cargo test -p deckmaste_english_v3` passed (540 main tests); `cargo test -p deckmaste_construction_v3_core` passed (28); `cargo clippy -p deckmaste_english_v3 -p deckmaste_construction_v3_core --tests -- -D warnings` passed. Follow-up assurance counts (`znsuzsus`): restored 0, re-spelled 0, ignored with blocker 0, added 3 (two English regressions and one compiler validation test), removed 0.
6. **Timing.** One finished-feature bounded census: 25.143 s, 12 workers (`voyrmlnk`). Identity manifest: 3,107 faces, comprising every supported face containing the word *as* (2,823), plus 300 random covered faces, seed 20261008, with 16 overlapping identities (selector measured on `mqrlzszk`). `cargo xtask english-v3 --identity-manifest target/english-v3/english-v3-enter-as-copy-complement/manifest.json --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-enter-as-copy-complement/after.json`; official `cargo xtask english-v3 diff BEFORE.json AFTER.json --json …/diff.json`. Evidence stays under `target/english-v3/english-v3-enter-as-copy-complement/`. Refresh changed grammar dependencies, so the same bounded manifest and only affected crate tests were rechecked; no full corpus census was run. Licence follow-up `znsuzsus`: the same 3,107-face manifest, 12 workers, zero samples per face, took 17.306 s; evidence is under `target/english-v3/entercopy-licence-fix/`.
