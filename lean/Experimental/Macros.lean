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
def it : Noun := .pro .bare .one .whole

/-- "them" -/
def them : Noun := .pro .bare .many .whole

/-- "that <word>", e.g. `that .player`. -/
def that (w : NounWord) : Noun := .pro (.word w) .one .whole

/-- "those <word>s", e.g. `those .card`. -/
def those (w : NounWord) : Noun := .pro (.word w) .many .whole

/-- "they": the player most recently named. -/
def they : Noun := .pro (.word .player) .one .whole

/-- "the <verb>ed <word>", e.g. "the tapped creatures". -/
def theVerbed (v : VerbLabel) (w : NounWord) (marking : VerbedMarking) (pl : Plurality) : Noun :=
  .pro (.verbed v w marking) pl .whole

/-! ## Quantities -/

def exactly (n : Nat) : Quantity := .range (some n) (some n)
def upTo (n : Nat) : Quantity := .range none (some n)
def anyNumber : Quantity := .range none none
def atLeast (n : Nat) : Quantity := .range (some n) none
def oneThrough (n : Nat) : Quantity := .range (some 1) (some n)

/-! ## Determiners -/

/-- "target …" -/
def target (p : Predicate) : Noun := .described (.target (.range (some 1) (some 1))) p

/-- "each …" -/
def each (p : Predicate) : Noun := .described .each p

/-- "all …" -/
def allOf (p : Predicate) : Noun := .described .all p

/-- the bare plural: "creatures" -/
def bare (p : Predicate) : Noun := .described .bare p

/-- "a …" -/
def a (p : Predicate) : Noun := .described (.a .unmarked) p

/-- "the …" -/
def the (p : Predicate) : Noun := .described .the p

/-- "N …", "up to N …" -/
def counted (q : Quantity) (p : Predicate) : Noun := .described (.count q none) p

/-- "if there is a …" -/
def thereIs (p : Predicate) : Condition := .thereIs (bare p)

/-- "the number of …s" -/
def countOf (p : Predicate) : Amount := .countOf (bare p)

/-- "the greatest power among …s" -/
def aggregate (op : AggregateOp) (ax : ProjAxis) (p : Predicate) : Amount :=
  .aggregate op ax (bare p)

/-! ## Predicates -/

def creature : Predicate := .hasType .creature
def artifact : Predicate := .hasType .artifact
def enchantment : Predicate := .hasType .enchantment
def land : Predicate := .hasType .land
def spell : Predicate := .isSpell
def untapped : Predicate := .hasStatus .untapped
def unblocked : Predicate := .not .blocked
def creatureYouControl : Predicate := .and [creature, .hasPossessor .controller .you]
def emblem : Predicate := .isEmblem
def copyOfACard : Predicate := .isCopyOfACard
def cardOnTheStack : Predicate := .and [.isCard, .isSpell]
def tokenOnTheBattlefield : Predicate := .and [.isToken, .permanent]

