---
needs: []
---
**Flavor words are a vocabulary list, not macro declarations.** Ruling
(user, 2026-09-07). `plugins_v2/builtin/macros/flavor_words/` holds 630
bodyless declarations whose only content is a spelling and a `FixedTerm`
grammar; the semantics never invokes one, since `Ability.italicHead`
carries `flavorWord (label : FlavorWordLabel)` with the label as a string
(`lean/Semantics/Words.lean`, inert vocabulary per `semantics-v2.md` §16).
They exist because english_v2 must recognise the italic label when parsing
and construction_core's macro table was its only vocabulary channel.

Move them out of the macro table: one line-file vocabulary catalog under
`plugins_v2/builtin/` (shape per `deckmaste_catalogs`' line-file I/O; name
it for what it is, not `macros/`), read by construction_core as vocabulary
rather than as declarations, with english_v2's `FlavorWordLabelTerm`
recognising from that list. Retire the `FlavorWord` meta,
`DeclarationKind::FlavorWord`, the `flavor_words` family in
`BUILTIN_FAMILIES`, and the semantics_v2 reader's registration of the
family. `cargo xtask facts check` byte-identical; english_v2's coverage
lock unchanged (`DECKMASTE_COVERAGE_LOCK=report`, every flavor-word
identity still covered, listed if not). Ability words have the same shape
(`abilityWord (label : AbilityWordLabel)`, 61 bodyless declarations) and
are a candidate for the same move; do not fold them in without a ruling.
Standard constraints apply.
