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


## Review result

The [review report](../../english-grammar-review.md) and
[revised design](../../english-grammar-design.md) record the resolved
architecture, finite interaction challenges and exact limits. English and
Semantics are separate Lake projects. Consolidation includes canonical
collections; declaration-backed role preference; mixed agreement; contextual
ellipsis; lexical feature and extraction constraints; frame-boundary and
shared-dependent scope; identity preference; and the final checked-candidate
interface. The migration-design ticket and wayfinder carry the explicit
elaborations and inherited regression obligations. No residual user decision
requires a grilling round; no model-wide completeness claim is made.

## Landing record

**PROVE.** This is the workbench exception under the accepted Lean design
ADR. Production Rust, lexical data, the coverage lock and selected corpus
analyses are unchanged; there is no production coverage loss or gain.
All 117 pre-review theorem declarations remain. The model's named laws and
inhabited/exclusion challenges are listed in the review. The final candidate
contract checks lexical features and extraction before selection and packing.
English and Semantics have independent Lake configurations and no dependency
on each other. Final gate evidence is recorded below after refresh.

**DISCLOSE.** Source theorem declarations: 297, including 180 additions.
Four baseline assertions were re-spelled: `elliptic_antecedent`,
`elliptic_surface`, `nested_reminder_rejected`, and
`qualification_sites_excluded`. Their grammatical outcomes are preserved;
the private `admitted_maximum` helper was made public without changing its
statement. Removed, ignored, restored: zero each. Generic proofs and fixtures
were adapted to the consolidated carriers. Semantics source/assertions were
not changed. Existing long lines were wrapped to the Lean width convention.

Deviations/additions: consolidation was explicitly authorized in this review;
the user also required the independent Lake project split and the NLP-only
boundary. New glossary terms describe countability, genitives, relatives,
extraction, right node raising, parentheticals and Identity Claims. No CR
citation was added or changed. All additional witness modules implement the
review's named structural challenges. The draft reminder-isolation assumption
was corrected against supported Oracle text before commitment. Concurrent
same-file LSP axiom audits returned inconsistent results; sequential and
compiled-environment audits replace that evidence. The only source-scan flag
is an inspected local notation for the full candidate type, not a proof escape.
No regression or recorded-ruling contradiction was integrated around.

**REPORT.** Evidence is for `rmkonlyw` and its refreshed descendants. Production
coverage, selection census and performance telemetry are not measured or
claimed by this workbench ticket. Remaining fragment elaborations and source
contract obligations are explicit in the design and pinned as migration-design
inputs; they are not hidden under corpus completeness or a tail-loop claim.
