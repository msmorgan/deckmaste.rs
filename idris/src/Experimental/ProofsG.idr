module Experimental.ProofsG

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "This deals 3 damage to any target. This deals 1 damage to that permanent
||| or player."
public export
okUnionAnaphorAfterJoin : Effect []
okUnionAnaphorAfterJoin =
  Sequentially [ DealDamage This (Lit 3)
                   (Macros.target (Macros.kindJoin AnyPlayer Macros.creature))
               , DealDamage This (Lit 1) (That JoinW) ]

||| "This deals 3 damage to that permanent or player."
public export
badUnionAnaphorNoAntecedent : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3) (Macros.That JoinW OneOf {ok = ok}))
badUnionAnaphorNoAntecedent Refl impossible


||| "Destroy target creature. This deals 3 damage to that permanent or player."
public export
badUnionAnaphorOnObject : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , DealDamage This (Lit 3) (Macros.That JoinW OneOf {ok = ok}) ])
badUnionAnaphorOnObject Refl impossible


||| "This deals 3 damage to any target. Counter that spell or ability."
public export
badAbilityJoinAnaphorOnPlayerUnion : Unspellable (Effect []) (\ok =>
  Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
               , CounterSpell (Macros.That AbilityJoinW OneOf {ok = ok}) ])
badAbilityJoinAnaphorOnPlayerUnion Refl impossible


||| "Counter target activated ability. Counter that spell or ability."
public export
badAbilityJoinAnaphorOnAbility : Unspellable (Effect []) (\ok =>
  Sequentially [ CounterSpell (Macros.target (AbilityHead AnyActivated))
               , CounterSpell (Macros.That AbilityJoinW OneOf {ok = ok}) ])
badAbilityJoinAnaphorOnAbility Refl impossible



||| "each creature with a +1/+1 counter on it"
public export
okPlusOneCounterDescription : Predicate [] Object
okPlusOneCounterDescription =
  HasCounters (Just Macros.plusOnePlusOne) {kn = Present}

||| "each creature with a poison counter on it"
public export
badPoisonCounterDescription : Unspellable (Predicate [] Object) (\ok =>
  HasCounters (Just (Named "Poison")) {kn = Present {ok}})
badPoisonCounterDescription Refl impossible


||| "Cumulative upkeep {2}"
public export
okCostedCumulativeUpkeep : Ability
okCostedCumulativeUpkeep =
  KeywordAbility "CumulativeUpkeep"
                 (Just (ParamCost (Mana [Macros.generic 2]))) Nothing

||| "Cumulative upkeep"
public export
badBareCumulativeUpkeep : Unspellable Ability (\ok =>
  KeywordAbility "CumulativeUpkeep" Nothing Nothing {pf = ok})
badBareCumulativeUpkeep Oh impossible


||| "Cumulative upkeep {2}" on an enchantment card
public export
okCumulativeUpkeepOnPermanent : Card
okCumulativeUpkeepOnPermanent =
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Enchantment])
       [KeywordAbility "CumulativeUpkeep"
          (Just (ParamCost (Mana [Macros.generic 2]))) Nothing] Nothing

||| "Cumulative upkeep {2}"
public export
badCumulativeUpkeepOnSpell : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility "CumulativeUpkeep" (Just (ParamCost (Mana [Macros.generic 2]))) Nothing] Nothing {fl = ok})
badCumulativeUpkeepOnSpell MkFaceLaws impossible


||| "Renown 1 (When this creature deals combat damage to a player, …)"
public export
okRenownWithRenownExpansion : Ability
okRenownWithRenownExpansion =
  KeywordAbility "Renown" (Just (ParamNumber (Lit 1)))
                 (Just (Macros.renownExpansion 1))

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


||| "a first strike counter"
public export
okFirstStrikeKeywordCounter : CounterKind
okFirstStrikeKeywordCounter = KeywordCounter "FirstStrike"

||| "a cumulative upkeep counter"
public export
badCumulativeUpkeepCounter : Unspellable CounterKind (\ok =>
  KeywordCounter "CumulativeUpkeep" {ok = ok})
badCumulativeUpkeepCounter Oh impossible


||| "The same is true for menace and trample."
public export
okKeywordListOverGrant : Ability
okKeywordListOverGrant =
  AlsoForKeywords (Static (Conditionally
                             (Macros.exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")]))
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing Nothing)) AsLongAs
                             {st = Static.CondFirstDone}))
                  [TheKeyword "Menace", TheKeyword "Trample"]

||| "Creatures you control get +1/+1. The same is true for menace and trample."
public export
badKeywordListOnPlainLine : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Gets Adds (Macros.allOf Macros.creatureYouControl)
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


||| "a creature with protection from a color"
public export
okKeywordClassWithSort : Predicate [] Object
okKeywordClassWithSort =
  HasKeyword (AnyKeywordIn (MkKeywordFamily "Protection" (Just Color)))

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


||| "creature that was dealt damage by this creature this turn"
public export
okDamageTakenComplement : Predicate [] Object
okDamageTakenComplement =
  HappenedTo DamageTaken Lookback.ThisTurn
             (Just (Involving Macros.thisCreature))

||| "Destroy target creature that attacked with this creature this turn."
public export
badAttackerComplementOnObject : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo AttackDeclaration Lookback.ThisTurn (Just (Involving Macros.thisCreature {cp = ok})))
badAttackerComplementOnObject MkLookbackComplement impossible


public export
badPlayerCastComplement : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn (Just (Involving Macros.anOpponent {cp = ok})))
badPlayerCastComplement MkLookbackComplement impossible


||| "target spell cast from your graveyard"
public export
okCastFromGraveyard : Predicate [] Object
okCastFromGraveyard = CastFrom (Macros.graveyardOf You)

||| "Counter target spell cast from the stack."
public export
badCastFromStack : Unspellable (Predicate [] Object) (\ok =>
  CastFrom (ZoneAt Stack Bare) {pf = ok})
badCastFromStack Oh impossible


