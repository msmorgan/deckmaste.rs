import Semantics.Events

/-!
# Semantics.Abilities

The Semantics v3 unified action-clause spine: phrases (`Location`, `Predicate`, `NounPhrase`,
`Amount`, `Quantity`, `Condition`, `GameEvent`, `DidClause`, `LookbackSpan`, `Subject`), trigger
headers and bounds (`UsageLimit`, `JoinedHeader`), actions (`Action`), static specifications
(`StaticSpec`, `WouldClause`, `Replacement`, `DeonticMods`), costs (`Cost`), and abilities
(`Ability`).

Because `GameEvent.does` and `DidClause` (`Predicate.did`, `Amount.tally`, `Condition.did`)
embed `action : Action` directly, the phrase, trigger, and ability grammars form a single
mutually recursive block (`docs/decisions/semantics-v3.md` §3).
-/

namespace Semantics

/-- Which version of a zone-changing object supplies the event pattern's characteristics
[CR#603.10,603.10a,700.4,603.6a,603.6c]. -/
inductive ObservationPoint where
  | arrival | departure
  deriving DecidableEq, Repr

inductive ManaHeld where
  | thisMana
  | unspent (type : Option ColorOrColorless)
  deriving DecidableEq, Repr

/-- A word a Text Change replaces where it is used in its rules sense, such as a color word, a
land type or a creature type, a creature type in a token's name included [CR#612.2,612.2a]. -/
inductive TextWord where
  | color (color : Color)
  | subtype (subtype : Subtype)
  deriving DecidableEq, Repr

inductive QualityOp where
  | adds | sets | loses
  deriving DecidableEq, Repr

/-- Only present axes are written. A present empty list explicitly clears that axis. An object
that is "still a land" keeps all its prior types [CR#205.1b], so that is an `adds` of the new
ones. -/
structure TypeLineChanges where
  supertypes : Option (List Supertype) := none
  types : Option (List CardType) := none
  subtypes : Option (List Subtype) := none
  deriving Repr, BEq

inductive CharacteristicStat where
  | power | toughness | loyalty | defense
  deriving DecidableEq, Repr

inductive SpentMode where
  | affectsIt | triggersThen
  deriving DecidableEq, Repr

/-- Optional payment is decided by its performer, the actor in context; required payment tests
whether it started [CR#118.12]. -/
inductive ContinuationPolicy where
  | optional
  | required
  deriving DecidableEq, Repr

/-- Where a name comes from: printed or chosen. "With the same name as …" is Sharing on the
name axis, `Predicate.shares .cardName .share`. -/
inductive NameSource where
  | printed (name : String)
  | chosen
  deriving DecidableEq, Repr

/-- The Coordinator that links the Conjuncts of a Coordination of noun phrases, locations or
predicates: "and", "or", or "and/or". Each is its own encoding, so "X and/or Y" never shares a
tree with "X and Y" or with "X or Y". -/
inductive Coordinator where
  | and
  | or
  | andOr
  deriving DecidableEq, Repr

open scoped Semantics.Authoring

mutual
  -- ## Phrase Grammar

  /-- Where objects are, or where a move takes them from or puts them: a zone, a place in a
  pile, or a combination of them. A pile is objects grouped in a zone, in order, the pile itself
  not an object [CR#700.3,700.3b]; grouping neither moves its objects nor reorders them
  [CR#700.3c]. A library is kept face down [CR#401.2], a graveyard face up and still ordered
  [CR#404.2,700.3c]. -/
  inductive Location where
    | inCaller (body : Location)
    /-- A zone: `whose` names its owner ("your graveyard"), and none is a bare zone name. -/
    | zone (zone : Zone) (whose : Option NounPhrase)
    /-- The `ordinal`th place from one end of a pile: "on top of", "on the bottom of", "seventh
    from the top" [CR#401.7]. -/
    | fromEnd (end_ : PileEnd) (ordinal : Ordinal := .nth 1) (loc : Location)
    /-- The cards exiled with `source`: those its linked ability exiled, face up unless that
    ability exiled them face down [CR#406.3,406.6,607.2a]. -/
    | exiledWith (source : NounPhrase)
    /-- A pile an action made ("that pile"), with the face it was given. -/
    | pile (pile : NounPhrase)
    /-- Anywhere: a location that names no zone, as exile's origin "from wherever it is"
    [CR#701.13a]. -/
    | anyLocation
    | not (loc : Location)
    /-- The union of `locs`, as its Coordinator prints it: "on the top or bottom of your library",
    "graveyard, hand, and library", "library and/or graveyard". -/
    | coord (coordinator : Coordinator) (locs : List Location)

  /-- What a choice ranges over. -/
  inductive ChoiceDomain where
    | nameOfCard (predicate : Predicate)
    | colorOtherThan (color : Color)
    | typeOtherThan (subtype : Subtype)
    | basicTypesOnly
    | nonbasicTypesOnly
    | number (quantity : Quantity)
    | players (predicate : Predicate)
    /-- "choose flying or trample": an ability chosen among named keywords. -/
    | abilitiesAmong (keywords : List KeywordTerm)

  /-- The grammatical subject and voice of an action clause (`does`, `would`, `cant`, `must`,
  `can`, `gatedBy`, `did`). -/
  inductive Subject where
    | actor (np : NounPhrase)
    | any
    | by (np : NounPhrase)

  /-- The span or provenance a lookback clause (`DidClause`) inspects. -/
  inductive LookbackSpan where
    /-- Within a temporal duration ("this turn", "last turn", "this game"), and optionally only
    when `timing` held as the action happened ("cast when a sorcery couldn't have been"
    [CR#307.5a]). -/
    | duration (d : Duration) (timing : Option Timing)
    /-- The event the enclosing action caused ("the cards discarded this way"). -/
    | thisWay
    /-- The event that triggered the enclosing ability [CR#603.2]. -/
    | triggering
    /-- The described object's own current existence, the span `Period.thisObject` names: "artifact
    spells you cast", "if you cast it from your hand" read the casting or activation of the object
    as it is now, not of an earlier object it was before a zone change [CR#400.7]. -/
    | thisObject

  /-- A historical action clause ("if you sacrificed a creature this turn", "each card exiled
  this way", "creatures that attacked this turn"), used by `Condition.did`, `Predicate.did`,
  and `Amount.tally`. With no span written (`within = none`) it is an attributive participle
  ("the exiled card"), which only a Participial Description reads: it names the objects a deed
  in scope acted on, in this ability or in an ability linked to it [CR#607.1,607.2a]. An
  `ordinal` picks out the `ordinal`th matching event, counted within each `per` when one is
  written ("the first spell you've cast this game"): the same shape as `GameEvent.nthOccurrence`
  and `WouldClause.nthOccurrence`. -/
  structure DidClause where
    subj : Subject
    action : Action
    within : Option LookbackSpan
    ordinal : Option (Ordinal × Option Period) := none

  /-- Which turn a timing or duration names. `that` reads back the turn an earlier phrase
  named ("that turn"); there is no "this", which names an object. -/
  inductive TurnRef where
    | current
    /-- The previous turn, or the most recent of `player`'s turns before the current one ("your
    last upkeep" [CR#702.30a]). -/
    | previous (player : Option NounPhrase)
    /-- The next turn, or the next of `player`'s turns. -/
    | next (player : Option NounPhrase)
    /-- Each turn, or each of `player`'s turns. -/
    | each (player : Option NounPhrase)
    /-- A turn after the current one [CR#702.170d]. -/
    | later
    | that
    /-- `player`'s most recent turn: the current turn while it is `player`'s, and otherwise the
    last turn `player` took ("their most recent turn" [CR#302.6]). Which turn that is at runtime
    is the engine's (ADR `semantics-v3.md` §0.1 item 4). -/
    | mostRecent (player : NounPhrase)

  /-- When a player may act, or when a trigger or replacement applies. With no timing written,
  a player may act any time they have priority [CR#117.1a]. -/
  inductive Timing where
    /-- During `part` of the turn `which` names, whose turn it is being `whose`'s. -/
    | inPart (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef)
    | relativeToEvent (side : EventSide) (event : GameEvent)
    /-- "while …": an event underway. -/
    | whileDoing (event : GameEvent)
    | onlyIf (condition : Condition)
    | and (conjuncts : List Timing)
    | or (disjuncts : List Timing)
    | not (timing : Timing)
    | anyTiming

  /-- How long an effect lasts, or the span a lookback reads. With no duration stated (`none`), an
  effect lasts until the game ends [CR#611.2a] ("Vexing Shusher", "Boseiju"). A printed "for the
  rest of the game" is `some restOfGame` (epic, "Screaming Nemesis"), and a rider that modifies
  the preceding action is `some .modifiesPrecedingAction` [CR#608.2c] ("Terror", "Pinpoint
  Avalanche"); neither is written as `none`. -/
  inductive Duration where
    /-- Bounded by two optional events: "until end of turn" has only an end, "on a later turn"
    only a start [CR#702.170d]. A history window ends at `GameEvent.now`. -/
    | between (start : Option GameEvent) (end_ : Option GameEvent)
    | forAsLongAs (condition : Condition) -- [CR#611.2b]
    /-- Later text that modifies the meaning of the action before it, the step just before it in
    a `sequentially`: in force only while that action's event happens. "Destroy target creature.
    It can't be regenerated." makes that destroy one no regeneration shield replaces; it is no
    separate continuous effect [CR#608.2c,701.19c]. -/
    | modifiesPrecedingAction

  /-- A Linked Facet: what a linked ability reads back about its Link Source [CR#607.2a]. -/
  inductive LinkedFacet where
    /-- colors of mana spent to cast it [CR#702.44a] -/
    | colorsSpent
    /-- mana spent to pay the total cost [CR#601.2h] -/
    | manaValueSpent
    /-- [CR#702.33c..702.33d] -/
    | timesPaid (which : LinkSource)
    /-- "was kicked" [CR#702.33d], optionally within a span ("this turn") -/
    | readback (which : LinkSource) (within : Option Duration)
    /-- Whether the choice `which` offered was taken: "if tribute wasn't paid" reads it false
    [CR#702.104b]. A cost's choice is taken as its payment is announced [CR#601.2b]: mutate's
    "if you do" [CR#702.140a]. -/
    | taken (which : LinkSource)

  inductive Predicate where
    | inCaller (body : Predicate)
    | hasType (type : CardType)
    | hasSubtype (subtype : Subtype)
    | hasSupertype (supertype : Supertype)
    | anyPlayer
    /-- The member is an opponent of `who` [CR#102.2,102.3]. -/
    | isOpponentOf (who : NounPhrase)
    /-- The member is a teammate of `who` [CR#102.3]. -/
    | isTeammateOf (who : NounPhrase)
    /-- "chosen": the member is one a choice in scope chose, of whatever sort the description
    names: "the chosen player" (`and [isPlayer, chosen]`), "the chosen creature", "the chosen
    cards". An object is chosen when a "choose" announces it, as a target chosen as the spell is
    cast [CR#601.2c] or as a choice made while the effect applies [CR#608.2d]. Its complement is
    `notChosen`. -/
    | chosen (ref : ChoiceRef)
    | qualityNoun (sort : QualitySort) (domain : Option ChoiceDomain)
    | counterKindOn (subject : NounPhrase)
    | ofChosen (ref : ChoiceRef) (sort : QualitySort)
    | ofYourChoice (sort : QualitySort) (domain : Option ChoiceDomain)
    /-- "each <value of `axis`> [among <domain>]" as a keyword's quality: "protection from each
    color", "protection from each color among permanents you control". It is shorthand for one
    ability per value, each with that value as its quality, and they behave as separate
    abilities [CR#702.16h,702.16i]. With a domain, the values are those its objects have, read
    continuously as a static ability's text is [CR#611.3a]. It stands only as the whole quality
    of a keyword (`eachKindPlacement`). -/
    | eachKind (axis : KindAxis) (domain : Option NounPhrase)
    | hasKeyword (keyword : KeywordTerm)
    | hasPossessor (axis : PossessorAxis) (possessor : NounPhrase)
    /-- The spell was a card holding `designation` before it was cast: "if this spell was
    foretold" reads a spell that was a foretold card before it was cast, even one cast for a cost
    other than a foretell cost [CR#702.143c]. -/
    | heldBeforeCast (designation : DesignationLabel)
    | inCombat (relation : CombatRelation) (counterpart : Option NounPhrase)
    /-- A relative-clause event filter using `NounPhrase.which` in the relativized argument slot
    ("creatures that attacked this turn", "each card exiled this way"). -/
    | did (clause : DidClause)
    | colorIs (color : Color)
    | colorCount (comparator : Comparator) (count : Nat)
    | named (source : NameSource)
    /-- Sharing: the member and `with_` have at least one value in common on `axis` (`share`)
    or none (`differ`) [CR#201.2a,201.2c,105.2,205.3m]. On the name axis it is not equality:
    an object with two names shares a name with an object that has either [CR#201.2a]. -/
    | shares (axis : QualitySort) (agreement : Agreement) (with_ : NounPhrase)
    | hasDesignation (designation : DesignationLabel) (holder : Option NounPhrase)
    /-- The member stands in the Relation `label` as its first side, with `counterpart` as the
    second, or with anything when `counterpart` is none: "paired", "paired with it", "encoded
    on that creature" [CR#702.95b,702.99b]. -/
    | related (label : RelationLabel) (counterpart : Option NounPhrase)
    | attachment (side : AttachmentSide) (word : Option AttachWord)
        (counterpart : Option NounPhrase)
    /-- The member is the `side` of a legal attachment whose other side is `counterpart`, under
    the attachment's own abilities and every other applicable effect [CR#701.3a]. With `.host`,
    the member is an object or player `counterpart` could legally be attached to, as an Aura
    entering the battlefield asks [CR#303.4f,303.4g,303.4i]: its enchant ability, and, say, a
    protection that keeps it off [CR#702.16c]. With `.attachment`, the member could legally be
    attached to `counterpart`: "Aura cards that could enchant it". Whether an attachment is legal
    is the engine's to evaluate. -/
    | legalAttachment (side : AttachmentSide) (counterpart : NounPhrase)
    | isCard
    | isToken
    | isEmblem
    | isCopyOfACard
    /-- The member has `type` with every type-changing effect that would affect it ignored
    [CR#613.1d]: sunburst asks whether its object "is entering as a creature, ignoring any
    type-changing effects that would affect it" [CR#702.44a]. -/
    | hasTypeIgnoringTypeChanges (type : CardType)
    /-- The whole single double-faced permanent has this side up [CR#701.27g]. -/
    | currentFace (side : CardFaceSide)
    | hasStatus (status : Status)
    | hasCounters (kind : Option CounterKind)
    | compare (axes : List ProjAxis) (comparator : Comparator) (bound : Amount)
    | superlative (op : AggregateOp) (axis : ProjAxis) (domain : Predicate)
    /-- A vote stands only where a spell or ability instructed players to vote [CR#701.38a]. -/
    | withMostVotes
    | choseExtreme (op : AggregateOp)
    | compareOver (domain : Predicate) (measure : Amount) (comparator : Comparator) (bound : Amount)
    /-- The member is at `loc`: in a zone, in a pile, or exiled with an object. -/
    | in_ (loc : Location)
    /-- A Coordination of predicates. With "and" every conjunct holds; with "or" or "and/or" some
    conjunct holds, and conjuncts of different kinds join: "creature or player" denotes either. -/
    | coord (coordinator : Coordinator) (items : List Predicate)
    | not (predicate : Predicate)
    /-- A bare printed "another" or "other" ("another target creature you control", "each other
    player"): other than its Contrast Anchor, which the checker resolves (`semantics-v3.md`
    §0.1). It need not contrast with a target. -/
    | other
    | notChosen
    /-- A printed "other than <anchor>": not `anchor` itself, nor one of it ("all permanents other
    than this creature", "each opponent other than defending player"). -/
    | otherThan (anchor : NounPhrase)
    | coinCameUp (face : CoinFace)
    | isSource
    | manaCostHas (symbol : ManaSymbol)
    | abilityHead (class_ : AbilityClass)
    | abilityOf (source : NounPhrase)
    /-- The member is an activated ability whose activation cost, everything before its colon
    [CR#602.1a], includes `cost`: "with {T} in its cost", "with the tap symbol or the untap
    symbol in its activation cost" [CR#302.6,107.5,107.6]. -/
    | activationCostHas (cost : Cost)
    /-- The member is an ability that has triggered but not yet left the stack: one waiting to be
    put on the stack, or one on it [CR#113.7,603.3]. A Saga, a dungeon or a Siege that is the
    source of one stays [CR#704.5s,704.5t,704.5v]. -/
    | triggeredNotLeftStack
    | targets (subject : NounPhrase) (extent : TargetExtent)

  inductive DetPhrase where
    | target (quantity : Quantity)
    | a (mode : ChoiceMode)
    /-- "each …": a group, resolution-time [CR#608.2] -/
    | each
    | all
    /-- "the <noun>", or with `plur = many` "the <nouns>" ("the exiled cards") [CR#607.2a] -/
    | the (plur : Plurality := .one)
    | count (quantity : Quantity) (mode : Option ChoiceMode)

  /-- A color, written, "the chosen color", or an object's color ("that creature's color"). -/
  inductive ColorTerm where
    | lit (color : Color)
    | chosen (ref : ChoiceRef)
    /-- The color of `object` [CR#105.2]: a Mana Substitute's colored pip reads the acted-on
    object back ("mana of that creature's color" [CR#702.51a]). -/
    | colorOf (object : NounPhrase)

  /-- The mana type a "tapped for mana of …" trigger specifies [CR#106.12a]: colorless, or a
  color written or chosen; the six types of [CR#106.1b]. -/
  inductive ManaTypeTerm where
    | colorless
    | ofColor (color : ColorTerm)

  inductive NounPhrase where
    | inCaller (body : NounPhrase)
    /-- The relative pronoun bound by the enclosing `Predicate.did` relative clause. -/
    | which
    /-- the source, by self-name or "this spell" [CR#113.7] -/
    | this
    /-- `subject` named by a Head Noun: "this creature", "this spell", "this Aura". -/
    | headed (head : NounWord) (subject : NounPhrase)
    /-- The permanent the spell `subject` becomes as it resolves, named before it resolves: "the
    permanent this spell becomes", "it" for a creature spell mana was spent on [CR#608.3a]. A
    type view of the spell's referent, refined in place: effects that apply to the spell
    continue to apply to that permanent [CR#400.7a,400.7b]. -/
    | becomes (subject : NounPhrase)
    | theGrantor (head : NounWord)
    | you
    /-- Whoever performs the action this phrase is part of: the controller [CR#109.5],
    unless an enclosing `Action.instruct` hands the action to another player or to a
    permanent, which it then is. -/
    | actor
    | combatPlayer (role : CombatRole)
    | described (determiner : DetPhrase) (predicate : Predicate)
    | eachOf (group : NounPhrase)
    /-- A Coordination of noun phrases. With "and" or "and/or" it denotes the group of what its
    conjuncts denote; with "or" it denotes one of them. -/
    | coord (coordinator : Coordinator) (phrases : List NounPhrase)
    /-- "The top N cards of <pile>": `amount` objects from one end of a pile, an ordered group
    [CR#700.3c]. -/
    | slice (end_ : PileEnd) (amount : Amount) (pile : Location)
    /-- "Cards from the top of <pile> until a <stop> card": the run of objects taken one at a
    time from that end, the last of them the first that matches `stop` [CR#701.57a]. -/
    | run (end_ : PileEnd) (stop : Predicate) (pile : Location)
    | someOf (count : SliceCount) (description : Option Predicate) (group : NounPhrase)
    /-- Agreement: the members of `group` share, or do not share, a value on `axis` ("two
    creatures that share a creature type"), a constraint on the selection, not a predicate on
    one member, read by `Agreement.holdsAmong`: a group has different names only if each member
    has a name [CR#201.2b]. -/
    | agree (axis : QualitySort) (agreement : Agreement) (group : NounPhrase)
    | theRest (kind : Kind) (plurality : Plurality)
    | pileOf (count : SliceCount) (by_ : Option NounPhrase)
    /-- A pronoun or demonstrative: what it reaches for and its grammatical number. -/
    | pro (reach : Reach) (plurality : Plurality)
    /-- A Principle A reflexive pronoun (`itself`, `themselves`, `yourself`), which resolves
    to the current clause's subject. -/
    | refl (plurality : Plurality)
    /-- A Definition or Template body's first mention of a referent parameter: the argument, checked and
    introduced where it stands, with the binding it introduces or resolves recorded under the
    Binding Identity `id`. -/
    | firstMention (id : BindingId) (phrase : NounPhrase)
    /-- A later mention of the same parameter: the binding recorded under `id`, read back by that
    identity rather than by a pronoun search. `phrase` is the argument again, kept only for its
    structural facts (its kind, whether it is "you" or the source); it is never checked or
    introduced a second time, so it selects nothing. -/
    | laterMention (id : BindingId) (phrase : NounPhrase)
    | attachHost (word : AttachWord) (head : NounWord)
    | possessorOf (axis : PossessorAxis) (subject : NounPhrase)
    | designated (designation : DesignationLabel) (whose : NounPhrase)
    /-- The object on the second side of the Relation `label` whose first side is `subject`:
    "the creature it haunts" [CR#702.55b]. -/
    | counterpart (label : RelationLabel) (subject : NounPhrase)
    | oneEachOf (roles : List Predicate) (pool : NounPhrase)
    | any

  inductive Amount where
    | inCaller (body : Amount)
    | lit (value : Int)
    | statOf (axis : ProjAxis) (subject : NounPhrase)
    | countOf (group : NounPhrase)
    | aggregate (op : AggregateOp) (axis : ProjAxis) (group : NounPhrase)
    | paid (facet : LinkedFacet) (subject : NounPhrase)
    | tally (clause : DidClause) (mode : TallyMode)
    | thatMuch
    | chosenNumber (ref : ChoiceRef)
    | votesFor (label : VoteLabel)
    | theOutcome (sort : OutcomeSort)
    | coinsShowing (face : CoinFace)
    | greatestStoredMatch (subject : NounPhrase)
    | groupSize
    | theDifference
    | letter (letter : Letter)
    | arith (op : ArithOp) (left right : Amount)
    | devotion (player : NounPhrase) (color : ColorTerm) (second : Option ColorTerm)
    | half (rounding : RoundMode) (amount : Amount)
    | aggregateOver (op : AggregateOp) (domain : Predicate) (body : Amount)
    /-- An aggregate over listed amounts rather than over a group or a domain: "the smaller of
    the number of +1/+1 and −1/−1 counters on it" [CR#704.5q] is `min` over the two counts,
    "whichever is greater" `max`. -/
    | aggregateOf (op : AggregateOp) (amounts : List Amount)
    | distinctCount (axis : KindAxis) (domain : NounPhrase)
    | anyAmount
    /-- "The Nth <kind> counter", as a put pattern's amount: a put that leaves its holder with N
    or more counters of the kind after having had fewer than N before, whether it puts one
    counter or several [CR#122.7]. It is a threshold, not a count of puts: counters removed in
    between, annihilated ones included [CR#122.3], leave the holder short of N again. The
    put-side counterpart of `Quantity.theLast`. -/
    | theNth (ordinal : Ordinal)

  inductive Quantity where
    | inCaller (body : Quantity)
    | range (low high : Option Nat)
    | upToOf (amount : Amount)
    | exactlyOf (amount : Amount)
    /-- "Up to N {P} worth of modes": any modes whose weights total at most `n` [CR#700.2i]. -/
    | worth (n : Nat)
    /-- "More than N": what a "can't" bounds ("can't cast more than one spell each turn"). -/
    | moreThanOf (amount : Amount)
    /-- "An additional N": `quantity` beyond what is otherwise allowed ("play an additional land
    on each of your turns"). -/
    | additional (quantity : Quantity)
    /-- "The last <kind> counter": the removal that leaves none of its kind, as suspend's "When
    the last time counter is removed from this card" [CR#702.62a] and a Siege's "When the last
    defense counter is removed" [CR#310.12b]. Emptiness is part of the trigger event, evaluated
    at trigger time, not an intervening if rechecked on resolution [CR#603.4]; suspend's separate
    "if it's exiled" is the intervening if [CR#702.62a]. -/
    | theLast

  inductive SliceCount where
    | counted (quantity : Quantity)
    | whole
    /-- "Any number of them with total <axis> <comparator> <bound>": as many as the chooser
    likes, so long as the sum of `axis` over the chosen ones compares to `bound`
    [CR#701.59a]. -/
    | totalling (axis : ProjAxis) (comparator : Comparator) (bound : Amount)

  inductive Door where
    | thisDoor
    | doorOf (state : Option LockState) (room : NounPhrase)

  /-- Which roll results a trigger watches. -/
  inductive RollWatch where
    | anyResult
    | resultIn (quantity : Quantity)
    | highestNatural

  inductive Condition where
    | inCaller (body : Condition)
    | duringPart (part : TurnPart) (whose : Option NounPhrase)
    /-- "if there is a …" -/
    | exists_ (subject : NounPhrase)
    | did (clause : DidClause)
    | matches (subject : NounPhrase) (predicate : Predicate)
    /-- The game has `designation`, one the game itself can have: "if it's day" [CR#731.1]. The
    game is neither an Object nor a Player, so it is no Reference, and is named only here and in
    `Action.gameGains`. -/
    | gameHas (designation : DesignationLabel)
    | compareAmt (subject : Amount) (comparator : Comparator) (bound : Amount)
    | voteLead (label : VoteLabel) (orTied : Bool)
    | anyResultIs (comparator : Comparator) (bound : Amount)
    | rolledDoubles
    /-- `player` has priority [CR#117.1a]. A mana ability may be activated without it
    [CR#117.1d], so instant speed is written with this condition. -/
    | hasPriority (player : NounPhrase)
    /-- `chooser` so chooses: a Choice `chooser` makes each time the clause it conditions would
    apply. In the untap step the active player determines which permanents they control will
    untap [CR#502.3], so "you may choose not to untap it during your untap step" is a prohibition
    on untapping it that holds only if you so choose: untapping stays the default, and it is no
    permission to untap. -/
    | soChooses (chooser : NounPhrase)
    /-- `condition` has held continuously from `since` until now: "has been under its
    controller's control continuously since their most recent turn began" [CR#302.6]. The
    perfect reads the condition's referents as they are now, so "its controller" is the
    present controller, and the condition is that this player has controlled the object
    throughout. An object that became a new object after `since` held nothing before it
    existed [CR#400.7]. `since` follows `condition`, so its pronouns read the condition's
    referents ("their"). Whether the condition held throughout is the engine's (ADR
    `semantics-v3.md` §0.1 item 4). -/
    | heldSince (condition : Condition) (since : GameEvent)
    | not (condition : Condition)
    | and (conjuncts : List Condition)
    | or (disjuncts : List Condition)

  inductive GameEvent where
    | inCaller (body : GameEvent)
    /-- An action event: active (`subj = .actor np`), unspecified-agent passive (`subj = .any`),
    or explicit-agent passive (`subj = .by np`). -/
    | does (subj : Subject) (action : Action)
    /-- The beginning of `part` of the turn `which` names, whose turn it is being `whose`'s. -/
    | beginningOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef)
    | endOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef)
    /-- The present moment, where a history window ends or a self-replacement applies. -/
    | now
    /-- A state trigger condition holding [CR#603.8]. -/
    | stateHolds (condition : Condition)
    /-- Saga chapter trigger mark [CR#714.2]. -/
    | chapterMark (chapters : List ChapterNumber)
    /-- "Whenever you cast your second spell each turn". "When the Nth counter is put" is no
    occurrence count but a threshold, `Amount.theNth` [CR#122.7]. -/
    | nthOccurrence (ordinal : Ordinal) (per : Option Period) (event : GameEvent)
    /-- "Whenever an ability you control causes you to…". -/
    | causes (cause : Causing) (event : GameEvent)
    /-- State-based actions check boundary for lookback [CR#704.3,704.5h]. -/
    | stateBasedActionsChecked

  inductive Causing where
    | source (source : NounPhrase)
    | event (event : GameEvent)
    | anEffect

  inductive FlipScope where
    | count (amount : Amount)
    | per (each : NounPhrase)

  inductive IgnoredOutcomes where
    | extreme (extreme : RollExtreme)
    | allBut (extreme : RollExtreme)
    | chosen (chooser : Option NounPhrase) (amount : Amount)

  inductive Ballot where
    | anyBallot
    | byLabel (options : List VoteLabel)
    | byCandidate (candidates : NounPhrase)

  inductive Exposed where
    | cards (cards : NounPhrase)
    | zone (zone : Location)
    | choice (sort : ChoiceSort)

  inductive VisibleThing where
    | topOfLibrary
    | wholeHand
    | objects (objects : NounPhrase)

  -- ## Trigger Headers and Bounds

  /-- How often something may be done: no more than `times` per `per` [CR#602.5b]. An event's
  ordinal ("the second spell each turn") is not a usage limit. -/
  inductive UsageLimit where
    | mk (times : Amount) (per : Period)

  /-- A second trigger header joined to the first with "or". -/
  structure JoinedHeader where
    event : GameEvent
    alternatives : List GameEvent
    while_ : Option Timing
    timing : Option Timing

  -- ## Actions, Static Specs, Costs, and Abilities

  /-- What mana may be spent on. -/
  inductive SpendPurpose where
    | toCast (predicate : Predicate)
    | toActivate (source : Option Predicate)
    | toPay (cost : CostNamed)
    /-- To turn a face-down permanent `subject` describes face up: the special action
    [CR#116.2b] whose cost is paid, a morph or disguise cost [CR#702.37e,702.168d] or a
    manifested or cloaked permanent's mana cost [CR#701.40b,701.58b]. -/
    | toTurnFaceUp (subject : Predicate)

  inductive AsThough where
    /-- "As though it didn't have defender": the rule applies as though `cond` held, its pronouns
    reading the deontic's own participants. -/
    | condition (cond : Condition)
    | mana (what : Option ColorOrColorless) (as_ : ManaMatch) (purpose : Option SpendPurpose)
    | greater (stat : Stat) (amount : Amount)

  inductive Exchanged where
    | lifeTotals (parties : NounPhrase)
    | controlOf (left right : NounPhrase)
    | cardsAcross (left right : NounPhrase)
    | zones (left right : Location)
    /-- Two numerical values [CR#701.12g]: life totals, powers or toughnesses, rolled results. -/
    | values (left right : Amount)

  /-- A Text Change: a specified rewrite of the subject's rules text, applied in layer 3
  [CR#612.1,613.1c]. Each row names the rewrite; the result is not checked again. -/
  inductive TextChange where
    /-- Removes all text found within square brackets in the subject's rules text, the brackets
    with their contents; the rest of the text is unchanged (cleave [CR#702.148a,702.148b]). The
    bracketed text is not marked in Core: lowering reads it from the authoring tree, where the
    `bracketed` template call survives. A text with no brackets is left as it was. -/
    | removeBracketed
    /-- Replaces every `target` determiner in the subject's rules text with `each`, keeping its
    description; nothing else changes (overload [CR#702.96a,702.96b]). -/
    | targetToEach
    /-- Appends the rules text of `source` after the subject's own, which it neither modifies nor
    replaces (splice [CR#702.47a,702.47c,612.10]). -/
    | appendTextOf (source : NounPhrase)
    /-- Replaces every instance of the word `from_`, used in its rules sense, with the word `to`
    of the same kind, the kind one of `sorts`; `none` is a word of the performer's choice, made
    as the effect applies [CR#612.2] ("one color word with another or one basic land type with
    another"). -/
    | replaceWord (sorts : List QualitySort) (from_ : Option TextWord) (to : Option TextWord)
    /-- Exchanges the rules text of the subject and `other`: each one's rules text becomes the
    other's previous rules text [CR#612.5,701.12h]. -/
    | exchangeWith (other : NounPhrase)

  /-- A characteristic value an effect writes that a `Characteristics` cannot list: every subtype
  of a space ("with all creature types") or a chosen quality ("of the chosen color"). A token's
  written characteristics carry it, and a Characteristic Edit writes it with its op. -/
  inductive WrittenQuality where
    | everyTypeOf (space : SubtypeSpace)
    | chosen (quality : Predicate)

  /-- Where a counter's kind comes from. -/
  inductive CounterKindSource where
    | printed (kind : CounterKind)
    | chosen (menu : List CounterKind)
    | distinctChosen (menu : List CounterKind)
    /-- The one kind a kind loop or a choice binds ("each kind of counter on it", doubling's
    "as many of that kind" [CR#701.10e]). -/
    | bound
    /-- The kind of the counters put or removed this way: "that many of those counters", and
    moving's "a counter" put where it was removed from [CR#122.5]. -/
    | those
    /-- One of each kind on `source`, the object or player itself or another: "a counter of each
    kind that's on this creature", proliferate's "each kind that permanent or player already
    has" [CR#701.34a]. -/
    | sameAs (source : NounPhrase)
    | anyKind

  /-- A mana cost a cost shift raises or lowers by: written ("{W}{B}"), or an object's mana cost
  ("the sacrificed permanent's mana cost" [CR#702.48a]). -/
  inductive ManaCostTerm where
    | lit (cost : ManaCost)
    | manaCostOf (object : NounPhrase)

  /-- A cost shift raises (`up`) or lowers (`down`) a cost; it never sets one, which is an
  alternative cost [CR#118.9] (`shiftSets`). -/
  inductive CostShift where
    /-- By an amount of generic mana: "costs {1} less", "costs {X} more". `floor`: a card-text
    floor on the mana left in the cost after this reduction ("This effect can't reduce the mana in
    that cost to less than one mana"; *Training Grounds*, *Power Artifact*). It counts all mana in
    the cost, colored included, not only the generic part this shift lowers: under *Training
    Grounds* {2}{G} becomes {G} and {2} becomes {1} (*Training Grounds* ruling, 2025-10-02). With
    no floor, only the rules floor of {0} applies [CR#601.2f]. -/
    | generic (change : Delta Amount) (floor : Option Amount)
    /-- By the symbols of a mana cost, each symbol of a type changing mana of that type.
    `coloredOnly` is "this effect reduces only the amount of colored mana you pay" (Edgewalker):
    a reduction that leaves generic mana alone; without it, colored mana in the cost beyond what
    the total cost has reduces generic mana [CR#702.48c,118.7]. -/
    | bySymbols (change : Delta ManaCostTerm) (coloredOnly : Bool)

  /-- The mana one act of a Mana Substitute pays for: one generic mana, or one mana of a color
  [CR#702.51a,702.66a,702.126a,701.67a]. -/
  inductive PipPaid where
    | generic
    | ofColor (color : ColorTerm)

  /-- A Deontic Exception: the one source a "can't" exempts. `byKeyword` names a keyword ability
  of the subject ("can't transform except due to its daybound ability" [CR#702.145b]);
  `byObjects` names the objects that are exempt, bare plural or quantified ("can't be blocked
  except by artifact creatures and/or black creatures" [CR#702.36b], "except by two or more
  creatures" [CR#702.111b]). -/
  inductive DeonticExcept where
    | byKeyword (label : KeywordLabel)
    | byObjects (objects : NounPhrase)

  /-- How much of a damage event a prevention or redirection takes: all of it, `amount` of it,
  the next `amount` damage (a shield, used up as it is applied [CR#615.7]), all but `amount` of
  it, or half of it, rounded as `rounding` says [CR#615.1,614.9]. -/
  inductive PreventCut where
    | all
    | some (amount : Amount)
    | shield (amount : Amount)
    | allBut (amount : Amount)
    | half (rounding : RoundMode)

  inductive DividedVerb where
    | damage (source : NounPhrase)
    | counters (kind : CounterKind)

  inductive ProducedMana where
    | runs (runs : List ProducedRun)
    | anyColor (freedom : ColorFreedom)
    | ofChosenColor (alternative : Option ProducedRun)
    | asPrintedCost (subject : NounPhrase)
    | producedByEvent (subject : NounPhrase)
    | couldProduce (subject : NounPhrase)
    | amongColorsOf (subject : NounPhrase)
    | amongWritten (colors : List Color)
    | lastNoted (subject : NounPhrase)

  inductive DeckTrait where
    | aCharacteristic (predicate : Predicate)
    /-- "cards with even mana values" [CR#202.3] -/
    | manaValueParity (parity : Parity)
    /-- "more than one of the same mana symbol in its mana cost" -/
    | repeatedManaSymbol
    /-- "has an activated ability" -/
    | hasAbilityOf (class_ : AbilityClass)
    /-- "... and land cards" -/
    | anyOf (traits : List DeckTrait)

  inductive DeckCondition where
    /-- "Each permanent card in your starting deck has mana value 2 or less." -/
    | everyCardIs (scope : Predicate) (trait : DeckTrait)
    /-- "No card in your starting deck has more than one of the same mana symbol in its mana
    cost." -/
    | noCardIs (scope : Predicate) (trait : DeckTrait)
    /-- "Each nonland card in your starting deck shares a card type." and "… has a different
    name.": an Agreement (`NounPhrase.agree`) over the starting deck's cards `group`
    describes. -/
    | groupAgrees (group : NounPhrase)
    /-- "at least twenty cards more than the minimum deck size", a minimum the format sets
    [CR#100.2a,100.2b]. -/
    | deckSizeOverMinimum (extra : Nat)

  /-- Combat participation is independent of the set of blocking relations. -/
  inductive CombatParticipation where
    | attacking (defender : Option NounPhrase)
    | blocked (value : Bool)
    | outsideCombat

  inductive CombatUpdate where
    | participation (state : CombatParticipation)
    /-- Adding a blocker makes its attacker blocked; removing the relation leaves that
    blockedness unchanged. -/
    | blocking (move : AttachMove) (attacker : NounPhrase)

  inductive Repetition where
    | again
    | moreTimes (times : Amount)
    | anyNumber
    | untilCond (condition : Condition)
    | againExcludingChosen
    | fixed (times : Amount) (body : Action)

  /-- An object's characteristics: name, mana cost, color and color indicator, card type,
  subtype, supertype, rules text and abilities, power, toughness, loyalty, and defense
  [CR#109.3]. Every field defaults to absent. -/
  structure Characteristics where
    name : Option String := none
    cost : Option ManaCost := none
    /-- A color indicator, or a token's colors; empty means "from the mana cost". -/
    colors : List Color := []
    supertypes : List Supertype := []
    types : List CardType := []
    subtypes : List Subtype := []
    text : List Ability := []
    power : Option Amount := none
    toughness : Option Amount := none
    loyalty : Option Amount := none
    defense : Option Amount := none

  /-- A characteristics set as an effect writes it, with the written qualities ("with every
  creature type") an effect can add [CR#111.3]. -/
  structure CharacteristicBundle where
    characteristics : Characteristics
    qualities : List WrittenQuality := []

  /-- The one face-down spec. `none` is a card exiled face down, which has no characteristics
  and no status [CR#406.3a,110.5d]; `some c` is a face-down spell or permanent, `c` its listed
  characteristics and so its copiable values [CR#708.2,613.2b]. Turning it face up is
  `setStatus faceUp`, and the values revert [CR#708.8]. Nothing face down is a token. -/
  structure FaceDown where
    characteristics : Option Characteristics

  /-- Typed characteristic writes; the enclosing static or copy form retains its context. -/
  inductive CharacteristicEdit where
    | typeLine (op : QualityOp) (changes : TypeLineChanges)
    | name (op : QualityOp) (name : String)
    | manaCost (cost : Option ManaCost)
    | colors (op : QualityOp) (colors : ColorSpec)
    /-- A stat set (`sets`), or raised (`adds`) or lowered (`loses`) by `amount`. In a static
    change of power and toughness, a set applies in layer 7b and a change by an amount in layer
    7c [CR#613.4b,613.4c]: "<subject> gets +P/+T" is two of these, the subject written once. A
    set of an object's own power or toughness by a characteristic-defining ability is written
    under `StaticSpec.characteristicDefining`, applied in layer 7a [CR#604.3a,613.4a]. -/
    | stat (op : QualityOp) (axis : CharacteristicStat) (amount : Amount)
    /-- The subject's power and toughness are switched [CR#613.4d]. -/
    | ptSwitch
    | quality (op : QualityOp) (quality : WrittenQuality)
    | addedAbilities (abilities : List Ability)
    /-- The abilities of `source` in the families `classes`, but for those `except` describes:
    "<subject> has all activated abilities of <source>". -/
    | addedAbilitiesOf (source : NounPhrase) (classes : List AbilityClass)
        (except : Option Predicate)
    /-- The abilities of `source` the selection picks (backup's "it gains the non-backup
    abilities of this creature printed below this one" [CR#702.165a]). -/
    | addedSelectionOf (source : NounPhrase) (selection : AbilitySelection)
    | removedAbilities (selection : AbilitySelection)

  inductive AbilitySelection where
    | specified (abilities : List AbilityLost)
    | family (class_ : AbilityClass)
    | allExcept (except : Option Predicate)
    /-- The abilities printed below `keyword`'s line, other than that keyword's own: backup's
    "the non-backup abilities of this creature printed below this one". Only printed abilities
    are selected, never gained ones [CR#702.165a,702.165c]. -/
    | printedAfter (keyword : KeywordLabel)

  inductive TokenSpec where
    /-- A token specification supplied by the immediate caller of a Definition. -/
    | inCaller (body : TokenSpec)
    /-- The creating spell or ability defines the token's characteristic values [CR#111.3] and
    sets its name and subtypes [CR#111.4]. -/
    | written (characteristics : CharacteristicBundle)
    | asThose
    | copyOf (source : NounPhrase) (exceptions : List CopyExcept)
    /-- A double-faced token: its front face and its back face [CR#111.10i,701.53b]. -/
    | doubleFaced (front : CharacteristicBundle) (back : CharacteristicBundle)

  /-- A prospective action clause ("if you would draw a card", "if this creature would be
  destroyed") for replacement effects [CR#614.1]. -/
  inductive WouldClause where
    | would (subj : Subject) (action : Action)
    | wouldAny (clauses : List WouldClause)
    /-- The named cause of a prospective event [CR#700.1,614.1a]. -/
    | causedBy (cause : Causing) (clause : WouldClause)
    /-- "The first time <clause> each turn": the `ordinal`th event `clause` matches in each
    `per`. The ordinal is the replaced event's, part of the particular event the effect watches
    for [CR#614.1], not a Usage Limit: matching events are counted over the span's whole
    history, so one that happened before the effect existed, or that the effect applied to and
    its controller declined to replace, still uses up "the first". -/
    | nthOccurrence (ordinal : Ordinal) (per : Option Period) (clause : WouldClause)

  /-- What a replacement effect makes of the event it replaces [CR#614.1]: an action in its
  place, an action done as part of it, or Entry Riders on the entering it is, each of which may
  apply only while a condition holds. -/
  inductive Replacement where
    /-- A replacement supplied by the immediate caller of a Definition. -/
    | inCaller (body : Replacement)
    /-- `action` happens in the event's place, the clause the word "instead" marks ("draw two
    cards instead", "you may instead mill …") [CR#614.1a,614.6]. -/
    | instead (action : Action)
    /-- `action` is done before the event, which still happens, modified by `body` if one is
    written. A choice a replacement of an entering requires is made before the permanent enters
    [CR#614.12a]: "As this enters, choose a color"; "As this creature enters, choose an
    opponent", then `body`, "That player may put an additional N +1/+1 counters on it as it
    enters" [CR#702.104a]; devour's sacrifice, then the counters it enters with [CR#702.82a]. So
    is the choice of what an Aura put onto the battlefield will enchant, made as it enters
    [CR#303.4f]. `body`'s action reads what `action` did. -/
    | before (action : Action) (body : Option Replacement)
    /-- The event still happens, and `action` is done as part of it ("enters with", "as it
    enters"): riot's and unleash's counter, "you may have this permanent enter with an additional
    +1/+1 counter on it" [CR#702.136a,702.98a], an Aura's attachment as it enters [CR#303.4f], or
    megamorph's counter on a permanent "as [it] is turned face up" [CR#702.37b,614.1e]. The
    constructor reads as "with"; `with` is a Lean keyword. -/
    | alongside (action : Action)
    /-- The replaced entering still happens, with `riders`: "this enters tapped", "enters with N
    counters", "enters as a copy of", "enters transformed" [CR#614.1c,614.1d,614.12]. Only an
    entering, a move to the battlefield or a token's creation, is ridden, and only by Entry
    Riders. -/
    | withRiders (riders : List MoveRider)
    /-- `body`, applying only while `condition` holds; otherwise the event happens unmodified,
    as in the self-replacement "draw a card. If you control three or more artifacts, draw two
    cards instead" [CR#614.15], or "if it is night, … it enters transformed" [CR#702.145b]. The
    condition is checked as the event would happen. One that decides whether the ability exists
    at all ("each Saga without read ahead has …" [CR#714.3a]) stays on the static ability
    (`StaticSpec.conditional`). -/
    | onlyIf (condition : Condition) (body : Replacement)

  /-- Optional modifiers on a deontic rule (`cant`, `can`, `must`, `gatedBy`)
  [CR#702.13b,702.111b,702.145b]: when or how often it applies (`timing`, "each turn", "during
  each of your turns"), what it applies as though, the one source it exempts, and, on a
  prohibition alone, the cause the prohibited event must have. -/
  structure DeonticMods where
    timing : Option Timing := none
    asThough : Option AsThough := none
    except : Option DeonticExcept := none
    /-- A cause-qualified prohibition: `cant` forbids only an event `cause` causes, as a
    would-clause's `WouldClause.causedBy` matches only one. "Damage doesn't cause you to lose
    life" is `cant you (loseLife anyAmount)` with the cause the damage dealt to you: it stops
    that result of the damage alone, the damage still being dealt and its other results still
    happening [CR#120.3a,120.3f]; with no cause, "you can't lose life" stops every loss of life
    [CR#119.8]. Only `cant` takes one (`deonticCause`). -/
    cause : Option Causing := none
    /-- A Permission Rider: the rest of the permission's own ability, the text after it in the
    same paragraph, not a separate ability (each paragraph break marks one [CR#113.2c]). It
    modifies the act `can` permits, carried out for an act performed under this permission and
    no other, and its pronouns read back from the permission's clause. "You may cast this spell
    as though it had flash. If you cast it any time a sorcery couldn't have been cast, the
    controller of the permanent it becomes sacrifices it at the beginning of the next cleanup
    step" is one ability: "if you cast it" is a cast made under that permission, its
    cast-timing check [CR#307.5a] reads that cast, and the ability, a static ability that
    functions on the stack, creates its delayed trigger as the permanent is put onto the
    battlefield [CR#608.3g]. A cast made under another effect's permission (Vedalken Orrery's)
    is not that cast, so nothing is sacrificed (Mystic Veil ruling, 2022-12-08). Only `can`
    takes one (`deonticRider`). -/
    rider : Option Action := none

  inductive StaticSpec where
    | inCaller (body : StaticSpec)
    | costShift (subject : NounPhrase) (shift : CostShift)
    /-- An alternative cost [CR#118.9]: the controller of a spell `subject` names may pay `cost`
    rather than its mana cost, or cast it without paying its mana cost when `cost` is `none`. It
    grants no cast; a cast from another zone or at another time is a `can` permission whose
    payment is `payingInstead` [CR#601.3]. A condition is the `onlyWhile` wrapper.
    Activating an ability follows casting's steps, its activation cost standing in for the mana
    cost [CR#602.2b], so `subject` may also name activated abilities ("you may pay {0} rather
    than pay cycling costs"). A cost paid while a triggered ability resolves, as echo's "unless
    you pay [cost]" is [CR#702.30a], is no alternative cost: a card that lets a player pay
    something in its place is spelled as a replacement of that payment (`replace` over
    `Action.pay`, whose `keyword` names the cost), not as an `altCost`. The checker does not
    police this: `altCost` refuses nothing for the ability class its subject names. -/
    | altCost (subject : NounPhrase) (cost : Option Cost)
    /-- An additional cost to cast the spell [CR#601.2f]: `offered` when the player may decline
    it ("you may pay"), and `times` how often it may be paid, as squad's and replicate's "any
    number of times" [CR#702.157a,702.56a]. -/
    | addedCost (cost : Cost) (offered : Bool) (times : PayTimes)
    | letterDefinition (letter : Letter) (amount : Amount)
    /-- Deontic prohibition ("can't") [CR#101.2]. -/
    | cant (subj : Subject) (action : Action) (mods : DeonticMods)
    /-- Deontic requirement ("must" / "if able"). -/
    | must (subj : Subject) (action : Action) (mods : DeonticMods)
    /-- Deontic permission ("can", "may play …") [CR#101.2]. "May" for an optional one-shot action
    is `Action.withContinuation .optional` [CR#603.5]. -/
    | can (subj : Subject) (action : Action) (mods : DeonticMods)
    /-- An indicative "as though" statement, "<subj> <action>s as though <asThough>": `action`
    happens, whenever it happens, as though `asThough` were true, for that event only
    [CR#609.4]. It permits nothing: "all damage is dealt as though its source had wither" is a
    statement about how all damage is dealt, not a "can" or "may" [CR#101.2], so no act it names
    is recorded as permitted and no "this way" reads it back. The condition's pronouns read the
    event's participants, as a deontic's "as though" does. -/
    | asThough (subj : Subject) (action : Action) (asThough : AsThough)
    /-- Cost-gated deontic rule ("can't … unless … pays …") [CR#508.1d,509.1c]. -/
    | gatedBy (cost : Cost) (subj : Subject) (action : Action) (mods : DeonticMods)
    | manaRetention (player : NounPhrase) (mana : ManaHeld)
    | partSkip (player : NounPhrase) (part : TurnPart)
    /-- A continuous change to `subject`'s characteristics: its stats ("gets +P/+T", "has base
    power and toughness P/T"), the abilities it has or gains, and the rest of its
    characteristics, each one Characteristic Edit, the subject written once. -/
    | characteristicChange (subject : NounPhrase) (edits : List CharacteristicEdit)
    /-- A Characteristic-Defining Ability: `subject`, the object itself, has the colors,
    subtypes, power or toughness `edits` define ("this creature's power is equal to …",
    "this object is every creature type", devoid's "this object is colorless") [CR#604.3a].
    Each edit is a color edit, a subtype edit (an every-type-of quality, a chosen subtype or a
    type line writing subtypes alone), or a power or toughness `sets`; a stat edit names its
    number, which a later edit reads back ("its toughness is equal to that number plus 1").
    Written as a printed
    static ability, unconditionally, and never inside a grant or an established effect: the
    checker refuses a subject other than the object itself, a condition, and a grant
    [CR#604.3a]. It stands on the card's own face, in a token's definition, or comes with a copy
    or a text change; a keyword whose definition writes one (changeling, devoid) has one where
    the keyword stands so, while another effect granting that keyword gives an ordinary
    continuous effect [CR#604.3a]. It functions in all zones and
    outside the game [CR#604.3]. Its effects apply first within their layer: a subtype edit in
    layer 4 and a color edit in layer 5, before the other effects there [CR#613.1d,613.1e,613.3];
    a power or toughness edit in layer 7a [CR#613.4a]. -/
    | characteristicDefining (subject : NounPhrase) (edits : List CharacteristicEdit)
    /-- "This effect doesn't remove <subject>": `spec`, a clause that removes Attachments already
    on its host, with `subject` exempt from that one state-based consequence of it
    [CR#702.16n,702.16p,303.4c,704.5m]. `spec` is a protection grant, whose Auras with the quality
    are put into their owners' graveyards and whose Equipment and Fortifications become
    unattached [CR#702.16c,702.16d,704.5m,704.5n], or a prohibition on attaching ("can't be
    enchanted"), which makes an Aura already attached enchant an illegal object
    [CR#303.4c,704.5m]. It wraps that clause alone, never a conjunction of clauses: *Guardian
    Beast*'s exemption reaches "can't be enchanted", not "have indestructible". `subject` is
    "this Aura", or a description of Attachments whose "attached to …" reads the clause's host
    back ("Auras already attached to those artifacts"). The checker refuses any other clause
    (`retentionClause`) and subject (`retentionSubject`).

    Applying the exemption is the engine's, as the zero-life exemption is: it covers this
    effect's removal and no other, so another protection from the same quality still removes the
    Aura [CR#702.16n,702.16p]. "Already attached to" names the Attachments attached when the
    effect starts to apply, not those attached later, which the effect keeps off [CR#702.16p];
    the subject is written as a description of the host's Attachments, and the engine reads it at
    that point in time. -/
    | retention (spec : StaticSpec) (subject : NounPhrase)
    | copyChange (subject : NounPhrase) (source : NounPhrase) (exceptions : List CopyExcept)
    | controlGrant (player : NounPhrase) (subject : NounPhrase)
    /-- A replacement effect: as `event` would happen, `replacement` modifies it ("If … would …,
    … instead", "this enters tapped", "as this enters, choose a color")
    [CR#614.1a,614.1c,614.1d]. Under `instead`, only when the action is wholly a `may` with no
    failure arm ("you may instead mill …") does declining it leave `event` to happen; otherwise
    that action is what happens in its place, a failure arm ("if you don't, put it into its
    owner's graveyard") included, so a `may` nested inside it spells out both arms. Under `before`
    and `alongside`, `event` happens whatever is chosen: declining a `may` does its failure arm
    if it has one (riot's "if you don't, it gains haste") and otherwise nothing. Under
    `withRiders`, the entering happens with the riders, and there is nothing to decline. Under `onlyIf`, `event`
    happens unmodified while the condition fails. -/
    | replace (event : WouldClause) (replacement : Replacement) (timing : Option Timing)
        (use : ReplUse) (limit : Option UsageLimit)
    | conditional (spec : StaticSpec) (condition : Condition) (marking : CondMarking)
    | visibility (verb : ExposeVerb) (player : NounPhrase) (what : VisibleThing)
    | additionalTriggers (event : GameEvent) (quantity : Quantity)
    | conjunction (subject : Option NounPhrase) (parts : List StaticSpec)
    /-- A Mana Substitute on a spell (convoke, delve, improvise): neither an additional nor an
    alternative cost, applied once the spell's total cost is locked
    [CR#702.51b,702.66b,702.126b]. -/
    | paySubstitute (subject : NounPhrase) (substitute : ManaSubstitute)
    /-- A special action: any time `player` has priority, within `when`, they may pay `cost` and
    do `does`, without using the stack [CR#116.1,116.2c]. With no `player` written, it is you:
    morph's turning face up, foretell's and plot's exiling are each one
    [CR#116.2b,116.2h,116.2k]. An effect may allow another player one ("unless that player
    pays {U} before that step") [CR#116.2c]. -/
    | specialAction (player : Option NounPhrase) (when : Option Timing) (cost : Cost)
        (does : Action)
    /-- An Off-Stack Static, "as [event], do": as `event` happens, `does` is carried out without
    using the stack, as a static ability is [CR#113.9]. It is for printed "As <event>, …" text.
    "As this enters, …" text is a replacement of the entering instead (`replace … (before …)` or
    `replace … (alongside …)`, [CR#614.1c]). -/
    | asEvent (event : GameEvent) (does : Action)
    /-- A Resolution Delayed Trigger: a static ability of a permanent spell that functions on the
    stack and creates a delayed triggered ability, `does`, written as the delayed trigger itself,
    bare or under a printed condition ("If this spell's dash cost was paid, return the permanent
    this spell becomes to its owner's hand at the beginning of the next end step"). The delayed
    trigger is created as the permanent is put onto the battlefield as the spell resolves
    [CR#608.3g], with the spell as its source and the spell's controller as it resolved as its
    controller [CR#603.7d]: a countered spell creates none, and a copy, which becomes a token
    permanent as it is put onto the battlefield [CR#608.3f], creates its own. It is its own
    ability, as dash's, blitz's and warp's are [CR#702.109a,702.152a,702.185a,113.2c], and so no
    Permission Rider; the permanent is named as the one the spell becomes (`becomes thisSpell`).
    The checker refuses a `does` that is not a delayed trigger (`createsDelayedTrigger`). -/
    | resolutionDelayedTrigger (does : Action)
    /-- An Off-Stack Static, "any time [condition] holds, do": whenever `condition` holds, `does`
    is carried out without using the stack, as a static ability is [CR#113.9,702.131b]. -/
    | whileHolds (condition : Condition) (does : Action)
    /-- A Roll Modifier, "after <roll>, <modify>": after the die `roll` names is rolled, and
    before its result is final, `modify` may change it [CR#706.2]. A modifier may be optional and
    may have a cost [CR#706.2a]; rerolls are considered before increases and decreases
    [CR#706.2b]. It is no triggered ability [CR#113.3c], which would resolve after the result was
    used. `limit` is a printed "only once each turn". -/
    | rollModifier (roll : GameEvent) (modify : Action) (limit : Option UsageLimit)
    /-- "<subject> is a Combatant": the creature-like combat rules apply to it. Its power deals
    combat and fight damage, marked damage is compared with its toughness, and it may be declared
    an attacker or a blocker [CR#120.3e,508.1a,509.1a,701.14a]. The role is a quality of a
    permanent, not an ability [CR#113.12], and the creature type definition confers it; whether
    the Combatant may take a given combat action is a separate deontic question. The
    summoning-sickness rule is one such question and no part of the role: the creature type
    definition confers it as two prohibitions of its own [CR#302.6]. -/
    | combatant (subject : NounPhrase)
    /-- A Mode Change: modifies how the modes of the spell or ability it is printed with are
    chosen [CR#700.2,702.42a,702.120a]. -/
    | modeChoice (change : ModeChange)
    /-- The spell gains a target, `description` saying what it may target, for as long as the
    static ability applies: an Aura spell always targets [CR#303.4a]. A target the spell requires
    only when an alternative or additional cost is chosen [CR#601.2c] is this form under a
    condition on that choice, as mutate's "if you do, it … targets a non-Human creature with the
    same owner as this spell" [CR#702.140a]. -/
    | addsTarget (subject : NounPhrase) (description : Predicate)
    /-- A Text Change of the subject's rules text, applied in layer 3 [CR#612.1,613.1c]. -/
    | textChange (subject : NounPhrase) (change : TextChange)
    /-- A cast-time option of a card in its owner's hand: as its owner casts `spell`, they may
    reveal this card [CR#601.2b]; if they do, `grants` applies to that spell, an additional cost
    and a text change (splice [CR#702.47a]). The cast is not replaced. Every card revealed this
    way for one spell is revealed at once, and the caster chooses the order in which their text
    follows the main spell's [CR#702.47b,702.47c]. -/
    | revealedAsCast (spell : NounPhrase) (grants : StaticSpec)

  /-- How a Mode Change modifies its own spell's `chooseModes`. -/
  inductive ModeChange where
    /-- "You may choose all modes instead": for `cost` (entwine [CR#702.42a]), or when
    `condition` holds ("if COND, you may choose both instead"). -/
    | allInstead (cost : Option Cost) (condition : Option Condition)
    /-- "For each mode you choose beyond the first, you pay an additional [cost]" (escalate
    [CR#702.120a]). -/
    | extraModesCost (cost : Cost)
    /-- The number of modes chosen: spree's one or more, tiered's one [CR#702.172a,702.183a]. -/
    | setQuantity (quantity : Quantity)
    /-- "You may choose the same mode more than once" [CR#700.2d]. -/
    | repeatable
    /-- "Choose one that hasn't been chosen [within]". -/
    | notChosenBefore (within : Duration)

  inductive PlayPayment where
    | itsOwnCost
    | withoutPaying
    | payingInstead (cost : Cost)
    /-- Pay the spell's mana cost or one alternative cost, chosen as it is cast [CR#601.2b], plus
    `cost` as an additional cost to cast it this way [CR#118.8,601.2f]: retrace's and
    jump-start's discard [CR#702.81a,702.133a]. -/
    | plusAdditional (cost : Cost)
    /-- Cast face down as `spec` says, paying `cost` rather than the mana cost: morph and
    disguise [CR#702.37a,702.168a,708.4]. -/
    | faceDown (spec : FaceDown) (cost : Cost)
    /-- "Using its <keyword> ability": the cast takes the alternative cost the keyword ability
    `label` names [CR#601.2b,601.2f], so that keyword's cost is paid, and its abilities that ask
    whether it was apply ("You may cast this card from your graveyard using its blitz ability"
    [CR#702.152a]). -/
    | byKeyword (label : KeywordLabel)

  /-- A move's rider. An Entry Rider says how a permanent arrives on the battlefield, from a
  move, a token's creation, or a static entry rider; a Pile Placement says how a moved card is
  placed in a zone other than the battlefield: its order in a pile, its face, and the counters
  it is exiled with. -/
  inductive MoveRider where
    | inCaller (body : MoveRider)
    | entersAs (status : Status)
    | entersAttacking (defender : Option NounPhrase)
    | entersTransformed
    | entersMelded (into : String)
    | withCounters (amount : Amount) (kind : CounterKindSource) (mark : EntryCounterMark)
    | under (controller : NounPhrase)
    | asCopyOf (optional : Bool) (source : NounPhrase) (exceptions : List CopyExcept)
    /-- Enters face down as `spec` lists, and with no other characteristics while it stays face
    down [CR#708.2,708.3]; or "exile it face down", `none` a card with no characteristics
    [CR#406.3,406.3a]. -/
    | faceDown (spec : FaceDown)
    /-- "In any order", "in a random order": how the objects moved to one place are ordered
    there [CR#401.4]. -/
    | inOrder (arrangement : Arrangement)
    /-- Enters attached to `host`, the object or player the effect names
    [CR#303.4i,301.5e]. An Aura that can't legally enchant `host`, or whose `host` is
    undefined, doesn't enter: enchant says so [CR#303.4i]. An Equipment or Fortification that
    can't legally be attached to it enters unattached [CR#301.5e,301.6], and so does a
    permanent that is none of the three [CR#303.4h]. In a pattern, `.attachedTo .any` matches
    any host the effect names, whether it is still defined or not, so an undefined host takes
    the [CR#303.4i] path. -/
    | attachedTo (host : NounPhrase)
    /-- In a pattern only: matches an entering that `rider` does not describe. A pattern's riders
    describe how the object would enter, whether or not an effect wrote the rider, and a listed
    rider matches when it describes the entering. Before [CR#303.4f]'s choice an entering is
    attached to a host only when its effect names it, so enchant's "doesn't specify the object
    or player the Aura will enchant" is `.not (.attachedTo .any)`. -/
    | not (rider : MoveRider)

  inductive Cost where
    /-- An unspecified cost in a payment pattern. -/
    | anyCost
    | inCaller (body : Cost)
    | mana (cost : ManaCost)
    | scaled (cost : Cost) (amount : Amount)
    | tapSymbol
    | untapSymbol
    /-- A loyalty symbol: put on (`up`, "[+1]", "[0]") or remove (`down`, "[−2]", "[−X]") that
    many loyalty counters [CR#606.4]; never set (`shiftSets`). -/
    | loyaltySymbol (change : Delta Amount)
    /-- An action performed as a cost ("Sacrifice a creature:") [CR#602.2b,118.1]. -/
    | perform (action : Action)
    | compound (costs : List Cost)
    | or (costs : List Cost)
    | itsManaCost
    /-- Pay `cost`, `substitute` standing in only for that cost's own generic mana
    (waterbend [CR#701.67b]). -/
    | withSubstitute (cost : Cost) (substitute : ManaSubstitute)

  /-- "For each [generic | colored] mana in the cost, you may <action> rather than pay that
  mana" [CR#702.51a,702.66a,702.126a,701.67a]: the action first, then the mana each act pays,
  read back from the acted-on object by anaphora ("that creature's color"). -/
  structure ManaSubstitute where
    action : Action
    pays : List PipPaid

  inductive ManaRider where
    | spendOnly (purposes : List SpendPurpose)
    | spendNotOn (purposes : List SpendPurpose)
    | onSpent (mode : SpentMode) (only : Bool) (spell : NounPhrase) (says : Action)

  inductive CopyExcept where
    | edits (edits : List CharacteristicEdit)
    | ability (ability : Ability)
    | thisAbility
    | entersWithCounters (amount : Amount) (kind : CounterKind) (mark : EntryCounterMark)

  structure RollRow where
    results : Quantity
    action : Action

  inductive CreationSpec where
    | token (spec : TokenSpec) (riders : List MoveRider)
    /-- An emblem, named where the rules name it ("an emblem named The Ring" [CR#701.54c]). -/
    | emblem (name : Option String) (abilities : List Ability)

  inductive Action where
    | inCaller (body : Action)
    /-- A `Definition` body read in its own Reference Scope (`enterOwnScope` / `leaveOwnScope`),
    carrying its optional `Deed` identity (`some (.action name)`), the one deed of the action: a
    resolving or paid step stamps it on what the body acts on, the event and deontic head gates
    admit it, and `does`/`would`/`did`/`cant` and composite action lowering read it
    [CR#701.22b]. A deed a permanent performs, written with no Handoff to a Permanent, is
    performed by the source permanent [CR#701.14a,701.44a]. -/
    | ownScope (deed : Option Deed) (body : Action)
    /-- Deal `amount` damage of `kind` to `recipient` (with the source supplied by the enclosing
    clause's subject or performer); `kind` is `none` for unqualified "damage". In a replacement's
    `instead` over a damage event it is that event modified [CR#614.6]: a recipient left
    unwritten (`any`) is the event's own, as in "it deals double that damage instead", and the
    damage keeps the event's kind. -/
    | dealDamage (amount : Amount) (recipient : NounPhrase) (kind : Option DamageKind := none)
    /-- Mark `amount` damage on `subject` [CR#120.3e], counterpart to `clearDamage` [CR#120.6]. -/
    | markDamage (subject : NounPhrase) (amount : Amount)
    /-- Prevent `cut` of a damage event [CR#615.1,615.1a]: the damage `damage` names, or, with
    none written, "that damage", the damage in scope. In a replacement's `instead` that is the
    damage the replacement watches ("If a source would deal damage to you, prevent 1 of that
    damage"); after a resolving `dealDamage` it is the damage that action deals ("The damage
    can't be prevented"). `also` is the prevention's additional effect, which takes place
    immediately after the prevention and may read the damage prevented [CR#615.5]. "Can't be
    prevented" is a prohibition over this action, `cant any (preventDamage .all …)`; prevention
    effects still apply to unpreventable damage, and their additional effects still happen
    [CR#615.12]. -/
    | preventDamage (cut : PreventCut) (damage : Option WouldClause) (also : Option Action)
    /-- Deal `cut` of a damage event to `to`, one permanent or player, instead: a redirection
    effect [CR#614.9]. The damage is the one `damage` names, or, with none written, the damage
    in scope, read as `preventDamage` reads it. A shield cut ("the next 2 damage") redirects
    only up to its amount, and the rest is dealt normally [CR#614.9,615.7]. -/
    | redirectDamage (cut : PreventCut) (to : NounPhrase) (damage : Option WouldClause)
    /-- Remove all damage marked on the permanent [CR#120.6]. -/
    | clearDamage (subject : NounPhrase)
    | setStatus (status : Status) (subject : NounPhrase)
    /-- Turn `subject` over to its other face; `into`, if written, is a characteristic the
    object has immediately after, as "transforms into" watches [CR#701.27e]. -/
    | turnOver (subject : NounPhrase) (into : Option Predicate)
    | combat (subject : NounPhrase) (update : CombatUpdate)
    /-- Declare as an attacker [CR#508.1]. -/
    | attack (defender : Option NounPhrase)
    /-- Declare as a blocker of `attacker` [CR#509.1]. -/
    | block (attacker : NounPhrase)
    /-- Attack and not be blocked [CR#509.1h]. -/
    | attackUnblocked (defender : Option NounPhrase)
    /-- Attack with `attackers` ("Whenever you attack with one or more creatures"). -/
    | attackWith (attackers : NounPhrase) (defender : Option NounPhrase)
    /-- A triggered ability triggers; its subject is the ability [CR#603.2]. -/
    | trigger
    /-- A creature is tapped to pay a Vehicle's crew cost [CR#702.122b,702.122d]. -/
    | crew (vehicle : NounPhrase)
    /-- A creature is tapped to pay a permanent's saddle cost [CR#702.171c]. -/
    | saddle (permanent : NounPhrase)
    /-- Target `subject` [CR#115.1]. -/
    | target (subject : NounPhrase)
    /-- Tap `permanent` for mana [CR#106.12a]. -/
    | tapForMana (permanent : NounPhrase) (type : Option ManaTypeTerm)
    /-- Commit a crime [CR#700.13]. -/
    | commitCrime
    | attachment (move : AttachMove) (subject : NounPhrase) (host : Option NounPhrase)
    /-- A player gains control of `subject`, from `from_` if written ("from you"): the control
    change an event, a lookback or a deontic names. A trigger on it looks back in time
    [CR#603.10d] ("whenever an opponent gains control of a permanent from you"); a deontic forbids
    it ("can't gain control of them"). It is admitted only there
    (`controlChangePattern`): a resolving "gain control of" is a Continuous Effect,
    `establish (controlGrant …)` [CR#611.2a,613.1b]. -/
    | changeControl (subject : NounPhrase) (from_ : Option NounPhrase)
    | gainDesignation (subject : NounPhrase) (designation : DesignationLabel)
        (duration : Option Duration)
    /-- The game gains `designation`, one the game itself can have: "it becomes night"
    [CR#731.1]. -/
    | gameGains (designation : DesignationLabel)
    | unlock (door : Door)
    /-- A Room gains its remaining unlocked designation, or both at once [CR#709.5i]. -/
    | fullyUnlock (room : NounPhrase)
    /-- Win, lose, or draw the game [CR#104.4c]; `cause` the one state-based cause it happens
    for, if any: "lose the game for having 0 or less life" [CR#704.5a]. -/
    | conclude (verb : OutcomeVerb) (cause : Option StateBasedCause := none)
    | restartGame
    | separateIntoPiles (group : NounPhrase) (piles : Nat) (faces : List PileFace)
    /-- The `when` rider is the printed "as you activate this ability": it times the choice by
    its description, comparisons included ("target opponent who has more life than you do as you
    activate this ability"). How activation and resolution evaluate that description
    [CR#601.2c,608.2b] is the engine's. -/
    | choose (first : Option NounPhrase) (chosen : NounPhrase) (disclosure : Disclosure)
        (when : Option Timing)
    | revealChoices (sort : HiddenSort)
    /-- Vote [CR#701.38a]; `times` how many votes, none for one ("vote an additional time"). -/
    | vote (first : Option NounPhrase) (disclosure : Disclosure) (ballot : Ballot)
        (times : Option Quantity)
    /-- Move `subject` from `from_` to `to`, observed at `at_` [CR#603.6a,603.6c,603.10a,700.4]. -/
    | move (subject : NounPhrase) (from_ : Location) (to : Option Location)
        (at_ : ObservationPoint) (riders : List MoveRider)
    /-- Turn `subject` face down as `spec` lists: manifest's first step [CR#701.40a,708.2]. -/
    | turnFaceDown (subject : NounPhrase) (spec : FaceDown)
    | copy (sort : CopySort) (subject : NounPhrase) (times : Amount)
        (exceptions : List CopyExcept)
    | chooseNewTargets (subject : NounPhrase)
    | copyTargets (copy : NounPhrase) (original : NounPhrase)
    | changeLife (delta : Delta Amount)
    | exchange (exchanged : Exchanged)
    | addMana (amount : Amount) (produced : ProducedMana) (riders : List ManaRider)
    /-- Draw as many cards as `quantity` says: exactly N, up to N (the performer choosing how
    many), or, under a prohibition, more than N [CR#121.1,121.2]. -/
    | draw (quantity : Quantity)
    | expose (verb : ExposeVerb) (exposed : Exposed)
    /-- Search `scope` for what `for_` describes; with no "for …" written, `for_` is `any`
    (the `search` Definition's default) [CR#701.23a,701.23d]. -/
    | search (scope : Location) (for_ : NounPhrase)
    /-- Randomize the objects in a pile so no player knows their order [CR#701.24a]. -/
    | shuffle (pile : Location)
    | flipCoins (count : FlipScope) (call : Option FlipCall) (outcome : Option CoinFace)
    /-- Roll `count` dice. As an event pattern, `count` is what the text watches: "roll a die"
    is `.lit 1`, matched by each die rolled, since one event can contain several occurrences
    [CR#603.2c]; "roll one or more dice" is `.anyAmount`, matched by the roll. -/
    | rollDice (count : Amount) (sides : DieSides) (watch : RollWatch)
    | applyResultsTable (rows : List RollRow)
    | ignoreOutcomes (which : IgnoredOutcomes)
    /-- Raise or lower a die result by one of `changes`, the performer's choice: "increase or
    decrease the result by 1" lists both [CR#706.2]. Never a set (`shiftSets`). -/
    | shiftResult (changes : List (Delta Amount))
    | storeResults (on : NounPhrase)
    | rerollStored (quantity : Quantity) (whose : NounPhrase)
    | establish (spec : StaticSpec) (duration : Option Duration)
    /-- Create `count` objects as `spec` says. Whether an effect creates them follows from the
    enclosing node; "if an effect would create" is `WouldClause.causedBy .anEffect`
    [CR#609.1,614.16]. -/
    | createObject (count : Amount) (spec : CreationSpec) (under : Option NounPhrase)
    /-- Put `amount` counters on `on`. As an event pattern, `amount` is what the text watches:
    "a counter is put" is `.lit 1`, matched by each counter put [CR#603.2c]; "one or more
    counters are put" is `.anyAmount`, matched by the event [CR#714.2b]; "the Nth counter is put"
    is `.theNth`, matched by the put that crosses N [CR#122.7]. "If an effect would put" is
    `WouldClause.causedBy .anEffect` [CR#609.1,614.16]; whether a put is an effect follows from
    its enclosing node, and a turn-based action is no effect [CR#703.1]. -/
    | putCounters (amount : Amount) (kind : CounterKindSource) (on : NounPhrase)
    | distribute (verb : DividedVerb) (amount : Amount) (among : NounPhrase)
    /-- Remove `quantity` counters from `from_`, all of them if none is written. As an event
    pattern, `quantity` is what the text watches: "a counter is removed" is `exactly 1`, matched
    by each counter removed [CR#603.2c]; "one or more" is `orMore 1`, matched by the event; "the
    last" is `theLast` [CR#702.62a,310.12b]. -/
    | removeCounters (quantity : Option Quantity) (kind : Option CounterKindSource)
        (from_ : NounPhrase)
    /-- Move `amount` counters of `kind` from `source` onto `destination`: remove them from the
    one and put them onto the other, publishing both outcomes [CR#122.5]. With no kind written,
    what is put is the counters removed (`those`). The move is all or nothing: if either half
    isn't possible (the same object, no such counter on the source, a destination that can't
    have counters put on it, either object gone from its zone), no counter is removed or put
    [CR#122.5], as an exchange happens whole or not at all [CR#701.12a]; that is the engine's to
    execute. -/
    | moveCounters (amount : Amount) (kind : Option CounterKindSource)
        (source destination : NounPhrase)
    | pay (cost : Cost) (times : PayTimes) (outcome : PaymentOutcome)
        (whose : Option NounPhrase) (keyword : Option KeywordLabel)
    /-- 3rd-person imperative handoff: `subj`, a player or a permanent, performs `action`
    ("target player discards a card", "target creature explores" [CR#701.44a]). Inside `action`,
    `NounPhrase.actor` is that performer. -/
    | instruct (subj : NounPhrase) (action : Action)
    /-- Branch on the decision or start of payment, independently of resulting events. -/
    | withContinuation (policy : ContinuationPolicy) (body : Action)
        (ifDid : Option Action) (ifNot : Option Action)
    /-- "<action>. Do this only once each turn.": the limit counts `action` taken, not the
    triggerings or resolutions of the ability that carries it. That ability triggers only if its
    source's controller has not yet taken `action` within `limit`, so declining a "may" leaves it
    free to trigger again [CR#603.2h]. -/
    | limited (action : Action) (limit : UsageLimit)
    /-- "If <condition>, <action>" or, `postposed`, "<action> if <condition>": the condition is
    checked as the spell or ability resolves, before `action` is taken [CR#608.2c]. A postposed
    condition is printed after its clause, so its anaphora read that clause's mentions as the
    clause ranks them; the instructions are read as a whole text by the rules of English
    [CR#608.2c]. -/
    | doIf (condition : Condition) (action : Action) (otherwise : Option Action)
        (postposed : Bool := false)
    | doForEach (group : NounPhrase) (body : Action)
    | doForEachKind (axis : KindAxis) (domain : Option NounPhrase) (sort : QualitySort)
        (body : Action)
    | repeat_ (repetition : Repetition)
    | sequentially (steps : List Action)
    | simultaneously (steps : List Action)
    /-- "Choose <quantity> —" over bulleted modes [CR#700.2]: each mode is its cost (spree's and
    tiered's priced bullets [CR#700.2h]), its weight (a pawprint mode's count of {P}
    [CR#700.2i]) and its action. -/
    | chooseModes (quantity : Quantity) (modes : List (Option Cost × Option Nat × Action))
    /-- Create a delayed triggered ability that does `body` the next time `event` (or one of
    `alternatives`) occurs, or, with a stated `duration`, each time within it
    [CR#603.7a,603.7b]. No text mentions the ability created, so it is read only by its Binding
    Identity, `delay.trigger` (`delayUnlessPaid`). -/
    | delay (event : GameEvent) (alternatives : List GameEvent) (duration : Option Duration)
        (body : Action)
    /-- A self-replacement: the spell's or ability's own `replaced` action, replaced by
    `replacement` [CR#614.15,616.1a]. -/
    | replace (replaced : Action) (replacement : Replacement)
    | holdUntil (action : Action) (event : GameEvent)
    /-- A reflexive triggered ability created by `body` [CR#603.12]: with no `event`, it triggers
    "when [the performer does]" what `body` offers; with `some ev`, "when [ev] this way". -/
    | triggerReflexively (body : Action) (event : Option GameEvent) (trigger : Action)
    | skipUntap (subject : NounPhrase) (steps : Amount)
    | skipPart (part : TurnPart) (count : Amount)
    | addTurnPart (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
        (followedBy : Option TurnPart)
    /-- Carry out a rules procedure on `subject`: activate it, cast it, or play it
    [CR#701.2a,701.5a,701.18a], paying as `payment` says. `resulting` is what the spell the
    procedure produces must be ("if the resulting spell's mana value is N or less"
    [CR#701.57a]). -/
    | procedure (kind : Procedure) (subject : NounPhrase) (payment : PlayPayment)
        (resulting : Option Predicate) (from_ : Option Location) (of_ : Option NounPhrase)
    /-- Make `subject` stand in the Relation `label` with `counterpart`: "exile it haunting
    target creature", "pair this creature with another", "encoded on a creature"
    [CR#702.55b,702.95b,702.99b]. -/
    | relate (subject : NounPhrase) (label : RelationLabel) (counterpart : NounPhrase)
    /-- Merge `subject` with the permanent `onto`, placed at `position`: the permanent becomes
    a merged permanent represented by both [CR#730.2]. -/
    | merge (subject : NounPhrase) (onto : NounPhrase) (position : MergePosition)

  inductive KeywordParam where
    | cost (cost : Cost)
    /-- A description printed after a marker or bound to the keyword: "protection from red",
    "islandwalk" [CR#702.16a,702.14a]. A declaration writes it `Quality`. -/
    | quality (predicate : Predicate)
    /-- A description printed as a nominal: "Enchant creature you control", "Champion a Kithkin"
    [CR#702.5a,702.72a]. It is a description, not a referent, so a declaration writes it
    `Predicate`, never `Subject` (a `NounPhrase`). -/
    | subject (predicate : Predicate)
    | number (amount : Amount)
    | deckCondition (condition : DeckCondition)
    /-- The ability printed with the keyword ("Boast — [Cost]: [Effect]"), which the keyword's
    definition restricts (`docs/decisions/semantics-v2.md` §7, ruling 2026-10-09). -/
    | ability (ability : Ability)

  inductive AbilityLost where
    | written (ability : Ability)
    | term (keyword : KeywordTerm)

  inductive Ability where
    /-- An ability supplied by the immediate caller of a Definition: the ability a keyword
    qualifies (boast's, exhaust's, solved's), read in its card's Reference Scope. -/
    | inCaller (body : Ability)
    /-- A keyword ability [CR#702.1]. `body` is the keyword's definition, held the way a text
    box holds rules text [CR#109.3]: empty where the card writes the keyword bare, and otherwise
    the abilities the keyword represents. -/
    | keyword (keyword : KeywordLabel) (params : List KeywordParam) (body : List Ability)
    | activated (cost : Cost) (action : Action) (timing : Option Timing)
        (limit : Option UsageLimit) (guard : Option Condition) (activator : Option NounPhrase)
    | triggered (event : GameEvent) (alternatives : List GameEvent)
        (while_ : Option Timing) (joins : List JoinedHeader) (timing : Option Timing)
        (limit : Option UsageLimit) (intervening : Option Condition) (action : Action)
    | static (spec : StaticSpec)
    | spell (timing : Option Timing) (action : Action)
    /-- "You may begin the game with this on the battlefield" (the Leylines). -/
    | mayBeginOnBattlefield
    /-- An italic head before an ability: an ability word or a flavor word. -/
    | italicHead (word : ItalicWord) (ability : Ability)
    /-- "that ability", read off an ability chosen earlier in the same text. -/
    | thatAbility (ref : ChoiceRef)
    /-- `ability` with the restrictions a keyword or a granting effect adds to it: when it may
    be activated or trigger (`timing`), how often (`limit`), the condition it is subject to
    (`guard`) and the zone it functions from (`from_`) [CR#113.10a,113.6b]. The guard reads by
    the wrapped ability's category: "as long as" for a static ability, "this ability triggers
    only if" for a triggered one, "activate only if" for an activated one
    [CR#702.169b..702.169d]. -/
    | qualified (ability : Ability) (timing : Option Timing) (limit : Option UsageLimit)
        (guard : Option Condition) (from_ : Option Zone)
    /-- One of two abilities by the Card Class of the card that has it: `onPermanent` on a
    permanent card, `onInstantOrSorcery` on an instant or sorcery card [CR#110.4,110.4a]. A
    keyword the rules define one way "on a permanent" and another "on an instant or sorcery
    spell" writes its two halves here [CR#702.55a,702.131a,702.131b,702.174b]. The checker checks
    both, each in the same incoming context; the engine keeps the one the card's Card Class
    (`cardClassOf`, read off the card types' fields) selects and the card has only that one. -/
    | byCardClass (onPermanent : Ability) (onInstantOrSorcery : Ability)
end

/-! The Coordinations as their Coordinator spells them, usable as patterns. -/

@[match_pattern] abbrev Location.or (locs : List Location) : Location := .coord .or locs
@[match_pattern] abbrev Predicate.and (conjuncts : List Predicate) : Predicate := .coord .and conjuncts
@[match_pattern] abbrev Predicate.or (disjuncts : List Predicate) : Predicate := .coord .or disjuncts
@[match_pattern] abbrev NounPhrase.and (phrases : List NounPhrase) : NounPhrase := .coord .and phrases
@[match_pattern] abbrev NounPhrase.or (phrases : List NounPhrase) : NounPhrase := .coord .or phrases

/-- The positive locations `loc` names, as a list of its disjuncts, or none for a complement
("anywhere other than …") or anywhere; a "from anywhere" move is never a leaves-the-battlefield
trigger [CR#603.6c]. Callers decide whether a move is observed on departure [CR#603.10a]
(`Location.observedOnDeparture`). -/
def Location.positiveZones? (loc : Location) (fuel : Nat := 16) : Option (List Location) :=
  match fuel, loc with
  | 0, _ => none
  | n + 1, .inCaller body => body.positiveZones? n
  | _ + 1, .zone .. | _ + 1, .fromEnd .. | _ + 1, .exiledWith _ | _ + 1, .pile _ => some [loc]
  | n + 1, .coord _ locs => locs.foldr (fun l acc =>
      match l.positiveZones? n, acc with
      | some ls, some rest => some (ls ++ rest)
      | _, _ => none) (some [])
  | _ + 1, .anyLocation | _ + 1, .not _ => none
termination_by structural fuel

instance : Coe Condition AsThough where
  coe := .condition

end Semantics

attribute [semantic_expression] Semantics.GameEvent Semantics.NounPhrase Semantics.Predicate
  Semantics.Amount Semantics.Quantity Semantics.Location Semantics.Condition Semantics.Duration
  Semantics.Timing Semantics.Subject Semantics.LookbackSpan Semantics.WouldClause
  Semantics.Replacement Semantics.Action Semantics.StaticSpec Semantics.Cost Semantics.Ability
  Semantics.TokenSpec

classify_semantic_syntax

-- A keyword ability is written through its typed Definition in `Semantics.Definitions.Keywords`,
-- whose signature fixes its parameters; `Ability.keyword` is that Definition's expansion carrier.
attribute [internal_expansion] Semantics.Ability.keyword

attribute [internal_expansion] Semantics.NounPhrase.pro Semantics.NounPhrase.which
  Semantics.NounPhrase.firstMention Semantics.NounPhrase.laterMention
  Semantics.Quantity.inCaller Semantics.Location.inCaller Semantics.Condition.inCaller
  Semantics.NounPhrase.inCaller Semantics.GameEvent.inCaller Semantics.Amount.inCaller
  Semantics.Predicate.inCaller Semantics.Action.inCaller
  Semantics.Action.ownScope Semantics.Cost.inCaller Semantics.StaticSpec.inCaller
  Semantics.Replacement.inCaller Semantics.Ability.inCaller Semantics.TokenSpec.inCaller
  Semantics.MoveRider.inCaller

attribute [semantic_literal] Semantics.Amount.lit

/-- A numeral is a literal Amount: `3` for `.lit 3`. The numeral stays an `OfNat.ofNat` term
that `Authoring.inspect` reads through to this literal leaf. -/
instance (n : Nat) : OfNat Semantics.Amount n := ⟨.lit (Int.ofNat n)⟩

namespace Semantics

/-- The one zone a location is in, if it names one: a disjunction names a zone only when all its
disjuncts name the same one ("on the top or bottom of your library"). -/
def Location.sort (z : Location) (fuel : Nat := 16) : Option Zone :=
  match fuel, z with
  | 0, _ => none
  | n + 1, .inCaller body => body.sort n
  | _ + 1, .zone zn _ => some zn
  | n + 1, .fromEnd _ _ p => p.sort n
  | _ + 1, .exiledWith _ => some .exile
  | n + 1, .coord _ (first :: rest) =>
    let zone := first.sort n
    if zone.isSome && rest.all (fun loc => loc.sort n == zone) then zone else none
  | _ + 1, .coord _ [] | _ + 1, .pile _ | _ + 1, .anyLocation | _ + 1, .not _ => none
termination_by structural fuel

/-- Whether a move from `from_` to `destination`, as a pattern, is observed on its departure.
Some zone-change triggers look back in time: leaves-the-battlefield abilities, abilities that
trigger when a card leaves a graveyard, and abilities that trigger when an object all players can
see (here, from exile, the stack or the command zone) is put into a hand or library [CR#603.10a].
The pattern's written origin decides, not an individual occurrence's: a positive origin list is
observed on its departure when any of its origins is, since an ability that triggers when a
permanent moves from the battlefield is a leaves-the-battlefield ability whatever else it watches
[CR#603.6c]. A move from anywhere is never a leaves-the-battlefield ability [CR#603.6c]; it, and a
move from a complement, is observed on its arrival.
The put and return Templates write this classification into the move, and the checker reads a
departure-observed move's battlefield patient as the permanent it was. -/
def Location.observedOnDeparture (from_ destination : Location) : Bool :=
  match from_.positiveZones? with
  | some zones => zones.any fun zone =>
      zone.sort == some .battlefield || zone.sort == some .graveyard ||
        ((destination.sort == some .hand || destination.sort == some .library) &&
          (zone.sort == some .exile || zone.sort == some .stack || zone.sort == some .command))
  | none => false

end Semantics
