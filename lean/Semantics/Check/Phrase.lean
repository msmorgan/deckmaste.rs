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

def NounPhrase.kind? : NounPhrase → Option Kind
  | .you | .combatPlayer _ | .playerGroup _ | .possessorOf _ _ => some .player
  | .described _ p => p.kind?
  | .eachOf g => g.kind?
  | .both l r => some (joinKinds (l.kind?.getD .object) (r.kind?.getD .object))
  | .eitherOf l r => some (joinKinds (l.kind?.getD .object) (r.kind?.getD .object))
  | .theRest k _ => some k
  | .pileOf _ _ => some .pile
  | .pro r _ _ => some r.kind
  | .attachHost _ h => some h.kind
  | _ => some .object

def NounPhrase.kindOr (d : Kind) (n : NounPhrase) : Kind := n.kind?.getD d

/-! ## Leaf helpers -/

def optCT : Option CardType → List CardType
  | none => []
  | some t => [t]

def soleAlt : List CardType → List (List CardType)
  | [] => []
  | ts => [ts]

def attachTysOk (w : AttachWord) (ts : List CardType) : Bool :=
  ts.all fun t => attachHeadOk w (.type t)

def attachWordsOk (ws : List AttachWord) (alts : List (List CardType)) : Bool :=
  ws.all fun w => alts.all (attachTysOk w)

def zoneAdmits (z : Zone) : List Zone → Bool
  | [] => true
  | zs => zs.elem z

def allNegated (negs : List CardType) : List CardType → Bool
  | [] => true
  | t :: ts => negs.elem t && allNegated negs ts

def anchorTyFits : List CardType → Option CardType → Bool
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
  | .object, .join _ _ => .object none none none none none
  | .player, _ => .player false
  | .quality q, _ => .quality q
  | .join l r, .join a b => .join (joinHalfPayload l a) (joinHalfPayload r b)
  | .join l r, .sole ty => .join (joinHalfPayload l (.sole ty)) (joinHalfPayload r (.sole ty))
  | _, _ => .gap

def elemPayload : Kind → Bool → Option CardType → Option Zone → Option Stamp → Payload
  | .object, true, _, _, _ => .ability none
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

def afterMoveTo (to : ZoneExpr) (out : Bindings) : Bindings :=
  if to.shuffles then afterShuffle out else out

def sourceZone : Option EventSource → Option Zone
  | some (.zones [z]) => some z.sort
  | _ => none

def lookbackZonesOk (ev : EventName) : List ZoneExpr → Bool
  | [] => false
  | zs => zs.all fun z => lookbackOriginOk ev z.sort

def lookbackSourceOk (ev : EventName) : EventSource → Bool
  | .anywhere => eventNamesOrigin ev
  | .zones zs => lookbackZonesOk ev zs
  | .anywhereBut zs => lookbackZonesOk ev zs

def complementPlain : Option EventComplement → Bool
  | none => true
  | some (.involving _) => true
  | some _ => false

def complementSourced : Option EventComplement → Bool
  | none => true
  | some (.involving _) => true
  | some (.fromZones _ _) => true
  | some _ => false

def LookbackClause.event : LookbackClause → EventName
  | .mk ev _ _ => ev

def LookbackClause.window : LookbackClause → Lookback
  | .mk _ w _ => w

def LookbackClause.complement : LookbackClause → Option EventComplement
  | .mk _ _ what => what

/-! ## What a predicate seeds -/

mutual
  def Predicate.seedTy : Predicate → Option CardType
    | .hasType t => some t
    | .hasSubtype s => s.type
    | .and ps => Predicate.seedTyAll ps
    | .or ps => Predicate.seedTyJoin ps
    | .compareOver dom _ _ _ => dom.seedTy
    | _ => none

  def Predicate.seedTyAll : List Predicate → Option CardType
    | [] => none
    | p :: ps =>
      match p.seedTy with
      | some t => some t
      | none => Predicate.seedTyAll ps

  def Predicate.seedTyJoin : List Predicate → Option CardType
    | [] => none
    | p :: ps =>
      match p.seedTy with
      | none => none
      | some t => if Predicate.allSeedTy t ps then some t else none

  def Predicate.allSeedTy (t : CardType) : List Predicate → Bool
    | [] => true
    | p :: ps =>
      match p.seedTy with
      | none => false
      | some u => t == u && Predicate.allSeedTy t ps
end

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
  | [] => .sole none
  | [k] => .sole (Predicate.seedTyJoin (Predicate.ofKind k ps))
  | k :: ks => .join (.sole (Predicate.seedTyJoin (Predicate.ofKind k ps))) (joinedSeedTys ps ks)