||| "… that was put somewhere from the battlefield this turn."
public export
okPlacementOriginBattlefield : Predicate [] Object
okPlacementOriginBattlefield =
  HappenedTo Placement Lookback.ThisTurn
             (Just (FromZones (FromZone [Macros.battlefieldZ]) Nothing))

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
           (Just (FromZones (FromZone [ZoneAt Stack Bare]) Nothing {ok = ok})))
badCastOriginFromStack Oh impossible


||| "if you've cast a spell from this turn"
public export
badEmptyOriginCoordination : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromZone []) Nothing {ok = ok})))
badEmptyOriginCoordination Oh impossible


||| "if you've cast a creature spell from your hand this turn"
public export
okOriginPayloadInvolving : Condition []
okOriginPayloadInvolving =
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromZone [Macros.handZ])
                    (Just (Involving (Macros.a Macros.creature)))))

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


||| "… that was put into a graveyard from the battlefield this turn."
public export
okPlacementIntoGraveyard : Predicate [] Object
okPlacementIntoGraveyard =
  HappenedTo Placement Lookback.ThisTurn
             (Just (IntoZone Macros.graveyardZ
                      (Just (FromZones (FromZone [Macros.battlefieldZ])
                                       Nothing))))

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


||| "if you shuffled your library this way"
public export
okShuffleLocusAtLibrary : Condition []
okShuffleLocusAtLibrary =
  Happened (VerbedAct "Shuffle") You Lookback.ThisWay
           (Just (AtZone Macros.yourLibrary))

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


||| "target creature spell, once it resolves"
public export
okResolvedCreatureSpell : Noun [] Object
okResolvedCreatureSpell =
  ResolvedPermanent (Macros.a (And [HasType Creature, Macros.spell]))

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


||| "If you control an artifact and an enchantment, …"
public export
okFlatConjunction : Condition []
okFlatConjunction =
  AndCond [ Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You])
          , Macros.exists (And [Macros.enchantment,
                                HasPossessor ControllerAx You]) ]

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


||| "unless you control an artifact"
public export
okUnlessOverNegatedCondition : StaticEffect []
okUnlessOverNegatedCondition =
  Conditionally (NotCond (Macros.exists (And [Macros.artifact,
                                              HasPossessor ControllerAx You])))
                (AltCost This Nothing) Unless {st = Static.CondFirstDone}

public export
badUnlessConjunction : Unspellable (StaticEffect []) (\ok =>
  Conditionally (AndCond [ Macros.exists (And [Macros.artifact, HasPossessor ControllerAx You])
                         , Macros.exists (And [Macros.enchantment, HasPossessor ControllerAx You]) ])
                (AltCost This Nothing) Unless {st = Static.CondFirstDone} {mk = ok})
badUnlessConjunction MkMarkingOk impossible


||| "… pays {1} for each artifact you control."
public export
okForEachScaledMana : Cost []
okForEachScaledMana =
  ScaledMana GenericUnit (Macros.forEach (And [Macros.artifact,
                                               HasPossessor ControllerAx You]))

||| "Counter target spell unless its controller pays {2}."
||| Refused as ScaledMana; Mana [Macros.generic 2] spells the flat cost.
public export
badLiteralScaledMana : Unspellable (Cost []) (\ok =>
  ScaledMana GenericUnit (Lit 2) {fe = ok})
badLiteralScaledMana Oh impossible


public export
badBareCountScaledMana : Unspellable (Cost []) (\ok =>
  ScaledMana GenericUnit (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You])) {fe = ok})
badBareCountScaledMana Oh impossible


