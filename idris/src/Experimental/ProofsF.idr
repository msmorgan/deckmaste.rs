module Experimental.ProofsF

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Prevent all damage that would be dealt to any player or permanent."
public export
okPreventDealtToPermanent : StaticEffect []
okPreventDealtToPermanent =
  DamageRule AnyDamage Unattributed
             (ToRecipient (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
             (Prevent CutAll Nothing) Repeatedly


||| "Prevent all damage that would be dealt to this this turn."
public export
badPreventedBareThis : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient This {rk = ok})
             (Prevent CutAll Nothing) Repeatedly)
badPreventedBareThis ObjectTakes impossible


||| "Prevent all damage that would be dealt to target artifact this turn."
public export
badPreventDealtToArtifact : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed
             (ToRecipient (Macros.target Macros.artifact) {rk = ok})
             (Prevent CutAll Nothing) Repeatedly)
badPreventDealtToArtifact ObjectTakes impossible


||| "… is dealt to target attacking creature instead."
public export
okRedirectToSingleCreature : StaticEffect []
okRedirectToSingleCreature =
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Redirect CutAll (Macros.target (And [Macros.creature, Attacking])))
             Repeatedly


||| "All damage that would be dealt to you is dealt to target artifact instead."
public export
badRedirectToArtifact : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Redirect CutAll (Macros.target Macros.artifact) {rk = ok}) Repeatedly)
badRedirectToArtifact ObjectTakes impossible


public export
badRedirectToPlural : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Redirect CutAll (Macros.allOf Macros.creatureYouControl) {one = ok}) Repeatedly)
badRedirectToPlural Refl impossible


||| "This deals 4 damage to target creature. The damage can't be prevented."
public export
okTheDamageAfterDealing : Effect []
okTheDamageAfterDealing =
  Sequentially [ DealDamage This (Lit 4) (Macros.target Macros.creature)
               , Continuously {ts = StaticFirstDone}
                   (CantPrevent AnyDamage ThatDamage NoPreventionOnly) Nothing ]


||| "The damage can't be prevented."
public export
badTheDamageUnannounced : Unspellable (StaticEffect []) (\ok =>
  CantPrevent AnyDamage (ThatDamage {ok}) NoPreventionOnly)
badTheDamageUnannounced Oh impossible


||| "You gain 3 life. The damage can't be prevented."
public export
badTheDamageAfterLifeGain : Unspellable (Effect []) (\ok =>
  Sequentially [ ChangeLife You (Up (Lit 3))
               , Continuously {ts = StaticFirstDone}
                   (CantPrevent AnyDamage (ThatDamage {ok}) NoPreventionOnly) Nothing ])
badTheDamageAfterLifeGain Oh impossible


||| "… If damage from a red source is prevented this way, you gain 3 life."
public export
okPreventedFromSourceAnnounced : StaticEffect []
okPreventedFromSourceAnnounced =
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent CutAll
                (Just (If (PreventedFromSource (And [Macros.source, ColorIs Red]))
                          (Macros.gainsLife You (Lit 3)) Nothing)))
             Repeatedly


||| "If damage from a red source is prevented this way, you gain 3 life."
public export
badPreventedFromSourceUnannounced : Unspellable (Effect []) (\ok =>
  If (PreventedFromSource (And [Macros.source, ColorIs Red]) {ok})
     (Macros.gainsLife You (Lit 3)) Nothing)
badPreventedFromSourceUnannounced Refl impossible


||| "… You gain life equal to the damage prevented this way."
public export
okPreventedThisWayAnnounced : StaticEffect []
okPreventedThisWayAnnounced =
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent CutAll (Just (Macros.gainsLife You Macros.preventedThisWay)))
             Repeatedly


public export
badPreventedThisWayAfterDamage : Unspellable (Effect []) (\ok =>
  Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.creature)
               , ChangeLife You (Up (Macros.preventedThisWay {ok})) ])
