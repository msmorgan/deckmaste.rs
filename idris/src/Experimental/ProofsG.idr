module Experimental.ProofsG

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "This deals 3 damage to that permanent or player."
public export
badUnionAnaphorNoAntecedent : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3) (That JoinW {ok = ok}))
badUnionAnaphorNoAntecedent Refl impossible


||| "Destroy target creature. This deals 3 damage to that permanent or player."
public export
badUnionAnaphorOnObject : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , DealDamage This (Lit 3) (That JoinW {ok = ok}) ])
badUnionAnaphorOnObject Refl impossible


||| "This deals 3 damage to any target. Counter that spell or ability."
public export
badAbilityJoinAnaphorOnPlayerUnion : Unspellable (Effect []) (\ok =>
  Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
               , Macros.counterSpell (That AbilityJoinW {ok = ok}) ])
badAbilityJoinAnaphorOnPlayerUnion Refl impossible


||| "Counter target activated ability. Counter that spell or ability."
public export
badAbilityJoinAnaphorOnAbility : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.counterSpell (Macros.target (AbilityHead AnyActivated))
               , Macros.counterSpell (That AbilityJoinW {ok = ok}) ])
badAbilityJoinAnaphorOnAbility Refl impossible



||| "each creature with a poison counter on it"
public export
badPoisonCounterDescription : Unspellable (Predicate [] Object) (\ok =>
  HasCounters (Just (Named "Poison")) {kn = Present {ok}})
badPoisonCounterDescription Refl impossible


||| "Cumulative upkeep"
public export
badBareCumulativeUpkeep : Unspellable Ability (\ok =>
  KeywordAbility "CumulativeUpkeep" Nothing Nothing {pf = ok})
badBareCumulativeUpkeep Oh impossible


||| "Cumulative upkeep {2}"
public export
badCumulativeUpkeepOnSpell : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility "CumulativeUpkeep" (Just (ParamCost (Mana [Macros.generic 2]))) Nothing] Nothing {fl = ok})
badCumulativeUpkeepOnSpell MkFaceLaws impossible


||| "Flying (When this creature deals combat damage to a player, …)"
public export
badBodyOnBodilessKeyword : Unspellable Ability (\ok =>
  KeywordAbility "Flying" Nothing (Just (Macros.renownExpansion 1)) {bf = ok})
badBodyOnBodilessKeyword Oh impossible


||| "Renown 1 (When you cast this spell, copy it …)"
public export
badRenownWithStormExpansion : Unspellable Ability (\ok =>
  KeywordAbility "Renown" (Just (ParamNumber (Lit 1)))
                 (Just Macros.stormExpansion) {bf = ok})
badRenownWithStormExpansion Oh impossible


||| "a cumulative upkeep counter"
public export
badCumulativeUpkeepCounter : Unspellable CounterKind (\ok =>
  KeywordCounter "CumulativeUpkeep" {ok = ok})
badCumulativeUpkeepCounter Oh impossible


||| "Creatures you control get +1/+1. The same is true for menace and trample."
public export
badKeywordListOnPlainLine : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Gets (Macros.allOf Macros.creatureYouControl)
                                (PtUp (Lit 1)) (PtUp (Lit 1))))
                  [TheKeyword "Menace", TheKeyword "Trample"]
                  {ex = Builtin.fst ok, lk = Builtin.snd ok})
badKeywordListOnPlainLine (Oh, _) impossible


public export
badEmptyKeywordList : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Macros.exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")]))
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing Nothing)) AsLongAs
                             {st = Static.CondFirstDone}))
                  [] {lk = ok})
badEmptyKeywordList Oh impossible


public export
badKeywordListRepeatingBase : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Macros.exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")]))
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing Nothing)) AsLongAs
                             {st = Static.CondFirstDone}))
                  [TheKeyword "Menace", TheKeyword "Flying",
                   TheKeyword "Trample"] {lk = ok})
badKeywordListRepeatingBase Oh impossible


public export
badParameterisedKeywordInList : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Macros.exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")]))
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing Nothing)) AsLongAs
                             {st = Static.CondFirstDone}))
                  [TheKeyword "Ward"] {lk = ok})
badParameterisedKeywordInList Oh impossible


||| "a creature with flyings"
public export
badClassOfParamlessKeyword : Unspellable (Predicate [] Object) (\ok =>
  HasKeyword (AnyKeywordIn (MkKeywordFamily "Flying" Nothing)) {kn = ok})
badClassOfParamlessKeyword Oh impossible


||| "a creature with renown of any color"
public export
badSortedClassOnNumberKeyword : Unspellable (Predicate [] Object) (\ok =>
  HasKeyword (AnyKeywordIn (MkKeywordFamily "Renown" (Just Color))) {kn = ok})
badSortedClassOnNumberKeyword Oh impossible


||| "{T}: Draw a card. Activate only if you created this turn."
public export
badBareTokenCreationLookback : Unspellable (Condition []) (\ok =>
  Happened TokenCreation You Lookback.ThisTurn Nothing {cw = LeftBare {ok = ok}})
badBareTokenCreationLookback Oh impossible


||| "Destroy target creature that attacked with this creature this turn."
public export
badAttackerComplementOnObject : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo AttackDeclaration Lookback.ThisTurn (Just (Involving Macros.thisCreature {cp = ok})))
badAttackerComplementOnObject MkLookbackComplement impossible


public export
badPlayerCastComplement : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn (Just (Involving Macros.anOpponent {cp = ok})))
badPlayerCastComplement MkLookbackComplement impossible


||| "Counter target spell cast from the stack."
public export
badCastFromStack : Unspellable (Predicate [] Object) (\ok =>
  CastFrom Macros.stackZ {pf = ok})
badCastFromStack Oh impossible


||| "… that died from the battlefield this turn."
public export
badDeathOriginZone : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo Death Lookback.ThisTurn
             (Just (FromZones (FromZone [Macros.battlefieldZ]) Nothing {ok = ok})))
badDeathOriginZone Oh impossible


||| "if you've cast a spell from the stack this turn"
public export
badCastOriginFromStack : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromZone [Macros.stackZ]) Nothing {ok = ok})))
badCastOriginFromStack Oh impossible


||| "if you've cast a spell from this turn"
public export
badEmptyOriginCoordination : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromZone []) Nothing {ok = ok})))
badEmptyOriginCoordination Oh impossible


||| "if you've cast a spell from your hand from the command zone this turn"
public export
badNestedOriginPayload : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromZone [Macros.handZ])
                    (Just (FromZones (FromZone [Macros.commandZ]) Nothing)) {pl = ok})))
badNestedOriginPayload Oh impossible


