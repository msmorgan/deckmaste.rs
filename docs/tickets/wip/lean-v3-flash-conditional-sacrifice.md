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

## Rulings (2026-10-23)

1. **One ability, the second sentence a rider on the permission.** The two sentences are one
   paragraph, so one ability [CR#113.2c]: "If you cast it" is a cast made under that permission,
   which is why the 2022-12-08 Gatherer ruling holds ("The sacrifice occurs only if you cast it
   using its own ability"). The ability is a static ability that functions on the stack and
   creates its delayed trigger as the permanent enters [CR#608.3g]. It is written as
   `DeonticMods.rider` on the `can`, stepped after the permission's clause so its pronouns read
   that clause (`thisSpell` once, every "it" `it`); the cast-timing check is the printed one
   [CR#307.5a], read of the permitted cast. A rider on any deontic but `can` is refused
   (`deonticRider`). No `resolving` constructor, no `.asEvent (move …)`, no Link Source.
2. **Scope: the nine cards** (Armor of Thorns, Grave Servitude, Lightning Reflexes, Mystic Veil,
   Parapet, Relic Ward, Soar, Spider Climb, Ward of Lights with its `retention`). Necromancy is
   [lean-v3-necromancy-linked-entry](../planned/lean-v3-necromancy-linked-entry.md); dash, blitz
   and warp are
   [lean-v3-dash-blitz-warp-delayed-trigger-form](../planned/lean-v3-dash-blitz-warp-delayed-trigger-form.md).
