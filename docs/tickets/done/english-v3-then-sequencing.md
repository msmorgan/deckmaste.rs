---
needs: []
---
# Read `, then` sequencing between clauses and predicates

## Why

`, then` joining two imperative or declarative clauses (or predicates sharing
one Subject) is the largest single unread surface in the supported corpus. On
change `wlvwtnppyovn` (32,828 supported faces, 13,716 covered, 19,112 unread)
it occurs in a failing unit of 2,188 unread faces and is the only recognised
cause on 874 of them. These are surface-bucket counts from an unread-face
reconnaissance, not gain forecasts or cause proofs.

The minimal pair is clean: `Draw a card and discard a card.` has one admitted
root and `Draw a card. Then discard a card.` has one, but `Draw a card, then
discard a card.` has none (`cargo xtask english-v3 probe --readings 0`, same
change). `vocab:Adverb/Then` exists; the gap is the linkage, not the word.

## Goal

Admit `X, then Y` where X and Y are imperative clauses, finite clauses, or
predicates sharing a Subject or modal, including the final link of a serial
list (`A, B, then C`). Preserve every grammatical Reading and both roundtrip
laws. Sentence-initial `Then …` already reads in the probe above; the 311
unread units that open with *Then* fail on other causes (comparisons, library
positions, *where X is*), so confirm that and do not count them as this
ticket's gains.

## Analysis

CGEL lists *next* and *then* among the ordering pure connectives
(CGEL, Ch. 8, §19, p. 778, [9ii]). In `X, then Y` with no overt coordinator,
*then* is the sole linking item between the two units. CGEL analyses *so* and
*yet* in the sole-linking-item position
([*There was a bus strike on,*][*so we had to go by taxi*]) as markers of
coordination, not asyndetic juxtaposition. They also link finite VPs
(CGEL, Ch. 15, §2.10, pp. 1319–1320, [79]–[80]). Applying that account to
Oracle *then* is this ticket's project analysis, not a claim of the cited
passage. CGEL's *and so* and *and yet* examples have an overt Coordinator and
a modifier within the second Conjunct (same section, [78]). The corresponding
account of Oracle *and then* is likewise the project's application. The
second unit is a Conjunct, so imperative-with-imperative and
predicate-with-predicate sharing must reuse the existing Coordination and
Shared Complement machinery rather than add a sequence-only Clause family.
Whether *then* is analysed as a Coordinator-like marker or as a connective
Adjunct heading the second Conjunct is the implementer's design decision. If
the two analyses differ in Reading count on the witnesses, STOP and report
before choosing.

## Witnesses

- Blur: "Exile target creature you control, then return that card to the
  battlefield under its owner's control."
- Obsessive Stitcher: "{T}: Draw a card, then discard a card."
- Risky Research: "Surveil 2, then draw two cards."
- Beneath the Sands: "Search your library for a basic land card, put it onto
  the battlefield tapped, then shuffle." (serial, *then* on the final link)
- Erode: "Its controller may search their library for a basic land card, put it
  onto the battlefield tapped, then shuffle." (declarative host; three
  predicates share the Subject and *may*)
- Vengeful Villagers: "Tap it, then you may sacrifice an artifact or creature."
  (second Conjunct has its own Subject and modal)

Each witness's only failing unit is the one quoted.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/then-sequencing-before.json` on the claim parent;
   record no/one/multiple, covered faces and exact Readings, stamped with the
   change id.
2. Write the six witnesses as tests first. Assert the coordination structure,
   not just recognition.
3. Iterate with `--face-id` selectors (the witnesses plus a sample of the 874);
   verify on `--all` once at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. A negative or wrong analysis
   that starts parsing (for example *then* read as a temporal Adjunct of the
   first clause only) is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule, `docs/decisions/english-lexical-analysis.md`).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- The 42-face frame-coordination residue in `english-v3-systemic-residuals`:
  none of those faces fails on `, then` (29 fail on library positions and are
  owned by `english-v3-library-position`; 10 fail on Deal's ordered
  amount/recipient segments). This ticket takes over none of them.