def Predicate.seedTys : Predicate → HeadTy
  | .or ps => if Predicate.joins ps then joinedSeedTys ps (Predicate.disjunctKinds ps) else .sole (Predicate.seedTyJoin ps)
  | p => .sole p.seedTy

mutual
  def Predicate.headTys : Predicate → List CardType
    | .and ps => Predicate.headTysAll ps
    | .or ps => Predicate.headTysJoin ps
    | p => optCT p.seedTy

  def Predicate.headTysJoin : List Predicate → List CardType
    | [] => []
    | p :: ps => p.headTys ++ Predicate.headTysJoin ps

  def Predicate.headTysAll : List Predicate → List CardType
    | [] => []
    | p :: ps =>
      match p.headTys with
      | [] => Predicate.headTysAll ps
      | ts => ts
end

mutual
  def Predicate.headTyAlts : Predicate → List (List CardType)
    | .and ps => soleAlt (Predicate.headTysJoin ps)
    | .or ps => Predicate.headTyAltsJoin ps
    | p => soleAlt (optCT p.seedTy)

  def Predicate.headTyAltsJoin : List Predicate → List (List CardType)
    | [] => []
    | p :: ps => p.headTyAlts ++ Predicate.headTyAltsJoin ps
end

mutual
  def Predicate.attachWordsIn : Predicate → List AttachWord
    | .isAttached (some w) => [w]
    | .attachedBy (some w) _ => [w]
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
    | .isAttached _ | .attachedBy _ _ | .attachedTo _ | .isToken | .isTransformed | .hasStatus _ =>
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
    | .isTransformed => false
    | .and ps => Predicate.seedsTokenAny ps
    | .or ps => Predicate.seedsTokenAll ps
    | .compareOver dom _ _ _ => dom.seedsToken
    | _ => false

  def Predicate.seedsTokenAny : List Predicate → Bool
    | [] => false
    | p :: ps => p.seedsToken || Predicate.seedsTokenAny ps

  def Predicate.seedsTokenAll : List Predicate → Bool
    | [] => false
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

  def Predicate.seedsAbilityAll : List Predicate → Bool
    | [] => false
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
    match p.seedTy with
    | some t => !t.permanent || anyNonPermanentTy ps
    | none => anyNonPermanentTy ps

def Predicate.negTypesOf : Predicate → List CardType
  | .not (.hasSubtype _) => []
  | .not p => optCT p.seedTy
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
  attachWordsOk (Predicate.attachWordsInAll fs) (soleAlt (Predicate.headTysJoin fs))

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
    if p.seedsAbility then ⟨det, .object, plur, .ability none⟩
    else ⟨det, .object, plur,
      .object p.seedTy (some (zoneOr .battlefield p.seedZone)) none
        (if p.seedsToken then some .token else none) none⟩
  | .player, _ => ⟨det, .player, plur, .player false⟩
  | .quality q, _ => ⟨det, .quality q, plur, .quality q⟩
  | k@(.join _ _), p => ⟨det, k, plur, joinHalfPayload k p.seedTys⟩
  | k, _ => ⟨det, k, plur, .gap⟩

def chosenBind (det : Determiner) (plur : Plurality) : Kind → Predicate → Binding
  | .player, _ => ⟨det, .player, plur, .player true⟩
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

def Amount.exact : Amount → Option Nat
  | .lit n => some n
  | _ => none

def Amount.nonZero : Amount → Bool
  | .lit 0 => false
  | _ => true

def Amount.plur : Amount → Plurality
  | .lit 1 => .one
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

def NounPhrase.anchorPhrase : NounPhrase → Bool
  | .eitherOf l r => l.anchorPhrase && r.anchorPhrase
  | .both _ _ => false
  | n => n.det.elim true (· == .target)

def NounPhrase.groupMention : NounPhrase → Bool
  | .librarySlice _ _ _ => true
  | .pro _ pl .whole => !pl.isOne
  | .both _ _ => true
  | .oneEachOf _ _ => true
  | n => n.det == some .target

