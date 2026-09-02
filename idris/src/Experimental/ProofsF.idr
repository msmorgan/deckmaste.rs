||| Unspellable pins, continued from Experimental.ProofsE.
module Experimental.ProofsF

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Prevent all damage that would be dealt to this this turn."
||| Bare "This" is the source as an object, standing in no zone and projecting no head type [CR#120.1a].
public export
badPreventedBareThis : Unspellable (StaticEffect []) (\ok =>
  Prevents AnyDamage AllOfIt (ToRecipient This {rk = ok}) Nothing Nothing)
badPreventedBareThis ObjectTakes impossible


||| "Prevent all damage that would be dealt to target artifact this turn."
||| [CR#120.1a] at the shield: damage reaches only a battle, a creature or a planeswalker.
public export
badPreventDealtToArtifact : Unspellable (StaticEffect []) (\ok =>
  Prevents AnyDamage AllOfIt
           (ToRecipient (Macros.target Macros.artifact) {rk = ok}) Nothing Nothing)
badPreventDealtToArtifact ObjectTakes impossible


||| "All damage that would be dealt to you is dealt to target artifact instead."
||| [CR#120.1a] at the destination, which [CR#614.9] names from the same closed list.
public export
badRedirectToArtifact : Unspellable (StaticEffect []) (\ok =>
  Redirects AnyDamage AllOfIt (Macros.shieldingIt You) Nothing
            (Macros.target Macros.artifact) {rk = ok})
badRedirectToArtifact ObjectTakes impossible


||| "All damage that would be dealt to you is dealt to creatures you control instead."
||| [CR#614.9] replaces damage dealt to one thing with the same damage dealt to another: one head per arrow.
public export
badRedirectToPlural : Unspellable (StaticEffect []) (\ok =>
  Redirects AnyDamage AllOfIt (Macros.shieldingIt You) Nothing
            (AllOf Macros.creatureYouControl) {one = ok})
badRedirectToPlural Oh impossible


||| "The damage can't be prevented." -- as a card's whole printed line.
||| [CR#608.2c] licenses later text to read the instruction it FOLLOWS --
||| its own worked example is "Destroy target creature. It can't be
||| regenerated" -- and here there is no earlier instruction to read.
public export
badTheDamageUnannounced : Unspellable (StaticEffect []) (\ok =>
  CantPrevent AnyDamage (ThatDamage {ok}) NoPreventionOnly)
badTheDamageUnannounced Oh impossible


||| "You gain 3 life. The damage can't be prevented."
||| A life gain announces no damage, so the rider still points at nothing.
public export
badTheDamageAfterLifeGain : Unspellable (Effect []) (\ok =>
  Sequentially [ ChangeLife You (Up (Lit 3))
               , Continuously
                   (CantPrevent AnyDamage (ThatDamage {ok}) NoPreventionOnly) Nothing ])
badTheDamageAfterLifeGain Oh impossible


||| "If damage from a red source is prevented this way, you gain 3 life."
||| -- with no prevention before it. [CR#615.5] makes the additional
||| effect part of the prevention, so the container's outcome never stands
||| outside the rider it qualifies.
public export
badPreventedFromSourceUnannounced : Unspellable (Effect []) (\ok =>
  If (PreventedFromSource (And [Macros.source, ColorIs Red]) {ok})
     (Macros.gainsLife You (Lit 3)) Nothing)
badPreventedFromSourceUnannounced Refl impossible


||| "Deal 3 damage to target creature. You gain life equal to the damage prevented this way."
||| The phrase names a prevention, and a damage outcome is not one.
public export
badPreventedThisWayAfterDamage : Unspellable (Effect []) (\ok =>
  Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.creature)
               , ChangeLife You (Up (PreventedThisWay {ok})) ])
badPreventedThisWayAfterDamage Refl impossible


||| "You gain life equal to the damage prevented this way."
||| [CR#615.5] makes the additional effect part of the prevention, so the announcement never leaves the rider.
public export
badPreventedThisWayUnannounced : Unspellable (Effect []) (\ok =>
  ChangeLife You (Up (PreventedThisWay {ok})))
badPreventedThisWayUnannounced Refl impossible


||| "You gain 2 life for each card less than two you draw this way."
||| No ceiling was announced, and [CR#608.2d] fixes the number only where one is: with no bound written there is nothing the announcement fell short of.
public export
badShortOfCeilingUnannounced : Unspellable (Effect []) (\ok =>
  Macros.gainsLife You (Times 2 (ShortOfCeiling {ok})))
badShortOfCeilingUnannounced Refl impossible


||| "Prevent the next [the damage prevented this way] damage that would be dealt to you."
||| A shield cannot be sized by what it prevents: [CR#615.7]'s cap is spent one damage at a time.
public export
badShieldSizedByItsOwnPrevention : Unspellable (StaticEffect []) (\ok =>
  Prevents AnyDamage (TheNext (PreventedThisWay {ok}))
           (Macros.shieldingIt You) Nothing Nothing)
badShieldSizedByItsOwnPrevention Refl impossible


||| "Destroy target source."
||| [CR#120.7] makes a source a position in an event, not an object standing in a zone.
public export
badDestroySource : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target Macros.source) {ok})
badDestroySource OnField impossible


||| "This creature deals 3 damage to target source."
||| A source is not a recipient: [CR#120.7] names what dealt damage, [CR#120.1a] what may be dealt it.
public export
badDamageToSource : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3) (Macros.target Macros.source) {rk = ok})
badDamageToSource ObjectTakes impossible


||| "…target nonsource…"
||| [CR#120.7] makes a source a position an object occupies, not a property it has or lacks.
public export
badNonsource : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.source {ng = ok})
badNonsource Oh impossible


||| "All damage that would be dealt to you is dealt to you and permanents you control instead."
||| [CR#614.9] puts one thing at each end of the arrow, and the mixed group is plural by construction.
public export
badRedirectToGroup : Unspellable (StaticEffect []) (\ok =>
  Redirects AnyDamage AllOfIt (Macros.shieldingIt You) Nothing
            (Macros.youAnd (AllOf (And [Permanent, ControlledBy You]))) {one = ok})
badRedirectToGroup Oh impossible


||| "… it deals that much damage plus that much instead."
||| This row announces no outcome, so the anaphor finds nothing in scope [CR#614.6].
public export
badScaleShiftByThatMuch : Unspellable (StaticEffect []) (\ok =>
  Scales AnyDamage (Macros.a Macros.source)
         (Macros.shieldingIt (Macros.a (Macros.kindJoin AnyPlayer Permanent)))
         (Shifted ShiftUp (ThatMuch {ok})) Repeatedly)
badScaleShiftByThatMuch Refl impossible


||| "If a source would deal damage to target artifact, it deals double that damage to that artifact instead."
||| [CR#120.1a] at a fourth position: the scaling row reuses the recipient table whole.
public export
badScaleToArtifact : Unspellable (StaticEffect []) (\ok =>
  Scales AnyDamage (Macros.a Macros.source)
         (ToRecipient (Macros.target Macros.artifact) {rk = ok})
         (Multiplied Doubled) Repeatedly)
badScaleToArtifact ObjectTakes impossible


||| "Whenever this creature is dealt damage, it deals the damage prevented this way to any target."
||| The phrase names a prevention, and no prevention happened at a damage event either.
public export
badPreventedThisWayAfterDamageEvent : Unspellable Ability (\ok =>
  Triggered Whenever (IsDealtDamage AnyDamage Macros.thisCreature) [] Nothing [] Nothing Nothing Nothing
            (DealDamage It (PreventedThisWay {ok = ok}) (Macros.target Macros.anyTarget)))
badPreventedThisWayAfterDamageEvent Refl impossible


||| "Whenever a creature dies, this creature deals that much damage to any target."
||| A death announces no magnitude, so the anaphor has nothing to point at.
public export
badThatMuchAfterDeath : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature)) [] Nothing [] Nothing Nothing Nothing
            (DealDamage Macros.thisCreature (ThatMuch {ok = ok})
                        (Macros.target Macros.anyTarget)))
badThatMuchAfterDeath Refl impossible


||| "Whenever this creature is dealt damage, that creature deals that much damage to any target."
||| The demonstrative screen at a fourth minting site: it never picks out the speaker.
public export
badThatCreatureIsDamagedSelf : Unspellable Ability (\ok =>
  Triggered Whenever (IsDealtDamage AnyDamage Macros.thisCreature) [] Nothing [] Nothing Nothing Nothing
            (DealDamage (That (TypeW Creature) {ok = ok}) ThatMuch
                        (Macros.target Macros.anyTarget)))
badThatCreatureIsDamagedSelf Refl impossible


||| "Target player can't gain life." — as a permanent's printed line.
||| A static ability does not target [CR#115.1a..115.1e].
public export
badStaticPlayerCantTargets : Unspellable Ability (\ok =>
  Static (Macros.playerCant "GainLife" (Macros.target AnyPlayer)) {ut = ok})
badStaticPlayerCantTargets Oh impossible


||| "Target creature becomes the basic land type of your choice until end of turn."
||| [CR#205.3d] refuses a subtype that corresponds to none of the object's types, and the subject's projected head is no land.
public export
badChosenBasicTypeOnCreature : Unspellable (Effect []) (\ok =>
  Continuously (SetsChosenQuality (Macros.target Macros.creature)
                                  (OfYourChoice (SubtypeQ Land) (Just BasicTypesOnly))
                                  {hr = ok})
               (Just Macros.untilEndOfTurn))
badChosenBasicTypeOnCreature Oh impossible


||| "Creatures are Mountains."
||| A subtype-only line carries no card type, and [CR#205.3i] puts Mountain in the land set.
public export
badCreaturesAreMountains : Unspellable (StaticEffect []) (\ok =>
  SetsType (AllOf Macros.creature)
           (MkToken Nothing [] (Macros.basicLandLine [landType "Mountain"]) [] Nothing)
           Nothing {af = ok})
badCreaturesAreMountains Oh impossible


||| "of the number of your choice"
||| A bound number reads back by numeric equality, never as a quality.
public export
badYourChoiceNumber : Unspellable (Predicate [] Object) (\ok =>
  OfYourChoice Number Nothing {read = ok})
badYourChoiceNumber Oh impossible


||| "Creatures you control of the chosen type get +1/+1. / As this enchantment enters, choose a creature type."
||| STRUCTURAL, and the label is the honest one: `AbilitySeq` threads a face's abilities left to right, so the read is typed before the chooser has bound anything. [CR#607.2d] links the two by role and orders neither.
public export
badReaderBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (Gets (AllOf (And [Macros.creature,
                                   OfChosen (SubtypeQ Creature) {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly) ]
       Nothing)
badReaderBeforeChooser Refl impossible


||| "As this enchantment enters, choose a creature type. / As this enchantment enters, choose a creature type. / Creatures you control of the chosen type get +1/+1."
||| A second choice of the same sort makes the read ambiguous [CR#607.4].
public export
badTwoChoosersOneSortRead : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly)
       , Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly)
       , Static (Gets (AllOf (And [Macros.creature,
                                   OfChosen (SubtypeQ Creature) {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing)
badTwoChoosersOneSortRead Refl impossible


||| "As this enchantment enters, choose a color. / Creatures you control of the chosen type get +1/+1."
||| The binding is sorted: a choice of one quality does not answer a read of another [CR#607.2d].
public export
badChosenReadWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing Openly)
       , Static (Gets (AllOf (And [Macros.creature,
                                   OfChosen (SubtypeQ Creature) {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing)
badChosenReadWrongSort Refl impossible


||| "This creature has protection from the chosen color. / As this creature enters, choose a color."
||| STRUCTURAL, the same threading at a keyword parameter: the read is typed before the chooser's ability. [CR#607.2d] links them by role.
public export
badChosenProtectionBeforeChoice : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [creatureType "Angel"] [Creature])
       [ Static (Gains Macros.thisCreature
                       (KeywordAbility "Protection" (Just (ParamQuality (OfChosen Color {ok = ok})))))
       , Static (EntersChoice Macros.thisCreature (QSort Color) Nothing Openly) ]
       (Just (2, 2)))
badChosenProtectionBeforeChoice Refl impossible


||| "Creatures you control are the chosen type in addition to their other types. / As this enchantment enters, choose a creature type."
||| STRUCTURAL, the same threading at the ascribed quality.
public export
badAscribedQualityBeforeChoice : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (AddsChosenQuality
                   (AllOf (And [Macros.creature, ControlledBy You]))
                   (OfChosen (SubtypeQ Creature) {ok = ok}))
       , Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly) ]
       Nothing)
badAscribedQualityBeforeChoice Refl impossible


||| "Creatures you control are artifacts in addition to their other types. The same is true for … . The same is true for … ."
||| One extension per statement: the singleton discipline at a third site.
public export
badDoubleExtension : Unspellable (StaticEffect []) (\ok =>
  AlsoOffBattlefield
    (AlsoOffBattlefield
       (BecomesAlso (AllOf (And [Macros.creature, ControlledBy You]))
                    (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing))) {nx = ok})
badDoubleExtension Oh impossible


||| "{U}: Counter target spell with the chosen name. / As this enchantment enters, choose a card name."
||| STRUCTURAL, the same threading with the reader inside an ACTIVATED ability. [CR#607.2d] links an activated reader to an as-enters chooser as readily as a static one.
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


||| "As this enchantment enters, choose a color. / {U}: Counter target spell with the chosen name."
||| [CR#607.2d]'s linked ability refers only to the first ability's choice, and a colour chooser made no name.
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
||| [CR#205.3m] is the creature-type list, and the subtype catalog is wider than the sort.
public export
badNonCreatureTypeExclusion : Unspellable (ChoiceDomain (QSort (SubtypeQ Creature))) (\ok =>
  TypeOtherThan (artifactType "Equipment") {ct = ok})
badNonCreatureTypeExclusion Refl impossible


||| "… target creature with different names."
||| [CR#201.2b] states the constraint over two or more objects in a group; one mention of one object has no two members to compare.
public export
badSingularNameAgreement : Unspellable (Noun [] Object) (\ok =>
  NamesAgree DifferentNames (Macros.target Macros.creature) {pl = ok})
badSingularNameAgreement Refl impossible


||| "Creature cards in graveyards can't be countered."
||| Countering removes a spell or ability from the stack [CR#701.6a]; a graveyard card is past that.
public export
badCounteredInGraveyard : Unspellable (StaticEffect []) (\ok =>
  Macros.objectCant "Counter"
    (AllOf (And [Macros.creature, InZone Macros.graveyardZ])) {zn = ok})
badCounteredInGraveyard Oh impossible


||| a plain enchantment printing "I — Draw a card."
||| [CR#714.1] makes the striated text box with chapter symbols part of the Saga frame.
public export
badChapterOnNonSaga : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Enchantment])
       [ Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing Nothing Nothing Macros.drawACard ]
       Nothing {ch = ok})
badChapterOnNonSaga Oh impossible


||| "— Draw a card." — a chapter line with no numeral before the dash.
||| A chapter symbol is its numeral [CR#107.15], so a marker with none is no marker.
public export
badEmptyChapterMark : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [] {cm = ok}) [] Nothing [] Nothing Nothing Nothing Macros.drawACard)
badEmptyChapterMark Oh impossible


||| "II, II — Draw a card."
||| [CR#107.15b] expands the joined marker into one ability per numeral, so a repeat prints one twice.
public export
badRepeatedChapterMark : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterII, ChapterII] {cm = ok}) [] Nothing [] Nothing Nothing Nothing Macros.drawACard)
badRepeatedChapterMark Oh impossible


||| "I — Draw a card. This ability triggers only once each turn."
||| [CR#714.2b] writes the whole header, so the printed line has nowhere for a rider.
public export
badChapterLimit : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing (Just OncePerTurn) Nothing
    Macros.drawACard {cd = ok})
badChapterLimit Oh impossible


||| "I — , if you control a creature, draw a card."
||| [CR#714.2b]'s expansion already carries the intervening "if" over the lore tally.
public export
badChapterIntervening : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) [] Nothing [] Nothing Nothing (Just (Exists (And [Macros.creature, ControlledBy You])))
    Macros.drawACard {cd = ok})
badChapterIntervening Oh impossible


||| "{T}: Draw a card. Do this only once each turn."
||| [CR#603.2h] states the rider of a TRIGGERED ability and states it as a rule about triggering -- "this ability triggers only if its source's controller has not yet taken the indicated action that turn" -- so it says nothing an activated ability could obey. [CR#602.2] puts an activated ability on the stack when its controller pays its cost, which no such rule gates; the cap an activated ability writes is "Activate only once each turn", which is `OncePerTurn`.
public export
badActionLimitOnActivated : Unspellable Ability (\ok =>
  Activated TapSymbol Macros.drawACard Nothing (Just ActionOncePerTurn) Nothing
            Nothing {ul = ok})
badActionLimitOnActivated Oh impossible


||| "I — while you control a creature, draw a card."
||| [CR#714.2b] writes the chapter header whole -- "When one or more lore
||| counters are put onto this Saga, if …" -- so the printed line has
||| nowhere to hang a concurrent clause either.
public export
badChapterWhile : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) []
    (Just (Macros.whileState (Exists (And [Macros.creature, ControlledBy You]))))
    [] Nothing Nothing Nothing
    Macros.drawACard {cd = ok})
badChapterWhile Oh impossible


||| "I — and whenever you draw a card, draw a card."
||| The same rule at the join: [CR#714.2b]'s expansion is the whole
||| header, so a chapter cannot be one half of a two-headed one.
public export
badChapterJoin : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) [] Nothing
    [ Macros.joinedHead Whenever (Draws You) ]
    Nothing Nothing Nothing
    Macros.drawACard {cd = ok})
badChapterJoin Oh impossible


||| "If I — would happen, draw a card instead."
||| The chapter symbol stands for a trigger [CR#107.15]; the event a replacement names is
||| the lore counter's placement [CR#714.2b].
public export
badChapterReplacement : Unspellable (StaticEffect []) (\ok =>
  Intercepts (ChapterMark [ChapterI]) [] Nothing Macros.drawACard Repeatedly Nothing {ok})
badChapterReplacement Oh impossible


||| "You may look at your hand any time."
||| [CR#402.3] already lets a player look at their own hand at any time.
public export
badLookAtHandRider : Unspellable (StaticEffect []) (\ok =>
  Visibility LookAt You WholeHand {vo = ok})
badLookAtHandRider Oh impossible


||| "Lands you control are every creature type."
||| [CR#205.3m] gives the creature types to creatures and kindreds, so the space demands its own host.
public export
badEveryCreatureTypeOnLand : Unspellable (StaticEffect []) (\ok =>
  AddsEveryType (AllOf (And [Macros.land, ControlledBy You])) CreatureSpace
                {sh = ok})
badEveryCreatureTypeOnLand Oh impossible


||| "Creatures you control are every basic land type."
||| The other direction of the same host gate: nothing here adds the land card type for the subtype to sit on.
public export
badEveryBasicLandTypeOnCreature : Unspellable (StaticEffect []) (\ok =>
  AddsEveryType (AllOf (And [Macros.creature, ControlledBy You])) BasicLandSpace
                {sh = ok})
badEveryBasicLandTypeOnCreature Oh impossible


||| "Draw a card. At the beginning of that turn's end step, you lose the game."
||| The possessive reaches a mention, so a clause that made no turn leaves nothing for it to reach.
public export
badDeicticTurnWithoutIntroducer : Unspellable (Effect []) (\ok =>
  Sequentially [Draw You (Lit 1),
                Delayed (BeginningOf EndStep (ByWord ThatTurns) {td = ok}) [] Nothing
                        (Concludes LoseGame You)])
badDeicticTurnWithoutIntroducer (NoTurnDeixis) impossible
badDeicticTurnWithoutIntroducer (TurnInScope) impossible


||| "After this combat phase, there is an additional turn."
||| [CR#500.8] adds a phase to a turn and [CR#500.9] a step to a phase; a turn is neither [CR#500.1].
public export
badAdditionalTurn : Unspellable (Effect []) (\ok =>
  AdditionalPart Turn (Just Combat) (Lit 1) Nothing {ad = ok})
badAdditionalTurn Oh impossible


||| "Spells with the chosen name can't be activated."
||| An activated ability is the only kind that can be activated [CR#602.1c], so the noun is ability-kinded.
public export
badActivatedSpellClass : Unspellable
  (StaticEffect [MkBinding AD (Quality CardName) OneOf QualityP]) (\ok =>
  Macros.objectCant "Activate"
    (AllOf (And [Macros.spell, Named ChosenName])) {dp = ok})
badActivatedSpellClass Participant impossible


||| "Activated abilities of artifacts can't be Nothing cast."
||| Casting is of a card as a spell [CR#601.1a], and an unactivated ability is no object at all [CR#109.1].
public export
badCastAbilityClass : Unspellable (StaticEffect []) (\ok =>
  Macros.objectCant "Cast"
    (AllOf (And [AbilityHead AnyActivated, AbilityOf (AllOf Macros.artifact)]))
    {dp = ok})
badCastAbilityClass Participant impossible


||| "You may {T} rather than pay this spell's mana cost."
||| [CR#107.5] fixes "{T}" as tapping a permanent, and a spell being paid for is on the stack [CR#601.2b].
public export
badAltCostTapSymbol : Unspellable (StaticEffect []) (\ok =>
  AltCost This (Just TapSymbol) {ap = ok})
badAltCostTapSymbol NoAltPayment impossible
badAltCostTapSymbol AltPaymentWritten impossible


||| "You may [+1] rather than pay this spell's mana cost."
||| [CR#606.2] makes the loyalty symbol an activation-cost component, payable only on a permanent [CR#606.3].
public export
badAltCostLoyaltySymbol : Unspellable (StaticEffect []) (\ok =>
  AltCost This (Just (LoyaltySymbol (LoyaltyUp 1))) {ap = ok})
badAltCostLoyaltySymbol NoAltPayment impossible
badAltCostLoyaltySymbol AltPaymentWritten impossible


||| "Escalate {2}" printed on a spell that writes no modes.
||| [CR#702.120a] makes escalate "a static ability of modal spells" meaning "for each mode you choose beyond the first ... pay an additional [cost]", and [CR#700.2] gives a spell modes only where its text writes them.
public export
badEscalateWithoutModes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [Macros.keywordCosting "Escalate" (Mana [Macros.generic 2])] Nothing {tx = ok})
badEscalateWithoutModes Oh impossible


||| "Entwine {2}" printed on a spell that writes no modes.
||| [CR#702.42a] makes entwine "a static ability of modal spells" meaning "You may choose all modes of this spell instead of just the number specified"; a spell with no modes has none to choose.
public export
badEntwineWithoutModes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
       [Macros.keywordCosting "Entwine" (Mana [Macros.generic 2])] Nothing {tx = ok})
badEntwineWithoutModes Oh impossible


||| "You may cast this card from your graveyard by {T} rather than paying its mana cost."
||| A licence's alternative cost is paid as the card is cast [CR#601.2f..601.2h], from the zone it is in [CR#601.2a]; [CR#107.5]'s "{T}" taps a permanent.
public export
badPlayPaymentTapSymbol : Unspellable (PlayPayment []) (\ok =>
  PayingInstead TapSymbol {ok})
badPlayPaymentTapSymbol Oh impossible


||| "As an additional cost to cast this spell, {T}."
||| [CR#118.8] pays an additional cost with the mana cost, at [CR#601.2f..601.2h], and a spell being cast is on the stack [CR#601.2b] where [CR#107.5]'s "{T}" taps a permanent.
public export
badAddedCostTapSymbol : Unspellable (StaticEffect []) (\ok =>
  AddedCost TapSymbol False {ap = ok})
badAddedCostTapSymbol AddedPaymentWritten impossible


||| "As an additional cost to cast this spell, [+1]."
||| [CR#606.2] makes the loyalty symbol an activation-cost component, payable only on a permanent [CR#606.3]; a spell being cast is not one.
public export
badAddedCostLoyaltySymbol : Unspellable (StaticEffect []) (\ok =>
  AddedCost (LoyaltySymbol (LoyaltyUp 1)) False {ap = ok})
badAddedCostLoyaltySymbol AddedPaymentWritten impossible


||| "You may sacrifice a Mountain rather than pay this spell's mana cost" written as a resolving clause
||| [CR#113.6d] prices one object, and the node names none: the clause has no spell to price.
public export
badAltCostClause : Unspellable (Effect []) (\ok =>
  Continuously (AltCost This (Just (Do (Macros.sacrifice You
                  (Macros.a (And [Macros.land, HasSubtype (landType "Mountain")]))))))
               Nothing {cl = ok})
badAltCostClause Oh impossible


||| "Regenerate target creature card in your graveyard."
||| [CR#701.19a] regenerates a permanent, shielding it the next time it would be destroyed.
public export
badRegenerateInGraveyard : Unspellable (Effect []) (\ok =>
  Regenerate (Macros.a (And [Macros.creature, InZone Macros.graveyardZ]))
             {zn = ok})
badRegenerateInGraveyard Oh impossible


||| "Creature cards in your graveyard can't be regenerated."
||| The prohibition's subject stands where the verb's object does, so [CR#701.19a] refuses it there too.
public export
badRegeneratedInGraveyard : Unspellable (StaticEffect []) (\ok =>
  Macros.objectCant "Regenerate"
                    (Macros.a (And [Macros.creature, InZone Macros.graveyardZ]))
                    {zn = ok})
badRegeneratedInGraveyard Oh impossible


||| "{T}: Add one mana of the chosen color." (on a card that chooses nothing)
||| The production reads a binding, and a card that made no choice has none to read [CR#607.2d].
public export
badChosenColorNoChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing {cq = ok}) []) Nothing Nothing Nothing Nothing ]
       Nothing)
badChosenColorNoChooser Refl impossible


||| "sources of the last chosen color" (on a card that chooses nothing)
||| The marked read demands that a choice stand at all, and zero is not one or more [CR#607.2d].
public export
badLastChosenColorNoChooser : Unspellable (Predicate [] Object) (\ok =>
  OfLastChosen Color {ok = ok})
badLastChosenColorNoChooser ChoiceMade impossible


||| "Prevent all damage that would be dealt to you by sources of the last chosen color. / As this enchantment enters, choose a color."
||| STRUCTURAL, the same threading at the MARKED read, whose count is zero before the chooser's ability.
public export
badLastChosenBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (Prevents AnyDamage AllOfIt
                          (Macros.shieldingIt You)
                          (Just (AllOf (And [Macros.source,
                                             OfLastChosen Color {ok = ok}])))
                          Nothing)
       , Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing Openly) ]
       Nothing)
badLastChosenBeforeChooser ChoiceMade impossible


||| "As this enchantment enters, choose a creature type. / Prevent all damage that would be dealt to you by sources of the last chosen color."
||| The marked binding is sorted too, so a creature-type choice leaves the colour count at zero [CR#607.2d].
public export
badLastChosenWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly)
       , Static (Prevents AnyDamage AllOfIt
                          (Macros.shieldingIt You)
                          (Just (AllOf (And [Macros.source,
                                             OfLastChosen Color {ok = ok}])))
                          Nothing) ]
       Nothing)
badLastChosenWrongSort ChoiceMade impossible


||| "Starting with you, each player votes for death or death."
||| [CR#701.38b] connects each of a ballot's words to A DIFFERENT effect, so
||| one word listed twice is one option twice over and the reads that name an
||| option ("if death gets more votes", "for each death vote") have nothing to
||| tell apart.
public export
badRepeatedBallotOption : Unspellable (Ballot []) (\ok =>
  ByLabel ["death", "death"] {ok})
badRepeatedBallotOption Oh impossible


||| "Starting with you, each player votes for death."
||| [CR#701.38a] has each player vote "for one choice from A LIST OF OPTIONS";
||| a list of one leaves no choice to make and no tally to compare.
public export
badSingletonBallot : Unspellable (Ballot []) (\ok =>
  ByLabel ["death"] {ok})
badSingletonBallot Oh impossible
