import Semantics.Definitions.CardTypes

/-!
# Semantics.Check.Words

The checker's vocabulary and Discourse Representation Theory (`DRS`) layer (`semantics-v3.md`
§1.2–§1.5). Holds the structured `DRS` state (`ReferentId`, `GramRole`, `Payload`, `Referent`,
`OwnScopeFrame`, `DRS`), the two-stage anaphor resolver (`J1` accessibility, `J2` clause-mate
exclusion, `D′` bare pronoun resolution, `AC>R1` demonstrative resolution, Principle A
reflexives, and the stamped readback of Participial Descriptions), the Reference Scope frame
stack (`enterOwnScope`, `enterCaller`, `leaveCaller`, `leaveOwnScope`), and the leaf predicates
the phrase and ability layers consult.
-/

namespace Semantics

/-- A referent's determiner, as the checker records how its phrase introduced it. -/
inductive Determiner where
  | target | a | each | all | the | part | count | self
  deriving DecidableEq, Repr

/-! ## Card types and stats -/

/-- Canonical order for remembered type facts, the declared order (`cardTypes`); order in the
written modifiers is immaterial. -/
def normalizeTypes (types : List CardType) : List CardType :=
  cardTypes.filter (· ∈ types)

def mergeTypes (left right : List CardType) : List CardType :=
  normalizeTypes (left ++ right)

def commonTypes (left right : List CardType) : List CardType :=
  normalizeTypes (left.filter (· ∈ right))

