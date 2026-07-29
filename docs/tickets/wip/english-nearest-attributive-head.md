---
needs: [english-test-structural-assertions]
---
**Keep `nearest` attributive when a following nominal head is available.** The
typed-assertion conversion exposed that `the nearest opponent` currently lowers
with `nearest` as a fused `Noun::Word(Vocab::Nearest)` head and pushes
`opponent …` into a zero relative clause. The old debug-string test passed
because `contains("Nearest,")` did not distinguish the noun and adjective
variants.

Prefer `Adjective::Word(Vocab::Nearest)` modifying the following `opponent`
head in this ordinary attributive shape. Preserve the fused-head noun reading
when no head follows, add typed positive and over-fire assertions, preserve
byte-exact rendering, and report the recovery delta. Standard constraints
apply.

## Completion

- Preferred attributive `nearest` when a following head completes while preserving the fused-head reading; corpus recovery and round-trip counts are unchanged.
