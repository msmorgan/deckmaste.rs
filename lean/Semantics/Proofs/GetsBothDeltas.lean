import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Proofs.GetsBothDeltas

"<subject> gets +P/+T" is one node, `StaticSpec.ptModification`, carrying a power delta and a
toughness delta with the subject written once [CR#613.4b,613.4c]. It replaced a conjunction of
two `modification`s whose toughness half read the subject back through `itsOther`.

The retired spelling is kept below as `getsPtTwoHalves`, and each twin pin asserts that the two
spellings get the same refusals, by `decide`: one twin per existing pin that writes `get`,
`getsPt` or `getsBase`, one per bench shape, and one per pin that writes "gets +P/+T" as two
`modification`s by hand. The one-node form checks the subject once, so where the subject
itself fails, the toughness half's second refusal of the same failure is gone; those four pins
are kept here at their old values in the retired spelling (`…TwoHalves`).
-/

open Semantics Semantics.Macros

namespace Semantics.Proofs.GetsBothDeltas

/-- The retired spelling of "<subject> gets +P/+T": two modifications, the toughness half
reading its subject back as "it" through a window over what the subject and the power delta
announced. -/
def getsPtTwoHalves (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec :=
  .conjunction none
    [ .modification subject .power power,
      .modification (itsOther subject power) .toughness toughness ]

/-- The retired spelling of "<subject> has base power and toughness P/T". -/
def getsBaseTwoHalves (subject : NounPhrase) (power toughness : Amount) : StaticSpec :=
  getsPtTwoHalves subject (.set power) (.set toughness)

/-- The retired spelling of "<subject> gets +P/+T <duration>". -/
def getTwoHalves (subject : NounPhrase) (power toughness : Delta Amount)
    (duration : Option Duration) : Instruction :=
  .establish (getsPtTwoHalves subject power toughness) duration

/-- An activated ability with no timing, limit, guard, or activator (`Deontic.act`). -/
def activatedAbility (cost : Cost) (instruction : Instruction) : Ability :=
  .activated cost instruction none none none none

/-- A single-faced card (`Faces.one`). -/
def oneFaced (face : Characteristics) : Card := .singleFaced { characteristics := face }

/-! ## The node -/

theorem getsPtIsOneNode (subject : NounPhrase) (power toughness : Delta Amount) :
    getsPt subject power toughness = .ptModification subject power toughness := rfl

/-- "Target creature gets +2/+2 until end of turn" introduces exactly one target. -/
theorem okTargetGetsIntroducesOneTarget :
    (Instruction.intro [] (get (target creature) (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn))).countP
      (fun b => b.det == .target) = 1 := by
  decide

/-- Two modifications that each write "target creature" introduce two targets: the defect the
one node removes from the RON spelling. -/
theorem subjectWrittenTwiceIntroducesTwoTargets :
    (Instruction.intro []
      (.establish
        (.conjunction none
          [ .modification (target creature) .power (.up (.lit 2)),
            .modification (target creature) .toughness (.up (.lit 2)) ])
        (some untilEndOfTurn))).countP (fun b => b.det == .target) = 2 := by
  decide

/-- Each delta keeps its own regime: a change by an amount is clamped, a set is signed
[CR#107.1b]. -/
theorem ptModificationSlots :
    (StaticSpec.ptModification thisCreature (.up (.lit 2)) (.set (.lit (-1)))).numberSlots =
      [(.lit 2, .clamped), (.lit (-1), .signed)] := by
  rfl

/-- "Target creature gets +X/+X until end of turn, where X is its power." Both deltas read the
subject, the same way in both spellings. -/
theorem okDeltasReadTheirSubject :
    Instruction.check []
      (get (target creature) (.up (.statOf (.stat .power) it)) (.up (.statOf (.stat .power) it))
        (some untilEndOfTurn)) = [] := by
  decide

theorem okDeltasReadTheirSubjectTwin :
    Instruction.check []
      (get (target creature) (.up (.statOf (.stat .power) it)) (.up (.statOf (.stat .power) it))
        (some untilEndOfTurn)) =
      Instruction.check []
        (getTwoHalves (target creature) (.up (.statOf (.stat .power) it))
          (.up (.statOf (.stat .power) it)) (some untilEndOfTurn)) := by
  decide

/-- The same after "destroy target artifact": a second object is in the discourse, and both
spellings read "its" as the creature. -/
theorem deltasReadTheirSubjectAfterAnotherObjectTwin :
    Instruction.check []
      (.sequentially
        [ destroy (target artifact),
          get (target creature) (.up (.statOf (.stat .power) it)) (.up (.statOf (.stat .power) it))
            (some untilEndOfTurn) ]) =
      Instruction.check []
        (.sequentially
          [ destroy (target artifact),
            getTwoHalves (target creature) (.up (.statOf (.stat .power) it))
              (.up (.statOf (.stat .power) it)) (some untilEndOfTurn) ]) := by
  decide

/-- "Target creature gets +1/+1 and gains flying" with the stat change as one part of a flat
coordination. The retired spelling was itself a coordination, so it could not stand as a
part (`badGetsInsideCoordinationTwoHalves`) and the bench wrote its two halves by hand
(`okFlatCoordinationTwin`). -/
theorem okGetsInsideCoordination :
    StaticSpec.check []
      (.conjunction none
        [getsPt (target creature) (.up (.lit 1)) (.up (.lit 1)), .abilityGrant it (keyword "Flying")])
      = [] := by
  decide

theorem badGetsInsideCoordinationTwoHalves :
    StaticSpec.check []
      (.conjunction none
        [ getsPtTwoHalves (target creature) (.up (.lit 1)) (.up (.lit 1)),
          .abilityGrant it (keyword "Flying") ])
      = [.notCoord] := by
  decide

/-! ## Twins of the existing pins that write `get`, `getsPt` or `getsBase` -/

/-- `Anaphora.badSingularReadOfBarePlural`: the same refusals in both spellings. -/
theorem badSingularReadOfBarePluralTwin :
    Instruction.check [] (.sequentially [ get (bare creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn), draw (.statOf (.stat .power) it) ]) =
      Instruction.check [] (.sequentially [ getTwoHalves (bare creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn), draw (.statOf (.stat .power) it) ]) := by
  decide

/-- `Choice.okGetsBattlefield`: the same refusals in both spellings. -/
theorem okGetsBattlefieldTwin :
    Instruction.check [] (get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) =
      Instruction.check [] (getTwoHalves (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) := by
  decide

/-- `Choice.badGetsGraveyard` in the retired two-modification spelling, whose toughness half
re-reads the subject: each of the subject's failures is refused twice. The one-node spelling
refuses each once (`Choice.badGetsGraveyard`). -/
theorem badGetsGraveyardTwoHalves :
    Instruction.check [] (.sequentially [destroy (target creature), getTwoHalves it (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)])
      = [.zoneIs .battlefield, .zoneIs .battlefield] := by
  decide

/-- `Counters.okAltHeaderAgreeingReadback`: the same refusals in both spellings. -/
theorem okAltHeaderAgreeingReadbackTwin :
    Ability.check [] (.triggered (blocks thisCreature (some (a creature))) [becomesBlocked thisCreature (some (a creature))] none [] none none none (get (that (.type .creature)) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn))) =
      Ability.check [] (.triggered (blocks thisCreature (some (a creature))) [becomesBlocked thisCreature (some (a creature))] none [] none none none (getTwoHalves (that (.type .creature)) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn))) := by
  decide

/-- `Counters.badAltHeaderMixedReadback` in the retired two-modification spelling, whose toughness half
re-reads the subject: each of the subject's failures is refused twice. The one-node spelling
refuses each once (`Counters.badAltHeaderMixedReadback`). -/
theorem badAltHeaderMixedReadbackTwoHalves :
    Ability.check [] (.triggered (blocks thisCreature none) [becomesBlocked thisCreature (some (a creature))] none [] none none none (getTwoHalves (that (.type .creature)) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn)))
      = [ .anaphor (.word (.type .creature)) .one 0, .zoneIs .battlefield,
          .anaphor .bare .one 0, .zoneIs .battlefield ] := by
  decide

/-- `Damage.okIt`: the same refusals in both spellings. -/
theorem okItTwin :
    Instruction.check [] (.sequentially [ .setStatus .tapped (target creature), get it (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn) ]) =
      Instruction.check [] (.sequentially [ .setStatus .tapped (target creature), getTwoHalves it (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn) ]) := by
  decide

/-- `Damage.okGetsCreature`: the same refusals in both spellings. -/
theorem okGetsCreatureTwin :
    Instruction.check [] (get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) =
      Instruction.check [] (getTwoHalves (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn)) := by
  decide

/-- `Damage.badGetsSource` in the retired two-modification spelling, whose toughness half
re-reads the subject: each of the subject's failures is refused twice. The one-node spelling
refuses each once (`Damage.badGetsSource`). -/
theorem badGetsSourceTwoHalves :
    Instruction.check [] (getTwoHalves (target source) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))
      = [.zoneIs .battlefield, .zoneIs .battlefield] := by
  decide

