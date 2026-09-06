import Semantics
import Semantics.Macros.Primitives
import Semantics.Check

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
def it : NounPhrase := .pro .bare .one .whole

/-- "them" -/
def them : NounPhrase := .pro .bare .many .whole
/-- "that card": the card slot's occupant. -/
def itCard : NounPhrase := .pro (.atSlot .card) .one .whole
/-- "that token": the token just created. -/
def itAsToken : NounPhrase := .pro .tokenBorn .one .whole

/-- "it", stamped by the verb that produced it: "the exiled card". -/
def itVerbed (verb : Deed) : NounPhrase := .pro (.stamped verb) .one .whole
/-- "it": the object the previous instruction introduced, read through a window over exactly
what that instruction announced. -/
def itPrior (prev : Instruction) : NounPhrase :=
  .pro .bare .one (.introduced ((Instruction.intro [] prev).map Binding.kind))
/-- "them", stamped by the verb that produced them: "the destroyed creatures". -/
def themVerbed (verb : Deed) : NounPhrase := .pro (.stamped verb) .many .whole
/-- "that turn" -/
def thatTurn : NounPhrase := .pro .thatTurn .one .whole
/-- "it", read as the subject of the condition just stated. -/
def itCondSubject (condition : Condition) : NounPhrase :=
  .pro .bare .one (.introduced ((Condition.introduced [] condition).map Binding.kind))

/-- "that <word>", e.g. `that .player`. -/
def that (w : NounWord) : NounPhrase := .pro (.word w) .one .whole
/-- "that player or planeswalker": the joined phrase just named. -/
def thatJoin : NounPhrase := that .join

/-- "those <word>s", e.g. `those .card`. -/
def those (w : NounWord) : NounPhrase := .pro (.word w) .many .whole

/-- "they": the player most recently named. -/
def they : NounPhrase := .pro (.word .player) .one .whole
/-- "that player or that permanent's controller": the player half of a "target player or
planeswalker" split, or the controller of the permanent half. -/
def splitOverPlaneswalker : NounPhrase :=
  Primitives.NounPhrase.eitherOf (.pro (.unionHalf .player) .one .whole)
    (.possessorOf .controller (.pro (.unionHalf (.type .planeswalker)) .one .whole))
/-- The same over "any target": the player, or the permanent's controller. -/
def splitOverPermanent : NounPhrase :=
  Primitives.NounPhrase.eitherOf (.pro (.unionHalf .player) .one .whole)
    (.possessorOf .controller (.pro (.unionHalf .permanent) .one .whole))

/-- "the <verb>ed <word>", e.g. "the tapped creatures". -/
def theVerbed (verb : Deed) (word : NounWord) (marking : VerbedMarking)
    (plurality : Plurality) : NounPhrase :=
  .pro (.verbed verb word marking) plurality .whole

/-! ## Quantities -/

def exactly (n : Nat) : Quantity := .range (some n) (some n)
def upTo (n : Nat) : Quantity := .range none (some n)
def anyNumber : Quantity := .range none none
def atLeast (n : Nat) : Quantity := .range (some n) none
def oneThrough (n : Nat) : Quantity := .range (some 1) (some n)

/-! ## Determiners -/

/-- "target …" -/
def target (p : Predicate) : NounPhrase := .described (.target (.range (some 1) (some 1))) p

/-- "each …" -/
def each (p : Predicate) : NounPhrase := .described .each p

/-- "all …" -/
def allOf (p : Predicate) : NounPhrase := .described .all p

/-- the bare plural: "creatures" -/
def bare (p : Predicate) : NounPhrase := .described .bare p

/-- "a …" -/
def a (p : Predicate) : NounPhrase := .described (.a .unmarked) p

/-- "a … at random" -/
def aAtRandom (p : Predicate) : NounPhrase := .described (.a .atRandom) p
/-- "a … of their choice", the chooser being the last-mentioned player. -/
def aTheirChoice (p : Predicate) : NounPhrase := .described (.a (.theirChoice (.top 1))) p
/-- "a … of your choice" -/
def aYourChoice (p : Predicate) : NounPhrase := .described (.a .yourChoice) p
/-- "the …" -/
def the (p : Predicate) : NounPhrase := .described .the p

/-- "N …", "up to N …" -/
def counted (q : Quantity) (p : Predicate) : NounPhrase := .described (.count q none) p
/-- "N <p> at random" -/
def countedAtRandom (q : Quantity) (p : Predicate) : NounPhrase :=
  .described (.count q (some .atRandom)) p

/-- "if there is a …" -/
def exists_ (p : Predicate) : Condition := .exists_ (bare p)
/-- "if it's a <p> card": the card slot's occupant, tested. -/
def itsACard (p : Predicate) : Condition := .matches itCard p
/-- "if it's a <p>" -/
def itsA (p : Predicate) : Condition := .matches it p
/-- "if it isn't a <p>" -/
def itIsntA (p : Predicate) : Condition := .not (itsA p)

/-- "the number of …s" -/
def countOf (p : Predicate) : Amount := .countOf (bare p)

/-- "the greatest power among …s" -/
def aggregate (op : AggregateOp) (axis : ProjAxis) (p : Predicate) : Amount :=
  .aggregate op axis (bare p)

/-! ## Zones -/

def battlefield : ZoneExpr := .zone .battlefield .bare
def exileZone : ZoneExpr := .zone .exile .bare
def hand : ZoneExpr := .zone .hand .bare
def library : ZoneExpr := .zone .library .bare
def graveyard : ZoneExpr := .zone .graveyard .bare
def stack : ZoneExpr := .zone .stack .bare
def command : ZoneExpr := .zone .command .bare
/-- "the command zone", scoped to nobody. -/
def commandZone : ZoneExpr := .zone .command .bare
def libraryOf (player : NounPhrase) : ZoneExpr := .zone .library (.possessedBy player)
def yourLibrary : ZoneExpr := libraryOf .you
/-- "on the bottom of your library in <arrangement>" -/
def onBottomIn (arrangement : Arrangement) : ZoneExpr :=
  .library (.oneEnd .bottom) (some arrangement) none .bare
def onTopIn (arrangement : Arrangement) : ZoneExpr :=
  .library (.oneEnd .top) (some arrangement) none .bare
/-- "on the bottom of its owner's library" -/
def onBottom : ZoneExpr := .library (.oneEnd .bottom) none none .bare
def onTop : ZoneExpr := .library (.oneEnd .top) none none .bare
/-- "on the top or bottom of your library" -/
def topOrBottom : ZoneExpr := .library (.eitherEnd none) none none .bare
/-- "on the top or bottom of its owner's library, <chooser>'s choice" -/
def choiceOfTopOrBottom (chooser : NounPhrase) : ZoneExpr :=
  .library (.eitherEnd (some chooser)) none none .bare
/-- "Nth from the top or bottom of your library" -/
def nthFromTopOrBottom (ordinal : Ordinal) : ZoneExpr :=
  .library (.eitherEnd none) none (some ordinal) .bare
def handOf (player : NounPhrase) : ZoneExpr := .zone .hand (.possessedBy player)
def graveyardOf (player : NounPhrase) : ZoneExpr := .zone .graveyard (.possessedBy player)
/-- "Nth from the top of its owner's library" -/
def nthFromTop (ordinal : Ordinal) : ZoneExpr := .library (.oneEnd .top) none (some ordinal) .bare

/-! ## Predicates -/

