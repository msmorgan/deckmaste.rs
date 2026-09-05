import Semantics
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

/-- "it", stamped by the verb that produced it: "the exiled card". -/
def itVerbed (verb : Deed) : NounPhrase := .pro (.stamped verb) .one .whole
/-- "them", stamped by the verb that produced them: "the destroyed creatures". -/
def themVerbed (verb : Deed) : NounPhrase := .pro (.stamped verb) .many .whole
/-- "that turn" -/
def thatTurn : NounPhrase := .pro .thatTurn .one .whole
/-- "it", read as the subject of the condition just stated. -/
def itCondSubject (bs : Bindings) (condition : Condition) : NounPhrase :=
  .pro .bare .one (.top (Condition.introduced bs condition).length)

/-- "that <word>", e.g. `that .player`. -/
def that (w : NounWord) : NounPhrase := .pro (.word w) .one .whole

/-- "those <word>s", e.g. `those .card`. -/
def those (w : NounWord) : NounPhrase := .pro (.word w) .many .whole

/-- "they": the player most recently named. -/
def they : NounPhrase := .pro (.word .player) .one .whole
/-- "that player or that permanent's controller": the player half of a "target player or
planeswalker" split, or the controller of the permanent half. -/
def splitOverPlaneswalker : NounPhrase :=
  .eitherOf (.pro (.unionHalf .player) .one .whole)
    (.possessorOf .controller (.pro (.unionHalf (.type .planeswalker)) .one .whole))
/-- The same over "any target": the player, or the permanent's controller. -/
def splitOverPermanent : NounPhrase :=
  .eitherOf (.pro (.unionHalf .player) .one .whole)
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
/-- "any other target" -/
def anyOtherTarget : Predicate := .and [anyTarget, .other]

/-- "a color", "a creature type": a quality noun. -/
def quality (sort : QualitySort) : Predicate := .qualityNoun sort none
/-- A quality noun with its domain: "a color other than blue". -/
def qualityFrom (sort : QualitySort) (domain : ChoiceDomain) : Predicate :=
  .qualityNoun sort (some domain)
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
def youAnd (subject : NounPhrase) : NounPhrase := .both .you subject
/-- "you or <subject>" -/
def youOr (subject : NounPhrase) : NounPhrase := .eitherOf .you subject
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
/-- "{A/B}" -/
def hybridPip (left right : Color) : ManaSymbol := .hybrid (.specific (.of left)) right
/-- "{1} for each …", "{R} for each …": a mana cost scaled by an amount. -/
def scaledMana (unit : ManaUnit) (amount : Amount) : Cost :=
  match unit with
  | .generic => .scaled (.mana [generic 1]) amount
  | .run cost => .scaled (.mana cost) amount

/-! ## Durations -/

def untilEndOfTurn : Duration := .until_ (.endOf .turn none)
def untilYourNextTurn : Duration := .until_ (.startOf .turn (some .you))
def untilEndOfCombat : Duration := .until_ (.endOf .combat none)

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

/-! ## Instructions -/

def move (subject : NounPhrase) (destination : ZoneExpr) : Instruction :=
  .move subject destination []
def destroy (subject : NounPhrase) : Instruction :=
  .enact none (.action "Destroy") (.move subject graveyard [])
def exile (subject : NounPhrase) : Instruction :=
  .enact none (.action "Exile") (.move subject exileZone [])
def sacrifice (agent : NounPhrase) (subject : NounPhrase) : Instruction :=
  .enact (some agent) (.action "Sacrifice") (.move subject graveyard [])
/-- "<agent> sacrifices it": the permanent slot's occupant. -/
def sacrificeIt (agent : NounPhrase) : Instruction :=
  sacrifice agent (.pro (.atSlot .permanent) .one .whole)
/-- "<agent> puts <subject> <destination>" -/
def puts (agent : NounPhrase) (subject : NounPhrase) (destination : ZoneExpr) : Instruction :=
  .enact (some agent) (.core .put) (.move subject destination [])
/-- "return <subject> to <zone>" -/
def returnTo (subject : NounPhrase) (destination : ZoneExpr) (riders : List TokenRider) :
    Instruction :=
  .enact none (.core .return_) (.move subject destination riders)
