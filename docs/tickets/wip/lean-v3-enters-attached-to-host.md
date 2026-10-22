---
needs: [lean-v3-keyword-triggers-entries-and-parties]
---
# An Aura put onto the battlefield attached to a host the effect names

Standard constraints apply. Split from `lean-v3-keyword-triggers-entries-and-parties` §3 (review
finding M2).

No Entry Rider writes "onto the battlefield attached to <host>", so an effect that names the
Aura's host has no Lean form. *Academy Researchers* ("you may put an Aura card from your hand onto
the battlefield attached to Academy Researchers") and *Bruna, Light of Alabaster* ("you may put
onto the battlefield attached to it any number of Aura cards that could enchant it from your
graveyard and/or hand") need it.

Change:
- Add an "enters attached to <host>" Entry Rider (`MoveRider`), riding a move to the battlefield
  or a token's creation only, as the other Entry Riders do [CR#614.1c]. Mirror it in
  `plugins_v2`.
- An Aura so put enters attached to the named host. If it can't legally enchant that host, or the
  host is undefined, it doesn't enter: it stays in its zone, goes to its owner's graveyard from the
  stack, and a token Aura isn't created [CR#303.4i]. A permanent that isn't an Aura, Equipment or
  Fortification enters unattached [CR#303.4h].
- Enchant's [CR#303.4f] replacement (`Definitions/Keywords.lean`, `enchant`) must not apply when
  the entering names its host: [CR#303.4f] covers only an effect that "doesn't specify the object
  or player the Aura will enchant". Today it applies to every entry not from the stack.
- Pin both cards and the specified-host exclusion.
