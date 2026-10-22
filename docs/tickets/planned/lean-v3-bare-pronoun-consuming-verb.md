---
needs: []
---
# A bare pronoun in an action slot reaches only what the verb can act on

Standard constraints apply. Found by `lean-v3-ninjutsu-defender-lki`, which removed the
public-zone filter (`publicOnly`) from reference resolution (`docs/decisions/semantics-v3.md`
§0.1 item 4). `badHiddenCost` ("Return a creature to its owner's hand: Tap it.") had been
refused only by that filter. Once the filter was gone, the bare-`it` spelling was admitted. The
pin now spells the pronoun `itPermanent` (`.atSlot .permanent`), which is refused.

## The defect

`docs/decisions/semantics-v3.md` §1.5 `J1` item 4, "Consuming-verb applicability", says a
referent is accessible only if the consuming verb or prepositional slot can apply to it in its
zone. A creature that died cannot be sacrificed or have counters put on it as a permanent
[CR#109.2,701.21a]. The checker enforces this only when the pronoun is spelled with a slot
carrier (`itPermanent`, `itCard`: `Reach.atSlot`, `slotZoneOk` in `Check/Words.lean`) or with a
carrier word (`that creature`). A bare `it` (`.pro .bare .one`) never consults the verb, so these
are admitted (scratch `#eval`, 2026-10-22):

| Text | Term | Result |
|---|---|---|
| "Return a creature to its owner's hand: Tap it." | `Ability.activated (.perform (putInto (a isCreature) hand)) (.setStatus .tapped it) …` | `[]` |
| "Return a creature to its owner's hand: Sacrifice it." | the same with `sacrifice it` | `[]` |
| "Destroy target creature. Tap it." | `.sequentially [destroy (target isCreature), .setStatus .tapped it]` | `[]` |

Each `it` resolves to an object in a hand or a graveyard, which tap and sacrifice cannot act on
[CR#701.26a,701.21a]. This is text-decidable: the zone the text moved the object to is known
(a text-decidable zone, not runtime visibility, so it stays on the checker's side of the engine
boundary).

## Work

- Decide where the slot comes from. The ruling in
  `docs/tickets/done/workbench-anaphora-a-bare-it.md` ("Settled design", A1) selects
  sort/zone-scoped reads at the template (macro) layer. Either the templates for verbs that act
  on permanents (`tap`, `untap`, `sacrifice`, `destroy`, counters onto a permanent, …) pass
  their patient slot to a bare pronoun, or the action stepper narrows `.bare` to its slot's
  carrier. This choice may need an owner ruling. STOP if so.
- Pin the three rows above as refused, and give `badHiddenCost` (`Proofs/Turn.lean`) back its
  bare `it`.
