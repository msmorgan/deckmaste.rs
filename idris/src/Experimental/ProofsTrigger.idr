module Experimental.ProofsTrigger

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Whenever you cast a spell, draw a card."
public export
okCastsSingularComplement : Ability
okCastsSingularComplement =
  Triggered Whenever (Casts You (Macros.a Macros.spell) Nothing) [] Nothing []
            Nothing Nothing Nothing (Draw You (Lit 1))

||| "Whenever you cast all spells, draw a card."
public export
badCastsPluralComplement : Unspellable (Ability) (\ok =>
  Triggered Whenever (Casts You (Macros.allOf Macros.spell) Nothing {one = ok}) [] Nothing [] Nothing Nothing Nothing
            (Draw You (Lit 1)))
badCastsPluralComplement Refl impossible

||| "Whenever this creature attacks while you're casting a spell, draw a card."
public export
okWhileDoingCast : Ability
okWhileDoingCast =
  Triggered Whenever (Macros.attacks Macros.thisCreature) []
            (Just (WhileDoing (Casts You (Macros.a Macros.spell) Nothing)))
            [] Nothing Nothing Nothing
            (Draw You (Lit 1))

||| "Whenever this creature attacks while a creature is dying, draw a card."
public export
badWhileDoingMoment : Unspellable Ability (\ok =>
  Triggered Whenever (Macros.attacks Macros.thisCreature) []
            (Just (WhileDoing (Dies (Macros.a Macros.creature)) {up = ok}))
            [] Nothing Nothing Nothing
            (Draw You (Lit 1)))
badWhileDoingMoment Oh impossible

||| "Whenever a creature dies, draw a card."
public export
okNontargetDeathHeader : Ability
okNontargetDeathHeader =
  Triggered Whenever (Dies (Macros.a Macros.creature)) [] Nothing []
            Nothing Nothing Nothing (Draw You (Lit 1))

||| "Whenever target creature dies, draw a card."
public export
badTargetedDeathHeader : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.target Macros.creature)) [] Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)) {hn = ok})
badTargetedDeathHeader Oh impossible

||| "Whenever a creature attacks, that creature gets +2/+0 until end of turn."
public export
okThatCreatureAfterAttack : Ability
okThatCreatureAfterAttack =
  Triggered Whenever (Attacks (Macros.a Macros.creature) NoDefender)
            [] Nothing [] Nothing Nothing Nothing
            (Macros.gets (Macros.That (TypeW Creature) OneOf) (PtUp (Lit 2)) (PtUp (Lit 0))
                         (Just Macros.untilEndOfTurn))

public export
badThatCreatureIsSelf : Unspellable Ability (\ok =>
  Triggered Whenever (Attacks Macros.thisCreature NoDefender) [] Nothing [] Nothing Nothing Nothing
            (Macros.gets (Macros.That (TypeW Creature) OneOf {ok = Builtin.fst ok}) (PtUp (Lit 2)) (PtUp (Lit 0))
                         {ok = Builtin.snd ok} (Just Macros.untilEndOfTurn)))
badThatCreatureIsSelf (Refl, _) impossible

||| "the last Intervention counter is removed from this enchantment by you"
public export
okAnnouncingRemovalAgent : GameEvent []
okAnnouncingRemovalAgent =
  CounterEvent CounterTaken (Just (NamedCounter "Intervention"))
               Macros.thisEnchantment LastCounter (Just You) False

public export
badAnnouncingRemovalAgent : Unspellable (GameEvent []) (\ok =>
  CounterEvent CounterTaken (Just (NamedCounter "Intervention")) Macros.thisEnchantment LastCounter
               (Just (Macros.target AnyPlayer)) False
               {ag = Present {ok}})
badAnnouncingRemovalAgent Refl impossible

||| "Whenever a creature attacks a player, …"
public export
okSingularAttackDefender : GameEvent []
okSingularAttackDefender =
  Attacks (Macros.a Macros.creature) (OneDefender (Macros.a AnyPlayer))