/-- `Deontic.badContinuousAsCost`: the same refusals in both spellings. -/
theorem badContinuousAsCostTwin :
    Ability.check [] (activatedAbility (.perform (get (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))) (draw (.lit 1))) =
      Ability.check [] (activatedAbility (.perform (getTwoHalves (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))) (draw (.lit 1))) := by
  decide

/-- `Faces.badStaticOnSorcery`: the same refusals in both spellings. -/
theorem badStaticOnSorceryTwin :
    Card.check (oneFaced { name := some "", cost := some [pip .green], types := [.sorcery], text := [.static (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))] }) =
      Card.check (oneFaced { name := some "", cost := some [pip .green], types := [.sorcery], text := [.static (getsPtTwoHalves (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))] }) := by
  decide

/-- `Faces.okEscalateWithModes`: the same refusals in both spellings. -/
theorem okEscalateWithModesTwin :
    Card.check (oneFaced { name := some "", cost := some [pip .black], types := [.instant], text := [ keywordCosting "Escalate" (.mana [generic 2]), .spell none (chooseModes (.range (some 1) (some 2)) [ get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn), get (target creature) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn) ]) ] }) =
      Card.check (oneFaced { name := some "", cost := some [pip .black], types := [.instant], text := [ keywordCosting "Escalate" (.mana [generic 2]), .spell none (chooseModes (.range (some 1) (some 2)) [ getTwoHalves (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn), getTwoHalves (target creature) (.down (.lit 1)) (.down (.lit 1)) (some untilEndOfTurn) ]) ] }) := by
  decide

