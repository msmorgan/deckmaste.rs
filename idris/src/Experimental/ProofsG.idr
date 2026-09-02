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


||| "Cumulative upkeep {2}" printed on a sorcery.
||| [CR#702.24a] expands the keyword into clauses about a permanent on the battlefield.
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
  AlsoForKeywords (Static (Gets (AllOf Macros.creatureYouControl)
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
||| Every element of the trailer is written BARE, and ward is not: [CR#702.21a]
||| writes a cost after the word. The class term did not relax this -- a WORD
||| arm still faces `keywordParamless`, exactly as before.
public export
badParameterisedKeywordInList : Unspellable Ability (\ok =>
  AlsoForKeywords (Static (Conditionally
                             (Exists (And [ExiledWith Macros.thisCreature,
                                           HasKeyword (TheKeyword "Flying")]))
                             (Gains Macros.thisCreature (KeywordAbility "Flying" Nothing)) AsLongAs))
                  [TheKeyword "Ward"] {lk = ok})
badParameterisedKeywordInList Oh impossible


||| "a creature with flyings"
||| A keyword CLASS quantifies over its word's parameter, and [CR#702.9a]
||| writes no parameter after flying, so there is nothing for the term to
||| quantify: the class of flying abilities is the one ability, which the
||| word already names.
public export
badClassOfParamlessKeyword : Unspellable (Predicate [] Object) (\ok =>
  HasKeyword (AnyKeywordIn (MkKeywordFamily "Flying" Nothing)) {kn = ok})
badClassOfParamlessKeyword Oh impossible


||| "a creature with renown of any color"
||| The class term's SORT narrows a quality [CR#105.1]; [CR#702.112a] writes a
||| number after renown, and a number has no sorts to range over.
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
||| A locus is where an act was PERFORMED, and a death is no act performed in a place: [CR#700.4] makes it a move from the battlefield to a graveyard, whose two ends are an origin and a destination and neither of them a place the act happened in.
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
||| [CR#110.4b] lists the five spell types that become permanents and an instant is not among them; [CR#608.2n] puts it into its owner's graveyard as the last part of resolving, so there is no permanent for the clause to name.
public export
badResolvedInstant : Unspellable (Noun [] Object) (\ok =>
  ResolvedPermanent (Macros.a (And [HasType Instant, Macros.spell])) {pm = ok})
badResolvedInstant Oh impossible


||| "this creature, once it resolves"
||| The read is the stack-to-battlefield transition [CR#608.3a]; a mention already on the battlefield has made it, and naming what it becomes names nothing further.
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
                (AltCost Nothing) Unless {mk = ok})
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
                   Nothing {un = ok})
badNamedAddition Oh impossible


||| "Target creature becomes a black black Zombie in addition to its other colors and types."
||| A colour list is a set [CR#105.2], so no phrase writes a colour twice.
||| Refused by `TokenCanonical`, which stands at the addition where
||| `ColorsDistinct` did and asks the setting row's whole distinctness.
public export
badRepeatedAdditionColor : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [Black, Black] (MkTypeLine [creatureType "Zombie"] []) [] Nothing)
                   Nothing {tc = ok})
badRepeatedAdditionColor Oh impossible


||| "Target creature becomes an artifact artifact in addition to its other types."
||| The distinctness half of the same gate, which the addition row did not
||| ask before: [CR#205.1b] retains the prior types and this line names
||| what is added, so a card type named twice adds nothing the first
||| mention did not.
public export
badRepeatedAdditionType : Unspellable (Effect []) (\ok =>
  Macros.becomesAs (Macros.target Macros.creature)
                   (MkToken Nothing [] (MkTypeLine [] [Artifact, Artifact]) [] Nothing)
                   Nothing {tc = ok})
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
  Triggered At (BeginningOf Upkeep (ByNoun You {pn = ok})) [] Nothing [] Nothing Nothing Nothing Macros.drawACard)
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


||| "Shuffle those cards into your library in any order."
||| [CR#401.4] gives the owner an order for cards put "in a specific
||| position". A shuffle randomizes the whole pile [CR#701.24a] and puts
||| them in no position at all, so there is nothing for an arrangement to
||| order.
public export
badShuffledArranged : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt Shuffled (Just AnyOrder) Nothing {af = ok} Bare)
badShuffledArranged Oh impossible


||| "Shuffle it into its owner's library third from the top."
||| [CR#401.7] counts the offset down from the top card; a shuffle names
||| no position for it to count from [CR#701.24a]. The reading this
||| leaves standing is Gravebane Zombie's beside Darksteel Colossus
||| [CR#701.24g] -- a SEPARATE clause states the position, and the two
||| destinations are two moves.
public export
badShuffledOrdinal : Unspellable (ZoneExpr []) (\ok =>
  LibraryAt Shuffled Nothing (Just (Nth 3)) {nf = ok} Bare)
badShuffledOrdinal Oh impossible


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
  KeywordAbility "Unearth" Nothing {pf = ok})
badBareUnearth Oh impossible


||| "Retrace {1}"
||| [CR#702.81a] writes retrace bare, so no cost stands beside the word.
public export
badCostedRetrace : Unspellable Ability (\ok =>
  KeywordAbility "Retrace" (Just (ParamCost (Mana [Macros.generic 1])))
                 {pf = ok})
badCostedRetrace Oh impossible


||| "Unearth {B}" printed on an instant.
||| [CR#702.84a] returns the card to the battlefield, which [CR#110.4] denies an instant card.
public export
badUnearthOnSpellCard : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Black]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Unearth" (Just (ParamCost (Mana [Macros.pip Black])))] Nothing {tx = ok})
badUnearthOnSpellCard Oh impossible


||| "Flashback {2}{U}" printed on an artifact.
||| [CR#702.34a] permits the graveyard cast only if the resulting spell is an instant or sorcery.
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
  Static (Gains (AllOf (And [Macros.creature, InZone (Macros.graveyardOf You)]))
                (Macros.keywordCosting "Warp" (Mana [Macros.generic 2]))
                {ok = ok}))
badWarpGrantInGraveyard Oh impossible


-- A WITNESS, not a pin: with the join a plain constructor, no cross-kind
-- pair is refused any more, so the pairs the old join gate would not
-- admit are simply written.

||| THE COPY PARTICIPLE'S MEASURED ZERO, held explicitly so that a
||| widening cannot pass it silently. "The copied spell" and "copied this
||| way" are ZERO supported lines apiece (re-measured 2026-09-02), and
||| two things keep them unwritable: "Copy" is not among the verb labels
||| at all, so `participleOf` finds no word to spell, and the copy verbs
||| mint no `VerbName` stamp for a participial read to count. This proof
||| holds the first, which is the one a new verb-facts row could undo.
||| Its sibling is the table: `verbedWordOk` answers `False` for the copy
||| words themselves, at all three kinds.
public export
copyParticipleUnwritten : verbedMarkingOk "Copy" Attributive = False
copyParticipleUnwritten = Refl

||| "Change the target of target spell or ability with a single target."
||| (Bolt Bend.) `Object \/ Ability` is writable, and its payload carries
||| each half separately: the spell half is a card on the stack [CR#112.1],
||| while `AbilityP` carries no zone field to place the ability half with.
||| `Joined Macros.spell (AbilityHead AnyOnStack)` now spells the head
||| itself (Diplomatic Escort, Shimmering Glasskite); this stays as the
||| payload's own witness, one half at a time.
public export
spellOrAbilityJoin : Payload (Object \/ Ability)
spellOrAbilityJoin = JoinP (ObjectP Nothing (Just Stack) Nothing Nothing Nothing)
                             (AbilityP Nothing)

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

||| The six-verb refusal's own fact. A joined phrase places nothing —
||| [CR#400.1] makes a zone a place where objects can be and [CR#109.1]
||| lists what an object is — so destroy, exile, tap, untap, return and
||| sacrifice each refuse it at their `Noun bs Object` slot, with no rule
||| written for the purpose, while the damage clause admits it because
||| damage asks for no zone [CR#120.1]. Counter left the family when
||| [CR#701.6a]'s "spell or ability" gave it a joined subject of its own:
||| it now asks `counterKind`, which refuses this union on [CR#109.1]
||| directly rather than through the missing zone.
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

||| an activated ability printed on a conspiracy card
||| [CR#315.5] gives a conspiracy card "any number of static or triggered
||| abilities" and licenses only those from the command zone, where
||| [CR#311.4], [CR#313.4] and [CR#314.4] each name an activated one too. The
||| shared command-zone frame admits it; the type's own rule does not.
public export
badConspiracyActivated : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Conspiracy])
       [ Macros.activated (Mana [Macros.generic 1]) Macros.drawACard ] Nothing {tx = ok})
badConspiracyActivated Oh impossible

||| a static ability printed on a dungeon card
||| [CR#309.4c] writes out the full text of every ability a dungeon card has --
||| one triggered room ability per room -- and licenses their triggering and
||| nothing else, so no static ability sits on the card.
public export
badDungeonStatic : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Dungeon])
       [ Static (Macros.entersWithCounters Macros.thisCreature (Lit 1)
                   Macros.plusOnePlusOne) ] Nothing {tx = ok})
badDungeonStatic Oh impossible


||| "your opponents' devotion to black"
||| [CR#700.5] defines devotion per player -- a count among the mana costs of
||| permanents ONE player controls -- so a plural possessor reads no total.
public export
badPluralDevotion : Unspellable (Amount []) (\ok =>
  Devotion (PlayerGroup YourOpponents) (LitColor Black) Nothing {one = ok})
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

||| "Whenever a creature attacks a planeswalker or a creature"
||| [CR#506.3] closes the set an attack may name, and a joined defender is
||| attacked on EACH half's own account, so each half is asked about its
||| own head type. The planeswalker half passes and the creature half does
||| not; writing the halves the other way round changes nothing.
public export
badMixedAttackDefenderHalves : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a (Joined (HasType Planeswalker)
                                         (HasType Creature))) {at = ok}))
badMixedAttackDefenderHalves Oh impossible

||| "This deals 3 damage to a land or a land."
||| [CR#120.1] states the whole recipient set — battles, creatures,
||| planeswalkers and players — and a joined phrase is dealt damage on each
||| half's own account, so a half that names a card type must name one on
||| the list. The repetition is not what refuses it: "a land or an
||| enchantment" falls the same way, and a half naming NO type still passes:
||| it names nothing off the set, which [CR#120.1a] bounds.
public export
badSameKindJoinDamage : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3)
             (Macros.a (Joined (HasType Land) (HasType Land))) {rk = ok})
badSameKindJoinDamage JoinTakes impossible


||| "creature that could block target creature card in your graveyard"
||| The hypothetical block is asked of a block that could be declared, and
||| [CR#509.1g] holds the relation between creatures in combat, so the
||| relatum names no other zone. `BlockerOf`'s relatum falls the same way.
public export
badCouldBlockGraveyardRelatum : Unspellable (Predicate [] Object) (\ok =>
  CouldBlock (Macros.target (And [Macros.creature,
                                  InZone (Macros.graveyardOf You)])) {zn = ok})
badCouldBlockGraveyardRelatum Oh impossible


||| "Target land blocks an attacking creature."
||| [CR#506.3]: "Only a creature can attack or block." The write puts a
||| permanent into a blocking assignment, so it takes the block's agent
||| type, and no land is one.
public export
badLandBecomesBlocking : Unspellable (Effect []) (\ok =>
  BecomesBlocking (Macros.target Macros.land)
                  (Macros.a (And [Macros.creature, Attacking])) {dn = ok})
badLandBecomesBlocking Participant impossible


||| "This creature blocks target planeswalker."
||| What a blocker is assigned to is an attacking creature [CR#509.1a], and
||| [CR#506.3] puts a planeswalker on the attacked side of combat, never
||| the attacking one, so it is never a thing blocked.
public export
badBecomesBlockingPlaneswalker : Unspellable (Effect []) (\ok =>
  BecomesBlocking Macros.thisCreature
                  (Macros.target (HasType Planeswalker)) {dw = ok})
badBecomesBlockingPlaneswalker Participant impossible


||| "During each opponent's next turn, ..."
||| [CR#102.1] makes the active player the player whose turn it is, so a
||| turn has one possessor and a span naming several names no turn.
public export
badPluralNextTurnSpan : Unspellable (Duration []) (\ok =>
  DuringNextTurnOf (Each Opponent) {one = ok})
badPluralNextTurnSpan Refl impossible


||| "Take an extra turn for each coin that comes up heads." with no coin
||| flipped.
||| [CR#705.2] gives the face to a coin some effect instructed a player to
||| flip; with no flip written there is no coin to have come up either way.
public export
badCoinsShowingWithoutFlip : Unspellable (Effect []) (\ok =>
  ExtraTurn You (CoinsShowing Heads {fl = ok}))
badCoinsShowingWithoutFlip Oh impossible

||| "If you rolled 7, sacrifice this creature." with no roll before it.
||| [CR#706.2] makes a result the number a die the text rolled came up on,
||| so a total over those results presupposes the roll, exactly as the
||| singular result does.
public export
badTotalWithoutRoll : Unspellable (Effect []) (\ok =>
  Macros.ifThen (CompareAmt (TheTotal {ok}) Eq (Lit 7))
                (Macros.sacrifice You Macros.thisCreature))
badTotalWithoutRoll Refl impossible

||| "creature that won a coin flip this turn"
||| [CR#705.2] has the player who flips the coin win or lose the flip and
||| says no other player is involved, so what won one is a player and a
||| creature never is.
public export
badCreatureWonFlip : Unspellable (Predicate [] Object) (\ok =>
  HappenedTo FlipWin Lookback.ThisTurn Nothing {sb = ok})
badCreatureWonFlip MkLookbackSubject impossible

||| "the amount of dice you rolled this turn"
||| [CR#706.2] makes the result a number the roll PRODUCED, read back off
||| the roll, not an amount the rolling happened in; how many dice were
||| rolled is a count and not a magnitude.
public export
badRollAsMagnitude : Unspellable (Amount []) (\ok =>
  EventSum DiceRoll You Lookback.ThisTurn Nothing {qm = ok})
badRollAsMagnitude Oh impossible

||| The readings those pins leave standing, so none passes for want of a
||| positive: a player's own won flip and rolled die are both looked back
||| on ("if you rolled a die this turn").
public export
youWonAFlipThisTurn : Condition []
youWonAFlipThisTurn = Happened FlipWin You Lookback.ThisTurn Nothing

public export
youRolledADieThisTurn : Condition []
youRolledADieThisTurn = Happened DiceRoll You Lookback.ThisTurn Nothing

||| "Ignore the lowest roll." with no roll before it.
||| [CR#706.6] makes an ignored roll one that "is considered to have never
||| happened", which presupposes a roll that did; with nothing rolled
||| there is no roll to set aside.
public export
badIgnoreWithoutRoll : Unspellable (Effect []) (\ok =>
  IgnoreOutcomes (IgnoreExtreme LowestRoll) {ok})
badIgnoreWithoutRoll Oh impossible

||| "Store those results on this creature." with no roll before it.
||| [CR#706.8a] stores "both the kind of die rolled and the result of that
||| roll", so the storing names a roll the text has already made.
public export
badStoreResultsWithoutRoll : Unspellable (Effect []) (\ok =>
  StoreResults Macros.thisCreature {ok})
badStoreResultsWithoutRoll Refl impossible

||| "Roll that many dice." with no roll announced before it.
||| [CR#706.1] has a rolling instruction specify what kind of die to roll;
||| the anaphoric arm specifies none of its own, taking the kind an
||| announced roll already carried, so with nothing announced it names no
||| die. `wyllExtraDie` is the same word written where the announcement
||| stands.
public export
badAnaphoricSidesWithoutRoll : Unspellable (Effect []) (\ok =>
  RollDice You (Lit 1) (ThoseDice {ok}))
badAnaphoricSidesWithoutRoll Refl impossible

||| "If you rolled doubles, sacrifice this creature." with no roll.
||| [CR#706.5] defines the phrase over "each of those rolls", which is the
||| roll the clause made; with none made the phrase compares nothing.
public export
badRolledDoublesWithoutRoll : Unspellable (Condition []) (\ok =>
  RolledDoubles {ok})
badRolledDoublesWithoutRoll Refl impossible

||| The discourse after "Flip a coin."
public export
afterACoinFlip : Bindings
afterACoinFlip = effIntro (the (Effect []) Macros.flipACoin)

||| "an ability whose coin comes up tails"
||| [CR#705.1] makes a coin flip a randomisation with two faces and
||| [CR#705.2] gives the face to the coin some effect had flipped for a
||| player or for an object; an ability is neither, so no coin is ever
||| flipped for one.
public export
badCoinCameUpOnAbility :
  Unspellable (Predicate ProofsG.afterACoinFlip Ability) (\ok =>
    CoinCameUp Tails {rk = ok})
badCoinCameUpOnAbility Oh impossible

||| "Whenever you roll a 0, …"
||| [CR#706.1a] numbers a die "from 1 to N", so a result test whose
||| ceiling is zero covers no result the roll can produce -- the same
||| ground on which a results-table row of zero is refused.
public export
badZeroRollTest : Unspellable (GameEvent []) (\ok =>
  RollsDice You OneDie AnyDie (ResultIn (Range Nothing (Just 0))
                                 {nz = ok} {wf = Oh} {lt = Oh}))
badZeroRollTest MaxAtLeastOne impossible

||| "Whenever you roll a 4 or higher on the planar die, …"
||| [CR#706.7] has any effect that refers to a numerical result of a die
||| roll -- naming the comparison of that result to a given number
||| outright -- ignore the rolling of the planar die, and [CR#901.9d]
||| repeats it; [CR#901.3a] numbers none of that die's six faces. So the
||| test names no result the roll can produce. `ichorElixirPlanarDice`
||| is the same narrowing written where no test stands, and
||| `atomwheelAcrobatsRoll` the same test written over a numbered die.
public export
badPlanarResultTest : Unspellable (GameEvent []) (\ok =>
  RollsDice You ManyDice PlanarDie
            (ResultIn (Range (Just 4) Nothing) {nz = UnboundedAbove}
                      {wf = Oh} {lt = Oh})
            {dw = ok})
badPlanarResultTest Oh impossible

||| "If you would flip a coin, instead flip two coins and ignore the
||| lower one."
||| [CR#706.6] writes the superlative over rolls, whose results are
||| numbers [CR#706.2], so the ends `RollExtreme` names are the ends of
||| an order. [CR#705.1] gives a coin two faces and ranks neither, so a
||| flipped coin has no lowest. `krarksThumbExtraFlip` is the ignore that
||| IS printed over flips, and `berserkersFrenzyRoll` the superlative
||| written where results stand.
public export
badExtremeOverFlips :
  Unspellable (Effect ProofsG.afterACoinFlip) (\ok =>
    IgnoreOutcomes (IgnoreExtreme LowestRoll) {ok})
badExtremeOverFlips Oh impossible


||| "When a player doesn't pay this creature's flying, …"
||| [CR#118.1] makes a cost an action or payment necessary to take another
||| action; [CR#702.9a] states flying whole as an evasion ability and names
||| no cost, so the phrase names nothing to pay.
public export
badPayCostlessKeyword : Unspellable (GameEvent []) (\ok =>
  PaysCost (Just (Macros.a AnyPlayer)) Unpaid Macros.thisCreature "Flying" {kc = ok})
badPayCostlessKeyword Oh impossible

||| "if you paid a cost this turn"
||| A payment is a payment OF a stated cost [CR#118.1], and the participial
||| lookback carries only a kind-to-kind complement, which cannot name the
||| keyword and the bearer that say which cost was paid. Bare, the clause
||| names no event.
public export
badBarePaymentLookback : Unspellable (Condition []) (\ok =>
  Happened CostPayment You Lookback.ThisTurn Nothing {sb = ok})
badBarePaymentLookback MkLookbackSubject impossible

||| "Destroy the rest." with nothing chosen and no group assembled.
||| Reading a choice's own partition as the licence [CR#608.2d] does not
||| make the phrase free: what "the rest" is the rest OF must still have
||| had something taken out of it.
public export
badRestWithoutAPartition : Unspellable (Noun [] Object) (\ok =>
  TheRest {ok})
badRestWithoutAPartition Oh impossible

||| "Choose any number of target creatures. Destroy the rest."
||| A target is chosen as the spell is cast [CR#601.2c], not while the
||| effect is applied, so the clause naming one announces no
||| resolution-time choice [CR#608.2d] and partitions nothing.
public export
badRestAfterTargetChoice : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.choose (TargetGroup Macros.anyNumber Macros.creature)
               , Macros.destroy (TheRest {ok}) ])
badRestAfterTargetChoice Oh impossible

||| The discourse after "Choose up to one creature. Destroy the rest."
public export
afterChoiceRestDisposed : Bindings
afterChoiceRestDisposed =
  effIntro (the (Effect [])
    (Sequentially [ Macros.choose (CountedGroup (Macros.upTo 1) Nothing Macros.creature)
                  , Macros.destroy TheRest ]))

||| "Choose up to one creature. Destroy the rest. Destroy the rest."
||| One disposition per remainder in the choice's shape too: the first
||| disposal spends the partition, and the second phrase finds nothing
||| outstanding to be the rest of.
public export
badChoiceRestDisposedTwice :
  Unspellable (Noun ProofsG.afterChoiceRestDisposed Object) (\ok => TheRest {ok})
badChoiceRestDisposedTwice Oh impossible

||| "an opponent who controls more lands than they control"
||| The member-relative comparison binds its member for the MEASURED side
||| alone; the bound is read in the outer context, where no member of the
||| description stands. A comparison whose two sides were both the
||| member's would state nothing about which member the phrase picks.
public export
badMemberInComparisonBound : Unspellable (Predicate [] Player) (\ok =>
  CompareOver Opponent (CountOf (And [Macros.land, ControlledBy You]))
              Greater (CountOf (And [Macros.land, ControlledBy (They {ok})])))
badMemberInComparisonBound Refl impossible

||| "the number of basic creature types among creatures you control"
||| [CR#305.6] gives "basic land type" its only reading -- five of the land
||| types -- and no other card type's subtypes are divided that way, so the
||| scope has nothing to mean off the land type.
public export
badBasicCreatureTypeAxis : Unspellable (Amount []) (\ok =>
  DistinctCount (SubtypeAxis Creature BasicOnly {sc = ok})
                (AllOf Macros.creatureYouControl))
badBasicCreatureTypeAxis Oh impossible

||| "Echo"
||| [CR#702.30a] states the keyword as "Echo [cost]", so the parameter is
||| never absent -- cumulative upkeep's ground [CR#702.24a], at the second
||| keyword whose parameter is a cost.
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
||| [CR#702.24a] fixes WHO pays a cumulative upkeep, so the passive omits
||| nothing the rules leave open -- but it MENTIONS no one, and a
||| demonstrative here reads a mention. That is the whole difference the
||| voice slot carries: the active header hands Thought Lash its "that
||| player", the passive hands Balduvian Fallen only the bearer.
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

||| "Whenever you pay this enchantment's cumulative upkeep, put that many
||| counters on it."
||| The size of a keyword-named cost is the cost's own, and [CR#702.24a]
||| refuses a partial payment outright, so paying one writes no number for
||| the tail to name. Font of Agonies' life payment is the contrast:
||| [CR#118.3b] subtracts an indicated amount, so that payment does.
public export
badKeywordCostPaymentThatMuch :
  Unspellable (Amount ProofsG.afterKeywordCostPayment) (\ok => ThatMuch {ok})
badKeywordCostPaymentThatMuch Refl impossible

||| The reading those pins leave standing, so none passes for want of a
||| positive: a life payment names its own paid thing, so it is looked
||| back on bare ("if you paid life this turn") where a cost payment
||| cannot be.
public export
youPaidLifeThisTurn : Condition []
youPaidLifeThisTurn = Happened LifePayment You Lookback.ThisTurn Nothing

||| The discourse after "Look at the top card of your library. Shuffle."
public export
afterShuffledLook : Bindings
afterShuffledLook =
  effIntro (the (Effect [])
    (Sequentially [ Macros.lookAt Macros.topCard, Macros.shuffle ]))

||| "Look at the top card of your library. Shuffle. Put that card into
||| your hand."
||| [CR#701.24b] keeps out of a shuffle only the cards a SEARCH found.
||| Every other card in the pile is randomized where no player knows its
||| order [CR#701.24a] -- and a revealed one becomes a new object outright
||| [CR#701.20d] -- so the mention a bare look left does not survive. The
||| reading this leaves standing is Mystical Tutor's, where the search's
||| own stamp is what carries the found card across the shuffle.
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
    (Sequentially [ Macros.lookAt Macros.topCard, Macros.shuffleInto This ]))

||| "Look at the top card of your library. Shuffle this card into its
||| owner's library. Put that card into your hand."
||| The shuffle-into randomizes the library it lands in [CR#701.24c], so
||| it takes the discourse with it exactly as the bare shuffle does: the
||| mention the look left does not survive [CR#701.24a]. What pins the
||| move and not just the keyword action is that the destination alone
||| says so -- no `Shuffle` clause is written here.
public export
badReadsShuffledIntoLibraryCard :
  Unspellable (Noun ProofsG.afterShuffledIntoLook Object) (\ok => That CardW {ok})
badReadsShuffledIntoLibraryCard Refl impossible

||| "if up to three is 4 or greater"
||| A ceiling is a number the acting player announces as the effect
||| applies [CR#608.2d], not one standing in the game state, so nothing
||| is there for a comparison to measure. `badCompareLiteralSubject` pins
||| the numeral for the neighbouring reason -- it states arithmetic --
||| and the two together leave the subject slot to the reads.
public export
badCompareCeilingSubject : Unspellable (Condition []) (\ok =>
  CompareAmt (UpTo (Lit 3)) AtLeast (Lit 4) {rd = ok})
badCompareCeilingSubject Oh impossible

||| "This creature enters with your choice of a counter on it."
||| A menu is the range the clause's "you" picks an arm from, and at zero
||| arms there is nothing to pick: [CR#122.1] places a counter of some
||| name, and an empty list names none.
public export
badEmptyCounterMenu : Unspellable (StaticEffect []) (\ok =>
  EntersWithCounters Macros.thisCreature (Lit 1) (ChosenKind [] {ne = ok}) Fresh)
badEmptyCounterMenu IsNonEmpty impossible

||| "Put your choice of a +1/+1 counter or a poison counter on target
||| creature."
||| The menu case of `badPutPoisonOnCreature`: [CR#122.1] places a
||| counter on an object OR a player and poison is a player's
||| [CR#122.1f], so an arm at the other scope is an arm no one could
||| pick. The kind slot widens to a menu; what may hold the counter does
||| not.
public export
badMixedScopeCounterMenu : Unspellable (Effect []) (\ok =>
  PutCounters (Lit 1) (ChosenKind [Macros.plusOnePlusOne, Poison])
              (Macros.target Macros.creature) {sc = ok})
badMixedScopeCounterMenu Oh impossible

||| "For each color among permanents you control, add one mana of that
||| color", bridged through the count.
||| A counted iteration leaves its body the amount's own mentions and a
||| number; [CR#105.1]'s five colours are what the axis ranges over and
||| the count discards them, so "of that color" finds nothing to read.
||| This is why the distributive pass binds the value itself.
public export
badRepeatedCarriesNoColor : Unspellable (Effect []) (\ok =>
  Repeated (DistinctCount ColorAxis (AllOf (And [Permanent, ControlledBy You])))
           (AddMana You (Lit 1) (OfChosenColor Nothing {cq = ok}) []))
badRepeatedCarriesNoColor Refl impossible

||| "For each color among permanents you control, … of that creature
||| type."
||| The pass binds a value ON its axis: [CR#105.1]'s colours are what
||| `ColorAxis` ranges over and [CR#205.3e]'s subtypes are not among them,
||| so the crossing names a value the pass never had.
public export
badAxisValueCrossing : Unspellable (Effect []) (\ok =>
  ForEachKindOf ColorAxis (Just (AllOf (And [Permanent, ControlledBy You])))
                (SubtypeQ Creature) (Draw You (Lit 1)) {sc = ok})
badAxisValueCrossing Refl impossible

||| "For each creature type, …" -- the domainless pass at an UNCLOSED
||| axis. No rule closes the creature types; [CR#205.3m]'s list is a
||| printed one amended set by set, where [CR#105.1]'s five colours and
||| [CR#205.2a]'s card types are the game's own vocabulary. A pass with
||| no group to draw its values from and no rule to enumerate them names
||| a range that does not exist.
public export
badDomainlessOpenAxis : Unspellable (Effect []) (\ok =>
  ForEachKindOf (SubtypeAxis Creature AnySubtype) Nothing
                (SubtypeQ Creature) (Draw You (Lit 1)) {cl = ok})
badDomainlessOpenAxis Oh impossible

||| "target permanent that's exactly one color"
||| [CR#105.2a] gives exactly one of the five colours its own printed
||| word, "monocolored", and [CR#105.2c] does the same at none, so the
||| counted spelling has nothing left to say below two.
public export
badExactlyOneColor : Unspellable (Predicate [] Object) (\ok =>
  ExactlyColors 1 {ok = ok})
badExactlyOneColor Oh impossible

||| "target permanent that's exactly six colors"
||| [CR#105.1] closes the colours at five, so a sixth is a colour no
||| object could be.
public export
badExactlySixColors : Unspellable (Predicate [] Object) (\ok =>
  ExactlyColors 6 {ok = ok})
badExactlySixColors Oh impossible

||| "if this creature's flying cost was paid"
||| [CR#607.2i] links a paid-cost read to an ability that OFFERS a cost,
||| and [CR#702.9a] writes flying with no parameter at all, so the word
||| names nothing that could have been paid.
public export
badPaidCostOnCostlessKeyword : Unspellable (Predicate [] Object) (\ok =>
  PaidCost (ByKeyword "Flying") Nothing {nc = ok})
badPaidCostOnCostlessKeyword Oh impossible

||| "for each time it was kickre'd"
||| The count read carries the same gate as the boolean one, and it is
||| fail-closed through the catalog: a word with no row of its own names
||| no ability [CR#702.1], so a misspelling names no cost either.
public export
badTimesPaidUnknownKeyword : Unspellable (Amount []) (\ok =>
  TimesPaid (ByKeyword "Kickre") Macros.thisCreature {nc = ok})
badTimesPaidUnknownKeyword Oh impossible


||| "Whenever you scry a card, …"
||| [CR#701.22a] scries a NUMBER of cards and carries none off, so the act has no patient the event could name.
public export
badScryPatient : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Scry"
              (Just (Macros.a (InZone (ZoneAt Library Bare)))) False {pt = ok})
badScryPatient ActOn impossible


||| "Whenever discards a card, …"
||| An act announced of no one: the active names its actor and the passive its patient, and this writes neither.
public export
badVoicelessAct : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Scry" Nothing False {vc = ok})
badVoicelessAct ActiveAct impossible
badVoicelessAct PassiveAct impossible


||| "Whenever a card in a graveyard is destroyed, …"
||| [CR#701.8a] moves a permanent from the BATTLEFIELD, and [CR#701.8b] says a permanent put into a graveyard any other way hasn't been destroyed.
public export
badDestroyInGraveyard : Unspellable (GameEvent []) (\ok =>
  VerbedEvent Nothing "Destroy"
              (Just (Macros.a (InZone Macros.graveyardZ))) False {zn = ok})
badDestroyInGraveyard Oh impossible


||| "Whenever you discard a permanent you control, …"
||| [CR#701.9a] discards a card from its owner's HAND; a permanent on the battlefield is in no one's hand to discard.
public export
badDiscardFromBattlefield : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Discard"
              (Just (Macros.a (InZone Macros.battlefieldZ))) False {zn = ok})
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


||| "Exchange life totals with target opponent" written with the opponent alone
||| [CR#701.12c] settles an exchange by having each player equal "the other player's previous life total", so the clause needs two parties; one mention names no other, and [CR#701.12a] refuses an exchange that cannot be completed in full.
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
||| [CR#109.3] makes a characteristic a property of an OBJECT and lists no life total among them, while [CR#119.1] gives the life total to each PLAYER, so one referent never has both; the comparison's axis list is scoped to one kind and a crossing list describes nothing.
public export
badMixedAxisComparison : Unspellable (Predicate [] Object) (\ok =>
  Compare [CharAxis Power, PlayerStatAxis LifeTotal] Greater (Lit 1) {at = ok})
badMixedAxisComparison (NextAxis _ (LastAxis _)) impossible


||| "{T}: … , where X is 3" printed on a card whose own mana cost is {X}.
||| [CR#107.3k] makes an activated ability's activation-cost X independent of every other X on the object, an explicit exception to [CR#107.3i], so the letter the printed cost announced [CR#107.3a] is not the ability's to read or to close.
public export
badActivatedClosesCardLetter :
  Unspellable (AbilityAt (costLetters (Just [Variable]))) (\ok =>
    Macros.activated TapSymbol (Define X (Lit 3) {ok = ok}))
badActivatedClosesCardLetter Oh impossible


||| "This creature can attack as though it were mana of any color."
||| [CR#609.4b] gives the mana matcher to spending and to nothing else -- it says what already-made mana may count as while a cost is paid -- so a premise of that sort under any other deed states a condition the rules cannot read.
public export
badManaPremiseAtAttack : Unspellable (StaticEffect []) (\ok =>
  Deontic Macros.thisCreature Permit ["Attack"] Agent NoDeonticPatient
          (Just (AsThoughMana Nothing MatchAnyColor Nothing)) {at = ok})
badManaPremiseAtAttack Oh impossible


||| "You may spend mana as though it weren't a creature."
||| [CR#609.4b] makes the spend permission's premise a statement about mana, and no description of an OBJECT is one; the premise would say the mana counts as something no cost is paid with.
public export
badObjectPremiseAtSpend : Unspellable (StaticEffect []) (\ok =>
  Deontic You Permit ["Spend"] Agent NoDeonticPatient
          (Just (AsThoughOf (Not Macros.creature))) {at = ok})
badObjectPremiseAtSpend Oh impossible


||| "Whenever you sacrifice a creature for mana, …"
||| [CR#106.12] defines "for mana" for ONE act and defines it as a second act -- activating a mana ability that includes the {T} symbol -- so the adjunct is a rule attached to tapping and not an adverb any verb may take.
public export
badForManaOnNontap : Unspellable (GameEvent []) (\ok =>
  VerbedEvent (Just You) "Sacrifice" (Just (Macros.a Macros.creature)) True
              {fm = ok})
badForManaOnNontap Oh impossible


||| "Add one mana of any type that land produced", with no production in scope.
||| [CR#106.12a] gives the trigger its mana by having a mana ability "resolve[] and produce[] mana", so outside such a header there is no production whose type the phrase could name.
public export
badProducedByEventWithoutEvent : Unspellable (ProducedMana []) (\ok =>
  ProducedByEvent (Macros.a Macros.land) {pm = ok})
badProducedByEventWithoutEvent Refl impossible


||| "You don't lose this mana as steps and phases end", with no add before it.
||| [CR#106.4] puts mana in a pool only when an effect adds it, so "this mana" with nothing added points at nothing; the sentence names no mana at all.
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
||| [CR#712.14a] states the transformed arrival for one destination -- a spell or ability that "puts a double-faced card onto the battlefield 'transformed' ... it enters the battlefield with its back face up" -- and a card in a hand has no face up at all [CR#712.14], so the rider names a way of arriving somewhere it cannot arrive.
public export
badTransformedArrivalOffField : Unspellable (Effect []) (\ok =>
  Move (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)]))
       Macros.handZ (MkMoveRiders [EntersTransformed] Nothing Nothing) {rf = ok})
badTransformedArrivalOffField Oh impossible