||| "if you've cast a spell from anywhere other than this turn"
public export
badEmptyOriginExclusion : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromAnywhereBut []) Nothing {ok = ok})))
badEmptyOriginExclusion Oh impossible


||| "… that died from anywhere this turn."
public export
badDeathOriginAnywhere : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo Death Lookback.ThisTurn
             (Just (FromZones FromAnywhere Nothing {ok = ok})))
badDeathOriginAnywhere Oh impossible


||| "… that was put into the battlefield this turn."
public export
badPlacementIntoBattlefield : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo Placement Lookback.ThisTurn
             (Just (IntoZone Macros.battlefieldZ Nothing {ok = ok})))
badPlacementIntoBattlefield Oh impossible


||| "… that was put into your graveyard into exile this turn."
public export
badNestedDestination : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo Placement Lookback.ThisTurn
             (Just (IntoZone (Macros.graveyardOf You)
                      (Just (IntoZone Macros.exileZ Nothing)) {pl = ok})))
badNestedDestination Oh impossible


||| "if a creature died in your graveyard this way"
public export
badLocusOnDeath : Unspellable (Condition []) (\ok =>
  Happened Death (Macros.a Macros.creature) Lookback.ThisWay
           (Just (AtZone (Macros.graveyardOf You) {ok = ok})))
badLocusOnDeath Oh impossible


||| "if you searched this way, shuffle"
public export
badBareSearchLookback : Unspellable (Condition []) (\ok =>
  Happened (VerbedAct "Search") You Lookback.ThisWay Nothing {cw = LeftBare {ok}})
badBareSearchLookback Oh impossible


||| "if you shuffled your graveyard this way"
public export
badShuffleLocusAtGraveyard : Unspellable (Condition []) (\ok =>
  Happened (VerbedAct "Shuffle") You Lookback.ThisWay
           (Just (AtZone (Macros.graveyardOf You) {ok = ok})))
badShuffleLocusAtGraveyard Oh impossible


public export
badResolvedInstant : Unspellable (Noun [] Object) (\ok =>
  ResolvedPermanent (Macros.a (And [HasType Instant, Macros.spell])) {pm = ok})
badResolvedInstant Oh impossible


||| "this creature, once it resolves"
public export
badResolvedOnBattlefield : Unspellable (Noun [] Object) (\ok =>
  ResolvedPermanent Macros.thisCreature {zn = ok})
badResolvedOnBattlefield Oh impossible


||| "if it entered from the battlefield"
public export
badEntryOriginBattlefield : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo Entry Lookback.ThisTurn
             (Just (FromZones (FromZone [Macros.battlefieldZ]) Nothing {ok = ok})))
badEntryOriginBattlefield Oh impossible


||| "If you control an artifact, create a token."
public export
badSingletonConjunction : Unspellable (Condition []) (\ok =>
  AndCond [Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You])] {tw = ok})
badSingletonConjunction Oh impossible


||| "If you control an artifact and an enchantment, and you control a land, …"
public export
badNestedConjunction : Unspellable (Condition []) (\ok =>
  AndCond [ AndCond [ Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You])
                    , Macros.exists (And [Macros.enchantment, HasPossessor ControllerAx You]) ]
          , Macros.exists (And [Macros.land, HasPossessor ControllerAx You]) ] {fl = ok})
badNestedConjunction Oh impossible


public export
badUnlessConjunction : Unspellable (StaticEffect []) (\ok =>
  Conditionally (AndCond [ Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You])
                         , Macros.exists (And [Macros.enchantment, HasPossessor ControllerAx You]) ])
                (AltCost This Nothing) Unless {st = Static.CondFirstDone} {mk = ok})
badUnlessConjunction MkMarkingOk impossible


||| "Counter target spell unless its controller pays {2}."
public export
badLiteralScaledMana : Unspellable (Cost []) (\ok =>
  ScaledMana GenericUnit (Lit 2) {fe = ok})
badLiteralScaledMana Oh impossible


public export
badBareCountScaledMana : Unspellable (Cost []) (\ok =>
  ScaledMana GenericUnit (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You])) {fe = ok})
badBareCountScaledMana Oh impossible


||| "Target creature becomes a Zombie named Bob in addition to its other types."
public export
badNamedAddition : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [] (MkTypeLine [creatureType "Zombie"] []) [] (Just "Bob"))
                   Nothing {ok = ok})
badNamedAddition Oh impossible


public export
badRepeatedAdditionColor : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [Black, Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                   Nothing {ok = ok})
badRepeatedAdditionColor Oh impossible


public export
badRepeatedAdditionType : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [] (MkTypeLine [] [Artifact, Artifact]) [] Nothing)
                   Nothing {ok = ok})
badRepeatedAdditionType Oh impossible


||| "For each of target creature, its controller draws a card."
public export
badSingletonForEach : Unspellable (Effect []) (\ok =>
  ForEachOf (Macros.target Macros.creature) (Draw You (Lit 1)) {pl = ok})
badSingletonForEach Refl impossible


||| "if you activated a loyalty ability this turn"
public export
badBareActivationLookback : Unspellable (Condition []) (\ok =>
  Happened AbilityActivation You Lookback.ThisTurn Nothing {cw = LeftBare {ok = ok}})
badBareActivationLookback Oh impossible


||| "At the beginning of all players' upkeep, draw a card." [CR#102.1]
public export
badPluralPartPossessor : Unspellable Ability (\ok =>
  Triggered At (BeginningOf ThePart Upkeep (ByPlayer (Macros.allOf AnyPlayer)) {pu = ok}) [] Nothing [] Nothing Nothing Nothing (Macros.draw You (Lit 1)))
badPluralPartPossessor Oh impossible


||| "Put those cards on the top or bottom of your library in any order."
public export
badDisjunctionOrdered : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt (EitherEnd Nothing) (Just AnyOrder) Nothing {af = ok} Bare)
badDisjunctionOrdered Oh impossible


||| "zeroth from the top"
public export
badZerothFromTop : Unspellable LibOrdinal (\ok => Nth 0 {nz = ok})
badZerothFromTop ItIsSucc impossible


||| "Shuffle those cards into your library in any order."
public export
badShuffledArranged : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt Shuffled (Just AnyOrder) Nothing {af = ok} Bare)
badShuffledArranged Oh impossible


||| "Shuffle it into its owner's library third from the top."
public export
badShuffledOrdinal : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt Shuffled Nothing (Just (Nth 3)) {nf = ok} Bare)
badShuffledOrdinal Oh impossible


||| "if there is no monstrous creature"
public export
badNoHolderOnObject : Unspellable (Condition []) (\ok =>
  NoHolder Monstrous {sc = ok})
