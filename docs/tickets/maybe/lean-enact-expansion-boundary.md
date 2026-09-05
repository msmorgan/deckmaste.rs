---
needs: []
---
[design] **Define what makes an enacted deed's tag agree with its instruction
body.** The 2026-09-05 Lean LSP review compiled `by decide` witnesses that
`Instruction.check []` accepts both of these terms (with the Semantics and
Macros namespaces open):

```lean
.enact none (.action "Destroy") (.draw .you (.lit 1))
.enact none (.action "Destroy")
  (.move (target creature) (.zone .hand .bare) [])
```

`Check/AbilityRules.lean` validates the body and selected deed facts separately.
`Instruction.enactPatient` in `Check/Abilities.lean` recognizes only a directly
nested `move`: enclosing that move in `sequentially` also bypasses the enacted
patient-zone check. The tag is significant to deontics and provenance under the
existing semantics_v2 contract, so its relationship to the body matters.

Decide whether `enact` is trusted macro expansion output or independently
validated authorable syntax. For trusted output, specify and enforce the
expansion boundary that prevents arbitrary tag/body combinations. For authorable
syntax, establish the required agreement through declared facts and structural
checks. Do not accumulate checks naming individual lexemes or shallow body
shapes; a general equivalence checker is not a predetermined solution.

Done when the malformed examples are refused at the chosen boundary, a
singleton sequence cannot evade that boundary, genuine expansions retain their
profiles and provenance, and the contract states exactly what `Card.check`
guarantees about expansions. Use Lean LSP for the new pins and preserve the
existing card and pin assertions.
