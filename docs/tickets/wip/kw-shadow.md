---
needs: []
---
Shadow keyword macro [CR#702.28b]: a bidirectional evasion restriction — a
creature with shadow can't be blocked by creatures without shadow, and can't
block creatures without shadow — encoded as two `Cant(Block)` rows over the
shadow keyword itself (`Has(Shadow)`, a self-declaring marker like Reach).
Pure data; block-legality enforcement is the shared later combat task.
