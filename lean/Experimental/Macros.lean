import Experimental

/-!
# Experimental.Macros

Spellings over the raw constructors. Port of `idris/src/Experimental/Macros.idr`, the
subset the `Cards` bench uses; every macro keeps its Idris name and argument order, minus
the obligations it forwarded.

The Idris bench qualifies every macro (`Macros.target`) so a reader can tell a spelling from
a constructor at a glance. Lean does not need that: a constructor is written with a leading
dot (`.hasType`), a macro without (`creature`), so the bench opens `Mtg.Macros` and writes
them bare.
-/

namespace Mtg.Macros

/-! ## Pronouns -/

/-- "it" -/
def it : NounPhrase := .pro .bare .one .whole

/-- "them" -/
def them : NounPhrase := .pro .bare .many .whole

/-- "that <word>", e.g. `that .player`. -/
def that (w : NounWord) : NounPhrase := .pro (.word w) .one .whole

/-- "those <word>s", e.g. `those .card`. -/
def those (w : NounWord) : NounPhrase := .pro (.word w) .many .whole

/-- "they": the player most recently named. -/
def they : NounPhrase := .pro (.word .player) .one .whole

/-- "the <verb>ed <word>", e.g. "the tapped creatures". -/
def theVerbed (verb : VerbLabel) (word : NounWord) (marking : VerbedMarking)
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

/-- "the …" -/
def the (p : Predicate) : NounPhrase := .described .the p

/-- "N …", "up to N …" -/
def counted (q : Quantity) (p : Predicate) : NounPhrase := .described (.count q none) p

/-- "if there is a …" -/
def exists_ (p : Predicate) : Condition := .exists_ (bare p)

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
def handOf (player : NounPhrase) : ZoneExpr := .zone .hand (.possessedBy player)
def graveyardOf (player : NounPhrase) : ZoneExpr := .zone .graveyard (.possessedBy player)
/-- "Nth from the top of its owner's library" -/
def nthFromTop (ordinal : Ordinal) : ZoneExpr := .library (.oneEnd .top) none (some ordinal) .bare

/-! ## Predicates -/