badPreventedThisWayAfterDamage Refl impossible


||| "You gain life equal to the damage prevented this way."
public export
badPreventedThisWayUnannounced : Unspellable (Effect []) (\ok =>
  ChangeLife You (Up (Macros.preventedThisWay {ok})))
badPreventedThisWayUnannounced Refl impossible


||| "… They gain 2 life for each card less than two they drew this way."
public export
okShortOfCeilingAnnounced : Effect []
okShortOfCeilingAnnounced =
  Sequentially [ Macros.may (Macros.each AnyPlayer) (Draw They (UpTo (Lit 2)))
               , Macros.gainsLife They (Macros.times 2 Macros.shortOfCeiling) ]


||| "You gain 2 life for each card less than two you draw this way."
public export
badShortOfCeilingUnannounced : Unspellable (Effect []) (\ok =>
  Macros.gainsLife You (Macros.times 2 (Macros.shortOfCeiling {ok})))
badShortOfCeilingUnannounced Refl impossible


public export
badShieldSizedByItsOwnPrevention : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent (Shield (Macros.preventedThisWay {ok})) Nothing) Repeatedly)
badShieldSizedByItsOwnPrevention Refl impossible


||| "Prevent the next 3 damage that would be dealt to you this turn."
public export
okShieldRepeatedly : StaticEffect []
okShieldRepeatedly =
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent (Shield (Lit 3)) Nothing) Repeatedly


public export
badShieldNextTimeOnly : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Prevent (Shield (Lit 3)) Nothing) NextTimeOnly {su = ok})
badShieldNextTimeOnly Oh impossible


||| "Destroy target creature."
public export
okDestroyCreature : Effect []
okDestroyCreature = Macros.destroy (Macros.target Macros.creature)


||| "Destroy target source."
||| Refused for an unzoned noun, not for the verb; no sibling spells it.
public export
badDestroySource : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target Macros.source) {ok})
badDestroySource Oh impossible


||| "Target creature gets +1/+1 until end of turn."
public export
okGetsCreature : Effect []
okGetsCreature =
  Macros.gets (Macros.target Macros.creature) (PtUp (Lit 1)) (PtUp (Lit 1))
              (Just Macros.untilEndOfTurn)


||| "Target source gets +1/+1 until end of turn." [CR#609.7a]
public export
badGetsSource : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.source) (PtUp (Lit 1)) (PtUp (Lit 1)) {ok}
              (Just Macros.untilEndOfTurn))
badGetsSource Oh impossible


||| "This creature deals 3 damage to target creature."
public export
okDamageToCreature : Effect []
okDamageToCreature = DealDamage This (Lit 3) (Macros.target Macros.creature)


||| "This creature deals 3 damage to target source."
public export
badDamageToSource : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3) (Macros.target Macros.source) {rk = ok})
badDamageToSource ObjectTakes impossible


||| "…target nonartifact…"
public export
okNonartifact : Predicate [] Object
okNonartifact = Not Macros.artifact


||| "…target nonsource…"
public export
badNonsource : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.source {ng = ok})
badNonsource Oh impossible


public export
badRedirectToGroup : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (ToRecipient You)
             (Redirect CutAll
                (Macros.youAnd (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))
                {one = ok})
             Repeatedly)
badRedirectToGroup Refl impossible


||| "Target opponent loses 1 life. You gain that much life."
public export
okThatMuchAfterLifeLoss : Effect []
okThatMuchAfterLifeLoss =
  Sequentially [ Macros.losesLife (Macros.target Opponent) (Lit 1)
               , Macros.gainsLife You ThatMuch ]


||| "… it deals that much damage plus that much instead."
public export
badScaleShiftByThatMuch : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage (DealtBy (Macros.a Macros.source))
             (ToRecipient (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
             (Scale (Shifted ShiftUp (ThatMuch {ok}))) Repeatedly)
badScaleShiftByThatMuch Refl impossible


public export
badScaleToArtifact : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage (DealtBy (Macros.a Macros.source))
             (ToRecipient (Macros.target Macros.artifact) {rk = ok})
             (Scale (Multiplied Doubled)) Repeatedly)
