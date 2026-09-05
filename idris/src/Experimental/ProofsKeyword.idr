module Experimental.ProofsKeyword

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Sacrifice a creature: Draw a card."
public export
okSacrificeAsCost : Ability
okSacrificeAsCost =
  Activated (Do (Macros.sacrifice You (Macros.a Macros.creature)))
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing

||| "Sacrifice a creature, Exile the sacrificed card: Draw a card."
public export
badCostReadsSiblingDeed : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                       Do (Macros.exile (Macros.TheVerbed "Sacrifice" CardW Attributive OneOf)) {ok}])
            (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)
badCostReadsSiblingDeed Oh impossible

||| "Target creature gains flying."
public export
okGainsKeyword : Instruction []
okGainsKeyword =
  Macros.gains (Macros.target Macros.creature) (Macros.keyword "Flying") Nothing

||| "Target creature gains a spell ability."
public export
badGainsSpellAbility : Unspellable (Instruction []) (\ok =>
  Macros.gains (Macros.target Macros.creature) (Spell Nothing (Draw You (Lit 1))) Nothing {gr = ok})
badGainsSpellAbility Oh impossible

||| "Creature spells you cast cost {1} less to cast."
public export
okCostSubjectOnStack : StaticSpec []
okCostSubjectOnStack =
  Costs (Macros.allOf (And [Macros.creature, Macros.spell]))
              (CostLess (Lit 1) Nothing)

||| "Creatures you control cost {1} less to cast."
public export
badCostSubjectOnBattlefield : Unspellable (StaticSpec []) (\ok =>
  Costs (Macros.allOf Macros.creatureYouControl) (CostLess (Lit 1) Nothing) {cs = ok})
badCostSubjectOnBattlefield MkCostSubject impossible

||| "Creatures you control have flying."
public export
okBattlefieldFlying : StaticSpec []
okBattlefieldFlying =
  Gains (Macros.allOf Macros.creatureYouControl)
        (KeywordAbility "Flying" Nothing Nothing)

||| "Creatures you control have convoke."
public export
badBattlefieldConvoke : Unspellable (StaticSpec []) (\ok =>
  Gains (Macros.allOf Macros.creatureYouControl) (KeywordAbility "Convoke" Nothing Nothing) {ok})
badBattlefieldConvoke Oh impossible

||| "Target creature gets +1/+1, gains flying, and gains trample."
public export
okFlatCoordination : StaticSpec []
okFlatCoordination =
  AndAlso Nothing [ Gets Adds (Macros.target Macros.creature)
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains (Macros.It OneOf) (KeywordAbility "Flying" Nothing Nothing)
                  , Gains (Macros.It OneOf) (KeywordAbility "Trample" Nothing Nothing) ]

||| "Target creature gets +1/+1 and gains flying and gains trample."
public export
badNestedCoordination : Unspellable (StaticSpec []) (\ok =>
  AndAlso Nothing (Coord.(::) (AndAlso Nothing [ Gets Adds (Macros.target Macros.creature)
                                      (PtUp (Lit 1)) (PtUp (Lit 1))
                               , Gains ((Macros.It OneOf)) (KeywordAbility "Flying" Nothing Nothing) ])
                      {nc = ok}
                      (Coord.(::) (Gains ((Macros.It OneOf)) (KeywordAbility "Trample" Nothing Nothing)) Coord.Nil)))
badNestedCoordination Oh impossible

||| "Whenever a creature enters, destroy that creature."
public export
okThatCreatureAfterAntecedent : Ability
okThatCreatureAfterAntecedent =
  Triggered Whenever (Enters (Macros.a Macros.creature) Nothing) [] Nothing []
            Nothing Nothing Nothing
            (Macros.destroy (Macros.That (TypeW Creature) OneOf))

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
            , Gains (Macros.It ManyOf) (KeywordAbility "Flying" Nothing Nothing) ])

||| "Enchanted creature gets +1/+1 and they have flying."
public export
badCoordinatedHostPlural : Unspellable Ability (\ok =>
  Static (AndAlso Nothing [ Gets Adds (AttachHost Enchanted (TypeW Creature))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains ((Macros.It ManyOf) {ok = ok}) (KeywordAbility "Flying" Nothing Nothing) ]))
badCoordinatedHostPlural Refl impossible

||| "Create a 1/1 white Soldier creature token with flying."
public export
okTokenKeywordAbility : Instruction []
okTokenKeywordAbility =
  Macros.create (Lit 1)
    (MkToken (Just (Lit 1 ** Lit 1)) [White]
             (MkTypeLine [creatureType "Soldier"] [Creature])
             [KeywordAbility "Flying" Nothing Nothing] Nothing)

||| "Create a 1/1 white Soldier creature token with 'Draw two cards.'"
public export
badTokenSpellAbility : Unspellable (Instruction []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                 (MkTypeLine [creatureType "Soldier"] [Creature])
                                 [Spell Nothing (Draw You (Lit 1))] Nothing) {ta = ok})
