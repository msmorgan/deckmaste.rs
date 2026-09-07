import Semantics.Card

/-!
# Semantics.Rules

The rules-as-data tables a plugin authors beside its cards: state-based actions, conferrals,
damage results, and the predefined-token catalog. A card says what one object does; these say
what the *rules* do to every object that matches a scope, so they are the same syntax read
against a scope predicate rather than against a printed face.

Nothing here is a new grammar. A conferral confers an ordinary `Ability`; a state-based
action performs an ordinary `Instruction` under an ordinary `Condition`; a predefined token is
the `CharacteristicBundle` a `TokenSpec.written` already carries. The tables add the scope and
the naming, and `Semantics.Check.Rules` adds the obligations that scope imposes.
-/

namespace Semantics

/-- A rules-defined state-based action [CR#704.1], authored under `rules/sba/`: *for every
object or player matching `scope`, with `this` bound to it, if `when` holds the game performs
`then_`*. The sweep [CR#704.3] evaluates the whole table. Rows that would need a player to
choose stay imperative in the engine (`docs/decisions/state-based-actions-are-data.md`); the
grammar has no complete read for a choice, so that is a convention here and not a checked
obligation. -/
structure SbaRule where
  scope : Predicate
  when : Condition
  then_ : Instruction
  deriving Repr, BEq

/-- A rules-defined conferral, authored under `rules/grant/`: *every object matching `scope`
has `confer`*. There is no `when`, because a conferred ability carries its own conditionality —
[CR#306.5b]'s planeswalker loyalty is a replacement effect [CR#614.1c] whose event is its
gate. Intrinsic abilities of a type or subtype [CR#305.6] are the other rows this shape
takes. -/
structure ConferralRule where
  scope : Predicate
  confer : Ability
  deriving Repr, BEq

/-- A rules-defined damage result, authored under `rules/damage/`: *damage dealt to a
permanent matching `recipient` removes that many `remove` counters from it* — [CR#120.3c] for
a planeswalker's loyalty, [CR#120.3h] for a battle's defense. The count is always the damage
event's own amount ("that many"), so there is no amount to write, and the result is additive
with the others damage has [CR#120.3]. -/
structure DamageResultRule where
  recipient : Predicate
  remove : CounterKind
  deriving Repr, BEq

/-- One entry of the predefined-token catalog [CR#111.10], authored under `tokens/`: the name
an effect creating the token writes ("create a Treasure token") and the characteristics the
rules define it with. The characteristics are a `CharacteristicBundle` — the same value
`TokenSpec.written` carries — because a predefined token differs from a written one only in
being named in advance; the effect that creates it may still modify or add to them
[CR#111.10]. -/
structure PredefinedToken where
  name : String
  token : CharacteristicBundle
  deriving Repr, BEq

/-- The three tables a plugin's `rules/` directory defines, concatenated across its files. The
predefined-token catalog is a separate list, because its entries are named and a plugin writes
them under `tokens/` rather than `rules/`. -/
structure RulesTables where
  sba : List SbaRule := []
  conferral : List ConferralRule := []
  damageResult : List DamageResultRule := []
  deriving Repr, BEq

end Semantics
