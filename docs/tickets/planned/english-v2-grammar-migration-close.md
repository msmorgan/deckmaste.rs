---
needs: [english-v2-grammar-migration-design, english-v2-adjective-inventory, english-v2-attachment-class-declared, english-v2-cost-family-lowering, english-v2-frame-complement-pair-nesting, english-v2-grammatical-relations, english-v2-keyword-subject-modifiers, english-v2-lexeme-owned-verb-frames, english-v2-locative-coordination-arms, english-v2-locative-licence-set, english-v2-of-complement-filter-removal, english-v2-person-number-agreement, english-v2-relative-clause, english-v2-remaining-prepositions, english-v2-scope-device-cross-host-gates, english-v2-subordinate-clause, english-v2-target-verb-subject-selection, english-v2-type-line-construction, english-v2-type-line-declaration]
---
# Validate the migrated grammar and resume long-tail work

Close the production migration under the [Lean design decision](../../decisions/english-lean-design-workbench.md).
The migration-design ticket must replace this initial dependency inventory with
the actual migration tasks before it closes. Do not treat these inherited
eighteen tickets as an exhaustive grammar implementation plan.

Check the resulting Rust grammar against the reviewed formal model and its
named structural witnesses. Run the production structural/coverage checks and
the changed dependency closure as required by the landing contract. Preserve
and account for every routed regression obligation. Identify any superseded
constructions, adapters, or compatibility paths still present and finish their
replacement within the agreed migration scope.

Assess representative remaining failures and record whether the existing
long-tail loop is useful again. This is a revisable engineering judgment, not
a proof that no general grammar is missing. A demonstrated missing foundational
capability gets a design/migration ticket and an added dependency here; a local
residue gets an owner in the refreshed tail register. Classify cases that were
previously reported as covered but selected the wrong structure too.

Acceptance: the agreed migration is implemented and validated, the obligation
register has no silently lost entries, and the wayfinder records the return to
the long-tail phase with named limitations. No minimum coverage gain, global
uniqueness theorem, or exhaustive classification of all failures is required
to resume. The tail umbrella retains the eventual corpus-completion target.
Standard constraints apply.
