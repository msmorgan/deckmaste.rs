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

## As landed

- One core row `Exchange : (what : Exchanged bs) -> Instruction bs`
  (`Effect.idr`) over a new `Exchanged` sort with four arms: `LifeTotals`
  (carrying `twoPartiesOk` [CR#701.12c]), `ControlOf` [CR#701.12b],
  `CardsAcross` [CR#701.12d..701.12e] and `Zones` [CR#701.12f]. Helpers
  `exchangedIntro` / `exchangedDeed` / `exchangedCostOk` beside it; one
  `instrProfile` row and one `costActionOk` row replace `ExchangeLife`'s.
- `Instruction.ExchangeLife` deleted, folded into `Exchanged.LifeTotals`;
  `twoPartiesOk` moved onto that arm. `Macros.exchangeControlOfThis` and
  `Macros.ExchangeCtx` deleted; no exchange macro replaces them (all five
  bench sites write the core row, as the three `ExchangeLife` sites did).
- Riders decision: engine behaviour, no obligation — the attachment transfer
  [CR#701.12e], the empty-zone case [CR#701.12f], [CR#701.12d]'s same-owner
  restriction, and [CR#701.12b]'s same-controller no-op. All four are
  consequences of, or runtime state at, resolution, not conditions the
  sentence can state. Recorded as a two-line cite on `Exchanged`. The
  obligations the workbench does state are: two singular parties
  (`LifeTotals`), both halves controllable — battlefield or stack
  [CR#108.4] (`ControlOf`), the two cards in different zones
  [CR#701.12d] (`CardsAcross`), and two distinct card zones
  [CR#701.12d,701.12f] (`Zones`).
- Bench, one printed vintage-legal witness per arm: Soul Conduit
  (`Cards/Cost.idr soulConduitExchange`, `LifeTotals`, re-spelled), Mirror
  Universe (`Cards/Turn.idr mirrorUniverse`, `LifeTotals`, re-spelled),
  Avarice Totem (`Cards/Keyword.idr avariceTotemExchange`, `ControlOf`,
  re-spelled off the deleted macro), Arcanum Wings
  (`Cards/Keyword.idr arcanumWingsAuraSwap`, `CardsAcross`, new — aura swap
  as its [CR#702.65a] rules-text body), Harness Infinity
  (`Cards/Piles.idr harnessInfinityExchange`, `Zones`, new).
- Pins re-spelled in `ProofsTrigger.idr`: `okExchangeTwoParties` (twin),
  `badExchangeOneParty` (now carrying the atomicity cite
  [CR#701.12a,701.12c] — the row states both halves, so a half-exchange is
  unwritable rather than half-done), `badExchangePluralParty`. Pins added in
  `ProofsZone.idr` with their twins: `badExchangeControlInGraveyard`
  (twin `okExchangeControlOnField`), `badExchangeCardsSameZone` (twin
  `okExchangeCardsAcrossZones`), `badExchangeZoneWithItself` and
  `badExchangeBattlefieldZone` (twin `okExchangeTwoZones`).
- Undone by design: the two-numerical-values and text-box arms at the end of
  [CR#701.12] are not modelled — the ticket scopes `Exchanged` to life totals,
  control and [CR#701.12d..701.12f].

## Landing record

Numbers before → after: exchange core rows 2 (`ExchangeLife` row +
`exchangeControlOfThis` macro-of-two-grants) → 1 (`Exchange`) over a 4-arm
`Exchanged` sort; `Macros` exports −2 (`ExchangeCtx`,
`exchangeControlOfThis`), +0; module count 46 → 46.

Gate lines:

- `cd idris && ./scripts/build` → `46/46: Building Cards (src/Cards.idr)`;
  no `Error` and no `Warning` line.
- `cargo xtask cite check --list-noncompliant` →
  `0 non-compliant citation-looking string(s)`.
- `cargo xtask cite check` →
  `checked 14392 citations against cr.txt (eff. 2026-08-07); 0 stale`.
- `cargo xtask cite bless` → registered [CR#702.65a]; pruned one storm-count
  rule cited only from `docs/tickets/done/` (an excluded path), file kept.
- `jj --no-pager diff --git | cargo xtask cite audit --diff` →
  `audited 13 citation site(s)`; each rule read against its claim.

Assurance counts: restored 0; re-spelled 6 (3 bench witnesses, 3 pins/twins
in `ProofsTrigger.idr`); ignored 0; added 9 (2 bench witnesses, 3 twins,
4 pins); removed 0.

Pin probes (each mis-stated once, message watched): all six exchange pins
re-checked as ordinary definitions in a scratch module fail with `Can't find
an implementation for So False.`, and `badExchangeControlInGraveyard`
re-stated against the other handle (`{az = ok}`, the battlefield half) fails
to elaborate — so the pinned handle is the load-bearing one. Over-tightness
probed positively: `Zones graveyardZ libraryZ` (Morality Shift) and
`ControlOf` with a spell half (Perplexing Chimera, Sudden Substitution) both
typecheck.

Deviations and additions:

- `Exchanged` has four arms, not three: the ticket's "zone and card
  exchanges" splits into `CardsAcross` ([CR#701.12d..701.12e], named cards)
  and `Zones` ([CR#701.12f], whole zones), which the rule distinguishes.
- Arcanum Wings is benched as aura swap's [CR#702.65a] rules-text body (an
  activated ability with a `may`), not as a keyword label: the label alone
  writes no `Exchanged` term, so the `CardsAcross` arm would have had no
  witness. It is the only supported-corpus card that exchanges named cards
  across zones.
- Four pins where the ticket asked for one. The ticket's "halves of different
  sorts" (a life total against control, a card against a zone) is refused
  structurally — `Exchanged`'s arms are separately typed, so such a term is
  not writable and cannot be pinned. The pins added instead refuse a half of
  the wrong sort *within* an arm, which is what remains writable.
- No macro added; the bench writes the core row at all five sites.

STOP: none.
