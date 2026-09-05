/-!
# Experimental.Words

The vocabulary: every leaf type a phrase, trigger, effect, or card is written from. Port of
`idris/src/Experimental/Words.idr`, syntax only.

A type is here iff a constructor field mentions it. The checker's own vocabulary (the
antecedent stack, obligation witnesses, the facts tables, event names, static kinds) lives in
`Check/`. Designations, keywords, acts, counters, and deeds are labels with generated facts
tables, not enums: the grammar carries the mechanism, the tables carry the set.
-/

namespace Mtg

/-- [CR#205.2a] -/
inductive CardType where
  | creature | artifact | land | enchantment | instant | sorcery | planeswalker | battle | kindred
  deriving DecidableEq, Repr

inductive Stat where
  | power | toughness | manaValue | loyalty
  deriving DecidableEq, Repr

inductive PlayerStat where
  | lifeTotal | startingLifeTotal
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
  | gap
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

inductive OutcomeSort where
  | damageDealt | lifeGained | lifeLost | countersPut | damagePrevented | rollResult
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

inductive NameAgreement where
  | differentNames | sameName
  deriving DecidableEq, Repr

inductive Determiner where
  | target | a | each | all | the | part | count | self | bare
  deriving DecidableEq, Repr

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

inductive LibraryEnd where
  | top | bottom
  deriving DecidableEq, Repr

inductive Arrangement where
  | anyOrder | randomOrder
  deriving DecidableEq, Repr

/-- The N of "Nth" [CR#401.7]; the Idris carries `IsSucc n`. -/
inductive Ordinal where
  | nth (n : Nat)
  deriving DecidableEq, Repr

/-- A designation's name [CR#701.15b,725.1,731.1]. Which designations exist, and what each
holds and does, is the designation facts table, generated like the keyword table; the grammar
carries only the mechanism. -/
abbrev DesignationLabel := String

abbrev VerbLabel := String
abbrev KeywordLabel := String
abbrev AbilityWordLabel := String
abbrev FlavorWordLabel := String
abbrev VoteLabel := String

inductive PileFace where
  | faceDown | faceUp
  deriving DecidableEq, Repr

inductive ChoiceSort where
  | quality (sort : QualitySort)
  | player
  deriving DecidableEq, Repr

/-- Which die a roll instruction names. -/
inductive DieSides where
  | sides (sides : Nat)
  | thoseDice
  deriving DecidableEq, Repr

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

/-- The noun a pronoun ("that player", "that card") is written with. -/
inductive NounWord where
  | type (type : CardType)
  | card
  | spell
  | player
  | permanent
  | token
  | copy
  | join
  /-- "Counter target spell or ability. … THAT SPELL OR ABILITY" -/
  | stack
  /-- "Whenever you activate an ability, … copy THAT ABILITY" -/
  | ability
  /-- "Copy target triggered ability you control. You may choose new targets for THE COPY" -/
  | abilityCopy
  /-- "the exiled creature card", "that land card": the type narrows the card word. -/
  | typedCard (type : CardType)
  /-- "Put THAT PILE into your hand and the other into your graveyard." -/
  | pile
  deriving DecidableEq, Repr

inductive VerbedMarking where
  | attributive | thisWay
  deriving DecidableEq, Repr

inductive SlotCarrier where
  | permanent | card | spell
  deriving DecidableEq, Repr

/-- How a pronoun is written: what it reaches back for. -/
inductive Reach where
  | bare
  | atSlot (slot : SlotCarrier)
  | stamped (verb : VerbLabel)
  | tokenBorn
  | word (word : NounWord)
  | unionHalf (word : NounWord)
  | verbed (verb : VerbLabel) (word : NounWord) (marking : VerbedMarking)
  | thatTurn
  deriving DecidableEq, Repr

/-- The stretch of the antecedent stack a pronoun resolves in. -/
inductive Window where
  | whole
  | top (depth : Nat)
  | below (depth : Nat)
  deriving DecidableEq, Repr

inductive EntryCounterMark where
  | fresh | additional | fewer
  deriving DecidableEq, Repr

inductive PlayerGroupWord where
  | allPlayers | yourOpponents | yourTeam
  deriving DecidableEq, Repr

inductive RoundMode where
  | up | down
  deriving DecidableEq, Repr

inductive ScaleFactor where
  | doubled | tripled
  deriving DecidableEq, Repr

inductive ShiftDir where
  | up | down
  deriving DecidableEq, Repr

inductive Lookback where
  | thisTurn | earlierThisTurn | thisCombat | lastTurn | thisGame | thisWay | triggering
  deriving DecidableEq, Repr

inductive TargetExtent where
  | someTarget | soleTarget
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

inductive PaidCostName where
  | byKeyword (keyword : KeywordLabel)
  | byNthKeyword (ordinal : Ordinal) (keyword : KeywordLabel)
  | theAlternative
  | theAdditional
  deriving DecidableEq, Repr

inductive PaidFacet where
  /-- colors of mana spent to cast it [CR#702.44a] -/
  | colorsSpent
  /-- mana spent to pay the total cost [CR#601.2h] -/
  | manaValueSpent
  /-- [CR#702.33c..702.33d] -/
  | timesPaid (which : PaidCostName)
  /-- "was kicked" [CR#702.33d] -/
  | readback (which : PaidCostName) (lookback : Option Lookback)
  deriving DecidableEq, Repr

inductive AbilityClass where
  | anyOnStack
  | anyActivated
  | anyTriggered
  | loyalty
  | keyword (k : KeywordLabel)
  deriving DecidableEq, Repr

-- this seems odd
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

/-- Which side of a deed a deontic rule speaks about. -/
inductive Role where
  | agent | patient
  deriving DecidableEq, Repr

inductive PayTimes where
  | once
  | anyNumberOfTimes
  | upTo (times : Nat)
  deriving DecidableEq, Repr

inductive ManaUnit where
  | generic
  | run (cost : ManaCost)
  deriving DecidableEq, Repr

inductive SpecialAction where
  | turnFaceUp | putCompanionIntoHand | foretell | unlockDoor
  deriving DecidableEq, Repr

inductive CostNamed where
  | containing (symbol : ManaSymbol)
  | ofKeyword (keyword : KeywordLabel)
  | ofSpecialAction (action : SpecialAction)
  deriving DecidableEq, Repr

inductive ColorFreedom where
  | sameColor | eachColor | distinctColors
  deriving DecidableEq, Repr

inductive ManaMatch where
  | anyColor
  | anyType
  | of (color : ColorOrColorless)
  deriving DecidableEq, Repr

inductive LoyaltyCost where
  | up (amount : Nat)
  | down (amount : Nat)
  | downX
  | zero
  deriving DecidableEq, Repr

/-- A card subtype. Instants and sorceries share their spell types [CR#205.3k]. -/
inductive Subtype where
  | of (host : CardType) (label : String)
  | spell (label : String)
  deriving DecidableEq, Repr

inductive MarkerWord where
  | token | emblem | spell | permanent | ability
  deriving DecidableEq, Repr

/-- A change of a quantity: up by, down by, or set to. -/
inductive Delta (α : Type) where
  | up (amount : α)
  | down (amount : α)
  | set (amount : α)
  deriving DecidableEq, Repr, BEq

inductive Supertype where
  | legendary | basic | snow | world
  deriving DecidableEq, Repr

-- this doesn't seem like it should be enum-sourced like this. keywords, actions, counters are not
inductive RoomHalf where
  | left | right
  deriving DecidableEq, Repr

inductive LockState where
  | locked | unlocked
  deriving DecidableEq, Repr

/-- How a designation was conferred: by an instruction, or in a keyword's expansion, named
by the keyword. -/
inductive Conferral where
  | instructed
  | byKeyword (keyword : KeywordLabel)
  deriving DecidableEq, Repr

inductive AttachWord where
  | enchanted | equipped | fortified
  deriving DecidableEq, Repr

inductive OutcomeVerb where
  | winGame | loseGame
  deriving DecidableEq, Repr

inductive DefinedSlots where
  | powerAlone | toughnessAlone | bothEach
  deriving DecidableEq, Repr

inductive CounterKind where
  | boost (power toughness : Delta Nat)
  | keyword (keyword : KeywordLabel)
  | named (label : String)
  deriving DecidableEq, Repr

inductive ProjAxis where
  | stat (stat : Stat)
  | playerStat (stat : PlayerStat)
  | counter (kind : CounterKind)
  | anyCounter (kind : Kind)
  deriving DecidableEq, Repr

/-- A Saga's chapter, counted from I [CR#714.2]. -/
abbrev ChapterNumber := Nat

/-- Each status category always has exactly one of its two values [CR#110.5]. -/
inductive StatusCat where
  | tap | flip | face | phase
  deriving DecidableEq, Repr

/-- The Idris `StatusVal : StatusCat → Type`, unindexed; `category` recovers the index. -/
inductive Status where
  | tapped | untapped | flipped | unflipped | faceUp | faceDown | phasedIn | phasedOut
  deriving DecidableEq, Repr

def Status.category : Status → StatusCat
  | .tapped | .untapped => .tap
  | .flipped | .unflipped => .flip
  | .faceUp | .faceDown => .face
  | .phasedIn | .phasedOut => .phase

inductive ColorSpec where
  | some (colors : List Color)
  | every
  deriving DecidableEq, Repr

/-- Phases and steps [CR#500.1]; `firstStrikeCombatDamage` is the extra step of [CR#510.4]. -/
inductive TurnPart where
  | turn | upkeep | endStep | combat | untapStep | endOfCombat | firstMain | postcombatMain
  | drawStep | mainPhase | beginningPhase | declareAttackers | declareBlockers
  | firstStrikeCombatDamage | combatDamage | cleanup
  deriving DecidableEq, Repr

inductive RankPeriod where
  | within (lookback : Lookback)
  | each (part : TurnPart)
  deriving DecidableEq, Repr

inductive PartQuant where
  | the | each
  deriving DecidableEq, Repr

end Mtg
