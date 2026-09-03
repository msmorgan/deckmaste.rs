module Experimental.ProofsF

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


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


||| "All damage that would be dealt to you is dealt to target artifact instead."
public export
badRedirectToArtifact : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (Macros.shieldingIt You)
             (Redirect CutAll (Macros.target Macros.artifact) {rk = ok}) Repeatedly)
badRedirectToArtifact ObjectTakes impossible


public export
badRedirectToPlural : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (Macros.shieldingIt You)
             (Redirect CutAll (Macros.allOf Macros.creatureYouControl) {one = ok}) Repeatedly)
badRedirectToPlural Refl impossible


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


||| "If damage from a red source is prevented this way, you gain 3 life."
public export
badPreventedFromSourceUnannounced : Unspellable (Effect []) (\ok =>
  If (PreventedFromSource (And [Macros.source, ColorIs Red]) {ok})
     (Macros.gainsLife You (Lit 3)) Nothing)
badPreventedFromSourceUnannounced Refl impossible


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


||| "You gain 2 life for each card less than two you draw this way."
public export
badShortOfCeilingUnannounced : Unspellable (Effect []) (\ok =>
  Macros.gainsLife You (Macros.times 2 (Macros.shortOfCeiling {ok})))
badShortOfCeilingUnannounced Refl impossible


public export
badShieldSizedByItsOwnPrevention : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (Macros.shieldingIt You)
             (Prevent (Shield (Macros.preventedThisWay {ok})) Nothing) Repeatedly)
badShieldSizedByItsOwnPrevention Refl impossible


public export
badShieldNextTimeOnly : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (Macros.shieldingIt You)
             (Prevent (Shield (Lit 3)) Nothing) NextTimeOnly {su = ok})
badShieldNextTimeOnly Oh impossible


||| "Destroy target source."
public export
badDestroySource : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target Macros.source) {ok})
badDestroySource Oh impossible


||| "Target source gets +1/+1 until end of turn." [CR#609.7a]
public export
badGetsSource : Unspellable (Effect []) (\ok =>
  Macros.gets (Macros.target Macros.source) (PtUp (Lit 1)) (PtUp (Lit 1)) {ok}
              (Just Macros.untilEndOfTurn))
badGetsSource Oh impossible


||| "This creature deals 3 damage to target source."
public export
badDamageToSource : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3) (Macros.target Macros.source) {rk = ok})
badDamageToSource ObjectTakes impossible


||| "…target nonsource…"
public export
badNonsource : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.source {ng = ok})
badNonsource Oh impossible


public export
badRedirectToGroup : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage Unattributed (Macros.shieldingIt You)
             (Redirect CutAll
                (Macros.youAnd (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))
                {one = ok})
             Repeatedly)
badRedirectToGroup Refl impossible


||| "… it deals that much damage plus that much instead."
public export
badScaleShiftByThatMuch : Unspellable (StaticEffect []) (\ok =>
  DamageRule AnyDamage (DealtBy (Macros.a Macros.source))
             (Macros.shieldingIt (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
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
            (DealDamage It (Macros.preventedThisWay {ok = ok}) (Macros.target Macros.anyTarget)))
badPreventedThisWayAfterDamageEvent Refl impossible


public export
badThatMuchAfterDeath : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing
            (DealDamage Macros.thisCreature (ThatMuch {ok = ok})
                        (Macros.target Macros.anyTarget)))
badThatMuchAfterDeath Refl impossible


public export
badThatCreatureIsDamagedSelf : Unspellable Ability (\ok =>
  Triggered Whenever (IsDealtDamage AnyDamage Macros.thisCreature) [] Nothing [] Nothing Nothing Nothing
            (DealDamage (That (TypeW Creature) {ok = ok}) ThatMuch
                        (Macros.target Macros.anyTarget)))
badThatCreatureIsDamagedSelf Refl impossible


||| "Target player can't gain life."
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


public export
badDoubleExtension : Unspellable (StaticEffect []) (\ok =>
  AlsoOffBattlefield
    (AlsoOffBattlefield
       (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (Bundle (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing) Nothing))) {nx = ok})
badDoubleExtension Oh impossible


public export
badNameMatchBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Activated (Mana [Macros.pip Blue])
                   (Macros.counterSpell
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
                   (Macros.counterSpell
                      (Macros.target (And [Macros.spell,
                                           Named (ChosenName {ok = ok})]))) Nothing Nothing Nothing Nothing ]
       Nothing)
badNameMatchWrongSort Refl impossible


||| "Choose a creature type other than Equipment."
public export
badNonCreatureTypeExclusion : Unspellable (ChoiceDomain (QSort (SubtypeQ Creature))) (\ok =>
  TypeOtherThan (artifactType "Equipment") {ct = ok})
badNonCreatureTypeExclusion Refl impossible


||| "… target creature with different names."
public export
badSingularNameAgreement : Unspellable (Noun [] Object) (\ok =>
  NamesAgree DifferentNames (Macros.target Macros.creature) {pl = ok})
