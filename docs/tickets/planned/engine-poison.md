---
needs: [engine-counters-api]
---
Poison counters, infect/toxic/corrupted hooks, and the poison SBA (~190 cards).
Poison counter storage is already wired via the proxy counter map; this adds
infect damage-as-counters, toxic stacking, and the 10-poison SBA.