badScaleToArtifact ObjectTakes impossible


public export
badPreventedThisWayAfterDamageEvent : Unspellable Ability (\ok =>
  Triggered Whenever (IsDealtDamage AnyDamage Macros.thisCreature) [] Nothing [] Nothing Nothing Nothing
            (DealDamage ((Macros.It OneOf)) (Macros.preventedThisWay {ok = ok}) (Macros.target Macros.anyTarget)))
badPreventedThisWayAfterDamageEvent Refl impossible


public export
badThatMuchAfterDeath : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing
            (DealDamage Macros.thisCreature (ThatMuch {ok = ok})
                        (Macros.target Macros.anyTarget)))
badThatMuchAfterDeath Refl impossible


||| "… That creature deals damage equal to its power to this creature."
public export
okThatCreatureAfterDamage : Effect []
okThatCreatureAfterDamage =
  Sequentially [ DealDamage Macros.thisCreature
                            (StatOf Power Macros.thisCreature)
                            (Macros.target Macros.creature)
               , DealDamage (Macros.That (TypeW Creature) OneOf)
                            (StatOf Power (Macros.It OneOf))
                            Macros.thisCreature ]


public export
badThatCreatureIsDamagedSelf : Unspellable Ability (\ok =>
  Triggered Whenever (IsDealtDamage AnyDamage Macros.thisCreature) [] Nothing [] Nothing Nothing Nothing
            (DealDamage (Macros.That (TypeW Creature) OneOf {ok = ok}) ThatMuch
                        (Macros.target Macros.anyTarget)))
badThatCreatureIsDamagedSelf Refl impossible


||| "Your opponents can't gain life."
public export
okStaticPlayerCant : Ability
okStaticPlayerCant =
  Static (Macros.playerCant "GainLife" (PlayerGroup YourOpponents))


||| "Target player can't gain life."
||| Refused as a Static; Continuously (playerCant …) spells the sentence.
public export
badStaticPlayerCantTargets : Unspellable Ability (\ok =>
  Static (Macros.playerCant "GainLife" (Macros.target AnyPlayer)) {ut = ok})
badStaticPlayerCantTargets Oh impossible


public export
badChosenBasicTypeOnCreature : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (Becomes (Macros.target Macros.creature) Sets (ChosenQuality (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly)))
                                  {ok = ok})
               (Just Macros.untilEndOfTurn))
badChosenBasicTypeOnCreature Oh impossible