||| "Target creature becomes a black Zombie in addition to its other types."
public export
okUnnamedAddition : Effect []
okUnnamedAddition =
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [Black]
                            (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                   Nothing

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


||| "For each opponent, you draw a card."
public export
okPluralForEach : Effect []
okPluralForEach = ForEachOf (Macros.each Opponent) (Draw You (Lit 1))

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


||| "At the beginning of your upkeep, draw a card."
public export
okSingularPartPossessor : Ability
okSingularPartPossessor =
  Triggered At (BeginningOf ThePart Upkeep (ByPlayer You)) [] Nothing []
           Nothing Nothing Nothing (Macros.draw You (Lit 1))

||| "At the beginning of all players' upkeep, draw a card." [CR#102.1]
public export
badPluralPartPossessor : Unspellable Ability (\ok =>
  Triggered At (BeginningOf ThePart Upkeep (ByPlayer (Macros.allOf AnyPlayer)) {pu = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badPluralPartPossessor Oh impossible


||| "on top of your library in any order, third from the top"
public export
okOneEndArrangedOrdinal : ZoneExpr []
okOneEndArrangedOrdinal =
  LibraryAt (OneEnd OnTop) (Just AnyOrder) (Just (Nth 3)) Bare

||| "Put those cards on the top or bottom of your library in any order."
public export
badDisjunctionOrdered : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt (EitherEnd Nothing) (Just AnyOrder) Nothing {af = ok} Bare)
badDisjunctionOrdered Oh impossible


||| "third from the top"
public export
okThirdFromTop : LibOrdinal
okThirdFromTop = Nth 3

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


||| "if there is no monarch"
public export
okNoHolderOnPlayer : Condition []
okNoHolderOnPlayer = NoHolder Monarch

||| "if there is no monstrous creature"
public export
badNoHolderOnObject : Unspellable (Condition []) (\ok =>
  NoHolder Monstrous {sc = ok})
badNoHolderOnObject Refl impossible


||| "your commander"
public export
okPossessedCommander : Noun [] Object
okPossessedCommander = Designated CommanderD You

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


||| "Creature cards in your graveyard have unearth {2}."
public export
okUnearthGrantInGraveyard : Ability
okUnearthGrantInGraveyard =
  Static (Gains (Macros.allOf (And [Macros.creature,
                                    InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting "Unearth" (Mana [Macros.generic 2])))

||| "Creature cards in your graveyard have warp {2}."
public export
badWarpGrantInGraveyard : Unspellable Ability (\ok =>
  Static (Gains (Macros.allOf (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting "Warp" (Mana [Macros.generic 2]))
                {ok = ok}))
badWarpGrantInGraveyard Oh impossible



public export
copyParticipleUnwritten : actNamesParticiple "Copy" = False
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

||| "This deals 3 damage to any target. If a player is dealt damage this way,
||| you draw a card."
public export
okDealtThisWayAfterDamage : Effect []
okDealtThisWayAfterDamage =
  Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
               , If (DealtThisWay AnyPlayer) (Draw You (Lit 1)) Nothing ]

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

||| "Target opponent loses 1 life for each attacking creature you control.
||| You gain that much life."
public export
okThatMuchAfterQuantity : Effect []
okThatMuchAfterQuantity =
  Sequentially [ Macros.losesLife (Macros.target Opponent)
                   (Macros.forEach (And [Attacking, Macros.creature,
                                         HasPossessor ControllerAx You]))
               , Macros.gainsLife You ThatMuch ]

||| "Flip a coin. Draw that many cards."
public export
badThatMuchAfterFlip : Unspellable (Effect []) (\ok =>
  Sequentially [(Macros.flipCoins You 1), Draw You (ThatMuch {ok})])
badThatMuchAfterFlip Refl impossible

||| "Flip a coin. If you win the flip, draw a card."
public export
okFlipArmAfterFlip : Effect []
okFlipArmAfterFlip =
  Sequentially [ (Macros.flipCoins You 1)
               , If (FlipCalled You WinsFlip) (Draw You (Lit 1)) Nothing ]

||| "If you win the flip, draw a card."
public export
badFlipArmWithoutFlip : Unspellable (Effect []) (\ok =>
  If (FlipCalled You WinsFlip {fl = ok}) (Draw You (Lit 1)) Nothing)
badFlipArmWithoutFlip Oh impossible

||| "Roll a d20. 1—9 | Draw a card."
public export
okResultsTableAfterRoll : Effect []
okResultsTableAfterRoll =
  Sequentially [ (Macros.rollDice You 1 20)
               , ResultsTable [MkRollRow (Macros.fromTo 1 9)
                                         (Macros.draw You (Lit 1))] ]

||| "1—9 | Draw a card."
public export
badTableWithoutRoll : Unspellable (Effect []) (\ok =>
  ResultsTable [Macros.rollRow (Macros.fromTo 1 9) (Draw You (Lit 1))] {ok})
badTableWithoutRoll Refl impossible

||| "Roll a d6."
public export
okSixSidedDie : Effect []
okSixSidedDie = RollDice You (Lit 1) (SidesOf 6)

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

||| a planeswalker card printing its starting loyalty
public export
okPlaneswalkerLoyaltyBox : Card
okPlaneswalkerLoyaltyBox =
  Macros.cardOf "" (Just [Macros.pip Blue]) [Legendary]
       (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [] (Macros.loyaltyBox 3)

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

||| an adventurer card whose inset frame is a named Adventure sorcery
public export
okNamedAdventure : Card
okNamedAdventure =
  Adventurer (Macros.frontFace "" (Just [Macros.pip Blue]) []
                               (MkTypeLine [] [Creature]) []
                               (Macros.printedBox (Just (1, 1))))
             (Macros.frontFace "" (Just [Macros.pip Blue]) []
                               (MkTypeLine [spellType "Adventure"] [Sorcery])
                               [] Nothing)

||| an adventurer card whose inset frame is a plain instant, naming no Adventure
public export
badUnnamedAdventure : Unspellable Card (\ok =>
  Adventurer (Macros.frontFace "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature]) []
                               (Macros.printedBox (Just (1, 1))))
             (Macros.frontFace "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant]) []
                               Nothing)
             {ai = ok})
badUnnamedAdventure Oh impossible

||| a flip card whose upside-down half is a creature
public export
okCreatureFlipHalf : Card
okCreatureFlipHalf =
  FlipCard (Macros.frontFace "" (Just [Macros.pip Green]) []
                             (MkTypeLine [] [Creature]) []
                             (Macros.printedBox (Just (1, 1))))
           (Macros.backFace "" [] (MkTypeLine [] [Creature]) []
                            (Macros.printedBox (Just (2, 2))))

||| a flip card whose upside-down half is an instant
public export
badSpellFlipHalf : Unspellable Card (\ok =>
  FlipCard (Macros.frontFace "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) []
                             (Macros.printedBox (Just (1, 1))))
           (Macros.backFace "" [] (MkTypeLine [] [Instant]) [] Nothing)
           {ah = ok})
badSpellFlipHalf Oh impossible


||| a transforming card whose back face prints no mana cost
public export
okTransformingBackWithoutCost : Card
okTransformingBackWithoutCost =
  Transforming (Macros.frontFace "" (Just [Macros.pip Green]) []
                                 (MkTypeLine [] [Creature]) []
                                 (Macros.printedBox (Just (1, 1))))
               (Macros.backFace "" [] (MkTypeLine [] [Creature]) []
                                (Macros.printedBox (Just (1, 1))))

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
       [ Macros.activated (Mana [Macros.generic 1]) (Draw You (Lit 1)) ] Nothing {fl = ok})
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

||| "the number of creatures that died this turn"
public export
okDeathTally : Amount []
okDeathTally =
  EventTally TallyCount Death (Macros.a Macros.creature) Lookback.ThisTurn
             Nothing

||| "the amount of creatures that died this turn"
public export
badDeathSum : Unspellable (Amount []) (\ok =>
  EventTally TallySum Death (Macros.a Macros.creature) Lookback.ThisTurn Nothing
           {qm = ok})
badDeathSum Oh impossible

||| "the greatest life total among all players"
public export
okPlayerAggregate : Amount []
okPlayerAggregate =
  Aggregate MaxOf (PlayerStatAxis LifeTotal) (Macros.allOf AnyPlayer)

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
                                        (Draw You (Lit 1)) {lt = ok}]])
badAmountRollRow Oh impossible

public export
badCreatureHalfRead : Unspellable (Effect []) (\ok =>
  Sequentially
    [ DealDamage This (Lit 3)
        (Macros.target (Macros.kindJoin AnyPlayer (HasType Planeswalker)))
    , (Macros.discard
        (EitherOf (Pro (UnionHalf PlayerW) OneOf)
                  (Macros.controllerOf (Macros.That (TypeW Creature) OneOf {ok = ok})))
        (Macros.a (InZone Macros.handZ))) ])
badCreatureHalfRead Refl impossible

||| "Whenever a creature attacks a player"
public export
okPlayerAttackDefender : GameEvent []
okPlayerAttackDefender =
  Attacks (Macros.a Macros.creature) (OneDefender (Macros.a AnyPlayer))

||| "Whenever a creature attacks a planeswalker or a creature"
public export
badMixedAttackDefenderHalves : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a (Joined (HasType Planeswalker)
                                         (HasType Creature))) {at = ok}))
