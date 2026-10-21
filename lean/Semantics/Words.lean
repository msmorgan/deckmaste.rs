import Semantics.Authoring

/-!
# Semantics.Words

The vocabulary: every leaf type a phrase, trigger, effect, or card is written from. Port of
`idris/src/Experimental/Words.idr`, syntax only.

A type is here iff a constructor field mentions it. The checker's own vocabulary (the
antecedent stack, obligation witnesses, the facts tables, event names, static kinds) lives in
`Check/`. Designations, keywords, acts, counters, and deeds are labels with generated facts
tables, not enums: the grammar carries the mechanism, the tables carry the set.
-/

namespace Semantics

/-- Polymorphic wildcard value (`any`) across action-clause subjects, noun phrases, amounts,
counter kinds, dice, roll watches, zones, damage kinds, and timings. -/
class AnyValue (α : Type) where
  any : α

export AnyValue (any)

/-- [CR#205.2a] A dungeon begins outside the game and is never a permanent or a spell
[CR#309.2,309.2c]; it is a card type all the same, and the `dungeon` type declaration names it. -/
inductive CardType where
  | creature | artifact | land | enchantment | instant | sorcery | planeswalker | battle | kindred
  | dungeon
  deriving DecidableEq, Repr

/-- A rules procedure a card can instruct a player to carry out during resolution: activating an
ability [CR#701.2a,602.1], casting a spell [CR#701.5a,601.2], or playing a card as a land or as a
spell [CR#701.18a,701.18b]. -/
inductive Procedure where
  | activate | cast | play
  deriving DecidableEq, Repr

/-- An object's numeric characteristic or calculated value [CR#109.3,202.3,208.1,306.5,310.4,310.4a]. -/
inductive Stat where
  | power | toughness | manaValue | loyalty | defense
  deriving DecidableEq, Repr

inductive PlayerStat where
  | lifeTotal | startingLifeTotal
  /-- A player's speed [CR#702.179a]: none until a rule or effect sets it, read as 0 then
  [CR#702.179b,702.179f]. -/
  | speed
  deriving DecidableEq, Repr

inductive Comparator where
  | atLeast | atMost | greater | less | eq
  deriving DecidableEq, Repr

/-- The sorts a quality noun ("a color", "a creature type", "a card name") ranges over. -/
inductive QualitySort where
  | color
  | subtype (host : CardType)
  | cardName
  | number
  | cardType
  | counterKind
  /-- "an ability" chosen among keywords ("choose flying or trample"). -/
  | ability
  deriving DecidableEq, Repr

inductive Letter where
  | x | y
  deriving DecidableEq, Repr

/-- The sort of thing a phrase denotes. -/
inductive Kind where
  | object
  | player
  | quality (sort : QualitySort)
  | outcome
  | letter (letter : Letter)
  | turnRef
  /-- A pile of objects; the pile itself is not an object [CR#700.3b]. -/
  | pile
  /-- Idris `(\/)`: a phrase denoting either sort ("target creature or player"). -/
  | join (left right : Kind)
  deriving DecidableEq, Repr

inductive AggregateOp where
  | sum | min | max
  deriving DecidableEq, Repr

inductive SubtypeScope where
  | any | basicOnly | nonbasicOnly
  deriving DecidableEq, Repr

/-- An axis a "for each kind of …" ranges over. -/
inductive KindAxis where
  | cardType
  | permanentType
  | color
  | subtype (host : CardType) (scope : SubtypeScope)
  | value (stat : Stat)
  | counterKind
  | colorPair
  deriving DecidableEq, Repr

/-- The sort of an action outcome read back by `thatMuch`, `thatMany`, or `ResultMode.instead`
[CR#120.3d]. -/
inductive OutcomeSort where
  | damageDealt | damageMarked | lifeGained | lifeLost | countersPut | damagePrevented | rollResult
  | coinFlipped | diceRolled | namedNumber | repeatCount | countersRemoved
  | manaAdded | manaProduced | ceilingShortfall | voteHeld
  deriving DecidableEq, Repr

inductive FlipCall where
  | wins | loses
  deriving DecidableEq, Repr

inductive CoinFace where
  | heads | tails
  deriving DecidableEq, Repr

inductive RollExtreme where
  | lowest | highest
  deriving DecidableEq, Repr

inductive Plurality where
  | one | many
  deriving DecidableEq, Repr

/-- Sharing on an axis: at least one value in common (`share`), or none (`differ`)
[CR#201.2a,201.2c,105.2,205.3m]. -/
inductive Agreement where
  | share | differ
  deriving DecidableEq, Repr

/-- Sharing over one axis's values: `share` holds when the two value lists meet, `differ` when
they do not. On names this is not equality: an object with two names shares a name with an
object that has either [CR#201.2a]. -/
def Agreement.holdsOn {α : Type} [BEq α] : Agreement → List α → List α → Bool
  | .share, mine, theirs => mine.any theirs.contains
  | .differ, mine, theirs => !mine.any theirs.contains

inductive PossessorAxis where
  | owner | controller
  deriving DecidableEq, Repr

/-- How an object stands in combat, optionally toward a counterpart: "attacking" is
`attackerOf` with none, "creature blocking it" is `blockerOf` with one. `declaredAttacker` is
the declare-attackers moment [CR#508.1]. -/
inductive CombatRelation where
  | attackerOf | declaredAttacker | blockerOf | blockedBy | attackedBy | couldBlock
  | couldBeBlockedBy
  deriving DecidableEq, Repr

/-- The two players of a combat [CR#506.2]. -/
inductive CombatRole where
  | attacking | defending
  deriving DecidableEq, Repr

/-- An attachment's coming or going. -/
inductive AttachMove where
  | attached | unattached
  deriving DecidableEq, Repr

inductive ArithOp where
  | plus | minus | times | differenceBetween
  deriving DecidableEq, Repr

inductive Zone where
  | battlefield | graveyard | exile | hand | library | stack | command
  deriving DecidableEq, Repr

inductive PileEnd where
  | top | bottom
  deriving DecidableEq, Repr

inductive Arrangement where
  | anyOrder | randomOrder
  deriving DecidableEq, Repr

/-- The N of "Nth" [CR#401.7]; the Idris carries `IsSucc n`. -/
inductive Ordinal where
  | nth (n : Nat)
  deriving DecidableEq, Repr

/-- A designation [CR#701.15b,725.1,731.1]: the NAME of the designation declaration that defines
it, verbatim (`goaded`, `monarch`, `ringBearer`), or, where a declaration lists several members,
the member's own name (`dayNight` lists `day` and `night`). The name is not English: the phrase
a card prints ("the monarch", "Ring-bearer") is the declaration's spelling. Which designations
exist, and what each holds and does, is the designation facts table generated from the
declarations' definitions (`Definition.designation`); the grammar carries only the mechanism. -/
inductive DesignationLabel where
  | named (name : String)
  deriving DecidableEq, Repr

/-- A Relation [CR#702.55b,702.95b,702.99b]: the NAME of the relation declaration that defines
it, verbatim (`haunting`, `paired`, `encodedOn`). Which relations exist, and the kinds of their
two sides, is the relation facts table generated from the declarations' definitions
(`Definition.relation`); the grammar carries only the mechanism. -/
inductive RelationLabel where
  | named (name : String)
  deriving DecidableEq, Repr

/-- Where a merging object is placed: on top of the permanent or under it [CR#730.2]. -/
inductive MergePosition where
  | top | bottom
  deriving DecidableEq, Repr

/-- Who holds a designation [CR#701.15b,725.1,731.1]: an Entity of the named kind, the card
itself (a commander is designated on its card, and the designation follows the card between
zones [CR#903.3]), or the game. -/
inductive DesignationScope where
  | heldBy (holder : Kind)
  | heldByCard
  | heldByGame
  deriving DecidableEq, Repr

/-- A keyword ability [CR#702.1]: the NAME of the keyword ability declaration that defines it,
verbatim (`flying`, `firstStrike`), never the phrase a card prints ("first strike"), which is the
declaration's spelling. A type of its own rather than a `String`, as `DesignationLabel` is, so a
card names a keyword by its declaration's name and an undeclared name is refused when it loads. -/
inductive KeywordLabel where
  | named (name : String)
  deriving DecidableEq, Repr
/-- An ability word [CR#207.2c], as the CR's list spells it (`threshold`, `council’s dilemma`).
It has no rules meaning; the CR lists the ability words, so a word not on the list is refused
(`knownAbilityWord`). A type of its own, as `KeywordLabel` is. -/
inductive AbilityWordLabel where
  | named (name : String)
  deriving DecidableEq, Repr
/-- A flavor word [CR#207.2d], as printed. Each is the card's own, and the CR lists none, so the
printed word is all it is. -/
abbrev FlavorWordLabel := String
/-- A vote's option, as the card prints it ("grace", "condemnation") [CR#701.38a]: a card's own
words, read back by the same card ("the option with the most votes"). -/
abbrev VoteLabel := String

/-- A keyword action's name [CR#701.1]. The keyword actions are an open set the registry
declares, so a deed of that source is keyed by the declaration's name verbatim (`exile`,
`collectEvidence`), as a designation is. An enacted deed whose label no declaration has is
refused (`knownAct`), and a participle naming one reaches nothing. -/
abbrev KeywordActionLabel := String

/-- Deeds the core rules define outside the keyword actions of [CR#701.1]: turn-based actions
[CR#508.1,509.1], tapping a permanent for mana [CR#106.12a], and the other actions the rules
define in their own sections. Closed, so a law matches them structurally instead of naming a
lexeme. -/
inductive CoreDeed where
  | attack | block | target | copy | draw | gainLife | loseGame | winGame | spend | trigger
  | put | return_ | gainControl | unlock | fullyUnlock | tapForMana
  deriving DecidableEq, Repr

/-- A condition that causes a state-based action [CR#704.1]. -/
inductive StateBasedCause where
  /-- A player has 0 or less life [CR#704.5a]. -/
  | nonpositiveLife
  deriving DecidableEq, Repr

/-- A deed: something a card's text says is done. Three sources define one, and only the middle
one is open: the core rules [CR#508.1,509.1,709.5f], the keyword actions the registry declares
[CR#701.1], and the verb a keyword ability defines for itself [CR#702.122b,702.171a,702.26a]. -/
inductive Deed where
  | core (deed : CoreDeed)
  | action (label : KeywordActionLabel)
  | ofAbility (keyword : KeywordLabel)
  deriving DecidableEq, Repr

inductive PileFace where
  | faceDown | faceUp
  deriving DecidableEq, Repr

inductive ChoiceSort where
  | quality (sort : QualitySort)
  | player
  deriving DecidableEq, Repr

/-- Which die a roll action or trigger names. -/
inductive DieSides where
  | sides (sides : Nat)
  | thoseDice
  | anyDie
  deriving DecidableEq, Repr

instance : AnyValue DieSides where
  any := .anyDie

inductive ChoiceRef where
  | theChoice | theLatestChoice
  deriving DecidableEq, Repr

inductive Disclosure where
  | openly | secretly
  deriving DecidableEq, Repr

inductive HiddenSort where
  | numbers | choices
  deriving DecidableEq, Repr

inductive ExposeVerb where
  | lookAt | reveal
  deriving DecidableEq, Repr

/-- A card subtype. Instants and sorceries share their spell types [CR#205.3k]. -/
inductive Subtype where
  | of (host : CardType) (label : String)
  | spell (label : String)
  deriving DecidableEq, Repr

/-- The noun a reference is written with: a pronoun's ("that player", "that card"), or a Head
Noun, the Object Class, Card Type or Subtype a reference names its referent by ("this
creature", "this spell", "this Aura", "the emblem that granted it") [CR#109.1,205.2a,205.3]. -/
inductive NounWord where
  | type (type : CardType)
  /-- A subtype, read as its card type: "this Aura", "this Equipment" [CR#205.3]. -/
  | subtype (subtype : Subtype)
  | card
  | spell
  | player
  | permanent
  | token
  | emblem
  | copy
  | join
  /-- "Counter target spell or ability. … THAT SPELL OR ABILITY" -/
  | stack
  /-- "Whenever you activate an ability, … copy THAT ABILITY" -/
  | ability
  /-- A written type narrows the word without replacing its carrier requirement. -/
  | ofType (word : NounWord) (type : CardType)
  /-- Copy origin narrows the word; an ability copy still has the ability head [CR#707.10]. -/
  | copied (word : NounWord)
  /-- "Put THAT PILE into your hand and the other into your graveyard." -/
  | pile
  deriving DecidableEq, Repr

inductive SlotCarrier where
  | permanent | card | spell
  deriving DecidableEq, Repr

/-- How a pronoun is written: what it reaches back for. -/
inductive Reach where
  | bare
  | atSlot (slot : SlotCarrier)
  | word (word : NounWord)
  | unionHalf (word : NounWord)
  deriving DecidableEq, Repr

/-- A Binding Identity: the name under which a Definition or Template body's first mention of a referent
parameter records the binding it introduces or resolves, and by which the body's later mentions
of that parameter read it back. It is a name, fixed when the mention is checked, never a stack
position: pushing and removing other bindings does not change which binding it names. -/
abbrev BindingId := String

inductive EntryCounterMark where
  | fresh | additional | fewer
  deriving DecidableEq, Repr

inductive RoundMode where
  | up | down
  deriving DecidableEq, Repr

inductive ScaleFactor where
  | doubled | tripled
  deriving DecidableEq, Repr

/-- How much of a spell's or ability's targets a "targets …" check reads [CR#115.9]: some target
("that targets [something]" [CR#115.9b]), its sole target ("that targets only a single
[something]"), or the subject alone ("that targets only it"); both "only" readings count the
different objects and players chosen as targets [CR#115.9c]. -/
inductive TargetExtent where
  | someTarget | soleTarget | only
  deriving DecidableEq, Repr

inductive CopySort where
  | fromStack | fromCardZone
  deriving DecidableEq, Repr

/-- A family of keywords ("a landwalk ability"). -/
structure KeywordFamily where
  word : KeywordLabel
  sort : Option QualitySort
  deriving DecidableEq, Repr

inductive KeywordTerm where
  | the (keyword : KeywordLabel)
  | anyIn (family : KeywordFamily)
  /-- A keyword named with its number, as "rampage 3" is. -/
  | theWith (keyword : KeywordLabel) (number : Nat)
  deriving DecidableEq, Repr

/-- A Link Source: the cost a Linked Facet reads back, an additional cost [CR#118.8,607.2i] or an
alternative cost [CR#118.9,607.1]. A cost is named by its keyword ("its kicker cost"), by its
keyword and rank ("its second kicker cost"), or as "the alternative" or "the additional" cost. -/
inductive LinkSource where
  | byKeyword (keyword : KeywordLabel)
  | byNthKeyword (ordinal : Ordinal) (keyword : KeywordLabel)
  | theAlternative
  | theAdditional
  deriving DecidableEq, Repr

inductive AbilityClass where
  | anyOnStack
  | anyActivated
  | anyTriggered
  | loyalty
  /-- A mana ability, activated or triggered [CR#605.1]. -/
  | mana
  | keyword (k : KeywordLabel)
  deriving DecidableEq, Repr

/-- The italic word that may head an ability: an ability word [CR#207.2c] or a flavor word
[CR#207.2d]. Neither has rules meaning; both are text a card prints over the ability
(`Ability.italicHead`), and the CR tells them apart (it lists the ability words and no flavor
words), so the head records which it is. -/
inductive ItalicWord where
  | abilityWord (label : AbilityWordLabel)
  | flavorWord (label : FlavorWordLabel)
  deriving DecidableEq, Repr

inductive Color where
  | white | blue | black | red | green
  deriving DecidableEq, Repr

inductive ColorOrColorless where
  | colorless
  | of (color : Color)
  deriving DecidableEq, Repr

inductive SimpleManaSymbol where
  | generic (amount : Nat)
  | specific (color : ColorOrColorless)
  deriving DecidableEq, Repr

inductive ManaSymbol where
  | simple (symbol : SimpleManaSymbol)
  | hybrid (left : SimpleManaSymbol) (right : Color)
  | phyrexian (color : Color) (second : Option Color)
  | variable
  | snow
  deriving DecidableEq, Repr

abbrev ManaCost := List ManaSymbol

/-- One run of produced mana, e.g. `{G}{G}` or `{C}`. -/
abbrev ProducedRun := List ColorOrColorless

/-- The type space a "with every … type" quality ranges over. -/
inductive SubtypeSpace where
  | basicLand | land | creature
  deriving DecidableEq, Repr

inductive PayTimes where
  | once
  | anyNumberOfTimes
  | upTo (times : Nat)
  deriving DecidableEq, Repr

inductive CostNamed where
  | containing (symbol : ManaSymbol)
  | ofKeyword (keyword : KeywordLabel)
  deriving DecidableEq, Repr

inductive ColorFreedom where
  | sameColor | eachColor | distinctColors
  deriving DecidableEq, Repr

inductive ManaMatch where
  | anyColor
  | anyType
  | of (color : ColorOrColorless)
  deriving DecidableEq, Repr

/-- A change of a quantity: up by, down by, or set to. -/
inductive Delta (α : Type) where
  | up (amount : α)
  | down (amount : α)
  | set (amount : α)
  deriving DecidableEq, Repr, BEq

inductive Parity where
  | even | odd
  deriving DecidableEq, Repr

inductive Supertype where
  | legendary | basic | snow | world
  deriving DecidableEq, Repr

/-- A half of a split permanent with a shared type line, a Room's door [CR#709.5c,709.5j]. Not a
vocabulary item a registry declares, as keywords, keyword actions and counters are: a split card
has two faces [CR#709.1], its left and right halves [CR#709.5c], so the halves are closed, as a
card's sides (`CardFaceSide`) are. -/
inductive RoomHalf where
  | left | right
  deriving DecidableEq, Repr

inductive LockState where
  | locked | unlocked
  deriving DecidableEq, Repr

/-- Which endpoint of an attachment relation the predicate describes. -/
inductive AttachmentSide where
  | host | attachment
  deriving DecidableEq, Repr

/-- The current side of a single double-faced permanent, independently of its Status. -/
inductive CardFaceSide where
  | front | back
  deriving DecidableEq, Repr

inductive AttachWord where
  | enchanted | equipped | fortified
  deriving DecidableEq, Repr

/-- How a game concludes for whoever performs it: a win, a loss, or a draw ("the game is a
draw" [CR#104.4c]). -/
inductive OutcomeVerb where
  | winGame | loseGame | drawGame
  deriving DecidableEq, Repr

/-- Which of power and toughness a characteristic-defining ability writes ("its power is equal
to …", "its power and toughness are each equal to …"): an authoring input to `ptDefinition`,
which expands it to `CharacteristicEdit.stat` sets [CR#604.3a,613.4a]. -/
inductive DefinedSlots where
  | powerAlone | toughnessAlone | bothEach
  deriving DecidableEq, Repr

/-- A counter's kind [CR#122.1]: the NAME of the counter declaration that defines it, verbatim
(`p1p1Counter`, `flyingCounter`, `chargeCounter`). "Counters with the same name or description
are interchangeable" [CR#122.1], so the name is all a kind is. What a counter of the kind does
is its Registry Definition (`Definition.counter`: holder and conferrals), and which kinds exist
is the counter facts table generated from those definitions, so a +1/+1 counter [CR#122.1a] and
a keyword counter [CR#122.1b] are looked up exactly as a charge counter is. -/
inductive CounterKind where
  | named (name : String)
  deriving DecidableEq, Repr

inductive ProjAxis where
  | stat (stat : Stat)
  | playerStat (stat : PlayerStat)
  | counter (kind : CounterKind)
  | anyCounter (kind : Kind)
  /-- The counters on an object or a player of the one counter kind a kind loop or a choice
  binds ("each kind of counter on it"), and of no other kind: doubling gives as many of that kind
  as it already has [CR#701.10e]. -/
  | boundCounter
  /-- Damage marked on a permanent [CR#120.3e,120.6,704.5g]. -/
  | markedDamage
  /-- The chapter numbers of an object's own chapter abilities [CR#714.2a,714.2d]: a Saga
  projects one per chapter ability it has, and a chapter ability projects its own. Several
  values or none, so it is read through a fold: "the final chapter number" is the greatest of
  them, `aggregate max chapter this` [CR#714.2d]. -/
  | chapter
  deriving DecidableEq, Repr

/-- A Saga's chapter, counted from I [CR#714.2a]. -/
abbrev ChapterNumber := Nat

/-- Physical permanent statuses across the four status categories [CR#110.5]. -/
inductive Status where
  | tapped | untapped | flipped | unflipped | faceUp | faceDown | phasedIn | phasedOut
  deriving DecidableEq, Repr

inductive ColorSpec where
  | some (colors : List Color)
  | every
  deriving DecidableEq, Repr

/-- Phases and steps [CR#500.1]; `firstStrikeCombatDamage` is the extra step of [CR#510.4]. -/
inductive TurnPart where
  | turn | beginningPhase | untapStep | upkeep | drawStep | mainPhase | firstMain | combat
  /-- The combat phase's own first step [CR#506.1], which the phase around it is not. -/
  | beginningOfCombat
  | declareAttackers | declareBlockers | firstStrikeCombatDamage | combatDamage | endOfCombat
  | postcombatMain
  /-- The turn's fifth phase [CR#500.1], the one holding the end and cleanup steps. -/
  | endingPhase
  | endStep | cleanup
  deriving DecidableEq, Repr

/-- What a usage limit or an event ordinal counts within: each turn, each of its player's turns,
each game, or each occurrence of a phase or step [CR#602.5b]. -/
inductive Period where
  | turn | yourTurn | game
  | part (part : TurnPart)
  /-- The object's own existence: "activate only once" counts until the object changes zones,
  when it becomes a new object [CR#602.5b,702.177a,400.7]. -/
  | thisObject
  deriving DecidableEq, Repr

/-- Which side of an event a timing names: "before attackers are declared", "after combat". -/
inductive EventSide where
  | before | after
  deriving DecidableEq, Repr

end Semantics

attribute [semantic_expression] Semantics.Delta

attribute [semantic_literal] Semantics.SimpleManaSymbol.generic
