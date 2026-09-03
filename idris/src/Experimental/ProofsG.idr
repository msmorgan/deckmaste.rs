||| Unspellable pins, continued from Experimental.ProofsF.
module Experimental.ProofsG

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "This deals 3 damage to that permanent or player."
||| The union anaphor needs an antecedent of its own sort [CR#115.1], and no union mention was made.
public export
badUnionAnaphorNoAntecedent : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3) (That JoinW {ok = ok}))
badUnionAnaphorNoAntecedent Refl impossible


||| "Destroy target creature. This deals 3 damage to that permanent or player."
||| A described object leaves an object payload where a union head leaves a union one.
public export
badUnionAnaphorOnObject : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , DealDamage This (Lit 3) (That JoinW {ok = ok}) ])
badUnionAnaphorOnObject Refl impossible


||| "This deals 3 damage to any target. Counter that spell or ability."
||| "Any target" names creatures, players, planeswalkers and battles [CR#115.4] -- the permanents and players [CR#115.2] contrasts with a spell or ability -- so the spell-or-ability word finds no mention of its own sort.
public export
badAbilityJoinAnaphorOnPlayerUnion : Unspellable (Effect []) (\ok =>
  Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.anyTarget)
               , Macros.counterSpell (That AbilityJoinW {ok = ok}) ])
badAbilityJoinAnaphorOnPlayerUnion Refl impossible


||| "Counter target activated ability. Counter that spell or ability."
||| "Spell or ability" is a PAIR [CR#115.2]; a clause that named only its ability half left one mention, and the word for that half is what reads it back.
public export
badAbilityJoinAnaphorOnAbility : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.counterSpell (Macros.target (AbilityHead AnyActivated))
               , Macros.counterSpell (That AbilityJoinW {ok = ok}) ])
badAbilityJoinAnaphorOnAbility Refl impossible



||| "each creature with a poison counter on it"
||| [CR#122.1] places a counter on an object or a player, and poison is a player's kind.
public export
badPoisonCounterDescription : Unspellable (Predicate [] Object) (\ok =>
  HasCounters (Just Poison) {kn = KindNamed {sc = ok}})
badPoisonCounterDescription Refl impossible


||| "Cumulative upkeep"
||| [CR#702.24a] states the keyword as "Cumulative upkeep [cost]", so the parameter is never absent.
public export
badBareCumulativeUpkeep : Unspellable Ability (\ok =>
  KeywordAbility "CumulativeUpkeep" Nothing {pf = ok})
badBareCumulativeUpkeep Oh impossible


||| "Cumulative upkeep {2}"
public export
badCumulativeUpkeepOnSpell : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility "CumulativeUpkeep" (Just (ParamCost (Mana [Macros.generic 2])))] Nothing {tx = ok})
badCumulativeUpkeepOnSpell Oh impossible


||| "a cumulative upkeep counter"
||| [CR#122.1b] closes the keyword-counter list by enumeration, and cumulative upkeep is not on it.
public export
badCumulativeUpkeepCounter : Unspellable CounterKind (\ok =>
  KeywordCounter "CumulativeUpkeep" {ok = ok})
badCumulativeUpkeepCounter Oh impossible


||| "Creatures you control get +1/+1. The same is true for menace and trample."
||| The trailer extends a keyword-conditional line, so the base sentence must name a keyword.
public export
badKeywordListOnPlainLine : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Gets (Macros.allOf Macros.creatureYouControl)
                                (PtUp (Lit 1)) (PtUp (Lit 1))))
                  [TheKeyword "Menace", TheKeyword "Trample"]
                  {ex = Builtin.fst ok, lk = Builtin.snd ok})
badKeywordListOnPlainLine (Oh, _) impossible


||| "This creature has flying as long as a card exiled with it has flying. The same is true for."
||| The trailer's whole content is the words it names, and an empty list names none.
public export
badEmptyKeywordList : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")]))
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing)) AsLongAs))
                  [] {lk = ok})
badEmptyKeywordList Oh impossible


||| "This creature has flying as long as a card exiled with it has flying. The same is true for menace, flying, and trample."
||| The list is disjoint from the base; a repeat would write the same sentence twice.
public export
badKeywordListRepeatingBase : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")]))
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing)) AsLongAs))
                  [TheKeyword "Menace", TheKeyword "Flying",
                   TheKeyword "Trample"] {lk = ok})
badKeywordListRepeatingBase Oh impossible


