import Semantics.Rules
import Semantics.Check.Card

/-!
# Semantics.Check.Rules

The obligations of the rules-as-data tables. Each table reuses the checker the same syntax
obeys inside a card — `Predicate`, `Condition`, `Instruction`, `Ability`, `TokenSpec` — and
adds only what the table's scope imposes: which kind the scope may bind, and what the rules
may do to everything it binds.

The scope is what a printed face supplies to a card: it binds `this` for the row's condition
and effect, so both are checked in the empty binding context, exactly as a card's own text is.
-/

namespace Semantics

/-- The kind a table row's scope binds, defaulting to an object. -/
def ruleScopeKind (p : Predicate) : Kind := p.kindOr .object

/-- A table row's scope binds objects or players and nothing else. State-based actions read
conditions on both — a creature's toughness [CR#704.5f], a player's life total [CR#704.5a] —
and while an ability is normally a characteristic of an object, a player can be granted one
[CR#113.1a,113.1b], so a conferral takes the same bound. -/
def ruleScopeBound : Kind := .join .object .player

/-- A state-based action doesn't use the stack [CR#704.1], and a target is chosen as a spell or
ability is put on the stack [CR#601.2c], so nothing a row does is targeted. -/
def SbaRule.check (r : SbaRule) : List Refusal :=
  let k := ruleScopeKind r.scope
  Predicate.check k [] r.scope ++ refuse (Kind.lte k ruleScopeBound) (.kindLte k ruleScopeBound) ++
    Condition.check [] r.when ++ Instruction.check [] r.then_ ++
    refuse (!anyTargetedAt (Instruction.intro [] r.then_)) .nontarget

/-- What a conferral gives is an ordinary ability, so it obeys the ability laws and must be one
an effect could grant at all: a spell ability is followed while a spell resolves [CR#113.3a] and
belongs to no permanent a rule could confer it onto. -/
def ConferralRule.check (r : ConferralRule) : List Refusal :=
  let k := ruleScopeKind r.scope
  Predicate.check k [] r.scope ++ refuse (Kind.lte k ruleScopeBound) (.kindLte k ruleScopeBound) ++
    Ability.check [] r.confer ++ refuse r.confer.grantable .grantable

/-- Damage reaches battles, creatures, planeswalkers and players and nothing else
[CR#120.1,120.1a], and the counters it removes must be counters that recipient can hold — the
counter registry declares the holder, so the two reads have to agree. -/
def DamageResultRule.check (r : DamageResultRule) : List Refusal :=
  let k := ruleScopeKind r.recipient
  Predicate.check k [] r.recipient ++
    refuse (damageableKind k r.recipient.seedTys) .damageRecipient ++
    r.remove.check ++ refuse (counterKindNamed k (some r.remove)) (.counterKindNamed k)

/-- A catalog entry obeys the token laws its characteristics would obey inside a `create`
[CR#111.3], and is named, because [CR#111.10] is a table effects index by name. -/
def PredefinedToken.check (t : PredefinedToken) : List Refusal :=
  refuse (!t.name.isEmpty) .tokenNamed ++ TokenSpec.check [] (.written t.token)

/-- Two entries under one name would let one shadow the other, as a duplicate label would in
any of the checker's tables. -/
def distinctPredefinedTokens : List PredefinedToken → Bool
  | [] => true
  | t :: ts => !(ts.map (·.name)).elem t.name && distinctPredefinedTokens ts

/-! ## Registry definitions -/

/-- What a definition confers obeys the laws its syntax obeys inside a card, in the empty
binding context: the definition supplies no binding, exactly as a rules table's scope supplies
only `this`. A conferred ability must be one an effect could grant at all [CR#113.3a], and
neither a state-based action [CR#704.1] nor a turn-based action [CR#703.1] uses the stack, so
neither can target [CR#601.2c]. -/
def Conferral.check : Conferral → List Refusal
  | .ability a => Ability.check [] a ++ refuse a.grantable .grantable
  | .property spec => StaticSpec.check [] spec
  | .stateBased when then_ =>
      Condition.check [] when ++ Instruction.check [] then_ ++
        refuse (!anyTargetedAt (Instruction.intro [] then_)) .nontarget
  | .turnBased _ then_ =>
      Instruction.check [] then_ ++
        refuse (!anyTargetedAt (Instruction.intro [] then_)) .nontarget

/-- A designation's zone, card type and room half say what the object holding it is, so a
designation a player, a card or the game holds [CR#701.15b,725.1,731.1,903.3] writes none of
the three. -/
def DesignationScope.narrowable : DesignationScope → Bool
  | .heldBy .object => true
  | _ => false

/-- A counter named by a label declares one [CR#122.1]; a +X/+Y counter [CR#122.1a] and a
keyword counter [CR#122.1b] are named by their own `CounterKind` constructor instead. -/
def CounterKind.declaresName : CounterKind → Bool
  | .named label => !label.isEmpty
  | _ => true

/-- The obligations of one registry definition. A counter is placed on an object or a player
[CR#122.1]; a subtype [CR#205.3] and a designation [CR#701.15b] are looked up by the name they
declare, so a nameless one is unreachable; and what either confers obeys the ability laws. -/
def Definition.check : Definition → List Refusal
  | .counter kind holder confers =>
      refuse (holder == .object || holder == .player) (.definitionHolder holder) ++
        refuse kind.declaresName .definitionNamed ++ confers.flatMap Conferral.check
  | .subtype sub rules =>
      refuse (!sub.label.isEmpty) .definitionNamed ++ rules.flatMap Conferral.check
  | .designation label scope _ zone type half =>
      refuse (!label.isEmpty) .definitionNamed ++
        refuse (scope.narrowable || (zone.isNone && type.isNone && half.isNone))
          (.definitionScoped label)

/-- Every refusal in a plugin's `rules/` directory, in reading order. -/
def RulesTables.check (t : RulesTables) : List Refusal :=
  t.sba.flatMap SbaRule.check ++ t.conferral.flatMap ConferralRule.check ++
    t.damageResult.flatMap DamageResultRule.check

end Semantics
