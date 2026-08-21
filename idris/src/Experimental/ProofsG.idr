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


||| "if you activated a loyalty ability this turn"
||| An activation is an activation of something: with no complement the clause names no event.
public export
badBareActivationLookback : Unspellable (Condition []) (\ok =>
  Happened AbilityActivation You Lookback.ThisTurn {cw = LeftBare {ok = ok}})
badBareActivationLookback Oh impossible


||| "At the beginning of you's upkeep, draw a card."
||| The possessive slot takes the attachment anaphor; every other possessor is a word.
public export
badNounPossessorYou : Unspellable Ability (\ok =>
  Triggered At (BeginningOf Upkeep (ByNoun You {pn = ok})) Macros.drawACard)
badNounPossessorYou AttachedPossessor impossible


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