||| "Whenever a creature attacks your opponents, …"
public export
badPluralAttackDefender : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (PlayerGroup YourOpponents) {sg = ok}))
badPluralAttackDefender Refl impossible

||| "Whenever a creature enters during your turn, draw a card."
public export
okHeaderOwnTurnWindow : Ability
okHeaderOwnTurnWindow =
  Triggered Whenever (Enters (Macros.a Macros.creature) Nothing) [] Nothing []
            (Just (DuringWindow Turn (Just You))) Nothing Nothing
            (Draw You (Lit 1))

||| "Whenever a creature enters during the turn, draw a card."
public export
badHeaderBareTurnWindow : Unspellable Ability (\ok =>
  Triggered Whenever (Enters (Macros.a Macros.creature) Nothing) [] Nothing [] (Just (DuringWindow Turn Nothing {hw = ok})) Nothing Nothing
            (Draw You (Lit 1)))
badHeaderBareTurnWindow Oh impossible

||| "If one or more tokens would be created under your control, …"
public export
okTokenCreationSubject : GameEvent []
okTokenCreationSubject =
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken) False Nothing
                (Just You)

||| "If one or more creatures would be created under your control, …"
public export
badNonTokenCreationSubject : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) Macros.creature) False Nothing (Just You) {tk = ok})
badNonTokenCreationSubject CountedTokens impossible
badNonTokenCreationSubject OneToken impossible

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

||| "Whenever a creature attacks a player"
public export
okPlayerAttackDefender : GameEvent []
okPlayerAttackDefender =
  Attacks (Macros.a Macros.creature) (OneDefender (Macros.a AnyPlayer))

||| "Whenever a creature attacks a planeswalker or a creature"
public export
badMixedAttackDefenderHalves : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a (Joined (HasType Planeswalker)
                                         (HasType Creature))) {at = ok}))
badMixedAttackDefenderHalves Oh impossible

public export
afterLifePayment : Bindings
afterLifePayment = eventAfter (the (GameEvent []) (PaysLife (Macros.a AnyPlayer)))

||| "Whenever a player pays life, that player draws a card."
public export
okLifePaymentPayerReadback : Noun ProofsTrigger.afterLifePayment Player
okLifePaymentPayerReadback = Macros.That PlayerW OneOf

public export
afterPassivePayment : Bindings
afterPassivePayment =
  eventAfter (the (GameEvent [])
    (PaysCost Nothing Paid Macros.thisCreature "CumulativeUpkeep"))

||| "Whenever this creature's cumulative upkeep is paid, that player …"
public export
badPassivePayerReadback :
  Unspellable (Noun ProofsTrigger.afterPassivePayment Player) (\ok => Macros.That PlayerW OneOf {ok})
badPassivePayerReadback Refl impossible

||| "Whenever a player pays life, you gain that much life."
public export
okLifePaymentThatMuch : Amount ProofsTrigger.afterLifePayment
okLifePaymentThatMuch = ThatMuch

public export
afterKeywordCostPayment : Bindings
afterKeywordCostPayment =
  eventAfter (the (GameEvent [])
    (PaysCost (Just You) Paid Macros.thisEnchantment "CumulativeUpkeep"))

public export
badKeywordCostPaymentThatMuch :
  Unspellable (Amount ProofsTrigger.afterKeywordCostPayment) (\ok => ThatMuch {ok})
badKeywordCostPaymentThatMuch Refl impossible

||| "whenever one or more time counters are put on this enchantment"
public export
okManyCountersOnPlacement : GameEvent []
okManyCountersOnPlacement =
  CounterEvent CounterPut (Just (NamedCounter "Time")) Macros.thisEnchantment
               ManyCounters Nothing False

||| "when the last time counter is put on this enchantment"
public export
badLastCounterOnPlacement : Unspellable (GameEvent []) (\ok =>
  CounterEvent CounterPut (Just (NamedCounter "Time")) Macros.thisEnchantment LastCounter Nothing False
               {lb = ok})
badLastCounterOnPlacement Oh impossible

