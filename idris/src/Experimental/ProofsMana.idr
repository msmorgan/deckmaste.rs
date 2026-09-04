module Experimental.ProofsMana

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "your hand"
public export
okOwnedHand : ZoneExpr []
okOwnedHand = ZoneAt Hand (PossessedBy You)

||| "your battlefield"
public export
badOwnedBattlefield : Unspellable (ZoneExpr []) (\ok =>
  ZoneAt Battlefield (PossessedBy You {ps = ok}))
badOwnedBattlefield HandIsOwned impossible

||| "Discard a card at random. This deals damage equal to the discarded
||| card's mana value to any target."
public export
okDiscardedCardWord : Ability
okDiscardedCardWord =
  Activated (Do (Macros.discard You (Macros.aAtRandom (InZone Macros.handZ))))
            (DealDamage This
                          (StatOf ManaValue (Macros.TheVerbed "Discard" CardW Attributive OneOf))
                          (Macros.target Macros.anyTarget)) Nothing Nothing Nothing Nothing

public export
badDiscardedCreatureWord : Unspellable Ability (\ok =>
  Activated (Do (Macros.discard You (Macros.aAtRandom (And [Macros.creature, InZone Macros.handZ]))))
            (DealDamage This
                          (StatOf ManaValue (Macros.TheVerbed "Discard" (TypeW Creature) Attributive OneOf {ok}))
                          (Macros.target Macros.anyTarget)) Nothing Nothing Nothing Nothing)
badDiscardedCreatureWord Refl impossible

public export
nestedCompoundCost : Ability
nestedCompoundCost =
  Activated (Compound [Compound [Mana [Macros.generic 1], TapSymbol], TapSymbol])
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing

||| "{1}, {T}: Draw a card."
public export
okSingleTapCost : Ability
okSingleTapCost =
  Activated (Compound [Mana [Macros.generic 1], TapSymbol])
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing

||| "{T}, {T}: Draw a card."
public export
badDoubleTapCost : Unspellable Ability (\ok =>
  Activated (Compound [TapSymbol, TapSymbol]) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing {tp = ok})
badDoubleTapCost Oh impossible

||| "{1}: Draw a card."
public export
okNonEmptyManaCost : Ability
okNonEmptyManaCost =
  Activated (Mana [Macros.generic 1]) (Draw You (Lit 1))
            Nothing Nothing Nothing Nothing

||| ": Draw a card."
public export
badEmptyManaCost : Unspellable Ability (\ok =>
  Activated (Mana [] {wr = ok}) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badEmptyManaCost IsNonEmpty impossible

||| "{W/U/P}"
public export
okDistinctPhyrexian : ManaSymbol
okDistinctPhyrexian = Phyrexian White (Just Blue)

||| "{W/W/P}"
public export
badSameColorPhyrexian : Unspellable ManaSymbol (\ok =>
  Phyrexian White (Just White) {ds = ok})
badSameColorPhyrexian Oh impossible

||| "{U/B}"
public export
okDistinctHybrid : ManaSymbol
okDistinctHybrid = Macros.hybridPip Blue Black

||| "{U/U}"
public export
badSameColorHybrid : Unspellable ManaSymbol (\ok =>
  Macros.hybridPip Blue Blue {ds = ok})
badSameColorHybrid Oh impossible

||| "Exile a creature you control. If you do, return it to the battlefield."
public export
okIfDoneReadsDoneBody : Instruction []
okIfDoneReadsDoneBody =
  IfDone (Macros.exile You (Macros.a Macros.creatureYouControl))
         (Just (Macros.move (Macros.It OneOf) Macros.battlefieldZ)) Nothing

||| "Sacrifice a creature. If you don't, exile it."
public export
badIfNotReadsMandatoryBody : Unspellable (Instruction []) (\ok =>
  (IfDone (Macros.sacrifice You (Macros.a Macros.creature)) Nothing (Just (Macros.exile You ((Macros.It OneOf) {ok})))))
badIfNotReadsMandatoryBody Refl impossible

||| "Counter target spell unless its controller pays {3}."
public export
okUnlessManaCost : Instruction []
okUnlessManaCost =
  Unless (CounterSpell (Macros.target Macros.spell))
         (Macros.controllerOf (Macros.It OneOf)) (Mana [Macros.generic 3])

||| "Counter target spell unless its controller taps."
public export
badUnlessTapSymbol : Unspellable (Instruction []) (\ok =>
  Unless (CounterSpell (Macros.target Macros.spell)) (Macros.controllerOf ((Macros.It OneOf))) TapSymbol {pb = ok})
badUnlessTapSymbol Oh impossible

||| "Activate only before the combat damage step."
public export
okBeforeCombatDamage : Ability
okBeforeCombatDamage =
  Activated (Mana [Macros.generic 2]) (Draw You (Lit 1))
            (Just (BeforePart CombatDamage Nothing)) Nothing Nothing Nothing

||| "Activate only before the turn."
||| A turn is made of its phases [CR#500.1], so a point before the turn one is
||| already taking is not a window inside it; a step or phase is
||| (`okBeforeCombatDamage`).
public export
badBeforeTheTurn : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) (Draw You (Lit 1))
            (Just (BeforePart Turn Nothing {bp = ok})) Nothing Nothing Nothing)