||| "Creatures are Mountains."
public export
badCreaturesAreMountains : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.allOf Macros.creature) Sets (Bundle (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"]) [] Nothing) Nothing) {ok = ok})
badCreaturesAreMountains Oh impossible


||| "of the creature type of your choice"
public export
okYourChoiceCreatureType : Predicate [] Object
okYourChoiceCreatureType = OfYourChoice (SubtypeQ Creature) Nothing


||| "of the number of your choice"
public export
badYourChoiceNumber : Unspellable (Predicate [] Object) (\ok =>
  OfYourChoice Number Nothing {read = ok})
badYourChoiceNumber Oh impossible


public export
badReaderBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (Gets Adds (Macros.allOf (And [Macros.creature,
                                   OfChosen (SubtypeQ Creature) {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly) ]
       Nothing)
badReaderBeforeChooser Refl impossible


public export
badTwoChoosersOneSortRead : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly)
       , Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly)
       , Static (Gets Adds (Macros.allOf (And [Macros.creature,
                                   OfChosen (SubtypeQ Creature) {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing)
badTwoChoosersOneSortRead Refl impossible


public export
badChosenReadWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing Openly)
       , Static (Gets Adds (Macros.allOf (And [Macros.creature,
                                   OfChosen (SubtypeQ Creature) {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing)
badChosenReadWrongSort Refl impossible


public export
badChosenProtectionBeforeChoice : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [creatureType "Angel"] [Creature])
       [ Static (Gains Macros.thisCreature
                       (KeywordAbility "Protection" (Just (ParamQuality (OfChosen Color {ok = ok}))) Nothing))
       , Static (EntersChoice Macros.thisCreature (QSort Color) Nothing Openly) ]
       (Just (2, 2)))
badChosenProtectionBeforeChoice Refl impossible


public export
badAscribedQualityBeforeChoice : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (ChosenQuality (OfChosen (SubtypeQ Creature) {ok = ok})))
       , Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly) ]
       Nothing)
badAscribedQualityBeforeChoice Refl impossible


||| "Creatures you control are every creature type."
public export
okSingleExtension : StaticEffect []
okSingleExtension =
  AlsoOffBattlefield
    (Becomes (Macros.allOf Macros.creatureYouControl) Adds
             (EveryTypeOf CreatureSpace))


public export
badDoubleExtension : Unspellable (StaticEffect []) (\ok =>
  AlsoOffBattlefield
    (AlsoOffBattlefield
       (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (Bundle (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing) Nothing))) {nx = ok})
badDoubleExtension Oh impossible


||| "… Counter target spell with the chosen name."
public export
okNameMatchAfterChooser : Card
okNameMatchAfterChooser =
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort CardName) Nothing
                              Openly)
       , Activated (Mana [Macros.pip Blue])
                   (CounterSpell
                      (Macros.target (And [Macros.spell, Named ChosenName])))
                   Nothing Nothing Nothing Nothing ]
       Nothing


public export
badNameMatchBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Activated (Mana [Macros.pip Blue])
                   (CounterSpell
                      (Macros.target (And [Macros.spell,
                                           Named (ChosenName {ok = ok})]))) Nothing Nothing Nothing Nothing
       , Static (EntersChoice Macros.thisEnchantment (QSort CardName) Nothing Openly) ]
       Nothing)
badNameMatchBeforeChooser Refl impossible


public export
badNameMatchWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing Openly)
       , Activated (Mana [Macros.pip Blue])
                   (CounterSpell
                      (Macros.target (And [Macros.spell,
                                           Named (ChosenName {ok = ok})]))) Nothing Nothing Nothing Nothing ]
       Nothing)
badNameMatchWrongSort Refl impossible


||| "Choose a creature type other than Wall."
public export
okCreatureTypeExclusion : ChoiceDomain (QSort (SubtypeQ Creature))
okCreatureTypeExclusion = TypeOtherThan (creatureType "Wall")


||| "Choose a creature type other than Equipment."
public export
badNonCreatureTypeExclusion : Unspellable (ChoiceDomain (QSort (SubtypeQ Creature))) (\ok =>
  TypeOtherThan (artifactType "Equipment") {ct = ok})
badNonCreatureTypeExclusion Refl impossible


||| "… two cards with the same name in your hand …"
public export
okPluralNameAgreement : Noun [] Object
okPluralNameAgreement =
  NamesAgree SameName (Macros.counted (Macros.exactly 2)
                                      (And [Not Macros.land, InZone Macros.handZ]))


||| "… target creature with different names."
public export
badSingularNameAgreement : Unspellable (Noun [] Object) (\ok =>
  NamesAgree DifferentNames (Macros.target Macros.creature) {pl = ok})
badSingularNameAgreement Refl impossible


||| "This spell can't be countered."
public export
okCantBeCountered : StaticEffect []
okCantBeCountered = Macros.objectCant "Counter" This


||| "Creature cards in graveyards can't be countered."
public export
badCounteredInGraveyard : Unspellable (StaticEffect []) (\ok =>
  Macros.objectCant "Counter"
    (Macros.allOf (And [Macros.creature, InZone Macros.graveyardZ])) {dp = ok})
badCounteredInGraveyard Oh impossible


