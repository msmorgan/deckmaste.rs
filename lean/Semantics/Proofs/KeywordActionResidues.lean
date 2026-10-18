import Semantics.Definitions
import Semantics.Check.Card

open Semantics Semantics.Templates Semantics.Definitions

/-!
# Semantics.Proofs.KeywordActionResidues

The shapes the last bodyless keyword actions needed (`semantics-v2-keyword-action-residues`): a
part bounded by a summed stat, a run of library cards, a double-faced token, a face-down entry, a
cost whose generic mana tapping can pay, a rules procedure carried out during resolution, and a
named emblem. Each keyword action's bench macro checks clean, and each shape has a refusal pin.
-/

namespace Semantics.Proofs.KeywordActionResidues

/-- "Collect evidence 3" [CR#701.59a]. -/
theorem okCollectEvidence : Action.check (DRS.ofList []) (Actor.collectEvidence (.lit 3)) = [] := by decide

/-- A total's bound is an amount like any other: an unresolved "its power" is refused. -/
theorem badWithTotalUnresolvedBound :
    Action.check (DRS.ofList []) (move (Actor.withTotal (.stat .manaValue) .atLeast
      (powerOf (that (.type .creature))) (allOf (.and [.isCard, .in_ (graveyardOf .you)])))
      exileZone) =
      [.anaphor (.word (.type .creature)) .one 0] := by decide

/-- "Discover 5" [CR#701.57a]: "that card" reads the card that stopped the run, and "the
remaining exiled cards" the rest of the run. -/
theorem okDiscover : Action.check (DRS.ofList []) (Actor.discover (.lit 5)) = [] := by decide

/-- A run comes from a pile, and a library pile is a player's: a creature has none, so "its
library" names no pile to run. -/
theorem badLibraryRunOfAnObject :
    Action.check (DRS.ofList []) (move (.run .top isLand (libraryOf thisCreature)) exileZone) =
      [.kindMismatch .player .object] := by decide

/-- "Incubate 3" [CR#701.53a,701.53b]. -/
theorem okIncubate : Action.check (DRS.ofList []) (Actor.incubate (.lit 3)) = [] := by decide

/-- Each face of a double-faced token is a characteristics set, written once per value. -/
theorem badDoubleFacedRepeatedType :
    Action.check (DRS.ofList []) (.createObject (.lit 1) (.token (.doubleFaced
      { characteristics := { types := [.artifact, .artifact] } }
      { characteristics := { types := [.creature] } }) []) true none) = [.distinct] := by decide

/-- "Waterbend {4}" [CR#701.67a]. -/
theorem okWaterbend : Action.check (DRS.ofList []) (Actor.waterbend (.mana [generic 4])) = [] := by decide

/-- What tapping pays with is a permanent, not a player: a substitute whose action names
players. -/
theorem badSubstituteWithPlayers :
    Action.check (DRS.ofList []) (.pay (.withSubstitute (.mana [generic 4])
      (substitute (tap (a .anyPlayer)) [.generic])) .once .paid none none) =
      [.deedFits, .kindMismatch .object .player] := by decide

/-- Convoke [CR#702.51a]: "mana of that creature's color" reads back the creature the action
tapped. -/
theorem okConvokeColorReadback :
    StaticSpec.check (DRS.ofList []) (.paySubstitute thisSpell
      (substitute (tap (a (.and [isUntapped, isCreature, .hasPossessor .controller .you])))
        [.generic, .ofColor (.colorOf (that (.type .creature)))])) = [] := by decide

/-- The mana a substitute pays reads only what its action acted on: after exiling a card,
"that creature's color" has no antecedent, even with a creature in scope outside. -/
theorem badSubstitutePaysWithoutAntecedent :
    StaticSpec.check (DRS.ofList []) (.paySubstitute thisSpell
      (substitute (exile (a (.and [.isCard, .in_ (graveyardOf .you)])))
        [.ofColor (.colorOf (that (.type .creature)))])) =
      [.anaphor (.word (.type .creature)) .one 0] := by decide

/-- Crew 3's cost [CR#702.122a]: "tap any number of other untapped creatures you control with
total power 3 or greater". -/
theorem okCrewCost :
    Cost.check (DRS.ofList []) (.perform (tap (Actor.withTotal (.stat .power) .atLeast (.lit 3)
      (allOf (.and [isUntapped, isCreature, .hasPossessor .controller .you, .otherThan .this]))))) =
      [] := by decide

/-- "Manifest the top card of your library" [CR#701.40a]. -/
theorem okManifest :
    Action.check (DRS.ofList []) (Actor.manifest (.slice .top (.lit 1) (libraryOf .you))) = [] := by decide

/-- "Cloak the top card of your library" [CR#701.58a]. -/
theorem okCloak :
    Action.check (DRS.ofList []) (Actor.cloak (.slice .top (.lit 1) (libraryOf .you))) = [] := by decide

/-- "Manifest dread" [CR#701.62a]: the manifested card is a part of the look, and the rest the
other card. -/
theorem okManifestDread : Action.check (DRS.ofList []) Actor.manifestDread = [] := by decide

/-- "Support 2" [CR#701.41a]. -/
theorem okSupport : Action.check (DRS.ofList []) (Actor.support (.lit 2)) = [] := by decide

/-- "Cast target instant or sorcery card" [CR#701.5a], "play target land card without paying"
[CR#701.18a] and "activate target activated ability" [CR#701.2a]: the procedures as
instructions. -/
theorem okCastProcedure :
    Action.check (DRS.ofList []) (.enact (.action "cast") (.procedure .cast (target (.and [isInstantOrSorcery, .isCard])) .itsOwnCost none none none) none none) = [] := by
  decide
theorem okPlayProcedure :
    Action.check (DRS.ofList []) (.enact (.action "play") (.procedure .play (target (.and [isLand, .isCard])) .withoutPaying none none none) none none) = [] := by decide
theorem okActivateProcedure :
    Action.check (DRS.ofList []) (.enact (.action "activate") (.procedure .activate (target (.abilityHead .anyActivated)) .itsOwnCost none none none) none none) = [] := by
  decide

/-- A procedure is carried out on an object: a player is not cast. -/
theorem badProcedureOnAPlayer :
    Action.check (DRS.ofList []) (.procedure .cast .you .itsOwnCost none none none) =
      [.kindMismatch .object .player] := by decide

/-- "Double the power of target creature until end of turn" [CR#701.10b]. -/
theorem okDoublePower :
    Action.check (DRS.ofList []) (doublePower (target isCreature) (some untilEndOfTurn)) = [] := by decide

/-- "Triple target creature's power and toughness until end of turn" [CR#701.11b]. -/
theorem okTriplePowerAndToughness :
    Action.check (DRS.ofList []) (triplePowerAndToughness (target isCreature) (some untilEndOfTurn)) = [] := by
  decide

/-- "Double your life total" [CR#701.10d]. -/
theorem okDoubleLifeTotal : Action.check (DRS.ofList []) doubleLifeTotal = [] := by decide

/-- A named emblem still has abilities [CR#114.1]. -/
theorem badNamedEmblemWithoutAbilities :
    Action.check (DRS.ofList []) (.createObject (.lit 1) (.emblem (some "The Ring") []) true none) = [.nonEmpty] := by
  decide

end Semantics.Proofs.KeywordActionResidues
