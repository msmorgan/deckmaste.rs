---
needs: []
---
# Head inheritance and defaults: prototype on the Nominal constructions

First half of replacing the grammar's authoring layer. The compiler currently makes authors write the lowered form: every construction must export every feature of its category's interface exactly once (`crates/deckmaste_construction_v3_core/src/ir.rs:1288–1343` for the export equations, `:1378–1383` for the missing-export check). Complete summaries are right for the chart; requiring authors to write them is the problem. Measured on `declarations.rs`: 2,082 of 7,483 stripped lines (28%) are `export`; 838 are pure forwards (`X = child.X`), 704 of them from a field named `head`; 993 are constants, 759 of them equal to the feature's declared default (138 more on features with no default). Seven features cost 404 export lines for about eight readers (`PurposeAdjunctLicence`, `MatrixComplementLicence`, `SourceComplementSelection`, `NominalAdjunctHost`, `ClauseComplementBoundary`, `WholeObjectSourcePresent`, `BareGenitiveHost`). Exports removable by rule: 816 by inheriting one field, 759 by taking the declared default, 1,306 if each block picks one rule, 1,575 if both compose per feature. This ticket is a prototype that settles which, on a bounded slice; the rest of the file follows in `grammar-compiler-expressions-and-tables`.

Licences themselves stay at admission. The 2026-10-07 permissive-admission experiment showed that without them the corpus produced 3.77 times the Readings and 26 exploded faces; only how they are carried changes.

## Change

- A feature is declared `head` (taken from the construction's head child unless overridden) or `local` (declared default unless set).
- A construction names its head child. An export it does not write is filled by the compiler: the head's value for a `head` feature, the declared default for a `local` feature with one. A written export overrides. Features with no default and no head source must still be exported.
- The compiler elaborates this to today's lowered form and keeps its existing checks (each interface feature supplied exactly once, same domain); the chart and runtime do not change.
- Convert the 21 constructions that produce `Nominal`, and nothing else, as the prototype.

## Gate

This ticket also makes the census `diff` able to fail: today it exits 0 whatever it finds (`crates/xtask/src/english_v3/diff.rs:3`). Add a check mode that exits non-zero on a lost face, an incomplete enumeration, an incompatible baseline, or, for an identical-digest landing, any changed digest. Later tickets use it.

Every `readings_sha256` identical on every face, checked with the census check mode as an identical-digest landing. The landing updates `crates/deckmaste_construction_v3_core/README.md` for the completed compiler capability. Report in the landing: how many export lines the prototype removed, whether the one-rule or the composed rule was needed to get there, and the compiler-side tests added in `crates/deckmaste_construction_v3_core`. Note: census delta (none); the judgment-set line does not apply yet.

## Implemented design

Standard constraints apply. Features opt into `head` or `local` propagation;
`head field;` names a singular lexical or constituent field. Explicit equations
from constructions, forms, schemas and policies override inference, while
conflicting explicit exports still reject. After schema binding, an omitted head
feature copies an accessible head feature, otherwise takes its declared default;
a local feature takes only its declared default. No applicable source/default is
a compile error naming the construction and feature. Fallback is static, never a
runtime repair of an absent value. Unqualified features retain explicit exports.
The compiler README documents the semantics and inspection procedure; the English
README lists all export origins for `MeasuredSymbolNominal`.

Only the 21 ordinary Nominal constructions were converted. Their 28 existing
interface features received propagation annotations (including two builtin
annotations, not new domains). `NominalDependentPresent`,
`ScalarMeasurementHeadUse` and `PropertyValuePosition` are local; the others
inherit. No feature domain/default, lexicon, category, construction, table or
policy was added or changed beyond this authoring mechanism. Shared policies
remain for their unconverted users.

The composed rule supplies 436 head forwards and 50 defaults. Direct export
statements fell from 252 to 102; construction blocks from 423 to 271 physical
lines, and declarations.rs from 7,702 to 7,552. Six blocks grow because replacing
a policy use exposes its non-inherited changes: Noun and BareFramedNoun each
10→12; SelectedComplementNominal 14→18; SymbolComplementNominal 13→17;
PrepositionComplementNominal 31→35; PostmodifiedNominal 14→15. Their expanded
export maps and ordered forms, like those of the other 15 blocks, are unchanged.
CorrelativePostpositiveNominal has no census witness; it was included in that
structural comparison rather than claiming corpus coverage for it.

`english-v3 diff --check no-loss` rejects incompatible/incomplete evidence and
lost faces; `--check identical-digests` additionally rejects every changed count
or digest, including unread faces. The 56-face subset and full identical-digest
checks passed; an intentionally altered scratch digest produced exit 1. Added
10 compiler tests and 4 census-check tests; restored/re-spelled/newly ignored/
removed tests: 0/0/0/0. Boolean requirements and policy composition remain next;
coordination still needs per-feature combiners, and typed lexical classes must
refine lexical source accessibility. No other constructions were converted.

Census delta (otnsrlky): none — faces read 30,794→30,794; total Readings 482,041→482,041; lost faces: none; all 32,828 face digests identical.
Judgment set (otnsrlky): does not apply yet for this ticket.
