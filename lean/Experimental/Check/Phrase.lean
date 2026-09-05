import Experimental.Phrase
import Experimental.Check.Events
import Experimental.Check.Keywords
import Experimental.Check.Refusal

/-!
# Experimental.Check.Phrase

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

namespace Mtg

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
    | .or ps => Predicate.kindOfAny ps
    | .not p => p.kind?
    | .joined l r => some (joinKinds (l.kind?.getD .object) (r.kind?.getD .object))
    | .hasPossessor _ _ | .combatRel _ _ | .happenedTo _ | .withMostVotes | .other | .notChosen
    | .otherThan _ | .coinCameUp _ | .targets _ _ => none
    | _ => some .object

  def Predicate.kindOfAny : List Predicate → Option Kind
    | [] => none
    | p :: ps =>
      match p.kind? with
      | some k => some k
      | none => Predicate.kindOfAny ps
end

def Predicate.kindOr (d : Kind) (p : Predicate) : Kind := p.kind?.getD d

def Noun.kind? : Noun → Option Kind
  | .you | .theDefendingPlayer | .theAttackingPlayer | .playerGroup _ | .possessorOf _ _ =>
    some .player
  | .described _ p => p.kind?
  | .eachOf g => g.kind?
  | .both l r => some (joinKinds (l.kind?.getD .object) (r.kind?.getD .object))
  | .eitherOf l r => some (joinKinds (l.kind?.getD .object) (r.kind?.getD .object))
  | .theRest k _ => some k
  | .pileOf _ _ => some .pile
  | .pro r _ _ => some r.kind
  | .attachHost _ h => some h.kind
  | _ => some .object

def Noun.kindOr (d : Kind) (n : Noun) : Kind := n.kind?.getD d

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

def exactlyOne : Nat → Bool
  | 1 => true
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

def LibPlace.arrangementOk : LibPlace → Option Arrangement → Bool
  | .oneEnd _, _ => true
  | .eitherEnd _, none => true
  | .eitherEnd _, some _ => false
  | .shuffled, none => true
  | .shuffled, some _ => false

def LibPlace.ordinalOk : LibPlace → Option Ordinal → Bool
  | .shuffled, some _ => false
  | _, _ => true

def LibPlace.shuffles : LibPlace → Bool
  | .shuffled => true
  | _ => false

def ZoneExpr.sort : ZoneExpr → Zone
  | .zoneAt z _ => z
  | .libraryAt _ _ _ _ => .library

def ZoneExpr.arrangement : ZoneExpr → Option Arrangement
  | .zoneAt _ _ => none
  | .libraryAt _ ord _ _ => ord

def ZoneExpr.ordinal : ZoneExpr → Option Ordinal
  | .zoneAt _ _ => none
  | .libraryAt _ _ off _ => off

def ZoneExpr.shuffles : ZoneExpr → Bool
  | .zoneAt _ _ => false
  | .libraryAt place _ _ _ => place.shuffles

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
    | .joined l r => Payload.joinSeed l.seedTy r.seedTy
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

def Predicate.seedTys : Predicate → HeadTy
  | .joined l r => .join l.seedTys r.seedTys
  | p => .sole p.seedTy

mutual
  def Predicate.headTys : Predicate → List CardType
    | .and ps => Predicate.headTysAll ps
    | .or ps => Predicate.headTysJoin ps
    | .joined l r => l.headTys ++ r.headTys
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
    | .joined l r => l.headTyAlts ++ r.headTyAlts
    | p => soleAlt (optCT p.seedTy)

  def Predicate.headTyAltsJoin : List Predicate → List (List CardType)
    | [] => []
    | p :: ps => p.headTyAlts ++ Predicate.headTyAltsJoin ps
end

mutual
  def Predicate.attachWordsIn : Predicate → List AttachWord
    | .isAttached w => [w]
    | .attachedBy w _ => [w]
    | .and ps => Predicate.attachWordsInAll ps
    | .joined l r => l.attachWordsIn ++ r.attachWordsIn
    | _ => []

  def Predicate.attachWordsInAll : List Predicate → List AttachWord
    | [] => []
    | p :: ps => p.attachWordsIn ++ Predicate.attachWordsInAll ps
end

mutual
  def Predicate.seedZone : Predicate → Option Zone
    | .inZone z => some z.sort
    | .attacking | .beingDeclaredAttacker | .blocking | .blocked => some .battlefield
    | .combatRel .attackedBy _ => none
    | .combatRel _ _ => some .battlefield
    | .hasDesignation d _ => d.seedZone
    | .isAttached _ | .attachedBy _ _ | .attachedTo _ | .isToken | .isTransformed | .hasStatus _ =>
      some .battlefield
    | .isSpell => some .stack
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
    | .attacking | .beingDeclaredAttacker | .blocking | .blocked => some .creature
    | .combatRel .attackedBy _ => none
    | .combatRel _ _ => some .creature
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
    | .counterKindOn _ | .abilityHead _ | .isSource | .permanent | .isCard | .isToken | .isSpell
    | .isEmblem | .isCopyOfACard | .inZone _ | .exiledWith _ | .joined _ _ => true
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
    | .combatRel .attackedBy _ => true
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

