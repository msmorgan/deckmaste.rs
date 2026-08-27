||| Unspellable pins, continued from Experimental.ProofsD.
module Experimental.ProofsE

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Creatures you control have convoke."
||| Convoke functions only while the spell is on the stack [CR#702.51a], so a permanent grant does nothing.
public export
badBattlefieldConvoke : Unspellable (StaticEffect []) (\ok =>
  Gains (AllOf Macros.creatureYouControl) (KeywordAbility "Convoke" Nothing) {ok})
badBattlefieldConvoke Oh impossible

||| "Enchanted creature's power and toughness are each equal to your life total."
||| A characteristic-defining ability affects no other object's characteristics [CR#604.3a].
public export
badGrantedPtDefinition : Unspellable (StaticEffect []) (\ok =>
  DefinesPt (AttachHost Enchanted (TypeW Creature)) BothEach
            (PlayerStatOf LifeTotal You) {sd = ok})
badGrantedPtDefinition Oh impossible


||| a creature card printing "2/2" whose own text defines its power and toughness
||| [CR#208.2] gives such a card a star in each slot its text defines.
public export
badStarlessDefinedPt : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Green]) [] (MkTypeLine [] [Creature])
       [Static (DefinesPt Macros.thisCreature BothEach
                          (CountOf Macros.creatureYouControl))]
       (Just (2, 2)) {bx = ok})
badStarlessDefinedPt MkCardBox impossible


||| "This creature's power and toughness are each equal to the number of creatures you control" established as a clause by a resolving spell.
||| A characteristic-defining ability is printed on its own card [CR#604.3,604.3a], so no resolution establishes one.
public export
badPtDefinitionClause : Unspellable (Effect []) (\ok =>
  Continuously (DefinesPt Macros.thisCreature BothEach
                          (CountOf Macros.creatureYouControl))
               Nothing {cl = ok})
badPtDefinitionClause Oh impossible


||| "the creature with the total power among creatures you control"
||| A fold that picks a member may not sum, because a sum names no member of anything.
public export
badSumSelection : Unspellable (Predicate [] Object) (\ok =>
  Superlative SumOf (CharAxis Power) Macros.creature {ex = ok})
badSumSelection Oh impossible


||| "the creature"
||| The definite article demands a description that identifies its referent; the superlative is the one licensor.
public export
badBareDefinite : Unspellable (Noun [] Object) (\ok =>
  Definite Macros.creature {uq = ok})
badBareDefinite Oh impossible


||| "Choose the creature with the least toughness among creatures you control."
||| A definite description names its referent, so a choice clause has nothing to offer.
public export
badChooseDefinite : Unspellable (Effect []) (\ok =>
  Choose (Definite (And [Macros.creature,
                         Superlative MinOf (CharAxis Toughness)
                                     Macros.creatureYouControl])) Nothing {ch = ok})
badChooseDefinite BareChoice impossible


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
  LastCounterRemoved Intervention Macros.thisEnchantment (Just (Macros.target AnyPlayer))
                     {ag = AgentVoiced {bl = ok}})
badAnnouncingRemovalAgent Refl impossible


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
  AsType Instant This Nothing {way = ok})
badAscribeInstant Oh impossible


