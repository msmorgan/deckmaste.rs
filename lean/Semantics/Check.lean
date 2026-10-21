import Semantics.Check.FactTypes
import Semantics.Check.Facts
import Semantics.Check.Words
import Semantics.Check.Keywords
import Semantics.Check.Refusal
import Semantics.Check.Events
import Semantics.Check.Phrase
import Semantics.Check.Queries
import Semantics.Check.Abilities
import Semantics.Check.AbilityRules
import Semantics.Check.Card
import Semantics.Check.Rules
import Semantics.Definitions.Subtypes

/-!
# Semantics.Check

Root entry point for the Clean-Sheet DRT Checker (`docs/decisions/semantics-v3.md` §1, §3) and
core kernel `by decide` verification pins for Stage 1.
-/

namespace Semantics.Check.CorePins

open Semantics

private def targetCreature : NounPhrase :=
  .described (.target (.range (some 1) (some 1))) (.hasType .creature)

private def anotherTargetCreature : NounPhrase :=
  .described (.target (.range (some 1) (some 1))) (.and [.other, .hasType .creature])

private def aCreature : NounPhrase :=
  .described (.a .unmarked) (.hasType .creature)

private def exileZone : Location :=
  .zone .exile none

private def handZone : Location :=
  .zone .hand (some .you)

private def graveyardZone : Location :=
  .zone .graveyard (some .you)

