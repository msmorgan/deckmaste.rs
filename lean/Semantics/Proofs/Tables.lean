import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Proofs.Tables

Table-integrity pins: not a port of an Idris family, but `decide` theorems asserting what the
checker's label tables assume of themselves — that `actFacts` and `abilityDeedFacts` each name
every row once. A duplicate label would let one row shadow another silently, so each table's
existing `distinct*` validator is asserted here rather than left unchecked. Keyword abilities,
counters, designations, relations and subtypes have no table: each is its own declaration in
`Semantics.Definitions`, where a duplicate is a Lean error.

`keywordActionLabels` (`Check.Facts`) and `actFacts` (`Check.Words`) are two hand-written lists
of the keyword actions, so two theorems here pin them against each other in both directions: a
row added to one list only is caught. They stay until the declarations themselves supply the
label list.

The rest read a few declarations and rows back through the accessors the checker uses: a
counter's holder, a designation's holder and scope, the keyword a keyword's variant is.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Tables

theorem actLabelsDistinct : distinctActLabels actFacts = true := by decide

/-- Every keyword action [CR#701.1] in `keywordActionLabels` has a row in the hand-written
`actFacts` table: a label added without one is caught here, not left to a lookup that silently
finds nothing. -/
theorem everyDeclaredActionHasFacts :
    keywordActionLabels.all (fun l => (actFacts.find? (·.1 == l)).isSome) = true := by decide

/-- And every `actFacts` row names a label in `keywordActionLabels`, so a row left behind when a
keyword action is retired from one list is caught too. -/
theorem everyActRowIsDeclared :
    actFacts.all (fun row => keywordActionLabels.contains row.1) = true := by decide

theorem abilityDeedLabelsDistinct : distinctAbilityDeedLabels abilityDeedFacts = true := by decide

/-! ## Declarations and rows read back -/

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

end Semantics.Proofs.Tables
