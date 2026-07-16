---
needs: []
---
Carry the second fighter on the `Act(Fight)` fact. The engine decomposes only
the first fighter onto the event (`resolve/action.rs`: `Ka::Fight(a, _)` —
"the second is the body's business") and the `Act` `FactView` never sets
`patient` (`eval.rs`), but `KeywordActionPattern::Fight(a, b)` matches its
second predicate against `fact.patient` — and a `None` part matches only
`Predicate::Any`. Net effect: "whenever this creature fights" authored as
`Act(Fight(Ref(This), Any))` silently misses when the creature is the SECOND
fighter, and any narrowed second-slot pattern (`Fight(Any, Type("Zombie"))`)
never matches at all.

Fix by either (a) emitting the resolved second fighter on the event and
exposing it as the fact's `patient` — then a fight trigger on either
participant needs the event emitted symmetrically or the pattern matched
against both slots ("each of those creatures fights the other",
[CR#701.14a]) — or (b) dropping the second slot from the pattern type until a
card needs it. Today's grammar can express patterns that can never fire,
which the no-dead-grammar philosophy exists to prevent.