||| "This creature has flying as long as a card exiled with it has flying. The same is true for ward."
public export
badParameterisedKeywordInList : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")]))
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing)) AsLongAs))
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
||| A creation is a creation of something: with no complement the clause names no event.
public export
badBareTokenCreationLookback : Unspellable (Condition []) (\ok =>
  Happened TokenCreation You Lookback.ThisTurn Nothing {cw = LeftBare {ok = ok}})
badBareTokenCreationLookback Oh impossible


||| "Destroy target creature that attacked with this creature this turn."
||| A creature attacks and does not attack with anything, so the complement's sort refuses.
public export
badAttackerComplementOnObject : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo AttackDeclaration Lookback.ThisTurn (Just (Involving Macros.thisCreature {cp = ok})))
badAttackerComplementOnObject MkLookbackComplement impossible


||| "When this creature enters, draw a card if you've cast an opponent this turn."
||| The complement has a sort and the cast's is an object: one does not cast a player.
public export
badPlayerCastComplement : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn (Just (Involving Macros.anOpponent {cp = ok})))
badPlayerCastComplement MkLookbackComplement impossible


||| "Counter target spell cast from the stack."
||| [CR#112.1] moves the card to the stack from the zone it was in, which is never the stack.
public export
badCastFromStack : Unspellable (Predicate [] Object) (\ok =>
  CastFrom Macros.stackZ {pf = ok})
badCastFromStack Oh impossible


||| "… that died from the battlefield this turn."
||| [CR#700.4] fixes both ends of a death -- the term MEANS "is put into a graveyard from the battlefield" -- so the clause has no origin left to name.
public export
badDeathOriginZone : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo Death Lookback.ThisTurn
             (Just (FromZones (FromZone [Macros.battlefieldZ]) Nothing {ok = ok})))
badDeathOriginZone Oh impossible


||| "if you've cast a spell from the stack this turn"
||| The origin rider rides `playableFrom`: [CR#112.1] puts the card ON the stack as it is cast, so the stack is where a cast ends and never where one starts.
public export
badCastOriginFromStack : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromZone [Macros.stackZ]) Nothing {ok = ok})))
badCastOriginFromStack Oh impossible


||| "if you've cast a spell from this turn"
||| Coordination at the zone sort still names zones; one that names none is no origin.
public export
badEmptyOriginCoordination : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromZone []) Nothing {ok = ok})))
badEmptyOriginCoordination Oh impossible


||| "if you've cast a spell from your hand from the command zone this turn"
||| One zone payload to a complement: the second "from" names nothing the first left unsaid.
public export
badNestedOriginPayload : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromZone [Macros.handZ])
                    (Just (FromZones (FromZone [Macros.commandZ]) Nothing)) {pl = ok})))
badNestedOriginPayload Oh impossible


||| "if you've cast a spell from anywhere other than this turn"
||| An exclusion picks out the rest of [CR#400.1]'s zone list by naming zones; one that names none excludes nothing.
public export
badEmptyOriginExclusion : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
           (Just (FromZones (FromAnywhereBut []) Nothing {ok = ok})))
badEmptyOriginExclusion Oh impossible


||| "… that died from anywhere this turn."
||| [CR#700.4] fixes both ends of a death, so the clause has no origin to name -- an unnamed one included.
public export
badDeathOriginAnywhere : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo Death Lookback.ThisTurn
             (Just (FromZones FromAnywhere Nothing {ok = ok})))
badDeathOriginAnywhere Oh impossible


||| "… that was put into the battlefield this turn."
||| [CR#603.6a] writes a permanent's arrival as putting it ONTO the battlefield and gives it the enters-the-battlefield ability, so this event never lands there.
public export
badPlacementIntoBattlefield : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo Placement Lookback.ThisTurn
             (Just (IntoZone Macros.battlefieldZ Nothing {ok = ok})))
badPlacementIntoBattlefield Oh impossible


