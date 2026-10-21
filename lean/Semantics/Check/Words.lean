import Semantics.Words
import Semantics.Check.Facts

/-!
# Semantics.Check.Words

The checker's vocabulary and Discourse Representation Theory (`DRS`) layer (`semantics-v3.md`
§1.2–§1.5). Holds the structured `DRS` state (`ReferentId`, `GramRole`, `Payload`, `Referent`,
`OwnScopeFrame`, `DRS`), the two-stage anaphor resolver (`J1` accessibility, `J2` clause-mate
exclusion, `D′` bare pronoun resolution, `AC>R1` demonstrative resolution, Principle A
reflexives, and the stamped readback of Participial Descriptions), the Reference Scope frame
stack (`enterOwnScope`, `enterCaller`, `leaveCaller`, `leaveOwnScope`), the facts tables
(`abilityDeedFacts`, `actFacts`), and the leaf predicates the phrase and ability layers consult.
-/

namespace Semantics

/-- A referent's determiner, as the checker records how its phrase introduced it. -/
inductive Determiner where
  | target | a | each | all | the | part | count | self | bare
  deriving DecidableEq, Repr

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

/-- The keyword actions [CR#701.1] a core constructor must name, declared so a guard reads a
feature rather than a verb's spelling. -/
inductive DeedFeature where
  | librarySearch
  deriving DecidableEq, Repr

/-- What the checker knows about one deed: what reference resolution and salience read. -/
structure ActFacts where
  participle : Option String := none
  dest : Option Zone := none
  loci : List Zone := []
  /-- A Permanent is its Actor: written with no Handoff to a Permanent, the source Permanent
  performs it [CR#701.44a,701.63a]. -/
  permanentActor : Bool := false
  feature : Option DeedFeature := none
  deriving Repr, BEq

/-- The deeds a keyword ability defines for itself [CR#702.122b,702.171a,702.26a]. -/
def abilityDeedFacts : List (KeywordLabel × ActFacts) :=
  [ ((.named "crew"), {}),
    ((.named "saddle"), {}),
    ((.named "phasing"), {}),
    ((.named "cycling"), {}),
    ((.named "mutate"), {}),
    ((.named "devour"), {}) ]

/-- What the checker knows about each keyword action [CR#701.1]. -/
def actFacts : List (KeywordActionLabel × ActFacts) :=
  [ ("activate", {}),
    ("adapt", { permanentActor := true }),
    ("airbend", { dest := some .exile }),
    ("amass", {}),
    ("assemble", {}),
    ("attach", {}),
    ("behold", {}),
    ("blight", {}),
    ("bolster", {}),
    ("cast", {}),
    ("clash", {}),
    ("cloak", { dest := some .battlefield }),
    ("collectEvidence", { dest := some .exile }),
    ("connive", { permanentActor := true }),
    ("convert", {}),
    ("counter", {}),
    ("create", { dest := some .battlefield }),
    ("destroy", { participle := some "destroyed", dest := some .graveyard }),
    ("detain", {}),
    ("discard", { participle := some "discarded", dest := some .graveyard }),
    ("discover", {}),
    ("double", {}),
    ("earthbend", {}),
    ("endure", { permanentActor := true }),
    ("exchange", {}),
    ("exert", {}),
    ("exile", { participle := some "exiled", dest := some .exile }),
    ("explore", { permanentActor := true }),
    ("faceAVillainousChoice", {}),
    ("fateseal", {}),
    ("fight", { permanentActor := true }),
    ("forage", {}),
    ("goad", {}),
    ("harness", { permanentActor := true }),
    ("heal", {}),
    ("incubate", { dest := some .battlefield }),
    ("investigate", { dest := some .battlefield }),
    ("learn", {}),
    ("manifest", { dest := some .battlefield }),
    ("manifestDread", {}),
    ("meld", { dest := some .battlefield }),
    ("mill", { participle := some "milled", dest := some .graveyard }),
    ("monstrosity", { permanentActor := true }),
    ("play", {}),
    ("populate", {}),
    ("proliferate", {}),
    ("recruit", {}),
    ("regenerate", { participle := some "regenerated" }),
    ("reveal", {}),
    ("sacrifice",
      { participle := some "sacrificed", dest := some .graveyard }),
    ("scry", {}),
    ("search",
      { loci := [.battlefield, .graveyard, .exile, .hand, .library, .stack, .command],
        feature := some .librarySearch }),
    ("shuffle", { loci := [.library] }),
    ("support", {}),
    ("surveil", {}),
    ("suspect", {}),
    ("tap", { participle := some "tapped" }),
    ("theRingTemptsYou", {}),
    ("timeTravel", {}),
    ("transform", {}),
    ("triple", {}),
    ("untap", { participle := some "untapped" }),
    ("ventureIntoTheDungeon", {}),
    ("vote", {}),
    ("waterbend", {}) ]

def distinctActLabels : List (KeywordActionLabel × ActFacts) → Bool
  | [] => true
  | f :: fs => !(fs.map (·.1)).elem f.1 && distinctActLabels fs

def distinctAbilityDeedLabels : List (KeywordLabel × ActFacts) → Bool
  | [] => true
  | f :: fs => !(fs.map (·.1)).elem f.1 && distinctAbilityDeedLabels fs

def deedFacts : Deed → Option ActFacts
  | .core _ => some {}
  | .action l => (actFacts.find? (·.1 == l)).map (·.2)
  | .ofAbility k => (abilityDeedFacts.find? (·.1 == k)).map (·.2)

def knownAct (v : Deed) : Bool := (deedFacts v).isSome
def deedFeatureOf (v : Deed) : Option DeedFeature := deedFacts v >>= (·.feature)
def participleOf (v : Deed) : Option String := deedFacts v >>= (·.participle)
def actDestOf (v : Deed) : Option Zone := deedFacts v >>= (·.dest)
def actLociOf (v : Deed) : List Zone := (deedFacts v).elim [] (·.loci)
def actPermanentActorOf (v : Deed) : Bool := (deedFacts v).elim false (·.permanentActor)
/-! ## Discourse Representation Structure (`DRS`) -/

/-- A Zone Change a stamp records: the zone the object left, if known, and the zone it entered
[CR#400.7]. -/
structure ZoneChange where
  origin : Option Zone
  destination : Option Zone
  deriving DecidableEq, Repr, BEq

/-- Provenance: the deed that last acted on a referent, if a deed names it; whether it was on the
battlefield then; the Zone Change that last moved it, if it changed zones, which a move records
whether or not a deed names it ("put", "return" [CR#400.7]); and the deeds `verb` was done by
(`through`), when a keyword ability's verb is done by doing another deed: devouring is
sacrificing [CR#702.82b]. -/
structure Stamp where
  verb : Option Deed
  wasField : Bool
  zoneChange : Option ZoneChange
  through : List Deed := []
  deriving DecidableEq, Repr, BEq

/-- Whether the stamped object changed zones. -/
def Stamp.moved (st : Stamp) : Bool := st.zoneChange.isSome

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

/-- Checker-internal performer metadata for `Action.instruct` handoffs. -/
structure ActorShape where
  kind         : Kind
  isYou        : Bool
  ascribable   : Bool
  refIds       : List ReferentId := []
  deriving DecidableEq, Repr

/-- Domain-sort and characteristic facts of a discourse referent (`semantics-v3.md` §1.2).
`parameterFrame`, `scopeFrame`, `hidden`, `masked`, and `amount` are gone. -/
inductive Payload where
  | object (ty : List CardType) (zone : Option Zone) (prov : Option Stamp) (orig : Option Origin)
      (size : Option Nat)
  | player (chosen : Bool)
  | quality (q : QualitySort)
  | outcome (sort : OutcomeSort)
  | difference
  | letter (l : Letter)
  | turnRef
  /-- An ability object and its remembered location [CR#113.1c,724.1b,724.2b]. -/
  | ability (orig : Option Origin) (zone : Option Zone := some .stack)
  | pile (zone : Option Zone) (size : Option Nat) (face : Option PileFace)
  | join (l r : Payload)
  /-- Checker-private handoff frame: the facts of the player or permanent performing the
  enclosing `Action.instruct` body. It is never a discourse referent. -/
  | actor (shape : ActorShape)
  deriving DecidableEq, Repr

namespace Payload

/-- The kind a payload denotes; an ability on the stack is an object [CR#113.1c]. -/
def kind : Payload → Kind
  | .object .. => .object
  | .player _ => .player
  | .quality q => .quality q
  | .outcome _ | .difference | .actor _ => .outcome
  | .letter l => .letter l
  | .turnRef => .turnRef
  | .ability _ _ => .object
  | .pile .. => .pile
  | .join l r => .join l.kind r.kind

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
  attachmentHost  : Bool := false
  announcedTargetResult : Bool := false
  createdResult : Bool := false
  librarySearchResult : Bool := false
  /-- Published to later abilities of the same text by the ability that acted on it
  (`Ability.textContext`): only a Participial Description with no span written reads it back,
  the ability that reads it being linked to the one that published it [CR#607.1,607.2a]. -/
  linked : Bool := false
  /-- Where a linked referent was published: the line of text that acted on it, by the context's
  `nextId` as the line was published, and the arm taken at each of that line's branch points
  on the way to the deed that acted on it. -/
  linkSite : ReferentId := 0
  linkArms : List (Nat × Nat) := []
  /-- A group's key, which a part taken from it records in `partOf` ("the card that stopped the
  run" is part of the run), so that a group reading counts the part once. Keys survive a zone
  change, as the group and its part move together. -/
  groupKey : Option ReferentId := none
  partOf : Option ReferentId := none
  deriving DecidableEq, Repr

def Referent.kind (b : Referent) : Kind := b.payload.kind
def Referent.zone (b : Referent) : Option Zone := b.payload.zone
def Referent.ty (b : Referent) : List CardType := b.payload.ty
def Referent.size (b : Referent) : Option Nat := b.payload.size
def Referent.face (b : Referent) : Option PileFace := b.payload.face

def Referent.isActorFrame (b : Referent) : Bool :=
  match b.payload with
  | .actor _ => true
  | _ => false

def Referent.isDifference (b : Referent) : Bool :=
  match b.payload with
  | .difference => true
  | _ => false

def Referent.sameCore (a b : Referent) : Bool :=
  a.det == b.det && a.plur == b.plur && a.payload == b.payload && a.idents == b.idents

structure SelectionEvidence where
  kind : Kind
  sourceGroup : Option ReferentId := none
  chosen : Bool := false
  scope : Nat
  deriving DecidableEq, Repr

/-- Explicit Reference Scope frame for `Action.ownScope` (`semantics-v3.md` §1.2, §1.4). -/
structure OwnScopeFrame where
  preEntryIds   : List ReferentId
  maskedIds     : List ReferentId
  callerClause  : Nat
  callerSelectionScope : Nat := 0
  callerPending : List (ReferentId × GramRole) := []
  callerReferentIds : List ReferentId := []
  /-- The caller's pending roles as the body was entered, which leaving the body restores: a
  mention in the body never changes a role the caller can see. -/
  entryPending : List (ReferentId × GramRole) := []
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
  prospectiveTokenTypes : List (List CardType) := []
  /-- The current action describes an event or permission pattern. -/
  actionPattern : Bool := false
  characteristicRead : Bool := false
  historicalReferentIds : List ReferentId := []
  historicalMoveFrom : Option Zone := none
  groupSizeScope : Option (List ReferentId) := none
  selectionEvidence : List SelectionEvidence := []
  selectionScope : Nat := 0
  nextSelectionScope : Nat := 1
  /-- The Binding Identities whose first mention bound nothing: that mention's refusal is the
  one their later mentions would repeat, so they report none of their own. -/
  unboundIdentities : List BindingId := []
  deriving DecidableEq, Repr

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

def outcomeB (s : OutcomeSort) : Referent :=
  { det := .the, plur := .one, payload := .outcome s }
def differenceB : Referent :=
  { det := .the, plur := .one, payload := .difference }
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

def countOnes (k : Kind) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    !b.isActorFrame && !b.isDifference && b.plur.isOne && Kind.lte k b.kind).length

def countDifferences (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b => b.isDifference && b.plur.isOne).length

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

/-- The groups "the rest" and "the other" can be taken from. An object a linked ability
published is read only by a Participial Description (`Referent.linked`), so none is one. -/
def countGroups (k : Kind) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    !b.linked && b.det != .part && b.det != .bare && objGroup k b).length

def countParts (k : Kind) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    !b.linked && b.det == .part && Kind.lte k b.kind).length

def DRS.recordSelection (drs : DRS) (kind : Kind) (sourceGroup : Option ReferentId)
    (chosen : Bool) : DRS :=
  { drs with selectionEvidence :=
      { kind, sourceGroup, chosen, scope := drs.selectionScope } :: drs.selectionEvidence }

def DRS.choiceWasPossible (drs : DRS) (kind : Kind) : Bool :=
  drs.selectionEvidence.any (fun evidence => evidence.scope == drs.selectionScope &&
    evidence.chosen && Kind.lte kind evidence.kind)

def DRS.subsetWasPossible (drs : DRS) (kind : Kind) : Bool :=
  drs.selectionEvidence.any (fun evidence => evidence.scope == drs.selectionScope &&
    Kind.lte kind evidence.kind && evidence.sourceGroup.any (fun id =>
      drs.isVisible id && (drs.findById? id).any (fun ref => objGroup kind ref)))

def enterSelectionScope (drs : DRS) : DRS :=
  { drs with
    selectionScope := drs.nextSelectionScope
    nextSelectionScope := drs.nextSelectionScope + 1 }

def leaveSelectionScope (before after : DRS) : DRS :=
  { after with
    selectionScope := before.selectionScope
    nextSelectionScope := max before.nextSelectionScope after.nextSelectionScope
    selectionEvidence := after.selectionEvidence.filter (fun evidence =>
      evidence.scope != after.selectionScope) }

def theRestOk (k : Kind) (drs : DRS) : Bool :=
  countGroups k drs ≤ 1 && (countParts k drs != 0 || drs.subsetWasPossible k)

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
  countGroups k drs ≤ 1 && countParts k drs != 0 &&
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

def partsClosed (drs : DRS) : DRS :=
  { drs with referents := drs.referents.map fun b =>
      if drs.isVisible b.id && b.det == .part then { b with det := .the } else b }

def restSource (k : Kind) (drs : DRS) : Option Referent :=
  let rec loop : List Referent → Option Referent
    | [] => none
    | b :: bs =>
      if b.linked then loop bs
      else if b.det == .part then
        match loop bs with
        | some s => some s
        | none => if Kind.lte k b.kind then some b else none
      else if objGroup k b then some b
      else loop bs
  loop drs.visibleReferents

def DRS.consumeRest (after before : DRS) (kind : Kind) : DRS :=
  let partIds := before.visibleReferents.filterMap fun ref =>
    if ref.det == .part && Kind.lte kind ref.kind then some ref.id else none
  let sourceGroup := (restSource kind before).map (·.id)
  { after with
    referents := after.referents.map fun ref =>
      if partIds.contains ref.id then { ref with det := .the } else ref
    selectionEvidence := after.selectionEvidence.filter fun evidence =>
      !(evidence.scope == before.selectionScope && Kind.lte kind evidence.kind &&
        evidence.sourceGroup.isSome && evidence.sourceGroup == sourceGroup) }

def zoneOfGroup (k : Kind) (drs : DRS) : Option Zone := restSource k drs >>= (·.zone)
def tyOfGroup (k : Kind) (drs : DRS) : List CardType := (restSource k drs).elim [] (·.ty)
/-- Whether this text has targeted a `k` that "another target" can be other than. A target a
linked ability published is read only by a Participial Description (`Referent.linked`). -/
def anyTargeted (k : Kind) (drs : DRS) : Bool :=
  drs.visibleReferents.any fun b => !b.linked && b.det == .target && Kind.lte k b.kind

def anchorTyOk (t : CardType) : List CardType → Bool
  | [] => true
  | ts => ts.contains t

def settleTargets (drs : DRS) : DRS :=
  { drs with referents := drs.referents.map fun b =>
      if drs.isVisible b.id && b.det == .target then { b with det := .the } else b }

def agreedField {α : Type} (f : α → α → Bool) : Option α → Option α → Option α
  | some x, some y => if f x y then some x else none
  | _, _ => none

def Zone.isPublic : Zone → Bool
  | .hand => false
  | .library => false
  | _ => true

def Referent.isPublic (b : Referent) : Bool :=
  match b.payload with
  | .object _ (some z) _ _ _ => z.isPublic
  | .pile _ _ (some face) => face == .faceUp
  | .pile (some z) _ none => z.isPublic
  | _ => true

def pluralizeBinding (b : Referent) : Referent :=
  match b.det, b.payload with
  | .self, _ | _, .letter _ => b
  | _, _ => { b with plur := .many }

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

def slotZoneOk : SlotCarrier → Option Zone → Bool
  | .permanent, zn => onFieldZone zn
  | .card, zn => isCardZone zn
  | .spell, zn => onStackZone zn

/-- The stamp a deed leaves on an object it acts on where it is (`zone`); no deed, no stamp. -/
def mkStamp : Option Deed → Option Zone → Option Stamp
  | none, _ => none
  | some v, zone => some ⟨some v, onFieldZone zone, none, []⟩

/-- The stamp a move leaves on the object it moved from `origin` to `destination`, whether or not
a deed names the move [CR#400.7]. -/
def moveStamp (deed : Option Deed) (origin destination : Option Zone) : Stamp :=
  ⟨deed, onFieldZone origin, some ⟨origin, destination⟩, []⟩

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
    | .object ty zn _ _ _ =>
      match word with
      | .type t | .subtype (.of t _) => onFieldZone zn && tyIs t ty
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
    | .object ty zn _ og _ =>
      match w with
      -- A subtype is read as its card type [CR#205.3].
      | .type t | .subtype (.of t _) => onFieldZone zn && tyIs t ty
      | .card => isCardZone zn
      | .spell | .subtype (.spell _) => onStackZone zn
      | .permanent => onFieldZone zn
      | .token => onFieldZone zn && isTokenOrigin og
      | .emblem => zn == some .command
      | .copy => isCopyOrigin og
      | .stack => onStackZone zn
      | _ => false
    | .player _ => w == .player
    | pl@(.join _ _) =>
      match w with
      | .type t | .subtype (.of t _) => halfReaches (.type t) pl
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

/-- Whether the word is a Head Noun: an Object Class, Card Type or Subtype an object is named
by [CR#109.1,205.2a,205.3]. -/
def NounWord.isHead : NounWord → Bool
  | .type _ | .subtype _ | .card | .spell | .permanent | .token | .emblem | .ability => true
  | _ => false

/-- A noun word refined by a card type or by "copied", which only an object noun takes. -/
def NounWord.refined : NounWord → Bool
  | .ofType .. | .copied _ => true
  | _ => false

def Reach.kind : Reach → Kind
  | .word w => w.kind
  | .unionHalf w => w.kind
  | _ => .object

def stampedBy (v : Deed) (s : Stamp) : Bool := s.verb == some v || s.through.contains v

/-- Whether `word` names a stamped object by what it was when its deed acted on it: a Card Type
or "permanent" reads the permanent it is, if it is on the battlefield now (one a move put there:
"the creature returned this way"), or else the permanent it was, a departed one by its last known
information [CR#608.2h], while "card" and "spell" read the zone it is in now. -/
def stampedWordOk : NounWord → Stamp → List CardType → Option Zone → Option Origin → Bool
  | .ofType word t, st, ty, zn, og => stampedWordOk word st ty zn og && tyIs t ty
  | .copied word, st, ty, zn, og => stampedWordOk word st ty zn og && isCopyOrigin og
  | .type t, st, ty, zn, _ | .subtype (.of t _), st, ty, zn, _ =>
    (st.wasField || onFieldZone zn) && tyIs t ty
  | .card, _, _, zn, _ => isCardZone zn
  | .spell, _, _, zn, _ | .subtype (.spell _), _, _, zn, _ => onStackZone zn
  | .permanent, st, _, zn, _ => st.wasField || onFieldZone zn
  | _, _, _, _, _ => false

def stampIs (v : Deed) : Option Stamp → Bool
  | none => false
  | some st => stampedBy v st

def Referent.stampedBy (b : Referent) (verb : Deed) : Bool :=
  stampIs verb b.payload.prov ||
    (b.librarySearchResult && deedFeatureOf verb == some .librarySearch)

def markTy (ty : List CardType) (b : Referent) : Referent :=
  match b.payload with
  | .object old zn st og sz =>
    { b with payload := .object (mergeTypes old ty) zn st og sz }
  | _ => b

def survivesShuffle (b : Referent) : Bool :=
  match b.payload with
  | .object _ (some .library) (some st) _ _ =>
    b.librarySearchResult || (st.verb.bind deedFeatureOf) == some .librarySearch
  | .object _ (some .library) none _ _ => false
  | _ => true

/-- Stage 1 (`J1`): sort, plurality, and zone/carrier reach (`semantics-v3.md` §1.5). -/
def reaches (r : Reach) (pl : Plurality) (b : Referent) : Bool :=
  match r with
  | .bare => Kind.lte .object b.kind && pl.isOne == b.plur.isOne
  | .atSlot sl => Kind.lte .object b.kind && pl.isOne == b.plur.isOne && slotZoneOk sl b.zone
  | .word w => pl.isOne == b.plur.isOne && wordNow w b
  | .unionHalf w => pl.isOne == b.plur.isOne && b.payload.joined && halfReaches w b.payload

/-- A characteristic read may name a departed permanent by its former type
[CR#608.2h]. This changes accessibility, not the referent's current zone. -/
def reachesCharacteristic (r : Reach) (pl : Plurality) (b : Referent) : Bool :=
  reaches r pl b || match r, b.payload with
    | .word w, .object ty zn (some st) og _ =>
      b.det != .self && pl.isOne == b.plur.isOne && st.moved &&
        stampedWordOk w st ty zn og
    | _, _ => false

/-- Stage 2 (`J2`): whether `id` is a clause-mate co-argument in the current clause. -/
def DRS.isClauseMate (drs : DRS) (id : ReferentId) : Bool :=
  drs.pendingRoles.any fun (pid, _) => pid == id

/-- Two-stage anaphor candidate resolution (`semantics-v3.md` §1.5):
- Bare pronouns (`.bare`, `.atSlot _`) use `D′`: filter by `J1` and exclude `J2` clause-mates
  (when `inArgSlot`), then select the nearest clause (highest `clause`) and highest grammatical
  role (`subj < obj < other`); a unique winner resolves, while `0` or `≥ 2` winners refuse.
- Demonstratives / definite NPs (`.word _`, `.unionHalf _`) use `AC>R1`: filter by
  `J1`, exclude `J2` clause-mates (`A`, when `inArgSlot`), exclude any referent already resolved
  by a bare pronoun in the current clause (`C`, `pronounResolved == true`), and pick the most
  recently introduced candidate (`R1`, highest `id`).
- Neither reaches an object a linked ability published (`Referent.linked`): only a Participial
  Description reads one back. -/
def resolveCandidates (r : Reach) (pl : Plurality) (drs : DRS) (inArgSlot : Bool := false) :
    List Referent :=
  let j1 := drs.visibleReferents.filter fun ref => !ref.linked && (
    (if drs.characteristicRead then reachesCharacteristic r pl ref else reaches r pl ref) ||
      (drs.historicalMoveFrom == some .battlefield &&
        drs.historicalReferentIds.contains ref.id && reachesCharacteristic r pl ref))
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
  | .word _ | .unionHalf _ =>
      let afterA := if inArgSlot then j1.filter (fun b => !drs.isClauseMate b.id) else j1
      let afterC := afterA.filter (fun b => !b.pronounResolved)
      match afterC with
      | [] => []
      | h :: t =>
          let best := t.foldl (fun acc b => if b.id > acc.id then b else acc) h
          [best]

/-- What a Participial Description reads back by: the deed its clause names ("the exiled card"),
or, where no deed names the clause's move, that Zone Change, by the destination it writes and the
origin, if one is written ("the card returned to the battlefield this way" [CR#400.7]). -/
inductive ParticipialAnchor where
  | deed (deed : Deed)
  | zoneChange (origin : Option Zone) (destination : Zone)
  deriving DecidableEq, Repr

/-- Whether `b`'s stamp is what `anchor` reads back: the deed, or a Zone Change into the
anchor's destination from its origin, if it names one. -/
def Referent.anchoredBy (b : Referent) : ParticipialAnchor → Bool
  | .deed deed => b.stampedBy deed
  | .zoneChange origin destination =>
    match b.payload.prov.bind (·.zoneChange) with
    | some change =>
      change.destination == some destination && origin.all (fun zone => change.origin == some zone)
    | none => false

/-- The objects `anchor` reads back that a Participial Description admits: of its number (`plur`;
none for a group determiner, which reads members of any number), within its span (`thisWay`:
only objects the current ability stamped [CR#607.1c]; no span written: also those a linked
ability published, where a singular "the" reads a group the linked ability acted on
[CR#607.2a,607.3]), named by `word` as `stampedWordOk` reads it, and in each of `zones`. -/
def participialCandidates (anchor : ParticipialAnchor) (plur : Option Plurality)
    (word : Option NounWord) (thisWay : Bool) (drs : DRS) (zones : List Zone := []) :
    List Referent :=
  drs.visibleReferents.filter fun b =>
    !b.isActorFrame && Kind.lte .object b.kind && b.anchoredBy anchor &&
      (!thisWay || !b.linked) &&
      (match plur with
       | none => true
       | some .one => b.plur.isOne || (!thisWay && b.linked)
       | some .many => !b.plur.isOne) &&
      (match word, b.payload with
       | none, _ => true
       | some w, .object ty zn (some st) og _ => stampedWordOk w st ty zn og
       | some _, _ => false) &&
      zones.all (fun zone => b.zone == some zone)

/-- How a Participial Description reads the candidates it found: the referents it denotes, or
how many it found where it denotes none. A group determiner ("each creature exiled this way")
reads every candidate together. "The" reads one place a deed acted: one object the current
ability acted on, or what one linked ability acted on, where the arms that ability may take
instead of each other ("you may exile …; if you don't, exile …") are one place, read by whichever
arm acted [CR#607.2a]. -/
def participialRead (plur : Option Plurality) (found : List Referent) :
    Except Nat (List Referent) :=
  if found.isEmpty then .error 0 else
  match plur with
  | none =>
    -- A part taken from a group that is read too is read as one of the group's members.
    .ok (found.filter fun ref => !ref.partOf.any (fun key => found.any (·.groupKey == some key)))
  | some _ =>
    let sites := (found.map fun ref => if ref.linked then (true, ref.linkSite) else (false, ref.id))
      |>.eraseDups
    -- Two objects of one site are alternatives when they lie on different arms of a branch point.
    let exclusive := fun (a b : Referent) =>
      a.linkArms.any fun (point, arm) => b.linkArms.any fun (point', arm') =>
        point == point' && arm != arm'
    let alternatives := found.all fun a => found.all fun b => a.id == b.id || exclusive a b
    match sites with
    | [(false, _)] => .ok found
    | [(true, _)] => if alternatives then .ok found else .error found.length
    | _ => .error sites.length

/-- How many winning antecedents `r` resolves to in `drs`; a read is sound at `1`. -/
def countReach (r : Reach) (pl : Plurality) (drs : DRS) (inArgSlot : Bool := false) : Nat :=
  (resolveCandidates r pl drs inArgSlot).length

def firstReach (r : Reach) (pl : Plurality) (drs : DRS) (inArgSlot : Bool := false) :
    Option Referent :=
  (resolveCandidates r pl drs inArgSlot).head?

def zoneOfReach (r : Reach) (pl : Plurality) (drs : DRS) : Option Zone :=
  firstReach r pl drs >>= (·.zone)
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

/-- Open a keyword action body's own Reference Scope (`Action.ownScope`, `semantics-v3.md` §1.4):
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
    callerSelectionScope := drs.selectionScope
    callerPending := drs.pendingRoles
    entryPending  := drs.pendingRoles
  }
  { (enterSelectionScope drs) with
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
        selectionScope := topFrame.callerSelectionScope
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
        selectionScope := before.selectionScope
        pendingRoles  := addedPending ++ before.pendingRoles
        resolvedIds   := before.resolvedIds }

/-- Close an own Reference Scope (`Action.ownScope`, `semantics-v3.md` §1.4): pops `topFrame`,
drops definition-internal scratch referents (`.part` and `.player _`), and stamps surviving
referents introduced during `topFrame` with `clause := topFrame.callerClause` and `role := .obj`.
A mention in the body never changes a role the caller can see: the caller's pending roles are
the ones it had as the body was entered, and an argument the body read in its caller
(`callerReferentIds`) is pending as the call's object, whatever role its mentions in the body
took. -/
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
      let keptIds := kept.map (·.id)
      let arguments := topFrame.callerReferentIds.filter fun id =>
        keptIds.contains id && !topFrame.entryPending.any (·.1 == id)
      { drs with
        referents     := kept
        scopeStack    := outerFrames
        selectionEvidence := drs.selectionEvidence.filter (fun evidence =>
          evidence.scope != drs.selectionScope)
        selectionScope := topFrame.callerSelectionScope
        pendingRoles  := topFrame.entryPending ++ arguments.map (·, GramRole.obj)
        currentClause := topFrame.callerClause }

/-! ## Binding Identity (`firstMention` / `laterMention`) -/

/-- The most recent visible referent recorded under `id`, if any (`semantics-v3.md` §1.4). -/
def identityBinding (id : BindingId) (drs : DRS) : Option Referent :=
  drs.visibleReferents.find? fun b => b.idents.contains id

/-- Record referent `refId` under `id` as well. -/
def recordIdentityById (id : BindingId) (drs : DRS) (refId : ReferentId) : DRS :=
  drs.updateById refId fun b => { b with idents := mergeIdents b.idents [id] }

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

def countTokenSpecs (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    b.det != .self && isTokenOrigin b.payload.orig).length

def tyOfThoseAny (w : NounWord) (drs : DRS) : List CardType :=
  match drs.visibleReferents.find? (wordNow w) with
  | some b => b.ty
  | none => []

def countChoosers (drs : DRS) : Nat := countOnes .player drs + countManys .player drs

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

/-- A copy of an ability is itself an ability [CR#707.10]. -/
def copyPayloadIn : Kind → Bool → List CardType → Option Zone → Payload
  | .object, true, _, z => .ability (some .copy) z
  | .object, false, ty, z => .object ty z none (some .copy) none
  | .join l r, ab, ty, z => .join (copyPayloadIn l ab ty z) (copyPayloadIn r ab ty z)
  | .player, _, _, _ => .player false
  | .quality q, _, _, _ => .quality q
  | _, _, _, _ => .difference

def CopySort.landsIn : CopySort → Option Zone → Option Zone
  | .fromStack, _ => some .stack
  | .fromCardZone, z => z

/-! ## Mana -/

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

def Delta.amount {α : Type} : Delta α → α
  | .up x => x
  | .down x => x
  | .set x => x

/-- Whether the Delta sets its value rather than raising or lowering it. -/
def Delta.isSet {α : Type} : Delta α → Bool
  | .set _ => true
  | _ => false

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

/-- A designation that a player or an object holds. -/
def designationHolderOk (label : DesignationLabel) : Bool :=
  match label.scope with
  | some (.heldBy _) => true
  | _ => false

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
  | .type _ | .subtype (.of ..) | .card | .spell | .permanent | .token => some .battlefield
  | .copy => some .stack
  | _ => none

def NounWord.attachHostTy : NounWord → List CardType
  | .ofType word t => mergeTypes word.attachHostTy [t]
  | .copied word => word.attachHostTy
  | .type t | .subtype (.of t _) => [t]
  | _ => []

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
  | .boundCounter => .join .object .player
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

def axesAt (k : Kind) : List ProjAxis → Bool
  | [] => false
  | as => as.all (·.scope == k)

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

def CardType.isInstantOrSorcery : CardType → Bool
  | .instant | .sorcery => true
  | _ => false

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
