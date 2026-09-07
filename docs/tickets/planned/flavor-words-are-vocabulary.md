---
needs: []
---
**A flavor word is an open slot, not vocabulary.** Ruling (user,
2026-09-07). `plugins_v2/builtin/macros/flavor_words/` holds 630 bodyless
declarations whose only content is a spelling and a `FixedTerm` grammar.
The semantics never invokes one: `Ability.italicHead` carries
`flavorWord (label : FlavorWordLabel)` with the label as a string
(`lean/Semantics/Words.lean`, inert vocabulary per `semantics-v2.md` §16),
and nothing about a flavor word is enumerable — any italic run before an
ability that is not an ability word is a flavor word, whatever it says.

Delete the 630 declarations and the family; no catalog or list replaces
them. The grammar gets a term that means "literally whatever string is
here": an italic-marked run captured verbatim as the `FlavorWordLabel`,
with the ability-word declarations still taking precedence where the run
is one of them. Retire the `FlavorWord` meta,
`DeclarationKind::FlavorWord`, the `flavor_words` entry in
`BUILTIN_FAMILIES`, `FlavorWordLabelTerm`'s per-word lookup, and the
semantics_v2 reader's registration of the family. `cargo xtask facts
check` byte-identical; english_v2's coverage lock unchanged
(`DECKMASTE_COVERAGE_LOCK=report`, every identity that parsed through a
flavor-word declaration still parses through the open slot, listed if
not). Ability words are NOT the same shape: they are a closed list with
rules meaning per `[CR#207.2c]` and keep their declarations. Standard
constraints apply.
