---
needs: [english-predicate-frames]
design: true
---
**[design] Drive supported-corpus English structural recovery down by
family-scoped grammar coverage, and hand the remainder to named successors.**
Use the role/token census established by `english-structural-coverage` as the
live queue. A face is covered when it parses without whole-sentence/clause,
modal-header, embedded-rules, activation-cost, keyword-argument, or nominal
recovery. Licensed lexical opacity may remain only at a documented lexical
boundary and is reported separately.

**The bar was zero. It is now at least 90% of supported faces parsing with no
structural recovery — see "Scope change".**

## Remaining migrations

- Replace the `protection from` / `affinity for` name checks and first-token
  plausibility heuristic with declarative keyword-argument signatures. Cover no
  argument, quantity, structured cost or symbol sequence, noun phrase, PP with
  permitted preposition, and embedded ability shapes, both alone and in
  comma/semicolon-separated keyword lists. Unsupported shapes recover at the
  keyword-argument role.
- Replace raw-string `Cost::SymbolList` with the existing structured
  `OracleSymbol`/symbol-sequence representation, and let quoted or embedded
  abilities occupy licensed grammatical slots instead of special-casing only
  `with "..."`.
- Record adjective comparison capability/kind in vocabulary metadata instead of
  matching `fewer | greater | less | more | other` in grammar control flow.
- Carry canonical lemmas/forms of CR-derived keyword actions in catalog or
  vocabulary metadata instead of rewriting only `prepared` during resolution.
- Audit named regular `Vocab` entries after the metadata migrations and retain
  stable variants only for grammar semantics or irregular forms. Coordinate a
  mechanical module split with `english-split-word-module`.
- Confirm no tooling consumes the inactive `ast.rs`, `parser.rs`, `render.rs`,
  `source_debug.rs`, or `catalogs.rs` tree, then delete it.

## Scope change

The completion bar moved from zero structural recovery to a bounded
threshold. This was a measured decision, not an abandoned one.

Every round of this campaign worked the same way: take a coherent family of
unresolved rows, add one typed production or repair one invariant, land it.
That technique drove the census down throughout and is not exhausted in
principle — but the residue it leaves is not more families. Nearly every
remaining face carries a recovery span whose text occurs nowhere else in the
supported corpus, so there is no shared construction left to target, and
family-scoped work has a hard ceiling far below the remaining total.
`english-structural-recovery-long-tail` carries that measurement, the ceiling
argument, and the scoping guidance for what replaces this technique.

What remains is a different problem: long sentences chaining constructions
that each already parse alone. Closing it needs composition machinery —
coordination generalization, feature gates, a keyword-rider field — not
further construction coverage. That work is delegated to the successor
tickets rather than continued under this claim.

Two campaign-wide corrections belong with the change. The noun-opacity
instrument counted head position only and was blind to modifier position;
the repair means opacity figures recorded before it are not comparable with
anything after, and the previously "frozen" cell was never measuring the
whole debt. And registering a grammar production can perturb unrelated
parses through discovery-order tie-breaks — see the §A analysis referenced
from `english-reduced-recipient-passive-relative`; a categorical gate that
can be expressed at dot 1 belongs in `accepts_prefix`, never at reduce.

Round-by-round history, per-round attribution, and all before/after figures
live in the harness done-record, deliberately outside the repository.

## Successors

Every ticket below was minted on this claim line. The completion criteria
require that no residue is left unowned, so this list is the check: each
diagnosed class is here or in the harness done-record.

**Structural-recovery residue.** Family-shaped, each carrying witnesses and
usually a root cause; any is pickable as a normal round.
`english-ability-derived-verb-batch`, `english-coin-flip-residue`,
`english-coordinated-trigger-conditions`, `english-coordination-agrees-audit`,
`english-coordination-residue`, `english-except-for-np-riders`,
`english-keyword-ability-parameters`, `english-keyword-action-verbs`,
`english-keyword-grant-arguments`, `english-negated-contracted-copular`,
`english-opacity-residue-families`, `english-opaque-head-modifier-ban`,
`english-predicate-coordination-residue`,
`english-predicate-coordination-redesign`, `english-quantifier-float-residue`,
`english-reduced-recipient-passive-relative`, `english-single-root-lowering`,
`english-voting-procedure`.

**The long tail.** `english-structural-recovery-long-tail` — not a family,
but the measurement and argument that family-scoped work is near its ceiling,
plus scoping guidance for the composition machinery that must replace it.
Read it before planning further work under this claim.

**Machinery and representation.** `english-agent-noun`,
`english-ast-grouping`, `english-closed-class-tables`,
`english-grammar-syntax-key-mirrors`, `english-literal-surfaces-derivation`,
`english-predicate-generics`, `english-recovery-walker-derive`,
`english-semantic-ir` (retired 2026-07-31: the IR direction is dead),
`english-surface-fact-diet`, `english-test-structural-assertions`.

**Hygiene and tooling.** `ability-word-catalog-regen`,
`english-split-grammar-modules`, `split-resolve-tests`; plus scope added to
the pre-existing `english-split-word-module`.

**Completed on this line and moved to `done/`.**
`english-contracted-subject-recipient-passive`,
`english-gift-keyword-argument`, `english-has-known-word-literal-audit`,
`english-imperative-under-clause`, `english-noninitial-trigger-clauses`.

## Campaign workflow

Take coherent, high-frequency grammar families from the live census. Each
completed slice must add a reusable typed production, populate shared metadata,
or document a genuine lexical exception, with causal positive/negative tests,
source-free rendering, and lower role/token totals. Before/after snapshots stay
ephemeral; no card text, identities, hashes, or aggregate baselines belong in
the repository.

Do not reach zero by splitting recovery spans, adding opaque semantic leaves,
or weakening lexical-slot constraints. Measure release-parser behavior when
chart search changes materially.

## Completion

- **At least 90% of supported faces parse with no structural recovery in any
  reported role**, and every construction family still reachable by a
  family-scoped round has either landed or been handed to a named successor
  ticket with its diagnosis. Reaching the threshold by splitting recovery
  spans, adding opaque semantic leaves, or weakening lexical-slot constraints
  does not count.
- No residue is left undiagnosed and unowned: each remaining class is either
  in a successor ticket or recorded in the harness done-record.
- Keyword arguments and the remaining lexical facts use declarative shared
  metadata, and the inactive parser tree is gone.
- Supported-card round-trip remains an independent source-free invariant and
  required corpus loading remains enforceable.
- English, xtask, release-corpus, workspace, formatting, and clippy gates pass
  under the project's deliberate unboxed-grammar-enum policy.
