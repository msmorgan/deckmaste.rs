# Card authoring binds no implicits

> Workbench succession (2026-09-05): current workbench evidence lives in
> `lean/`; Idris paths below are historical. See
> [Lean is the workbench](lean-is-the-workbench.md).

## Decision

A card in the authoring bench never binds an implicit argument. No proof
binding (`{ok = …}`, `{na = …}`), and no default keyword argument
(`{param = Just …}`, `{verb = Cast}`). A card is constructors and card-language
functions applied to positional arguments, the way a Haskell term is.

Where a construction has an optional slot or an obligation the elaborator
cannot discharge from the card's own words, the card language supplies a
**wrapping macro** — a plain, total function in `Experimental/Macros.idr` — that
takes the slot positionally, names itself after the English it spells, and
fills the implicit itself. The card calls the macro.

**Amended 2026-08-22 — constructions take every parameter as a required
positional.** `{default …}` slots on core constructors are banned outright,
not only hidden from cards. The reason is translation, not convenience: a
realised card macro tree must translate into an Idris term positionally,
term-for-term, so a card can be verified later with no defaulting or
permutation step between the macro language and Idris. Optional content is an
explicit argument (`Maybe`, a riders record, …) and the common case is a
wrapping macro. `{auto 0 …}` proof gates are not slots and are unaffected.
Sweep: `workbench-no-default-slots`.

Proof obligations are discharged by `auto` search or supplied inside a macro,
never by the card. A macro forwards each obligation that mentions one of its own
arguments as its own `{auto 0 …}` parameter, so the search still runs at the
call site against the card's concrete words; it never weakens a gate and never
reaches for `believe_me` or `assert_total`.

## Rationale

An implicit binding at a card site is elaboration leaking into authoring: the
card is being asked to name a proof term or a defaulted field that the reader of
the card text has no reason to know. It also makes the bench a bad witness —
what is exercised is the elaborator's failure mode rather than the language a
card is meant to be written in.

Naming the slot in the macro instead puts the choice where the English is. A
trigger with an intervening-if clause becomes `triggeredIf`; a keyword with a
cost parameter becomes `keywordCosting`. The default value stops being an
invisible fact about the constructor and becomes a visible word.

Macros stay declarative in the sense of [Macros are declarative](macros-are-declarative.md):
they are typed positional templates over the semantics, not a second language.

## Consequences

New constructions plan their authoring surface, not just their type: any slot a
card would have to bind arrives with the macro that spells it. A slot bound at
most bench sites is not optional and should be positional on the construction
instead.

`grep -cE '\{[a-zA-Z_]+ *= ' idris/src/Experimental/Cards.idr` must stay `0`;
a new binding there is a missing macro, not an authoring style. This binds the
bench only — `Experimental/Proofs*.idr` bind implicits deliberately, because a
pin's whole subject is the proof term it refuses to admit.

## Tracked references

- [Macros are declarative](macros-are-declarative.md)
- [The kind index joins; union marking is spelling](kind-index-joins-union-marking-is-spelling.md)
- [Idris is a soundness gate](idris-is-a-soundness-gate.md)