badMixedAttackDefenderHalves Oh impossible

||| "This deals 3 damage to any target."
public export
okJoinDamageRecipient : Effect []
okJoinDamageRecipient =
  DealDamage This (Lit 3) (Macros.target Macros.anyTarget)

||| "This deals 3 damage to a land or a land."
public export
badSameKindJoinDamage : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3)
             (Macros.a (Joined (HasType Land) (HasType Land))) {rk = ok})
badSameKindJoinDamage JoinTakes impossible


||| "creature that could block each attacking creature"
public export
okCouldBlockAttacker : Predicate [] Object
okCouldBlockAttacker =
  CombatRel CouldBlock (Macros.allOf (And [Macros.creature, Attacking]))

||| "creature that could block target creature card in your graveyard"
public export
badCouldBlockGraveyardRelatum : Unspellable (Predicate [] Object) (\ok =>
  CombatRel CouldBlock (Macros.target (And [Macros.creature,
                                            InZone (Macros.graveyardOf You)])) {ok = ok})
badCouldBlockGraveyardRelatum Oh impossible


||| "This creature blocks an attacking creature."
public export
okCreatureBecomesBlocking : Effect []
okCreatureBecomesBlocking =
  BecomesBlocking Macros.thisCreature
                  (Macros.a (And [Macros.creature, Attacking]))

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


||| "During target opponent's next turn, …"
public export
okSingularNextTurnSpan : Duration []
okSingularNextTurnSpan = DuringNextTurnOf (Macros.target Opponent)

||| "During each opponent's next turn, ..."
public export
badPluralNextTurnSpan : Unspellable (Duration []) (\ok =>
  DuringNextTurnOf (Macros.each Opponent) {one = ok})
badPluralNextTurnSpan Refl impossible


||| "Flip a coin. Take an extra turn for each coin that comes up heads."
public export
okCoinsShowingAfterFlip : Effect []
okCoinsShowingAfterFlip =
  Sequentially [ (Macros.flipCoins You 1)
               , ExtraTurn You (CoinsShowing Heads) ]

||| "Take an extra turn for each coin that comes up heads."
public export
badCoinsShowingWithoutFlip : Unspellable (Effect []) (\ok =>
  ExtraTurn You (CoinsShowing Heads {fl = ok}))
badCoinsShowingWithoutFlip Oh impossible

||| "Roll two d6. If you rolled 7, sacrifice this creature."
public export
okTotalAfterRoll : Effect []
okTotalAfterRoll =
  Sequentially [ (Macros.rollDice You 2 6)
               , Macros.ifThen (CompareAmt Macros.theTotal Eq (Lit 7))
                               (Macros.sacrifice You Macros.thisCreature) ]

||| "If you rolled 7, sacrifice this creature."
public export
badTotalWithoutRoll : Unspellable (Effect []) (\ok =>
  Macros.ifThen (CompareAmt (TheOutcome RollResult {ok}) Eq (Lit 7))
                (Macros.sacrifice You Macros.thisCreature))
badTotalWithoutRoll Refl impossible

||| "creature that died this turn"
public export
okObjectDeathLookbackSubject : Predicate [] Object
okObjectDeathLookbackSubject = HappenedTo Death Lookback.ThisTurn Nothing

||| "creature that won a coin flip this turn"
public export
badCreatureWonFlip : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo FlipWin Lookback.ThisTurn Nothing {sb = ok})
badCreatureWonFlip MkLookbackSubject impossible

||| "the amount of dice you rolled this turn"
public export
badRollAsMagnitude : Unspellable (Amount []) (\ok =>
  EventTally TallySum DiceRoll You Lookback.ThisTurn Nothing {qm = ok})
badRollAsMagnitude Oh impossible

public export
youWonAFlipThisTurn : Condition []
youWonAFlipThisTurn = Happened FlipWin You Lookback.ThisTurn Nothing

public export
youRolledADieThisTurn : Condition []
youRolledADieThisTurn = Happened DiceRoll You Lookback.ThisTurn Nothing

||| "Roll two d20. Ignore the lowest roll."
public export
okIgnoreAfterRoll : Effect []
okIgnoreAfterRoll =
  Sequentially [ (Macros.rollDice You 2 20)
               , IgnoreOutcomes (IgnoreExtreme LowestRoll) ]

||| "Ignore the lowest roll."
public export
badIgnoreWithoutRoll : Unspellable (Effect []) (\ok =>
  IgnoreOutcomes (IgnoreExtreme LowestRoll) {ok})
badIgnoreWithoutRoll Oh impossible

||| "Roll five d6. Store those results on this creature."
public export
okStoreResultsAfterRoll : Effect []
okStoreResultsAfterRoll =
  Sequentially [ (Macros.rollDice You 5 6)
               , StoreResults Macros.thisCreature ]

||| "Store those results on this creature."
public export
badStoreResultsWithoutRoll : Unspellable (Effect []) (\ok =>
  StoreResults Macros.thisCreature {ok})
badStoreResultsWithoutRoll Refl impossible

||| "If you would roll one or more d6, instead roll that many of those dice."
public export
okThoseDiceAfterRollEvent : Effect []
okThoseDiceAfterRollEvent =
  Macros.ifWouldInstead (RollsDice You ManyDice (SidedDie 6) AnyResult)
    (RollDice You ThatMuch ThoseDice) Nothing

||| "Roll that many dice."
public export
badAnaphoricSidesWithoutRoll : Unspellable (Effect []) (\ok =>
  RollDice You (Lit 1) (ThoseDice {ok}))
badAnaphoricSidesWithoutRoll Refl impossible

public export
afterATwoDieRoll : Bindings
afterATwoDieRoll = effIntro (the (Effect []) (Macros.rollDice You 2 6))

