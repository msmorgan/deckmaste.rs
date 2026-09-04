module Experimental.ProofsFaces

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Snow-Covered Forest — Basic Snow Land — Forest"
public export
okSingleSnow : Card
okSingleSnow =
  Macros.card "Snow-Covered Forest" Nothing [Basic, Snow]
              (MkTypeLine [landType "Forest"] [Land]) [] Nothing

||| "Snow Snow Land — Forest"
public export
badDuplicateSnow : Unspellable Card (\ok =>
  Macros.card "" Nothing [Snow, Snow] (MkTypeLine [landType "Forest"] [Land]) [] Nothing {fl = ok})
badDuplicateSnow MkCharacteristicsLaws impossible

||| "Draw a card." printed on an instant
public export
okSpellAbilityOnInstant : Card
okSpellAbilityOnInstant =
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [Spell Nothing (Draw You (Lit 1))] Nothing

||| "Draw a card."
public export
badSpellAbilityOnPermanent : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature])
       [Spell Nothing (Draw You (Lit 1))] (Just (1, 1)) {fl = ok})
badSpellAbilityOnPermanent MkCharacteristicsLaws impossible

||| "Creatures you control get +1/+1."
public export
badStaticOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
       [Static (Gets Adds (Macros.allOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1)))] Nothing {fl = ok})
badStaticOnSorcery MkCharacteristicsLaws impossible

||| "Cast this spell only during the declare attackers step. Draw a card."
public export
okCastWindowOnSpell : Card
okCastWindowOnSpell =
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [Spell (Just (DuringPart DeclareAttackers Nothing)) (Draw You (Lit 1))] Nothing

||| The same sentence spelled as a static permitting the cast. A cast window
||| is stated by the spell [CR#506.7], which is `okCastWindowOnSpell`'s window
||| slot, not a permission a spell card's static ability grants.
public export
badCastWindowAsDeonticStatic : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Static (OnlyDuring DeclareAttackers Nothing
                   (Macros.deontic This Permit ["Cast"] Patient NoDeonticPatient))
       , Spell Nothing (Draw You (Lit 1)) ] Nothing {fl = ok})
badCastWindowAsDeonticStatic MkCharacteristicsLaws impossible

||| "Flying"
public export
badKeywordOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Flying" Nothing Nothing] Nothing {fl = ok})
badKeywordOnInstant MkCharacteristicsLaws impossible

||| "{T}: Draw a card."
public export
badTapSorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Tap Sorcery" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [Activated TapSymbol (Draw You (Lit 1)) Nothing Nothing Nothing Nothing] Nothing {fl = ok})
badTapSorcery MkCharacteristicsLaws impossible

||| a creature card printed with no power or toughness
public export
badCreatureCardNoPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) [] Nothing {fl = ok})
badCreatureCardNoPt (MkCharacteristicsLaws {bx = MkCardBox}) impossible

||| a land card printed with "{1}"
public export
badLandWithManaCost : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Land]) [] Nothing {fl = ok})
badLandWithManaCost MkCharacteristicsLaws impossible

||| "Legendary Legendary Creature"
public export
badDuplicateSupertype : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary, Legendary] (MkTypeLine [] [Creature])
       [] (Just (1, 1)) {fl = ok})
badDuplicateSupertype MkCharacteristicsLaws impossible

||| "Land Creature Instant"
public export
badMixedPermanentSpellLine : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land, Creature, Instant]) [] (Just (1, 1)) {fl = ok})
badMixedPermanentSpellLine (MkCharacteristicsLaws {ln = MkCardLine}) impossible

||| a card printed with an empty type line
public export
badCardNoTypes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] []) [] Nothing {fl = ok})
badCardNoTypes (MkCharacteristicsLaws {ln = MkCardLine}) impossible

||| "Creature Creature"
public export
badCardDuplicateType : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) [] (MkTypeLine [] [Creature, Creature]) []
       (Just (2, 2)) {fl = ok})
badCardDuplicateType (MkCharacteristicsLaws {ln = MkCardLine}) impossible

public export
turnedFaceDownHeader : Ability
turnedFaceDownHeader =
  Triggered Whenever (StatusEvent (Macros.a Permanent) FaceDown)
            [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1))

||| "Target creature doesn't untap during its controller's next untap step."
public export
okDoesntUntapNextOnBattlefield : Instruction []
okDoesntUntapNextOnBattlefield =
  DoesntUntapNext (Macros.target Macros.creature) (Lit 1)

