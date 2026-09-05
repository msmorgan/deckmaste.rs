import Semantics.Words
import Semantics.Events

/-!
# Semantics.Check.Words

The checker's vocabulary layer: everything `Words.idr` computes that is not syntax. The
antecedent stack (`Binding`, `Bindings`, `Payload`), the reads over it (`countReach`,
`countChoice`, `countOutcomes`, …), the facts tables (acts, counters, designations), and the
leaf predicates the phrase and effect rules consult.

Every function keeps its Idris name and clauses. `So (f x)` obligations are the `Bool`
functions here; the rule layer turns them into refusals. Everything is structurally recursive
so `decide` can run it.

The keyword facts table (`keywordFacts`, generated into `FactsGen.idr` by xtask) is not here
yet; the keyword predicates take the table as an argument until it is.
-/

namespace Semantics

/-! ## Card types and stats -/

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

structure DeedRole where
  kinds : List Kind
  types : List CardType
  bare : Bool
  zone : Option Zone
  deriving Repr, BEq

def noRole : DeedRole := ⟨[], [], false, none⟩

inductive PremiseSort where
  | object | mana | value
  deriving DecidableEq, Repr

/-- The deeds a core constructor must name structurally, declared so a guard reads a feature
rather than a verb's spelling. -/
inductive DeedFeature where
  | attacking | blocking | targeting | controlGrant | librarySearch | sacrificing | unlocking
  deriving DecidableEq, Repr

structure ActFacts where
  label : VerbLabel
  participle : Option String := none
  dest : Option Zone := none
  stepwise : Bool := false
  loci : List Zone := []
  intransitive : Bool := false
  agentRole : DeedRole := noRole
  patientRole : DeedRole := noRole
  feature : Option DeedFeature := none
  abilityRole : Option Role := none
  counterfactual : Option PremiseSort := none
  rides : Bool := false
  plays : Bool := false
  bounded : Bool := false
  /-- The deed opens an opponent's library ("fateseal" [CR#701.29a]); the same look over one's
  own library is a different deed. -/
  opponentsLibrary : Bool := false
  deriving Repr, BEq

private def playerAgent : DeedRole := ⟨[.player], [], true, none⟩
private def fieldObject : DeedRole := ⟨[.object], [], false, some .battlefield⟩
private def permanentTypes : List CardType :=
  [.creature, .artifact, .land, .enchantment, .planeswalker, .battle]
private def spellTypes : List CardType :=
  [.creature, .artifact, .enchantment, .instant, .sorcery, .planeswalker, .battle, .kindred]
private def allTypes : List CardType :=
  [.creature, .artifact, .land, .enchantment, .instant, .sorcery, .planeswalker, .battle, .kindred]

