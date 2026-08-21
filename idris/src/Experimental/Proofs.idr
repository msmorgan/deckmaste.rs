||| Compiler-checked proofs that certain English phrases are unspellable.
module Experimental.Proofs

import Experimental
import Experimental.Macros
import public Experimental.Unspellable

%default total

%unbound_implicits off


||| "Choose a flying."
||| A keyword is a modifier, never a noun head.
public export
badKeywordHead : Unspellable (Effect []) (\ok =>
  Choose (Macros.a (HasKeyword Flying) {hd = ok}))
badKeywordHead Oh impossible


||| "of the chosen name"
||| A bound card name reads back by name equality, never as a quality.
public export
badChosenCardNameRead :
  Unspellable
    (Predicate [MkBinding AD (Quality CardName) OneOf QualityP] Object)
    (\ok => OfChosen CardName {read = ok})
badChosenCardNameRead Oh impossible


||| "of the chosen number"
||| A bound number reads back by numeric equality, never as a quality.
public export
badChosenNumberRead :
  Unspellable
    (Predicate [MkBinding AD (Quality Number) OneOf QualityP] Object)
    (\ok => OfChosen Number {read = ok})
badChosenNumberRead Oh impossible


||| "Choose two target creatures. You gain life equal to their power."
||| Two creatures have no single power [CR#208.1]; a fold word writes the group's.
public export
badGroupPower : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup (Macros.exactly 2) Macros.creature),
                Macros.gainsLife You (Macros.powerOf Them {one = ok})])
badGroupPower Refl impossible


||| "Choose two target creatures. Their owner loses 1 life."
||| Two cards need not share an owner [CR#108.3].
public export
badGroupOwner : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup (Macros.exactly 2) Macros.creature),
                Macros.losesLife (OwnerOf Them {one = ok}) (Lit 1)])
badGroupOwner Refl impossible


||| "Two target creatures fight target creature."
||| The binary fight frame takes singular combatants, not a group.
public export
badFightGroup : Unspellable (Effect []) (\ok =>
  Fights (TargetGroup (Macros.exactly 2) Macros.creature) {pa = ok}
         (Macros.target Macros.creature))
badFightGroup Refl impossible


||| "a creature two target opponents control"
||| An object has one controller [CR#109.4], and a counted plural names no set.
public export
badControlledByGroup : Unspellable (Predicate [] Object) (\ok =>
  ControlledBy (TargetGroup (Macros.exactly 2) Opponent) {ps = ok})
badControlledByGroup Oh impossible


||| "two target opponents' graveyards"
||| Graveyards are per-player zones [CR#400.1]; a counted plural names no one.
public export
badGraveyardOfGroup : Unspellable (ZoneExpr []) (\ok =>
  Macros.graveyardOf (TargetGroup (Macros.exactly 2) Opponent) {pn = ok})
badGraveyardOfGroup Oh impossible


||| "When two target creatures die this turn, you gain 1 life."
||| The minted dies-watcher is singular; plural watches are unminted.
public export
badDiesGroup : Unspellable (Effect []) (\ok =>
  Delayed (Dies (TargetGroup (Macros.exactly 2) Macros.creature))
          {span = Just ThisTurn} (Macros.gainsLife You (Lit 1)) {one = ok})
badDiesGroup Refl impossible


||| "each of target creature"
||| The complement to EachOf must be plural.
public export
badEachOfSingular : Unspellable (Noun [] Object) (\ok =>
  EachOf (Macros.target Macros.creature) {pl = ok})
badEachOfSingular Refl impossible


||| "This deals 2 damage divided as you choose among target creature."
||| Damage division requires at least two objects to divide between.
public export
badDivideAmongSingular : Unspellable (Effect []) (\ok =>
  Macros.dealsDivided This (Lit 2) (Macros.target Macros.creature) {pl = ok})
badDivideAmongSingular Refl impossible


||| "Look at the top card of your opponents' library."
||| A slice names ONE library [CR#400.1]; a plural group possessor reaches several.
public export
badSliceOfGroupPossessor : Unspellable (Effect []) (\ok =>
  Macros.lookAt (LibrarySlice OnTop (Lit 1) (PlayerGroup YourOpponents) {sp = ok}))
badSliceOfGroupPossessor Oh impossible


||| "Whenever you cast all spells, draw a card."
||| Malformed trigger condition: Casts requires a singular spell.
public export
badCastsPluralComplement : Unspellable (Ability) (\ok =>
  Triggered Whenever (Casts You (AllOf Macros.spell) {one = ok})
            Macros.drawACard)
badCastsPluralComplement Refl impossible


||| "Tap target creature an opponent controls or land you control. That player loses 1 life."
||| Unspellable because its disjunction binds no coherent singular player antecedent.
public export
badDisjunctAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target (Or [And [Macros.creature, ControlledBy Macros.anOpponent],
                                        And [Macros.land, ControlledBy You]])),
                Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badDisjunctAntecedent Refl impossible


||| "Destroy target creature: Draw a card."
||| Destroy is not a cost verb.
public export
badDestroyAsCost : Unspellable Ability (\ok =>
  Activated (Do (Macros.destroy (Macros.a Macros.creature)) {ok})
            Macros.drawACard)
badDestroyAsCost Oh impossible


||| "Tap target creature with flying that doesn't have flying."
||| A keyword beside its own negation contradicts.
public export
badKeywordContradiction : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.target (And [Macros.creature, HasKeyword Flying,
                            Not (HasKeyword Flying)] {cf = ok})))
