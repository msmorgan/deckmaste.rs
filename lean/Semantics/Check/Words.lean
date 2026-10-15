import Semantics.Words
import Semantics.Check.Facts

/-!
# Semantics.Check.Words

The checker's vocabulary and Discourse Representation Theory (`DRS`) layer (`semantics-v3.md`
§1.2–§1.5). Holds the structured `DRS` state (`ReferentId`, `GramRole`, `Payload`, `Referent`,
`OwnScopeFrame`, `DRS`), the two-stage anaphor resolver (`J1` accessibility, `J2` clause-mate
exclusion, `D′` bare pronoun resolution, `AC>R1` demonstrative resolution, Principle A reflexives,
and unique stamped participles), the Reference Scope frame stack (`enterOwnScope`, `enterCaller`,
`leaveCaller`, `leaveOwnScope`), the facts tables (`coreDeedFacts`, `abilityDeedFacts`, `actFacts`),
and the leaf predicates the phrase and ability layers consult.
-/

namespace Semantics

/-! ## Card types and stats -/

/-- Canonical order for remembered type facts; order in the written modifiers is immaterial. -/
def normalizeTypes (types : List CardType) : List CardType :=
  [ .creature, .artifact, .land, .enchantment, .instant, .sorcery, .planeswalker, .battle,
    .kindred, .dungeon ].filter (· ∈ types)

def mergeTypes (left right : List CardType) : List CardType :=
  normalizeTypes (left ++ right)

def commonTypes (left right : List CardType) : List CardType :=
  normalizeTypes (left.filter (· ∈ right))

/-- Each characteristic stat's required card type, if any [CR#208.3,306.5,310.4,310.4a]. -/
def Stat.comparedType : Stat → Option CardType
  | .power => some .creature
  | .toughness => some .creature
  | .manaValue => none
  | .loyalty => some .planeswalker
  | .defense => some .battle

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

/-- Join subtyping on the left, then on the right, then equality. -/
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

/-- Which outcomes can be directly compared as a game value [CR#120.3d,706.2]. -/
def OutcomeSort.comparable : OutcomeSort → Bool
  | .rollResult | .damageMarked => true
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

/-- The Entity domain a referent is drawn from: a player, an object, or either [CR#102.1,109.1]. -/
inductive EntityDomain where
  | player | object | either
  deriving DecidableEq, Repr

/-- An object class [CR#109.1]: the overlapping ways an object is classified. -/
inductive ObjectClass where
  | card | token | spell | permanent | emblem | ability
  deriving DecidableEq, Repr

/-- What a referent must be: an Entity domain, optionally narrowed by object classes and card
types. Only an object has a class. Empty `classes` admit no ability; empty `types` admit only a
noun that names no type (`deedAltOk`), so a row that takes any typed noun lists `allTypes`. -/
structure ReferentSort where
  domain : EntityDomain
  classes : List ObjectClass := []
  types : List CardType := []
  deriving Repr, BEq

/-- Which kinds a domain admits: `.either` admits both, the others admit only their own. -/
def EntityDomain.admits : EntityDomain → Kind → Bool
  | .player, .player => true
  | .object, .object => true
  | .either, .object => true
  | .either, .player => true
  | _, _ => false

/-- A domain's admitted kinds, listed. -/
def EntityDomain.kinds : EntityDomain → List Kind
  | .player => [.player]
  | .object => [.object]
  | .either => [.object, .player]

structure DeedRole where
  sort : Option ReferentSort
  bare : Bool
  zone : Option Zone
  deriving Repr, BEq

def noRole : DeedRole := ⟨none, false, none⟩

/-- The card types a role's sort names; an absent sort names none. -/
def DeedRole.types (dr : DeedRole) : List CardType := dr.sort.elim [] (·.types)

inductive PremiseSort where
  | object | mana | value
  deriving DecidableEq, Repr

/-- The keyword actions [CR#701.1] a core constructor must name, declared so a guard reads a
feature rather than a verb's spelling. -/
inductive DeedFeature where
  | librarySearch | sacrificing | tapping
  deriving DecidableEq, Repr

/-- What the checker knows about one deed. -/
structure ActFacts where
  participle : Option String := none
  dest : Option Zone := none
  loci : List Zone := []
  intransitive : Bool := false
  agentRole : DeedRole := noRole
  patientRole : DeedRole := noRole
  hostRole : DeedRole := noRole
  feature : Option DeedFeature := none
  ownPatient : Bool := false
  deriving Repr, BEq

def permanentTypes : List CardType :=
  [.creature, .artifact, .land, .enchantment, .planeswalker, .battle]
def playerAgent : DeedRole := ⟨some ⟨.player, [], []⟩, true, none⟩
def permanentAgent : DeedRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩
def fieldObject : DeedRole := ⟨some ⟨.object, [], []⟩, false, some .battlefield⟩
def spellTypes : List CardType :=
  [.creature, .artifact, .enchantment, .instant, .sorcery, .planeswalker, .battle, .kindred]
def allTypes : List CardType :=
  [.creature, .artifact, .land, .enchantment, .instant, .sorcery, .planeswalker, .battle, .kindred,
    .dungeon]

/-- The deeds the core rules define, as a total function [CR#508.1,509.1,709.5f,106.12a]. -/
def coreDeedFacts : CoreDeed → ActFacts
  | .attack =>
      { agentRole := ⟨some ⟨.either, [], [.creature]⟩, true, some .battlefield⟩,
        patientRole := ⟨some ⟨.object, [], [.planeswalker, .battle]⟩, false, some .battlefield⟩ }
  | .block =>
      { agentRole := ⟨some ⟨.object, [], [.creature]⟩, true, some .battlefield⟩,
        patientRole := ⟨some ⟨.object, [], [.creature]⟩, false, some .battlefield⟩ }
  | .target =>
      { agentRole := ⟨none, true, some .stack⟩,
        patientRole := ⟨some ⟨.either, [], allTypes⟩, true, none⟩ }
  | .copy =>
      { agentRole := ⟨none, true, some .stack⟩,
        patientRole := ⟨some ⟨.object, [.spell, .ability], spellTypes⟩, true, some .stack⟩ }
  | .draw =>
      { agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], []⟩, true, some .library⟩ }
  | .gainLife => { agentRole := playerAgent }
  | .loseGame => { agentRole := playerAgent }
  | .winGame => { agentRole := playerAgent }
  | .spend => { agentRole := playerAgent }
  | .trigger =>
      { agentRole := ⟨some ⟨.object, [.ability], []⟩, true, some .stack⟩ }
  | .put =>
      { agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], []⟩, false, none⟩ }
  | .return_ =>
      { agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], []⟩, false, none⟩ }
  | .gainControl => { agentRole := playerAgent, patientRole := fieldObject }
  | .unlock => { agentRole := playerAgent, patientRole := ⟨none, false, some .battlefield⟩ }
  | .fullyUnlock => { agentRole := playerAgent, patientRole := fieldObject }
  | .tapForMana => { agentRole := playerAgent, patientRole := fieldObject }

/-- The deeds a keyword ability defines for itself [CR#702.122b,702.171a,702.26a]. -/
def abilityDeedFacts : List (KeywordLabel × ActFacts) :=
  [ ((.named "crew"),
      { agentRole := ⟨some ⟨.object, [], [.creature]⟩, true, some .battlefield⟩,
        patientRole := ⟨some ⟨.object, [], [.artifact]⟩, true, some .battlefield⟩ }),
    ((.named "saddle"),
      { agentRole := ⟨some ⟨.object, [], [.creature]⟩, true, some .battlefield⟩,
        patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩ }),
    ((.named "phasing"),
      { intransitive := true,
        agentRole := ⟨some ⟨.object, [], []⟩, true, some .battlefield⟩ }),
    ((.named "mutate"),
      { intransitive := true,
        agentRole := ⟨some ⟨.object, [], [.creature]⟩, true, some .battlefield⟩ }),
    ((.named "devour"),
      { agentRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩,
        patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, none⟩ }) ]

/-- What the checker knows about each keyword action [CR#701.1]. -/
def actFacts : List (KeywordActionLabel × ActFacts) :=
  [ ("activate",
      { agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [.ability], []⟩, true, some .stack⟩ }),
    ("adapt", { agentRole := permanentAgent }),
    ("airbend", { dest := some .exile, agentRole := playerAgent }),
    ("amass", { agentRole := playerAgent }),
    ("assemble", {}),
    ("attach",
      { agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], [.artifact, .enchantment]⟩, true, none⟩,
        hostRole := ⟨some ⟨.either, [], []⟩, false, none⟩ }),
    ("behold", { agentRole := playerAgent }),
    ("blight", { agentRole := playerAgent }),
    ("bolster", { agentRole := playerAgent }),
    ("cast",
      { agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], spellTypes⟩, true, some .stack⟩ }),
    ("clash", { agentRole := playerAgent }),
    ("cloak", { dest := some .battlefield, agentRole := playerAgent }),
    ("collectEvidence", { dest := some .exile, agentRole := playerAgent }),
    ("connive", { agentRole := permanentAgent }),
    ("convert", { intransitive := true, agentRole := playerAgent, patientRole := fieldObject }),
    ("counter",
      { agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [.spell, .ability], spellTypes⟩, true, some .stack⟩ }),
    ("create", { dest := some .battlefield, agentRole := playerAgent }),
    ("destroy",
      { participle := some "destroyed", dest := some .graveyard, agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩ }),
    ("detain", { agentRole := playerAgent }),
    ("discard",
      { participle := some "discarded", dest := some .graveyard, agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], []⟩, false, some .hand⟩, ownPatient := true }),
    ("discover", { agentRole := playerAgent }),
    ("double", {}),
    ("earthbend", { agentRole := playerAgent }),
    ("endure", { agentRole := permanentAgent }),
    ("exchange", { agentRole := playerAgent }),
    ("exert", { agentRole := playerAgent }),
    ("exile",
      { participle := some "exiled", dest := some .exile, agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], []⟩, false, none⟩ }),
    ("explore", { agentRole := permanentAgent }),
    ("faceAVillainousChoice", { agentRole := playerAgent }),
    ("fateseal", { agentRole := playerAgent }),
    ("fight", { agentRole := permanentAgent }),
    ("forage", { agentRole := playerAgent }),
    ("goad", { agentRole := playerAgent }),
    ("harness", { agentRole := permanentAgent }),
    ("heal", { agentRole := playerAgent }),
    ("incubate", { dest := some .battlefield, agentRole := playerAgent }),
    ("investigate", { dest := some .battlefield, agentRole := playerAgent }),
    ("learn", {}),
    ("manifest", { dest := some .battlefield, agentRole := playerAgent }),
    ("manifestDread", { agentRole := playerAgent }),
    ("meld",
      { dest := some .battlefield, agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], []⟩, false, none⟩ }),
    ("mill",
      { participle := some "milled", dest := some .graveyard, agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], []⟩, false, some .library⟩ }),
    ("monstrosity", { agentRole := permanentAgent }),
    ("play",
      { agentRole := playerAgent, patientRole := ⟨some ⟨.object, [], allTypes⟩, true, none⟩ }),
    ("populate", { agentRole := playerAgent }),
    ("proliferate", { agentRole := playerAgent }),
    ("recruit", { agentRole := playerAgent }),
    ("regenerate",
      { participle := some "regenerated", agentRole := ⟨none, true, none⟩,
        patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩ }),
    ("reveal",
      { agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [.card], allTypes⟩, true, none⟩ }),
    ("sacrifice",
      { participle := some "sacrificed", dest := some .graveyard, agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩,
        feature := some .sacrificing, ownPatient := true }),
    ("scry", { agentRole := playerAgent }),
    ("search",
      { loci := [.battlefield, .graveyard, .exile, .hand, .library, .stack, .command],
        agentRole := playerAgent, feature := some .librarySearch }),
    ("shuffle", { loci := [.library], agentRole := playerAgent }),
    ("support", {}),
    ("surveil", { agentRole := playerAgent }),
    ("suspect", { agentRole := playerAgent }),
    ("tap",
      { participle := some "tapped", agentRole := playerAgent, patientRole := fieldObject,
        feature := some .tapping }),
    ("theRingTemptsYou", {}),
    ("timeTravel", { agentRole := playerAgent }),
    ("transform",
      { intransitive := true, agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩ }),
    ("triple", {}),
    ("untap",
      { participle := some "untapped", agentRole := playerAgent,
        patientRole := ⟨some ⟨.object, [], permanentTypes⟩, true, some .battlefield⟩ }),
    ("ventureIntoTheDungeon", {}),
    ("vote", { agentRole := playerAgent }),
    ("waterbend", { agentRole := playerAgent }) ]

