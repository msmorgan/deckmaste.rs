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
  KeywordAbility CumulativeUpkeep Nothing {pf = ok})
badBareCumulativeUpkeep Oh impossible


||| "Cumulative upkeep {2}" printed on a sorcery.
||| [CR#702.24a] expands the keyword into clauses about a permanent on the battlefield.
public export
badCumulativeUpkeepOnSpell : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility CumulativeUpkeep (Just (ParamCost (Mana [Macros.generic 2])))] Nothing {tx = ok})
badCumulativeUpkeepOnSpell Oh impossible


||| "a cumulative upkeep counter"
||| [CR#122.1b] closes the keyword-counter list by enumeration, and cumulative upkeep is not on it.
public export
badCumulativeUpkeepCounter : Unspellable CounterKind (\ok =>
  KeywordCounter CumulativeUpkeep {ok = ok})
badCumulativeUpkeepCounter Oh impossible


||| "Creatures you control get +1/+1. The same is true for menace and trample."
||| The trailer extends a keyword-conditional line, so the base sentence must name a keyword.
public export
badKeywordListOnPlainLine : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Gets (AllOf Macros.creatureYouControl)
                                (PtUp (Lit 1)) (PtUp (Lit 1))))
                  [Menace, Trample]
                  {ex = Builtin.fst ok, lk = Builtin.snd ok})
badKeywordListOnPlainLine (Oh, _) impossible


||| "This creature has flying as long as a card exiled with it has flying. The same is true for."
||| The trailer's whole content is the words it names, and an empty list names none.
public export
badEmptyKeywordList : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword Flying]))
                             (Gains Macros.thisCreature (KeywordAbility Flying Nothing)) AsLongAs))
                  [] {lk = ok})
badEmptyKeywordList Oh impossible


||| "This creature has flying as long as a card exiled with it has flying. The same is true for menace, flying, and trample."
||| The list is disjoint from the base; a repeat would write the same sentence twice.
public export
badKeywordListRepeatingBase : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword Flying]))
                             (Gains Macros.thisCreature (KeywordAbility Flying Nothing)) AsLongAs))
                  [Menace, Flying, Trample] {lk = ok})
badKeywordListRepeatingBase Oh impossible


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


||| "Counter target spell cast from the battlefield."
||| HELD: [CR#601.2a] moves the card "from where it is" and names no excluded zone, so this row is left for the zone round.
public export
badCastFromBattlefield : Unspellable (Predicate [] Object) (\ok =>
  CastFrom Macros.battlefieldZ {pf = ok})
badCastFromBattlefield Oh impossible


||| "Counter target spell cast from the stack."
||| [CR#112.1] moves the card to the stack from the zone it was in, which is never the stack.
public export
badCastFromStack : Unspellable (Predicate [] Object) (\ok =>
  CastFrom Macros.stackZ {pf = ok})
badCastFromStack Oh impossible


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
                (AltCost Nothing) Unless {mk = ok})
badUnlessConjunction MkMarkingOk impossible


||| "Counter target spell unless its controller pays {2}."
||| A scaled payment scales; a literal magnitude is what the fixed symbol run already spells.
public export
badLiteralScaledMana : Unspellable (Cost []) (\ok =>
  ScaledMana (Lit 2) {fe = ok})
badLiteralScaledMana Oh impossible


||| "Counter target spell unless its controller pays the number of artifacts you control."
||| The scaled payment scales a per-unit: a numeral in braces with the counted phrase after "for each".
public export
badBareCountScaledMana : Unspellable (Cost []) (\ok =>
  ScaledMana (CountOf (And [Macros.artifact, ControlledBy You])) {fe = ok})
badBareCountScaledMana Oh impossible


||| "Target creature becomes a Zombie named Bob in addition to its other types."
||| A name is set and never added; [CR#205.1b] adds types and leaves the rest.
public export
badNamedAddition : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [] (MkTypeLine [Zombie] []) [] (Just "Bob"))
                   Nothing {un = ok})
badNamedAddition Oh impossible


||| "Target creature becomes a black black Zombie in addition to its other colors and types."
||| A colour list is a set [CR#105.2], so no phrase writes a colour twice.
public export
badRepeatedAdditionColor : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [Black, Black] (MkTypeLine [Zombie] []) [] Nothing)
                   Nothing {cd = ok})
badRepeatedAdditionColor Oh impossible


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
  Triggered At (BeginningOf Upkeep (ByNoun You {pn = ok})) Nothing Nothing Nothing Nothing Macros.drawACard)
badNounPossessorYou AttachedPossessor impossible