badKeywordContradiction Oh impossible


||| "Tap target nonland Forest."
||| Forest is a land type [CR#205.3i], so the conjunction contradicts itself.
public export
badForestNonland : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.target (And [HasSubtype Forest, Not Macros.land] {cf = ok})))
badForestNonland Oh impossible


||| "Snow Snow Land — Forest"
||| The type line's distinctness is per WORD, not per line.
public export
badDuplicateSnow : Unspellable Card (\ok =>
  Macros.card "" Nothing [Snow, Snow] (MkTypeLine [Forest] [Land]) [] Nothing {sp = ok})
badDuplicateSnow Oh impossible


||| "This deals 1 damage to any other target."
||| "Other" presupposes an earlier mention, and nothing precedes it.
public export
badOther : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))
badOther Refl impossible


||| "Target creature fights target creature. Tap it."
||| Two singular Object mentions precede "it", so uniqueness refuses.
public export
badIt : Unspellable (Effect []) (\ok =>
  Sequentially [Fights (Macros.target Macros.creature) (Macros.target Macros.creature),
                SetStatus Tapped (It {ok})])
badIt Refl impossible


||| "This deals 3 damage to each creature. Tap it."
||| A group is no singular antecedent: "it" has no referent.
public export
badTheyIt : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Each Macros.creature),
                SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badTheyIt (Refl, _) impossible


||| "Destroy target creature. At the beginning of the end step, sacrifice it."
||| The destroyed target is in the graveyard when the delayed sacrifice reads it [CR#701.21a].
public export
badStale : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               Delayed (BeginningOf EndStep NoPossessor) (Macros.sacrifice You It {ok})])
badStale OnField impossible


||| "This deals 2 damage to any target. At the beginning of the end step, this deals 1 damage to any other target."
||| A delayed clause announces its own targets [CR#603.3d,601.2c], so "other" has no witness.
public export
badDelayedOther : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 2) (Macros.target AnyTarget),
               Delayed (BeginningOf EndStep NoPossessor) (DealDamage This (Lit 1) (Macros.target (Macros.anyOtherTarget {ok})))])
badDelayedOther Refl impossible


||| "Exile target creature you control. Return that creature to the battlefield."
||| Exile retags the carrier [CR#110.1], so the typed demonstrative has no antecedent.
public export
badStaleCarrier : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile (Macros.target Macros.creatureYouControl),
               Move (That (TypeW Creature) {ok}) Macros.battlefieldZ])
badStaleCarrier Refl impossible


||| "Return a creature to its owner's hand: Tap it."
||| Hand is hidden [CR#400.2], so a cost mention there is unreadable past the colon.
public export
badHiddenCost : Unspellable Ability (\ok =>
  Activated (Do (Move (Macros.a Macros.creature) Macros.handZ))
            (SetStatus Tapped (It {ok = Builtin.fst ok}) {ok = Builtin.snd ok}))
badHiddenCost (Refl, _) impossible


||| "Discard a card, Sacrifice a creature: Exile it."
||| Two cost moves leave a bare "It" past the colon ambiguous.
public export
badTwoCostMentions : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.discardsACard You),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Macros.exile (It {ok})))
badTwoCostMentions Refl impossible


