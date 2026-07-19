---
needs: [engine-transform, parse-look-at-top, engine-explore]
---
Canonize **Delver of Secrets // Insectile Aberration** — the first canon
Transforming double-faced card. `{U}` 1/1 Human Wizard front; 3/2 Human Insect
back with flying. Front ability: "At the beginning of your upkeep, look at the top
card of your library. You may reveal that card. If an instant or sorcery card is
revealed this way, transform this creature." [CR#701.27a]

Unblockers (why this waits):
- **transform** — the `Transform` verb [CR#701.27a] (`engine-transform` provides
  `Action::Transform` + the front/back face-state runtime).
- **look at the top card** — `parse-look-at-top` (the "Look at the top card of your
  library" single-card grammar).
- **reveal + branch-on-revealed-type** — `engine-explore` builds the Reveal seam
  (`PlayerAction::Reveal` / `GameEvent::Revealed`) and the "if a [type] card is
  revealed" branch [CR#701.44a]; Delver's "you may reveal … if an instant or
  sorcery card is revealed this way" reuses exactly that (a may-reveal variant plus
  the revealed-type branch). The `Effect::If` structure itself already exists
  (`parse-conditional-effect`, done), so only the reveal seam + revealed-type
  condition are missing.

Notes:
- First canon `Card::TwoFaced { layout: Transforming }` — verify the canon loader +
  renderer handle the two-faced shape end-to-end (the shape landed in
  `core-card-shapes`; hand-authored canon RON in `plugins/canon/cards/` should
  deserialize, but no canon DFC exercises it yet).
- Graduates `Action::Transform` with a real card → remove the DEFERRED
  `no_dead_grammar` allowlist entry for `Action::Transform` (added while no card
  used it).
- Canon is hand-authored RON, not extraction, so `pipeline-layout-extraction` is
  NOT required here.