||| "… that was put into your graveyard into exile this turn."
||| One clause names one arrival: the second destination names nothing the first left unsaid.
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
||| [CR#701.23a] searches for a card IN A ZONE, so a search that names none names no act -- the same refusal a bare "you discarded" takes for leaving out the card [CR#701.9a].
public export
badBareSearchLookback : Unspellable (Condition []) (\ok =>
  Happened (VerbedAct "Search") You Lookback.ThisWay Nothing {cw = LeftBare {ok}})
badBareSearchLookback Oh impossible


||| "if you shuffled your graveyard this way"
||| [CR#701.24a] randomizes a LIBRARY or a face-down pile; a graveyard is neither, so the act names it nowhere.
public export
badShuffleLocusAtGraveyard : Unspellable (Condition []) (\ok =>
  Happened (VerbedAct "Shuffle") You Lookback.ThisWay
           (Just (AtZone (Macros.graveyardOf You) {ok = ok})))
badShuffleLocusAtGraveyard Oh impossible


||| "If that mana is spent on an instant spell, it gains haste until end of turn."
public export
badResolvedInstant : Unspellable (Noun [] Object) (\ok =>
  ResolvedPermanent (Macros.a (And [HasType Instant, Macros.spell])) {pm = ok})
badResolvedInstant Oh impossible


||| "this creature, once it resolves"
||| The read is the stack-to-battlefield transition [CR#110.4b,608.3a]; a mention already on the battlefield has made it, and naming what it becomes names nothing further.
public export
badResolvedOnBattlefield : Unspellable (Noun [] Object) (\ok =>
  ResolvedPermanent Macros.thisCreature {zn = ok})
badResolvedOnBattlefield OnTheStack impossible


||| "if it entered from the battlefield"
||| [CR#400.7] makes a zone change a move from one zone to ANOTHER, so the battlefield is the one zone a permanent cannot enter it from.
public export
badEntryOriginBattlefield : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo Entry Lookback.ThisTurn
             (Just (FromZones (FromZone [Macros.battlefieldZ]) Nothing {ok = ok})))
badEntryOriginBattlefield Oh impossible


||| "If you control an artifact, create a token."
||| A coordination offers two clauses or more; at one it spells what the bare condition spells.
public export
badSingletonConjunction : Unspellable (Condition []) (\ok =>
  AndCond [Exists (And [Macros.artifact, ControlledBy You])] {tw = ok})
badSingletonConjunction Oh impossible


||| "If you control an artifact and an enchantment, and you control a land, …"
||| English writes the flat serial list, so a nested coordination is the same clause written twice.
public export
badNestedConjunction : Unspellable (Condition []) (\ok =>
  AndCond [ AndCond [ Exists (And [Macros.artifact, ControlledBy You])
                    , Exists (And [Macros.enchantment, ControlledBy You]) ]
          , Exists (And [Macros.land, ControlledBy You]) ] {fl = ok})
badNestedConjunction Oh impossible


||| "Unless you control an artifact and an enchantment, you may cast this spell without paying its mana cost."
||| "Unless" reads a negated condition, and a coordination is not negated at its own frame.
public export
badUnlessConjunction : Unspellable (StaticEffect []) (\ok =>
  Conditionally (AndCond [ Exists (And [Macros.artifact, ControlledBy You])
                         , Exists (And [Macros.enchantment, ControlledBy You]) ])
                (AltCost This Nothing) Unless {mk = ok})
badUnlessConjunction MkMarkingOk impossible


||| "Counter target spell unless its controller pays {2}."
||| A scaled payment scales; a literal magnitude is what the fixed symbol run already spells.
public export
badLiteralScaledMana : Unspellable (Cost []) (\ok =>
  ScaledMana GenericUnit (Lit 2) {fe = ok})
badLiteralScaledMana Oh impossible


||| "Counter target spell unless its controller pays the number of artifacts you control."
||| The scaled payment scales a per-unit: a numeral in braces with the counted phrase after "for each".
public export
badBareCountScaledMana : Unspellable (Cost []) (\ok =>
  ScaledMana GenericUnit (CountOf (And [Macros.artifact, ControlledBy You])) {fe = ok})
badBareCountScaledMana Oh impossible


||| "Target creature becomes a Zombie named Bob in addition to its other types."
||| A name is set and never added; [CR#205.1b] adds types and leaves the rest.
public export
badNamedAddition : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [] (MkTypeLine [creatureType "Zombie"] []) [] (Just "Bob"))
                   Nothing {ok = ok})
badNamedAddition Oh impossible


||| "Target creature becomes a black black Zombie in addition to its other colors and types."
public export
badRepeatedAdditionColor : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [Black, Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                   Nothing {ok = ok})
badRepeatedAdditionColor Oh impossible


||| "Target creature becomes an artifact artifact in addition to its other types."
public export
badRepeatedAdditionType : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [] (MkTypeLine [] [Artifact, Artifact]) [] Nothing)
                   Nothing {ok = ok})
