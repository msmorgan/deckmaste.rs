---
needs: []
---
# Read *divided as you choose among*: deals 2 damage divided as you choose among one or two targets.

## Why

*divide* is not a word. On census `qsupksky` (32,828 supported faces, 25,474
covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket
"divided as you choose" touches **81** unread faces and is the sole cause on
**66** (touched / sole as in the wave-5 recon; surface counts, not gain
forecasts). Attestation (`cargo xtask english-v3 grep -i`): *divided* 81
faces, *divided as you choose* 78; *divide* 2 (Defensive Formation's finite
"You can divide that creature's combat damage as you choose among …"; Squall,
SeeD Mercenary's ability word *Rough Divide*). Shapes: *N damage divided as
you choose among any number of targets / one or two targets / one, two, or
three targets / up to N targets / those permanents / them*, *its power
divided as its controller chooses among …*, and a supplementive *…, divided
as you choose.* (6 faces).

Probes (`--readings 1`): every witness reports the vocabulary gap *divided*.
Diagnostics with the declared verb *distribute*: "Distribute two counters
among one or two target creatures." 1; "Distribute two counters as you
choose among one or two target creatures." 0; "Forked Bolt deals 2 damage
distributed among one or two targets." 0; "Two counters are distributed
among one or two target creatures." 0. So three pieces are missing: the verb,
its past-participial use with the *among* frame, and the *as you choose*
Adjunct before a frame Complement.

## Goal

*divide* is a verb with *distribute*'s attested *among* frame; *N damage
divided as you choose among X* reads as a Nominal with a past-participial
postmodifier whose internal Adjunct is *as you choose* and whose *among* PP
is the verb's frame Complement.

## Analysis

CGEL Ch. 5 §14.2(e), p. 446, [15iv] (*some letters written by her
grandmother*): past-participial clauses as NP-internal Modifiers. Ch. 13
§5.5, p. 1149, [34] ("Manner"): an *as* phrase whose Complement is a
comparative clause functions as manner element in its clause (*The louvres
are constructed [as — shown in the diagram]*). Those back the postmodifier
and the manner *as* phrase only.

Existing routes: `Distribute` (`crates/deckmaste_lexical_source/lexicon/verbs.ron:156–159`,
`[ObjectNounPhrase]` and `[ObjectNounPhrase, MarkedRole("Preposition",
"Among", "NounPhrase")]`, no `participle`); `Choose`
(`verbs.ron:22`, participle declared). `ParticipialPostmodifiedNominal`
(`crates/deckmaste_english_v3/src/declarations.rs:3681`, BarePassive past
participles; "Prevent all damage dealt to it this turn." reads 1).
*as* + finite clause is `ClauseComplementPreposition` (`declarations.rs:3308`,
`vocab:Preposition/As` declares `FiniteClauseComplement: Yes`,
`lexicon/core.ron:200`); the object-gap clause is `FiniteObjectGap`
(`declarations.rs:3920`; "Exile the card you choose." reads 1).

Orchestrator rulings 2026-10-09:

- R12a. Declare the verb *divide* (bare, third person, preterite and
  participle *divided*) with the attested *among* frame, copied from
  Distribute's; no new Role. Its plain transitive frame is declared only if
  attested (pruning rule; Defensive Formation is the one finite use).
- R12b. *as you choose* / *as its controller chooses* is an *as* phrase whose
  Complement is the existing gapped clause (`FiniteObjectGap`, *choose*
  taking its Object gap), attached inside the participial clause, not to
  *deals*. No intransitive *choose* frame, and no frame `Literal("as")`.
- R12c. If no existing route admits an Adjunct between a verb and its frame
  Complement, STOP with the measured options before adding one; do not solve
  it with a frame that names *as*.
- R12d. Why *distributed among …* fails as a participial postmodifier while
  *dealt to it* reads must be diagnosed first; if the cause is Distribute's
  missing `participle`, declaring *divided* correctly is the fix for this
  ticket, and Distribute's own participle is declared only if attested.
- R12e. Gaining *divided* must not give *damage* a second Reading for the
  same constituency (participle attached both to *damage* and to the clause).

R12f (orchestrator ruling, 2026-10-09; resolves R12c). A declared
phrase-head licence permits an Adjunct before a preposition-marked frame
Complement, never before an Object NP. The clause-complement PP route may
take the existing finite Object-gap predicate when its preposition declares
that permission. The *among* Complement belongs to *divide*, not *choose*;
the participle attaches once. The bounded selector additionally includes
`(?i)\bas (you|they|its|that|their|he|she|it)\b`; report every increase on a
previously covered face with its Reading shapes. Supplementive division is
included only if these same refinements admit it. This licensing detail is a
project ruling; CGEL p. 1149 [34] supports the manner comparative-clause
analysis only. The short `verbs.ron` declaration of *divide* is authorized;
mechanically derived forms are exempt from the attestation pruning rule.

Safety refinement under R12f: comparative Object gaps additionally require a
declared verb capability, separate from ordinary relative Object gaps. The
supported corpus attests *as you choose* and *as its controller chooses*;
Choose declares that capability. Without it, temporal *As this land enters*
also acquired an incorrect transitive Object-gap analysis. The permission
propagates through the existing gap predicates and their coordination; it
does not restrict their relative-clause use. This is a project licensing
refinement, not a claim made by CGEL p. 1149.

The existing scalar measurement Noun Phrase selected a lexical noun, so it
could not contain the valid *damage divided …* Nominal. It now selects a
Nominal with declared Scalar Measurement Head Use, supplied by the three
existing scalar mass nouns and retained through a participial Postmodifier.
Other nominal expansions do not acquire that permission: plain measurements
retain one constituency, and recipient PPs do not gain a second attachment
inside the measured head. This refines the existing route rather than adding
a second measured-phrase Construction. The selector includes every supported
face containing *damage*, *mana* or *life* to measure its reach.

## Witnesses

Each is the face's only failing unit at `qsupksky`; re-probed 2026-10-09 on
trunk `nxzntorp` with `--readings 1 --as-card`: witness 0 (gap *divided*),
control 1. (D) = `--category Document`.

- Forked Bolt: "Forked Bolt deals 2 damage divided as you choose among one or
  two targets." (control "Forked Bolt deals 2 damage to any target.")
- Magma Opus: "Magma Opus deals 4 damage divided as you choose among any
  number of targets." (control "… 4 damage to any target.")
- Samut, the Tested (D): "−2: Samut deals 2 damage divided as you choose
  among one or two targets." (control "−2: Samut deals 2 damage to any
  target.")
- Death to Our Enemies: "When you do, it deals 7 damage divided as you choose
  among one or two targets." (control "… 7 damage to any target.")
- Boulderfall: "Boulderfall deals 5 damage divided as you choose among any
  number of targets." (control "… 5 damage to any target.")

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text matches case-insensitive `\bdivid(e|es|ed|ing)\b|\bas\b|\bamong\b|\b(damage|mana|life)\b`, plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, `--workers 6` and `--samples-per-face 0`, run
   under `/usr/bin/time -v`; compare it with `cargo xtask english-v3 diff`
   against the claim parent's census of the same manifest. Never run `--all`.
   Tests: `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for
   every crate whose code or consumed data changed (`deckmaste_english_v3`
   always; `deckmaste_lexical_source` if `core.ron`, `verbs.ron` or
   `vocabulary.rs` changed). No xtask tests, no Lean checks, no post-refresh
   re-census unless refresh touched grammar, lexicon or core. New tests go
   under `tests/suite/` registered in `tests/main.rs`. Evidence lives under
   the workspace's ignored `target/english-v3/`, never `/tmp`.
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
   Reading. *as you choose* attached to *deals* rather than inside the participial clause, or the *among* PP read as anything but the frame Complement of *divided*, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *divided evenly, rounded down, among …* (2 faces).
- The supplementive *…, divided as you choose.* after the target NP (6
  faces) unless it falls out with no extra Construction.
- Engine semantics of division (`engine-divided-distribution-as-you-choose`).

## Landing record

1. **Census delta** (`umzuyqxmtqpx`): bounded manifest of 12,158 faces;
   claim-parent `xqvnutsq` behavior 9,036 covered / 584,429 Readings →
   9,102 / 584,929. Parent behavior was measured in `umzuyqxmtqpx` with its
   saved pre-edit binary and parent lexical data. Repository `diff --lines`:
   66 gained, zero lost, zero covered-face decreases or increases, 4,089
   digest-only, 8,003 unchanged; no source-hash changes or issues. Lost faces:
   none. Digest changes preserve counts while representing scalar heads as
   Nominals, adding the optional passive Adjunct, and retiring the old
   Choose frame index. Evidence: `target/english-v3/english-v3-divided-as-you-choose/`
   (`manifest.json`, `before.json`, `after.json`, `diff.json`). Selector:
   case-insensitive `\bdivid(e|es|ed|ing)\b|\bas\b|\bamong\b|\b(damage|mana|life)\b`
   (11,975 supported faces) plus 300 random covered controls, seed 20261008;
   117 overlaps removed (`umzuyqxmtqpx`). After grammar-bearing refresh,
   the same manifest (`uyspppsqonst`) covers 9,257 faces / 596,336 Readings.
   `diff-refresh.json` against the pre-refresh result reports 155 gained,
   zero lost/decreases/increases, 8 digest-only, 11,995 unchanged. These
   gains inherit arithmetic (`lrolnmwooptk`, `wnmmonzsruku`), asyndetic
   premodifiers (`xqtkuyqkkmkw`, `zmlqouowzvut`), and next-time adjuncts
   (`rxwmpvrunvkm`, `stymwqqornwx`). The eight digest changes inherit
   arithmetic operand wrappers, with counts preserved. Final parent-to-refresh
   `diff-final.json`: 221 gained, zero lost/decreases/increases, 4,090
   digest-only, 7,847 unchanged; no issues or source-hash changes. Evidence:
   `after-refreshed.json`, `diff-refresh.json`, `diff-final.json`.
2. **Witnesses** (`umzuyqxmtqpx`): Forked Bolt, Magma Opus, Samut, the Tested,
   Death to Our Enemies, and Boulderfall each read 1 after (0 on the parent);
   each corresponding control remains 1. Their ticket witness text, rather
   than unrelated lines of the complete face, is counted here.
3. **Spot checks** (`umzuyqxmtqpx`): ten newly covered faces each read 1.
   Every selected tree is a scalar measured NP whose *damage* Nominal has
   one selected passive Postmodifier: Divide with an internal manner As PP,
   comparative finite Choose Object gap, and its Among frame Complement.
   Arc Lightning: one, two, or three targets; Arc Mage: activated ability,
   one or two targets; Chandra's Pyrohelix: one or two targets; Deft Dismissal:
   one, two, or three attacking or blocking creatures; Explosive Crystal:
   any number of targets; Fiery Justice: any number of targets, with its
   life-gain sentence preserved; Flames of the Firebrand: one, two, or three
   targets; Forked Lightning: one, two, or three target creatures; Twin Bolt:
   one or two targets; Roil's Retribution: any number of attacking or blocking
   creatures. Probe samples: `spot-0.json` through `spot-9.json` under the
   evidence directory.
4. **Deviations and STOPs**: the R12c STOP found no existing intervening
   Adjunct site; resolved by **orchestrator ruling R12f, 2026-10-09**, quoted
   in Analysis. CGEL p. 1149 backs the manner comparative-clause analysis,
   not the licensing details. Two helper Constructions were necessary:
   FiniteGappedClause places the existing finite Object gap in a comparative
   Complement; InterveningPrepositionAdjunct supplies the optional licensed
   position before a selected preposition, never before an Object NP. One
   canonical frame, ObjectAmongComplement, represents Divide's attested
   frame; the short verb declaration supplies its mechanical forms. The
   superseded Choose-among frame and its generated selected-frame route are
   retired together. The initial core refinement retained per-form child
   Categories for ClauseComplementPreposition's alternatives. Refresh found
   that `rxwmpvrunvkm` had landed the same capability; reconciliation reuses
   that implementation and retains this ticket's production-category
   regression (only Symbol's test comparison traits remain in the own core
   implementation diff). Grammar feature interfaces and test registrations
   retain both sides of the refresh. Comparative
   Object Gap Licence removes the wrong temporal Enter gap under R12f's
   safety rule. MeasuredNounPhrase now takes a licensed Nominal to contain
   the existing participial Postmodifier; Scalar Measurement Head Use
   preserves plain measurement constituency and avoids extra PP attachment.
   The selector was widened to all As/Among text and the three measured
   nouns. Glossary gaps filled: Finite Object Gap Complement, Intervening
   Adjunct Licence, Comparative Object Gap Licence, Scalar Measurement Head
   Use. Beyond the five witness tests, one negative/temporal safeguard,
   one independently assembled roundtrip test, and one compiler regression
   were added. Supported-corpus grep finds five comma-supplement faces
   (Angel of Salvation, Embolden, Pollen Remedy, Remedy, Serra's Hymn),
   versus the ticket's six; all five remain unread, as does Defensive
   Formation (`umzuyqxmtqpx`). No supplementary mechanism was added.
5. **Tests** (`umzuyqxmtqpx`): all commands below used
   `CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0`.
   `cargo test -p deckmaste_english_v3` passed 683;
   `cargo test -p deckmaste_lexical_source` passed 28 unit + 36 integration;
   `cargo test -p deckmaste_construction_v3_core` passed 31.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`,
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`, and
   `cargo clippy -p deckmaste_construction_v3_core --tests -- -D warnings`
   all passed. Initial clippy found normalization clone/line-limit issues
   and two re-spelled tests one line over the limit; extraction of unchanged
   validation and noun-value helpers fixed them without weakening assertions.
   Core's extraction briefly failed to compile on a redundant slice conversion;
   that call was corrected, and its full suite passed. English's full suite
   was also repeated after the test-helper refactor and passed 683. No test
   execution failed. After refresh (`uyspppsqonst`), the same three full
   `cargo test -p` commands passed once each: English 712, lexical source
   28 unit + 36 integration, construction core 33; zero failed, ignored,
   or filtered. Logs: `test-<crate>-refreshed.log` in the evidence directory.
   Citation check: zero noncompliant, zero stale;
   `jj diff --git | cargo xtask cite audit --diff` selected no changed CR sites.
   Test changes: restored 0, re-spelled 9, ignored 0, added 8 (7 English,
   1 construction core), removed 0 (`umzuyqxmtqpx`). Re-spelling preserves
   exact values and card outcomes for passive fields, scalar noun heads,
   and Choose's retained infinitive frame index.
6. **Timing** (`uyspppsqonst`): refreshed bounded census **2:45.94 wall**, 6 workers,
   `--samples-per-face 0`, prebuilt binary under `/usr/bin/time -v`;
   peak RSS 12,906,784 KiB. The orchestrator owns the full corpus run.
