---
needs: []
---
**Draw is not a keyword action.** Retire `Action::Composite { name: "Draw", … }`
in favour of a bodiless `PlayerAction::Draw` carried by `Action::By(who, …)`.

## Why

Draw was modelled as a `Composite` on the premise that it is a CR 701 keyword
action. It is not. CR 701 lists 69 keyword actions (701.2 Activate … 701.69
Heal); drawing is **[CR#121.1]**, a section-1 Game Concepts rule — the same tier
as damage ([CR#120.3]) and counters.

Mis-filing alone would be cosmetic. The modelling error is that the composite
body is *wrong*, not merely mis-labelled:

> **[CR#121.5]** If an effect moves cards from a player's library to that
> player's hand without using the word "draw," the player has not drawn those
> cards. This makes a difference for abilities that trigger on drawing cards and
> effects that replace card draws, as well as if the player's library is empty.

`Action::draw_one` stores the body `Each(TopOfLibrary(1, whose) → Move(It,
Hand))`. [CR#121.5] says that expansion **is not a draw**. `Composite`'s
invariant is "the verb's meaning IS its `body`" — draw is the one member that
cannot satisfy it, because the CR defines its decomposition as a different
event.

Contrast **Mill**, which really is [CR#701.17] and really is composite:
[CR#701.17a] defines it as exactly "puts that many cards from the top of their
library into their graveyard," with no [CR#121.5]-style non-reducibility clause.
Mill's body is its meaning. Draw's is not.

Three places in the model already argue against the composite:

1. **`draw_one`'s own doc comment disclaims the `Composite` invariant**
   (`crates/deckmaste_core/src/action.rs`): *"The body is NOT the executor …
   the stored body is the faithful render/re-emit facet only."* The exception
   was written down instead of read as a signal.
2. **Idris already treats Draw as a first-class *event*** — `MkEventQuery
   [Draw]` and `Static (Replaces (MkEventQuery [Draw] …))`
   (`idris/src/Cards.idr`). Draw is irreducible on the event side and composite
   on the action side, in one model.
3. **The keyword-action name namespace holds seven real 701 actions and one
   impostor** (`idris/src/Core.idr` `n`, mirrored in
   `crates/deckmaste_cards/src/idris_emit.rs`): Scry [CR#701.22], Surveil
   [CR#701.25], Fateseal [CR#701.29], Mill [CR#701.17], Discard [CR#701.9],
   Destroy [CR#701.8], Fight [CR#701.14] — and Draw.

Draw also carries draw-only machinery no zone-move has: the empty-library loss
([CR#121.4], [CR#104.3c]), the number-of-cards replacement stage ([CR#121.2a]),
and the face-down-until-cast rule ([CR#121.8]).

## Target shape

`PlayerAction` is the enum for player-agent verbs, and [CR#121.1] is "*a player*
draws a card". Today the performer has no slot — `whose` rides the body's
selection so the engine can scrape it back off. `By` gives it a real one:

```
OneShotEffect::Batch(count, Act(Action::By(who, PlayerAction::Draw)))
```

The two levels map the CR directly: the `Batch` is the *instruction* level that
number-referring replacement effects modify ([CR#121.2a]), each element is one
individual card draw ([CR#121.2]). No body, no carve-out doc, no performer
scrape. `PlayerAction::VentureIntoDungeon` is the precedent for a bare,
argument-free player verb.

This is mostly *deleting* a fiction — the engine already does not execute that
body — rather than building machinery.

## Scope

1. **Core** (`crates/deckmaste_core`): add `PlayerAction::Draw`; retire
   `Action::draw_one`; re-point `OneShotEffect::draw` at `Batch(count,
   Act(By(who, Draw)))`. Drop the "body is NOT the executor" carve-out.
2. **Engine**: the `Act(Draw)` resolve path keeps its late top-of-library bind
   and its empty-library check *before* the move — that behaviour is correct and
   must not regress; it simply stops being justified by a stored body.
3. **Cards** (`crates/deckmaste_cards`): remove `"Draw"` from the keyword-action
   arms in `idris_emit.rs` (`"Mill" | "Draw"`, the `"Destroy" | "Discard" |
   "Draw" | …` arm, and the `vec!["Draw"]` actor facet); re-point the `Draw`
   macro.
4. **Idris** (`idris/src/Core.idr`): remove `Draw` from the `n` keyword-action
   namespace; give it its player-action home. Regenerate `plugins/wizards`
   (grammar changed).
5. **Gate set**: drop `"Draw"` from `keyword_action_only` in
   `crates/deckmaste_cards/tests/no_dead_grammar.rs` — that set is documented as
   "[CR#701] keyword-action NAME"s, which Draw is not.

## Drive-bys (in scope — same lines)

- **Wrong-topic citation.** `draw_one`'s doc cites `[CR#120.3,104.3c]` for the
  empty-library loss. [CR#120.3] is *damage results*; the rule wanted is
  **[CR#121.4]**. Exactly the right-number-wrong-topic class the hash checker
  cannot catch.
- **Stale docs.** `crates/deckmaste_cards/src/macros.rs` and
  `crates/deckmaste_cards/tests/no_dead_grammar.rs` still describe `Draw(1)` as
  `Action::By(You, Draw(1))` — wrong against today's composite, correct again
  under the target shape. Verify rather than assume they need no edit.

## Gate

Standard constraints apply (fmt, clippy, CR-citation check/audit, wizards
regen). Deltas worth naming:

- `cargo xtask cite audit --diff` must be read by eye for the new [CR#121.x]
  cites — this ticket exists *because* a citation was right-numbered and
  wrong-topiced.
- `cd idris && ./scripts/build` PASS; `cargo xtask idris-check plugins/canon` no
  regressions; `cargo xtask fidelity` PASS (render fidelity for draw cards must
  hold — the removed body was the render facet, so the renderer must get its
  text from the verb name instead).
- An engine test pins the empty-library case: drawing from an empty library
  still loses the game ([CR#121.4]) rather than silently no-opping.
- `rg -n 'Composite.*"?Draw' crates/ idris/` returns nothing.
