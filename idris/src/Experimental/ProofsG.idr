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


||| "This deals 4 damage to target permanent or player. Destroy that permanent or player."
||| The anaphor inherits the head's refusals: it stands in no zone [CR#110.1] and may denote a player [CR#102.1].
public export
badDestroyUnionAnaphor : Unspellable (Effect []) (\ok =>
  Sequentially [ DealDamage This (Lit 4)
                   (Macros.target (KindJoin JoinAnyPlayer JoinPermanent))
               , Macros.destroy (That JoinW)
                   {ok = Builtin.fst ok, na = Builtin.snd ok} ])
badDestroyUnionAnaphor (OnField, _) impossible


||| "each creature card in your graveyard with a +1/+1 counter on it"
||| Counters cease to exist when their object changes zones [CR#122.2], so a counter row admits no graveyard.
public export
badCounterDescriptionInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, InZone (Macros.graveyardOf You),
       HasCounters (Just Macros.plusOnePlusOne)] {zc = ok})
badCounterDescriptionInGraveyard Oh impossible


||| "each creature with a poison counter on it"
||| [CR#122.1] places a counter on an object or a player, and poison is a player's kind.
public export
badPoisonCounterDescription : Unspellable (Predicate [] Object) (\ok =>
  HasCounters (Just Poison) {kn = KindNamed {sc = ok}})
badPoisonCounterDescription Refl impossible


||| "each with a counter on it"
||| The qualifier is never the head; a postnominal clause hangs on a noun already written.
public export
badHeadlessCounterDescription : Unspellable (Noun [] Object) (\ok =>
  Each (HasCounters Nothing) {hd = ok})
badHeadlessCounterDescription Oh impossible


||| "Cumulative upkeep"
||| [CR#702.24a] states the keyword as "Cumulative upkeep [cost]", so the parameter is never absent.
public export
badBareCumulativeUpkeep : Unspellable Ability (\ok =>
  KeywordAbility CumulativeUpkeep {pf = ok})
badBareCumulativeUpkeep Oh impossible


||| "Cumulative upkeep {2}" printed on a sorcery.
||| [CR#702.24a] expands the keyword into clauses about a permanent on the battlefield.
public export
badCumulativeUpkeepOnSpell : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility CumulativeUpkeep
          {param = Just (ParamCost (Mana [Macros.generic 2]))}] Nothing {tx = ok})
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
                             (Gains Macros.thisCreature (KeywordAbility Flying))))
                  [] {lk = ok})
badEmptyKeywordList Oh impossible


||| "This creature has flying as long as a card exiled with it has flying. The same is true for menace, flying, and trample."
||| The list is disjoint from the base; a repeat would write the same sentence twice.
public export
badKeywordListRepeatingBase : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword Flying]))
                             (Gains Macros.thisCreature (KeywordAbility Flying))))
                  [Menace, Flying, Trample] {lk = ok})
badKeywordListRepeatingBase Oh impossible


||| "{T}: Draw a card. Activate only if you created this turn."
||| A creation is a creation of something: with no complement the clause names no event.
public export
badBareTokenCreationLookback : Unspellable (Condition []) (\ok =>
  Happened TokenCreation You Lookback.ThisTurn {cw = LeftBare {ok = ok}})
badBareTokenCreationLookback Oh impossible


||| "Destroy target creature that dealt combat damage this turn."
||| The combat-damage history read names whom the damage was dealt to.
public export
badBareCombatDamageLookback : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo CombatDamage Lookback.ThisTurn {cw = LeftBare {ok = ok}})
badBareCombatDamageLookback Oh impossible


||| "Destroy target creature that attacked with this creature this turn."
||| A creature attacks and does not attack with anything, so the complement's sort refuses.
public export
badAttackerComplementOnObject : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo AttackDeclaration Lookback.ThisTurn
    {what = Just (Involving Macros.thisCreature {cp = ok})})
badAttackerComplementOnObject MkLookbackComplement impossible


||| "When this creature enters, draw a card if you've cast an opponent this turn."
||| The complement has a sort and the cast's is an object: one does not cast a player.
public export
badPlayerCastComplement : Unspellable (Condition []) (\ok =>
  Happened SpellCast You Lookback.ThisTurn
    {what = Just (Involving Macros.anOpponent {cp = ok})})
badPlayerCastComplement MkLookbackComplement impossible


