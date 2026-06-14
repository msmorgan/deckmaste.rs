---
needs: []
---
Generator task. Route triggered-ability event subjects through `filter.rs` instead
of the hardcoded "a creature"-only match (`triggered_ability.rs` `event_subject`),
so filtered ETB/dies triggers parse — "a creature you control enters", "another
creature you control dies", "a Goblin enters" → `Enters`/`Dies(<filter>)`. Also add
"each opponent" as a damage target (`effect.rs` `damage_target`) →
`DealDamage(Filter(OpponentOf(Ref(You))), N)`. The core grammar + engine already
model these (event macros take a filter; engine-trigger-events evaluates ETB/dies;
player-filter damage resolves) — this is the parse side. Unblocks ETB payoffs and
aristocrats triggers; Impact Tremors ("Whenever a creature you control enters, ~
deals 1 damage to each opponent") is the worked example. ~199 "you control"
enters/dies + ~178 "to each opponent" cards graduate. Feeds canon-goblins-elves;
part of tui-client.
