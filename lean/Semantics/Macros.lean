import Semantics
import Semantics.Macros.Primitives
import Semantics.Check
import Semantics.CallerScope

/-!
# Semantics.Macros

Spellings over the raw constructors for `semantics-v3` (`docs/decisions/semantics-v3.md` §§1–3).
Rules-defined terms (`Definition`s, including all CR §701 keyword actions and CR §702 keyword
abilities) evaluate in their own Reference Scope (`ownScope` + `inCaller`), while phrasing
helpers (`Template`s) expand transparently in the caller's DRS scope and use
`firstMention` / `laterMention` whenever a `NounPhrase` parameter is mentioned more than once
before a zone change.
-/

namespace Semantics.Macros

/-! ## Pronouns -/

/-- "it" -/
semantic_template it : NounPhrase := .pro .bare .one

/-- "them" -/
semantic_template them : NounPhrase := .pro .bare .many

/-- "that card": the card slot's occupant. -/
semantic_template itCard : NounPhrase := .pro (.atSlot .card) .one

/-- "it": the permanent slot's occupant, as "sacrifice it" reads it. -/
semantic_template itPermanent : NounPhrase := .pro (.atSlot .permanent) .one

/-- "that token": the token just created. -/
semantic_template itAsToken : NounPhrase := .pro .tokenBorn .one

/-- "itself" / "himself" / "herself" -/
semantic_template itself : NounPhrase := .refl .one

/-- "themselves" -/
semantic_template themselves : NounPhrase := .refl .many

/-- "that <word>", e.g. `that .player`. -/
semantic_template that (w : NounWord) : NounPhrase := .pro (.word w) .one

/-- "that player or planeswalker": the joined phrase just named. -/
semantic_template thatJoin : NounPhrase := that .join

/-- "those <word>s", e.g. `those .card`. -/
semantic_template those (w : NounWord) : NounPhrase := .pro (.word w) .many

/-- "they": the player most recently named. -/
semantic_template they : NounPhrase := .pro (.word .player) .one

/-- "that player or that permanent's controller": the player half of a "target player or
planeswalker" split, or the controller of the permanent half. -/
semantic_template splitOverPlaneswalker : NounPhrase :=
  Primitives.NounPhrase.eitherOf (.pro (.unionHalf .player) .one)
    (.possessorOf .controller (.pro (.unionHalf (.type .planeswalker)) .one))

/-- The same over "any target": the player, or the permanent's controller. -/
semantic_template splitOverPermanent : NounPhrase :=
  Primitives.NounPhrase.eitherOf (.pro (.unionHalf .player) .one)
    (.possessorOf .controller (.pro (.unionHalf .permanent) .one))

/-- "the <verb>ed <word>", e.g. "the tapped creatures"; with no word, the pronoun the deed
stamped: "it" or "them", "the exiled card". -/
semantic_template theVerbed (verb : Deed) (plurality : Plurality) (word : Option NounWord := none)
    (marking : VerbedMarking := .attributive) : NounPhrase :=
  .pro (.verbed verb word marking) plurality

/-! ## Quantities -/

semantic_template exactly (n : Nat) : Quantity := .range (some n) (some n)
semantic_template upTo (n : Nat) : Quantity := .range none (some n)
semantic_template anyNumber : Quantity := .range none none
semantic_template atLeast (n : Nat) : Quantity := .range (some n) none
semantic_template oneThrough (n : Nat) : Quantity := .range (some 1) (some n)

/-! ## Determiners -/

/-- "target …" -/
semantic_template target (p : Predicate) : NounPhrase := .described (.target (.range (some 1) (some 1))) p

/-- "each …" -/
semantic_template each (p : Predicate) : NounPhrase := .described .each p

/-- "all …" -/
semantic_template allOf (p : Predicate) : NounPhrase := .described .all p

/-- the bare plural: "creatures" -/
semantic_template bare (p : Predicate) : NounPhrase := .described .bare p

/-- "a …" -/
semantic_template a (p : Predicate) : NounPhrase := .described (.a .unmarked) p

/-- "a … at random" -/
semantic_template aAtRandom (p : Predicate) : NounPhrase := .described (.a .atRandom) p

/-- "a … of their choice", the chooser being the last-mentioned player. -/
semantic_template aTheirChoice (p : Predicate) : NounPhrase := .described (.a .theirChoice) p

/-- "a … of your choice" -/
semantic_template aYourChoice (p : Predicate) : NounPhrase := .described (.a .yourChoice) p

/-- "the …" -/
semantic_template the (p : Predicate) : NounPhrase := .described .the p

/-- "N …", "up to N …" -/
semantic_template counted (q : Quantity) (p : Predicate) : NounPhrase := .described (.count q none) p

/-- "N <p> at random" -/
semantic_template countedAtRandom (q : Quantity) (p : Predicate) : NounPhrase :=
  .described (.count q (some .atRandom)) p

/-- "if there is a …" -/
semantic_template exists_ (p : Predicate) : Condition := .exists_ (bare p)

/-- "the number of …s" -/
semantic_template countOf (p : Predicate) : Amount := .countOf (bare p)

/-- "the greatest power among …s" -/
semantic_template aggregate (op : AggregateOp) (axis : ProjAxis) (p : Predicate) : Amount :=
  .aggregate op axis (bare p)

/-! ## Zones -/

