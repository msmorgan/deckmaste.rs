---
needs: [english-v2-closed-class-single-owner]
---
**Retire the two Rust guards that name a lexeme.** `this_way: MannerReference`
declares `demonstrative: lex SingularDemonstrative checked by
singular_demonstrative_is_this()` and `noun: lex Noun checked by
noun_is_way()`; both closures pin one word and read no feature. It is the
literal `"this" "way"` routed through the lexicon to satisfy the rule that a
form literal may not equal a lexicon surface, and it breaks the standing guard
rule (a guard naming a lexeme, noun, or construction is a STOP, never shipped;
`english-v2-rewrite.md`: the residual restriction is declaration or lexeme
metadata, "never a `require` naming `target` or any other word"). Of the
`checked by` guards in the grammar these two are the only ones that name a
word; every other reads a declared licence or a structural predicate.

Pinned shape: the fact lives on the lexeme. Declare a noun feature for the
manner-anaphor class (`way` today; the noun inventory's per-word override
mechanism carries it) and select the demonstrative through a declared feature
or a `vocab` member `require` (the `require relation is Among` idiom). The
construction then reads `require noun.<feature> is …` and the two Rust
functions are deleted. Keep the construction compositional
(`lex(demonstrative) noun(noun)`); do not fall back to a literal.

Fence: any `checked by` whose body compares against a lexeme constructor.

Acceptance: `singular_demonstrative_is_this` and `noun_is_way` gone, every
"this way" identity still covered with the same analysis, byte-exact laws
green. Standard constraints apply.
