---
needs: []
---
Finish the two WC99 matchup decks (Mark Le Pine's Sped Red and Matt Linde's
Mono-Green Stompy) in `deckmaste_noncanon`.

The exact 60-card lists now resolve from builtin, canon, and the generated
Wizards corpus. Cursed Scroll and Wild Dogs are authored canon examples; the
remaining cards graduate from Oracle text. The obsolete `plugins/noncanon`
proxy layer and its allowlists have been removed. The generated-data-gated
historical matchup is Sped Red versus Stompy.

## Completed scope

- Both exact historical decklists resolve without proxies or allowlists.
- Cursed Scroll's name choice, random reveal, and conditional damage execute.
- The historical matchup completes under its generated-data test gate.
- `plugins/noncanon` has no remaining responsibility and is removed.
