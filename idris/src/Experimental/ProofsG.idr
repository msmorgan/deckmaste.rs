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



||| "Unearth"
||| [CR#702.84a] states the keyword as "Unearth [cost]", so the parameter is never absent.
public export
badBareUnearth : Unspellable Ability (\ok =>
  KeywordAbility Unearth {pf = ok})
badBareUnearth Oh impossible


||| "Retrace {1}"
||| [CR#702.81a] writes retrace bare, so no cost stands beside the word.
public export
badCostedRetrace : Unspellable Ability (\ok =>
  KeywordAbility Retrace {param = Just (ParamCost (Mana [Macros.generic 1]))}
                 {pf = ok})
badCostedRetrace Oh impossible


||| "Unearth {B}" printed on an instant.
||| [CR#702.84a] returns the card to the battlefield, which [CR#110.4] denies an instant card.
public export
badUnearthOnSpellCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [KeywordAbility Unearth
          {param = Just (ParamCost (Mana [Macros.pip Black]))}] Nothing {tx = ok})
badUnearthOnSpellCard Oh impossible


||| "Flashback {2}{U}" printed on an artifact.
||| [CR#702.34a] permits the graveyard cast only if the resulting spell is an instant or sorcery.
public export
badFlashbackOnPermanentCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2, Macros.pip Blue]) []
       (MkTypeLine [] [Artifact])
       [KeywordAbility Flashback
          {param = Just (ParamCost (Mana [Macros.generic 2, Macros.pip Blue]))}]
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