||| "if you rolled doubles"
public export
okRolledDoublesAfterRoll : Condition ProofsG.afterATwoDieRoll
okRolledDoublesAfterRoll = RolledDoubles

||| "If you rolled doubles, sacrifice this creature."
public export
badRolledDoublesWithoutRoll : Unspellable (Condition []) (\ok =>
  RolledDoubles {ok})
badRolledDoublesWithoutRoll Refl impossible

public export
afterACoinFlip : Bindings
afterACoinFlip = effIntro (the (Effect []) (Macros.flipCoins You 1))

||| "a player whose coin comes up tails"
public export
okCoinCameUpOnPlayer : Predicate ProofsG.afterACoinFlip Player
okCoinCameUpOnPlayer = CoinCameUp Tails

||| "an ability whose coin comes up tails"
public export
badCoinCameUpOnAbility :
  Unspellable (Predicate ProofsG.afterACoinFlip Ability) (\ok =>
    CoinCameUp Tails {rk = ok})
badCoinCameUpOnAbility Oh impossible

||| "Whenever you roll a 4 or higher, …"
public export
okBoundedRollTest : GameEvent []
okBoundedRollTest =
  RollsDice You OneDie AnyDie (ResultIn (Range (Just 4) Nothing))

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
  Macros.theRest {ok})
badRestWithoutAPartition Oh impossible

||| "Choose any number of target creatures. Destroy the rest."
public export
badRestAfterTargetChoice : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.choose (Described (TargetDet Macros.anyNumber) Macros.creature)
               , Macros.destroy (Macros.theRest {ok}) ])
badRestAfterTargetChoice Oh impossible

public export
afterChoiceRestDisposed : Bindings
afterChoiceRestDisposed =
  effIntro (the (Effect [])
    (Sequentially [ Macros.choose (Macros.counted (Macros.upTo 1) Macros.creature)
                  , Macros.destroy Macros.theRest ]))

||| "Choose up to one creature. Destroy the rest. Destroy the rest."
public export
badChoiceRestDisposedTwice :
  Unspellable (Noun ProofsG.afterChoiceRestDisposed Object) (\ok => Macros.theRest {ok})
badChoiceRestDisposedTwice Oh impossible

||| "an opponent who controls more lands than they control"
public export
badMemberInComparisonBound : Unspellable (Predicate [] Player) (\ok =>
  CompareOver Opponent (Macros.countOf (And [Macros.land, HasPossessor ControllerAx You]))
              Greater (Macros.countOf (And [Macros.land, HasPossessor ControllerAx (They {ok})])))
badMemberInComparisonBound Refl impossible

||| "the number of basic land types among lands you control"
public export
okBasicLandTypeAxis : Amount []
okBasicLandTypeAxis =
  DistinctCount (SubtypeAxis Land BasicOnly)
                (Macros.allOf (And [Macros.land,
                                    HasPossessor ControllerAx You]))

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
  Unspellable (Noun ProofsG.afterPassivePayment Player) (\ok => Macros.That PlayerW OneOf {ok})
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
    (Sequentially [ Macros.lookAt (Macros.topSlice (Lit 1)), Macros.shuffle ]))

public export
badReadsShuffledLibraryCard :
  Unspellable (Noun ProofsG.afterShuffledLook Object) (\ok => Macros.That CardW OneOf {ok})
badReadsShuffledLibraryCard Refl impossible

public export
afterShuffledIntoLook : Bindings
afterShuffledIntoLook =
  effIntro (the (Effect [])
    (Sequentially [ Macros.lookAt (Macros.topSlice (Lit 1)), Macros.shuffleInto You This ]))

public export
badReadsShuffledIntoLibraryCard :
  Unspellable (Noun ProofsG.afterShuffledIntoLook Object) (\ok => Macros.That CardW OneOf {ok})
badReadsShuffledIntoLibraryCard Refl impossible

||| "if you control four or more creatures"
public export
okCompareCountSubject : Condition []
okCompareCountSubject =
  CompareAmt (Macros.countOf Macros.creatureYouControl) AtLeast (Lit 4)

||| "if up to three is 4 or greater"
public export
badCompareCeilingSubject : Unspellable (Condition []) (\ok =>
  CompareAmt (UpTo (Lit 3)) AtLeast (Lit 4) {rd = ok})
badCompareCeilingSubject Oh impossible

||| "This creature enters with your choice of a +1/+1 counter or a first
||| strike counter on it."
public export
okCounterMenu : StaticEffect []
okCounterMenu =
  EntersRider Macros.thisCreature
    (WithCounters (Lit 1)
       (ChosenKind [Macros.plusOnePlusOne, KeywordCounter "FirstStrike"])
       Fresh)

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

||| "Put a poison counter on target opponent."
public export
okPoisonCounterLabel : Effect []
okPoisonCounterLabel =
  PutCounters (Lit 1) (PrintedKind (Named "Poison")) (Macros.target Opponent)

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

public export
afterCountersPut : Bindings
afterCountersPut =
  eventAfter (the (GameEvent [])
    (CounterEvent CounterPut Nothing
                  (Macros.a (And [Macros.creature, OtherThan This]))
                  ManyCounters (Just You) False))

||| "Put that many counters of each of those kinds on this creature."
public export
okThoseKindsAfterCountersPut : Effect ProofsG.afterCountersPut
okThoseKindsAfterCountersPut = PutCounters ThatMuch ThoseKinds This

||| "Put a counter of each of those kinds on target creature."
public export
badThoseKindsUnannounced : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (ThoseKinds {ok}) (Macros.target Macros.creature))
badThoseKindsUnannounced Refl impossible

||| "For each color among permanents you control, add one mana of that color."
public export
okChosenColorPerColor : Effect []
okChosenColorPerColor =
  ForEachKindOf ColorAxis
    (Just (Macros.allOf (And [Permanent, HasPossessor ControllerAx You]))) Color
    (AddMana You (Lit 1) (OfChosenColor Nothing) [])

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

||| "Cumulative upkeep {2}"
public export
okManaCumulativeUpkeep : AbilityAt []
okManaCumulativeUpkeep = Macros.cumulativeUpkeep (Mana [Macros.generic 2])

