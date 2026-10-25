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

## Ruling (owner, 2026-10-24)

Enforce J1 item 4 in the action stepper, not in templates. Authors write a bare `it`. When the
stepper steps a patient slot that requires a specific zone, J1 item 4 filters out referents whose
known current zone contradicts that slot. The slots are `.battlefield` for `setStatus`,
`turnOver`, `combat`, `clearDamage` and `markDamage`, and an `Action.move` whose written `from_`
is the battlefield. This refuses "Destroy target creature. Tap it." and "Return a creature to its
owner's hand: Tap/Sacrifice it." The earlier choice between template-layer slots and stepper
narrowing (the A1 ruling of `docs/tickets/done/workbench-anaphora-a-bare-it.md`) is settled for
the stepper; no STOP is needed.

## Work

- In the action stepper, where a patient slot with a required zone resolves `.bare`, narrow it
  to the slot's carrier (`Reach.atSlot`, `slotZoneOk`, `Check/Words.lean:1062`; `reaches`, `:1215`).
  Referents whose known current zone contradicts the slot are filtered out; a referent of unknown
  zone is kept (runtime visibility stays the engine's).
- Pin the three rows above as refused, add an accepted pin for a bare `it` whose referent is on
  the battlefield (a permanent the text put or left there), and give `badHiddenCost`
  (`Proofs/Turn.lean:60`) back its bare `it`.

## Added 2026-10-25: retire `itPermanent` (Atraxa audit triage)

Once the stepper narrows a bare `it` in a sacrifice or tap slot, the `itPermanent` Template
(`Templates.lean:95`) is redundant: Oracle prints a bare "it" at every site. Write `it` at each
and delete the Template:

- `Cards/Trigger.lean:612`, *Frost Walker*: "When this creature becomes the target of a spell or
  ability, sacrifice it." A probe on 2026-10-25 found that bare `it` here resolves to the
  targeting spell and checks `[]` today, a wrong reading. After the narrowing (sacrifice is a
  move from the battlefield) it must resolve to the source. Add the pin
  `frostWalkerSacrificesItself` asserting the resolved id is the source's.
- `Cards/Choice.lean:1306` and `Cards/Anaphora.lean:585`, *Soul Ransom*: "This Aura's controller
  sacrifices it". Pins `okSoulRansomRansom` (`Cards/Anaphora.lean:586`),
  `possessiveDeicticIsReadableByIt` (`:593`), `possessiveDescribedBaseUnchanged` (`:606`) stay
  green.
- `Proofs/Turn.lean:62`, `badHiddenCost` (already listed above).

If a site still needs `.atSlot` after the narrowing, keep the Template, STOP and report the site.
`itCard` (`Templates.lean:93`; Bioplasm `Cards/Anaphora.lean:420`, Vaevictis Asmadi `:784`) is
not covered by this ruling: its carrier comes from a predicate ("if it's a creature card") or a
card-zone origin, not from an action slot. Whether the stepper should narrow those too is an
open owner question; leave `itCard` alone here.
