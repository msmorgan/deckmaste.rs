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
        [ when (Primitives.GameEvent.enters thisArtifact none)
            (Primitives.Instruction.sequentially
              [ lookAt (topSlice (.lit 5)),
                offer
                  (Primitives.Instruction.sequentially
                    [ revealCards
                        (fromAmong (exactly 1) (Primitives.Predicate.or [Primitives.Predicate.colorIs .blue, artifact]) them),
                      move (that .card) hand ]) (agent := Primitives.NounPhrase.you),
                move (theRest .object) (onBottomIn .randomOrder) ]),
          activated
            (Primitives.Cost.compound [Primitives.Cost.mana [generic 5, pip .blue], Primitives.Cost.tapSymbol,
                        Primitives.Cost.perform (sacrifice thisArtifact (agent := Primitives.NounPhrase.you))])
            (forbidBeingBlocked (allOf creatureYouControl) (some Primitives.Duration.thisTurn)) ] } }

/-- "two cards at random" from a hand. -/
def twoCardsAtRandom : NounPhrase := countedAtRandom (exactly 2) (Primitives.Predicate.inZone hand)
/-- "two cards" from a hand, chosen. -/
def twoCardsChosen : NounPhrase := counted (exactly 2) (Primitives.Predicate.inZone hand)

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
            (Primitives.Ability.spell none (Primitives.Instruction.sequentially
              [ voteStartingWith Primitives.NounPhrase.you .openly (Primitives.Ballot.byLabel ["death", "torture"]) (agent := (each
                  Primitives.Predicate.anyPlayer)),
                doIf (Primitives.Condition.voteLead "death" false)
                  (sacrifice (aTheirChoice creature) (agent := (each Primitives.Predicate.opponent))),
                doIf (Primitives.Condition.voteLead "torture" true) (loseLife (.lit 4) (agent := (each Primitives.Predicate.opponent))) ]))
                    ] } }

/-- Council's Judgment -/
def councilsJudgment : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Council's Judgment", cost := some [generic 1, pip .white, pip .white],
      types := [.sorcery],
      text :=
        [ abilityWord "will of the council"
            (Primitives.Ability.spell none (Primitives.Instruction.sequentially
              [ voteStartingWith Primitives.NounPhrase.you .openly
                  (Primitives.Ballot.byCandidate (a (Primitives.Predicate.and [ permanent, Primitives.Predicate.not land,
                                           Primitives.Predicate.not (Primitives.Predicate.hasPossessor .controller Primitives.NounPhrase.you) ]))) (agent :=
                                               (each Primitives.Predicate.anyPlayer)),
                exile (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.withMostVotes])) ])) ] } }

/-- Orchard Elemental -/
def orchardElemental : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Orchard Elemental", cost := some [generic 5, pip .green], types := [.creature],
      subtypes := [creatureType "Elemental"],
      text :=
        [ abilityWord "council's dilemma"
            (when (Primitives.GameEvent.enters thisCreature none)
              (Primitives.Instruction.sequentially
                [ voteStartingWith Primitives.NounPhrase.you .openly
                    (Primitives.Ballot.byLabel ["sprout", "harvest"]) (agent := (each Primitives.Predicate.anyPlayer)),
                  Primitives.Instruction.putCounters (times (.lit 2) (Primitives.Amount.votesFor "sprout")) (Primitives.CounterKindSource.printed plusOnePlusOne)
                    thisCreature,
                  gainLife (times (.lit 3) (Primitives.Amount.votesFor "harvest")) (agent := Primitives.NounPhrase.you) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Plea for Power -/
def pleaForPower : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Plea for Power", cost := some [generic 3, pip .blue], types := [.sorcery],
      text :=
        [ abilityWord "will of the council"
            (Primitives.Ability.spell none (Primitives.Instruction.sequentially
              [ voteStartingWith Primitives.NounPhrase.you .openly (Primitives.Ballot.byLabel ["time", "knowledge"]) (agent := (each
                  Primitives.Predicate.anyPlayer)),
                doIf (Primitives.Condition.voteLead "time" false) (Primitives.Instruction.addTurn (.lit 1) (agent := Primitives.NounPhrase.you)),
                doIf (Primitives.Condition.voteLead "knowledge" true) (Primitives.Instruction.draw (.lit 3) (agent := Primitives.NounPhrase.you)) ])) ] } }

/-- Coercive Portal -/
def coercivePortal : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Coercive Portal", cost := some [generic 4], types := [.artifact],
      text :=
        [ abilityWord "will of the council"
            (at_ (Primitives.GameEvent.beginningOf .the .upkeep (Primitives.HeaderPossessor.byPlayer Primitives.NounPhrase.you))
              (Primitives.Instruction.sequentially
                [ voteStartingWith Primitives.NounPhrase.you .openly
                    (Primitives.Ballot.byLabel ["carnage", "homage"]) (agent := (each Primitives.Predicate.anyPlayer)),
                  doIf (Primitives.Condition.voteLead "carnage" false)
                    (Primitives.Instruction.sequentially
                      [ sacrifice thisArtifact (agent := Primitives.NounPhrase.you),
                        destroy (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.not land])) ]),
                  doIf (Primitives.Condition.voteLead "homage" true) (Primitives.Instruction.draw (.lit 1) (agent := Primitives.NounPhrase.you)) ])) ] } }

