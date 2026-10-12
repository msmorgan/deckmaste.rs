---
needs: []
---
# Grammar document cleanup: delete the misleading documents, replace the governing record, shelve the old queue

Documentation and ticket housekeeping only; no code changes. This is the first step of the grammar consolidation. The memory cleanup is already done and is not part of it. Evidence for every item below is in `docs/evidence/grammar-review-2026-10-11/` (`document-audit.md`, `synthesis.md`), which is untracked; read it by path from the main checkout.

## Lift first

Before any deletion, copy into one untracked file under `docs/evidence/` the passages the audit marks for lifting, verbatim with their source line numbers:

- From `english-v2-rewrite.md`: "Selected roles preempt postmodifiers" (1402–1407); the finding that regressions arrived as coverage gains (1322–1326).
- From `english-grammar-is-derived.md`: nothing beyond "one declaration generates parse, render and build", which the new record restates.
- From `english-clauses-are-structural.md`: the passive does not masquerade as a copular participial adjective; a modal is a predicate-layer scope.
- From `english-coordination-is-structural.md`: its design, one generic n-ary coordination per homogeneous run with shared material owned by one host.
- From `english-v3-residual-batch-records.md`: move the whole file there.
- The list of faces covered only by a known-wrong analysis, the lessons at lines 130 and 162, and the coverage and reading series of the retired orchestration-state memory, and the retired blocker-triage list (23 constructions). These are seeds for the judgment set in `grammar-judgment-set`; they are in `docs/evidence/grammar-review-2026-10-11/retired-memory/` (untracked, main checkout).
- From `english-grammar-migration-obligations.md`: keep the witnesses (an open ticket cites them), drop the status; this file is amended, not deleted.

"Attestation is provenance, never a filter" (`english-v2-rewrite.md:1294–1302`) and the 2026-09-02 literal rule (1128–1130) go into the new record below, not into the evidence file.

## Delete

- Six decision records: `english-v2-rewrite.md`, `english-lean-design-workbench.md`, `english-grammar-is-derived.md`, `english-productions-ship-their-inverse.md`, `english-clauses-are-structural.md`, `english-coordination-is-structural.md`. Remove their entries from `docs/decisions/README.md` (six are indexed there).
- Dead historical documents under `docs/`: `english-grammar-review.md`, `english-derived-family-inventory.md`, `english-parser-performance.md`, `english-recovery-audit.md`, `english-regular-vocabulary-tables-design.md`, `english-grammar-design.md`, `english-grammar-wayfinder.md`.
- Repair or remove every remaining reference to a deleted file (`grep` for each name in `docs/`, `crates/`, `README.md`, `CLAUDE.md`; known referrers include `semantics-v2.md`, `builtin-v2-macro-spelling-and-grammar.md`, `kind-index-joins-union-marking-is-spelling.md`, `docs/tickets/fog.md`, `planned/readme-oracle-text-primer.md`, `planned/semantics-v1-cutover.md`, and a comment in `crates/deckmaste_english/src/grammar/ability.rs`). Do not edit `done/` tickets.

## Replace

`docs/decisions/english-lexical-analysis.md` becomes one short record, in place, with no rulings log, no dated amendments and no rule restated from another document. It says six things:

1. The engine contract, carried over unchanged: independent lexical analysis (old lines 52–80), admission on finite summaries and the packing contract (145–161), both round-trip laws (104–125), shortened-name scoping and the per-line census as facts. The construction-economy clause, the Lean and sequence sections, the clause keeping three superseded documents alive, the pruning rule, the word-naming ban and the landing-record amendment (old lines 36–50, 177–240, 14–19, 295–322, 244–246/285–293/340, 324–349) are not carried over.
2. What a Reading is. It is one whole tree from the packed forest, equal only to an identical tree. The grammar may stop producing trees that are not a different grammatical analysis: order of applying modifiers to one head, grouping within a same-coordinator chain, and clause versus verb-phrase coordination with nothing to tell them apart; the gate is that per line the set of analysis keys is unchanged while tree count falls. Dropping adjunct or postmodifier analyses of a preposition phrase the verb's frame selects is not decided. A known-wrong analysis never counts as covered; coverage is reported as read, read only by a known-wrong analysis, and unread. Be + participle keeps both the adjectival and the verbal-passive reading. A multiword game term declared by the plugin as one term is atomic. A bare imperative chain has one analysis, coordinated imperative clauses. A newly admitted wrong reading is a defect. Attestation is provenance, never a filter.
3. The lexicon rule. Content words enter only through lexemes. Only core closed-class lexemes may be referenced directly, by lexeme id; everything else reaches the grammar through typed classes and frames. A lexeme is named directly when exactly one can fill the slot, and a class is used for two or more; a class with one member or a feature with one holder is a reported finding. The loader rejects unknown ids, class names and values.
4. The authoring rule: authors state what a construction changes and the compiler derives the rest.
5. The gate: refactors keep every per-face reading digest identical; linguistic changes lose no face and are checked against the judgment set and the analysis-key rule above.
6. Size: totals per declaration kind are recorded in a tracked file and a test fails if any rises (ceiling during steps 0–7); from the coverage-resume step, at most 0.25 grammar lines added per face gained, enforced by the tool, with vocabulary-only faces first and a landing over the rate refused. The owner alone approves a raise or reset.

