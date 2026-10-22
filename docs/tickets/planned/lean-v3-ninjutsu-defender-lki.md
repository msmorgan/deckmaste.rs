---
needs: [lean-v3-combat-status-and-entry-riders]
---
# Ninjutsu and sneak attack what the returned creature was attacking

Standard constraints apply. Split from
`lean-v3-combat-status-and-entry-riders` §2
(provenance `docs/evidence/mtg-rules-review-2026-10-21/departures-lean.md` #15). **Blocked on an
owner ruling** (below) before implementation.

## Goal

- Ninjutsu (`Definitions/Keywords.lean`) ends in `putOntoBattlefieldTappedAttacking this_
  yourHand`, i.e. `.entersAttacking none`: the controller chooses the defender [CR#508.4]. The
  Ninja "enters attacking the same player, planeswalker, or battle as the creature that was
  returned" [CR#702.49c].
- Sneak is a bare cast permission with no Entry Rider. A permanent spell whose sneak cost was paid
  "enters the battlefield tapped and attacking", attacking the same target as the creature
  returned to pay the cost [CR#702.190b]. The rider is gated on `keywordCostPaid sneak`.
- The defender is what the returned creature was attacking, read by last known information as it
  left the battlefield [CR#608.2h]. Nothing in the repository cites [CR#702.49c] or
  [CR#702.190b] yet.

## The refusal

The natural spelling of the defender,

```lean
the (and [ or [isPlayer, isPlaneswalker, isBattle],
           inCombat .attackedBy (the (and [was (returnToHand which), isCreature])) ])
```

is refused `participialDescription (.zoneChange none .hand) 0`: "the returned creature" finds no
candidate. The returned creature is in its owner's hand, a hidden zone [CR#400.2], and the checker
drops it in two places:

- Across an activated ability's colon the effect keeps only the cost's mentions in public zones
  (`publicOnly`, `Check/Phrase.lean` `.activated` arm; ADR `docs/decisions/semantics-v3.md` box 1
  table, ~line 125, citing [CR#400.2,400.7j]; `lean/CONTRACTS.md` ~line 564).
- What a printed ability publishes to linked abilities (sneak's rider would read the cast
  permission's payment) must be in a public zone (`linkedResult` / `publicCostResult` in
  `Check/Phrase.lean`, ~1919-1932).

The ADR's LKI paragraph (`semantics-v3.md` ~179-195) lets a characteristic read see a *visible*
departed permanent, but the referent is no longer visible, and an attack target is not a
characteristic.

Scratch `#eval Ability.check (DRS.ofList []) …` evidence at the split (2026-10-22):

| Ability | Result |
|---|---|
| `activated (compound [perform (sacrifice (a (and [isCreature, youControl])))]) (dealDamage (powerOf (the (and [was (sacrifice which), isCreature]))) (target isCreature))` | `[]` |
| `activated (compound [perform (exile (a (and [isCreature, youControl])))]) (dealDamage (powerOf (the (and [was (exile which), isCard]))) (target isCreature))` | `[]` |
| the same with `returnToHand` and `the (and [was (returnToHand which), isCreature])` (or `isCard`, or `thisWay`) | `[participialDescription (zoneChange none hand) 0]` |
| `sequentially [returnToHand (target isCreature), tap (the (and [was (returnToHand which) thisWay, isCreature]))]` (one effect, no colon) | `[]` |

[CR#400.7j] says an effect can *find* an object a cost moved to a public zone; [CR#608.2h] says an
effect that needs information from an object moved "from a public zone to a hidden zone" uses its
last known information. The checker models the first and has no room for the second.

## Options — pending owner ruling

- **(a)** Amend the ADR and checker: an object a cost (or a linked ability) moved from a public
  zone to a hidden zone stays visible for last-known-information reads (characteristics and the
  combat relation `attackedBy`), but is not actionable. Touches `publicOnly`, `linkedResult` and
  the LKI paragraph.
- **(b)** Bind the defender inside the cost, before the creature leaves the battlefield, so the
  effect reads a public referent (the player, planeswalker or battle) rather than the returned
  creature.
- **(c)** Another encoding the owner prefers.

## Drafts (validated up to the refusal)

The defender template gains an optional parameter ("tapped and attacking <defender>" is printed
wording; with none named the controller chooses [CR#508.4]):

```lean
semantic_template putOntoBattlefieldTappedAttacking (subject : NounPhrase)
    (from_ : Location := .anyLocation) (defender : Option NounPhrase := none) : Action :=
  zoneChange subject battlefield from_ [(.entersAs .tapped), (.entersAttacking defender)]
```

Ninjutsu ends in `(putOntoBattlefieldTappedAttacking this_ yourHand returnedCreaturesDefender)`,
with `returnedCreaturesDefender` the noun phrase above bound by a `let`. Sneak gains, after its
cast permission:

```lean
static
  (replace (would thisPermanent enter)
    (doIf (keywordCostPaid (.named "sneak") thisPermanent)
      (withRiders [.entersAs .tapped, .entersAttacking (some returnedCreaturesDefender)])))
```

Pins (`Proofs/Turn.lean`), which held by `rfl`: the ninjutsu definition's activated effect equals
`putOntoBattlefieldTappedAttacking this_ yourHand returnedCreaturesDefender`, and sneak's second
part equals the static above.

v2 mirrors (`plugins_v2/builtin/macros`), which passed `cargo test -p deckmaste_semantics_v2`:

- `instructions/putOntoBattlefieldTappedAttacking.ron` gains
  `"defender": Default(Any, None)` and `EntersAttacking(Param(defender))`.
- a new `abilities/entersAttacking.ron` (`EntryRider(Param(subject),
  EntersAttacking(Param(defender)))`), because a keyword body may not write `EntryRider`
  directly (`keyword_bodies_are_written_in_card_vocabulary`).
- ninjutsu's defender: `the(and([or([anyPlayer, planeswalker, hasType(Battle)]),
  inCombat(AttackedBy, linkedObject(ByDeed(Return), Return, this))]))`.
- sneak: `static(onlyWhile(entersTapped(thisPermanent), keywordCostPaid(sneak, thisPermanent)))`
  and `static(onlyWhile(entersAttacking(thisPermanent, the(and([…, inCombat(AttackedBy,
  linkedObject(TheAlternative, Return, thisPermanent))]))), keywordCostPaid(sneak,
  thisPermanent)))`.

The Rust `Attacking(Option<Reference>)` (`crates/deckmaste_core/src/action.rs`) can already carry
the defender.
