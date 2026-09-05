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
ready; a restricted illustrative lexicon does not discharge them.

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
