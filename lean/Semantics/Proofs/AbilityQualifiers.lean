import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Check.Rules

/-!
# Semantics.Proofs.AbilityQualifiers

The ability printed with a keyword is a keyword parameter, and the keyword's restriction is a
qualifier on it (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-09). `Ability.qualified`,
`KeywordParam.ability`, `Period.thisObject`, `AbilitySelection.printedAfter` and
`PlayerStat.speed` are pinned here, with the eight keyword bodies they write.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.AbilityQualifiers

def backupLabel : KeywordLabel := .named "backup"

/-- "{T}: Draw a card." -/
def tapToDraw : Ability := activated .tapSymbol (draw 1)
/-- "This creature gets +1/+0." -/
def pumpSelf : Ability := .static (Templates.modification thisCreature .power (.up 1))
/-- "When this creature dies, draw a card." -/
def drawOnDeath : Ability := when (does thisCreature die) (draw 1)
/-- "this Case is solved" -/
def caseSolved : Condition := .matches thisCase (.hasDesignation (.named "solved" (.heldBy .object)) none)

/-! ## The qualifier -/

/-- Required pin: a qualified activated ability with a guard checks: "[cost]: [effect].
Activate only if this Case is solved." [CR#702.169d] -/
theorem okQualifiedActivatedGuard :
    Ability.check (DRS.ofList []) (qualified tapToDraw (guard := some caseSolved)) = [] := by
  decide

/-- A guard on a static ability reads "as long as" [CR#702.169b]. -/
theorem okQualifiedStaticGuard :
    Ability.check (DRS.ofList []) (qualified pumpSelf (guard := some caseSolved)) = [] := by
  decide

/-- "Activate only once": a limit over the object's own existence [CR#602.5b,702.177a]. -/
theorem okOncePerObject :
    Ability.check (DRS.ofList []) (qualified tapToDraw (limit := some oncePerObject)) = [] := by
  decide

/-- The timing, limit and zone of a forecast-shaped qualifier [CR#702.57a,702.57b]. -/
theorem okFromHandDuringUpkeep :
    Ability.check (DRS.ofList []) (qualified tapToDraw (some (duringPart .upkeep (some (ownerOf .this))))
      (some oncePerTurn) none (some .hand)) = [] := by
  decide

/-- A qualifier leaves the wrapped ability's category [CR#113.10a]. -/
theorem qualifiedCategory :
    (qualified drawOnDeath (guard := some caseSolved)).category = some .triggered := by
  decide

/-! ## Speed -/

/-- Required pin: "your speed is 4" [CR#702.178a,702.179a]. -/
theorem okYourSpeedIsFour :
    Condition.check (DRS.ofList []) (.compareAmt (speedOf .you) .eq 4) = [] := by
  decide

/-- Speed is a player's: an object's speed is refused. -/
theorem badSpeedOfAnObject :
    (Condition.check (DRS.ofList []) (.compareAmt (speedOf thisCreature) .eq 4)).isEmpty = false := by
  decide

/-! ## Printed-position selection -/

def grantBelowBackup : Ability :=
  .static (abilityGrantSelected (target isCreature) thisCreature (.printedAfter backupLabel))

/-- Required pin: "the abilities printed below backup" checks as a grant [CR#702.165a]. -/
theorem okPrintedAfterGrant : Ability.check (DRS.ofList []) grantBelowBackup = [] := by decide

/-- Required pin: `printedAfter` of a keyword the card does not print is refused as a reference
[CR#702.165c]. -/
theorem badPrintedAfterAbsentKeyword :
    printedAfterLaws [grantBelowBackup] = [.printedAfterKeyword backupLabel] := by
  decide

/-- The same selection on a card printing the keyword is admitted. -/
theorem okPrintedAfterPresentKeyword :
    printedAfterLaws [.keyword backupLabel [.number 1] [], grantBelowBackup] = [] := by
  decide

/-! ## The eight bodies

Each keyword's Definition (`Semantics.Definitions`) checks clean as an ability; the printed ability is the keyword's parameter. -/

namespace Bodies

/-- Required pin: forecast [CR#702.57a,702.57b]. -/
theorem okForecast : Ability.check (DRS.ofList []) (forecast tapToDraw) = [] := by decide
/-- Required pin: boast [CR#702.142a]. -/
theorem okBoast : Ability.check (DRS.ofList []) (boast tapToDraw) = [] := by decide
/-- Required pin: exhaust [CR#702.177a]. -/
theorem okExhaust : Ability.check (DRS.ofList []) (exhaust tapToDraw) = [] := by decide
/-- Required pin: solved over a static ability [CR#702.169b]. -/
theorem okSolvedStatic : Ability.check (DRS.ofList []) (solved pumpSelf) = [] := by decide
/-- Required pin: solved over a triggered ability [CR#702.169c]. -/
theorem okSolvedTriggered : Ability.check (DRS.ofList []) (solved drawOnDeath) = [] := by decide
/-- Required pin: solved over an activated ability [CR#702.169d]. -/
theorem okSolvedActivated : Ability.check (DRS.ofList []) (solved tapToDraw) = [] := by decide
/-- Required pin: ∞ [CR#702.186b]. -/
theorem okInfinity : Ability.check (DRS.ofList []) (infinity tapToDraw) = [] := by decide
/-- Required pin: max speed [CR#702.178a]. -/
theorem okMaxSpeed : Ability.check (DRS.ofList []) (maxSpeed tapToDraw) = [] := by decide
/-- Required pin: power-up [CR#702.193a]. -/
theorem okPowerUp : Ability.check (DRS.ofList []) (powerUp tapToDraw) = [] := by decide
/-- Required pin: backup [CR#702.165a]. -/
theorem okBackup : Ability.check (DRS.ofList []) (backup 1) = [] := by decide

end Bodies

end Semantics.Proofs.AbilityQualifiers