/-- `Keyword.badKeywordListOnPlainLine`: the same refusals in both spellings. -/
theorem badKeywordListOnPlainLineTwin :
    Ability.check [] (.alsoForKeywords (.static (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))) [.the "Menace", .the "Trample"]) =
      Ability.check [] (.alsoForKeywords (.static (getsPtTwoHalves (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)))) [.the "Menace", .the "Trample"]) := by
  decide

/-- `Mana.okContinuousPumpClause`: the same refusals in both spellings. -/
theorem okContinuousPumpClauseTwin :
    Instruction.check [] (.establish (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1))) (some untilEndOfTurn)) =
      Instruction.check [] (.establish (getsPtTwoHalves (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1))) (some untilEndOfTurn)) := by
  decide

/-- `Static.okGetsBattlefieldSubject`: the same refusals in both spellings. -/
theorem okGetsBattlefieldSubjectTwin :
    Ability.check [] (.static (onlyWhile (getsPt thisCreature (.up (.lit 2)) (.up (.lit 0))) (.matches thisCreature attacking))) =
      Ability.check [] (.static (onlyWhile (getsPtTwoHalves thisCreature (.up (.lit 2)) (.up (.lit 0))) (.matches thisCreature attacking))) := by
  decide

/-- `Static.badThatCreatureIsCondSubject` in the retired two-modification spelling, whose toughness half
re-reads the subject: each of the subject's failures is refused twice. The one-node spelling
refuses each once (`Static.badThatCreatureIsCondSubject`). -/
theorem badThatCreatureIsCondSubjectTwoHalves :
    Ability.check [] (.static (onlyWhile (getsPtTwoHalves (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0))) (.matches thisCreature attacking)))
      = [ .anaphor (.word (.type .creature)) .one 0, .zoneIs .battlefield,
          .anaphor .bare .one 0, .zoneIs .battlefield ] := by
  decide

/-- `Static.okEquippedCreature`: the same refusals in both spellings. -/
theorem okEquippedCreatureTwin :
    Ability.check [] (.static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 1)) (.up (.lit 1)))) =
      Ability.check [] (.static (getsPtTwoHalves (.attachHost .equipped (.type .creature)) (.up (.lit 1)) (.up (.lit 1)))) := by
  decide

/-- `Static.badEquippedLand`: the same refusals in both spellings. -/
theorem badEquippedLandTwin :
    Ability.check [] (.static (getsPt (.attachHost .equipped (.type .land)) (.up (.lit 1)) (.up (.lit 1)))) =
      Ability.check [] (.static (getsPtTwoHalves (.attachHost .equipped (.type .land)) (.up (.lit 1)) (.up (.lit 1)))) := by
  decide

/-- `Static.badFortifiedCreature`: the same refusals in both spellings. -/
theorem badFortifiedCreatureTwin :
    Ability.check [] (.static (getsPt (.attachHost .fortified (.type .creature)) (.up (.lit 1)) (.up (.lit 1)))) =
      Ability.check [] (.static (getsPtTwoHalves (.attachHost .fortified (.type .creature)) (.up (.lit 1)) (.up (.lit 1)))) := by
  decide

