---
needs: []
design: true
---
**Collapse `MayPay` into `May(Pay(cost))` by making `Pay` a first-class player
action.** From the 2026-07-19 `parse-may-pay-if-you-do` work: `May { effect,
if_did, if_not }` and `MayPay { actor, cost, and_then, or_else }` are the SAME
shape — an optional action plus an "if you do"/"if you don't" branch pair. The
only difference is the first slot's type: `May.effect` is an `OneShotEffect`,
`MayPay.cost` is a `Cost`. If paying is itself an action, `MayPay` stops being a
core node.

## Proposal

Add `Pay(cost: Cost, by: Reference)` as a player action (an `OneShotEffect` /
`PlayerAction`), then:

- `MayPay { actor, cost, and_then, or_else }` becomes
  `May { effect: Pay(cost, by: actor), if_did: and_then, if_not: or_else }`.
  The mapping is exact — `May`'s "did the optional thing happen?" IS "did the
  payment succeed / was it declined?", and `actor` moves into `Pay`, where a
  payment's agent belongs. `MayPay` demotes to a render macro over `May(Pay(…))`,
  exactly as `Unless` is already sugar over `MustPay` (`core-may-pay-must-pay`,
  done/).
- `MustPay { actor, cost, or_else }` (the "do X unless they pay" punisher) is
  the one to think hardest about — mandatory-with-punishment, not optional, so
  not a `May`; it wants to be `Pay` under an `If`/`Unless`. Decide whether that
  is cleaner or whether `MustPay` stays.
- `AdditionalCost { pay, body }` ([CR#601.2f]) gets closer too — the payment
  composes through the normal action/event machinery instead of a bespoke node.

## Why (and the caveats to weigh in design)

More aligned with the repo's minimal-primitives principle (one `Pay` action
reused everywhere, vs a bespoke `MayPay`/`MustPay` pair). Caveats:

- Making `Pay` an action does NOT erase the cost-payment subsystem (partial
  payment, mana-ability interleaving, [CR#601.2b]); it relocates it behind the
  `Pay` action boundary. Design must say how `Pay`'s success/failure event is
  observed so `if_did`/`if_not` branch on it.
- Idris soundness gate: `Pay : Cost -> PlayerAction` must be sound in
  `idris/src/Core.idr`; paying reads as a genuine action (not an intrinsic to
  data-fy), so this looks legitimate, but confirm.
- `"you may pay {2}"` with no consequence is useless but not meaningless — the
  node can represent it; nothing forces `and_then` the way `MayPay` did.

## Payoff / linkage

The parser side (`parse-may-pay-if-you-do`, integrated) already folds these
clauses; it emits `MayPay(…)` today and retargets to `May(Pay(…))` in minutes
once the core node exists — only the emitted string changes, the fold logic
(offer + greedy branch split + target/energy/negative-tail rules) is unchanged.
The `MayPay` render arm (`crates/deckmaste_cards/src/render/effect.rs`) moves to
a `Pay` fragment inside the `May` arm.

**Needs a design pass first** (what `PlayerAction` is here; how `Pay` success/
failure feeds `if_did`/`if_not`; the fate of `MustPay`/`Unless`/`AdditionalCost`)
before implementation. Standard constraints apply (idris soundness, render
parity, CR citations, wizards regen). Related: `core-may-pay-must-pay` (done/),
`engine-alt-costs`, `core-alt-costs`.

## Frames-side evidence (macro-frames, 2026-07-31)

The macro-frames effort independently supports this direction. If `Pay` is
a `PlayerAction`, effect-position payment gets the same subject-attachment
story (`By(subject, Pay(cost))`) as every other player verb, so the frame
lexicon composes its Cost-kind entries into sentence position through one
uniform wrapper — and `MayPay`, a node the lexicon would otherwise need
bespoke frames for, disappears. Round 1 already logged `By`'s
embed-default as concrete friction (a recovery spelling `GainLife(You, N)`
does not parse; see the round-2 plan's entry-`body:` mechanism), and the
round-2 residual census will quantify PlayerAction surface friction
corpus-wide. Sequencing: land this reshape before the frames coverage
rounds author the many PlayerAction catalog entries, so they are written
once against the new shape.
