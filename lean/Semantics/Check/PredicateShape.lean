import Semantics.Abilities
import Semantics.Check.Words
import Semantics.Check.CardFacts

namespace Semantics

def Predicate.seedTy (p : Predicate) (fuel : Nat := 32) : List CardType :=
  match fuel, p with
  | 0, _ => []
  | n + 1, .inCaller body => body.seedTy n
  | _ + 1, .hasType t => [t]
  | _ + 1, .hasSubtype s => s.type.toList
  | n + 1, .superlative _ _ dom | n + 1, .compareOver dom _ _ _ => dom.seedTy n
  | n + 1, .and ps => ps.foldr (fun q acc => mergeTypes (q.seedTy n) acc) []
  | _ + 1, .or [] | _ + 1, .coord .andOr [] => []
  | n + 1, .or [q] | n + 1, .coord .andOr [q] => q.seedTy n
  | n + 1, .or (q :: qs) | n + 1, .coord .andOr (q :: qs) => qs.foldl (fun acc r => commonTypes acc (r.seedTy n)) (q.seedTy n)
  | _ + 1, _ => []
termination_by structural fuel

/-- Look through private scopes for structural predicate facts only. Context-sensitive
checking and introductions still traverse the original scopes. -/
def Predicate.scopeBody (p : Predicate) (fuel : Nat := 32) : Predicate :=
  match fuel, p with
  | 0, _ => p
  | n+1, .inCaller body => body.scopeBody n
  | _+1, body => body
termination_by structural fuel

def Predicate.flattenOne (p : Predicate) (fuel : Nat := 32) : List Predicate :=
  match fuel, p with
  | 0, _ => []
  | n+1, .inCaller p => p.flattenOne n
  | n+1, .and ps => ps.flatMap (fun p => p.flattenOne n)
  | _+1, .not p => [.not p.scopeBody]
  | _+1, p => [p]
termination_by structural fuel

def Predicate.flatten (ps : List Predicate) : List Predicate := ps.flatMap (fun p => p.flattenOne)