/-- `Static.okContinuousClause`: the same refusals in both spellings. -/
theorem okContinuousClauseTwin :
    Instruction.check [] (.establish (getsPt (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1))) (some untilEndOfTurn)) =
      Instruction.check [] (.establish (getsPtTwoHalves (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1))) (some untilEndOfTurn)) := by
  decide

/-- `Trigger.okThatCreatureAfterAttack`: the same refusals in both spellings. -/
theorem okThatCreatureAfterAttackTwin :
    Ability.check [] (whenever (attacks (a creature)) (get (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn))) =
      Ability.check [] (whenever (attacks (a creature)) (getTwoHalves (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn))) := by
  decide

/-- `Trigger.badThatCreatureIsSelf`: the same refusals in both spellings. -/
theorem badThatCreatureIsSelfTwin :
    Ability.check [] (whenever (attacks thisCreature) (get (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn))) =
      Ability.check [] (whenever (attacks thisCreature) (getTwoHalves (that (.type .creature)) (.up (.lit 2)) (.up (.lit 0)) (some untilEndOfTurn))) := by
  decide

/-- `Turn.okUntilEndOfTurnDuration`: the same refusals in both spellings. -/
theorem okUntilEndOfTurnDurationTwin :
    Instruction.check [] (get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) =
      Instruction.check [] (getTwoHalves (target creature) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn)) := by
  decide

/-- `Turn.badUntilBeginningOfUpkeep`: the same refusals in both spellings. -/
theorem badUntilBeginningOfUpkeepTwin :
    Instruction.check [] (get (target creature) (.up (.lit 3)) (.up (.lit 3)) (some (.untilEvent (.beginningOf .the .upkeep (.byPlayer .you))))) =
      Instruction.check [] (getTwoHalves (target creature) (.up (.lit 3)) (.up (.lit 3)) (some (.untilEvent (.beginningOf .the .upkeep (.byPlayer .you))))) := by
  decide

/-- `Faces.okBushidoWithBushidoDefinition`: "it gets +2/+2" inside the keyword's definition. -/
theorem okBushidoWithBushidoDefinitionTwin :
    Ability.check [] (bushido 2) =
      Ability.check []
        (.keyword "Bushido" [.number (.lit 2)]
          [ triggeredOr (blocks thisCreature none) [becomesBlocked thisCreature none]
              (getTwoHalves thisCreature (.up (.lit 2)) (.up (.lit 2)) (some untilEndOfTurn)) ]) := by
  decide

/-! ## Twins of the pins that write "gets +P/+T" as two modifications by hand

Each keeps its own spelling; the twin writes the one node with the subject once. -/

/-- `Anaphora.okOwnReadsOneInDelta` in the two-modification spelling it had before. -/
theorem okOwnReadsOneInDeltaTwoHalves :
    Instruction.check []
      (establishFor (target creature)
        [ .modification (.pro .bare .one (.top (NounPhrase.introduced [] (target creature)).length))
            .power
            (.up (.lit 1)),
          .modification (.pro .bare .one (.top (NounPhrase.introduced [] (target creature)).length))
            .toughness (.up (.lit 1)) ]
        none) = [] := by
  decide

/-- `Anaphora.badSharedSubjectTwoInDelta` in the two-modification spelling it had before: each
half re-reads the shared subject, so the ambiguity is refused twice. -/
theorem badSharedSubjectTwoInDeltaTwoHalves :
    Instruction.check []
      (establishFor (Primitives.NounPhrase.both (target creature) (target artifact))
        [ .modification
            (.pro .bare .one
              (.top (NounPhrase.introduced [] (Primitives.NounPhrase.both (target creature) (target artifact))).length))
            .power (.up (.lit 1)),
          .modification
            (.pro .bare .one
              (.top (NounPhrase.introduced [] (Primitives.NounPhrase.both (target creature) (target artifact))).length))
            .toughness (.up (.lit 1)) ]
        none) = [.anaphor .bare .one 2, .anaphor .bare .one 2] := by
  decide

/-- `Keyword.okFlatCoordination`. -/
theorem okFlatCoordinationTwin :
    StaticSpec.check []
      (.conjunction none
        [ .ptModification (target creature) (.up (.lit 1)) (.up (.lit 1)),
          .abilityGrant it (keyword "Flying"),
          .abilityGrant it (keyword "Trample") ]) =
      StaticSpec.check []
        (.conjunction none
          [ .modification (target creature) .power (.up (.lit 1)),
            .modification it .toughness (.up (.lit 1)),
            .abilityGrant it (keyword "Flying"),
            .abilityGrant it (keyword "Trample") ]) := by
  decide

