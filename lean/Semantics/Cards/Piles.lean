import Semantics
import Semantics.Macros
import Semantics.Check.Card

/-!
# Semantics.Cards.Piles

Port of `idris/src/Experimental/Cards/Piles.idr`: the printed cards of the Piles family (votes,
piles, and the pronouns that read them), each a `Spelled`.
-/

open Semantics Semantics.Macros

namespace Semantics.Cards

/-- Tezzeret's Gatebreaker -/
def tezzeretsGatebreaker : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tezzeret's Gatebreaker", cost := some [generic 4], types := [.artifact],
      text :=
        [ when (.enters thisArtifact none)
            (.sequentially
              [ lookAt (topSlice (.lit 5)),
                may .you
                  (.sequentially
                    [ revealCards
                        (fromAmong (exactly 1) (.or [.colorIs .blue, artifact]) them),
                      move (that .card) hand ]),
                move (theRest .object) (onBottomIn .randomOrder) ]),
          activated
            (.compound [.mana [generic 5, pip .blue], .tapSymbol,
                        .perform (sacrifice .you thisArtifact)])
            (cantBeBlocked (allOf creatureYouControl) (some .thisTurn)) ] } }

/-- "two cards at random" from a hand. -/
def twoCardsAtRandom : NounPhrase := countedAtRandom (exactly 2) (.inZone hand)
/-- "two cards" from a hand, chosen. -/
def twoCardsChosen : NounPhrase := counted (exactly 2) (.inZone hand)

/-- The at-random mode changes nothing a later pronoun can read. -/
theorem atRandomModeIsAnnouncementNeutral :
    NounPhrase.introduced [] twoCardsAtRandom = NounPhrase.introduced [] twoCardsChosen := by
  rfl

/-- Tyrant's Choice -/
def tyrantsChoice : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Tyrant's Choice", cost := some [generic 1, pip .black], types := [.sorcery],
      text :=
        [ abilityWord "will of the council"
            (.spell none (.sequentially
              [ voteStartingWith .you (each .anyPlayer) .openly (.byLabel ["death", "torture"]),
                if_ (.voteLead "death" false)
                  (sacrifice (each .opponent) (aTheirChoice creature)),
                if_ (.voteLead "torture" true) (losesLife (each .opponent) (.lit 4)) ])) ] } }

/-- Council's Judgment -/
def councilsJudgment : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Council's Judgment", cost := some [generic 1, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ abilityWord "will of the council"
            (.spell none (.sequentially
              [ voteStartingWith .you (each .anyPlayer) .openly
                  (.byCandidate (a (.and [ permanent, .not land,
                                           .not (.hasPossessor .controller .you) ]))),
                exile (allOf (.and [permanent, .withMostVotes])) ])) ] } }

/-- Orchard Elemental -/
def orchardElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Orchard Elemental", cost := some [generic 5, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ abilityWord "council's dilemma"
            (when (.enters thisCreature none)
              (.sequentially
                [ voteStartingWith .you (each .anyPlayer) .openly
                    (.byLabel ["sprout", "harvest"]),
                  .putCounters (times (.lit 2) (.votesFor "sprout")) (.printed plusOnePlusOne)
                    thisCreature,
                  gainsLife .you (times (.lit 3) (.votesFor "harvest")) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Plea for Power -/
def pleaForPower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Plea for Power", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ abilityWord "will of the council"
            (.spell none (.sequentially
              [ voteStartingWith .you (each .anyPlayer) .openly (.byLabel ["time", "knowledge"]),
                if_ (.voteLead "time" false) (.extraTurn .you (.lit 1)),
                if_ (.voteLead "knowledge" true) (.draw .you (.lit 3)) ])) ] } }

/-- Coercive Portal -/
def coercivePortal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coercive Portal", cost := some [generic 4], types := [.artifact],
      text :=
        [ abilityWord "will of the council"
            (at_ (.beginningOf .the .upkeep (.byPlayer .you))
              (.sequentially
                [ voteStartingWith .you (each .anyPlayer) .openly
                    (.byLabel ["carnage", "homage"]),
                  if_ (.voteLead "carnage" false)
                    (.sequentially
                      [ sacrifice .you thisArtifact,
                        destroy (allOf (.and [permanent, .not land])) ]),
                  if_ (.voteLead "homage" true) (.draw .you (.lit 1)) ])) ] } }