## Amend

- `builtin-v2-macro-spelling-and-grammar.md`: correct the stale crate and path names (`deckmaste_construction_core`, `plugins/builtin_v2`); replace its closed frame-atom list (132–147) with a pointer to the frame enum in code; state how a plugin frame relates to the grammar's own `frame` declarations and that a multiword term entered whole is atomic.
- `kind-index-joins-union-marking-is-spelling.md`: remove the two clauses at lines 35–40 and 80–82 that direct measurement tables into english-v3 construction declarations.
- `english-grammar-migration-obligations.md`: as above.
- `CLAUDE.md`, Scope: scope is what is legal in Vintage. The Conspiracy and Planechase sets are not out of scope as sets, and their Vintage-legal cards must parse; the Conspiracy card type and the Plane card type are out of scope because they are not playable in Vintage. Vanguard, Scheme cards (Archenemy) and ante are permanently out of scope. Un-sets are deferred, not permanently out of scope (some Un-cards are Vintage-legal). Remove "Do not ask about them, model them, or reserve room for them" for the deferred and in-scope items; keep it for the permanently excluded ones.
- `CLAUDE.md`, English grammar section: remove "Preserve every grammatical Reading", the "no guard that names a lexeme..." STOP and its "permitted guard" sentence, and the claim that the digest diff enforces no silent loss; point to the new record and keep the vocabulary-location and corpus-work lines.
- `docs/tickets/README.md`, Landing record: replace the six-item record and its enforcement sentence (the `environment.rs` file named there does not exist) with a three-line note that applies to English grammar tickets only: census delta (faces and Readings, lost faces named); size delta per declaration kind from the size tool; judgment-set result. Other tickets carry no record. Tickets whose instrument does not exist yet say which line does not apply.
- `crates/deckmaste_construction_v3_core/README.md`: correct the doctrine that authors must write every export. The passages are at lines 98–100 (defaults do not infer feature equations), 139 (policies cannot call policies), 146–151 (generic repetition does not fold member features), 157–159 and 177 (every construction exports every feature exactly once), 200–204 (defaults do not replace required exports), and 306–311 (the compiler does not invent a judgment from Category names). State them as properties of the lowered form the chart consumes, not as authoring rules, and say the authoring layer is being replaced by the tickets that follow. Also document `selected_frame` and `frame_category`, which the README omits.
- `docs/contexts/oracle-english/CONTEXT.md`, minimum only: remove citations of orchestrator rulings and links to done tickets used as authority, and remove the `_Avoid_` lines (at 148, 154, 161) that forbid the words Selection, Disambiguation, Merging and Collapsing. The full rewrite is `grammar-glossary-rewrite`.

## Shelve the old queue

- Create `docs/tickets/shelved/` and move, unedited, the 37 english-v3 coverage tickets in `planned/` (the 38 there, less `english-v3-licence-plumbing-economy`, which `english-v3-licence-plumbing-economy` itself rewrites) and the 3 `wip/` tickets returned by dropping the unfinished workspaces below (`english-v3-quantified-times-frequency`, `-slash-pair-predicative`, `-tied-for-adjective`). The other two `wip/` tickets (`english-v3-sentence-header-mode-list`, `-multiplied-variable-quantity`) stay where they are: their workspaces own them and are kept.
- `kata.toml` `[kanban] columns` is `["critical", "planned", "maybe", "wip", "done"]`. Leave `shelved` out of it so shelved tickets are never listed ready, blocked or ordered, and add a `shelved/` row to the folder table in `docs/tickets/README.md` (not claimable; work is re-derived from the blockers report at the coverage-resume step). Run `kata kanban check` before and after: no dangling needs, no duplicate slugs. If the driver refuses a folder outside `columns`, stop and report it rather than adding the column.
- Drop the three unfinished parked workspaces `english-v3-quantified-times-frequency`, `english-v3-slash-pair-predicative` and `english-v3-tied-for-adjective` (first, using the item-returning drop, so their tickets return to `planned/` and can be shelved with the rest; a ticket owned by a live workspace is not moved from outside it). Keep `english-v3-sentence-header-mode-list` and `english-v3-multiplied-variable-quantity` as they are, un-integrated.

## Gate

`cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports 0 stale; `kata kanban check` is OK; no live file links to a deleted one. This ticket is not an English grammar landing, so it carries no three-line note.