||| "Exile target creature. Sacrifice it."
||| An exiled referent is not sacrificeable [CR#701.21a].
public export
badSacrificeExiled : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.exile (Macros.target Macros.creature),
               Macros.sacrifice You It {ok}])
badSacrificeExiled OnField impossible


||| "When target creature dies this turn, return that creature to the battlefield."
||| The death retag flips the carrier [CR#700.4,110.1], so "creature" no longer reads.
public export
badDeadCreatureRead : Unspellable (Effect []) (\ok =>
  Delayed (Dies (Macros.target Macros.creature)) {span = Just ThisTurn}
          (Move (That (TypeW Creature) {ok}) Macros.battlefieldZ))
badDeadCreatureRead Refl impossible


||| "of the chosen creature type", with only a color chosen
||| The quality read is sort-filtered, so there is no witness.
public export
badChosenWrongSort : Unspellable
  (Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object)
  (\ok => OfChosen CreatureType {ok})
badChosenWrongSort Refl impossible


||| "Choose two target creatures. Choose two target creatures. Tap them."
||| Two group mentions leave "them" ambiguous.
public export
badThemAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [Choose (TargetGroup (Macros.exactly 2) Macros.creature),
               Choose (TargetGroup (Macros.exactly 2) Macros.creature),
               SetStatus Tapped (Them {ok})])
badThemAmbig Refl impossible


||| "Target creature an opponent controls fights target creature an opponent controls. That player loses 1 life."
||| Uniqueness reaches inside relative clauses: two opponents leave "that player" ambiguous.
public export
badInnerAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [Fights (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent]))
                       (Macros.target (And [Macros.creature, ControlledBy Macros.anOpponent])),
               Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badInnerAmbig Refl impossible


||| "Discard a card: Return the sacrificed card to the battlefield."
||| The participle's verb filter has no witness: nothing was sacrificed.
public export
badVerbedWrongVerb : Unspellable Ability (\ok =>
  Activated (Do (Macros.discardsACard You))
            (Move (TheVerbed Sacrifice CardW {ok}) Macros.battlefieldZ))
badVerbedWrongVerb Refl impossible


||| "Sacrifice an artifact: Return the sacrificed creature to the battlefield."
||| The noun word misses on the type axis: an artifact was sacrificed.
public export
badVerbedWrongNoun : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You (Macros.a (HasType Artifact))))
            (Move (TheVerbed Sacrifice (TypeW Creature) {ok}) Macros.battlefieldZ))
badVerbedWrongNoun Refl impossible


||| "Sacrifice a creature, Sacrifice a creature: Return the sacrificed card to the battlefield."
||| Two same-verb stamps leave the participle ambiguous.
public export
badVerbedAmbig : Unspellable Ability (\ok =>
  Activated (Compound [Do (Macros.sacrifice You (Macros.a Macros.creature)),
                       Do (Macros.sacrifice You (Macros.a Macros.creature))])
            (Move (TheVerbed Sacrifice CardW {ok}) Macros.battlefieldZ))
badVerbedAmbig Refl impossible


||| "Sacrifice this artifact: Exile target creature. At the beginning of the end step, return that card to the battlefield."
||| Two card mentions make "that card" ambiguous.
public export
badBareCardRead : Unspellable Ability (\ok =>
  Activated (Do (Macros.sacrifice You Macros.thisArtifact))
            (Sequentially [Macros.exile (Macros.target Macros.creature),
                           Delayed (BeginningOf EndStep NoPossessor) (Move (That CardW {ok}) Macros.battlefieldZ)]))
badBareCardRead Refl impossible


||| "Tap target creature card in your graveyard."
||| Tapping takes a battlefield object [CR#701.26a].
public export
badTapGraveyard : Unspellable (Effect []) (\ok =>
  SetStatus Tapped (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badTapGraveyard OnField impossible


||| "Destroy target tapped."
||| A bare status word heads nothing; the modifier needs a head beside it.
public export
badBareTappedHead : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target Macros.tapped {hd = ok}))
badBareTappedHead Oh impossible


||| "Destroy target tapped creature card in your graveyard."
||| Status is only a battlefield permanent's [CR#110.5d], so this names two zones at once.
public export
badTappedGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped,
                                       InZone (Macros.graveyardOf You)] {zc = ok})))