badRepeatedAdditionType Oh impossible


||| "For each of target creature, its controller draws a card."
||| An element binder needs a group to take elements of, so its domain is plural.
public export
badSingletonForEach : Unspellable (Effect []) (\ok =>
  ForEachOf (Macros.target Macros.creature) (Draw You (Lit 1)) {pl = ok})
badSingletonForEach Refl impossible


||| "if you activated a loyalty ability this turn"
||| An activation is an activation of something: with no complement the clause names no event.
public export
badBareActivationLookback : Unspellable (Condition []) (\ok =>
  Happened AbilityActivation You Lookback.ThisTurn Nothing {cw = LeftBare {ok = ok}})
badBareActivationLookback Oh impossible


||| "At the beginning of you's upkeep, draw a card."
||| The possessive slot takes the attachment anaphor; every other possessor is a word.
public export
badNounPossessorYou : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Upkeep (ByNoun You {pn = ok})) [] Nothing [] Nothing Nothing Nothing (Macros.draw You (Lit 1)))
badNounPossessorYou AttachedPossessor impossible


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
||| [CR#702.84a] states the keyword as "Unearth [cost]", so the parameter is never absent.
public export
badBareUnearth : Unspellable Ability (\ok =>
  KeywordAbility "Unearth" Nothing {pf = ok})
badBareUnearth Oh impossible


||| "Retrace {1}"
||| [CR#702.81a] writes retrace bare, so no cost stands beside the word.
public export
badCostedRetrace : Unspellable Ability (\ok =>
  KeywordAbility "Retrace" (Just (ParamCost (Mana [Macros.generic 1])))
                 {pf = ok})
badCostedRetrace Oh impossible


||| "Unearth {B}"
public export
badUnearthOnSpellCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Unearth" (Just (ParamCost (Mana [Macros.pip Black])))] Nothing {tx = ok})
badUnearthOnSpellCard Oh impossible


||| "Flashback {2}{U}"
public export
badFlashbackOnPermanentCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [KeywordAbility "Flashback" (Just (ParamCost (Mana [Macros.generic 2, Macros.pip Blue])))]
       Nothing {tx = ok})
badFlashbackOnPermanentCard Oh impossible


||| "Creature cards in your graveyard have warp {2}."
||| [CR#702.185a] leaves warp's statics on the stack, so the grant reaches no graveyard card [CR#113.6e].
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

||| The six-verb refusal's own fact. A joined phrase places nothing —
public export
anyTargetIsPlaceless :
  nounZone {bs = []} (Macros.target Macros.anyTarget) = Nothing
anyTargetIsPlaceless = Refl

public export
anyTargetTakesDamage : DamageRecipient (Macros.target {bs = []} Macros.anyTarget)
anyTargetTakesDamage = JoinTakes

||| The mixed group binds nothing jointly [CR#109.5]: "you" is deixis and
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

||| "This deals 2 damage to any target. If a mana ability is dealt damage this way, you draw a card."
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
||| [CR#209.1] has each planeswalker card print a loyalty number in its lower right corner.
public export
badPlaneswalkerNoLoyalty : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary] (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [] Nothing {bx = ok})
badPlaneswalkerNoLoyalty MkCardBox impossible

||| a planeswalker card printing "3/3" where its loyalty number goes
||| [CR#209.1] and [CR#208.1] name the same lower right corner, so one face writes one of the two.
public export
badPlaneswalkerPtBox : Unspellable Card (\ok =>
  Macros.cardOf "" (Just [Macros.pip Blue]) [Legendary] (MkTypeLine [planeswalkerType "Jace"] [Planeswalker])
       [] (Macros.printedBox (Just (3, 3))) {bx = ok})
badPlaneswalkerPtBox MkCardBox impossible

||| a battle card printed with no defense
||| [CR#210.1] has each battle card print a defense number in its lower right corner.
public export
badBattleNoDefense : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [battleType "Siege"] [Battle]) [] Nothing {bx = ok})
badBattleNoDefense MkCardBox impossible

||| an adventurer card whose inset frame is a plain instant, naming no Adventure
public export
badUnnamedAdventure : Unspellable Card (\ok =>
  Adventurer (MkFace "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature]) []
                     (Macros.printedBox (Just (1, 1))))
             (MkFace "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant]) [] Nothing)
             {ai = ok})
badUnnamedAdventure Oh impossible

||| a flip card whose upside-down half is an instant
public export
badSpellFlipHalf : Unspellable Card (\ok =>
  FlipCard (MkFace "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) []
                   (Macros.printedBox (Just (1, 1))))
           (MkAltFace "" [] (MkTypeLine [] [Instant]) [] Nothing)
           {ah = ok})
badSpellFlipHalf Oh impossible

||| an activated ability printed on a conspiracy card
public export
badConspiracyActivated : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Conspiracy])
       [ Macros.activated (Mana [Macros.generic 1]) (Macros.draw You (Lit 1)) ] Nothing {tx = ok})
badConspiracyActivated Oh impossible

||| a static ability printed on a dungeon card
public export
badDungeonStatic : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Dungeon])
       [ Static (Macros.entersWithCounters Macros.thisCreature (Lit 1)
                   Macros.plusOnePlusOne) ] Nothing {tx = ok})
