import Semantics.Card
import Semantics.Check.AbilityRules

/-!
# Semantics.Check.Card

The card frame laws of `Card.idr` and the checker's entry point: `check : Card → List
Refusal`, and `Spelled`, a card the checker admits.
-/

namespace Semantics

/-- Which side of a two-faced card a face is printed on; the back has no mana cost. -/
inductive FaceSide where
  | front | back
  deriving DecidableEq, Repr

/-- A card is a permanent card or a spell card by its types [CR#110.4a]. -/
inductive CardClass where
  | permanentCard | spellCard
  deriving DecidableEq, Repr

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

def classAbilityOk : CardClass → Ability → Bool
  | .permanentCard, .keyword k _ _ => keywordCardOk .permanentCard k
  | .permanentCard, .spell _ _ => false
  | .permanentCard, .alsoForKeywords ab _ => classAbilityOk .permanentCard ab
  | .permanentCard, .italicHead _ ab => classAbilityOk .permanentCard ab
  | .permanentCard, _ => true
  | .spellCard, .keyword k _ _ => keywordCardOk .spellCard k
  | .spellCard, .activated c _ _ _ _ _ => c.offBattlefield
  | .spellCard, .triggered _ _ _ _ _ _ _ _ => true
  | .spellCard, .static se => se.onSpellCardOk
  | .spellCard, .alsoForKeywords ab _ => classAbilityOk .spellCard ab
  | .spellCard, .spell _ _ => true
  | .spellCard, .italicHead _ ab => classAbilityOk .spellCard ab
  | .spellCard, .mayBeginOnBattlefield => false
  | _, .thatAbility _ => false

def cardTextOk (tys : List CardType) (text : List Ability) : Bool :=
  text.all (classAbilityOk (cardClassOf tys))

def chapterLineOk (subs : List Subtype) : Ability → Bool
  | .triggered (.chapterMark _) _ _ _ _ _ _ _ => framesWith .chapters subs
  | .italicHead _ ab => chapterLineOk subs ab
  | .alsoForKeywords ab _ => chapterLineOk subs ab
  | _ => true

def chapterFrameOk (subs : List Subtype) (text : List Ability) : Bool := text.all (chapterLineOk subs)

def doorFrameOk (text : List Ability) : Bool := text.all fun a => !a.namesThisDoor

def StaticSpec.definedSlots : StaticSpec → Option DefinedSlots
  | .definesPt _ sl _ => some sl
  | .conditionally se _ _ => se.definedSlots
  | _ => none

def Ability.definesPt : Ability → Option DefinedSlots
  | .static se => se.definedSlots
  | .italicHead _ ab => ab.definesPt
  | _ => none

def textDefines (f : DefinedSlots → Bool) (text : List Ability) : Bool :=
  text.any fun a => (a.definesPt).elim false f

/-- A power or toughness slot against the text that may define it: on a creature an absent slot
is the printed `*` a characteristic-defining ability fills, and nothing else fills it
[CR#208.1,604.3]; on a noncreature a definer leaves the slot absent. -/
def ptSlotOk (creature : Bool) (defined : Bool) (slot : Option Amount) : Bool :=
  if creature then slot.isNone == defined else !defined || slot.isNone

/-- The stat slots a face prints against its types [CR#208.1,209.1,210.1]: a creature has
power and toughness, a planeswalker loyalty (the back face of a transforming one may leave it
absent), a battle defense; loyalty and defense belong only to those types. -/
def boxSuitsTypes (side : FaceSide) (tys : List CardType) (c : Characteristics) : Bool :=
  (c.power.isSome == c.toughness.isSome) &&
    (!tys.elem .planeswalker || c.loyalty.isSome || side == .back) &&
    (c.loyalty.isNone || tys.elem .planeswalker) &&
    (!tys.elem .battle || c.defense.isSome) && (c.defense.isNone || tys.elem .battle)

def cardBoxOk (side : FaceSide) (tys : List CardType) (text : List Ability)
    (c : Characteristics) : Bool :=
  boxSuitsTypes side tys c &&
    ptSlotOk (tys.elem .creature) (textDefines DefinedSlots.power text) c.power &&
    ptSlotOk (tys.elem .creature) (textDefines DefinedSlots.toughness text) c.toughness

def cardCostOk : FaceSide → List CardType → Option ManaCost → Bool
  | _, _, none => true
  | .back, _, some _ => false
  | .front, tys, some _ => !tys.elem .land

def Characteristics.cardLineOk (c : Characteristics) : Bool :=
  c.lineNonEmpty && typesDistinct c.types && typesCombinable c.types &&
    subsFitLine c.subtypes c.types

def Ability.keywordWantsModes : Ability → Bool
  | .keyword k _ _ => (keywordFactsFor k).elim false (·.wantsModes)
  | .italicHead _ ab => ab.keywordWantsModes
  | .alsoForKeywords ab _ => ab.keywordWantsModes
  | _ => false

def Ability.writesModes : Ability → Bool
  | .spell _ (.modal _ _) => true
  | .italicHead _ ab => ab.writesModes
  | .alsoForKeywords ab _ => ab.writesModes
  | _ => false

def modalFrameOk (text : List Ability) : Bool :=
  !text.any Ability.keywordWantsModes || text.any Ability.writesModes

def jointBindings : List QualitySort → Bindings → Bindings
  | [], bs => bs
  | q :: qs, bs => qualityB q :: jointBindings qs bs

def Ability.choiceDelta (bs : Bindings) : Ability → List Binding
  | .activated _ instr _ _ _ _ => instr.choiceDelta
  | .triggered _ _ _ _ _ _ _ instr => instr.choiceDelta
  | .static se => se.choiceDelta bs
  | .alsoForKeywords ab _ => Ability.choiceDelta bs ab
  | .italicHead _ ab => Ability.choiceDelta bs ab
  | .spell _ instr => instr.choiceDelta
  | _ => []

def textChoiceDelta (bs : Bindings) (text : List Ability) : List Binding :=
  text.flatMap (Ability.choiceDelta bs)

def jointChoicesOk (qs : List QualitySort) (made : List Binding) : Bool :=
  qs.all fun q => countChoice (.quality q) made != 0

/-- The text laws a shared-line half shares with a face; a door header belongs to a Room's
shared line, so the door-frame law is a face's alone. -/
def halfTextLaws (line : Characteristics) (text : List Ability) : List Refusal :=
  refuse (cardTextOk line.types text) .cardText ++ refuse (modalFrameOk text) .modalFrame ++
    refuse (chapterFrameOk line.subtypes text) .chapterFrame

/-- The laws a text obeys on a type line: class fit, modal frame, chapter frame, door frame. -/
def textLaws (line : Characteristics) (text : List Ability) : List Refusal :=
  halfTextLaws line text ++ refuse (doorFrameOk text) .doorFrame

/-- The laws of a type line alone (a shared line has nothing else). -/
def Characteristics.lineLaws (c : Characteristics) : List Refusal :=
  refuse c.cardLineOk .cardLine ++ refuse (supersDistinct c.supertypes) .distinct

/-- Idris `CharacteristicsLaws side c`: a face is named. -/
def Characteristics.check (side : FaceSide) (c : Characteristics) : List Refusal :=
  let bs := jointBindings c.choices (costLetters c.cost)
  refuse c.name.isSome .cardName ++ Ability.checkText bs c.text ++ c.lineLaws ++
    textLaws c c.text ++ refuse (cardBoxOk side c.types c.text c) .cardBox ++
    refuse (cardCostOk side c.types c.cost) .cardCost ++ (c.cost.map ManaCost.check).getD [] ++
    refuse (jointChoicesOk c.choices (textChoiceDelta bs c.text)) .jointChoices

def SharedLineHalf.check (shared : Characteristics) (h : SharedLineHalf) : List Refusal :=
  Ability.checkText (costLetters h.cost) h.text ++ halfTextLaws shared h.text ++
    refuse (cardBoxOk .front shared.types h.text shared) .cardBox ++
    refuse (cardCostOk .front shared.types h.cost) .cardCost ++
    (h.cost.map ManaCost.check).getD []

def adventureInsetOk (c : Characteristics) : Bool := framesWith .adventureInset c.subtypes
def flipHalfOk (c : Characteristics) : Bool := anyPermanentType c.types

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

def Characteristics.ptWritten (c : Characteristics) : Bool := c.power.isSome && c.toughness.isSome

def levelerFrameOk (inner : Characteristics) (bands : List LevelBand) : Bool :=
  inner.types.elem .creature && inner.ptWritten && !bands.isEmpty

/-- A level band writes a power and toughness and text [CR#711.2]. -/
def LevelBand.check (inner : Characteristics) (b : LevelBand) : List Refusal :=
  refuse b.range.ok .levelRange ++ Ability.checkText [] b.band.text ++ textLaws inner b.band.text ++
    refuse b.band.ptWritten .levelerFrame ++
    refuse (cardBoxOk .front inner.types b.band.text b.band) .cardBox

/-- [CR#718.1] the inset frame's second set: a mana cost and a power/toughness box. -/
def prototypeFrameOk (inner : Characteristics) : Bool :=
  inner.types.elem .creature && inner.ptWritten

def prototypeAltCheck (inner alternative : Characteristics) : List Refusal :=
  refuse alternative.cost.isSome .cardCost ++
    refuse (alternative.cost.elim true manaRun) .manaRun ++
    (alternative.cost.map ManaCost.check).getD [] ++
    refuse alternative.ptWritten .prototypeFrame ++
    refuse (cardBoxOk .front inner.types [] alternative) .cardBox

/-- The checker's entry point: every refusal in a card, in reading order. -/
def Card.check : Card → List Refusal
  | .singleFaced face => face.check .front
  | .transforming front back => front.check .front ++ back.check .back
  | .modalDfc front back => front.check .front ++ back.check .front
  | .split left right => left.check .front ++ right.check .front
  | .sharedLineSplit shared left right =>
    shared.lineLaws ++ refuse (anyPermanentType shared.types) .cardLine ++
      left.check shared ++ right.check shared
  | .adventurer normal adventure =>
    normal.check .front ++ adventure.check .front ++
      refuse (adventureInsetOk adventure) .adventureInset
  | .flip normal alternative =>
    normal.check .front ++ alternative.check .back ++ refuse (flipHalfOk normal) .flipHalf ++
      refuse (flipHalfOk alternative) .flipHalf
  | .leveler inner bands =>
    inner.check .front ++ refuse (levelerFrameOk inner bands) .levelerFrame ++
      bands.flatMap (LevelBand.check inner) ++ refuse (bandsDisjoint bands) .bandsDisjoint
  | .prototype inner alternative =>
    inner.check .front ++ refuse (prototypeFrameOk inner) .prototypeFrame ++
      prototypeAltCheck inner alternative

/-- A card the checker admits: writing one runs the checker, as writing a card ran the Idris
elaborator. -/
structure Spelled where
  card : Card
  ok : card.check = []

/-- `spelled <| card …`: the checker's proof is found by `decide` at the definition. -/
def spelled (c : Card) (ok : c.check = [] := by decide) : Spelled := ⟨c, ok⟩

end Semantics
