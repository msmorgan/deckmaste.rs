module Experimental.ProofsAnaphora

import Experimental
import Experimental.Macros
import Experimental.Unspellable
import Data.List.Elem

%default total

%unbound_implicits off


||| "Choose two — Draw a card; draw a card." [CR#700.2d]
public export
identicalModesAllowed : Instruction []
identicalModesAllowed =
  Macros.chooseModes (Macros.exactly 2) [(Draw You (Lit 1)), (Draw You (Lit 1))]

||| "Choose a player or planeswalker."
public export
okChoosePlayerOrPlaneswalker : Instruction []
okChoosePlayerOrPlaneswalker =
  Choose Nothing Nothing (Macros.a (Macros.kindJoin AnyPlayer (HasType Planeswalker)))
         Openly

||| "Choose you."
public export
badChooseYou : Unspellable (Instruction []) (\ok =>
  Choose Nothing Nothing You Openly {ch = ok})
badChooseYou BareChoice impossible

public export
badConditionalArmAntecedent : Unspellable (Instruction []) (\ok =>
  Sequentially [OnlyIf (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))
                   (Macros.exists Macros.creatureYouControl)
                   Nothing,
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf) {ok})])
badConditionalArmAntecedent Refl impossible

public export
badBothArmsAntecedent : Unspellable (Instruction []) (\ok =>
  Sequentially [May You (Macros.gainsLife You (Lit 1))
                     (Just (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"])))
                     (Just (Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [creatureType "Soldier"]))),
                PutCounters (Lit 1) (PrintedKind Macros.plusOnePlusOne) ((Macros.It OneOf) {ok})])
badBothArmsAntecedent Refl impossible

||| "Put target creature onto the battlefield."
public export
okMoveToBattlefield : Instruction []
okMoveToBattlefield =
  Move (Macros.target Macros.creature) (ZoneAt Battlefield BareScope) []

||| "Put target creature into your library."
public export
badMoveToBareLibrary : Unspellable (Instruction []) (\ok =>
  Move (Macros.target Macros.creature) (ZoneAt Library BareScope) [] {ok})
badMoveToBareLibrary BattlefieldOk impossible

||| "Look at the top four cards of your library. You choose one of them."
public export
okAgentChoiceOfSome : Instruction []
okAgentChoiceOfSome =
  Sequentially [Macros.lookAt (Macros.topSlice (Lit 4)),
                Choose Nothing (Just You) (Macros.someOf (Macros.exactly 1) (Macros.It ManyOf)) Openly]

||| "Look at the top four cards of your library. Choose one of them."
public export
badChooseSomeOf : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.lookAt ((Macros.topSlice (Lit 4))), Choose Nothing Nothing (Macros.someOf (Macros.exactly 1) ((Macros.It ManyOf))) Openly {ch = ok}])
badChooseSomeOf BareChoice impossible

||| "Exile target creature."
public export
okMoveToExile : Instruction []
okMoveToExile = Move (Macros.target Macros.creature) Macros.exileZ []

||| "Put target creature onto the stack."
public export
badMoveToStack : Unspellable (Instruction []) (\ok =>
  Move (Macros.target Macros.creature) (ZoneAt Stack BareScope) [] {ok})
badMoveToStack BattlefieldOk impossible

||| "When this creature enters, if a creature died this turn, draw a card."
public export
okLookbackObjectDied : Ability
okLookbackObjectDied =
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing []
            Nothing Nothing
            (Just (Happened (Macros.a Macros.creature) (MkLookback Death Lookback.ThisTurn Nothing)))
            (Draw You (Lit 1))

||| "When this creature enters, if you died this turn, draw a card."
public export
badLookbackPlayerDied : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Happened You (MkLookback Death Lookback.ThisTurn Nothing {sb = ok}))) (Draw You (Lit 1)))
badLookbackPlayerDied MkLookbackSubject impossible

public export
badLookbackObjectCast : Unspellable Ability (\ok =>
  Triggered When (Enters Macros.thisCreature Nothing) [] Nothing [] Nothing Nothing (Just (Happened (Macros.a Macros.creature) (MkLookback SpellCast Lookback.ThisTurn Nothing {sb = ok}))) (Draw You (Lit 1)))
badLookbackObjectCast MkLookbackSubject impossible

||| "target creature that entered this turn"
public export
okHappenedToObjectEntry : Noun [] Object
okHappenedToObjectEntry =
  Macros.target (And [Macros.creature,
                      HappenedTo (MkLookback Entry Lookback.ThisTurn Nothing)])

||| "target creature who cast a spell this turn"
public export
badHappenedToObjectCast : Unspellable (Noun [] Object) (\ok =>
  Macros.target (And [Macros.creature, HappenedTo (MkLookback SpellCast Lookback.ThisTurn Nothing {sb = ok})]))
badHappenedToObjectCast MkLookbackSubject impossible

||| "each opponent who died this turn"
public export
badHappenedToPlayerDied : Unspellable (Noun [] Player) (\ok =>
  Macros.each (And [Opponent, HappenedTo (MkLookback Death Lookback.ThisTurn Nothing {sb = ok})]))
badHappenedToPlayerDied MkLookbackSubject impossible

||| "each opponent who a state matched this turn"
public export
badStateMatchLookback : Unspellable (Noun [] Player) (\ok =>
  Macros.each (And [Opponent, HappenedTo (MkLookback StateMatch Lookback.ThisTurn Nothing {sb = ok})]))
badStateMatchLookback MkLookbackSubject impossible

||| "Choose a creature you control."
public export
okChooseIndefinite : Instruction []
okChooseIndefinite = Choose Nothing Nothing (Macros.a Macros.creatureYouControl) Openly

||| "Choose the creature with the least toughness among creatures you control."
public export
badChooseDefinite : Unspellable (Instruction []) (\ok =>
  Choose Nothing Nothing (Macros.the (And [Macros.creature,
                         Superlative MinOf (StatAxis Toughness)
                                     Macros.creatureYouControl])) Openly {ch = ok})
badChooseDefinite BareChoice impossible

||| "This deals 1 damage to that permanent or player."
public export
okUnionAnaphorAfterJoin : Instruction []
okUnionAnaphorAfterJoin =
  Sequentially [ DealDamage This (Lit 3)
                   (Macros.target (Macros.kindJoin AnyPlayer Macros.creature))
               , DealDamage This (Lit 1) (Macros.That JoinW OneOf) ]

||| "This deals 3 damage to that permanent or player."
public export
badUnionAnaphorNoAntecedent : Unspellable (Instruction []) (\ok =>
  DealDamage This (Lit 3) (Macros.That JoinW OneOf {ok = ok}))
badUnionAnaphorNoAntecedent Refl impossible

||| "Destroy target creature. This deals 3 damage to that permanent or player."
public export
badUnionAnaphorOnObject : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , DealDamage This (Lit 3) (Macros.That JoinW OneOf {ok = ok}) ])
badUnionAnaphorOnObject Refl impossible

||| "Counter target activated ability. Counter that spell or ability." An
||| ability on the stack is an object in the stack zone [CR#109.1,113.1c,405.1],
||| so the one stack read reaches it.
public export
okStackAnaphorOnAbility : Instruction []
okStackAnaphorOnAbility =
  Sequentially [ CounterSpell (Macros.target (AbilityHead AnyActivated))
               , CounterSpell (Macros.That StackW OneOf) ]

public export
afterAnyTargetDamage : Bindings
afterAnyTargetDamage =
  instrIntro (the (Instruction [])
    (DealDamage This (Lit 3) (Macros.target Macros.anyTarget)))

||| "This deals 3 damage to any target. Counter that spell or ability."
||| Refused: an any-target union is not on the stack. `That JoinW` spells the
||| permanent-or-player read (`okUnionAnaphorAfterJoin`).
public export
badStackAnaphorOnPlayerUnion :
  Unspellable (Noun ProofsAnaphora.afterAnyTargetDamage Object) (\ok =>
    Macros.That StackW OneOf {ok = ok})
badStackAnaphorOnPlayerUnion Refl impossible

||| "a creature with protection from a color"
public export
okKeywordClassWithSort : Predicate [] Object
okKeywordClassWithSort =
  HasKeyword (AnyKeywordIn (MkKeywordFamily "Protection" (Just Color)))

||| "a creature with flyings"
public export
badClassOfParamlessKeyword : Unspellable (Predicate [] Object) (\ok =>
  HasKeyword (AnyKeywordIn (MkKeywordFamily "Flying" Nothing)) {kn = ok})
badClassOfParamlessKeyword KeywordTermInFactsTable impossible

||| "a creature with renown of any color"
public export
badSortedClassOnNumberKeyword : Unspellable (Predicate [] Object) (\ok =>
  HasKeyword (AnyKeywordIn (MkKeywordFamily "Renown" (Just Color))) {kn = ok})
badSortedClassOnNumberKeyword KeywordTermInFactsTable impossible

||| "{T}: Draw a card. Activate only if you created this turn."
public export
badBareTokenCreationLookback : Unspellable (Condition []) (\ok =>
  Happened You (MkLookback TokenCreation Lookback.ThisTurn Nothing {cw = LeftBare {ok = ok}}))
badBareTokenCreationLookback Oh impossible

||| "creature that was dealt damage by this creature this turn"
public export
okDamageTakenComplement : Predicate [] Object
okDamageTakenComplement =
  HappenedTo (MkLookback DamageTaken Lookback.ThisTurn (Just (Involving Macros.thisCreature)))

||| "Destroy target creature that attacked with this creature this turn."
public export
badAttackerComplementOnObject : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo (MkLookback AttackDeclaration Lookback.ThisTurn (Just (Involving Macros.thisCreature {cp = ok}))))
badAttackerComplementOnObject MkLookbackComplement impossible

public export
badPlayerCastComplement : Unspellable (Condition []) (\ok =>
  Happened You (MkLookback SpellCast Lookback.ThisTurn (Just (Involving Macros.anOpponent {cp = ok}))))
badPlayerCastComplement MkLookbackComplement impossible

||| "target spell cast from your graveyard"
public export
okCastFromGraveyard : Predicate [] Object
okCastFromGraveyard = CastFrom (Macros.graveyardOf You)

||| "Counter target spell cast from the stack."
public export
badCastFromStack : Unspellable (Predicate [] Object) (\ok =>
  CastFrom (ZoneAt Stack BareScope) {pf = ok})
badCastFromStack Oh impossible

||| "… that was put somewhere from the battlefield this turn."
public export
okPlacementOriginBattlefield : Predicate [] Object
okPlacementOriginBattlefield =
  HappenedTo (MkLookback Placement Lookback.ThisTurn (Just (FromZones (FromZone [Macros.battlefieldZ]) Nothing)))

||| "… that died from the battlefield this turn."
public export
badDeathOriginZone : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo (MkLookback Death Lookback.ThisTurn (Just (FromZones (FromZone [Macros.battlefieldZ]) Nothing {ok = ok}))))
badDeathOriginZone Oh impossible

||| "if you've cast a spell from the stack this turn"
public export
badCastOriginFromStack : Unspellable (Condition []) (\ok =>
  Happened You (MkLookback SpellCast Lookback.ThisTurn (Just (FromZones (FromZone [ZoneAt Stack BareScope]) Nothing {ok = ok}))))
