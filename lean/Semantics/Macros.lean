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
/-- "that token": the token just created. -/
semantic_macro itAsToken : NounPhrase := .pro .tokenBorn .one .whole

/-- "it", stamped by the verb that produced it: "the exiled card". -/
semantic_macro itVerbed (verb : Deed) : NounPhrase := .pro (.stamped verb) .one .whole
/-- "it": the object the previous instruction introduced, read through a window over exactly
what that instruction announced. -/
semantic_macro itPrior (prev : Instruction) : NounPhrase :=
  .pro .bare .one (.introduced ((Instruction.intro [] prev).map Binding.kind))
/-- "them", stamped by the verb that produced them: "the destroyed creatures". -/
semantic_macro themVerbed (verb : Deed) : NounPhrase := .pro (.stamped verb) .many .whole
/-- "that turn" -/
semantic_macro thatTurn : NounPhrase := .pro .thatTurn .one .whole
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

/-- "the <verb>ed <word>", e.g. "the tapped creatures". -/
semantic_macro theVerbed (verb : Deed) (word : NounWord) (marking : VerbedMarking)
    (plurality : Plurality) : NounPhrase :=
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
/-- "if it's a <p> card": the card slot's occupant, tested. -/
semantic_macro itsACard (p : Predicate) : Condition := .matches itCard p
/-- "if it's a <p>" -/
semantic_macro itsA (p : Predicate) : Condition := .matches it p
/-- "if it isn't a <p>" -/
semantic_macro itIsntA (p : Predicate) : Condition := .not (itsA p)

/-- "the number of …s" -/
semantic_macro countOf (p : Predicate) : Amount := .countOf (bare p)

/-- "the greatest power among …s" -/
semantic_macro aggregate (op : AggregateOp) (axis : ProjAxis) (p : Predicate) : Amount :=
  .aggregate op axis (bare p)

/-! ## Zones -/

semantic_macro battlefield : ZoneExpr := .zone .battlefield .bare
semantic_macro exileZone : ZoneExpr := .zone .exile .bare
semantic_macro hand : ZoneExpr := .zone .hand .bare
semantic_macro library : ZoneExpr := .zone .library .bare
semantic_macro graveyard : ZoneExpr := .zone .graveyard .bare
semantic_macro stack : ZoneExpr := .zone .stack .bare
semantic_macro command : ZoneExpr := .zone .command .bare
/-- "the command zone", scoped to nobody. -/
semantic_macro commandZone : ZoneExpr := .zone .command .bare
semantic_macro libraryOf (player : NounPhrase) : ZoneExpr := .zone .library (.possessedBy player)
semantic_macro yourLibrary : ZoneExpr := libraryOf .you
/-- "on the bottom of your library in <arrangement>" -/
semantic_macro onBottomIn (arrangement : Arrangement) : ZoneExpr :=
  .library (.oneEnd .bottom) (some arrangement) none .bare
semantic_macro onTopIn (arrangement : Arrangement) : ZoneExpr :=
  .library (.oneEnd .top) (some arrangement) none .bare
/-- "on the bottom of its owner's library" -/
semantic_macro onBottom : ZoneExpr := .library (.oneEnd .bottom) none none .bare
semantic_macro onTop : ZoneExpr := .library (.oneEnd .top) none none .bare
/-- "on the top or bottom of your library" -/
semantic_macro topOrBottom : ZoneExpr := .library (.eitherEnd none) none none .bare
/-- "on the top or bottom of its owner's library, <chooser>'s choice" -/
semantic_macro choiceOfTopOrBottom (chooser : NounPhrase) : ZoneExpr :=
  .library (.eitherEnd (some chooser)) none none .bare
/-- "Nth from the top or bottom of your library" -/
semantic_macro nthFromTopOrBottom (ordinal : Ordinal) : ZoneExpr :=
  .library (.eitherEnd none) none (some ordinal) .bare
semantic_macro handOf (player : NounPhrase) : ZoneExpr := .zone .hand (.possessedBy player)
semantic_macro graveyardOf (player : NounPhrase) : ZoneExpr := .zone .graveyard (.possessedBy player)
/-- "Nth from the top of its owner's library" -/
semantic_macro nthFromTop (ordinal : Ordinal) : ZoneExpr := .library (.oneEnd .top) none (some ordinal) .bare