- Library positions, comparisons and *where X is* clauses inside a
  `then`-Conjunct; they are owned elsewhere.

## Landing record

### Analysis and route ownership

The chosen project analysis treats sole-linking *then* as a Coordinator-like
marker. This is the implementer's application to Oracle English, supported by
the witness comparison; CGEL's passages describe ordering connectives and the
sole-linking uses of *so* and *yet*, not this project's classification of
*then*. The Analysis section now makes that distinction explicit.

The two candidate implementations enumerate the same complete Document
Readings on every quoted witness (limit 10,000; no witness reaches it):

Witness | Before | Coordinator-like marker | Connective Adjunct in second Conjunct
--- | ---: | ---: | ---:
Blur | 0 | 22 | 22
Obsessive Stitcher | 0 | 2 | 2
Risky Research | 0 | 2 | 2
Beneath the Sands | 0 | 14 | 14
Erode | 0 | 16 | 16
Vengeful Villagers | 0 | 1 | 1

The ticket's count-difference STOP condition therefore does not arise. There
were no contradictions with a recorded ruling and no unresolved STOPs.

`vocab:Coordinator/Then` owns the comma link. Its declared Bare Coordination,
Comma Coordination and General Coordination permissions license the existing
clausal and verbal Coordination machinery. Production requirements read
features only. Serial lists use the existing continuation and final-link
constructors: Beneath the Sands retains Search, then the flat tail containing
Put and Shuffle. Erode retains one Subject and modal above that serial
predicate; Vengeful Villagers retains the second Clause's own Subject and
modal. Existing agreement and Shared Complement policies remain in force.

The experimental Adjunct route is retired on both sides: no experimental
Connective Conjunct category, construction, schema or lexical feature remains.
The pre-existing Adverb owner is still live for sentence-initial *Then* and is
not a superseded comma-link route. The positive sentence-initial control
checks that owner explicitly. No gains are credited to sentence-initial
*Then* alone.

### Proof and census

All census, candidate, inventory and review artifacts are ignored files in
`target/english-v3/`; the completed evidence set is retained under
`target/english-v3/english-v3-then-sequencing/` in the default workspace.
The baseline measured the claim grammar before implementation; the final
measurement uses the chosen grammar. Both snapshots carry working change
`lklxwwprzolopyowyuputxtpvzttpnpm`, whose identity persists across edits;
covered counts and lexical inventory hashes distinguish their measured trees.
The claim is `urzkqprqoqlwvxpqlstlnyprnsqpltqv`.

Metric | Before: covered 15,997 | After: covered 16,977
--- | ---: | ---:
Supported faces | 32,828 | 32,828
No Reading | 16,831 | 15,851
One Reading | 6,986 | 7,025
Multiple Readings | 9,011 | 9,952
Exact and checked Readings | 245,342 | 346,445
Complete face enumerations | 32,828 | 32,828
Limited / undetermined / failed faces | 0 / 0 / 0 | 0 / 0 / 0
Validation issues | 0 | 0
Duplicate Readings / internal failures / cyclic derivations | 0 / 0 / 0 | 0 / 0 / 0