/-- "return <subject> to the battlefield" -/
def returnToBattlefield (subject : NounPhrase) : Instruction := returnTo subject battlefield []
/-- "return <subject> to the battlefield transformed under <controller>'s control" -/
def returnToBattlefieldTransformed (subject controller : NounPhrase) : Instruction :=
  returnTo subject battlefield [.entersTransformed, .under controller]
/-- "transform <subject>" -/
def transform (subject : NounPhrase) : Instruction :=
  .enact none (.action "Transform") (.turnOver subject)
/-- "meld <subject> into <name>" -/
def meldInto (subject : NounPhrase) (into : String) : Instruction :=
  .enact none (.action "Meld") (.move subject battlefield [.entersMelded into])
def tap (subject : NounPhrase) : Instruction :=
  .enact none (.action "Tap") (.setStatus .tapped subject)
def discard (agent : NounPhrase) (subject : NounPhrase) : Instruction :=
  .enact (some agent) (.action "Discard") (.move subject graveyard [])
def shuffle : Instruction := .shuffle .you
def untap (subject : NounPhrase) : Instruction :=
  .enact none (.action "Untap") (.setStatus .untapped subject)
/-- "<agent> exiles <subject>" -/
def exiles (agent : NounPhrase) (subject : NounPhrase) : Instruction :=
  .enact (some agent) (.action "Exile") (.move subject exileZone [])
/-- "Exile <subject> until <event>." -/
def exileUntil (subject : NounPhrase) (event : GameEvent) : Instruction :=
  .heldUntil (exile subject) event
/-- "<agent> mills <amount> cards" from <whose> library. -/
def mills (agent : NounPhrase) (amount : Amount) (whose : NounPhrase) : Instruction :=
  .enact (some agent) (.action "Mill") (.move (.librarySlice .top amount whose) graveyard [])
def putOntoBattlefield (subject : NounPhrase) : Instruction := .move subject battlefield []
def putOntoBattlefieldTapped (subject : NounPhrase) : Instruction :=
  .move subject battlefield [.entersAs .tapped]
/-- "put <subject> onto the battlefield under your control" -/
def putOntoBattlefieldUnderYourControl (subject : NounPhrase) : Instruction :=
  .move subject battlefield [.under .you]
/-- "Search your library for <quantity> <p>" -/
def searchLibraryFor (quantity : Quantity) (p : Predicate) : Instruction :=
  .search .you (.oneZone yourLibrary) quantity p
/-- "search <whose>'s graveyard, hand, and library for <q> <p>" -/
def searchZonesOf (whose : NounPhrase) (quantity : Quantity) (p : Predicate) : Instruction :=
  .search .you (.someZones (some whose) [.graveyard, .hand, .library]) quantity p
def regenerate (subject : NounPhrase) : Instruction := .regenerate subject
def losesLife (player : NounPhrase) (amount : Amount) : Instruction :=
  .changeLife player (.down amount)
def gainsLife (player : NounPhrase) (amount : Amount) : Instruction :=
  .changeLife player (.up amount)
/-- "<player>'s life total becomes <amount>" -/
def lifeBecomes (player : NounPhrase) (amount : Amount) : Instruction :=
  .changeLife player (.set amount)
def draw (player : NounPhrase) (amount : Amount) : Instruction := .draw player amount

def lookAt (cards : NounPhrase) : Instruction := .expose .lookAt .you (.cards cards)
def revealCards (cards : NounPhrase) : Instruction := .expose .reveal .you (.cards cards)
/-- "the card found by a search" -/
def foundCard : NounPhrase := itVerbed (.action "Search")
/-- "reveal it": the card a search found. -/
def revealsIt : Instruction := revealCards foundCard
def shuffleInto (agent : NounPhrase) (subject : NounPhrase) : Instruction :=
  .enact (some agent) (.action "Shuffle") (.move subject (.library .shuffled none none .bare) [])
def if_ (condition : Condition) (instruction : Instruction) : Instruction :=
  .if_ condition instruction none
/-- "choose <subject>" -/
def choose (subject : NounPhrase) : Instruction := .choose none none subject .openly none
/-- "<player> chooses <subject>" -/
def chooses (player : NounPhrase) (subject : NounPhrase) : Instruction :=
  .choose none (some player) subject .openly none