badNoHolderOnObject Refl impossible


||| "your monarch"
public export
badPossessedMonarch : Unspellable (Noun [] Object) (\ok =>
  Designated Monarch You {sc = ok})
badPossessedMonarch Refl impossible



||| "Unearth"
public export
badBareUnearth : Unspellable Ability (\ok =>
  KeywordAbility "Unearth" Nothing Nothing {pf = ok})
badBareUnearth Oh impossible


||| "Retrace {1}"
public export
badCostedRetrace : Unspellable Ability (\ok =>
  KeywordAbility "Retrace" (Just (ParamCost (Mana [Macros.generic 1]))) Nothing
                 {pf = ok})
badCostedRetrace Oh impossible


||| "Unearth {B}"
public export
badUnearthOnSpellCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Unearth" (Just (ParamCost (Mana [Macros.pip Black]))) Nothing] Nothing {fl = ok})
badUnearthOnSpellCard MkFaceLaws impossible


||| "Flashback {2}{U}"
public export
badFlashbackOnPermanentCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [KeywordAbility "Flashback" (Just (ParamCost (Mana [Macros.generic 2, Macros.pip Blue]))) Nothing]
       Nothing {fl = ok})
badFlashbackOnPermanentCard MkFaceLaws impossible


||| "Creature cards in your graveyard have warp {2}."
public export
badWarpGrantInGraveyard : Unspellable Ability (\ok =>
  Static (Gains (Macros.allOf (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting "Warp" (Mana [Macros.generic 2]))
                {ok = ok}))
badWarpGrantInGraveyard Oh impossible



public export
copyParticipleUnwritten : verbedMarkingOk "Copy" Attributive = False
copyParticipleUnwritten = Refl

||| "Change the target of target spell or ability with a single target."
public export
spellOrAbilityJoin : Payload (Object \/ Ability)
spellOrAbilityJoin = JoinP (ObjectP Nothing (Just Stack) Nothing Nothing Nothing)
                             (AbilityP Nothing)

public export
abilityUnderSpellOrAbility : So (kindLte Ability (Object \/ Ability))
abilityUnderSpellOrAbility = kindLteJoinR Object Ability

public export
joinedCreatureTy :
  tyOfReach (Word JoinW) OneOf (effIntro {bs = []}
    (DealDamage This (Lit 3)
       (Macros.target (Macros.kindJoin AnyPlayer Macros.creature))))
  = Just Creature
joinedCreatureTy = Refl

public export
anyTargetIsPlaceless :
  nounZone {bs = []} (Macros.target Macros.anyTarget) = Nothing
anyTargetIsPlaceless = Refl

public export
anyTargetTakesDamage : DamageRecipient (Macros.target {bs = []} Macros.anyTarget)
anyTargetTakesDamage = JoinTakes

public export
youAndBindsNothing :
  nounDelta {bs = []} (Macros.youAnd Macros.thisCreature) = []
youAndBindsNothing = Refl

||| "You draw a card. If a player is dealt damage this way, you draw a card."
public export
badDealtThisWayNoDamage : Unspellable (Effect []) (\ok =>
  Sequentially [ Draw You (Lit 1)
               , If (DealtThisWay AnyPlayer {wy = ok}) (Draw You (Lit 1)) Nothing ])
badDealtThisWayNoDamage Oh impossible

public export
badDealtThisWayAbility : Unspellable (Effect []) (\ok =>
  Sequentially [ DealDamage This (Lit 2) (Macros.target Macros.anyTarget)
               , If (DealtThisWay IsManaAbility {rk = ok}) (Draw You (Lit 1))
                    Nothing ])
badDealtThisWayAbility Oh impossible

||| "Flip a coin. Draw that many cards."
public export
badThatMuchAfterFlip : Unspellable (Effect []) (\ok =>
  Sequentially [(Macros.flipCoins You 1), Draw You (ThatMuch {ok})])
badThatMuchAfterFlip Refl impossible

||| "If you win the flip, draw a card."
public export
badFlipArmWithoutFlip : Unspellable (Effect []) (\ok =>
  If (FlipCalled You WinsFlip {fl = ok}) (Draw You (Lit 1)) Nothing)
badFlipArmWithoutFlip Oh impossible

||| "1—9 | Draw a card."
public export
badTableWithoutRoll : Unspellable (Effect []) (\ok =>
  ResultsTable [Macros.rollRow (Macros.fromTo 1 9) (Draw You (Lit 1))] {ok})
badTableWithoutRoll Refl impossible

||| "Roll a d0."
public export
badNoughtSidedDie : Unspellable (Effect []) (\ok =>
  RollDice You (Lit 1) (SidesOf 0 {nz = ok}))
badNoughtSidedDie ItIsSucc impossible

||| a planeswalker card printed with no starting loyalty
public export
badPlaneswalkerNoLoyalty : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary] (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [] Nothing {fl = ok})
badPlaneswalkerNoLoyalty (MkFaceLaws {bx = MkCardBox}) impossible

||| a planeswalker card printing "3/3" where its loyalty number goes
public export
badPlaneswalkerPtBox : Unspellable Card (\ok =>
  Macros.cardOf "" (Just [Macros.pip Blue]) [Legendary] (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [] (Macros.printedBox (Just (3, 3))) {bx = ok})
badPlaneswalkerPtBox MkCardBox impossible

||| a battle card printed with no defense
public export
badBattleNoDefense : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [battleType "Siege"] [Battle]) [] Nothing {fl = ok})
badBattleNoDefense (MkFaceLaws {bx = MkCardBox}) impossible

||| an adventurer card whose inset frame is a plain instant, naming no Adventure
public export
badUnnamedAdventure : Unspellable Card (\ok =>
  Adventurer (Macros.frontFace "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature]) []
                               (Macros.printedBox (Just (1, 1))))
             (Macros.frontFace "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant]) []
                               Nothing)
             {ai = ok})
badUnnamedAdventure Oh impossible

||| a flip card whose upside-down half is an instant
public export
badSpellFlipHalf : Unspellable Card (\ok =>
  FlipCard (Macros.frontFace "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) []
                             (Macros.printedBox (Just (1, 1))))
           (Macros.backFace "" [] (MkTypeLine [] [Instant]) [] Nothing)
           {ah = ok})
badSpellFlipHalf Oh impossible


||| a transforming card whose back face prints a mana cost of its own
public export
badTransformingBackWithCost : Unspellable Card (\ok =>
  Transforming (Macros.frontFace "" (Just [Macros.pip Green]) []
                                 (MkTypeLine [] [Creature]) []
                                 (Macros.printedBox (Just (1, 1))))
               (MkFace "" (Just [Macros.pip Green]) [] []
                       (MkTypeLine [] [Creature]) []
                       (Macros.printedBox (Just (1, 1))))
               {bf = ok})
