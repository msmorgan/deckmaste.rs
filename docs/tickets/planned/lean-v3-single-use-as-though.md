---
needs: [lean-v3-checker-and-drt-fixes]
---
# A single-use "as though" statement

*Biomancer's Familiar*: "{T}: The next time target creature adapts this turn, it adapts as though
it had no +1/+1 counters on it." `StaticSpec.asThough`, an "as though" statement [CR#609.4], covers
every event it names for as long as it is in force; only `StaticSpec.replace` carries a single use
(`ReplUse.nextTimeOnly`).
The card is not a replacement: Gatherer says the ability "just lets the creature adapt despite
already having +1/+1 counters on it", so the creature still adapts. `Cards/Counters.lean`
`biomancersFamiliar` is spelled with `asThough … thisTurn`, which applies to every adapt this
turn; its docstring points here.

Give an as-though statement the "next time" use (a `ReplUse`-like field on `asThough`, or a
usage limit on `establish`), spell *Biomancer's Familiar* with it, and pin that a second adapt the
same turn is not covered. Settled: "X adapts" is the adapt about X, done by its controller
(`DeedPermanent.isAbout`, `lean-v3-checker-and-drt-fixes`).
