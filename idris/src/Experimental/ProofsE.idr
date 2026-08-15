||| Unspellable pins, continued from Experimental.ProofsD.
module Experimental.ProofsE

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Instant and sorcery spells you cast have lifelink."
||| Lifelink is read when the spell deals damage [CR#702.15d], so control fixes the class then.
public export
badCastGrantAtResolution : Unspellable (StaticEffect []) (\ok =>
  Gains (AllOf (And [Macros.instantOrSorcery, Macros.spell, CastBy You]))
        (KeywordAbility Lifelink) {ok})
badCastGrantAtResolution MkGrantSubject impossible


||| "Artifact spells you control have convoke."
||| Convoke functions while the spell is cast [CR#702.51a], the moment [CR#601.2a] applies the grant.
public export
badControlGrantAtCasting : Unspellable (StaticEffect []) (\ok =>
  Gains (AllOf (And [Macros.artifact, Macros.spell, ControlledBy You]))
        (KeywordAbility Convoke) {ok})
badControlGrantAtCasting MkGrantSubject impossible


||| "Creatures you control have convoke."
||| Convoke functions only while the spell is on the stack [CR#702.51a], so a permanent grant does nothing.
public export
badBattlefieldConvoke : Unspellable (StaticEffect []) (\ok =>
  Gains (AllOf Macros.creatureYouControl) (KeywordAbility Convoke) {ok})
badBattlefieldConvoke MkGrantSubject impossible

||| "Enchanted creature's power and toughness are each equal to your life total."
||| A characteristic-defining ability affects no other object's characteristics [CR#604.3a].
public export
badGrantedPtDefinition : Unspellable (StaticEffect []) (\ok =>
  DefinesPt (AttachHost Enchanted (TypeW Creature)) BothEach
            (PlayerStatOf LifeTotal You) {sd = ok})
badGrantedPtDefinition MkSelfDefined impossible


||| "This creature's power and toughness are each equal to 3."
||| The star stands where a fixed number would be printed [CR#208.2], so a written value prints twice.
public export
badWrittenPtDefinition : Unspellable (StaticEffect []) (\ok =>
  DefinesPt Macros.thisCreature BothEach (Lit 3) {dv = ok})
badWrittenPtDefinition MkDefiningValue impossible


||| a creature card printing "2/2" whose own text defines its power and toughness
||| [CR#208.2] gives such a card a star in each slot its text defines.
public export
badStarlessDefinedPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature])
       [Static (DefinesPt Macros.thisCreature BothEach
                          (CountOf Macros.creatureYouControl))]
       (Just (2, 2)) {pts = ok})
badStarlessDefinedPt MkCardPt impossible


||| "This creature's power and toughness are each equal to the number of creatures you control" established as a clause by a resolving spell.
||| A characteristic-defining ability is printed on its own card [CR#604.3,604.3a], so no resolution establishes one.
public export
badPtDefinitionClause : Unspellable (Effect []) (\ok =>
  Continuously (DefinesPt Macros.thisCreature BothEach
                          (CountOf Macros.creatureYouControl))
               Nothing {sp = ok})
badPtDefinitionClause SpanUnstated impossible


||| "This creature's power and toughness are switched." as a printed line.
||| The switch is written as an instruction with a duration, not as a standing statement [CR#613.4d].
public export
badSwitchLine : Unspellable Ability (\ok =>
  Static (SwitchesPt Macros.thisCreature) {ln = ok})
badSwitchLine MkStaticLine impossible


||| "Target creature card in a graveyard has base power and toughness 1/1."
||| Setting base power and toughness is a continuous effect on a permanent [CR#613.4b].
public export
badBasePtInGraveyard : Unspellable (StaticEffect []) (\ok =>
  HasBasePt (Macros.target (And [Macros.creature, InZone Macros.graveyardZ]))
            (Lit 1) (Lit 1) {zn = ok})
badBasePtInGraveyard MkZoneFits impossible


||| "the creature with the total power among creatures you control"
||| A fold that picks a member may not sum, because a sum names no member of anything.
public export
badSumSelection : Unspellable (Predicate [] Object) (\ok =>
  Superlative SumOf (CharAxis Power) Macros.creature {ex = ok})
badSumSelection MkIsExtremal impossible


||| "the creature"
||| The definite article demands a description that identifies its referent; the superlative is the one licensor.
public export
badBareDefinite : Unspellable (Noun [] Object) (\ok =>
  Definite Macros.creature {uq = ok})
badBareDefinite MkUniquifying impossible


||| "Choose the creature with the least toughness among creatures you control."
||| A definite description names its referent, so a choice clause has nothing to offer.
public export
badChooseDefinite : Unspellable (Effect []) (\ok =>
  Choose (Definite (And [Macros.creature,
                         Superlative MinOf (CharAxis Toughness)
                                     Macros.creatureYouControl])) {ch = ok})
badChooseDefinite BareChoice impossible


||| "creature with power 4 or greater with the greatest power among creatures"
||| The superlative is a comparison, so it takes the one-bound-per-phrase discipline.
public export
badSuperlativeAndBound : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Compare Power AtLeast (Lit 4),
       Superlative MaxOf (CharAxis Power) Macros.creature] {lc = ok})
badSuperlativeAndBound MkLoneComparison impossible


||| "the creature with the highest life total among creatures you control"
||| A life total is a player's [CR#119.1], so no description of an object is qualified by one.
public export
badLifeTotalSuperlative : Unspellable (Predicate [] Object) (\ok =>
  Superlative MaxOf (PlayerStatAxis LifeTotal) Macros.creature {sc = ok})
badLifeTotalSuperlative Refl impossible


||| "When target player removes the last intervention counter from this enchantment, the game is a draw."
||| The event's context accounting is its patient's, so an announcing agent would be dropped silently.
public export
badAnnouncingRemovalAgent : Unspellable (GameEvent []) (\ok =>
  LastCounterRemoved Intervention Macros.thisEnchantment
                     {by = Just (Macros.target AnyPlayer)}
                     {ag = AgentVoiced {bl = ok}})
badAnnouncingRemovalAgent MkBindingless impossible


||| "Each player gets a charge counter."
||| A charge counter is placed on an object [CR#122.1], so the player verb refuses it.
public export
badGetsChargeCounter : Unspellable (Effect []) (\ok =>
  GetsCounters You (Lit 1) Charge {sc = ok})
badGetsChargeCounter Refl impossible


||| "this instant"
||| [CR#109.2] projects a type word with no zone word onto the battlefield, where an instant never is [CR#110.4a].
public export
badAscribeInstant : Unspellable (Noun [] Object) (\ok =>
  AsType Instant This {way = ok})
badAscribeInstant Refl impossible


||| "this Zombie"
||| A creature type is never a self-name; the ascription reads its own table, not the subtype catalog.
public export
badAscribeCreatureType : Unspellable (Noun [] Object) (\ok =>
  AsType Creature This {sub = Just Zombie} {way = ok})
badAscribeCreatureType Refl impossible


||| "this Curse"
||| A card with two enchantment types self-names by the one whose rules the sentence reaches [CR#303.4].
public export
badAscribeCurse : Unspellable (Noun [] Object) (\ok =>
  AsType Enchantment This {sub = Just Curse} {way = ok})
badAscribeCurse Refl impossible


||| "target creature that is fortified"
||| The participle catalog admits "fortified" at the head reader [CR#301.6] and not at the presence check.
public export
badIsFortified : Unspellable (Predicate [] Object) (\ok =>
  IsAttached Fortified {ok})
badIsFortified Refl impossible


||| "target creature that is the monarch"
||| The monarch is a designation a player can have [CR#725.1], so no object description is qualified by it.
public export
badObjectMonarch : Unspellable (Predicate [] Object) (\ok =>
  HasDesignation Monarch {sc = ok})
badObjectMonarch Refl impossible


||| "You become goaded."
||| Goaded is a designation a permanent can have [CR#701.15b], so no player gains one.
public export
badGoadedPlayer : Unspellable (Effect []) (\ok =>
  GainsDesignation You Goaded {sc = ok})
badGoadedPlayer Refl impossible


||| "goad target creature card in your graveyard"
||| An object is given a designation on the battlefield and nowhere else [CR#701.15b].
public export
badGoadInGraveyard : Unspellable (Effect []) (\ok =>
  GainsDesignation (Macros.target (And [Macros.creature,
                                        InZone (Macros.graveyardOf You)]))
                   Goaded {zn = ok})
badGoadInGraveyard (HolderOnField {ok = OnField}) impossible


||| "Unflip target creature."
||| Flipping is one-way [CR#710.4]: once flipped, a permanent cannot become unflipped.
public export
badUnflipInstruction : Unspellable (Effect []) (\ok =>
  SetStatus Unflipped (Macros.target Macros.creature) {at = ok})
badUnflipInstruction MkStatusEffectVal impossible


||| "Mill a card." spelled off the BOTTOM of the library
||| [CR#701.17a] mills from the top of the library, so a bottom slice is no mill.
public export
badBottomMill : Unspellable (Effect []) (\ok =>
  Does You Mill (Move (LibrarySlice OnBottom (Lit 1) You) Macros.graveyardZ) {tb = ok})
badBottomMill MillB impossible


||| "Ward" printed as a bare keyword line
||| A parameterized keyword may not shed its parameter: [CR#702.21a] writes the ability as "Ward [cost]".
public export
badBareWardLine : Unspellable Ability (\ok => KeywordAbility Ward {pf = ok})
badBareWardLine MkKeywordParamFits impossible


||| "Ward red"
||| The parameter's type is the keyword's: [CR#702.21a] demands a cost, not a quality.
public export
badWardQuality : Unspellable Ability (\ok =>
  KeywordAbility Ward {param = Just (ParamQuality (ColorIs Red))} {pf = ok})
badWardQuality MkKeywordParamFits impossible


||| "Flying {2}"
||| A keyword whose row takes no parameter refuses one rather than ignoring it.
public export
badParamOnNullaryKeyword : Unspellable Ability (\ok =>
  KeywordAbility Flying {param = Just (ParamCost (Mana [Macros.generic 2]))} {pf = ok})
badParamOnNullaryKeyword MkKeywordParamFits impossible


||| "creatures with ward"
||| The description position refuses the parameterized rows the ability position demands a parameter for.
public export
badWardDescription : Unspellable (Predicate [] Object) (\ok => HasKeyword Ward {np = ok})
badWardDescription MkKeywordParamless impossible


||| "Protection from red" printed as a line on an instant card
||| [CR#702.16b] gives protection to a permanent or player, which an instant card's line never is.
public export
badProtectionOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Instant])
       [KeywordAbility Protection {param = Just (ParamQuality (ColorIs Red))}] Nothing {tx = ok})
badProtectionOnInstant MkCardText impossible


||| "Renown 0"
||| [CR#702.112a] puts N counters on the creature, and a zero-count keyword instructs nothing.
public export
badRenownZero : Unspellable Ability (\ok =>
  KeywordAbility Renown {param = Just (ParamNumber (Lit 0) {wc = ok})})
badRenownZero MkWrittenCount impossible


||| "Enchant red"
||| [CR#702.5a]'s "[object or player]" is a description, and descriptions here have head nouns.
public export
badHeadlessEnchant : Unspellable Ability (\ok =>
  KeywordAbility Enchant {param = Just (ParamSubject (ColorIs Red) {hd = ok})})
badHeadlessEnchant MkHeaded impossible


||| "Protection from player"
||| [CR#702.5a] writes enchant's slot as "[object or player]" where [CR#702.16a] writes protection's as "[quality]".
public export
badProtectionFromPlayerRestriction : Unspellable Ability (\ok =>
  KeywordAbility Protection {param = Just (ParamSubject AnyPlayer)} {pf = ok})
badProtectionFromPlayerRestriction MkKeywordParamFits impossible


||| "Equip {2}" printed as a line on a sorcery card
||| [CR#702.6a] makes equip an activated ability of Equipment cards, which a sorcery is not.
public export
badEquipOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility Equip {param = Just (ParamCost (Mana [Macros.generic 2]))}]
       Nothing {tx = ok})
badEquipOnSorcery MkCardText impossible


||| "Choose one or more creatures."
||| The counted untargeted group is not a choice complement; "choose one or more" is the modal headcount.
public export
badChooseCountedGroup : Unspellable (Effect []) (\ok =>
  Choose (CountedGroup (Macros.atLeast 1) Macros.creature) {ch = ok})
badChooseCountedGroup BareChoice impossible


||| "each of one or more creatures"
||| The counted untargeted group has not fixed its members, so there is nothing to distribute into.
public export
badEachOfCountedGroup : Unspellable (Noun [] Object) (\ok =>
  EachOf (CountedGroup (Macros.atLeast 1) Macros.creature) {gm = ok})
badEachOfCountedGroup MkGroupMention impossible


||| "one of one or more creatures"
||| The same cell at the partitive: the group's members are not yet fixed to pick among.
public export
badPartitiveOfCountedGroup : Unspellable (Noun [] Object) (\ok =>
  SomeOf (Macros.exactly 1) (CountedGroup (Macros.atLeast 1) Macros.creature)
         {gm = ok})
badPartitiveOfCountedGroup MkGroupMention impossible


||| "One or more opponents lose 1 life. Draw that many cards."
||| The group anaphor counts objects, so a player group leaves no size to read back.
public export
badPlayerGroupSize : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.losesLife (CountedGroup (Macros.atLeast 1) Opponent) (Lit 1)
               , Draw You (GroupSize {ok}) ])
badPlayerGroupSize Refl impossible


||| "Whenever a creature attacks your opponents, …"
||| A creature attacks one defender: [CR#508.1b] announces which one each attacking creature attacks.
public export
badPluralAttackDefender : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature) {whom = Just (PlayerGroup YourOpponents)} {df = ok})
badPluralAttackDefender OneDefender impossible


||| "Look at the top four cards of your library. Put one of them into your hand. An opponent chooses the rest."
||| A choice needs a set to select from, and the group complement names what was left over.
public export
badAgentChooseTheRest : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt (Macros.topCards 4)
               , Move (Macros.oneOf Them) Macros.handZ
               , Choose TheRest {by = Just (Macros.a Opponent)} {ch = ok} ])
badAgentChooseTheRest AgentChoice impossible


||| "Whenever a creature is put into the battlefield, draw a card."
||| English writes "put onto the battlefield"; the battlefield's arrival is its own event [CR#603.6a].
public export
badPutIntoBattlefield : Unspellable (GameEvent []) (\ok =>
  PutInto (Macros.a Macros.creature) Macros.battlefieldZ {dk = ok})
badPutIntoBattlefield MkPutDest impossible


||| "Whenever a card in exile is put into a graveyard from exile, …"
||| The source axis names what a card leaves [CR#603.6c], and nothing is put somewhere out of exile.
public export
badPutFromExile : Unspellable (GameEvent []) (\ok =>
  PutInto (Macros.a (InZone Macros.exileZ)) Macros.graveyardZ
          {from = Just (FromZone Macros.exileZ)} {sk = ok})
badPutFromExile MkPutSource impossible


||| "The next time this creature would be put into a graveyard from the battlefield this turn, exile it instead."
||| The zone arrival takes the interception in the standing form only; [CR#614.3]'s one-shot word is another row.
public export
badNextTimePutInto : Unspellable (Effect []) (\ok =>
  Macros.nextTimeWouldInstead
    (PutInto Macros.thisCreature Macros.graveyardZ
             {from = Just (FromZone Macros.battlefieldZ)})
    (Macros.exile Macros.thisCreature) (Just Macros.thisTurn) {uo = ok})
badNextTimePutInto MkReplUseOk impossible


||| "If a +1/+1 counter would be removed from this creature, draw a card instead this turn."
||| The counter placement is interceptable and the removal is not, which is why the event has two names.
public export
badInterceptCounterRemoval : Unspellable (Effect []) (\ok =>
  Macros.ifWouldInstead
    (CounterEvent CounterTaken Macros.plusOnePlusOne Macros.thisCreature)
    Macros.drawACard (Just Macros.thisTurn)
    {ok = Builtin.fst ok, uo = Builtin.snd ok})
badInterceptCounterRemoval (MkInterceptable, _) impossible


||| "… if a creature was put into a zone this turn, …"
||| The history read names its destination, for which a subject-event-window query has no slot.
public export
badPlacementLookback : Unspellable (Condition []) (\ok =>
  Happened Placement (Macros.a Macros.creature) ThisTurn {sb = ok})
badPlacementLookback MkLookbackSubject impossible


||| "Whenever a creature enters during your upkeep, draw a card."
||| The upkeep is what a header names as an event [CR#603.2b], never as a qualifier on another one.
public export
badHeaderUpkeepWindow : Unspellable Ability (\ok =>
  Triggered Whenever (Enters (Macros.a Macros.creature))
            {window = Just (DuringWindow Upkeep (Just Yours) {hw = ok})}
            Macros.drawACard)
badHeaderUpkeepWindow MkHeaderWindowOk impossible


||| "Whenever a creature enters during the turn, draw a card."
||| A window with no possessor restricts nothing.
public export
badHeaderBareTurnWindow : Unspellable Ability (\ok =>
  Triggered Whenever (Enters (Macros.a Macros.creature))
            {window = Just (DuringWindow Turn Nothing {hw = ok})}
            Macros.drawACard)
badHeaderBareTurnWindow MkHeaderWindowOk impossible


||| "Whenever a creature enters during your precombat main phase, draw a card."
||| [CR#505.1] gives a turn two main phases, and this vocabulary has a row for each and none for the pair.
public export
badHeaderMainPhaseWindow : Unspellable Ability (\ok =>
  Triggered Whenever (Enters (Macros.a Macros.creature))
            {window = Just (DuringWindow FirstMain (Just Yours) {hw = ok})}
            Macros.drawACard)
badHeaderMainPhaseWindow MkHeaderWindowOk impossible


||| "Whenever a creature enters or at the beginning of your upkeep, draw a card."
||| One word governs both disjuncts, and [CR#603.2b] fixes the turn-part beginning's word at "At".
public export
badCoordinatedPartBeginning : Unspellable Ability (\ok =>
  Triggered Whenever (Enters (Macros.a Macros.creature))
            {alt = Just (BeginningOf Upkeep (Just Yours))}
            Macros.drawACard {ae = OneAlt {wo = ok}})
badCoordinatedPartBeginning MkTriggerWordOk impossible


||| "The next time you would create one or more tokens, create a 1/1 white Soldier creature token instead."
||| [CR#614.3]'s one-shot word belongs to a carrier, and token creation spins none up.
public export
badNextTimeWouldCreate : Unspellable (StaticEffect []) (\ok =>
  Intercepts (TokensCreated (CountedGroup (Macros.atLeast 1) IsToken)
                            {under = Just You})
             (Macros.create (Lit 1) (Macros.creatureTok 1 1 [White] [Soldier]))
             NextTimeOnly {uo = ok})
badNextTimeWouldCreate MkReplUseOk impossible


||| "If you would create one or more tokens under your control, …"
||| [CR#111.2] makes the creator the controller, so naming both says one thing twice.
public export
badCreatedByYouUnderYourControl : Unspellable (GameEvent []) (\ok =>
  TokensCreated (CountedGroup (Macros.atLeast 1) IsToken)
                {by = Just You} {under = Just You} {vo = ok})
badCreatedByYouUnderYourControl CreatedPlain impossible
badCreatedByYouUnderYourControl CreatedBy impossible
badCreatedByYouUnderYourControl CreatedUnder impossible


||| "If one or more creatures would be created under your control, …"
||| Only a token is ever created [CR#111.1]; the head word may be added to and never replaced.
public export
badNonTokenCreationSubject : Unspellable (GameEvent []) (\ok =>
  TokensCreated (CountedGroup (Macros.atLeast 1) Macros.creature)
                {under = Just You} {tk = ok})
badNonTokenCreationSubject CountedTokens impossible
badNonTokenCreationSubject OneToken impossible




||| "If a +1/+1 counter would be put on a creature you control, that many plus one +1/+1 counters are put on it instead."
||| A batch of one has no size to read back; the article has already spelled the quantity.
public export
badSingularCounterBatchSize : Unspellable (StaticEffect []) (\ok =>
  Intercepts (CounterEvent CounterPut Macros.plusOnePlusOne
                           (Macros.a Macros.creatureYouControl) {many = OneCounter})
             (PutCounters (Plus (ThatMuch {ok}) (Lit 1))
                          Macros.plusOnePlusOne It)
             Repeatedly)
badSingularCounterBatchSize Refl impossible


||| "The next time a creature would enter, exile it instead."
||| [CR#614.3]'s one-shot word belongs to a carrier that spins the replacement up, and entry has none.
public export
badNextTimeWouldEnter : Unspellable (StaticEffect []) (\ok =>
  Intercepts (Enters (Macros.a Macros.creature)) (Macros.exile It)
             NextTimeOnly {uo = ok})
badNextTimeWouldEnter MkReplUseOk impossible


||| "If an effect would create one or more tokens, it creates twice that many of those tokens instead."
||| [CR#111.2] fixes the controller by naming the creator, so an abstract-effect creator must state one.
public export
badCausedCreationBareControl : Unspellable (GameEvent []) (\ok =>
  TokensCreated (CountedGroup (Macros.atLeast 1) IsToken)
                {cause = Just AnEffect} {vo = ok})
badCausedCreationBareControl CreatedPlain impossible
badCausedCreationBareControl CreatedBy impossible
badCausedCreationBareControl CreatedUnder impossible
badCausedCreationBareControl CreatedByCauser impossible


||| "If you and an effect would put one or more +1/+1 counters on a creature you control, …"
||| One clause names one doer: the causer stands where the player agent would and excludes it.
public export
badCausedCounterWithAgent : Unspellable (GameEvent []) (\ok =>
  CounterEvent CounterPut Macros.plusOnePlusOne
               (Macros.a Macros.creatureYouControl)
               {by = Just You} {cause = Just AnEffect} {cz = ok})
badCausedCounterWithAgent NotCaused impossible
badCausedCounterWithAgent CausedByEffect impossible


||| "Destroy target creature. Create two of those tokens."
||| "Those tokens" points at a definition of characteristics [CR#111.3], which ordinary creatures leave none of.
public export
badAnaphoricTokenAfterNonToken : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , Create You (Lit 2) (TokenAsThose {ok}) [] ])
badAnaphoricTokenAfterNonToken Refl impossible


||| "Create two 1/1 white Soldier creature tokens. Exile them. Create one of those tokens."
||| A token off the battlefield ceases to exist [CR#111.7], so the word reaches nothing.
public export
badAnaphoricTokenOffBattlefield : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.create (Lit 2) (Macros.creatureTok 1 1 [White] [Soldier])
               , Macros.exile Them
               , Create You (Lit 1) (TokenAsThose {ok}) [] ])
badAnaphoricTokenOffBattlefield Refl impossible

||| "Target creature becomes a Coward until end of turn. It's still a land."
||| Removing a subtype does not affect card types [CR#205.1a], so a retention rider puts nothing back.
public export
badStillOnSubtypeSet : Unspellable (StaticEffect []) (\ok =>
  SetsType (Macros.target Macros.creature)
           (MkToken Nothing [] (MkTypeLine [Coward] []) [] Nothing)
           (Just Land) {ro = ok})
badStillOnSubtypeSet MkRetentionOk impossible


||| "This land becomes an artifact until end of turn. It's still a creature."
||| The retention rider names the type the setting took away, which here it never does.
public export
badStillACreature : Unspellable (StaticEffect []) (\ok =>
  SetsType Macros.thisLand
           (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing)
           (Just Creature) {ro = ok})
badStillACreature MkRetentionOk impossible


||| "Target creature becomes an artifact until end of turn. It's still an instant."
||| [CR#205.1a] retains the instant and sorcery types with no rider at all.
public export
badStillAnInstant : Unspellable (StaticEffect []) (\ok =>
  SetsType (Macros.target Macros.creature)
           (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing)
           (Just Instant) {ro = ok})
badStillAnInstant MkRetentionOk impossible


||| "This land becomes a 2/2 creature this turn."
||| The current-turn adverbial stays the restriction family's; the type setting writes "until end of turn".
public export
badTypeSetThisTurn : Unspellable (Effect []) (\ok =>
  Continuously (SetsType Macros.thisLand
                         (MkToken (Just (Lit 2, Lit 2)) []
                                  (MkTypeLine [] [Creature]) [] Nothing)
                         Nothing)
               (Just ThisTurn) {sp = ok})
badTypeSetThisTurn (SpanStated) impossible


||| "Target creature loses all abilities until your next upkeep."
||| The ability loss writes three adverbials and no fourth; the upkeep endpoint is another row's.
public export
badLoseAbilitiesUntilUpkeep : Unspellable (Effect []) (\ok =>
  Continuously (LosesAllAbilities (Macros.target Macros.creature))
               (Just (Until (StartOf Upkeep (Just Yours)))) {sp = ok})
badLoseAbilitiesUntilUpkeep (SpanStated) impossible

||| "Target creature gets +1/+1 and gains flying and gains trample."
||| A coordination inside a coordination spells one flat list twice.
public export
badNestedCoordination : Unspellable (StaticEffect []) (\ok =>
  AndAlso (Coord.(::) (AndAlso [ Gets (Macros.target Macros.creature)
                                      (PtUp (Lit 1)) (PtUp (Lit 1))
                               , Gains It (KeywordAbility Flying) ])
                      {nc = ok}
                      (Coord.(::) (Gains It (KeywordAbility Trample)) Coord.Nil)))
badNestedCoordination MkNotCoord impossible


||| "Creatures you control get +1/+1 and you gain control of them."
||| A coordination states whatever its parts state, and the control grant is no statement [CR#604.1].
public export
badCoordinatedGainsControl : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets (AllOf Macros.creatureYouControl) (PtUp (Lit 1)) (PtUp (Lit 1))
                  , GainsControl You Them ]) {ln = ok})
badCoordinatedGainsControl MkStaticLine impossible


||| "Target creature gets +1/+0 and can't be blocked this turn."
||| One envelope covers every part, and the grant and the restriction share no current-turn word.
public export
badCoordinatedSpanDisagree : Unspellable (Effect []) (\ok =>
  Continuously (AndAlso [ Gets (Macros.target Macros.creature)
                               (PtUp (Lit 1)) (PtUp (Lit 0))
                        , Deontic It Forbid Block Patient Nothing ])
               (Just Macros.thisTurn) {cs = ok})
badCoordinatedSpanDisagree MkCoordSpanOk impossible


||| "Target creature gets +1/+1 and gains flying until end of upkeep."
||| The carrier writes no adverbial of its own, so every part's own row must admit the span.
public export
badCoordinatedUnattestedSpan : Unspellable (Effect []) (\ok =>
  Continuously (AndAlso [ Gets (Macros.target Macros.creature)
                               (PtUp (Lit 1)) (PtUp (Lit 1))
                        , Gains It (KeywordAbility Flying) ])
               (Just (Until (EndOf Upkeep Nothing))) {cs = ok})
badCoordinatedUnattestedSpan MkCoordSpanOk impossible


||| "This creature gets +1/+1 and that creature has flying."
||| The demonstrative screen at the third minting site: it never picks out the sentence's own subject.
public export
badThatCreatureIsStaticSubject : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains (That (TypeW Creature) {ok = ok}) (KeywordAbility Flying) ]))
badThatCreatureIsStaticSubject Refl impossible


||| "Enchanted creature gets +1/+1 and they have flying."
||| An attachment host is one thing [CR#303.4d,301.5c], so the plural anaphor finds no group.
public export
badCoordinatedHostPlural : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains (Them {ok = ok}) (KeywordAbility Flying) ]))
badCoordinatedHostPlural Refl impossible


||| "Enchanted player can't lose the game and they can't win the game."
||| The statement announces its subject at the object rows only, so a player host announces nothing.
public export
badCoordinatedPlayerHost : Unspellable Ability (\ok =>
  Static (AndAlso [ OutcomeGate CantLose (AttachHost Enchanted PlayerW)
                  , OutcomeGate CantWin (They {ok = ok}) ]))
badCoordinatedPlayerHost Refl impossible


||| "Enchanted land gets +1/+1 and can't block."
||| The host's type is threaded forward, and only a creature can attack or block [CR#506.3].
public export
badCoordinatedLandHostBlocks : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Land))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Deontic It Forbid Block Agent Nothing {dp = ok} ]))
badCoordinatedLandHostBlocks Participant impossible


||| "Flash" printed as a bare line on an instant card.
||| [CR#702.8a] grants exactly the timing an instant already has; the line is a permanent's.
public export
badFlashOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [KeywordAbility Flash] Nothing {tx = ok})
badFlashOnInstant MkCardText impossible


||| "Target creature gains flash."
||| Flash functions where the card is played from and on the stack [CR#702.8a,113.6e], never on the battlefield.
public export
badBattlefieldFlash : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.target Macros.creature) (KeywordAbility Flash) {ok})
badBattlefieldFlash MkGrantSubject impossible


||| "Target spell gains indestructible."
||| [CR#702.12b] is about a permanent that can't be destroyed, and a spell is not one.
public export
badSpellIndestructible : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.target Macros.spell) (KeywordAbility Indestructible) {ok})
badSpellIndestructible MkGrantSubject impossible


||| "this battle"
||| The ascription table answers by subtype here, [CR#109.2] admitting a card type or a subtype.
public export
badAscribeBattle : Unspellable (Noun [] Object) (\ok =>
  AsType Battle This {way = ok})
badAscribeBattle Refl impossible


||| a "Kindred Enchantment — Siege" card
||| [CR#308.2] extends kindred to creature types only, so a Siege still needs the battle type [CR#205.3c].
public export
badSiegeWithoutBattle : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [Siege] [Kindred, Enchantment])
       [KeywordAbility Flying] Nothing {ln = ok})
badSiegeWithoutBattle MkCardLine impossible


||| a "Kindred — Merfolk" card naming no other card type
||| "Each kindred card has another card type" [CR#308.1].
public export
badKindredAlone : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [Merfolk] [Kindred])
       [KeywordAbility Flying] Nothing {ln = ok})
badKindredAlone MkCardLine impossible


||| "Enchanted planeswalker can't attack."
||| Only a creature can attack or block [CR#506.3]; the passive is the row this type opened.
public export
badPlaneswalkerAttacks : Unspellable Ability (\ok =>
  Static (Deontic (AttachHost Enchanted (TypeW Planeswalker))
                  Forbid Attack Agent Nothing {dp = ok}))
badPlaneswalkerAttacks Participant impossible


||| "Create a 1/1 white Soldier creature token with 'Draw two cards.'"
||| A token is a permanent [CR#111.1] and a spell ability is a resolving spell's [CR#113.3a].
public export
badTokenSpellAbility : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1, Lit 1)) [White]
                                 (MkTypeLine [Soldier] [Creature])
                                 [Spell Macros.drawACard] Nothing) {ta = ok})
badTokenSpellAbility MkTokenAbilities impossible


||| "Instant and sorcery spells you cast have '{T}: Draw a card.'"
||| [CR#113.6e] gives a granted play-modifying ability the stack alone to function in.
public export
badQuotedGrantOnSpell : Unspellable (StaticEffect []) (\ok =>
  Gains (AllOf (And [Macros.instantOrSorcery, Macros.spell, CastBy You]))
        (Activated TapSymbol Macros.drawACard) {ok})
badQuotedGrantOnSpell MkGrantSubject impossible


||| "You get an emblem with 'flying'."
||| [CR#114.3] leaves an emblem no types, mana cost or color, so a keyword names a subject it lacks.
public export
badKeywordEmblem : Unspellable (Effect []) (\ok =>
  GetsEmblem You [KeywordAbility Flying] {ea = ok})
badKeywordEmblem MkEmblemAbilities impossible


||| "You get an emblem."
||| An emblem is a marker representing an object with one or more abilities [CR#114.1].
public export
badEmptyEmblem : Unspellable (Effect []) (\ok =>
  GetsEmblem You [] {ea = ok})
badEmptyEmblem MkEmblemAbilities impossible


||| "You get an emblem with 'Draw a card': Draw a card."
||| A cost is what the activator pays [CR#602.1a]; the emblem is the ability's payoff.
public export
badEmblemAsCost : Unspellable Ability (\ok =>
  Activated (Do (GetsEmblem You [Static (Gets (AllOf Macros.creatureYouControl)
                                              (PtUp (Lit 1)) (PtUp (Lit 1)))]) {ok})
            Macros.drawACard)
badEmblemAsCost MkCostAction impossible


||| "[+0]: Draw a card."
||| [CR#606.4] shows the direction by the symbol, and at zero both directions show the bare "[0]".
public export
badLoyaltyUpZero : Unspellable Ability (\ok =>
  Activated (LoyaltySymbol (LoyaltyUp 0 {nz = ok})) Macros.drawACard)
badLoyaltyUpZero MkLoyaltyStep impossible


||| "[−0]: Draw a card."
||| The other direction of the same refusal, sharing one gate [CR#606.4].
public export
badLoyaltyDownZero : Unspellable Ability (\ok =>
  Activated (LoyaltySymbol (LoyaltyDown 0 {nz = ok})) Macros.drawACard)
badLoyaltyDownZero MkLoyaltyStep impossible


||| "[+1], {T}: Draw a card."
||| A loyalty symbol is a whole cost, never one component among several [CR#606.5].
public export
badCompoundLoyalty : Unspellable Ability (\ok =>
  Activated (Compound ((LoyaltySymbol (LoyaltyUp 1) :: (TapSymbol :: Nil)) {nl = ok}))
            Macros.drawACard)
badCompoundLoyalty MkNotLoyalty impossible


||| "[+1]: Draw a card. Activate only once each turn."
||| [CR#606.3] already caps every loyalty ability, keyed to the permanent rather than the ability.
public export
badLoyaltyOncePerTurn : Unspellable Ability (\ok =>
  Activated (LoyaltySymbol (LoyaltyUp 1)) Macros.drawACard
            {limit = Just OncePerTurn} {ld = ok})
badLoyaltyOncePerTurn MkLoyaltyDefaults impossible


||| "[+1]: Draw a card. Activate only as a sorcery."
||| [CR#606.3] gives a loyalty ability its whole window, which is sorcery timing said longhand.
public export
badLoyaltySorceryWindow : Unspellable Ability (\ok =>
  Activated (LoyaltySymbol (LoyaltyUp 1)) Macros.drawACard
            {window = Just AsSorcery} {ld = ok})
badLoyaltySorceryWindow MkLoyaltyDefaults impossible


||| "[+1]: Draw a card. Activate only if you control a creature."
||| [CR#606.3] states the permission in full and conditions it on nothing but the once-per-turn cap.
public export
badLoyaltyGuard : Unspellable Ability (\ok =>
  Activated (LoyaltySymbol (LoyaltyUp 1)) Macros.drawACard
            {guard = Just (Exists Macros.creatureYouControl)} {ld = ok})
badLoyaltyGuard MkLoyaltyDefaults impossible


||| "you pay [+1]"
||| The pay clause names its payer, and [CR#606.4] moves counters on the permanent instead.
public export
badPayLoyalty : Unspellable (Effect []) (\ok =>
  Pay You (LoyaltySymbol (LoyaltyUp 1)) {pb = ok})
badPayLoyalty MkPayable impossible


||| "[+1]: Draw a card." printed on a sorcery card
||| A loyalty cost cannot be paid off the battlefield [CR#606.3,606.4], which [CR#113.6j] requires.
public export
badLoyaltySorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Loyalty Sorcery" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [Activated (LoyaltySymbol (LoyaltyUp 1)) Macros.drawACard] Nothing {tx = ok})
badLoyaltySorcery MkCardText impossible


||| "put them on top of your library in a random order"
||| [CR#401.4] grants the arrangement to the owner, so an override is written only where the order is hidden.
public export
badRandomOnTop : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt OnTop (Just RandomOrder) {af = ok} Bare)
badRandomOnTop MkArrangementFits impossible


||| "Scry 1." with no one scrying
||| [CR#701.22a] states the action of a player and whose library at once, so the tag needs a doer.
public export
badAgentlessScry : Unspellable (Effect []) (\ok =>
  Composite Scry (Macros.lookAt (Macros.topCards 1)) {na = ok})
badAgentlessScry MkNonAgentive impossible


||| "You scry 1", spelled over the bottom card of your library
||| [CR#701.22a] fixes the slice at the top N cards of the library.
public export
badScryBottom : Unspellable (Effect []) (\ok =>
  Does You Scry (Macros.lookAt Macros.bottomCard) {tb = ok})
badScryBottom ScryB impossible


||| "You scry 2", spelled with a reveal
||| [CR#701.22a] says "look at", and a reveal shows the cards to every player [CR#701.20a].
public export
badScryReveals : Unspellable (Effect []) (\ok =>
  Does You Scry (Macros.revealCards (Macros.topCards 2)) {tb = ok})
badScryReveals ScryB impossible


||| "Add." — a production that names no mana at all.
||| A producing effect instructs a player to add that mana [CR#106.3], so there is always mana to name.
public export
badEmptyProduction : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [] {ok}) [])
badEmptyProduction MkProducedRuns impossible

||| "Add {R} or ." — one alternative producing nothing beside one that does.
||| [CR#106.3]'s refusal one level in: an alternative that produces nothing names no mana.
public export
badEmptyAlternative : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [[OfColor Red], []] {ok}) [])
badEmptyAlternative MkProducedRuns impossible

||| "Add one mana in any combination of colors."
||| At one mana the freedom axis is vacuous: one unit takes one color either way [CR#106.1a].
public export
badLoneCombination : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (AnyColor EachColor) [] {ff = ok})
badLoneCombination MkFreedomFits impossible

||| "Add {C}. Spend this mana only."
||| [CR#106.6] makes the rider a restriction on how the mana can be spent, which needs a purpose.
public export
badPurposelessSpend : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [[Colorless]]) [SpendOnly [] {ne = ok}])
badPurposelessSpend MkSpendPurposes impossible

||| "Add {C}. Spend this mana only to activate an ability of one you control."
||| Where the restriction names the ability's source [CR#113.7] it describes it with a head noun.
public export
badHeadlessSpendSource : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [[Colorless]])
          [SpendOnly [ToActivate (Just (ControlledBy You)) {hd = ok}]])
badHeadlessSpendSource MkMaybeHeaded impossible


||| "This artifact is a copy of target artifact." — as a card's own line.
||| [CR#707.4] is about an effect that causes a permanent to copy, and an effect wants a carrier.
public export
badStandingCopy : Unspellable Ability (\ok =>
  Static (BecomesCopy Macros.thisArtifact (Macros.target Macros.artifact) []) {ln = ok})
badStandingCopy MkStaticLine impossible

||| "This creature becomes a copy of target creature until end of combat."
||| The end-of-combat endpoint is the type set's; the copy effect is a separate row.
public export
badCopyUntilEndOfCombat : Unspellable (Effect []) (\ok =>
  Continuously (BecomesCopy Macros.thisCreature (Macros.target Macros.creature) [])
               (Just (Until (EndOf Combat Nothing))) {sp = ok})
badCopyUntilEndOfCombat SpanStated impossible

||| "Target creature card in your graveyard becomes a copy of target creature."
||| [CR#707.4] keeps the copying permanent on the battlefield.
public export
badGraveyardBecomesCopy : Unspellable (StaticEffect []) (\ok =>
  BecomesCopy (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
              (Macros.target Macros.creature) [] {zn = ok})
badGraveyardBecomesCopy MkZoneFits impossible

||| "Create a token that's a copy of it, except it's in addition to its other types."
||| An exception that adds no word is not an exception; the tail names at least one type.
public export
badEmptyCopyTypeException : Unspellable (Effect []) (\ok =>
  Create You (Lit 1)
         (TokenCopyOf (Macros.target Macros.creature)
                      [ExceptTypes (MkTypeLine [] []) {ne = ok}]) [])
badEmptyCopyTypeException MkLineNonEmpty impossible

||| "Copy target creature."
||| The stack verbs act on an object on the stack [CR#707.10,112.1], which a permanent is not.
public export
badCopyPermanent : Unspellable (Effect []) (\ok =>
  CopyStack You (Macros.target Macros.creature) (Lit 1) [] {zn = ok})
badCopyPermanent OnTheStack impossible


||| "Choose new targets for target creature."
||| [CR#707.10c] gives the retarget a spell or ability, and a permanent is no longer on the stack [CR#112.1].
public export
badRetargetPermanent : Unspellable (Effect []) (\ok =>
  ChooseNewTargets (Macros.target Macros.creature) {zn = ok})
badRetargetPermanent OnTheStack impossible


||| "Copy target instant or sorcery spell. Untap that token."
||| A copy of a permanent spell becomes a token only as it resolves [CR#707.10f]; until then it is on the stack.
public export
badStackCopyAsToken : Unspellable (Effect []) (\ok =>
  Sequentially [CopyStack You (Macros.target (And [Macros.instantOrSorcery, Macros.spell]))
                          (Lit 1) [],
                SetStatus Untapped (That TokenW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badStackCopyAsToken (Refl, _) impossible


||| "Create a token that's a copy of target creature. Untap that copy."
||| The mirror: a token is a battlefield marker [CR#111.1], and "copy" names the stack object [CR#707.10].
public export
badTokenCopyAsCopyMention : Unspellable (Effect []) (\ok =>
  Sequentially [Create You (Lit 1) (TokenCopyOf (Macros.target Macros.creature) []) [],
                SetStatus Untapped (That CopyW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badTokenCopyAsCopyMention (Refl, _) impossible