||| "Counter target spell cast from the battlefield."
||| [CR#601.2a] moves a card from where it is to the stack, and a permanent got there by being cast.
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


||| "Counter target spell cast from the top of your library."
||| The origin names a whole zone, never a position inside one.
public export
badCastFromLibraryTop : Unspellable (Predicate [] Object) (\ok =>
  CastFrom (LibraryAt (OneEnd OnTop) Nothing (OwnedBy You)) {wz = ok})
badCastFromLibraryTop Oh impossible


||| "Put target creature into its owner's library second from the bottom."
||| [CR#401.7] states the construction as "Nth from the top".
public export
badBottomOrdinal : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt (OneEnd OnBottom) Nothing {off = Just Second} {ofit = ok} Bare)
badBottomOrdinal Oh impossible


||| "Put those cards into their owner's library third from the top in any order."
||| One place holds one card [CR#401.7], so there is no order to state [CR#401.4].
public export
badOrdinalOrderRider : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt (OneEnd OnTop) (Just AnyOrder) {off = Just Third} {ofit = ok} Bare)
badOrdinalOrderRider Oh impossible


||| "Put two target creatures into their owners' libraries third from the top."
||| The same fact asked of the patient: [CR#401.7]'s ordinal wants one card.
public export
badPluralOrdinalPlacement : Unspellable (Effect []) (\ok =>
  Move (TargetGroup (Macros.exactly 2) Macros.creature)
       (Macros.nthFromTop Third) {arr = ok})
badPluralOrdinalPlacement Oh impossible


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


||| "If you don't control an artifact and an enchantment, …"
||| The negation goes on the conjuncts and never on the coordination.
public export
badNegatedCondConjunction : Unspellable (Condition []) (\ok =>
  NotCond (AndCond [ Exists (And [Macros.artifact, ControlledBy You])
                   , Exists (And [Macros.enchantment, ControlledBy You]) ]) {ng = ok})
badNegatedCondConjunction Oh impossible


||| "Unless you control an artifact and an enchantment, you may cast this spell without paying its mana cost."
||| "Unless" reads a negated condition, and a coordination is not negated at its own frame.
public export
badUnlessConjunction : Unspellable (StaticEffect []) (\ok =>
  Conditionally (AndCond [ Exists (And [Macros.artifact, ControlledBy You])
                         , Exists (And [Macros.enchantment, ControlledBy You]) ])
                (AltCost Nothing) {marking = Unless} {mk = ok})
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


||| "For each of them, for each creature, draw a card."
||| The binder refuses a second binding in its own body, though the body may be a sequence.
public export
badNestedForEach : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup Macros.anyNumber Macros.creatureYouControl),
                ForEachOf Them (ForEachOf (Each Macros.creature) (Draw You (Lit 1)))
                          {nf = ok}])
badNestedForEach Oh impossible


||| "Draw a card. Repeat this process a number of times equal to the number of creatures you control."
||| A repetition's count is written, never computed off the board.
public export
badCountedRepeat : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.drawACard,
                Repeat (MoreTimes (Macros.forEach Macros.creatureYouControl)
                                  {rc = ok})])
badCountedRepeat Oh impossible


||| "Draw a card. Repeat this process zero more times."
||| A repetition spelled zero times is no instruction [CR#107.1b].
public export
badZeroRepeat : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.drawACard, Repeat (MoreTimes (Lit 0) {rc = ok})])
badZeroRepeat Oh impossible