badDungeonStatic Oh impossible


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
badSingularAggregateOf : Unspellable (Amount []) (\ok =>
  AggregateOf SumOf (CharAxis Power) (Macros.target Macros.creature)
              {pl = ok})
badSingularAggregateOf Refl impossible

||| "the greatest life total among all creatures"
public export
badAggregateOfWrongSort : Unspellable (Amount []) (\ok =>
  AggregateOf MaxOf (PlayerStatAxis LifeTotal) (Macros.allOf Macros.creature)
              {sc = ok})
badAggregateOfWrongSort Refl impossible

||| "up to X | Draw a card."
public export
badAmountRollRow : Unspellable (Effect []) (\ok =>
  Sequentially [(Macros.rollDice You 1 20),
                ResultsTable [MkRollRow (UpToOf (LetterVal X))
                                        (Macros.draw You (Lit 1)) {lt = ok}]])
badAmountRollRow Oh impossible

||| "This deals 3 damage to target player or planeswalker. That player or that creature's controller discards a card."
||| A union mention's targets are the objects and/or players its head describes [CR#115.1], and this head's object half can only be a planeswalker, so the split read's class arm names nothing.
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
  CouldBlock (Macros.target (And [Macros.creature,
                                  InZone (Macros.graveyardOf You)])) {zn = ok})
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

||| The readings those pins leave standing, so none passes for want of a
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

||| The discourse after "Flip a coin."
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

||| The discourse after "Choose up to one creature. Destroy the rest."
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
  CompareOver Opponent (CountOf (And [Macros.land, ControlledBy You]))
              Greater (CountOf (And [Macros.land, ControlledBy (They {ok})])))
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
  KeywordAbility "Echo" Nothing {pf = ok})
badBareEcho Oh impossible

||| The discourse after "Whenever this creature's cumulative upkeep is
||| paid" -- the passive payment header, which writes no payer.
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

||| The discourse after "Whenever you pay this enchantment's cumulative
||| upkeep" -- the active payment header.
public export
afterKeywordCostPayment : Bindings
afterKeywordCostPayment =
  eventAfter (the (GameEvent [])
    (PaysCost (Just You) Paid Macros.thisEnchantment "CumulativeUpkeep"))

||| "Whenever you pay this enchantment's cumulative upkeep, put that many counters on it."
public export
badKeywordCostPaymentThatMuch :
  Unspellable (Amount ProofsG.afterKeywordCostPayment) (\ok => ThatMuch {ok})
badKeywordCostPaymentThatMuch Refl impossible

||| The reading those pins leave standing, so none passes for want of a
public export
youPaidLifeThisTurn : Condition []
youPaidLifeThisTurn = Happened LifePayment You Lookback.ThisTurn Nothing

||| The discourse after "Look at the top card of your library. Shuffle."
public export
afterShuffledLook : Bindings
afterShuffledLook =
  effIntro (the (Effect [])
    (Sequentially [ Macros.lookAt Macros.topCard, Macros.shuffle ]))

||| "Look at the top card of your library. Shuffle. Put that card into your hand."
public export
badReadsShuffledLibraryCard :
  Unspellable (Noun ProofsG.afterShuffledLook Object) (\ok => That CardW {ok})
badReadsShuffledLibraryCard Refl impossible

||| The discourse after "Look at the top card of your library. Shuffle
||| this card into its owner's library."
public export
afterShuffledIntoLook : Bindings
afterShuffledIntoLook =
  effIntro (the (Effect [])
    (Sequentially [ Macros.lookAt Macros.topCard, Macros.shuffleInto You This ]))

