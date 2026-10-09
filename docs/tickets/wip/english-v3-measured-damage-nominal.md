---
needs: []
---
# Let a cardinal-measured *damage* take a Determiner and postmodifier: Prevent the next 3 damage that would be dealt to any target this turn.

## Why

*N damage* reads only as a closed NP. On census `qsupksky` (default `@` `mqrlzszk` is empty; 32,828 supported faces, 25,474 covered, 7,354 unread; wave-5 recon of 2026-10-08), the recon bucket "prevent the
next N damage" touches **118** unread faces and is the sole cause on **100**.
Counts are unread faces *touched* (at least one localised failing unit matches) / *sole* (every failing unit matches and no other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (`--readings 1`): "Prevent 3 damage." 1; "Prevent the 3 damage." 0;
"Prevent 3 damage that would be dealt to you this turn." 0; "Prevent the next
damage that would be dealt to any target this turn." 1; "Exile the next three
cards." 1.

## Goal

A cardinal-measured *damage* reads as a Nominal, so a Determiner (*the
next 3 damage*) and a relative postmodifier (*the next 3 damage that would
be dealt …*) attach, with byte-exact realisation. *deals 3 damage* keeps its Readings.

## Analysis

Existing route: *3 damage* reads today only through the closed
`MeasuredNounPhrase` (`crates/deckmaste_english_v3/src/declarations.rs:3988`,
`[quantity: MeasurePhrase, head: lexical(Noun)]`, requires
`MeasurePosition = Before`), which takes no Determiner and no postmodifier.
Both `CardinalMeasuredNominal` (`declarations.rs:2548`, admits Determiners
and postmodifiers) and `CardinalMeasuredNounPhrase` (`declarations.rs:2576`)
require `CardinalMeasurementUse = Yes`, declared only on *mana*
(`crates/deckmaste_lexical_source/lexicon/core.ron:167`); *damage* has
`MeasurePosition: Before` only (`core.ron:168`).

Orchestrator ruling 2026-10-08 (R7a): declare `CardinalMeasurementUse: Yes`
on *damage* in the lexicon; no new Construction. *the next 3 damage that
would be dealt …* then reads through the route *3 mana* uses
(`CardinalMeasuredNominal` under a Determiner). No CGEL citation is added;
the existing analysis is reused unchanged.

Watch the bare string: with the feature, *deals 3 damage* becomes eligible
for `MeasuredNounPhrase`, `CardinalMeasuredNounPhrase` and
`CardinalMeasuredNominal` alike. First measure what *3 mana* yields today
(*Add 3 mana.*). If *3 damage* gains a second label for one constituency that
*3 mana* does not already have, that is a spurious duplicate (Method 8);
resolve it by restricting or retiring the redundant closed route, not by a
guard naming *damage*. A duplicate *3 mana* already carries is inherited:
record it, do not widen it.

Owner rulings (2026-10-09): admit Arabic numerals through the existing
numeral categories on the measured routes, and verify *damage* OR *mana* plus
300 seeded covered controls. The subsequent restriction ruling supersedes
the initial `Yes` declaration in R7a: bare Arabic *N damage* / *N mana* keeps
its closed measured NP, with no additional postmodifier or relative route.
Only the measured Nominal under an outer Determiner gains attachment.
The attempted open bare route's 1,753 increases and seven decreases were
wrong analyses; restrict the routes and retain the existing test expectations.

Resolution: extend the declared Cardinal Measurement Use licence with
`DeterminerRequired` on *damage*. `CardinalMeasuredNominal` reads that licence,
uses the existing `QuantityConjunct` numeral projections, and exports
`DeterminerRequirement = Yes`. Its `DeterminerProjection` licence survives
Nominal modifiers; Nominal concord rejects mixing it with an ordinary
recipient. The numeral quantity slot declares whether its projection is new
or inherited, preserving the parent mana projections. This blocks the wrong
*any target and 1 damage to you* coordination without broadening the existing
test sets. The existing bare cardinal route still reads `Yes`, and the closed
`MeasuredNounPhrase` is unchanged. No Construction or
word guard is added. *the next 3 mana* is an authorised refinement beyond the
ticket's letter; any corpus *mana* gains are disclosed below. These are project
rulings, not additional CGEL claims.

Memory-first ruling (2026-10-09): first prove zero covered-face increases on
the five witness faces plus a small selection of *deals 3 damage to any target*
faces. Then run the larger selector with six workers, measure peak RSS, and
kill the run if its resident set exceeds 30 GB; report and do not retry.

## Witnesses


Each is the face's only failing unit at `qsupksky` (0 Readings), each
control (*the next damage …*) 1; (D) = `--category Document`:

- Hold at Bay: "Prevent the next 7 damage that would be dealt to any target
  this turn."
- Esper Battlemage (D): "{W}, {T}: Prevent the next 2 damage that would be
  dealt to you this turn."
- Anoint: "Prevent the next 3 damage that would be dealt to target creature
  this turn."
- Barbed Wire (D): "{2}: Prevent the next 1 damage that would be dealt by this
  artifact this turn."
- Kitsune Healer (D): "{T}: Prevent the next 1 damage that would be dealt to
  any target this turn." 

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): one bounded census of every supported face
   whose analysed text contains case-insensitive "damage" or "mana", plus 300 random covered controls
   (`cargo xtask english-v3 diff --manifest … --random 300 --seed 20261008`),
   with the identity manifest, 6 workers and `--samples-per-face 0`; compare
   it with `cargo xtask english-v3 diff` against the refreshed claim parent's
   census of the same manifest. Run the small selector first; wrap the large
   run in the owner-approved foreground Python process-tree RSS guard
   (sum `/proc` RSS every two seconds, record the peak), killing it above
   30 GB without retrying; this replaces the unavailable `/usr/bin/time -v`. Never run `--all`. Tests: `cargo test -p` and
   `cargo clippy -p … --tests -- -D warnings` for `deckmaste_lexical_source` and `deckmaste_english_v3`. No xtask tests, no
   Lean checks, no post-refresh re-census unless refresh touched grammar,
   lexicon or core. New tests go under tests/suite/ registered in
   tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Probe each witness before implementation and record the counts; write each
   witness as a test. Run the tests and clippy once, at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the refreshed claim-parent census.
   Target zero decreases and zero increases on previously covered faces.
   Any remaining increase must be named and shown to be the same duplicate
   already carried by *3 mana* on the parent.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *3 damage* reading both as the closed NP and as the determinerless Nominal NP on the same string (a spurious duplicate), or *deals 3 damage* gaining Readings, is a defect; restrict or retire the closed route per Method 8. A wrong analysis that parses is a defect, not a gain. The owner restriction
   ruling keeps the closed NP and restricts the open route to a Determiner;
   it supersedes the earlier suggestion to retire the closed route.
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

