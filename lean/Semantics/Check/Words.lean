import Semantics.Words
import Semantics.Check.Facts

/-!
# Semantics.Check.Words

The checker's vocabulary layer: everything `Words.idr` computes that is not syntax. The
antecedent stack (`Binding`, `Bindings`, `Payload`), the reads over it (`countReach`,
`countChoice`, `countOutcomes`, …), the facts tables (acts, counters, designations), and the
leaf predicates the phrase and effect rules consult.

Every function keeps its Idris name and clauses. `So (f x)` obligations are the `Bool`
functions here; the rule layer turns them into refusals. Everything is structurally recursive
so `decide` can run it.

Registry data lives in `Check.Facts`; this module interprets its declared columns.
-/

namespace Semantics

/-! ## Card types and stats -/

/-- Canonical order for remembered type facts; order in the written modifiers is immaterial. -/
def normalizeTypes (types : List CardType) : List CardType :=
  [ .creature, .artifact, .land, .enchantment, .instant, .sorcery, .planeswalker, .battle,
    .kindred ].filter (· ∈ types)

def mergeTypes (left right : List CardType) : List CardType :=
  normalizeTypes (left ++ right)

def commonTypes (left right : List CardType) : List CardType :=
  normalizeTypes (left.filter (· ∈ right))

def Stat.comparedType : Stat → Option CardType
  | .power => some .creature
  | .toughness => some .creature
  | .manaValue => none
  | .loyalty => some .planeswalker