badSingularNameAgreement Refl impossible


||| "Creature cards in graveyards can't be countered."
public export
badCounteredInGraveyard : Unspellable (StaticEffect []) (\ok =>
  Macros.objectCant "Counter"
    (Macros.allOf (And [Macros.creature, InZone Macros.graveyardZ])) {dp = ok})
badCounteredInGraveyard Oh impossible


public export
badChapterOnNonSaga : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Enchantment])
       [ Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing Nothing Nothing (Macros.draw You (Lit 1)) ]
       Nothing {fl = ok})
badChapterOnNonSaga MkFaceLaws impossible


||| "— Draw a card."
public export
badEmptyChapterMark : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [] {cm = ok}) [] Nothing [] Nothing Nothing Nothing (Macros.draw You (Lit 1)))
badEmptyChapterMark Oh impossible


||| "II, II — Draw a card."
public export
badRepeatedChapterMark : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterII, ChapterII] {cm = ok}) [] Nothing [] Nothing Nothing Nothing (Macros.draw You (Lit 1)))
badRepeatedChapterMark Oh impossible


||| "I — Draw a card. This ability triggers only once each turn."
public export
badChapterLimit : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing (Just OncePerTurn) Nothing
    (Macros.draw You (Lit 1)) {cd = ok})
badChapterLimit Oh impossible


||| "I — , if you control a creature, draw a card."
public export
badChapterIntervening : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing Nothing (Just (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You])))
    (Macros.draw You (Lit 1)) {cd = ok})
badChapterIntervening Oh impossible


||| "{T}: Draw a card. Do this only once each turn."
public export
badActionLimitOnActivated : Unspellable Ability (\ok =>
  Activated TapSymbol (Macros.draw You (Lit 1)) Nothing (Just ActionOncePerTurn) Nothing
            Nothing {ul = ok})
badActionLimitOnActivated Oh impossible


||| "I — while you control a creature, draw a card."
public export
badChapterWhile : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) []
    (Just (Macros.whileState (Macros.exists (And [Macros.creature, HasPossessor ControllerAx You]))))
    [] Nothing Nothing Nothing
    (Macros.draw You (Lit 1)) {cd = ok})
badChapterWhile Oh impossible


||| "I — and whenever you draw a card, draw a card."
public export
badChapterJoin : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) [] Nothing
    [ Macros.joinedHead Whenever (Draws You) ]
    Nothing Nothing Nothing
    (Macros.draw You (Lit 1)) {cd = ok})
badChapterJoin Oh impossible


||| "If I — would happen, draw a card instead."
public export
badChapterReplacement : Unspellable (StaticEffect []) (\ok =>
  Intercepts (ChapterMark [ChapterI]) [] Nothing (Macros.draw You (Lit 1)) Repeatedly Nothing {ok})
badChapterReplacement Oh impossible


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


||| "Draw a card. At the beginning of that turn's end step, you lose the game."
public export
badDeicticTurnWithoutIntroducer : Unspellable (Effect []) (\ok =>
  Sequentially [Draw You (Lit 1),
                Delayed (BeginningOf ThePart EndStep (Macros.thatTurns {ok})) [] Nothing
                        (Concludes LoseGame You)])
badDeicticTurnWithoutIntroducer Refl impossible


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


public export
badPlayPaymentTapSymbol : Unspellable (PlayPayment []) (\ok =>
  PayingInstead TapSymbol {ok})
badPlayPaymentTapSymbol Oh impossible


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


||| "{T}: Add one mana of the chosen color."
public export
badChosenColorNoChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing {cq = ok}) []) Nothing Nothing Nothing Nothing ]
       Nothing)
badChosenColorNoChooser Refl impossible


||| "sources of the last chosen color"
public export
badLastChosenColorNoChooser : Unspellable (Predicate [] Object) (\ok =>
  OfTheLastChosen Color {ok = ok})
badLastChosenColorNoChooser Oh impossible


public export
badLastChosenBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (DamageRule AnyDamage (DealtBy (Macros.allOf (And [Macros.source,
                                                OfTheLastChosen Color {ok = ok}]))) (Macros.shieldingIt You) (Prevent CutAll Nothing) Repeatedly)
       , Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing Openly) ]
       Nothing)
badLastChosenBeforeChooser Oh impossible


public export
badLastChosenWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly)
       , Static (DamageRule AnyDamage (DealtBy (Macros.allOf (And [Macros.source,
                                                OfTheLastChosen Color {ok = ok}]))) (Macros.shieldingIt You) (Prevent CutAll Nothing) Repeatedly) ]
       Nothing)
badLastChosenWrongSort Oh impossible


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
  Macros.jointCard [Color] "Joint choice witness" Nothing []
    (MkTypeLine [creatureType "Shapeshifter"] [Creature])
    [ Static (Gains Macros.thisCreature
               (Macros.keywordQuality "Protection" (OfChosen Color)))
    , Static (Macros.entersChoosing Macros.thisCreature Color)
    ]
    (Just (1, 1))
