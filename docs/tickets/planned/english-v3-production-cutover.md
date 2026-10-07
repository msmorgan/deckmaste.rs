---
needs: [english-v3-systemic-residuals, english-v3-generic-frame-consumption, english-v3-scalar-cardinals, english-v3-number-transparent-concord, english-v3-by-complement-functions, english-v3-census-tractability, english-v3-selected-preposition-nominal-licensing, english-v3-licence-plumbing-economy, english-v3-census-efficiency]
---
# Cut production English consumers over to v3

Begin only after the four residual implementation batches and
`english-v3-systemic-residuals` reconciliation. That reconciliation adds any
newly measured systemic repair tickets to this ticket's `needs:`; four batches
completing does not itself establish readiness. Require the explicit remaining
identity/structure ownership and measured runtime/forest bounds at that handoff.

Make the v3 all-Readings parser, generated AST and renderer the production
Oracle English interface. Migrate corpus, inspection and plugin-facing
consumers together so no adapter silently reselects a single Reading or routes
some inputs back through v2. Keep preference as an explicit non-destructive
view where a caller needs presentation order.

Retire replaced v2 scanner, parser, selection, compiler and generated-AST paths
after transferring every live regression witness and source owner. Remove
compatibility shims that would make v3 depend on v2 types. Decide whether the
fresh construction crates take the unversioned names only after all downstream
imports use the new contract; the rename is part of this cutover if chosen.

Acceptance proves no silent corpus loss; reports new and removed Readings with
wrong-analysis retirements separated from regressions; checks both roundtrip
laws and complete traversal; and leaves one production parser route. The
supported corpus may still have named long-tail no-Reading failures, but no
consumer may use v2 to conceal them. Every unit that v2 parsed correctly and v3
does not must appear in an explicit accepted-regression list owned by the
long-tail ticket. Retiring a v2 Reading as wrong requires a grammatical witness
showing why it was invalid. Cutover also adopts the corpus-runtime and forest
growth ceiling established by `english-v3-census-tractability`, rather than
ratifying whatever the run happens to cost. Standard constraints apply.

## Prerequisites added 2026-10-05

- `english-v3-by-complement-functions`: the systemic-residuals records name the
  Avacyn complement-function defect a cutover blocker.
- `english-v3-census-tractability`: the complete run must be affordable enough
  to be the acceptance evidence.
- `english-v3-selected-preposition-nominal-licensing`: pre-existing invalid destination NP/PP groupings parse; selected noun-PP licensing is a correctness prerequisite.

## Prerequisite added 2026-10-07

- `english-v3-census-efficiency`: the complete run is the acceptance evidence,
  so its memory must stay bounded as Readings grow, and runs must be comparable by
  per-face digest.

## Legacy baseline

The retired v2 parser's per-identity coverage lock is recoverable with `jj --no-pager file show -r zwnwwmlp- english-v2-coverage.lock` (schema 4, 19,953 covered identities). How this ticket's accepted-regression list is measured is undecided and is not needed until cutover is in sight.

## Legacy consumers to migrate

This inventory was compiled on 2026-10-06 by grepping for `deckmaste_english`
imports and types outside that crate (`rg -l 'deckmaste_english::' crates
--glob '!crates/deckmaste_english/**'` and `rg -n 'deckmaste_english\b'
crates/*/Cargo.toml`). Paths below are relative to `crates/`. No migration
has been made yet.