badTransformingBackWithCost MkFaceLaws impossible

||| an activated ability printed on a conspiracy card
public export
badConspiracyActivated : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Conspiracy])
       [ Macros.activated (Mana [Macros.generic 1]) (Macros.draw You (Lit 1)) ] Nothing {fl = ok})
badConspiracyActivated MkFaceLaws impossible

||| a static ability printed on a dungeon card
public export
badDungeonStatic : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Dungeon])
       [ Static (Macros.entersWithCounters Macros.thisCreature (Lit 1)
                   Macros.plusOnePlusOne) ] Nothing {fl = ok})
badDungeonStatic MkFaceLaws impossible


||| "your opponents' devotion to black"
public export
badPluralDevotion : Unspellable (Amount []) (\ok =>
  Devotion (PlayerGroup YourOpponents) (LitColor Black) Nothing {one = ok})
badPluralDevotion Refl impossible

||| "the amount of creatures that died this turn"
public export
badDeathSum : Unspellable (Amount []) (\ok =>
  EventSum Death (Macros.a Macros.creature) Lookback.ThisTurn Nothing
           {qm = ok})
badDeathSum Oh impossible

||| "the total power of target creature"
public export
badSingularAggregate : Unspellable (Amount []) (\ok =>
  Aggregate SumOf (CharAxis Power) (Macros.target Macros.creature)
              {pl = ok})
badSingularAggregate Refl impossible

||| "the greatest life total among all creatures"
public export
badAggregateWrongSort : Unspellable (Amount []) (\ok =>
  Aggregate MaxOf (PlayerStatAxis LifeTotal) (Macros.allOf Macros.creature)
              {sc = ok})
badAggregateWrongSort Refl impossible

||| "up to X | Draw a card."
public export
badAmountRollRow : Unspellable (Effect []) (\ok =>
  Sequentially [(Macros.rollDice You 1 20),
                ResultsTable [MkRollRow (UpToOf (LetterVal X))
                                        (Macros.draw You (Lit 1)) {lt = ok}]])
badAmountRollRow Oh impossible

public export
badCreatureHalfRead : Unspellable (Effect []) (\ok =>
  Sequentially
    [ DealDamage This (Lit 3)
        (Macros.target (Macros.kindJoin AnyPlayer (HasType Planeswalker)))
    , (Macros.discard (Macros.thatSplitController (That (TypeW Creature) {ok = ok})) (Macros.a (InZone Macros.handZ))) ])
badCreatureHalfRead Refl impossible

||| "Whenever a creature attacks a planeswalker or a creature"
public export
badMixedAttackDefenderHalves : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a (Joined (HasType Planeswalker)
                                         (HasType Creature))) {at = ok}))
badMixedAttackDefenderHalves Oh impossible

||| "This deals 3 damage to a land or a land."
public export
badSameKindJoinDamage : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3)
             (Macros.a (Joined (HasType Land) (HasType Land))) {rk = ok})
badSameKindJoinDamage JoinTakes impossible


||| "creature that could block target creature card in your graveyard"
public export
badCouldBlockGraveyardRelatum : Unspellable (Predicate [] Object) (\ok =>
  CombatRel CouldBlock (Macros.target (And [Macros.creature,
                                            InZone (Macros.graveyardOf You)])) {ok = ok})
badCouldBlockGraveyardRelatum Oh impossible


||| "Target land blocks an attacking creature."
public export
badLandBecomesBlocking : Unspellable (Effect []) (\ok =>
  BecomesBlocking (Macros.target Macros.land)
                  (Macros.a (And [Macros.creature, Attacking])) {dn = ok})
badLandBecomesBlocking Oh impossible


||| "This creature blocks target planeswalker."
public export
badBecomesBlockingPlaneswalker : Unspellable (Effect []) (\ok =>
  BecomesBlocking Macros.thisCreature
                  (Macros.target (HasType Planeswalker)) {dw = ok})
badBecomesBlockingPlaneswalker Oh impossible


||| "During each opponent's next turn, ..."
public export
badPluralNextTurnSpan : Unspellable (Duration []) (\ok =>
  DuringNextTurnOf (Macros.each Opponent) {one = ok})
badPluralNextTurnSpan Refl impossible


||| "Take an extra turn for each coin that comes up heads."
public export
badCoinsShowingWithoutFlip : Unspellable (Effect []) (\ok =>
  ExtraTurn You (CoinsShowing Heads {fl = ok}))
badCoinsShowingWithoutFlip Oh impossible

||| "If you rolled 7, sacrifice this creature."
public export
badTotalWithoutRoll : Unspellable (Effect []) (\ok =>
  Macros.ifThen (CompareAmt (TheTotal {ok}) Eq (Lit 7))
                (Macros.sacrifice You Macros.thisCreature))
badTotalWithoutRoll Refl impossible

||| "creature that won a coin flip this turn"
public export
badCreatureWonFlip : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo FlipWin Lookback.ThisTurn Nothing {sb = ok})
badCreatureWonFlip MkLookbackSubject impossible

||| "the amount of dice you rolled this turn"
public export
badRollAsMagnitude : Unspellable (Amount []) (\ok =>
  EventSum DiceRoll You Lookback.ThisTurn Nothing {qm = ok})
badRollAsMagnitude Oh impossible

public export
youWonAFlipThisTurn : Condition []
youWonAFlipThisTurn = Happened FlipWin You Lookback.ThisTurn Nothing

public export
youRolledADieThisTurn : Condition []
youRolledADieThisTurn = Happened DiceRoll You Lookback.ThisTurn Nothing

||| "Ignore the lowest roll."
public export
badIgnoreWithoutRoll : Unspellable (Effect []) (\ok =>
  IgnoreOutcomes (IgnoreExtreme LowestRoll) {ok})
badIgnoreWithoutRoll Oh impossible

||| "Store those results on this creature."
public export
badStoreResultsWithoutRoll : Unspellable (Effect []) (\ok =>
  StoreResults Macros.thisCreature {ok})
badStoreResultsWithoutRoll Refl impossible

||| "Roll that many dice."
public export
badAnaphoricSidesWithoutRoll : Unspellable (Effect []) (\ok =>
  RollDice You (Lit 1) (ThoseDice {ok}))
badAnaphoricSidesWithoutRoll Refl impossible

||| "If you rolled doubles, sacrifice this creature."
public export
badRolledDoublesWithoutRoll : Unspellable (Condition []) (\ok =>
  RolledDoubles {ok})
