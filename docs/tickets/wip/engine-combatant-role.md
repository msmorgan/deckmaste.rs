---
needs: [ability-conferral-without-innate]
---
**Replace the `May(Attack)` proxy with an explicit conferred Combatant role.**
The candidate [Game Model glossary](../../contexts/game-model/CONTEXT.md) defines Combatant as the
bundled creature-like role: power and toughness matter for relevant damage;
the Permanent is in the domains for attacking, blocking, and fighting; and the
continuous-control rule informally called summoning sickness applies to its
attacks and tap/untap-symbol abilities. Individual permissions and
restrictions determine whether it may perform a particular action; they do not
define or revoke the role.

The current implementation from `engine-combatant-capability` uses presence of
the Creature type's `May(Attack)` conferral as a witness for the whole bundle:
`is_combatant` delegates to `attackable`, while `May(Block)` and the
summoning-sickness `Cant` pair are separately conferred alongside it. That is a
working invariant for today's Creature definition, but it makes one permission
the accidental identity of the larger concept. A source that confers only
`May(Attack)` or only `May(Block)` would silently acquire or fail to acquire
unrelated damage, fight, and summoning-sickness semantics.

Represent Combatant as an ordinary conferred rules property in the type and
subtype registries after the generic `Innate` wrapper is removed. The Creature
Card Type normally confers that property while the Object is a Permanent.
Derive the role's ordinary attack and block permissions, its eligibility for
creature-only combat and fight rules, creature damage marking, and the
applicability of the summoning-sickness `Cant` pair from the role. Preserve
independent `May(Attack)` and `May(Block)` expressions for effects that grant
only an action permission; neither one alone confers Combatant.

Counterfactual treatment is scoped. If an effect says a non-Combatant Permanent
may block as though it were a 1/1 creature, treat it as a 1/1 Combatant only
while determining and performing that block ([CR#609.4]). It does not become a
Combatant for unrelated purposes: its tap ability is not summoning-sick, it is
not generally a legal fight participant, and damage outside that operation is
not given creature damage semantics merely because it received the blocking
permission.

Acceptance cases:

- A Creature restricted from attacking, blocking, or both remains a Combatant.
- A Combatant with defender remains a Combatant even though it cannot attack.
- Gaining or losing the Creature type gains or loses the role through layered
  registry conferral, unless another current source confers it.
- A fresh noncreature with `{T}: Draw a card` and only the scoped permission to
  block as though it were a 1/1 creature may activate that ability; the scoped
  block treatment does not globally enable summoning sickness.
- Attack, block, fight, damage, and tap/untap-activation tests read the explicit
  role rather than using either individual `May` row as its proxy.

This is a correction to the abstraction boundary established by
`engine-combatant-capability`, not a return to literal `Type::Creature` checks.
The registry remains the source of truth, so custom Card Types may confer the
complete Combatant role without adding hard-coded engine branches.
