---
needs: [plugins-v2-dialect]
---
**Generate `lean/Semantics/Macros.lean` from the `plugins_v2` declarations**
so the Lean macro layer stops being hand-written (`semantics-v2.md` §6
"eventually"). Not needed by the gate, which checks expanded terms. Parked
until the macro-body families land.

## Residue from `semantics-v2-macro-capture-and-plurality` (2026-10-07)

That landing answered capture and plurality in the loader (`semantics-v2.md`
§12.2) and retired the pronoun-baking helpers, but did not port the 21 macros
left in §12.1's "It computes" bucket one by one. Each still decides here: the
nine that build a pronoun window are inventory for
`lean-drt-anaphora-refactor`; the rest (`amass`, `chooseSpree`,
`dealDamageOwnPower`, `joinedHead`, `joinedHeadWhile`, `lookAndSort`,
`lookAndSortInto`, `lookedTop`, `loseCounters`, `modular`, `partyOf`,
`rollRow`, `scaledMana`) either port over the §12.2 devices or are re-spelled.
The RON `lookAndSortSlice` replaces `lookAndSort`/`lookAndSortInto` for the
keyword bodies; the bench still calls the Lean pair.
