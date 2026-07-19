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

## Approach — bidirectional effect macros, sentence-composed

No Rust parser arms and no Rust render arms (USER RULING: macros, not
hand-rolled recognizers). Each production is an effect-kind `.ron` macro with an
English `template`; parse emits the macro invocation, render fills the same
template (template-first, like `Scry`/`Investigate`/`Amass`) — so render
round-trips for free. Multi-sentence cards ride the existing `parse_sequence`
(`crates/deckmaste_migrations/src/parsers/effect.rs`): each sentence matches its
own macro, joined into `Sequentially`; the tail reads the peeked card via the
`That(Card)` anaphor (`parse_sequence`'s documented contract — later sentences
read the first sentence's binds/products as `That(Card)`). "it" is **literal
template text**; the macro bodies hardcode `That` — this sidesteps the
`it → Reference::This` self-reference slot-reader (`effect.rs`
`macro_slot_reader`/`self_reference`), which would otherwise mis-bind the
anaphor to the printed-on object.

Macros:

| macro | template | body |
|---|---|---|
| `ExileTop` | `exile the top card of your library` | `With(Existing(TopOfLibrary(1)), Move(That, Exile))` |
| `LookAtTop` (head) | `look at the top card of your library` | publish the top card as `That(Card)`; grant controller look-visibility; **no move** |
| tail → hand | `you may reveal it and put it into your hand` | `May(Sequentially([Reveal(That), Move(That, Hand)]))` |
| tail type-gate | `if it's a ${0:Type} card, you may reveal it and put it into your hand` | `If(Matches(That, Type(${0})), May(…hand))` |
| tail → graveyard | `you may put it into your graveyard` | `May(Move(That, Graveyard))` — include only if a real card needs it (Surveil-1 covers most) |

## Spike (do this first)

Confirm how a **standalone** `LookAtTop` head publishes `That(Card)` to the
*next* sentence. `parse_sequence` says later sentences read the first
sentence's product/announce as `That(Card)`, but "look at" binds an *existing*
card rather than producing a token — verify the peeked bind lands on the
antecedent stack for siblings (likely a `With`/look-bind node; the peek
visibility is granted at `Existing(TopOfLibrary)` binder resolution per
`engine-scry-recompose-ordered-move`). If a small core assist is needed to
publish `That(Card)`, report before building the tails on top.

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