/-- A noncreature permanent has no power or toughness [CR#208.3], so a stat read is refused
when every head-type alternative of the noun lacks the type the stat belongs to. No known type
is permissive, and one alternative that carries the type suffices. -/
def statHeadTysOk (c : Stat) (alts : List (List CardType)) : Bool :=
  match c.comparedType with
  | none => true
  | some want => alts.all (·.elem want)

def QualitySort.chosenReadOk : QualitySort → Bool
  | .color => true
  | .subtype _ => true
  | .cardName => true
  | .number => false
  | .cardType => true
  | .counterKind => false
  | .ability => true

/-! ## Kinds -/

/-- `kindLte x y` on a join on the right; `x` is never itself a join here. -/
def Kind.lteAtom (x : Kind) : Kind → Bool
  | .join a b => Kind.lteAtom x a || Kind.lteAtom x b
  | y => x == y

/-- The Idris clauses in order: a join on the left, then a join on the right, then equality.
Two structural recursions so the kernel can evaluate it. -/
def Kind.lte : Kind → Kind → Bool
  | .join a b, y => Kind.lte a y && Kind.lte b y
  | x, y => Kind.lteAtom x y

def AggregateOp.isExtremal : AggregateOp → Bool
  | .sum => false
  | .min => true
  | .max => true

def KindAxis.sort : KindAxis → Option QualitySort
  | .cardType => some .cardType
  | .permanentType => none
  | .color => some .color
  | .subtype host _ => some (.subtype host)
  | .value _ => some .number
  | .counterKind => some .counterKind
  | .colorPair => none

/-- A subtype axis scoped to basic or nonbasic types ranges over land types: the basic land
types are five land types [CR#305.6], and no other card type's subtypes are partitioned so. -/
def KindAxis.ok : KindAxis → Bool
  | .subtype host scope => scope == .any || host == .land
  | _ => true

def KindAxis.closed : KindAxis → Bool
  | .cardType => true
  | .permanentType => true
  | .color => true
  | .subtype .land .basicOnly => true
  | .subtype _ _ => false
  | .value _ => false
  | .counterKind => false
  | .colorPair => true

def colorCountOk (n : Nat) : Bool := n ≥ 2 && n ≤ 5

/-- A color count an object can have: zero ("colorless") through five; the bounds that
compare are the ones that do not always or never hold [CR#105.2]. -/
def colorBoundOk : Comparator → Nat → Bool
  | .eq, n => n ≤ 5
  | .atLeast, n => n ≥ 2 && n ≤ 5
  | .atMost, n => n ≥ 1 && n ≤ 4
  | .greater, n => n ≥ 1 && n ≤ 4
  | .less, n => n ≥ 2 && n ≤ 5

def OutcomeSort.comparable : OutcomeSort → Bool
  | .rollResult => true
  | _ => false

def Plurality.isOne : Plurality → Bool
  | .one => true
  | .many => false

def outputPlur : Plurality → Plurality → Plurality
  | .one, .one => .one
  | _, _ => .many

def atLeastTwo : Nat → Bool
  | _ + 2 => true
  | _ => false

/-! ## The act facts table -/

/-- The deeds the core rules define, as a total function: every constructor of the closed
taxonomy has a row, so no lookup here can fail. Attacking and blocking are the turn-based
actions of the declare-attackers and declare-blockers steps [CR#508.1,509.1]; unlocking and
fully unlocking belong to a card type's own rules, not to a keyword [CR#709.5f,709.5i]. -/
private def coreDeedRuleFacts : CoreDeed → ActFacts
  /- The declaring side of an attack is the active player or a creature they control
  [CR#508.1,508.1a]. -/
  | .attack =>
      { agentRole := ⟨some ⟨.either, [], [.creature]⟩, true, some .battlefield⟩,
        patientRole := ⟨some ⟨.object, [], [.planeswalker, .battle]⟩, false, some .battlefield⟩,
        counterfactual := some .object, bounded := true }
  | .block =>
      { agentRole := ⟨some ⟨.object, [], [.creature]⟩, true, some .battlefield⟩,
        patientRole := ⟨some ⟨.object, [], [.creature]⟩, false, some .battlefield⟩,
        counterfactual := some .object, bounded := true }
  | .target =>
      { agentRole := ⟨none, true, some .stack⟩,
        patientRole := ⟨some ⟨.either, [], allTypes⟩, true, none⟩,
        counterfactual := some .object, bounded := true }
  | .copy =>
      { agentRole := ⟨none, true, some .stack⟩,
        patientRole := ⟨some ⟨.object, [.spell, .ability], spellTypes⟩, true, some .stack⟩,
        bounded := true }
  | .draw =>
      { agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], []⟩, true, some .library⟩, bounded := true }
  | .gainLife => { agentRole := playerAgent, bounded := true }
  | .loseGame => { agentRole := playerAgent }
  | .winGame => { agentRole := playerAgent }
  | .spend => { agentRole := playerAgent, counterfactual := some .mana, bounded := true }
  | .trigger =>
      { agentRole := ⟨some ⟨.object, [.ability], []⟩, true, some .stack⟩, bounded := true }
  | .put =>
      { agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], []⟩, false, none⟩ }
  | .return_ =>
      { agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], []⟩, false, none⟩ }
  | .gainControl => { agentRole := playerAgent, patientRole := fieldObject }
  | .unlock => { agentRole := playerAgent, patientRole := ⟨none, false, some .battlefield⟩ }
  | .fullyUnlock => { agentRole := playerAgent, patientRole := fieldObject }

/-- The deeds a keyword ability defines for itself, keyed by the keyword that defines them:
"crews a Vehicle" [CR#702.122b], "becomes saddled" [CR#702.171a], "phase in" [CR#702.26a]. The
keyword is the key because the deed exists only where its keyword does. -/
def abilityDeedFacts : List (KeywordLabel × ActFacts) :=
  [ ("Crew",
      { agentRole := ⟨some ⟨.object, [], [.creature]⟩, true, some .battlefield⟩,
        patientRole := ⟨some ⟨.object, [], [.artifact]⟩, true, some .battlefield⟩,
        counterfactual := some .value, bounded := true }),
    ("Saddle",
      { agentRole := ⟨some ⟨.object, [], [.creature]⟩, true, some .battlefield⟩,
        patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩,
        counterfactual := some .value, bounded := true }),
    ("Phasing",
      { intransitive := true,
        agentRole := ⟨some ⟨.object, [], []⟩, true, some .battlefield⟩ }) ]

def distinctActLabels : List (KeywordActionLabel × ActFacts) → Bool
  | [] => true
  | f :: fs => !(fs.map (·.1)).elem f.1 && distinctActLabels fs

def distinctAbilityDeedLabels : List (KeywordLabel × ActFacts) → Bool
  | [] => true
  | f :: fs => !(fs.map (·.1)).elem f.1 && distinctAbilityDeedLabels fs

/-- The facts for a deed, dispatched on its source. A core deed always has them. -/
def coreDeedFacts (deed : CoreDeed) : ActFacts :=
  { coreDeedRuleFacts deed with confers := coreDeedConferrals deed }

def deedFacts : Deed → Option ActFacts
  | .core d => some (coreDeedFacts d)
  | .action l => (actFacts.find? (·.1 == l)).map (·.2)
  | .ofAbility k => (abilityDeedFacts.find? (·.1 == k)).map (·.2)

def knownAct (v : Deed) : Bool := (deedFacts v).isSome
def deedFeatureOf (v : Deed) : Option DeedFeature := deedFacts v >>= (·.feature)
def featureLabel (f : DeedFeature) : Option Deed :=
  (actFacts.find? (·.2.feature == some f)).map (fun row => .action row.1)
/-- The deed a feature names, where a guard needs the deed itself rather than the feature; a
label-less keyword action where the table names none. -/
def deedLabel (f : DeedFeature) : Deed := (featureLabel f).getD (.action "")
def participleOf (v : Deed) : Option String := deedFacts v >>= (·.participle)
def actPatientKindsOf (v : Deed) : List Kind :=
  (deedFacts v).elim [] fun f => f.patientRole.sort.elim [] (·.domain.kinds)
def actNamesPatient (v : Deed) : Bool := !(actPatientKindsOf v).isEmpty
def actZoneOf (v : Deed) : Option Zone := deedFacts v >>= (·.patientRole.zone)
def actDestOf (v : Deed) : Option Zone := deedFacts v >>= (·.dest)
def actLociOf (v : Deed) : List Zone := (deedFacts v).elim [] (·.loci)
def actNamesLocus (v : Deed) : Bool := !(actLociOf v).isEmpty
def actStepwiseOf (v : Deed) : Bool := (deedFacts v).elim false (·.stepwise)
def actOpponentsLibrary (v : Deed) : Bool := (deedFacts v).elim false (·.opponentsLibrary)
def actIntransitiveOf (v : Deed) : Bool := (deedFacts v).elim false (·.intransitive)
def actNamesParticiple (v : Deed) : Bool := (participleOf v).isSome

/-! ## The antecedent stack -/

/-- Provenance: the verb that last moved a binding, whether it was on the battlefield then,
and whether it changed zones. -/
structure Stamp where
  verb : Deed
  wasField : Bool
  moved : Bool
  deriving Repr, BEq

inductive Origin where
  | token | copy
  deriving DecidableEq, Repr

def isTokenOrigin : Option Origin → Bool
  | some .token => true
  | _ => false

def isCopyOrigin : Option Origin → Bool
  | some .copy => true
  | _ => false

def facesFit : List PileFace → Nat → Bool
  | [], _ => true
  | fs, n => fs.length == n

def pileMentionFace : List PileFace → Option PileFace
  | [] => none
  | f :: fs => if fs.all (· == f) then some f else none

/-- What a binding knows about its referent. The Idris indexes this by `Kind`; here the kind
is derived from the payload by `Payload.kind`. -/
inductive Payload where
  | object (ty : List CardType) (zone : Option Zone) (prov : Option Stamp) (orig : Option Origin)
      (size : Option Nat)
  | player (chosen : Bool)
  | quality (q : QualitySort)
  | outcome (sort : OutcomeSort)
  | gap
  | letter (l : Letter)
  | turnRef
  /-- An ability object and its remembered location. Stack origin is the default;
  movement can remove it [CR#113.1c,724.1b,724.2b]. -/
  | ability (orig : Option Origin) (zone : Option Zone := some .stack)
  | pile (zone : Option Zone) (size : Option Nat) (face : Option PileFace)
  | join (l r : Payload)
  deriving Repr, BEq

namespace Payload

/-- The kind a payload denotes; an ability on the stack is an object [CR#113.1c]. -/
def kind : Payload → Kind
  | .object .. => .object
  | .player _ => .player
  | .quality q => .quality q
  | .outcome _ => .outcome
  | .gap => .gap
  | .letter l => .letter l
  | .turnRef => .turnRef
  | .ability _ _ => .object
  | .pile .. => .pile
  | .join l r => .join l.kind r.kind

end Payload

structure Binding where
  det : Determiner
  plur : Plurality
  payload : Payload
  deriving Repr, BEq

/-- The kind a binding denotes, derived from its payload. -/
def Binding.kind (b : Binding) : Kind := b.payload.kind

abbrev Bindings := List Binding

namespace Payload

def zone : Payload → Option Zone
  | .object _ zn _ _ _ => zn
  | .ability _ zn => zn
  | .pile zn _ _ => zn
  | .join l r => (l.zone).elim r.zone some
  | _ => none

def joinSeed : List CardType → List CardType → List CardType
  | [], t => t
  | t, [] => t
  | t, u => commonTypes t u

def ty : Payload → List CardType
  | .object t _ _ _ _ => t
  | .join l r => joinSeed l.ty r.ty
  | _ => []

def size : Payload → Option Nat
  | .object _ _ _ _ sz => sz
  | .pile _ sz _ => sz
  | .join l r => (l.size).elim r.size some
  | _ => none

def face : Payload → Option PileFace
  | .pile _ _ fc => fc
  | _ => none

def prov : Payload → Option Stamp
  | .object _ _ pv _ _ => pv
  | .join l r => (l.prov).elim r.prov some
  | _ => none

def orig : Payload → Option Origin
  | .object _ _ _ og _ => og
  | .ability og _ => og
  | .join l r => (l.orig).elim r.orig some
  | _ => none

def joined : Payload → Bool
  | .join _ _ => true
  | _ => false

end Payload

def Binding.zone (b : Binding) : Option Zone := b.payload.zone
def Binding.ty (b : Binding) : List CardType := b.payload.ty
def Binding.size (b : Binding) : Option Nat := b.payload.size
def Binding.face (b : Binding) : Option PileFace := b.payload.face

/-- Re-size an object or pile binding; any other binding is returned unchanged. -/
def sized (sz : Option Nat) : Binding → Binding
  | ⟨det, pl, .object ty zn pv og _⟩ => ⟨det, pl, .object ty zn pv og sz⟩
  | ⟨det, pl, .pile zn _ fc⟩ => ⟨det, pl, .pile zn sz fc⟩
  | b => b

/-- The head type(s) a phrase seeds, one per join half. -/
inductive HeadTy where
  | sole (ty : List CardType)
  | join (a b : HeadTy)
  deriving Repr, BEq

def outcomeB (s : OutcomeSort) : Binding := ⟨.the, .one, .outcome s⟩
def gapB : Binding := ⟨.the, .one, .gap⟩
def turnRefB : Binding := ⟨.the, .one, .turnRef⟩
def letterB (l : Letter) : Binding := ⟨.a, .one, .letter l⟩
def qualityB (q : QualitySort) : Binding := ⟨.a, .one, .quality q⟩

def ChoiceSort.binding : ChoiceSort → Binding
  | .quality q => qualityB q
  | .player => ⟨.a, .one, .player true⟩

def ChoiceSort.binds : ChoiceSort → Kind → Payload → Bool
  | .quality q, k, _ => Kind.lte (.quality q) k
  | .player, _, .player ch => ch
  | .player, _, _ => false

def choiceSortAt : Kind → Option ChoiceSort
  | .quality q => some (.quality q)
  | .player => some .player
  | _ => none

def introducedChoiceAt (k : Kind) : List Binding :=
  match choiceSortAt k with
  | none => []
  | some s => [s.binding]

def countOnes (k : Kind) : Bindings → Nat
  | [] => 0
  | b@⟨_, .one, _⟩ :: bs => if Kind.lte k b.kind then countOnes k bs + 1 else countOnes k bs
  | _ :: bs => countOnes k bs

def countOutcomes (s : OutcomeSort) : Bindings → Nat
  | [] => 0
  | ⟨_, .one, .outcome s'⟩ :: bs =>
    if s == s' then countOutcomes s bs + 1 else countOutcomes s bs
  | _ :: bs => countOutcomes s bs

def outcomeInScope (s : OutcomeSort) : Bindings → Bool
  | [] => false
  | ⟨_, .one, .outcome s'⟩ :: bs => s == s' || outcomeInScope s bs
  | _ :: bs => outcomeInScope s bs

def damageDealtInScope (bs : Bindings) : Bool := outcomeInScope .damageDealt bs
def coinFlipInScope (bs : Bindings) : Bool := outcomeInScope .coinFlipped bs

def ignorableInScope (bs : Bindings) : Bool :=
  countOutcomes .rollResult bs == 1 || coinFlipInScope bs

def OutcomeSort.isAmount : OutcomeSort → Bool
  | .coinFlipped | .manaAdded | .manaProduced | .ceilingShortfall | .voteHeld =>
    false
  | _ => true

def countAmountOutcomes : Bindings → Nat
  | [] => 0
  | ⟨_, .one, .outcome s⟩ :: bs =>
    if s.isAmount then countAmountOutcomes bs + 1 else countAmountOutcomes bs
  | _ :: bs => countAmountOutcomes bs

def countChoice (s : ChoiceSort) : Bindings → Nat
  | [] => 0
  | ⟨_, .one, p⟩ :: bs => if s.binds p.kind p then countChoice s bs + 1 else countChoice s bs
  | _ :: bs => countChoice s bs

/-- Which announced choice a read names: the one standing choice, or the most recent of
several. -/
def ChoiceRef.ok : ChoiceRef → Nat → Bool
  | .theChoice, n => n == 1
  | .theLatestChoice, n => n != 0

def distinctLabels : List VoteLabel → Bool
  | [] => true
  | l :: ls => !ls.elem l && distinctLabels ls

def ballotLabelsOk (opts : List VoteLabel) : Bool := 2 ≤ opts.length && distinctLabels opts

def countLetter (l : Letter) : Bindings → Nat
  | [] => 0
  | b@⟨_, .one, _⟩ :: bs =>
    if Kind.lte (.letter l) b.kind then countLetter l bs + 1 else countLetter l bs
  | _ :: bs => countLetter l bs

def dropLetter (l : Letter) : Bindings → Bindings
  | [] => []
  | b :: bs => if Kind.lte (.letter l) b.kind then dropLetter l bs else b :: dropLetter l bs

def openLetter (l : Letter) (b : Binding) : Bool :=
  match b.det with
  | .a => b.plur.isOne && Kind.lte (.letter l) b.kind
  | _ => false

def anyOpenLetter (l : Letter) (bs : Bindings) : Bool := bs.any (openLetter l)

def defineLetter (l : Letter) : Bindings → Bindings
  | [] => []
  | b :: bs =>
    if openLetter l b then ⟨.the, b.plur, b.payload⟩ :: defineLetter l bs
    else b :: defineLetter l bs

def introducedLetters (l : Letter) (bs : Bindings) : List Binding :=
  if countLetter l bs == 0 then [letterB l] else []

def countManysAny : Bindings → Nat
  | [] => 0
  | ⟨_, .many, _⟩ :: bs => countManysAny bs + 1
  | _ :: bs => countManysAny bs

def countManys (k : Kind) : Bindings → Nat
  | [] => 0
  | b@⟨_, .many, _⟩ :: bs => if Kind.lte k b.kind then countManys k bs + 1 else countManys k bs
  | _ :: bs => countManys k bs

def objGroup (k : Kind) (b : Binding) : Bool := Kind.lte k b.kind && !b.plur.isOne

def countGroups (k : Kind) : Bindings → Nat
  | [] => 0
  | ⟨.part, _, _⟩ :: bs => countGroups k bs
  | ⟨.bare, _, _⟩ :: bs => countGroups k bs
  | b :: bs => if objGroup k b then countGroups k bs + 1 else countGroups k bs

def countParts (k : Kind) : Bindings → Nat
  | [] => 0
  | b@⟨.part, _, _⟩ :: bs => if Kind.lte k b.kind then countParts k bs + 1 else countParts k bs
  | _ :: bs => countParts k bs

def theRestOk (k : Kind) (bs : Bindings) : Bool := countGroups k bs ≤ 1 && countParts k bs != 0

def partsTaken (k : Kind) : Bindings → Nat
  | [] => 0
  | b@⟨.part, _, _⟩ :: bs =>
    if Kind.lte k b.kind
      then (match b.size with
            | some n => n + partsTaken k bs
            | none => partsTaken k bs + 1)
      else partsTaken k bs
  | _ :: bs => partsTaken k bs

def countedGroupSize (k : Kind) : Bindings → Option Nat
  | [] => none
  | ⟨.part, _, _⟩ :: bs => countedGroupSize k bs
  | ⟨.bare, _, _⟩ :: bs => countedGroupSize k bs
  | b :: bs => if objGroup k b then b.size else countedGroupSize k bs

def theOtherOk (k : Kind) (bs : Bindings) : Bool :=
  theRestOk k bs &&
    (match countedGroupSize k bs with
     | none => false
     | some n => n == partsTaken k bs + 1)

def theRestFits (k : Kind) : Plurality → Bindings → Bool
  | .one, bs => theOtherOk k bs
  | .many, bs => theRestOk k bs

def groupSpent (k : Kind) : Bindings → Bindings
  | [] => []
  | ⟨.part, pl, p⟩ :: bs => ⟨.the, pl, p⟩ :: groupSpent k bs
  | b :: bs => if objGroup k b then groupSpent k bs else b :: groupSpent k bs

/-- `out` re-places bindings of `outer` without touching the table: every binding keeps its
determiner, kind and plurality, and only plural ones, one object per agent, may have moved
[CR#701.21a]. -/
def spentDistributively : Bindings → Bindings → Bool
  | [], [] => true
  | a :: as_, b :: bs =>
    a.det == b.det && a.kind == b.kind && a.plur == b.plur &&
      (a.payload == b.payload || !a.plur.isOne) && spentDistributively as_ bs
  | _, _ => false

def partsClosed : Bindings → Bindings
  | [] => []
  | ⟨.part, pl, p⟩ :: bs => ⟨.the, pl, p⟩ :: partsClosed bs
  | b :: bs => b :: partsClosed bs

def partsDistributed : Bindings → Bool
  | [] => true
  | ⟨.part, pl, _⟩ :: bs => !pl.isOne && partsDistributed bs
  | _ :: bs => partsDistributed bs

def restSource (k : Kind) : Bindings → Option Binding
  | [] => none
  | b@⟨.part, _, _⟩ :: bs =>
    match restSource k bs with
    | some s => some s
    | none => if Kind.lte k b.kind then some b else none
  | b :: bs => if objGroup k b then some b else restSource k bs

def zoneOfGroup (k : Kind) (bs : Bindings) : Option Zone := restSource k bs >>= (·.zone)
def tyOfGroup (k : Kind) (bs : Bindings) : List CardType := (restSource k bs).elim [] (·.ty)
def provOfGroup (k : Kind) (bs : Bindings) : Option Stamp := restSource k bs >>= (·.payload.prov)

def anyTargeted (k : Kind) : Bindings → Bool
  | [] => false
  | b@⟨.target, _, _⟩ :: bs => Kind.lte k b.kind || anyTargeted k bs
  | _ :: bs => anyTargeted k bs

def anchorTyOk (t : CardType) : List CardType → Bool
  | [] => true
  | ts => ts.contains t

def anyTargetedAt : Bindings → Bool
  | [] => false
  | ⟨.target, _, _⟩ :: _ => true
  | _ :: bs => anyTargetedAt bs

def settleTargets : Bindings → Bindings
  | [] => []
  | ⟨.target, plur, payload⟩ :: bs => ⟨.the, plur, payload⟩ :: settleTargets bs
  | b :: bs => b :: settleTargets bs

def outcomesOnly : Bindings → Bindings
  | [] => []
  | b@⟨_, _, .outcome _⟩ :: bs => b :: outcomesOnly bs
  | _ :: bs => outcomesOnly bs

def agreedField {α : Type} (f : α → α → Bool) : Option α → Option α → Option α
  | some x, some y => if f x y then some x else none
  | _, _ => none

/-- The binding two arms of a choice agree on, if any: the union of their payloads. -/
def unionPayload : Payload → Payload → Option (Kind × Payload)
  | .object t1 z1 v1 o1 s1, .object t2 z2 v2 o2 s2 =>
    some (.object, .object (commonTypes t1 t2) (agreedField (· == ·) z1 z2)
      (agreedField (· == ·) v1 v2) (agreedField (· == ·) o1 o2) (agreedField (· == ·) s1 s2))
  | .ability o1 z1, .ability o2 z2 =>
    some (.object, .ability (agreedField (· == ·) o1 o2) (agreedField (· == ·) z1 z2))
  | .player a, .player b => if a == b then some (.player, .player a) else none
  | .object _ z1 _ o1 _, .ability o2 z2 =>
    some (.object, .object [] (agreedField (· == ·) z1 z2) none (agreedField (· == ·) o1 o2) none)
  | .ability o1 z1, .object _ z2 _ o2 _ =>
    some (.object, .object [] (agreedField (· == ·) z1 z2) none (agreedField (· == ·) o1 o2) none)
  | p@(.object _ _ _ _ _), .player false => some (.join .object .player, .join p (.player false))
  | .player false, q@(.object _ _ _ _ _) => some (.join .object .player, .join q (.player false))
  | _, _ => none

def unionBinding (b c : Binding) : Option Binding :=
  if b == c then some b
  else if b.det == c.det && b.plur == c.plur
    then (unionPayload b.payload c.payload).map fun (_, pl) => ⟨b.det, b.plur, pl⟩
    else none

def unionBindings : Bindings → Bindings → Option Bindings
  | [], [] => some []
  | b :: bs, c :: cs =>
    match unionBinding b c, unionBindings bs cs with
    | some u, some us => some (u :: us)
    | _, _ => none
  | _, _ => none

def Zone.isPublic : Zone → Bool
  | .hand => false
  | .library => false
  | _ => true

def Zone.exposable : Zone → Bool
  | .hand => true
  | .library => true
  | _ => false

def Binding.isPublic (b : Binding) : Bool :=
  match b.payload with
  | .object _ (some z) _ _ _ => z.isPublic
  | .pile _ _ (some .faceDown) => false
  | .pile (some z) _ _ => z.isPublic
  | _ => true

def publicOnly (bs : Bindings) : Bindings := bs.filter (·.isPublic)

def pluralizeBinding (b : Binding) : Binding :=
  match b.det with
  | .self => b
  | _ => { b with plur := .many }

def pluralizeIntroduced (bs : Bindings) : Bindings := bs.map pluralizeBinding

def stampMoves : Option Stamp → Bool
  | none => false
  | some s => s.moved

def stampWasField : Option Stamp → Bool
  | none => false
  | some s => s.wasField

/-! ## Pronoun reach -/

def tyIs (t : CardType) (types : List CardType) : Bool := types.contains t

def isCardZone : Option Zone → Bool
  | some .graveyard | some .exile | some .hand | some .library | some .command => true
  | _ => false

def zoneIsB : Option Zone → Zone → Bool
  | none, _ => false
  | some a, b => a == b

def onFieldZone (z : Option Zone) : Bool := zoneIsB z .battlefield
def onStackZone (z : Option Zone) : Bool := zoneIsB z .stack

/-- The reaches that resolve to an object binding a move can re-stamp. -/
def Reach.tracksObject : Reach → Bool
  | .bare | .atSlot _ | .stamped _ | .tokenBorn => true
  | _ => false

def slotZoneOk : SlotCarrier → Option Zone → Bool
  | .permanent, zn => onFieldZone zn
  | .card, zn => isCardZone zn
  | .spell, zn => onStackZone zn

def mkStamp : Option Deed → Option Zone → Bool → Option Stamp
  | none, _, _ => none
  | some v, oldZn, moved => some ⟨v, onFieldZone oldZn, moved⟩

/-- A type refinement reads one object's evidence, never a different half of a join. -/
def payloadHasType (t : CardType) : Payload → Bool
  | .object ty _ _ _ _ => tyIs t ty
  | _ => false

/-- Copy origin belongs to the same object or ability that the word selects. -/
def payloadIsCopy : Payload → Bool
  | .object _ _ _ og _ | .ability og _ => isCopyOrigin og
  | _ => false

def halfWordReaches (w : NounWord) (pl : Payload) : Bool :=
  match w with
  | .ofType word t => halfWordReaches word pl && payloadHasType t pl
  | .copied word => halfWordReaches word pl && payloadIsCopy pl
  | word =>
    match pl with
    | .object ty _ _ _ _ =>
      match word with
      | .type t => tyIs t ty
      | .permanent => ty.isEmpty
      | _ => false
    | .player _ => word == .player
    | _ => false

def halfReaches (w : NounWord) : Payload → Bool
  | .join l r => halfReaches w l || halfReaches w r
  | pl => halfWordReaches w pl

/-- A word's carrier discipline and each refinement must hold on the same binding. -/
def wordReaches (w : NounWord) (b : Binding) : Bool :=
  match w with
  | .ofType word t => wordReaches word b && payloadHasType t b.payload
  | .copied word => wordReaches word b && payloadIsCopy b.payload
  | w =>
    match b.payload with
    | .object ty zn pv og _ =>
      match w with
      | .type t => onFieldZone zn && tyIs t ty
      | .card => isCardZone zn
      | .spell => onStackZone zn
      | .permanent => onFieldZone zn || stampWasField pv
      | .token => onFieldZone zn && isTokenOrigin og
      | .copy => isCopyOrigin og
      | .stack => onStackZone zn
      | _ => false
    | .player _ => w == .player
    | pl@(.join _ _) =>
      match w with
      | .type t => halfReaches (.type t) pl
      | .player => halfReaches .player pl
      | .permanent => halfReaches .permanent pl
      | .join => Kind.lte .player b.kind
      | .stack => onStackZone pl.zone
      | _ => false
    | .ability _ zn =>
      match w with
      | .ability => true
      | .stack => onStackZone zn
      | _ => false
    | .pile _ _ _ => w == .pile
    | pl =>
      match w with
      | .join => pl.joined && Kind.lte .player b.kind
      | .stack => onStackZone pl.zone
      | _ => false

/-- A self-binding is never read by a noun word. -/
def wordNow (w : NounWord) (b : Binding) : Bool :=
  match b.det with
  | .self => false
  | _ => wordReaches w b

def NounWord.kind : NounWord → Kind
  | .ofType word _ | .copied word => word.kind
  | .player => .player
  | .join => .join .object .player
  | .pile => .pile
  | _ => .object

def Reach.kind : Reach → Kind
  | .word w => w.kind
  | .unionHalf w => w.kind
  | .verbed _ w _ => w.kind
  | .thatTurn => .turnRef
  | _ => .object

def stampedBy (v : Deed) (s : Stamp) : Bool := v == s.verb
def Stamp.feature (s : Stamp) : Option DeedFeature := deedFeatureOf s.verb

def verbedWordOk : NounWord → Stamp → List CardType → Option Zone → Option Origin → Bool
  | .ofType word t, st, ty, zn, og => verbedWordOk word st ty zn og && tyIs t ty
  | .copied word, st, ty, zn, og => verbedWordOk word st ty zn og && isCopyOrigin og
  | .type t, st, ty, _, _ => st.wasField && tyIs t ty
  | .card, _, _, zn, _ => isCardZone zn
  | .spell, _, _, zn, _ => onStackZone zn
  | .permanent, st, _, _, _ => st.wasField
  | _, _, _, _, _ => false

def stampIs (v : Deed) : Option Stamp → Bool
  | none => false
  | some st => stampedBy v st

def markTy (ty : List CardType) : Binding → Binding
  | ⟨det, plur, .object old zn st og sz⟩ =>
    ⟨det, plur, .object (mergeTypes old ty) zn st og sz⟩
  | b => b

def markFirst (q : Binding → Bool) (ty : List CardType) : Bindings → Bindings
  | [] => []
  | b :: bs => if q b then markTy ty b :: bs else b :: markFirst q ty bs

def survivesShuffle (b : Binding) : Bool :=
  match b.payload with
  | .object _ (some .library) (some st) _ _ => st.feature == some .librarySearch
  | .object _ (some .library) none _ _ => false
  | _ => true

def afterShuffle (bs : Bindings) : Bindings := bs.filter survivesShuffle

def stampWordOk (v : Deed) (w : NounWord) (st : Stamp) (ty : List CardType)
    (zn : Option Zone) (og : Option Origin) : Bool :=
  stampedBy v st && verbedWordOk w st ty zn og

def reaches (r : Reach) (pl : Plurality) (b : Binding) : Bool :=
  match r with
  | .bare => Kind.lte .object b.kind && pl.isOne == b.plur.isOne
  | .atSlot sl => Kind.lte .object b.kind && pl.isOne == b.plur.isOne && slotZoneOk sl b.zone
  | .stamped v => Kind.lte .object b.kind && pl.isOne == b.plur.isOne && stampIs v b.payload.prov
  | .tokenBorn =>
    Kind.lte .object b.kind && pl.isOne == b.plur.isOne && isTokenOrigin b.payload.orig
  | .word w => pl.isOne == b.plur.isOne && wordNow w b
  | .unionHalf w => pl.isOne == b.plur.isOne && b.payload.joined && halfReaches w b.payload
  | .verbed v w _ =>
    match b.payload with
    | .object ty zn (some st) og _ =>
      pl.isOne == b.plur.isOne && stampWordOk v w st ty zn og
    | _ => false
  | .thatTurn => Kind.lte .turnRef b.kind && pl.isOne == b.plur.isOne

/-- How many antecedents a read resolves to; a read is sound at exactly one. -/
def countReach (r : Reach) (pl : Plurality) : Bindings → Nat
  | [] => 0
  | b :: bs => if reaches r pl b then countReach r pl bs + 1 else countReach r pl bs

def firstReach (r : Reach) (pl : Plurality) : Bindings → Option Binding
  | [] => none
  | b :: bs => if reaches r pl b then some b else firstReach r pl bs

def provOfReach (r : Reach) (pl : Plurality) (bs : Bindings) : Option Stamp :=
  firstReach r pl bs >>= (·.payload.prov)
def zoneOfReach (r : Reach) (pl : Plurality) (bs : Bindings) : Option Zone :=
  firstReach r pl bs >>= (·.zone)
def tyOfReach (r : Reach) (pl : Plurality) (bs : Bindings) : List CardType :=
  (firstReach r pl bs).elim [] (·.ty)
def faceOfReach (r : Reach) (pl : Plurality) (bs : Bindings) : Option PileFace :=
  firstReach r pl bs >>= (·.face)

/-- Validate an expansion's introduction pattern against the current prefix. A letter
mention is optional: it introduces a binding only if the enclosing context lacked it.
Other mentions must match in order; they never search past an unrelated binding. -/
def introductionWidth : List Kind → Bindings → Option Nat
  | [], _ => some 0
  | .letter l :: pattern, bs =>
    match bs with
    | b :: rest =>
      if b.kind == .letter l then (introductionWidth pattern rest).map (· + 1)
      else introductionWidth pattern bs
    | [] => introductionWidth pattern []
  | kind :: pattern, b :: rest =>
    if b.kind == kind then (introductionWidth pattern rest).map (· + 1) else none
  | _ :: _, [] => none

def view : Window → Bindings → Bindings
  | .whole, bs => bs
  | .top n, bs => bs.take n
  | .below n, bs => bs.drop n
  | .introduced pattern, bs => (introductionWidth pattern bs).elim [] (bs.take ·)
  | .outsideIntroduced pattern, bs => (introductionWidth pattern bs).elim [] (bs.drop ·)

def overWindow (f : Bindings → Bindings) : Window → Bindings → Bindings
  | .whole, bs => f bs
  | .top n, bs => f (bs.take n) ++ bs.drop n
  | .below n, bs => bs.take n ++ f (bs.drop n)
  | .introduced pattern, bs =>
    (introductionWidth pattern bs).elim bs fun n => f (bs.take n) ++ bs.drop n
  | .outsideIntroduced pattern, bs =>
    (introductionWidth pattern bs).elim bs fun n => bs.take n ++ f (bs.drop n)

def countTokenSpecs : Bindings → Nat
  | [] => 0
  | ⟨.self, _, _⟩ :: bs => countTokenSpecs bs
  | ⟨_, _, .object _ _ _ og _⟩ :: bs =>
    if isTokenOrigin og then countTokenSpecs bs + 1 else countTokenSpecs bs
  | _ :: bs => countTokenSpecs bs

def tyOfThoseAny (w : NounWord) : Bindings → List CardType
  | [] => []
  | b :: bs => if wordNow w b then b.ty else tyOfThoseAny w bs

def countChoosers (bs : Bindings) : Nat := countOnes .player bs + countManys .player bs

def zoneFits : Option Zone → Option Zone → Bool
  | _, none => true
  | none, some _ => true
  | subj, some b => zoneIsB subj b

/-- Which kinds "target" can precede [CR#115.1]. -/
def Kind.targetable : Kind → Bool
  | .object => true
  | .player => true
  | .join a b => Kind.targetable a && Kind.targetable b
  | _ => false

/-- Which kinds a spell or ability targets from ("a spell that targets …"). -/
def Kind.targeter : Kind → Bool
  | .object => true
  | .join a b => Kind.targeter a && Kind.targeter b
  | _ => false

/-- Which kinds a determiner phrase can be built over. -/
def Kind.phrasal : Kind → Bool
  | .object => true
  | .player => true
  | .quality _ => true
  | .join a b => Kind.phrasal a && Kind.phrasal b
  | _ => false

def CardType.damageable : CardType → Bool
  | .creature | .planeswalker | .battle => true
  | _ => false

/-- `[]` means unknown-therefore-permissive, at both levels. -/
def damageableHeadTysOk (alts : List (List CardType)) : Bool :=
  alts.all fun types => types.isEmpty || types.any CardType.damageable

/-- A copy of an ability is itself an ability [CR#707.10]: the source's payload shape, not its
kind, decides which object the copy is. -/
def copyPayloadIn : Kind → Bool → List CardType → Option Zone → Payload
  | .object, true, _, z => .ability (some .copy) z
  | .object, false, ty, z => .object ty z none (some .copy) none
  | .join l r, ab, ty, z => .join (copyPayloadIn l ab ty z) (copyPayloadIn r ab ty z)
  | .player, _, _, _ => .player false
  | .quality q, _, _, _ => .quality q
  | _, _, _, _ => .gap

def CopySort.landsIn : CopySort → Option Zone → Option Zone
  | .fromStack, _ => some .stack
  | .fromCardZone, z => z

def Kind.qualityParam : Kind → Bool
  | .object => true
  | .player => true
  | _ => false

/-- Modes are costed all together or not at all: a spell whose modes carry costs is a spree,
and every one of its modes has a cost [CR#702.172a]. -/
def modesCostedUniformly {α β : Type} (modes : List (Option α × β)) : Bool :=
  modes.all (·.1.isSome) || modes.all (·.1.isNone)

/-! ## Mana -/

def halvesDistinct : SimpleManaSymbol → Color → Bool
  | .generic _, _ => true
  | .specific .colorless, _ => true
  | .specific (.of c), d => c != d

def phyrexianDistinct : Color → Option Color → Bool
  | _, none => true
  | c, some d => c != d

def manaHasX : ManaCost → Bool
  | [] => false
  | .variable :: _ => true
  | _ :: ms => manaHasX ms

def costLetters : Option ManaCost → Bindings
  | none => []
  | some c => if manaHasX c then [letterB .x] else []

def manaRun (c : ManaCost) : Bool := !c.isEmpty

def runsNonEmpty : List ProducedRun → Bool
  | [] => true
  | [] :: _ => false
  | (_ :: _) :: rs => runsNonEmpty rs

def producedRunsWritten : List ProducedRun → Bool
  | [] => false
  | rs => runsNonEmpty rs

/-! ## Subtypes and type lines -/

def Subtype.type : Subtype → Option CardType
  | .of host _ => some host
  | .spell _ => none

def Subtype.fits : Subtype → CardType → Bool
  | .of host _, t => host == t
  | .spell _, t => t == .instant || t == .sorcery

def Subtype.label : Subtype → String
  | .of _ label => label
  | .spell label => label

def Subtype.frame (s : Subtype) : Option FrameFeature :=
  (subtypeFacts.find? (·.subtype == s)).map (·.frame)

def framesWith (f : FrameFeature) (subs : List Subtype) : Bool := subs.any (·.frame == some f)

def distinctSubtypeFacts : List SubtypeFacts → Bool
  | [] => true
  | f :: fs => !(fs.map (·.subtype)).elem f.subtype && distinctSubtypeFacts fs

def spaceHosted : SubtypeSpace → List CardType → Bool
  | .basicLand, ty => tyIs .land ty
  | .land, ty => tyIs .land ty
  | .creature, ty => tyIs .creature ty || tyIs .kindred ty

def CardType.ascribesAs : CardType → Bool
  | .kindred | .instant | .sorcery => false
  | _ => true

def MarkerWord.grantorOrigin : MarkerWord → Option Zone
  | .emblem => some .command
  | _ => none

def MarkerWord.zone : MarkerWord → Zone
  | .token => .battlefield
  | .emblem => .command
  | .spell => .stack
  | .permanent => .battlefield
  | .ability => .stack

def ascriptionOk (t : CardType) : Option Subtype → Bool
  | none => t.ascribesAs
  | some s => t.ascribesAs && s.fits t

def Delta.amount {α : Type} : Delta α → α
  | .up x => x
  | .down x => x
  | .set x => x

/-- A counter only adds to or subtracts from power and toughness [CR#122.1a]. -/
def counterShift : Delta Nat → Bool
  | .set _ => false
  | _ => true

def supersDistinct : List Supertype → Bool
  | [] => true
  | s :: ss => !ss.elem s && supersDistinct ss

/-- "Nth" counts from one [CR#401.7]: the Idris carried `IsSucc n` on the constructor. -/
def Ordinal.ok : Ordinal → Bool
  | .nth n => n != 0

def OptOrdinal.ok : Option Ordinal → Bool
  | none => true
  | some o => o.ok

/-! ## Designations

The grammar names a designation by its label (`DesignationLabel`); what the checker knows
about one lives here. The table is the open part: a keyword's expansion (Ascend's, Monstrosity's)
brings its designation with it, and the CR adds new ones without touching the grammar. -/

def findDesignation (label : DesignationLabel) : List DesignationFacts → Option DesignationFacts
  | [] => none
  | f :: fs => if f.label == label then some f else findDesignation label fs

def distinctDesignationLabels : List DesignationFacts → Bool
  | [] => true
  | f :: fs => !(fs.map (·.label)).elem f.label && distinctDesignationLabels fs

def DesignationLabel.facts (label : DesignationLabel) : Option DesignationFacts :=
  findDesignation label designationTable

def DesignationLabel.known (label : DesignationLabel) : Bool := label.facts.isSome

def DesignationLabel.scope (label : DesignationLabel) : Option DesignationScope :=
  label.facts.map (·.scope)
def DesignationLabel.checked (label : DesignationLabel) : Bool :=
  (label.facts.map (·.effectful)).getD false
def DesignationLabel.seedZone (label : DesignationLabel) : Option Zone := label.facts.bind (·.zone)
def DesignationLabel.seedType (label : DesignationLabel) : Option CardType :=
  label.facts.bind (·.type)

def DesignationLabel.holder (label : DesignationLabel) : Option Kind :=
  match label.scope with
  | some (.heldBy k) => some k
  | some .heldByCard => some .object
  | _ => none

/-- A possessive on a designation ("your Ring-bearer", "your commander") is meaningful only
where an object holds the designation for a player; a player-held or game-wide designation has
no possessor [CR#701.54e]. -/
def DesignationLabel.possessorOk (label : DesignationLabel) : Bool :=
  label.holder == some .object

def DesignationLabel.heldByItsCard (label : DesignationLabel) : Bool :=
  label.scope == some .heldByCard

def DesignationLabel.gameWide (label : DesignationLabel) : Bool :=
  label.scope == some .heldByGame

/-- Idris `DesignationHolder d z`: a player-held designation needs no zone; an object-held one
needs the holder on the battlefield. -/
def designationHolderOk (label : DesignationLabel) (z : Option Zone) : Bool :=
  match label.scope with
  | some (.heldBy .player) => true
  | some (.heldBy .object) => zoneIsB z .battlefield
  | _ => false

/-- The table's own label for a Room half, where a guard needs the label itself rather than the
half; the empty label where the table names none. -/
def RoomHalf.designation (h : RoomHalf) : DesignationLabel :=
  ((designationTable.find? (·.half == some h)).map (·.label)).getD ""

def DesignationLabel.half (label : DesignationLabel) : Option RoomHalf := label.facts >>= (·.half)

/-- A combat relation written without its counterpart ("attacking", "blocking", "blocked"):
the ones that read as a bare participle. -/
def CombatRelation.bare : CombatRelation → Bool
  | .attackerOf | .declaredAttacker | .blockerOf | .blockedBy => true
  | .attackedBy | .couldBlock | .couldBeBlockedBy => false

/-! ## Attachment, status, counters -/

def NounWord.attachHostZone : NounWord → Option Zone
  | .ofType word _ | .copied word => word.attachHostZone
  | .type _ | .card | .spell | .permanent | .token => some .battlefield
  | .copy => some .stack
  | _ => none

def NounWord.attachHostTy : NounWord → List CardType
  | .ofType word t => mergeTypes word.attachHostTy [t]
  | .copied word => word.attachHostTy
  | .type t => [t]
  | _ => []

/-- Whether a word names an ability, independently of its additional refinements. -/
def NounWord.isAbility : NounWord → Bool
  | .ability => true
  | .ofType word _ | .copied word => word.isAbility
  | _ => false

/-- Type refinements require an object with characteristics; multiple types can coexist
[CR#205.2b]. The profile retains them all. -/
def NounWord.attachRefinementsOk : NounWord → Bool
  | .ofType word _ =>
    word.attachRefinementsOk && word.kind == .object && !word.isAbility
  | .copied word => word.attachRefinementsOk && word.kind == .object
  | _ => true

def NounWord.base : NounWord → NounWord
  | .ofType word _ | .copied word => word.base
  | word => word

def attachHeadOk (a : AttachWord) (word : NounWord) : Bool :=
  word.attachRefinementsOk &&
    match a, word.base with
    | .enchanted, _ => true
    | .equipped, .type _ => word.attachHostTy.contains .creature
    | .equipped, .permanent => true
    | .fortified, .type _ => word.attachHostTy.contains .land
    | _, _ => false

def DefinedSlots.power : DefinedSlots → Bool
  | .toughnessAlone => false
  | _ => true

def DefinedSlots.toughness : DefinedSlots → Bool
  | .powerAlone => false
  | _ => true

def distinctCounterLabels : List CounterFacts → Bool
  | [] => true
  | f :: fs => !(fs.map (·.label)).elem f.label && distinctCounterLabels fs

def counterFactsFor (l : String) : Option CounterFacts := counterFacts.find? (·.label == l)
def knownCounter (l : String) : Bool := (counterFactsFor l).isSome

def CounterKind.scope : CounterKind → Kind
  | .boost _ _ => .object
  | .keyword _ => .object
  | .named l => (counterFactsFor l).elim .object (·.holder)

def ProjAxis.scope : ProjAxis → Kind
  | .stat _ => .object
  | .playerStat _ => .player
  | .counter c => c.scope
  | .anyCounter k => k

def ProjAxis.type : ProjAxis → Option CardType
  | .stat c => c.comparedType
  | _ => none

def allAxisType (t : CardType) : List ProjAxis → Bool
  | [] => true
  | a :: as =>
    match a.type with
    | none => false
    | some u => t == u && allAxisType t as

def axisTypes : List ProjAxis → Option CardType
  | [] => none
  | a :: as =>
    match a.type with
    | none => none
    | some t => if allAxisType t as then some t else none

/-- Idris `AxesAt k axes`: every axis projects the kind `k`, and there is at least one. -/
def axesAt (k : Kind) : List ProjAxis → Bool
  | [] => false
  | as => as.all (·.scope == k)

/-- Idris `CounterKindNamed k kind`: an absent kind is fine; a named one must live on `k`. -/
def counterKindNamed (k : Kind) : Option CounterKind → Bool
  | none => true
  | some c => c.scope == k

def chapterMarksDistinct : List ChapterNumber → Bool
  | [] => true
  | a :: rest => !rest.elem a && chapterMarksDistinct rest

def chapterMarksOk : List ChapterNumber → Bool
  | [] => false
  | ns => chapterMarksDistinct ns && ns.all (· != 0)

def hasAnyTypeCharacteristic (supertypes : List Supertype) (types : List CardType)
    (subtypes : List Subtype) : Bool :=
  !(supertypes.isEmpty && types.isEmpty && subtypes.isEmpty)

def subsFitLine : List Subtype → List CardType → Bool
  | [], _ => true
  | s :: ss, tys =>
    (tys.any s.fits || (s.fits .creature && tys.elem .kindred)) && subsFitLine ss tys

def CardType.permanent : CardType → Bool
  | .kindred | .instant | .sorcery => false
  | _ => true

def permanentSpellType (types : List CardType) : Bool :=
  types.any CardType.permanent && !types.contains .land &&
    !types.contains .instant && !types.contains .sorcery

def CardType.isInstantOrSorcery : CardType → Bool
  | .instant | .sorcery => true
  | _ => false

def placeableTy (types : List CardType) : Bool :=
  types.isEmpty ||
    (types.any CardType.permanent && !types.any CardType.isInstantOrSorcery)

def destTypeOk (ty : List CardType) : Zone → Bool
  | .battlefield => placeableTy ty
  | _ => true

def Status.clash : Status → Status → Bool
  | .tapped, .untapped | .untapped, .tapped | .flipped, .unflipped | .unflipped, .flipped
  | .faceUp, .faceDown | .faceDown, .faceUp | .phasedIn, .phasedOut | .phasedOut, .phasedIn => true
  | _, _ => false

/-- Which status values a description can be written with ("tapped creature"). -/
def Status.word : Status → Bool
  | .tapped | .untapped | .faceUp | .faceDown => true
  | _ => false

/-- Which status values an instruction can set directly. -/
def Status.markable : Status → Bool
  | .unflipped => false
  | _ => true

def colorsDistinct : List Color → Bool
  | [] => true
  | c :: cs => !cs.elem c && colorsDistinct cs

def ColorSpec.ok : ColorSpec → Bool
  | .some cs => colorsDistinct cs
  | .every => true

def typesDistinct : List CardType → Bool
  | [] => true
  | t :: ts => !ts.elem t && typesDistinct ts

def addedFits (subj : List CardType) (types : List CardType) (subtypes : List Subtype) : Bool :=
  subtypes.all fun s =>
    types.any s.fits || subj.any s.fits

def anyNewType (subj : List CardType) : List CardType → Bool
  | [] => false
  | t :: ts => !tyIs t subj || anyNewType subj ts

def addsSomething (subj : List CardType) (types : List CardType) (subtypes : List Subtype) :
    Bool :=
  match subtypes with
  | [] => anyNewType subj types
  | _ :: _ => true

def CardType.retainable : CardType → Bool
  | .instant | .sorcery => false
  | _ => true

def retentionOk (types : List CardType) : Option CardType → Bool
  | none => true
  | some t => match types with
    | [] => false
    | _ :: _ => t.retainable

def lastType : List CardType → Option CardType
  | [] => none
  | [t] => some t
  | _ :: ts => lastType ts

/-- A phase or step inside the turn: a turn is made of its phases [CR#500.1], so the turn is
never one of its own parts. -/
def TurnPart.proper : TurnPart → Bool
  | .turn => false
  | _ => true

end Semantics