badCastOriginFromStack Oh impossible

||| "if you've cast a spell from this turn"
public export
badEmptyOriginCoordination : Unspellable (Condition []) (\ok =>
  Happened You (MkLookback SpellCast Lookback.ThisTurn (Just (FromZones (FromZone []) Nothing {ok = ok}))))
badEmptyOriginCoordination Oh impossible

||| "if you've cast a creature spell from your hand this turn"
public export
okOriginPayloadInvolving : Condition []
okOriginPayloadInvolving =
  Happened You (MkLookback SpellCast Lookback.ThisTurn (Just (FromZones (FromZone [Macros.handZ])
                    (Just (Involving (Macros.a Macros.creature))))))

||| "if you've cast a spell from your hand from the command zone this turn"
public export
badNestedOriginPayload : Unspellable (Condition []) (\ok =>
  Happened You (MkLookback SpellCast Lookback.ThisTurn (Just (FromZones (FromZone [Macros.handZ])
                    (Just (FromZones (FromZone [Macros.commandZ]) Nothing)) {pl = ok}))))
badNestedOriginPayload Oh impossible

||| "if you've cast a spell from anywhere other than this turn"
public export
badEmptyOriginExclusion : Unspellable (Condition []) (\ok =>
  Happened You (MkLookback SpellCast Lookback.ThisTurn (Just (FromZones (FromAnywhereBut []) Nothing {ok = ok}))))
badEmptyOriginExclusion Oh impossible

||| "… that died from anywhere this turn."
public export
badDeathOriginAnywhere : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo (MkLookback Death Lookback.ThisTurn (Just (FromZones FromAnywhere Nothing {ok = ok}))))
badDeathOriginAnywhere Oh impossible

||| "… that was put into a graveyard from the battlefield this turn."
public export
okPlacementIntoGraveyard : Predicate [] Object
okPlacementIntoGraveyard =
  HappenedTo (MkLookback Placement Lookback.ThisTurn (Just (IntoZone Macros.graveyardZ
                      (Just (FromZones (FromZone [Macros.battlefieldZ])
                                       Nothing)))))

||| "… that was put into the battlefield this turn."
public export
badPlacementIntoBattlefield : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo (MkLookback Placement Lookback.ThisTurn (Just (IntoZone Macros.battlefieldZ Nothing {ok = ok}))))
badPlacementIntoBattlefield Oh impossible

||| "… that was put into your graveyard into exile this turn."
public export
badNestedDestination : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo (MkLookback Placement Lookback.ThisTurn (Just (IntoZone (Macros.graveyardOf You)
                      (Just (IntoZone Macros.exileZ Nothing)) {pl = ok}))))
badNestedDestination Oh impossible

||| "if you shuffled your library this way"
public export
okShuffleLocusAtLibrary : Condition []
okShuffleLocusAtLibrary =
  Happened You (MkLookback (VerbedAct "Shuffle") Lookback.ThisWay (Just (AtZone Macros.yourLibrary)))

||| "if a creature died in your graveyard this way"
public export
badLocusOnDeath : Unspellable (Condition []) (\ok =>
  Happened (Macros.a Macros.creature) (MkLookback Death Lookback.ThisWay (Just (AtZone (Macros.graveyardOf You) {ok = ok}))))
badLocusOnDeath Oh impossible

||| "if you searched this way, shuffle"
public export
badBareSearchLookback : Unspellable (Condition []) (\ok =>
  Happened You (MkLookback (VerbedAct "Search") Lookback.ThisWay Nothing {cw = LeftBare {ok}}))
badBareSearchLookback Oh impossible

||| "if you shuffled your graveyard this way"
public export
badShuffleLocusAtGraveyard : Unspellable (Condition []) (\ok =>
  Happened You (MkLookback (VerbedAct "Shuffle") Lookback.ThisWay (Just (AtZone (Macros.graveyardOf You) {ok = ok}))))
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
  HappenedTo (MkLookback Entry Lookback.ThisTurn (Just (FromZones (FromZone [Macros.battlefieldZ]) Nothing {ok = ok}))))
badEntryOriginBattlefield Oh impossible

||| "For each opponent, you draw a card."
public export
okPluralForEach : Instruction []
okPluralForEach = ForEachOf (Macros.each Opponent) (Draw You (Lit 1))

||| "For each of target creature, its controller draws a card."
public export
badSingletonForEach : Unspellable (Instruction []) (\ok =>
  ForEachOf (Macros.target Macros.creature) (Draw You (Lit 1)) {pl = ok})
badSingletonForEach Refl impossible

||| "if you activated a loyalty ability this turn"
public export
badBareActivationLookback : Unspellable (Condition []) (\ok =>
  Happened You (MkLookback AbilityActivation Lookback.ThisTurn Nothing {cw = LeftBare {ok = ok}}))
badBareActivationLookback Oh impossible

||| "on top of your library in any order, third from the top"
public export
okOneEndArrangedOrdinal : ZoneExpr []
okOneEndArrangedOrdinal =
  LibraryAt (OneEnd OnTop) (Just AnyOrder) (Just (Nth 3)) BareScope

||| "Put those cards on the top or bottom of your library in any order."
public export
badDisjunctionOrdered : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt (EitherEnd Nothing) (Just AnyOrder) Nothing {af = ok} BareScope)
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
  LibraryAt Shuffled (Just AnyOrder) Nothing {af = ok} BareScope)
badShuffledArranged Oh impossible

||| "Shuffle it into its owner's library third from the top."
public export
badShuffledOrdinal : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt Shuffled Nothing (Just (Nth 3)) {nf = ok} BareScope)
badShuffledOrdinal Oh impossible

public export
copyParticipleUnwritten : actNamesParticiple "Copy" = False
copyParticipleUnwritten = Refl

||| "Copy target instant or sorcery spell twice. You may choose new targets for
||| those spells." A copy of a spell is itself a spell [CR#707.10,112.1a], so
||| the plural spell read reaches the copies.
public export
okPluralSpellReadAfterCopy : Instruction []
okPluralSpellReadAfterCopy =
  Sequentially
    [ Copy FromStack You
        (Macros.target (And [Macros.instantOrSorcery, Macros.spell])) (Lit 2) []
    , Macros.may You (ChooseNewTargets (Macros.That SpellW ManyOf)) ]

||| "Copy target instant or sorcery spell. You may choose new targets for the
||| copy."
public export
okCopyReadAfterCopy : Instruction []
okCopyReadAfterCopy =
  Sequentially
    [ Copy FromStack You
        (Macros.target (And [Macros.instantOrSorcery, Macros.spell])) (Lit 1) []
    , Macros.may You (ChooseNewTargets (Macros.That CopyW OneOf)) ]

||| "Copy target instant or sorcery spell. You may choose new targets for that
||| spell." Refused: the copy is itself a spell [CR#707.10], so the singular
||| spell read reaches the original and the copy alike. `That CopyW` spells the
||| copy (`okCopyReadAfterCopy`).
public export
badSingularSpellReadAfterCopy : Unspellable (Instruction []) (\ok =>
  Sequentially
    [ Copy FromStack You
        (Macros.target (And [Macros.instantOrSorcery, Macros.spell])) (Lit 1) []
    , Macros.may You (ChooseNewTargets (Macros.That SpellW OneOf {ok = ok})) ])
badSingularSpellReadAfterCopy Refl impossible

||| "Copy target activated ability twice. You may choose new targets for those
||| abilities." A copy of an ability is itself an ability [CR#707.10], so the
||| plural ability read reaches the copies.
public export
okPluralAbilityReadAfterCopy : Instruction []
okPluralAbilityReadAfterCopy =
  Sequentially
    [ Copy FromStack You (Macros.target (AbilityHead AnyActivated)) (Lit 2) []
    , Macros.may You (ChooseNewTargets (Macros.That AbilityW ManyOf)) ]

||| "Copy target activated ability. You may choose new targets for that
||| ability." Refused: the copy is itself an ability [CR#707.10], so the
||| singular ability read reaches the original and the copy alike.
||| `That AbilityCopyW` spells the copy (`okAbilityCopyReadAfterCopy`).
public export
badSingularAbilityReadAfterCopy : Unspellable (Instruction []) (\ok =>
  Sequentially
    [ Copy FromStack You (Macros.target (AbilityHead AnyActivated)) (Lit 1) []
    , Macros.may You (ChooseNewTargets (Macros.That AbilityW OneOf {ok = ok})) ])
badSingularAbilityReadAfterCopy Refl impossible

||| "Copy target activated ability. You may choose new targets for the copy."
public export
okAbilityCopyReadAfterCopy : Instruction []
okAbilityCopyReadAfterCopy =
  Sequentially
    [ Copy FromStack You (Macros.target (AbilityHead AnyActivated)) (Lit 1) []
    , Macros.may You (ChooseNewTargets (Macros.That AbilityCopyW OneOf)) ]

