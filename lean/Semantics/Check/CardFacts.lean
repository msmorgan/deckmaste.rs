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

def NounPhrase.namesThis (p : NounPhrase) (fuel : Nat := 64) : Bool :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .firstMention _ b => b.namesThis n
  | n + 1, .asType _ b _ | n + 1, .asMarker _ b => b.namesThis n
  | _ + 1, .this => true
  | _, _ => false
termination_by structural fuel

def Predicate.impliedClass (p : Predicate) (fuel : Nat := 64) : Option CardClass :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .bracketed b => b.impliedClass n
  | _ + 1, .hasType t =>
    if t.isInstantOrSorcery then some .instantOrSorceryCard
    else if t.permanent then some .permanentCard else none
  | n + 1, .or ps =>
    match ps with
    | [] => none
    | p :: rest => let c := p.impliedClass n
                   if rest.all (fun q => q.impliedClass n == c) then c else none
  | n + 1, .and ps =>
    let cs := (ps.filterMap fun q => q.impliedClass n).eraseDups
    match cs with | [c] => some c | _ => none
  | _, _ => none
termination_by structural fuel

def Condition.impliedClass (c : Condition) (fuel : Nat := 64) : Option CardClass :=
  match fuel, c with
  | n + 1, .inCaller b => b.impliedClass n
  | _ + 1, .matches .this p => p.impliedClass
  | n + 1, .and cs =>
    let classes := (cs.filterMap fun q => q.impliedClass n).eraseDups
    match classes with | [c] => some c | _ => none
  | _, _ => none
termination_by structural fuel

def Action.classGuard (i : Action) : Option CardClass :=
  match i.scopeBody with | .doIf c _ none => c.impliedClass | _ => none

def StaticSpec.classGuard (s : StaticSpec) : Option CardClass :=
  match s.scopeBody with
  | .conditional _ _ .unless_ => none
  | .conditional _ c _ => c.impliedClass
  | _ => none

def Ability.classGuard (a : Ability) (fuel : Nat := 64) : Option CardClass :=
  match fuel, a with
  | _ + 1, .spell _ i | _ + 1, .activated _ i _ _ _ _ => i.classGuard
  | _ + 1, .triggered _ _ _ _ _ _ c i => (c >>= Condition.impliedClass) <|> i.classGuard
  | _ + 1, .static s => s.classGuard
  | n + 1, .qualified a _ _ _ _ => a.classGuard n
  | _, _ => none
termination_by structural fuel

def Cost.offBattlefield (c : Cost) (fuel : Nat := 64) : Bool :=
  match fuel, c with
  | n + 1, .inCaller b | n + 1, .scaled b _ => b.offBattlefield n
  | _ + 1, .tapSymbol | _ + 1, .untapSymbol | _ + 1, .loyaltySymbol _ => false
  | n + 1, .compound cs | n + 1, .or cs => cs.all (fun c => c.offBattlefield n)
  | _, _ => true
termination_by structural fuel

