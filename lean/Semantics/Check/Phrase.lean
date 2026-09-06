import Semantics.Phrase
import Semantics.Check.Events
import Semantics.Check.Keywords
import Semantics.Check.Refusal

/-!
# Semantics.Check.Phrase

The phrase layer of the checker: port of every function `Phrase.idr` declared beside its
syntax, and one rule set per constructor for the obligations the Idris put in constructor
types.

## Kinds

The Idris `Kind` index is inferred here. `kind?` gives the kind a phrase fixes by itself
(`hasType` is an object predicate; `hasPossessor` is polymorphic and gives `none`), and
`kindOr d` falls back to the kind the context expects, which is how Idris resolved a
polymorphic constructor from its use site. The sort rules refuse a `kindMismatch` where two
constraints disagree.

## The stack

Every attribute function takes the antecedent stack `bs` it is evaluated against, and passes
the shifted stack (`nomIntro bs n`, `amtIntro bs a`, …) into the fields the Idris typed at the
shifted index. That threading is the whole content of the Idris indices, so it is spelled out
call by call.
-/

namespace Semantics

/-! ## Kinds -/

def joinKinds (a b : Kind) : Kind := if a == b then a else .join a b

mutual
  def Predicate.kind? : Predicate → Option Kind
    | .anyPlayer | .opponent | .chosenPlayer _ | .choseExtreme _ => some .player
    | .qualityNoun q _ => some (.quality q)
    | .counterKindOn _ => some (.quality .counterKind)
    | .hasDesignation d _ => d.holder
    | .attachment .host _ _ => none
    | .compare axes _ _ => (axes.head?).map (·.scope)
    | .superlative _ _ dom => dom.kind?
    | .compareOver dom _ _ _ => dom.kind?
    | .and ps => Predicate.kindOfAny ps
    | .or ps => Predicate.kindOfAll ps
    | .not p => p.kind?
    | .hasPossessor _ _ | .inCombat _ _ | .happenedTo _ | .withMostVotes | .other | .notChosen
    | .otherThan _ | .coinCameUp _ | .targets _ _ => none
    | _ => some .object

  def Predicate.kindOfAny : List Predicate → Option Kind
    | [] => none
    | p :: ps =>
      match p.kind? with
      | some k => some k
      | none => Predicate.kindOfAny ps

  /-- A disjunction names the join of its disjuncts' kinds: "a creature or player" is one
  phrase at a joined kind. -/
  def Predicate.kindOfAll : List Predicate → Option Kind
    | [] => none
    | p :: ps =>
      match p.kind?, Predicate.kindOfAll ps with
      | some k, some k' => some (joinKinds k k')
      | some k, none => some k
      | none, k' => k'
end

/-- The distinct kinds a disjunction's disjuncts name, first appearance first. -/
def Predicate.disjunctKinds : List Predicate → List Kind
  | [] => []
  | p :: ps =>
    let rest := Predicate.disjunctKinds ps
    match p.kind? with
    | some k => if rest.elem k then rest else k :: rest
    | none => rest

/-- Whether a disjunction joins kinds ("creature or player"). -/
def Predicate.joins (ps : List Predicate) : Bool :=
  match Predicate.disjunctKinds ps with
  | _ :: _ :: _ => true
  | _ => false

def Predicate.kindOr (d : Kind) (p : Predicate) : Kind := p.kind?.getD d

/-- Whether a disjunct belongs to the `k` half of a join: its kind is `k`, or unnamed. -/
def Predicate.inKind (p : Predicate) (k : Kind) : Bool :=
  match p.kind? with
  | some k' => k == k'
  | none => true

mutual
  def NounPhrase.kind? : NounPhrase → Option Kind
    | .gap k => some k
    | .you | .combatPlayer _ | .playerGroup _ | .possessorOf _ _ => some .player
    | .described _ p => p.kind?
    | .eachOf g => g.kind?
    | .and ns | .or ns => some (NounPhrase.kindOfAll ns)
    | .theRest k _ => some k
    | .pileOf _ _ => some .pile
    | .pro r _ _ => some r.kind
    | .attachHost _ h => some h.kind
    | _ => some .object
  termination_by structural n => n

  def NounPhrase.kindOfAll : List NounPhrase → Kind
    | [] => .object
    | [n] => n.kind?.getD .object
    | n :: ns => joinKinds (n.kind?.getD .object) (NounPhrase.kindOfAll ns)
  termination_by structural ns => ns
end

def NounPhrase.kindOr (d : Kind) (n : NounPhrase) : Kind := n.kind?.getD d

/-! ## Leaf helpers -/

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

def zoneAdmits (z : Zone) : List Zone → Bool
  | [] => true
  | zs => zs.elem z

def allNegated (negs : List CardType) : List CardType → Bool
  | [] => true
  | t :: ts => negs.elem t && allNegated negs ts

def anchorTyFits : List CardType → List CardType → Bool
  | [], _ => true
  | us, t => us.any (anchorTyOk · t)

def atMostOne : Nat → Bool
  | 0 | 1 => true
  | _ => false

def zoneOr (z : Zone) : Option Zone → Zone
  | none => z
  | some w => w

/-- The binding payload for a phrase at kind `k` seeding `ty`. -/
def joinHalfPayload : Kind → HeadTy → Payload
  | .object, .sole ty => .object ty none none none none
  | .object, .join _ _ => .object [] none none none none
  | .player, _ => .player false
  | .quality q, _ => .quality q
  | .join l r, .join a b => .join (joinHalfPayload l a) (joinHalfPayload r b)
  | .join l r, .sole ty => .join (joinHalfPayload l (.sole ty)) (joinHalfPayload r (.sole ty))
  | _, _ => .gap

def elemPayload : Kind → Bool → List CardType → Option Zone → Option Stamp → Payload
  | .object, true, _, zn, _ => .ability none zn
  | .object, false, ty, zn, pv => .object ty zn pv none (some 1)
  | .player, _, _, _, _ => .player false
  | .quality q, _, _, _, _ => .quality q
  | .join l r, ab, ty, zn, pv => .join (elemPayload l ab ty zn pv) (elemPayload r ab ty zn pv)
  | _, _, _, _, _ => .gap

def zonesDistinct : List Zone → Bool
  | [] => true
  | z :: zs => !zs.elem z && zonesDistinct zs

def atLeastTwoZones : List Zone → Bool
  | zs@(_ :: _ :: _) => zonesDistinct zs
  | _ => false

def chosenDet : Kind → Determiner → Determiner
  | .object, _ => .part
  | _, d => d

def counterKind : Kind → Bool
  | .object => true
  | .join a b => counterKind a && counterKind b
  | _ => false

def controlKind : Kind → Bool
  | .object => true
  | .join a b => controlKind a && controlKind b
  | _ => false

def possessorKind : PossessorAxis → Kind → Bool
  | .controller, k => controlKind k
  | .owner, k => Kind.lte k .object

def copyKind : Kind → Bool
  | .object => true
  | .join a b => copyKind a && copyKind b
  | _ => false

/-! ## Zones and places -/

def LibraryPlace.arrangementOk : LibraryPlace → Option Arrangement → Bool
  | .oneEnd _, _ => true
  | .eitherEnd _, none => true
  | .eitherEnd _, some _ => false
  | .shuffled, none => true
  | .shuffled, some _ => false

def LibraryPlace.ordinalOk : LibraryPlace → Option Ordinal → Bool
  | .shuffled, some _ => false
  | _, _ => true

def LibraryPlace.shuffles : LibraryPlace → Bool
  | .shuffled => true
  | _ => false

def ZoneExpr.sort : ZoneExpr → Zone
  | .zone z _ => z
  | .library _ _ _ _ => .library

def ZoneExpr.arrangement : ZoneExpr → Option Arrangement
  | .zone _ _ => none
  | .library _ ord _ _ => ord

def ZoneExpr.ordinal : ZoneExpr → Option Ordinal
  | .zone _ _ => none
  | .library _ _ off _ => off

def ZoneExpr.shuffles : ZoneExpr → Bool
  | .zone _ _ => false
  | .library place _ _ _ => place.shuffles

def afterMoveTo (to : Option ZoneExpr) (out : Bindings) : Bindings :=
  if to.any ZoneExpr.shuffles then afterShuffle out else out

def sourceZone : Option EventSource → Option Zone
  | some (.zones [z]) => some z.sort
  | _ => none

def EventSource.zonesOk (ok : Zone → Bool) : EventSource → Bool
  | .anywhere => true
  | .zones zs | .anywhereBut zs => !zs.isEmpty && zs.all (fun z => ok z.sort)

def LookbackClause.event : LookbackClause → GameEvent
  | .mk ev _ => ev

def LookbackClause.window : LookbackClause → Lookback
  | .mk _ w => w

/-! ## What a predicate seeds -/

mutual
  /-- Explicit card-type facts take precedence over subtype-derived fallback evidence. -/
  def Predicate.writtenTypes : Predicate → List CardType
    | .hasType t => [t]
    | .and ps => Predicate.writtenTypesAll ps
    | .or ps => Predicate.writtenTypesJoin ps
    | .compareOver dom _ _ _ => dom.writtenTypes
    | _ => []

  def Predicate.writtenTypesAll : List Predicate → List CardType
    | [] => []
    | p :: ps => mergeTypes p.writtenTypes (Predicate.writtenTypesAll ps)

  def Predicate.writtenTypesJoin : List Predicate → List CardType
    | [] => []
    | [p] => p.writtenTypes
    | p :: ps => commonTypes p.writtenTypes (Predicate.writtenTypesJoin ps)
end