||| "this sorcery"
||| [CR#109.2] projects a type word with no zone word onto the battlefield, where a sorcery never is [CR#110.4].
public export
badAscribeSorcery : Unspellable (Noun [] Object) (\ok =>
  AsType Sorcery This Nothing {way = ok})
badAscribeSorcery Oh impossible


||| "this kindred"
||| [CR#308.1] gives every kindred card another card type, so "kindred" is never the word a card names itself by.
public export
badAscribeKindred : Unspellable (Noun [] Object) (\ok =>
  AsType Kindred This Nothing {way = ok})
badAscribeKindred Oh impossible


||| "this Aura land"
||| [CR#205.1a] sorts subtypes into one set per card type, so an enchantment type carried by a land word names nothing.
public export
badAscribeForeignSubtype : Unspellable (Noun [] Object) (\ok =>
  AsType Land This (Just (enchantmentType "Aura")) {way = ok})
badAscribeForeignSubtype Oh impossible


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
  GainsDesignation You Goaded Instructed Nothing {sc = ok})
badGoadedPlayer Refl impossible


||| "goad target creature card in your graveyard"
||| An object is given a designation on the battlefield and nowhere else [CR#701.15b].
public export
badGoadInGraveyard : Unspellable (Effect []) (\ok =>
  GainsDesignation (Macros.target (And [Macros.creature,
                                        InZone (Macros.graveyardOf You)]))
                   Goaded Instructed Nothing {zn = ok})
badGoadInGraveyard (HolderOnField {ok = OnField}) impossible


||| "Unflip target creature."
||| Flipping is one-way [CR#710.4]: once flipped, a permanent cannot become unflipped.
public export
badUnflipInstruction : Unspellable (Effect []) (\ok =>
  SetStatus Unflipped (Macros.target Macros.creature) {at = ok})
badUnflipInstruction Oh impossible


||| "Ward" printed as a bare keyword line
||| A parameterized keyword may not shed its parameter: [CR#702.21a] writes the ability as "Ward [cost]".
public export
badBareWardLine : Unspellable Ability (\ok => KeywordAbility "Ward" Nothing {pf = ok})
badBareWardLine Oh impossible


||| "Ward red"
||| The parameter's type is the keyword's: [CR#702.21a] demands a cost, not a quality.
public export
badWardQuality : Unspellable Ability (\ok =>
  KeywordAbility "Ward" (Just (ParamQuality (ColorIs Red))) {pf = ok})
badWardQuality Oh impossible


||| "Flying {2}"
||| A keyword whose row takes no parameter refuses one rather than ignoring it.
public export
badParamOnNullaryKeyword : Unspellable Ability (\ok =>
  KeywordAbility "Flying" (Just (ParamCost (Mana [Macros.generic 2]))) {pf = ok})
badParamOnNullaryKeyword Oh impossible


||| "Flyign"
||| A word outside the catalog. A SPELLING pin, not a rules one:
||| [CR#702.1] has an object list "only the name of the ability as a
||| keyword", and a word with no row of its own names no ability at all.
public export
badUnknownKeywordLabel : Unspellable Ability (\ok =>
  KeywordAbility "Flyign" Nothing {pf = ok})
badUnknownKeywordLabel Oh impossible


||| "each creature with flyign"
||| The same refusal at the predicate's own gate, which reads knownness
||| directly instead of through a parameter fit.
public export
badUnknownKeywordPredicate : Unspellable (Predicate [] Object) (\ok =>
  HasKeyword "Flyign" {kn = ok})
badUnknownKeywordPredicate Oh impossible


||| "Protection from red" printed as a line on an instant card
||| [CR#702.16b] gives protection to a permanent or player, which an instant card's line never is.
public export
badProtectionOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Instant])
       [KeywordAbility "Protection" (Just (ParamQuality (ColorIs Red)))] Nothing {tx = ok})
badProtectionOnInstant Oh impossible


||| "Protection from player"
||| [CR#702.5a] writes enchant's slot as "[object or player]" where [CR#702.16a] writes protection's as "[quality]".
public export
badProtectionFromPlayerRestriction : Unspellable Ability (\ok =>
  KeywordAbility "Protection" (Just (ParamSubject AnyPlayer)) {pf = ok})
badProtectionFromPlayerRestriction Oh impossible


||| "Equip {2}" printed as a line on a sorcery card
||| [CR#702.6a] makes equip an activated ability of Equipment cards, which a sorcery is not.
public export
badEquipOnSorcery : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 1]) [] (MkTypeLine [] [Sorcery])
       [KeywordAbility "Equip" (Just (ParamCost (Mana [Macros.generic 2])))]
       Nothing {tx = ok})
badEquipOnSorcery Oh impossible


||| "each of one or more creatures"
||| The counted untargeted group has not fixed its members, so there is nothing to distribute into.
public export
badEachOfCountedGroup : Unspellable (Noun [] Object) (\ok =>
  EachOf (CountedGroup (Macros.atLeast 1) Macros.creature) {gm = ok})
badEachOfCountedGroup Oh impossible


||| "one of one or more creatures"
||| The same cell at the partitive: the group's members are not yet fixed to pick among.
public export
badPartitiveOfCountedGroup : Unspellable (Noun [] Object) (\ok =>
  SomeOf (Macros.exactly 1) (CountedGroup (Macros.atLeast 1) Macros.creature)
         {gm = ok})
badPartitiveOfCountedGroup Oh impossible


||| "Whenever a creature attacks your opponents, …"
||| A creature attacks one defender: [CR#508.1b] announces which one each attacking creature attacks.
public export
badPluralAttackDefender : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (PlayerGroup YourOpponents) {sg = ok}))
badPluralAttackDefender Refl impossible


||| "Whenever a creature attacks another creature, …"
||| Only a player, a planeswalker, or a battle can be attacked [CR#506.3]; the slot's kind admits an object, and the rule is what closes the set.
public export
badCreatureAttackDefender : Unspellable (GameEvent []) (\ok =>
  Attacks (Macros.a Macros.creature)
          (OneDefender (Macros.a Macros.creature) {at = ok}))
badCreatureAttackDefender Oh impossible


||| "Look at the top four cards of your library. Put one of them into your hand. An opponent chooses the rest."
||| A choice needs a set to select from, and the group complement names what was left over.
public export
badAgentChooseTheRest : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.lookAt (Macros.topCards 4)
               , Move (Macros.oneOf Them) Macros.handZ (MkMoveRiders [] Nothing Nothing)
               , Choose TheRest (Just (Macros.a Opponent)) {ch = ok} ])
badAgentChooseTheRest AgentChoice impossible


||| "… if a creature was put this turn, …"
||| A placement is a move from one zone to ANOTHER [CR#400.7]; with neither end written the clause names no move.
public export
badPlacementLookback : Unspellable (Condition []) (\ok =>
  Happened Placement (Macros.a Macros.creature) ThisTurn Nothing
           {cw = LeftBare {ok = ok}})
badPlacementLookback Oh impossible


||| "Whenever a creature enters during the turn, draw a card."
||| A window with no possessor restricts nothing.
public export
badHeaderBareTurnWindow : Unspellable Ability (\ok =>
  Triggered Whenever (Enters (Macros.a Macros.creature) Nothing) [] (Just (DuringWindow Turn Nothing {hw = ok})) Nothing Nothing
            Macros.drawACard)
badHeaderBareTurnWindow Oh impossible


||| "If one or more creatures would be created under your control, …"
||| Only a token is ever created [CR#111.1]; the head word may be added to and never replaced.
public export
badNonTokenCreationSubject : Unspellable (GameEvent []) (\ok =>
  TokensCreated (CountedGroup (Macros.atLeast 1) Macros.creature) Nothing Nothing (Just You) {tk = ok})
badNonTokenCreationSubject CountedTokens impossible
badNonTokenCreationSubject OneToken impossible


||| "If a +1/+1 counter would be put on a creature you control, that many plus one +1/+1 counters are put on it instead."
||| A batch of one has no size to read back; the article has already spelled the quantity.
public export
badSingularCounterBatchSize : Unspellable (StaticEffect []) (\ok =>
  Intercepts (CounterEvent CounterPut (Just Macros.plusOnePlusOne)
                           (Macros.a Macros.creatureYouControl) OneCounter Nothing Nothing) []
             (PutCounters (Plus (ThatMuch {ok}) (Lit 1))
                          (PrintedKind Macros.plusOnePlusOne) It)
             Repeatedly Nothing)
badSingularCounterBatchSize Refl impossible


||| "If you and an effect would put one or more +1/+1 counters on a creature you control, …"
||| One clause names one doer: the causer stands where the player agent would and excludes it.
public export
badCausedCounterWithAgent : Unspellable (GameEvent []) (\ok =>
  CounterEvent CounterPut (Just Macros.plusOnePlusOne)
               (Macros.a Macros.creatureYouControl) ManyCounters (Just You) (Just AnEffect) {cz = ok})
badCausedCounterWithAgent NotCaused impossible
badCausedCounterWithAgent CausedByEffect impossible


||| "Destroy target creature. Create two of those tokens."
||| "Those tokens" points at a definition of characteristics [CR#111.3], which ordinary creatures leave none of.
public export
badAnaphoricTokenAfterNonToken : Unspellable (Effect []) (\ok =>
  Sequentially [ Macros.destroy (Macros.target Macros.creature)
               , Create You (Lit 2) (TokenAsThose {ok}) [] ])
badAnaphoricTokenAfterNonToken Refl impossible


||| "Target creature becomes a Coward until end of turn. It's still a land."
||| Removing a subtype does not affect card types [CR#205.1a], so a retention rider puts nothing back.
public export
badStillOnSubtypeSet : Unspellable (StaticEffect []) (\ok =>
  SetsType (Macros.target Macros.creature)
           (MkToken Nothing [] (MkTypeLine [creatureType "Coward"] []) [] Nothing)
           (Just Land) {ro = ok})
badStillOnSubtypeSet Oh impossible


||| "Target creature becomes an artifact until end of turn. It's still an instant."
||| [CR#205.1a] retains the instant and sorcery types with no rider at all.
public export
badStillAnInstant : Unspellable (StaticEffect []) (\ok =>
  SetsType (Macros.target Macros.creature)
           (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing)
           (Just Instant) {ro = ok})
badStillAnInstant Oh impossible


||| "Target creature gets +1/+1 and gains flying and gains trample."
||| A coordination inside a coordination spells one flat list twice.
public export
badNestedCoordination : Unspellable (StaticEffect []) (\ok =>
  AndAlso (Coord.(::) (AndAlso [ Gets (Macros.target Macros.creature)
                                      (PtUp (Lit 1)) (PtUp (Lit 1))
                               , Gains It (KeywordAbility "Flying" Nothing) ])
                      {nc = ok}
                      (Coord.(::) (Gains It (KeywordAbility "Trample" Nothing)) Coord.Nil)))
badNestedCoordination Oh impossible


||| a coordination of no statements
||| A coordination states whatever its parts state, and nothing states nothing.
public export
badEmptyCoordination : Unspellable (StaticEffect []) (\ok =>
  AndAlso [] {ne = ok})
badEmptyCoordination ItIsSucc impossible


||| "This creature gets +1/+1 and that creature has flying."
||| The demonstrative screen at the third minting site: it never picks out the sentence's own subject.
public export
badThatCreatureIsStaticSubject : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets Macros.thisCreature (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains (That (TypeW Creature) {ok = ok}) (KeywordAbility "Flying" Nothing) ]))
badThatCreatureIsStaticSubject Refl impossible


||| "Enchanted creature gets +1/+1 and they have flying."
||| An attachment host is one thing [CR#303.4d,301.5c], so the plural anaphor finds no group.
public export
badCoordinatedHostPlural : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Creature))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Gains (Them {ok = ok}) (KeywordAbility "Flying" Nothing) ]))
badCoordinatedHostPlural Refl impossible


||| "Enchanted land gets +1/+1 and can't block."
||| The host's type is threaded forward, and only a creature can attack or block [CR#506.3].
public export
badCoordinatedLandHostBlocks : Unspellable Ability (\ok =>
  Static (AndAlso [ Gets (AttachHost Enchanted (TypeW Land))
                         (PtUp (Lit 1)) (PtUp (Lit 1))
                  , Deontic It Forbid Block Agent NoDeonticPatient {dp = ok} ]))
badCoordinatedLandHostBlocks Participant impossible


||| "equipped player"
||| [CR#301.5]: an Equipment attaches to a creature and can't legally be attached to anything that isn't one.
public export
badEquippedPlayer : Unspellable (Noun [] Player) (\ok =>
  AttachHost Equipped PlayerW {ok})
badEquippedPlayer Oh impossible


||| "fortified creature"
||| [CR#301.6]: a Fortification attaches to a land and can't legally be attached to an object that isn't one.
public export
badFortifiedCreature : Unspellable (Noun [] Object) (\ok =>
  AttachHost Fortified (TypeW Creature) {ok})
badFortifiedCreature Oh impossible


||| "Target creature gains flash."
||| Flash functions where the card is played from and on the stack [CR#702.8a,113.6e], never on the battlefield.
public export
badBattlefieldFlash : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.target Macros.creature) (KeywordAbility "Flash" Nothing) {ok})
badBattlefieldFlash Oh impossible


||| "Target spell gains indestructible."
||| [CR#702.12b] is about a permanent that can't be destroyed, and a spell is not one.
public export
badSpellIndestructible : Unspellable (StaticEffect []) (\ok =>
  Gains (Macros.target Macros.spell) (KeywordAbility "Indestructible" Nothing) {ok})
badSpellIndestructible Oh impossible


||| a "Kindred Enchantment — Siege" card
||| [CR#308.2] extends kindred to creature types only, so a Siege still needs the battle type [CR#205.3c].
public export
badSiegeWithoutBattle : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [battleType "Siege"] [Kindred, Enchantment])
       [KeywordAbility "Flying" Nothing] Nothing {ln = ok})
badSiegeWithoutBattle MkCardLine impossible


||| a "Kindred — Merfolk" card naming no other card type
||| "Each kindred card has another card type" [CR#308.1].
public export
badKindredAlone : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.generic 2]) []
       (MkTypeLine [creatureType "Merfolk"] [Kindred])
       [KeywordAbility "Flying" Nothing] Nothing {ln = ok})
