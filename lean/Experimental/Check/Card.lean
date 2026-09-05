import Experimental.Card
import Experimental.Check.EffectRules

/-!
# Experimental.Check.Card

The card frame laws of `Card.idr` and the checker's entry point: `check : Card → List
Refusal`, and `Spelled`, a card the checker admits.
-/

namespace Mtg

def cardClassOf : List CardType → CardClass
  | [] => .permanentCard
  | t :: ts => if t.isSpell then .spellCard else cardClassOf ts

def anyPermanentType (tys : List CardType) : Bool := tys.any CardType.permanent
def anySpellType (tys : List CardType) : Bool := tys.any CardType.isSpell
def hasNonKindredType (tys : List CardType) : Bool := tys.any (· != .kindred)

def typesCombinable (tys : List CardType) : Bool :=
  !(anyPermanentType tys && anySpellType tys) && (!tys.elem .kindred || hasNonKindredType tys)

def keywordCardOk : CardClass → KeywordLabel → Bool
  | .permanentCard, k => (keywordFactsFor k).elim false (·.onPermanentCard)
  | .spellCard, k => (keywordFactsFor k).elim false (·.onSpellCard)

def StaticSpec.onSpellCardOk : StaticSpec → Bool
  | .deontic _ .forbid deeds .patient _ _ _ _ =>
    deeds.all fun deed => deedZoneOf deed .patient == some .stack
  | .altCost .this _ => true
  | .costs .this _ => true
  | .addedCost _ _ => true
  | .onlyDuring _ _ se => se.onSpellCardOk
  | .conditionally se _ _ => se.onSpellCardOk
  | _ => false

def classAbilityOk : CardClass → AbilityAt → Bool
  | .permanentCard, .keyword k _ _ => keywordCardOk .permanentCard k
  | .permanentCard, .spell _ _ => false
  | .permanentCard, .alsoForKeywords ab _ => classAbilityOk .permanentCard ab
  | .permanentCard, .italicHead _ ab => classAbilityOk .permanentCard ab
  | .permanentCard, _ => true
  | .spellCard, .keyword k _ _ => keywordCardOk .spellCard k
  | .spellCard, .activated c _ _ _ _ _ => c.offBattlefield
  | .spellCard, .triggered _ _ _ _ _ _ _ _ _ => true
  | .spellCard, .static se => se.onSpellCardOk
  | .spellCard, .alsoForKeywords ab _ => classAbilityOk .spellCard ab
  | .spellCard, .spell _ _ => true
  | .spellCard, .italicHead _ ab => classAbilityOk .spellCard ab
  | .spellCard, .mayBeginOnBattlefield => false

def cardTextOk (tys : List CardType) (text : AbilitySeq) : Bool :=
  text.all (classAbilityOk (cardClassOf tys))

def chapterLineOk (subs : List Subtype) : AbilityAt → Bool
  | .triggered _ (.chapterMark _) _ _ _ _ _ _ _ => subs.elem (.of .enchantment "Saga")
  | .italicHead _ ab => chapterLineOk subs ab
  | .alsoForKeywords ab _ => chapterLineOk subs ab
  | _ => true

def chapterFrameOk (subs : List Subtype) (text : AbilitySeq) : Bool := text.all (chapterLineOk subs)

def doorFrameOk (text : AbilitySeq) : Bool := text.all fun a => !a.namesThisDoor

def PrintedStat.starred : PrintedStat → Bool
  | .num _ => false
  | _ => true

def boxPt : Option PrintedBox → Option (PrintedStat × PrintedStat)
  | some (.pt p t) => some (p, t)
  | _ => none

def StaticSpec.definedSlots : StaticSpec → Option DefinedSlots
  | .definesPt _ sl _ => some sl
  | .conditionally se _ _ => se.definedSlots
  | _ => none

def AbilityAt.definesPt : AbilityAt → Option DefinedSlots
  | .static se => se.definedSlots
  | .italicHead _ ab => ab.definesPt
  | _ => none

def textDefines (f : DefinedSlots → Bool) (text : AbilitySeq) : Bool :=
  text.any fun a => (a.definesPt).elim false f

def definedSlotsStarred : Option (PrintedStat × PrintedStat) → Bool → Bool → Bool
  | none, dp, dt => !dp && !dt
  | some (p, t), dp, dt => (!dp || p.starred) && (!dt || t.starred)

def boxSuitsType : FaceSide → CardType → Option PrintedBox → Bool
  | _, .creature, some (.pt _ _) => true
  | _, .creature, _ => false
  | _, .planeswalker, some (.loyalty _) => true
  | .back, .planeswalker, none => true
  | _, .planeswalker, _ => false
  | _, .battle, some (.defense _) => true
  | _, .battle, _ => false
  | _, _, _ => true

def boxFitsLine (side : FaceSide) (tys : List CardType) (box : Option PrintedBox) : Bool :=
  tys.all fun t => boxSuitsType side t box

def cardBoxOk (side : FaceSide) (tys : List CardType) (text : AbilitySeq) (box : Option PrintedBox) :
    Bool :=
  boxFitsLine side tys box &&
    definedSlotsStarred (boxPt box) (textDefines DefinedSlots.power text)
      (textDefines DefinedSlots.toughness text)

def cardCostOk : FaceSide → List CardType → Option ManaCost → Bool
  | _, _, none => true
  | .back, _, some _ => false
  | .front, tys, some _ => !tys.elem .land

def TypeLine.cardLineOk (l : TypeLine) : Bool :=
  l.nonEmpty && typesDistinct l.types && typesCombinable l.types && subsFitLine l.subtypes l.types

def AbilityAt.keywordWantsModes : AbilityAt → Bool
  | .keyword k _ _ => (keywordFactsFor k).elim false (·.wantsModes)
  | .italicHead _ ab => ab.keywordWantsModes
  | .alsoForKeywords ab _ => ab.keywordWantsModes
  | _ => false

def AbilityAt.writesModes : AbilityAt → Bool
  | .spell _ (.modal _ _) => true
  | .italicHead _ ab => ab.writesModes
  | .alsoForKeywords ab _ => ab.writesModes
  | _ => false

def modalFrameOk (text : AbilitySeq) : Bool :=
  !text.any AbilityAt.keywordWantsModes || text.any AbilityAt.writesModes

def jointBindings : List QualitySort → Bindings → Bindings
  | [], bs => bs
  | q :: qs, bs => qualityB q :: jointBindings qs bs

def AbilityAt.choiceDelta (bs : Bindings) : AbilityAt → List Binding
  | .activated _ instr _ _ _ _ => instr.choiceDelta
  | .triggered _ _ _ _ _ _ _ _ instr => instr.choiceDelta
  | .static se => se.choiceDelta bs
  | .alsoForKeywords ab _ => AbilityAt.choiceDelta bs ab
  | .italicHead _ ab => AbilityAt.choiceDelta bs ab
  | .spell _ instr => instr.choiceDelta
  | _ => []

def textChoiceDelta (bs : Bindings) (text : AbilitySeq) : List Binding :=
  text.flatMap (AbilityAt.choiceDelta bs)

def jointChoicesOk (qs : List QualitySort) (made : List Binding) : Bool :=
  qs.all fun q => countChoice (.quality q) made != 0

/-- The laws a text obeys on a type line: class fit, modal frame, chapter frame, door frame. -/
def textLaws (l : TypeLine) (text : AbilitySeq) : List Refusal :=
  refuse (cardTextOk l.types text) .cardText ++ refuse (modalFrameOk text) .modalFrame ++
    refuse (chapterFrameOk l.subtypes text) .chapterFrame ++ refuse (doorFrameOk text) .doorFrame

/-- Idris `CharacteristicsLaws side c`. -/
def Characteristics.check (side : FaceSide) (c : Characteristics) : List Refusal :=
  let bs := jointBindings c.choices (costLetters c.cost)
  AbilitySeq.check bs c.text ++ refuse c.typeLine.cardLineOk .cardLine ++
    refuse (supersDistinct c.typeLine.supertypes) .distinct ++ textLaws c.typeLine c.text ++
    refuse (cardBoxOk side c.typeLine.types c.text c.box) .cardBox ++
    refuse (cardCostOk side c.typeLine.types c.cost) .cardCost ++
    refuse (jointChoicesOk c.choices (textChoiceDelta bs c.text)) .jointChoices

def SharedLineHalf.check (l : TypeLine) (box : Option PrintedBox) (h : SharedLineHalf) :
    List Refusal :=
  AbilitySeq.check (costLetters h.cost) h.text ++ textLaws l h.text ++
    refuse (cardBoxOk .front l.types h.text box) .cardBox ++
    refuse (cardCostOk .front l.types h.cost) .cardCost

def adventureInsetOk (l : TypeLine) : Bool := l.subtypes.elem (.spell "Adventure")
def flipHalfOk (l : TypeLine) : Bool := anyPermanentType l.types

/-- [CR#711.2a] a closed band, [CR#711.2b] the open last band. -/
def LevelRange.ok : LevelRange → Bool
  | .between from_ to => from_ ≤ to
  | .atLeast _ => true

def LevelRange.low : LevelRange → Nat
  | .between from_ _ => from_
  | .atLeast from_ => from_

def LevelRange.high : LevelRange → Option Nat
  | .between _ to => some to
  | .atLeast _ => none

def rangesOverlap (a b : LevelRange) : Bool :=
  match a.high, b.high with
  | none, none => true
  | some x, none => max a.low b.low ≤ x
  | none, some y => max a.low b.low ≤ y
  | some x, some y => max a.low b.low ≤ min x y

def bandsDisjoint : List LevelBand → Bool
  | [] => true
  | b :: bs => bs.all (fun c => !rangesOverlap b.range c.range) && bandsDisjoint bs

def levelerPtBox : Option PrintedBox → Bool
  | some (.pt _ _) => true
  | _ => false

def levelerFrameOk (l : TypeLine) (box : Option PrintedBox) (bands : List LevelBand) : Bool :=
  l.types.elem .creature && levelerPtBox box && !bands.isEmpty

def LevelBand.check (l : TypeLine) (b : LevelBand) : List Refusal :=
  refuse b.range.ok .levelRange ++ AbilitySeq.check [] b.text ++ textLaws l b.text ++
    refuse (cardBoxOk .front l.types b.text (some b.box)) .cardBox

/-- [CR#718.1] the inset frame's second set: a mana cost and a power/toughness box. -/
def prototypeFrameOk (l : TypeLine) : Option PrintedBox → Bool
  | some (.pt _ _) => l.types.elem .creature
  | _ => false

def PrototypeAlt.check (l : TypeLine) (a : PrototypeAlt) : List Refusal :=
  refuse (manaRun a.cost) .manaRun ++ refuse (cardBoxOk .front l.types [] (some a.box)) .cardBox

/-- The checker's entry point: every refusal in a card, in reading order. -/
def Card.check : Card → List Refusal
  | .singleFaced face => face.check .front
  | .transforming front back => front.check .front ++ back.check .back
  | .modalDfc front back => front.check .front ++ back.check .front
  | .split left right => left.check .front ++ right.check .front
  | .sharedLineSplit line box left right =>
    refuse line.cardLineOk .cardLine ++ refuse (supersDistinct line.supertypes) .distinct ++
      refuse (anyPermanentType line.types) .cardLine ++ left.check line box ++ right.check line box
  | .adventurer normal adventure =>
    normal.check .front ++ adventure.check .front ++
      refuse (adventureInsetOk adventure.typeLine) .adventureInset
  | .flip normal alternative =>
    normal.check .front ++ alternative.check .back ++ refuse (flipHalfOk normal.typeLine) .flipHalf ++
      refuse (flipHalfOk alternative.typeLine) .flipHalf
  | .leveler inner bands =>
    inner.check .front ++ refuse (levelerFrameOk inner.typeLine inner.box bands) .levelerFrame ++
      bands.flatMap (LevelBand.check inner.typeLine) ++ refuse (bandsDisjoint bands) .bandsDisjoint
  | .prototype inner alt =>
    inner.check .front ++ refuse (prototypeFrameOk inner.typeLine inner.box) .prototypeFrame ++
      alt.check inner.typeLine

/-- A card the checker admits: writing one runs the checker, as writing a card ran the Idris
elaborator. -/
structure Spelled where
  card : Card
  ok : card.check = []

/-- `spelled <| card …`: the checker's proof is found by `decide` at the definition. -/
def spelled (c : Card) (ok : c.check = [] := by decide) : Spelled := ⟨c, ok⟩

end Mtg
