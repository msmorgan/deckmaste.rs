---
needs: []
---
**Cards write the dialect they may write.** `plugins-v2-dialect` landed the
reader side (embeds bare, positional application, numeral leaves, case as
the macro/constructor mark) and made every card macro-only, but the cards
and family bodies were converted by identifier rewrite, so they still spell
`Simple(symbol: Specific(color: Of(color: White)))` where the dialect reads
`White`, `colorIs(color: Red)` where it reads `colorIs(Red)`, and
`Lit(value: 3)` where it reads `3`. This ticket is the load-and-reserialise
pass the second dialect rulings describe: a one-off converter keeps each
file's leading comment block verbatim and reserialises only the value
through the dialect's writer (embeds bare, positional where unambiguous,
numerals bare), over `plugins_v2/canon`, `plugins_v2/testing` and the
family bodies under `plugins_v2/builtin/macros/`; interior comments are
counted, listed, and re-placed by hand; the converter is deleted. Oracles:
`lean/Generated` byte-identical before and after, `lean-check` 118/118 and
2/2, `cargo xtask facts check` byte-identical, the corpus test green. The
writer's positional/embed elision is what `semantics-v2-embed-candidates`
reads its evidence from. Standard constraints apply.
