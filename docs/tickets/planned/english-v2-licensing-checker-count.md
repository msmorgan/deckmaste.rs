---
needs: []
---
**Count the hand-written Rust licensing checkers as a gate.** The rewrite
decision's guardrail is that every escape hatch is countable
(`docs/decisions/english-v2-rewrite.md`, "Guardrail 2"). The Plan 09 taxonomy
audit counted nine `checked by` predicate functions in the grammar; the file
now carries about a dozen, and nothing measures the number, so growth is
invisible at review.

Pinned shape: an xtask census (beside `form_literal_vocab_overlaps`) that lists
each `checked by` function with its kind, split into the permitted classes
(reads a declared licence feature; structural predicate such as
`rightmost_leaf_is`) and the forbidden one (compares against a lexeme,
construction, verb, noun, preposition, or card constructor). The forbidden
class must be zero once `english-v2-this-way-lexeme-guard` lands; until then
the census names the grandfathered pair. The permitted total is recorded in
every landing record's numbers so a rise is a review question, not a gate
failure.

Fence: a checker classified by its name rather than its body.

Acceptance: the census runs in the coverage gate output, the landing-record
rule in `CLAUDE.md` names the line, and the forbidden count is enforced.
Standard constraints apply.