||| "Saga — I, II, III — Draw a card."
public export
okChapterOnSaga : Card
okChapterOnSaga =
  Macros.card "" (Just [Macros.pip White]) []
       (MkTypeLine [enchantmentType "Saga"] [Enchantment])
       [ Macros.triggered When (ChapterMark [ChapterI])
           (Draw You (Lit 1))
       , Macros.triggered When (ChapterMark [ChapterII])
           (Draw You (Lit 1))
       , Macros.triggered When (ChapterMark [ChapterIII])
           (Draw You (Lit 1)) ]
       Nothing


public export
badChapterOnNonSaga : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Enchantment])
       [ Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)) ]
       Nothing {fl = ok})
badChapterOnNonSaga MkFaceLaws impossible


||| "I — Draw a card."
public export
okChapterMark : Ability
okChapterMark =
  Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing Nothing Nothing
    (Draw You (Lit 1))


||| "— Draw a card."
public export
badEmptyChapterMark : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [] {cm = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badEmptyChapterMark Oh impossible


||| "II, II — Draw a card."
public export
badRepeatedChapterMark : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterII, ChapterII] {cm = ok}) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badRepeatedChapterMark Oh impossible


||| "Whenever you draw a card, draw a card. This triggers only once each turn."
public export
okTriggerLimitOffChapter : Ability
okTriggerLimitOffChapter =
  Triggered Whenever (Draws You) [] Nothing [] Nothing (Just OncePerTurn) Nothing
    (Draw You (Lit 1))


||| "I — Draw a card. This ability triggers only once each turn."
public export
badChapterLimit : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing (Just OncePerTurn) Nothing
    (Draw You (Lit 1)) {cd = ok})
badChapterLimit Oh impossible


||| "I — , if you control a creature, draw a card."
public export
badChapterIntervening : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing Nothing (Just (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You])))
    (Draw You (Lit 1)) {cd = ok})
badChapterIntervening Oh impossible


||| "{T}: Draw a card. Activate only once each turn."
public export
okActivatedTurnLimit : Ability
okActivatedTurnLimit =
  Activated TapSymbol (Draw You (Lit 1)) Nothing (Just OncePerTurn) Nothing
            Nothing


||| "{T}: Draw a card. Do this only once each turn."
public export
badActionLimitOnActivated : Unspellable Ability (\ok =>
  Activated TapSymbol (Draw You (Lit 1)) Nothing (Just ActionOncePerTurn) Nothing
            Nothing {ul = ok})
badActionLimitOnActivated Oh impossible


||| "I — while you control a creature, draw a card."
public export
badChapterWhile : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) []
    (Just (WhileTrue (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You]))))
    [] Nothing Nothing Nothing
    (Draw You (Lit 1)) {cd = ok})
badChapterWhile Oh impossible


||| "I — and whenever you draw a card, draw a card."
public export
badChapterJoin : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) [] Nothing
    [ Macros.joinedHead Whenever (Draws You) ]
    Nothing Nothing Nothing
    (Draw You (Lit 1)) {cd = ok})
badChapterJoin Oh impossible


||| "If you would draw a card, draw two cards instead."
public export
okDrawReplacement : StaticEffect []
okDrawReplacement =
  Intercepts (Draws You) [] Nothing (Draw You (Lit 2)) Repeatedly Nothing


||| "If I — would happen, draw a card instead."
public export
badChapterReplacement : Unspellable (StaticEffect []) (\ok =>
  Intercepts (ChapterMark [ChapterI]) [] Nothing (Draw You (Lit 1)) Repeatedly Nothing {ok})
badChapterReplacement Oh impossible


||| "You may look at the top card of your library any time."
public export
okLookAtTopOfLibrary : StaticEffect []
okLookAtTopOfLibrary = Visibility LookAt You TopOfLibrary


||| "You may look at your hand any time."
public export
badLookAtHandRider : Unspellable (StaticEffect []) (\ok =>
  Visibility LookAt You WholeHand {vo = ok})
