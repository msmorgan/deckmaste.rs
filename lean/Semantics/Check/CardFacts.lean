import Semantics.Abilities
import Semantics.Check.Keywords

namespace Semantics

def Action.scopeBody (i : Action) (fuel : Nat := 64) : Action :=
  match fuel, i with
  | n + 1, .inCaller b | n + 1, .ownScope _ b | n + 1, .bracketed b => b.scopeBody n
  | _, b => b
termination_by structural fuel

def StaticSpec.scopeBody (s : StaticSpec) (fuel : Nat := 64) : StaticSpec :=
  match fuel, s with
  | n + 1, .inCaller b => b.scopeBody n
  | _, b => b
termination_by structural fuel

def Action.canonicalPatient (action : Action) (fuel : Nat := 32) : Option NounPhrase :=
  match fuel, action with
  | n + 1, .inCaller body => (body.canonicalPatient n).map NounPhrase.inCaller
  | n + 1, .bracketed body => body.canonicalPatient n
  | _ + 1, .move subject .. | _ + 1, .setStatus _ subject
  | _ + 1, .turnOver subject | _ + 1, .clearDamage subject
  | _ + 1, .target subject | _ + 1, .block subject => some subject
  | _, _ => none
termination_by structural fuel

mutual
  def Action.namesThisDoor (i : Action) (fuel : Nat := 64) : Bool :=
    match fuel, i with
    | n + 1, .inCaller b | n + 1, .ownScope _ b | n + 1, .bracketed b
    | n + 1, .enact _ b _ _ | n + 1, .instruct _ b | n + 1, .limited b _
    | n + 1, .doForEach _ b | n + 1, .doForEachKind _ _ _ b => b.namesThisDoor n
    | _ + 1, .unlock .thisDoor => true
    | n + 1, .withContinuation _ b yes no =>
      b.namesThisDoor n || yes.any (fun i => i.namesThisDoor n) || no.any (fun i => i.namesThisDoor n)
    | n + 1, .doIf condition b no =>
      b.namesThisDoor n || condition.namesThisDoor n || no.any (fun i => i.namesThisDoor n)
    | n + 1, .sequentially bs | n + 1, .simultaneously bs => bs.any (fun i => i.namesThisDoor n)
    | n + 1, .repeat_ (.fixed _ b) => b.namesThisDoor n
    | n + 1, .chooseModes _ modes => modes.any (fun (cost, _, body) =>
      cost.any (fun c => c.namesThisDoor n) || body.namesThisDoor n)
    | n + 1, .delay ev alts duration body =>
      ev.namesThisDoor n || alts.any (fun e => e.namesThisDoor n) ||
      duration.any (fun d => d.namesThisDoor n) || body.namesThisDoor n
    | n + 1, .holdUntil body ev => body.namesThisDoor n || ev.namesThisDoor n
    | n + 1, .replace a b => a.namesThisDoor n || b.namesThisDoor n
    | n + 1, .triggerReflexively body ev consequent =>
      body.namesThisDoor n || ev.any (fun e => e.namesThisDoor n) || consequent.namesThisDoor n
    | n + 1, .establish spec duration =>
      spec.namesThisDoor n || duration.any (fun d => d.namesThisDoor n)
    | n + 1, .repeat_ (.untilCond condition) => condition.namesThisDoor n
    | n + 1, .addMana _ _ riders => riders.any (fun rider => match rider with
      | .onSpent _ _ _ body => body.namesThisDoor n | _ => false)
    | n + 1, .pay cost .. => cost.namesThisDoor n
    | n + 1, .preventDamage _ damage also =>
      damage.any (fun c => c.namesThisDoor n) || also.any (fun i => i.namesThisDoor n)
    | n + 1, .redirectDamage _ _ damage => damage.any (fun c => c.namesThisDoor n)
    | n + 1, .applyResultsTable rows => rows.any (fun row => row.action.namesThisDoor n)
    | n + 1, .createObject _ (.emblem _ abilities) _ => abilities.any (fun a => a.namesThisDoor n)
    | _, _ => false
  termination_by structural fuel

  def GameEvent.namesThisDoor (e : GameEvent) (fuel : Nat := 64) : Bool :=
    match fuel, e with
    | n + 1, .inCaller b | n + 1, .nthOccurrence _ _ b => b.namesThisDoor n
    | n + 1, .causes cause b =>
      b.namesThisDoor n || (match cause with | .event e => e.namesThisDoor n | _ => false)
    | n + 1, .does _ i => i.namesThisDoor n
    | n + 1, .stateHolds condition => condition.namesThisDoor n
    | _, _ => false
  termination_by structural fuel

  def Cost.namesThisDoor (cost : Cost) (fuel : Nat := 64) : Bool :=
    match fuel, cost with
    | n + 1, .inCaller c | n + 1, .scaled c _ => c.namesThisDoor n
    | n + 1, .withSubstitute cost substitute => cost.namesThisDoor n || substitute.action.namesThisDoor n
    | n + 1, .compound cs | n + 1, .or cs => cs.any (fun c => c.namesThisDoor n)
    | n + 1, .perform i => i.namesThisDoor n
    | _, _ => false
  termination_by structural fuel

  def StaticSpec.namesThisDoor (spec : StaticSpec) (fuel : Nat := 64) : Bool :=
    match fuel, spec with
    | n + 1, .inCaller s | n + 1, .retention s _ => s.namesThisDoor n
    | n + 1, .conditional s condition _ => s.namesThisDoor n || condition.namesThisDoor n
    | n + 1, .conjunction _ parts => parts.any (fun s => s.namesThisDoor n)
    | n + 1, .characteristicChange _ edits => edits.any (fun edit => match edit with
      | .addedAbilities abilities => abilities.any (fun a => a.namesThisDoor n)
      | _ => false)
    | n + 1, .cant _ i mods => i.namesThisDoor n || (match mods.cause with
      | some (.event e) => e.namesThisDoor n | _ => false)
    | n + 1, .must _ i mods | n + 1, .can _ i mods =>
      i.namesThisDoor n || mods.rider.any (fun r => r.namesThisDoor n)
    | n + 1, .gatedBy cost _ i _ => cost.namesThisDoor n || i.namesThisDoor n
    | n + 1, .replace event i timing _ _ => event.namesThisDoor n || i.namesThisDoor n || timing.any (fun t => t.namesDoor n)
    | n + 1, .whileHolds condition i => condition.namesThisDoor n || i.namesThisDoor n
    | n + 1, .specialAction _ timing cost i => cost.namesThisDoor n || i.namesThisDoor n || timing.any (fun t => t.namesDoor n)
    | n + 1, .paySubstitute _ substitute => substitute.action.namesThisDoor n
    | n + 1, .asEvent ev i | n + 1, .rollModifier ev i _ => ev.namesThisDoor n || i.namesThisDoor n
    | n + 1, .additionalTriggers ev _ => ev.namesThisDoor n
    | n + 1, .addedCost cost _ _ => cost.namesThisDoor n
    | n + 1, .altCost _ cost => cost.any (fun c => c.namesThisDoor n)
    | _, _ => false
  termination_by structural fuel

  def Condition.namesThisDoor (condition : Condition) (fuel : Nat := 64) : Bool :=
    match fuel, condition with
    | n + 1, .inCaller c | n + 1, .not c => c.namesThisDoor n
    | n + 1, .and cs | n + 1, .or cs => cs.any (fun c => c.namesThisDoor n)
    | n + 1, .did clause => clause.action.namesThisDoor n
    | _, _ => false
  termination_by structural fuel

  def Duration.namesThisDoor (duration : Duration) (fuel : Nat := 64) : Bool :=
    match fuel, duration with
    | n + 1, .between start finish => start.any (fun e => e.namesThisDoor n) || finish.any (fun e => e.namesThisDoor n)
    | n + 1, .forAsLongAs condition => condition.namesThisDoor n
    | _, _ => false
  termination_by structural fuel

  def WouldClause.namesThisDoor (clause : WouldClause) (fuel : Nat := 64) : Bool :=
    match fuel, clause with
    | n + 1, .would _ i => i.namesThisDoor n
    | n + 1, .wouldAny clauses => clauses.any (fun c => c.namesThisDoor n)
    | n + 1, .causedBy cause c => c.namesThisDoor n || (match cause with
      | .event e => e.namesThisDoor n | _ => false)
    | n + 1, .nthOccurrence _ _ c => c.namesThisDoor n
    | _, _ => false
  termination_by structural fuel

  def Replacement.namesThisDoor (replacement : Replacement) (fuel : Nat := 64) : Bool :=
    match fuel, replacement with
    | n + 1, .inCaller body => body.namesThisDoor n
    | n + 1, .instead i | n + 1, .alongside i => i.namesThisDoor n
    | n + 1, .before i body => i.namesThisDoor n || body.any (fun b => b.namesThisDoor n)
    | n + 1, .onlyIf condition body => condition.namesThisDoor n || body.namesThisDoor n
    | _, _ => false
  termination_by structural fuel

  def Timing.namesDoor (timing : Timing) (fuel : Nat := 64) : Bool :=
    match fuel, timing with
    | n + 1, .relativeToEvent _ ev | n + 1, .whileDoing ev => ev.namesThisDoor n
    | n + 1, .onlyIf condition => condition.namesThisDoor n
    | n + 1, .and ts | n + 1, .or ts => ts.any (fun t => t.namesDoor n)
    | n + 1, .not t => t.namesDoor n
    | _, _ => false
  termination_by structural fuel

  def Ability.namesThisDoor (a : Ability) (fuel : Nat := 64) : Bool :=
    match fuel, a with
    | n + 1, .activated cost i wh _ _ _ =>
      cost.namesThisDoor n || i.namesThisDoor n || (match wh with
        | some (.whileDoing ev) => ev.namesThisDoor n | _ => false)
    | n + 1, .spell wh i => i.namesThisDoor n || (match wh with
        | some (.whileDoing ev) => ev.namesThisDoor n | _ => false)
    | n + 1, .triggered e alts wh joins _ _ _ i =>
      e.namesThisDoor n || alts.any (fun e => e.namesThisDoor n) ||
      (match wh with | some (.whileDoing ev) => ev.namesThisDoor n | _ => false) ||
      joins.any (fun j => j.event.namesThisDoor n || j.alternatives.any (fun e => e.namesThisDoor n)) ||
      i.namesThisDoor n
    | n + 1, .italicHead _ a | n + 1, .inCaller a | n + 1, .qualified a _ _ _ _ => a.namesThisDoor n
    | n + 1, .static spec => spec.namesThisDoor n
    | n + 1, .keyword _ _ body => body.any (fun a => a.namesThisDoor n)
    | n + 1, .byCardClass onPermanent onInstantOrSorcery =>
      onPermanent.namesThisDoor n || onInstantOrSorcery.namesThisDoor n
    | _, _ => false
  termination_by structural fuel
