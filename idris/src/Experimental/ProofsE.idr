module Experimental.ProofsE

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Instant and sorcery spells you cast have lifelink."
||| Lifelink is read when the spell DEALS DAMAGE [CR#702.15d], so the class it
||| is granted to is the one control fixes then; the corpus writes all 7
||| resolution-regime grants with "you control" and none with "you cast".
public export
badCastGrantAtResolution : Unspellable (StaticEffect []) (\ok =>
  Gains (AllOf (And [Macros.instantOrSorcery, Macros.spell, CastBy You]))
        (KeywordAbility Lifelink) {ok})
badCastGrantAtResolution MkGrantSubject impossible


||| "Artifact spells you control have convoke."
||| Convoke functions while the spell is being cast [CR#702.51a], which is the
||| moment [CR#601.2a] applies a grant to it, so the class is the casting's;
||| all 30 casting-regime grants write "you cast" and none writes "you control".
public export
badControlGrantAtCasting : Unspellable (StaticEffect []) (\ok =>
  Gains (AllOf (And [Macros.artifact, Macros.spell, ControlledBy You]))
        (KeywordAbility Convoke) {ok})
badControlGrantAtCasting MkGrantSubject impossible


||| "Creatures you control have convoke."
||| A casting-time keyword granted to a permanent does nothing: convoke
||| functions only while the spell with convoke is on the stack [CR#702.51a],
||| and no line grants one to a battlefield object.
public export
badBattlefieldConvoke : Unspellable (StaticEffect []) (\ok =>
  Gains (AllOf Macros.creatureYouControl) (KeywordAbility Convoke) {ok})
badBattlefieldConvoke MkGrantSubject impossible

||| "Enchanted creature's power and toughness are each equal to your life total."
||| A characteristic-defining ability defines the numbers of the object it is
||| printed on: [CR#604.3a] admits none that "directly affect[s] the
||| characteristics of any other objects", and English writes a line about
||| another object with the word "base" instead.
public export
badGrantedPtDefinition : Unspellable (StaticEffect []) (\ok =>
  DefinesPt (AttachHost Enchanted (TypeW Creature)) BothEach
            (PlayerStatOf LifeTotal You) {sd = ok})
badGrantedPtDefinition MkSelfDefined impossible


||| "This creature's power and toughness are each equal to 3."
||| The star in the box stands where a fixed number would be printed
||| [CR#208.2], so a definition that wrote one would print it twice; the
||| numeral and the announced X are each zero supported lines.
public export
badWrittenPtDefinition : Unspellable (StaticEffect []) (\ok =>
  DefinesPt Macros.thisCreature BothEach (Lit 3) {dv = ok})
badWrittenPtDefinition MkDefiningValue impossible


||| A creature card printing "2/2" whose own text defines its power and toughness.
||| [CR#208.2] gives such a card a star in each slot its text defines, and all
||| 228 supported cards with a defining line print one; the converse is not
||| demanded, [CR#208.2b]'s as-enters cards printing a star and defining nothing.
public export
badStarlessDefinedPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature])
       [Static (DefinesPt Macros.thisCreature BothEach
                          (CountOf Macros.creatureYouControl))]
       (Just (2, 2)) {pts = ok})
badStarlessDefinedPt MkCardPt impossible


||| "This creature's power and toughness are each equal to the number of creatures
||| you control" established as a clause by a resolving spell.
||| A characteristic-defining ability is printed on the card it affects
||| [CR#604.3a] and functions in every zone [CR#604.3], so no resolution
||| establishes one and the clause frame is not this row's.
public export
badPtDefinitionClause : Unspellable (Effect []) (\ok =>
  Continuously (DefinesPt Macros.thisCreature BothEach
                          (CountOf Macros.creatureYouControl))
               Nothing {sp = ok})
badPtDefinitionClause SpanUnstated impossible


||| "This creature's power and toughness are switched." as a printed line.
||| The switch is written as an instruction with a duration on all 25 supported
||| lines and as a standing statement zero times; [CR#613.4d] describes the
||| effect and assigns no wording.
public export
badSwitchLine : Unspellable Ability (\ok =>
  Static (SwitchesPt Macros.thisCreature) {ln = ok})
badSwitchLine MkStaticLine impossible


||| "Target creature card in a graveyard has base power and toughness 1/1."
||| Setting base power and toughness is a continuous effect on a permanent
||| [CR#613.4b], and a graveyard card is not one.
public export
badBasePtInGraveyard : Unspellable (StaticEffect []) (\ok =>
  HasBasePt (Macros.target (And [Macros.creature, InZone Macros.graveyardZ]))
            (Lit 1) (Lit 1) {zn = ok})
badBasePtInGraveyard MkZoneFits impossible


||| "the creature with the total power among creatures you control"
||| A fold in AMOUNT position may sum; a fold that PICKS A MEMBER may not,
||| because a sum names no member of anything. "with the total" is written
||| zero times in the corpus, supported and unsupported alike, and the old
||| `Semantics` module gates its own element-picking twin the same way.
public export
badSumSelection : Unspellable (Predicate [] Object) (\ok =>
  Superlative SumOf (CharAxis Power) Macros.creature {ex = ok})
badSumSelection MkIsExtremal impossible


||| "the creature"
||| The definite article demands a description that identifies its referent,
||| and this grammar's one licensor is the superlative. The corpus's bare
||| definites all READ BACK a mention already made, which is `That`'s row and
||| `It`'s, not a determiner over a description.
public export
badBareDefinite : Unspellable (Noun [] Object) (\ok =>
  Definite Macros.creature {uq = ok})
badBareDefinite MkUniquifying impossible


||| "Choose the creature with the least toughness among creatures you control."
||| A definite description names its referent, so a choice clause has nothing
||| to offer. Every selection verb in this family writes the indefinite --
||| bolster's own "Choose a creature you control with the least toughness"
||| ([CR#701.39a]) --
||| and "choose the <superlative>" is zero supported lines.
public export
badChooseDefinite : Unspellable (Effect []) (\ok =>
  Choose (Definite (And [Macros.creature,
                         Superlative MinOf (CharAxis Toughness)
                                     Macros.creatureYouControl])) {ch = ok})
badChooseDefinite MkChoosable impossible


||| "creature with power 4 or greater with the greatest power among creatures"
||| The superlative bounds the phrase's own number against a fold, so it is a
||| comparison and takes the one-bound-per-phrase discipline: no supported line
||| writes a numeric bound and a superlative in one noun phrase.
public export
badSuperlativeAndBound : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Compare Power AtLeast (Lit 4),
       Superlative MaxOf (CharAxis Power) Macros.creature] {lc = ok})
badSuperlativeAndBound MkLoneComparison impossible


||| "the creature with the highest life total among creatures you control"
||| The agreement table's crossed cell read by its SECOND consumer: a life
||| total is something each player has [CR#119.1], so no description of an
||| object is qualified by one. Zero corpus lines, as at the fold.
public export
badLifeTotalSuperlative : Unspellable (Predicate [] Object) (\ok =>
  Superlative MaxOf (PlayerStatAxis LifeTotal) Macros.creature {sc = ok})
badLifeTotalSuperlative Refl impossible


||| "When target player removes the last intervention counter from this
||| enchantment, the game is a draw."
||| The event's context accounting is its patient's, so an agent that announced
||| a referent would be dropped silently; the three agentive lines in the corpus
||| all write "you" and none announces.
public export
badAnnouncingRemovalAgent : Unspellable (GameEvent []) (\ok =>
  LastCounterRemoved Intervention Macros.thisEnchantment
                     {by = Just (Macros.target AnyPlayer)}
                     {ag = RemovalVoiced {bl = ok}})
badAnnouncingRemovalAgent MkBindingless impossible


||| "Each player gets a charge counter."
||| A charge counter is a marker placed on an OBJECT [CR#122.1], and the scope
||| table gates the two verbs so neither describes its own recipients twice.
||| The corpus writes "gets a charge counter" zero times, as it writes every
||| object kind at the player verb.
public export
badGetsChargeCounter : Unspellable (Effect []) (\ok =>
  GetsCounters You (Lit 1) Charge {sc = ok})
badGetsChargeCounter Refl impossible


||| "this instant"
||| [CR#109.2] projects a description carrying a type word and no zone word
||| onto the battlefield, and an instant is never there: [CR#110.4a] lists the
||| permanent card types and neither instant nor sorcery is among them. Both
||| words are zero supported own-line occurrences after "this", against
||| creature's 12,195; the source on the stack is bare `This`.
public export
badAscribeInstant : Unspellable (Noun [] Object) (\ok =>
  AsType Instant This {way = ok})
badAscribeInstant Refl impossible


||| "this Zombie"
||| A creature type is never a self-name. Every capitalised word following
||| "this"/"This" in supported own-line text is one of nine, all of them
||| artifact ([CR#205.3g]), enchantment ([CR#205.3h]) or battle ([CR#205.3q])
||| types; no creature type appears once. Membership in the subtype catalog is
||| not the licence, which is why the ascription reads its own table.
public export
badAscribeCreatureType : Unspellable (Noun [] Object) (\ok =>
  AsType Creature This {sub = Just Zombie} {way = ok})
badAscribeCreatureType Refl impossible


||| "this Curse"
||| A card carrying two enchantment types self-names by the one whose rules
||| the sentence reaches: [CR#303.4] gives Aura its attachment rules, and all
||| seven self-referring Aura Curses among the 42 supported ones write "this
||| Aura". The Cartouches and Runes do the same. "This Curse" is zero lines.
public export
badAscribeCurse : Unspellable (Noun [] Object) (\ok =>
  AsType Enchantment This {sub = Just Curse} {way = ok})
badAscribeCurse Refl impossible


||| "target creature that is fortified"
||| The participle catalog answers two questions and this is the second: the
||| word is attested at the HEAD reader ("fortified land", two cards, the term
||| [CR#301.6] gives the Equipment rules when they apply to lands), and the
||| presence check is written zero times — reminder text included — against 27
||| "is enchanted" and 22 "is equipped". A closure spelled as a cell.
public export
badIsFortified : Unspellable (Predicate [] Object) (\ok =>
  IsAttached Fortified {ok})
badIsFortified Refl impossible


||| "target creature that is the monarch"
||| The scope column is the mechanism's only index, and here it does its work:
||| the monarch is a designation A PLAYER can have [CR#725.1], so no
||| description of an object is qualified by it. Zero supported lines write an
||| object as the monarch, reminder text included.
public export
badObjectMonarch : Unspellable (Predicate [] Object) (\ok =>
  HasDesignation Monarch {sc = ok})
badObjectMonarch Refl impossible


||| "You become goaded."
||| The same gate at the giving row. Goaded is a designation A PERMANENT can
||| have [CR#701.15b] — "only permanents", as [CR#701.60b] says of its
||| sibling — and the corpus writes no player gaining one: "you become goaded"
||| and every variant is zero, reminder text included.
public export
badGoadedPlayer : Unspellable (Effect []) (\ok =>
  GainsDesignation You Goaded {sc = ok})
badGoadedPlayer Refl impossible


||| "goad target creature card in your graveyard"
||| The zone demand the scope carries: an object is given a designation on the
||| battlefield and nowhere else. [CR#701.15b] makes goaded a permanent's
||| designation, and no line reaches a graveyard with any of the giving verbs.
public export
badGoadInGraveyard : Unspellable (Effect []) (\ok =>
  GainsDesignation (Macros.target (And [Macros.creature,
                                        InZone (Macros.graveyardOf You)]))
                   Goaded {zn = ok})
badGoadInGraveyard (HolderOnField {ok = OnField}) impossible