badTappedGraveyard Oh impossible


||| "Destroy target tapped untapped creature."
||| One value per category [CR#110.5]: a category clash, not a negation pair.
public export
badTappedUntapped : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, Macros.tapped, Macros.untapped] {cf = ok})))
badTappedUntapped Oh impossible


||| "nontapped"
||| The status word does not negate; the opposite value is its own row.
public export
badNonTapped : Unspellable (Predicate [] Object) (\ok =>
  Not Macros.tapped {ng = ok})
badNonTapped Oh impossible


||| "phased-in"
||| The value exists in the closed product [CR#110.5], but its surface cell is unwritten.
public export
badPhasedInWord : Unspellable (Predicate [] Object) (\ok =>
  HasStatus PhasedIn {at = ok})
badPhasedInWord Oh impossible


||| "Untap target creature card in your graveyard."
||| Untap takes a battlefield object [CR#701.26b].
public export
badUntapGraveyard : Unspellable (Effect []) (\ok =>
  SetStatus Untapped (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badUntapGraveyard OnField impossible


||| "Destroy target permanent instant."
||| "Permanent" beside a projected instant head describes nothing [CR#110.4].
public export
badPermanentInstant : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Permanent, HasType Instant] {cf = ok})))
badPermanentInstant Oh impossible


||| "Destroy target permanent. Tap that permanent."
||| Destruction retags to the graveyard, and [CR#110.1] takes the word away with the zone.
public export
badThatPermanentDeparted : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Permanent),
               SetStatus Tapped (That PermanentW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badThatPermanentDeparted (Refl, _) impossible


||| "Destroy target token card in your graveyard."
||| A token off the battlefield has ceased to exist [CR#111.7].
public export
badTokenGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [IsToken, InZone (Macros.graveyardOf You)] {zc = ok})))
badTokenGraveyard Oh impossible


||| "Destroy target nontoken token."
||| The negation pair the contradiction scan already refuses.
public export
badNontokenToken : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [IsToken, Macros.nontoken] {cf = ok})))
badNontokenToken Oh impossible


||| "Tap target creature. Untap that token."
||| The origin field is written only by the create clause [CR#111.1].
public export
badThatTokenOfCard : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target Macros.creature),
               SetStatus Untapped (That TokenW {ok = Builtin.fst ok}) {ok = Builtin.snd ok}])
badThatTokenOfCard (Refl, _) impossible