||| "Look at the top card of your library. Shuffle this card into its owner's library. Put that card into your hand."
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
  PutCounters (Lit 1) (ChosenKind [Macros.plusOnePlusOne, Poison])
              (Macros.target Macros.creature) {sc = ok})
badMixedScopeCounterMenu Oh impossible

||| "Put a counter of each of those kinds on target creature."
||| Nothing has put counters, so "those kinds" reads no antecedent.
public export
badThoseKindsUnannounced : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (ThoseKinds {ok}) (Macros.target Macros.creature))
badThoseKindsUnannounced Refl impossible

||| "For each color among permanents you control, add one mana of that color"
public export
badRepeatedCarriesNoColor : Unspellable (Effect []) (\ok =>
  Repeated (DistinctCount ColorAxis (Macros.allOf (And [Permanent, ControlledBy You])))
           (AddMana You (Lit 1) (OfChosenColor Nothing {cq = ok}) []))
badRepeatedCarriesNoColor Refl impossible

||| "For each color among permanents you control, … of that creature type."
public export
badAxisValueCrossing : Unspellable (Effect []) (\ok =>
  ForEachKindOf ColorAxis (Just (Macros.allOf (And [Permanent, ControlledBy You])))
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
||| [CR#701.22a] scries a NUMBER of cards and carries none off, so the act has no patient the event could name.
public export
badScryPatient : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Scry"
              (Just (Macros.a (InZone (ZoneAt Library Bare)))) Nothing False
              {pt = ok})
badScryPatient ActOn impossible


||| "Whenever discards a card, …"
||| An act announced of no one: the active names its actor and the passive its patient, and this writes neither.
public export
badVoicelessAct : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Scry" Nothing Nothing False {vc = ok})
badVoicelessAct ActiveAct impossible
badVoicelessAct PassiveAct impossible
badVoicelessAct IntransitiveAct impossible


||| "Whenever a card is put, …"
public export
badPassiveWithoutParticiple : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Put" (Just (Macros.a IsCard)) Nothing False {vc = ok})
badPassiveWithoutParticiple PassiveAct impossible
badPassiveWithoutParticiple IntransitiveAct impossible


||| "Whenever a card is milled into a Phyrexian, …"
public export
badBecomesWithoutIntransitive : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Mill" (Just (Macros.a (InZone (ZoneAt Library Bare))))
              (Just (HasSubtype (creatureType "Phyrexian"))) False {bc = ok})
badBecomesWithoutIntransitive BecomesInto impossible


||| "Whenever a card in a graveyard is destroyed, …"
||| [CR#701.8a] moves a permanent from the BATTLEFIELD, and [CR#701.8b] says a permanent put into a graveyard any other way hasn't been destroyed.
public export
badDestroyInGraveyard : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Destroy"
              (Just (Macros.a (InZone Macros.graveyardZ))) Nothing False {zn = ok})
badDestroyInGraveyard Oh impossible


||| "Whenever you discard a permanent you control, …"
||| [CR#701.9a] discards a card from its owner's HAND; a permanent on the battlefield is in no one's hand to discard.
public export
badDiscardFromBattlefield : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Discard"
              (Just (Macros.a (InZone Macros.battlefieldZ))) Nothing False {zn = ok})
badDiscardFromBattlefield Oh impossible


