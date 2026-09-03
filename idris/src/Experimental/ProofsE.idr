module Experimental.ProofsE

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Creatures you control have flying."
public export
okBattlefieldFlying : StaticEffect []
okBattlefieldFlying =
  Gains (Macros.allOf Macros.creatureYouControl)
        (KeywordAbility "Flying" Nothing Nothing)

||| "Creatures you control have convoke."
public export
badBattlefieldConvoke : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.allOf Macros.creatureYouControl) (KeywordAbility "Convoke" Nothing Nothing) {ok})
badBattlefieldConvoke Oh impossible

||| "power and toughness are each equal to the number of creatures you control"
public export
okSelfDefinedPt : StaticEffect []
okSelfDefinedPt =
  DefinesPt Macros.thisCreature BothEach
            (Macros.countOf Macros.creatureYouControl)

public export
badGrantedPtDefinition : Unspellable (StaticEffect []) (\ok =>
  DefinesPt (AttachHost Enchanted (TypeW Creature)) BothEach
            (PlayerStatOf LifeTotal You) {sd = ok})
badGrantedPtDefinition Oh impossible


||| a white creature face reading "Protection from red"
public export
okProtectionOnCreature : Card
okProtectionOnCreature =
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Creature])
       [KeywordAbility "Protection" (Just (ParamQuality (ColorIs Red))) Nothing]
       (Just (2, 2))

public export
badStarlessDefinedPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature])
       [Static (DefinesPt Macros.thisCreature BothEach
                          (Macros.countOf Macros.creatureYouControl))]
       (Just (2, 2)) {fl = ok})
badStarlessDefinedPt (MkFaceLaws {bx = MkCardBox}) impossible


||| "Creatures you control get +1/+1 until end of turn."
public export
okContinuousClause : Effect []
okContinuousClause =
  Continuously {ts = StaticFirstDone}
               (Gets Adds (Macros.allOf Macros.creatureYouControl)
                     (PtUp (Lit 1)) (PtUp (Lit 1)))
               (Just Macros.untilEndOfTurn)

public export
badPtDefinitionClause : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (DefinesPt Macros.thisCreature BothEach
                          (Macros.countOf Macros.creatureYouControl))
               Nothing {cl = ok})
badPtDefinitionClause Oh impossible


||| "the creature with the greatest power"
public export
okExtremalSelection : Predicate [] Object
okExtremalSelection = Superlative MaxOf (CharAxis Power) Macros.creature

||| "the creature with the total power among creatures you control"
public export
badSumSelection : Unspellable (Predicate [] Object) (\ok =>
  Superlative SumOf (CharAxis Power) Macros.creature {ex = ok})
badSumSelection Oh impossible


||| "the creature with the least toughness among creatures you control"
public export
okDefiniteSuperlative : Noun [] Object
okDefiniteSuperlative =
  Macros.the (And [Macros.creature,
                   Superlative MinOf (CharAxis Toughness)
                               Macros.creatureYouControl])

||| "the creature"
public export
badBareDefinite : Unspellable (Noun [] Object) (\ok =>
  Macros.the Macros.creature {ok})
badBareDefinite Oh impossible


||| "Choose a creature you control."
public export
okChooseIndefinite : Effect []
okChooseIndefinite = Choose (Macros.a Macros.creatureYouControl) Nothing Openly

||| "Choose the creature with the least toughness among creatures you control."
public export
badChooseDefinite : Unspellable (Effect []) (\ok =>
  Choose (Macros.the (And [Macros.creature,
                         Superlative MinOf (CharAxis Toughness)
                                     Macros.creatureYouControl])) Nothing Openly {ch = ok})
badChooseDefinite BareChoice impossible


||| "the player with the highest life total"
public export
okPlayerStatSuperlative : Predicate [] Player
okPlayerStatSuperlative =
  Superlative MaxOf (PlayerStatAxis LifeTotal) AnyPlayer

||| "the creature with the highest life total among creatures you control"
public export
badLifeTotalSuperlative : Unspellable (Predicate [] Object) (\ok =>
  Superlative MaxOf (PlayerStatAxis LifeTotal) Macros.creature {sc = ok})
badLifeTotalSuperlative Refl impossible


