module Experimental.Words

%default total

-- ===== Vocabulary =====

||| Card types, as catalog atoms ([CR#205.2a]). Types are intrinsic to
||| NEITHER core nor semantics: each is DECLARED by a `TypeDef` macro
||| carrying its rules grants (`plugins/builtin/macros/cardtype/Creature.ron`
||| confers the combat grants, and even permanence is its `permanent: true`
||| field), so this enum stands in for reading those declarations.
public export
-- spelling: (construction-owned catalog -- Creature="creature", Artifact=
-- "artifact", Land="land", Enchantment="enchantment", Instant="instant",
-- Sorcery="sorcery"; each row is a TypeDef macro's own word (see
-- cardtype/Creature.ron), consumed by HasType/AsType and by the card
-- container's type line, never spelled alone)
-- Instant and Sorcery arrive with the card container rather than with a
-- clause: [CR#113.3a] defines the spell ability by what its CARD is, so
-- the container cannot ask the question without the two words.
data CardType = Creature | Artifact | Land | Enchantment | Instant | Sorcery

||| Fight participation, per type — stands in for a combat-participant
||| grant on the TypeDef, distinct from Creature.ron's May(Attack)/
||| May(Block): fight keys on type membership [CR#701.14a,701.14b] and
||| deals non-combat damage [CR#701.14d].
public export
combatant : CardType -> Bool
combatant Creature = True
combatant Artifact = False
combatant Land = False
combatant Enchantment = False
combatant Instant = False
combatant Sorcery = False

||| The projected-head gate the fight slots consume: an untyped head
||| cannot prove participation.
public export
data FightParticipant : Maybe CardType -> Type where
  Fighter : {auto 0 ok : combatant t = True} -> FightParticipant (Just t)

||| The numeric characteristics a phrase can NAME — one axis for both
||| readers this grammar has: the bound a phrase compares against
||| (`Compare`) and the value an amount reads off an object (`StatOf`).
||| Power and toughness [CR#208.1], mana value [CR#202.3]. Core's `Stat`
||| (`deckmaste_core/src/count.rs`) carries five, adding loyalty
||| [CR#209.1] and defense [CR#210.1]; both are absent from BOTH frames
||| here by measurement — neither is bounded, defense is never read off
||| an object at all, and the one loyalty read (Drain Life's cap clause)
||| is the phrasal-standard frame this grammar does not write either way
||| (see `Comparator`), while every other corpus "loyalty" line counts
||| COUNTERS, a different reader.
public export
-- spelling: (construction-owned catalog -- Power="power", Toughness=
-- "toughness", ManaValue="mana value"; each row is the characteristic's own
-- word, inside Compare's frame or after StatOf's possessive, never spelled
-- alone. Same three names core's `Stat` gives its own first three rows
-- (count.rs))
data Characteristic = Power | Toughness | ManaValue

public export
sameChar : Characteristic -> Characteristic -> Bool
sameChar Power Power = True
sameChar Power _ = False
sameChar Toughness Toughness = True
sameChar Toughness _ = False
sameChar ManaValue ManaValue = True
sameChar ManaValue _ = False

||| The PLAYER's own numbers — `Characteristic`'s twin one participant
||| sort over, and a row core was built expecting: its
||| `Count::PlayerStatOf(Reference, PlayerAttr)` carries the comment "the
||| player-side twin of StatOf, mirroring the Idris `PlayerStatOf`"
||| (`deckmaste_core/src/count.rs`), so the name here is settled by parity
||| and only the contents were open.
|||
||| TWO rows, not core's four, and the difference is measured rather than
||| conceded. Core's `PlayerAttr` adds `HandSize`, `HandSizeLimit` and
||| `LandPlaysPerTurn`; this grammar has no stat READ for any of them and
||| the corpus has none either. "Hand size" occurs 68 times and every one
||| is the maximum-hand-size STATEMENT ("you have no maximum hand size",
||| "your maximum hand size is eleven") — a static line about a limit, not
||| a number a clause reads — and "equal to your hand size" is written
||| ZERO times, the corpus counting the cards instead ("equal to the
||| number of cards in your hand", which is `CountOf`'s reading and
||| already writable). So the vocabulary is two rows because two are read.
|||
||| The STARTING total is a genuinely separate read and not a bound on the
||| current one. The CR states it twice in the same words — "Each player
||| begins the game with a starting life total of 20" is [CR#103.4] at the
||| game's setup and [CR#119.1] in the life-total section, one sentence
||| filed under both — and thirty supported lines read it as a term of its own
||| — almost all of them comparing the current total against it. Core does
||| NOT carry this row, so unlike the parent name this cell is a gap at
||| both layers, and by the merge doctrine the measurement here is the
||| core's too.
public export
-- spelling: (construction-owned catalog -- LifeTotal="life total",
-- StartingLifeTotal="starting life total"; each row is the stat's own
-- word after PlayerStatOf's possessive, "your life total" / "that
-- player's starting life total", never spelled alone. The threshold
-- frame writes the SAME read with a different surface -- "you have 40 or
-- more life" is the dominant spelling at 31 lines against 2 for "your
-- life total is 40 or more" -- which is the frame's business and not
-- this catalog's)
data PlayerStat = LifeTotal | StartingLifeTotal

public export
samePlayerStat : PlayerStat -> PlayerStat -> Bool
samePlayerStat LifeTotal LifeTotal = True
samePlayerStat LifeTotal _ = False
samePlayerStat StartingLifeTotal StartingLifeTotal = True
samePlayerStat StartingLifeTotal _ = False

||| The four comparison RELATIONS English writes — core's `Cmp`
||| (`condition.rs`) minus its `Eq`, and the old `Semantics` module's own
||| five-row `Cmp` under the same four names, which is a name straddle
||| across independent vocabularies exactly as `CountersOn` is.
|||
||| The relation is the vocabulary; the comparative WORD is not, because
||| it varies perfectly with what is already written beside it — finding
||| 275's idiom, and the reason this enum grew rather than doubled.
||| Against a WRITTEN bound the two or-relations follow it ("4 or
||| greater", "2 or less"; four hundred forty-one and seven hundred
||| forty-four lines) and the long form is written ZERO times; against a
||| READ they precede it ("greater than or equal to your starting life
||| total", "less than or equal to the number of lands you control";
||| seven lines and one hundred) and the short form is written zero times
||| there. The strict pair precedes its bound either way.
|||
||| `Eq` stays out on its own measurement: a comparison "is equal to" is
||| four supported lines and every one is unwritable for a second reason
||| (a die result, unspent mana, a sum), while the corpus's enormous
||| "equal to" mass is the AMOUNT-level copy ("gain life equal to its
||| power") and not a condition at all.
public export
-- spelling: (construction-owned -- the word is DERIVED from the relation,
-- the bound's class and the head's register, and is carried by no row:
-- AtLeast/AtMost follow a written bound ("4 or greater", "2 or less") and
-- precede a read ("greater than or equal to <read>"); Greater/Less always
-- precede ("less than 7", "more life than an opponent"). The register is
-- the head's and is unchanged from chapter forty-four: a stat takes
-- greater/less, a count takes more/fewer, which the english crate stores
-- rather than derives (`ComparativeWord`, syntax/phrase.rs))
data Comparator = AtLeast | AtMost | Greater | Less

public export
sameCmp : Comparator -> Comparator -> Bool
sameCmp AtLeast AtLeast = True
sameCmp AtLeast _ = False
sameCmp AtMost AtMost = True
sameCmp AtMost _ = False
sameCmp Greater Greater = True
sameCmp Greater _ = False
sameCmp Less Less = True
sameCmp Less _ = False

||| Which card type a characteristic PRESUPPOSES of the object read —
||| the characteristic half of the closed table `deedType` writes for
||| deeds. Power and toughness belong to creatures: a noncreature
||| permanent has neither, and a noncreature object off the battlefield
||| has them only if printed ([CR#208.3]). Mana value belongs to every
||| object ([CR#202.3]; [CR#202.3a] gives even a costless object zero),
||| so it presupposes nothing. The Vehicle case [CR#208.3] leaves open —
||| a noncreature card CAN carry printed power in a graveyard — is never
||| written, so the creature row is exact for what English spells.
public export
comparedType : Characteristic -> Maybe CardType
comparedType Power = Just Creature
comparedType Toughness = Just Creature
comparedType ManaValue = Nothing

||| Quality sorts — the choosable characteristics ([CR#105.1,302.3,607.2d];
||| only what the chapters need). `QualityNoun` selects all four.
public export
-- spelling: (construction-owned catalog -- Color="color", CreatureType=
-- "creature type", CardName="card name", Number="number"; consumed by
-- QualityNoun, and by OfChosen only where ChosenQualityRead admits it;
-- never spelled alone)
data QualitySort = Color | CreatureType | CardName | Number

public export
sameQ : QualitySort -> QualitySort -> Bool
sameQ Color Color = True
sameQ Color _ = False
sameQ CreatureType CreatureType = True
sameQ CreatureType _ = False
sameQ CardName CardName = True
sameQ CardName _ = False
sameQ Number Number = True
sameQ Number _ = False

||| Whether the surface `OfChosen` honestly reads this chosen sort.
||| Colors and creature types take "of the chosen …"; card names and
||| numbers use their own later equality surfaces. Linked choices remain
||| governed by [CR#607.2d].
public export
chosenQualityReadOk : QualitySort -> Bool
chosenQualityReadOk Color = True
chosenQualityReadOk CreatureType = True
chosenQualityReadOk CardName = False
chosenQualityReadOk Number = False

||| The erased readback-surface witness, kept named so failing catalog pins
||| report the unavailable surface rather than a generic equality goal.
public export
data ChosenQualityRead : QualitySort -> Type where
  MkChosenQualityRead : {auto 0 ok : chosenQualityReadOk q = True} ->
                        ChosenQualityRead q

||| What a binding can bind ([CR#115.1] — targets are objects and/or
||| players; the union kind is deferred with the carrier lattice), plus
||| chosen qualities, which enter the same discourse, and event OUTCOMES
||| (what a clause DID, readable as "that much"), which only clauses
||| introduce.
|||
||| `Letter` is the third, and the only one the TEXT names rather than
||| computes: a value the card's own rider defines and its body reads back
||| by a letter ([CR#107.3c]). It is counted apart from `Outcome` and `Gap`
||| for the reason those two are counted apart from each other — an
||| outcome is the magnitude of what a clause DID, a gap the margin by
||| which a comparison HELD, and a letter neither: it is a name the
||| sentence introduced, and the three are read by three different words
||| ("that much", "the difference", "X"). Sharing a sort would let each
||| word spell where another belongs and make each anaphor's uniqueness
||| the others' business. The sort is `Letter` rather than `DefinedX`
||| because [CR#107.3p] gives Y the same rules, so the second word is a
||| second spelling of this row and not a second sort — and the row
||| carries the WORD, which is `Quality`'s shape at the sort beside it: a
||| letter is a name, two names in one sentence are two mentions, and the
||| uniqueness each anaphor demands is per NAME ("X" reads the X-lettered
||| binding while "Y" reads the Y-lettered one, Bioplasm writing both).
||| Indexing the kind is what makes that uniqueness the existing
||| machinery's: `sameKind` already routes `Quality` through `sameQ`, so
||| `countLetter` is `countQuality`'s twin and nothing else changes.
|||
||| `Gap` is the second magnitude sort and the second clause-introduced
||| one: the amount by which a COMPARISON was true, readable as "the
||| difference". It is named and counted apart from `Outcome` because the
||| two are fixed by different clauses and read by different words — an
||| outcome is the magnitude of what a clause DID, and no condition does
||| anything — so conflating them would let "that much" spell where
||| English writes "the difference" and would make each anaphor's
||| uniqueness the other's business.
||| WHICH letter a definition rider names. Two words and no more:
||| [CR#107.3p] says only that "some objects use the letter Y in addition
||| to the letter X" and that "Y follows the same rules as X", and the
||| corpus writes exactly those two — every rider line uses X, three cards
||| add Y, and no card writes a third letter. The words differ in NO
||| table, which is finding 275's idiom and is why this is an index on one
||| row rather than a second sort.
public export
data LetterWord = LetterX | LetterY

public export
sameLetterWord : LetterWord -> LetterWord -> Bool
sameLetterWord LetterX LetterX = True
sameLetterWord LetterX LetterY = False
sameLetterWord LetterY LetterX = False
sameLetterWord LetterY LetterY = True

public export
data Kind = Object | Player | Quality QualitySort | Outcome | Gap
          | Letter LetterWord

public export
sameKind : Kind -> Kind -> Bool
sameKind Object Object = True
sameKind Object Player = False
sameKind Object (Quality _) = False
sameKind Object Outcome = False
sameKind Object Gap = False
sameKind Object (Letter _) = False
sameKind Player Object = False
sameKind Player Player = True
sameKind Player (Quality _) = False
sameKind Player Outcome = False
sameKind Player Gap = False
sameKind Player (Letter _) = False
sameKind (Quality _) Object = False
sameKind (Quality _) Player = False
sameKind (Quality a) (Quality b) = sameQ a b
sameKind (Quality _) Outcome = False
sameKind (Quality _) Gap = False
sameKind (Quality _) (Letter _) = False
sameKind Outcome Object = False
sameKind Outcome Player = False
sameKind Outcome (Quality _) = False
sameKind Outcome Outcome = True
sameKind Outcome Gap = False
sameKind Outcome (Letter _) = False
sameKind Gap Object = False
sameKind Gap Player = False
sameKind Gap (Quality _) = False
sameKind Gap Outcome = False
sameKind Gap Gap = True
sameKind Gap (Letter _) = False
sameKind (Letter _) Object = False
sameKind (Letter _) Player = False
sameKind (Letter _) (Quality _) = False
sameKind (Letter _) Outcome = False
sameKind (Letter _) Gap = False
sameKind (Letter a) (Letter b) = sameLetterWord a b

||| How a set of numbers folds to ONE. Core's `AggregateOp`
||| (`deckmaste_core/src/count.rs`) and the old `Semantics` module's own
||| enum both carry these names already, which is `CountersOn`'s situation
||| at a third site: three independent vocabularies, one obvious set of
||| words, and no import between them.
|||
||| THREE rows against those two catalogs' four, and the fourth is a
||| MEASURED zero rather than a concession: the word "average" appears ZERO
||| times in the corpus, supported and unsupported alike. What English
||| writes is the sum ("the total power of creatures you control") and the
||| two extremes ("the greatest power among creatures you control", "the
||| lowest life total among all players"), and a mean would have to arrive
||| with a rounding direction of its own ([CR#107.1a]) for no line to
||| spell.
|||
||| The empty set is not this catalog's question, and the CR answers it
||| once for every row: a number that cannot be determined uses 0 instead
||| ([CR#107.2]), which settles the greatest of no creatures exactly as it
||| settles the sum of none.
public export
-- spelling: (construction-owned catalog -- the op fixes the PREPOSITION as
-- well as the word, and both are spelling facts derived from what is
-- already written (finding 275). The extremes write "among" and the sum
-- writes "of" -- "the greatest power among creatures you control" against
-- "the total power of creatures you control", neither ever the other way
-- -- and the extremal word is a table on the STAT WORD the axis spells.
-- Chapter sixty-two read that table off the axis SORT and was too coarse
-- by one cell: power and toughness take greatest/least (90/4 and 11/24)
-- and "life total" takes highest/lowest (5/2), with the four crossings
-- chapter sixty-two measured still zero ("greatest life total", "highest
-- power", "least life total", "lowest power"), but MANA VALUE -- a
-- characteristic -- writes its minimum "lowest" (Culling Scales,
-- Stronghold Gambit) and "least mana value" zero times. The MODIFIER
-- frame adds the third pair: the life total spelled "life" takes "most"
-- (21 lines, "the player with the most life") and no minimum at all.
-- SumOf writes "total" at both axes)
data AggregateOp = SumOf | MinOf | MaxOf

||| May this fold's op pick an ELEMENT as well as report a number? The
||| superlative noun-modifier's gate, and the one the old `Semantics`
||| module wrote for its own element-picking twin under this name
||| (`IsExtremal`, gating `Pick` to the two extremes).
|||
||| It is the English that decides, not the arithmetic: "the creature with
||| the greatest power" names a member of the set, and a SUM names no
||| member of anything -- "with the total power" is written zero times in
||| the whole corpus, supported and unsupported alike (`badSumSelection`).
||| The fold in AMOUNT position keeps all three rows, which is why this is
||| a gate on the modifier rather than a split of the catalog.
public export
isExtremal : AggregateOp -> Bool
isExtremal SumOf = False
isExtremal MinOf = True
isExtremal MaxOf = True

public export
data IsExtremal : AggregateOp -> Type where
  MkIsExtremal : {auto 0 ok : isExtremal op = True} -> IsExtremal op

public export
sameAggregateOp : AggregateOp -> AggregateOp -> Bool
sameAggregateOp SumOf SumOf = True
sameAggregateOp SumOf _ = False
sameAggregateOp MinOf MinOf = True
sameAggregateOp MinOf _ = False
sameAggregateOp MaxOf MaxOf = True
sameAggregateOp MaxOf _ = False

||| WHICH number a fold reads off each member of its set -- core's
||| `Projection.by` narrowed to the reads this grammar has, and the
||| set-generalization `StatOf`, `PlayerStatOf` and `CountersOn` have each
||| promised in their own comment since they were minted ("a group's
||| aggregate is written explicitly … and is future vocabulary").
|||
||| A CLOSED catalog and not a per-element expression, which is the one
||| real design choice in the fold and is decided by measurement. Core and
||| the old `Semantics` module both bind `It` to each element and read an
||| arbitrary count through it; the corpus's "among" complement never uses
||| that width, all 103 description-domain lines reading a characteristic
||| off an object or a life total off a player and nothing else. What DOES
||| want the general shape is a different construction with a different
||| surface -- "the greatest number of creatures a player controls" (10
||| lines) folds a count RELATIVIZED to each member and writes a relative
||| clause where this family writes "among" -- and a general slot here
||| would still not reach it, since the element those lines bind sits
||| inside the domain's own description. So the width buys nothing the
||| corpus writes (ledger).
public export
-- spelling: (construction-owned catalog -- each row is its argument's own
-- word, "power"/"toughness"/"mana value" and "life total", exactly as it
-- is spelled after `StatOf`'s and `PlayerStatOf`'s possessive; the
-- superlative or total word before it is `AggregateOp`'s, see there)
data ProjAxis = CharAxis Characteristic | PlayerStatAxis PlayerStat

||| WHAT a projected axis may be read off -- the fold's agreement table,
||| `counterScope`'s shape one vocabulary over and answering in `Kind` for
||| the same reason: one row over `{k : Kind}` can then demand
||| `projScope ax = k` and get the domain agreement for free instead of a
||| fold per sort.
|||
||| Full rows, so a new axis must declare its sort before the fold will
||| take it, and the two crossed cells the table refuses are measured
||| zeros: "the greatest power among players" and "the highest life total
||| among creatures you control" are written zero times, power being a
||| creature's ([CR#208.1]) and a life total a player's ([CR#119.1]) --
||| `badPowerAmongPlayers`, `badLifeTotalAmongObjects`.
public export
projScope : ProjAxis -> Kind
projScope (CharAxis _) = Object
projScope (PlayerStatAxis _) = Player

public export
sameProjAxis : ProjAxis -> ProjAxis -> Bool
sameProjAxis (CharAxis a) (CharAxis b) = sameChar a b
sameProjAxis (CharAxis _) _ = False
sameProjAxis (PlayerStatAxis a) (PlayerStatAxis b) = samePlayerStat a b
sameProjAxis (PlayerStatAxis _) _ = False

||| The surface-projected SORT of an event outcome; its magnitude stays
||| runtime, never stored. Mana produced, counters, and card counts are
||| later sorts.
public export
data OutcomeSort = DamageDealt | LifeGained | LifeLost

||| Singular mention or group mention — the guard that keeps "it" from
||| resolving to a plural antecedent.
public export
data Plurality = OneOf | ManyOf

public export
isOne : Plurality -> Bool
isOne OneOf = True
isOne ManyOf = False

||| The number a DISTRIBUTED output takes: one thing per agent, over
||| many agents, is many things — "Each player creates a green Elephant
||| creature token. THOSE CREATURES have …" (Elephant Resurgence). The
||| per-agent amount is one, and the discourse mention is still plural.
public export
outputPlur : (agent : Plurality) -> (perAgent : Plurality) -> Plurality
outputPlur OneOf OneOf = OneOf
outputPlur OneOf ManyOf = ManyOf
outputPlur ManyOf OneOf = ManyOf
outputPlur ManyOf ManyOf = ManyOf

||| How many objects a counted mention takes — core's `Quantity`
||| (`deckmaste_core/src/quantity.rs`): ONE primitive, a range with both
||| bounds optional (`Nothing` = unbounded that side, so "any number of"
||| is `Range Nothing Nothing`; [CR#601.2c] has the caster announce that
||| variable count before choosing). The readable named forms are macros
||| over it in both frames (`exactly`/`upTo`/`anyNumber` in
||| `Experimental.Macros`). A magnitude is not a quantity — that is
||| `Amount`.
public export
-- spelling: (construction-owned -- the primitive itself has no word; its
-- macros do (Experimental.Macros: exactly/upTo/anyNumber). Verified real
-- family: crates/deckmaste_english/src/constructions/quantity.rs's combinators
-- "quantity_exact" (exactly n) and "quantity_up_to" (upTo n) -- name+semantics
-- match. "any number of" (anyNumber): TODO(reason: no matching combinator
-- confirmed among that file's registered names within this pass's scope))
data Quantity : Type where
  Range : Maybe Nat -> Maybe Nat -> Quantity

||| A written quantity permits at least one object ([CR#115.1] — a slot
||| cannot target nothing), and the UPPER bound carries the demand: a
||| statically zero maximum is unwritten English however it is spelled
||| (`badZeroGroup`). An ABSENT maximum is the unbounded "any number of"
||| and a zero LOWER bound is what every "up to" has, so neither is
||| touched.
public export
data NonZeroQ : Quantity -> Type where
  UnboundedAbove : NonZeroQ (Range lo Nothing)
  MaxAtLeastOne : NonZeroQ (Range lo (Just (S n)))

public export
leNat : Nat -> Nat -> Bool
leNat Z _ = True
leNat (S _) Z = False
leNat (S a) (S b) = leNat a b

||| A written quantity also runs UPWARD, from a minimum of at least one.
||| A DESCENDING range admits nothing at all ("between three and two
||| target creatures" names an empty interval, and the plurality read off
||| the maximum would lie about it besides), and a ZERO minimum spells
||| nothing the unbounded form does not already say: [CR#107.1c] has
||| "any number" permit zero outright, so "zero or more target creatures"
||| is a second spelling of "any number of target creatures" — one the
||| corpus never writes (`badDescendingRange`, `badZeroLowerRange`). An
||| absent minimum is every "up to", untouched.
public export
quantWellFormed : Quantity -> Bool
quantWellFormed (Range Nothing _) = True
quantWellFormed (Range (Just Z) _) = False
quantWellFormed (Range (Just (S n)) Nothing) = True
quantWellFormed (Range (Just (S n)) (Just hi)) = leNat (S n) hi

public export
data WellFormedQ : Quantity -> Type where
  MkWellFormedQ : {auto 0 ok : quantWellFormed q = True} -> WellFormedQ q

||| A counted mention's grammatical number, read off its quantity: one
||| is SINGULAR — "target creature" and "up to one target creature" both
||| rement as "it" — and every wider quantity is a group. The MAXIMUM is
||| what number reads; choosing fewer [CR#115.6] is runtime's null read,
||| not a grammar fact.
public export
quantPlur : Quantity -> Plurality
quantPlur (Range _ (Just (S Z))) = OneOf
quantPlur (Range _ _) = ManyOf

public export
eqNat : Nat -> Nat -> Bool
eqNat a b = leNat a b && leNat b a

||| Whether a modal headcount FITS the list of modes it heads
||| ([CR#700.2]) — two demands, both about the PAIR rather than either
||| half, which is why they are one relation and not a widening of
||| `WellFormedQ`. First, impossibility: a headcount cannot reach past
||| the modes offered ("choose three" of two options names nothing), so
||| a WRITTEN maximum is no greater than the count. Second, a modal
||| offers a CHOICE — [CR#700.2] calls a spell modal when its bulleted
||| options come "preceded by instructions for a player to choose a
||| number of those options", so a headcount that fixes the whole list
||| instructs nothing ("Choose two —" over two modes has one answer;
||| zero cards write it, `badModalFixedWhole`). An open top escapes by
||| having a range to choose in, as does every "up to".
public export
modesFit : Quantity -> Nat -> Bool
modesFit (Range Nothing Nothing) n = True
modesFit (Range Nothing (Just hi)) n = leNat hi n
modesFit (Range (Just lo) Nothing) n = leNat lo n
modesFit (Range (Just lo) (Just hi)) n = leNat hi n && not (eqNat lo hi && eqNat hi n)

public export
data ModesFit : Quantity -> Nat -> Type where
  MkModesFit : {auto 0 ok : modesFit q n = True} -> ModesFit q n

||| Which modal headcounts English WRITES — the attestation table beside
||| `modesFit`'s well-formedness one, and a separate relation for the
||| separate question. The attested vocabulary is the three small fixed
||| counts ("choose one/two/three —"), the one-capped "up to", the two
||| open tops ("one or both", "one or more"), and the unbounded head;
||| "choose up to two —", "choose up to three —", "choose four", "two or
||| more", and "one or two" are written zero times, all scopes, so an
||| unattested head is refused rather than inferred from the range
||| algebra (`badModalUpToTwo`). The two OPEN TOPS take their maximum
||| from the list — "one or both" over exactly two modes, "one or more"
||| over more — so the top is required to BE the count rather than
||| merely fit under it.
public export
modalHead : Quantity -> Nat -> Bool
modalHead (Range Nothing Nothing) n = True
modalHead (Range Nothing (Just (S Z))) n = True
modalHead (Range Nothing (Just _)) n = False
modalHead (Range (Just (S Z)) Nothing) n = True
modalHead (Range (Just _) Nothing) n = False
modalHead (Range (Just lo) (Just hi)) n =
  (eqNat lo hi && leNat lo 3) || (eqNat lo 1 && eqNat hi n)

public export
data ModalHead : Quantity -> Nat -> Type where
  MkModalHead : {auto 0 ok : modalHead q n = True} -> ModalHead q n

||| A written numeral is at least one — "for each zero creatures" is
||| unwritten English (`badForEachZero`).
public export
data AtLeastOne : Nat -> Type where
  OneUp : AtLeastOne (S n)

||| A sequence runs at least two clauses — what makes it a sequence
||| rather than a sentence (`badEmptySequence`, `badSingletonSequence`).
public export
data AtLeastTwo : Nat -> Type where
  TwoUp : AtLeastTwo (S (S n))

||| The introducing word of a mention — a SURFACE projection ("target",
||| "a", "each", "all", or a definite/derived mention). Rules facts (the
||| settled-target boundary, the "other" presupposition) are functions of
||| it, never stored alongside it. Counted target mentions share ONE tag
||| whatever their quantity: no consumer ever told "up to" apart, so the
||| possible emptiness [CR#115.6] lives in the quantity, as it does in
||| core's single announce form.
public export
-- spelling: (construction-owned -- TargetD/AD/EachD/AllD/TheD mark WHICH Noun
-- constructor built a binding; the words live on Noun's own TargetGroup/A/
-- Each/AllOf/TheVerbed rows, not here. The english crate has its own
-- `Determiner` hole (constructions/coordination.rs's shared_determiner_nominal)
-- confirming the concept; TODO(reason: no single owning family name verified
-- for the per-word constructions within this pass's scope))
data Determiner = TargetD | AD | EachD | AllD | TheD | PartD
                -- the EVENT SUBJECT's own mention, minted by a trigger's
                -- after-discourse for a self subject and by nothing else.
                -- It is a determiner row rather than a payload flag because
                -- what distinguishes it is exactly what a determiner
                -- distinguishes: which construction built the binding, and
                -- therefore which readers may see it. `It` sees it; the
                -- DEMONSTRATIVES do not (`wordNow`), which is English's own
                -- rule and finding 348's.
                | SelfD

||| Zone sorts ([CR#400.1] family) — the fold-state tag a binding
||| carries. Ownership is not stored here; it lives in the surface
||| `ZoneExpr` where English writes it. The library is the one ORDERED
||| zone ([CR#401.2] — "a single face-down pile", whose order players may
||| neither inspect nor change), and the order is likewise no part of the
||| SORT: it lives on the position phrase (`LibraryAt`), exactly as
||| ownership does. Core makes the same split (`Zone::Library` beside
||| `Destination::Library(Anchor)`).
|||
||| The STACK ([CR#405.1] makes it the place a cast card goes) is a zone
||| rather than a second object sort because [CR#112.1] identifies the
||| two in one sentence — "a spell is a card on the stack" — so the spell
||| WORD is this zone's carrier noun exactly as "card" is the
||| hidden-and-graveyard zones' ([CR#108.2]) and a type word is the
||| battlefield's ([CR#109.2]). It is never a `Move` destination
||| (`DestOk` has no row, which is core's own `exclude(Library, Stack)`),
||| and it is shared rather than possessed ([CR#400.1]; no `Possessable`
||| row), so "your stack" is unwritable for the same reason "your
||| battlefield" is. Core's `Command` row is still not ported.
public export
data Zone = Battlefield | Graveyard | Exile | Hand | Library | Stack

public export
sameZone : Zone -> Zone -> Bool
sameZone Battlefield Battlefield = True
sameZone Battlefield _ = False
sameZone Graveyard Graveyard = True
sameZone Graveyard _ = False
sameZone Exile Exile = True
sameZone Exile _ = False
sameZone Hand Hand = True
sameZone Hand _ = False
sameZone Library Library = True
sameZone Library _ = False
sameZone Stack Stack = True
sameZone Stack _ = False

||| A position IN the ordered library ([CR#401.2]) — the two words
||| English writes bare, and the whole of the attested position
||| vocabulary at offset zero. Core spells the axis once with an offset
||| (`Anchor = FromTop(Count) | FromBottom(Count)`); English writes the
||| offset with a DIFFERENT phrase, the ordinal "second from the top"
||| ([CR#401.7]'s own subject), so it is not this table's third row but
||| an ordinal vocabulary this file does not have (ledger).
public export
-- spelling: (construction-owned catalog -- each row is the position phrase
-- LibraryAt writes over its zone word: OnTop = "on top of <scope> library",
-- OnBottom = "on the bottom of <scope> library". Noun-side the same words
-- lead the slice ("the top <n> cards of <scope> library"). Never spelled
-- alone -- see LibraryAt and LibrarySlice)
data LibPos = OnTop | OnBottom

||| How a GROUP landing at a library position is arranged ([CR#401.4]):
||| the two riders English writes. Core carries four (`Arrangement`,
||| adding `SameOrder` and a `ChosenOrder(Reference)`) and English writes
||| neither of the extra two — zero lines each — so this is a recorded
||| NARROWING of core, `DividedVerb`'s discipline (finding 126) applied
||| to the order rider. The rider is not a default made explicit:
||| [CR#401.4] already gives the cards' owner the arrangement ("may
||| arrange them in any order"), so "in any order" RESTATES the rule and
||| only "in a random order" overrides it — which is why the absent rider
||| and the any-order rider mean the same thing and both are written.
public export
-- spelling: (construction-owned catalog -- each row is the trailing adverbial
-- on a library placement: AnyOrder = "in any order", RandomOrder = "in a
-- random order". Never spelled alone -- see LibraryAt)
data Arrangement = AnyOrder | RandomOrder

||| Keyword-action tags ([CR#701]) — core's `Composite` verb names.
||| `Destroy` and `Discard` mirror `plugins/builtin/macros/action/`;
||| `Sacrifice` is a core whittling candidate (see the decision record);
||| `Exile` is speculative pending its real macro definition. (Its own
||| namespace: the tag `Exile` and the zone `Exile` are distinct words.)
namespace Verb
  public export
  -- spelling: (construction-owned catalog -- each row is a keyword-action tag
  -- consumed by Composite/Does, spelled through its own macro: Destroy =
  -- action/Destroy.ron's "destroy <Param(0)>" (verified against that file);
  -- Sacrifice = core whittling candidate, spelled by the `sacrifice` macro;
  -- Exile = speculative, spelled by `exile`; Discard = spelled by `discards`/
  -- `discardsACard`. Never spelled alone -- see Experimental.Macros)
  data VerbName = Destroy | Sacrifice | Exile | Discard

public export
sameVerb : VerbName -> VerbName -> Bool
sameVerb Destroy Destroy = True
sameVerb Destroy _ = False
sameVerb Sacrifice Sacrifice = True
sameVerb Sacrifice _ = False
sameVerb Exile Exile = True
sameVerb Exile _ = False
sameVerb Discard Discard = True
sameVerb Discard _ = False

||| The provenance a tagged move writes: which verb took the referent,
||| and whether it stood on the battlefield when the verb did (the
||| at-verb frame a bare type word's participle needs — finding 29).
public export
record Stamp where
  constructor MkStamp
  verb : VerbName
  wasField : Bool

||| The representation provenance a mention can record: the referent was
||| minted by a token-creation clause ([CR#111.1] — a token is a marker
||| representing a permanent that isn't represented by a card). ONE row:
||| the card-born complement has no reader, "nontoken" being a
||| description-side negation (`Not IsToken`) and never a readback word.
public export
data Origin = TokenOrigin

public export
isTokenOrigin : Maybe Origin -> Bool
isTokenOrigin (Just TokenOrigin) = True
isTokenOrigin Nothing = False

||| Per-kind mention data, kind-indexed so a binding can only record
||| what its kind can have. The object zone is the ONE piece of
||| fold-state — `Move` updates it, everything else is a projection of
||| the phrase. An ill-sorted binding ("a player in your hand") is
||| thereby unrepresentable, which is the refusal the surface grammar
||| makes (`InZone` is Object-kinded) extended to the representation.
public export
data Payload : Kind -> Type where
  ObjectP : (ty : Maybe CardType) -> (zone : Maybe Zone) ->
            (prov : Maybe Stamp) -> (orig : Maybe Origin) -> Payload Object
  PlayerP : Payload Player
  QualityP : Payload (Quality q)
  OutcomeP : (sort : OutcomeSort) -> Payload Outcome
  -- the gap carries no sort of its own: what it measures is written in
  -- the comparison that licensed it, one clause away, and nothing reads
  -- it back except the one word.
  GapP : Payload Gap
  -- the letter carries no sort either: what it is worth is the rider's
  -- amount, one clause away, and the mention exists so the body can say
  -- the letter at all.
  LetterP : Payload (Letter w)

||| One discourse mention.
public export
record Binding where
  constructor MkBinding
  det : Determiner
  kind : Kind
  plur : Plurality
  payload : Payload kind

||| The one context: a nearest-first list of mentions.
public export
Bindings : Type
Bindings = List Binding

||| The zone a binding tracks — object fold-state; nothing else has one,
||| structurally.
public export
bindingZone : Binding -> Maybe Zone
bindingZone (MkBinding _ _ _ (ObjectP _ zn _ _)) = zn
bindingZone (MkBinding _ _ _ PlayerP) = Nothing
bindingZone (MkBinding _ _ _ QualityP) = Nothing
bindingZone (MkBinding _ _ _ (OutcomeP _)) = Nothing
bindingZone (MkBinding _ _ _ GapP) = Nothing
bindingZone (MkBinding _ _ _ LetterP) = Nothing

||| The projected head type a binding carries, if its kind can.
public export
bindingTy : Binding -> Maybe CardType
bindingTy (MkBinding _ _ _ (ObjectP ty _ _ _)) = ty
bindingTy (MkBinding _ _ _ PlayerP) = Nothing
bindingTy (MkBinding _ _ _ QualityP) = Nothing
bindingTy (MkBinding _ _ _ (OutcomeP _)) = Nothing
bindingTy (MkBinding _ _ _ GapP) = Nothing
bindingTy (MkBinding _ _ _ LetterP) = Nothing

||| The mention an event clause prepends for what it did — sort from
||| the clause's surface, value runtime.
public export
outcomeB : OutcomeSort -> Binding
outcomeB s = MkBinding TheD Outcome OneOf (OutcomeP s)

||| The mention a COMPARISON prepends for the margin by which it held --
||| `outcomeB`'s twin at the other magnitude sort. Definite (`TheD`),
||| which is what the surface writes: "the difference", never "a
||| difference".
public export
gapB : Binding
gapB = MkBinding TheD Gap OneOf GapP

||| The mention a definition RIDER prepends for the letter it defines.
||| Definite for the same reason the other two are: the sentence has just
||| said which value this is. The WORD is the mention's own, so a sentence
||| that defines both letters carries two mentions and each anaphor finds
||| its own ([CR#107.3p]).
public export
letterB : LetterWord -> Binding
letterB w = MkBinding TheD (Letter w) OneOf LetterP

public export
sameCT : CardType -> CardType -> Bool
sameCT Creature Creature = True
sameCT Creature _ = False
sameCT Artifact Artifact = True
sameCT Artifact _ = False
sameCT Land Land = True
sameCT Land _ = False
sameCT Enchantment Enchantment = True
sameCT Enchantment _ = False
sameCT Instant Instant = True
sameCT Instant _ = False
sameCT Sorcery Sorcery = True
sameCT Sorcery _ = False

||| Singular mentions of a kind, counted — the wildcard pronoun's
||| obligation is `= 1`: zero is an unbound anaphor, two an ambiguous
||| one. Kind matching routes through `sameKind`.
public export
countOnes : Kind -> Bindings -> Nat
countOnes k [] = Z
countOnes k (MkBinding _ k' OneOf _ :: bs) =
  if sameKind k k' then S (countOnes k bs) else countOnes k bs
countOnes k (_ :: bs) = countOnes k bs

||| Chosen-quality mentions of a sort, counted — "the chosen color"
||| demands exactly one.
public export
countQuality : QualitySort -> Bindings -> Nat
countQuality q [] = Z
countQuality q (MkBinding _ k OneOf _ :: bs) =
  if sameKind (Quality q) k then S (countQuality q bs) else countQuality q bs
countQuality q (_ :: bs) = countQuality q bs

||| Letter mentions of a WORD, counted — "X" demands exactly one
||| X-lettered binding and says nothing about Y. `countQuality`'s twin at
||| the sort beside it, and for the same reason: the kind carries the word,
||| so the count is the ordinary one asked at one index.
public export
countLetter : LetterWord -> Bindings -> Nat
countLetter w [] = Z
countLetter w (MkBinding _ k OneOf _ :: bs) =
  if sameKind (Letter w) k then S (countLetter w bs) else countLetter w bs
countLetter w (_ :: bs) = countLetter w bs

||| Group mentions of a kind, counted — the plural wildcard's
||| obligation is `= 1`, the ManyOf twin of `countOnes`.
public export
countManys : Kind -> Bindings -> Nat
countManys k [] = Z
countManys k (MkBinding _ k' ManyOf _ :: bs) =
  if sameKind k k' then S (countManys k bs) else countManys k bs
countManys k (_ :: bs) = countManys k bs

||| GROUP mentions of objects, counted — `countManys` minus the parts.
||| A partitive ("two of them") is plural and is NOT a group the sentence
||| may subtract from: it is what was subtracted. The complement's
||| presupposition counts the groups only, which is why the partitive
||| needed a determiner tag of its own.
public export
countGroups : Bindings -> Nat
countGroups [] = Z
countGroups (MkBinding PartD _ _ _ :: bs) = countGroups bs
countGroups (MkBinding _ Object ManyOf _ :: bs) = S (countGroups bs)
countGroups (_ :: bs) = countGroups bs

||| Parts TAKEN from a group, counted — object mentions the partitive
||| determiner announced.
public export
countParts : Bindings -> Nat
countParts [] = Z
countParts (MkBinding PartD Object _ _ :: bs) = S (countParts bs)
countParts (_ :: bs) = countParts bs

||| What "the rest" presupposes: one assembled group, and at least one
||| part taken out of it — with no group there is nothing to be the rest
||| OF, and with nothing taken the sentence would have written "them".
public export
theRestOk : Bindings -> Bool
theRestOk bs = eqNat (countGroups bs) 1 && not (eqNat (countParts bs) Z)

||| What disposing of a partition's REMAINDER leaves behind: the group
||| mention is SPENT. "The rest" names what is outstanding of a group, so
||| once the rest has been placed a second "the rest" in the same breath
||| names nothing (`badRestDisposedTwice`). Dropping the group makes
||| `theRestOk` false for every later clause on the SAME path, while a
||| branch arm typed in the discourse BEFORE the disposition still sees
||| it — which is what the corpus writes, its only lines with two "the
||| rest" phrases putting them in mutually exclusive arms.
||| The parts stay: a part that was taken is a referent that moved, and
||| later text may still read it. Group mentions assembled some other way
||| go with it, which costs nothing written.
public export
groupSpent : Bindings -> Bindings
groupSpent [] = []
groupSpent (b@(MkBinding PartD _ _ _) :: bs) = b :: groupSpent bs
groupSpent (MkBinding _ Object ManyOf _ :: bs) = groupSpent bs
groupSpent (b :: bs) = b :: groupSpent bs

||| The zone of the unique group mention — what a complement read
||| inherits, the group's own place ([CR#701.20b]: exposing a card does
||| not move it, so a looked-at library slice is still in the library).
public export
zoneOfGroup : Bindings -> Maybe Zone
zoneOfGroup [] = Nothing
zoneOfGroup (MkBinding PartD _ _ _ :: bs) = zoneOfGroup bs
zoneOfGroup (MkBinding _ Object ManyOf (ObjectP _ zn _ _) :: bs) = zn
zoneOfGroup (_ :: bs) = zoneOfGroup bs

||| …and its projected head type, by the same walk.
public export
tyOfGroup : Bindings -> Maybe CardType
tyOfGroup [] = Nothing
tyOfGroup (MkBinding PartD _ _ _ :: bs) = tyOfGroup bs
tyOfGroup (MkBinding _ Object ManyOf (ObjectP ty _ _ _) :: bs) = ty
tyOfGroup (_ :: bs) = tyOfGroup bs

||| Is any target-determined mention of this kind in scope? — the
||| presupposition of the modifier "other" ([CR#115.4]), stated entirely
||| in target vocabulary.
public export
anyTargeted : Kind -> Bindings -> Bool
anyTargeted k [] = False
anyTargeted k (MkBinding TargetD k' _ _ :: bs) =
  if sameKind k k' then True else anyTargeted k bs
anyTargeted k (_ :: bs) = anyTargeted k bs

||| Head-type compatibility between an "other" phrase and a candidate
||| anchor mention: an anchor that projects no head type is compatible
||| with any head (wildcards, "any target").
public export
anchorTyOk : CardType -> Maybe CardType -> Bool
anchorTyOk t Nothing = True
anchorTyOk t (Just t') = sameCT t t'

||| Is ANY target-determined mention in scope, of any kind at all? The
||| kind-blind twin, needed by the static ability line:
||| [CR#115.1a..115.1e] enumerate the things that can target and a static
||| ability is not among them, so the question is not "which kind" but
||| "any".
public export
anyTargetedAt : Bindings -> Bool
anyTargetedAt [] = False
anyTargetedAt (MkBinding TargetD _ _ _ :: _) = True
anyTargetedAt (_ :: bs) = anyTargetedAt bs

||| The typed twin of `anyTargeted`: is a target mention of this kind
||| AND a compatible head type in scope? The style guide writes two
||| separately described roles WITHOUT "other" ("target creature and
||| target planeswalker"), and the corpus pairs "other" only with
||| overlapping heads, so a cross-head anchor is no witness.
public export
anyTargetedTy : Kind -> CardType -> Bindings -> Bool
anyTargetedTy k t [] = False
anyTargetedTy k t (b@(MkBinding TargetD k' _ _) :: bs) =
  if sameKind k k' && anchorTyOk t (bindingTy b) then True else anyTargetedTy k t bs
anyTargetedTy k t (_ :: bs) = anyTargetedTy k t bs

||| SOME listed head type has a compatible anchor. Its own empty list is
||| the exhausted search, not an untyped head — the two readings of `[]`
||| have to stay apart, or every typed head would fall through to
||| accepting anything (`badOtherCrossHead` caught exactly that).
public export
anchorFoundSome : Kind -> List CardType -> Bindings -> Bool
anchorFoundSome k [] ctx = False
anchorFoundSome k (t :: ts) ctx = anyTargetedTy k t ctx || anchorFoundSome k ts ctx

||| The "other" presupposition's witness search, over the head types the
||| phrase OFFERS. The empty set is the genuinely untyped head (Arc
||| Trail's "any other target", a player-kind "other") and accepts any
||| same-kind anchor, which is exactly `anyTargeted`; a written head
||| demands a type-compatible anchor. A COORDINATED head offers one type
||| per alternative rather than none: "another target creature or land"
||| is anchored by an earlier creature or by an earlier land, and by an
||| earlier artifact it is not (`badDisjunctiveOtherCrossHead`).
public export
anchorFound : Kind -> List CardType -> Bindings -> Bool
anchorFound k [] ctx = anyTargeted k ctx
anchorFound k (t :: ts) ctx = anchorFoundSome k (t :: ts) ctx

||| A future clause's context: the outer clause's announced targets
||| cross the boundary as SETTLED PARTICULARS — readable like any
||| mention ([CR#603.7c] refers to particular objects determiner-blind)
||| but no longer "targets", because the delayed ability announces its
||| own in its own event ([CR#603.3d,601.2c]), which is where the
||| "other" presupposition stops. A view derived from the determiner;
||| no timing tag exists.
public export
settleTargets : Bindings -> Bindings
settleTargets [] = []
settleTargets (MkBinding TargetD k plur payload :: bs) =
  MkBinding TheD k plur payload :: settleTargets bs
settleTargets (b :: bs) = b :: settleTargets bs

||| Zone visibility ([CR#400.2] — library and hand are hidden zones).
public export
publicZone : Zone -> Bool
publicZone Battlefield = True
publicZone Graveyard = True
publicZone Exile = True
publicZone Hand = False
publicZone Library = False
-- [CR#400.2] lists the stack among the public zones by name.
publicZone Stack = True

||| WHICH way an exposure clause shows a card, and it is the whole of
||| the axis: [CR#701.20a] has revealing "show that card to all players",
||| and [CR#701.20e] has looking follow "the same rules as revealing a
||| card, except that the card is shown only to the specified player".
||| One operation, two audiences, and the specified player is the
||| clause's own subject — which is why the audience is not a slot.
|||
||| Core draws the line in the same place from the other side, with one
||| `Reveal { what, to }` whose optional `to` "names a player instead =
||| 'look at'" (`deckmaste_core/src/action.rs`). A tag here rather than
||| an optional player, because an audience slot would let the grammar
||| write "reveal it to target opponent", which oracle does not.
public export
-- spelling: (construction-owned catalog -- each row is the exposure clause's
-- own verb: LookAt = "<subject> look(s) at <what>", Reveal = "<subject>
-- reveal(s) <what>". Never spelled alone -- see Expose)
data ExposeVerb = LookAt | Reveal

||| Which zones English EXPOSES whole. Only the hand: a graveyard, the
||| battlefield, and exile are public already ([CR#400.2]) so revealing
||| one says nothing, and a library is hidden but too big to show —
||| "reveal your library" is zero lines, what oracle exposes of a library
||| being a positioned SLICE, which is the card complement and not this
||| row.
public export
exposableZone : Zone -> Bool
exposableZone Hand = True
exposableZone Battlefield = False
exposableZone Graveyard = False
exposableZone Exile = False
exposableZone Library = False
exposableZone Stack = False

public export
data ExposableZone : Zone -> Type where
  MkExposableZone : {auto 0 ok : exposableZone z = True} -> ExposableZone z

||| Which zones English SEARCHES. [CR#701.23a] defines the action over
||| any zone and says the hidden ones expressly ("even if it's a hidden
||| zone"); the corpus writes library, graveyard, and hand — two of them
||| hidden, which is the case the rule calls out, and one public. The
||| battlefield and exile are zero lines each: nothing to search for in a
||| zone everyone can already read.
public export
searchableZone : Zone -> Bool
searchableZone Library = True
searchableZone Graveyard = True
searchableZone Hand = True
searchableZone Battlefield = False
searchableZone Exile = False
searchableZone Stack = False

public export
data SearchableZone : Zone -> Type where
  MkSearchableZone : {auto 0 ok : searchableZone z = True} -> SearchableZone z

||| Colon-readability of one mention: tracked objects by zone
||| visibility; players, qualities, and untracked objects pass —
||| structurally, since only `ObjectP` has a zone at all.
public export
pubB : Binding -> Bool
pubB (MkBinding _ _ _ (ObjectP _ (Just z) _ _)) = publicZone z
pubB (MkBinding _ _ _ (ObjectP _ Nothing _ _)) = True
pubB (MkBinding _ _ _ PlayerP) = True
pubB (MkBinding _ _ _ QualityP) = True
pubB (MkBinding _ _ _ (OutcomeP _)) = True  -- what happened is a public fact
pubB (MkBinding _ _ _ GapP) = True          -- so is a comparison's margin
pubB (MkBinding _ _ _ LetterP) = True       -- and so is a value the text defines

||| The cost boundary's filter: a mention a cost leaves in a hidden zone
||| is unreadable past the colon; unmoved and publicly-moved mentions
||| survive. [CR#400.7] fires only on a zone change and [CR#400.7j] is
||| its public-zone exception, so the filter keys on the CURRENT zone,
||| not on having moved.
public export
publicOnly : Bindings -> Bindings
publicOnly [] = []
publicOnly (b :: bs) = if pubB b then b :: publicOnly bs else publicOnly bs

||| A shuffle's filter: a mention still lying in the shuffled library is
||| gone from the discourse. [CR#701.24a] leaves the order known to no
||| player and [CR#701.20d] makes any revealed card that was reordered
||| "stop being revealed and become a new object". Mentions that LEFT the
||| library first are untouched, which is what [CR#701.24b] says of the
||| found cards. Keys on the CURRENT zone, `publicOnly`'s discipline.
public export
notInLibrary : Binding -> Bool
notInLibrary (MkBinding _ _ _ (ObjectP _ (Just z) _ _)) = not (sameZone z Library)
notInLibrary (MkBinding _ _ _ (ObjectP _ Nothing _ _)) = True
notInLibrary (MkBinding _ _ _ PlayerP) = True
notInLibrary (MkBinding _ _ _ QualityP) = True
notInLibrary (MkBinding _ _ _ (OutcomeP _)) = True
notInLibrary (MkBinding _ _ _ GapP) = True
notInLibrary (MkBinding _ _ _ LetterP) = True

public export
shuffledAway : Bindings -> Bindings
shuffledAway [] = []
shuffledAway (b :: bs) =
  if notInLibrary b then b :: shuffledAway bs else shuffledAway bs

||| The current zone of the wildcard pronoun's referent — the unique
||| singular object mention (uniqueness is `It`'s own gate).
public export
zoneOfIt : Bindings -> Maybe Zone
zoneOfIt [] = Nothing
zoneOfIt (MkBinding det Object OneOf (ObjectP ty zn _ _) :: bs) = zn
zoneOfIt (b :: bs) = zoneOfIt bs

||| The current zone of the plural wildcard's group referent.
public export
zoneOfThem : Bindings -> Maybe Zone
zoneOfThem [] = Nothing
zoneOfThem (MkBinding det Object ManyOf (ObjectP ty zn _ _) :: bs) = zn
zoneOfThem (b :: bs) = zoneOfThem bs

||| The noun-word vocabulary — ONE set of words for the sorted reads,
||| its axes kept apart (finding 27 — type words are declared catalog
||| atoms, never intrinsic sorts; the intrinsic words are the engine's
||| own). What differs per read is the ANCHORING: the demonstrative
||| checks its word against the referent's current state (`wordNow`), the
||| participle against the verb event's frame (`verbedMatch`).
public export
-- spelling: (construction-owned catalog -- TypeW t = t's own CardType word,
-- CardW = "card", SpellW = "spell", PlayerW = "player"; PermanentW =
-- "permanent", TokenW = "token"; consumed by
-- That/Those/TheVerbed, e.g. That (TypeW Creature) = "that creature",
-- That SpellW = "that spell". Never spelled alone)
data NounWord = TypeW CardType | CardW | SpellW | PlayerW
              | PermanentW | TokenW

||| The two SURFACES of the participle read, and the round's answer to
||| what "this way" is. The attributive premodifier ("the exiled card")
||| and the postnominal deictic ("the card exiled this way") pick out the
||| same referent under this grammar's uniqueness discipline — a read's
||| obligation is `= 1`, so there is never a second stamp of the same
||| verb for the deictic to disambiguate against — which makes them one
||| construction with two spellings rather than two constructions.
|||
||| It is a TABLE and not a free slot, because the two surfaces are not
||| interchangeable per verb: exile, sacrifice, and discard write both,
||| and DESTROY writes the deictic only — "the destroyed [noun]" is zero
||| lines (`badDestroyedAttributive`).
public export
-- spelling: ["the <verb> <noun>", "the <noun> <verb> this way"] (row
-- order: Attributive/ThisWay; the verb is VerbName's lemma inflected as
-- a past participle either way, and the marking decides only where it
-- stands and whether the deictic follows. Spelled only through
-- TheVerbed / ThoseVerbed)
data VerbedMarking = Attributive | ThisWay

||| Which surface each verb tag writes.
public export
verbedMarkingOk : VerbName -> VerbedMarking -> Bool
verbedMarkingOk Destroy Attributive = False
verbedMarkingOk Destroy ThisWay = True
verbedMarkingOk Sacrifice Attributive = True
verbedMarkingOk Sacrifice ThisWay = True
verbedMarkingOk Exile Attributive = True
verbedMarkingOk Exile ThisWay = True
verbedMarkingOk Discard Attributive = True
verbedMarkingOk Discard ThisWay = True

||| The marking demand as a witness, named apart so a pin says which
||| question refused.
public export
data VerbedMarkingOk : VerbName -> VerbedMarking -> Type where
  MkVerbedMarkingOk : {auto 0 ok : verbedMarkingOk v m = True} -> VerbedMarkingOk v m

public export
tyIs : CardType -> Maybe CardType -> Bool
tyIs t Nothing = False
tyIs t (Just t') = sameCT t t'

||| A tracked non-battlefield zone — where an object answers to "card"
||| ([CR#108.2]); a new Zone must take a side here.
public export
isCardZone : Maybe Zone -> Bool
isCardZone Nothing = False
isCardZone (Just Battlefield) = False
isCardZone (Just Graveyard) = True
isCardZone (Just Exile) = True
isCardZone (Just Hand) = True
isCardZone (Just Library) = True
-- The stack is the second zone where an object does NOT answer to
-- "card": [CR#112.1] calls a card on the stack a SPELL, and the corpus
-- writes "counter target spell" and never "counter target card on the
-- stack".
isCardZone (Just Stack) = False

||| Currently on the battlefield, strictly — the typed noun's
||| demonstrative demand ([CR#110.1]); untracked does not qualify.
public export
onFieldZone : Maybe Zone -> Bool
onFieldZone Nothing = False
onFieldZone (Just Battlefield) = True
onFieldZone (Just Graveyard) = False
onFieldZone (Just Exile) = False
onFieldZone (Just Hand) = False
onFieldZone (Just Library) = False
onFieldZone (Just Stack) = False

||| On the STACK, strictly — where an object answers to "spell"
||| ([CR#112.1]: "a spell is a card on the stack"). Untracked does not
||| qualify, for `isCardZone`'s and `onFieldZone`'s reason: a phrase that
||| says nothing about where its referent is has not said it is here.
public export
onStackZone : Maybe Zone -> Bool
onStackZone Nothing = False
onStackZone (Just Battlefield) = False
onStackZone (Just Graveyard) = False
onStackZone (Just Exile) = False
onStackZone (Just Hand) = False
onStackZone (Just Library) = False
onStackZone (Just Stack) = True

||| Build the stamp a retag writes: the moving verb's tag (if any)
||| plus whether the referent stood on the battlefield BEFORE the
||| move — the at-verb frame.
public export
mkStamp : Maybe VerbName -> Maybe Zone -> Maybe Stamp
mkStamp Nothing oldZn = Nothing
mkStamp (Just v) oldZn = Just (MkStamp v (onFieldZone oldZn))

||| The word's own test, asked once the self has been screened off.
public export
wordReaches : NounWord -> Binding -> Bool
wordReaches (TypeW t) (MkBinding _ _ _ (ObjectP ty zn _ _)) = onFieldZone zn && tyIs t ty
wordReaches (TypeW t) (MkBinding _ _ _ PlayerP) = False
wordReaches (TypeW t) (MkBinding _ _ _ QualityP) = False
wordReaches (TypeW t) (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches (TypeW t) (MkBinding _ _ _ GapP) = False
wordReaches (TypeW t) (MkBinding _ _ _ LetterP) = False
wordReaches CardW (MkBinding _ _ _ (ObjectP _ zn _ _)) = isCardZone zn
wordReaches CardW (MkBinding _ _ _ PlayerP) = False
wordReaches CardW (MkBinding _ _ _ QualityP) = False
wordReaches CardW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches CardW (MkBinding _ _ _ GapP) = False
wordReaches CardW (MkBinding _ _ _ LetterP) = False
-- The stack's carrier word, and [CR#112.1]'s identity is what makes it
-- one rather than a corpus count: "a spell is a card on the stack", so
-- the same object answers to "card" nowhere the stack is and to "spell"
-- only there.
wordReaches SpellW (MkBinding _ _ _ (ObjectP _ zn _ _)) = onStackZone zn
wordReaches SpellW (MkBinding _ _ _ PlayerP) = False
wordReaches SpellW (MkBinding _ _ _ QualityP) = False
wordReaches SpellW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches SpellW (MkBinding _ _ _ GapP) = False
wordReaches SpellW (MkBinding _ _ _ LetterP) = False
wordReaches PlayerW (MkBinding _ _ _ (ObjectP _ _ _ _)) = False
wordReaches PlayerW (MkBinding _ _ _ PlayerP) = True
wordReaches PlayerW (MkBinding _ _ _ QualityP) = False
wordReaches PlayerW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches PlayerW (MkBinding _ _ _ GapP) = False
wordReaches PlayerW (MkBinding _ _ _ LetterP) = False
-- the battlefield IS the word's whole test ([CR#110.1] — "a permanent is
-- a card or token on the battlefield", and it stops being one when it
-- moves away): no type, no provenance, no other zone.
wordReaches PermanentW (MkBinding _ _ _ (ObjectP _ zn _ _)) = onFieldZone zn
wordReaches PermanentW (MkBinding _ _ _ PlayerP) = False
wordReaches PermanentW (MkBinding _ _ _ QualityP) = False
wordReaches PermanentW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches PermanentW (MkBinding _ _ _ GapP) = False
wordReaches PermanentW (MkBinding _ _ _ LetterP) = False
-- battlefield AND minted-as-token: a token that has left the battlefield
-- has ceased to exist ([CR#111.7]), so the current-state word reaches
-- nothing there; a card-born referent is never "that token" however it
-- stands ([CR#111.1]).
wordReaches TokenW (MkBinding _ _ _ (ObjectP _ zn _ og)) =
  onFieldZone zn && isTokenOrigin og
wordReaches TokenW (MkBinding _ _ _ PlayerP) = False
wordReaches TokenW (MkBinding _ _ _ QualityP) = False
wordReaches TokenW (MkBinding _ _ _ (OutcomeP _)) = False
wordReaches TokenW (MkBinding _ _ _ GapP) = False
wordReaches TokenW (MkBinding _ _ _ LetterP) = False

||| The demonstrative's noun check — CURRENT-state anchoring, the
||| carrier discipline ([CR#109.2,110.1]): a type word demands the
||| referent currently answer to it (on the battlefield, projected
||| type matching), the CARD word a tracked non-battlefield object
||| ([CR#108.2]), the PLAYER word a player. Quality mentions answer
||| to no noun word.
public export
wordNow : NounWord -> Binding -> Bool
-- THE DEMONSTRATIVE SKIPS THE SELF, and English is why rather than a
-- convenience: "that creature" never means the speaker, so a trigger's
-- own subject is not among the things "that creature" can pick out even
-- when it is the only creature the sentence mentioned. One clause, and
-- every demonstrative reader inherits it -- the counts, the zone read,
-- and the retag scans all route through this function.
wordNow w b = case b.det of
                SelfD => False
                _ => wordReaches w b


public export
kindOfW : NounWord -> Kind
kindOfW (TypeW _) = Object
kindOfW CardW = Object
kindOfW SpellW = Object
kindOfW PlayerW = Player
kindOfW PermanentW = Object
kindOfW TokenW = Object

||| The PROVENANCE half of the participle read: is this mention the one
||| the named verb event stamped? That is the whole of what the participle
||| contributes as a determiner — "the sacrificed …" picks out the
||| referent of the LAST sacrifice (finding 26) — and it says nothing
||| about which word may then describe it. Core keeps the same axis apart
||| (`Reference::Bound`/`Linked` against `That(Sort)`, `reference.rs`).
public export
stampedBy : VerbName -> Stamp -> Bool
stampedBy v (MkStamp v' _) = sameVerb v v'

||| The WORD half, over that same stamped mention: a TYPE word demands
||| the referent stood on the battlefield AT the verb (the stamp's
||| `wasField` — a bare type word denotes a permanent [CR#109.2], so "the
||| discarded creature" is unwritten; hands lose cards, not creatures)
||| plus its projected type; the intrinsic CARD word checks the CURRENT
||| zone; no participle reads a player. Each row reads the word's own
||| anchor and nothing about the verb — the axis separation the
||| demonstrative shares (`wordNow`, finding 28).
public export
verbedWordOk : NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
verbedWordOk (TypeW t) (MkStamp _ wasF) ty zn = wasF && tyIs t ty
verbedWordOk CardW st ty zn = isCardZone zn
-- The spell word takes a participle for the same reason the card word
-- does — it names a current zone rather than a remembered state — and
-- the corpus writes one ("the spell cast this way").
verbedWordOk SpellW st ty zn = onStackZone zn
verbedWordOk PlayerW st ty zn = False
-- the participial read asks only that the referent stood on the
-- battlefield at the stamped event — "the sacrificed permanent" projects
-- no type (Broadside Bombardiers reads it through a possessive), which
-- is the TypeW row minus its type demand.
verbedWordOk PermanentW (MkStamp _ wasF) ty zn = wasF
-- no participial token read is measured ("the sacrificed token" was not
-- counted by the recon), and the signature carries no origin to check;
-- closed at False and ledgered rather than widened on no line.
verbedWordOk TokenW st ty zn = False

||| The participle's two halves as the one check the scans want: the
||| provenance picks the mention, the word describes it.
public export
stampWordOk : VerbName -> NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
stampWordOk v w st ty zn = stampedBy v st && verbedWordOk w st ty zn

||| Does "the [verbed] [noun]" reach this binding? Singular, stamped,
||| noun word compatible (`stampWordOk`).
public export
verbedMatch : VerbName -> NounWord -> Binding -> Bool
verbedMatch v w (MkBinding _ _ OneOf (ObjectP ty zn (Just st) _)) = stampWordOk v w st ty zn
verbedMatch v w (MkBinding _ _ OneOf (ObjectP _ _ Nothing _)) = False
verbedMatch v w (MkBinding _ _ ManyOf (ObjectP _ _ _ _)) = False
verbedMatch v w (MkBinding _ _ _ PlayerP) = False
verbedMatch v w (MkBinding _ _ _ QualityP) = False
verbedMatch v w (MkBinding _ _ _ (OutcomeP _)) = False
verbedMatch v w (MkBinding _ _ _ GapP) = False
verbedMatch v w (MkBinding _ _ _ LetterP) = False

||| The GROUP twin — "[the] cards [verbed] this way", and the one the
||| participle read had been refusing since it was minted: `verbedMatch`
||| answers `False` to every `ManyOf` binding, so a plural action's
||| participants had no participle to be read by. The corpus writes the
||| plural at least as often as the singular where the family lives,
||| because the actions the construction reads back are group actions.
public export
verbedMatchMany : VerbName -> NounWord -> Binding -> Bool
verbedMatchMany v w (MkBinding _ _ ManyOf (ObjectP ty zn (Just st) _)) = stampWordOk v w st ty zn
verbedMatchMany v w (MkBinding _ _ ManyOf (ObjectP _ _ Nothing _)) = False
verbedMatchMany v w (MkBinding _ _ OneOf (ObjectP _ _ _ _)) = False
verbedMatchMany v w (MkBinding _ _ _ PlayerP) = False
verbedMatchMany v w (MkBinding _ _ _ QualityP) = False
verbedMatchMany v w (MkBinding _ _ _ (OutcomeP _)) = False
verbedMatchMany v w (MkBinding _ _ _ GapP) = False
verbedMatchMany v w (MkBinding _ _ _ LetterP) = False

||| Mentions the definite participle read reaches — its obligation is
||| `= 1`, the same strict uniqueness as every other read.
public export
countVerbed : VerbName -> NounWord -> Bindings -> Nat
countVerbed v w [] = Z
countVerbed v w (b :: bs) =
  if verbedMatch v w b then S (countVerbed v w bs) else countVerbed v w bs

||| Group mentions the plural participle read reaches — the same strict
||| `= 1` uniqueness, counted over groups.
public export
countManyVerbed : VerbName -> NounWord -> Bindings -> Nat
countManyVerbed v w [] = Z
countManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then S (countManyVerbed v w bs) else countManyVerbed v w bs

||| The current zone of the plural participle read's referent.
public export
zoneOfManyVerbed : VerbName -> NounWord -> Bindings -> Maybe Zone
zoneOfManyVerbed v w [] = Nothing
zoneOfManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then bindingZone b else zoneOfManyVerbed v w bs

||| The projected head type of the plural participle read's referent.
public export
tyOfManyVerbed : VerbName -> NounWord -> Bindings -> Maybe CardType
tyOfManyVerbed v w [] = Nothing
tyOfManyVerbed v w (b :: bs) =
  if verbedMatchMany v w b then bindingTy b else tyOfManyVerbed v w bs

||| The current zone of the participle read's referent.
public export
zoneOfVerbed : VerbName -> NounWord -> Bindings -> Maybe Zone
zoneOfVerbed v w [] = Nothing
zoneOfVerbed v w (b :: bs) =
  if verbedMatch v w b then bindingZone b else zoneOfVerbed v w bs

||| Singular mentions the demonstrative's word currently reaches —
||| `That`'s obligation is `= 1` (strict uniqueness after the filter;
||| there is no nearest-wins).
public export
countWord : NounWord -> Bindings -> Nat
countWord w [] = Z
countWord w (b :: bs) =
  case (b.plur, wordNow w b) of
    (OneOf, True) => S (countWord w bs)
    _ => countWord w bs

||| Group mentions the word currently reaches — the plural twin.
public export
countManyWord : NounWord -> Bindings -> Nat
countManyWord w [] = Z
countManyWord w (b :: bs) =
  case (b.plur, wordNow w b) of
    (ManyOf, True) => S (countManyWord w bs)
    _ => countManyWord w bs

||| The current zone of a sorted demonstrative's referent.
public export
zoneOfThat : NounWord -> Bindings -> Maybe Zone
zoneOfThat w [] = Nothing
zoneOfThat w (b :: bs) =
  case (b.plur, wordNow w b) of
    (OneOf, True) => bindingZone b
    _ => zoneOfThat w bs

||| The current zone of a sorted plural demonstrative's group referent.
public export
zoneOfThose : NounWord -> Bindings -> Maybe Zone
zoneOfThose w [] = Nothing
zoneOfThose w (b :: bs) =
  case (b.plur, wordNow w b) of
    (ManyOf, True) => bindingZone b
    _ => zoneOfThose w bs

||| The projected-type twins of the zone-of family — what a read's
||| referent projects, for verb slots that demand a head type.
public export
tyOfIt : Bindings -> Maybe CardType
tyOfIt [] = Nothing
tyOfIt (MkBinding det Object OneOf (ObjectP ty zn pv _) :: bs) = ty
tyOfIt (b :: bs) = tyOfIt bs

public export
tyOfThem : Bindings -> Maybe CardType
tyOfThem [] = Nothing
tyOfThem (MkBinding det Object ManyOf (ObjectP ty zn pv _) :: bs) = ty
tyOfThem (b :: bs) = tyOfThem bs

public export
tyOfThat : NounWord -> Bindings -> Maybe CardType
tyOfThat w [] = Nothing
tyOfThat w (b :: bs) =
  case (b.plur, wordNow w b) of
    (OneOf, True) => bindingTy b
    _ => tyOfThat w bs

public export
tyOfThose : NounWord -> Bindings -> Maybe CardType
tyOfThose w [] = Nothing
tyOfThose w (b :: bs) =
  case (b.plur, wordNow w b) of
    (ManyOf, True) => bindingTy b
    _ => tyOfThose w bs

public export
tyOfVerbed : VerbName -> NounWord -> Bindings -> Maybe CardType
tyOfVerbed v w [] = Nothing
tyOfVerbed v w (b :: bs) =
  if verbedMatch v w b then bindingTy b else tyOfVerbed v w bs

||| Player mentions of either number — the antecedent pool for the
||| possessive chooser pronoun ("of THEIR choice"): a singular subject
||| or one distributive group.
public export
countChoosers : Bindings -> Nat
countChoosers bs = countOnes Player bs + countManys Player bs

||| The zone half of sacrifice's implicit restriction ([CR#701.21a] —
||| only a permanent can be sacrificed), and of every other
||| battlefield-demanding slot: the referent's zone must BE the
||| battlefield. The permissive untracked row is gone — it existed for
||| the sorted self-reference, which now projects its own zone
||| ([CR#109.2], `nounZone (AsType …)`). The controller half needs
||| fold-state the context does not carry (not-settled).
public export
data OnBattlefield : Maybe Zone -> Type where
  OnField : OnBattlefield (Just Battlefield)

||| The attested next-untap-step counts — the timed restriction's one
||| independent endpoint value, a closed table over the written number:
||| one (66 lines; the numeral is unwritten and "step" singular) and two
||| (one line, Telekinesis; the word "two" and plural "steps"). Three and
||| up are unwritten English (`badUntapNextThree`), and the table is what
||| keeps them so. Construction-owned: the endpoint occurs zero times
||| with any non-untap restriction, so no `Duration`, `SpanOk`, or
||| turn-part cell opens for it.
public export
data NextUntapCount : Nat -> Type where
  OneNextStep : NextUntapCount 1
  TwoNextSteps : NextUntapCount 2

||| The two PLURAL PLAYER words — the bare plural "players" and the
||| possessive plural "your opponents" — which `Noun`'s `PlayerGroup` row
||| spells and which nothing else does. This retires `CapDomain`, the
||| three-cell enum chapter thirty-nine minted in this vocabulary's place
||| and promised to revisit: its third cell was "you", which is `You`, so
||| the cap now takes an ordinary player noun and the two plural phrases
||| are ordinary player nouns beside it.
|||
||| The vocabulary is CLOSED because the corpus is. Sentence-initially the
||| bare plural is 98 cards and the possessive plural 42 segments, and
||| there is no third phrase of this shape: "all players" as a subject is
||| written ZERO times (its nine occurrences are the comparative "among
||| all players" and the possessive "all players' hands") and "all
||| opponents" likewise zero (six occurrences, all "all opponents'
||| graveyards"). So the alternative shape — a determiner row over a
||| PREDICATE, `AllOf`'s arrangement — is refused by the surfaces
||| themselves: `AllOf` spells the word "all", which neither of these
||| phrases writes, and a predicate slot would additionally mint the
||| possessive over arbitrary descriptions ("your opponents who control an
||| artifact", zero lines). Exactly one corpus line modifies the bare
||| plural at all — Aurelia's Fury's "Players dealt damage this way" —
||| and it is blocked twice over besides, so the general row would be
||| minted for a line it still could not spell.
|||
||| The two words differ in DENOTATION and in nothing this grammar reads:
||| [CR#102.1] makes a player "one of the people in the game", so the bare
||| plural is all of them, and an opponent is someone a player is playing
||| against ([CR#102.2,102.3]), so the possessive is all of them but you.
||| No table in this vocabulary asks which, which is why they are one
||| indexed row rather than two constructors — finding 275's lesson read
||| the other way round.
public export
-- spelling: ["players", "your opponents"] (row order:
-- AllPlayers/YourOpponents. The name of the first is its DENOTATION and
-- not its surface: the word written is the bare plural "players", never
-- "all players". Through `StaticEffect.CantUntapMoreThan` each also
-- fixes the step possessive that closes the sentence, "their untap
-- steps" -- but that is the plural agreeing with a plural subject, which
-- `You` answers "your untap step" by the same rule)
data PlayerGroupWord = AllPlayers | YourOpponents

public export
samePlayerGroupWord : PlayerGroupWord -> PlayerGroupWord -> Bool
samePlayerGroupWord AllPlayers AllPlayers = True
samePlayerGroupWord AllPlayers _ = False
samePlayerGroupWord YourOpponents YourOpponents = True
samePlayerGroupWord YourOpponents _ = False

||| The attested cap bounds — the same closed-table-over-a-written-number
||| move `NextUntapCount` makes, and the same two cells: one (five lines;
||| the singular set word follows) and two (two lines, Static Orb and the
||| emblem; "permanents" plural). Three and up are unwritten English
||| (`badUntapCapThree`). The only bound PHRASING anywhere is "can't untap
||| more than N" — no "at most", no "untap only" (Storage Matrix's "can
||| untap only permanents of the chosen type" is a different construction
||| entirely, recorded and not minted) — so the comparator is
||| construction-owned and this table carries the number alone.
public export
data CapBound : Nat -> Type where
  OneUntap : CapBound 1
  TwoUntaps : CapBound 2

||| How far back a history query scans — [CR#608.2i]'s look-back-in-time
||| window, and a SEPARATE vocabulary from `Duration` rather than a row in
||| it. Core draws the same line in the same place and its module doc is
||| the design's anchor: `Timing` is a permission window and `Lookback` a
||| history window, "never conflating the duration and history readings of
||| 'this turn'" (`deckmaste_core/src/temporal.rs`). The Idris-side
||| evidence is `Duration`'s own forward-only commitment — every endpoint
||| it spells is a boundary the game has not reached — so "until end of
||| turn" and "this turn" in a lookback are two different words wearing
||| one spelling, and a shared enum would have made the difference
||| unsayable.
|||
||| Three rows, each with real membership in the CONDITION position:
||| `ThisTurn` is the mass (296 lines), `ThisCombat` seventeen (Kytheon,
||| the velocity Vehicles, the pack-tactics family, Tolsimir), `LastTurn`
||| eleven ("if a player cast two or more spells last turn", "if you lost
||| life last turn"). The last of those the round's brief did not list and
||| the corpus does; core carries it too.
|||
||| Two windows core has are NOT minted here, each refused on its own
||| measurement. `ThisGame` reads backward on twenty-four lines and almost
||| all of them are a specialised count — "for each time you've cast your
||| commander from the command zone this game" and its family, plus named
||| card counts and Ring temptations — none of which routes through this
||| query's event vocabulary. The `SinceYour` family is five lines across
||| three spellings ("since your last turn", "since your last upkeep",
||| "since your last turn ended"), two syntactic positions and two
||| possessors (O-Kagachi writes "during their last turn"), which is too
||| thin and too varied to close a row over. Both are ledgered.
|||
||| It lives in its OWN NAMESPACE, and that is the separation enforced
||| rather than merely documented: `Duration` already owns the words
||| `ThisTurn` and `LastTurn`, so a shared top-level name space would have
||| made the two vocabularies collide on the very spelling core warns
||| about. `Counter.Delta`'s arrangement, for the same reason — one
||| English word, two grammatical objects, told apart by where they live.
namespace Lookback
  public export
  -- spelling: ["this turn", "this combat", "last turn"] (row order:
  -- ThisTurn/ThisCombat/LastTurn; the adverbial closing the clause, after
  -- the past-tense verb phrase. Spelled only through Condition.Happened
  -- and Amount.EventCount)
  data Lookback = ThisTurn | ThisCombat | LastTurn

||| The same demand at the STACK, and a separate type for `OnBattlefield`'s
||| reason: the carrier that reaches a spell has to say the phrase names
||| one ([CR#112.1] — "a spell is a card on the stack"), where a
||| `zoneFits` silence would let an untracked phrase through on saying
||| nothing. The class word is exactly that phrase — "any target" names
||| [CR#115.4]'s damage class and places its referent nowhere — and
||| [CR#115.4] rules it out of this position in as many words: the class
||| spans creature, player, planeswalker and battle, and a spell "can't be
||| chosen this way" (`badCounterAnyTarget`, `badCastsAnyTarget`).
public export
data OnStack : Maybe Zone -> Type where
  OnTheStack : OnStack (Just Stack)

||| WHERE a counter may be put and taken off — the closed full-row table
||| the battlefield demand above was standing in for, and the one gate in
||| this file that opens a SECOND zone. Chapter nineteen answered the
||| whole question `OnBattlefield`, which was right about the graveyard
||| and wrong about exile: the rules put counters outside the battlefield
||| in as many words ([CR#122.1a] gives a +X/+Y counter its meaning "on a
||| creature card in a zone other than the battlefield", [CR#122.1b] the
||| same for a keyword counter), and English writes them there — lines
||| exile a card WITH counters on it, and the ordinary put and remove
||| verbs reach that card afterwards (Jhoira of the Ghitu's "Put four
||| time counters on the exiled card").
|||
||| The other four zones are measured silences and stay shut. Not one
||| corpus line puts a counter on a card in a GRAVEYARD: the lines that
||| write "counter" and "in a graveyard" together put the counter on a
||| battlefield object and read the graveyard for a COUNT, or return the
||| card to the battlefield first (`badPutCountersGraveyard`,
||| `badRemoveCountersDead`). The hand and the library are the same
||| silence, and the STACK writes none either — which the rules explain
||| rather than merely record: [CR#122.6] reads a bare counter instruction
||| as the battlefield's unless something says otherwise, so a zone gets a
||| row here only when oracle NAMES it, and exile is the one it names.
|||
||| Which KIND is not an axis of this table: many kinds ride an exile and
||| more arrive by the put verb afterwards. The zone is open to counters,
||| not to a counter.
public export
counterZone : Maybe Zone -> Bool
counterZone Nothing = False
counterZone (Just Battlefield) = True
counterZone (Just Graveyard) = False
counterZone (Just Exile) = True
counterZone (Just Hand) = False
counterZone (Just Library) = False
counterZone (Just Stack) = False

||| The counter clause's zone demand as a witness, named apart from
||| `OnBattlefield` so a pin says which question refused — and kept a
||| separate type rather than a widened `OnBattlefield` because the other
||| eleven battlefield-demanding slots (destroy, sacrifice, tap, fight,
||| the copular reads, the stat deltas) did NOT widen.
public export
data CounterHolder : Maybe Zone -> Type where
  MkCounterHolder : {auto 0 ok : counterZone z = True} -> CounterHolder z

||| A reference's OWN zone against the zone its description reads —
||| `zonesAgree`'s agreement rule pointed at the one place the subject is
||| external to the phrase. A noun phrase places its referent itself, so
||| "creature card in your graveyard" has its zone story checked inside
||| the conjunction; a copular condition instead says something about a
||| referent placed somewhere else already, and "it's in a graveyard" said
||| of a battlefield mention describes nothing ([CR#109.2a] — a
||| description naming a zone means an object in THAT zone, so it picks
||| out nothing the subject could be). Silence on either side is no
||| evidence and passes.
public export
zoneFits : Maybe Zone -> Maybe Zone -> Bool
zoneFits Nothing _ = True
zoneFits (Just _) Nothing = True
zoneFits (Just a) (Just b) = sameZone a b

||| The copular condition's zone gate as a witness (`OnBattlefield`'s
||| shape — a plain `Maybe Zone` relation, so the mutual block below can
||| call it from a constructor's type). It is chapter twenty-one's other
||| half of the trailing-condition timing correction: with the condition
||| typed in `preIntro`, the subject of "Destroy target creature if it's
||| in a graveyard" is on the battlefield where its phrase announced it,
||| and this is what notices (`badTrailingPostStateZone`).
public export
data ZoneFits : Maybe Zone -> Maybe Zone -> Type where
  MkZoneFits : {auto 0 ok : zoneFits subj desc = True} -> ZoneFits subj desc

||| What "target" can take ([CR#115.1] — objects and players; a
||| quality is choosable, never targetable).
public export
data Targetable : Kind -> Type where
  ObjectTgt : Targetable Object
  PlayerTgt : Targetable Player

||| Damageable head types ([CR#120.1a] — damage can't be dealt to an
||| object that's not a battle, a creature, or a planeswalker): one row,
||| for the one damageable type this vocabulary has a word for. The class
||| word "any target" does not arrive here — it names the [CR#115.4] class
||| itself and has its own recipient row — and neither does the untyped
||| head. That row read a missing type word as "no evidence of an illegal
||| one", which was defensible while nothing could project `Nothing`
||| deliberately; a disjunctive head does exactly that, and honestly
||| ("artifact or enchantment" fixes no type), so keeping the row would
||| have made every disjunction damageable — including the two types
||| [CR#120.1a] names as the ones damage can't reach
||| (`badDamageDisjunctHead`).
public export
data DamageableTy : Maybe CardType -> Type where
  DamCreature : DamageableTy (Just Creature)

||| The kinds a noun PHRASE can describe — objects, players, chosen
||| qualities. Outcomes are clause-introduced only: no determiner
||| phrase binds one, which is what keeps `bindFor` total.
public export
data Phrasal : Kind -> Type where
  PhObject : Phrasal Object
  PhPlayer : Phrasal Player
  PhQuality : Phrasal (Quality q)

public export
targetablePhrasal : Targetable k -> Phrasal k
targetablePhrasal ObjectTgt = PhObject
targetablePhrasal PlayerTgt = PhPlayer

-- ===== Abilities (the granted-ability vocabulary, minimally) =====

||| Keyword abilities, as macro NAMES mirroring
||| `plugins/builtin/macros/keyword/` — parameterized keywords spell
||| their parameters explicitly (e.g. a from-quality as `Maybe`, written
||| `Nothing` in the plain form), never as defaults. A row here is
||| STANDING, and core decides who gets it: `Trample`, `Vigilance`,
||| `Deathtouch`, `DoubleStrike` and `FirstStrike` have NO macro under
||| that directory because the engine implements them itself, and a word
||| the engine owns cannot be spelled as anything else. Every keyword
||| that DOES have a macro there is a composite of abilities, and
||| composites are the composite carrier's to spell (ledger), never a row
||| of their own; `Haste` and `Flying` are the two composite rows here,
||| left where the cards that use them put them.
|||
||| The GRANT round adds four more composite rows on the same terms —
||| convoke, improvise, storm and lifelink all have macros under that
||| directory and are named here, not implemented here. What they are FOR
||| is a class of spells being granted an ability it was not printed with,
||| which is a sentence the vocabulary could not write with any of the
||| original seven: six of those seven never function on a spell at all,
||| and the seventh (deathtouch) was the family's one enum word already.
||| Affinity is deliberately NOT here: every one of its five lines writes a
||| parameter ("affinity for artifacts", "for Auras", "for creatures"), and
||| a parameterized keyword needs a slot this row does not have.
public export
-- spelling: ["haste", "flying", "trample", "vigilance", "deathtouch",
-- "double strike", "first strike", "convoke", "improvise", "storm",
-- "lifelink"] (bare keyword-ability lines, no params/cost; exact row order
-- follows Keyword)
data Keyword = Haste | Flying | Trample | Vigilance | Deathtouch
             | DoubleStrike | FirstStrike
             | Convoke | Improvise | Storm | Lifelink

||| Keyword equality, per-row catch-alls. This is vocabulary only: it does
||| not classify or implement keyword mechanics.
public export
sameKeyword : Keyword -> Keyword -> Bool
sameKeyword Haste Haste = True
sameKeyword Haste _ = False
sameKeyword Flying Flying = True
sameKeyword Flying _ = False
sameKeyword Trample Trample = True
sameKeyword Trample _ = False
sameKeyword Vigilance Vigilance = True
sameKeyword Vigilance _ = False
sameKeyword Deathtouch Deathtouch = True
sameKeyword Deathtouch _ = False
sameKeyword DoubleStrike DoubleStrike = True
sameKeyword DoubleStrike _ = False
sameKeyword FirstStrike FirstStrike = True
sameKeyword FirstStrike _ = False
sameKeyword Convoke Convoke = True
sameKeyword Convoke _ = False
sameKeyword Improvise Improvise = True
sameKeyword Improvise _ = False
sameKeyword Storm Storm = True
sameKeyword Storm _ = False
sameKeyword Lifelink Lifelink = True
sameKeyword Lifelink _ = False

-- (The `Ability` CONTAINER that carries these — the type above `Effect` —
-- lives in the mutual block, its activated row needing `Cost` and
-- `Effect`.)

-- ===== Colors, subtypes, counters, and token characteristics =====

||| The five colors ([CR#105.1]; colorless is not one of them, which is
||| why the token's color slot spells its absence as an empty list rather
||| than growing a sixth row). Ported from core ALMOST VERBATIM by
||| explicit ruling — `deckmaste_core/src/color.rs` opens with exactly
||| these five rows in this order — and this is the one closed vocabulary
||| the workbench does NOT whittle to witnessed rows: a color is a
||| rules-fixed catalog, not a construction, so a missing row would be a
||| hole in the rules rather than an unattested phrase.
|||
||| Its own namespace, the `Verb` split's reason: the color `Color` and
||| the quality sort `Color` ("Choose a color", `QualitySort`) are
||| distinct words, and Idris resolves the two by the type each stands in.
namespace Chroma
  public export
  -- spelling: ["white", "blue", "black", "red", "green"] (row order:
  -- White/Blue/Black/Red/Green -- each row is the color's own word, written
  -- inside a token's characteristics and coordinated with "and" from two up
  -- ("a 2/2 red and green Satyr creature token", Xenagos the Reveler);
  -- never spelled alone)
  data Color = White | Blue | Black | Red | Green

  ||| A color or the absence of one — the piece chapter nineteen minted
  ||| `Color` without and named as the mana-symbol channel's ([CR#107.4c]:
  ||| "{C}" is one colorless mana, and colorless is not a color
  ||| [CR#105.1]). It lands where core puts it, beside `Color` in the same
  ||| file (`deckmaste_core/src/color.rs`) rather than beside the symbols
  ||| that consume it, and on the same ruling: a closed rules catalog, so
  ||| both rows stand whether or not a bench line spells them.
  public export
  -- spelling: (component-owned -- never a word. Written inside a symbol's
  -- braces: "C" for Colorless, the color's own letter for OfColor -- core's
  -- `ColorOrColorless::from_code`, whose "C" arm is the one this row adds)
  data ColorOrColorless = Colorless | OfColor Color

public export
sameColor : Color -> Color -> Bool
sameColor White White = True
sameColor White _ = False
sameColor Blue Blue = True
sameColor Blue _ = False
sameColor Black Black = True
sameColor Black _ = False
sameColor Red Red = True
sameColor Red _ = False
sameColor Green Green = True
sameColor Green _ = False

-- ===== Mana symbols and printed mana costs =====

||| The component a hybrid or Phyrexian symbol is built OUT of: a generic
||| amount or one specific kind of mana ([CR#107.4b] makes a numeral
||| generic, [CR#107.4a,107.4c] make the six specific pips). Ported from
||| core almost verbatim by the same explicit ruling `Color` was
||| (`deckmaste_core/src/mana.rs`, `SimpleManaSymbol`), and the settled
||| Idris spec agrees row for row (`Semantics.idr`): a rules-fixed
||| catalog, not whittled to witnessed rows.
public export
-- spelling: (component-owned -- never spelled alone. Written inside a
-- ManaSymbol's braces: Generic n as the numeral "{2}", Specific as the
-- one-letter code "{W}"/"{C}"; the halves of a hybrid are joined by "/")
data SimpleManaSymbol = Generic Nat | Specific ColorOrColorless

||| A hybrid's two halves are DIFFERENT mana. [CR#107.4e] makes a hybrid
||| symbol "represent a cost that can be paid in one of two ways", and two
||| ways spelled the same word is one way: the printed set has no "{W/W}",
||| and the generic and colorless halves clash with nothing.
public export
halvesDistinct : SimpleManaSymbol -> Color -> Bool
halvesDistinct (Generic _) d = True
halvesDistinct (Specific Colorless) d = True
halvesDistinct (Specific (OfColor c)) d = not (sameColor c d)

||| The hybrid's distinctness demand as a witness.
public export
data HalvesDistinct : SimpleManaSymbol -> Color -> Type where
  MkHalvesDistinct : {auto 0 ok : halvesDistinct l r = True} -> HalvesDistinct l r

||| A hybrid Phyrexian symbol names two DIFFERENT colors, and [CR#107.4f]
||| says so by counting: it lists "ten hybrid Phyrexian mana symbols",
||| which is the number of unordered pairs of five distinct colors. The
||| plain five take no second color at all, so the silence passes.
public export
phyrexianDistinct : Color -> Maybe Color -> Bool
phyrexianDistinct c Nothing = True
phyrexianDistinct c (Just d) = not (sameColor c d)

||| The Phyrexian symbol's distinctness demand as a witness.
public export
data PhyrexianDistinct : Color -> Maybe Color -> Type where
  MkPhyrexianDistinct : {auto 0 ok : phyrexianDistinct c d = True} ->
                        PhyrexianDistinct c d

||| A printed mana symbol ([CR#107.4] lists the closed set). The five
||| colored, the colorless, and the numerals ride `Simple`; the hybrid
||| families are ONE compositional row, so `{W/U}` is `Hybrid (Specific
||| (OfColor White)) Blue`, the monocolored `{2/B}` is `Hybrid (Generic 2)
||| Black`, and the colorless-hybrid `{C/W}` is `Hybrid (Specific
||| Colorless) White` ([CR#107.4e] — "a hybrid mana symbol is also a
||| colored mana symbol, even if one of its components is colorless").
||| `Phyrexian` takes a COLOR rather than a `SimpleManaSymbol` on its left,
||| which is [CR#107.4f]'s own list making the type carry its
||| well-formedness: the fifteen printed Phyrexian symbols are five colored
||| and ten hybrid-colored, and there is no `{2/P}` or `{C/P}` to write.
||| Deliberately a hair more permissive than the printed set exactly where
||| core is — an unprinted `{5/W}` is representable — which core marks as
||| variant headroom and this file inherits rather than re-litigates.
|||
||| This is the PRINTED cost language and not what a mana ability
||| produces; core keeps `ManaSpec` apart for that, and no clause here
||| produces mana at all (ledger).
public export
-- spelling: (each symbol writes itself between braces, and a cost is the
-- symbols run together with no separator: Simple (Generic n) = "{n}",
-- Simple (Specific c) = "{W}".."{G}"/"{C}", Hybrid l r = "{l/r}" on the two
-- halves' own codes, Phyrexian c Nothing = "{W/P}", Phyrexian c (Just d) =
-- "{W/U/P}", Variable = "{X}", SnowMana = "{S}")
data ManaSymbol : Type where
  Simple : SimpleManaSymbol -> ManaSymbol
  Hybrid : (l : SimpleManaSymbol) -> (r : Color) ->
           {auto 0 ds : HalvesDistinct l r} -> ManaSymbol
  Phyrexian : (c : Color) -> (d : Maybe Color) ->
              {auto 0 ds : PhyrexianDistinct c d} -> ManaSymbol
  Variable : ManaSymbol
  SnowMana : ManaSymbol

||| A printed mana cost: the symbol sequence, in the order the card
||| writes it ([CR#202.1]; [CR#202.1a] makes paying it a per-symbol
||| match). A LIST because that is all oracle's spelling is, "{1}{R}"
||| being two symbols run together, and because core's `ManaCost` is the
||| same newtype over a symbol slice. The empty list is the {0} cost's
||| neighbour and not the same thing: [CR#118.5] makes "{0}" a payment of
||| nothing, written `[Simple (Generic 0)]`, where the empty list is
||| core's "no mana cost" ([CR#202.1b], unpayable) — a distinction the
||| type keeps for free and no clause here exercises.
public export
ManaCost : Type
ManaCost = List ManaSymbol

||| A cost COMPONENT's symbol run is written — the one place the empty
||| list is not the [CR#202.1b] "no mana cost" it is on a card. Before a
||| colon the run IS the component's whole spelling, so an empty one
||| spells an activation line opening on a bare colon, which no corpus
||| line writes; the payment of nothing is "{0}" ([CR#118.5]), a symbol,
||| and the run holding it is one element long (`badEmptyManaCost`).
public export
manaRunWritten : ManaCost -> Bool
manaRunWritten [] = False
manaRunWritten (_ :: _) = True

||| The written-run demand as a witness.
public export
data ManaRun : ManaCost -> Type where
  MkManaRun : {auto 0 ok : manaRunWritten c = True} -> ManaRun c

||| Subtypes, as catalog atoms ([CR#205.3m] creature types;
||| [CR#205.3i,305.6] the five basic land types). Witnessed rows, and
||| DELIBERATELY not a port of core's shape: core declares subtypes as
||| open plugin data (`Subtype { name, types, confers }`,
||| `deckmaste_core/src/type.rs`) resolved through a generated catalog of
||| hundreds, and a bench cannot witness hundreds of rows. So a subtype
||| grows a row here exactly as `CardType` does, one card at a time; the
||| port-the-catalog alternative is ledgered rather than taken.
public export
-- spelling: (construction-owned catalog -- each row is the subtype's own
-- word, capitalised, written inside a token's characteristics ("Zombie
-- Army"), as a bare noun head ("an Army you control"), or after "becomes a"
-- in a type-addition clause; never spelled alone)
data Subtype = Zombie | Army | Soldier | Thopter | Construct | Fractal
             | Coward | Demon | Angel | Elemental | Plant | Dragon | Plains | Island | Swamp
             | Mountain | Forest | Goblin | Equipment | Avatar | Insect
             | Elder | Dinosaur | Human | Advisor | Wizard | Shaman
             | Vedalken | Artificer | Knight | Myr
             -- the first ENCHANTMENT type ([CR#205.3h], which sends the
             -- word on to [CR#303.4]'s Aura rules), and the first row
             -- minted for a SELF-REFERENCE rather than for a description:
             -- "this Aura" is how an Aura names itself, 477 supported
             -- own-line occurrences over 400 cards, the largest carrier
             -- in the family (`ascribesAsSubtype`).
             -- CURSE rides in on the same card's type line ([CR#205.3h]
             -- names it beside Aura), and its ascription cell is the
             -- interesting one: an Aura Curse says "this Aura" and never
             -- "this Curse", which is a fact about what self-reference is
             -- FOR -- an Aura's own rules [CR#303.4] are what the sentence
             -- reaches. Of 42 supported Aura Curses seven self-refer and
             -- all seven write "this Aura"; the five Aura Cartouches and
             -- five Aura Runes self-refer to a card and write it too.
             -- "This Curse" is zero lines, as is every second word.
             | Aura | Curse

public export
sameSub : Subtype -> Subtype -> Bool
sameSub Goblin Goblin = True
sameSub Goblin _ = False
sameSub Equipment Equipment = True
sameSub Equipment _ = False
sameSub Avatar Avatar = True
sameSub Avatar _ = False
sameSub Insect Insect = True
sameSub Insect _ = False
sameSub Elder Elder = True
sameSub Elder _ = False
sameSub Dinosaur Dinosaur = True
sameSub Dinosaur _ = False
sameSub Human Human = True
sameSub Human _ = False
sameSub Advisor Advisor = True
sameSub Advisor _ = False
sameSub Wizard Wizard = True
sameSub Wizard _ = False
sameSub Shaman Shaman = True
sameSub Shaman _ = False
sameSub Vedalken Vedalken = True
sameSub Vedalken _ = False
sameSub Artificer Artificer = True
sameSub Artificer _ = False
sameSub Zombie Zombie = True
sameSub Zombie _ = False
sameSub Army Army = True
sameSub Army _ = False
sameSub Soldier Soldier = True
sameSub Soldier _ = False
sameSub Knight Knight = True
sameSub Knight _ = False
sameSub Myr Myr = True
sameSub Myr _ = False
sameSub Thopter Thopter = True
sameSub Thopter _ = False
sameSub Construct Construct = True
sameSub Construct _ = False
sameSub Fractal Fractal = True
sameSub Fractal _ = False
sameSub Coward Coward = True
sameSub Coward _ = False
sameSub Demon Demon = True
sameSub Demon _ = False
sameSub Angel Angel = True
sameSub Angel _ = False
sameSub Elemental Elemental = True
sameSub Elemental _ = False
sameSub Plant Plant = True
sameSub Plant _ = False
sameSub Dragon Dragon = True
sameSub Dragon _ = False
sameSub Plains Plains = True
sameSub Plains _ = False
sameSub Island Island = True
sameSub Island _ = False
sameSub Swamp Swamp = True
sameSub Swamp _ = False
sameSub Mountain Mountain = True
sameSub Mountain _ = False
sameSub Forest Forest = True
sameSub Forest _ = False
sameSub Aura Aura = True
sameSub Aura _ = False
sameSub Curse Curse = True
sameSub Curse _ = False

||| Which card type a subtype's set belongs to — [CR#205.1a] names the
||| sets themselves. [CR#205.3m] supplies the creature rows;
||| [CR#205.3i,305.6] supply Plains, Island, Swamp, Mountain, and Forest
||| as land rows. Full rows: a subtype from another set (an artifact type
||| like Equipment, an enchantment type like Aura) must declare its card
||| type before it can be written.
public export
subtypeType : Subtype -> CardType
subtypeType Zombie = Creature
subtypeType Army = Creature
subtypeType Soldier = Creature
subtypeType Knight = Creature
subtypeType Myr = Creature
subtypeType Thopter = Creature
subtypeType Construct = Creature
subtypeType Fractal = Creature
subtypeType Coward = Creature
subtypeType Demon = Creature
subtypeType Angel = Creature
subtypeType Elemental = Creature
subtypeType Plant = Creature
subtypeType Dragon = Creature
subtypeType Plains = Land
subtypeType Island = Land
subtypeType Swamp = Land
subtypeType Mountain = Land
subtypeType Forest = Land
subtypeType Goblin = Creature
-- the first ARTIFACT type ([CR#205.3g], which sends the word on to
-- [CR#301.5]'s equipment rules): the catalog's per-set membership is
-- exactly what a for-each domain over "each Equipment you control" needs
-- said before it can be written.
subtypeType Equipment = Artifact
subtypeType Avatar = Creature
subtypeType Insect = Creature
subtypeType Elder = Creature
subtypeType Dinosaur = Creature
subtypeType Human = Creature
subtypeType Advisor = Creature
subtypeType Wizard = Creature
subtypeType Shaman = Creature
subtypeType Vedalken = Creature
subtypeType Artificer = Creature
subtypeType Aura = Enchantment
subtypeType Curse = Enchantment

||| Which CARD TYPE word a self-reference may be ascribed (`AsType`).
||| [CR#109.2] reads a description carrying a type word and no zone word
||| onto the BATTLEFIELD, so a word that names no permanent ascribes
||| nothing: "this instant" and "this sorcery" are each zero supported
||| own-line occurrences, against creature 12,195, artifact 1,284, land
||| 1,242 and enchantment 1,098. The rule and the count agree, which is
||| why this is a table and not a free parameter (`badAscribeInstant`).
public export
ascribesAsType : CardType -> Bool
ascribesAsType Creature = True
ascribesAsType Artifact = True
ascribesAsType Land = True
ascribesAsType Enchantment = True
ascribesAsType Instant = False
ascribesAsType Sorcery = False

||| Which SUBTYPE word a self-reference may be ascribed — the same
||| question at the other half of [CR#109.2]'s "card type or subtype", and
||| a separate table because mere membership in `Subtype` is not the
||| licence. Exactly nine words in the whole supported corpus follow
||| "this" as a self-name (Aura 477, Equipment 253, Vehicle 166, Saga 92,
||| Siege 37, Class 27, Spacecraft 20, Case 13, Room 4), and no creature
||| type is among them: a census of every capitalised word after
||| "this"/"This" in own-line text returns those nine and nothing else, so
||| "this Zombie" is zero and stays refused (`badAscribeCreatureType`).
||| Two of the nine have rows in the catalog above; the other seven are
||| measured and unrowed, growing a row when a card benches one (queue).
public export
ascribesAsSubtype : Subtype -> Bool
ascribesAsSubtype Aura = True
ascribesAsSubtype Equipment = True
ascribesAsSubtype Curse = False
ascribesAsSubtype Zombie = False
ascribesAsSubtype Army = False
ascribesAsSubtype Soldier = False
ascribesAsSubtype Thopter = False
ascribesAsSubtype Construct = False
ascribesAsSubtype Fractal = False
ascribesAsSubtype Coward = False
ascribesAsSubtype Demon = False
ascribesAsSubtype Angel = False
ascribesAsSubtype Elemental = False
ascribesAsSubtype Plant = False
ascribesAsSubtype Dragon = False
ascribesAsSubtype Plains = False
ascribesAsSubtype Island = False
ascribesAsSubtype Swamp = False
ascribesAsSubtype Mountain = False
ascribesAsSubtype Forest = False
ascribesAsSubtype Goblin = False
ascribesAsSubtype Avatar = False
ascribesAsSubtype Insect = False
ascribesAsSubtype Elder = False
ascribesAsSubtype Dinosaur = False
ascribesAsSubtype Human = False
ascribesAsSubtype Advisor = False
ascribesAsSubtype Wizard = False
ascribesAsSubtype Shaman = False
ascribesAsSubtype Vedalken = False
ascribesAsSubtype Artificer = False
ascribesAsSubtype Knight = False
ascribesAsSubtype Myr = False

||| The ascription word as one question, which is how [CR#109.2] asks it:
||| one rule over "a card type or subtype", one projection, one gate.
||| Under a subtype the card type is still written, and [CR#205.3c] is
||| what makes that honest rather than redundant — each subtype is
||| correlated to its own card type, so the pair must agree and the
||| binding an ascription mints keeps naming a card type
||| (`badAscribeMismatchedSubtype`).
public export
ascriptionOk : CardType -> Maybe Subtype -> Bool
ascriptionOk t Nothing = ascribesAsType t
ascriptionOk t (Just s) = ascribesAsSubtype s && sameCT (subtypeType s) t

||| Binding-free counter deltas, deliberately separate from `LifeOp`'s
||| discourse-indexed amount operations. A zero component is a valid counter
||| component ("-0/-1", six lines); no nonzero proof belongs to this literal
||| vocabulary. Two rows and not three: [CR#122.1a] knows "+X/+Y" and
||| "-X/-Y" and nothing sets a stat counter to a value, so `LifeOp.SetTo`'s
||| twin would be a phrase no card prints.
namespace Counter
  public export
  -- spelling: (construction-owned literal operation -- BoostCounter owns
  -- the surrounding counter name. Up n contributes "+n", Down n "-n".)
  data Delta : Type where
    Up : Nat -> Delta
    Down : Nat -> Delta

  public export
  sameDelta : Delta -> Delta -> Bool
  sameDelta (Up a) (Up b) = a == b
  sameDelta (Up _) _ = False
  sameDelta (Down a) (Down b) = a == b
  sameDelta (Down _) _ = False

||| Which bare keyword names belong to [CR#122.1b]'s named base
||| keyword-counter family. The rule closes that list at fifteen and it is
||| a FORCING function, not a measurement: a keyword row off [CR#122.1b]'s
||| list must answer False here before anything can put a counter of its
||| name on anything. Parameterized variants remain outside this
||| vocabulary.
|||
||| The table answered True throughout until the grant round, and the
||| three casting-time words are what broke that: [CR#122.1b] names
||| lifelink among its fifteen and names neither convoke, improvise nor
||| storm, so those three answer False — the first Falses this table has
||| had, and the reason the row is spelled out per keyword rather than
||| defaulted.
public export
keywordCounterOk : Keyword -> Bool
keywordCounterOk Haste = True
keywordCounterOk Flying = True
keywordCounterOk Trample = True
keywordCounterOk Vigilance = True
keywordCounterOk Deathtouch = True
keywordCounterOk DoubleStrike = True
keywordCounterOk FirstStrike = True
keywordCounterOk Convoke = False
keywordCounterOk Improvise = False
keywordCounterOk Storm = False
keywordCounterOk Lifelink = True

||| The eligibility decision is explicit for every Keyword row, so a future
||| keyword must opt in (or out) before it can become a keyword counter.
public export
data KeywordCounterEligible : Keyword -> Type where
  MkKeywordCounterEligible : {auto 0 ok : keywordCounterOk k = True} ->
                            KeywordCounterEligible k

||| WHEN a keyword functions on a SPELL — the table the grant round needed
||| and the one that decides which relation may describe the class. Three
||| answers, each read off the keyword's own rule:
|||
||| `Nothing` is a battlefield word. Haste, flying, trample, vigilance,
||| double strike and first strike are combat and permanent business; a
||| spell on the stack has nothing for them to do, and the corpus grants
||| none of them to one (zero lines apiece).
|||
||| `Just AtCasting` is a word that functions while the spell is BEING
||| cast: convoke "functions while the spell with convoke is on the stack"
||| ([CR#702.51a]) and improvise says the same of itself ([CR#702.126a]),
||| both letting a permanent be tapped for mana in the spell's total cost;
||| storm "is a triggered ability that functions on the stack"
||| ([CR#702.40a]) and copies the spell as it is cast. [CR#113.6d] and
||| [CR#113.6e] are the general warrant — an ability that modifies what a
||| particular object costs to cast, or how it can be cast, functions on
||| the stack — and these three function NOWHERE else, which is why they
||| answer for the battlefield too.
|||
||| `Just AtResolution` is a word that functions when the spell DOES
||| something: lifelink is defined over "a source" ([CR#702.15b]) and its
||| rules "function no matter what zone an object with lifelink deals
||| damage from" ([CR#702.15d]), and deathtouch is likewise a fact about a
||| source dealing damage ([CR#702.2b]). Those two are the only rows that
||| work in BOTH places, which is why a battlefield grant of them is
||| ordinary English and a battlefield grant of convoke is not.
public export
data StackRegime = AtCasting | AtResolution

public export
sameRegime : StackRegime -> StackRegime -> Bool
sameRegime AtCasting AtCasting = True
sameRegime AtCasting _ = False
sameRegime AtResolution AtResolution = True
sameRegime AtResolution _ = False

public export
keywordStackRegime : Keyword -> Maybe StackRegime
keywordStackRegime Haste = Nothing
keywordStackRegime Flying = Nothing
keywordStackRegime Trample = Nothing
keywordStackRegime Vigilance = Nothing
keywordStackRegime DoubleStrike = Nothing
keywordStackRegime FirstStrike = Nothing
keywordStackRegime Convoke = Just AtCasting
keywordStackRegime Improvise = Just AtCasting
keywordStackRegime Storm = Just AtCasting
keywordStackRegime Deathtouch = Just AtResolution
keywordStackRegime Lifelink = Just AtResolution

||| SUPERTYPES ([CR#205.4a] closes the list at five: basic, legendary,
||| ongoing, snow, world). Three rows here, whittled to what a witness
||| needs exactly as `CardType` is whittled to six of fifteen — a
||| supertype is an ordinary catalog word, not a rules-fixed structure
||| like `Color`, so an unwitnessed row would be a phrase nothing here
||| writes.
|||
||| The field this rides on is the CARD's and not `TypeLine`'s, and
||| [CR#205.4b] is the reason it can be: "an object's supertype is
||| independent of its card type and subtype", so the three lists are
||| three facts printed on one line rather than one bundle. `TypeLine`'s
||| two readers — the token's defined characteristics and the type
||| ADDITION clause — write no supertype between them, and the third
||| reader, the printed card, is the one that has them. `Legendary`,
||| `Basic` and `Snow` have witnesses; World and Ongoing remain unminted.
public export
-- spelling: ["legendary", "basic", "snow"] (the supertype words, printed
-- before card types; a line may carry more than one: "Snow-Covered Forest"
-- is Basic Snow Land — Forest. Spelled only through Card.)
data Supertype = Legendary | Basic | Snow

public export
sameSupertype : Supertype -> Supertype -> Bool
sameSupertype Legendary Legendary = True
sameSupertype Legendary _ = False
sameSupertype Basic Basic = True
sameSupertype Basic _ = False
sameSupertype Snow Snow = True
sameSupertype Snow _ = False

||| The DESIGNATIONS a player can have — [CR#725.1] and [CR#726.1] use the
||| same six words for both ("the monarch/the initiative is a designation a
||| player can have"), which is why they are one vocabulary and not two
||| rows in two places.
|||
||| Core's designation taxonomy is COMPLETE and wired
||| (`deckmaste_core/src/designation.rs`: `DesignationScope{Object, Player,
||| Game}` over `DesignationDef{Stored, Derived, DerivedIf}` with shape,
||| uniqueness and persistence metadata); the OLD module has an open
||| `MkDesignation Scope String` carrying player and object scopes only;
||| and this workbench, until now, had NO designation machinery of any
||| kind. So these rows are the first Idris consumers of the axis at every
||| scope, not merely at the game scope the plugin emitter records as
||| absent ("Game-scoped designations have no Idris `Scope` and are simply
||| absent", `deckmaste_plugin/src/idris_emit.rs`).
|||
||| Three designations the old module carries are NOT minted, and one
||| argument covers all three: the city's blessing (28 lines, 15 of them
||| checks), monstrous (64 / 9) and renowned (12 / 3) are each conferred
||| by a KEYWORD -- ascend, monstrosity, renown -- so their BECOMING sits
||| behind the keyword boundary while only their checks are ordinary card
||| text. Minting the check alone would give this grammar a designation
||| nothing in it can confer, which is a worse state than not having it.
||| They wait for the keyword transition together.
public export
-- spelling: ["the monarch", "the initiative"] (row order:
-- Monarch/TheInitiative; the definite noun phrase naming the role. The
-- BECOMING verb differs per value and the effect row carries it -- see
-- TakesDesignation)
data PlayerDesignation = Monarch | TheInitiative

public export
samePlayerDesignation : PlayerDesignation -> PlayerDesignation -> Bool
samePlayerDesignation Monarch Monarch = True
samePlayerDesignation Monarch _ = False
samePlayerDesignation TheInitiative TheInitiative = True
samePlayerDesignation TheInitiative _ = False

||| The designations an OBJECT can have, and the two the corpus writes as
||| ordinary card text. `Goaded` is [CR#701.15b]'s -- "neither an ability
||| nor part of the permanent's copiable values", which is what makes it a
||| designation and not a keyword despite arriving by a keyword action.
||| `RingBearer` is [CR#701.54a]'s, and it is the reason the READ is not a
||| bare presence test: see YourRingBearer.
public export
-- spelling: ["your Ring-bearer", "goaded"] (row order: RingBearer/Goaded;
-- the Ring-bearer is written as a possessed noun and never bare, goaded as
-- a prenominal participle. Spelled only through the predicate rows)
data ObjectDesignation = RingBearer | Goaded

||| The GAME's own designations ([CR#731.1] — "day and night are
||| designations that the game itself can have. The game starts with
||| neither"). A third scope, and the one core carries that no Idris
||| module has ever had a consumer for.
|||
||| It is a two-value enum and NOT a third `Kind`. The temptation was to
||| give the game a sort beside `Object` and `Player` so a designation
||| read could be kind-indexed throughout; the corpus does not pay for it.
||| Four check lines and eleven transition lines buy two rows, not a
||| universe: nothing else in the grammar ever needs to describe, target,
||| count or quantify over the game, and a `Kind` row would have obliged
||| every kind-keyed table in the file to answer for it.
public export
-- spelling: ["day", "night"] (row order: Day/Night; the bare word after
-- "it becomes" or "it's". Spelled only through BecomesTime and ItIsNow)
data TimeOfDay = Day | Night

public export
sameTimeOfDay : TimeOfDay -> TimeOfDay -> Bool
sameTimeOfDay Day Day = True
sameTimeOfDay Day _ = False
sameTimeOfDay Night Night = True
sameTimeOfDay Night _ = False

||| WHICH time-of-day the CHECK writes -- finding 246's idiom at a third
||| site, and the sharpest asymmetry the round measured: "if it's night"
||| and its siblings are four lines, and "if it's day" is written ZERO
||| times. The transition writes both directions freely, so this is a fact
||| about the check and not about the designation (`badItIsDay`).
public export
timeCheckOk : TimeOfDay -> Bool
timeCheckOk Day = False
timeCheckOk Night = True

public export
data TimeChecked : TimeOfDay -> Type where
  MkTimeChecked : {auto 0 ok : timeCheckOk t = True} -> TimeChecked t

||| The ATTACHMENT PARTICIPLE — the word a permanent uses to refer to
||| whatever it is attached to. Three words, and the rules define all
||| three: [CR#303.4b] ("the object or player an Aura is attached to is
||| called enchanted"), [CR#301.5a] ("the creature an Equipment is
||| attached to is called the 'equipped creature'"), and [CR#301.6], which
||| applies the Equipment rules to Fortifications in relation to lands
||| just as they apply to Equipment in relation to creatures".
|||
||| The word is DECOUPLED from the speaker's own type, and that is what
||| lets this land with no Aura-subtype machinery and no attach verb:
||| [CR#303.4m] and [CR#301.5f] both say the reference works "even if the
||| permanent with the ability isn't an Aura"/"isn't an Equipment". So the
||| participle is a pointer a permanent may write, not a fact about what
||| kind of permanent it is, and nothing about attachment as a RELATION is
||| needed to spell it. The corpus confirms the boundary: the paraphrase
||| "the creature this is attached to" is written zero times, so the
||| participle is the only spelling.
|||
||| The gap it closes is COMMON to both prior arts. Core has the
||| reference — `Reference::AttachHostOf`, "the permanent that attachment
||| R is attached to" (`deckmaste_core/src/reference.rs`) — and the OLD
||| module has the same primitive with `refIntro (AttachHostOf r) =
||| refIntro r`, a passthrough that announces nothing (`Semantics.idr`).
||| NEITHER has a NOUN surface. This is the first one in the repo.
public export
-- spelling: ["enchanted", "equipped", "fortified"] (row order:
-- Enchanted/Equipped/Fortified; the participle before its head word.
-- Spelled only through Noun.AttachHost)
data AttachWord = Enchanted | Equipped | Fortified

public export
sameAttachWord : AttachWord -> AttachWord -> Bool
sameAttachWord Enchanted Enchanted = True
sameAttachWord Enchanted _ = False
sameAttachWord Equipped Equipped = True
sameAttachWord Equipped _ = False
sameAttachWord Fortified Fortified = True
sameAttachWord Fortified _ = False

||| WHICH head word each participle takes. Full rows over (participle x
||| head), every cell measured.
|||
||| The grid is lopsided in a way that follows the rules exactly.
||| ENCHANTED spans six heads because [CR#303.4b] lets an Aura attach to
||| an object OR a player: creature 931 lines, land 79, permanent 76,
||| player 54 (the Curse family), artifact 22, enchantment 4. EQUIPPED is
||| creature-only at 605, [CR#301.5a] naming no other host. FORTIFIED is
||| land-only at 3, and those three lines mint the row rather than
||| record it because [CR#301.6] cross-references the whole Equipment block:
||| "fortified land" is the rules' own term with exactly the status
||| "equipped creature" has, so the row is rules-backed vocabulary and
||| not a corpus-thin guess -- finding 85's bar asked for a line of the
||| right SHAPE, and these are that shape.
||| No participle takes a card, a spell or a token head: an attachment
||| hosts a permanent or a player and nothing else.
public export
attachHeadOk : AttachWord -> NounWord -> Bool
attachHeadOk Enchanted (TypeW Creature) = True
attachHeadOk Enchanted (TypeW Artifact) = True
attachHeadOk Enchanted (TypeW Land) = True
attachHeadOk Enchanted (TypeW Enchantment) = True
attachHeadOk Enchanted (TypeW Instant) = False
attachHeadOk Enchanted (TypeW Sorcery) = False
attachHeadOk Enchanted CardW = False
attachHeadOk Enchanted SpellW = False
attachHeadOk Enchanted PlayerW = True
attachHeadOk Enchanted PermanentW = True
attachHeadOk Enchanted TokenW = False
attachHeadOk Equipped (TypeW Creature) = True
attachHeadOk Equipped (TypeW Artifact) = False
attachHeadOk Equipped (TypeW Land) = False
attachHeadOk Equipped (TypeW Enchantment) = False
attachHeadOk Equipped (TypeW Instant) = False
attachHeadOk Equipped (TypeW Sorcery) = False
attachHeadOk Equipped CardW = False
attachHeadOk Equipped SpellW = False
attachHeadOk Equipped PlayerW = False
attachHeadOk Equipped PermanentW = False
attachHeadOk Equipped TokenW = False
attachHeadOk Fortified (TypeW Creature) = False
attachHeadOk Fortified (TypeW Artifact) = False
attachHeadOk Fortified (TypeW Land) = True
attachHeadOk Fortified (TypeW Enchantment) = False
attachHeadOk Fortified (TypeW Instant) = False
attachHeadOk Fortified (TypeW Sorcery) = False
attachHeadOk Fortified CardW = False
attachHeadOk Fortified SpellW = False
attachHeadOk Fortified PlayerW = False
attachHeadOk Fortified PermanentW = False
attachHeadOk Fortified TokenW = False

public export
data AttachHeadOk : AttachWord -> NounWord -> Type where
  MkAttachHeadOk : {auto 0 ok : attachHeadOk w h = True} -> AttachHeadOk w h

||| The zone a host's head word evidences. A permanent head places its
||| referent on the battlefield -- an Aura or Equipment is attached to a
||| permanent there ([CR#303.4b,301.5a]) -- and a PLAYER head places
||| nothing, players having no zone.
public export
attachHostZone : NounWord -> Maybe Zone
attachHostZone PlayerW = Nothing
attachHostZone (TypeW _) = Just Battlefield
attachHostZone CardW = Just Battlefield
attachHostZone SpellW = Just Battlefield
attachHostZone PermanentW = Just Battlefield
attachHostZone TokenW = Just Battlefield

||| The card type a host's head word projects, where it writes one.
public export
attachHostTy : NounWord -> Maybe CardType
attachHostTy (TypeW t) = Just t
attachHostTy CardW = Nothing
attachHostTy SpellW = Nothing
attachHostTy PlayerW = Nothing
attachHostTy PermanentW = Nothing
attachHostTy TokenW = Nothing

||| The two GAME OUTCOMES an effect may state. [CR#104.2b] and
||| [CR#104.3e] are one sentence each and say the same thing twice — "an
||| effect may state that a player wins the game", "…that a player loses
||| the game" — which is why this is one indexed vocabulary and not two
||| sibling rows (finding 246's idiom).
|||
||| BOTH prior arts already have it and this workbench had NOTHING: core's
||| `Action::{WinGame, LoseGame}(Reference)`
||| (`deckmaste_core/src/action.rs`) and the LEGACY grammar's
||| `Outcome b = WinGame (Reference b APlayer) | LoseGame …` wrapped by
||| `Conclude : Outcome b -> OneShotEffect b` (`Semantics.idr`), witnessed
||| there by Platinum Angel. So this round is a port-with-measurement
||| against two agreeing prior arts rather than an extension of anything
||| here.
|||
||| NAMED APART from `OutcomeSort` (the deed telescope's DamageDealt /
||| LifeGained / LifeLost) and from `Kind`'s own `Outcome` constructor,
||| both of which are unrelated and neither of which this vocabulary
||| touches — the `LifeGain`/`LifeLoss` rename's precedent.
public export
-- spelling: ["wins the game", "loses the game"] (row order:
-- WinGame/LoseGame; the finite verb phrase after its patient, inflected
-- for the subject -- "you win the game", "that player loses the game".
-- Spelled only through Effect.Concludes)
data OutcomeVerb = WinGame | LoseGame

public export
sameOutcomeVerb : OutcomeVerb -> OutcomeVerb -> Bool
sameOutcomeVerb WinGame WinGame = True
sameOutcomeVerb WinGame _ = False
sameOutcomeVerb LoseGame LoseGame = True
sameOutcomeVerb LoseGame _ = False

||| WHICH outcome a static gate suppresses — the name both prior arts use
||| ([CR#101.2]'s precedence over [CR#104]'s outcomes), kept verbatim for
||| parity.
public export
-- spelling: ["can't lose the game", "can't win the game"] (row order:
-- CantLose/CantWin; the finite phrase after its subject. Spelled only
-- through StaticEffect.OutcomeGate)
data OutcomeGateKind = CantLose | CantWin

||| WHICH of a creature's two numbers a characteristic-defining ability
||| defines. [CR#208.2a] writes the choice into its own sentence — such an
||| ability is worded "[This creature's] [power or toughness] is equal
||| to . . ." or "[This creature's] power and toughness are each equal
||| to . . ." — so this is a slot the rule declares, and the corpus fills
||| all three cells: the pair is 142 supported lines, power alone 66
||| (Adeline, Resplendent Cathar; Crackling Drake; Bronze Guardian) and
||| toughness alone 9 (Traproot Kami, People of the Woods, Wintermoor
||| Commander).
|||
||| ONE ROW INDEXED rather than three constructors, the idiom the counter
||| kinds, the face verbs and the designations all took: the cells differ
||| in the word English writes and in NO table, and the printed box is the
||| only thing that reads the difference — a defined slot prints the star
||| and an undefined one prints its number, which is where "*/4" comes
||| from (`cardPtOk`).
public export
-- spelling: ["power", "toughness", "power and toughness"] (row order:
-- PowerAlone/ToughnessAlone/BothEach; the possessed subject phrase leads
-- and the copula agrees with the slot -- "is equal to" for a single
-- number and "are each equal to" for the pair, which is where the word
-- "each" comes from. Spelled only through StaticEffect.DefinesPt)
data DefinedSlots = PowerAlone | ToughnessAlone | BothEach

||| Whether the definition reaches a given number, asked once per slot by
||| the container that prints them.
public export
definesPower : DefinedSlots -> Bool
definesPower PowerAlone = True
definesPower ToughnessAlone = False
definesPower BothEach = True

public export
definesToughness : DefinedSlots -> Bool
definesToughness PowerAlone = False
definesToughness ToughnessAlone = True
definesToughness BothEach = True

||| Counter kinds ([CR#122.1] — "a marker placed on an object or player
||| that modifies its characteristics and/or interacts with a rule,
||| ability, or effect"). Core spells the kind as an open bare-identifier
||| reference into a plugin registry (`CounterRef`,
||| `deckmaste_core/src/counter.rs`); this vocabulary answers that
||| openness with TWO PRODUCTS and a witnessed flat tail, never by
||| transcribing the registry — a counter NAME sighted in a corpus line is
||| data, and data that no construction here writes is the ledger's.
|||
||| `BoostCounter` is the stat product, one constructor for [CR#122.1a]'s
||| "+X/+Y" and "-X/-Y" over the twelve printed pairs, with the everyday
||| two spelled by macros over it (`plusOnePlusOne`, `minusOneMinusOne`).
||| `KeywordCounter` is the keyword product, closed by [CR#122.1b]'s
||| fifteen names through `keywordCounterOk`. A FLAT named kind earns its
||| row one at a time, and the test has TWO halves chapter sixteen ran
||| together: the corpus must write the kind as a one-shot put or remove,
||| and this grammar must be able to write that line. The first half is
||| much weaker than finding 85 supposed. Its refusal said charge, oil,
||| fade, loyalty and shield "are costs, upkeep triggers and enters-with
||| riders, other axes", and the measurement says otherwise for two of
||| them: charge writes 88 one-shot puts and 64 one-shot removes as a
||| card's own line and none as reminder text, oil 28 and 29 (chapter
||| sixty-nine). So it is the SECOND half that has been doing the work,
||| and the catalog grows exactly as far as the bench does — a kind enters
||| when a line writing it lands. Oil, quest, level, spore, storage, ki,
||| verse and depletion all clear the first half and wait on the second
||| (ledger). Loyalty and shield clear neither, being [CR#122.1e]'s and
||| [CR#122.1c]'s own mechanics, and energy routes with the symbol family,
||| which is finding 85's Ticket refusal.
||| Engine `Counter.confers`, the layer system and the state-based actions
||| stay RON-side throughout.
public export
-- spelling: (BoostCounter delegates its two operations to Counter.Delta and
-- writes them as one word, "+1/+1" or "-0/-1"; KeywordCounter writes the
-- keyword's own lowercase word; every flat row writes its own lowercase
-- word. Each stands between the count and the noun "counter(s)", never
-- alone.)
data CounterKind : Type where
  BoostCounter : Counter.Delta -> Counter.Delta -> CounterKind
  Stun : CounterKind
  Time : CounterKind
  KeywordCounter : (k : Keyword) -> {auto 0 ok : KeywordCounterEligible k} -> CounterKind
  -- The FLAT TAIL, each row paying a bench line and each meaning exactly
  -- what its own card says it means.
  -- No CR rule enumerates the kinds: [CR#122.1] defines what a counter IS
  -- and [CR#122.1c..122.1j] are one-kind special cases, so a name outside
  -- those rules is card-supplied and the abilities that read it supply the
  -- meaning -- finding 200's posture, and finding 398's pattern again.
  -- CHARGE is the tail's giant and the one finding 85 refused on a premise
  -- that does not hold (see above): Coretapper writes both of its lines as
  -- bare one-shot puts on a targeted artifact.
  Charge : CounterKind
  -- OMEN, three cards and one line each, is the kind that finally pays
  -- `GameDrawn`'s recorded bench gap: Celestial Convergence counts one
  -- down each upkeep and reads its own holding for zero.
  Omen : CounterKind
  -- SPITE is the row the previous chapter refused and this one pays: its
  -- single card's tally is a plain one-shot put whose holder is written
  -- with a SUBTYPE word -- "put a spite counter on this Aura" (Curse of
  -- Vengeance) -- and the ascription's subtype half is what the holder was
  -- waiting on. REV, its twin, still does not mint: Chainsaw is the only
  -- card, its holder now writes, and its "Equip {3}" is the numeric
  -- keyword parameter this catalog does not carry, so no line of it lands
  -- and the bench is still what the catalog grows by.
  Spite : CounterKind
  -- INTERVENTION earns its row on Divine Intervention's ordinary upkeep
  -- REMOVE line and not on the agentive trigger beneath it, which is worth
  -- recording because the round was briefed the other way round.
  Intervention : CounterKind
  -- The three PLAYER kinds, [CR#122.1]'s other half read at last: a
  -- counter is "a marker placed on an object OR PLAYER", and until this
  -- round every row in the catalog was an object's. Each earns its row by
  -- finding 85's test unchanged -- a one-shot line of the sort this
  -- grammar writes -- at the player verb rather than the put verb, which
  -- is the only adaptation the test needs and no weakening of it: poison
  -- 25 lines ("Each opponent gets a poison counter", Prologue to
  -- Phyresis, a whole card), rad 20 ("Whenever this creature attacks,
  -- each player gets two rad counters", Screeching Scorchbeast),
  -- experience 15 ("Whenever another creature you control dies, you get
  -- an experience counter"). Poison and rad carry rules of their own
  -- ([CR#122.1f], [CR#122.1i]); experience carries none and is `Time`'s
  -- situation exactly -- the abilities that read it supply the meaning
  -- (finding 200).
  -- TICKET is REFUSED and the reason is not its two lines. Both write
  -- "You get {TK} (a ticket counter)" -- SYMBOL notation with the counter
  -- name in a parenthetical gloss -- which is the energy family's
  -- spelling regime and not this verb's, so it routes with energy to the
  -- symbol round rather than waiting on a count.
  Poison : CounterKind
  Rad : CounterKind
  Experience : CounterKind

||| WHAT a counter kind can sit on — the catalog's scope table, and the
||| axis [CR#122.1] has carried since the beginning without this grammar
||| reading it: "a marker placed on an object or player". Full rows, so a
||| new kind must declare which it is before any verb will take it.
|||
||| The table is a MEASUREMENT and the corpus is unusually clean about it.
||| Objects never "get" a counter (zero lines) and players never receive
||| "put" (zero lines); the two verbs and the two scopes line up one to
||| one across the whole family, which is why the scope can gate the verbs
||| rather than each verb re-describing its own recipients
||| (`badPutPoisonOnCreature`, `badGetsBoostCounter`). [CR#702.90b] is the
||| one place the rules and the oracle disagree about the WORD -- infect
||| "causes that source's controller to GIVE the player that many poison
||| counters" where every printed line says "gets" -- and the grammar
||| follows the oracle, this being a record of what cards say.
|||
||| Answering in `Kind` rather than in a scope enum of its own is what
||| makes the count read cheap: one row over `{k : Kind}` can demand
||| `counterScope kind = k` and get the holder agreement for free.
public export
counterScope : CounterKind -> Kind
counterScope (BoostCounter _ _) = Object
counterScope Stun = Object
counterScope Time = Object
counterScope (KeywordCounter _) = Object
counterScope Charge = Object
counterScope Omen = Object
counterScope Spite = Object
counterScope Intervention = Object
counterScope Poison = Player
counterScope Rad = Player
counterScope Experience = Player

public export
sameCounter : CounterKind -> CounterKind -> Bool
sameCounter (BoostCounter ap at) (BoostCounter bp bt) =
  Counter.sameDelta ap bp && Counter.sameDelta at bt
sameCounter (BoostCounter _ _) _ = False
sameCounter Stun Stun = True
sameCounter Stun _ = False
sameCounter Time Time = True
sameCounter Time _ = False
sameCounter (KeywordCounter a) (KeywordCounter b) = sameKeyword a b
sameCounter (KeywordCounter _) _ = False
sameCounter Charge Charge = True
sameCounter Charge _ = False
sameCounter Omen Omen = True
sameCounter Omen _ = False
sameCounter Spite Spite = True
sameCounter Spite _ = False
sameCounter Intervention Intervention = True
sameCounter Intervention _ = False
sameCounter Poison Poison = True
sameCounter Poison _ = False
sameCounter Rad Rad = True
sameCounter Rad _ = False
sameCounter Experience Experience = True
sameCounter Experience _ = False

||| The TYPE LINE a token defines and a type-addition clause adds
||| ([CR#205.1] — the line carries the card types and the subtypes). ONE
||| record for both readers, because English writes one phrase in one
||| order for both: "0/0 black Zombie Army creature token" and "becomes a
||| Spirit artifact creature in addition to its other types" put the
||| subtypes before the types alike. Core keeps the halves apart at both
||| sites (`Token`'s `types`/`subtypes`, `Modification`'s
||| `CardTypes`/`Subtypes`) and this record is those two lists under one
||| name. Supertypes are a third list neither reader witnesses (ledger).
public export
record TypeLine where
  constructor MkTypeLine
  subs : List Subtype
  tys : List CardType

public export
lineHasType : CardType -> List CardType -> Bool
lineHasType t [] = False
lineHasType t (u :: us) = sameCT t u || lineHasType t us

||| A type line says SOMETHING — the demand a type-addition clause makes
||| ("becomes a … in addition to its other types" names at least one
||| word; `badBecomesNothing`). A token makes the stronger demand
||| (`tokenTyped`).
public export
lineNonEmpty : TypeLine -> Bool
lineNonEmpty (MkTypeLine [] []) = False
lineNonEmpty (MkTypeLine _ _) = True

-- `TokenChars`, its four gates and `tokenHeadTy` live in the mutual
-- block below ("The token bundle"): a token's power and toughness are
-- AMOUNTS as of the round that made them computable, and `Amount` is
-- declared there. What stays here is bs-free and shared with the
-- type-addition clause: the type line, its two checks, and the color
-- list's.

||| Every subtype in a line sits on a card type the same line names — the
||| [CR#205.1a] set-membership fact as a check on the phrase: "Zombie Army
||| creature token" writes the creature type its two subtypes belong to,
||| and a "Zombie artifact token" would name a subtype the object's types
||| cannot carry (`badZombieArtifactToken`).
public export
subsFitLine : List Subtype -> List CardType -> Bool
subsFitLine [] tys = True
subsFitLine (s :: ss) tys = lineHasType (subtypeType s) tys && subsFitLine ss tys



||| Where a card type stands in a type line — a rank rather than a
||| comparison table, because the corpus fixes a TOTAL order over these
||| four words and a rank is that order written once. Measured pairwise
||| over supported oracle: "artifact creature" against no "creature
||| artifact", "artifact land", "land creature" (Dryad Arbor's token),
||| "enchantment creature", and the one line that places enchantment ahead
||| of artifact. The Enchantment/Land pair is written neither way and
||| rides on transitivity, which is what a rank buys and a pair table
||| would have had to guess at. The one counterexample is the Licid
||| template's "becomes a creature enchantment … instead of a creature"
||| (Flanking Licid), a pre-standardization wording.
public export
typeRank : CardType -> Nat
typeRank Enchantment = 0
typeRank Artifact = 1
typeRank Land = 2
typeRank Creature = 3
-- The two SPELL types take the last ranks and the ranks are
-- UNWITNESSED, which the comment says rather than the table hiding it:
-- no printed type line and no corpus line writes either beside another
-- card type here, so the order is a convention the rank has to pick and
-- nothing measures. ([CR#205.1a] does keep them under a type-setting
-- effect — "an object with either the instant or sorcery card type
-- retains that type" — which is the one place a mixed line could arise,
-- and it is an effect this grammar has no construction for.)
typeRank Instant = 4
typeRank Sorcery = 5

public export
ltNat : Nat -> Nat -> Bool
ltNat a b = leNat (S a) b

||| Strictly ascending by rank — the ORDER and the duplicate-freeness at
||| once, a repeated word being the one thing a strict order cannot
||| admit. [CR#111.3] makes a token's stated characteristics its text, so
||| "creature artifact token" and "white white Soldier" are not two
||| spellings of a bundle but two bundles oracle never writes
||| (`badTokenTypeOrder`, `badTokenDuplicateColor`).
public export
typesOrdered : List CardType -> Bool
typesOrdered [] = True
typesOrdered (t :: []) = True
typesOrdered (t :: u :: ts) = ltNat (typeRank t) (typeRank u) &&
                              typesOrdered (u :: ts)

||| Is this card type a PERMANENT type? [CR#110.4] gives the list —
||| "there are six permanent types: artifact, battle, creature,
||| enchantment, land, and planeswalker" — and names the exclusion in the
||| next breath: "instant and sorcery cards can't enter the battlefield
||| and thus can't be permanents". It stands here, with the type-line
||| vocabulary, because two constructions read it: the card container's
||| class and the placement clause's destination.
public export
permanentType : CardType -> Bool
permanentType Creature = True
permanentType Artifact = True
permanentType Land = True
permanentType Enchantment = True
permanentType Instant = False
permanentType Sorcery = False

||| May a phrase under this projected head type be PLACED on the
||| battlefield? [CR#110.4a] lists the permanent card types and [CR#110.4]
||| rules the other two out in as many words. Asked of a `Maybe`, and the
||| silence PASSES for `zoneFits`' reason — a phrase that writes no type
||| word asserts nothing to contradict, and Oblivion Ring's "return the
||| exiled card to the battlefield" is exactly that phrase.
public export
placeableTy : Maybe CardType -> Bool
placeableTy Nothing = True
placeableTy (Just t) = permanentType t

||| The destination's demand on its patient, per destination zone. Only
||| the battlefield asks: [CR#400.1] lets any object be in any of the
||| other five, and the corpus moves instants and sorceries between them
||| freely ("return target instant or sorcery card … to your hand").
public export
destTypeOk : Maybe CardType -> Zone -> Bool
destTypeOk ty Battlefield = placeableTy ty
destTypeOk ty Graveyard = True
destTypeOk ty Exile = True
destTypeOk ty Hand = True
destTypeOk ty Library = True
destTypeOk ty Stack = True

||| The placement's type demand as a witness, so a pin says which
||| question refused.
public export
data Placeable : Maybe CardType -> Zone -> Type where
  MkPlaceable : {auto 0 ok : destTypeOk ty z = True} -> Placeable ty z

||| [CR#110.5]'s status axes, closed by rule: four status categories, each
||| with exactly two values, and every permanent has one value in each.
||| The CATEGORY is the type's index, so "one value per category" is a
||| fact of the representation. Status is not a characteristic
||| ([CR#110.5a]) and persists until changed ([CR#110.5c]); only a
||| permanent on the battlefield has any ([CR#110.5d]) — which is why the
||| description predicate seeds the battlefield and no zone word may argue
||| with it. The entry defaults ([CR#110.5b]) are the entry riders' and
||| the engine's fact, not a projection of any phrase here.
public export
data StatusCat = TapC | FlipC | FaceC | PhaseC

public export
-- spelling: (the value's own word, prenominal or predicative: "tapped",
-- "untapped", "flipped", "unflipped", "face up", "face down", "phased in",
-- "phased out"; the face pair hyphenates prenominally -- "face-down
-- creature". Which values the DESCRIPTION surface writes at all is
-- statusWordOk's answer; the paired values are each their own word, so
-- none is spelled as the other's negation)
data StatusVal : StatusCat -> Type where
  Tapped    : StatusVal TapC
  Untapped  : StatusVal TapC
  Flipped   : StatusVal FlipC
  Unflipped : StatusVal FlipC
  FaceUp    : StatusVal FaceC
  FaceDown  : StatusVal FaceC
  PhasedIn  : StatusVal PhaseC
  PhasedOut : StatusVal PhaseC

public export
sameStatusVal : {0 c, c' : StatusCat} -> StatusVal c -> StatusVal c' -> Bool
sameStatusVal Tapped Tapped = True
sameStatusVal Tapped _ = False
sameStatusVal Untapped Untapped = True
sameStatusVal Untapped _ = False
sameStatusVal Flipped Flipped = True
sameStatusVal Flipped _ = False
sameStatusVal Unflipped Unflipped = True
sameStatusVal Unflipped _ = False
sameStatusVal FaceUp FaceUp = True
sameStatusVal FaceUp _ = False
sameStatusVal FaceDown FaceDown = True
sameStatusVal FaceDown _ = False
sameStatusVal PhasedIn PhasedIn = True
sameStatusVal PhasedIn _ = False
sameStatusVal PhasedOut PhasedOut = True
sameStatusVal PhasedOut _ = False

||| Same category, opposite value — the pair [CR#110.5] forbids one
||| permanent from holding at once. Values of DIFFERENT categories stack:
||| a morph turned sideways is tapped AND face down, and the scan below
||| must let it be.
public export
statusClash : {0 c, c' : StatusCat} -> StatusVal c -> StatusVal c' -> Bool
statusClash Tapped Untapped = True
statusClash Tapped _ = False
statusClash Untapped Tapped = True
statusClash Untapped _ = False
statusClash Flipped Unflipped = True
statusClash Flipped _ = False
statusClash Unflipped Flipped = True
statusClash Unflipped _ = False
statusClash FaceUp FaceDown = True
statusClash FaceUp _ = False
statusClash FaceDown FaceUp = True
statusClash FaceDown _ = False
statusClash PhasedIn PhasedOut = True
statusClash PhasedIn _ = False
statusClash PhasedOut PhasedIn = True
statusClash PhasedOut _ = False

||| Which status values the ordinary DESCRIPTION surface writes. Full
||| rows, each a measurement: "tapped" and "untapped" are written on
||| creature, artifact, land, permanent and token heads, "face-down"
||| likewise, and "face-up" once prenominally. "Unflipped" and
||| "phased-in" are written ZERO times anywhere, "flipped" occurs once in
||| the whole corpus, and participial "phased out" occurs in frames this
||| round's recon did not classify — so those four cells refuse, and the
||| single flipped line is the ledger's to reclaim with a classified
||| frame, not this table's to guess at.
public export
statusWordOk : {0 c : StatusCat} -> StatusVal c -> Bool
statusWordOk Tapped = True
statusWordOk Untapped = True
statusWordOk Flipped = False
statusWordOk Unflipped = False
statusWordOk FaceUp = True
statusWordOk FaceDown = True
statusWordOk PhasedIn = False
statusWordOk PhasedOut = False

public export
data StatusWord : StatusVal c -> Type where
  MkStatusWord : {auto 0 ok : statusWordOk v = True} -> StatusWord v

||| Which status values the becomes-status EVENT spells — a second total
||| reader of [CR#110.5]'s value product, deliberately NOT derived from
||| `statusWordOk` beside it: the two tables disagree on the face pair,
||| which is a real description ("face-down creature") and an unwritten
||| event ("becomes face down" is written zero times — the face, flip,
||| and phasing transitions have their own verb families, ledgered).
||| Only the tap pair is written as a "becomes" transition, the frame
||| [CR#603.2e] gives its trigger reading. The Unflipped cell is
||| additionally rules-backed rather than merely unwritten: flipping is
||| one-way, so no becomes-unflipped transition can occur at all
||| ([CR#710.4]).
public export
statusEventOk : {0 c : StatusCat} -> StatusVal c -> Bool
statusEventOk Tapped = True
statusEventOk Untapped = True
statusEventOk Flipped = False
statusEventOk Unflipped = False
statusEventOk FaceUp = False
statusEventOk FaceDown = False
statusEventOk PhasedIn = False
statusEventOk PhasedOut = False

public export
data StatusEventVal : StatusVal c -> Type where
  MkStatusEventVal : {auto 0 ok : statusEventOk v = True} -> StatusEventVal v

||| Which FACE value the turning EVENT spells — the third total reader of
||| [CR#110.5]'s value product, and the narrowest, its domain being the
||| face pair alone. The index is what closes it: a table over
||| `StatusVal FaceC` has exactly two rows and no other value can be
||| written at all, so this asks only the question `statusEventOk` left
||| open when it refused both face cells to the "becomes" frame.
|||
||| Measured, and the split is stark: 113 supported lines write a trigger
||| header over "is turned face up" ("When this creature is turned face
||| up, …" is the morph mass, "Whenever a permanent you control is turned
||| face up, …" the description-subject family), and ZERO write one over
||| "is turned face down". The zero is queried and refused here rather
||| than left to a missing constructor, which is finding 246's discipline:
||| a value the corpus declines is a measurement the grammar states
||| (`badTurnedFaceDownEvent`). It is attestation only and no rule forbids
||| the transition — [CR#708.2a] turns face-up permanents face down all
||| day — which is exactly how it differs from the flip pair, whose
||| unflipped cell [CR#710.4] makes impossible.
public export
faceEventOk : StatusVal FaceC -> Bool
faceEventOk FaceUp = True
faceEventOk FaceDown = False

public export
data FaceEventVal : StatusVal FaceC -> Type where
  MkFaceEventVal : {auto 0 ok : faceEventOk v = True} -> FaceEventVal v

||| Colors are duplicate-free but NOT ordered, and the corpus is why:
||| Additive Evolution writes "a 0/0 green and blue Fractal creature
||| token", which the mana order would have spelled the other way round.
||| So the demand is only that no color is written twice ([CR#105.2] — a
||| color is a property an object has or lacks and never counts).
public export
colorMember : Color -> List Color -> Bool
colorMember c [] = False
colorMember c (d :: ds) = sameColor c d || colorMember c ds

public export
colorsDistinct : List Color -> Bool
colorsDistinct [] = True
colorsDistinct (c :: cs) = not (colorMember c cs) && colorsDistinct cs


||| The type-addition clause's own subtype check, which is the token's
||| with one more place to look: a subtype the clause adds may sit on a
||| card type the SAME clause adds ("becomes a Spirit artifact creature")
||| or on one the subject already has (amass's "it becomes a Zombie",
||| [CR#701.47a], said of an Army creature). A subject that projects no
||| head type answers for nothing, so only the added types can carry the
||| subtype there — the honest reading of silence the disjunctive head
||| taught (finding 50).
public export
addedFits : Maybe CardType -> TypeLine -> Bool
addedFits subj (MkTypeLine [] tys) = True
addedFits subj (MkTypeLine (s :: ss) tys) =
  (lineHasType (subtypeType s) tys ||
   (case subj of
      Nothing => False
      Just t => sameCT (subtypeType s) t)) &&
  addedFits subj (MkTypeLine ss tys)

public export
data AddedFits : Maybe CardType -> TypeLine -> Type where
  MkAddedFits : {auto 0 ok : addedFits subj tl = True} -> AddedFits subj tl

||| Does a type-addition clause ADD anything? "In addition to its other
||| types" [CR#205.1b] retains what the object had and states what it
||| gains, so a clause that states only what the subject already is states
||| nothing, and the corpus writes no line where the added type is the
||| subject's own head (Tezzeret's adds creature to an ARTIFACT). The
||| subject's head is the only thing the discourse knows about it, so that
||| is what "already" can mean here: a SUBTYPE is never provably redundant
||| (no mention carries its subtypes) and an untyped subject entails
||| nothing, both of which pass — the gate under-refuses in `predEq`'s
||| direction. [CR#701.47a] guards the subtype case in the text instead,
||| with a condition rather than a grammar rule (`badBecomesOwnType`).
public export
anyNewType : Maybe CardType -> List CardType -> Bool
anyNewType subj [] = False
anyNewType subj (t :: ts) = not (tyIs t subj) || anyNewType subj ts

public export
addsSomething : Maybe CardType -> TypeLine -> Bool
addsSomething subj (MkTypeLine [] tys) = anyNewType subj tys
addsSomething subj (MkTypeLine (_ :: _) tys) = True

public export
data AddsSomething : Maybe CardType -> TypeLine -> Type where
  MkAddsSomething : {auto 0 ok : addsSomething subj tl = True} ->
                    AddsSomething subj tl

public export
data LineNonEmpty : TypeLine -> Type where
  MkLineNonEmpty : {auto 0 ok : lineNonEmpty tl = True} -> LineNonEmpty tl

||| The arrival state a created token can be given ([CR#508.4] for the
||| attacking designation; core files both as `EnterRider`s on its
||| `Create` action rather than as replacement effects, and so does this).
||| Two rows, and the corpus fixes their combination as much as their
||| existence: a token is created "tapped" alone (a hundred fifty-nine
||| lines — "target opponent creates a tapped Treasure token") or "tapped
||| and attacking" (sixty-six), and ATTACKING WITHOUT TAPPED
||| is written zero times, so the pair is not a free product
||| (`ridersOk`, `badAttackingUntapped`). The enters-tapped REPLACEMENT
||| ("This land enters tapped") is a different construction and the
||| replacement axis's; these ride the create instruction.
public export
-- spelling: (construction-owned -- the arrival phrase after the noun "token",
-- as a relative clause: "that's tapped", "that's tapped and attacking"
-- (plural "that are tapped and attacking"), or as the prenominal adjective
-- "a tapped Treasure token". Never spelled alone)
data TokenRider = EntersTapped | EntersAttacking

||| The attested rider combinations, enumerated rather than checked
||| pairwise: nothing, tapped, tapped-and-attacking. Written as a closed
||| list of shapes because that is exactly what the corpus offers, and
||| because the ORDER is fixed too — no line writes "attacking and
||| tapped".
public export
ridersOk : List TokenRider -> Bool
ridersOk [] = True
ridersOk [EntersTapped] = True
ridersOk [EntersTapped, EntersAttacking] = True
ridersOk _ = False

public export
data RidersOk : List TokenRider -> Type where
  MkRidersOk : {auto 0 ok : ridersOk rs = True} -> RidersOk rs

||| The head type a created token's mention projects — the LAST card type
||| its line names, which is the head noun of the compound English writes
||| ("artifact creature token" heads on "creature", and [CR#205.1b]'s own
||| example spells the compound the same way round, "artifact land
||| creatures"). Distinct from `seedTyAll`'s first-member rule for a
||| conjunction, and it has to be: a modifier list has no fixed order
||| where a type line does. A line with no types projects nothing —
||| unreachable through `Create` (`TokenTyped`), and written out for
||| totality.
public export
lastType : List CardType -> Maybe CardType
lastType [] = Nothing
lastType [t] = Just t
lastType (_ :: ts) = lastType ts


||| The parts of the turn a duration can name its endpoint by — the turn
||| itself and the steps and phases inside it. Deliberately NOT a mirror
||| of core's `PhaseStep` (`deckmaste_core/src/event.rs`, whose trees
||| enumerate all thirteen): these are the parts an ENDPOINT is written
||| against, and the corpus writes a duration-class adverbial against only
||| these. The draw step and the two main phases carry none (ledger). This
||| is the SHARED vocabulary — the trigger headers name the same parts and
||| a second enum would drift from this one — so a new row is a totality
||| error on `spanUse` and on every table the triggers chapter adds.
public export
-- spelling: ["turn", "upkeep", "end step", "combat", "untap step",
-- "end of combat"] (row order: Turn/Upkeep/EndStep/Combat/UntapStep/
-- EndOfCombat; the bare part word -- DurationEnd's boundary and
-- possession supply everything around it. EndOfCombat is the one row
-- whose word contains its own boundary noun, which is why its beginning
-- is written "at end of combat" and not "at the beginning of …" --
-- see partUse), kind: TODO(reason: adverbial fragment -- not one of
-- Nominal/Sentence/Cost/KeywordLine/Ability)
data TurnPart = Turn | Upkeep | EndStep | Combat | UntapStep | EndOfCombat
              | FirstMain | PostcombatMain | DrawStep

||| Whose part a possessed endpoint names. CLOSED and pronominal: the
||| corpus possesses a duration endpoint with a possessive DETERMINER and
||| nothing else — "your next turn", "that player's next end step" — so
||| this is a two-word vocabulary, not a noun slot. A noun-valued
||| possessor would make `Duration` bindings-indexed (every consumer
||| re-indexed) to buy a form no construction here writes; the one line
||| that needs one is a base-type SETTING clause, a layer word this
||| grammar has no construction for, so its possessor waits with it
||| (ledger). `ThatPlayers` is the anaphoric row: it presupposes a unique
||| player antecedent the way `They` does, an obligation no current cell
||| opens (`spanUse`), so the demand waits for the cell that needs it.
public export
-- spelling: ["your", "that player's"] (row order: Yours/ThatPlayers; the
-- possessive determiner alone, always followed by "next" -- see
-- DurationEnd), kind: TODO(reason: adverbial fragment)
data Whose = Yours | ThatPlayers

||| WHOSE part a trigger header or an activation window names — and a
||| SEPARATE vocabulary from `Whose` above rather than four rows added to
||| it, on `Whose`'s own docstring: that type is the DURATION endpoint's,
||| "closed and pronominal", possessing an endpoint "with a possessive
||| DETERMINER and nothing else". The header site is not pronominal. It
||| writes quantifiers — "each player's upkeep" 76 headers, "each
||| opponent's upkeep" 33, "each of your postcombat main phases" 7 — and
||| the duration endpoint writes NONE of them: "until end of each player's
||| turn" and every sibling spelling is zero lines. Two readers, two
||| measurements, two vocabularies, which is `statusWordOk` and
||| `statusEventOk`'s arrangement over [CR#110.5]'s values and `Lookback`'s
||| beside `Duration`.
||| The concrete price of sharing was the argument's other half: `spanUse`
||| is full rows over (boundary x part x possession) and four more
||| possessors would have added ninety cells to it, every one `Unattested`,
||| to a table whose whole value is that each cell is a measurement.
|||
||| Core's `WhoseTurn` is the prior art and this is a superset of it:
||| `Your`, `EachPlayers` and `AnOpponents` are core's three
||| (`deckmaste_core/src/event.rs`), and the corpus adds two more at this
||| site — the opponent QUANTIFIER, which core's "an opponent's" does not
||| cover, and the plural-iterated self.
||| Finding 254's derived possessor does NOT apply here and the reason is
||| structural: there the possessive agreed with a restricted NOUN the
||| clause already wrote ("its controller's untap step"), so the spelling
||| could read it off. A trigger header has no such noun — nothing in "At
||| the beginning of each opponent's upkeep" is a phrase the possessor
||| could derive from — so it is a slot.
namespace Owner
  public export
  -- spelling: ["your", "that player's", "each player's", "each
  -- opponent's", "each of your", "an opponent's"] (row order:
  -- Yours/ThatPlayers/EachPlayers/EachOpponents/EachYours/AnOpponents; the
  -- possessive determiner before the part word. EachPlayers has a SECOND
  -- spelling that drops the possessor entirely -- "each upkeep", "each end
  -- step", "each combat" -- which names the same moments because a turn
  -- has exactly one of each ([CR#500.1]) and it is the active player's;
  -- EachYours pluralises the part word after it, "each of your postcombat
  -- main phaseS". Spelled only through BeginningOf and Timing.DuringPart)
  data Owner = Yours | ThatPlayers | EachPlayers | EachOpponents
             | EachYours | AnOpponents

public export
sameOwner : Owner -> Owner -> Bool
sameOwner Yours Yours = True
sameOwner Yours _ = False
sameOwner ThatPlayers ThatPlayers = True
sameOwner ThatPlayers _ = False
sameOwner EachPlayers EachPlayers = True
sameOwner EachPlayers _ = False
sameOwner EachOpponents EachOpponents = True
sameOwner EachOpponents _ = False
sameOwner EachYours EachYours = True
sameOwner EachYours _ = False
sameOwner AnOpponents AnOpponents = True
sameOwner AnOpponents _ = False

||| The endpoint an "until" adverbial names: a boundary of a turn part,
||| optionally possessed. Two axes, because the corpus writes both
||| independently — "until end of turn" against "until your next turn" is
||| the SAME part at opposite boundaries, and each boundary takes
||| possession or not.
|||
||| "Next" is DERIVABLE, not a parameter: a possessed endpoint is always
||| the next one (there is no "your previous upkeep" to distinguish it
||| from), and a bare endpoint is always the current turn's. The article
||| goes with the possession the same way, and the possessive can even
||| land outside the part it possesses — Brazen Cannonade writes "until
||| end of combat on your next turn", not a possessed combat. All of that
||| is construction-assigned surface for the spelling layer, which is why
||| none of it is structure here.
public export
-- spelling: (construction-owned -- the boundary word and the article are
-- assigned by the pair: StartOf writes no boundary word at all ("until your
-- next turn", "until the next end step"), EndOf writes "end of" bare and
-- "the end of" possessed, and the possessed combat form extraposes its
-- possessive onto the turn ("until end of combat on your next turn",
-- Brazen Cannonade). Spelled only through Duration.)
data DurationEnd = StartOf TurnPart (Maybe Whose)
                 | EndOf TurnPart (Maybe Whose)

||| WHICH constructions write a given duration adverbial — the fact the
||| corpus assigns and no rule derives. The guide lists the wordings and
||| assigns neither, so the counts are the whole evidence.
|||
||| Two kinds of `False` are worth telling apart, so they are separate
||| rows: `Unattested` means no corpus line writes the phrase at all,
||| while `Unclaimed` means the phrase is real oracle English that this
||| grammar's constructions do not write — the endpoint exists, its clause
||| is somewhere else (a play permission, a copy effect). The rest
||| name the observed splits, and they are strikingly clean: the
||| current-turn adverbials divide the grants from the restrictions
||| exactly, and the cross-turn span is the only one both families write.
|||
||| A row NAME is the set of constructions measured to write its cell, so
||| a new construction renames rows or splits them; the round-by-round
||| record is in `experiment-log.md`. Two measurements belong beside the
||| table: the one corpus line pairing control with an end step writes it
||| as a DELAYED clause, not a duration adverbial, which is why the
||| end-step cells stay `Unclaimed`; and [CR#603.7b] is what puts the
||| delayed clause on "this turn" ("unless it has a stated duration, such
||| as 'this turn'").
|||
||| Two rows grew their names in the layer-words round rather than
||| splitting, which is what the convention above asks for: the base-P/T
||| SETTING joins the two cells it writes — "until end of turn" on 63
||| supported lines and "until your next turn" on three (Mass Diminish,
||| Will Kenrith, Rowan) — and the SWITCH joins one, writing "until end of
||| turn" on all twenty-five of its lines and no other adverbial at all.
||| Neither claims an `Unclaimed` cell: those stay a play permission's, a
||| copy effect's and a rule change's. The [CR#208.2a] DEFINITION joins
||| nothing, being a printed line rather than a clause.
public export
data SpanUse = Unattested | Unclaimed | GrantsAndControl | GrantsTypesControlReplacementPermissionSetAndSwitch
             | KeywordGrantOnly | RestrictionsShieldsPermissionsAndDelays | GrantsRestrictionsReplacementAndBaseSet
             | ControlGrantAndPermission | GrantsRestrictionsControlAndPermission
             | PermissionOnly