||| "if you dealt damage to an opponent this turn"
||| [CR#120.1] gives the dealing to an object and makes that object the damage's source; a player is never what dealt damage.
public export
badPlayerDamageDealer : Unspellable (Condition []) (\ok =>
  Happened DamageDealing You Lookback.ThisTurn Nothing {sb = ok})
badPlayerDamageDealer MkLookbackSubject impossible


||| "player that targets this creature"
||| [CR#115.1] has a player CHOOSE the targets of a spell or ability, and [CR#115.1a,115.1c,115.1d] give the word "target" to the spell or the ability alone, so no player is ever what targets.
public export
badPlayerTargeter : Unspellable (Predicate [] Player) (\ok =>
  Targets Macros.thisCreature SomeTarget {tr = ok})
badPlayerTargeter SpellTargets impossible
badPlayerTargeter AbilityTargets impossible
badPlayerTargeter (EitherTargets _ _) impossible


||| "Whenever you become the target of a player, …"
||| The same rule at the other seat, where the relation is one: [CR#115.1] has the player do the choosing, so the thing that targeted is the spell or the ability it chose for [CR#115.1a,115.1c,115.1d].
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
||| [CR#701.12c]'s "the other player" is one player, so an exchange runs between two; a plural arm puts a whole group on one side and leaves that phrase with no other to equal.
public export
badExchangePluralParty : Unspellable (Effect []) (\ok =>
  ExchangeLife (BothOf You (PlayerGroup YourOpponents)) {tp = ok})
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
||| Countering removes the spell from the stack [CR#701.6a], so no second countering of it can occur.
public export
badCounterBoundTwice : Unspellable (StaticEffect []) (\ok =>
  Deontic This Forbid ["Counter"] Patient (Just (MoreThan (Lit 1))) NoDeonticPatient
          Nothing NoDeonticRider {bd = ok})
badCounterBoundTwice Oh impossible


||| "You may spend mana as though it weren't a creature."
||| [CR#609.4b] makes the spend permission's premise a statement about mana, and no description of an OBJECT is one; the premise would say the mana counts as something no cost is paid with.
public export
badObjectPremiseAtSpend : Unspellable (StaticEffect []) (\ok =>
  Deontic You Permit ["Spend"] Agent Nothing NoDeonticPatient
          (Just (AsThoughOf (Not Macros.creature))) NoDeonticRider {at = ok})
badObjectPremiseAtSpend Oh impossible


||| "Whenever you sacrifice a creature for mana, …"
||| [CR#106.12] defines "for mana" for ONE act and defines it as a second act -- activating a mana ability that includes the {T} symbol -- so the adjunct is a rule attached to tapping and not an adverb any verb may take.
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
||| [CR#701.27a] turns a PERMANENT over, and [CR#110.1] leaves a card in a graveyard no permanent at all, so there is nothing on that card's other side for the act to turn up.
public export
badTurnOverOffField : Unspellable (Effect []) (\ok =>
  TurnOver (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
           {ok})
badTurnOverOffField OnField impossible


||| "Return target creature card from your graveyard to your hand transformed."
public export
badTransformedArrivalOffField : Unspellable (Effect []) (\ok =>
  Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
       Macros.handZ [EntersTransformed] {rf = ok})
badTransformedArrivalOffField Oh impossible


||| "Reveal the top five cards of your library. An opponent separates those cards into two piles. Put those cards into your hand."
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
||| The partitive asks the same question its demonstrative does: [CR#700.3] makes piles only where a clause groups objects into them, and a card that groups nothing has no parts for "one pile" to take one of.
public export
badPilePartitiveWithoutAPartition : Unspellable (Effect []) (\ok =>
  Macros.move (Macros.onePile {ok = ok}) Macros.handZ)
badPilePartitiveWithoutAPartition Refl impossible


||| "Reveal the top five cards of your library. Put all cards in those cards into your hand."
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


||| "Reveal the top five cards of your library. An opponent separates those cards into two piles. Turn those piles face down."
public export
badPileFaceAsAStatus : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
       [ Spell (Sequentially
                  [ Macros.revealCards (Macros.topSlice (Lit 5))
                  , SeparateIntoPiles Macros.anOpponent Them 2 []
                  , SetStatus FaceDown (Those PileW) {ok = ok} ]) ]
       Nothing)
badPileFaceAsAStatus OnField impossible


||| "If a triggered ability of a permanent you control would trigger, draw a card instead."
public export
badTriggeringReplaced : Unspellable (StaticEffect []) (\ok =>
  Intercepts (Triggers (Macros.a (And [ AbilityHead AnyTriggered
                                      , AbilityOf (Macros.a (And [Permanent, ControlledBy You])) ])))
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
       Nothing {dr = ok})
badDoorHeaderOffSharedLine Oh impossible


||| "unlock a locked door of a Room card in your graveyard"
||| [CR#709.5c] gives the unlocked designations to "a permanent on the battlefield", so a half off the battlefield has no designation to be given and no lock state to be in.
public export
badUnlockDoorOffBattlefield : Unspellable (Effect []) (\ok =>
  Unlock (DoorOf (Just Locked)
            (Macros.a (And [HasSubtype (enchantmentType "Room"),
                            InZone Macros.graveyardZ])) {zn = ok}))
badUnlockDoorOffBattlefield OnField impossible


||| "unlock a locked door of this" leaves the source placeless; unlocked designations belong to battlefield permanents [CR#709.5c].
public export
badDoorOfBareThis : Unspellable (Effect []) (\ok =>
  Unlock (DoorOf (Just Locked) This {zn = ok}))
badDoorOfBareThis OnField impossible


||| "Sacrifice a creature."
||| A mandatory clause with neither continuation denotes exactly its body, and the row is the two arms.
public export
badIfDoneWithNeitherArm : Unspellable (Effect []) (\ok =>
  IfDone (Macros.sacrifice You (Macros.a Macros.creature)) Nothing Nothing {br = ok})
badIfDoneWithNeitherArm Oh impossible


||| "This creature deals 3 damage to any target. If you do, draw a card."
||| No player takes the action, so [CR#603.12]'s pro-verb has no subject to inflect and "if you do" abbreviates nothing.
public export
badIfDoneOverAgentlessBody : Unspellable (Effect []) (\ok =>
  IfDone (DealDamage Macros.thisCreature (Lit 3) (Macros.target Macros.anyTarget))
         (Just (Macros.draw You (Lit 1))) Nothing {en = ok})
badIfDoneOverAgentlessBody Oh impossible


||| "Take an extra turn after this one. If you do, draw a card."
||| The clause schedules its action rather than taking it, so nothing has "occurred earlier during the resolution" [CR#603.12] for the arm to test.
public export
badIfDoneOverScheduledBody : Unspellable (Effect []) (\ok =>
  IfDone (ExtraTurn You (Lit 1)) (Just (Macros.draw You (Lit 1))) Nothing {en = ok})
badIfDoneOverScheduledBody Oh impossible


||| "create a legendary legendary 20/20 black Avatar creature token"
||| A supertype is a property an object has or lacks [CR#205.4b], so the word twice on a token is one fact twice, exactly as it is on a card.
public export
badTokenDuplicateSupertype : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1)
    (MkSupertypedToken (Just (Lit 20 ** Lit 20)) [Black] [Legendary, Legendary]
       (MkTypeLine [creatureType "Avatar"] [Creature]) [] (Just "Marit Lage"))
    {tc = ok})
badTokenDuplicateSupertype Oh impossible


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
  Macros.ifSo (Exists AnyPlayer)
    (Macros.ifSo (Exists AnyPlayer) (KeepsUnspentMana You (UnspentMana Nothing)))

public export
nestedTurnPartWindows : StaticEffect []
nestedTurnPartWindows =
  OnlyDuring Combat Nothing
    (OnlyDuring MainPhase Nothing (KeepsUnspentMana You (UnspentMana Nothing)))

public export
repeatWithIndependentException : Repetition []
repeatWithIndependentException = AgainExcept (Exists AnyPlayer)

public export
voteStartingWithSpecifiedPlayer : Effect []
voteStartingWithSpecifiedPlayer =
  VoteStarting Macros.anOpponent (Macros.each AnyPlayer) (ByLabel ["alpha", "beta"])

public export
oneWayResultShift : Effect []
oneWayResultShift =
  Sequentially [(Macros.rollDice You 1 6), ShiftResultOneWay True (Lit 1)]

public export
objectScopedChaos : Effect []
objectScopedChaos = ChaosEnsuesFor Macros.thisRoom

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
  GetsAdditionalPartAfter You Upkeep (Lit 1) MainPhase

||| [CR#709.5j] “This door” requires a door of this permanent.
public export
badDelayedDoorDeixis : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Instant])
    [Spell (Delayed (UnlocksDoor You ThisDoor) [] Nothing (Macros.draw You (Lit 1))
                    {so = DelayOnce})]
    Nothing {dr = ok})
badDelayedDoorDeixis Oh impossible

public export
joinedDealerDamageComplement :
  LookbackComplement DamageDealing Object (Object \/ Player)
joinedDealerDamageComplement = MkLookbackComplement

public export
lastChosenPlayerRead :
  Predicate [choiceB PlayerC, choiceB PlayerC] Player
lastChosenPlayerRead = LastChosenPlayer

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
             {so = DelayOnce}) = True
delayedDoorTraversal = Refl

public export
distributiveGroupSurvives : Effect []
distributiveGroupSurvives =
  Sequentially
    [ DoesGroup (Macros.each Opponent) "Shuffle" (Shuffle They)
    , ChangeLife (Those PlayerW) (Down (Lit 1))
    ]

public export
secondChooserDevotionRead :
  Amount [qualityB Color, qualityB Color]
secondChooserDevotionRead =
  Devotion You LastChosenColor Nothing

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