def creature : Predicate := .hasType .creature
def artifact : Predicate := .hasType .artifact
def enchantment : Predicate := .hasType .enchantment
def land : Predicate := .hasType .land
def instant : Predicate := .hasType .instant
def sorcery : Predicate := .hasType .sorcery
def instantOrSorcery : Predicate := .or [instant, sorcery]
def tapped : Predicate := .hasStatus .tapped
def faceDown : Predicate := .hasStatus .faceDown
/-- "another player": any player other than you. -/
def otherPlayer : Predicate := .and [.anyPlayer, .otherThan .you]
/-- "spell": an object on the stack that is not an ability [CR#112.1,113.1]. -/
def spell : Predicate := .and [.inZone stack, .not (.abilityHead .anyOnStack)]
def untapped : Predicate := .hasStatus .untapped
def nontoken : Predicate := .not .isToken
/-- "permanent": an object on the battlefield [CR#110.1]. -/
def permanent : Predicate := .inZone battlefield
/-- "permanent card": a card that could be put onto the battlefield, one with an artifact,
battle, creature, enchantment, land, or planeswalker type [CR#110.4a]; a reading of the card's
types, wherever it is. -/
def permanentCard : Predicate :=
  .and
    [ .isCard,
      .or
        [ .hasType .artifact, .hasType .battle, .hasType .creature, .hasType .enchantment,
          .hasType .land, .hasType .planeswalker ] ]
/-- "colorless": an object of no color [CR#105.2c]. -/
def colorless : Predicate := .colorCount .eq 0
/-- "multicolored": an object of two or more colors [CR#105.2b]. -/
def multicolored : Predicate := .colorCount .atLeast 2
/-- "monocolored" -/
def monocolored : Predicate := .colorCount .eq 1
/-- "historic": an artifact, legendary, or Saga [CR#700.6]. -/
def historic : Predicate :=
  .or [.hasType .artifact, .hasSupertype .legendary, .hasSubtype (.of .enchantment "Saga")]
def attacking : Predicate := .inCombat .attackerOf none
def blocking : Predicate := .inCombat .blockerOf none
def blocked : Predicate := .inCombat .blockedBy none
def unblocked : Predicate := .not blocked
def creatureYouControl : Predicate := .and [creature, .hasPossessor .controller .you]
def creatureYouDontControl : Predicate := .and [creature, .not (.hasPossessor .controller .you)]
def creatureYourOpponentsControl : Predicate :=
  .and [creature, .hasPossessor .controller (.playerGroup .yourOpponents)]
/-- "another creature", anchored on <n>. -/
def otherCreature (n : NounPhrase) : Predicate := .and [creature, .otherThan n]
/-- "another creature you control", anchored on <n>. -/
def otherCreatureYouControl (n : NounPhrase) : Predicate :=
  .and [creature, .hasPossessor .controller .you, .otherThan n]
/-- "source": the object dealing the damage under discussion. -/
def source : Predicate := .isSource
def emblem : Predicate := .isEmblem
/-- "the chosen player" -/
def chosenPlayer : Predicate := .chosenPlayer .theChoice
/-- "the last chosen player" -/
def theLastChosenPlayer : Predicate := .chosenPlayer .theLatestChoice
/-- "of the chosen <quality>" -/
def ofChosen (sort : QualitySort) : Predicate := .ofChosen .theChoice sort
/-- "of the last chosen <quality>" -/
def ofTheLastChosen (sort : QualitySort) : Predicate := .ofChosen .theLatestChoice sort
def copyOfACard : Predicate := .isCopyOfACard
def cardOnTheStack : Predicate := .and [.isCard, spell]
def tokenOnTheBattlefield : Predicate := .and [.isToken, permanent]

/-- The four party roles, in rule order [CR#700.8]. -/
def partyRoles : List Predicate :=
  [ .hasSubtype (.of .creature "Cleric"), .hasSubtype (.of .creature "Rogue"),
    .hasSubtype (.of .creature "Warrior"), .hasSubtype (.of .creature "Wizard") ]

/-- "any target" [CR#115.4]. -/
def anyTarget : Predicate :=
  .or [.hasType .creature, .hasType .planeswalker, .hasType .battle, .anyPlayer]

/-- "cast by <player>" -/
def castBy (player : NounPhrase) : Predicate := .castBy player none
/-- "the Nth spell <player> cast <period>" -/
def nthCastBy (ordinal : Ordinal) (player : NounPhrase) (period : RankPeriod) : Predicate :=
  .castBy player (some (ordinal, period))
/-- "any other target" -/
def anyOtherTarget : Predicate := .and [anyTarget, .other]

/-- "a color", "a creature type": a quality noun. -/
def quality (sort : QualitySort) : Predicate := .qualityNoun sort none
/-- A quality noun with its domain: "a color other than blue". -/
def qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate :=
  .qualityNoun sort (some domain)
/-- "a <dom> with <stat> <r> <bound>", the stat read of the member itself. -/
def comparesOwnStat (stat : Stat) (domain : Predicate) (comparator : Comparator) (bound : Amount) :
    Predicate :=
  .compareOver domain (.statOf (.stat stat) (.pro .bare .one (.top 1))) comparator bound
/-- "the chosen color" -/
def thatColor : ColorTerm := .chosen .theChoice

def creatureType (label : String) : Subtype := .of .creature label
def artifactType (label : String) : Subtype := .of .artifact label
def landType (label : String) : Subtype := .of .land label
def enchantmentType (label : String) : Subtype := .of .enchantment label
def spellType (label : String) : Subtype := .spell label
def planeswalkerType (label : String) : Subtype := .of .planeswalker label
/-- An outlaw: an Assassin, Mercenary, Pirate, Rogue, or Warlock [CR#700.12]. -/
def outlaw : Predicate :=
  .or [ .hasSubtype (creatureType "Assassin"), .hasSubtype (creatureType "Mercenary"),
        .hasSubtype (creatureType "Pirate"), .hasSubtype (creatureType "Rogue"),
        .hasSubtype (creatureType "Warlock") ]
def outlawYouControl : Predicate := .and [outlaw, .hasPossessor .controller .you]
/-- "an Army you control" -/
def armyYouControl : Predicate :=
  .and [.hasSubtype (creatureType "Army"), creature, .hasPossessor .controller .you]

/-! ## Nouns -/

def anOpponent : NounPhrase := a .opponent
def thisCreature : NounPhrase := .asType .creature .this none
def thisArtifact : NounPhrase := .asType .artifact .this none
def thisLand : NounPhrase := .asType .land .this none
def thisPermanent : NounPhrase := .asMarker .permanent .this
def thisSpell : NounPhrase := .asMarker .spell .this
def thisRoom : NounPhrase := .asType .enchantment .this (some (enchantmentType "Room"))
def thisPlaneswalker : NounPhrase := .asType .planeswalker .this none
def thisAbility : NounPhrase := .asMarker .ability .this
def thisEnchantment : NounPhrase := .asType .enchantment .this none
def thisAura : NounPhrase := .asType .enchantment .this (some (enchantmentType "Aura"))
def thisEquipment : NounPhrase := .asType .artifact .this (some (artifactType "Equipment"))
def thisSiege : NounPhrase := .asType .battle .this (some (.of .battle "Siege"))
def thisVehicle : NounPhrase := .asType .artifact .this (some (artifactType "Vehicle"))
def thisSpacecraft : NounPhrase := .asType .artifact .this (some (artifactType "Spacecraft"))
def thisSaga : NounPhrase := .asType .enchantment .this (some (enchantmentType "Saga"))
def thisCase : NounPhrase := .asType .enchantment .this (some (enchantmentType "Case"))
def thisClass : NounPhrase := .asType .enchantment .this (some (enchantmentType "Class"))
/-- "your commander" -/
def yourCommander : NounPhrase := .designated "commander" .you
/-- "a card exiled with this artifact" -/
def exiledWithThisArtifact : Predicate := .exiledWith thisArtifact
/-- "the rest of them" -/
def theRest (kind : Kind) : NounPhrase := .theRest kind .many
/-- "the other pile": the one remaining after a choice among two. -/
def theOther (kind : Kind) : NounPhrase := .theRest kind .one
/-- "N of <group>" -/
def someOf (quantity : Quantity) (group : NounPhrase) : NounPhrase :=
  .someOf (.counted quantity) none group
/-- "among <group>": the whole of a group, sliced. -/
def among (group : NounPhrase) : NounPhrase := .someOf .whole none group
/-- "N <p> from among <group>" -/
def fromAmong (quantity : Quantity) (p : Predicate) (group : NounPhrase) : NounPhrase :=
  .someOf (.counted quantity) (some p) group
/-- "<group> with the same name" -/
def withTheSameName (group : NounPhrase) : NounPhrase := .namesAgree .sameName group
/-- "<group> with different names" -/
def withDifferentNames (group : NounPhrase) : NounPhrase := .namesAgree .differentNames group
/-- "the <p> from among <group>": every member the description picks. -/
def allFromAmong (p : Predicate) (group : NounPhrase) : NounPhrase := .someOf .whole (some p) group
/-- "the <p> among <group>" -/
def allAmong (p : Predicate) (group : NounPhrase) : NounPhrase := allFromAmong p group
/-- "each object" -/
def everyObject : NounPhrase := allOf (.and [])
/-- "one pile": one of the piles just made. -/
def onePile : NounPhrase := .pileOf (.counted (exactly 1)) none
/-- "the pile of <player>'s choice" -/
def pileOfChoice (player : NounPhrase) : NounPhrase := .pileOf (.counted (exactly 1)) (some player)
/-- "you and <subject>" -/
def youAnd (subject : NounPhrase) : NounPhrase := Primitives.NounPhrase.both .you subject
/-- "you or <subject>" -/
def youOr (subject : NounPhrase) : NounPhrase := Primitives.NounPhrase.eitherOf .you subject
def controllerOf (subject : NounPhrase) : NounPhrase := .possessorOf .controller subject
def ownerOf (subject : NounPhrase) : NounPhrase := .possessorOf .owner subject
/-- "the top N cards of your library" -/
def topSlice (amount : Amount) : NounPhrase := .librarySlice .top amount .you
/-- "the bottom card of your library" -/
def bottomCard : NounPhrase := .librarySlice .bottom (.lit 1) .you
/-- The stack after "look at the top N cards of your library". -/
def lookedTop (bs : Bindings) (amount : Amount) : Bindings := nomIntro bs (topSlice amount)
/-- "[a player]'s party" [CR#700.8]. -/
def partyOf (player : NounPhrase) : NounPhrase :=
  .oneEachOf partyRoles (allOf (.and [creature, .hasPossessor .controller player]))
/-- "your party" -/
def party : NounPhrase := partyOf .you
/-- "the number of creatures in <player>'s party" [CR#700.8a]. -/
def partySizeOf (player : NounPhrase) : Amount := .countOf (partyOf player)
def partySize : Amount := partySizeOf .you
/-- "<player> has a full party": four creatures in that party [CR#700.8c]. -/
def fullPartyOf (player : NounPhrase) : Condition := .compareAmt (partySizeOf player) .eq (.lit 4)
def fullParty : Condition := fullPartyOf .you

/-! ## Mana -/

def generic (amount : Nat) : ManaSymbol := .simple (.generic amount)
def pip (color : Color) : ManaSymbol := .simple (.specific (.of color))
/-- "{C}" -/
def colorlessPip : ManaSymbol := .simple (.specific .colorless)
/-- "{A/B}" -/
def hybridPip (left right : Color) : ManaSymbol := .hybrid (.specific (.of left)) right
/-- "{G/P}": a Phyrexian mana symbol [CR#107.4f]. -/
def phyrexianPip (color : Color) : ManaSymbol := .phyrexian color none
/-- "{1} for each …", "{R} for each …": a mana cost scaled by an amount. -/
def scaledMana (unit : ManaUnit) (amount : Amount) : Cost :=
  match unit with
  | .generic => .scaled (.mana [generic 1]) amount
  | .run cost => .scaled (.mana cost) amount

/-! ## Durations -/

def untilEndOfTurn : Duration := .until_ (.endOf .turn none)
def untilYourNextTurn : Duration := .until_ (.startOf .turn (some .you))
def untilEndOfCombat : Duration := .until_ (.endOf .combat none)
def untilYourNextEndStep : Duration := .until_ (.startOf .endStep (some .you))

/-! ## Amounts -/

def lifeTotalOf (player : NounPhrase) : Amount := .statOf (.playerStat .lifeTotal) player
def powerOf (subject : NounPhrase) : Amount := .statOf (.stat .power) subject
def toughnessOf (subject : NounPhrase) : Amount := .statOf (.stat .toughness) subject
def countersOn (kind : CounterKind) (holder : NounPhrase) : Amount := .statOf (.counter kind) holder
def plus (left right : Amount) : Amount := .arith .plus left right
def minus (left right : Amount) : Amount := .arith .minus left right
def times (per amount : Amount) : Amount := .arith .times per amount
/-- "that much damage prevented this way" -/
def preventedThisWay : Amount := .theOutcome .damagePrevented
/-- "N for each <p>" -/
def forEach (per : Nat) (p : Predicate) : Amount := times (.lit per) (countOf p)

/-! ## Counters -/

def plusOnePlusOne : CounterKind := .boost (.up 1) (.up 1)
def flyingCounter : CounterKind := .keyword "Flying"
def minusOneMinusOne : CounterKind := .boost (.down 1) (.down 1)

/-! ## Instructions -/

def move (subject : NounPhrase) (destination : ZoneExpr) : Instruction :=
  .move subject destination []
def destroy (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Destroy") (.move subject graveyard []) (agent := agent)
def exile (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Exile") (.move subject exileZone []) (agent := agent)
/-- "exile <subject> with N <kind> counters on it" -/
def exileWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind)
    (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Exile") (.move subject exileZone [.withCounters amount (.printed kind) .fresh])
      (agent := agent)
def sacrifice (subject : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Sacrifice") (.move subject graveyard []) (agent := some agent)
/-- "<agent> sacrifices it": the permanent slot's occupant. -/
def sacrificeIt (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  sacrifice (.pro (.atSlot .permanent) .one .whole) (agent := agent)
/-- "<agent> puts <subject> <destination>" -/
def put (subject : NounPhrase) (destination : ZoneExpr) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  .enact (.core .put) (.move subject destination []) (agent := some agent)
/-- "return <subject> to <zone>" -/
def returnTo (subject : NounPhrase) (destination : ZoneExpr) (riders : List TokenRider)
    (agent : Option NounPhrase := none) : Instruction :=
  .enact (.core .return_) (.move subject destination riders) (agent := agent)
/-- "return <subject> to the battlefield" -/
def returnToBattlefield (subject : NounPhrase) : Instruction := returnTo subject battlefield []
/-- "return <subject> to the battlefield transformed under <controller>'s control" -/
def returnToBattlefieldTransformed (subject controller : NounPhrase) : Instruction :=
  returnTo subject battlefield [.entersTransformed, .under controller]
/-- "return <subject> to the battlefield under <who>'s control with N <kind> counters on it" -/
def returnToBattlefieldWithCounters (subject who : NounPhrase) (amount : Amount)
    (kind : CounterKind) : Instruction :=
  .move subject battlefield [.under who, .withCounters amount (.printed kind) .fresh]
/-- "transform <subject>" -/
def transform (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Transform") (.turnOver subject) (agent := agent)
/-- "meld <subject> into <name>" -/
def meldInto (subject : NounPhrase) (into : String) (agent : Option NounPhrase := none) :
    Instruction :=
  .enact (.action "Meld") (.move subject battlefield [.entersMelded into]) (agent := agent)
/-- Placement-only fragment of manifest for the anaphora bench [CR#701.40a].
This retains the existing fragment; face-down characteristics and the turn-up special
action are not represented here. -/
def manifestPlacement (subject : NounPhrase) : Instruction :=
  .enact (.action "Manifest") (.move subject battlefield []) (agent := none)
def tap (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Tap") (.setStatus .tapped subject) (agent := agent)
def discard (subject : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Discard") (.move subject graveyard []) (agent := some agent)
def shuffle (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .shuffle (agent := agent)
def untap (subject : NounPhrase) (agent : Option NounPhrase := none) : Instruction :=
  .enact (.action "Untap") (.setStatus .untapped subject) (agent := agent)
/-- "Exile <subject> until <event>." -/
def exileUntil (subject : NounPhrase) (event : GameEvent) : Instruction :=
  .holdUntil (exile subject) event
/-- "<agent> mills <amount> cards" from <whose> library. -/
def mill (amount : Amount) (whose : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Mill") (.move (.librarySlice .top amount whose) graveyard []) (agent := some
      agent)
def putOntoBattlefield (subject : NounPhrase) : Instruction := .move subject battlefield []
def putOntoBattlefieldTapped (subject : NounPhrase) : Instruction :=
  .move subject battlefield [.entersAs .tapped]
/-- "put <subject> onto the battlefield under your control" -/
def putOntoBattlefieldUnderYourControl (subject : NounPhrase) : Instruction :=
  .move subject battlefield [.under .you]
/-- "put <subject> onto the battlefield tapped and attacking" -/
def putOntoBattlefieldTappedAttacking (subject : NounPhrase) : Instruction :=
  .move subject battlefield [.entersAs .tapped, .entersAttacking none]
/-- "Search your library for <quantity> <p>" -/
def searchLibraryFor (quantity : Quantity) (p : Predicate) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  .search (.oneZone yourLibrary) quantity p (agent := agent)
/-- "search <whose>'s graveyard, hand, and library for <q> <p>" -/
def searchZonesOf (whose : NounPhrase) (quantity : Quantity) (p : Predicate)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .search (.someZones (some whose) [.graveyard, .hand, .library]) quantity p (agent := agent)
/-- "search your library and/or graveyard for a <p>" -/
def searchLibraryOrGraveyard (p : Predicate) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .search (.someZones (some .you) [.library, .graveyard]) (exactly 1) p (agent := agent)
/-- "<who> searches their library for a <p>" -/
def searchTheirLibraryFor (p : Predicate) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .search (.oneZone (libraryOf they)) (exactly 1) p (agent := agent)
/-- "<who> reveals their hand" -/
def revealTheirHand (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .expose .reveal (.zone (handOf
    they)) (agent := agent)
def regenerate (subject : NounPhrase) : Instruction := Primitives.Instruction.regenerate subject
def loseLife (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .changeLife (.down amount) (agent := agent)
def gainLife (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .changeLife (.up amount) (agent := agent)
/-- "<player>'s life total becomes <amount>" -/
def setLife (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .changeLife (.set amount) (agent := agent)
def draw (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .draw amount (agent :=
    agent)

def lookAt (cards : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .expose .lookAt
    (.cards cards) (agent := agent)
/-- "look at <player>'s hand" -/
def lookAtHandOf (player : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .expose .lookAt
    (.zone (handOf player)) (agent := agent)
def revealCards (cards : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := .expose .reveal
    (.cards cards) (agent := agent)
/-- "the card found by a search" -/
def foundCard : NounPhrase := itVerbed (.action "Search")
/-- "reveal it": the card a search found. -/
def revealIt : Instruction := revealCards foundCard
def shuffleInto (subject : NounPhrase) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Shuffle") (.move subject ((.library .shuffled none none .bare)) []) (agent := some
      agent)
def doIf (condition : Condition) (instruction : Instruction) : Instruction :=
  .doIf condition instruction none
/-- "choose <subject>" -/
def choose (subject : NounPhrase) (disclosure : Disclosure := .openly)
    (agent : Option NounPhrase := none) : Instruction :=
  .choose none subject disclosure none (agent := agent)
/-- "choose <subject> as you <event>" -/
def chooseWhile (subject : NounPhrase) (while_ : Concurrent) : Instruction :=
  .choose none subject .openly (some while_) (agent := none)
def rollDice (count : Nat) (sides : Nat) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .rollDice (.lit count) (.sides sides) (agent := agent)
/-- One row of a results table: "<results> — <instruction>". -/
def rollRow (results : Quantity) (instruction : Instruction) : RollRow := ⟨results, instruction⟩
def flipCoins (count : Nat) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .flipCoins (.count (.lit count)) (agent := agent)
/-- "<player> flips a coin" as an event -/
def flipsCoin (player : NounPhrase) : GameEvent := .flipsCoin player none
/-- "<decider> may <body>" -/
def offer (body : Instruction) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction := Primitives.Instruction.offer body none none
    (agent := agent)
/-- "<decider> may <body>. When they do, <trigger>": a reflexive trigger on the choice. -/
def offerWhen (body : Instruction) (trigger : Instruction) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  .triggerReflexively (Primitives.Instruction.offer body none none (agent := agent)) trigger
/-- "the chosen number" -/
def chosenNumber : Amount := .chosenNumber .theChoice
/-- "Choose one or more — [cost] — <mode>; …" [CR#702.172a] -/
def chooseSpree (modes : List (Option Cost × Instruction)) : Instruction := .chooseModes (atLeast 1)
    modes
/-- "<voters> vote for <ballot>" -/
def vote (disclosure : Disclosure) (ballot : Ballot) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .vote none disclosure ballot (agent := agent)
/-- "Starting with <first>, <voters> vote for <ballot>" -/
def voteStartingWith (first : NounPhrase) (disclosure : Disclosure) (ballot : Ballot)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .vote (some first) disclosure ballot (agent := agent)
/-- "Choose N — <modes>", no mode costing anything. -/
def chooseModes (quantity : Quantity) (modes : List Instruction) : Instruction :=
  .chooseModes quantity (modes.map (none, ·))
/-- "<source> deals damage equal to its power to <recipient>" -/
def dealDamageOwnPower (source : NounPhrase) (recipient : NounPhrase) :
    Instruction :=
  let own := if source.introducesOwnReferent then
      .pro .bare source.plur (.top 1) else source
  .dealDamage source (.statOf (.stat .power) own) recipient

/-- A token's characteristics from the parts a creature token names. -/
def creatureTokenOf (power toughness : Amount) (colors : List Color) (subtypes : List Subtype) :
    CharacteristicBundle :=
  { characteristics :=
      { colors, types := [.creature], subtypes, power := some power,
        toughness := some toughness } }
def creatureToken (power toughness : Nat) (colors : List Color) (subtypes : List Subtype) :
    CharacteristicBundle :=
  creatureTokenOf (.lit power) (.lit toughness) colors subtypes
/-- "create N <token>" -/
def create (count : Amount) (token : CharacteristicBundle) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  Primitives.Instruction.create count (.written token) [] (agent := agent)
/-- "create N <token> tapped and attacking" -/
def createTappedAttacking (count : Amount) (token : CharacteristicBundle)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  Primitives.Instruction.create count (.written token) [.entersAs .tapped, .entersAttacking none] (agent := agent)

/-- "for each color of mana spent to cast <n>" -/
def colorsSpentToCast (spell : NounPhrase) : Amount := .paid .colorsSpent spell
/-- "if colored mana was spent to cast <spell>" -/
def coloredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .atLeast (.lit 1)
/-- "if no colored mana was spent to cast <spell>" -/
def noColoredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .eq (.lit 0)
/-- "the amount of mana spent to cast <spell>" -/
def manaValueSpentToCast (spell : NounPhrase) : Amount := .paid .manaValueSpent spell
/-- "if no mana was spent to cast <spell>" -/
def noManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (manaValueSpentToCast spell) .eq (.lit 0)

/-- "between N and M" -/
def fromTo (low high : Nat) : Quantity := .range (some low) (some high)
/-- The participant described by the nearest enclosing lookback. -/
def relative (kind : Kind) : NounPhrase := .gap kind
/-- The number of occurrences of a historical event involving the subject. -/
def eventCount (event : GameEvent) (who : NounPhrase) (lookback : Lookback) : Amount :=
  .eventTally .count who (.mk event lookback)
/-- A historical event involving the subject. -/
def happened (event : GameEvent) (who : NounPhrase) (lookback : Lookback) : Condition :=
  .happened who (.mk event lookback)
/-- A historical event whose gap stands for the entity being described. -/
def happenedTo (event : GameEvent) (lookback : Lookback) : Predicate :=
  .happenedTo (.mk event lookback)
/-- The accumulated magnitude of historical events involving the subject. -/
def eventSum (event : GameEvent) (who : NounPhrase) (lookback : Lookback) : Amount :=
  .eventTally .sum who (.mk event lookback)
/-- "if there is no <designation>" -/
def thereIsNo (designation : DesignationLabel) : Condition := .noHolder designation
/-- "<subject>'s <keyword> cost was paid", read back. -/
def paidCostRead (which : PaidCostName) (window : Option Lookback) (subject : NounPhrase) :
    Amount :=
  .paid (.readback which window) subject
/-- "the number of times <subject>'s <keyword> cost was paid" -/
def timesPaid (which : PaidCostName) (subject : NounPhrase) : Amount :=
  .paid (.timesPaid which) subject
/-- "if <subject>'s <cost> was paid" -/
def costWasPaid (which : PaidCostName) (window : Option Lookback) (subject : NounPhrase) :
    Condition :=
  .compareAmt (paidCostRead which window subject) .atLeast (.lit 1)
/-- "the last chosen color" -/
def theLastChosenColor : ColorTerm := .chosen .theLatestChoice
/-- "the last chosen number" -/
def theLastChosenNumber : Amount := .chosenNumber .theLatestChoice
/-- "the amount by which the ceiling was not reached" -/
def shortOfCeiling : Amount := .theOutcome .ceilingShortfall
/-- "the number of counters removed this way" -/
def removedThisWay : Amount := .theOutcome .countersRemoved
/-- "increase or decrease the result by N" -/
def shiftResult (amount : Amount) : Instruction := .shiftResult none amount
/-- "<player> may play N additional lands" -/
def mayPlayAdditionalLands (player : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .deonticRule player .permit [.action "Play"] .agent (some (.additional quantity))
    (.counterpart (allOf land))
    none .noRider
/-- "<player> may <deed> <what> [as though …] [rider]" -/
def mayPlayDeed (deed : Deed) (player what : NounPhrase) (asThough : Option AsThough)
    (rider : DeonticRider) : StaticSpec :=
  .deonticRule player .permit [deed] .agent none (.counterpart what) asThough rider
/-- A deontic clause with no bound, premise, or rider. -/
def deontic (subject : NounPhrase) (compulsion : Compulsion) (deeds : Deeds) (role : Role)
    (patient : DeonticPatient) : StaticSpec :=
  .deonticRule subject compulsion deeds role none patient none .noRider
/-- "When <event>, <instruction>" as a delayed trigger. -/
def delay (event : GameEvent) (instruction : Instruction) : Instruction :=
  .delay event [] none instruction
/-- "When <event> <duration>, <instruction>": a delayed trigger with a window. -/
def delayWithin (event : GameEvent) (duration : Duration) (instruction : Instruction) :
    Instruction :=
  .delay event [] (some duration) instruction
/-- "<subject> phases out until <event>" -/
def phaseOutUntil (subject : NounPhrase) (event : GameEvent) : Instruction :=
  .holdUntil (.setStatus .phasedOut subject) event
/-- "attach <what> to it": the object the sentence just named. -/
def attachToIt (what : NounPhrase) : Instruction :=
  Primitives.Instruction.attachTo what (.pro .bare .one (.outsideIntroduced ((NounPhrase.introduced [] what).map Binding.kind)))
/-- "there is an additional <part> [after <anchor>]" -/
def addPart (part : TurnPart) (anchor : Option TurnPart) (count : Amount) : Instruction :=
  Primitives.Instruction.addPart part anchor count none (agent := none)
/-- "there is an additional <part> after this phase, followed by an additional <next>" -/
def addPartThen (part : TurnPart) (anchor : Option TurnPart) (count : Amount) (next : TurnPart) :
    Instruction :=
  Primitives.Instruction.addPart part anchor count (some next) (agent := none)
/-- "<player> gets an additional <part>" -/
def getAdditionalPart (part : TurnPart) (count : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  Primitives.Instruction.addPart part none count none (agent := some agent)
/-- "<subject> can't attack [this turn]" -/
def forbidAttack (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .establish (.deonticRule subject .forbid [.core .attack] .agent none .noPatient none .noRider)
    duration
/-- "<subject> can't block [this turn]" -/
def forbidBlock (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .establish (.deonticRule subject .forbid [.core .block] .agent none .noPatient none .noRider)
    duration
/-- "<subject> can't be blocked [this turn]" -/
def forbidBeingBlocked (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .establish (.deonticRule subject .forbid [.core .block] .patient none .noPatient none .noRider)
    duration
/-- "<subject> blocks it this turn if able": the object the sentence just named. -/
def requireBlockIt (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .establish
    (.deonticRule subject .require [.core .block] .agent none
      (.counterpart (.pro .bare .one (.outsideIntroduced ((NounPhrase.introduced [] subject).map Binding.kind)))) none
      .noRider)
    duration

/-- "<source> deals N damage divided as you choose among <among>" -/
def dealDivided (source : NounPhrase) (amount : Amount) (among : NounPhrase) : Instruction :=
  .distribute (.damage source) amount among
/-- "distribute N <kind> counters among <among>" -/
def distributeCounters (amount : Amount) (kind : CounterKind) (among : NounPhrase) : Instruction :=
  .distribute (.counters kind) amount among
/-- "remove <q> <kind> counters from <from>" -/
def removeCounters (quantity : Quantity) (kind : Option CounterKindSource) (from_ : NounPhrase) :
    Instruction :=
  .removeCounters (some quantity) kind from_
/-- "<who> loses all [<kind>] counters" -/
def loseAllCounters (kind : Option CounterKindSource) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  Primitives.Instruction.loseCounters kind none (agent := agent)
/-- "remove all [<kind>] counters from <from>" -/
def removeAllCounters (kind : Option CounterKindSource) (from_ : NounPhrase) : Instruction :=
  .removeCounters none kind from_

/-- The agent's number as an agent: "each …" acts one at a time (Idris `agentPlur`). -/
def agentPlur : NounPhrase → Plurality
  | .described .each _ => .one
  | n => n.plur
/-- The agent re-read after its own clause (Idris `agentRef`): an agent that introduces no
binding is re-spelled as itself ("you", "that player"); one that does is read back as the
player it just bound, windowed over the agent's own bindings. -/
def agentRef (agent : NounPhrase) : NounPhrase :=
  match NounPhrase.agentIntroduced [] agent with
  | [] => agent
  | ds => .pro (.word .player) (agentPlur agent) (.introduced (ds.map Binding.kind))

/-- "<player> may pay <cost>. If they don't, <instruction>." -/
def doUnless (instruction : Instruction) (cost : Cost) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  Primitives.Instruction.offer (.pay cost .once (agent := (agentRef agent))) none (some instruction) (agent := agent)

/-- "it" or "them", by number. -/
def itOrThem : Plurality → NounPhrase
  | .one => it
  | .many => them
/-- The window over exactly what a phrase introduced; a phrase that introduced nothing (a
pronoun) is read again through the whole stack. -/
def sameWindow : Bindings → Window
  | [] => .whole
  | bs => .introduced (bs.map Binding.kind)
/-- "it" (or "them"): the subject of a stat change, read back through a window holding only
what that subject and the change's amount announced. -/
def itsOther (subject : NounPhrase) (delta : Delta Amount) : NounPhrase :=
  .pro .bare subject.plur
    (sameWindow
      (Delta.introduced (selfSubjIntro [] subject) delta ++
        NounPhrase.selfSubjIntroduced subject ++ NounPhrase.introduced [] subject))
/-- "<subject> gets +P/+T [until …]": the two stat changes as one static clause; the toughness
half reads its subject back as "it". -/
def getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec :=
  .conjunction none
    [.modification subject .power power, .modification (itsOther subject power) .toughness
        toughness]
/-- "<subject> has base power and toughness P/T" -/
def getsBase (subject : NounPhrase) (power toughness : Amount) : StaticSpec :=
  getsPt subject (.set power) (.set toughness)

/-- The shared subject of a clause, read back as a pronoun that sees only what the subject
itself announced. -/
def ownSubject (subject : NounPhrase) : NounPhrase :=
  .pro .bare subject.plur (.introduced ((selfSubjIntro [] subject).map Binding.kind))
/-- "<subject>'s controller sacrifices it" [CR#701.21a]. The relational subject
introduces the object once; the deed re-reads that introduction. An already-referential
subject introduces no new object and can be read directly again. -/
def controllerSacrifices (subject : NounPhrase) : Instruction :=
  let controller := controllerOf subject
  let patient :=
    if subject.introducesOwnReferent then
      .pro .bare subject.plur (.introduced [.player, subject.kindOr .object])
    else if (selfSubjIntro [] subject).isEmpty then subject
    else ownSubject controller
  sacrifice patient (agent := controller)

/-- "them" (or "it"): the cards a look at a library slice just announced, seen alone. -/
def lookedCards (slice : NounPhrase) : NounPhrase :=
  .pro .bare slice.plur (.introduced ((NounPhrase.introduced [] slice).map Binding.kind))
/-- "<looker> looks at the top N cards of <whose> library, puts any number of them on the bottom
in any order and the rest on top in any order" -/
def lookAndSort (whose : NounPhrase) (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  let slice : NounPhrase := .librarySlice .top amount whose
  .sequentially
    [ .expose .lookAt (.cards slice) (agent := agent),
      move (someOf anyNumber (lookedCards slice)) (onBottomIn .anyOrder),
      move (theRest .object) (onTopIn .anyOrder) ]
/-- The same look, spilling the cards put aside into <spill> instead of the bottom. -/
def lookAndSortInto (whose : NounPhrase) (amount : Amount) (spill : ZoneExpr)
    (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  let slice : NounPhrase := .librarySlice .top amount whose
  .sequentially
    [ .expose .lookAt (.cards slice) (agent := agent),
      move (someOf anyNumber (lookedCards slice)) spill,
      move (theRest .object) (onTopIn .anyOrder) ]
/-- "<agent> scries N" [CR#701.22a] -/
def scry (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Scry") (lookAndSort (agentRef agent) amount (agent := (agentRef agent))) (agent
      := some agent)
/-- "<agent> fateseals N" [CR#701.29a] -/
def fateseal (whose : NounPhrase) (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Fateseal") (lookAndSort whose amount (agent := (agentRef agent))) (agent := some
      agent)
/-- "<agent> surveils N" [CR#701.25a] -/
def surveil (amount : Amount) (agent : NounPhrase := Primitives.NounPhrase.you) : Instruction :=
  .enact (.action "Surveil") (lookAndSortInto (agentRef agent) amount graveyard (agent := (agentRef
      agent))) (agent := some agent)
/-- "Proliferate" with its reminder text [CR#701.34a]: "Choose any number of permanents and/or
players, then give each another counter of each kind already there." -/
def proliferate : Instruction :=
  .enact (.action "Proliferate") (.sequentially
      [ .choose none (counted anyNumber
            (.or [ .and [permanent, .hasCounters none],
                   .compare [.anyCounter .player] .atLeast (.lit 1) ])) .openly none (agent :=
                       none),
        .putCounters (.lit 1) .own (.eachOf (those .join)) ]) (agent := none)
/-- "amass <subtype> N" with its reminder text [CR#701.47a]: "If you don't control an Army,
create a 0/0 black <subtype> Army creature token. Choose an Army you control. Put N +1/+1
counters on it. It's a <subtype> in addition to its other types." -/
def amass (subtype : String) (count : Nat) : Instruction :=
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
def makeMonstrous (amount : Amount) : Instruction :=
  .doIf (.not (.matches thisPermanent (.hasDesignation "monstrous" none)))
    (.sequentially
      [ .putCounters amount (.printed plusOnePlusOne) thisPermanent,
        .gainDesignation thisPermanent "monstrous" (.byDeed (.action "Monstrosity")) none ])
    none
def get (subject : NounPhrase) (power toughness : Delta Amount) (duration : Option Duration) :
    Instruction :=
  .establish (getsPt subject power toughness) duration

def gain (subject : NounPhrase) (ability : Ability) (duration : Option Duration) : Instruction :=
  .establish (.abilityGrant subject ability) duration
/-- "<subject> gains haste [until …]" -/
def gainHaste (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  gain subject (.keyword "Haste" [] none) duration
/-- "<subject> becomes <added> in addition to its other types [until …]" -/
def become (subject : NounPhrase) (added : CharacteristicBundle) (duration : Option Duration) :
    Instruction :=
  .establish (Primitives.StaticSpec.qualityChange subject .adds (.bundle added none)) duration
/-- "<subject> becomes <colors> [until …]" -/
def becomeColor (subject : NounPhrase) (colors : ColorSpec) (duration : Option Duration) :
    Instruction :=
  .establish (Primitives.StaticSpec.qualityChange subject .sets (.colored colors)) duration
/-- Several static clauses sharing one subject, as one instruction. -/
def establishFor (subject : NounPhrase) (parts : List StaticSpec) (duration : Option Duration) :
    Instruction :=
  .establish (.conjunction (some subject) parts) duration
/-- "If <event> would happen, <replacement> instead [duration]." -/
def replaceEvent (event : GameEvent) (replacement : Instruction) (duration : Option Duration) :
    Instruction :=
  .establish (.replacement event [] none replacement .repeatedly none) duration
/-- "Prevent all <kind> damage that would be dealt <scope> [duration]." -/
def preventAll (kind : DamageKind) (scope : DamageScope) (duration : Option Duration) :
    Instruction :=
  .establish (.damageRule kind .unattributed scope (.prevent .all none) .repeatedly) duration
/-- "Prevent the next N <kind> damage that would be dealt <scope> [duration]." -/
def preventNext (kind : DamageKind) (scope : DamageScope) (amount : Amount)
    (duration : Option Duration) : Instruction :=
  .establish (.damageRule kind .unattributed scope (.prevent (.shield amount) none) .repeatedly)
    duration
/-- "Prevent all <kind> damage that would be dealt by <source> <scope> [duration]." -/
def preventAllBy (kind : DamageKind) (source : NounPhrase) (scope : DamageScope)
    (duration : Option Duration) : Instruction :=
  .establish (.damageRule kind (.dealtBy source) scope (.prevent .all none) .repeatedly) duration
/-- "The next time <event> would happen, <replacement> instead [duration]." -/
def replaceNextEvent (event : GameEvent) (replacement : Instruction)
    (duration : Option Duration) : Instruction :=
  .establish (.replacement event [] none replacement .nextTimeOnly none) duration
/-- "<player> gains control of <subject> [duration]" -/
def gainControl (subject : NounPhrase) (duration : Option Duration) (agent : NounPhrase := Primitives.NounPhrase.you) :
    Instruction :=
  .establish (.controlGrant agent subject) duration
/-- "<subject> can't be <deed>ed" -/
def objectCant (deed : Deed) (subject : NounPhrase) : StaticSpec :=
  .deonticRule subject .forbid [deed] .patient none .noPatient none .noRider
/-- "<player> can't <deed>" -/
def playerCant (deed : Deed) (player : NounPhrase) : StaticSpec :=
  .deonticRule player .forbid [deed] .agent none .noPatient none .noRider
/-- "<spec> as long as <condition>" -/
def onlyWhile (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .asLongAs
/-- "<spec> unless <condition>" -/
def onlyUnless (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec (.not condition) .unless_
/-- "<spec> if <condition>" -/
def onlyIfSo (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditional spec condition .ifSo
/-- "<who> can't <deed> <what>" -/
def cantDoTo (deed : Deed) (who what : NounPhrase) : StaticSpec :=
  .deonticRule who .forbid [deed] .agent none (.counterpart what) none .noRider
/-- "<n> can <deed> as though it didn't have <p>" -/
def canDoAsThough (n : NounPhrase) (deed : Deed) (p : Predicate) : StaticSpec :=
  .deonticRule n .permit [deed] .agent none .noPatient (some (.of p)) .noRider
/-- "<what> can't be the target of <by>" -/
def cantBeTargetedBy (what by_ : NounPhrase) : StaticSpec :=
  .deonticRule what .forbid [.core .target] .patient none (.targetedBy by_) none .noRider
/-- "<what> can be the target of <by> as though it didn't have <p>" -/
def canBeTargetedAsThough (what by_ : NounPhrase) (p : Predicate) : StaticSpec :=
  .deonticRule what .permit [.core .target] .patient none (.targetedBy by_) (some (.of p)) .noRider
/-- "<spec> <duration>": a clause holding for a stated duration. -/
def establishThroughout (spec : StaticSpec) (duration : Duration) : Instruction :=
  .establish spec (some duration)
/-- "<n> doesn't untap during [<whose>] untap step" -/
def doesntUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose (objectCant (.action "Untap") subject)
/-- "you may choose not to untap <subject> during [<whose>] untap step" -/
def mayDeclineUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.deonticRule subject .permit [.action "Untap"] .patient none .noPatient none .noRider)
/-- "untap <subject> during [<whose>] untap step" -/
def untapsDuring (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  Primitives.StaticSpec.partScope .untapStep whose
    (.deonticRule subject .require [.action "Untap"] .patient none .noPatient none .noRider)
/-- "<subject> can block an additional creature each combat" -/
def mayBlockAdditional (subject : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .deonticRule subject .permit [.core .block] .agent (some (.additional quantity))
    (.counterpart (allOf creature)) none .noRider
/-- "<who> may vote an additional time" -/
def mayVoteAdditional (who : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .deonticRule who .permit [.action "Vote"] .agent (some (.additional quantity)) .noPatient none
    .noRider
/-- "<who> may spend mana as though it were mana of <as> [to <purpose>]" -/
def maySpendAsThough (who : NounPhrase) (what : Option ColorOrColorless) (as_ : ManaMatch)
    (purpose : Option SpendPurpose) : StaticSpec :=
  .deonticRule who .permit [.core .spend] .agent none .noPatient (some (.mana what as_ purpose))
    .noRider
/-- "<player> can't <deed> more than N <p>" -/
def cantMoreThan (player : NounPhrase) (deed : Deed) (bound : Nat) (p : Predicate) :
    StaticSpec :=
  .deonticRule player .forbid [deed] .agent (some (.moreThan (.lit bound))) (.counterpart (allOf p))
    none .noRider

/-! ## Events -/

def leavesBattlefield (subject : NounPhrase) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (.zones [battlefield]))
/-- "<subject> leaves <zone>" -/
def leavesZone (subject : NounPhrase) (zone : ZoneExpr) : GameEvent :=
  Primitives.GameEvent.leaves subject (some (.zones [zone]))
/-- "at the beginning of <possessor>'s <part>" -/
def beginningOfPossessed (quantifier : PartQuant) (part : TurnPart) (possessor : NounPhrase) :
    GameEvent :=
  .beginningOf quantifier part (.byPlayer possessor)
/-- "that turn's": the extra turn just granted, as a header possessor. -/
def thatTurns : HeaderPossessor := .byTurn thatTurn
def dealsCombatDamage (source : NounPhrase) (patient : NounPhrase) : GameEvent :=
  Primitives.GameEvent.dealsDamage .combatOnly source (some patient)
def attacks (subject : NounPhrase) : GameEvent := .combat .attackerOf subject none
/-- "<subject> attacks <whom>" -/
def attacksPlayer (subject whom : NounPhrase) : GameEvent :=
  .combat .attackerOf subject (some whom)
/-- "<n> is put into <zone> from <source>" -/
def putIntoFrom (subject : NounPhrase) (destination : ZoneExpr) (source : EventSource) : GameEvent :=
  Primitives.GameEvent.putInto subject destination (some source)
/-- "N <kind> counters are put on / removed from <subject>" -/
def counterEvent (move : CounterMove) (kind : CounterKind) (batch : CounterBatch)
    (subject : NounPhrase) : GameEvent :=
  .counterEvent move (some kind) subject batch none false
/-- "counters are put on / removed from <subject>", the kind unsaid. -/
def bareCounterEvent (move : CounterMove) (batch : CounterBatch) (subject : NounPhrase) :
    GameEvent :=
  .counterEvent move none subject batch none false
/-- "the last <kind> counter is removed from <subject> by <who>" -/
def lastCounterRemovedBy (kind : CounterKind) (subject who : NounPhrase) : GameEvent :=
  .counterEvent .removed (some kind) subject .emptying (some who) false
/-- "one or more counters are put on <subject> by an effect" -/
def manyCountersPutByEffect (subject : NounPhrase) : GameEvent :=
  .counterEvent .put none subject .many none true
/-- "<who> puts one or more counters on <subject>" -/
def manyBareCountersPutBy (who subject : NounPhrase) : GameEvent :=
  .counterEvent .put none subject .many (some who) false
/-- "one or more tokens would be created" -/
def tokensCreated (tokens : NounPhrase) : GameEvent := .tokensCreated tokens false none none
/-- "you roll a die and the result is <q>" -/
def youRollResultIn (quantity : Quantity) : GameEvent :=
  .rollsDice .you .one none (.resultIn quantity)
/-- "you roll a die and the natural result is the highest" -/
def youRollHighestNatural : GameEvent := .rollsDice .you .one none .highestNatural
/-- "one or more tokens would be created under <under>'s control by an effect" -/
def tokensCreatedByEffectUnder (tokens under : NounPhrase) : GameEvent :=
  .tokensCreated tokens true none (some under)
def blocks (subject : NounPhrase) (blocked : Option NounPhrase) : GameEvent :=
  .combat .blockerOf subject blocked
def becomesBlocked (subject : NounPhrase) (by_ : Option NounPhrase) : GameEvent :=
  .combat .blockedBy subject by_
/-- "the last <kind> counter is removed from <subject>" -/
def lastCounterRemoved (kind : CounterKind) (subject : NounPhrase) : GameEvent :=
  .counterEvent .removed (some kind) subject .emptying none false
/-- "<subject> regenerates": the verb as an event. -/
def regenerates (subject : NounPhrase) : GameEvent :=
  .verbedEvent none (.action "Regenerate") (some subject) none none

/-- Copy exceptions retain their own edit block; added abilities remain copy-specific. -/
def copyCharacteristics (c : Characteristics) (typesAdded : Bool) : List CopyExcept :=
  let typeOp := if typesAdded then QualityOp.adds else .sets
  let types := if c.supertypes.isEmpty && c.types.isEmpty && c.subtypes.isEmpty then []
    else [.typeLine typeOp c.typeChanges]
  let values := ({ c with text := [] } : Characteristics).valueEdits .sets
  let edits := types ++ values
  (if edits.isEmpty then [] else [.edits edits]) ++ c.text.map CopyExcept.ability

/-! ## Abilities -/

def keyword (label : KeywordLabel) : Ability := .keyword label [] none
/-- An ability word in italics before an ability: "Will of the council — …" [CR#207.2c]. -/
def abilityWord (word : AbilityWordLabel) (ability : Ability) : Ability :=
  .italicHead (.abilityWord word) ability
/-- A flavor word in italics before an ability [CR#207.2d]. -/
def flavorWord (word : FlavorWordLabel) (ability : Ability) : Ability :=
  .italicHead (.flavorWord word) ability
/-- "Companion — <condition>" -/
def companion (condition : DeckCondition) : Ability :=
  .keyword "Companion" [.deckCondition condition] none
/-- "Pay N life" as a cost. -/
def payLife (player : NounPhrase) (amount : Nat) : Cost :=
  .perform (.changeLife (.down (.lit amount)) (agent := player))
/-- "<keyword> <cost>" -/
def keywordCosting (label : KeywordLabel) (cost : Cost) : Ability :=
  .keyword label [.cost cost] none
/-- "<keyword> <quality>", e.g. "protection from red" -/
def keywordQuality (label : KeywordLabel) (quality : Predicate) : Ability :=
  .keyword label [.quality quality] none
/-- "<keyword> <subject>", e.g. "enchant creature" -/
def keywordSubject (label : KeywordLabel) (subject : Predicate) : Ability :=
  .keyword label [.subject subject] none
/-- "<keyword> N", e.g. "bushido 2" -/
def keywordNumber (label : KeywordLabel) (amount : Amount) : Ability :=
  .keyword label [.number amount] none
/-- "<keyword> <quality> <cost>", e.g. "plainscycling {2}" [CR#702.29e] -/
def keywordQualityCosting (label : KeywordLabel) (quality : Predicate) (cost : Cost) : Ability :=
  .keyword label [.quality quality, .cost cost] none
/-- "<keyword> N—<cost>", e.g. "suspend 4—{1}{U}" -/
def keywordNumberCosting (label : KeywordLabel) (amount : Amount) (cost : Cost) : Ability :=
  .keyword label [.number amount, .cost cost] none
/-- "Level up [cost]" [CR#702.87a] -/
def levelUp (cost : Cost) : Ability := keywordCosting "LevelUp" cost
/-- "When <event>, if <condition>, <instruction>" -/
def triggeredIf (event : GameEvent) (condition : Condition)
    (instruction : Instruction) : Ability :=
  .triggered event [] none [] none none (some condition) instruction
/-- "Whenever <event>, <instruction>. This ability triggers only once each turn." -/
def triggeredOnlyOnce (event : GameEvent) (limit : UsageLimit) (instruction : Instruction) :
    Ability :=
  .triggered event [] none [] none (some limit) none instruction
/-- "if it isn't <p>", read of the ability just named. -/
def itIsntAnAbility (p : Predicate) : Condition := .not (.matches (that .ability) p)
/-- "As <subject> enters, choose a <quality>." -/
def entersChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  Primitives.StaticSpec.entryChoice subject (.quality sort) none .openly
/-- "As <subject> enters, choose a <quality> from <domain>." -/
def entersChoosingFrom (subject : NounPhrase) (sort : QualitySort) (domain : ChoiceDomain) :
    StaticSpec :=
  Primitives.StaticSpec.entryChoice subject (.quality sort) (some domain) .openly
/-- "As <subject> enters, choose a player [from <domain>]." -/
def entersChoosingPlayer (subject : NounPhrase) (domain : Option ChoiceDomain) : StaticSpec :=
  Primitives.StaticSpec.entryChoice subject .player domain .openly
/-- "As <subject> enters, secretly choose a player [from <domain>]." -/
def entersChoosingPlayerSecretly (subject : NounPhrase) (domain : Option ChoiceDomain) :
    StaticSpec :=
  Primitives.StaticSpec.entryChoice subject .player domain .secretly
/-- "<subject> enters tapped" -/
def entersTapped (subject : NounPhrase) : StaticSpec := .entryRider subject (.entersAs .tapped)
/-- "As <subject> becomes attached, choose a <quality>." -/
def attachChoosing (subject : NounPhrase) (sort : QualitySort) : StaticSpec :=
  Primitives.StaticSpec.attachmentChoice subject (.quality sort) none
/-- "<subject> enters with N <kind> counters on it" -/
def entersWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .fresh)
/-- "<subject> enters with an additional N <kind> counters on it" -/
def entersWithAdditionalCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .additional)
/-- "<subject> enters with N fewer <kind> counters on it" -/
def entersWithFewerCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entryRider subject (.withCounters amount (.printed kind) .fewer)

def triggered (event : GameEvent) (instruction : Instruction) : Ability :=
  .triggered event [] none [] none none none instruction
/-- "Whenever <event> and whenever <joined events>, <instruction>" -/
def triggeredJoined (event : GameEvent) (joins : List JoinedHeader) (instruction : Instruction) :
    Ability :=
  .triggered event [] none joins none none none instruction
/-- A further "whenever <event>" header joined onto a trigger. -/
def joinedHead (event : GameEvent) : JoinedHeader := ⟨event, [], none, none⟩
/-- "Whenever <event> or <alternatives>, <instruction>" -/
def triggeredOr (event : GameEvent) (alternatives : List GameEvent) (instruction : Instruction) :
    Ability :=
  .triggered event alternatives none [] none none none instruction
/-- "Whenever <event> while <concurrent>, <instruction>" -/
def triggeredWhile (event : GameEvent) (while_ : Concurrent) (instruction : Instruction) :
    Ability :=
  .triggered event [] (some while_) [] none none none instruction
/-- "Whenever <event> during <timing>, <instruction>" -/
def triggeredOnlyDuring (event : GameEvent) (timing : Timing) (instruction : Instruction) :
    Ability :=
  .triggered event [] none [] (some timing) none none instruction
/-- A joined "whenever <event> while <concurrent>" header. -/
def joinedHeadWhile (event : GameEvent) (while_ : Concurrent) : JoinedHeader :=
  ⟨event, [], some while_, none⟩
/-- "When <event>, <instruction>" -/
def when (event : GameEvent) (instruction : Instruction) : Ability := triggered event instruction
/-- "Whenever <event>, <instruction>" -/
def whenever (event : GameEvent) (instruction : Instruction) : Ability :=
  triggered event instruction
/-- "At <event>, <instruction>" (`at` is a Lean keyword). -/
def at_ (event : GameEvent) (instruction : Instruction) : Ability := triggered event instruction
/-- "After <event>, <instruction>": the dice template's word. -/
def after (event : GameEvent) (instruction : Instruction) : Ability := triggered event instruction

/-- Renown N's reminder text: "When this creature deals combat damage to a player, if it isn't
renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
def renownExpansion (count : Nat) : Ability :=
  triggeredIf (dealsCombatDamage thisCreature (a .anyPlayer))
    (.not (.matches thisCreature (.hasDesignation "renowned" none)))
    (.sequentially
      [ .putCounters (.lit count) (.printed plusOnePlusOne) thisCreature,
        .gainDesignation thisCreature "renowned" (.byKeyword "Renown") none ])
/-- Storm's reminder text: "When you cast this spell, copy it for each other spell that was cast
before it this turn. You may choose new targets for the copies." [CR#702.40a] -/
def stormExpansion : Ability :=
  when (.casts .you (some thisSpell) none)
    (.sequentially
      [ .copy .fromStack thisSpell (eventCount (.casts (relative .player) (some (a (.and [spell,
        .otherThan thisSpell]))) none) (a .anyPlayer) .earlierThisTurn) [] (agent := .you),
        offer (.chooseNewTargets (.pro (.word .copy) .many .whole)) (agent := .you) ])
/-- "Storm" with its reminder text. -/
def storm : Ability := .keyword "Storm" [] (some stormExpansion)
/-- "Renown N" with its reminder text. -/
def renown (count : Nat) : Ability :=
  .keyword "Renown" [.number (.lit count)] (some (renownExpansion count))
/-- Cumulative upkeep's reminder text: "At the beginning of your upkeep, if this permanent is
on the battlefield, put an age counter on it. Then you may pay [cost] for each age counter on
it. If you don't, sacrifice it." [CR#702.24a] -/
def cumulativeUpkeepExpansion (cost : Cost) : Ability :=
  triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
    (.matches thisPermanent (.inZone battlefield))
    (.sequentially
      [ .putCounters (.lit 1) (.printed (.named "Age")) thisPermanent,
        Primitives.Instruction.offer (.pay (.scaled cost (times (.lit 1) (countersOn (.named "Age") thisPermanent))) .once
            (agent := .you)) none (some (sacrifice thisPermanent (agent := .you))) (agent := .you)
            ])
/-- "Cumulative upkeep [cost]" with its reminder text. -/
def cumulativeUpkeep (cost : Cost) : Ability :=
  .keyword "CumulativeUpkeep" [.cost cost] (some (cumulativeUpkeepExpansion cost))
def activated (cost : Cost) (instruction : Instruction) : Ability :=
  .activated cost instruction none none none none
/-- "[cost]: <instruction>. Activate only <timing>." -/
def activatedOnlyDuring (cost : Cost) (instruction : Instruction) (timing : Timing) : Ability :=
  .activated cost instruction (some timing) none none none
/-- "[cost]: <instruction>. Only <who> may activate this ability." -/
def activatedBy (cost : Cost) (instruction : Instruction) (who : NounPhrase) : Ability :=
  .activated cost instruction none none none (some who)
/-- "[cost]: <instruction>. Activate only once <limit>." -/
def activatedOnlyOnce (cost : Cost) (instruction : Instruction) (limit : UsageLimit) : Ability :=
  .activated cost instruction none (some limit) none none
/-- "[cost]: <instruction>. Activate only if <guard>." -/
def activatedOnlyIf (cost : Cost) (instruction : Instruction) (guard : Condition) : Ability :=
  .activated cost instruction none none (some guard) none
/-- "[cost]: <instruction>. Activate only once <limit> and only if <guard>." -/
def activatedOnlyOnceIf (cost : Cost) (instruction : Instruction) (limit : UsageLimit)
    (guard : Condition) : Ability :=
  .activated cost instruction none (some limit) (some guard) none

/-! ## Cards -/

/-- A printed power, toughness, loyalty, or defense. -/
def stat (value : Nat) : Option Amount := some (.lit value)
/-- One level band of a leveler: its range, power/toughness box, and text [CR#711.2a]. -/
def levelBand (range : LevelRange) (power toughness : Nat) (text : List Ability) : LevelBand :=
  { range, text, power := stat power, toughness := stat toughness }
/-- A prototype frame's inset set: its mana cost and power/toughness box [CR#718.1]. -/
def prototypeAlt (cost : ManaCost) (power toughness : Nat) : PrototypeFrame :=
  { cost := some cost, power := stat power, toughness := stat toughness }

register_semantic_macros

end Semantics.Macros