badBeforeTheTurn Oh impossible

||| "Activate only before each player's attackers are declared."
public export
okDistributiveAttackWindow : Ability
okDistributiveAttackWindow =
  Activated (Mana [Macros.generic 2]) (Draw You (Lit 1))
            (Just (BeforePart DeclareAttackers (Just (Macros.each AnyPlayer))))
            Nothing Nothing Nothing

||| "Activate only before all players' attackers are declared."
||| A turn part has one active player [CR#102.1].
public export
badPluralAttackWindow : Unspellable Ability (\ok =>
  Activated (Mana [Macros.generic 2]) (Draw You (Lit 1)) (Just (BeforePart DeclareAttackers (Just (Macros.allOf AnyPlayer)) {pk = ok})) Nothing Nothing Nothing)
badPluralAttackWindow Oh impossible

||| "Cast this spell only before the combat damage step."
public export
okCastBeforeCombatDamage : Ability
okCastBeforeCombatDamage =
  Spell (Just (BeforePart CombatDamage Nothing)) (Draw You (Lit 1))

||| "Cast this spell only before the turn."
||| A turn is made of its phases [CR#500.1], so a point before the turn the
||| spell would be cast in is not a window inside it (`okCastBeforeCombatDamage`).
public export
badCastBeforeTheTurn : Unspellable Ability (\ok =>
  Spell (Just (BeforePart Turn Nothing {bp = ok})) (Draw You (Lit 1)))
badCastBeforeTheTurn Oh impossible

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

||| "Add {R}."
public export
okSingleProduction : Instruction []
okSingleProduction = AddMana You (Lit 1) (Runs [[OfColor Red]]) []

||| "Add."
public export
badEmptyProduction : Unspellable (Instruction []) (\ok =>
  AddMana You (Lit 1) (Runs [] {ok}) [])
badEmptyProduction Oh impossible

||| "Add {R} or ."
public export
badEmptyAlternative : Unspellable (Instruction []) (\ok =>
  AddMana You (Lit 1) (Runs [[OfColor Red], []] {ok}) [])
badEmptyAlternative Oh impossible

||| "Add {C}. Spend this mana only to activate abilities."
public export
okSpendPurpose : Instruction []
okSpendPurpose =
  AddMana You (Lit 1) (Runs [[Colorless]]) [SpendOnly [ToActivate Nothing]]

||| "Add {C}. Spend this mana only."
public export
badPurposelessSpend : Unspellable (Instruction []) (\ok =>
  AddMana You (Lit 1) (Runs [[Colorless]]) [SpendOnly [] {ne = ok}])
badPurposelessSpend IsNonEmpty impossible

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
okAddedCostSacrifice : StaticSpec []
okAddedCostSacrifice =
  AddedCost (Do (Macros.sacrifice You (Macros.a Macros.artifact))) False

||| "As an additional cost to cast this spell, {T}."
public export
badAddedCostTapSymbol : Unspellable (StaticSpec []) (\ok =>
  AddedCost TapSymbol False {ap = ok})
badAddedCostTapSymbol AddedPaymentWritten impossible

||| "As an additional cost to cast this spell, [+1]."
public export
badAddedCostLoyaltySymbol : Unspellable (StaticSpec []) (\ok =>
  AddedCost (LoyaltySymbol (LoyaltyUp 1)) False {ap = ok})
badAddedCostLoyaltySymbol AddedPaymentWritten impossible

||| "Creatures you control get +1/+1 until end of turn."
public export
okContinuousPumpClause : Instruction []
okContinuousPumpClause =
  Continuously (Gets Adds (Macros.allOf Macros.creatureYouControl)
                     (PtUp (Lit 1)) (PtUp (Lit 1)))
               (Just Macros.untilEndOfTurn)

||| "You may sacrifice a Mountain rather than pay this spell's mana cost"
public export
badAltCostClause : Unspellable (Instruction []) (\ok =>
  Continuously (AltCost This (Just (Do (Macros.sacrifice You
                  (Macros.a (And [Macros.land, HasSubtype (landType "Mountain")]))))))
               Nothing {cl = ok})
badAltCostClause Oh impossible

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

||| "… pays {1} for each artifact you control."
public export
okForEachScaledMana : Cost []
okForEachScaledMana =
  Macros.scaledMana GenericUnit (Macros.forEach 1 (And [Macros.artifact,
                                                 HasPossessor ControllerAx You]))

||| "Counter target spell unless its controller pays {2}."
public export
badLiteralScaledMana : Unspellable (Cost []) (\ok =>
  Macros.scaledMana GenericUnit (Lit 2) {fe = ok})
badLiteralScaledMana Oh impossible

public export
badBareCountScaledMana : Unspellable (Cost []) (\ok =>
  Macros.scaledMana GenericUnit (Macros.countOf (And [Macros.artifact, HasPossessor ControllerAx You])) {fe = ok})
badBareCountScaledMana Oh impossible

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

||| "For each color among permanents you control, add one mana of that color."
public export
okChosenColorPerColor : Instruction []
okChosenColorPerColor =
  ForEachKindOf ColorAxis
    (Just (Macros.allOf (And [Permanent, HasPossessor ControllerAx You]))) Color
    (AddMana You (Lit 1) (OfChosenColor Nothing) [])

||| "For each color among permanents you control, add one mana of that color"
public export
badRepeatedCarriesNoColor : Unspellable (Instruction []) (\ok =>
  Repeated (DistinctCount ColorAxis (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))
           (AddMana You (Lit 1) (OfChosenColor Nothing {cq = ok}) []))
badRepeatedCarriesNoColor Refl impossible

||| "For each color among permanents you control, … of that creature type."
public export
badAxisValueCrossing : Unspellable (Instruction []) (\ok =>
  ForEachKindOf ColorAxis (Just (Macros.allOf (And [Permanent, HasPossessor ControllerAx You])))
                (SubtypeQ Creature) (Draw You (Lit 1)) {sc = ok})
badAxisValueCrossing Refl impossible

||| "For each creature type, …"
public export
badDomainlessOpenAxis : Unspellable (Instruction []) (\ok =>
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

||| "Whenever a player taps a land for mana, …"
public export
okTappedForMana : GameEvent []
okTappedForMana =
  TappedForMana (Just (Macros.a AnyPlayer)) (Macros.a Macros.land)

||| "Whenever a card in a graveyard is tapped for mana, …" — only a permanent
||| is tapped for mana [CR#106.12].
public export
badTappedForManaOffField : Unspellable (GameEvent []) (\ok =>
  TappedForMana (Just You) (Macros.a (InZone Macros.graveyardZ)) {zn = ok})
badTappedForManaOffField Oh impossible

public export
afterALandTapForMana : Bindings
afterALandTapForMana =
  eventAfter (the (GameEvent [])
    (TappedForMana (Just (Macros.a AnyPlayer)) (Macros.a Macros.land)))

public export
afterAPlainLandTap : Bindings
afterAPlainLandTap =
  eventAfter (the (GameEvent [])
    (VerbedEvent (Just (Macros.a AnyPlayer)) "Tap" (Just (Macros.a Macros.land))
                 Nothing))

||| "one mana of any type that land produced"
public export
okProducedByTapEvent : ProducedMana ProofsMana.afterALandTapForMana
okProducedByTapEvent = ProducedByEvent (Macros.That (TypeW Land) OneOf)

||| "Whenever a player taps a land, add one mana of any type that land
||| produced" — a tap that is not a mana ability resolving produces no mana
||| [CR#106.12a].
public export
badProducedByPlainTap :
  Unspellable (ProducedMana ProofsMana.afterAPlainLandTap) (\ok =>
    ProducedByEvent (Macros.That (TypeW Land) OneOf) {pm = ok})
badProducedByPlainTap Refl impossible

||| "Add one mana of any type that land produced"
public export
badProducedByEventWithoutEvent : Unspellable (ProducedMana []) (\ok =>
  ProducedByEvent (Macros.a Macros.land) {pm = ok})
badProducedByEventWithoutEvent Refl impossible

public export
afterManaAdded : Bindings
afterManaAdded =
  instrIntro (the (Instruction []) (AddMana You (Lit 1) (Runs [[Colorless]]) []))

||| "You don't lose this mana as steps and phases end."
public export
okThisManaAfterAdd : StaticSpec ProofsMana.afterManaAdded
okThisManaAfterAdd = KeepsUnspentMana You ThisMana

||| "You don't lose this mana as steps and phases end"
public export
badThisManaWithoutAdd : Unspellable (StaticSpec []) (\ok =>
  KeepsUnspentMana You (ThisMana {ok = ok}))
badThisManaWithoutAdd Refl impossible

||| "Target player sacrifices a creature of their choice."
public export
okBoundTheirChoice : Instruction []
okBoundTheirChoice =
  Macros.sacrifice (Macros.target AnyPlayer)
                   (Macros.aTheirChoice Macros.creature)

||| "Destroy a creature of their choice."
public export
badUnboundTheirChoice : Unspellable (Instruction []) (\ok =>
  Macros.destroy (Macros.aTheirChoice Macros.creature {ch = ok}))
badUnboundTheirChoice Refl impossible

||| "Target creature becomes a black Zombie in addition to its other types."
public export
okUnnamedAddition : Instruction []
okUnnamedAddition =
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [Black]
                            (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                   Nothing

||| "Target creature becomes a Zombie named Bob in addition to its other types."
public export
badNamedAddition : Unspellable (Instruction []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [] (MkTypeLine [creatureType "Zombie"] []) [] (Just "Bob"))
                   Nothing {ok = ok})
badNamedAddition Oh impossible

public export
badRepeatedAdditionColor : Unspellable (Instruction []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [Black, Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                   Nothing {ok = ok})
badRepeatedAdditionColor Oh impossible

public export
badRepeatedAdditionType : Unspellable (Instruction []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [] (MkTypeLine [] [Artifact, Artifact]) [] Nothing)
                   Nothing {ok = ok})
badRepeatedAdditionType Oh impossible