def distinctActLabels : List (KeywordActionLabel × ActFacts) → Bool
  | [] => true
  | f :: fs => !(fs.map (·.1)).elem f.1 && distinctActLabels fs

def distinctAbilityDeedLabels : List (KeywordLabel × ActFacts) → Bool
  | [] => true
  | f :: fs => !(fs.map (·.1)).elem f.1 && distinctAbilityDeedLabels fs

def deedFacts : Deed → Option ActFacts
  | .core d => some (coreDeedFacts d)
  | .action l => (actFacts.find? (·.1 == l)).map (·.2)
  | .ofAbility k => (abilityDeedFacts.find? (·.1 == k)).map (·.2)

def knownAct (v : Deed) : Bool := (deedFacts v).isSome
def deedFeatureOf (v : Deed) : Option DeedFeature := deedFacts v >>= (·.feature)
def featureLabel (f : DeedFeature) : Option Deed :=
  (actFacts.find? (·.2.feature == some f)).map (fun row => .action row.1)
def deedLabel (f : DeedFeature) : Deed := (featureLabel f).getD (.action "")
def participleOf (v : Deed) : Option String := deedFacts v >>= (·.participle)
def actPatientKindsOf (v : Deed) : List Kind :=
  (deedFacts v).elim [] fun f => f.patientRole.sort.elim [] (·.domain.kinds)
def actNamesPatient (v : Deed) : Bool := !(actPatientKindsOf v).isEmpty
def actZoneOf (v : Deed) : Option Zone := deedFacts v >>= (·.patientRole.zone)
def actDestOf (v : Deed) : Option Zone := deedFacts v >>= (·.dest)
def actLociOf (v : Deed) : List Zone := (deedFacts v).elim [] (·.loci)
def actNamesLocus (v : Deed) : Bool := !(actLociOf v).isEmpty
def actOwnPatient (v : Deed) : Bool := (deedFacts v).elim false (·.ownPatient)
def actIntransitiveOf (v : Deed) : Bool := (deedFacts v).elim false (·.intransitive)
def actNamesParticiple (v : Deed) : Bool := (participleOf v).isSome

/-! ## Discourse Representation Structure (`DRS`) -/

/-- Provenance: the verb that last moved a referent, whether it was on the battlefield then,
and whether it changed zones. -/
structure Stamp where
  verb : Deed
  wasField : Bool
  moved : Bool
  deriving DecidableEq, Repr, BEq

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

