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
            (.sequence
              [ lookAt (topSlice (.lit 5)),
                offer
                  (.sequence
                    [ revealCards
                        (fromAmong (exactly 1) (.or [.colorIs .blue, artifact]) them),
                      move (that .card) hand ]) (agent := .you),
                move (theRest .object) (onBottomIn .randomOrder) ]),
          activated
            (.compound [.mana [generic 5, pip .blue], .tapSymbol,
                        .perform (sacrifice thisArtifact (agent := .you))])
            (forbidBeingBlocked (allOf creatureYouControl) (some .thisTurn)) ] } }

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
            (.spell none (.sequence
              [ voteStartingWith .you .openly (.byLabel ["death", "torture"]) (agent := (each
                  .anyPlayer)),
                doIf (.voteLead "death" false)
                  (sacrifice (aTheirChoice creature) (agent := (each .opponent))),
                doIf (.voteLead "torture" true) (loseLife (.lit 4) (agent := (each .opponent))) ]))
                    ] } }

/-- Council's Judgment -/
def councilsJudgment : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Council's Judgment", cost := some [generic 1, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ abilityWord "will of the council"
            (.spell none (.sequence
              [ voteStartingWith .you .openly
                  (.byCandidate (a (.and [ permanent, .not land,
                                           .not (.hasPossessor .controller .you) ]))) (agent :=
                                               (each .anyPlayer)),
                exile (allOf (.and [permanent, .withMostVotes])) ])) ] } }

/-- Orchard Elemental -/
def orchardElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Orchard Elemental", cost := some [generic 5, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ abilityWord "council's dilemma"
            (when (.enters thisCreature none)
              (.sequence
                [ voteStartingWith .you .openly
                    (.byLabel ["sprout", "harvest"]) (agent := (each .anyPlayer)),
                  .putCounters (times (.lit 2) (.votesFor "sprout")) (.printed plusOnePlusOne)
                    thisCreature,
                  gainLife (times (.lit 3) (.votesFor "harvest")) (agent := .you) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Plea for Power -/
def pleaForPower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Plea for Power", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ abilityWord "will of the council"
            (.spell none (.sequence
              [ voteStartingWith .you .openly (.byLabel ["time", "knowledge"]) (agent := (each
                  .anyPlayer)),
                doIf (.voteLead "time" false) (.addTurn (.lit 1) (agent := .you)),
                doIf (.voteLead "knowledge" true) (.draw (.lit 3) (agent := .you)) ])) ] } }

/-- Coercive Portal -/
def coercivePortal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coercive Portal", cost := some [generic 4], types := [.artifact],
      text :=
        [ abilityWord "will of the council"
            (at_ (.beginningOf .the .upkeep (.byPlayer .you))
              (.sequence
                [ voteStartingWith .you .openly
                    (.byLabel ["carnage", "homage"]) (agent := (each .anyPlayer)),
                  doIf (.voteLead "carnage" false)
                    (.sequence
                      [ sacrifice thisArtifact (agent := .you),
                        destroy (allOf (.and [permanent, .not land])) ]),
                  doIf (.voteLead "homage" true) (.draw (.lit 1) (agent := .you)) ])) ] } }

/-- Custodi Squire -/
def custodiSquire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Custodi Squire", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Cleric"],
      text :=
        [ keyword "Flying",
          abilityWord "will of the council"
            (when (.enters thisCreature none)
              (.sequence
                [ voteStartingWith .you .openly
                    (.byCandidate
                      (a (.and [ .or [artifact, creature, enchantment],
                                 .inZone (graveyardOf .you) ]))) (agent := (each .anyPlayer)),
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
              (.sequence
                [ voteStartingWith .you .openly
                    (.byLabel ["strength", "numbers"]) (agent := (each .anyPlayer)),
                  .putCounters (.votesFor "strength") (.printed plusOnePlusOne) thisCreature,
                  create (.votesFor "numbers")
                    (creatureToken 1 1 [.white] [creatureType "Soldier"]) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Truth or Consequences: the secret-council ability alone, as the Idris bench holds it. -/
def truthOrConsequencesVote : Ability :=
  abilityWord "secret council"
    (.spell none (.sequence
      [ vote .secretly (.byLabel ["truth", "consequences"]) (agent := (each .anyPlayer)),
        .draw (.votesFor "truth") (agent := .you),
        choose (aAtRandom .opponent),
        .dealDamage .this (times (.lit 3) (.votesFor "consequences")) (that .player) ]))
theorem okTruthOrConsequencesVote : Ability.check [] truthOrConsequencesVote = [] := by decide

/-- Death or Glory -/
def deathOrGlory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death or Glory", cost := some [generic 4, pip .white], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ .separateIntoPiles
                (allOf (.and [creature, .inZone (graveyardOf .you)])) 2 [] (agent := .you),
              exile (pileOfChoice anOpponent),
              move (theOther .pile) battlefield ]) ] } }

/-- Steam Augury -/
def steamAugury : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Steam Augury", cost := some [generic 2, pip .blue, pip .red], types := [.instant],
      text :=
        [ .spell none (.sequence
            [ revealCards (topSlice (.lit 5)),
              .separateIntoPiles them 2 [] (agent := .you),
              choose onePile (agent := some anOpponent),
              move (that .pile) hand,
              move (theOther .pile) graveyard ]) ] } }

/-- Do or Die -/
def doOrDie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Do or Die", cost := some [generic 1, pip .black], types := [.sorcery],
      text :=
        [ .spell none (.sequence
            [ .separateIntoPiles
                (allOf (.and [creature, .hasPossessor .controller (target .anyPlayer)])) 2 [] (agent
                    := .you),
              .doAndForbid
                (destroy (allOf (.and [creature, .inPile (pileOfChoice they)])))
                (.action "Regenerate") (themVerbed (.action "Destroy")) ]) ] } }

/-- Liliana of the Veil -/
def lilianaOfTheVeil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Liliana of the Veil", cost := some [generic 1, pip .black, pip .black],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [planeswalkerType "Liliana"],
      text :=
        [ activated (.loyaltySymbol (.up 1)) (discard (a (.inZone hand)) (agent := (each
            .anyPlayer))),
          activated (.loyaltySymbol (.down 2)) (sacrifice (a creature) (agent := (target
              .anyPlayer))),
          activated (.loyaltySymbol (.down 6))
            (.sequence
              [ .separateIntoPiles
                  (allOf (.and [permanent, .hasPossessor .controller (target .anyPlayer)])) 2 []
                      (agent := .you),
                sacrifice (allOf (.and [permanent, .inPile (pileOfChoice they)])) (agent := they) ])
                    ],
      loyalty := stat 3 } }

/-- Harness Infinity: the exchange instruction alone, as the Idris bench holds it. -/
def harnessInfinityExchange : Instruction := .exchange (.zones hand graveyard)
theorem okHarnessInfinityExchange : Instruction.check [] harnessInfinityExchange = [] := by decide

theorem countedAtRandomIsPlural : NounPhrase.plur twoCardsAtRandom = .many := by decide

end Semantics.Cards