||| "Put those cards on the top or bottom of your library in any order."
||| [CR#401.4] arranges cards sharing one position; a disjunction names two
||| ends, so there is no single pile to order.
public export
badDisjunctionOrdered : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt (EitherEnd Nothing) (Just AnyOrder) Nothing {af = ok} Bare)
badDisjunctionOrdered Oh impossible


||| "zeroth from the top"
||| [CR#401.7] counts library positions from the top card, which is the
||| first, so a library has no zeroth position to put a card into.
public export
badZerothFromTop : Unspellable LibOrdinal (\ok => Nth 0 {nz = ok})
badZerothFromTop ItIsSucc impossible


||| "if there is no monstrous creature"
||| The absence check reads a player-held designation [CR#725.1]; an
||| object-held marker is described on the object that holds it.
public export
badNoHolderOnObject : Unspellable (Condition []) (\ok =>
  NoHolder Monstrous {sc = ok})
badNoHolderOnObject Refl impossible


||| "your monarch"
||| Only the card-scope designation is a possessed noun [CR#903.3]; the
||| monarch is held by a player and read as a description.
public export
badPossessedMonarch : Unspellable (Noun [] Object) (\ok =>
  Designated Monarch You {sc = ok})
badPossessedMonarch Refl impossible



||| "Unearth"
||| [CR#702.84a] states the keyword as "Unearth [cost]", so the parameter is never absent.
public export
badBareUnearth : Unspellable Ability (\ok =>
  KeywordAbility Unearth Nothing {pf = ok})
badBareUnearth Oh impossible


||| "Retrace {1}"
||| [CR#702.81a] writes retrace bare, so no cost stands beside the word.
public export
badCostedRetrace : Unspellable Ability (\ok =>
  KeywordAbility Retrace (Just (ParamCost (Mana [Macros.generic 1])))
                 {pf = ok})
badCostedRetrace Oh impossible


||| "Unearth {B}" printed on an instant.
||| [CR#702.84a] returns the card to the battlefield, which [CR#110.4] denies an instant card.
public export
badUnearthOnSpellCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [KeywordAbility Unearth (Just (ParamCost (Mana [Macros.pip Black])))] Nothing {tx = ok})
badUnearthOnSpellCard Oh impossible


||| "Flashback {2}{U}" printed on an artifact.
||| [CR#702.34a] permits the graveyard cast only if the resulting spell is an instant or sorcery.
public export
badFlashbackOnPermanentCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [KeywordAbility Flashback (Just (ParamCost (Mana [Macros.generic 2, Macros.pip Blue])))]
       Nothing {tx = ok})
badFlashbackOnPermanentCard Oh impossible


||| "Creature cards in your graveyard have warp {2}."
||| [CR#702.185a] leaves warp's statics on the stack, so the grant reaches no graveyard card [CR#113.6e].
public export
badWarpGrantInGraveyard : Unspellable Ability (\ok =>
  Static (Gains (AllOf (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting Warp (Mana [Macros.generic 2]))
                {ok = ok}))
badWarpGrantInGraveyard Oh impossible


-- A WITNESS, not a pin: with the join a plain constructor, no cross-kind
-- pair is refused any more, so the pairs the old join gate would not
-- admit are simply written.

||| "Change the target of target spell or ability with a single target."
||| (Bolt Bend.) `Object \/ Ability` is writable, and its payload carries
||| each half separately: the spell half is a card on the stack [CR#112.1],
||| while `AbilityP` carries no zone field to place the ability half with.
||| No head spells this join yet
||| -- `Macros.kindJoin` joins a player -- so the witness is the payload.
public export
spellOrAbilityJoin : Payload (Object \/ Ability)
spellOrAbilityJoin = JoinP (ObjectP Nothing (Just Stack) Nothing Nothing) AbilityP

||| ...and the order reads either half back: "counter that ability" after
||| "target spell or ability" resolves `Ability` against the join.
public export
abilityUnderSpellOrAbility : So (kindLte Ability (Object \/ Ability))
abilityUnderSpellOrAbility = kindLteJoinR Object Ability

||| ...and the joined head's class reaches the binding it mints: "target
||| creature or player" leaves an Object half typed `Creature`, which is
||| what the demonstrative echo reads back off `JoinP`. A join that forgot
||| its own head would read `Nothing` here.
public export
joinedCreatureTy :
  tyOfThat JoinW (effIntro {bs = []}
    (DealDamage This (Lit 3)
       (Macros.target (Macros.kindJoin AnyPlayer Macros.creature))))
  = Just Creature
joinedCreatureTy = Refl

||| The seven-verb refusal's own fact. A joined phrase places nothing —
||| [CR#400.1] makes a zone a place where objects can be and [CR#109.1]
||| lists what an object is — so destroy, exile, tap, untap, return,
||| counter and sacrifice each refuse it at their `Noun bs Object` slot,
||| with no rule written for the purpose, while the damage clause admits it
||| because damage asks for no zone [CR#120.1].
public export
anyTargetIsPlaceless :
  nounZone {bs = []} (Macros.target Macros.anyTarget) = Nothing
anyTargetIsPlaceless = Refl

||| ...and the damage half of the same fact: the joined phrase IS a damage
||| recipient, by the one row that replaced three.
public export
anyTargetTakesDamage : DamageRecipient (Macros.target {bs = []} Macros.anyTarget)
anyTargetTakesDamage = JoinTakes

||| The mixed group binds nothing jointly [CR#109.5]: "you" is deixis and
||| mints nothing, and a coordination of two phrases mints each arm's
||| bindings and no third one. Structural, not stipulated — no constructor
||| writes the joint binding, which is why nothing reads the pair back.
public export
youAndBindsNothing :
  nounDelta {bs = []} (Macros.youAnd Macros.thisCreature) = []
youAndBindsNothing = Refl

||| "You draw a card. If a player is dealt damage this way, you draw a card."
||| "This way" reads an instruction the text has already written
||| [CR#608.2c], and a draw dealt no damage anywhere in scope for it to
||| read back. The licence is loose about WHICH damage; it still needs one.
public export
badDealtThisWayNoDamage : Unspellable (Effect []) (\ok =>
  Sequentially [ Draw You (Lit 1)
               , If (DealtThisWay AnyPlayer {wy = ok}) (Draw You (Lit 1)) Nothing ])
badDealtThisWayNoDamage Oh impossible

||| "This deals 2 damage to any target. If a mana ability is dealt damage
||| this way, you draw a card."
||| Damage is dealt to battles, creatures, planeswalkers and players
||| [CR#120.1]; an ability is none of them, so the refinement narrows to a
||| kind no damage could have reached.
public export
badDealtThisWayAbility : Unspellable (Effect []) (\ok =>
  Sequentially [ DealDamage This (Lit 2) (Macros.target Macros.anyTarget)
               , If (DealtThisWay IsManaAbility {rk = ok}) (Draw You (Lit 1))
                    Nothing ])
badDealtThisWayAbility Oh impossible

||| "Flip a coin. Draw that many cards."
||| [CR#705.2] gives a flip a face and, when the flipper called it, a
||| winner, and the rules give it nothing else — no number — so a
||| quantity read after one has no value to name. [CR#706.2] is where a
||| randomiser does leave a number, and "the result" is that read.
public export
badThatMuchAfterFlip : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.flipACoin, Draw You (ThatMuch {ok})])
badThatMuchAfterFlip Refl impossible

||| "If you win the flip, draw a card." with no coin flipped.
||| [CR#705.2] has the flipper call heads or tails and win the flip when
||| the call matches the result, so the arm reads a flip the text made;
||| with none written there is nothing to have been won.
public export
badFlipArmWithoutFlip : Unspellable (Effect []) (\ok =>
  If (FlipCalled You WinsFlip {fl = ok}) (Draw You (Lit 1)) Nothing)
badFlipArmWithoutFlip Oh impossible

||| "1—9 | Draw a card." with no roll before it.
||| [CR#706.3a] makes each striation mean "If the result was in this
||| range, [effect]", and [CR#706.2] makes the result the number the die
||| came up; with no roll written there is no result to range over.
public export
badTableWithoutRoll : Unspellable (Effect []) (\ok =>
  ResultsTable [Macros.rollRow (Macros.fromTo 1 9) (Draw You (Lit 1))] {ok})
badTableWithoutRoll Refl impossible

||| "Roll a d0."
||| [CR#706.1a] has an N-sided die carry N equally likely outcomes
||| numbered from 1 to N, N a positive integer, so a nought-sided die has
||| no face to come up and the instruction specifies no kind of die
||| [CR#706.1].
public export
badNoughtSidedDie : Unspellable (Effect []) (\ok =>
  RollDice You (Lit 1) 0 {nz = ok})
badNoughtSidedDie ItIsSucc impossible

||| a planeswalker card printed with no starting loyalty
||| [CR#209.1] has each planeswalker card print a loyalty number in its lower right corner.
public export
badPlaneswalkerNoLoyalty : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [Legendary] (MkTypeLine [Jace] [Planeswalker])
       [] Nothing {bx = ok})
badPlaneswalkerNoLoyalty MkCardBox impossible

||| a planeswalker card printing "3/3" where its loyalty number goes
||| [CR#209.1] and [CR#208.1] name the same lower right corner, so one face writes one of the two.
public export
badPlaneswalkerPtBox : Unspellable Card (\ok =>
  Macros.cardOf "" (Just [Macros.pip Blue]) [Legendary] (MkTypeLine [Jace] [Planeswalker])
       [] (Macros.printedBox (Just (3, 3))) {bx = ok})
badPlaneswalkerPtBox MkCardBox impossible

||| a battle card printed with no defense
||| [CR#210.1] has each battle card print a defense number in its lower right corner.
public export
badBattleNoDefense : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2, Macros.pip White]) []
       (MkTypeLine [Siege] [Battle]) [] Nothing {bx = ok})
badBattleNoDefense MkCardBox impossible

||| an adventurer card whose inset frame is a plain instant, naming no Adventure
||| A player plays the card "as an Adventure" [CR#715.3], and [CR#205.3k] makes
||| Adventure the spell type that names it; without it the frame is nothing to play as.
public export
badUnnamedAdventure : Unspellable Card (\ok =>
  Adventurer (MkFace "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Creature]) []
                     (Macros.printedBox (Just (1, 1))))
             (MkFace "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant]) [] Nothing)
             {ai = ok})
badUnnamedAdventure Oh impossible

||| a flip card whose upside-down half is an instant
||| [CR#710.2] reads the alternative characteristics only on the battlefield, and
||| [CR#110.4] keeps an instant off it.
public export
badSpellFlipHalf : Unspellable Card (\ok =>
  FlipCard (MkFace "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature]) []
                   (Macros.printedBox (Just (1, 1))))
           (MkAltFace "" [] (MkTypeLine [] [Instant]) [] Nothing)
           {ah = ok})
badSpellFlipHalf Oh impossible


||| "your opponents' devotion to black"
||| [CR#700.5] defines devotion per player -- a count among the mana costs of
||| permanents ONE player controls -- so a plural possessor reads no total.
public export
badPluralDevotion : Unspellable (Amount []) (\ok =>
  Devotion (PlayerGroup YourOpponents) Black Nothing {one = ok})
badPluralDevotion Refl impossible

||| "the amount of creatures that died this turn"
||| A death is a zone change [CR#700.4], not a quantity, so there is no
||| amount to sum; how MANY died is `EventCount`'s reading.
public export
badDeathSum : Unspellable (Amount []) (\ok =>
  EventSum Death (Macros.a Macros.creature) Lookback.ThisTurn Nothing
           {qm = ok})
badDeathSum Oh impossible

||| "the total power of target creature"
||| The mention fold reads a GROUP; one referent's power is `StatOf`'s, and
||| [CR#208.1] gives each creature its own single number.
public export
badSingularAggregateOf : Unspellable (Amount []) (\ok =>
  AggregateOf SumOf (CharAxis Power) (Macros.target Macros.creature)
              {pl = ok})
badSingularAggregateOf Refl impossible

||| "the greatest life total among all creatures"
||| The fold's axis and complement agree in sort here as everywhere: a life
||| total is a player's [CR#119.1].
public export
badAggregateOfWrongSort : Unspellable (Amount []) (\ok =>
  AggregateOf MaxOf (PlayerStatAxis LifeTotal) (AllOf Macros.creature)
              {sc = ok})
badAggregateOfWrongSort Refl impossible

||| "up to X | Draw a card."
||| [CR#706.3a] gives the results column three forms — a single number,
||| "N1—N2", "N+" — all numbers, so an amount-bounded row is outside the
||| rule's own list.
public export
badAmountRollRow : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.rollADie 20,
                ResultsTable [MkRollRow (UpToOf (LetterVal X))
                                        Macros.drawACard {lt = ok}]])
badAmountRollRow Oh impossible

||| "This deals 3 damage to target player or planeswalker. That player or that creature's controller discards a card."
||| A union mention's targets are the objects and/or players its head describes [CR#115.1], and this head's object half can only be a planeswalker, so the split read's class arm names nothing.
public export
badCreatureHalfRead : Unspellable (Effect []) (\ok =>
  Sequentially
    [ DealDamage This (Lit 3)
        (Macros.target (Macros.kindJoin AnyPlayer (HasType Planeswalker)))
    , Macros.discardsACard
        (Macros.thatSplitController (That (TypeW Creature) {ok = ok})) ])
badCreatureHalfRead Refl impossible
