import Semantics.Definitions.Phrasings
import Semantics.Check.Phrase

/-!
# Semantics.Definitions.Actions

Rules-defined actions spelled over `Semantics.Templates`: attaching, creating and
countering, the other actions, the predefined tokens, the phrasing templates that need these
actions, and the keyword actions.
-/

namespace Semantics.Definitions

open Semantics.Templates

/-! ## Attach, create, and counter -/

/-- "attach <subject> to <host>" (`Definition`, [CR#701.3a]). -/
semantic_definition attachTo (subject host : NounPhrase) : Action :=
  .ownScope (some (.action "attach"))
    (.attachment .attached (.firstMention "attachTo.subject" (.inCaller subject))
      (some (.firstMention "attachTo.host" (.inCaller host))))
/-- "create <count> <token> [riders]": the tokens are put onto the battlefield, entering under
their creator's control (`Definition`, [CR#701.7a,111.2]). -/
semantic_definition create (count : Amount) (token : TokenSpec) (riders : List MoveRider := []) :
    Action :=
  .ownScope (some (.action "create"))
    (.createObject (.inCaller count) (.token (.inCaller token) (riders.map MoveRider.inCaller)) none)
/-- "counter <subject>": one labeled stack-to-graveyard move (`Definition`, [CR#701.6a]). -/
semantic_definition counter (subject : NounPhrase) : Action :=
  .ownScope (some (.action "counter"))
    (.move (.firstMention "counter.subject" (.inCaller subject)) (.zone .stack none)
      (some (.zone .graveyard none))
      .arrival [])

/-! ## Actions -/

/-- "cast <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.5a]). -/
semantic_definition cast (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action :=
  .ownScope (some (.action "cast"))
    (.procedure .cast (.firstMention "cast.subject" (.inCaller subject)) payment.inCaller
      (resulting.map .inCaller) (from_.map .inCaller) none)
/-- "play <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.18a]). -/
semantic_definition play (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action :=
  .ownScope (some (.action "play"))
    (.procedure .play (.firstMention "play.subject" (.inCaller subject)) payment.inCaller
      (resulting.map .inCaller) (from_.map .inCaller) none)
/-- "activate <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.2a]). -/
semantic_definition activate (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action :=
  .ownScope (some (.action "activate"))
    (.procedure .activate (.firstMention "activate.subject" (.inCaller subject))
      payment.inCaller (resulting.map .inCaller) (from_.map .inCaller) none)
/-- "<subject> resolves" (`Definition`, [CR#608.1,608.2]): the resolution of a spell or ability.
No player pays for, casts it from a zone or chooses its result, so the payment, origin and
resulting slots are the neutral `itsOwnCost`, `none` and `none`.

"Put into a graveyard during its resolution" (haunt's spell half [CR#702.55a]) and "as it
resolves" (buyback [CR#702.27a], rebound [CR#702.88a]) write the resolution as the cause of the
move: `GameEvent.causes (.event (does .any (resolve thisSpell))) <move>` on a trigger event and
`WouldClause.causedBy (.event …) <clause>` on a replacement's would-clause. `Causing.event` is the
relation the rules give: the spell is put into the graveyard as the final part of its resolution
[CR#608.2n], so the resolution causes the move, and the one form serves a trigger event and a
would-clause alike. `Timing.whileDoing` would state only a co-occurrence and lives in the timing
slot only, which a replacement's would-clause has not. A countered spell's move (`counter`) is
caused by no resolution [CR#701.6a], so none of the three applies to it. -/
semantic_definition resolve (subject : NounPhrase) : Action :=
  .ownScope (some (.action "resolve"))
    (.procedure .resolve (.firstMention "resolve.subject" (.inCaller subject)) .itsOwnCost none
      none none)
/-- "destroy <subject>" (`Definition`, [CR#701.8]). -/
semantic_definition destroy (subject : NounPhrase) : Action :=
  .ownScope (some (.action "destroy"))
    (.move (.firstMention "destroy.subject" (.inCaller subject)) (.zone .battlefield none)
      graveyard .departure [])
/-- "exile <subject>" (`Definition`, [CR#701.13]). -/
semantic_definition exile (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.move (.firstMention "exile.subject" (.inCaller subject)) .anyLocation exileZone
      .arrival [])
/-- "exile <subject> face down" (`Definition`, [CR#701.13,406.3]). -/
semantic_definition exileFaceDown (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.move (.firstMention "exileFaceDown.subject" (.inCaller subject)) .anyLocation
      exileZone .arrival
      [.faceDown exiledFaceDown])
/-- "exile <subject> with N <kind> counters on it" (`Definition`, [CR#701.13]). -/
semantic_definition exileWithCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action :=
  .ownScope (some (.action "exile"))
    (.move (.firstMention "exileWithCounters.subject" (.inCaller subject)) .anyLocation
      exileZone .arrival
      [.withCounters (.inCaller amount) kind .fresh])
/-- "sacrifice <subject>" (`Definition`, [CR#701.21]). -/
semantic_definition sacrifice (subject : NounPhrase) : Action :=
  .ownScope (some (.action "sacrifice"))
    (.move (.firstMention "sacrifice.subject" (.inCaller subject)) (.zone .battlefield none)
      graveyard .departure [])
/-- "transform <subject> [into <face>]" (`Definition`, [CR#701.27]): `into` is what a "transforms
into" trigger watches, a characteristic the object has immediately after [CR#701.27e]. -/
semantic_definition transform (subject : NounPhrase) (into : Option Predicate := none) : Action :=
  .ownScope (some (.action "transform"))
    (.turnOver (.firstMention "transform.subject" (.inCaller subject)) (into.map .inCaller))
/-- "<subject> transforms into <name>": the transform deed whose result is the named face, done by
anyone [CR#701.27a,701.27e]. -/
semantic_template transformsInto (subject : NounPhrase) (name : NameSource) : GameEvent :=
  does any (transform subject (into := some (named name)))
/-- "meld <subject> into <name>" (`Definition`, [CR#701.42]). -/
semantic_definition meldInto (subject : NounPhrase) (into : String) : Action :=
  .ownScope (some (.action "meld"))
    (.move (.firstMention "meldInto.subject" (.inCaller subject)) .anyLocation battlefield
      .arrival [.entersMelded into])
/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a]. -/
semantic_definition manifestPlacement (subject : NounPhrase) : Action :=
  .ownScope (some (.action "manifest"))
    (.move (.firstMention "manifestPlacement.subject" (.inCaller subject)) .anyLocation
      battlefield .arrival [])
/-- "tap <subject>" (`Definition`, [CR#701.26a]). -/
semantic_definition tap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "tap"))
    (.setStatus .tapped (.firstMention "tap.subject" (.inCaller subject)))
/-- "the regeneration of <subject>": a regeneration shield's application, "remove all damage
marked on it and tap it. If it's an attacking or blocking creature, remove it from combat"
[CR#701.19a], the deed "can't be regenerated" forbids. Activating or casting what creates a
shield is not regenerating (`regenerate`, [CR#701.19c]) (`Definition`). -/
semantic_definition regeneration (subject : NounPhrase) : Action :=
  .ownScope (some (.action "regenerate"))
    (sequentially
      [ clearDamage (.firstMention "regeneration.subject" (.inCaller subject)),
        instruct
          (.possessorOf .controller (.laterMention "regeneration.subject" (.inCaller subject)))
          (tap (.laterMention "regeneration.subject" (.inCaller subject))),
        doIf
          (or
            [ is (.laterMention "regeneration.subject" (.inCaller subject))
                (.inCombat .attackerOf none),
              is (.laterMention "regeneration.subject" (.inCaller subject))
                (.inCombat .blockerOf none) ])
          (removeFromCombat (.laterMention "regeneration.subject" (.inCaller subject)))
          none ])
/-- "Destroy <subject>. It can't be regenerated." / "… They can't be regenerated.": the
regeneration the prohibition forbids is of the object or objects just destroyed, and the
prohibition rides that destruction, so no regeneration shield replaces it
[CR#608.2c,701.19c]. -/
semantic_template destroyCantRegenerate (subject : NounPhrase) : Action :=
  doUnderProhibition (destroy subject) (regeneration (.pro .bare subject.plur))
/-- "<performer> discards <subject>": a move from the performer's own hand, the card's owner's,
to the graveyard (`Definition`, [CR#701.9a]). -/
semantic_definition discard (subject : NounPhrase) : Action :=
  .ownScope (some (.action "discard"))
    (.move (.firstMention "discard.subject" (.inCaller subject)) (.zone .hand (some .actor))
      graveyard .arrival [])
/-- A bare "shuffle": the performer's own library (`Definition`, [CR#701.24a]). -/
semantic_definition shuffle : Action :=
  .ownScope (some (.action "shuffle"))
    (.shuffle (libraryOf .actor))
/-- "untap <subject>" (`Definition`, [CR#701.26b]). -/
semantic_definition untap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "untap"))
    (.setStatus .untapped (.firstMention "untap.subject" (.inCaller subject)))
/-- "<performer> mills <amount> cards": from the performer's own library (`Definition`,
[CR#701.17a]). -/
semantic_definition mill (amount : Amount) : Action :=
  .ownScope (some (.action "mill"))
    (.move (.slice .top (.inCaller amount) (libraryOf .actor))
      (.zone .library none) graveyard .arrival [])
/-- "<cards> are milled": the mill whose patient is a card description, as an event a trigger
watches ("Whenever one or more nonland cards are milled") (`Definition`, [CR#701.17a,701.17c]). -/
semantic_definition millCards (cards : NounPhrase) : Action :=
  .ownScope (some (.action "mill"))
    (.move (.firstMention "millCards.cards" (.inCaller cards))
      library graveyard .arrival [])
/-- "search <whose> graveyard, hand, and library for <for_>", the owner written once
(`Definition`, [CR#701.23]). -/
semantic_definition searchZonesOf (whose : NounPhrase) (for_ : NounPhrase) : Action :=
  .ownScope (some (.action "search"))
    (.search
      (and
        [ .zone .graveyard (some (.firstMention "searchZonesOf.whose" (.inCaller whose))),
          .zone .hand (some (.laterMention "searchZonesOf.whose" (.inCaller whose))),
          .zone .library (some (.laterMention "searchZonesOf.whose" (.inCaller whose))) ])
      (.inCaller for_))
/-- "<creature> fights <creature>" (`Definition`, [CR#701.14a]). -/
semantic_definition fight (other : NounPhrase) : Action :=
  .ownScope (some (.action "fight" .performs))
    (doIf (and [
      is (.firstMention "fight.other" (.inCaller other)) isCreature,
      is (.firstMention "fight.source" .actor) isCreature])
      (.simultaneously [
        .dealDamage (.statOf (.stat .power) (.laterMention "fight.source" .actor))
          (.laterMention "fight.other" (.inCaller other)),
        instruct (.laterMention "fight.other" (.inCaller other))
          (.dealDamage (.statOf (.stat .power) (.laterMention "fight.other" (.inCaller other)))
            (.laterMention "fight.source" .actor))])
      none)
/-- "<pair> fight each other": the two-creature form of fight, spelled as the rule's own sentence,
"Each of those creatures deals damage equal to its power to the other creature" (`Definition`,
[CR#701.14a..701.14c]). The deed is `fight`'s, so "whenever a creature fights" and "if it fought
this way" read this fight as they read the other form. If either creature is no longer a
creature on the battlefield, neither deals damage [CR#701.14b]. "The other" is the current
member's counterpart in the group of two (`DRS.distributivePair?`). That group has one member
per mention, so if both target slots name one creature, each slot deals its power to the other
slot and the creature takes twice its power [CR#701.14c]; no separate case is written. -/
semantic_definition fightEachOther (pair : NounPhrase) : Action :=
  .ownScope (some (.action "fight" .performs))
    (doIf (is (.firstMention "fightEachOther.pair" (.inCaller pair)) isCreature)
      (instruct (eachOf (.laterMention "fightEachOther.pair" (.inCaller pair)))
        (.dealDamage (powerOf it) (theOther .object)))
      none)
/-- "reveal <cards>" (`Definition`, [CR#701.20]). -/
semantic_definition reveal (cards : NounPhrase) : Action :=
  .ownScope (some (.action "reveal"))
    (.expose .reveal (.cards (.firstMention "reveal.cards" (.inCaller cards))))
/-- "shuffle <subject> into its owner's library": the move, then the shuffle of that library,
which happens even if nothing moved (`Definition`, [CR#701.24c,701.24d]). -/
semantic_definition shuffleInto (subject : NounPhrase) : Action :=
  .ownScope (some (.action "shuffle"))
    (sequentially
      [ .move (.firstMention "shuffleInto.subject" (.inCaller subject)) .anyLocation
          (some (.zone .library none))
          .arrival [],
        .shuffle (.zone .library none) ])
/-- "vote for <ballot> [<times> times]" (`Definition`, [CR#701.38]). -/
semantic_definition vote (disclosure : Disclosure := .openly)
    (ballot : Ballot := anyBallot)
    (times : Option Quantity := none) : Action :=
  .ownScope (some (.action "vote"))
    (.vote none disclosure ballot.inCaller (times.map Quantity.inCaller))
/-- "Starting with <first>, vote for <ballot>" (`Definition`, [CR#701.38]). -/
semantic_definition voteStartingWith (first : NounPhrase) (disclosure : Disclosure)
    (ballot : Ballot) : Action :=
  .ownScope (some (.action "vote"))
    (.vote (some (.firstMention "voteStartingWith.first" (.inCaller first))) disclosure
      ballot.inCaller none)
/-- "scry N" (`Definition`, [CR#701.22a]): the performer's own library. -/
semantic_definition scry (amount : Amount) : Action :=
  .ownScope (some (.action "scry"))
    (lookAndSortSlice (.slice .top (.inCaller amount) (libraryOf .actor)) onBottom
      [(.inOrder .anyOrder)])
/-- "fateseal N" (`Definition`, [CR#701.29a]): the top N cards of an opponent's library, an
opponent of the performer. -/
semantic_definition fateseal (amount : Amount) : Action :=
  .ownScope (some (.action "fateseal"))
    (lookAndSortSlice (.slice .top (.inCaller amount) (libraryOf (a (isOpponentOf .actor))))
      onBottom [(.inOrder .anyOrder)])
/-- "surveil N" (`Definition`, [CR#701.25a]): the performer's own library. -/
semantic_definition surveil (amount : Amount) : Action :=
  .ownScope (some (.action "surveil"))
    (lookAndSortSlice (.slice .top (.inCaller amount) (libraryOf .actor)) graveyard)
/-- "Proliferate" with its reminder text (`Definition`, [CR#701.34a]). -/
semantic_definition proliferate : Action :=
  .ownScope (some (.action "proliferate"))
    (sequentially
    [ .choose none (counted anyNumber (and [andOr [isPermanent, isPlayer], .hasCounters none]))
        .openly none,
      .doForEach (those .join) (putCounters 1 (.sameAs it) it) ])
/-- "<performer> amasses <subtype> N" with its reminder text (`Definition`, [CR#701.47a]): the
chosen Army is `that (.type .creature)` and `it`, resolved inside `ownScope`. -/
semantic_definition amass (subtype : Subtype) (count : Amount) : Action :=
  .ownScope (some (.action "amass"))
    (sequentially
    [ doIf (not (exists_ army))
        (create 1 (.written (creatureToken 0 0 [.black] [subtype, Subtype.army])))
        none,
      choose (a army),
      putCounters (.inCaller count) p1p1Counter (that (.type .creature)),
      doIf (not (is it (.hasSubtype subtype)))
        (become it { characteristics := { subtypes := [subtype] } })
        none ])
/-- "monstrosity N" with its reminder text (`Definition`, [CR#701.37a]): an instruction about
"this permanent" (`DeedPermanent.isAbout`), the Source unless a handoff names another, that the
ability's controller follows [CR#608.2c,701.37c]; the body reads that permanent as `.actor`. -/
semantic_definition monstrosity (amount : Amount) : Action :=
  .ownScope (some (.action "monstrosity" .isAbout))
    (doIf (not (is .actor (hasDesignation monstrous)))
      (sequentially
        [ putCounters (.inCaller amount) p1p1Counter .actor,
          gainDesignation .actor monstrous ])
      none)
/-- "double the number of each kind of counter on <on>" (`Definition`, [CR#701.10e]): for each
kind of counter on it, put as many of that kind on it as it already has. -/
semantic_definition doubleCounters (on : NounPhrase) : Action :=
  .ownScope (some (.action "double"))
    (.doForEachKind .counterKind (some (.firstMention "doubleCounters.on" (.inCaller on)))
      .counterKind
      (putCounters (.statOf .boundCounter (.laterMention "doubleCounters.on" (.inCaller on)))
        .bound (.laterMention "doubleCounters.on" (.inCaller on))))
semantic_definition exchangeCards (left right : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (.exchange
      (.cardsAcross (.firstMention "exchangeCards.left" (.inCaller left))
        (.firstMention "exchangeCards.right" (.inCaller right))))
semantic_definition exchangeControl (left right : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (.exchange
      (.controlOf (.firstMention "exchangeControl.left" (.inCaller left))
        (.firstMention "exchangeControl.right" (.inCaller right))))
semantic_definition exchangeLifeTotals (parties : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (.exchange (.lifeTotals (.firstMention "exchangeLifeTotals.parties" (.inCaller parties))))
semantic_definition exchangeTextBoxes (left right : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (.establish
      (.textChange (.firstMention "exchangeTextBoxes.left" (.inCaller left))
        (.exchangeWith (.firstMention "exchangeTextBoxes.right" (.inCaller right))))
      none)
semantic_definition exchangeValues (left right : Amount) : Action :=
  .ownScope (some (.action "exchange"))
    (.exchange (.values (.inCaller left) (.inCaller right)))
semantic_definition exchangeZones (left right : Location) : Action :=
  .ownScope (some (.action "exchange"))
    (.exchange (.zones (.inCaller left) (.inCaller right)))
semantic_definition exileFrom (subject : NounPhrase) (from_ : Location) : Action :=
  .ownScope (some (.action "exile"))
    (.move (.firstMention "exileFrom.subject" (.inCaller subject)) (.inCaller from_)
      exileZone .arrival [])
/-- "exile <subject> from <from_> face down" (`Definition`, [CR#701.13,406.3]): a move that names
its origin, so an ability whose cost or effect makes it functions only in that zone
[CR#113.6m]. -/
semantic_definition exileFaceDownFrom (subject : NounPhrase) (from_ : Location) : Action :=
  .ownScope (some (.action "exile"))
    (.move (.firstMention "exileFaceDownFrom.subject" (.inCaller subject)) (.inCaller from_)
      exileZone .arrival [.faceDown exiledFaceDown])
/-- "<performer> reveals their hand" (`Definition`, [CR#701.20]). -/
semantic_definition revealHand : Action :=
  .ownScope (some (.action "reveal"))
    (.expose .reveal (.zone (handOf .actor)))
/-- "<performer> collects evidence N" (`Definition`, [CR#701.59a]). -/
semantic_definition collectEvidence (amount : Amount) : Action :=
  .ownScope (some (.action "collectEvidence"))
    (exile
      (withTotal (.stat .manaValue) .atLeast (.inCaller amount)
        (allOf (and [.isCard, .in_ (graveyardOf .actor)]))))
/-- "<performer> casts <subject> without paying its mana cost [if the resulting spell is …]"
(`Definition`, [CR#701.5,601.2]). -/
semantic_definition castWithoutPaying (subject : NounPhrase) (resulting : Option Predicate) :
    Action :=
  .ownScope (some (.action "cast"))
    (.procedure .cast (.firstMention "castWithoutPaying.subject" (.inCaller subject))
      .withoutPaying
      (resulting.map Predicate.inCaller)
      none none)
/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
semantic_definition discover (amount : Amount) : Action :=
  let fits : Predicate := .compare [.stat .manaValue] .atMost (.inCaller amount)
  .ownScope (some (.action "discover"))
    (sequentially
    [ exile (.run .top (and [non isLand, fits]) (libraryOf .actor)),
      .withContinuation .optional (castWithoutPaying (that .card) fits) none
        (putInto (that .card) hand),
      .move (theRest .object) .anyLocation onBottom .arrival
        [(.inOrder .randomOrder)] ])
/-- "<performer> incubates N" (`Definition`, [CR#701.53a,701.53b]). -/
semantic_definition incubate (amount : Amount) : Action :=
  .ownScope (some (.action "incubate"))
    (create 1
      (.doubleFaced
        { characteristics :=
            { types := [.artifact], subtypes := [incubator],
              text := [activated (mana [generic 2]) (transform this_)] } }
        { characteristics :=
            { name := some "Phyrexian Token", types := [.artifact, .creature],
              subtypes := [phyrexian], power := some 0, toughness := some 0 } })
      [.withCounters (.inCaller amount) p1p1Counter .fresh])
/-- "<performer> waterbends <cost>" (`Definition`, [CR#701.67a,701.67b]). -/
semantic_definition waterbend (cost : Cost) : Action :=
  .ownScope (some (.action "waterbend"))
    (.pay
      (.withSubstitute (.inCaller cost)
        (substitute (tap (a (and [isUntapped, or [isArtifact, isCreature], actorControls])))
          [.generic]))
      .once .paid none none)
/-- "<performer> manifests <subject>" (`Definition`, [CR#701.40a,708.2,708.2a]). -/
semantic_definition manifest (subject : NounPhrase) : Action :=
  .ownScope (some (.action "manifest"))
    (sequentially
    [ .turnFaceDown (.firstMention "manifest.subject" (.inCaller subject)) faceDownCreature,
      .move (.laterMention "manifest.subject" (.inCaller subject)) .anyLocation battlefield .arrival
        [(.faceDown faceDownCreature)] ])
/-- "<performer> cloaks <subject>" (`Definition`, [CR#701.58a]). -/
semantic_definition cloak (subject : NounPhrase) : Action :=
  .ownScope (some (.action "cloak"))
    (sequentially
    [ .turnFaceDown (.firstMention "cloak.subject" (.inCaller subject)) faceDownWardedCreature,
      .move (.laterMention "cloak.subject" (.inCaller subject)) .anyLocation battlefield .arrival
        [(.faceDown faceDownWardedCreature)] ])
/-- "<performer> manifests dread" (`Definition`, [CR#701.62a]). -/
semantic_definition manifestDread : Action :=
  .ownScope (some (.action "manifestDread"))
    (sequentially
    [ .expose .lookAt (.cards (.slice .top 2 (libraryOf .actor))),
      manifest (someOf (exactly 1) them),
      putInto (theRest .object) graveyard ])
/-- "support N" (`Definition`, [CR#701.41a]). -/
semantic_definition support (amount : Amount) : Action :=
  .ownScope (some (.action "support"))
    (putCounters 1 p1p1Counter
      (.eachOf
        (.described (.target (.upToOf (.inCaller amount)))
          (and [isCreature, isOther]))))

/-- "<performer> explores" (`Definition`, [CR#701.44a]). -/
semantic_definition explore : Action :=
  .ownScope (some (.action "explore" .performs))
    (instruct (controllerOf (.firstMention "explore.permanent" .actor))
      (sequentially
        [ reveal (.slice .top 1 (libraryOf .actor)),
          doIf (is (that .card) isLand)
            (putInto (that .card) (.zone .hand (ownerOf (that .card))))
            (sequentially
              [ putCounters 1 p1p1Counter
                  (.laterMention "explore.permanent" .actor),
                may (putInto (that .card) graveyard) ]) ]))
/-- "<performer> connives [N]" (`Definition`, [CR#701.50a,701.50d]): loops over
`"each nonland card discarded this way"`
(`allOf(and([not(land), card, did(any, discard(which), thisWay)]))`). -/
semantic_definition connive (amount : Amount := 1) : Action :=
  .ownScope (some (.action "connive" .performs))
    (instruct (controllerOf (.firstMention "connive.permanent" .actor))
      (sequentially
        [ draw (.inCaller amount),
          discard (counted (.exactlyOf (.inCaller amount)) .isCard),
          .doForEach (allOf (and [non isLand, .isCard,
              .did { subj := .any, action := discard .which, within := some .thisWay }]))
            (putCounters 1 p1p1Counter
              (.laterMention "connive.permanent" .actor)) ]))
/-- "Airbend <subject>" (`Definition`, [CR#701.65a]): per-card `doForEach` loop over
`allOf(and([card, did(any, exile(which), thisWay)]))` where `it` resolves by `D′` to the
singular loop referent. -/
semantic_definition airbend (subject : NounPhrase) : Action :=
  .ownScope (some (.action "airbend"))
    (sequentially
      [ exile (.firstMention "airbend.subject" (.inCaller subject)),
        .doForEach (allOf (and [.isCard, .did { subj := .any
                                                action := exile .which
                                                within := some .thisWay }]))
          (instruct (ownerOf it)
            (.establish
              (can (ownerOf it)
                (cast it (from_ := exileZone)
                  (payment := .payingInstead (.mana [generic 2]))))
              (some (.forAsLongAs (is it (.in_ exileZone)))))) ])
/-- "double <subject>'s power" (`Definition`, [CR#701.10b,701.10c]): it gets +X/+0, X its power. -/
semantic_definition doublePower (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "double"))
    (get (.firstMention "doublePower.subject" (.inCaller subject))
      (.up (powerOf (.laterMention "doublePower.subject" (.inCaller subject)))) (.up 0)
      duration)
/-- "double <subject>'s power and toughness": it gets +X/+Y, X its power and Y its toughness
(`Definition`, [CR#701.10b,701.10c]). -/
semantic_definition doublePowerAndToughness (subject : NounPhrase)
    (duration : Option Duration := none) : Action :=
  .ownScope (some (.action "double"))
    (get (.firstMention "doublePowerAndToughness.subject" (.inCaller subject))
      (.up (powerOf (.laterMention "doublePowerAndToughness.subject" (.inCaller subject))))
      (.up (toughnessOf (.laterMention "doublePowerAndToughness.subject" (.inCaller subject))))
      duration)
/-- "double <subject>'s toughness": it gets +0/+X, X its toughness (`Definition`,
[CR#701.10b,701.10c]). -/
semantic_definition doubleToughness (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "double"))
    (get (.firstMention "doubleToughness.subject" (.inCaller subject)) (.up 0)
      (.up (toughnessOf (.laterMention "doubleToughness.subject" (.inCaller subject))))
      duration)
/-- "triple <subject>'s power and toughness" (`Definition`, [CR#701.11b,701.11c]). -/
semantic_definition triplePowerAndToughness (subject : NounPhrase)
    (duration : Option Duration := none) : Action :=
  .ownScope (some (.action "triple"))
    (get (.firstMention "triplePowerAndToughness.subject" (.inCaller subject))
      (.up
        (times 2
          (powerOf (.laterMention "triplePowerAndToughness.subject" (.inCaller subject)))))
      (.up
        (times 2
          (toughnessOf (.laterMention "triplePowerAndToughness.subject" (.inCaller subject)))))
      duration)
/-- "triple <subject>'s power": it gets +X/+0, X twice its power (`Definition`,
[CR#701.11b,701.11c]). -/
semantic_definition triplePower (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "triple"))
    (get (.firstMention "triplePower.subject" (.inCaller subject))
      (.up (times 2 (powerOf (.laterMention "triplePower.subject" (.inCaller subject)))))
      (.up 0) duration)
/-- "triple <subject>'s toughness": it gets +0/+X, X twice its toughness (`Definition`,
[CR#701.11b,701.11c]). -/
semantic_definition tripleToughness (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "triple"))
    (get (.firstMention "tripleToughness.subject" (.inCaller subject)) (.up 0)
      (.up
        (times 2
          (toughnessOf (.laterMention "tripleToughness.subject" (.inCaller subject)))))
      duration)
/-- "double <performer>'s life total" (`Definition`, [CR#701.10d]), written as that total's set
[CR#119.5]. -/
semantic_definition doubleLifeTotal : Action :=
  .ownScope (some (.action "double"))
    (.changeLife (.set (times 2 (lifeTotalOf .actor))))

/-! ## Tokens -/

/-- A Food token's characteristics: "a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" [CR#111.10b] -/
semantic_definition foodToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [food],
        text :=
          [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice thisArtifact)])
              (gainLife 3) ] } }
/-- A Treasure token's characteristics: "a colorless Treasure artifact token with '{T},
Sacrifice this token: Add one mana of any color.'" [CR#111.10a] -/
semantic_definition treasureToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [treasure],
        text :=
          [ activated (.compound [.tapSymbol, .perform (sacrifice thisArtifact)])
              (.addMana 1 (.anyColor .sameColor) []) ] } }

/-- A Clue token's characteristics: "a colorless Clue artifact token with '{2}, Sacrifice this
token: Draw a card.'" [CR#111.10f] -/
semantic_definition clueToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [clue],
        text :=
          [ activated (.compound [.mana [generic 2], .perform (sacrifice thisArtifact)])
              (draw 1) ] } }

/-! ## Templates over Definitions -/

/-- "Exile <subject> until <event>." -/
semantic_template exileUntil (subject : NounPhrase) (event : GameEvent) : Action :=
  .holdUntil (exile subject) event
/-- "attach <what> to it": ordinary pronoun `it` resolved by `D′` (no `Window` override). -/
semantic_template attachToIt (what : NounPhrase) : Action :=
  attachTo what it
/-- "<subject>'s controller sacrifices it" (`Template` using `firstMention` / `laterMention`
without `Window` plumbing, [CR#701.21a]). -/
semantic_template controllerSacrifices (subject : NounPhrase) : Action :=
  instruct (.possessorOf .controller (.firstMention "controllerSacrifices.subject" subject))
    (sacrifice (.laterMention "controllerSacrifices.subject" subject))
/-- "discard <discarded>, then <step> for each nonland card discarded this way" (`Template` using
moved-parameter readback, [CR#701.50a,400.7,400.7j]). -/
semantic_template discardThenForEachNonland (discarded : NounPhrase) (step : Action) :
    Action :=
  sequentially
    [ discard discarded,
      .doForEach
        (allOf (and [non isLand, .isCard, .did { subj := .any
                                                 action := discard .which
                                                 within := some .thisWay }]))
        step ]
/-- "<subject> doesn't untap during [<whose>] untap step": the untap step's untapping is the
`untap` Deed [CR#502.3,701.26b], so this forbids the same Deed a "can't untap" pattern names. -/
semantic_template doesntUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  duringUntapStep whose (cant any (untap subject))
/-- "you may choose not to untap <subject> during [<whose>] untap step": the active player
determines which of their permanents untap [CR#502.3], so this is the prohibition `doesntUntap`
writes, holding only if you so choose. Untapping stays the default. -/
semantic_template mayDeclineUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  duringUntapStep whose (onlyIfSo (cant any (untap subject)) (.soChooses you))
/-- "untap <subject> during [<whose>] untap step" -/
semantic_template untapDuring (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  duringUntapStep whose (must any (untap subject))
/-- "<subject> may be attached to <host>": the legal-host permission over the attach deed's host
role [CR#701.3a,301.5]. -/
semantic_template mayBeAttachedTo (subject host : NounPhrase) : StaticSpec :=
  can any (attachTo subject host)
/-- "create <count> <token>" -/
semantic_template createToken (token : CharacteristicBundle) (count : Amount := 1) : Action :=
  create count (.written token)
/-- "create <count> tapped <token>" -/
semantic_template createTappedToken (token : CharacteristicBundle) (count : Amount := 1) :
    Action :=
  create count (.written token) [.entersAs .tapped]
/-- "create <count> <token> tapped and attacking" -/
semantic_template createTappedAttacking (count : Amount) (token : CharacteristicBundle) : Action :=
  create count (.written token) [.entersAs .tapped, .entersAttacking none]
/-- "create a token that's a copy of <source> [riders]" -/
semantic_template createTokenCopy (source : NounPhrase) (riders : List MoveRider := []) : Action :=
  create 1 (.copyOf source []) riders


/-- A resolving regeneration action installs a single-use shield for this turn;
only its eventual application carries the Regenerate label (`Definition`, [CR#701.19a,701.19c]). -/
semantic_definition regenerate (subject : NounPhrase) : Action :=
  .ownScope none
    (.establish (.replace
      (.would .any (destroy (.firstMention "regenerate.subject" (.inCaller subject))))
      (instead (regeneration (.laterMention "regenerate.subject" (.inCaller subject))))
      none .nextTimeOnly none) thisTurn)

/-! ## Keyword actions -/

/-- "adapt N" with its reminder text (`Definition`, [CR#701.46a]): an instruction about "this
permanent" (`DeedPermanent.isAbout`), the Source unless a handoff names another ("target creature
adapts"), that the ability's controller follows [CR#608.2c]; the body reads that permanent as
`.actor`. -/
semantic_definition adapt (amount : Amount) : Action :=
  .ownScope (some (.action "adapt" .isAbout))
    (doIf (not (is .actor (.hasCounters p1p1Counter)))
      (putCounters (.inCaller amount) p1p1Counter .actor)
      none)
semantic_definition behold (quality : Predicate) : Action :=
  .ownScope (some (.action "behold"))
    (chooseOne
      (.expose .reveal (.cards (a (and [.isCard, .inCaller quality, .in_ (handOf .actor)]))))
      (choose (a (and [.inCaller quality, isPermanent, actorControls]))))
semantic_definition blight (amount : Amount) : Action :=
  .ownScope (some (.action "blight"))
    (putCounters (.inCaller amount) m1m1Counter
      (a (and [isCreature, actorControls])))
semantic_definition bolster (amount : Amount) : Action :=
  .ownScope (some (.action "bolster"))
    (sequentially
      [ choose
          (a
            (and
              [ isCreature,
                actorControls,
                .superlative .min (.stat .toughness) (and [isCreature, actorControls]) ])),
        putCounters (.inCaller amount) p1p1Counter (that creature) ])
semantic_definition clash : Action :=
  .ownScope (some (.action "clash"))
    (sequentially
      [ .expose .reveal (.cards (.slice .top 1 (libraryOf .actor))),
        may (putInto (that .card) onBottom) ])
/-- "convert <subject>": turn it so that its other face is up, under the transform rules
[CR#701.28a,701.27a..701.27f]. Converting is transforming written with another verb: a "transforms into" trigger
sees either [CR#701.27e], and "can't transform" forbids either [CR#701.28f], so the word is this
Template's and the tree is `transform`'s. -/
semantic_template convert (subject : NounPhrase) : Action := transform subject
/-- "detain <subject>": until the next turn of the controller of the detaining spell or ability,
"your next turn" [CR#701.35a,109.5], which is not its source's controller once the source has
changed control [CR#113.8]. -/
semantic_definition detain (subject : NounPhrase) : Action :=
  .ownScope (some (.action "detain"))
    (establishThroughout
      (.conjunction none
        [ cant (actor (.firstMention "detain.subject" (.inCaller subject))) (.attack none),
          cant (actor (.laterMention "detain.subject" (.inCaller subject))) (.block .any),
          cant .any
            (activate (allOf
                (and
                  [ .abilityHead .anyActivated,
                    .abilityOf (.laterMention "detain.subject" (.inCaller subject)) ]))) ])
      untilYourNextTurn)
semantic_definition earthbend (amount : Amount) : Action :=
  .ownScope (some (.action "earthbend"))
    (sequentially
      [ .establish
          (.characteristicChange (target (and [isLand, actorControls]))
            [ .typeLine .adds { types := some [.creature] },
              .stat .sets .power 0,
              .stat .sets .toughness 0,
              .addedAbilities [keyword (.named "haste")] ])
          none,
        putCounters (.inCaller amount) p1p1Counter (that land),
        delay (does (that land) die)
          (returnTo (that land) battlefield [.entersAs .tapped, .under .actor])
          [does .any (.move (that land) battlefield exileZone .departure [])] ])
semantic_definition endure (amount : Amount) : Action :=
  .ownScope (some (.action "endure" .performs))
    (instruct (controllerOf (.firstMention "endure.permanent" .actor))
      (chooseOne
        (putCounters (.inCaller amount) p1p1Counter (.laterMention "endure.permanent" .actor))
        (createToken (creatureTokenOf (.inCaller amount) (.inCaller amount) [.white] [spirit]))))
semantic_definition exert (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exert"))
    (.skipUntap (.firstMention "exert.subject" (.inCaller subject)) 1)
semantic_definition faceAVillainousChoice (first second : Action) : Action :=
  .ownScope (some (.action "faceAVillainousChoice"))
    (chooseOne (.inCaller first) (.inCaller second))
semantic_definition forage : Action :=
  .ownScope (some (.action "forage"))
    (chooseOne
      (exileFrom (counted (exactly 3) (cardIn (graveyardOf .actor))) (graveyardOf .actor))
      (sacrifice (a (and [.hasSubtype food, isPermanent, actorControls]))))
/-- "goad <subject>": goaded until the next turn of the controller of the goading spell or
ability, "your next turn" [CR#701.15a,109.5], as detain's. -/
semantic_definition goad (subject : NounPhrase) : Action :=
  .ownScope (some (.action "goad"))
    (gainDesignation (.firstMention "goad.subject" (.inCaller subject)) goaded
      untilYourNextTurn)
semantic_definition harness (subject : NounPhrase) : Action :=
  .ownScope (some (.action "harness"))
    (doIf
      (not
        (is (.firstMention "harness.subject" (.inCaller subject))
          (hasDesignation harnessed)))
      (gainDesignation (.laterMention "harness.subject" (.inCaller subject)) harnessed)
      none)
semantic_definition heal (subject : NounPhrase) : Action :=
  .ownScope (some (.action "heal"))
    (clearDamage (.firstMention "heal.subject" (.inCaller subject)))
semantic_definition investigate : Action :=
  .ownScope (some (.action "investigate")) (createToken clueToken)
semantic_definition populate : Action :=
  .ownScope (some (.action "populate"))
    (sequentially
      [ choose (a (and [.isToken, isCreature, isPermanent, actorControls])),
        createTokenCopy thatToken ])
semantic_definition recruit : Action :=
  .ownScope (some (.action "recruit"))
    (sequentially
      [ draw 1,
        discard (a (cardIn (handOf .actor))),
        doIf (not (is (the (and [isCard, was (discard which) thisWay])) isLand))
          (createToken (creatureToken 1 1 [.white] [human, soldier]))
          none ])
/-- "search <scope>", or "search <scope> for <for_>" (`Definition`, [CR#701.23a]). -/
semantic_definition search (scope : Location) (for_ : NounPhrase := any) : Action :=
  .ownScope (some (.action "search"))
    (.search (.inCaller scope) (.inCaller for_))
/-- "search your library and/or graveyard for <for_>", "your" written once. -/
semantic_template searchLibraryOrGraveyard (for_ : NounPhrase) : Action :=
  search (andOr [yourLibrary, yourGraveyard]) for_
semantic_definition suspect (subject : NounPhrase) : Action :=
  .ownScope (some (.action "suspect"))
    (gainDesignation (.firstMention "suspect.subject" (.inCaller subject)) suspected)
/-- "The Ring tempts you": the Ring emblem if you have none, then you choose a creature you
control as your Ring-bearer (`Definition`, [CR#701.54a,701.54c]). The emblem's abilities count
the times the Ring has tempted you by the deed's own steps, `tempting`. -/
semantic_definition theRingTemptsYou : Action :=
  let ringBearers : Predicate := and [hasDesignation ringBearer, youControl]
  let tempting : List Action :=
    [ choose (a (and [isCreature, actorControls])), gainDesignation (that creature) ringBearer ]
  let timesTempted : Amount :=
    eventCount
      (.ownScope (some (.action "theRingTemptsYou"))
        (sequentially tempting))
      you thisGame
  .ownScope (some (.action "theRingTemptsYou"))
    (sequentially
      (doIf (not (exists_ (and [isEmblem, named "The Ring", ownedBy theActor])))
          (getNamedEmblem "The Ring"
            [ static
                (characteristicChange (allOf ringBearers)
                  [.typeLine .adds { supertypes := some [.legendary] }]),
              static
                (cant
                  (by_
                    (allOf
                      (and
                        [ isCreature,
                          hasPower .greater
                            (powerOf
                              (a (and [isCreature, hasDesignation ringBearer, youControl]))) ])))
                  (block (allOf (and [isCreature, hasDesignation ringBearer, youControl])))),
              static
                (onlyWhile
                  (abilityGrant this_
                    (whenever (does (a ringBearers) attack)
                      (sequentially [draw 1, discard (a (cardIn hand))])))
                  (compareAmt timesTempted .atLeast 2)),
              static
                (onlyWhile
                  (abilityGrant this_
                    (whenever
                      (does (by_ (a isCreature)) (block (a ringBearers)))
                      (atNext .endOfCombat
                        (instruct (controllerOf (that creature)) (sacrifice (that creature))))))
                  (compareAmt timesTempted .atLeast 3)),
              static
                (onlyWhile
                  (abilityGrant this_
                    (whenever
                      (does (a ringBearers)
                        (dealDamage (to := a isPlayer) (kind := some .combat)))
                      (instruct eachOpponent (loseLife 3))))
                  (compareAmt timesTempted .atLeast 4)) ])
          :: tempting))
/-- "empower Jace N" (`Definition`, [CR#701.71a]): "If you don't control a Jace planeswalker token,
create a blue Jace planeswalker token with 0 loyalty, '[−1]: Surveil 1,' and '[−3]: Draw a card.'
Choose a Jace planeswalker token you control. Put N loyalty counters on it." -/
semantic_definition empowerJace (amount : Amount) : Action :=
  let jaceToken : Predicate := and [.isToken, isPlaneswalker, .hasSubtype jace, actorControls]
  .ownScope (some (.action "empowerJace"))
    (sequentially
      [ doIf (not (exists_ jaceToken))
          (createToken
            { characteristics :=
                { colors := [.blue], types := [.planeswalker], subtypes := [jace],
                  loyalty := some 0,
                  text :=
                    [ activated (loyaltySymbol (.down 1)) (surveil 1),
                      activated (loyaltySymbol (.down 3)) (draw 1) ] } })
          none,
        choose (a jaceToken),
        putCounters (.inCaller amount) loyaltyCounter it ])
/-- "learn" (`Definition`, [CR#701.48a]): "You may discard a card. If you do, draw a card. If you
didn't discard a card, you may reveal a Lesson card you own from outside the game and put it into
your hand." The second condition is the discard's result, not the choice: a player who chose to
discard with no card in hand discarded none. -/
semantic_definition learn : Action :=
  .ownScope (some (.action "learn"))
    (sequentially
      [ may (discard (a .isCard)) (some (draw 1)),
        doIf (not (exists_ (and [.isCard, was (discard which) thisWay])))
          (may
            (sequentially
              [ reveal (a (and [.isCard, .hasSubtype lesson, ownedBy .actor, outsideTheGame])),
                putInto it (handOf .actor) ]))
          none ])
/-- "time travel" [CR#701.56a]: any number of permanents the performer controls with time counters
and/or suspended cards they own in exile with time counters, a suspended card being one in exile
that has suspend and a time counter [CR#702.62b]; for each, put or remove a time counter
(`Definition`). -/
semantic_definition timeTravel : Action :=
  .ownScope (some (.action "timeTravel"))
    (sequentially
      [ choose
          (counted anyNumber
            (andOr
              [ and [isPermanent, actorControls, .hasCounters timeCounter],
                and
                  [ .isCard, .in_ exileZone, ownedBy .actor,
                    .hasKeyword (.the (.named "suspend")), .hasCounters timeCounter ] ])),
        .doForEach them
          (chooseOne (putCounters 1 timeCounter it)
            (removeCounters (oneThrough 1) timeCounter it)) ])

end Semantics.Definitions
