---
needs: [grammar-document-cleanup]
---
# Per-sentence enumeration: the re-baseline landing

Change the enumeration and validation unit of the census from the document segment to the sentence, and change nothing else in the same landing. Reading counts are today the product of the counts of a document's segments: Necromentia's sentences probe at 2, 50 and 361 readings and the paragraph has 36,100, which took 0.042 s of chart time and 13.7 s of validation. The product is an enumeration artifact, not a grammar property, and it hides where ambiguity lives (79 faces exceed 1,000 readings; the top 100 faces hold 49% of 724,921 readings).

## What the census does today

- It segments a document at Ability, not at Paragraph: `census_segments` (`crates/xtask/src/english_v3/mod.rs:331`) takes `.segments(&Category::Ability)` of the forest and reports `Category::Ability`.
- A face's count is the sum of its lines' counts, and zero if any line is unread (`crates/xtask/src/english_v3/report.rs:229`).
- Sentences are not the only material under an Ability: an activated ability carries a separate `Cost` (`crates/deckmaste_english_v3/src/declarations.rs:2675`), a keyword ability need not contain a Sentence, and a modal item has a Clause header and nested mode paragraphs (`Mode` at `declarations.rs:2987`). Empty documents and type lines already have explicit exceptions.
- `Forest::segments` unions alternatives at identical ordered spans (`crates/deckmaste_english_v3/src/forest.rs:179`); it does not prove that the readings of a unit combine as an independent Cartesian product.

## Definition

**Admission.** Parse the complete analyzed face, with its original category, lexical environment and card-name context. Decomposition uses only constituents that take part in an admitted whole-root analysis; parsing substrings on their own never establishes admission of a face. Counts are for the alternatives admitted in the original context, not all readings of the isolated substring.

**Units.** Follow the document declarations, keeping the surrounding construction structure.

| Document structure | Enumeration unit |
|---|---|
| Ordinary paragraph | Each outermost `Sentence`, including its punctuation |
| Activated ability | Whole `Cost`, then the units of its body |
| Keyword line | Whole keyword `Ability`, including parameters, coordination and supplementation |
| Dash-header modal list | Header `Clause`; each mode paragraph decomposed recursively |
| Sentence-header modal list | Sentences of the `SentenceParagraph`; named-mode bodies decomposed recursively |
| Parenthetical paragraph item | The paragraph, decomposed recursively, parentheses kept in the surrounding structure |
| Ability-word or flavor-word label | Validated surrounding structure, including the label's lexical value |
| Quoted granted ability inside a Sentence | Stays inside that Sentence; its nested sentences are not also counted |
| Type line | One whole `TypeLine` unit |
| Empty document | One whole `EmptyDocument` unit, count 1 |
| Structure that cannot safely be split | One whole residual unit, with a reported reason |

A Sentence is a stopping boundary, so quoted documents and relative clauses are never double-counted. A modal list is document structure, so its mode paragraphs are traversed before the boundary is reached.

**Independence.** A split of region R into units U1..Un is accepted only when filling a fixed, validated surrounding structure gives a bijection: Readings(R) is in bijection with Readings(U1) x ... x Readings(Un). Every combination is admitted, every old Reading is represented, and no two tuples rebuild the same Reading. Equal spans, disjoint spans and matching counts are not evidence; `Forest::segments` unions completed nodes at equal spans without proving the bijection.

**Non-independent alternatives.** Alternatives are kept as alternatives, never multiplied. If they disagree about boundaries, surrounding constructors, shared material or permitted combinations, retain the smallest enclosing region whose alternatives can be enumerated intact; if none can be established, retain the whole Ability, or the whole Document if the disagreement crosses Ability boundaries. Each retained region reports its span, category, whole count, proposed parts and one reason: `boundary-disagreement`, `correlated-choices`, `shared-material`, `alternative-wrapper` or `factorization-unproved`. Cycles and validation failures stay errors.

**Validation.** Enumerate and validate every unit Reading, and every surrounding constructor, label, separator, cost/header connection and root constraint. Check all distinct boundary-summary combinations, not one representative. Where generated admission or realization cannot be verified compositionally, retain and validate the containing region whole. Both existing roundtrip laws are preserved.

**Re-baseline equality.** In the single migration landing:

- At every accepted Paragraph split, old contextual count = product of the new part counts.
- At every old census Ability segment, the same equality holds, with cost, header and keyword-line units as factors and residual units included.
- Reconstructed old Reading identities and digests are compared as well; count equality alone could hide a substitution.
- Arithmetic is exact. Overflow, incomplete enumeration or failed reconstruction fails the migration.
- A segment that is not split is verified for unchanged count and digest and reported as "retained whole; sentence factorization not established". `N = N` there is never presented as evidence of decomposition.

**Face aggregation.** The census keeps its sum-of-enumerated-units convention over the finer units: face count = sum of the unit counts, zero when no root is admitted. It is labelled "enumerated unit Readings" and is never called a Document-tree count. An incomplete or failed run has no exact count or complete digest; any partial checked count is reported separately. Ambiguity is reported separately from the sum, since several unambiguous units are not ambiguity. The face digest hashes a versioned, unambiguous encoding of the ordered unit descriptors and tree-set digests plus the validated surrounding structure and root constraints, so a wrapper, header or boundary change changes the digest even when the unit trees do not.

