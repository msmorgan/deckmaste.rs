---
needs: []
---
# Read asyndetic premodifier coordination: Destroy target nonartifact, nonblack creature.

## Why

*nonartifact and nonblack creature* reads; *nonartifact, nonblack creature*
does not. On census `qsupksky` (32,828 supported faces, 25,474 covered, 7,354
unread; wave-5 recon of 2026-10-08), the recon bucket "comma-stacked negative
premodifiers" touches **83** unread faces and is the sole cause on **64**
(touched / sole as in the wave-5 recon; surface counts, not gain forecasts).
`cargo xtask english-v3 grep -i '\bnon-?[A-Za-z]+, non'` finds 83 faces
(*noncreature, nonland*; *nonartifact, nonblack*; *non-Vampire, non-Werewolf,
non-Zombie*; *nonland, nontoken*).

Probes (`--readings 1`): "Destroy target nonartifact, nonblack creature." 0;
"… nonartifact and nonblack creature." 1; "… nonartifact nonblack
creature." 1; "Destroy target red, green creature." 0.

## Goal

Two or more premodifiers joined by commas alone, before one head, read as an
asyndetic coordination of Premodifiers by function, sharing one head:
`[[P1, P2(, P3)] Head]` (orchestrator ruling R15e).

## Analysis

CGEL Ch. 15 §1, p. 1276, [4]: coordination may be syndetic (*all his
colleagues and all his students*) or asyndetic (*all his colleagues, all his
students*); p. 1277 notes that asyndetic coordination is not always clearly
distinguishable from non-coordinative juxtaposition. That backs the
asyndetic analysis only.

Existing route: the `Coordination` schema
(`crates/deckmaste_english_v3/src/declarations.rs:1498`, separator by the
declared `CommaCoordination` feature, `:1495`) and its instance list
(`:2581`), whose rows include `NounPremodifier` and `AdjectivePhrase`;
`SerialCoordination` (`:1491`, `:2461`) and `CoordinationSeriesEnd`
(`:1482`) for longer series. Consumers: `NounPremodifiedNominal` (`:2618`)
and `PremodifiedNominal` (`:2632`). *nonartifact* lexes as the negative type
word (`lexeme:type/artifact/non`), *nonblack* as the negative colour word
(`vocab:ColorWord/Black/non`), so the conjuncts may be unlike categories,
under R15e; the existing syndetic Reading coordinates Nominals instead.

Orchestrator rulings 2026-10-09:

- R15a. Extend the existing coordination schemas with a coordinator-less
  (asyndetic) form; do not add a stacked-premodifier-with-comma Construction.
  The asyndetic Reading has the same members and grouping as the syndetic
  one minus the coordinator.
- R15b. Pruning rule: open the asyndetic form only for the instance rows
  whose asyndetic use is attested in the supported corpus (measure with
  `cargo xtask english-v3 grep`; premodifier rows). Asyndetic NP, clause,
  predicate or PP coordination stays closed unless attested *and* measured
  at zero loss; *red, green creature* is unattested and must stay 0.
- R15c. One analysis per string: *nonartifact, nonblack creature* must not
  read both as an asyndetic coordination and as two stacked premodifiers
  separated by a comma (two labels for one constituency).
- R15d. Comma lists that end in a coordinator (*white, blue, or black*)
  keep their existing `CoordinationSeriesEnd` Reading count.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0, control (the
first premodifier and its comma removed) 1.

- Terror: "Destroy target nonartifact, nonblack creature."
- Duress: "You choose a noncreature, nonland card from it."
- Druidic Satchel: "If it's a noncreature, nonland card, you gain 2 life."
- Victim of Night: "Destroy target non-Vampire, non-Werewolf, non-Zombie
  creature." (control "Destroy target non-Vampire creature.")
- Filter Out: "Return all noncreature, nonland permanents to their owners'
  hands."

## STOP: claim-parent grouping contradicts the premise (2026-10-09)

Measured on unchanged claim parent `krllwokk` (working copy `xqtkuyqk`),
using `cargo xtask english-v3 probe --readings 16 --json --as-card …`:
Terror, Duress, Druidic Satchel, Victim of Night and Filter Out witnesses
all have 0 Readings; their listed controls have 1, 4, 1, 1 and 1,
respectively. Enumeration is complete. Terror's witness was first probed
with `--readings 8`, likewise complete. No witness leaves the red list.