||| "the last Intervention counter is removed from this enchantment by you"
public export
okAnnouncingRemovalAgent : GameEvent []
okAnnouncingRemovalAgent =
  CounterEvent CounterTaken (Just (Named "Intervention"))
               Macros.thisEnchantment LastCounter (Just You) False

public export
badAnnouncingRemovalAgent : Unspellable (GameEvent []) (\ok =>
  CounterEvent CounterTaken (Just (Named "Intervention")) Macros.thisEnchantment LastCounter
               (Just (Macros.target AnyPlayer)) False
               {ag = Present {ok}})
badAnnouncingRemovalAgent Refl impossible


||| "Put a charge counter on this artifact."
public export
okChargeCounterOnArtifact : Effect []
okChargeCounterOnArtifact =
  PutCounters (Lit 1) (PrintedKind (Named "Charge")) Macros.thisArtifact

||| "Each player gets a charge counter."
public export
badGetsChargeCounter : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind (Named "Charge")) You {sc = ok})
badGetsChargeCounter Oh impossible


||| "this creature"
public export
okAscribeCreature : Noun [] Object
okAscribeCreature = AsType Creature This Nothing

||| "this instant"
public export
badAscribeInstant : Unspellable (Noun [] Object) (\ok =>
  AsType Instant This Nothing {way = ok})
badAscribeInstant Oh impossible


||| "this sorcery"
public export
badAscribeSorcery : Unspellable (Noun [] Object) (\ok =>
  AsType Sorcery This Nothing {way = ok})
badAscribeSorcery Oh impossible


||| "this kindred"
public export
badAscribeKindred : Unspellable (Noun [] Object) (\ok =>
  AsType Kindred This Nothing {way = ok})
badAscribeKindred Oh impossible


||| "this Aura land"
public export
badAscribeForeignSubtype : Unspellable (Noun [] Object) (\ok =>
  AsType Land This (Just (enchantmentType "Aura")) {way = ok})
badAscribeForeignSubtype Oh impossible


||| "target creature that's goaded"
public export
okObjectDesignation : Predicate [] Object
okObjectDesignation = HasDesignation Goaded

||| "target creature that is the monarch"
public export
badObjectMonarch : Unspellable (Predicate [] Object) (\ok =>
  HasDesignation Monarch {sc = ok})
badObjectMonarch Refl impossible


||| "You become the monarch."
public export
okBecomesMonarch : Effect []
okBecomesMonarch = GainsDesignation You Monarch Instructed Nothing

||| "You become goaded."
public export
badGoadedPlayer : Unspellable (Effect []) (\ok =>
  GainsDesignation You Goaded Instructed Nothing {sc = ok})
badGoadedPlayer Refl impossible


||| "Goad target creature."
public export
okGoadOnBattlefield : Effect []
okGoadOnBattlefield =
  GainsDesignation (Macros.target Macros.creature) Goaded Instructed Nothing

||| "goad target creature card in your graveyard"
public export
badGoadInGraveyard : Unspellable (Effect []) (\ok =>
  GainsDesignation (Macros.target (And [Macros.creature,
                                        InZone (Macros.graveyardOf You)]))
                   Goaded Instructed Nothing {zn = ok})
badGoadInGraveyard (HolderOnField {ok = Oh}) impossible


||| "Untap target creature."
public export
okUntapInstruction : Effect []
okUntapInstruction = SetStatus Untapped (Macros.target Macros.creature)

||| "Unflip target creature."
public export
badUnflipInstruction : Unspellable (Effect []) (\ok =>
  SetStatus Unflipped (Macros.target Macros.creature) {at = ok})
badUnflipInstruction Oh impossible


||| "Ward {2}"
public export
okWardCost : Ability
okWardCost =
  KeywordAbility "Ward" (Just (ParamCost (Mana [Macros.generic 2]))) Nothing

||| "Ward"
public export
badBareWardLine : Unspellable Ability (\ok => KeywordAbility "Ward" Nothing Nothing {pf = ok})
badBareWardLine Oh impossible


||| "Ward red"
public export
badWardQuality : Unspellable Ability (\ok =>
  KeywordAbility "Ward" (Just (ParamQuality (ColorIs Red))) Nothing {pf = ok})
badWardQuality Oh impossible