/-- The four party roles, in rule order [CR#700.8]. -/
def partyRoles : List Predicate :=
  [ .hasSubtype (.of .creature "Cleric"), .hasSubtype (.of .creature "Rogue"),
    .hasSubtype (.of .creature "Warrior"), .hasSubtype (.of .creature "Wizard") ]

/-- "any target" [CR#115.4]. -/
def anyTarget : Predicate :=
  .joined (.or [.hasType .creature, .hasType .planeswalker, .hasType .battle]) .anyPlayer

def creatureType (label : String) : Subtype := .of .creature label
def artifactType (label : String) : Subtype := .of .artifact label
def landType (label : String) : Subtype := .of .land label
def enchantmentType (label : String) : Subtype := .of .enchantment label
def spellType (label : String) : Subtype := .spell label

/-! ## Nouns -/

def anOpponent : Noun := a .opponent
def thisCreature : Noun := .asType .creature .this none
def thisArtifact : Noun := .asType .artifact .this none
def controllerOf (n : Noun) : Noun := .possessorOf .controller n
def ownerOf (n : Noun) : Noun := .possessorOf .owner n
/-- "the top N cards of your library" -/
def topSlice (amt : Amount) : Noun := .librarySlice .top amt .you
/-- "[a player]'s party" [CR#700.8]. -/
def partyOf (who : Noun) : Noun :=
  .oneEachOf partyRoles (allOf (.and [creature, .hasPossessor .controller who]))
/-- "your party" -/
def party : Noun := partyOf .you

/-! ## Zones -/

def battlefieldZ : ZoneExpr := .zoneAt .battlefield .bare
def exileZ : ZoneExpr := .zoneAt .exile .bare
def handZ : ZoneExpr := .zoneAt .hand .bare
def libraryZ : ZoneExpr := .zoneAt .library .bare
def graveyardZ : ZoneExpr := .zoneAt .graveyard .bare
def handOf (n : Noun) : ZoneExpr := .zoneAt .hand (.possessedBy n)
def graveyardOf (n : Noun) : ZoneExpr := .zoneAt .graveyard (.possessedBy n)
/-- "Nth from the top of its owner's library" -/
def nthFromTop (n : Ordinal) : ZoneExpr := .libraryAt (.oneEnd .top) none (some n) .bare

/-! ## Mana -/

def generic (n : Nat) : ManaSymbol := .simple (.generic n)
def pip (c : Color) : ManaSymbol := .simple (.specific (.of c))
def colorlessPip : ManaSymbol := .simple (.specific .colorless)

/-! ## Durations -/

def untilEndOfTurn : Duration := .until (.endOf .turn none)
def untilEndOfCombat : Duration := .until (.endOf .combat none)
def untilYourNextTurn : Duration := .until (.startOf .turn (some .you))

/-! ## Instructions -/

def move (what : Noun) (to : ZoneExpr) : Instruction := .move what to []
def destroy (n : Noun) : Instruction := .enact none "Destroy" (.move n graveyardZ [])
def exile (n : Noun) : Instruction := .enact none "Exile" (.move n exileZ [])
def sacrifice (agent : Noun) (n : Noun) : Instruction :=
  .enact (some agent) "Sacrifice" (.move n graveyardZ [])
def tap (n : Noun) : Instruction := .enact none "Tap" (.setStatus .tapped n)
def losesLife (who : Noun) (amt : Amount) : Instruction := .changeLife who (.down amt)
def gainsLife (who : Noun) (amt : Amount) : Instruction := .changeLife who (.up amt)
def draw (who : Noun) (amt : Amount) : Instruction := .draw who amt

def lookAt (n : Noun) : Instruction := .expose .lookAt .you (.cards n)
def revealCards (n : Noun) : Instruction := .expose .reveal .you (.cards n)
def shuffleInto (agent : Noun) (n : Noun) : Instruction :=
  .enact (some agent) "Shuffle" (.move n (.libraryAt .shuffled none none .bare) [])
def ifThen (c : Condition) (e : Instruction) : Instruction := .ifThen c e none
def rollDice (who : Noun) (count sides : Nat) : Instruction :=
  .rollDice who (.lit count) (.sides sides)

/-- A token's characteristics from the parts a creature token names. -/
def creatureTokOf (power toughness : Amount) (cs : List Color) (ss : List Subtype) :
    TokenChars :=
  ⟨some (power, toughness), cs, ⟨[], [.creature], ss⟩, [], none, []⟩
def creatureTok (power toughness : Nat) (cs : List Color) (ss : List Subtype) : TokenChars :=
  creatureTokOf (.lit power) (.lit toughness) cs ss
/-- "create N <token>" -/
def create (count : Amount) (tok : TokenChars) : Instruction :=
  .create .you count (.written tok) []

/-- "for each color of mana spent to cast <n>" -/
def colorsSpentToCast (n : Noun) : Amount := .paid .colorsSpent n
/-- "if colored mana was spent to cast <n>" -/
def coloredManaSpentToCast (n : Noun) : Condition :=
  .compareAmt (colorsSpentToCast n) .atLeast (.lit 1)

/-- "<n> can't attack [this turn]" -/
def cantAttack (n : Noun) (span : Option Duration) : Instruction :=
  .continuously (.deontic n .forbid ["Attack"] .agent none .noPatient none .noRider) span
/-- "<n> can't block [this turn]" -/
def cantBlock (n : Noun) (span : Option Duration) : Instruction :=
  .continuously (.deontic n .forbid ["Block"] .agent none .noPatient none .noRider) span

/-- "<src> deals N damage divided as you choose among <among>" -/
def dealsDivided (src : Noun) (amt : Amount) (among : Noun) : Instruction :=
  .distribute (.damage src) amt among

/-- The agent re-read after its own clause: "you" stays "you", anyone else is "they". -/
def agentRef : Noun → Noun
  | .you => .you
  | _ => they

/-- "<who> may pay <c>. If they don't, <e>." -/
def unlessPays (who : Noun) (e : Instruction) (c : Cost) : Instruction :=
  .may who (.pay (agentRef who) c .once) none (some e)

/-- "<n> gets +P/+T [until …]": the two stat changes as one static clause; the toughness
half reads its subject back as "it". -/
def getsPt (n : Noun) (power toughness : Delta Amount) : StaticSpec :=
  .andAlso none [.modify n .power power, .modify (itOrThem (nounPlurOf n)) .toughness toughness]
where
  itOrThem : Plurality → Noun
    | .one => it
    | .many => them
  /-- The number of a noun phrase, as far as the syntax alone can tell. -/
  nounPlurOf : Noun → Plurality
    | .described d _ =>
      match d with
      | .target (.range _ (some 1)) => .one
      | .a _ => .one
      | .the => .one
      | .count (.range _ (some 1)) _ => .one
      | _ => .many
    | .pro _ pl _ => pl
    | .eachOf _ | .both _ _ | .playerGroup _ | .oneEachOf _ _ => .many
    | _ => .one

def gets (n : Noun) (power toughness : Delta Amount) (d : Option Duration) : Instruction :=
  .continuously (getsPt n power toughness) d

def gains (n : Noun) (ab : AbilityAt) (d : Option Duration) : Instruction :=
  .continuously (.gains n ab) d

/-! ## Events -/

def leavesBattlefield (n : Noun) : GameEvent := .leaves n (some (.zones [battlefieldZ]))
def dealsCombatDamage (n : Noun) (to : Noun) : GameEvent := .dealsDamage .combatOnly n (.one to)

/-! ## Abilities -/

def keyword (kw : KeywordLabel) : AbilityAt := .keyword kw none none

def triggered (word : TriggerWord) (ev : GameEvent) (instr : Instruction) : AbilityAt :=
  .triggered word ev [] none [] none none none instr

def activated (cost : Cost) (instr : Instruction) : AbilityAt :=
  .activated cost instr none none none none

/-! ## Cards -/

def printedBox : Option (Int × Int) → Option PrintedBox
  | none => none
  | some (p, t) => some (.pt (.num p) (.num t))

/-- A single-faced card from its printed parts. -/
def card (name : String) (cost : Option ManaCost) (typeLine : TypeLine) (text : AbilitySeq)
    (stats : Option (Int × Int) := none) : Card :=
  .singleFaced { name, cost, typeLine, text, box := printedBox stats }

end Mtg.Macros