badLookAtHandRider Oh impossible


||| "Lands you control are every creature type."
public export
badEveryCreatureTypeOnLand : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.allOf (And [Macros.land, HasPossessor ControllerAx You])) Adds (EveryTypeOf CreatureSpace)
                {ok = ok})
badEveryCreatureTypeOnLand Oh impossible


||| "Creatures you control are every basic land type."
public export
badEveryBasicLandTypeOnCreature : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (EveryTypeOf BasicLandSpace)
                {ok = ok})
badEveryBasicLandTypeOnCreature Oh impossible


||| "Target land becomes every basic land type until end of turn."
public export
setsEveryBasicLandType : Effect []
setsEveryBasicLandType =
  Continuously {ts = StaticFirstDone} (Becomes (Macros.target Macros.land) Sets (EveryTypeOf BasicLandSpace))
               (Just Macros.untilEndOfTurn)


||| "Target creature loses the creature type of your choice until end of turn."
public export
losesChosenCreatureType : Effect []
losesChosenCreatureType =
  Continuously {ts = StaticFirstDone} (Becomes (Macros.target Macros.creature) Loses
                        (ChosenQuality (OfYourChoice (SubtypeQ Creature) Nothing)))
               (Just Macros.untilEndOfTurn)


||| "Target creature loses all colors until end of turn."
public export
losesAllColors : Effect []
losesAllColors =
  Continuously {ts = StaticFirstDone} (Becomes (Macros.target Macros.creature) Loses (Colored EveryColor))
               (Just Macros.untilEndOfTurn)


public export
badAddsNoColor : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.target Macros.creature) Adds (Colored (SomeColors [])) {ok = ok})
badAddsNoColor Oh impossible


||| "Target creature loses colorless until end of turn."
public export
badLosesNoColor : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.target Macros.creature) Loses (Colored (SomeColors [])) {ok = ok})
badLosesNoColor Oh impossible


||| "Equipped permanent isn't a 2/2 creature."
public export
badLosesPt : Unspellable (StaticEffect []) (\ok =>
  Becomes (AttachHost Equipped PermanentW) Loses
          (Bundle (MkToken (Just (Lit 2 ** Lit 2)) [] (Macros.typesOnly [Creature]) [] Nothing)
                  Nothing) {ok = ok})
badLosesPt Oh impossible


public export
badStillOnAddition : Unspellable (StaticEffect []) (\ok =>
  Becomes (Macros.target Macros.creature) Adds
          (Bundle (MkToken Nothing [] (Macros.typesOnly [Artifact]) [] Nothing)
                  (Just Creature)) {ok = ok})
badStillOnAddition Oh impossible


||| "… At the beginning of that turn's end step, you lose the game."
public export
okDeicticTurnAfterExtraTurn : Effect []
okDeicticTurnAfterExtraTurn =
  Sequentially [ExtraTurn You (Lit 1),
                Delayed (BeginningOf ThePart EndStep Macros.thatTurns) [] Nothing
                        (Concludes LoseGame You)]


||| "Draw a card. At the beginning of that turn's end step, you lose the game."
public export
badDeicticTurnWithoutIntroducer : Unspellable (Effect []) (\ok =>
  Sequentially [Draw You (Lit 1),
                Delayed (BeginningOf ThePart EndStep (Macros.thatTurns {ok})) [] Nothing
                        (Concludes LoseGame You)])
badDeicticTurnWithoutIntroducer Refl impossible


||| "After this combat phase, there is an additional upkeep step."
public export
okAdditionalUpkeep : Effect []
okAdditionalUpkeep = AdditionalPart Nothing Upkeep (Just Combat) (Lit 1) Nothing


||| "After this combat phase, there is an additional turn."
public export
badAdditionalTurn : Unspellable (Effect []) (\ok =>
  AdditionalPart Nothing Turn (Just Combat) (Lit 1) Nothing {ad = ok})
badAdditionalTurn Oh impossible


