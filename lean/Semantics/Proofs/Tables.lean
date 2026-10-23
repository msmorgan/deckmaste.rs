import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Tables

Catalog pins: not a port of an Idris family, but `decide` theorems reading a few declarations
back through the accessors the checker uses: a counter's holder, a designation's holder and scope,
the keyword a keyword's variant is, and whether a keyword action is one a permanent performs. No
catalog has a table: keyword abilities, keyword actions, counters, designations, relations and
subtypes are each their own declaration in `Semantics.Definitions`, where a duplicate is a Lean
error.
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

/-- A permanent performs these seven keyword actions, so each Definition's generated deed says a
Permanent performs it
[CR#701.14a,701.37a,701.44a,701.46a,701.50a,701.63a,701.64a]; a player performs a search, an
exile, a scry, a sacrifice, a destroy and a shuffle. -/
theorem permanentPerformedActions :
    [adapt.deed, connive.deed, endure.deed, explore.deed, fight.deed, harness.deed,
        monstrosity.deed].all (·.permanentActor) = true ∧
      [search.deed, exile.deed, scry.deed, sacrifice.deed, destroy.deed, shuffle.deed].all
        (!·.permanentActor) = true := by decide

end Semantics.Proofs.Tables