||| "Whenever you create one or more tokens, …"
public export
okTokensCreatedByYou : GameEvent []
okTokensCreatedByYou =
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken) False (Just You)
                Nothing

||| "Whenever an effect and you would create one or more tokens, …"
public export
badTokensCreatedByCauserAndPlayer : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                True (Just You) Nothing {vo = ok})
badTokensCreatedByCauserAndPlayer Oh impossible

||| "Whenever one or more tokens are created under your opponents' control, …"
public export
badTokensCreatedUnderPlural : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                False Nothing (Just (PlayerGroup YourOpponents)) {vo = ok})
badTokensCreatedUnderPlural Oh impossible

||| "Whenever an opponent creates one or more tokens, …"
public export
badTokensCreatedByMintingNoun : Unspellable (GameEvent []) (\ok =>
  TokensCreated (Macros.counted (Macros.atLeast 1) IsToken)
                False (Just Macros.anOpponent) Nothing {vo = ok})
badTokensCreatedByMintingNoun Oh impossible

||| "Whenever this creature becomes the target of a spell, …"
public export
okSpellTargetingEvent : GameEvent []
okSpellTargetingEvent =
  BecomesTarget Macros.thisCreature (Macros.a Macros.spell)

||| "Whenever you become the target of a player, …"
public export
badPlayerTargetingEvent : Unspellable (GameEvent []) (\ok =>
  BecomesTarget You (Macros.a AnyPlayer) {tr = ok})
badPlayerTargetingEvent SpellTargets impossible
badPlayerTargetingEvent (EitherTargets _ _) impossible

||| "If an ability ... triggers, it triggers an additional time."
public export
okMultipliedTrigger : StaticSpec []
okMultipliedTrigger =
  TriggersAdditionally
    (Triggers (Macros.a (And [ AbilityHead AnyTriggered
                             , AbilityOf
                                 (Macros.a Macros.creatureYouControl) ])))
    (Macros.exactly 1)

||| "If a creature you control dies, that ability triggers an additional time."
public export
badMultipliedNonTrigger : Unspellable (StaticSpec []) (\ok =>
  TriggersAdditionally (Dies (Macros.a Macros.creatureYouControl))
                       (Macros.exactly 1) {ok})
badMultipliedNonTrigger Oh impossible

||| "Exchange life totals with target opponent."
public export
okExchangeTwoParties : Instruction []
okExchangeTwoParties = Exchange (LifeTotals (Both You (Macros.target Opponent)))

||| "Exchange life totals with target opponent" with one party: the row states
||| both halves at once, so an exchange that can't be completed in its entirety
||| is unwritable rather than half-done [CR#701.12a,701.12c].
public export
badExchangeOneParty : Unspellable (Instruction []) (\ok =>
  Exchange (LifeTotals (Macros.target Opponent) {tp = ok}))
badExchangeOneParty Oh impossible

||| "You and your opponents exchange life totals."
public export
badExchangePluralParty : Unspellable (Instruction []) (\ok =>
  Exchange (LifeTotals (Both You (PlayerGroup YourOpponents)) {tp = ok}))
badExchangePluralParty Oh impossible

||| "Whenever an opponent commits a crime, draw a card."
public export
okCrimeBySinglePlayer : Ability
okCrimeBySinglePlayer =
  Triggered Whenever (CommitsCrime Macros.anOpponent) [] Nothing [] Nothing
            Nothing Nothing (Draw You (Lit 1))

||| "Whenever all players commit a crime, draw a card."
||| A crime targets an opponent, something an opponent controls, or a card in
||| an opponent's graveyard [CR#700.13]; the whole player set has no opponent,
||| so such a crime has no opponent-owned object. One player at a time commits
||| one (`okCrimeBySinglePlayer`).
public export
badCrimeByAllPlayers : Unspellable Ability (\ok =>
  Triggered Whenever (CommitsCrime (Macros.allOf AnyPlayer) {sc = ok}) []
            Nothing [] Nothing Nothing Nothing (Draw You (Lit 1)))
badCrimeByAllPlayers OneCriminal impossible