public export
badUntapNextGraveyard : Unspellable (Instruction []) (\ok =>
  DoesntUntapNext (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) (Lit 1) {ok = ok})
badUntapNextGraveyard Oh impossible

||| "Turn target creature face down."
public export
okTurnFaceDownOnBattlefield : Instruction []
okTurnFaceDownOnBattlefield =
  SetStatus FaceDown (Macros.target Macros.creature)

||| "Turn target creature card in your graveyard face down."
public export
badTurnFaceDownGraveyard : Unspellable (Instruction []) (\ok =>
  SetStatus FaceDown (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok = ok})
badTurnFaceDownGraveyard Oh impossible

||| "Target creature card in your hand phases out."
public export
badPhasesOutInHand : Unspellable (Instruction []) (\ok =>
  SetStatus PhasedOut (Macros.target (And [Macros.creature, InZone (Macros.handOf You)])) {ok = ok})
badPhasesOutInHand Oh impossible

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
badStarlessDefinedPt (MkCharacteristicsLaws {bx = MkCardBox}) impossible

||| "each creature with flying"
public export
okKnownKeywordPredicate : Predicate [] Object
okKnownKeywordPredicate = HasKeyword (TheKeyword "Flying")

||| "each creature with flyign"
public export
badUnknownKeywordPredicate : Unspellable (Predicate [] Object) (\ok =>
  HasKeyword (TheKeyword "Flyign") {kn = ok})
badUnknownKeywordPredicate Oh impossible

||| "Protection from red" on a creature card
public export
okProtectionOnPermanentCard : Card
okProtectionOnPermanentCard =
  Macros.card "" (Just [Macros.pip White]) []
       (MkTypeLine [creatureType "Soldier"] [Creature])
       [KeywordAbility "Protection" (Just (ParamQuality (ColorIs Red))) Nothing]
       (Just (2, 2))

||| "Protection from red"
public export
badProtectionOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Protection" (Just (ParamQuality (ColorIs Red))) Nothing] Nothing {fl = ok})
badProtectionOnInstant MkCharacteristicsLaws impossible

||| "Protection from red"
public export
okProtectionFromAColor : Ability
okProtectionFromAColor =
  KeywordAbility "Protection" (Just (ParamQuality (ColorIs Red))) Nothing

||| "Protection from player"
public export
badProtectionFromPlayerRestriction : Unspellable Ability (\ok =>
  KeywordAbility "Protection" (Just (ParamSubject AnyPlayer)) Nothing {pf = ok})
badProtectionFromPlayerRestriction Oh impossible

||| "Equip {2}" on an Equipment card
public export
okEquipOnEquipment : Card
okEquipOnEquipment =
  Macros.card "" (Just [Macros.generic 1]) []
       (MkTypeLine [artifactType "Equipment"] [Artifact])
       [KeywordAbility "Equip" (Just (ParamCost (Mana [Macros.generic 2]))) Nothing]
       Nothing

||| "Equip {2}"
public export
badEquipOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility "Equip" (Just (ParamCost (Mana [Macros.generic 2]))) Nothing]
       Nothing {fl = ok})
badEquipOnSorcery MkCharacteristicsLaws impossible

||| "each of up to two target creatures"
public export
okEachOfATargetGroup : Noun [] Object
okEachOfATargetGroup =
  EachOf (Described (TargetDet (Macros.upTo 2)) Macros.creature)

||| "each of one or more creatures"
public export
badEachOfCountedGroup : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.counted (Macros.atLeast 1) Macros.creature) {gm = ok})
badEachOfCountedGroup Oh impossible

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
badFortifiedCreatureNoun : Unspellable (Noun [] Object) (\ok =>
  AttachHost Fortified (TypeW Creature) {ok})
badFortifiedCreatureNoun Oh impossible

||| "Target creature gains indestructible."
public export
okGainsIndestructible : StaticSpec []
okGainsIndestructible =
  Gains (Macros.target Macros.creature)
        (KeywordAbility "Indestructible" Nothing Nothing)

||| "Target creature gains flash."
public export
badBattlefieldFlash : Unspellable (StaticSpec []) (\ok =>
  Gains (Macros.target Macros.creature) (KeywordAbility "Flash" Nothing Nothing) {ok})
badBattlefieldFlash Oh impossible

