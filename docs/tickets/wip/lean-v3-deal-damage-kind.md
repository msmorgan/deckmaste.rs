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
- Change `kind` on `Action.dealDamage` (`Semantics/Abilities.lean:1030`),
  `DamageResult` (`Semantics/Abilities.lean:816-821`), `StaticSpec.damageRule`
  (`Semantics/Abilities.lean:882-883`), and `StaticSpec.preventionBan`
  (`Semantics/Abilities.lean:884`) — plus their templates (`dealDamage` in
  `Semantics/Templates.lean:1070-1072`, `preventAll` / `preventNext` /
  `preventAllBy` in `Semantics/Templates.lean:1384-1396`, `damageRule` in
  `Semantics/Templates.lean:1924-1927`, and `preventionBan` in
  `Semantics/Templates.lean:1980-1981`) — to
  `(kind : Option DamageKind := none)`, and add
  `instance [AuthoringContext] : Coe DamageKind (Option DamageKind) := ⟨some⟩`
  so `.combat` and `.noncombat` pass bare without `some`:
  - `kind = none` (default): unqualified `"damage"` (effect damage in an
    imperative per `[CR#120.2b]`; all damage in an event, replacement, deontic,
    lookback, or static damage rule).
  - `kind = .combat`: `"combat damage"`.
  - `kind = .noncombat`: `"noncombat damage"`.
- Drop the trailing `.noncombatOnly` (and raw `.any`) arguments from all
  imperative `dealDamage` / `.dealDamage` call sites across
  `Semantics/Cards/*.lean`, `Semantics/Definitions/`,
  `Semantics/Check/Queries.lean`, and `Semantics/Proofs/*.lean`, and rename
  `.combatOnly` → `.combat` and `.noncombatOnly` → `.noncombat` on event,
  replacement, deontic, lookback, and static damage-rule sites (e.g.,
  *Solphim, Mayhem Dominus* in `Cards/Keyword.lean:484` and combat-damage
  triggers).