badRolledDoublesWithoutRoll Refl impossible

public export
afterACoinFlip : Bindings
afterACoinFlip = effIntro (the (Effect []) (Macros.flipCoins You 1))

||| "an ability whose coin comes up tails"
public export
badCoinCameUpOnAbility :
  Unspellable (Predicate ProofsG.afterACoinFlip Ability) (\ok =>
    CoinCameUp Tails {rk = ok})
badCoinCameUpOnAbility Oh impossible

||| "Whenever you roll a 0, …"
public export
badZeroRollTest : Unspellable (GameEvent []) (\ok =>
  RollsDice You OneDie AnyDie (ResultIn (Range Nothing (Just 0))
                                 {nz = ok} {wf = Oh} {lt = Oh}))
badZeroRollTest MaxAtLeastOne impossible

||| "Whenever you roll a 4 or higher on the planar die, …"
public export
badPlanarResultTest : Unspellable (GameEvent []) (\ok =>
  RollsDice You ManyDice PlanarDie
            (ResultIn (Range (Just 4) Nothing) {nz = UnboundedAbove}
                      {wf = Oh} {lt = Oh})
            {dw = ok})
badPlanarResultTest Oh impossible

||| "If you would flip a coin, instead flip two coins and ignore the lower one."
public export
badExtremeOverFlips :
  Unspellable (Effect ProofsG.afterACoinFlip) (\ok =>
    IgnoreOutcomes (IgnoreExtreme LowestRoll) {ok})
badExtremeOverFlips Oh impossible


||| "When a player doesn't pay this creature's flying, …"
public export
badPayCostlessKeyword : Unspellable (GameEvent []) (\ok =>
  PaysCost (Just (Macros.a AnyPlayer)) Unpaid Macros.thisCreature "Flying" {kc = ok})
badPayCostlessKeyword Oh impossible

||| "if you paid a cost this turn"
public export
badBarePaymentLookback : Unspellable (Condition []) (\ok =>
  Happened CostPayment You Lookback.ThisTurn Nothing {sb = ok})
badBarePaymentLookback MkLookbackSubject impossible

||| "Destroy the rest."
public export
badRestWithoutAPartition : Unspellable (Noun [] Object) (\ok =>
  TheRest {ok})
badRestWithoutAPartition Oh impossible

||| "Choose any number of target creatures. Destroy the rest."
public export
badRestAfterTargetChoice : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.choose (Macros.targets Macros.anyNumber Macros.creature)
               , Macros.destroy (TheRest {ok}) ])
badRestAfterTargetChoice Oh impossible

public export
afterChoiceRestDisposed : Bindings
afterChoiceRestDisposed =
  effIntro (the (Effect [])
    (Sequentially [ Macros.choose (Macros.counted (Macros.upTo 1) Macros.creature)
                  , Macros.destroy TheRest ]))

||| "Choose up to one creature. Destroy the rest. Destroy the rest."
public export
badChoiceRestDisposedTwice :
  Unspellable (Noun ProofsG.afterChoiceRestDisposed Object) (\ok => TheRest {ok})
badChoiceRestDisposedTwice Oh impossible

||| "an opponent who controls more lands than they control"
public export
badMemberInComparisonBound : Unspellable (Predicate [] Player) (\ok =>
  CompareOver Opponent (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))
              Greater (Macros.countOf (And [Macros.land, HasPossessor ControllerAx (They {ok})])))
badMemberInComparisonBound Refl impossible

||| "the number of basic creature types among creatures you control"
public export
badBasicCreatureTypeAxis : Unspellable (Amount []) (\ok =>
  DistinctCount (SubtypeAxis Creature BasicOnly {sc = ok})
                (Macros.allOf Macros.creatureYouControl))
badBasicCreatureTypeAxis Oh impossible

||| "Echo"
public export
badBareEcho : Unspellable Ability (\ok =>
  KeywordAbility "Echo" Nothing Nothing {pf = ok})
badBareEcho Oh impossible

public export
afterPassivePayment : Bindings
afterPassivePayment =
  eventAfter (the (GameEvent [])
    (PaysCost Nothing Paid Macros.thisCreature "CumulativeUpkeep"))

||| "Whenever this creature's cumulative upkeep is paid, that player …"
public export
badPassivePayerReadback :
  Unspellable (Noun ProofsG.afterPassivePayment Player) (\ok => That PlayerW {ok})
badPassivePayerReadback Refl impossible

public export
afterKeywordCostPayment : Bindings
afterKeywordCostPayment =
  eventAfter (the (GameEvent [])
    (PaysCost (Just You) Paid Macros.thisEnchantment "CumulativeUpkeep"))

public export
badKeywordCostPaymentThatMuch :
  Unspellable (Amount ProofsG.afterKeywordCostPayment) (\ok => ThatMuch {ok})
badKeywordCostPaymentThatMuch Refl impossible

public export
youPaidLifeThisTurn : Condition []
youPaidLifeThisTurn = Happened LifePayment You Lookback.ThisTurn Nothing

public export
afterShuffledLook : Bindings
afterShuffledLook =
  effIntro (the (Effect [])
    (Sequentially [ Macros.lookAt Macros.topCard, Macros.shuffle ]))

public export
badReadsShuffledLibraryCard :
  Unspellable (Noun ProofsG.afterShuffledLook Object) (\ok => That CardW {ok})
badReadsShuffledLibraryCard Refl impossible

public export
afterShuffledIntoLook : Bindings
afterShuffledIntoLook =
  effIntro (the (Effect [])
    (Sequentially [ Macros.lookAt Macros.topCard, Macros.shuffleInto You This ]))

public export
badReadsShuffledIntoLibraryCard :
  Unspellable (Noun ProofsG.afterShuffledIntoLook Object) (\ok => That CardW {ok})
badReadsShuffledIntoLibraryCard Refl impossible

||| "if up to three is 4 or greater"
public export
badCompareCeilingSubject : Unspellable (Condition []) (\ok =>
  CompareAmt (UpTo (Lit 3)) AtLeast (Lit 4) {rd = ok})
badCompareCeilingSubject Oh impossible

||| "This creature enters with your choice of a counter on it."
public export
badEmptyCounterMenu : Unspellable (StaticEffect []) (\ok =>
  EntersRider Macros.thisCreature (WithCounters (Lit 1) (ChosenKind [] {ne = ok}) Fresh))
badEmptyCounterMenu IsNonEmpty impossible

||| "Put your choice of a +1/+1 counter or a poison counter on target creature."
public export
badMixedScopeCounterMenu : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (ChosenKind [Macros.plusOnePlusOne, Named "Poison"])
              (Macros.target Macros.creature) {sc = ok})
