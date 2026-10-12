---
needs: []
---
# Head inheritance and defaults: prototype on the Nominal constructions

First half of replacing the grammar's authoring layer. The compiler currently makes authors write the lowered form: every construction must export every feature of its category's interface exactly once (`crates/deckmaste_construction_v3_core/src/ir.rs:1288–1343` for the export equations, `:1378–1383` for the missing-export check). Complete summaries are right for the chart; requiring authors to write them is the problem. Measured on `declarations.rs`: 2,082 of 7,483 stripped lines (28%) are `export`; 838 are pure forwards (`X = child.X`), 704 of them from a field named `head`; 993 are constants, 759 of them equal to the feature's declared default (138 more on features with no default). Seven features cost 404 export lines for about eight readers (`PurposeAdjunctLicence`, `MatrixComplementLicence`, `SourceComplementSelection`, `NominalAdjunctHost`, `ClauseComplementBoundary`, `WholeObjectSourcePresent`, `BareGenitiveHost`). Exports removable by rule: 816 by inheriting one field, 759 by taking the declared default, 1,306 if each block picks one rule, 1,575 if both compose per feature. This ticket is a prototype that settles which, on a bounded slice; the rest of the file follows in `grammar-compiler-expressions-and-tables`.

Licences themselves stay at admission. The 2026-10-07 permissive-admission experiment showed that without them the corpus produced 3.77 times the Readings and 26 exploded faces; only how they are carried changes.

## Change

- Each Category interface entry is `head` (taken from the construction's head unless overridden), `local` (declared default unless set), or unqualified (strict explicit export).
- A field named `head` supplies the head by convention; `head other;` overrides it and `head none;` declares headlessness. Head-mode entries require a head or directive even with complete explicit exports. A constituent head supplies features exposed by its Category, with static declared-default fallback otherwise. Lexical heads may inherit only features with declared defaults; others require explicit exports. Headless constructions must explicitly export every head-mode entry. Local entries use their declared defaults. Explicit exports override inference.
- The compiler elaborates this to today's lowered form and keeps its existing checks (each interface feature supplied exactly once, same domain); the chart and runtime do not change.
- Convert the 21 constructions that produce `Nominal`, and nothing else, as the prototype.

## Gate

This ticket also makes the census `diff` able to fail: today it exits 0 whatever it finds (`crates/xtask/src/english_v3/diff.rs:3`). Add a check mode that exits non-zero on a lost face, an incomplete enumeration, an incompatible baseline, or, for an identical-digest landing, any changed digest. Later tickets use it.

Every `readings_sha256` identical on every face, checked with the census check mode as an identical-digest landing. The landing updates `crates/deckmaste_construction_v3_core/README.md` for the completed compiler capability. Report in the landing: how many export lines the prototype removed, whether the one-rule or the composed rule was needed to get there, and the compiler-side tests added in `crates/deckmaste_construction_v3_core`. Note: census delta (none); the judgment-set line does not apply yet.

## Revised design

Standard constraints apply. Propagation belongs to `(Category, feature)` pairs;
only `Nominal` is annotated. Features retain their domains and defaults, and every
unqualified interface remains strict. The three local entries are
`NominalDependentPresent`, `ScalarMeasurementHeadUse`, and `PropertyValuePosition`;
the other 25 inherit. Each form fills independently against its concrete head
Category after policy expansion and schema binding. Explicit exports override
inference; duplicate explicit exports still conflict. Missing exports name the
Construction and feature. No Reading shape, field order, form index, admission,
chart or runtime changes. The compiler README states the rules; no explain tool
or maintained export inventory is needed.

The missing-head check initially rejected shared coordination instances producing
Nominal. Work stopped for the scope ruling. The owner authorized required headless
declarations without converting these instances: `head none;` was added to exactly
two schemas, `SerialCoordination` and `Coordination`. All their exports, policies
and forms remain unchanged. The directive applies to all instances and has no
effect for interfaces without head-mode entries. No other shared declaration
needed this escape.

Only the 21 ordinary Nominal constructions were converted. All 21 redundant
`head head;` directives are gone. The five lexical-headed constructions (Noun,
BareFramedNoun, SelectedComplementNominal, SymbolComplementNominal,
PrepositionComplementNominal) now explicitly forward builtin `number` and
`countability`, which lack defaults. The trial removing their
`NominalAgreementNumber = Head` and `ObliqueNumber = None` exports passed the
56-face subset check: all 1,364 Readings and digests identical. Those ten exports
remain removed. No lexicon, domain/default, construction, category, table or
policy was added. Zero users were confirmed before deleting
UnmodifiedNominalProperties, PostmodifiedNominalProperties and
ParticipialNominalHeadFeatures; NominalHeadFeatures and NominalHeadProperties
remain for their schema rows.

Physical lines: declarations.rs 7,702 originally → 7,552 at first review → 7,456;
the 21 construction blocks 423 → 271 → 250. Direct exports 252 → 102 → 102.
Five blocks remain longer than their original policy-based spelling because
substantive policy changes are now visible directly: Noun 10→11,
BareFramedNoun 10→11, SelectedComplementNominal 14→17,
SymbolComplementNominal 13→16, PrepositionComplementNominal 31→34.
CorrelativePostpositiveNominal has no corpus witness; compiler/source checks,
rather than a claimed witness, cover its conversion.

Baseline provenance: `target/english-v3/inherit-before.json` records change ID
`qplsoopn`, which subsequently became the compiler commit. The completed census
was captured in that change's clean pre-implementation working tree, before any
compiler edits or propagation annotations; it was not captured after annotations.
Its SHA-256 remains
`c0857837d12302264f486b629cf68242522a2ce32b9a04531934acb3a3162da4`.
This is a sound pre-change baseline: all exports were explicit and the complete
32,828-face report has unlimited, complete enumeration and no validation issues.

The census check additionally requires equal `selection_status`; both complete
reports and matching subsets remain supported. Boolean requirements and policy
composition are the next authoring work. Coordination still needs per-feature
combiners; headless coordination keeps its explicit policies meanwhile. Typed
lexical classes should refine lexical source availability, including builtin
features currently requiring explicit forwards.

Validation: the revised full census (`inherit-revised-after.json`) and
`cargo xtask english-v3 diff inherit-before.json inherit-revised-after.json
--check identical-digests` passed over all 32,828 faces (paths under
`target/english-v3/`), with zero incomplete enumerations or validation issues.
Full tests passed: construction core 60, construction macro 44, English 1,165,
xtask 439. All four crates passed clippy with `--tests -- -D warnings`.
Citation checks: zero noncompliant, zero stale, no changed citation sites.

Assurance: the ticket adds 16 compiler tests and 5 census-check tests in total.
This revision adds 6 compiler tests and 1 check test, and re-spells the earlier
10 compiler tests for Category modes and the revised head contract. Restored,
newly ignored and removed tests: 0 each. The existing English mana-symbol
verbless-subject blocker and xtask live-corpus census ignore remain unchanged.
No assertions were weakened.

Census delta (utxtvuto): none — faces read 30,794→30,794; total Readings 482,041→482,041; lost faces: none; all 32,828 face digests identical.
Judgment set (utxtvuto): does not apply yet for this ticket.
