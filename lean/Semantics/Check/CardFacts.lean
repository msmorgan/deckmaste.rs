import Semantics.Abilities
import Semantics.Check.Keywords

namespace Semantics

def Instruction.scopeBody (i : Instruction) (fuel : Nat := 64) : Instruction :=
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

def Instruction.classGuard (i : Instruction) : Option CardClass :=
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

def Instruction.namesThisDoor (i : Instruction) (fuel : Nat := 64) : Bool :=
  match fuel, i with
  | n + 1, .inCaller b | n + 1, .ownScope _ b | n + 1, .bracketed b => b.namesThisDoor n
  | _ + 1, .unlock .thisDoor => true
  | _, _ => false
termination_by structural fuel

def GameEvent.namesThisDoor (e : GameEvent) (fuel : Nat := 64) : Bool :=
  match fuel, e with
  | n + 1, .inCaller b | n + 1, .nthOccurrence _ _ b | n + 1, .causes _ b => b.namesThisDoor n
  | _ + 1, .does _ i => i.namesThisDoor
  | _, _ => false
termination_by structural fuel

def Timing.namesThisDoor : Option Timing → Bool
  | some (.whileDoing ev) => ev.namesThisDoor
  | _ => false

def JoinedHeader.namesThisDoor (j : JoinedHeader) : Bool :=
  j.event.namesThisDoor || j.alternatives.any (fun e => e.namesThisDoor) ||
    Timing.namesThisDoor j.while_

def Ability.namesThisDoor (a : Ability) (fuel : Nat := 64) : Bool :=
  match fuel, a with
  | _ + 1, .activated _ i _ _ _ _ | _ + 1, .spell _ i => i.namesThisDoor
  | _ + 1, .triggered e alts wh joins _ _ _ i =>
    e.namesThisDoor || alts.any (fun e => e.namesThisDoor) || Timing.namesThisDoor wh ||
      joins.any JoinedHeader.namesThisDoor || i.namesThisDoor
  | n + 1, .italicHead _ a | n + 1, .alsoForKeywords a _ | n + 1, .qualified a _ _ _ _ =>
    a.namesThisDoor n
  | _, _ => false
termination_by structural fuel

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
  | _ + 1, .altCost .. | _ + 1, .may _ (.procedure ..) _ => r != some .atResolution
  | _ + 1, .instead (.would _ (.move n (.zones [.zone .stack .bare]) _ _ _ _)) _ _ _ _ =>
    if n.namesThis then r != some .atResolution else r == none
  | _, _ => r == none
termination_by structural fuel

def bodyEventRegime (e : GameEvent) (fuel : Nat := 64) : Option StackRegime :=
  match fuel, e with
  | n + 1, .inCaller e | n + 1, .nthOccurrence _ _ e | n + 1, .causes _ e =>
    bodyEventRegime e n
  | _ + 1, .does _ (.procedure .cast ..) => some .atCasting
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
