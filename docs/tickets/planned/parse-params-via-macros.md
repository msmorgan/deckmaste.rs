---
needs: [macro-slot-codec]
---
Extend `parse-via-macros` past nullary / integer keywords to the full
**parameterized** corpus across kinds: `Protection(<filter>)`, `Ward(<cost>)`,
pump `${0}/${1}`, count-bearing effects, filter and effect sub-clauses. Uses the
`macro-slot-codec` to invert each `${i}` slot from English into arg RON, and the
`macro-parse-index` matcher to recognize the skeleton. The generic
`macro_template` parser also serves as a typed sub-reader that the frame parsers
(`effect.rs`, triggered, activated) call for nested fragments.

Grows macro-driven coverage into the parameterized middle of the corpus; the
irregular tail (hybrid costs, quality-word equip, genuinely compound lines)
stays with the bespoke parsers.
