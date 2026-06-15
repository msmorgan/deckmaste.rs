---
needs: []
---
BLOCKER for tribal lords (and any continuous effect whose scope names `~` or
`you`). The layer / continuous-effect path evaluates a static ability's
`Matching` scope through `matches_derived` (`layer.rs:497`), whose relation/ref
leaves delegate to `target::matches` with NO watcher. So `Filter::Ref(This)` and
`Filter::Ref(You)` hit `todo!("… at a frameless position — targeting threads no
carrier")` at `target.rs:123` / `target.rs:134` the moment a layer rebuild
touches the effect.

Every canonical lord scope — `AllOf([Creature, Not(Ref(This)), Subtype(X),
ControlledBy(Ref(You))])` — therefore PANICS in a live game (Goblin Chieftain,
Elvish Archdruid, Goblin King, Elvish Champion, Imperious Perfect, Dwynen), as
does a spell-built floating scope like Overrun's `Continuously(Modify(of:
Matching(AllOf([Creature, ControlledBy(Ref(You))])), …))`. `engine-filter-breadth`
added the `ControlledBy`/`Ref` match ARMS but did not thread the carrier; the
live trigger/count path already threads its watcher (Dwynen's `CountOf` over
`Attacking, ControlledBy(You)` resolves correctly), so this is specifically the
derived / continuous-effect path.

Fix: thread the continuous effect's source object (the `ContinuousEffect`
carrier/controller — already stored) as the watcher into `matches_derived` and
on to `target::matches`, so `Ref(This)`/`Ref(You)` resolve against the host
instead of panicking. Verify a `Static(Modify(of: Matching(AllOf([…,
ControlledBy(Ref(You))])), [AddPower(1), AddToughness(1)]))` lord buffs other
controlled creatures (scoped to the controller) without panic, and that Overrun's
floating scope resolves. Distinct from engine-filter-walker (pure combinator
refactor; it explicitly preserves these `todo!` seams). Gates demo-goblins-elves.
