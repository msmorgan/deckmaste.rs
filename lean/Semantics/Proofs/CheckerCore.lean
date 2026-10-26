import Semantics.Check
import Semantics.Definitions

/-!
# Semantics.Proofs.CheckerCore

The DRT checker's core pins, rehomed from `Semantics.Check`: a Definition's parameter handed off
and read back by identity, identity cleared by a zone change, reflexive and clause-mate pronoun
resolution, pre-event (`would`) stepping, the activated-ability colon, event-head gating, and one
composite pin per checker slice. Each pin is closed by `decide`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.CheckerCore

private def targetCreature : NounPhrase := target isCreature

private def anotherTargetCreature : NounPhrase :=
  .described (.target (.range (some 1) (some 1))) (.and [.other, .hasType .creature])

private def aCreature : NounPhrase := a isCreature

private def handZone : Location :=
  .zone .hand (some .you)

private def graveyardZone : Location :=
  .zone .graveyard (some .you)

/-- Checker fixture, not a keyword action and not `explore`: a Definition whose body hands off to
its own parameter, bound at its first mention and read back by identity across the handoff and a
`doIf` (`ownScope` + `inCaller` + `firstMention` / `laterMention` + `doIf` + `putCounters`).
`explore` hands off to its performer's controller instead [CR#701.44a]. -/
def revealThenCountDef (subject : NounPhrase) : Action :=
  .ownScope (some (.action "revealThenCount")) <|
    .instruct (.firstMention "subject" (.inCaller subject)) <|
      .sequentially [
        .expose .reveal (.cards (.slice .top (.lit 1) (.zone .library (some .you)))),
        .doIf
          (.matches (.pro .bare .one) (.hasType .land))
          (.move (.pro .bare .one) .anyLocation (some handZone) .departure [])
          (some (.putCounters (.lit 1) (.printed (.named "p1p1Counter"))
            (.laterMention "subject" (.inCaller subject))))
      ]