- Other measure nouns (*life*) unless attested on a face this ticket covers.
- *Prevent all but N damage*.

## Landing record

1. **Census delta**: refreshed claim `lxmzprsw` (parent-code measurement
   `uzykpstnnpxo`) → `pplynlpnqzxn`: 5,157 → 5,283 covered faces;
   2,207,768 → 2,244,417 Readings. Lost none; gained 126; increases 0;
   decreases 0; digest-only 13; source changes 0. Selector: 7,322 identities,
   supported analysed text containing damage OR mana plus 300 random covered
   controls (seed 20261008). All enumerations complete; every Reading checked
   byte-exact; issues, duplicates and internal failures 0. Stock diff evidence:
   `target/english-v3/english-v3-measured-damage-nominal/diff.json`.
   Review refresh measured at `ttwuwpxuonzm`: 5,283 → 5,366 covered;
   2,244,417 → 2,510,941 Readings; 83 trunk gains, lost/decreases/increases
   0, digest-only 561 (`diff-refresh.json`). Combined delta from measured
   parent: gained 209, lost/decreases/increases 0, digest-only 573
   (`diff-final.json`); all 2,510,941 Readings exact, all faces complete,
   issues/duplicates/internal failures 0.
2. **Witnesses** (`pplynlpnqzxn`, complete probes): Hold at Bay 18;
   Esper Battlemage 18; Anoint 18; Barbed Wire 42; Kitsune Healer 18.
   All five were probed at 0 on refreshed parent code before implementation.
   `Prevent 3 damage.`, `Add 3 mana.`, and `deals 3 damage to any target`
   remain 1. Bare numeric damage with a relative or PP remains 0.
   All witness and control counts remain identical on refreshed `ttwuwpxuonzm`.