badMixedScopeCounterMenu Oh impossible

||| "Put a zorp counter on target creature."
public export
badUnknownCounterLabel : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind (Named "Zorp" {ok}))
              (Macros.target Macros.creature))
badUnknownCounterLabel Oh impossible

||| "Put a flying counter on target creature." [CR#122.1b]
public export
badKeywordCounterNamedPlainly : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (PrintedKind (Named "Flying" {ok}))
              (Macros.target Macros.creature))
badKeywordCounterNamedPlainly Oh impossible

||| "Put a counter of each of those kinds on target creature."
public export
badThoseKindsUnannounced : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (ThoseKinds {ok}) (Macros.target Macros.creature))
badThoseKindsUnannounced Refl impossible

||| "For each color among permanents you control, add one mana of that color"
public export
badRepeatedCarriesNoColor : Unspellable (Effect []) (\ok =>
  Repeated (DistinctCount ColorAxis (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))
           (AddMana You (Lit 1) (OfChosenColor Nothing {cq = ok}) []))
badRepeatedCarriesNoColor Refl impossible

||| "For each color among permanents you control, … of that creature type."
public export
badAxisValueCrossing : Unspellable (Effect []) (\ok =>
  ForEachKindOf ColorAxis (Just (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))
                (SubtypeQ Creature) (Draw You (Lit 1)) {sc = ok})
badAxisValueCrossing Refl impossible

||| "For each creature type, …"
public export
badDomainlessOpenAxis : Unspellable (Effect []) (\ok =>
  ForEachKindOf (SubtypeAxis Creature AnySubtype) Nothing
                (SubtypeQ Creature) (Draw You (Lit 1)) {cl = ok})
badDomainlessOpenAxis Oh impossible

||| "target permanent that's exactly one color"
public export
badExactlyOneColor : Unspellable (Predicate [] Object) (\ok =>
  ExactlyColors 1 {ok = ok})
badExactlyOneColor Oh impossible

||| "target permanent that's exactly six colors"
public export
badExactlySixColors : Unspellable (Predicate [] Object) (\ok =>
  ExactlyColors 6 {ok = ok})
badExactlySixColors Oh impossible

||| "if this creature's flying cost was paid"
public export
badPaidCostOnCostlessKeyword : Unspellable (Predicate [] Object) (\ok =>
  PaidCost (ByKeyword "Flying") Nothing {nc = ok})
badPaidCostOnCostlessKeyword Oh impossible

||| "for each time it was kickre'd"
public export
badTimesPaidUnknownKeyword : Unspellable (Amount []) (\ok =>
  TimesPaid (ByKeyword "Kickre") Macros.thisCreature {nc = ok})
badTimesPaidUnknownKeyword Oh impossible


||| "Whenever you scry a card, …"
public export
badScryPatient : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Scry"
              (Just (Macros.a (InZone (ZoneAt Library Bare)))) Nothing False
              {pt = ok})
badScryPatient Oh impossible


||| "Whenever discards a card, …"
public export
badVoicelessAct : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Scry" Nothing Nothing False {vc = ok})
badVoicelessAct Oh impossible


||| "Whenever a card is put, …"
public export
badPassiveWithoutParticiple : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Put" (Just (Macros.a IsCard)) Nothing False {vc = ok})
badPassiveWithoutParticiple Oh impossible


||| "Whenever a card is milled into a Phyrexian, …"
public export
badBecomesWithoutIntransitive : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Mill" (Just (Macros.a (InZone (ZoneAt Library Bare))))
              (Just (HasSubtype (creatureType "Phyrexian"))) False {bc = ok})
badBecomesWithoutIntransitive Oh impossible


||| "Whenever an effect and you would create one or more tokens, …"
public export
badTokensCreatedByCauserAndPlayer : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                (Just AnEffect) (Just You) Nothing {vo = ok})
badTokensCreatedByCauserAndPlayer Oh impossible


||| "Whenever one or more tokens are created under your opponents' control, …"
public export
badTokensCreatedUnderPlural : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                Nothing Nothing (Just (PlayerGroup YourOpponents)) {vo = ok})
badTokensCreatedUnderPlural Oh impossible


||| "Whenever an opponent creates one or more tokens, …"
public export
badTokensCreatedByMintingNoun : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                Nothing (Just Macros.anOpponent) Nothing {vo = ok})
badTokensCreatedByMintingNoun Oh impossible


||| "Whenever a card in a graveyard is destroyed, …"
public export
badDestroyInGraveyard : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Destroy"
              (Just (Macros.a (InZone Macros.graveyardZ))) Nothing False {zn = ok})
badDestroyInGraveyard Oh impossible


||| "Whenever you discard a permanent you control, …"
public export
badDiscardFromBattlefield : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Discard"
              (Just (Macros.a (InZone Macros.battlefieldZ))) Nothing False {zn = ok})
badDiscardFromBattlefield Oh impossible


||| "if you dealt damage to an opponent this turn"
public export
badPlayerDamageDealer : Unspellable (Condition []) (\ok =>
  Happened DamageDealing You Lookback.ThisTurn Nothing {sb = ok})
badPlayerDamageDealer MkLookbackSubject impossible


||| "player that targets this creature"
public export
badPlayerTargeter : Unspellable (Predicate [] Player) (\ok =>
  Targets Macros.thisCreature SomeTarget {tr = ok})
badPlayerTargeter SpellTargets impossible
badPlayerTargeter AbilityTargets impossible
badPlayerTargeter (EitherTargets _ _) impossible


||| "Whenever you become the target of a player, …"
public export
badPlayerTargetingEvent : Unspellable (GameEvent []) (\ok =>
  BecomesTarget You (Macros.a AnyPlayer) {tr = ok})
badPlayerTargetingEvent SpellTargets impossible
badPlayerTargetingEvent AbilityTargets impossible
badPlayerTargetingEvent (EitherTargets _ _) impossible


||| "Exchange life totals with target opponent"
public export
badExchangeOneParty : Unspellable (Effect []) (\ok =>
  ExchangeLife (Macros.target Opponent) {tp = ok})
badExchangeOneParty Oh impossible


||| "You and your opponents exchange life totals."
public export
badExchangePluralParty : Unspellable (Effect []) (\ok =>
  ExchangeLife (Both You (PlayerGroup YourOpponents)) {tp = ok})
badExchangePluralParty Oh impossible


||| "a creature with power or life total 3 or greater"
public export
badMixedAxisComparison : Unspellable (Predicate [] Object) (\ok =>
  Compare [CharAxis Power, PlayerStatAxis LifeTotal] Greater (Lit 1) {at = ok})
