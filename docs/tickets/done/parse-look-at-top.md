---
needs: []
---
Parse the **single-card** look-at-top-of-library family (shapes 4 & 5). The
peek-N-and-keep-some shapes ("Look at the top N cards … put one into your hand
and **the rest** on the bottom") are **deferred** — they need a
complement/"the rest" selection primitive that does not exist in core/engine
(its own ticket: `balance-choose-complement-player-selection`, "no primitive
for 'keep N of a set, act on the REST'"). Only the single-card shapes are
parser-only; the ticket's original "~484 parser-only" premise was wrong (the
bulk of the 484 are the complement-blocked shapes 1–3).

## Scope (this ticket)

- **Shape 5** — "Exile the top card of your library." (bare move; the impulse
  "you may play it this turn" tail stays blocked by `engine-cast-from-zones`).
- **Shape 4** — "Look at the top card of your library." + a single-card tail:
  "you may reveal it and put it into your hand" (+ type-gate "If it's a
  `<type>` card, …"); "you may put it into your graveyard" (= Surveil-1
  reminder shape).

No engine work: every primitive exists — `Existing(TopOfLibrary)`, `Move`, the
`Reveal` seam (`engine-explore`, done), `May`, `If`/`Matches`, and the
`That(Card)` anaphor.

## Approach — whole-shape bidirectional effect macros (mirror `Explore.ron`)

No Rust parser arms and no Rust render arms (USER RULING: macros, not
hand-rolled recognizers). Each shape is ONE effect-kind `.ron` macro whose
`template` is the **entire** English (one OR two sentences) and whose body is
`Each(Existing(TopOfLibrary(count: 1)), <body>)` — structurally identical to
`Explore.ron`, which already peeks a single top card and acts on it as the
per-iteration anaphor `It`. Parse emits the macro invocation, render fills the
same template (template-first, like `Scry`/`Investigate`/`Amass`) — render
round-trips for free.

Two-sentence shapes are handled by the whole-body macro fallthrough, NOT by
`parse_sequence` composition: for a body like "Look at the top card of your
library. You may reveal it and put it into your hand.", the earlier
`CLAUSE_PARSERS` all decline (`parse_if` — its base has no look-at production;
`parse_may` — body doesn't start with "you may"; `parse_sequence` — its first
sentence has no production), so `parse_macro_effect` (last in the list) receives
the whole body and matches the multi-sentence template. Multi-sentence templates
are already supported (`DestroyNoRegen.ron`: `"destroy ${0}. It can't be
regenerated"`). Because there is deliberately **no** standalone "look at the top
card" head production, `parse_sequence` never wins — keep it that way. The
peeked card is `It` inside the one nested `Each` (no cross-sentence anaphor, no
`That(Card)`, no `it → This` slot-reader hazard).

Macros:

| macro | template | body (inside `Each(Existing(TopOfLibrary(count: 1)), …)`) |
|---|---|---|
| `ExileTop` | `exile the top card of your library` | `Move(It, Exile)` |
| `LookTopRevealToHand` | `look at the top card of your library. you may reveal it and put it into your hand` | `May(effect: Sequentially([Reveal(what: It), Move(It, Hand)]))` |
| `LookTopRevealToHandIfType` | `look at the top card of your library. if it's a ${0} card, you may reveal it and put it into your hand` | `If(Matches(It, Type("${0}")), May(effect: Sequentially([Reveal(what: It), Move(It, Hand)])))` |
| `LookTopMayGraveyard` | `look at the top card of your library. you may put it into your graveyard` | `May(effect: Move(It, Graveyard))` — include only if a real card needs it (Surveil-1 covers most) |

The `${0}` type slot in `LookTopRevealToHandIfType` is the one open detail —
declare its real `params` type (a card-type name reader) rather than `Any`; land
the untyped `ExileTop`/`LookTopRevealToHand` first and add the type-gate macro
second.

## Graduating cards (real, from the todo corpus)

- Exile-top: Precognition Field, Mystic Forge, Thought Lash.
- Reveal-to-hand (type-gated): Frost Augur, Dryad Greenseeker, Domri Rade `[+1]`.

## Verify

`cargo xtask generate plugins/wizards` graduation delta (before/after
"graduated N"); `cargo xtask fidelity plugins/wizards` round-trip on an
exile-top and a reveal-to-hand card. Standard constraints apply.

## Deferred (not this ticket)

- Shapes 1–3 (peek-N + "the rest") → `balance-choose-complement-player-selection`
  (the complement primitive).
- IMPULSE tail ("you may play it this turn") → `engine-cast-from-zones`.
