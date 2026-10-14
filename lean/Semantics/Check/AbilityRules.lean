import Semantics.Check.Abilities

/-!
# Semantics.Check.AbilityRules

Minimal core ability-level rule checks.
-/

namespace Semantics

def Ability.checkAll (drs : DRS) (abs : List Ability) : List Refusal :=
  (stepAbilityList 64 drs abs).2

def CopyExcept.check (drs : DRS) (exception : CopyExcept) : List Refusal :=
  (stepCopyExceptWith (stepOps 64) drs exception).2

def CopyExcept.checkAll (drs : DRS) (exceptions : List CopyExcept) : List Refusal :=
  (stepCopyExceptListWith (stepOps 64) drs exceptions).2

def ZoneExpr.check (drs : DRS) (z : ZoneExpr) : List Refusal :=
  (stepZoneExprWith (stepOps 64) drs z).2

def PlayPayment.check (drs : DRS) (p : PlayPayment) : List Refusal :=
  (stepPlayPaymentWith (stepOps 64) drs p).2

def ProducedMana.check (drs : DRS) (p : ProducedMana) : List Refusal :=
  (stepProducedManaWith (stepOps 64) drs p).2

def Duration.check (drs : DRS) (d : Duration) : List Refusal :=
  (stepDurationWith (stepOps 64) drs d).2

def TurnRef.check (drs : DRS) (turn : TurnRef) : List Refusal :=
  (stepTurnRefWith (stepOps 64) drs turn).2

def CharacteristicBundle.check (drs : DRS) (bundle : CharacteristicBundle) : List Refusal :=
  (stepCharacteristicBundleWith (stepOps 64) drs bundle).2

def TextChange.check (drs : DRS) (change : TextChange) : List Refusal :=
  (stepTextChangeWith (stepOps 64) drs change).2

def Timing.check (drs : DRS) (timing : Timing) : List Refusal :=
  (stepTimingWith (stepOps 64) drs timing).2

def CounterKind.check (kind : CounterKind) : List Refusal := refuse kind.known .knownCounter

def Ballot.check (drs : DRS) (ballot : Ballot) : List Refusal :=
  (stepBallotWith (stepOps 64) drs ballot).2

def sortedDomainCheck (drs : DRS) (sort : ChoiceSort)
    (domain : Option ChoiceDomain) : List Refusal :=
  (sortedDomainCheckWith (stepOps 64) drs sort domain).2

end Semantics
