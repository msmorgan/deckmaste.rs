---
needs: [english-v2-grammar-frame-compiler, english-v2-lexeme-owned-verb-frames, english-v2-adjective-inventory, english-v2-subordinate-clause, english-v2-remaining-prepositions, english-v2-scope-device-cross-host-gates, english-v2-relative-clause, english-v2-grammar-measure-phrases, english-v2-grammar-document, english-v2-grammar-context-ellipsis, english-v2-type-line-construction, english-v2-grammar-lexical-source, english-v2-target-verb-subject-selection]
---
# Validate the migrated grammar and resume long-tail work

Close the production migration under the [Lean design decision](../../decisions/english-lean-design-workbench.md).
The thirteen direct dependencies are the migration implementation tasks in the
[wayfinder](../../english-grammar-wayfinder.md). Check every row of the
[obligation register](../../english-grammar-migration-obligations.md), including
formerly covered wrong analyses and the separately gated Target Verb rivalry.

Check the resulting Rust grammar against the reviewed formal model and its
named structural witnesses. Run the production structural/coverage checks and
the changed dependency closure as required by the landing contract. Preserve
and account for every routed regression obligation. Identify superseded
constructions, adapters, tail codecs and structural-specificity arbitration still present and finish their replacement. Verify
that scoped alternatives reconstruct the exact retained readings on the named
finite witnesses, including correlated sites and frame anchorings. No task is
complete just because coverage stayed constant while its feature was withheld.

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
