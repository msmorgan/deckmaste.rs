---
needs: []
---
# Declare *combat* a turn-interval noun: beginning of combat, until end of combat, each combat

## Why

Bare *combat* as a game interval never reads. On change `xxknlzypsnwy` (32,828
supported faces, 17,322 covered, 15,506 unread; recon of 2026-10-07),
*beginning/end of combat*, *during/each/this combat* touch **637** unread faces
and are the sole cause on **202** (recon bucket "at end of combat / combat
step-phase": 683 / 202; *beginning of combat* alone is 307 touched / 112 sole).
Counts are unread faces *touched* (at least one localised failing unit matches)
/ *sole* (every failing unit matches and no other recon STRONG bucket does).
They are surface counts, not gain forecasts.

Probes (admitted roots): "At the beginning of combat, draw a card." 0; "At end
of combat, draw a card." 0; "During combat, draw a card." 0; "This creature can
block an additional creature each combat." 0. The *turn* counterparts read:
"Draw a card at end of turn." 1; "Draw a card each turn." 1; "Draw a card
during each combat." 1. In `core.ron` `lexeme:CommonNoun/Turn` declares
`NominalBareClass: Interval` and `NominalAdjunctClass: Temporal`;
`lexeme:turn_part/combat` declares only `NounPremodifier: Yes`. The
constructions exist; *combat* lacks the declared class that licenses them.

## Goal

*combat* reads wherever *turn* reads as a game interval: bare in the
*of*-Complement of *beginning*/*end* (*at the beginning of combat*, *until end
of combat*), bare as the Complement of *during*, and as a determined temporal
NP Adjunct (*each combat*, *this combat*). The change is a feature declaration
on the existing lexeme, consumed by the existing routes; no new construction.
*At the beginning of combat on your turn* must keep the *on your turn* PP
attachment that the turn-based triggers already have (record its Reading
count).

## Analysis

Some singular count nouns occur without a determiner in fixed expressions or
frames, including expressions of time: *at dawn*, *by daybreak*, *before
sunrise* (CGEL, Ch. 5, §8.5(b), p. 409, [20v]). §8.5 treats such bare NPs
under restricted non-referential interpretations.
Treating *combat* as such a noun alongside *turn* is the project's lexical
classification, not a CGEL ruling about this word. The rules name the combat
phase's first and last steps *beginning of combat* and *end of combat*
[CR#506.1]; the Oracle strings are still compositional (*the beginning of
combat*, *until end of combat*), so do not add atomic step-name lexemes that
would duplicate the compositional analysis (Method 8).

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Dire Fleet Warmonger: "At the beginning of combat on your turn, you may
  sacrifice another creature."
- Might of the Ancestors: "At the beginning of combat on your turn, target
  creature you control gets +2/+0 and gains vigilance until end of turn."
- Glyph of Destruction: "Target blocking Wall you control gets +10/+0 until end
  of combat."
- Phantom Whelp: "When this creature attacks or blocks, return it to its
  owner's hand at end of combat."
- Echo Circlet: "Equipped creature can block an additional creature each
  combat."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-combat-interval-noun-before.json` on
   the claim parent, stamped with its change id and covered count; after: the
   same command to
   `target/english-v3/english-v3-combat-interval-noun-after.json` on the final
   tree. Evidence lives under the workspace's ignored `target/english-v3/`,
   never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *combat* read as a Premodifier of a following noun when it is the
   head of the interval NP, or *on your turn* attached inside *combat* when it
   modifies the trigger, is a defect. A wrong analysis that parses is a defect,
   not a gain.
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

- The *combat phase* atomic-vs-composite duplicate (Moment of Silence and three
  others): `english-v3-systemic-residuals`. Do not widen it; if this ticket's
  declaration adds a third analysis of *combat phase*, that is a defect.
- *combat damage* (premodifier use, already declared), step names (*declare
  blockers step*), *additional combat phase*.

## Landing record

In addition to the standard record: the feature declaration made and every
construction that consumes it; Reading counts of each witness, and of the four
*combat phase* faces before/after; before/after counts stamped with change ids;
timings as integer ns and ns/B with host load and worker count.


### Landing — 2026-10-07

Standard constraints apply. All evidence is in this workspace's ignored
`target/english-v3/`. The complete before/after commands use `--all --workers
12 --samples-per-face 0`, without a Reading limit. Both reports are stamped
`wvmukxorkwyxxpsnkzvollmqxokyyqlv`: before / covered 19,205; after / covered
19,414. Their lexical-inventory digests distinguish the measured states. Before
contains only the new tests relative to claim `ltquxnqqyrxu`; its runtime
sources are identical to that claim parent. Final refresh attribution is
recorded below.

#### PROVE

Coverage rises from 19,205 to 19,414 of 32,828 supported faces: 209 gains,
zero lost faces, zero Reading-count decreases on previously covered faces.
Total Readings rise from 543,312 to 549,892. Of the 6,580 additions, 6,565
belong to newly covered faces and 15 to three already-covered faces:

- Kemba's Legion: 2 → 11; the existing Object readings survive, with temporal
  Adjunct readings of *each combat* added. The inherited *for each Equipment*
  attachment alternatives remain, including the nominal restrictor.
- Fearless Swashbuckler: 4 → 8; *this combat* gains its temporal Adjunct
  function alongside the surviving Object readings of *attacked this combat*.
- Flummoxed Cyclops: 1 → 3; *this combat* gains its temporal Adjunct function
  alongside the surviving Object reading of *block this combat*.

`census-delta.json` names every gain and changed Reading count.
`reading-increase-audit.json` contains every before/after tree for these three
faces, using the base lexical features in an independent in-memory Lexicon.
No silent loss, retirement, regression or re-coverage debt is introduced.

The full census and the complete all-gains sample census report zero issues,
internal failures, duplicate Readings, admission failures, byte-exact
realization failures, lexical-ownership failures and construction/leaf
traversal failures. The independent-value test checks both roundtrip laws and
exact Word identity for *combat*, *during combat*, *until end of combat*,
*this combat* and *each combat*. Each constituent has exactly one Reading.
The five witness tests retain *combat* as the interval head, and the Fog
control retains its noun Premodifier in *combat damage*. A bare interval does
not become an ordinary determinerless singular Object.

Word-named licensing guards added: zero. Only existing lexical features are
assigned; Lexical Environment loading and GrammarEnvironment construction
succeed. V3 emits neither the legacy coverage lock nor its permitted-checker
total; the complete V3 census is the covered-count authority here.

Assurance: added 8 tests; restored 0; re-spelled 0; ignored 0; removed 0.
All five witness tests and the *during combat* test first fail on the base;
the independent-value test also first fails. Existing tests and value
comparisons are preserved. The focused green run passes 11 tests, including
the three unchanged nominal-adjunct tests.

#### DISCLOSE

The census is 7,233 unique / 11,972 multiple before and 7,245 unique / 12,169
multiple after. Specificity resolution is not V3 admission or a V3 census
field; Construction Costs are unchanged. Multiple grammatical Readings remain
admitted.

Every newly covered identity, complete Reading count, selected Reading
identity and cost is listed in `newly-covered-readings.md` and
`newly-covered-samples.json`. `selected-analysis-audit.json` records the
selected analysis's interval constructions and realized constituents for all
209 gains. The following 11 newly covered faces were spot-checked; counts are
whole-face counts stamped with the after tree / covered 19,414 above:

| Face | Readings | Interval analysis |
|---|---:|---|
| Dire Fleet Warmonger | 4 | Bare interval in *of combat* under *beginning* |
| Might of the Ancestors | 4 | Bare interval in *of combat* under *beginning* |
| Glyph of Destruction | 40 | Bare boundary PP *until end of combat* |
| Phantom Whelp | 6 | Bare boundary PP *at end of combat* |
| Echo Circlet | 2 | Nominal temporal Adjunct *each combat* |
| Silent Assassin | 2 | Bare boundary PP *at end of combat* |
| Luke Cage, Hero for Hire | 1 | Bare interval in *of combat* under *beginning* |
| Mindwrack Harpy | 1 | Bare interval in *of combat* under *beginning* |
| Mongrel Pack | 1 | Bare interval Complement in Adjunct *during combat* |
| Ma Chao, Western Warrior | 3 | Nominal temporal Adjunct *this combat* |
| Heat Stroke | 4 | Bare boundary PP *At end of combat* |

The trigger PP *At the beginning of combat on your turn* has exactly one
Reading; Dire Fleet Warmonger's first sentence also has one. Its *on your
turn* PP modifies the boundary nominal, outside the bare *combat*
Constituent. Initial test expectations incorrectly required the shorter
*At the beginning of combat* to be a constituent; they were corrected to the
inherited whole-PP boundary. No existing test was weakened, and no *on*
licence or attachment was changed. The open *on* question in the 2026-10-06
project ruling remains open; no ruling was inferred from CGEL.

The four *combat phase* controls retain their full counts: Moment of Silence
2 → 2; Stonehorn Dignitary 2 → 2; False Peace 3 → 3; Empty City Ruse 3 → 3.
The existing atomic/composite duplicate remains owned by
`english-v3-systemic-residuals`; this ticket adds no third analysis.

Lexical declaration and consumers:

- Existing `lexeme:turn_part/combat` receives `NominalBareClass: Interval`
  and `NominalAdjunctClass: Temporal`; `NounPremodifier: Yes` remains.
- Existing `vocab:Preposition/During` receives `NominalBareClass: Interval`.
  Its existing Adjunct and preposed-Adjunct licences are unchanged.
- `BareIntervalNominal`, `BareBoundaryComplement`, `BareBoundaryNominal` and
  `BareTemporalPreposition` compose the existing bare-interval route.
  `NominalAdjunctPhrase` and `NominalAdjunctPredicate` consume the temporal
  licence through existing nominal feature propagation and adjunct tables.

Deviations and additions:

- The noun-only proposal omitted *during*'s required matching bare-interval
  feature. The noun-only test run confirms that omission; the existing
  feature is assigned to the existing preposition, with Basandra's attested
  sentence and Mongrel Pack's newly covered face as witnesses.
- Added the *during combat* test, the Fog Premodifier/ordinary-NP controls and
  independently constructed interval and determined-adjunct values beyond
  the five witness tests. They check the declared lexical distributions and
  both roundtrip laws.
- Features added: none. Tables added: none. Policies added: none.
  Constructions added: none. Frames, Lexemes and Word Forms added: none.
  No route is superseded or retired, and no unreachable declaration is
  introduced on either the lexicon or grammar side. Determined NPs and bare
  intervals remain disjoint routes, with no duplicate label added.
- CGEL, Ch. 5 §8.5(b), pp. 409–410 supports the restricted bare-NP pattern.
  Classifying *combat* as an interval noun alongside *turn* is this ticket's
  project classification, not a CGEL judgment about that word.

STOPs: none. No recorded ruling is contradicted or invalid gain accepted.
No glossary gap arose: the existing interval/adjunct features and linguistic
terms are reused; no new public concept is named.

#### REPORT

Declaration inventory, stamped with the before/after trees and covered counts
above: 204 ordinary Constructions + 43 shared schemas = 247 in both trees;
612 static / 681 compiled Productions in both. Non-blank lines of
`crates/deckmaste_english_v3/src/declarations.rs`: claim parent 3,227; measured
after 3,227; net change 0. The declaration file is untouched by this ticket.

`combat-inventories-before.json` and `combat-inventories-after.json` contain
identical complete spelling-to-owner maps and named homograph lists: 38,375
spellings, 1,128 exact-case homographs, 1,088 case-folded homographs.
Form-literal/vocabulary overlap inventories are empty in both. No spelling or
lexical owner was added or retired.

Performance advisory; all figures carry the corresponding whole-corpus
change-id / covered stamp above. Host load is one/five/fifteen minutes:

| Tree | Workers | Host load | Corpus wall ns | Checked-text thread CPU |
|---|---:|---|---:|---:|
| Before / 19,205 | 12 | 9.13/48.10/60.63 | 66,425,150,670 | 316,413 ns/B |
| After / 19,414 | 12 | 8.36/18.33/42.43 | 112,410,711,867 | 439,043 ns/B |

Both exceed the 16,260,000,000 ns quiet-host ceiling. The after run overlaps
gate compilation; these loaded-host measurements do not isolate the feature's
cost. The all-gains validation run takes 3,662,228,627 ns at 12 workers,
host load 21.49/21.10/39.47, with 482,277 ns/B checked-text thread CPU,
stamped with the after tree / whole-corpus covered 19,414.

#### Verification and refresh

Complete census commands:

```sh
cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-combat-interval-noun-before.json
cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-combat-interval-noun-after.json
cargo xtask gate --changed --from ltquxnqqyrxu --run --clippy
```

The derived gate is:

```sh
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Focused tests and `cargo fmt --check -p deckmaste_english_v3 -p
 deckmaste_lexical_source` pass. Citation checks report zero noncompliant
sites and zero stale citations; no citation was added or changed.

The derived reverse-dependency test command passes in full, including xtask's
corpus and Lean integration tests. Strict clippy also passes. `gate.log`
records both commands and their complete output.

Final refresh imports trunk `sumqouus` (the docs-symlink ignore fix) and
coordination metadata. It changes no English runtime source or consumed
corpus data, so the refreshed base is runtime-identical to the measured
before tree / covered 19,205. No face or Reading decrease arrives from
refresh. The measured after tree / covered 19,414 and its gate remain valid.
The final declaration file is byte-identical to the refreshed claim parent:
3,227 non-blank lines → 3,227, net 0. Final review finds only this ticket's
two lexical assignments, eight tests and landing record above the claim.

### Review notes (2026-10-07)

- **Overgeneration.** Adding the Interval class to *During* lets
  "During turn, draw a card." read (ungrammatical; 0 corpus faces), because
  noun and preposition agree on one shared class with no pairwise restriction.
  Routed to [english-v3-systemic-residuals](../planned/english-v3-systemic-residuals.md).
- **Reconciliation.** 209 faces gained against the 202 sole-cause estimate.
  The estimate was stamped at change `xxknlzypsnwy`, covered 17,322; later
  landings cleared co-causes, so the surplus is expected.
- **Cite correction.** The Analysis paragraph previously attributed the
  "standard referential denotation" remark to [20v]. On p. 409 that remark
  concerns [20i] (*in hospital*, *in bed*), not the [20v] times; the sentence
  now cites only the section's non-referential heading.
