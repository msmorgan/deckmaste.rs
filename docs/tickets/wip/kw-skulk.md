---
needs: []
---
Skulk keyword macro [CR#702.118b]: an evasion `Cant(Block)` over blocker
power — "can't be blocked by creatures with greater power" — read live as
`Stat(Power, Greater, StatOf(This, Power))` (the `This` resolves to the
carrier; the Stat filter's subject is the candidate blocker). Pure data;
block-legality enforcement is the shared later combat task.
