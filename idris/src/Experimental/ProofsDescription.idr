module Experimental.ProofsDescription

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "a creature target opponent controls"
public export
okControlledByOne : Predicate [] Object
okControlledByOne = HasPossessor ControllerAx (Macros.target Opponent)

||| "a creature two target opponents control"
public export
badControlledByGroup : Unspellable (Predicate [] Object) (\ok =>
  HasPossessor ControllerAx (Described (TargetDet (Macros.exactly 2)) Opponent) {ps = ok})
badControlledByGroup Oh impossible

||| "each of up to two target creatures"
public export
okEachOfGroup : Noun [] Object
okEachOfGroup = EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature)

||| "each of target creature"
public export
badEachOfSingular : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.target Macros.creature) {pl = ok})
badEachOfSingular Refl impossible

||| "Look at the top card of target player's library."
public export
okSliceOfOnePossessor : Instruction []
okSliceOfOnePossessor =
  Macros.lookAt (LibrarySlice OnTop (Lit 1) (Macros.target AnyPlayer))

||| "Look at the top card of two target players' library."
public export
badSliceOfCountedPossessor : Unspellable (Instruction []) (\ok =>
  Macros.lookAt (LibrarySlice OnTop (Lit 1)
                              (Described (TargetDet (Macros.exactly 2)) AnyPlayer) {sp = ok}))
badSliceOfCountedPossessor Oh impossible

||| "Tap target creature an opponent controls. That player loses 1 life."
public export
okThatPlayer : Instruction []
okThatPlayer =
  Sequentially [SetStatus Tapped (Macros.target (And [Macros.creature,
                  HasPossessor ControllerAx Macros.anOpponent])),
                Macros.losesLife (Macros.That PlayerW OneOf) (Lit 1)]

public export
badDisjunctAntecedent : Unspellable (Instruction []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target (Or [And [Macros.creature, HasPossessor ControllerAx Macros.anOpponent],
                                        And [Macros.land, HasPossessor ControllerAx You]])),
                Macros.losesLife (Macros.That PlayerW OneOf {ok}) (Lit 1)])
badDisjunctAntecedent Refl impossible

||| "Tap target creature with flying."
public export
okKeywordConjunction : Instruction []
okKeywordConjunction =
  SetStatus Tapped (Macros.target (And [Macros.creature,
                                        HasKeyword (TheKeyword "Flying")]))

||| "Tap target creature with flying that doesn't have flying."
public export
okKeywordSelfNegation : Instruction []
okKeywordSelfNegation =
  SetStatus Tapped (Macros.target (And [Macros.creature, HasKeyword (TheKeyword "Flying"),
                            Not (HasKeyword (TheKeyword "Flying"))]))

||| "Tap target nonland Forest."
public export
badForestNonland : Unspellable (Instruction []) (\ok =>
  SetStatus Tapped (Macros.target (And [HasSubtype (landType "Forest"), Not Macros.land] {cf = ok})))
badForestNonland Oh impossible

||| "creature with power 2 or less"
public export
okContradictionFreeAnd : Predicate [] Object
okContradictionFreeAnd =
  And [Macros.creature, Compare [StatAxis Power] AtMost (Lit 2)]

||| "noncreature that is attacking or blocking"
public export
badNoncreatureAttackingOrBlocking : Unspellable (Predicate [] Object) (\ok =>
  And [Not Macros.creature, Or [Attacking, Blocking]] {cf = ok})
badNoncreatureAttackingOrBlocking Oh impossible

||| "noncreature that is an attacking artifact or a blocking land"
public export
badWrappedStatusLaunder : Unspellable (Predicate [] Object) (\ok =>
  And [Or [And [Macros.artifact, Attacking], And [Macros.land, Blocking]], Not Macros.creature] {cf = ok})
badWrappedStatusLaunder Oh impossible

||| "blocked creature that's unblocked"
public export
okBlockedAndUnblocked : Predicate [] Object
okBlockedAndUnblocked = And [Macros.creature, Blocked, Macros.unblocked]

||| "between two and three target creatures"
public export
okAscendingRange : Noun [] Object
okAscendingRange = Described (TargetDet (Range (Just 2) (Just 3))) Macros.creature

