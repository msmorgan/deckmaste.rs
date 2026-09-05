---
needs: []
---
**Restore missing obligations from erased Idris indices, beginning with choice
domain sorts.** `Predicate.check` in `lean/Semantics/Check/PhraseRules.lean`
checks the contents of `qualityNoun q dom` and `ofYourChoice q dom` without
requiring the domain's sort to match `q`. Idris required
`ChoiceDomain (QSort q)` at these positions. `sortedDomainCheck` in
`Check/AbilityRules.lean` restores that relationship at other consumers.

The 2026-09-05 review compiled this malformed acceptance through Lean LSP:

```lean
import Semantics.Check
import Semantics.Macros
open Semantics Semantics.Macros
example : Instruction.check []
    (.choose none (some .you)
      (.described (.a .unmarked) (.qualityNoun .color (some (.players .opponent))))
      .openly none) = [] := by decide
```

Make every choice-domain consumer enforce the sort relationship, with one
shared source for that check at a usable module boundary. Audit the other
erased indices in `Experimental.*` against the corresponding Lean constructor
fields: indices selected legal inhabitants just as explicit proof arguments
did. Lean is the target; read Idris for evidence without extending it.

Done when wrong-sort domains have exact-refusal pins beside same-slot valid
twins, the complete malformed choice is refused, and every erased index has a
named enforcing check or an explicit recorded reason it is no longer required.
Route any larger discovered redesign to a named ticket. Use Lean LSP for the
pins and run `lean/scripts/build`; preserve existing assertions.
