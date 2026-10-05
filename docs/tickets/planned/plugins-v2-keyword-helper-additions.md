---
needs: [plugins-v2-implicit-actor-spelling]
---
**New helpers the keyword bodies asked for, and the small fixes found beside
them.** Split from `plugins-v2-keyword-body-defects` on 2026-10-05. The
helpers that write a performer are spelled with the implicit actor, so the
ticket as a whole follows `plugins-v2-implicit-actor-spelling`; the items
marked *independent* below do not depend on it and may land earlier, alone or
together. Standard constraints apply.

## New helpers

The owner accepted these names on 2026-10-05 ("getsBoth is a strange name to
me but sure I guess"). Re-spell each caller listed onto the helper.

- `mayOrElse(body, otherwise)` — "you may …; if you don't, …". Fabricate,
  fading and madness fall to named calls today because the "if you do" slot
  comes first. The 2026-10-04 proposal took a player first; under the actor
  it does not. Needs B.
- `getsBoth(subject, delta, duration)` — one delta to both power and
  toughness, "+X/+X" (melee, rampage). *Independent* of B. If
  `semantics-v2-gets-both-deltas` has landed, write it over the
  one-modification form that ticket introduces; otherwise over `gets`, and
  that ticket re-spells it.
- `createTokenCopy(source, riders)` — myriad's tapped-and-attacking token copy.
  Writes a creation, so needs B.
- `artifactCreatureToken(power, toughness, colors, subtypes, abilities = [])` —
  a token spec with the extra card type (fabricate's Servo). *Independent*
  (a spec, no performer).
- `mayCastFrom(what, zone, paying)` — a general cast permission whose permitted
  player is the actor (`airbend`, aftermath). Airbend's "its owner may cast
  it" is `act(ownerOf(it), establish(mayCastFrom(it, exileZone, mana([2]))))`;
  this relies on `semantics-v2-actor-handoff`'s prototype finding that `actor`
  reads as the controller inside a static ability. Needs A and B.
- `cantBeBlockedByFewerThan(subject, n)` — the bound in menace. *Independent*.
- Event helper `discards(player, card)`, and **Megrim** joins canon to prove
  "whenever … discards": "Whenever an opponent discards a card, this
  enchantment deals 2 damage to that player." The event is already pinned in
  Lean ("Whenever you discard a card", `Proofs/Anaphora.lean`
  `okDiscardFromHand`). *Independent* of B, but it is the proof the
  actor-handoff design names for discard triggers, so land it no earlier than
  `semantics-v2-actor-handoff`.

## Small fixes

- `anotherSpell` becomes `and([spell, otherThan(thisSpell)])`, matching Lean
  `stormExpansion` (`Macros.lean` ~L1146); storm, split second and surge call
  it. Today it is `And([InZone(stack), Not(AbilityHead(AnyOnStack)),
  OtherThan(thisSpell)])`. *Independent*.
- Delete the RON helper `returnToBattlefieldWithCounters`
  (`macros/instructions/returnToBattlefieldWithCounters.ron`): it has no
  callers, and persist and undying already go through `returnToBattlefield`.
  The old ticket's "one of the two is wrong" was mistaken. The Lean macro of
  the same name is used by the bench and stays. *Independent*.
- Add `isCard` to the Lean bench where RON writes `cardIn`: the bench writes
  "a card in hand" without it, and the conjunct is meaningful because a token
  can sit in a hand, graveyard or exile until state-based actions are checked
  [CR#111.7,704.5d]. *Independent*.
- Fix the `amass` comment to the rule's wording, "Put N +1/+1 counters on that
  creature" [CR#701.47a], not "on it" (`semantics-v2-actor-handoff` re-spells
  `amass` and may do this first). *Independent*.
- Reorder helper parameters freely so required ones precede defaulted ones,
  removing the named calls forced by order today: `returnToBattlefield`
  (riders before agent and origin), `activated` (limit before guard),
  `deonticRule`, `damage`, `verbedEvent`, `addMana`. List every reorder in the
  landing record. `returnToBattlefield` and `addMana` lose their agent in B, so
  reorder them after it; the other four are *independent*.
- `transfigure` and `transmute` write `Pro(Bare, One, top(1))` for the card
  their search found; use the existing `foundCard` helper if the Lean check
  accepts it, and strike them from `ALLOWED_RAW`
  (`crates/deckmaste_semantics_v2/tests/keyword_bodies.rs`). If the check
  refuses, they stay and the record says why. *Independent*.

## Proof

- Each helper has at least one caller re-spelled onto it, and Megrim checks in
  `cargo xtask lean-check`.
- A re-spelling onto a helper leaves `cargo xtask expansions` byte-identical;
  `anotherSpell`, the `isCard` conjuncts and Megrim are term changes and are
  listed as such.

## Out of scope

"Gets" writing its subject twice (`semantics-v2-gets-both-deltas`).