Both measurements use input SHA-256
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`.
Before lexical inventory SHA-256:
`3bc160c3406eb3ee82f923dfcdc0f8a803fa3932c0812d6623e331e8c527e779`.
After lexical inventory SHA-256:
`958e1ccfed83835a7cd480940ab23ae665b763d288e68ca25a7e278a9fc3fd4d`.

The lost-identity list is `[]`; the list of previously covered identities
whose exact Reading counts decreased is also `[]`. No retirement,
re-coverage obligation or regression needs routing. The complete comparison
is in `loss-accounting.json` in the retained evidence directory.

Every counted Reading passed declaration admission, lexical ownership and
context, byte-exact realization, and construction/leaf traversal comparison
with independent materialization traces. The independent constructed Stitcher
value additionally proves value → realization → retained equal Reading and
both complete node and word traversal identities. Enumeration retains every
Reading; there is no destructive specificity selection. The current lexical
analysis ruling supersedes the old zero-tie requirement. The one/multiple
census above reports ambiguity directly; unique/specificity-resolved selection
counts are not an active v3 metric.

Newly covered identities number 980. Every selected representative was checked
for its comma-link role and inspected in the grouped verb/Conjunct review;
every link is clausal or verbal, with verbal content on both sides. These are
representatives for reporting, not grounds for discarding other grammatical
Readings or preferring game-semantic interpretations. No negative oracle or
first-Clause-only temporal-Adjunct reading is counted as a gain.
The complete identity/name/selected-analysis/Reading-count list is
[landing-identities.md](../../../target/english-v3/english-v3-then-sequencing/landing-identities.md);
`gains-reviewed.json` retains each source and representative fingerprint.
Selected examples, all on the after tree stamped above. Counts below cover
the full face; the comparison above covers only each quoted witness unit
(Obsessive Stitcher therefore has 10 full-face Readings and 2 quoted-unit
Readings):

Face | Selected representative analysis | Exact Readings
--- | --- | ---:
Blur | `Coordination<SecondaryVerbPhrase>: exile/control -> return` | 22
Obsessive Stitcher | `Coordination<SecondaryVerbPhrase>: draw -> discard` | 10
Risky Research | `Coordination<SecondaryVerbPhrase>: surveil -> draw` | 2
Beneath the Sands | `CoordinationSeriesEnd<SecondaryPredicateSeries>: put/tap -> shuffle` | 14
Erode | `CoordinationSeriesEnd<SecondaryPredicateSeries>: put/tap -> shuffle` | 16
Vengeful Villagers | `ClauseCoordination<Clause>: tap -> may/sacrifice` | 1
Safewright Quest | `CoordinationSeriesEnd<SecondaryPredicateSeries>: put -> shuffle` | 16
Thunderherd Migration | `CoordinationSeriesEnd<SecondaryPredicateSeries>: put/tap -> shuffle` | 168
Squadron Hawk | `CoordinationSeriesEnd<FinitePredicateSeries>: put -> shuffle` | 27
Solemn Simulacrum | `CoordinationSeriesEnd<FinitePredicateSeries>: put/tap -> shuffle` | 44
Cruel Ultimatum | `CoordinationSeriesEnd<FinitePredicateSeries>: discard -> lose; CoordinationSeriesEnd<FinitePredicateSeries>: draw -> gain` | 10
Blighted Woodland | `CoordinationSeriesEnd<SecondaryPredicateSeries>: put/tap -> shuffle` | 14

The active v3 command does not emit the retired coverage-lock `covered` metric
or legacy permitted-licensing-checker total; those metrics are not applicable.
The supported-face coverage count is 16,977. New production checkers naming a
word, lexeme, verb, construction or card: 0. Lexical source load errors and
GrammarEnvironment load errors: 0.

### Inventories and performance advisory

These inventories are stamped with change
`lklxwwprzolopyowyuputxtpvzttpnpm`, before covered 15,997 and after covered
16,977. Named grammar constructors remain 237 (193 ordinary constructions and
44 schemas); declared categories remain 138. The final environment has 598
static productions and 666 compiled productions. No named construction or
category was added. There is exactly one added lexical owner; removing it
from the normalized after inventory reproduces the before inventory hash
byte-for-byte, proving that all pre-existing owners, forms and frames survive.

Licensed surface homographs, excluding Catalog atoms and including declared
capitalization variants: 393 before, 395 after. Added named surfaces:
`then` and `Then`, both owned by `vocab:Adverb/Then` and
`vocab:Coordinator/Then`; no prior homograph is retired. Case-folded inventory:
358 before, 359 after, with sole added surface `then` and the same owners.
The complete named inventory is
[homographs.md](../../../target/english-v3/english-v3-then-sequencing/homographs.md)
and `grammar-inventory.json`. Form-literal/vocabulary overlap inventories are
`[]` before and after: the grammar's added literal is punctuation, not a word.

Performance is advisory, not a gate. All times are integers in nanoseconds,
and CPU telemetry is integer nanoseconds per checked byte. The quiet-host
coverage ceiling is 16,260,000,000 ns. Both corpus runs exceed that ceiling;
the host was not quiet, and the after run overlapped compiler verification.
These are v3 corpus-command measurements, not a claim of meeting the retired
coverage command's timing ceiling.

Metric | Before: change lklxwwpr, covered 15,997 | After: change lklxwwpr, covered 16,977
--- | ---: | ---:
Workers | 12 | 12
Host load (1 / 5 / 15 minutes) | 29.455078125 / 15.52978515625 / 9.390625 | 5.85400390625 / 8.37841796875 / 9.30224609375
Setup wall time | 18,536,437,228 ns | 6,521,190,111 ns
Corpus wall time | 90,681,013,516 ns | 111,575,648,988 ns
Total thread CPU | 711,025,851,409 ns | 849,020,374,435 ns
Checked-text CPU telemetry | 314,396 ns/B | 384,282 ns/B
Checked text bytes | 1,388,730 | 1,523,321

### Assurance and validation

The six witness tests were written first and all six failed before the change.
The final suite adds eight tests: the six named witness structures, the
independently constructed Stitcher roundtrip/traversal test, and the marker
punctuation/distribution test. Restored: 0; re-spelled: 0; added: 8; removed: 0;
newly ignored: 0. No existing test was deleted, weakened or newly ignored.

The derived reverse-dependency gate is
`cargo xtask gate --changed --from urzkqprqoqlwvxpqlstlnyprnsqpltqv --run`.
Its Cargo command is
`cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
Gate exits 0: 745 passed, 0 failed, 1 pre-existing ignored test;
all eight new tests pass. The inherited ignore is unchanged:
`macros::templates::tests::macro_schema_census_count_matches_21`, blocked on
the on-demand live-corpus census cross-check.