||| "Change the target of target spell or ability with a single target." One
||| object [CR#109.1]: the reading is one stack-zone payload, not a join.
public export
spellOrAbilityPayload : Payload Object
spellOrAbilityPayload = ObjectP Nothing (Just Stack) Nothing Nothing Nothing

||| An ability goes on the stack with no card associated with it [CR#405.1].
public export
abilityIsOnTheStack : payloadZone (AbilityP Nothing) = Just Stack
abilityIsOnTheStack = Refl

public export
joinedCreatureTy :
  tyOfReach (Word JoinW) OneOf (instrIntro {bs = []}
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

||| "If a player is dealt damage this way, you draw a card."
public export
okDealtThisWayAfterDamage : Instruction []
okDealtThisWayAfterDamage =
  Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
               , If (DealtThisWay AnyPlayer) (Draw You (Lit 1)) Nothing ]

||| "You draw a card. If a player is dealt damage this way, you draw a card."
public export
badDealtThisWayNoDamage : Unspellable (Instruction []) (\ok =>
  Sequentially [ Draw You (Lit 1)
               , If (DealtThisWay AnyPlayer {wy = ok}) (Draw You (Lit 1)) Nothing ])
badDealtThisWayNoDamage Oh impossible

||| "This deals 2 damage to any target. If a mana ability is dealt damage this
||| way, draw a card." Refused: what the description seeds is on the stack, and
||| nothing on the stack is dealt damage. `DealtThisWay AnyPlayer` spells the
||| recipient read (`okDealtThisWayAfterDamage`).
public export
badDealtThisWayAbility : Unspellable (Instruction []) (\ok =>
  Sequentially [ DealDamage This (Lit 2) (Macros.target Macros.anyTarget)
               , If (DealtThisWay IsManaAbility {nz = ok}) (Draw You (Lit 1))
                    Nothing ])
badDealtThisWayAbility Oh impossible

||| "the number of creatures that died this turn"
public export
okDeathTally : Amount []
okDeathTally =
  EventTally TallyCount (Macros.a Macros.creature) (MkLookback Death Lookback.ThisTurn Nothing)

||| "the amount of creatures that died this turn"
public export
badDeathSum : Unspellable (Amount []) (\ok =>
  EventTally TallySum (Macros.a Macros.creature) (MkLookback Death Lookback.ThisTurn Nothing) {qm = ok})
badDeathSum Oh impossible

||| "creature that died this turn"
public export
okObjectDeathLookbackSubject : Predicate [] Object
okObjectDeathLookbackSubject = HappenedTo (MkLookback Death Lookback.ThisTurn Nothing)

||| "creature that won a coin flip this turn"
public export
badCreatureWonFlip : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo (MkLookback FlipWin Lookback.ThisTurn Nothing {sb = ok}))
badCreatureWonFlip MkLookbackSubject impossible

||| "the amount of dice you rolled this turn"
public export
badRollAsMagnitude : Unspellable (Amount []) (\ok =>
  EventTally TallySum You (MkLookback DiceRoll Lookback.ThisTurn Nothing) {qm = ok})
badRollAsMagnitude Oh impossible

public export
youWonAFlipThisTurn : Condition []
youWonAFlipThisTurn = Happened You (MkLookback FlipWin Lookback.ThisTurn Nothing)

public export
youRolledADieThisTurn : Condition []
youRolledADieThisTurn = Happened You (MkLookback DiceRoll Lookback.ThisTurn Nothing)

||| "Roll two d20. Ignore the lowest roll."
public export
okIgnoreAfterRoll : Instruction []
okIgnoreAfterRoll =
  Sequentially [ (Macros.rollDice You 2 20)
               , IgnoreOutcomes (IgnoreExtreme LowestRoll) ]

||| "Ignore the lowest roll."
public export
badIgnoreWithoutRoll : Unspellable (Instruction []) (\ok =>
  IgnoreOutcomes (IgnoreExtreme LowestRoll) {ok})
badIgnoreWithoutRoll Oh impossible

public export
afterATwoDieRoll : Bindings
afterATwoDieRoll = instrIntro (the (Instruction []) (Macros.rollDice You 2 6))

||| "if you rolled doubles"
public export
okRolledDoublesAfterRoll : Condition ProofsAnaphora.afterATwoDieRoll
okRolledDoublesAfterRoll = RolledDoubles

||| "If you rolled doubles, sacrifice this creature."
public export
badRolledDoublesWithoutRoll : Unspellable (Condition []) (\ok =>
  RolledDoubles {ok})
badRolledDoublesWithoutRoll Refl impossible

public export
afterACoinFlip : Bindings
afterACoinFlip = instrIntro (the (Instruction []) (Macros.flipCoins You 1))

||| "a player whose coin comes up tails"
public export
okCoinCameUpOnPlayer : Predicate ProofsAnaphora.afterACoinFlip Player
okCoinCameUpOnPlayer = CoinCameUp Tails

||| "the damage whose coin comes up tails". Refused: only objects and players
||| flip coins. `CoinCameUp` on a player spells the coin read
||| (`okCoinCameUpOnPlayer`).
public export
badCoinCameUpOnOutcome :
  Unspellable (Predicate ProofsAnaphora.afterACoinFlip Outcome) (\ok =>
    CoinCameUp Tails {rk = ok})
badCoinCameUpOnOutcome Oh impossible

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
  Unspellable (Instruction ProofsAnaphora.afterACoinFlip) (\ok =>
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
  Happened You (MkLookback CostPayment Lookback.ThisTurn Nothing {sb = ok}))
badBarePaymentLookback MkLookbackSubject impossible

||| "Destroy the rest."
public export
badRestWithoutAPartition : Unspellable (Noun [] Object) (\ok =>
  Macros.theRest Object {ok})
badRestWithoutAPartition Oh impossible

||| "Choose any number of target creatures. Destroy the rest."
public export
badRestAfterTargetChoice : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.choose (Described (TargetDet Macros.anyNumber) Macros.creature)
               , Macros.destroy (Macros.theRest Object {ok}) ])
badRestAfterTargetChoice Oh impossible

public export
youPaidLifeThisTurn : Condition []
youPaidLifeThisTurn = Happened You (MkLookback LifePayment Lookback.ThisTurn Nothing)

public export
afterShuffledLook : Bindings
afterShuffledLook =
  instrIntro (the (Instruction [])
    (Sequentially [ Macros.lookAt (Macros.topSlice (Lit 1)), Macros.shuffle ]))

public export
badReadsShuffledLibraryCard :
  Unspellable (Noun ProofsAnaphora.afterShuffledLook Object) (\ok => Macros.That CardW OneOf {ok})
badReadsShuffledLibraryCard Refl impossible

||| "Whenever you scry, …"
public export
okPatientlessScry : GameEvent []
okPatientlessScry = VerbedEvent (Just You) "Scry" Nothing Nothing False

||| "Whenever you scry a card, …"
public export
badScryPatient : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Scry"
              (Just (Macros.a (InZone (ZoneAt Library BareScope)))) Nothing False
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
  VerbedEvent Nothing "Mill" (Just (Macros.a (InZone (ZoneAt Library BareScope))))
              (Just (HasSubtype (creatureType "Phyrexian"))) False {bc = ok})
badBecomesWithoutIntransitive Oh impossible

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
  Happened You (MkLookback DamageDealing Lookback.ThisTurn Nothing {sb = ok}))
badPlayerDamageDealer MkLookbackSubject impossible

public export
joinedDealerDamageComplement :
  LookbackComplement DamageDealing Object (Object \/ Player)
joinedDealerDamageComplement = MkLookbackComplement

public export
lastChosenPlayerRead :
  Predicate [choiceB PlayerC, choiceB PlayerC] Player
lastChosenPlayerRead = Macros.theLastChosenPlayer

||| "... a chosen player. ... that player."
||| Only a definite description refers to the choice [CR#607.2d].
public export
badIndefiniteChosenPlayerRead : Unspellable (Instruction [choiceB PlayerC]) (\ok =>
  Sequentially [ DealDamage This (Lit 3) (Macros.a Macros.chosenPlayer)
               , DealDamage This (Lit 3) (They {ok = ok}) ])
badIndefiniteChosenPlayerRead Refl impossible

public export
countBy : (Binding -> Bool) -> Bindings -> Nat
countBy p [] = Z
countBy p (b :: bs) = if p b then S (countBy p bs) else countBy p bs

public export
anyBy : (Binding -> Bool) -> Bindings -> Bool
anyBy p [] = False
anyBy p (b :: bs) = p b || anyBy p bs

public export
countBySplit : (p : Binding -> Bool) -> (xs, ys : Bindings) ->
               countBy p (xs ++ ys) = countBy p xs + countBy p ys
countBySplit p [] ys = Refl
countBySplit p (b :: xs) ys with (p b)
  _ | True = cong S (countBySplit p xs ys)
  _ | False = countBySplit p xs ys

public export
anyBySplit : (p : Binding -> Bool) -> (xs, ys : Bindings) ->
             anyBy p (xs ++ ys) = anyBy p xs || anyBy p ys
anyBySplit p [] ys = Refl
anyBySplit p (b :: xs) ys with (p b)
  _ | True = Refl
  _ | False = anyBySplit p xs ys

||| A satisfied count exhibits its antecedent: counting is a fold, so a
public export
countByWitness : (p : Binding -> Bool) -> (bs : Bindings) -> (n : Nat) ->
                 countBy p bs = S n -> (b : Binding ** (Elem b bs, So (p b)))
countByWitness _ [] _ Refl impossible
countByWitness p (b :: bs) n prf with (p b) proof eq
  countByWitness p (b :: bs) n prf | True = (b ** (Here, eqToSo eq))
  countByWitness p (b :: bs) n prf | False =
    let (c ** (el, ok)) = countByWitness p bs n prf in (c ** (There el, ok))

public export
anyByWitness : (p : Binding -> Bool) -> (bs : Bindings) ->
               So (anyBy p bs) -> (b : Binding ** (Elem b bs, So (p b)))
anyByWitness _ [] Oh impossible
anyByWitness p (b :: bs) ok with (p b) proof eq
  anyByWitness p (b :: bs) ok | True = (b ** (Here, eqToSo eq))
  anyByWitness p (b :: bs) ok | False =
    let (c ** (el, ok')) = anyByWitness p bs ok in (c ** (There el, ok'))

||| An empty prefix counts nothing, so no anaphor is writable before its
public export
countByEmpty : (p : Binding -> Bool) -> countBy p [] = Z
countByEmpty p = Refl

public export
oneOfKind : Kind -> Binding -> Bool
oneOfKind k (MkBinding _ j OneOf _) = kindLte k j
oneOfKind k (MkBinding _ _ ManyOf _) = False

public export
countOnesIsFold : (k : Kind) -> (bs : Bindings) ->
                  countOnes k bs = countBy (oneOfKind k) bs
countOnesIsFold k [] = Refl
countOnesIsFold k (MkBinding d j OneOf p :: bs) with (kindLte k j)
  _ | True = cong S (countOnesIsFold k bs)
  _ | False = countOnesIsFold k bs
countOnesIsFold k (MkBinding d j ManyOf p :: bs) = countOnesIsFold k bs

public export
manyOfKind : Kind -> Binding -> Bool
manyOfKind k (MkBinding _ j ManyOf _) = kindLte k j
manyOfKind k (MkBinding _ _ OneOf _) = False

public export
countManysIsFold : (k : Kind) -> (bs : Bindings) ->
                   countManys k bs = countBy (manyOfKind k) bs
countManysIsFold k [] = Refl
countManysIsFold k (MkBinding d j ManyOf p :: bs) with (kindLte k j)
  _ | True = cong S (countManysIsFold k bs)
  _ | False = countManysIsFold k bs
countManysIsFold k (MkBinding d j OneOf p :: bs) = countManysIsFold k bs

public export
anyMany : Binding -> Bool
anyMany (MkBinding _ _ ManyOf _) = True
anyMany (MkBinding _ _ OneOf _) = False

public export
countManysAnyIsFold : (bs : Bindings) -> countManysAny bs = countBy anyMany bs
countManysAnyIsFold [] = Refl
countManysAnyIsFold (MkBinding d j ManyOf p :: bs) = cong S (countManysAnyIsFold bs)
countManysAnyIsFold (MkBinding d j OneOf p :: bs) = countManysAnyIsFold bs

public export
outcomeIs : OutcomeSort -> Binding -> Bool
outcomeIs s (MkBinding _ Outcome OneOf (OutcomeP t)) = s == t
outcomeIs s (MkBinding _ _ _ _) = False

public export
countOutcomesIsFold : (s : OutcomeSort) -> (bs : Bindings) ->
                      countOutcomes s bs = countBy (outcomeIs s) bs
countOutcomesIsFold s [] = Refl
countOutcomesIsFold s (MkBinding d Outcome OneOf (OutcomeP t) :: bs) with (s == t)
  _ | True = cong S (countOutcomesIsFold s bs)
  _ | False = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Outcome ManyOf (OutcomeP t) :: bs) =
  countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Object p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Player p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d (Quality q) p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Gap p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d (LetterK l) p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d TurnRef p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d Pile p pay :: bs) = countOutcomesIsFold s bs
