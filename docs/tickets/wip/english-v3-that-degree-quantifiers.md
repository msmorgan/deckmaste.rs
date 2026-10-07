---
needs: []
---
# Read degree *that* with *many*/*much*: that many cards, that much life

## Why

*That many* / *that much* never read. On change `xxknlzypsnwy` (32,828
supported faces, 17,322 covered, 15,506 unread; recon of 2026-10-07), the two
strings touch **590** unread faces and are the sole cause on **228** (recon
bucket "many/much": 602 / 229; *that many* 378 touched, *that much* 224, *twice
that many* 38). Counts are unread faces *touched* (at least one localised
failing unit matches) / *sole* (every failing unit matches and no other recon
STRONG bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "You gain that much life." 0; "Mill that many cards."
0; "Destroy that many creatures." 0; "You gain 2 life." 1. The lexemes exist:
`vocab:Determinative/Many`, `vocab:Determinative/Much` and
`vocab:SingularDemonstrative/That` in `core.ron`. The gap is the Determinative
Phrase with a degree modifier, not vocabulary.

## Goal

*that many* and *that much* read as a Determinative Phrase (head *many*/*much*,
degree modifier *that*) in Determiner function of a plural count or non-count
NP (*that many cards*, *that much life*, *that much damage*). The DP composes
through the shared Quantitative Determiner interface used by numerals and *up
to N*; do not build a second counted-NP family. Number agreement follows the
head: *many* with plural count, *much* with non-count.

## Analysis

The degree determinatives *many*, *much*, *few* and *little* take degree
modifiers like gradable adjectives, including *very*, *so*, *too*, *how*,
*this* and *that*, forming a Determinative Phrase (CGEL, Ch. 5, §11(c), p. 431,
[3]). *That* used anaphorically as a degree modifier (*Kim is about that old
too*) is described at Ch. 6, §3.2, p. 549. The anaphoric resolution of the
degree (the damage dealt, the life gained) is semantics, outside this grammar
ticket.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Tamanoa: "Whenever a noncreature source you control deals damage, you gain
  that much life."
- Mourning Thrull: "Whenever this creature deals damage, you gain that much
  life."
- Crosstown Courier: "Whenever this creature deals combat damage to a player,
  that player mills that many cards."
- Guilty Conscience: "Whenever enchanted creature deals damage, this Aura deals
  that much damage to that creature."
- Firedrinker Satyr: "Whenever this creature is dealt damage, it deals that
  much damage to you."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-that-degree-quantifiers-before.json`
   on the claim parent, stamped with its change id and covered count; after:
   the same command to
   `target/english-v3/english-v3-that-degree-quantifiers-after.json` on the
   final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *that* read as a demonstrative Determiner of a fused-head
   *many*/*much*, or *that much life* read with *life* outside the NP, is a
   defect. A wrong analysis that parses is a defect, not a gain.
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
11. Timings as integer ns and ns/B, with host load and worker count.
12. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- *twice that many* / *three times that much* (multiplier before the DP): note
  how many become readable; a multiplier modifier is a separate dependent.
- Comparative *as much X as …*, *how many*, *the same number of*.
- *that many* as a fused head without a noun (*draw that many*): include only
  if attested and the fused-head route already exists for numerals; record it.

## Landing record

In addition to the standard record: the DP structure chosen and the interface
it composes through; agreement evidence for *many* vs *much*; the disposition
of *twice that many*; before/after counts stamped with change ids; timings as
integer ns and ns/B with host load and worker count.

### Landing record — 2026-10-07

Standard constraints apply. Before figures are stamped `zzzsrztrvnqzxquwrumqzyxylpvumryt`
/ covered 20,030: that census has claim `trxpyvykowpksvztypwypstvukqlsqqo`'s
runtime declarations, before implementation. Final figures are stamped
`truuumzxqqxutmwwttoxtrzpkywykpqz` / covered 20,280, with implementation
`zzzsrztrvnqzxquwrumqzyxylpvumryt`. These stamps apply to the inventories,
assurance counts and corpus figures below. Runtime digests accompany the
reports. The first refresh was a no-op. The final refresh moves the claim
after `capture-and-scope` claim `mtxtksrr`, with no tree-content change in the
claim evolution and unchanged runtime digests. The claim-parent census
therefore also describes the refreshed base; no check behavior changed.

#### PROVE — retained Readings and structural laws

Both full censuses enumerate all 32,828 supported faces completely, with zero
issues, failures, duplicate Readings, admission failures, exact-realization
failures, lexical-ownership failures and construction/leaf traversal failures.
Coverage increases from 20,030 to 20,280 faces: 250 gains, zero lost faces.
Every previously covered face retains exactly its previous Reading count:
zero decreases and zero increases. Total Readings rise from 597,766 to
621,244; all 23,478 additions occur on newly covered faces. No Reading is
retired, so there is no loss ledger or re-coverage debt. Refresh introduced
no trunk-attributed decrease.

Independent degree DPs and their complete NPs establish both roundtrip laws,
exact grammatical structure and construction/leaf traversal identity. Each
pinned DP and NP has exactly one Reading. The unsupported nounless DP does
not acquire a fused-head NP route. Parsed and independently constructed wrong
noun numbers/countability and unlicensed degree roles are rejected.

Word-named licensing guards added: zero. The new conditions read `DegreeUse`
and the existing `DeterminerUse` licence, using `determiner_license`.
Lexical source loading and generated GrammarEnvironment construction succeed
in the focused tests and censuses. Existing loader/compiler error handling is
unchanged. V3 emits no
retired-parser coverage lock or permitted-licensing-checker total; those
figures are unavailable for this parser, rather than inferred to be zero.

Assurance: 8 tests added, 0 restored, 14 existing tests re-spelled through the
shared NP rename, 0 removed, 0 new ignores. Re-spelled tests preserve their
cards, outcomes and full value comparisons: four comparative-quantity tests,
four scalar-cardinal tests, four targeting-projection tests, and the two
slash-pair NP tests in `grammar.rs`. Six initial tests failed before the fix;
all eight focused tests subsequently passed. The new negative surface uses
“those much life”; ordinary “those many cards” can instead be a definite NP
with an internal modifier in general English (CGEL, Ch. 5 §7.11, p. 394), so
it is not asserted to be an ungrammatical Oracle construction here.

#### DISCLOSE — constituency, Agreement and breadth

`DegreeDeterminativePhrase { modifier, head }` has Determinative *that* as its
Degree Modifier and *many*/*much* as its head. It directly realizes the existing
`QuantitativeDeterminer` interface, which now exports `DeterminerUse` instead
of a number. Cardinal, comparative and quantitative-PP projections map their
existing number into the same licence without broadening their noun selection.
The existing counted-NP schema becomes `QuantifiedNounPhrase`, admitting both
counts and amounts through `determiner_license`; there is one NP family.

CGEL, Ch. 5 §11(c), p. 431 supports degree modification within a DP, including
*that* with *many*/*much*. Ch. 5 §7.11, p. 393 supports count plural heads for
*many* and non-count singular heads for *much*. Ch. 6 §3.2, p. 549 supports
anaphoric degree *that*. The declared `DegreeUse` feature and this shared
compiler interface are project representations, not claims attributed to CGEL.
The anaphoric degree's referent remains outside the grammar.

All 250 gained identities, names, complete Reading counts, selected Reading
identities/costs and NP-head analyses are listed in ignored
`newly-covered-readings.md` and `selected-analysis-audit.json`. The sampled
run completely validates their 23,478 Readings; each selected DP has the
correct lexical modifier/head and is the quantity of a `QuantifiedNounPhrase`
with its Nominal inside that NP. No gain is counted from a fused-head
*many*/*much* analysis or from a noun outside the quantified NP. Named
spot-checks, with complete final face counts, follow; every displayed phrase
has the head/modifier analysis just described:

| Face | Inspected quantified NP | Final face Readings |
|---|---|---:|
| Tamanoa | that much life | 2 |
| Mourning Thrull | that much life | 1 |
| Crosstown Courier | that many cards | 3 |
| Guilty Conscience | that much damage | 2 |
| Firedrinker Satyr | that much damage | 2 |
| Arcbond | that much damage | 6 |
| War Elemental | that many +1/+1 counters | 7 |
| Hornet Nest | that many 1/1 green Insect creature tokens | 8 |
| Whirlpool Rider | that many cards | 3 |
| Towering-Wave Mystic | that many cards | 1 |
| Wall of Essence | that much life | 1 |
| Leeches | that much damage | 2 |

The census changes from 7,371 unique / 12,659 multiple to 7,431 unique /
12,849 multiple. There is no specificity-resolution admission step; preference
ranks retained Readings without removing them. Existing multiplicities are
unchanged.

Supported reminder-stripped surface buckets overlap: *that many* occurs on
378 faces, covered 0 → 144; *that much* on 224, covered 0 → 111. Five gained
faces contain both, reconciling the 255 bucket gains to 250 distinct faces.
*Twice that many* occurs on 25 faces and *twice that much* on 13; both retain
0 covered faces. A multiplier modifying the DP remains deferred. There are
no supported *this many*/*this much* tokens or nounless *draw*/*discard*/*mill
that many* matches in the recorded scope search; no such route was added.
These are measured buckets, not gain forecasts or completeness claims for
other hosts.

Deviations and additions:

- Added feature `DegreeUse`, table `cardinal_determiner_use`, and construction
  `DegreeDeterminativePhrase`. Added no policy, schema, category, category
  instance, lexical entry, form, frame or plugin-body change. The feature
  declares degree roles rather than overloading temporal/demonstrative or
  comparison licences. The table preserves the existing number constraints
  while reusing `DeterminerUse` and `determiner_license` for count and mass.
- Replaced `CountedNounPhrase` with the broadened `QuantifiedNounPhrase` and
  re-spelled its tests; the new name fits amounts as well as counts. The old
  grammar declaration is absent. No lexical route or frame selected that
  construction, so there is no obsolete lexical declaration to retain or
  retire. All cardinal, comparative and quantitative-PP routes remain live.
- Defined Degree Modifier, Degree Use and Quantified Noun Phrase in the owning
  glossary and expanded Quantitative Determiner to include amounts and degree
  DPs. No Comprehensive Rules claim/citation was added; no glossary gap remains.

STOPs: none; no recorded ruling was contradicted. No overlapping construction
or frame was introduced.

#### REPORT — economy, provenance and performance

Claim-parent versus final non-blank declaration lines:
`crates/deckmaste_english_v3/src/declarations.rs` 3,259 → 3,269, **net +10**.
The inherited grammar already exceeds the 2,800-line ultimate ceiling; the
landing reuses its determiner category, licence, table and NP family. Named
construction/schema inventory is 205 ordinary + 43 shared schemas (248)
before and 206 ordinary + 43 shared schemas (249) after. The sole net addition
is the degree DP; the NP replacement is a rename and broadening. No instance
is added. The complete feature/table/policy/construction additions are listed
above and in `declaration-inventory.json`.

Declared-form homographs are unchanged: 1,127 spellings, of which 1,011 are
attested as exact spellings in raw supported Oracle text including reminders.
The named owner inventories are `inventories.json` and
`attested-homographs.json`. Form-literal/vocabulary overlap inventory: empty,
unchanged. No lexical surface or owner changed.

| Measured tree / whole-corpus covered | Workers | Host load (1/5/15 min) | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| `zzzsrztrvnqzxquwrumqzyxylpvumryt` / 20,030 (claim runtime) | 6 | 5.325/8.801/7.812 | 166,082,112,445 | 253,743 ns/B |
| `truuumzxqqxutmwwttoxtrzpkywykpqz` / 20,280 (final runtime) | 6 | 19.185/13.608/11.256 | 156,549,202,995 | 333,292 ns/B |

Both busy-host runs exceed the inherited 16.26s quiet-host ceiling. Per-byte
CPU rises while wall time falls; the final run overlaps gate compilation on
a much busier shared host, so these measurements do not isolate the change's
causal cost. This is a performance advisory, not an admission gate. The
all-gains sampled run on the final stamp additionally takes 8,623,579,712 ns,
481,624 ns/B, 6 workers, host load 17.360/16.001/12.624.

#### Verification and evidence

Full before/after commands: `cargo xtask english-v3 --all --workers 6
--samples-per-face 0 --output target/english-v3/before.json` and the same
command for `after.json`. The final run directly invokes the freshly rebuilt
`target/debug/cargo-xtask` binary. Iteration uses explicit `--face-id`
selectors for the five witnesses and all gains, with complete enumeration and
one checked sample per face. `delta.json` accounts for every face's count.

Gate: `cargo xtask gate --changed --from english-v3-that-degree-quantifiers
--run --clippy`, deriving:

```text
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

All evidence, censuses, probes, named inventories, analysis ledgers, scratch
verifiers and logs are under the feature workspace's ignored
`target/english-v3/`. They are retained before retirement under the
coordinator's ignored `target/english-v3/english-v3-that-degree-quantifiers/`.
No evidence snapshot, verifier or process artifact is tracked or embedded in
source.

Gate passes: 832 tests passed, zero failed, one inherited ignored
`macro_schema_census_count_matches_21`, whose existing blocker is
“cross-checks the live corpus against the census; run on demand”. Strict clippy
passes with warnings denied. Formatting passes for every changed Rust file.
Citation checks report zero noncompliant strings and zero stale sites among
16,112 citations; the piped whole-feature diff audit selects zero new
Comprehensive Rules sites. No citation blessing is needed.