badKindredAlone MkCardLine impossible


||| "Enchanted planeswalker can't attack."
||| Only a creature can attack or block [CR#506.3]; the passive is the row this type opened.
public export
badPlaneswalkerAttacks : Unspellable Ability (\ok =>
  Static (Deontic (AttachHost Enchanted (TypeW Planeswalker))
                  Forbid Attack Agent NoDeonticPatient {dp = ok}))
badPlaneswalkerAttacks Participant impossible


||| "Create a 1/1 white Soldier creature token with 'Draw two cards.'"
||| A token is a permanent [CR#111.1] and a spell ability is a resolving spell's [CR#113.3a].
public export
badTokenSpellAbility : Unspellable (Effect []) (\ok =>
  Macros.create (Lit 1) (MkToken (Just (Lit 1 ** Lit 1)) [White]
                                 (MkTypeLine [creatureType "Soldier"] [Creature])
                                 [Spell Macros.drawACard] Nothing) {ta = ok})
badTokenSpellAbility Oh impossible


||| "Instant and sorcery spells you cast have '{T}: Draw a card.'"
||| [CR#113.6e] gives a granted play-modifying ability the stack alone to function in.
public export
badQuotedGrantOnSpell : Unspellable (StaticEffect []) (\ok =>
  Gains (AllOf (And [Macros.instantOrSorcery, Macros.spell, CastBy You]))
        (Activated TapSymbol Macros.drawACard Nothing Nothing Nothing) {ok})
badQuotedGrantOnSpell Oh impossible


||| "You get an emblem with 'flying'."
||| [CR#114.3] leaves an emblem no types, mana cost or color, so a keyword names a subject it lacks.
public export
badKeywordEmblem : Unspellable (Effect []) (\ok =>
  GetsEmblem You [KeywordAbility "Flying" Nothing] {ea = ok})
badKeywordEmblem Oh impossible


||| "You get an emblem."
||| An emblem is a marker representing an object with one or more abilities [CR#114.1].
public export
badEmptyEmblem : Unspellable (Effect []) (\ok =>
  GetsEmblem You [] {ea = ok})
badEmptyEmblem Oh impossible


||| "you pay [+1]"
||| The pay clause names its payer, and [CR#606.4] moves counters on the permanent instead.
public export
badPayLoyalty : Unspellable (Effect []) (\ok =>
  Pay You (LoyaltySymbol (LoyaltyUp 1)) {pb = ok})
badPayLoyalty Oh impossible


||| "[+1]: Draw a card." printed on a sorcery card
||| A loyalty cost cannot be paid off the battlefield [CR#606.3,606.4], which [CR#113.6j] requires.
public export
badLoyaltySorcery : Unspellable Card (\ok =>
  Macros.card "Impossible Loyalty Sorcery" (Just [Macros.pip Blue]) []
       (MkTypeLine [] [Sorcery])
       [Activated (LoyaltySymbol (LoyaltyUp 1)) Macros.drawACard Nothing Nothing Nothing] Nothing {tx = ok})
badLoyaltySorcery Oh impossible


||| "Add." — a production that names no mana at all.
||| A producing effect instructs a player to add that mana [CR#106.3], so there is always mana to name.
public export
badEmptyProduction : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [] {ok}) [])
badEmptyProduction Oh impossible

||| "Add {R} or ." — one alternative producing nothing beside one that does.
||| [CR#106.3]'s refusal one level in: an alternative that produces nothing names no mana.
public export
badEmptyAlternative : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [[OfColor Red], []] {ok}) [])
badEmptyAlternative Oh impossible


||| "Add {C}. Spend this mana only."
||| [CR#106.6] makes the rider a restriction on how the mana can be spent, which needs a purpose.
public export
badPurposelessSpend : Unspellable (Effect []) (\ok =>
  AddMana You (Lit 1) (Runs [[Colorless]]) [SpendOnly [] {ne = ok}])
badPurposelessSpend MkSpendPurposes impossible


||| "Create a token that's a copy of it, except it's in addition to its other types."
||| An exception that adds no word is not an exception; the tail names at least one type.
public export
badEmptyCopyTypeException : Unspellable (Effect []) (\ok =>
  Create You (Lit 1)
         (TokenCopyOf (Macros.target Macros.creature)
                      [ExceptTypes (MkTypeLine [] []) {ne = ok}]) [])
badEmptyCopyTypeException Oh impossible

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


||| "If you control a creature, create a 1/1 black Zombie creature token. Otherwise, tap it."
||| The "Otherwise" arm runs when the condition was false, so the clause it replaces never happened: the leading twin of `badOtherwiseReadsIfArm`.
public export
badOtherwiseReadsLeadingArm : Unspellable (Effect []) (\ok =>
  If (Exists Macros.creatureYouControl)
     (Macros.create (Lit 1) (Macros.creatureTok 1 1 [Black] [creatureType "Zombie"]))
     (Just (SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok})))
badOtherwiseReadsLeadingArm (Refl, _) impossible


||| "If you have less life than an opponent, you gain 6 life. That player loses 1 life."
||| A condition holds or fails; a player it named is a referent only inside the clause it governs, and the clause exports nothing.
public export
badLeadingConditionAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [If (CompareAmt (PlayerStatOf LifeTotal You) Less
                               (PlayerStatOf LifeTotal Macros.anOpponent))
                   (Macros.gainsLife You (Lit 6))
                   Nothing,
                Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badLeadingConditionAntecedent Refl impossible