/-- `Keyword.badNestedCoordination`. -/
theorem badNestedCoordinationTwin :
    StaticSpec.check []
      (.conjunction none
        [ .conjunction none
            [ .ptModification (target creature) (.up (.lit 1)) (.up (.lit 1)),
              .abilityGrant it (keyword "Flying") ],
          .abilityGrant it (keyword "Trample") ]) =
      StaticSpec.check []
        (.conjunction none
          [ .conjunction none
              [ .modification (target creature) .power (.up (.lit 1)),
                .modification it .toughness (.up (.lit 1)),
                .abilityGrant it (keyword "Flying") ],
            .abilityGrant it (keyword "Trample") ]) := by
  decide

/-- `Keyword.badThatCreatureIsStaticSubject`. -/
theorem badThatCreatureIsStaticSubjectTwin :
    Ability.check []
      (.static
        (.conjunction none
          [ .ptModification thisCreature (.up (.lit 1)) (.up (.lit 1)),
            .abilityGrant (that (.type .creature)) (keyword "Flying") ])) =
      Ability.check []
        (.static
          (.conjunction none
            [ .modification thisCreature .power (.up (.lit 1)),
              .modification it .toughness (.up (.lit 1)),
              .abilityGrant (that (.type .creature)) (keyword "Flying") ])) := by
  decide

/-- `Keyword.okCoordinatedPlural`. -/
theorem okCoordinatedPluralTwin :
    Ability.check []
      (.static
        (.conjunction none
          [ .ptModification (allOf creatureYouControl) (.up (.lit 1)) (.up (.lit 1)),
            .abilityGrant them (keyword "Flying") ])) =
      Ability.check []
        (.static
          (.conjunction none
            [ .modification (allOf creatureYouControl) .power (.up (.lit 1)),
              .modification them .toughness (.up (.lit 1)),
              .abilityGrant them (keyword "Flying") ])) := by
  decide

/-- `Keyword.badCoordinatedHostPlural`. -/
theorem badCoordinatedHostPluralTwin :
    Ability.check []
      (.static
        (.conjunction none
          [ .ptModification (.attachHost .enchanted (.type .creature)) (.up (.lit 1)) (.up (.lit 1)),
            .abilityGrant them (keyword "Flying") ])) =
      Ability.check []
        (.static
          (.conjunction none
            [ .modification (.attachHost .enchanted (.type .creature)) .power (.up (.lit 1)),
              .modification it .toughness (.up (.lit 1)),
              .abilityGrant them (keyword "Flying") ])) := by
  decide

/-- `Keyword.sharedSubjectSurvivesSecondSingular`. -/
theorem sharedSubjectSurvivesSecondSingularTwin :
    Instruction.check []
      (.sequentially
        [ exile (target artifact),
          establishFor (target creature)
            [ .ptModification (ownSubject (target creature)) (.up (.lit 1)) (.up (.lit 1)),
              .abilityGrant (ownSubject (target creature)) (keyword "Flying") ]
            (some untilEndOfTurn) ]) =
      Instruction.check []
        (.sequentially
          [ exile (target artifact),
            establishFor (target creature)
              [ .modification (ownSubject (target creature)) .power (.up (.lit 1)),
                .modification (ownSubject (target creature)) .toughness (.up (.lit 1)),
                .abilityGrant (ownSubject (target creature)) (keyword "Flying") ]
              (some untilEndOfTurn) ]) := by
  decide

/-- `Deontic.okCoordinatedLandHostBlocks`. -/
theorem okCoordinatedLandHostBlocksTwin :
    Ability.check []
      (.static
        (.conjunction none
          [ .ptModification (.attachHost .enchanted (.type .land)) (.up (.lit 1)) (.up (.lit 1)),
            deontic it .forbid [.core .block] .agent .noPatient ])) =
      Ability.check []
        (.static
          (.conjunction none
            [ .modification (.attachHost .enchanted (.type .land)) .power (.up (.lit 1)),
              .modification it .toughness (.up (.lit 1)),
              deontic it .forbid [.core .block] .agent .noPatient ])) := by
  decide

/-- `Static.okSingleStaticXRider`. -/
theorem okSingleStaticXRiderTwin :
    StaticSpec.check []
      (.conjunction none
        [ .ptModification thisCreature (.up (.letter .x)) (.up (.lit 0)),
          .letterDefinition .x (countOf creatureYouControl) ]) =
      StaticSpec.check []
        (.conjunction none
          [ .modification thisCreature .power (.up (.letter .x)),
            .modification thisCreature .toughness (.up (.lit 0)),
            .letterDefinition .x (countOf creatureYouControl) ]) := by
  decide

