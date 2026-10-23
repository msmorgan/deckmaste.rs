---
needs: []
---
# "If you cast it any time a sorcery couldn't have been cast, … sacrifices it at the beginning of the next cleanup step"

Standard constraints apply. Lean first (`lean/Semantics/`). Provenance: *Ward of Lights*, in the
Ward cycle of
[lean-v3-effect-keeps-existing-attachments](../done/lean-v3-effect-keeps-existing-attachments.md),
which could not be spelled for want of this form.

Ten Vintage-legal cards (`data/derived/cards.jsonl`) print "You may cast this spell as though it
had flash. If you cast it any time a sorcery couldn't have been cast, the controller of the
permanent it becomes sacrifices it at the beginning of the next cleanup step.": *Armor of
Thorns*, *Grave Servitude*, *Lightning Reflexes*, *Mystic Veil*, *Necromancy*, *Parapet*,
*Relic Ward*, *Soar*, *Spider Climb*, *Ward of Lights*.

The pieces exist: the cast permission (`can`/`gatedBy` with `asThoughHadFlash`, as *Ghitu Fire*'s
`okGhituFire`), the cast-timing lookback (`okCastWhenASorceryCouldNotHaveBeen` [CR#307.5a]), "the
permanent it becomes" (`NounPhrase.becomes` [CR#608.3a]) and "at the beginning of the next
cleanup step" (`atNext .cleanup` [CR#514.3a]). What is missing is the form of the second
sentence: a spell's static ability under which the delayed triggered ability is created as the
spell resolves [CR#603.7a], conditioned on how the spell was cast. Decide that form (a replacement
on the spell's resolution, or a dedicated static form), pin it, and spell the ten cards,
*Ward of Lights* with its `retention` like *Flickering Ward*'s.