/-- Custodi Squire -/
def custodiSquire : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Custodi Squire", cost := some [generic 4, pip .white], types := [.creature],
      subtypes := [creatureType "Spirit", creatureType "Cleric"],
      text :=
        [ keyword "Flying",
          abilityWord "will of the council"
            (when (Primitives.GameEvent.enters thisCreature none)
              (Primitives.Instruction.sequentially
                [ voteStartingWith Primitives.NounPhrase.you .openly
                    (Primitives.Ballot.byCandidate
                      (a (Primitives.Predicate.and [ Primitives.Predicate.or [artifact, creature, enchantment],
                                 Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you) ]))) (agent := (each Primitives.Predicate.anyPlayer)),
                  returnTo (allOf (Primitives.Predicate.and [Primitives.Predicate.isCard, Primitives.Predicate.withMostVotes])) hand [] ])) ],
      power := stat 3, toughness := stat 3 } }

/-- Lieutenants of the Guard -/
def lieutenantsOfTheGuard : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Lieutenants of the Guard", cost := some [generic 4, pip .white],
      types := [.creature], subtypes := [creatureType "Human", creatureType "Soldier"],
      text :=
        [ abilityWord "council's dilemma"
            (when (Primitives.GameEvent.enters thisCreature none)
              (Primitives.Instruction.sequentially
                [ voteStartingWith Primitives.NounPhrase.you .openly
                    (Primitives.Ballot.byLabel ["strength", "numbers"]) (agent := (each Primitives.Predicate.anyPlayer)),
                  Primitives.Instruction.putCounters (Primitives.Amount.votesFor "strength") (Primitives.CounterKindSource.printed plusOnePlusOne) thisCreature,
                  create (Primitives.Amount.votesFor "numbers")
                    (creatureToken 1 1 [.white] [creatureType "Soldier"]) ])) ],
      power := stat 2, toughness := stat 2 } }

/-- Truth or Consequences: the secret-council ability alone, as the Idris bench holds it. -/
def truthOrConsequencesVote : Ability :=
  abilityWord "secret council"
    (Primitives.Ability.spell none (Primitives.Instruction.sequentially
      [ vote .secretly (Primitives.Ballot.byLabel ["truth", "consequences"]) (agent := (each Primitives.Predicate.anyPlayer)),
        Primitives.Instruction.draw (Primitives.Amount.votesFor "truth") (agent := Primitives.NounPhrase.you),
        choose (aAtRandom Primitives.Predicate.opponent),
        Primitives.Instruction.dealDamage Primitives.NounPhrase.this (times (.lit 3) (Primitives.Amount.votesFor "consequences")) (that .player) ]))
theorem okTruthOrConsequencesVote : Ability.check [] truthOrConsequencesVote = [] := by decide

/-- Death or Glory -/
def deathOrGlory : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Death or Glory", cost := some [generic 4, pip .white], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.separateIntoPiles
                (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.inZone (graveyardOf Primitives.NounPhrase.you)])) 2 [] (agent := Primitives.NounPhrase.you),
              exile (pileOfChoice anOpponent),
              move (theOther .pile) battlefield ]) ] } }

/-- Steam Augury -/
def steamAugury : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Steam Augury", cost := some [generic 2, pip .blue, pip .red], types := [.instant],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ revealCards (topSlice (.lit 5)),
              Primitives.Instruction.separateIntoPiles them 2 [] (agent := Primitives.NounPhrase.you),
              choose onePile (agent := some anOpponent),
              move (that .pile) hand,
              move (theOther .pile) graveyard ]) ] } }

/-- Do or Die -/
def doOrDie : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Do or Die", cost := some [generic 1, pip .black], types := [.sorcery],
      text :=
        [ Primitives.Ability.spell none (Primitives.Instruction.sequentially
            [ Primitives.Instruction.separateIntoPiles
                (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.hasPossessor .controller (target Primitives.Predicate.anyPlayer)])) 2 [] (agent
                    := Primitives.NounPhrase.you),
              Primitives.Instruction.doAndForbid
                (destroy (allOf (Primitives.Predicate.and [creature, Primitives.Predicate.inPile (pileOfChoice they)])))
                (.action "Regenerate") (themVerbed (.action "Destroy")) ]) ] } }

/-- Liliana of the Veil -/
def lilianaOfTheVeil : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Liliana of the Veil", cost := some [generic 1, pip .black, pip .black],
      supertypes := [.legendary], types := [.planeswalker],
      subtypes := [planeswalkerType "Liliana"],
      text :=
        [ activated (Primitives.Cost.loyaltySymbol (.up 1)) (discard (a (Primitives.Predicate.inZone hand)) (agent := (each
            Primitives.Predicate.anyPlayer))),
          activated (Primitives.Cost.loyaltySymbol (.down 2)) (sacrifice (a creature) (agent := (target
              Primitives.Predicate.anyPlayer))),
          activated (Primitives.Cost.loyaltySymbol (.down 6))
            (Primitives.Instruction.sequentially
              [ Primitives.Instruction.separateIntoPiles
                  (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.hasPossessor .controller (target Primitives.Predicate.anyPlayer)])) 2 []
                      (agent := Primitives.NounPhrase.you),
                sacrifice (allOf (Primitives.Predicate.and [permanent, Primitives.Predicate.inPile (pileOfChoice they)])) (agent := they) ])
                    ],
      loyalty := stat 3 } }

/-- Harness Infinity: the exchange instruction alone, as the Idris bench holds it. -/
def harnessInfinityExchange : Instruction := Primitives.Instruction.exchange (Primitives.Exchanged.zones hand graveyard)
theorem okHarnessInfinityExchange : Instruction.check [] harnessInfinityExchange = [] := by decide

theorem countedAtRandomIsPlural : NounPhrase.plur twoCardsAtRandom = .many := by decide

end Semantics.Cards