The syndetic control “Destroy target nonartifact and nonblack creature.”
has exactly 1 Reading, but its Target Noun Phrase contains `Coordination`
in Category `Nominal`: left `Noun(nonartifact)`, right
`PremodifiedNominal(Adjective(nonblack), Noun(creature))`.
The fragment “nonartifact and nonblack” has 0 Readings as
`NounPremodifier`. Thus the existing control does **not** demonstrate
mixed-category Premodifier Coordination sharing one head. Copying its
grouping into the asyndetic form contradicts the Goal; introducing the
shared-head grouping would add a syndetic analysis that does not currently
exist, contrary to the premise used by R15a and Method item 5.

STOP under the repository's ticket-vs-ruling contradiction rule; resolved
by orchestrator ruling R15e (2026-10-09), below.
No grammar, lexicon or tests changed. Probe evidence and the supported-corpus
negative-comma attestation are under ignored
`target/english-v3/english-v3-asyndetic-premodifier-coordination/`.

## Orchestrator ruling R15e (2026-10-09)

The syndetic premise was a recon error. Coordinates share Premodifier function,
with any category admitted by the noun and adjective premodifier consumers.
The required grouping is `[[P1, P2(, P3)] Head]`. R15a's requirement to match
the existing syndetic grouping is withdrawn. CGEL Ch. 15 §3.2, p. 1326 states
the likeness-of-function principle; p. 1328 discusses mixed pre-head modifiers.
The specific Oracle route and restrictions here are this ruling, not CGEL.