badMixedAxisComparison (NextAxis _ (LastAxis _)) impossible


||| "{T}: … , where X is 3"
public export
badActivatedClosesCardLetter :
  Unspellable (AbilityAt (costLetters (Just [Variable]))) (\ok =>
    Macros.activated TapSymbol (Define X (Lit 3) {ok = ok}))
badActivatedClosesCardLetter Oh impossible


||| "This creature can attack as though it were mana of any color."
public export
badManaPremiseAtAttack : Unspellable (StaticEffect []) (\ok =>
  Deontic Macros.thisCreature Permit ["Attack"] Agent Nothing NoDeonticPatient
          (Just (AsThoughMana Nothing MatchAnyColor Nothing)) NoDeonticRider
          {at = ok})
badManaPremiseAtAttack Oh impossible


||| "This spell can't be countered more than once."
public export
badCounterBoundTwice : Unspellable (StaticEffect []) (\ok =>
  Deontic This Forbid ["Counter"] Patient (Just (MoreThan (Lit 1))) NoDeonticPatient
          Nothing NoDeonticRider {bd = ok})
badCounterBoundTwice Oh impossible


||| "You may spend mana as though it weren't a creature."
public export
badObjectPremiseAtSpend : Unspellable (StaticEffect []) (\ok =>
  Deontic You Permit ["Spend"] Agent Nothing NoDeonticPatient
          (Just (AsThoughOf (Not Macros.creature))) NoDeonticRider {at = ok})
badObjectPremiseAtSpend Oh impossible


||| "Whenever you sacrifice a creature for mana, …"
public export
badForManaOnNontap : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Sacrifice" (Just (Macros.a Macros.creature)) Nothing True
              {fm = ok})
badForManaOnNontap Oh impossible


||| "Add one mana of any type that land produced"
public export
badProducedByEventWithoutEvent : Unspellable (ProducedMana []) (\ok =>
  ProducedByEvent (Macros.a Macros.land) {pm = ok})
badProducedByEventWithoutEvent Refl impossible


||| "You don't lose this mana as steps and phases end"
public export
badThisManaWithoutAdd : Unspellable (StaticEffect []) (\ok =>
  KeepsUnspentMana You (ThisMana {ok = ok}))
badThisManaWithoutAdd Refl impossible


||| "Transform target creature card in your graveyard."
public export
badTurnOverOffField : Unspellable (Effect []) (\ok =>
  TurnOver (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
           {ok})
badTurnOverOffField Oh impossible


||| "Return target creature card from your graveyard to your hand transformed."
public export
badTransformedArrivalOffField : Unspellable (Effect []) (\ok =>
  Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
       Macros.handZ [EntersTransformed] {rf = ok})
badTransformedArrivalOffField Oh impossible


public export
badCardWordReadsPiles : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent Them 2 []
                  , Macros.move (Those CardW {ok = ok}) Macros.handZ ]) ]
       Nothing)
badCardWordReadsPiles Refl impossible


||| "Reveal the top five cards of your library. Put those piles into your hand."
public export
badPileWordWithoutAPartition : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , Macros.move (Those PileW {ok = ok}) Macros.handZ ]) ]
       Nothing)
badPileWordWithoutAPartition Refl impossible


||| "Put one pile into your hand."
public export
badPilePartitiveWithoutAPartition : Unspellable (Effect []) (\ok =>
  Macros.move (Macros.onePile {ok = ok}) Macros.handZ)
badPilePartitiveWithoutAPartition Refl impossible


public export
badMembershipInANonPile : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , Macros.move (Macros.allOf (And [IsCard, InPile Them {pm = ok}]))
                                Macros.handZ ]) ]
       Nothing)
badMembershipInANonPile PilePartitive impossible
badMembershipInANonPile ThatPile impossible
badMembershipInANonPile ThosePiles impossible


public export
badPileFaceAsAStatus : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent Them 2 []
                  , SetStatus FaceDown (Those PileW) {ok = ok} ]) ]
       Nothing)
badPileFaceAsAStatus Oh impossible


public export
badTriggeringReplaced : Unspellable (StaticEffect []) (\ok =>
  Intercepts (Triggers (Macros.a (And [ AbilityHead AnyTriggered
                                      , AbilityOf (Macros.a (And [Permanent, HasPossessor ControllerAx You])) ])))
             [] Nothing (Draw You (Lit 1)) Repeatedly Nothing {ok})
badTriggeringReplaced Oh impossible


||| "If a creature you control dies, that ability triggers an additional time."
public export
badMultipliedNonTrigger : Unspellable (StaticEffect []) (\ok =>
  TriggersAdditionally (Dies (Macros.a Macros.creatureYouControl))
                       (Macros.exactly 1) {ok})
badMultipliedNonTrigger Oh impossible


||| "unlock this door"
public export
badUnlockThisDoor : Unspellable (Effect []) (\ok =>
  Unlock ThisDoor {nh = ok})
badUnlockThisDoor Oh impossible


||| "When you unlock this door, this Room deals 1 damage to each opponent."
public export
badDoorHeaderOffSharedLine : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Red]) []
       (MkTypeLine [enchantmentType "Room"] [Enchantment])
       [ Macros.triggered When (UnlocksDoor You ThisDoor)
           (DealDamage Macros.thisRoom (Lit 1) (Macros.each Opponent)) ]
       Nothing {fl = ok})
badDoorHeaderOffSharedLine MkFaceLaws impossible


||| "unlock a locked door of a Room card in your graveyard"
public export
badUnlockDoorOffBattlefield : Unspellable (Effect []) (\ok =>
  Unlock (DoorOf (Just Locked)
            (Macros.a (And [HasSubtype (enchantmentType "Room"),
                            InZone Macros.graveyardZ])) {zn = ok}))
badUnlockDoorOffBattlefield Oh impossible


public export
badDoorOfBareThis : Unspellable (Effect []) (\ok =>
  Unlock (DoorOf (Just Locked) This {zn = ok}))
badDoorOfBareThis Oh impossible


||| "Sacrifice a creature."
public export
badIfDoneWithNeitherArm : Unspellable (Effect []) (\ok =>
  IfDone (Macros.sacrifice You (Macros.a Macros.creature)) Nothing Nothing {br = ok})
badIfDoneWithNeitherArm Oh impossible


||| "This creature deals 3 damage to any target. If you do, draw a card."
public export
badIfDoneOverAgentlessBody : Unspellable (Effect []) (\ok =>
  IfDone (DealDamage Macros.thisCreature (Lit 3) (Macros.target Macros.anyTarget))
         (Just (Macros.draw You (Lit 1))) Nothing {en = ok})