||| "of the chosen colour or outcome"
||| The kind join is gated to the one cross-kind pair English writes [CR#115.4];
||| a quality and an outcome have no join.
public export
badJoinQualityOutcome : Unspellable Kind (\ok =>
  (\/) (Quality Color) Outcome {ok = ok})
badJoinQualityOutcome Oh impossible


||| "this turn or target creature"
||| A turn reference names a time, and no join carries it to an object.
public export
badJoinTurnRefObject : Unspellable Kind (\ok =>
  (\/) TurnRef Object {ok = ok})
badJoinTurnRefObject Oh impossible


||| "X or target player"
||| A letter word names a chosen number, and no join carries it to a player.
public export
badJoinLetterPlayer : Unspellable Kind (\ok =>
  (\/) (Letter LetterX) Player {ok = ok})
badJoinLetterPlayer Oh impossible


-- ...and the joins the gate DOES admit, pinned by Refl: "any target"
-- [CR#115.4] in both orders, the absorption, and the like-with-like case.
public export
joinObjectPlayer : Object \/ Player = ObjectOrPlayer
joinObjectPlayer = Refl

public export
joinPlayerObject : Player \/ Object = ObjectOrPlayer
joinPlayerObject = Refl

public export
joinAbsorbsObject : ObjectOrPlayer \/ Object = ObjectOrPlayer
joinAbsorbsObject = Refl

public export
joinObjectObject : Object \/ Object = Object
joinObjectObject = Refl


||| "{T}: Draw a card. Activate only during each player's turn, before attackers are declared."
||| The boundary-relative window takes the three possessors the corpus writes in front of
||| the point; a quantifier over every turn is not one of them.
public export
badBeforeAttackersEachPlayers : Unspellable Ability (\ok =>
  Activated TapSymbol Macros.drawACard
            {window = Just (BeforePoint AttackersDeclared (Just EachPlayers) {pk = ok})})
badBeforeAttackersEachPlayers Oh impossible


||| "if you activated a loyalty ability this turn"
||| An activation is an activation of something: with no complement the clause names no event.
public export
badBareActivationLookback : Unspellable (Condition []) (\ok =>
  Happened AbilityActivation You Lookback.ThisTurn {cw = LeftBare {ok = ok}})
badBareActivationLookback Oh impossible


||| "When you next activate a loyalty ability this turn, draw a card."
||| No delayed activation line is spellable: three coordinate the activation with a cast or
||| hang a payment rider, and the fourth's complement is an exhaust ability. So the event
||| vocabulary claims the trigger and not the delay.
public export
badDelayedActivation : Unspellable (Effect []) (\ok =>
  Delayed (Activates You (Macros.a (AbilityHead LoyaltyClass))) Macros.drawACard {aw = ok})
badDelayedActivation Oh impossible


||| "At the beginning of enchanted player's combat phase, draw a card."
||| The nominal possessor reaches the three steps the Curses write it at, and a
||| new part does not inherit the slot.
public export
badNounPossessorAtCombat : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Combat (ByNoun (AttachHost Enchanted PlayerW)) {pu = ok})
            Macros.drawACard)
badNounPossessorAtCombat Oh impossible


||| "At the beginning of you's upkeep, draw a card."
||| The possessive slot takes the attachment anaphor; every other possessor is a word.
public export
badNounPossessorYou : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Upkeep (ByNoun You {pn = ok})) Macros.drawACard)
badNounPossessorYou AttachedPossessor impossible


||| "Search each player's graveyard, hand, and library for a creature card."
||| The sweep's possessor slot refuses every group word; no line names the
||| zones of a group.
public export
badSweepAcrossPlayers : Unspellable (Effect []) (\ok =>
  Search You (GraveyardHandLibraryOf (PlayerGroup AllPlayers) {pn = ok})
         Macros.creature)
badSweepAcrossPlayers Oh impossible


||| "Put target creature into its owner's hand." tagged as a placement.
||| The imperative spells the bare move and the agentive names its subject;
||| one event, so the tag has no subjectless frame.
public export
badSubjectlessPut : Unspellable (Effect []) (\ok =>
  Composite Put (Move (Macros.target Macros.creature) Macros.handZ) {na = ok})
badSubjectlessPut Oh impossible


||| "Put those cards on the top or bottom of your library in any order."
||| [CR#401.4] arranges cards sharing one position; a disjunction names two
||| ends, so there is no single pile to order.
public export
badDisjunctionOrdered : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt (EitherEnd Nothing) (Just AnyOrder) {af = ok} Bare)
badDisjunctionOrdered Oh impossible


||| "Search each opponent's graveyard, hand, and library for a creature card."
||| The other group word, refused by the same gate where `Possessor` admits it.
public export
badSweepAcrossOpponents : Unspellable (Effect []) (\ok =>
  Search You (GraveyardHandLibraryOf (PlayerGroup YourOpponents) {pn = ok})
         Macros.creature)
badSweepAcrossOpponents Oh impossible


||| "Target player puts that card into exile."
||| The exile tag owns that destination; no line writes a placement into it.
public export
badPutIntoExile : Unspellable (Effect []) (\ok =>
  Does (Macros.target AnyPlayer) Put
       (Move (Macros.target Macros.creature) Macros.exileZ)
       {tb = PutB {pz = ok}})
badPutIntoExile Oh impossible
