---
needs: [semantics-v2-actor-handoff, plugins-v2-implicit-actor-spelling, semantics-v2-keyword-definition-by-card-class, lean-keyword-definition-regimes, semantics-v2-linked-choice-readback]
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

- **A variant carries the "Gift" label** through a pass-through field, so its
  body IS `gift(draw(1))` under that label with its own spelling. Owner,
  2026-10-05: "yes to 2". (Without it the loader labels `giftACard` a keyword
  of its own, "GiftACard", with no facts row — see F4.)
- **Decided 2026-10-06:** gift's permanent and spell forms are one
  definition holding both halves, each guarded by the card's class with the
  existing conditional forms, not a new selector device
  (`semantics-v2-keyword-definition-by-card-class`).
- **Three model gaps, each its own ticket, in this order** (decided
  2026-10-06): these are three gaps in the model that gift exposed, each its
  own ticket.
  1. `semantics-v2-keyword-definition-by-card-class`: the class guard, and a
     spell ability [CR#113.3a] admitted in a keyword definition.
  2. `semantics-v2-linked-choice-readback`: the chosen player read back
     through a paid-cost facet.
  3. `lean-keyword-definition-regimes`: each part checked against its own
     regime.

  Then gift. No stub lands before them: a stub body would be a negative
  oracle.
- **Excluded:** "Gift a Rhystic Study" (Archival Whorl) is not Vintage-legal.

## Stopped 2026-10-05

The first attempt wrote the faithful body and ran the checker on probe cards;
it committed nothing and the ticket returned to `planned/` (owner: "yes to
3"). Findings:

- **F1** — a keyword definition is one fixed list with no per-class part, and
  gift's spell form is a spell ability, which `Ability.category`,
  `AbilityCategory` and the facts generator all refuse [CR#702.174b,113.3a].
  → `semantics-v2-keyword-definition-by-card-class`.
- **F2** — the permanent form's enters trigger fails the keyword-regime law
  (gift's row is `atCasting`; an enters event has no regime), the law already
  blocking Offspring and Squad. → `lean-keyword-definition-regimes`.
- **F3** — "the chosen player" cannot be read back: the opponent chosen as the
  additional cost is out of the second ability's scope (`choiceRef theChoice
  player 0`), and `PaidFacet` reads no player [CR#607.2d].
  → `semantics-v2-linked-choice-readback`.
- **F4** — a variant `giftACard` is labelled "GiftACard" by the loader
  (`keyword_ability_label`, `crates/deckmaste_semantics_v2/src/keywords.rs`)
  with no facts row; writing `body: [gift(draw(1))]` nests a keyword inside a
  keyword, which has no category and is refused. Answered by the decided
  pass-through label (step 2).
- **F5** — `Shape::from_params` (`crates/xtask/src/facts.rs` ~L102) has no case
  for an `Instruction` parameter, and `forwarded_keyword_params` rejects one
  unless the file writes `keyword_params: []`. Work step 1.

## The work

Blocked steps name their gap ticket; the rest can start once the existing
needs land.

1. **Let a keyword declaration omit `spelling`.** Decided 2026-10-05: the
   keyword declaration format allows a missing `spelling` for a declaration
   that is deliberately unparseable (the bare `gift`). Owner: "yes". Make
   `spelling` optional in the `KeywordAbility` meta-macro
   (`macros/meta/KeywordAbility.ron`) and the declaration schema, keeping the
   keyword and nominal declarations the parser needs. Then retype `gift`'s
   parameter to an instruction and remove its `spelling`. F5 belongs here:
   the generator maps gift's `Instruction` parameter to no keyword argument —
   `gift.ron` writes `keyword_params: []` (or `forwarded_keyword_params`
   learns to forward an `Instruction` as nothing, as it does `Ability`), and
   `Shape::from_params` admits the signature as `Nothing`.
2. **The pass-through label** (decided, F4): a variant declaration names the
   keyword whose label it carries, so `giftACard`'s body is `gift(draw(1))`
   labelled "Gift" and the facts row is gift's.
3. **Write gift's body.** Blocked: the per-class second ability on
   `semantics-v2-keyword-definition-by-card-class` (F1); the permanent form's
   enters trigger on `lean-keyword-definition-regimes` (F2); the chosen
   player as performer on `semantics-v2-linked-choice-readback` (F3).
4. Add the six variants. Each effect is written in the implicit-actor spelling
   (`plugins-v2-implicit-actor-spelling`); add any token or extra-turn helper
   the effects need and list it. Needs steps 2 and 3.
5. Check the variants in Lean (a pin each, or `lean-check` over a canon card
   per variant if one is added) and that the English side still reads every
   printed "Gift a …" line in the corpus. Pins cover both classes for "Gift a
   card" and "Gift a tapped Fish".

## Proof

`cargo xtask lean-check` passes with a pin per variant; `cargo xtask
expansions` shows `gift` and the six new declarations and nothing else.

## Out of scope

The "gives a gift" trigger [CR#702.174c] beyond what the bare declaration
needs to be nameable.
