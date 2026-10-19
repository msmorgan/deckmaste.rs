import Semantics.Templates

/-!
# Semantics.Definitions.Actions

Rules-defined actions spelled over `Semantics.Templates`: turn parts, attaching, creating and
countering, the other actions, the predefined tokens, the phrasing templates that need these
actions, and the keyword actions.
-/

namespace Semantics.Definitions

open Semantics.Templates

/-! ## Turn parts -/

semantic_definition beginningOfCombatStep : TurnPart := .beginningOfCombat
semantic_definition beginningPhase : TurnPart := .beginningPhase
semantic_definition cleanup : TurnPart := .cleanup
semantic_definition cleanupStep : TurnPart := .cleanup
semantic_definition TurnPart.combat : TurnPart := .combat
semantic_definition combatDamageStep : TurnPart := .combatDamage
semantic_definition combatPhase : TurnPart := .combat
semantic_definition declareAttackersStep : TurnPart := .declareAttackers
semantic_definition declareBlockersStep : TurnPart := .declareBlockers
semantic_definition drawStep : TurnPart := .drawStep
semantic_definition endOfCombatStep : TurnPart := .endOfCombat
semantic_definition endStep : TurnPart := .endStep
semantic_definition endingPhase : TurnPart := .endingPhase
semantic_definition mainPhase : TurnPart := .mainPhase
semantic_definition postcombatMainPhase : TurnPart := .postcombatMain
semantic_definition precombatMainPhase : TurnPart := .firstMain
semantic_definition untapStep : TurnPart := .untapStep
semantic_definition upkeep : TurnPart := .upkeep
semantic_definition upkeepStep : TurnPart := .upkeep

/-! ## Attach, create, and counter -/