mutual
  def Predicate.seedTy : Predicate → List CardType
    | .hasType t => [t]
    | .hasSubtype s => optCT s.type
    | .and ps =>
      let written := Predicate.writtenTypesAll ps
      if written.isEmpty then Predicate.inferredTypesAll ps else written
    | .or ps =>
      if Predicate.joins ps then
        (Predicate.disjunctKinds ps).foldr
          (fun k acc => Payload.joinSeed (Predicate.seedTyJoinOfKind k ps) acc) []
      else Predicate.seedTyJoin ps
    | .compareOver dom _ _ _ => dom.seedTy
    | _ => []

  /-- Each joined kind retains the facts shared by its own alternatives. -/
  def Predicate.seedTyJoinOfKind (k : Kind) : List Predicate → List CardType
    | [] => []
    | p :: ps =>
      if p.inKind k then
        p.seedTy.filter (fun t => Predicate.allSeedTyOfKind k t ps)
      else Predicate.seedTyJoinOfKind k ps

  def Predicate.allSeedTyOfKind (k : Kind) (t : CardType) : List Predicate → Bool
    | [] => true
    | p :: ps =>
      (!p.inKind k || p.seedTy.contains t) && Predicate.allSeedTyOfKind k t ps

  def Predicate.inferredTypesAll : List Predicate → List CardType
    | [] => []
    | p :: ps => mergeTypes p.seedTy (Predicate.inferredTypesAll ps)

  def Predicate.seedTyJoin : List Predicate → List CardType
    | [] => []
    | p :: ps => p.seedTy.filter (fun t => Predicate.allSeedTy t ps)

  def Predicate.allSeedTy (t : CardType) : List Predicate → Bool
    | [] => true
    | p :: ps => p.seedTy.contains t && Predicate.allSeedTy t ps
end

def Predicate.seedTyAll (ps : List Predicate) : List CardType :=
  let written := Predicate.writtenTypesAll ps
  if written.isEmpty then Predicate.inferredTypesAll ps else written

/-- The disjuncts of `ps` whose kind is `k` or unnamed. -/
def Predicate.ofKind (k : Kind) : List Predicate → List Predicate
  | [] => []
  | p :: ps =>
    let rest := Predicate.ofKind k ps
    match p.kind? with
    | some k' => if k == k' then p :: rest else rest
    | none => p :: rest

/-- The head type a joined disjunction seeds, one half per kind, in the shape `kindOfAll`
folds so that `joinHalfPayload` can walk both together. -/
def joinedSeedTys (ps : List Predicate) : List Kind → HeadTy
  | [] => .sole []
  | [k] => .sole (Predicate.seedTyJoin (Predicate.ofKind k ps))
  | k :: ks => .join (.sole (Predicate.seedTyJoin (Predicate.ofKind k ps))) (joinedSeedTys ps ks)

def Predicate.seedTys : Predicate → HeadTy
  | .or ps => if Predicate.joins ps then joinedSeedTys ps (Predicate.disjunctKinds ps) else .sole (Predicate.seedTyJoin ps)
  | p => .sole p.seedTy

mutual
  def Predicate.headTys : Predicate → List CardType
    | .and ps => Predicate.headTysAll ps
    | .or ps => Predicate.headTysJoin ps
    | p => p.seedTy

  def Predicate.headTysJoin : List Predicate → List CardType
    | [] => []
    | p :: ps => p.headTys ++ Predicate.headTysJoin ps

  def Predicate.headTysAll : List Predicate → List CardType
    | [] => []
    | p :: ps => mergeTypes p.headTys (Predicate.headTysAll ps)
end

def combineTypeAlts (left right : List (List CardType)) : List (List CardType) :=
  left.flatMap fun l => right.map (mergeTypes l)

mutual
  def Predicate.headTyAlts : Predicate → List (List CardType)
    | .and ps => Predicate.headTyAltsAll ps
    | .or ps => Predicate.headTyAltsJoin ps
    | p => [p.seedTy]

  def Predicate.headTyAltsAll : List Predicate → List (List CardType)
    | [] => [[]]
    | p :: ps => combineTypeAlts p.headTyAlts (Predicate.headTyAltsAll ps)

  def Predicate.headTyAltsJoin : List Predicate → List (List CardType)
    | [] => []
    | p :: ps => p.headTyAlts ++ Predicate.headTyAltsJoin ps
end

mutual
  def Predicate.attachWordsIn : Predicate → List AttachWord
    | .attachment .host (some w) _ => [w]
    | .and ps => Predicate.attachWordsInAll ps
    | .or ps => Predicate.attachWordsInAll ps
    | _ => []

  def Predicate.attachWordsInAll : List Predicate → List AttachWord
    | [] => []
    | p :: ps => p.attachWordsIn ++ Predicate.attachWordsInAll ps
end

mutual
  def Predicate.seedZone : Predicate → Option Zone
    | .inZone z => some z.sort
    | .inCombat .attackedBy _ => none
    | .inCombat _ _ => some .battlefield
    | .hasDesignation d _ => d.seedZone
    | .attachment _ _ _ | .isToken | .currentFace _ | .hasStatus _ =>
      some .battlefield
    | .isEmblem => some .command
    | .targets _ _ => some .stack
    | .exiledWith _ => some .exile
    | .compareOver dom _ _ _ => dom.seedZone
    | .abilityHead _ | .abilityOf _ | .activatedBy _ | .isManaAbility => some .stack
    | .and ps => Predicate.seedZoneAll ps
    | .or ps => Predicate.seedZoneJoin ps
    | _ => none

  def Predicate.seedZoneAll : List Predicate → Option Zone
    | [] => none
    | p :: ps =>
      match p.seedZone with
      | some z => some z
      | none => Predicate.seedZoneAll ps

  def Predicate.seedZoneJoin : List Predicate → Option Zone
    | [] => none
    | p :: ps =>
      match p.seedZone with
      | none => none
      | some z => if Predicate.allSeedZone z ps then some z else none

  def Predicate.allSeedZone (z : Zone) : List Predicate → Bool
    | [] => true
    | p :: ps =>
      match p.seedZone with
      | none => false
      | some w => z == w && Predicate.allSeedZone z ps
end

mutual
  def Predicate.seedsToken : Predicate → Bool
    | .isToken => true
    | .and ps => Predicate.seedsTokenAny ps
    | .or ps => Predicate.seedsTokenAll ps
    | .compareOver dom _ _ _ => dom.seedsToken
    | _ => false

  def Predicate.seedsTokenAny : List Predicate → Bool
    | [] => false
    | p :: ps => p.seedsToken || Predicate.seedsTokenAny ps

  /-- Every arm of a disjunction seeds: a singleton by itself, a longer list arm by arm. -/
  def Predicate.seedsTokenAll : List Predicate → Bool
    | [] => false
    | [p] => p.seedsToken
    | p :: ps => p.seedsToken && Predicate.seedsTokenAll ps
end