/-- A positional own-read partitions the group it has just named, a one-card group included
[CR#701.22a]. -/
def NounPhrase.partitiveBase : NounPhrase → Bool
  | .pro _ _ (.top _) => true
  | n => n.det == some .all || n.groupMention

def NounPhrase.countableGroup (n : NounPhrase) : Bool := n.det == some .bare || n.groupMention

def NounPhrase.countedMention : NounPhrase → Bool
  | .namesAgree _ _ => false
  | n => n.det == some .count || n.det == some .target

def NounPhrase.ascribable : NounPhrase → Bool
  | .this => true
  | _ => false

def NounPhrase.isYou : NounPhrase → Bool
  | .you => true
  | _ => false

def NounPhrase.targeted : NounPhrase → Bool
  | .asType _ n _ => n.targeted
  | .resolvedPermanent n => n.targeted
  | .asMarker _ n => n.targeted
  | .eachOf g => g.targeted
  | .someOf _ _ g => g.targeted
  | .both l r => l.targeted || r.targeted
  | .eitherOf l r => l.targeted || r.targeted
  | n => n.det == some .target

def NounPhrase.costNounOk : NounPhrase → Bool
  | .asType _ n _ => n.costNounOk
  | .resolvedPermanent n => n.costNounOk
  | .asMarker _ n => n.costNounOk
  | .eachOf g => g.costNounOk
  | .namesAgree _ g => g.costNounOk
  | .someOf _ _ g => g.costNounOk
  | .eitherOf l r => l.costNounOk && r.costNounOk
  | .both _ _ => false
  | .pro (.verbed _ _ _) _ _ => false
  | _ => true

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
  | .pro (.word .ability) _ _ => true
  | .pro (.word .abilityCopy) _ _ => true
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

def NounPhrase.plur : NounPhrase → Plurality
  | .this | .theGrantor _ | .combatPlayer _ | .you | .attachHost _ _ | .designated _ _ => .one
  | .asType _ n _ => n.plur
  | .resolvedPermanent n => n.plur
  | .asMarker _ n => n.plur
  | .playerGroup _ => .many
  | .described d _ => d.plur
  | .eachOf _ => .many
  | .namesAgree _ g => g.plur
  | .both _ _ => .many
  | .eitherOf l r => if l.plur == r.plur then l.plur else .many
  | .librarySlice _ amt whose => outputPlur whose.plur amt.plur
  | .someOf q _ _ => q.plur
  | .theRest _ pl => pl
  | .pileOf q _ => q.plur
  | .pro _ pl _ => pl
  | .possessorOf _ n => n.plur
  | .oneEachOf _ _ => .many

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
def sliceTyOf : Option Predicate → Option CardType → Option CardType
  | none, gty => gty
  | some p, gty =>
    match p.seedTy with
    | some t => some t
    | none => gty

mutual
  def NounPhrase.delta (bs : Bindings) : NounPhrase → List Binding
    | .this | .theGrantor _ | .combatPlayer _ | .you | .playerGroup _
    | .theRest _ _ | .pro _ _ _ | .attachHost _ _ | .designated _ _ => []
    | .asType _ n _ => NounPhrase.delta bs n
    | .resolvedPermanent n => NounPhrase.delta bs n
    | .asMarker _ n => NounPhrase.delta bs n
    | .described d p => DetPhrase.delta bs d (p.kindOr .object) p (Predicate.delta bs p)
    | .eachOf g => NounPhrase.delta bs g
    | .both l r => NounPhrase.delta (NounPhrase.delta bs l ++ bs) r ++ NounPhrase.delta bs l
    | .eitherOf l r => NounPhrase.delta bs l ++ NounPhrase.delta bs r
    | .librarySlice _ amt whose =>
      ⟨.the, .object, outputPlur whose.plur amt.plur,
        .object none (some .library) none none amt.exact⟩ :: NounPhrase.delta bs whose
    | .namesAgree _ g => NounPhrase.delta bs g
    | .someOf q d g =>
      ⟨.part, .object, q.plur, .object (sliceTyOf d (NounPhrase.ty bs g)) (NounPhrase.zone bs g) none none q.exact⟩
        :: (SliceCount.delta bs q ++ OptPredicate.delta bs d ++ NounPhrase.delta bs g)
    | .pileOf q none =>
      ⟨.part, .pile, q.plur,
        .pile (zoneOfReach (.word .pile) .many bs) q.exact (faceOfReach (.word .pile) .many bs)⟩
        :: SliceCount.delta bs q
    | .pileOf q (some by_) =>
      ⟨.part, .pile, q.plur,
        .pile (zoneOfReach (.word .pile) .many bs) q.exact (faceOfReach (.word .pile) .many bs)⟩
        :: (SliceCount.delta bs q ++ NounPhrase.delta bs by_)
    | .possessorOf _ n => ⟨.the, .player, n.plur, .player false⟩ :: (NounPhrase.selfSubjDelta n ++ NounPhrase.delta bs n)
    | .oneEachOf roles pool =>
      ⟨.bare, .object, .many, .object (NounPhrase.ty bs pool) (NounPhrase.zone bs pool) none none none⟩
        :: (Predicate.deltaAll bs roles ++ NounPhrase.delta bs pool)
  termination_by structural x => x

  def OptPredicate.delta (bs : Bindings) : Option Predicate → List Binding
    | none => []
    | some p => Predicate.delta bs p
  termination_by structural x => x

  def Predicate.delta (bs : Bindings) : Predicate → List Binding
    | .abilityOf n => NounPhrase.delta bs n
    | .activatedBy n => NounPhrase.delta bs n
    | .targets m _ => NounPhrase.delta bs m
    | .hasPossessor _ n => NounPhrase.delta bs n
    | .castBy n _ => NounPhrase.delta bs n
    | .inCombat _ none => []
    | .inCombat _ (some m) => NounPhrase.delta bs m
    | .counterKindOn n => NounPhrase.delta bs n
    | .happenedTo (.mk _ _ what) => OptComplement.delta bs what
    | .castFrom z => ZoneExpr.delta bs z
    | .named src => NameSource.delta bs src
    | .hasDesignation _ none => []
    | .hasDesignation _ (some h) => NounPhrase.delta bs h
    | .attachedBy _ by_ => NounPhrase.delta bs by_
    | .attachedTo host => NounPhrase.delta bs host
    | .inPile p => NounPhrase.delta bs p
    | .inZone z => ZoneExpr.delta bs z
    | .and ps => Predicate.deltaAll bs ps
    | .not _ => []
    | .or ps => if Predicate.joins ps then Predicate.deltaAll bs ps else []
    | .otherThan n => NounPhrase.delta bs n
    | .compare _ _ b => Amount.delta bs b
    | .superlative _ _ d => Predicate.delta bs d
    | .withMostVotes => []
    | .choseExtreme _ => [outcomeB .namedNumber]
    | .compareOver dom _ _ bound => gapB :: (Predicate.delta bs dom ++ Amount.delta bs bound)
    | _ => []
  termination_by structural x => x

  def Predicate.deltaAll (bs : Bindings) : List Predicate → List Binding
    | [] => []
    | p :: ps => Predicate.delta bs p ++ Predicate.deltaAll bs ps
  termination_by structural x => x

  def DetPhrase.delta (bs : Bindings) (d : DetPhrase) (k : Kind) (p : Predicate)
      (pd : List Binding) : List Binding :=
    match d with
    | .target q => sized q.exact (bindFor .target q.plur k p) :: (Quantity.delta bs q ++ pd)
    | .count q _ => sized q.exact (bindFor .count q.plur k p) :: (Quantity.delta bs q ++ pd)
    | .the => if p.choiceRead then pd else bindFor .the .one k p :: pd
    | d => bindFor d.det d.plur k p :: pd

  def LibraryPlace.delta (bs : Bindings) : LibraryPlace → List Binding
    | .oneEnd _ => []
    | .eitherEnd none => []
    | .eitherEnd (some n) => NounPhrase.delta bs n
    | .shuffled => []


  def ZoneExpr.delta (bs : Bindings) : ZoneExpr → List Binding
    | .zone _ (.possessedBy n) => NounPhrase.delta bs n
    | .zone _ .bare => []
    | .library pl _ _ (.possessedBy n) => LibraryPlace.delta bs pl ++ NounPhrase.delta bs n
    | .library pl _ _ .bare => LibraryPlace.delta bs pl
  termination_by structural x => x

  def ZoneExpr.deltaAll (bs : Bindings) : List ZoneExpr → List Binding
    | [] => []
    | z :: zs => ZoneExpr.delta bs z ++ ZoneExpr.deltaAll bs zs
  termination_by structural x => x

  def NameSource.delta (bs : Bindings) : NameSource → List Binding
    | .printed _ => []
    | .chosen => []
    | .sameAs n => NounPhrase.delta bs n


  def EventSource.delta (bs : Bindings) : EventSource → List Binding
    | .anywhere => []
    | .zones zs => ZoneExpr.deltaAll bs zs
    | .anywhereBut zs => ZoneExpr.deltaAll bs zs
  termination_by structural x => x

  def OptComplement.delta (bs : Bindings) : Option EventComplement → List Binding
    | none => []
    | some (.involving what) => NounPhrase.delta bs what
    | some (.fromZones src what) => EventSource.delta bs src ++ OptComplement.delta bs what
    | some (.intoZone to what) => ZoneExpr.delta bs to ++ OptComplement.delta bs what
    | some (.atZone z) => ZoneExpr.delta bs z
  termination_by structural x => x

  def Amount.delta (bs : Bindings) : Amount → List Binding
    | .lit _ => []
    | .statOf _ nom => NounPhrase.delta bs nom
    | .paid _ n => NounPhrase.delta bs n
    | .eventTally _ who (.mk _ _ what) =>
      NounPhrase.delta bs who ++ OptComplement.delta (NounPhrase.delta bs who ++ bs) what
    | .countOf g => NounPhrase.delta bs g
    | .aggregate _ _ g => NounPhrase.delta bs g
    | .thatMuch | .chosenNumber _ | .votesFor _ | .theOutcome _ | .coinsShowing _
    | .greatestStoredMatch _ | .groupSize | .theDifference => []
    | .letter l => letterDelta l bs
    | .arith _ a b => Amount.delta bs a ++ Amount.delta (Amount.intro bs a) b
    | .devotion who _ _ => NounPhrase.delta bs who
    | .half _ a => Amount.delta bs a
    | .aggregateOver _ dom _ => Predicate.delta bs dom
    | .distinctCount _ dom => NounPhrase.delta bs dom
    | .upTo b => outcomeB .ceilingShortfall :: Amount.delta bs b
  termination_by structural x => x

  /-- The stack after an amount, Idris `amtIntro`: not always `delta ++ bs`, because a nested
  amount is read at its outer amount's own intro. -/
  def Amount.intro (bs : Bindings) : Amount → Bindings
    | .lit _ => bs
    | .statOf _ nom => NounPhrase.delta bs nom ++ bs
    | .paid _ n => NounPhrase.delta bs n ++ bs
    | .eventTally _ who (.mk _ _ what) =>
      OptComplement.delta (NounPhrase.delta bs who ++ bs) what ++ (NounPhrase.delta bs who ++ bs)
    | .countOf g => NounPhrase.delta bs g ++ bs
    | .aggregate _ _ g => NounPhrase.delta bs g ++ bs
    | .thatMuch | .chosenNumber _ | .votesFor _ | .theOutcome _ | .coinsShowing _
    | .greatestStoredMatch _ | .groupSize | .theDifference => bs
    | .letter l => letterDelta l bs ++ bs
    | .arith _ a b => Amount.intro (Amount.intro bs a) b
    | .devotion who _ _ => NounPhrase.delta bs who ++ bs
    | .half _ a => Amount.intro bs a
    | .aggregateOver _ dom _ => Predicate.delta bs dom ++ bs
    | .distinctCount _ dom => NounPhrase.delta bs dom ++ bs
    | .upTo b => outcomeB .ceilingShortfall :: Amount.intro bs b
  termination_by structural x => x

  def Quantity.delta (bs : Bindings) : Quantity → List Binding
    | .range _ _ => []
    | .upToOf a => Amount.delta bs a
    | .exactlyOf a => Amount.delta bs a


  def SliceCount.delta (bs : Bindings) : SliceCount → List Binding
    | .counted q => Quantity.delta bs q
    | .whole => []


  def NounPhrase.selfSubjDelta : NounPhrase → List Binding
    | .asType t .this _ =>
      [⟨.self, .object, .one, .object (some t) (some .battlefield) none none none⟩]
    | .attachHost _ (.type t) =>
      [⟨.the, .object, .one, .object (some t) (some .battlefield) none none none⟩]
    | .attachHost _ .permanent =>
      [⟨.the, .object, .one, .object none (some .battlefield) none none none⟩]
    | .attachHost _ .player => [⟨.the, .player, .one, .player false⟩]
    | _ => []


  def NounPhrase.zone (bs : Bindings) : NounPhrase → Option Zone
    | .this | .combatPlayer _ | .you | .playerGroup _ | .eitherOf _ _
    | .possessorOf _ _ | .designated _ _ => none
    | .asType _ _ _ => some .battlefield
    | .resolvedPermanent _ => some .battlefield
    | .asMarker m _ => some m.zone
    | .theGrantor m => some m.zone
    | .described _ p => p.phraseZone (p.kindOr .object)
    | .eachOf g => NounPhrase.zone bs g
    | .namesAgree _ g => NounPhrase.zone bs g
    | .both l r => if NounPhrase.zone bs l == NounPhrase.zone bs r then NounPhrase.zone bs l else none
    | .librarySlice _ _ _ => some .library
    | .someOf _ _ g => NounPhrase.zone bs g
    | .theRest k _ => zoneOfGroup k bs
    | .pileOf _ _ => zoneOfReach (.word .pile) .many bs
    | .pro r pl w => zoneOfReach r pl (view w bs)
    | .attachHost _ h => h.attachHostZone
    | .oneEachOf _ pool => NounPhrase.zone bs pool
  termination_by structural x => x

  def NounPhrase.ty (bs : Bindings) : NounPhrase → Option CardType
    | .this | .theGrantor _ | .combatPlayer _ | .you | .playerGroup _
    | .eitherOf _ _ | .librarySlice _ _ _ | .pileOf _ _ | .possessorOf _ _ | .designated _ _ => none
    | .asType t _ _ => some t
    | .resolvedPermanent n => NounPhrase.ty bs n
    | .asMarker _ n => NounPhrase.ty bs n
    | .described _ p => p.seedTy
    | .eachOf g => NounPhrase.ty bs g
    | .namesAgree _ g => NounPhrase.ty bs g
    | .both l r => if NounPhrase.ty bs l == NounPhrase.ty bs r then NounPhrase.ty bs l else none
    | .someOf _ d g => sliceTyOf d (NounPhrase.ty bs g)
    | .theRest k _ => tyOfGroup k bs
    | .pro r pl w => tyOfReach r pl (view w bs)
    | .attachHost _ h => h.attachHostTy
    | .oneEachOf _ pool => NounPhrase.ty bs pool
  termination_by structural x => x

end

def nomIntro (bs : Bindings) (n : NounPhrase) : Bindings := NounPhrase.delta bs n ++ bs

def sliceTy (bs : Bindings) (d : Option Predicate) (g : NounPhrase) : Option CardType :=
  sliceTyOf d (NounPhrase.ty bs g)

def NounPhrase.headTys (bs : Bindings) : NounPhrase → List (List CardType)
  | .described _ p => p.headTyAlts
  | .eachOf g => NounPhrase.headTys bs g
  | .namesAgree _ g => NounPhrase.headTys bs g
  | .resolvedPermanent n => NounPhrase.headTys bs n
  | .asMarker _ n => NounPhrase.headTys bs n
  | .both l r => NounPhrase.headTys bs l ++ NounPhrase.headTys bs r
  | .eitherOf l r => NounPhrase.headTys bs l ++ NounPhrase.headTys bs r
  | .oneEachOf _ pool => NounPhrase.headTys bs pool
  | n => soleAlt (optCT (NounPhrase.ty bs n))

def NounPhrase.tys (bs : Bindings) : NounPhrase → HeadTy
  | .described _ p => p.seedTys
  | .eachOf g => NounPhrase.tys bs g
  | .namesAgree _ g => NounPhrase.tys bs g
  | .someOf _ d g => .sole (sliceTyOf d (NounPhrase.ty bs g))
  | n@(.both l r) =>
    if l.kindOr .object == r.kindOr .object then .sole (NounPhrase.ty bs n)
    else .join (NounPhrase.tys bs l) (NounPhrase.tys (nomIntro bs l) r)
  | n@(.eitherOf l r) =>
    if l.kindOr .object == r.kindOr .object then .sole (NounPhrase.ty bs n)
    else .join (NounPhrase.tys bs l) (NounPhrase.tys bs r)
  | n => .sole (NounPhrase.ty bs n)

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
  | n => (NounPhrase.delta bs n).isEmpty

def NounPhrase.bindingless (bs : Bindings) (n : NounPhrase) : Bool := (NounPhrase.delta bs n).isEmpty

/-- Idris `EventAgent`: an optional agent that introduces nothing. -/
def eventAgentOk (bs : Bindings) : Option NounPhrase → Bool
  | none => true
  | some n => NounPhrase.bindingless bs n

def attackableKind : Kind → HeadTy → Bool
  | .player, _ => true
  | .object, .sole t => featureAltOk .attacking .patient (optCT t)
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
  | .object, .sole t => damageableHeadTysOk (soleAlt (optCT t))
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

/-- Idris `Movable`: an object that is not an ability [CR#113.1c,608.2n], or a pile. -/
def NounPhrase.movable (n : NounPhrase) : Bool :=
  match n.kindOr .object with
  | .object => !n.isAbility
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

def deedNounOk (bs : Bindings) (v : VerbLabel) (r : Role) (n : NounPhrase) : Bool :=
  match NounPhrase.headTys bs n with
  | [] => n.det.isNone && deedBareOk v r
  | ts => deedHeadTysOk v r ts

def featureNounOk (bs : Bindings) (f : DeedFeature) (r : Role) (n : NounPhrase) : Bool :=
  (featureLabel f).elim false fun v => deedNounOk bs v r n

/-- Who declares an attack: the active player [CR#508.1] or a creature they control
[CR#508.1a]. The deed table's agent role decides which kinds attack, the `attacking` feature
keeps the object reading's type gate, and an object attacker is a battlefield permanent:
`attackableKind`'s counterpart on the declaring side. -/
def NounPhrase.attackerOk (bs : Bindings) (n : NounPhrase) : Bool :=
  let k := n.kindOr .object
  featureKindOk .attacking .agent k && featureNounOk bs .attacking .agent n &&
    combatPartyKind k (NounPhrase.zone bs n)

/-! ## Agents, choices, and the stacks they leave -/

def NounPhrase.agentDelta (bs : Bindings) : NounPhrase → List Binding
  | .described .each p => bindFor .the .one (p.kindOr .object) p :: Predicate.delta bs p
  | n => NounPhrase.delta bs n

def agentIntro (bs : Bindings) (n : NounPhrase) : Bindings := NounPhrase.agentDelta bs n ++ bs

def agentCtx (bs : Bindings) : Option NounPhrase → Bindings
  | none => bs
  | some n => agentIntro bs n

def kindValueIntro (bs : Bindings) (q : QualitySort) : Option NounPhrase → Bindings
  | some dom => qualityB q :: nomIntro bs dom
  | none => qualityB q :: bs

def kindDomainOk : KindAxis → Option NounPhrase → Bool
  | _, some _ => true
  | ax, none => ax.closed

def NounPhrase.chosenDelta (bs : Bindings) : NounPhrase → List Binding
  | .described (.a _) p =>
    let k := p.kindOr .object
    chosenBind (chosenDet k .a) .one k p :: Predicate.delta bs p
  | .described (.count q _) p =>
    let k := p.kindOr .object
    chosenBind (chosenDet k .count) q.plur k p :: (Quantity.delta bs q ++ Predicate.delta bs p)
  | .namesAgree _ g => NounPhrase.chosenDelta bs g
  | n => NounPhrase.delta bs n

def chosenIntro (bs : Bindings) (n : NounPhrase) : Bindings := NounPhrase.chosenDelta bs n ++ bs

def chosenIntroBy (bs : Bindings) : Plurality → NounPhrase → NounPhrase → Bindings
  | .one, by_, n => NounPhrase.delta bs by_ ++ (NounPhrase.chosenDelta (agentIntro bs by_) n ++ bs)
  | .many, by_, n => pluralizeDelta (NounPhrase.chosenDelta (agentIntro bs by_) n) ++ nomIntro bs by_

def chooseIntro (bs : Bindings) : Option NounPhrase → NounPhrase → Bindings
  | none, n => chosenIntro bs n
  | some by_, n => chosenIntroBy bs by_.plur by_ n

def optAmtIntro (bs : Bindings) : Option Amount → Bindings
  | none => bs
  | some a => Amount.intro bs a

def Delta.intro (bs : Bindings) (d : Delta Amount) : Bindings := Amount.intro bs d.amount
def Delta.delta (bs : Bindings) (d : Delta Amount) : List Binding := Amount.delta bs d.amount

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

def SearchScope.delta (bs : Bindings) : SearchScope → List Binding
  | .oneZone z => ZoneExpr.delta bs z
  | .someZones none _ => []
  | .someZones (some whose) _ => NounPhrase.delta bs whose

def Exposed.intro (bs : Bindings) : Exposed → Bindings
  | .cards n => nomIntro bs n
  | .zone z => ZoneExpr.delta bs z ++ bs
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

def selfSubjIntro (bs : Bindings) (n : NounPhrase) : Bindings := NounPhrase.selfSubjDelta n ++ nomIntro bs n

def subjCtx (bs : Bindings) : Option NounPhrase → Bindings
  | none => bs
  | some n => selfSubjIntro bs n

def elemIntro (bs : Bindings) (k : Kind) (g : NounPhrase) : Bindings :=
  ⟨.the, k, .one, elemPayload k g.isAbility (NounPhrase.ty bs g) (NounPhrase.zone bs g) (NounPhrase.prov bs g)⟩
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
  | ⟨_, .gap, _, .gap⟩ :: bs => dropGaps bs
  | b :: bs => b :: dropGaps bs

mutual
  def Condition.delta (bs : Bindings) : Condition → List Binding
    | .exists_ _ => []
    | .happened who _ => NounPhrase.selfSubjDelta who
    | .gameIs _ => []
    | .noHolder _ => []
    | .matches n _ => NounPhrase.delta bs n ++ NounPhrase.selfSubjDelta n
    | .compareAmt subj _ bound => gapB :: (Amount.delta (Amount.intro bs subj) bound ++ Amount.delta bs subj)
    | .dealtThisWay _ => []
    | .choseThisWay who _ => NounPhrase.selfSubjDelta who
    | .preventedFromSource _ => []
    | .flipCalled _ _ => []
    | .flipFace _ => []
    | .voteLead _ _ => []
    | .anyResultIs _ _ => []
    | .rolledDoubles => []
    | .not (.compareAmt subj _ bound) =>
      Amount.delta (Amount.intro bs subj) bound ++ Amount.delta bs subj
    | .not c => dropGaps (Condition.delta bs c)
    | .and cs => Condition.deltaAll bs cs
    | .or _ => []

  def Condition.deltaAll (bs : Bindings) : List Condition → List Binding
    | [] => []
    | c :: cs => Condition.delta bs c ++ Condition.deltaAll bs cs
end

def Condition.remarkAt : Condition → Option ((Binding → Bool) × Option CardType)
  | .matches n p =>
    match n.remarkTest with
    | none => none
    | some q => some (q, p.seedTy)
  | _ => none

def Condition.remark (bs : Bindings) (c : Condition) : Bindings :=
  match c.remarkAt with
  | none => bs
  | some (q, t) => markFirst q t bs

def Condition.intro (bs : Bindings) (c : Condition) : Bindings := Condition.delta bs c ++ c.remark bs

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

def setZone (p : Option VerbLabel) (z : Option Zone) (b : Binding) : Binding :=
  match b.payload with
  | .object ty oldZn _ og sz =>
    { b with payload := .object ty z (mkStamp p oldZn (oldZn != z)) og sz }
  | .pile _ sz fc => { b with payload := .pile z sz fc }
  | _ => b

def setZoneHead (p : Option VerbLabel) (z : Option Zone) : Bindings → Bindings
  | [] => []
  | b :: bs => setZone p z b :: bs

def setZoneReach (r : Reach) (pl : Plurality) (p : Option VerbLabel) (z : Option Zone) :
    Bindings → Bindings
  | [] => []
  | b :: bs => if reaches r pl b then setZone p z b :: bs else b :: setZoneReach r pl p z bs

/-- The stack after `n` moves to `z` under verb `p`, Idris `moveIntro`. -/
def moveIntro (bs : Bindings) (p : Option VerbLabel) (n : NounPhrase) (z : Option Zone) : Bindings :=
  match n with
  | .described _ _ | .librarySlice _ _ _ | .someOf _ _ _ | .oneEachOf _ _ | .pileOf _ _ =>
    setZoneHead p z (nomIntro bs n)
  | .eachOf g => moveIntro bs p g z
  | .namesAgree _ g => moveIntro bs p g z
  | .both _ _ | .eitherOf _ _ => nomIntro bs n
  | .theRest k _ => groupSpent k bs
  | .pro r pl w => overWindow (setZoneReach r pl p z) w bs
  | .this => ⟨.self, .object, .one, .object none z (mkStamp p none z.isSome) none none⟩ :: bs
  | .attachHost _ (.type t) =>
    ⟨.the, .object, .one,
      .object (some t) z (mkStamp p (some .battlefield) (z != some .battlefield)) none none⟩ :: bs
  | .attachHost _ .permanent =>
    ⟨.the, .object, .one,
      .object none z (mkStamp p (some .battlefield) (z != some .battlefield)) none none⟩ :: bs
  | .attachHost _ .player => ⟨.the, .player, .one, .player false⟩ :: bs
  | .attachHost _ _ => bs
  | .asType t .this _ =>
    ⟨.self, .object, .one,
      .object (some t) z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .asType t _ _ =>
    ⟨.the, .object, .one,
      .object (some t) z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .resolvedPermanent m =>
    ⟨.the, .object, m.plur,
      .object (NounPhrase.ty bs m) z (mkStamp p (some .battlefield) (z != some .battlefield)) none none⟩
      :: bs
  | .asMarker _ .this =>
    ⟨.self, .object, .one, .object none z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .asMarker _ _ =>
    ⟨.the, .object, .one, .object none z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .theGrantor m =>
    ⟨.the, .object, .one,
      .object none z (mkStamp p m.grantorOrigin (z != some m.zone)) none none⟩ :: bs
  | .you | .combatPlayer _ | .playerGroup _ | .designated _ _ => bs
  | .possessorOf ax m => nomIntro bs (.possessorOf ax m)
termination_by structural n

def stampIntro (bs : Bindings) (p : Option VerbLabel) (n : NounPhrase) : Bindings :=
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
