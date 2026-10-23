---
needs: [lean-v3-keyword-facts-deleted]
---
# A shared sentence factored into a modal header ("Choose one. Until end of turn, target creature ... that ability.")

Standard constraints apply. A design the owner favours, not yet ruled.

## Problem

*Wild Shape* and *Call Damage Control* factor a shared sentence into the "Choose <qty>." header
over bullets that are not clauses. They factor opposite halves:

- *Wild Shape*: "Choose one. Until end of turn, target creature you control has that base power and
  toughness, becomes that creature type, and gains that ability." with bullets "1/3 Turtle with
  hexproof.", "1/5 Spider with reach.", "3/3 Elephant with trample." The header holds target,
  duration and verb frame; each bullet is a creature shape.
- *Call Damage Control*: "Choose up to two. Return those cards from your graveyard to your hand."
  with bullets "Target artifact card." (creature, enchantment, land likewise). The bullets hold the
  targets; the header holds the verb and "from your graveyard", reached through the cataphoric
  "those cards".

## Design (approved by the owner, 2026-10-23)

Keep `Action.chooseModes` taking complete modes, with no cataphoric plumbing in the Core AST or
the checker. Factor the header at the template layer with a higher-order splice template:

```lean
semantic_template chooseModesOver {α : Type} (quantity : Quantity) (items : List α)
    (frame : splice (α → Action)) : Action :=
  chooseModes quantity (items.map frame)
```

- *Call Damage Control*: `chooseModesOver (upTo 2) [target (artifact card), …]
  (fun cards => returnToHand cards (from_ := yourGraveyard))`.
- *Wild Shape*: `chooseModesOver (exactly 1) [shape 1/3 turtle hexproof, …]
  (fun shape => becomesShape (target (and [isCreature, youControl])) shape untilEndOfTurn)`.
- `Authoring.inspectCore` sees the call before reduction, so the header and each bullet are written
  once.

Implementation: in `lean/Semantics/Authoring.lean` the `.lam` arm binds splice parameters only when
the type is `NounPhrase`; generalise that guard to `containsExpression (← getEnv) type`.

Why no cataphora in the checker: modes are chosen [CR#601.2b] before targets are [CR#601.2c]; with
"up to two", "those cards" may be one card or none; and modal branches export no cross-branch plural
([CR#700.2a,700.2c]: a mode with targets is chosen or not, and targets of an unchosen mode are
not chosen).

## VERIFY results (2026-10-23)

- Corpus claim: confirmed by heuristic only. A scan of `data/derived/cards.jsonl` `text` for a
  "Choose …" header with text after the header sentence and before the first bullet finds 72 cards;
  filtering for bullets that are not clauses leaves *Wild Shape* and *Call Damage Control*. The
  other filter survivors (*Widow's Bite*, *Vision Charm*, *Kemba's Outfitter*) have clausal bullets.
  Not an exhaustive proof: the regex looks at the first header only.
- Oracle: both cards match the text above in `data/derived/cards.jsonl`. Vintage legality: both
  `legal` in `data/scryfall/oracle-cards.jsonl` (`legalities.vintage`); `cards.jsonl` carries no
  legality field.
- `Action.chooseModes`: it is `chooseModes (quantity : Quantity) (modes : List (Option Cost ×
  Option Nat × Action))` (`Abilities.lean` ~1276, not ~1250). Quantity is confirmed, but the modes
  are triples, not bare Actions, so the template body needs `items.map (fun i => (none, none,
  frame i))`, not `items.map frame`.
- `returnToHand`: `returnToHand (subject : NounPhrase) (from_ : Location := wherever) : Action`
  (`Templates.lean` ~1536). Confirmed.
- `CharacteristicBundle` (`Abilities.lean` ~755): a `Characteristics` plus `qualities`;
  `Characteristics` has `types`, `subtypes`, `text : List Ability`, `power` and `toughness`. It can
  hold the 1/3 Turtle with hexproof. Whether an existing action applies it as "has that base P/T,
  becomes that type and gains that ability" for a duration was not checked; `becomesShape` above is
  a placeholder name.
- `semantic_template` with `{α : Type}`: not confirmed. `elabAuthoredDecl` (`Authoring.lean` ~177)
  passes each binder through unchanged except splice-annotated ones, so an implicit binder should
  elaborate, but no template with an implicit type parameter exists today, and `caller_splice_type%`
  wraps `α → Action` as a forall over `AuthoringContext`. Needs a scratch probe.
- `Authoring.lean` guard: confirmed at ~364: `(← whnf type).isConstOf `Semantics.NounPhrase` is
  the only case that binds a splice parameter. A `α → Action` lambda falls to the final
  `inspectCore` line and is not bound.
- Cataphora rejected: [CR#601.2b] "If the spell is modal, the player announces the mode choice";
  [CR#601.2c] targets are announced after; [CR#700.2a] modes are chosen as part of casting;
  [CR#700.2c] a mode's targets are chosen only if that mode was. Read in `data/rules/cr.txt`.

## Tests

- *Call Damage Control* and *Wild Shape* check clean, each header and bullet written once.
- `chooseModesOver` reduces to the same `chooseModes` as the written-out card.
- A negative: a frame whose lambda ignores its argument gives a modal ability with identical modes.