/-- `Static.badDoubleStaticRider`. -/
theorem badDoubleStaticRiderTwin :
    StaticSpec.check []
      (.conjunction none
        [ .ptModification thisCreature (.up (.letter .x)) (.up (.lit 0)),
          .letterDefinition .x (countOf creatureYouControl),
          .letterDefinition .x (countOf creature) ]) =
      StaticSpec.check []
        (.conjunction none
          [ .modification thisCreature .power (.up (.letter .x)),
            .modification thisCreature .toughness (.up (.lit 0)),
            .letterDefinition .x (countOf creatureYouControl),
            .letterDefinition .x (countOf creature) ]) := by
  decide

/-! ## Twins of bench shapes (`lean/Semantics/Cards`)

Each is the bench clause as written, against the same clause in the retired spelling. -/

/-- Somberwald Alpha: "Whenever a creature you control becomes blocked, it gets +1/+1". -/
theorem somberwaldAlphaTwin :
    Ability.check []
      (whenever (becomesBlocked (a creatureYouControl) none)
        (get it (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))) =
      Ability.check []
        (whenever (becomesBlocked (a creatureYouControl) none)
          (getTwoHalves it (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))) := by
  decide

/-- Deeproot Warrior: "Whenever this creature becomes blocked, it gets +1/+1". -/
theorem deeprootWarriorTwin :
    Ability.check []
      (whenever (becomesBlocked thisCreature none)
        (get it (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))) =
      Ability.check []
        (whenever (becomesBlocked thisCreature none)
          (getTwoHalves it (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn))) := by
  decide

