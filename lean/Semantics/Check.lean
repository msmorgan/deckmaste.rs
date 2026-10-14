import Semantics.Check.FactTypes
import Semantics.Check.Facts
import Semantics.Check.Words
import Semantics.Check.Keywords
import Semantics.Check.Refusal
import Semantics.Check.Events
import Semantics.Check.Phrase
import Semantics.Check.EventContext
import Semantics.Check.PhraseRules
import Semantics.Check.Triggers
import Semantics.Check.Abilities
import Semantics.Check.AbilityRules
import Semantics.Check.Card
import Semantics.Check.Rules

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

private def exileZone : ZoneExpr :=
  .zone .exile .bare

private def handZone : ZoneExpr :=
  .zone .hand (.possessedBy .you)

private def graveyardZone : ZoneExpr :=
  .zone .graveyard (.possessedBy .you)

/-- Representative multi-step definition 1: `explore` [CR#701.44a]
(`ownScope` + `inCaller` + `firstMention` / `laterMention` + `doIf` + `putCounters`). -/
def exploreDef (subject : NounPhrase) : Instruction :=
  .ownScope (some (.action "explore")) <|
    .instruct (.inCaller (.firstMention "subject" subject)) <|
      .sequentially [
        .expose .reveal (.cards (.slice .top (.lit 1) (.library (.possessedBy .you)))),
        .doIf
          (.matches (.pro .bare .one) (.hasType .land))
          (.move (.pro .bare .one) .anywhere (some handZone) [] true .departure)
          (some (.putCounters (.lit 1) (.printed (.named "p1p1"))
            (.inCaller (.laterMention "subject" subject))))
      ]

theorem explore_def_sound :
    (exploreDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Representative multi-step definition 2: `connive` [CR#701.50a]
(`ownScope` + `inCaller` + `firstMention` / `laterMention` + `draw` + `move` + `doIf`). -/
def conniveDef (subject : NounPhrase) : Instruction :=
  .ownScope (some (.action "connive")) <|
    .instruct (.inCaller (.firstMention "subject" subject)) <|
      .sequentially [
        .draw (.lit 1),
        .move (.described (.a .unmarked) (.and [.isCard, .inZone handZone]))
          (.zones [handZone]) (some graveyardZone) [] true .departure,
        .doIf
          (.did ⟨.actor .you,
            .move (.described (.a .unmarked) (.not (.hasType .land)))
              (.zones [handZone]) (some graveyardZone) [] true .departure,
            .thisWay⟩)
          (.putCounters (.lit 1) (.printed (.named "p1p1"))
            (.inCaller (.laterMention "subject" subject)))
          none
      ]

theorem connive_def_sound :
    (conniveDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Representative multi-step definition 3: `airbend`
(`ownScope` + `inCaller` + `firstMention` + zone-changing `move` + stamped participle `"the exiled card"`). -/
def airbendDef (subject : NounPhrase) : Instruction :=
  .ownScope (some (.action "airbend")) <|
    .sequentially [
      .move (.inCaller (.firstMention "subject" subject))
        .anywhere (some exileZone) [] true .departure,
      .establish
        (.may (.actor .you)
          (.procedure .cast
            (.pro (.verbed (.action "exile") (some .card) .attributive) .one)
            (.payingInstead (.mana [.simple (.generic 2)])) none none none)
          { rider := .noRider })
        none
    ]

theorem airbend_def_sound :
    (airbendDef targetCreature).check (DRS.ofList []) = [] := by decide

/-- Zone-change identity clearing (`[CR#400.7,400.7j]`): a post-move `laterMention` of a pre-move
`firstMention` parameter is deterministically refused with `Refusal.bindingIdentity`. -/
def postMoveLaterMentionRefused : Instruction :=
  .ownScope (some (.action "airbend")) <|
    .sequentially [
      .move (.inCaller (.firstMention "subject" targetCreature))
        .anywhere (some exileZone) [] true .departure,
      .setStatus .tapped (.inCaller (.laterMention "subject" targetCreature))
    ]

theorem post_move_later_mention_refused :
    postMoveLaterMentionRefused.check (DRS.ofList []) = [.bindingIdentity "subject"] := by decide

/-- Principle A (`NounPhrase.refl`) vs. `J2` (clause-mate exclusion):
`"Target creature deals 2 damage to itself"` succeeds, whereas `"Target creature deals 2 damage to it"`
is refused because the only referent in scope is a clause-mate. -/
theorem reflexive_subject_succeeds :
    (Instruction.instruct targetCreature
      (.dealDamage (.lit 2) (.refl .one) .any)).check (DRS.ofList []) = [] := by decide

theorem clause_mate_pronoun_refused :
    (Instruction.instruct targetCreature
      (.dealDamage (.lit 2) (.pro .bare .one) .any)).check (DRS.ofList []) =
      [.anaphor .bare .one 0] := by decide

/-- Re-mentioned pronoun subject in clause 2 (`"Tap target creature. It deals 2 damage to itself"`):
records `(r.id, .subj)` in `drs.pendingRoles`, licensing `"itself"` and excluding `"it"` in the
object slot. -/
theorem rementioned_subject_licenses_reflexive :
    (Instruction.sequentially [
      .setStatus .tapped targetCreature,
      .instruct (.pro .bare .one) (.dealDamage (.lit 2) (.refl .one) .any)
    ]).check (DRS.ofList []) = [] := by decide

theorem rementioned_subject_excludes_clause_mate_pronoun :
    (Instruction.sequentially [
      .setStatus .tapped targetCreature,
      .instruct (.pro .bare .one) (.dealDamage (.lit 2) (.pro .bare .one) .any)
    ]).check (DRS.ofList []) = [.anaphor .bare .one 0] := by decide

/-- Rule `D′` + Rule `AC>R1` complementary split in the same clause:
Clause 0 introduces subject (`targetCreature`) and object (`anotherTargetCreature`).
Clause 1 resolves `"it"` (`.pro .bare .one`) to the subject via `D′` (setting `pronounResolved := true`),
and resolves `"that creature"` (`.pro (.word (.type .creature)) .one`) to the remaining creature via `AC>R1`. -/
theorem pronoun_and_demonstrative_split :
    (Instruction.sequentially [
      .instruct targetCreature (.dealDamage (.lit 1) anotherTargetCreature .any),
      .instruct (.pro .bare .one)
        (.dealDamage (.lit 2) (.pro (.word (.type .creature)) .one) .any)
    ]).check (DRS.ofList []) = [] := by decide

/-- Pre-event (`would`) stepping (*Kalitas* replacement):
`"If a nontoken creature an opponent controls would die, instead exile that creature and create a 2/2 black Zombie creature token."` -/
def kalitasReplacement : Ability :=
  .static <|
    .instead
      (.would .any
        (.move (.described (.a .unmarked)
            (.and [.not .isToken, .hasType .creature, .hasPossessor .controller (.described (.a .unmarked) .opponent)]))
          (.zones [.zone .battlefield .bare]) (some graveyardZone) [] true .departure))
      (.sequentially [
        .move (.pro (.word (.type .creature)) .one) .anywhere (some exileZone) [] true .departure,
        .createObject (.lit 1)
          (.token (.written ⟨{ colors := [.black], types := [.creature], power := some (.lit 2), toughness := some (.lit 2) }⟩) [])
          true
      ])
      none

theorem kalitas_replacement_sound :
    kalitasReplacement.check = [] := by decide

/-- Activated `cost : effect` box across the colon:
`"Sacrifice a creature: Draw cards equal to its power."` -/
def sacrificeDrawActivated : Ability :=
  .activated
    (.perform (.enact (.action "sacrifice")
      (.move aCreature (.zones [.zone .battlefield .bare]) (some graveyardZone) [] true .departure)
      none none))
    (.draw (.statOf (.stat .power) (.pro .bare .one)))
    none none none none

theorem sacrifice_draw_activated_sound :
    sacrificeDrawActivated.check = [] := by decide

/-- Closed event and deontic head-gating (`semantics-v3.md` §3.6–3.7):
compound instructions (`sequentially`) are refused inside `does`, `would`, `did`, and `cant` with
`Refusal.eventHead`, while `ownScope (some (.action name))` is admitted. -/
theorem compound_event_head_refused :
    (GameEvent.does (.actor .you) (.sequentially [.draw (.lit 1), .draw (.lit 1)])).check
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
        (.described (.target (.range (some 1) (some 1))) (.hasSubtype (.of .artifact "Equipment")))
        (some (.pro (.word (.type .creature)) .one))
    ])

theorem weeping_angel_combat_pin_sound :
    weepingAngelCombatPin.check = [] := by decide

/-- Slice B pin (counters + characteristic modification + pronoun/demonstrative):
`"Double the number of counters on target creature, then it becomes a Dragon in addition to its other types until end of turn."` -/
def sliceBCountersAndCharPin : Ability :=
  .spell none <|
    .sequentially [
      .doubleCounters targetCreature,
      .establish
        (.characteristicChange (.pro .bare .one)
          [.typeLine .adds { subtypes := some [.of .creature "Dragon"] }])
        (some (.between none (some (.endOf .turn none .current))))
    ]

theorem slice_b_counters_and_char_pin_sound :
    sliceBCountersAndCharPin.check = [] := by decide

/-- Slice C pin (library slice + pile split + `pileOf` + `inPile` + `theRest`):
`"Reveal the top five cards of your library. An opponent separates those cards into two piles. Put one pile into your hand and the other into your graveyard."` -/
def sliceCFactOrFictionPin : Ability :=
  .spell none <|
    .sequentially [
      .expose .reveal (.cards (.slice .top (.lit 5) (.library (.possessedBy .you)))),
      .instruct (.described (.a .unmarked) .opponent)
        (.separateIntoPiles (.pro .bare .many) 2 []),
      .move (.described .all (.and [.isCard, .inPile (.pileOf (.counted (.range (some 1) (some 1))) none)]))
        .anywhere (some handZone) [] true .departure,
      .move (.theRest .pile .one) .anywhere (some graveyardZone) [] true .departure
    ]

theorem slice_c_fact_or_fiction_pin_sound :
    sliceCFactOrFictionPin.check = [] := by decide

end Semantics.Check.CorePins
