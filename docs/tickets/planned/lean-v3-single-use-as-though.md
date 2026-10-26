---
needs: [lean-v3-checker-and-drt-fixes, semantics-v3-peer-files-and-toposort]
---
# A single-use "as though" statement

*Biomancer's Familiar*: "{T}: The next time target creature adapts this turn, it adapts as though
it had no +1/+1 counters on it." `StaticSpec.asThough`, an "as though" statement [CR#609.4], covers
every event it names for as long as it is in force; only `StaticSpec.replace` carries a single use
(`ReplUse.nextTimeOnly`).
The card is not a replacement: Gatherer says the ability "just lets the creature adapt despite
already having +1/+1 counters on it", so the creature still adapts. Until this lands the card is
unspelled: `asThough … thisTurn` would cover every adapt that turn, and `nextTimeOnly` would make
it a replacement [CR#614.1]. `Cards/Counters.lean` `biomancersFamiliarWatchesTheCreatureThatAdapts`
pins its "target creature adapts" pattern and points here.

Give an as-though statement the "next time" use (a `ReplUse`-like field on `asThough`, or a
usage limit on `establish`), spell *Biomancer's Familiar* with it, and pin that a second adapt the
same turn is not covered. Settled: "X adapts" is the adapt about X, done by its controller
(`DeedPermanent.isAbout`, `lean-v3-checker-and-drt-fixes`).
