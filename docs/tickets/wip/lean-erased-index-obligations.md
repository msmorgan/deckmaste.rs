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

## Landing record

Change `wszuuxnqlpvvqpqwnxtlwmzskqxtnmyt`; English lock `covered`: 20,254.

**PROVE:** `sortedDomainCheck` now owns contents and sort checking in the
phrase layer; all four choice-domain consumers call it. Wrong-sort color and
subtype domains have exact-refusal pins and same-slot valid twins, including
the complete malformed choice and both static consumers. Existing content
errors are retained alongside sort errors. `lean/CONTRACTS.md` maps all 129
indexed reference datatypes, parameterized records, and dependent pairs to
checks, derived fields, or explicitly removed consumers. The refreshed
`lean/scripts/build` passed all 61 jobs without warnings. Assurance: 14 pins
added, 2 re-spelled for the consolidated helper, 0 restored, ignored, or
removed. LSP diagnostics are empty; the audited complete-choice refusal proof
uses only `propext`.

**DISCLOSE:** No accepted card was lost and no new card acceptance is claimed.
Known larger representation work stays with
`lean-grammatical-reference-scopes`, `lean-conjunction-type-evidence`,
`lean-enact-expansion-boundary`, and `lean-lookback-carries-a-game-event`;
the index map does not claim those contracts are settled. Deviations and
additions: shared checker-contract documentation and README link; no new
syntax constructors, glossary terms, lexical guards, or unresolved STOPs.

**REPORT:** English coverage and declarations are untouched; English selection,
structural inventories, licensing counts and performance were not remeasured.
No CR citation sites changed.
