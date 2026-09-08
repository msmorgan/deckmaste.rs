---
needs: []
---
**The facts generator reads bodies through the dialect.** `cargo xtask facts`
deserialises the keyword-ability, keyword-action, counter-kind and
designation bodies into typed rows through a plainer `MacroSet` than the
semantics_v2 reader, so a body written positionally (`HeldBy(Player)`)
broke `facts check` during `plugins-v2-cosmetic-conversion`; those four
families were excluded from the positional rule and keep their binders
(963 rewrites not made). Point the facts reader at the same `MacroSet`
configuration the v2 reader uses (`denying_unknown_fields`,
`reading_positional_arguments`), then apply the positional rewrite to the
four families with `Facts.lean` and `FactsGen.idr` byte-identical as the
oracle. Standard constraints apply.