/-- Jeskai Ascendancy: "creatures you control get +1/+1 until end of turn. Untap those
creatures." -/
theorem jeskaiAscendancyPumpTwin :
    Ability.check []
      (whenever (.casts .you (some (a (.and [spell, .not creature]))) none)
        (.sequentially
          [ get (bare creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
            untap (those (.type .creature)) ])) =
      Ability.check []
        (whenever (.casts .you (some (a (.and [spell, .not creature]))) none)
          (.sequentially
            [ getTwoHalves (bare creatureYouControl) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
              untap (those (.type .creature)) ])) := by
  decide

/-- Diminish: "Target creature has base power and toughness 1/1 until end of turn." -/
theorem diminishTwin :
    Ability.check []
      (.spell none (.establish (getsBase (target creature) (.lit 1) (.lit 1)) (some untilEndOfTurn))) =
      Ability.check []
        (.spell none
          (.establish (getsBaseTwoHalves (target creature) (.lit 1) (.lit 1)) (some untilEndOfTurn))) := by
  decide

/-- Cycle of Life: "Target creature you cast this turn has base power and toughness 0/1 until
your next upkeep." -/
theorem cycleOfLifeTwin :
    Instruction.check []
      (.establish (getsBase (target (.and [creature, castBy .you])) (.lit 0) (.lit 1))
        (some (.until_ (.startOf .upkeep (some .you))))) =
      Instruction.check []
        (.establish (getsBaseTwoHalves (target (.and [creature, castBy .you])) (.lit 0) (.lit 1))
          (some (.until_ (.startOf .upkeep (some .you))))) := by
  decide

/-- Bonds of Faith: "Enchanted creature gets +2/+2 as long as it's a Human." The condition
reads "it" after the stat change. -/
theorem bondsOfFaithPumpTwin :
    Ability.check []
      (.static
        (onlyWhile (getsPt (.attachHost .enchanted (.type .creature)) (.up (.lit 2)) (.up (.lit 2)))
          (.matches it (.hasSubtype (creatureType "Human"))))) =
      Ability.check []
        (.static
          (onlyWhile
            (getsPtTwoHalves (.attachHost .enchanted (.type .creature)) (.up (.lit 2)) (.up (.lit 2)))
            (.matches it (.hasSubtype (creatureType "Human"))))) := by
  decide

/-- Built to Smash: "Target attacking creature gets +3/+3 until end of turn. If it's an
artifact creature, it gains trample until end of turn." -/
theorem builtToSmashTwin :
    Instruction.check []
      (.sequentially
        [ get (target (.and [creature, attacking])) (.up (.lit 3)) (.up (.lit 3)) (some untilEndOfTurn),
          .doIf (itsA (.and [artifact, creature])) (gain it (keyword "Trample") (some untilEndOfTurn))
            none ]) =
      Instruction.check []
        (.sequentially
          [ getTwoHalves (target (.and [creature, attacking])) (.up (.lit 3)) (.up (.lit 3))
              (some untilEndOfTurn),
            .doIf (itsA (.and [artifact, creature])) (gain it (keyword "Trample") (some untilEndOfTurn))
              none ]) := by
  decide

/-- An age-counter Ooze: "This creature gets +1/+1 for each age counter on it." Each delta
reads the subject as "it". -/
theorem ageCounterPumpTwin :
    Ability.check []
      (.static
        (getsPt thisCreature (.up (times (.lit 1) (countersOn (.named "ageCounter") it)))
          (.up (times (.lit 1) (countersOn (.named "ageCounter") it))))) =
      Ability.check []
        (.static
          (getsPtTwoHalves thisCreature (.up (times (.lit 1) (countersOn (.named "ageCounter") it)))
            (.up (times (.lit 1) (countersOn (.named "ageCounter") it))))) := by
  decide

/-- Nyxathid: "As this creature enters, choose an opponent. This creature gets −1/−1 for each
card in the chosen player's hand." -/
theorem nyxathidTwin :
    Card.check
      (oneFaced
        { name := some "Nyxathid", cost := some [generic 1, pip .black, pip .black],
          types := [.creature], subtypes := [creatureType "Elemental"],
          text :=
            [ .static (entersChoosingPlayer thisCreature (some (.players .opponent))),
              .static (getsPt thisCreature
                (.down (countOf (.and [.isCard, .inZone (handOf (the chosenPlayer))])))
                (.down (countOf (.and [.isCard, .inZone (handOf (the chosenPlayer))])))) ],
          power := stat 7, toughness := stat 7 }) =
      Card.check
        (oneFaced
          { name := some "Nyxathid", cost := some [generic 1, pip .black, pip .black],
            types := [.creature], subtypes := [creatureType "Elemental"],
            text :=
              [ .static (entersChoosingPlayer thisCreature (some (.players .opponent))),
                .static (getsPtTwoHalves thisCreature
                  (.down (countOf (.and [.isCard, .inZone (handOf (the chosenPlayer))])))
                  (.down (countOf (.and [.isCard, .inZone (handOf (the chosenPlayer))])))) ],
            power := stat 7, toughness := stat 7 }) := by
  decide

/-- Archpriest of Iona: "target creature gets +1/+1 and gains flying until end of turn",
under a full-party intervening if. -/
theorem archpriestOfIonaFullPartyTwin :
    Ability.check []
      (triggeredIf (.beginningOf .the .combat (.byPlayer .you)) fullParty
        (.sequentially
          [ get (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
            gain it (keyword "Flying") (some untilEndOfTurn) ])) =
      Ability.check []
        (triggeredIf (.beginningOf .the .combat (.byPlayer .you)) fullParty
          (.sequentially
            [ getTwoHalves (target creature) (.up (.lit 1)) (.up (.lit 1)) (some untilEndOfTurn),
              gain it (keyword "Flying") (some untilEndOfTurn) ])) := by
  decide

/-- Nightmarish End: "Target creature gets −X/−X until end of turn, where X is the number of
cards in your hand." -/
theorem nightmarishEndTwin :
    Instruction.check []
      (.sequentially
        [ get (target creature) (.down (.letter .x)) (.down (.letter .x)) (some untilEndOfTurn),
          Primitives.Instruction.define .x (countOf (.and [.isCard, .inZone (handOf .you)])) ]) =
      Instruction.check []
        (.sequentially
          [ getTwoHalves (target creature) (.down (.letter .x)) (.down (.letter .x))
              (some untilEndOfTurn),
            Primitives.Instruction.define .x (countOf (.and [.isCard, .inZone (handOf .you)])) ]) := by
  decide

/-- Distortion Strike: "Target creature gets +1/+0 until end of turn and can't be blocked this
turn." -/
theorem distortionStrikeLineTwin :
    Instruction.check []
      (.sequentially
        [ get (target creature) (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn),
          .establish (deontic (that (.type .creature)) .forbid [.core .block] .patient .noPatient)
            (some .thisTurn) ]) =
      Instruction.check []
        (.sequentially
          [ getTwoHalves (target creature) (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn),
            .establish (deontic (that (.type .creature)) .forbid [.core .block] .patient .noPatient)
              (some .thisTurn) ]) := by
  decide

/-- "Equipped creature gets +2/+1." -/
theorem equippedPumpTwin :
    Ability.check [] (.static (getsPt (.attachHost .equipped (.type .creature)) (.up (.lit 2)) (.up (.lit 1)))) =
      Ability.check []
        (.static (getsPtTwoHalves (.attachHost .equipped (.type .creature)) (.up (.lit 2)) (.up (.lit 1)))) := by
  decide

/-- "Enchanted creature gets −3/−0." -/
theorem enchantedPenaltyTwin :
    Ability.check [] (.static (getsPt (.attachHost .enchanted (.type .creature)) (.down (.lit 3)) (.down (.lit 0)))) =
      Ability.check []
        (.static (getsPtTwoHalves (.attachHost .enchanted (.type .creature)) (.down (.lit 3)) (.down (.lit 0)))) := by
  decide

/-- "Other creatures you control get +1/+1." -/
theorem otherCreaturesAnthemTwin :
    Ability.check [] (.static (getsPt (allOf (otherCreatureYouControl thisCreature)) (.up (.lit 1)) (.up (.lit 1)))) =
      Ability.check []
        (.static (getsPtTwoHalves (allOf (otherCreatureYouControl thisCreature)) (.up (.lit 1)) (.up (.lit 1)))) := by
  decide

/-- "Whenever you cast a spell, each creature you control gets +1/+0 until end of turn. Scry 1." -/
theorem eachCreaturePumpTwin :
    Ability.check []
      (whenever (.casts .you (some (a spell)) none)
        (.sequentially
          [ get (each creatureYouControl) (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn),
            scry (.lit 1) ])) =
      Ability.check []
        (whenever (.casts .you (some (a spell)) none)
          (.sequentially
            [ getTwoHalves (each creatureYouControl) (.up (.lit 1)) (.up (.lit 0)) (some untilEndOfTurn),
              scry (.lit 1) ])) := by
  decide

/-- Awaken the Bear, which the bench writes in the retired spelling by hand: "Target creature
gets +3/+3 and gains trample until end of turn." -/
theorem awakenTheBearTwin :
    Instruction.check []
      (.establish
        (.conjunction none
          [ .ptModification (target creature) (.up (.lit 3)) (.up (.lit 3)),
            .abilityGrant it (keyword "Trample") ])
        (some untilEndOfTurn)) =
      Instruction.check []
        (.establish
          (.conjunction none
            [ .modification (target creature) .power (.up (.lit 3)),
              .modification (itsOther (target creature) (.up (.lit 3))) .toughness (.up (.lit 3)),
              .abilityGrant it (keyword "Trample") ])
          (some untilEndOfTurn)) := by
  decide

/-- Spidersilk Armor, written in the retired spelling by hand: "Creatures you control get
+0/+1 and have reach." -/
theorem spidersilkArmorTwin :
    Ability.check []
      (.static
        (.conjunction none
          [ .ptModification (allOf creatureYouControl) (.up (.lit 0)) (.up (.lit 1)),
            .abilityGrant them (keyword "Reach") ])) =
      Ability.check []
        (.static
          (.conjunction none
            [ .modification (allOf creatureYouControl) .power (.up (.lit 0)),
              .modification (itsOther (allOf creatureYouControl) (.up (.lit 0))) .toughness
                (.up (.lit 1)),
              .abilityGrant them (keyword "Reach") ])) := by
  decide

/-- The bench's "enchanted creature loses all abilities and is a blue Frog creature with base
power and toughness 1/1", written in the retired spelling by hand; the stat change's subject
is "it". -/
theorem enchantedBecomesVanillaTwin :
    Ability.check []
      (.static
        (.conjunction none
          [ Primitives.StaticSpec.allAbilityLoss (.attachHost .enchanted (.type .creature)) none,
            Primitives.StaticSpec.qualityChange it .sets
              (Primitives.QualityPayload.bundle
                { characteristics :=
                    { colors := [.blue], types := [.creature], subtypes := [creatureType "Frog"] } }
                none),
            .ptModification it (.set (.lit 1)) (.set (.lit 1)) ])) =
      Ability.check []
        (.static
          (.conjunction none
            [ Primitives.StaticSpec.allAbilityLoss (.attachHost .enchanted (.type .creature)) none,
              Primitives.StaticSpec.qualityChange it .sets
                (Primitives.QualityPayload.bundle
                { characteristics :=
                    { colors := [.blue], types := [.creature], subtypes := [creatureType "Frog"] } }
                none),
              .modification it .power (.set (.lit 1)),
              .modification (itsOther it (.set (.lit 1))) .toughness (.set (.lit 1)) ])) := by
  decide

end Semantics.Proofs.GetsBothDeltas
