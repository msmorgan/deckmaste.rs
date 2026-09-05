---
needs: []
---
**Make exchange one core row over an `Exchanged` sort.** Ruling 2026-09-04 on
exchange (cleanroom review 3, D-Q4).

- `Effect.Instruction.ExchangeLife parties` is a core row while exchange of
  control is `Macros.exchangeControlOfThis` — two simultaneous `GainsControl`s
  with an `ExchangeCtx` helper. Two grants cannot express the atomicity the
  rule states: when such a spell or ability resolves, if the entire exchange
  can't be completed, no part of it occurs [CR#701.12a]. The macro also cannot
  reach the zone and card exchanges of [CR#701.12d..701.12f].
- One row `Exchange : (what : Exchanged bs) -> Instruction bs`, with
  `Exchanged` covering life totals, control of two permanents, and the zone
  and card exchanges. `ExchangeLife` folds in and `twoPartiesOk` moves onto
  the life arm; `exchangeControlOfThis` and `ExchangeCtx` go.
- Decide, and record, which of the rule's riders the workbench states as an
  obligation and which is engine behaviour: when a card that is attached to an
  object is exchanged across zones, it stops being attached and the other card
  becomes attached in its place [CR#701.12e], and an exchange of two zones
  happens even when one of them is empty [CR#701.12f].
- Bench one printed card per `Exchanged` arm; pin an exchange whose halves are
  of different sorts.

Size: M. Done when: `Exchange` is the only exchange row, `ExchangeLife` and
`exchangeControlOfThis` are gone from the tree, each arm has a printed witness
and the pins probe non-vacuous; build at its module count. Standard
constraints apply, including the RON-shaped constraint.