||| "Spells with the chosen name can't be activated."
public export
badActivatedSpellClass : Unspellable
  (StaticEffect [MkBinding AD (Quality CardName) OneOf QualityP]) (\ok =>
  Macros.objectCant "Activate"
    (Macros.allOf (And [Macros.spell, Named ChosenName])) {dp = ok})
badActivatedSpellClass Oh impossible


||| "Activated abilities of artifacts can't be Nothing cast."
public export
badCastAbilityClass : Unspellable (StaticEffect []) (\ok =>
  Macros.objectCant "Cast"
    (Macros.allOf (And [AbilityHead AnyActivated, AbilityOf (Macros.allOf Macros.artifact)]))
    {dp = ok})
badCastAbilityClass Oh impossible


||| "You may sacrifice a Mountain rather than pay this spell's mana cost."
public export
okAltCostSacrifice : StaticEffect []
okAltCostSacrifice =
  AltCost This (Just (Do (Macros.sacrifice You
                 (Macros.a (And [Macros.land, HasSubtype (landType "Mountain")])))))


||| "You may {T} rather than pay this spell's mana cost."
public export
badAltCostTapSymbol : Unspellable (StaticEffect []) (\ok =>
  AltCost This (Just TapSymbol) {ap = ok})
badAltCostTapSymbol (Present {ok = Oh}) impossible


||| "You may [+1] rather than pay this spell's mana cost."
public export
badAltCostLoyaltySymbol : Unspellable (StaticEffect []) (\ok =>
  AltCost This (Just (LoyaltySymbol (LoyaltyUp 1))) {ap = ok})
badAltCostLoyaltySymbol (Present {ok = Oh}) impossible


||| "Escalate {2}"
public export
badEscalateWithoutModes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [Macros.keywordCosting "Escalate" (Mana [Macros.generic 2])] Nothing {fl = ok})
badEscalateWithoutModes MkFaceLaws impossible


||| "Entwine {2}"
public export
badEntwineWithoutModes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
       [Macros.keywordCosting "Entwine" (Mana [Macros.generic 2])] Nothing {fl = ok})
badEntwineWithoutModes MkFaceLaws impossible


||| "… rather than pay this spell's mana cost, pay {2}."
public export
okPlayPaymentMana : PlayPayment []
okPlayPaymentMana = PayingInstead (Mana [Macros.generic 2])


public export
badPlayPaymentTapSymbol : Unspellable (PlayPayment []) (\ok =>
  PayingInstead TapSymbol {ok})
badPlayPaymentTapSymbol Oh impossible


||| "As an additional cost to cast this spell, sacrifice an artifact."
public export
okAddedCostSacrifice : StaticEffect []
okAddedCostSacrifice =
  AddedCost (Do (Macros.sacrifice You (Macros.a Macros.artifact))) False


||| "As an additional cost to cast this spell, {T}."
public export
badAddedCostTapSymbol : Unspellable (StaticEffect []) (\ok =>
  AddedCost TapSymbol False {ap = ok})
badAddedCostTapSymbol AddedPaymentWritten impossible


||| "As an additional cost to cast this spell, [+1]."
public export
badAddedCostLoyaltySymbol : Unspellable (StaticEffect []) (\ok =>
  AddedCost (LoyaltySymbol (LoyaltyUp 1)) False {ap = ok})
badAddedCostLoyaltySymbol AddedPaymentWritten impossible


||| "You may sacrifice a Mountain rather than pay this spell's mana cost"
public export
badAltCostClause : Unspellable (Effect []) (\ok =>
  Continuously {ts = StaticFirstDone} (AltCost This (Just (Do (Macros.sacrifice You
                  (Macros.a (And [Macros.land, HasSubtype (landType "Mountain")]))))))
               Nothing {cl = ok})
badAltCostClause Oh impossible


||| "Regenerate target creature."
public export
okRegenerateCreature : Effect []
okRegenerateCreature = Regenerate (Macros.target Macros.creature)


