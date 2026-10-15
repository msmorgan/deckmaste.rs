import Semantics.Abilities
import Semantics.Check.Words
import Semantics.Check.CardFacts

namespace Semantics

def Predicate.seedTy (p : Predicate) (fuel : Nat := 32) : List CardType :=
  match fuel, p with
  | 0, _ => []
  | n + 1, .inCaller body | n + 1, .bracketed body => body.seedTy n
  | _ + 1, .hasType t => [t]
  | _ + 1, .hasSubtype s => s.type.toList
  | n + 1, .superlative _ _ dom | n + 1, .compareOver dom _ _ _ => dom.seedTy n
  | n + 1, .and ps => ps.foldr (fun q acc => mergeTypes (q.seedTy n) acc) []
  | _ + 1, .or [] => []
  | n + 1, .or [q] => q.seedTy n
  | n + 1, .or (q :: qs) => qs.foldl (fun acc r => commonTypes acc (r.seedTy n)) (q.seedTy n)
  | _ + 1, _ => []
termination_by structural fuel

def optCT : Option CardType → List CardType
  | none => []
  | some t => [t]

def soleAlt : List CardType → List (List CardType)
  | [] => []
  | ts => [ts]

def attachTysOk (w : AttachWord) (ts : List CardType) : Bool :=
  ts.isEmpty || ts.any fun t => attachHeadOk w (.type t)

def attachWordsOk (ws : List AttachWord) (alts : List (List CardType)) : Bool :=
  ws.all fun w => alts.all (attachTysOk w)

def allNegated (negs : List CardType) : List CardType → Bool
  | [] => true
  | t :: ts => negs.elem t && allNegated negs ts

def Predicate.headTyAltsAll (ps : List Predicate) : List (List CardType) :=
  ps.foldr (fun p acc => combineTypeAlts p.headTyAlts acc) [[]]

def Predicate.attachWordsIn (p : Predicate) (fuel : Nat := 32) : List AttachWord :=
  match fuel, p with
  | 0, _ => []
  | n+1, .inCaller p | n+1, .bracketed p => p.attachWordsIn n
  | _+1, .attachment .host (some w) _ => [w]
  | n+1, .and ps | n+1, .or ps => ps.flatMap (fun p => p.attachWordsIn n)
  | _+1, _ => []
termination_by structural fuel

def Predicate.attachWordsInAll (ps : List Predicate) : List AttachWord :=
  ps.flatMap (fun p => p.attachWordsIn)

def Predicate.seedType (p : Predicate) (fuel : Nat := 32) : Option CardType :=
  match fuel, p with
  | 0, _ => none
  | n+1, .inCaller p | n+1, .bracketed p => p.seedType n
  | _+1, .inCombat .attackedBy _ => none
  | _+1, .inCombat _ _ => some .creature
  | _+1, .hasDesignation d _ => d.seedType
  | _+1, .compare cs _ _ => axisTypes cs
  | _+1, .superlative _ axis _ => axis.type
  | n+1, .compareOver dom _ _ _ => dom.seedType n
  | _+1, .hasSubtype s => s.type
  | n+1, .and ps => (ps.filterMap (fun p => p.seedType n)).head?
  | n+1, .or (p::ps) =>
    let t := p.seedType n
    if ps.all (fun p => p.seedType n == t) then t else none
  | _+1, _ => none
termination_by structural fuel

/-- Look through private scopes for structural predicate facts only. Context-sensitive
checking and introductions still traverse the original scopes. -/
def Predicate.scopeBody (p : Predicate) (fuel : Nat := 32) : Predicate :=
  match fuel, p with
  | 0, _ => p
  | n+1, .inCaller body | n+1, .bracketed body => body.scopeBody n
  | _+1, body => body
termination_by structural fuel

def Predicate.flattenOne (p : Predicate) (fuel : Nat := 32) : List Predicate :=
  match fuel, p with
  | 0, _ => []
  | n+1, .inCaller p | n+1, .bracketed p => p.flattenOne n
  | n+1, .and ps => ps.flatMap (fun p => p.flattenOne n)
  | _+1, .not p => [.not p.scopeBody]
  | _+1, p => [p]
termination_by structural fuel

def Predicate.flatten (ps : List Predicate) : List Predicate := ps.flatMap (fun p => p.flattenOne)