||| "A creature card in your graveyard doesn't untap during its controller's untap step."
||| The lock's subject stands on the battlefield — only a permanent untaps [CR#701.26b].
public export
badUntapLockGraveyard : Unspellable Ability (\ok =>
  Static (DoesntUntap (Macros.a (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok}))
badUntapLockGraveyard Oh impossible


||| "This artifact doesn't untap." written as a clause
||| A durationless "doesn't untap" is the static ability line, not a clause.
public export
badUntapLockClause : Unspellable (Effect []) (\ok =>
  Continuously (DoesntUntap (AsType Artifact This)) Nothing {sp = ok})
badUntapLockClause SpanUnstated impossible


||| "Target creature card in your graveyard fights target creature."
||| Only battlefield creatures fight [CR#701.14b].
public export
badFightGraveyard : Unspellable (Effect []) (\ok =>
  Fights (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {za = ok}
         (Macros.target Macros.creature))
badFightGraveyard OnField impossible


||| "Target land fights target creature you don't control."
||| Only combatant types fight [CR#701.14a]; a land declares no fight participation.
public export
badFightLand : Unspellable (Effect []) (\ok =>
  Fights (Macros.target (HasType Land)) {ta = ok} (Macros.target Macros.creatureYouDontControl))
badFightLand Fighter impossible


||| "When target creature card in your graveyard dies this turn, return that card to the battlefield."
||| Dying is the battlefield-to-graveyard transition [CR#700.4]; a graveyard head contradicts it.
public export
badDiesInGraveyard : Unspellable (Effect []) (\ok =>
  Delayed (Dies (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {zn = ok}) {span = Just ThisTurn}
          (Move (That CardW) Macros.battlefieldZ))
badDiesInGraveyard Oh impossible


||| "Destroy target creature. This deals 3 damage to it."
||| Damage reaches players and battlefield objects only [CR#120.1].
public export
badDamageGraveyardCard : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
               DealDamage This (Lit 3) It {rk = ok}])
badDamageGraveyardCard ObjectTakes impossible


||| "This deals 1 damage to a color."
||| A quality cannot take damage [CR#120.1].
public export
badDamageToColor : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.a (QualityNoun Color)) {rk = ok})
badDamageToColor ObjectTakes impossible


||| "This deals 1 damage to target artifact."
||| Damage goes to battles, creatures, planeswalkers, or players [CR#120.1a].
public export
badDamageArtifact : Unspellable (Effect []) (\ok =>
  DealDamage This (Lit 1) (Macros.target (HasType Artifact)) {rk = ok})
badDamageArtifact ObjectTakes impossible


||| "Choose a noncolor."
||| A phrase needs a positive head [CR#105.1,608.2d]; Not is never one.
public export
badNegatedQualityHead : Unspellable (Effect []) (\ok =>
  Choose (Macros.a (Not (QualityNoun Color)) {hd = ok}))
badNegatedQualityHead Oh impossible


||| "non-player"
||| The universal player word names one of the game's people [CR#102.1]; "non-" has no complement.
public export
badNegatedPlayerHead : Unspellable (Predicate [] Player) (\ok =>
  Not AnyPlayer {ng = ok})
badNegatedPlayerHead Oh impossible


||| "Tap target creature an opponent doesn't control. That player loses 1 life."
||| Negation binds nothing, so no opponent is named for "that player".
public export
badNegatedAntecedent : Unspellable (Effect []) (\ok =>
  Sequentially [SetStatus Tapped (Macros.target (And [Macros.creature, Not (ControlledBy Macros.anOpponent)])),
                Macros.losesLife (That PlayerW {ok}) (Lit 1)])
badNegatedAntecedent Refl impossible


||| "Destroy target creature on the battlefield in a graveyard."
||| An object is in ONE zone; the conjuncts refuse in either order.
public export
badConflictingZones : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone Macros.battlefieldZ, InZone Macros.graveyardZ] {zc = ok})))
badConflictingZones Oh impossible


||| "Choose target color."
||| Targets are objects and players [CR#115.1]; qualities are chosen.
public export
badTargetColor : Unspellable (Effect []) (\ok =>
  Choose (Macros.target (QualityNoun Color) {tk = ok}))
badTargetColor ObjectTgt impossible


||| "Destroy target creature. It gets +3/+3 until end of turn."
||| The one-shot stat modification takes a battlefield object.
public export
badGetsGraveyard : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target Macros.creature),
                Macros.gets It (PtUp (Lit 3)) (PtUp (Lit 3)) (Just Macros.untilEndOfTurn) {ok}])
badGetsGraveyard Oh impossible


||| "Put target creature into target player's hand."
||| A card never enters another player's hand [CR#400.3].
public export
badMoveToTargetsHand : Unspellable (Effect []) (\ok =>
  Move (Macros.target Macros.creature) (Macros.handOf (Macros.target AnyPlayer)) {ok})
badMoveToTargetsHand HandOkBare impossible


||| "Destroy target creature card in your graveyard."
||| Only a battlefield permanent is destroyable [CR#701.8a].
public export
badDestroyGraveyard : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.target (And [Macros.creature, InZone (Macros.graveyardOf You)])) {ok})
badDestroyGraveyard OnField impossible