theorem reveal_then_count_def_sound :
    (revealThenCountDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Checker fixture, not a keyword action and not `connive`: a Definition whose body hands off to
its own parameter and on to that object's controller, and reads the parameter back by identity
after a draw, a move and a `did … thisWay` condition (`ownScope` + `inCaller` + `firstMention` /
`laterMention` + `draw` + `move` + `doIf`). `connive` hands off to its performer's controller
instead [CR#701.50a]. -/
def drawDiscardCountDef (subject : NounPhrase) : Action :=
  .ownScope (some (.action "drawDiscardCount")) <|
    .instruct (.firstMention "subject" (.inCaller subject)) <|
      .instruct (.possessorOf .controller .actor) <|
      .sequentially [
        .draw (.exactlyOf (.lit 1)),
        .move (.described (.a .unmarked) (.and [.isCard, .in_ handZone]))
          (handZone) (some graveyardZone) .departure [],
        .doIf
          (.did { subj := .actor .you
                  action := .move (.described (.a .unmarked) (.not (.hasType .land)))
                    (handZone) (some graveyardZone) .departure []
                  within := some .thisWay })
          (.putCounters (.lit 1) (.printed (.named "p1p1Counter"))
            (.laterMention "subject" (.inCaller subject)))
          none
      ]

theorem draw_discard_count_def_sound :
    (drawDiscardCountDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Zone-change identity clearing [CR#400.7]: a later mention of a parameter first mentioned
before the move that made it a new object is refused with `Refusal.bindingIdentity`. A checker
fixture: its header names no keyword action. -/
theorem post_move_later_mention_refused :
    (Action.ownScope (some (.action "exileThenTap")) <|
      .sequentially
        [ exile (.firstMention "subject" (.inCaller targetCreature)),
          tap (.laterMention "subject" (.inCaller targetCreature)) ]).check (DRS.ofList []) =
      [.bindingIdentity "subject"] := by decide

/-- Principle A (`NounPhrase.refl`) vs. `J2` (clause-mate exclusion):
`"Target creature deals 2 damage to itself"` succeeds, whereas `"Target creature deals 2 damage to it"`
is refused because the only referent in scope is a clause-mate. -/
theorem reflexive_subject_succeeds :
    (Action.instruct targetCreature
      (.dealDamage (.lit 2) (.refl .one))).check (DRS.ofList []) = [] := by decide

theorem clause_mate_pronoun_refused :
    (Action.instruct targetCreature
      (.dealDamage (.lit 2) (.pro .bare .one))).check (DRS.ofList []) =
      [.anaphor .bare .one 0] := by decide

/-- Re-mentioned pronoun subject in clause 2 (`"Tap target creature. It deals 2 damage to itself"`):
records `(r.id, .subj)` in `drs.pendingRoles`, licensing `"itself"` and excluding `"it"` in the
object slot. -/
theorem rementioned_subject_licenses_reflexive :
    (Action.sequentially [
      .setStatus .tapped targetCreature,
      .instruct (.pro .bare .one) (.dealDamage (.lit 2) (.refl .one))
    ]).check (DRS.ofList []) = [] := by decide

theorem rementioned_subject_excludes_clause_mate_pronoun :
    (Action.sequentially [
      .setStatus .tapped targetCreature,
      .instruct (.pro .bare .one) (.dealDamage (.lit 2) (.pro .bare .one))
    ]).check (DRS.ofList []) = [.anaphor .bare .one 0] := by decide

/-- Rule `D′` + Rule `AC>R1` complementary split in the same clause:
Clause 0 introduces subject (`targetCreature`) and object (`anotherTargetCreature`).
Clause 1 resolves `"it"` (`.pro .bare .one`) to the subject via `D′` (setting `pronounResolved := true`),
and resolves `"that creature"` (`.pro (.word (.type .creature)) .one`) to the remaining creature via `AC>R1`. -/
theorem pronoun_and_demonstrative_split :
    (Action.sequentially [
      .instruct targetCreature (.dealDamage (.lit 1) anotherTargetCreature),
      .instruct (.pro .bare .one)
        (.dealDamage (.lit 2) (.pro (.word (.type .creature)) .one))
    ]).check (DRS.ofList []) = [] := by decide

/-- Pre-event (`would`) stepping (*Kalitas* replacement):
`"If a nontoken creature an opponent controls would die, instead exile that creature and create a 2/2 black Zombie creature token."` -/
def kalitasReplacement : Ability :=
  static
    (replace
      (would (a (and [non isToken, isCreature, anOpponentControls])) die)
      (instead (sequentially
        [ exile (that creature),
          create 1 (creatureToken 2 2 [.black] [zombie]) ])))

theorem kalitas_replacement_sound :
    kalitasReplacement.check = [] := by decide

/-- Activated `cost : effect` box across the colon:
`"Sacrifice a creature: Draw cards equal to its power."` -/
def sacrificeDrawActivated : Ability :=
  activated (.perform (sacrifice aCreature)) (draw (powerOf it))

theorem sacrifice_draw_activated_sound :
    sacrificeDrawActivated.check = [] := by decide

/-- Closed head-gating (`semantics-v3.md` §3.6):
compound actions (`sequentially`) are refused inside `does`, `would`, `did`, and `cant` with
`Refusal.eventHead`, while a Definition's `ownScope` header is admitted. -/
theorem compound_event_head_refused :
    (GameEvent.does (.actor .you) (.sequentially [.draw (.exactlyOf (.lit 1)), .draw (.exactlyOf (.lit 1))])).check
      (DRS.ofList []) = [.eventHead] := by decide

theorem own_scope_action_event_head_admitted :
    (GameEvent.does (.actor aCreature) (revealThenCountDef aCreature)).check
      (DRS.ofList []) = [] := by decide

/-- Slice A pin (*Weeping Angel*-style combat trigger + attachment/status):
`"Whenever a creature attacks you, tap it and attach target Equipment to that creature."` -/
def weepingAngelCombatPin : Ability :=
  whenever (does aCreature (attack (some you)))
    (sequentially
      [ tap it,
        attachTo (target (hasSubtype equipment)) (that creature) ])

theorem weeping_angel_combat_pin_sound :
    weepingAngelCombatPin.check = [] := by decide

/-- Slice B pin (counters + characteristic modification + pronoun/demonstrative):
`"Double the number of counters on target creature, then it becomes a Dragon in addition to its other types until end of turn."`
[CR#701.10e] -/
def sliceBCountersAndCharPin : Ability :=
  spell <|
    sequentially
      [ doubleCounters targetCreature,
        establish
          (.characteristicChange it [.typeLine .adds { subtypes := some [dragon] }])
          untilEndOfTurn ]

theorem slice_b_counters_and_char_pin_sound :
    sliceBCountersAndCharPin.check = [] := by decide

/-- Slice D pin (vote outcome + quality choice + `ofChosen` resolution):
`"Choose a color. Starting with you, each player votes for grace or condemnation. Draw a card for each grace vote. Return all creatures of the chosen color to their owners' hands."` -/
def sliceDVoteAndChoicePin : Ability :=
  spell <|
    sequentially
      [ choose (a (qualityNoun .color)),
        instruct eachPlayer (voteStartingWith you .openly (byLabel ["grace", "condemnation"])),
        draw (votesFor "grace"),
        returnToHand (allOf (and [isCreature, ofChosen .color])) ]

theorem slice_d_vote_and_choice_pin_sound :
    sliceDVoteAndChoicePin.check = [] := by decide

/-- Slice E pin (copying + `chooseNewTargets` + damage prevention + `theOutcome .damagePrevented`):
`"Copy target instant or sorcery spell. You may choose new targets for the copy. Prevent the next 3 damage that would be dealt to any target this turn, then draw cards equal to the damage prevented this way."` -/
def sliceECopyAndPreventPin : Ability :=
  spell <|
    sequentially
      [ copyOnStack (target (or [isInstant, isSorcery])),
        may (chooseNewTargets (that .copy)),
        establish
          (.replace (.would .any (.dealDamage .anyAmount .any none))
            (.instead (.preventDamage (.shield 3) none (some (draw preventedThisWay))))
            none .repeatedly none)
          untilEndOfTurn ]

theorem slice_e_copy_and_prevent_pin_sound :
    sliceECopyAndPreventPin.check = [] := by decide

end Semantics.Proofs.CheckerCore
