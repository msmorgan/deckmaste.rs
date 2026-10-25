import Semantics.Definitions
import Semantics.Check.Card
import Semantics.Cards.Keyword

/-!
# Semantics.Proofs.KeywordAbilityCensus

The keyword abilities `semantics-v2-keyword-ability-census` bodied from vocabulary that already
existed: saddle and teamwork over a group bounded by its total power (`SliceCount.totalling`, as
crew's cost is written), awaken as an alternative cost and a spell ability, and indestructible as
a prohibition over the destroy deed. Each definition is written as its declaration builds it, checks.
`lean-v3-keyword-bodies-and-catalog-sync` bodied frenzy, freerunning, encore, paradigm, partner with
and vigilance the same way; their pins follow the census's.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.KeywordAbilityCensus

/-- "Tap any number of other untapped creatures you control with total power 2 or greater: This
permanent becomes saddled until end of turn. Activate only as a sorcery." [CR#702.171a,702.171b] -/
def saddleDefinition : List Ability :=
  [ .activated
      (.perform (tap (withTotal (.stat .power) .atLeast 2
        (allOf (.and [isUntapped, and [isCreature, youControl, isOther]])))))
      (.gainDesignation thisPermanent (.named "saddled" (.heldBy .object)) (some untilEndOfTurn)) (some asSorcery) none none none ]

/-- "Saddle 2" with its Definition checks [CR#702.171a]. -/
theorem okSaddleDefinition :
    Ability.check (DRS.ofList []) (.keyword (.named "saddle") [.number 2] saddleDefinition)
      = [] := by
  decide

/-- Saddling taps other creatures: a total over "creatures you control" with no "other" may
include the saddled permanent itself, and is a different cost. The bound is still an amount: an
unresolved "its power" is refused. -/
theorem badSaddleUnresolvedBound :
    Cost.check (DRS.ofList []) (.perform (tap (withTotal (.stat .power) .atLeast
      (powerOf (that (.type .creature)))
      (allOf (.and [isUntapped, and [isCreature, youControl, isOther]]))))) =
      [.anaphor (.word (.type .creature)) .one 0] := by
  decide

/-- "As an additional cost to cast this spell, you may tap any number of creatures you control
with total power 2 or more." [CR#702.194a] -/
def teamworkDefinition : List Ability :=
  [ .static (.addedCost
      (.perform (tap (withTotal (.stat .power) .atLeast 2 (allOf (and [isCreature, youControl])))))
      true .once) ]

/-- "Teamwork 2" with its Definition checks
[CR#702.194a]. -/
theorem okTeamworkDefinition :
    Ability.check (DRS.ofList []) (.keyword (.named "teamwork") [.number 2] teamworkDefinition)
      = [] := by
  decide

/-- "{5}{U}" -/
def awakenCost : Cost := .mana [generic 5, pip .blue]

/-- "You may pay {5}{U} rather than pay this spell's mana cost as you cast this spell" and "If
this spell's awaken cost was paid, put four +1/+1 counters on target land you control. That land
becomes a 0/0 Elemental creature with haste. It's still a land." [CR#702.113a] -/
def awakenDefinition : List Ability :=
  [ .static (.altCost .this (some awakenCost)),
    .spell none
      (doIf (costWasPaid (.byKeyword (.named "awaken")) none .this)
        (.sequentially
          [ .putCounters 4 (.printed p1p1Counter)
              (target (.and [isLand, .hasPossessor .controller .you])),
            .establish
              (.characteristicChange (that (.type .land))
                [ .typeLine .adds { types := some [.creature], subtypes := some [elemental] },
                  .stat .sets .power 0, .stat .sets .toughness 0,
                  .addedAbilities [keyword (.named "haste")] ])
              none ])) ]

/-- "Awaken 4—{5}{U}" with its Definition checks
[CR#702.113a]. -/
theorem okAwakenDefinition :
    Ability.check (DRS.ofList []) (.keyword (.named "awaken") [.number 4, .cost awakenCost]
        awakenDefinition)
      = [] := by
  decide

/-- "A permanent with indestructible can't be destroyed." [CR#702.12b] -/
def indestructibleDefinition : List Ability :=
  [ .static (cant any (destroy thisPermanent)) ]

/-- Indestructible with its Definition checks
[CR#702.12b]. -/
theorem okIndestructibleDefinition :
    Ability.check (DRS.ofList []) (.keyword (.named "indestructible") [] indestructibleDefinition)
      = [] := by
  decide

/-- What is destroyed is a permanent [CR#701.8a]: a player can't be named in the destroy deed's
patient role. -/
theorem badDestroyedPlayer :
    (StaticSpec.check (DRS.ofList []) (cant any (destroy .you))).isEmpty = false := by
  decide

/-- Haste's body checks: its creature can attack, and its controller can activate its
abilities with {T} or {Q} in their costs, as though it had been under its controller's control
continuously since their most recent turn began [CR#702.10b,702.10c,302.6]. -/
theorem okHasteDefinition :
    (match haste with | .keyword _ _ body => body.length | _ => 0) = 2 ∧
      Ability.check (DRS.ofList []) haste = [] := by decide

/-! ## Bodies of `lean-v3-keyword-bodies-and-catalog-sync` -/

/-- The static ability a keyword's body holds first, if it is one. -/
private def firstStatic? : Ability → Option StaticSpec
  | .keyword _ _ (.static spec :: _) => some spec
  | _ => none

/-- "Frenzy 1": one triggered ability, on this creature's attacking and not being blocked
[CR#702.68a,509.1h], and it checks. -/
theorem okFrenzyDefinition :
    (match frenzy 1 with
      | .keyword _ _ [.triggered (.does (.actor _) (.attackUnblocked none)) [] none [] none none none _]
        => true
      | _ => false) = true ∧
      Ability.check (DRS.ofList []) (frenzy 1) = [] := by decide

/-- "Freerunning {1}{U}": an alternative cost [CR#118.9] that applies only while its condition
holds [CR#702.173a], and it checks. -/
theorem okFreerunningDefinition :
    (match firstStatic? (freerunning [generic 1, pip .blue]) with
      | some (.conditional (.altCost _ (some _)) (.did _) _) => true
      | _ => false) = true ∧
      Ability.check (DRS.ofList []) (freerunning [generic 1, pip .blue]) = [] := by decide

/-- "Encore {3}{B}": one activated ability, activated only as a sorcery [CR#702.141a], and it
checks. -/
theorem okEncoreDefinition :
    (match encore [generic 3, pip .black] with
      | .keyword _ _ [.activated (.compound [_, .perform _]) _ (some _) none none none] => true
      | _ => false) = true ∧
      Ability.check (DRS.ofList []) (encore [generic 3, pip .black]) = [] := by decide

/-- "Partner with Blaring Recruiter": the one ability of the game partner with represents, the
entering trigger, in which target player may search their library for the named card
[CR#702.124j], and it checks. -/
theorem okPartnerWithDefinition :
    (match partnerWith (named "Blaring Recruiter") with
      | .keyword _ _
          [.triggered _ [] none [] none none none
            (.instruct (.described (.target _) _)
              (.withContinuation .optional
                (.sequentially [.ownScope (some (.action "search")) _, _, _, _]) none none))] =>
        true
      | _ => false) = true ∧
      Ability.check (DRS.ofList []) (partnerWith (named "Blaring Recruiter")) = [] := by decide

/-- Vigilance forbids only the tap its creature's attacking causes [CR#702.20b,508.1f]: the
prohibition carries that cause, and it checks. -/
theorem okVigilanceDefinition :
    (match firstStatic? vigilance with
      | some (.cant .any (.ownScope (some (.action "tap")) _)
          { cause := some (.event (.does (.actor _) (.attack none))), .. }) => true
      | _ => false) = true ∧
      Ability.check (DRS.ofList []) vigilance = [] := by decide

/-- Paradigm is two spell abilities [CR#702.192a], the first creating a delayed triggered ability
for the rest of the game whose copy is created in exile, and it checks. -/
theorem okParadigmDefinition :
    (match paradigm with
      | .keyword _ _
          [ .spell none
              (.doIf _
                (.delay _ [] (some _) (.sequentially [.copy (.inZone .exile) _ _ _, _])) none _),
            .spell none _ ] => true
      | _ => false) = true ∧
      Ability.check (DRS.ofList []) paradigm = [] := by decide

/-- Whether a keyword Definition's body is empty. -/
private def bodiless : Ability → Bool
  | .keyword _ _ [] => true
  | _ => false

/-- The keywords whose rule the CR states as a rule of the game reading the keyword, not as an
ability of its object, hold no body (`lean/CONTRACTS.md`, "Keyword abilities with no body"):
their rules are rows, the combat damage turn-based actions [CR#703.4k,703.4m], deathtouch's and
start your engines!'s state-based actions [CR#704.5h,704.5aa], phasing's turn-based action
[CR#703.4a] and toxic's damage result [CR#702.164c], or flying's rule [CR#702.17b]; or they are
Commander deck construction [CR#702.124a]. -/
theorem intentionallyBodiless :
    [ firstStrike, doubleStrike, trample, deathtouch, banding, bandsWithOther isCreature, reach,
      phasing, startYourEngines, toxic 1, partner, partnerCharacterSelect,
      partnerFatherAndSon, partnerFriendsForever, partnerSurvivors, chooseABackground,
      doctorsCompanion ].all bodiless = true := by decide

/-- "Assist": the chosen player pays generic mana for the spell, one generic mana each
[CR#702.132a], and it checks. -/
theorem okAssistDefinition :
    (match assist with
      | .keyword _ _ [.static (.addedCost _ true .once),
          .static (.paySubstitute _ ⟨.instruct _ _, [.generic]⟩)] => true
      | _ => false) = true ∧
      Ability.check (DRS.ofList []) assist = [] := by decide

/-- The intervening condition and the payment of echo's upkeep trigger. -/
private def echoParts? : Ability → Option (Condition × Option NounPhrase × Option KeywordLabel)
  | .keyword _ _
      [.triggered _ _ _ _ _ _ (some condition)
        (.withContinuation .optional (.pay _ _ _ whose keyword) none _)] =>
    some (condition, whose, keyword)
  | _ => none

/-- The keyword cost a replacement of a payment watches. -/
private def replacedPayment? : Ability → Option KeywordLabel
  | .static (.replace (.would _ (.pay _ _ _ _ keyword)) _ _ _ _) => keyword
  | _ => none

/-- "Echo {2}{W}" [CR#702.30a]: it triggers only if this permanent hasn't been under your control
continuously since the beginning of your previous upkeep, and its payment is of this permanent's
echo cost, the payment *Thick-Skinned Goblin*'s replacement watches. -/
theorem okEchoDefinition :
    echoParts? (echo [generic 2, pip .white]) =
        some
          (not (heldSince (is thisPermanent youControl)
              (.beginningOf .upkeep (some .you) (.previous (some .you)))),
            some thisPermanent, some (.named "echo")) ∧
      replacedPayment? Cards.thickSkinnedGoblinEchoPayment = some (.named "echo") := ⟨rfl, rfl⟩

theorem okEchoChecks : Ability.check (DRS.ofList []) (echo [generic 2, pip .white]) = [] := by
  decide

end Semantics.Proofs.KeywordAbilityCensus
