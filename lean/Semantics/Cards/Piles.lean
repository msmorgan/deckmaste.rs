import Semantics
import Semantics.Definitions
import Semantics.Check.Card

/-!
# Semantics.Cards.Piles

Port of `idris/src/Experimental/Cards/Piles.idr`: the printed cards of the Piles family (votes,
piles, and the pronouns that read them), each a `Spelled`.
-/

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Cards

/-- Tezzeret's Gatebreaker -/
def tezzeretsGatebreaker : Spelled := spelled <| singleFacedCard
  { name := "Tezzeret's Gatebreaker", cost := [generic 4], types := [.artifact],
    text :=
      [ when (does thisArtifact enter)
          (sequentially
            [ lookAt (topSlice 5),
              may
                (sequentially
                  [ reveal
                      (fromAmong (exactly 1) (or [hasColor .blue, isArtifact]) them),
                    putInto (that card) hand ]),
              putInto (theRest .object) onBottom (riders := [(inOrder .randomOrder)]) ]),
        activated
          (compound [[generic 5, pip .blue], tapSymbol,
                      (sacrifice thisArtifact)])
          (establishThroughout (cant any (block (allOf (and [isCreature, youControl]))))
             thisTurn) ] }

/-- "two cards at random" from a hand. -/
def twoCardsAtRandom : NounPhrase := countedAtRandom (exactly 2) (cardIn hand)
/-- "two cards" from a hand, chosen. -/
def twoCardsChosen : NounPhrase := counted (exactly 2) (cardIn hand)

/-- The at-random mode changes nothing a later pronoun can read. -/
theorem atRandomModeIsAnnouncementNeutral :
    (stepNounPhrase 64 (DRS.ofList []) .obj true twoCardsAtRandom).1 =
      (stepNounPhrase 64 (DRS.ofList []) .obj true twoCardsChosen).1 := by
  rfl

/-- Tyrant's Choice -/
def tyrantsChoice : Spelled := spelled <| singleFacedCard
  { name := "Tyrant's Choice", cost := [generic 1, pip .black], types := [.sorcery],
    text :=
      [ willOfTheCouncil
          (spell (sequentially
            [ instruct eachPlayer (voteStartingWith you .openly (byLabel ["death", "torture"])),
              doIf (voteLead "death" false)
                (instruct eachOpponent (sacrifice (aTheirChoice isCreature))),
              doIf (voteLead "torture" true) (instruct eachOpponent (loseLife 4)) ]))
                  ] }

/-- Council's Judgment -/
def councilsJudgment : Spelled := spelled <| singleFacedCard
  { name := "Council's Judgment", cost := [generic 1, pip .white, pip .white],
    types := [.sorcery],
    text :=
      [ willOfTheCouncil
          (spell (sequentially
            [ instruct eachPlayer (voteStartingWith you .openly
                (byCandidate (a (and [ isPermanent, non isLand,
                                         youDontControl ])))),
              exile (allOf (and [isPermanent, withMostVotes])) ])) ] }

/-- Orchard Elemental -/
def orchardElemental : Spelled := spelled <| singleFacedCard
  { name := "Orchard Elemental", cost := [generic 5, pip .green], types := [.creature],
    subtypes := [elemental],
    text :=
      [ councilsDilemma
          (when (does thisCreature enter)
            (sequentially
              [ instruct eachPlayer (voteStartingWith you .openly
                  (byLabel ["sprout", "harvest"])),
                putCounters (times 2 (votesFor "sprout")) p1p1Counter
                  thisCreature,
                gainLife (times 3 (votesFor "harvest")) ])) ],
    power := stat 2, toughness := stat 2 }

/-- Plea for Power -/
def pleaForPower : Spelled := spelled <| singleFacedCard
  { name := "Plea for Power", cost := [generic 3, pip .blue], types := [.sorcery],
    text :=
      [ willOfTheCouncil
          (spell (sequentially
            [ instruct eachPlayer (voteStartingWith you .openly (byLabel ["time", "knowledge"])),
              doIf (voteLead "time" false) (extraTurn 1),
              doIf (voteLead "knowledge" true) (draw 3) ])) ] }

/-- Coercive Portal -/
def coercivePortal : Spelled := spelled <| singleFacedCard
  { name := "Coercive Portal", cost := [generic 4], types := [.artifact],
    text :=
      [ willOfTheCouncil
          (at_ (beginningOfYour .upkeep)
            (sequentially
              [ instruct eachPlayer (voteStartingWith you .openly
                  (byLabel ["carnage", "homage"])),
                doIf (voteLead "carnage" false)
                  (sequentially
                    [ sacrifice thisArtifact,
                      destroy (allOf (and [isPermanent, non isLand])) ]),
                doIf (voteLead "homage" true) (draw 1) ])) ] }