semantic_template battlefield : ZoneExpr := .zone .battlefield .bare
semantic_template exileZone : ZoneExpr := .zone .exile .bare
semantic_template hand : ZoneExpr := .zone .hand .bare
semantic_template graveyard : ZoneExpr := .zone .graveyard .bare
semantic_template stack : ZoneExpr := .zone .stack .bare
semantic_template command : ZoneExpr := .zone .command .bare
/-- "the command zone", scoped to nobody. -/
semantic_template commandZone : ZoneExpr := .zone .command .bare
semantic_template yourLibrary : ZoneExpr := .zone .library (.possessedBy .you)
/-- "<player>'s library": a face-down pile [CR#401.2]. -/
semantic_template library (player : NounPhrase) : Pile := .library (.possessedBy player)
/-- "on the bottom of its owner's library" -/
semantic_template onBottom : ZoneExpr := .place .bottom (.library .bare)
/-- "on top of its owner's library" -/
semantic_template onTop : ZoneExpr := .place .top (.library .bare)
semantic_template handOf (player : NounPhrase) : ZoneExpr := .zone .hand (.possessedBy player)
/-- "your hand" -/
semantic_template yourHand : ZoneExpr := handOf .you
semantic_template graveyardOf (player : NounPhrase) : ZoneExpr := .zone .graveyard (.possessedBy player)

/-! ## Predicates -/

semantic_template creature : Predicate := .hasType .creature
semantic_template artifact : Predicate := .hasType .artifact
semantic_template enchantment : Predicate := .hasType .enchantment
semantic_template land : Predicate := .hasType .land
semantic_template instant : Predicate := .hasType .instant
semantic_template sorcery : Predicate := .hasType .sorcery
semantic_template instantOrSorcery : Predicate := .or [instant, sorcery]
/-- "has a permanent type": an artifact, battle, creature, enchantment, land, or planeswalker
[CR#110.4]; read off the types, so it holds of a card or a token. -/
semantic_template permanentType : Predicate :=
  .or [ .hasType .artifact, .hasType .battle, .hasType .creature, .hasType .enchantment,
        .hasType .land, .hasType .planeswalker ]
semantic_template tapped : Predicate := .hasStatus .tapped
semantic_template faceDown : Predicate := .hasStatus .faceDown
/-- "another player": any player other than you. -/
semantic_template otherPlayer : Predicate := .and [.anyPlayer, .otherThan .you]
/-- "spell": an object on the stack that is not an ability [CR#112.1,113.1]. -/
semantic_template spell : Predicate := .and [.inZone stack, .not (.abilityHead .anyOnStack)]
semantic_template untapped : Predicate := .hasStatus .untapped
semantic_template nontoken : Predicate := .not .isToken
/-- "permanent": an object on the battlefield [CR#110.1]. -/
semantic_template permanent : Predicate := .inZone battlefield
/-- "permanent card": a card that could be put onto the battlefield, one with an artifact,
battle, creature, enchantment, land, or planeswalker type [CR#110.4a]; a reading of the card's
types, wherever it is. -/
semantic_template permanentCard : Predicate :=
  .and
    [ .isCard,
      .or
        [ .hasType .artifact, .hasType .battle, .hasType .creature, .hasType .enchantment,
          .hasType .land, .hasType .planeswalker ] ]
/-- "colorless": an object of no color [CR#105.2c]. -/
semantic_template colorless : Predicate := .colorCount .eq 0
/-- "multicolored": an object of two or more colors [CR#105.2b]. -/
semantic_template multicolored : Predicate := .colorCount .atLeast 2
/-- "monocolored" -/
semantic_template monocolored : Predicate := .colorCount .eq 1
/-- "historic": an artifact, legendary, or Saga [CR#700.6]. -/
semantic_template historic : Predicate :=
  .or [.hasType .artifact, .hasSupertype .legendary, .hasSubtype (.of .enchantment "Saga")]
semantic_template attacking : Predicate := .inCombat .attackerOf none
semantic_template blocking : Predicate := .inCombat .blockerOf none
semantic_template blocked : Predicate := .inCombat .blockedBy none
semantic_template unblocked : Predicate := .not blocked
semantic_template creatureYouControl : Predicate := .and [creature, .hasPossessor .controller .you]
semantic_template creatureYouDontControl : Predicate := .and [creature, .not (.hasPossessor .controller .you)]
semantic_template creatureYourOpponentsControl : Predicate :=
  .and [creature, .hasPossessor .controller (.playerGroup .yourOpponents)]
/-- "another creature", anchored on <n>. -/
semantic_template otherCreature (n : NounPhrase) : Predicate := .and [creature, .otherThan n]
/-- "another creature you control", anchored on <n>. -/
semantic_template otherCreatureYouControl (n : NounPhrase) : Predicate :=
  .and [creature, .hasPossessor .controller .you, .otherThan n]
/-- "source": the object dealing the damage under discussion. -/
semantic_template source : Predicate := .isSource
semantic_template emblem : Predicate := .isEmblem
/-- "the chosen player" -/
semantic_template chosenPlayer : Predicate := .chosenPlayer .theChoice
/-- "the last chosen player" -/
semantic_template theLastChosenPlayer : Predicate := .chosenPlayer .theLatestChoice
/-- "of the chosen <quality>" -/
semantic_template ofChosen (sort : QualitySort) : Predicate := .ofChosen .theChoice sort
/-- "of the last chosen <quality>" -/
semantic_template ofTheLastChosen (sort : QualitySort) : Predicate := .ofChosen .theLatestChoice sort
semantic_template copyOfACard : Predicate := .isCopyOfACard
semantic_template cardOnTheStack : Predicate := .and [.isCard, spell]
semantic_template tokenOnTheBattlefield : Predicate := .and [.isToken, permanent]
/-- "that targets only <subject>" [CR#115.9c] -/
semantic_template targetsOnly (subject : NounPhrase) : Predicate := .targets subject .only

/-- The four party roles, in rule order [CR#700.8]. -/
semantic_template partyRoles : List Predicate :=
  [ .hasSubtype (.of .creature "Cleric"), .hasSubtype (.of .creature "Rogue"),
    .hasSubtype (.of .creature "Warrior"), .hasSubtype (.of .creature "Wizard") ]

/-- "any target" [CR#115.4]. -/
semantic_template anyTarget : Predicate :=
  .or [.hasType .creature, .hasType .planeswalker, .hasType .battle, .anyPlayer]

/-- "cast by <player>" -/
semantic_template castBy (player : NounPhrase) : Predicate := .castBy player none
/-- "the Nth spell <player> cast <period>" -/
semantic_template nthCastBy (ordinal : Ordinal) (player : NounPhrase) (period : Period) : Predicate :=
  .castBy player (some (ordinal, period))
/-- "any other target" -/
semantic_template anyOtherTarget : Predicate := .and [anyTarget, .other]

/-- "a color", "a creature type": a quality noun. -/
semantic_template quality (sort : QualitySort) : Predicate := .qualityNoun sort none
/-- A quality noun with its domain: "a color other than blue". -/
semantic_template qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate :=
  .qualityNoun sort (some domain)
/-- "itself": the member a comparison over a domain is measuring (`semantics-v3.md` §1.3 Box 5). -/
semantic_template ownMember : NounPhrase := .pro .bare .one
/-- "a <dom> with <stat> <r> <bound>": the stat of `member`, the member itself unless written. -/
semantic_template comparesOwnStat (stat : Stat) (domain : Predicate) (comparator : Comparator) (bound : Amount)
    (member : NounPhrase := ownMember) : Predicate :=
  .compareOver domain (.statOf (.stat stat) member) comparator bound
/-- "the chosen color" -/
semantic_template thatColor : ColorTerm := .chosen .theChoice

semantic_template creatureType (label : String) : Subtype := .of .creature label
semantic_template artifactType (label : String) : Subtype := .of .artifact label
semantic_template landType (label : String) : Subtype := .of .land label
semantic_template enchantmentType (label : String) : Subtype := .of .enchantment label
semantic_template spellType (label : String) : Subtype := .spell label
semantic_template planeswalkerType (label : String) : Subtype := .of .planeswalker label
/-- An outlaw: an Assassin, Mercenary, Pirate, Rogue, or Warlock [CR#700.12]. -/
semantic_template outlaw : Predicate :=
  .or [ .hasSubtype (creatureType "Assassin"), .hasSubtype (creatureType "Mercenary"),
        .hasSubtype (creatureType "Pirate"), .hasSubtype (creatureType "Rogue"),
        .hasSubtype (creatureType "Warlock") ]
semantic_template outlawYouControl : Predicate := .and [outlaw, .hasPossessor .controller .you]
/-- "an Army you control" -/
semantic_template armyYouControl : Predicate :=
  .and [.hasSubtype (creatureType "Army"), creature, .hasPossessor .controller .you]

/-! ## Nouns -/

semantic_template anOpponent : NounPhrase := a .opponent
semantic_template thisCreature : NounPhrase := .asType .creature .this none
semantic_template thisArtifact : NounPhrase := .asType .artifact .this none
semantic_template thisLand : NounPhrase := .asType .land .this none
semantic_template thisPermanent : NounPhrase := .asMarker .permanent .this
semantic_template thisSpell : NounPhrase := .asMarker .spell .this
semantic_template thisRoom : NounPhrase := .asType .enchantment .this (some (enchantmentType "Room"))
semantic_template thisPlaneswalker : NounPhrase := .asType .planeswalker .this none
semantic_template thisAbility : NounPhrase := .asMarker .ability .this
semantic_template thisEnchantment : NounPhrase := .asType .enchantment .this none
semantic_template thisAura : NounPhrase := .asType .enchantment .this (some (enchantmentType "Aura"))
semantic_template thisEquipment : NounPhrase := .asType .artifact .this (some (artifactType "Equipment"))
semantic_template thisSiege : NounPhrase := .asType .battle .this (some (.of .battle "Siege"))
semantic_template thisVehicle : NounPhrase := .asType .artifact .this (some (artifactType "Vehicle"))
semantic_template thisSpacecraft : NounPhrase := .asType .artifact .this (some (artifactType "Spacecraft"))
semantic_template thisSaga : NounPhrase := .asType .enchantment .this (some (enchantmentType "Saga"))
semantic_template thisFortification : NounPhrase :=
  .asType .artifact .this (some (artifactType "Fortification"))
semantic_template thisCase : NounPhrase := .asType .enchantment .this (some (enchantmentType "Case"))
semantic_template thisClass : NounPhrase := .asType .enchantment .this (some (enchantmentType "Class"))
/-- "your commander" -/
semantic_template yourCommander : NounPhrase := .designated (.named "commander") .you
/-- "a card exiled with this artifact" -/
semantic_template exiledWithThisArtifact : Predicate := .exiledWith thisArtifact
/-- "the rest of them" -/
semantic_template theRest (kind : Kind) : NounPhrase := .theRest kind .many
/-- "the other pile": the one remaining after a choice among two. -/
semantic_template theOther (kind : Kind) : NounPhrase := .theRest kind .one
/-- "N of <group>" -/
semantic_template someOf (quantity : Quantity) (group : NounPhrase) : NounPhrase :=
  .someOf (.counted quantity) none group
/-- "among <group>": the whole of a group, sliced. -/
semantic_template among (group : NounPhrase) : NounPhrase := .someOf .whole none group
/-- "N <p> from among <group>" -/
semantic_template fromAmong (quantity : Quantity) (p : Predicate) (group : NounPhrase) : NounPhrase :=
  .someOf (.counted quantity) (some p) group
/-- "<group> with the same name" -/
semantic_template withTheSameName (group : NounPhrase) : NounPhrase := .agree .cardName .share group
/-- "<group> with different names" -/
semantic_template withDifferentNames (group : NounPhrase) : NounPhrase := .agree .cardName .differ group
/-- "the <p> from among <group>": every member the description picks. -/
semantic_template allFromAmong (p : Predicate) (group : NounPhrase) : NounPhrase := .someOf .whole (some p) group
/-- "the <p> among <group>" -/
semantic_template allAmong (p : Predicate) (group : NounPhrase) : NounPhrase := allFromAmong p group
/-- "each object" -/
semantic_template everyObject : NounPhrase := allOf (.and [])
/-- "one pile": one of the piles just made. -/
semantic_template onePile : NounPhrase := .pileOf (.counted (exactly 1)) none
/-- "the pile of <player>'s choice" -/
semantic_template pileOfChoice (player : NounPhrase) : NounPhrase := .pileOf (.counted (exactly 1)) (some player)
/-- "you and <subject>" -/
semantic_template youAnd (subject : NounPhrase) : NounPhrase := Primitives.NounPhrase.both .you subject
/-- "you or <subject>" -/
semantic_template youOr (subject : NounPhrase) : NounPhrase := Primitives.NounPhrase.eitherOf .you subject
semantic_template controllerOf (subject : NounPhrase) : NounPhrase := .possessorOf .controller subject
semantic_template ownerOf (subject : NounPhrase) : NounPhrase := .possessorOf .owner subject
/-- "the top N cards of your library" -/
semantic_template topSlice (amount : Amount) : NounPhrase := .slice .top amount (library .you)
/-- "the bottom card of your library" -/
semantic_template bottomCard : NounPhrase := .slice .bottom (.lit 1) (library .you)
/-- "[a player]'s party" [CR#700.8]. -/
semantic_template partyOf (player : NounPhrase) : NounPhrase :=
  .oneEachOf partyRoles (allOf (.and [creature, .hasPossessor .controller player]))
/-- "your party" -/
semantic_template party : NounPhrase := partyOf .you
/-- "the number of creatures in <player>'s party" [CR#700.8a]. -/
semantic_template partySizeOf (player : NounPhrase) : Amount := .countOf (partyOf player)
semantic_template partySize : Amount := partySizeOf .you
/-- "<player> has a full party": four creatures in that party [CR#700.8c]. -/
semantic_template fullPartyOf (player : NounPhrase) : Condition := .compareAmt (partySizeOf player) .eq (.lit 4)
semantic_template fullParty : Condition := fullPartyOf .you

/-! ## Mana -/

semantic_template generic (amount : Nat) : ManaSymbol := .simple (.generic amount)
semantic_template pip (color : Color) : ManaSymbol := .simple (.specific (.of color))
/-- "{C}" -/
semantic_template colorlessPip : ManaSymbol := .simple (.specific .colorless)
/-- "{A/B}" -/
semantic_template hybridPip (left right : Color) : ManaSymbol := .hybrid (.specific (.of left)) right
/-- "{G/P}": a Phyrexian mana symbol [CR#107.4f]. -/
semantic_template phyrexianPip (color : Color) : ManaSymbol := .phyrexian color none
/-- "{1} for each …": generic mana cost scaled by an amount. -/
semantic_template scaledGenericMana (amount : Amount) : Cost :=
  .scaled (.mana [generic 1]) amount

/-! ## Durations -/

/-- "until end of turn" -/
semantic_template untilEndOfTurn : Duration := .between none (some (.endOf .turn none .current))
/-- "this turn", as an effect lasts: until the turn ends. -/
semantic_template thisTurn : Duration := .between none (some (.endOf .turn none .current))
/-- "until your next turn" -/
semantic_template untilYourNextTurn : Duration :=
  .between none (some (.beginningOf .turn (some .you) (.next none)))
/-- "until end of combat" -/
semantic_template untilEndOfCombat : Duration := .between none (some (.endOf .combat none .current))
/-- "until your next end step" -/
semantic_template untilYourNextEndStep : Duration :=
  .between none (some (.beginningOf .endStep (some .you) (.next none)))
/-- "until <player>'s next turn" -/
semantic_template untilNextTurnOf (player : NounPhrase) : Duration :=
  .between none (some (.beginningOf .turn (some player) (.next none)))
/-- "until <event>" -/
semantic_template untilEvent (event : GameEvent) : Duration := .between none (some event)
/-- "for the rest of the game" -/
semantic_template restOfGame : Duration := .between none none
/-- "during <player>'s next turn" (`firstMention` / `laterMention` so `player` is not duplicated
[CR#115.1,608.2c]). -/
semantic_template duringNextTurnOf (player : NounPhrase) : Duration :=
  .between
    (some (.beginningOf .turn (some (.firstMention "duringNextTurnOf.player" player)) (.next none)))
    (some (.endOf .turn (some (.laterMention "duringNextTurnOf.player" player)) (.next none)))
/-- "on a later turn": from the end of this turn on [CR#702.143a,702.170d]. -/
semantic_template onALaterTurn : Duration := .between (some (.endOf .turn none .current)) none

/-! ## History windows

A lookback's span ends at `now` ("this turn" so far), except "last turn", which ended. -/

/-- "this turn", as a lookback reads it: from the turn's beginning to now. -/
semantic_template soFarThisTurn : Duration := .between (some (.beginningOf .turn none .current)) (some .now)
/-- "earlier this turn" -/
semantic_template earlierThisTurn : Duration :=
  .between (some (.beginningOf .turn none .current)) (some .now)
/-- "this combat" -/
semantic_template thisCombat : Duration := .between (some (.beginningOf .combat none .current)) (some .now)
/-- "last turn" -/
semantic_template lastTurn : Duration :=
  .between (some (.beginningOf .turn none .previous)) (some (.endOf .turn none .previous))
/-- "this game" -/
semantic_template thisGame : Duration := .between none (some .now)

/-! ## Timings -/

/-- "only as a sorcery": during your main phase, while the stack is empty [CR#307.5]. -/
semantic_template asSorcery : Timing :=
  .and [.inPart .mainPhase (some .you) .current, .onlyIf (.not (exists_ (.inZone stack)))]
/-- "only as an instant": any time you have priority [CR#117.1a]. -/
semantic_template asInstant : Timing := .onlyIf (.hasPriority .you)
/-- "during <whose> <part>" -/
semantic_template duringPart (part : TurnPart) (whose : Option NounPhrase := none) : Timing :=
  .inPart part whose .current
/-- "before <whose> <part>" -/
semantic_template beforePart (part : TurnPart) (whose : Option NounPhrase := none) : Timing :=
  .relativeToEvent .before (.beginningOf part whose .current)
/-- "during each of your turns" -/
semantic_template duringEachOfYourTurns : Timing := .inPart .turn (some .you) (.each none)
/-- "while you're searching your library" -/
semantic_template whileSearchingYourLibrary : Timing :=
  .whileDoing (.does (.actor .you) (.search (.oneZone yourLibrary) anyNumber (.and [])))
/-- "while <event>" -/
semantic_template whileDoing (event : GameEvent) : Timing := .whileDoing event
/-- "while <condition>" -/
semantic_template whileTrue (condition : Condition) : Timing := .onlyIf condition

/-! ## Usage limits -/

/-- "only once each turn" [CR#602.5b] -/
semantic_template oncePerTurn : UsageLimit := ⟨.moreThan (.lit 1), .turn⟩
/-- "only once each game" -/
semantic_template oncePerGame : UsageLimit := ⟨.moreThan (.lit 1), .game⟩
/-- "only once during each of your turns" -/
semantic_template onceEachYourTurn : UsageLimit := ⟨.moreThan (.lit 1), .yourTurn⟩
/-- "Activate only once": no more than one while the object exists [CR#602.5b,702.177a]. -/
semantic_template oncePerObject : UsageLimit := ⟨.moreThan (.lit 1), .thisObject⟩
/-- "<instruction>. Do this only once each turn." [CR#603.2h] -/
semantic_template doOnlyOnceEachTurn (instruction : Action) : Action :=
  .limited instruction oncePerTurn

/-! ## Amounts -/

semantic_template lifeTotalOf (player : NounPhrase) : Amount := .statOf (.playerStat .lifeTotal) player
/-- "<player>'s speed" [CR#702.179a]. -/
semantic_template speedOf (player : NounPhrase) : Amount := .statOf (.playerStat .speed) player
semantic_template powerOf (subject : NounPhrase) : Amount := .statOf (.stat .power) subject
semantic_template toughnessOf (subject : NounPhrase) : Amount := .statOf (.stat .toughness) subject
semantic_template countersOn (kind : CounterKind) (holder : NounPhrase) : Amount := .statOf (.counter kind) holder
/-- "<saga>'s final chapter number": the greatest of its chapter numbers, 0 with none
[CR#714.2d]. -/
semantic_template finalChapterNumber (saga : NounPhrase) : Amount := .aggregate .max .chapter saga
/-- "a chapter ability of <saga>" [CR#714.2,714.2a] -/
semantic_template chapterAbilityOf (saga : NounPhrase) : Predicate :=
  .and [.abilityOf saga, .compare [.chapter] .atLeast (.lit 1)]
semantic_template plus (left right : Amount) : Amount := .arith .plus left right
semantic_template minus (left right : Amount) : Amount := .arith .minus left right
semantic_template times (per amount : Amount) : Amount := .arith .times per amount
/-- "the smaller of <a> and <b>", "<a> or <b>, whichever is smaller" [CR#704.5q]. -/
semantic_template smallerOf (a b : Amount) : Amount := .aggregateOf .min [a, b]
/-- "the greater of <a> and <b>", "<a> or <b>, whichever is greater". -/
semantic_template greaterOf (a b : Amount) : Amount := .aggregateOf .max [a, b]
/-- "that much damage prevented this way" -/
semantic_template preventedThisWay : Amount := .theOutcome .damagePrevented
/-- "N for each <p>" -/
semantic_template forEach (per : Nat) (p : Predicate) : Amount := times (.lit per) (countOf p)

/-! ## Counters -/

/-- The bench's spelling of a counter declaration's name read as its kind, as a bare
`p1p1Counter` reads in `plugins_v2`. Any other kind is written `.named "<declaration name>"`. -/
semantic_template p1p1Counter : CounterKind := .named "p1p1Counter"
semantic_template flyingCounter : CounterKind := .named "flyingCounter"
semantic_template m1m1Counter : CounterKind := .named "m1m1Counter"
/-- What one +X/+Y counter confers [CR#122.1a]: "+X/+Y" to its bearer [CR#613.4c]. A counter
definition's conferral is per counter (`Definition.counter`), so the deltas read no count. -/
semantic_template boost (power toughness : Delta Amount) : Conferral :=
  .property (.ptModification .this power toughness)
/-- What one keyword counter confers [CR#122.1b]: its bearer gains the keyword [CR#613.1f], named
by its declaration's name (`grants (.named "flying")`). -/
semantic_template grants (keyword : KeywordLabel) : Conferral :=
  .property (.abilityGrant .this (.keyword keyword [] []))
/-- What the creature type confers [CR#508.1a]: its object is a Combatant. -/
semantic_template combatant : Conferral := .property (.combatant .this)

/-! ## Actions -/

semantic_template move (subject : NounPhrase) (destination : ZoneExpr) : Action :=
  .move subject .anywhere (some destination) .arrival none []

/-- "destroy <subject>" (`Definition`, [CR#701.8]). -/
semantic_definition destroy (subject : NounPhrase) : Action :=
  .ownScope (some (.action "destroy"))
    (.enact (.action "destroy")
      (.move (.inCaller (.firstMention "destroy.subject" subject)) (.zones [.zone .battlefield .bare]) (some graveyard) .departure none [])
      none none)

/-- "exile <subject>" (`Definition`, [CR#701.13]). -/
semantic_definition exile (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exile.subject" subject)) .anywhere (some exileZone) .arrival none [])
      none none)

/-- A card exiled face down: no characteristics [CR#406.3a]. -/
semantic_template exiledFaceDown : FaceDown := ⟨none⟩
/-- A 2/2 face-down creature with no text, no name, no subtypes and no mana cost [CR#708.2a]. -/
semantic_template faceDownCreature : FaceDown :=
  ⟨some { types := [.creature], power := some (.lit 2), toughness := some (.lit 2) }⟩
/-- A 2/2 face-down creature with ward {2}, no name, no subtypes and no mana cost
[CR#701.58a,702.168a]. -/
semantic_template faceDownWardedCreature : FaceDown :=
  ⟨some { types := [.creature], text := [.keyword (.named "ward") [.cost (.mana [generic 2])] []],
          power := some (.lit 2), toughness := some (.lit 2) }⟩

/-- "exile <subject> face down" (`Definition`, [CR#701.13,406.3]). -/
semantic_definition exileFaceDown (subject : NounPhrase) : Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exileFaceDown.subject" subject)) .anywhere (some exileZone) .arrival none
        [.placement (.faceDown exiledFaceDown)])
      none none)

/-- "exile <subject> with N <kind> counters on it" (`Definition`, [CR#701.13]). -/
semantic_definition exileWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    Action :=
  .ownScope (some (.action "exile"))
    (.enact (.action "exile")
      (.move (.inCaller (.firstMention "exileWithCounters.subject" subject)) .anywhere (some exileZone) .arrival none
        [.placement (.withCounters (.inCaller amount) (.printed kind) .fresh)])
      none none)

/-- "sacrifice <subject>" (`Definition`, [CR#701.21]). -/
semantic_definition sacrifice (subject : NounPhrase) : Action :=
  .ownScope (some (.action "sacrifice"))
    (.enact (.action "sacrifice")
      (.move (.inCaller (.firstMention "sacrifice.subject" subject)) (.zones [.zone .battlefield .bare]) (some graveyard) .departure none [])
      none none)

/-- "put <subject> <destination>" -/
semantic_template put (subject : NounPhrase) (destination : ZoneExpr) : Action :=
  .enact (.core .put) (.move subject .anywhere (some destination) .arrival none []) none none

/-- "put <subject> <destination>" with the move's riders ("second from the top", "on the top or
bottom") -/
semantic_template putWith (subject : NounPhrase) (destination : ZoneExpr) (riders : List MoveRider) :
    Action :=
  .enact (.core .put) (.move subject .anywhere (some destination) .arrival none riders) none none

/-- "return <subject> to <zone>" -/
semantic_template returnTo (subject : NounPhrase) (destination : ZoneExpr) (riders : List MoveRider) :
    Action :=
  .enact (.core .return_) (.move subject .anywhere (some destination) .arrival none riders) none none

/-- "return <subject> to the battlefield" -/
semantic_template returnToBattlefield (subject : NounPhrase) : Action := returnTo subject battlefield []

/-- "return <subject> to the battlefield transformed under <controller>'s control" -/
semantic_template returnToBattlefieldTransformed (subject controller : NounPhrase) : Action :=
  returnTo subject battlefield [.entry .entersTransformed, .entry (.under controller)]

/-- "return <subject> to the battlefield under <who>'s control with N <kind> counters on it" -/
semantic_template returnToBattlefieldWithCounters (subject who : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Action :=
  .move subject .anywhere (some battlefield) .arrival none
    [.entry (.under who), .entry (.withCounters amount (.printed kind) .fresh)]

/-- "transform <subject>" (`Definition`, [CR#701.27]). -/
semantic_definition transform (subject : NounPhrase) : Action :=
  .ownScope (some (.action "transform"))
    (.enact (.action "transform") (.turnOver (.inCaller (.firstMention "transform.subject" subject))) none none)

/-- "meld <subject> into <name>" (`Definition`, [CR#701.42]). -/
semantic_definition meldInto (subject : NounPhrase) (into : String) : Action :=
  .ownScope (some (.action "meld"))
    (.enact (.action "meld")
      (.move (.inCaller (.firstMention "meldInto.subject" subject)) .anywhere (some battlefield) .arrival none [.entry (.entersMelded into)])
      none none)

/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a]. -/
semantic_definition manifestPlacement (subject : NounPhrase) : Action :=
  .ownScope (some (.action "manifest"))
    (.enact (.action "manifest") (.move (.inCaller (.firstMention "manifestPlacement.subject" subject)) .anywhere (some battlefield) .arrival none []) none none)

/-- "tap <subject>" (`Definition`, [CR#701.26a]). -/
semantic_definition tap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "tap"))
    (.enact (.action "tap") (.setStatus .tapped (.inCaller (.firstMention "tap.subject" subject))) none none)

/-- "discard <subject>" (`Definition`, [CR#701.9a]). -/
semantic_definition discard (subject : NounPhrase) : Action :=
  .ownScope (some (.action "discard"))
    (.enact (.action "discard")
      (.move (.inCaller (.firstMention "discard.subject" subject)) (.zones [.zone .hand .bare]) (some graveyard) .arrival none [])
      none none)

/-- A bare "shuffle": the performer's own library (`Definition`, [CR#701.24a]). -/
semantic_definition shuffle : Action :=
  .ownScope (some (.action "shuffle"))
    (.enact (.action "shuffle") (.shuffle (library .actor)) none none)

/-- "untap <subject>" (`Definition`, [CR#701.26b]). -/
semantic_definition untap (subject : NounPhrase) : Action :=
  .ownScope (some (.action "untap"))
    (.enact (.action "untap") (.setStatus .untapped (.inCaller (.firstMention "untap.subject" subject))) none none)

/-- "Exile <subject> until <event>." -/
semantic_template exileUntil (subject : NounPhrase) (event : GameEvent) : Action :=
  .holdUntil (exile subject) event

/-- "mill <amount> cards" from <whose> library (`Definition`, [CR#701.17]). -/
semantic_definition mill (amount : Amount) (whose : NounPhrase) : Action :=
  .ownScope (some (.action "mill"))
    (.enact (.action "mill")
      (.move (.slice .top (.inCaller amount) (library (.inCaller (.firstMention "mill.whose" whose))))
        (.zones [.zone .library .bare]) (some graveyard) .arrival none [])
      none none)

semantic_template putOntoBattlefield (subject : NounPhrase) : Action :=
  .move subject .anywhere (some battlefield) .arrival none []
semantic_template putOntoBattlefieldTapped (subject : NounPhrase) : Action :=
  .move subject .anywhere (some battlefield) .arrival none [.entry (.entersAs .tapped)]
/-- "put <subject> onto the battlefield under your control" -/
semantic_template putOntoBattlefieldUnderYourControl (subject : NounPhrase) : Action :=
  .move subject .anywhere (some battlefield) .arrival none [.entry (.under .you)]
/-- "put <subject> onto the battlefield tapped and attacking" -/
semantic_template putOntoBattlefieldTappedAttacking (subject : NounPhrase)
    (from_ : EventSource := .anywhere) : Action :=
  .move subject from_ (some battlefield) .arrival none
    [.entry (.entersAs .tapped), .entry (.entersAttacking none)]
/-- A search of one zone: the scope constructor, which has no generated primitive because
`SearchScope` is not a `semantic_expression` type. -/
semantic_template oneZone (zone : ZoneExpr) : SearchScope := .oneZone zone
/-- "Search your library for <quantity> <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchLibraryFor (quantity : Quantity) (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search") (.search (.oneZone yourLibrary) (.inCaller quantity) (.inCaller p)) none none)
/-- "search <whose>'s graveyard, hand, and library for <q> <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchZonesOf (whose : NounPhrase) (quantity : Quantity) (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search")
      (.search (.someZones (some (.inCaller (.firstMention "searchZonesOf.whose" whose))) [.graveyard, .hand, .library])
        (.inCaller quantity) (.inCaller p))
      none none)
/-- "search your library and/or graveyard for a <p>" (`Definition`, [CR#701.23]). -/
semantic_definition searchLibraryOrGraveyard (p : Predicate) : Action :=
  .ownScope (some (.action "search"))
    (.enact (.action "search")
      (.search (.someZones (some .you) [.library, .graveyard]) (exactly 1) (.inCaller p))
      none none)
/-- "reveal their hand" -/
semantic_definition revealTheirHand : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal") (.expose .reveal (.zone (handOf (.inCaller they)))) none none)

/-- "<creature> fights <creature>" (`Definition`, [CR#701.14a]). -/
semantic_definition fight (other : NounPhrase) : Action :=
  .ownScope (some (.action "fight"))
    (.enact (.action "fight")
      (.doIf (.and [
        .matches (.firstMention "fight.0" (.inCaller other)) creature,
        .matches (.firstMention "fight.source" .actor) creature])
        (.simultaneously [
          .dealDamage (.statOf (.stat .power) (.laterMention "fight.source" .actor))
            (.inCaller (.laterMention "fight.0" other)) .any,
          .instruct (.laterMention "fight.0" (.inCaller other))
            (.dealDamage (.statOf (.stat .power) (.laterMention "fight.0" (.inCaller other)))
              (.laterMention "fight.source" .actor) .any)])
        none)
      none none)

/-- The replacement's application, rather than installation of a new shield
[CR#701.19b,701.19c]. -/
semantic_template regenerationApplication (subject : NounPhrase) : Action :=
  .enact (.action "regenerate")
    (.sequentially [
      .clearDamage (.firstMention "regenerationApplication.subject" subject),
      .instruct (.possessorOf .controller (.laterMention "regenerationApplication.subject" subject))
        (tap (.laterMention "regenerationApplication.subject" subject)),
      .doIf (.or [.matches (.laterMention "regenerationApplication.subject" subject) (.inCombat .attackerOf none),
                  .matches (.laterMention "regenerationApplication.subject" subject) (.inCombat .blockerOf none)])
        (.combat (.laterMention "regenerationApplication.subject" subject) (.participation .outsideCombat)) none])
    none none

/-- A resolving regeneration instruction installs a single-use shield for this turn;
only its eventual application carries the Regenerate label (`Definition`, [CR#701.19a,701.19c]). -/
semantic_definition regenerate (subject : NounPhrase) : Action :=
  .ownScope (some (.action "regenerate"))
    (.establish (.instead
      (.would .any (destroy (.firstMention "regenerate.0" (.inCaller subject))))
      (regenerationApplication (.laterMention "regenerate.0" (.inCaller subject)))
      none .nextTimeOnly none) (some thisTurn))

/-- Losing counters is removal from the named player. -/
semantic_template loseCounters (kind : Option CounterKindSource) (amount : Option Amount)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Action :=
  .removeCounters (amount.map Quantity.exactlyOf) kind agent .one true
semantic_template loseLife (amount : Amount) : Action :=
  .changeLife (.down amount)
semantic_template gainLife (amount : Amount) : Action :=
  .changeLife (.up amount)
/-- "<player>'s life total becomes <amount>" -/
semantic_template setLife (amount : Amount) : Action :=
  .changeLife (.set amount)
semantic_template draw (amount : Amount) : Action := .draw amount

semantic_template lookAt (cards : NounPhrase) : Action := .expose .lookAt (.cards cards)
/-- "look at <player>'s hand" -/
semantic_template lookAtHandOf (player : NounPhrase) : Action :=
  .expose .lookAt (.zone (handOf player))
/-- "reveal <cards>" (`Definition`, [CR#701.20]). -/
semantic_definition revealCards (cards : NounPhrase) : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal") (.expose .reveal (.cards (.inCaller (.firstMention "revealCards.cards" cards)))) none none)
/-- "shuffle <subject> into its owner's library": the move, then the shuffle of that library,
which happens even if nothing moved (`Definition`, [CR#701.24c,701.24d]). -/
semantic_definition shuffleInto (subject : NounPhrase) : Action :=
  .ownScope (some (.action "shuffle"))
    (.sequentially
      [ .enact (.action "shuffle")
          (.move (.inCaller (.firstMention "shuffleInto.subject" subject)) .anywhere (some (.zone .library .bare)) .arrival none []) none none,
        .shuffle (.library .bare) ])
semantic_template doIf (condition : Condition) (instruction : Action)
    (otherwise : Option Action := none) : Action :=
  .doIf condition instruction otherwise
/-- "choose <subject>" -/
semantic_template choose (subject : NounPhrase) (disclosure : Disclosure := .openly) : Action :=
  .choose none subject disclosure none
/-- "choose <subject> as you <event>" -/
semantic_template chooseWhile (subject : NounPhrase) (while_ : Timing) : Action :=
  .choose none subject .openly (some while_)
semantic_template rollDice (count : Nat) (sides : Nat) : Action :=
  .rollDice (.lit count) (.sides sides) .one .anyResult
/-- One row of a results table: "<results> — <instruction>". -/
semantic_template rollRow (results : Quantity) (instruction : Action) : RollRow := ⟨results, instruction⟩
semantic_template flipCoins (count : Nat) : Action :=
  .flipCoins (.count (.lit count)) none none
/-- "<player> flips a coin" as an event -/
semantic_template flipsCoin (player : NounPhrase) : GameEvent :=
  .does (.actor player) (.flipCoins (.count (.lit 1)) none none)
/-- "may <body>" -/
semantic_template offer (body : Action) : Action := Primitives.Action.offer body none none
/-- "may <body>. When they do, <trigger>": a reflexive trigger on the choice. -/
semantic_template offerWhen (body : Action) (triggerBody : Action) : Action :=
  .triggerReflexively (Primitives.Action.offer body none none) triggerBody
/-- "the chosen number" -/
semantic_template chosenNumber : Amount := .chosenNumber .theChoice
/-- "Choose one or more — [cost] — <mode>; …" [CR#702.172a] -/
semantic_template chooseSpree (modes : List (Option Cost × Action)) : Action :=
  .chooseModes (atLeast 1) (modes.map fun (c, e) => (c, none, e))
/-- "vote for <ballot>" (`Definition`, [CR#701.38]). -/
semantic_definition vote (disclosure : Disclosure) (ballot : Ballot) : Action :=
  .ownScope (some (.action "vote"))
    (.enact (.action "vote") (.vote none disclosure ballot) none none)
/-- "Starting with <first>, vote for <ballot>" (`Definition`, [CR#701.38]). -/
semantic_definition voteStartingWith (first : NounPhrase) (disclosure : Disclosure) (ballot : Ballot) :
    Action :=
  .ownScope (some (.action "vote"))
    (.enact (.action "vote") (.vote (some (.inCaller (.firstMention "voteStartingWith.first" first))) disclosure ballot) none none)
/-- "Choose N — <modes>", no mode costing anything. -/
semantic_template chooseModes (quantity : Quantity) (modes : List Action) : Action :=
  .chooseModes quantity (modes.map (none, none, ·))
/-- "<source> deals damage equal to its power to <recipient>" (`Template`, `firstMention` /
`laterMention` without `Window.top`). -/
semantic_template dealDamageOwnPower (source : NounPhrase) (recipient : NounPhrase) :
    Action :=
  .instruct (.firstMention "dealDamageOwnPower.source" source)
    (.dealDamage (.statOf (.stat .power) (.laterMention "dealDamageOwnPower.source" source))
      recipient .any)

/-- A token's characteristics from the parts a creature token names. -/
semantic_template creatureTokenOf (power toughness : Amount) (colors : List Color) (subtypes : List Subtype) :
    CharacteristicBundle :=
  { characteristics :=
      { colors, types := [.creature], subtypes, power := some power,
        toughness := some toughness } }
semantic_template creatureToken (power toughness : Nat) (colors : List Color) (subtypes : List Subtype) :
    CharacteristicBundle :=
  creatureTokenOf (.lit power) (.lit toughness) colors subtypes
/-- "create N <token>" (`Definition`, [CR#701.7]). -/
semantic_definition create (count : Amount) (token : CharacteristicBundle) : Action :=
  Primitives.Action.create count (.written token) []
/-- "create N <token> tapped and attacking" (`Definition`, [CR#701.7]). -/
semantic_definition createTappedAttacking (count : Amount) (token : CharacteristicBundle) : Action :=
  Primitives.Action.create count (.written token) [.entersAs .tapped, .entersAttacking none]

/-- "for each color of mana spent to cast <n>" -/
semantic_template colorsSpentToCast (spell : NounPhrase) : Amount := .paid .colorsSpent spell
/-- "if colored mana was spent to cast <spell>" -/
semantic_template coloredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .atLeast (.lit 1)
/-- "if no colored mana was spent to cast <spell>" -/
semantic_template noColoredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .eq (.lit 0)
/-- "the amount of mana spent to cast <spell>" -/
semantic_template manaValueSpentToCast (spell : NounPhrase) : Amount := .paid .manaValueSpent spell
/-- "if no mana was spent to cast <spell>" -/
semantic_template noManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (manaValueSpentToCast spell) .eq (.lit 0)

/-- "between N and M" -/
semantic_template fromTo (low high : Nat) : Quantity := .range (some low) (some high)
/-- The participant described by the nearest enclosing lookback (`NounPhrase.which`). -/
semantic_template relative (_kind : Kind) : NounPhrase := .which
/-- The number of occurrences of a historical action clause. -/
semantic_template eventCount (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount :=
  .tally ⟨subj, action, .duration within timing⟩ .events
/-- A historical action clause. -/
semantic_template happened (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Condition :=
  .did ⟨subj, action, .duration within timing⟩
/-- The action the enclosing instruction caused: "if you searched your library this way". -/
semantic_template happenedThisWay (action : Action) (subj : Subject) : Condition :=
  .did ⟨subj, action, .thisWay⟩
/-- A historical action clause whose `.which` stands for the entity being described. -/
semantic_template happenedTo (subj : Subject) (action : Action) (within : Duration)
    (timing : Option Timing := none) : Predicate :=
  .did ⟨subj, action, .duration within timing⟩
/-- The accumulated magnitude of historical action events. -/
semantic_template eventSum (action : Action) (subj : Subject) (within : Duration)
    (timing : Option Timing := none) : Amount :=
  .tally ⟨subj, action, .duration within timing⟩ .quantity
/-- "if there is no <designation>" -/
semantic_template thereIsNo (designation : DesignationLabel) : Condition := .noHolder designation
/-- "<subject>'s <keyword> cost was paid", read back. -/
semantic_template paidCostRead (which : LinkSource) (window : Option Duration) (subject : NounPhrase) :
    Amount :=
  .paid (.readback which window) subject
/-- "the number of times <subject>'s <keyword> cost was paid" -/
semantic_template timesPaid (which : LinkSource) (subject : NounPhrase) : Amount :=
  .paid (.timesPaid which) subject
/-- "the player chosen as <subject>'s <label> cost was paid", read back by the linked ability
[CR#607.2d,702.174a] -/
semantic_template chosenAsPaid (label : KeywordLabel) (subject : NounPhrase) : NounPhrase :=
  .paid (.chosenPlayer (.byKeyword label)) subject
/-- "if <subject>'s <cost> was paid" -/
semantic_template costWasPaid (which : LinkSource) (window : Option Duration) (subject : NounPhrase) :
    Condition :=
  .compareAmt (paidCostRead which window subject) .atLeast (.lit 1)
/-- "the sacrificed creature", "the exiled card": the one object `deed` acted on as the Link
Source `which` of `subject` was paid or resolved [CR#607.2a]. -/
semantic_template linkedObject (which : LinkSource) (deed : Deed) (subject : NounPhrase) : NounPhrase :=
  .paid (.objects which deed .one) subject
/-- "the exiled cards", "creatures it devoured": the objects `deed` acted on as the Link Source
`which` of `subject` was paid or resolved [CR#607.2a,702.82b]. -/
semantic_template linkedObjects (which : LinkSource) (deed : Deed) (subject : NounPhrase) :
    NounPhrase :=
  .paid (.objects which deed .many) subject
/-- "if <subject>'s <which> was taken": the choice the Link Source offered was taken
[CR#702.104b]. -/
semantic_template choiceWasTaken (which : LinkSource) (subject : NounPhrase) : Condition :=
  .compareAmt (.paid (.taken which) subject) .atLeast (.lit 1)
/-- "<object>'s mana cost" as the mana a cost shift changes by [CR#702.48a]. -/
semantic_template manaCostOf (object : NounPhrase) : ManaCostTerm := .manaCostOf object
/-- "<subject>'s total cost is reduced by <cost>", symbol by symbol [CR#702.48c,118.7]. -/
semantic_template reducedBySymbols (subject : NounPhrase) (cost : ManaCostTerm) : StaticSpec :=
  .costShift subject (.bySymbols cost false false)
/-- "the last chosen color" -/
semantic_template theLastChosenColor : ColorTerm := .chosen .theLatestChoice
/-- "the last chosen number" -/
semantic_template theLastChosenNumber : Amount := .chosenNumber .theLatestChoice
/-- "the amount by which the ceiling was not reached" -/
semantic_template shortOfCeiling : Amount := .theOutcome .ceilingShortfall
/-- "the number of counters removed this way" -/
semantic_template removedThisWay : Amount := .theOutcome .countersRemoved
/-- "increase or decrease the result by N" -/
semantic_template shiftResult (amount : Amount) : Action := .shiftResult none amount

/-- Map a legacy `Deed` + patient to its `v3` `Action` for deontic helper templates. -/
def deonticActionFor (deed : Deed) (patient : NounPhrase) : Action :=
  match deed with
  | .core .attack => .attack (match patient with | .any => none | p => some p)
  | .core .block => .block patient
  | .core .target => .target patient
  | .core .copy => .copy .fromStack patient .anyAmount []
  | .core .trigger => .trigger
  | .core .fullyUnlock => .fullyUnlock patient
  | .core .unlock => .unlock (.doorOf none patient)
  | .ofAbility (.named "crew") => .crew patient
  | .ofAbility (.named "saddle") => .saddle patient
  | .ofAbility (.named "phasing") => .setStatus .phasedIn patient
  | .core .draw => .draw .anyAmount
  | .core .gainLife => .changeLife (.up .anyAmount)
  | .core .loseGame => .conclude .loseGame
  | .core .winGame => .conclude .winGame
  | .core .put => .putCounters .anyAmount .anyKind patient .one false
  | .core .return_ => .move patient .anywhere (some (.zone .battlefield .bare)) .arrival none []
  | .core .gainControl => .changeControl patient
  | .core .tapForMana => .tapForMana patient none
  | .action "tap" => .setStatus .tapped patient
  | .action "untap" => .setStatus .untapped patient
  | .action "destroy" =>
    .ownScope (some (.action "destroy"))
      (.enact (.action "destroy")
        (.move (.inCaller patient) (.zones [.zone .battlefield .bare]) (some (.zone .graveyard .bare)) .departure none [])
        none none)
  | .action "sacrifice" =>
    .ownScope (some (.action "sacrifice"))
      (.enact (.action "sacrifice")
        (.move (.inCaller patient) (.zones [.zone .battlefield .bare]) (some (.zone .graveyard .bare)) .departure none [])
        none none)
  | .action "regenerate" =>
    .ownScope (some (.action "regenerate"))
      (.enact (.action "regenerate") (.clearDamage (.inCaller patient)) none none)
  | .action "counter" =>
    .ownScope (some (.action "counter"))
      (.enact (.action "counter")
        (.move (.inCaller patient) (.zones [.zone .stack .bare]) (some (.zone .graveyard .bare)) .arrival none [])
        none none)
  | .action "cast" => .procedure .cast patient .itsOwnCost none none none
  | .action "play" => .procedure .play patient .itsOwnCost none none none
  | .action "activate" => .procedure .activate patient .itsOwnCost none none none
  | .action "attach" => .attachment .attached .this (some patient)
  | .action "search" => .search (.oneZone .anyZone) (.range none none) (.and [])
  | .action "transform" => .turnOver patient
  | .action name =>
    .ownScope (some (.action name)) (.enact (.action name) (.clearDamage patient) none none)
  | _ => .setStatus .tapped patient

/-- "<player> may play N additional lands" -/
semantic_template mayPlayAdditionalLands (player : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .may (.actor player) (.procedure .play (allOf land) .itsOwnCost none none none)
    { bound := some (.additional quantity), rider := .noRider }
/-- "<player> may <deed> <what> [as though …] [rider]" -/
semantic_template mayPlayDeed (deed : Deed) (player what : NounPhrase) (asThough : Option AsThough)
    (rider : DeonticRider) : StaticSpec :=
  .may (.actor player) (deonticActionFor deed what) { asThough := asThough, rider := rider }
/-- "You may play this card from <zone> <timing>": the land type's play permission [CR#305.1]. -/
semantic_template playableFrom (zone : ZoneExpr) (timing : Timing) : Conferral :=
  .property (mayPlayDeed (.action "play") .you .this none
    (.play (some zone) none (some timing) false .itsOwnCost))
/-- "You may cast this card from <zone> <timing>": the instant type's cast permission
[CR#304.1]. -/
semantic_template castableFrom (zone : ZoneExpr) (timing : Timing) : Conferral :=
  .property (mayPlayDeed (.action "cast") .you .this none
    (.play (some zone) none (some timing) false .itsOwnCost))
/-- "When <event>, <instruction>" as a delayed trigger. -/
semantic_template delay (event : GameEvent) (instruction : Action)
    (alternatives : List GameEvent := []) : Action :=
  .delay event alternatives none instruction
/-- "When <event> <duration>, <instruction>": a delayed trigger with a window. -/
semantic_template delayWithin (event : GameEvent) (duration : Duration) (instruction : Action) :
    Action :=
  .delay event [] (some duration) instruction
/-- "<subject> phases out until <event>" -/
semantic_template phaseOutUntil (subject : NounPhrase) (event : GameEvent) : Action :=
  .holdUntil (.setStatus .phasedOut subject) event
/-- "attach <what> to it": ordinary pronoun `it` resolved by `D′` (no `Window` override). -/
semantic_template attachToIt (what : NounPhrase) : Action :=
  Primitives.Action.attachTo what it
/-- "take <count> extra turns after this one": the performer takes them [CR#500.7]. -/
semantic_template extraTurn (count : Amount) : Action := .addTurnPart .turn none count none
/-- "there is an additional <part> [after <anchor>] [, followed by an additional <next>]": phases
added to the current turn [CR#500.8]. -/
semantic_template additionalPhase (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
    (followedBy : Option TurnPart := none) : Action :=
  .addTurnPart part anchor count followedBy
/-- "there is an additional <part> [after <anchor>]": steps added to the current turn's phase
[CR#500.9]. -/
semantic_template additionalStep (part : TurnPart) (anchor : Option TurnPart) (count : Amount) :
    Action :=
  .addTurnPart part anchor count none
/-- "<subject> can't attack [this turn]" -/
semantic_template forbidAttack (subject : NounPhrase) (duration : Option Duration) : Action :=
  .establish (.cant (.actor subject) (.attack none) { rider := .noRider }) duration
/-- "<subject> can't block [this turn]" -/
semantic_template forbidBlock (subject : NounPhrase) (duration : Option Duration) : Action :=
  .establish (.cant (.actor subject) (.block .any) { rider := .noRider }) duration
/-- "<subject> can't be blocked [this turn]" -/
semantic_template forbidBeingBlocked (subject : NounPhrase) (duration : Option Duration) : Action :=
  .establish (.cant .any (.block subject) { rider := .noRider }) duration
/-- "<subject> blocks it this turn if able": ordinary pronoun `it` resolved by `D′` (no `Window`
override). -/
semantic_template requireBlockIt (subject : NounPhrase) (duration : Option Duration) : Action :=
  .establish (.must (.actor subject) (.block it) { rider := .noRider }) duration

/-- "<source> deals N damage divided as you choose among <among>" -/
semantic_template dealDivided (source : NounPhrase) (amount : Amount) (among : NounPhrase) : Action :=
  .distribute (.damage source) amount among
/-- "distribute N <kind> counters among <among>" -/
semantic_template distributeCounters (amount : Amount) (kind : CounterKind) (among : NounPhrase) : Action :=
  .distribute (.counters kind) amount among
/-- "remove <q> <kind> counters from <from>" -/
semantic_template removeCounters (quantity : Quantity) (kind : Option CounterKindSource) (from_ : NounPhrase) :
    Action :=
  .removeCounters (some quantity) kind from_ .one true
/-- "<who> loses all [<kind>] counters" -/
semantic_template loseAllCounters (kind : Option CounterKindSource) (agent : NounPhrase := Primitives.NounPhrase.you) : Action :=
  loseCounters kind none (agent := agent)
/-- "remove all [<kind>] counters from <from>" -/
semantic_template removeAllCounters (kind : Option CounterKindSource) (from_ : NounPhrase) : Action :=
  .removeCounters none kind from_ .one true

/-- "may pay <cost>. If they don't, <instruction>." -/
semantic_template doUnless (instruction : Action) (cost : Cost) : Action :=
  Primitives.Action.offer (.pay cost .once .paid none none) none (some instruction)

/-- "<subject> gets +P/+T [until …]": one change to both stats, the subject written once. -/
semantic_template getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec :=
  .ptModification subject power toughness
/-- "<subject> has base power and toughness P/T" -/
semantic_template getsBase (subject : NounPhrase) (power toughness : Amount) : StaticSpec :=
  getsPt subject (.set power) (.set toughness)

/-- "<subject>'s controller sacrifices it" (`Template` using `firstMention` / `laterMention`
without `Window` plumbing, [CR#701.21a]). -/
semantic_template controllerSacrifices (subject : NounPhrase) : Action :=
  .instruct (.possessorOf .controller (.firstMention "controllerSacrifices.subject" subject))
    (sacrifice (.laterMention "controllerSacrifices.subject" subject))

/-- "look at <slice>, put any number of them into <spill> [riders] and the rest on top of its
owner's library in any order" (`Template` using `firstMention` / `laterMention` without `Window`
plumbing). -/
semantic_template lookAndSortSlice (slice : NounPhrase) (spill : ZoneExpr)
    (spillRiders : List MoveRider := []) : Action :=
  .sequentially
    [ .expose .lookAt (.cards (.firstMention "lookAndSortSlice.slice" slice)),
      .move (someOf anyNumber (.laterMention "lookAndSortSlice.slice" slice)) .anywhere
        (some spill) .arrival none spillRiders,
      .move (theRest .object) .anywhere (some onTop) .arrival none
        [.placement (.inOrder .anyOrder)] ]

namespace Actor

/-- "<performer> discards <subject>": a move from the performer's hand to the graveyard
(`Definition`, [CR#701.9a]). -/
semantic_definition discard (subject : NounPhrase) : Action :=
  .ownScope (some (.action "discard"))
    (.enact (.action "discard")
      (.move (.inCaller (.firstMention "discard.subject" subject)) (.zones [.zone .hand (.possessedBy .actor)]) (some graveyard) .arrival none [])
      none none)

end Actor

/-- "discard <discarded>, then <step> for each nonland card discarded this way" (`Template` using
moved-parameter readback, [CR#701.50a,400.7,400.7j]). -/
semantic_template discardThenForEachNonland (discarded : NounPhrase) (step : Action) :
    Action :=
  .sequentially
    [ Actor.discard discarded,
      .doForEach
        (allOf (.and [.not land, .isCard, .did ⟨.any, Actor.discard .which, .thisWay⟩]))
        step ]

/-- "look at the top N cards of <whose> library, put any number of them on the bottom in any order
and the rest on top in any order" (`Template` using `firstMention` / `laterMention` without
`Window` plumbing). -/
semantic_template lookAndSort (whose : NounPhrase) (amount : Amount) : Action :=
  let slice : NounPhrase := .slice .top amount (library whose)
  .sequentially
    [ .expose .lookAt (.cards (.firstMention "lookAndSortSlice.slice" slice)),
      .move (someOf anyNumber (.laterMention "lookAndSortSlice.slice" slice)) .anywhere
        (some onBottom) .arrival none [.placement (.inOrder .anyOrder)],
      .move (theRest .object) .anywhere (some onTop) .arrival none
        [.placement (.inOrder .anyOrder)] ]
/-- The same look, spilling the cards put aside into <spill> instead of the bottom. -/
semantic_template lookAndSortInto (whose : NounPhrase) (amount : Amount) (spill : ZoneExpr) :
    Action :=
  let slice : NounPhrase := .slice .top amount (library whose)
  .sequentially
    [ .expose .lookAt (.cards (.firstMention "lookAndSortSlice.slice" slice)),
      move (someOf anyNumber (.laterMention "lookAndSortSlice.slice" slice)) spill,
      .move (theRest .object) .anywhere (some onTop) .arrival none
        [.placement (.inOrder .anyOrder)] ]
/-- "scry N" (`Definition`, [CR#701.22a]): the performer's own library. -/
semantic_definition scry (amount : Amount) : Action :=
  .ownScope (some (.action "scry"))
    (.enact (.action "scry") (lookAndSort .actor (.inCaller amount)) none none)
/-- "fateseal N" (`Definition`, [CR#701.29a]). -/
semantic_definition fateseal (whose : NounPhrase) (amount : Amount) : Action :=
  .ownScope (some (.action "fateseal"))
    (.enact (.action "fateseal") (lookAndSort (.inCaller (.firstMention "fateseal.whose" whose)) (.inCaller amount)) none none)
/-- "surveil N" (`Definition`, [CR#701.25a]): the performer's own library. -/
semantic_definition surveil (amount : Amount) : Action :=
  .ownScope (some (.action "surveil"))
    (.enact (.action "surveil") (lookAndSortInto .actor (.inCaller amount) graveyard) none none)
/-- "Proliferate" with its reminder text (`Definition`, [CR#701.34a]). -/
semantic_definition proliferate : Action :=
  .ownScope (some (.action "proliferate"))
    (.enact (.action "proliferate") (.sequentially
      [ .choose none (counted anyNumber
            (.or [ .and [permanent, .hasCounters none],
                   .compare [.anyCounter .player] .atLeast (.lit 1) ])) .openly none,
        .putCounters (.lit 1) .own (.eachOf (those .join)) .one true ])
      none none)
/-- "amass <subtype> N" with its reminder text (`Definition`, [CR#701.47a]). -/
semantic_definition amass (subtype : String) (count : Nat) : Action :=
  .ownScope (some (.action "amass"))
    (.enact (.action "amass") (.sequentially
      [ .doIf (.not (exists_ armyYouControl))
          (create (.lit 1)
            (creatureToken 0 0 [.black] [creatureType subtype, creatureType "Army"]))
          none,
        choose (a armyYouControl),
        .putCounters (.lit count) (.printed p1p1Counter) (that (.type .creature)) .one true,
        .doIf (.not (.matches it (.hasSubtype (creatureType subtype))))
          (.establish
            (Primitives.StaticSpec.qualityChange it .adds
              (.bundle { characteristics := { subtypes := [creatureType subtype] } } none))
            none)
          none ])
      none none)
/-- "monstrosity N" with its reminder text (`Definition`, [CR#701.37a]). -/
semantic_definition makeMonstrous (amount : Amount) : Action :=
  .ownScope (some (.action "monstrosity"))
    (.enact (.action "monstrosity")
      (.doIf (.not (.matches thisPermanent (.hasDesignation (.named "monstrous") none)))
        (.sequentially
          [ .putCounters (.inCaller amount) (.printed p1p1Counter) thisPermanent .one true,
            .gainDesignation thisPermanent (.named "monstrous") none ])
        none)
      none none)
semantic_template get (subject : NounPhrase) (power toughness : Delta Amount) (duration : Option Duration) :
    Action :=
  .establish (getsPt subject power toughness) duration

semantic_template gain (subject : NounPhrase) (ability : Ability) (duration : Option Duration) : Action :=
  .establish (.abilityGrant subject ability) duration
/-- "<subject> gains haste [until …]" -/
semantic_template gainHaste (subject : NounPhrase) (duration : Option Duration) : Action :=
  gain subject (.keyword (.named "haste") [] []) duration
/-- "<subject> becomes <added> in addition to its other types [until …]" -/
semantic_template become (subject : NounPhrase) (added : CharacteristicBundle) (duration : Option Duration) :
    Action :=
  .establish (Primitives.StaticSpec.qualityChange subject .adds (.bundle added none)) duration
/-- "<subject> becomes <colors> [until …]" -/
semantic_template becomeColor (subject : NounPhrase) (colors : ColorSpec) (duration : Option Duration) :
    Action :=
  .establish (Primitives.StaticSpec.qualityChange subject .sets (.colored colors)) duration
/-- Several static clauses sharing one subject, as one instruction. -/
semantic_template establishFor (subject : NounPhrase) (parts : List StaticSpec) (duration : Option Duration) :
    Action :=
  .establish (.conjunction (some subject) parts) duration
/-- "If <clause> would happen, <replacement> instead [duration]." -/
semantic_template replaceEvent (clause : WouldClause) (replacement : Action) (duration : Option Duration) :
    Action :=
  .establish (.instead clause replacement none .repeatedly none) duration
/-- "Prevent all <kind> damage that would be dealt <scope> [duration]." -/
semantic_template preventAll (kind : DamageKind) (scope : DamageScope) (duration : Option Duration) :
    Action :=
  .establish (.damageRule kind .unattributed scope (.prevent .all none) .repeatedly) duration
/-- "Prevent the next N <kind> damage that would be dealt <scope> [duration]." -/
semantic_template preventNext (kind : DamageKind) (scope : DamageScope) (amount : Amount)
    (duration : Option Duration) : Action :=
  .establish (.damageRule kind .unattributed scope (.prevent (.shield amount) none) .repeatedly)
    duration
/-- "Prevent all <kind> damage that would be dealt by <source> <scope> [duration]." -/
semantic_template preventAllBy (kind : DamageKind) (source : NounPhrase) (scope : DamageScope)
    (duration : Option Duration) : Action :=
  .establish (.damageRule kind (.dealtBy source) scope (.prevent .all none) .repeatedly) duration
/-- "The next time <clause> would happen, <replacement> instead [duration]." -/
semantic_template replaceNextEvent (clause : WouldClause) (replacement : Action)
    (duration : Option Duration) : Action :=
  .establish (.instead clause replacement none .nextTimeOnly none) duration
/-- "<player> gains control of <subject> [duration]" -/
semantic_template gainControl (subject : NounPhrase) (duration : Option Duration) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Action :=
  .establish (.controlGrant agent subject) duration
/-- "<subject> can't be <deed>ed" -/
semantic_template objectCant (deed : Deed) (subject : NounPhrase) : StaticSpec :=
  .cant .any (deonticActionFor deed subject) { rider := .noRider }
/-- "<player> can't <deed>" -/
semantic_template playerCant (deed : Deed) (player : NounPhrase) : StaticSpec :=
  .cant (.actor player) (deonticActionFor deed .any) { rider := .noRider }
/-- "<spec> as long as <condition>" -/
semantic_template onlyWhile (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .asLongAs
/-- "<spec> unless <condition>" -/
semantic_template onlyUnless (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec (.not condition) .unless_
/-- "<spec> if <condition>" -/
semantic_template onlyIfSo (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .ifSo
/-- "<who> can't <deed> <what>" -/
semantic_template cantDoTo (deed : Deed) (who what : NounPhrase) : StaticSpec :=
  .cant (.actor who) (deonticActionFor deed what) { rider := .noRider }
/-- "<n> can <deed> as though it didn't have <p>" -/
semantic_template canDoAsThough (n : NounPhrase) (deed : Deed) (p : Predicate) : StaticSpec :=
  .may (.actor n) (deonticActionFor deed .any) { asThough := some (.of p), rider := .noRider }
/-- "<what> can't be the target of <by>" -/
semantic_template cantBeTargetedBy (what by_ : NounPhrase) : StaticSpec :=
  .cant (.by by_) (.target what) { rider := .noRider }
/-- "<what> can be the target of <by> as though it didn't have <p>" -/
semantic_template canBeTargetedAsThough (what by_ : NounPhrase) (p : Predicate) : StaticSpec :=
  .may (.by by_) (.target what) { asThough := some (.of p), rider := .noRider }
/-- "<spec> <duration>": a clause holding for a stated duration. -/
semantic_template establishThroughout (spec : StaticSpec) (duration : Duration) : Action :=
  .establish spec (some duration)
/-- "<n> doesn't untap during [<whose>] untap step" -/
semantic_template doesntUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose (objectCant (.action "untap") subject)
/-- "you may choose not to untap <subject> during [<whose>] untap step" -/
semantic_template mayDeclineUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.may .any (untap subject) { rider := .noRider })
/-- "untap <subject> during [<whose>] untap step" -/
semantic_template untapsDuring (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.must .any (untap subject) { rider := .noRider })
/-- "<subject> can block an additional creature each combat" -/
semantic_template mayBlockAdditional (subject : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .may (.actor subject) (.block (allOf creature))
    { bound := some (.additional quantity), rider := .noRider }
/-- "<who> may vote an additional time" -/
semantic_template mayVoteAdditional (who : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .may (.actor who) (vote .openly .anyBallot)
    { bound := some (.additional quantity), rider := .noRider }
/-- "<who> may spend mana as though it were mana of <as> [to <purpose>]" -/
semantic_template maySpendAsThough (who : NounPhrase) (what : Option ColorOrColorless) (as_ : ManaMatch)
    (purpose : Option SpendPurpose) : StaticSpec :=
  .may (.actor who) (.pay .anyCost .once .paid none none)
    { asThough := some (.mana what as_ purpose), rider := .noRider }
/-- "<subject> may be attached to <host>": the legal-host permission over the attach deed's host
role [CR#701.3a,301.5]. -/
semantic_template mayBeAttachedTo (subject host : NounPhrase) : StaticSpec :=
  .may .any (Primitives.Action.attachTo subject host) { rider := .noRider }
/-- "<player> can't <deed> more than N <p>" -/
semantic_template cantMoreThan (player : NounPhrase) (deed : Deed) (bound : Nat) (p : Predicate) :
    StaticSpec :=
  .cant (.actor player) (deonticActionFor deed (allOf p))
    { bound := some (.moreThan (.lit bound)), rider := .noRider }

/-! ## The performer and the handoff -/

/-- "whoever performs this instruction" -/
semantic_template actor : NounPhrase := .actor
/-- "<performer> <body>": the handoff (`Action.instruct`), to a player or a permanent. -/
semantic_template instruct (performer : NounPhrase) (body : Action) : Action :=
  .instruct performer body
/-- Alias for `instruct` for compatibility during porting. -/
semantic_template act (performer : NounPhrase) (body : Action) : Action :=
  .instruct performer body
/-- "… <performer> controls" -/
semantic_template actorControls : Predicate := .hasPossessor .controller .actor

/-- "For each <pip> mana …, you may <action> rather than pay that mana": a Mana Substitute
[CR#702.51a,702.66a,702.126a,701.67a], spelled as the RON helper `substitute`. -/
semantic_template substitute (action : Action) (pays : List PipPaid) : ManaSubstitute :=
  { action, pays }

/-! ## Text Changes -/

/-- A Text Change of `subject`'s rules text, as `change` names it. -/
semantic_template textChange (subject : NounPhrase) (change : TextChange) : StaticSpec :=
  .textChange subject change
/-- "Remove all text found within square brackets" (cleave [CR#702.148a]). -/
semantic_template removeBracketed : TextChange := .removeBracketed
/-- "Replace all instances of the word 'target' with the word 'each'" (overload
[CR#702.96a]). -/
semantic_template targetToEach : TextChange := .targetToEach
/-- "Gains the text of <source>'s rules text" (splice [CR#702.47a,612.10]). -/
semantic_template appendTextOf (source : NounPhrase) : TextChange := .appendTextOf source
/-- "Replace all instances of one <sort> word with another", the kind one of `sorts` [CR#612.2];
an absent word is the performer's choice. -/
semantic_template replaceWord (sorts : List QualitySort) (from_ : Option TextWord := none)
    (to : Option TextWord := none) : TextChange :=
  .replaceWord sorts from_ to
/-- Exchange the subject's rules text with `other`'s [CR#612.5,701.12h]. -/
semantic_template exchangeWith (other : NounPhrase) : TextChange := .exchangeWith other

namespace Actor

/-- "<performer> reveals their hand" (`Definition`, [CR#701.20]). -/
semantic_definition revealHand : Action :=
  .ownScope (some (.action "reveal"))
    (.enact (.action "reveal") (.expose .reveal (.zone (handOf .actor))) none none)
/-- "<performer> may cast <what> from <zone>", paying its own cost. -/
semantic_template mayCastFrom (what : NounPhrase) (zone : ZoneExpr) : StaticSpec :=
  mayPlayDeed (.action "cast") .actor what none (.play (some zone) none none false .itsOwnCost)
/-- An Army the performer controls. -/
semantic_template army : Predicate := .and [.hasSubtype (creatureType "Army"), creature, actorControls]
/-- "<performer> amasses <subtype> N" (`Definition`, [CR#701.47a]): resolved via `that (.type .creature)`
and `it` inside `ownScope` without `itPrior` window overrides. -/
semantic_definition amass (subtype : String) (count : Nat) : Action :=
  .ownScope (some (.action "amass"))
    (.enact (.action "amass") (.sequentially
      [ .doIf (.not (exists_ army))
          (create (.lit 1) (creatureToken 0 0 [.black] [creatureType subtype, creatureType "Army"]))
          none,
        choose (a army),
        .putCounters (.lit count) (.printed p1p1Counter) (that (.type .creature)) .one true,
        .doIf (.not (.matches it (.hasSubtype (creatureType subtype))))
          (.establish
            (Primitives.StaticSpec.qualityChange it .adds
              (.bundle { characteristics := { subtypes := [creatureType subtype] } } none))
            none)
          none ])
      none none)

/-- "any number of <group> with total <axis> <comparator> <bound>" [CR#701.59a]. -/
semantic_template withTotal (axis : ProjAxis) (comparator : Comparator) (bound : Amount)
    (group : NounPhrase) : NounPhrase :=
  .someOf (.totalling axis comparator bound) none group
/-- "<performer> collects evidence N" (`Definition`, [CR#701.59a]). -/
semantic_definition collectEvidence (amount : Amount) : Action :=
  .ownScope (some (.action "collectEvidence"))
    (.enact (.action "collectEvidence")
      (move (withTotal (.stat .manaValue) .atLeast (.inCaller amount)
        (allOf (.and [.isCard, .inZone (graveyardOf .actor)]))) exileZone)
      none none)
/-- "<performer> casts <subject> without paying its mana cost [if the resulting spell is …]"
(`Definition`, [CR#701.5,601.2]). -/
semantic_definition castWithoutPaying (subject : NounPhrase) (resulting : Option Predicate) :
    Action :=
  .ownScope (some (.action "cast"))
    (.enact (.action "cast")
      (.procedure .cast (.inCaller (.firstMention "castWithoutPaying.subject" subject)) .withoutPaying (resulting.map Predicate.inCaller) none none)
      none none)
/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
semantic_definition discover (amount : Amount) : Action :=
  let fits : Predicate := .compare [.stat .manaValue] .atMost (.inCaller amount)
  .ownScope (some (.action "discover"))
    (.enact (.action "discover") (.sequentially
      [ move (.run .top (.and [.not land, fits]) (library .actor)) exileZone,
        .withContinuation .optional (castWithoutPaying (that .card) (some fits)) none
          (some (move (that .card) hand)),
        .move (theRest .object) .anywhere (some onBottom) .arrival none
          [.placement (.inOrder .randomOrder)] ])
      none none)
/-- "<performer> incubates N" (`Definition`, [CR#701.53a,701.53b]). -/
semantic_definition incubate (amount : Amount) : Action :=
  .ownScope (some (.action "incubate"))
    (.enact (.action "incubate") (.createObject (.lit 1) (.token
      (.doubleFaced
        { characteristics :=
            { types := [.artifact], subtypes := [artifactType "Incubator"],
              text := [.activated (.mana [generic 2]) (transform .this) none none none none] } }
        { characteristics :=
            { name := some "Phyrexian Token", types := [.artifact, .creature],
              subtypes := [creatureType "Phyrexian"], power := some (.lit 0),
              toughness := some (.lit 0) } })
      [.withCounters (.inCaller amount) (.printed p1p1Counter) .fresh]) true none)
      none none)
/-- "<performer> waterbends <cost>" (`Definition`, [CR#701.67a,701.67b]). -/
semantic_definition waterbend (cost : Cost) : Action :=
  .ownScope (some (.action "waterbend"))
    (.enact (.action "waterbend")
      (.pay (.withSubstitute (.inCaller cost)
        (substitute (tap (a (.and [untapped, .or [artifact, creature], actorControls]))) [.generic]))
        .once .paid none none)
      none none)
/-- "<performer> manifests <subject>" (`Definition`, [CR#701.40a,708.2,708.2a]). -/
semantic_definition manifest (subject : NounPhrase) : Action :=
  .ownScope (some (.action "manifest"))
    (.enact (.action "manifest") (.sequentially
      [ .turnFaceDown (.inCaller (.firstMention "manifest.subject" subject)) faceDownCreature,
        .move (that .card) .anywhere (some battlefield) .arrival none
          [.entry (.faceDown faceDownCreature)] ])
      none none)
/-- "<performer> cloaks <subject>" (`Definition`, [CR#701.58a]). -/
semantic_definition cloak (subject : NounPhrase) : Action :=
  .ownScope (some (.action "cloak"))
    (.enact (.action "cloak") (.sequentially
      [ .turnFaceDown (.inCaller (.firstMention "cloak.subject" subject)) faceDownWardedCreature,
        .move (that .card) .anywhere (some battlefield) .arrival none
          [.entry (.faceDown faceDownWardedCreature)] ])
      none none)
/-- "<performer> manifests dread" (`Definition`, [CR#701.62a]). -/
semantic_definition manifestDread : Action :=
  .ownScope (some (.action "manifestDread"))
    (.enact (.action "manifestDread") (.sequentially
      [ .expose .lookAt (.cards (.slice .top (.lit 2) (library .actor))),
        manifest (someOf (exactly 1) them),
        move (theRest .object) graveyard ])
      none none)
/-- "support N" (`Definition`, [CR#701.41a]). -/
semantic_definition support (amount : Amount) : Action :=
  .ownScope (some (.action "support"))
    (.enact (.action "support") (.putCounters (.lit 1) (.printed p1p1Counter)
      (.eachOf (.described (.target (.upToOf (.inCaller amount))) (.and [creature, .otherThan .this])))
      .one true)
      none none)

end Actor

/-- "<performer> explores" (`Definition`, [CR#701.44a]). -/
semantic_definition explore : Action :=
  .ownScope (some (.action "explore"))
    (.enact (.action "explore")
      (.instruct (controllerOf (.firstMention "explore.permanent" .actor))
        (.sequentially
          [ revealCards (.slice .top (.lit 1) (library .actor)),
            .doIf (.matches (that .card) land)
              (move (that .card) (.zone .hand (.possessedBy (ownerOf (that .card)))))
              (some (.sequentially
                [ .putCounters (.lit 1) (.printed p1p1Counter)
                    (.laterMention "explore.permanent" .actor) .one true,
                  offer (move (that .card) graveyard) ])) ]))
      none none)

/-- "<performer> connives [N]" (`Definition`, [CR#701.50a,701.50d]): loops over
`"each nonland card discarded this way"` (`allOf(and([not(land), card, did(any, discard(which), thisWay)]))`). -/
semantic_definition connive (amount : Amount := .lit 1) : Action :=
  .ownScope (some (.action "connive"))
    (.enact (.action "connive")
      (.instruct (controllerOf (.firstMention "connive.permanent" .actor))
        (.sequentially
          [ .draw (.inCaller amount),
            Actor.discard (counted (.exactlyOf (.inCaller amount)) .isCard),
            .doForEach (allOf (.and [.not land, .isCard,
                .did ⟨.any, Actor.discard .which, .thisWay⟩]))
              (.putCounters (.lit 1) (.printed p1p1Counter)
                (.laterMention "connive.permanent" .actor) .one true) ]))
      none none)

/-- "Airbend <subject>" (`Definition`, [CR#701.65a]): per-card `doForEach` loop over
`allOf(and([card, did(any, exile(which), thisWay)]))` where `it` resolves by `D′` to the
singular loop referent. -/
semantic_definition airbend (subject : NounPhrase) : Action :=
  .ownScope (some (.action "airbend"))
    (.enact (.action "airbend")
      (.sequentially
        [ exile (.inCaller (.firstMention "airbend.subject" subject)),
          .doForEach (allOf (.and [.isCard, .did ⟨.any, exile .which, .thisWay⟩]))
            (.instruct (ownerOf it)
              (.establish
                (mayPlayDeed (.action "cast") (ownerOf it) it none
                  (.play (some exileZone) none none false (.payingInstead (.mana [generic 2]))))
                (some (.forAsLongAs (.matches it (.inZone exileZone)))))) ])
      none none)

/-- "double <subject>'s power" (`Definition`, [CR#701.10b,701.10c]): it gets +X/+0, X its power. -/
semantic_definition doublePower (subject : NounPhrase) (duration : Option Duration) : Action :=
  .ownScope (some (.action "double"))
    (.establish (.ptModification (.firstMention "doublePower.0" (.inCaller subject))
      (.up (powerOf (.laterMention "doublePower.0" (.inCaller subject)))) (.up (.lit 0)))
      duration)
/-- "triple <subject>'s power and toughness" (`Definition`, [CR#701.11b,701.11c]). -/
semantic_definition triplePowerAndToughness (subject : NounPhrase) (duration : Option Duration) :
    Action :=
  .ownScope (some (.action "triple"))
    (.establish (.ptModification (.firstMention "triplePT.0" (.inCaller subject))
      (.up (times (.lit 2) (powerOf (.laterMention "triplePT.0" (.inCaller subject)))))
      (.up (times (.lit 2) (toughnessOf (.laterMention "triplePT.0" (.inCaller subject))))))
      duration)
/-- "double <performer>'s life total" (`Definition`, [CR#701.10d]), written as that total's set [CR#119.5]. -/
semantic_definition doubleLifeTotal : Action :=
  .ownScope (some (.action "double"))
    (.changeLife (.set (times (.lit 2) (lifeTotalOf .actor))))

/-! ## Events -/

semantic_template leavesBattlefield (subject : NounPhrase) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (.zones [battlefield]))
/-- "<subject> leaves <zone>" -/
semantic_template leavesZone (subject : NounPhrase) (zone : ZoneExpr) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (.zones [zone]))
/-- "at the beginning of <possessor>'s <part>", each time it comes -/
semantic_template beginningOfPossessed (part : TurnPart) (possessor : NounPhrase) : GameEvent :=
  .beginningOf part (some possessor) (.each none)
/-- "at the beginning of your <part>", each time it comes -/
semantic_template beginningOfYour (part : TurnPart) : GameEvent := .beginningOf part (some .you) (.each none)
/-- "at the beginning of each <part>" -/
semantic_template beginningOfEach (part : TurnPart) : GameEvent := .beginningOf part none (.each none)
/-- "at the beginning of the next <part>": a delayed trigger's moment. -/
semantic_template beginningOfNext (part : TurnPart) : GameEvent := .beginningOf part none (.next none)
/-- "at the beginning of that turn's <part>" -/
semantic_template beginningOfThatTurns (part : TurnPart) : GameEvent := .beginningOf part none .that
semantic_template dealsCombatDamage (source : NounPhrase) (patient : NounPhrase) : GameEvent :=
  Primitives.GameEvent.dealsDamage .combatOnly source (some patient)
semantic_template attacks (subject : NounPhrase) : GameEvent := .does (.actor subject) (.attack none)
/-- "<subject> attacks <whom>" -/
semantic_template attacksPlayer (subject whom : NounPhrase) : GameEvent :=
  .does (.actor subject) (.attack (some whom))
/-- "<n> is put into <zone> from <source>" -/
semantic_template putIntoFrom (subject : NounPhrase) (destination : ZoneExpr) (source : EventSource) : GameEvent :=
  Primitives.GameEvent.putInto subject destination (some source)
/-- "N <kind> counters are put on / removed from <subject>" -/
semantic_template counterEvent (move : CounterMove) (kind : CounterKind) (batch : CounterBatch)
    (subject : NounPhrase) : GameEvent :=
  match move with
  | .put => .does .any (.putCounters .anyAmount (.printed kind) subject batch false)
  | .removed => .does .any (.removeCounters none (some (.printed kind)) subject batch false)
/-- "counters are put on / removed from <subject>", the kind unsaid. -/
semantic_template bareCounterEvent (move : CounterMove) (batch : CounterBatch) (subject : NounPhrase) :
    GameEvent :=
  match move with
  | .put => .does .any (.putCounters .anyAmount .anyKind subject batch false)
  | .removed => .does .any (.removeCounters none none subject batch false)
/-- "the last <kind> counter is removed from <subject> by <who>" -/
semantic_template lastCounterRemovedBy (kind : CounterKind) (subject who : NounPhrase) : GameEvent :=
  .does (.by who) (.removeCounters none (some (.printed kind)) subject .emptying false)
/-- "one or more counters are put on <subject> by an effect" -/
semantic_template manyCountersPutByEffect (subject : NounPhrase) : GameEvent :=
  .does .any (.putCounters .anyAmount .anyKind subject .many true)
/-- "<who> puts one or more counters on <subject>" -/
semantic_template manyBareCountersPutBy (who subject : NounPhrase) : GameEvent :=
  .does (.actor who) (.putCounters .anyAmount .anyKind subject .many false)
/-- A token-creation event, retaining its creator, controller, and effect restriction. -/
semantic_template tokenCreationEvent (tokens : NounPhrase) (byEffect : Bool)
    (by_ under : Option NounPhrase) : GameEvent :=
  .does (match by_ with | some creator => .by creator | none => .any)
    (.createObject .anyAmount (.token (.copyOf tokens []) []) byEffect under)
/-- "one or more tokens would be created" -/
semantic_template tokensCreated (tokens : NounPhrase) : GameEvent :=
  tokenCreationEvent tokens false none none
/-- "you roll a die and the result is <q>" -/
semantic_template youRollResultIn (quantity : Quantity) : GameEvent :=
  .does (.actor .you) (.rollDice (.lit 1) (.sides 6) .one (.resultIn quantity))
/-- "you roll a die and the natural result is the highest" -/
semantic_template youRollHighestNatural : GameEvent :=
  .does (.actor .you) (.rollDice (.lit 1) (.sides 6) .one .highestNatural)
/-- "one or more tokens would be created under <under>'s control by an effect" -/
semantic_template tokensCreatedByEffectUnder (tokens under : NounPhrase) : GameEvent :=
  tokenCreationEvent tokens true none (some under)
semantic_template blocks (subject : NounPhrase) (blocked : Option NounPhrase) : GameEvent :=
  .does (.actor subject) (.block (blocked.getD .any))
semantic_template becomesBlocked (subject : NounPhrase) (by_ : Option NounPhrase) : GameEvent :=
  .does (match by_ with | some b => .by b | none => .any) (.block subject)
/-- "the last <kind> counter is removed from <subject>" -/
semantic_template lastCounterRemoved (kind : CounterKind) (subject : NounPhrase) : GameEvent :=
  .does .any (.removeCounters none (some (.printed kind)) subject .emptying false)
/-- "<subject> regenerates": the verb as an event. -/
semantic_template regenerates (subject : NounPhrase) : GameEvent :=
  .does .any (regenerationApplication subject)

private theorem regenerationEventUsesTheApplication :
    regenerates thisCreature = .does .any (regenerationApplication thisCreature) := rfl

private theorem regenerationEventDescribesTheApplicationRatherThanTheShield :
    let event := regenerates thisCreature
    let applied := (stepGameEvent 64 (DRS.ofList []) event).1
    let shield := (stepAction 64 .postEvent .obj (DRS.ofList []) (regenerate thisCreature)).1
    GameEvent.check (DRS.ofList []) event = [] ∧
      countOnes .object applied = 1 ∧
      (applied.findById? 0).map (fun ref => (ref.kind, ref.zone)) =
        some (.object, some .battlefield) ∧
      countOnes .object shield = 0 ∧
      Action.check (DRS.ofList [])
        (.triggerThisWay (regenerate thisCreature) event
          (.putCounters (.lit 1) (.printed (.named "m1m1Counter")) it .many true)) = [] := by decide

/-- Copy exceptions retain their own edit block; added abilities remain copy-specific. -/
semantic_template copyCharacteristics (c : Characteristics) (typesAdded : Bool) : List CopyExcept :=
  let typeOp := if typesAdded then QualityOp.adds else .sets
  let types := if c.supertypes.isEmpty && c.types.isEmpty && c.subtypes.isEmpty then []
    else [.typeLine typeOp c.typeChanges]
  let values := ({ c with text := [] } : Characteristics).valueEdits .sets
  let edits := types ++ values
  (if edits.isEmpty then [] else [.edits edits]) ++ c.text.map CopyExcept.ability

/-! ## Abilities -/

semantic_template keyword (label : KeywordLabel) : Ability := .keyword label [] []
/-- An ability word in italics before an ability: "Will of the council — …" [CR#207.2c]. -/
semantic_template abilityWord (word : AbilityWordLabel) (ability : Ability) : Ability :=
  .italicHead (.abilityWord word) ability
/-- A flavor word in italics before an ability [CR#207.2d]. -/
semantic_template flavorWord (word : FlavorWordLabel) (ability : Ability) : Ability :=
  .italicHead (.flavorWord word) ability
/-- "Companion — <condition>" -/
semantic_definition companion (condition : DeckCondition) : Ability :=
  .keyword (.named "companion") [.deckCondition condition] []
/-- "Pay N life" as a cost. -/
semantic_template payLife (amount : Nat) : Cost :=
  .perform (.changeLife (.down (.lit amount)))
/-- "<keyword> <cost>" -/
semantic_template keywordCosting (label : KeywordLabel) (cost : Cost) : Ability :=
  .keyword label [.cost cost] []
/-- "<keyword> <quality>", e.g. "protection from red" -/
semantic_template keywordQuality (label : KeywordLabel) (quality : Predicate) : Ability :=
  .keyword label [.quality quality] []
/-- "<keyword> <subject>", e.g. "enchant creature" -/
semantic_template keywordSubject (label : KeywordLabel) (subject : Predicate) : Ability :=
  .keyword label [.subject subject] []
/-- "<keyword> N", e.g. "bushido 2" -/
semantic_template keywordNumber (label : KeywordLabel) (amount : Amount) : Ability :=
  .keyword label [.number amount] []
/-- "<keyword> <quality> <cost>", e.g. "plainscycling {2}" [CR#702.29e] -/
semantic_template keywordQualityCosting (label : KeywordLabel) (quality : Predicate) (cost : Cost) : Ability :=
  .keyword label [.quality quality, .cost cost] []
/-- "<keyword> N—<cost>", e.g. "suspend 4—{1}{U}" -/
semantic_template keywordNumberCosting (label : KeywordLabel) (amount : Amount) (cost : Cost) : Ability :=
  .keyword label [.number amount, .cost cost] []
/-- "Level up [cost]" [CR#702.87a] -/
semantic_definition levelUp (cost : Cost) : Ability := keywordCosting (.named "levelUp") cost
/-- "When <event>, if <condition>, <instruction>" -/
semantic_template triggeredIf (event : GameEvent) (condition : Condition)
    (instruction : Action) : Ability :=
  .triggered event [] none [] none none (some condition) instruction
/-- "Whenever <event>, <instruction>. This ability triggers only once each turn." -/
semantic_template triggeredOnlyOnce (event : GameEvent) (limit : UsageLimit) (instruction : Action) :
    Ability :=
  .triggered event [] none [] none (some limit) none instruction
/-- "As <subject> enters, choose a <quality>." -/
semantic_template entersChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  Primitives.StaticSpec.entryChoice subject (.quality sort) none .openly
/-- "As <subject> enters, choose a <quality> from <domain>." -/
semantic_template entersChoosingFrom (subject : NounPhrase) (sort : QualitySort) (domain : ChoiceDomain) :
    StaticSpec :=
  Primitives.StaticSpec.entryChoice subject (.quality sort) (some domain) .openly
/-- "As <subject> enters, choose a player [from <domain>]." -/
semantic_template entersChoosingPlayer (subject : NounPhrase) (domain : Option ChoiceDomain) : StaticSpec :=
  Primitives.StaticSpec.entryChoice subject .player domain .openly
/-- "As <subject> enters, secretly choose a player [from <domain>]." -/
semantic_template entersChoosingPlayerSecretly (subject : NounPhrase) (domain : Option ChoiceDomain) :
    StaticSpec :=
  Primitives.StaticSpec.entryChoice subject .player domain .secretly
/-- "<subject> enters tapped" -/
semantic_template entersTapped (subject : NounPhrase) : StaticSpec := .entryRider subject (.entersAs .tapped)
/-- "As <subject> becomes attached, choose a <quality>." -/
semantic_template attachChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  Primitives.StaticSpec.attachmentChoice subject (.quality sort) none
/-- "<subject> enters with N <kind> counters on it" -/
semantic_template entersWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .fresh)
/-- "<subject> enters with an additional N <kind> counters on it" -/
semantic_template entersWithAdditionalCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .additional)
/-- "<subject> enters with N fewer <kind> counters on it" -/
semantic_template entersWithFewerCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .fewer)

semantic_template triggered (event : GameEvent) (instruction : Action) : Ability :=
  .triggered event [] none [] none none none instruction
/-- "Whenever <event> and whenever <joined events>, <instruction>" -/
semantic_template triggeredJoined (event : GameEvent) (joins : List JoinedHeader) (instruction : Action) :
    Ability :=
  .triggered event [] none joins none none none instruction
/-- A further "whenever <event>" header joined onto a trigger. -/
semantic_template joinedHead (event : GameEvent) : JoinedHeader := ⟨event, [], none, none⟩
/-- "Whenever <event> or <alternatives>, <instruction>" -/
semantic_template triggeredOr (event : GameEvent) (alternatives : List GameEvent) (instruction : Action) :
    Ability :=
  .triggered event alternatives none [] none none none instruction
/-- "Whenever <event> while <concurrent>, <instruction>" -/
semantic_template triggeredWhile (event : GameEvent) (while_ : Timing) (instruction : Action) :
    Ability :=
  .triggered event [] (some while_) [] none none none instruction
/-- "Whenever <event> during <timing>, <instruction>" -/
semantic_template triggeredOnlyDuring (event : GameEvent) (timing : Timing) (instruction : Action) :
    Ability :=
  .triggered event [] none [] (some timing) none none instruction
/-- A joined "whenever <event> while <concurrent>" header. -/
semantic_template joinedHeadWhile (event : GameEvent) (while_ : Timing) : JoinedHeader :=
  ⟨event, [], some while_, none⟩
/-- "When <event>, <instruction>" -/
semantic_template when (event : GameEvent) (instruction : Action) : Ability := triggered event instruction
/-- "Whenever <event>, <instruction>" -/
semantic_template whenever (event : GameEvent) (instruction : Action) : Ability :=
  triggered event instruction
/-- "At <event>, <instruction>" (`at` is a Lean keyword). -/
semantic_template at_ (event : GameEvent) (instruction : Action) : Ability := triggered event instruction
/-- "After <event>, <instruction>": the dice template's word. -/
semantic_template after (event : GameEvent) (instruction : Action) : Ability := triggered event instruction

/-- Renown N's reminder text: "When this creature deals combat damage to a player, if it isn't
renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
semantic_definition renownExpansion (count : Nat) : Ability :=
  triggeredIf (dealsCombatDamage thisCreature (a .anyPlayer))
    (.not (.matches thisCreature (.hasDesignation (.named "renowned") none)))
    (.sequentially
      [ .putCounters (.lit count) (.printed p1p1Counter) thisCreature .one true,
        .gainDesignation thisCreature (.named "renowned") none ])
/-- Storm's reminder text: "When you cast this spell, copy it for each other spell that was cast
before it this turn. You may choose new targets for the copies." [CR#702.40a] -/
semantic_definition stormExpansion : Ability :=
  when (.does (.actor .you) (.procedure .cast thisSpell .itsOwnCost none none none))
    (.sequentially
      [ .copy .fromStack thisSpell (eventCount (.procedure .cast (a (.and [spell, .otherThan thisSpell])) .itsOwnCost none none none) (.actor (a .anyPlayer)) earlierThisTurn) [],
        offer (.chooseNewTargets (.pro (.word .copy) .many)) ])
/-- "Storm" with its reminder text. -/
semantic_definition storm : Ability := .keyword (.named "storm") [] [stormExpansion]
/-- Cascade's reminder text [CR#702.85a]. -/
semantic_definition cascadeExpansion : Ability :=
  let cheaper : Predicate :=
    .compare [.stat .manaValue] .less (.statOf (.stat .manaValue) thisSpell)
  when (.does (.actor .you) (.procedure .cast thisSpell .itsOwnCost none none none))
    (.sequentially
      [ exile (.run .top (.and [.not land, cheaper]) (library .you)),
        offer (Actor.castWithoutPaying (that .card) (some cheaper)),
        .move (allFromAmong (.not (.did ⟨.any, .procedure .cast .which .itsOwnCost none none none, .thisWay⟩)) (theVerbed (.action "exile") .many)) .anywhere (some onBottom)
          .arrival none [.placement (.inOrder .randomOrder)] ])
/-- "Cascade" with its reminder text. -/
semantic_definition cascade : Ability := .keyword (.named "cascade") [] [cascadeExpansion]
/-- Ripple N's definition [CR#702.60a]: "When you cast this spell, you may reveal the top N cards
of your library. If you do, you may cast any of those cards with the same name as this spell
without paying their mana costs, then put all revealed cards not cast this way on the bottom of
your library in any order." -/
semantic_definition rippleExpansion (amount : Amount) : Ability :=
  when (.does (.actor .you) (.procedure .cast thisSpell .itsOwnCost none none none))
    (Primitives.Action.offer (revealCards (.slice .top amount (library .you)))
      (some (.sequentially
        [ Actor.castWithoutPaying
            (.someOf (.counted anyNumber) (some (.shares .cardName .share .this)) (those .card)) none,
          .move (allFromAmong (.not (.did ⟨.any, .procedure .cast .which .itsOwnCost none none none, .thisWay⟩)) (theVerbed (.action "reveal") .many (some .card))) .anywhere
            (some onBottom) .arrival none [.placement (.inOrder .anyOrder)] ]))
      none)
/-- "Ripple N" with its definition [CR#702.60a]. -/
semantic_definition ripple (amount : Amount) : Ability :=
  .keyword (.named "ripple") [.number (Semantics.Amount.inCaller amount)] [rippleExpansion (Semantics.Amount.inCaller amount)]
/-- "Renown N" with its reminder text. -/
semantic_definition renown (count : Nat) : Ability :=
  .keyword (.named "renown") [.number (.lit count)] [renownExpansion count]
/-- Cumulative upkeep's reminder text [CR#702.24a]. -/
semantic_definition cumulativeUpkeepExpansion (cost : Cost) : Ability :=
  triggeredIf (.beginningOf .upkeep (some .you) (.each none))
    (.matches thisPermanent permanent)
    (.sequentially
      [ .putCounters (.lit 1) (.printed (.named "ageCounter")) thisPermanent .one true,
        Primitives.Action.offer (.pay (.scaled cost (times (.lit 1) (countersOn (.named "ageCounter") thisPermanent))) .once .paid none none)
          none (some (sacrifice thisPermanent)) ])
/-- "Cumulative upkeep [cost]" with its reminder text. -/
semantic_definition cumulativeUpkeep (cost : Cost) : Ability :=
  .keyword (.named "cumulativeUpkeep") [.cost (Semantics.Cost.inCaller cost)] [cumulativeUpkeepExpansion (Semantics.Cost.inCaller cost)]
/-- Flying's definition, a static ability: "This creature can't be blocked except by creatures
with flying and/or reach." [CR#702.9b] -/
semantic_definition flyingExpansion : Ability :=
  .static (.cant .any (.block thisCreature)
    { rider := .noRider,
      except := some (.byObjects (.and [creature,
        .or [.hasKeyword (.the (.named "flying")), .hasKeyword (.the (.named "reach"))]])) })
/-- "Flying" with its definition. -/
semantic_definition flying : Ability := .keyword (.named "flying") [] [flyingExpansion]
/-- Bushido N's definition, a triggered ability [CR#702.45a]. -/
semantic_definition bushidoExpansion (count : Nat) : Ability :=
  triggeredOr (blocks thisCreature none) [becomesBlocked thisCreature none]
    (get thisCreature (.up (.lit count)) (.up (.lit count)) (some untilEndOfTurn))
/-- "Bushido N" with its definition. -/
semantic_definition bushido (count : Nat) : Ability :=
  .keyword (.named "bushido") [.number (.lit count)] [bushidoExpansion count]
/-- Modular N's definition, a static ability and a triggered one [CR#702.43a]. -/
semantic_definition modularExpansion (count : Nat) : List Ability :=
  [ .static (entersWithCounters thisPermanent (.lit count) p1p1Counter),
    when (Primitives.GameEvent.dies thisPermanent)
      (offer (.putCounters (countersOn p1p1Counter thisPermanent) (.printed p1p1Counter)
        (target (.and [artifact, creature])) .one true)) ]
/-- "Modular N" with its definition. -/
semantic_definition modular (count : Nat) : Ability :=
  .keyword (.named "modular") [.number (.lit count)] (modularExpansion count)
semantic_template activated (cost : Cost) (instruction : Action) : Ability :=
  .activated cost instruction none none none none
/-- "[cost]: <instruction>. Activate only <timing>." -/
semantic_template activatedOnlyDuring (cost : Cost) (instruction : Action) (timing : Timing) : Ability :=
  .activated cost instruction (some timing) none none none
/-- "[cost]: <instruction>. Only <who> may activate this ability." -/
semantic_template activatedBy (cost : Cost) (instruction : Action) (who : NounPhrase) : Ability :=
  .activated cost instruction none none none (some who)
/-- "[cost]: <instruction>. Activate only once <limit>." -/
semantic_template activatedOnlyOnce (cost : Cost) (instruction : Action) (limit : UsageLimit) : Ability :=
  .activated cost instruction none (some limit) none none
/-- "[cost]: <instruction>. Activate only if <guard>." -/
semantic_template activatedOnlyIf (cost : Cost) (instruction : Action) (guard : Condition) : Ability :=
  .activated cost instruction none none (some guard) none
/-- `ability` with the restrictions a keyword or a granting effect adds to it [CR#113.10a]. -/
semantic_template qualified (ability : Ability) (timing : Option Timing := none)
    (limit : Option UsageLimit := none) (guard : Option Condition := none)
    (from_ : Option Zone := none) : Ability :=
  .qualified ability timing limit guard from_
/-- "<subject> has <selection> of <source>" (backup [CR#702.165a]). -/
semantic_template abilityGrantSelected (subject : NounPhrase) (source : NounPhrase)
    (selection : AbilitySelection) : StaticSpec :=
  .abilityGrantSelected subject source selection
/-- "[cost]: <instruction>. Activate only once <limit> and only if <guard>." -/
semantic_template activatedOnlyOnceIf (cost : Cost) (instruction : Action) (limit : UsageLimit)
    (guard : Condition) : Ability :=
  .activated cost instruction none (some limit) (some guard) none

/-- Cycling [cost]'s definition, an activated ability: "[Cost], Discard this card: Draw a card."
[CR#702.29a] -/
semantic_definition cyclingExpansion (cost : Cost) : Ability :=
  activated (.compound [cost, .perform (discard .this)]) (draw (.lit 1))
/-- Flash's definition: "You may play this card any time you could cast an instant."
[CR#702.8a] -/
semantic_definition flashExpansion : Ability :=
  .static (mayPlayDeed (.action "play") .you .this none
    (.play none none (some asInstant) false .itsOwnCost))
/-- Sneak [cost]'s definition [CR#702.190a]. -/
semantic_definition sneakExpansion (cost : Cost) : Ability :=
  .static (mayPlayDeed (.action "cast") .you .this none
    (.play none none (some (.and [asInstant, duringPart .declareBlockers (some .you)])) false
      (.payingInstead (.compound [cost,
        .perform (returnTo (a (.and [creature, unblocked, .hasPossessor .controller .you])) hand [])]))))
/-- Morph [cost]'s definition [CR#702.37a,702.37e,116.2b]. -/
semantic_definition morphExpansion (cost : Cost) : List Ability :=
  [ .static (mayPlayDeed (.action "cast") .you .this none
      (.play none none none false (.faceDown faceDownCreature (.mana [generic 3])))),
    .static (.specialAction none cost (.setStatus .faceUp .this)) ]
/-- Megamorph [cost]'s definition [CR#702.37b]. -/
semantic_definition megamorphExpansion (cost : Cost) : List Ability :=
  [ .static (mayPlayDeed (.action "cast") .you .this none
      (.play none none none false (.faceDown faceDownCreature (.mana [generic 3])))),
    .static (.specialAction none cost
      (.sequentially [.setStatus .faceUp .this, .putCounters (.lit 1) (.printed p1p1Counter) .this .one true])) ]
/-- Disguise [cost]'s definition [CR#702.168a,702.168d]. -/
semantic_definition disguiseExpansion (cost : Cost) : List Ability :=
  [ .static (mayPlayDeed (.action "cast") .you .this none
      (.play none none none false (.faceDown faceDownWardedCreature (.mana [generic 3])))),
    .static (.specialAction none cost (.setStatus .faceUp .this)) ]
/-- Foretell [cost]'s definition [CR#702.143a,116.2h]. -/
semantic_definition foretellExpansion (cost : Cost) : List Ability :=
  [ .static (.specialAction (some (duringPart .turn (some .you))) (.mana [generic 2])
      (.sequentially
        [ exileFaceDown .this,
          .establish (.visibility .lookAt .you (.objects it))
            (some (.forAsLongAs (.matches it (.inZone exileZone)))),
          .establish
            (mayPlayDeed (.action "cast") .you it none
              (.play (some exileZone) none none false (.payingInstead cost)))
            (some onALaterTurn) ])) ]
/-- Plot [cost]'s definition [CR#702.170a,702.170d,116.2k]. -/
semantic_definition plotExpansion (cost : Cost) : List Ability :=
  [ .static (.specialAction (some asSorcery) cost
      (.sequentially
        [ exile .this,
          .establish
            (mayPlayDeed (.action "cast") .you it none
              (.play (some exileZone) none (some asSorcery) false .withoutPaying))
            (some onALaterTurn) ])) ]
/-- Hideaway N's definition [CR#702.75a]. -/
semantic_definition hideawayExpansion (amount : Amount) : List Ability :=
  let exiledCard : NounPhrase := theVerbed (.action "exile") .one (some .card)
  [ when (Primitives.GameEvent.enters thisPermanent none)
      (.sequentially
        [ .expose .lookAt (.cards (.slice .top amount (library .you))),
          exileFaceDown (someOf (exactly 1) them),
          .move (theRest .object) .anywhere (some onBottom) .arrival none [.placement (.inOrder .randomOrder)],
          .establish (.visibility .lookAt .you (.objects exiledCard))
            (some (.forAsLongAs (.matches exiledCard (.inZone exileZone)))) ]) ]
/-- "Cycling [cost]" with its definition. -/
semantic_definition cycling (cost : Cost) : Ability :=
  .keyword (.named "cycling") [.cost (Semantics.Cost.inCaller cost)] [cyclingExpansion (Semantics.Cost.inCaller cost)]

/-! ## Cards -/

/-- A printed power, toughness, loyalty, or defense. -/
semantic_template stat (value : Nat) : Option Amount := some (.lit value)
/-- One level band of a leveler: its range, power/toughness box, and text [CR#711.2a]. -/
semantic_template levelBand (range : LevelRange) (power toughness : Nat) (text : List Ability) : LevelBand :=
  { range, text, power := stat power, toughness := stat toughness }
/-- A prototype frame's inset set: its mana cost and power/toughness box [CR#718.1]. -/
semantic_template prototypeAlt (cost : ManaCost) (power toughness : Nat) : PrototypeFrame :=
  { cost := some cost, power := stat power, toughness := stat toughness }

/-- A Food token's characteristics: "a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" [CR#111.10b] -/
semantic_template foodToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [artifactType "Food"],
        text :=
          [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice thisArtifact)])
              (gainLife (.lit 3)) ] } }
/-- A Treasure token's characteristics: "a colorless Treasure artifact token with '{T},
Sacrifice this token: Add one mana of any color.'" [CR#111.10a] -/
semantic_template treasureToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [artifactType "Treasure"],
        text :=
          [ activated (.compound [.tapSymbol, .perform (sacrifice thisArtifact)])
              (.addMana (.lit 1) (.anyColor .sameColor) []) ] } }
/-- "create N tapped <token>" [CR#111.2] -/
semantic_template createTappedToken (token : CharacteristicBundle) (count : Amount := .lit 1) :
    Action :=
  Primitives.Action.create count (.written token) [.entersAs .tapped]

theorem okAirbendOne :
    (airbend (target creature)).check (DRS.ofList []) = [] := by decide

theorem okAirbendUpToTwo :
    (airbend (.described (.target (upTo 2)) creature)).check (DRS.ofList []) = [] := by decide

theorem okConniveOne :
    (connive (.lit 1)).check (DRS.ofList []) = [] := by decide

theorem okConniveTarget :
    (instruct (target creature) (connive (.lit 2))).check (DRS.ofList []) = [] := by decide

theorem okRippleFour :
    (ripple (.lit 4)).check = [] := by decide

end Semantics.Macros