badTokenSpellAbility Oh impossible

||| "Creatures you control have '{T}: Draw a card.'"
public export
okQuotedGrantOnPermanent : StaticSpec []
okQuotedGrantOnPermanent =
  Gains (Macros.allOf Macros.creatureYouControl)
        (Activated TapSymbol (Draw You (Lit 1)) Nothing Nothing Nothing Nothing)

||| "Instant and sorcery spells you cast have '{T}: Draw a card.'"
public export
badQuotedGrantOnSpell : Unspellable (StaticSpec []) (\ok =>
  Gains (Macros.allOf (And [Macros.instantOrSorcery, Macros.spell, Macros.castBy You]))
        (Activated TapSymbol (Draw You (Lit 1)) Nothing Nothing Nothing Nothing) {ok})
badQuotedGrantOnSpell Oh impossible

||| "of the creature type of your choice"
public export
okYourChoiceCreatureType : Predicate [] Object
okYourChoiceCreatureType = OfYourChoice (SubtypeQ Creature) Nothing

||| "of the number of your choice"
public export
badYourChoiceNumber : Unspellable (Predicate [] Object) (\ok =>
  OfYourChoice Number Nothing {read = ok})
badYourChoiceNumber Oh impossible

||| "As this enchantment enters, choose a creature type. Creatures of the
||| chosen type get +1/+1."
public export
okReaderAfterChooser : Card
okReaderAfterChooser =
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly)
       , Static (Gets Adds (Macros.allOf (And [Macros.creature,
                                   Macros.ofChosen (SubtypeQ Creature)]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing

public export
badReaderBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (Gets Adds (Macros.allOf (And [Macros.creature,
                                   Macros.ofChosen (SubtypeQ Creature) {ok = ok}]))
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
                                   Macros.ofChosen (SubtypeQ Creature) {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing)
badTwoChoosersOneSortRead Refl impossible

public export
badChosenReadWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment (QSort Color) Nothing Openly)
       , Static (Gets Adds (Macros.allOf (And [Macros.creature,
                                   Macros.ofChosen (SubtypeQ Creature) {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing)
badChosenReadWrongSort Refl impossible

public export
badChosenProtectionBeforeChoice : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [creatureType "Angel"] [Creature])
       [ Static (Gains Macros.thisCreature
                       (KeywordAbility "Protection" (Just (ParamQuality (Macros.ofChosen Color {ok = ok}))) Nothing))
       , Static (EntersChoice Macros.thisCreature (QSort Color) Nothing Openly) ]
       (Just (2, 2)))
badChosenProtectionBeforeChoice Refl impossible

public export
badAscribedQualityBeforeChoice : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (Becomes (Macros.allOf (And [Macros.creature, HasPossessor ControllerAx You])) Adds (ChosenQuality (Macros.ofChosen (SubtypeQ Creature) {ok = ok})))
       , Static (EntersChoice Macros.thisEnchantment (QSort (SubtypeQ Creature)) Nothing Openly) ]
       Nothing)
badAscribedQualityBeforeChoice Refl impossible

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

||| "The same is true for menace and trample."
public export
okKeywordListOverGrant : Ability
okKeywordListOverGrant =
  AlsoForKeywords (Static (Conditionally
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing Nothing))
                             (Macros.exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")])) AsLongAs))
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
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing Nothing))
                             (Macros.exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")])) AsLongAs))
                  [] {lk = ok})
badEmptyKeywordList Oh impossible

public export
badKeywordListRepeatingBase : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing Nothing))
                             (Macros.exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")])) AsLongAs))
                  [TheKeyword "Menace", TheKeyword "Flying",
                   TheKeyword "Trample"] {lk = ok})
badKeywordListRepeatingBase Oh impossible

public export
badParameterisedKeywordInList : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing Nothing))
                             (Macros.exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")])) AsLongAs))
                  [TheKeyword "Ward"] {lk = ok})
badParameterisedKeywordInList Oh impossible

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

||| "Echo {2}{W}"
public export
okParameterisedEcho : Ability
okParameterisedEcho =
  KeywordAbility "Echo"
    (Just (ParamCost (Mana [Macros.generic 2, Macros.pip White]))) Nothing

||| "Echo"
public export
badBareEcho : Unspellable Ability (\ok =>
  KeywordAbility "Echo" Nothing Nothing {pf = ok})
badBareEcho Oh impossible

||| "each creature with flying"
public export
okKnownKeywordClass : AbilityClass
okKnownKeywordClass = KeywordClass "Flying"

||| "each creature with flyign" — the refusal names the label and the table
public export
badUnknownKeywordClass : Unspellable AbilityClass (\ok =>
  KeywordClass "Flyign" {kn = ok})
badUnknownKeywordClass KeywordInFactsTable impossible

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

