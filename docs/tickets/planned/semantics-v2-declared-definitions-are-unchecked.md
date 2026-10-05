---
needs: []
---
**No gate checks a Registry Definition, and two counter rules do not fit the
per-counter reading.** Two findings routed by the landing record of
`semantics-v2-counter-kind-is-a-name` (2026-10-05), recorded here so they do
not live only there. Standard constraints apply. Related:
`semantics-v2-counter-kind-is-a-name` (the landing that found both),
`semantics-v2-designation-is-a-name` (the designation definitions a gate
would also cover).

## 1. Declared definitions are never checked

`cargo xtask lean-check` emits and proves CARDS only. Nothing emits the
`Definition` node a registry declaration in `plugins_v2/builtin/macros/`
carries, so the Lean `Definition.check` (`lean/Semantics/Check/Rules.lean`)
runs only over the hand-written pins in `lean/Semantics/Proofs/Rules.lean`.

Evidence: the +1/+1 and -1/-1 counter definitions are refused today.
`Proofs/Rules.lean` pins `p1p1CounterDefinitionZone` and
`m1m1CounterDefinitionZone` assert `Definition.check … = [.zoneIs
.battlefield]`: a stat change's subject must be a permanent, and the
definition's subject is the counter's bearer. The retired count-multiplied
spelling was refused the same way, twice (`p1p1CounterCountMultipliedTwin`),
so the refusal predates the landing. [CR#122.1a] reads (rule numbers inside
the quotations are written in citation form):

> [CR#122.1a] A +X/+Y counter on a creature or on a creature card in a zone other
> than the battlefield, where X and Y are numbers, adds X to that object's
> power and Y to that object's toughness. Similarly, -X/-Y counters subtract
> from power and toughness. See [CR#613.4c].

Two pieces of work, separable:

- **(a) A gate.** Emit every Registry Definition as Lean and prove
  `Definition.check = []` for each, beside the card gate. Coverage, counted
  from `cargo xtask expansions` on the landing's tree: 554 definitions — 73
  counters, 462 subtypes, 19 designations (in 18 declaration files). How many
  are refused today is unmeasured beyond the two counter definitions above.
- **(b) The zone rule for a counter's conferral (open, for the owner).** A
  counter's conferral applies to its bearer wherever [CR#122.1a] says it
  does, which includes a creature card in a zone other than the battlefield,
  while `StaticSpec.check` requires a stat change's subject on the
  battlefield. Should a counter definition's conferral be checked with the
  bearer's zone left open, with the zones its rule names, or some other way?
  Not decided.

## 2. One-or-more counters do not fit the per-counter reading (open, for the owner)

The ruling of 2026-10-05 (`docs/decisions/semantics-v2.md` §7) says a counter
definition states what ONE counter confers and the model applies it once per
counter held. Three counter rules say instead that one or more counters
create a single effect:

> [CR#122.1c] One or more shield counters on a permanent create a single
> replacement effect and a single prevention effect that protect the
> permanent. These effects are "If this permanent would be destroyed as the
> result of an effect, instead remove a shield counter from it" and "If
> damage would be dealt to this permanent, prevent that damage and remove a
> shield counter from it." See [CR#614] "Replacement Effects" and [CR#615]
> "Prevention Effects."

> [CR#122.1d] One or more stun counters on a permanent create a single
> replacement effect that stops the permanent from untapping. That effect is
> "If a permanent with a stun counter on it would become untapped, instead
> remove a stun counter from it."

> [CR#122.1h] One or more finality counters on a permanent create a single
> replacement effect that stops the permanent from going to the graveyard.
> That effect is "If this permanent would be put into a graveyard from the
> battlefield, exile it instead."

[CR#122.1c,122.1d,122.1h]. `shieldCounter`, `stunCounter` and
`finalityCounter` confer nothing today (`confers` empty), so nothing is
wrong yet. Open question for the owner: when one of them needs a definition,
how is it written — a second reading of `confers` ("while one or more are
held"), a flag on the definition, or something else?
