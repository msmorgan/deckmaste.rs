---
needs: []
---
# Coordinate focused Adjuncts: Activate only as a sorcery and only once each turn.

## Why

A single focused Adjunct reads (*Activate only as a sorcery.*, *Activate only
once each turn.*); two of them joined by *and* or *but* do not. Named as a
residue in `english-v3-systemic-residuals` ("Once-frequency residue"). Blocker
report at trunk `ymqwzour` (`target/english-v3/blockers-wave8.json`): hidden
under `at:turn .` (about 25 of 43), `at:sorcery .` (14 of 16), `at:step .`,
`at:graveyard .`, `at:once .`. `cargo xtask english-v3 grep '\b(and|but) only\b'`
finds **82** faces, **57** unread; 57 failing lines match, **54** of them the
only failing line of their face (*and only* 45, *but only* 9). The 25 covered
faces are all *only during … and only if …* (Defiant Stand, Hakim,
Loreweaver). Sole lines by coordinate pair: *if* + *as a sorcery* 10, *during*
+ *once* 7, *if* + *once* 6, *as* + *once* 6, *as* + *if* 6, *once* + *if* 4,
others 3. So one order of one pair reads and the rest do not.

## Goal

A coordination of focused Adjuncts, of like or unlike category, reads as one
Adjunct of the clause: *only* PP + *and only* AdvP / PP / *if*-PP, and *but
only* PP after a complete clause, with byte-exact realisation.

## Analysis

CGEL Ch. 15 §3.2, p. 1326: "a difference of category is generally tolerated
where there is likeness of function". That backs coordinating a PP (*only as
a sorcery*) with an AdvP (*only once each turn*) as Adjuncts. Ch. 6 §7.3.1,
p. 587, [41]–[42] (cited by `FocusedPhrase`) backs each focused coordinate.
Treating *but only …* after a clause without its own *only* as the same
coordination is a project ruling.

Existing route (`crates/deckmaste_english_v3/src/declarations.rs`): schema
`FocusedPhrase` with instances for `NounPhrase`, `PurposeInfinitival`,
`PrepositionPhrase` and `AdverbPhrase`, each exporting `FocusedUse = Yes`;
the coordination schemas already carry `FocusedUse` through `AdverbPhraseSeries`
and the PP series, and "Activate during your turn and during your upkeep."
reads 1. What is missing is a coordination whose coordinates are focused and
may differ in category.

Orchestrator rulings 2026-10-10:

- CF1. First identify the route that reads the 25 covered *only during … and
  only if …* faces, and record its tree. If it is a coordination of focused
  Adjuncts, generalise it (order, AdvP coordinate, *as*-PP coordinate); no new
  Construction. If it is some other analysis (the *if* PP attached elsewhere,
  or *and only if* as a clause opener), STOP with both trees before adding a
  coordination beside it: which one those faces keep is a ruling.
- CF1a. The licence reads `FocusedUse` on each coordinate (both `Yes`) and the
  coordinates' Adjunct function licences. No test on *only*, *once* or
  *sorcery*.
- CF2. One analysis per string. *Activate only as a sorcery and only once each
  turn.* is one clause with a coordinated Adjunct. It must not also read as a
  clause coordination with an elided second *activate*, nor with the second
  *only* focusing into the first coordinate.
- CF3. *X but only Y* (*Any player may activate this ability but only as a
  sorcery.*, 9 sole faces): the first coordinate is the clause without a
  focused Adjunct. In scope if the same coordination reads it with *but* and a
  single focused coordinate attached to the clause; if it needs a distinct
  construction, list it with the count and leave it.
- CF4. Three coordinates with commas (*only during the declare blockers step,
  only if …, and only once each turn*, Grizzled Wolverine) follow from the
  series schema or stay out; do not add a construction for one face.