||| "Regenerate target creature card in your graveyard."
public export
badRegenerateInGraveyard : Unspellable (Effect []) (\ok =>
  Regenerate (Macros.a (And [Macros.creature, InZone Macros.graveyardZ]))
             {zn = ok})
badRegenerateInGraveyard Oh impossible


public export
badRegenerateBareThis : Unspellable (Effect []) (\ok =>
  Regenerate This {zn = ok})
badRegenerateBareThis Oh impossible


||| "Creature cards in your graveyard can't be regenerated."
public export
badRegeneratedInGraveyard : Unspellable (StaticEffect []) (\ok =>
  Macros.objectCant "Regenerate"
                    (Macros.a (And [Macros.creature, InZone Macros.graveyardZ]))
                    {dp = ok})
badRegeneratedInGraveyard Oh impossible


||| "…choose a color. {T}: Add one mana of the chosen color."
public export
okChosenColorAfterChooser : Card
okChosenColorAfterChooser =
  Macros.card "" Nothing [] (MkTypeLine [] [Land])
       [ Static (EntersChoice Macros.thisLand (QSort Color) Nothing Openly)
       , Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing) []) Nothing Nothing Nothing
           Nothing ]
       Nothing


||| "{T}: Add one mana of the chosen color."
public export
badChosenColorNoChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing {cq = ok}) []) Nothing Nothing Nothing Nothing ]
       Nothing)
badChosenColorNoChooser Refl impossible


||| "Prevent all damage sources of the last chosen color would deal to you."
public export
okLastChosenAfterChooser : Card
okLastChosenAfterChooser =
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing
                              Openly)
       , Static (DamageRule AnyDamage
                   (DealtBy (Macros.allOf (And [Macros.source,
                                                OfTheLastChosen Color])))
                   (ToRecipient You) (Prevent CutAll Nothing) Repeatedly) ]
       Nothing


||| "sources of the last chosen color"
public export
badLastChosenColorNoChooser : Unspellable (Predicate [] Object) (\ok =>
  OfTheLastChosen Color {ok = ok})
badLastChosenColorNoChooser Oh impossible


public export
badLastChosenBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.allOf (And [Macros.source,
                                                OfTheLastChosen Color {ok = ok}]))) (ToRecipient You) (Prevent CutAll Nothing) Repeatedly)
       , Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing Openly) ]
       Nothing)
badLastChosenBeforeChooser Oh impossible


public export
badLastChosenWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly)
       , Static (DamageRule AnyDamage (DealtBy (Macros.allOf (And [Macros.source,
                                                OfTheLastChosen Color {ok = ok}]))) (ToRecipient You) (Prevent CutAll Nothing) Repeatedly) ]
       Nothing)
badLastChosenWrongSort Oh impossible


||| "Starting with you, each player votes for death or torture."
public export
okDistinctBallotOptions : Ballot []
okDistinctBallotOptions = ByLabel ["death", "torture"]


||| "Starting with you, each player votes for death or death."
public export
badRepeatedBallotOption : Unspellable (Ballot []) (\ok =>
  ByLabel ["death", "death"] {ok})
badRepeatedBallotOption Oh impossible


||| "Starting with you, each player votes for death."
public export
badSingletonBallot : Unspellable (Ballot []) (\ok =>
  ByLabel ["death"] {ok})
badSingletonBallot Oh impossible


public export
jointCrossAbilityChoice : Card
jointCrossAbilityChoice =
  SingleFaced
    (MkFace "Joint choice witness" Nothing [Color] []
      (MkTypeLine [creatureType "Shapeshifter"] [Creature])
      [ Static (Gains Macros.thisCreature
                 (Macros.keywordQuality "Protection" (OfChosen Color)))
      , Static (Macros.entersChoosing Macros.thisCreature Color)
      ]
      (Macros.printedBox (Just (1, 1))))
    {fl = MkFaceLaws}
