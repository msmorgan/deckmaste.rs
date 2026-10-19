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

def StaticSpec.onInstantOrSorceryCardOk (s : StaticSpec) (fuel : Nat := 64) : Bool :=
  match fuel, s with
  | n + 1, .inCaller b => b.onInstantOrSorceryCardOk n
  | _ + 1, .altCost .this _ | _ + 1, .costShift .this _ | _ + 1, .addedCost _ _ _
  | _ + 1, .modeChoice _ => true
  | _ + 1, .cant _ action _ =>
    match action.scopeBody with
    | .enact (.action "counter") body _ _ => body.canonicalPatient.any (·.namesThis)
    | _ => false
  | _ + 1, .instead event _ _ _ _ =>
    match event with
    | .would _ action =>
      match action.scopeBody with
      | .move src _ _ _ _ _ => src.namesThis
      | _ => false
    | .wouldAny _ | .causedBy .. => false
  | n + 1, .conditional se _ _ => se.onInstantOrSorceryCardOk n
  | _, _ => false
termination_by structural fuel

def classAbilityOk (c : CardClass) (a : Ability) (fuel : Nat := 64) : Bool :=
  match fuel, a with
  | 0, _ => false
  | _ + 1, .keyword k _ _ => keywordCardOk c k
  | n + 1, .alsoForKeywords a _ | n + 1, .italicHead _ a | n + 1, .qualified a _ _ _ _ =>
    classAbilityOk c a n
  | _ + 1, .spell _ _ => c == .instantOrSorceryCard
  | _ + 1, .activated cost _ _ _ _ _ => c == .permanentCard || cost.offBattlefield
  | _ + 1, .static se => c == .permanentCard || se.onInstantOrSorceryCardOk
  | _ + 1, .mayBeginOnBattlefield | _ + 1, .thatAbility _ => c == .permanentCard
  | _ + 1, .triggered .. => true
termination_by structural fuel

def cardTextOk (tys : List CardType) (text : List Ability) : Bool :=
  text.all (fun a => classAbilityOk (cardClassOf tys) a)

def chapterLineOk (subs : List Subtype) (ab : Ability) (fuel : Nat := 64) : Bool :=
  match fuel with
  | 0 => true
  | n + 1 => match ab with
    | .triggered event _ _ _ _ _ _ _ =>
      match event.scopeBody with
      | .chapterMark _ => framesWith .chapters subs
      | _ => true
    | .italicHead _ ab => chapterLineOk subs ab n
    | .qualified ab _ _ _ _ => chapterLineOk subs ab n
    | .alsoForKeywords ab _ => chapterLineOk subs ab n
    | _ => true
termination_by structural fuel

def chapterFrameOk (subs : List Subtype) (text : List Ability) : Bool := text.all (chapterLineOk subs)

def doorFrameOk (text : List Ability) : Bool := text.all fun a => !a.namesThisDoor

/-! ## Printed-position references

