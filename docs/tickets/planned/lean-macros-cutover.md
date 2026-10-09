---
needs: [lean-macros-from-ron]
---
**Cut the hand Lean macro layer over to the generated one.**
`cargo xtask lean macros generate` writes `lean/Semantics/Macros/Declared.lean`
(namespace `Semantics.Macros.Declared`), one `semantic_macro` per
`plugins_v2/builtin` macro declaration, and `cargo xtask lean macros align`
proves each hand `Semantics.Macros` macro with a declared namesake equal to
it by `rfl`. At `lean-macros-from-ron` (measured on change `uuqznyso`): 975
declarations derived, 372 hand macros with a namesake, 324 aligned, 48
drifted. This ticket finishes what that one started.

1. **Retire the aligned hand macros** for their generated namesakes (the bench
   opens `Semantics.Macros.Declared` or the generated file moves into
   `Semantics.Macros`); positional call sites must follow the RON parameter
   order, which moves defaulted binders last (§12.1).
2. **Resolve the 48 drifts** `align` names, by class: 31 "body, loader device"
   (the declaration's body carries `firstMention`/`laterMention`/
   `ownScope`/`inCaller`, the §12.2 loader rewrites the hand bench does not
   spell: every keyword action read at `Instruction`, plus `doublePower`,
   `duringNextTurnOf`, `regenerationApplication`, the token bundles and the
   `Actor.*` performer helpers), 15 "signature" (`amass`, `modular`,
   `bushido`, `renown`, `fateseal` take `Nat`/`String` where the declaration
   takes `Amount`/`Subtype`; `move`, `create`, `createTappedToken`,
   `creatureToken[Of]`, `returnToBattlefield`, `establishFor`, `forEach`,
   `companion`, `Actor.mayCastFrom` differ in binders) and 2 "body"
   (`gainControl`, `levelUp`). Then make `align --deny-drift` part of the
   Lean landing checks.
3. **The residue of §12.1's "It computes" bucket** that has no declaration:
   `chooseSpree`, `dealDamageOwnPower`, `joinedHead`, `joinedHeadWhile`,
   `loseCounters`, `partyOf`, `rollRow`, `scaledMana` port to RON over the
   §12.2 devices or are re-spelled; `lookAndSort`, `lookAndSortInto` and
   `lookedCards` retire once `scry`, `fateseal` and `surveil` are the
   generated declarations (their RON bodies call `lookAndSortSlice`, which
   `Proofs/BindingIdentity.lean` already reads from the generated file).
   `lookedTop` builds a checker `Bindings` context, not a phrase, and stays
   Lean-side. The nine pronoun-window macros stay with
   `lean-drt-anaphora-refactor`.
4. **The six helper declarations the generator cannot derive** (listed at
   the foot of `Declared.lean`): `activated`, `rollsDice`, `copy` (no sample
   assignment reads), `applyResultsTable`, `chooseModes` (an untyped list of
   tuples), and `triggeredJoined` (`JoinedHeaders`, no second sample).