||| "Cumulative upkeep — an opponent loses 1 life."
public export
badOpponentPaysYourCost : Unspellable (AbilityAt []) (\ok =>
  Macros.cumulativeUpkeep (Do (Macros.losesLife Macros.anOpponent (Lit 1))) {py = ok})
badOpponentPaysYourCost Oh impossible

||| "this creature gets +1/+1"
public export
okAddPtUpward : StaticEffect []
okAddPtUpward =
  Gets Adds Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 1))

||| "this creature has base power and toughness -1/-1"
public export
badSetBasePtDownward : Unspellable (StaticEffect []) (\ok =>
  Gets Sets Macros.thisCreature (PtDown (Lit 1)) (PtDown (Lit 1)) {lo = ok})
badSetBasePtDownward Oh impossible

||| "this creature loses 1/1"
public export
badLosePtOp : Unspellable (StaticEffect []) (\ok =>
  Gets Loses Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 1)) {lo = ok})
badLosePtOp Oh impossible

||| "whenever one or more time counters are put on this enchantment"
public export
okManyCountersOnPlacement : GameEvent []
okManyCountersOnPlacement =
  CounterEvent CounterPut (Just (Named "Time")) Macros.thisEnchantment
               ManyCounters Nothing False

||| "when the last time counter is put on this enchantment"
public export
badLastCounterOnPlacement : Unspellable (GameEvent []) (\ok =>
  CounterEvent CounterPut (Just (Named "Time")) Macros.thisEnchantment LastCounter Nothing False
               {lb = ok})
badLastCounterOnPlacement Oh impossible

||| "target monocolored permanent" — exactly one color is the printed lemma
public export
monocoloredIsOneColor : Predicate [] Object
monocoloredIsOneColor = ColorCount Eq 1

||| "target permanent that's exactly zero colors"
public export
badExactlyZeroColors : Unspellable (Predicate [] Object) (\ok =>
  ColorCount Eq 0 {ok = ok})
badExactlyZeroColors Oh impossible

||| "target permanent that's exactly six colors"
public export
badExactlySixColors : Unspellable (Predicate [] Object) (\ok =>
  ColorCount Eq 6 {ok = ok})
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


||| "Whenever you scry, …"
public export
okPatientlessScry : GameEvent []
okPatientlessScry = VerbedEvent (Just You) "Scry" Nothing Nothing False

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


||| "Whenever a creature transforms into a Phyrexian, …"
public export
okIntransitiveBecomes : GameEvent []
okIntransitiveBecomes =
  VerbedEvent Nothing "Transform" (Just (Macros.a Macros.creature))
              (Just (HasSubtype (creatureType "Phyrexian"))) False

||| "Whenever a card is milled into a Phyrexian, …"
public export
badBecomesWithoutIntransitive : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Mill" (Just (Macros.a (InZone (ZoneAt Library Bare))))
              (Just (HasSubtype (creatureType "Phyrexian"))) False {bc = ok})
badBecomesWithoutIntransitive Oh impossible


||| "Whenever you create one or more tokens, …"
public export
okTokensCreatedByYou : GameEvent []
okTokensCreatedByYou =
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken) False (Just You)
                Nothing

||| "Whenever an effect and you would create one or more tokens, …"
public export
badTokensCreatedByCauserAndPlayer : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                True (Just You) Nothing {vo = ok})
badTokensCreatedByCauserAndPlayer Oh impossible


||| "Whenever one or more tokens are created under your opponents' control, …"
public export
badTokensCreatedUnderPlural : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                False Nothing (Just (PlayerGroup YourOpponents)) {vo = ok})
badTokensCreatedUnderPlural Oh impossible


||| "Whenever an opponent creates one or more tokens, …"
public export
badTokensCreatedByMintingNoun : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                False (Just Macros.anOpponent) Nothing {vo = ok})
badTokensCreatedByMintingNoun Oh impossible


||| "Whenever you discard a card, …"
public export
okDiscardFromHand : GameEvent []
okDiscardFromHand =
  VerbedEvent (Just You) "Discard" (Just (Macros.a (InZone Macros.handZ)))
              Nothing False

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


||| "spell that targets this creature"
public export
okSpellTargeter : Predicate [] Object
okSpellTargeter = Targets Macros.thisCreature SomeTarget

||| "player that targets this creature"
public export
badPlayerTargeter : Unspellable (Predicate [] Player) (\ok =>
  Targets Macros.thisCreature SomeTarget {tr = ok})
badPlayerTargeter SpellTargets impossible
badPlayerTargeter AbilityTargets impossible
badPlayerTargeter (EitherTargets _ _) impossible


||| "Whenever this creature becomes the target of a spell, …"
public export
okSpellTargetingEvent : GameEvent []
okSpellTargetingEvent =
  BecomesTarget Macros.thisCreature (Macros.a Macros.spell)

||| "Whenever you become the target of a player, …"
public export
badPlayerTargetingEvent : Unspellable (GameEvent []) (\ok =>
  BecomesTarget You (Macros.a AnyPlayer) {tr = ok})
badPlayerTargetingEvent SpellTargets impossible
badPlayerTargetingEvent AbilityTargets impossible
badPlayerTargetingEvent (EitherTargets _ _) impossible


||| "Exchange life totals with target opponent."
public export
okExchangeTwoParties : Effect []
okExchangeTwoParties = ExchangeLife (Both You (Macros.target Opponent))

||| "Exchange life totals with target opponent"
||| Refused with one party; Both You (target Opponent) spells the sentence.
public export
badExchangeOneParty : Unspellable (Effect []) (\ok =>
  ExchangeLife (Macros.target Opponent) {tp = ok})
badExchangeOneParty Oh impossible


||| "You and your opponents exchange life totals."
public export
badExchangePluralParty : Unspellable (Effect []) (\ok =>
  ExchangeLife (Both You (PlayerGroup YourOpponents)) {tp = ok})
badExchangePluralParty Oh impossible


||| "a creature with power or toughness 2 or greater"
public export
okSingleScopeAxisComparison : Predicate [] Object
okSingleScopeAxisComparison =
  Compare [CharAxis Power, CharAxis Toughness] AtLeast (Lit 2)