mutual
  def Action.namesThisDoor (i : Action) (fuel : Nat := 64) : Bool :=
    match fuel, i with
    | n + 1, .inCaller b | n + 1, .ownScope _ b | n + 1, .bracketed b
    | n + 1, .enact _ b _ _ | n + 1, .instruct _ b | n + 1, .limited b _
    | n + 1, .doForEach _ b | n + 1, .doForEachKind _ _ _ b
    | n + 1, .doAndForbid b _ _ => b.namesThisDoor n
    | _ + 1, .unlock .thisDoor => true
    | n + 1, .withContinuation _ b yes no =>
      b.namesThisDoor n || yes.any (fun i => i.namesThisDoor n) || no.any (fun i => i.namesThisDoor n)
    | n + 1, .doOnlyIf b condition no | n + 1, .doIf condition b no =>
      b.namesThisDoor n || condition.namesThisDoor n || no.any (fun i => i.namesThisDoor n)
    | n + 1, .sequentially bs | n + 1, .simultaneously bs => bs.any (fun i => i.namesThisDoor n)
    | n + 1, .repeat_ (.fixed _ b) => b.namesThisDoor n
    | n + 1, .chooseModes _ modes => modes.any (fun (cost, _, body) =>
      cost.any (fun c => c.namesThisDoor n) || body.namesThisDoor n)
    | n + 1, .delay ev alts duration body =>
      ev.namesThisDoor n || alts.any (fun e => e.namesThisDoor n) ||
      duration.any (fun d => d.namesThisDoor n) || body.namesThisDoor n
    | n + 1, .holdUntil body ev => body.namesThisDoor n || ev.namesThisDoor n
    | n + 1, .replace a b | n + 1, .triggerReflexively a b => a.namesThisDoor n || b.namesThisDoor n
    | n + 1, .triggerThisWay body ev consequent =>
      body.namesThisDoor n || ev.namesThisDoor n || consequent.namesThisDoor n
    | n + 1, .establish spec duration =>
      spec.namesThisDoor n || duration.any (fun d => d.namesThisDoor n)
    | n + 1, .repeat_ (.untilCond condition) => condition.namesThisDoor n
    | n + 1, .addMana _ _ riders => riders.any (fun rider => match rider with
      | .onSpent _ _ _ body => body.namesThisDoor n | _ => false)
    | n + 1, .pay cost .. => cost.namesThisDoor n
    | n + 1, .applyResultsTable rows => rows.any (fun row => row.action.namesThisDoor n)
    | n + 1, .createObject _ (.emblem _ abilities) _ _ => abilities.any (fun a => a.namesThisDoor n)
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
    | n + 1, .abilityGrant _ a => a.namesThisDoor n
    | n + 1, .cant _ i _ | n + 1, .must _ i _ | n + 1, .can _ i _ => i.namesThisDoor n
    | n + 1, .gatedBy cost _ i _ => cost.namesThisDoor n || i.namesThisDoor n
    | n + 1, .instead event i timing _ _ => event.namesThisDoor n || i.namesThisDoor n || timing.any (fun t => t.namesDoor n)
    | n + 1, .whileHolds condition i => condition.namesThisDoor n || i.namesThisDoor n
    | n + 1, .specialAction timing cost i => cost.namesThisDoor n || i.namesThisDoor n || timing.any (fun t => t.namesDoor n)
    | n + 1, .damageRule _ _ _ (.prevent _ also) _ => also.any (fun i => i.namesThisDoor n)
    | n + 1, .paySubstitute _ substitute => substitute.action.namesThisDoor n
    | n + 1, .asEvent ev i => ev.namesThisDoor n || i.namesThisDoor n
    | n + 1, .damageResult rule => rule.does.namesThisDoor n
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
    | n + 1, .italicHead _ a | n + 1, .alsoForKeywords a _ | n + 1, .qualified a _ _ _ _ => a.namesThisDoor n
    | n + 1, .static spec => spec.namesThisDoor n
    | n + 1, .keyword _ _ body => body.any (fun a => a.namesThisDoor n)
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
      (.holdUntil (.unlock .thisDoor) .now)]).namesThisDoor = true := by decide

private theorem unrelatedUnlockIsNotThisDoor :
    (Action.sequentially []).namesThisDoor = false := by decide

def Ability.grantedKeyword : Ability → Option KeywordLabel
  | .keyword k [] _ => if keywordParamless k then some k else none
  | _ => none

def StaticSpec.keyword (spec : StaticSpec) (fuel : Nat := 64) : Option KeywordLabel :=
  match fuel, spec with
  | n + 1, .inCaller body | n + 1, .conditional body _ _ => body.keyword n
  | _ + 1, .abilityGrant _ ability => ability.grantedKeyword
  | _, _ => none
termination_by structural fuel

def Action.keyword (action : Action) (fuel : Nat := 64) : Option KeywordLabel :=
  match fuel, action with
  | n + 1, .inCaller body | n + 1, .ownScope _ body | n + 1, .bracketed body => body.keyword n
  | n + 1, .establish spec _ => spec.keyword n
  | _, _ => none
termination_by structural fuel

def Ability.lineKeyword : Ability → Option KeywordLabel
  | .static spec => spec.keyword
  | .triggered _ _ _ _ _ _ _ action => action.keyword
  | _ => none

def Ability.keywordExtendable (ability : Ability) : Bool := ability.lineKeyword.isSome

def keywordListOk (ability : Ability) (keywords : List KeywordTerm) : Bool :=
  match ability.lineKeyword with
  | none => false
  | some base => !keywords.isEmpty && keywords.all KeywordTerm.bare &&
    keywords.eraseDups.length == keywords.length && !keywords.contains (.the base)

def Ability.category (a : Ability) (fuel : Nat := 64) : Option AbilityCategory :=
  match fuel, a with
  | _ + 1, .static _ => some .static
  | _ + 1, .triggered .. => some .triggered
  | _ + 1, .activated .. => some .activated
  | _ + 1, .spell .. => some .spell
  | n + 1, .qualified a _ _ _ _ => a.category n
  | _, _ => none
termination_by structural fuel

def Ability.limitFits (a : Ability) (limit : Option UsageLimit) : Bool :=
  limit.isNone || a.category == some .activated

