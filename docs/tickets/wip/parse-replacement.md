---
needs: []
---
Parser for replacement effects: "if … would …, instead …", "as … enters",
and "… enters tapped" templates. Feeds the engine-replacements registry.

## Implementation note (batch2 worker, 2026-06-14)

Added `parsers/replacement.rs` (registered in `resolve.rs` after the
triggered/activated parsers, before static), emitting
`Static(effects: [Replacement(...)])` over the existing `Replacement::{Instead,
Also}` + `AsEnters` grammar (canon Diregraf Ghoul / Kabira Crossroads already
use `AsEnters(Tap(This))`). Two templates, both reusing the shared effect
grammar (`effect::parse_clause`) and the trigger event grammar
(`triggered_ability::parse_event`, promoted to `pub(super)`):

- **"As ~ enters, <effect>."** -> `Replacement(AsEnters(<effect>))` — self
  subject only; a targeting effect declines (a self-replacement has no announce
  list).
- **"If <event> would …, <effect> instead."** -> `Replacement(Instead(would:
  <event>, instead: <effect>))` — enters/dies events, self or filtered subject.

Deliberately CONSERVATIVE-DECLINE (a wrong replacement graduates a wrong card):
the parser abstains on everything its productions don't fully cover, which —
per a corpus scan of the ~30k wizards `.ron.todo` files — is most of the
replacement-effect text today, because those clauses need upstream productions
the engine does not yet have:

- Bare `~ enters tapped.` is left to the mana-ability parser (this module
  declines it); the *conditional* forms that dominate the corpus ("enters tapped
  unless you control a basic land", "… with two charge counters") need a
  condition slot on the self-replacement the grammar does not yet have.
- "As ~ enters, choose <X>" ([CR#614.12], the bulk of the "as enters" corpus)
  needs a `Choose`-style effect production.
- `Instead` lines replacing exile / draw / create-token / "double" need
  exile/draw-replacement effect+event productions.

Coverage therefore widens automatically as those shared grammars grow; the
parser is correct and ready now.