/-- Each status category always has exactly one of its two values [CR#110.5]. -/
def statusClashOf : Predicate → Predicate → Bool
  | .hasStatus v, .hasStatus w => v.clash w
  | _, _ => false

/-- A colorless object has no color [CR#105.2c]: "colorless" is written `colorCount .eq 0`. -/
def Predicate.isColorless (p : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, p with
  | 0, _ => false
  | n+1, .inCaller body | n+1, .bracketed body => body.isColorless n
  | _+1, .colorCount .eq 0 => true
  | _+1, _ => false
termination_by structural fuel

def colorClashOf : Predicate → Predicate → Bool
  | p, .colorIs _ => p.isColorless
  | .colorIs _, p => p.isColorless
  | _, _ => false

/-- "Permanent" is written `inZone battlefield` [CR#110.1]. -/
def Predicate.isPermanentHead (p : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, p with
  | 0, _ => false
  | n+1, .inCaller body | n+1, .bracketed body => body.isPermanentHead n
  | _+1, .inZone (.zone .battlefield _) => true
  | _+1, _ => false

termination_by structural fuel

/-- A token is not a card [CR#111.6]; an emblem is neither [CR#114.5]. An emblem is not a
permanent either, but "permanent" is spelled as the battlefield, so the zone law carries that. -/

def cardTokenClashOf : Predicate → Predicate → Bool
  | .isCard, .isToken | .isToken, .isCard | .isEmblem, .isCard | .isCard, .isEmblem
  | .isEmblem, .isToken | .isToken, .isEmblem
  | .isCard, .isCopyOfACard | .isCopyOfACard, .isCard => true
  | _, _ => false

def noClash (clash : Predicate → Predicate → Bool) : List Predicate → Bool
  | [] => true
  | p :: ps => !ps.any (clash p) && noClash clash ps

/-- Instant and sorcery cards can't be permanents [CR#110.4]. -/
def anyNonPermanentTy : List Predicate → Bool
  | [] => false
  | p :: ps =>
    p.seedTy.any CardType.isInstantOrSorcery || anyNonPermanentTy ps

def Predicate.negTypesOf (p : Predicate) (fuel : Nat := 32) : List CardType :=
  match fuel, p with
  | 0, _ => []
  | n+1, .inCaller body | n+1, .bracketed body => body.negTypesOf n
  | _+1, .not (.hasSubtype _) => []
  | _+1, .not p => p.seedTy
  | _+1, _ => []
termination_by structural fuel

def negTypes (ps : List Predicate) : List CardType := ps.flatMap Predicate.negTypesOf

/-- A subtype is correlated to its card type [CR#205.3c]; negate that type and the
description's seed is empty. -/
def Predicate.seedTypeAlts (p : Predicate) (fuel : Nat := 32) : List CardType :=
  match fuel, p with
  | 0, _ => []
  | n+1, .inCaller body | n+1, .bracketed body => body.seedTypeAlts n
  | _+1, .hasSubtype s =>
    match s.type with
    | some .creature => [.creature, .kindred]
    | some t => [t]
    | none => [.instant, .sorcery]
  | _+1, p => optCT p.seedType
termination_by structural fuel

def anySeedEmptied (negs : List CardType) : List Predicate → Bool
  | [] => false
  | p :: ps =>
    match p.seedTypeAlts with
    | [] => anySeedEmptied negs ps
    | ts => allNegated negs ts || anySeedEmptied negs ps

/-- An Equipment is attached only to a creature [CR#301.5] and a Fortification only to a land
[CR#301.6]. -/
def noAttachHeadClash (fs : List Predicate) : Bool :=
  attachWordsOk (Predicate.attachWordsInAll fs) (Predicate.headTyAltsAll fs)

def Predicate.contradictionFree (ps : List Predicate) : Bool :=
  let fs := Predicate.flatten ps
  !anySeedEmptied (negTypes fs) fs && noClash statusClashOf fs && noClash colorClashOf fs &&
    noClash cardTokenClashOf fs && noAttachHeadClash fs &&
    !(fs.any Predicate.isPermanentHead && anyNonPermanentTy fs)


def Predicate.negatable (predicate : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body | n + 1, .bracketed body => body.negatable n
  | _ + 1, .anyPlayer | _ + 1, .chosenPlayer _ | _ + 1, .qualityNoun _ none
  | _ + 1, .counterKindOn _ | _ + 1, .isSource => false
  | _, _ => true
termination_by structural fuel

def Predicate.says (predicate : Predicate) (fuel : Nat := 32) : Bool :=
  match fuel, predicate with
  | n + 1, .inCaller body | n + 1, .bracketed body | n + 1, .not body => body.says n
  | n + 1, .and predicates => predicates.any (fun p => p.says n)
  | _, _ => true
termination_by structural fuel

def Amount.read (amount : Amount) (fuel : Nat := 32) : Bool :=
  match fuel, amount with
  | n + 1, .inCaller body => body.read n
  | _ + 1, .lit _ | _ + 1, .arith _ _ _ | _ + 1, .thatMuch | _ + 1, .chosenNumber _
  | _ + 1, .groupSize | _ + 1, .half _ _ | _ + 1, .upTo _ => false
  | _ + 1, .theOutcome sort => sort.comparable
  | _, _ => true
termination_by structural fuel

def Predicate.roleSubtype (predicate : Predicate) (fuel : Nat := 32) : Option Subtype :=
  match fuel, predicate with
  | n + 1, .inCaller body | n + 1, .bracketed body => body.roleSubtype n
  | _ + 1, .hasSubtype sub => some sub
  | _, _ => none
termination_by structural fuel

def rolesDistinct : List Predicate → Bool
  | [] => true
  | predicate :: rest =>
    !(rest.any fun candidate => match predicate.roleSubtype, candidate.roleSubtype with
      | some left, some right => left == right
      | _, _ => false) && rolesDistinct rest

def rolesOk (roles : List Predicate) : Bool := !roles.isEmpty && rolesDistinct roles

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

private theorem cardAndTokenContradictThroughScopes :
    Predicate.contradictionFree [.inCaller (.and [.isCard, .isToken])] = false := by decide

private theorem colorlessAndColoredContradict :
    Predicate.contradictionFree [.colorCount .eq 0, .colorIs .red] = false := by decide

private theorem typeDisjunctionIsNotTypeConjunction :
    Predicate.contradictionFree [.or [.hasType .instant, .hasType .creature],
      .inZone (.zone .battlefield .bare)] = true := by decide

end Semantics
