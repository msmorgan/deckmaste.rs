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

/-- A card is a permanent card or an instant or sorcery card by its types [CR#110.4a]. -/
inductive CardClass where
  | permanentCard | instantOrSorceryCard
  deriving DecidableEq, Repr

def cardClassOf : List CardType → CardClass
  | [] => .permanentCard
  | t :: ts => if t.isInstantOrSorcery then .instantOrSorceryCard else cardClassOf ts

def anyPermanentType (tys : List CardType) : Bool := tys.any CardType.permanent
def anyInstantOrSorceryType (tys : List CardType) : Bool := tys.any CardType.isInstantOrSorcery
def hasNonKindredType (tys : List CardType) : Bool := tys.any (· != .kindred)

def typesCombinable (tys : List CardType) : Bool :=
  !(anyPermanentType tys && anyInstantOrSorceryType tys) && (!tys.elem .kindred || hasNonKindredType tys)

def keywordCardOk : CardClass → KeywordLabel → Bool
  | .permanentCard, k => (keywordFactsFor k).elim false (·.onPermanentCard)
  | .instantOrSorceryCard, k => (keywordFactsFor k).elim false (·.onInstantOrSorceryCard)

def StaticSpec.onInstantOrSorceryCardOk : StaticSpec → Bool
  | .deonticRule _ .forbid deeds .patient _ _ _ _ =>
    deeds.all fun deed => deedZoneOf deed .patient == some .stack
  | .altCost .this _ => true
  | .costShift .this _ => true
  | .addedCost _ _ => true
  | .partScope _ _ se => se.onInstantOrSorceryCardOk
  | .conditional se _ _ => se.onInstantOrSorceryCardOk
  | _ => false

def classAbilityOk : CardClass → Ability → Bool
  | .permanentCard, .keyword k _ _ => keywordCardOk .permanentCard k
  | .permanentCard, .spell _ _ => false
  | .permanentCard, .alsoForKeywords ab _ => classAbilityOk .permanentCard ab
  | .permanentCard, .italicHead _ ab => classAbilityOk .permanentCard ab
  | .permanentCard, _ => true
  | .instantOrSorceryCard, .keyword k _ _ => keywordCardOk .instantOrSorceryCard k
  | .instantOrSorceryCard, .activated c _ _ _ _ _ => c.offBattlefield
  | .instantOrSorceryCard, .triggered _ _ _ _ _ _ _ _ => true
  | .instantOrSorceryCard, .static se => se.onInstantOrSorceryCardOk
  | .instantOrSorceryCard, .alsoForKeywords ab _ => classAbilityOk .instantOrSorceryCard ab
  | .instantOrSorceryCard, .spell _ _ => true
  | .instantOrSorceryCard, .italicHead _ ab => classAbilityOk .instantOrSorceryCard ab
  | .instantOrSorceryCard, .mayBeginOnBattlefield => false
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

mutual
  /-- The stat slots a static clause defines; a shared-subject clause ("its power is … and its
  toughness is …") defines what its parts define. Idris stopped at the clause because its
  printed `*` was explicit; here the absent slot is the star, so the definer must be found
  wherever it sits. -/
  def StaticSpec.definedSlots : StaticSpec → List DefinedSlots
    | .ptDefinition _ sl _ => [sl]
    | .conditional se _ _ => se.definedSlots
    | .conjunction _ parts => StaticSpec.definedSlotsAll parts
    | _ => []
  def StaticSpec.definedSlotsAll : List StaticSpec → List DefinedSlots
    | [] => []
    | se :: rest => se.definedSlots ++ StaticSpec.definedSlotsAll rest
end

def Ability.definesPt : Ability → List DefinedSlots
  | .static se => se.definedSlots
  | .italicHead _ ab => ab.definesPt
  | _ => []

def textDefines (f : DefinedSlots → Bool) (text : List Ability) : Bool :=
  text.any fun a => a.definesPt.any f

/-- A power or toughness slot against the text that may define it: on a creature an absent slot
is the printed `*` a characteristic-defining ability fills, and nothing else fills it
[CR#208.1,604.3]; on a noncreature a definer leaves the slot absent. -/
def ptSlotOk (creature : Bool) (defined : Bool) (slot : Option Amount) : Bool :=
  if creature then slot.isNone == defined else !defined || slot.isNone

/-- The stat slots a face prints against its types [CR#208.1,209.1,210.1]: a creature has
power and toughness, a planeswalker loyalty (the back face of a transforming one may leave it
absent), a battle defense; loyalty and defense belong only to those types. -/
def boxSuitsTypes (side : FaceSide) (tys : List CardType) (c : Characteristics) : Bool :=
  (!tys.elem .planeswalker || c.loyalty.isSome || side == .back) &&
    (c.loyalty.isNone || tys.elem .planeswalker) &&
    (!tys.elem .battle || c.defense.isSome) && (c.defense.isNone || tys.elem .battle)

/-- A written stat in a printed box is a literal; absent slots retain their box/type laws.
Effect-written characteristics instead check their expressions in the supplied context. -/
def Characteristics.printedStatsOk (c : Characteristics) : Bool :=
  [c.power, c.toughness, c.loyalty, c.defense].all fun
    | none | some (.lit _) => true
    | some _ => false

def cardBoxOk (side : FaceSide) (tys : List CardType) (text : List Ability)
    (c : Characteristics) : Bool :=
  c.printedStatsOk && boxSuitsTypes side tys c &&
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
  | .spell _ (.chooseModes _ _) => true
  | .italicHead _ ab => ab.writesModes
  | .alsoForKeywords ab _ => ab.writesModes
  | _ => false

def modalFrameOk (text : List Ability) : Bool :=
  !text.any Ability.keywordWantsModes || text.any Ability.writesModes

def jointBindings : List QualitySort → Bindings → Bindings
  | [], bs => bs
  | q :: qs, bs => qualityB q :: jointBindings qs bs

def Ability.introducedChoices (bs : Bindings) : Ability → List Binding
  | .activated _ instr _ _ _ _ => instr.introducedChoices
  | .triggered _ _ _ _ _ _ _ instr => instr.introducedChoices
  | .static se => se.introducedChoices bs
  | .alsoForKeywords ab _ => Ability.introducedChoices bs ab
  | .italicHead _ ab => Ability.introducedChoices bs ab
  | .spell _ instr => instr.introducedChoices
  | _ => []

def textIntroducedChoices (bs : Bindings) (text : List Ability) : List Binding :=
  text.flatMap (Ability.introducedChoices bs)

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
def CardFace.check (side : FaceSide) (f : CardFace) : List Refusal :=
  let c := f.characteristics
  let bs := jointBindings f.choices (costLetters c.cost)
  refuse c.name.isSome .cardName ++ Ability.checkText bs c.text ++ c.lineLaws ++
    textLaws c c.text ++ refuse (cardBoxOk side c.types c.text c) .cardBox ++
    refuse (cardCostOk side c.types c.cost) .cardCost ++ (c.cost.map ManaCost.check).getD [] ++
    refuse (jointChoicesOk f.choices (textIntroducedChoices bs c.text)) .jointChoices

/-- A half reads its own cost's letters and the choices the shared line announces. -/
def SharedLineHalf.bindings (shared : CardFace) (h : SharedLineHalf) : Bindings :=
  jointBindings shared.choices (costLetters h.cost)

def SharedLineHalf.introducedChoices (shared : CardFace) (h : SharedLineHalf) : List Binding :=
  textIntroducedChoices (h.bindings shared) h.text

def SharedLineHalf.check (shared : CardFace) (h : SharedLineHalf) : List Refusal :=
  let line := shared.characteristics
  Ability.checkText (h.bindings shared) h.text ++ halfTextLaws line h.text ++
    refuse (cardBoxOk .front line.types h.text line) .cardBox ++
    refuse (cardCostOk .front line.types h.cost) .cardCost ++
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

/-- A band's power/toughness box, as the characteristics set the box law reads [CR#711.2]. -/
def LevelBand.box (b : LevelBand) : Characteristics :=
  { power := b.power, toughness := b.toughness }

/-- A level band writes a power and toughness and text [CR#711.2]. -/
def LevelBand.check (inner : Characteristics) (b : LevelBand) : List Refusal :=
  refuse b.range.ok .levelRange ++ Ability.checkText [] b.text ++ textLaws inner b.text ++
    refuse b.box.ptWritten .levelerFrame ++
    refuse (cardBoxOk .front inner.types b.text b.box) .cardBox

/-- [CR#718.1] the inset frame's second set: a mana cost and a power/toughness box. -/
def prototypeFrameOk (inner : Characteristics) : Bool :=
  inner.types.elem .creature && inner.ptWritten

/-- The inset frame's second set, as the characteristics set the box law reads [CR#718.1]. -/
def PrototypeFrame.box (a : PrototypeFrame) : Characteristics :=
  { cost := a.cost, power := a.power, toughness := a.toughness }

def prototypeAltCheck (inner : Characteristics) (alternative : PrototypeFrame) : List Refusal :=
  refuse alternative.cost.isSome .cardCost ++
    refuse (alternative.cost.elim true manaRun) .manaRun ++
    (alternative.cost.map ManaCost.check).getD [] ++
    refuse alternative.box.ptWritten .prototypeFrame ++
    refuse (cardBoxOk .front inner.types [] alternative.box) .cardBox

/-- The checker's entry point: every refusal in a card, in reading order. -/
def Card.check : Card → List Refusal
  | .singleFaced face => face.check .front
  | .transforming front back => front.check .front ++ back.check .back
  | .modalDfc front back => front.check .front ++ back.check .front
  | .split left right => left.check .front ++ right.check .front
  | .sharedLineSplit shared left right =>
    let line := shared.characteristics
    line.lineLaws ++ refuse (anyPermanentType line.types) .cardLine ++
      left.check shared ++ right.check shared ++
      refuse (jointChoicesOk shared.choices
        (left.introducedChoices shared ++ right.introducedChoices shared)) .jointChoices
  | .adventurer normal adventure =>
    normal.check .front ++ adventure.check .front ++
      refuse (adventureInsetOk adventure.characteristics) .adventureInset
  | .flip normal alternative =>
    normal.check .front ++ alternative.check .back ++
      refuse (flipHalfOk normal.characteristics) .flipHalf ++
      refuse (flipHalfOk alternative.characteristics) .flipHalf
  | .leveler inner bands =>
    let line := inner.characteristics
    inner.check .front ++ refuse (levelerFrameOk line bands) .levelerFrame ++
      bands.flatMap (LevelBand.check line) ++ refuse (bandsDisjoint bands) .bandsDisjoint
  | .prototype inner alternative =>
    let line := inner.characteristics
    inner.check .front ++ refuse (prototypeFrameOk line) .prototypeFrame ++
      prototypeAltCheck line alternative

/-- A card with semantic validity and retained evidence of macro-only authoring. The
elaborator connects the authoring tree to the input term before macro reduction. -/
structure Spelled where
  private mk ::
  card : Card
  authoring : Authoring.Form
  onlyMacros : authoring.onlyMacros = true
  ok : card.check = []

open Lean Meta Elab Term in
/-- `spelled <| card …` checks the authoring boundary and synthesizes both proofs. -/
elab "spelled" " <| " card:term : term => do
  let value ← elabTermEnsuringType card (mkConst ``Semantics.Card)
  let form ← Authoring.inspect value
  unless form.onlyMacros do
    throwError "Card definitions must use semantic macros; raw constructors: {form.rawNames.eraseDups}"
  let tree := toExpr form
  let authored ← mkEq (mkApp (mkConst ``Authoring.Form.onlyMacros) tree) (mkConst ``Bool.true)
  let onlyMacros ← mkDecideProof authored
  let verdict := mkApp (mkConst ``Card.check) value
  let valid ← mkEq verdict (mkApp (mkConst ``List.nil [Level.zero]) (mkConst ``Refusal))
  let ok ← mkDecideProof valid
  return mkAppN (mkConst ``Spelled.mk) #[value, tree, onlyMacros, ok]

end Semantics