/-- "<player> secretly chooses <subject>" -/
def secretlyChooses (player : NounPhrase) (subject : NounPhrase) : Instruction :=
  .choose none (some player) subject .secretly none
/-- "choose <subject> as you <event>" -/
def chooseWhile (subject : NounPhrase) (while_ : Concurrent) : Instruction :=
  .choose none none subject .openly (some while_)
def rollDice (player : NounPhrase) (count sides : Nat) : Instruction :=
  .rollDice player (.lit count) (.sides sides)
/-- One row of a results table: "<results> — <instruction>". -/
def rollRow (results : Quantity) (instruction : Instruction) : RollRow := ⟨results, instruction⟩
def flipCoins (player : NounPhrase) (count : Nat) : Instruction :=
  .flipCoins player (.count (.lit count))
/-- "<player> flips a coin" as an event -/
def flipsCoin (player : NounPhrase) : GameEvent := .flipsCoin player none
/-- "<decider> may <body>" -/
def may (decider : NounPhrase) (body : Instruction) : Instruction := .may decider body none none
/-- "the chosen number" -/
def chosenNumber : Amount := .chosenNumber .theChoice
/-- "Choose one or more — [cost] — <mode>; …" [CR#702.172a] -/
def spree (modes : List (Option Cost × Instruction)) : Instruction := .modal (atLeast 1) modes
/-- "<voters> vote for <ballot>" -/
def vote (voters : NounPhrase) (disclosure : Disclosure) (ballot : Ballot) : Instruction :=
  .vote none voters disclosure ballot
/-- "Starting with <first>, <voters> vote for <ballot>" -/
def voteStartingWith (first voters : NounPhrase) (disclosure : Disclosure) (ballot : Ballot) :
    Instruction :=
  .vote (some first) voters disclosure ballot
/-- "Choose N — <modes>", no mode costing anything. -/
def chooseModes (quantity : Quantity) (modes : List Instruction) : Instruction :=
  .modal quantity (modes.map (none, ·))
/-- "<source> deals damage equal to its power to <recipient>" -/
def dealsDamageOwnPower (bs : Bindings) (source : NounPhrase) (recipient : NounPhrase) :
    Instruction :=
  .dealDamage source
    (.statOf (.stat .power)
      (.pro .bare .one
        (.top (NounPhrase.selfSubjIntroduced source ++ NounPhrase.introduced bs source).length)))
    recipient

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
def create (count : Amount) (token : CharacteristicBundle) : Instruction :=
  .create .you count (.written token) []

/-- "for each color of mana spent to cast <n>" -/
def colorsSpentToCast (spell : NounPhrase) : Amount := .paid .colorsSpent spell
/-- "if colored mana was spent to cast <spell>" -/
def coloredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .atLeast (.lit 1)

/-- "between N and M" -/
def fromTo (low high : Nat) : Quantity := .range (some low) (some high)
/-- "the number of <event>s <who> <lookback> involving <what>" -/
def eventCountInvolving (event : EventName) (who : NounPhrase) (lookback : Lookback)
    (what : NounPhrase) : Amount :=
  .eventTally .count who (.mk event lookback (some (.involving what)))
/-- "the number of times <event> happened to <who> <lookback>" -/
def eventCount (event : EventName) (who : NounPhrase) (lookback : Lookback) : Amount :=
  .eventTally .count who (.mk event lookback none)
/-- "if <who> <event>ed <what> <lookback>" -/
def happenedInvolving (event : EventName) (who : NounPhrase) (lookback : Lookback)
    (what : NounPhrase) : Condition :=
  .happened who (.mk event lookback (some (.involving what)))
/-- "if <who> <event>ed <lookback>" -/
def happened (event : EventName) (who : NounPhrase) (lookback : Lookback) : Condition :=
  .happened who (.mk event lookback none)
/-- "that <event>ed <lookback>" -/
def happenedTo (event : EventName) (lookback : Lookback) : Predicate :=
  .happenedTo (.mk event lookback none)
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
/-- "increase or decrease the result by N" -/
def shiftResult (amount : Amount) : Instruction := .shiftResult none amount
/-- "<player> may play N additional lands" -/
def mayPlayAdditionalLands (player : NounPhrase) (quantity : Quantity) : StaticSpec :=
  .deontic player .permit [.action "Play"] .agent (some (.additional quantity))
    (.counterpart (allOf land))
    none .noRider
/-- "<player> may <deed> <what> [as though …] [rider]" -/
def mayPlayDeed (deed : Deed) (player what : NounPhrase) (asThough : Option AsThough)
    (rider : DeonticRider) : StaticSpec :=
  .deontic player .permit [deed] .agent none (.counterpart what) asThough rider
/-- A deontic clause with no bound, premise, or rider. -/
def deontic (subject : NounPhrase) (compulsion : Compulsion) (deeds : Deeds) (role : Role)
    (patient : DeonticPatient) : StaticSpec :=
  .deontic subject compulsion deeds role none patient none .noRider
/-- "When <event>, <instruction>" as a delayed trigger. -/
def delayed (event : GameEvent) (instruction : Instruction) : Instruction :=
  .delayed event [] none instruction
/-- "there is an additional <part> [after <anchor>]" -/
def additionalPart (part : TurnPart) (anchor : Option TurnPart) (count : Amount) : Instruction :=
  .additionalPart none part anchor count none
/-- "there is an additional <part> after this phase, followed by an additional <next>" -/
def additionalPartThen (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
    (next : TurnPart) : Instruction :=
  .additionalPart none part anchor count (some next)
/-- "<player> gets an additional <part>" -/
def getsAdditionalPart (player : NounPhrase) (part : TurnPart) (count : Amount) : Instruction :=
  .additionalPart (some player) part none count none
/-- "<subject> can't attack [this turn]" -/
def cantAttack (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .continuously (.deontic subject .forbid [.core .attack] .agent none .noPatient none .noRider)
    duration
/-- "<subject> can't block [this turn]" -/
def cantBlock (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .continuously (.deontic subject .forbid [.core .block] .agent none .noPatient none .noRider)
    duration
/-- "<subject> can't be blocked [this turn]" -/
def cantBeBlocked (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .continuously (.deontic subject .forbid [.core .block] .patient none .noPatient none .noRider)
    duration

/-- "<source> deals N damage divided as you choose among <among>" -/
def dealsDivided (source : NounPhrase) (amount : Amount) (among : NounPhrase) : Instruction :=
  .distribute (.damage source) amount among

/-- The agent re-read after its own clause: "you" stays "you", anyone else is "they". -/
def agentRef : NounPhrase → NounPhrase
  | .you => .you
  | _ => they

/-- "<player> may pay <cost>. If they don't, <instruction>." -/
def unless_ (player : NounPhrase) (instruction : Instruction) (cost : Cost) : Instruction :=
  .may player (.pay (agentRef player) cost .once) none (some instruction)

/-- "it" or "them", by number. -/
def itOrThem : Plurality → NounPhrase
  | .one => it
  | .many => them
/-- The window over exactly what a phrase introduced; a phrase that introduced nothing (a
pronoun) is read again through the whole stack. -/
def sameWindow : Nat → Window
  | 0 => .whole
  | n + 1 => .top (n + 1)
/-- "it" (or "them"): the subject of a stat change, read back through a window holding only
what that subject and the change's amount announced. -/
def itsOther (subject : NounPhrase) (delta : Delta Amount) : NounPhrase :=
  .pro .bare subject.plur
    (sameWindow
      (Delta.introduced (selfSubjIntro [] subject) delta ++
        NounPhrase.selfSubjIntroduced subject ++ NounPhrase.introduced [] subject).length)
/-- "<subject> gets +P/+T [until …]": the two stat changes as one static clause; the toughness
half reads its subject back as "it". -/
def getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec :=
  .andAlso none
    [.modify subject .power power, .modify (itsOther subject power) .toughness toughness]

/-- The shared subject of a clause, read back as a pronoun that sees only what the subject
itself announced. -/
def ownSubject (subject : NounPhrase) : NounPhrase :=
  .pro .bare subject.plur (.top (selfSubjIntro [] subject).length)
/-- "them" (or "it"): the cards a look at a library slice just announced, seen alone. -/
def lookedCards (slice : NounPhrase) : NounPhrase :=
  .pro .bare slice.plur (.top (NounPhrase.introduced [] slice).length)
/-- "<looker> looks at the top N cards of <whose> library, puts any number of them on the bottom
in any order and the rest on top in any order" -/
def lookAndSort (looker whose : NounPhrase) (amount : Amount) : Instruction :=
  let slice : NounPhrase := .librarySlice .top amount whose
  .sequentially
    [ .expose .lookAt looker (.cards slice),
      move (someOf anyNumber (lookedCards slice)) (onBottomIn .anyOrder),
      move (theRest .object) (onTopIn .anyOrder) ]
/-- "<agent> scries N" [CR#701.22a] -/
def scry (agent : NounPhrase) (amount : Amount) : Instruction :=
  .enact (some agent) (.action "Scry") (lookAndSort (agentRef agent) (agentRef agent) amount)
/-- "<agent> fateseals N" [CR#701.29a] -/
def fateseal (agent whose : NounPhrase) (amount : Amount) : Instruction :=
  .enact (some agent) (.action "Fateseal") (lookAndSort (agentRef agent) whose amount)
def gets (subject : NounPhrase) (power toughness : Delta Amount) (duration : Option Duration) :
    Instruction :=
  .continuously (getsPt subject power toughness) duration

def gains (subject : NounPhrase) (ability : Ability) (duration : Option Duration) : Instruction :=
  .continuously (.gains subject ability) duration
/-- "<subject> gains haste [until …]" -/
def gainsHaste (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  gains subject (.keyword "Haste" none none) duration
/-- "<subject> becomes <added> in addition to its other types [until …]" -/
def becomes (subject : NounPhrase) (added : CharacteristicBundle) (duration : Option Duration) :
    Instruction :=
  .continuously (.becomes subject .adds (.bundle added none)) duration
/-- Several static clauses sharing one subject, as one instruction. -/
def sharedSubject (subject : NounPhrase) (parts : List StaticSpec) (duration : Option Duration) :
    Instruction :=
  .continuously (.andAlso (some subject) parts) duration
/-- "If <event> would happen, <replacement> instead [duration]." -/
def ifWouldInstead (event : GameEvent) (replacement : Instruction) (duration : Option Duration) :
    Instruction :=
  .continuously (.intercepts event [] none replacement .repeatedly none) duration
/-- "Prevent all <kind> damage that would be dealt <scope> [duration]." -/
def preventAll (kind : DamageKind) (scope : DamageScope) (duration : Option Duration) :
    Instruction :=
  .continuously (.damageRule kind .unattributed scope (.prevent .all none) .repeatedly) duration
/-- "The next time <event> would happen, <replacement> instead [duration]." -/
def nextTimeWouldInstead (event : GameEvent) (replacement : Instruction)
    (duration : Option Duration) : Instruction :=
  .continuously (.intercepts event [] none replacement .nextTimeOnly none) duration
/-- "<player> gains control of <subject> [duration]" -/
def gainControl (player : NounPhrase) (subject : NounPhrase) (duration : Option Duration) :
    Instruction :=
  .continuously (.gainsControl player subject) duration
/-- "<subject> can't be <deed>ed" -/
def objectCant (deed : Deed) (subject : NounPhrase) : StaticSpec :=
  .deontic subject .forbid [deed] .patient none .noPatient none .noRider
/-- "<player> can't <deed>" -/
def playerCant (deed : Deed) (player : NounPhrase) : StaticSpec :=
  .deontic player .forbid [deed] .agent none .noPatient none .noRider
/-- "<spec> as long as <condition>" -/
def onlyWhile (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditionally spec condition .asLongAs
/-- "<spec> unless <condition>" -/
def onlyUnless (spec : StaticSpec) (condition : Condition) : StaticSpec :=
  .conditionally spec (.not condition) .unless_
/-- "<who> can't <deed> <what>" -/
def cantDoTo (deed : Deed) (who what : NounPhrase) : StaticSpec :=
  .deontic who .forbid [deed] .agent none (.counterpart what) none .noRider
/-- "<what> can't be the target of <by>" -/
def cantBeTargetedBy (what by_ : NounPhrase) : StaticSpec :=
  .deontic what .forbid [.core .target] .patient none (.targetedBy by_) none .noRider
/-- "<what> can be the target of <by> as though it didn't have <p>" -/
def canBeTargetedAsThough (what by_ : NounPhrase) (p : Predicate) : StaticSpec :=
  .deontic what .permit [.core .target] .patient none (.targetedBy by_) (some (.of p)) .noRider
/-- "<spec> <duration>": a clause holding for a stated duration. -/
def throughout (spec : StaticSpec) (duration : Duration) : Instruction :=
  .continuously spec (some duration)
/-- "<n> doesn't untap during [<whose>] untap step" -/
def doesntUntap (subject : NounPhrase) (whose : Option NounPhrase) : StaticSpec :=
  .onlyDuring .untapStep whose (objectCant (.action "Untap") subject)
/-- "<player> can't <deed> more than N <p>" -/
def cantMoreThan (player : NounPhrase) (deed : Deed) (bound : Nat) (p : Predicate) :
    StaticSpec :=
  .deontic player .forbid [deed] .agent (some (.moreThan (.lit bound))) (.counterpart (allOf p))
    none .noRider

/-! ## Events -/

def leavesBattlefield (subject : NounPhrase) : GameEvent :=
  .leaves subject (some (.zones [battlefield]))
/-- "at the beginning of <possessor>'s <part>" -/
def beginningOfPossessed (quantifier : PartQuant) (part : TurnPart) (possessor : NounPhrase) :
    GameEvent :=
  .beginningOf quantifier part (.byPlayer possessor)
/-- "that turn's": the extra turn just granted, as a header possessor. -/
def thatTurns : HeaderPossessor := .byTurn thatTurn
def dealsCombatDamage (source : NounPhrase) (patient : NounPhrase) : GameEvent :=
  .dealsDamage .combatOnly source (some patient)
def attacks (subject : NounPhrase) : GameEvent := .combat .attackerOf subject none
/-- "<subject> attacks <whom>" -/
def attacksPlayer (subject whom : NounPhrase) : GameEvent :=
  .combat .attackerOf subject (some whom)
def blocks (subject : NounPhrase) (blocked : Option NounPhrase) : GameEvent :=
  .combat .blockerOf subject blocked
def becomesBlocked (subject : NounPhrase) (by_ : Option NounPhrase) : GameEvent :=
  .combat .blockedBy subject by_
/-- "the last <kind> counter is removed from <subject>" -/
def lastCounterRemoved (kind : CounterKind) (subject : NounPhrase) : GameEvent :=
  .counterEvent .removed (some kind) subject .last none false
/-- "<subject> regenerates": the verb as an event. -/
def regenerates (subject : NounPhrase) : GameEvent :=
  .verbedEvent none (.action "Regenerate") (some subject) none

/-! ## Abilities -/

def keyword (label : KeywordLabel) : Ability := .keyword label none none
/-- An ability word in italics before an ability: "Will of the council — …" [CR#207.2c]. -/
def abilityWord (word : AbilityWordLabel) (ability : Ability) : Ability :=
  .italicHead (.abilityWord word) ability
/-- A flavor word in italics before an ability [CR#207.2d]. -/
def flavorWord (word : FlavorWordLabel) (ability : Ability) : Ability :=
  .italicHead (.flavorWord word) ability
/-- "Companion — <condition>" -/
def companion (condition : DeckCondition) : Ability :=
  .keyword "Companion" (some (.deckCondition condition)) none
/-- "Pay N life" as a cost. -/
def payLife (player : NounPhrase) (amount : Nat) : Cost :=
  .perform (.changeLife player (.down (.lit amount)))
/-- "<keyword> <cost>" -/
def keywordCosting (label : KeywordLabel) (cost : Cost) : Ability :=
  .keyword label (some (.cost cost)) none
/-- "<keyword> <quality>", e.g. "protection from red" -/
def keywordQuality (label : KeywordLabel) (quality : Predicate) : Ability :=
  .keyword label (some (.quality quality)) none
/-- "<keyword> <subject>", e.g. "enchant creature" -/
def keywordSubject (label : KeywordLabel) (subject : Predicate) : Ability :=
  .keyword label (some (.subject subject)) none
/-- "<keyword> N", e.g. "bushido 2" -/
def keywordNumber (label : KeywordLabel) (amount : Amount) : Ability :=
  .keyword label (some (.number amount)) none
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
  .entersChoice subject (.quality sort) none .openly
/-- "As <subject> enters, choose a <quality> from <domain>." -/
def entersChoosingFrom (subject : NounPhrase) (sort : QualitySort) (domain : ChoiceDomain) :
    StaticSpec :=
  .entersChoice subject (.quality sort) (some domain) .openly
/-- "<subject> enters tapped" -/
def entersTapped (subject : NounPhrase) : StaticSpec := .entersRider subject (.entersAs .tapped)
/-- "<subject> enters with N <kind> counters on it" -/
def entersWithCounters (subject : NounPhrase) (amount : Amount) (kind : CounterKind) :
    StaticSpec :=
  .entersRider subject (.withCounters amount (.printed kind) .fresh)

def triggered (event : GameEvent) (instruction : Instruction) : Ability :=
  .triggered event [] none [] none none none instruction
/-- "Whenever <event> and whenever <joined events>, <instruction>" -/
def triggeredJoined (event : GameEvent) (joins : List JoinedHeader) (instruction : Instruction) :
    Ability :=
  .triggered event [] none joins none none none instruction
/-- A further "whenever <event>" header joined onto a trigger. -/
def joinedHead (event : GameEvent) : JoinedHeader := ⟨event, [], none, none⟩
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
        .gainsDesignation thisCreature "renowned" (.byKeyword "Renown") none ])
/-- Storm's reminder text: "When you cast this spell, copy it for each other spell that was cast
before it this turn. You may choose new targets for the copies." [CR#702.40a] -/
def stormExpansion : Ability :=
  when (.casts .you thisSpell none)
    (.sequentially
      [ .copy .fromStack .you thisSpell
          (eventCountInvolving .spellCast (a .anyPlayer) .earlierThisTurn
            (a (.and [spell, .otherThan thisSpell])))
          [],
        may .you (.chooseNewTargets (.pro (.word .copy) .many .whole)) ])
/-- Cumulative upkeep's reminder text: "At the beginning of your upkeep, if this permanent is
on the battlefield, put an age counter on it. Then you may pay [cost] for each age counter on
it. If you don't, sacrifice it." [CR#702.24a] -/
def cumulativeUpkeepExpansion (cost : Cost) : Ability :=
  triggeredIf (.beginningOf .the .upkeep (.byPlayer .you))
    (.matches thisPermanent (.inZone battlefield))
    (.sequentially
      [ .putCounters (.lit 1) (.printed (.named "Age")) thisPermanent,
        .may .you
          (.pay .you (.scaled cost (times (.lit 1) (countersOn (.named "Age") thisPermanent)))
            .once)
          none (some (sacrifice .you thisPermanent)) ])
/-- "Cumulative upkeep [cost]" with its reminder text. -/
def cumulativeUpkeep (cost : Cost) : Ability :=
  .keyword "CumulativeUpkeep" (some (.cost cost)) (some (cumulativeUpkeepExpansion cost))
def activated (cost : Cost) (instruction : Instruction) : Ability :=
  .activated cost instruction none none none none
/-- "[cost]: <instruction>. Activate only <timing>." -/
def activatedOnlyDuring (cost : Cost) (instruction : Instruction) (timing : Timing) : Ability :=
  .activated cost instruction (some timing) none none none

/-! ## Cards -/

/-- A printed power, toughness, loyalty, or defense. -/
def stat (value : Nat) : Option Amount := some (.lit value)
/-- One level band of a leveler: its range, power/toughness box, and text [CR#711.2a]. -/
def levelBand (range : LevelRange) (power toughness : Nat) (text : List Ability) : LevelBand :=
  { range, text, power := stat power, toughness := stat toughness }
/-- A prototype frame's inset set: its mana cost and power/toughness box [CR#718.1]. -/
def prototypeAlt (cost : ManaCost) (power toughness : Nat) : PrototypeFrame :=
  { cost := some cost, power := stat power, toughness := stat toughness }

end Semantics.Macros
