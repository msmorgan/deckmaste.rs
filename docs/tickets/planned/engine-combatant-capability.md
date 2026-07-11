---
needs: [engine-composite-type-registry]
---
Turn "creature" into a conferred combat capability rather than a literal `Type::Creature`
gate — the "Combatant abstraction". Once card types are data-with-`confers`, the `Creature`
type confers the capabilities that make a permanent a combatant, and engines gate on those
instead of `card_types.contains(&Type::Creature)` at the five current sites: attack
eligibility and blocker eligibility (`legal.rs`), combat-damage marking (`step.rs`), and the
summoning-sickness `{T}` gate (duplicated at three sites — `legal.rs`, `activate.rs`,
`cast.rs` autotap).

**The grant.** The `Creature` type confers `May(Attack(This))` + `May(Block(This))` (and
combat damage is marked only on combatants, [CR#120.3d,120.3e]). This is the same
default-deny/grant flip as attachment: combat already consults deontic restriction rows
(`Cant(Attack)` at attack eligibility, menace `Block` rows), so only the base "can
attack/block" is still a literal type check — Combatant makes it a conferred `May`. Vehicles
and man-lands then confer the grant while animated, with no type-change needed.

**Summoning sickness = a scoped Cant-pair, not a boolean read.** [CR#302.6] restricts exactly
two things for a not-continuously-controlled creature — it can't attack, and it can't activate
an ability whose cost includes `{T}`/`{Q}` — and never blocking. Model it as a conferral
scoped `summoning_sick && Not(Has(Haste))` that confers the pair: `Cant(Attack(This))` and
`Cant(Activate(what: This, where: <ability cost includes {T}/{Q}>))`. The activate half MUST
be cost-scoped — a sick creature freely activates non-tap abilities (`{2}: gain 1 life`); a
blanket `Cant(Activate)` is wrong. It is a *pair*, not a triple, precisely because blocking
stays granted. This collapses the `!summoning_sick` literal in attack eligibility and the
3×-duplicated `summoning_sick && Creature` tap-gate into the deontic collectors the engine
already consults for `Cant(Attack)`.

**Haste demotes from composite to marker (like Reach).** Under this model haste confers
nothing; it is purely the exemption tag the sickness conferral's scope references
(`Not(Has(Haste))`) — [CR#602.5a], [CR#702.10b,702.10c] ("ignore this rule for creatures with
haste"). Structurally identical to Reach, a marker whose meaning lives entirely in flying's
block predicate ([CR#702.9b]). One fewer special-cased composite keyword.

**State stays.** `obj.summoning_sick` remains the tracked state (continuous control since the
controller's most recent turn began); only its *consumption* moves — from four ad-hoc literal
reads to the single conferral condition.

**One new primitive.** An `Activate` deontic action carrying a cost/ability predicate
(`Attack`/`Block`/`Attach`/`Cast` deontics exist; `Activate` does not yet) — the only addition
the tap-half of the pair needs.

Depends on the composite type registry substrate landing first. This is design work, not a
mechanical migration.
