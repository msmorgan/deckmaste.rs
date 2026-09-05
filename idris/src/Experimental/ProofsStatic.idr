module Experimental.ProofsStatic

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "This creature gets +2/+0 as long as this creature is attacking."
public export
okGetsBattlefieldSubject : Ability
okGetsBattlefieldSubject =
  Static (Macros.onlyWhile (Macros.getsPt Macros.thisCreature (Up (Lit 2))
                                 (Up (Lit 0)))
                           (Matches Macros.thisCreature Attacking))

||| "That creature gets +2/+0 as long as this creature is attacking."
public export
badThatCreatureIsCondSubject : Unspellable Ability (\ok =>
  Static (Macros.onlyWhile (Macros.getsPt (Macros.That (TypeW Creature) OneOf {ok = Builtin.fst ok}) (Up (Lit 2)) (Up (Lit 0))
                                 {ok = Builtin.snd ok})
                           (Matches Macros.thisCreature Attacking)))
badThatCreatureIsCondSubject (Refl, _) impossible

||| "Equipped creature gets +1/+1."
public export
okEquippedCreature : Ability
okEquippedCreature =
  Static (Macros.getsPt (AttachHost Equipped (TypeW Creature)) (Up (Lit 1))
               (Up (Lit 1)))

||| "Equipped land gets +1/+1."
public export
badEquippedLand : Unspellable Ability (\ok =>
  Static (Macros.getsPt (AttachHost Equipped (TypeW Land) {ok = ok}) (Up (Lit 1)) (Up (Lit 1))))
badEquippedLand Oh impossible

||| "Fortified creature gets +1/+1."
public export
badFortifiedCreature : Unspellable Ability (\ok =>
  Static (Macros.getsPt (AttachHost Fortified (TypeW Creature) {ok = ok}) (Up (Lit 1)) (Up (Lit 1))))
badFortifiedCreature Oh impossible

||| "You can't lose the game."
public export
okUntargetedOutcomeGate : Ability
okUntargetedOutcomeGate = Static (Macros.playerCant "LoseGame" You)

||| "Target player can't lose the game."
public export
badTargetedOutcomeGate : Unspellable Ability (\ok =>
  Static (Macros.playerCant "LoseGame" (Macros.target AnyPlayer)) {ut = ok})
badTargetedOutcomeGate Oh impossible

||| "where X is the number of creatures you control."
public export
okSingleStaticXRider : StaticSpec []
okSingleStaticXRider =
  AndAlso Nothing [ Modify Macros.thisCreature Power (Up (LetterVal X))
                  , Modify Macros.thisCreature Toughness (Up (Lit 0))
                  , DefinesLetter X (Macros.countOf Macros.creatureYouControl) ]

public export
badDoubleStaticRider : Unspellable (StaticSpec []) (\ok =>
  AndAlso Nothing [ Modify Macros.thisCreature Power (Up (LetterVal X))
          , Modify Macros.thisCreature Toughness (Up (Lit 0))
          , DefinesLetter X (Macros.countOf Macros.creatureYouControl)
          , DefinesLetter X (Macros.countOf Macros.creature) {ok} ])
badDoubleStaticRider Oh impossible

||| "power and toughness are each equal to the number of creatures you control"
public export
okSelfDefinedPt : StaticSpec []
okSelfDefinedPt =
  DefinesPt Macros.thisCreature BothEach
            (Macros.countOf Macros.creatureYouControl)

public export
badGrantedPtDefinition : Unspellable (StaticSpec []) (\ok =>
  DefinesPt (AttachHost Enchanted (TypeW Creature)) BothEach
            (PlayerStatOf LifeTotal You) {sd = ok})
badGrantedPtDefinition Oh impossible

||| "Creatures you control get +1/+1 until end of turn."
public export
okContinuousClause : Instruction []
okContinuousClause =
  Continuously
               (Macros.getsPt (Macros.allOf Macros.creatureYouControl)
                     (Up (Lit 1)) (Up (Lit 1)))
               (Just Macros.untilEndOfTurn)

public export
badPtDefinitionClause : Unspellable (Instruction []) (\ok =>
  Continuously (DefinesPt Macros.thisCreature BothEach
                          (Macros.countOf Macros.creatureYouControl))
               Nothing {cl = ok})
badPtDefinitionClause Oh impossible

||| "You become the monarch."
public export
okBecomesMonarch : Instruction []
okBecomesMonarch = GainsDesignation You Monarch Instructed Nothing

||| "You become goaded."
public export
badGoadedPlayer : Unspellable (Instruction []) (\ok =>
  GainsDesignation You Goaded Instructed Nothing {sc = ok})
badGoadedPlayer Refl impossible

||| "Each land you control becomes a 2/2 creature. It's still a land."
public export
okStillALand : StaticSpec []
okStillALand =
  Becomes (Macros.allOf Macros.land) Sets
          (Bundle (MkToken (Just (Lit 2 ** Lit 2)) []
                           (MkTypeLine [] [Creature]) [] Nothing) (Just Land))

||| "Target creature becomes a Coward until end of turn. It's still a land."
public export
badStillOnSubtypeSet : Unspellable (StaticSpec []) (\ok =>
  Becomes (Macros.target Macros.creature) Sets (Bundle (MkToken Nothing [] (MkTypeLine [creatureType "Coward"] []) [] Nothing) (Just Land)) {ok = ok})
badStillOnSubtypeSet Oh impossible