/-- Keyword actions [CR#701]: one-shot verbs in effect position that confer nothing, unlike the
keyword abilities of the keyword facts table. -/
def actFacts : List ActFacts :=
  [ { label := "Destroy", participle := some "destroyed", dest := some .graveyard,
      agentRole := playerAgent, patientRole := fieldObject },
    { label := "Sacrifice", participle := some "sacrificed", dest := some .graveyard,
      agentRole := playerAgent, patientRole := ⟨[.object], permanentTypes, true, some .battlefield⟩,
      feature := some .sacrificing, bounded := true },
    { label := "Exile", participle := some "exiled", dest := some .exile, agentRole := playerAgent,
      patientRole := ⟨[.object], [], false, none⟩ },
    { label := "Discard", participle := some "discarded", dest := some .graveyard,
      agentRole := playerAgent, patientRole := ⟨[.object], [], false, some .hand⟩ },
    { label := "Mill", participle := some "milled", dest := some .graveyard,
      agentRole := playerAgent, patientRole := ⟨[.object], [], false, some .library⟩ },
    { label := "Scry", stepwise := true, agentRole := playerAgent },
    { label := "Surveil", stepwise := true, agentRole := playerAgent },
    { label := "Tap", participle := some "tapped", agentRole := playerAgent,
      patientRole := fieldObject },
    { label := "Untap", participle := some "untapped", agentRole := playerAgent,
      patientRole := ⟨[.object], permanentTypes, true, some .battlefield⟩, bounded := true },
    { label := "Return", agentRole := playerAgent, patientRole := ⟨[.object], [], false, none⟩ },
    { label := "GainControl", agentRole := playerAgent, patientRole := fieldObject,
      feature := some .controlGrant },
    { label := "Put", agentRole := playerAgent, patientRole := ⟨[.object], [], false, none⟩ },
    { label := "Search",
      loci := [.battlefield, .graveyard, .exile, .hand, .library, .stack, .command],
      agentRole := playerAgent, feature := some .librarySearch, bounded := true },
    { label := "Shuffle", loci := [.library], agentRole := playerAgent },
    { label := "Proliferate", agentRole := playerAgent },
    { label := "The Ring Tempts You" },
    { label := "Transform", intransitive := true, agentRole := playerAgent,
      patientRole := fieldObject },
    { label := "Convert", intransitive := true, agentRole := playerAgent,
      patientRole := fieldObject },
    { label := "Meld", dest := some .battlefield, patientRole := ⟨[.object], [], false, none⟩ },
    { label := "Unlock", agentRole := playerAgent, patientRole := ⟨[], [], false, some .battlefield⟩,
      feature := some .unlocking },
    { label := "Fully Unlock", agentRole := playerAgent, patientRole := fieldObject },
    /- The declaring side of an attack is the active player or a creature they control
    [CR#508.1,508.1a]. -/
    { label := "Attack", agentRole := ⟨[.object, .player], [.creature], true, some .battlefield⟩,
      patientRole := ⟨[.object], [.planeswalker, .battle], false, some .battlefield⟩,
      feature := some .attacking, counterfactual := some .object, bounded := true },
    { label := "Block", agentRole := ⟨[.object], [.creature], true, some .battlefield⟩,
      patientRole := ⟨[.object], [.creature], false, some .battlefield⟩,
      feature := some .blocking, counterfactual := some .object, bounded := true },
    { label := "Target", agentRole := ⟨[], [], true, some .stack⟩,
      patientRole := ⟨[.object, .player], allTypes, true, none⟩,
      feature := some .targeting, counterfactual := some .object, bounded := true },
    { label := "Cast", agentRole := playerAgent,
      patientRole := ⟨[.object], spellTypes, true, some .stack⟩,
      counterfactual := some .object, rides := true, plays := true, bounded := true },
    { label := "Play", agentRole := playerAgent, patientRole := ⟨[.object], allTypes, true, none⟩,
      counterfactual := some .object, rides := true, plays := true, bounded := true },
    { label := "Counter", agentRole := ⟨[], [], true, some .stack⟩,
      patientRole := ⟨[.object], spellTypes, true, some .stack⟩, rides := true },
    { label := "Copy", agentRole := ⟨[], [], true, some .stack⟩,
      patientRole := ⟨[.object], spellTypes, true, some .stack⟩, bounded := true },
    { label := "Activate", agentRole := playerAgent, patientRole := ⟨[.object], [], true, some .stack⟩,
      abilityRole := some .patient, bounded := true },
    { label := "Regenerate", participle := some "regenerated", agentRole := ⟨[], [], true, none⟩,
      patientRole := ⟨[.object], permanentTypes, true, some .battlefield⟩, rides := true,
      bounded := true },
    { label := "GainLife", agentRole := playerAgent, bounded := true },
    { label := "Draw", agentRole := playerAgent, patientRole := ⟨[.object], [], true, some .library⟩,
      bounded := true },
    { label := "Trigger", agentRole := ⟨[.object], [], true, some .stack⟩,
      abilityRole := some .agent, bounded := true },
    { label := "LoseGame", agentRole := playerAgent },
    { label := "WinGame", agentRole := playerAgent },
    { label := "Spend", agentRole := playerAgent, counterfactual := some .mana, bounded := true },
    { label := "Crew", agentRole := ⟨[.object], [.creature], true, some .battlefield⟩,
      patientRole := ⟨[.object], [.artifact], true, some .battlefield⟩,
      counterfactual := some .value, bounded := true },
    { label := "Saddle", agentRole := ⟨[.object], [.creature], true, some .battlefield⟩,
      patientRole := ⟨[.object], permanentTypes, true, some .battlefield⟩,
      counterfactual := some .value, bounded := true },
    { label := "Vote", agentRole := playerAgent, bounded := true },
    { label := "Venture Into The Dungeon" },
    { label := "Abandon" },
    { label := "Adapt" },
    { label := "Airbend", dest := some .exile, agentRole := playerAgent },
    { label := "Amass", agentRole := playerAgent },
    { label := "Assemble" },
    { label := "Attach", agentRole := playerAgent },
    { label := "Behold", agentRole := playerAgent },
    { label := "Blight", agentRole := playerAgent },
    { label := "Bolster" },
    { label := "Clash", agentRole := playerAgent },
    { label := "Cloak", dest := some .battlefield, agentRole := playerAgent },
    { label := "Collect Evidence", dest := some .exile, agentRole := playerAgent },
    { label := "Connive" },
    { label := "Create", dest := some .battlefield, agentRole := playerAgent },
    { label := "Detain" },
    { label := "Discover", agentRole := playerAgent },
    { label := "Double" },
    { label := "Earthbend", agentRole := playerAgent },
    { label := "Endure" },
    { label := "Exchange", agentRole := playerAgent },
    { label := "Exert", agentRole := playerAgent },
    { label := "Explore" },
    { label := "Face A Villainous Choice", agentRole := playerAgent },
    { label := "Fateseal", stepwise := true, agentRole := playerAgent, opponentsLibrary := true },
    { label := "Fight" },
    { label := "Forage", agentRole := playerAgent },
    { label := "Goad", agentRole := playerAgent },
    { label := "Harness" },
    { label := "Heal" },
    { label := "Incubate", dest := some .battlefield, agentRole := playerAgent },
    { label := "Investigate", dest := some .battlefield, agentRole := playerAgent },
    { label := "Learn" },
    { label := "Manifest", dest := some .battlefield, agentRole := playerAgent },
    { label := "Manifest Dread", agentRole := playerAgent },
    { label := "Monstrosity" },
    { label := "Open An Attraction" },
    { label := "Planeswalk", agentRole := playerAgent },
    { label := "Populate" },
    { label := "Recruit", agentRole := playerAgent },
    { label := "Reveal", agentRole := playerAgent },
    { label := "Roll To Visit Your Attractions" },
    { label := "Set In Motion" },
    { label := "Support" },
    { label := "Suspect", agentRole := playerAgent },
    { label := "Time Travel" },
    { label := "Triple" },
    { label := "Phase In", intransitive := true, agentRole := ⟨[.object], [], true, some .battlefield⟩ },
    { label := "Waterbend", agentRole := playerAgent } ]

def distinctActLabels : List ActFacts → Bool
  | [] => true
  | f :: fs => !(fs.map (·.label)).elem f.label && distinctActLabels fs

def actFactsFor (v : VerbLabel) : Option ActFacts := actFacts.find? (·.label == v)

def knownAct (v : VerbLabel) : Bool := (actFactsFor v).isSome
def deedFeatureOf (v : VerbLabel) : Option DeedFeature := actFactsFor v >>= (·.feature)
def featureLabel (f : DeedFeature) : Option VerbLabel :=
  (actFacts.find? (·.feature == some f)).map (·.label)
/-- The table's own label for a structurally named deed, where a guard needs the label itself
rather than the feature; the empty label where the table names none. -/
def deedLabel (f : DeedFeature) : VerbLabel := (featureLabel f).getD ""
def participleOf (v : VerbLabel) : Option String := actFactsFor v >>= (·.participle)
def actPatientKindsOf (v : VerbLabel) : List Kind := (actFactsFor v).elim [] (·.patientRole.kinds)
def actNamesPatient (v : VerbLabel) : Bool := !(actPatientKindsOf v).isEmpty
def actZoneOf (v : VerbLabel) : Option Zone := actFactsFor v >>= (·.patientRole.zone)
def actDestOf (v : VerbLabel) : Option Zone := actFactsFor v >>= (·.dest)
def actLociOf (v : VerbLabel) : List Zone := (actFactsFor v).elim [] (·.loci)
def actNamesLocus (v : VerbLabel) : Bool := !(actLociOf v).isEmpty
def actStepwiseOf (v : VerbLabel) : Bool := (actFactsFor v).elim false (·.stepwise)
def actOpponentsLibrary (v : VerbLabel) : Bool := (actFactsFor v).elim false (·.opponentsLibrary)
def actIntransitiveOf (v : VerbLabel) : Bool := (actFactsFor v).elim false (·.intransitive)
def actNamesParticiple (v : VerbLabel) : Bool := (participleOf v).isSome

/-! ## The antecedent stack -/

/-- Provenance: the verb that last moved a binding, whether it was on the battlefield then,
and whether it changed zones. -/
structure Stamp where
  verb : VerbLabel
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
  | object (ty : Option CardType) (zone : Option Zone) (prov : Option Stamp) (orig : Option Origin)
      (size : Option Nat)
  | player (chosen : Bool)
  | quality (q : QualitySort)
  | outcome (sort : OutcomeSort)
  | gap
  | letter (l : Letter)
  | turnRef
  /-- An ability on the stack: an object with no printed characteristics and no zone of its
  own [CR#109.1,113.1c,405.1]. -/
  | ability (orig : Option Origin)
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
  | .ability _ => .object
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
  | .ability _ => some .stack
  | .pile zn _ _ => zn
  | .join l r => (l.zone).elim r.zone some
  | _ => none

def joinSeed : Option CardType → Option CardType → Option CardType
  | none, t => t
  | t, none => t
  | some t, some u => if t == u then some t else none

def ty : Payload → Option CardType
  | .object t _ _ _ _ => t
  | .join l r => joinSeed l.ty r.ty
  | _ => none

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
  | .ability og => og
  | .join l r => (l.orig).elim r.orig some
  | _ => none

def joined : Payload → Bool
  | .join _ _ => true
  | _ => false

end Payload

def Binding.zone (b : Binding) : Option Zone := b.payload.zone
def Binding.ty (b : Binding) : Option CardType := b.payload.ty
def Binding.size (b : Binding) : Option Nat := b.payload.size
def Binding.face (b : Binding) : Option PileFace := b.payload.face

/-- Re-size an object or pile binding; any other binding is returned unchanged. -/
def sized (sz : Option Nat) : Binding → Binding
  | ⟨det, pl, .object ty zn pv og _⟩ => ⟨det, pl, .object ty zn pv og sz⟩
  | ⟨det, pl, .pile zn _ fc⟩ => ⟨det, pl, .pile zn sz fc⟩
  | b => b

/-- The head type(s) a phrase seeds, one per join half. -/
inductive HeadTy where
  | sole (ty : Option CardType)
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

def choiceDeltaAt (k : Kind) : List Binding :=
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

def letterDelta (l : Letter) (bs : Bindings) : List Binding :=
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
def tyOfGroup (k : Kind) (bs : Bindings) : Option CardType := restSource k bs >>= (·.ty)
def provOfGroup (k : Kind) (bs : Bindings) : Option Stamp := restSource k bs >>= (·.payload.prov)

def anyTargeted (k : Kind) : Bindings → Bool
  | [] => false
  | b@⟨.target, _, _⟩ :: bs => Kind.lte k b.kind || anyTargeted k bs
  | _ :: bs => anyTargeted k bs

def anchorTyOk (t : CardType) : Option CardType → Bool
  | none => true
  | some t' => t == t'

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
    some (.object, .object (agreedField (· == ·) t1 t2) (agreedField (· == ·) z1 z2)
      (agreedField (· == ·) v1 v2) (agreedField (· == ·) o1 o2) (agreedField (· == ·) s1 s2))
  | .ability o1, .ability o2 => some (.object, .ability (agreedField (· == ·) o1 o2))
  | .player a, .player b => if a == b then some (.player, .player a) else none
  | .object _ _ _ o1 _, .ability o2 =>
    some (.object, .object none (some .stack) none (agreedField (· == ·) o1 o2) none)
  | .ability o1, .object _ _ _ o2 _ =>
    some (.object, .object none (some .stack) none (agreedField (· == ·) o1 o2) none)
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

def pluralizeDelta (bs : Bindings) : Bindings := bs.map pluralizeBinding

def stampMoves : Option Stamp → Bool
  | none => false
  | some s => s.moved

def stampWasField : Option Stamp → Bool
  | none => false
  | some s => s.wasField

/-! ## Pronoun reach -/

def tyIs (t : CardType) : Option CardType → Bool
  | none => false
  | some t' => t == t'

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

def mkStamp : Option VerbLabel → Option Zone → Bool → Option Stamp
  | none, _, _ => none
  | some v, oldZn, moved => some ⟨v, onFieldZone oldZn, moved⟩

def halfReaches (w : NounWord) : Payload → Bool
  | .join l r => halfReaches w l || halfReaches w r
  | .object ty _ _ _ _ =>
    match w with
    | .type t => tyIs t ty
    | .permanent => ty.isNone
    | _ => false
  | .player _ => w == .player
  | _ => false

/-- Which bindings a noun word reads. The Idris clauses match on `(word, payload)` pairs; here
the payload is matched first and the word inside, clause for clause. -/
def wordReaches (w : NounWord) (b : Binding) : Bool :=
  match b.payload with
  | .object ty zn pv og _ =>
    match w with
    | .type t => onFieldZone zn && tyIs t ty
    | .card => isCardZone zn
    | .typedCard t => isCardZone zn && tyIs t ty
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
  | .ability og =>
    match w with
    | .ability => true
    | .abilityCopy => isCopyOrigin og
    | .stack => true
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

def stampedBy (v : VerbLabel) (s : Stamp) : Bool := v == s.verb
def Stamp.feature (s : Stamp) : Option DeedFeature := deedFeatureOf s.verb

def verbedWordOk : NounWord → Stamp → Option CardType → Option Zone → Bool
  | .type t, st, ty, _ => st.wasField && tyIs t ty
  | .card, _, _, zn => isCardZone zn
  | .typedCard t, _, ty, zn => isCardZone zn && tyIs t ty
  | .spell, _, _, zn => onStackZone zn
  | .permanent, st, _, _ => st.wasField
  | _, _, _, _ => false

def stampIs (v : VerbLabel) : Option Stamp → Bool
  | none => false
  | some st => stampedBy v st

def markTy (ty : Option CardType) : Binding → Binding
  | ⟨det, plur, .object none zn st og sz⟩ => ⟨det, plur, .object ty zn st og sz⟩
  | b => b

def markFirst (q : Binding → Bool) (ty : Option CardType) : Bindings → Bindings
  | [] => []
  | b :: bs => if q b then markTy ty b :: bs else b :: markFirst q ty bs

def survivesShuffle (b : Binding) : Bool :=
  match b.payload with
  | .object _ (some .library) (some st) _ _ => st.feature == some .librarySearch
  | .object _ (some .library) none _ _ => false
  | _ => true

def afterShuffle (bs : Bindings) : Bindings := bs.filter survivesShuffle

def stampWordOk (v : VerbLabel) (w : NounWord) (st : Stamp) (ty : Option CardType)
    (zn : Option Zone) : Bool :=
  stampedBy v st && verbedWordOk w st ty zn

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
    | .object ty zn (some st) _ _ => pl.isOne == b.plur.isOne && stampWordOk v w st ty zn
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
def tyOfReach (r : Reach) (pl : Plurality) (bs : Bindings) : Option CardType :=
  firstReach r pl bs >>= (·.ty)
def faceOfReach (r : Reach) (pl : Plurality) (bs : Bindings) : Option PileFace :=
  firstReach r pl bs >>= (·.face)

def view : Window → Bindings → Bindings
  | .whole, bs => bs
  | .top n, bs => bs.take n
  | .below n, bs => bs.drop n

def overWindow (f : Bindings → Bindings) : Window → Bindings → Bindings
  | .whole, bs => f bs
  | .top n, bs => f (bs.take n) ++ bs.drop n
  | .below n, bs => bs.take n ++ f (bs.drop n)

def countTokenSpecs : Bindings → Nat
  | [] => 0
  | ⟨.self, _, _⟩ :: bs => countTokenSpecs bs
  | ⟨_, _, .object _ _ _ og _⟩ :: bs =>
    if isTokenOrigin og then countTokenSpecs bs + 1 else countTokenSpecs bs
  | _ :: bs => countTokenSpecs bs

def tyOfThoseAny (w : NounWord) : Bindings → Option CardType
  | [] => none
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
def damageableHeadTysOk (alts : List (List CardType)) : Bool := alts.all (·.all CardType.damageable)

/-- A copy of an ability is itself an ability [CR#707.10]: the source's payload shape, not its
kind, decides which object the copy is. -/
def copyPayloadIn : Kind → Bool → Option CardType → Option Zone → Payload
  | .object, true, _, _ => .ability (some .copy)
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

def spaceHosted : SubtypeSpace → Option CardType → Bool
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

inductive DesignationScope where
  | heldBy (holder : Kind)
  | heldByCard
  | heldByGame
  deriving DecidableEq, Repr

structure DesignationFacts where
  label : DesignationLabel
  scope : DesignationScope
  /-- Whether an instruction may confer it directly ("becomes the monarch"); a designation that
  only a rule confers (the commander) is not conferred by text. -/
  effectful : Bool
  zone : Option Zone
  type : Option CardType
  /-- The keyword whose expansion confers it, if a keyword owns it. -/
  keyword : Option KeywordLabel
  deriving Repr, BEq

def playerHeld (label : String) (keyword : Option KeywordLabel := none) : DesignationFacts :=
  ⟨label, .heldBy .player, true, none, none, keyword⟩

def creatureHeld (label : String) (keyword : Option KeywordLabel := none) : DesignationFacts :=
  ⟨label, .heldBy .object, true, some .battlefield, some .creature, keyword⟩

def permanentHeld (label : String) (keyword : Option KeywordLabel := none) : DesignationFacts :=
  ⟨label, .heldBy .object, true, some .battlefield, none, keyword⟩

/-- The designations the CR defines today: those of [CR#701.15b,701.37b,701.54b,701.60b,701.64b], the keyword ones of [CR#702.112b,702.131c,702.158b,702.171b,702.195b], levels [CR#716.2b], solved [CR#719.3b], the monarch and the initiative [CR#725.1,726.1], day and night [CR#731.1]. -/
def designationTable : List DesignationFacts := [
  playerHeld "the monarch", playerHeld "the initiative",
  playerHeld "the city's blessing" (some "Ascend"), playerHeld "an enduring story",
  creatureHeld "goaded", creatureHeld "Ring-bearer", creatureHeld "monstrous" (some "Monstrosity"),
  creatureHeld "renowned" (some "Renown"), creatureHeld "suspected", creatureHeld "prepared",
  creatureHeld "Alpha sector", creatureHeld "Beta sector", creatureHeld "Gamma sector",
  permanentHeld "saddled" (some "Saddle"), permanentHeld "harnessed", permanentHeld "level",
  permanentHeld "solved", permanentHeld "left half unlocked", permanentHeld "right half unlocked",
  ⟨"commander", .heldByCard, false, none, none, none⟩,
  ⟨"day", .heldByGame, true, none, none, none⟩,
  ⟨"night", .heldByGame, true, none, none, none⟩ ]

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

def RoomHalf.designation : RoomHalf → DesignationLabel
  | .left => "left half unlocked"
  | .right => "right half unlocked"

def DesignationLabel.half (label : DesignationLabel) : Option RoomHalf :=
  if label == RoomHalf.left.designation then some .left
  else if label == RoomHalf.right.designation then some .right
  else none

/-- Idris `GivingWarrant d`: instructed conferral needs an effectful designation; conferral by
a keyword must be by the keyword that owns that designation. -/
def conferralOk (label : DesignationLabel) : Conferral → Bool
  | .instructed => label.checked
  | .byKeyword keyword => label.facts.bind (·.keyword) == some keyword

/-- A combat relation written without its counterpart ("attacking", "blocking", "blocked"):
the ones that read as a bare participle. -/
def CombatRelation.bare : CombatRelation → Bool
  | .attackerOf | .declaredAttacker | .blockerOf | .blockedBy => true
  | .attackedBy | .couldBlock | .couldBeBlockedBy => false

/-! ## Attachment, status, counters -/

def attachHeadOk : AttachWord → NounWord → Bool
  | .enchanted, _ => true
  | .equipped, .type .creature => true
  | .equipped, .permanent => true
  | .fortified, .type .land => true
  | _, _ => false

def NounWord.attachHostZone : NounWord → Option Zone
  | .type _ | .card | .typedCard _ | .spell | .permanent | .token => some .battlefield
  | .copy => some .stack
  | _ => none

def NounWord.attachHostTy : NounWord → Option CardType
  | .type t => some t
  | .typedCard t => some t
  | _ => none

def DefinedSlots.power : DefinedSlots → Bool
  | .toughnessAlone => false
  | _ => true

def DefinedSlots.toughness : DefinedSlots → Bool
  | .powerAlone => false
  | _ => true

structure CounterFacts where
  label : String
  /-- What the counter is placed on: an object or a player [CR#122.1]. -/
  holder : Kind
  deriving Repr, BEq

def counterFacts : List CounterFacts :=
  [ ⟨"Charge", .object⟩, ⟨"Time", .object⟩, ⟨"Lore", .object⟩, ⟨"Poison", .player⟩,
    ⟨"Age", .object⟩, ⟨"Stun", .object⟩, ⟨"Energy", .player⟩, ⟨"Oil", .object⟩,
    ⟨"Loyalty", .object⟩, ⟨"Quest", .object⟩, ⟨"Finality", .object⟩, ⟨"Shield", .object⟩,
    ⟨"Storage", .object⟩, ⟨"Fade", .object⟩, ⟨"Spore", .object⟩, ⟨"Experience", .player⟩,
    ⟨"Depletion", .object⟩, ⟨"Level", .object⟩, ⟨"Verse", .object⟩, ⟨"Rad", .player⟩,
    ⟨"Ki", .object⟩, ⟨"Divinity", .object⟩, ⟨"Study", .object⟩, ⟨"Plan", .object⟩,
    ⟨"Ice", .object⟩, ⟨"Doom", .object⟩, ⟨"Tide", .object⟩, ⟨"Soul", .object⟩,
    ⟨"Page", .object⟩, ⟨"Fuse", .object⟩, ⟨"Flood", .object⟩, ⟨"Bounty", .object⟩,
    ⟨"Strike", .object⟩, ⟨"Hour", .object⟩, ⟨"Growth", .object⟩, ⟨"Egg", .object⟩,
    ⟨"Dream", .object⟩, ⟨"Brick", .object⟩, ⟨"Blood", .object⟩, ⟨"Omen", .object⟩,
    ⟨"Luck", .object⟩, ⟨"Plague", .object⟩, ⟨"Slime", .object⟩, ⟨"Fungus", .object⟩,
    ⟨"Wish", .object⟩, ⟨"Wind", .object⟩, ⟨"Stash", .object⟩, ⟨"Scream", .object⟩,
    ⟨"Defense", .object⟩, ⟨"Hone", .object⟩, ⟨"Sleight", .object⟩, ⟨"Spite", .object⟩,
    ⟨"Rev", .object⟩, ⟨"Intervention", .object⟩, ⟨"Suspect", .object⟩, ⟨"Bloodstain", .object⟩ ]

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

def typeLineNonEmpty (supertypes : List Supertype) (types : List CardType)
    (subtypes : List Subtype) : Bool :=
  !(supertypes.isEmpty && types.isEmpty && subtypes.isEmpty)

def subsFitLine : List Subtype → List CardType → Bool
  | [], _ => true
  | s :: ss, tys =>
    (tys.any s.fits || (s.fits .creature && tys.elem .kindred)) && subsFitLine ss tys

def CardType.permanent : CardType → Bool
  | .kindred | .instant | .sorcery => false
  | _ => true

def permanentSpellType : Option CardType → Bool
  | none => false
  | some .land => false
  | some t => t.permanent

def CardType.isSpell : CardType → Bool
  | .instant | .sorcery => true
  | _ => false

def placeableTy : Option CardType → Bool
  | none => true
  | some t => t.permanent

def destTypeOk (ty : Option CardType) : Zone → Bool
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

def addedFits (subj : Option CardType) (types : List CardType) (subtypes : List Subtype) : Bool :=
  subtypes.all fun s =>
    types.any s.fits || (match subj with | none => false | some t => s.fits t)

def anyNewType (subj : Option CardType) : List CardType → Bool
  | [] => false
  | t :: ts => !tyIs t subj || anyNewType subj ts

def addsSomething (subj : Option CardType) (types : List CardType) (subtypes : List Subtype) :
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
