import Semantics.Templates
import Semantics.Definitions.Counters
import Semantics.Definitions.Designations
import Semantics.Definitions.Relations
import Semantics.Definitions.Subtypes

/-!
# Semantics.Definitions.Phrasings

The Templates that name a card type or a catalog entry (a subtype, a designation): "this
creature", "any target", "historic", "outlaw", "your party", "your commander", the card-type
noun words and the creature tokens. They are `semantic_template`s, transparent phrasings as every
Template is, declared in the Definitions layer so that `Semantics.Templates` names no catalog
item: the layering is Primitives → Templates → Definitions → Cards. A phrasing helper that needs
a composite Definition lives after it, in `Semantics.Definitions.Actions` or
`Semantics.Definitions.Keywords`.
-/

namespace Semantics.Definitions

open Semantics.Templates

/-! ## Pronouns -/

/-- "that player or that permanent's controller": the player half of a "target player or
planeswalker" split, or the controller of the permanent half. -/
semantic_template splitOverPlaneswalker : NounPhrase :=
  eitherOf (.pro (.unionHalf .player) .one)
    (.possessorOf .controller (.pro (.unionHalf (.type .planeswalker)) .one))

/-! ### Types, kinds and roles -/

semantic_template isCreature : Predicate := .hasType .creature
semantic_template isArtifact : Predicate := .hasType .artifact
semantic_template isEnchantment : Predicate := .hasType .enchantment
semantic_template isLand : Predicate := .hasType .land
semantic_template isInstant : Predicate := .hasType .instant
semantic_template isSorcery : Predicate := .hasType .sorcery
semantic_template isPlaneswalker : Predicate := .hasType .planeswalker
semantic_template isBattle : Predicate := .hasType .battle
semantic_template isKindred : Predicate := .hasType .kindred
semantic_template isInstantOrSorcery : Predicate := or [isInstant, isSorcery]
/-- "has a permanent type": an artifact, battle, creature, enchantment, land, or planeswalker
[CR#110.4]; read off the types, so it holds of a card or a token. -/
semantic_template isPermanentType : Predicate :=
  .or [ .hasType .artifact, .hasType .battle, .hasType .creature, .hasType .enchantment,
        .hasType .land, .hasType .planeswalker ]
/-- "permanent card": a card that could be put onto the battlefield, one with an artifact,
battle, creature, enchantment, land, or planeswalker type [CR#110.4a]; a reading of the card's
types, wherever it is. -/
semantic_template isPermanentCard : Predicate := .and [.isCard, isPermanentType]
/-- An outlaw: an Assassin, Mercenary, Pirate, Rogue, and/or Warlock [CR#700.12]. -/
semantic_template isOutlaw : Predicate :=
  .coord .andOr
    [ .hasSubtype assassin, .hasSubtype mercenary, .hasSubtype pirate, .hasSubtype rogue,
      .hasSubtype warlock ]
/-- "historic": an artifact, legendary, or Saga [CR#700.6]. -/
semantic_template isHistoric : Predicate :=
  .or [.hasType .artifact, .hasSupertype .legendary, .hasSubtype saga]
/-- "any target" [CR#115.4]. -/
semantic_template isAnyTarget : Predicate :=
  .or [.hasType .creature, .hasType .planeswalker, .hasType .battle, .isPlayer]
/-- "any other target" -/
semantic_template isAnyOtherTarget : Predicate := and [isAnyTarget, .other]

/-! ### Other predicates -/

/-- The four party roles, in rule order [CR#700.8]. -/
semantic_template partyRoles : List Predicate :=
  [.hasSubtype cleric, .hasSubtype rogue, .hasSubtype warrior, .hasSubtype wizard]
/-- An Army the performer controls. -/
semantic_template army : Predicate :=
  .and [.hasSubtype Subtype.army, isCreature, actorControls]

/-! ## Nouns -/

semantic_template thisCreature : NounPhrase := .headed (.type .creature) .this
semantic_template thisArtifact : NounPhrase := .headed (.type .artifact) .this
semantic_template thisLand : NounPhrase := .headed (.type .land) .this
semantic_template thisRoom : NounPhrase := .headed (.subtype room) .this
semantic_template thisPlaneswalker : NounPhrase := .headed (.type .planeswalker) .this
semantic_template thisEnchantment : NounPhrase := .headed (.type .enchantment) .this
semantic_template thisAura : NounPhrase := .headed (.subtype aura) .this
semantic_template thisEquipment : NounPhrase := .headed (.subtype equipment) .this
semantic_template thisSiege : NounPhrase := .headed (.subtype siege) .this
semantic_template thisVehicle : NounPhrase := .headed (.subtype vehicle) .this
semantic_template thisSpacecraft : NounPhrase := .headed (.subtype spacecraft) .this
semantic_template thisSaga : NounPhrase := .headed (.subtype saga) .this
semantic_template thisFortification : NounPhrase := .headed (.subtype fortification) .this
semantic_template thisCase : NounPhrase := .headed (.subtype case) .this
semantic_template thisClass : NounPhrase := .headed (.subtype class_) .this
/-- "your commander" -/
semantic_template yourCommander : NounPhrase := .designated commander .you
/-- "the monarch": only one player can be the monarch at a time [CR#725.3]. -/
semantic_template theMonarch : NounPhrase := the (.and [isPlayer, .hasDesignation monarch none])
/-- "the player who has the initiative": only one player can have it at a time [CR#726.3]. -/
semantic_template theInitiativeHolder : NounPhrase :=
  the (.and [isPlayer, .hasDesignation initiative none])
/-- "[a player]'s party" [CR#700.8]. -/
semantic_template partyOf (player : NounPhrase) : NounPhrase :=
  .oneEachOf partyRoles (allOf (.and [isCreature, .hasPossessor .controller player]))
/-- "your party" -/
semantic_template party : NounPhrase := partyOf .you
/-- "the number of creatures in <player>'s party" [CR#700.8a]. -/
semantic_template partySizeOf (player : NounPhrase) : Amount := .countOf (partyOf player)
semantic_template partySize : Amount := partySizeOf .you
/-- "<player> has a full party": four creatures in that party [CR#700.8c]. -/
semantic_template fullPartyOf (player : NounPhrase) : Condition :=
  .compareAmt (partySizeOf player) .eq 4
semantic_template fullParty : Condition := fullPartyOf .you

/-! ### Noun words

The bare nouns a pronoun or an attached host reads: "that creature", "those cards",
"enchanted creature".
-/

semantic_template creature : NounWord := .type .creature
semantic_template artifact : NounWord := .type .artifact
semantic_template enchantment : NounWord := .type .enchantment
semantic_template land : NounWord := .type .land
semantic_template planeswalker : NounWord := .type .planeswalker
semantic_template battle : NounWord := .type .battle

/-! ## Actions -/

/-- A 2/2 face-down creature with no text, no name, no subtypes and no mana cost [CR#708.2a]. -/
semantic_template faceDownCreature : FaceDown :=
  ⟨some { types := [.creature], power := some 2, toughness := some 2 }⟩
/-- A 2/2 face-down creature with ward {2}, no name, no subtypes and no mana cost
[CR#701.58a,702.168a]. Ward is written bare by its label: cloak, a keyword action, reads it, and
keyword actions are declared below the keyword abilities. -/
semantic_template faceDownWardedCreature : FaceDown :=
  ⟨some { types := [.creature], text := [.keyword (.named "ward") [.cost (.mana [generic 2])] []],
          power := some 2, toughness := some 2 }⟩
/-- A token's characteristics from the parts a creature token names, with the abilities it is
created with ("… with flying") [CR#111.3]. -/
semantic_template creatureTokenOf (power toughness : Amount) (colors : List Color)
    (subtypes : List Subtype)
    (abilities : List Ability := []) : CharacteristicBundle :=
  { characteristics :=
      { colors, types := [.creature], subtypes, text := abilities, power := power,
        toughness := toughness } }
semantic_template creatureToken (power toughness : Nat) (colors : List Color)
    (subtypes : List Subtype) (abilities : List Ability := []) : CharacteristicBundle :=
  creatureTokenOf (.lit power) (.lit toughness) colors subtypes abilities
semantic_template artifactCreatureToken (power toughness : Nat) (colors : List Color)
    (subtypes : List Subtype) (abilities : List Ability := []) : CharacteristicBundle :=
  { characteristics :=
      { colors, types := [.artifact, .creature], subtypes, text := abilities,
        power := some (.lit power), toughness := some (.lit toughness) } }

end Semantics.Definitions
