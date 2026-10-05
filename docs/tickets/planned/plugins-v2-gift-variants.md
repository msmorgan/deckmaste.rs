---
needs: [semantics-v2-actor-handoff, plugins-v2-implicit-actor-spelling]
---
**Gift becomes one unspelled `gift(effect)` declaration plus six spelled
variants.** Split from `plugins-v2-keyword-body-defects` on 2026-10-05.
Standard constraints apply.

## Why

`keyword_abilities/gift.ron` declares `params: [Subject]` and carries a STOP:
"Gift a [something]" is a fixed additional cost plus a second ability whose
effect "is defined by the [something] listed" [CR#702.174a,702.174b], and a
noun-phrase parameter cannot supply that effect. So `gift` cannot be invoked.

## What is decided

Owner, 2026-10-05: "bare gift declaration must exist, I think, for the "gift
was promised" part and the keyword/nominal declarations for parsing. giftACard
= gift(draw(1)) with the "Gift a card" spelling on the macro or something like
that. bare `gift` is "private" by way of not having its own spelling and
therefore being unparseable".

- **Bare `gift(effect)` stays, unspelled.** It takes the effect as an
  instruction and writes both abilities: the optional additional cost "you may
  choose an opponent" [CR#702.174a], and the second ability, which hands the
  effect to the chosen player — `act(the(chosenPlayer), effect)` — under "if
  its gift cost was paid" on a permanent's enters trigger or an instant or
  sorcery's spell ability [CR#702.174b]. With no spelling it cannot be parsed.
  It is what "the gift was promised" [CR#702.174k] and "gives a gift"
  [CR#702.174c] name.
- **Six spelled variants**, one declaration each, each `gift(<effect>)` with
  its own spelling [CR#702.174d..702.174i]:

  | Macro | Spelling | Effect (performed by the chosen player) |
  |---|---|---|
  | `giftAFood` | "Gift a Food" | creates a Food token |
  | `giftACard` | "Gift a card" | draws a card: `gift(draw(1))` |
  | `giftATappedFish` | "Gift a tapped Fish" | creates a tapped 1/1 blue Fish creature token |
  | `giftAnExtraTurn` | "Gift an extra turn" | takes an extra turn after this one |
  | `giftATreasure` | "Gift a Treasure" | creates a Treasure token |
  | `giftAnOctopus` | "Gift an Octopus" | creates an 8/8 blue Octopus creature token |

- **Excluded:** "Gift a Rhystic Study" (Archival Whorl) is not Vintage-legal.

## The work

1. Retype `gift`'s parameter to an instruction and write its body; remove its
   `spelling`. The `KeywordAbility` meta-macro requires `spelling` today
   (`macros/meta/KeywordAbility.ron`): make it optional or give gift a
   different declaration kind, whichever keeps the keyword and nominal
   declarations the parser needs. If neither works without an expander
   feature, STOP and report.
2. Add the six variants. Each effect is written in the implicit-actor spelling
   (`plugins-v2-implicit-actor-spelling`); add any token or extra-turn helper
   the effects need and list it.
3. Check the variants in Lean (a pin each, or `lean-check` over a canon card
   per variant if one is added) and that the English side still reads every
   printed "Gift a …" line in the corpus.

## Proof

`cargo xtask lean-check` passes with a pin per variant; `cargo xtask
expansions` shows `gift` and the six new declarations and nothing else.

## Out of scope

The "gives a gift" trigger [CR#702.174c] beyond what the bare declaration
needs to be nameable.