/-- Custodi Squire -/
def custodiSquire : Spelled := spelled <| singleFacedCard
  { name := "Custodi Squire", cost := [generic 4, pip .white], types := [.creature],
    subtypes := [spirit, cleric],
    text :=
      [ flying,
        willOfTheCouncil
          (when (does thisCreature enter)
            (sequentially
              [ instruct eachPlayer (voteStartingWith you .openly
                  (byCandidate
                    (a (and [ or [isArtifact, isCreature, isEnchantment],
                               in_ yourGraveyard ])))),
                returnToHand (allOf (and [isCard, withMostVotes])) ])) ],
    power := stat 3, toughness := stat 3 }

/-- Lieutenants of the Guard -/
def lieutenantsOfTheGuard : Spelled := spelled <| singleFacedCard
  { name := "Lieutenants of the Guard", cost := [generic 4, pip .white],
    types := [.creature], subtypes := [human, soldier],
    text :=
      [ councilsDilemma
          (when (does thisCreature enter)
            (sequentially
              [ instruct eachPlayer (voteStartingWith you .openly
                  (byLabel ["strength", "numbers"])),
                putCounters (votesFor "strength") p1p1Counter thisCreature,
                createToken (creatureToken 1 1 [.white] [soldier]) (count :=
                                                                      votesFor "numbers") ])) ],
    power := stat 2, toughness := stat 2 }

/-- Truth or Consequences: the secret-council ability alone, as the Idris bench holds it. -/
def truthOrConsequencesVote : Ability :=
  secretCouncil
    (spell (sequentially
      [ instruct eachPlayer (vote .secretly (byLabel ["truth", "consequences"])),
        draw (votesFor "truth"),
        choose (aAtRandom isOpponent),
        instruct this_
          (dealDamage (times 3 (votesFor "consequences")) (that player)) ]))
theorem okTruthOrConsequencesVote : Ability.check (DRS.ofList []) truthOrConsequencesVote = [] := by decide

/-- Death or Glory -/
def deathOrGlory : Spelled := spelled <| singleFacedCard
  { name := "Death or Glory", cost := [generic 4, pip .white], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ separateIntoPiles
              (allOf (and [isCreature, in_ yourGraveyard])) 2,
            exile (pileOfChoice anOpponent),
            returnToBattlefield (theOther .pile) ]) ] }

/-- Steam Augury -/
def steamAugury : Spelled := spelled <| singleFacedCard
  { name := "Steam Augury", cost := [generic 2, pip .blue, pip .red], types := [.instant],
    text :=
      [ spell (sequentially
          [ reveal (topSlice 5),
            separateIntoPiles them 2,
            instruct anOpponent (choose onePile),
            putInto (that .pile) hand,
            putInto (theOther .pile) graveyard ]) ] }

/-- Do or Die -/
def doOrDie : Spelled := spelled <| singleFacedCard
  { name := "Do or Die", cost := [generic 1, pip .black], types := [.sorcery],
    text :=
      [ spell (sequentially
          [ separateIntoPiles
              (allOf (and [isCreature, controlledBy (target isPlayer)])) 2,
            destroyCantRegenerate
              (allOf (and [isCreature, inPile (pileOfChoice they)])) ]) ] }

/-- Liliana of the Veil -/
def lilianaOfTheVeil : Spelled := spelled <| singleFacedCard
  { name := "Liliana of the Veil", cost := [generic 1, pip .black, pip .black],
    supertypes := [.legendary], types := [.planeswalker],
    subtypes := [liliana],
    text :=
      [ activated (loyaltySymbol (up 1))
          (instruct eachPlayer (discard (a (cardIn hand)))),
        activated (loyaltySymbol (down 2)) (instruct (target isPlayer) (sacrifice (a isCreature))),
        activated (loyaltySymbol (down 6))
          (sequentially
            [ separateIntoPiles
                (allOf (and [isPermanent, controlledBy (target isPlayer)])) 2,
              instruct they (sacrifice (allOf (and [isPermanent, inPile (pileOfChoice they)]))) ])
                  ],
    loyalty := stat 3 }

/-- Harness Infinity: the exchange action alone, as the Idris bench holds it. -/
def harnessInfinityExchange : Action := exchange (zones hand graveyard)
theorem okHarnessInfinityExchange : Action.check (DRS.ofList []) harnessInfinityExchange = [] := by decide

theorem countedAtRandomIsPlural : NounPhrase.plur twoCardsAtRandom = .many := by decide

end Semantics.Cards
