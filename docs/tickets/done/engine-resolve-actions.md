---
needs: []
---
`Destroy`, `ReturnToHand`, and `Counter` (spell → owner's graveyard [CR#701.6a])
resolve. `WillDestroy` is the replaceable destruction event [CR#701.8a]: both the
`Destroy` action and the lethal-damage SBA emit it; its apply drops the destroy for
objects with a destruction-replacement static (indestructible [CR#702.12b]) else
commits the Bf→Gy move, retiring the previous loud SBA guard.
