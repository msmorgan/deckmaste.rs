---
needs: []
---
Battlefield-scoped "for each" selections drop " on the battlefield" on render.
The migration filter parser (`crates/deckmaste_migrations/src/parsers/filter.rs`,
~line 147) CONSUMES " on the battlefield" while emitting NO predicate atom, so
`filter_noun` has nothing to reproduce: Ancestral Mask renders "Enchanted
creature gets +2/+2 for each other enchantment." vs oracle "… other enchantment
on the battlefield." Newly exposed by the for-each pump render arm (the clause
was `[unrendered]` before).

Fix (pick one): have the parser emit a battlefield-scope atom the renderer can
spell, OR have `filter_noun` append "on the battlefield" for the
default-battlefield-scope selection shape. Prefer the parser-atom route so
parse⇄render stays a single rule.

Verify: `cargo xtask fidelity` on Ancestral Mask round-trips; no regression to
selections that legitimately omit the scope phrase.