/-! ## Predicates -/

semantic_macro creature : Predicate := .hasType .creature
semantic_macro artifact : Predicate := .hasType .artifact
semantic_macro enchantment : Predicate := .hasType .enchantment
semantic_macro land : Predicate := .hasType .land
semantic_macro instant : Predicate := .hasType .instant
semantic_macro sorcery : Predicate := .hasType .sorcery
semantic_macro instantOrSorcery : Predicate := .or [instant, sorcery]
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
semantic_macro nthCastBy (ordinal : Ordinal) (player : NounPhrase) (period : RankPeriod) : Predicate :=
  .castBy player (some (ordinal, period))
/-- "any other target" -/
semantic_macro anyOtherTarget : Predicate := .and [anyTarget, .other]

/-- "a color", "a creature type": a quality noun. -/
semantic_macro quality (sort : QualitySort) : Predicate := .qualityNoun sort none
/-- A quality noun with its domain: "a color other than blue". -/
semantic_macro qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate :=
  .qualityNoun sort (some domain)
/-- "a <dom> with <stat> <r> <bound>", the stat read of the member itself. -/
semantic_macro comparesOwnStat (stat : Stat) (domain : Predicate) (comparator : Comparator) (bound : Amount) :
    Predicate :=
  .compareOver domain (.statOf (.stat stat) (.pro .bare .one (.top 1))) comparator bound
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
semantic_macro yourCommander : NounPhrase := .designated "commander" .you
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
semantic_macro topSlice (amount : Amount) : NounPhrase := .librarySlice .top amount .you
/-- "the bottom card of your library" -/
semantic_macro bottomCard : NounPhrase := .librarySlice .bottom (.lit 1) .you
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

semantic_macro untilEndOfTurn : Duration := .until_ (.endOf .turn none)
semantic_macro untilYourNextTurn : Duration := .until_ (.startOf .turn (some .you))
semantic_macro untilEndOfCombat : Duration := .until_ (.endOf .combat none)
semantic_macro untilYourNextEndStep : Duration := .until_ (.startOf .endStep (some .you))

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

semantic_macro plusOnePlusOne : CounterKind := .boost (.up 1) (.up 1)
semantic_macro flyingCounter : CounterKind := .keyword "Flying"
semantic_macro minusOneMinusOne : CounterKind := .boost (.down 1) (.down 1)

/-! ## Instructions -/

semantic_macro move (subject : NounPhrase) (destination : ZoneExpr) : Instruction :=
  .move subject destination []
