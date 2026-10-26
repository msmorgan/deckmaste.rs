import Semantics.Authoring

/-!
# Semantics.Words

The vocabulary: every leaf type a phrase, trigger, effect, or card is written from. Port of
`idris/src/Experimental/Words.idr`, syntax only.

A type is here iff a constructor field mentions it. The checker's own vocabulary (the
antecedent stack, obligation witnesses, event names, static kinds) lives in `Check/`. Card types,
subtypes, designations, keywords, keyword actions and counters are not enums: each member is a
declaration in `Semantics.Definitions` whose value carries its own fields, so the grammar carries
the mechanism and the declarations carry the set.
-/

namespace Semantics

/-- Polymorphic wildcard value (`any`) across action-clause subjects, noun phrases, amounts,
counter kinds, dice, roll watches, zones, damage kinds, and timings. -/
class AnyValue (α : Type) where
  any : α

export AnyValue (any)

/-- A rules procedure: one a card can instruct a player to carry out during resolution, activating
an ability [CR#701.2a,602.1], casting a spell [CR#701.5a,601.2], or playing a card as a land or as
a spell [CR#701.18a,701.18b]; or a spell's or ability's resolution [CR#608.1,608.2], which no
player carries out but which an event can name ("during its resolution"), as the final part of
which an instant or sorcery spell is put into its owner's graveyard [CR#608.2n]. -/
inductive Procedure where
  | activate | cast | play | resolve
  deriving DecidableEq, Repr

/-- An object's numeric characteristic or calculated value [CR#109.3,202.3,208.1,306.5,310.4,310.4a]. -/
inductive Stat where
  | power | toughness | manaValue
  /-- Loyalty is read by zone: on the battlefield it is the number of loyalty counters on the
  planeswalker [CR#306.5c], and elsewhere the number printed on the card [CR#306.5a]. -/
  | loyalty
  /-- Defense is read by zone: on the battlefield it is the number of defense counters on the
  battle [CR#310.4c], and elsewhere the number printed on the card [CR#310.4a]. -/
  | defense
  /-- A creature's base power or base toughness [CR#208.4]: its power or toughness after
  characteristic-defining abilities and effects that set power and/or toughness, ignoring other
  effects and counters [CR#208.4b,613.4b]. -/
  | basePower | baseToughness
  deriving DecidableEq, Repr

/-- A card type [CR#205.2a]: its name and the fields the rules read off it, whether it is a
permanent type [CR#110.4], the characteristic stats its objects have [CR#302.4,306.5,310.4],
and whether its cards are instant or sorcery cards [CR#110.4a]. A card names a declared card
type (`Semantics.Definitions.CardTypes`), never the raw structure: `spelled` refuses it. What a
card type confers on its objects is not a field; it is the type's `RegistryDefinition.cardType` row. -/
structure CardType where
  /-- The word the type line prints. -/
  name : String
  /-- A permanent type: one of the six whose objects can be permanents [CR#110.4]. -/
  permanent : Bool
  /-- The characteristic stats only objects of this type have: power and toughness for a
  creature [CR#302.4], loyalty for a planeswalker [CR#306.5], defense for a battle [CR#310.4]. -/
  stats : List Stat := []
  /-- An instant or sorcery card [CR#110.4,304.1,307.1]. With `permanent`, it settles a card's
  Card Class [CR#110.4a]; a type that is neither, as kindred [CR#308.1] and dungeon
  [CR#309.2c] are, gives a card no Card Class of its own. -/
  instantOrSorcery : Bool := false
  deriving DecidableEq, Repr

/-- A card's Card Class: a permanent card or an instant or sorcery card, by its card types
[CR#110.4,110.4a]. A keyword the rules define one way on a permanent and another on an instant or
sorcery spell selects its half by it (`Ability.byCardClass`). -/
inductive CardClass where
  | permanentCard | instantOrSorceryCard
  deriving DecidableEq, Repr

/-- The Card Class of a card with card types `tys`, read off their fields: an instant or sorcery
card if any of them is an instant or sorcery type, a permanent card if any is a permanent type
[CR#110.4,110.4a], and none otherwise. A kindred card reads by its other card type [CR#308.1]; a
dungeon card has no Card Class [CR#309.2c]. -/
def cardClassOf (tys : List CardType) : Option CardClass :=
  if tys.any (·.instantOrSorcery) then some .instantOrSorceryCard
  else if tys.any (·.permanent) then some .permanentCard
  else none

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

/-- The sort of an action outcome read back by `thatMuch`, `thatMany` or `theOutcome`
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
[CR#201.2a,201.2c,105.2,205.3m]. On names `differ` also needs a name: a nameless object
does not have a different name [CR#201.2c]. -/
inductive Agreement where
  | share | differ
  deriving DecidableEq, Repr

/-- Sharing over one axis's values: `share` holds when the two value lists meet, `differ` when
they do not. On names this is not equality: an object with two names shares a name with an
object that has either [CR#201.2a]. On names `differ` also requires the first object to have a
name: one with none does not have a different name than any object [CR#201.2c]. -/
def Agreement.holdsOn {α : Type} [BEq α] (axis : QualitySort) :
    Agreement → List α → List α → Bool
  | .share, mine, theirs => mine.any theirs.contains
  | .differ, mine, theirs =>
    (axis != .cardName || !mine.isEmpty) && !mine.any theirs.contains

/-- Agreement over a group's value lists, one list per member. `share` holds when one value is
common to every member: on names [CR#201.2a], and on any axis by the Littjara Kinseekers ruling
(2021-02-05, "at least one creature type all three have") and the Common Cause ruling
(2004-10-04, "they all have to share one color"); `differ` when no two members have a value in
common, and on names only if each member has at least one name [CR#201.2b]. An empty group shares
no value. -/
def Agreement.holdsAmong {α : Type} [BEq α] (axis : QualitySort) :
    Agreement → List (List α) → Bool
  | .share, [] => false
  | .share, first :: rest => first.any fun value => rest.all (·.contains value)
  | .differ, group =>
    (axis != .cardName || group.all (!·.isEmpty)) && pairwiseDisjoint group
where
  pairwiseDisjoint : List (List α) → Bool
    | [] => true
    | first :: rest => rest.all (fun other => !first.any other.contains) && pairwiseDisjoint rest

inductive PossessorAxis where
  | owner | controller
  deriving DecidableEq, Repr

/-- How an object stands in combat, optionally toward a counterpart: "attacking" is
`attackerOf` with none, "creature blocking it" is `blockerOf` with one. `declaredAttacker` is
the declare-attackers moment [CR#508.1].

`blockedBy` with no counterpart ("blocked") and `unblocked` are the two statuses an attacking
creature gets when blockers are declared, and neither is the other's negation: before then an
attacking creature is neither [CR#509.1h]. Both are sticky. A blocked creature stays blocked
when every creature blocking it is removed from combat [CR#509.1h], and a creature put onto the
battlefield attacking during the declare blockers step or later enters unblocked [CR#508.4d].
Either status ends when the creature is removed from combat [CR#506.4] or the combat phase
ends, or when an effect says it becomes blocked or unblocked [CR#509.1h]. -/
inductive CombatRelation where
  | attackerOf | declaredAttacker | blockerOf | blockedBy | unblocked | attackedBy | couldBlock
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

/-- Who holds a designation [CR#701.15b,725.1,731.1]: an Entity of the named kind, the card
itself (a commander is designated on its card, and the designation follows the card between
zones [CR#903.3]), or the game. -/
inductive DesignationScope where
  | heldBy (holder : Kind)
  | heldByCard
  | heldByGame
  deriving DecidableEq, Repr

/-- A designation [CR#701.15b,725.1,731.1]: the NAME of the designation declaration that defines
it, verbatim (`goaded`, `monarch`, `ringBearer`), or, where a declaration lists several members,
the member's own name (`dayNight` lists `day` and `night`), with who holds it. The name is not
English: the phrase a card prints ("the monarch", "Ring-bearer") is the declaration's spelling.
Each designation is its own declaration in `Semantics.Definitions.Designations`, and the
constructor is an internal expansion, so a card names a declared designation or none; the
grammar carries only the mechanism. Plotted [CR#702.170a,702.170c] and foretold
[CR#702.143c,702.143d], states of a card in exile, are designations like the rest. `sole` is
whether at most one holder has it at a time, so "the" and the designation name one entity: only
one player can be the monarch [CR#725.3] or have the initiative [CR#726.3]; several players can
each have the city's blessing. -/
inductive DesignationLabel where
  | named (name : String) (scope : DesignationScope) (sole : Bool := false)
  deriving DecidableEq, Repr

/-- A Relation [CR#702.55b,702.95b,702.99b]: the NAME of the relation declaration that defines
it, verbatim (`haunting`, `paired`, `encodedOn`). Each relation is its own declaration in
`Semantics.Definitions.Relations`, and the constructor is an internal expansion, so a card names
a declared relation or none; the grammar carries only the mechanism. -/
inductive RelationLabel where
  | named (name : String)
  deriving DecidableEq, Repr

/-- Where a merging object is placed: on top of the permanent or under it [CR#730.2]. -/
inductive MergePosition where
  | top | bottom
  deriving DecidableEq, Repr

/-- A keyword ability [CR#702.1]: the NAME of the keyword ability declaration that defines it,
verbatim (`flying`, `firstStrike`), never the phrase a card prints ("first strike"), which is the
declaration's spelling. A type of its own rather than a `String`, as `DesignationLabel` is, so a
card names a keyword by its declaration's name, through the label constant the declaration
generates, and an undeclared name is no identifier at all. -/
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

/-- A keyword action's name [CR#701.1], as its Action Definitions write it (`exile`,
`collectEvidence`; `attach` for `attachTo`). A card never spells one: each Action Definition
generates its authored deed constant (`attachTo.deed`), which a card names by the Definition's own
name, so an undeclared action is no identifier at all. -/
abbrev KeywordActionLabel := String

/-- Deeds the core rules define outside the keyword actions of [CR#701.1], each one a deed the
checker stamps on what it acts on or records as a caused event: targeting [CR#115.1] ("the
targeted creature"), drawing [CR#121.1] ("the drawn card"), triggering [CR#603.2], gaining
control [CR#613.1b] and fully unlocking [CR#709.5i]. Closed, so a law matches them structurally
instead of naming a lexeme. Any other act the core rules define (attacking, blocking, copying, a
life change, tapping for mana) is its own `Action` head and names no deed here. "Put" and
"return" name none either: each is ordinary English for a Zone Change [CR#400.7], written as the
move itself. -/
inductive CoreDeed where
  | target | draw | trigger | gainControl | fullyUnlock
  deriving DecidableEq, Repr

/-- A condition that causes a state-based action [CR#704.1]. -/
inductive StateBasedCause where
  /-- A player has 0 or less life [CR#704.5a]. -/
  | nonpositiveLife
  deriving DecidableEq, Repr

/-- What a keyword action says of a permanent. -/
inductive DeedPermanent where
  /-- Nothing: a player performs it. -/
  | none
  /-- A permanent is instructed to perform it [CR#701.14a,701.44a,701.50a,701.63a]. -/
  | performs
  /-- It is an instruction about "this permanent" that the ability's controller follows
  [CR#701.46a,701.37a,608.2c]: "target creature adapts" is the adapt whose "this permanent" is the
  creature. -/
  | isAbout
  deriving DecidableEq, Repr

/-- A deed: something a card's text says is done. Three sources define one, and only the middle
one is open: the core rules [CR#115.1,121.1,709.5i], the keyword actions the Action Definitions
declare [CR#701.1], and the verb a keyword ability defines for itself
[CR#702.122b,702.171a,702.26a]. -/
inductive Deed where
  | core (deed : CoreDeed)
  /-- A keyword action, written by its Action Definition. `permanent`: what it says of a
  permanent; written with no Handoff to a Permanent, that permanent is the Source. -/
  | action (label : KeywordActionLabel) (permanent : DeedPermanent := .none)
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
  /-- "THAT SOURCE's controller": no Head Noun. It reads back only a referent introduced as a
  source ("a source of your choice", "whenever a source deals damage"), whatever its zone, as a
  source can be a permanent, a spell, a card an effect refers to, or a command-zone object
  [CR#609.7a]. It never reads the Source of the text. -/
  | source
  deriving DecidableEq, Repr

inductive SlotCarrier where
  | permanent | card | spell | offStack
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

/-- How much of a spell's or ability's targets a "targets …" check reads [CR#115.9]: some target
("that targets [something]" [CR#115.9b]), its sole target ("that targets only a single
[something]"), or the subject alone ("that targets only it"); both "only" readings count the
different objects and players chosen as targets [CR#115.9c]. -/
inductive TargetExtent where
  | someTarget | soleTarget | only
  deriving DecidableEq, Repr

inductive CopySort where
  | fromStack | fromCardZone
  /-- A copy of an object created in a zone the effect names, "create a copy of this object in
  exile" (paradigm [CR#702.192a]), not the zone the object is in. -/
  | inZone (zone : Zone)
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
is its Registry Definition (`RegistryDefinition.counter`: its conferrals). The kind carries what the
counter is placed on, an object unless the declaration says a player [CR#122.1]. Each kind is its
own declaration in `Semantics.Definitions.Counters`, and the constructor is an internal
expansion, so a card names a declared kind or none: a +1/+1 counter [CR#122.1a] and a keyword
counter [CR#122.1b] are declared exactly as a charge counter is. -/
inductive CounterKind where
  | named (name : String) (holder : Kind := .object)
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
  /-- The N of each `keyword N` ability an object has: several values or none, so it is read
  through a fold, as `chapter` is. A creature's total toxic value, the sum of the N values of
  its toxic abilities, is `aggregate sum (keywordNumber toxic) it` [CR#702.164b]. -/
  | keywordNumber (keyword : KeywordLabel)
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
each game, or each occurrence of a phase or step [CR#602.5b]. One type serves both: a Usage Limit
(`UsageLimit.mk`) and an event's ordinal (`GameEvent.nthOccurrence`, `WouldClause.nthOccurrence`,
`DidClause.ordinal`), which is not a Usage Limit. An event ordinal never counts
within `thisObject`, and spells "each turn" `turn`, not `part .turn`. -/
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

attribute [semantic_expression] Semantics.CardType Semantics.Delta Semantics.Subtype
  Semantics.CounterKind
  Semantics.DesignationLabel Semantics.RelationLabel Semantics.KeywordLabel
  Semantics.AbilityWordLabel
  Semantics.Deed

attribute [semantic_literal] Semantics.SimpleManaSymbol.generic

-- A `CardType`, `Subtype`, `CounterKind`, `DesignationLabel` or `RelationLabel` is a semantic
-- expression with no primitive wrapper: a card names a catalog entry
-- (`Semantics.Definitions.CardTypes`, `.Subtypes`, `.Counters`, `.Designations`, `.Relations`),
-- and `spelled` refuses a raw constructor.
attribute [internal_expansion] Semantics.CardType Semantics.Subtype Semantics.CounterKind
  Semantics.DesignationLabel Semantics.RelationLabel

-- A `KeywordLabel` is a semantic expression whose one constructor is internal: each keyword
-- ability Definition generates its authored label constant (`kicker.label`,
-- `Authoring.keywordLabelConstant`), which a card names by the Definition's own name, and
-- `spelled` refuses a raw `KeywordLabel.named`. An `AbilityWordLabel` is the same: a card writes
-- the ability word's Template (`landfall`), never the raw label.
attribute [internal_expansion] Semantics.KeywordLabel.named Semantics.AbilityWordLabel.named

-- A `Deed` is a semantic expression whose keyword-action constructor is internal: each Action
-- Definition generates its authored deed constant (`exile.deed`, `Authoring.deedConstant`), which
-- a card names by the Definition's own name, and `spelled` refuses a raw `Deed.action`.
attribute [internal_expansion] Semantics.Deed.action