3. **Spot checks** (`pplynlpnqzxn`): ten confirmed newly covered faces,
   each selecting a byte-exact determined `CardinalMeasuredNominal` with a
   `SubjectRelativeNominal`: Hold at Bay, Anoint, Kitsune Healer, Alabaster
   Wall, Barrenton Medic, Clergy en-Vec, Combat Medic, Conservator, Argivian
   Blacksmith, Bandage. Final probe evidence: `target/english-v3/english-v3-measured-damage-nominal/final-spot-{0..9}.json`.
4. **Deviations and STOPs** (`pplynlpnqzxn`): no Construction or frame added
   or deleted; seven tests added (five witnesses, two measurement controls).
   Owner-authorised numeral refinement makes `the next 3 mana` read 1;
   no mana-only face gained. Five gains also contain mana: Necra Disciple 18,
   Orim's Cure 36, Refraction Trap 888, Scars of the Veteran 1,944, Shining
   Shoal 558; their new determined damage analyses cause the gains.
   The 13 unchanged-count digest changes re-spell the inherited measured
   mana quantity through `CardinalQuantityConjunct`: Blighted Burgeoning,
   Buried in the Garden, Dawn's Reflection, Fertile Ground, Gauntlet of Power,
   Glittering Frost, Market Festival, Open the Omenpaths, Regal Behemoth,
   Shimmerwilds Growth, Trace of Abundance, Utopia Sprawl, Verdant Haven.
   The rejected open bare route and mixed-recipient coordination leak are
   fixed through declared licences under the owner rulings, without expanding
   expected sets. Closed NP routes remain intact. The existing Cardinal
   Measurement Use glossary entry is clarified; no glossary gap remains.
   Environment STOP resolved by the owner's approval of foreground Python
   RSS accounting in place of unavailable GNU time. Review refresh appended
   test modules on both sides of `tests/main.rs`; retained both and resolved
   the conflict. Its 64 comparative-value gains come from trunk `xrmmlsotvqpw`;
   19 anywhere gains from `uzrzywomopku` / `rwxrkzuwrrqq`. All 561 refresh
   digest changes contain the inherited `PostpositiveNominal`, generalized
   to a schema by `rwxrkzuwrrqq`; their counts are preserved. No outstanding STOP.
5. **Tests** (`pplynlpnqzxn`): `cargo test -p deckmaste_english_v3` passed
   (625 tests); `cargo test -p deckmaste_lexical_source` passed (27 unit +
   35 integration); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   The five previously failing tests retain their expected sets and pass.
   The initial corrective run's two coordination failures were fixed in
   grammar before the final successful run. Restored outcomes 5; re-spelled
   1 invalid bare-Nominal fixture for the existing quantity projection;
   ignored 0; added 7; removed 0. On refreshed `ttwuwpxuonzm`, reran only
   the same four commands for the affected crates: English 641 passed,
   lexical source 27 + 35 passed, both clippy commands passed. No xtask
   tests or Lean checks.
6. **Timing** (`pplynlpnqzxn`): one 7,322-face bounded candidate census,
   six workers, 710.44 s wall; peak process-tree RSS 18,547,256 KiB
   (18.99 GB), sampled from `/proc` every two seconds under a foreground
   30 GB kill guard. Kernel `wait4` peak 18,543,840 KiB. Required bounded
   recheck after grammar/lexicon refresh (`ttwuwpxuonzm`): same 7,322 faces,
   six workers, 874.84 s wall, peak process-tree RSS 21,951,868 KiB
   (22.48 GB); kernel peak 21,949,240 KiB. Neither guard triggered.