def StaticSpec.regimeFits (r : Option StackRegime) (s : StaticSpec)
    (fuel : Nat := 64) : Bool :=
  match fuel, s with
  | n + 1, .inCaller b | n + 1, .conditional b _ _ => b.regimeFits r n
  | _ + 1, .costShift (.asMarker .ability _) _ => r == none
  | _ + 1, .addedCost .. | _ + 1, .costShift .. | _ + 1, .paySubstitute ..
  | _ + 1, .modeChoice .. | _ + 1, .addsTarget .. => r == some .atCasting
  | _ + 1, .altCost .. => r != some .atResolution
  | _ + 1, .can _ action _ =>
    match action.scopeBody with
    | .procedure .. => r != some .atResolution
    | _ => r == none
  | _ + 1, .instead (.would _ (.move n (.zone .stack none) _ _ _ _)) _ _ _ _ =>
    if n.namesThis then r != some .atResolution else r == none
  | _, _ => r == none
termination_by structural fuel

def bodyEventRegime (e : GameEvent) (fuel : Nat := 64) : Option StackRegime :=
  match fuel, e with
  | n + 1, .inCaller e | n + 1, .nthOccurrence _ _ e | n + 1, .causes _ e =>
    bodyEventRegime e n
  | _ + 1, .does _ action =>
    match action.scopeBody with
    | .procedure .cast .. => some .atCasting
    | _ => none
  | _, _ => none
termination_by structural fuel

def Ability.regimeFits (r : Option StackRegime) (a : Ability) (fuel : Nat := 64) : Bool :=
  match fuel, a with
  | _ + 1, .static s => s.regimeFits r
  | _ + 1, .triggered e _ _ _ _ _ _ _ => bodyEventRegime e == r
  | _ + 1, .activated .. => r == none
  | _ + 1, .spell .. => r == some .atResolution
  | n + 1, .qualified a _ _ _ _ => a.regimeFits r n
  | _, _ => false
termination_by structural fuel

def KeywordFacts.admitsClass (row : KeywordFacts) : CardClass → Bool
  | .permanentCard => row.onPermanentCard
  | .instantOrSorceryCard => row.onInstantOrSorceryCard

def KeywordFacts.bodyPartFits (row : KeywordFacts) (a : Ability) : Bool :=
  match a.category with
  | none => false
  | some c =>
    row.definition.any (fun part => part.category == c && a.regimeFits part.regime) &&
      a.classGuard.all row.admitsClass &&
      (c != .spell || !row.onPermanentCard || a.classGuard == some .instantOrSorceryCard)

def keywordBodyPartFits (k : KeywordLabel) (a : Ability) : Bool :=
  (keywordFactsFor k).elim false (fun row => row.bodyPartFits a)

def Ability.generalFormOf (k : KeywordLabel) : Ability → List Ability
  | a@(.keyword k' _ body) => if k' == k then body else [a]
  | a => [a]

def keywordBodyFits (k : KeywordLabel) (body : List Ability) : Bool :=
  (body.flatMap (Ability.generalFormOf k)).all (keywordBodyPartFits k)

def Predicate.writtenTypes (p : Predicate) (fuel : Nat := 64) : List CardType :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .bracketed b | n + 1, .compareOver b _ _ _ =>
    b.writtenTypes n
  | _ + 1, .hasType t => [t]
  | n + 1, .and ps => ps.foldr (fun p ts => mergeTypes (p.writtenTypes n) ts) []
  | n + 1, .or (p :: ps) =>
    ps.foldl (fun ts p => commonTypes ts (p.writtenTypes n)) (p.writtenTypes n)
  | _, _ => []
termination_by structural fuel

def Predicate.writtenTypesAll (ps : List Predicate) : List CardType :=
  ps.foldr (fun p ts => mergeTypes p.writtenTypes ts) []

def combineTypeAlts (left right : List (List CardType)) : List (List CardType) :=
  left.flatMap fun l => right.map (mergeTypes l)

def Predicate.headTyAlts (p : Predicate) (fuel : Nat := 64) : List (List CardType) :=
  match fuel, p with
  | n + 1, .inCaller b | n + 1, .bracketed b | n + 1, .compareOver b _ _ _ => b.headTyAlts n
  | n + 1, .and ps =>
    ps.foldr (fun p alts => combineTypeAlts (p.headTyAlts n) alts) [[]]
  | n + 1, .or ps => ps.flatMap (fun p => p.headTyAlts n)
  | _ + 1, .hasType t => [[t]]
  | _ + 1, .hasSubtype s => [s.type.toList]
  | _, _ => [[]]
termination_by structural fuel

end Semantics