countOutcomesIsFold s (MkBinding d (a \/ b) p pay :: bs) = countOutcomesIsFold s bs

public export
quantOutcome : Binding -> Bool
quantOutcome (MkBinding _ Outcome OneOf (OutcomeP t)) = outcomeIsQuantity t
quantOutcome (MkBinding _ _ _ _) = False

public export
countQuantOutcomesIsFold : (bs : Bindings) ->
                           countQuantOutcomes bs = countBy quantOutcome bs
countQuantOutcomesIsFold [] = Refl
countQuantOutcomesIsFold (MkBinding d Outcome OneOf (OutcomeP t) :: bs)
    with (outcomeIsQuantity t)
  _ | True = cong S (countQuantOutcomesIsFold bs)
  _ | False = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Outcome ManyOf (OutcomeP t) :: bs) =
  countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Object p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Player p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d (Quality q) p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Gap p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d (LetterK l) p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d TurnRef p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d Pile p pay :: bs) = countQuantOutcomesIsFold bs
countQuantOutcomesIsFold (MkBinding d (a \/ b) p pay :: bs) = countQuantOutcomesIsFold bs

public export
countQualityIsCountOnes : (q : QualitySort) -> (bs : Bindings) ->
                          countChoice (QSort q) bs = countOnes (Quality q) bs
countQualityIsCountOnes q [] = Refl
countQualityIsCountOnes q (MkBinding d j OneOf p :: bs) with (kindLte (Quality q) j)
  _ | True = cong S (countQualityIsCountOnes q bs)
  _ | False = countQualityIsCountOnes q bs
countQualityIsCountOnes q (MkBinding d j ManyOf p :: bs) = countQualityIsCountOnes q bs

public export
countQualityIsFold : (q : QualitySort) -> (bs : Bindings) ->
                     countChoice (QSort q) bs = countBy (oneOfKind (Quality q)) bs
countQualityIsFold q bs =
  trans (countQualityIsCountOnes q bs) (countOnesIsFold (Quality q) bs)

public export
countLetterIsCountOnes : (l : Letter) -> (bs : Bindings) ->
                         countLetter l bs = countOnes (LetterK l) bs
countLetterIsCountOnes l [] = Refl
countLetterIsCountOnes l (MkBinding d j OneOf p :: bs) with (kindLte (LetterK l) j)
  _ | True = cong S (countLetterIsCountOnes l bs)
  _ | False = countLetterIsCountOnes l bs
countLetterIsCountOnes l (MkBinding d j ManyOf p :: bs) = countLetterIsCountOnes l bs

public export
countLetterIsFold : (l : Letter) -> (bs : Bindings) ->
                    countLetter l bs = countBy (oneOfKind (LetterK l)) bs
countLetterIsFold l bs =
  trans (countLetterIsCountOnes l bs) (countOnesIsFold (LetterK l) bs)

public export
countReachIsFold : (r : Reach) -> (pl : Plurality) -> (bs : Bindings) ->
                   countReach r pl bs = countBy (reaches r pl) bs
countReachIsFold r pl [] = Refl
countReachIsFold r pl (b :: bs) with (reaches r pl b)
  _ | True = cong S (countReachIsFold r pl bs)
  _ | False = countReachIsFold r pl bs

public export
countWordIsFold : (w : NounWord) -> (bs : Bindings) ->
                  countReach (Word w) OneOf bs = countBy (reaches (Word w) OneOf) bs
countWordIsFold w bs = countReachIsFold (Word w) OneOf bs

public export
countManyWordIsFold : (w : NounWord) -> (bs : Bindings) ->
                      countReach (Word w) ManyOf bs = countBy (reaches (Word w) ManyOf) bs
countManyWordIsFold w bs = countReachIsFold (Word w) ManyOf bs

public export
countOnesAtIsFold : (sl : SlotCarrier) -> (bs : Bindings) ->
                    countReach (AtSlot sl) OneOf bs =
                      countBy (reaches (AtSlot sl) OneOf) bs
countOnesAtIsFold sl bs = countReachIsFold (AtSlot sl) OneOf bs

public export
countVerbedItIsFold : (v : VerbLabel) -> (bs : Bindings) ->
                      countReach (Stamped v) OneOf bs =
                        countBy (reaches (Stamped v) OneOf) bs
countVerbedItIsFold v bs = countReachIsFold (Stamped v) OneOf bs

public export
countVerbedThemIsFold : (v : VerbLabel) -> (bs : Bindings) ->
                        countReach (Stamped v) ManyOf bs =
                          countBy (reaches (Stamped v) ManyOf) bs
countVerbedThemIsFold v bs = countReachIsFold (Stamped v) ManyOf bs

public export
countItTokenIsFold : (bs : Bindings) ->
                     countReach TokenBorn OneOf bs = countBy (reaches TokenBorn OneOf) bs
countItTokenIsFold bs = countReachIsFold TokenBorn OneOf bs

public export
countUnionHalfIsFold : (w : NounWord) -> (bs : Bindings) ->
                       countReach (UnionHalf w) OneOf bs =
                         countBy (reaches (UnionHalf w) OneOf) bs
countUnionHalfIsFold w bs = countReachIsFold (UnionHalf w) OneOf bs

public export
countVerbedIsFold : (v : VerbLabel) -> (w : NounWord) -> (bs : Bindings) ->
                    countReach (Verbed v w Attributive) OneOf bs =
                      countBy (reaches (Verbed v w Attributive) OneOf) bs
countVerbedIsFold v w bs = countReachIsFold (Verbed v w Attributive) OneOf bs

public export
countManyVerbedIsFold : (v : VerbLabel) -> (w : NounWord) -> (bs : Bindings) ->
                        countReach (Verbed v w Attributive) ManyOf bs =
                          countBy (reaches (Verbed v w Attributive) ManyOf) bs
countManyVerbedIsFold v w bs =
  countReachIsFold (Verbed v w Attributive) ManyOf bs

public export
groupOne : Binding -> Bool
groupOne (MkBinding PartD _ _ _) = False
groupOne (MkBinding BareD _ _ _) = False
groupOne b = objGroup Object b

