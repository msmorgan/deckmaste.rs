---
needs: [lean-v3-authoring-impl-14-ast-and-authoring-cleanup]
---
# Make `DamageKind` optional on `dealDamage` and drop `-Only` suffixes

In Oracle text, `"damage"` is unqualified by default: in an imperative
instruction (`"~ deals 3 damage to any target"`), any damage dealt as an effect
of a spell or ability (including `fight`, `[CR#701.14d]`) is noncombat damage by
rule (`[CR#120.2b]`), never written as `"noncombat damage"`, while in an event,
replacement, deontic, lookback, or static damage-rule pattern, unqualified
`"damage"` matches all damage and `"combat damage"` (`[CR#120.2a,510.2]`) or
`"noncombat damage"` (`[CR#120.2b]`) restricts it.

Today `DamageKind` (`Semantics/Events.lean:35`) has three constructors with
`-Only` suffixes (`.any | .combatOnly | .noncombatOnly`), `Action.dealDamage`
(`Semantics/Abilities.lean:1030`) takes a required `(kind : DamageKind)` with no
default, and ~155 imperative `dealDamage` sites across `Cards/*.lean`,
`Proofs/*.lean`, and `Check/Queries.lean` pass a trailing `.noncombatOnly` (plus
~45 raw `.dealDamage` sites in `Proofs/*.lean` and `Definitions/Actions.lean`
passing `.any`).

## Design

- Redefine `DamageKind` in `Semantics/Events.lean:35` as the explicit qualifier
  only, dropping `.any` and the `-Only` suffixes:
  ```lean
  inductive DamageKind where
    | combat | noncombat
    deriving DecidableEq, Repr
  ```
- Change `kind` on `Action.dealDamage`, `DamageResult`, `StaticSpec.damageRule`
  and `StaticSpec.preventionBan` (`Semantics/Abilities.lean`), plus their
  templates (`dealDamage`, `preventAll` / `preventNext` / `preventAllBy`,
  `damageRule` and `preventionBan` in `Semantics/Templates.lean`), to
  `(kind : Option DamageKind := none)`:
  - `kind = none` (default): unqualified `"damage"` (effect damage in an
    imperative per `[CR#120.2b]`; all damage in an event, replacement, deontic,
    lookback, or static damage rule).
  - `kind = some .combat`: `"combat damage"`.
  - `kind = some .noncombat`: `"noncombat damage"`.
- Drop the trailing `.noncombatOnly` (and raw `.any`) arguments from all
  imperative `dealDamage` / `.dealDamage` call sites across
  `Semantics/Cards/*.lean`, `Semantics/Definitions/`,
  `Semantics/Check/Queries.lean`, and `Semantics/Proofs/*.lean`, drop `.any`
  from event and rule sites, and rewrite `.combatOnly` → `some .combat` and
  `.noncombatOnly` → `some .noncombat` on event, replacement, deontic,
  lookback, and static damage-rule sites (e.g., *Chandra's Spitfire* in
  `Cards/Keyword.lean`, *Solphim, Mayhem Dominus*'s would-pattern
  `solphimPattern` in `Proofs/CorpusRecency.lean`, and combat-damage
  triggers).
- Remove `instance : AnyValue DamageKind`: no damage-kind value means "any".

## Owner rulings

- **Spelling of a qualified kind.** Authors write `some .combat` and
  `some .noncombat`, matching the `some .mainPhase` convention. No
  `Coe DamageKind (Option DamageKind)` instance: Lean resolves a leading-dot
  name against the expected type's namespace (`Option.combat`) before any
  coercion applies, so the instance cannot make `.combat` pass bare, and core's
  `α → Option α` coercion already covers qualified names.
- **Parameter order.** `kind : Option DamageKind := none` comes after the
  required parameters of `preventAll`, `preventNext`, `preventAllBy`,
  `damageRule` and `preventionBan`, constructors and templates alike, so sites
  without a kind omit it: Lean never skips a defaulted parameter that a later
  positional argument follows.