||| "Target spell gains indestructible."
public export
badSpellIndestructible : Unspellable (StaticSpec []) (\ok =>
  Gains (Macros.target Macros.spell) (KeywordAbility "Indestructible" Nothing Nothing) {ok})
badSpellIndestructible Oh impossible

||| a "Kindred Enchantment — Merfolk" card
public export
okKindredWithAnotherType : Card
okKindredWithAnotherType =
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Merfolk"] [Kindred, Enchantment])
       [KeywordAbility "Flying" Nothing Nothing] Nothing

||| a "Kindred Enchantment — Siege" card
public export
badSiegeWithoutBattle : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [battleType "Siege"] [Kindred, Enchantment])
       [KeywordAbility "Flying" Nothing Nothing] Nothing {fl = ok})
badSiegeWithoutBattle (MkCharacteristicsLaws {ln = MkCardLine}) impossible

||| a "Kindred — Merfolk" card naming no other card type
public export
badKindredAlone : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Merfolk"] [Kindred])
       [KeywordAbility "Flying" Nothing Nothing] Nothing {fl = ok})
badKindredAlone (MkCharacteristicsLaws {ln = MkCardLine}) impossible

||| "Enchanted creature can't attack."
public export
okEnchantedCreatureCantAttack : Ability
okEnchantedCreatureCantAttack =
  Static (Macros.deontic (AttachHost Enchanted (TypeW Creature))
                  Forbid ["Attack"] Agent NoDeonticPatient)

