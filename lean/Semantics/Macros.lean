import Semantics
import Semantics.Macros.Primitives
import Semantics.Check
import Semantics.MacroParameters

/-!
# Semantics.Macros

Spellings over the raw constructors. Port of `idris/src/Experimental/Macros.idr`, the
subset the `Cards` bench uses; every macro keeps its Idris name and argument order, minus
the obligations it forwarded.

The Idris bench qualifies every macro (`Macros.target`) so a reader can tell a spelling from
a constructor at a glance. Lean does not need that: a constructor is written with a leading
dot (`.hasType`), a macro without (`creature`), so the bench opens `Semantics.Macros` and writes
them bare.
-/

namespace Semantics.Macros

/-! ## Pronouns -/

/-- "it" -/
semantic_macro it : NounPhrase := .pro .bare .one .whole

/-- "them" -/
semantic_macro them : NounPhrase := .pro .bare .many .whole
/-- "that card": the card slot's occupant. -/
semantic_macro itCard : NounPhrase := .pro (.atSlot .card) .one .whole
/-- "it": the permanent slot's occupant, as "sacrifice it" reads it. -/
semantic_macro itPermanent : NounPhrase := .pro (.atSlot .permanent) .one .whole
/-- "that token": the token just created. -/
semantic_macro itAsToken : NounPhrase := .pro .tokenBorn .one .whole

/-- "it": the object the previous instruction introduced, read through a window over exactly
what that instruction announced. -/
semantic_macro itPrior (prev : Instruction) : NounPhrase :=
  .pro .bare .one (.introduced ((Instruction.intro [] prev).map Binding.kind))
/-- "it", read as the subject of the condition just stated. -/
semantic_macro itCondSubject (condition : Condition) : NounPhrase :=
  .pro .bare .one (.introduced ((Condition.introduced [] condition).map Binding.kind))

/-- "that <word>", e.g. `that .player`. -/
semantic_macro that (w : NounWord) : NounPhrase := .pro (.word w) .one .whole
/-- "that player or planeswalker": the joined phrase just named. -/
semantic_macro thatJoin : NounPhrase := that .join

/-- "those <word>s", e.g. `those .card`. -/
semantic_macro those (w : NounWord) : NounPhrase := .pro (.word w) .many .whole

/-- "they": the player most recently named. -/
semantic_macro they : NounPhrase := .pro (.word .player) .one .whole
/-- "that player or that permanent's controller": the player half of a "target player or
planeswalker" split, or the controller of the permanent half. -/
semantic_macro splitOverPlaneswalker : NounPhrase :=
  Primitives.NounPhrase.eitherOf (.pro (.unionHalf .player) .one .whole)
    (.possessorOf .controller (.pro (.unionHalf (.type .planeswalker)) .one .whole))
/-- The same over "any target": the player, or the permanent's controller. -/
semantic_macro splitOverPermanent : NounPhrase :=
  Primitives.NounPhrase.eitherOf (.pro (.unionHalf .player) .one .whole)
    (.possessorOf .controller (.pro (.unionHalf .permanent) .one .whole))

/-- "the <verb>ed <word>", e.g. "the tapped creatures"; with no word, the pronoun the deed
stamped: "it" or "them", "the exiled card". -/
semantic_macro theVerbed (verb : Deed) (plurality : Plurality) (word : Option NounWord := none)
    (marking : VerbedMarking := .attributive) : NounPhrase :=
  .pro (.verbed verb word marking) plurality .whole

/-! ## Quantities -/

semantic_macro exactly (n : Nat) : Quantity := .range (some n) (some n)
semantic_macro upTo (n : Nat) : Quantity := .range none (some n)
semantic_macro anyNumber : Quantity := .range none none
semantic_macro atLeast (n : Nat) : Quantity := .range (some n) none
semantic_macro oneThrough (n : Nat) : Quantity := .range (some 1) (some n)

/-! ## Determiners -/

/-- "target …" -/
semantic_macro target (p : Predicate) : NounPhrase := .described (.target (.range (some 1) (some 1))) p

/-- "each …" -/
semantic_macro each (p : Predicate) : NounPhrase := .described .each p

/-- "all …" -/
semantic_macro allOf (p : Predicate) : NounPhrase := .described .all p

/-- the bare plural: "creatures" -/
semantic_macro bare (p : Predicate) : NounPhrase := .described .bare p

/-- "a …" -/
semantic_macro a (p : Predicate) : NounPhrase := .described (.a .unmarked) p

/-- "a … at random" -/
semantic_macro aAtRandom (p : Predicate) : NounPhrase := .described (.a .atRandom) p
/-- "a … of their choice", the chooser being the last-mentioned player. -/
semantic_macro aTheirChoice (p : Predicate) : NounPhrase := .described (.a (.theirChoice (.top 1))) p
/-- "a … of your choice" -/
semantic_macro aYourChoice (p : Predicate) : NounPhrase := .described (.a .yourChoice) p
/-- "the …" -/
semantic_macro the (p : Predicate) : NounPhrase := .described .the p

/-- "N …", "up to N …" -/
semantic_macro counted (q : Quantity) (p : Predicate) : NounPhrase := .described (.count q none) p
/-- "N <p> at random" -/
semantic_macro countedAtRandom (q : Quantity) (p : Predicate) : NounPhrase :=
  .described (.count q (some .atRandom)) p

/-- "if there is a …" -/
semantic_macro exists_ (p : Predicate) : Condition := .exists_ (bare p)

/-- "the number of …s" -/
semantic_macro countOf (p : Predicate) : Amount := .countOf (bare p)

/-- "the greatest power among …s" -/
semantic_macro aggregate (op : AggregateOp) (axis : ProjAxis) (p : Predicate) : Amount :=
  .aggregate op axis (bare p)

/-! ## Zones -/