semantic_macro destroy (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Destroy") (.move subject graveyard []) (agent := agent)
semantic_macro exile (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Exile") (.move subject exileZone []) (agent := agent)
/-- "exile <subject> with N <kind> counters on it" -/
semantic_macro exileWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind)
    (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Exile") (.move subject exileZone [.withCounters amount (.printed kind) .fresh])
      (agent := agent)
semantic_macro sacrifice (subject : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Sacrifice") (.move subject graveyard []) (agent := some agent)
/-- "<agent> sacrifices it": the permanent slot's occupant. -/
semantic_macro sacrificeIt (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  sacrifice (.pro (.atSlot .permanent) .one .whole) (agent := agent)
/-- "<agent> puts <subject> <destination>" -/
semantic_macro put (subject : NounPhrase) (destination : ZoneExpr) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  .enact (.core .put) (.move subject destination []) (agent := some agent)
/-- "return <subject> to <zone>" -/
semantic_macro returnTo (subject : NounPhrase) (destination : ZoneExpr) (riders : List TokenRider)
    (agent : Option NounPhrase := none) : Instruction :=
  .enact (.core .return_) (.move subject destination riders) (agent := agent)
/-- "return <subject> to the battlefield" -/
semantic_macro returnToBattlefield (subject : NounPhrase) : Instruction := returnTo subject battlefield []
/-- "return <subject> to the battlefield transformed under <controller>'s control" -/
semantic_macro returnToBattlefieldTransformed (subject controller : NounPhrase) : Instruction :=
  returnTo subject battlefield [.entersTransformed, .under controller]
/-- "return <subject> to the battlefield under <who>'s control with N <kind> counters on it" -/
semantic_macro returnToBattlefieldWithCounters (subject who : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Instruction :=
  .move subject battlefield [.under who, .withCounters amount (.printed kind) .fresh]
/-- "transform <subject>" -/
semantic_macro transform (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Transform") (.turnOver subject) (agent := agent)
/-- "meld <subject> into <name>" -/
semantic_macro meldInto (subject : NounPhrase) (into : String) (agent : Option NounPhrase := none) :
    Instruction :=
  .enact (.action "Meld") (.move subject battlefield [.entersMelded into]) (agent := agent)
/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a].
This retains the existing fragment; face-down characteristics and the turn-up special
action are not represented here. -/
semantic_macro manifestPlacement (subject : NounPhrase) : Instruction :=
  .enact (.action "Manifest") (.move subject battlefield []) (agent := none)
semantic_macro tap (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Tap") (.setStatus .tapped subject) (agent := agent)
semantic_macro discard (subject : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Discard") (.move subject graveyard []) (agent := some agent)
semantic_macro shuffle (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .shuffle (agent := agent)
semantic_macro untap (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Untap") (.setStatus .untapped subject) (agent := agent)
/-- "Exile <subject> until <event>." -/
semantic_macro exileUntil (subject : NounPhrase) (event : GameEvent) : Instruction :=
  .holdUntil (exile subject) event
/-- "<agent> mills <amount> cards" from <whose> library. -/
semantic_macro mill (amount : Amount) (whose : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Mill") (.move (.librarySlice .top amount whose) graveyard []) (agent := some
      agent)
semantic_macro putOntoBattlefield (subject : NounPhrase) : Instruction := .move subject battlefield []
semantic_macro putOntoBattlefieldTapped (subject : NounPhrase) : Instruction :=
  .move subject battlefield [.entersAs .tapped]
/-- "put <subject> onto the battlefield under your control" -/
semantic_macro putOntoBattlefieldUnderYourControl (subject : NounPhrase) : Instruction :=
  .move subject battlefield [.under .you]
/-- "put <subject> onto the battlefield tapped and attacking" -/
semantic_macro putOntoBattlefieldTappedAttacking (subject : NounPhrase) : Instruction :=
  .move subject battlefield [.entersAs .tapped, .entersAttacking none]
/-- "Search your library for <quantity> <p>" -/
semantic_macro searchLibraryFor (quantity : Quantity) (p : Predicate) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  .search (.oneZone yourLibrary) quantity p (agent := agent)
/-- "search <whose>'s graveyard, hand, and library for <q> <p>" -/
semantic_macro searchZonesOf (whose : NounPhrase) (quantity : Quantity) (p : Predicate)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .search (.someZones (some whose) [.graveyard, .hand, .library]) quantity p (agent := agent)
/-- "search your library and/or graveyard for a <p>" -/
semantic_macro searchLibraryOrGraveyard (p : Predicate) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .search (.someZones (some .you) [.library, .graveyard]) (exactly 1) p (agent := agent)
/-- "<who> searches their library for a <p>" -/
semantic_macro searchTheirLibraryFor (p : Predicate) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .search (.oneZone (libraryOf they)) (exactly 1) p (agent := agent)
/-- "<who> reveals their hand" -/
semantic_macro revealTheirHand (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .expose .reveal (.zone (handOf
    they)) (agent := agent)
/-- Fight captures each selected creature once and guards the whole simultaneous event
[CR#701.14a..701.14c]. -/
semantic_macro fight (left right : capture NounPhrase) : Instruction :=
  .doIf (.and [.matches left (.and [creature, permanent]),
              .matches right (.and [creature, permanent])])
    (.enact (.action "Fight") (.simultaneously [
      .dealDamage left (.statOf (.stat .power) left) right,
      .dealDamage right (.statOf (.stat .power) right) left])) none

/-- The replacement's application, rather than installation of a new shield
[CR#701.19b,701.19c]. -/
semantic_macro regenerationApplication (subject : capture NounPhrase) : Instruction :=
  .enact (.action "Regenerate") <|
    .sequentially [
      .clearDamage subject,
      .enact (.action "Tap") (.setStatus .tapped subject)
        (some (.possessorOf .controller subject)),
      .doIf (.or [.matches subject (.inCombat .attackerOf none),
                  .matches subject (.inCombat .blockerOf none)])
        (.combat subject (.participation .outsideCombat)) none]

/-- A resolving regeneration instruction installs a single-use shield for this turn;
only its eventual application carries the Regenerate label [CR#701.19a,701.19c]. -/
semantic_macro regenerate (subject : capture NounPhrase) : Instruction :=
  .establish (.replacement
    (.verbedEvent none (.action "Destroy") (some subject) none none)
    [] none (regenerationApplication subject) .nextTimeOnly none) (some .thisTurn)

/-- Losing counters is removal from the named player. Capturing that player first keeps
amount references in the order the sentence introduces them. -/
semantic_macro loseCounters (kind : Option CounterKindSource) (amount : Option Amount)
    (agent : capture NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .removeCounters (amount.map Quantity.exactlyOf) kind agent
semantic_macro loseLife (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .changeLife (.down amount) (agent := agent)
semantic_macro gainLife (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .changeLife (.up amount) (agent := agent)
/-- "<player>'s life total becomes <amount>" -/
semantic_macro setLife (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .changeLife (.set amount) (agent := agent)
semantic_macro draw (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .draw amount (agent :=
    agent)

semantic_macro lookAt (cards : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .expose .lookAt
    (.cards cards) (agent := agent)
/-- "look at <player>'s hand" -/
semantic_macro lookAtHandOf (player : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .expose .lookAt
    (.zone (handOf player)) (agent := agent)
semantic_macro revealCards (cards : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .expose .reveal
    (.cards cards) (agent := agent)
/-- "the card found by a search" -/
semantic_macro foundCard : NounPhrase := itVerbed (.action "Search")
/-- "reveal it": the card a search found. -/
semantic_macro revealIt : Instruction := revealCards foundCard
semantic_macro shuffleInto (subject : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Shuffle") (.move subject ((.library .shuffled none none .bare)) []) (agent := some
      agent)
semantic_macro doIf (condition : Condition) (instruction : Instruction) : Instruction :=
  .doIf condition instruction none
/-- "choose <subject>" -/
semantic_macro choose (subject : NounPhrase) (disclosure : Disclosure := .openly)
    (agent : Option NounPhrase := none) : Instruction :=
  .choose none subject disclosure none (agent := agent)
/-- "choose <subject> as you <event>" -/
semantic_macro chooseWhile (subject : NounPhrase) (while_ : Concurrent) : Instruction :=
  .choose none subject .openly (some while_) (agent := none)
semantic_macro rollDice (count : Nat) (sides : Nat) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .rollDice (.lit count) (.sides sides) (agent := agent)
/-- One row of a results table: "<results> — <instruction>". -/
semantic_macro rollRow (results : Quantity) (instruction : Instruction) : RollRow := ⟨results, instruction⟩
semantic_macro flipCoins (count : Nat) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .flipCoins (.count (.lit count)) (agent := agent)
/-- "<player> flips a coin" as an event -/
semantic_macro flipsCoin (player : NounPhrase) : GameEvent := .flipsCoin player none
/-- "<decider> may <body>" -/
semantic_macro offer (body : Instruction) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := Primitives.Instruction.offer body none none
    (agent := agent)
/-- "<decider> may <body>. When they do, <trigger>": a reflexive trigger on the choice. -/
semantic_macro offerWhen (body : Instruction) (trigger : Instruction) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  .triggerReflexively (Primitives.Instruction.offer body none none (agent := agent)) trigger
/-- "the chosen number" -/
semantic_macro chosenNumber : Amount := .chosenNumber .theChoice
/-- "Choose one or more — [cost] — <mode>; …" [CR#702.172a] -/
semantic_macro chooseSpree (modes : List (Option Cost × Instruction)) : Instruction := .chooseModes (atLeast 1)
    modes
/-- "<voters> vote for <ballot>" -/
semantic_macro vote (disclosure : Disclosure) (ballot : Ballot) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .vote none disclosure ballot (agent := agent)
/-- "Starting with <first>, <voters> vote for <ballot>" -/
semantic_macro voteStartingWith (first : NounPhrase) (disclosure : Disclosure) (ballot : Ballot)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .vote (some first) disclosure ballot (agent := agent)
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
semantic_macro create (count : Amount) (token : CharacteristicBundle) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  Primitives.Instruction.create count (.written token) [] (agent := agent)
/-- "create N <token> tapped and attacking" -/
semantic_macro createTappedAttacking (count : Amount) (token : CharacteristicBundle)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  Primitives.Instruction.create count (.written token) [.entersAs .tapped, .entersAttacking none] (agent := agent)

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
semantic_macro eventCount (event : GameEvent) (who : NounPhrase) (lookback : Lookback) : Amount :=
  .eventTally .count who (.mk event lookback)
/-- A historical event involving the subject. -/
semantic_macro happened (event : GameEvent) (who : NounPhrase) (lookback : Lookback) : Condition :=
  .happened who (.mk event lookback)
/-- A historical event whose gap stands for the entity being described. -/
semantic_macro happenedTo (event : GameEvent) (lookback : Lookback) : Predicate :=
  .happenedTo (.mk event lookback)
/-- The accumulated magnitude of historical events involving the subject. -/
semantic_macro eventSum (event : GameEvent) (who : NounPhrase) (lookback : Lookback) : Amount :=
  .eventTally .sum who (.mk event lookback)
/-- "if there is no <designation>" -/
semantic_macro thereIsNo (designation : DesignationLabel) : Condition := .noHolder designation
/-- "<subject>'s <keyword> cost was paid", read back. -/
semantic_macro paidCostRead (which : PaidCostName) (window : Option Lookback) (subject : NounPhrase) :
    Amount :=
  .paid (.readback which window) subject
/-- "the number of times <subject>'s <keyword> cost was paid" -/
semantic_macro timesPaid (which : PaidCostName) (subject : NounPhrase) : Amount :=
  .paid (.timesPaid which) subject
/-- "if <subject>'s <cost> was paid" -/
semantic_macro costWasPaid (which : PaidCostName) (window : Option Lookback) (subject : NounPhrase) :
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
  .deonticRule player .permit [.action "Play"] .agent (some (.additional quantity))
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
/-- "there is an additional <part> [after <anchor>]" -/
semantic_macro addPart (part : TurnPart) (anchor : Option TurnPart) (count : Amount) : Instruction :=
  Primitives.Instruction.addPart part anchor count none (agent := none)
/-- "there is an additional <part> after this phase, followed by an additional <next>" -/
semantic_macro addPartThen (part : TurnPart) (anchor : Option TurnPart) (count : Amount) (next : TurnPart) :
    Instruction :=
  Primitives.Instruction.addPart part anchor count (some next) (agent := none)
/-- "<player> gets an additional <part>" -/
semantic_macro getAdditionalPart (part : TurnPart) (count : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  Primitives.Instruction.addPart part none count none (agent := some agent)
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

/-- "<player> may pay <cost>. If they don't, <instruction>." -/
semantic_macro doUnless (instruction : Instruction) (cost : Cost) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  Primitives.Instruction.offer (.pay cost .once (agent := (agentRef agent))) none (some instruction) (agent := agent)

/-- "it" or "them", by number. -/
semantic_macro itOrThem : Plurality → NounPhrase
  | .one => it
  | .many => them
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
/-- "<subject> gets +P/+T [until …]": the two stat changes as one static clause; the toughness
half reads its subject back as "it". -/
semantic_macro getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec :=
  .conjunction none
    [.modification subject .power power, .modification (itsOther subject power) .toughness
        toughness]
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
  sacrifice patient (agent := controller)

/-- "them" (or "it"): the cards a look at a library slice just announced, seen alone. -/
semantic_macro lookedCards (slice : NounPhrase) : NounPhrase :=
  .pro .bare slice.plur (.introduced ((NounPhrase.introduced [] slice).map Binding.kind))
/-- "<looker> looks at the top N cards of <whose> library, puts any number of them on the bottom
in any order and the rest on top in any order" -/
semantic_macro lookAndSort (whose : NounPhrase) (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  let slice : NounPhrase := .librarySlice .top amount whose
  .sequentially
    [ .expose .lookAt (.cards slice) (agent := agent),
      move (someOf anyNumber (lookedCards slice)) (onBottomIn .anyOrder),
      move (theRest .object) (onTopIn .anyOrder) ]
/-- The same look, spilling the cards put aside into <spill> instead of the bottom. -/
semantic_macro lookAndSortInto (whose : NounPhrase) (amount : Amount) (spill : ZoneExpr)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  let slice : NounPhrase := .librarySlice .top amount whose
  .sequentially
    [ .expose .lookAt (.cards slice) (agent := agent),
      move (someOf anyNumber (lookedCards slice)) spill,
      move (theRest .object) (onTopIn .anyOrder) ]
/-- "<agent> scries N" [CR#701.22a] -/
semantic_macro scry (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Scry") (lookAndSort (agentRef agent) amount (agent := (agentRef agent))) (agent
      := some agent)
/-- "<agent> fateseals N" [CR#701.29a] -/
semantic_macro fateseal (whose : NounPhrase) (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Fateseal") (lookAndSort whose amount (agent := (agentRef agent))) (agent := some
      agent)
/-- "<agent> surveils N" [CR#701.25a] -/
semantic_macro surveil (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Surveil") (lookAndSortInto (agentRef agent) amount graveyard (agent := (agentRef
      agent))) (agent := some agent)
/-- "Proliferate" with its reminder text [CR#701.34a]: "Choose any number of permanents and/or
players, then give each another counter of each kind already there." -/
semantic_macro proliferate : Instruction :=
  .enact (.action "Proliferate") (.sequentially
      [ .choose none (counted anyNumber
            (.or [ .and [permanent, .hasCounters none],
                   .compare [.anyCounter .player] .atLeast (.lit 1) ])) .openly none (agent :=
                       none),
        .putCounters (.lit 1) .own (.eachOf (those .join)) ]) (agent := none)
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
      .putCounters (.lit count) (.printed plusOnePlusOne) (that (.type .creature)),
      .doIf (itIsntA (.hasSubtype (creatureType subtype)))
        (.establish
          (Primitives.StaticSpec.qualityChange it .adds
            (.bundle { characteristics := { subtypes := [creatureType subtype] } } none))
          none)
        none ]
/-- "monstrosity N" with its reminder text [CR#701.37a]: "If this permanent isn't monstrous,
put N +1/+1 counters on it and it becomes monstrous." -/
semantic_macro makeMonstrous (amount : Amount) : Instruction :=
  .doIf (.not (.matches thisPermanent (.hasDesignation "monstrous" none)))
    (.sequentially
      [ .putCounters amount (.printed plusOnePlusOne) thisPermanent,
        .gainDesignation thisPermanent "monstrous" (.byDeed (.action "Monstrosity")) none ])
    none
semantic_macro get (subject : NounPhrase) (power toughness : Delta Amount) (duration : Option Duration) :
    Instruction :=
  .establish (getsPt subject power toughness) duration

semantic_macro gain (subject : NounPhrase) (ability : Ability) (duration : Option Duration) : Instruction :=
  .establish (.abilityGrant subject ability) duration
/-- "<subject> gains haste [until …]" -/
semantic_macro gainHaste (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  gain subject (.keyword "Haste" [] none) duration
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
  Primitives.StaticSpec.partScope .untapStep whose (objectCant (.action "Untap") subject)
/-- "you may choose not to untap <subject> during [<whose>] untap step" -/
semantic_macro mayDeclineUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.deonticRule subject .permit [.action "Untap"] .patient none .noPatient none .noRider)
/-- "untap <subject> during [<whose>] untap step" -/
semantic_macro untapsDuring (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.deonticRule subject .require [.action "Untap"] .patient none .noPatient none .noRider)
/-- "<subject> can block an additional creature each combat" -/
semantic_macro mayBlockAdditional (subject : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .deonticRule subject .permit [.core .block] .agent (some (.additional quantity))
    (.counterpart (allOf creature)) none .noRider
/-- "<who> may vote an additional time" -/
semantic_macro mayVoteAdditional (who : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .deonticRule who .permit [.action "Vote"] .agent (some (.additional quantity)) .noPatient none
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

/-! ## Events -/

semantic_macro leavesBattlefield (subject : NounPhrase) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (.zones [battlefield]))
/-- "<subject> leaves <zone>" -/
semantic_macro leavesZone (subject : NounPhrase) (zone : ZoneExpr) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (.zones [zone]))
/-- "at the beginning of <possessor>'s <part>" -/
semantic_macro beginningOfPossessed (quantifier : PartQuant) (part : TurnPart) (possessor : NounPhrase) :
    GameEvent :=
  .beginningOf quantifier part (.byPlayer possessor)
/-- "that turn's": the extra turn just granted, as a header possessor. -/
semantic_macro thatTurns : HeaderPossessor := .byTurn thatTurn
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
  .verbedEvent none (.action "Regenerate") (some subject) none none

/-- Copy exceptions retain their own edit block; added abilities remain copy-specific. -/
semantic_macro copyCharacteristics (c : Characteristics) (typesAdded : Bool) : List CopyExcept :=
  let typeOp := if typesAdded then QualityOp.adds else .sets
  let types := if c.supertypes.isEmpty && c.types.isEmpty && c.subtypes.isEmpty then []
    else [.typeLine typeOp c.typeChanges]
  let values := ({ c with text := [] } : Characteristics).valueEdits .sets
  let edits := types ++ values
  (if edits.isEmpty then [] else [.edits edits]) ++ c.text.map CopyExcept.ability

/-! ## Abilities -/

semantic_macro keyword (label : KeywordLabel) : Ability := .keyword label [] none
/-- An ability word in italics before an ability: "Will of the council — …" [CR#207.2c]. -/
semantic_macro abilityWord (word : AbilityWordLabel) (ability : Ability) : Ability :=
  .italicHead (.abilityWord word) ability
/-- A flavor word in italics before an ability [CR#207.2d]. -/
semantic_macro flavorWord (word : FlavorWordLabel) (ability : Ability) : Ability :=
  .italicHead (.flavorWord word) ability
/-- "Companion — <condition>" -/
semantic_macro companion (condition : DeckCondition) : Ability :=
  .keyword "Companion" [.deckCondition condition] none
/-- "Pay N life" as a cost. -/
semantic_macro payLife (player : NounPhrase) (amount : Nat) : Cost :=
  .perform (.changeLife (.down (.lit amount)) (agent := player))
/-- "<keyword> <cost>" -/
semantic_macro keywordCosting (label : KeywordLabel) (cost : Cost) : Ability :=
  .keyword label [.cost cost] none
/-- "<keyword> <quality>", e.g. "protection from red" -/
semantic_macro keywordQuality (label : KeywordLabel) (quality : Predicate) : Ability :=
  .keyword label [.quality quality] none
/-- "<keyword> <subject>", e.g. "enchant creature" -/
semantic_macro keywordSubject (label : KeywordLabel) (subject : Predicate) : Ability :=
  .keyword label [.subject subject] none
/-- "<keyword> N", e.g. "bushido 2" -/
semantic_macro keywordNumber (label : KeywordLabel) (amount : Amount) : Ability :=
  .keyword label [.number amount] none
/-- "<keyword> <quality> <cost>", e.g. "plainscycling {2}" [CR#702.29e] -/
semantic_macro keywordQualityCosting (label : KeywordLabel) (quality : Predicate) (cost : Cost) : Ability :=
  .keyword label [.quality quality, .cost cost] none
/-- "<keyword> N—<cost>", e.g. "suspend 4—{1}{U}" -/
semantic_macro keywordNumberCosting (label : KeywordLabel) (amount : Amount) (cost : Cost) : Ability :=
  .keyword label [.number amount, .cost cost] none
/-- "Level up [cost]" [CR#702.87a] -/
semantic_macro levelUp (cost : Cost) : Ability := keywordCosting "LevelUp" cost
/-- "When <event>, if <condition>, <instruction>" -/
semantic_macro triggeredIf (event : GameEvent) (condition : Condition)
    (instruction : Instruction) : Ability :=
  .triggered event [] none [] none none (some condition) instruction
/-- "Whenever <event>, <instruction>. This ability triggers only once each turn." -/
semantic_macro triggeredOnlyOnce (event : GameEvent) (limit : UsageLimit) (instruction : Instruction) :
    Ability :=
  .triggered event [] none [] none (some limit) none instruction
/-- "if it isn't <p>", read of the ability just named. -/
semantic_macro itIsntAnAbility (p : Predicate) : Condition := .not (.matches (that .ability) p)
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
semantic_macro triggeredWhile (event : GameEvent) (while_ : Concurrent) (instruction : Instruction) :
    Ability :=
  .triggered event [] (some while_) [] none none none instruction
/-- "Whenever <event> during <timing>, <instruction>" -/
semantic_macro triggeredOnlyDuring (event : GameEvent) (timing : Timing) (instruction : Instruction) :
    Ability :=
  .triggered event [] none [] (some timing) none none instruction
/-- A joined "whenever <event> while <concurrent>" header. -/
semantic_macro joinedHeadWhile (event : GameEvent) (while_ : Concurrent) : JoinedHeader :=
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
    (.not (.matches thisCreature (.hasDesignation "renowned" none)))
    (.sequentially
      [ .putCounters (.lit count) (.printed plusOnePlusOne) thisCreature,
        .gainDesignation thisCreature "renowned" (.byKeyword "Renown") none ])
/-- Storm's reminder text: "When you cast this spell, copy it for each other spell that was cast
before it this turn. You may choose new targets for the copies." [CR#702.40a] -/
semantic_macro stormExpansion : Ability :=
  when (.casts .you (some thisSpell) none)
    (.sequentially
      [ .copy .fromStack thisSpell (eventCount (.casts (relative .player) (some (a (.and [spell,
        .otherThan thisSpell]))) none) (a .anyPlayer) .earlierThisTurn) [] (agent := .you),
        offer (.chooseNewTargets (.pro (.word .copy) .many .whole)) (agent := .you) ])
/-- "Storm" with its reminder text. -/
semantic_macro storm : Ability := .keyword "Storm" [] (some stormExpansion)
/-- "Renown N" with its reminder text. -/
semantic_macro renown (count : Nat) : Ability :=
  .keyword "Renown" [.number (.lit count)] (some (renownExpansion count))
/-- Cumulative upkeep's reminder text: "At the beginning of your upkeep, if this permanent is
on the battlefield, put an age counter on it. Then you may pay [cost] for each age counter on
it. If you don't, sacrifice it." [CR#702.24a] -/
semantic_macro cumulativeUpkeepExpansion (cost : Cost) : Ability :=
  triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
    (.matches thisPermanent (.inZone battlefield))
    (.sequentially
      [ .putCounters (.lit 1) (.printed (.named "Age")) thisPermanent,
        Primitives.Instruction.offer (.pay (.scaled cost (times (.lit 1) (countersOn (.named "Age") thisPermanent))) .once
            (agent := .you)) none (some (sacrifice thisPermanent (agent := .you))) (agent := .you)
            ])
/-- "Cumulative upkeep [cost]" with its reminder text. -/
semantic_macro cumulativeUpkeep (cost : Cost) : Ability :=
  .keyword "CumulativeUpkeep" [.cost cost] (some (cumulativeUpkeepExpansion cost))
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

/-! ## Cards -/

/-- A printed power, toughness, loyalty, or defense. -/
semantic_macro stat (value : Nat) : Option Amount := some (.lit value)
/-- One level band of a leveler: its range, power/toughness box, and text [CR#711.2a]. -/
semantic_macro levelBand (range : LevelRange) (power toughness : Nat) (text : List Ability) : LevelBand :=
  { range, text, power := stat power, toughness := stat toughness }
/-- A prototype frame's inset set: its mana cost and power/toughness box [CR#718.1]. -/
semantic_macro prototypeAlt (cost : ManaCost) (power toughness : Nat) : PrototypeFrame :=
  { cost := some cost, power := stat power, toughness := stat toughness }


end Semantics.Macros