def Predicate.negatable (predicate : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body => body.negatable n
  | _ + 1, .anyPlayer | _ + 1, .chosen _ | _ + 1, .qualityNoun _ none
  | _ + 1, .counterKindOn _ | _ + 1, .isSource | _ + 1, .eachKind _ _ => false
  | _, _ => true
termination_by structural fuel

/-- Whether a keyword's quality is "each <value>" as a whole (`Predicate.eachKind`), so that the
keyword is one ability per value [CR#702.16h,702.16i]. -/
def Predicate.isEachKind (predicate : Predicate) (fuel : Nat := 8) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body => body.isEachKind n
  | _ + 1, .eachKind _ _ => true
  | _, _ => false
termination_by structural fuel

/-- Whether a keyword's arguments hold an "each <value>" quality. -/
def KeywordParam.isEachKind : KeywordParam → Bool
  | .quality predicate => predicate.isEachKind
  | _ => false

/-- The qualities "each <value of `axis`>" stands for when the axis alone fixes them: the five
colors [CR#105.1]. Over a domain, which values there are is the game's to read as the effect
applies [CR#611.3a]; on another axis the bench lists none. -/
def KindAxis.eachQualities? : KindAxis → Option (List Predicate)
  | .color => some ([Color.white, .blue, .black, .red, .green].map .colorIs)
  | _ => none

/-- The axis and domain of an "each <value>" quality. -/
def Predicate.eachKindOf? (predicate : Predicate) (fuel : Nat := 8) :
    Option (KindAxis × Option NounPhrase) :=
  match fuel, predicate with
  | n + 1, .inCaller body => body.eachKindOf? n
  | _ + 1, .eachKind axis domain => some (axis, domain)
  | _, _ => none
termination_by structural fuel

/-- The abilities a keyword with one quality argument stands for, each as its keyword and
quality: itself, or, with an "each <value>" quality, one per value, which behave as separate
abilities [CR#702.16h,702.16i]. `none` where the values are the game's to read. -/
def Ability.keywordInstances? : Ability → Option (List (KeywordLabel × Predicate))
  | .keyword label [.quality quality] _ =>
    match quality.eachKindOf? with
    | some (axis, none) => axis.eachQualities?.map (·.map (label, ·))
    | some (_, some _) => none
    | none => some [(label, quality)]
  | _ => none

def Predicate.says (predicate : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body | n + 1, .not body => body.says n
  | n + 1, .and predicates => predicates.any (fun p => p.says n)
  | _, _ => true
termination_by structural fuel

def Amount.read (amount : Amount) (fuel : Nat := 32) : Bool :=
  match fuel, amount with
  | n + 1, .inCaller body => body.read n
  | _ + 1, .lit _ | _ + 1, .arith _ _ _ | _ + 1, .thatMuch | _ + 1, .chosenNumber _
  | _ + 1, .groupSize | _ + 1, .half _ _ => false
  | _ + 1, .theOutcome sort => sort.comparable
  | _, _ => true
termination_by structural fuel

def Condition.isAnd (condition : Condition) (fuel : Nat := 32) : Bool :=
  match fuel, condition with
  | n + 1, .inCaller body => body.isAnd n
  | _ + 1, .and _ => true
  | _, _ => false
termination_by structural fuel

def Condition.isOr (condition : Condition) (fuel : Nat := 32) : Bool :=
  match fuel, condition with
  | n + 1, .inCaller body => body.isOr n
  | _ + 1, .or _ => true
  | _, _ => false
termination_by structural fuel

def Predicate.uniquifies (predicate : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body => body.uniquifies n
  | _ + 1, .chosen _ | _ + 1, .superlative .. | _ + 1, .inCombat .attackedBy _
  | _ + 1, .in_ (.exiledWith _) | _ + 1, .related _ (some _) => true
  | _ + 1, .did clause => clause.ordinal.isSome
  | _ + 1, .hasDesignation label none => label.sole
  | n + 1, .and predicates => predicates.any (·.uniquifies n)
  | _, _ => false
termination_by structural fuel

def Predicate.hasOther (predicate : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body => body.hasOther n
  | _ + 1, .other | _ + 1, .otherThan _ => true
  | n + 1, .and predicates => predicates.any (·.hasOther n)
  | _, _ => false
termination_by structural fuel

def Predicate.hasBareOther (predicate : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body => body.hasBareOther n
  | _ + 1, .other => true
  | n + 1, .and predicates => predicates.any (·.hasBareOther n)
  | _, _ => false
termination_by structural fuel

/-- Whether a noun phrase's own description prints a bare "another"/"other". -/
def NounPhrase.hasBareOther (np : NounPhrase) (fuel : Nat := 8) : Bool :=
  match fuel, np with
  | n + 1, .inCaller body | n + 1, .firstMention _ body | n + 1, .headed _ body =>
    body.hasBareOther n
  | _ + 1, .described _ predicate => predicate.hasBareOther
  | _, _ => false
termination_by structural fuel

/-- Whether a noun phrase names an opponent of yours ("an opponent", "your opponents"). -/
def NounPhrase.namesOpponent : NounPhrase → Bool
  | .described _ (.isOpponentOf .you) => true
  | _ => false

/-- The controller restriction a description states: `some true` for "you control", `some false`
for "an opponent controls", "your opponents control" or "you don't control". -/
def Predicate.controlFact (predicate : Predicate) (fuel : Nat := 32) : Option Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body => body.controlFact n
  | _ + 1, .hasPossessor .controller .you => some true
  | _ + 1, .hasPossessor .controller who => if who.namesOpponent then some false else none
  | _ + 1, .not (.hasPossessor .controller .you) => some false
  | n + 1, .and predicates => predicates.findSome? (·.controlFact n)
  | _, _ => none
termination_by structural fuel

/-- Whether a description says that an opponent controls it ("an opponent controls", "your
opponents control"); "you don't control" does not. -/
def Predicate.opponentControls (predicate : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body => body.opponentControls n
  | _ + 1, .hasPossessor .controller who => who.namesOpponent
  | n + 1, .and predicates => predicates.any (·.opponentControls n)
  | _, _ => false
termination_by structural fuel

/-- Whether what an earlier mention recorded of `ref` says what `conjunct` says: a Card Type its
types include, or a controller restriction it states ("an opponent controls" saying "you don't
control", not the reverse). Any other conjunct is not one a mention records, and admits none. -/
def Referent.admitsConjunct (ref : Referent) : Predicate → Bool
  | .hasType t => Kind.lte .object ref.kind && tyIs t ref.ty
  | .hasPossessor .controller .you => ref.controlledByYou == some true
  | .hasPossessor .controller who => who.namesOpponent && ref.opponentControls
  | .not (.hasPossessor .controller .you) => ref.controlledByYou == some false
  | _ => false

/-- The mentions in `drs`'s Reference Scope a `the` description of one object that `predicate`
does not make unique reads back: one earlier particular (a target, or an "a" or "the" mention)
whose recorded facts say every conjunct of `predicate` ("the creature you control" after "target
creature you control and target creature you don't control"). A moved object is a new object
[CR#400.7], which the restriction of its earlier mention no longer describes; neither the Source
or "you", a hypothetical a Condition introduced, nor an object a linked ability published
(`Referent.linked`) is read. -/
def DRS.definiteAntecedents (drs : DRS) (predicate : Predicate) : List Referent :=
  let conjuncts := predicate.flattenOne
  if conjuncts.isEmpty then [] else
  drs.visibleReferents.filter fun ref =>
    !ref.linked && !ref.defaultAnchor && !ref.fromCondition &&
      ref.movedFrom.isEmpty && ref.plur.isOne &&
      (ref.det == .target || ref.det == .a || ref.det == .the) &&
      conjuncts.all ref.admitsConjunct

/-- The Choice reference a description's "chosen" names, if it says "chosen". -/
def Predicate.chosenRef? (predicate : Predicate) (fuel : Nat := 32) : Option ChoiceRef :=
  match fuel, predicate with
  | n + 1, .inCaller body => body.chosenRef? n
  | _ + 1, .chosen ref => some ref
  | n + 1, .and predicates => predicates.findSome? (·.chosenRef? n)
  | _, _ => none
termination_by structural fuel

/-- Whether what an earlier mention recorded of a chosen object says what `conjunct` says: as
`Referent.admitsConjunct`, and further "chosen" itself, and "card" of any object. -/
def Referent.admitsChosenConjunct (ref : Referent) : Predicate → Bool
  | .chosen _ => ref.chosenObject
  | .isCard => Kind.lte .object ref.kind
  | conjunct => ref.admitsConjunct conjunct

/-- The objects a "chosen" description (`Predicate.chosen`) reads back: those a "choose" in scope
chose, one or a group by `plur`, whose recorded facts say every conjunct of `predicate` ("the
chosen creature you control" after "Choose target creature you control and target creature you
don't control"). A moved object is a new object [CR#400.7], and an object a linked ability
published (`Referent.linked`) is not read. -/
def DRS.chosenAntecedents (drs : DRS) (plur : Plurality) (predicate : Predicate) :
    List Referent :=
  let conjuncts := predicate.flattenOne
  drs.visibleReferents.filter fun ref =>
    ref.chosenObject && !ref.linked && ref.movedFrom.isEmpty &&
      ref.plur.isOne == plur.isOne && conjuncts.all ref.admitsChosenConjunct

def Predicate.isOther (predicate : Predicate) : Bool :=
  match predicate.scopeBody with | .other | .otherThan _ => true | _ => false

def coordinableAll (predicates : List Predicate) : Bool :=
  predicates.all (fun predicate => !predicate.hasOther)

end Semantics