semantic_macro battlefield : ZoneExpr := .zone .battlefield .bare
semantic_macro exileZone : ZoneExpr := .zone .exile .bare
semantic_macro hand : ZoneExpr := .zone .hand .bare
semantic_macro graveyard : ZoneExpr := .zone .graveyard .bare
semantic_macro stack : ZoneExpr := .zone .stack .bare
semantic_macro command : ZoneExpr := .zone .command .bare
/-- "the command zone", scoped to nobody. -/
semantic_macro commandZone : ZoneExpr := .zone .command .bare
semantic_macro yourLibrary : ZoneExpr := .zone .library (.possessedBy .you)
/-- "<player>'s library": a face-down pile [CR#401.2]. -/
semantic_macro library (player : NounPhrase) : Pile := .library (.possessedBy player)
/-- "on the bottom of its owner's library" -/
semantic_macro onBottom : ZoneExpr := .place .bottom (.library .bare)
/-- "on top of its owner's library" -/
semantic_macro onTop : ZoneExpr := .place .top (.library .bare)
semantic_macro handOf (player : NounPhrase) : ZoneExpr := .zone .hand (.possessedBy player)
/-- "your hand" -/
semantic_macro yourHand : ZoneExpr := handOf .you
semantic_macro graveyardOf (player : NounPhrase) : ZoneExpr := .zone .graveyard (.possessedBy player)

/-! ## Predicates -/

semantic_macro creature : Predicate := .hasType .creature
semantic_macro artifact : Predicate := .hasType .artifact
semantic_macro enchantment : Predicate := .hasType .enchantment
semantic_macro land : Predicate := .hasType .land
semantic_macro instant : Predicate := .hasType .instant
semantic_macro sorcery : Predicate := .hasType .sorcery
semantic_macro instantOrSorcery : Predicate := .or [instant, sorcery]
/-- "has a permanent type": an artifact, battle, creature, enchantment, land, or planeswalker
[CR#110.4]; read off the types, so it holds of a card or a token. -/
semantic_macro permanentType : Predicate :=
  .or [ .hasType .artifact, .hasType .battle, .hasType .creature, .hasType .enchantment,
        .hasType .land, .hasType .planeswalker ]
semantic_macro tapped : Predicate := .hasStatus .tapped
semantic_macro faceDown : Predicate := .hasStatus .faceDown
/-- "another player": any player other than you. -/
semantic_macro otherPlayer : Predicate := .and [.anyPlayer, .otherThan .you]
/-- "spell": an object on the stack that is not an ability [CR#112.1,113.1]. -/
semantic_macro spell : Predicate := .and [.inZone stack, .not (.abilityHead .anyOnStack)]
semantic_macro untapped : Predicate := .hasStatus .untapped
semantic_macro nontoken : Predicate := .not .isToken
/-- "permanent": an object on the battlefield [CR#110.1]. -/
semantic_macro permanent : Predicate := .inZone battlefield
/-- "permanent card": a card that could be put onto the battlefield, one with an artifact,
battle, creature, enchantment, land, or planeswalker type [CR#110.4a]; a reading of the card's
types, wherever it is. -/
semantic_macro permanentCard : Predicate :=
  .and
    [ .isCard,
      .or
        [ .hasType .artifact, .hasType .battle, .hasType .creature, .hasType .enchantment,
          .hasType .land, .hasType .planeswalker ] ]
/-- "colorless": an object of no color [CR#105.2c]. -/
semantic_macro colorless : Predicate := .colorCount .eq 0
/-- "multicolored": an object of two or more colors [CR#105.2b]. -/
semantic_macro multicolored : Predicate := .colorCount .atLeast 2
/-- "monocolored" -/
semantic_macro monocolored : Predicate := .colorCount .eq 1
/-- "historic": an artifact, legendary, or Saga [CR#700.6]. -/
semantic_macro historic : Predicate :=
  .or [.hasType .artifact, .hasSupertype .legendary, .hasSubtype (.of .enchantment "Saga")]
semantic_macro attacking : Predicate := .inCombat .attackerOf none
semantic_macro blocking : Predicate := .inCombat .blockerOf none
semantic_macro blocked : Predicate := .inCombat .blockedBy none
semantic_macro unblocked : Predicate := .not blocked
semantic_macro creatureYouControl : Predicate := .and [creature, .hasPossessor .controller .you]
semantic_macro creatureYouDontControl : Predicate := .and [creature, .not (.hasPossessor .controller .you)]
semantic_macro creatureYourOpponentsControl : Predicate :=
  .and [creature, .hasPossessor .controller (.playerGroup .yourOpponents)]
/-- "another creature", anchored on <n>. -/
semantic_macro otherCreature (n : NounPhrase) : Predicate := .and [creature, .otherThan n]
/-- "another creature you control", anchored on <n>. -/
semantic_macro otherCreatureYouControl (n : NounPhrase) : Predicate :=
  .and [creature, .hasPossessor .controller .you, .otherThan n]
/-- "source": the object dealing the damage under discussion. -/
semantic_macro source : Predicate := .isSource
semantic_macro emblem : Predicate := .isEmblem
/-- "the chosen player" -/
semantic_macro chosenPlayer : Predicate := .chosenPlayer .theChoice
/-- "the last chosen player" -/
semantic_macro theLastChosenPlayer : Predicate := .chosenPlayer .theLatestChoice
/-- "of the chosen <quality>" -/
semantic_macro ofChosen (sort : QualitySort) : Predicate := .ofChosen .theChoice sort
/-- "of the last chosen <quality>" -/
semantic_macro ofTheLastChosen (sort : QualitySort) : Predicate := .ofChosen .theLatestChoice sort
semantic_macro copyOfACard : Predicate := .isCopyOfACard
semantic_macro cardOnTheStack : Predicate := .and [.isCard, spell]
semantic_macro tokenOnTheBattlefield : Predicate := .and [.isToken, permanent]

/-- The four party roles, in rule order [CR#700.8]. -/
semantic_macro partyRoles : List Predicate :=
  [ .hasSubtype (.of .creature "Cleric"), .hasSubtype (.of .creature "Rogue"),
    .hasSubtype (.of .creature "Warrior"), .hasSubtype (.of .creature "Wizard") ]

/-- "any target" [CR#115.4]. -/
semantic_macro anyTarget : Predicate :=
  .or [.hasType .creature, .hasType .planeswalker, .hasType .battle, .anyPlayer]

/-- "cast by <player>" -/
semantic_macro castBy (player : NounPhrase) : Predicate := .castBy player none
/-- "the Nth spell <player> cast <period>" -/
semantic_macro nthCastBy (ordinal : Ordinal) (player : NounPhrase) (period : Period) : Predicate :=
  .castBy player (some (ordinal, period))
/-- "any other target" -/
semantic_macro anyOtherTarget : Predicate := .and [anyTarget, .other]

/-- "a color", "a creature type": a quality noun. -/
semantic_macro quality (sort : QualitySort) : Predicate := .qualityNoun sort none
/-- A quality noun with its domain: "a color other than blue". -/
semantic_macro qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate :=
  .qualityNoun sort (some domain)
/-- "itself": the member a comparison over a domain is measuring, the one subject binding
`Predicate.check` places immediately before the measure. -/
semantic_macro ownMember : NounPhrase := .pro .bare .one (.top 1)
/-- "a <dom> with <stat> <r> <bound>": the stat of `member`, the member itself unless written. -/
semantic_macro comparesOwnStat (stat : Stat) (domain : Predicate) (comparator : Comparator) (bound : Amount)
    (member : NounPhrase := ownMember) : Predicate :=
  .compareOver domain (.statOf (.stat stat) member) comparator bound
/-- "the chosen color" -/
semantic_macro thatColor : ColorTerm := .chosen .theChoice

semantic_macro creatureType (label : String) : Subtype := .of .creature label
semantic_macro artifactType (label : String) : Subtype := .of .artifact label
semantic_macro landType (label : String) : Subtype := .of .land label
semantic_macro enchantmentType (label : String) : Subtype := .of .enchantment label
semantic_macro spellType (label : String) : Subtype := .spell label
semantic_macro planeswalkerType (label : String) : Subtype := .of .planeswalker label
/-- An outlaw: an Assassin, Mercenary, Pirate, Rogue, or Warlock [CR#700.12]. -/
semantic_macro outlaw : Predicate :=
  .or [ .hasSubtype (creatureType "Assassin"), .hasSubtype (creatureType "Mercenary"),
        .hasSubtype (creatureType "Pirate"), .hasSubtype (creatureType "Rogue"),
        .hasSubtype (creatureType "Warlock") ]
semantic_macro outlawYouControl : Predicate := .and [outlaw, .hasPossessor .controller .you]
/-- "an Army you control" -/
semantic_macro armyYouControl : Predicate :=
  .and [.hasSubtype (creatureType "Army"), creature, .hasPossessor .controller .you]

/-! ## Nouns -/

semantic_macro anOpponent : NounPhrase := a .opponent
semantic_macro thisCreature : NounPhrase := .asType .creature .this none
semantic_macro thisArtifact : NounPhrase := .asType .artifact .this none
semantic_macro thisLand : NounPhrase := .asType .land .this none
semantic_macro thisPermanent : NounPhrase := .asMarker .permanent .this
semantic_macro thisSpell : NounPhrase := .asMarker .spell .this
semantic_macro thisRoom : NounPhrase := .asType .enchantment .this (some (enchantmentType "Room"))
semantic_macro thisPlaneswalker : NounPhrase := .asType .planeswalker .this none
semantic_macro thisAbility : NounPhrase := .asMarker .ability .this
semantic_macro thisEnchantment : NounPhrase := .asType .enchantment .this none
semantic_macro thisAura : NounPhrase := .asType .enchantment .this (some (enchantmentType "Aura"))
semantic_macro thisEquipment : NounPhrase := .asType .artifact .this (some (artifactType "Equipment"))
semantic_macro thisSiege : NounPhrase := .asType .battle .this (some (.of .battle "Siege"))
semantic_macro thisVehicle : NounPhrase := .asType .artifact .this (some (artifactType "Vehicle"))
semantic_macro thisSpacecraft : NounPhrase := .asType .artifact .this (some (artifactType "Spacecraft"))
semantic_macro thisSaga : NounPhrase := .asType .enchantment .this (some (enchantmentType "Saga"))
semantic_macro thisCase : NounPhrase := .asType .enchantment .this (some (enchantmentType "Case"))
semantic_macro thisClass : NounPhrase := .asType .enchantment .this (some (enchantmentType "Class"))
/-- "your commander" -/
semantic_macro yourCommander : NounPhrase := .designated (.named "commander") .you
/-- "a card exiled with this artifact" -/
semantic_macro exiledWithThisArtifact : Predicate := .exiledWith thisArtifact
/-- "the rest of them" -/
semantic_macro theRest (kind : Kind) : NounPhrase := .theRest kind .many
/-- "the other pile": the one remaining after a choice among two. -/
semantic_macro theOther (kind : Kind) : NounPhrase := .theRest kind .one
/-- "N of <group>" -/
semantic_macro someOf (quantity : Quantity) (group : NounPhrase) : NounPhrase :=
  .someOf (.counted quantity) none group
/-- "among <group>": the whole of a group, sliced. -/
semantic_macro among (group : NounPhrase) : NounPhrase := .someOf .whole none group
/-- "N <p> from among <group>" -/
semantic_macro fromAmong (quantity : Quantity) (p : Predicate) (group : NounPhrase) : NounPhrase :=
  .someOf (.counted quantity) (some p) group
/-- "<group> with the same name" -/
semantic_macro withTheSameName (group : NounPhrase) : NounPhrase := .namesAgree .sameName group
/-- "<group> with different names" -/
semantic_macro withDifferentNames (group : NounPhrase) : NounPhrase := .namesAgree .differentNames group
/-- "the <p> from among <group>": every member the description picks. -/
semantic_macro allFromAmong (p : Predicate) (group : NounPhrase) : NounPhrase := .someOf .whole (some p) group
/-- "the <p> among <group>" -/
semantic_macro allAmong (p : Predicate) (group : NounPhrase) : NounPhrase := allFromAmong p group
/-- "each object" -/
semantic_macro everyObject : NounPhrase := allOf (.and [])
/-- "one pile": one of the piles just made. -/
semantic_macro onePile : NounPhrase := .pileOf (.counted (exactly 1)) none
/-- "the pile of <player>'s choice" -/
semantic_macro pileOfChoice (player : NounPhrase) : NounPhrase := .pileOf (.counted (exactly 1)) (some player)
/-- "you and <subject>" -/
semantic_macro youAnd (subject : NounPhrase) : NounPhrase := Primitives.NounPhrase.both .you subject
/-- "you or <subject>" -/
semantic_macro youOr (subject : NounPhrase) : NounPhrase := Primitives.NounPhrase.eitherOf .you subject
semantic_macro controllerOf (subject : NounPhrase) : NounPhrase := .possessorOf .controller subject
semantic_macro ownerOf (subject : NounPhrase) : NounPhrase := .possessorOf .owner subject
/-- "the top N cards of your library" -/
semantic_macro topSlice (amount : Amount) : NounPhrase := .slice .top amount (library .you)
/-- "the bottom card of your library" -/
semantic_macro bottomCard : NounPhrase := .slice .bottom (.lit 1) (library .you)
/-- The stack after "look at the top N cards of your library". -/
semantic_macro lookedTop (bs : Bindings) (amount : Amount) : Bindings := nomIntro bs (topSlice amount)
/-- "[a player]'s party" [CR#700.8]. -/
semantic_macro partyOf (player : NounPhrase) : NounPhrase :=
  .oneEachOf partyRoles (allOf (.and [creature, .hasPossessor .controller player]))
/-- "your party" -/
semantic_macro party : NounPhrase := partyOf .you
/-- "the number of creatures in <player>'s party" [CR#700.8a]. -/
semantic_macro partySizeOf (player : NounPhrase) : Amount := .countOf (partyOf player)
semantic_macro partySize : Amount := partySizeOf .you
/-- "<player> has a full party": four creatures in that party [CR#700.8c]. -/
semantic_macro fullPartyOf (player : NounPhrase) : Condition := .compareAmt (partySizeOf player) .eq (.lit 4)
semantic_macro fullParty : Condition := fullPartyOf .you

/-! ## Mana -/

semantic_macro generic (amount : Nat) : ManaSymbol := .simple (.generic amount)
semantic_macro pip (color : Color) : ManaSymbol := .simple (.specific (.of color))
/-- "{C}" -/
semantic_macro colorlessPip : ManaSymbol := .simple (.specific .colorless)
/-- "{A/B}" -/
semantic_macro hybridPip (left right : Color) : ManaSymbol := .hybrid (.specific (.of left)) right
/-- "{G/P}": a Phyrexian mana symbol [CR#107.4f]. -/
semantic_macro phyrexianPip (color : Color) : ManaSymbol := .phyrexian color none
/-- "{1} for each …", "{R} for each …": a mana cost scaled by an amount. -/
semantic_macro scaledMana (unit : ManaUnit) (amount : Amount) : Cost :=
  match unit with
  | .generic => .scaled (.mana [generic 1]) amount
  | .run cost => .scaled (.mana cost) amount

/-! ## Durations -/

/-- "until end of turn" -/
semantic_macro untilEndOfTurn : Duration := .between none (some (.endOf .turn none .current))
/-- "this turn", as an effect lasts: until the turn ends. -/
semantic_macro thisTurn : Duration := .between none (some (.endOf .turn none .current))
/-- "until your next turn" -/
semantic_macro untilYourNextTurn : Duration :=
  .between none (some (.beginningOf .turn (some .you) (.next none)))
/-- "until end of combat" -/
semantic_macro untilEndOfCombat : Duration := .between none (some (.endOf .combat none .current))
/-- "until your next end step" -/
semantic_macro untilYourNextEndStep : Duration :=
  .between none (some (.beginningOf .endStep (some .you) (.next none)))
/-- "until <player>'s next turn" -/
semantic_macro untilNextTurnOf (player : NounPhrase) : Duration :=
  .between none (some (.beginningOf .turn (some player) (.next none)))
/-- "until <event>" -/
semantic_macro untilEvent (event : GameEvent) : Duration := .between none (some event)
/-- "for the rest of the game" -/
semantic_macro restOfGame : Duration := .between none none
/-- "during <player>'s next turn" -/
semantic_macro duringNextTurnOf (player : NounPhrase) : Duration :=
  .between (some (.beginningOf .turn (some player) (.next none)))
    (some (.endOf .turn (some player) (.next none)))
/-- "on a later turn": from the end of this turn on [CR#702.143a,702.170d]. -/
semantic_macro onALaterTurn : Duration := .between (some (.endOf .turn none .current)) none

/-! ## History windows

A lookback's span ends at `now` ("this turn" so far), except "last turn", which ended. -/

/-- "this turn", as a lookback reads it: from the turn's beginning to now. -/
semantic_macro soFarThisTurn : Duration := .between (some (.beginningOf .turn none .current)) (some .now)
/-- "earlier this turn" -/
semantic_macro earlierThisTurn : Duration :=
  .between (some (.beginningOf .turn none .current)) (some .now)
/-- "this combat" -/
semantic_macro thisCombat : Duration := .between (some (.beginningOf .combat none .current)) (some .now)
/-- "last turn" -/
semantic_macro lastTurn : Duration :=
  .between (some (.beginningOf .turn none .previous)) (some (.endOf .turn none .previous))
/-- "this game" -/
semantic_macro thisGame : Duration := .between none (some .now)

/-! ## Timings -/

/-- "only as a sorcery": during your main phase, while the stack is empty [CR#307.5]. -/
semantic_macro asSorcery : Timing :=
  .and [.inPart .mainPhase (some .you) .current, .onlyIf (.not (exists_ (.inZone stack)))]
/-- "only as an instant": any time you have priority [CR#117.1a,117.1d]. -/
semantic_macro asInstant : Timing := .onlyIf (.hasPriority .you)
/-- "during <whose> <part>" -/
semantic_macro duringPart (part : TurnPart) (whose : Option NounPhrase := none) : Timing :=
  .inPart part whose .current
/-- "before <whose> <part>" -/
semantic_macro beforePart (part : TurnPart) (whose : Option NounPhrase := none) : Timing :=
  .relativeToEvent .before (.beginningOf part whose .current)
/-- "during each of your turns" -/
semantic_macro duringEachOfYourTurns : Timing := .inPart .turn (some .you) (.each none)
/-- "while you're searching your library" -/
semantic_macro whileSearchingYourLibrary : Timing :=
  .whileDoing (.verbedEvent (some .you) (.action "search") none none (some yourLibrary))
/-- "while <event>" -/
semantic_macro whileDoing (event : GameEvent) : Timing := .whileDoing event
/-- "while <condition>" -/
semantic_macro whileTrue (condition : Condition) : Timing := .onlyIf condition

/-! ## Usage limits -/

/-- "only once each turn" [CR#602.5b] -/
semantic_macro oncePerTurn : UsageLimit := ⟨.moreThan (.lit 1), .turn⟩
/-- "only once each game" -/
semantic_macro oncePerGame : UsageLimit := ⟨.moreThan (.lit 1), .game⟩
/-- "only once during each of your turns" -/
semantic_macro onceEachYourTurn : UsageLimit := ⟨.moreThan (.lit 1), .yourTurn⟩

/-! ## Amounts -/

semantic_macro lifeTotalOf (player : NounPhrase) : Amount := .statOf (.playerStat .lifeTotal) player
semantic_macro powerOf (subject : NounPhrase) : Amount := .statOf (.stat .power) subject
semantic_macro toughnessOf (subject : NounPhrase) : Amount := .statOf (.stat .toughness) subject
semantic_macro countersOn (kind : CounterKind) (holder : NounPhrase) : Amount := .statOf (.counter kind) holder
semantic_macro plus (left right : Amount) : Amount := .arith .plus left right
semantic_macro minus (left right : Amount) : Amount := .arith .minus left right
semantic_macro times (per amount : Amount) : Amount := .arith .times per amount
/-- "that much damage prevented this way" -/
semantic_macro preventedThisWay : Amount := .theOutcome .damagePrevented
/-- "N for each <p>" -/
semantic_macro forEach (per : Nat) (p : Predicate) : Amount := times (.lit per) (countOf p)

/-! ## Counters -/

/-- The bench's spelling of a counter declaration's name read as its kind, as a bare
`p1p1Counter` reads in `plugins_v2`. Any other kind is written `.named "<declaration name>"`. -/
semantic_macro p1p1Counter : CounterKind := .named "p1p1Counter"
semantic_macro flyingCounter : CounterKind := .named "flyingCounter"
semantic_macro m1m1Counter : CounterKind := .named "m1m1Counter"
/-- What one +X/+Y counter confers [CR#122.1a]: "+X/+Y" to its bearer [CR#613.4c]. A counter
definition's conferral is per counter (`Definition.counter`), so the deltas read no count. -/
semantic_macro boost (power toughness : Delta Amount) : Conferral :=
  .property (.ptModification .this power toughness)
/-- What one keyword counter confers [CR#122.1b]: its bearer gains the keyword [CR#613.1f], named
by its declaration's name (`grants (.named "flying")`). -/
semantic_macro grants (keyword : KeywordLabel) : Conferral :=
  .property (.abilityGrant .this (.keyword keyword [] []))

/-! ## Instructions -/

semantic_macro move (subject : NounPhrase) (destination : ZoneExpr) : Instruction :=
  .move subject .wherever destination []
semantic_macro destroy (subject : NounPhrase) : Instruction :=
  .enact (.action "destroy") (.move subject (.zone .battlefield .bare) graveyard [])
semantic_macro exile (subject : NounPhrase) : Instruction :=
  .enact (.action "exile") (.move subject .wherever exileZone [])
/-- "exile <subject> with N <kind> counters on it" -/
semantic_macro exileWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    Instruction :=
  .enact (.action "exile") (.move subject .wherever exileZone [.withCounters amount (.printed kind) .fresh])
semantic_macro sacrifice (subject : NounPhrase) : Instruction :=
  .enact (.action "sacrifice") (.move subject (.zone .battlefield .bare) graveyard [])
/-- "put <subject> <destination>" -/
semantic_macro put (subject : NounPhrase) (destination : ZoneExpr) : Instruction :=
  .enact (.core .put) (.move subject .wherever destination [])
/-- "put <subject> <destination>" with the move's riders ("second from the top", "on the top or
bottom") -/
semantic_macro putWith (subject : NounPhrase) (destination : ZoneExpr) (riders : List TokenRider) :
    Instruction :=
  .enact (.core .put) (.move subject .wherever destination riders)
/-- "return <subject> to <zone>" -/
semantic_macro returnTo (subject : NounPhrase) (destination : ZoneExpr) (riders : List TokenRider) :
    Instruction :=
  .enact (.core .return_) (.move subject .wherever destination riders)
/-- "return <subject> to the battlefield" -/
semantic_macro returnToBattlefield (subject : NounPhrase) : Instruction := returnTo subject battlefield []
/-- "return <subject> to the battlefield transformed under <controller>'s control" -/
semantic_macro returnToBattlefieldTransformed (subject controller : NounPhrase) : Instruction :=
  returnTo subject battlefield [.entersTransformed, .under controller]
/-- "return <subject> to the battlefield under <who>'s control with N <kind> counters on it" -/
semantic_macro returnToBattlefieldWithCounters (subject who : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Instruction :=
  .move subject .wherever battlefield [.under who, .withCounters amount (.printed kind) .fresh]
/-- "transform <subject>" -/
semantic_macro transform (subject : NounPhrase) : Instruction :=
  .enact (.action "transform") (.turnOver subject)
/-- "meld <subject> into <name>" -/
semantic_macro meldInto (subject : NounPhrase) (into : String) : Instruction :=
  .enact (.action "meld") (.move subject .wherever battlefield [.entersMelded into])
/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a].
This retains the existing fragment; face-down characteristics and the turn-up special
action are not represented here. -/
semantic_macro manifestPlacement (subject : NounPhrase) : Instruction :=
  .enact (.action "manifest") (.move subject .wherever battlefield [])
semantic_macro tap (subject : NounPhrase) : Instruction :=
  .enact (.action "tap") (.setStatus .tapped subject)
/-- "discard <subject>": a move from its owner's hand [CR#701.9a]. An indefinite subject implies
its own selection, by default the affected player's [CR#701.9b]. -/
semantic_macro discard (subject : NounPhrase) : Instruction :=
  .enact (.action "discard") (.move subject (.zone .hand .bare) graveyard [])
/-- A bare "shuffle": the performer's own library [CR#701.24a]. -/
semantic_macro shuffle : Instruction := .shuffle (library .actor)
semantic_macro untap (subject : NounPhrase) : Instruction :=
  .enact (.action "untap") (.setStatus .untapped subject)
/-- "Exile <subject> until <event>." -/
semantic_macro exileUntil (subject : NounPhrase) (event : GameEvent) : Instruction :=
  .holdUntil (exile subject) event
/-- "mill <amount> cards" from <whose> library. -/
semantic_macro mill (amount : Amount) (whose : NounPhrase) : Instruction :=
  .enact (.action "mill") (.move (.slice .top amount (library whose)) (.zone .library .bare) graveyard [])
semantic_macro putOntoBattlefield (subject : NounPhrase) : Instruction := .move subject .wherever battlefield []
semantic_macro putOntoBattlefieldTapped (subject : NounPhrase) : Instruction :=
  .move subject .wherever battlefield [.entersAs .tapped]
/-- "put <subject> onto the battlefield under your control" -/
semantic_macro putOntoBattlefieldUnderYourControl (subject : NounPhrase) : Instruction :=
  .move subject .wherever battlefield [.under .you]
/-- "put <subject> onto the battlefield tapped and attacking" -/
semantic_macro putOntoBattlefieldTappedAttacking (subject : NounPhrase) : Instruction :=
  .move subject .wherever battlefield [.entersAs .tapped, .entersAttacking none]
/-- A search of one zone: the scope constructor, which has no generated primitive because
`SearchScope` is not a `semantic_expression` type. -/
semantic_macro oneZone (zone : ZoneExpr) : SearchScope := .oneZone zone
/-- "Search your library for <quantity> <p>" -/
semantic_macro searchLibraryFor (quantity : Quantity) (p : Predicate) : Instruction :=
  .search (.oneZone yourLibrary) quantity p
/-- "search <whose>'s graveyard, hand, and library for <q> <p>" -/
semantic_macro searchZonesOf (whose : NounPhrase) (quantity : Quantity) (p : Predicate) : Instruction :=
  .search (.someZones (some whose) [.graveyard, .hand, .library]) quantity p
/-- "search your library and/or graveyard for a <p>" -/
semantic_macro searchLibraryOrGraveyard (p : Predicate) : Instruction :=
  .search (.someZones (some .you) [.library, .graveyard]) (exactly 1) p
/-- "reveal their hand" -/
semantic_macro revealTheirHand : Instruction := .expose .reveal (.zone (handOf they))
/-- "<creature> fights <creature>" [CR#701.14a]: the rules instruct the first creature, so a card
hands the deed to it (`act <first> (fight <second>)`) and `actor` in the body is the fighting
permanent; the other creature is the parameter, captured once. Mirrors `fight.ron`: that a
creature no longer on the battlefield deals no fight damage [CR#701.14b] is the rules' to apply,
not a guard here. -/
semantic_macro fight (other : capture NounPhrase) : Instruction :=
  .enact (.action "fight") (.simultaneously [
    .dealDamage .actor (.statOf (.stat .power) .actor) other,
    .dealDamage other (.statOf (.stat .power) other) .actor])

/-- The replacement's application, rather than installation of a new shield
[CR#701.19b,701.19c]. -/
semantic_macro regenerationApplication (subject : capture NounPhrase) : Instruction :=
  .enact (.action "regenerate") <|
    .sequentially [
      .clearDamage subject,
      .act (.possessorOf .controller subject) (.enact (.action "tap") (.setStatus .tapped subject)),
      .doIf (.or [.matches subject (.inCombat .attackerOf none),
                  .matches subject (.inCombat .blockerOf none)])
        (.combat subject (.participation .outsideCombat)) none]

/-- A resolving regeneration instruction installs a single-use shield for this turn;
only its eventual application carries the Regenerate label [CR#701.19a,701.19c]. -/
semantic_macro regenerate (subject : capture NounPhrase) : Instruction :=
  .establish (.replacement
    (.verbedEvent none (.action "destroy") (some subject) none none)
    [] none (regenerationApplication subject) .nextTimeOnly none) (some thisTurn)

/-- Losing counters is removal from the named player. Capturing that player first keeps
amount references in the order the sentence introduces them. -/
semantic_macro loseCounters (kind : Option CounterKindSource) (amount : Option Amount)
    (agent : capture NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .removeCounters (amount.map Quantity.exactlyOf) kind agent
semantic_macro loseLife (amount : Amount) : Instruction :=
  .changeLife (.down amount)
semantic_macro gainLife (amount : Amount) : Instruction :=
  .changeLife (.up amount)
/-- "<player>'s life total becomes <amount>" -/
semantic_macro setLife (amount : Amount) : Instruction :=
  .changeLife (.set amount)
semantic_macro draw (amount : Amount) : Instruction := .draw amount

semantic_macro lookAt (cards : NounPhrase) : Instruction := .expose .lookAt (.cards cards)
/-- "look at <player>'s hand" -/
semantic_macro lookAtHandOf (player : NounPhrase) : Instruction :=
  .expose .lookAt (.zone (handOf player))
semantic_macro revealCards (cards : NounPhrase) : Instruction := .expose .reveal (.cards cards)
/-- "shuffle <subject> into its owner's library": the move, then the shuffle of that library,
which happens even if nothing moved [CR#701.24c,701.24d]. -/
semantic_macro shuffleInto (subject : NounPhrase) : Instruction :=
  .sequentially
    [ .enact (.action "shuffle") (.move subject .wherever (.zone .library .bare) []),
      .shuffle (.library .bare) ]
semantic_macro doIf (condition : Condition) (instruction : Instruction) : Instruction :=
  .doIf condition instruction none
/-- "choose <subject>" -/
semantic_macro choose (subject : NounPhrase) (disclosure : Disclosure := .openly) : Instruction :=
  .choose none subject disclosure none
/-- "choose <subject> as you <event>" -/
semantic_macro chooseWhile (subject : NounPhrase) (while_ : Timing) : Instruction :=
  .choose none subject .openly (some while_)
semantic_macro rollDice (count : Nat) (sides : Nat) : Instruction :=
  .rollDice (.lit count) (.sides sides)
/-- One row of a results table: "<results> — <instruction>". -/
semantic_macro rollRow (results : Quantity) (instruction : Instruction) : RollRow := ⟨results, instruction⟩
semantic_macro flipCoins (count : Nat) : Instruction :=
  .flipCoins (.count (.lit count))
/-- "<player> flips a coin" as an event -/
semantic_macro flipsCoin (player : NounPhrase) : GameEvent := .flipsCoin player none
/-- "may <body>" -/
semantic_macro offer (body : Instruction) : Instruction := Primitives.Instruction.offer body none none
/-- "may <body>. When they do, <trigger>": a reflexive trigger on the choice. -/
semantic_macro offerWhen (body : Instruction) (trigger : Instruction) : Instruction :=
  .triggerReflexively (Primitives.Instruction.offer body none none) trigger
/-- "the chosen number" -/
semantic_macro chosenNumber : Amount := .chosenNumber .theChoice
/-- "Choose one or more — [cost] — <mode>; …" [CR#702.172a] -/
semantic_macro chooseSpree (modes : List (Option Cost × Instruction)) : Instruction := .chooseModes (atLeast 1)
    modes
/-- "vote for <ballot>" -/
semantic_macro vote (disclosure : Disclosure) (ballot : Ballot) : Instruction :=
  .vote none disclosure ballot
/-- "Starting with <first>, vote for <ballot>" -/
semantic_macro voteStartingWith (first : NounPhrase) (disclosure : Disclosure) (ballot : Ballot) :
    Instruction :=
  .vote (some first) disclosure ballot
/-- "Choose N — <modes>", no mode costing anything. -/
semantic_macro chooseModes (quantity : Quantity) (modes : List Instruction) : Instruction :=
  .chooseModes quantity (modes.map (none, ·))
/-- "<source> deals damage equal to its power to <recipient>" -/
semantic_macro dealDamageOwnPower (source : NounPhrase) (recipient : NounPhrase) :
    Instruction :=
  let own := if source.introducesOwnReferent then
      .pro .bare source.plur (.top 1) else source
  .dealDamage source (.statOf (.stat .power) own) recipient

/-- A token's characteristics from the parts a creature token names. -/
semantic_macro creatureTokenOf (power toughness : Amount) (colors : List Color) (subtypes : List Subtype) :
    CharacteristicBundle :=
  { characteristics :=
      { colors, types := [.creature], subtypes, power := some power,
        toughness := some toughness } }
semantic_macro creatureToken (power toughness : Nat) (colors : List Color) (subtypes : List Subtype) :
    CharacteristicBundle :=
  creatureTokenOf (.lit power) (.lit toughness) colors subtypes
/-- "create N <token>" -/
semantic_macro create (count : Amount) (token : CharacteristicBundle) : Instruction :=
  Primitives.Instruction.create count (.written token) []
/-- "create N <token> tapped and attacking" -/
semantic_macro createTappedAttacking (count : Amount) (token : CharacteristicBundle) : Instruction :=
  Primitives.Instruction.create count (.written token) [.entersAs .tapped, .entersAttacking none]

/-- "for each color of mana spent to cast <n>" -/
semantic_macro colorsSpentToCast (spell : NounPhrase) : Amount := .paid .colorsSpent spell
/-- "if colored mana was spent to cast <spell>" -/
semantic_macro coloredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .atLeast (.lit 1)
/-- "if no colored mana was spent to cast <spell>" -/
semantic_macro noColoredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .eq (.lit 0)
/-- "the amount of mana spent to cast <spell>" -/
semantic_macro manaValueSpentToCast (spell : NounPhrase) : Amount := .paid .manaValueSpent spell
/-- "if no mana was spent to cast <spell>" -/
semantic_macro noManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (manaValueSpentToCast spell) .eq (.lit 0)

/-- "between N and M" -/
semantic_macro fromTo (low high : Nat) : Quantity := .range (some low) (some high)
/-- The participant described by the nearest enclosing lookback. -/
semantic_macro relative (kind : Kind) : NounPhrase := .gap kind
/-- The number of occurrences of a historical event involving the subject. -/
semantic_macro eventCount (event : GameEvent) (who : NounPhrase) (within : Duration)
    (timing : Option Timing := none) : Amount :=
  .eventTally .count who (.mk event within timing)
/-- A historical event involving the subject. -/
semantic_macro happened (event : GameEvent) (who : NounPhrase) (within : Duration)
    (timing : Option Timing := none) : Condition :=
  .happened who (.mk event within timing)
/-- The event the enclosing instruction caused, involving the subject: "if you searched your
library this way". -/
semantic_macro happenedThisWay (event : GameEvent) (who : NounPhrase) : Condition :=
  .happened who (.thisWay event)
/-- A historical event whose gap stands for the entity being described. -/
semantic_macro happenedTo (event : GameEvent) (within : Duration) (timing : Option Timing := none) :
    Predicate :=
  .happenedTo (.mk event within timing)
/-- The accumulated magnitude of historical events involving the subject. -/
semantic_macro eventSum (event : GameEvent) (who : NounPhrase) (within : Duration)
    (timing : Option Timing := none) : Amount :=
  .eventTally .sum who (.mk event within timing)
/-- "if there is no <designation>" -/
semantic_macro thereIsNo (designation : DesignationLabel) : Condition := .noHolder designation
/-- "<subject>'s <keyword> cost was paid", read back. -/
semantic_macro paidCostRead (which : PaidCostName) (window : Option Duration) (subject : NounPhrase) :
    Amount :=
  .paid (.readback which window) subject
/-- "the number of times <subject>'s <keyword> cost was paid" -/
semantic_macro timesPaid (which : PaidCostName) (subject : NounPhrase) : Amount :=
  .paid (.timesPaid which) subject
/-- "the player chosen as <subject>'s <label> cost was paid", read back by the linked ability
[CR#607.2d,702.174a] -/
semantic_macro chosenAsPaid (label : KeywordLabel) (subject : NounPhrase) : NounPhrase :=
  .paid (.chosenPlayer (.byKeyword label)) subject
/-- "if <subject>'s <cost> was paid" -/
semantic_macro costWasPaid (which : PaidCostName) (window : Option Duration) (subject : NounPhrase) :
    Condition :=
  .compareAmt (paidCostRead which window subject) .atLeast (.lit 1)
/-- "the last chosen color" -/
semantic_macro theLastChosenColor : ColorTerm := .chosen .theLatestChoice
/-- "the last chosen number" -/
semantic_macro theLastChosenNumber : Amount := .chosenNumber .theLatestChoice
/-- "the amount by which the ceiling was not reached" -/
semantic_macro shortOfCeiling : Amount := .theOutcome .ceilingShortfall
/-- "the number of counters removed this way" -/
semantic_macro removedThisWay : Amount := .theOutcome .countersRemoved
/-- "increase or decrease the result by N" -/
semantic_macro shiftResult (amount : Amount) : Instruction := .shiftResult none amount
/-- "<player> may play N additional lands" -/
semantic_macro mayPlayAdditionalLands (player : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .deonticRule player .permit [.action "play"] .agent (some (.additional quantity))
    (.counterpart (allOf land))
    none .noRider
/-- "<player> may <deed> <what> [as though …] [rider]" -/
semantic_macro mayPlayDeed (deed : Deed) (player what : NounPhrase) (asThough : Option AsThough)
    (rider : DeonticRider) : StaticSpec :=
  .deonticRule player .permit [deed] .agent none (.counterpart what) asThough rider
/-- A deontic clause with no bound, premise, or rider. -/
semantic_macro deontic (subject : NounPhrase) (compulsion : Compulsion) (deeds : Deeds) (role : Role)
    (patient : DeonticPatient) : StaticSpec :=
  .deonticRule subject compulsion deeds role none patient none .noRider
/-- "When <event>, <instruction>" as a delayed trigger. -/
semantic_macro delay (event : GameEvent) (instruction : Instruction) : Instruction :=
  .delay event [] none instruction
/-- "When <event> <duration>, <instruction>": a delayed trigger with a window. -/
semantic_macro delayWithin (event : GameEvent) (duration : Duration) (instruction : Instruction) :
    Instruction :=
  .delay event [] (some duration) instruction
/-- "<subject> phases out until <event>" -/
semantic_macro phaseOutUntil (subject : NounPhrase) (event : GameEvent) : Instruction :=
  .holdUntil (.setStatus .phasedOut subject) event
/-- "attach <what> to it": the object the sentence just named. -/
semantic_macro attachToIt (what : NounPhrase) : Instruction :=
  Primitives.Instruction.attachTo what (.pro .bare .one (.outsideIntroduced ((NounPhrase.introduced [] what).map Binding.kind)))
/-- "take <count> extra turns after this one": the performer takes them [CR#500.7]. -/
semantic_macro extraTurn (count : Amount) : Instruction := .addTurnPart .turn none count none
/-- "there is an additional <part> [after <anchor>] [, followed by an additional <next>]": phases
added to the current turn [CR#500.8]. -/
semantic_macro additionalPhase (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
    (followedBy : Option TurnPart := none) : Instruction :=
  .addTurnPart part anchor count followedBy
/-- "there is an additional <part> [after <anchor>]": steps added to the current turn's phase
[CR#500.9]. -/
semantic_macro additionalStep (part : TurnPart) (anchor : Option TurnPart) (count : Amount) :
    Instruction :=
  .addTurnPart part anchor count none
/-- "<subject> can't attack [this turn]" -/
semantic_macro forbidAttack (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .establish (.deonticRule subject .forbid [.core .attack] .agent none .noPatient none .noRider)
    duration
/-- "<subject> can't block [this turn]" -/
semantic_macro forbidBlock (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .establish (.deonticRule subject .forbid [.core .block] .agent none .noPatient none .noRider)
    duration
/-- "<subject> can't be blocked [this turn]" -/
semantic_macro forbidBeingBlocked (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .establish (.deonticRule subject .forbid [.core .block] .patient none .noPatient none .noRider)
    duration
/-- "<subject> blocks it this turn if able": the object the sentence just named. -/
semantic_macro requireBlockIt (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .establish
    (.deonticRule subject .require [.core .block] .agent none
      (.counterpart (.pro .bare .one (.outsideIntroduced ((NounPhrase.introduced [] subject).map Binding.kind)))) none
      .noRider)
    duration

/-- "<source> deals N damage divided as you choose among <among>" -/
semantic_macro dealDivided (source : NounPhrase) (amount : Amount) (among : NounPhrase) : Instruction :=
  .distribute (.damage source) amount among
/-- "distribute N <kind> counters among <among>" -/
semantic_macro distributeCounters (amount : Amount) (kind : CounterKind) (among : NounPhrase) : Instruction :=
  .distribute (.counters kind) amount among
/-- "remove <q> <kind> counters from <from>" -/
semantic_macro removeCounters (quantity : Quantity) (kind : Option CounterKindSource) (from_ : NounPhrase) :
    Instruction :=
  .removeCounters (some quantity) kind from_
/-- "<who> loses all [<kind>] counters" -/
semantic_macro loseAllCounters (kind : Option CounterKindSource) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  loseCounters kind none (agent := agent)
/-- "remove all [<kind>] counters from <from>" -/
semantic_macro removeAllCounters (kind : Option CounterKindSource) (from_ : NounPhrase) : Instruction :=
  .removeCounters none kind from_

/-- The agent's number as an agent: "each …" acts one at a time (Idris `agentPlur`). -/
semantic_macro agentPlur : NounPhrase → Plurality
  | .described .each _ => .one
  | n => n.plur
/-- The agent re-read after its own clause (Idris `agentRef`): an agent that introduces no
binding is re-spelled as itself ("you", "that player"); one that does is read back as the
player it just bound, windowed over the agent's own bindings. -/
semantic_macro agentRef (agent : NounPhrase) : NounPhrase :=
  match NounPhrase.agentIntroduced [] agent with
  | [] => agent
  | ds => .pro (.word .player) (agentPlur agent) (.introduced (ds.map Binding.kind))

/-- "may pay <cost>. If they don't, <instruction>." -/
semantic_macro doUnless (instruction : Instruction) (cost : Cost) : Instruction :=
  Primitives.Instruction.offer (.pay cost .once) none (some instruction)

/-- The window over exactly what a phrase introduced; a phrase that introduced nothing (a
pronoun) is read again through the whole stack. -/
semantic_macro sameWindow : Bindings → Window
  | [] => .whole
  | bs => .introduced (bs.map Binding.kind)
/-- "it" (or "them"): the subject of a stat change, read back through a window holding only
what that subject and the change's amount announced. -/
semantic_macro itsOther (subject : NounPhrase) (delta : Delta Amount) : NounPhrase :=
  .pro .bare subject.plur
    (sameWindow
      (Delta.introduced (selfSubjIntro [] subject) delta ++
        NounPhrase.selfSubjIntroduced subject ++ NounPhrase.introduced [] subject))
/-- "<subject> gets +P/+T [until …]": one change to both stats, the subject written once. -/
semantic_macro getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec :=
  .ptModification subject power toughness
/-- "<subject> has base power and toughness P/T" -/
semantic_macro getsBase (subject : NounPhrase) (power toughness : Amount) : StaticSpec :=
  getsPt subject (.set power) (.set toughness)

/-- The shared subject of a clause, read back as a pronoun that sees only what the subject
itself announced. -/
semantic_macro ownSubject (subject : NounPhrase) : NounPhrase :=
  .pro .bare subject.plur (.introduced ((selfSubjIntro [] subject).map Binding.kind))
/-- "<subject>'s controller sacrifices it" [CR#701.21a]. The relational subject
introduces the object once; the deed re-reads that introduction. An already-referential
subject introduces no new object and can be read directly again. -/
semantic_macro controllerSacrifices (subject : NounPhrase) : Instruction :=
  let controller := controllerOf subject
  let patient :=
    if subject.introducesOwnReferent then
      .pro .bare subject.plur (.introduced [.player, subject.kindOr .object])
    else if (selfSubjIntro [] subject).isEmpty then subject
    else ownSubject controller
  .act controller (sacrifice patient)

/-- "them" (or "it"): the cards a look at a library slice just announced, seen alone. -/
semantic_macro lookedCards (slice : NounPhrase) : NounPhrase :=
  .pro .bare slice.plur (.introduced ((NounPhrase.introduced [] slice).map Binding.kind))
/-- "look at the top N cards of <whose> library, put any number of them on the bottom in any order
and the rest on top in any order" -/
semantic_macro lookAndSort (whose : NounPhrase) (amount : Amount) : Instruction :=
  let slice : NounPhrase := .slice .top amount (library whose)
  .sequentially
    [ .expose .lookAt (.cards slice),
      .move (someOf anyNumber (lookedCards slice)) .wherever onBottom [.inOrder .anyOrder],
      .move (theRest .object) .wherever onTop [.inOrder .anyOrder] ]
/-- The same look, spilling the cards put aside into <spill> instead of the bottom. -/
semantic_macro lookAndSortInto (whose : NounPhrase) (amount : Amount) (spill : ZoneExpr) :
    Instruction :=
  let slice : NounPhrase := .slice .top amount (library whose)
  .sequentially
    [ .expose .lookAt (.cards slice),
      move (someOf anyNumber (lookedCards slice)) spill,
      .move (theRest .object) .wherever onTop [.inOrder .anyOrder] ]
/-- "scry N" [CR#701.22a]: the performer's own library. -/
semantic_macro scry (amount : Amount) : Instruction :=
  .enact (.action "scry") (lookAndSort .actor amount)
/-- "fateseal N" [CR#701.29a] -/
semantic_macro fateseal (whose : NounPhrase) (amount : Amount) : Instruction :=
  .enact (.action "fateseal") (lookAndSort whose amount)
/-- "surveil N" [CR#701.25a]: the performer's own library. -/
semantic_macro surveil (amount : Amount) : Instruction :=
  .enact (.action "surveil") (lookAndSortInto .actor amount graveyard)
/-- "Proliferate" with its reminder text [CR#701.34a]: "Choose any number of permanents and/or
players, then give each another counter of each kind already there." -/
semantic_macro proliferate : Instruction :=
  .enact (.action "proliferate") (.sequentially
      [ .choose none (counted anyNumber
            (.or [ .and [permanent, .hasCounters none],
                   .compare [.anyCounter .player] .atLeast (.lit 1) ])) .openly none,
        .putCounters (.lit 1) .own (.eachOf (those .join)) ])
/-- "amass <subtype> N" with its reminder text [CR#701.47a]: "If you don't control an Army,
create a 0/0 black <subtype> Army creature token. Choose an Army you control. Put N +1/+1
counters on it. It's a <subtype> in addition to its other types." -/
semantic_macro amass (subtype : String) (count : Nat) : Instruction :=
  .sequentially
    [ .doIf (.not (exists_ armyYouControl))
        (create (.lit 1)
          (creatureToken 0 0 [.black] [creatureType subtype, creatureType "Army"]))
        none,
      choose (a armyYouControl),
      .putCounters (.lit count) (.printed p1p1Counter) (that (.type .creature)),
      .doIf (.not (.matches it (.hasSubtype (creatureType subtype))))
        (.establish
          (Primitives.StaticSpec.qualityChange it .adds
            (.bundle { characteristics := { subtypes := [creatureType subtype] } } none))
          none)
        none ]
/-- "monstrosity N" with its reminder text [CR#701.37a]: "If this permanent isn't monstrous,
put N +1/+1 counters on it and it becomes monstrous." -/
semantic_macro makeMonstrous (amount : Amount) : Instruction :=
  .doIf (.not (.matches thisPermanent (.hasDesignation (.named "monstrous") none)))
    (.sequentially
      [ .putCounters amount (.printed p1p1Counter) thisPermanent,
        .gainDesignation thisPermanent (.named "monstrous") none ])
    none
semantic_macro get (subject : NounPhrase) (power toughness : Delta Amount) (duration : Option Duration) :
    Instruction :=
  .establish (getsPt subject power toughness) duration

semantic_macro gain (subject : NounPhrase) (ability : Ability) (duration : Option Duration) : Instruction :=
  .establish (.abilityGrant subject ability) duration
/-- "<subject> gains haste [until …]" -/
semantic_macro gainHaste (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  gain subject (.keyword (.named "haste") [] []) duration
/-- "<subject> becomes <added> in addition to its other types [until …]" -/
semantic_macro become (subject : NounPhrase) (added : CharacteristicBundle) (duration : Option Duration) :
    Instruction :=
  .establish (Primitives.StaticSpec.qualityChange subject .adds (.bundle added none)) duration
/-- "<subject> becomes <colors> [until …]" -/
semantic_macro becomeColor (subject : NounPhrase) (colors : ColorSpec) (duration : Option Duration) :
    Instruction :=
  .establish (Primitives.StaticSpec.qualityChange subject .sets (.colored colors)) duration
/-- Several static clauses sharing one subject, as one instruction. -/
semantic_macro establishFor (subject : NounPhrase) (parts : List StaticSpec) (duration : Option Duration) :
    Instruction :=
  .establish (.conjunction (some subject) parts) duration
/-- "If <event> would happen, <replacement> instead [duration]." -/
semantic_macro replaceEvent (event : GameEvent) (replacement : Instruction) (duration : Option Duration) :
    Instruction :=
  .establish (.replacement event [] none replacement .repeatedly none) duration
/-- "Prevent all <kind> damage that would be dealt <scope> [duration]." -/
semantic_macro preventAll (kind : DamageKind) (scope : DamageScope) (duration : Option Duration) :
    Instruction :=
  .establish (.damageRule kind .unattributed scope (.prevent .all none) .repeatedly) duration
/-- "Prevent the next N <kind> damage that would be dealt <scope> [duration]." -/
semantic_macro preventNext (kind : DamageKind) (scope : DamageScope) (amount : Amount)
    (duration : Option Duration) : Instruction :=
  .establish (.damageRule kind .unattributed scope (.prevent (.shield amount) none) .repeatedly)
    duration
/-- "Prevent all <kind> damage that would be dealt by <source> <scope> [duration]." -/
semantic_macro preventAllBy (kind : DamageKind) (source : NounPhrase) (scope : DamageScope)
    (duration : Option Duration) : Instruction :=
  .establish (.damageRule kind (.dealtBy source) scope (.prevent .all none) .repeatedly) duration
/-- "The next time <event> would happen, <replacement> instead [duration]." -/
semantic_macro replaceNextEvent (event : GameEvent) (replacement : Instruction)
    (duration : Option Duration) : Instruction :=
  .establish (.replacement event [] none replacement .nextTimeOnly none) duration
/-- "<player> gains control of <subject> [duration]" -/
semantic_macro gainControl (subject : NounPhrase) (duration : Option Duration) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  .establish (.controlGrant agent subject) duration
/-- "<subject> can't be <deed>ed" -/
semantic_macro objectCant (deed : Deed) (subject : NounPhrase) : StaticSpec :=
  .deonticRule subject .forbid [deed] .patient none .noPatient none .noRider
/-- "<player> can't <deed>" -/
semantic_macro playerCant (deed : Deed) (player : NounPhrase) : StaticSpec :=
  .deonticRule player .forbid [deed] .agent none .noPatient none .noRider
/-- "<spec> as long as <condition>" -/
semantic_macro onlyWhile (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .asLongAs
/-- "<spec> unless <condition>" -/
semantic_macro onlyUnless (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec (.not condition) .unless_
/-- "<spec> if <condition>" -/
semantic_macro onlyIfSo (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .ifSo
/-- "<who> can't <deed> <what>" -/
semantic_macro cantDoTo (deed : Deed) (who what : NounPhrase) : StaticSpec :=
  .deonticRule who .forbid [deed] .agent none (.counterpart what) none .noRider
/-- "<n> can <deed> as though it didn't have <p>" -/
semantic_macro canDoAsThough (n : NounPhrase) (deed : Deed) (p : Predicate) : StaticSpec :=
  .deonticRule n .permit [deed] .agent none .noPatient (some (.of p)) .noRider
/-- "<what> can't be the target of <by>" -/
semantic_macro cantBeTargetedBy (what by_ : NounPhrase) : StaticSpec :=
  .deonticRule what .forbid [.core .target] .patient none (.targetedBy by_) none .noRider
/-- "<what> can be the target of <by> as though it didn't have <p>" -/
semantic_macro canBeTargetedAsThough (what by_ : NounPhrase) (p : Predicate) : StaticSpec :=
  .deonticRule what .permit [.core .target] .patient none (.targetedBy by_) (some (.of p)) .noRider
/-- "<spec> <duration>": a clause holding for a stated duration. -/
semantic_macro establishThroughout (spec : StaticSpec) (duration : Duration) : Instruction :=
  .establish spec (some duration)
/-- "<n> doesn't untap during [<whose>] untap step" -/
semantic_macro doesntUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose (objectCant (.action "untap") subject)
/-- "you may choose not to untap <subject> during [<whose>] untap step" -/
semantic_macro mayDeclineUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.deonticRule subject .permit [.action "untap"] .patient none .noPatient none .noRider)
/-- "untap <subject> during [<whose>] untap step" -/
semantic_macro untapsDuring (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.deonticRule subject .require [.action "untap"] .patient none .noPatient none .noRider)
/-- "<subject> can block an additional creature each combat" -/
semantic_macro mayBlockAdditional (subject : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .deonticRule subject .permit [.core .block] .agent (some (.additional quantity))
    (.counterpart (allOf creature)) none .noRider
/-- "<who> may vote an additional time" -/
semantic_macro mayVoteAdditional (who : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .deonticRule who .permit [.action "vote"] .agent (some (.additional quantity)) .noPatient none
    .noRider
/-- "<who> may spend mana as though it were mana of <as> [to <purpose>]" -/
semantic_macro maySpendAsThough (who : NounPhrase) (what : Option ColorOrColorless) (as_ : ManaMatch)
    (purpose : Option SpendPurpose) : StaticSpec :=
  .deonticRule who .permit [.core .spend] .agent none .noPatient (some (.mana what as_ purpose))
    .noRider
/-- "<player> can't <deed> more than N <p>" -/
semantic_macro cantMoreThan (player : NounPhrase) (deed : Deed) (bound : Nat) (p : Predicate) :
    StaticSpec :=
  .deonticRule player .forbid [deed] .agent (some (.moreThan (.lit bound))) (.counterpart (allOf p))
    none .noRider

/-! ## The performer and the handoff

An instruction's performer is not written: it is `actor`, the controller [CR#109.5] unless an
enclosing `act` hands the instruction to another player or a permanent. No instruction carries a
performer field: a choice's chooser and an added turn's taker are the actor too. The `Actor`
helpers below are the spellings the plain macros lack: the performer's own hand (`revealHand`,
`discard`), a permission, an Army and amass. -/

/-- "whoever performs this instruction" -/
semantic_macro actor : NounPhrase := .actor
/-- "<performer> <body>": the handoff, to a player or a permanent. "Target player discards a
card" is `act (target .anyPlayer) (Actor.discard (a .isCard))`. -/
semantic_macro act (performer : NounPhrase) (body : Instruction) : Instruction := .act performer body
/-- "… <performer> controls" -/
semantic_macro actorControls : Predicate := .hasPossessor .controller .actor

/-- "For each <pip> mana …, you may <action> rather than pay that mana": a Mana Substitute
[CR#702.51a,702.66a,702.126a,701.67a], spelled as the RON helper `substitute`. -/
semantic_macro substitute (action : Instruction) (pays : List PipPaid) : ManaSubstitute :=
  { action, pays }

namespace Actor

/-- "<performer> reveals their hand" -/
semantic_macro revealHand : Instruction := .expose .reveal (.zone (handOf .actor))
/-- "<performer> discards <subject>": a move from the performer's hand to the graveyard
[CR#701.9a]. -/
semantic_macro discard (subject : NounPhrase) : Instruction :=
  .enact (.action "discard") (.move subject (.zone .hand (.possessedBy .actor)) graveyard [])
/-- "<performer> may cast <what> from <zone>", paying its own cost. -/
semantic_macro mayCastFrom (what : NounPhrase) (zone : ZoneExpr) : StaticSpec :=
  mayPlayDeed (.action "cast") .actor what none (.play (some zone) none none false .itsOwnCost)
/-- An Army the performer controls. -/
semantic_macro army : Predicate := .and [.hasSubtype (creatureType "Army"), creature, actorControls]
/-- "<performer> amasses <subtype> N" [CR#701.47a]: "If you don't control an Army creature,
create a 0/0 black <subtype> Army creature token. Choose an Army creature you control. Put N
+1/+1 counters on that creature. If it isn't a <subtype>, it becomes a <subtype> in addition to
its other types.", with "you" the performer. "That creature" and "it" are the chosen Army, read
through the choice's own introduction (`itPrior`), so an object the enclosing text named
("Its controller amasses …") is not a second candidate. -/
semantic_macro amass (subtype : String) (count : Nat) : Instruction :=
  .sequentially
    [ .doIf (.not (exists_ army))
        (create (.lit 1) (creatureToken 0 0 [.black] [creatureType subtype, creatureType "Army"]))
        none,
      choose (a army),
      .putCounters (.lit count) (.printed p1p1Counter) (itPrior (choose (a army))),
      .doIf (.not (.matches (itPrior (choose (a army))) (.hasSubtype (creatureType subtype))))
        (.establish
          (Primitives.StaticSpec.qualityChange (itPrior (choose (a army))) .adds
            (.bundle { characteristics := { subtypes := [creatureType subtype] } } none))
          none)
        none ]

/-- "any number of <group> with total <axis> <comparator> <bound>" [CR#701.59a]. -/
semantic_macro withTotal (axis : ProjAxis) (comparator : Comparator) (bound : Amount)
    (group : NounPhrase) : NounPhrase :=
  .someOf (.totalling axis comparator bound) none group
/-- "<performer> collects evidence N" [CR#701.59a]: exile any number of cards from their
graveyard with total mana value N or greater. -/
semantic_macro collectEvidence (amount : Amount) : Instruction :=
  .enact (.action "collectEvidence")
    (move (withTotal (.stat .manaValue) .atLeast amount
      (allOf (.and [.isCard, .inZone (graveyardOf .actor)]))) exileZone)
/-- "<performer> casts <subject> without paying its mana cost [if the resulting spell is …]"
[CR#701.5a,601.2]. -/
semantic_macro castWithoutPaying (subject : NounPhrase) (resulting : Option Predicate) :
    Instruction :=
  .enact (.action "cast") (.procedure .cast subject .withoutPaying resulting)
/-- "<performer> discovers N" [CR#701.57a]: the run exiled from the top of their library, the
card that stopped it cast or put into their hand, the rest on the bottom at random. -/
semantic_macro discover (amount : Amount) : Instruction :=
  let fits : Predicate := .compare [.stat .manaValue] .atMost amount
  .enact (.action "discover") (.sequentially
    [ move (.run .top (.and [.not land, fits]) (library .actor)) exileZone,
      .withContinuation .optional (castWithoutPaying (that .card) (some fits)) none
        (some (move (that .card) hand)),
      .move (theRest .object) .wherever onBottom [.inOrder .randomOrder] ])
/-- "<performer> incubates N" [CR#701.53a,701.53b]: an Incubator token with N +1/+1 counters. -/
semantic_macro incubate (amount : Amount) : Instruction :=
  .enact (.action "incubate") (.createObject (.lit 1) (.token
    (.doubleFaced
      { characteristics :=
          { types := [.artifact], subtypes := [artifactType "Incubator"],
            text := [.activated (.mana [generic 2]) (transform .this) none none none none] } }
      { characteristics :=
          { name := some "Phyrexian Token", types := [.artifact, .creature],
            subtypes := [creatureType "Phyrexian"], power := some (.lit 0),
            toughness := some (.lit 0) } })
    [.withCounters amount (.printed p1p1Counter) .fresh]))
/-- "<performer> waterbends <cost>" [CR#701.67a]: tapping an untapped artifact or creature they
control rather than paying each generic mana in the cost, that cost's own generic mana only
[CR#701.67b]. -/
semantic_macro waterbend (cost : Cost) : Instruction :=
  .enact (.action "waterbend")
    (.pay (.withSubstitute cost
      (substitute (tap (a (.and [untapped, .or [artifact, creature], actorControls]))) [.generic]))
      .once)
/-- "<performer> manifests <subject>" [CR#701.40a]: onto the battlefield face down as a 2/2
creature with no text, name, subtypes or mana cost [CR#708.2]. -/
semantic_macro manifest (subject : NounPhrase) : Instruction :=
  .enact (.action "manifest") (.move subject .wherever battlefield
    [.faceDown { types := [.creature], power := some (.lit 2), toughness := some (.lit 2) }])
/-- "<performer> cloaks <subject>" [CR#701.58a]: as manifest, with ward {2}. -/
semantic_macro cloak (subject : NounPhrase) : Instruction :=
  .enact (.action "cloak") (.move subject .wherever battlefield
    [.faceDown { types := [.creature], text := [.keyword (.named "ward") [.cost (.mana [generic 2])] []],
                 power := some (.lit 2), toughness := some (.lit 2) }])
/-- "<performer> manifests dread" [CR#701.62a]: look at the top two cards of their library,
manifest one, put the other into their graveyard. -/
semantic_macro manifestDread : Instruction :=
  .enact (.action "manifestDread") (.sequentially
    [ .expose .lookAt (.cards (.slice .top (.lit 2) (library .actor))),
      manifest (someOf (exactly 1) them),
      move (theRest .object) graveyard ])
/-- "support N" [CR#701.41a]: a +1/+1 counter on each of up to N other target creatures. -/
semantic_macro support (amount : Amount) : Instruction :=
  .enact (.action "support") (.putCounters (.lit 1) (.printed p1p1Counter)
    (.eachOf (.described (.target (.upToOf amount)) (.and [creature, .otherThan .this]))))

end Actor

/-- "double <subject>'s power" [CR#701.10b,701.10c]: it gets +X/+0, X its power. -/
semantic_macro doublePower (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .establish (.ptModification subject (.up (powerOf subject)) (.up (.lit 0))) duration
/-- "triple <subject>'s power and toughness" [CR#701.11b,701.11c]. -/
semantic_macro triplePowerAndToughness (subject : NounPhrase) (duration : Option Duration) :
    Instruction :=
  .establish (.ptModification subject (.up (times (.lit 2) (powerOf subject)))
    (.up (times (.lit 2) (toughnessOf subject)))) duration
/-- "double <performer>'s life total" [CR#701.10d], written as that total's set [CR#119.5]. -/
semantic_macro doubleLifeTotal : Instruction :=
  .changeLife (.set (times (.lit 2) (lifeTotalOf .actor)))

/-! ## Events -/

semantic_macro leavesBattlefield (subject : NounPhrase) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (.zones [battlefield]))
/-- "<subject> leaves <zone>" -/
semantic_macro leavesZone (subject : NounPhrase) (zone : ZoneExpr) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (.zones [zone]))
/-- "at the beginning of <possessor>'s <part>", each time it comes -/
semantic_macro beginningOfPossessed (part : TurnPart) (possessor : NounPhrase) : GameEvent :=
  .beginningOf part (some possessor) (.each none)
/-- "at the beginning of your <part>", each time it comes -/
semantic_macro beginningOfYour (part : TurnPart) : GameEvent := .beginningOf part (some .you) (.each none)
/-- "at the beginning of each <part>" -/
semantic_macro beginningOfEach (part : TurnPart) : GameEvent := .beginningOf part none (.each none)
/-- "at the beginning of the next <part>": a delayed trigger's moment. -/
semantic_macro beginningOfNext (part : TurnPart) : GameEvent := .beginningOf part none (.next none)
/-- "at the beginning of that turn's <part>" -/
semantic_macro beginningOfThatTurns (part : TurnPart) : GameEvent := .beginningOf part none .that
semantic_macro dealsCombatDamage (source : NounPhrase) (patient : NounPhrase) : GameEvent :=
  Primitives.GameEvent.dealsDamage .combatOnly source (some patient)
semantic_macro attacks (subject : NounPhrase) : GameEvent := .combat .attackerOf subject none
/-- "<subject> attacks <whom>" -/
semantic_macro attacksPlayer (subject whom : NounPhrase) : GameEvent :=
  .combat .attackerOf subject (some whom)
/-- "<n> is put into <zone> from <source>" -/
semantic_macro putIntoFrom (subject : NounPhrase) (destination : ZoneExpr) (source : EventSource) : GameEvent :=
  Primitives.GameEvent.putInto subject destination (some source)
/-- "N <kind> counters are put on / removed from <subject>" -/
semantic_macro counterEvent (move : CounterMove) (kind : CounterKind) (batch : CounterBatch)
    (subject : NounPhrase) : GameEvent :=
  .counterEvent move (some kind) subject batch none false
/-- "counters are put on / removed from <subject>", the kind unsaid. -/
semantic_macro bareCounterEvent (move : CounterMove) (batch : CounterBatch) (subject : NounPhrase) :
    GameEvent :=
  .counterEvent move none subject batch none false
/-- "the last <kind> counter is removed from <subject> by <who>" -/
semantic_macro lastCounterRemovedBy (kind : CounterKind) (subject who : NounPhrase) : GameEvent :=
  .counterEvent .removed (some kind) subject .emptying (some who) false
/-- "one or more counters are put on <subject> by an effect" -/
semantic_macro manyCountersPutByEffect (subject : NounPhrase) : GameEvent :=
  .counterEvent .put none subject .many none true
/-- "<who> puts one or more counters on <subject>" -/
semantic_macro manyBareCountersPutBy (who subject : NounPhrase) : GameEvent :=
  .counterEvent .put none subject .many (some who) false
/-- "one or more tokens would be created" -/
semantic_macro tokensCreated (tokens : NounPhrase) : GameEvent := .tokensCreated tokens false none none
/-- "you roll a die and the result is <q>" -/
semantic_macro youRollResultIn (quantity : Quantity) : GameEvent :=
  .rollsDice .you .one none (.resultIn quantity)
/-- "you roll a die and the natural result is the highest" -/
semantic_macro youRollHighestNatural : GameEvent := .rollsDice .you .one none .highestNatural
/-- "one or more tokens would be created under <under>'s control by an effect" -/
semantic_macro tokensCreatedByEffectUnder (tokens under : NounPhrase) : GameEvent :=
  .tokensCreated tokens true none (some under)
semantic_macro blocks (subject : NounPhrase) (blocked : Option NounPhrase) : GameEvent :=
  .combat .blockerOf subject blocked
semantic_macro becomesBlocked (subject : NounPhrase) (by_ : Option NounPhrase) : GameEvent :=
  .combat .blockedBy subject by_
/-- "the last <kind> counter is removed from <subject>" -/
semantic_macro lastCounterRemoved (kind : CounterKind) (subject : NounPhrase) : GameEvent :=
  .counterEvent .removed (some kind) subject .emptying none false
/-- "<subject> regenerates": the verb as an event. -/
semantic_macro regenerates (subject : NounPhrase) : GameEvent :=
  .verbedEvent none (.action "regenerate") (some subject) none none

/-- Copy exceptions retain their own edit block; added abilities remain copy-specific. -/
semantic_macro copyCharacteristics (c : Characteristics) (typesAdded : Bool) : List CopyExcept :=
  let typeOp := if typesAdded then QualityOp.adds else .sets
  let types := if c.supertypes.isEmpty && c.types.isEmpty && c.subtypes.isEmpty then []
    else [.typeLine typeOp c.typeChanges]
  let values := ({ c with text := [] } : Characteristics).valueEdits .sets
  let edits := types ++ values
  (if edits.isEmpty then [] else [.edits edits]) ++ c.text.map CopyExcept.ability

/-! ## Abilities -/

semantic_macro keyword (label : KeywordLabel) : Ability := .keyword label [] []
/-- An ability word in italics before an ability: "Will of the council — …" [CR#207.2c]. -/
semantic_macro abilityWord (word : AbilityWordLabel) (ability : Ability) : Ability :=
  .italicHead (.abilityWord word) ability
/-- A flavor word in italics before an ability [CR#207.2d]. -/
semantic_macro flavorWord (word : FlavorWordLabel) (ability : Ability) : Ability :=
  .italicHead (.flavorWord word) ability
/-- "Companion — <condition>" -/
semantic_macro companion (condition : DeckCondition) : Ability :=
  .keyword (.named "companion") [.deckCondition condition] []
/-- "Pay N life" as a cost. -/
semantic_macro payLife (amount : Nat) : Cost :=
  .perform (.changeLife (.down (.lit amount)))
/-- "<keyword> <cost>" -/
semantic_macro keywordCosting (label : KeywordLabel) (cost : Cost) : Ability :=
  .keyword label [.cost cost] []
/-- "<keyword> <quality>", e.g. "protection from red" -/
semantic_macro keywordQuality (label : KeywordLabel) (quality : Predicate) : Ability :=
  .keyword label [.quality quality] []
/-- "<keyword> <subject>", e.g. "enchant creature" -/
semantic_macro keywordSubject (label : KeywordLabel) (subject : Predicate) : Ability :=
  .keyword label [.subject subject] []
/-- "<keyword> N", e.g. "bushido 2" -/
semantic_macro keywordNumber (label : KeywordLabel) (amount : Amount) : Ability :=
  .keyword label [.number amount] []
/-- "<keyword> <quality> <cost>", e.g. "plainscycling {2}" [CR#702.29e] -/
semantic_macro keywordQualityCosting (label : KeywordLabel) (quality : Predicate) (cost : Cost) : Ability :=
  .keyword label [.quality quality, .cost cost] []
/-- "<keyword> N—<cost>", e.g. "suspend 4—{1}{U}" -/
semantic_macro keywordNumberCosting (label : KeywordLabel) (amount : Amount) (cost : Cost) : Ability :=
  .keyword label [.number amount, .cost cost] []
/-- "Level up [cost]" [CR#702.87a] -/
semantic_macro levelUp (cost : Cost) : Ability := keywordCosting (.named "levelUp") cost
/-- "When <event>, if <condition>, <instruction>" -/
semantic_macro triggeredIf (event : GameEvent) (condition : Condition)
    (instruction : Instruction) : Ability :=
  .triggered event [] none [] none none (some condition) instruction
/-- "Whenever <event>, <instruction>. This ability triggers only once each turn." -/
semantic_macro triggeredOnlyOnce (event : GameEvent) (limit : UsageLimit) (instruction : Instruction) :
    Ability :=
  .triggered event [] none [] none (some limit) none instruction
/-- "As <subject> enters, choose a <quality>." -/
semantic_macro entersChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  Primitives.StaticSpec.entryChoice subject (.quality sort) none .openly
/-- "As <subject> enters, choose a <quality> from <domain>." -/
semantic_macro entersChoosingFrom (subject : NounPhrase) (sort : QualitySort) (domain : ChoiceDomain) :
    StaticSpec :=
  Primitives.StaticSpec.entryChoice subject (.quality sort) (some domain) .openly
/-- "As <subject> enters, choose a player [from <domain>]." -/
semantic_macro entersChoosingPlayer (subject : NounPhrase) (domain : Option ChoiceDomain) : StaticSpec :=
  Primitives.StaticSpec.entryChoice subject .player domain .openly
/-- "As <subject> enters, secretly choose a player [from <domain>]." -/
semantic_macro entersChoosingPlayerSecretly (subject : NounPhrase) (domain : Option ChoiceDomain) :
    StaticSpec :=
  Primitives.StaticSpec.entryChoice subject .player domain .secretly
/-- "<subject> enters tapped" -/
semantic_macro entersTapped (subject : NounPhrase) : StaticSpec := .entryRider subject (.entersAs .tapped)
/-- "As <subject> becomes attached, choose a <quality>." -/
semantic_macro attachChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  Primitives.StaticSpec.attachmentChoice subject (.quality sort) none
/-- "<subject> enters with N <kind> counters on it" -/
semantic_macro entersWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .fresh)
/-- "<subject> enters with an additional N <kind> counters on it" -/
semantic_macro entersWithAdditionalCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .additional)
/-- "<subject> enters with N fewer <kind> counters on it" -/
semantic_macro entersWithFewerCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .fewer)

semantic_macro triggered (event : GameEvent) (instruction : Instruction) : Ability :=
  .triggered event [] none [] none none none instruction
/-- "Whenever <event> and whenever <joined events>, <instruction>" -/
semantic_macro triggeredJoined (event : GameEvent) (joins : List JoinedHeader) (instruction : Instruction) :
    Ability :=
  .triggered event [] none joins none none none instruction
/-- A further "whenever <event>" header joined onto a trigger. -/
semantic_macro joinedHead (event : GameEvent) : JoinedHeader := ⟨event, [], none, none⟩
/-- "Whenever <event> or <alternatives>, <instruction>" -/
semantic_macro triggeredOr (event : GameEvent) (alternatives : List GameEvent) (instruction : Instruction) :
    Ability :=
  .triggered event alternatives none [] none none none instruction
/-- "Whenever <event> while <concurrent>, <instruction>" -/
semantic_macro triggeredWhile (event : GameEvent) (while_ : Timing) (instruction : Instruction) :
    Ability :=
  .triggered event [] (some while_) [] none none none instruction
/-- "Whenever <event> during <timing>, <instruction>" -/
semantic_macro triggeredOnlyDuring (event : GameEvent) (timing : Timing) (instruction : Instruction) :
    Ability :=
  .triggered event [] none [] (some timing) none none instruction
/-- A joined "whenever <event> while <concurrent>" header. -/
semantic_macro joinedHeadWhile (event : GameEvent) (while_ : Timing) : JoinedHeader :=
  ⟨event, [], some while_, none⟩
/-- "When <event>, <instruction>" -/
semantic_macro when (event : GameEvent) (instruction : Instruction) : Ability := triggered event instruction
/-- "Whenever <event>, <instruction>" -/
semantic_macro whenever (event : GameEvent) (instruction : Instruction) : Ability :=
  triggered event instruction
/-- "At <event>, <instruction>" (`at` is a Lean keyword). -/
semantic_macro at_ (event : GameEvent) (instruction : Instruction) : Ability := triggered event instruction
/-- "After <event>, <instruction>": the dice template's word. -/
semantic_macro after (event : GameEvent) (instruction : Instruction) : Ability := triggered event instruction

/-- Renown N's reminder text: "When this creature deals combat damage to a player, if it isn't
renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
semantic_macro renownExpansion (count : Nat) : Ability :=
  triggeredIf (dealsCombatDamage thisCreature (a .anyPlayer))
    (.not (.matches thisCreature (.hasDesignation (.named "renowned") none)))
    (.sequentially
      [ .putCounters (.lit count) (.printed p1p1Counter) thisCreature,
        .gainDesignation thisCreature (.named "renowned") none ])
/-- Storm's reminder text: "When you cast this spell, copy it for each other spell that was cast
before it this turn. You may choose new targets for the copies." [CR#702.40a] -/
semantic_macro stormExpansion : Ability :=
  when (.casts .you (some thisSpell) none)
    (.sequentially
      [ .copy .fromStack thisSpell (eventCount (.casts (relative .player) (some (a (.and [spell,
        .otherThan thisSpell]))) none) (a .anyPlayer) earlierThisTurn) [],
        offer (.chooseNewTargets (.pro (.word .copy) .many .whole)) ])
/-- "Storm" with its reminder text. -/
semantic_macro storm : Ability := .keyword (.named "storm") [] [stormExpansion]
/-- Cascade's reminder text [CR#702.85a]: "When you cast this spell, exile cards from the top of
your library until you exile a nonland card whose mana value is less than this spell's mana value.
You may cast that card without paying its mana cost if the resulting spell's mana value is less
than this spell's mana value. Then put all cards exiled this way that weren't cast on the bottom of
your library in a random order." The run is exiled from the library pile; "that card" reads the
card that stopped it, and the cards put back are the exiled ones that were not cast. -/
semantic_macro cascadeExpansion : Ability :=
  let cheaper : Predicate :=
    .compare [.stat .manaValue] .less (.statOf (.stat .manaValue) thisSpell)
  when (.casts .you (some thisSpell) none)
    (.sequentially
      [ exile (.run .top (.and [.not land, cheaper]) (library .you)),
        offer (Actor.castWithoutPaying (that .card) (some cheaper)),
        .move (allFromAmong (.not .wasCast) (theVerbed (.action "exile") .many)) .wherever onBottom
          [.inOrder .randomOrder] ])
/-- "Cascade" with its reminder text. -/
semantic_macro cascade : Ability := .keyword (.named "cascade") [] [cascadeExpansion]
/-- "Renown N" with its reminder text. -/
semantic_macro renown (count : Nat) : Ability :=
  .keyword (.named "renown") [.number (.lit count)] [renownExpansion count]
/-- Cumulative upkeep's reminder text: "At the beginning of your upkeep, if this permanent is
on the battlefield, put an age counter on it. Then you may pay [cost] for each age counter on
it. If you don't, sacrifice it." [CR#702.24a] -/
semantic_macro cumulativeUpkeepExpansion (cost : Cost) : Ability :=
  triggeredIf (.beginningOf .upkeep (some .you) (.each none))
    (.matches thisPermanent (.inZone battlefield))
    (.sequentially
      [ .putCounters (.lit 1) (.printed (.named "ageCounter")) thisPermanent,
        Primitives.Instruction.offer (.pay (.scaled cost (times (.lit 1) (countersOn (.named "ageCounter") thisPermanent))) .once)
          none (some (sacrifice thisPermanent)) ])
/-- "Cumulative upkeep [cost]" with its reminder text. -/
semantic_macro cumulativeUpkeep (cost : Cost) : Ability :=
  .keyword (.named "cumulativeUpkeep") [.cost cost] [cumulativeUpkeepExpansion cost]
/-- Flying's definition, a static ability: "This creature can't be blocked except by creatures
with flying and/or reach." [CR#702.9b] -/
semantic_macro flyingExpansion : Ability :=
  .static (deontic thisCreature .forbid [.core .block] .patient
    (.counterpart (allOf (.and [creature,
      .not (.or [.hasKeyword (.the (.named "flying")), .hasKeyword (.the (.named "reach"))])]))))
/-- "Flying" with its definition. -/
semantic_macro flying : Ability := .keyword (.named "flying") [] [flyingExpansion]
/-- Bushido N's definition, a triggered ability: "Whenever this creature blocks or becomes
blocked, it gets +N/+N until end of turn." [CR#702.45a] -/
semantic_macro bushidoExpansion (count : Nat) : Ability :=
  triggeredOr (blocks thisCreature none) [becomesBlocked thisCreature none]
    (get thisCreature (.up (.lit count)) (.up (.lit count)) (some untilEndOfTurn))
/-- "Bushido N" with its definition. -/
semantic_macro bushido (count : Nat) : Ability :=
  .keyword (.named "bushido") [.number (.lit count)] [bushidoExpansion count]
/-- Modular N's definition, a static ability and a triggered one [CR#702.43a]: "This permanent
enters with N +1/+1 counters on it" and "When this permanent is put into a graveyard from the
battlefield, you may put a +1/+1 counter on target artifact creature for each +1/+1 counter on
this permanent." -/
semantic_macro modularExpansion (count : Nat) : List Ability :=
  [ .static (entersWithCounters thisPermanent (.lit count) p1p1Counter),
    when (Primitives.GameEvent.dies thisPermanent)
      (offer (.putCounters (countersOn p1p1Counter thisPermanent) (.printed p1p1Counter)
        (target (.and [artifact, creature])))) ]
/-- "Modular N" with its definition. -/
semantic_macro modular (count : Nat) : Ability :=
  .keyword (.named "modular") [.number (.lit count)] (modularExpansion count)
semantic_macro activated (cost : Cost) (instruction : Instruction) : Ability :=
  .activated cost instruction none none none none
/-- "[cost]: <instruction>. Activate only <timing>." -/
semantic_macro activatedOnlyDuring (cost : Cost) (instruction : Instruction) (timing : Timing) : Ability :=
  .activated cost instruction (some timing) none none none
/-- "[cost]: <instruction>. Only <who> may activate this ability." -/
semantic_macro activatedBy (cost : Cost) (instruction : Instruction) (who : NounPhrase) : Ability :=
  .activated cost instruction none none none (some who)
/-- "[cost]: <instruction>. Activate only once <limit>." -/
semantic_macro activatedOnlyOnce (cost : Cost) (instruction : Instruction) (limit : UsageLimit) : Ability :=
  .activated cost instruction none (some limit) none none
/-- "[cost]: <instruction>. Activate only if <guard>." -/
semantic_macro activatedOnlyIf (cost : Cost) (instruction : Instruction) (guard : Condition) : Ability :=
  .activated cost instruction none none (some guard) none
/-- "[cost]: <instruction>. Activate only once <limit> and only if <guard>." -/
semantic_macro activatedOnlyOnceIf (cost : Cost) (instruction : Instruction) (limit : UsageLimit)
    (guard : Condition) : Ability :=
  .activated cost instruction none (some limit) (some guard) none

/-- Cycling [cost]'s definition, an activated ability: "[Cost], Discard this card: Draw a card."
[CR#702.29a] -/
semantic_macro cyclingExpansion (cost : Cost) : Ability :=
  activated (.compound [cost, .perform (discard .this)]) (draw (.lit 1))
/-- Flash's definition: "You may play this card any time you could cast an instant."
[CR#702.8a] -/
semantic_macro flashExpansion : Ability :=
  .static (mayPlayDeed (.action "play") .you .this none
    (.play none none (some asInstant) false .itsOwnCost))
/-- Sneak [cost]'s definition: "Any time you could cast an instant during your declare blockers
step, you may cast this spell by paying [cost] and returning an unblocked creature you control to
its owner's hand rather than paying this spell's mana cost." [CR#702.190a] -/
semantic_macro sneakExpansion (cost : Cost) : Ability :=
  .static (mayPlayDeed (.action "cast") .you .this none
    (.play none none (some (.and [asInstant, duringPart .declareBlockers (some .you)])) false
      (.payingInstead (.compound [cost,
        .perform (returnTo (a (.and [creature, unblocked, .hasPossessor .controller .you])) hand [])]))))
/-- "Cycling [cost]" with its definition. -/
semantic_macro cycling (cost : Cost) : Ability :=
  .keyword (.named "cycling") [.cost cost] [cyclingExpansion cost]

/-! ## Cards -/

/-- A printed power, toughness, loyalty, or defense. -/
semantic_macro stat (value : Nat) : Option Amount := some (.lit value)
/-- One level band of a leveler: its range, power/toughness box, and text [CR#711.2a]. -/
semantic_macro levelBand (range : LevelRange) (power toughness : Nat) (text : List Ability) : LevelBand :=
  { range, text, power := stat power, toughness := stat toughness }
/-- A prototype frame's inset set: its mana cost and power/toughness box [CR#718.1]. -/
semantic_macro prototypeAlt (cost : ManaCost) (power toughness : Nat) : PrototypeFrame :=
  { cost := some cost, power := stat power, toughness := stat toughness }

/-- A Food token's characteristics: "a colorless Food artifact token with '{2}, {T}, Sacrifice
this token: You gain 3 life.'" [CR#111.10b] -/
semantic_macro foodToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [artifactType "Food"],
        text :=
          [ activated (.compound [.mana [generic 2], .tapSymbol, .perform (sacrifice thisArtifact)])
              (gainLife (.lit 3)) ] } }
/-- A Treasure token's characteristics: "a colorless Treasure artifact token with '{T},
Sacrifice this token: Add one mana of any color.'" [CR#111.10a] -/
semantic_macro treasureToken : CharacteristicBundle :=
  { characteristics :=
      { types := [.artifact], subtypes := [artifactType "Treasure"],
        text :=
          [ activated (.compound [.tapSymbol, .perform (sacrifice thisArtifact)])
              (.addMana (.lit 1) (.anyColor .sameColor) []) ] } }
/-- "create N tapped <token>" [CR#111.2] -/
semantic_macro createTappedToken (token : CharacteristicBundle) (count : Amount := .lit 1) :
    Instruction :=
  Primitives.Instruction.create count (.written token) [.entersAs .tapped]

end Semantics.Macros