||| "Flying {2}"
public export
badParamOnNullaryKeyword : Unspellable Ability (\ok =>
  KeywordAbility "Flying" (Just (ParamCost (Mana [Macros.generic 2]))) Nothing {pf = ok})
badParamOnNullaryKeyword Oh impossible


||| "Flyign"
public export
badUnknownKeywordLabel : Unspellable Ability (\ok =>
  KeywordAbility "Flyign" Nothing Nothing {pf = ok})
badUnknownKeywordLabel Oh impossible


||| "each creature with flying"
public export
okKnownKeywordPredicate : Predicate [] Object
okKnownKeywordPredicate = HasKeyword (TheKeyword "Flying")

||| "each creature with flyign"
public export
badUnknownKeywordPredicate : Unspellable (Predicate [] Object) (\ok =>
  HasKeyword (TheKeyword "Flyign") {kn = ok})
badUnknownKeywordPredicate Oh impossible


||| "Protection from red"
public export
badProtectionOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Protection" (Just (ParamQuality (ColorIs Red))) Nothing] Nothing {fl = ok})
badProtectionOnInstant MkFaceLaws impossible


||| "Protection from player"
public export
badProtectionFromPlayerRestriction : Unspellable Ability (\ok =>
  KeywordAbility "Protection" (Just (ParamSubject AnyPlayer)) Nothing {pf = ok})
badProtectionFromPlayerRestriction Oh impossible


||| "Equip {2}"
public export
badEquipOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility "Equip" (Just (ParamCost (Mana [Macros.generic 2]))) Nothing]
       Nothing {fl = ok})
badEquipOnSorcery MkFaceLaws impossible


||| "each of up to two target creatures"
public export
okEachOfTargetGroup : Noun [] Object
okEachOfTargetGroup = EachOf (Macros.targets (Macros.upTo 2) Macros.creature)

||| "each of one or more creatures"
public export
badEachOfCountedGroup : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.counted (Macros.atLeast 1) Macros.creature) {gm = ok})
badEachOfCountedGroup Oh impossible


||| "one of the top two cards of your library"
public export
okPartitiveOfSlice : Noun [] Object
okPartitiveOfSlice =
  SomeOf (CountedSlice (Macros.exactly 1)) Nothing
         (LibrarySlice OnTop (Lit 2) You)

||| "one of one or more creatures"
public export
badPartitiveOfCountedGroup : Unspellable (Noun [] Object) (\ok =>
  SomeOf (CountedSlice (Macros.exactly 1)) Nothing
         (Macros.counted (Macros.atLeast 1) Macros.creature)
         {gm = ok})
badPartitiveOfCountedGroup Oh impossible


||| "Whenever a creature attacks a player, …"
public export
okSingularAttackDefender : GameEvent []
okSingularAttackDefender =
  Attacks (Macros.a Macros.creature) (OneDefender (Macros.a AnyPlayer))

||| "Whenever a creature attacks your opponents, …"
public export
badPluralAttackDefender : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (PlayerGroup YourOpponents) {sg = ok}))
badPluralAttackDefender Refl impossible


||| "Whenever a creature attacks a planeswalker, …"
public export
okPlaneswalkerAttackDefender : GameEvent []
okPlaneswalkerAttackDefender =
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a (HasType Planeswalker)))

||| "Whenever a creature attacks another creature, …"
public export
badCreatureAttackDefender : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a Macros.creature) {at = ok}))
badCreatureAttackDefender Oh impossible


public export
badAgentChooseTheRest : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt ((Macros.topSlice (Lit 4)))
               , Move (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) Macros.handZ []
               , Choose Macros.theRest (Just (Macros.a Opponent)) Openly {ch = ok} ])
badAgentChooseTheRest AgentChoice impossible


||| "… if a creature died this turn, …"
public export
okDeathLookback : Condition []
okDeathLookback =
  Happened Death (Macros.a Macros.creature) ThisTurn Nothing

||| "… if a creature was put this turn, …"
public export
badPlacementLookback : Unspellable (Condition []) (\ok =>
  Happened Placement (Macros.a Macros.creature) ThisTurn Nothing
           {cw = LeftBare {ok = ok}})
badPlacementLookback Oh impossible