"The abilities printed below <keyword>" reads the card's own text: only printed abilities are
selected [CR#702.165c], so the keyword is one the card prints. The walk follows the forms a
grant is written in (a static clause, an action that establishes one, and the ability
forms around them), as `namesThisDoor` does. -/

def AbilitySelection.printedKeyword : AbilitySelection → Option KeywordLabel
  | .printedAfter k => some k
  | _ => none

def CharacteristicEdit.printedKeyword : CharacteristicEdit → Option KeywordLabel
  | .removedAbilities selection => selection.printedKeyword
  | _ => none

def StaticSpec.printedAfterKeywords (s : StaticSpec) (fuel : Nat := 64) : List KeywordLabel :=
  match fuel, s with
  | n + 1, .inCaller b | n + 1, .conditional b _ _ => b.printedAfterKeywords n
  | n + 1, .conjunction _ ps => ps.flatMap (fun p => p.printedAfterKeywords n)
  | _ + 1, .abilityGrantSelected _ _ sel => sel.printedKeyword.toList
  | _ + 1, .characteristicChange _ es => es.filterMap CharacteristicEdit.printedKeyword
  | _, _ => []
termination_by structural fuel

def Action.printedAfterKeywords (i : Action) (fuel : Nat := 64) : List KeywordLabel :=
  match fuel, i with
  | n + 1, .inCaller b | n + 1, .ownScope _ b | n + 1, .instruct _ b
  | n + 1, .limited b _ => b.printedAfterKeywords n
  | _ + 1, .establish se _ => se.printedAfterKeywords
  | n + 1, .doIf _ b other => b.printedAfterKeywords n ++
    other.toList.flatMap (fun i => i.printedAfterKeywords n)
  | n + 1, .sequentially is | n + 1, .simultaneously is =>
    is.flatMap (fun i => i.printedAfterKeywords n)
  | _, _ => []
termination_by structural fuel

def Ability.printedAfterKeywords (a : Ability) (fuel : Nat := 64) : List KeywordLabel :=
  match fuel, a with
  | n + 1, .keyword _ ps body =>
    ps.flatMap (fun p => match p with | .ability a => a.printedAfterKeywords n | _ => []) ++
      body.flatMap (fun a => a.printedAfterKeywords n)
  | _ + 1, .activated _ i _ _ _ _ | _ + 1, .triggered _ _ _ _ _ _ _ i
  | _ + 1, .spell _ i => i.printedAfterKeywords
  | _ + 1, .static se => se.printedAfterKeywords
  | n + 1, .italicHead _ a | n + 1, .alsoForKeywords a _ | n + 1, .qualified a _ _ _ _ =>
    a.printedAfterKeywords n
  | _, _ => []
termination_by structural fuel

def Ability.printedAfterKeywordsAll (text : List Ability) : List KeywordLabel :=
  text.flatMap (fun a => a.printedAfterKeywords)

/-- Whether a text line prints `k`'s keyword line. -/
def Ability.printsKeyword (k : KeywordLabel) (ab : Ability) (fuel : Nat := 64) : Bool :=
  match fuel with
  | 0 => false
  | n + 1 => match ab with
    | .keyword k' _ _ => k' == k
    | .italicHead _ ab => ab.printsKeyword k n
    | .alsoForKeywords ab _ => ab.printsKeyword k n
    | _ => false
termination_by structural fuel

/-- Each keyword a printed-position selection names is one the text prints. -/
def printedAfterLaws (text : List Ability) : List Refusal :=
  (Ability.printedAfterKeywordsAll text).filterMap fun k =>
    if text.any (Ability.printsKeyword k) then none else some (.printedAfterKeyword k)

def StaticSpec.definedSlots (s : StaticSpec) (fuel : Nat := 64) : List DefinedSlots :=
  match fuel, s with
  | n + 1, .inCaller b | n + 1, .conditional b _ _ => b.definedSlots n
  | _ + 1, .ptDefinition _ sl _ => [sl]
  | n + 1, .conjunction _ ps => ps.flatMap (fun p => p.definedSlots n)
  | _, _ => []
termination_by structural fuel

def Ability.definesPt (a : Ability) (fuel : Nat := 64) : List DefinedSlots :=
  match fuel, a with
  | _ + 1, .static se => se.definedSlots
  | n + 1, .italicHead _ a | n + 1, .qualified a _ _ _ _ => a.definesPt n
  | _, _ => []
termination_by structural fuel

def textDefines (f : DefinedSlots → Bool) (text : List Ability) : Bool :=
  text.any fun a => (a.definesPt).any f

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

def Characteristics.lineNonEmpty (c : Characteristics) : Bool :=
  hasAnyTypeCharacteristic c.supertypes c.types c.subtypes

def Characteristics.cardLineOk (c : Characteristics) : Bool :=
  c.lineNonEmpty && typesDistinct c.types && typesCombinable c.types &&
    subsFitLine c.subtypes c.types

def Ability.keywordWantsModes (ab : Ability) (fuel : Nat := 64) : Bool :=
  match fuel with
  | 0 => false
  | n + 1 => match ab with
    | .keyword k _ _ => (keywordFactsFor k).elim false (·.wantsModes)
    | .italicHead _ ab => ab.keywordWantsModes n
    | .alsoForKeywords ab _ => ab.keywordWantsModes n
    | .qualified ab _ _ _ _ => ab.keywordWantsModes n
    | _ => false
termination_by structural fuel

def Ability.writesModes (ab : Ability) (fuel : Nat := 64) : Bool :=
  match fuel with
  | 0 => false
  | n + 1 => match ab with
    | .spell _ action =>
      match action.scopeBody with
      | .chooseModes _ _ => true
      | _ => false
    | .italicHead _ ab => ab.writesModes n
    | .alsoForKeywords ab _ => ab.writesModes n
    | .qualified ab _ _ _ _ => ab.writesModes n
    | _ => false
termination_by structural fuel

/-- A Mode Change written on the text. -/
def Ability.writesModeChoice (ab : Ability) (fuel : Nat := 64) : Bool :=
  match fuel with
  | 0 => false
  | n + 1 => match ab with
    | .static (.modeChoice _) => true
    | .italicHead _ ab => ab.writesModeChoice n
    | .alsoForKeywords ab _ => ab.writesModeChoice n
    | .qualified ab _ _ _ _ => ab.writesModeChoice n
    | _ => false
termination_by structural fuel

/-- Any ability whose action is a modal choice: a spell, an activated ability or a
triggered ability [CR#700.2a,700.2b]. -/
def Ability.hasModes (ab : Ability) (fuel : Nat := 64) : Bool :=
  match fuel with
  | 0 => false
  | n + 1 => match ab with
    | .spell _ action | .activated _ action _ _ _ _
    | .triggered _ _ _ _ _ _ _ action =>
      match action.scopeBody with
      | .chooseModes _ _ => true
      | _ => false
    | .italicHead _ ab => ab.hasModes n
    | .alsoForKeywords ab _ => ab.hasModes n
    | .qualified ab _ _ _ _ => ab.hasModes n
    | _ => false
termination_by structural fuel

/-- A modal keyword needs a modal spell; a written Mode Change needs modes to change. -/
def modalFrameOk (text : List Ability) : Bool :=
  (!text.any Ability.keywordWantsModes || text.any Ability.writesModes) &&
    (!text.any Ability.writesModeChoice || text.any Ability.hasModes)

/-- The text laws a shared-line half shares with a face; a door header belongs to a Room's
shared line, so the door-frame law is a face's alone. -/
def halfTextLaws (line : Characteristics) (text : List Ability) : List Refusal :=
  refuse (cardTextOk line.types text) .cardText ++ refuse (modalFrameOk text) .modalFrame ++
    refuse (chapterFrameOk line.subtypes text) .chapterFrame ++ printedAfterLaws text

/-- The laws a text obeys on a type line: class fit, modal frame, chapter frame, door frame. -/
def textLaws (line : Characteristics) (text : List Ability) : List Refusal :=
  halfTextLaws line text ++ refuse (doorFrameOk text) .doorFrame

/-- The laws of a type line alone (a shared line has nothing else). -/
def Characteristics.lineLaws (c : Characteristics) : List Refusal :=
  refuse c.cardLineOk .cardLine ++ refuse (supersDistinct c.supertypes) .distinct

/-- Idris `CharacteristicsLaws side c`: a face is named. -/
def Characteristics.check (drs : DRS) (c : Characteristics) : List Refusal :=
  [c.power, c.toughness, c.loyalty, c.defense].flatMap
    (fun a => a.toList.flatMap (Amount.check drs)) ++ Ability.checkAll drs c.text

def CardFace.check (side : FaceSide) (f : CardFace) : List Refusal :=
  let c := f.characteristics
  refuse c.name.isSome .cardName ++ Ability.checkText (DRS.ofList (costLetters c.cost)) c.text ++
    c.lineLaws ++ textLaws c c.text ++ refuse (cardBoxOk side c.types c.text c) .cardBox ++
    refuse (cardCostOk side c.types c.cost) .cardCost

def SharedLineHalf.check (shared : CardFace) (h : SharedLineHalf) : List Refusal :=
  let line := shared.characteristics
  Ability.checkText (DRS.ofList (costLetters h.cost)) h.text ++ halfTextLaws line h.text ++
    refuse (cardBoxOk .front line.types h.text line) .cardBox ++
    refuse (cardCostOk .front line.types h.cost) .cardCost

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
  refuse b.range.ok .levelRange ++ Ability.checkText (DRS.ofList []) b.text ++ textLaws inner b.text ++
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
    refuse (alternative.cost.elim true (!·.isEmpty)) .nonEmpty ++
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
      left.check shared ++ right.check shared
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

/-- A card with semantic validity and retained evidence of template- and definition-only
authoring. The elaborator connects the authoring tree to the input term before their reduction. -/
structure Spelled where
  private mk ::
  card : Card
  authoring : Authoring.Form
  onlyAuthored : authoring.onlyAuthored = true
  ok : card.check = []

syntax (name := spelledTerm) "spelled" " <| " term : term

open Lean Meta Elab Term in
/-- `spelled <| card …` checks the authoring boundary and synthesizes both proofs. -/
@[term_elab spelledTerm] unsafe def elabSpelled : TermElab := fun stx _ => do
  let `(spelled <| $card) := stx | throwUnsupportedSyntax
  let value ← elabTermEnsuringType card (mkConst ``Semantics.Card)
  let form ← Authoring.inspect value
  unless form.onlyAuthored do
    throwError "Card definitions must use semantic templates or definitions; raw constructors: {form.rawNames.eraseDups}"
  let tree := toExpr form
  let authored ← mkEq (mkApp (mkConst ``Authoring.Form.onlyAuthored) tree) (mkConst ``Bool.true)
  let onlyAuthored ← mkDecideProof authored
  let verdict := mkApp (mkConst ``Card.check) value
  let refusals ← evalExpr (List Refusal)
    (mkApp (mkConst ``List [Level.zero]) (mkConst ``Refusal)) verdict
  unless refusals.isEmpty do
    throwError "Card semantic check failed: {repr refusals}"
  let valid ← mkEq verdict (mkApp (mkConst ``List.nil [Level.zero]) (mkConst ``Refusal))
  let ok ← mkDecideProof valid
  return mkAppN (mkConst ``Spelled.mk) #[value, tree, onlyAuthored, ok]

end Semantics
