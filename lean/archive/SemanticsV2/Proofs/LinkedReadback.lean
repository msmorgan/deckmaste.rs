import Semantics.Macros
import Semantics.Check.Card
import Semantics.Check.Rules
import Semantics.Macros.Declared

/-!
# Semantics.Proofs.LinkedReadback

A linked ability reads back a Linked Facet of a Link Source, and a named link between two
objects is a declared Relation (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-09). The
facets `objects` and `taken`, the Link Source `byDeed`, the three relation forms, `merge`,
`addsTarget`, and the mana-cost term of `CostShift.bySymbols` are pinned here; the renamed
facets keep their pins in `Proofs/LinkedChoiceReadback.lean` and `Proofs/Description.lean`.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.LinkedReadback

def sacrifice : Deed := .action "sacrifice"
def exile : Deed := .action "exile"
def tap : Deed := .action "tap"
def haunting : RelationLabel := .named "haunting"
def paired : RelationLabel := .named "paired"

/-- "the sacrificed creature", sacrificed by the alternative cost (emerge) [CR#702.119a]. -/
def theSacrificed : NounPhrase := linkedObject .theAlternative sacrifice .this

/-- Required pin: "the sacrificed creature's mana value" reads back one object
[CR#702.119a,607.2a]. -/
theorem okSacrificedCreaturesManaValue :
    Amount.check [] (.statOf (.stat .manaValue) theSacrificed) = [] := by
  decide

/-- Required pin: "the exiled card" reads the printed ability that exiled it (`byDeed`)
[CR#607.2a]. -/
theorem okExiledCardByDeed :
    NounPhrase.check (some .object) [] (linkedObject (.byDeed exile) exile .this) = [] := by
  decide

/-- Required pin: the objects of a deed the source never performs are refused: an ability named
by its exiling never sacrificed anything [CR#607.2a]. -/
theorem badObjectsOfADeedTheSourceNeverPerforms :
    NounPhrase.check (some .object) [] (linkedObject (.byDeed exile) sacrifice .this)
      = [.linkedFacetNamed] := by
  decide

/-- A keyword ability's own deed is performed only by that keyword's source: a kicker cost
never crewed anything. -/
theorem badKeywordDeedOfAnotherKeyword :
    NounPhrase.check (some .object) [] (linkedObject (.byKeyword (.named "kicker"))
      (.ofAbility (.named "crew")) .this) = [.linkedFacetNamed] := by
  decide

/-- The objects a deed acted on are objects, read as a noun phrase, never as an amount. -/
theorem badObjectsReadAsAnAmount :
    Amount.check [] (.paid (.objects .theAlternative sacrifice .one) .this)
      = [.linkedFacetKind] := by
  decide

/-- The objects facet is an object: a player context refuses it. -/
theorem badObjectsAsPlayer :
    NounPhrase.check (some .player) [] theSacrificed = [.kindMismatch .player .object] := by
  decide

/-- "The exiled cards" read as one object refuse a singular stat read. -/
theorem badManyObjectsStat :
    Amount.check [] (.statOf (.stat .power) (linkedObjects (.byDeed exile) exile .this))
      = [.singular] := by
  decide

/-- Required pin: "if tribute wasn't paid" reads whether the offered choice was taken
[CR#702.104b]. A keyword with a paid cost names a source. -/
theorem okChoiceTaken :
    Condition.check [] (.not (choiceWasTaken (.byKeyword (.named "kicker")) .this)) = [] := by
  decide

/-- A keyword with no cost names no source for `taken`. -/
theorem badTakenUnnamed :
    Condition.check [] (choiceWasTaken (.byKeyword (.named "flying")) .this)
      = [.linkedFacetNamed] := by
  decide

/-- Required pin: "exile it haunting target creature" relates two objects [CR#702.55a]. -/
theorem okRelateHaunting :
    Instruction.check [] (.relate .this haunting (target creature)) = [] := by
  decide

/-- Required pin: `relate` against a declared kind mismatch is refused: a haunting's second side
is an object, never a player [CR#702.55b]. -/
theorem badRelateKindMismatch :
    Instruction.check [] (.relate .this haunting .you) = [.kindMismatch .object .player] := by
  decide

/-- An undeclared relation is refused. -/
theorem badRelateUndeclared :
    Instruction.check [] (.relate .this (.named "befriended") (target creature))
      = [.knownRelation (.named "befriended")] := by
  decide

/-- "the creature it haunts" [CR#702.55b]. -/
theorem okCounterpart :
    NounPhrase.check (some .object) [] (.counterpart haunting .this) = [] := by
  decide

/-- "Paired" and "paired with this creature" are object predicates [CR#702.95b]. -/
theorem okRelated :
    (Condition.check [] (.matches thisCreature (.related paired none)),
      Condition.check [] (.exists_ (.described .bare (.and [creature, .related paired (some thisCreature)]))))
      = ([], []) := by
  decide

/-- The relation definitions the registry declares check clean, and a relation with a side that
is no entity is refused. -/
theorem okRelationDefinition :
    (Definition.check (.relation haunting .object .object),
      Definition.check (.relation (.named "x") .object .pile))
      = ([], [.definitionHolder .pile]) := by
  decide

/-- Required pin: a mutating spell merges with one permanent [CR#730.2]. -/
theorem okMerge :
    Instruction.check [] (.merge .this (target creature) .top) = [] := by
  decide

/-- A merge onto a group is refused: one permanent is merged with. -/
theorem badMergeOntoGroup :
    Instruction.check [] (.merge .this (allOf creature) .bottom) = [.singular] := by
  decide

/-- Required pin: mutate's target, gained with its alternative cost [CR#702.140a]. -/
theorem okAddsTarget :
    StaticSpec.check [] (.addsTarget thisSpell
      (.and [creature, .not (.hasSubtype (creatureType "Human"))])) = [] := by
  decide

/-- Required pin: offering's reduction by the sacrificed permanent's mana cost [CR#702.48a]. -/
theorem okReducedByManaCost :
    StaticSpec.check [] (reducedBySymbols thisSpell
      (manaCostOf (linkedObject .theAdditional sacrifice .this))) = [] := by
  decide

/-- A written cost shift of no symbols is refused, as it was under `run`. -/
theorem badEmptySymbols :
    StaticSpec.check [] (.costShift thisSpell (.bySymbols (.lit []) false false))
      = [.nonEmpty] := by
  decide

/-! ## The seven bodies

Each keyword's declared definition, as `cargo xtask lean macros` derives it from its
declaration, checks clean as an ability. -/

namespace Bodies
open Semantics.Macros.Declared (emerge emergeFrom offering harmonize haunt soulbond cipher mutate)

def cost : Cost := .mana [generic 2, pip .blue]

/-- Required pin: emerge [CR#702.119a]. -/
theorem okEmerge : Ability.check [] (emerge cost) = [] := by decide
/-- Required pin: emerge from [quality] [CR#702.119b]. -/
theorem okEmergeFrom : Ability.check [] (emergeFrom artifact cost) = [] := by decide
/-- Required pin: offering [CR#702.48a]. -/
theorem okOffering :
    Ability.check [] (offering (.hasSubtype (creatureType "Goblin"))) = [] := by decide
/-- Required pin: harmonize [CR#702.180a]. -/
theorem okHarmonize : Ability.check [] (harmonize cost) = [] := by decide
/-- Required pin: haunt [CR#702.55a]. -/
theorem okHaunt : Ability.check [] haunt = [] := by decide
/-- Required pin: soulbond [CR#702.95a]. -/
theorem okSoulbond : Ability.check [] soulbond = [] := by decide
/-- Required pin: cipher [CR#702.99a]. -/
theorem okCipher : Ability.check [] cipher = [] := by decide
/-- Required pin: mutate [CR#702.140a]. -/
theorem okMutate : Ability.check [] (mutate cost) = [] := by decide

end Bodies

end Semantics.Proofs.LinkedReadback