public export
countGroupsIsFold : (bs : Bindings) -> countGroups Object bs = countBy groupOne bs
countGroupsIsFold [] = Refl
countGroupsIsFold (MkBinding PartD j p pay :: bs) = countGroupsIsFold bs
countGroupsIsFold (MkBinding TargetD j p pay :: bs)
    with (objGroup Object (MkBinding TargetD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding AD j p pay :: bs) with (objGroup Object (MkBinding AD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding EachD j p pay :: bs)
    with (objGroup Object (MkBinding EachD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding AllD j p pay :: bs) with (objGroup Object (MkBinding AllD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding TheD j p pay :: bs) with (objGroup Object (MkBinding TheD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding CountD j p pay :: bs)
    with (objGroup Object (MkBinding CountD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding SelfD j p pay :: bs)
    with (objGroup Object (MkBinding SelfD j p pay))
  _ | True = cong S (countGroupsIsFold bs)
  _ | False = countGroupsIsFold bs
countGroupsIsFold (MkBinding BareD j p pay :: bs) = countGroupsIsFold bs

public export
partOne : Binding -> Bool
partOne (MkBinding PartD j _ _) = kindLte Object j
partOne b = False

public export
countPartsIsFold : (bs : Bindings) -> countParts Object bs = countBy partOne bs
countPartsIsFold [] = Refl
countPartsIsFold (MkBinding PartD j p pay :: bs) with (kindLte Object j)
  _ | True = cong S (countPartsIsFold bs)
  _ | False = countPartsIsFold bs
countPartsIsFold (MkBinding TargetD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding AD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding EachD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding AllD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding TheD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding CountD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding SelfD j p pay :: bs) = countPartsIsFold bs
countPartsIsFold (MkBinding BareD j p pay :: bs) = countPartsIsFold bs

public export
targetOfKind : Kind -> Binding -> Bool
targetOfKind k (MkBinding TargetD j _ _) = kindLte k j
targetOfKind k b = False

public export
anyTargetedIsAny : (k : Kind) -> (bs : Bindings) ->
                   anyTargeted k bs = anyBy (targetOfKind k) bs
anyTargetedIsAny k [] = Refl
anyTargetedIsAny k (MkBinding TargetD j p pay :: bs) with (kindLte k j)
  _ | True = Refl
  _ | False = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding AD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding EachD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding AllD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding TheD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding PartD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding CountD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding SelfD j p pay :: bs) = anyTargetedIsAny k bs
anyTargetedIsAny k (MkBinding BareD j p pay :: bs) = anyTargetedIsAny k bs

public export
resolveOnes : (k : Kind) -> (bs : Bindings) -> countOnes k bs = 1 ->
              (b : Binding ** (Elem b bs, So (oneOfKind k b)))
resolveOnes k bs ok =
  countByWitness (oneOfKind k) bs Z (trans (sym (countOnesIsFold k bs)) ok)

public export
resolveManys : (k : Kind) -> (bs : Bindings) -> countManys k bs = 1 ->
               (b : Binding ** (Elem b bs, So (manyOfKind k b)))
resolveManys k bs ok =
  countByWitness (manyOfKind k) bs Z (trans (sym (countManysIsFold k bs)) ok)

public export
itReadsOnlyPrefix : (bs : Bindings) -> countReach Bare OneOf bs = 1 -> Noun bs Object
itReadsOnlyPrefix bs ok = (Macros.It OneOf) {bs} {ok}

public export
itResolvesInPrefix : (bs : Bindings) -> countReach Bare OneOf bs = 1 ->
                     (b : Binding ** (Elem b bs, So (reaches Bare OneOf b)))
itResolvesInPrefix bs ok =
  countByWitness (reaches Bare OneOf) bs Z
    (trans (sym (countReachIsFold Bare OneOf bs)) ok)

public export
itAtReadsOnlyPrefix : (sl : SlotCarrier) -> (bs : Bindings) ->
                      countReach (AtSlot sl) OneOf bs = 1 -> Noun bs Object
itAtReadsOnlyPrefix sl bs ok = Pro (AtSlot sl) OneOf {bs} {ok}

public export
itAtResolvesInPrefix : (sl : SlotCarrier) -> (bs : Bindings) ->
                       countReach (AtSlot sl) OneOf bs = 1 ->
                       (b : Binding ** (Elem b bs, So (reaches (AtSlot sl) OneOf b)))
itAtResolvesInPrefix sl bs ok =
  countByWitness (reaches (AtSlot sl) OneOf) bs Z
    (trans (sym (countReachIsFold (AtSlot sl) OneOf bs)) ok)

public export
itVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                          KnownAct v -> countReach (Stamped v) OneOf bs = 1 ->
                          Noun bs Object
itVerbedReadsOnlyPrefix bs v kn ok = Macros.ItVerbed v OneOf {bs} {ok}

public export
itVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                           countReach (Stamped v) OneOf bs = 1 ->
                           (b : Binding ** (Elem b bs, So (reaches (Stamped v) OneOf b)))
itVerbedResolvesInPrefix bs v ok =
  countByWitness (reaches (Stamped v) OneOf) bs Z
    (trans (sym (countReachIsFold (Stamped v) OneOf bs)) ok)

public export
itTokenReadsOnlyPrefix : (bs : Bindings) -> countReach TokenBorn OneOf bs = 1 ->
                         Noun bs Object
itTokenReadsOnlyPrefix bs ok = Pro TokenBorn OneOf {bs} {ok}

public export
itTokenResolvesInPrefix : (bs : Bindings) -> countReach TokenBorn OneOf bs = 1 ->
                          (b : Binding ** (Elem b bs, So (reaches TokenBorn OneOf b)))
itTokenResolvesInPrefix bs ok =
  countByWitness (reaches TokenBorn OneOf) bs Z
    (trans (sym (countReachIsFold TokenBorn OneOf bs)) ok)

public export
elemInSuffix : {0 b : Binding} -> {0 rest : Bindings} -> (co : Bindings) ->
               Elem b rest -> Elem b (co ++ rest)
elemInSuffix [] el = el
elemInSuffix (c :: cs) el = There (elemInSuffix cs el)

public export
elemInPrefix : {0 b : Binding} -> {0 made : Bindings} -> (before : Bindings) ->
               Elem b made -> Elem b (made ++ before)
elemInPrefix bef Here = Here
elemInPrefix bef (There el) = There (elemInPrefix bef el)

||| A segment read counts NO MORE than the whole prefix does, in either
public export
countBySegmentNoLarger : (p : Binding -> Bool) -> (xs, ys : Bindings) ->
                         (LTE (countBy p xs) (countBy p (xs ++ ys)),
                          LTE (countBy p ys) (countBy p (xs ++ ys)))
countBySegmentNoLarger p xs ys =
  rewrite countBySplit p xs ys in
    (lteAddRight (countBy p xs), lteRightPlus (countBy p xs) (countBy p ys))
  where
    lteRightPlus : (n, m : Nat) -> LTE m (n + m)
    lteRightPlus Z m = reflexive
    lteRightPlus (S n) m = lteSuccRight (lteRightPlus n m)

public export
itOtherThanReadsOnlyPrefix : (co, rest : Bindings) ->
                             countOnes Object rest = 1 -> Noun (co ++ rest) Object
itOtherThanReadsOnlyPrefix co rest ok = ItOtherThan co rest {sp = Refl} {ok}

public export
itOtherThanResolvesInPrefix : (co, rest : Bindings) ->
                              countOnes Object rest = 1 ->
                              (b : Binding ** (Elem b (co ++ rest),
                                               So (oneOfKind Object b)))
itOtherThanResolvesInPrefix co rest ok =
  let (b ** (el, k)) = resolveOnes Object rest ok in
      (b ** (elemInSuffix co el, k))

public export
itPriorReadsOnlyPrefix : (made, before : Bindings) ->
                         countReach Bare OneOf made = 1 -> Noun (made ++ before) Object
itPriorReadsOnlyPrefix made before ok = Own Bare OneOf made before {sp = Refl} {ok}

public export
itPriorResolvesInPrefix : (made, before : Bindings) ->
                          countReach Bare OneOf made = 1 ->
                          (b : Binding ** (Elem b (made ++ before),
                                           So (reaches Bare OneOf b)))
itPriorResolvesInPrefix made before ok =
  let (b ** (el, k)) = countByWitness (reaches Bare OneOf) made Z
                         (trans (sym (countReachIsFold Bare OneOf made)) ok) in
      (b ** (elemInPrefix before el, k))

public export
ownReadsOnlyPrefix : (pl : Plurality) -> (own, outer : Bindings) ->
                     countReach Bare pl own = 1 -> Noun (own ++ outer) Object
ownReadsOnlyPrefix pl own outer ok = Own Bare pl own outer {sp = Refl} {ok}

public export
ownResolvesInPrefix : (pl : Plurality) -> (own, outer : Bindings) ->
                      countReach Bare pl own = 1 ->
                      (b : Binding ** (Elem b (own ++ outer), So (reaches Bare pl b)))
ownResolvesInPrefix pl own outer ok =
  let (b ** (el, k)) = countByWitness (reaches Bare pl) own Z
                         (trans (sym (countReachIsFold Bare pl own)) ok) in
      (b ** (elemInPrefix outer el, k))

public export
theyReadsOnlyPrefix : (bs : Bindings) -> countReach (Word PlayerW) OneOf bs = 1 ->
                      Noun bs Player
theyReadsOnlyPrefix bs ok = Macros.That PlayerW OneOf {bs} {ok}

public export
theyResolvesInPrefix : (bs : Bindings) -> countReach (Word PlayerW) OneOf bs = 1 ->
                       (b : Binding ** (Elem b bs,
                                                So (reaches (Word PlayerW) OneOf b)))
theyResolvesInPrefix bs ok =
  countByWitness (reaches (Word PlayerW) OneOf) bs Z
    (trans (sym (countReachIsFold (Word PlayerW) OneOf bs)) ok)

public export
themReadsOnlyPrefix : (bs : Bindings) -> countReach Bare ManyOf bs = 1 -> Noun bs Object
themReadsOnlyPrefix bs ok = (Macros.It ManyOf) {bs} {ok}

public export
themResolvesInPrefix : (bs : Bindings) -> countReach Bare ManyOf bs = 1 ->
                       (b : Binding ** (Elem b bs, So (reaches Bare ManyOf b)))
themResolvesInPrefix bs ok =
  countByWitness (reaches Bare ManyOf) bs Z
    (trans (sym (countReachIsFold Bare ManyOf bs)) ok)

public export
themVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                            KnownAct v -> countReach (Stamped v) ManyOf bs = 1 ->
                            Noun bs Object
themVerbedReadsOnlyPrefix bs v kn ok = Macros.ItVerbed v ManyOf {bs} {ok}

public export
themVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) ->
                             countReach (Stamped v) ManyOf bs = 1 ->
                             (b : Binding ** (Elem b bs,
                                                      So (reaches (Stamped v) ManyOf b)))
themVerbedResolvesInPrefix bs v ok =
  countByWitness (reaches (Stamped v) ManyOf) bs Z
    (trans (sym (countReachIsFold (Stamped v) ManyOf bs)) ok)

public export
thatReadsOnlyPrefix : (bs : Bindings) -> (w : NounWord) ->
                      countReach (Word w) OneOf bs = 1 -> Noun bs (kindOfW w)
thatReadsOnlyPrefix bs w ok = Macros.That w OneOf {bs} {ok}

public export
thatResolvesInPrefix : (bs : Bindings) -> (w : NounWord) ->
                       countReach (Word w) OneOf bs = 1 ->
                       (b : Binding ** (Elem b bs, So (reaches (Word w) OneOf b)))
thatResolvesInPrefix bs w ok =
  countByWitness (reaches (Word w) OneOf) bs Z
    (trans (sym (countReachIsFold (Word w) OneOf bs)) ok)

public export
thatHalfReadsOnlyPrefix : (bs : Bindings) -> (w : NounWord) ->
                          countReach (UnionHalf w) OneOf bs = 1 ->
                          Noun bs (kindOfW w)
thatHalfReadsOnlyPrefix bs w ok = Pro (UnionHalf w) OneOf {bs} {ok}

public export
thatHalfResolvesInPrefix : (bs : Bindings) -> (w : NounWord) ->
                           countReach (UnionHalf w) OneOf bs = 1 ->
                           (b : Binding ** (Elem b bs,
                                                    So (reaches (UnionHalf w) OneOf b)))
thatHalfResolvesInPrefix bs w ok =
  countByWitness (reaches (UnionHalf w) OneOf) bs Z
    (trans (sym (countReachIsFold (UnionHalf w) OneOf bs)) ok)

public export
thoseReadsOnlyPrefix : (bs : Bindings) -> (w : NounWord) ->
                       countReach (Word w) ManyOf bs = 1 -> Noun bs (kindOfW w)
thoseReadsOnlyPrefix bs w ok = Macros.That w ManyOf {bs} {ok}

public export
thoseResolvesInPrefix : (bs : Bindings) -> (w : NounWord) ->
                        countReach (Word w) ManyOf bs = 1 ->
                        (b : Binding ** (Elem b bs, So (reaches (Word w) ManyOf b)))
thoseResolvesInPrefix bs w ok =
  countByWitness (reaches (Word w) ManyOf) bs Z
    (trans (sym (countReachIsFold (Word w) ManyOf bs)) ok)

public export
theVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                           (m : VerbedMarking) ->
                           countReach (Verbed v w m) OneOf bs = 1 ->
                           ActNamesParticiple v -> Noun bs (kindOfW w)
theVerbedReadsOnlyPrefix bs v w m ok mk = Pro (Verbed v w m) OneOf {bs} {ok}

public export
theVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                            (m : VerbedMarking) ->
                            countReach (Verbed v w m) OneOf bs = 1 ->
                            (b : Binding ** (Elem b bs,
                                                     So (reaches (Verbed v w m) OneOf b)))
theVerbedResolvesInPrefix bs v w m ok =
  countByWitness (reaches (Verbed v w m) OneOf) bs Z
    (trans (sym (countReachIsFold (Verbed v w m) OneOf bs)) ok)

public export
thoseVerbedReadsOnlyPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                             (m : VerbedMarking) ->
                             countReach (Verbed v w m) ManyOf bs = 1 ->
                             ActNamesParticiple v -> Noun bs (kindOfW w)
thoseVerbedReadsOnlyPrefix bs v w m ok mk =
  Pro (Verbed v w m) ManyOf {bs} {ok}

public export
thoseVerbedResolvesInPrefix : (bs : Bindings) -> (v : VerbLabel) -> (w : NounWord) ->
                              (m : VerbedMarking) ->
                              countReach (Verbed v w m) ManyOf bs = 1 ->
                              (b : Binding ** (Elem b bs,
                                                       So (reaches (Verbed v w m) ManyOf b)))
thoseVerbedResolvesInPrefix bs v w m ok =
  countByWitness (reaches (Verbed v w m) ManyOf) bs Z
    (trans (sym (countReachIsFold (Verbed v w m) ManyOf bs)) ok)

public export
notZeroSucc : (n : Nat) -> So (not (n == Z)) -> (k : Nat ** n = S k)
notZeroSucc Z Oh impossible
notZeroSucc (S k) ok = (k ** Refl)

||| "the rest"
public export
theRestReadsOnlyPrefix : (bs : Bindings) -> So (theRestOk Object bs) -> Noun bs Object
theRestReadsOnlyPrefix bs ok = Macros.theRest Object {bs} {ok}

public export
theRestResolvesInPrefix : (bs : Bindings) -> So (theRestOk Object bs) ->
                          (p : Binding ** (Elem p bs, So (partOne p)))
theRestResolvesInPrefix bs ok =
  let (_, pOk) = soAnd {a = countGroups Object bs <= 1} ok
      (k ** pEq) = notZeroSucc (countParts Object bs) pOk
   in countByWitness partOne bs k (trans (sym (countPartsIsFold bs)) pEq)

public export
theRestGroupResolvesInPrefix : (bs : Bindings) -> countGroups Object bs = 1 ->
                               (g : Binding ** (Elem g bs, So (groupOne g)))
theRestGroupResolvesInPrefix bs gEq =
  countByWitness groupOne bs Z (trans (sym (countGroupsIsFold bs)) gEq)

public export
thatMuchReadsOnlyPrefix : (bs : Bindings) -> countQuantOutcomes bs = 1 -> Amount bs
thatMuchReadsOnlyPrefix bs ok = ThatMuch {bs} {ok}

public export
thatMuchResolvesInPrefix : (bs : Bindings) -> countQuantOutcomes bs = 1 ->
                           (b : Binding ** (Elem b bs, So (quantOutcome b)))
thatMuchResolvesInPrefix bs ok =
  countByWitness quantOutcome bs Z
                 (trans (sym (countQuantOutcomesIsFold bs)) ok)

public export
preventedThisWayReadsOnlyPrefix : (bs : Bindings) ->
                                  countOutcomes DamagePrevented bs = 1 -> Amount bs
preventedThisWayReadsOnlyPrefix bs ok = Macros.preventedThisWay {bs} {ok}

public export
preventedThisWayResolvesInPrefix :
  (bs : Bindings) -> countOutcomes DamagePrevented bs = 1 ->
  (b : Binding ** (Elem b bs, So (outcomeIs DamagePrevented b)))
preventedThisWayResolvesInPrefix bs ok =
  countByWitness (outcomeIs DamagePrevented) bs Z
                 (trans (sym (countOutcomesIsFold DamagePrevented bs)) ok)

public export
theResultReadsOnlyPrefix : (bs : Bindings) ->
                           countOutcomes RollResult bs = 1 -> Amount bs
theResultReadsOnlyPrefix bs ok = TheOutcome RollResult {ok}

public export
theResultResolvesInPrefix :
  (bs : Bindings) -> countOutcomes RollResult bs = 1 ->
  (b : Binding ** (Elem b bs, So (outcomeIs RollResult b)))
theResultResolvesInPrefix bs ok =
  countByWitness (outcomeIs RollResult) bs Z
                 (trans (sym (countOutcomesIsFold RollResult bs)) ok)

public export
groupSizeReadsOnlyPrefix : (bs : Bindings) -> countManysAny bs = 1 -> Amount bs
groupSizeReadsOnlyPrefix bs ok = GroupSize {bs} {ok}

public export
groupSizeResolvesInPrefix : (bs : Bindings) -> countManysAny bs = 1 ->
                            (b : Binding ** (Elem b bs, So (anyMany b)))
groupSizeResolvesInPrefix bs ok =
  countByWitness anyMany bs Z (trans (sym (countManysAnyIsFold bs)) ok)

public export
theDifferenceReadsOnlyPrefix : (bs : Bindings) -> countOnes Gap bs = 1 -> Amount bs
theDifferenceReadsOnlyPrefix bs ok = TheDifference {bs} {ok}

public export
theDifferenceResolvesInPrefix : (bs : Bindings) -> countOnes Gap bs = 1 ->
                                (b : Binding ** (Elem b bs, So (oneOfKind Gap b)))
theDifferenceResolvesInPrefix bs ok = resolveOnes Gap bs ok

public export
openLetterIsAny : (l : Letter) -> (bs : Bindings) ->
                  anyOpenLetter l bs = anyBy (openLetter l) bs
openLetterIsAny l [] = Refl
openLetterIsAny l (b :: bs) with (openLetter l b)
  _ | True = Refl
  _ | False = openLetterIsAny l bs

public export
defineReadsOnlyPrefix : (bs : Bindings) -> (l : Letter) -> (amt : Amount bs) ->
                        So (anyOpenLetter l bs) -> Instruction bs
defineReadsOnlyPrefix bs l amt ok = Define l amt {bs} {ok}

public export
defineResolvesInPrefix : (bs : Bindings) -> (l : Letter) -> So (anyOpenLetter l bs) ->
                         (b : Binding ** (Elem b bs, So (openLetter l b)))
defineResolvesInPrefix bs l ok =
  anyByWitness (openLetter l) bs (replace {p = So} (openLetterIsAny l bs) ok)

public export
defineClosesLetter : (l : Letter) -> (bs : Bindings) ->
                     anyOpenLetter l (defineLetter l bs) = False
defineClosesLetter l [] = Refl
defineClosesLetter l (b :: bs) with (openLetter l b) proof eq
  _ | True = defineClosesLetter l bs
  _ | False = rewrite eq in defineClosesLetter l bs

||| "where X is ..."
public export
badDefineWithoutUse : Not (So (anyOpenLetter X []))
badDefineWithoutUse Oh impossible

||| a second "where X is" on one ability: the first settled every
public export
badSecondDefine : (bs : Bindings) -> (l : Letter) ->
                  Not (So (anyOpenLetter l (defineLetter l bs)))
badSecondDefine bs l ok = absurd (replace {p = So} (defineClosesLetter l bs) ok)

public export
costXStaysOpen : So (anyOpenLetter X (costIntro (LoyaltySymbol {bs = []} LoyaltyDownX)))
costXStaysOpen = Oh

public export
ofChosenReadsOnlyPrefix : (bs : Bindings) -> (q : QualitySort) ->
                          countChoice (QSort q) bs = 1 -> ChosenQualityRead q ->
                          Predicate bs Object
ofChosenReadsOnlyPrefix bs q ok read = Macros.ofChosen q {bs} {ok} {read}

public export
ofChosenResolvesInPrefix : (bs : Bindings) -> (q : QualitySort) ->
                           countChoice (QSort q) bs = 1 ->
                           (b : Binding ** (Elem b bs,
                                            So (oneOfKind (Quality q) b)))
ofChosenResolvesInPrefix bs q ok =
  countByWitness (oneOfKind (Quality q)) bs Z
                 (trans (sym (countQualityIsFold q bs)) ok)

public export
choiceStandsSucc : (n : Nat) -> ChoiceStands n -> (k : Nat ** n = S k)
choiceStandsSucc Z Oh impossible
choiceStandsSucc (S k) Oh = (k ** Refl)

public export
ofLastChosenColorReadsOnlyPrefix : (bs : Bindings) ->
                                   ChoiceStands (countChoice (QSort Color) bs) ->
                                   Predicate bs Object
ofLastChosenColorReadsOnlyPrefix bs ok = Macros.ofTheLastChosen Color {bs} {ok}

public export
ofLastChosenColorResolvesInPrefix :
  (bs : Bindings) -> ChoiceStands (countChoice (QSort Color) bs) ->
  (b : Binding ** (Elem b bs, So (oneOfKind (Quality Color) b)))
ofLastChosenColorResolvesInPrefix bs ok =
  let (k ** eq) = choiceStandsSucc (countChoice (QSort Color) bs) ok
   in countByWitness (oneOfKind (Quality Color)) bs k
                     (trans (sym (countQualityIsFold Color bs)) eq)

public export
chosenNameReadsOnlyPrefix : (bs : Bindings) -> countChoice (QSort CardName) bs = 1 ->
                            NameSource bs
chosenNameReadsOnlyPrefix bs ok = ChosenName {bs} {ok}

public export
chosenNameResolvesInPrefix : (bs : Bindings) -> countChoice (QSort CardName) bs = 1 ->
                             (b : Binding ** (Elem b bs,
                                              So (oneOfKind (Quality CardName) b)))
chosenNameResolvesInPrefix bs ok = ofChosenResolvesInPrefix bs CardName ok

public export
ofChosenColorReadsOnlyPrefix : (bs : Bindings) -> (alt : Maybe ProducedRun) ->
                               countChoice (QSort Color) bs = 1 ->
                               ChosenQualityRead Color -> ProducedMana bs
ofChosenColorReadsOnlyPrefix bs alt cq rd =
  OfChosenColor alt {bs} {cq} {rd}

public export
ofChosenColorResolvesInPrefix : (bs : Bindings) -> countChoice (QSort Color) bs = 1 ->
                                (b : Binding ** (Elem b bs,
                                                 So (oneOfKind (Quality Color) b)))
ofChosenColorResolvesInPrefix bs ok = ofChosenResolvesInPrefix bs Color ok

||| "Any other target"
public export
otherReadsOnlyPrefix : (bs : Bindings) -> (k : Kind) ->
                       So (anyTargeted k bs) -> Predicate bs k
otherReadsOnlyPrefix bs k ok = Other {bs} {k} {ok}

public export
otherResolvesInPrefix : (bs : Bindings) -> (k : Kind) -> So (anyTargeted k bs) ->
                        (b : Binding ** (Elem b bs, So (targetOfKind k b)))
otherResolvesInPrefix bs k ok =
  anyByWitness (targetOfKind k) bs (replace {p = So} (anyTargetedIsAny k bs) ok)

public export
turnInScopeReadsOnlyPrefix : (bs : Bindings) -> countReach ThatTurn OneOf bs = 1 ->
                             Noun bs TurnRef
turnInScopeReadsOnlyPrefix bs ok = Pro ThatTurn OneOf {bs} {ok}

public export
turnInScopeResolvesInPrefix : (bs : Bindings) -> countReach ThatTurn OneOf bs = 1 ->
                              (b : Binding ** (Elem b bs,
                                               So (reaches ThatTurn OneOf b)))
turnInScopeResolvesInPrefix bs ok =
  countByWitness (reaches ThatTurn OneOf) bs Z
    (trans (sym (countReachIsFold ThatTurn OneOf bs)) ok)

||| "that turn" after exactly one extra turn is minted
public export
okThatTurnAfterOneTurn :
  Noun (instrIntro {bs = []} (ExtraTurn You (Lit 1))) TurnRef
okThatTurnAfterOneTurn =
  Macros.thatTurn {bs = instrIntro {bs = []} (ExtraTurn You (Lit 1))}

public export
badThatTurnWithoutTurn : Unspellable (Noun [] TurnRef) (\ok =>
  Macros.thatTurn {bs = []} {ok})
badThatTurnWithoutTurn Refl impossible

public export
tokenAsThoseReadsOnlyPrefix : (bs : Bindings) -> countTokenSpecs bs = 1 ->
                              TokenSpec bs
tokenAsThoseReadsOnlyPrefix bs ok = TokenAsThose {bs} {ok}

public export
noTokenAsThoseWithoutAntecedent : Not (countTokenSpecs [] = 1)
noTokenAsThoseWithoutAntecedent Refl impossible

public export
oneTokenIsOneSpec :
  countTokenSpecs [MkBinding AD Object OneOf
                             (ObjectP (Just Creature) (Just Battlefield)
                                      Nothing (Just TokenOrigin) Nothing)] = 1
oneTokenIsOneSpec = Refl

public export
manyTokensAreOneSpec :
  countTokenSpecs [MkBinding AD Object ManyOf
                             (ObjectP (Just Creature) (Just Battlefield)
                                      Nothing (Just TokenOrigin) Nothing)] = 1
manyTokensAreOneSpec = Refl

||| A non-token object leaves no definition whatever its plurality, which
public export
oneNonTokenIsNoSpec :
  countTokenSpecs [MkBinding TheD Object OneOf
                             (ObjectP (Just Creature) (Just Battlefield)
                                      Nothing Nothing Nothing)] = 0
oneNonTokenIsNoSpec = Refl

public export
sumIsOne : (m, n : Nat) -> m + n = 1 -> Either (m = 1) (n = 1)
sumIsOne Z n prf = Right prf
sumIsOne (S Z) Z prf = Left Refl
sumIsOne (S Z) (S _) Refl impossible
sumIsOne (S (S _)) _ Refl impossible

public export
theirChoiceReadsOnlyPrefix : (bs : Bindings) -> countChoosers bs = 1 ->
                             ChoiceMode bs
theirChoiceReadsOnlyPrefix bs ok = TheirChoice {bs} {ch = ok}

public export
theirChoiceResolvesInPrefix :
  (bs : Bindings) -> countChoosers bs = 1 ->
  Either (b : Binding ** (Elem b bs, So (oneOfKind Player b)))
         (b : Binding ** (Elem b bs, So (manyOfKind Player b)))
theirChoiceResolvesInPrefix bs ok =
  case sumIsOne (countOnes Player bs) (countManys Player bs) ok of
    Left one => Left (resolveOnes Player bs one)
    Right many => Right (resolveManys Player bs many)

public export
thisNeedsNoAntecedent : Noun [] Object
thisNeedsNoAntecedent = This

||| "You"
public export
youNeedsNoAntecedent : Noun [] Player
youNeedsNoAntecedent = You

public export
playerGroupNeedsNoAntecedent : (w : PlayerGroupWord) -> Noun [] Player
playerGroupNeedsNoAntecedent w = PlayerGroup w

||| "Enchanted creature"
public export
attachHostNeedsNoAntecedent : (w : AttachWord) -> (h : NounWord) ->
                              AttachHeadOk w h -> Noun [] (kindOfW h)
attachHostNeedsNoAntecedent w h ok = AttachHost w h {ok}

public export
letterValIntroducesAtEmptyPrefix : (l : Letter) -> Amount []
letterValIntroducesAtEmptyPrefix l = LetterVal l

||| "[n]'s controller sacrifices it"
public export
controllerSacrificesReadsNoPrefix : (bs : Bindings) -> (n : Noun bs Object) ->
                                    nounPlur n = OneOf ->
                                    ZoneIs (nounZone n) Battlefield -> Instruction bs
controllerSacrificesReadsNoPrefix bs n one zn = ControllerSacrifices n {one} {zn}

public export
ownSurvivesSecondSingular : Instruction []
ownSurvivesSecondSingular =
  Sequentially [Macros.exile You (Macros.target Macros.artifact),
                Macros.dealsDamageOwnPower (Macros.target Macros.creature)
                                           (Macros.target Macros.anyTarget)]

||| "Exile target creature. Draw cards equal to its power."
public export
okItReadsTheOnlyBareSingular : Instruction []
okItReadsTheOnlyBareSingular =
  Sequentially [Macros.exile You (Macros.target Macros.creature),
                Draw You (StatOf Power (Macros.It OneOf))]

public export
badItAcrossOwnSlot : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.exile You (Macros.target Macros.artifact),
                DealDamage (Macros.target Macros.creature) (StatOf Power ((Macros.It OneOf) {ok}))
                           (Macros.target Macros.anyTarget)])
badItAcrossOwnSlot Refl impossible

||| "Draw cards equal to its power."
public export
badSingularReadOfBarePlural : Unspellable (Instruction []) (\ok =>
  Sequentially [Macros.gets (Macros.bare Macros.creatureYouControl)
                            (PtUp (Lit 1)) (PtUp (Lit 1)) (Just Macros.untilEndOfTurn),
                Draw You (StatOf Power ((Macros.It OneOf) {ok}))])
badSingularReadOfBarePlural Refl impossible

public export
badOwnEmptyDelta : Unspellable (Instruction []) (\ok =>
  Macros.dealsDamageOwnPower Macros.thisCreature (Macros.target Macros.anyTarget) {ok})
badOwnEmptyDelta Refl impossible

||| "Target creature gets +1/+1"
public export
okOwnReadsOneInDelta : Instruction []
okOwnReadsOneInDelta =
  Macros.sharedSubject (Macros.target Macros.creature)
    [ Gets Adds (Own Bare OneOf
                     (nounDelta {k = Object}
                                (Macros.target {bs = []} Macros.creature))
                     [] {sp = Refl})
           (PtUp (Lit 1)) (PtUp (Lit 1)) ]
    Nothing

public export
badSharedSubjectTwoInDelta : Unspellable (Instruction []) (\ok =>
  Macros.sharedSubject (Both (Macros.target {bs = []} Macros.creature)
                             (Macros.target
                               {bs = nomIntro (Macros.target {bs = []} Macros.creature)}
                               Macros.artifact))
    [ Gets Adds (Own Bare OneOf (nounDelta {k = Object}
                    (Both (Macros.target {bs = []} Macros.creature)
                          (Macros.target
                            {bs = nomIntro (Macros.target {bs = []} Macros.creature)}
                            Macros.artifact))) []
                {sp = Refl} {ok})
           (PtUp (Lit 1)) (PtUp (Lit 1)) ]
    Nothing)
badSharedSubjectTwoInDelta Refl impossible

public export
badOwnTwoInDelta : Unspellable (Instruction []) (\ok =>
  Macros.dealsDamageOwnPower (Both (Macros.target Macros.creature) (Macros.target Macros.creature))
                             (Macros.target Macros.anyTarget) {ok})
badOwnTwoInDelta Refl impossible

||| "[n] [static1] and [static2]"
public export
sharedSubjectReadsNoPrefix : (bs : Bindings) -> (k : Nat) -> (n : Noun bs Object) ->
                             (parts : StaticParts k (selfSubjIntro n)) -> IsSucc k ->
                             StaticSpec bs
sharedSubjectReadsNoPrefix bs k n parts ne = AndAlso (Just n) parts {ne}

||| A noun hands the next clause its own mints in front of the prefix it
public export
nomIntroIsDeltaThenPrefix : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                            nomIntro n = nounDelta n ++ bs
nomIntroIsDeltaThenPrefix bs k n = Refl

||| A type-naming test writes a card type and nothing a counted gate
public export
markTyKeepsOnes : (j : Kind) -> (ty : Maybe CardType) -> (b : Binding) ->
                  oneOfKind j (markTy ty b) = oneOfKind j b
markTyKeepsOnes j ty (MkBinding det Object OneOf (ObjectP Nothing zn st og _)) = Refl
markTyKeepsOnes j ty (MkBinding det Object ManyOf (ObjectP Nothing zn st og _)) = Refl
markTyKeepsOnes j ty (MkBinding det Object plur (ObjectP (Just t) zn st og _)) = Refl
markTyKeepsOnes j ty (MkBinding det Pile plur (PileP zn sz fc)) = Refl
markTyKeepsOnes j ty (MkBinding det Player plur PlayerP) = Refl
markTyKeepsOnes j ty (MkBinding det Player plur ChosenPlayerP) = Refl
markTyKeepsOnes j ty (MkBinding det (Quality q) plur QualityP) = Refl
markTyKeepsOnes j ty (MkBinding det Outcome plur (OutcomeP s)) = Refl
markTyKeepsOnes j ty (MkBinding det Gap plur GapP) = Refl
markTyKeepsOnes j ty (MkBinding det (LetterK l) plur LetterP) = Refl
markTyKeepsOnes j ty (MkBinding det TurnRef plur TurnRefP) = Refl
markTyKeepsOnes j ty (MkBinding det Object plur (AbilityP og)) = Refl
markTyKeepsOnes j ty (MkBinding det (a \/ b) plur (JoinP l r)) = Refl

public export
markTyKeepsAt : (sl : SlotCarrier) -> (ty : Maybe CardType) -> (b : Binding) ->
                reaches (AtSlot sl) OneOf (markTy ty b) = reaches (AtSlot sl) OneOf b
markTyKeepsAt sl ty (MkBinding det Object plur (ObjectP Nothing zn st og _)) = Refl
markTyKeepsAt sl ty (MkBinding det Object plur (ObjectP (Just t) zn st og _)) = Refl
markTyKeepsAt sl ty (MkBinding det Pile plur (PileP zn sz fc)) = Refl
markTyKeepsAt sl ty (MkBinding det Player plur PlayerP) = Refl
markTyKeepsAt sl ty (MkBinding det Player plur ChosenPlayerP) = Refl
markTyKeepsAt sl ty (MkBinding det (Quality q) plur QualityP) = Refl
markTyKeepsAt sl ty (MkBinding det Outcome plur (OutcomeP s)) = Refl
markTyKeepsAt sl ty (MkBinding det Gap plur GapP) = Refl
markTyKeepsAt sl ty (MkBinding det (LetterK l) plur LetterP) = Refl
markTyKeepsAt sl ty (MkBinding det TurnRef plur TurnRefP) = Refl
markTyKeepsAt sl ty (MkBinding det Object plur (AbilityP og)) = Refl
markTyKeepsAt sl ty (MkBinding det (a \/ b) plur (JoinP l r)) = Refl

public export
markFirstKeepsLength : (q : Binding -> Bool) -> (ty : Maybe CardType) ->
                       (bs : Bindings) -> length (markFirst q ty bs) = length bs
markFirstKeepsLength q ty [] = Refl
markFirstKeepsLength q ty (b :: bs) with (q b)
  _ | True = Refl
  _ | False = cong S (markFirstKeepsLength q ty bs)

public export
keptBy : Bool -> Nat -> Nat
keptBy True n = S n
keptBy False n = n

public export
countByCons : (r : Binding -> Bool) -> (x : Binding) -> (bs : Bindings) ->
              countBy r (x :: bs) = keptBy (r x) (countBy r bs)
countByCons r x bs with (r x)
  _ | True = Refl
  _ | False = Refl

public export
countByHeadCong : (r : Binding -> Bool) -> (x, y : Binding) -> r x = r y ->
                  (bs : Bindings) -> countBy r (x :: bs) = countBy r (y :: bs)
countByHeadCong r x y prf bs =
  trans (countByCons r x bs)
        (trans (cong (\v => keptBy v (countBy r bs)) prf)
               (sym (countByCons r y bs)))

public export
markFirstKeeps : (r : Binding -> Bool) -> (q : Binding -> Bool) ->
                 (ty : Maybe CardType) ->
                 ((b : Binding) -> r (markTy ty b) = r b) ->
                 (bs : Bindings) -> countBy r (markFirst q ty bs) = countBy r bs
markFirstKeeps r q ty pres [] = Refl
markFirstKeeps r q ty pres (b :: bs) with (q b)
  _ | True = countByHeadCong r (markTy ty b) b (pres b) bs
  _ | False with (r b)
    _ | True = cong S (markFirstKeeps r q ty pres bs)
    _ | False = markFirstKeeps r q ty pres bs

public export
condRemarkKeepsOnes : (bs : Bindings) -> (c : Condition bs) -> (j : Kind) ->
                      countOnes j (condRemark c) = countOnes j bs
condRemarkKeepsOnes bs c j with (condRemarkAt c)
  _ | Nothing = Refl
  _ | Just (q, ty) =
    trans (countOnesIsFold j (markFirst q ty bs))
          (trans (markFirstKeeps (oneOfKind j) q ty (markTyKeepsOnes j ty) bs)
                 (sym (countOnesIsFold j bs)))

public export
condRemarkKeepsAt : (bs : Bindings) -> (c : Condition bs) -> (sl : SlotCarrier) ->
                    countReach (AtSlot sl) OneOf (condRemark c) =
                      countReach (AtSlot sl) OneOf bs
condRemarkKeepsAt bs c sl with (condRemarkAt c)
  _ | Nothing = Refl
  _ | Just (q, ty) =
    trans (countReachIsFold (AtSlot sl) OneOf (markFirst q ty bs))
          (trans (markFirstKeeps (reaches (AtSlot sl) OneOf) q ty
                                 (markTyKeepsAt sl ty) bs)
                 (sym (countReachIsFold (AtSlot sl) OneOf bs)))

public export
condIntroIsDeltaThenRemark : (bs : Bindings) -> (c : Condition bs) ->
                             condIntro c = condDelta c ++ condRemark c
condIntroIsDeltaThenRemark bs c = Refl

public export
otherwiseCtxIsThenBranchOnly : (bs : Bindings) -> (e : Instruction bs) ->
                               otherwiseCtx e = outcomesOnly (deedDelta e) ++ annIntro e
otherwiseCtxIsThenBranchOnly bs e = Refl

public export
gateSplitsAtNomIntro : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                       (j : Kind) ->
                       countOnes j (nomIntro n) =
                         countOnes j (nounDelta n) + countOnes j bs
gateSplitsAtNomIntro bs k n j =
  trans (countOnesIsFold j (nounDelta n ++ bs))
        (trans (countBySplit (oneOfKind j) (nounDelta n) bs)
               (cong2 (+) (sym (countOnesIsFold j (nounDelta n)))
                          (sym (countOnesIsFold j bs))))

public export
gateSplitsAtCondIntro : (bs : Bindings) -> (c : Condition bs) -> (j : Kind) ->
                        countOnes j (condIntro c) =
                          countOnes j (condDelta c) + countOnes j bs
gateSplitsAtCondIntro bs c j =
  trans (countOnesIsFold j (condDelta c ++ condRemark c))
        (trans (countBySplit (oneOfKind j) (condDelta c) (condRemark c))
               (cong2 (+) (sym (countOnesIsFold j (condDelta c)))
                          (trans (sym (countOnesIsFold j (condRemark c)))
                                 (condRemarkKeepsOnes bs c j))))

public export
slotGateSplitsAtNomIntro : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                           (sl : SlotCarrier) ->
                           countReach (AtSlot sl) OneOf (nomIntro n) =
                             countReach (AtSlot sl) OneOf (nounDelta n) +
                             countReach (AtSlot sl) OneOf bs
slotGateSplitsAtNomIntro bs k n sl =
  trans (countReachIsFold (AtSlot sl) OneOf (nounDelta n ++ bs))
        (trans (countBySplit (reaches (AtSlot sl) OneOf) (nounDelta n) bs)
               (cong2 (+) (sym (countReachIsFold (AtSlot sl) OneOf (nounDelta n)))
                          (sym (countReachIsFold (AtSlot sl) OneOf bs))))

public export
slotGateSplitsAtCondIntro : (bs : Bindings) -> (c : Condition bs) ->
                            (sl : SlotCarrier) ->
                            countReach (AtSlot sl) OneOf (condIntro c) =
                              countReach (AtSlot sl) OneOf (condDelta c) +
                              countReach (AtSlot sl) OneOf bs
slotGateSplitsAtCondIntro bs c sl =
  trans (countReachIsFold (AtSlot sl) OneOf (condDelta c ++ condRemark c))
        (trans (countBySplit (reaches (AtSlot sl) OneOf)
                             (condDelta c) (condRemark c))
               (cong2 (+) (sym (countReachIsFold (AtSlot sl) OneOf (condDelta c)))
                          (trans (sym (countReachIsFold (AtSlot sl) OneOf (condRemark c)))
                                 (condRemarkKeepsAt bs c sl))))

public export
unionHalfGateSplitsAtNomIntro : (bs : Bindings) -> (k : Kind) -> (n : Noun bs k) ->
                                (w : NounWord) ->
                                countReach (UnionHalf w) OneOf (nomIntro n) =
                                  countReach (UnionHalf w) OneOf (nounDelta n) +
                                  countReach (UnionHalf w) OneOf bs
unionHalfGateSplitsAtNomIntro bs k n w =
  trans (countReachIsFold (UnionHalf w) OneOf (nounDelta n ++ bs))
        (trans (countBySplit (reaches (UnionHalf w) OneOf) (nounDelta n) bs)
               (cong2 (+) (sym (countReachIsFold (UnionHalf w) OneOf (nounDelta n)))
                          (sym (countReachIsFold (UnionHalf w) OneOf bs))))

public export
instructionsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (e : Instruction bs) ->
                      Instructions n (instrIntro e) -> Instructions (S n) bs
instructionsThreadPrefix bs n e es = e :: es

public export
simInstructionsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (e : Instruction bs) ->
                         SimInstructions n (annIntro e) -> SimInstructions (S n) bs
simInstructionsThreadPrefix bs n e es = e :: es

public export
costSeqThreadsPrefix : (bs : Bindings) -> (n : Nat) -> (c : Cost bs) ->
                       CostSeq n (costIntro c) -> CostSeq (S n) bs
costSeqThreadsPrefix bs n c cs = (::) c cs

public export
staticPartsThreadPrefix : (bs : Bindings) -> (n : Nat) -> (se : StaticSpec bs) ->
                          NotCoord se -> StaticParts n (staticIntro se) ->
                          StaticParts (S n) bs
staticPartsThreadPrefix bs n se nc rest = (::) se {nc} rest

||| An arithmetic amount reads its left operand's output, not the other
public export
amountPlusThreadsPrefix : (bs : Bindings) -> (a : Amount bs) ->
                          Amount (amtIntro a) -> Amount bs
amountPlusThreadsPrefix bs a b = Plus a b

public export
payThreadsPrefix : (bs : Bindings) -> (who : Noun bs Player) ->
                   (c : Cost (nomIntro who)) -> Payable c -> PayAgrees who c ->
                   Instruction bs
payThreadsPrefix bs who c pb ag = Pay who c PaidOnce {pb} {ag}

public export
onlyIfThreadsPrefix : (bs : Bindings) -> (e : Instruction bs) ->
                      Condition (preIntro e) -> Instruction bs
onlyIfThreadsPrefix bs e c = OnlyIf e c Nothing

public export
ifThreadsPrefix : (bs : Bindings) -> (c : Condition bs) ->
                  Instruction (condIntro c) -> Instruction bs
ifThreadsPrefix bs c e = If c e Nothing

public export
onlyWhileThreadsPrefix : (bs : Bindings) -> (se : StaticSpec bs) ->
                         (c : Condition (staticIntro se)) ->
                         MarkingOk AsLongAs c -> StaticSpec bs
onlyWhileThreadsPrefix bs se c mk =
  Conditionally {bs} se c AsLongAs {mk}

public export
thisWayThreadsPrefix : (bs : Bindings) -> (body : Instruction bs) ->
                       (ev : GameEvent (instrIntro body)) ->
                       Instruction (thisWayCtx body ev) -> ThisWayOutcome body ->
                       Instruction bs
thisWayThreadsPrefix bs body ev trig oc = ThisWay body ev trig {oc}

||| "where X is"
public export
defineIntroIsRemark : (bs : Bindings) -> (l : Letter) -> (amt : Amount bs) ->
                      (ok : So (anyOpenLetter l bs)) ->
                      instrIntro (Define l amt {ok}) = defineLetter l (amtIntro amt)
defineIntroIsRemark bs l amt ok = Refl

public export
twinShiftMintsOneLetter :
  countLetter X (staticIntro (Gets Adds {bs = []} Macros.thisCreature
                                   (PtDown (LetterVal X)) (PtDown (LetterVal X)))) = 1
twinShiftMintsOneLetter = Refl

public export
letterValDeltaIsPrefixFold : (bs : Bindings) -> (l : Letter) ->
                             amtDelta (LetterVal l {bs}) = letterDelta l bs
letterValDeltaIsPrefixFold bs l = Refl

||| "Each opponent discards a card. Exile those cards."
public export
distributedDeedReadsBackPlural : Instruction []
distributedDeedReadsBackPlural =
  Sequentially [ Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ))
               , Macros.exile You (Macros.TheVerbed "Discard" CardW Attributive ManyOf) ]

