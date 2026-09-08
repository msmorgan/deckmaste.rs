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

/-- What a registry definition supplies to everything it names [CR#113.12,305.6]: an intrinsic
ability, an ability-free continuous property, a state-based action, or a turn-based action.
The scope is the definition itself — the counter's bearer, the subtype's object — so a row
here writes no predicate, where a `ConferralRule` under `rules/grant/` must.

The four flavors are the four things the rules do with a definition, and the CR keeps them
apart: an intrinsic ability is an ordinary ability of the object, as a basic land type's mana
ability is [CR#305.6]; a rule that only states a quality of an object grants no ability and
sets no characteristic [CR#113.12]; a
state-based action happens automatically and doesn't use the stack [CR#704.1]; a turn-based
action likewise happens automatically as a step or phase begins [CR#703.1]. -/
inductive Conferral where
  /-- An ability the definition gives the object, in the ordinary ability hierarchy — a basic
  land type's mana ability is one [CR#305.6]. -/
  | ability (confer : Ability)
  /-- A continuous property with no ability behind it [CR#113.12] — an Equipment's host rule
  [CR#301.5], a keyword counter's grant [CR#122.1b]. -/
  | property (spec : StaticSpec)
  /-- A state-based action the definition puts on every object it names [CR#704.1]. -/
  | stateBased (when : Condition) (then_ : Instruction)
  /-- A turn-based action the definition puts on every object it names [CR#703.1]. -/
  | turnBased (part : TurnPart) (then_ : Instruction)
  deriving Repr, BEq

/-- What one registry declaration MEANS: the rules content the declared name stands for. A
registry declares a namespace's members [CR#122.1,205.3,701.15b]; this is one member's
definition, where a `RulesTables` row is scoped by a predicate and names nothing.

`Semantics.Check.Facts` is generated from these: a counter definition's `.named` kind is a
`CounterFacts` row, a subtype definition is a `SubtypeFacts` row, a designation definition is
a `DesignationFacts` row. A definition whose kind is a `CounterKind` of its own — a +X/+Y
counter [CR#122.1a] or a keyword counter [CR#122.1b] — contributes no row to the table of
named counters, because it is not looked up by label. -/
inductive Definition where
  /-- A counter [CR#122.1]: what it is, what holds it, and what it does to its bearer. -/
  | counter (kind : CounterKind) (holder : Kind) (confers : List Conferral)
  /-- A subtype [CR#205.3] and the rules its type rule defines for it, empty for the subtypes
  that are inert vocabulary. -/
  | subtype (subtype : Subtype) (rules : List Conferral)
  /-- A designation [CR#701.15b]: a named marker rules and effects identify, which is not an
  ability and confers nothing, so the node carries columns rather than conferrals. `effectful`
  is whether an instruction may confer it directly; `zone`, `type` and `half` narrow the object
  that holds it. -/
  | designation (label : DesignationLabel) (scope : DesignationScope) (effectful : Bool)
      (zone : Option Zone) (type : Option CardType) (half : Option RoomHalf)
  deriving Repr, BEq

/-- A definition's name DENOTES its term.

A registry declaration's name is a macro whose body is the `Definition` above, so the name
stands at two kinds of position: where a definition is wanted, it is the node; where the
declared TERM is wanted — a card's subtype list, a counter reference, a designation — the
position takes the term the definition names, not the node that defines it. Each flavor names
one term: `.subtype` names its `subtype`, `.counter` its `kind`, `.designation` its `label`.

Only the subtype projection is defined, because the subtype registry is the one whose names a
card writes today (`plugins-v2-subtypes-macro-only`); the counter and designation projections
follow when their registries become macro-only. -/
def Definition.subtypeTerm : Definition → Option Subtype
  | .subtype term _rules => some term
  | .counter .. | .designation .. => none

/-- The three tables a plugin's `rules/` directory defines, concatenated across its files. The
predefined-token catalog is a separate list, because its entries are named and a plugin writes
them under `tokens/` rather than `rules/`. -/
structure RulesTables where
  sba : List SbaRule := []
  conferral : List ConferralRule := []
  damageResult : List DamageResultRule := []
  deriving Repr, BEq

end Semantics
