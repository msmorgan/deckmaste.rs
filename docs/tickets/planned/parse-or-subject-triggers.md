---
needs: []
---
Parse OR-composed trigger subjects and events: `Whenever ~ or another
<filter> you control enters, …` (Ally/enchantment/creature variants),
`When ~ enters or dies, …`, `Whenever ~ enters or attacks, …`,
`When ~ enters or leaves the battlefield, …`, `Whenever ~ attacks or
blocks, …`, and the plural-subject spelling on partner-style cards
(`Whenever ~ enter or attack, …`).

These lower to a disjunction of the event forms the triggered-ability parser
already emits individually — the master event-filter forms exist
(`core-eventfilter-master-forms`, done). The work is the head production
that recognizes `X or another F` / `<event> or <event>` and emits the
composed trigger (one ability with an event disjunction, or the equivalent
the grammar prefers — check how existing multi-event abilities are spelled
before inventing a shape).

**~325 of 17,022 one-away cards** (2026-07-16 tally; `~ or another <filter>
enters` is the biggest slice at ~120).

Verify: `cargo xtask generate plugins/wizards` graduation delta;
`cargo xtask fidelity` on an enters-or-attacks and an or-another-Ally card.