||| "You discard a creature."
||| Discarding moves a card from a HAND [CR#701.9a].
public export
badDiscardBattlefield : Unspellable (Effect []) (\ok =>
  Macros.discards You (Macros.a Macros.creature) {dk = ok})
badDiscardBattlefield DiscardTracked impossible


||| "Destroy target creature." spelled over an exile body
||| Tag and body must agree, or indestructible would cant an exile [CR#701.8b,702.12b].
public export
badDestroyTaggedExile : Unspellable (Effect []) (\ok =>
  Composite Destroy (Move (Macros.target Macros.creature) Macros.exileZ) {ok})
badDestroyTaggedExile DestroyB impossible


||| "Destroy target creature card in a graveyard." spelled raw
||| The zone demand lives on the tag-body relation, so the raw spelling proves it too [CR#701.8a].
public export
badCompositeDestroyGraveyard : Unspellable (Effect []) (\ok =>
  Composite Destroy (Move (Macros.target (And [Macros.creature, InZone Macros.graveyardZ])) Macros.graveyardZ) {ok = DestroyB {z = ok}})
badCompositeDestroyGraveyard OnField impossible


||| "Sacrifice a creature." with no actor
||| An agentive tag cannot shed its actor [CR#701.21a].
public export
badAgentlessSacrifice : Unspellable (Effect []) (\ok =>
  Composite Sacrifice (Move (Macros.a Macros.creature) Macros.graveyardZ) {ok = SacrificeB} {na = ok})
badAgentlessSacrifice Oh impossible


||| "You discard a creature." spelled under Does
||| Discarding moves a hand card [CR#701.9a], and the demand rides the tag relation.
public export
badDoesDiscardBattlefield : Unspellable (Effect []) (\ok =>
  Does You Discard (Move (Macros.a Macros.creature) Macros.graveyardZ) {tb = DiscardB {d = ok}})
badDoesDiscardBattlefield DiscardTracked impossible


||| "Choose zero target creatures."
||| A written quantity permits at least one; the demand is on its MAXIMUM.
public export
badZeroGroup : Unspellable (Effect []) (\ok =>
  Choose (TargetGroup (Macros.exactly 0) Macros.creature {nz = Builtin.fst ok} {wf = Builtin.snd ok}))
badZeroGroup (MaxAtLeastOne, _) impossible


||| "your battlefield"
||| Shared zones take no possessor: [CR#400.1] gives each player library, hand, and graveyard.
public export
badOwnedBattlefield : Unspellable (ZoneExpr []) (\ok =>
  ZoneAt Battlefield (OwnedBy You {ps = ok}))
badOwnedBattlefield HandIsOwned impossible


||| "Discard a card at random: This deals damage equal to the discarded creature's mana value to any target."
||| A bare type word denotes a permanent [CR#109.2]; what was discarded left a hand [CR#701.9a].
public export
badDiscardedCreatureWord : Unspellable Ability (\ok =>
  Activated (Do (Macros.discards You (Macros.aAtRandom (And [Macros.creature, InZone Macros.handZ]))))
            (DealDamage This
                          (Macros.manaValueOf (TheVerbed Discard (TypeW Creature) {ok}))
                          (Macros.target AnyTarget)))
badDiscardedCreatureWord Refl impossible


||| "Destroy a creature of their choice."
||| "Of their choice" demands a player antecedent: a subject or one distributive group [CR#608.2d].
public export
badUnboundTheirChoice : Unspellable (Effect []) (\ok =>
  Macros.destroy (Macros.aTheirChoice Macros.creature {ch = ok}))
badUnboundTheirChoice Refl impossible


||| "This deals that much damage to any target."
||| "That much" with nothing done yet: no outcome to read.
public export
badThatMuchUnbound : Unspellable (Effect []) (\ok =>
  DealDamage This (ThatMuch {ok}) (Macros.target AnyTarget))
badThatMuchUnbound Refl impossible


||| "This deals 3 damage to any target. You lose 2 life. You gain that much life."
||| Two event clauses leave "that much" ambiguous.
public export
badThatMuchAmbig : Unspellable (Effect []) (\ok =>
  Sequentially [DealDamage This (Lit 3) (Macros.target AnyTarget),
                Macros.losesLife You (Lit 2),
                Macros.gainsLife You (ThatMuch {ok})])
badThatMuchAmbig Refl impossible


||| "Destroy target land. Destroy another target creature."
||| "Other" needs a head-compatible anchor, and a land is none for "another creature" [CR#601.2c].
public export
badOtherCrossHead : Unspellable (Effect []) (\ok =>
  Sequentially [Macros.destroy (Macros.target (HasType Land)),
                Macros.destroy (Macros.target (And [Macros.creature, Other] {oa = ok}))])
badOtherCrossHead Oh impossible


||| "for each you control"
||| Every for-each domain is noun-headed; this names no set to count.
public export
badForEachHeadless : Unspellable (Amount []) (\ok =>
  Macros.forEach (ControlledBy You) {hd = ok})
badForEachHeadless Oh impossible


||| "1 life for each 0 creatures"
||| A written numeral is at least one.
public export
badForEachZero : Unspellable (Amount []) (\ok =>
  Macros.nForEach 0 Macros.creature {nz = ok})
badForEachZero ItIsSucc impossible


||| "creature that isn't on the battlefield"
||| A bare "creature" means the battlefield [CR#109.2], which the negation contradicts.
public export
badNotOnBattlefield : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, Not (InZone Macros.battlefieldZ)] {zc = ok})
badNotOnBattlefield Oh impossible


||| "of the chosen color and not of the chosen color"
||| No member negates a sibling.
public export
badQualityContradiction : Unspellable
  (Predicate [MkBinding AD (Quality Color) OneOf QualityP] Object)
  (\ok => And [OfChosen Color, Not (OfChosen Color)] {cf = ok})
badQualityContradiction Oh impossible


||| "creature that is a noncreature", the clash one level down
||| The member scan flattens conjunctions, so nesting is no laundering.
public export
badNestedContradiction : Unspellable (Predicate [] Object) (\ok =>
  And [Macros.creature, And [Not Macros.creature]] {cf = ok})
badNestedContradiction Oh impossible


||| "attacking card in your hand"
||| Attacking seeds the battlefield [CR#508.1a,506.4], which the hand clause contradicts.
public export
badAttackingInHand : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, InZone Macros.handZ] {zc = ok})
badAttackingInHand Oh impossible


||| "Two target creatures deal 3 damage to target player."
||| The damage frame names one source; a group subject distributes instead.
public export
badGroupDamageSource : Unspellable (Effect []) (\ok =>
  DealDamage (TargetGroup (Macros.exactly 2) Macros.creature) (Lit 3) (Macros.target AnyPlayer) {ds = ok})
badGroupDamageSource Oh impossible


||| "non-any target"
||| [CR#115.4] defines "any target" positively, so "non-" has no complement to take.
public export
badNegatedAnyTarget : Unspellable (Predicate [] Object) (\ok =>
  Not AnyTarget {ng = ok})
badNegatedAnyTarget Oh impossible


||| "any target in a graveyard"
||| It takes no modifier but "other"; a restriction would make it a synonym for "any object".
public export
badAnyTargetInGraveyard : Unspellable (Predicate [] Object) (\ok =>
  And [AnyTarget, InZone Macros.graveyardZ] {at = ok})
badAnyTargetInGraveyard Oh impossible


||| "a any target" / "each any target"
||| "Any target" is ITSELF the targeting form, so only the targeting determiners admit it.
public export
badAnyTargetUnderA : Unspellable (Noun [] Object) (\ok =>
  Macros.a AnyTarget {af = ok})
badAnyTargetUnderA Oh impossible


||| "two any targets"
||| The counted plural drops "any": oracle writes "two targets" [CR#115.4].
public export
badGroupAnyTarget : Unspellable (Noun [] Object) (\ok =>
  TargetGroup (Macros.exactly 2) AnyTarget {af = ok})
badGroupAnyTarget Oh impossible


||| "Discard this creature."
||| A type-worded self-reference denotes the permanent [CR#109.2], not a hand card [CR#701.9a].
public export
badDiscardThisCreature : Unspellable (Effect []) (\ok =>
  Macros.discards You Macros.thisCreature {dk = ok})
badDiscardThisCreature DiscardTracked impossible


||| "this creature" spelled over "target creature"
||| The ascription is the SOURCE's and only the source's; re-sorting a target spells nothing new.
public export
badAscribedTarget : Unspellable (Noun [] Object) (\ok =>
  AsType Creature (Macros.target Macros.creature) {asc = ok})
badAscribedTarget AscribeThis impossible


||| "non-" over a conjunction rather than a single modifier
||| Negation is atomic: it attaches to one modifier, and a conjunction negates per-member.
public export
badNegatedConjunction : Unspellable (Predicate [] Object) (\ok =>
  Not (And [Macros.creature]) {ng = ok})
badNegatedConjunction Oh impossible


||| "creature you control that you don't control"
||| "You" denotes one player at both mentions, so the phrase asserts and denies one fact.
public export
badControlContradiction : Unspellable (Predicate [] Object) (\ok =>
  And [ControlledBy You, Not (ControlledBy You)] {cf = ok})
badControlContradiction Oh impossible


||| "attacking noncreature"
||| Only a creature can attack [CR#506.3], so the status word presupposes the type.
public export
badAttackingNoncreature : Unspellable (Predicate [] Object) (\ok =>
  And [Attacking, Not Macros.creature] {cf = ok})
badAttackingNoncreature Oh impossible
