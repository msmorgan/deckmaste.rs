# workbench-counter-partitive-family

The counter-partitive slice — "remove X counters from among permanents you
control" / distributing or collecting counters ACROSS a described group as one
act, where the existing counter rows all act per-object or per-member. Measured
at 18 supported cards (Tayam, Luminous Enigma and family) during
workbench-counter-row-twins (done 2026-08-27), which built the multiplicative
self-reading row and found this shape is not a variant of it: the partitive is
a new slice over the group, a round of its own.

Re-verify the family with `jq 'select(.supported)'` over
`data/derived/cards.jsonl` at claim; the count is the close-out's, not a spec.

- **Routed from workbench-cost-3 (close, 2026-08-28):** "remove ALL counters
  from [X]" (25 lines) — the universal removal quantity; counter-removal
  machinery, so it lands here (`RemoveCounters` now takes a `Quantity`).

## As landed

Both items built. Two rows' worth of slot work, no pins minted — nothing in
either shape is rules-impossible.

**The partitive — `RemoveCountersAmong`, built and benched.** The count sits
on the COUNTERS, the source is a described GROUP, and the distribution is the
actor's [CR#608.2d]. Its own `Effect` row rather than a widening of
`RemoveCounters`' source slot: the two write different prepositions ("from"
against "from among") and the pooling is the whole of the difference, so one
shared slot would carry a gate saying which preposition each reading spells.
The group takes `PartitiveBase` (`SomeOf`'s gate) and `CounterMemory`
([CR#122.2,400.7]); the count stays a `Quantity` because the family writes
"one or more", "any number of", "up to three" and plain numbers alike. Nine
tables filled as `RemoveCounters` fills them, `effIntro` included, so
`RemovedThisWay` reads the removal back.

Re-measured 2026-09-02 over `data/derived/cards.jsonl` with
`jq 'select(.supported)'`: **18 supported cards**, one line each — Dawnhand
Dissident, Eventide's Shadow, Galloping Lizrog, Hierophant Bio-Titan, Hopeful
Initiate, Iron Spider, Jetfire, Light Up the Night, Novijen Sages, Ooze Flux,
Overseer of Vault 76, Quilled Greatwurm, Retribution of the Ancients,
Sensational Spider-Man, Slippery Bogbonder, Tayam, Tekuthal, The Filigree
Sylex. The ticket's 18 is confirmed. 17 are the removal, 11 of those in an
activation or alternative cost. Two further cards match the words and are NOT
this family — Aragorn, Company Leader and Elspeth Resplendent write "a counter
from among first strike, vigilance, deathtouch, and lifelink", a menu of
counter KINDS, which `CounterKindSource`'s printed menu already holds.

Bench: **Galloping Lizrog whole** (effect seat, and the did-branch reading
`RemovedThisWay` back) and `novijenSagesDraw` (cost seat; a fragment because
graft is not in the keyword catalog).

**The universal removal — the count position split, benched.**
`RemoveCounters`' count is now `Maybe (Quantity bs)` and the silence spells
"remove all counters". Not a `Quantity` arm: `SliceCount`'s standing note says
why (a `Quantity` states a count the text wrote, and `quantExact`,
`quantWellFormed` and `NonZeroQ` are all built on that, at every other
`Quantity` position too). The split is made at this position instead, in the
shape this row's own player cell already uses — `LosesCounters` takes a
`Maybe Amount` whose silence spells "all". `SliceCount`'s named-arm form was
the alternative and buys nothing here: one universal, and no `NonZeroQ`
demand to carry. `Phrase.idr` gains `optQuantIntro` and
`optQuantWellFormed`/`OptWellFormedQ` beside `optAmtIntro`; `Macros` gains
`removeCounters` (the counted wrapper) and `removeAllCounters`.

Bench: **Vampire Hexmage whole** — "Sacrifice this creature: Remove all
counters from target permanent", both `Maybe`s at their silence.

**Deviation — the routed count was short.** The ticket and the row's old
docstring carried 25 lines for "remove all counters". Re-measured 2026-09-02:
**50 supported cards, 50 lines** write `remove all [kind] counters from`, of
which **12** are kind-blind ("remove all counters from"). The docstring now
carries the re-measured numbers and names the routing ticket's figure as
superseded.

**Remainder — the MOVE partitive, one line.** Slippery Bogbonder's "move any
number of counters from among creatures you control onto that creature" is the
18th card and the family's only non-removal. `MoveCounters`' `src` does not
reach it. Left unbuilt deliberately at one supported line: a shared
source-split across both verbs is the shape it would want, and one line does
not buy the refactor. No pin — a count is not a refusal.