/-- A colorless object has no color [CR#105.2c]. -/
def colorClashOf : Predicate → Predicate → Bool
  | .isColorless, .colorIs _ => true
  | .colorIs _, .isColorless => true
  | _, _ => false

/-- A token is not a card [CR#111.6]; an emblem is neither [CR#114.5]. -/
def cardTokenClashOf : Predicate → Predicate → Bool
  | .isCard, .isToken | .isToken, .isCard | .isEmblem, .isCard | .isCard, .isEmblem
  | .isEmblem, .permanent | .permanent, .isEmblem | .isEmblem, .isToken | .isToken, .isEmblem
  | .isCard, .isCopyOfACard | .isCopyOfACard, .isCard => true
  | _, _ => false

def noClash (clash : Predicate → Predicate → Bool) : List Predicate → Bool
  | [] => true
  | p :: ps => !ps.any (clash p) && noClash clash ps

/-- Instant and sorcery cards can't be permanents [CR#110.4]. -/
def Predicate.isPermanentHead : Predicate → Bool
  | .permanent => true
  | _ => false

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

def Predicate.isComparison : Predicate → Bool
  | .compare _ _ _ | .superlative _ _ _ | .withMostVotes | .choseExtreme _ | .compareOver _ _ _ _ =>
    true
  | _ => false

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
    | .joined l r => l.negFree && r.negFree
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
  | .lit _ | .timesOf _ _ | .thatMuch | .chosenNumber _ | .groupSize | .plus _ _ | .minus _ _
  | .half _ _ | .differenceBetween _ _ | .upTo _ => false
  | .theOutcome s => s.comparable
  | _ => true

def boundEq : Amount → Amount → Bool
  | .lit a, .lit b => a == b
  | .letter a, .letter b => a == b
  | _, _ => false

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

def Noun.det : Noun → Option Determiner
  | .described d _ => some d.det
  | .eachOf _ => some .each
  | .librarySlice _ _ _ => some .the
  | .someOf _ _ _ => some .part
  | .namesAgree _ g => g.det
  | .theRest _ _ => some .the
  | .pileOf _ _ => some .part
  | .oneEachOf _ _ => some .bare
  | _ => none

def Noun.anchorPhrase : Noun → Bool
  | .eitherOf l r => l.anchorPhrase && r.anchorPhrase
  | .both _ _ => false
  | n => n.det.elim true (· == .target)

def Noun.groupMention : Noun → Bool
  | .librarySlice _ _ _ => true
  | .pro _ pl .whole => !pl.isOne
  | .both _ _ => true
  | .oneEachOf _ _ => true
  | n => n.det == some .target

/-- A positional own-read partitions the group it has just named, a one-card group included
[CR#701.22a]. -/
def Noun.partitiveBase : Noun → Bool
  | .pro _ _ (.top _) => true
  | n => n.det == some .all || n.groupMention

def Noun.countableGroup (n : Noun) : Bool := n.det == some .bare || n.groupMention

def Noun.countedMention : Noun → Bool
  | .namesAgree _ _ => false
  | n => n.det == some .count || n.det == some .target

def Noun.ascribable : Noun → Bool
  | .this => true
  | _ => false

/-- The reach a pronoun is written with, if the noun is one. -/
def Noun.proRef : Noun → Option (Reach × Plurality × Window)
  | .pro r pl w => some (r, pl, w)
  | _ => none

def Noun.eqRef : Noun → Noun → Bool
  | .this, .this => true
  | .theGrantor a, .theGrantor b => a == b
  | .theDefendingPlayer, .theDefendingPlayer => true
  | .theAttackingPlayer, .theAttackingPlayer => true
  | .you, .you => true
  | .playerGroup v, .playerGroup w => v == w
  | .pro r pl w, m =>
    match m.proRef with
    | some (r', pl', w') => r == r' && pl == pl' && w == w'
    | none => false
  | _, _ => false

def Noun.isYou : Noun → Bool
  | .you => true
  | _ => false

def Noun.targeted : Noun → Bool
  | .asType _ n _ => n.targeted
  | .resolvedPermanent n => n.targeted
  | .asMarker _ n => n.targeted
  | .eachOf g => g.targeted
  | .someOf _ _ g => g.targeted
  | .both l r => l.targeted || r.targeted
  | .eitherOf l r => l.targeted || r.targeted
  | n => n.det == some .target

def Noun.costNounOk : Noun → Bool
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

def Noun.selfDefinedOk : Noun → Bool
  | .this => true
  | .asType _ n _ => n.selfDefinedOk
  | _ => false

def Noun.choosable (n : Noun) : Bool :=
  n.det == some .a || n.det == some .target || n.det == some .count

def Noun.agentChoosable : Noun → Bool
  | .someOf _ _ _ => true
  | .pileOf _ _ => true
  | n => n.choosable

def Noun.countedExistential : Noun → Bool
  | .namesAgree _ g => g.countedMention
  | _ => false

def Noun.existentialMention (n : Noun) : Bool := n.det == some .bare || n.countedExistential

/-- Whether the noun names an ability on the stack [CR#113.1c], read off the description and
the reading word, never the antecedent stack. -/
def Noun.isAbility : Noun → Bool
  | .described _ p => p.seedsAbility
  | .pro (.word .ability) _ _ => true
  | .pro (.word .abilityCopy) _ _ => true
  | .eachOf g => g.isAbility
  | .namesAgree _ g => g.isAbility
  | .resolvedPermanent n => n.isAbility
  | .asMarker _ n => n.isAbility
  | .someOf _ _ g => g.isAbility
  | _ => false

def Noun.moveDestOk : Noun → Bool
  | .pro r _ .whole => !r.tracksObject
  | .pro _ _ _ => false
  | _ => true

def Noun.remarkTest : Noun → Option (Binding → Bool)
  | .pro (.word .ability) .one .whole => some (reaches (.word .ability) .one)
  | .pro r pl .whole => if r.tracksObject then some (reaches r pl) else none
  | _ => none

/-! ## Number -/

def Noun.plur : Noun → Plurality
  | .this | .theGrantor _ | .theDefendingPlayer | .theAttackingPlayer | .you | .attachHost _ _
  | .designated _ _ => .one
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

def Noun.soleHolderOk : Noun → Bool
  | .playerGroup _ => true
  | n => n.plur.isOne

def Noun.slicePossessorOk : Noun → Bool
  | .described .each _ => true
  | .eachOf _ => true
  | .playerGroup _ => true
  | n => n.plur.isOne

def Noun.perMemberOk (n : Noun) : Bool :=
  n.det == some .each || n.det == some .all || n.plur.isOne

def Noun.agentPlur : Noun → Plurality
  | .described .each _ => .one
  | n => n.plur

/-- "Starting with you" fixes the turn order in which the players who choose make their
choices, so it says nothing unless several players choose [CR#101.4]. -/
def choiceOrderOk : Option Noun → Option Noun → Bool
  | none, _ => true
  | some _, none => false
  | some _, some by_ => !by_.plur.isOne

def choiceClauseOk : Option Noun → Noun → Bool
  | none, n => n.choosable
  | some _, n => n.agentChoosable

/-- A plural possessor must distribute over players [CR#102.1]. -/
def partPossessorOk : Option Noun → Bool
  | none => true
  | some n => n.plur.isOne || n.det == some .each

def windowOk : TurnPart → Option Noun → Bool
  | .turn, none => false
  | _, w => partPossessorOk w

def pointWindowOk (w : Option Noun) : Bool := partPossessorOk w

/-- A duration's possessor must be a single definite player. -/
def durationPossessorOk : Option Noun → Bool
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
  def Noun.delta (bs : Bindings) : Noun → List Binding
    | .this | .theGrantor _ | .theDefendingPlayer | .theAttackingPlayer | .you | .playerGroup _
    | .theRest _ _ | .pro _ _ _ | .attachHost _ _ | .designated _ _ => []
    | .asType _ n _ => Noun.delta bs n
    | .resolvedPermanent n => Noun.delta bs n
    | .asMarker _ n => Noun.delta bs n
    | .described d p => DetPhrase.delta bs d (p.kindOr .object) p (Predicate.delta bs p)
    | .eachOf g => Noun.delta bs g
    | .both l r => Noun.delta (Noun.delta bs l ++ bs) r ++ Noun.delta bs l
    | .eitherOf l r => Noun.delta bs l ++ Noun.delta bs r
    | .librarySlice _ amt whose =>
      ⟨.the, .object, outputPlur whose.plur amt.plur,
        .object none (some .library) none none amt.exact⟩ :: Noun.delta bs whose
    | .namesAgree _ g => Noun.delta bs g
    | .someOf q d g =>
      ⟨.part, .object, q.plur, .object (sliceTyOf d (Noun.ty bs g)) (Noun.zone bs g) none none q.exact⟩
        :: (SliceCount.delta bs q ++ OptPredicate.delta bs d ++ Noun.delta bs g)
    | .pileOf q none =>
      ⟨.part, .pile, q.plur,
        .pile (zoneOfReach (.word .pile) .many bs) q.exact (faceOfReach (.word .pile) .many bs)⟩
        :: SliceCount.delta bs q
    | .pileOf q (some by_) =>
      ⟨.part, .pile, q.plur,
        .pile (zoneOfReach (.word .pile) .many bs) q.exact (faceOfReach (.word .pile) .many bs)⟩
        :: (SliceCount.delta bs q ++ Noun.delta bs by_)
    | .possessorOf _ n => ⟨.the, .player, n.plur, .player false⟩ :: (Noun.selfSubjDelta n ++ Noun.delta bs n)
    | .oneEachOf roles pool =>
      ⟨.bare, .object, .many, .object (Noun.ty bs pool) (Noun.zone bs pool) none none none⟩
        :: (Predicate.deltaAll bs roles ++ Noun.delta bs pool)
  termination_by structural x => x

  def OptPredicate.delta (bs : Bindings) : Option Predicate → List Binding
    | none => []
    | some p => Predicate.delta bs p
  termination_by structural x => x

  def Predicate.delta (bs : Bindings) : Predicate → List Binding
    | .abilityOf n => Noun.delta bs n
    | .activatedBy n => Noun.delta bs n
    | .targets m _ => Noun.delta bs m
    | .hasPossessor _ n => Noun.delta bs n
    | .castBy n _ => Noun.delta bs n
    | .combatRel _ m => Noun.delta bs m
    | .counterKindOn n => Noun.delta bs n
    | .happenedTo (.mk _ _ what) => OptComplement.delta bs what
    | .castFrom z => ZoneExpr.delta bs z
    | .named src => NameSource.delta bs src
    | .hasDesignation _ none => []
    | .hasDesignation _ (some h) => Noun.delta bs h
    | .attachedBy _ by_ => Noun.delta bs by_
    | .attachedTo host => Noun.delta bs host
    | .inPile p => Noun.delta bs p
    | .inZone z => ZoneExpr.delta bs z
    | .and ps => Predicate.deltaAll bs ps
    | .not _ => []
    | .or _ => []
    | .otherThan n => Noun.delta bs n
    | .compare _ _ b => Amount.delta bs b
    | .superlative _ _ d => Predicate.delta bs d
    | .withMostVotes => []
    | .choseExtreme _ => [outcomeB .namedNumber]
    | .compareOver dom _ _ bound => gapB :: (Predicate.delta bs dom ++ Amount.delta bs bound)
    | .joined l r => Predicate.delta bs l ++ Predicate.delta bs r
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

  def LibPlace.delta (bs : Bindings) : LibPlace → List Binding
    | .oneEnd _ => []
    | .eitherEnd none => []
    | .eitherEnd (some n) => Noun.delta bs n
    | .shuffled => []


  def ZoneExpr.delta (bs : Bindings) : ZoneExpr → List Binding
    | .zoneAt _ (.possessedBy n) => Noun.delta bs n
    | .zoneAt _ .bare => []
    | .libraryAt pl _ _ (.possessedBy n) => LibPlace.delta bs pl ++ Noun.delta bs n
    | .libraryAt pl _ _ .bare => LibPlace.delta bs pl
  termination_by structural x => x

  def ZoneExpr.deltaAll (bs : Bindings) : List ZoneExpr → List Binding
    | [] => []
    | z :: zs => ZoneExpr.delta bs z ++ ZoneExpr.deltaAll bs zs
  termination_by structural x => x

  def NameSource.delta (bs : Bindings) : NameSource → List Binding
    | .printed _ => []
    | .chosen => []
    | .sameAs n => Noun.delta bs n


  def EventSource.delta (bs : Bindings) : EventSource → List Binding
    | .anywhere => []
    | .zones zs => ZoneExpr.deltaAll bs zs
    | .anywhereBut zs => ZoneExpr.deltaAll bs zs
  termination_by structural x => x

  def OptComplement.delta (bs : Bindings) : Option EventComplement → List Binding
    | none => []
    | some (.involving what) => Noun.delta bs what
    | some (.fromZones src what) => EventSource.delta bs src ++ OptComplement.delta bs what
    | some (.intoZone to what) => ZoneExpr.delta bs to ++ OptComplement.delta bs what
    | some (.atZone z) => ZoneExpr.delta bs z
  termination_by structural x => x

  def Amount.delta (bs : Bindings) : Amount → List Binding
    | .lit _ => []
    | .statOf _ nom => Noun.delta bs nom
    | .playerStatOf _ nom => Noun.delta bs nom
    | .countersOn _ holder => Noun.delta bs holder
    | .paid _ n => Noun.delta bs n
    | .eventTally _ who (.mk _ _ what) =>
      Noun.delta bs who ++ OptComplement.delta (Noun.delta bs who ++ bs) what
    | .countOf g => Noun.delta bs g
    | .aggregate _ _ g => Noun.delta bs g
    | .timesOf per a => Amount.delta bs per ++ Amount.delta (Amount.intro bs per) a
    | .thatMuch | .chosenNumber _ | .votesFor _ | .theOutcome _ | .coinsShowing _
    | .greatestStoredMatch _ | .groupSize | .theDifference => []
    | .letter l => letterDelta l bs
    | .plus a b => Amount.delta bs a ++ Amount.delta (Amount.intro bs a) b
    | .minus a b => Amount.delta bs a ++ Amount.delta (Amount.intro bs a) b
    | .devotion who _ _ => Noun.delta bs who
    | .half _ a => Amount.delta bs a
    | .differenceBetween a b => Amount.delta bs a ++ Amount.delta (Amount.intro bs a) b
    | .aggregateOver _ dom _ => Predicate.delta bs dom
    | .distinctCount _ dom => Noun.delta bs dom
    | .upTo b => outcomeB .ceilingShortfall :: Amount.delta bs b
  termination_by structural x => x

  /-- The stack after an amount, Idris `amtIntro`: not always `delta ++ bs`, because a nested
  amount is read at its outer amount's own intro. -/
  def Amount.intro (bs : Bindings) : Amount → Bindings
    | .lit _ => bs
    | .statOf _ nom => Noun.delta bs nom ++ bs
    | .playerStatOf _ nom => Noun.delta bs nom ++ bs
    | .countersOn _ holder => Noun.delta bs holder ++ bs
    | .paid _ n => Noun.delta bs n ++ bs
    | .eventTally _ who (.mk _ _ what) =>
      OptComplement.delta (Noun.delta bs who ++ bs) what ++ (Noun.delta bs who ++ bs)
    | .countOf g => Noun.delta bs g ++ bs
    | .aggregate _ _ g => Noun.delta bs g ++ bs
    | .timesOf per a => Amount.intro (Amount.intro bs per) a
    | .thatMuch | .chosenNumber _ | .votesFor _ | .theOutcome _ | .coinsShowing _
    | .greatestStoredMatch _ | .groupSize | .theDifference => bs
    | .letter l => letterDelta l bs ++ bs
    | .plus a b => Amount.intro (Amount.intro bs a) b
    | .minus a b => Amount.intro (Amount.intro bs a) b
    | .devotion who _ _ => Noun.delta bs who ++ bs
    | .half _ a => Amount.intro bs a
    | .differenceBetween a b => Amount.intro (Amount.intro bs a) b
    | .aggregateOver _ dom _ => Predicate.delta bs dom ++ bs
    | .distinctCount _ dom => Noun.delta bs dom ++ bs
    | .upTo b => outcomeB .ceilingShortfall :: Amount.intro bs b
  termination_by structural x => x

  def Quantity.delta (bs : Bindings) : Quantity → List Binding
    | .range _ _ => []
    | .upToOf a => Amount.delta bs a
    | .exactlyOf a => Amount.delta bs a


  def SliceCount.delta (bs : Bindings) : SliceCount → List Binding
    | .counted q => Quantity.delta bs q
    | .whole => []


  def Noun.selfSubjDelta : Noun → List Binding
    | .asType t .this _ =>
      [⟨.self, .object, .one, .object (some t) (some .battlefield) none none none⟩]
    | .attachHost _ (.type t) =>
      [⟨.the, .object, .one, .object (some t) (some .battlefield) none none none⟩]
    | .attachHost _ .permanent =>
      [⟨.the, .object, .one, .object none (some .battlefield) none none none⟩]
    | .attachHost _ .player => [⟨.the, .player, .one, .player false⟩]
    | _ => []


  def Noun.zone (bs : Bindings) : Noun → Option Zone
    | .this | .theDefendingPlayer | .theAttackingPlayer | .you | .playerGroup _ | .eitherOf _ _
    | .possessorOf _ _ | .designated _ _ => none
    | .asType _ _ _ => some .battlefield
    | .resolvedPermanent _ => some .battlefield
    | .asMarker m _ => some m.zone
    | .theGrantor m => some m.zone
    | .described _ p => p.phraseZone (p.kindOr .object)
    | .eachOf g => Noun.zone bs g
    | .namesAgree _ g => Noun.zone bs g
    | .both l r => if Noun.zone bs l == Noun.zone bs r then Noun.zone bs l else none
    | .librarySlice _ _ _ => some .library
    | .someOf _ _ g => Noun.zone bs g
    | .theRest k _ => zoneOfGroup k bs
    | .pileOf _ _ => zoneOfReach (.word .pile) .many bs
    | .pro r pl w => zoneOfReach r pl (view w bs)
    | .attachHost _ h => h.attachHostZone
    | .oneEachOf _ pool => Noun.zone bs pool
  termination_by structural x => x

  def Noun.ty (bs : Bindings) : Noun → Option CardType
    | .this | .theGrantor _ | .theDefendingPlayer | .theAttackingPlayer | .you | .playerGroup _
    | .eitherOf _ _ | .librarySlice _ _ _ | .pileOf _ _ | .possessorOf _ _ | .designated _ _ => none
    | .asType t _ _ => some t
    | .resolvedPermanent n => Noun.ty bs n
    | .asMarker _ n => Noun.ty bs n
    | .described _ p => p.seedTy
    | .eachOf g => Noun.ty bs g
    | .namesAgree _ g => Noun.ty bs g
    | .both l r => if Noun.ty bs l == Noun.ty bs r then Noun.ty bs l else none
    | .someOf _ d g => sliceTyOf d (Noun.ty bs g)
    | .theRest k _ => tyOfGroup k bs
    | .pro r pl w => tyOfReach r pl (view w bs)
    | .attachHost _ h => h.attachHostTy
    | .oneEachOf _ pool => Noun.ty bs pool
  termination_by structural x => x

end

def nomIntro (bs : Bindings) (n : Noun) : Bindings := Noun.delta bs n ++ bs

def sliceTy (bs : Bindings) (d : Option Predicate) (g : Noun) : Option CardType :=
  sliceTyOf d (Noun.ty bs g)

def Noun.headTys (bs : Bindings) : Noun → List (List CardType)
  | .described _ p => p.headTyAlts
  | .eachOf g => Noun.headTys bs g
  | .namesAgree _ g => Noun.headTys bs g
  | .resolvedPermanent n => Noun.headTys bs n
  | .asMarker _ n => Noun.headTys bs n
  | .both l r => Noun.headTys bs l ++ Noun.headTys bs r
  | .eitherOf l r => Noun.headTys bs l ++ Noun.headTys bs r
  | .oneEachOf _ pool => Noun.headTys bs pool
  | n => soleAlt (optCT (Noun.ty bs n))

def Noun.tys (bs : Bindings) : Noun → HeadTy
  | .described _ p => p.seedTys
  | .eachOf g => Noun.tys bs g
  | .namesAgree _ g => Noun.tys bs g
  | .someOf _ d g => .sole (sliceTyOf d (Noun.ty bs g))
  | n@(.both l r) =>
    if l.kindOr .object == r.kindOr .object then .sole (Noun.ty bs n)
    else .join (Noun.tys bs l) (Noun.tys (nomIntro bs l) r)
  | n@(.eitherOf l r) =>
    if l.kindOr .object == r.kindOr .object then .sole (Noun.ty bs n)
    else .join (Noun.tys bs l) (Noun.tys bs r)
  | n => .sole (Noun.ty bs n)

def Noun.prov (bs : Bindings) : Noun → Option Stamp
  | .theRest k _ => provOfGroup k bs
  | .pro r pl w => provOfReach r pl (view w bs)
  | .eachOf g => Noun.prov bs g
  | .namesAgree _ g => Noun.prov bs g
  | .someOf _ _ g => Noun.prov bs g
  | _ => none

def Noun.paidSubjectOk (bs : Bindings) : Noun → Bool
  | .this => true
  | .asType _ n _ => Noun.paidSubjectOk bs n
  | .asMarker _ n => Noun.paidSubjectOk bs n
  | n => onStackZone (Noun.zone bs n)

def Noun.costSubjectOk (bs : Bindings) : Noun → Bool
  | .this => true
  | n => onStackZone (Noun.zone bs n)

def Noun.counterMemoryOk (bs : Bindings) : Noun → Bool
  | .pro (.verbed _ _ _) _ _ => false
  | .pro r pl w => !r.tracksObject || !stampMoves (provOfReach r pl (view w bs))
  | _ => true

def Noun.testSubjectOk (bs : Bindings) : Noun → Bool
  | .described .the _ => true
  | .librarySlice _ _ _ => true
  | n => (Noun.delta bs n).isEmpty

def Noun.bindingless (bs : Bindings) (n : Noun) : Bool := (Noun.delta bs n).isEmpty

/-- Idris `EventAgent`: an optional agent that introduces nothing. -/
def eventAgentOk (bs : Bindings) : Option Noun → Bool
  | none => true
  | some n => Noun.bindingless bs n

def attackableKind : Kind → HeadTy → Bool
  | .player, _ => true
  | .object, .sole t => featureAltOk .attacking .patient (optCT t)
  | .join a b, .join l r => attackableKind a l && attackableKind b r
  | .join a b, .sole t => attackableKind a (.sole t) && attackableKind b (.sole t)
  | _, _ => false

def combatRelOk : CombatRelation → Kind → Kind → Option Zone → HeadTy → Bool
  | .attackerOf, k, km, _, tys => k == .object && attackableKind km tys
  | .attackedBy, _, km, z, _ => km == .object && zoneIsB z .battlefield
  | _, k, km, z, _ => k == .object && km == .object && zoneIsB z .battlefield

def damageableKind : Kind → HeadTy → Bool
  | .player, _ => true
  | .object, .sole t => damageableHeadTysOk (soleAlt (optCT t))
  | .join a b, .join l r => damageableKind a l && damageableKind b r
  | .join a b, .sole t => damageableKind a (.sole t) && damageableKind b (.sole t)
  | _, _ => false

/-- Idris `DamageRecipient n`: a player, a join with damageable halves, or an object on the
battlefield whose head types can be dealt damage. -/
def Noun.damageRecipient (bs : Bindings) (n : Noun) : Bool :=
  match n.kindOr .object with
  | .player => true
  | .join a b => damageableKind (.join a b) (Noun.tys bs n)
  | .object => zoneIsB (Noun.zone bs n) .battlefield && damageableHeadTysOk (Noun.headTys bs n)
  | _ => false

def Noun.attackable (bs : Bindings) (n : Noun) : Bool :=
  attackableKind (n.kindOr .object) (Noun.tys bs n)

def hostedRead (bs : Bindings) (p : Predicate) (n : Noun) : Bool :=
  match p.qualityReadHost with
  | none => true
  | some h => tyIs h (Noun.ty bs n)

/-- Idris `PileMention`: a pile phrase, "that pile", or "those piles". -/
def Noun.pileMention : Noun → Bool
  | .pileOf _ _ => true
  | .pro (.word .pile) _ .whole => true
  | _ => false

/-- Idris `LinkSource`: "exiled with this" names the source itself. -/
def Noun.linkSource : Noun → Bool
  | .this => true
  | .asType _ .this _ => true
  | _ => false

/-- Idris `StackActOn p n`: a spell on the stack, or a join whose kinds `p` admits. -/
def Noun.stackActOn (bs : Bindings) (p : Kind → Bool) (n : Noun) : Bool :=
  match n.kindOr .object with
  | .join a b => p (.join a b)
  | _ => zoneIsB (Noun.zone bs n) .stack

def Noun.counterable (bs : Bindings) (n : Noun) : Bool := n.stackActOn bs counterKind
def Noun.copiable (bs : Bindings) (n : Noun) : Bool := n.stackActOn bs copyKind

def copySourceOk (bs : Bindings) : CopySort → Noun → Bool
  | .fromStack, n => n.copiable bs
  | .fromCardZone, n => isCardZone (Noun.zone bs n)

/-- Idris `Movable`: an object that is not an ability [CR#113.1c,608.2n], or a pile. -/
def Noun.movable (n : Noun) : Bool :=
  match n.kindOr .object with
  | .object => !n.isAbility
  | .pile => true
  | _ => false

/-- Idris `DestOk`: a bare battlefield, exile, hand, or graveyard, or a bare library place. -/
def ZoneExpr.destOk : ZoneExpr → Bool
  | .zoneAt .battlefield .bare | .zoneAt .exile .bare | .zoneAt .hand .bare
  | .zoneAt .graveyard .bare => true
  | .libraryAt place arrg offs .bare => place.arrangementOk arrg && place.ordinalOk offs
  | _ => false

def orderOk (pl : Plurality) (z : ZoneExpr) : Bool :=
  match z.arrangement with
  | none => true
  | some _ => !pl.isOne

def Noun.discardOk (bs : Bindings) : Noun → Bool
  | .this => true
  | n => Noun.zone bs n == some .hand

def deedNounOk (bs : Bindings) (v : VerbLabel) (r : Role) (n : Noun) : Bool :=
  match Noun.headTys bs n with
  | [] => n.det.isNone && deedBareOk v r
  | ts => deedHeadTysOk v r ts

def featureNounOk (bs : Bindings) (f : DeedFeature) (r : Role) (n : Noun) : Bool :=
  (featureLabel f).elim false fun v => deedNounOk bs v r n

/-! ## Agents, choices, and the stacks they leave -/

def Noun.agentDelta (bs : Bindings) : Noun → List Binding
  | .described .each p => bindFor .the .one (p.kindOr .object) p :: Predicate.delta bs p
  | n => Noun.delta bs n

def agentIntro (bs : Bindings) (n : Noun) : Bindings := Noun.agentDelta bs n ++ bs

def agentCtx (bs : Bindings) : Option Noun → Bindings
  | none => bs
  | some n => agentIntro bs n

def kindValueIntro (bs : Bindings) (q : QualitySort) : Option Noun → Bindings
  | some dom => qualityB q :: nomIntro bs dom
  | none => qualityB q :: bs

def kindDomainOk : KindAxis → Option Noun → Bool
  | _, some _ => true
  | ax, none => ax.closed

def Noun.chosenDelta (bs : Bindings) : Noun → List Binding
  | .described (.a _) p =>
    let k := p.kindOr .object
    chosenBind (chosenDet k .a) .one k p :: Predicate.delta bs p
  | .described (.count q _) p =>
    let k := p.kindOr .object
    chosenBind (chosenDet k .count) q.plur k p :: (Quantity.delta bs q ++ Predicate.delta bs p)
  | .namesAgree _ g => Noun.chosenDelta bs g
  | n => Noun.delta bs n

def chosenIntro (bs : Bindings) (n : Noun) : Bindings := Noun.chosenDelta bs n ++ bs

def chosenIntroBy (bs : Bindings) : Plurality → Noun → Noun → Bindings
  | .one, by_, n => Noun.delta bs by_ ++ (Noun.chosenDelta (agentIntro bs by_) n ++ bs)
  | .many, by_, n => pluralizeDelta (Noun.chosenDelta (agentIntro bs by_) n) ++ nomIntro bs by_

def chosenAnnBy (bs : Bindings) : Plurality → Noun → Noun → Bindings
  | .one, by_, n => Noun.chosenDelta (agentIntro bs by_) n ++ bs
  | .many, by_, n => pluralizeDelta (Noun.chosenDelta (agentIntro bs by_) n) ++ bs

def chooseIntro (bs : Bindings) : Option Noun → Noun → Bindings
  | none, n => chosenIntro bs n
  | some by_, n => chosenIntroBy bs by_.plur by_ n

def chooseAnn (bs : Bindings) : Option Noun → Noun → Bindings
  | none, n => chosenIntro bs n
  | some by_, n => chosenAnnBy bs by_.plur by_ n

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
  | .someZones (some whose) _ => Noun.delta bs whose

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

def selfSubjIntro (bs : Bindings) (n : Noun) : Bindings := Noun.selfSubjDelta n ++ nomIntro bs n

def subjCtx (bs : Bindings) : Option Noun → Bindings
  | none => bs
  | some n => selfSubjIntro bs n

def elemIntro (bs : Bindings) (k : Kind) (g : Noun) : Bindings :=
  ⟨.the, k, .one, elemPayload k g.isAbility (Noun.ty bs g) (Noun.zone bs g) (Noun.prov bs g)⟩
    :: nomIntro bs g

def Condition.negated : Condition → Bool
  | .not _ => true
  | _ => false

def markingOk : CondMarking → Condition → Bool
  | .asLongAs, _ => true
  | .ifSo, _ => true
  | .unless, c => c.negated

def dropGaps : List Binding → List Binding
  | [] => []
  | ⟨_, .gap, _, .gap⟩ :: bs => dropGaps bs
  | b :: bs => b :: dropGaps bs

mutual
  def Condition.delta (bs : Bindings) : Condition → List Binding
    | .thereIs _ => []
    | .happened who _ => Noun.selfSubjDelta who
    | .gameIs _ => []
    | .noHolder _ => []
    | .matches n _ => Noun.delta bs n ++ Noun.selfSubjDelta n
    | .compareAmt subj _ bound => gapB :: (Amount.delta (Amount.intro bs subj) bound ++ Amount.delta bs subj)
    | .dealtThisWay _ => []
    | .choseThisWay who _ => Noun.selfSubjDelta who
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
def moveIntro (bs : Bindings) (p : Option VerbLabel) (n : Noun) (z : Option Zone) : Bindings :=
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
      .object (Noun.ty bs m) z (mkStamp p (some .battlefield) (z != some .battlefield)) none none⟩
      :: bs
  | .asMarker _ .this =>
    ⟨.self, .object, .one, .object none z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .asMarker _ _ =>
    ⟨.the, .object, .one, .object none z (mkStamp p none (z != some .battlefield)) none none⟩ :: bs
  | .theGrantor m =>
    ⟨.the, .object, .one,
      .object none z (mkStamp p m.grantorOrigin (z != some m.zone)) none none⟩ :: bs
  | .you | .theDefendingPlayer | .theAttackingPlayer | .playerGroup _ | .designated _ _ => bs
  | .possessorOf ax m => nomIntro bs (.possessorOf ax m)
termination_by structural n

def stampIntro (bs : Bindings) (p : Option VerbLabel) (n : Noun) : Bindings :=
  moveIntro bs p n (Noun.zone bs n)

end Mtg

namespace Mtg

/-! ## `other` anchoring -/

def complementAnchorsOk (bs : Bindings) (ts : List CardType) : List Predicate → Bool
  | [] => true
  | .otherThan n :: ps => complementAnchorsOk bs ts ps && anchorTyFits ts (Noun.ty bs n)
  | _ :: ps => complementAnchorsOk bs ts ps

def otherAnchorOk (bs : Bindings) (k : Kind) (ts : List CardType) (ps : List Predicate) : Bool :=
  let fs := Predicate.flatten ps
  if atMostOne (countOthers fs)
    then (if Predicate.hasBareOtherAny ps then anyTargeted k bs else complementAnchorsOk bs ts fs)
    else false

end Mtg
