---
needs: [english-v2-require-through-optional-role]
---
Control role as a declared frame valence, not a preposition-class fence
(require-through landing review HIGH-1). The dissolution of `ControlPhrase`
replaced `require relation is Under` with `require control.preposition_attachment
is SelectedOnly`, and `SelectedOnly` = {into, onto, to, under}. Result: `Put
target creature card onto the battlefield into your graveyard.` and `Return
target creature card to your hand into your graveyard.` now parse (parse
failures on the parent), and three lock identities — Cavalier of Thorns,
Animal Magnetism, Genesis Ultimatum, all `Put X onto the battlefield and the
rest into your graveyard` — were blessed with destination = ["the
battlefield", "the rest"] and control = `into your graveyard`.

Pin: the control complement `under <player>'s control` is a verb-selected
preposition, so it lives at the frame as a declared optional role whose
preposition is `under` on the valence row (the recorded rule "verb-selected
prepositions stay frame-level; the general PP is for everything else"). The
fence reads the frame's declared role preposition through the generated
accessor; no preposition class, no per-construction hand-attached guard
(derive from the valence row for every frame that declares the role, including
`put_onto_source_after`). Both negative oracles above are pinned as `is_err()`.

The three wrongly blessed identities lose their sole reading. Their correct
analysis (a gapped coordinated destination: `onto the battlefield and the rest
into your graveyard`) is not expressible today, so they retire —
`retirement/re-coverage obligation: Cavalier of Thorns, Animal Magnetism,
Genesis Ultimatum` — and are routed to the `Gerund clauses and modal
ellipsis (A9, A11)` entry in `../fog.md` (gapped coordination: `… onto the
battlefield and the rest into your graveyard`), which lists them as re-coverage
targets; they are re-covered when that entry graduates and lands. Coverage 16,771 → 16,768 is the expected, ruled
decrease (coordinator ruling: a decrease is permitted only for identities
whose sole surviving reading is wrong); any other loss is a STOP. Report
winner changes against the parent tip. Standard constraints apply.

## Landing record

STOP — no implementation landed. The required fence cannot currently be
expressed through the declared valence row: `checked by` arguments are limited
by the construction DSL parser to existing `role.feature` slots. A generated
verb-frame accessor is neither a feature nor an admissible callback argument,
so the fence cannot read the declared optional lexical atom without extending
that schema. Reinstating the existing preposition-class fence, naming the
preposition in a guard, or adding a per-construction guard would violate the
ticket's explicit fences. No coverage retirement, lock mutation, test change,
or grammar change was retained; the specified coverage and ambiguity gates
were therefore not run to completion.

`kata refresh` subsequently rewrote this stopped record onto the current
default line. Because the blocker leaves no candidate implementation to gate,
the measurements remain intentionally absent rather than stale.

- Assurance census: restored 0; re-spelled 0; ignored 0; added 0; removed 0.
- Deviations and additions: none.
- glossary gap: none.