/-- Custodi Squire -/
def custodiSquire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Custodi Squire", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Cleric"],
      text :=
        [ keyword "Flying",
          abilityWord "will of the council"
            (when (.enters thisCreature none)
              (.sequentially
                [ voteStartingWith .you (each .anyPlayer) .openly
                    (.byCandidate
                      (a (.and [ .or [artifact, creature, enchantment],
                                 .inZone (graveyardOf .you) ]))),
                  returnTo (allOf (.and [.isCard, .withMostVotes])) hand [] ])) ],
      power := stat 3, toughness := stat 3 } }

/-- Lieutenants of the Guard -/
def lieutenantsOfTheGuard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lieutenants of the Guard", cost := some [generic 4, pip .white],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ abilityWord "council's dilemma"
            (when (.enters thisCreature none)
              (.sequentially
                [ voteStartingWith .you (each .anyPlayer) .openly
                    (.byLabel ["strength", "numbers"]),
                  .putCounters (.votesFor "strength") (.printed plusOnePlusOne) thisCreature,
                  create (.votesFor "numbers")
                    (creatureToken 1 1 [.white] [creatureType "Soldier"]) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Truth or Consequences: the secret-council ability alone, as the Idris bench holds it. -/
def truthOrConsequencesVote : Ability :=
  abilityWord "secret council"
    (.spell none (.sequentially
      [ vote (each .anyPlayer) .secretly (.byLabel ["truth", "consequences"]),
        .draw .you (.votesFor "truth"),
        choose (aAtRandom .opponent),
        .dealDamage .this (times (.lit 3) (.votesFor "consequences")) (that .player) ]))
theorem okTruthOrConsequencesVote : Ability.check [] truthOrConsequencesVote = [] := by decide

/-- Death or Glory -/
def deathOrGlory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death or Glory", cost := some [generic 4, pip .white], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .separateIntoPiles .you
                (allOf (.and [creature, .inZone (graveyardOf .you)])) 2 [],
              exile (pileOfChoice anOpponent),
              move (theOther .pile) battlefield ]) ] } }

/-- Steam Augury -/
def steamAugury : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Steam Augury", cost := some [generic 2, pip .blue, pip .red], types := [.instant],
      text :=
        [ .spell none (.sequentially
            [ revealCards (topSlice (.lit 5)),
              .separateIntoPiles .you them 2 [],
              chooses anOpponent onePile,
              move (that .pile) hand,
              move (theOther .pile) graveyard ]) ] } }

/-- Do or Die -/
def doOrDie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Do or Die", cost := some [generic 1, pip .black], types := [.sorcery],
      text :=
        [ .spell none (.sequentially
            [ .separateIntoPiles .you
                (allOf (.and [creature, .hasPossessor .controller (target .anyPlayer)])) 2 [],
              .cantBe
                (destroy (allOf (.and [creature, .inPile (pileOfChoice they)])))
                (.action "Regenerate") (themVerbed (.action "Destroy")) ]) ] } }

/-- Liliana of the Veil -/
def lilianaOfTheVeil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Liliana of the Veil", cost := some [generic 1, pip .black, pip .black],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [planeswalkerType "Liliana"],
      text :=
        [ activated (.loyaltySymbol (.up 1)) (discard (each .anyPlayer) (a (.inZone hand))),
          activated (.loyaltySymbol (.down 2)) (sacrifice (target .anyPlayer) (a creature)),
          activated (.loyaltySymbol (.down 6))
            (.sequentially
              [ .separateIntoPiles .you
                  (allOf (.and [permanent, .hasPossessor .controller (target .anyPlayer)])) 2 [],
                sacrifice they (allOf (.and [permanent, .inPile (pileOfChoice they)])) ]) ],
      loyalty := stat 3 } }

/-- Harness Infinity: the exchange instruction alone, as the Idris bench holds it. -/
def harnessInfinityExchange : Instruction := .exchange (.zones hand graveyard)
theorem okHarnessInfinityExchange : Instruction.check [] harnessInfinityExchange = [] := by decide

theorem countedAtRandomIsPlural : NounPhrase.plur twoCardsAtRandom = .many := by decide

end Semantics.Cards