public export
sharedSubjectSurvivesSecondSingular : Instruction []
sharedSubjectSurvivesSecondSingular =
  Sequentially [Macros.exile (Macros.target Macros.artifact),
                Macros.sharedSubject (Macros.target Macros.creature)
                  [ Gets Adds (Macros.ownSubject (Macros.target Macros.creature))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains (Macros.ownSubject (Macros.target Macros.creature))
                          (Macros.keyword "Flying") ]
                  (Just Macros.untilEndOfTurn)]

||| "target creature" -- the subject a shared-subject clause re-reads
public export
okSharedSubjectDelta :
  Noun (selfSubjIntro {bs = []} (Macros.target Macros.creature)) Object
okSharedSubjectDelta = Macros.ownSubject {bs = []} (Macros.target Macros.creature)

public export
badSharedSubjectEmptyDelta : Unspellable (Noun [] Object) (\ok =>
  Macros.ownSubject {bs = []} (Macros.ItVerbed "Untap" OneOf) {ok})
badSharedSubjectEmptyDelta Refl impossible

||| "Pay 2 life: Draw a card."
public export
okOwnPayerCost : Ability
okOwnPayerCost =
  Activated (Macros.payLife You 2) (Draw You (Lit 1))
            Nothing Nothing Nothing Nothing

||| "An opponent pays 2 life: Draw a card."
public export
badForeignPayerCost : Unspellable Ability (\ok =>
  Activated (Macros.payLife Macros.anOpponent 2) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing {py = ok})
badForeignPayerCost Oh impossible

||| "An opponent sacrifices a creature: Draw a card."
public export
badForeignSacrificeCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice Macros.anOpponent (Macros.a Macros.creature))) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing {py = ok})
badForeignSacrificeCost Oh impossible

||| "Counter target spell."
public export
okCounterSpell : Instruction []
okCounterSpell = CounterSpell (Macros.target Macros.spell)

||| "Counter target creature."
public export
badCounterPermanent : Unspellable (Instruction []) (\ok =>
  CounterSpell (Macros.target Macros.creature) {ct = ok})
badCounterPermanent StackSpell impossible

||| "Counter target creature or player."
public export
badCounterJoinedPlayer : Unspellable (Instruction []) (\ok =>
  CounterSpell (Macros.target Macros.anyTarget) {ct = ok})
badCounterJoinedPlayer StackJoin impossible

||| "Companion — Each permanent card in your starting deck has mana value 2 or less."
public export
okCompanionCharacteristicRead : AbilityAt []
okCompanionCharacteristicRead =
  Macros.companion
    (EveryCardIs (And [Permanent, IsCard])
       (ACharacteristic (Compare [StatAxis ManaValue] AtMost (Lit 2))))

||| "Companion — Each card on the battlefield in your starting deck ...": a
||| zone is a place objects are during a game [CR#400.1] and the starting deck
||| is outside it [CR#103.2b], so it is not a characteristic [CR#109.3].
||| `EveryCardIs (And [Permanent, IsCard])` spells "each permanent card".
public export
badCompanionZoneScope : Unspellable (AbilityAt []) (\ok =>
  Macros.companion
    (EveryCardIs (InZone Macros.battlefieldZ)
       (ACharacteristic (Compare [StatAxis ManaValue] AtMost (Lit 2))) {dr = ok}))
badCompanionZoneScope ReadsCharacteristics impossible

||| "Companion — Each permanent card in your starting deck is tapped.":
||| whether a permanent is tapped is not a characteristic [CR#109.3], and a
||| card set aside for the starting deck is not on the battlefield [CR#103.2b].
public export
badCompanionBattlefieldStatus : Unspellable (AbilityAt []) (\ok =>
  Macros.companion
    (EveryCardIs (And [Permanent, IsCard])
       (ACharacteristic (HasStatus Tapped) {dr = ok})))
badCompanionBattlefieldStatus ReadsCharacteristics impossible

||| "Companion — Each nonland card in your starting deck shares a card type."
public export
okCompanionSharedCardType : AbilityAt []
okCompanionSharedCardType =
  Macros.companion (CardsShare (And [Not Macros.land, IsCard]) CardTypeQ)

||| "Companion — Each nonland card in your starting deck shares a color.":
||| color is a characteristic [CR#109.3] and the deck-building condition is
||| checked against the starting deck [CR#702.139a].
public export
okCompanionSharedColor : AbilityAt []
okCompanionSharedColor =
  Macros.companion (CardsShare (And [Not Macros.land, IsCard]) Color)

||| "Companion — Each nonland card in your starting deck shares a counter
||| kind.": a kind of counter is not among an object's characteristics
||| [CR#109.3].
public export
badCompanionSharedCounterKind : Unspellable (AbilityAt []) (\ok =>
  Macros.companion
    (CardsShare (And [Not Macros.land, IsCard]) CounterKindQ {dc = ok}))
badCompanionSharedCounterKind ComparesCharacteristic impossible

||| "Each player scries 1.": one scry clause over a distributed player
||| reference [CR#701.22a,701.22c].
public export
okEachPlayerScriesOne : Instruction []
okEachPlayerScriesOne = Macros.scry (Macros.each AnyPlayer) (Lit 1)