||| "between three and two target creatures"
public export
badDescendingRange : Unspellable (Noun [] Object) (\ok =>
  Described (TargetDet (Range (Just 3) (Just 2))) Macros.creature {ok = (MaxAtLeastOne, ok, ObjectTgt)})
badDescendingRange Oh impossible

||| "attacking artifact or attacking land"
public export
okWholeZoneJoin : Predicate [] Object
okWholeZoneJoin =
  Or [And [Macros.artifact, Attacking], And [Macros.land, Attacking]]

||| "attacking artifact or land"
public export
badPartialZoneJoin : Unspellable (Predicate [] Object) (\ok =>
  Or [And [Macros.artifact, Attacking], Macros.land] {pd = ok})
badPartialZoneJoin Oh impossible

||| "creature you control or artifact you control"
public export
okDistinctStructuredDisjuncts : Predicate [] Object
okDistinctStructuredDisjuncts =
  Or [ And [Macros.creature, HasPossessor ControllerAx You]
     , And [Macros.artifact, HasPossessor ControllerAx You] ]

||| "creature you control or creature you control"
public export
okRepeatedStructuredDisjunct : Predicate [] Object
okRepeatedStructuredDisjunct =
  Or [And [Macros.creature, HasPossessor ControllerAx You], And [Macros.creature, HasPossessor ControllerAx You]]

||| "if this creature is attacking"
public export
okMatchesThisCreature : Condition []
okMatchesThisCreature = Matches Macros.thisCreature Attacking

||| "if target creature is an artifact"
public export
badMatchesTargetSubject : Unspellable (Condition []) (\ok =>
  Matches (Macros.target Macros.creature) Macros.artifact {bl = ok})
badMatchesTargetSubject Oh impossible

||| "Tap target creature. You gain 1 life if it's an artifact."
public export
okMatchesArtifact : Instruction []
okMatchesArtifact =
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                OnlyIf (Macros.gainsLife You (Lit 1))
                       (Matches (Macros.It OneOf) Macros.artifact) Nothing]

||| "Tap target creature. You gain 1 life if it's."
public export
badMatchesNothing : Unspellable (Instruction []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                OnlyIf (Macros.gainsLife You (Lit 1)) (Matches ((Macros.It OneOf)) (And []) {sy = ok}) Nothing])
badMatchesNothing Oh impossible

||| "if the number of artifacts you control is 4 or greater"
public export
okCompareCountSubject : Condition []
okCompareCountSubject =
  CompareAmt (Macros.countOf (And [Macros.artifact,
                                   HasPossessor ControllerAx You]))
             AtLeast (Lit 4)

||| "if 3 is 4 or greater"
public export
badCompareLiteralSubject : Unspellable (Condition []) (\ok =>
  CompareAmt (Lit 3) AtLeast (Lit 4) {rd = ok})
badCompareLiteralSubject Oh impossible

||| "each of up to two target creatures"
public export
okEachOfTargetGroup : Noun [] Object
okEachOfTargetGroup = EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature)

||| "each of each creature"
public export
badEachOfDistributive : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.each Macros.creature) {gm = ok})
badEachOfDistributive Oh impossible

||| "each of all creatures"
public export
badEachOfAll : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.allOf Macros.creature) {gm = ok})
badEachOfAll Oh impossible

||| "each of each of up to two target creatures"
public export
badNestedEachOf : Unspellable (Noun [] Object) (\ok =>
  EachOf (EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature)) {gm = ok})
badNestedEachOf Oh impossible

||| "target white creature"
public export
okWhiteCreature : Noun [] Object
okWhiteCreature = Macros.target (And [Macros.creature, ColorIs White])

||| "target colorless white creature"
public export
badColorlessWhite : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, IsColorless, ColorIs White] {cf = ok}))
badColorlessWhite Oh impossible

||| "Draw X cards, where X is the number of creatures you control."
public export
okSingleXRider : Instruction []
okSingleXRider =
  Sequentially [ Draw You (LetterVal X)
               , Define X (Macros.countOf Macros.creatureYouControl) ]

public export
badDoubleXRider : Unspellable (Instruction []) (\ok =>
  Sequentially [ Draw You (LetterVal X)
               , Define X (Macros.countOf Macros.creatureYouControl)
               , Define X (Macros.countOf Macros.creature) {ok} ])
badDoubleXRider Oh impossible