| File | Legacy contract consumed | V3 replacement or remaining gap |
|---|---|---|
| `xtask/src/english/data.rs` | `Catalogs`, legacy source normalization, supported derived face loader | `raw_corpus` durable identities and `deckmaste_lexical_source` environment; keep raw and analyzed input distinct |
| `xtask/src/english/bracket.rs` | `ParseReport`, `Span`, parse provenance and constituent spans | Generated `Reading::visit` plus lexical/source occurrences; no direct generated constituent-span bracket API |
| `xtask/src/english/inspect.rs` | Parsed AST, selection/cost/construction evidence, hand-walked clause and phrase types, normalization tests | `english-v3 probe`, `Reading`, `visit` / `visit_words`, explicit Construction Cost preference; port every live assertion rather than deleting it |
| `xtask/src/english/unknown_phrases.rs` | Recovery roles/text and selected AST | V3 no-Reading/unknown-word census; no recovery AST exists |
| `xtask/src/english/recovery.rs` | Recovery, opacity and packed-selection census | V3 complete Reading census, unknown words and chart/materialization counters; no opacity/recovery fallback |
| `xtask/src/english/recovery_fingerprints.rs` | `DiagnosticLimits`, failure stages/categories, frontier/feature/rejection fingerprints | V3 lexical alternatives and packed forest inspection exist; bounded rejection/frontier diagnostic fingerprints have no direct equivalent |
| `xtask/src/english/recovery_worklist.rs` | Indirect recovery-group worklists from the legacy census | Identity-based v3 residual evidence; retain causal annotations without pretending recovery groups are Readings |
| `xtask/src/english/roundtrip.rs` | `parse_with_identity`, selected `OracleText::render`, normalized-source equality | Existing v3 per-Reading admission, exact realization and traversal validation; preserve independently constructed-value law witnesses too |
| `xtask/src/english/shape.rs` | Indirect serde traversal of legacy AST types | Generated `Reading::visit` / `visit_words`; v3 has no serde AST serialization contract |
| `xtask/src/english/shapes.rs` | Legacy AST production-shape frequency | V3 census construction counts and full Reading trees; ranking must retain all Readings |
| `xtask/src/english/lint.rs` | Legacy shape traversal, noun/verb identities, typed AST assertions | Port sound diagnostics to generated v3 features/trees; no equivalent ready-made lint set |
| `xtask/src/english/probe/facts.rs` | Parse selections, ties, chart/forest maxima and recovery facts | V3 retained-Reading counts, chart counters and validation traces; ties are valid Ambiguity |
| `xtask/src/english/probe/invariants.rs` | `ConstructionId` / `Span` test fixtures and selection-tie diagnostics | V3 structural identity and corpus invariants; retire tie-as-error policy under the lexical-analysis decision |
| `xtask/src/english/probe/calibrate.rs` | Indirect legacy corpus facts used to derive percentile ceilings | V3 complete census/forest measurements; the adopted forest bounds belong to `english-v3-census-tractability` |
| `xtask/src/english/probe/mod.rs` | Indirect legacy parsing and probe orchestration through facts | `english-v3 probe` plus corpus validation |
| `xtask/src/english/performance.rs` | `ParseReport`, selected chart/forest work, parser timing | Existing v3 census telemetry and packed-forest bounds; maintain deterministic local work checks |
| `xtask/src/english/mod.rs` | Legacy command dispatch | One v3 route after all commands and their live witnesses migrate |
| `xtask/src/macros/inspect.rs` | `Catalogs` / `FragmentKind`, spelling-frame compilation and projected patterns | V3 environment and start `Category`; typed frame-pattern projection has no direct v3 equivalent |
| `xtask/src/macros/pilot.rs` | `parse_fragment`, `project_fragment`, clean selected projection; spelling render/recovery gates | Enumerate admitted v3 fragments; migrate spelling projection and rendering before these gates can switch |
| `xtask/src/macros/residuals.rs` | Legacy semantic renderer output parsed as `Fragment`, projected/unified residuals, draft-frame compilation | V3 fragment parsing exists; frame-pattern projection/unification is still missing. Do not confuse `deckmaste_legacy_render` (semantic rendering) with the English parser renderer |
| `deckmaste_spelling/src/compile.rs` | `FragmentKind`, `parse_fragment`, `project_fragment`, `ProjectedAtom`, features, noun/verb/scalar downcasts and numeral witnesses | Start `Category` and generated v3 trees/lexical features exist; typed frame holes, projection paths and witness substitutions need a new v3 implementation |
| `deckmaste_spelling/src/projection.rs` | `ConstructionProjection`, `ProjectedValue` / `ProjectedAtom` / member/variant/product/witness data, including owned value downcasts | No v3 generic role-addressed projection equivalent; generated traversal alone does not supply frame substitution |
| `deckmaste_spelling/src/lexicon.rs` | `Catalogs`, `FragmentKind`, category-indexed compiled frames, `FrameKind` conversion | V3 lexical environment and generated `Category`; frame registry/index conversion must migrate |
| `deckmaste_spelling/src/unify.rs` | Projected trees/atoms/owned values, legacy pronouns, noun/determiner/scalar downcasts, vocabulary and parser tests | V3 Lexical Readings/features and generated AST; no ready-made typed frame unifier |
| `deckmaste_spelling/src/render.rs` | `parse_fragment`, `project_fragment`, `render_fragment`, `Numeral`, noun vocabulary, owned projection values | Generated v3 realization exists; structural hole filling / reassembly and residual rendering need migration |
| `deckmaste_spelling/src/witness.rs` | `Numeral`, legacy fragment/self-reference witness conventions | V3 lexical numeral readings and explicit context/identity declarations; transfer authentic frame witnesses |
| `deckmaste_spelling/tests/pilot.rs` | `Catalogs`, `FragmentKind`, spelling lexicon/frame/roundtrip integration tests | Re-spell against migrated v3 frames with the same semantic outcomes |
| `xtask/src/english_v3/comparison.rs` | Explicit audit-only legacy parser/AST/catalogs/normalization/provenance | Archive the baseline evidence and retire this legacy-dependent comparison after cutover acceptance; it is never a production fallback |

`Cargo.toml`, `xtask/Cargo.toml` and `deckmaste_spelling/Cargo.toml` retain the
legacy package/dependency edges. `macro_ron/src/frames.rs` names
`deckmaste_english::FragmentKind` only in a comment: its independent `FrameKind`
schema needs updated category correspondence, not removal of a Rust dependency.
`xtask/src/gate.rs` contains legacy source paths only as changed-path fixtures;
re-spell them if the paths retire. `english/recovery_fingerprints.rs`'s worklist
helper and `english/shape.rs`'s users are included despite their indirect
consumption. `scripts/dump_english_brackets` invokes the legacy bracket command and must
switch with that command. `deckmaste_spelling/src/view.rs` and `guard.rs` process semantic
values without importing legacy parser types; they are not extra parser routes.

Unfinished tickets that leave the parser implicit: `builtin-v2-spelling-frame-consumer`
("generated English AST"), `builtin-v2-english-card-consumer` (closure-wide
parser environment), `spelling-engine-requirements` (English AST),
`plugins-v2-canon` (English parser), and the design-gated
`semantics-v2-english-translation` (raw English AST). Resolve these references
to the retained-Reading v3 contract when they are claimed. The
`parse-subject-filter-stringly-channels` mention concerns the separate semantic
`filter::parse_phrase` path; it is not a `deckmaste_english` consumer.