/-- Representative multi-step definition 1: `explore` [CR#701.44a]
(`ownScope` + `inCaller` + `firstMention` / `laterMention` + `doIf` + `putCounters`). -/
def exploreDef (subject : NounPhrase) : Action :=
  .ownScope (some (.action "explore")) <|
    .instruct (.firstMention "subject" (.inCaller subject)) <|
      .sequentially [
        .expose .reveal (.cards (.slice .top (.lit 1) (.zone .library (some .you)))),
        .doIf
          (.matches (.pro .bare .one) (.hasType .land))
          (.move (.pro .bare .one) .anyLocation (some handZone) .departure (some .anEffect) [])
          (some (.putCounters (.lit 1) (.printed (.named "p1p1Counter"))
            (.laterMention "subject" (.inCaller subject)) .one true))
      ]

theorem explore_def_sound :
    (exploreDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Representative multi-step definition 2: `connive` [CR#701.50a]
(`ownScope` + `inCaller` + `firstMention` / `laterMention` + `draw` + `move` + `doIf`). -/
def conniveDef (subject : NounPhrase) : Action :=
  .ownScope (some (.action "connive")) <|
    .instruct (.firstMention "subject" (.inCaller subject)) <|
      .instruct (.possessorOf .controller .actor) <|
      .sequentially [
        .draw (.exactlyOf (.lit 1)),
        .move (.described (.a .unmarked) (.and [.isCard, .in_ handZone]))
          (handZone) (some graveyardZone) .departure (some .anEffect) [],
        .doIf
          (.did ⟨.actor .you,
            .move (.described (.a .unmarked) (.not (.hasType .land)))
              (handZone) (some graveyardZone) .departure (some .anEffect) [],
            some .thisWay⟩)
          (.putCounters (.lit 1) (.printed (.named "p1p1Counter"))
            (.laterMention "subject" (.inCaller subject)) .one true)
          none
      ]

theorem connive_def_sound :
    (conniveDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Representative multi-step definition 3: `airbend`
(`ownScope` + `inCaller` + `firstMention` + zone-changing `move` + Participial Description
`"the exiled card"`). -/
def airbendDef (subject : NounPhrase) : Action :=
  .ownScope (some (.action "airbend")) <|
    .sequentially [
      .move (.firstMention "subject" (.inCaller subject))
        .anyLocation (some exileZone) .departure (some .anEffect) [],
      .establish
        (.can (.actor .you)
          (.procedure .cast
            (.described (.the .one)
              (.and [.did ⟨.any, .move .which .anyLocation (some exileZone) .departure none [], none⟩,
                .isCard]))
            (.payingInstead (.mana [.simple (.generic 2)])) none none none)
          {})
        none
    ]

theorem airbend_def_sound :
    (airbendDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Zone-change identity clearing (`[CR#400.7,400.7j]`): a post-move `laterMention` of a pre-move
`firstMention` parameter is deterministically refused with `Refusal.bindingIdentity`. -/
def postMoveLaterMentionRefused : Action :=
  .ownScope (some (.action "airbend")) <|
    .sequentially [
      .move (.firstMention "subject" (.inCaller targetCreature))
        .anyLocation (some exileZone) .departure (some .anEffect) [],
      .setStatus .tapped (.laterMention "subject" (.inCaller targetCreature))
    ]

theorem post_move_later_mention_refused :
    postMoveLaterMentionRefused.check (DRS.ofList []) = [.bindingIdentity "subject"] := by decide

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
  .static <|
    .replace
      (.would .any
        (.move (.described (.a .unmarked)
            (.and [.not .isToken, .hasType .creature, .hasPossessor .controller (.described (.a .unmarked) (.isOpponentOf .you))]))
          (.zone .battlefield none) (some graveyardZone) .departure (some .anEffect) []))
      (.instead (.sequentially [
        .move (.pro (.word (.type .creature)) .one) .anyLocation (some exileZone) .departure (some .anEffect) [],
        .createObject (.lit 1)
          (.token (.written ⟨{ colors := [.black], types := [.creature], power := some (.lit 2), toughness := some (.lit 2) }, []⟩) [])
          true none
      ]))
      none .repeatedly none

theorem kalitas_replacement_sound :
    kalitasReplacement.check = [] := by decide

/-- Activated `cost : effect` box across the colon:
`"Sacrifice a creature: Draw cards equal to its power."` -/
def sacrificeDrawActivated : Ability :=
  .activated
    (.perform (.enact (.action "sacrifice")
      (.move aCreature (.zone .battlefield none) (some graveyardZone) .departure (some .anEffect) [])
      none none))
    (.draw (.exactlyOf (.statOf (.stat .power) (.pro .bare .one))))
    none none none none

theorem sacrifice_draw_activated_sound :
    sacrificeDrawActivated.check = [] := by decide

/-- Closed event and deontic head-gating (`semantics-v3.md` §3.6–3.7):
compound actions (`sequentially`) are refused inside `does`, `would`, `did`, and `cant` with
`Refusal.eventHead`, while `ownScope (some (.action name))` is admitted. -/
theorem compound_event_head_refused :
    (GameEvent.does (.actor .you) (.sequentially [.draw (.exactlyOf (.lit 1)), .draw (.exactlyOf (.lit 1))])).check
      (DRS.ofList []) = [.eventHead] := by decide

theorem own_scope_action_event_head_admitted :
    (GameEvent.does (.actor aCreature) (exploreDef aCreature)).check
      (DRS.ofList []) = [] := by decide

/-- Slice A pin (*Weeping Angel*-style combat trigger + attachment/status):
`"Whenever a creature attacks you, tap it and attach target Equipment to that creature."` -/
def weepingAngelCombatPin : Ability :=
  .triggered
    (.does (.actor aCreature) (.attack (some .you))) [] none [] none none none
    (.sequentially [
      .setStatus .tapped (.pro .bare .one),
      .attachment .attached
        (.described (.target (.range (some 1) (some 1))) (.hasSubtype Definitions.equipment))
        (some (.pro (.word (.type .creature)) .one))
    ])

theorem weeping_angel_combat_pin_sound :
    weepingAngelCombatPin.check = [] := by decide

/-- Slice B pin (counters + characteristic modification + pronoun/demonstrative):
`"Double the number of counters on target creature, then it becomes a Dragon in addition to its other types until end of turn."` -/
def sliceBCountersAndCharPin : Ability :=
  .spell none <|
    .sequentially [
      .ownScope (some (.action "double"))
        (.doForEachKind .counterKind (some (.firstMention "double.on" targetCreature))
          .counterKind
          (.putCounters (.statOf .boundCounter (.laterMention "double.on" targetCreature))
            .bound (.laterMention "double.on" targetCreature) .one true)),
      .establish
        (.characteristicChange (.pro .bare .one)
          [.typeLine .adds { subtypes := some [Definitions.dragon] }])
        (some (.between none (some (.endOf .turn none .current))))
    ]

theorem slice_b_counters_and_char_pin_sound :
    sliceBCountersAndCharPin.check = [] := by decide

/-- Slice C pin (library slice + pile split + `pileOf` + `in_` + `theRest`):
`"Reveal the top five cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other into your graveyard."` -/
def sliceCFactOrFictionPin : Ability :=
  .spell none <|
    .sequentially [
      .expose .reveal (.cards (.slice .top (.lit 5) (.zone .library (some .you)))),
      .instruct (.described (.a .unmarked) (.isOpponentOf .you))
        (.separateIntoPiles (.pro .bare .many) 2 []),
      .move (.described .all (.and [.isCard, .in_ (.pile (.pileOf (.counted (.range (some 1) (some 1))) none))]))
        .anyLocation (some handZone) .departure (some .anEffect) [],
      .move (.theRest .pile .one) .anyLocation (some graveyardZone) .departure (some .anEffect) []
    ]

theorem slice_c_fact_or_fiction_pin_sound :
    sliceCFactOrFictionPin.check = [] := by decide

/-- Slice D pin (vote outcome + quality choice + `ofChosen` resolution):
`"Choose a color. Starting with you, each player votes for grace or condemnation. Draw a card for each grace vote."` -/
def sliceDVoteAndChoicePin : Ability :=
  .spell none <|
    .sequentially [
      .choose none (.described (.a .unmarked) (.qualityNoun .color none)) .openly none,
      .instruct (.described .each .anyPlayer)
        (.vote (some .you) .openly (.byLabel ["grace", "condemnation"]) none),
      .draw (.exactlyOf (.votesFor "grace")),
      .move (.described .all (.and [.hasType .creature, .ofChosen .theChoice .color]))
        (.zone .battlefield none) (some handZone) .departure (some .anEffect) []
    ]

theorem slice_d_vote_and_choice_pin_sound :
    sliceDVoteAndChoicePin.check = [] := by decide

/-- Slice E pin (copying + `chooseNewTargets` + damage prevention + `theOutcome .damagePrevented`):
`"Copy target instant or sorcery spell. You may choose new targets for the copy. Prevent the next 3 damage that would be dealt to any target this turn, then draw cards equal to the damage prevented this way."` -/
def sliceECopyAndPreventPin : Ability :=
  .spell none <|
    .sequentially [
      .copy .fromStack
        (.described (.target (.range (some 1) (some 1))) (.or [.hasType .instant, .hasType .sorcery]))
        (.lit 1) [],
      .chooseNewTargets (.pro (.word .copy) .one),
      .establish
        (.damageRule .unattributed .everywhere
          (.prevent (.shield (.lit 3)) (some (.draw (.exactlyOf (.theOutcome .damagePrevented)))))
          .nextTimeOnly)
        (some (.between none (some (.endOf .turn none .current))))
    ]

theorem slice_e_copy_and_prevent_pin_sound :
    sliceECopyAndPreventPin.check = [] := by decide

end Semantics.Check.CorePins