/-- "attach <subject> to <host>" (`Definition`, [CR#701.3a]). -/
semantic_definition attachTo (subject host : NounPhrase) : Action :=
  .ownScope (some (.action "attach"))
    (.enact (.action "attach")
      (.attachment .attached (.inCaller (.firstMention "attachTo.subject" subject))
        (some (.inCaller (.firstMention "attachTo.host" host))))
      none none)
/-- "create <count> <token> [riders]": the tokens are put onto the battlefield, entering under
their creator's control (`Definition`, [CR#701.7a,111.2]). -/
semantic_definition create (count : Amount) (token : TokenSpec) (riders : List MoveRider := []) :
    Action :=
  .ownScope (some (.action "create"))
    (.enact (.action "create")
      (.createObject (.inCaller count) (.token (.inCaller token) (riders.map MoveRider.inCaller))
        true none)
      none none)
/-- "counter <subject>": one labeled stack-to-graveyard move (`Definition`, [CR#701.6a]). -/
semantic_definition counter (subject : NounPhrase) : Action :=
  .ownScope (some (.action "counter"))
    (.enact (.action "counter")
      (.move (.inCaller (.firstMention "counter.subject" subject)) (.zone .stack none)
        (some (.zone .graveyard none))
        .arrival none [])
      none none)

/-! ## Actions -/

/-- "cast <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.5a]). -/
semantic_definition cast (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action :=
  .ownScope (some (.action "cast"))
    (.procedure .cast (.inCaller subject) payment.inCaller (resulting.map .inCaller)
      (from_.map .inCaller) none)
/-- "play <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.18a]). -/
semantic_definition play (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action :=
  .ownScope (some (.action "play"))
    (.procedure .play (.inCaller subject) payment.inCaller (resulting.map .inCaller)
      (from_.map .inCaller) none)
/-- "activate <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.2a]). -/
semantic_definition activate (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action :=
  .ownScope (some (.action "activate"))
    (.procedure .activate (.inCaller subject) payment.inCaller (resulting.map .inCaller)
      (from_.map .inCaller) none)
/-- "the regeneration of <subject>": the deed "can't be regenerated" forbids, a shield's
application rather than the action that creates one (`Definition`, [CR#701.19c]). -/
semantic_definition regeneration (subject : NounPhrase) : Action :=
  .ownScope (some (.action "regenerate"))
    (.enact (.action "regenerate") (.clearDamage (.inCaller subject)) none none)
/-- "destroy <subject>" (`Definition`, [CR#701.8]). -/
semantic_definition destroy (subject : NounPhrase) : Action :=
  .ownScope (some (.action "destroy"))
    (.enact (.action "destroy")
      (.move (.inCaller (.firstMention "destroy.subject" subject)) (.zone .battlefield none)
        (some graveyard) .departure none [])
      none none)
/-- "Destroy <subject>. It can't be regenerated." / "… They can't be regenerated.": the
regeneration the prohibition forbids is of the object or objects just destroyed [CR#701.19c]. -/
semantic_template destroyCantRegenerate (subject : NounPhrase) : Action :=
  doAndForbid (destroy subject) (regeneration (.pro .bare subject.plur))
/-- "exile <subject>" (`Definition`, [CR#701.13]). -/
semantic_definition exile (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exile.subject" subject)) .anyLocation (some exileZone)
        .arrival none [])
      none none)
/-- "exile <subject> face down" (`Definition`, [CR#701.13,406.3]). -/
semantic_definition exileFaceDown (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exileFaceDown.subject" subject)) .anyLocation
        (some exileZone) .arrival none
        [.faceDown exiledFaceDown])
      none none)
/-- "exile <subject> with N <kind> counters on it" (`Definition`, [CR#701.13]). -/
semantic_definition exileWithCounters (subject : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exileWithCounters.subject" subject)) .anyLocation
        (some exileZone) .arrival none
        [.withCounters (.inCaller amount) (.printed kind) .fresh])
      none none)
/-- "sacrifice <subject>" (`Definition`, [CR#701.21]). -/
semantic_definition sacrifice (subject : NounPhrase) : Action :=
  .ownScope (some (.action "sacrifice"))
    (.enact (.action "sacrifice")
      (.move (.inCaller (.firstMention "sacrifice.subject" subject)) (.zone .battlefield none)
        (some graveyard) .departure none [])
      none none)
/-- "transform <subject>" (`Definition`, [CR#701.27]). -/
semantic_definition transform (subject : NounPhrase) : Action :=
  .ownScope (some (.action "transform"))
    (.enact (.action "transform")
      (.turnOver (.inCaller (.firstMention "transform.subject" subject)))
      none none)
/-- "meld <subject> into <name>" (`Definition`, [CR#701.42]). -/
semantic_definition meldInto (subject : NounPhrase) (into : String) : Action :=
  .ownScope (some (.action "meld"))
    (.enact (.action "meld")
      (.move (.inCaller (.firstMention "meldInto.subject" subject)) .anyLocation (some battlefield)
        .arrival none [.entersMelded into])
      none none)
/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a]. -/
semantic_definition manifestPlacement (subject : NounPhrase) : Action :=
  .ownScope (some (.action "manifest"))
    (.enact (.action "manifest")
      (.move (.inCaller (.firstMention "manifestPlacement.subject" subject)) .anyLocation
        (some battlefield) .arrival none [])
      none none)
/-- "tap <subject>" (`Definition`, [CR#701.26a]). -/
semantic_definition tap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "tap"))
    (.enact (.action "tap") (.setStatus .tapped (.inCaller (.firstMention "tap.subject" subject)))
      none none)
/-- "<performer> discards <subject>": a move from the performer's own hand, the card's owner's,
to the graveyard (`Definition`, [CR#701.9a]). -/
semantic_definition discard (subject : NounPhrase) : Action :=
  .ownScope (some (.action "discard"))
    (.enact (.action "discard")
      (.move (.inCaller (.firstMention "discard.subject" subject)) (.zone .hand (some .actor))
        (some graveyard) .arrival none [])
      none none)
/-- A bare "shuffle": the performer's own library (`Definition`, [CR#701.24a]). -/
semantic_definition shuffle : Action :=
  .ownScope (some (.action "shuffle"))
    (.enact (.action "shuffle") (.shuffle (libraryOf .actor)) none none)
/-- "untap <subject>" (`Definition`, [CR#701.26b]). -/
semantic_definition untap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "untap"))
    (.enact (.action "untap")
      (.setStatus .untapped (.inCaller (.firstMention "untap.subject" subject)))
      none none)
/-- "<performer> mills <amount> cards": from the performer's own library (`Definition`,
[CR#701.17a]). -/
semantic_definition mill (amount : Amount) : Action :=
  .ownScope (some (.action "mill"))
    (.enact (.action "mill")
      (.move (.slice .top (.inCaller amount) (libraryOf .actor))
        (.zone .library none) (some graveyard) .arrival none [])
      none none)
/-- "<cards> are milled": the mill whose patient is a card description, as an event a trigger
watches ("Whenever one or more nonland cards are milled") (`Definition`, [CR#701.17a,701.17c]). -/
semantic_definition millCards (cards : NounPhrase) : Action :=
  .ownScope (some (.action "mill"))
    (.enact (.action "mill")
      (.move (.inCaller (.firstMention "millCards.cards" cards))
        library (some graveyard) .arrival none [])
      none none)
/-- "flip <subject>": a flip card's permanent turns to its flipped half, whose alternative name,
text box, type line, power and toughness apply (`Definition`, [CR#710.1a,710.2]). Flipping is
not a keyword action, so its scope names no deed. -/
semantic_definition flip (subject : NounPhrase := .actor) : Action :=
  .ownScope none (.setStatus .flipped (.inCaller (.firstMention "flip.subject" subject)))
/-- "Search your library for <quantity> <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchLibraryFor (quantity : Quantity) (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search") (.search (yourLibrary) (.inCaller quantity) (.inCaller p)) none none)
/-- "search <whose>'s graveyard, hand, and library for <q> <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchZonesOf (whose : NounPhrase) (quantity : Quantity) (p : Predicate) :
    Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search")
      (.search
        (or
          [ .zone .graveyard (some (.inCaller (.firstMention "searchZonesOf.whose" whose))),
            .zone .hand (some (.laterMention "searchZonesOf.whose" (.inCaller whose))),
            .zone .library (some (.laterMention "searchZonesOf.whose" (.inCaller whose))) ])
        (.inCaller quantity) (.inCaller p))
      none none)
/-- "search your library and/or graveyard for a <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchLibraryOrGraveyard (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search")
      (.search (or [.zone .library (some .you), .zone .graveyard (some .you)]) (exactly 1)
        (.inCaller p))
      none none)
/-- "<creature> fights <creature>" (`Definition`, [CR#701.14a]). -/
semantic_definition fight (other : NounPhrase) : Action :=
  .ownScope (some (.action "fight"))
    (.enact (.action "fight")
      (doIf (and [
        matches_ (.firstMention "fight.0" (.inCaller other)) isCreature,
        matches_ (.firstMention "fight.source" .actor) isCreature])
        (.simultaneously [
          .dealDamage (.statOf (.stat .power) (.laterMention "fight.source" .actor))
            (.inCaller (.laterMention "fight.0" other)) .any,
          instruct (.laterMention "fight.0" (.inCaller other))
            (.dealDamage (.statOf (.stat .power) (.laterMention "fight.0" (.inCaller other)))
              (.laterMention "fight.source" .actor) .any)])
        none)
      none none)
/-- "reveal <cards>" (`Definition`, [CR#701.20]). -/
semantic_definition reveal (cards : NounPhrase) : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal")
      (.expose .reveal (.cards (.inCaller (.firstMention "reveal.cards" cards))))
      none none)
/-- "shuffle <subject> into its owner's library": the move, then the shuffle of that library,
which happens even if nothing moved (`Definition`, [CR#701.24c,701.24d]). -/
semantic_definition shuffleInto (subject : NounPhrase) : Action :=
  .ownScope (some (.action "shuffle"))
    (sequentially
      [ .enact (.action "shuffle")
          (.move (.inCaller (.firstMention "shuffleInto.subject" subject)) .anyLocation
            (some (.zone .library none))
            .arrival none [])
          none none,
        .shuffle (.zone .library none) ])
/-- "vote for <ballot> [<times> times]" (`Definition`, [CR#701.38]). -/
semantic_definition vote (disclosure : Disclosure := .openly)
    (ballot : Ballot := anyBallot)
    (times : Option Quantity := none) : Action :=
  .ownScope (some (.action "vote"))
    (.enact (.action "vote") (.vote none disclosure ballot times) none none)
/-- "Starting with <first>, vote for <ballot>" (`Definition`, [CR#701.38]). -/
semantic_definition voteStartingWith (first : NounPhrase) (disclosure : Disclosure)
    (ballot : Ballot) : Action :=
  .ownScope (some (.action "vote"))
    (.enact (.action "vote")
      (.vote (some (.inCaller (.firstMention "voteStartingWith.first" first))) disclosure ballot
        none)
      none none)
/-- "scry N" (`Definition`, [CR#701.22a]): the performer's own library. -/
semantic_definition scry (amount : Amount) : Action :=
  .ownScope (some (.action "scry"))
    (.enact (.action "scry")
      (lookAndSortSlice (.slice .top (.inCaller amount) (libraryOf .actor)) onBottom
        [(.inOrder .anyOrder)])
      none none)
/-- "fateseal N" (`Definition`, [CR#701.29a]): the top N cards of an opponent's library, an
opponent of the performer. -/
semantic_definition fateseal (amount : Amount) : Action :=
  .ownScope (some (.action "fateseal"))
    (.enact (.action "fateseal")
      (lookAndSortSlice (.slice .top (.inCaller amount) (libraryOf (a (isOpponentOf .actor))))
        onBottom [(.inOrder .anyOrder)])
      none none)
/-- "surveil N" (`Definition`, [CR#701.25a]): the performer's own library. -/
semantic_definition surveil (amount : Amount) : Action :=
  .ownScope (some (.action "surveil"))
    (.enact (.action "surveil")
      (lookAndSortSlice (.slice .top (.inCaller amount) (libraryOf .actor)) graveyard)
      none none)
/-- "Proliferate" with its reminder text (`Definition`, [CR#701.34a]). -/
semantic_definition proliferate : Action :=
  .ownScope (some (.action "proliferate"))
    (.enact (.action "proliferate") (sequentially
      [ .choose none (counted anyNumber
            (or [ and [isPermanent, .hasCounters none],
                   .compare [.anyCounter .player] .atLeast (.lit 1) ])) .openly none,
        .putCounters (.lit 1) .own (.eachOf (those .join)) .one true ])
      none none)
/-- "<performer> amasses <subtype> N" with its reminder text (`Definition`, [CR#701.47a]): the
chosen Army is `that (.type .creature)` and `it`, resolved inside `ownScope`. -/
semantic_definition amass (subtype : Subtype) (count : Amount) : Action :=
  .ownScope (some (.action "amass"))
    (.enact (.action "amass") (sequentially
      [ doIf (not (exists_ army))
          (create (.lit 1) (.written (creatureToken 0 0 [.black] [subtype, Subtype.army])))
          none,
        choose (a army),
        .putCounters (.inCaller count) (.printed p1p1Counter) (that (.type .creature)) .one true,
        doIf (not (matches_ it (.hasSubtype subtype)))
          (become it { characteristics := { subtypes := [subtype] } })
          none ])
      none none)
/-- "monstrosity N" with its reminder text (`Definition`, [CR#701.37a]). -/
semantic_definition monstrosity (amount : Amount) : Action :=
  .ownScope (some (.action "monstrosity"))
    (.enact (.action "monstrosity")
      (doIf (not (matches_ .actor (.hasDesignation monstrous none)))
        (sequentially
          [ .putCounters (.inCaller amount) (.printed p1p1Counter) .actor .one true,
            .gainDesignation .actor monstrous none ])
        none)
      none none)
/-- "double the number of each kind of counter on <on>" (`Definition`, [CR#701.10e]): for each
kind of counter on it, put as many of that kind on it as it already has. -/
semantic_definition doubleCounters (on : NounPhrase) : Action :=
  .ownScope (some (.action "double"))
    (.doForEachKind .counterKind (some (.firstMention "doubleCounters.on" (.inCaller on)))
      .counterKind
      (.putCounters (.statOf .boundCounter (.laterMention "doubleCounters.on" (.inCaller on)))
        .bound (.laterMention "doubleCounters.on" (.inCaller on)) .one true))
semantic_definition exchangeCards (left right : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange")
      (.exchange
        (.cardsAcross (.inCaller (.firstMention "exchangeCards.left" left))
          (.inCaller (.firstMention "exchangeCards.right" right)))))
semantic_definition exchangeControl (left right : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange")
      (.exchange
        (.controlOf (.inCaller (.firstMention "exchangeControl.left" left))
          (.inCaller (.firstMention "exchangeControl.right" right)))))
semantic_definition exchangeLifeTotals (parties : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange")
      (.exchange (.lifeTotals (.inCaller (.firstMention "exchangeLifeTotals.parties" parties)))))
semantic_definition exchangeTextBoxes (left right : NounPhrase) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange")
      (.establish
        (.textChange (.inCaller (.firstMention "exchangeTextBoxes.left" left))
          (.exchangeWith (.inCaller (.firstMention "exchangeTextBoxes.right" right))))
        none))
semantic_definition exchangeValues (left right : Amount) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange") (.exchange (.values (.inCaller left) (.inCaller right))))
semantic_definition exchangeZones (left right : Location) : Action :=
  .ownScope (some (.action "exchange"))
    (enact (.action "exchange") (.exchange (.zones (.inCaller left) (.inCaller right))))
semantic_definition exileFrom (subject : NounPhrase) (from_ : Location) : Action :=
  .ownScope (some (.action "exile"))
    (enact (.action "exile")
      (.move (.inCaller (.firstMention "exileFrom.subject" subject)) (.inCaller from_)
        (some exileZone) .arrival none []))
/-- "<performer> reveals their hand" (`Definition`, [CR#701.20]). -/
semantic_definition revealHand : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal") (.expose .reveal (.zone (handOf .actor))) none none)
/-- "<performer> collects evidence N" (`Definition`, [CR#701.59a]). -/
semantic_definition collectEvidence (amount : Amount) : Action :=
  .ownScope (some (.action "collectEvidence"))
    (.enact (.action "collectEvidence")
      (exile
        (withTotal (.stat .manaValue) .atLeast (.inCaller amount)
          (allOf (and [.isCard, .in_ (graveyardOf .actor)]))))
      none none)
/-- "<performer> casts <subject> without paying its mana cost [if the resulting spell is …]"
(`Definition`, [CR#701.5,601.2]). -/
semantic_definition castWithoutPaying (subject : NounPhrase) (resulting : Option Predicate) :
    Action :=
  .ownScope (some (.action "cast"))
    (.enact (.action "cast")
      (.procedure .cast (.inCaller (.firstMention "castWithoutPaying.subject" subject))
        .withoutPaying
        (resulting.map Predicate.inCaller)
        none none)
      none none)
/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
semantic_definition discover (amount : Amount) : Action :=
  let fits : Predicate := .compare [.stat .manaValue] .atMost (.inCaller amount)
  .ownScope (some (.action "discover"))
    (.enact (.action "discover") (sequentially
      [ exile (.run .top (and [non isLand, fits]) (libraryOf .actor)),
        .withContinuation .optional (castWithoutPaying (that .card) (some fits)) none
          (some (putInto (that .card) hand)),
        .move (theRest .object) .anyLocation (some onBottom) .arrival none
          [(.inOrder .randomOrder)] ])
      none none)
/-- "<performer> incubates N" (`Definition`, [CR#701.53a,701.53b]). -/
semantic_definition incubate (amount : Amount) : Action :=
  .ownScope (some (.action "incubate"))
    (enact (.action "incubate")
      (create (.lit 1)
        (.doubleFaced
          { characteristics :=
              { types := [.artifact], subtypes := [incubator],
                text := [activated (mana [generic 2]) (transform this_)] } }
          { characteristics :=
              { name := some "Phyrexian Token", types := [.artifact, .creature],
                subtypes := [phyrexian], power := some (.lit 0), toughness := some (.lit 0) } })
        [.withCounters (.inCaller amount) (.printed p1p1Counter) .fresh]))
/-- "<performer> waterbends <cost>" (`Definition`, [CR#701.67a,701.67b]). -/
semantic_definition waterbend (cost : Cost) : Action :=
  .ownScope (some (.action "waterbend"))
    (.enact (.action "waterbend")
      (.pay
        (.withSubstitute (.inCaller cost)
          (substitute (tap (a (and [isUntapped, or [isArtifact, isCreature], actorControls])))
            [.generic]))
        .once .paid none none)
      none none)
/-- "<performer> manifests <subject>" (`Definition`, [CR#701.40a,708.2,708.2a]). -/
semantic_definition manifest (subject : NounPhrase) : Action :=
  .ownScope (some (.action "manifest"))
    (.enact (.action "manifest") (sequentially
      [ .turnFaceDown (.inCaller (.firstMention "manifest.subject" subject)) faceDownCreature,
        .move (that .card) .anyLocation (some battlefield) .arrival none
          [(.faceDown faceDownCreature)] ])
      none none)
/-- "<performer> cloaks <subject>" (`Definition`, [CR#701.58a]). -/
semantic_definition cloak (subject : NounPhrase) : Action :=
  .ownScope (some (.action "cloak"))
    (.enact (.action "cloak") (sequentially
      [ .turnFaceDown (.inCaller (.firstMention "cloak.subject" subject)) faceDownWardedCreature,
        .move (that .card) .anyLocation (some battlefield) .arrival none
          [(.faceDown faceDownWardedCreature)] ])
      none none)
/-- "<performer> manifests dread" (`Definition`, [CR#701.62a]). -/
semantic_definition manifestDread : Action :=
  .ownScope (some (.action "manifestDread"))
    (.enact (.action "manifestDread") (sequentially
      [ .expose .lookAt (.cards (.slice .top (.lit 2) (libraryOf .actor))),
        manifest (someOf (exactly 1) them),
        putInto (theRest .object) graveyard ])
      none none)
/-- "support N" (`Definition`, [CR#701.41a]). -/
semantic_definition support (amount : Amount) : Action :=
  .ownScope (some (.action "support"))
    (.enact (.action "support")
      (.putCounters (.lit 1) (.printed p1p1Counter)
        (.eachOf
          (.described (.target (.upToOf (.inCaller amount))) (and [isCreature, .otherThan this_])))
        .one true)
      none none)

/-- "<performer> explores" (`Definition`, [CR#701.44a]). -/
semantic_definition explore : Action :=
  .ownScope (some (.action "explore"))
    (.enact (.action "explore")
      (instruct (controllerOf (.firstMention "explore.permanent" .actor))
        (sequentially
          [ reveal (.slice .top (.lit 1) (libraryOf .actor)),
            doIf (matches_ (that .card) isLand)
              (putInto (that .card) (.zone .hand (some (ownerOf (that .card)))))
              (some (sequentially
                [ .putCounters (.lit 1) (.printed p1p1Counter)
                    (.laterMention "explore.permanent" .actor) .one true,
                  may (putInto (that .card) graveyard) ])) ]))
      none none)
/-- "<performer> connives [N]" (`Definition`, [CR#701.50a,701.50d]): loops over
`"each nonland card discarded this way"`
(`allOf(and([not(land), card, did(any, discard(which), thisWay)]))`). -/
semantic_definition connive (amount : Amount := .lit 1) : Action :=
  .ownScope (some (.action "connive"))
    (.enact (.action "connive")
      (instruct (controllerOf (.firstMention "connive.permanent" .actor))
        (sequentially
          [ .draw (.inCaller amount),
            discard (counted (.exactlyOf (.inCaller amount)) .isCard),
            .doForEach (allOf (and [non isLand, .isCard,
                .did ⟨.any, discard .which, .thisWay⟩]))
              (.putCounters (.lit 1) (.printed p1p1Counter)
                (.laterMention "connive.permanent" .actor) .one true) ]))
      none none)
/-- "Airbend <subject>" (`Definition`, [CR#701.65a]): per-card `doForEach` loop over
`allOf(and([card, did(any, exile(which), thisWay)]))` where `it` resolves by `D′` to the
singular loop referent. -/
semantic_definition airbend (subject : NounPhrase) : Action :=
  .ownScope (some (.action "airbend"))
    (.enact (.action "airbend")
      (sequentially
        [ exile (.inCaller (.firstMention "airbend.subject" subject)),
          .doForEach (allOf (and [.isCard, .did ⟨.any, exile .which, .thisWay⟩]))
            (instruct (ownerOf it)
              (.establish
                (can (ownerOf it)
                  (cast it (from_ := some exileZone)
                    (payment := .payingInstead (.mana [generic 2]))))
                (some (.forAsLongAs (matches_ it (.in_ exileZone)))))) ])
      none none)
/-- "double <subject>'s power" (`Definition`, [CR#701.10b,701.10c]): it gets +X/+0, X its power. -/
semantic_definition doublePower (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "double"))
    (get (.firstMention "doublePower.subject" (.inCaller subject))
      (.up (powerOf (.laterMention "doublePower.subject" (.inCaller subject)))) (.up (.lit 0))
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
    (get (.firstMention "doubleToughness.subject" (.inCaller subject)) (.up (.lit 0))
      (.up (toughnessOf (.laterMention "doubleToughness.subject" (.inCaller subject))))
      duration)
/-- "triple <subject>'s power and toughness" (`Definition`, [CR#701.11b,701.11c]). -/
semantic_definition triplePowerAndToughness (subject : NounPhrase)
    (duration : Option Duration := none) : Action :=
  .ownScope (some (.action "triple"))
    (get (.firstMention "triplePowerAndToughness.subject" (.inCaller subject))
      (.up
        (times (.lit 2)
          (powerOf (.laterMention "triplePowerAndToughness.subject" (.inCaller subject)))))
      (.up
        (times (.lit 2)
          (toughnessOf (.laterMention "triplePowerAndToughness.subject" (.inCaller subject)))))
      duration)
/-- "triple <subject>'s power": it gets +X/+0, X twice its power (`Definition`,
[CR#701.11b,701.11c]). -/
semantic_definition triplePower (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "triple"))
    (get (.firstMention "triplePower.subject" (.inCaller subject))
      (.up (times (.lit 2) (powerOf (.laterMention "triplePower.subject" (.inCaller subject)))))
      (.up (.lit 0)) duration)
/-- "triple <subject>'s toughness": it gets +0/+X, X twice its toughness (`Definition`,
[CR#701.11b,701.11c]). -/
semantic_definition tripleToughness (subject : NounPhrase) (duration : Option Duration := none) :
    Action :=
  .ownScope (some (.action "triple"))
    (get (.firstMention "tripleToughness.subject" (.inCaller subject)) (.up (.lit 0))
      (.up
        (times (.lit 2)
          (toughnessOf (.laterMention "tripleToughness.subject" (.inCaller subject)))))
      duration)
/-- "double <performer>'s life total" (`Definition`, [CR#701.10d]), written as that total's set
[CR#119.5]. -/
semantic_definition doubleLifeTotal : Action :=
  .ownScope (some (.action "double"))
    (.changeLife (.set (times (.lit 2) (lifeTotalOf .actor))))

/-! ## Tokens -/

/-- A Food token's characteristics: "a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" [CR#111.10b] -/
semantic_definition foodToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [food],
        text :=
          [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice thisArtifact)])
              (gainLife (.lit 3)) ] } }
/-- A Treasure token's characteristics: "a colorless Treasure artifact token with '{T},
Sacrifice this token: Add one mana of any color.'" [CR#111.10a] -/
semantic_definition treasureToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [treasure],
        text :=
          [ activated (.compound [.tapSymbol, .perform (sacrifice thisArtifact)])
              (.addMana (.lit 1) (.anyColor .sameColor) []) ] } }

/-- A Clue token's characteristics: "a colorless Clue artifact token with '{2}, Sacrifice this
token: Draw a card.'" [CR#111.10f] -/
semantic_definition clueToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [clue],
        text :=
          [ activated (.compound [.mana [generic 2], .perform (sacrifice thisArtifact)])
              (draw (.lit 1)) ] } }

/-! ## Templates over Definitions -/

/-- "Exile <subject> until <event>." -/
semantic_template exileUntil (subject : NounPhrase) (event : GameEvent) : Action :=
  .holdUntil (exile subject) event
/-- The replacement's application, rather than installation of a new shield
[CR#701.19b,701.19c]. -/
semantic_template regenerationApplication (subject : NounPhrase) : Action :=
  .enact (.action "regenerate")
    (sequentially
      [ .clearDamage (.firstMention "regenerationApplication.subject" subject),
        instruct
          (.possessorOf .controller (.laterMention "regenerationApplication.subject" subject))
          (tap (.laterMention "regenerationApplication.subject" subject)),
        doIf
          (or
            [ matches_ (.laterMention "regenerationApplication.subject" subject)
                (.inCombat .attackerOf none),
              matches_ (.laterMention "regenerationApplication.subject" subject)
                (.inCombat .blockerOf none) ])
          (.combat (.laterMention "regenerationApplication.subject" subject)
            (.participation .outsideCombat))
          none ])
    none none
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
        (allOf (and [non isLand, .isCard, .did ⟨.any, discard .which, .thisWay⟩]))
        step ]
/-- "you may choose not to untap <subject> during [<whose>] untap step" -/
semantic_template mayDeclineUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  duringUntapStep whose (.can .any (untap subject) {})
/-- "untap <subject> during [<whose>] untap step" -/
semantic_template untapsDuring (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  duringUntapStep whose (.must .any (untap subject) {})
/-- "<subject> may be attached to <host>": the legal-host permission over the attach deed's host
role [CR#701.3a,301.5]. -/
semantic_template mayBeAttachedTo (subject host : NounPhrase) : StaticSpec :=
  .can .any (attachTo subject host) {}
/-- "create <count> <token>" -/
semantic_template createToken (token : CharacteristicBundle) (count : Amount := .lit 1) : Action :=
  create count (.written token)
/-- "create <count> tapped <token>" -/
semantic_template createTappedToken (token : CharacteristicBundle) (count : Amount := .lit 1) :
    Action :=
  create count (.written token) [.entersAs .tapped]
/-- "create <count> <token> tapped and attacking" -/
semantic_template createTappedAttacking (count : Amount) (token : CharacteristicBundle) : Action :=
  create count (.written token) [.entersAs .tapped, .entersAttacking none]
/-- "create a token that's a copy of <source> [riders]" -/
semantic_template createTokenCopy (source : NounPhrase) (riders : List MoveRider := []) : Action :=
  create (.lit 1) (.copyOf source []) riders

/-! The regeneration shield is a Definition, but its replacement applies
`regenerationApplication`, a Template over `tap`, so it follows that Template here. -/

/-- A resolving regeneration action installs a single-use shield for this turn;
only its eventual application carries the Regenerate label (`Definition`, [CR#701.19a,701.19c]). -/
semantic_definition regenerate (subject : NounPhrase) : Action :=
  .ownScope (some (.action "regenerate"))
    (.establish (.instead
      (.would .any (destroy (.firstMention "regenerate.0" (.inCaller subject))))
      (regenerationApplication (.laterMention "regenerate.0" (.inCaller subject)))
      none .nextTimeOnly none) (some thisTurn))

private theorem regenerationEventDescribesTheApplicationRatherThanTheShield :
    let event := does any (regenerationApplication thisCreature)
    let applied := (stepGameEvent 64 (DRS.ofList []) event).1
    let shield := (stepAction 64 .postEvent .obj (DRS.ofList []) (regenerate thisCreature)).1
    GameEvent.check (DRS.ofList []) event = [] ∧
      countOnes .object applied = 1 ∧
      (applied.findById? 0).map (fun ref => (ref.kind, ref.zone)) =
        some (.object, some .battlefield) ∧
      countOnes .object shield = 0 ∧
      Action.check (DRS.ofList [])
        (.triggerReflexively (regenerate thisCreature) (some event)
          (.putCounters (.lit 1) (.printed m1m1Counter) it .many true)) = [] := by decide

/-! ## Keyword actions -/

semantic_definition adapt (amount : Amount) : Action :=
  .ownScope (some (.action "adapt"))
    (enact (.action "adapt")
      (doIf (not (matches_ .actor (.hasCounters (some p1p1Counter))))
        (putCounters (.inCaller amount) (.printed p1p1Counter) .actor)
        none))
semantic_definition behold (quality : Predicate) : Action :=
  .ownScope (some (.action "behold"))
    (enact (.action "behold")
      (chooseOne
        (.expose .reveal (.cards (a (and [.isCard, .inCaller quality, .in_ (handOf .actor)]))))
        (choose (a (and [.inCaller quality, isPermanent, actorControls])))))
semantic_definition blight (amount : Amount) : Action :=
  .ownScope (some (.action "blight"))
    (enact (.action "blight")
      (putCounters (.inCaller amount) (.printed m1m1Counter)
        (a (and [isCreature, actorControls]))))
semantic_definition bolster (amount : Amount) : Action :=
  .ownScope (some (.action "bolster"))
    (enact (.action "bolster")
      (sequentially
        [ choose
            (a
              (and
                [ isCreature,
                  actorControls,
                  .superlative .min (.stat .toughness) (and [isCreature, actorControls]) ])),
          putCounters (.inCaller amount) (.printed p1p1Counter) (that creature) ]))
semantic_definition clash : Action :=
  .ownScope (some (.action "clash"))
    (enact (.action "clash")
      (sequentially
        [ .expose .reveal (.cards (.slice .top (.lit 1) (libraryOf .actor))),
          may (putInto (that .card) onBottom) ]))
semantic_definition convert (subject : NounPhrase) : Action :=
  .ownScope (some (.action "convert"))
    (enact (.action "convert") (.turnOver (.inCaller (.firstMention "convert.arg0" subject))))
semantic_definition detain (subject : NounPhrase) : Action :=
  .ownScope (some (.action "detain"))
    (enact (.action "detain")
      (establishThroughout
        (.conjunction none
          [ .conjunction none
              [ cant (.actor (.firstMention "detain.0" (.inCaller subject))) (.attack none),
                cant (.actor (.laterMention "detain.0" (.inCaller subject))) (.block .any) ],
            cant .any
              (activate (allOf
                  (and
                    [ .abilityHead .anyActivated,
                      .abilityOf (.laterMention "detain.0" (.inCaller subject)) ]))) ])
        (untilNextTurnOf (controllerOf this_))))
semantic_definition earthbend (amount : Amount) : Action :=
  .ownScope (some (.action "earthbend"))
    (enact (.action "earthbend")
      (sequentially
        [ .establish
            (.characteristicChange (target (and [isLand, actorControls]))
              [ .typeLine .adds { types := some [.creature] },
                .stat .sets .power (.lit 0),
                .stat .sets .toughness (.lit 0),
                .addedAbilities [keyword (.named "haste")] ])
            none,
          putCounters (.inCaller amount) (.printed p1p1Counter) (that land),
          delay (does (that land) die)
            (returnTo (that land) battlefield [.entersAs .tapped, .under .actor])
            [does .any (.move (that land) battlefield (some exileZone) .departure none [])] ]))
semantic_definition endure (amount : Amount) : Action :=
  .ownScope (some (.action "endure"))
    (enact (.action "endure")
      (instruct (controllerOf .actor)
        (chooseOne (putCounters (.inCaller amount) (.printed p1p1Counter) (that .permanent))
          (createToken (creatureTokenOf (.inCaller amount) (.inCaller amount) [.white] [spirit])))))
semantic_definition exert (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exert"))
    (enact (.action "exert") (.skipUntap (.inCaller (.firstMention "exert.arg0" subject)) (.lit 1)))
semantic_definition faceAVillainousChoice (first second : Action) : Action :=
  .ownScope (some (.action "faceAVillainousChoice"))
    (enact (.action "faceAVillainousChoice") (chooseOne (.inCaller first) (.inCaller second)))
semantic_definition forage : Action :=
  .ownScope (some (.action "forage"))
    (enact (.action "forage")
      (chooseOne
        (exileFrom (counted (exactly 3) (cardIn (graveyardOf .actor))) (graveyardOf .actor))
        (sacrifice (a (and [.hasSubtype food, isPermanent, actorControls])))))
semantic_definition goad (subject : NounPhrase) : Action :=
  .ownScope (some (.action "goad"))
    (enact (.action "goad")
      (.gainDesignation (.inCaller (.firstMention "goad.arg0" subject)) goaded
        (some (untilNextTurnOf (controllerOf this_)))))
semantic_definition harness (subject : NounPhrase) : Action :=
  .ownScope (some (.action "harness"))
    (enact (.action "harness")
      (doIf
        (not
          (matches_ (.firstMention "harness.0" (.inCaller subject))
            (.hasDesignation harnessed none)))
        (.gainDesignation (.laterMention "harness.0" (.inCaller subject)) harnessed none)
        none))
semantic_definition heal (subject : NounPhrase) : Action :=
  .ownScope (some (.action "heal"))
    (enact (.action "heal") (.clearDamage (.inCaller (.firstMention "heal.arg0" subject))))
semantic_definition investigate : Action :=
  .ownScope (some (.action "investigate")) (enact (.action "investigate") (createToken clueToken))
semantic_definition populate : Action :=
  .ownScope (some (.action "populate"))
    (enact (.action "populate")
      (sequentially
        [ choose (a (and [.isToken, isCreature, isPermanent, actorControls])),
          createTokenCopy thatToken ]))
semantic_definition recruit : Action :=
  .ownScope (some (.action "recruit"))
    (enact (.action "recruit")
      (sequentially
        [ .draw (.lit 1),
          discard (a (cardIn (handOf .actor))),
          doIf (not (matches_ (theVerbed (.action "discard") .one (some .card) .thisWay) isLand))
            (createToken (creatureToken 1 1 [.white] [human, soldier]))
            none ]))
semantic_definition search (zone : Location) (quantity : Quantity) (quality : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (enact (.action "search") (.search zone (.inCaller quantity) (.inCaller quality)))
semantic_definition suspect (subject : NounPhrase) : Action :=
  .ownScope (some (.action "suspect"))
    (enact (.action "suspect")
      (.gainDesignation (.inCaller (.firstMention "suspect.arg0" subject)) suspected none))
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
        (enact (.action "theRingTemptsYou") (sequentially tempting)))
      you thisGame
  .ownScope (some (.action "theRingTemptsYou"))
    (enact (.action "theRingTemptsYou")
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
                      (triggered (does (a ringBearers) attack)
                        (sequentially [draw (.lit 1), discard (a (cardIn hand))])))
                    (compareAmt timesTempted .atLeast (.lit 2))),
                static
                  (onlyWhile
                    (abilityGrant this_
                      (triggered
                        (does (by_ (a isCreature)) (block (a ringBearers)))
                        (atNext .endOfCombat
                          (instruct (controllerOf (that creature)) (sacrifice (that creature))))))
                    (compareAmt timesTempted .atLeast (.lit 3))),
                static
                  (onlyWhile
                    (abilityGrant this_
                      (triggered
                        (does (a ringBearers)
                          (dealDamage (to := a isPlayer) (kind := .combatOnly)))
                        (instruct eachOpponent (loseLife (.lit 3)))))
                    (compareAmt timesTempted .atLeast (.lit 4))) ])
            :: tempting)))
semantic_definition timeTravel : Action :=
  .ownScope (some (.action "timeTravel"))
    (enact (.action "timeTravel")
      (sequentially
        [ choose
            (counted anyNumber
              (or
                [ and [isPermanent, actorControls, .hasCounters (some timeCounter)],
                  and
                    [.isCard, .in_ exileZone, ownedBy .actor, .hasCounters (some timeCounter)] ])),
          .doForEach them
            (chooseOne (putCounters (.lit 1) (.printed timeCounter) it)
              (removeCounters (oneThrough 1) (some (.printed timeCounter)) it)) ]))

end Semantics.Definitions