/-- Whose an object referent is, relative to the controller of the text ("you" [CR#109.5]):
the player in whose hand, graveyard or library the card is [CR#400.3], or the player who
controls the permanent [CR#108.4]. -/
inductive Possessor where
  | you | other
  deriving DecidableEq, Repr

/-- Two possessors are known to differ only when one is the controller and the other is not. -/
def Possessor.differs : Option Possessor → Option Possessor → Bool
  | some .you, some .other | some .other, some .you => true
  | _, _ => false

/-- Stable monotonic identifier assigned to each discourse referent on introduction. -/
abbrev ReferentId := Nat

/-- Grammatical role of a referent at its latest mention or introduction (`semantics-v3.md` §1.2). -/
inductive GramRole where
  | subj
  | obj
  | other
  deriving DecidableEq, Repr

def GramRole.rank : GramRole → Nat
  | .subj  => 0
  | .obj   => 1
  | .other => 2

/-- Checker-internal performer metadata for `Instruction.instruct` handoffs. -/
structure ActorShape where
  kind         : Kind
  isYou        : Bool
  selfDefined  : Bool
  ascribable   : Bool
  twoParties   : Bool
  opponentOnly : Bool
  ability      : Bool := false
  refIds       : List ReferentId := []
  deriving DecidableEq, Repr

/-- Domain-sort and characteristic facts of a discourse referent (`semantics-v3.md` §1.2).
`parameterFrame`, `scopeFrame`, `hidden`, `masked`, and `amount` are gone. -/
inductive Payload where
  | object (ty : List CardType) (zone : Option Zone) (prov : Option Stamp) (orig : Option Origin)
      (size : Option Nat) (possessor : Option Possessor := none)
  | player (chosen : Bool)
  | quality (q : QualitySort)
  | outcome (sort : OutcomeSort)
  | gap
  | letter (l : Letter)
  | turnRef
  /-- An ability object and its remembered location [CR#113.1c,724.1b,724.2b]. -/
  | ability (orig : Option Origin) (zone : Option Zone := some .stack)
  | pile (zone : Option Zone) (size : Option Nat) (face : Option PileFace)
  | join (l r : Payload)
  /-- Checker-private handoff frame: the facts of the player or permanent performing the
  enclosing `Instruction.instruct` body. It is never a discourse referent. -/
  | actor (shape : ActorShape)
  deriving DecidableEq, Repr

namespace Payload

/-- The kind a payload denotes; an ability on the stack is an object [CR#113.1c]. -/
def kind : Payload → Kind
  | .object .. => .object
  | .player _ => .player
  | .quality q => .quality q
  | .outcome _ | .gap | .actor _ => .outcome
  | .letter l => .letter l
  | .turnRef => .turnRef
  | .ability _ _ => .object
  | .pile .. => .pile
  | .join l r => .join l.kind r.kind

def zone : Payload → Option Zone
  | .object _ zn _ _ _ _ => zn
  | .ability _ zn => zn
  | .pile zn _ _ => zn
  | .join l r => (l.zone).elim r.zone some
  | _ => none

def joinSeed : List CardType → List CardType → List CardType
  | [], t => t
  | t, [] => t
  | t, u => commonTypes t u

def ty : Payload → List CardType
  | .object t _ _ _ _ _ => t
  | .join l r => joinSeed l.ty r.ty
  | _ => []

def size : Payload → Option Nat
  | .object _ _ _ _ sz _ => sz
  | .pile _ sz _ => sz
  | .join l r => (l.size).elim r.size some
  | _ => none

def face : Payload → Option PileFace
  | .pile _ _ fc => fc
  | _ => none

def prov : Payload → Option Stamp
  | .object _ _ pv _ _ _ => pv
  | .join l r => (l.prov).elim r.prov some
  | _ => none

def orig : Payload → Option Origin
  | .object _ _ _ og _ _ => og
  | .ability og _ => og
  | .join l r => (l.orig).elim r.orig some
  | _ => none

def joined : Payload → Bool
  | .join _ _ => true
  | _ => false

/-- An object's possessor, where the text fixed it; a join's object half. -/
def possessor : Payload → Option Possessor
  | .object _ _ _ _ _ ps => ps
  | .join l r => (l.possessor).elim r.possessor some
  | _ => none

end Payload

/-- A discourse referent in a `DRS` (`semantics-v3.md` §1.2). -/
structure Referent where
  det             : Determiner
  plur            : Plurality
  payload         : Payload
  idents          : List BindingId := []
  id              : ReferentId := 0
  clause          : Nat := 0
  role            : GramRole := .other
  pronounResolved : Bool := false
  targets         : List Kind := []
  attachmentHost  : Bool := false
  deriving DecidableEq, Repr

abbrev Binding := Referent

def Referent.kind (b : Referent) : Kind := b.payload.kind
def Referent.zone (b : Referent) : Option Zone := b.payload.zone
def Referent.ty (b : Referent) : List CardType := b.payload.ty
def Referent.size (b : Referent) : Option Nat := b.payload.size
def Referent.face (b : Referent) : Option PileFace := b.payload.face
def Referent.possessor (b : Referent) : Option Possessor := b.payload.possessor

def Referent.isActorFrame (b : Referent) : Bool :=
  match b.payload with
  | .actor _ => true
  | _ => false

def Referent.isGap (b : Referent) : Bool :=
  match b.payload with
  | .gap => true
  | _ => false

def Referent.sameCore (a b : Referent) : Bool :=
  a.det == b.det && a.plur == b.plur && a.payload == b.payload && a.idents == b.idents

/-- Explicit Reference Scope frame for `Instruction.ownScope` (`semantics-v3.md` §1.2, §1.4). -/
structure OwnScopeFrame where
  preEntryIds   : List ReferentId
  maskedIds     : List ReferentId
  callerClause  : Nat
  callerPending : List (ReferentId × GramRole) := []
  callerReferentIds : List ReferentId := []
  deriving DecidableEq, Repr

/-- Discourse Representation Structure (`semantics-v3.md` §1.2). -/
structure DRS where
  referents     : List Referent := []
  nextId        : ReferentId := 0
  currentClause : Nat := 0
  performer     : Option ReferentId := none
  scopeStack    : List OwnScopeFrame := []
  outerReferentIds  : List (List ReferentId) := []
  pendingRoles  : List (ReferentId × GramRole) := []
  resolvedIds   : List ReferentId := []
  relativeKind  : Option Kind := none
  relativeBinder : Option ReferentId := none
  relativeUses  : Nat := 0
  /-- The current action describes an event or permission pattern. -/
  actionPattern : Bool := false
  characteristicRead : Bool := false
  deriving DecidableEq, Repr

abbrev Bindings := DRS

def DRS.ofList (rs : List Referent) : DRS :=
  let n := rs.length
  let indexed := rs.zipIdx.map fun (r, idx) =>
    if r.id == 0 && n > 0 then { r with id := n - 1 - idx } else r
  let maxId := indexed.foldl (fun acc r => Nat.max acc (r.id + 1)) 0
  { referents := indexed, nextId := maxId }

instance : Coe (List Referent) DRS where
  coe := DRS.ofList

instance : EmptyCollection DRS where
  emptyCollection := {}

/-- A referent is visible if no enclosing `OwnScopeFrame` masks its `ReferentId`. -/
def DRS.isVisible (drs : DRS) (id : ReferentId) : Bool :=
  drs.scopeStack.all fun f => !f.maskedIds.elem id

/-- All referents currently visible in `drs` (excluding masked caller referents). -/
def DRS.visibleReferents (drs : DRS) : List Referent :=
  drs.referents.filter fun r => drs.isVisible r.id

def DRS.isEmpty (drs : DRS) : Bool := drs.visibleReferents.isEmpty

def DRS.length (drs : DRS) : Nat := drs.referents.length

def DRS.find? (drs : DRS) (p : Referent → Bool) : Option Referent :=
  drs.visibleReferents.find? p

def DRS.findById? (drs : DRS) (id : ReferentId) : Option Referent :=
  drs.referents.find? (·.id == id)

def DRS.map {α : Type} (f : Referent → α) (drs : DRS) : List α :=
  drs.visibleReferents.map f

def DRS.filter (p : Referent → Bool) (drs : DRS) : List Referent :=
  drs.visibleReferents.filter p

def DRS.any (p : Referent → Bool) (drs : DRS) : Bool :=
  drs.visibleReferents.any p

def DRS.all (p : Referent → Bool) (drs : DRS) : Bool :=
  drs.visibleReferents.all p

def DRS.sameReferents (a b : DRS) : Bool :=
  let va := a.visibleReferents
  let vb := b.visibleReferents
  va.length == vb.length && (va.zip vb).all fun (x, y) => x.sameCore y

def mergeIdents (old new : List BindingId) : List BindingId :=
  old ++ new.filter (!old.contains ·)

def DRS.updateById (drs : DRS) (id : ReferentId) (f : Referent → Referent) : DRS :=
  { drs with referents := drs.referents.map fun r =>
      if r.id == id then
        let updated := f r
        { updated with id := r.id, idents := mergeIdents r.idents updated.idents }
      else r }

/-- Assign fresh monotonic `ReferentId`s and `drs.currentClause` to newly introduced referents
and prepend them to `drs.referents`. -/
def DRS.prepend (drs : DRS) (additions : List Referent) (role : GramRole := .other)
    (inArgSlot : Bool := false) : DRS × List Referent :=
  let n := additions.length
  let stamped := additions.zipIdx.map fun (r, idx) =>
    let rid := drs.nextId + (n - 1 - idx)
    let rRole := if idx == 0 && r.role == .other then role else r.role
    { r with id := rid, clause := drs.currentClause, role := rRole }
  let pending := match stamped with
    | head :: _ =>
      if inArgSlot && !head.isActorFrame then (head.id, role) :: drs.pendingRoles
      else drs.pendingRoles
    | [] => drs.pendingRoles
  ({ drs with
     referents := stamped ++ drs.referents
     nextId := drs.nextId + n
     pendingRoles := pending }, stamped)

def DRS.push (r : Referent) (drs : DRS) (role : GramRole := .other)
    (inArgSlot : Bool := false) : DRS :=
  (drs.prepend [r] role inArgSlot).1

def DRS.pushAll (additions : List Referent) (drs : DRS) (role : GramRole := .other)
    (inArgSlot : Bool := false) : DRS :=
  (drs.prepend additions role inArgSlot).1

instance : HAppend (List Referent) DRS DRS where
  hAppend rs drs := drs.pushAll rs

/-- Advance the clause counter (`semantics-v3.md` §1.2): commits buffered `pendingRoles` updates
(`clause` and `role`) onto `drs.referents`, clears `pendingRoles`, and resets `pronounResolved`. -/
def nextClause (drs : DRS) : DRS :=
  let updated := drs.referents.map fun r =>
    match drs.pendingRoles.find? (fun (id, _) => id == r.id) with
    | some (_, slotRole) =>
        { r with clause := drs.currentClause, role := slotRole, pronounResolved := false }
    | none =>
        { r with pronounResolved := false }
  { drs with
    referents     := updated
    currentClause := drs.currentClause + 1
    pendingRoles  := [] }

/-- Record that an existing referent `id` was mentioned in a verb argument slot with `role`
(and if resolved by a bare pronoun, mark `pronounResolved := true`). -/
def DRS.recordMention (drs : DRS) (id : ReferentId) (role : GramRole := .other)
    (inArgSlot : Bool := false) (isBarePronoun : Bool := false) : DRS :=
  let refs := if isBarePronoun then
    drs.referents.map fun r => if r.id == id then { r with pronounResolved := true } else r
  else drs.referents
  let pending := if inArgSlot then (id, role) :: drs.pendingRoles else drs.pendingRoles
  let resolved := if drs.resolvedIds.elem id then drs.resolvedIds else id :: drs.resolvedIds
  { drs with referents := refs, pendingRoles := pending, resolvedIds := resolved }

/-- Re-size an object or pile referent; any other referent is returned unchanged. -/
def sized (sz : Option Nat) (b : Referent) : Referent :=
  match b.payload with
  | .object ty zn pv og _ ps => { b with payload := .object ty zn pv og sz ps }
  | .pile zn _ fc => { b with payload := .pile zn sz fc }
  | _ => b

/-- The head type(s) a phrase seeds, one per join half. -/
inductive HeadTy where
  | sole (ty : List CardType)
  | join (a b : HeadTy)
  deriving Repr, BEq

def outcomeB (s : OutcomeSort) : Referent :=
  { det := .the, plur := .one, payload := .outcome s }
def gapB : Referent :=
  { det := .the, plur := .one, payload := .gap }
def turnRefB : Referent :=
  { det := .the, plur := .one, payload := .turnRef }
def letterB (l : Letter) : Referent :=
  { det := .a, plur := .one, payload := .letter l }
def qualityB (q : QualitySort) : Referent :=
  { det := .a, plur := .one, payload := .quality q }

def ChoiceSort.binding : ChoiceSort → Referent
  | .quality q => qualityB q
  | .player => { det := .a, plur := .one, payload := .player true }

def ChoiceSort.binds : ChoiceSort → Kind → Payload → Bool
  | .quality q, k, _ => Kind.lte (.quality q) k
  | .player, _, .player ch => ch
  | .player, _, _ => false

def choiceSortAt : Kind → Option ChoiceSort
  | .quality q => some (.quality q)
  | .player => some .player
  | _ => none

def introducedChoiceAt (k : Kind) : List Referent :=
  match choiceSortAt k with
  | none => []
  | some s => [s.binding]

def countOnes (k : Kind) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    !b.isActorFrame && !b.isGap && b.plur.isOne && Kind.lte k b.kind).length

def countGaps (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b => b.isGap && b.plur.isOne).length

def countOutcomes (s : OutcomeSort) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    match b.plur, b.payload with
    | .one, .outcome s' => s == s'
    | _, _ => false).length

def outcomeInScope (s : OutcomeSort) (drs : DRS) : Bool :=
  drs.visibleReferents.any fun b =>
    match b.plur, b.payload with
    | .one, .outcome s' => s == s'
    | _, _ => false

def damageDealtInScope (drs : DRS) : Bool := outcomeInScope .damageDealt drs
def coinFlipInScope (drs : DRS) : Bool := outcomeInScope .coinFlipped drs

def ignorableInScope (drs : DRS) : Bool :=
  countOutcomes .rollResult drs == 1 || coinFlipInScope drs

def OutcomeSort.isAmount : OutcomeSort → Bool
  | .coinFlipped | .manaAdded | .manaProduced | .ceilingShortfall | .voteHeld =>
    false
  | _ => true

def countAmountOutcomes (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    match b.plur, b.payload with
    | .one, .outcome s => s.isAmount
    | _, _ => false).length

def countChoice (s : ChoiceSort) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    b.plur.isOne && s.binds b.kind b.payload).length

/-- Which announced choice a read names: the one standing choice, or the most recent of
several. -/
def ChoiceRef.ok : ChoiceRef → Nat → Bool
  | .theChoice, n => n == 1
  | .theLatestChoice, n => n != 0

def distinctLabels : List VoteLabel → Bool
  | [] => true
  | l :: ls => !ls.elem l && distinctLabels ls

def ballotLabelsOk (opts : List VoteLabel) : Bool := 2 ≤ opts.length && distinctLabels opts

def countLetter (l : Letter) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    b.plur.isOne && Kind.lte (.letter l) b.kind).length

def dropLetter (l : Letter) (drs : DRS) : DRS :=
  { drs with referents := drs.referents.filter fun b =>
      !(drs.isVisible b.id && Kind.lte (.letter l) b.kind) }

def openLetter (l : Letter) (b : Referent) : Bool :=
  match b.det with
  | .a => b.plur.isOne && Kind.lte (.letter l) b.kind
  | _ => false

def anyOpenLetter (l : Letter) (drs : DRS) : Bool :=
  drs.visibleReferents.any (openLetter l)

def defineLetter (l : Letter) (drs : DRS) : DRS :=
  { drs with referents := drs.referents.map fun b =>
      if drs.isVisible b.id && openLetter l b then { b with det := .the } else b }

def introducedLetters (l : Letter) (drs : DRS) : List Referent :=
  if countLetter l drs == 0 then [letterB l] else []

def countManysAny (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b => !b.isActorFrame && !b.plur.isOne).length

def countManys (k : Kind) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    !b.isActorFrame && !b.plur.isOne && Kind.lte k b.kind).length

def objGroup (k : Kind) (b : Referent) : Bool := Kind.lte k b.kind && !b.plur.isOne

def countGroups (k : Kind) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    b.det != .part && b.det != .bare && objGroup k b).length

