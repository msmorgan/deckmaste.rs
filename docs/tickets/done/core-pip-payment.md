---
needs: []
---
**Core: per-pip alternative payment (`PayPips`) for convoke / delve / improvise —
payment of individual pips, NOT cost *reduction*.** Design question resolved
(from the 2026-06-28 idris↔rust model audit); promoted out of `maybe/`.

## Why payment, not reduction (rules-correctness)

The rules text is alternative *payment* of pips that remain in the cost:

- Convoke [CR#702.51a]: "For each colored mana in this spell's total cost, you
  may tap an untapped creature of that color you control *rather than pay that
  mana*." Delve [CR#702.66a]: "…you may exile a card from your graveyard *rather
  than pay that mana*." (Improvise is the same family — tap an untapped artifact
  rather than pay a generic pip.)
- They are explicitly **not** cost modifiers: [CR#702.51b] "isn't an additional
  or alternative cost and applies only after the total cost … is determined."
- The timeline forbids the reduction model: reductions live in cost
  *determination*, after which the total is locked in [CR#601.2f]; convoke/delve
  apply *after* lock-in.

Modeling these as `CostChange` reducers is observably wrong: it would lower the
mana value [CR#202.3] (breaking "mana value N" / "mana spent" reads — convoked
creatures are tracked as having *paid* that mana, and paying a changed cost still
"counts as paying the original cost" [CR#118.7]), and it mis-orders against
cost-floor effects (Trinisphere) and the locked-in total.

## Active window (scoping)

The granted static *exists* while the spell is on the stack [CR#702.51a], but the
alternative-payment option is only *exercisable* in the mana-assembly window of
**this** casting [CR#601.2g] — after the total cost is locked [CR#601.2f] and
before payment settles [CR#601.2h]. It is NOT usable while the cast spell waits to
resolve, nor during opponents' responses. Encode the window **structurally**:
`PayPips` is read by exactly one engine step — the cost-assembly handler — and by
nothing else, so there is no stack-lifetime "is it active?" predicate to get
wrong.

## Shape

Port Idris `StaticEffect.PayPips PipClass PayAct` (`idris/src/Core.idr`):

- `PipClass = Generic | Colored(Color)` — which pips of the locked-in cost this
  may pay.
- `PayAct = TapToPay(Filter) | ExileToPay(Filter)` — convoke = `TapToPay` a
  creature (matching color, or any for a generic pip); improvise = `TapToPay` an
  artifact (generic); delve = `ExileToPay` a graveyard card (generic). The
  `Filter` is open (plugin-safe).
- Borne by a `StaticAbility`, **not** a `CostChange`.

Engine: the cost-assembly step [CR#601.2g] walks the locked-in cost's pips and
offers each eligible pip its `PayAct` alternative; pips paid this way are not paid
with mana at [CR#601.2h]. The total cost and mana value are never mutated.
Composes with `core-aggregate-stat-cost`'s `TapTotal` (same payment layer).

## Reconciliation with `engine-cost-modification`

Distinct mechanism. `engine-cost-modification` (planned/) acts at the earlier
cost-*determination* step [CR#601.2f] for genuine reducers/increasers (affinity,
"costs {N} less", taxers); `PayPips` acts at the later cost-*payment* window
[CR#601.2g]. Neither subsumes the other — that ticket's scope is corrected to
exclude convoke/delve/improvise (see its note).

## Citation caveat

This repo's `data/rules/cr.json` snapshot keeps convoke/delve only in 702.x; its
[CR#601.2g] text names the mana-abilities window but does not yet name
convoke/improvise. The behavior and the window are settled; verify the precise
sub-rule wording against the live CR before relying on it in code.

Verdict: **improvement / rules-correctness**. Effort: **M–L** — a payment-time
pip iterator plus the per-pip decision (which permanent to tap / card to exile).