||| "Discard a card. Exile the discarded card."
public export
okTheVerbedAfterSingularDiscard : Instruction []
okTheVerbedAfterSingularDiscard =
  Sequentially [ Macros.discard You (Macros.a (InZone Macros.handZ))
               , Macros.exile You (Macros.TheVerbed "Discard" CardW Attributive OneOf) ]

||| "Each opponent discards a card. Exile that card."
public export
badDistributedDiscardSingular : Unspellable (Instruction []) (\ok =>
  Sequentially [ Macros.discard (Macros.each Opponent) (Macros.a (InZone Macros.handZ))
               , Macros.exile You (Macros.TheVerbed "Discard" CardW Attributive OneOf {ok}) ])
badDistributedDiscardSingular Refl impossible

||| "Tap target creature."
public export
okTapBattlefieldPermanent : Instruction []
okTapBattlefieldPermanent = SetStatus Tapped (Macros.target Macros.creature)

||| "Tap the top card of your library."
public export
badTapLibraryTop : Unspellable (Instruction []) (\ok =>
  SetStatus Tapped (Macros.topSlice (Lit 1)) {ok})
badTapLibraryTop Oh impossible

||| "if there is no monarch"
public export
okNoHolderOnPlayer : Condition []
okNoHolderOnPlayer = NoHolder Monarch

||| "if there is no monstrous creature"
public export
badNoHolderOnObject : Unspellable (Condition []) (\ok =>
  NoHolder Monstrous {sc = ok})
badNoHolderOnObject Refl impossible

||| "Roll five d6. Store those results on this creature."
public export
okStoreResultsAfterRoll : Instruction []
okStoreResultsAfterRoll =
  Sequentially [ (Macros.rollDice You 5 6)
               , StoreResults Macros.thisCreature ]

||| "Store those results on this creature."
public export
badStoreResultsWithoutRoll : Unspellable (Instruction []) (\ok =>
  StoreResults Macros.thisCreature {ok})
badStoreResultsWithoutRoll Refl impossible

||| "If you would roll one or more d6, instead roll that many of those dice."
public export
okThoseDiceAfterRollEvent : Instruction []
okThoseDiceAfterRollEvent =
  Macros.ifWouldInstead (RollsDice You ManyDice (SidedDie 6) AnyResult)
    (RollDice You ThatMuch ThoseDice) Nothing

||| "Roll that many dice."
public export
badAnaphoricSidesWithoutRoll : Unspellable (Instruction []) (\ok =>
  RollDice You (Lit 1) (ThoseDice {ok}))
badAnaphoricSidesWithoutRoll Refl impossible