def countParts (k : Kind) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    b.det == .part && Kind.lte k b.kind).length

def theRestOk (k : Kind) (drs : DRS) : Bool := countGroups k drs ≤ 1 && countParts k drs != 0

def partsTaken (k : Kind) (drs : DRS) : Nat :=
  drs.visibleReferents.foldl (fun acc b =>
    if b.det == .part && Kind.lte k b.kind then
      acc + b.size.getD 1
    else acc) 0

def countedGroupSize (k : Kind) (drs : DRS) : Option Nat :=
  let rec loop : List Referent → Option Nat
    | [] => none
    | b :: bs =>
      if b.det == .part || b.det == .bare then loop bs
      else if objGroup k b then b.size
      else loop bs
  loop drs.visibleReferents

def theOtherOk (k : Kind) (drs : DRS) : Bool :=
  theRestOk k drs &&
    (match countedGroupSize k drs with
     | none => false
     | some n => n == partsTaken k drs + 1)

def theRestFits (k : Kind) : Plurality → DRS → Bool
  | .one, drs => theOtherOk k drs
  | .many, drs => theRestOk k drs

/-- `out` re-places referents of `outer` without touching the table: every referent keeps its
determiner, kind and plurality, and only plural ones, one object per agent, may have moved
[CR#701.21a]. -/
def spentDistributivelyList : List Referent → List Referent → Bool
  | [], [] => true
  | a :: as_, b :: bs =>
    a.det == b.det && a.kind == b.kind && a.plur == b.plur &&
      (a.payload == b.payload || !a.plur.isOne) && spentDistributivelyList as_ bs
  | _, _ => false

def spentDistributively (outer out : DRS) : Bool :=
  spentDistributivelyList outer.visibleReferents out.visibleReferents

def partsClosed (drs : DRS) : DRS :=
  { drs with referents := drs.referents.map fun b =>
      if drs.isVisible b.id && b.det == .part then { b with det := .the } else b }

def partsDistributed (drs : DRS) : Bool :=
  drs.visibleReferents.all fun b => b.det != .part || !b.plur.isOne

def restSource (k : Kind) (drs : DRS) : Option Referent :=
  let rec loop : List Referent → Option Referent
    | [] => none
    | b :: bs =>
      if b.det == .part then
        match loop bs with
        | some s => some s
        | none => if Kind.lte k b.kind then some b else none
      else if objGroup k b then some b
      else loop bs
  loop drs.visibleReferents

def zoneOfGroup (k : Kind) (drs : DRS) : Option Zone := restSource k drs >>= (·.zone)
def tyOfGroup (k : Kind) (drs : DRS) : List CardType := (restSource k drs).elim [] (·.ty)
def provOfGroup (k : Kind) (drs : DRS) : Option Stamp := restSource k drs >>= (·.payload.prov)

def anyTargeted (k : Kind) (drs : DRS) : Bool :=
  drs.visibleReferents.any fun b => b.det == .target && Kind.lte k b.kind

def anchorTyOk (t : CardType) : List CardType → Bool
  | [] => true
  | ts => ts.contains t

def settleTargets (drs : DRS) : DRS :=
  { drs with referents := drs.referents.map fun b =>
      if drs.isVisible b.id && b.det == .target then { b with det := .the } else b }

def outcomesOnly (drs : DRS) : DRS :=
  { drs with referents := drs.referents.filter fun b =>
      !drs.isVisible b.id || (match b.payload with | .outcome _ => true | _ => false) }

def agreedField {α : Type} (f : α → α → Bool) : Option α → Option α → Option α
  | some x, some y => if f x y then some x else none
  | _, _ => none

/-- The payload two arms of a choice agree on, if any: the union of their payloads. -/
def unionPayload : Payload → Payload → Option (Kind × Payload)
  | .object t1 z1 v1 o1 s1 p1, .object t2 z2 v2 o2 s2 p2 =>
    some (.object, .object (commonTypes t1 t2) (agreedField (· == ·) z1 z2)
      (agreedField (· == ·) v1 v2) (agreedField (· == ·) o1 o2) (agreedField (· == ·) s1 s2)
      (agreedField (· == ·) p1 p2))
  | .ability o1 z1, .ability o2 z2 =>
    some (.object, .ability (agreedField (· == ·) o1 o2) (agreedField (· == ·) z1 z2))
  | .player a, .player b => if a == b then some (.player, .player a) else none
  | .object _ z1 _ o1 _ _, .ability o2 z2 =>
    some (.object, .object [] (agreedField (· == ·) z1 z2) none (agreedField (· == ·) o1 o2) none none)
  | .ability o1 z1, .object _ z2 _ o2 _ _ =>
    some (.object, .object [] (agreedField (· == ·) z1 z2) none (agreedField (· == ·) o1 o2) none none)
  | p@(.object ..), .player false => some (.join .object .player, .join p (.player false))
  | .player false, q@(.object ..) => some (.join .object .player, .join q (.player false))
  | _, _ => none

def unionBinding (b c : Referent) : Option Referent :=
  if b.sameCore c then
    some { b with
      clause := Nat.max b.clause c.clause
      role := if b.role.rank ≤ c.role.rank then b.role else c.role }
  else if b.det == c.det && b.plur == c.plur then do
    let p := b.payload
    let q := c.payload
    let value ← if p == q then some p else (unionPayload p q).map Prod.snd
    pure { b with
      payload := value
      idents := b.idents.filter c.idents.contains
      clause := Nat.max b.clause c.clause
      role := if b.role.rank ≤ c.role.rank then b.role else c.role }
  else none

def unionReferentLists : List Referent → List Referent → Option (List Referent)
  | [], [] => some []
  | b :: bs, c :: cs =>
    match unionBinding b c, unionReferentLists bs cs with
    | some u, some us => some (u :: us)
    | _, _ => none
  | _, _ => none

def unionBindings (l r : DRS) : Option DRS := do
  let refs ← unionReferentLists l.referents r.referents
  pure { l with
    referents := refs
    nextId := Nat.max l.nextId r.nextId
    currentClause := Nat.max l.currentClause r.currentClause }

def Zone.isPublic : Zone → Bool
  | .hand => false
  | .library => false
  | _ => true

def Referent.isPublic (b : Referent) : Bool :=
  match b.payload with
  | .object _ (some z) _ _ _ _ => z.isPublic
  | .pile _ _ (some face) => face == .faceUp
  | .pile (some z) _ none => z.isPublic
  | _ => true

def pluralizeBinding (b : Referent) : Referent :=
  match b.det with
  | .self => b
  | _ => { b with plur := .many }

def pluralizeIntroduced (bs : List Referent) : List Referent := bs.map pluralizeBinding

def stampMoves : Option Stamp → Bool
  | none => false
  | some s => s.moved

def stampWasField : Option Stamp → Bool
  | none => false
  | some s => s.wasField

/-! ## Pronoun reach & Anaphor resolution (`J1`, `J2`, `D′`, `AC>R1`, Principle A) -/

def tyIs (t : CardType) (types : List CardType) : Bool := types.contains t

def isCardZone : Option Zone → Bool
  | some .graveyard | some .exile | some .hand | some .library | some .command => true
  | _ => false

def zoneIsB : Option Zone → Zone → Bool
  | none, _ => false
  | some a, b => a == b

def onFieldZone (z : Option Zone) : Bool := zoneIsB z .battlefield
def onStackZone (z : Option Zone) : Bool := zoneIsB z .stack

/-- The reaches that resolve to an object referent a move can re-stamp. -/
def Reach.tracksObject : Reach → Bool
  | .bare | .atSlot _ | .verbed _ none _ | .tokenBorn => true
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
  | .object ty _ _ _ _ _ => tyIs t ty
  | _ => false

/-- Copy origin belongs to the same object or ability that the word selects. -/
def payloadIsCopy : Payload → Bool
  | .object _ _ _ og _ _ | .ability og _ => isCopyOrigin og
  | _ => false

def halfWordReaches (w : NounWord) (pl : Payload) : Bool :=
  match w with
  | .ofType word t => halfWordReaches word pl && payloadHasType t pl
  | .copied word => halfWordReaches word pl && payloadIsCopy pl
  | word =>
    match pl with
    | .object ty zn _ _ _ _ =>
      match word with
      | .type t => onFieldZone zn && tyIs t ty
      | .permanent => onFieldZone zn
      | _ => false
    | .player _ => word == .player
    | _ => false

def halfReaches (w : NounWord) : Payload → Bool
  | .join l r => halfReaches w l || halfReaches w r
  | pl => halfWordReaches w pl

/-- A word's carrier discipline and each refinement must hold on the same referent. -/
def wordReaches (w : NounWord) (b : Referent) : Bool :=
  match w with
  | .ofType word t => wordReaches word b && payloadHasType t b.payload
  | .copied word => wordReaches word b && payloadIsCopy b.payload
  | w =>
    match b.payload with
    | .object ty zn _ og _ _ =>
      match w with
      | .type t => onFieldZone zn && tyIs t ty
      | .card => isCardZone zn
      | .spell => onStackZone zn
      | .permanent => onFieldZone zn
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

/-- A self-referent is never read by a noun word. -/
def wordNow (w : NounWord) (b : Referent) : Bool :=
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
  | .verbed _ (some w) _ => w.kind
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

def markTy (ty : List CardType) (b : Referent) : Referent :=
  match b.payload with
  | .object old zn st og sz ps =>
    { b with payload := .object (mergeTypes old ty) zn st og sz ps }
  | _ => b

def markFirstList (drs : DRS) (q : Referent → Bool) (ty : List CardType) :
    List Referent → List Referent
  | [] => []
  | b :: bs =>
    if drs.isVisible b.id && q b then markTy ty b :: bs
    else b :: markFirstList drs q ty bs

def markFirst (q : Referent → Bool) (ty : List CardType) (drs : DRS) : DRS :=
  { drs with referents := markFirstList drs q ty drs.referents }

def survivesShuffle (b : Referent) : Bool :=
  match b.payload with
  | .object _ (some .library) (some st) _ _ _ => st.feature == some .librarySearch
  | .object _ (some .library) none _ _ _ => false
  | _ => true

def stampWordOk (v : Deed) (w : NounWord) (st : Stamp) (ty : List CardType)
    (zn : Option Zone) (og : Option Origin) : Bool :=
  stampedBy v st && verbedWordOk w st ty zn og

/-- Stage 1 (`J1`): sort, plurality, zone/carrier, and stamp reach (`semantics-v3.md` §1.5). -/
def reaches (r : Reach) (pl : Plurality) (b : Referent) : Bool :=
  match r with
  | .bare => Kind.lte .object b.kind && pl.isOne == b.plur.isOne
  | .atSlot sl => Kind.lte .object b.kind && pl.isOne == b.plur.isOne && slotZoneOk sl b.zone
  | .verbed v none _ =>
    Kind.lte .object b.kind && pl.isOne == b.plur.isOne && stampIs v b.payload.prov
  | .tokenBorn =>
    Kind.lte .object b.kind && pl.isOne == b.plur.isOne && isTokenOrigin b.payload.orig
  | .word w => pl.isOne == b.plur.isOne && wordNow w b
  | .unionHalf w => pl.isOne == b.plur.isOne && b.payload.joined && halfReaches w b.payload
  | .verbed v (some w) _ =>
    match b.payload with
    | .object ty zn (some st) og _ _ =>
      pl.isOne == b.plur.isOne && stampWordOk v w st ty zn og
    | _ => false

/-- A characteristic read may name a departed permanent by its former type
[CR#608.2h]. This changes accessibility, not the referent's current zone. -/
def reachesCharacteristic (r : Reach) (pl : Plurality) (b : Referent) : Bool :=
  reaches r pl b || match r, b.payload with
    | .word w, .object ty zn (some st) og _ _ =>
      b.det != .self && pl.isOne == b.plur.isOne && st.moved &&
        verbedWordOk w st ty zn og
    | _, _ => false

/-- Stage 2 (`J2`): whether `id` is a clause-mate co-argument in the current clause. -/
def DRS.isClauseMate (drs : DRS) (id : ReferentId) : Bool :=
  drs.pendingRoles.any fun (pid, _) => pid == id

/-- Two-stage anaphor candidate resolution (`semantics-v3.md` §1.5):
- Bare pronouns (`.bare`, `.atSlot _`) use `D′`: filter by `J1` and exclude `J2` clause-mates
  (when `inArgSlot`), then select the nearest clause (highest `clause`) and highest grammatical
  role (`subj < obj < other`); a unique winner resolves, while `0` or `≥ 2` winners refuse.
- Demonstratives / definite NPs (`.word _`, `.tokenBorn`, `.unionHalf _`) use `AC>R1`: filter by
  `J1`, exclude `J2` clause-mates (`A`, when `inArgSlot`), exclude any referent already resolved
  by a bare pronoun in the current clause (`C`, `pronounResolved == true`), and pick the most
  recently introduced candidate (`R1`, highest `id`).
- Stamped participles (`.verbed _ _ _`) filter by `J1` and require a unique match. -/
def resolveCandidates (r : Reach) (pl : Plurality) (drs : DRS) (inArgSlot : Bool := false) :
    List Referent :=
  let j1 := drs.visibleReferents.filter
    (if drs.characteristicRead then reachesCharacteristic r pl else reaches r pl)
  match r with
  | .bare | .atSlot _ =>
      let cands := if inArgSlot then j1.filter (fun b => !drs.isClauseMate b.id) else j1
      match cands with
      | [] => []
      | _ =>
          let maxClause := cands.foldl (fun acc b => Nat.max acc b.clause) 0
          let inClause := cands.filter (·.clause == maxClause)
          let minRank := inClause.foldl (fun acc b => Nat.min acc b.role.rank) 2
          inClause.filter (·.role.rank == minRank)
  | .word _ | .tokenBorn | .unionHalf _ =>
      let afterA := if inArgSlot then j1.filter (fun b => !drs.isClauseMate b.id) else j1
      let afterC := afterA.filter (fun b => !b.pronounResolved)
      match afterC with
      | [] => []
      | h :: t =>
          let best := t.foldl (fun acc b => if b.id > acc.id then b else acc) h
          [best]
  | .verbed _ _ _ =>
      j1

/-- How many winning antecedents `r` resolves to in `drs`; a read is sound at `1`. -/
def countReach (r : Reach) (pl : Plurality) (drs : DRS) (inArgSlot : Bool := false) : Nat :=
  (resolveCandidates r pl drs inArgSlot).length

def firstReach (r : Reach) (pl : Plurality) (drs : DRS) (inArgSlot : Bool := false) :
    Option Referent :=
  (resolveCandidates r pl drs inArgSlot).head?

def provOfReach (r : Reach) (pl : Plurality) (drs : DRS) : Option Stamp :=
  firstReach r pl drs >>= (·.payload.prov)
def zoneOfReach (r : Reach) (pl : Plurality) (drs : DRS) : Option Zone :=
  firstReach r pl drs >>= (·.zone)
def possessorOfReach (r : Reach) (pl : Plurality) (drs : DRS) : Option Possessor :=
  firstReach r pl drs >>= (·.possessor)
def tyOfReach (r : Reach) (pl : Plurality) (drs : DRS) : List CardType :=
  (firstReach r pl drs).elim [] (·.ty)
def faceOfReach (r : Reach) (pl : Plurality) (drs : DRS) : Option PileFace :=
  firstReach r pl drs >>= (·.face)

/-- Principle A reflexive resolution (`NounPhrase.refl plur`, `semantics-v3.md` §1.5):
requires an accessible clause-mate subject referent agreeing with `plur`. -/
def resolveReflexive (pl : Plurality) (drs : DRS) : List Referent :=
  drs.visibleReferents.filter fun b =>
    !b.isActorFrame && b.plur.isOne == pl.isOne &&
    ((drs.pendingRoles.any fun (id, role) => id == b.id && role == .subj) ||
     (b.clause == drs.currentClause && b.role == .subj))

def countReflexive (pl : Plurality) (drs : DRS) : Nat :=
  (resolveReflexive pl drs).length

def firstReflexive (pl : Plurality) (drs : DRS) : Option Referent :=
  (resolveReflexive pl drs).head?

/-! ## Actor handoff & Reference Scope (`enterOwnScope`, `enterCaller`, `leaveCaller`, `leaveOwnScope`) -/

/-- The innermost handoff's performer; `none` outside every handoff, where the performer is
the controller. -/
def actorShape? (drs : DRS) : Option ActorShape :=
  drs.referents.findSome? fun b =>
    match b.payload with
    | .actor shape => some shape
    | _ => none

/-- Leave the innermost handoff: remove its actor frame and nothing else. -/
def dropActorFrame (drs : DRS) : DRS :=
  let rec dropFirst : List Referent → List Referent
    | [] => []
    | b :: bs => if b.isActorFrame then bs else b :: dropFirst bs
  { drs with referents := dropFirst drs.referents }

/-- Close a relative clause while retaining its definite attachment hosts in the outer DRS. -/
def leaveRelative (before after : DRS) : DRS :=
  let hosts := after.referents.filterMap fun r =>
    if r.attachmentHost && r.id ≥ before.nextId then
      some { r with clause := before.currentClause, role := .other, pronounResolved := false }
    else none
  { before with
    referents := hosts ++ before.referents
    nextId := max before.nextId after.nextId }

/-- Open a keyword action body's own Reference Scope (`Instruction.ownScope`, `semantics-v3.md` §1.4):
pushes an `OwnScopeFrame` masking all pre-entry referents except the active performer. -/
def enterOwnScope (drs : DRS) : DRS :=
  let preIds := drs.referents.map (·.id)
  let actorIds := (actorShape? drs).elim [] (·.refIds)
  let masked := drs.referents.filterMap fun r =>
    if r.isActorFrame || drs.performer == some r.id || actorIds.elem r.id then none
    else some r.id
  let frame : OwnScopeFrame := {
    preEntryIds   := preIds
    maskedIds     := masked
    callerClause  := drs.currentClause
    callerPending := drs.pendingRoles
  }
  { drs with
    scopeStack    := frame :: drs.scopeStack
    pendingRoles  := []
    currentClause := drs.currentClause + 1 }

/-- Temporarily pop the top `OwnScopeFrame` to evaluate an `inCaller` argument in the immediate
caller's visibility (`semantics-v3.md` §1.4). -/
def enterCaller (drs : DRS) : DRS :=
  match drs.scopeStack with
  | [] => drs
  | topFrame :: outerFrames =>
      { drs with
        scopeStack    := outerFrames
        currentClause := topFrame.callerClause
        pendingRoles  := topFrame.callerPending
        resolvedIds   := [] }

/-- Restore the top `OwnScopeFrame` after evaluating an `inCaller` argument, unmasking any
referents bound or introduced by that argument (`semantics-v3.md` §1.4). -/
def leaveCaller (before after : DRS) (unmaskIds : List ReferentId := []) : DRS :=
  match before.scopeStack with
  | [] => after
  | topFrame :: _ =>
      let introduced := after.referents.filterMap fun r =>
        if r.id ≥ before.nextId then some r.id
        else match before.findById? r.id with
          | some old => if r.idents != old.idents then some r.id else none
          | none => some r.id
      let newlyBound := (unmaskIds ++ after.resolvedIds ++ introduced).eraseDups
      let masked' := match newlyBound with
        | [] => topFrame.maskedIds
        | ids => topFrame.maskedIds.filter (!ids.elem ·)
      let addedPending := after.pendingRoles.take
        (after.pendingRoles.length - topFrame.callerPending.length)
      let topFrame' : OwnScopeFrame := {
        topFrame with
        maskedIds     := masked'
        callerPending := after.pendingRoles
        callerReferentIds := (topFrame.callerReferentIds ++ newlyBound).eraseDups
      }
      { after with
        scopeStack    := topFrame' :: after.scopeStack
        currentClause := before.currentClause
        pendingRoles  := addedPending ++ before.pendingRoles
        resolvedIds   := before.resolvedIds }

/-- Close an own Reference Scope (`Instruction.ownScope`, `semantics-v3.md` §1.4): pops `topFrame`,
drops definition-internal scratch referents (`.part` and `.player _`), and stamps surviving
referents introduced during `topFrame` with `clause := topFrame.callerClause` and `role := .obj`. -/
def leaveOwnScope (drs : DRS) : DRS :=
  match drs.scopeStack with
  | [] => drs
  | topFrame :: outerFrames =>
      let kept := drs.referents.filterMap fun r =>
        if topFrame.preEntryIds.elem r.id || topFrame.callerReferentIds.elem r.id then
          some r
        else if r.det == .part || (match r.payload with | .player _ => true | _ => false) then
          none
        else
          some { r with clause := topFrame.callerClause, role := .obj, pronounResolved := false }
      { drs with
        referents     := kept
        scopeStack    := outerFrames
        pendingRoles  := topFrame.callerPending
        currentClause := topFrame.callerClause }

/-! ## Binding Identity (`firstMention` / `laterMention`) -/

/-- The most recent visible referent recorded under `id`, if any (`semantics-v3.md` §1.4). -/
def identityBinding (id : BindingId) (drs : DRS) : Option Referent :=
  drs.visibleReferents.find? fun b => b.idents.contains id

/-- Record referent `refId` under `id` as well. -/
def recordIdentityById (id : BindingId) (drs : DRS) (refId : ReferentId) : DRS :=
  drs.updateById refId fun b => { b with idents := mergeIdents b.idents [id] }

/-- Record the head of a list of introduced referents under `id`. -/
def recordHead (id : BindingId) : List Referent → List Referent
  | b :: bs => { b with idents := mergeIdents b.idents [id] } :: bs
  | [] => []

/-- Apply an update to all visible referents recorded under `id`. -/
def overIdentity (f : Referent → Referent) (id : BindingId) (drs : DRS) : DRS :=
  { drs with referents := drs.referents.map fun r =>
      if drs.isVisible r.id && r.idents.contains id then
        let updated := f r
        { updated with id := r.id }
      else r }

/-! ## `NounResult` -/

/-- A noun evaluation result: the updated `DRS`, the `ReferentId` of the introduced or resolved
head referent (if any), its `Referent` value, the `ReferentId`s of coordinated conjuncts, and the
referents introduced by this phrase. -/
structure NounResult where
  context   : DRS
  refId     : Option ReferentId
  value     : Option Referent
  additions : List Referent := []
  refIds    : List ReferentId := []
  deriving DecidableEq, Repr

def NounResult.introduced (_drs : DRS) (result : NounResult) : List Referent :=
  result.additions

def introducedNounResult (drs : DRS) (additions : List Referent) (ownsHead : Bool)
    (role : GramRole := .other) (inArgSlot : Bool := false) : NounResult :=
  let (context, stamped) := drs.prepend additions role inArgSlot
  let head := if ownsHead then stamped.head? else none
  { context   := context
    refId     := head.map (·.id)
    value     := head
    additions := stamped
    refIds    := (head.map (·.id)).toList }

/-- A containing noun publishes its own mentions over the child's updated context. -/
def NounResult.prepend (child : NounResult) (additions : List Referent)
    (role : GramRole := .other) (inArgSlot : Bool := false) : NounResult :=
  let res := introducedNounResult child.context additions true role inArgSlot
  { res with additions := res.additions ++ child.additions }

def NounResult.withValue (result : NounResult) (fallback : Referent)
    (view : Referent → Referent) : NounResult :=
  let value := view (result.value.getD fallback)
  let context := match result.refId with
    | some id => result.context.updateById id (fun _ => value)
    | none => result.context
  let additions := match result.refId with
    | some id => result.additions.map fun r => if r.id == id then { value with id := r.id } else r
    | none => result.additions
  { result with value := some value, context, additions }

/-- A first mention's result: record the referent(s) introduced or resolved by `result` under `id`. -/
def NounResult.recordIdentity (id : BindingId) (result : NounResult) : NounResult :=
  let ids := if result.refIds.isEmpty then result.refId.toList else result.refIds
  if ids.isEmpty then result
  else
    let context := ids.foldl (recordIdentityById id) result.context
    let additions := result.additions.map fun r =>
      if ids.elem r.id then { r with idents := mergeIdents r.idents [id] } else r
    { result with
      context   := context
      value     := (result.refId >>= context.findById?).or result.value
      additions := additions }

/-! ## Filtering a `DRS` -/

def filterContext (keep : Referent → Bool) (drs : DRS) : DRS :=
  { drs with referents := drs.referents.filter fun b =>
      b.isActorFrame || !drs.isVisible b.id || keep b }

def publicOnly (drs : DRS) : DRS := filterContext Referent.isPublic drs

def afterShuffle (drs : DRS) : DRS :=
  let refs := drs.referents.filter fun r => r.isActorFrame || survivesShuffle r
  let keptIds := refs.map (·.id)
  { drs with
    referents := refs
    pendingRoles := drs.pendingRoles.filter (fun p => keptIds.elem p.1)
    resolvedIds := drs.resolvedIds.filter keptIds.elem }

def groupSpent (k : Kind) (drs : DRS) : DRS :=
  let filtered := filterContext (fun b => b.det == .part || !objGroup k b) drs
  { filtered with referents := filtered.referents.map fun b =>
      if filtered.isVisible b.id && b.det == .part then { b with det := .the } else b }

def countTokenSpecs (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    b.det != .self && isTokenOrigin b.payload.orig).length

def tyOfThoseAny (w : NounWord) (drs : DRS) : List CardType :=
  match drs.visibleReferents.find? (wordNow w) with
  | some b => b.ty
  | none => []

def countChoosers (drs : DRS) : Nat := countOnes .player drs + countManys .player drs

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

/-- A copy of an ability is itself an ability [CR#707.10]. -/
def copyPayloadIn : Kind → Bool → List CardType → Option Zone → Payload
  | .object, true, _, z => .ability (some .copy) z
  | .object, false, ty, z => .object ty z none (some .copy) none none
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

def costLetters : Option ManaCost → List Referent
  | none => []
  | some c => if manaHasX c then [letterB .x] else []

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

def CardType.ascribesAs : CardType → Bool
  | .kindred | .instant | .sorcery | .dungeon => false
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

def supersDistinct : List Supertype → Bool
  | [] => true
  | s :: ss => !ss.elem s && supersDistinct ss

/-- "Nth" counts from one [CR#401.7]. -/
def Ordinal.ok : Ordinal → Bool
  | .nth n => n != 0

def OptOrdinal.ok : Option Ordinal → Bool
  | none => true
  | some o => o.ok

/-! ## Designations -/

def DesignationLabel.name : DesignationLabel → String
  | .named n => n

def findDesignation (name : String) : List DesignationFacts → Option DesignationFacts
  | [] => none
  | f :: fs => if f.label == name then some f else findDesignation name fs

def distinctDesignationLabels : List DesignationFacts → Bool
  | [] => true
  | f :: fs => !(fs.map (·.label)).elem f.label && distinctDesignationLabels fs

def DesignationLabel.facts (label : DesignationLabel) : Option DesignationFacts :=
  findDesignation label.name designationTable

def DesignationLabel.known (label : DesignationLabel) : Bool := label.facts.isSome

def DesignationLabel.scope (label : DesignationLabel) : Option DesignationScope :=
  label.facts.map (·.scope)
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

def designationHolderOk (label : DesignationLabel) (z : Option Zone) : Bool :=
  match label.scope with
  | some (.heldBy .player) => true
  | some (.heldBy .object) => zoneIsB z .battlefield
  | _ => false

def RoomHalf.designation (h : RoomHalf) : DesignationLabel :=
  .named (((designationTable.find? (·.half == some h)).map (·.label)).getD "")

def DesignationLabel.half (label : DesignationLabel) : Option RoomHalf := label.facts >>= (·.half)

/-! ## Relations -/

def RelationLabel.name : RelationLabel → String
  | .named n => n

def RelationLabel.facts (label : RelationLabel) : Option RelationFacts :=
  relationTable.find? (·.label == label.name)

def RelationLabel.known (label : RelationLabel) : Bool := label.facts.isSome

def RelationLabel.left (label : RelationLabel) : Kind := (label.facts.map (·.left)).getD .object

def RelationLabel.right (label : RelationLabel) : Kind := (label.facts.map (·.right)).getD .object

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

def NounWord.isAbility : NounWord → Bool
  | .ability => true
  | .ofType word _ | .copied word => word.isAbility
  | _ => false

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

def CounterKind.name : CounterKind → String
  | .named n => n

def CounterKind.scope (c : CounterKind) : Kind := (counterFactsFor c.name).elim .object (·.holder)

/-- What kind of entity a projection axis reads [CR#120.3e,120.6,704.5g]. -/
def ProjAxis.scope : ProjAxis → Kind
  | .stat _ => .object
  | .playerStat _ => .player
  | .counter c => c.scope
  | .anyCounter k => k
  | .chapter => .object
  | .markedDamage => .object

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

def axesAt (k : Kind) : List ProjAxis → Bool
  | [] => false
  | as => as.all (·.scope == k)

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
  | .kindred | .instant | .sorcery | .dungeon => false
  | _ => true

def permanentSpellType (types : List CardType) : Bool :=
  types.any CardType.permanent && !types.contains .land &&
    !types.contains .instant && !types.contains .sorcery

def CardType.isInstantOrSorcery : CardType → Bool
  | .instant | .sorcery => true
  | _ => false

def Status.clash : Status → Status → Bool
  | .tapped, .untapped | .untapped, .tapped | .flipped, .unflipped | .unflipped, .flipped
  | .faceUp, .faceDown | .faceDown, .faceUp | .phasedIn, .phasedOut | .phasedOut, .phasedIn => true
  | _, _ => false

/-- Which status values a description can be written with ("tapped creature"). -/
def Status.word : Status → Bool
  | .tapped | .untapped | .faceUp | .faceDown => true
  | _ => false

def colorsDistinct : List Color → Bool
  | [] => true
  | c :: cs => !cs.elem c && colorsDistinct cs

def ColorSpec.ok : ColorSpec → Bool
  | .some cs => colorsDistinct cs
  | .every => true

def typesDistinct : List CardType → Bool
  | [] => true
  | t :: ts => !ts.elem t && typesDistinct ts

def lastType : List CardType → Option CardType
  | [] => none
  | [t] => some t
  | _ :: ts => lastType ts

end Semantics
