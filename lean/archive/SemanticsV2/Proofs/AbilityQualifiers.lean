import Semantics.Macros
import Semantics.Check.Card
import Semantics.Check.Rules
import Semantics.Macros.Declared

/-!
# Semantics.Proofs.AbilityQualifiers

The ability printed with a keyword is a keyword parameter, and the keyword's restriction is a
qualifier on it (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-09). `Ability.qualified`,
`KeywordParam.ability`, `Period.thisObject`, `AbilitySelection.printedAfter` and
`PlayerStat.speed` are pinned here, with the eight keyword bodies they write.
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.AbilityQualifiers

def backupLabel : KeywordLabel := .named "backup"

/-- "{T}: Draw a card." -/
def tapToDraw : Ability := activated .tapSymbol (draw (.lit 1))
/-- "This creature gets +1/+0." -/
def pumpSelf : Ability := .static (.modification thisCreature .power (.up (.lit 1)))
/-- "When this creature dies, draw a card." -/
def drawOnDeath : Ability := when (Primitives.GameEvent.dies thisCreature) (draw (.lit 1))
/-- "this Case is solved" -/
def caseSolved : Condition := .matches thisCase (.hasDesignation (.named "solved") none)

/-! ## The qualifier -/

/-- Required pin: a qualified activated ability with a guard checks: "[cost]: [effect].
Activate only if this Case is solved." [CR#702.169d] -/
theorem okQualifiedActivatedGuard :
    Ability.check [] (qualified tapToDraw (guard := some caseSolved)) = [] := by
  decide

/-- A guard on a static ability reads "as long as" [CR#702.169b]. -/
theorem okQualifiedStaticGuard :
    Ability.check [] (qualified pumpSelf (guard := some caseSolved)) = [] := by
  decide

/-- Required pin: a usage limit on a static ability is refused: a limit restricts activation
[CR#602.5b]. -/
theorem badLimitOnStatic :
    Ability.check [] (qualified pumpSelf (limit := some oncePerTurn)) = [.qualifiedLimit] := by
  decide

/-- A usage limit as a qualifier is refused on a triggered ability, whose own limit is written on
its header. -/
theorem badLimitOnTriggered :
    Ability.check [] (qualified drawOnDeath (limit := some oncePerTurn))
      = [.qualifiedLimit] := by
  decide

/-- "Activate only once": a limit over the object's own existence [CR#602.5b,702.177a]. -/
theorem okOncePerObject :
    Ability.check [] (qualified tapToDraw (limit := some oncePerObject)) = [] := by
  decide

/-- The timing, limit and zone of a forecast-shaped qualifier [CR#702.57a,702.57b]. -/
theorem okFromHandDuringUpkeep :
    Ability.check [] (qualified tapToDraw (some (duringPart .upkeep (some (ownerOf .this))))
      (some oncePerTurn) none (some .hand)) = [] := by
  decide

/-- A qualifier leaves the wrapped ability's category [CR#113.10a]. -/
theorem qualifiedCategory :
    (qualified drawOnDeath (guard := some caseSolved)).category = some .triggered := by
  decide

/-! ## Speed -/

/-- Required pin: "your speed is 4" [CR#702.178a,702.179a]. -/
theorem okYourSpeedIsFour :
    Condition.check [] (.compareAmt (speedOf .you) .eq (.lit 4)) = [] := by
  decide

/-- Speed is a player's: an object's speed is refused. -/
theorem badSpeedOfAnObject :
    (Condition.check [] (.compareAmt (speedOf thisCreature) .eq (.lit 4))).isEmpty = false := by
  decide

/-! ## Printed-position selection -/

def grantBelowBackup : Ability :=
  .static (abilityGrantSelected (target creature) thisCreature (.printedAfter backupLabel))

/-- Required pin: "the abilities printed below backup" checks as a grant [CR#702.165a]. -/
theorem okPrintedAfterGrant : Ability.check [] grantBelowBackup = [] := by decide

/-- Required pin: `printedAfter` of a keyword the card does not print is refused as a reference
[CR#702.165c]. -/
theorem badPrintedAfterAbsentKeyword :
    printedAfterLaws [grantBelowBackup] = [.printedAfterKeyword backupLabel] := by
  decide

/-- The same selection on a card printing the keyword is admitted. -/
theorem okPrintedAfterPresentKeyword :
    printedAfterLaws [.keyword backupLabel [.number (.lit 1)] [], grantBelowBackup] = [] := by
  decide

/-- An unknown keyword is refused as a keyword term. -/
theorem badPrintedAfterUnknownKeyword :
    Ability.check [] (.static (abilityGrantSelected (target creature) thisCreature
      (.printedAfter (.named "bakcup")))) = [.knownKeywordTerm] := by
  decide

/-! ## The eight bodies

Each keyword's declared definition, as `cargo xtask lean macros` derives it from its
declaration, checks clean as an ability; the printed ability is the keyword's parameter. -/

namespace Bodies
open Semantics.Macros.Declared (forecast boast exhaust solved infinity maxSpeed powerUp backup)

/-- Required pin: forecast [CR#702.57a,702.57b]. -/
theorem okForecast : Ability.check [] (forecast tapToDraw) = [] := by decide
/-- Forecast of a static ability is refused: its definition qualifies an activated ability, and
a static ability takes no limit. -/
theorem badForecastOfStatic :
    Ability.check [] (forecast pumpSelf)
      = [.qualifiedLimit, .keywordBodyFits (.named "forecast")] := by
  decide
/-- Required pin: boast [CR#702.142a]. -/
theorem okBoast : Ability.check [] (boast tapToDraw) = [] := by decide
/-- Required pin: exhaust [CR#702.177a]. -/
theorem okExhaust : Ability.check [] (exhaust tapToDraw) = [] := by decide
/-- Required pin: solved over a static ability [CR#702.169b]. -/
theorem okSolvedStatic : Ability.check [] (solved pumpSelf) = [] := by decide
/-- Required pin: solved over a triggered ability [CR#702.169c]. -/
theorem okSolvedTriggered : Ability.check [] (solved drawOnDeath) = [] := by decide
/-- Required pin: solved over an activated ability [CR#702.169d]. -/
theorem okSolvedActivated : Ability.check [] (solved tapToDraw) = [] := by decide
/-- Required pin: ∞ [CR#702.186b]. -/
theorem okInfinity : Ability.check [] (infinity tapToDraw) = [] := by decide
/-- Required pin: max speed [CR#702.178a]. -/
theorem okMaxSpeed : Ability.check [] (maxSpeed tapToDraw) = [] := by decide
/-- Required pin: power-up [CR#702.193a]. -/
theorem okPowerUp : Ability.check [] (powerUp tapToDraw) = [] := by decide
/-- Required pin: backup [CR#702.165a]. -/
theorem okBackup : Ability.check [] (backup (.lit 1)) = [] := by decide

end Bodies

end Semantics.Proofs.AbilityQualifiers