/-! An activated or triggered ability on the stack is an object [CR#113.1c]. -/
mutual
  def Predicate.seedsAbility : Predicate → Bool
    | .abilityHead _ | .abilityOf _ | .activatedBy _ | .isManaAbility => true
    | .and ps => Predicate.seedsAbilityAny ps
    | .or ps => Predicate.seedsAbilityAll ps
    | .compareOver dom _ _ _ => dom.seedsAbility
    | _ => false

  def Predicate.seedsAbilityAny : List Predicate → Bool
    | [] => false
    | p :: ps => p.seedsAbility || Predicate.seedsAbilityAny ps

  /-- Every arm of a disjunction seeds: a singleton by itself, a longer list arm by arm. -/
  def Predicate.seedsAbilityAll : List Predicate → Bool
    | [] => false
    | [p] => p.seedsAbility
    | p :: ps => p.seedsAbility && Predicate.seedsAbilityAll ps
end

def Predicate.zoneAdmit : Predicate → List Zone
  | .hasPossessor .controller _ => [.battlefield, .stack]
  | _ => []

mutual
  def Predicate.seedType : Predicate → Option CardType
    | .inCombat .attackedBy _ => none
    | .inCombat _ _ => some .creature
    | .hasDesignation d _ => d.seedType
    | .compare cs _ _ => axisTypes cs
    | .superlative _ (.stat c) _ => c.comparedType
    | .compareOver dom _ _ _ => dom.seedType
    | .hasSubtype s => s.type
    | .and ps => Predicate.seedTypeAll ps
    | .or ps => Predicate.seedTypeJoin ps
    | _ => none

  def Predicate.seedTypeAll : List Predicate → Option CardType
    | [] => none
    | p :: ps =>
      match p.seedType with
      | some t => some t
      | none => Predicate.seedTypeAll ps

  def Predicate.seedTypeJoin : List Predicate → Option CardType
    | [] => none
    | p :: ps =>
      match p.seedType with
      | none => none
      | some t => if Predicate.allSeedType t ps then some t else none

  def Predicate.allSeedType (t : CardType) : List Predicate → Bool
    | [] => true
    | p :: ps =>
      match p.seedType with
      | none => false
      | some u => t == u && Predicate.allSeedType t ps
end

mutual
  def Predicate.hasHead : Predicate → Bool
    | .hasType _ | .hasSubtype _ | .anyPlayer | .opponent | .chosenPlayer _ | .qualityNoun _ _
    | .counterKindOn _ | .abilityHead _ | .isSource | .isCard | .isToken
    | .isEmblem | .isCopyOfACard | .inZone _ | .exiledWith _ => true
    | .hasDesignation d _ => d.heldByItsCard
    | .compareOver dom _ _ _ => dom.hasHead
    | .and ps => Predicate.hasHeadAny ps
    | .or ps => Predicate.hasHeadAll ps
    | _ => false

  def Predicate.hasHeadAny : List Predicate → Bool
    | [] => false
    | p :: ps => p.hasHead || Predicate.hasHeadAny ps

  def Predicate.hasHeadAll : List Predicate → Bool
    | [] => true
    | p :: ps => p.hasHead && Predicate.hasHeadAll ps
end

mutual
  def Predicate.uniquifies : Predicate → Bool
    | .chosenPlayer _ => true
    | .superlative _ _ _ => true
    | .withMostVotes => false
    | .choseExtreme _ => false
    | .inCombat .attackedBy _ => true
    /- "the exiled card" is linked to the exiling ability printed on the same object
    [CR#607.2a]; a singular reference still resolves when that ability exiled several cards
    [CR#607.3]. -/
    | .exiledWith _ => true
    | .castBy _ rank => rank.isSome
    | .and ps => Predicate.uniquifiesAny ps
    | _ => false

  def Predicate.uniquifiesAny : List Predicate → Bool
    | [] => false
    | p :: ps => p.uniquifies || Predicate.uniquifiesAny ps
end

mutual
  /-- Conjunctions flattened, as the Idris `flattenPs`. -/
  def Predicate.flatten : List Predicate → List Predicate
    | [] => []
    | p :: ps => Predicate.flattenOne p ++ Predicate.flatten ps
  termination_by structural ps => ps

  def Predicate.flattenOne : Predicate → List Predicate
    | .and qs => Predicate.flatten qs
    | p => [p]
  termination_by structural p => p
end

def zonesAgree : Option Zone → List Predicate → Bool
  | _, [] => true
  | acc, p :: ps =>
    match p.seedZone with
    | none => zonesAgree acc ps
    | some z =>
      match acc with
      | none => zonesAgree (some z) ps
      | some w => w == z && zonesAgree (some w) ps

def Predicate.negZonesOf : Predicate → List Zone
  | .not (.inZone z) => [z.sort]
  | _ => []

def negZones (ps : List Predicate) : List Zone := ps.flatMap Predicate.negZonesOf

def zoneAdmitsAll (z : Zone) (ps : List Predicate) : Bool :=
  ps.all fun p => zoneAdmits z p.zoneAdmit

def zonesOk (ps : List Predicate) : Bool :=
  let fs := Predicate.flatten ps
  let z := zoneOr .battlefield (Predicate.seedZoneAll fs)
  zonesAgree none fs && !(negZones fs).elem z && zoneAdmitsAll z fs

/-- Each status category always has exactly one of its two values [CR#110.5]. -/
def statusClashOf : Predicate → Predicate → Bool
  | .hasStatus v, .hasStatus w => v.clash w
  | _, _ => false

/-- A colorless object has no color [CR#105.2c]: "colorless" is written `colorCount .eq 0`. -/
def Predicate.isColorless : Predicate → Bool
  | .colorCount .eq 0 => true
  | _ => false

def colorClashOf : Predicate → Predicate → Bool
  | p, .colorIs _ => p.isColorless
  | .colorIs _, p => p.isColorless
  | _, _ => false

/-- "Permanent" is written `inZone battlefield` [CR#110.1]. -/
def Predicate.isPermanentHead : Predicate → Bool
  | .inZone (.zone .battlefield _) => true
  | _ => false

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

def Predicate.negTypesOf : Predicate → List CardType
  | .not (.hasSubtype _) => []
  | .not p => p.seedTy
  | _ => []

def negTypes (ps : List Predicate) : List CardType := ps.flatMap Predicate.negTypesOf

/-- A subtype is correlated to its card type [CR#205.3c]; negate that type and the
description's seed is empty. -/
def Predicate.seedTypeAlts : Predicate → List CardType
  | .hasSubtype s =>
    match s.type with
    | some .creature => [.creature, .kindred]
    | some t => [t]
    | none => [.instant, .sorcery]
  | p => optCT p.seedType

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

def contradictionFree (ps : List Predicate) : Bool :=
  let fs := Predicate.flatten ps
  !anySeedEmptied (negTypes fs) fs && noClash statusClashOf fs && noClash colorClashOf fs &&
    noClash cardTokenClashOf fs && noAttachHeadClash fs &&
    !(fs.any Predicate.isPermanentHead && anyNonPermanentTy fs)

mutual
  def Predicate.hasOther : Predicate → Bool
    | .other => true
    | .otherThan _ => true
    | .and ps => Predicate.hasOtherAny ps
    | .or _ => false
    | _ => false

  def Predicate.hasOtherAny : List Predicate → Bool
    | [] => false
    | p :: ps => p.hasOther || Predicate.hasOtherAny ps
end

def Predicate.isOther : Predicate → Bool
  | .other => true
  | .otherThan _ => true
  | _ => false

mutual
  def Predicate.hasBareOther : Predicate → Bool
    | .other => true
    | .and ps => Predicate.hasBareOtherAny ps
    | _ => false

  def Predicate.hasBareOtherAny : List Predicate → Bool
    | [] => false
    | p :: ps => p.hasBareOther || Predicate.hasBareOtherAny ps
end

def Predicate.isSourceHead : Predicate → Bool
  | .isSource => true
  | .isCard => true
  | _ => false

def Predicate.headIsPlaceless (p : Predicate) : Bool :=
  (Predicate.flatten [p]).any Predicate.isSourceHead

def Predicate.phraseZone (k : Kind) (p : Predicate) : Option Zone :=
  if p.headIsPlaceless || !Kind.lte k .object then none
  else some (zoneOr .battlefield p.seedZone)

def countOthers (ps : List Predicate) : Nat := (ps.filter Predicate.isOther).length

def Predicate.armPresupposes (p : Predicate) : Option CardType :=
  if p.hasHead then none else p.seedType

def seedsUniform (z : Option Zone) (t : Option CardType) : List Predicate → Bool
  | [] => true
  | p :: ps => z == p.seedZone && t == p.armPresupposes && seedsUniform z t ps

def parallelDisjuncts : List Predicate → Bool
  | [] => true
  | p :: ps => seedsUniform p.seedZone p.armPresupposes ps

def coordinableAll (ps : List Predicate) : Bool := ps.all fun p => !p.hasOther

def Predicate.negatable : Predicate → Bool
  | .anyPlayer => false
  | .chosenPlayer _ => false
  | .qualityNoun _ none => false
  | .qualityNoun _ (some _) => true
  | .counterKindOn _ => false
  | .isSource => false
  | _ => true

/-- The binding a described phrase introduces, at kind `k`. -/
def bindFor (det : Determiner) (plur : Plurality) : Kind → Predicate → Binding
  | .object, p =>
    if p.seedsAbility then ⟨det, plur, .ability none⟩
    else ⟨det, plur,
      .object p.seedTy (some (zoneOr .battlefield p.seedZone)) none
        (if p.seedsToken then some .token else none) none⟩
  | .player, _ => ⟨det, plur, .player false⟩
  | .quality q, _ => ⟨det, plur, .quality q⟩
  | k@(.join _ _), p => ⟨det, plur, joinHalfPayload k p.seedTys⟩
  | .letter l, _ => ⟨det, plur, .letter l⟩
  | .turnRef, _ => ⟨det, plur, .turnRef⟩
  | .pile, _ => ⟨det, plur, .pile none none none⟩
  -- No predicate fixes an outcome sort, so an outcome-kinded described phrase has nothing to
  -- bind; keep it a gap, same as the truly kindless case.
  | .outcome, _ | .gap, _ => ⟨det, plur, .gap⟩

def chosenBind (det : Determiner) (plur : Plurality) : Kind → Predicate → Binding
  | .player, _ => ⟨det, plur, .player true⟩
  | k, p => bindFor det plur k p

def Predicate.qualityReadOk : Predicate → Bool
  | .ofChosen _ _ | .ofYourChoice _ _ | .named _ => true
  | _ => false

def Predicate.qualityReadHost : Predicate → Option CardType
  | .ofChosen _ (.subtype h) => some h
  | .ofYourChoice (.subtype h) _ => some h
  | _ => none

def Predicate.isOr : Predicate → Bool
  | .or _ => true
  | _ => false

mutual
  def Predicate.says : Predicate → Bool
    | .and ps => Predicate.saysAny ps
    | .not p => p.says
    | _ => true

  def Predicate.saysAny : List Predicate → Bool
    | [] => false
    | p :: ps => p.says || Predicate.saysAny ps
end

mutual
  def Predicate.negFree : Predicate → Bool
    | .and ps => Predicate.negFreeAll ps
    | .or ps => Predicate.negFreeAll ps
    | .not _ => false
    | _ => true

  def Predicate.negFreeAll : List Predicate → Bool
    | [] => true
    | p :: ps => p.negFree && Predicate.negFreeAll ps
end

/-- A party role is a creature subtype [CR#700.8]. -/
def Predicate.roleSubtype : Predicate → Option Subtype
  | .hasSubtype s => some s
  | _ => none

def roleElem (p : Predicate) : List Predicate → Bool
  | [] => false
  | q :: qs =>
    match p.roleSubtype, q.roleSubtype with
    | some a, some b => a == b || roleElem p qs
    | _, _ => roleElem p qs

/-- One creature is the party member for only one role [CR#700.8b], so a repeated role would
count it twice. -/
def rolesDistinct : List Predicate → Bool
  | [] => true
  | p :: ps => !roleElem p ps && rolesDistinct ps

/-- A one-each group is written from at least one distinct role [CR#700.8]. -/
def rolesOk : List Predicate → Bool
  | [] => false
  | rs => rolesDistinct rs

/-! ## Determiners, quantities, amounts -/

def DetPhrase.det : DetPhrase → Determiner
  | .target _ => .target
  | .a _ => .a
  | .each => .each
  | .all => .all
  | .the => .the
  | .count _ _ => .count
  | .bare => .bare

/-- A literal's value as a count, when it is one: a negative literal names no count
[CR#107.1b], so it is not exact. -/
def Amount.exact : Amount → Option Nat
  | .lit (.ofNat n) => some n
  | _ => none

def Amount.nonZero : Amount → Bool
  | .lit n => n != 0
  | _ => true

def Amount.plur : Amount → Plurality
  | .lit n => if n == 1 then .one else .many
  | .upTo b => b.plur
  | _ => .many

def Amount.read : Amount → Bool
  | .lit _ | .arith _ _ _ | .thatMuch | .chosenNumber _ | .groupSize | .half _ _ | .upTo _ => false
  | .theOutcome s => s.comparable
  | _ => true

def Quantity.wellFormed : Quantity → Bool
  | .range (some lo) (some hi) => lo ≤ hi
  | _ => true

/-- Idris `NonZeroQ`, decided: a range is nonzero unless its ceiling is zero. -/
def Quantity.nonZero : Quantity → Bool
  | .range _ (some 0) => false
  | _ => true

def Quantity.exact : Quantity → Option Nat
  | .range (some lo) (some hi) => if lo == hi then some lo else none
  | .exactlyOf a => a.exact
  | _ => none

def Quantity.plur : Quantity → Plurality
  | .range _ (some 1) => .one
  | _ => .many

def Quantity.literal : Quantity → Bool
  | .range _ _ => true
  | _ => false

def Quantity.modesFit : Quantity → Nat → Bool
  | .range none none, _ => true
  | .range none (some hi), n => hi ≤ n
  | .range (some lo) none, n => lo ≤ n
  | .range (some _) (some hi), n => hi ≤ n
  | _, _ => true

def DetPhrase.plur : DetPhrase → Plurality
  | .target q => q.plur
  | .count q _ => q.plur
  | .a _ | .the => .one
  | _ => .many

def DetPhrase.quant : DetPhrase → Option Quantity
  | .target q => some q
  | .count q _ => some q
  | _ => none

def SliceCount.exact : SliceCount → Option Nat
  | .counted q => q.exact
  | .whole => none

def SliceCount.plur : SliceCount → Plurality
  | .counted q => q.plur
  | .whole => .many

/-- A definite description re-reads the chosen-player binding [CR#607.2d]. -/
def Predicate.choiceRead : Predicate → Bool
  | .chosenPlayer _ => true
  | _ => false

def NounPhrase.det : NounPhrase → Option Determiner
  | .described d _ => some d.det
  | .eachOf _ => some .each
  | .librarySlice _ _ _ => some .the
  | .someOf _ _ _ => some .part
  | .namesAgree _ g => g.det
  | .theRest _ _ => some .the
  | .pileOf _ _ => some .part
  | .oneEachOf _ _ => some .bare
  | _ => none

mutual
  def NounPhrase.anchorPhrase : NounPhrase → Bool
    | .or ns => NounPhrase.allAnchorPhrase ns
    | .and _ => false
    | n => n.det.elim true (· == .target)
  termination_by structural n => n

  def NounPhrase.allAnchorPhrase : List NounPhrase → Bool
    | [] => true
    | n :: ns => n.anchorPhrase && NounPhrase.allAnchorPhrase ns
  termination_by structural ns => ns
end

def NounPhrase.groupMention : NounPhrase → Bool
  | .librarySlice _ _ _ => true
  | .pro _ pl .whole => !pl.isOne
  | .and _ => true
  | .oneEachOf _ _ => true
  | n => n.det == some .target

/-- A positional own-read partitions the group it has just named, a one-card group included
[CR#701.22a]. -/
def NounPhrase.partitiveBase : NounPhrase → Bool
  | .pro _ _ (.top _) | .pro _ _ (.introduced _) => true
  | n => n.det == some .all || n.groupMention

def NounPhrase.countableGroup (n : NounPhrase) : Bool := n.det == some .bare || n.groupMention

def NounPhrase.countedMention : NounPhrase → Bool
  | .namesAgree _ _ => false
  | n => n.det == some .count || n.det == some .target

def NounPhrase.ascribable : NounPhrase → Bool
  | .gap _ => true
  | .this | .designated _ _ => true
  | _ => false

def NounPhrase.isYou : NounPhrase → Bool
  | .you => true
  | _ => false

mutual
  def NounPhrase.targeted : NounPhrase → Bool
    | .asType _ n _ => n.targeted
    | .resolvedPermanent n => n.targeted
    | .asMarker _ n => n.targeted
    | .eachOf g => g.targeted
    | .someOf _ _ g => g.targeted
    | .and ns | .or ns => NounPhrase.anyTargeted ns
    | n => n.det == some .target
  termination_by structural n => n

  def NounPhrase.anyTargeted : List NounPhrase → Bool
    | [] => false
    | n :: ns => n.targeted || NounPhrase.anyTargeted ns
  termination_by structural ns => ns
end

mutual
  def NounPhrase.costNounOk : NounPhrase → Bool
    | .asType _ n _ => n.costNounOk
    | .resolvedPermanent n => n.costNounOk
    | .asMarker _ n => n.costNounOk
    | .eachOf g => g.costNounOk
    | .namesAgree _ g => g.costNounOk
    | .someOf _ _ g => g.costNounOk
    | .or ns => NounPhrase.allCostNounOk ns
    | .and _ => false
    | .pro (.verbed _ _ _) _ _ => false
    | _ => true
  termination_by structural n => n

  def NounPhrase.allCostNounOk : List NounPhrase → Bool
    | [] => true
    | n :: ns => n.costNounOk && NounPhrase.allCostNounOk ns
  termination_by structural ns => ns
end

def NounPhrase.selfDefinedOk : NounPhrase → Bool
  | .this => true
  | .asType _ n _ => n.selfDefinedOk
  | _ => false

def NounPhrase.choosable (n : NounPhrase) : Bool :=
  n.det == some .a || n.det == some .target || n.det == some .count

def NounPhrase.agentChoosable : NounPhrase → Bool
  | .someOf _ _ _ => true
  | .pileOf _ _ => true
  | n => n.choosable

def NounPhrase.countedExistential : NounPhrase → Bool
  | .namesAgree _ g => g.countedMention
  | _ => false

def NounPhrase.existentialMention (n : NounPhrase) : Bool := n.det == some .bare || n.countedExistential

/-- Whether the noun names an ability on the stack [CR#113.1c], read off the description and
the reading word, never the antecedent stack. -/
def NounPhrase.isAbility : NounPhrase → Bool
  | .described _ p => p.seedsAbility
  | .pro (.word word) _ _ => word.isAbility
  | .asMarker .ability _ => true
  | .eachOf g => g.isAbility
  | .namesAgree _ g => g.isAbility
  | .resolvedPermanent n => n.isAbility
  | .asMarker _ n => n.isAbility
  | .someOf _ _ g => g.isAbility
  | _ => false

def NounPhrase.moveDestOk : NounPhrase → Bool
  | .pro r _ .whole => !r.tracksObject
  | .pro _ _ _ => false
  | _ => true

def NounPhrase.remarkTest : NounPhrase → Option (Binding → Bool)
  | .pro (.word .ability) .one .whole => some (reaches (.word .ability) .one)
  | .pro r pl .whole => if r.tracksObject then some (reaches r pl) else none
  | _ => none

/-! ## Number -/

mutual
  def NounPhrase.plur : NounPhrase → Plurality
    | .gap _ => .one
    | .this | .theGrantor _ | .combatPlayer _ | .you | .attachHost _ _ | .designated _ _ => .one
    | .asType _ n _ => n.plur
    | .resolvedPermanent n => n.plur
    | .asMarker _ n => n.plur
    | .playerGroup _ => .many
    | .described d _ => d.plur
    | .eachOf _ => .many
    | .namesAgree _ g => g.plur
    | .and _ => .many
    | .or ns => NounPhrase.commonPlurality ns
    | .librarySlice _ amt whose => outputPlur whose.plur amt.plur
    | .someOf q _ _ => q.plur
    | .theRest _ pl => pl
    | .pileOf q _ => q.plur
    | .pro _ pl _ => pl
    | .possessorOf _ n => n.plur
    | .oneEachOf _ _ => .many
  termination_by structural n => n

  def NounPhrase.commonPlurality : List NounPhrase → Plurality
    | [] => .many
    | [n] => n.plur
    | n :: ns => if n.plur == NounPhrase.commonPlurality ns then n.plur else .many
  termination_by structural ns => ns
end

def NounPhrase.soleHolderOk : NounPhrase → Bool
  | .playerGroup _ => true
  | n => n.plur.isOne

def NounPhrase.slicePossessorOk : NounPhrase → Bool
  | .described .each _ => true
  | .eachOf _ => true
  | .playerGroup _ => true
  | n => n.plur.isOne

def NounPhrase.perMemberOk (n : NounPhrase) : Bool :=
  n.det == some .each || n.det == some .all || n.plur.isOne

/-- "Starting with you" fixes the turn order in which the players who choose make their
choices, so it says nothing unless several players choose [CR#101.4]. -/
def choiceOrderOk : Option NounPhrase → Option NounPhrase → Bool
  | none, _ => true
  | some _, none => false
  | some _, some by_ => !by_.plur.isOne

def choiceClauseOk : Option NounPhrase → NounPhrase → Bool
  | none, n => n.choosable
  | some _, n => n.agentChoosable

/-- A plural possessor must distribute over players [CR#102.1]. -/
def partPossessorOk : Option NounPhrase → Bool
  | none => true
  | some n => n.plur.isOne || n.det == some .each

def windowOk : TurnPart → Option NounPhrase → Bool
  | .turn, none => false
  | _, w => partPossessorOk w

def pointWindowOk (w : Option NounPhrase) : Bool := partPossessorOk w

/-- A duration's possessor must be a single definite player. -/
def durationPossessorOk : Option NounPhrase → Bool
  | none => true
  | some n => n.plur.isOne && n.det.elim true (· == .the)

/-! ## What a phrase introduces -/

/-- Idris `sliceTy d grp`, over the group's own type so the block below stays structural. -/
def sliceTyOf : Option Predicate → List CardType → List CardType
  | none, gty => gty
  | some p, gty => mergeTypes gty p.seedTy

mutual
  def NounPhrase.introduced (bs : Bindings) : NounPhrase → List Binding
    | .gap _ => []
    | .this | .theGrantor _ | .combatPlayer _ | .you | .playerGroup _
    | .theRest _ _ | .pro _ _ _ | .attachHost _ _ | .designated _ _ => []
    | .asType _ n _ => NounPhrase.introduced bs n
    | .resolvedPermanent n => NounPhrase.introduced bs n
    | .asMarker _ n => NounPhrase.introduced bs n
    | .described d p => DetPhrase.introduced bs d (p.kindOr .object) p (Predicate.introduced bs p)
    | .eachOf g => NounPhrase.introduced bs g
    | .and ns => NounPhrase.introducedSeq bs ns
    | .or ns => NounPhrase.introducedAlternatives bs ns
    | .librarySlice _ amt whose =>
      ⟨.the, outputPlur whose.plur amt.plur,
        .object [] (some .library) none none amt.exact⟩ :: NounPhrase.introduced bs whose
    | .namesAgree _ g => NounPhrase.introduced bs g
    | .someOf q d g =>
      ⟨.part, q.plur, .object (sliceTyOf d (NounPhrase.ty bs g)) (NounPhrase.zone bs g) none none q.exact⟩
        :: (SliceCount.introduced bs q ++ OptPredicate.introduced bs d ++ NounPhrase.introduced bs g)
    | .pileOf q none =>
      ⟨.part, q.plur,
        .pile (zoneOfReach (.word .pile) .many bs) q.exact (faceOfReach (.word .pile) .many bs)⟩
        :: SliceCount.introduced bs q
    | .pileOf q (some by_) =>
      ⟨.part, q.plur,
        .pile (zoneOfReach (.word .pile) .many bs) q.exact (faceOfReach (.word .pile) .many bs)⟩
        :: (SliceCount.introduced bs q ++ NounPhrase.introduced bs by_)
    | .possessorOf _ n => ⟨.the, n.plur, .player false⟩ :: (NounPhrase.selfSubjIntroduced n ++ NounPhrase.introduced bs n)
    | .oneEachOf roles pool =>
      ⟨.bare, .many, .object (NounPhrase.ty bs pool) (NounPhrase.zone bs pool) none none none⟩
        :: (Predicate.introducedAll bs roles ++ NounPhrase.introduced bs pool)
  termination_by structural x => x

  def NounPhrase.introducedSeq (bs : Bindings) : List NounPhrase → List Binding
    | [] => []
    | n :: ns =>
      let intro := NounPhrase.introduced bs n
      NounPhrase.introducedSeq (intro ++ bs) ns ++ intro
  termination_by structural ns => ns

  def NounPhrase.introducedAlternatives (bs : Bindings) : List NounPhrase → List Binding
    | [] => []
    | n :: ns => NounPhrase.introduced bs n ++ NounPhrase.introducedAlternatives bs ns
  termination_by structural ns => ns

  def OptPredicate.introduced (bs : Bindings) : Option Predicate → List Binding
    | none => []
    | some p => Predicate.introduced bs p
  termination_by structural x => x

  def Predicate.introduced (bs : Bindings) : Predicate → List Binding
    | .abilityOf n => NounPhrase.introduced bs n
    | .activatedBy n => NounPhrase.introduced bs n
    | .targets m _ => NounPhrase.introduced bs m
    | .hasPossessor _ n => NounPhrase.introduced bs n
    | .castBy n _ => NounPhrase.introduced bs n
    | .inCombat _ none => []
    | .inCombat _ (some m) => NounPhrase.introduced bs m
    | .counterKindOn n => NounPhrase.introduced bs n
    | .happenedTo (.mk ev _) => GameEvent.mentioned bs ev
    | .castFrom z => ZoneExpr.introduced bs z
    | .named src => NameSource.introduced bs src
    | .hasDesignation _ none => []
    | .hasDesignation _ (some h) => NounPhrase.introduced bs h
    | .attachment _ _ (some counterpart) => NounPhrase.introduced bs counterpart
    | .inPile p => NounPhrase.introduced bs p
    | .inZone z => ZoneExpr.introduced bs z
    | .and ps => Predicate.introducedAll bs ps
    | .not _ => []
    | .or ps => if Predicate.joins ps then Predicate.introducedAll bs ps else []
    | .otherThan n => NounPhrase.introduced bs n
    | .compare _ _ b => Amount.introduced bs b
    | .superlative _ _ d => Predicate.introduced bs d
    | .withMostVotes => []
    | .choseExtreme _ => [outcomeB .namedNumber]
    | .compareOver dom _ _ bound => gapB :: (Predicate.introduced bs dom ++ Amount.introduced bs bound)
    | _ => []
  termination_by structural x => x

  def Predicate.introducedAll (bs : Bindings) : List Predicate → List Binding
    | [] => []
    | p :: ps => Predicate.introduced bs p ++ Predicate.introducedAll bs ps
  termination_by structural x => x

  def DetPhrase.introduced (bs : Bindings) (d : DetPhrase) (k : Kind) (p : Predicate)
      (pd : List Binding) : List Binding :=
    match d with
    | .target q => sized q.exact (bindFor .target q.plur k p) :: (Quantity.introduced bs q ++ pd)
    | .count q _ => sized q.exact (bindFor .count q.plur k p) :: (Quantity.introduced bs q ++ pd)
    | .the => if p.choiceRead then pd else bindFor .the .one k p :: pd
    | d => bindFor d.det d.plur k p :: pd

  def LibraryPlace.introduced (bs : Bindings) : LibraryPlace → List Binding
    | .oneEnd _ => []
    | .eitherEnd none => []
    | .eitherEnd (some n) => NounPhrase.introduced bs n
    | .shuffled => []


  def ZoneExpr.introduced (bs : Bindings) : ZoneExpr → List Binding
    | .zone _ (.possessedBy n) => NounPhrase.introduced bs n
    | .zone _ .bare => []
    | .library pl _ _ (.possessedBy n) => LibraryPlace.introduced bs pl ++ NounPhrase.introduced bs n
    | .library pl _ _ .bare => LibraryPlace.introduced bs pl
  termination_by structural x => x

  def ZoneExpr.introducedAll (bs : Bindings) : List ZoneExpr → List Binding
    | [] => []
    | z :: zs => ZoneExpr.introduced bs z ++ ZoneExpr.introducedAll bs zs
  termination_by structural x => x

  def NameSource.introduced (bs : Bindings) : NameSource → List Binding
    | .printed _ => []
    | .chosen => []
    | .sameAs n => NounPhrase.introduced bs n


  def EventSource.introduced (bs : Bindings) : EventSource → List Binding
    | .anywhere => []
    | .zones zs => ZoneExpr.introducedAll bs zs
    | .anywhereBut zs => ZoneExpr.introducedAll bs zs
  termination_by structural x => x

  def OptNoun.introduced (bs : Bindings) : Option NounPhrase → List Binding
    | none => []
    | some n => NounPhrase.introduced bs n
  termination_by structural n => n

  def OptEventSource.introduced (bs : Bindings) : Option EventSource → List Binding
    | none => []
    | some src => EventSource.introduced bs src
  termination_by structural src => src

  def OptZoneExpr.introduced (bs : Bindings) : Option ZoneExpr → List Binding
    | none => []
    | some z => ZoneExpr.introduced bs z
  termination_by structural z => z

  /-- Nouns mentioned in a historical event pattern. Observing history does not move the
  current bindings or introduce a new event outcome. The bound gap introduces nothing. -/
  def GameEvent.mentioned (bs : Bindings) : GameEvent → List Binding
    | .dies n | .isDealtDamage _ n | .draws n | .losesGame n | .statusEvent n _
    | .flipsCoin n _ | .paysLife n | .lifeChanges n _ | .triggers n | .commitsCrime n =>
      NounPhrase.introduced bs n
    | .leaves n src | .enters n src =>
      let ns := NounPhrase.introduced bs n
      OptEventSource.introduced (ns ++ bs) src ++ ns
    | .combat _ n other | .dealsDamage _ n other =>
      let ns := NounPhrase.introduced bs n
      OptNoun.introduced (ns ++ bs) other ++ ns
    | .attacksWith who whom attackers =>
      let ws := NounPhrase.introduced bs who
      let ds := OptNoun.introduced (ws ++ bs) whom
      NounPhrase.introduced (ds ++ ws ++ bs) attackers ++ ds ++ ws
    | .attachment _ n host | .becomesTarget n host | .activates n host =>
      let ns := NounPhrase.introduced bs n
      NounPhrase.introduced (ns ++ bs) host ++ ns
    | .beginningOf _ _ .noPossessor | .beginningOf _ _ (.byTurn _) => []
    | .beginningOf _ _ (.byPlayer n) => NounPhrase.introduced bs n
    | .casts who what src =>
      let ws := NounPhrase.introduced bs who
      let ns := OptNoun.introduced (ws ++ bs) what
      OptEventSource.introduced (ns ++ ws ++ bs) src ++ ns ++ ws
    | .gameBecomes _ | .stateHolds _ | .chapterMark _ => []
    | .putInto n dest src =>
      let ns := NounPhrase.introduced bs n
      let zs := ZoneExpr.introduced (ns ++ bs) dest
      OptEventSource.introduced (zs ++ ns ++ bs) src ++ zs ++ ns
    | .counterEvent _ _ n _ by_ _ =>
      let ns := NounPhrase.introduced bs n
      OptNoun.introduced (ns ++ bs) by_ ++ ns
    | .tokensCreated n _ by_ under =>
      let ns := NounPhrase.introduced bs n
      let ws := OptNoun.introduced (ns ++ bs) by_
      OptNoun.introduced (ws ++ ns ++ bs) under ++ ws ++ ns
    | .statBecomes n _ value =>
      let ns := NounPhrase.introduced bs n
      Amount.introduced (ns ++ bs) value ++ ns
    | .rollsDice n _ _ _ => NounPhrase.introduced bs n
    | .paysCost who _ whose _ =>
      let ws := OptNoun.introduced bs who
      NounPhrase.introduced (ws ++ bs) whose ++ ws
    | .verbedEvent who _ what becomes locus =>
      let ws := OptNoun.introduced bs who
      let ns := OptNoun.introduced (ws ++ bs) what
      let ps := OptPredicate.introduced (ns ++ ws ++ bs) becomes
      OptZoneExpr.introduced (ps ++ ns ++ ws ++ bs) locus ++ ps ++ ns ++ ws
    | .tappedForMana who what _ =>
      let ws := OptNoun.introduced bs who
      NounPhrase.introduced (ws ++ bs) what ++ ws
    | .unlocksDoor who .thisDoor => NounPhrase.introduced bs who
    | .unlocksDoor who (.doorOf _ room) =>
      let ws := NounPhrase.introduced bs who
      NounPhrase.introduced (ws ++ bs) room ++ ws
    | .nthOccurrence _ _ ev => GameEvent.mentioned bs ev
    | .causes cause ev =>
      let cs := Causing.mentioned bs cause
      GameEvent.mentioned (cs ++ bs) ev ++ cs
  termination_by structural ev => ev

  def Causing.mentioned (bs : Bindings) : Causing → List Binding
    | .source n => NounPhrase.introduced bs n
    | .event ev => GameEvent.mentioned bs ev
    | .anEffect => []
  termination_by structural cause => cause

  def Amount.introduced (bs : Bindings) : Amount → List Binding
    | .lit _ => []
    | .statOf _ nom => NounPhrase.introduced bs nom
    | .paid _ n => NounPhrase.introduced bs n
    | .eventTally _ who (.mk ev _) =>
      NounPhrase.introduced bs who ++ GameEvent.mentioned (NounPhrase.introduced bs who ++ bs) ev
    | .countOf g => NounPhrase.introduced bs g
    | .aggregate _ _ g => NounPhrase.introduced bs g
    | .thatMuch | .chosenNumber _ | .votesFor _ | .theOutcome _ | .coinsShowing _
    | .greatestStoredMatch _ | .groupSize | .theDifference => []
    | .letter l => introducedLetters l bs
    | .arith _ a b => Amount.introduced bs a ++ Amount.introduced (Amount.intro bs a) b
    | .devotion who _ _ => NounPhrase.introduced bs who
    | .half _ a => Amount.introduced bs a
    | .aggregateOver _ dom _ => Predicate.introduced bs dom
    | .distinctCount _ dom => NounPhrase.introduced bs dom
    | .upTo b => outcomeB .ceilingShortfall :: Amount.introduced bs b
  termination_by structural x => x

  /-- The stack after an amount, Idris `amtIntro`: not always `introduced ++ bs`, because a nested
  amount is read at its outer amount's own intro. -/
  def Amount.intro (bs : Bindings) : Amount → Bindings
    | .lit _ => bs
    | .statOf _ nom => NounPhrase.introduced bs nom ++ bs
    | .paid _ n => NounPhrase.introduced bs n ++ bs
    | .eventTally _ who (.mk ev _) =>
      GameEvent.mentioned (NounPhrase.introduced bs who ++ bs) ev ++ NounPhrase.introduced bs who ++
        bs
    | .countOf g => NounPhrase.introduced bs g ++ bs
    | .aggregate _ _ g => NounPhrase.introduced bs g ++ bs
    | .thatMuch | .chosenNumber _ | .votesFor _ | .theOutcome _ | .coinsShowing _
    | .greatestStoredMatch _ | .groupSize | .theDifference => bs
    | .letter l => introducedLetters l bs ++ bs
    | .arith _ a b => Amount.intro (Amount.intro bs a) b
    | .devotion who _ _ => NounPhrase.introduced bs who ++ bs
    | .half _ a => Amount.intro bs a
    | .aggregateOver _ dom _ => Predicate.introduced bs dom ++ bs
    | .distinctCount _ dom => NounPhrase.introduced bs dom ++ bs
    | .upTo b => outcomeB .ceilingShortfall :: Amount.intro bs b
  termination_by structural x => x

  def Quantity.introduced (bs : Bindings) : Quantity → List Binding
    | .range _ _ => []
    | .upToOf a => Amount.introduced bs a
    | .exactlyOf a => Amount.introduced bs a


  def SliceCount.introduced (bs : Bindings) : SliceCount → List Binding
    | .counted q => Quantity.introduced bs q
    | .whole => []


  def NounPhrase.selfSubjIntroduced : NounPhrase → List Binding
    | .asType t .this _ =>
      [⟨.self, .one, .object [t] (some .battlefield) none none none⟩]
    | .attachHost _ word =>
      match word.base with
      | .type _ | .permanent =>
        [⟨.the, .one, .object word.attachHostTy (some .battlefield) none none none⟩]
      | .player => [⟨.the, .one, .player false⟩]
      | _ => []
    | _ => []


  def NounPhrase.commonZone (bs : Bindings) : List NounPhrase → Option Zone
    | [] => none
    | [n] => NounPhrase.zone bs n
    | n :: ns =>
      if NounPhrase.zone bs n == NounPhrase.commonZone bs ns then NounPhrase.zone bs n else none
  termination_by structural ns => ns

  def NounPhrase.commonTy (bs : Bindings) : List NounPhrase → List CardType
    | [] => []
    | [n] => NounPhrase.ty bs n
    | n :: ns => commonTypes (NounPhrase.ty bs n) (NounPhrase.commonTy bs ns)
  termination_by structural ns => ns

  def NounPhrase.zone (bs : Bindings) : NounPhrase → Option Zone
    | .gap _ => none
    | .this | .combatPlayer _ | .you | .playerGroup _ | .or _
    | .possessorOf _ _ | .designated _ _ => none
    | .asType _ _ _ => some .battlefield
    | .resolvedPermanent _ => some .battlefield
    | .asMarker m _ => some m.zone
    | .theGrantor m => some m.zone
    | .described _ p => p.phraseZone (p.kindOr .object)
    | .eachOf g => NounPhrase.zone bs g
    | .namesAgree _ g => NounPhrase.zone bs g
    | .and ns => NounPhrase.commonZone bs ns
    | .librarySlice _ _ _ => some .library
    | .someOf _ _ g => NounPhrase.zone bs g
    | .theRest k _ => zoneOfGroup k bs
    | .pileOf _ _ => zoneOfReach (.word .pile) .many bs
    | .pro r pl w => zoneOfReach r pl (view w bs)
    | .attachHost _ h => h.attachHostZone
    | .oneEachOf _ pool => NounPhrase.zone bs pool
  termination_by structural x => x

  def NounPhrase.ty (bs : Bindings) : NounPhrase → List CardType
    | .gap _ => []
    | .this | .theGrantor _ | .combatPlayer _ | .you | .playerGroup _
    | .or _ | .librarySlice _ _ _ | .pileOf _ _ | .possessorOf _ _ | .designated _ _ => []
    | .asType t _ _ => [t]
    | .resolvedPermanent n => NounPhrase.ty bs n
    | .asMarker _ n => NounPhrase.ty bs n
    | .described _ p => p.seedTy
    | .eachOf g => NounPhrase.ty bs g
    | .namesAgree _ g => NounPhrase.ty bs g
    | .and ns => NounPhrase.commonTy bs ns
    | .someOf _ d g => sliceTyOf d (NounPhrase.ty bs g)
    | .theRest k _ => tyOfGroup k bs
    | .pro r pl w => tyOfReach r pl (view w bs)
    | .attachHost _ h => h.attachHostTy
    | .oneEachOf _ pool => NounPhrase.ty bs pool
  termination_by structural x => x

end

/-- Whether the phrase introduces its own referent first, before mentions nested in its
arguments. Coordinated references have no single such head. -/
def NounPhrase.introducesOwnReferent (n : NounPhrase) : Bool :=
  !(NounPhrase.selfSubjIntroduced n).isEmpty || nominal n
where
  nominal : NounPhrase → Bool
    | .asType _ n _ | .resolvedPermanent n | .asMarker _ n | .eachOf n | .namesAgree _ n =>
      nominal n
    | .described .the p => !p.choiceRead
    | .described _ _ | .librarySlice _ _ _ | .someOf _ _ _ | .pileOf _ _
    | .possessorOf _ _ | .oneEachOf _ _ => true
    | _ => false

def nomIntro (bs : Bindings) (n : NounPhrase) : Bindings := NounPhrase.introduced bs n ++ bs

def sliceTy (bs : Bindings) (d : Option Predicate) (g : NounPhrase) : List CardType :=
  sliceTyOf d (NounPhrase.ty bs g)

mutual
  def NounPhrase.headTys (bs : Bindings) : NounPhrase → List (List CardType)
    | .described _ p =>
      let alts := p.headTyAlts
      if alts.all List.isEmpty then [] else alts
    | .eachOf g => NounPhrase.headTys bs g
    | .namesAgree _ g => NounPhrase.headTys bs g
    | .resolvedPermanent n => NounPhrase.headTys bs n
    | .asMarker _ n => NounPhrase.headTys bs n
    | .and ns | .or ns => NounPhrase.headTysAll bs ns
    | .oneEachOf _ pool => NounPhrase.headTys bs pool
    | n => soleAlt (NounPhrase.ty bs n)
  termination_by structural n => n

  def NounPhrase.headTysAll (bs : Bindings) : List NounPhrase → List (List CardType)
    | [] => []
    | n :: ns => NounPhrase.headTys bs n ++ NounPhrase.headTysAll bs ns
  termination_by structural ns => ns
end

mutual
  def NounPhrase.tys (bs : Bindings) : NounPhrase → HeadTy
    | .described _ p => p.seedTys
    | .eachOf g => NounPhrase.tys bs g
    | .namesAgree _ g => NounPhrase.tys bs g
    | .someOf _ d g => .sole (sliceTyOf d (NounPhrase.ty bs g))
    | .and ns => NounPhrase.coordinatedTys true bs ns
    | .or ns => NounPhrase.coordinatedTys false bs ns
    | n => .sole (NounPhrase.ty bs n)
  termination_by structural n => n

  def NounPhrase.coordinatedTys (sequential : Bool) (bs : Bindings) : List NounPhrase → HeadTy
    | [] => .sole []
    | [n] => NounPhrase.tys bs n
    | n :: ns =>
      if n.kindOr .object == NounPhrase.kindOfAll ns then
        .sole (if sequential then NounPhrase.commonTy bs (n :: ns) else [])
      else .join (NounPhrase.tys bs n)
        (NounPhrase.coordinatedTys sequential (if sequential then nomIntro bs n else bs) ns)
  termination_by structural ns => ns
end

def NounPhrase.prov (bs : Bindings) : NounPhrase → Option Stamp
  | .theRest k _ => provOfGroup k bs
  | .pro r pl w => provOfReach r pl (view w bs)
  | .eachOf g => NounPhrase.prov bs g
  | .namesAgree _ g => NounPhrase.prov bs g
  | .someOf _ _ g => NounPhrase.prov bs g
  | _ => none

def NounPhrase.paidSubjectOk (bs : Bindings) : NounPhrase → Bool
  | .this => true
  | .asType _ n _ => NounPhrase.paidSubjectOk bs n
  | .asMarker _ n => NounPhrase.paidSubjectOk bs n
  | n => onStackZone (NounPhrase.zone bs n)

def NounPhrase.costSubjectOk (bs : Bindings) : NounPhrase → Bool
  | .this => true
  | n => onStackZone (NounPhrase.zone bs n)

def NounPhrase.counterMemoryOk (bs : Bindings) : NounPhrase → Bool
  | .pro (.verbed _ _ _) _ _ => false
  | .pro r pl w => !r.tracksObject || !stampMoves (provOfReach r pl (view w bs))
  | _ => true

def NounPhrase.testSubjectOk (bs : Bindings) : NounPhrase → Bool
  | .described .the _ => true
  | .librarySlice _ _ _ => true
  | n => (NounPhrase.introduced bs n).isEmpty

def NounPhrase.bindingless (bs : Bindings) (n : NounPhrase) : Bool := (NounPhrase.introduced bs n).isEmpty

/-- Idris `EventAgent`: an optional agent that introduces nothing. -/
def eventAgentOk (bs : Bindings) : Option NounPhrase → Bool
  | none => true
  | some n => NounPhrase.bindingless bs n

def attackableKind : Kind → HeadTy → Bool
  | .player, _ => true
  | .object, .sole t => deedAltOk (.core .attack) .patient t
  | .join a b, .join l r => attackableKind a l && attackableKind b r
  | .join a b, .sole t => attackableKind a (.sole t) && attackableKind b (.sole t)
  | _, _ => false

/-- The attacker of an attacked player, planeswalker or battle: a creature on the battlefield,
or the attacking player themself [CR#506.2]. -/
def combatPartyKind : Kind → Option Zone → Bool
  | .player, _ => true
  | .object, z => zoneIsB z .battlefield
  | .join a b, z => combatPartyKind a z && combatPartyKind b z
  | _, _ => false

def combatRelOk : CombatRelation → Kind → Kind → Option Zone → HeadTy → Bool
  | .attackerOf, k, km, _, tys => k == .object && attackableKind km tys
  | .attackedBy, _, km, z, _ => combatPartyKind km z
  | _, k, km, z, _ => k == .object && km == .object && zoneIsB z .battlefield

def damageableKind : Kind → HeadTy → Bool
  | .player, _ => true
  | .object, .sole t => damageableHeadTysOk (soleAlt t)
  | .join a b, .join l r => damageableKind a l && damageableKind b r
  | .join a b, .sole t => damageableKind a (.sole t) && damageableKind b (.sole t)
  | _, _ => false

/-- Idris `DamageRecipient n`: a player, a join with damageable halves, or an object on the
battlefield whose head types can be dealt damage. -/
def NounPhrase.damageRecipient (bs : Bindings) (n : NounPhrase) : Bool :=
  match n.kindOr .object with
  | .player => true
  | .join a b => damageableKind (.join a b) (NounPhrase.tys bs n)
  | .object => zoneIsB (NounPhrase.zone bs n) .battlefield && damageableHeadTysOk (NounPhrase.headTys bs n)
  | _ => false

def NounPhrase.attackable (bs : Bindings) (n : NounPhrase) : Bool :=
  attackableKind (n.kindOr .object) (NounPhrase.tys bs n)

def hostedRead (bs : Bindings) (p : Predicate) (n : NounPhrase) : Bool :=
  match p.qualityReadHost with
  | none => true
  | some h => tyIs h (NounPhrase.ty bs n)

/-- Idris `PileMention`: a pile phrase, "that pile", or "those piles". -/
def NounPhrase.pileMention : NounPhrase → Bool
  | .pileOf _ _ => true
  | .pro (.word .pile) _ .whole => true
  | _ => false

/-- Idris `LinkSource`: "exiled with this" names the source itself. -/
def NounPhrase.linkSource : NounPhrase → Bool
  | .this => true
  | .asType _ .this _ => true
  | _ => false

/-- Idris `StackActOn p n`: a spell on the stack, or a join whose kinds `p` admits. -/
def NounPhrase.stackActOn (bs : Bindings) (p : Kind → Bool) (n : NounPhrase) : Bool :=
  match n.kindOr .object with
  | .join a b => p (.join a b)
  | _ => zoneIsB (NounPhrase.zone bs n) .stack

def NounPhrase.counterable (bs : Bindings) (n : NounPhrase) : Bool := n.stackActOn bs counterKind
def NounPhrase.copiable (bs : Bindings) (n : NounPhrase) : Bool := n.stackActOn bs copyKind

def copySourceOk (bs : Bindings) : CopySort → NounPhrase → Bool
  | .fromStack, n => n.copiable bs
  | .fromCardZone, n => isCardZone (NounPhrase.zone bs n)

/-- Objects and piles can move. Abilities can be exiled with other stack objects
[CR#724.1b,724.2b]. -/
def NounPhrase.movable (n : NounPhrase) : Bool :=
  match n.kindOr .object with
  | .object => true
  | .pile => true
  | _ => false

/-- Idris `DestOk`: a bare battlefield, exile, hand, or graveyard, or a bare library place. -/
def ZoneExpr.destOk : ZoneExpr → Bool
  | .zone .battlefield .bare | .zone .exile .bare | .zone .hand .bare
  | .zone .graveyard .bare => true
  | .library place arrg offs .bare => place.arrangementOk arrg && place.ordinalOk offs
  | _ => false

def orderOk (pl : Plurality) (z : ZoneExpr) : Bool :=
  match z.arrangement with
  | none => true
  | some _ => !pl.isOne

def NounPhrase.discardOk (bs : Bindings) : NounPhrase → Bool
  | .this => true
  | n => NounPhrase.zone bs n == some .hand

def deedNounOk (bs : Bindings) (v : Deed) (r : Role) (n : NounPhrase) : Bool :=
  match NounPhrase.headTys bs n with
  | [] => n.det.isNone && deedBareOk v r
  | ts => ts.all fun types =>
      if types.isEmpty then n.det.isNone && deedBareOk v r else deedAltOk v r types

/-- Who declares an attack: the active player [CR#508.1] or a creature they control
[CR#508.1a]. The attack deed's agent role decides which kinds attack and keeps the object
reading's type gate, and an object attacker is a battlefield permanent: `attackableKind`'s
counterpart on the declaring side. -/
def NounPhrase.attackerOk (bs : Bindings) (n : NounPhrase) : Bool :=
  let k := n.kindOr .object
  deedKindOk (.core .attack) .agent k && deedNounOk bs (.core .attack) .agent n &&
    combatPartyKind k (NounPhrase.zone bs n)

/-! ## Agents, choices, and the stacks they leave -/

def NounPhrase.agentIntroduced (bs : Bindings) : NounPhrase → List Binding
  | .described .each p => bindFor .the .one (p.kindOr .object) p :: Predicate.introduced bs p
  | n => NounPhrase.introduced bs n

def agentIntro (bs : Bindings) (n : NounPhrase) : Bindings := NounPhrase.agentIntroduced bs n ++ bs

def agentCtx (bs : Bindings) : Option NounPhrase → Bindings
  | none => bs
  | some n => agentIntro bs n

def kindValueIntro (bs : Bindings) (q : QualitySort) : Option NounPhrase → Bindings
  | some dom => qualityB q :: nomIntro bs dom
  | none => qualityB q :: bs

def kindDomainOk : KindAxis → Option NounPhrase → Bool
  | _, some _ => true
  | ax, none => ax.closed

def NounPhrase.chosenIntroduced (bs : Bindings) : NounPhrase → List Binding
  | .described (.a _) p =>
    let k := p.kindOr .object
    chosenBind (chosenDet k .a) .one k p :: Predicate.introduced bs p
  | .described (.count q _) p =>
    let k := p.kindOr .object
    chosenBind (chosenDet k .count) q.plur k p :: (Quantity.introduced bs q ++ Predicate.introduced bs p)
  | .namesAgree _ g => NounPhrase.chosenIntroduced bs g
  | n => NounPhrase.introduced bs n

def chosenIntro (bs : Bindings) (n : NounPhrase) : Bindings := NounPhrase.chosenIntroduced bs n ++ bs

def chosenIntroBy (bs : Bindings) : Plurality → NounPhrase → NounPhrase → Bindings
  | .one, by_, n => NounPhrase.introduced bs by_ ++ (NounPhrase.chosenIntroduced (agentIntro bs by_) n ++ bs)
  | .many, by_, n => pluralizeIntroduced (NounPhrase.chosenIntroduced (agentIntro bs by_) n) ++ nomIntro bs by_

def chooseIntro (bs : Bindings) : Option NounPhrase → NounPhrase → Bindings
  | none, n => chosenIntro bs n
  | some by_, n => chosenIntroBy bs by_.plur by_ n

def optAmtIntro (bs : Bindings) : Option Amount → Bindings
  | none => bs
  | some a => Amount.intro bs a

def Delta.intro (bs : Bindings) (d : Delta Amount) : Bindings := Amount.intro bs d.amount
def Delta.introduced (bs : Bindings) (d : Delta Amount) : List Binding := Amount.introduced bs d.amount

def FlipScope.intro (bs : Bindings) : FlipScope → Bindings
  | .count n => Amount.intro bs n
  | .per each => nomIntro bs each

def IgnoredOutcomes.intro (bs : Bindings) : IgnoredOutcomes → Bindings
  | .extreme _ => bs
  | .allBut _ => bs
  | .chosen _ n => Amount.intro bs n

def IgnoredOutcomes.ignorableFor (bs : Bindings) : IgnoredOutcomes → Bool
  | .extreme _ => countOutcomes .rollResult bs == 1
  | .allBut _ => countOutcomes .rollResult bs == 1
  | .chosen _ _ => ignorableInScope bs

def Quantity.intro (bs : Bindings) : Quantity → Bindings
  | .range _ _ => bs
  | .upToOf a => Amount.intro bs a
  | .exactlyOf a => Amount.intro bs a

def optQuantIntro (bs : Bindings) : Option Quantity → Bindings
  | none => bs
  | some q => q.intro bs

def SearchScope.zone : SearchScope → Option Zone
  | .oneZone z => some z.sort
  | .someZones _ _ => none

def SearchScope.introduced (bs : Bindings) : SearchScope → List Binding
  | .oneZone z => ZoneExpr.introduced bs z
  | .someZones none _ => []
  | .someZones (some whose) _ => NounPhrase.introduced bs whose

def Exposed.intro (bs : Bindings) : Exposed → Bindings
  | .cards n => nomIntro bs n
  | .zone z => ZoneExpr.introduced bs z ++ bs
  | .choice _ => bs

def VisibleThing.intro (bs : Bindings) : VisibleThing → Bindings
  | .topOfLibrary => bs
  | .wholeHand => bs
  | .objects n => nomIntro bs n

def visibilityOk : ExposeVerb → VisibleThing → Bool
  | .reveal, .topOfLibrary => true
  | .reveal, .wholeHand => true
  | .lookAt, .topOfLibrary => true
  | .lookAt, .wholeHand => false
  | _, .objects _ => true

def selfSubjIntro (bs : Bindings) (n : NounPhrase) : Bindings := NounPhrase.selfSubjIntroduced n ++ nomIntro bs n

/-- Resolve a referential operand to its actual slot, never by comparing payload values. -/
def NounPhrase.referenceAddress (bs : Bindings) : NounPhrase → Option (List Nat)
  | .pro r pl w => (windowAddresses w bs).find? fun address =>
      (bindingAt bs address).elim false (reaches r pl)
  | .asType _ n _ | .asMarker _ n | .resolvedPermanent n | .namesAgree _ n =>
      n.referenceAddress bs
  | _ => none

/-- Explicit resolved/type/marker views retain their meaning when captured. -/
def NounPhrase.captureView (bs : Bindings) (subject : NounPhrase) (value : Binding) : Binding :=
  match subject with
  | .asType _ _ _ | .asMarker _ _ | .resolvedPermanent _ =>
    let payload := match value.payload with
      | .object _ _ prov origin size =>
        .object (subject.ty bs) (subject.zone bs) prov origin size
      | .ability origin _ => .ability origin (subject.zone bs)
      | p => p
    { value with payload }
  | _ => value

abbrev OperandSlot := Option (List Nat) × Determiner × Plurality × Payload

/-- The public noun introductions stay in the ordinary context. The private frame points
at those same slots, so moving a captured operand updates subsequent ordinary reads. -/
def captureOperands (bs : Bindings) (subjects : List NounPhrase) : Bindings :=
  let (context, slots) := subjects.foldl (fun (context, slots) subject =>
    let next := nomIntro context subject
    let width := next.length - context.length
    let address := if subject.introducesOwnReferent && width > 0 then some [0]
      else (subject.referenceAddress context).map (shiftAddress width)
    let value := (address >>= bindingAt next).getD
      ⟨.the, subject.plur, elemPayload (subject.kindOr .object) subject.isAbility
        (subject.ty context) (subject.zone context) none⟩
    let value := subject.captureView context value
    let next := address.elim next (fun a => setBindingAt next a value)
    let shifted := slots.map fun (a, d, p, v) => (a.map (shiftAddress width), d, p, v)
    (next, shifted ++ [(address, value.det, value.plur, value.payload)]))
    (bs, ([] : List OperandSlot))
  ⟨.the, .one, .operandFrame slots⟩ :: context


def subjCtx (bs : Bindings) : Option NounPhrase → Bindings
  | none => bs
  | some n => selfSubjIntro bs n

def elemIntro (bs : Bindings) (k : Kind) (g : NounPhrase) : Bindings :=
  ⟨.the, .one, elemPayload k g.isAbility (NounPhrase.ty bs g) (NounPhrase.zone bs g) (NounPhrase.prov bs g)⟩
    :: nomIntro bs g

def Condition.negated : Condition → Bool
  | .not _ => true
  | _ => false

def markingOk : CondMarking → Condition → Bool
  | .asLongAs, _ => true
  | .ifSo, _ => true
  | .unless_, c => c.negated

def dropGaps : List Binding → List Binding
  | [] => []
  | ⟨_, _, .gap⟩ :: bs => dropGaps bs
  | b :: bs => b :: dropGaps bs

mutual
  def Condition.introduced (bs : Bindings) : Condition → List Binding
    | .exists_ _ => []
    | .happened who _ => NounPhrase.selfSubjIntroduced who
    | .gameIs _ => []
    | .noHolder _ => []
    | .matches n _ => NounPhrase.introduced bs n ++ NounPhrase.selfSubjIntroduced n
    | .compareAmt subj _ bound => gapB :: (Amount.introduced (Amount.intro bs subj) bound ++ Amount.introduced bs subj)
    | .dealtThisWay _ => []
    | .choseThisWay who _ => NounPhrase.selfSubjIntroduced who
    | .preventedFromSource _ => []
    | .flipCalled _ _ => []
    | .flipFace _ => []
    | .voteLead _ _ => []
    | .anyResultIs _ _ => []
    | .rolledDoubles => []
    | .not (.compareAmt subj _ bound) =>
      Amount.introduced (Amount.intro bs subj) bound ++ Amount.introduced bs subj
    | .not c => dropGaps (Condition.introduced bs c)
    | .and cs => Condition.introducedAll bs cs
    | .or _ => []

  def Condition.introducedAll (bs : Bindings) : List Condition → List Binding
    | [] => []
    | c :: cs => Condition.introduced bs c ++ Condition.introducedAll bs cs
end

def Condition.remarkAt : Condition → Option ((Binding → Bool) × List CardType)
  | .matches n p =>
    match n.remarkTest with
    | none => none
    | some q => some (q, p.seedTy)
  | _ => none

mutual
  def Condition.remark (bs : Bindings) : Condition → Bindings
    | .matches (.pro r pl w) p => overWindow (markFirst (reaches r pl) p.seedTy) w bs
    | .and cs => Condition.remarkAll bs cs
    | c =>
      match c.remarkAt with
      | none => bs
      | some (q, t) => markFirst q t bs
  termination_by structural c => c

  def Condition.remarkAll (bs : Bindings) : List Condition → Bindings
    | [] => bs
    | c :: cs => Condition.remarkAll (Condition.remark bs c) cs
  termination_by structural cs => cs
end

def Condition.intro (bs : Bindings) (c : Condition) : Bindings := Condition.introduced bs c ++ c.remark bs

def interveningIntro (bs : Bindings) : Option Condition → Bindings
  | none => bs
  | some c => c.intro bs

def playSourceOk : Option Zone → Option ZoneExpr → Bool → Bool
  | zn, none, false => zn.elim true Zone.placementDestOk
  | zn, some z, false =>
    playableFrom (some z.sort) && (!zn.elim true Zone.placementDestOk || zoneFits zn (some z.sort))
  | _, none, true => true
  | _, some z, true => playableFrom (some z.sort)

def Condition.isAnd : Condition → Bool
  | .and _ => true
  | _ => false

def Condition.isOr : Condition → Bool
  | .or _ => true
  | _ => false

def atLeastTwoCs : List Condition → Bool
  | _ :: _ :: _ => true
  | _ => false

/-! ## Moving a binding -/

def setZone (p : Option Deed) (z : Option Zone) (b : Binding) : Binding :=
  match b.payload with
  | .object ty oldZn _ og sz =>
    { b with payload := .object ty z (mkStamp p oldZn (oldZn != z)) og sz }
  | .pile _ sz fc => { b with payload := .pile z sz fc }
  | .ability og _ => { b with payload := .ability og z }
  | _ => b

def setZoneHead (p : Option Deed) (z : Option Zone) : Bindings → Bindings
  | [] => []
  | b :: bs => setZone p z b :: bs

def setZoneReach (r : Reach) (pl : Plurality) (p : Option Deed) (z : Option Zone) :
    Bindings → Bindings
  | [] => []
  | b :: bs => if reaches r pl b then setZone p z b :: bs else b :: setZoneReach r pl p z bs

/-- The stack after `n` moves to `z` under verb `p`, Idris `moveIntro`. -/
def moveIntro (bs : Bindings) (p : Option Deed) (n : NounPhrase) (z : Option Zone) : Bindings :=
  match n with
  | .gap _ => bs
  | .described _ _ | .librarySlice _ _ _ | .someOf _ _ _ | .oneEachOf _ _ | .pileOf _ _ =>
    setZoneHead p z (nomIntro bs n)
  | .eachOf g => moveIntro bs p g z
  | .namesAgree _ g => moveIntro bs p g z
  | .and _ | .or _ => nomIntro bs n
  | .theRest k _ => groupSpent k bs
  | .pro r pl w => overWindow (setZoneReach r pl p z) w bs
  | .this => ⟨.self, .one, .object [] z (mkStamp p none z.isSome) none none⟩ :: bs
  | .attachHost _ word =>
    match word.base with
    | .type _ | .permanent =>
      ⟨.the, .one,
        .object word.attachHostTy z (mkStamp p (some .battlefield) (z != some .battlefield))
          none none⟩ :: bs
    | .player => ⟨.the, .one, .player false⟩ :: bs
    | _ => bs
  | .asType t .this _ =>
    ⟨.self, .one,
      .object [t] z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .asType t _ _ =>
    ⟨.the, .one,
      .object [t] z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .resolvedPermanent m =>
    ⟨.the, m.plur,
      .object (NounPhrase.ty bs m) z (mkStamp p (some .battlefield) (z != some .battlefield)) none none⟩
      :: bs
  | .asMarker .ability .this => ⟨.self, .one, .ability none z⟩ :: bs
  | .asMarker .ability _ => ⟨.the, .one, .ability none z⟩ :: bs
  | .asMarker _ .this =>
    ⟨.self, .one, .object [] z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .asMarker _ _ =>
    ⟨.the, .one, .object [] z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .theGrantor m =>
    ⟨.the, .one,
      .object [] z (mkStamp p m.grantorOrigin (z != some m.zone)) none none⟩ :: bs
  | .you | .combatPlayer _ | .playerGroup _ | .designated _ _ => bs
  | .possessorOf ax m => nomIntro bs (.possessorOf ax m)
termination_by structural n

def stampIntro (bs : Bindings) (p : Option Deed) (n : NounPhrase) : Bindings :=
  moveIntro bs p n (NounPhrase.zone bs n)

end Semantics

namespace Semantics

/-! ## `other` anchoring -/

def complementAnchorsOk (bs : Bindings) (ts : List CardType) : List Predicate → Bool
  | [] => true
  | .otherThan n :: ps => complementAnchorsOk bs ts ps && anchorTyFits ts (NounPhrase.ty bs n)
  | _ :: ps => complementAnchorsOk bs ts ps

def otherAnchorOk (bs : Bindings) (k : Kind) (ts : List CardType) (ps : List Predicate) : Bool :=
  let fs := Predicate.flatten ps
  if atMostOne (countOthers fs)
    then (if Predicate.hasBareOtherAny ps then anyTargeted k bs else complementAnchorsOk bs ts fs)
    else false

end Semantics