def creature : Predicate := .hasType .creature
def artifact : Predicate := .hasType .artifact
def enchantment : Predicate := .hasType .enchantment
def land : Predicate := .hasType .land
/-- "spell": an object on the stack that is not an ability [CR#112.1,113.1]. -/
def spell : Predicate := .and [.inZone stack, .not (.abilityHead .anyOnStack)]
def untapped : Predicate := .hasStatus .untapped
/-- "permanent": an object on the battlefield [CR#110.1]. -/
def permanent : Predicate := .inZone battlefield
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
def emblem : Predicate := .isEmblem
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

def creatureType (label : String) : Subtype := .of .creature label
def artifactType (label : String) : Subtype := .of .artifact label
def landType (label : String) : Subtype := .of .land label
def enchantmentType (label : String) : Subtype := .of .enchantment label
def spellType (label : String) : Subtype := .spell label

/-! ## Nouns -/

def anOpponent : NounPhrase := a .opponent
def thisCreature : NounPhrase := .asType .creature .this none
def thisArtifact : NounPhrase := .asType .artifact .this none
def theDefendingPlayer : NounPhrase := .combatPlayer .defending
def theAttackingPlayer : NounPhrase := .combatPlayer .attacking
def controllerOf (subject : NounPhrase) : NounPhrase := .possessorOf .controller subject
def ownerOf (subject : NounPhrase) : NounPhrase := .possessorOf .owner subject
/-- "the top N cards of your library" -/
def topSlice (amount : Amount) : NounPhrase := .librarySlice .top amount .you
/-- "[a player]'s party" [CR#700.8]. -/
def partyOf (player : NounPhrase) : NounPhrase :=
  .oneEachOf partyRoles (allOf (.and [creature, .hasPossessor .controller player]))
/-- "your party" -/
def party : NounPhrase := partyOf .you

/-! ## Mana -/

def generic (amount : Nat) : ManaSymbol := .simple (.generic amount)
def pip (color : Color) : ManaSymbol := .simple (.specific (.of color))
def colorlessPip : ManaSymbol := .simple (.specific .colorless)

/-! ## Durations -/

def untilEndOfTurn : Duration := .until_ (.endOf .turn none)
def untilEndOfCombat : Duration := .until_ (.endOf .combat none)
def untilYourNextTurn : Duration := .until_ (.startOf .turn (some .you))

/-! ## Amounts -/

def lifeTotalOf (player : NounPhrase) : Amount := .statOf (.playerStat .lifeTotal) player
def powerOf (subject : NounPhrase) : Amount := .statOf (.stat .power) subject
def toughnessOf (subject : NounPhrase) : Amount := .statOf (.stat .toughness) subject
def countersOn (kind : CounterKind) (holder : NounPhrase) : Amount := .statOf (.counter kind) holder
def plus (left right : Amount) : Amount := .arith .plus left right
def minus (left right : Amount) : Amount := .arith .minus left right
def times (per amount : Amount) : Amount := .arith .times per amount

/-! ## Instructions -/

def move (subject : NounPhrase) (destination : ZoneExpr) : Instruction :=
  .move subject destination []
def destroy (subject : NounPhrase) : Instruction :=
  .enact none "Destroy" (.move subject graveyard [])
def exile (subject : NounPhrase) : Instruction := .enact none "Exile" (.move subject exileZone [])
def sacrifice (agent : NounPhrase) (subject : NounPhrase) : Instruction :=
  .enact (some agent) "Sacrifice" (.move subject graveyard [])
def tap (subject : NounPhrase) : Instruction := .enact none "Tap" (.setStatus .tapped subject)
def regenerate (subject : NounPhrase) : Instruction := .regenerate subject
def losesLife (player : NounPhrase) (amount : Amount) : Instruction :=
  .changeLife player (.down amount)
def gainsLife (player : NounPhrase) (amount : Amount) : Instruction :=
  .changeLife player (.up amount)
def draw (player : NounPhrase) (amount : Amount) : Instruction := .draw player amount

def lookAt (cards : NounPhrase) : Instruction := .expose .lookAt .you (.cards cards)
def revealCards (cards : NounPhrase) : Instruction := .expose .reveal .you (.cards cards)
def shuffleInto (agent : NounPhrase) (subject : NounPhrase) : Instruction :=
  .enact (some agent) "Shuffle" (.move subject (.library .shuffled none none .bare) [])
def if_ (condition : Condition) (instruction : Instruction) : Instruction :=
  .if_ condition instruction none
def rollDice (player : NounPhrase) (count sides : Nat) : Instruction :=
  .rollDice player (.lit count) (.sides sides)

/-- A token's characteristics from the parts a creature token names. -/
def creatureTokenOf (power toughness : Amount) (colors : List Color) (subtypes : List Subtype) :
    Characteristics :=
  { colors, types := [.creature], subtypes, power := some power, toughness := some toughness }
def creatureToken (power toughness : Nat) (colors : List Color) (subtypes : List Subtype) :
    Characteristics :=
  creatureTokenOf (.lit power) (.lit toughness) colors subtypes
/-- "create N <token>" -/
def create (count : Amount) (token : Characteristics) : Instruction :=
  .create .you count (.written token) []

/-- "for each color of mana spent to cast <n>" -/
def colorsSpentToCast (spell : NounPhrase) : Amount := .paid .colorsSpent spell
/-- "if colored mana was spent to cast <spell>" -/
def coloredManaSpentToCast (spell : NounPhrase) : Condition :=
  .compareAmt (colorsSpentToCast spell) .atLeast (.lit 1)

/-- "<subject> can't attack [this turn]" -/
def cantAttack (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .continuously (.deontic subject .forbid ["Attack"] .agent none .noPatient none .noRider) duration
/-- "<subject> can't block [this turn]" -/
def cantBlock (subject : NounPhrase) (duration : Option Duration) : Instruction :=
  .continuously (.deontic subject .forbid ["Block"] .agent none .noPatient none .noRider) duration

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

/-- "<subject> gets +P/+T [until …]": the two stat changes as one static clause; the toughness
half reads its subject back as "it". -/
def getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec :=
  .andAlso none
    [.modify subject .power power, .modify (itOrThem (nounPlurOf subject)) .toughness toughness]
where
  itOrThem : Plurality → NounPhrase
    | .one => it
    | .many => them
  /-- The number of a noun phrase, as far as the syntax alone can tell. -/
  nounPlurOf : NounPhrase → Plurality
    | .described d _ =>
      match d with
      | .target (.range _ (some 1)) => .one
      | .a _ => .one
      | .the => .one
      | .count (.range _ (some 1)) _ => .one
      | _ => .many
    | .pro _ plurality _ => plurality
    | .eachOf _ | .both _ _ | .playerGroup _ | .oneEachOf _ _ => .many
    | _ => .one

def gets (subject : NounPhrase) (power toughness : Delta Amount) (duration : Option Duration) :
    Instruction :=
  .continuously (getsPt subject power toughness) duration

def gains (subject : NounPhrase) (ability : Ability) (duration : Option Duration) : Instruction :=
  .continuously (.gains subject ability) duration

/-! ## Events -/

def leavesBattlefield (subject : NounPhrase) : GameEvent :=
  .leaves subject (some (.zones [battlefield]))
def dealsCombatDamage (source : NounPhrase) (patient : NounPhrase) : GameEvent :=
  .dealsDamage .combatOnly source (some patient)
def attacks (subject : NounPhrase) : GameEvent := .combat .attackerOf subject none
def blocks (subject : NounPhrase) (blocked : Option NounPhrase) : GameEvent :=
  .combat .blockerOf subject blocked
def becomesBlocked (subject : NounPhrase) (by_ : Option NounPhrase) : GameEvent :=
  .combat .blockedBy subject by_
def becomesAttached (subject : NounPhrase) (host : NounPhrase) : GameEvent :=
  .attachment .attached subject host
/-- "<subject> regenerates": the verb as an event. -/
def regenerates (subject : NounPhrase) : GameEvent :=
  .verbedEvent none "Regenerate" (some subject) none

/-! ## Abilities -/

def keyword (label : KeywordLabel) : Ability := .keyword label none none

def triggered (word : TriggerWord) (event : GameEvent) (instruction : Instruction) : Ability :=
  .triggered word event [] none [] none none none instruction

def activated (cost : Cost) (instruction : Instruction) : Ability :=
  .activated cost instruction none none none none

/-! ## Cards -/

/-- A printed power or toughness. -/
def stat (value : Nat) : Option Amount := some (.lit value)

end Mtg.Macros