**Coverage states.** Decomposition preserves syntactic admission. The report has read (admitted root) and unread (no admitted root), and incomplete enumeration as its own failure state, never put silently in either total. Until the judgment set exists, read means only admitted; once it exists (`grammar-judgment-set`), a face with admitted analyses nobody has judged counts as read and the report says plainly that this is not a correctness claim. Fragment judgments match anchored structures under the original card context; whole-face correctness is never inferred from each fragment having some acceptable reading.

**Versions and cache.** Add a decomposition version; bump the report schema and the Reading-digest encoding (8 and 1 today, `crates/xtask/src/english_v3/report.rs:486`). Old incremental results are invalidated; old aggregate counts and digests are not translated into invented sentence records. The old baseline is kept only as migration evidence. After the verified bridge, comparisons require the new baseline and matching decomposition, digest and context versions. Generated reports stay untracked.

## Required test cases

Structural probes with measured whole-tree counts (`cargo xtask english-v3 probe`), asserted with the stated accounting. The sentence-only product fails on the rows marked X.

| Text (category document unless stated) | Trees | Accounting |
|---|---:|---|
| `Destroy target creature with flying. Destroy target creature with flying.` | 4 | sentences 2 x 2 |
| `{T}: Destroy target creature with flying. Draw a card.` | 2 | cost 1 x sentences 2 x 1 |
| `Sacrifice a creature with flying: Draw a card.` (X: sentence-only gives 1) | 2 | cost 2 x sentence 1 |
| `Choose one —` / `• Draw a card.` / `• Destroy target creature with flying.` | 2 | header 1 x modes 1 x 2 |
| `Choose one —` / `• Destroy target creature with flying. Draw a card.` / `• Target creature is tapped.` | 4 | header 1 x sentences 2 x 1 x 2 |
| `Choose a creature with flying —` / `• Draw a card.` / `• Gain 1 life.` (X: mode-sentence product gives 1) | 2 | header 2 x sentences 1 x 1 |
| `Choose one.` / `• First — Draw a card.` / `• Second — Gain 1 life.` | 1 | sentence header and named-mode bodies, labels retained |
| `Ward {2}, protection from red` | 1 | one residual keyword-line unit |
| `Protection from creatures with flying` (X: empty sentence product gives 1) | 2 | one residual keyword-line unit |
| `I — Draw a card.` / `II — Destroy target creature with flying.` | 2 | bodies 1 x 2, labels retained |
| `Level up {1}` | 1 | keyword residual |
| `Level up {1}` / `LEVEL 1-2` / `2/2` / `Flying` | 0 | no root; no decomposition-based admission |
| `{1}: Level 2` / `This creature has flying.` | 0 | no root |
| `This creature has "Destroy target creature with flying. Draw a card."` | 2 | one outer Sentence unit; quotation stays nested |
| `Draw a card. (Destroy target creature with flying.)` | 2 | sentences 1 x 2, parentheses validated |
| `Legendary Creature — Human Wizard` (type-line) | 1 | whole type-line unit |
| empty text | 1 | explicit empty unit |
| `Draw a card` as sentence, then as document (X: fragment count 1) | 1; 0 | fragment admission does not imply Document admission |
| `Draw a card.` / `Destroy target creature with flying.` | 2 Document trees, unit sum 3 | face aggregation is a sum, not a product |

Necromentia: its three sentences probe at 2, 50 and 361, the old segment count is 36,100 = 2 x 50 x 361, and the new enumeration count is 413. No tested admitted ordinary multi-sentence paragraph contradicted the product; this does not establish that equal-span unions always factor.

## Change

- Enumerate and validate the units above; the census records per-unit counts and digests, and a face's count is derived from them.
- The covered and unread sets, and the lexical and chart results, stay as they are.
- Output stays under `target/english-v3/`; nothing the tool emits is tracked.
- The landing updates the census description in `crates/deckmaste_english_v3/README.md`.

## Gate

This is the one re-baseline: reading digests are recomputed per unit and so differ from the old ones by design. The check replacing digest identity is the re-baseline equality above, with the covered set and unread set identical. Nothing else may change in the landing.

## Report

Three-line note: census delta (faces, total enumerated unit Readings now counted per unit, retained-whole exceptions listed; none lost); the size line arrives with `grammar-size-report-and-ratchet`; the analysis-key and judgment-set lines do not apply yet (their instruments arrive in `grammar-analysis-key` and `grammar-judgment-set`).

## Landing note

1. **Census delta** (zupzwowr; diagnostics corrected in uktltvqm): 30,794 → 30,794 read faces; 724,921 → 482,041 enumerated unit Readings; lost faces: none. Migration: 54,339 old segments checked, 38,675 factorised; 39,890 Paragraph regions checked; 0 failures. Retained-whole exceptions: 1,449 (1,299 alternative-wrapper, 150 boundary-disagreement; factorization-unproved, correlated-choices and shared-material: 0); 13,897 keyword units are ordinary units, not exceptions; helper fallback paths: 0. Of these exceptions, 1,239 alternative-wrapper Ability units (142,904 Readings) and all 150 boundary-disagreement units (2,008 Readings) begin with labels: retained whole because label alternatives are not modelled as surrounding-structure variants. The other 60 alternative-wrapper Paragraph units (1,551 Readings) have alternative trailing-sentence attachments inside the final mode or in the enclosing Paragraph. Every exception is listed in the untracked `target/english-v3/presentence-retained-whole.json`.
2. **Size**: not applicable yet; the instrument arrives with `grammar-size-report-and-ratchet`. Grammar declarations and lexicon entries are unchanged.
3. **Judgment set and analysis key**: not applicable yet; their instruments arrive with `grammar-judgment-set` and `grammar-analysis-key`.