The negative syndetic counterpart is unattested (0 supported faces). Admit it
only if the same schema row supplies it without an extra declaration; otherwise
leave it closed. Preserve Terror's existing Nominal Reading. R15b no longer
requires unattested “red, green creature” to remain unread: do not add a guard
for that purpose. Asyndetic NP, clause, predicate and PP coordination remain
closed. R15c's single analysis and R15d's unchanged counts for lists ending in
a coordinator stand. Report every increase on a previously covered face with
its Reading shapes. The selector includes every supported face matching
`[a-z]+, [a-z]+ (creature|card|permanent|spell|land|artifact|enchantment|token)s?\b`.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding, user direction and R15e): one bounded
   after census of every supported face whose analysed text contains `, `
   (21,626 on claim parent `krllwokk`), plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`).
   Their identity union contains 21,748 faces and includes the negative-comma
   and widened noun-head patterns specified by R15e. Use `--workers 6`,
   `--samples-per-face 0` and a prebuilt binary under `/usr/bin/time -v`;
   compare with the claim-parent census on that same manifest using the
   existing `cargo xtask english-v3 diff`. Never run `--all`.
   Run `cargo test -p deckmaste_english_v3` and clippy once at the end.
   No xtask tests or Lean checks. After a grammar-bearing refresh, repeat
   the bounded census and the full changed-crate suites. Evidence stays in
   ignored `target/english-v3/english-v3-asyndetic-premodifier-coordination/`.
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
   Reading. A comma-separated premodifier pair read as stacked (non-coordinate) premodifiers, as NP or clause coordination, or without the shared-head grouping required by R15e, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- Asyndetic coordination outside premodifiers (unattested or unmeasured).
- Faces whose *noncreature, nonland* NP has another blocker (e.g. Blot Out
  the Sky: its control "If X is 6 or more, destroy all nonland permanents."
  also reads 0).

## Landing record

1. **Census delta**, before refresh (`xqtkuyqk`, claim-parent grammar
   `krllwokk` before):
   covered faces 16,760 → 16,822; total Readings 804,350 → 809,071.
   Gains 62; lost faces none; decreases, increases and digest-only changes 0.
   Selector: every supported analysed face containing `, ` (21,626 on
   `krllwokk`) plus 300 random covered faces, seed 20261008; identity union
   21,748 faces. This includes both R15e patterns. Evidence:
   `target/english-v3/english-v3-asyndetic-premodifier-coordination/{manifest,before,after,diff}.json`.
   After grammar/lexicon refresh (`zumztwqt`): same 21,748-face manifest,
   covered 16,760 → 16,923 and Readings 804,350 → 817,296 against the
   original parent. Gains 163; lost faces none; decreases and increases 0;
   digest-only 4. Stock `final-diff.json` and `refresh-diff.json`, both with
   `--lines`, separate the original 62 gains from 101 refresh gains.
   Refresh adds arithmetic (`lrolnmwo`, refined by `wnmmonzs`) and Produce
   (`zwkyqqyv`, refined by `wkmovwxx`). The four digest-only faces—An-Havva
   Inn, Hellfire, Invasion of Tarkir, Sidisi, Regent of the Mire—retain their
   counts: `lrolnmwo` replaces direct Measure Phrase operands with
   `ScalarMeasureOperand(MeasurePhrase)` projections in ArithmeticMeasure.
   Refresh-only losses, decreases and increases are all 0 (`zumztwqt`).
2. **Witnesses** (`xqtkuyqk`, quoted ticket fragments): Terror 1; Duress 4;
   Druidic Satchel 1; Victim of Night 1; Filter Out 1. Each was unread on
   `krllwokk`, established with `cargo xtask english-v3 probe` before editing.
3. **Spot checks** (`xqtkuyqk`): Terror and Shriekmaw
   `[[nonartifact, nonblack] creature]`; Duress and Druidic Satchel
   `[[noncreature, nonland] card]`; Victim of Night
   `[[non-Vampire, non-Werewolf, non-Zombie] creature]`; Filter Out
   `[[noncreature, nonland] permanents]`; Angelic Voices
   `[[nonartifact, nonwhite] creatures]`; Paradoxical Outcome
   `[[nonland, nontoken] permanents] you control` (both relative-attachment
   Readings retained); Power Word Kill
   `[[non-Angel, non-Demon, non-Devil, non-Dragon] creature]`; Crisis of
   Conscience `[[nonland, nontoken] permanents]`. All ten are newly covered.
4. **Deviations and STOPs**: The STOP on the erroneous syndetic premise was
   resolved by the [orchestrator ruling R15e](#orchestrator-ruling-r15e-2026-10-09).
   Three constructions provide noun/adjective member projections and the
   shared-head consumer; existing comma schemas supply the coordination.
   These are a net +3 Constructions against the 250 ceiling (`xqtkuyqk`).
   No extra syndetic declaration, frame or lexicon route was added or retired.
   Terror's syndetic Nominal Reading stays unchanged; unattested
   `red, green creature` reads 1 on `xqtkuyqk`, permitted by R15e.
   The draft census found Royal Decree increasing 2 → 3 (`xqtkuyqk`): its
   original list `[Swamp], [Mountain], [black permanent], or [red permanent]`
   gained `[Swamp], [[Mountain, black] permanent], or [red permanent]`.
   Propagating the declared unmarked-conjunct licence excludes the latter
   under R15d. The orchestrator review ruling (2026-10-09) accepts its cost:
   a CoordinatedPremodifiedNominal cannot be a non-final member of a
   coordinator-ending Nominal list, so `[[P1, P2] N], N, or N` stays closed.
   That ruling records 0 supported faces attesting this shape; this restriction
   is a project ruling, not a CGEL claim.
   The final census restores 2 → 2 and changes no other covered
   face. No claim-parent Reading was removed. The glossary adds Premodifier
   Coordination and clarifies UnmarkedConjunctLicence; CGEL pp. 1326–28
   supports likeness of function, while Oracle restrictions cite R15e.
   Beyond the five witness tests, mixed and triple-member value roundtrips,
   the exact syndetic control and the Royal Decree regression protect grouping
   and preservation. No existing route is superseded. Refresh's registration
   conflict was resolved by retaining both Produce and this ticket's modules;
   the English/lexicon changes triggered the bounded re-census and full English
   suite. Later shared-claim rewrites changed only other tickets' status paths,
   with no change to tested grammar or consumed data.
5. **Tests** (`xqtkuyqk`): `cargo test -p deckmaste_english_v3` passed
   (679 tests, 0 failures, 0 ignored);
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed.
   Citation checks: 0 noncompliant, 0 stale; diff audit selected no CR sites.
   Post-refresh full `cargo test -p deckmaste_english_v3` passed (`zumztwqt`):
   696 tests, 0 failed, ignored or filtered; 0 doctests. Only this feature's
   changed code crate requires the post-refresh suite.
   Review recheck (`zmlqouow`), on the current refreshed grammar:
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed.
   Restored 0 / re-spelled 0 / ignored 0 / added 9 / removed 0 (`xqtkuyqk`).
6. **Timing** (`xqtkuyqk`): final bounded census 6m34.14s, 6 workers,
   peak RSS 14,207,812 KB (`/usr/bin/time -v`, prebuilt binary,
   `--samples-per-face 0`). After refresh (`zumztwqt`): bounded re-census
   3m56.15s, 6 workers, zero samples, peak RSS 16,933,624 KB under the same
   prebuilt-binary `/usr/bin/time -v` command.
