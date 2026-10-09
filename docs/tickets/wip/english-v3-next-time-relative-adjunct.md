---
needs: []
---
# Read a preposed temporal NP Adjunct: The next time you would draw a card this turn, …

## Why

No NP reads as a preposed Adjunct. On census `qsupksky` (32,828 supported
faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon
bucket "the next time" touches **80** unread faces and is the sole cause on
**71** (touched / sole as in the wave-5 recon; surface counts, not gain
forecasts). `cargo xtask english-v3 grep '\b[Tt]he next time\b'` finds 80
faces; every match is clause-initial (after an ability cost, a sentence
boundary or a preposed *if* clause). The same position holds *This turn, …*
(7 faces, e.g. Combine Guildmage, War Cadence) and *When the last time …* (2).

Probes (`--readings 1`): "This turn, draw a card." 0, "Draw a card this
turn." 1; "Prevent that damage the next time it would be dealt this turn."
0 (postposed; unattested).

## Goal

A temporal NP Adjunct (*the next time [relative clause]*, *this turn*)
reads before a Clause, with the comma, as a preposed Adjunct, and *the next
time* takes its relative clause.

## Analysis

CGEL Ch. 8 §6.3, p. 698 (temporal location NPs): *next* and *last* occur
widely as dependents of the temporal nouns, and "*The* occurs only in
combination with another dependent: … *the night we first met*". Ch. 12
§3, p. 1045 [31ii] (*the day [we met Kim at the races —]*: the gap functions
as Adjunct in the relative) and p. 1053 [60i] (*the day (that) Kim was
born*). Those back the NP and its relative only, not the preposed position.

Existing routes: `NominalAdjunctPhrase`
(`crates/deckmaste_english_v3/src/declarations.rs:3707`,
`[determiner: lexical(Determinative), head: Nominal]`, licensed by the
`nominal_adjunct_license` table at `:3694` over the declared
`NominalAdjunctDeterminer` × `NominalAdjunctClass`), attached post-verbally
only through the `NominalAdjunctPredicate` schema (`:1579`).
`lexeme:CommonNoun/Time` (`lexicon/core.ron:417`) declares no
`NominalAdjunctClass`; *step* and *turn* are `Temporal` (`core.ron:134–135`).
*the* (`vocab:DefiniteMarker/The`, `core.ron:178`) has no
`NominalAdjunctDeterminer`; *next* is `vocab:AttributiveAdjective/Next`
(`lexicon/vocabulary.rs:111`). The preposed-PP route is `InitialPreposition`
(`declarations.rs:2346`, `[dependent: PrepositionPhrase, ", ", clause:
Clause]`, licensed by the declared `PreposedAdjunct` function).

Orchestrator rulings 2026-10-09:

- R11a. Preposed position by refining the existing route: generalise
  `InitialPreposition` (or its schema) to admit a `NominalAdjunctPhrase`
  dependent, licensed by a declared feature; no second, parallel
  preposed-Adjunct Construction.
- R11b. Declare `NominalAdjunctClass: Temporal` on *time* (data). *the* is
  licensed in a temporal NP Adjunct only when the Nominal carries another
  dependent (CGEL p. 698); express that with a declared or exported feature
  (e.g. a modified-Nominal feature), never by naming *the*, *next* or *time*.
- R11c. The relative clause on *time* is the existing relative-clause route
  with an Adjunct gap (CGEL p. 1045); no fused or conjunction analysis of
  *the next time*.
- R11d. Pruning rule: postposed *… the next time it would …* is unattested;
  do not license it. *each time* is attested preposed on 1 face only and
  mostly as *for each time* (40 faces, a PP Complement); it is out of scope
  unless it falls out of R11a with no extra declaration.
- R11e. *This turn, …* reads through the same generalised route as a
  by-product; record those faces under spot checks, do not count them as a
  separate construction.

Implementation update (user, 2026-10-09): trunk `otzuyrluxpsw` already supplies
`AdverbialGapRelativeClause`, a complete FiniteClause relative on a head declaring
`RelativisableAdjunctClass: Temporal`. Reuse that route for R11c; no relative
Construction is added. The claim-parent probe reads *the first time you cast a
spell each turn* as a NounPhrase (16 Readings); it is a green control, not a
remaining witness.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0, control 1. (D) =
`--category Document`.

- Dazzling Reflection: "The next time that creature would deal damage this
  turn, prevent that damage." (control "If that creature would deal damage
  this turn, prevent that damage.")
- Words of Waste (D): "{1}: The next time you would draw a card this turn,
  each opponent discards a card instead." (control "{1}: If you would draw a
  card this turn, …")
- New Way Forward: "The next time a source of your choice would deal damage
  to you this turn, prevent that damage." (control "If a source of your
  choice would …")
- Circle of Protection: Red (D): "{1}: The next time a red source of your
  choice would deal damage to you this turn, prevent that damage." (control
  "{1}: If a red source of your choice would …")