Strict clippy passes for that same crate closure with `--all-targets -- -D warnings`.
The edited Rust files pass scoped nightly rustfmt. Citation verification:
16,059 citations checked before refresh and 16,112 after refresh, 0 stale,
0 noncompliant strings; the actual piped
jj diff audit selected 0 changed CR citation sites. No new CR rule needed
registration.
The CGEL passages were read directly, including the distinction between the
passages' *so/yet* examples and the project's Oracle *then* application.

Refresh incorporated only other ticket documentation. The verified grammar
and lexicon still match the saved candidate byte-for-byte; no code or consumed
data in the test closure changed. Corpus and test results remain applicable.
The citation count rose because the newly incorporated tickets contain
citations; the refreshed tree also passes the complete citation check.

### Deviations and additions

- Three declared lexical permissions, one distribution policy, one comma form
  in an existing schema, and one attested lexical owner supply the missing link.
  Existing constructor families, agreement and shared-frame machinery are reused.
- Two tests beyond the six requested witnesses prove independently constructed
  values, traversal identity, punctuation licensing and the retained initial
  Adverb route. They address structural laws and overgeneration directly.
- The glossary adds Connective Adjunct (CGEL-backed) and Coordinator Distribution
  (explicitly a project term), resolving the terminology gaps encountered here.
- The experimental Adjunct alternative was compared, then fully removed from
  both grammar and lexicon. Its sources and probe evidence remain ignored.
- The inherited whole-repository formatter changed unrelated files; every such
  edit was restored. Only the feature's five intended paths remain changed.
- No xtask command, flag, tracked fixture or verifier was added; plugin bodies
  are untouched. Evidence generation stays in ignored `target/english-v3/`.

No residue is transferred from the 42-face frame-coordination inventory.
Library positions, comparisons and *where X is* remain with their existing
owners; this landing does not claim their sentence-initial-*Then* failures.