badIfDoneOverAgentlessBody Oh impossible


||| "Take an extra turn after this one. If you do, draw a card."
public export
badIfDoneOverScheduledBody : Unspellable (Effect []) (\ok =>
  IfDone (ExtraTurn You (Lit 1)) (Just (Macros.draw You (Lit 1))) Nothing {en = ok})
badIfDoneOverScheduledBody Oh impossible


||| "create a legendary legendary 20/20 black Avatar creature token"
public export
badTokenDuplicateSupertype : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1)
    (MkSupertypedToken (Just (Lit 20 ** Lit 20)) [Black] [Legendary, Legendary]
       (MkTypeLine [creatureType "Avatar"] [Creature]) [] (Just "Marit Lage"))
    {wf = ok})
badTokenDuplicateSupertype (Oh, Oh, Oh, Oh, Oh, Oh) impossible


public export
lessAsThoughCondition : AsThough []
lessAsThoughCondition = AsThoughLess Power (Lit 1)

public export
manaRunReductionFloor : CostShift []
manaRunReductionFloor =
  CostShiftRunWithFloor [Macros.pip White] (Lit 1) False

public export
nestedStaticConditionals : StaticEffect []
nestedStaticConditionals =
  Macros.ifSo (Macros.exists AnyPlayer)
    (Macros.ifSo (Macros.exists AnyPlayer) (KeepsUnspentMana You (UnspentMana Nothing)))

public export
nestedTurnPartWindows : StaticEffect []
nestedTurnPartWindows =
  OnlyDuring Combat Nothing
    (OnlyDuring MainPhase Nothing (KeepsUnspentMana You (UnspentMana Nothing)))

public export
repeatWithIndependentException : Repetition []
repeatWithIndependentException = AgainExcept (Macros.exists AnyPlayer)

public export
voteStartingWithSpecifiedPlayer : Effect []
voteStartingWithSpecifiedPlayer =
  Vote (Just Macros.anOpponent) (Macros.each AnyPlayer) Openly
       (ByLabel ["alpha", "beta"])

public export
oneWayResultShift : Effect []
oneWayResultShift =
  Sequentially [(Macros.rollDice You 1 6), ShiftResult (Just ShiftUp) (Lit 1)]

public export
objectScopedChaos : Effect []
objectScopedChaos = ChaosEnsues (Just Macros.thisRoom)

public export
abilityCounterRecipient : Effect []
abilityCounterRecipient =
  PutCounters (Lit 1) OwnKinds (Macros.a (AbilityHead AnyOnStack))

public export
removeOwnCounterKinds : Effect []
removeOwnCounterKinds = RemoveCounters (Just (Macros.exactly 1)) (Just OwnKinds) You

public export
namedAdditionalPartAnchor : Effect []
namedAdditionalPartAnchor =
  AdditionalPart (Just You) Upkeep (Just MainPhase) (Lit 1) Nothing

public export
badDelayedDoorDeixis : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
    [Spell (Delayed (UnlocksDoor You ThisDoor) [] Nothing (Macros.draw You (Lit 1))
                    {so = Absent})]
    Nothing {fl = ok})
badDelayedDoorDeixis MkFaceLaws impossible

public export
joinedDealerDamageComplement :
  LookbackComplement DamageDealing Object (Object \/ Player)
joinedDealerDamageComplement = MkLookbackComplement

public export
lastChosenPlayerRead :
  Predicate [choiceB PlayerC, choiceB PlayerC] Player
lastChosenPlayerRead = TheLastChosenPlayer


||| "This deals 3 damage to a chosen player. This deals 3 damage to that
||| player." Only a definite description refers to the choice [CR#607.2d];
||| the indefinite binds a second player, so the read is ambiguous.
public export
badIndefiniteChosenPlayerRead : Unspellable (Effect [choiceB PlayerC]) (\ok =>
  Sequentially [ DealDamage This (Lit 3) (Macros.a ChosenPlayer)
               , DealDamage This (Lit 3) (They {ok = ok}) ])
badIndefiniteChosenPlayerRead Refl impossible

public export
generalManaSymbolMatcher : Predicate [] Object
generalManaSymbolMatcher = ManaCostHas (Simple (Specific (OfColor Red)))

public export
playerItRead : Noun [MkBinding AD Player OneOf PlayerP] Player
playerItRead = They

public export
delayedDoorTraversal :
  effectNamesThisDoor
    (Delayed (UnlocksDoor You ThisDoor) [] Nothing (Macros.draw You (Lit 1))
             {so = Absent}) = True
delayedDoorTraversal = Refl

public export
distributiveGroupSurvives : Effect []
distributiveGroupSurvives =
  Sequentially
    [ Does (Macros.each Opponent) "Shuffle" (Shuffle They)
    , ChangeLife (Those PlayerW) (Down (Lit 1))
    ]

public export
secondChooserDevotionRead :
  Amount [qualityB Color, qualityB Color]
secondChooserDevotionRead =
  Devotion You TheLastChosenColor Nothing

public export
emblemGrantorRead : Noun [] Object
emblemGrantorRead = TheEmblemGrantor

public export
alternativeCostReadbacks : List (Predicate [] Object)
alternativeCostReadbacks =
  [ PaidCost (ByKeyword "Escape") Nothing
  , PaidCost (ByKeyword "Foretell") Nothing
  , PaidCost (ByKeyword "Bestow") Nothing
  , PaidCost (ByKeyword "Disguise") Nothing
  , PaidCost (ByKeyword "Mutate") Nothing
  , PaidCost (ByKeyword "Overload") Nothing
  , PaidCost (ByKeyword "Disturb") Nothing
  , PaidCost (ByKeyword "Dash") Nothing
  , PaidCost (ByKeyword "Evoke") Nothing
  , PaidCost (ByKeyword "Blitz") Nothing
  , PaidCost (ByKeyword "Cleave") Nothing
  , PaidCost (ByKeyword "Harmonize") Nothing
  , PaidCost (ByKeyword "Impending") Nothing
  , PaidCost (ByKeyword "Awaken") Nothing
  , PaidCost (ByKeyword "Buyback") Nothing
  , PaidCost (ByKeyword "Casualty") Nothing
  , PaidCost (ByKeyword "Squad") Nothing
  , PaidCost (ByKeyword "Offspring") Nothing
  , PaidCost (ByKeyword "Gift") Nothing
  , PaidCost (ByKeyword "Replicate") Nothing
  ]

public export
modalCostReadbacks : (Predicate [] Object, Amount [])
modalCostReadbacks =
  ( PaidCost (ByKeyword "Entwine") Nothing
  , TimesPaid (ByKeyword "Escalate") This
  )
