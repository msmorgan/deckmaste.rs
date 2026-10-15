import Semantics.Abilities
import Semantics.Check.Refusal
import Semantics.Check.Events
import Semantics.Check.Keywords

namespace Semantics

private def localRefuse (ok : Bool) (refusal : Refusal) : List Refusal :=
  if ok then [] else [refusal]

def Amount.nonZero (amount : Amount) (fuel : Nat := 32) : Bool :=
  match fuel, amount with
  | n + 1, .inCaller body | n + 1, .upTo body => body.nonZero n
  | _ + 1, .lit n => n != 0
  | _, _ => true
termination_by structural fuel

def Quantity.nonZero (quantity : Quantity) (fuel : Nat := 32) : Bool :=
  match fuel, quantity with
  | n + 1, .inCaller body => body.nonZero n
  | _ + 1, .range _ (some 0) | _ + 1, .worth 0 => false
  | n + 1, .exactlyOf amount | n + 1, .upToOf amount => amount.nonZero n
  | _, _ => true
termination_by structural fuel

def Quantity.literal (quantity : Quantity) (fuel : Nat := 32) : Bool :=
  match fuel, quantity with
  | n + 1, .inCaller body => body.literal n
  | _ + 1, .range _ _ => true
  | _, _ => false
termination_by structural fuel

def RollWatch.localChecks : RollWatch → List Refusal
  | .resultIn quantity => localRefuse quantity.nonZero .nonZeroQ ++ localRefuse quantity.literal .quantLiteral
  | _ => []

def DieSides.localChecks (drs : DRS) : DieSides → List Refusal
  | .sides n => localRefuse (n != 0) .nonZeroQ
  | .thoseDice =>
    let n := countOutcomes .diceRolled drs
    localRefuse (n == 1) (.outcomeInScope .diceRolled n)
  | .anyDie => []

def PilePlacement.localChecks : PilePlacement → List Refusal
  | .nthFromEnd ordinal => localRefuse ordinal.ok .ordinalNonZero
  | _ => []

def ProducedMana.localChecks (drs : DRS) : ProducedMana → List Refusal
  | .runs rs => localRefuse (producedRunsWritten rs) .producedRuns
  | .ofChosenColor _ =>
    let n := countChoice (.quality .color) drs
    localRefuse (n == 1) (.choiceRef .theChoice (.quality .color) n)
  | .producedByEvent _ =>
    let n := countOutcomes .manaProduced drs
    localRefuse (n == 1) (.outcomeInScope .manaProduced n)
  | .amongWritten colors => localRefuse (colorCountOk colors.length) .colorCountOk
  | _ => []

def ManaRider.localChecks : ManaRider → List Refusal
  | .spendOnly purposes | .spendNotOn purposes => localRefuse (!purposes.isEmpty) .nonEmpty
  | .onSpent _ _ _ _ => []

def ManaHeld.localChecks (drs : DRS) : ManaHeld → List Refusal
  | .thisMana =>
    let n := countOutcomes .manaAdded drs
    localRefuse (n == 1) (.outcomeInScope .manaAdded n)
  | .unspent _ => []

def Amount.forEach (amount : Amount) (fuel : Nat := 32) : Bool :=
  match fuel, amount with
  | n + 1, .inCaller body => body.forEach n
  | _ + 1, .arith .times _ _ => true
  | _, _ => false
termination_by structural fuel

def Cost.localChecks : Cost → List Refusal
  | .mana symbols => localRefuse (!symbols.isEmpty) .nonEmpty
  | .scaled _ amount => localRefuse amount.forEach .forEachAmount
  | _ => []

def CostShift.localChecks : CostShift → List Refusal
  | .bySymbols (.lit symbols) _ _ => localRefuse (!symbols.isEmpty) .nonEmpty
  | _ => []

def CreationSpec.localChecks : CreationSpec → List Refusal
  | .emblem _ abilities => localRefuse (!abilities.isEmpty) .nonEmpty
  | .token _ _ => []

def CounterKindSource.localChecks (drs : DRS) : CounterKindSource → List Refusal
  | .printed kind => localRefuse (knownCounter kind.name) .knownCounter
  | .chosen menu | .distinctChosen menu =>
    localRefuse (!menu.isEmpty) .nonEmpty ++
      menu.flatMap (fun kind => localRefuse (knownCounter kind.name) .knownCounter)
  | .bound =>
    let n := countChoice (.quality .counterKind) drs
    localRefuse (n == 1) (.choiceRef .theChoice (.quality .counterKind) n)
  | .those =>
    let n := countOutcomes .countersPut drs
    localRefuse (n == 1) (.outcomeInScope .countersPut n)
  | .own | .sameAs _ | .anyKind => []

def counterHolderKind (kind : Kind) : Bool := Kind.lte kind (.join .object .player)

def CounterKindSource.scope : CounterKindSource → Kind → Bool
  | .printed counter, kind => counter.scope == kind
  | .chosen menu, kind | .distinctChosen menu, kind => menu.all (fun counter => counter.scope == kind)
  | .sameAs _, kind => Kind.lte kind .object
  | _, kind => counterHolderKind kind

def optCounterSourceScope : Option CounterKindSource → Kind → Bool
  | none, kind => counterHolderKind kind
  | some source, kind => source.scope kind

def Amount.unit (amount : Amount) (fuel : Nat := 32) : Bool :=
  match fuel, amount with
  | n + 1, .inCaller body => body.unit n
  | _ + 1, .lit 1 => true
  | _, _ => false
termination_by structural fuel

def kindAmountOk (amount : Amount) : CounterKindSource → Bool
  | .sameAs _ => amount.unit
  | _ => true

def optKindAmountOk (amount : Amount) : Option CounterKindSource → Bool
  | none => true
  | some source => kindAmountOk amount source

def Amount.settableValue (amount : Amount) (fuel : Nat := 32) : Bool :=
  match fuel, amount with
  | n + 1, .inCaller body => body.settableValue n
  | _ + 1, .statOf (.playerStat .lifeTotal) _ | _ + 1, .statOf (.stat .power) _
  | _ + 1, .statOf (.stat .toughness) _ | _ + 1, .theOutcome .rollResult => true
  | _, _ => false
termination_by structural fuel

def Exchanged.localChecks : Exchanged → List Refusal
  | .values left right => localRefuse left.settableValue .settableValue ++
    localRefuse right.settableValue .settableValue
  | _ => []

def LinkedFacet.named : LinkedFacet → Bool
  | .colorsSpent | .manaValueSpent => true
  | .timesPaid source | .readback source _ | .chosenPlayer source | .taken source => source.named
  | .objects source deed _ =>
    source.named && knownAct deed && (actPatientKindsOf deed).elem .object && source.performs deed

def LinkedFacet.choiceFound : LinkedFacet → Bool
  | .chosenPlayer source => !source.named || source.choosesPlayer
  | _ => true

def LinkedFacet.localChecks (asEntity : Bool) (facet : LinkedFacet) : List Refusal :=
  localRefuse facet.named .linkedFacetNamed ++
    localRefuse (facet.readsEntity == asEntity) .linkedFacetKind ++
    match facet with
    | .chosenPlayer source => localRefuse facet.choiceFound (.linkedChoiceInScope source)
    | _ => []

end Semantics