public export
badStillAnInstant : Unspellable (StaticSpec []) (\ok =>
  Becomes (Macros.target Macros.creature) Sets (Bundle (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing) (Just Instant)) {ok = ok})
badStillAnInstant Oh impossible

||| "Target creature gets +1/+0."
public export
okSingletonCoordination : StaticSpec []
okSingletonCoordination =
  AndAlso Nothing [ Modify (Macros.target Macros.creature) Power (Up (Lit 1)) ]

||| a coordination of no statements
public export
badEmptyCoordination : Unspellable (StaticSpec []) (\ok =>
  AndAlso Nothing [] {ne = ok})
badEmptyCoordination ItIsSucc impossible

||| "Creatures you control are every creature type."
public export
okSingleExtension : StaticSpec []
okSingleExtension =
  AlsoOffBattlefield
    (Becomes (Macros.allOf Macros.creatureYouControl) Adds
             (EveryTypeOf CreatureSpace))

public export
badDoubleExtension : Unspellable (StaticSpec []) (\ok =>
  AlsoOffBattlefield
    (AlsoOffBattlefield
       (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (Bundle (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing) Nothing))) {nx = ok})
badDoubleExtension Oh impossible

||| "If you would draw a card, draw two cards instead."
public export
okDrawReplacement : StaticSpec []
okDrawReplacement =
  Intercepts (Draws You) [] Nothing (Draw You (Lit 2)) Repeatedly Nothing

||| "If I — would happen, draw a card instead."
public export
badChapterReplacement : Unspellable (StaticSpec []) (\ok =>
  Intercepts (ChapterMark [ChapterI]) [] Nothing (Draw You (Lit 1)) Repeatedly Nothing {ok})
badChapterReplacement Oh impossible

||| "unless you control an artifact"
public export
okUnlessOverNegatedCondition : StaticSpec []
okUnlessOverNegatedCondition =
  Conditionally (AltCost This Nothing)
                (NotCond (Macros.exists (And [Macros.artifact,
                                              HasPossessor ControllerAx You]))) Unless

public export
badUnlessConjunction : Unspellable (StaticSpec []) (\ok =>
  Conditionally (AltCost This Nothing)
                (AndCond [ Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You])
                         , Macros.exists (And [Macros.enchantment, HasPossessor ControllerAx You]) ]) Unless {mk = ok})
badUnlessConjunction MkMarkingOk impossible

||| "This creature blocks an attacking creature."
public export
okCreatureBecomesBlocking : Instruction []
okCreatureBecomesBlocking =
  BecomesBlocking Macros.thisCreature
                  (Macros.a (And [Macros.creature, Attacking]))

||| "Target land blocks an attacking creature.": a land an effect has made a
||| creature blocks [CR#205.1b,509.1a].
public export
okLandBecomesBlocking : Instruction []
okLandBecomesBlocking =
  BecomesBlocking (Macros.target Macros.land)
                  (Macros.a (And [Macros.creature, Attacking]))

||| "This creature blocks target planeswalker."
public export
badBecomesBlockingPlaneswalker : Unspellable (Instruction []) (\ok =>
  BecomesBlocking Macros.thisCreature
                  (Macros.target (HasType Planeswalker)) {dw = ok})
badBecomesBlockingPlaneswalker Oh impossible

||| "this creature gets +1/+0"
public export
okAddPtUpward : StaticSpec []
okAddPtUpward =
  Modify Macros.thisCreature Power (Up (Lit 1))

||| "this creature's mana value is 2 more than it was"; a mana value is read off
||| the mana cost [CR#202.3], not modified in the layer that spells "gets".
public export
badModifyManaValue : Unspellable (StaticSpec []) (\ok =>
  Modify Macros.thisCreature ManaValue (Up (Lit 2)) {ms = ok})
badModifyManaValue Oh impossible

||| "this planeswalker's loyalty becomes 3"; loyalty is the number of loyalty
||| counters on it [CR#306.5c], so a loyalty change belongs to the counter lane.
public export
badSetLoyalty : Unspellable (StaticSpec []) (\ok =>
  Modify Macros.thisPlaneswalker Loyalty (Set (Lit 3)) {ms = ok})
badSetLoyalty Oh impossible

||| "During target opponent's next turn, …"
public export
okSingularNextTurnSpan : Duration []
okSingularNextTurnSpan = DuringNextTurnOf (Macros.target Opponent)

||| "During each opponent's next turn, ..."
public export
badPluralNextTurnSpan : Unspellable (Duration []) (\ok =>
  DuringNextTurnOf (Macros.each Opponent) {one = ok})
badPluralNextTurnSpan Refl impossible

||| "Target opponent skips all combat phases of their next turn."
public export
okSkipDuringTheirNextTurn : Instruction []
okSkipDuringTheirNextTurn =
  Macros.throughout (Skips (Macros.target Opponent) Combat)
                    (DuringNextTurnOf (Macros.That PlayerW OneOf))

||| "You skip all combat phases of their next turn."
public export
badSkipDuringUnboundNextTurn : Unspellable (Instruction []) (\ok =>
  Macros.throughout (Skips You Combat)
                    (DuringNextTurnOf (Macros.That PlayerW OneOf {ok})))
badSkipDuringUnboundNextTurn Refl impossible

||| "Your opponents can't gain life." A rules-meaningful sentence with no
||| printed card on the bench.
public export
opponentsCantGainLife : StaticSpec []
opponentsCantGainLife = Macros.playerCant "GainLife" (PlayerGroup YourOpponents)
