---
needs: [grammar-judgment-set]
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

Every `readings_sha256` identical on every face. Report in the landing: how many export lines the prototype removed, whether the one-rule or the composed rule was needed to get there, and the compiler-side tests added in `crates/deckmaste_construction_v3_core`. Three-line note: census delta (none); size delta per declaration kind from the size tool; judgment-set result.
