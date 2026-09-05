/-!
# Experimental.Words

The vocabulary: every leaf type a phrase, trigger, effect, or card is written from. Port of
`idris/src/Experimental/Words.idr`, syntax only.

Not ported, by design (checker machinery, not syntax): the antecedent stack and its payloads
(`Binding`, `Bindings`, `Payload`, `HeadTy`, `Stamp`, `Origin`), the obligation witnesses
(`OptOk`, `Joins`, `Targetable`, `Targeter`, `Phrasal`, `AxesAt`, `KnownAct`, `KnownKeyword`,
`BasicLandType(s)`, `DesignationHolder`), and the facts tables and their rows (`ActFacts`,
`DeedRole`, `DesignationFacts`, `CounterFacts`, `DeedFeature`, `PremiseSort`).

Naming: types UpperCamelCase, constructors lowerCamelCase in the type's namespace, so the
Idris anti-collision suffixes (`PlayerW`, `TapC`, `OwnerAx`) are gone: `NounWord.player`,
`StatusCat.tap`, `PossessorAxis.owner`.
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
  deriving DecidableEq, Repr

inductive Letter where
  | x | y
  deriving DecidableEq, Repr

/-- The sort of thing a phrase denotes. -/
inductive Kind where
  | object
  | player
  | quality (q : QualitySort)
  | outcome
  | gap
  | letter (l : Letter)
  | turnRef
  /-- A pile of objects; the pile itself is not an object [CR#700.3b]. -/
  | pile
  /-- Idris `(\/)`: a phrase denoting either sort ("target creature or player"). -/
  | join (a b : Kind)
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
  | subtype (host : CardType) (only : SubtypeScope)
  | value (stat : Stat)
  | counterKind
  | colorPair
  deriving DecidableEq, Repr

inductive OutcomeSort where
  | damageDealt | lifeGained | lifeLost | countersPut | damagePrevented | rollResult
  | coinFlipped | diceRolled | planarRolled | namedNumber | repeatCount | countersRemoved
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

inductive CombatRelation where
  | blockerOf | blockedBy | attackedBy | attackerOf | couldBlock | couldBeBlockedBy
  deriving DecidableEq, Repr

inductive Zone where
  | battlefield | graveyard | exile | hand | library | stack | command
  deriving DecidableEq, Repr

inductive LibPos where
  | top | bottom
  deriving DecidableEq, Repr

inductive Arrangement where
  | anyOrder | randomOrder
  deriving DecidableEq, Repr

/-- The N of "Nth" [CR#401.7]; the Idris carries `IsSucc n`. -/
inductive Ordinal where
  | nth (n : Nat)
  deriving DecidableEq, Repr

abbrev VerbLabel := String
abbrev KeywordLabel := String
abbrev AbilityWordLabel := String
abbrev FlavorWordLabel := String
abbrev VoteLabel := String

inductive PileFace where
  | faceDown | faceUp
  deriving DecidableEq, Repr

inductive ChoiceSort where
  | quality (q : QualitySort)
  | player
  deriving DecidableEq, Repr

/-- Which die a roll instruction names. -/
inductive DieSides where
  | sides (n : Nat)
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
  | type (t : CardType)
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
  | typedCard (t : CardType)
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
  | word (w : NounWord)
  | unionHalf (w : NounWord)
  | verbed (verb : VerbLabel) (w : NounWord) (marking : VerbedMarking)
  | thatTurn
  deriving DecidableEq, Repr

/-- The stretch of the antecedent stack a pronoun resolves in. -/
inductive Window where
  | whole
  | top (n : Nat)
  | below (n : Nat)
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
  | the (k : KeywordLabel)
  | anyIn (family : KeywordFamily)
  deriving DecidableEq, Repr

inductive PaidCostName where
  | byKeyword (kw : KeywordLabel)
  | byNthKeyword (ord : Ordinal) (kw : KeywordLabel)
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
  | readback (which : PaidCostName) (window : Option Lookback)
  deriving DecidableEq, Repr

inductive AbilityClass where
  | anyOnStack
  | anyActivated
  | anyTriggered
  | loyalty
  | keyword (k : KeywordLabel)
  deriving DecidableEq, Repr

inductive ItalicWord where
  | abilityWord (label : AbilityWordLabel)
  | flavorWord (label : FlavorWordLabel)
  deriving DecidableEq, Repr

inductive Color where
  | white | blue | black | red | green
  deriving DecidableEq, Repr

inductive ColorOrColorless where
  | colorless
  | of (c : Color)
  deriving DecidableEq, Repr

inductive SimpleManaSymbol where
  | generic (n : Nat)
  | specific (c : ColorOrColorless)
  deriving DecidableEq, Repr

inductive ManaSymbol where
  | simple (s : SimpleManaSymbol)
  | hybrid (l : SimpleManaSymbol) (r : Color)
  | phyrexian (c : Color) (d : Option Color)
  | variable
  | snow
  deriving DecidableEq, Repr

abbrev ManaCost := List ManaSymbol

/-- One run of produced mana, e.g. `{G}{G}` or `{C}`. -/
abbrev ProducedRun := List ColorOrColorless

/-- The type space a "with every … type" quality ranges over. -/
inductive TypeSpace where
  | basicLand | land | creature
  deriving DecidableEq, Repr

/-- Which side of a deed a deontic rule speaks about. -/
inductive Role where
  | agent | patient
  deriving DecidableEq, Repr

inductive PayTimes where
  | once
  | anyNumberOfTimes
  | upTo (n : Nat)
  deriving DecidableEq, Repr

inductive ManaUnit where
  | generic
  | run (cost : ManaCost)
  deriving DecidableEq, Repr

inductive SpecialAction where
  | turnFaceUp | putCompanionIntoHand | foretell | unlockDoor
  deriving DecidableEq, Repr

inductive CostNamed where
  | containing (sym : ManaSymbol)
  | ofKeyword (kw : KeywordLabel)
  | ofSpecialAction (act : SpecialAction)
  deriving DecidableEq, Repr

inductive ColorFreedom where
  | sameColor | eachColor | distinctColors
  deriving DecidableEq, Repr

inductive ManaMatch where
  | anyColor
  | anyType
  | of (c : ColorOrColorless)
  deriving DecidableEq, Repr

inductive LoyaltyCost where
  | up (n : Nat)
  | down (n : Nat)
  | downX
  | zero
  deriving DecidableEq, Repr

/-- A card subtype. Instants and sorceries share their spell types [CR#205.3k]. -/
inductive Subtype where
  | of (host : CardType) (label : String)
  | spell (label : String)
  deriving DecidableEq, Repr

inductive MarkerWord where
  | token | emblem | spell | permanent
  deriving DecidableEq, Repr

/-- A change of a quantity: up by, down by, or set to. -/
inductive Delta (α : Type) where
  | up (x : α)
  | down (x : α)
  | set (x : α)
  deriving DecidableEq, Repr, BEq

inductive Supertype where
  | legendary | basic | snow | ongoing | world
  deriving DecidableEq, Repr

inductive Designation where
  -- player-held [CR#725.1,726.1,702.131c,702.195b]
  | monarch | theInitiative | citysBlessing | enduringStory
  -- object-held: the permanent markers [CR#701.15b,701.54b,701.64b,716.2b,719.3b]
  | goaded | ringBearer | monstrous | renowned | suspected | saddled
  | prepared | harnessed | level | solved
  -- object-held: the three sector designations [CR#702.158b]
  | alphaSector | betaSector | gammaSector
  -- object-held: the unlocked pair [CR#709.5c]
  | leftHalfUnlocked | rightHalfUnlocked
  -- card-held [CR#903.3]
  | commander
  -- game-held [CR#731.1]
  | day | night
  deriving DecidableEq, Repr

inductive DesignationScope where
  | heldBy (k : Kind)
  | heldByCard
  | heldByGame
  deriving DecidableEq, Repr

inductive RoomHalf where
  | left | right
  deriving DecidableEq, Repr

inductive LockState where
  | locked | unlocked
  deriving DecidableEq, Repr

inductive ConferringWord where
  | monstrosity | saddle | ascend | storied | renown
  deriving DecidableEq, Repr

/-- How a designation was conferred: by instruction, or in a keyword's expansion. -/
inductive GivingWarrant where
  | instructed
  | inExpansionOf (w : ConferringWord)
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
  | keyword (k : KeywordLabel)
  | named (label : String)
  deriving DecidableEq, Repr

inductive ProjAxis where
  | stat (s : Stat)
  | playerStat (s : PlayerStat)
  | counter (k : CounterKind)
  | anyCounter (k : Kind)
  deriving DecidableEq, Repr

inductive ChapterNumber where
  | i | ii | iii | iv | v | vi
  deriving DecidableEq, Repr

/-- The type line: supertypes, card types, subtypes [CR#205.1]. The Idris carried the
supertypes beside the line in each place a line appears; here they are on it. -/
structure TypeLine where
  supertypes : List Supertype
  types : List CardType
  subtypes : List Subtype
  deriving DecidableEq, Repr

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

inductive TurnPart where
  | turn | upkeep | endStep | combat | untapStep | endOfCombat | firstMain | postcombatMain
  | drawStep | mainPhase | beginningPhase | declareAttackers | declareBlockers | combatDamage
  | cleanup
  deriving DecidableEq, Repr

inductive RankPeriod where
  | within (w : Lookback)
  | each (part : TurnPart)
  deriving DecidableEq, Repr

inductive PartQuant where
  | the | each
  deriving DecidableEq, Repr

end Mtg
