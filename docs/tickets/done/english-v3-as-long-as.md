---
needs: []
---
# Read "as long as" conditional adjuncts

## Why

*As long as* + clause, before or after its host clause, never reads. On change
`wlvwtnppyovn` (32,828 supported faces, 13,716 covered, 19,112 unread) it
occurs in a failing unit of 1,083 unread faces and is the only recognised
cause on 323. These are surface-bucket counts, not gain forecasts. Minimal
pairs on that change (admitted roots): "This creature has flying as long as
you control an artifact." 0, "… if you control an artifact." 1; "As long as
you control a Dragon, this creature has flying." 0, "If you control a Dragon,
…" 1.

## Goal

*As long as* + finite clause reads as a conditional Adjunct, clause-final and
clause-initial (with its comma), with the same host breadth as *if*. *For as
long as* + clause ("gain control of target artifact for as long as you control
this creature") reads as a duration Adjunct. Every ambiguity the corpus does
not resolve stays as multiple Readings; for example, clause-final *as long as*
may attach to a coordinated predicate or to one Conjunct.

## Analysis

CGEL, Ch. 8, §14.4(c), p. 758, [61i] and [62ii] lists *as/so long as*
among items used instead of *if* in conditional adjuncts, marked as excluding
the subordinator *that*, and expressing a necessary and sufficient condition
like *provided*. This supports treating the conditional item as a governor in
its own right, rather than composing its meaning from a comparison. CGEL,
Ch. 13 §4.5(d), p. 1134 distinguishes duration comparison from reanalysed
conditional *as long as*, a Compound Preposition meaning “provided”.

The **orchestrator ruling dated 2026-10-06**, resolving the STOP below, chooses
one Compound Preposition taking a Clause Complement for conditional Oracle
English, with the clause Adjunct breadth of *if*. Scalar equality with the
Duration Adverb *long* is admitted only inside *for*'s duration Complement.
Retaining both conditional labels would be a spurious duplicate. That
project restriction is the ruling, not a restriction attributed to CGEL.
The conditional Preposition Function Licence is Adjunct only: initial with
its comma, and final. No other supported-corpus function was attested.

## Witnesses

- Aeronaut Tinkerer: "This creature has flying as long as you control an
  artifact."
- Kargan Dragonrider: "As long as you control a Dragon, this creature has
  flying."
- Arisen Gorgon: "This creature has deathtouch as long as you control a
  Liliana planeswalker."
- Pristine Angel: "As long as this creature is untapped, it has protection
  from artifacts and from each color."
- Master Thief: "When this creature enters, gain control of target artifact
  for as long as you control this creature."
- Manor Gargoyle: "This creature has indestructible as long as it has
  defender." (also has an unrelated failing unit: "{1}: Until end of turn, this
  creature loses defender and gains flying.")

The first five witnesses each have only the quoted failing unit.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/as-long-as-before.json` on the claim parent,
   stamped with its change id.
2. Write the witnesses as tests first, each with its *if* twin. The two must
   have the same Reading count and structure modulo the governing Preposition.
3. Iterate on `--face-id` selectors; verify on `--all` at the end.
4. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
5. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. *long* read as an Adjective
   predicated of a participant, or *as long* read as a degree Modifier of the
   preceding clause, is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. Gate: `cargo xtask gate --changed --from <claim> --run`.
9. A CGEL citation may back only what the cited passage itself says; a project
   or orchestrator ruling is cited as a ruling, never attributed to CGEL.
10. Retire a superseded route on both the lexicon and the grammar side; do not
    leave unreachable declarations.

## Out of scope

- *as though* (264 touched / 63 sole) and *for as long as* outside a duration
  Adjunct.
- Host-clause causes unrelated to the Adjunct. Count a face only when it
  wholly reads.

## Investigation record (2026-10-06; STOP resolved)

At initial claim, `rslmnrov` had parent `ttnruzyt`. The before tree's consumed grammar and
lexical sources are byte-identical to that parent. Complete enumeration at
`zktwupyonwttzvqzrnynyzlkkqnzsszy` examined 32,828 faces: 15,851 no Reading,
7,025 one Reading and 9,952 multiple Readings; covered 16,977, exact Readings
346,445. Validation issues, internal failures and duplicate Readings are all
zero. Corpus wall time was 191,570,023,244 ns with 12 workers; initial host load
was 8.5693359375 / 13.54833984375 / 12.07666015625. Checked-text thread CPU was
425,022 ns/B. This exceeds the 16,260,000,000 ns quiet-host advisory ceiling;
the host was busy and compilation overlapped enumeration. The active v3
command does not emit the retired coverage-lock count.

All evidence is ignored under this workspace's `target/english-v3/`:
`as-long-as-before.json`, `as-long-as-before-provenance.json`, the before,
if-twin, compound and scalar witness JSON reports, and
`as-long-as-candidate-comparison.json`. The claim-parent census remains the
baseline for the final per-face Reading-count comparison.

Witness | Before | If twin | Compound conditional | Scalar conditional
--- | ---: | ---: | ---: | ---:
Aeronaut Tinkerer | 0 | 2 | 2 | 2
Kargan Dragonrider | 0 | 1 | 1 | 1
Arisen Gorgon | 0 | 2 | 2 | 2
Pristine Angel | 0 | 4 | 4 | 4
Manor Gargoyle (quoted unit) | 0 | 2 | 2 | 2
Master Thief (duration) | 0 | — | 3 | 3

Both candidates have complete enumeration and zero reported validation issues
on the witnesses. Their five conditional Reading sets are disjoint: the
compound has a Preposition Phrase with a finite Clause Complement, whereas
the scalar candidate has an equative Adverb Phrase with a duration Adverb
head. Equal cardinality does not equate these grammatical Readings. Master
Thief's three duration Readings are identical across the candidates: `for`
selects the equative duration Adverb Phrase, with the existing three clause/
predicate placements. The scalar experiment includes a clause-final Adverb
attachment so its host breadth is comparable rather than artificially narrow.

**STOP: the original Analysis section's different-Reading-set condition was
met.** The claimant reported the disjoint candidate Reading sets before
choosing. The orchestrator resolved it on **2026-10-06**:

> “conditional *as long as* is one Compound Preposition taking a Clause
> Complement”; scalar equality is admitted “only inside the duration
> Complement of *for*”.

The ruling rejects retaining both conditional analyses as a spurious
duplicate: conditional meaning is not compositional from duration. Its
implementation removes the experimental clause-final Adverb construction,
disables free Adjunct distribution for the duration Adverb, and retires the
standalone Adjective owner. The shared Adjective construction remains live
for other supported Adjectives; there is no long-specific grammar declaration
to leave unreachable.

Source distinctions: CGEL, Ch. 13 §4.5(d), p. 1134 explicitly distinguishes
ordinary duration comparison from reanalysed conditional `as long as`, a
Compound Preposition meaning “provided”. CGEL, Ch. 8 §14.4(c), p. 758 supplies
the conditional distribution, exclusion of `that` and necessary-and-sufficient
interpretation. CGEL, Ch. 13 §1.3, p. 1104 supplies the scalar-equality
governor and expanded Complement; Ch. 6 §5.2, p. 569 identifies duration
`long` as an Adverb; Ch. 7 §5.1(d), p. 640 includes `for long` among restricted
AdvP Complements. Applying those duration passages to the Oracle witnesses
is the claimant's grammatical application. Any exclusion of the scalar
conditional candidate must be recorded as a project ruling, not as a CGEL
ruling about these cards.

The stripped claim-parent census has 1,083 faces containing `as long as`, all
unread. Its only other `long` occurrence is the catalog name `Long Rest`,
which has separate lexical ownership. This is the evidence for retiring the
standalone Adjective route. The final full-corpus comparison verifies zero
lost faces and zero previously covered faces with fewer Readings.

Four tests were added first. The before run reports three expected failures
(conditional absence, duration absence and missing independent lexical value)
and one passing exclusion test. No existing test was changed or removed.
The final tests retain those subjects and add exact conditional/duration
Reading sets and the expanded-comparison marker exclusion. Completion and
verification are recorded below.

## Landing record

### Prove: retained coverage and structural laws

The original baseline claim `rslmnrovxlxolrplkopponxxkktvwpov` had parent
`ttnruzytzorynxytmyqmmuqqyxnppspp`. The before report was measured at change
`zktwupyonwttzvqzrnynyzlkkqnzsszy`, covered 16,977, with consumed grammar and lexical
sources byte-identical to that claim parent. The final report was measured
at change `zktwupyonwttzvqzrnynyzlkkqnzsszy`, covered 17,322. The lexical inventory hashes and
saved source SHA-256 values distinguish the two trees despite their shared
change id. All numbers in this record refer to those trees unless explicitly
stamped otherwise. The active v3 command has no coverage-lock metric; the
retired lock is untouched and is not authority for this enumeration.

Metric | Before: covered 16,977 | After: covered 17,322
--- | ---: | ---:
Supported faces | 32,828 | 32,828
No Reading | 15,851 | 15,506
One Reading | 7,025 | 7,111
Multiple Readings | 9,952 | 10,211
Exact Readings | 346,445 | 367,211
Complete enumerations | 32,828 | 32,828
Limited / failed / undetermined | 0 / 0 / 0 | 0 / 0 / 0
Validation issues / internal failures / duplicate Readings | 0 / 0 / 0 | 0 / 0 / 0

Before lexical inventory SHA-256: `958e1ccfed83835a7cd480940ab23ae665b763d288e68ca25a7e278a9fc3fd4d`.
After lexical inventory SHA-256: `50a975d9d292bd934306fe90df2485e942ae298fe8630f8d0884ec89b2399210`.

The complete per-face comparison against the claim-parent census reports
lost faces `[]` and previously covered faces with decreased Reading counts
`[]`. All 16,977 previously covered faces retain exactly their prior
Reading counts. No removed Reading, retirement obligation or regression
requires routing. Retiring the Adjective owner loses zero faces. Outside
`as long as`, the only supported `long` token is the separately owned catalog
name Long Rest; no standalone Adjective token is attested. Its lexical owner
is deleted; the experimental `ClausalAdverb` construction is also deleted,
and duration Adverb permission excludes both free Adjunct placements. No
superseded declaration remains unreachable. Other Adjectives retain their
live shared projection.

Every retained Reading passed declaration admission, byte-exact realization,
lexical ownership and construction/leaf traversal identity checks. The
independently constructed conditional Preposition Phrase proves value →
realization → equal retained Reading for both declared and initial
capitalization, with exact node and leaf traversal. Exact-set tests prove the
two Aeronaut conditional Readings and the three Master Thief duration Readings;
their sets include no competing duration or conditional Reading respectively.
The five conditional witnesses preserve their *if* twins' complete structures
and counts modulo the governor. The accepted English lexical-analysis contract
retains grammatical ambiguity; destructive specificity selection and a zero-tie
requirement are superseded. Unique/specificity-resolved selection is not an
active v3 metric; the one/multiple census above reports retained ambiguity.

New forbidden word-naming production checkers: 0. Guards read declared
features only. Lexical source and GrammarEnvironment load errors: 0. The
active v3 command emits no legacy permitted-licensing-checker total; that
retired metric is not applicable.

### Disclose: gains, ruling and assurance

There are 345 newly covered faces: 307 conditional-only, 37
duration-only, and Tide Shaper with conditional and duration phrases in
separate clauses. Every selected representative was reviewed for governor
ownership, Complement structure and host attachment. All conditional heads
are one Compound Preposition with a finite Clause Complement, including
coordinated protases; all duration heads are Adverbs in scalar equality
selected inside *for*. No predicated Adjective *long*, clause degree Modifier
*as long*, spurious conditional duration duplicate or negative oracle is
counted as a gain. Representatives describe the gain, while every grammatical
Reading remains retained.

The complete identity/name/Reading-count/selected-analysis list is
[as-long-as-landing-identities.md](../../../target/english-v3/english-v3-as-long-as/as-long-as-landing-identities.md).
The matching sources and fingerprints are in `as-long-as-gains-reviewed.json`.
Examples below count complete faces, rather than just the quoted units in the
investigation table (Pristine Angel therefore has 8 full-face Readings):

Face | Selected representative Reading | Exact Readings
--- | --- | ---:
Aeronaut Tinkerer | C: PrepositionPredicate | 2
Kargan Dragonrider | C: InitialPreposition | 1
Arisen Gorgon | C: ClausalPreposition | 2
Pristine Angel | C: InitialPreposition | 8
Master Thief | D: For + EquativeAdverb + finite Clause | 3
Brawn | C: InitialPreposition/coordinated protasis | 3
Tanglewalker | C: ClausalPreposition | 4
Sedge Sliver | C: PrepositionPredicate | 2
Freewind Equenaut | C: InitialPreposition | 1
Wingrattle Scarecrow | C: PrepositionPredicate; C: PrepositionPredicate | 4
Gravecrawler | C: PrepositionPredicate | 9
Dragonlord Silumgar | D: For + EquativeAdverb + finite Clause | 3
Cytoplast Manipulator | D: For + EquativeAdverb + finite Clause | 23

C denotes the conditional Compound Preposition; D denotes duration scalar
equality inside *for*. Manor Gargoyle's quoted conditional gains its two
Readings, while its full face remains unread because its other ability has
an unrelated cause. That quoted-unit gain is not counted as face coverage.

The original ticket mandated a STOP when the alternatives produced different
Reading sets. It was reported before choosing. The **orchestrator ruling,
2026-10-06**, resolved it: “conditional *as long as* is one Compound Preposition
taking a Clause Complement”, and scalar equality is admitted “only inside
the duration Complement of *for*”. Retaining both conditional labels is a
spurious duplicate because the conditional meaning is not compositional from
duration. This exclusion is the project's ruling. The CGEL passages cited in
Analysis and the investigation support only their own linguistic claims.
There is no unresolved STOP.

The remaining 738 unread faces containing `as long as` remain corpus residue
for the live [systemic-residuals ticket](../planned/english-v3-systemic-residuals.md).
They are not claimed as covered. Manor Gargoyle's unrelated remaining ability
is included in that residue; no prior coverage is removed or deferred.

Restored tests: 0; re-spelled existing tests: 0; added: 6; removed: 0;
newly ignored: 0. The first four tests preserved the reported red subjects;
exact-set and expanded-marker tests strengthen that coverage. The final
reverse-dependency gate is
`cargo xtask gate --changed --from rslmnrov --run`, deriving
`cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
It exits 0: 751 passed, 0 failed, 1 pre-existing ignored test.
The inherited ignore is
`macros::templates::tests::macro_schema_census_count_matches_21`, blocked on
the on-demand live-corpus census cross-check. No existing test was weakened.
Strict clippy passes for the same closure with `--all-targets -- -D warnings`.
Scoped nightly rustfmt passes from the workspace root. Citation checks report
0 noncompliant strings and 0 stale citations; the actual piped jj diff audit
selects 0 changed CR citation sites. No CR citation registration was needed.

Deviations and additions:

- Two Constructions, `AdverbComplementPreposition` and `EquativeAdverb`, model
  distinct selected-Complement structures needed by the duration witness.
  The conditional governor uses the existing finite-clause Preposition
  constructions and their existing clause/predicate attachment breadth.
- Three lexical owners, duration/selection permissions, an expanded comparison
  marker permission and duration concord supply those structures. The expanded
  marker restriction prevents the numeric-equality marker from admitting an
  invalid duration comparison; its rejection test passes.
- The experimental `ClausalAdverb` route and standalone Adjective owner are
  retired under the ruling; there is no conditional scalar route left.
- The original four test subjects remain, with stronger exact-set duration
  coverage, plus a conditional exact-set test and expanded-marker exclusions.
- Compound Preposition already existed in the glossary for *instead*. Its
  definition is extended consistently using the read reanalysis passage.
  Glossary gaps resolved in this landing: Duration Adverb, Scalar Equality,
  Expanded Comparative Complement. They are linguistic definitions, with
  CGEL citations; no Game Model term or rule citation is introduced.

### Report: inventory and performance provenance

These inventories are stamped with change `zktwupyonwttzvqzrnynyzlkkqnzsszy`, before covered
16,977 and after covered 17,322. Named constructors: 237 → 239
(193 → 195 ordinary constructions, 44 schemas unchanged); categories: 138
unchanged; static productions: 598 → 600; compiled productions: 666 → 668.
Constructions actually traversed in the corpus: 189 → 191.
Licensed surface homographs excluding Catalog atoms: 395 → 397; added named
surfaces `as` and `As`, each with `vocab:Adverb/EquativeAs` and
`vocab:Preposition/As`. Case-folded homographs: 359 → 360, sole added surface
`as`. No former homograph is removed or altered. The complete named list is
[as-long-as-homographs.md](../../../target/english-v3/english-v3-as-long-as/as-long-as-homographs.md).
Form-literal/vocabulary overlap lists are `[]` before and after.

Performance is advisory. The quiet-host ceiling is 16,260,000,000 ns; both
complete corpus runs exceed it. The baseline overlapped compilation, and the
final host load and worker count are given below. These are active v3 census
measurements, not fitted to the retired coverage command. The before process
wall including its build was not separately measured; its setup and corpus
wall values are reported without inventing a process measurement. Final Cargo
command process wall: 583,718,213,682 ns, including any
build-directory contention. CPU telemetry is integer nanoseconds per byte.

Metric | Before: change zktwupyo, covered 16,977 | After: change zktwupyo, covered 17,322
--- | ---: | ---:
Workers | 12 | 12
Host load (1 / 5 / 15 minutes) | 8.5693359375 / 13.54833984375 / 12.07666015625 | 11.26318359375 / 18.7236328125 / 14.7177734375
Setup wall | 8,621,271,177 ns | 6,764,821,184 ns
Corpus wall | 191,570,023,244 ns | 59,493,292,994 ns
Thread CPU | 965,441,948,374 ns | 655,446,376,631 ns
Checked-text CPU telemetry | 425,022 ns/B | 283,733 ns/B
Checked text bytes | 1,523,321 | 1,563,864

Evidence is ignored under the workspace's `target/english-v3/`, preserved in
`target/english-v3/english-v3-as-long-as/` before workspace retirement. The
baseline census, final census, per-face comparison, candidate comparison,
source provenance, gain review, inventories and verification logs are retained
there. No evidence or per-task verifier enters source code or the gate.

### Refresh verification (2026-10-07)

Kata refresh exited 0 without a conflict and preserved both implementations.
It incorporated `owyyynoyxstupwposuuzymvuqwpunnyl`, the
[library complement-cluster repair](../done/english-v3-library-position.md#post-landing-fix-2026-10-06).
The original claim-parent census remains the baseline; refresh did not replace
it with a newer census. The original counts above describe the pre-refresh
tree, not the final combined tree.

The final combined tree is stamped with change `zktwupyonwttzvqzrnynyzlkkqnzsszy`,
covered 17,322, and grammar SHA-256
`269c6601cba024778c832f6302914805e85552e00c2fd1a44b2fd31e54f3dea2`.
The core lexicon source hash remains
`7d84362ae5f5d985009ced06b671c6cf2056f40b9fbac6353a798fc9b18b7579`.
Its full `--all` census has 32,828 complete faces:
15,506 no Reading, 7,111 one Reading,
10,211 multiple Readings, and
367,247 exact retained Readings. Limited,
failed, undetermined, issues, duplicate Readings and internal failures are all 0.
The lexical inventory hash remains `50a975d9d292bd934306fe90df2485e942ae298fe8630f8d0884ec89b2399210`.

Against the original claim-parent census: 345 newly covered faces, lost
faces `[]`, and previously covered faces with decreased Reading counts `[]`.
The 345 gain counts and fresh representative fingerprints are unchanged.
The library repair adds exactly 36 Readings on ten previously covered faces;
all other previously covered face counts are unchanged. These additions are
attributed to that repair, with the selected Object-plus-Locative-Complement
cluster Reading restored by `ObjectLocativeTail`:

Face | Claim-parent / pre-refresh Readings | Combined Readings
--- | ---: | ---:
Juniper Order Ranger | 6 | 7
X-23, Deadly Weapon | 6 | 7
Claim the Kingdom | 14 | 16
Stand Together | 5 | 6
River Heralds' Boon | 5 | 6
Brokers Ascendancy | 8 | 9
Captain America, Team Leader | 120 | 144
Serrated Biskelion | 5 | 6
Evolutionary Escalation | 8 | 9
Ich-Tekik, Salvage Splicer | 21 | 24

The final gate reran the same derived Cargo command and exits 0:
752 passed, 0 failed, 1 unchanged ignored test. Its one additional
inherited regression test belongs to the library repair; this ticket still
adds 6, removes 0, re-spells 0, restores 0 and newly ignores 0. Strict clippy
and scoped nightly rustfmt pass again from the workspace root. Final citation
checks have 0 stale and 0 noncompliant strings, with 0 changed CR sites selected
by the actual piped jj diff audit.

Combined-tree inventories, stamped with the change and covered count above:
240 constructor names (196 ordinary Constructions, 44 shared schemas),
138 Categories, 601 static Productions, 669 compiled Productions;
192 Constructions traversed in the corpus.
Licensed homographs remain 397 named surfaces / 360 case-folded surfaces;
form-literal/vocabulary overlaps remain `[]`. The added construction and
retired unused frame declaration belong to the incorporated library repair.
There is no additional as-long-as construction, test, lexical owner or ruling.

Performance advisory on that same combined tree: Cargo command process wall
284,063,140,451 ns (including build-directory contention),
setup wall 5,897,201,774 ns, corpus wall
72,007,041,739 ns, total thread CPU
659,490,359,675 ns, checked text
1,563,864 bytes, checked-text thread CPU
285,372 ns/B. Workers: 12;
host load (1 / 5 / 15 minutes): 3.8974609375 / 5.98095703125 / 9.60693359375.
The 16,260,000,000 ns quiet-host advisory ceiling is still exceeded; these
measurements are provenance and are not fitted to the ceiling.

Final evidence: `as-long-as-refresh-after.json`, its source provenance,
`as-long-as-refresh-census-comparison.json`, fresh gain review and inventory,
and refresh gate/clippy/fmt/citation logs, all in the ignored evidence directory
linked above. No lost Reading requires justification or re-coverage routing.

### Review notes (2026-10-07)

- **Reach.** The plain `Adverb` construction now exports `DurationUse`, so
  bare "for long" reads with 0 supported-corpus attestations. Already routed
  to [english-v3-systemic-residuals](../planned/english-v3-systemic-residuals.md) (pruning-rule candidate).
- **Remaining faces.** 738 *as long as* faces stay unread, owed to their other
  blockers (samples in the record above); already routed to the same ticket.
- **Stamp correction.** The "before" stamp reads the work change `zktwupyo`,
  but the baseline tree measured was the claim parent `ttnruzyt`.
- **Reconciliation.** 345 faces gained against the 323 sole-cause estimate; no
  shortfall.