||| "Whenever a creature enters during your turn, draw a card."
public export
okHeaderOwnTurnWindow : Ability
okHeaderOwnTurnWindow =
  Triggered Whenever (Enters (Macros.a Macros.creature) Nothing) [] Nothing []
            (Just (DuringWindow Turn (Just You))) Nothing Nothing
            (Macros.draw You (Lit 1))

||| "Whenever a creature enters during the turn, draw a card."
public export
badHeaderBareTurnWindow : Unspellable Ability (\ok =>
  Triggered Whenever (Enters (Macros.a Macros.creature) Nothing) [] Nothing [] (Just (DuringWindow Turn Nothing {hw = ok})) Nothing Nothing
            (Draw You (Lit 1)))
badHeaderBareTurnWindow Oh impossible


||| "If one or more tokens would be created under your control, …"
public export
okTokenCreationSubject : GameEvent []
okTokenCreationSubject =
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken) False Nothing
                (Just You)

||| "If one or more creatures would be created under your control, …"
public export
badNonTokenCreationSubject : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) Macros.creature) False Nothing (Just You) {tk = ok})
badNonTokenCreationSubject CountedTokens impossible
badNonTokenCreationSubject OneToken impossible


||| "… that many plus one +1/+1 counters are put on it instead"
public export
okManyCounterBatchSize : StaticEffect []
okManyCounterBatchSize =
  Intercepts (CounterEvent CounterPut (Just Macros.plusOnePlusOne)
                           (Macros.a Macros.creatureYouControl) ManyCounters
                           Nothing False) [] Nothing
             (PutCounters (Plus ThatMuch (Lit 1))
                          (PrintedKind Macros.plusOnePlusOne) It)
             Repeatedly Nothing

public export
badSingularCounterBatchSize : Unspellable (StaticEffect []) (\ok =>
  Intercepts (CounterEvent CounterPut (Just Macros.plusOnePlusOne)
                           (Macros.a Macros.creatureYouControl) OneCounter Nothing False) [] Nothing
             (PutCounters (Plus (ThatMuch {ok}) (Lit 1))
                          (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf)))
             Repeatedly Nothing)
badSingularCounterBatchSize Refl impossible


||| "Whenever one or more +1/+1 counters are put on a creature you control, …"
public export
okUncausedCounterWithAgent : GameEvent []
okUncausedCounterWithAgent =
  CounterEvent CounterPut (Just Macros.plusOnePlusOne)
               (Macros.a Macros.creatureYouControl) ManyCounters (Just You)
               False

public export
badCausedCounterWithAgent : Unspellable (GameEvent []) (\ok =>
  CounterEvent CounterPut (Just Macros.plusOnePlusOne)
               (Macros.a Macros.creatureYouControl) ManyCounters (Just You) True {cz = ok})
badCausedCounterWithAgent Oh impossible


||| "Create a 1/1 black Zombie creature token. Create two of those tokens."
public export
okAnaphoricTokenAfterToken : Effect []
okAnaphoricTokenAfterToken =
  Sequentially [ Macros.create (Lit 1)
                   (Macros.creatureTok 1 1 [Black] [creatureType "Zombie"])
               , Create You (Lit 2) TokenAsThose [] ]

||| "Destroy target creature. Create two of those tokens."
public export
badAnaphoricTokenAfterNonToken : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , Create You (Lit 2) (TokenAsThose {ok}) [] ])
badAnaphoricTokenAfterNonToken Refl impossible


||| "Each land you control becomes a 2/2 creature. It's still a land."
public export
okStillALand : StaticEffect []
okStillALand =
  Becomes (Macros.allOf Macros.land) Sets
          (Bundle (MkToken (Just (Lit 2 ** Lit 2)) []
                           (MkTypeLine [] [Creature]) [] Nothing) (Just Land))

||| "Target creature becomes a Coward until end of turn. It's still a land."
public export
badStillOnSubtypeSet : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.target Macros.creature) Sets (Bundle (MkToken Nothing [] (MkTypeLine [creatureType "Coward"] []) [] Nothing) (Just Land)) {ok = ok})
badStillOnSubtypeSet Oh impossible


public export
badStillAnInstant : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.target Macros.creature) Sets (Bundle (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing) (Just Instant)) {ok = ok})
badStillAnInstant Oh impossible


