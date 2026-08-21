||| Unspellable pins, continued from Experimental.ProofsE.
module Experimental.ProofsF

import Experimental
import Experimental.Macros
import Experimental.Unspellable

%default total

%unbound_implicits off


||| "Prevent all damage that would be dealt to this this turn."
||| Bare "This" is the source as an object, standing in no zone and projecting no head type [CR#120.1a].
public export
badPreventedBareThis : Unspellable (StaticEffect []) (\ok =>
  Prevents AnyDamage AllOfIt (ToRecipient This {rk = ok}) Nothing Nothing)
badPreventedBareThis ObjectTakes impossible


||| "Prevent all damage that would be dealt to target artifact this turn."
||| [CR#120.1a] at the shield: damage reaches only a battle, a creature or a planeswalker.
public export
badPreventDealtToArtifact : Unspellable (StaticEffect []) (\ok =>
  Prevents AnyDamage AllOfIt
           (ToRecipient (Macros.target Macros.artifact) {rk = ok}) Nothing Nothing)
badPreventDealtToArtifact ObjectTakes impossible


||| "If creatures would deal damage to you, prevent that damage."
||| The event clause names one source: [CR#609.7] makes it the single object that dealt the damage.
public export
badPreventFromPluralSource : Unspellable (StaticEffect []) (\ok =>
  PreventsFrom AnyDamage (DealtBy (AllOf Macros.creature) {ds = ok})
               (Macros.shieldingIt You) CutAll Repeatedly Nothing)
badPreventFromPluralSource MkDamageSource impossible


||| "The next time creatures would deal damage to you this turn, that damage is dealt to this creature instead."
||| The same gate at the second event-shaped row: a source is the one object that dealt the damage [CR#120.7].
public export
badRedirectFromPluralSource : Unspellable (StaticEffect []) (\ok =>
  RedirectsFrom AnyDamage (DealtBy (AllOf Macros.creature) {ds = ok})
                (Macros.shieldingIt You) Macros.thisCreature NextTimeOnly)
badRedirectFromPluralSource MkDamageSource impossible


||| "All damage that would be dealt to you is dealt to target artifact instead."
||| [CR#120.1a] at the destination, which [CR#614.9] names from the same closed list.
public export
badRedirectToArtifact : Unspellable (StaticEffect []) (\ok =>
  Redirects AnyDamage AllOfIt (Macros.shieldingIt You) Nothing
            (Macros.target Macros.artifact) {rk = ok})
badRedirectToArtifact ObjectTakes impossible


||| "All damage that would be dealt to you is dealt to creatures you control instead."
||| [CR#614.9] replaces damage dealt to one thing with the same damage dealt to another: one head per arrow.
public export
badRedirectToPlural : Unspellable (StaticEffect []) (\ok =>
  Redirects AnyDamage AllOfIt (Macros.shieldingIt You) Nothing
            (AllOf Macros.creatureYouControl) {one = ok})
badRedirectToPlural MkSingleRecipient impossible


||| "Deal 3 damage to target creature. You gain life equal to the damage prevented this way."
||| The phrase names a prevention, and a damage outcome is not one.
public export
badPreventedThisWayAfterDamage : Unspellable (Effect []) (\ok =>
  Sequentially [ DealDamage This (Lit 3) (Macros.target Macros.creature)
               , ChangeLife You (Up (PreventedThisWay {ok})) ])
badPreventedThisWayAfterDamage Refl impossible


||| "You gain life equal to the damage prevented this way."
||| [CR#615.5] makes the additional effect part of the prevention, so the announcement never leaves the rider.
public export
badPreventedThisWayUnannounced : Unspellable (Effect []) (\ok =>
  ChangeLife You (Up (PreventedThisWay {ok})))
badPreventedThisWayUnannounced Refl impossible


||| "Prevent the next [the damage prevented this way] damage that would be dealt to you."
||| A shield cannot be sized by what it prevents: [CR#615.7]'s cap is spent one damage at a time.
public export
badShieldSizedByItsOwnPrevention : Unspellable (StaticEffect []) (\ok =>
  Prevents AnyDamage (TheNext (PreventedThisWay {ok}))
           (Macros.shieldingIt You) Nothing Nothing)
badShieldSizedByItsOwnPrevention Refl impossible


||| "Destroy target source."
||| [CR#120.7] makes a source a position in an event, not an object standing in a zone.
public export
badDestroySource : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target Macros.source) {ok})
badDestroySource OnField impossible


||| "This creature deals 3 damage to target source."
||| A source is not a recipient: [CR#120.7] names what dealt damage, [CR#120.1a] what may be dealt it.
public export
badDamageToSource : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 3) (Macros.target Macros.source) {rk = ok})
badDamageToSource ObjectTakes impossible


||| "…target nonsource…"
||| [CR#120.7] makes a source a position an object occupies, not a property it has or lacks.
public export
badNonsource : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.source {ng = ok})
badNonsource MkNegatable impossible


||| "Tap target permanent or player."
||| [CR#115.2] licenses the player as an exception, not a thing in a zone, so the union head is placeless.
public export
badTapKindJoin : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.target (KindJoin JoinAnyPlayer JoinPermanent)) {ok})
badTapKindJoin OnField impossible


||| "Exile target permanent or player."
||| Exile demands no zone, so what refuses is that the mention may denote a player [CR#115.2].
public export
badExileKindJoin : Unspellable (Effect []) (\ok =>
  Macros.exile (Macros.target (KindJoin JoinAnyPlayer JoinPermanent)) {na = ok})
badExileKindJoin MkNotPlayerSpanning impossible


||| "…target creature or permanent or player…"
||| The union head is already two nouns joined by "or", so coordinating it writes the disjunction twice.
public export
badKindJoinInOr : Unspellable (Predicate [] Object) (\ok =>
  Or [KindJoin JoinAnyPlayer JoinPlaneswalker, Macros.creature] {cd = ok})
badKindJoinInOr MkCoordinableDisjuncts impossible


||| "Tap you and permanents you control."
||| A set with a player in it stands nowhere, so the battlefield verbs refuse it [CR#115.2].
public export
badTapYouAnd : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (YouAnd (AllOf (And [Permanent, ControlledBy You]))) {ok})
badTapYouAnd OnField impossible


||| "…you and you and permanents you control…"
||| The row spells its own player half [CR#109.5], so nesting spells the second person twice.
public export
badNestedYouAnd : Unspellable (Noun [] Object) (\ok =>
  YouAnd (YouAnd (AllOf (And [Permanent, ControlledBy You]))) {nn = ok})
badNestedYouAnd MkNotMixedGroup impossible


||| "All damage that would be dealt to you is dealt to you and permanents you control instead."
||| [CR#614.9] puts one thing at each end of the arrow, and the mixed group is plural by construction.
public export
badRedirectToGroup : Unspellable (StaticEffect []) (\ok =>
  Redirects AnyDamage AllOfIt (Macros.shieldingIt You) Nothing
            (YouAnd (AllOf (And [Permanent, ControlledBy You]))) {one = ok})
badRedirectToGroup MkSingleRecipient impossible


||| "If creatures would deal damage to a permanent or player, they deal double that damage instead."
||| [CR#120.7] makes the source one object, and the body reads the agent back with a singular pronoun.
public export
badScaleFromPluralSource : Unspellable (StaticEffect []) (\ok =>
  Scales AnyDamage (AllOf Macros.creature)
         (Macros.shieldingIt (Macros.a (KindJoin JoinAnyPlayer JoinPermanent)))
         (Multiplied Doubled) Repeatedly {ds = ok})
badScaleFromPluralSource MkDamageSource impossible


||| "… it deals that much damage plus 0 instead."
||| A written zero is not a count, where [CR#107.1b]'s computed one is ordinary.
public export
badScaleShiftByZero : Unspellable (StaticEffect []) (\ok =>
  Scales AnyDamage (Macros.a Macros.source)
         (Macros.shieldingIt (Macros.a (KindJoin JoinAnyPlayer JoinPermanent)))
         (Shifted ShiftUp (Lit 0) {wc = ok}) Repeatedly)
badScaleShiftByZero MkWrittenCount impossible


||| "… it deals that much damage plus that much instead."
||| This row announces no outcome, so the anaphor finds nothing in scope [CR#614.6].
public export
badScaleShiftByThatMuch : Unspellable (StaticEffect []) (\ok =>
  Scales AnyDamage (Macros.a Macros.source)
         (Macros.shieldingIt (Macros.a (KindJoin JoinAnyPlayer JoinPermanent)))
         (Shifted ShiftUp (ThatMuch {ok})) Repeatedly)
badScaleShiftByThatMuch Refl impossible


||| "If a source would deal damage to target artifact, it deals double that damage to that artifact instead."
||| [CR#120.1a] at a fourth position: the scaling row reuses the recipient table whole.
public export
badScaleToArtifact : Unspellable (StaticEffect []) (\ok =>
  Scales AnyDamage (Macros.a Macros.source)
         (ToRecipient (Macros.target Macros.artifact) {rk = ok})
         (Multiplied Doubled) Repeatedly)
badScaleToArtifact ObjectTakes impossible


||| "Whenever this creature is dealt damage, it deals the damage prevented this way to any target."
||| The phrase names a prevention, and no prevention happened at a damage event either.
public export
badPreventedThisWayAfterDamageEvent : Unspellable Ability (\ok =>
  Triggered Whenever (IsDealtDamage Macros.thisCreature)
            (DealDamage It (PreventedThisWay {ok = ok}) (Macros.target AnyTarget)))
badPreventedThisWayAfterDamageEvent Refl impossible


||| "Whenever a creature dies, this creature deals that much damage to any target."
||| A death announces no magnitude, so the anaphor has nothing to point at.
public export
badThatMuchAfterDeath : Unspellable Ability (\ok =>
  Triggered Whenever (Dies (Macros.a Macros.creature))
            (DealDamage Macros.thisCreature (ThatMuch {ok = ok})
                        (Macros.target AnyTarget)))
badThatMuchAfterDeath Refl impossible


||| "Whenever this creature is dealt damage, that creature deals that much damage to any target."
||| The demonstrative screen at a fourth minting site: it never picks out the speaker.
public export
badThatCreatureIsDamagedSelf : Unspellable Ability (\ok =>
  Triggered Whenever (IsDealtDamage Macros.thisCreature)
            (DealDamage (That (TypeW Creature) {ok = ok}) ThatMuch
                        (Macros.target AnyTarget)))
badThatCreatureIsDamagedSelf Refl impossible


||| "Target player can't gain life." — as a permanent's printed line.
||| A static ability does not target [CR#115.1a..115.1e].
public export
badStaticPlayerCantTargets : Unspellable Ability (\ok =>
  Static (PlayerCant GainsLife (Macros.target AnyPlayer)) {ut = ok})
badStaticPlayerCantTargets MkUntargeting impossible


||| "Players can't gain life." — as a spell's clause, with no duration.
||| A resolution that establishes a prohibition states how long it lasts; one that does not is a printed line.
public export
badSpanlessPlayerCant : Unspellable (Effect []) (\ok =>
  Continuously (PlayerCant GainsLife (PlayerGroup AllPlayers)) Nothing {sp = ok})
badSpanlessPlayerCant SpanUnstated impossible


||| "Players can't gain life until end of turn."
||| A prohibition writes "this turn" where a grant writes "until end of turn".
public export
badPlayerCantUntilEndOfTurn : Unspellable (Effect []) (\ok =>
  Continuously (PlayerCant GainsLife (PlayerGroup AllPlayers))
               (Just Macros.untilEndOfTurn) {sp = ok})
badPlayerCantUntilEndOfTurn SpanStated impossible


||| "Target creature becomes the basic land type of your choice until end of turn."
||| [CR#305.7] states its consequences of a land's subtype, and the subject's projected head is no land.
public export
badChosenBasicTypeOnCreature : Unspellable (Effect []) (\ok =>
  Continuously (SetsChosenBasicType (Macros.target Macros.creature) {ls = ok})
               (Just Macros.untilEndOfTurn))
badChosenBasicTypeOnCreature MkLandSubject impossible


||| "Target land card in your graveyard becomes the basic land type of your choice until end of turn."
||| A land type is what a land has on the battlefield [CR#305.6].
public export
badChosenBasicTypeInGraveyard : Unspellable (Effect []) (\ok =>
  Continuously (SetsChosenBasicType
                  (Macros.target (And [Macros.land,
                                       InZone (Macros.graveyardOf You)]))
                  {zn = ok})
               (Just Macros.untilEndOfTurn))
badChosenBasicTypeInGraveyard Oh impossible


||| "Creatures are Mountains."
||| A subtype-only line carries no card type, and [CR#205.3i] puts Mountain in the land set.
public export
badCreaturesAreMountains : Unspellable (StaticEffect []) (\ok =>
  SetsType (AllOf Macros.creature)
           (MkToken Nothing [] (Macros.basicLandLine [Mountain]) [] Nothing)
           Nothing {af = ok})
badCreaturesAreMountains Oh impossible


||| "of the card name of your choice"
||| A name is chosen and then matched by name equality, never read as a quality of a description.
public export
badYourChoiceCardName : Unspellable (Predicate [] Object) (\ok =>
  OfYourChoice CardName {read = ok})
badYourChoiceCardName Oh impossible


||| "of the number of your choice"
||| A bound number reads back by numeric equality, never as a quality.
public export
badYourChoiceNumber : Unspellable (Predicate [] Object) (\ok =>
  OfYourChoice Number {read = ok})
badYourChoiceNumber Oh impossible


||| "Destroy one of the creature type of your choice."
||| The choice determiner is a modifier and not a head: it says which ones, never what they are.
public export
badLoneYourChoice : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.a (OfYourChoice CreatureType) {hd = ok}))
badLoneYourChoice MkHeaded impossible


||| "Creatures you control of the chosen type get +1/+1. / As this enchantment enters, choose a creature type."
||| A line reads what its predecessors said; [CR#607.2d] links the reader to a choice already made.
public export
badReaderBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (Gets (AllOf (And [Macros.creature,
                                   OfChosen CreatureType {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1)))
       , Static (EntersChoice Macros.thisEnchantment CreatureType) ]
       Nothing)
badReaderBeforeChooser Refl impossible


||| "As this enchantment enters, choose a creature type. / As this enchantment enters, choose a creature type. / Creatures you control of the chosen type get +1/+1."
||| A second choice of the same sort makes the read ambiguous [CR#607.4].
public export
badTwoChoosersOneSortRead : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (Gets (AllOf (And [Macros.creature,
                                   OfChosen CreatureType {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing)
badTwoChoosersOneSortRead Refl impossible


||| "As this enchantment enters, choose a color. / Creatures you control of the chosen type get +1/+1."
||| The binding is sorted: a choice of one quality does not answer a read of another [CR#607.2d].
public export
badChosenReadWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment Color)
       , Static (Gets (AllOf (And [Macros.creature,
                                   OfChosen CreatureType {ok = ok}]))
                      (PtUp (Lit 1)) (PtUp (Lit 1))) ]
       Nothing)
badChosenReadWrongSort Refl impossible


||| "This creature has protection from the chosen color. / As this creature enters, choose a color."
||| The keyword parameter reads the card's discourse, so it reads nothing a later line chose [CR#702.16a].
public export
badChosenProtectionBeforeChoice : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [Angel] [Creature])
       [ Static (Gains Macros.thisCreature
                       (KeywordAbility Protection
                          {param = Just (ParamQuality (OfChosen Color {ok = ok}))}))
       , Static (EntersChoice Macros.thisCreature Color) ]
       (Just (2, 2)))
badChosenProtectionBeforeChoice Refl impossible


||| "Creatures you control are the chosen type in addition to their other types. / As this enchantment enters, choose a creature type."
||| The same demand at the ascribed quality: the ordering rule is the container's [CR#607.2d].
public export
badAscribedQualityBeforeChoice : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (AddsChosenQuality
                   (AllOf (And [Macros.creature, ControlledBy You]))
                   (OfChosen CreatureType {ok = ok}))
       , Static (EntersChoice Macros.thisEnchantment CreatureType) ]
       Nothing)
badAscribedQualityBeforeChoice Refl impossible


||| "Creatures you control get +1/+1. The same is true for creature spells you control and creature cards you own that aren't on the battlefield."
||| The extension is an ascription's rider and nothing else's.
public export
badExtendedPump : Unspellable (StaticEffect []) (\ok =>
  AlsoOffBattlefield
    (Gets (AllOf (And [Macros.creature, ControlledBy You]))
          (PtUp (Lit 1)) (PtUp (Lit 1))) {ex = ok})
badExtendedPump MkExtendableScope impossible


||| "Creatures you control are artifacts in addition to their other types. The same is true for … . The same is true for … ."
||| One extension per statement: the singleton discipline at a third site.
public export
badDoubleExtension : Unspellable (StaticEffect []) (\ok =>
  AlsoOffBattlefield
    (AlsoOffBattlefield
       (BecomesAlso (AllOf (And [Macros.creature, ControlledBy You]))
                    (MkToken Nothing [] (MkTypeLine [] [Artifact]) [] Nothing))) {ex = ok})
badDoubleExtension MkExtendableScope impossible


||| "{U}: Counter target spell with the chosen name. / As this enchantment enters, choose a card name."
||| [CR#607.2d] links the reader to the choice the first ability made, which a later line has not made.
public export
badNameMatchBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Activated (Mana [Macros.pip Blue])
                   (Macros.counterSpell
                      (Macros.target (And [Macros.spell,
                                           Named (ChosenName {ok = ok})])))
       , Static (EntersChoice Macros.thisEnchantment CardName) ]
       Nothing)
badNameMatchBeforeChooser Refl impossible


||| "As this enchantment enters, choose a color. / {U}: Counter target spell with the chosen name."
||| [CR#607.2d]'s linked ability refers only to the first ability's choice, and a colour chooser made no name.
public export
badNameMatchWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment Color)
       , Activated (Mana [Macros.pip Blue])
                   (Macros.counterSpell
                      (Macros.target (And [Macros.spell,
                                           Named (ChosenName {ok = ok})]))) ]
       Nothing)
badNameMatchWrongSort Refl impossible


||| "Creatures you control are the chosen name."
||| A name is matched and never ascribed: the ascription rows take a quality read, which a match is not.
public export
badAscribedName :
  Unspellable
    (StaticEffect [MkBinding AD (Quality CardName) OneOf QualityP])
    (\ok => SetsChosenQuality (AllOf (And [Macros.creature, ControlledBy You]))
                              (Named ChosenName) {qr = ok})
badAscribedName MkQualityRead impossible


||| "Choose a creature type other than Equipment."
||| [CR#205.3m] is the creature-type list, and the subtype catalog is wider than the sort.
public export
badNonCreatureTypeExclusion : Unspellable (ChoiceDomain CreatureType) (\ok =>
  TypeOtherThan Equipment {ct = ok})
badNonCreatureTypeExclusion Refl impossible


||| "… all creatures with the same name as all creatures."
||| [CR#201.2a] compares an object to an object, and a group has no one name to be measured against.
public export
badGroupNameRelatum : Unspellable (Predicate [] Object) (\ok =>
  Named (SameNameAs (AllOf Macros.creature) {one = ok}))
badGroupNameRelatum Refl impossible


||| "Creature cards in graveyards can't be countered."
||| Countering removes a spell or ability from the stack [CR#701.6a]; a graveyard card is past that.
public export
badCounteredInGraveyard : Unspellable (StaticEffect []) (\ok =>
  ObjectCant Countered (AllOf (And [Macros.creature, InZone Macros.graveyardZ]))
    {sub = CounteredOnStack {zn = ok}})
badCounteredInGraveyard Oh impossible


||| "Creatures can't be played."
||| A spell is cast and a land is played [CR#305.1,601.1a].
public export
badPlayedNonland : Unspellable (StaticEffect []) (\ok =>
  ObjectCant Played (AllOf Macros.creature) {sub = PlayedIsLand {ld = ok}})
badPlayedNonland MkLandSubject impossible


||| an instant printing "Spells you control can't be countered" as a standing line
||| [CR#113.6g] licenses a standing line about the object itself, not one describing a class.
public export
badStaticClassCounterOnInstant : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip Blue]) [] (MkTypeLine [] [Instant])
       [ Static (ObjectCant Countered
                   (AllOf (And [Macros.spell, ControlledBy You]))) ]
       Nothing {tx = ok})
badStaticClassCounterOnInstant MkCardText impossible


||| a plain enchantment printing "I — Draw a card."
||| [CR#714.1] makes the striated text box with chapter symbols part of the Saga frame.
public export
badChapterOnNonSaga : Unspellable Card (\ok =>
  Macros.card "" (Just [Macros.pip White]) [] (MkTypeLine [] [Enchantment])
       [ Triggered When (ChapterMark [ChapterI]) Macros.drawACard ]
       Nothing {ch = ok})
badChapterOnNonSaga MkCardChapters impossible


||| "— Draw a card." — a chapter line with no numeral before the dash.
||| A chapter symbol is its numeral [CR#107.15], so a marker with none is no marker.
public export
badEmptyChapterMark : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [] {cm = ok}) Macros.drawACard)
badEmptyChapterMark Oh impossible


||| "III, II — Draw a card."
||| The comma-joined marker ascends, [CR#107.15b] expanding it into one ability per numeral.
public export
badDescendingChapterMark : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterIII, ChapterII] {cm = ok}) Macros.drawACard)
badDescendingChapterMark Oh impossible


||| "II, II — Draw a card."
||| [CR#107.15b] expands the joined marker into one ability per numeral, so a repeat prints one twice.
public export
badRepeatedChapterMark : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterII, ChapterII] {cm = ok}) Macros.drawACard)
badRepeatedChapterMark Oh impossible


||| "Whenever I —, draw a card."
||| [CR#714.2b] supplies the whole header, so a chapter line prints no trigger word.
public export
badChapterWhenever : Unspellable Ability (\ok =>
  Triggered Whenever (ChapterMark [ChapterI]) Macros.drawACard {wo = ok})
badChapterWhenever MkTriggerWordOk impossible


||| "I — Draw a card. This ability triggers only once each turn."
||| [CR#714.2b] writes the whole header, so the printed line has nowhere for a rider.
public export
badChapterLimit : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI]) {limit = Just OncePerTurn}
    Macros.drawACard {cd = ok})
badChapterLimit MkChapterDefaults impossible


||| "I — , if you control a creature, draw a card."
||| [CR#714.2b]'s expansion already carries the intervening "if" over the lore tally.
public export
badChapterIntervening : Unspellable Ability (\ok =>
  Triggered When (ChapterMark [ChapterI])
    {intervening = Just (Exists (And [Macros.creature, ControlledBy You]))}
    Macros.drawACard {cd = ok})
badChapterIntervening MkChapterDefaults impossible


||| "If I — would happen, draw a card instead."
||| [CR#714.2b] expands the marker into a counter placement, which is the event a replacement names.
public export
badChapterReplacement : Dependent.Unspellable (StaticEffect []) (\x, y =>
  Intercepts (ChapterMark [ChapterI]) Macros.drawACard Repeatedly
    {ok = x} {uo = y})
badChapterReplacement (MkInterceptable ** _) impossible


||| "You may cast this card from your graveyard as though it had flash."
||| A permission widens where a card may be cast from or when, never both in one clause.
public export
badZonedFlashPermission : Unspellable (StaticEffect []) (\ok =>
  MayPlay You This {verb = Cast} {from = Just (Macros.graveyardOf You)}
          {asThough = Just HadFlash} {pz = ok})
badZonedFlashPermission MkPlaySource impossible


||| "You may cast creatures you control as though they had flash."
||| A permission's complement is named by what the object will become [CR#701.5b], not by a battlefield word.
public export
badFlashPermissionOnPermanent : Unspellable (StaticEffect []) (\ok =>
  MayPlay You (AllOf Macros.creatureYouControl)
          {verb = Cast} {asThough = Just HadFlash} {pz = ok})
badFlashPermissionOnPermanent MkPlaySource impossible


||| "You may look at your hand any time."
||| [CR#402.3] already lets a player look at their own hand at any time.
public export
badLookAtHandRider : Unspellable (StaticEffect []) (\ok =>
  Visibility LookAt You WholeHand {vo = ok})
badLookAtHandRider Oh impossible


||| "Play with the top card of your library revealed." as a resolving spell's clause with no duration
||| A clause that states no duration lasts until the end of the game [CR#611.2a].
public export
badSpanlessVisibility : Unspellable (Effect []) (\ok =>
  Continuously (Visibility Reveal You TopOfLibrary) Nothing {sp = ok})
badSpanlessVisibility (SpanUnstated) impossible


||| "Until your next turn, you may look at the top card of your library any time."
||| The rider writes three adverbials and this is not one of them; the play permission's table is wider.
public export
badVisibilityUntilYourNextTurn : Unspellable (Effect []) (\ok =>
  Continuously (Visibility LookAt You TopOfLibrary)
               (Just (Until (StartOf Turn (Just Yours)))) {sp = ok})
badVisibilityUntilYourNextTurn (SpanStated) impossible


||| "You may play any number of additional lands on each of your turns."
||| [CR#305.2] lets a continuous effect increase the number, and an unbounded top removes the limit instead.
public export
badAnyNumberOfAdditionalLands : Unspellable (StaticEffect []) (\ok =>
  MayPlayAdditionalLands You Macros.anyNumber {bi = ok})
badAnyNumberOfAdditionalLands Oh impossible


||| "You may play an additional land." as a resolving spell's clause with no duration
||| A clause that states no duration lasts until the end of the game [CR#611.2a].
public export
badSpanlessLandAllowance : Unspellable (Effect []) (\ok =>
  Continuously (MayPlayAdditionalLands You (Macros.exactly 1)) Nothing {sp = ok})
badSpanlessLandAllowance (SpanUnstated) impossible


||| "Until end of turn, you may play an additional land."
||| [CR#305.2a] counts the allowance per turn, so the turn is the only unit the family names.
public export
badLandAllowanceUntilEndOfTurn : Unspellable (Effect []) (\ok =>
  Continuously (MayPlayAdditionalLands You (Macros.exactly 1))
               (Just Macros.untilEndOfTurn) {sp = ok})
badLandAllowanceUntilEndOfTurn (SpanStated) impossible


||| "Lands you control are every creature type."
||| [CR#205.3m] gives the creature types to creatures and kindreds, so the space demands its own host.
public export
badEveryCreatureTypeOnLand : Unspellable (StaticEffect []) (\ok =>
  AddsEveryType (AllOf (And [Macros.land, ControlledBy You])) CreatureSpace
                {sh = ok})
badEveryCreatureTypeOnLand Oh impossible


||| "Creatures you control are every basic land type."
||| The other direction of the same host gate: nothing here adds the land card type for the subtype to sit on.
public export
badEveryBasicLandTypeOnCreature : Unspellable (StaticEffect []) (\ok =>
  AddsEveryType (AllOf (And [Macros.creature, ControlledBy You])) BasicLandSpace
                {sh = ok})
badEveryBasicLandTypeOnCreature Oh impossible


||| "Kindred permanents you control are every creature type."
||| The space carries one host demand, and a kindred subject is not the creature host it names.
public export
badEveryCreatureTypeOnKindred : Unspellable (StaticEffect []) (\ok =>
  AddsEveryType (AllOf (HasType Kindred)) CreatureSpace {sh = ok})
badEveryCreatureTypeOnKindred Oh impossible


||| "You skip your next upkeep step."
||| The upkeep step is skipped by a permanent's standing line and by no scheduling effect.
public export
badSkipNextUpkeepStep : Unspellable (Effect []) (\ok =>
  SkipsNext You Upkeep 1 {sk = ok})
badSkipNextUpkeepStep MkScheduledSkip impossible


||| "Players skip their turns."
||| A standing turn skip is an ending, not a tax: [CR#500.11] proceeds as though the turn did not exist.
public export
badStandingSkipTurn : Unspellable (StaticEffect []) (\ok =>
  Skips (PlayerGroup AllPlayers) Turn {sk = ok})
badStandingSkipTurn MkStandingSkip impossible


||| "You skip your next end step."
||| The skip reader's row is empty at the end step, where the span and header readers are not.
public export
badSkipEndStep : Unspellable (Effect []) (\ok =>
  SkipsNext You EndStep 1 {sk = ok})
badSkipEndStep MkScheduledSkip impossible


||| "Take three extra turns after this one."
||| The count vocabulary is closed at one and two.
public export
badThreeExtraTurns : Unspellable (Effect []) (\ok =>
  ExtraTurn You 3 {ct = ok})
badThreeExtraTurns OneExtraTurn impossible


||| "Skip your draw step." written as a RESOLVING clause.
||| The standing skip is a printed line and never a resolving clause; the resolving form says "next".
public export
badSkipClause : Unspellable (Effect []) (\ok =>
  Continuously (Skips You DrawStep) Nothing {sp = ok})
badSkipClause SpanUnstated impossible


||| "Until end of turn, you skip your draw step."
||| One span slot: the skip's own table writes no endpoint, though [CR#614.10]'s replacement class does.
public export
badSkipUntilEndOfTurn : Unspellable (Effect []) (\ok =>
  Continuously (Skips You DrawStep) (Just (Until (EndOf Turn Nothing))) {sp = ok})
badSkipUntilEndOfTurn SpanStated impossible


||| "Draw a card. At the beginning of that turn's end step, you lose the game."
||| The possessive reaches a mention, so a clause that made no turn leaves nothing for it to reach.
public export
badDeicticTurnWithoutIntroducer : Unspellable (Effect []) (\ok =>
  Sequentially [Draw You (Lit 1),
                Delayed (BeginningOf EndStep (Just ThatTurns) {td = ok})
                        (Concludes LoseGame You)])
badDeicticTurnWithoutIntroducer (NoTurnDeixis) impossible
badDeicticTurnWithoutIntroducer (TurnInScope) impossible


||| "Take an extra turn after this one. At the beginning of that turn's upkeep, draw a card."
||| The deictic possessor has one cell and it is the end step.
public export
badThatTurnsUpkeep : Unspellable (Effect []) (\ok =>
  Sequentially [ExtraTurn You 1,
                Delayed (BeginningOf Upkeep (Just ThatTurns) {pu = ok})
                        Macros.drawACard])
badThatTurnsUpkeep MkPartTriggerable impossible


||| "After this main phase, there is an additional main phase."
||| The main phase is written into the addition as the follower, never as the part added first.
public export
badAddedMainPhase : Unspellable (Effect []) (\ok =>
  AdditionalPart MainPhase (Just MainPhase) 1 {ad = ok})
badAddedMainPhase MkAddedPart impossible


||| "After this main phase, there is an additional combat phase followed by an additional combat phase."
||| The combat phase follows nothing; two of them are written as a count and not a sequence.
public export
badCombatFollows : Unspellable (Effect []) (\ok =>
  AdditionalPart Combat (Just MainPhase) 1 {followedBy = Just Combat} {fb = ok})
badCombatFollows NoFollower impossible
badCombatFollows MkFollowerPart impossible


||| "After this main phase, there are three additional combat phases."
||| The count vocabulary is closed at one and two.
public export
badThreeAdditionalPhases : Unspellable (Effect []) (\ok =>
  AdditionalPart Combat (Just MainPhase) 3 {ct = ok})
badThreeAdditionalPhases OnePhase impossible


||| "After this end step, there is an additional combat phase."
||| The anchor names only the main phase and the combat phase; the end step anchors nothing.
public export
badEndStepAnchor : Unspellable (Effect []) (\ok =>
  AdditionalPart Combat (Just EndStep) 1 {an = ok})
badEndStepAnchor BareAnchor impossible
badEndStepAnchor MkAnchorPart impossible


||| "This creature enters with 0 +1/+1 counters on it."
||| A written zero is not a count; where none is possible the line writes a variable instead.
public export
badEntersZeroCounters : Unspellable (StaticEffect []) (\ok =>
  EntersWithCounters Macros.thisCreature (Lit 0) Macros.plusOnePlusOne {wc = ok})
badEntersZeroCounters MkWrittenCount impossible


||| "A creature in your graveyard enters with an additional +1/+1 counter on it."
||| [CR#614.1c] replaces an event ending on the battlefield, so the subject is described there.
public export
badEntersCountersInGraveyard : Unspellable (StaticEffect []) (\ok =>
  EntersWithCounters (Macros.a (And [Macros.creature, InZone Macros.graveyardZ]))
                     (Lit 1) Macros.plusOnePlusOne {mark = Additional} {zn = ok})
badEntersCountersInGraveyard Oh impossible


||| "Each creature you control enters with an additional +1/+1 counter on it." written as a RESOLVING clause.
||| The entry-counter line is a printed static ability and never a resolving clause [CR#603.6d].
public export
badEntersCountersClause : Unspellable (Effect []) (\ok =>
  Continuously (EntersWithCounters
                  (Each (And [Macros.creature, ControlledBy You]))
                  (Lit 1) Macros.plusOnePlusOne {mark = Additional}) Nothing {sp = ok})
badEntersCountersClause SpanUnstated impossible


||| "Spells with the chosen name can't be activated."
||| An activated ability is the only kind that can be activated [CR#602.1c], so the noun is ability-kinded.
public export
badActivatedSpellClass : Unspellable
  (StaticEffect [MkBinding AD (Quality CardName) OneOf QualityP]) (\ok =>
  ObjectCant Activated (AllOf (And [Macros.spell, Named ChosenName])) {sub = ok})
badActivatedSpellClass CounteredOnStack impossible
badActivatedSpellClass CastOnStack impossible
badActivatedSpellClass CopiedOnStack impossible
badActivatedSpellClass PlayedIsLand impossible
badActivatedSpellClass ActivatedIsAbility impossible


||| "Activated abilities of artifacts can't be cast."
||| Casting is of a card as a spell [CR#601.1a], and an unactivated ability is no object at all [CR#109.1].
public export
badCastAbilityClass : Unspellable (StaticEffect []) (\ok =>
  ObjectCant Cast
    (AllOf (And [AbilityHead AnyActivated, AbilityOf (AllOf Macros.artifact)]))
    {sub = ok})
badCastAbilityClass CounteredOnStack impossible
badCastAbilityClass CastOnStack impossible
badCastAbilityClass CopiedOnStack impossible
badCastAbilityClass PlayedIsLand impossible
badCastAbilityClass ActivatedIsAbility impossible


||| "Mana abilities can't be activated."
||| Mana-ability-ness is predicative and heads nothing.
public export
badManaAbilityClassSubject : Unspellable (StaticEffect []) (\ok =>
  ObjectCant Activated (AllOf IsManaAbility {hd = ok}))
badManaAbilityClassSubject MkHeaded impossible


||| "Activated abilities not of artifacts can't be activated."
||| Neither anchor negates; the negation goes inside the anchor's own description.
public export
badNegatedAbilityAnchor : Unspellable (StaticEffect []) (\ok =>
  ObjectCant Activated
    (AllOf (And [ AbilityHead AnyActivated
                , Not (AbilityOf (AllOf Macros.artifact)) {ng = ok} ])))
badNegatedAbilityAnchor MkNegatable impossible


||| "You may {T} rather than pay this spell's mana cost."
||| [CR#107.5] fixes "{T}" as tapping a permanent, and a spell being paid for is on the stack [CR#601.2b].
public export
badAltCostTapSymbol : Unspellable (StaticEffect []) (\ok =>
  AltCost (Just TapSymbol) {ap = ok})
badAltCostTapSymbol NoAltPayment impossible
badAltCostTapSymbol AltPaymentWritten impossible


||| "You may [+1] rather than pay this spell's mana cost."
||| [CR#606.2] makes the loyalty symbol an activation-cost component, payable only on a permanent [CR#606.3].
public export
badAltCostLoyaltySymbol : Unspellable (StaticEffect []) (\ok =>
  AltCost (Just (LoyaltySymbol (LoyaltyUp 1))) {ap = ok})
badAltCostLoyaltySymbol NoAltPayment impossible
badAltCostLoyaltySymbol AltPaymentWritten impossible


||| "You may sacrifice a Mountain rather than pay this spell's mana cost" written as a resolving clause
||| [CR#113.6d] locates the alternative cost in the card's own printed text.
public export
badAltCostClause : Unspellable (Effect []) (\ok =>
  Continuously (AltCost (Just (Do (Macros.sacrifice You
                  (Macros.a (And [Macros.land, HasSubtype Mountain]))))))
               Nothing {sp = ok})
badAltCostClause SpanUnstated impossible


||| "You may sacrifice a Mountain rather than pay this spell's mana cost until end of turn."
||| A printed static ability has nothing to state a duration against [CR#604.1].
public export
badAltCostUntilEndOfTurn : Unspellable (Effect []) (\ok =>
  Continuously (AltCost (Just (Do (Macros.sacrifice You
                  (Macros.a (And [Macros.land, HasSubtype Mountain]))))))
               (Just Macros.untilEndOfTurn) {sp = ok})
badAltCostUntilEndOfTurn SpanStated impossible


||| "Regenerate target creature card in your graveyard."
||| [CR#701.19a] regenerates a permanent, shielding it the next time it would be destroyed.
public export
badRegenerateInGraveyard : Unspellable (Effect []) (\ok =>
  Regenerate (Macros.a (And [Macros.creature, InZone Macros.graveyardZ]))
             {zn = ok})
badRegenerateInGraveyard Oh impossible


||| "Creature cards in your graveyard can't be regenerated."
||| The prohibition's subject stands where the verb's object does, so [CR#701.19a] refuses it there too.
public export
badRegeneratedInGraveyard : Unspellable (StaticEffect []) (\ok =>
  ObjectCant Regenerated
             (Macros.a (And [Macros.creature, InZone Macros.graveyardZ]))
             {sub = ok})
badRegeneratedInGraveyard (RegeneratedOnField {zn = Oh}) impossible


||| "Destroy target creature. Target creature can't be regenerated."
||| A rider modifies the instruction it follows, so its subject reads back what that instruction named.
public export
badRiderAnnouncesTarget : Unspellable (Effect []) (\ok =>
  CantBe (Macros.destroy (Macros.target Macros.creature)) Regenerated
         (Macros.target Macros.creature) {bl = ok})
badRiderAnnouncesTarget MkBindingless impossible


||| "Destroy target creature. It can't be cast."
||| No preceding clause can have caused the event these prohibitions deny, so they never ride an instruction.
public export
badCastRider : Unspellable (Effect []) (\ok =>
  CantBe (Macros.destroy (Macros.target Macros.creature)) Cast It {rd = ok})
badCastRider Refl impossible


||| "{T}: Add {R}{G} or one mana of the chosen color."
||| This reader takes a single symbol beside the chosen colour, not a multi-symbol run.
public export
badMultiSymbolBesideChosen : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land])
       [ Static (EntersChoice Macros.thisLand Color)
       , Activated TapSymbol
           (AddMana You (Lit 1)
                    (OfChosenColor (Just [OfColor Red, OfColor Green])
                                   {ar = ok}) []) ]
       Nothing)
badMultiSymbolBesideChosen Oh impossible


||| "{T}: Add one mana of the chosen color." (on a card that chooses nothing)
||| The production reads a binding, and a card that made no choice has none to read [CR#607.2d].
public export
badChosenColorNoChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Land])
       [ Activated TapSymbol
           (AddMana You (Lit 1) (OfChosenColor Nothing {cq = ok}) []) ]
       Nothing)
badChosenColorNoChooser Refl impossible


||| "sources of the last chosen color" (on a card that chooses nothing)
||| The marked read demands that a choice stand at all, and zero is not one or more [CR#607.2d].
public export
badLastChosenColorNoChooser : Unspellable (Predicate [] Object) (\ok =>
  OfLastChosenColor {ok = ok})
badLastChosenColorNoChooser ChoiceMade impossible


||| "Prevent all damage that would be dealt to you by sources of the last chosen color. / As this enchantment enters, choose a color."
||| A line reads what its predecessors said and nothing that comes later [CR#607.2d].
public export
badLastChosenBeforeChooser : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (Prevents AnyDamage AllOfIt
                          (Macros.shieldingIt You)
                          (Just (AllOf (And [Macros.source,
                                             OfLastChosenColor {ok = ok}])))
                          Nothing)
       , Static (EntersChoice Macros.thisEnchantment Color) ]
       Nothing)
badLastChosenBeforeChooser ChoiceMade impossible


||| "As this enchantment enters, choose a creature type. / Prevent all damage that would be dealt to you by sources of the last chosen color."
||| The marked binding is sorted too, so a creature-type choice leaves the colour count at zero [CR#607.2d].
public export
badLastChosenWrongSort : Unspellable Card (\ok =>
  Macros.card "" Nothing [] (MkTypeLine [] [Enchantment])
       [ Static (EntersChoice Macros.thisEnchantment CreatureType)
       , Static (Prevents AnyDamage AllOfIt
                          (Macros.shieldingIt You)
                          (Just (AllOf (And [Macros.source,
                                             OfLastChosenColor {ok = ok}])))
                          Nothing) ]
       Nothing)
badLastChosenWrongSort ChoiceMade impossible