- Words of Wilding (D): "{1}: The next time you would draw a card this turn,
  create a 2/2 green Bear creature token instead." (control "{1}: If you
  would draw a card this turn, …")

## Method

Standard constraints apply.
Deltas:

1. Verification budget (user update 2026-10-09): a bounded census, never
   `--all`. Select every supported face matching case-insensitive
   `\b(the (next|first|second|third|last) time|this turn,)` in analysed text,
   plus 300 random covered controls (seed 20261008) chosen by the official
   `cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`.
   Include the additional demonstrative and distributive *time* faces the
   declared temporal licence reaches. Run the prebuilt binary with the identity
   manifest, `--workers 6 --samples-per-face 0`, under `/usr/bin/time -v`, and
   compare against the claim-parent bounded census with the official `diff`
   (schema v8, face counts sum line counts; `--lines` attributes changes).
   Tests and clippy cover each touched crate; no xtask tests or Lean checks.
   Evidence stays under `target/english-v3/english-v3-next-time-relative-adjunct/`.
   After the six-item record and `jj --no-pager commit`, refresh and stop for
   orchestrator review. Re-check only the bounded census and full suites of
   touched crates if refresh changes grammar, lexicon or construction core.
2. Probe first: probe each witness and control before implementation and
   record the counts; write each witness as a test. Run the tests and clippy
   once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified
   in the landing record; a decrease inherited from trunk is attributed by
   change id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *the next time …* read as a Subject or as anything but a preposed temporal Adjunct with its relative clause, or *the* licensed on a bare *time*, is a defect. A wrong analysis that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit lexicon declarations in `crates/deckmaste_lexical_source/lexicon/`
   and `grammar:` frame fields in `plugins_v2` keyword actions; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule); check attestation with `cargo xtask english-v3 grep`.
   Data over code: a declaration an existing Construction already reads is
   preferred to a grammar change.
8. Refine an existing Construction before adding one: the Construction count
   is above the ADR's 250 ceiling. A new frame or Construction must not overlap
   an existing one on the same string (two labels for one constituency is a
   spurious duplicate, not an ambiguity); when it supersedes one, retire the
   old route and re-spell its tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.

## Out of scope

- *for each time …* (PP Complement) and *last turn* (a determiner-less
  temporal NP: `english-v3-last-turn-temporal-adjunct`).
- Postposed *the next time …* (unattested).

## Landing record

1. **Census delta** (pre-refresh `rxwmpvru`, before on its empty claim-parent tree
   `szxrxoqk`, after on the implementation): bounded manifest **1176 faces**;
   covered **809 → 883**, Readings **104760 → 111582**. Official
   `diff --lines`: gained **74**, lost **0** (none), decreases **0**, increases
   **0**, digest-only **0**, unchanged **1102**; **111** changed lines.
   Selector: case-insensitive `\b(the (next|first|second|third|last) time|this turn,)`
   (**840** faces), plus the additional demonstrative/distributive *time*
   surface (**46** additional faces), plus **300** random covered controls,
   seed **20261008** (overlaps removed). Refreshed `rxwmpvru`: covered
   **884**, Readings **111613**; claim-parent diff gained **75**, lost **0**,
   decreases **0**, increases **0**, digest-only **330**, unchanged **771**.
   Refresh-only diff: gained **1** (Sentinel Sarah Lyons), digest-only **338**,
   lost/decreases/increases **0**; these are inherited from `ouzqotlxvsmv`
   (the bounding cardinal licence and optional modifier representation). Evidence: `target/english-v3/english-v3-next-time-relative-adjunct/{manifest,before,after,diff}.json`.
   Review correction (`stymwqqo`): all **1176** faces unchanged against the
   reviewed census (`review-delta.json`): lost/gained/decreases/increases/
   digest-only **0**, covered **884**, Readings **111613**. Decreased faces:
   none. Claim-parent `review-diff.json` retains the **75** gains and the
   **330** inherited digest-only changes above.
