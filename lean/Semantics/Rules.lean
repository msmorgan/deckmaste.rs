import Semantics.Card

/-!
# Semantics.Rules

The rules-as-data tables a plugin authors beside its cards: state-based actions, conferrals (the
results of damage among them), and the predefined-token catalog. A card says what one object
does; these say what the *rules* do to every object that matches a scope, so they are the same
syntax read against a scope predicate rather than against a printed face.

Nothing here is a new grammar. A conferral confers an ordinary `Ability` or `StaticSpec`; a
state-based action performs an ordinary `Action` under an ordinary `Condition`; a predefined
token is the `CharacteristicBundle` a `TokenSpec.written` already carries. The tables add the
scope and the naming, and `Semantics.Check.Rules` adds the obligations that scope imposes.
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
  then_ : Action

/-- One entry of the predefined-token catalog [CR#111.10], authored under `tokens/`: the name
an effect creating the token writes ("create a Treasure token") and the characteristics the
rules define it with. The characteristics are a `CharacteristicBundle` — the same value
`TokenSpec.written` carries — because a predefined token differs from a written one only in
being named in advance; the effect that creates it may still modify or add to them
[CR#111.10]. -/
structure PredefinedToken where
  name : String
  token : CharacteristicBundle

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
  /-- A continuous property with no ability of the definition's own behind it: a quality the
  rules state of an object [CR#113.12], as an Equipment's host rule is [CR#301.5], or a keyword
  counter's grant. A keyword counter is no ability, but its bearer gains the keyword
  [CR#122.1b] in layer 6 [CR#613.1f]; a gained ability can be removed by another effect
  [CR#113.12], so an ability-removing effect removes it in timestamp order [CR#613.7,613.7c]. -/
  | property (spec : StaticSpec)
  /-- A state-based action the definition puts on every object that holds its subject
  [CR#704.1]: once per holder per check [CR#704.3], however many counters of the kind it holds,
  and free to read counts (ruling 2026-10-10). -/
  | stateBased (when : Condition) (then_ : Action)
  /-- A turn-based action the definition puts on every object it names [CR#703.1]. -/
  | turnBased (part : TurnPart) (then_ : Action)

/-- A rules-defined conferral, authored under `rules/grant/`: *every object or player matching
`scope` has `confer`*. There is no `when`, because a conferral carries its own conditionality —
[CR#306.5b]'s planeswalker loyalty is a replacement effect [CR#614.1c] whose event is its
gate. Intrinsic abilities of a type or subtype [CR#305.6] are the other rows this shape takes,
as an `ability` conferral. A rule the game applies to everything in scope, which no effect
removing abilities can switch off, is a `property` conferral: the results of damage are
[CR#120.3a,120.3c,120.3e,120.3g,120.3h]. In a row scoped to players, "you" is the player. -/
structure ConferralRule where
  scope : Predicate
  confer : Conferral

/-- What one registry declaration MEANS: the rules content the declared name stands for. A
registry declares a namespace's members [CR#122.1,205.3,701.15b]; this is one member's
definition, where a `RulesTables` row is scoped by a predicate and names nothing.

`Semantics.Check.Facts` is generated from these: every counter definition is a `CounterFacts`
row labelled with its kind's name, a subtype definition is a `SubtypeFacts` row, a designation
definition is a `DesignationFacts` row. A card type definition is no facts row: the checker's
card-type tables (`CardType.permanent`) do not read it yet. -/
inductive Definition where
  /-- A counter [CR#122.1]: its kind, what holds it, and what it does to its bearer.

  The kind is the declaration's own name; the registry supplies it from the name the declaration
  carries, so the author never writes it and nothing in the body refers to the counter
  being defined. An `ability` or `property` conferral states what ONE counter of the kind
  confers, and the model applies it once per counter held: a +1/+1 counter confers "+1/+1"
  [CR#122.1a,613.4c], so N of them give +N/+N, and a keyword counter confers its keyword
  [CR#122.1b,613.1f], so N of them grant it N times [CR#122.1b,113.2c]; that is having it once
  only where the keyword's own rule makes instances redundant (flying [CR#702.9c]), while each
  exalted or decayed instance has its own triggered ability [CR#702.83a,702.147a] and triggers
  separately [CR#113.2c], so a consumer counts instances per keyword. Such a conferral reads no
  count of its own kind. A `stateBased` conferral is
  scoped to the permanent that holds the counters, applies once per holder per check [CR#704.3],
  and may read counts, its own kind's included: the +1/+1 and −1/−1 annihilation rule on the
  −1/−1 counter [CR#704.5q,122.3] (ruling 2026-10-10). The reading is the model's; the checker
  checks each conferral once, in the empty binding context (`Definition.check`), and computes no
  multiplicity. A counter whose rule makes one or more of it create a single effect — a shield,
  stun or finality counter [CR#122.1c,122.1d,122.1h] — writes no conferral today; giving it one
  needs a reading this one is not. -/
  | counter (kind : CounterKind) (holder : Kind) (confers : List Conferral)
  /-- A subtype [CR#205.3] and the rules its type rule defines for it, empty for the subtypes
  that are inert vocabulary. -/
  | subtype (subtype : Subtype) (rules : List Conferral)
  /-- A designation [CR#701.15b]: a named marker rules and effects identify, which is not an
  ability and confers nothing, so the node carries columns rather than conferrals. The label is
  the declaration's own name, or a member's where the declaration lists several (`day` and
  `night` [CR#731.1]); the registry supplies it, never the author. `effectful`
  is whether an action may confer it directly; `zone`, `type` and `half` narrow the object
  that holds it. -/
  | designation (label : DesignationLabel) (scope : DesignationScope) (effectful : Bool)
      (zone : Option Zone) (type : Option CardType) (half : Option RoomHalf)
  /-- A card type [CR#205.2a]: the type, whether it is a permanent type [CR#110.4], and what it
  confers on every object of that type. A quality the type gives its objects — a creature is a
  Combatant [CR#508.1a], a land card may be played [CR#305.1], an instant may be cast whenever
  its owner has priority [CR#304.1] — is no ability of the object [CR#113.12], so it is the
  ability-free `property` flavor. Most card types confer nothing. -/
  | cardType (cardType : CardType) (permanentType : Bool) (confers : List Conferral)
  /-- A Relation [CR#702.55b,702.95b,702.99b]: a named link between two entities, made by one
  ability and read by others, with the kind of each side. The label is the declaration's own
  name; the registry supplies it, never the author. -/
  | relation (label : RelationLabel) (left : Kind) (right : Kind)

/-! A registry declaration's name means its TERM everywhere, its own definition included: a
counter declaration's name is its `CounterKind` (`.named "m1m1Counter"`), a subtype's its
`Subtype`, a designation's or relation's its label. The `Definition` above is a separate record
the registry holds under the name, which `cargo xtask facts` and `cargo xtask lean definitions`
read; the name never expands to it (ruling 2026-10-10, retiring the `Definition.*Term`
projections that read the term back out of the record). -/

/-- The two tables a plugin's `rules/` directory defines, concatenated across its files. The
predefined-token catalog is a separate list, because its entries are named and a plugin writes
them under `tokens/` rather than `rules/`. -/
structure RulesTables where
  sba : List SbaRule := []
  conferral : List ConferralRule := []

end Semantics
