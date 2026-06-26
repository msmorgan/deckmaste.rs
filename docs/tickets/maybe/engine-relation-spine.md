---
needs: []
---
[design] **Engine: unify the relation vocabulary (Attack / Block / Cast / Attach /
Target / Counter) that is currently enumerated three separate times.** Large
data-model refactor surfaced by the idris↔rust structure audit (2026-06-26);
speculative — claim only with a design pass.

Today the same relation set is spelled independently in three places, with no
shared type and no connection between them:

- durative → `crates/deckmaste_core/src/filter.rs`: `StateFilter::Attacking`,
  `Blocking`, `Unblocked`, `RelationFilter::AttachedTo`
- inchoative (events) → `event.rs`: `StateFilterEvent::Attacking/Blocking/Blocked`,
  `Event::BecomesTarget`, casting as a stringly `Event::Performed { verb: "Cast" }`
- deontic → `deontic.rs`: `DeonticAction::Attack/Block/Target/Attach/Cast/Play/Activate`

So "Attack" appears 3× unconnected; `Filter::Unblocked` is a redundant primitive;
and there is **no way to express defender-side combat triggers** ("whenever you
are attacked", "whenever a planeswalker you control is attacked", [CR#508.1]) or
"deals combat damage to a player" ([CR#510]) because the combat events are
agent-side only and damage has no event variant (only the stringly verb).

Idris models this as ONE `Relation` enum (`idris/src/Core.idr`) with `agentKind`
fixing each relation's agent kind, projected into three aspects: durative `Holds
Relation Role`, inchoative `Begins Relation` (one event, role facets pick
attacker vs defender), deontic `Enact Relation agent patient`. From it "unblocked
attacker" is *derived* (`And [Holds Attack Agent, Not (Holds Block Patient)]`), not
a primitive.

Possible Rust adoption (staged — this touches the three largest core enums, their
RON spellings, the renderer, and every combat/attach/cast card):

1. Introduce `core::relation::Relation` (+ an `agent_kind` table); have the three
   aspects reference it instead of re-enumerating names. Removes the triplication
   and the redundant `Unblocked`.
2. Add an `Event::Begins(Relation)` form with Agent/Patient role facets (folding
   the combat `StateFilterEvent` variants and the `Performed{verb:"Cast"}`
   convention). Unlocks the defender-side trigger family.
3. Add a typed damage event with a `combat: Option<bool>` coordinate (replacing the
   stringly damage verb) so "deals combat damage" is expressible.

Caveat: this is about the **data-model shape**, not transferring Idris's
dependent-type soundness — Rust stays runtime-validated. Worth doing only if the
combat/relation surface is being reworked anyway; otherwise the defender-side
event (item 2, smaller) is the highest-value slice to lift out first.