end

def Timing.namesThisDoor : Option Timing → Bool
  | some timing => timing.namesDoor
  | none => false

def JoinedHeader.namesThisDoor (j : JoinedHeader) : Bool :=
  j.event.namesThisDoor || j.alternatives.any (fun e => e.namesThisDoor) || Timing.namesThisDoor j.while_

private theorem delayedDoorReferenceIsFound :
    (Action.delay (.does .any (.unlock .thisDoor)) [] none
      (.sequentially [])).namesThisDoor = true := by decide

private theorem nestedDoorReferenceIsFound :
    (Action.sequentially [.replace (.sequentially [])
      (.instead (.holdUntil (.unlock .thisDoor) .now))]).namesThisDoor = true := by decide

private theorem unrelatedUnlockIsNotThisDoor :
    (Action.sequentially []).namesThisDoor = false := by decide

/-- The general category an ability belongs to [CR#113.3]: instructions followed while an
instant or sorcery spell resolves, a statement that is simply true, a trigger condition with an
effect, or a cost with an effect [CR#113.3a,113.3b,113.3c,113.3d]. -/
inductive AbilityCategory where
  | static | triggered | activated | spell
  deriving DecidableEq, Repr

def Ability.category (a : Ability) (fuel : Nat := 64) : Option AbilityCategory :=
  match fuel, a with
  | _ + 1, .static _ => some .static
  | _ + 1, .triggered .. => some .triggered
  | _ + 1, .activated .. => some .activated
  | _ + 1, .spell .. => some .spell
  | n + 1, .inCaller a | n + 1, .qualified a _ _ _ _ => a.category n
  | _, _ => none
termination_by structural fuel

def Predicate.writtenTypes (p : Predicate) (fuel : Nat := 64) : List CardType :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .bracketed b | n + 1, .compareOver b _ _ _ =>
    b.writtenTypes n
  | _ + 1, .hasType t => [t]
  | n + 1, .and ps => ps.foldr (fun p ts => mergeTypes (p.writtenTypes n) ts) []
  | n + 1, .or (p :: ps) | n + 1, .coord .andOr (p :: ps) =>
    ps.foldl (fun ts p => commonTypes ts (p.writtenTypes n)) (p.writtenTypes n)
  | _, _ => []
termination_by structural fuel

def combineTypeAlts (left right : List (List CardType)) : List (List CardType) :=
  left.flatMap fun l => right.map (mergeTypes l)

def Predicate.headTyAlts (p : Predicate) (fuel : Nat := 64) : List (List CardType) :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .bracketed b | n + 1, .compareOver b _ _ _ => b.headTyAlts n
  | n + 1, .and ps =>
    ps.foldr (fun p alts => combineTypeAlts (p.headTyAlts n) alts) [[]]
  | n + 1, .or ps | n + 1, .coord .andOr ps => ps.flatMap (fun p => p.headTyAlts n)
  | _ + 1, .hasType t => [[t]]
  | _ + 1, .hasSubtype s => [s.type.toList]
  | _, _ => [[]]
termination_by structural fuel

end Semantics
