import Semantics.Check.Phrase

namespace Semantics

/-- Where a number below zero reads as zero and where it stands [CR#107.1b]: an effect's result
is clamped unless it sets, doubles, or triples a life total or a power and toughness. -/
inductive NumberRegime where
  | clamped | signed
  deriving DecidableEq, Repr

/-- An `Option Amount` slot in the clamped regime. -/
private def optClamped : Option Amount → List (Amount × NumberRegime)
  | none => []
  | some amount => [(amount, .clamped)]

/-- A boost or a penalty ("gets +2/+0", "gets −X/−0") is a signed change whose magnitude is
clamped: the sign is the constructor, so the amount inside cannot go below zero [CR#107.1b].
`set` is the exception the rule names — an effect that sets a life total or a power and
toughness reads a negative. -/
def Delta.numberSlots : Delta Amount → List (Amount × NumberRegime)
  | .up amount | .down amount => [(amount, .clamped)]
  | .set amount => [(amount, .signed)]

/-- Exchanging two numerical values [CR#701.12g] sets each side to the other: a set of a life
total or a power and toughness, so both stand below zero [CR#107.1b]. -/
def Exchanged.numberSlots : Exchanged → List (Amount × NumberRegime)
  | .lifeTotals _ | .controlOf _ _ | .cardsAcross _ _ | .zones _ _ => []
  | .values left right => [(left, .signed), (right, .signed)]

/-- A cost shift is an amount of mana, and a floor is the least it can be reduced to: both are
counts, so neither goes below zero [CR#107.1b]. -/
def CostShift.numberSlots : CostShift → List (Amount × NumberRegime)
  | .generic change floor => (change.amount, .clamped) :: optClamped floor
  | .bySymbols _ _ => []

/-- "as though its power were greater than 4" compares a game value, and a comparison uses a
negative when it needs one [CR#107.1b]. -/
def AsThough.numberSlots : AsThough → List (Amount × NumberRegime)
  | .condition _ | .mana _ _ _ => []
  | .greater _ amount => [(amount, .signed)]

/-- How much damage a prevention shield stops: damage is never negative [CR#107.1b]. -/
def PreventCut.numberSlots : PreventCut → List (Amount × NumberRegime)
  | .all | .half _ => []
  | .some amount | .shield amount | .allBut amount => [(amount, .clamped)]

/-- Doubling and tripling damage carries its factor as a `ScaleFactor`, not an `Amount`, so it
declares no slot. A shift is a magnitude with the direction as its sign, and the damage it
adjusts is not negative [CR#107.1b]. -/
def DamageScale.numberSlots : DamageScale → List (Amount × NumberRegime)
  | .multiplied _ | .halved _ => []
  | .shifted change => [(change.amount, .clamped)]

/-- A damage replacement's own slots. The `also` action a prevention runs declares its
own. -/
def DamageOp.numberSlots : DamageOp → List (Amount × NumberRegime)
  | .prevent cut _ | .redirect cut _ => cut.numberSlots
  | .scale damageScale => damageScale.numberSlots

/-- "two more times": a count of repetitions [CR#107.1b]. -/
def Repetition.numberSlots : Repetition → List (Amount × NumberRegime)
  | .again | .anyNumber | .untilCond _ | .againExcludingChosen => []
  | .moreTimes times | .fixed times _ => [(times, .clamped)]

/-- How many coins are flipped: a count [CR#107.1b]. -/
def FlipScope.numberSlots : FlipScope → List (Amount × NumberRegime)
  | .count amount => [(amount, .clamped)]
  | .per _ => []

/-- How many results a player may ignore: a count [CR#107.1b]. -/
def IgnoredOutcomes.numberSlots : IgnoredOutcomes → List (Amount × NumberRegime)
  | .extreme _ | .allBut _ => []
  | .chosen _ amount => [(amount, .clamped)]

/-- A rider's own slots: how many counters it enters with or is exiled with, a count
[CR#107.1b]. The copy
exceptions of `asCopyOf` declare their own. -/
def MoveRider.numberSlots (value : MoveRider) (fuel : Nat := 64) : List (Amount × NumberRegime) :=
  match fuel, value with
  | 0, _ => []
  | n + 1, .inCaller body => body.numberSlots n
  | _ + 1, .withCounters amount _ _ => [(amount, .clamped)]
  | _ + 1, .entersAs _ | _ + 1, .entersAttacking _ | _ + 1, .entersTransformed
  | _ + 1, .entersMelded _ | _ + 1, .under _ | _ + 1, .asCopyOf _ _ _ | _ + 1, .faceDown _
  | _ + 1, .inOrder _ => []
termination_by structural fuel

def MoveRider.ridersSlots : List MoveRider → List (Amount × NumberRegime)
  | [] => []
  | rider :: rest => rider.numberSlots ++ MoveRider.ridersSlots rest

/-- A replacement's own slots: the riders it adds to the move it replaces, as a move's own riders
are the move's. The actions it carries and the condition it applies under declare their own. -/
def Replacement.numberSlots (value : Replacement) (fuel : Nat := 64) :
    List (Amount × NumberRegime) :=
  match fuel, value with
  | 0, _ => []
  | n + 1, .inCaller body => body.numberSlots n
  | _ + 1, .withRiders riders => MoveRider.ridersSlots riders
  | _ + 1, .instead _ | _ + 1, .also _ | _ + 1, .onlyIf _ _ => []
termination_by structural fuel

/-- A cost's own slots: "for each" multiplies a cost by a count [CR#107.1b]. A nested cost and
a performed action declare their own. -/
def Cost.numberSlots (value : Cost) (fuel : Nat := 64) : List (Amount × NumberRegime) :=
  match fuel, value with
  | 0, _ => []
  | _n + 1, .inCaller body => body.numberSlots _n
  | _n + 1, .anyCost | _n + 1, .mana _ => []
  | _n + 1, .scaled _ amount => [(amount, .clamped)]
  | _n + 1, .tapSymbol | _n + 1, .untapSymbol | _n + 1, .loyaltySymbol _ => []
  | _n + 1, .perform _ | _n + 1, .compound _ | _n + 1, .or _ | _n + 1, .itsManaCost | _n + 1, .withSubstitute _ _ => []
termination_by structural fuel

/-- A stat edit's slot: a set stands below zero, as an effect that sets a power and toughness
does, and a change by an amount reads one below zero as zero [CR#107.1b]. -/
def CharacteristicEdit.numberSlots : CharacteristicEdit → List (Amount × NumberRegime)
  | .stat .sets _ amount => [(amount, .signed)]
  | .stat _ _ amount => [(amount, .clamped)]
  | _ => []

/-- The static spec's own slots and the regime each is read in [CR#107.1b]. -/
def StaticSpec.numberSlots (value : StaticSpec) (fuel : Nat := 64) : List (Amount × NumberRegime) :=
  match fuel, value with
  | 0, _ => []
  | _n + 1, .inCaller body => body.numberSlots _n
  | _n + 1, .characteristicChange _ edits | _n + 1, .characteristicDefining _ edits => edits.flatMap CharacteristicEdit.numberSlots
  | _n + 1, .combatant _ | _n + 1, .textChange _ _ => []
  | _n + 1, .costShift _ shift => shift.numberSlots
  | _n + 1, .altCost _ _ | _n + 1, .addedCost _ _ _ | _n + 1, .paySubstitute _ _ | _n + 1, .specialAction _ _ _ | _n + 1, .modeChoice _
  | _n + 1, .addsTarget _ _ | _n + 1, .asEvent _ _ | _n + 1, .whileHolds _ _
  | _n + 1, .revealedAsCast _ _ => []
  -- The letter X, defined by the text [CR#107.3c]: a definition is a calculation, not a result,
  -- so a negative stands and the slot that consumes X clamps it or not [CR#107.1b].
  | _n + 1, .letterDefinition _ amount => [(amount, .signed)]
  | _n + 1, .cant _ _ mods | _n + 1, .must _ _ mods | _n + 1, .can _ _ mods | _n + 1, .gatedBy _ _ _ mods =>
    mods.asThough.toList.flatMap AsThough.numberSlots
  | _n + 1, .manaRetention _ _ | _n + 1, .partSkip _ _ => []
  | _n + 1, .retention _ _ => []
  | _n + 1, .copyChange _ _ _ | _n + 1, .controlGrant _ _ => []
  | _n + 1, .replace _ _ _ _ _ | _n + 1, .damageResult _ => []
  | _n + 1, .damageRule _ _ op _ _ => op.numberSlots
  | _n + 1, .preventionBan _ _ _ => []
  | _n + 1, .conditional _ _ _ => []
  | _n + 1, .visibility _ _ _ | _n + 1, .additionalTriggers _ _ => []
  | _n + 1, .conjunction _ _ => []
termination_by structural fuel

/-- The action's own slots and the regime each is read in [CR#107.1b]. -/
def Action.numberSlots (value : Action) (fuel : Nat := 64) : List (Amount × NumberRegime) :=
  match fuel, value with
  | 0, _ => []
  | _n + 1, .inCaller body | _n + 1, .ownScope _ body | _n + 1, .bracketed body | _n + 1, .instruct _ body => body.numberSlots _n
  -- You can't deal negative damage [CR#107.1b].
  | _n + 1, .dealDamage amount _ _ | _n + 1, .markDamage _ amount => [(amount, .clamped)]
  | _n + 1, .setStatus _ _ | _n + 1, .turnOver _ | _n + 1, .combat _ _ => []
  | _n + 1, .attack _ | _n + 1, .attackUnblocked _ | _n + 1, .attackWith _ _ | _n + 1, .block _ | _n + 1, .target _ | _n + 1, .changeControl _ _
  | _n + 1, .tapForMana _ _ | _n + 1, .commitCrime | _n + 1, .trigger
  | _n + 1, .crew _ | _n + 1, .saddle _ | _n + 1, .fullyUnlock _ => []
  | _n + 1, .attachment _ _ _ | _n + 1, .clearDamage _ => []
  | _n + 1, .gainDesignation _ _ _ | _n + 1, .gameGains _ | _n + 1, .unlock _ | _n + 1, .conclude _ _ => []
  | _n + 1, .restartGame | _n + 1, .separateIntoPiles _ _ _ => []
  | _n + 1, .choose _ _ _ _ | _n + 1, .revealChoices _ | _n + 1, .vote _ _ _ _ => []
  | _n + 1, .move _ _ _ _ _ riders => MoveRider.ridersSlots riders
  | _n + 1, .turnFaceDown _ _ => []
  -- How many copies to make: a count [CR#107.1b].
  | _n + 1, .copy _ _ times _ => [(times, .clamped)]
  | _n + 1, .chooseNewTargets _ | _n + 1, .copyTargets _ _ => []
  -- Gaining and losing life are clamped; "your life total becomes …" is the set the rule
  -- excepts, and doubling a life total is written as that set [CR#107.1b].
  | _n + 1, .changeLife delta => delta.numberSlots
  | _n + 1, .exchange exchanged => exchanged.numberSlots
  | _n + 1, .addMana amount _ _ => [(amount, .clamped)]
  | _n + 1, .draw (.exactlyOf amount) | _n + 1, .draw (.upToOf amount)
  | _n + 1, .draw (.moreThanOf amount) => [(amount, .clamped)]
  | _n + 1, .draw _ => []
  | _n + 1, .expose _ _ | _n + 1, .search _ _ | _n + 1, .shuffle _ => []
  | _n + 1, .flipCoins count _ _ => count.numberSlots
  | _n + 1, .rollDice count _ _ => [(count, .clamped)]
  | _n + 1, .applyResultsTable _ => []
  | _n + 1, .ignoreOutcomes which => which.numberSlots
  -- The sign is the direction; the amount added to a result is a magnitude [CR#107.1b].
  | _n + 1, .shiftResult changes => changes.map ((·.amount, .clamped))
  | _n + 1, .storeResults _ | _n + 1, .rerollStored _ _ => []
  -- "where X is …" in an action: signed, as the static definition is [CR#107.1b,107.3c].
  | _n + 1, .establish spec none => match spec with
    | .letterDefinition _ amount => [(amount, .signed)]
    | _ => []
  | _n + 1, .establish _ _ => []
  | _n + 1, .createObject count spec _ _ => (count, .clamped) ::
      (match spec with
       | .token _ riders => MoveRider.ridersSlots riders
       | .emblem _ _ => [])
  | _n + 1, .putCounters amount _ _ _ => [(amount, .clamped)]
  | _n + 1, .distribute _ amount _ => [(amount, .clamped)]
  | _n + 1, .removeCounters _ _ _ _ => []
  | _n + 1, .enact _ _ _ _ | _n + 1, .pay _ _ _ _ _ => []
  | _n + 1, .withContinuation _ _ _ _ | _n + 1, .doIf _ _ _ | _n + 1, .limited _ _ => []
  | _n + 1, .doForEach _ _ | _n + 1, .doForEachKind _ _ _ _ => []
  | _n + 1, .repeat_ repetition => repetition.numberSlots
  | _n + 1, .sequentially _ | _n + 1, .simultaneously _ | _n + 1, .chooseModes _ _ => []
  | _n + 1, .delay _ _ _ _ | _n + 1, .replace _ _ | _n + 1, .holdUntil _ _ => []
  | _n + 1, .triggerReflexively _ _ _ => []
  | _n + 1, .skipUntap _ steps => [(steps, .clamped)]
  | _n + 1, .skipPart _ count => [(count, .clamped)]
  | _n + 1, .addTurnPart _ _ count _ => [(count, .clamped)]
  | _n + 1, .procedure _ _ _ _ _ _ | _n + 1, .relate _ _ _ | _n + 1, .merge _ _ _ => []

termination_by structural fuel

def GameEvent.numberSlots (event : GameEvent) (fuel : Nat := 64) : List (Amount × NumberRegime) :=
  match fuel, event with
  | 0, _ => []
  | _n + 1, .inCaller body => body.numberSlots _n
  | _ + 1, .does _ _ | _ + 1, .beginningOf _ _ _ | _ + 1, .endOf _ _ _ | _ + 1, .now
  | _ + 1, .stateHolds _ | _ + 1, .chapterMark _
  | _ + 1, .nthOccurrence _ _ _ | _ + 1, .causes _ _ | _ + 1, .stateBasedActionsChecked => []
termination_by structural fuel

end Semantics
