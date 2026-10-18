import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Check.Rules

/-!
# Semantics.Proofs.LinkedReadback

A linked ability reads back a Linked Facet of a Link Source, and a named link between two
objects is a declared Relation (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-09). The
facets `objects` and `taken`, the Link Source `byDeed`, the three relation forms, `merge`,
`addsTarget`, and the mana-cost term of `CostShift.bySymbols` are pinned here; the renamed
facets keep their pins in `Proofs/LinkedChoiceReadback.lean` and `Proofs/Description.lean`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.LinkedReadback

def sacrifice : Deed := .action "sacrifice"
def exile : Deed := .action "exile"
def tap : Deed := .action "tap"
def haunting : RelationLabel := .named "haunting"
def paired : RelationLabel := .named "paired"

/-- "the sacrificed creature", sacrificed by the alternative cost (emerge) [CR#702.119a]. -/
def theSacrificed : NounPhrase := linkedObject .theAlternative sacrifice .this

/-- Required pin: "the sacrificed creature's mana value" reads back one object
[CR#702.119a]. -/
theorem okSacrificedCreaturesManaValue :
    Amount.check (DRS.ofList []) (.statOf (.stat .manaValue) theSacrificed) = [] := by
  decide

/-- Required pin: "the exiled card" reads the printed ability that exiled it (`byDeed`)
[CR#607.2a]. -/
theorem okExiledCardByDeed :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (linkedObject (.byDeed exile) exile .this) = [] := by
  decide

/-- Required pin: the objects of a deed the source never performs are refused: an ability named
by its exiling never sacrificed anything [CR#607.2a]. -/
theorem badObjectsOfADeedTheSourceNeverPerforms :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (linkedObject (.byDeed exile) sacrifice .this)
      = [.linkedFacetNamed] := by
  decide

/-- A keyword ability's own deed is performed only by that keyword's source: a kicker cost
never crewed anything. -/
theorem badKeywordDeedOfAnotherKeyword :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (linkedObject (.byKeyword (.named "kicker"))
      (.ofAbility (.named "crew")) .this) = [.linkedFacetNamed] := by
  decide

/-- The objects a deed acted on are objects, read as a noun phrase, never as an amount. -/
theorem badObjectsReadAsAnAmount :
    Amount.check (DRS.ofList []) (.paid (.objects .theAlternative sacrifice .one) .this)
      = [.linkedFacetKind] := by
  decide

/-- The objects facet is an object: a player context refuses it. -/
theorem badObjectsAsPlayer :
    NounPhrase.checkAt (some .player) (DRS.ofList []) theSacrificed = [.kindMismatch .player .object] := by
  decide

/-- "The exiled cards" read as one object refuse a singular stat read. -/
theorem badManyObjectsStat :
    Amount.check (DRS.ofList []) (.statOf (.stat .power) (linkedObjects (.byDeed exile) exile .this))
      = [.singular] := by
  decide

/-- Required pin: "if tribute wasn't paid" reads whether the offered choice was taken
[CR#702.104b]. A keyword with a paid cost names a source. -/
theorem okChoiceTaken :
    Condition.check (DRS.ofList []) (.not (choiceWasTaken (.byKeyword (.named "kicker")) .this)) = [] := by
  decide

/-- A keyword with no cost names no source for `taken`. -/
theorem badTakenUnnamed :
    Condition.check (DRS.ofList []) (choiceWasTaken (.byKeyword (.named "flying")) .this)
      = [.linkedFacetNamed] := by
  decide

/-- Required pin: "exile it haunting target creature" relates two objects [CR#702.55a]. -/
theorem okRelateHaunting :
    Action.check (DRS.ofList []) (.relate .this haunting (target creature)) = [] := by
  decide

/-- Required pin: `relate` against a declared kind mismatch is refused: a haunting's second side
is an object, never a player [CR#702.55b]. -/
theorem badRelateKindMismatch :
    Action.check (DRS.ofList []) (.relate .this haunting .you) = [.kindMismatch .object .player] := by
  decide

/-- An undeclared relation is refused. -/
theorem badRelateUndeclared :
    Action.check (DRS.ofList []) (.relate .this (.named "befriended") (target creature))
      = [.knownRelation (.named "befriended")] := by
  decide

/-- "the creature it haunts" [CR#702.55b]. -/
theorem okCounterpart :
    NounPhrase.checkAt (some .object) (DRS.ofList []) (.counterpart haunting .this) = [] := by
  decide

/-- "Paired" and "paired with this creature" are object predicates [CR#702.95b]. -/
theorem okRelated :
    (Condition.check (DRS.ofList []) (.matches thisCreature (.related paired none)),
      Condition.check (DRS.ofList []) (.exists_ (.described .bare (.and [creature, .related paired (some thisCreature)]))))
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
    Action.check (DRS.ofList []) (.merge .this (target creature) .top) = [] := by
  decide

/-- A merge onto a group is refused: one permanent is merged with. -/
theorem badMergeOntoGroup :
    Action.check (DRS.ofList []) (.merge .this (allOf creature) .bottom) = [.singular] := by
  decide

/-- Required pin: mutate's target, gained with its alternative cost [CR#702.140a]. -/
theorem okAddsTarget :
    StaticSpec.check (DRS.ofList []) (.addsTarget thisSpell
      (.and [creature, .not (.hasSubtype (creatureType "Human"))])) = [] := by
  decide

/-- Required pin: offering's reduction by the sacrificed permanent's mana cost [CR#702.48a]. -/
theorem okReducedByManaCost :
    StaticSpec.check (DRS.ofList []) (reducedBySymbols thisSpell
      (manaCostOf (linkedObject .theAdditional sacrifice .this))) = [] := by
  decide

/-- A written cost shift of no symbols is refused, as it was under `run`. -/
theorem badEmptySymbols :
    StaticSpec.check (DRS.ofList []) (.costShift thisSpell (.bySymbols (.lit []) false false))
      = [.nonEmpty] := by
  decide

/-! ## The seven bodies

Each keyword's declared definition, as `cargo xtask lean macros` derives it from its
declaration, checks clean as an ability. -/

namespace Bodies

def cost : Cost := .mana [generic 2, pip .blue]

/-- Required pin: emerge [CR#702.119a]. -/
theorem okEmerge : Ability.check (DRS.ofList []) (emerge cost) = [] := by decide
/-- Required pin: emerge from [quality] [CR#702.119b]. -/
theorem okEmergeFrom : Ability.check (DRS.ofList []) (emergeFrom artifact cost) = [] := by decide
/-- Required pin: offering [CR#702.48a]. -/
theorem okOffering :
    Ability.check (DRS.ofList []) (offering (.hasSubtype (creatureType "Goblin"))) = [] := by decide
/-- Required pin: harmonize [CR#702.180a]. -/
theorem okHarmonize : Ability.check (DRS.ofList []) (harmonize cost) = [] := by decide
/-- Required pin: haunt [CR#702.55a]. -/
theorem okHaunt : Ability.check (DRS.ofList []) haunt = [] := by decide
/-- Required pin: soulbond [CR#702.95a]. -/
theorem okSoulbond : Ability.check (DRS.ofList []) soulbond = [] := by decide
/-- Required pin: cipher [CR#702.99a]. -/
theorem okCipher : Ability.check (DRS.ofList []) cipher = [] := by decide
/-- Required pin: mutate [CR#702.140a]. -/
theorem okMutate : Ability.check (DRS.ofList []) (mutate cost) = [] := by decide

end Bodies

end Semantics.Proofs.LinkedReadback