||| "Target creature gets +1/+1, gains flying, and gains trample."
public export
okFlatCoordination : StaticEffect []
okFlatCoordination =
  AndAlso Nothing [ Gets Adds (Macros.target Macros.creature)
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains It (KeywordAbility "Flying" Nothing Nothing)
                  , Gains It (KeywordAbility "Trample" Nothing Nothing) ]

||| "Target creature gets +1/+1 and gains flying and gains trample."
public export
badNestedCoordination : Unspellable (StaticEffect []) (\ok =>
  AndAlso Nothing (Coord.(::) (AndAlso Nothing [ Gets Adds (Macros.target Macros.creature)
                                      (PtUp (Lit 1)) (PtUp (Lit 1))
                               , Gains ((Macros.It OneOf)) (KeywordAbility "Flying" Nothing Nothing) ])
                      {nc = ok}
                      (Coord.(::) (Gains ((Macros.It OneOf)) (KeywordAbility "Trample" Nothing Nothing)) Coord.Nil)))
badNestedCoordination Oh impossible


||| "Target creature gets +1/+1."
public export
okSingletonCoordination : StaticEffect []
okSingletonCoordination =
  AndAlso Nothing [ Gets Adds (Macros.target Macros.creature)
                         (PtUp (Lit 1)) (PtUp (Lit 1)) ]

||| a coordination of no statements
public export
badEmptyCoordination : Unspellable (StaticEffect []) (\ok =>
  AndAlso Nothing [] {ne = ok})
badEmptyCoordination ItIsSucc impossible


||| "Whenever a creature enters, destroy that creature."
public export
okThatCreatureAfterAntecedent : Ability
okThatCreatureAfterAntecedent =
  Triggered Whenever (Enters (Macros.a Macros.creature) Nothing) [] Nothing []
            Nothing Nothing Nothing
            (Macros.destroy (That (TypeW Creature)))

||| "This creature gets +1/+1 and that creature has flying."
public export
badThatCreatureIsStaticSubject : Unspellable Ability (\ok =>
  Static (AndAlso Nothing [ Gets Adds Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains (Macros.That (TypeW Creature) OneOf {ok = ok}) (KeywordAbility "Flying" Nothing Nothing) ]))
badThatCreatureIsStaticSubject Refl impossible


||| "Creatures you control get +1/+1 and they have flying."
public export
okCoordinatedPlural : Ability
okCoordinatedPlural =
  Static (AndAlso Nothing
            [ Gets Adds (Macros.allOf Macros.creatureYouControl)
                   (PtUp (Lit 1)) (PtUp (Lit 1))
            , Gains Them (KeywordAbility "Flying" Nothing Nothing) ])

||| "Enchanted creature gets +1/+1 and they have flying."
public export
badCoordinatedHostPlural : Unspellable Ability (\ok =>
  Static (AndAlso Nothing [ Gets Adds (AttachHost Enchanted (TypeW Creature))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains ((Macros.It ManyOf) {ok = ok}) (KeywordAbility "Flying" Nothing Nothing) ]))
badCoordinatedHostPlural Refl impossible


||| "Enchanted creature can't block."
public export
okEnchantedCreatureCantBlock : Ability
okEnchantedCreatureCantBlock =
  Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                  Forbid ["Block"] Agent NoDeonticPatient)

||| "Enchanted land gets +1/+1 and can't block."
public export
badCoordinatedLandHostBlocks : Unspellable Ability (\ok =>
  Static (AndAlso Nothing [ Gets Adds (AttachHost Enchanted (TypeW Land))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Macros.deontic ((Macros.It OneOf)) Forbid ["Block"] Agent NoDeonticPatient {dp = ok} ]))
badCoordinatedLandHostBlocks Oh impossible


||| "enchanted player"
public export
okEnchantedPlayer : Noun [] Player
okEnchantedPlayer = AttachHost Enchanted PlayerW

||| "equipped player"
public export
badEquippedPlayer : Unspellable (Noun [] Player) (\ok =>
  AttachHost Equipped PlayerW {ok})
badEquippedPlayer Oh impossible