/-- Whether one of `types` has `stat`, read off the types' own `stats`: power and toughness for a
creature [CR#302.4], loyalty for a planeswalker [CR#306.5], defense for a battle [CR#310.4]. -/
def typesHaveStat (types : List CardType) (stat : Stat) : Bool :=
  types.any (·.stats.elem stat)

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
  | .subtype host scope => host == .land && scope == .basicOnly
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

/-! ## Deeds the checker reads -/

/-- Whether a Permanent performs `v`: written with no Handoff to a Permanent, the source
Permanent performs it (`Deed.action`'s `permanentActor`) [CR#701.44a,701.63a]. -/
def Deed.permanentActor : Deed → Bool
  | .action _ permanentActor => permanentActor
  | _ => false

/-- The deed a search (`Action.search`) stamps on what it finds [CR#701.23a]: a card found by
searching a library is read back as searched for, and survives the shuffle that follows, whatever
later deeds stamp it. -/
def searchDeed : Deed := .action "search"
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
sacrificing [CR#702.82b]; and whether an earlier move took it off the battlefield
(`leftField`), as the creature that died before it was exiled. -/
structure Stamp where
  verb : Option Deed
  wasField : Bool
  zoneChange : Option ZoneChange
  through : List Deed := []
  leftField : Bool := false
  deriving DecidableEq, Repr, BEq

/-- Whether the stamped object changed zones. -/
def Stamp.moved (st : Stamp) : Bool := st.zoneChange.isSome

/-- What a Participial Description reads back by: the deed its clause names ("the exiled card"),
or, where no deed names the clause's move, that Zone Change, by the destination it writes and the
origin, if one is written ("the card returned to the battlefield this way" [CR#400.7]). -/
inductive ParticipialAnchor where
  | deed (deed : Deed)
  | zoneChange (origin : Option Zone) (destination : Zone)
  deriving DecidableEq, Repr

/-- The class of an event, as a lookback that names no Deed and writes no Zone Change reads it
back ("the damage dealt this way", "the life lost this way", "if you paid life this way"): one
class per event head (`Action.eventHeadOk`), a status by the status it sets, a change of life
by whether it is gained or lost [CR#119.3], an exposure by whether cards are looked at or
revealed, damage or counters divided as dealt or put, and a payment of a cost, however written
[CR#118.1]. -/
inductive EventVerb where
  | damageDealt | damageMarked | damageCleared | damagePrevented | damageRedirected
  | status (status : Status)
  | turnedOver | turnedFaceDown | combat | attacked | blocked | attackedUnblocked
  | targeted | tappedForMana | crimeCommitted | attachment (move : AttachMove)
  | controlChanged | designationGained | unlocked | fullyUnlocked | concluded
  | triggered | crewed | saddled
  | chose | voted | copied | life (sort : OutcomeSort) | exchanged | manaAdded | drew
  | exposed (verb : ExposeVerb) | searched | shuffled | coinsFlipped | diceRolled
  | created | countersPut | countersRemoved | moved | paid
  | procedure (kind : Procedure) | related | merged
  | untapSkipped | partSkipped | turnPartAdded
  deriving DecidableEq, Repr

/-- An event the current spell or ability's own text caused, which a later "this way" in that
text reads back ("If you search your library this way, shuffle" [CR#608.2c]): the Deed that names
it, if one does; the Zone Change it writes, if it writes one; its classes (`EventVerb`), which
class an event naming neither (dealing damage, gaining life); the kind of its Actor, if known;
the kind of what it moved or acted on, if it names one; and whether a permission, not an
instruction, wrote it ("You may cast spells from the top of your library. If you cast a spell
this way, …"). It is recorded whether or not the event will happen: a declined optional action,
an untaken branch, or a permission no one uses still wrote it (`semantics-v3.md` §3.8). -/
structure CausedEvent where
  deed : Option Deed
  zoneChange : Option ZoneChange
  verbs : List EventVerb
  performer : Option Kind
  patient : Option Kind := none
  permitted : Bool := false
  deriving DecidableEq, Repr

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

/-- Two members of a group taken together: objects stay an object, with the facts both share, and
players a player; members of different sorts join. -/
def together : Payload → Payload → Payload
  | .object t z p o _, .object t' z' p' o' _ =>
    .object (joinSeed t t') (if z == z' then z else none) (if p == p' then p else none)
      (if o == o' then o else none) none
  | .player c, .player c' => .player (c && c')
  | l, r => .join l r

/-- The payload of the group its members' payloads make up, if there are any. -/
def groupOf : List Payload → Option Payload
  | [] => none
  | first :: rest => some (rest.foldl together first)

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
  /-- A mention of the Source ("this", "this creature") or of "you": the Contrast Anchor a bare
  "another"/"other" falls back to ([CR#109.5]), and so never an earlier mention or clause
  participant it is anchored on. -/
  defaultAnchor : Bool := false
  /-- Introduced inside a Condition (an intervening "if", a `doIf` condition, `exists_`): a
  hypothetical, never a Contrast Anchor. -/
  fromCondition : Bool := false
  /-- Introduced as the subject of an existential ("if there is a …", "if two or more creatures
  are tied …"): read back in the consequent and closed after it (`semantics-v3.md` §1.3, box 2). -/
  existentialDomain : Bool := false
  /-- An existential's domain a replacement's condition published ("If there is …, instead put
  two of them …"): the replacement does what the replaced action would have done, to what that
  action read, so the domain ranks below ordinary earlier mentions (`preferOrdinary`; owner ruling
  2026-10-22, batch 4). -/
  replacementDomain : Bool := false
  /-- Whether its description says "you control" (`some true`) or that an opponent, or not you,
  controls it (`some false`); `none` when it says neither. -/
  controlledByYou : Option Bool := none
  /-- The ids this object's referent had before the Zone Changes the checker applied to it,
  latest first: a move is a new object [CR#400.7], read under a fresh id. -/
  movedFrom : List ReferentId := []
  /-- Introduced by a description that names it a source ("a source of your choice", "whenever a
  source deals damage"): the only antecedent "that source" reads back [CR#609.7a]. -/
  introducedAsSource : Bool := false
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
  /-- The caller's Source type evidence (`DRS.sourceTypes`), which the caller's arguments read
  and leaving the body restores. -/
  callerSourceTypes : List CardType := []
  deriving DecidableEq, Repr

/-- What a bare "another"/"other" contrasts with, as the checker resolved it (`semantics-v3.md`
§0.1): the current member of the distributive group its clause's subject names, the nearest
earlier mention in scope, a participant of the same clause read forward, or, failing those, the
Source for an Object or "you" for a Player ([CR#109.5]). -/
inductive ContrastAnchor where
  /-- The current member of the distributive group mention `id` (step 1): "Attacking creatures
  you control get +1/+0 … for each other attacking creature" counts, for each affected creature,
  the attacking creatures other than that creature. The affected set is fixed when the effect
  begins ([CR#611.2c]), but the amount is a function of each member. -/
  | member (id : ReferentId)
  /-- An earlier mention as a whole (step 2). -/
  | mention (id : ReferentId)
  /-- A participant the clause introduces after its agent, by its facts: it takes its id only
  when the clause steps it, after the agent. -/
  | participant (facts : Payload)
  | source
  | you
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
  /-- The Source's Card Types from its card frame; empty when unknown, as in a Definition. -/
  sourceTypes : List CardType := []
  /-- The Card Types the Head Noun of the enclosing conjunction names, read by its bare
  "another"/"other". -/
  relativeTypes : List CardType := []
  /-- The zone the Head Noun of the enclosing conjunction places its referent in. -/
  relativeZone : Option Zone := none
  /-- Whether the enclosing conjunction says "you control" it (`some true`) or that an opponent,
  or not you, controls it (`some false`). -/
  relativeControl : Option Bool := none
  /-- A mention of the Source coordinated before the phrase being stepped ("this card and up to
  one other target creature card"): that phrase's Contrast Anchor. -/
  coordinatedSelf : Option ReferentId := none
  /-- Referents the predicate being stepped describes (the subject of `is`), never its Contrast
  Anchor. -/
  contrastExcluded : List ReferentId := []
  /-- The participants a clause introduces after its agent, read forward by the agent's bare
  "another"/"other" (step 3). -/
  clauseParticipants : List Referent := []
  /-- The distributive group the subject of the clause being stepped names ("each …", "all …",
  a bare plural): a bare "another"/"other" in the rest of that clause contrasts with the group's
  current member (step 1). `none` outside such a clause, and inside a nested clause whose subject
  names no such group. -/
  distributiveSubject : Option ReferentId := none
  /-- Contrast Anchors resolved so far, latest first. -/
  contrastAnchors : List ContrastAnchor := []
  /-- The objects a `withContinuation` body or arm stamped ("you may exile it"), escaping it only
  to Participial Descriptions ("each card exiled this way"), which range over the events that
  happened (owner ruling 2026-10-22). A target an optional body or one of its arms announced is
  an ordinary referent as announced; only its stamp from the body or arm is here, under its id
  (owner ruling 2026-10-23). They are not `referents`, so no pronoun, group, count or Contrast
  Anchor reads them; `DRS.participialReferents` does, and a Participial Description
  that reads one mentions it, moving it into `referents` (in place of the earlier object of its
  id, if it is one). -/
  participialOnly : List Referent := []
  /-- The delayed triggered abilities a resolving `delay` created, latest first, each recorded
  under the Binding Identity `delay.trigger`. They are not `referents`: no card prints a mention
  of one, so no pronoun, word, group or count reads them; only a later mention of that identity
  does (`laterMention`), as the special action that stops one from triggering names it
  [CR#116.2c]. -/
  delayedTriggers : List Referent := []
  /-- The events the current spell or ability's text caused so far, latest first, its costs
  included (`CausedEvent`): a `thisWay` lookback reads them back. Another ability's events,
  even a linked one's, are never among them, since "this way" is no linking phrase
  [CR#607.1,607.2a]. -/
  causedEvents : List CausedEvent := []
  /-- The events that trigger the enclosing triggered ability (its trigger event, a delayed
  ability's event, a reflexive ability's "when you do"): a `triggering` lookback reads them back
  [CR#603.2,603.12]. -/
  triggerEvents : List CausedEvent := []
  /-- The anchor of the Participial Description being read, whose anchoring clause reads the
  referents its deed stamped rather than the events recorded (`causedEvents`). -/
  participialAnchorRead : Option ParticipialAnchor := none
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

/-- The referents a Participial Description reads: those an optional body stamped
(`DRS.participialOnly`), then the visible ones. An earlier object the body stamped keeps its id,
so it may appear twice, once as it stood before the body and once after. -/
def DRS.participialReferents (drs : DRS) : List Referent :=
  drs.participialOnly.filter (fun r => drs.isVisible r.id) ++ drs.visibleReferents

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

/-- `refs` with each part dropped whose group is among them, so that a group reading counts the
part once. -/
def wholeGroups (refs : List Referent) : List Referent :=
  refs.filter fun ref => !ref.partOf.any (fun key => refs.any (·.groupKey == some key))

def countManysAny (drs : DRS) : Nat :=
  (wholeGroups (drs.visibleReferents.filter fun b => !b.isActorFrame && !b.plur.isOne)).length

def countManys (k : Kind) (drs : DRS) : Nat :=
  (drs.visibleReferents.filter fun b =>
    !b.isActorFrame && !b.plur.isOne && Kind.lte k b.kind).length

def objGroup (k : Kind) (b : Referent) : Bool := Kind.lte k b.kind && !b.plur.isOne

/-- `refs` without the existential domains replacement conditions published
(`Referent.replacementDomain`) when an ordinary mention is among them: such a domain ranks below
earlier mentions, and is read only where none is in scope (owner ruling 2026-10-22, batch 4). Any
other existential's domain ranks as an ordinary mention, the nearest ("If two or more permanents
a player controls are tied for greatest, their controller chooses one of them"). -/
def preferOrdinary (refs : List Referent) : List Referent :=
  if refs.any (!·.replacementDomain) then refs.filter (!·.replacementDomain) else refs

/-- The groups "the rest" and "the other" can be taken from. An object a linked ability
published is read only by a Participial Description (`Referent.linked`), so none is one. A
domain a replacement's existential condition published counts only where no ordinary group is
in scope (`preferOrdinary`). -/
def countGroups (k : Kind) (drs : DRS) : Nat :=
  (preferOrdinary (wholeGroups (drs.visibleReferents.filter fun b =>
    !b.linked && b.det != .part && objGroup k b))).length

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
  ((preferOrdinary ((wholeGroups drs.visibleReferents).filter fun b =>
    b.det != .part && objGroup k b)).head?).bind (·.size)

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
def anchorTyOk (t : CardType) : List CardType → Bool
  | [] => true
  | ts => ts.contains t

/-- Known Card Types fit a head's: a head naming no Card Type, or an anchor of unknown type, fits;
otherwise the anchor's types include one the head names. -/
def anchorTyFits (headTypes anchorTypes : List CardType) : Bool :=
  headTypes.isEmpty || headTypes.any (fun type => anchorTyOk type anchorTypes)

/-- Two kinds a phrase can denote one thing under: either is within the other. -/
def Kind.overlaps (a b : Kind) : Bool := Kind.lte a b || Kind.lte b a

/-- Zones fit unless both are known and differ. -/
def zonesFit : Option Zone → Option Zone → Bool
  | some a, some b => a == b
  | _, _ => true

/-- Controller restrictions fit unless both are known and contradict ("you control" against "an
opponent controls"). -/
def controlFits : Option Bool → Option Bool → Bool
  | some a, some b => a == b
  | _, _ => true

/-- The Source's zone as its Card Types tell it: an instant or sorcery is a spell on the stack;
any other Source may function from more than one zone, so its zone is unknown. -/
def sourceZone (types : List CardType) : Option Zone :=
  if !types.isEmpty && types.all (·.instantOrSorcery) then some .stack
  else none

/-- Whether `b` can be the Contrast Anchor of a bare "another"/"other" of kind `k` whose Head Noun
names `headTypes` in `headZone`, under the controller restriction `headControl` (steps 1 to 3):
compatible in kind, Card Type, zone and controller; not the
Source or "you", which are the fallback (step 4); and not a hypothetical a Condition introduced.
A referent a linked ability published is read only by a Participial Description
(`Referent.linked`). -/
def Referent.anchorsContrast (k : Kind) (headTypes : List CardType) (headZone : Option Zone)
    (headControl : Option Bool) (b : Referent) : Bool :=
  !b.linked && !b.defaultAnchor && !b.fromCondition && !b.isActorFrame &&
    Kind.overlaps k b.kind && anchorTyFits headTypes b.ty && zonesFit headZone b.zone &&
    controlFits headControl b.controlledByYou

/-- The later of two candidate anchors by DRS recency: the clause of the latest mention, then
the order of introduction. -/
def Referent.recentOver (a b : Referent) : Bool :=
  a.clause > b.clause || (a.clause == b.clause && a.id > b.id)

/-- Resolve the Contrast Anchor of a bare "another"/"other" of kind `k` whose Head Noun names
`headTypes` in `headZone` (`semantics-v3.md` §0.1). A mention of the Source coordinated with the
phrase is its anchor ("this card and up to one other target creature card"). Otherwise: (1) the
current member of the compatible distributive group the clause's subject names
(`distributiveSubject`): the affected set is fixed as the effect begins ([CR#611.2c]), but the
amount is a function of each member; (2) the nearest
earlier compatible mention in scope, never the described thing (`contrastExcluded`); (3) failing
that, the first compatible particular the same clause introduces after its agent
(`clauseParticipants`); (4) otherwise the Source, for an Object head in a zone the Source may be
in and not restricted to a controller other than "you", whatever its Card Types, or "you", for a
Player head ([CR#109.5]). -/
def DRS.contrastAnchor? (drs : DRS) (k : Kind) (headTypes : List CardType)
    (headZone : Option Zone) (headControl : Option Bool) : Option ContrastAnchor :=
  match drs.coordinatedSelf with
  | some id => some (.mention id)
  | none =>
  let group := drs.distributiveSubject.bind fun id =>
    if drs.isVisible id && !drs.contrastExcluded.contains id then drs.findById? id else none
  match group.filter (·.anchorsContrast k headTypes headZone headControl) with
  | some b => some (.member b.id)
  | none =>
  let earlier := drs.visibleReferents.filter fun b =>
    !drs.contrastExcluded.contains b.id && b.anchorsContrast k headTypes headZone headControl
  let nearest := earlier.foldl (fun best b => match best with
    | some a => if b.recentOver a then some b else some a
    | none => some b) none
  match nearest with
  | some b => some (.mention b.id)
  | none =>
    match drs.clauseParticipants.find? (·.anchorsContrast k headTypes headZone headControl) with
    | some b => some (.participant b.payload)
    | none =>
      if k == .player then some .you
      else if Kind.overlaps k .object && zonesFit headZone (sourceZone drs.sourceTypes) &&
          controlFits headControl (some true) then
        some .source
      else none

def settleTargets (drs : DRS) : DRS :=
  { drs with referents := drs.referents.map fun b =>
      if drs.isVisible b.id && b.det == .target then { b with det := .the } else b }

def agreedField {α : Type} (f : α → α → Bool) : Option α → Option α → Option α
  | some x, some y => if f x y then some x else none
  | _, _ => none

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
  | some v, zone => some { verb := some v, wasField := onFieldZone zone, zoneChange := none }

/-- The stamp a move leaves on the object it moved from `origin` to `destination`, whether or not
a deed names the move [CR#400.7]; `leftField` when an earlier move took it off the battlefield. -/
def moveStamp (deed : Option Deed) (origin destination : Option Zone) (leftField : Bool := false) :
    Stamp :=
  { verb := deed, wasField := onFieldZone origin, zoneChange := some ⟨origin, destination⟩,
    leftField }

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
  -- Introduced as a source, in any zone [CR#609.7a].
  | .source => b.introducedAsSource && b.kind == .object
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
    (b.librarySearchResult && verb == searchDeed)

def markTy (ty : List CardType) (b : Referent) : Referent :=
  match b.payload with
  | .object old zn st og sz =>
    { b with payload := .object (mergeTypes old ty) zn st og sz }
  | _ => b

def survivesShuffle (b : Referent) : Bool :=
  match b.payload with
  | .object _ (some .library) (some st) _ _ =>
    b.librarySearchResult || st.verb == some searchDeed
  | .object _ (some .library) none _ _ => false
  | _ => true

/-- Stage 1 (`J1`): sort, plurality, and zone/carrier reach (`semantics-v3.md` §1.5). -/
def reaches (r : Reach) (pl : Plurality) (b : Referent) : Bool :=
  match r with
  | .bare => Kind.lte .object b.kind && pl.isOne == b.plur.isOne
  | .atSlot sl => Kind.lte .object b.kind && pl.isOne == b.plur.isOne && slotZoneOk sl b.zone
  | .word w => pl.isOne == b.plur.isOne && wordNow w b
  | .unionHalf w => pl.isOne == b.plur.isOne && b.payload.joined && halfReaches w b.payload

/-- A characteristic read may name a departed permanent by its former type, by its last known
information [CR#608.2h], also after a later move ("exile it. If you do, create a token that's a
copy of that creature"). This changes accessibility, not the referent's current zone. -/
def reachesCharacteristic (r : Reach) (pl : Plurality) (b : Referent) : Bool :=
  reaches r pl b || match r, b.payload with
    | .word w, .object ty zn (some st) og _ =>
      b.det != .self && pl.isOne == b.plur.isOne && st.moved &&
        stampedWordOk w { st with wasField := st.wasField || st.leftField } ty zn og
    | _, _ => false

/-- Stage 2 (`J2`): whether `id` is a clause-mate co-argument in the current clause. -/
def DRS.isClauseMate (drs : DRS) (id : ReferentId) : Bool :=
  drs.pendingRoles.any fun (pid, _) => pid == id

/-- `D′`'s clause for `ref`: the current clause when `ref` is re-mentioned in a direct argument
slot of it (buffered in `pendingRoles`), otherwise the clause it was introduced or last
foregrounded in (`semantics-v3.md` §1.5, owner ruling 2026-10-21). -/
def DRS.rankingClause (drs : DRS) (ref : Referent) : Nat :=
  if drs.isClauseMate ref.id then drs.currentClause else ref.clause

/-- `D′`'s role rank for `ref`: its best direct-argument slot in the current clause when it is
re-mentioned there, otherwise the role it was introduced or last foregrounded in. -/
def DRS.rankingRole (drs : DRS) (ref : Referent) : Nat :=
  match drs.pendingRoles.filter (·.1 == ref.id) with
  | [] => ref.role.rank
  | slots => slots.foldl (fun acc (_, role) => Nat.min acc role.rank) 2

/-- Two-stage anaphor candidate resolution (`semantics-v3.md` §1.5):
- Bare pronouns (`.bare`, `.atSlot _`) use `D′`: filter by `J1` and exclude `J2` clause-mates
  (when `inArgSlot`), then select the nearest clause (highest `rankingClause`) and highest
  grammatical role (`rankingRole`, `subj < obj < other`); a unique winner resolves, while `0` or
  `≥ 2` winners refuse. A referent re-mentioned in a direct argument slot of the current clause
  ranks there, so "That creature deals damage equal to its power …" reads "its" as that
  creature; in an argument slot such a referent is a `J2` clause-mate and is already excluded.
- Demonstratives / definite NPs (`.word _`, `.unionHalf _`) use `AC>R1`: filter by
  `J1`, exclude `J2` clause-mates (`A`, when `inArgSlot`), prefer a referent not already resolved
  by a bare pronoun in the current clause (`C`, `pronounResolved == true`: such a referent is
  excluded only when another candidate remains, so "its controller creates … copies of that
  creature" corefers), and pick the most recently introduced candidate (`R1`, highest `id`).
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
      -- A plural pronoun reads a replacement condition's existential domain only where no
      -- earlier mention fits.
      let cands := if pl.isOne then cands else preferOrdinary cands
      match cands with
      | [] => []
      | _ =>
          let maxClause := cands.foldl (fun acc b => Nat.max acc (drs.rankingClause b)) 0
          let inClause := cands.filter (drs.rankingClause · == maxClause)
          let minRank := inClause.foldl (fun acc b => Nat.min acc (drs.rankingRole b)) 2
          -- A conjunct is not a second antecedent beside its coordination's group.
          wholeGroups (inClause.filter (drs.rankingRole · == minRank))
  | .word _ | .unionHalf _ =>
      let afterA := if inArgSlot then j1.filter (fun b => !drs.isClauseMate b.id) else j1
      -- `C` is a preference: a pronoun's referent stays a candidate when nothing else does.
      let afterC := match afterA.filter (fun b => !b.pronounResolved) with
        | [] => afterA
        | kept => kept
      let afterC := if pl.isOne then afterC else preferOrdinary afterC
      match afterC with
      | [] => []
      | h :: t =>
          let best := t.foldl (fun acc b => if b.id > acc.id then b else acc) h
          [best]

/-- Whether `event` is one a lookback whose action is read back by `anchor` (or, naming no
Deed and writing no Zone Change, by its classes `verbs`) looks back at: the same Deed, or casting
a card one played, since playing a card casts it unless it is a land ("You may play those cards
this turn. If you cast a spell this way, …" [CR#701.18b]); a Zone Change into the anchor's
destination, from its origin if both name one; or a shared class. -/
def CausedEvent.readBy (event : CausedEvent) (anchor : Option ParticipialAnchor)
    (verbs : List EventVerb) : Bool :=
  match anchor with
  | some (.deed deed) =>
    event.deed == some deed ||
      (deed == .action "cast" && event.deed == some (.action "play"))
  | some (.zoneChange origin destination) =>
    event.zoneChange.any fun change =>
      change.destination == some destination &&
        origin.all (fun zone => change.origin.all (· == zone))
  | none => event.verbs.any verbs.contains

/-- Whether an event's participant (its Actor, or what it moved or acted on) may be what a
lookback's subject names: either is unknown, or one kind is included in the other (a player for
"you", an object for "it"). -/
def performerCompatible (performer subject : Option Kind) : Bool :=
  performer.all fun p => subject.all fun k => Kind.lte p k || Kind.lte k p

/-- Whether `event` is done by, or, when the lookback's subject is what the event moved or acted
on (`subjectIsPatient`: "that creature dies", "a land enters"), done to, what the subject
names. -/
def CausedEvent.subjectFits (event : CausedEvent) (subject : Option Kind)
    (subjectIsPatient : Bool) : Bool :=
  if subjectIsPatient then performerCompatible event.patient subject
  else performerCompatible event.performer subject

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
only objects the current ability stamped, an optional body's included
(`DRS.participialOnly`), since "this way" is no linking phrase
[CR#607.1,607.2a]; no span written: also those a linked
ability published, where a singular "the" reads a group the linked ability acted on
[CR#607.2a,607.3]), named by `word` as `stampedWordOk` reads it, and in each of `zones`. -/
def participialCandidates (anchor : ParticipialAnchor) (plur : Option Plurality)
    (word : Option NounWord) (thisWay : Bool) (drs : DRS) (zones : List Zone := []) :
    List Referent :=
  -- One object is one candidate: an earlier object an optional body stamped again is read once,
  -- as it stood after the body if that stamp reads it, else as it stood before.
  let found := drs.participialReferents.filter fun b =>
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
  found.foldr (fun b kept => b :: kept.filter (·.id != b.id)) []

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
    .ok (wholeGroups found)
  | some _ =>
    let found := wholeGroups found
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

/-- Close a relative clause while retaining its definite attachment hosts in the outer DRS, and
the Contrast Anchors resolved inside it: "for each other attacking creature" still contrasts
with something once its counted domain is closed. -/
def leaveRelative (before after : DRS) : DRS :=
  let hosts := after.referents.filterMap fun r =>
    if r.attachmentHost && r.id ≥ before.nextId then
      some { r with clause := before.currentClause, role := .other, pronounResolved := false }
    else none
  { before with
    referents := hosts ++ before.referents
    nextId := max before.nextId after.nextId
    contrastAnchors := after.contrastAnchors }

/-- Close a measured domain ("the number of …", "the greatest … among …") as a relative clause
is closed (`leaveRelative`), keeping the targets announced inside it: they are chosen as the spell
or ability is put on the stack [CR#601.2c,115.1], so "the number of cards in target player's hand
… to that player" reads the target back. -/
def leaveMeasure (before after : DRS) : DRS :=
  let closed := leaveRelative before after
  let targets := after.referents.filter fun r =>
    r.det == .target && r.id ≥ before.nextId && !r.attachmentHost
  { closed with referents := targets ++ closed.referents }

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
    callerSourceTypes := drs.sourceTypes
  }
  -- A Definition is written for any Source, so its Source's Card Types are unknown.
  { (enterSelectionScope drs) with
    scopeStack    := frame :: drs.scopeStack
    pendingRoles  := []
    currentClause := drs.currentClause + 1
    sourceTypes   := [] }

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
        resolvedIds   := []
        sourceTypes   := topFrame.callerSourceTypes }

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
        resolvedIds   := before.resolvedIds
        sourceTypes   := before.sourceTypes }

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
        currentClause := topFrame.callerClause
        sourceTypes   := topFrame.callerSourceTypes }

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
  | .spell _, t => t.instantOrSorcery

def Subtype.label : Subtype → String
  | .of _ label => label
  | .spell label => label

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

/-- What an event ordinal ("your second spell each turn") may count within. `thisObject` is a
Usage Limit's alone: an object's existence [CR#400.7] spans no run of events to count. "Each turn"
is spelled `Period.turn`; `part .turn` would say the same and is refused, so the one reading has
one spelling. -/
def Period.eventOrdinalOk : Period → Bool
  | .thisObject | .part .turn => false
  | _ => true

/-! ## Designations -/

def DesignationLabel.name : DesignationLabel → String
  | .named n _ => n

def DesignationLabel.scope : DesignationLabel → DesignationScope
  | .named _ scope => scope

def DesignationLabel.holder (label : DesignationLabel) : Option Kind :=
  match label.scope with
  | .heldBy k => some k
  | .heldByCard => some .object
  | .heldByGame => none

/-- A possessive on a designation ("your Ring-bearer", "your commander") is meaningful only
where an object holds the designation for a player; a player-held or game-wide designation has
no possessor [CR#701.54e]. -/
def DesignationLabel.possessorOk (label : DesignationLabel) : Bool :=
  label.holder == some .object

def DesignationLabel.heldByItsCard (label : DesignationLabel) : Bool :=
  label.scope == .heldByCard

def DesignationLabel.gameWide (label : DesignationLabel) : Bool :=
  label.scope == .heldByGame

/-- A designation that a player or an object holds. -/
def designationHolderOk (label : DesignationLabel) : Bool :=
  match label.scope with
  | .heldBy _ => true
  | _ => false

/-! ## Relations -/

def RelationLabel.name : RelationLabel → String
  | .named n => n

/-! ## Attachment, status, counters -/

/-- Where the host an attachment word names is: on the battlefield for a description without
"card" or "spell" [CR#109.2], on the stack for a spell [CR#109.2b], and nowhere in particular for
a card, which its enchant ability places ("enchant creature card in a graveyard"; the card
Spellweaver Volute enchants "won't be on the battlefield"). -/
def NounWord.attachHostZone : NounWord → Option Zone
  | .ofType word _ | .copied word => word.attachHostZone
  | .type _ | .subtype (.of ..) | .permanent | .token => some .battlefield
  | .spell | .copy => some .stack
  | _ => none

def NounWord.attachHostTy : NounWord → List CardType
  | .ofType word t => mergeTypes word.attachHostTy [t]
  | .copied word => word.attachHostTy
  | .type t | .subtype (.of t _) => [t]
  | _ => []

def CounterKind.name : CounterKind → String
  | .named n _ => n

/-- What a counter of the kind is placed on: an object or a player [CR#122.1]. -/
def CounterKind.holder : CounterKind → Kind
  | .named _ holder => holder

def CounterKind.scope (c : CounterKind) : Kind := c.holder

/-- What kind of entity a projection axis reads [CR#120.3e,120.6,704.5g]. -/
def ProjAxis.scope : ProjAxis → Kind
  | .stat _ => .object
  | .playerStat _ => .player
  | .counter c => c.scope
  | .anyCounter k => k
  | .boundCounter => .join .object .player
  | .chapter => .object
  | .markedDamage => .object
  | .keywordNumber _ => .object

/-- Whether an axis projects one object to several values, read only through a fold
[CR#714.2d,702.164b]. -/
def ProjAxis.multiValued : ProjAxis → Bool
  | .chapter | .keywordNumber _ => true
  | _ => false

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