||| "a creature with power or life total 3 or greater"
public export
badMixedAxisComparison : Unspellable (Predicate [] Object) (\ok =>
  Compare [CharAxis Power, PlayerStatAxis LifeTotal] Greater (Lit 1) {at = ok})
badMixedAxisComparison (NextAxis _ (LastAxis _)) impossible


||| "{T}: This deals X damage to any target, where X is 3."
public export
okActivatedOpensItsOwnLetter : AbilityAt []
okActivatedOpensItsOwnLetter =
  Macros.activated TapSymbol
    (Sequentially [ DealDamage This (LetterVal X)
                      (Macros.target Macros.anyTarget)
                  , Define X (Lit 3) ])

||| "{T}: … , where X is 3"
public export
badActivatedClosesCardLetter :
  Unspellable (AbilityAt (costLetters (Just [Variable]))) (\ok =>
    Macros.activated TapSymbol (Define X (Lit 3) {ok = ok}))
badActivatedClosesCardLetter Oh impossible


||| "You may cast spells as though they had flash."
public export
okObjectPremiseAtCast : StaticEffect []
okObjectPremiseAtCast =
  Deontic You Permit ["Cast"] Agent Nothing
          (DeonticCounterpart (Macros.allOf Macros.spell))
          (Just (AsThoughOf (HasKeyword (TheKeyword "Flash"))))
          (PlayRider Nothing Nothing Nothing False ItsOwnCost)

||| "This creature can attack as though it were mana of any color."
public export
badManaPremiseAtAttack : Unspellable (StaticEffect []) (\ok =>
  Deontic Macros.thisCreature Permit ["Attack"] Agent Nothing NoDeonticPatient
          (Just (AsThoughMana Nothing MatchAnyColor Nothing)) NoDeonticRider
          {at = ok})
badManaPremiseAtAttack Oh impossible


||| "This creature can't be blocked by more than one creature."
public export
okBlockBoundOnBlock : StaticEffect []
okBlockBoundOnBlock =
  Deontic Macros.thisCreature Forbid ["Block"] Patient (Just (MoreThan (Lit 1)))
          (DeonticCounterpart (Macros.allOf Macros.creature)) Nothing
          NoDeonticRider

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


||| "Whenever a player taps a land for mana, …"
public export
okForManaOnTap : GameEvent []
okForManaOnTap =
  VerbedEvent (Just (Macros.a AnyPlayer)) "Tap" (Just (Macros.a Macros.land))
              Nothing True

||| "Whenever you sacrifice a creature for mana, …"
public export
badForManaOnNontap : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Sacrifice" (Just (Macros.a Macros.creature)) Nothing True
              {fm = ok})
badForManaOnNontap Oh impossible


public export
afterALandTapForMana : Bindings
afterALandTapForMana =
  eventAfter (the (GameEvent [])
    (VerbedEvent (Just (Macros.a AnyPlayer)) "Tap" (Just (Macros.a Macros.land))
                 Nothing True))

||| "one mana of any type that land produced"
public export
okProducedByTapEvent : ProducedMana ProofsG.afterALandTapForMana
okProducedByTapEvent = ProducedByEvent (That (TypeW Land))

||| "Add one mana of any type that land produced"
public export
badProducedByEventWithoutEvent : Unspellable (ProducedMana []) (\ok =>
  ProducedByEvent (Macros.a Macros.land) {pm = ok})
badProducedByEventWithoutEvent Refl impossible


public export
afterManaAdded : Bindings
afterManaAdded =
  effIntro (the (Effect []) (AddMana You (Lit 1) (Runs [[Colorless]]) []))

||| "You don't lose this mana as steps and phases end."
public export
okThisManaAfterAdd : StaticEffect ProofsG.afterManaAdded
okThisManaAfterAdd = KeepsUnspentMana You ThisMana

||| "You don't lose this mana as steps and phases end"
public export
badThisManaWithoutAdd : Unspellable (StaticEffect []) (\ok =>
  KeepsUnspentMana You (ThisMana {ok = ok}))
badThisManaWithoutAdd Refl impossible


||| "Transform target creature."
public export
okTurnOverOnField : Effect []
okTurnOverOnField = TurnOver (Macros.target Macros.creature)

||| "Transform target creature card in your graveyard."
public export
badTurnOverOffField : Unspellable (Effect []) (\ok =>
  TurnOver (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
           {ok})
badTurnOverOffField Oh impossible


||| "Put target creature card from your graveyard onto the battlefield
||| transformed."
public export
okTransformedArrivalOnField : Effect []
okTransformedArrivalOnField =
  Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
       Macros.battlefieldZ [EntersTransformed]

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
                  , SeparateIntoPiles Macros.anOpponent ((Macros.It ManyOf)) 2 []
                  , Macros.move (Macros.That CardW ManyOf {ok = ok}) Macros.handZ ]) ]
       Nothing)
badCardWordReadsPiles Refl impossible


||| "Reveal the top five cards of your library. Put those piles into your hand."
public export
badPileWordWithoutAPartition : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , Macros.move (Macros.That PileW ManyOf {ok = ok}) Macros.handZ ]) ]
       Nothing)
badPileWordWithoutAPartition Refl impossible


||| "Reveal the top five cards of your library. An opponent separates them
||| into two piles. Put one pile into your hand."
public export
okOnePileAfterPartition : Effect []
okOnePileAfterPartition =
  Sequentially [ Macros.revealCards (Macros.topSlice (Lit 5))
               , SeparateIntoPiles Macros.anOpponent Them 2 []
               , Macros.move Macros.onePile Macros.handZ ]

||| "Put one pile into your hand."
public export
badPilePartitiveWithoutAPartition : Unspellable (Effect []) (\ok =>
  Macros.move (Macros.onePile {ok = ok}) Macros.handZ)
badPilePartitiveWithoutAPartition Refl impossible


||| "Put each card in the pile of your choice into your hand."
public export
okMembershipInAPile : Card
okMembershipInAPile =
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent Them 2 []
                  , Macros.move (Macros.allOf (And [IsCard,
                                    InPile (Macros.pileOfChoice You)]))
                                Macros.handZ ]) ]
       Nothing

