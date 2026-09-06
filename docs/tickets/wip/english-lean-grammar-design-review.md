---
needs: [english-lean-document-grammar, english-lean-selection-laws]
---
# Review the whole formal grammar for migration

Review `English` and `docs/english-grammar-design.md` as the proposed whole
grammar under the [Lean design decision](../../decisions/english-lean-design-workbench.md).
Check the style-guide scope map and inherited obligations, then challenge the
model with combinations not used to construct each capability. Judge selected
or packed structure and explicit exclusions, rather than constructor counts
or whole-card coverage gains. Check the interaction of the selection laws with
the document grammar, which could have developed independently.

The composition scope decisions in `docs/english-grammar-design.md` are
explicit review inputs: extraction/relative form constraints, countability and
modifier feature carriers, mixed agreement, temporal NP distribution, and
antecedent accessibility. Challenge these before declaring a migration slice
ready; a restricted illustrative lexicon does not discharge them. The document
section adds explicit questions about source whitespace, structured notation,
inline reminders and frame metadata; audit those alongside quotation and
keyword boundary interactions. The selection section adds three explicit
correspondence obligations: derive comparison claims/regions from grammatical
features, project actual frame-pair syntax to the checked right-periphery and
anchor abstractions, and supply a nested-mobile grammatical witness for the
joint-alternative law. Challenge scope keys with document embeddings and
homographic lexical identities; equal text is not a packing criterion.

Resolve design-blocking findings with model changes and updated witnesses or
proofs. Record a short conclusion in the design document: represented scope,
the exact proven claims and assumptions, remaining limitations, and which
questions production migration must answer. A proof about the model is not a
proof of Oracle adequacy or of the future Rust implementation.

Acceptance: the agreed scope is accounted for and design-blocking decisions
are resolved; a finite named set of cross-capability challenges is checked;
all Lean targets pass without proof placeholders. Remaining uncertainties are
explicit, not hidden behind a claim of complete grammar. Exhaustive
classification of the corpus and global absence of unintended ties are not
review gates. Standard constraints apply.


## Review checkpoint — 2026-09-05

Review evidence and proposed residual decisions are in
[english-grammar-review.md](../../english-grammar-review.md), change `nqzqyuro`.
The review pass is complete; acceptance is not yet satisfied and this ticket
must not integrate as done. The user requested a grilling round on residuals.

Implemented the bounded nested-quotation punctuation fix and added twelve
positive interaction assertions: a real nested-mobile class with four distinct
admitted readings, exact packing, quotation embedding, and exact deep-quote
spelling. All previous 117 theorem declarations remain unchanged; none were
removed, re-spelled, ignored or restored. The complete Lean build passes all
68 jobs with warnings as errors. LSP diagnostics and all twelve axiom audits
are clean, using only standard axioms. Production code/corpus counts and
coverage were untouched; no coverage gain or loss is claimed.

Outstanding closure decisions: grammatical evidence and locality for
preference/scope; canonical groups and document/phrase boundaries; contextual
admissibility and feature propagation; source/layout validation ownership.
The report records the concrete counterexamples and uncertainty on each
recommendation. After discussion, implement or assign the resulting finite
closure obligations under this review before unblocking migration design.
No other WIP ticket was edited.


## User clarification and project separation

English is JUST an NLP grammar for parsing/bracketing Oracle English. It must
not interact with Semantics; the user requested separate Lake projects.
English now lives under `english/`; `lean/` is the independent Semantics
project. The accepted design decision records that boundary. Game-reference
resolution, card validation and frame legality are outside this review.

The earlier implementation-choice questions were premature. Investigate and
resolve grammar representation choices against concrete evidence; only a real
residual user decision should return to the grilling frontier. Continue the
review's grammar closure work after validating the project split.


Project split verified: `english/scripts/build` passes 13 jobs;
`lean/scripts/build` passes 56 jobs. Both cross-project import probes fail with
the expected unknown-module result. LSP diagnostics are clean at the new
English root. All 129 English theorem declarations were moved unchanged;
no Semantics source or assertion changed. The two manifests have no package
dependencies. Historical done-ticket paths remain as landing provenance.