- CF5. Unfocused first coordinate with *and* ("Activate only as a sorcery and
  once each turn." 0) is unattested: no work, and no check to block it.
- CF6. Newly covered faces are gains. Reading-count increases are possible
  only on the 25 covered `(and|but) only` faces, as a second analysis of the
  same string; under CF1 that is a STOP, not a result to report after the
  fact. Any other increase is a defect.

- CF7 (orchestrator, 2026-10-10). Retain Reading 1; retire
  `only [during … and only if …]` as a wrong analysis. A coordination
  exports Focused Use when any coordinate is focused; FocusedPhrase cannot
  focus such a coordination. Unfocused coordinations under one focusing
  modifier retain their Reading. Every decrease must be exactly the retired
  shape, one Reading per affected line; report count classes and restore any
  other decrease.
- CF8 (orchestrator, 2026-10-10). ONE Adjunct-member projection Construction
  is permitted, with no compiler change. It checks Focused Use and the
  member's Adjunct function licence. All focused-Adjunct coordinations use
  this common member route; the old same-category route cannot also build
  them. Disclose the Construction and count against the 250 ceiling. CF7
  stands.
- W9-A (standing ruling). One analysis per string means one route for the
  ticket's construction. Independent ambiguity also present in the control
  stays admitted and is reported beside its control's count.

## Witnesses

Each is the last sentence of the face's only failing line at `ymqwzour`.
Probed 2026-10-10 with the `ymqwzour` binary (`--category sentence --readings
3`): all 0. Re-probe on the claim parent first. Not probed (the binary stopped
loading plugin data when trunk moved): the covered order "Activate only during
your upkeep and only if Hakim isn't enchanted." (census: its face is covered)
and the reversed orders; probe them before designing.

- Thunderhead Gunner: "Activate only as a sorcery and only once each turn."
- Instill Energy: "Activate only during your turn and only once each turn."
- Temple of Cyclical Time: "Activate only if it has no time counters on it and
  only as a sorcery."
- Security Detail: "Activate only if you control no creatures and only once
  each turn."
- Scandalmonger: "Any player may activate this ability but only as a sorcery."

Also 0 today: "Only your opponents may activate this ability and only as a
sorcery." (Detention Vortex); "Any player may activate this ability but only
during their draw step." Controls that must not change: "Activate only as a
sorcery." 1; "Activate only once each turn." 1; "Activate only if you control
no creatures." 1; "Activate during your turn and during your upkeep." 1; "Any
player may activate this ability." 1; "Only your opponents may activate this
ability." 1.

Claim-parent re-probes (`wrxuwlwq`, 2026-10-10) completed without issues:
the seven failing witnesses remain 0 and the six controls remain 1. Hakim's
covered and reversed orders each have 2: coordinated focused PPs, and the
wrong outer-focus shape retired under CF7. Reversed Thunderhead, Instill,
and Security orders remain 0. Evidence:
`target/english-v3/english-v3-coordinated-focused-adjuncts/before-*.json`
and `baseline-probe-summary.json`. CF1's tree STOP was resolved by CF7;
the compiler's fixed schema Category binding STOP was resolved by CF8.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bonly\b` case-insensitively (the surface the change can touch;
1,473 faces) plus 300 random covered controls (`cargo xtask english-v3
diff --manifest … --random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against the
claim parent. Zero lost; CF6 for increases. Full `cargo test -p
deckmaste_english_v3` and `cargo clippy -p deckmaste_english_v3 --tests -- -D
warnings`, once, at the end: a rebuild after a grammar edit takes 2–3 minutes
and the six test binaries about 75 seconds, so batch edits. Tests assert the
role (each focused phrase a coordinate, the coordination an Adjunct of the
one clause, the CF2 negatives), never an incidental count or a Debug-printed
tree. Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *Activate only during your turn, before attackers are declared.*
  (`english-v3-supplementary-temporal-adjunct`).
- *only any time they could cast a sorcery* (Teferi, 2 faces): temporal NP
  focus.
- *only if* + clause content that fails for its own reason (*seven or more
  cards in your graveyard* reads; *you've cast three or more instant and/or
  sorcery spells* on Urabrask may not): report, do not chase.

## Landing record

1. **Census delta** (`qktrquum`, original claim parent `wrxuwlwq`):
   1,763 faces; initial 1,491 → 1,528 covered / 29,555 → 30,010 Readings;
   37 ticket gains, 25 decreased faces, zero losses / increases / digest-only
   changes / issues / wrong-only gains found. Selector: all 1,473 supported
   case-insensitive word-boundary `only` faces plus 300 covered controls,
   seed 20261008, ten overlaps. All 300 are covered on the claim parent;
   selector includes CF7's same-sentence surface and the ticket's pattern.
   Decreases by affected **line** count, all the retired outer-focus route:
   **2→1 (8)**: Arcum's Sleigh, Ashen Ghoul, Coffin Puppets, Essence Anchor,
   Gutterbones, Hakim, Loreweaver, Kjeldoran Guard, Nemesis Phoenix;
   **4→2 (1)**: Kuldotha Phoenix;
   **6→3 (15)**: Assassin's Blade, Champion's Victory, Command of Unsummoning,
   Deep Wood, Defiant Stand, Eightfold Maze, Harsh Justice, Heavy Fog, Just
   Fate, Kongming's Contraptions, Rally the Troops, Remove, Scorching Winds,
   Treetop Defense, Warrior's Stand;
   **14→7 (1)**: Defenders of Humanity. Exactly 62 retired Readings; another
   decreased shape: none. Final refreshed subset (`vyopwkts`): **1,563
   covered / 30,182 Readings**, **72 gains (37 ticket + 35 trunk)**,
   the same 25 decreases, zero losses / increases / issues, nine digest-only
   changes. Official refresh-only diffs attribute trunk gains to
   `roomopmrpkys` (The Moment, The Pandorica), `tlpwnxqnnquu` (Minsc, Beloved
   Ranger), `wkooyukozlpw` (Powerful Broker), `mpvousxzotns` (26 faces),
   `mzkvyunrqtrw` (Grim Repriser), and `ztrnpkyvzntv` (Satsuki, the Living
   Lore). `rtzolrtxrrru` changes no selected face or line. The nine modal
   header digest-only changes belong to `ztrnpkyvzntv`: Arbalest Engineers
   (1), Feast or Famine (2), Light of Hope (1), Open the Omenpaths (7),
   Professor's Warning (1), Savage Beating (6), Shriveling Rot (2), Stand or
   Fall (9), Vannifar, Evolved Enigma (4); counts unchanged. No refresh
   decrease. The final progressive-gap / die refresh (`opxvnokryrro`,
   `vlxrulqwmzzk`) adds False Orders (21), Wall of Shadows (13), and Ydwen
   Efreet (30), with no further losses / decreases / increases / digests.
   The final verbless-while refresh `wxwozrtzrsqp` changes no selected face
   or line. Evidence directory:
   `target/english-v3/english-v3-coordinated-focused-adjuncts/`;
   initial `{manifest,before,after,diff}.json`, final
   `after-retry7-refresh.json` / `diff-retry7-refresh.json`, and all official
   refresh-only diff reports retained there.
2. **Witnesses** (`qktrquum`, complete sentence probes, zero issues):
   Thunderhead Gunner 1, Instill Energy 1, Temple of Cyclical Time 6,
   Security Detail 1; Scandalmonger, Detention Vortex and draw-step *but only*
   witness 0. Hakim and reversed Hakim 2→1; reversed Thunderhead, Instill and
   Security 0→1 each. Six original controls remain 1 each. Temple's single
   Adjunct control also has 6 (three `time counters` analyses × two `on it`
   attachments), preserved under W9-A. Grizzled Wolverine serial restriction
   1. Attacked-step control 3; Kuldotha conditional sentence control 1 /
   complete activated-line control 2; Defenders' single-Adjunct activated
   control 7; Hall of Oracles single-Adjunct control 4. BEFORE probes show
   all seven initially failing witnesses at 0 and original controls at 1.
3. **Spot checks** (`qktrquum`, ten newly covered faces): Thunderhead Gunner,
   Bound by Moonsilver and Nimbleclaw Adept: focused predicative PP + bounded
   frequency AdvP; Instill Energy and Gate to Phyrexia: temporal PP + bounded
   frequency AdvP; Security Detail: conditional PP + bounded frequency AdvP;
   Temple of Cyclical Time: conditional + predicative PPs (six independent
   control Readings); In the Trenches: predicative PP + unbounded frequency
   AdvP; Hall of Oracles: predicative + conditional PPs (four independent
   control Readings); Grizzled Wolverine: three focused members through the
   serial schema. Each coordination is one predicate Adjunct; each member
   retains its own focus and underlying Category.
4. **Deviations and STOPs** (`qktrquum`): CF1's tree STOP resolved by CF7;
   fixed schema-binding STOP resolved by CF8. Added ONE Construction,
   `FocusedAdjunctMember` (two forms: PP, AdvP); also 3 Categories
   (FocusedAdjunctMember, FocusedAdjunctSeries, FocusedAdjunctCoordination),
   5 instances of existing schemas (NominalAdjunctPredicate, Coordination,
   CoordinationSeriesEnd, CoordinationSeriesContinuation, SerialCoordination),
   2 policies, 2 tables, 1 feature (accepted under CF8 by the orchestrator).
   The roughly 70 `export NominalAdjunctHost = Yes` lines restate the default
   (hygiene candidate: drop them if the compiler does not need them);
   NominalAdjunctPredicate's `nonconnective_trailing_adjunct_host` requirement
   and `ClauseComplementBoundary = None` export moved unchanged into its
   existing five-host instance. Side effects, unattested: NP coordinations
   also lose the outer-focus Reading (CF7 union; "You may cast only creature
   spells and only artifact spells." 2 → 1); `or` coordinates focused Adjuncts
   (`\bor only\b`: 0 faces); a PP may stack after the coordination
   ("… and only once each turn during your upkeep." reads 1; attestation
   unchecked). Residual: CF3 *X but only Y* (nine faces and Detention Vortex)
   still reads 0; the test `clause_plus_focused_phrase_requires_its_own_route`
   pins that residual and must be re-spelled by the follow-up.
   All focused Adjunct coordinations use the projection; old same-category
   PP/AdvP paths reject two focused coordinates, mixed focused/unfocused
   paths remain. Focused Use propagates by union. NominalAdjunctHost keeps
   the right frequency member's temporal restriction inside that member.
   No compiler or lexicon code/data added by this ticket; no word guard.
   Construction count at initial census: 308 (241 ordinary + 67 schemas),
   original claim parent 307, ticket net +1, eventual ceiling 250. Final
   count: **311** (244 ordinary + 67 schemas), including one additional
   Construction each from `tlpwnxqnnquu`, `ztrnpkyvzntv`, and
   `opxvnokryrro`. Glossary adds
   Focused Adjunct Member / Nominal Adjunct Host and clarifies Focused Use.
   CF7's one-Reading-per-line wording is resolved by standing ruling W9-A:
   the single retired outer-focus route is replicated by independent ambiguity
   that the kept route also has. Reviewer enumeration (Defiant Stand 6→3,
   Arcum's Sleigh 2→1, Kuldotha Phoenix 4→2, Defenders of Humanity 14→7):
   the removed Readings are exactly the `only [X and only Y]` half, and the
   kept set is identical modulo the member wrapper. Orchestrator review
   2026-10-10: ACCEPTED. Named-token refresh conflict resolved by retaining
   both NounPhraseStructure additions and NominalAdjunctHost / FocusedUse
   exports. Final progressive-gap conflict resolved by preserving the three
   existing gap Categories' NominalAdjunctHost feature and the new trunk
   GerundParticipleObjectGap Category unchanged. Kata snapshot races retried.
5. **Tests** (`qktrquum`; final verification on `vyopwkts`):
   `cargo build -p xtask` passed;
   `cargo test -p deckmaste_english_v3` **951 passed**, zero failures / ignores;
   `cargo test -p deckmaste_lexical_source` **86 passed**, zero failures / ignores;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Required full suites and clippy repeated after every code/data refresh;
   unchanged lexical-source results retained on grammar-only refreshes.
   `cargo test -p deckmaste_english_v3 --test suite_2 coordinated_focused_adjuncts::`
   passed 9 tests after the helper's clippy `let … else` fix. Initial full
   suite passed 905 after re-spelling Ashen Ghoul's deliberately retired
   two-Reading expectation as one; exact comparison, card, text and role
   remain. Defiant Stand is re-spelled for the common coordination Category.
   Assurance: restored 0 / re-spelled 2 / ignored 0 / added 9 / removed 0
   (removals: none). Noncombat refresh full-suite subtotal is 938, correcting
   the earlier 934 arithmetic error. Citation checks: zero non-compliant /
   stale; diff audit selected zero sites, no new CR rules to bless.
   `conflicts()` empty after repair.
6. **Timing** (`qktrquum`): initial bounded AFTER **112.79 s** under
   `/usr/bin/time -v`, **6 workers**, **0 samples per face**. Final verbless-while
   retry (`vyopwkts`): **587.04 s** command elapsed (including Cargo lock
   wait), **46.00 s** corpus wall, identical selector / workers / samples,
   zero issues. Earlier bounded refresh timings remain in evidence logs.
   No `--all` census or Lean check was run.