public export
badMembershipInANonPile : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , Macros.move (Macros.allOf (And [IsCard, InPile ((Macros.It ManyOf)) {pm = ok}]))
                                Macros.handZ ]) ]
       Nothing)
badMembershipInANonPile PilePartitive impossible
badMembershipInANonPile ThatPile impossible
badMembershipInANonPile ThosePiles impossible


||| "Turn target creature face down."
public export
okStatusOnBattlefieldNoun : Effect []
okStatusOnBattlefieldNoun =
  SetStatus FaceDown (Macros.target Macros.creature)

public export
badPileFaceAsAStatus : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent ((Macros.It ManyOf)) 2 []
                  , SetStatus FaceDown (Macros.That PileW ManyOf) {ok = ok} ]) ]
       Nothing)
badPileFaceAsAStatus Oh impossible


||| "If one or more +1/+1 counters would be put on a creature you control,
||| that many plus one are put instead."
public export
okInterceptsCounterEvent : StaticEffect []
okInterceptsCounterEvent =
  Intercepts (Macros.manyCounterEvent CounterPut Macros.plusOnePlusOne
                                      (Macros.a Macros.creatureYouControl))
             [] Nothing
             (PutCounters (Plus ThatMuch (Lit 1))
                          (PrintedKind Macros.plusOnePlusOne) It)
             Repeatedly Nothing

public export
badTriggeringReplaced : Unspellable (StaticEffect []) (\ok =>
  Intercepts (Triggers (Macros.a (And [ AbilityHead AnyTriggered
                                      , AbilityOf (Macros.a (And [Permanent, HasPossessor ControllerAx You])) ])))
             [] Nothing (Draw You (Lit 1)) Repeatedly Nothing {ok})
badTriggeringReplaced Oh impossible


||| "If an ability of a creature you control triggers, it triggers an
||| additional time."
public export
okMultipliedTrigger : StaticEffect []
okMultipliedTrigger =
  TriggersAdditionally
    (Triggers (Macros.a (And [ AbilityHead AnyTriggered
                             , AbilityOf
                                 (Macros.a Macros.creatureYouControl) ])))
    (Macros.exactly 1)

||| "If a creature you control dies, that ability triggers an additional time."
public export
badMultipliedNonTrigger : Unspellable (StaticEffect []) (\ok =>
  TriggersAdditionally (Dies (Macros.a Macros.creatureYouControl))
                       (Macros.exactly 1) {ok})
badMultipliedNonTrigger Oh impossible


||| "unlock a locked door of a Room you control"
public export
okUnlockRoomDoor : Effect []
okUnlockRoomDoor =
  Unlock (DoorOf (Just Locked)
            (Macros.targets (Macros.upTo 1)
               (And [HasSubtype (enchantmentType "Room"),
                     HasPossessor ControllerAx You])))

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


||| "Sacrifice a creature. If you do, draw a card."
public export
okIfDoneWithArm : Effect []
okIfDoneWithArm =
  IfDone (Macros.sacrifice You (Macros.a Macros.creature))
         (Just (Macros.draw You (Lit 1))) Nothing

||| "Sacrifice a creature."
public export
badIfDoneWithNeitherArm : Unspellable (Effect []) (\ok =>
  IfDone (Macros.sacrifice You (Macros.a Macros.creature)) Nothing Nothing {br = ok})
badIfDoneWithNeitherArm Oh impossible


||| "This creature deals 3 damage to any target. If you do, draw a card."
public export
badIfDoneOverAgentlessBody : Unspellable (Effect []) (\ok =>
  IfDone (DealDamage Macros.thisCreature (Lit 3) (Macros.target Macros.anyTarget))
         (Just (Draw You (Lit 1))) Nothing {en = ok})
badIfDoneOverAgentlessBody Oh impossible


||| "Take an extra turn after this one. If you do, draw a card."
public export
badIfDoneOverScheduledBody : Unspellable (Effect []) (\ok =>
  IfDone (ExtraTurn You (Lit 1)) (Just (Draw You (Lit 1))) Nothing {en = ok})
badIfDoneOverScheduledBody Oh impossible


||| "create a legendary 20/20 black Avatar creature token named Marit Lage"
public export
okTokenSingleSupertype : Effect []
okTokenSingleSupertype =
  Macros.create (Lit 1)
    (MkSupertypedToken (Just (Lit 20 ** Lit 20)) [Black] [Legendary]
       (MkTypeLine [creatureType "Avatar"] [Creature]) [] (Just "Marit Lage"))

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
  Conditionally {bs = []} {condBase = []}
    {staticBase = condIntro (Macros.exists {bs = []} AnyPlayer)}
    (Macros.exists {bs = []} AnyPlayer)
    (Conditionally {bs = condIntro (Macros.exists {bs = []} AnyPlayer)}
      {condBase = condIntro (Macros.exists {bs = []} AnyPlayer)}
      {staticBase = condIntro
        (Macros.exists {bs = condIntro (Macros.exists {bs = []} AnyPlayer)} AnyPlayer)}
      (Macros.exists {bs = condIntro (Macros.exists {bs = []} AnyPlayer)} AnyPlayer)
      (KeepsUnspentMana You (UnspentMana Nothing)) IfSo) IfSo

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
    [Spell (Delayed (UnlocksDoor You ThisDoor) [] Nothing (Draw You (Lit 1))
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
    (Delayed (UnlocksDoor You ThisDoor) [] Nothing (Draw You (Lit 1))
             {so = Absent}) = True
delayedDoorTraversal = Refl

public export
distributiveGroupSurvives : Effect []
distributiveGroupSurvives =
  Sequentially
    [ Enact (Just (Macros.each Opponent)) "Shuffle" (Shuffle They)
    , ChangeLife (Macros.That PlayerW ManyOf) (Down (Lit 1))
    ]

public export
secondChooserDevotionRead :
  Amount [qualityB Color, qualityB Color]
secondChooserDevotionRead =
  Devotion You TheLastChosenColor Nothing

public export
emblemGrantorRead : Noun [] Object
emblemGrantorRead = TheGrantor EmblemMarker

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