||| "Enchanted planeswalker can't attack.": a planeswalker an effect has made
||| a creature can attack [CR#205.1b], and the restriction is created even
||| while it is not one [CR#208.3a].
public export
okPlaneswalkerAttacks : Ability
okPlaneswalkerAttacks =
  Static (Macros.deontic (AttachHost Enchanted (TypeW Planeswalker))
                  Forbid ["Attack"] Agent NoDeonticPatient)

||| "you pay {2}"
public export
okPayMana : Instruction []
okPayMana = Pay You (Mana [Macros.generic 2]) PaidOnce

||| "you pay [+1]"
public export
badPayLoyalty : Unspellable (Instruction []) (\ok =>
  Pay You (LoyaltySymbol (LoyaltyUp 1)) PaidOnce {pb = ok})
badPayLoyalty Oh impossible

||| "[+1]: Draw a card." on a planeswalker card
public export
okLoyaltyOnPlaneswalker : Card
okLoyaltyOnPlaneswalker =
  Macros.cardOf "" (Just [Macros.pip Blue]) [Legendary]
       (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [Activated (LoyaltySymbol (LoyaltyUp 1)) (Draw You (Lit 1))
                  Nothing Nothing Nothing Nothing]
       (Macros.loyaltyBox 3)

||| "[+1]: Draw a card."
public export
badLoyaltySorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Loyalty Sorcery" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [Activated (LoyaltySymbol (LoyaltyUp 1)) (Draw You (Lit 1)) Nothing Nothing Nothing Nothing] Nothing {fl = ok})
badLoyaltySorcery MkCharacteristicsLaws impossible

||| "a token that's a copy of target creature, except it's an artifact"
public export
okCopyTypeException : Instruction []
okCopyTypeException =
  Create You (Lit 1)
         (TokenCopyOf (Macros.target Macros.creature)
                      [ExceptTypes (MkTypeLine [] [Artifact])]) []

public export
badEmptyCopyTypeException : Unspellable (Instruction []) (\ok =>
  Create You (Lit 1)
         (TokenCopyOf (Macros.target Macros.creature)
                      [ExceptTypes (MkTypeLine [] []) {ne = ok}]) [])
badEmptyCopyTypeException Oh impossible

||| "Copy target instant or sorcery spell."
public export
okCopyStackSpell : Instruction []
okCopyStackSpell =
  Copy FromStack You
       (Macros.target (And [Macros.instantOrSorcery, Macros.spell])) (Lit 1) []

||| "Copy target creature."
public export
badCopyPermanent : Unspellable (Instruction []) (\ok =>
  Copy FromStack You (Macros.target Macros.creature) (Lit 1) [] {cp = ok})
badCopyPermanent StackSpell impossible

||| "Create a token that's a copy of target creature. Untap that token."
public export
okSetStatusOnBattlefield : Instruction []
okSetStatusOnBattlefield =
  Sequentially [Create You (Lit 1)
                       (TokenCopyOf (Macros.target Macros.creature) []) [],
                SetStatus Untapped (Macros.That TokenW OneOf)]

||| "Copy target instant or sorcery spell. Untap that token."
public export
badStackCopyAsToken : Unspellable (Instruction []) (\ok =>
  Sequentially [Copy FromStack You (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                          (Lit 1) [],
                SetStatus Untapped (Macros.That TokenW OneOf {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badStackCopyAsToken (Refl, _) impossible

||| "Create a token that's a copy of target creature. Untap that copy."
public export
badTokenCopyAsCopyMention : Unspellable (Instruction []) (\ok =>
  Sequentially [Create You (Lit 1) (TokenCopyOf (Macros.target Macros.creature) []) [],
                SetStatus Untapped (Macros.That CopyW OneOf {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badTokenCopyAsCopyMention (Refl, _) impossible

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
badChapterOnNonSaga MkCharacteristicsLaws impossible

||| "You may sacrifice a Mountain rather than pay this spell's mana cost."
public export
okAltCostSacrifice : StaticSpec []
okAltCostSacrifice =
  AltCost This (Just (Do (Macros.sacrifice You
                 (Macros.a (And [Macros.land, HasSubtype (landType "Mountain")])))))

||| "You may {T} rather than pay this spell's mana cost."
public export
badAltCostTapSymbol : Unspellable (StaticSpec []) (\ok =>
  AltCost This (Just TapSymbol) {ap = ok})
badAltCostTapSymbol (Present {ok = Oh}) impossible

||| "You may [+1] rather than pay this spell's mana cost."
public export
badAltCostLoyaltySymbol : Unspellable (StaticSpec []) (\ok =>
  AltCost This (Just (LoyaltySymbol (LoyaltyUp 1))) {ap = ok})
badAltCostLoyaltySymbol (Present {ok = Oh}) impossible

||| "Escalate {2}. Choose one or both —"
public export
okEscalateWithModes : Card
okEscalateWithModes =
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [ Macros.keywordCosting "Escalate" (Mana [Macros.generic 2])
       , Spell Nothing (Macros.chooseModes (Range (Just 1) (Just 2))
                  [ Macros.gets (Macros.target Macros.creature)
                                (PtUp (Lit 1)) (PtUp (Lit 1))
                                (Just Macros.untilEndOfTurn)
                  , Macros.gets (Macros.target Macros.creature)
                                (PtDown (Lit 1)) (PtDown (Lit 1))
                                (Just Macros.untilEndOfTurn) ]) ]
       Nothing

||| "Escalate {2}"
public export
badEscalateWithoutModes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [Macros.keywordCosting "Escalate" (Mana [Macros.generic 2])] Nothing {fl = ok})
badEscalateWithoutModes MkCharacteristicsLaws impossible

||| "Entwine {2}"
public export
badEntwineWithoutModes : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Sorcery])
       [Macros.keywordCosting "Entwine" (Mana [Macros.generic 2])] Nothing {fl = ok})
badEntwineWithoutModes MkCharacteristicsLaws impossible

public export
jointCrossAbilityChoice : Card
jointCrossAbilityChoice =
  SingleFaced
    (MkFace (MkCharacteristics "Joint choice witness" Nothing [Color] []
      (MkTypeLine [creatureType "Shapeshifter"] [Creature])
      [ Static (Gains Macros.thisCreature
                 (Macros.keywordQuality "Protection" (Macros.ofChosen Color)))
      , Static (Macros.entersChoosing Macros.thisCreature Color)
      ]
      (Macros.printedBox (Just (1, 1)))))
    {fl = MkCharacteristicsLaws}

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
badCumulativeUpkeepOnSpell MkCharacteristicsLaws impossible

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

||| "your commander"
public export
okPossessedCommander : Noun [] Object
okPossessedCommander = Designated CommanderD You

||| "your monarch"
public export
badPossessedMonarch : Unspellable (Noun [] Object) (\ok =>
  Designated Monarch You {sc = ok})
badPossessedMonarch Refl impossible

||| "Unearth {B}"
public export
okCostedUnearth : Ability
okCostedUnearth =
  KeywordAbility "Unearth" (Just (ParamCost (Mana [Macros.pip Black]))) Nothing

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

||| "Unearth {B}" on a creature card
public export
okUnearthOnPermanentCard : Card
okUnearthOnPermanentCard =
  Macros.card "" (Just [Macros.pip Black]) []
       (MkTypeLine [creatureType "Zombie"] [Creature])
       [KeywordAbility "Unearth" (Just (ParamCost (Mana [Macros.pip Black]))) Nothing]
       (Just (2, 2))

||| "Unearth {B}"
public export
badUnearthOnSpellCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Unearth" (Just (ParamCost (Mana [Macros.pip Black]))) Nothing] Nothing {fl = ok})
badUnearthOnSpellCard MkCharacteristicsLaws impossible

||| "Flashback {2}{U}"
public export
badFlashbackOnPermanentCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [KeywordAbility "Flashback" (Just (ParamCost (Mana [Macros.generic 2, Macros.pip Blue]))) Nothing]
       Nothing {fl = ok})
badFlashbackOnPermanentCard MkCharacteristicsLaws impossible

||| "for each attacking creature you control. You gain that much life."
public export
okThatMuchAfterQuantity : Instruction []
okThatMuchAfterQuantity =
  Sequentially [ Macros.losesLife (Macros.target Opponent)
                   (Macros.forEach 1 (And [Attacking, Macros.creature,
                                           HasPossessor ControllerAx You]))
               , Macros.gainsLife You ThatMuch ]

||| "Flip a coin. Draw that many cards."
public export
badThatMuchAfterFlip : Unspellable (Instruction []) (\ok =>
  Sequentially [(Macros.flipCoins You 1), Draw You (ThatMuch {ok})])
badThatMuchAfterFlip Refl impossible

||| "Flip a coin. If you win the flip, draw a card."
public export
okFlipArmAfterFlip : Instruction []
okFlipArmAfterFlip =
  Sequentially [ (Macros.flipCoins You 1)
               , If (FlipCalled You WinsFlip) (Draw You (Lit 1)) Nothing ]

||| "If you win the flip, draw a card."
public export
badFlipArmWithoutFlip : Unspellable (Instruction []) (\ok =>
  If (FlipCalled You WinsFlip {fl = ok}) (Draw You (Lit 1)) Nothing)
badFlipArmWithoutFlip Oh impossible

||| "Roll a d6."
public export
okSixSidedDie : Instruction []
okSixSidedDie = RollDice You (Lit 1) (SidesOf 6)

||| "Roll a d0."
public export
badNoughtSidedDie : Unspellable (Instruction []) (\ok =>
  RollDice You (Lit 1) (SidesOf 0 {nz = ok}))
badNoughtSidedDie ItIsSucc impossible

||| a planeswalker card printing its starting loyalty
public export
okPlaneswalkerLoyaltyBox : Card
okPlaneswalkerLoyaltyBox =
  Macros.cardOf "" (Just [Macros.pip Blue]) [Legendary]
       (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [] (Macros.loyaltyBox 3)

||| a planeswalker card printed with no starting loyalty
public export
badPlaneswalkerNoLoyalty : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary] (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [] Nothing {fl = ok})
badPlaneswalkerNoLoyalty (MkCharacteristicsLaws {bx = MkCardBox}) impossible

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
badBattleNoDefense (MkCharacteristicsLaws {bx = MkCardBox}) impossible

||| an adventurer card whose inset frame is a named Adventure sorcery
public export
okNamedAdventure : Card
okNamedAdventure =
  Adventurer (Macros.frontFace "" (Just [Macros.pip Blue]) []
                               (MkTypeLine [] [Creature]) []
                               (Macros.printedBox (Just (1, 1))))
             (Macros.alternative "" (Just [Macros.pip Blue]) []
                                 (MkTypeLine [spellType "Adventure"] [Sorcery])
                                 [] Nothing)

||| an adventurer card whose inset frame is a plain instant, naming no Adventure
public export
badUnnamedAdventure : Unspellable Card (\ok =>
  Adventurer (Macros.frontFace "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature]) []
                               (Macros.printedBox (Just (1, 1))))
             (Macros.alternative "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant]) []
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
           (Macros.alternative "" Nothing [] (MkTypeLine [] [Creature]) []
                               (Macros.printedBox (Just (2, 2))))

||| a flip card whose upside-down half is an instant
public export
badSpellFlipHalf : Unspellable Card (\ok =>
  FlipCard (Macros.frontFace "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) []
                             (Macros.printedBox (Just (1, 1))))
           (Macros.alternative "" Nothing [] (MkTypeLine [] [Instant]) [] Nothing)
           {ah = ok})
badSpellFlipHalf Oh impossible

failing "Mismatch between: CardFace and Characteristics"
  ||| a flip card's upside-down half spelled as a back face -- a flip card's back is the normal card back, so the half is alternative characteristics, not a second face [CR#710.1]
  badFaceAsFlipAlternative : Card
  badFaceAsFlipAlternative =
    FlipCard (Macros.frontFace "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) []
                               (Macros.printedBox (Just (1, 1))))
             (Macros.backFace "" [] (MkTypeLine [] [Creature]) []
                              (Macros.printedBox (Just (2, 2))))

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
               (MkFace (MkCharacteristics "" (Just [Macros.pip Green]) [] []
                                          (MkTypeLine [] [Creature]) []
                                          (Macros.printedBox (Just (1, 1)))))
               {bf = ok})
badTransformingBackWithCost MkCharacteristicsLaws impossible

||| "your devotion to black"
public export
okSingularDevotion : Amount []
okSingularDevotion = Devotion You (LitColor Black) Nothing

||| "your opponents' devotion to black"
public export
badPluralDevotion : Unspellable (Amount []) (\ok =>
  Devotion (PlayerGroup YourOpponents) (LitColor Black) Nothing {one = ok})
badPluralDevotion Refl impossible

||| "Flip a coin. Take an extra turn for each coin that comes up heads."
public export
okCoinsShowingAfterFlip : Instruction []
okCoinsShowingAfterFlip =
  Sequentially [ (Macros.flipCoins You 1)
               , ExtraTurn You (CoinsShowing Heads) ]

||| "Take an extra turn for each coin that comes up heads."
public export
badCoinsShowingWithoutFlip : Unspellable (Instruction []) (\ok =>
  ExtraTurn You (CoinsShowing Heads {fl = ok}))
badCoinsShowingWithoutFlip Oh impossible

||| "Transform target creature."
public export
okTurnOverOnField : Instruction []
okTurnOverOnField = TurnOver (Macros.target Macros.creature)

||| "Transform target creature card in your graveyard."
public export
badTurnOverOffField : Unspellable (Instruction []) (\ok =>
  TurnOver (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
           {ok})
badTurnOverOffField Oh impossible

||| "unlock a locked door of a Room you control"
public export
okUnlockRoomDoor : Instruction []
okUnlockRoomDoor =
  Unlock (DoorOf (Just Locked)
            (Described (TargetDet (Macros.upTo 1))
               (And [HasSubtype (enchantmentType "Room"),
                     HasPossessor ControllerAx You])))

||| "unlock this door"
public export
badUnlockThisDoor : Unspellable (Instruction []) (\ok =>
  Unlock ThisDoor {nh = ok})
badUnlockThisDoor Oh impossible

||| "unlock a locked door of a Room card in your graveyard"
public export
badUnlockDoorOffBattlefield : Unspellable (Instruction []) (\ok =>
  Unlock (DoorOf (Just Locked)
            (Macros.a (And [HasSubtype (enchantmentType "Room"),
                            InZone Macros.graveyardZ])) {zn = ok}))
badUnlockDoorOffBattlefield Oh impossible

public export
badDoorOfBareThis : Unspellable (Instruction []) (\ok =>
  Unlock (DoorOf (Just Locked) This {zn = ok}))
badDoorOfBareThis Oh impossible

||| "When you unlock this door, this Room deals 1 damage to each opponent."
||| -- a door header belongs to a Room's shared line.
public export
okRoomDoorHeaderOnSharedLine : Card
okRoomDoorHeaderOnSharedLine =
  SharedLineSplit (MkTypeLine [enchantmentType "Room"] [Enchantment]) [] Nothing
    (MkSharedHalf "" (Just [Macros.pip Red])
       [ Macros.triggered When (UnlocksDoor You ThisDoor)
           (DealDamage Macros.thisRoom (Lit 1) (Macros.each Opponent)) ])
    (MkSharedHalf "" (Just [Macros.generic 3, Macros.pip Red])
       [ Macros.triggered When (UnlocksDoor You ThisDoor)
           (DealDamage Macros.thisRoom (Lit 2) (Macros.each Opponent)) ])

||| "When you unlock this door, this Room deals 1 damage to each opponent."
public export
badDoorHeaderOffSharedLine : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Red]) []
       (MkTypeLine [enchantmentType "Room"] [Enchantment])
       [ Macros.triggered When (UnlocksDoor You ThisDoor)
           (DealDamage Macros.thisRoom (Lit 1) (Macros.each Opponent)) ]
       Nothing {fl = ok})
badDoorHeaderOffSharedLine MkCharacteristicsLaws impossible

public export
generalManaSymbolMatcher : Predicate [] Object
generalManaSymbolMatcher = ManaCostHas (Simple (Specific (OfColor Red)))

public export
playerItRead : Noun [MkBinding AD Player OneOf (PlayerP False)] Player
playerItRead = They

public export
delayedDoorTraversal :
  effectNamesThisDoor
    (Delayed (UnlocksDoor You ThisDoor) [] Nothing (Draw You (Lit 1))
             {so = Absent}) = True
delayedDoorTraversal = Refl

public export
distributiveGroupSurvives : Instruction []
distributiveGroupSurvives =
  Sequentially
    [ Enact (Just (Macros.each Opponent)) "Shuffle" (Shuffle They)
    , ChangeLife (Macros.That PlayerW ManyOf) (LifeDown (Lit 1))
    ]

public export
secondChooserDevotionRead :
  Amount [qualityB Color, qualityB Color]
secondChooserDevotionRead =
  Devotion You Macros.theLastChosenColor Nothing

public export
emblemGrantorRead : Noun [] Object
emblemGrantorRead = TheGrantor EmblemMarker

public export
alternativeCostReadbacks : List (Amount [])
alternativeCostReadbacks =
  [ Macros.paidCostRead (ByKeyword "Escape") Nothing This
  , Macros.paidCostRead (ByKeyword "Foretell") Nothing This
  , Macros.paidCostRead (ByKeyword "Bestow") Nothing This
  , Macros.paidCostRead (ByKeyword "Disguise") Nothing This
  , Macros.paidCostRead (ByKeyword "Mutate") Nothing This
  , Macros.paidCostRead (ByKeyword "Overload") Nothing This
  , Macros.paidCostRead (ByKeyword "Disturb") Nothing This
  , Macros.paidCostRead (ByKeyword "Dash") Nothing This
  , Macros.paidCostRead (ByKeyword "Evoke") Nothing This
  , Macros.paidCostRead (ByKeyword "Blitz") Nothing This
  , Macros.paidCostRead (ByKeyword "Cleave") Nothing This
  , Macros.paidCostRead (ByKeyword "Harmonize") Nothing This
  , Macros.paidCostRead (ByKeyword "Impending") Nothing This
  , Macros.paidCostRead (ByKeyword "Awaken") Nothing This
  , Macros.paidCostRead (ByKeyword "Buyback") Nothing This
  , Macros.paidCostRead (ByKeyword "Casualty") Nothing This
  , Macros.paidCostRead (ByKeyword "Squad") Nothing This
  , Macros.paidCostRead (ByKeyword "Offspring") Nothing This
  , Macros.paidCostRead (ByKeyword "Gift") Nothing This
  , Macros.paidCostRead (ByKeyword "Replicate") Nothing This
  ]

public export
modalCostReadbacks : (Amount [], Amount [])
modalCostReadbacks =
  ( Macros.paidCostRead (ByKeyword "Entwine") Nothing This
  , Macros.timesPaid (ByKeyword "Escalate") This
  )

||| "It doesn't untap during its controller's next untap step."
public export
okUntapNextSingleIt : Instruction []
okUntapNextSingleIt =
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                DoesntUntapNext (Macros.It OneOf) (Lit 1)]

public export
badUntapNextAmbiguousIt : Unspellable (Instruction []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
                SetStatus Tapped (Macros.target Macros.artifact),
                DoesntUntapNext ((Macros.It OneOf) {ok = ok}) (Lit 1)])
badUntapNextAmbiguousIt Refl impossible

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

||| "Roll a d20. 1—9 | Draw a card."
public export
okResultsTableAfterRoll : Instruction []
okResultsTableAfterRoll =
  Sequentially [ (Macros.rollDice You 1 20)
               , ResultsTable [MkRollRow (Macros.fromTo 1 9)
                                         (Draw You (Lit 1))] ]

||| "1—9 | Draw a card."
public export
badTableWithoutRoll : Unspellable (Instruction []) (\ok =>
  ResultsTable [Macros.rollRow (Macros.fromTo 1 9) (Draw You (Lit 1))] {ok})
badTableWithoutRoll Refl impossible

||| "LEVEL 1-2 [2/3]" beside "LEVEL 3+ [2/4]" on a 2/2 creature
public export
okLevelerBands : Card
okLevelerBands =
  Macros.leveler "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Creature]) []
       (Just (2, 2))
       [ Macros.levelBand (LevelBetween 1 2) 2 3 []
       , Macros.levelBand (LevelAtLeast 3) 2 4 [] ]

||| "LEVEL 4-2 [2/3]" -- no number of level counters is at least 4 and at most 2 [CR#711.2a]
public export
badEmptyLevelRange : Unspellable Card (\ok =>
  Macros.leveler "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Creature]) []
       (Just (2, 2)) [ Macros.levelBand (LevelBetween 4 2) 2 3 [] ] {bl = ok})
badEmptyLevelRange (AndBand {hd = MkLevelBandLaws {rg = Oh}}) impossible

||| "LEVEL 1-4 [2/3]" beside "LEVEL 3+ [2/4]" -- level 3 falls in both, each setting base power and toughness [CR#711.2a,711.2b]
public export
badOverlappingLevelBands : Unspellable Card (\ok =>
  Macros.leveler "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Creature]) []
       (Just (2, 2))
       [ Macros.levelBand (LevelBetween 1 4) 2 3 []
       , Macros.levelBand (LevelAtLeast 3) 2 4 [] ] {dj = ok})
badOverlappingLevelBands Oh impossible

||| "LEVEL 1+ [2/2]" printed on a sorcery -- no striated text box to level [CR#711.1]
public export
badLevelBandOffLevelerFrame : Unspellable Card (\ok =>
  Macros.leveler "" (Just [Macros.pip Red]) [] (MkTypeLine [] [Sorcery]) [] Nothing
       [ Macros.levelBand (LevelAtLeast 1) 2 2 [] ] {lv = ok})
badLevelBandOffLevelerFrame Oh impossible

||| "Prototype {1}{R} — 1/1" on a 2/2 artifact creature
public export
okPrototypeAlt : Card
okPrototypeAlt =
  Macros.prototype "" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact, Creature]) [] (Just (2, 2))
       (Macros.prototypeAlt [Macros.generic 1, Macros.pip Red] 1 1)

||| "Prototype — 1/1" -- the inset frame's second set includes a mana cost, and a prototyped spell is cast using only that one [CR#702.160a,718.3a]
public export
badPrototypeWithoutAltCost : Unspellable Card (\ok =>
  Macros.prototype "" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact, Creature]) [] (Just (2, 2))
       (Macros.prototypeAlt [] 1 1) {al = ok})
badPrototypeWithoutAltCost (MkPrototypeAltLaws {mc = IsNonEmpty}) impossible

||| "Prototype {1}{R} — loyalty 3" -- the inset frame's second set is power and toughness [CR#702.160a,718.1]
public export
badPrototypeAltLoyaltyBox : Unspellable Card (\ok =>
  Macros.prototype "" (Just [Macros.generic 2]) []
       (MkTypeLine [] [Artifact, Creature]) [] (Just (2, 2))
       (MkPrototypeAlt [Macros.generic 1, Macros.pip Red]
                       (LoyaltyBox (PrintedNum 3))) {al = ok})
badPrototypeAltLoyaltyBox (MkPrototypeAltLaws {bx = MkCardBox {ok = Oh}}) impossible

||| "Prototype {1}{R} — 1/1" printed on a noncreature artifact -- no first set of power and toughness for the inset frame to give a second [CR#718.1,718.2]
public export
badPrototypeOffCreatureFrame : Unspellable Card (\ok =>
  Macros.prototype "" (Just [Macros.generic 2]) [] (MkTypeLine [] [Artifact]) []
       Nothing (Macros.prototypeAlt [Macros.generic 1, Macros.pip Red] 1 1)
       {pf = ok})
badPrototypeOffCreatureFrame Oh impossible

||| "When you cast this spell, copy it for each other spell that was cast
||| before it this turn." Storm counts the spells cast earlier in the turn
||| [CR#702.40a], not every spell cast during it.
public export
stormCountsEarlierThisTurn :
  Macros.stormExpansion =
    Macros.triggered When (Casts You Macros.thisSpell Nothing)
      (Sequentially
         [ Copy FromStack You Macros.thisSpell
             (Macros.eventCountInvolving SpellCast (Macros.a AnyPlayer) EarlierThisTurn
                (Macros.a (And [Macros.spell, OtherThan Macros.thisSpell])))
             []
         , Macros.may You (ChooseNewTargets (Pro (Word CopyW) ManyOf Whole)) ])
stormCountsEarlierThisTurn = Refl
