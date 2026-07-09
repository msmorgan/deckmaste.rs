---
needs: []
---
Untap-skip / stays-tapped primitive. There is no engine action or replacement for
"doesn't untap during its controller's untap step": `DeonticAction` has no
`Untap` variant and there's no `SkipNextUntap` / stays-tapped action.

Add a skip-untap representation (a deontic `Cant(Untap …)` during the untap step,
or a "skip next untap" rider/replacement), then wire the untap step to honor it.

Cards blocked (~35+ one-away):
- Temple/painland mana riders: `{T}: Add {C}. ~ doesn't untap during your next
  untap step.` (~17) — flagged by parse-mana-abilities.
- `Enchanted creature doesn't untap during its controller's untap step.` (~9),
  `~ doesn't untap during your untap step.`, `You may choose not to untap ~ …`.
Distinguish the permanent flavor (skip every untap) from the one-shot "next untap
step" rider.