||| "fortified creature"
public export
badFortifiedCreature : Unspellable (Noun [] Object) (\ok =>
  AttachHost Fortified (TypeW Creature) {ok})
badFortifiedCreature Oh impossible


||| "Target creature gains flash."
public export
badBattlefieldFlash : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.target Macros.creature) (KeywordAbility "Flash" Nothing Nothing) {ok})
badBattlefieldFlash Oh impossible


||| "Target spell gains indestructible."
public export
badSpellIndestructible : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.target Macros.spell) (KeywordAbility "Indestructible" Nothing Nothing) {ok})
badSpellIndestructible Oh impossible


||| a "Kindred Enchantment — Siege" card
public export
badSiegeWithoutBattle : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [battleType "Siege"] [Kindred, Enchantment])
       [KeywordAbility "Flying" Nothing Nothing] Nothing {fl = ok})
badSiegeWithoutBattle (MkFaceLaws {ln = MkCardLine}) impossible


||| a "Kindred — Merfolk" card naming no other card type
public export
badKindredAlone : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Merfolk"] [Kindred])
       [KeywordAbility "Flying" Nothing Nothing] Nothing {fl = ok})
badKindredAlone (MkFaceLaws {ln = MkCardLine}) impossible


||| "Enchanted planeswalker can't attack."
public export
badPlaneswalkerAttacks : Unspellable Ability (\ok =>
  Static (Macros.deontic (AttachHost Enchanted (TypeW Planeswalker))
                  Forbid ["Attack"] Agent NoDeonticPatient {dp = ok}))
badPlaneswalkerAttacks Oh impossible


||| "Create a 1/1 white Soldier creature token with flying."
public export
okTokenKeywordAbility : Effect []
okTokenKeywordAbility =
  Macros.create (Lit 1)
    (MkToken (Just (Lit 1 ** Lit 1)) [White]
             (MkTypeLine [creatureType "Soldier"] [Creature])
             [KeywordAbility "Flying" Nothing Nothing] Nothing)

||| "Create a 1/1 white Soldier creature token with 'Draw two cards.'"
public export
badTokenSpellAbility : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                 (MkTypeLine [creatureType "Soldier"] [Creature])
                                 [Spell (Draw You (Lit 1))] Nothing) {wf = ok})
badTokenSpellAbility (Oh, Oh, Oh, Oh, Oh, Oh) impossible


||| "Instant and sorcery spells you cast have '{T}: Draw a card.'"
public export
badQuotedGrantOnSpell : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.allOf (And [Macros.instantOrSorcery, Macros.spell, Macros.castBy You]))
        (Activated TapSymbol (Draw You (Lit 1)) Nothing Nothing Nothing Nothing) {ok})
badQuotedGrantOnSpell Oh impossible


||| "You get an emblem with 'At the beginning of your end step, draw a card.'"
public export
okTriggeredEmblem : Effect []
okTriggeredEmblem =
  GetsEmblem You
    [ Macros.triggered At (BeginningOf ThePart EndStep Macros.yours)
                       (Macros.draw You (Lit 1)) ]

||| "You get an emblem with 'flying'."
public export
badKeywordEmblem : Unspellable (Effect []) (\ok =>
  GetsEmblem You [KeywordAbility "Flying" Nothing Nothing] {ea = ok})
badKeywordEmblem Oh impossible


||| "You get an emblem."
public export
badEmptyEmblem : Unspellable (Effect []) (\ok =>
  GetsEmblem You [] {ea = ok})
badEmptyEmblem Oh impossible


||| "you pay {2}"
public export
okPayMana : Effect []
okPayMana = Pay You (Mana [Macros.generic 2]) PaidOnce

||| "you pay [+1]"
public export
badPayLoyalty : Unspellable (Effect []) (\ok =>
  Pay You (LoyaltySymbol (LoyaltyUp 1)) PaidOnce {pb = ok})
badPayLoyalty Oh impossible


||| "[+1]: Draw a card."
public export
badLoyaltySorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Loyalty Sorcery" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [Activated (LoyaltySymbol (LoyaltyUp 1)) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing] Nothing {fl = ok})
badLoyaltySorcery MkFaceLaws impossible


||| "Add {R}."
public export
okSingleProduction : Effect []
okSingleProduction = AddMana You (Lit 1) (Runs [[OfColor Red]]) []