2. **Witnesses** (pre-refresh `rxwmpvru`, complete probes): Dazzling Reflection **12**;
   Words of Waste **24**; New Way Forward **20**; Circle of Protection: Red
   **40**; Words of Wilding **24**. All five were **0** before implementation;
   their *If* controls each admitted **1** at the probe cap. The inherited
   *the first time you cast a spell each turn* NP already admitted **16** and
   was excluded from the red list. Witness units differ from whole-face totals.
3. **Spot checks** (pre-refresh `rxwmpvru`): Dazzling Reflection, Words of Waste, New Way
   Forward, Circle of Protection: Red, Words of Wilding, Aegis of Honor,
   Awe Strike, Circle of Protection: Blue, Jade Monolith, and Mystic Reflection.
   Each selected whole-face Reading has a preposed temporal Nominal Adjunct
   headed by *time*, with an Adverbial Gap Relative Clause containing a complete
   Finite Clause; none treats that NP as the Subject. Each realizes byte-exactly
   with no probe issues. Probe trees are `spot-full-*.json` in the evidence directory.
4. **Deviations and STOPs** (`rxwmpvru`): no new or retired Construction,
   frame or lexeme; reuse the relative route from `otzuyrluxpsw`. Extend the
   existing InitialPreposition with a temporal NP form. This required generic
   per-form Categories for single-child fields in the construction core;
   cardinality, feature reachability and shared optional/repeated projections
   remain checked. Two core regression tests accompany that support. The
   selected-frame validator was factored out to satisfy clippy. No STOPs.
   Refresh conflicted only on test-module registrations; both suites were
   retained. Glossary gaps filled: Nominal Dependent Present, Nominal Adjunct Class,
   Nominal Adjunct Determiner. No superseded route remains to retire.
   Review ruling: the general licence also admits unattested preposed
   *Each turn, …*, *The next turn, …*, *That time, …*, *Each time, …*, and
   postposed *each/this/that time* without another declaration; R11d permits
   these forms that fall out freely. The mass-*time* twin under *the* doubles
   Readings on the gained faces and is inherited from trunk's NounPhrase;
   this remains a follow-up question for the orchestrator, unchanged here.
   Review correction (`stymwqqo`): NominalAdjunctPhrase now uses the determined NP's
   determiner–countability licence, removing the inadmissible mass analysis
   of *each time*. Its complete probe went **2 → 1** Readings; one additional
   regression test asserts the single Reading.
5. **Tests** (pre-refresh `rxwmpvru`): `cargo test -p deckmaste_english_v3` passed
   **669** tests; `cargo test -p deckmaste_lexical_source` passed **63**;
   `cargo test -p deckmaste_construction_v3_core` passed **32**. The checks
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`, and
   `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings` passed. English's full suite
   was rerun after compiler/clippy cleanup. Restored **0**, re-spelled **0**,
   ignored **0**, added **10** (**8** English, **2** core), removed **0**;
   no removals to justify. Citation checks: no noncompliant citations, **0**
   stale; piped `jj --no-pager diff --git | cargo xtask cite audit --diff`
   passed with no new citation sites. After refresh (`rxwmpvru`), the same
   full test commands passed **684**, **64**, and **32** tests respectively,
   and all three exact clippy commands passed again. The refreshed citation
   checks also pass; the elapsed-time figure is spelled as minutes and seconds
   to avoid the citation scanner interpreting it as a rule number.
   After the review correction (`stymwqqo`), those same full test commands
   passed **685**, **64**, and **32** tests respectively; all three clippy
   commands and the citation checks passed. The new test's initial build
   needed a BTreeSet accessor correction before the suite ran. Final feature
   counts: restored **0**, re-spelled **0**, ignored **0**, added **11**
   (**9** English, **2** core), removed **0**; no removals to justify.
6. **Timing** (pre-refresh `rxwmpvru`): the bounded landing census took **46.21 s**,
   **6 workers**, peak RSS **8173084 kB**, measured with `/usr/bin/time -v`
   on prebuilt `target/debug/cargo-xtask`, identity manifest and
   `--samples-per-face 0`. Refresh-only bounded recheck (`rxwmpvru`):
   **1 min 54.11 s**, **6 workers**, peak RSS **8188948 kB**. Review correction
   recheck (`stymwqqo`): **43.43 s**, **6 workers**, peak RSS **7972736 kB**.
   No full corpus run.