||| "Draw Y cards, where X is the number of creatures you control."
public export
badUnlicensedY : Unspellable (Instruction []) (\ok =>
  Sequentially [ Draw You (LetterVal Y)
               , Define X (Macros.countOf Macros.creatureYouControl) {ok} ])
badUnlicensedY Oh impossible

||| "the power of target land creature"
||| -- an animated land is a land AND a creature, and a creature has power, so
||| one head-type alternative carrying `Creature` is enough.
public export
okAnimatedLandPower : Amount []
okAnimatedLandPower = StatOf Power (Macros.target (And [Macros.land, Macros.creature]))

||| "the power of target land"
||| -- a noncreature permanent has no power [CR#208.3]; the animated land is
||| written as a creature (`okAnimatedLandPower`), not as a bare land.
public export
badLandPower : Unspellable (Amount []) (\ok =>
  StatOf Power (Macros.target Macros.land) {ty = ok})
badLandPower Oh impossible

||| "the loyalty of target battle"
||| -- loyalty is printed on planeswalkers [CR#209.1]; a battle has defense
||| instead [CR#210.1].
public export
badBattleLoyalty : Unspellable (Amount []) (\ok =>
  StatOf Loyalty (Macros.target (HasType Battle)) {ty = ok})
badBattleLoyalty Oh impossible

||| "the greatest power among creatures"
public export
okPowerAmongObjects : Amount []
okPowerAmongObjects = Macros.aggregate MaxOf (StatAxis Power) Macros.creature

||| "the greatest power among players"
public export
badPowerAmongPlayers : Unspellable (Amount []) (\ok =>
  Macros.aggregate MaxOf (StatAxis Power) AnyPlayer {sc = ok})
badPowerAmongPlayers Refl impossible

||| "the highest life total among creatures you control"
public export
badLifeTotalAmongObjects : Unspellable (Amount []) (\ok =>
  Macros.aggregate MaxOf (PlayerStatAxis LifeTotal) Macros.creatureYouControl {sc = ok})
badLifeTotalAmongObjects Refl impossible

||| "the creature with the greatest power"
public export
okExtremalSelection : Predicate [] Object
okExtremalSelection = Superlative MaxOf (StatAxis Power) Macros.creature

||| "the creature with the total power among creatures you control"
public export
badSumSelection : Unspellable (Predicate [] Object) (\ok =>
  Superlative SumOf (StatAxis Power) Macros.creature {ex = ok})
badSumSelection Oh impossible

||| "the creature with the least toughness among creatures you control"
public export
okDefiniteSuperlative : Noun [] Object
okDefiniteSuperlative =
  Macros.the (And [Macros.creature,
                   Superlative MinOf (StatAxis Toughness)
                               Macros.creatureYouControl])

||| "the creature"
public export
badBareDefinite : Unspellable (Noun [] Object) (\ok =>
  Macros.the Macros.creature {ok})
badBareDefinite Oh impossible

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
okObjectDesignation = HasDesignation Goaded Nothing

||| "target creature that is the monarch"
public export
badObjectMonarch : Unspellable (Predicate [] Object) (\ok =>
  HasDesignation Monarch Nothing {sc = ok})
badObjectMonarch Refl impossible

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

||| "Destroy target creature. Its controller loses life equal to its power."
public export
okItAfterAntecedent : Instruction []
okItAfterAntecedent =
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , Macros.losesLife (Macros.controllerOf (Macros.It OneOf))
                                  (StatOf Power (Macros.It OneOf)) ]

public export
badOtherwiseReadsLeadingArm : Unspellable (Instruction []) (\ok =>
  If (Macros.exists Macros.creatureYouControl)
     (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [creatureType "Zombie"]))
     (Just (SetStatus Tapped ((Macros.It OneOf) {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsLeadingArm (Refl, _) impossible

public export
badLeadingConditionAntecedent : Unspellable (Instruction []) (\ok =>
  Sequentially [If (CompareAmt (PlayerStatOf LifeTotal You) Less
                               (PlayerStatOf LifeTotal Macros.anOpponent))
                   (Macros.gainsLife You (Lit 6))
                   Nothing,
                Macros.losesLife (Macros.That PlayerW OneOf {ok}) (Lit 1)])
badLeadingConditionAntecedent Refl impossible

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

||| "Roll two d6. If you rolled 7, sacrifice this creature."
public export
okTotalAfterRoll : Instruction []
okTotalAfterRoll =
  Sequentially [ (Macros.rollDice You 2 6)
               , Macros.ifThen (CompareAmt (TheOutcome RollResult) Eq (Lit 7))
                               (Macros.sacrifice You Macros.thisCreature) ]

||| "If you rolled 7, sacrifice this creature."
public export
badTotalWithoutRoll : Unspellable (Instruction []) (\ok =>
  Macros.ifThen (CompareAmt (TheOutcome RollResult {ok}) Eq (Lit 7))
                (Macros.sacrifice You Macros.thisCreature))
badTotalWithoutRoll Refl impossible

public export
afterTopLook : Bindings
afterTopLook =
  instrIntro (the (Instruction []) (Macros.lookAt (Macros.topSlice (Lit 1))))

||| "Look at the top card of your library. Put that card into your graveyard."
public export
okReadsLookedAtLibraryCard : Noun ProofsDescription.afterTopLook Object
okReadsLookedAtLibraryCard = Macros.That CardW OneOf

public export
afterShuffledIntoLook : Bindings
afterShuffledIntoLook =
  instrIntro (the (Instruction [])
    (Sequentially [ Macros.lookAt (Macros.topSlice (Lit 1)), Macros.shuffleInto You This ]))

public export
badReadsShuffledIntoLibraryCard :
  Unspellable (Noun ProofsDescription.afterShuffledIntoLook Object) (\ok => Macros.That CardW OneOf {ok})
badReadsShuffledIntoLibraryCard Refl impossible

||| "if you control four or more creatures"
public export
okCompareCreatureCountSubject : Condition []
okCompareCreatureCountSubject =
  CompareAmt (Macros.countOf Macros.creatureYouControl) AtLeast (Lit 4)

||| "if up to three is 4 or greater"
public export
badCompareCeilingSubject : Unspellable (Condition []) (\ok =>
  CompareAmt (UpTo (Lit 3)) AtLeast (Lit 4) {rd = ok})
badCompareCeilingSubject Oh impossible

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

||| "if this creature's kicker cost was paid"
public export
okPaidCostOnKeywordWithACost : Amount []
okPaidCostOnKeywordWithACost =
  Paid (PaidCostReadback (ByKeyword "Kicker") Nothing) This

||| "if this creature's flying cost was paid"
public export
badPaidCostOnCostlessKeyword : Unspellable (Amount []) (\ok =>
  Paid (PaidCostReadback (ByKeyword "Flying") Nothing) This {nf = ok})
badPaidCostOnCostlessKeyword Oh impossible

||| "for each time it was kickre'd"
public export
badTimesPaidUnknownKeyword : Unspellable (Amount []) (\ok =>
  Paid (TimesPaid (ByKeyword "Kickre")) Macros.thisCreature {nf = ok})
badTimesPaidUnknownKeyword Oh impossible

||| "for each color of mana spent to cast this spell"
public export
okColorsSpentOnThis : Amount []
okColorsSpentOnThis = Macros.colorsSpentToCast This

||| "for each color of mana spent to cast a creature on the battlefield"
public export
badPaidReadOffStack : Unspellable (Amount []) (\ok =>
  Paid ColorsSpent (Macros.a Macros.creature) {sb = ok})
badPaidReadOffStack PaymentHappened impossible

||| "if colored mana was spent to cast it" is a read of Paid ColorsSpent
public export
coloredManaSpentIsPaidColorsSpent :
  Macros.coloredManaSpentToCast (This {bs = []}) =
    CompareAmt (Paid ColorsSpent This) AtLeast (Lit 1)
coloredManaSpentIsPaidColorsSpent = Refl

||| "spell that targets this creature"
public export
okSpellTargeter : Predicate [] Object
okSpellTargeter = Targets Macros.thisCreature SomeTarget

||| "player that targets this creature"
public export
badPlayerTargeter : Unspellable (Predicate [] Player) (\ok =>
  Targets Macros.thisCreature SomeTarget {tr = ok})
badPlayerTargeter SpellTargets impossible
badPlayerTargeter (EitherTargets _ _) impossible

||| "a creature with power or toughness 2 or greater"
public export
okSingleScopeAxisComparison : Predicate [] Object
okSingleScopeAxisComparison =
  Compare [StatAxis Power, StatAxis Toughness] AtLeast (Lit 2)

||| "a creature with power or life total 3 or greater"
public export
badMixedAxisComparison : Unspellable (Predicate [] Object) (\ok =>
  Compare [StatAxis Power, PlayerStatAxis LifeTotal] Greater (Lit 1) {at = ok})
badMixedAxisComparison (NextAxis _ (LastAxis _)) impossible

||| "a player with 13 or less life"
public export
okPlayerReadInPlayerDomain : Predicate [] Player
okPlayerReadInPlayerDomain =
  And [AnyPlayer, Compare [PlayerStatAxis LifeTotal] AtMost (Lit 13)]

||| "an object with 13 or less life"
public export
badPlayerReadInObjectDomain : Unspellable (Predicate [] Object) (\ok =>
  Compare [PlayerStatAxis LifeTotal] AtMost (Lit 13) {at = ok})
badPlayerReadInObjectDomain (LastAxis _) impossible

||| "target card on the stack"
public export
okCardAndSpell : Predicate [] Object
okCardAndSpell = Macros.cardOnTheStack

||| "each token on the battlefield"
public export
okTokenAndPermanent : Predicate [] Object
okTokenAndPermanent = Macros.tokenOnTheBattlefield

||| "a card token"
public export
badCardToken : Unspellable (Predicate [] Object) (\ok =>
  And [IsCard, IsToken] {cf = ok})
badCardToken Oh impossible

||| "an emblem permanent"
public export
badEmblemPermanent : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.emblem, Permanent] {cf = ok})
badEmblemPermanent Oh impossible

||| "a card that is a copy of a card"
public export
badCardCopyOfACard : Unspellable (Predicate [] Object) (\ok =>
  And [IsCard, Macros.copyOfACard] {cf = ok})
badCardCopyOfACard Oh impossible

||| "Target creature can't attack this turn."
public export
okCantAttackCreature : Instruction []
okCantAttackCreature =
  Macros.cantAttack (Macros.target Macros.creature) (Just ThisTurn)

||| "Target land can't attack this turn."
public export
badCantAttackLand : Unspellable (Instruction []) (\ok =>
  Macros.cantAttack (Macros.target Macros.land) (Just ThisTurn) {dp = ok})
badCantAttackLand Oh impossible

||| "Target creature can't block this turn."
public export
okCantBlockCreature : Instruction []
okCantBlockCreature =
  Macros.cantBlock (Macros.target Macros.creature) (Just ThisTurn)

||| "Target creature or land can't block this turn."
public export
badCantDisjunctSubject : Unspellable (Instruction []) (\ok =>
  Macros.cantBlock (Macros.target (Or [Macros.creature, Macros.land])) (Just ThisTurn) {dp = ok})
badCantDisjunctSubject Oh impossible

||| "your party": one each of Cleric, Rogue, Warrior and Wizard [CR#700.8].
public export
okPartyOfFourRoles : Noun [] Object
okPartyOfFourRoles = Macros.party

||| "one each of Cleric and Cleric": a repeated role counts one creature twice
||| [CR#700.8b]. Four independent "a Cleric you control" finds spell that
||| double count instead, and are a different sentence.
public export
badRepeatedPartyRole : Unspellable (Noun [] Object) (\ok =>
  OneEachOf [HasSubtype (creatureType "Cleric"), HasSubtype (creatureType "Cleric")]
            (Macros.allOf Macros.creatureYouControl) {rk = ok})
badRepeatedPartyRole (RolesAre {di = Oh}) impossible

||| "one each of nothing": a one-each group is written from at least one role
||| [CR#700.8].
public export
badEmptyPartyRoles : Unspellable (Noun [] Object) (\ok =>
  OneEachOf [] (Macros.allOf Macros.creatureYouControl) {rk = ok})
badEmptyPartyRoles RolesAre impossible

||| "a fortified land"
public export
okFortifiedLand : Noun [] Object
okFortifiedLand = Macros.a (And [Macros.land, IsAttached Fortified])

||| "a fortified creature": a Fortification can't legally be attached to
||| anything that isn't a land [CR#301.6]; "equipped creature" is the word for
||| an attachment on a creature [CR#301.5].
public export
badFortifiedCreature : Unspellable (Noun [] Object) (\ok =>
  Macros.a (And [Macros.creature, IsAttached Fortified] {cf = ok}))
badFortifiedCreature Oh impossible
