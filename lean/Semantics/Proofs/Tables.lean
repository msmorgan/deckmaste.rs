import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Tables

Catalog pins: not a port of an Idris family, but `decide` theorems reading a few declarations
back through the accessors the checker uses: a counter's holder, a designation's holder and scope,
the keyword a keyword's variant is, whether a keyword action is one a permanent performs, and a
card type's permanence, stats and Card Class. No catalog has a table: card types, keyword
abilities, keyword actions, counters, designations, relations and subtypes are each their own
declaration in `Semantics.Definitions`, where a duplicate is a Lean error.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Tables

/-! ## Declarations read back -/

/-- A counter's holder is its declaration's: poison and rad counters are a player's
[CR#122.1f,122.1i], as the declarations say energy and experience counters are; a +1/+1, keyword
or charge counter is an object's [CR#122.1a,122.1b]. `CounterKind.scope` reads the holder. -/
theorem counterHoldersAreDeclared :
    ([poison, radCounter, energy, experience] : List CounterKind).map CounterKind.holder
        = [.player, .player, .player, .player] ∧
      ([chargeCounter, p1p1Counter, flyingCounter] : List CounterKind).map CounterKind.scope
        = [.object, .object, .object] := by decide

/-- A designation's scope is its declaration's: a player is the monarch [CR#725.1] and has the
city's blessing [CR#702.131c], a card is a commander [CR#903.3], and day and night, the one
declaration that lists two members, are each the game's [CR#731.1]. -/
theorem designationScopesAreDeclared :
    (monarch.scope, citysBlessing.scope, commander.scope, day.scope, night.scope)
      = (.heldBy .player, .heldBy .player, .heldByCard, .heldByGame, .heldByGame) := by decide

/-- Only the monarch [CR#725.3] and the initiative [CR#726.3] are held by at most one player at a
time; the city's blessing is each player's own [CR#702.131c]. -/
theorem designationSoleHoldersAreDeclared :
    (monarch.sole, initiative.sole, citysBlessing.sole) = (true, true, false) := by decide

/-- A designation's holder follows from its scope: a permanent is goaded [CR#701.15b], a player
is the monarch [CR#725.1], a card is a commander, and day and night are the game's
[CR#731.1]. -/
theorem designationHoldersFollowFromScope :
    (goaded.holder, goaded.possessorOk) = (some .object, true) ∧
      (monarch.holder, monarch.possessorOk) = (some .player, false) ∧
      (commander.heldByItsCard, commander.holder) = (true, some .object) ∧
      (day.gameWide, day.holder, designationHolderOk day) = (true, none, false) := by decide

/-- Hexproof bare and "hexproof from [quality]" are one keyword [CR#702.11d], as cycling and
typecycling are [CR#702.29f]: each variant's Definition generates the label its keyword's does. -/
theorem hexproofAndCyclingVariantsShareTheirKeyword :
    hexproofFrom.label = hexproof.label ∧ typecycling.label = cycling.label := by decide

/-- A permanent is instructed to perform these four keyword actions, so each Definition's
generated deed says a Permanent performs it [CR#701.14a,701.44a,701.50a,701.63a]; adapt, harness
and monstrosity are instructions about "this permanent" that the ability's controller follows
[CR#701.46a,701.64a,701.37a,608.2c], and a player performs a search, an exile, a scry, a
sacrifice, a destroy and a shuffle. -/
theorem permanentPerformedActions :
    [connive.deed, endure.deed, explore.deed, fight.deed].all (·.permanentActor) = true ∧
      [adapt.deed, harness.deed, monstrosity.deed, search.deed, exile.deed, scry.deed,
        sacrifice.deed, destroy.deed, shuffle.deed].all (!·.permanentActor) = true := by decide

/-! ## Card types read back -/

/-- The six permanent types are artifact, battle, creature, enchantment, land and planeswalker
[CR#110.4]; instant, sorcery, kindred and dungeon are not [CR#110.4,309.2c]. -/
theorem permanentTypesAreDeclared :
    cardTypes.filter (·.permanent) =
        [.creature, .artifact, .land, .enchantment, .planeswalker, .battle] ∧
      cardTypes.filter (!·.permanent) = [.instant, .sorcery, .kindred, .dungeon] := by decide

/-- A card type's stats are its declaration's: power and toughness are a creature's
[CR#302.4], loyalty a planeswalker's [CR#306.5] and defense a battle's [CR#310.4]; no other card
type has a stat of its own. -/
theorem cardTypeStatsAreDeclared :
    (CardType.creature.stats, CardType.planeswalker.stats, CardType.battle.stats) =
        ([.power, .toughness], [.loyalty], [.defense]) ∧
      (cardTypes.filter (·.stats.isEmpty)).length = 7 := by decide

/-- Card Class read off the declared fields [CR#110.4,110.4a]: an artifact, battle, creature,
enchantment, land or planeswalker card is a permanent card; an instant or sorcery card is an
instant or sorcery card; a kindred card reads by its other card type [CR#308.1]; and kindred
alone, as a dungeon, gives no Card Class [CR#309.2c]. -/
theorem cardClassIsDeclared :
    ([.artifact, .battle, .creature, .enchantment, .land, .planeswalker] : List CardType).map
        (fun t => cardClassOf [t]) = List.replicate 6 (some .permanentCard) ∧
      ([[.instant], [.sorcery], [.kindred, .instant], [.kindred, .sorcery]] :
          List (List CardType)).map cardClassOf = List.replicate 4 (some .instantOrSorceryCard) ∧
      cardClassOf [.kindred, .creature] = some .permanentCard ∧
      cardClassOf [.kindred] = none ∧ cardClassOf [.dungeon] = none := by decide

end Semantics.Proofs.Tables