||| "Add."
public export
badEmptyProduction : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [] {ok}) [])
badEmptyProduction Oh impossible

||| "Add {R} or ."
public export
badEmptyAlternative : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [[OfColor Red], []] {ok}) [])
badEmptyAlternative Oh impossible


||| "Add {C}. Spend this mana only to activate abilities."
public export
okSpendPurpose : Effect []
okSpendPurpose =
  AddMana You (Lit 1) (Runs [[Colorless]]) [SpendOnly [ToActivate Nothing]]

||| "Add {C}. Spend this mana only."
public export
badPurposelessSpend : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [[Colorless]]) [SpendOnly [] {ne = ok}])
badPurposelessSpend MkSpendPurposes impossible


||| "a token that's a copy of target creature, except it's an artifact"
public export
okCopyTypeException : Effect []
okCopyTypeException =
  Create You (Lit 1)
         (TokenCopyOf (Macros.target Macros.creature)
                      [ExceptTypes (MkTypeLine [] [Artifact])]) []

public export
badEmptyCopyTypeException : Unspellable (Effect []) (\ok =>
  Create You (Lit 1)
         (TokenCopyOf (Macros.target Macros.creature)
                      [ExceptTypes (MkTypeLine [] []) {ne = ok}]) [])
badEmptyCopyTypeException Oh impossible

||| "Copy target instant or sorcery spell."
public export
okCopyStackSpell : Effect []
okCopyStackSpell =
  Copy FromStack You
       (Macros.target (And [Macros.instantOrSorcery, Macros.spell])) (Lit 1) []

||| "Copy target creature."
public export
badCopyPermanent : Unspellable (Effect []) (\ok =>
  Copy FromStack You (Macros.target Macros.creature) (Lit 1) [] {cp = ok})
badCopyPermanent StackSpell impossible


||| "Choose new targets for target instant or sorcery spell."
public export
okRetargetStackSpell : Effect []
okRetargetStackSpell =
  ChooseNewTargets (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))

||| "Choose new targets for target creature."
public export
badRetargetPermanent : Unspellable (Effect []) (\ok =>
  ChooseNewTargets (Macros.target Macros.creature) {cp = ok})
badRetargetPermanent StackSpell impossible


||| "Create a token that's a copy of target creature. Untap that token."
public export
okSetStatusOnBattlefield : Effect []
okSetStatusOnBattlefield =
  Sequentially [Create You (Lit 1)
                       (TokenCopyOf (Macros.target Macros.creature) []) [],
                SetStatus Untapped (That TokenW)]

||| "Copy target instant or sorcery spell. Untap that token."
public export
badStackCopyAsToken : Unspellable (Effect []) (\ok =>
  Sequentially [Copy FromStack You (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                          (Lit 1) [],
                SetStatus Untapped (Macros.That TokenW OneOf {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badStackCopyAsToken (Refl, _) impossible


||| "Create a token that's a copy of target creature. Untap that copy."
public export
badTokenCopyAsCopyMention : Unspellable (Effect []) (\ok =>
  Sequentially [Create You (Lit 1) (TokenCopyOf (Macros.target Macros.creature) []) [],
                SetStatus Untapped (Macros.That CopyW OneOf {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badTokenCopyAsCopyMention (Refl, _) impossible


||| "Destroy target creature. Its controller loses life equal to its power."
public export
okItAfterAntecedent : Effect []
okItAfterAntecedent =
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , Macros.losesLife (Macros.controllerOf It) (Macros.powerOf It) ]

public export
badOtherwiseReadsLeadingArm : Unspellable (Effect []) (\ok =>
  If (Macros.exists Macros.creatureYouControl)
     (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [creatureType "Zombie"]))
     (Just (SetStatus Tapped ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsLeadingArm (Refl, _) impossible


public export
badLeadingConditionAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [If (CompareAmt (PlayerStatOf LifeTotal You) Less
                               (PlayerStatOf LifeTotal Macros.anOpponent))
                   (Macros.gainsLife You (Lit 6))
                   Nothing,
                Macros.losesLife (Macros.That PlayerW OneOf {ok}) (Lit 1)])
badLeadingConditionAntecedent Refl impossible
