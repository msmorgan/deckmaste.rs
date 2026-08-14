||| The semantics-target workbench: what shape should the semantics layer
||| be, sitting between English and core? Deliberately divorced from
||| `Semantics` — no import, no reuse of its types; nothing here is
||| emitted, mirrored, or run, the typechecker is the only consumer.
|||
||| Findings, deferrals, and frontier notes live in `experiment-log.md`
||| beside this file. (Named off the module on purpose: Idris2 reads a
||| same-named `.md` as literate source, so an `Experimental.md` here
||| would shadow this module.) Per-declaration docs and `-- spelling:`
||| notes stay here, beside the code they describe.
module Experimental

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
-- -- and the extremal word varies perfectly with the AXIS: a
-- characteristic takes greatest/least and a life total takes
-- highest/lowest, with all four crossed spellings ("greatest life total",
-- "highest power", "least life total", "lowest power") written zero times.
-- SumOf writes "total" at both axes)
data AggregateOp = SumOf | MinOf | MaxOf

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
public export
-- spelling: ["haste", "flying", "trample", "vigilance", "deathtouch",
-- "double strike", "first strike"] (bare keyword-ability lines, no
-- params/cost; exact row order follows Keyword)
data Keyword = Haste | Flying | Trample | Vigilance | Deathtouch
             | DoubleStrike | FirstStrike

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
sameSub Zombie Zombie = True
sameSub Zombie _ = False
sameSub Army Army = True
sameSub Army _ = False
sameSub Soldier Soldier = True
sameSub Soldier _ = False
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
||| keyword-counter family. The rule closes that list at fifteen and every
||| row this vocabulary carries is on it, so the table answers True
||| throughout — it is a FORCING function, not a measurement: a keyword row
||| off [CR#122.1b]'s list (banding, phasing) must answer False here before
||| anything can put a counter of its name on anything. Parameterized
||| variants remain outside this vocabulary.
public export
keywordCounterOk : Keyword -> Bool
keywordCounterOk Haste = True
keywordCounterOk Flying = True
keywordCounterOk Trample = True
keywordCounterOk Vigilance = True
keywordCounterOk Deathtouch = True
keywordCounterOk DoubleStrike = True
keywordCounterOk FirstStrike = True

||| The eligibility decision is explicit for every Keyword row, so a future
||| keyword must opt in (or out) before it can become a keyword counter.
public export
data KeywordCounterEligible : Keyword -> Type where
  MkKeywordCounterEligible : {auto 0 ok : keywordCounterOk k = True} ->
                            KeywordCounterEligible k

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
||| row one at a time and only where a corpus line writes it as a one-shot
||| put or remove: `Stun` ([CR#122.1d]) and `Time` are the two that have,
||| while charge, oil, fade, loyalty and shield are not rows — their lines
||| are costs, upkeep triggers and enters-with riders, other axes
||| (ledger). Engine `Counter.confers`, the layer system and the
||| state-based actions stay RON-side throughout.
public export
-- spelling: (BoostCounter delegates its two operations to Counter.Delta and
-- writes them as one word, "+1/+1" or "-0/-1"; KeywordCounter writes the
-- keyword's own lowercase word; Stun and Time write "stun" and "time". Each
-- stands between the count and the noun "counter(s)", never alone.)
data CounterKind : Type where
  BoostCounter : Counter.Delta -> Counter.Delta -> CounterKind
  Stun : CounterKind
  Time : CounterKind
  KeywordCounter : (k : Keyword) -> {auto 0 ok : KeywordCounterEligible k} -> CounterKind
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
||| is somewhere else (a play permission, a base-P/T setting). The rest
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
public export
data SpanUse = Unattested | Unclaimed | GrantsAndControl | GrantsTypesControlReplacementAndPermission
             | KeywordGrantOnly | RestrictionsShieldsPermissionsAndDelays | GrantsRestrictionsAndReplacement
             | ControlGrantAndPermission | GrantsRestrictionsControlAndPermission
             | PermissionOnly

-- ===== Event patterns (the vocabulary the "would" clause and the
-- "until" clause share) =====

||| The events oracle names when it intercepts one, waits for one, or
||| TRIGGERS off one — the vocabulary four constructions read and no rule
||| enumerates. Closed and row-enumerated, so a new event is a totality
||| error on every table below before anything can be written with it.
|||
||| Which events are HERE is the corpus's answer, counted over the whole
||| supported corpus with the moods counted separately because they are
||| different constructions. What sits below the minted rows is ledgered
||| with its count rather than minted: the put-into-a-graveyard event, the
||| becomes-the-target event, the create event, and the life-change pair.
|||
||| The NAMES are the event class and not any construction's spelling,
||| which is chapter twenty-eight's correction: `GameEvent`'s rows carry
||| the finite verb phrase ("dies", "enters") and this key carries the
||| happening it names ("Death", "Entry"), because the round that gave the
||| vocabulary a third and fourth reader is the round the old
||| `Would`-prefixed spelling stopped being true of half of them.
|||
||| Core spells this space as ONE open filter type (`EventFilter`,
||| `deckmaste_core/src/event.rs`) shared by triggers, replacements,
||| durations, and condition lookbacks. The MERGE is taken here — the
||| delayed clause's own query type is gone and reads this vocabulary like
||| everyone else — and what stays apart from core is the table below,
||| which records that the four constructions still disagree about which
||| events they write.
public export
-- spelling: (construction-owned -- each row NAMES a happening whose verb
-- phrase, subject and mood the reading construction supplies: the
-- interception writes it after "would" ("would die", "would be
-- destroyed"), the held-until rider and the trigger header write it
-- finite ("leaves the battlefield", "dies"). Spelled only through
-- GameEvent)
data EventName = Death | Departure | Destruction | DamageTaken
               | CardDrawn | Entry | AttackDeclaration | BlockDeclaration
               | CombatDamage | PartBeginning | SpellCast | StatusChange
               | TurnedFaceUp | PhasingChange | BlockedDeclaration
               | LastCounterRemoval | LifeGain | LifeLoss | TimeShift
               -- a player LOSING the game, named apart from three
               -- neighbours it is not: `OutcomeVerb`'s `LoseGame`, which
               -- is the effect that states it; `LifeLoss`, which is life
               -- and not the game; and `Kind`'s `Outcome`. The word
               -- "loss" is the event's, and the game is what is lost.
               | GameLoss

||| Event-name equality, the closed-vocabulary comparison every other
||| closed word in this file already carries (`sameCounter`,
||| `sameStatusVal`, `sameKeyword`). Minted when the history read's
||| predicate surface needed to compare two phrases wholly: both of that
||| row's arguments are closed words, so `predEq` can answer honestly
||| there instead of conservatively.
public export
sameEventName : EventName -> EventName -> Bool
sameEventName Death Death = True
sameEventName Death _ = False
sameEventName Departure Departure = True
sameEventName Departure _ = False
sameEventName Destruction Destruction = True
sameEventName Destruction _ = False
sameEventName DamageTaken DamageTaken = True
sameEventName DamageTaken _ = False
sameEventName CardDrawn CardDrawn = True
sameEventName CardDrawn _ = False
sameEventName GameLoss GameLoss = True
sameEventName GameLoss _ = False
sameEventName Entry Entry = True
sameEventName Entry _ = False
sameEventName AttackDeclaration AttackDeclaration = True
sameEventName AttackDeclaration _ = False
sameEventName BlockDeclaration BlockDeclaration = True
sameEventName BlockDeclaration _ = False
sameEventName CombatDamage CombatDamage = True
sameEventName CombatDamage _ = False
sameEventName PartBeginning PartBeginning = True
sameEventName PartBeginning _ = False
sameEventName SpellCast SpellCast = True
sameEventName SpellCast _ = False
sameEventName StatusChange StatusChange = True
sameEventName StatusChange _ = False
sameEventName TurnedFaceUp TurnedFaceUp = True
sameEventName TurnedFaceUp _ = False
sameEventName PhasingChange PhasingChange = True
sameEventName PhasingChange _ = False
sameEventName BlockedDeclaration BlockedDeclaration = True
sameEventName BlockedDeclaration _ = False
sameEventName LastCounterRemoval LastCounterRemoval = True
sameEventName LastCounterRemoval _ = False
sameEventName LifeGain LifeGain = True
sameEventName LifeGain _ = False
sameEventName LifeLoss LifeLoss = True
sameEventName LifeLoss _ = False
sameEventName TimeShift TimeShift = True
sameEventName TimeShift _ = False

||| The same for the history window.
public export
sameLookback : Lookback -> Lookback -> Bool
sameLookback ThisTurn ThisTurn = True
sameLookback ThisTurn _ = False
sameLookback ThisCombat ThisCombat = True
sameLookback ThisCombat _ = False
sameLookback LastTurn LastTurn = True
sameLookback LastTurn _ = False

||| WHICH constructions write a clause over a given event — `SpanUse`'s
||| shape asked of the event axis, and it tells the same two silences
||| apart. `EventUnattested` means no corpus line writes any clause over
||| the event in any mood; `EventUnclaimed` means the clause is real
||| oracle English and no construction HERE writes it.
|||
||| Four readers now, and the class names spell the SET each event is
||| written by. `Death` is the widest: the interception, nine hundred and
||| thirty trigger headers, and Graceful Reprieve's delayed clause, while
||| nothing anywhere ends a duration at a death. `Departure` never takes
||| the interception outside a granted quoted ability ("It gains 'If this
||| creature would leave the battlefield, exile it instead'"), a container
||| this grammar has no word for, but it takes the other three: [CR#610.3]
||| riders, trigger headers, and the delayed clause Portcullis and Stangg
||| write. `Destruction` is the one row NO construction claims: its
||| replacement is regeneration's four-part instruction ([CR#614.8] — tap,
||| remove from combat, heal), none of which this vocabulary writes, and
||| its trigger header is written zero times in the modern templating.
||| `DamageTaken` loses the interception for the redirection family's
||| reason ([CR#614.9] — "that damage is dealt to [other] instead", a
||| damage clause whose amount is the intercepted event's) and keeps the
||| trigger. The five new rows are triggers and nothing else, except the
||| turn-part beginning, which is also the delayed clause's ordinary word
||| — though [CR#603.7] says that word "won't usually begin" a delayed
||| trigger, and the corpus agrees at 412 to 1.
public export
data EventUse = EventUnattested | EventUnclaimed | TriggeredOnly
              | InterceptedAndTriggered | InterceptedTriggeredAndDelayed
              | HeldTriggeredAndDelayed | TriggeredAndDelayed

public export
eventUse : EventName -> EventUse
eventUse Death = InterceptedTriggeredAndDelayed
eventUse Departure = HeldTriggeredAndDelayed
eventUse Destruction = EventUnclaimed
eventUse DamageTaken = TriggeredOnly
eventUse CardDrawn = InterceptedAndTriggered
-- The GAME LOSS opens exactly two readers and the corpus is why. EIGHT
-- lines watch it as a trigger ("Whenever a player loses the game, put
-- five +1/+1 counters on this creature") and SEVEN replace it ("If you
-- would lose the game, instead …"), so trigger and intercept are both
-- attested and both land here. The other two are silences: no duration
-- ends at a loss, and no line writes the DELAYED form ("when a player
-- next loses the game this turn") -- which is `badDelayedGameLoss`, and
-- is not the same silence as the replacement's "next time", that being
-- the multiplicity of a replacement and not a delayed trigger.
eventUse GameLoss = InterceptedAndTriggered
eventUse Entry = TriggeredOnly
eventUse AttackDeclaration = TriggeredOnly
eventUse BlockDeclaration = TriggeredOnly
eventUse CombatDamage = TriggeredOnly
eventUse PartBeginning = TriggeredAndDelayed
-- The CAST event, chapter twenty-eight's biggest unminted family and the
-- first row that needed a whole zone under it. It is trigger-only, and
-- each of the other three readers is a measured silence rather than an
-- oversight: nothing INTERCEPTS a cast ([CR#614.1]'s replacements watch
-- events, and the cast is a player ACTION taken with priority -- the
-- nearest real family, "you may cast … without paying its mana cost", is
-- [CR#118.9]'s alternative cost and not a replacement of the casting);
-- no line ends a duration at one; and the DELAYED form is real ("When
-- you next cast a creature spell this turn, …") but not one line of it
-- is writable -- every body either grants an ability to an object on the
-- STACK, which `Gains` refuses by zone, or copies the spell (ledger).
eventUse SpellCast = TriggeredOnly
-- The STATUS transition, trigger-only like the object events beside
-- it: nothing intercepts a becomes-tapped, nothing holds a zone change
-- on one, and nothing delays on one — each a measured silence — while
-- the trigger headers are the family's whole corpus ([CR#603.2e]).
eventUse StatusChange = TriggeredOnly
-- The face-up TURNING, [CR#708.7]'s permission read as an event: the
-- ability that allowed the permanent to be face down "may also allow
-- the permanent's controller to turn it face up", and the headers watch
-- that turning happen. Trigger-only, and the other three readers are
-- measured silences: nothing replaces a turning, no [CR#610.3] rider
-- waits for one, and no delayed clause names one.
eventUse TurnedFaceUp = TriggeredOnly
-- Phasing, the same answer against the same three silences
-- ([CR#702.26b,702.26c] — the status changes, and the trigger headers
-- are the whole corpus of clauses written over it).
eventUse PhasingChange = TriggeredOnly
-- The BLOCKED declaration, [CR#509.1h]'s half of the turn-based action
-- read on the attacker where `BlockDeclaration` reads it on the blocker.
-- Trigger-only, and each other reader is a measured zero: nothing writes
-- "would become blocked" (the evasion abilities that keep a block from
-- happening are restrictions on the declaration, [CR#509.1b], not
-- replacements of an event), no duration ends at one, and no delayed
-- clause names one.
eventUse BlockedDeclaration = TriggeredOnly
-- The LAST counter's removal, [CR#702.62a] and [CR#702.63a] writing the
-- canonical forms into two keywords' expansions. Trigger-only, and the
-- other three readers are measured zeroes: nothing replaces the removal
-- of a last counter, no rider waits for one, no delayed clause names one.
eventUse LastCounterRemoval = TriggeredOnly
-- The two LIFE-CHANGE events, named apart from the OUTCOME words
-- `LifeGained`/`LifeLost` that the deed telescope already owns (the
-- nominalisation is Death's and Departure's). They are the first names
-- in this table
-- with NO PRODUCER: no `GameEvent` row makes one, the gains-life trigger
-- family being unbuilt. `EventUnclaimed` is exactly the cell for that and
-- it needed no invention -- the class means "real oracle English and no
-- construction HERE writes it", which is precisely the situation: the
-- headers are written in quantity and this vocabulary has no row for them
-- yet. The names exist because the LOOKBACK reads `EventName` directly
-- and does not go through `GameEvent` at all, so "if you gained life this
-- turn" is writable while "Whenever you gain life" is not.
eventUse LifeGain = EventUnclaimed
eventUse LifeLoss = EventUnclaimed
-- The day/night SHIFT, the game scope's own event ([CR#731.1a] naming the
-- two phrases). Trigger-only: nothing replaces it, no duration ends at it,
-- no delayed clause names it.
eventUse TimeShift = TriggeredOnly

||| May the would/instead clause intercept an event of this class? Full
||| rows in the answer axis, so a new `EventUse` declares every reader.
public export
admitsIntercept : EventUse -> Bool
admitsIntercept EventUnattested = False
admitsIntercept EventUnclaimed = False
admitsIntercept TriggeredOnly = False
admitsIntercept InterceptedAndTriggered = True
admitsIntercept InterceptedTriggeredAndDelayed = True
admitsIntercept HeldTriggeredAndDelayed = False
admitsIntercept TriggeredAndDelayed = False

||| May a [CR#610.3] zone-change rider wait for an event of this class?
public export
admitsHold : EventUse -> Bool
admitsHold EventUnattested = False
admitsHold EventUnclaimed = False
admitsHold TriggeredOnly = False
admitsHold InterceptedAndTriggered = False
admitsHold InterceptedTriggeredAndDelayed = False
admitsHold HeldTriggeredAndDelayed = True
admitsHold TriggeredAndDelayed = False

||| May a TRIGGERED ability's header name an event of this class
||| ([CR#603.1] — "[When/Whenever/At] [trigger condition or event],
||| [effect]")? The reader chapter twenty-eight added, and the widest of
||| the four: every event this vocabulary spells is written as a trigger
||| header except the destruction, whose modern templating is "dies".
public export
admitsTrigger : EventUse -> Bool
admitsTrigger EventUnattested = False
admitsTrigger EventUnclaimed = False
admitsTrigger TriggeredOnly = True
admitsTrigger InterceptedAndTriggered = True
admitsTrigger InterceptedTriggeredAndDelayed = True
admitsTrigger HeldTriggeredAndDelayed = True
admitsTrigger TriggeredAndDelayed = True

||| May a DELAYED clause wait for an event of this class ([CR#603.7])?
||| The narrowest reader: three events carry the whole family — the
||| turn-part beginning ("at the beginning of the next end step"), the
||| departure (Portcullis, Stangg, Mysterio), and the death (Graceful
||| Reprieve's "when target creature dies this turn").
public export
admitsDelay : EventUse -> Bool
admitsDelay EventUnattested = False
admitsDelay EventUnclaimed = False
admitsDelay TriggeredOnly = False
admitsDelay InterceptedAndTriggered = False
admitsDelay InterceptedTriggeredAndDelayed = True
admitsDelay HeldTriggeredAndDelayed = True
admitsDelay TriggeredAndDelayed = True

||| Which duration-adverbial class an event-ended "until" phrase falls in
||| — the event axis's own row of chapter seventeen's attestation table,
||| and the finding is that every row is a silence.
|||
||| `Departure` is `Unclaimed` and the composition is why: the lines that
||| write "until [object] leaves the battlefield" are overwhelmingly zone
||| changes ([CR#610.3] — a one-shot exile that schedules its own undo,
||| NOT a continuous effect) and phasings ([CR#610.4], the same shape),
||| leaving three genuine [CR#611.2a] continuous durations: two base-TYPE
||| settings and one becomes-a-copy, all constructions this grammar lacks,
||| so the cell names them and claims nothing. Every other event is
||| `Unattested`: no corpus line ends a duration at a death, a
||| destruction, a damage event, or a draw — the lines that appear to
||| write "until … dies" all cross a clause boundary ("until end of turn,
||| whenever another creature dies").
public export
eventSpan : EventName -> SpanUse
eventSpan Death = Unattested
eventSpan Departure = Unclaimed
eventSpan Destruction = Unattested
eventSpan DamageTaken = Unattested
eventSpan CardDrawn = Unattested
eventSpan GameLoss = Unattested
-- The four new OBJECT events are `Unattested` for the same reason the
-- older four are: no corpus line ends a duration at an entry, an attack,
-- a block, or a combat-damage event. The turn-part beginning is the one
-- row that is neither, and its silence is a DOUBLE-SPELLING refusal
-- rather than an absence: "until the beginning of your next upkeep" is
-- real English, and the adverbial that writes it is `DurationEnd`'s own
-- `StartOf` row. One phrase, one slot (`badUntilBeginningOfUpkeep`).
eventSpan Entry = Unattested
eventSpan AttackDeclaration = Unattested
eventSpan BlockDeclaration = Unattested
eventSpan CombatDamage = Unattested
-- Zero lines end a duration at a cast: the eleven "until … cast"
-- matches are all iterated-reveal repetitions ("until they cast a
-- spell" is written none), not adverbials.
eventSpan SpellCast = Unattested
eventSpan PartBeginning = Unclaimed
-- Measured: no corpus line ends a duration at a status transition. The
-- tapped-STATE family is [CR#611.2b]'s "for as long as … remains
-- tapped" condition, another axis entirely; the two "until …" lines
-- that mention the transition end at a TURN boundary and merely
-- contain a becomes-tapped trigger (chapter thirty-five).
eventSpan StatusChange = Unattested
-- Measured, and the face pair's two directions answer differently for
-- once. No line ends a duration at a turning face UP. Exactly one ends
-- at a turning face DOWN — Vesuvan Shapeshifter's "until this creature
-- is turned face down, it becomes a copy of that creature" — which is
-- the only clause of any kind written over the face-down direction and
-- is not this row's: the direction has no event constructor at all, its
-- trigger headers being zero.
eventSpan TurnedFaceUp = Unattested
-- Measured: no line ends a duration at a phasing. The nearest lines end
-- at a DEPARTURE while their effect is a phase-out ("target creature
-- phases out until this enchantment leaves the battlefield"), which is
-- `Departure`'s cell and not this one.
eventSpan PhasingChange = Unattested
-- Measured: no line ends a duration at a block declaration on either
-- side. The two "until … blocks" matches both cross a clause boundary
-- the way the death ones do ("Until end of turn, whenever a creature an
-- opponent controls blocks, draw a card" is a nested TRIGGER inside a
-- turn-bounded span), which is `Death`'s own reading.
eventSpan BlockedDeclaration = Unattested
-- Measured: no duration ends at a last counter's removal. The family's
-- whole corpus is trigger headers.
eventSpan LastCounterRemoval = Unattested
-- No duration ends at a life change; the "until you gain life" spelling is
-- written zero times.
eventSpan LifeGain = Unattested
eventSpan LifeLoss = Unattested
eventSpan TimeShift = Unattested

||| How many times an interception fires — [CR#614.3]'s two ways for a
||| replacement effect to end, "used up" against "duration expired", and
||| English marks the difference with the clause's own opening word.
|||
||| The word tracks the CARRIER, but the table is indexed by the EVENT,
||| which is the one axis it has. Measured exhaustively: "the next time
||| [someone] would [event] this turn" never occurs as a standing ability
||| in the supported corpus — those lines are one-shots spun up by an
||| activation cost or a trigger (the destruction ones being regeneration
||| and nothing else, [CR#614.8]) — while the conditional is what a
||| standing line writes, twenty-one cards printing "If you would draw a
||| card, … instead" with no duration word at all. So a two-dimensional
||| table cannot say what the corpus says: the carrier dimension is
||| LEDGERED, and what stays here is the event dimension with the standing
||| form open wherever a standing line writes one. The cell that reached
||| past its own measurement was the draw's `Repeatedly`, justified by
||| counting the duration-bounded "if you would draw a card THIS TURN"
||| (zero, correctly), which says nothing about the durationless line
||| (`thoughtReflection`). What the table still refuses is the one-shot
||| word where no carrier writes it (`badNextTimeWouldDie`).
public export
-- spelling: (construction-owned -- the clause's opening word: Repeatedly
-- writes "if <event>", NextTimeOnly writes "the next time <event>".
-- Consumed by StaticEffect.Intercepts, never spelled alone)
data ReplUse = Repeatedly | NextTimeOnly

public export
replUseOk : EventName -> ReplUse -> Bool
replUseOk Death Repeatedly = True
replUseOk Death NextTimeOnly = False
replUseOk Departure Repeatedly = True
replUseOk Departure NextTimeOnly = False
-- Both words, and the carrier is what picks between them: regeneration
-- writes the one-shot ([CR#614.8], twenty-five reminder lines) from a
-- cost prefix, and four supported cards print the standing "would be
-- destroyed, … instead" as a line. Neither cell is reachable through
-- this vocabulary — `Interceptable` shuts the event at `Unclaimed`,
-- [CR#614.8]'s four-part replacement being unwritable here — so the row
-- states what is measured rather than what one carrier happens to use.
replUseOk Destruction Repeatedly = True
replUseOk Destruction NextTimeOnly = True
replUseOk DamageTaken Repeatedly = True
replUseOk DamageTaken NextTimeOnly = True
-- The standing draw replacement is twenty-one printed cards — "If you
-- would draw a card, draw two cards instead" (Thought Reflection) and
-- its family — and not one of them writes a duration. The nine "the
-- next time you would draw a card this turn" lines are all one-shots.
replUseOk CardDrawn Repeatedly = True
replUseOk CardDrawn NextTimeOnly = True
-- Both words, both written, and the split is one card apart: five lines
-- write the standing "If you would lose the game, instead …" (Exquisite
-- Archangel, Lich's Mirror, The Golden Throne and two unsupported) and
-- two write the bounded "The next time you would lose the game this
-- turn, instead …" (Stunning Reversal, Nira). So neither cell is a
-- convenience and neither is pinnable.
replUseOk GameLoss Repeatedly = True
replUseOk GameLoss NextTimeOnly = True
-- The five trigger-only events answer NEITHER, and the table says so
-- rather than the interception's own gate saying it twice: an event no
-- would/instead clause writes has no multiplicity word to choose, so
-- both cells are `False` and the refusal a writer meets is
-- `Interceptable`'s.
replUseOk SpellCast Repeatedly = False
replUseOk SpellCast NextTimeOnly = False
replUseOk Entry Repeatedly = False
replUseOk Entry NextTimeOnly = False
replUseOk AttackDeclaration Repeatedly = False
replUseOk AttackDeclaration NextTimeOnly = False
replUseOk BlockDeclaration Repeatedly = False
replUseOk BlockDeclaration NextTimeOnly = False
replUseOk CombatDamage Repeatedly = False
replUseOk CombatDamage NextTimeOnly = False
replUseOk PartBeginning Repeatedly = False
replUseOk PartBeginning NextTimeOnly = False
replUseOk StatusChange Repeatedly = False
replUseOk StatusChange NextTimeOnly = False
replUseOk TurnedFaceUp Repeatedly = False
replUseOk TurnedFaceUp NextTimeOnly = False
replUseOk PhasingChange Repeatedly = False
replUseOk PhasingChange NextTimeOnly = False
replUseOk BlockedDeclaration Repeatedly = False
replUseOk BlockedDeclaration NextTimeOnly = False
replUseOk LastCounterRemoval Repeatedly = False
replUseOk LastCounterRemoval NextTimeOnly = False
-- The life-change replacement IS real ("If you would gain life, …
-- instead") and is `EventUnclaimed`'s business rather than these cells':
-- with no producer there is no event term for a would/instead clause to
-- take, so the multiplicity word has nothing to choose and both cells
-- answer False. When the producer lands, these are the cells to
-- re-measure.
replUseOk LifeGain Repeatedly = False
replUseOk LifeGain NextTimeOnly = False
replUseOk LifeLoss Repeatedly = False
replUseOk LifeLoss NextTimeOnly = False
replUseOk TimeShift Repeatedly = False
replUseOk TimeShift NextTimeOnly = False

-- ===== The trigger header's opening word =====

||| The word a triggered ability opens with ([CR#603.1] — "[When/
||| Whenever/At] [trigger condition or event], [effect]"). No rule
||| distinguishes the three: [CR#113.3c] lists them together as words a
||| triggered ability "include(s) (and usually begin(s) with)", and
||| nothing assigns one to an event. So the word is a SLOT and the corpus
||| is the only evidence about which slot fillings are real.
|||
||| What the corpus assigns absolutely is `At`, and [CR#603.2b] is why:
||| it gives "at the beginning of" a phase or step its own clause, and
||| the corpus honors it without exception — every "At the beginning of"
||| line is a turn-part beginning and no object event takes the word.
|||
||| What the corpus does NOT assign is the When/Whenever split. The word
||| tracks REPEATABILITY: a once-per-object event with a fixed subject
||| takes "When", the same event over a DESCRIPTION takes "Whenever"
||| because the description ranges over many objects, and an event that
||| repeats for one object takes "Whenever" even with the fixed subject.
||| The residue is what keeps it out of the type: every counterexample is
||| an ability that destroys its own source, so the word is answering a
||| question about the EFFECT. A gate keyed on the event alone would have
||| to refuse real oracle either way, so it refuses only what [CR#603.2b]
||| settles.
public export
-- spelling: ["When", "Whenever", "At"] (the header's first word, followed
-- by the event clause, a comma, and the effect; the turn-part event
-- supplies "the beginning of" itself -- see GameEvent.BeginningOf),
-- kind: TODO(reason: header fragment -- not one of Nominal/Sentence/
-- Cost/KeywordLine/Ability)
data TriggerWord = When | Whenever | At

||| Which opening word an event's header may take. Full rows over
||| (event x word), so a new event declares its word before it can be
||| triggered on. Every object event answers `True` to both English
||| words and `False` to `At`; the turn-part beginning inverts it exactly
||| ([CR#603.2b]). The two events no trigger writes at all answer `False`
||| throughout, and the refusal a writer meets there is `Triggerable`'s.
public export
triggerWordOk : EventName -> TriggerWord -> Bool
triggerWordOk Death When = True
triggerWordOk Death Whenever = True
triggerWordOk Death At = False
triggerWordOk Departure When = True
triggerWordOk Departure Whenever = True
triggerWordOk Departure At = False
triggerWordOk Destruction When = False
triggerWordOk Destruction Whenever = False
triggerWordOk Destruction At = False
triggerWordOk DamageTaken When = True
triggerWordOk DamageTaken Whenever = True
triggerWordOk DamageTaken At = False
triggerWordOk CardDrawn When = True
triggerWordOk CardDrawn Whenever = True
triggerWordOk CardDrawn At = False
-- Both words again, and the eight watching lines divide them the way
-- finding 247 said they divide everywhere: `Whenever` for the repeatable
-- class (five lines, "whenever a player loses the game") and `When` for
-- the singular occasion (two, both naming ONE player the card has
-- already fixed -- the enchanted player, the chosen player). `At` is
-- [CR#603.2b]'s and stays there (`badGameLossAtTrigger`).
triggerWordOk GameLoss When = True
triggerWordOk GameLoss Whenever = True
triggerWordOk GameLoss At = False
triggerWordOk Entry When = True
triggerWordOk Entry Whenever = True
triggerWordOk Entry At = False
-- Both header words, and the corpus divides them the way finding 172
-- said it would: "Whenever [someone] casts" at nine hundred forty-eight
-- against a hundred twenty-one "When", the casting being a repeatable
-- event whichever subject it takes. `At` is refused with every other
-- object event ([CR#603.2b] reserves it for phases and steps).
triggerWordOk SpellCast When = True
triggerWordOk SpellCast Whenever = True
triggerWordOk SpellCast At = False
triggerWordOk AttackDeclaration When = True
triggerWordOk AttackDeclaration Whenever = True
triggerWordOk AttackDeclaration At = False
triggerWordOk BlockDeclaration When = True
triggerWordOk BlockDeclaration Whenever = True
triggerWordOk BlockDeclaration At = False
triggerWordOk CombatDamage When = True
triggerWordOk CombatDamage Whenever = True
triggerWordOk CombatDamage At = False
triggerWordOk PartBeginning When = False
triggerWordOk PartBeginning Whenever = False
triggerWordOk PartBeginning At = True
-- Both English words: the direct "Whenever … becomes tapped" mass
-- against the Aura family's "When enchanted … becomes tapped"
-- ([CR#603.1] assigns neither word; the guide's split tracks whether
-- the event is naturally singular in context, which the EFFECT decides
-- — an Aura's destroy ends the relationship — so the grammar declines
-- to encode it, tolerated over-generation). `At` is refused with every
-- other object event ([CR#603.2b]).
triggerWordOk StatusChange When = True
triggerWordOk StatusChange Whenever = True
triggerWordOk StatusChange At = False
-- Both English words, and this family writes the split the other way
-- round from the mass: the morph line "When this creature is turned
-- face up, …" is the largest cell by far, "Whenever a permanent you
-- control is turned face up, …" the description-subject one. The
-- grammar declines to encode the split here as everywhere ([CR#603.1]
-- assigns no word), and refuses `At` with every other object event
-- ([CR#603.2b]).
triggerWordOk TurnedFaceUp When = True
triggerWordOk TurnedFaceUp Whenever = True
triggerWordOk TurnedFaceUp At = False
-- Both again, though the corpus is nearly all `Whenever`: the one
-- `When` line ("When this creature phases out or leaves the
-- battlefield, …") writes a COORDINATED event this vocabulary has no
-- word for, so the cell is opened on the standing slot policy rather
-- than on a writable line.
triggerWordOk PhasingChange When = True
triggerWordOk PhasingChange Whenever = True
triggerWordOk PhasingChange At = False
-- Both words across the whole block family, 250 `Whenever` to 20 `When`,
-- and the split is the standing tolerated one: [CR#603.1] assigns no
-- word and the guide's repeatability test keys on the effect. `At` stays
-- [CR#603.2b]'s.
triggerWordOk BlockedDeclaration When = True
triggerWordOk BlockedDeclaration Whenever = True
triggerWordOk BlockedDeclaration At = False
-- Every explicit line writes `When` and none writes `Whenever`, which is
-- what a once-per-object event should look like -- but the grammar
-- declines to encode the split here as everywhere ([CR#603.1] assigns no
-- word), so the cell stays open as tolerated over-generation. `At` is
-- [CR#603.2b]'s.
triggerWordOk LastCounterRemoval When = True
triggerWordOk LastCounterRemoval Whenever = True
triggerWordOk LastCounterRemoval At = False
-- Answered from the English the corpus writes ("Whenever you gain life,
-- …", "When you gain life, …") rather than left blank, though no producer
-- can reach these cells yet: a table with full rows has to say something,
-- and saying what the corpus says is the answer that stays true when the
-- producer lands. `At` is [CR#603.2b]'s as always.
triggerWordOk LifeGain When = True
triggerWordOk LifeGain Whenever = True
triggerWordOk LifeGain At = False
triggerWordOk LifeLoss When = True
triggerWordOk LifeLoss Whenever = True
triggerWordOk LifeLoss At = False
-- Every one of the ten lines writes `Whenever`; the cell for `When`
-- stays open on the standing slot policy ([CR#603.1] assigns no word).
triggerWordOk TimeShift When = True
triggerWordOk TimeShift Whenever = True
triggerWordOk TimeShift At = False

||| WHICH events a history query names, and with a subject of WHICH
||| SORT — the lookback's attestation table, two-axis because one axis
||| could not say what the corpus says. `replUseOk`'s and `triggerWordOk`'s
||| shape at a third site.
|||
||| The second axis is forced by one family. Every other event name takes
||| a subject of a single sort in this position, but the ATTACK
||| declaration takes both: "if you attacked this turn" is the raid mass,
||| thirty-eight lines with a PLAYER subject, and "if this creature
||| attacked or blocked this turn" is six with an OBJECT one. A
||| single-valued `EventName -> Kind` table would have had to refuse one of
||| them, so the table is over the pair and `Happened` is indexed at the
||| kind it answers True for.
|||
||| Full rows over (name x the two phrasal kinds), each cell measured in
||| the CONDITION position and nowhere else. The Trues: death (the morbid
||| mass), departure (12), entry, damage taken (6), block declaration (4)
||| and attack declaration (6) on the object side; spell cast (21), card
||| drawn, life gained (37), life lost (18) and attack declaration (38) on
||| the player side. The zeroes are all queried and all real — no line
||| writes a destruction, a status change, a face turning, a phasing, a
||| becomes-blocked, a last-counter removal or a turn-part beginning as a
||| history read, and "if an upkeep began this turn" is not English.
|||
||| Two cells are refused for a reason SHARPER than a zero and the
||| distinction is worth keeping: combat damage is written twice in this
||| position and both lines carry a RECIPIENT complement ("if this
||| creature dealt combat damage to an opponent this turn") that a
||| one-noun query cannot spell, and the player side of the attack
||| declaration writes a complement too on the lines that are not bare
||| ("if you attacked with a Hero this turn"). The complement is a real
||| gap, ledgered, and admitting the cells on lines the row cannot finish
||| would have hidden it.
public export
lookbackSubjectOk : EventName -> Kind -> Bool
lookbackSubjectOk Death Object = True
lookbackSubjectOk Death Player = False
lookbackSubjectOk Departure Object = True
lookbackSubjectOk Departure Player = False
lookbackSubjectOk Destruction Object = False
lookbackSubjectOk Destruction Player = False
lookbackSubjectOk DamageTaken Object = True
-- RE-MEASURED when the second reader landed, and the only cell that
-- moved. Chapter forty-two measured every cell in the condition position
-- because that was the only reader there was, and found one line ("if you
-- haven't been dealt combat damage since your last turn") in a window this
-- vocabulary does not carry. The PREDICATE position writes the same event
-- over a player eight times -- "for each opponent who was dealt damage
-- this turn", "each player who was dealt combat damage this turn" -- so
-- the cell is True. A shared table's cells are a property of the QUERY
-- and not of a reader, which is why a new reader can only ever add Trues:
-- the union is what the table always meant, and the round that adds a
-- reader owes the re-measurement.
lookbackSubjectOk DamageTaken Player = True
lookbackSubjectOk CardDrawn Object = False
lookbackSubjectOk CardDrawn Player = True
-- The loss event's history cells are CLOSED, and this round measured
-- them rather than inheriting the queue's guess that a history read
-- might exist. Two corpus lines look backward at losses and neither is
-- this query: Hot Pursuit's "if two or more players have lost the game"
-- and Rampant Frogantua's "for each player who has lost the game" both
-- COUNT PLAYERS in a standing state, and neither names a window --
-- there is no "lost the game this turn" anywhere. This row asks whether
-- an event happened inside a `Lookback`, so a windowless count over
-- players is a different reader's business (the player-set count, which
-- this grammar has no term for). Recorded, closed, and pinned at the
-- player sort (`badGameLossLookback`).
lookbackSubjectOk GameLoss Object = False
lookbackSubjectOk GameLoss Player = False
lookbackSubjectOk Entry Object = True
lookbackSubjectOk Entry Player = False
lookbackSubjectOk AttackDeclaration Object = True
lookbackSubjectOk AttackDeclaration Player = True
lookbackSubjectOk BlockDeclaration Object = True
lookbackSubjectOk BlockDeclaration Player = False
lookbackSubjectOk CombatDamage Object = False
lookbackSubjectOk CombatDamage Player = False
lookbackSubjectOk PartBeginning Object = False
lookbackSubjectOk PartBeginning Player = False
lookbackSubjectOk SpellCast Object = False
lookbackSubjectOk SpellCast Player = True
lookbackSubjectOk StatusChange Object = False
lookbackSubjectOk StatusChange Player = False
lookbackSubjectOk TurnedFaceUp Object = False
lookbackSubjectOk TurnedFaceUp Player = False
lookbackSubjectOk PhasingChange Object = False
lookbackSubjectOk PhasingChange Player = False
lookbackSubjectOk BlockedDeclaration Object = False
lookbackSubjectOk BlockedDeclaration Player = False
lookbackSubjectOk LastCounterRemoval Object = False
lookbackSubjectOk LastCounterRemoval Player = False
lookbackSubjectOk LifeGain Object = False
lookbackSubjectOk LifeGain Player = True
lookbackSubjectOk LifeLoss Object = False
lookbackSubjectOk LifeLoss Player = True
-- the shift has no SUBJECT of either sort -- the game is not a phrasal
-- kind -- so no history read can be written over it in either position.
lookbackSubjectOk TimeShift Object = False
lookbackSubjectOk TimeShift Player = False
-- the two non-phrasal kinds carry no subject at all: a quality names no
-- participant and an outcome is a deed's own mention.
lookbackSubjectOk _ (Quality _) = False
lookbackSubjectOk _ Outcome = False
lookbackSubjectOk _ Gap = False
lookbackSubjectOk _ (Letter _) = False

public export
data LookbackSubject : EventName -> Kind -> Type where
  MkLookbackSubject : {auto 0 ok : lookbackSubjectOk ev k = True} ->
                      LookbackSubject ev k

-- ===== Which turn-part beginnings a header names =====

||| The two silences again, asked of the (part x possession) grid the
||| turn-part trigger header writes. `PartUnattested` is a beginning no
||| corpus line names; `PartUnclaimed` is one it names with a possessor
||| this vocabulary has no word for.
public export
data PartUse = PartUnattested | PartUnclaimed | PartTriggered

||| WHICH turn-part beginnings a header names, over the SHARED endpoint
||| vocabulary chapter seventeen minted for durations and said out loud
||| was the trigger's too. Full rows over five parts and three
||| possessions — fifteen cells — so a new `TurnPart` or a new `Whose` is
||| a totality error here as well as on `spanUse`.
|||
||| `Upkeep` is the family's centre, possessed. `EndStep` writes both the
||| possessed form and the UNPOSSESSED one ("At the beginning of the end
||| step, destroy all Goblins"), the one cell where the bare part word is
||| the whole phrase. `Combat` writes the possessed form and extraposes
||| the possessive onto the turn ("At the beginning of combat on your
||| turn"), which is Brazen Cannonade's move in the duration table
||| appearing here as an ordinary spelling. `Turn` and `UntapStep` are
||| written ZERO times in every possession: the turn's own beginning is
||| not a trigger event English names — the upkeep is what it names
||| instead — and no line triggers at an untap step's beginning.
|||
||| The `PartUnclaimed` cells are the possessor gap chapter twenty-seven
||| measured from the other side: "at the beginning of each upkeep",
||| "each opponent's upkeep", "the upkeep of enchanted creature's
||| controller" and "each end step" are real headers whose possessor is a
||| QUANTIFIER or a nominal, where `Whose` is a two-word pronominal
||| vocabulary by construction. `ThatPlayers` stays a silence here too:
||| one line writes "at the beginning of that player's upkeep" and none
||| writes any other part.
public export
partUse : TurnPart -> Maybe Owner -> PartUse
partUse Turn Nothing = PartUnattested
partUse Turn (Just Yours) = PartUnattested
partUse Turn (Just ThatPlayers) = PartUnattested
partUse Turn (Just EachPlayers) = PartUnattested
partUse Turn (Just EachOpponents) = PartUnattested
partUse Turn (Just EachYours) = PartUnattested
partUse Turn (Just AnOpponents) = PartUnattested
-- RECLASSIFIED. This cell was `PartUnclaimed` -- "names it with a
-- possessor this vocabulary has no word for" -- and the words now exist,
-- so the honest answer is that NOTHING writes a possessor-less upkeep
-- header at all: the 36 "each upkeep" lines are `EachPlayers`' second
-- spelling, not this cell's (`badTriggerAtEachUpkeep` still refuses, the
-- pin unaffected by which silence it names).
partUse Upkeep Nothing = PartUnattested
partUse Upkeep (Just Yours) = PartTriggered
partUse Upkeep (Just ThatPlayers) = PartUnclaimed
partUse Upkeep (Just EachPlayers) = PartTriggered
partUse Upkeep (Just EachOpponents) = PartTriggered
partUse Upkeep (Just EachYours) = PartUnattested
partUse Upkeep (Just AnOpponents) = PartUnattested
-- The possessor-less end step is the DELAYED clause's and not the
-- header's: "at the beginning of the next end step" writes no possessor
-- because it names one occurrence rather than a class, and the header's
-- bare "each end step" is `EachPlayers`' second spelling. One cell, one
-- construction, and the round that gave the header its quantifiers is
-- what made the difference sayable.
partUse EndStep Nothing = PartTriggered
partUse EndStep (Just Yours) = PartTriggered
partUse EndStep (Just ThatPlayers) = PartUnattested
partUse EndStep (Just EachPlayers) = PartTriggered
partUse EndStep (Just EachOpponents) = PartTriggered
partUse EndStep (Just EachYours) = PartUnattested
partUse EndStep (Just AnOpponents) = PartUnattested
partUse Combat Nothing = PartUnattested
partUse Combat (Just Yours) = PartTriggered
partUse Combat (Just ThatPlayers) = PartUnattested
partUse Combat (Just EachPlayers) = PartTriggered
partUse Combat (Just EachOpponents) = PartUnattested
partUse Combat (Just EachYours) = PartUnattested
partUse Combat (Just AnOpponents) = PartUnattested
partUse UntapStep Nothing = PartUnattested
partUse UntapStep (Just Yours) = PartUnattested
partUse UntapStep (Just ThatPlayers) = PartUnattested
partUse UntapStep (Just EachPlayers) = PartUnattested
partUse UntapStep (Just EachOpponents) = PartUnattested
partUse UntapStep (Just EachYours) = PartUnattested
partUse UntapStep (Just AnOpponents) = PartUnattested
partUse EndOfCombat Nothing = PartTriggered
partUse EndOfCombat (Just Yours) = PartUnattested
partUse EndOfCombat (Just ThatPlayers) = PartUnattested
partUse EndOfCombat (Just EachPlayers) = PartUnattested
partUse EndOfCombat (Just EachOpponents) = PartUnattested
partUse EndOfCombat (Just EachYours) = PartUnattested
partUse EndOfCombat (Just AnOpponents) = PartUnattested
partUse FirstMain Nothing = PartUnattested
partUse FirstMain (Just Yours) = PartTriggered
partUse FirstMain (Just ThatPlayers) = PartUnattested
partUse FirstMain (Just EachPlayers) = PartTriggered
partUse FirstMain (Just EachOpponents) = PartUnattested
partUse FirstMain (Just EachYours) = PartUnattested
partUse FirstMain (Just AnOpponents) = PartUnattested
partUse PostcombatMain Nothing = PartUnattested
-- The SPELLING SPLIT, and at this site it is perfect: the possessor
-- `Yours` writes "your second main phase" (5 headers) and `EachYours`
-- writes "each of your postcombat main phases" (7), with both cross
-- cells at zero -- no header writes "your postcombat main phase" or
-- "each of your second main phases". [CR#505.1] names the two words for
-- one phase ("the first main phase (also known as the precombat main
-- phase)"), so this is finding 275's agreement a fourth time: one part
-- row, two spellings, chosen by the possessor. The scope matters and is
-- stated -- "your postcombat main phase" occurs 8 times ELSEWHERE, so
-- the perfection is the header reader's and not the language's.
partUse PostcombatMain (Just Yours) = PartTriggered
partUse PostcombatMain (Just ThatPlayers) = PartUnattested
partUse PostcombatMain (Just EachPlayers) = PartUnattested
partUse PostcombatMain (Just EachOpponents) = PartUnattested
partUse PostcombatMain (Just EachYours) = PartTriggered
partUse PostcombatMain (Just AnOpponents) = PartUnattested
partUse DrawStep Nothing = PartUnattested
partUse DrawStep (Just Yours) = PartTriggered
partUse DrawStep (Just ThatPlayers) = PartUnattested
partUse DrawStep (Just EachPlayers) = PartTriggered
partUse DrawStep (Just EachOpponents) = PartUnattested
partUse DrawStep (Just EachYours) = PartUnattested
partUse DrawStep (Just AnOpponents) = PartUnattested

||| The answer axis, full rows, so a new `PartUse` declares its reader.
public export
admitsPartTrigger : PartUse -> Bool
admitsPartTrigger PartUnattested = False
admitsPartTrigger PartUnclaimed = False
admitsPartTrigger PartTriggered = True

-- ===== Prevention shields (the damage class and the shield's size) =====

||| WHICH damage a prevention shield stops. [CR#615.1] makes the shield
||| watch "a damage event that would happen", and the only qualifier
||| oracle puts on the noun is the combat/noncombat split ([CR#510.2] —
||| combat damage is what the combat damage step deals, which is the whole
||| of what the adjective names). Closed at three because no fourth
||| qualifier is written: no line prevents "all trample damage" or "all
||| excess damage".
public export
-- spelling: ["damage", "combat damage", "noncombat damage"] (the bare noun
-- and its two adjectives; spelled only through StaticEffect.Prevents)
data DamageKind = AnyDamage | CombatOnly | NoncombatOnly

-- ===== Deontic restrictions (the one-shot "can't" vocabulary) =====

||| The deed a restriction denies — the verb alone. Core and the
||| predecessor grammar both mark WHICH PART the subject plays by which
||| SLOT carries the reference — `DeonticAction::Block { by, on }`
||| (`deckmaste_core/src/deontic.rs`), `Enact Block <agent> <patient>`
||| (`Semantics.idr`) — but a clause with ONE subject has no second slot
||| to put it in, so English marks it in the VOICE: "can't block" against
||| "can't be blocked". That is an axis of its own (`Role`), not a second
||| deed word, so this enum stops at the verbs. Closed and row-enumerated,
||| and every table over it is written out, so a new deed must declare
||| which types carry its grant, in which voice, before it can be written
||| at all. These lead the one-shot restrictions in the corpus; the rest
||| of the deontic surface is either the parked ability layer or a deed
||| with no clause of its own yet.
public export
-- spelling: ["attack", "block"] (the bare verb; `Role` inflects it into
-- the verb phrase after "can't" -- "block" against "be blocked" -- and the
-- PAIR is what Effect.Cant spells, never this enum alone)
data Deed = Attack | Block

||| Which part the restriction's SUBJECT plays in the deed: core's two
||| slots as one axis, since a one-subject clause has only its voice to
||| say it with. `Agent` is the active reading and core's `by` slot
||| ("can't block"), `Patient` the passive and core's `on` ("can't be
||| blocked"). The words are the predecessor grammar's own
||| (`Enact Block <agent> <patient>`, `Semantics.idr`).
public export
-- spelling: (construction-owned -- the VOICE of Deed's verb: Agent leaves
-- it bare, Patient makes it passive ("be blocked"). Consumed by
-- Effect.Cant, never spelled alone)
data Role = Agent | Patient

||| The three POLARITIES a deed clause can carry — the convergence
||| chapter forty-six named and deferred, executed here on its own stated
||| trigger (finding 331: "a GATE polarity with evidence would justify
||| refactoring both rows onto one compulsion axis").
|||
||| All three prior arts agree on the shape and disagree on the spine.
||| Core has `Deontic{May, Cant, Must, Gate}` over a `DeonticAction` whose
||| variants carry `by`/`on` predicates (`deckmaste_core/src/deontic.rs`);
||| the OLD module has `Constrain : Compulsion -> Deed` over an eight-kind
||| relation spine with a `Priced` sibling for gates (`Semantics.idr`);
||| this vocabulary keeps its own narrow `Deed x Role` grid and puts the
||| polarity on ONE axis over it. What is NOT taken from either is the
||| PERMISSION: core's `May` is the existential floor a granted row
||| widens, and this grammar has no line asking for it — "may attack" is
||| written zero times, attacking being permitted by default ([CR#506.3]).
||| Three rows, measured; a fourth would be symmetry.
|||
||| The GATE carries its cost on the VALUE rather than in a fourth field
||| on the row, which is `BoostCounter`'s arrangement and for its reason:
||| the payload exists exactly where the polarity needs it, so no other
||| polarity has to carry an empty slot and no gate can be written without
||| one.
public export
data CompTag = ForbidT | RequireT | GateT

||| Whether a polarity's cell writes a PATIENT, and the answer is
||| three-valued because the corpus is: the block restriction writes one
||| in fourteen lines and omits it in many more, the block requirement
||| writes one in all thirty-eight, and every other cell writes none.
public export
data PatientNeed = PatientRefused | PatientOptional | PatientRequired

public export
notRequired : PatientNeed -> Bool
notRequired PatientRequired = False
notRequired PatientOptional = True
notRequired PatientRefused = True

public export
admitsPatient : PatientNeed -> Bool
admitsPatient PatientRefused = False
admitsPatient PatientOptional = True
admitsPatient PatientRequired = True

||| WHICH cell writes a patient. Full rows over (polarity x deed x role),
||| every cell measured in its own family and none by symmetry.
public export
deonticPatientOk : CompTag -> Deed -> Role -> PatientNeed
-- the restriction: "can't block creatures with power 3 or greater" is
-- fourteen lines and the bare "can't block" many more, so the block
-- agent's cell is OPTIONAL. No restriction names an attack's defender in
-- this slot -- "can't attack you" writes a DEFENDING PLAYER, which is a
-- different participant and has no noun here (ledger).
deonticPatientOk ForbidT Attack Agent = PatientRefused
deonticPatientOk ForbidT Attack Patient = PatientRefused
deonticPatientOk ForbidT Block Agent = PatientOptional
deonticPatientOk ForbidT Block Patient = PatientRefused
-- the requirement: finding 334's measurement unchanged, required in one
-- cell and refused in three.
deonticPatientOk RequireT Attack Agent = PatientRefused
deonticPatientOk RequireT Attack Patient = PatientRefused
deonticPatientOk RequireT Block Agent = PatientRequired
deonticPatientOk RequireT Block Patient = PatientRefused
-- the gate: Hipparion writes one ("can't block creatures with power 3 or
-- greater unless you pay {1}") and the plural-subject lines write none,
-- so the same cell is optional here too. The attack gates write no
-- patient at all.
deonticPatientOk GateT Attack Agent = PatientRefused
deonticPatientOk GateT Attack Patient = PatientRefused
deonticPatientOk GateT Block Agent = PatientOptional
deonticPatientOk GateT Block Patient = PatientRefused

||| Which card types carry a deed's grant IN A GIVEN VOICE — the stand-in
||| for reading `May(Attack)`/`May(Block)` off the TypeDef declaration
||| (`plugins/builtin/macros/cardtype/Creature.ron`), and a DIFFERENT
||| table from `combatant`, which the fight chapter minted precisely
||| because fight keys on type membership and deals non-combat damage
||| ([CR#701.14b,701.14d]) where these deeds are combat proper.
||| [CR#506.3] — "Only a creature can attack or block" — answers the
||| active rows, and the passive of BLOCK too, because what a blocker
||| blocks is an attacking creature ([CR#509.1a]). The passive of ATTACK
||| is answered by the SECOND sentence of that same rule: only a player, a
||| planeswalker, or a battle can be attacked, and not one of those is a
||| card type this grammar spells. So that whole row is False — and False
||| for a reason worth writing down, because the PHRASE is real oracle
||| ("The Aetherspark can't be attacked" of a planeswalker, "you can't be
||| attacked except by creatures with flying" of a player). The row waits
||| on the planeswalker and battle card types and on the ledgered
||| player-subject restriction, not on a corpus witness.
public export
deedType : Deed -> Role -> CardType -> Bool
deedType Attack Agent Creature = True
deedType Attack Agent Artifact = False
deedType Attack Agent Land = False
deedType Attack Agent Enchantment = False
-- The two SPELL types answer False everywhere for a reason no count is
-- needed for: [CR#506.3] says "only a creature can attack or block",
-- and an instant or sorcery is never even a permanent ([CR#110.4]:
-- "instant and sorcery cards can't enter the battlefield and thus
-- can't be permanents").
deedType Attack Agent Instant = False
deedType Attack Agent Sorcery = False
deedType Attack Patient Creature = False
deedType Attack Patient Artifact = False
deedType Attack Patient Land = False
deedType Attack Patient Enchantment = False
deedType Attack Patient Instant = False
deedType Attack Patient Sorcery = False
deedType Block Agent Creature = True
deedType Block Agent Artifact = False
deedType Block Agent Land = False
deedType Block Agent Enchantment = False
deedType Block Agent Instant = False
deedType Block Agent Sorcery = False
deedType Block Patient Creature = True
deedType Block Patient Artifact = False
deedType Block Patient Land = False
deedType Block Patient Enchantment = False
deedType Block Patient Instant = False
deedType Block Patient Sorcery = False

||| The deed's demand on its subject's projected head, as a witness —
||| `FightParticipant`'s shape for the combat grants, reading the voice
||| along with the verb. There is no `Nothing` row: an UNTYPED head
||| cannot prove participation, so a disjunctive subject, which
||| honestly fixes no type (finding 50), is refused rather than waved
||| through on its silence.
public export
data DeedParticipant : Deed -> Role -> Maybe CardType -> Type where
  Participant : {auto 0 ok : deedType d r t = True} -> DeedParticipant d r (Just t)

||| Which static effect a `Continuously` clause establishes, as the span
||| tables' key — the axis `spanUse` classifies its adverbials against.
||| One row per `StaticEffect` constructor (`staticKind`), so a new static
||| row is a totality error on both tables and must declare which
||| durations it writes before it can be written at all.
|||
||| `Replacement` and `Prevention` carry no subject noun where the other
||| five do, and [CR#611.2c] is why: it cuts the continuous effects in two
||| — those that "modify the characteristics or change the controller of
||| any objects", whose affected set is fixed when the effect begins, and
||| those that do neither and therefore "modify the rules of the game" —
||| and the rule's own worked example is a prevention effect.
|||
||| `Conditional`, `PlayPermission` and `EntryRider` are not clause
||| constructions at all: they answer `False` to every cell of
||| `admitsSpan` and to `absentOk`, so no `Continuously` clause can
||| establish one, and the only construction that writes them is the
||| static ABILITY line (`staticAsAbility`) — a table with two readers
||| rather than one, the same move `eventUse` makes.
public export
data StaticKind = PtDelta | KeywordGrant | DeedRestriction | TypeAddition
                | ControlGrant | Replacement | Prevention
                | Conditional | PlayPermission | EntryRider
                | CostModification

||| The MARKING WORD a conditional static writes over its condition. "As
||| long as" and "unless" are not two constructions but one construction
||| with two spellings of the SAME negation: "unless [C]" holds the
||| statement true while C is false, which is "as long as not [C]" with
||| the "not" moved onto the marking word. No rule assigns either word —
||| [CR#611.3a] licenses the wrapper and says nothing about its surface —
||| so this is the corpus's fact, and the corpus is lopsided: the bare "as
||| long as" is the overwhelming majority and rarely negated, while
||| "unless" writes only the two shapes that need it, "can't … unless" and
||| "enters tapped unless".
public export
-- spelling: ["as long as", "unless"] (row order: AsLongAs/Unless; the
-- subordinator introducing the condition clause. Unless writes the
-- NEGATION it carries, so its condition's own words are the positive
-- ones -- "unless you control an artifact", never "unless you control no
-- artifact". Spelled only through StaticEffect.Conditionally)
data CondMarking = AsLongAs | Unless

||| The VERB a play permission writes. [CR#604.6] brackets the pair in
||| its own templates — "You may [cast/play] [this card] …" — so this is
||| a slot the rule itself declares, and chapter twenty-eight's reading of
||| it needed one correction: it recorded the verb as DERIVED from the
||| complement's kind, "cast" belonging to a spell and "play" to a card,
||| and [CR#701.5b] says the opposite in four words — "to cast a card is
||| to cast it as a spell". A card is what both verbs take.
|||
||| What separates them is the LAND, and the glossary derivation is still
||| the right one read the other way: "playing a card" means "playing that
||| card as a land or casting that card as a spell, whichever is
||| appropriate" ([CR#601.1a]), so "play" is the union and "cast" the half
||| of it that excludes lands — [CR#305.9] making that exclusion a rule,
||| "if an object is both a land and another card type, it can be played
||| only as a land. It can't be cast as a spell."
public export
-- spelling: ["play", "cast"] (row order: Play/Cast; the bare verb after
-- the modal, "you may play <what>" / "you may cast <what>". Spelled only
-- through StaticEffect.MayPlay)
data PlayVerb = Play | Cast

||| Which verb may take which complement. `Play` takes anything
||| ([CR#601.1a]'s union), `Cast` refuses a LAND-headed one outright
||| ([CR#305.9]) and passes a silent head for `zoneFits`' reason — a
||| phrase that names no type has not named a land (`badCastALand`).
public export
castableTy : PlayVerb -> Maybe CardType -> Bool
castableTy Play _ = True
castableTy Cast Nothing = True
castableTy Cast (Just Creature) = True
castableTy Cast (Just Artifact) = True
castableTy Cast (Just Land) = False
castableTy Cast (Just Enchantment) = True
castableTy Cast (Just Instant) = True
castableTy Cast (Just Sorcery) = True

||| The attestation table's other half: which classes of adverbial each
||| construction writes. Full rows in both directions. The shape is the
||| finding — the cross-turn span is the one class both GRANTS and the
||| RESTRICTION admit, and the two current-turn words divide those
||| families with nothing shared, which is why the detain family's
||| cross-turn span ("can't attack or block until your next turn") is the
||| one place a restriction and a grant write the same words. The type
||| addition is out of the class entirely.
|||
||| The `False` cells that rest on a MEASUREMENT rather than a rule. The
||| TYPE ADDITION takes "until end of turn" and never end of combat, and
||| Coward // Killer settles the current-turn split inside one sentence,
||| giving the restriction "this turn" and the addition "until end of
||| turn"; its cross-turn cell is closed because no line writes a NAKED
||| type addition across turns, the three that look like it being
||| otherwise explained (`badTypeAdditionAcrossTurns`). The CONTROL grant
||| is the widest of the five yet writes "this turn" ZERO times, so the
||| restriction's own current-turn word stays the restriction's alone.
||| Prevention writes ONE adverbial, "this turn", and nothing writes an
||| interception or a shield at an upkeep, at a combat endpoint, or under
||| a for-as-long-as condition.
public export
admitsSpan : StaticKind -> SpanUse -> Bool
admitsSpan PtDelta Unattested = False
admitsSpan PtDelta Unclaimed = False
admitsSpan PtDelta GrantsAndControl = True
admitsSpan PtDelta GrantsTypesControlReplacementAndPermission = True
admitsSpan PtDelta KeywordGrantOnly = False
admitsSpan PtDelta RestrictionsShieldsPermissionsAndDelays = False
admitsSpan PtDelta GrantsRestrictionsAndReplacement = True
admitsSpan PtDelta ControlGrantAndPermission = False
admitsSpan PtDelta GrantsRestrictionsControlAndPermission = True
admitsSpan PtDelta PermissionOnly = False
admitsSpan KeywordGrant Unattested = False
admitsSpan KeywordGrant Unclaimed = False
admitsSpan KeywordGrant GrantsAndControl = True
admitsSpan KeywordGrant GrantsTypesControlReplacementAndPermission = True
admitsSpan KeywordGrant KeywordGrantOnly = True
admitsSpan KeywordGrant RestrictionsShieldsPermissionsAndDelays = False
admitsSpan KeywordGrant GrantsRestrictionsAndReplacement = True
admitsSpan KeywordGrant ControlGrantAndPermission = False
admitsSpan KeywordGrant GrantsRestrictionsControlAndPermission = True
admitsSpan KeywordGrant PermissionOnly = False
admitsSpan DeedRestriction Unattested = False
admitsSpan DeedRestriction Unclaimed = False
admitsSpan DeedRestriction GrantsAndControl = False
admitsSpan DeedRestriction GrantsTypesControlReplacementAndPermission = False
admitsSpan DeedRestriction KeywordGrantOnly = False
admitsSpan DeedRestriction RestrictionsShieldsPermissionsAndDelays = True
admitsSpan DeedRestriction GrantsRestrictionsAndReplacement = True
admitsSpan DeedRestriction ControlGrantAndPermission = False
admitsSpan DeedRestriction GrantsRestrictionsControlAndPermission = True
admitsSpan DeedRestriction PermissionOnly = False
admitsSpan TypeAddition Unattested = False
admitsSpan TypeAddition Unclaimed = False
admitsSpan TypeAddition GrantsAndControl = False
admitsSpan TypeAddition GrantsTypesControlReplacementAndPermission = True
admitsSpan TypeAddition KeywordGrantOnly = False
admitsSpan TypeAddition RestrictionsShieldsPermissionsAndDelays = False
admitsSpan TypeAddition GrantsRestrictionsAndReplacement = False
admitsSpan TypeAddition ControlGrantAndPermission = False
admitsSpan TypeAddition GrantsRestrictionsControlAndPermission = False
admitsSpan TypeAddition PermissionOnly = False
admitsSpan ControlGrant Unattested = False
admitsSpan ControlGrant Unclaimed = False
admitsSpan ControlGrant GrantsAndControl = True
admitsSpan ControlGrant GrantsTypesControlReplacementAndPermission = True
admitsSpan ControlGrant KeywordGrantOnly = False
admitsSpan ControlGrant RestrictionsShieldsPermissionsAndDelays = False
admitsSpan ControlGrant GrantsRestrictionsAndReplacement = False
admitsSpan ControlGrant ControlGrantAndPermission = True
admitsSpan ControlGrant GrantsRestrictionsControlAndPermission = True
admitsSpan ControlGrant PermissionOnly = False
admitsSpan Replacement Unattested = False
admitsSpan Replacement Unclaimed = False
admitsSpan Replacement GrantsAndControl = False
admitsSpan Replacement GrantsTypesControlReplacementAndPermission = True
admitsSpan Replacement KeywordGrantOnly = False
admitsSpan Replacement RestrictionsShieldsPermissionsAndDelays = True
admitsSpan Replacement GrantsRestrictionsAndReplacement = True
admitsSpan Replacement ControlGrantAndPermission = False
admitsSpan Replacement GrantsRestrictionsControlAndPermission = False
admitsSpan Replacement PermissionOnly = False
admitsSpan Prevention Unattested = False
admitsSpan Prevention Unclaimed = False
admitsSpan Prevention GrantsAndControl = False
admitsSpan Prevention GrantsTypesControlReplacementAndPermission = False
admitsSpan Prevention KeywordGrantOnly = False
admitsSpan Prevention RestrictionsShieldsPermissionsAndDelays = True
admitsSpan Prevention GrantsRestrictionsAndReplacement = False
admitsSpan Prevention ControlGrantAndPermission = False
admitsSpan Prevention GrantsRestrictionsControlAndPermission = False
admitsSpan Prevention PermissionOnly = False
admitsSpan Conditional Unattested = False
admitsSpan Conditional Unclaimed = False
admitsSpan Conditional GrantsAndControl = False
admitsSpan Conditional GrantsTypesControlReplacementAndPermission = False
admitsSpan Conditional KeywordGrantOnly = False
admitsSpan Conditional RestrictionsShieldsPermissionsAndDelays = False
admitsSpan Conditional GrantsRestrictionsAndReplacement = False
admitsSpan Conditional ControlGrantAndPermission = False
admitsSpan Conditional GrantsRestrictionsControlAndPermission = False
admitsSpan Conditional PermissionOnly = False
admitsSpan PlayPermission Unattested = False
admitsSpan PlayPermission Unclaimed = False
admitsSpan PlayPermission GrantsAndControl = False
admitsSpan PlayPermission GrantsTypesControlReplacementAndPermission = True
admitsSpan PlayPermission KeywordGrantOnly = False
admitsSpan PlayPermission RestrictionsShieldsPermissionsAndDelays = True
admitsSpan PlayPermission GrantsRestrictionsAndReplacement = False
admitsSpan PlayPermission ControlGrantAndPermission = True
admitsSpan PlayPermission GrantsRestrictionsControlAndPermission = True
admitsSpan PlayPermission PermissionOnly = True
admitsSpan EntryRider Unattested = False
admitsSpan EntryRider Unclaimed = False
admitsSpan EntryRider GrantsAndControl = False
admitsSpan EntryRider GrantsTypesControlReplacementAndPermission = False
admitsSpan EntryRider KeywordGrantOnly = False
admitsSpan EntryRider RestrictionsShieldsPermissionsAndDelays = False
admitsSpan EntryRider GrantsRestrictionsAndReplacement = False
admitsSpan EntryRider ControlGrantAndPermission = False
admitsSpan EntryRider GrantsRestrictionsControlAndPermission = False
admitsSpan EntryRider PermissionOnly = False
-- The cost statement writes NO adverbial, and the row is all False on a
-- measurement rather than on a principle. Of six hundred and fifty-three
-- supported cost-modification lines, exactly ONE carries a duration --
-- "Spells with the chosen name cost {1} less to cast this turn" (Cheering
-- Fanatic), which is unwritable for its chosen-name subject anyway -- and
-- flipping its cell would rename a `SpanUse` row that currently tells the
-- truth about which constructions write "this turn". So the one line is
-- recorded rather than admitted (ledger), and the two "during your turn"
-- lines are not durations at all but a window on when the reduction
-- applies.
admitsSpan CostModification Unattested = False
admitsSpan CostModification Unclaimed = False
admitsSpan CostModification GrantsAndControl = False
admitsSpan CostModification GrantsTypesControlReplacementAndPermission = False
admitsSpan CostModification KeywordGrantOnly = False
admitsSpan CostModification RestrictionsShieldsPermissionsAndDelays = False
admitsSpan CostModification GrantsRestrictionsAndReplacement = False
admitsSpan CostModification ControlGrantAndPermission = False
admitsSpan CostModification GrantsRestrictionsControlAndPermission = False
admitsSpan CostModification PermissionOnly = False

||| Whether a construction can write NO duration at all. A grant can: the
||| unwritten span is [CR#611.2a]'s end-of-game default, which the guide
||| permits where the effect is "intentionally indefinite under the
||| rules" (Through the Breach's bare "It gains haste."). A restriction
||| cannot — a durationless "can't" is the STATIC ability line
||| ("Enchanted creature can't attack", Pacifism), a different
||| construction and the parked ability layer's, so the whole clause is
||| unwritable here rather than the span being optional (`badStaticCant`).
|||
||| The type addition and the control grant answer `True`, and there the
||| unwritten span is the NORM rather than the exception: most "in
||| addition to its other types" lines and the bare "Gain control of
||| target creature." state no duration, with Memnarch and Phyrexian
||| Infiltrator printing [CR#611.2a]'s default out loud in reminder text —
||| "(This effect lasts indefinitely.)"
|||
||| Both SHIELD rows answer `False`, for the deed restriction's reason
||| exactly: a durationless interception or shield is the STATIC ABILITY
||| line and not a clause at all. [CR#603.6d] says so for the entry riders
||| in as many words and [CR#611.3] for the rest — a continuous effect
||| from a static ability carries no duration because it lasts while the
||| ability functions. The corpus divides on the same line: every one-shot
||| interception and every one-shot shield states a span, while the
||| durationless "Prevent all …" lines are static abilities to a line and
||| the standing interceptions are the permanent's own ability. So the
||| whole clause is unwritable here rather than the span being optional
||| (`badStandingIntercept`, `badStandingPrevention`).
public export
absentOk : StaticKind -> Bool
absentOk PtDelta = True
absentOk KeywordGrant = True
absentOk DeedRestriction = False
absentOk TypeAddition = True
absentOk ControlGrant = True
absentOk Replacement = False
absentOk Prevention = False
-- The three new rows answer `False` for the restriction's reason taken
-- one step further: they are not clause constructions at all. A
-- durationless "as long as" static, a durationless entry rider and a
-- durationless play permission are each the static ABILITY line and
-- nothing else, so there is no `Continuously` clause for the span to be
-- absent from (`badConditionalClause`, `badEntryRiderClause`,
-- `badStandingPermission`). The PERMISSION is the sharpest of the three
-- because it writes spans freely as a clause and states none at all only
-- when it is a card's own line ("You may cast this card from your
-- graveyard").
absentOk Conditional = False
absentOk PlayPermission = False
absentOk EntryRider = False
-- A fourth row with the same answer for the same reason: every cost
-- statement in the corpus is a static ability's own LINE ("Noncreature
-- spells cost {1} more to cast"), never a clause a resolution
-- establishes, so there is no `Continuously` for its span to be absent
-- from (`badCostClause`).
absentOk CostModification = False

||| Whether English writes this static effect as a bare ability LINE —
||| the OTHER reader of the same axis. [CR#604.1] says what the line is
||| ("static abilities … are written as statements, and they're simply
||| true") and [CR#611.3b] says why it states no duration (the continuous
||| effect "applies at all times that the permanent generating it is on
||| the battlefield"), so the question is only which of these effects
||| English is willing to state.
|||
||| Nine of the ten rows answer yes, and the one that says no says
||| something about ENGLISH rather than about the effect. The ability line
||| and the resolving clause differ in ASPECT, not in vocabulary: the
||| clause is an instruction and takes the inchoative verb, the line is a
||| statement and takes the stative one — "Target creature gains flying
||| until end of turn" against "Creatures you control have haste",
||| "becomes an Island in addition to its other types" against "are the
||| chosen type in addition to their other types". The stat delta and the
||| deed restriction write ONE word in both frames ("get", "can't")
||| because English has no separate inchoative for them, which is why the
||| split was invisible until a construction needed both.
|||
||| `ControlGrant` is the row that says no, and the reason is that the
||| stative form of "gain control of" is a different VERB. English writes
||| the standing fact as "You control enchanted creature" and writes
||| "gains control of" as a durationless line zero times, so the line is a
||| construction this row does not spell rather than an inflection of it
||| (`badStaticGainsControl`, ledger).
public export
staticAsAbility : StaticKind -> Bool
staticAsAbility PtDelta = True
staticAsAbility KeywordGrant = True
staticAsAbility DeedRestriction = True
staticAsAbility TypeAddition = True
staticAsAbility ControlGrant = False
staticAsAbility Replacement = True
staticAsAbility Prevention = True
staticAsAbility Conditional = True
staticAsAbility PlayPermission = True
staticAsAbility EntryRider = True
staticAsAbility CostModification = True

||| Which zones a play permission names as its source ([CR#604.6] — a
||| static ability of this shape "appl(ies) while a card is in any zone
||| that you could cast or play it from"). Full rows over the five zones
||| and the silence, because the permission's own phrase is what places
||| its card: the impulse family exiles first and then permits, the
||| graveyard family names the zone in the permission ("You may cast this
||| card from your graveyard"), the library family names the top, and the
||| hand is the rules' own default, named per card type ([CR#302.1] for a
||| creature, [CR#305.1] for a land). The BATTLEFIELD is the one refusal
||| and it is a rules fact rather than a count: a permanent on the
||| battlefield has already been played (`badPlayFromBattlefield`).
||| Silence passes for `zoneFits`' reason — an untracked referent asserts
||| nothing about where it is.
public export
playableFrom : Maybe Zone -> Bool
playableFrom Nothing = True
playableFrom (Just Battlefield) = False
playableFrom (Just Graveyard) = True
playableFrom (Just Exile) = True
playableFrom (Just Hand) = True
playableFrom (Just Library) = True
-- The STACK is the second refusal and it is the battlefield's reason
-- exactly one step earlier: [CR#112.1] makes an object there a spell,
-- and a spell has already been cast. [CR#113.6d,113.6e] are the near
-- miss and worth naming — an ability that modifies what its own object
-- costs to cast, or restricts how it can be cast, "functions on the
-- stack" — but that is a cost or a restriction functioning there, not a
-- permission to cast something already on it (`badPlayFromStack`).
playableFrom (Just Stack) = False

public export
data PlayableFrom : Maybe Zone -> Type where
  MkPlayableFrom : {auto 0 ok : playableFrom z = True} -> PlayableFrom z

||| The verb's demand on its complement, as a witness named apart so a
||| pin says which of the permission's two questions refused.
public export
data CastableTy : PlayVerb -> Maybe CardType -> Type where
  MkCastableTy : {auto 0 ok : castableTy v ty = True} -> CastableTy v ty

||| Which adverbial a DELAYED trigger states as its duration
||| ([CR#603.7b] — it "will trigger only once … unless it has a stated
||| duration, such as 'this turn'"). The rule names the phrase and the
||| corpus writes no other. Full rows over `SpanUse`, so a new adverbial
||| class declares this reader too, and every class but the current-turn
||| one is `False` — the delayed clause writes no boundary endpoint and no
||| for-as-long-as ([CR#611.2b]'s adverbial belongs to a continuous
||| effect, and a delayed trigger is not one).
public export
admitsDelaySpan : SpanUse -> Bool
admitsDelaySpan Unattested = False
admitsDelaySpan Unclaimed = False
admitsDelaySpan GrantsAndControl = False
admitsDelaySpan GrantsTypesControlReplacementAndPermission = False
admitsDelaySpan KeywordGrantOnly = False
admitsDelaySpan RestrictionsShieldsPermissionsAndDelays = True
admitsDelaySpan GrantsRestrictionsAndReplacement = False
admitsDelaySpan ControlGrantAndPermission = False
admitsDelaySpan GrantsRestrictionsControlAndPermission = False
admitsDelaySpan PermissionOnly = False

||| How an indefinite phrase MARKS its choice method — the axis three
||| constructors used to spell three times. Choice method is surface data
||| (finding 11): no CR rule derives a chooser, so what the grammar
||| records is what the text writes, and the phrase itself is one
||| determiner throughout ("a …", article and all).
|||
||| Core keeps the same axis off the choice itself: `Binder::ChooseOne`
||| carries the filter and a separate `by` slot naming who chooses,
||| defaulting to the controller and OVERRIDDEN for the foreign chooser
||| ("that player sacrifices a creature of their choice",
||| [CR#608.2d,701.21a], `binder.rs`), while the chooserless form is a
||| different constructor entirely (`Selection::Random`, `selection.rs`).
||| `Unmarked` is core's elided `by`, `TheirChoice` its override,
||| `AtRandom` its random sibling.
|||
||| `TheirChoice` carries the pronoun's own obligation: "their" is a
||| POSSESSIVE, so it needs exactly one player antecedent — a singular
||| subject or one distributive group (`countChoosers`,
||| `badUnboundTheirChoice`). A NOMINAL chooser slot mirroring core's
||| `by: Reference` waits on the plural player read the distributive
||| antecedent would need (ledger).
public export
-- spelling: (construction-owned catalog -- each row is the ADVERBIAL an
-- indefinite phrase writes after its noun, and each is spelled through
-- its own macro: Unmarked = `a` (no adverbial), TheirChoice =
-- `aTheirChoice`'s "of their choice", AtRandom = `aAtRandom`'s "at
-- random". Never spelled alone -- see Experimental.Macros)
data ChoiceMode : Bindings -> Type where
  Unmarked : ChoiceMode bs
  TheirChoice : {auto 0 ch : countChoosers bs = 1} -> ChoiceMode bs
  AtRandom : ChoiceMode bs

-- ===== The grammar (mutual: types thread contexts through VALUES) =====

||| WHICH zones English writes a possessor for — the closed table behind
||| the owned zone phrase, and [CR#400.1] is the whole of it: "Each player
||| has their own library, hand, and graveyard. The other zones are shared
||| by all players." So "your hand" and "an opponent's graveyard" are
||| phrases and "your battlefield" is not, and the reason is a fact about
||| the ZONE rather than about the phrase that names it. A new shared zone
||| declares its absence by having no row to write.
public export
data Possessable : Zone -> Type where
  HandIsOwned : Possessable Hand
  GraveyardIsOwned : Possessable Graveyard
  LibraryIsOwned : Possessable Library

mutual
  ||| Whether a zone phrase writes a possessor, and who: the two axes
  ||| a zone expression carries, the sort and the scope, kept apart
  ||| here as core keeps them apart — the zone is a bare `Zone`
  ||| everywhere it is named (`StatePredicate::InZone(Zone)`,
  ||| `Destination::Zone(Zone)`) and whose it is, when that matters, is
  ||| a SEPARATE relation beside it (`RelationPredicate::Owner`,
  ||| `filter.rs`). `Bare` is the phrase with no possessive written: on
  ||| a shared zone that is the only form there is, and on a per-player
  ||| zone it is the sort-only form macro expansions need — the CR
  ||| routes those per-object, e.g. destroy's "its owner's graveyard"
  ||| ([CR#701.8a]) without the card text mentioning the owner, and an
  ||| owned expansion form would inject a phantom mention into the
  ||| discourse.
  public export
  data ZoneScope : Bindings -> Zone -> Type where
    -- spelling: (construction-owned -- the absence of a possessive; the
    -- zone word stands alone, e.g. "the battlefield", "exile", and the
    -- sort-only "hand"/"graveyard" of the macro expansions)
    Bare : ZoneScope bs z
    -- the possessor answers `Possessor`, which is the CONTROL clause's
    -- own table read a second time: [CR#108.3] gives a card one owner as
    -- [CR#109.4] gives an object one controller, so "cards in your
    -- opponents' graveyards" names as many zones as there are members and
    -- asks the same membership question of each card. Seven supported
    -- lines write it (graveyards 6, hands 1) and none of them is writable
    -- for other reasons -- every one scales a P/T by a count -- so the
    -- cell is minted on the table's symmetry with its bench recorded as
    -- owed. The plural RELATIONAL possessor ("their owners' hands") is a
    -- different family and still ledgered. The zone must be one a player
    -- has ([CR#400.1], `Possessable`).
    -- spelling: (construction-owned -- the possessive premodifier
    -- "<Param(0)>'s" before the zone word, e.g. "your hand", "its
    -- owner's hand"; a plural possessor pluralises the zone word too,
    -- "your opponents' graveyards")
    OwnedBy : (n : Noun bs Player) -> {auto 0 ps : Possessable z} ->
              {auto 0 pn : Possessor n} -> ZoneScope bs z

  ||| A zone as English writes it: the zone it names and the scope it
  ||| writes over it. One constructor, because a zone phrase is one
  ||| construction — the six spellings this file had were the two axes
  ||| multiplied out, and they are macros now (`battlefieldZ`,
  ||| `exileZ`, `handZ`, `graveyardZ`, `handOf`, `graveyardOf`).
  ||| Mentions inside a destination expression do not yet enter the
  ||| discourse (no current positive writes one).
  public export
  data ZoneExpr : Bindings -> Type where
    -- spelling: (construction-owned -- the zone word under whatever
    -- possessive its scope writes; the macros own the six surfaces --
    -- see Experimental.Macros), kind: Nominal
    ZoneAt : (z : Zone) -> ZoneScope bs z -> ZoneExpr bs
    -- the library AT a position ([CR#401.2] -- the one ordered zone), with
    -- the order rider a plural arrival takes ([CR#401.4]). A separate row
    -- rather than a `Maybe LibPos` on `ZoneAt`, and for core's own reason:
    -- a bare library is not a destination at all (`DestOk` has no row for
    -- `ZoneAt Library _`, which is core's `exclude(Library)` on
    -- `Destination` exactly), so the positioned form is the SINGLE
    -- canonical spelling. `ZoneAt Library` remains the zone as a WHOLE --
    -- what a search looks through and a shuffle randomizes.
    -- spelling: (construction-owned -- "on top of <scope> library" /
    -- "on the bottom of <scope> library", plus the order rider when
    -- present: "… in any order" / "… in a random order"; the macros own
    -- the surfaces -- see Experimental.Macros), kind: Nominal
    LibraryAt : (pos : LibPos) -> (ord : Maybe Arrangement) ->
                ZoneScope bs Library -> ZoneExpr bs

  ||| The sort a zone expression names — what fold-state records. The
  ||| scope is no part of it: whose hand a card is in does not change
  ||| that it is in a hand. Nor is the POSITION: a card put on the bottom
  ||| of a library is in a library.
  public export
  zoneSort : ZoneExpr bs -> Zone
  zoneSort (ZoneAt z _) = z
  zoneSort (LibraryAt _ _ _) = Library

  ||| Does this zone phrase name the zone AS A WHOLE? The sort does not
  ||| answer it — `zoneSort` reads `Library` off "the top of your library"
  ||| as readily as off "your library" — and one clause needs the
  ||| difference. [CR#701.23a] defines searching as looking at "all cards
  ||| in that zone", so a search takes the pile and never a place in it;
  ||| [CR#401.2] is why the two can be told apart at all, a library being
  ||| "a single face-down pile" whose positions are not zones of their own.
  public export
  wholeZone : ZoneExpr bs -> Bool
  wholeZone (ZoneAt _ _) = True
  wholeZone (LibraryAt _ _ _) = False

  ||| The whole-zone demand as a witness.
  public export
  data WholeZone : ZoneExpr bs -> Type where
    MkWholeZone : {auto 0 ok : wholeZone z = True} -> WholeZone z

  ||| The order rider a zone phrase writes, if it can carry one — the
  ||| ordered library's alone ([CR#401.4] speaks of a POSITION in a
  ||| library and nothing else). What `Move` consults to ask its patient
  ||| for a plural.
  public export
  zoneArrangement : ZoneExpr bs -> Maybe Arrangement
  zoneArrangement (ZoneAt _ _) = Nothing
  zoneArrangement (LibraryAt _ ord _) = ord

  ||| An object/player criteria set — the noun phrase's modifier list,
  ||| FLAT: head noun and relative clauses are sibling constraints on one
  ||| referent, exactly as parsed (no rearrangement to figure out).
  public export
  data Predicate : Bindings -> Kind -> Type where
    -- spelling: ["<Param(0)>"] (Param(0) = CardType's own word -- see
    -- CardType), kind: Nominal (hasHead = True)
    HasType : CardType -> Predicate bs Object            -- head noun "creature"/…
    -- the SUBTYPE word as a head noun ("an Army you control",
    -- [CR#701.47a]; "a Demon"): a different axis from the card-type word
    -- and kept apart from it in both projections. It PRESUPPOSES the card
    -- type whose set the subtype belongs to ([CR#205.1a], `subtypeType`) —
    -- an Army is a creature — and it PROJECTS no card type at all, which
    -- is not a hedge but the difference between the axes: `negTypesOf`
    -- reads a negated member's projected head, so projecting Creature here
    -- would make "that isn't a Demon" mean "noncreature" and refuse
    -- "target attacking Vampire that isn't a Demon" (Clavileño), which is
    -- ordinary oracle. The presupposition still bites where it should
    -- (`badZombieNoncreature`).
    -- spelling: ["<Param(0)>"] (Param(0) = Subtype's own word -- see Subtype),
    -- kind: Nominal (hasHead = True)
    HasSubtype : Subtype -> Predicate bs Object
    -- spelling: ["player"], kind: Nominal (hasHead = True)
    AnyPlayer : Predicate bs Player                      -- head noun "player" (any player, [CR#102.1])
    -- spelling: ["opponent"], kind: Nominal (hasHead = True)
    Opponent : Predicate bs Player                       -- head noun "opponent" (of You — team form [CR#102.3] deferred)
    -- head noun "color" / "creature type" — the choosable quality
    -- ([CR#105.1,302.3]).
    -- spelling: ["<Param(0)>"] (Param(0) = QualitySort's own word -- see
    -- QualitySort), kind: Nominal (hasHead = True)
    QualityNoun : (q : QualitySort) -> Predicate bs (Quality q)
    -- "of the chosen [quality]": reads the unique chosen quality (the
    -- guide's stored-quality naming; choice made at resolution
    -- [CR#608.2d]). The chosen-OBJECT twin ("the chosen creatures")
    -- waits with the definite reads.
    -- spelling: ["of the chosen <Param(0)>"], kind: TODO(reason: non-head
    -- modifier per hasHead -- not a complete Nominal alone). CardName is
    -- later read as "with the chosen name"/"with that name", and Number by
    -- numeric equality, not "of the chosen number".
    OfChosen : (q : QualitySort) -> {auto 0 ok : countQuality q bs = 1} ->
               {auto 0 read : ChosenQualityRead q} -> Predicate bs Object
    -- "with [keyword]" / "that has [keyword]" ([CR#702.1d]): a non-head
    -- modifier. It has no type, zone, or head projection/presupposition.
    HasKeyword : Keyword -> Predicate bs Object
    -- zero relative "[player] controls" -- and what it asks is MEMBERSHIP:
    -- [CR#109.4] gives an object one controller, so the clause tests
    -- whether that one player is the possessor named, and a plural
    -- possessor asks whether it is one OF them. "Creatures your opponents
    -- control" is 377 supported cards and the phrase the CR writes into
    -- hexproof's own definition ([CR#702.11b]); the Rust core spells it as
    -- one composition over a player predicate,
    -- `ControlledBy(OpponentOf(Ref(You)))` (`filter.rs`, `deontic.rs`),
    -- which is this row with the set named a second way. WHICH possessors
    -- may stand here is `Possessor`'s measured table, and the singular
    -- ones are unchanged: the widening admits one word, not plurality
    -- (`badControlledByGroup` still refuses two target opponents).
    -- spelling: ["<Param(0)> control"] (auto-inflection covers "controls";
    -- the plural possessor takes the plural verb, "your opponents
    -- control"), kind: TODO(reason: non-head relative-clause modifier per
    -- hasHead)
    ControlledBy : (n : Noun bs Player) -> {auto 0 ps : Possessor n} -> Predicate bs Object
    -- zero relative "[player] cast(s)" -- the CASTING relation, and a row
    -- of its own rather than a verb the row above could spell. The player
    -- the two clauses name is the same one: [CR#601.2a] makes the caster
    -- the controller in as many words ("that player becomes its
    -- controller"), and the glossary retires the noun outright ("Caster
    -- (Obsolete) ... cards that were printed with the term 'caster' have
    -- received errata in the Oracle card reference to say 'controller'").
    -- What the two clauses do NOT name is the same class of spell.
    -- [CR#707.10] puts a copy on the stack uncast -- "a copy of a spell
    -- isn't cast ... A copy of a spell is controlled by the player under
    -- whose control it was put on the stack" -- so the controlled set is
    -- strictly the larger, and Errant, Street Artist writes both words in
    -- one phrase to say so ("Copy target spell you control that wasn't
    -- cast"). The corpus keeps them apart cell by cell: the cost statement
    -- writes "you cast" on 153 lines and "you control" on none, the
    -- counter-proof writes "you control" on 15 and "you cast" on none, and
    -- the keyword grants split by regime -- the casting-time keywords
    -- (convoke, cascade, replicate) take "cast" and the resolution-time
    -- ones (lifelink, deathtouch) take "control".
    -- The relation PLACES its referent, where the controller relation only
    -- admits it: casting moves the card to the stack ([CR#601.2a]) and it
    -- is a spell only there ([CR#112.1]), which is what `seedZone` says.
    -- The zone a spell was cast FROM is a different axis and unbuilt
    -- ("spells you cast from your graveyard", 24 lines), and the one line
    -- describing a battlefield permanent by its casting ("target creature
    -- you cast this turn", Cycle of Life) marks a LOOKBACK and belongs to
    -- the history vocabulary; 310 of the family's 311 lines write "spell".
    -- WHICH possessors may stand here is `Possessor`'s table at its THIRD
    -- reader, measured before it was shared: "your opponents cast" is 27
    -- lines, the singular definite read 5, the bare plural zero
    -- (`badCastByAllPlayers`).
    -- spelling: ["<Param(0)> cast(s)"] (auto-inflection covers "casts";
    -- the plural possessor takes the plural verb, "your opponents cast"),
    -- kind: TODO(reason: non-head relative-clause modifier per hasHead)
    CastBy : (n : Noun bs Player) -> {auto 0 ps : Possessor n} -> Predicate bs Object
    -- the attacking-designation modifier ([CR#508.1a]) — a
    -- battlefield state word, not a type.
    -- spelling: ["attacking"], kind: TODO(reason: non-head status modifier
    -- per hasHead)
    Attacking : Predicate bs Object
    -- the blocking-designation modifier ([CR#509.1a]) — the defending
    -- player's twin of `Attacking`, and the word the corpus coordinates
    -- with it ("target attacking or blocking creature").
    -- spelling: ["blocking"], kind: TODO(reason: non-head status modifier
    -- per hasHead)
    Blocking : Predicate bs Object
    -- "blocking [m]" — the block RELATION as a description, the
    -- designation word above with the thing blocked said out loud.
    -- [CR#509.1g] is the warrant and the reason it is a second row rather
    -- than an argument on `Blocking`: the rule gives the blocker two
    -- separate facts, that it "becomes a blocking creature" and that it
    -- "is blocking the attacking creatures chosen for it", and the corpus
    -- writes them in different phrases — "target attacking or blocking
    -- creature" coordinates the bare designation with `Attacking` and
    -- takes no complement, while "target creature blocking this creature"
    -- names the relatum and never coordinates. Two words, two rows.
    -- (The alternative considered and declined: one row with a
    -- `Maybe`-valued relatum. It would have made every existing
    -- "attacking or blocking" site carry a `Nothing`, and the
    -- coordination that motivates the bare row is exactly what the
    -- optional slot cannot express.)
    -- The RELATUM is an ordinary object noun standing on the battlefield:
    -- [CR#506.3] admits only creatures to the relation at all — the type
    -- presupposition `seedType` carries — and a blocking relation holds
    -- between two objects in combat, so a phrase naming another zone
    -- contradicts (`zoneFits`' silence-passing discipline,
    -- `badBlockingGraveyardRelatum`).
    -- spelling: ["blocking <Param(0)>"], kind: TODO(reason: non-head
    -- postnominal participial relative per hasHead -- "creatures blocking
    -- this creature", never a phrase's head)
    BlockerOf : (m : Noun bs Object) ->
                {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                Predicate bs Object
    -- "blocked by [m]" — the same relation read from the attacker's side,
    -- and [CR#509.1h]'s own wording ("an attacking creature with one or
    -- more creatures declared as blockers for it becomes a blocked
    -- creature"). `ControlledBy`'s naming and `ControlledBy`'s shape: a
    -- passive participle taking the relatum after "by".
    -- The pair is written coordinated more often than either is written
    -- alone — "all creatures blocking or blocked by this creature" is the
    -- end-of-combat body's standing phrase — which is an ordinary
    -- disjunction over two rows and needs nothing else. In particular it
    -- needs no new GROUP MENTION kind: the earlier claim that this family
    -- wanted a third `groupMention` row beside `TargetGroup` and
    -- `LibrarySlice` was mistaken, `AllOf`/`Each` over a disjunction of
    -- these two predicates being the whole of what "all creatures
    -- blocking or blocked by this creature" needs.
    -- No SINGULARITY demand on either row, and the rules are why in both
    -- directions: an attacker may be blocked by many ([CR#509.1h] — "one
    -- or more creatures declared as blockers for it"), and a blocker may
    -- be blocking more than one ([CR#509.1g] — "the attacking creatures
    -- chosen for it", plural). `ControlledBy`'s one-possessor gate is
    -- [CR#109.4]'s fact about control and does not transfer.
    -- spelling: ["blocked by <Param(0)>"], kind: TODO(reason: non-head
    -- postnominal participial relative per hasHead)
    BlockedBy : (m : Noun bs Object) ->
                {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                Predicate bs Object
    -- "that [past-verb] [w]" / "who [past-verb] [w]" -- the history read
    -- in the DESCRIPTION, and the second of the two surfaces finding 296
    -- said were one query. `Happened` asks whether an event occurred;
    -- this asks it of the phrase's own referent, so the head noun IS the
    -- subject and the row takes no subject argument at all -- its kind
    -- index is the subject sort, and the SAME `lookbackSubjectOk` table
    -- gates it. One query, two readers, one attestation table, which is
    -- what the merged queue entry claimed and this row is the test of.
    -- `BlockerOf`'s and `BlockedBy`'s shape two rows up: a postnominal
    -- participial relative that heads nothing and modifies the noun in
    -- front of it.
    -- THE SEEDING IS THE ROW'S CENTRAL FACT and it is a silence, not a
    -- battlefield: [CR#608.2i] says the objects a history read names
    -- "don't need to be currently in the zone they were in at the time of
    -- that previous game state or action", so the read may not place its
    -- referent anywhere. The corpus proves it in one line -- Continue?'s
    -- "creature cards in your graveyard that were put there from the
    -- battlefield this turn", where the phrase's own zone clause says
    -- GRAVEYARD and the event happened on the battlefield -- which a
    -- battlefield seed would have refused through `zonesOk`. (That line is
    -- the seeding's proof and not a witness: its surface is the
    -- zone-change wording rather than a past verb this row spells, so it
    -- stays a near-miss.) `seedType` is silent for the ordinary reason:
    -- the head noun carries the type and this modifier presupposes none.
    -- spelling: ["that <past verb for Param(0)> <Param(1)>" (an object
    -- head), "who <past verb for Param(0)> <Param(1)>" (a player head)]
    -- (the RELATIVIZER agrees with the head's sort, which is finding 275's
    -- lesson a third time -- an axis that varies perfectly with something
    -- already written is a spelling fact; the past-verb table and its
    -- voice split are `Happened`'s own and shared with it), kind:
    -- TODO(reason: non-head postnominal participial relative per hasHead)
    HappenedTo : {k : Kind} -> (ev : EventName) -> (w : Lookback) ->
                 {auto 0 sb : LookbackSubject ev k} -> Predicate bs k
    -- "[color] [head]" -- an object's COLOR as a description, and the
    -- oldest unaddressed gap in this vocabulary rather than a deferred
    -- one: finding 77 ported `Chroma.Color` from core for TOKEN minting
    -- and said so, excluding the mana channel and never touching the
    -- description reading, so no ruling ever declined this row. [CR#105.1]
    -- closes the set at five and [CR#105.2] gives the reading -- "an
    -- object can be one or more of the five colors" -- which is also why
    -- the row stacks rather than clashes with itself: a gold card is
    -- black AND red, so two color words in one phrase agree.
    -- It PRESUPPOSES NOTHING. Colors belong to objects in every zone and
    -- to spells on the stack ([CR#105.2] reads them off the mana cost), so
    -- neither a zone nor a type seed may ride here -- "target black card
    -- in a graveyard" and "target red spell" are both ordinary oracle.
    -- The NEGATION is written as a PREFIX and this row owns that surface:
    -- "nonblack" 77 lines, nonwhite 15, nonblue 5, nonred 4, nongreen 9,
    -- against zero writing "not black". `IsToken`'s arrangement with
    -- "nontoken", one row up in spirit.
    -- spelling: ["<Param(0)>"] (the color's own word, prenominal --
    -- see Chroma.Color; negated, the prefix form "non<Param(0)>"),
    -- kind: TODO(reason: non-head prenominal modifier per hasHead)
    ColorIs : (c : Chroma.Color) -> Predicate bs Object
    -- "colorless [head]" (71 description lines) -- and NOT a sixth value
    -- of the row above, because [CR#105.2c] says what it is in four
    -- words: "a colorless object has no color". [CR#105.4] repeats it for
    -- the choosing rule ("'Multicolored' is not a color. Neither is
    -- 'colorless'"). Core splits them the same way and for the same
    -- reason. Named `IsColorless` because `Chroma.ColorOrColorless`
    -- already owns the bare word in the mana channel -- the same one
    -- English, two grammatical objects arrangement `Lookback` needed.
    -- It CLASHES with the row above, which is the one contradiction this
    -- round's rows can raise: "colorless white creature" describes
    -- nothing (`colorClashOf`, `badColorlessWhite`).
    -- spelling: ["colorless"], kind: TODO(reason: non-head prenominal
    -- modifier per hasHead)
    IsColorless : Predicate bs Object
    -- "multicolored [head]" (65 lines) -- [CR#105.2b], "two or more of the
    -- five colors".
    -- spelling: ["multicolored"], kind: TODO(reason: non-head prenominal
    -- modifier per hasHead)
    Multicolored : Predicate bs Object
    -- "monocolored [head]" (15 lines) -- [CR#105.2a], "exactly one of the
    -- five colors". A recorded DIVERGENCE from core, which carries
    -- `Multicolored` and `Colorless` and no monocolored variant: core can
    -- reach it by counting colors and this vocabulary has no color-count
    -- term, so omitting the row would have left fifteen attested lines
    -- unwritable to buy a parity that costs the corpus. English writes two
    -- words where a count would have written one bound, and the two words
    -- are two rows.
    -- spelling: ["monocolored"], kind: TODO(reason: non-head prenominal
    -- modifier per hasHead)
    Monocolored : Predicate bs Object
    -- "legendary/basic/snow [head]" -- the SUPERTYPE as a description
    -- ([CR#205.4]), legendary 181 lines, basic 383, snow 67. Chapter
    -- thirty's `Supertype` was minted for the printed card's own field and
    -- ledgered as "a third list neither reader witnesses"; this is the
    -- third reader arriving, so the vocabulary moved up beside the other
    -- closed catalogs and the card frame keeps its list.
    -- Negated by prefix like the color row: "nonlegendary" 43 lines,
    -- "nonbasic" 68 -- and the second of those is chapter thirty-nine's
    -- named gap, Winter Moon's fifth set word ("Players can't untap more
    -- than one nonbasic land during their untap steps").
    -- World and Ongoing stay unminted, chapter thirty's own note.
    -- spelling: ["<Param(0)>"] (the supertype's own word, prenominal --
    -- see Supertype; negated, the prefix form "non<Param(0)>"),
    -- kind: TODO(reason: non-head prenominal modifier per hasHead)
    HasSupertype : (s : Supertype) -> Predicate bs Object
    -- "named [name]" -- the object's NAME as a description (133 lines),
    -- core's `Named(Ident)` at the same address. [CR#201.2a] is the
    -- equality rule the row leans on: "two or more objects have the same
    -- name if they have at least one name in common".
    -- THE STRING IS A PAYLOAD AND NOT A GATE, which is the repo's standing
    -- validator ruling and the reason this row is written the way it is:
    -- no auto-implicit anywhere may branch on what the name IS. `predEq`
    -- COMPARES two names, which is a different thing and is exactly what
    -- [CR#201.2a] makes rules-real -- it asks whether two phrases name the
    -- same card, never whether a phrase names a particular one -- so the
    -- contradiction scan can see "creature named Forest that isn't named
    -- Forest" without any gate ever reading a string's content.
    -- spelling: ["named <Param(0)>"], kind: TODO(reason: non-head
    -- postnominal modifier per hasHead)
    Named : (name : String) -> Predicate bs Object
    -- "the monarch" / "the initiative" -- a player DESIGNATION as a
    -- description, and the route the intervening-if slot already needed:
    -- "if you're the monarch" is 21 lines and is `Matches You` over this
    -- row, so the 18-line intervening family (12 of them excluding the
    -- absence checks) costs no condition vocabulary of its own.
    -- The ABSENCE check is NOT this row. "There is no monarch" is five
    -- lines and asks whether ANY player holds the designation, which is an
    -- existential over the holder rather than a description of one;
    -- recorded, not minted.
    -- Seeds nothing: a player has no zone and no card type.
    -- Negation measures ZERO ("isn't the monarch" and every sibling), so
    -- the row refuses it.
    -- spelling: ["<Param(0)>"] (the designation's own definite phrase --
    -- see PlayerDesignation -- predicative after the copula, "you're the
    -- monarch"), kind: TODO(reason: non-head definite description per
    -- hasHead)
    HasPlayerDesignation : (d : PlayerDesignation) -> Predicate bs Player
    -- "goaded [head]" (7 description lines) -- [CR#701.15b]'s designation
    -- read prenominally, the participle beside `Attacking` and `Blocking`
    -- and seeding what they seed: only a creature is goaded, and only on
    -- the battlefield.
    -- spelling: ["goaded"], kind: TODO(reason: non-head prenominal
    -- participle per hasHead)
    IsGoaded : Predicate bs Object
    -- "your Ring-bearer" -- and a COMPOUND wearing one phrase, which is
    -- why it is a row rather than a composition. [CR#701.54e] spells the
    -- condition out: a creature "is your Ring-bearer" exactly when it "is
    -- on the battlefield under your control and has the Ring-bearer
    -- designation" -- three conjuncts, of which the corpus writes NONE
    -- separately. All three attested lines write the four words whole, so
    -- composing `ControlledBy You` with a designation read would have
    -- spelled a phrase no card prints while leaving the printed one
    -- unwritable. The conjuncts live in this comment, where they belong.
    -- spelling: ["your Ring-bearer"] (predicative after the copula in
    -- Matches, "[n] is your Ring-bearer"; never bare and never
    -- third-person possessed), kind: TODO(reason: non-head possessed
    -- description per hasHead)
    YourRingBearer : Predicate bs Object
    -- "is enchanted" / "is equipped" -- the INVERSE direction: not what a
    -- permanent is attached TO but whether something has an attachment on
    -- it. Nineteen "is equipped" lines (Enkira) and fourteen "is
    -- enchanted", written predicatively after the copula and never
    -- prenominally, which is why they are conditions' vocabulary rather
    -- than a determiner's.
    -- Seeds the battlefield and nothing else: an attachment is attached to
    -- a permanent there ([CR#303.4b,301.5a]), and the head noun carries
    -- the type.
    -- Negation is written -- four isn't-forms -- so both rows take it.
    -- A THIRD direction exists and is NOT here: "is attached to" written
    -- of the ATTACHMENT itself ("as long as this Equipment is attached to
    -- a creature"), twelve lines, which names the relation rather than
    -- either end's participle. Recorded.
    -- spelling: ["enchanted", "equipped"] (predicative after the copula in
    -- Matches, "[n] is equipped"), kind: TODO(reason: non-head predicative
    -- participle per hasHead)
    IsEnchanted : Predicate bs Object
    IsEquipped : Predicate bs Object
    -- "permanent" / "permanent card" / "permanent spell" -- ONE row for
    -- the [CR#110.4a,110.4b] type-set, and the phrase's own zone story
    -- picks the carrier exactly as it does for a type word: bare, the
    -- description defaults to the battlefield ([CR#109.2]) and names
    -- [CR#110.1]'s permanent; under a card-zone clause it is the
    -- "permanent card" Eureka moves; under the stack it is the "permanent
    -- spell". It projects NO single type (`seedTy` stays silent --
    -- [CR#110.4] gives six permanent types and the word fixes none) and no
    -- zone of its own, which is what keeps Aether Helix's two sentences
    -- one vocabulary. The type-set itself is `permanentType`; this row
    -- gives it the noun surface, and the contradiction scan reads it
    -- (`badPermanentInstant`). Battle and planeswalker are outside the
    -- six-row CardType and therefore outside the writable set (ledger).
    -- spelling: ["permanent"] (battlefield), ["permanent card"] (card
    -- zones), ["permanent spell"] (stack) -- the carrier composition
    -- HasType already makes
    Permanent : Predicate bs Object
    -- "token" as a HEAD and "nontoken" as its negation. Seeds the
    -- battlefield: a token elsewhere has ceased to exist ([CR#111.7]).
    -- The CREATION compound stays `TokenChars`' surface ("create a 0/0
    -- green and blue Fractal creature token"), and the ordering is the
    -- closed fact: type-before-token throughout, token-before-type
    -- ("token creature") written zero times.
    -- spelling: ["token"]; negated, prenominal ["nontoken"]
    IsToken : Predicate bs Object
    -- "tapped"/"untapped"/"face-down" … -- the status word, [CR#110.5]'s
    -- value in an ordinary description. Non-head (a bare "choose a tapped"
    -- is unwritable), type-neutral ([CR#110.5] holds of every permanent),
    -- and battlefield-seeding ([CR#110.5d] -- a card in a graveyard is
    -- neither tapped nor untapped, `badTappedGraveyard`). Which values the
    -- surface writes at all is `statusWordOk`'s measured answer.
    -- Same-category values contradict and cross-category values stack
    -- (`statusClash`, `badTappedUntapped`). The paired values are words,
    -- not negations ("untapped" is never spelled "nontapped"), so the row
    -- is not negatable.
    -- spelling: (the value's word -- see StatusVal; prenominal in a noun
    -- phrase, predicative after the copula in Matches)
    HasStatus : {c : StatusCat} -> (v : StatusVal c) ->
                {auto 0 at : StatusWord v} -> Predicate bs Object
    -- "with [characteristic] [n] or less/greater" / "with power greater
    -- than the number of cards in your hand" — a BOUND on one of the
    -- object's own numbers ([CR#208.1] power and toughness, [CR#202.3]
    -- mana value), and core's
    -- `CharacteristicPredicate::Stat(Stat, Cmp, Count)` in the same
    -- three parts (`filter.rs`, whose own example is
    -- `Stat(Power, AtLeast, 3)`). The standard may be written or read
    -- (`ComparableBound`), and the vs-READ half is where this frame's
    -- weight actually lies: seventy-seven supported lines bound a power,
    -- toughness or mana value by another read against the numeral
    -- family's own thousands.
    -- spelling: (construction-owned -- the postnominal qualifier, whose
    -- word ORDER is the bound's class: "with <Param(0)> <Param(1)>
    -- <Param(2)>" against a written bound ("with power 2 or less") and
    -- "with <Param(0)> <Param(2)> <Param(1)>" against a read ("with power
    -- greater than the number of cards in your hand"); the english crate
    -- cuts the same surface in two,
    -- `QuantityRepr::OrComparison(value, word)` beside
    -- `ComparisonComplement { Than | ThanOrEqualTo, standard }`
    -- (syntax/phrase.rs), and chapter fifty-seven's finding is that the
    -- two are one relation spelled twice), kind: TODO(reason: non-head
    -- postnominal qualifier per hasHead)
    Compare : (c : Characteristic) -> (r : Comparator) -> (bound : Amount bs) ->
              {auto 0 cb : ComparableBound bound} -> Predicate bs Object
    -- spelling: ["in <Param(0)>"] (also "from <Param(0)>", see comment),
    -- kind: Nominal (hasHead = True; implicit head is the zone's carrier,
    -- e.g. "a card in your hand")
    InZone : ZoneExpr bs -> Predicate bs Object          -- zone clause "in/from [zone]" ([CR#109.2a])
    -- "exiled with [this object]" -- the LINKAGE read, and the first
    -- phrase in this file that reaches a group no sentence in its own
    -- ability assembled. [CR#406.6] states it in the exile chapter and
    -- [CR#607.2a] again in the linked-abilities one: an object with an
    -- ability that exiles cards and an ability that refers to cards
    -- "exiled with [this object]" has the two LINKED, and the second
    -- "refers only to cards that have been exiled due to the first". So
    -- the group is source-keyed rather than discourse-keyed, which is what
    -- `TheVerbed` could never have been -- a stamp lives in the bindings
    -- one clause hands the next, and this survives the sentence, the
    -- ability, and the turn.
    --
    -- A HEAD-bearing zone modifier, exactly as `InZone` is and for the
    -- same reason: the phrase carries the carrier noun "card" and places
    -- its referent in exile ([CR#406.2]), so "all cards exiled with this
    -- artifact" is a determiner over this predicate and needs no
    -- card-headed word of its own -- which is what makes the whole family
    -- cheap, every corpus spelling being an ordinary determiner over an
    -- ordinary modifier.
    --
    -- The SOURCE is the object whose abilities are linked and nothing
    -- else, which is [CR#607.1]'s "printed on it" read as a gate
    -- (`LinkSource`; `badExiledWithOtherSource`). Oracle writes it as the
    -- self-word ("this artifact", "it") or, on older cards, as the printed
    -- name -- one referent under three spellings, and finding 188 already
    -- settled that the name is not read back.
    -- spelling: ["exiled with <Param(0)>"] (Param(0) = the source noun,
    -- written "this <permanent word>" in modern templating and as the
    -- card's own name on older printings), kind: Nominal (hasHead = True;
    -- implicit head is the exile zone's carrier "card", as InZone)
    ExiledWith : (src : Noun bs Object) ->
                 {auto 0 ls : LinkSource src} -> Predicate bs Object
    -- sibling modifiers, one referent. The conjunction is where the
    -- phrase-level obligations live, and they are listed here in
    -- DECLARATION order: explicit zones must agree and may not
    -- contradict the phrase's own default (`ZoneCoherent`), no member
    -- may negate a sibling or a type a sibling presupposes
    -- (`ContradictionFree`), an "other" needs a head-compatible anchor
    -- and fills its one slot at most once (`OtherAnchored`), the
    -- class word "any target" takes no modifiers but "other" and is
    -- itself written exactly once (`AnyTargetLone`), and a phrase puts
    -- a bound on a characteristic at most once (`LoneComparison`).
    -- spelling: (construction-owned -- flat modifier-list juxtaposition, not
    -- itself a word; kind follows whether a member hasHead)
    And : (ps : List (Predicate bs k)) -> {auto 0 zc : ZoneCoherent ps} ->
          {auto 0 cf : ContradictionFree ps} -> {auto 0 oa : OtherAnchored ps} ->
          {auto 0 at : AnyTargetLone ps} -> {auto 0 lc : LoneComparison ps} ->
          Predicate bs k
    -- sibling ALTERNATIVES, still one referent. Where a conjunction's
    -- members all describe the same object at once, a disjunction's
    -- describe it in place of one another: "Destroy target artifact or
    -- enchantment." (Disenchant) writes the word "target" ONCE, so it
    -- announces ONE target ([CR#601.2c]), and the determiner scopes over
    -- the whole coordination. The kind index forces the alternatives to
    -- describe the same sort of thing without a gate. The obligations, in
    -- DECLARATION order: a coordination needs two alternatives
    -- (`TwoDisjuncts`), they are PARALLEL -- the same grammatical rank, and
    -- committing their referent alike, zone and presupposed type both
    -- (`ParallelDisjuncts`) -- a word that fills one phrase-level slot is
    -- not an alternative (`CoordinableDisjuncts`), and no alternative
    -- repeats another (`DistinctDisjuncts`).
    -- The COORDINATOR IS DERIVED, not fixed, and the claim that this row
    -- always spells "or" was measured and is false. English writes the
    -- SAME alternation under both words and the environment picks which:
    -- a singular determiner takes "or" ("an instant or sorcery spell" 291
    -- lines, "instant or sorcery card" 173, "artifact or enchantment"
    -- 199, "Aura or Equipment" 21), while an affirmative plural or
    -- distributive class head takes "and" ("instant and sorcery spells"
    -- 57, "instant and sorcery cards" 31, "each instant and sorcery …"
    -- 27, "Aura and Equipment" 17, "creature and planeswalker" 16,
    -- "artifact and enchantment" 7, "artifact and creature" 4, plus 10
    -- multi-item coordinations from Dovin, Hand of Control to Second
    -- Sunrise). The bare plural takes "or" only in a
    -- DOWNWARD-ENTAILING environment -- 4 lines, three of them "can't
    -- cast instant or sorcery spells" (Abeyance, Azor, Sphinx's Decree)
    -- and one "spend this mana only to cast" (Jaya Ballard) -- which is
    -- the negative-polarity licence, not a second construction. So the
    -- word tracks polarity and class-plurality, and one term is written
    -- both ways: finding 275's idiom, that an axis varying perfectly with
    -- something already written is a spelling fact, applied to the
    -- coordinator itself.
    -- The SEMANTICS are unchanged by the word, which is what makes the
    -- derivation honest rather than a pun. A plural class head describes
    -- a set whose members each satisfy ONE of the alternatives --
    -- [CR#205.2b]'s "objects with more than one card type" satisfy "the
    -- criteria for any effect that applies to any of their card types",
    -- and [CR#300.2] says the same of the type list -- so "instant and
    -- sorcery spells" is the UNION, one alternative apiece, and not the
    -- intersection. Worth saying for this pair in particular, because the
    -- intersection is not impossible: [CR#205.1a] has an object with
    -- either type RETAIN it when an effect sets a new one, so a
    -- both-typed object is reachable and the coordinator still does not
    -- mean it -- the corpus writes the plural head of the two classes
    -- together and never of the overlap. What is NOT this row:
    -- coordinated COLORS ("white and blue Bird") and coordinated
    -- SUPERTYPES are genuine conjunction -- one object bearing both -- and
    -- stay `And`; a negated union is written as juxtaposed negations
    -- ("nonartifact, nonland"), which is `And` of `Not` and needs nothing
    -- here. No CR rule defines the coordination itself (finding 398's
    -- pattern), so the derivation rests on the corpus and the semantics
    -- above.
    -- spelling: (construction-owned -- serial-comma coordination whose
    -- COORDINATOR is derived from the determiner and the polarity: "or"
    -- under a singular determiner or in a downward-entailing environment,
    -- "and" under an affirmative plural or "each" class head; the guide
    -- puts the Oxford comma before the coordinator from three items up.
    -- Core keeps only the operator -- `Predicate::Or`, `filter.rs` -- and
    -- says why in the lowering: the core grammar "is a compiled artifact
    -- and carries no record of the semantic spelling")
    Or : (ps : List (Predicate bs k)) -> {auto 0 tw : TwoDisjuncts ps} ->
         {auto 0 pd : ParallelDisjuncts ps} ->
         {auto 0 cd : CoordinableDisjuncts ps} ->
         {auto 0 dd : DistinctDisjuncts ps} -> Predicate bs k
    -- "don't"/"non-" on a modifier — over a negatable one only
    -- (`Negatable`: not the class word, not "other", not a negation).
    -- spelling: (construction-owned -- negates its inner predicate's own
    -- frame: "non-<Param(0)>" for a type word, "isn't <Param(0)>"/"doesn't
    -- <Param(0)>" for a clause; the transform depends on the negated
    -- predicate's own shape), kind: TODO(reason: non-head per hasHead)
    Not : (p : Predicate bs k) -> {auto 0 ng : Negatable p} -> Predicate bs k
    -- the modifier "other"/"another" ([CR#115.4]): distinct from every
    -- earlier target of this kind; presupposes one exists.
    -- spelling: ["other"] (register variant "another"), kind: TODO(reason:
    -- non-head modifier per hasHead)
    Other : {auto 0 ok : anyTargeted k bs = True} -> Predicate bs k
    -- the ANCHORED COMPLEMENT -- "other" over a named referent: the
    -- described domain MINUS the anchor ("each other creature you
    -- control", "each other player"). One modifier word, and a different
    -- relation from `Other` above, which is why it is a second row and not
    -- a widening (the `Compare`/`CompareAmt` and `Not`/`NotCond` shape --
    -- one namespace, two frames, and the argument in the name). `Other` is
    -- the TARGETING distinctness a slot announces against the slots before
    -- it ([CR#115.4]'s "another target"), so its anchor is every earlier
    -- target and the discourse supplies it; this one subtracts ONE
    -- referent the clause names, and carries it.
    -- The anchor is a READ or a written TARGET, and singular
    -- (`ComplementAnchor`, chapter twenty-six). It used to be a read ONLY,
    -- which conflated two questions: an anchor written as "a creature"
    -- would indeed announce a referent the sentence never spelled
    -- (`badComplementAnchorAnnounces`), but "other than target player"
    -- spells its referent out loud and the corpus writes it twice (Death
    -- by Dragons, Terrifying Presence). The announcement travels with the
    -- phrase now (`predDelta`). Its KIND is the phrase's own, by the index
    -- -- "each other player" excludes a player and cannot exclude a
    -- creature -- and its NUMBER is singular.
    -- What it does NOT need is a count, and that is the finding:
    -- subtracting a REFERENT from a DESCRIPTION needs no cardinality at
    -- all, so it lands now; subtracting a SUBSET from a GROUP ("the rest",
    -- "the other", "both") still needs a binding to record how many, and
    -- stays ledgered.
    -- spelling: ["other"] (register variant "another"; the ANCHOR is
    -- unpronounced -- English leaves it to salience, and the source-anchored
    -- reading is the corpus's mass), kind: TODO(reason: non-head modifier
    -- per hasHead)
    OtherThan : (n : Noun bs k) -> {auto 0 ca : ComplementAnchor n} -> Predicate bs k
    -- "any target" ([CR#115.4]: creature, player, planeswalker, or
    -- battle). NOT yet de-macroable: needs `Or` and the object/player
    -- kind join; primitive only until a chapter grows those.
    -- spelling: ["any target"], kind: Nominal (hasHead = True; matches
    -- constructors.ron's own `AnyTarget` entry, announcement: true)
    AnyTarget : Predicate bs Object

  ||| The head type a predicate projects onto its referent — what "that
  ||| creature" remembers across a zone change.
  public export
  seedTy : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedTy (HasType t) = Just t
  seedTy (And ps) = seedTyAll ps
  -- alternatives project only what they AGREE on: "artifact or
  -- enchantment" names a referent whose type the phrase declines to
  -- fix, so it projects none — the honest silence, not a guess at the
  -- first alternative.
  seedTy (Or ps) = seedTyJoin ps
  seedTy _ = Nothing

  public export
  seedTyAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe CardType
  seedTyAll [] = Nothing
  seedTyAll (p :: ps) = case seedTy p of
    Just t => Just t
    Nothing => seedTyAll ps

  ||| A conjunction takes the FIRST head its members write; a
  ||| disjunction takes the one every alternative writes, or none.
  public export
  seedTyJoin : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe CardType
  seedTyJoin [] = Nothing
  seedTyJoin (p :: ps) = case seedTy p of
    Nothing => Nothing
    Just t => if allSeedTy t ps then Just t else Nothing

  public export
  allSeedTy : {0 bs : Bindings} -> {0 k : Kind} ->
              CardType -> List (Predicate bs k) -> Bool
  allSeedTy t [] = True
  allSeedTy t (p :: ps) = case seedTy p of
    Nothing => False
    Just u => sameCT t u && allSeedTy t ps

  public export
  optCT : Maybe CardType -> List CardType
  optCT Nothing = []
  optCT (Just t) = [t]

  ||| The head types a phrase OFFERS, as a SET — `seedTy`'s answer to the
  ||| question "other" asks. The two differ on a coordination only, and
  ||| they must: `seedTy` projects the ONE type the phrase fixes onto its
  ||| referent, and "artifact or enchantment" fixes none, but it does not
  ||| thereby offer nothing to anchor against — it offers one head per
  ||| alternative. The empty list is the head that really is untyped (the
  ||| class word, a zone clause's implicit card).
  public export
  headTys : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List CardType
  headTys (And ps) = headTysAll ps
  headTys (Or ps) = headTysJoin ps
  headTys p = optCT (seedTy p)

  public export
  headTysAll : {0 bs : Bindings} -> {0 k : Kind} ->
               List (Predicate bs k) -> List CardType
  headTysAll [] = []
  headTysAll (p :: ps) = case headTys p of
    [] => headTysAll ps
    ts => ts

  public export
  headTysJoin : {0 bs : Bindings} -> {0 k : Kind} ->
                List (Predicate bs k) -> List CardType
  headTysJoin [] = []
  headTysJoin (p :: ps) = headTys p ++ headTysJoin ps

  ||| The zone a predicate places its referent in — a bare description
  ||| means the battlefield ([CR#109.2]); a zone clause says otherwise,
  ||| and a battlefield STATE word says the same thing the zone clause
  ||| would: attackers are declared from creatures their controller
  ||| controls ([CR#508.1a]) and leaving the battlefield removes a
  ||| permanent from combat ([CR#506.4]), so the status predicate seeds
  ||| its own zone rather than leaving the phrase silent.
  public export
  seedZone : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe Zone
  seedZone (InZone z) = Just (zoneSort z)
  seedZone Attacking = Just Battlefield
  -- blockers are chosen from creatures the defending player controls
  -- ([CR#509.1a]) and a creature removed from combat "stops being an
  -- attacking, blocking, blocked, and/or unblocked creature"
  -- ([CR#506.4]), so the defending word seeds its zone exactly as the
  -- attacking one does.
  seedZone Blocking = Just Battlefield
  -- the two relational rows seed exactly what the designation word does:
  -- the relation holds on the battlefield or nowhere ([CR#509.1g],
  -- [CR#509.1h]).
  seedZone (BlockerOf _) = Just Battlefield
  seedZone (BlockedBy _) = Just Battlefield
  -- the history read seeds NOTHING, and it is the row where that answer is
  -- load-bearing rather than incidental ([CR#608.2i]; see the row's own
  -- comment for Continue?'s proof).
  seedZone (HappenedTo _ _) = Nothing
  -- the characteristic rows presuppose no zone: a colour, a supertype
  -- and a name belong to an object wherever it stands, and to spells on
  -- the stack ([CR#105.2] reads colour off the mana cost).
  seedZone (ColorIs _) = Nothing
  seedZone IsColorless = Nothing
  seedZone Multicolored = Nothing
  seedZone Monocolored = Nothing
  seedZone (HasSupertype _) = Nothing
  seedZone (Named _) = Nothing
  -- the player designation seeds nothing (a player has no zone); the two
  -- object designations seed the battlefield, [CR#701.15b]'s goaded
  -- creature and [CR#701.54e]'s Ring-bearer both being permanents.
  seedZone (HasPlayerDesignation _) = Nothing
  seedZone IsGoaded = Just Battlefield
  seedZone YourRingBearer = Just Battlefield
  seedZone IsEnchanted = Just Battlefield
  seedZone IsEquipped = Just Battlefield
  -- a token lives only on the battlefield ([CR#111.7]), and status is
  -- only a battlefield permanent's ([CR#110.5d]) — both words seed their
  -- zone exactly as the combat designations above do. The permanent HEAD
  -- deliberately seeds nothing: its zone story is the phrase's, which is
  -- what "permanent card from their hand" requires ([CR#110.4a]).
  seedZone IsToken = Just Battlefield
  seedZone (HasStatus _) = Just Battlefield
  -- a controller relation places its referent NOWHERE and rules zones out
  -- instead: [CR#109.4] gives a controller to objects "on the stack or on
  -- the battlefield" and to nothing else, which is a two-zone SET where
  -- this table names one zone. That set is `zoneAdmit`'s answer beside
  -- this one -- the second table this comment asked for while the row
  -- seeded the battlefield alone, a seed that bought the graveyard
  -- refusal (`badControlledInGraveyard`, unchanged: the graveyard is in
  -- neither zone) at the price of "spell(s) you control", which is 79
  -- supported lines and not the copy machinery this comment once claimed
  -- (15 of them say a spell can't be countered, 26 copy one, 7 grant it a
  -- keyword).
  seedZone (ControlledBy _) = Nothing
  -- the cast relation DOES place its referent, which is the difference
  -- between the two clauses this table can see: casting moves the card to
  -- the stack ([CR#601.2a]) and it is a spell only there ([CR#112.1]).
  seedZone (CastBy _) = Just Stack
  -- the linkage read places its referent as flatly as the zone clause
  -- does: [CR#607.2a] says the phrase "refers only to cards IN THE EXILE
  -- ZONE that were put there" by the linked ability, so a card that has
  -- since left is out of the group and "a creature exiled with this
  -- creature" describes nothing (`badExiledWithOnBattlefield`).
  seedZone (ExiledWith _) = Just Exile
  seedZone (And ps) = seedZoneAll ps
  -- the same agreement rule the head projection uses: "attacking or
  -- blocking" places its referent on the battlefield because BOTH
  -- alternatives do, while "artifact or enchantment" places it
  -- nowhere of its own and leaves the phrase's default to speak. A
  -- disagreement never reaches here — `ParallelDisjuncts` refuses it
  -- at the coordination — so a `Nothing` out of the join always means
  -- silence rather than a quarrel.
  seedZone (Or ps) = seedZoneJoin ps
  seedZone _ = Nothing

  public export
  seedZoneAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe Zone
  seedZoneAll [] = Nothing
  seedZoneAll (p :: ps) = case seedZone p of
    Just z => Just z
    Nothing => seedZoneAll ps

  public export
  seedZoneJoin : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe Zone
  seedZoneJoin [] = Nothing
  seedZoneJoin (p :: ps) = case seedZone p of
    Nothing => Nothing
    Just z => if allSeedZone z ps then Just z else Nothing

  public export
  allSeedZone : {0 bs : Bindings} -> {0 k : Kind} ->
                Zone -> List (Predicate bs k) -> Bool
  allSeedZone z [] = True
  allSeedZone z (p :: ps) = case seedZone p of
    Nothing => False
    Just w => sameZone z w && allSeedZone z ps

  ||| Which zones a member ADMITS its referent in, where `seedZone` names
  ||| the one zone a member PLACES it in. Two different questions, and only
  ||| one row has ever needed the second: a controller relation constrains
  ||| its referent to the stack or the battlefield without choosing between
  ||| them ([CR#109.4] — "Only objects on the stack or on the battlefield
  ||| have a controller. Objects that are neither on the stack nor on the
  ||| battlefield aren't controlled by any player"), and a table that can
  ||| name only ONE zone had to answer the battlefield and refuse "spell
  ||| you control" along with the graveyard.
  |||
  ||| The empty list is no demand at all, which is what every other row
  ||| answers — including the rows that PLACE their referent, since a seed
  ||| is already an exact answer and asking twice would say nothing new.
  ||| What the two tables cost together is one conjunct in `zonesOk`: the
  ||| phrase's effective zone must be a zone every member admits.
  public export
  zoneAdmit : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List Zone
  zoneAdmit (ControlledBy _) = [Battlefield, Stack]
  zoneAdmit _ = []

  ||| The card type a modifier PRESUPPOSES of its referent — the type twin
  ||| of `seedZone`, and a different question from `seedTy`, which projects
  ||| the phrase's own HEAD. Only a creature can attack or block
  ||| ([CR#506.3]), so the status word presupposes the type exactly as it
  ||| presupposes the battlefield. It recurses through BOTH list forms: a
  ||| disjunction presupposes what every alternative presupposes, which is
  ||| how "attacking or blocking" keeps demanding a creature though neither
  ||| word survives alone. A conjunction needs its row for the
  ||| alternatives' sake — the coherence scans see a top-level conjunction
  ||| through `flattenPs`, but one BURIED in an alternative is left whole
  ||| and the presupposition written there is written all the same
  ||| (`badWrappedStatusLaunder`).
  public export
  seedType : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedType Attacking = Just Creature
  seedType Blocking = Just Creature
  seedType (BlockerOf _) = Just Creature
  seedType (BlockedBy _) = Just Creature
  seedType (HappenedTo _ _) = Nothing
  seedType (ColorIs _) = Nothing
  seedType IsColorless = Nothing
  seedType Multicolored = Nothing
  seedType Monocolored = Nothing
  seedType (HasSupertype _) = Nothing
  seedType (Named _) = Nothing
  seedType (HasPlayerDesignation _) = Nothing
  seedType IsGoaded = Just Creature
  seedType YourRingBearer = Just Creature
  seedType IsEnchanted = Nothing
  seedType IsEquipped = Nothing
  -- a bound on a number presupposes an object that HAS that number, which
  -- is the status word's shape with a table in place of a fixed answer:
  -- power and toughness demand a creature ([CR#208.3]), mana value demands
  -- nothing ([CR#202.3]). What it does NOT presuppose is a zone, and that
  -- is the difference from the status words above: a creature card in a
  -- graveyard keeps the power printed on it ([CR#208.1,208.3]) and every
  -- object has a mana value wherever it stands -- which is why "return
  -- target creature card with power 2 or less from your graveyard to the
  -- battlefield" is ordinary oracle and no zone seed may refuse it.
  seedType (Compare c _ _) = comparedType c
  -- a subtype word presupposes the card type whose closed set it comes
  -- from ([CR#205.1a]; [CR#205.3m] makes every row here a creature type).
  -- It presupposes no ZONE: a Zombie card in a graveyard is still a
  -- Zombie, so the word places nothing and the phrase's default speaks.
  seedType (HasSubtype s) = Just (subtypeType s)
  seedType (And ps) = seedTypeAll ps
  seedType (Or ps) = seedTypeJoin ps
  seedType _ = Nothing

  ||| A conjunction presupposes what its first presupposing member
  ||| does — `seedZoneAll`'s rule, for the type twin.
  public export
  seedTypeAll : {0 bs : Bindings} -> {0 k : Kind} ->
                List (Predicate bs k) -> Maybe CardType
  seedTypeAll [] = Nothing
  seedTypeAll (p :: ps) = case seedType p of
    Just t => Just t
    Nothing => seedTypeAll ps

  public export
  seedTypeJoin : {0 bs : Bindings} -> {0 k : Kind} ->
                 List (Predicate bs k) -> Maybe CardType
  seedTypeJoin [] = Nothing
  seedTypeJoin (p :: ps) = case seedType p of
    Nothing => Nothing
    Just t => if allSeedType t ps then Just t else Nothing

  public export
  allSeedType : {0 bs : Bindings} -> {0 k : Kind} ->
                CardType -> List (Predicate bs k) -> Bool
  allSeedType t [] = True
  allSeedType t (p :: ps) = case seedType p of
    Nothing => False
    Just u => sameCT t u && allSeedType t ps

  ||| A phrase names a positive HEAD: a type word, a player word, a
  ||| quality word, "any target", or a zone clause (whose implicit
  ||| head is the zone's carrier — "a card in your hand"). Modifiers
  ||| alone (`Not`, `Other`, state words, relative clauses) head
  ||| nothing: "choose a noncolor" is unwritable ([CR#105.1,608.2d]).
  ||| Full rows: a new predicate form must declare its headedness.
  public export
  hasHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasHead (HasType _) = True
  hasHead (HasSubtype _) = True
  hasHead AnyPlayer = True
  hasHead Opponent = True
  hasHead (QualityNoun _) = True
  hasHead (OfChosen _) = False
  hasHead (HasKeyword _) = False
  hasHead (ControlledBy _) = False
  hasHead (CastBy _) = False
  hasHead Attacking = False
  hasHead Blocking = False
  hasHead (BlockerOf _) = False
  hasHead (BlockedBy _) = False
  hasHead (HappenedTo _ _) = False
  hasHead (ColorIs _) = False
  hasHead IsColorless = False
  hasHead Multicolored = False
  hasHead Monocolored = False
  hasHead (HasSupertype _) = False
  hasHead (Named _) = False
  hasHead (HasPlayerDesignation _) = False
  hasHead IsGoaded = False
  hasHead YourRingBearer = False
  hasHead IsEnchanted = False
  hasHead IsEquipped = False
  hasHead Permanent = True
  hasHead IsToken = True
  hasHead (HasStatus _) = False
  hasHead (Compare _ _ _) = False
  hasHead (InZone _) = True
  -- the linkage read heads its phrase for `InZone`'s reason exactly:
  -- the exile zone's carrier noun is "card" ([CR#406.2]), so "a card
  -- exiled with this artifact" needs no head word beside it.
  hasHead (ExiledWith _) = True
  hasHead (And ps) = hasHeadAny ps
  -- ANY member heads a conjunction — one head plus its modifiers —
  -- but EVERY alternative has to head a disjunction, because each
  -- one stands where the phrase's head would: "artifact or
  -- enchantment" heads, "attacking or blocking" does not, and
  -- `ParallelDisjuncts` is what stops the two mixing, so this reads
  -- the answer they share.
  hasHead (Or ps) = hasHeadAll ps
  hasHead (Not _) = False
  hasHead Other = False
  hasHead (OtherThan _) = False
  hasHead AnyTarget = True

  public export
  hasHeadAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasHeadAny [] = False
  hasHeadAny (p :: ps) = hasHead p || hasHeadAny ps

  public export
  hasHeadAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasHeadAll [] = True
  hasHeadAll (p :: ps) = hasHead p && hasHeadAll ps

  ||| The determiner gate's witness form (a distinctive search name).
  public export
  data Headed : Predicate bs k -> Type where
    MkHeaded : {auto 0 ok : hasHead p = True} -> Headed p

  ||| Every member of a conjunction, nested conjunctions flattened — the
  ||| member scan the coherence gates share, so a clash one level down is
  ||| refused exactly as a sibling clash is. It flattens conjunctions ONLY:
  ||| a disjunction's alternatives are not siblings of the conjunction
  ||| around it (splicing them in would make "attacking or blocking" read
  ||| as "attacking and blocking"), so an `Or` passes through whole.
  public export
  flattenPs : {0 bs : Bindings} -> {0 k : Kind} ->
              List (Predicate bs k) -> List (Predicate bs k)
  flattenPs [] = []
  flattenPs (And qs :: ps) = flattenPs qs ++ flattenPs ps
  flattenPs (p :: ps) = p :: flattenPs ps

  ||| A conjunction's explicit zones agree: an object is in ONE zone,
  ||| and the seed projections take the first zone written, so a
  ||| contradicting later conjunct must be refused, not ignored.
  public export
  zonesAgree : {0 bs : Bindings} -> {0 k : Kind} -> Maybe Zone -> List (Predicate bs k) -> Bool
  zonesAgree acc [] = True
  zonesAgree acc (p :: ps) = case seedZone p of
    Nothing => zonesAgree acc ps
    Just z => case acc of
      Nothing => zonesAgree (Just z) ps
      Just w => sameZone w z && zonesAgree (Just w) ps

  ||| The zones a member rules OUT — EXPLICIT zone clauses only. Zone
  ||| negation is real oracle ("Each Vampire creature card you own that
  ||| isn't on the battlefield has madness.", Falkenrath Gorger), so
  ||| `Not (InZone …)` stays writable; what it cannot do is contradict the
  ||| zone the phrase actually places its referent in. A negated modifier
  ||| that merely PRESUPPOSES a zone rules out nothing: presupposition
  ||| projects through negation, so "nonattacking creature" still stands on
  ||| the battlefield (`rawNonattacking`).
  public export
  negZonesOf : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List Zone
  negZonesOf (Not (InZone z)) = [zoneSort z]
  negZonesOf _ = []

  public export
  negZones : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> List Zone
  negZones [] = []
  negZones (p :: ps) = negZonesOf p ++ negZones ps

  public export
  zoneMember : Zone -> List Zone -> Bool
  zoneMember z [] = False
  zoneMember z (w :: ws) = sameZone z w || zoneMember z ws

  ||| The admissibility scan: the phrase's effective zone against what
  ||| each member ADMITS. An empty answer is silence and passes, which is
  ||| `zoneFits`' discipline at the second table.
  public export
  zoneAdmits : Zone -> List Zone -> Bool
  zoneAdmits z [] = True
  zoneAdmits z zs = zoneMember z zs

  public export
  zoneAdmitsAll : {0 bs : Bindings} -> {0 k : Kind} ->
                  Zone -> List (Predicate bs k) -> Bool
  zoneAdmitsAll z [] = True
  zoneAdmitsAll z (p :: ps) = zoneAdmits z (zoneAdmit p) && zoneAdmitsAll z ps

  ||| The conjunction's whole zone story: the explicit zones agree with
  ||| each other, the phrase's EFFECTIVE zone — a bare description means
  ||| the battlefield ([CR#109.2]) — is not one the phrase rules out, and
  ||| that same zone is one every member ADMITS. "creature that isn't on
  ||| the battlefield" contradicts its own default; "creature card in your
  ||| graveyard that isn't on the battlefield" does not; "creature you
  ||| control in your graveyard" passes the first two tests and fails the
  ||| third, the graveyard being neither zone a controller relation admits.
  public export
  zonesOk : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  zonesOk ps = zonesAgree Nothing (flattenPs ps) &&
               not (zoneMember (zoneOr Battlefield (seedZoneAll (flattenPs ps)))
                               (negZones (flattenPs ps))) &&
               zoneAdmitsAll (zoneOr Battlefield (seedZoneAll (flattenPs ps)))
                             (flattenPs ps)

  public export
  data ZoneCoherent : List (Predicate bs k) -> Type where
    MkZoneCoherent : {auto 0 ok : zonesOk ps = True} -> ZoneCoherent ps

  ||| Syntactic predicate equality — enough to spot a member that
  ||| contradicts a sibling, or an alternative that repeats one. Still
  ||| CONSERVATIVE on the rows carrying a noun, but no longer VACUOUSLY so
  ||| on the rows carrying STRUCTURE: `ControlledBy` compares its possessor
  ||| with `nounEqRef` and a conjunction its members pointwise, so neither
  ||| the syntactically identical contradiction nor the syntactically
  ||| identical alternative launders through a blanket `False`. That
  ||| `False` reads "not provably the SAME referent", so the gate
  ||| under-refuses rather than over-refuses.
  public export
  predEq : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Predicate bs k -> Bool
  predEq (HasType a) (HasType b) = sameCT a b
  predEq (HasType _) _ = False
  predEq (HasSubtype a) (HasSubtype b) = sameSub a b
  predEq (HasSubtype _) _ = False
  predEq AnyPlayer AnyPlayer = True
  predEq AnyPlayer _ = False
  predEq Opponent Opponent = True
  predEq Opponent _ = False
  -- the kind index already forces the two sorts equal here
  predEq (QualityNoun a) (QualityNoun a) = True
  predEq (QualityNoun _) _ = False
  predEq (OfChosen a) (OfChosen b) = sameQ a b
  predEq (OfChosen _) _ = False
  predEq (HasKeyword a) (HasKeyword b) = sameKeyword a b
  predEq (HasKeyword _) _ = False
  predEq (ControlledBy a) (ControlledBy b) = nounEqRef a b
  predEq (ControlledBy _) _ = False
  predEq (CastBy a) (CastBy b) = nounEqRef a b
  predEq (CastBy _) _ = False
  -- two linkage reads name the same group whenever their sources are
  -- the same object, which `LinkSource` has already made certain.
  predEq (ExiledWith a) (ExiledWith b) = nounEqRef a b
  predEq (ExiledWith _) _ = False
  predEq Attacking Attacking = True
  predEq Attacking _ = False
  predEq Blocking Blocking = True
  predEq Blocking _ = False
  -- the relational rows compare like `ControlledBy`: the row is the same
  -- word, so what is left to compare is the relatum.
  predEq (BlockerOf a) (BlockerOf b) = nounEqRef a b
  predEq (BlockerOf _) _ = False
  predEq (BlockedBy a) (BlockedBy b) = nounEqRef a b
  predEq (BlockedBy _) _ = False
  -- both arguments are closed words, so the row compares wholly rather
  -- than conservatively: two history reads are the same phrase exactly
  -- when they name the same event over the same window.
  predEq (HappenedTo a v) (HappenedTo b w) = sameEventName a b && sameLookback v w
  predEq (HappenedTo _ _) _ = False
  predEq (ColorIs a) (ColorIs b) = sameColor a b
  predEq (ColorIs _) _ = False
  predEq IsColorless IsColorless = True
  predEq IsColorless _ = False
  predEq Multicolored Multicolored = True
  predEq Multicolored _ = False
  predEq Monocolored Monocolored = True
  predEq Monocolored _ = False
  predEq (HasSupertype a) (HasSupertype b) = sameSupertype a b
  predEq (HasSupertype _) _ = False
  -- name EQUALITY, which [CR#201.2a] makes rules-real, and the one
  -- place a name string is looked at anywhere in this grammar: it asks
  -- whether two phrases name the same card and never what either names.
  predEq (Named a) (Named b) = a == b
  predEq (Named _) _ = False
  predEq (HasPlayerDesignation a) (HasPlayerDesignation b) = samePlayerDesignation a b
  predEq (HasPlayerDesignation _) _ = False
  predEq IsGoaded IsGoaded = True
  predEq IsGoaded _ = False
  predEq YourRingBearer YourRingBearer = True
  predEq YourRingBearer _ = False
  predEq IsEnchanted IsEnchanted = True
  predEq IsEnchanted _ = False
  predEq IsEquipped IsEquipped = True
  predEq IsEquipped _ = False
  predEq Permanent Permanent = True
  predEq Permanent _ = False
  predEq IsToken IsToken = True
  predEq IsToken _ = False
  predEq (HasStatus v) (HasStatus w) = sameStatusVal v w
  predEq (HasStatus _) _ = False
  -- all three parts: the same characteristic, the same relation, and the
  -- same bound. `boundEq` is conservative where the rest of this function
  -- is, and a read bound is where that conservatism now falls.
  predEq (Compare c r b) (Compare d s e) = sameChar c d && sameCmp r s &&
                                           boundEq b e
  predEq (Compare _ _ _) _ = False
  predEq (InZone z) (InZone w) = sameZone (zoneSort z) (zoneSort w)
  predEq (InZone _) _ = False
  -- member by member, in order: a structured alternative repeated word
  -- for word is the same repetition "artifact or artifact" is, and the
  -- conservative row saw none of it (`badRepeatedStructuredDisjunct`).
  -- Order-sensitive, which under-refuses a re-ordered spelling of the
  -- same modifiers — conservative in the direction the rest of the
  -- function is.
  predEq (And xs) (And ys) = predEqAll xs ys
  predEq (And _) _ = False
  -- the coordination keeps the blanket row, and pays nothing for it:
  -- nothing may nest an `Or` in an `Or` (`CoordinableDisjuncts`), so
  -- two coordinations never meet as alternatives, and `Not` reaches
  -- neither combinator, so no negation pair launders through the
  -- `False` either.
  predEq (Or _) _ = False
  predEq (Not a) (Not b) = predEq a b
  predEq (Not _) _ = False
  predEq Other Other = True
  predEq Other _ = False
  -- two complements are the same modifier when they exclude the same
  -- referent, which `nounEqRef` answers with its usual conservatism.
  predEq (OtherThan a) (OtherThan b) = nounEqRef a b
  predEq (OtherThan _) _ = False
  predEq AnyTarget AnyTarget = True
  predEq AnyTarget _ = False

  ||| Two member lists, pointwise and in order.
  public export
  predEqAll : {0 bs : Bindings} -> {0 k : Kind} ->
              List (Predicate bs k) -> List (Predicate bs k) -> Bool
  predEqAll [] [] = True
  predEqAll (x :: xs) (y :: ys) = predEq x y && predEqAll xs ys
  predEqAll _ _ = False

  ||| Are these two members each other's negation?
  public export
  negates : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Predicate bs k -> Bool
  negates (Not a) b = predEq a b
  negates a (Not b) = predEq a b
  negates _ _ = False

  public export
  anyNegates : {0 bs : Bindings} -> {0 k : Kind} ->
               Predicate bs k -> List (Predicate bs k) -> Bool
  anyNegates p [] = False
  anyNegates p (q :: qs) = negates p q || anyNegates p qs

  public export
  noNegatedPair : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  noNegatedPair [] = True
  noNegatedPair (p :: ps) = not (anyNegates p ps) && noNegatedPair ps

  ||| Two members claim the SAME status category with OPPOSITE values —
  ||| "tapped untapped creature" describes nothing ([CR#110.5] gives a
  ||| permanent exactly one value per category) — while values of
  ||| different categories stack ("tapped face-down permanent" is a
  ||| morph turned sideways).
  public export
  statusClashOf : {0 bs : Bindings} -> {0 k : Kind} ->
                  Predicate bs k -> Predicate bs k -> Bool
  statusClashOf (HasStatus v) (HasStatus w) = statusClash v w
  statusClashOf _ _ = False

  ||| A member says COLORLESS and another says a color — "colorless white
  ||| creature" describes nothing, [CR#105.2c] giving a colorless object
  ||| "no color" in as many words. `statusClashOf`'s shape at the one
  ||| other place a closed value pair contradicts, and its whole extent:
  ||| two COLORS never clash, [CR#105.2] letting an object be "one or more
  ||| of the five", so a gold card is black and red and the phrase that
  ||| says both agrees. The count words are left alone -- whether
  ||| "monocolored multicolored" contradicts is arithmetic over a set the
  ||| phrase does not write, and no corpus line poses it.
  public export
  colorClashOf : {0 bs : Bindings} -> {0 k : Kind} ->
                 Predicate bs k -> Predicate bs k -> Bool
  colorClashOf IsColorless (ColorIs _) = True
  colorClashOf (ColorIs _) IsColorless = True
  colorClashOf _ _ = False

  public export
  anyColorClash : {0 bs : Bindings} -> {0 k : Kind} ->
                  Predicate bs k -> List (Predicate bs k) -> Bool
  anyColorClash p [] = False
  anyColorClash p (q :: qs) = colorClashOf p q || anyColorClash p qs

  public export
  noColorClash : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  noColorClash [] = True
  noColorClash (p :: ps) = not (anyColorClash p ps) && noColorClash ps

  public export
  anyStatusClash : {0 bs : Bindings} -> {0 k : Kind} ->
                   Predicate bs k -> List (Predicate bs k) -> Bool
  anyStatusClash p [] = False
  anyStatusClash p (q :: qs) = statusClashOf p q || anyStatusClash p qs

  public export
  noStatusClash : {0 bs : Bindings} -> {0 k : Kind} ->
                  List (Predicate bs k) -> Bool
  noStatusClash [] = True
  noStatusClash (p :: ps) = not (anyStatusClash p ps) && noStatusClash ps

  ||| Does a member spell the permanent head?
  public export
  isPermanentHead : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isPermanentHead Permanent = True
  isPermanentHead _ = False

  public export
  anyPermanentHead : {0 bs : Bindings} -> {0 k : Kind} ->
                     List (Predicate bs k) -> Bool
  anyPermanentHead [] = False
  anyPermanentHead (p :: ps) = isPermanentHead p || anyPermanentHead ps

  ||| Does any member PROJECT a non-permanent head type beside the
  ||| permanent word? [CR#110.4] rules instants and sorceries out of the
  ||| permanent types in as many words, so "permanent" beside a projected
  ||| instant head describes nothing (`badPermanentInstant`). Read off
  ||| `seedTy` — the projection the phrase itself fixes — through the
  ||| `permanentType` table the placement gate already owns.
  public export
  anyNonPermanentTy : {0 bs : Bindings} -> {0 k : Kind} ->
                      List (Predicate bs k) -> Bool
  anyNonPermanentTy [] = False
  anyNonPermanentTy (p :: ps) = case seedTy p of
    Just t => not (permanentType t) || anyNonPermanentTy ps
    Nothing => anyNonPermanentTy ps

  ||| The card types a member rules OUT: "non-creature" negates the type
  ||| word's own head. A negated STATUS word rules out no type — the
  ||| presupposition projects THROUGH the negation, which is why
  ||| "nonattacking creature" is plentiful oracle (`rawNonattacking`).
  public export
  negTypesOf : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> List CardType
  negTypesOf (Not p) = case seedTy p of
    Just t => [t]
    Nothing => []
  negTypesOf _ = []

  public export
  negTypes : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> List CardType
  negTypes [] = []
  negTypes (p :: ps) = negTypesOf p ++ negTypes ps

  ||| The types the members presuppose, positively.
  public export
  seedTypes : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> List CardType
  seedTypes [] = []
  seedTypes (p :: ps) = case seedType p of
    Just t => t :: seedTypes ps
    Nothing => seedTypes ps

  public export
  typeMember : CardType -> List CardType -> Bool
  typeMember t [] = False
  typeMember t (u :: us) = sameCT t u || typeMember t us

  public export
  anyTypeClash : List CardType -> List CardType -> Bool
  anyTypeClash [] seeds = False
  anyTypeClash (t :: ts) seeds = typeMember t seeds || anyTypeClash ts seeds

  ||| No member is the syntactic negation of a sibling ("of the chosen
  ||| color and not of the chosen color" describes nothing), and no
  ||| member negates a TYPE another member presupposes: only a creature
  ||| can attack ([CR#506.3]), so "attacking noncreature" describes
  ||| nothing either. Status-category clashes ([CR#110.5]) and a
  ||| permanent paired with a non-permanent projected type ([CR#110.4])
  ||| describe nothing too. Positive types do NOT clash with each other —
  ||| they stack, an attacking artifact being an artifact creature — so
  ||| only the negation raises. The flattened scan catches the nested
  ||| spelling of both.
  public export
  contradictionFree : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  contradictionFree ps = noNegatedPair (flattenPs ps) &&
                         not (anyTypeClash (negTypes (flattenPs ps))
                                           (seedTypes (flattenPs ps))) &&
                         noStatusClash (flattenPs ps) &&
                         noColorClash (flattenPs ps) &&
                         not (anyPermanentHead (flattenPs ps) &&
                              anyNonPermanentTy (flattenPs ps))

  public export
  data ContradictionFree : List (Predicate bs k) -> Type where
    MkContradictionFree : {auto 0 ok : contradictionFree ps = True} ->
                          ContradictionFree ps

  ||| Does this member carry the "other" modifier?
  public export
  hasOther : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasOther Other = True
  -- the anchored complement fills the SAME selector slot — one "other"
  -- per phrase however it is anchored — so the slot scan sees both.
  hasOther (OtherThan _) = True
  hasOther (And ps) = hasOtherAny ps
  -- no reach into a disjunction is needed, and none would be honest:
  -- the modifier fills one slot for the WHOLE coordinated phrase
  -- ("Another target Wolf or Werewolf you control"), so
  -- `CoordinableDisjuncts` refuses it as an alternative and there is
  -- nothing inside an `Or` for the cap to miss.
  hasOther (Or _) = False
  hasOther _ = False

  public export
  hasOtherAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasOtherAny [] = False
  hasOtherAny (p :: ps) = hasOther p || hasOtherAny ps

  ||| The anchor obligation at the phrase level: `Other`'s own gate demands
  ||| a same-KIND target mention, and the conjunction it sits in supplies
  ||| the head type that mention must be compatible with. The modifier also
  ||| has ONE slot per phrase — the style guide's selector order gives
  ||| other/another a single position and no corpus line doubles it — so a
  ||| second "other" is unwritable (`badDoubleOther`). The cap is written
  ||| FIRST and the CONTEXT-reading conjunct last, so the conjunction
  ||| reduces for a phrase whose context is abstract: `anyOtherTarget`
  ||| carries its anchor presupposition as a hypothesis, and `x && True`
  ||| would stay stuck on the neutral `x`. The two branches are exclusive
  ||| by the cap above them, which is why they are an `if` and not a third
  ||| conjunct.
  public export
  otherAnchorOk : {bs : Bindings} -> (k : Kind) -> List CardType ->
                  List (Predicate bs k) -> Bool
  otherAnchorOk k ts ps =
    if atMostOne (countOthers (flattenPs ps))
      then (if hasBareOtherAny ps
              then anchorFound k ts bs
              else complementAnchorsOk ts (flattenPs ps))
      else False

  ||| Does an anchor's own head type fit the phrase it is subtracted
  ||| from? The same question `anchorFound` asks of a discourse mention,
  ||| asked of the noun the complement carries — and the empty head list
  ||| reads the same way it does there, as the genuinely untyped head
  ||| (the player kind, whose kind agreement is the whole obligation)
  ||| rather than as an exhausted search.
  public export
  anchorTyFitsSome : List CardType -> Maybe CardType -> Bool
  anchorTyFitsSome [] t = False
  anchorTyFitsSome (u :: us) t = anchorTyOk u t || anchorTyFitsSome us t

  public export
  anchorTyFits : List CardType -> Maybe CardType -> Bool
  anchorTyFits [] t = True
  anchorTyFits (u :: us) t = anchorTyFitsSome (u :: us) t

  ||| Every anchored complement in a modifier list excludes something the
  ||| phrase could have described. "Each other creature" anchored to a LAND
  ||| subtracts nothing and is written by no corpus line
  ||| (`badComplementCrossHead`) — the same evidence `anyTargetedTy` reads
  ||| for the bare word. An anchor that projects no head type at all (bare
  ||| `This`, "you") fits any head, exactly as an untyped mention does.
  public export
  complementAnchorsOk : {bs : Bindings} -> {k : Kind} -> List CardType ->
                        List (Predicate bs k) -> Bool
  complementAnchorsOk ts [] = True
  -- the recursion is written FIRST so a list whose complement is its
  -- last member reduces to the bare anchor test — the `x && True`
  -- neutral is what leaves an abstract anchor stuck, and the macros
  -- carrying one thread it to their callers.
  complementAnchorsOk ts (OtherThan n :: ps) =
    complementAnchorsOk ts ps && anchorTyFits ts (nounTy n)
  complementAnchorsOk ts (_ :: ps) = complementAnchorsOk ts ps

  ||| The head-typed "other" presupposition as a witness ([CR#115.4];
  ||| the guide reserves "another" for excluding the source or first
  ||| referent, and writes two separately described roles without it).
  ||| The head is read as a SET (`headTys`), so a coordinated head
  ||| demands an anchor compatible with SOME alternative rather than
  ||| with none. Player-kind "other" needs no head type — kind
  ||| agreement is the whole obligation.
  public export
  data OtherAnchored : List (Predicate bs k) -> Type where
    MkOtherAnchored : {auto 0 ok : otherAnchorOk k (headTysAll ps) ps = True} ->
                      OtherAnchored ps

  public export
  isAnyTarget : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isAnyTarget AnyTarget = True
  isAnyTarget _ = False

  public export
  isOther : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isOther Other = True
  isOther (OtherThan _) = True
  isOther _ = False

  ||| The BARE "other" alone — the one whose anchor the discourse has to
  ||| supply. `hasOther` above scans for the selector SLOT (both
  ||| spellings of the word fill it), this scans for the phrases that
  ||| make a presupposition of the CONTEXT; the anchored complement
  ||| carries its own referent and asks the discourse for nothing.
  public export
  hasBareOther : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  hasBareOther Other = True
  hasBareOther (And ps) = hasBareOtherAny ps
  hasBareOther _ = False

  public export
  hasBareOtherAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  hasBareOtherAny [] = False
  hasBareOtherAny (p :: ps) = hasBareOther p || hasBareOtherAny ps

  public export
  anyIsAnyTarget : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsAnyTarget [] = False
  anyIsAnyTarget (p :: ps) = isAnyTarget p || anyIsAnyTarget ps

  ||| Is the class word this phrase's HEAD — is the phrase "any target"
  ||| (or Arc Trail's "any other target")? The flattened member scan
  ||| answers exactly that: `AnyTargetLone` already forbids any
  ||| companion but "other", so a phrase holding the word IS the class
  ||| word. It deliberately does NOT reach into embedded nouns the way
  ||| `anyTargetFree` does — "target creature the controller of any
  ||| target controls" has a creature head and a creature's zone; the
  ||| class word is somebody else's possessor there.
  public export
  headIsAnyTarget : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  headIsAnyTarget p = anyIsAnyTarget (flattenPs [p])

  public export
  allLoneOk : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  allLoneOk [] = True
  allLoneOk (p :: ps) = (isAnyTarget p || isOther p) && allLoneOk ps

  public export
  countAnyTargets : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Nat
  countAnyTargets [] = Z
  countAnyTargets (p :: ps) =
    if isAnyTarget p then S (countAnyTargets ps) else countAnyTargets ps

  public export
  countOthers : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Nat
  countOthers [] = Z
  countOthers (p :: ps) = if isOther p then S (countOthers ps) else countOthers ps

  ||| The multiplicity caps a single modifier slot writes.
  public export
  atMostOne : Nat -> Bool
  atMostOne Z = True
  atMostOne (S Z) = True
  atMostOne (S (S _)) = False

  public export
  exactlyOne : Nat -> Bool
  exactlyOne Z = False
  exactlyOne (S Z) = True
  exactlyOne (S (S _)) = False

  ||| "Any target" is a lone CLASS word: the guide reserves it for the
  ||| rules-defined damage target class ([CR#115.4]) and forbids it as
  ||| a synonym for "any object", so it takes no modifiers — the sole
  ||| corpus companion is "other" (Arc Trail's "any other target").
  ||| Both words are also written ONCE: oracle never repeats the class
  ||| word inside one phrase, nor the modifier (`badDoubleAnyTarget`,
  ||| `badDoubleOther`).
  public export
  anyTargetLone : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyTargetLone ps = if anyIsAnyTarget (flattenPs ps)
                       then allLoneOk (flattenPs ps) &&
                            exactlyOne (countAnyTargets (flattenPs ps)) &&
                            atMostOne (countOthers (flattenPs ps))
                       else True

  public export
  data AnyTargetLone : List (Predicate bs k) -> Type where
    MkAnyTargetLone : {auto 0 ok : anyTargetLone ps = True} -> AnyTargetLone ps

  public export
  isComparison : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isComparison (Compare _ _ _) = True
  isComparison _ = False

  public export
  countComparisons : {0 bs : Bindings} -> {0 k : Kind} ->
                     List (Predicate bs k) -> Nat
  countComparisons [] = Z
  countComparisons (p :: ps) =
    if isComparison p then S (countComparisons ps) else countComparisons ps

  ||| A phrase bounds a characteristic AT MOST ONCE. The corpus writes no
  ||| noun phrase with two bounds in it — the pairs that look like one are
  ||| two separate phrases ("Creatures you control with power 2 or less
  ||| can't be blocked by creatures with power 3 or greater") — so a second
  ||| bound is the multiplicity error "other" and the class word already
  ||| answer for (`badDoubleComparison`). Writing it as a CAP rather than
  ||| as arithmetic is what makes it honest and cheap at once: the
  ||| empty-range pair is refused because the phrase says it twice, not
  ||| because a range solver went looking, and the satisfiable pair is
  ||| refused on exactly the same evidence. An interval, when the corpus
  ||| finally wants one, is a construction with its own word.
  public export
  loneComparison : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  loneComparison ps = atMostOne (countComparisons (flattenPs ps))

  public export
  data LoneComparison : List (Predicate bs k) -> Type where
    MkLoneComparison : {auto 0 ok : loneComparison ps = True} ->
                       LoneComparison ps

  ||| A coordination offers two alternatives or more. At one it offers
  ||| none and spells exactly what the bare alternative spells
  ||| (`badSingletonOr`), at zero it spells nothing at all
  ||| (`badEmptyOr`) — the arity discipline `Sequentially` writes with
  ||| its own `AtLeastTwo`, in the member-scan form the sibling gates
  ||| here use (the shared witness would put a `Predicate` list under
  ||| an external family and cost the type its strict positivity).
  public export
  atLeastTwoPs : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  atLeastTwoPs [] = False
  atLeastTwoPs (_ :: []) = False
  atLeastTwoPs (_ :: _ :: _) = True

  public export
  data TwoDisjuncts : List (Predicate bs k) -> Type where
    MkTwoDisjuncts : {auto 0 ok : atLeastTwoPs ps = True} -> TwoDisjuncts ps

  public export
  headsUniform : {0 bs : Bindings} -> {0 k : Kind} ->
                 Bool -> List (Predicate bs k) -> Bool
  headsUniform b [] = True
  headsUniform b (p :: ps) = (if hasHead p then b else not b) && headsUniform b ps

  ||| Do two alternatives place their referent the same way? Naming the
  ||| same zone counts, and so does saying nothing about it — but
  ||| silence beside a commitment does NOT, because the phrase's own
  ||| projection has to speak for the coordination entire.
  public export
  sameSeedZone : Maybe Zone -> Maybe Zone -> Bool
  sameSeedZone Nothing Nothing = True
  sameSeedZone (Just z) (Just w) = sameZone z w
  sameSeedZone _ _ = False

  ||| The type twin of `sameSeedZone`, over what the alternatives
  ||| PRESUPPOSE.
  public export
  sameSeedType : Maybe CardType -> Maybe CardType -> Bool
  sameSeedType Nothing Nothing = True
  sameSeedType (Just t) (Just u) = sameCT t u
  sameSeedType _ _ = False

  public export
  seedsUniform : {0 bs : Bindings} -> {0 k : Kind} -> Maybe Zone -> Maybe CardType ->
                 List (Predicate bs k) -> Bool
  seedsUniform z t [] = True
  seedsUniform z t (p :: ps) = sameSeedZone z (seedZone p) &&
                               sameSeedType t (seedType p) &&
                               seedsUniform z t ps

  ||| Alternatives are PARALLEL: each one has to be able to stand where the
  ||| others stand. Three ways a phrase can fail that, and the guide names
  ||| them with one sentence — "Repeat the carrier when the alternatives
  ||| have different domains or modifiers". A head noun and a bare modifier
  ||| are not interchangeable ("artifact or attacking",
  ||| `badHeadlessDisjunct`); neither are a hand card and a battlefield
  ||| permanent, because the phrase places its referent ONCE (the corpus
  ||| writes cross-zone alternatives under a shared preposition — "an
  ||| Equipment card from your hand or graveyard" — which is a zone
  ||| disjunction the single-valued projection cannot carry, so it is
  ||| refused here and ledgered rather than mis-projected,
  ||| `badCrossZoneDisjunction`); and neither is an alternative that
  ||| COMMITS beside one that stays silent. The comparison is on the SEEDS
  ||| themselves, silence included, not on the defaults they fall back to:
  ||| reading both through [CR#109.2]'s battlefield made "attacking
  ||| artifact or land" look parallel, and the disagreement then reappeared
  ||| as a projection of NOTHING (`badPartialZoneJoin`). Agreement here is
  ||| what makes the joins honest downstream: a `Nothing` out of
  ||| `seedZoneJoin` now means every alternative was silent.
  public export
  parallelDisjuncts : {0 bs : Bindings} -> {0 k : Kind} ->
                      List (Predicate bs k) -> Bool
  parallelDisjuncts [] = True
  parallelDisjuncts (p :: ps) = headsUniform (hasHead p) ps &&
                                seedsUniform (seedZone p) (seedType p) ps

  public export
  data ParallelDisjuncts : List (Predicate bs k) -> Type where
    MkParallelDisjuncts : {auto 0 ok : parallelDisjuncts ps = True} ->
                          ParallelDisjuncts ps

  public export
  isOr : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  isOr (Or _) = True
  isOr _ = False

  public export
  anyIsOr : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyIsOr [] = False
  anyIsOr (p :: ps) = isOr p || anyIsOr ps

  ||| May this predicate stand as ONE alternative? Three words cannot, and
  ||| for one reason between them: each is written once for the whole
  ||| phrase, so putting it on one side of an "or" spells something the
  ||| phrase already said. "Any target" is ITSELF the class [CR#115.4]
  ||| defines by disjunction — "creatures, players, planeswalkers, or
  ||| battles" — so coordinating it re-opens a closed union
  ||| (`badAnyTargetInOr`); "other" fills its one selector slot for the
  ||| coordination entire ("Another target Wolf or Werewolf you control",
  ||| `badOtherInOr`); and a disjunction inside a disjunction is the flat
  ||| coordination written with brackets oracle has no way to print
  ||| (`badNestedOr`) — core reaches the same shape by flattening
  ||| associatively in `normalize`, where the workbench refuses the second
  ||| spelling outright. The scan reads each alternative through
  ||| `flattenPs`, so a singleton conjunction launders none of them.
  public export
  coordinable : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  coordinable p = not (headIsAnyTarget p) &&
                  not (hasOther p) &&
                  not (anyIsOr (flattenPs [p]))

  public export
  coordinableAll : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  coordinableAll [] = True
  coordinableAll (p :: ps) = coordinable p && coordinableAll ps

  public export
  data CoordinableDisjuncts : List (Predicate bs k) -> Type where
    MkCoordinableDisjuncts : {auto 0 ok : coordinableAll ps = True} ->
                             CoordinableDisjuncts ps

  public export
  anyPredEq : {0 bs : Bindings} -> {0 k : Kind} ->
              Predicate bs k -> List (Predicate bs k) -> Bool
  anyPredEq p [] = False
  anyPredEq p (q :: qs) = predEq p q || anyPredEq p qs

  ||| No alternative repeats another: "artifact or artifact" offers no
  ||| alternative at all and spells what the bare word spells, which is
  ||| the refusal the arity gate already makes at one member
  ||| (`badSingletonOr`, `badRepeatedDisjunct`) — one meaning, one
  ||| spelling. Conservative exactly where `predEq` is.
  public export
  noRepeatedPair : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  noRepeatedPair [] = True
  noRepeatedPair (p :: ps) = not (anyPredEq p ps) && noRepeatedPair ps

  public export
  data DistinctDisjuncts : List (Predicate bs k) -> Type where
    MkDistinctDisjuncts : {auto 0 ok : noRepeatedPair ps = True} ->
                          DistinctDisjuncts ps

  ||| Which modifiers a phrase can negate — ATOMIC rows only. Oracle's
  ||| negation words (non-, isn't, doesn't) attach to ONE modifier, so a
  ||| conjunction is negated per-member in English and De Morgan is the
  ||| writer's job, not the grammar's: `Not (And …)` is unwritable
  ||| (`badNegatedConjunction`), which also stops a singleton `And`
  ||| laundering every ban below. A DISJUNCTION is refused on the same
  ||| evidence and more sharply: no corpus line spells "non-(A or B)" at
  ||| all, while the De Morgan the writer does instead is plentiful and
  ||| comma-chained ("noncreature, nonland card"), so `Not (Or …)` is
  ||| unwritable (`badNegatedDisjunction`). "Any target" and "other" are
  ||| not negatable — the class word is never negated ([CR#115.4] defines
  ||| it positively) and "non-other" is unwritten — and neither is the
  ||| universal player word, which names one of the people in the game
  ||| ([CR#102.1]) and so has no complement class inside the kind. Nor is a
  ||| negation itself negated: oracle spells no double negative.
  public export
  negatable : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  negatable (HasType _) = True
  -- "target attacking Vampire that isn't a Demon" (Clavileño, First of
  -- the Blessed): the subtype word negates as freely as the type word.
  negatable (HasSubtype _) = True
  negatable AnyPlayer = False
  negatable Opponent = True
  negatable (QualityNoun _) = True
  negatable (OfChosen _) = True
  negatable (HasKeyword _) = True
  negatable (ControlledBy _) = True
  -- the cast relation does NOT negate, where the control clause does:
  -- "creatures you don't control" is 165 lines and "spells you didn't
  -- cast" is zero. English negates the casting with an AGENTLESS passive
  -- instead ("target spell you control that wasn't cast", Errant, Street
  -- Artist), which is a construction this grammar has at neither clause,
  -- so the cell is refused and its sentence is ledgered rather than
  -- pinned -- the agentive phrase is unwritten, but what it would MEAN is
  -- attested one card over.
  negatable (CastBy _) = False
  -- the linkage read does NOT negate, and the measurement is total:
  -- "not exiled with" is zero corpus lines against a hundred
  -- seventy-five positive ones. A card outside the group is not
  -- described by the group's own phrase -- the family's complements are
  -- written as other zones or other descriptions instead
  -- (`badNegatedExiledWith`).
  negatable (ExiledWith _) = False
  negatable Attacking = True
  negatable Blocking = True
  -- the RELATIONAL rows do not negate, and the measurement is total the
  -- way `ExiledWith`'s is: "not blocking [m]" and "not blocked by [m]"
  -- are zero corpus lines apiece. The two negations the family does
  -- write are the bare designation's — "target nonattacking, nonblocking
  -- creature" and "if it wasn't blocking" — which is the row above and
  -- stays True. A creature outside the relation is described by another
  -- phrase, not by this one's complement (`badNegatedBlockedBy`).
  negatable (BlockerOf _) = False
  negatable (BlockedBy _) = False
  -- the history read DOES negate, and on the auxiliary exactly as it does
  -- at the condition surface: "all untapped creatures that didn't attack
  -- this turn", "each creature that didn't enter this turn" -- six lines,
  -- and the two surfaces agreeing on where English puts the "not" is a
  -- result rather than an assumption.
  negatable (HappenedTo _ _) = True
  -- the three PREFIX negations, each written and each measured:
  -- "nonblack" 77 lines and its four siblings, "nonlegendary" 43 and
  -- "nonbasic" 68, "not named" 4. The three colour-COUNT words take no
  -- negation at all -- "noncolorless", "nonmulticolored" and
  -- "nonmonocolored" are zero lines apiece -- so they refuse it.
  negatable (ColorIs _) = True
  negatable IsColorless = False
  negatable Multicolored = False
  negatable Monocolored = False
  negatable (HasSupertype _) = True
  negatable (Named _) = True
  -- none of the three designation reads negates: "isn't the monarch",
  -- "isn't your Ring-bearer" and "nongoaded" are zero lines apiece, and
  -- the one negative the family writes -- "there is no monarch" -- is the
  -- absence check, another construction.
  negatable (HasPlayerDesignation _) = False
  negatable IsGoaded = False
  negatable YourRingBearer = False
  -- the attachment presence reads DO negate: four isn't-forms against the
  -- thirty-three positives.
  negatable IsEnchanted = True
  negatable IsEquipped = True
  -- "nonpermanent" was not measured by this round's recon, and the
  -- status pairs are each two words, never a "non-" of each other
  -- ([CR#110.5] names both values) — so only the token row negates
  -- ("nontoken", two hundred twenty-two lines).
  negatable Permanent = False
  negatable IsToken = True
  negatable (HasStatus _) = False
  negatable (Compare _ _ _) = False
  negatable (InZone _) = True
  negatable (And _) = False
  negatable (Or _) = False
  negatable (Not _) = False
  negatable Other = False
  negatable (OtherThan _) = False
  negatable AnyTarget = False

  public export
  data Negatable : Predicate bs k -> Type where
    MkNegatable : {auto 0 ok : negatable p = True} -> Negatable p

  ||| Does this phrase SAY anything? Every atomic modifier does, by
  ||| being a word; a conjunction says what its members say, so the
  ||| EMPTY one says nothing at all. `Headed` is the stronger demand the
  ||| determiner slots make and the copular condition cannot — "if it's
  ||| attacking" and "if it's tapped" head nothing and are real oracle —
  ||| so the weaker one is its own table (`badMatchesNothing`). A
  ||| coordination is already two alternatives deep (`TwoDisjuncts`) and
  ||| a negation says what it negates. Full rows.
  public export
  predSays : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  predSays (HasType _) = True
  predSays (HasSubtype _) = True
  predSays AnyPlayer = True
  predSays Opponent = True
  predSays (QualityNoun _) = True
  predSays (OfChosen _) = True
  predSays (HasKeyword _) = True
  predSays (ControlledBy _) = True
  predSays (CastBy _) = True
  predSays (ExiledWith _) = True
  predSays Attacking = True
  predSays Blocking = True
  predSays (BlockerOf _) = True
  predSays (BlockedBy _) = True
  predSays (HappenedTo _ _) = True
  predSays (ColorIs _) = True
  predSays IsColorless = True
  predSays Multicolored = True
  predSays Monocolored = True
  predSays (HasSupertype _) = True
  predSays (Named _) = True
  predSays (HasPlayerDesignation _) = True
  predSays IsGoaded = True
  predSays YourRingBearer = True
  predSays IsEnchanted = True
  predSays IsEquipped = True
  predSays Permanent = True
  predSays IsToken = True
  predSays (HasStatus _) = True
  predSays (Compare _ _ _) = True
  predSays (InZone _) = True
  predSays (And ps) = predSaysAny ps
  predSays (Or _) = True
  predSays (Not p) = predSays p
  predSays Other = True
  predSays (OtherThan _) = True
  predSays AnyTarget = True

  public export
  predSaysAny : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  predSaysAny [] = False
  predSaysAny (p :: ps) = predSays p || predSaysAny ps

  public export
  data PredSays : Predicate bs k -> Type where
    MkPredSays : {auto 0 ok : predSays p = True} -> PredSays p

  ||| Is this phrase free of negation, at any depth a combinator
  ||| reaches? The condition frames take a "not" of their own
  ||| ([CR#701.47a] writes one — "If it isn't a [subtype], …"), and what
  ||| they may not take it of is a phrase that already carries one:
  ||| "if it isn't a non-artifact" is a negation of a negation however
  ||| the two are spelled, and the corpus writes none
  ||| (`badNegatedNegativeMatch`). This is `negatable (Not _) = False`,
  ||| the predicate layer's atomic-only negation, said one construction
  ||| up — where the inner negation can hide inside a conjunction that
  ||| the predicate layer's own row never sees. Full rows.
  public export
  predNegFree : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  predNegFree (HasType _) = True
  predNegFree (HasSubtype _) = True
  predNegFree AnyPlayer = True
  predNegFree Opponent = True
  predNegFree (QualityNoun _) = True
  predNegFree (OfChosen _) = True
  predNegFree (HasKeyword _) = True
  predNegFree (ControlledBy _) = True
  predNegFree (CastBy _) = True
  predNegFree (ExiledWith _) = True
  predNegFree Attacking = True
  predNegFree Blocking = True
  predNegFree (BlockerOf _) = True
  predNegFree (BlockedBy _) = True
  predNegFree (HappenedTo _ _) = True
  predNegFree (ColorIs _) = True
  predNegFree IsColorless = True
  predNegFree Multicolored = True
  predNegFree Monocolored = True
  predNegFree (HasSupertype _) = True
  predNegFree (Named _) = True
  predNegFree (HasPlayerDesignation _) = True
  predNegFree IsGoaded = True
  predNegFree YourRingBearer = True
  predNegFree IsEnchanted = True
  predNegFree IsEquipped = True
  predNegFree Permanent = True
  predNegFree IsToken = True
  predNegFree (HasStatus _) = True
  predNegFree (Compare _ _ _) = True
  predNegFree (InZone _) = True
  predNegFree (And ps) = predNegFreeAll ps
  predNegFree (Or ps) = predNegFreeAll ps
  predNegFree (Not _) = False
  predNegFree Other = True
  predNegFree (OtherThan _) = True
  predNegFree AnyTarget = True

  public export
  predNegFreeAll : {0 bs : Bindings} -> {0 k : Kind} ->
                   List (Predicate bs k) -> Bool
  predNegFreeAll [] = True
  predNegFreeAll (p :: ps) = predNegFree p && predNegFreeAll ps

  ||| Does no part of this phrase spell "any target"? The class word is
  ||| ITSELF the targeting form — "a any target" and "each any target" are
  ||| unwritable — so only the targeting determiners admit it. The scan
  ||| reaches through EMBEDDED nouns, not just sibling predicates: a
  ||| relative clause's possessor ("a creature the controller of any target
  ||| controls") and an owned zone's possessor spell the class word just as
  ||| loudly under a non-targeting determiner (`badAnyTargetEmbedded`). The
  ||| Pred → Noun → Pred descent is structural, so it terminates.
  public export
  anyTargetFree : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  anyTargetFree (HasType _) = True
  anyTargetFree (HasSubtype _) = True
  anyTargetFree AnyPlayer = True
  anyTargetFree Opponent = True
  anyTargetFree (QualityNoun _) = True
  anyTargetFree (OfChosen _) = True
  anyTargetFree (HasKeyword _) = True
  anyTargetFree (ControlledBy n) = nounAnyTargetFree n
  anyTargetFree (CastBy n) = nounAnyTargetFree n
  anyTargetFree (ExiledWith n) = nounAnyTargetFree n
  anyTargetFree Attacking = True
  anyTargetFree Blocking = True
  anyTargetFree (BlockerOf m) = nounAnyTargetFree m
  anyTargetFree (BlockedBy m) = nounAnyTargetFree m
  anyTargetFree (HappenedTo _ _) = True
  anyTargetFree (ColorIs _) = True
  anyTargetFree IsColorless = True
  anyTargetFree Multicolored = True
  anyTargetFree Monocolored = True
  anyTargetFree (HasSupertype _) = True
  anyTargetFree (Named _) = True
  anyTargetFree (HasPlayerDesignation _) = True
  anyTargetFree IsGoaded = True
  anyTargetFree YourRingBearer = True
  anyTargetFree IsEnchanted = True
  anyTargetFree IsEquipped = True
  anyTargetFree Permanent = True
  anyTargetFree IsToken = True
  anyTargetFree (HasStatus _) = True
  -- the bound used to be a numeral or the announced X, neither of which
  -- carries a noun for the class word to hide in; that warrant expired
  -- when the bound admitted reads, and the row re-answers the question
  -- through the amount exactly as the comment there promised it would.
  anyTargetFree (Compare _ _ b) = amtAnyTargetFree b
  anyTargetFree (InZone z) = zoneAnyTargetFree z
  anyTargetFree (And ps) = anyTargetFreeAll ps
  -- the alternatives are scanned exactly as conjuncts are: the class
  -- word may not be an alternative at all (`CoordinableDisjuncts`),
  -- but a possessor buried inside one still spells it out loud.
  anyTargetFree (Or ps) = anyTargetFreeAll ps
  anyTargetFree (Not p) = anyTargetFree p
  anyTargetFree Other = True
  -- the anchor is a noun, and a noun can bury the class word in a
  -- possessor exactly as a relative clause does.
  anyTargetFree (OtherThan n) = nounAnyTargetFree n
  anyTargetFree AnyTarget = False

  public export
  anyTargetFreeAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyTargetFreeAll [] = True
  anyTargetFreeAll (p :: ps) = anyTargetFree p && anyTargetFreeAll ps

  public export
  data AnyTargetFree : Predicate bs k -> Type where
    MkAnyTargetFree : {auto 0 ok : anyTargetFree p = True} -> AnyTargetFree p

  ||| A description that names no ZONE of its own — what a clause
  ||| supplying the place demands of the phrase it places. The search is
  ||| the one such clause today ([CR#701.23a] puts the zone on the verb),
  ||| and a description carrying its own would answer the question twice
  ||| (`badSearchZonedDescription`).
  public export
  data ZoneFree : Predicate bs k -> Type where
    MkZoneFree : {auto 0 ok : seedZone p = Nothing} -> ZoneFree p

  ||| May a counted target mention spell the class word? The answer is the
  ||| QUANTITY's, not the determiner's, and the line it draws is a MINIMUM
  ||| rather than a maximum. [CR#115.4] names "any target," "another
  ||| target," "two targets," "or similar" as one family, and the corpus
  ||| writes every plural member of it with a range that starts at one and
  ||| leaves the count to the caster ("up to two targets", "one or two
  ||| targets", "one, two, or three targets", "any number of targets").
  ||| What it never writes is a FIXED plural count — "two targets" as a
  ||| phrase of its own is zero lines, and so is "among two targets" —
  ||| because the class word's plural forms all belong to a structure that
  ||| lets the caster pick how many things to hit ([CR#601.2c] has a
  ||| variable target count announced before the targets; [CR#601.2d] has
  ||| the division announced with it). So the gate asks for a minimum of
  ||| one or none at all, and the one refusal left is the exact group from
  ||| two up (`badGroupAnyTarget`).
  |||
  ||| The unbounded quantity used to be refused beside it, on the ground
  ||| that its structure was unbuilt; the division built it, and
  ||| Boulderfall's "deals 5 damage divided as you choose among any number
  ||| of targets" is the positive that retired the ban. What stops a plural
  ||| class-word mention from standing as a BARE recipient is no longer
  ||| this gate but `PerMember`, which asks the question where it belongs —
  ||| of the clause that writes the amount. What a permitting quantity
  ||| licenses is the class word as the phrase's HEAD, never the class word
  ||| wherever it turns up: a possessor buried in a relative clause spells
  ||| "any target" under a counted mention exactly as loudly as under "a"
  ||| (`badEmbeddedAnyTargetExact1`), so the embedded scan runs beneath
  ||| every quantity. The head case needs no scan of its own —
  ||| `AnyTargetLone` allows the class word no companion but "other".
  public export
  anyTargetOkAt : {0 bs : Bindings} -> {0 k : Kind} ->
                  Quantity -> Predicate bs k -> Bool
  anyTargetOkAt (Range Nothing _) p = headIsAnyTarget p || anyTargetFree p
  anyTargetOkAt (Range (Just (S Z)) _) p = headIsAnyTarget p || anyTargetFree p
  anyTargetOkAt (Range (Just _) _) p = anyTargetFree p

  public export
  data AnyTargetAtCount : Quantity -> Predicate bs k -> Type where
    MkAnyTargetAtCount : {auto 0 ok : anyTargetOkAt q p = True} ->
                         AnyTargetAtCount q p

  public export
  zoneOr : Zone -> Maybe Zone -> Zone
  zoneOr z Nothing = z
  zoneOr z (Just w) = w

  ||| Build the binding a determined mention introduces: projections of
  ||| the phrase only. Total over the PHRASAL kinds — outcomes have no
  ||| determiner phrase, which the `Phrasal` witness enforces. The zone
  ||| recorded is the phrase's own, so the class word records none, on
  ||| the same ground `nounZone` gives it none: a later read of an
  ||| any-target mention is no better placed than the mention was
  ||| (`badDestroyAnyTargetRemention`).
  public export
  bindFor : Determiner -> Plurality -> {k : Kind} -> Phrasal k -> Predicate bs k -> Binding
  bindFor det plur PhObject p =
    MkBinding det Object plur
              (ObjectP (seedTy p)
                       (if headIsAnyTarget p
                          then Nothing
                          else Just (zoneOr Battlefield (seedZone p)))
                       Nothing Nothing)
  bindFor det plur PhPlayer p = MkBinding det Player plur PlayerP
  bindFor det plur {k = Quality q} PhQuality p = MkBinding det (Quality q) plur QualityP

  ||| A noun in its argument position — the determiner layer of the
  ||| phrase, deciding how (and whether) the referent enters the
  ||| discourse.
  public export
  data Noun : Bindings -> Kind -> Type where
    -- spelling: ["~"], kind: Nominal (matches constructors.ron's `This`
    -- entry exactly -- nullary self-reference sigil)
    This : Noun bs Object       -- the source, by self-name or "this spell" [CR#113.7]
    -- the TYPE ASCRIPTION: a noun read under a card-type word, which is
    -- what "this artifact"/"this land"/"this creature" is -- `This`'s
    -- identity plus a type noun, two axes and not one constructor. Core
    -- keeps identity type-free (`Reference::This` carries nothing,
    -- `reference.rs`), so the type word is this layer's business, and the
    -- projections it earns are the type word's: it names the head type,
    -- and a description including a card type and no zone/card/spell/
    -- source word denotes a PERMANENT ([CR#109.2]) -- so the ascribed
    -- phrase stands on the battlefield where the bare source ("this
    -- spell", cycling's "Discard this card" [CR#702.29a]) stands nowhere
    -- the grammar tracks. Unmoved it introduces what its argument
    -- introduces; MOVED it mints a fresh binding -- the move makes it a
    -- new object [CR#400.7], which is why cost-position "Sacrifice this
    -- artifact" leaves a referent the effect's "It" can read ([CR#400.7j]
    -- is the exception letting the effect find it). WHICH nouns take an
    -- ascription is a closed table (`Ascribable`).
    -- spelling: (construction-owned -- the card-type word read over its
    -- argument's own phrase; at the source that is "this <Param(0)>",
    -- constructors.ron's `This` sigil under a type word, and the
    -- macros own it: thisCreature/thisArtifact/thisEnchantment/thisLand),
    -- kind: Nominal
    AsType : (t : CardType) -> (n : Noun bs Object) ->
             {auto 0 asc : Ascribable n} -> Noun bs Object
    -- spelling: ["you"], kind: Nominal (matches constructors.ron's `You`
    -- entry exactly)
    You : Noun bs Player        -- "you" [CR#109.5]
    -- "players" / "your opponents" -- the PLURAL PLAYER subject, deferred
    -- since chapter thirty-nine and the blocker four families have paid
    -- for. It is `You`'s plural neighbour in every respect that matters:
    -- a fixed word naming game participants, carrying no description to
    -- gate and announcing nothing.
    -- EXOPHORIC (`nounDelta` is the empty list), and that is measured
    -- rather than assumed. Across all 140 corpus segments with one of
    -- these subjects, no later clause pronominalises it: the "they" and
    -- "their" that follow are the possessive agreeing INSIDE the same
    -- clause ("players skip their untap steps", "your opponents can't
    -- cast spells from anywhere other than their hands"), and where a
    -- second sentence names the group again it REPEATS the noun
    -- (Everybody Lives!, "Players gain hexproof… Players can't lose
    -- life…"). The corpus's fourteen "those players" all read back a
    -- counted target mention, a quantifier phrase or a relational noun,
    -- and none of them a subject of this shape. So a binding here would
    -- mint a read nothing writes, which is `You`'s own answer and the
    -- attachment host's: these are [CR#102.1]'s people and their
    -- opponents, named the way [CR#109.5] names "you", so there is
    -- nothing for a later phrase to pick out.
    -- PLURAL always, and the number is what keeps the union relative
    -- clause honestly out: "creatures your opponents control" is 386
    -- lines and stays unwritable, because `ControlledBy` demands a
    -- SINGULAR possessor on [CR#109.4]'s one controller. The plural
    -- possessor is a relation this row does not supply (ledger).
    -- spelling: ["players", "your opponents"] (Param(0) = PlayerGroupWord's
    -- own word -- see PlayerGroupWord), kind: Nominal
    PlayerGroup : (w : PlayerGroupWord) -> Noun bs Player
    -- the NON-targeting determiners each demand an `AnyTargetFree`
    -- phrase: "any target" is itself the targeting form, so "a any
    -- target" / "each any target" are unwritable ([CR#115.4]).
    -- spelling: ["each <Param(0)>"], kind: Nominal
    Each : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
           {auto 0 hd : Headed p} ->
           {auto 0 af : AnyTargetFree p} -> Noun bs k    -- "each …": a group, resolution-time [CR#608.2]
    -- "a …": the indefinite -- one determiner ([CR#608.2d,400.7]), whose
    -- CHOICE METHOD is a separate axis the text marks or leaves unmarked
    -- (`ChoiceMode`; chooser and method are surface facts, and a random
    -- discard has no chooser [CR#701.9b]). Three constructors spelled that
    -- one determiner three times; core spells it once and hangs the
    -- chooser off a slot (`Binder::ChooseOne`'s `by`, `binder.rs`). The
    -- mode's own obligations ride the mode, not this row.
    -- spelling: (construction-owned -- the indefinite article over its
    -- phrase, followed by whatever adverbial its mode writes; the macros
    -- own the surfaces: a/aTheirChoice/aAtRandom), kind: Nominal
    Indefinite : (m : ChoiceMode bs) -> (p : Predicate bs k) ->
                 {auto ph : Phrasal k} ->
                 {auto 0 hd : Headed p} ->
                 {auto 0 af : AnyTargetFree p} -> Noun bs k
    -- "[quantity] target [pred]": the counted target mention -- one
    -- binding, announced [CR#601.2c], and only objects and players are
    -- targetable ([CR#115.1] -- `badTargetColor`). ONE constructor serves
    -- every quantity, mirroring core's single announce form
    -- `TargetSpec::Target(Quantity, Predicate)`. That quantity is ARITY
    -- data: the phrase's grammatical number reads it (`quantPlur`), it
    -- permits at least one (`badZeroGroup`) and runs upward from one
    -- (`badDescendingRange`, `badZeroLowerRange`), and at exactly one the
    -- phrase IS the singular "target [noun]" -- the `target` macro, whose
    -- numeral rendering leaves unwritten. Distinctness stays announce
    -- business ([CR#601.2c]); the quantity never encodes it.
    -- spelling: [(text: "target <Param(1)>", when: [(0, "Exactly(1)")])],
    -- kind: Nominal (mirrors constructors.ron's `Target` entry exactly:
    -- params ["Quantity","Predicate"], the same Exactly(1) guard; wider
    -- quantities spell their own numeral via Quantity's macros, e.g.
    -- "<Param(0)> target <Param(1)>")
    TargetGroup : (q : Quantity) -> (p : Predicate bs k) ->
                  {auto tk : Targetable k} -> {auto 0 nz : NonZeroQ q} ->
                  {auto 0 wf : WellFormedQ q} ->
                  {auto 0 hd : Headed p} ->
                  {auto 0 af : AnyTargetAtCount q p} -> Noun bs k
    -- "all [pred]s": the set-level group — a surface determiner the
    -- guide keeps distinct from distributive "each" (the CR fixes both
    -- sets at resolution and separates them no further).
    -- spelling: ["all <Param(0)>"], kind: Nominal (auto-inflects plural)
    AllOf : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
            {auto 0 hd : Headed p} ->
            {auto 0 af : AnyTargetFree p} -> Noun bs k
    -- "each of [group]": the distributive determiner over a GROUP MENTION
    -- rather than over a description -- "Put a +1/+1 counter on each of up
    -- to two target creatures" (Ajani, Adversary of Tyrants). `Each`
    -- distributes over whatever answers a phrase; this distributes over
    -- the members of one mention already made, so it announces nothing of
    -- its own and contributes its complement's binding unchanged. The
    -- complement is exactly the group forms the corpus writes after the
    -- word (`GroupMention`): a counted target mention or a plural read. It
    -- is NOT a second distributive ("each of each creature") and not the
    -- universal ("each of all creatures"), neither of which is written.
    -- The phrase stays PLURAL -- its mention is the group's, and the next
    -- sentence reads it as one (Sparkmage's Gambit). What the determiner
    -- buys is the per-member reading of the clause's own amount: the
    -- sentence writes "a counter" and every member gets one, which is what
    -- `PerMember` demands of a recipient and what a bare plural mention
    -- cannot say.
    -- spelling: ["each of <Param(0)>"], kind: Nominal
    EachOf : (grp : Noun bs k) ->
             {auto 0 pl : nounPlur grp = ManyOf} ->
             {auto 0 gm : GroupMention grp} -> Noun bs k
    -- "the top [n] card(s) of [whose] library" -- the POSITIONED SLICE, a
    -- definite description whose members are fixed by where they lie
    -- rather than by what they are ([CR#401.2]: a library is a single
    -- face-down pile, so a position picks out cards and says nothing about
    -- them). The corpus writes the top slice in quantity against ONE line
    -- for the bottom (Grenzo, Dungeon Warden's "the bottom card of your
    -- library") -- an asymmetry worth stating, because the bottom is the
    -- dominant DESTINATION and almost never a source.
    -- It projects NO card type, and that is the honesty this row exists to
    -- keep: the cards are in a hidden zone ([CR#400.2]) whose order and
    -- contents no player may inspect ([CR#401.2]), so the phrase describes
    -- a place and the grammar may not pretend to know what is there. Every
    -- type-demanding verb refuses it for free as a result, and the zone
    -- demand refuses it twice over (`badTapLibraryTop`). The count is the
    -- ordinary magnitude vocabulary: "the top card" is `Lit 1`, "the top X
    -- cards of your library" is `XVal`.
    -- spelling: ["the <Param(0)> <Param(1)> card(s) of <Param(2)>'s library"]
    -- (Param(0) = LibPos's own word "top"/"bottom"; the noun "card"
    -- pluralises with the count, and at one the numeral is unwritten --
    -- "the top card", never "the top one card"), kind: Nominal
    LibrarySlice : (pos : LibPos) -> (amt : Amount bs) ->
                   (whose : Noun bs Player) ->
                   {auto 0 one : nounPlur whose = OneOf} ->
                   {auto 0 wc : WrittenCount amt} -> Noun bs Object
    -- "[q] of [group]" -- the PARTITIVE determiner: a selection of some
    -- members out of a group mention already made ("Put one of them into
    -- your hand"). It is `EachOf`'s sibling and its opposite in the one
    -- way that matters: the distributive reaches every member and
    -- announces nothing, this one PICKS members and so announces the
    -- mention it picked -- which is what makes the complement beside it
    -- ("the rest") a subtraction with something to subtract. It inherits
    -- the group's head type and zone, because a part of a group is in the
    -- group's place and answers the group's description; what it does not
    -- inherit is the group's number, which is its own quantity's
    -- (`quantPlur`).
    -- spelling: ["<Param(0)> of <Param(1)>"] (Param(0) = the Quantity's own
    -- numeral or bound, e.g. "one", "two", "up to one"), kind: Nominal
    SomeOf : (q : Quantity) -> (grp : Noun bs Object) ->
             {auto 0 gm : GroupMention grp} ->
             {auto 0 nz : NonZeroQ q} ->
             {auto 0 wf : WellFormedQ q} -> Noun bs Object
    -- "the rest": the group COMPLEMENT -- the members of an assembled
    -- group that the preceding instruction did not take. The corpus writes
    -- it as a complement in quantity (a further nineteen lines write the
    -- unrelated temporal idiom "for the rest of the game"), and almost all
    -- of those subtract from a library slice a look or a reveal or an
    -- exile-from-the-top assembled.
    -- This is finding 111's subtract-a-subset half, and what unblocks it
    -- is not new counting machinery but the GROUP: two "target" instances
    -- are two mentions and never a pair ([CR#601.2c], finding 127), while
    -- a slice phrase names one group whose members a partitive then
    -- divides. So the gate asks for exactly that pair -- one group
    -- mention, and at least one part taken from it (`theRestOk`;
    -- `badRestWithoutGroup`, `badRestWithoutPart`). The pair complement
    -- over never-assembled mentions stays refused.
    -- spelling: ["the rest"], kind: Nominal
    TheRest : {auto 0 ok : theRestOk bs = True} -> Noun bs Object
    -- "it" / "its": the wildcard pronoun — exactly one singular Object
    -- mention may precede. Zero = unbound, two = ambiguous; both
    -- unspellable.
    -- spelling: ["it"] (possessive "its"), kind: Nominal
    It : {auto 0 ok : countOnes Object bs = 1} -> Noun bs Object
    -- "they" for a player (singular; player groups are a later chapter).
    -- spelling: ["they"], kind: Nominal (singular epicene)
    They : {auto 0 ok : countOnes Player bs = 1} -> Noun bs Player
    -- "them": the plural wildcard — exactly one group mention of the
    -- kind may precede (the ManyOf twin of `It`).
    -- spelling: ["them"], kind: Nominal
    Them : {auto 0 ok : countManys Object bs = 1} -> Noun bs Object
    -- "those [noun word]s": the sorted plural demonstrative — exactly
    -- one group mention its word currently reaches.
    -- spelling: ["those <Param(0)>"], kind: Nominal (Param(0) = NounWord's
    -- own word -- see NounWord)
    Those : (w : NounWord) -> {auto 0 ok : countManyWord w bs = 1} -> Noun bs (kindOfW w)
    -- "that [noun word]": the sorted demonstrative — exactly one
    -- mention its word currently reaches may precede (`wordNow` — the
    -- current-state anchoring; the participle read anchors the same
    -- words to the verb event instead).
    -- spelling: ["that <Param(0)>"], kind: Nominal (Param(0) = NounWord's
    -- own word -- see NounWord)
    That : (w : NounWord) -> {auto 0 ok : countWord w bs = 1} -> Noun bs (kindOfW w)
    -- "enchanted creature" / "equipped creature" / "fortified land" /
    -- "enchanted player" -- the ATTACHMENT HOST, a permanent's pointer to
    -- whatever it is attached to. See AttachWord for the rules that define
    -- the three participles and for why this needs no attach relation.
    -- EXOPHORIC, and both prior arts agree: like `This` it names the game
    -- situation rather than reading a mention, so `nounDelta` is the empty
    -- list -- core's reference carries no announcement and the old
    -- module's `refIntro` passes straight through.
    -- Its EVIDENCE is the head word, which is `AsType`'s arrangement:
    -- there a type word placed the self on the battlefield ([CR#109.2]),
    -- and here the head word places the host and names its type, so
    -- "enchanted creature" projects a battlefield creature and "enchanted
    -- player" projects nothing at all. The kind follows the head too
    -- (`kindOfW`), which is `That`'s own indexing and what lets the player
    -- host share the row instead of forking it.
    -- SINGULAR always: an Aura or Equipment is attached to one thing
    -- ([CR#303.4b,301.5a]).
    -- spelling: ["<Param(0)> <Param(1)>"] (the participle and its head
    -- word -- see AttachWord and NounWord)
    AttachHost : (w : AttachWord) -> (h : NounWord) ->
                 {auto 0 ok : AttachHeadOk w h} -> Noun bs (kindOfW h)
    -- "the [verbed] [noun]" ("the exiled card", "the sacrificed
    -- artifact"): the definite participle read -- exactly one mention
    -- stamped by that verb tag and reached by the noun word may precede.
    -- The disambiguator real text switches to where a bare demonstrative
    -- would be ambiguous (finding 26). Two axes kept apart as queries: the
    -- PROVENANCE picks the mention (`stampedBy`) and the WORD describes it
    -- (`verbedWordOk`), the pair being what the read's uniqueness counts.
    -- The word fixes the phrase's kind exactly as it does for
    -- `That`/`Those`.
    -- spelling: ["the <Param(0)> <Param(1)>"] (Param(0) = VerbName's lemma,
    -- rendered as its past participle by auto-inflection; Param(1) =
    -- NounWord's own word), kind: Nominal
    -- The MARKING is a defaulted slot over `verbedMarkingOk`'s table:
    -- one read, two surfaces, and the table is what keeps `Destroy`'s
    -- deictic-only spelling from being spelled the other way.
    TheVerbed : (v : VerbName) -> (w : NounWord) ->
                {default Attributive marking : VerbedMarking} ->
                {auto 0 ok : countVerbed v w bs = 1} ->
                {auto 0 mk : VerbedMarkingOk v marking} -> Noun bs (kindOfW w)
    -- "[the] [noun]s [verbed] this way": the participle read's PLURAL
    -- twin, and the row the "this way" family mostly lives in. It is to
    -- `Them` what `TheVerbed` is to `That` — the marked read English
    -- switches to when a bare plural pronoun would not say which group —
    -- and it carries the same two axes and the same strict uniqueness,
    -- counted over `ManyOf` bindings (`countManyVerbed`).
    -- spelling: ["the <Param(1)>s <Param(0)>", "<Param(1)>s <Param(0)>
    -- this way"] (the marking decides; the head noun pluralises, and the
    -- definite article is written or not by the frame the phrase stands
    -- in -- "You may play cards exiled this way" writes none)
    ThoseVerbed : (v : VerbName) -> (w : NounWord) ->
                  {default Attributive marking : VerbedMarking} ->
                  {auto 0 ok : countManyVerbed v w bs = 1} ->
                  {auto 0 mk : VerbedMarkingOk v marking} -> Noun bs (kindOfW w)
    -- "[object]'s controller" / "its owner": relational nouns — a NEW
    -- player referent derived from a SINGULAR object mention
    -- ([CR#108.3,109.4]; a group's owners need the plural relational,
    -- "their owners' hands" — future vocabulary, `badGroupOwner`).
    -- spelling: ["<Param(0)>'s controller"], kind: Nominal
    ControllerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Noun bs Player
    -- spelling: ["<Param(0)>'s owner"], kind: Nominal
    OwnerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Noun bs Player

  ||| Referent equality between two possessor nouns — deliberately the
  ||| SMALLEST honest relation, and the reason `predEq`'s `ControlledBy`
  ||| row is no longer vacuous. True only for the atomic words whose
  ||| referent the binding context already fixes ("you" is you, "it" is the
  ||| unique singular object), so two occurrences inside ONE phrase denote
  ||| the same thing. Everything else is False, INCLUDING two target
  ||| mentions: [CR#601.2c] chooses each "target" instance separately, so
  ||| two of them may denote different objects and must never be equated.
  ||| (Declared after `Noun` because a function's TYPE is elaborated in
  ||| source order even inside `mutual`.)
  public export
  nounEqRef : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Noun bs k -> Bool
  nounEqRef This This = True
  nounEqRef This _ = False
  nounEqRef (AsType _ _) _ = False
  nounEqRef You You = True
  nounEqRef You _ = False
  -- the plural words are atomic and their referents are the game's own:
  -- two occurrences of "your opponents" in one phrase denote one set,
  -- exactly as two occurrences of "you" denote one player.
  nounEqRef (PlayerGroup v) (PlayerGroup w) = samePlayerGroupWord v w
  nounEqRef (PlayerGroup _) _ = False
  nounEqRef (Each _) _ = False
  nounEqRef (Indefinite _ _) _ = False
  nounEqRef (TargetGroup _ _) _ = False
  nounEqRef (AllOf _) _ = False
  nounEqRef (EachOf _) _ = False
  nounEqRef (LibrarySlice _ _ _) _ = False
  nounEqRef (SomeOf _ _) _ = False
  nounEqRef TheRest _ = False
  nounEqRef It It = True
  nounEqRef It _ = False
  nounEqRef They They = True
  nounEqRef They _ = False
  nounEqRef Them _ = False
  nounEqRef (Those _) _ = False
  nounEqRef (That _) _ = False
  -- conservative, `That`'s answer: the row's two arguments are closed
  -- words and could be compared, but no consumer needs the distinction and
  -- `nounEqRef`'s False is "not provably the same".
  nounEqRef (AttachHost _ _) _ = False
  nounEqRef (TheVerbed _ _) _ = False
  nounEqRef (ThoseVerbed _ _) _ = False
  nounEqRef (ControllerOf _) _ = False
  nounEqRef (OwnerOf _) _ = False

  ||| The noun side of the "any target" scan: does no phrase embedded
  ||| in this noun spell the class word? Determined mentions carry a
  ||| predicate to check; the atomic words and the reads carry none —
  ||| a read's antecedent already passed the scan where it was written.
  ||| Full rows, so a new noun form declares its answer.
  public export
  nounAnyTargetFree : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounAnyTargetFree This = True
  nounAnyTargetFree (AsType t n) = nounAnyTargetFree n
  nounAnyTargetFree You = True
  nounAnyTargetFree (PlayerGroup _) = True
  nounAnyTargetFree (Each p) = anyTargetFree p
  nounAnyTargetFree (Indefinite m p) = anyTargetFree p
  nounAnyTargetFree (TargetGroup _ p) = anyTargetFree p
  nounAnyTargetFree (AllOf p) = anyTargetFree p
  nounAnyTargetFree (EachOf grp) = nounAnyTargetFree grp
  nounAnyTargetFree (LibrarySlice _ _ whose) = nounAnyTargetFree whose
  nounAnyTargetFree (SomeOf _ grp) = nounAnyTargetFree grp
  nounAnyTargetFree TheRest = True
  nounAnyTargetFree It = True
  nounAnyTargetFree They = True
  nounAnyTargetFree Them = True
  nounAnyTargetFree (Those _) = True
  nounAnyTargetFree (That _) = True
  nounAnyTargetFree (AttachHost _ _) = True
  nounAnyTargetFree (TheVerbed _ _) = True
  nounAnyTargetFree (ThoseVerbed _ _) = True
  nounAnyTargetFree (ControllerOf n) = nounAnyTargetFree n
  nounAnyTargetFree (OwnerOf n) = nounAnyTargetFree n

  ||| …and the zone side: an owned zone's possessor is a noun like any
  ||| other ("a card in the hand of the controller of any target").
  public export
  zoneAnyTargetFree : {0 bs : Bindings} -> ZoneExpr bs -> Bool
  zoneAnyTargetFree (ZoneAt z Bare) = True
  zoneAnyTargetFree (ZoneAt z (OwnedBy n)) = nounAnyTargetFree n
  zoneAnyTargetFree (LibraryAt _ _ Bare) = True
  zoneAnyTargetFree (LibraryAt _ _ (OwnedBy n)) = nounAnyTargetFree n

  ||| Destination legality for the move primitive ([CR#400.3] — cards enter
  ||| only their owner's hand/library/graveyard, so an owned destination
  ||| naming an arbitrary player is unwritable, and the bare zone IS the
  ||| owner-rooted destination; the possessive is rendering's business,
  ||| finding 34).
  |||
  ||| The LIBRARY row is the position phrase and never the bare zone: a
  ||| library is ordered ([CR#401.2]), so "put it into your library" names
  ||| no place to put it and oracle never writes it — every corpus library
  ||| placement spells a position. `ZoneAt Library _` therefore has no row
  ||| at all (`badMoveToBareLibrary`), which is core's `exclude(Library)`
  ||| on `Destination`. The position takes the bare scope like every other
  ||| destination, [CR#400.3] routing the card to its owner's library
  ||| whatever the sentence's possessive says (finding 34).
  public export
  data DestOk : ZoneExpr bs -> Type where
    BattlefieldOk : DestOk (ZoneAt Battlefield Bare)
    ExileOk : DestOk (ZoneAt Exile Bare)
    HandOkBare : DestOk (ZoneAt Hand Bare)
    GraveyardOkBare : DestOk (ZoneAt Graveyard Bare)
    LibraryPosOk : DestOk (LibraryAt pos arrg Bare)

  ||| May THIS patient take THIS destination's order rider? [CR#401.4] asks
  ||| the question and answers it: an arrangement exists only when an
  ||| effect "puts TWO OR MORE cards in a specific position in a library at
  ||| the same time", so a singular placement has no order to state.
  ||| English agrees exactly — the order riders follow a plural patient
  ||| every time they appear and a singular one zero times
  ||| (`badSingularOrderRider`). The absent rider is legal at either
  ||| number: [CR#401.4]'s own default for a group, vacuous for one card.
  public export
  orderOk : {0 bs : Bindings} -> Plurality -> ZoneExpr bs -> Bool
  orderOk pl z = case zoneArrangement z of
                   Nothing => True
                   Just _ => not (isOne pl)

  public export
  data ArrangementOk : {0 bs : Bindings} -> Plurality -> ZoneExpr bs -> Type where
    MkArrangementOk : {auto 0 ok : orderOk pl z = True} -> ArrangementOk pl z

  ||| The bindings a noun phrase prepends to the discourse — its own
  ||| head first (determined mentions bind, reads don't), then the
  ||| mentions its predicate introduces in textual order: relative
  ||| clauses FOLD, so "target creature an opponent controls" leaves
  ||| both the creature and the opponent readable.
  public export
  nounDelta : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List Binding
  nounDelta This = []
  nounDelta (AsType t n) = nounDelta n
  nounDelta You = []
  nounDelta (PlayerGroup _) = []
  nounDelta (Each p {ph}) = bindFor EachD ManyOf ph p :: predDelta p
  nounDelta (Indefinite m p {ph}) = bindFor AD OneOf ph p :: predDelta p
  nounDelta (TargetGroup q p {tk}) = bindFor TargetD (quantPlur q) (targetablePhrasal tk) p :: predDelta p
  nounDelta (AllOf p {ph}) = bindFor AllD ManyOf ph p :: predDelta p
  -- the determiner announces NOTHING of its own: "each of them" and
  -- "each of up to two target creatures" contribute the complement's
  -- binding and no second one ([CR#601.2c] announced the targets when
  -- the group was written; the word "each" adds no mention).
  nounDelta (EachOf grp) = nounDelta grp
  -- the slice is a DEFINITE description that binds: it names cards the
  -- following sentence reads back ("Look at the top four cards of your
  -- library. Put one of them …"), and the possessor folds out with it as
  -- every relative clause's does. The payload carries the library and NO
  -- head type -- see the constructor.
  nounDelta (LibrarySlice pos amt whose {wc}) =
    MkBinding TheD Object (amtPlur amt) (ObjectP Nothing (Just Library) Nothing Nothing)
      :: nounDelta whose
  -- the partitive announces the part it picked, inheriting the group's
  -- place and description and taking its number from its own quantity.
  nounDelta (SomeOf q grp) =
    MkBinding PartD Object (quantPlur q) (ObjectP (nounTy grp) (nounZone grp) Nothing Nothing)
      :: nounDelta grp
  nounDelta TheRest = []
  nounDelta It = []
  nounDelta They = []
  nounDelta Them = []
  nounDelta (That w) = []
  nounDelta (AttachHost _ _) = []
  nounDelta (Those w) = []
  nounDelta (TheVerbed v w) = []
  nounDelta (ThoseVerbed v w) = []
  nounDelta (ControllerOf n) = MkBinding TheD Player OneOf PlayerP :: nounDelta n
  nounDelta (OwnerOf n) = MkBinding TheD Player OneOf PlayerP :: nounDelta n

  ||| The mentions a predicate's clauses introduce, textual order.
  public export
  predDelta : {bs : Bindings} -> {k : Kind} -> Predicate bs k -> List Binding
  predDelta (ControlledBy n) = nounDelta n
  predDelta (CastBy n) = nounDelta n
  predDelta (BlockerOf m) = nounDelta m
  predDelta (BlockedBy m) = nounDelta m
  predDelta (HappenedTo _ _) = []
  predDelta (ColorIs _) = []
  predDelta IsColorless = []
  predDelta Multicolored = []
  predDelta Monocolored = []
  predDelta (HasSupertype _) = []
  predDelta (Named _) = []
  predDelta (HasPlayerDesignation _) = []
  predDelta IsGoaded = []
  predDelta YourRingBearer = []
  predDelta IsEnchanted = []
  predDelta IsEquipped = []
  predDelta (InZone z) = zoneDelta z
  predDelta (And ps) = predDeltaAll ps
  -- negation is a binding HOLE: a positive controller relation names
  -- the one controller ([CR#109.4]); its negation selects nobody, so
  -- nothing inside `Not` folds out (`badNegatedAntecedent`).
  predDelta (Not p) = []
  -- a disjunction is a hole for the neighbouring reason: only ONE
  -- alternative is realized, and the phrase does not say which, so a
  -- possessor written inside one of them names nobody the discourse
  -- can read back (`badDisjunctAntecedent`). The phrase's OWN binding
  -- is unaffected — the determiner makes it at the noun layer, which
  -- is why "Destroy target artifact or enchantment" still leaves an
  -- "it" behind.
  predDelta (Or ps) = []
  -- the anchored complement carries its ANCHOR's announcement, chapter
  -- twenty-six. The anchor is usually a read and contributes nothing,
  -- which is why this row was invisible under the old blanket demand; the
  -- two lines that write a target there (Death by Dragons) announce it
  -- like any other target phrase ([CR#601.2c]).
  predDelta (OtherThan n) = nounDelta n
  -- the comparison's bound may be a READ since chapter fifty-seven, and a
  -- read names something ("power greater than target creature's power"),
  -- so the qualifier carries what its standard announces exactly as the
  -- anchored complement carries its anchor's. Every written bound answers
  -- with the empty list, so nothing that composed before moves.
  predDelta (Compare _ _ b) = amtDelta b
  predDelta _ = []

  public export
  predDeltaAll : {bs : Bindings} -> {k : Kind} -> List (Predicate bs k) -> List Binding
  predDeltaAll [] = []
  predDeltaAll (p :: ps) = predDelta p ++ predDeltaAll ps

  public export
  zoneDelta : {bs : Bindings} -> ZoneExpr bs -> List Binding
  zoneDelta (ZoneAt z (OwnedBy n)) = nounDelta n
  zoneDelta (ZoneAt z Bare) = []
  zoneDelta (LibraryAt _ _ (OwnedBy n)) = nounDelta n
  zoneDelta (LibraryAt _ _ Bare) = []

  ||| What a noun contributes to the discourse that follows it.
  public export
  nomIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  nomIntro n = nounDelta n ++ bs

  ||| An amount expression — where "equal to its power" lives, so it
  ||| threads like everything else.
  public export
  data Amount : Bindings -> Type where
    -- spelling: ["<Param(0)>"] (bare numeral), kind: TODO(reason: amount
    -- fragment -- not one of Nominal/Sentence/Cost/KeywordLine/Ability)
    Lit : Nat -> Amount bs
    -- "[its/…] power" / "toughness" / "mana value" ([CR#208.1,202.3]):
    -- one object's own numbers, so the argument is singular -- a group's
    -- aggregate is written with a fold word instead and is the `Aggregate`
    -- row below. What stays refused HERE is the bare plural possessive
    -- ("two creatures' power"), which the corpus never writes
    -- (`badGroupPower`); Soulblast's "the total power of the sacrificed
    -- creatures" is written and still unreachable, its domain being a
    -- MENTION rather than a description (ledger).
    -- WHICH number is read is `Characteristic`, not a row of its own:
    -- core reads every stat through one `Count::StatOf(Reference, Stat)`
    -- (`count.rs`). The characteristic leads the arguments as it does in
    -- `Compare` (core orders the pair the other way; the axis is the same).
    -- spelling: ["<Param(1)>'s <Param(0)>"] (Param(0) = Characteristic's
    -- own word, its construction-owned catalog entry), kind: TODO(reason:
    -- amount fragment, see Lit)
    -- "[whose] life total" / "[whose] starting life total" ([CR#119.1]) --
    -- the PLAYER's own number, `StatOf`'s twin at the other participant
    -- sort and the row core was built expecting by name (see PlayerStat).
    -- Singular for the same reason `StatOf` is: one player has one life
    -- total, and the plural read is written ZERO times as a plain amount
    -- -- the eight corpus lines with "life totals" in them are all the
    -- EXCHANGE family ([CR#701.12c]), which swaps two and reads neither
    -- (`badPluralLifeTotalRead`). The
    -- subject slot takes any singular player noun, and the corpus uses
    -- that breadth: "your life total" throughout, "that player's life
    -- total" at Sengir, "an opponent's life total" at the comparisons.
    -- It is a READ (`readAmount`), which is the whole unlock: the
    -- threshold family's 31 lines are `CompareAmt` against a written
    -- bound and needed nothing but a left-hand side.
    -- The read-vs-read comparison ("more life than an opponent", 24
    -- supported lines) was the one thing this row could not reach when it
    -- landed, and chapter fifty-seven reaches it: the bound admits a read
    -- (`ComparableBound`), so both sides of the comparison are this
    -- vocabulary's. The CROSS-PLAYER read ("the highest life total among
    -- all players") is not this row either -- it folds a set and lives at
    -- `Aggregate`, whose player-sorted axis is this catalog.
    -- spelling: ["<Param(1)>'s <Param(0)>"] (Param(0) = PlayerStat's own
    -- word; the threshold frame rewrites the whole comparison as "[whose]
    -- have N or more life" -- see PlayerStat), kind: TODO(reason: amount
    -- fragment, see Lit)
    PlayerStatOf : (w : PlayerStat) -> (n : Noun bs Player) ->
                   {auto 0 one : nounPlur n = OneOf} -> Amount bs
    StatOf : (c : Characteristic) -> (n : Noun bs Object) ->
             {auto 0 one : nounPlur n = OneOf} -> Amount bs
    -- the CARDINALITY of a set the phrase describes -- "the number of
    -- cards in your hand", and the domain half of every for-each amount;
    -- mentions inside the predicate fold as everywhere. Core keeps this
    -- orthogonal to multiplication (`Count::CountOf` against
    -- `Count::Times`, `count.rs`) and so does this. The domain is a real
    -- noun phrase: every corpus for-each domain is noun-HEADED.
    -- spelling: ["the number of <Param(0)>"] (the bare nominal read;
    -- the for-each adverbial is the `forEach`/`nForEach` macros' own
    -- surface over the same amount), kind: TODO(reason: amount
    -- fragment, see Lit)
    CountOf : {k : Kind} -> (p : Predicate bs k) ->
              {auto 0 hd : Headed p} ->
              {auto 0 af : AnyTargetFree p} -> Amount bs
    -- "the greatest [characteristic] among [described set]" / "the total
    -- [characteristic] of [described set]" / "the lowest life total among
    -- [described players]" -- the FOLD, and the set-generalization the
    -- singular reads beside it have promised since they were minted: those
    -- rows read ONE thing's number and said a group's aggregate was written
    -- explicitly and was future vocabulary. This is that vocabulary. A
    -- hundred and three supported lines write an extreme over a described
    -- set and twenty-seven write the sum.
    -- The DOMAIN is a DESCRIPTION and not a mention, which is `CountOf`'s
    -- slot exactly -- core shares one `Countable` between its cardinality
    -- and its fold (`count.rs`) and so does this, the same head and
    -- any-target demands for the same reasons and the same bare-plural
    -- spelling. The corpus's MENTION domains are a second surface this row
    -- does not reach ("the total power of the sacrificed creatures",
    -- Soulblast; "the greatest power among them"), nineteen lines between
    -- the two families, and they want a fold over a noun (ledger).
    -- No CR rule DEFINES the construction. The rules use it -- a Saga's
    -- final chapter number is "the greatest value among chapter abilities
    -- it has" ([CR#714.2d]) -- without ever saying what it means, so this
    -- is ordinary English composition over numbers the rules already
    -- track, and what the CR supplies is the empty case ([CR#107.2], see
    -- `AggregateOp`).
    -- It is a READ (`readAmount`), and the corpus writes it at a
    -- comparison's standard as well as at an instruction's amount ("X
    -- can't be greater than the greatest toughness among creatures you
    -- control", Soul Immolation).
    -- spelling: ["<Param(0)'s word> <Param(1)> <Param(0)'s preposition>
    -- <Param(2)>"] -- the fold's word and preposition are the op's and vary
    -- with the axis (see AggregateOp), giving "the greatest power among
    -- creatures you control" and "the total power of creatures you
    -- control". The corpus also writes an optional "all" before the
    -- complement ("among all players", 4 lines against 103), which is one
    -- fold under two spellings -- chapter forty-five's bare-each ruling at
    -- another site -- and this rendering writes the bare plural, kind:
    -- TODO(reason: amount fragment, see Lit)
    Aggregate : {k : Kind} -> (op : AggregateOp) -> (ax : ProjAxis) ->
                (p : Predicate bs k) ->
                {auto 0 sc : projScope ax = k} ->
                {auto 0 hd : Headed p} ->
                {auto 0 af : AnyTargetFree p} -> Amount bs
    -- "the number of [kind] counters on [n]" / "the number of [kind]
    -- counters [who] has" -- the counter READ, 296 lines for the object
    -- spelling and 9 for the player one, and ONE row for both because
    -- round thirty-nine's lesson applies exactly: the two spellings vary
    -- perfectly with the holder's SORT and nothing else, so the
    -- difference is an agreement fact and not an axis. Indexed at
    -- `{k : Kind}` like `Matches` and `DealDamage`, with the kind's own
    -- scope tied to the holder's sort by the one table
    -- (`counterScope kind = k`) -- which is why `counterScope` answers in
    -- `Kind` rather than in a scope enum of its own, the agreement coming
    -- out for free instead of being written twice.
    -- The holder is SINGULAR, `StatOf`'s demand for `StatOf`'s reason: a
    -- read names one thing's counters, and the group aggregate is written
    -- explicitly ("the number of counters among creatures you control", 6
    -- lines) -- the `Aggregate` fold's shape at an axis that catalog does
    -- not carry, and four of the six write the kind-blind quantifier
    -- besides (ledger).
    -- The name is core's (`Count::CountersOn`, `count.rs`). It is also the
    -- name the OLD `Semantics` module's own counter read carries, and the
    -- two never meet -- that module is a separate family this one imports
    -- nothing from -- so this is a name straddle across two independent
    -- vocabularies and not a conflict.
    -- spelling: ["the number of <Param(0)> counters on <Param(1)>" (an
    -- object holder), "the number of <Param(0)> counters <Param(1)>
    -- has/have" (a player holder)] (Param(0) = CounterKind's own word;
    -- the possessive verb agrees with the holder phrase's own number),
    -- kind: TODO(reason: amount fragment, see Lit)
    CountersOn : {k : Kind} -> (kind : CounterKind) -> (holder : Noun bs k) ->
                 {auto 0 sc : counterScope kind = k} ->
                 {auto 0 one : nounPlur holder = OneOf} -> Amount bs
    -- "the number of times [who] [past-verb] [w]" -- the history read's
    -- COUNT-valued twin, core's `Count::EventCount(EventFilter, Lookback)`
    -- beside `Condition::Happened` for the reason core states: "EventCount
    -- counts; EventSum sums". The same three arguments and the same
    -- attestation table, because the two rows ask one question and differ
    -- only in what they answer with.
    -- It is what the NUMERIC lookbacks route through: "if you've drawn two
    -- or more cards this turn" is this amount under `CompareAmt`, exactly
    -- as chapter forty's counter read feeds the same comparison, and no
    -- comparison vocabulary is added.
    -- What it does NOT do is SUM. Core keeps `EventSum` apart for the
    -- magnitude reads -- "if you gained 3 or more life this turn", fifteen
    -- lines, where the number wanted is the life and not the number of
    -- gainings -- and this vocabulary has no row for that yet; the
    -- unnumbered form ("if you gained life this turn", eighteen lines) is
    -- `Happened`'s and lands. The sum is ledgered.
    -- spelling: ["the number of times <Param(1)> <past verb for Param(0)>
    -- <Param(2)>"] (the verb and its voice are `Happened`'s -- see there;
    -- the corpus more often writes the count INSIDE the subject phrase
    -- ("two or more cards") than as a fronted nominal, which is the
    -- comparison's own linearisation and not this row's), kind:
    -- TODO(reason: amount fragment, see Lit)
    EventCount : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
                 (w : Lookback) ->
                 {auto 0 sb : LookbackSubject ev k} -> Amount bs
    -- "[per] times [amt]" -- the scaling half, whose left factor is a
    -- WRITTEN numeral. Core's multiplication is count-by-count
    -- (`Count::Times(Count, Count)`; no CR rule licenses a product, so the
    -- shape answers to the corpus alone), and every corpus multiplication
    -- writes a numeral against a phrase instead -- "twice the number of
    -- cards in your hand", the per-unit of a for-each line -- with no
    -- phrase-by-phrase product written anywhere, so the numeral is the
    -- factor's type and generalizing it waits on a line that needs it. A
    -- written numeral is at least one (`AtLeastOne`, `badForEachZero`),
    -- which is finding 45's discipline unmoved: the comparisons that
    -- legitimately carry zero READ a count rather than write one, and
    -- `Lit` stays ungated.
    -- spelling: ["<Param(0)> times <Param(1)>"] (and "twice" for the
    -- factor two — the numeral's own word; the for-each line spells the
    -- same term with the count's adverbial instead, see `nForEach`),
    -- kind: TODO(reason: amount fragment, see Lit)
    Times : (per : Nat) -> (a : Amount bs) ->
            {auto 0 nz : AtLeastOne per} -> Amount bs
    -- "that much": reads the unique event outcome in scope — the
    -- magnitude of what an earlier clause DID. Sort-blind (the
    -- corpus reads cross damage→life, count→life, damage→mana);
    -- the value is runtime, never stored (the §3 ruling).
    -- spelling: ["that much"], kind: TODO(reason: amount fragment, see Lit)
    ThatMuch : {auto 0 ok : countOnes Outcome bs = 1} -> Amount bs
    -- "the difference" -- the MARGIN by which a comparison held, read by
    -- the statement that comparison licensed ("if you have fewer than
    -- seven cards in hand, draw cards equal to the difference"). Twenty-six
    -- supported lines, and `ThatMuch`'s shape at the second magnitude
    -- sort: a uniqueness gate over the telescope, the anaphor standing
    -- where the arithmetic would otherwise be repeated. What it denotes is
    -- `Minus` of the comparison's two sides, exactly as "that much"
    -- denotes an amount an earlier clause wrote -- which is why the
    -- vocabulary needs both rows and neither replaces the other.
    -- It is POSITIVE by the licence: the condition was true, so the larger
    -- side is the larger side and [CR#107.1b]'s floor is never reached
    -- from here. The gap is minted by `condIntro` at every comparison and
    -- read by nothing else.
    -- spelling: ["the difference"], kind: TODO(reason: amount fragment,
    -- see Lit)
    TheDifference : {auto 0 ok : countOnes Gap bs = 1} -> Amount bs
    -- "X" -- the letter a rider DEFINED, read back by the body it binds
    -- ([CR#107.3c]: where the text defines the value, that is the value
    -- while the spell or ability is on the stack and the controller does
    -- not choose it). ELEVEN HUNDRED AND EIGHTEEN supported cards, the
    -- largest single family this workbench has met, and it is the same
    -- shape as the two anaphors beside it: a uniqueness gate over the
    -- telescope, the letter standing where the amount would otherwise be
    -- repeated.
    -- It is a DIFFERENT ROW from `XVal` and the two rules say why:
    -- [CR#107.3a] is the X announced with a cost, whose value the
    -- controller chooses, and [CR#107.3c] is the X the text defines,
    -- whose value the controller may not. One letter, two rules, two
    -- rows -- and the corpus keeps them apart too: of the 1,118 rider
    -- cards exactly ONE also has {X} in its cost (Riptide Replicator,
    -- whose cast-time X sets the counters it enters with while its
    -- activated ability's rider defines a second X off those counters --
    -- [CR#107.3c] fixes the ability's value from its own text and
    -- [CR#107.3m] leaves the permanent's own X at zero, so the two never
    -- meet).
    -- The WORD is the row's argument and not a second row, which is
    -- [CR#107.3p] in the shape the corpus writes it: Y "follows the same
    -- rules as X", the three cards that use it define both letters in one
    -- clause, and the uniqueness each read demands is asked of its own
    -- word (`countLetter`). One row, two words, two mentions.
    -- spelling: ["<Param(0)>"] (Param(0) = the letter's own word, "X" or
    -- "Y"), kind: TODO(reason: amount fragment, see Lit)
    DefinedLetter : (w : LetterWord) ->
                    {auto 0 ok : countLetter w bs = 1} -> Amount bs
    -- "X" — announced with the cost ([CR#107.3a,107.3i]): a fixed
    -- value by resolution, not a discourse referent.
    -- spelling: ["X"], kind: TODO(reason: amount fragment, see Lit)
    XVal : Amount bs
    -- "[a] plus [b]" — the second operand reads after the first.
    -- spelling: ["<Param(0)> plus <Param(1)>"], kind: TODO(reason: amount
    -- fragment, see Lit)
    Plus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs
    -- "[a] minus [b]" -- the DIRECTIONAL difference, `Plus`'s twin in every
    -- part including the threading order, and floored at zero: a
    -- calculation that would yield a negative number uses zero instead
    -- ([CR#107.1b], whose carve-out is for doubling, tripling and SETTING a
    -- life total or a creature's power and toughness -- none of which is an
    -- amount). Core is `Count::Minus(a, b)` with the same floor stated in
    -- the same words (`deckmaste_core/src/count.rs`), saturating in the
    -- evaluator with a test that asserts two minus five is zero
    -- (`resolve/count.rs`), and -- the decisive part -- the plugin's Idris
    -- emitter already writes `app("Minus", [a, b])` toward this file
    -- (`deckmaste_plugin/src/idris_emit.rs`), so this is a second name core
    -- held in trust, `PlayerStatOf`'s situation one vocabulary over.
    -- The corpus writes thirty-four supported lines and eighteen of them
    -- ride the "where X is …" definition rider, which is a different gap
    -- and a large one (ledger).
    -- spelling: ["<Param(0)> minus <Param(1)>"], kind: TODO(reason: amount
    -- fragment, see Lit)
    Minus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs

  ||| What an amount contributes to the discourse — `nounDelta`'s twin one
  ||| vocabulary over, and `amtIntro`'s delta. Only the reads contribute
  ||| anything, and only through the noun or description they name. It is
  ||| written apart from `amtIntro` rather than re-keyed onto it because
  ||| the sum EXTENDS in the other order (its second operand reads after
  ||| the first), and that order is `amtIntro`'s own fact.
  public export
  amtDelta : {bs : Bindings} -> Amount bs -> List Binding
  amtDelta (Lit _) = []
  amtDelta (StatOf _ nom) = nounDelta nom
  amtDelta (PlayerStatOf _ nom) = nounDelta nom
  amtDelta (CountersOn _ holder) = nounDelta holder
  amtDelta (EventCount _ who _) = nounDelta who
  amtDelta (CountOf p) = predDelta p
  amtDelta (Aggregate _ _ p) = predDelta p
  amtDelta (Times _ a) = amtDelta a
  amtDelta ThatMuch = []
  amtDelta TheDifference = []
  amtDelta (DefinedLetter _) = []
  amtDelta XVal = []
  amtDelta (Plus a b) = amtDelta a ++ amtDelta b
  amtDelta (Minus a b) = amtDelta a ++ amtDelta b

  ||| …and the amount side, which chapter fifty-seven owes: a comparison's
  ||| bound may now be a READ, and a read names something ("power greater
  ||| than target creature's power"), so the class word can hide there
  ||| exactly as it hides in a possessor. The written rows carry no noun
  ||| and answer True for that reason and no other.
  public export
  amtAnyTargetFree : {0 bs : Bindings} -> Amount bs -> Bool
  amtAnyTargetFree (Lit _) = True
  amtAnyTargetFree (StatOf _ n) = nounAnyTargetFree n
  amtAnyTargetFree (PlayerStatOf _ n) = nounAnyTargetFree n
  amtAnyTargetFree (CountersOn _ holder) = nounAnyTargetFree holder
  amtAnyTargetFree (EventCount _ who _) = nounAnyTargetFree who
  amtAnyTargetFree (CountOf p) = anyTargetFree p
  amtAnyTargetFree (Aggregate _ _ p) = anyTargetFree p
  amtAnyTargetFree (Times _ a) = amtAnyTargetFree a
  amtAnyTargetFree ThatMuch = True
  amtAnyTargetFree TheDifference = True
  amtAnyTargetFree (DefinedLetter _) = True
  amtAnyTargetFree XVal = True
  amtAnyTargetFree (Plus a b) = amtAnyTargetFree a && amtAnyTargetFree b
  amtAnyTargetFree (Minus a b) = amtAnyTargetFree a && amtAnyTargetFree b

  public export
  amtIntro : {bs : Bindings} -> Amount bs -> Bindings
  amtIntro (Lit n) = bs
  amtIntro (StatOf c nom) = nomIntro nom
  amtIntro (PlayerStatOf w nom) = nomIntro nom
  amtIntro (CountersOn _ holder) = nomIntro holder
  amtIntro (EventCount _ who _) = nomIntro who
  amtIntro (CountOf p) = predDelta p ++ bs
  amtIntro (Aggregate _ _ p) = predDelta p ++ bs
  amtIntro (Times per a) = amtIntro a
  amtIntro ThatMuch = bs
  amtIntro TheDifference = bs
  amtIntro (DefinedLetter _) = bs
  amtIntro XVal = bs
  amtIntro (Plus a b) = amtIntro b
  amtIntro (Minus a b) = amtIntro b

  ||| The grammatical number a COUNTED creation writes, read off its
  ||| amount — `quantPlur`'s twin one vocabulary over. Exactly one is
  ||| singular; every other written amount is a group, the plural noun the
  ||| corpus writes with it ("create that many … tokens"), whatever it
  ||| evaluates to at resolution.
  public export
  amtPlur : {0 bs : Bindings} -> Amount bs -> Plurality
  amtPlur (Lit (S Z)) = OneOf
  amtPlur (Lit _) = ManyOf
  amtPlur (StatOf _ _) = ManyOf
  amtPlur (PlayerStatOf _ _) = ManyOf
  amtPlur (CountersOn _ _) = ManyOf
  amtPlur (EventCount _ _ _) = ManyOf
  amtPlur (CountOf _) = ManyOf
  amtPlur (Aggregate _ _ _) = ManyOf
  amtPlur (Times _ _) = ManyOf
  amtPlur ThatMuch = ManyOf
  amtPlur TheDifference = ManyOf
  amtPlur (DefinedLetter _) = ManyOf
  amtPlur XVal = ManyOf
  amtPlur (Plus _ _) = ManyOf
  amtPlur (Minus _ _) = ManyOf

  ||| Is this amount WRITTEN as a value — the numeral, or the X announced
  ||| with the cost ([CR#107.3a])? Half of the bound test
  ||| (`comparableBound`) and, with `readAmount`, the whole of the
  ||| comparator's SPELLING: the same relation follows a written bound
  ||| ("2 or less") and precedes a read ("less than or equal to the number
  ||| of lands you control"), and neither word order is ever written for
  ||| the other class. That is a fact about which words a sentence
  ||| chooses, not about which comparisons exist, which is why it is a
  ||| Bool here and a gate nowhere.
  public export
  writtenBound : {0 bs : Bindings} -> Amount bs -> Bool
  writtenBound (Lit _) = True
  writtenBound (StatOf _ _) = False
  writtenBound (PlayerStatOf _ _) = False
  writtenBound (CountersOn _ _) = False
  writtenBound (EventCount _ _ _) = False
  writtenBound (CountOf _) = False
  writtenBound (Aggregate _ _ _) = False
  writtenBound (Times _ _) = False
  writtenBound ThatMuch = False
  writtenBound TheDifference = False
  writtenBound (DefinedLetter _) = False
  writtenBound XVal = True
  writtenBound (Plus _ _) = False
  writtenBound (Minus _ _) = False

  ||| A count English WRITES is at least one. Drawing, creating, and
  ||| counter-placing all spell their number, and no corpus line spells it
  ||| zero — not as a numeral ("draw zero cards"), not as a determiner
  ||| ("create no tokens"), in any scope. [CR#121.1] makes a draw the
  ||| movement of a card, [CR#111.1] a token a marker put onto the
  ||| battlefield, and [CR#122.1] a counter a marker placed on something; a
  ||| zero of any of them instructs nothing (`badDrawZero`,
  ||| `badCreateZero`, `badPutZeroCounters`).
  |||
  ||| It is the LITERAL SPELLING that is refused and not the value. An
  ||| amount that is READ can evaluate to zero and stay perfectly written —
  ||| X's value is its controller's to choose and announce ([CR#107.3a]), a
  ||| for-each domain can be empty, "that much" can be nothing — so those
  ||| rows pass and only the written numeral is asked to be positive. That
  ||| is why `Lit` itself stays ungated: a bound of zero is a real
  ||| comparison ("with mana value 0 or less" measures rather than
  ||| instructs). A sum is written where BOTH its operands are.
  public export
  writtenCount : {0 bs : Bindings} -> Amount bs -> Bool
  writtenCount (Lit Z) = False
  writtenCount (Lit (S _)) = True
  writtenCount (StatOf _ _) = True
  writtenCount (PlayerStatOf _ _) = True
  writtenCount (CountersOn _ _) = True
  writtenCount (EventCount _ _ _) = True
  writtenCount (CountOf _) = True
  writtenCount (Aggregate _ _ _) = True
  writtenCount (Times _ a) = writtenCount a
  writtenCount ThatMuch = True
  writtenCount TheDifference = True
  writtenCount (DefinedLetter _) = True
  writtenCount XVal = True
  writtenCount (Plus a b) = writtenCount a && writtenCount b
  -- the floor is what makes the difference a count at all: a subtraction
  -- that would go negative is zero, and zero is a written count's refusal
  -- only when it is SPELLED (`writtenCount (Lit Z)`), never when it is
  -- computed ([CR#107.1b]; finding 45's discipline unmoved).
  writtenCount (Minus a b) = writtenCount a && writtenCount b

  public export
  data WrittenCount : Amount bs -> Type where
    MkWrittenCount : {auto 0 ok : writtenCount a = True} -> WrittenCount a

  ||| Two bounds, compared as written. Conservative in `predEq`'s
  ||| direction and for its reason: the catch-all reads "not provably the
  ||| same value". Since chapter fifty-seven a READ can reach here too,
  ||| and it falls to that catch-all rather than to a structural
  ||| comparison — two qualifiers standing on the same read are not
  ||| provably the same bound, and the only consumer is a repetition
  ||| check that under-refuses safely.
  public export
  boundEq : {0 bs : Bindings} -> Amount bs -> Amount bs -> Bool
  boundEq (Lit a) (Lit b) = a == b
  boundEq XVal XVal = True
  boundEq _ _ = False

  ||| Life-total change operands ([CR#119.3]), and the SET-TO beside the
  ||| two deltas. The third row is chapter twenty-two's ledgered
  ||| player-attribute set arriving: the corpus writes "life total becomes"
  ||| ("Your life total becomes 10"), and it is the operand the life
  ||| EXCHANGE would need — [CR#701.12c] has each player "gain or lose the
  ||| amount of life necessary to equal the other player's previous life
  ||| total", which is a set realized as whichever direction reaches it
  ||| (finding 121).
  |||
  ||| That is also why the set contributes no OUTCOME mention where the
  ||| deltas each contribute one: "that much" reads a magnitude a clause
  ||| produced, and a set-to produces a gain for one player and a loss for
  ||| another depending on where their total stood. The sort is not
  ||| knowable from the sentence, so the clause writes none rather than
  ||| guessing — finding 121's asymmetry, structural.
  public export
  data LifeOp : Bindings -> Type where
    -- spelling: (construction-owned -- selects ChangeLife's verb "gains",
    -- not independently spelled; mirrors constructors.ron's `GainLife`
    -- entry, body ChangeLife(Param(0), Up(Param(1))))
    Up : Amount bs -> LifeOp bs     -- "gains [amt] life"
    -- spelling: (construction-owned -- selects ChangeLife's verb "loses";
    -- the `LosesLife`/Down counterpart per the GainLife comment above)
    Down : Amount bs -> LifeOp bs   -- "loses [amt] life"
    -- spelling: (construction-owned -- selects ChangeLife's copula frame,
    -- "<Param(0)>'s life total becomes <Param(1)>"; core keeps the set
    -- beside the deltas likewise)
    Set : Amount bs -> LifeOp bs    -- "[whose] life total becomes [amt]"

  public export
  lifeIntro : {bs : Bindings} -> LifeOp bs -> Bindings
  lifeIntro (Up a) = amtIntro a
  lifeIntro (Down a) = amtIntro a
  lifeIntro (Set a) = amtIntro a

  ||| May this amount stand as a comparison's SUBJECT — the thing measured,
  ||| on the left of "is"? A comparison MEASURES on the left, so a written
  ||| value cannot stand there: "if 3 is 4 or greater" states an arithmetic
  ||| fact and not a fact about the game (`badCompareLiteralSubject`), and
  ||| the announced X is written likewise (`badCompareXSubject`). This is
  ||| the one half of the frame where the divergence from core survives
  ||| chapter fifty-seven — core's `Compare(Count, Cmp, Count)` restricts
  ||| neither side, and the BOUND side is now unrestricted here too
  ||| (`comparableBound`).
  ||| The refusal of X is measured at the frame this vocabulary writes: X
  ||| against a written bound is zero lines. Thassa's Oracle does write X
  ||| as a subject once, against a READ ("if X is greater than or equal to
  ||| the number of cards in your library"), and that line is unwritable
  ||| for its devotion read anyway — an over-refusal recorded rather than
  ||| a claim that the sentence does not exist.
  public export
  readAmount : {0 bs : Bindings} -> Amount bs -> Bool
  readAmount (Lit _) = False
  readAmount (StatOf _ _) = True
  readAmount (PlayerStatOf _ _) = True
  readAmount (CountersOn _ _) = True
  readAmount (EventCount _ _ _) = True
  readAmount (CountOf _) = True
  readAmount (Aggregate _ _ _) = True
  readAmount (Times _ _) = False
  readAmount ThatMuch = False
  readAmount TheDifference = False
  readAmount (DefinedLetter _) = False
  readAmount XVal = False
  readAmount (Plus _ _) = False
  readAmount (Minus _ _) = False

  public export
  data ReadAmount : Amount bs -> Type where
    MkReadAmount : {auto 0 ok : readAmount a = True} -> ReadAmount a

  ||| May this amount stand as a comparison's BOUND — the standard the
  ||| measurement is held against? Anything the sentence NAMES: a written
  ||| value ("2 or less") or a read of the game state ("less than or equal
  ||| to the number of lands you control"). The two classes are the two
  ||| SPELLINGS and not two grammars, which is chapter fifty-seven's
  ||| finding and the reason this is one test over both frames rather than
  ||| a comparator-by-class table: all eight cells of relation × class are
  ||| attested.
  |||
  ||| What is refused is what a sentence never names at a standard: the
  ||| scaled product ("twice the number of cards in your hand"), the sum,
  ||| and the event-outcome read all appear as bounds ZERO times
  ||| (`badScaledBound`, `badScaledConditionBound`) — they are amounts a
  ||| clause COMPUTES for an instruction, and a comparison's standard is
  ||| something already there to be pointed at.
  public export
  comparableBound : {0 bs : Bindings} -> Amount bs -> Bool
  comparableBound a = writtenBound a || readAmount a

  public export
  data ComparableBound : Amount bs -> Type where
    MkComparableBound : {auto 0 ok : comparableBound b = True} -> ComparableBound b

  ||| May this amount DEFINE a letter? Everything a sentence can compute,
  ||| and nothing it has already written: "where X is 4" is written zero
  ||| times because the numeral would say it, and "where X is X" is
  ||| written zero times because it would say nothing (`badWrittenXDef`).
  ||| The complement of `writtenBound` at a third question, which is what
  ||| makes the rider's own reason legible — a rider is worth writing
  ||| exactly when the value has to be worked out.
  ||| Every attested right side passes: the counts and the stat reads,
  ||| and the arithmetic too ("1 plus the number of basic land types",
  ||| "the number of cards in your hand minus 4", "twice the number of
  ||| Foods you control").
  public export
  letterDefines : {0 bs : Bindings} -> Amount bs -> Bool
  letterDefines d = not (writtenBound d)

  public export
  data LetterDefinition : Amount bs -> Type where
    MkLetterDefinition : {auto 0 ok : letterDefines d = True} -> LetterDefinition d

  ||| A noun phrase that contributes NOTHING to the discourse — every read,
  ||| and the bare source; never a determined mention. Re-keyed onto
  ||| `nounDelta` rather than mirroring the constructor list, so the
  ||| question asked is the one that matters ("does this phrase bind?").
  |||
  ||| What consumes it is the condition subject (`Matches`): a condition
  ||| introduces nothing (`condDelta`), so a phrase that WOULD have
  ||| introduced something can only be written there by losing it silently.
  ||| Refusing the phrase is the honest form of that — and it is what makes
  ||| the target-announcing conditional (Blood Lust) unwritable rather than
  ||| mis-written (`badMatchesTargetSubject`; ledger).
  public export
  data Bindingless : Noun bs k -> Type where
    MkBindingless : {auto 0 ok : nounDelta n = []} -> Bindingless n

  ||| Which phrase may ANCHOR a complement — chapter twenty-six's
  ||| replacement for the blanket `Bindingless` demand `OtherThan` carried,
  ||| and the correction is that "valid anchor" and "introduces no binding"
  ||| were never the same question. The old demand refused the only two
  ||| corpus lines whose anchor is written as a target (Death by Dragons,
  ||| Terrifying Presence); those targets are announced like any other
  ||| ([CR#601.2c]), and the announcement now travels with the phrase
  ||| (`predDelta`) instead of being refused for existing.
  |||
  ||| So the SHAPE question is asked on its own: an anchor is a READ, or a
  ||| phrase the text writes as a target. What stays out is the determiner
  ||| that would announce a referent the sentence never spelled — "other
  ||| than a creature" is unwritten English and `Indefinite` still refuses
  ||| it (`badComplementAnchorAnnounces`) — and every determiner that names
  ||| a set rather than a referent (`Each`, `AllOf`, `EachOf`, the slice,
  ||| the partitive, the remainder), which subtract nothing an anchor can
  ||| subtract.
  public export
  anchorPhrase : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  anchorPhrase This = True
  anchorPhrase (AsType t n) = anchorPhrase n
  anchorPhrase You = True
  anchorPhrase (PlayerGroup _) = True
  anchorPhrase (Each _) = False
  anchorPhrase (Indefinite _ _) = False
  anchorPhrase (TargetGroup _ _) = True
  anchorPhrase (AllOf _) = False
  anchorPhrase (EachOf _) = False
  anchorPhrase (LibrarySlice _ _ _) = False
  anchorPhrase (SomeOf _ _) = False
  anchorPhrase TheRest = False
  anchorPhrase It = True
  anchorPhrase They = True
  anchorPhrase Them = True
  anchorPhrase (Those _) = True
  anchorPhrase (That _) = True
  anchorPhrase (AttachHost _ _) = True
  anchorPhrase (TheVerbed _ _) = True
  anchorPhrase (ThoseVerbed _ _) = True
  anchorPhrase (ControllerOf _) = True
  anchorPhrase (OwnerOf _) = True

  ||| The complement anchor as a witness: the right SHAPE, and singular.
  ||| The number half keeps the family honest — the constructor subtracts
  ||| ONE anchored referent, and a plural read passed there ("other than
  ||| those creatures") is group subtraction, the subset-complement family
  ||| finding 111 deferred and that no corpus line writes at all
  ||| (`badPluralComplementAnchor`).
  public export
  data ComplementAnchor : Noun bs k -> Type where
    MkComplementAnchor : {auto 0 sh : anchorPhrase n = True} ->
                         {auto 0 one : nounPlur n = OneOf} ->
                         ComplementAnchor n

  ||| Whose exiles a linkage read may name — one object, the one the
  ||| reading ability is printed on. [CR#607.1] builds the whole relation
  ||| out of "two abilities printed on IT", [CR#406.6] repeats it for this
  ||| family by name, and [CR#607.5] shows what the restriction is worth
  ||| with a worked example: a Quicksilver Elemental that has gained both
  ||| Arc-Slogger's exile and Sisters of Stone Death's pair can return only
  ||| what it exiled with the SECOND, because the phrase names that
  ||| ability's object and not the exile zone. So the source slot takes the
  ||| self-word and nothing else (`badExiledWithOtherSource`,
  ||| `badExiledWithTargetSource`).
  |||
  ||| One SPELLING is left out and it is a whole other rule rather than a
  ||| gap here: [CR#607.2n] links "cards exiled with cards named [this
  ||| object's name]" to a before-the-game static ability, whose source is
  ||| a NAME and not an object (one corpus line, Volatile Chimera). It
  ||| wants the printed name read back, which finding 188 keeps unread
  ||| (ledger). Both self-words qualify, which is the same split chapter
  ||| thirty drew between "this card" and "this creature".
  public export
  data LinkSource : Noun bs k -> Type where
    SelfLinked : LinkSource This
    SortedSelfLinked : {0 t : CardType} -> {0 asc : Ascribable This} ->
                       LinkSource (AsType t This {asc})

  ||| Which phrases a choice clause can SELECT — the introduction
  ||| discipline chapter twenty gave the indefinite article, asked of
  ||| "Choose". A choice binds a NEW referent out of a described set, so
  ||| the phrase has to describe one: the corpus writes "choose a/an …",
  ||| "choose target …", "choose two/three …", "choose up to …", "choose
  ||| any number of …", "choose another …" — every one of them a selection
  ||| from a description — and writes "choose you", "choose it" and "choose
  ||| them" zero times each. Choosing an ALREADY DEFINITE participant is
  ||| not a choice at all: there is nothing to select among, and the clause
  ||| would announce a mention it did not bind (`badChooseYou`). The
  ||| distributive and universal determiners are out for the neighbouring
  ||| reason, leaving the two introducing determiners exactly. It reads the
  ||| constructor rather than `nounDelta`, unlike `Bindingless` beside it,
  ||| because a relative clause's possessor makes a read's delta nonempty
  ||| without making the read a choice.
  public export
  choosable : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  choosable This = False
  choosable (AsType _ _) = False
  choosable You = False
  choosable (PlayerGroup _) = False
  choosable (Each _) = False
  choosable (Indefinite _ _) = True
  choosable (TargetGroup _ _) = True
  choosable (AllOf _) = False
  choosable (EachOf _) = False
  choosable (LibrarySlice _ _ _) = False
  -- a partitive IS a selection out of a set, and the corpus writes it as
  -- one -- but always with the chooser named ("an opponent chooses two of
  -- them"), and `Choose` has no agent slot to name one. Admitting it here
  -- would spell an agentless "Choose two of them" the corpus does not
  -- write (`badChooseSomeOf`); the agentful choice clause is ledgered.
  choosable (SomeOf _ _) = False
  choosable TheRest = False
  choosable It = False
  choosable They = False
  choosable Them = False
  choosable (Those _) = False
  choosable (That _) = False
  choosable (AttachHost _ _) = False
  choosable (TheVerbed _ _) = False
  choosable (ThoseVerbed _ _) = False
  choosable (ControllerOf _) = False
  choosable (OwnerOf _) = False

  public export
  data Choosable : Noun bs k -> Type where
    MkChoosable : {auto 0 ok : choosable n = True} -> Choosable n

  ||| Which phrases name a GROUP the sentence can then reach INTO — the
  ||| complement "each of" takes and the complement a division is spread
  ||| "among". Measured rather than guessed: of the words the corpus writes
  ||| after "each of", the object-denoting ones are a counted target
  ||| mention ("each of up to two target creatures") and a plural read
  ||| ("each of them", "each of those creatures"). Nothing else: no "each
  ||| of each …", no "each of all …", no "each of a …"
  ||| (`badEachOfDistributive`, `badEachOfAll`, `badEachOfIndefinite`). The
  ||| rest of the corpus's "each of" lines are the temporal and quality
  ||| phrases this vocabulary does not reach ("each of your turns").
  |||
  ||| The reason is what the determiner does: it distributes over MEMBERS,
  ||| so it needs a phrase whose members the sentence has already fixed. A
  ||| description has none until it resolves, which is what the plain
  ||| distributive `Each` is for; a definite singular has one, which is no
  ||| group at all.
  public export
  groupMention : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  groupMention (TargetGroup _ _) = True
  groupMention Them = True
  groupMention (Those _) = True
  groupMention This = False
  groupMention (AsType _ _) = False
  groupMention You = False
  -- "each of players" is not English and "each of your opponents" is
  -- seven lines against `Each Opponent`'s 914 -- a second spelling of a
  -- cell already written, not a group this vocabulary reaches into.
  groupMention (PlayerGroup _) = False
  groupMention (Each _) = False
  groupMention (Indefinite _ _) = False
  groupMention (AllOf _) = False
  groupMention (EachOf _) = False
  -- the slice is THE assembled group this chapter is about: a look or a
  -- reveal over it is what lets the next sentence reach its members.
  groupMention (LibrarySlice _ _ _) = True
  -- a part is not a group to reach into: "each of two of them" and "the
  -- rest of two of them" are zero lines each.
  groupMention (SomeOf _ _) = False
  -- nor is the complement: "each of the rest" is zero lines.
  groupMention TheRest = False
  groupMention It = False
  groupMention They = False
  groupMention (That _) = False
  groupMention (AttachHost _ _) = False
  groupMention (TheVerbed _ _) = False
  -- The plural participle read DOES name a group, which is the whole
  -- difference from its singular twin, so the partitive determiners
  -- ("one of them", "the rest") can stand over it.
  groupMention (ThoseVerbed _ _) = True
  groupMention (ControllerOf _) = False
  groupMention (OwnerOf _) = False

  public export
  data GroupMention : Noun bs k -> Type where
    MkGroupMention : {auto 0 ok : groupMention n = True} -> GroupMention n

  ||| May a clause whose amount is PER MEMBER take this recipient? The
  ||| damage and counter clauses write ONE magnitude and one recipient
  ||| phrase, and the phrase has to say whether that magnitude is each
  ||| member's or the group's between them. A SINGULAR recipient asks
  ||| nothing; the two distributive determiners answer "each member's"
  ||| outright — `Each` over a description ("deals 1 damage to each
  ||| creature you don't control", Barrage of Boulders) and `EachOf` over a
  ||| group mention ("on each of up to two target creatures", Ajani,
  ||| Adversary of Tyrants). A BARE plural recipient answers neither, and
  ||| the corpus never writes one: "put a … counter on" reaches a counted
  ||| group only through "each of", and "deals … damage to" likewise
  ||| (`badBarePluralCounterRecipient`, `badBarePluralDamageRecipient`).
  ||| The plural READS are the same story from the other side — "counters
  ||| on them" is always a relative clause and never a recipient, "damage
  ||| to them" is the singular epicene player every time, which is `They`
  ||| and singular already, and "damage to those …" is written zero times
  ||| (`badThemCounterRecipient`). The universal is out with them: "damage
  ||| to all creatures" and "counter on all creatures" are each zero lines,
  ||| the corpus writing the sweep distributively
  ||| (`badAllOfDamageRecipient`).
  |||
  ||| The DIVISION is the other answer to the same question and is not
  ||| here: it says the magnitude is the group's, to be split
  ||| ([CR#601.2d]), and it writes its own clause (`Distribute`).
  ||| the verb.
  public export
  perMemberOk : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bool
  perMemberOk (Each _) = True
  perMemberOk (EachOf _) = True
  perMemberOk n = isOne (nounPlur n)

  ||| The per-member recipient gate's witness form (a distinctive search
  ||| name, like `DamageSource` and `Headed`).
  public export
  data PerMember : Noun bs k -> Type where
    MkPerMember : {auto 0 ok : perMemberOk n = True} -> PerMember n

  ||| WHOSE untapping the set-level cap may bound — what `CapDomain` was,
  ||| now asked of an ordinary player noun. The family is closed at seven
  ||| distinct lines on eight cards and the subject across them is the
  ||| bare plural five times, "you" once (Mungha Wurm) and "your
  ||| opponents" once (Dovin Baan's quoted emblem). Nothing else is ever
  ||| written there — no "each opponent", no "target player", no described
  ||| player set — so the three atomic phrases are the whole table
  ||| (`badUntapCapEachPlayer`).
  ||| The enum retired because its evidence did: two of its three cells
  ||| ARE the plural noun this chapter mints, and the third was `You` all
  ||| along. What is left is this gate, which says what the enum said
  ||| without a private vocabulary to say it in.
  public export
  capSubjectOk : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  capSubjectOk You = True
  capSubjectOk (PlayerGroup _) = True
  capSubjectOk This = False
  capSubjectOk (AsType _ _) = False
  capSubjectOk (Each _) = False
  capSubjectOk (Indefinite _ _) = False
  capSubjectOk (TargetGroup _ _) = False
  capSubjectOk (AllOf _) = False
  capSubjectOk (EachOf _) = False
  capSubjectOk (LibrarySlice _ _ _) = False
  capSubjectOk (SomeOf _ _) = False
  capSubjectOk TheRest = False
  capSubjectOk It = False
  capSubjectOk They = False
  capSubjectOk Them = False
  capSubjectOk (Those _) = False
  capSubjectOk (That _) = False
  capSubjectOk (AttachHost _ _) = False
  capSubjectOk (TheVerbed _ _) = False
  capSubjectOk (ThoseVerbed _ _) = False
  capSubjectOk (ControllerOf _) = False
  capSubjectOk (OwnerOf _) = False

  ||| The cap subject's demand as a witness, so a pin says which question
  ||| refused.
  public export
  data CapSubject : Noun bs k -> Type where
    MkCapSubject : {auto 0 ok : capSubjectOk n = True} -> CapSubject n

  ||| May this phrase stand as a POSSESSOR — the player a relative clause
  ||| says controls something, the player a possessive says a zone belongs
  ||| to, and the player a relative clause says cast something? One table,
  ||| three readers (`ControlledBy`, `OwnedBy` and `CastBy`), and the
  ||| reason they share it is that they ask one question of one rule each:
  ||| [CR#109.4] gives an object one controller, [CR#108.3] gives a card
  ||| one owner, and [CR#601.2a] has one player propose a casting.
  |||
  ||| The reading is MEMBERSHIP, not group quantification, and that is
  ||| what lets the plural in at all. "Creatures your opponents control"
  ||| does not quantify over the opponents; it asks whether the ONE
  ||| controller each creature has is a member of the named set, which is
  ||| exactly why [CR#109.4] makes the phrase unambiguous rather than
  ||| making it unwritable. The singular possessor is that same question
  ||| over a one-member set, so the row's meaning unifies instead of
  ||| forking. The Rust core composes it that way outright —
  ||| `ControlledBy(OpponentOf(Ref(You)))` over a player PREDICATE
  ||| (`deckmaste_core/src/filter.rs`, live in `deontic.rs`) — and the CR
  ||| writes the phrase into hexproof's own definition ([CR#702.11b]).
  |||
  ||| So the gate is the singular test WIDENED by one measured word, not a
  ||| constructor table: a counted target mention is still admitted at one
  ||| and refused at two (`badControlledByGroup`), which a flat table
  ||| could not say. The widening is exactly "your opponents" — 377
  ||| supported cards in the relative clause, 7 in the possessive
  ||| (graveyards 6, hands 1). The other plural player phrases are
  ||| measured and stay out: the bare plural is written ZERO times in
  ||| either position ("players control" as a relative clause is zero, and
  ||| "players' graveyards" zero — the corpus writes "all players' hands"
  ||| and "each player's graveyard" instead), which is
  ||| `badControlledByAllPlayers` and `badOwnedByAllPlayers`; and the
  ||| plural READ ("creatures those players control") is two lines, both
  ||| inside constructions this grammar cannot write at all — Camouflage's
  ||| blocker partition and Officious Interrogation's per-member count —
  ||| so it is refused rather than half-admitted, with its antecedent
  ||| question left where it lies.
  |||
  ||| The THIRD reader was measured before it joined, which is finding
  ||| 309's union-over-readers doctrine at mint time again: "spells your
  ||| opponents cast" is 27 lines, a singular definite read ("spells that
  ||| player casts", "spells they cast") 5, and the bare plural "players
  ||| cast" zero (`badCastByAllPlayers`) — the same three answers the
  ||| other two readers give, cell for cell, which is what keeps one table
  ||| honest.
  public export
  possessorOk : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bool
  possessorOk (PlayerGroup YourOpponents) = True
  possessorOk (PlayerGroup AllPlayers) = False
  possessorOk n = isOne (nounPlur n)

  ||| The possessor demand as a witness, so a pin says which question
  ||| refused.
  public export
  data Possessor : Noun bs k -> Type where
    MkPossessor : {auto 0 ok : possessorOk n = True} -> Possessor n

  ||| WHO a cost statement may be about. [CR#609.2]'s own worked example
  ||| settles the general case in as many words -- "an effect that says
  ||| spells cost more to cast will apply only to spells on the stack,
  ||| since a spell is always on the stack while a player is casting it"
  ||| -- so the described subject is a phrase that PLACES its referent on
  ||| the stack, which is what `Macros.spell` already is and what
  ||| `Gets`' battlefield demand is one zone over.
  |||
  ||| The corpus is unusually clean about it: of 628 supported
  ||| cost-to-cast lines, 615 write the word "spell" in the subject and 10
  ||| more read a spell mention back with a pronoun; the remaining three
  ||| are Commander-format lines about a commander. Not one line puts a
  ||| battlefield noun there (`badCostSubjectOnBattlefield`).
  |||
  ||| So this is the stack test WIDENED by one measured subject rather
  ||| than a constructor table -- `possessorOk`'s shape at a second site.
  ||| The widening is the SELF, and it is 388 of the 628 lines: "This
  ||| spell costs {1} less to cast" is the same word "spell" on the same
  ||| stack, but the self-reference is exophoric and projects no zone at
  ||| all ([CR#113.7]; chapter forty-eight), so the zone test cannot see
  ||| it and the row would refuse the family's dominant surface.
  public export
  costSubjectOk : {bs : Bindings} -> Noun bs Object -> Bool
  costSubjectOk This = True
  costSubjectOk n = onStackZone (nounZone n)

  ||| The cost statement's subject demand as a witness, so a pin says
  ||| which question refused.
  public export
  data CostSubject : Noun bs Object -> Type where
    MkCostSubject : {auto 0 ok : costSubjectOk n = True} -> CostSubject n

  ||| A truth-valued test a clause can be conditioned on — core's
  ||| `Condition` (`deckmaste_core/src/condition.rs`), and named after it.
  ||| The three rows here are core's first three exactly
  ||| (`Exists(Predicate)`, `Matches(Reference, Predicate)`,
  ||| `Compare(Count, Cmp, Count)`), which is not a coincidence: they are
  ||| the three questions English asks of the board without any vocabulary
  ||| beyond the phrase, the reference, and the amount this grammar already
  ||| has. Core's other rows all reach past that — attachment, event
  ||| history, cost tags, turn and phase — and each arrives with the axis
  ||| it names.
  |||
  ||| The word "if" here has "only its normal English meaning"
  ||| ([CR#603.4]'s own parenthetical): this is the ORDINARY conditional,
  ||| not the intervening-"if" of a triggered ability, which that rule
  ||| reserves for an "if" immediately following a trigger condition and
  ||| checks twice ([CR#603.4], [CR#608.2a]). That distinction is a CARRIER
  ||| distinction and not a condition one, and the seam is deliberate: the
  ||| trigger layer reuses this type verbatim for its intervening-"if"
  ||| clause, so nothing here may assume a one-shot reader.
  public export
  data Condition : Bindings -> Type where
    -- "you control a creature with power 4 or greater" / "there is a …":
    -- at least one object answers the description ([CR#603.4]'s
    -- ordinary-English "if"). Core's `Exists(Predicate)` in one argument,
    -- and the same demands `CountOf` makes of a domain, because it is the
    -- same kind of slot: the phrase must be noun-HEADED
    -- (`badExistsUnheaded`) and must not spell the damage class, which
    -- names a target and describes no object (`badExistsAnyTarget`). No
    -- article of its own: the existential quantifier IS the indefinite
    -- English writes, and the count comparison below spells the other form
    -- ("one or more"), which is why both are rows and neither is sugar for
    -- the other; core keeps them apart likewise.
    -- spelling: ["there is <Param(0)>", "there are <Param(0)>"] (and the
    -- FRONTED-SUBJECT spelling "you control <Param(0)>" when the phrase
    -- carries a controller clause -- the relative "creature you control"
    -- becoming the finite "you control a creature", which is the form the
    -- corpus writes 338 times against 7 for the existential-there),
    -- kind: TODO(reason: condition fragment -- not one of
    -- Nominal/Sentence/Cost/KeywordLine/Ability)
    Exists : {k : Kind} -> (p : Predicate bs k) ->
             {auto 0 hd : Headed p} ->
             {auto 0 af : AnyTargetFree p} -> Condition bs
    -- "[who] [past-verb] [w]" -- the HISTORY read, [CR#608.2i]'s
    -- look-back-in-time as a condition. Core's
    -- `Condition::Happened { event, within: Lookback }`
    -- (`deckmaste_core/src/condition.rs`, cited to the same rule and
    -- naming morbid and raid as its corpus) at the same address, with the
    -- one difference this vocabulary always makes: core matches an open
    -- `EventFilter` where this names an `EventName` from the shared
    -- catalog and describes the subject beside it. The rule is what
    -- licenses the read at all -- an effect may "require information about
    -- previous game states and actions rather than considering the current
    -- game state", and the object "need not currently meet the criteria
    -- described in the action, as long as they did so at the specified
    -- time" -- which is exactly why this is not a description: a creature
    -- that died this turn is in a graveyard now.
    -- The SUBJECT is a NOUN and not a predicate, and the corpus forced it.
    -- Three of the four written subject shapes are nouns no predicate can
    -- spell: "you" carries the raid mass and the cast, draw and life
    -- families (over a hundred lines), and the SELF carries fifty-eight
    -- more ("if this creature attacked or blocked this turn"). Only the
    -- indefinite description ("if a creature died this turn") would have
    -- fitted a predicate. So the row takes the phrase and drops its
    -- announcement, which is `Exists`' own arrangement read the other way:
    -- a condition contributes nothing either way (`condDelta`), so the
    -- choice is free at the binding layer and settled by what English
    -- writes.
    -- WHICH events and WITH WHICH SUBJECT SORT is `lookbackSubjectOk`, a
    -- two-axis table because the attack declaration takes both sorts and a
    -- one-axis one would have had to refuse a family.
    -- The WINDOW is required and carries no default -- core's rule
    -- ("history counting never gets a silent default window") and the
    -- corpus's, every line writing its adverbial.
    -- spelling: ["<Param(1)> <past verb for Param(0)> <Param(2)>"] (the
    -- verb word is the event name's own past tense and its VOICE follows
    -- the subject sort: object rows write the plain past -- "a creature
    -- died this turn", "this creature attacked this turn", "a permanent
    -- left the battlefield this turn", "this creature was dealt damage
    -- this turn" -- while player rows write the perfect with the pronoun
    -- contracted, "you've cast a noncreature spell this turn", "you've
    -- drawn two or more cards this turn", and the two life rows write the
    -- plain past again, "you gained life this turn". Param(2) is
    -- Lookback's own adverbial), kind: TODO(reason: condition fragment,
    -- see Exists)
    Happened : {k : Kind} -> (ev : EventName) -> (who : Noun bs k) ->
               (w : Lookback) ->
               {auto 0 sb : LookbackSubject ev k} -> Condition bs
    -- "it's [day/night]" -- the GAME's own designation read, and the first
    -- consumer any Idris module here has ever had for the game scope
    -- ([CR#731.1] -- "day and night are designations that the game itself
    -- can have"). It takes no subject at all, which is what a game-scoped
    -- read is: there is one game and the phrase names it with a dummy
    -- pronoun.
    -- Gated by `timeCheckOk`, whose asymmetry is the measurement: "if it's
    -- night" and its siblings are four lines and "if it's day" is written
    -- zero times (`badItIsDay`). The TRANSITION writes both directions, so
    -- this is a fact about the check.
    -- DAYBOUND and NIGHTBOUND are the boundary here: [CR#702.145a] puts
    -- them "on opposite faces of some double-faced cards", so their whole
    -- text is keyword-static boilerplate over a card frame this vocabulary
    -- does not model, and none of it is evidence for this row.
    -- spelling: ["it's <Param(0)>"] (the dummy subject and the copula --
    -- see TimeOfDay), kind: TODO(reason: condition fragment, see Exists)
    ItIsNow : (t : TimeOfDay) -> {auto 0 tc : TimeChecked t} -> Condition bs
    -- "it's an artifact creature" -- a REFERENCE answers a description:
    -- core's `Matches(Reference, Predicate)` with the kind index this
    -- grammar carries and core does not. (Core cites [CR#603.4] on this
    -- row; the rule licenses the ordinary-English "if" and says nothing
    -- about references answering descriptions, so the cite is not
    -- propagated here.) The subject is a read and never a mention
    -- (`Bindingless`): the condition introduces nothing, so a determined
    -- phrase written here would be announced and then dropped. The
    -- predicate need not be headed -- "if it's attacking" and "if it's
    -- tapped" are as real as "if it's a creature card" -- but the class
    -- word is refused as it is everywhere.
    -- spelling: ["<Param(0)> is <Param(1)>"] (auto-inflection supplies the
    -- copula and its contraction, "it's"/"they're"; the same predicate
    -- spelled postnominally in a noun phrase is spelled predicatively
    -- here), kind: TODO(reason: condition fragment, see Exists)
    Matches : {k : Kind} -> (n : Noun bs k) -> (p : Predicate bs k) ->
              {auto 0 bl : Bindingless n} ->
              {auto 0 sy : PredSays p} ->
              {auto 0 zc : ZoneFits (nounZone n) (seedZone p)} ->
              {auto 0 af : AnyTargetFree p} -> Condition bs
    -- "its mana value is 2 or less" / "you have more life than an
    -- opponent" -- a measured amount against a standard, which is core's
    -- `Compare(Count, Cmp, Count)` with the ONE comparator axis
    -- (`Comparator`) and the ONE amount vocabulary. No parallel
    -- machinery: the postnominal qualifier "with power 4 or greater" and
    -- the predicative "if its power is 4 or greater" are two FRAMES over
    -- the same relation, and what differs between them is word order, not
    -- vocabulary -- which is why the bound test (`ComparableBound`) is
    -- ONE table read by both. The remaining gate is the left side, where
    -- English does demand a reader (`ReadAmount`).
    -- spelling: ["<Param(0)> is <Param(1)> <Param(2)>" against a written
    -- bound, "<Param(0)> is <Param(2)> <Param(1)>" against a read] (which
    -- word the comparator spells is the head's register: a stat takes
    -- "or greater"/"or less" and "greater/less than", a count takes "or
    -- more"/"or fewer" and "more/fewer than" -- one relation, two
    -- registers. The dominant surfaces rewrite the whole comparison
    -- around its subject: the count subject spells existentially, "there
    -- are <Param(2)> or more <domain>" (173 lines), and the read-vs-read
    -- family spells a HAVE- or CONTROL-clause, "you have more life than
    -- an opponent", "you control fewer creatures than an opponent"),
    -- kind: TODO(reason: condition fragment, see Exists)
    CompareAmt : (subj : Amount bs) -> (r : Comparator) -> (bound : Amount bs) ->
                 {auto 0 rd : ReadAmount subj} ->
                 {auto 0 cb : ComparableBound bound} -> Condition bs
    -- "if you don't control an Army creature" / "if it isn't a Zombie" --
    -- the CONDITION negation, core's `Not(Condition)`
    -- (`deckmaste_core/src/condition.rs`) and the ledger's own entry,
    -- opened here because [CR#701.47a] writes BOTH of amass's branches
    -- with it and the corpus writes it far past the carriers the ledger
    -- had counted -- a hundred and twelve one-shot lines across the two
    -- rows it reaches -- and the sentence that fixes the shape is real
    -- card text and not the rule alone: "Put two +1/+1 counters on target
    -- artifact, creature, or land you control. Untap that permanent. If it
    -- isn't a creature, it becomes a 0/0 creature in addition to its other
    -- types."
    -- WHICH rows may be negated is a closed table and not this row's
    -- license (`CondNegatable`): the two frames above are written, the
    -- comparison frame is not -- "if its power isn't 4 or greater" is
    -- written zero times, English saying it with the opposite comparator
    -- instead -- and a negation of a negation is written zero times too.
    -- The negated condition still contributes nothing (`condDelta`).
    -- spelling: (construction-owned -- negates its inner condition's own
    -- frame, exactly as Predicate's Not does: the fronted-subject
    -- existential becomes "<subject> don't/doesn't <verb> <Param(0)>" or
    -- the determiner negation "<subject> controls no <Param(0)>", and the
    -- copular reference frame becomes "<Param(0)> isn't <Param(1)>". The
    -- transform depends on the negated condition's own shape), kind:
    -- TODO(reason: condition fragment, see Exists)
    -- The NAME is core's `Not` disambiguated from the predicate negation
    -- exactly as `CompareAmt` is disambiguated from `Compare` — one
    -- namespace, two frames, and the argument type in the name.
    NotCond : (c : Condition bs) -> {auto 0 ng : CondNegatable c} -> Condition bs

  ||| Which condition frames English negates — a closed full-row table, so
  ||| a new condition declares whether its own frame takes a "not". The
  ||| existential and the reference frames do; the comparison frame does
  ||| not, because a bound has a NEGATIVE of its own — "if its power isn't
  ||| 4 or greater" is written zero times and "3 or less" is what English
  ||| writes there — and a negation under a negation is written zero times
  ||| likewise (`badNegatedComparison`, `badDoubleNegatedCondition`). The
  ||| two open rows are open CONDITIONALLY: the frame negates a POSITIVE
  ||| phrase only, a blanket `True` having let the polarity launder
  ||| (`predNegFree`, `badNegatedNegativeMatch`).
  public export
  condNegatable : {0 bs : Bindings} -> Condition bs -> Bool
  condNegatable (Exists p) = predNegFree p
  -- the history read NEGATES, and the corpus writes the negation on the
  -- auxiliary rather than on the subject: "if you haven't cast a spell
  -- this turn" is eight lines and the didn't/hasn't/haven't family
  -- nineteen, against a single line writing the other surface ("if no
  -- creature died this turn"), which is a spelling residue for the same
  -- meaning and ledgered rather than built.
  condNegatable (Happened _ _ _) = True
  -- "it isn't night" is zero lines; the check is written positively or
  -- not at all.
  condNegatable (ItIsNow _) = False
  condNegatable (Matches n p) = predNegFree p
  condNegatable (CompareAmt subj r bound) = False
  condNegatable (NotCond c) = False

  public export
  data CondNegatable : Condition bs -> Type where
    MkCondNegatable : {auto 0 ok : condNegatable c = True} -> CondNegatable c

  ||| Is this condition NEGATED at its own frame? The question the
  ||| "unless" marking asks, and a different one from `condNegatable`
  ||| beside it: that table says which frames English will put a "not"
  ||| on, this one reads whether one is there. Full rows, so a new
  ||| condition declares its answer here too.
  public export
  condNegated : {0 bs : Bindings} -> Condition bs -> Bool
  condNegated (Exists _) = False
  condNegated (Happened _ _ _) = False
  condNegated (ItIsNow _) = False
  condNegated (Matches _ _) = False
  condNegated (CompareAmt _ _ _) = False
  condNegated (NotCond _) = True

  ||| Which marking word a conditional static may write over which
  ||| condition. "As long as" takes either polarity where "unless" IS the
  ||| negation, so it takes only a negated condition and spells the
  ||| positive underneath it. The refusal is what keeps the two spellings
  ||| one construction: "unless you control no artifact" is a double
  ||| negative the corpus writes zero times (`badUnlessOnPositive` names
  ||| the other direction).
  public export
  markingOk : {0 bs : Bindings} -> CondMarking -> Condition bs -> Bool
  markingOk AsLongAs _ = True
  markingOk Unless c = condNegated c

  ||| The marking demand as a witness, named apart so a pin says which
  ||| question refused.
  public export
  data MarkingOk : {0 bs : Bindings} -> CondMarking -> Condition bs -> Type where
    MkMarkingOk : {0 c : Condition bs} ->
                  {auto 0 ok : markingOk m c = True} -> MarkingOk m c

  ||| What a condition contributes to the discourse: NOTHING, on every row.
  ||| This is the disjunction hole (`predDelta (Or _) = []`) at clause
  ||| level and for the same reason — a condition may be false, so a
  ||| mention written inside one names nobody the sentences after it can
  ||| read back (`badConditionAntecedent`). It is a full-row table rather
  ||| than a constant so that a new condition has to declare its answer.
  |||
  ||| Corpus is not unanimous, and the exception is named rather than
  ||| smoothed over: a conditional whose if-clause announces a TARGET does
  ||| leave a referent behind, because [CR#601.2c] announces targets as the
  ||| spell is cast whatever clause spells them, so the condition's truth
  ||| never gated the announcement. That family is refused at the subject
  ||| slot (`Bindingless`) rather than admitted with a lie about its
  ||| bindings, and it waits on the ledger.
  public export
  condDelta : {bs : Bindings} -> Condition bs -> List Binding
  condDelta (Exists p) = []
  -- a history read announces nothing, `Exists`' answer and for its reason:
  -- the phrase names what HAPPENED and the object it names need not even
  -- be where it was ([CR#608.2i]), so there is nothing for a later clause
  -- to pick up.
  condDelta (Happened _ _ _) = []
  condDelta (ItIsNow _) = []
  condDelta (Matches n p) = []
  condDelta (CompareAmt subj r bound) = []
  condDelta (NotCond c) = []


  ||| Durations, as the trailing adverbial writes them ([CR#611.2a] — a
  ||| resolution-generated continuous effect "lasts as long as stated";
  ||| with no stated duration it lasts until end of game, the explicit
  ||| `Nothing` spelling per the no-defaults convention).
  |||
  ||| "FOR AS LONG AS" ([CR#611.2b]) is core's
  ||| `Duration::ForAsLongAs(Condition)` (`continuous.rs`), which is why
  ||| the type is INDEXED by the discourse: a condition is typed in
  ||| bindings, so every duration is. The condition reads what the clause
  ||| it trails ANNOUNCED, supplied by the `Continuously` envelope's own
  ||| telescope (`staticIntro se`) — what Old Man of the Sea needs.
  |||
  ||| The EVENT-ENDED row is where one phrase turned out to be TWO
  ||| constructions. [CR#610.3] makes "exile [object] until [event]" a
  ||| ONE-SHOT zone change that schedules its own undo — "a second one-shot
  ||| effect is created immediately after the specified event" — not a
  ||| continuous effect at all, and [CR#610.4] says the same of phasing.
  ||| So the row is here, its answer is `Unclaimed`, and the writable half
  ||| is a clause rider (`HeldUntil`). Core makes the opposite cut on
  ||| purpose (`continuous.rs`): right for a runtime, which has to schedule
  ||| the undo either way, wrong for a grammar, where the two phrases sit
  ||| in different slots of different clauses.
  |||
  ||| "This turn" is its own row because it is not an `Until` form: it
  ||| names the current turn as a whole, without a boundary word, and it is
  ||| what the one-shot restrictions write where the grants write "until
  ||| end of turn" (`spanUse`).
  public export
  -- spelling: ["this turn", "until <Param(0)>", "for as long as <Param(0)>",
  -- "until <Param(0)>"] (row order: ThisTurn/Until/ForAsLongAs/UntilEvent;
  -- Until's own word is the bare "until" and the endpoint supplies the rest
  -- -- see DurationEnd; the for-as-long-as row's parameter is a condition,
  -- which spells its own frame -- see Condition; UntilEvent writes the same
  -- bare "until" over a finite event clause -- see GameEvent),
  -- kind: TODO(reason: trailing-adverbial fragment --
  -- not one of Nominal/Sentence/Cost/KeywordLine/Ability)
  data Duration : Bindings -> Type where
    ThisTurn : Duration bs
    Until : DurationEnd -> Duration bs
    ForAsLongAs : Condition bs -> Duration bs
    UntilEvent : GameEvent bs -> Duration bs

  ||| An event PATTERN — a happening and the object or player it happens
  ||| to. The vocabulary two constructions share: the would/instead clause
  ||| names one to intercept ([CR#614.1a]) and the zone-change rider names
  ||| one to wait for ([CR#610.3]), and the MOOD is the reading
  ||| construction's rather than the pattern's ("would die" against
  ||| "leaves the battlefield").
  |||
  ||| The subject is a full noun phrase and not a bare reference, because
  ||| both constructions target through it: "Prevent all damage that would
  ||| be dealt to target creature this turn" announces a target inside the
  ||| event pattern ([CR#601.2c]). Which row goes with which construction
  ||| is `eventUse`'s answer and not this type's.
  ||| The optional COMBAT PARTNER on a block event — the creature blocked
  ||| or the creature blocking, said or left out. `CtrlSingular`'s shape:
  ||| a data family over the `Maybe` with one row per case, so the gate
  ||| that would ride a required noun rides the written one and asks
  ||| nothing at all of the unwritten one.
  |||
  ||| The slot is optional because the corpus writes both, in quantity and
  ||| on the same cards: "Whenever this creature blocks, …" against
  ||| "Whenever this creature blocks a creature with flying, …" (24
  ||| transitive headers), and bare "Whenever this creature becomes
  ||| blocked, …" against "Whenever this creature becomes blocked by a
  ||| creature, …" (61 with the complement). One event with a slot, not
  ||| two events — [CR#509.1] declares blockers once and the phrase either
  ||| names the other end of the relation or does not.
  |||
  ||| What the written partner must satisfy is what the SUBJECT satisfies,
  ||| and for the same reasons: only a creature in combat is at either end
  ||| ([CR#506.3,509.1a]), so the phrase may not state another zone, and a
  ||| bare self offers no type evidence to place it (`SelfSorted` — the
  ||| corpus writes "becomes blocked by this creature" and never "by
  ||| this").
  public export
  data BlockPartner : {0 bs : Bindings} -> Maybe (Noun bs Object) -> Type where
    NoPartner : BlockPartner Nothing
    OnePartner : {0 m : Noun bs Object} ->
                 {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                 {auto 0 ss : SelfSorted m} -> BlockPartner (Just m)

  public export
  -- spelling: (construction-owned -- the reading construction supplies the
  -- mood and the opening word: Intercepts writes "if <subject> would
  -- <event>" or "the next time <subject> would <event>" per ReplUse,
  -- HeldUntil and Duration.UntilEvent write the finite "<subject>
  -- <event>s", the trigger header writes the same finite form after
  -- When/Whenever/At. Never spelled alone)
  data GameEvent : Bindings -> Type where
    -- "[n] dies" ([CR#700.4] — dying IS the battlefield-to-graveyard
    -- transition, so the watched object is one the battlefield can hold;
    -- `badWouldDieInGraveyard`).
    Dies : (n : Noun bs Object) ->
           {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
           {auto 0 ss : SelfSorted n} -> GameEvent bs
    -- "[n] leaves the battlefield" ([CR#603.6c] names the transition —
    -- from the battlefield to another zone) — the O-Ring endpoint.
    Leaves : (n : Noun bs Object) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
             {auto 0 ss : SelfSorted n} -> GameEvent bs
    -- "[n] would be destroyed" ([CR#701.8a] — destruction moves a
    -- permanent from the battlefield to its owner's graveyard;
    -- [CR#701.8c] names regeneration as the effect that replaces the
    -- destruction event, which is this row's whole corpus).
    IsDestroyed : (n : Noun bs Object) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  {auto 0 ss : SelfSorted n} -> GameEvent bs
    -- "damage would be dealt to [n]" ([CR#120.1] — damage is dealt to a
    -- player or a battlefield permanent), the event prevention watches
    -- and redirection replaces. Reads the same recipient row the damage
    -- CLAUSE reads (`DamageRecipient`), which is what keeps one damage
    -- vocabulary rather than two.
    IsDealtDamage : {k : Kind} -> (to : Noun bs k) ->
                    {auto 0 rk : DamageRecipient to} -> GameEvent bs
    -- "[who] would draw a card" ([CR#121.1] — a draw moves the top card
    -- of a library to its owner's hand; [CR#614.11] makes draw
    -- replacement its own paragraph).
    Draws : (who : Noun bs Player) -> GameEvent bs
    -- "[who] loses the game" -- the OUTCOME as an event, and the one
    -- outcome that is ever an event: [CR#603.9] defines the trigger
    -- family for losing alone ("Some triggered abilities trigger
    -- specifically when a player loses the game"), and the corpus agrees
    -- exactly -- "would win the game" is written ZERO times and no
    -- trigger header names a win at all, so there is no win row here and
    -- no `OutcomeVerb` index over this one. `Concludes` states an
    -- outcome and this watches one; they are the two directions of
    -- [CR#104] and not one row twice.
    -- [CR#603.9]'s own sentence carries two things this row is glad not
    -- to spell: the event fires when a player "loses OR LEAVES the game,
    -- regardless of the reason", so the cause is never named here, and
    -- it is carved out for the draw ("unless that player leaves the game
    -- as the result of a draw"), which is the engine's arithmetic over
    -- `GameDrawn` rather than anything a phrase says. [CR#603.10f] adds
    -- that these abilities LOOK BACK IN TIME, which is why the subject
    -- may still be read after the player is gone -- and it is the
    -- semantic warrant for `eventAfter` minting the subject's mention at
    -- all (Sengir reads "that player's life total", Ramses reads "they").
    -- The Rust core has NO loss row in its `EventFilter` (`event.rs`)
    -- either, so this is a gap measured at both layers rather than a port
    -- of something already there -- and by the merge doctrine the
    -- measurement here is the core's too.
    -- spelling: ["<Param(0)> loses the game"], kind: Sentence (the
    -- subject phrase and the finite verb; the trigger word is the
    -- header's -- see TriggerWord -- and the "would" of the replacement
    -- is Intercepts' own)
    LosesGame : (who : Noun bs Player) -> GameEvent bs
    -- "[n] enters" ([CR#603.6a] -- "enters-the-battlefield abilities
    -- trigger when a permanent enters the battlefield"). The corpus has
    -- RETEMPLATED this event out from under its old wording: exactly ONE
    -- supported line still writes "enters the battlefield" as the
    -- trigger's own verb phrase, against thousands of bare "enters"
    -- headers, so the row spells the short form and the long one is
    -- history rather than a variant.
    -- The SUBJECT is type-ascribed when it is the source, and the
    -- ascription is what places it: [CR#109.2] reads a description
    -- including a card type onto the battlefield, which is the evidence
    -- this event's own zone demand asks for, and bare `This` offers none
    -- (`SelfSorted`, `badEntersBareThis`).
    Enters : (n : Noun bs Object) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
             {auto 0 ss : SelfSorted n} -> GameEvent bs
    -- "[n] attacks" ([CR#508.1] declares attackers; the event is the
    -- declaration). It does NOT carry `Cant`'s deed gate, and the reason
    -- is the same rule read the other way. [CR#506.3] says "only a
    -- creature can attack or block" of the object at the moment it
    -- attacks; the trigger's subject is a DESCRIPTION that need not be a
    -- creature when the sentence is read, and the corpus writes exactly
    -- that -- "When a Vehicle you control attacks, …", a Vehicle being an
    -- artifact ([CR#301.7]) that a crew ability animates first.
    Attacks : (n : Noun bs Object) ->
              {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
              {auto 0 ss : SelfSorted n} -> GameEvent bs
    -- "[n] blocks" / "[n] blocks [what]" ([CR#509.1] declares blockers,
    -- [CR#509.1g] makes the chosen creature "a blocking creature" and
    -- says in the same breath what it is blocking). The attack row's
    -- shape with ONE addition, an optional patient, because the corpus
    -- writes the relation's other end when it wants to act on it:
    -- "Whenever this creature blocks a creature with flying, this
    -- creature gets +2/+0 until end of turn" is 24 headers against the
    -- bare form's mass, and the patient is what the body then reads.
    -- The patient THREADS the subject's context — `Fights`' second slot
    -- and `DealsCombatDamage`'s recipient, typed at `nomIntro n` — so
    -- "that creature" in the body resolves to whichever phrase came last,
    -- and its gates ride `BlockPartner` rather than the constructor,
    -- there being nothing to demand of an unwritten phrase.
    -- The row does NOT gate on the subject being a legal blocker: that a
    -- creature must be untapped to be declared ([CR#509.1a]) is the
    -- declaring engine's check on a turn-based action, not a fact about
    -- the sentence, exactly as `Tap` declines to demand untappedness.
    -- spelling: ["<Param(0)> block(s)"] / ["<Param(0)> block(s)
    -- <Param(1)>"] (the finite clause the header word introduces; the
    -- patient is written bare, with no preposition, where the passive row
    -- below writes "by")
    Blocks : (n : Noun bs Object) ->
             (what : Maybe (Noun (nomIntro n) Object)) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
             {auto 0 ss : SelfSorted n} ->
             {auto 0 bp : BlockPartner what} -> GameEvent bs
    -- "[n] becomes blocked" / "[n] becomes blocked by [by]" — the SAME
    -- turn-based action watched from the other end, and [CR#509.1h] is
    -- the whole warrant for it being its own row: "an attacking creature
    -- with one or more creatures declared as blockers for it becomes a
    -- blocked creature". The blocker becomes blocking ([CR#509.1g]) and
    -- the attacker becomes blocked; one declaration, two subjects, two
    -- events, which is why this row carries its own `EventName`
    -- (`BlockedDeclaration`) and answers the four readers separately.
    -- It is NOT a becomes-status event and must not be routed through
    -- one: blocked-ness is a combat designation and no [CR#110.5]
    -- category holds it, which is why `StatusCat` has four rows and not
    -- five.
    -- The complement is the same optional slot, written "by": 61 headers
    -- name the blocker and the rest leave it out. The `Blocks`-side
    -- coordination the corpus writes constantly — "Whenever this creature
    -- blocks or becomes blocked by a creature, …" — is a coordinated
    -- EVENT, one clause naming two happenings, which this vocabulary has
    -- no word for and which is ledgered with the other coordinations.
    -- spelling: ["<Param(0)> become(s) blocked"] / ["<Param(0)>
    -- become(s) blocked by <Param(1)>"]
    BecomesBlocked : (n : Noun bs Object) ->
                     (by : Maybe (Noun (nomIntro n) Object)) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     {auto 0 ss : SelfSorted n} ->
                     {auto 0 bp : BlockPartner by} -> GameEvent bs
    -- "[n] deals combat damage to [to]" ([CR#510.2] — combat damage is
    -- what the combat damage step deals, the adjective `DamageKind`
    -- already carries). TWO noun slots because the corpus writes both
    -- and the recipient is what the family divides on: six hundred
    -- twenty-five of the six hundred eighty-eight lines write "to a
    -- player". The recipient row is the damage CLAUSE's own
    -- (`DamageRecipient`), the third construction to read it.
    DealsCombatDamage : {k : Kind} -> (n : Noun bs Object) ->
                        (to : Noun (nomIntro n) k) ->
                        {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                        {auto 0 ss : SelfSorted n} ->
                        {auto 0 rk : DamageRecipient to} -> GameEvent bs
    -- "the beginning of [whose] [part]" ([CR#603.2b] — "when a phase or
    -- step begins, all abilities that trigger 'at the beginning of' that
    -- phase or step trigger"). The ONLY event with no noun phrase in it
    -- at all, which is why it is also the only one whose header word is
    -- fixed (`triggerWordOk`, `At`). Its part-and-possession grid is
    -- chapter seventeen's endpoint vocabulary, shared as that chapter
    -- said it would be, and which cells are written is `partUse`'s
    -- answer (`badTriggerAtUntapStep`, `badTriggerAtEachUpkeep`).
    BeginningOf : (part : TurnPart) -> (whose : Maybe Owner) ->
                  {auto 0 pu : PartTriggerable part whose} -> GameEvent bs
    -- "[who] cast(s) [what]" ([CR#601.2] -- casting is what a player does
    -- to move a card to the stack and pay its costs; [CR#701.5a] words the
    -- keyword action the same way). Chapter twenty-eight's largest
    -- unminted family, and the one that could not be written before
    -- because its complement is a SPELL: [CR#112.1] makes a spell a card
    -- ON THE STACK, so the complement's zone clause is the whole of what
    -- the word means and the demand is that it name that zone. TWO noun
    -- slots, the caster and the spell, which is `DealsCombatDamage`'s
    -- shape and for its reason -- the corpus writes both and divides on
    -- both. The complement is the ordinary noun phrase and needs no
    -- vocabulary of its own, "a creature spell" being the type word under
    -- the stack clause exactly as "a creature card in your graveyard" is
    -- the type word under a graveyard one.
    -- spelling: ["<Param(0)> cast(s) <Param(1)>"] (the finite clause the
    -- header word introduces; the complement's own zone clause is
    -- UNSPELLED -- "a spell", "a creature spell" -- the stack being the
    -- zone whose carrier noun is the word itself [CR#112.1], the same
    -- elision "card" makes for the four hidden zones)
    Casts : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
            {auto 0 zn : OnStack (nounZone what)} ->
            {auto 0 one : nounPlur what = OneOf} ->
            {auto 0 nt : Nontarget what} -> GameEvent bs
    -- "[n] becomes [tapped/untapped]" ([CR#603.2e] — a "becomes" trigger
    -- event fires only at the time the named event happens, neither when
    -- the state already exists nor again while it persists, so this row
    -- names a TRANSITION where `HasStatus` describes the standing state
    -- and `Untap` requests the change: three readers, three
    -- constructions. The subject is an ordinary object noun — the corpus
    -- varies it across the bare type, the controller-qualified phrase,
    -- the sorted self, and the permanent head while the event relation
    -- stays one — and it stands on the battlefield, only permanents
    -- having status at all ([CR#110.5d]): a phrase stating another zone
    -- contradicts, silence passes (`zoneFits`' discipline). WHICH values
    -- the event spells is `statusEventOk`'s measured answer — the tap
    -- pair and nothing else — a different table from `statusWordOk`,
    -- whose face cells are real DESCRIPTIONS and unwritten EVENTS.
    -- spelling: ["<Param(0)> becomes <Param(1)>"] (the finite clause
    -- the header word introduces; Param(1) is the status value's own
    -- word — see StatusVal)
    BecomesStatus : {c : StatusCat} -> (n : Noun bs Object) ->
                    (v : StatusVal c) ->
                    {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                    {auto 0 ss : SelfSorted n} ->
                    {auto 0 at : StatusEventVal v} -> GameEvent bs
    -- "[n] is turned face up" — the face transition as its OWN event and
    -- not a becomes-status value, which is what `statusEventOk`'s two
    -- refused face cells were holding the place for. [CR#708.7] is the
    -- warrant: the ability that allowed a permanent to be face down "may
    -- also allow the permanent's controller to turn it face up", so what
    -- the header watches is a TURNING, and the corpus writes it with the
    -- passive participle every time. `IsDestroyed`'s naming for
    -- `IsDestroyed`'s reason — the row spells a passive and the "Is"
    -- prefix says so — which also keeps the event distinct from the
    -- imperative `ToFace` that performs the same transition.
    -- VALUE-INDEXED over the face pair, `BecomesStatus`' idiom at the
    -- narrower domain ([CR#110.5] pairs the values and the index picks
    -- the category), with the per-value attestation table beside it:
    -- `faceEventOk` opens the up cell at 113 lines and refuses the down
    -- cell at zero (`badTurnedFaceDownEvent`). The refusal is a
    -- MEASUREMENT written as a cell rather than as a missing constructor,
    -- which is why the row is indexed at all — the face-down turning is a
    -- real game event ([CR#708.2a]) that no header names.
    -- The subject-side gates are `BecomesStatus`' own, unchanged: a
    -- permanent is what turns face up ([CR#708.7] — spells normally
    -- can't be turned face up), demanded as `zoneFits`' silence-passing
    -- battlefield gate, and the self subject is sorted (the "When this
    -- creature is turned face up" morph mass writes the type word without
    -- exception).
    -- spelling: ["<Param(0)> is turned <Param(1)>"] (the finite clause the
    -- header word introduces; Param(1) is the status value's own word —
    -- see StatusVal — so the one open cell reads "is turned face up")
    IsTurnedFace : (n : Noun bs Object) -> (v : StatusVal FaceC) ->
                   {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                   {auto 0 ss : SelfSorted n} ->
                   {auto 0 at : FaceEventVal v} -> GameEvent bs
    -- "[n] phases out" / "[n] phases in" — phasing observed, [CR#702.26]'s
    -- keyword read as an event. Value-indexed like the row above and with
    -- NO attestation table, because both cells are written: seven trigger
    -- headers each direction (Teferi's Imp writes both on one card).
    -- [CR#702.26b] and [CR#702.26c] are the same sentence twice in
    -- opposite directions — "its status changes to 'phased out'" against
    -- "'phased in'" — so one relation over [CR#110.5]'s pair is what the
    -- corpus writes, not two verbs that happen to rhyme.
    -- This is NOT a becomes-status event and must not be read as one:
    -- "becomes phased out" is written zero times and stays refused
    -- (`statusEventOk`, `badBecomesPhasedOut`); what is written is the
    -- verb, and this row spells the verb.
    -- The battlefield gate is exactly right in BOTH directions, which is
    -- the one place phasing could have surprised it: the phasing event
    -- "doesn't actually cause a permanent to change zones" ([CR#702.26d]),
    -- so a phased-out permanent is still a battlefield object and the
    -- phase-in subject needs no zone of its own.
    -- spelling: ["<Param(0)> phases <Param(1)>"] (the finite clause the
    -- header word introduces. Param(1) is the one place a status value is
    -- NOT spelled with its own word: the row writes the PARTICLE, "out"
    -- for PhasedOut and "in" for PhasedIn, where "phased out" is the
    -- description's word for the state the verb leaves behind)
    PhaseTransition : (n : Noun bs Object) -> (v : StatusVal PhaseC) ->
                      {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                      {auto 0 ss : SelfSorted n} -> GameEvent bs
    -- "day becomes night or night becomes day" -- the GAME's own
    -- transition, and the one event row in this vocabulary whose spelling
    -- is a DISJUNCTION written whole. [CR#731.1a] gives both phrases ("the
    -- phrases 'day becomes night' and 'night becomes day' refer to the
    -- game losing the first designation and gaining the second"), and ten
    -- of the family's eleven headers write BOTH of them joined by "or",
    -- always in that order and never separately.
    -- So the row takes no direction argument and spells the coordination
    -- as one lexicalised phrase, which is what keeps it clear of chapter
    -- thirty-eight's coordinated-EVENT gap: that gap is about composing
    -- two events a card names separately, and this card text never
    -- separates them. The single solo line is recorded rather than
    -- generalised into a direction slot no other line would use.
    -- It carries NO noun -- there is one game and the phrase names it
    -- without a subject phrase -- which makes it the second event row
    -- after `BeginningOf` with no participant at all, and it answers
    -- `BeginningOf`'s answers everywhere for that reason.
    -- spelling: ["day becomes night or night becomes day"] (the whole
    -- fixed phrase; the header word precedes it)
    DayNightShift : GameEvent bs
    -- "the last [kind] counter is removed from [n]" -- the counter
    -- family's own event, and the third of the three abilities suspend
    -- represents ([CR#702.62a] writes it out; [CR#702.63a] writes the
    -- same header into vanishing). It watches a REMOVAL that empties the
    -- holding, which is why it is an event and not a condition: the
    -- trigger fires at the moment the count reaches zero, once.
    -- FADING IS NOT THIS FAMILY and the distinction is worth stating
    -- because the two keywords look alike: [CR#702.32a] says "remove a
    -- fade counter from this permanent. If you CAN'T, sacrifice the
    -- permanent" -- a failure-to-remove check inside the upkeep trigger,
    -- firing when the holding is ALREADY empty, where this row fires on
    -- the removal that empties it. One counts down to zero and acts; the
    -- other tries at zero and fails.
    -- The subject is an OBJECT and never a player, which is structural
    -- rather than measured-and-refused: there is no player-holder cell to
    -- pin because the noun slot is object-sorted, and the corpus agrees
    -- at zero lines. Its kind is scope-gated to match ([CR#122.1]).
    -- The zone demand is the counter family's own two-zone table and not
    -- the battlefield: every explicit line watches a card IN EXILE
    -- (suspend's own second ability, the two Knight/unblockable lines),
    -- and vanishing's watches a permanent -- both cells `CounterHolder`
    -- already opens.
    -- WHICH kinds can be written here is finding 85's test as always:
    -- `Time` is a row and carries three of the five explicit lines; ore
    -- and refine carry one apiece and are NOT rows, so those two lines
    -- are evidence without vocabulary (finding 200's posture).
    -- spelling: ["the last <Param(0)> counter is removed from <Param(1)>"]
    -- (the finite clause the header word introduces; Param(0) =
    -- CounterKind's own word)
    LastCounterRemoved : (kind : CounterKind) -> (n : Noun bs Object) ->
                         {auto 0 sc : counterScope kind = Object} ->
                         {auto 0 zn : CounterHolder (nounZone n)} ->
                         {auto 0 ss : SelfSorted n} -> GameEvent bs

  ||| Which row an event pattern is, for the event tables.
  public export
  eventName : {0 bs : Bindings} -> GameEvent bs -> EventName
  eventName (Dies _) = Death
  eventName (Leaves _) = Departure
  eventName (IsDestroyed _) = Destruction
  eventName (IsDealtDamage _) = DamageTaken
  eventName (Draws _) = CardDrawn
  eventName (LosesGame _) = GameLoss
  eventName (Enters _) = Entry
  eventName (Attacks _) = AttackDeclaration
  eventName (Blocks _ _) = BlockDeclaration
  eventName (BecomesBlocked _ _) = BlockedDeclaration
  eventName (DealsCombatDamage _ _) = CombatDamage
  eventName (BeginningOf _ _) = PartBeginning
  eventName (Casts _ _) = SpellCast
  eventName (BecomesStatus _ _) = StatusChange
  eventName (IsTurnedFace _ _) = TurnedFaceUp
  eventName (PhaseTransition _ _) = PhasingChange
  eventName DayNightShift = TimeShift
  eventName (LastCounterRemoved _ _) = LastCounterRemoval

  ||| What an event pattern contributes to the discourse: its subject
  ||| phrase, exactly as a clause contributes its own. The event has not
  ||| happened — it is what the construction is watching FOR — but the
  ||| phrase naming its subject is written and announced all the same
  ||| ([CR#601.2c]), which is why the replacement clause can say "it".
  public export
  eventIntro : {bs : Bindings} -> GameEvent bs -> Bindings
  eventIntro (Dies n) = nomIntro n
  eventIntro (Leaves n) = nomIntro n
  eventIntro (IsDestroyed n) = nomIntro n
  eventIntro (IsDealtDamage to) = nomIntro to
  eventIntro (Draws who) = nomIntro who
  eventIntro (LosesGame who) = nomIntro who
  eventIntro (Enters n) = nomIntro n
  eventIntro (Attacks n) = nomIntro n
  -- the block events announce the LAST phrase they write, which is the
  -- patient when one is written and the subject when none is: the
  -- patient's own context already contains the subject's announcement
  -- (`DealsCombatDamage`'s row, for its reason).
  eventIntro (Blocks n Nothing) = nomIntro n
  eventIntro (Blocks _ (Just what)) = nomIntro what
  eventIntro (BecomesBlocked n Nothing) = nomIntro n
  eventIntro (BecomesBlocked _ (Just by)) = nomIntro by
  eventIntro (DealsCombatDamage n to) = nomIntro to
  eventIntro (BeginningOf _ _) = bs
  eventIntro (Casts _ what) = nomIntro what
  eventIntro (BecomesStatus n _) = nomIntro n
  eventIntro (IsTurnedFace n _) = nomIntro n
  eventIntro (PhaseTransition n _) = nomIntro n
  eventIntro DayNightShift = bs
  eventIntro (LastCounterRemoved _ n) = nomIntro n

  ||| The event SUBJECT's contribution to the trigger's body — and the one
  ||| place a bare self-reference comes to have a mention at all.
  |||
  ||| `This` never announces. That is core's line and this vocabulary keeps
  ||| it: an exophoric reference "names the game situation, is never bound
  ||| by an operator", and lives on the frame OUTSIDE the anaphora record
  ||| `It` reads (`deckmaste_engine/src/stack.rs`). So `nounDelta This` is
  ||| the empty list and stays so, and every sorted self built over it
  ||| announces nothing either.
  |||
  ||| What core ALSO has is binder-scoped minting — inside a binder, `It`
  ||| is the innermost bound element — and the workbench's analogue is the
  ||| CARRIER POSITION. The grammar already does this once, at the move:
  ||| `moveIntro (AsType t n)` mints a fresh binding for a moved sorted
  ||| self because [CR#400.7] makes the moved object a new one, which is
  ||| what lets Flickering Spirit read "it" (finding 16). This function is
  ||| the same move at the EVENT-SUBJECT position, and [CR#603.6]'s reading
  ||| discipline is why the position deserves it: a trigger's body reads
  ||| the object the event happened to, so the header's subject is a thing
  ||| the body can refer to — which is what 270 "this creature …, it" lines
  ||| write and what nothing in this grammar could say until now.
  ||| The mention is `SelfD`, so `It` sees it and the demonstratives do not
  ||| (`wordNow`).
  |||
  ||| It is applied ONLY where the event announces nothing else. Where a
  ||| patient or a recipient is written, that phrase is the announcement
  ||| and this one is not added — not to keep `It` unambiguous but because
  ||| minting there would be WRONG: the two corpus lines that write a
  ||| patient and then a bare "it" resolve the pronoun to the SUBJECT
  ||| ("Whenever this creature attacks a battle, it gets +1/+1"), which is
  ||| a subject-preference rule and not this grammar's uniqueness pronoun.
  ||| Refusing them as ambiguous would have been the wrong refusal; leaving
  ||| them unwritable is the honest one (finding 349).
  public export
  selfSubjIntro : {bs : Bindings} -> Noun bs Object -> Bindings
  -- the sorted self: the type word places it ([CR#109.2], `nounZone`'s own
  -- reading), and the mention carries no provenance and no origin because
  -- no verb took it and no clause minted it.
  selfSubjIntro (AsType t This) =
    MkBinding SelfD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing) :: bs
  -- the ATTACHMENT HOST mints too, and with an ORDINARY determinant where
  -- the self mints `SelfD`. The two exophora differ and the corpus says
  -- so: no line demonstrates back to a trigger's own subject (finding
  -- 348, zero lines), and five lines demonstrate back to an attachment
  -- host ("When enchanted creature dies, THAT CREATURE's controller ..."),
  -- which is English's rule read correctly -- "that creature" cannot mean
  -- the speaker, and the enchanted creature is not the speaker but a
  -- third party the sentence named. So this mention is visible to the
  -- demonstratives as well as to "it", and `TheD` is what says that.
  -- written per CONCRETE head word rather than over `h`, because the
  -- row's kind is `kindOfW h` and only a concrete head fixes it at
  -- `Object` -- the two attested object heads are the type word and the
  -- permanent word, and the rest are `attachHeadOk`'s False cells and
  -- unbuildable anyway.
  selfSubjIntro (AttachHost _ (TypeW t)) =
    MkBinding TheD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing) :: bs
  selfSubjIntro (AttachHost _ PermanentW) =
    MkBinding TheD Object OneOf (ObjectP Nothing (Just Battlefield) Nothing Nothing) :: bs
  selfSubjIntro n = nomIntro n

  ||| What a CONDITION contributes to the statement it conditions —
  ||| `selfSubjIntro`'s move at the second container, and deliberately the
  ||| same three lines for the subject it was minted for.
  |||
  |||     selfSubjIntro (AsType t This) = MkBinding SelfD … :: bs
  |||     selfSubjIntro n               = nomIntro n
  |||
  |||     condIntro (Matches (AsType t This) _) = MkBinding SelfD … :: bs
  |||     condIntro _                           = bs
  |||
  ||| The difference in the fallback is the whole difference between the
  ||| two positions: an event's subject is a NOUN, so a description
  ||| announces itself through `nomIntro` and only the self needed
  ||| minting; a condition announces NOTHING on any row (`condDelta`), so
  ||| there is nothing to fall back to and the unminted case contributes
  ||| the incoming context unchanged.
  |||
  ||| The scoping is round twelve's exactly: the CONTAINER threads and the
  ||| `Condition` stays opaque. `condDelta` is untouched and still answers
  ||| the empty list on every row, so a mention written inside a condition
  ||| still reaches nothing after the sentence — which is
  ||| `badConditionAntecedent`'s refusal at `Effect.If`, a DIFFERENT
  ||| container that this function does not touch and whose threading runs
  ||| the other way (its condition is typed at `preIntro e`, reading what
  ||| the effect announced). Finding 76's cross-carrier contract is what
  ||| makes both extensions cheap: `Condition` is reused verbatim by every
  ||| carrier and each carrier decides its own threading.
  ||| The minted mention is `SelfD`, so `It` sees it and the demonstratives
  ||| do not (`wordNow`, finding 348).
  |||
  ||| The COMPARISON row is chapter fifty-eight's, and it is a second
  ||| contribution rather than a second subject: a comparison that holds
  ||| holds BY an amount, and the statement reads that amount as "the
  ||| difference" (`TheDifference`, twenty-six supported lines). It is the
  ||| same per-carrier contract at a third use — the container threads and
  ||| the condition stays opaque — and it is why this function is named for
  ||| what a condition CONTRIBUTES rather than for the subject it was first
  ||| built to mint.
  public export
  condIntro : {bs : Bindings} -> Condition bs -> Bindings
  condIntro (Matches (AsType t This) _) =
    MkBinding SelfD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing) :: bs
  -- the attachment host at the condition's subject, the same two concrete
  -- heads and the same ordinary determinant.
  condIntro (Matches (AttachHost _ (TypeW t)) _) =
    MkBinding TheD Object OneOf (ObjectP (Just t) (Just Battlefield) Nothing Nothing) :: bs
  condIntro (Matches (AttachHost _ PermanentW) _) =
    MkBinding TheD Object OneOf (ObjectP Nothing (Just Battlefield) Nothing Nothing) :: bs
  -- the comparison's MARGIN. Minted for every comparison and not only for
  -- the ones a card reads back, exactly as an event's subject is minted
  -- whether or not the body says "it": what is written is the condition,
  -- and whether the statement reads the margin is the statement's
  -- business. `Matches` and the rest contribute nothing, so the catch-all
  -- stays the incoming context.
  condIntro (CompareAmt _ _ _) = gapB :: bs
  condIntro _ = bs

  ||| …and the same contribution through a trigger's INTERVENING slot,
  ||| which is the carrier the gap-reading corpus actually writes: five of
  ||| the six writable "equal to the difference" lines are a trigger with
  ||| an intervening "if" ([CR#603.4]'s double check), not an effect-level
  ||| conditional. `Nothing` is the incoming discourse unchanged, so the
  ||| twenty-two headers that write no condition are typed exactly as
  ||| before.
  public export
  interveningIntro : {bs : Bindings} -> Maybe (Condition bs) -> Bindings
  interveningIntro Nothing = bs
  interveningIntro (Just c) = condIntro c

  ||| The discourse AFTER the event has happened — the second projection
  ||| the trigger container forced, and the sharpest single difference
  ||| between this vocabulary's readers. The interception's replacement
  ||| reads `eventIntro`, because [CR#614.6] says the replaced event never
  ||| happens and the phrase's announcement is all there is; the TRIGGER's
  ||| effect reads this, because [CR#603.6] says the ability "look(s) for
  ||| the object in the zone that it moved to". So "Whenever a creature
  ||| dies, return it to the battlefield" finds a card in a graveyard where
  ||| "If a creature would die, exile it instead" finds a permanent on the
  ||| battlefield.
  |||
  ||| Three ZONE-CHANGE events retag ([CR#603.6] calls them zone-change
  ||| triggers): dying moves the object to its owner's graveyard
  ||| ([CR#700.4]), entering places it on the battlefield ([CR#603.6a]),
  ||| and the departure retags to NO zone at all. That third one is the
  ||| silence written as a retag rather than as an omission: [CR#603.6c]
  ||| says a leaves-the-battlefield ability "checks for it only in the
  ||| first zone that it went to" and the sentence never says which zone
  ||| that is. Keeping the battlefield tag would have been a claim the
  ||| event contradicts, and it let a battlefield-demanding verb read a
  ||| departed object (`badLeavesThenTap`); the BINDING survives, so "it"
  ||| still resolves and the zone-blind verbs still take it.
  public export
  eventAfter : {bs : Bindings} -> GameEvent bs -> Bindings
  eventAfter (Dies n) = moveIntro Nothing n (Just Graveyard)
  eventAfter (Leaves n) = moveIntro Nothing n Nothing
  eventAfter (IsDestroyed n) = moveIntro Nothing n (Just Graveyard)
  eventAfter (IsDealtDamage to) = nomIntro to
  eventAfter (Draws who) = nomIntro who
  eventAfter (LosesGame who) = nomIntro who
  eventAfter (Enters n) = moveIntro Nothing n (Just Battlefield)
  eventAfter (Attacks n) = selfSubjIntro n
  -- a block declaration moves nothing anywhere ([CR#509.1g,509.1h] change
  -- designations, not zones), so both rows keep their announcement:
  -- `IsDealtDamage`'s row again, not the retagging three's.
  eventAfter (Blocks n Nothing) = selfSubjIntro n
  eventAfter (Blocks _ (Just what)) = nomIntro what
  eventAfter (BecomesBlocked n Nothing) = selfSubjIntro n
  eventAfter (BecomesBlocked _ (Just by)) = nomIntro by
  eventAfter (DealsCombatDamage n to) = nomIntro to
  -- The cast event does NOT retag, and it is the one row whose
  -- non-retagging is the rule's own doing rather than a silence:
  -- [CR#601.2a] moves the card to the stack as the FIRST step of
  -- casting, so the phrase was already announced with the zone the
  -- event leaves it in. "Whenever you cast a creature spell, that spell
  -- gains … " reads a spell on the stack, which is where the phrase put
  -- it.
  eventAfter (Casts _ what) = nomIntro what
  eventAfter (BeginningOf _ _) = bs
  -- the status transition changes no ZONE — status is the permanent's
  -- physical state and not a characteristic ([CR#110.5,110.5a]), and
  -- both directions are a rotation in place, moving nothing anywhere
  -- ([CR#701.26a,701.26b]) — so the subject's
  -- binding survives into the trigger body untouched: `IsDealtDamage`'s
  -- row, not the retagging three's. "Destroy it" and "that permanent's
  -- controller" read the phrase where it announced itself.
  eventAfter (BecomesStatus n _) = selfSubjIntro n
  -- Neither face nor phase transition retags, and each has its own rule
  -- saying so. Turning face up changes the permanent's copiable values
  -- and nothing about where it is ([CR#708.8] — "any abilities relating
  -- to the permanent entering the battlefield don't trigger … because
  -- the permanent has already entered the battlefield"), and phasing is
  -- explicitly not a zone change ([CR#702.26d]). So both take
  -- `IsDealtDamage`'s row and the trigger body reads the subject where
  -- its phrase announced it: "untap that creature", "put a +1/+1 counter
  -- on it".
  eventAfter (IsTurnedFace n _) = selfSubjIntro n
  eventAfter (PhaseTransition n _) = selfSubjIntro n
  eventAfter DayNightShift = bs
  -- taking a counter off moves nothing: [CR#122.2] has counters cease to
  -- exist when the OBJECT changes zones, never the other way round, so
  -- the subject keeps its announcement.
  eventAfter (LastCounterRemoved _ n) = selfSubjIntro n

  ||| The grammatical number of an event's SUBJECT, for the one reader
  ||| that demands one. [CR#603.7b] gives a delayed trigger a single firing
  ||| by default — "it will trigger only once, the next time its trigger
  ||| event occurs" — and the corpus writes the watched referent singular
  ||| without exception (Graceful Reprieve, Stangg). The turn-part
  ||| beginning has no subject at all and answers `OneOf`, there being one
  ||| such beginning per turn. The other three readers ask nothing:
  ||| "Whenever one or more creatures die" is ordinary trigger English, so
  ||| the demand belongs to the delayed clause and not to the vocabulary
  ||| (`badDiesGroup`).
  public export
  eventSubjectPlur : {bs : Bindings} -> GameEvent bs -> Plurality
  eventSubjectPlur (Dies n) = nounPlur n
  eventSubjectPlur (Leaves n) = nounPlur n
  eventSubjectPlur (IsDestroyed n) = nounPlur n
  eventSubjectPlur (IsDealtDamage to) = nounPlur to
  eventSubjectPlur (Draws who) = nounPlur who
  eventSubjectPlur (LosesGame who) = nounPlur who
  eventSubjectPlur (Enters n) = nounPlur n
  eventSubjectPlur (Attacks n) = nounPlur n
  eventSubjectPlur (Blocks n _) = nounPlur n
  eventSubjectPlur (BecomesBlocked n _) = nounPlur n
  eventSubjectPlur (DealsCombatDamage n _) = nounPlur n
  eventSubjectPlur (BeginningOf _ _) = OneOf
  eventSubjectPlur (Casts _ what) = nounPlur what
  eventSubjectPlur (BecomesStatus n _) = nounPlur n
  eventSubjectPlur (IsTurnedFace n _) = nounPlur n
  eventSubjectPlur (PhaseTransition n _) = nounPlur n
  eventSubjectPlur DayNightShift = OneOf
  eventSubjectPlur (LastCounterRemoved _ n) = nounPlur n

  ||| The context a delayed body reads: the event's own after-discourse
  ||| with the outer clause's targets SETTLED — [CR#603.7c] refers to
  ||| particular objects determiner-blind, and the delayed ability
  ||| announces its own targets in its own event ([CR#603.3d,601.2c]).
  public export
  delayedCtx : {bs : Bindings} -> GameEvent bs -> Bindings
  delayedCtx ev = settleTargets (eventAfter ev)

  ||| The would/instead clause's gate on its event, keyed onto the event
  ||| table rather than mirroring the constructor list.
  public export
  data Interceptable : GameEvent bs -> Type where
    MkInterceptable : {auto 0 ok : admitsIntercept (eventUse (eventName ev)) = True} ->
                      Interceptable ev

  ||| The [CR#610.3] rider's gate on its event, the same question asked
  ||| by the other reader.
  public export
  data Holdable : GameEvent bs -> Type where
    MkHoldable : {auto 0 ok : admitsHold (eventUse (eventName ev)) = True} ->
                 Holdable ev

  ||| The trigger header's gate, the same question asked by the third
  ||| reader ([CR#603.1]).
  public export
  data Triggerable : GameEvent bs -> Type where
    MkTriggerable : {auto 0 ok : admitsTrigger (eventUse (eventName ev)) = True} ->
                    Triggerable ev

  ||| The delayed clause's gate, the same question asked by the fourth
  ||| ([CR#603.7]).
  public export
  data Awaitable : GameEvent bs -> Type where
    MkAwaitable : {auto 0 ok : admitsDelay (eventUse (eventName ev)) = True} ->
                  Awaitable ev

  ||| Whether an interception may be written with THIS multiplicity
  ||| word, over the event it intercepts ([CR#614.3]'s two endings).
  public export
  data ReplUseOk : GameEvent bs -> ReplUse -> Type where
    MkReplUseOk : {auto 0 ok : replUseOk (eventName ev) u = True} -> ReplUseOk ev u

  ||| Whether a header may open with THIS word, over the event it names
  ||| ([CR#603.1]'s three-way choice, [CR#603.2b]'s one fixed assignment).
  public export
  data TriggerWordOk : GameEvent bs -> TriggerWord -> Type where
    MkTriggerWordOk : {auto 0 ok : triggerWordOk (eventName ev) w = True} ->
                      TriggerWordOk ev w

  ||| Whether a turn-part beginning is one the corpus names.
  public export
  data PartTriggerable : TurnPart -> Maybe Owner -> Type where
    MkPartTriggerable : {auto 0 ok : admitsPartTrigger (partUse p w) = True} ->
                        PartTriggerable p w

  ||| The attestation table: every duration this vocabulary can spell,
  ||| against the constructions that write it. FULL ROWS over (boundary x
  ||| part x possession) plus the bare current-turn adverbial — thirty-one
  ||| cells — so a new `TurnPart`, a new `Whose`, or a new boundary is a
  ||| totality error that must be answered with evidence before anything
  ||| can be written with it.
  |||
  ||| The evidence behind the open cells is joint patterns, the adverbial
  ||| having to belong to THAT clause rather than merely share a line with
  ||| it: "until end of turn" writes stat changes and keyword grants in
  ||| quantity and NOT ONE single-deed restriction; "this turn" inverts it
  ||| exactly, restrictions and neither grant; "until your next turn" is
  ||| written by all three, the one span that is nobody's alone; "until end
  ||| of combat" takes the two grants only (Glyph of Destruction, one
  ||| banding line); "until your next upkeep" takes the keyword grant alone
  ||| and no stat change at all.
  |||
  ||| The `Unclaimed` cells are where the frontier is: they are play
  ||| permissions, control grants, base-P/T settings and a copy effect,
  ||| every one waiting on a construction rather than on a duration.
  public export
  spanUse : {0 bs : Bindings} -> Duration bs -> SpanUse
  spanUse ThisTurn = RestrictionsShieldsPermissionsAndDelays
  -- "until [poss] next [part]" — the start boundary writes no boundary
  -- word, and takes possession or the definite article, never nothing.
  spanUse (Until (StartOf Turn Nothing)) = Unattested
  spanUse (Until (StartOf Turn (Just Yours))) = GrantsRestrictionsAndReplacement
  spanUse (Until (StartOf Turn (Just ThatPlayers))) = Unclaimed
  spanUse (Until (StartOf Upkeep Nothing)) = Unattested
  spanUse (Until (StartOf Upkeep (Just Yours))) = KeywordGrantOnly
  spanUse (Until (StartOf Upkeep (Just ThatPlayers))) = Unattested
  -- The end-step endpoints, and chapter twenty-eight's first opening.
  -- The YOURS cell is the impulse family's own word and nobody else's:
  -- every line writing "until your next end step" permits playing
  -- just-exiled cards (Yasmin Khan, Dragonhawk). Its two siblings stay
  -- `Unclaimed` on a measurement rather than an assumption: "until the
  -- next end step" is one becomes-a-copy line, and "until that player's
  -- next end step" is one play permission carrying [CR#118.9]'s
  -- alternative cost, a rider the permission row does not spell
  -- (`badGainsUntilYourNextEndStep`).
  spanUse (Until (StartOf EndStep Nothing)) = Unclaimed
  spanUse (Until (StartOf EndStep (Just Yours))) = PermissionOnly
  spanUse (Until (StartOf EndStep (Just ThatPlayers))) = Unclaimed
  spanUse (Until (StartOf Combat Nothing)) = Unattested
  spanUse (Until (StartOf Combat (Just Yours))) = Unattested
  spanUse (Until (StartOf Combat (Just ThatPlayers))) = Unattested
  spanUse (Until (StartOf UntapStep Nothing)) = Unattested
  spanUse (Until (StartOf UntapStep (Just Yours))) = Unattested
  spanUse (Until (StartOf UntapStep (Just ThatPlayers))) = Unattested
  -- The sixth part writes NO duration endpoint in any possession, and
  -- the reason is that "until end of combat" already has a cell:
  -- [CR#511.2] splits the two readings in one sentence — abilities that
  -- trigger "at end of combat" trigger as the STEP begins, while effects
  -- that last "until end of combat" expire at the end of the combat
  -- PHASE — so the duration names `Combat` and only the header names
  -- this row. "Until the end of combat step" is two corpus lines and
  -- both are reminder text.
  spanUse (Until (StartOf EndOfCombat Nothing)) = Unattested
  spanUse (Until (StartOf EndOfCombat (Just Yours))) = Unattested
  spanUse (Until (StartOf EndOfCombat (Just ThatPlayers))) = Unattested
  -- "until (the) end of [part]" — the end boundary is the one that
  -- writes bare, and the bare forms are where the grants live.
  spanUse (Until (EndOf Turn Nothing)) = GrantsTypesControlReplacementAndPermission
  spanUse (Until (EndOf Turn (Just Yours))) = ControlGrantAndPermission
  spanUse (Until (EndOf Turn (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf Upkeep Nothing)) = Unattested
  spanUse (Until (EndOf Upkeep (Just Yours))) = Unclaimed
  spanUse (Until (EndOf Upkeep (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf EndStep Nothing)) = Unattested
  spanUse (Until (EndOf EndStep (Just Yours))) = Unattested
  spanUse (Until (EndOf EndStep (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf Combat Nothing)) = GrantsAndControl
  -- The possessed end-of-combat endpoint, and chapter twenty-eight's
  -- second opening. One line writes it -- Brazen Cannonade's "Until end of
  -- combat on your next turn, you may play that card" -- which is the
  -- weight chapter seventeen gave Glyph of Destruction at the unpossessed
  -- cell. The possessive extraposes onto the TURN and not the combat,
  -- which is `DurationEnd`'s own spelling note and not a second structure.
  spanUse (Until (EndOf Combat (Just Yours))) = PermissionOnly
  spanUse (Until (EndOf Combat (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf UntapStep Nothing)) = Unattested
  spanUse (Until (EndOf UntapStep (Just Yours))) = Unattested
  spanUse (Until (EndOf UntapStep (Just ThatPlayers))) = Unattested
  -- The end of the end-of-combat step is a boundary English never names
  -- at all: the phrase that would spell it, "until end of combat", is
  -- [CR#511.2]'s PHASE endpoint and lives four rows up.
  spanUse (Until (EndOf EndOfCombat Nothing)) = Unattested
  spanUse (Until (EndOf EndOfCombat (Just Yours))) = Unattested
  spanUse (Until (EndOf EndOfCombat (Just ThatPlayers))) = Unattested
  -- The three parts chapter forty-five added for the trigger header write
  -- NO duration endpoint at all -- "until your first main phase", "until
  -- end of your draw step" and every sibling are zero lines -- which is
  -- the same silence the untap step has held since chapter seventeen and
  -- the reason the header's possessor vocabulary is not this table's
  -- (see Owner).
  spanUse (Until (StartOf FirstMain Nothing)) = Unattested
  spanUse (Until (StartOf FirstMain (Just Yours))) = Unattested
  spanUse (Until (StartOf FirstMain (Just ThatPlayers))) = Unattested
  spanUse (Until (StartOf PostcombatMain Nothing)) = Unattested
  spanUse (Until (StartOf PostcombatMain (Just Yours))) = Unattested
  spanUse (Until (StartOf PostcombatMain (Just ThatPlayers))) = Unattested
  spanUse (Until (StartOf DrawStep Nothing)) = Unattested
  spanUse (Until (StartOf DrawStep (Just Yours))) = Unattested
  spanUse (Until (StartOf DrawStep (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf FirstMain Nothing)) = Unattested
  spanUse (Until (EndOf FirstMain (Just Yours))) = Unattested
  spanUse (Until (EndOf FirstMain (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf PostcombatMain Nothing)) = Unattested
  spanUse (Until (EndOf PostcombatMain (Just Yours))) = Unattested
  spanUse (Until (EndOf PostcombatMain (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf DrawStep Nothing)) = Unattested
  spanUse (Until (EndOf DrawStep (Just Yours))) = Unattested
  spanUse (Until (EndOf DrawStep (Just ThatPlayers))) = Unattested
  -- "for as long as [condition]" -- the one adverbial with no turn
  -- boundary in it at all ([CR#611.2b]), and the widest class in the
  -- table: every construction here writes it except the type ADDITION.
  spanUse (ForAsLongAs _) = GrantsRestrictionsControlAndPermission
  -- "until [event]" — the endpoint named by a happening rather than by
  -- a turn boundary, and every one of its rows is a silence. The answer
  -- is delegated to the event axis so that a new event has to declare
  -- its duration class along with everything else (`eventSpan`).
  spanUse (UntilEvent ev) = eventSpan (eventName ev)

  ||| The `Continuously` clause's duration slot as a witness, reading both
  ||| tables: the stated absence against `absentOk`, a written adverbial
  ||| against the construction's own row in `spanUse`.
  public export
  data SpanOk : StaticKind -> Maybe (Duration bs) -> Type where
    SpanUnstated : {auto 0 ok : absentOk k = True} -> SpanOk k Nothing
    SpanStated : {auto 0 ok : admitsSpan k (spanUse d) = True} -> SpanOk k (Just d)

  ||| The delayed clause's own duration gate ([CR#603.7b]). Unstated is
  ||| always fine — that is the rule's once-only default, not an
  ||| omission — so the unstated row carries no side condition, which is
  ||| where it parts from `SpanOk`'s.
  public export
  data DelaySpanOk : Maybe (Duration bs) -> Type where
    DelayOnce : DelaySpanOk Nothing
    DelayFor : {auto 0 ok : admitsDelaySpan (spanUse d) = True} -> DelaySpanOk (Just d)

  ||| Where a play permission takes its card FROM: the zone phrase it
  ||| writes, or, when it writes none, the zone the complement's own
  ||| binding already carries. A written phrase must name a zone one can
  ||| play from ([CR#604.6]) AND must agree with whatever the complement
  ||| says, which is `zoneFits`' ordinary silence-is-no-evidence reading —
  ||| "this card" projects nothing, so the phrase is free to place it,
  ||| while an exiled binding under a "from your graveyard" phrase would be
  ||| a quarrel (`badPlayFromWrongZone`).
  public export
  playSourceOk : {0 bs : Bindings} -> Maybe Zone -> Maybe (ZoneExpr bs) -> Bool
  playSourceOk zn Nothing = playableFrom zn
  playSourceOk zn (Just z) = playableFrom (Just (zoneSort z)) &&
                             zoneFits zn (Just (zoneSort z))

  ||| The source demand as a witness, named apart so a pin says which of
  ||| the permission's three questions refused.
  public export
  data PlaySource : {0 bs : Bindings} -> Maybe Zone -> Maybe (ZoneExpr bs) -> Type where
    MkPlaySource : {0 fz : Maybe (ZoneExpr bs)} ->
                   {auto 0 ok : playSourceOk zn fz = True} -> PlaySource zn fz

  ||| A token's defined characteristics ([CR#111.3] — "A token doesn't have
  ||| any characteristics not defined by the spell or ability that created
  ||| it"), in the fixed adjective order oracle writes them: power and
  ||| toughness, colors, the type line, the noun "token", the with-clause,
  ||| then the name. Fire Navy Trebuchet spells the whole order in one
  ||| phrase — "a 2/1 colorless Construct artifact creature token with
  ||| flying named Ballistic Boulder" — which is what fixes the
  ||| with-before-named half nothing shorter could.
  |||
  ||| Core's `Token` (`deckmaste_core/src/token.rs`) field for field, minus
  ||| what no corpus line here needs: supertypes (ledger) and the abilities
  ||| beyond bare keywords — the with-clause slot holds `Keyword` itself
  ||| rather than the `Ability` container, which is what every corpus token
  ||| line writes ("with flying"). The QUOTED form is real English and a
  ||| quotation the container has no row for; narrowing the field is what
  ||| keeps the container from admitting it silently (ledger). Color rides a
  ||| LIST, exactly as core's `color_indicator` does, and the empty list is
  ||| where English writes "colorless" ([CR#105.2c]). The name is a `Maybe`:
  ||| [CR#111.4] synthesizes an unnamed token's name from its subtypes, so
  ||| the slot is written only when the creating effect states one.
  |||
  ||| The P/T slot holds two AMOUNTS, which is what indexes the record: the
  ||| corpus writes the X/X token on 62 supported cards (52 of its 63 lines
  ||| under a definition rider — "create an X/X green Elemental creature
  ||| token, where X is the number of lands you control"), and a pair of `Nat`s
  ||| could not receive them. Core's `Token` carries two independent
  ||| `Option<StatValue>` fields for the same reason, each of which may be
  ||| a number or a computed value (`deckmaste_core/src/token.rs`); the
  ||| pair-in-one-`Maybe` here is the ENGLISH shape of that, since the
  ||| corpus writes power and toughness as one "N/N" phrase and never one
  ||| without the other. The index is the whole record's rather than the
  ||| field's because a record parameter is what a field can mention: the
  ||| bundle is written at the create clause's own context, and everything
  ||| else in it stays bs-free.
  public export
  record TokenChars (bs : Bindings) where
    constructor MkToken
    pt : Maybe (Amount bs, Amount bs)
    colors : List Color
    line : TypeLine
    abilities : List Keyword
    name : Maybe String

  ||| A token is a PERMANENT ([CR#111.1] — "a marker used to represent any
  ||| permanent that isn't represented by a card"), so its line names at
  ||| least one card type. The type-less spelling is the PREDEFINED name
  ||| ("create a Treasure token", [CR#111.10]), which core gives its own
  ||| `TokenSpec::Named` row and which waits here with that catalog (ledger).
  public export
  tokenTyped : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenTyped t = lineNonEmpty (MkTypeLine [] t.line.tys)

  ||| A CREATURE token writes power and toughness ([CR#208.1]), which its
  ||| creating effect has to define because no card prints them for it
  ||| ([CR#111.3]). The converse is NOT demanded: a Vehicle token carries a
  ||| P/T without the creature type ([CR#301.7]), so a noncreature token's
  ||| slot is left free rather than forced empty — a one-directional table,
  ||| in the direction the corpus fixes (`badCreatureTokenNoPt`).
  ||| Written as two tests joined rather than as a case over the pair: a
  ||| `case` inside this mutual block lifts to a function the positivity
  ||| checker cannot see through, and the token's gates index the record.
  public export
  ptWritten : {0 bs : Bindings} -> Maybe (Amount bs, Amount bs) -> Bool
  ptWritten Nothing = False
  ptWritten (Just _) = True

  public export
  tokenPtOk : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenPtOk t = not (lineHasType Creature t.line.tys) || ptWritten t.pt

  ||| The three token demands as witnesses, named apart so a pin says which
  ||| one refused (the `Headed`/`FightParticipant` discipline).
  public export
  data TokenTyped : TokenChars bs -> Type where
    MkTokenTyped : {auto 0 ok : tokenTyped t = True} -> TokenTyped t

  public export
  data TokenPt : TokenChars bs -> Type where
    MkTokenPt : {auto 0 ok : tokenPtOk t = True} -> TokenPt t

  public export
  data SubtypesFit : TokenChars bs -> Type where
    MkSubtypesFit : {auto 0 ok : subsFitLine t.line.subs t.line.tys = True} ->
                    SubtypesFit t

  ||| The token bundle's surface form as one witness: its colors written
  ||| once each, its type line in the order oracle writes it.
  public export
  tokenCanonical : {0 bs : Bindings} -> TokenChars bs -> Bool
  tokenCanonical t = colorsDistinct t.colors && typesOrdered t.line.tys

  public export
  data TokenCanonical : TokenChars bs -> Type where
    MkTokenCanonical : {auto 0 ok : tokenCanonical t = True} -> TokenCanonical t

  public export
  tokenHeadTy : {0 bs : Bindings} -> TokenChars bs -> Maybe CardType
  tokenHeadTy t = lastType t.line.tys

  ||| One slot of a stat modification: which WAY it runs, and by how much
  ||| ([CR#613.4c] layer 7c). The sign is the constructor and the
  ||| magnitude is an ordinary amount, which is core's `NumericOp::Up(Count)`
  ||| / `Down(Count)` field for field (`deckmaste_core/src/continuous.rs`,
  ||| whose own comment says the deltas "stay non-negative") and is
  ||| `Counter.Delta`'s shape one vocabulary over — that row put the sign
  ||| in the constructor for the counter names ("+1/+1", "-1/-1") and this
  ||| one does it for the pump. The reason is [CR#107.1b]: an `Amount` is
  ||| floored at zero, `Minus` says so in as many words, and a signed
  ||| magnitude would need a second number system. Nothing needs one — a
  ||| "-X/-X" whose X exceeds the toughness simply kills, and the
  ||| carve-out [CR#107.1b] states is for SETTING power and toughness,
  ||| which is layer 7b's business and not this row's.
  |||
  ||| The two slots are INDEPENDENT, which the corpus fixes: "+2/-2"
  ||| (Canyon Crab, 101 supported lines) and "-1/+1" (Alpha Kavu, 12) are
  ||| both written, so the pair is two signs and not one.
  public export
  data PtShift : Bindings -> Type where
    -- spelling: ["+<Param(0)>"]
    PtUp : (amt : Amount bs) -> PtShift bs
    -- spelling: ["-<Param(0)>"]
    PtDown : (amt : Amount bs) -> PtShift bs

  public export
  shiftAmount : {0 bs : Bindings} -> PtShift bs -> Amount bs
  shiftAmount (PtUp a) = a
  shiftAmount (PtDown a) = a

  public export
  shiftRises : {0 bs : Bindings} -> PtShift bs -> Bool
  shiftRises (PtUp _) = True
  shiftRises (PtDown _) = False

  ||| A magnitude the sentence WRITES as zero — the untouched slot of a
  ||| one-sided pump ("+2/+0", 1,120 supported lines; "+0/+3", 146).
  public export
  writtenZero : {0 bs : Bindings} -> Amount bs -> Bool
  writtenZero (Lit Z) = True
  writtenZero _ = False

  public export
  sameDirection : {0 bs : Bindings} -> PtShift bs -> PtShift bs -> Bool
  sameDirection p t = if shiftRises p then shiftRises t else not (shiftRises t)

  ||| The untouched slot takes its PARTNER's sign, and that is measured
  ||| rather than assumed. A written zero is signless in meaning — zero
  ||| added and zero subtracted are the same modification — so which sign
  ||| English writes on it is a spelling fact, and the corpus fixes it
  ||| absolutely: "-2/-0" (129 supported lines), "-0/-X" (10), "+2/+0"
  ||| (1,120), "+0/+3" (146), and the four disagreeing cells ("+0/-N",
  ||| "-0/+N", "+N/-0", "-N/+0") ZERO times between them
  ||| (`badDisagreeingZeroPump`). So the gate is a CANONICALITY gate,
  ||| `TokenCanonical`'s job at a second site: it leaves exactly one term
  ||| per pump instead of two that spell the same thing.
  ||| The both-zero pair is left where it falls. "+0/+0" is written once
  ||| and inside a reminder text (Starlight Spectacular's parenthetical),
  ||| "-0/-0" never, and both agree with themselves — a rule separating
  ||| them would be a rule about a cell that says nothing.
  public export
  pumpSignsOk : {0 bs : Bindings} -> PtShift bs -> PtShift bs -> Bool
  pumpSignsOk p t =
    if writtenZero (shiftAmount p) || writtenZero (shiftAmount t)
       then sameDirection p t
       else True

  public export
  data PumpSigns : PtShift bs -> PtShift bs -> Type where
    MkPumpSigns : {auto 0 ok : pumpSignsOk p t = True} -> PumpSigns p t

  ||| WHICH WAY a cost statement moves a cost, and by how much --
  ||| `PtShift`'s shape one vocabulary over, and for `PtShift`'s reason:
  ||| the direction is a word English writes ("more"/"less") and the
  ||| magnitude is an ordinary amount, so the sign lives in the
  ||| constructor and no signed number system is minted. Core makes the
  ||| same cut with the same two polarities
  ||| (`CostChange::Increase`/`Reduce`, `deckmaste_core/src/continuous.rs`).
  |||
  ||| The magnitude is an `Amount` and not a mana COST, which is the one
  ||| place this parts from the symbol run English prints, and the corpus
  ||| decides it three ways. The generic component is a NUMBER the
  ||| sentence writes between braces -- {1} 377 lines, {2} 145, {3} 41 --
  ||| and the two big shaped families are amounts this vocabulary already
  ||| has: the for-each reduction is `Times n (CountOf p)` unchanged
  ||| ("costs {1} less to cast for each artifact you control", the affinity
  ||| shape [CR#702.41a], 227 domains), and the X reduction is the
  ||| definition rider's letter read in the slot ("costs {X} less to cast,
  ||| where X is the total power of creatures you control", 38 lines).
  ||| Holding a symbol run instead would spell the letter as the printed
  ||| {X} GLYPH with nothing binding it, which would make the rider
  ||| decorative -- and chapter fifty-nine's whole finding is that the
  ||| rider is a binder and not sugar.
  ||| What that leaves out is the COLORED component, and it is small and
  ||| measured: about twenty-five lines write a symbol run with a colored
  ||| or multi-symbol payload ("costs {2}{R} more to cast", Strive's
  ||| "costs {R} more to cast for each target beyond the first"). They are
  ||| recorded rather than half-admitted (ledger). [CR#118.7a] is a rule
  ||| about what a generic reduction AFFECTS at payment time and not about
  ||| what a sentence may write, so it gates nothing here.
  public export
  data CostShift : Bindings -> Type where
    -- spelling: ["{<Param(0)>} less"]
    CostLess : (amt : Amount bs) -> CostShift bs
    -- spelling: ["{<Param(0)>} more"]
    CostMore : (amt : Amount bs) -> CostShift bs

  public export
  costAmount : {0 bs : Bindings} -> CostShift bs -> Amount bs
  costAmount (CostLess a) = a
  costAmount (CostMore a) = a

  ||| The continuous effects a resolving clause can establish
  ||| ([CR#611.2]) — the PART of the sentence that survives its
  ||| resolution, with the duration adverbial factored out onto the
  ||| envelope that carries it (`Continuously`). Core makes the same cut:
  ||| `Continuously { effect: StaticEffect, duration: Duration }`
  ||| (`deckmaste_core/src/effect.rs`, `continuous.rs`) puts every lasting
  ||| change behind one duration-bearing node, whether the change is a
  ||| characteristic modification or a prohibition, and English agrees with
  ||| it: the adverbial is one slot, written once at the end of the clause,
  ||| and the constructions below differ only in what precedes it.
  |||
  ||| A slice of core's static rows deliberately — the ones this vocabulary
  ||| has clauses for. The rest, becoming a copy and losing abilities and
  ||| setting base power and toughness, arrive with the ability layer, each
  ||| a new `StaticKind` row the span tables will force to declare its
  ||| adverbials.
  public export
  data StaticEffect : Bindings -> Type where
    -- "[n] gets [+p/+t]" — the stat-modifying continuous effect
    -- ([CR#613.4c] layer 7c, core's `Modification::PowerAndToughness…`).
    -- The one-shot form modifies a battlefield object
    -- (`badGetsGraveyard`); graveyard-reaching changes are static
    -- abilities, a later chapter.
    -- Each slot is a SIGNED MAGNITUDE and the magnitude is the ordinary
    -- amount vocabulary (`PtShift`), which is what the pair of integers
    -- this row used to carry could not say. Three families arrive with
    -- it and none needs a row of its own: the for-each pump ("gets +1/+1
    -- for each Equipment you control", 353 supported cards) is
    -- `nForEach`'s term unchanged, since that macro is already
    -- `Times n (CountOf p)`; the letter pump ("gets +X/+X, where X is …",
    -- 300 supported cards, 191 of the rider lines in an effect's own
    -- position) is the definition rider's amount read in a slot; and the
    -- READ pump ("gets +X/+Y, where X is the exiled creature card's
    -- power") is a stat read.
    -- The amounts are typed in the SUBJECT's context, which the corpus
    -- forces: 46 supported cards write a domain that reads the subject
    -- back ("for each Aura attached to it", "for each of its colors",
    -- "for each +1/+1 counter on it").
    -- What the slot carries out is the subject and nothing else
    -- (`staticIntro`), a computed magnitude announcing no more here than
    -- it does at `EntersWithCounters`.
    -- The VALUE is fixed once, on resolution ([CR#611.2d] — a resolving
    -- spell or ability's variable "is determined only once"), which is
    -- what makes the pump a one-shot clause with a duration rather than a
    -- standing statement; the static-ability line that writes the same
    -- sentence is NOT this row, its value being recomputed at any given
    -- moment ([CR#611.3a] — an effect from a static ability "isn't
    -- 'locked in'"), and that family waits on a rider of its own
    -- (ledger).
    -- spelling: ["<Param(0)> gets <Param(1)>/<Param(2)>"] (each slot
    -- writes its own sign and magnitude, e.g. "+1/+1", "-X/-0"; the
    -- trailing duration adverbial belongs to the Continuously envelope,
    -- not here), kind: Sentence
    Gets : (n : Noun bs Object) -> (pow : PtShift (nomIntro n)) ->
           (tou : PtShift (nomIntro n)) ->
           {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
           {auto 0 ps : PumpSigns pow tou} -> StaticEffect bs
    -- "[spell class] cost[s] [more/less] to cast" -- the cost-modification
    -- statement, and the statement CLASS the static vocabulary has been
    -- missing: chapter sixty-one went looking for a row to put twenty-five
    -- rider lines on and found there was none at all. Six hundred and
    -- fifty-three supported lines write it.
    -- Both of English's subjects go through one row, which is core's own
    -- cut (`StaticEffect::CostModifier { of, change }`, whose `of` is a
    -- predicate that the spell's own rows match through `Ref(This)` --
    -- `deckmaste_core/src/continuous.rs`, `resolve`d at
    -- `engine/src/cast.rs`): the SELF is 388 lines ("This spell costs {1}
    -- less to cast") and the described class is the rest, and the two
    -- differ in which noun stands there and in nothing else
    -- (`CostSubject`).
    -- WHERE it applies is the rules' own answer and not a modelling
    -- choice. [CR#601.2f] runs one pipeline -- the total cost is the mana
    -- cost plus additional costs and increases, minus reductions, in any
    -- order the player likes, floored at {0} and then locked in -- and
    -- [CR#613.11] sends every cost-affecting continuous effect through
    -- that order rather than through timestamps. Paying the changed cost
    -- counts as paying the original ([CR#118.7]).
    -- What this row is NOT: an ADDITIONAL cost ([CR#118.8]) is a pipeline
    -- step of its own, and it stays out on a measurement. The corpus
    -- writes it 347 times as a spell's own text ("As an additional cost to
    -- cast this spell, …") and SIX times as a statement about a class
    -- ("Spells cost an additional 'Sacrifice a Swamp' to cast for each
    -- black mana symbol in their mana costs", Drought) -- but not one of
    -- those six has a mana payload: they name a quoted action, a loyalty
    -- symbol or a life payment, and four of the six are about activated
    -- abilities. So the shape this row could hold is exactly the shape the
    -- statement never writes (ledger). An ALTERNATIVE cost ([CR#118.9])
    -- swaps the base rather than moving it, which is a different row
    -- again.
    -- The amount is a WRITTEN count ([CR#118.5]'s "{0}" is a payment of
    -- nothing, and no line writes a reduction of it -- `badZeroCostShift`).
    -- spelling: ["<Param(0)> cost(s) <Param(1)> to cast"] (the verb agrees
    -- with the subject phrase's own number, singular for the self and
    -- plural for a class; Param(1) writes its magnitude between braces and
    -- its direction word after -- see CostShift), kind: Sentence
    CostsToCast : (n : Noun bs Object) -> (sh : CostShift bs) ->
                  {auto 0 cs : CostSubject n} ->
                  {auto 0 wc : WrittenCount (costAmount sh)} -> StaticEffect bs
    -- "[statement], where X is [amount]" -- the definition rider at the
    -- STATEMENT frame, `WhereLetter`'s twin one container over. The letter
    -- vocabulary is one binder read by two frames, exactly as `Compare`
    -- and `CompareAmt` are one comparison read by two: what differs is
    -- what the rider binds OVER, an instruction there and a statement
    -- here, and nothing else. A hundred forty-two supported rider lines
    -- sit in a static ability's position.
    -- The RULE is a different one, which is why this is a row and not a
    -- widening. [CR#107.3c] fixes a defined X "while that spell or ability
    -- is on the stack", and a static ability never goes on the stack: its
    -- letter is read at any moment the statement applies ([CR#611.3a] --
    -- a continuous effect from a static ability "isn't 'locked in'; it
    -- applies at any given moment to whatever its text indicates"), which
    -- is what Death's Shadow needs to track a life total that changes.
    -- The resolving pump's value is fixed once instead ([CR#611.2d]), so
    -- the two frames are two regimes and the vocabulary they share is the
    -- letter, not the timing. What licenses the letter at all here is
    -- [CR#107.3]'s own opening sentence -- "Some objects have abilities
    -- that define the value of X" -- and the statement is a static ability
    -- by [CR#604.1] ("written as statements, and they're simply true"),
    -- generating its continuous effect by [CR#604.2].
    -- The wrapper carries no `StaticKind` of its own and passes its
    -- body's through, which is the one place it differs from
    -- `Conditionally`: an "as long as" clause is itself a statement class
    -- with an adverbial profile of its own, while a rider is a DEFINITION
    -- and the sentence it rides is still a pump or an entry line. Every
    -- table keyed on the kind should answer about the statement English
    -- writes, so `staticKind`, `staticLineOk` and `staticIntro` all
    -- recurse (the letter stays visible outward, as it does through
    -- `effIntro` at the other frame -- [CR#107.3i] within the ability).
    -- spelling: ["<Param(2)>, where <Param(0)> is <Param(1)>"] (Param(0) =
    -- the letter's own word; a rider nested immediately inside another
    -- joins its clause with "and", as at WhereLetter), kind: Sentence
    WhereLetterStatic : (w : LetterWord) -> (def : Amount bs) ->
                        {auto 0 xd : LetterDefinition def} ->
                        (se : StaticEffect (Experimental.letterB w :: bs)) ->
                        StaticEffect bs
    -- "[n] gains [ability]" — the keyword grant, same battlefield
    -- discipline. The ability slot is the whole container now, and the
    -- gate is what keeps the widening honest: English grants an ACTIVATED
    -- ability by quoting it, which is not this clause's spelling
    -- (`Grantable`, `badGainsActivated`).
    -- spelling: ["<Param(0)> gains <Param(1)>"] (the trailing duration
    -- adverbial belongs to the Continuously envelope, not here),
    -- kind: Sentence
    Gains : (n : Noun bs Object) -> (ab : Ability) ->
            {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
            {auto 0 gr : Grantable ab} -> StaticEffect bs
    -- "[n] can't [deed, in a voice]" -- the DEONTIC: a continuous effect
    -- denying its subject a deed, which is core's `Deontic(Cant(…))` under
    -- the same envelope (`deckmaste_core/src/deontic.rs`) with the two
    -- halves English writes. Verb and voice are separate arguments for
    -- core's own reason: core tells the readings apart by which SLOT
    -- carries the reference (`Block { by, on }`), so fusing them into one
    -- word would spell "block"'s two voices as unrelated vocabulary. The
    -- deed is the restriction the declare steps check ([CR#508.1c] for
    -- attacking, [CR#509.1b] for blocking and for being blocked), and it
    -- beats any permission it meets ([CR#101.2]). Two demands, each a
    -- shape this grammar makes elsewhere: the subject stands on the
    -- battlefield (combat is fought there -- [CR#506.4] takes a permanent
    -- that leaves out of combat; `badCantInGraveyard`) and carries the
    -- deed's grant IN THAT VOICE ([CR#506.3]; `badCantAttackLand`,
    -- `badCantDisjunctSubject`, `badCantBeAttacked`). The third demand --
    -- that the span be the RESTRICTION's adverbial and not the grant's,
    -- and that it be written at all -- is the envelope's
    -- (`SpanOk DeedRestriction`; `badCantUntilEndOfTurn`, `badStaticCant`).
    -- "[n] can't [deed]" / "[n] [deed]s [p] if able" / "[n] can't [deed]
    -- [p] unless [payer] pays [cost]" -- the DEONTIC, one row over the
    -- Deed x Role grid with the polarity on its own axis. Chapters
    -- twenty-two and forty-six wrote the first two as separate rows and
    -- finding 331 said when they should merge; this is that trigger
    -- fired. See CompTag for the argument and the three prior arts.
    -- The subject may be plural ("Other creatures can't attack this
    -- turn.", Intimidation Bolt), so no grammatical number is demanded.
    -- THE PAYER IS DERIVED and not a slot, finding 254's rule at a fourth
    -- site: every attack and block gate names the gated subject's own
    -- controller ("its controller", "their controller", or "you" when the
    -- subject is the self), so the possessive is a function of the
    -- subject and the spelling owns it. The ONE cell where it differs is
    -- Block/Patient, where the DEFENDING player pays ("can't be blocked
    -- unless defending player pays"), and that difference is a fact about
    -- which participant the deed's patient role names -- also carried by
    -- the spelling, not by a slot.
    -- WHAT THE GATE DOES NOT SPELL is the scaled cost. Eighteen of the
    -- twenty-six cost-gate lines write "for each …" ("pays {2} for each
    -- creature they control that's attacking you"), which is a scaled
    -- cost term the `Cost` vocabulary has no row for -- and core cannot
    -- spell it either, its gate carrying a flat `Arc<[CostComponent]>`
    -- (`deckmaste_core/src/deontic.rs`), so this is a gap in both
    -- vocabularies and not a narrowing of one. Ledgered.
    -- spelling: ["<Param(0)> can't <Param(2)+Param(3)> <Param(4)>"
    -- (Forbid), "<Param(0)> <Param(2)+Param(3)> <Param(4)> each combat if
    -- able" (Require, standing; under a Continuously span the cadence
    -- drops -- see chapter forty-six), "<Param(0)> can't
    -- <Param(2)+Param(3)> <Param(4)> unless <derived payer> pays
    -- <Param(1)'s cost>" (GatedBy)], kind: Sentence (Params 2 and 3 spell
    -- ONE verb phrase, the deed word inflected by its voice; Param(4) is
    -- written only where `deonticPatientOk` admits it. The gate's UNLESS
    -- is construction-owned and is the third in this grammar: not
    -- `CondMarking.Unless`, which marks a negated CONDITION on a
    -- conditional static, and not [CR#118.12a]'s may-else, which offers a
    -- choice -- this one prices a deed)
    Deontic : (n : Noun bs Object) -> (c : Compulsion bs) ->
              (deed : Deed) -> (role : Role) ->
              (patient : Maybe (Noun (nomIntro n) Object)) ->
              {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
              {auto 0 dp : DeedParticipant deed role (nounTy n)} ->
              {auto 0 pt : DeonticPatient (compulsionTag c) deed role patient} -> StaticEffect bs
    -- "you may choose not to untap [n] during your untap step" -- the
    -- PERMISSION TO DECLINE, seven lines and one shape. [CR#502.3] is what
    -- it overrides: the active player "determines which permanents they
    -- control will untap", normally all of them, and this line makes one
    -- permanent's untapping optional. Its subject is the self or a named
    -- permanent throughout.
    -- It files under `DeedRestriction` with `DoesntUntap` -- the untap
    -- step's other static, and the class it needs behaviourally: a
    -- durationless clause is refused (`absentOk`) and the static ABILITY
    -- line is admitted (`staticAsAbility`), which is exactly the profile
    -- of seven lines that are all printed card text and none of them a
    -- clause with a span.
    -- spelling: ["you may choose not to untap <Param(0)> during your untap
    -- step"], kind: Sentence (the possessive is derived and not a slot,
    -- finding 254's rule at a third site: the permission is the untapping
    -- player's own)
    MayDeclineUntap : (n : Noun bs Object) ->
                      {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                      StaticEffect bs
    -- "[who] can't lose the game" / "[who] can't win the game" -- the
    -- OUTCOME GATE, and a channel of its own on a settled ruling both
    -- prior arts already record. It is NOT a `Deontic`: `Compulsion`
    -- governs DEEDS a player performs ([CR#508.1c] and its family), and
    -- losing the game is not a deed but an outcome the rules impose. It is
    -- NOT event suppression either: the gate is PRECEDENCE, not
    -- consumption -- [CR#101.2] gives a "can't" priority over an effect
    -- that "allows or directs something to happen", so the outcome is
    -- suppressed at each check while the gate lasts rather than replaced
    -- once.
    -- Three things pierce it and none is this row's business: concession
    -- ([CR#101.1]'s only exception, [CR#104.3a]), the last-player-standing
    -- win, which "overrides all effects that would preclude that player
    -- from winning" ([CR#104.2a]), and the simultaneous case, where a
    -- player who would both win and lose loses ([CR#104.3f]).
    -- ONE CLAUSE, ONE GATE. Platinum Angel's sentence is two of them
    -- coordinated -- "You can't lose the game AND your opponents can't win
    -- the game" -- which is why the row takes one kind and one subject
    -- rather than a pair.
    -- It files under `DeedRestriction`, which is a behavioural class and
    -- not a second claim that this is a deed: that class is what a printed
    -- static line with an optional this-turn span needs, and
    -- `MayDeclineUntap` is already filed there on the same reasoning
    -- (`DoesntUntap`'s "one class of statement per every kind-keyed
    -- reader").
    -- spelling: ["<Param(1)> <Param(0)>"] (the subject and the gate's own
    -- finite phrase -- see OutcomeGateKind; a this-turn span rides the
    -- Continuously envelope as every restriction's does), kind: Sentence
    OutcomeGate : (k : OutcomeGateKind) -> (who : Noun bs Player) ->
                  StaticEffect bs
    -- "[n] doesn't untap during [its controller's] untap step" -- the
    -- untap-step lock, nearly every "doesn't untap" line writing exactly
    -- this "during" frame and "does not untap" written zero times. The
    -- untap-step untap is a turn-based action in which the active player
    -- DETERMINES which permanents they control will untap and untaps them
    -- all simultaneously ([CR#502.3,703.4c]), so the lock restricts a
    -- step's transition rather than denying a deed a type grants -- which
    -- is why it is its own row and no `Deed` row: `Cant`'s grant gate
    -- ([CR#506.3]) has nothing to say about untapping, which every
    -- permanent does. It files under `DeedRestriction` all the same, the
    -- `EntersWithCounters` precedent: one class of statement per every
    -- kind-keyed reader. The step's POSSESSOR is not a slot: a permanent
    -- untaps only during its controller's untap step, so the possessive
    -- AGREES with the subject and the spelling owns it. The one-shot
    -- "during its controller's next untap step" family, the single
    -- non-"during" line, and the set-level "can't untap" caps are
    -- measured and stay other constructions (ledger).
    -- spelling: ["<Param(0)> doesn't untap during your untap step"
    -- (self subject), "<Param(0)> doesn't untap during its controller's
    -- untap step" (otherwise)], kind: Sentence
    DoesntUntap : (n : Noun bs Object) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  StaticEffect bs
    -- "[d] can't untap more than [k] [p] during [d's] untap step(s)" --
    -- the SET-LEVEL cap, the third and last construction the doesn't-untap
    -- census split off (finding 250's seven "can't untap" lines, finding
    -- 257's deferral). It is not `DoesntUntap` with a number: that row
    -- names FIXED objects and denies them the step's untap outright, where
    -- this one names no object at all and bounds a CARDINALITY.
    -- [CR#502.3] is exactly what it bounds -- "the active player
    -- DETERMINES which permanents they control will untap. Then they
    -- untap them all simultaneously" -- so the cap restricts how many
    -- permanents that determination may include, which is why its subject
    -- names a player GROUP and its object is a described SET rather than a
    -- noun phrase naming anybody.
    -- Three axes were expected here and two are real. The SUBJECT is an
    -- ordinary player noun held to the three attested phrases
    -- (`CapSubject`) and the BOUND a closed two-row table over the
    -- written number (`CapBound`); the third, the untap step's possessor,
    -- DISSOLVED on measurement into a spelling agreement with the
    -- subject, one to one across all seven lines. So `Whose` gains
    -- nothing and the spelling owns the possessive, which is
    -- `DoesntUntap`'s own arrangement one row up.
    -- The subject was a private three-value enum (`CapDomain`) for four
    -- chapters, because the plural player phrases it needed were not
    -- nouns yet. They are now (`PlayerGroup`), and its third cell was
    -- `You` from the start, so the enum retires here as finding 274 said
    -- it would: same three phrases, no vocabulary of their own.
    -- The SET is an ordinary predicate carrying `AllOf`'s three demands
    -- and no determiner: it must say a head noun ("land", "artifact",
    -- "creature", "permanents" -- `Headed`, `badUntapCapHeadless`), it
    -- must not hide the damage class, and it must describe permanents on
    -- the battlefield, [CR#502.3]'s untap reaching nothing else
    -- (`badUntapCapGraveyardSet`). Five sets are written and four are
    -- ordinary vocabulary; the fifth, "nonbasic land", needs a SUPERTYPE
    -- predicate this grammar does not have, so Winter Moon is recorded as
    -- corpus evidence and carries no bench positive.
    -- It files under `DeedRestriction` with `DoesntUntap`, the same class
    -- of statement for every kind-keyed reader, and it composes under
    -- `Conditionally` untouched -- Winter Orb and Static Orb write the cap
    -- inside an "as long as", which is the existing wrapper doing exactly
    -- what [CR#611.3a] licenses it for.
    -- spelling: ["<Param(0)> can't untap more than <Param(1)> <Param(2)>
    -- during <derived possessive> untap step(s)"], kind: Sentence
    -- (Param(0) is the subject noun's own phrase -- "players", "you",
    -- "your opponents"; the closing possessive and the step word's number
    -- are DERIVED from it, "your untap step" for the singular subject and
    -- "their untap steps" for the two plural ones. Param(1) is the bound
    -- as a WORD, "one"/"two", and the set word agrees with it in number)
    CantUntapMoreThan : (who : Noun bs Player) -> (k : Nat) ->
                        (p : Predicate bs Object) ->
                        {auto 0 cs : CapSubject who} ->
                        {auto 0 bd : CapBound k} ->
                        {auto 0 hd : Headed p} ->
                        {auto 0 zn : ZoneFits (seedZone p) (Just Battlefield)} ->
                        {auto 0 af : AnyTargetFree p} -> StaticEffect bs
    -- "[n] becomes [type line] in addition to its other types" — the
    -- ADDING type change, and [CR#205.1b] is the whole of why it is one
    -- construction rather than the type-SETTING one: that rule names the
    -- phrase outright ("effects that use phrases such as 'in addition to
    -- its other types'") and says the object retains all its prior types,
    -- where [CR#205.1a]'s bare setting replaces them. Core spells the
    -- change as layer-4 collection ops, `Modification::CardTypes(Add …)`
    -- and `Subtypes(Add …)` ([CR#613.1d]; `continuous.rs`), and the two
    -- lists here are those two ops under the one English phrase that
    -- writes them together ("becomes a Spirit artifact creature in
    -- addition to its other types"). Two hundred and twenty corpus lines.
    -- Three demands: the subject stands on the battlefield (a type change
    -- is a continuous effect on a permanent; `badBecomesInGraveyard`), the
    -- line says something (`badBecomesNothing`), and every subtype it adds
    -- sits on a card type the clause adds or the subject already has
    -- (`AddedFits`; `badBecomesZombieLand`). What the row does NOT spell
    -- is the type-SETTING sibling and the base-P/T setting that so often
    -- rides with it ("becomes a 5/5 creature") — the layer words, ledger.
    -- spelling: ["<Param(0)> becomes <Param(1)> in addition to its other
    -- types"] (Param(1) = the type line, subtypes before types, articles and
    -- pluralisation the renderer's; the trailing duration adverbial belongs
    -- to the Continuously envelope, not here), kind: Sentence
    BecomesAlso : (n : Noun bs Object) -> (added : TypeLine) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  {auto 0 ne : LineNonEmpty added} ->
                  {auto 0 nw : AddsSomething (nounTy n) added} ->
                  {auto 0 af : AddedFits (nounTy n) added} -> StaticEffect bs
    -- "[who] gain(s) control of [what]" -- the control-changing continuous
    -- effect ([CR#613.1b] layer 2; core's
    -- `Modification::SetController(Reference)`). ONE row for the whole
    -- English verb, span or none, which is a recorded DIVERGENCE from
    -- core: core keeps a one-shot `Action::GainControl` for the exchange
    -- family beside the layer-2 modification for the duration-bounded
    -- grants, two constructors for what English writes with one verb and
    -- one optional trailing adverbial. Here the adverbial is already the
    -- `Continuously` envelope's slot, and the unwritten span is
    -- [CR#611.2a]'s end-of-game default the same way it is for every other
    -- static -- which is what Phyrexian Infiltrator's reminder text says
    -- out loud, "(This effect lasts indefinitely.)" -- so the split would
    -- spell one construction twice.
    -- The SUBJECT is a slot for the reason `ChangeLife` and `Draw` have
    -- one: both spellings are ordinary oracle, the imperative with its
    -- unpronounced `You` and the written-out "[player] gains control of
    -- …", and the distributive subject is real too, so no grammatical
    -- number is demanded of it.
    -- The patient is a PERMANENT: [CR#110.2] gives every permanent a
    -- controller and [CR#109.4] gives objects that are neither on the
    -- stack nor on the battlefield none at all, so the battlefield demand
    -- tap, destroy, and "gets" already carry applies here too
    -- (`badGainControlGraveyard`). The stack half of [CR#109.4] is real
    -- English -- "exchange control of target noncreature spell and target
    -- creature" -- and waits with the spell carrier (ledger).
    -- spelling: ["<Param(0)> gain(s) control of <Param(1)>"] (the imperative
    -- leaves the agent unpronounced; the trailing duration adverbial belongs
    -- to the Continuously envelope, not here), kind: Sentence
    GainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                   {auto 0 zn : ZoneFits (nounZone what) (Just Battlefield)} -> StaticEffect bs
    -- "if [event], [replacement] instead" / "the next time [event],
    -- [replacement] instead" -- the REPLACEMENT effect as a continuous
    -- one, which is [CR#614.1]'s own first sentence ("Some continuous
    -- effects are replacement effects") and core's own filing
    -- (`StaticEffect::Replacement`, `deckmaste_core/src/continuous.rs`).
    -- The word is [CR#614.1a]'s: an effect that says "instead" is a
    -- replacement effect.
    -- The intercepted event is a PATTERN and not a clause: nothing is
    -- being instructed there, so it takes `GameEvent` rather than an
    -- `Effect`, and which events are writable at all is the event table's
    -- answer (`Interceptable`; `badInterceptDestruction`).
    -- The replacement clause is typed in what the event ANNOUNCED, so
    -- "exile it" finds the creature the event named -- and it is a HOLE
    -- outward, for the conditional arm's reason exactly ([CR#614.7]: if
    -- the intercepted event never happens the replacement "simply doesn't
    -- do anything"; `badInterceptReplacementAntecedent`).
    -- The multiplicity word is a slot because English makes it one and the
    -- event decides which ([CR#614.3]'s two endings; `ReplUse`,
    -- `replUseOk`).
    -- spelling: ["if <Param(0)>, <Param(1)> instead",
    -- "the next time <Param(0)>, <Param(1)> instead"] (the opening word is
    -- Param(2)'s -- see ReplUse; "instead" trails the replacement clause,
    -- and the fronted variant "instead <Param(1)>" is the same sentence
    -- linearized differently, a hundred sixty-eight corpus lines; the
    -- duration adverbial belongs to the Continuously envelope, not here),
    -- kind: Sentence
    Intercepts : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                 (use : ReplUse) ->
                 {auto 0 ok : Interceptable ev} ->
                 {auto 0 uo : ReplUseOk ev use} -> StaticEffect bs
    -- "prevent [all/the next N] [kind] damage that would be dealt
    -- [scope]" — the prevention shield, [CR#615.1]'s continuous effect
    -- and core's `StaticEffect::Prevention` (`continuous.rs`) at the
    -- same address. [CR#615.1a] makes the word the whole test: an effect
    -- that says "prevent" is a prevention effect.
    -- Three slots because oracle writes three and no more: the damage
    -- CLASS ([CR#510.2]'s combat adjective and its negation), the SIZE
    -- ([CR#615.7]'s numbered shield against the unnumbered one), and the
    -- SCOPE, which is the recipient or nothing at all. The size and the
    -- scope are independent — Fog's unnumbered shield names no recipient
    -- and Shieldmate's Blessing's numbered one names a target — so
    -- neither can be folded into the other.
    -- No SOURCE restriction: "by attacking creatures", "by a source of
    -- your choice", "by sources you don't control" are a hundred and
    -- forty-odd lines of a rider [CR#609.7] gives its own machinery and
    -- this vocabulary has no phrase for (ledger).
    -- spelling: ["prevent <Param(1)> <Param(0)> damage that would be dealt
    -- <Param(2)>"] (Param(1) writes "all" or "the next <amount>", Param(0)
    -- the bare noun or its adjective, Param(2) the "to <recipient>" phrase
    -- or nothing; the duration adverbial belongs to the Continuously
    -- envelope, not here), kind: Sentence
    Prevents : (kind : DamageKind) -> (size : Shield bs) ->
               (scope : DamageScope (shieldIntro size)) -> StaticEffect bs
    -- "as long as [condition], [effect]" -- the CONDITIONAL static
    -- ([CR#611.3a] -- a continuous effect from a static ability "isn't
    -- 'locked in'; it applies at any given moment to whatever its text
    -- indicates", which is what makes a live condition possible at all),
    -- and core's `StaticEffect::Conditionally(Condition, StaticEffect)` at
    -- the same address (`continuous.rs`, cited to the same rule).
    -- It WRAPS rather than takes a field, which is core's own note and
    -- English's: the clause is one adverbial over a whole statement, and
    -- the statement it wraps is any of the others. The bare "as long as"
    -- and the DURATION adverbial [CR#611.2b] spells with "for" in front
    -- are one preposition apart and two different constructions, which is
    -- why the duration row and this one both exist
    -- (`badConditionalClause`).
    -- THE THREADING RUNS CONDITION-TO-STATEMENT, and this comment used to
    -- claim the opposite. It described an intent that was never built:
    -- both arguments sat at the raw `bs` and neither read the other, so
    -- "As long as Frodo Baggins is your Ring-bearer, it must be blocked if
    -- able" had nothing for its "it" to resolve against. Chapter fifty
    -- builds the threading the corpus asks for -- 141 of the 424 fronted
    -- lines pronominalise the CONDITION's subject in the body -- by typing
    -- the statement at `condIntro c`.
    -- What is threaded is ONLY the self, and only as a `SelfD` mention:
    -- the condition itself still announces nothing (`condDelta`, every row
    -- the empty list), so this is the container minting for its own body
    -- and not the condition becoming transparent.
    -- It does NOT nest itself (`badDoubleConditional`), and the MARKING
    -- word's one gate is a polarity check: "unless [C]" IS "as long as
    -- not [C]" with the negation moved onto the subordinator, so the row
    -- demands a negated condition and spells the positive inside
    -- (`MarkingOk`, `badUnlessOnPositive`).
    -- spelling: ["as long as <Param(0)>, <Param(1)>"] (the fronted form is
    -- the corpus's ordinary one; the trailing "…, as long as <Param(0)>"
    -- linearization is the same sentence WHEN THE CONDITION IS
    -- INDEPENDENT -- "This creature gets +4/+4 as long as there are seven
    -- or more cards in your graveyard" -- and the inner statement supplies
    -- its own frame. It is NOT the same sentence when a pronoun crosses:
    -- the fronted form pronominalises the CONDITION's subject in the body
    -- (141 lines, threaded here) and the trailing form pronominalises the
    -- STATEMENT's subject in the condition ("X has hexproof as long as
    -- IT's untapped", about 70), which needs the threading to run the
    -- other way. One constructor cannot type both arguments in each
    -- other's context, so the trailing-with-pronoun flow waits for its own
    -- orientation and only the independent trailing form is this row's. Under the Unless marking
    -- the clause is written the other way round and the condition sheds
    -- its own negation: "<Param(1)> unless <Param(0)'s positive>"),
    -- kind: Sentence
    Conditionally : (c : Condition bs) -> (se : StaticEffect (condIntro c)) ->
                    {default AsLongAs marking : CondMarking} ->
                    {auto 0 nn : NotConditional se} ->
                    {auto 0 mk : MarkingOk marking c} -> StaticEffect bs
    -- "[who] may play [what]" -- the PERMISSION static, core's
    -- `Deontic::May(Cast{…})` (`deontic.rs`) and the permissive twin of
    -- `Cant`. [CR#604.6] is the rule that files it: a static ability of
    -- this shape "appl(ies) while a card is in any zone that you could
    -- cast or play it from", and it names the templates outright.
    -- The VERB is a SLOT, [CR#604.6] bracketing it as one. Deriving it
    -- from the complement's kind was wrong rather than blocked:
    -- [CR#701.5b] says "to cast a card is to cast it as a spell", so both
    -- verbs take a card. What separates them is the LAND -- [CR#601.1a]
    -- makes "play" the union of playing a land and casting a spell, and
    -- [CR#305.9] makes the exclusion a rule, "it can't be cast as a
    -- spell" -- so the gate is a type demand on the complement's head and
    -- not a zone one (`castableTy`, `badCastALand`).
    -- The SOURCE zone is a DEFAULTED slot: the impulse family exiles its
    -- cards in the sentence before and says "that card" with no zone
    -- phrase, so the binding already carries the zone, while the graveyard
    -- family writes the zone IN the permission over a complement that
    -- carries none. One slot, `Nothing` meaning "read it off the
    -- complement" (`playSourceOk`).
    -- The permission is NOT the deed restriction with a polarity flip:
    -- `Cant` demands a battlefield subject because combat is fought there,
    -- and this row demands the exact opposite (`badPlayFromBattlefield`).
    -- spelling: ["<Param(0)> may play <Param(1)>", "<Param(0)> may cast
    -- <Param(1)>", "<Param(0)> may play <Param(1)> from <Param(3)>",
    -- "<Param(0)> may cast <Param(1)> from <Param(3)>"] (selects on the
    -- verb slot and on whether the source is written; the duration adverbial
    -- belongs to the Continuously envelope, not here; the fronted
    -- linearization "Until your next end step, you may play those cards" is
    -- the same sentence), kind: Sentence
    MayPlay : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
              {default Play verb : PlayVerb} ->
              {default Nothing from : Maybe (ZoneExpr (nomIntro what))} ->
              {auto 0 pz : PlaySource (nounZone what) from} ->
              {auto 0 cv : CastableTy verb (nounTy what)} -> StaticEffect bs
    -- "[n] enters tapped" -- the ENTRY rider, and the third family chapter
    -- twenty-five sent to a container that did not exist. [CR#603.6d]
    -- settles what it is in as many words: text reading "[This permanent]
    -- enters tapped" is "a static ability -- not a triggered ability --
    -- whose effect occurs as part of the event that puts the permanent
    -- onto the battlefield", and [CR#614.1d] files that effect as a
    -- replacement. So the SENTENCE is a static ability line, which is what
    -- this row is, and the replacement machinery is the engine's.
    -- The rider vocabulary is chapter nineteen's `TokenRider`, shared
    -- rather than re-minted -- the token's with-clause and the permanent's
    -- own line name the same two riders -- and the line writes only ONE of
    -- them: "enters attacking" is written zero times as a permanent's own
    -- ability, because a permanent's line cannot know there is a combat
    -- (`entryRiderOk`, `badEntersAttackingLine`).
    -- spelling: ["<Param(0)> enters <Param(1)>"] (Param(1) writes the bare
    -- participle, "tapped"; the corpus's three bare lines are "This
    -- artifact/creature/land enters tapped."), kind: Sentence
    EntersRider : (n : Noun bs Object) -> (rider : TokenRider) ->
                  {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                  {auto 0 ro : EntryRiderOk rider} -> StaticEffect bs
    -- "[n] enters with [amt] [kind] counters on it" -- the OTHER entry
    -- replacement, and the same [CR#603.6d,614.1d] pair files it: the
    -- sentence is a static ability and the replacement is the engine's.
    -- The ledger held it on the observation that it "wants a counter
    -- AMOUNT where `TokenRider` is a two-row enum", and the re-check is
    -- that the observation was about the wrong container. `TokenRider` is
    -- SHARED with `TokenChars`, an unindexed record, so an `Amount bs`
    -- cannot go in it at any price; put here instead, the amount and the
    -- kind are the vocabulary `PutCounters` has carried since chapter
    -- nineteen, unchanged and unwidened -- which is why this is a second
    -- constructor and not a third rider, and why it shares `EntryRider` as
    -- its `staticKind`: the two sentences are one class of statement and
    -- every table that keys on the kind already has its cell.
    -- The COUNT is `WrittenCount`'s written magnitude; the KIND is
    -- `CounterKind`, which reaches under half the corpus lines and leaves
    -- the rest to the counter catalog itself -- oil, fade, charge,
    -- indestructible, finality, shield and their neighbours (ledger).
    -- spelling: ["<Param(0)> enters with <Param(1)> <Param(2)> counter(s)
    -- on it"] (the counter noun pluralises with the count and the kind
    -- writes its own word before it -- see CounterKind), kind: Sentence
    EntersWithCounters : (n : Noun bs Object) -> (amt : Amount bs) ->
                         (kind : CounterKind) ->
                         {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                         {auto 0 wc : WrittenCount amt} -> StaticEffect bs

  ||| Which riders a PERMANENT's own line writes, against the two chapter
  ||| nineteen minted for a token's with-clause. One row of the two, and
  ||| the asymmetry is the finding: a token is created BY a resolving
  ||| effect that knows whether combat is happening, so "create a 1/1 …
  ||| token tapped and attacking" is real; a permanent's own static ability
  ||| applies whenever that permanent enters, from any zone and in any
  ||| step, so it can say "tapped" and has no occasion to say "attacking".
  public export
  entryRiderOk : TokenRider -> Bool
  entryRiderOk EntersTapped = True
  entryRiderOk EntersAttacking = False

  public export
  data EntryRiderOk : TokenRider -> Type where
    MkEntryRiderOk : {auto 0 ok : entryRiderOk r = True} -> EntryRiderOk r

  ||| The POLARITY a deed clause carries, with the gate's cost on its own
  ||| value. See CompTag for the convergence's argument and its three
  ||| prior arts.
  public export
  data Compulsion : Bindings -> Type where
    -- "can't [deed]" ([CR#508.1c,509.1b] — a restriction)
    Forbid : Compulsion bs
    -- "[deed]s … if able" ([CR#508.1d,509.1c] — a requirement)
    Require : Compulsion bs
    -- "can't [deed] unless [payer] pays [cost]" -- the GATE, and a
    -- restriction with a price rather than a third thing: [CR#508.1d] and
    -- [CR#509.1c] both name it in their own text ("if a creature can't
    -- attack unless a player pays a cost, that player is not required to
    -- pay that cost"), which is also the rule that keeps it out of the
    -- requirement solver.
    GatedBy : (c : Cost bs) -> Compulsion bs

  public export
  compulsionTag : {0 bs : Bindings} -> Compulsion bs -> CompTag
  compulsionTag Forbid = ForbidT
  compulsionTag Require = RequireT
  compulsionTag (GatedBy _) = GateT

  ||| The patient slot's gate, reading `deonticPatientOk` at the cell the
  ||| clause names. `BlockPartner`'s family with a three-valued table
  ||| behind it: the unwritten cell is admitted where the table does not
  ||| REQUIRE a patient, and the written one where it does not REFUSE one.
  public export
  data DeonticPatient : {0 bs : Bindings} -> CompTag -> Deed -> Role ->
                        Maybe (Noun bs Object) -> Type where
    NoDeonticPatient : {auto 0 ok : notRequired (deonticPatientOk t d r) = True} ->
                       DeonticPatient t d r Nothing
    DeonticPatientWritten : {0 m : Noun bs Object} ->
                            {auto 0 ok : admitsPatient (deonticPatientOk t d r) = True} ->
                            {auto 0 zn : ZoneFits (nounZone m) (Just Battlefield)} ->
                            DeonticPatient t d r (Just m)

  ||| The optional KIND on the player's removal verb. `CtrlSingular`'s
  ||| shape at a third site, and here it carries the scope demand: a named
  ||| kind must be a player's ([CR#122.1]'s two scopes, `counterScope`),
  ||| while the unwritten cell has nothing to demand because the sentence
  ||| names nothing to check. "Each opponent loses all +1/+1 counters" is
  ||| refused by this gate (`badLosesAllBoostCounters`); "Each opponent
  ||| loses all counters" passes it saying nothing.
  public export
  data PlayerCounterKind : Maybe CounterKind -> Type where
    AllKindsLost : PlayerCounterKind Nothing
    OneKindLost : {0 k : CounterKind} ->
                  {auto 0 sc : counterScope k = Player} ->
                  PlayerCounterKind (Just k)

  ||| A static statement that is not already conditioned. The corpus
  ||| writes one "as long as" per statement and never two, which is the
  ||| singleton discipline `badNestedCompound` and `badDoubleNegated`
  ||| already keep at two other sites.
  public export
  notConditional : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notConditional (Conditionally _ _) = False
  notConditional _ = True

  ||| A statement that does not already carry a definition rider -- the
  ||| gate that keeps the rider OUTSIDE a duration adverbial. Both
  ||| orders are expressible and they spell the same sentence, and the
  ||| corpus fixes which one is written: of the pump lines that carry
  ||| both, 245 write the duration first ("gets +X/+0 until end of turn,
  ||| where X is …") and NONE writes it after the rider. So the duration
  ||| belongs to the clause and the rider binds the whole clause, which
  ||| is `WhereLetter` at the effect frame; a rider inside the envelope
  ||| is the second spelling of one sentence and is refused here
  ||| (`badRiderInsideDuration`). It is `pumpSignsOk`'s discipline at a
  ||| third site: one term per sentence.
  public export
  notLetterRider : {0 bs : Bindings} -> StaticEffect bs -> Bool
  notLetterRider (WhereLetterStatic _ _ _) = False
  notLetterRider _ = True

  public export
  data NotLetterRider : StaticEffect bs -> Type where
    MkNotLetterRider : {auto 0 ok : notLetterRider se = True} -> NotLetterRider se

  public export
  data NotConditional : StaticEffect bs -> Type where
    MkNotConditional : {auto 0 ok : notConditional se = True} -> NotConditional se

  ||| HOW MUCH damage a shield stops — [CR#615.7]'s numbered shield against
  ||| the unnumbered one, and they are two rows rather than one because the
  ||| rule treats them differently: a numbered shield is "used up" one
  ||| damage at a time and an unnumbered one is not used up at all,
  ||| expiring only with its duration ([CR#615.3]). English marks the same
  ||| split with two determiners, "all" and "the next [N]", and writes no
  ||| third: no line prevents "half the damage" or "up to 3 damage". The
  ||| number is the ordinary magnitude vocabulary and no parallel number
  ||| path — a numeral ("Prevent the next 3 damage …", Healing Salve) or
  ||| the announced X, which is what `WrittenCount` already spells.
  public export
  -- spelling: ["all", "the next <Param(0)>"] (the determiner before the
  -- damage noun; spelled only through StaticEffect.Prevents)
  data Shield : Bindings -> Type where
    AllOfIt : Shield bs
    TheNext : (amt : Amount bs) -> {auto 0 wc : WrittenCount amt} -> Shield bs

  public export
  shieldIntro : {bs : Bindings} -> Shield bs -> Bindings
  shieldIntro AllOfIt = bs
  shieldIntro (TheNext amt) = amtIntro amt

  ||| WHO the shield stands in front of. Two rows, and the empty one is not
  ||| an omission: "Prevent all combat damage that would be dealt this
  ||| turn" (Fog, Holy Day) names no recipient at all and shields every
  ||| damage event of its class, which [CR#611.2c] calls modifying the
  ||| rules of the game and gives its own worked example of. The recipient
  ||| row reads the damage clause's own table (`DamageRecipient`), so what
  ||| may be shielded is exactly what may be damaged.
  public export
  -- spelling: ["", "to <Param(0)>"] (row order: Everywhere/ToRecipient;
  -- the empty row writes nothing after "dealt". Spelled only through
  -- StaticEffect.Prevents)
  data DamageScope : Bindings -> Type where
    Everywhere : DamageScope bs
    ToRecipient : {k : Kind} -> (n : Noun bs k) ->
                  {auto 0 rk : DamageRecipient n} -> DamageScope bs

  public export
  scopeIntro : {bs : Bindings} -> DamageScope bs -> Bindings
  scopeIntro Everywhere = bs
  scopeIntro (ToRecipient n) = nomIntro n

  ||| Which row a static effect is, for the span tables.
  public export
  staticKind : {0 bs : Bindings} -> StaticEffect bs -> StaticKind
  -- the rider carries no kind of its own: what the sentence IS is what
  -- its statement is, and the definition rides along.
  staticKind (WhereLetterStatic _ _ se) = staticKind se
  staticKind (Gets _ _ _) = PtDelta
  staticKind (CostsToCast _ _) = CostModification
  staticKind (Gains _ _) = KeywordGrant
  staticKind (Deontic _ _ _ _ _) = DeedRestriction
  staticKind (DoesntUntap _) = DeedRestriction
  staticKind (CantUntapMoreThan _ _ _) = DeedRestriction
  staticKind (MayDeclineUntap _) = DeedRestriction
  staticKind (OutcomeGate _ _) = DeedRestriction
  staticKind (BecomesAlso _ _) = TypeAddition
  staticKind (GainsControl _ _) = ControlGrant
  staticKind (Intercepts _ _ _) = Replacement
  staticKind (Prevents _ _ _) = Prevention
  staticKind (Conditionally _ _) = Conditional
  staticKind (MayPlay _ _) = PlayPermission
  -- Both entry replacements answer the SAME kind, which is the claim
  -- that they are one class of statement: [CR#603.6d,614.1d] file them
  -- together and every table keyed on the kind already has the cell.
  staticKind (EntersRider _ _) = EntryRider
  staticKind (EntersWithCounters _ _ _) = EntryRider

  ||| May this statement stand as a bare ability LINE? `staticAsAbility`
  ||| asked of the kind, plus the one thing a kind cannot say: the
  ||| conditional is a WRAPPER, and [CR#604.1] has a static ability
  ||| "written as a statement" that is "simply true", so an "as long as"
  ||| clause is a line exactly when the statement it qualifies is one. The
  ||| wrapper's own cell said yes unconditionally, which let the control
  ||| grant — whose stative form is a different verb — reach the line
  ||| through it (`badConditionalGainControl`). It recurses on the PAYLOAD
  ||| only, the condition being a truth test and not a statement. The kind
  ||| stays what it was: `absentOk` and `SpanOk` read the wrapper's own
  ||| cell (`badConditionalClause`).
  public export
  staticLineOk : {0 bs : Bindings} -> StaticEffect bs -> Bool
  staticLineOk (Conditionally _ se) =
    staticAsAbility Conditional && staticLineOk se
  -- the rider is a definition and not a statement, so the line question
  -- is its body's alone (contrast the conditional, whose wrapper has a
  -- cell of its own).
  staticLineOk (WhereLetterStatic _ _ se) = staticLineOk se
  staticLineOk se@(Gets _ _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(CostsToCast _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(Gains _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(Deontic _ _ _ _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(DoesntUntap _) = staticAsAbility (staticKind se)
  staticLineOk se@(CantUntapMoreThan _ _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(MayDeclineUntap _) = staticAsAbility (staticKind se)
  staticLineOk se@(OutcomeGate _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(BecomesAlso _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(GainsControl _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(Intercepts _ _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(Prevents _ _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(MayPlay _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(EntersRider _ _) = staticAsAbility (staticKind se)
  staticLineOk se@(EntersWithCounters _ _ _) = staticAsAbility (staticKind se)

  ||| What a continuous clause contributes to the discourse: its
  ||| subject, exactly as the one-shot clauses contribute theirs.
  ||| The two SHIELD rows have no subject and contribute what their
  ||| phrases announced instead: the intercepted event's own noun, and
  ||| the shield's recipient. Neither contributes what its clause WOULD
  ||| do, the replacement being a hole for [CR#614.7]'s reason.
  public export
  staticIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticIntro (WhereLetterStatic _ _ se) = staticIntro se
  staticIntro (Gets n _ _) = nomIntro n
  staticIntro (CostsToCast n _) = nomIntro n
  staticIntro (Gains n _) = nomIntro n
  staticIntro (Deontic n _ _ _ _) = nomIntro n
  staticIntro (DoesntUntap n) = nomIntro n
  -- the cap announces NOTHING, and it is the first static row with no
  -- subject phrase to announce: the domain is a closed word rather than a
  -- noun, and the set sits under a bound with no determiner over it, so
  -- there is no mention for a later reader to pick up. `BeginningOf`'s
  -- answer at the event layer, for `BeginningOf`'s reason.
  staticIntro (CantUntapMoreThan _ _ _) = bs
  staticIntro (MayDeclineUntap n) = nomIntro n
  staticIntro (OutcomeGate _ who) = nomIntro who
  staticIntro (BecomesAlso n _) = nomIntro n
  staticIntro (GainsControl who what) = nomIntro what
  staticIntro (Intercepts ev repl use) = eventIntro ev
  staticIntro (Prevents kind size scope) = scopeIntro scope
  -- the wrapper announces what its statement announced: the "as long as"
  -- phrase is an adverbial and a condition contributes nothing.
  staticIntro (Conditionally c se) = staticIntro se
  staticIntro (MayPlay who what) = nomIntro what
  staticIntro (EntersRider n _) = nomIntro n
  staticIntro (EntersWithCounters n _ _) = nomIntro n

  ||| Which clause a DIVISION writes. [CR#601.2d] and [CR#115.7f] both name
  ||| "divide or distribute" as one mechanic over one pair of examples —
  ||| "such as damage or counters" — and English writes that one mechanic
  ||| with two idioms and no third: the damage verb takes an adverbial
  ||| ("Arc Lightning deals 3 damage DIVIDED AS YOU CHOOSE among one, two,
  ||| or three targets") and the counter verb changes its own word
  ||| ("DISTRIBUTE two +1/+1 counters among one or two target creatures you
  ||| control", Armament Corps). Nothing else is divided, and the two
  ||| spellings never cross ("counters divided as you choose" is zero lines
  ||| and so is "distribute … damage"), so the table is closed at two.
  |||
  ||| A recorded NARROWING of core, which takes a free `body` and reads the
  ||| share back through a `Count::Allotment` anaphor
  ||| (`deckmaste_core/src/effect.rs`, `count.rs`). That generality is right
  ||| for an engine and wrong for this grammar: no oracle line writes a
  ||| divided clause whose body is anything but one of these two. The share
  ||| stays implicit here for the same reason — the words "divided as you
  ||| choose" ARE the allotment.
  public export
  data DividedVerb : Bindings -> Type where
    -- "[src] deals [amt] damage divided as you choose among [group]"
    DividedDamage : (src : Noun bs Object) ->
                    {auto 0 ds : DamageSource src} -> DividedVerb bs
    -- "Distribute [amt] [kind] counters among [group]" — agent-silent,
    -- exactly as `PutCounters` is.
    DistributedCounters : (kind : CounterKind) -> DividedVerb bs

  ||| What a divided verb has already announced when its amount is
  ||| written — the source phrase for damage, nothing for counters.
  public export
  divIntro : {bs : Bindings} -> DividedVerb bs -> Bindings
  divIntro (DividedDamage src) = nomIntro src
  divIntro (DistributedCounters _) = bs

  ||| The divided verb as a tag, so the recipient obligation can be
  ||| stated per row without carrying the row's own arguments.
  public export
  data DivTag = DivDamage | DivCounters

  public export
  divTag : {0 bs : Bindings} -> DividedVerb bs -> DivTag
  divTag (DividedDamage _) = DivDamage
  divTag (DistributedCounters _) = DivCounters

  ||| What each divided verb demands of the group it divides among —
  ||| the SAME demand the undivided clause makes of its recipient, since
  ||| dividing changes how much each member gets and not what a member
  ||| may be. Damage reuses `DamageRecipient` whole (which is what lets
  ||| the class word stand here: "among any number of targets"); the
  ||| counter verb demands the BATTLEFIELD and not `PutCounters`' two
  ||| zones, and that is measured rather than inherited: every
  ||| "distribute … counters among" line divides among creatures on the
  ||| battlefield, and no line divides counters among cards in exile —
  ||| the exile family writes one card at a time. Its `k = Object` is
  ||| what stops a division of counters among players.
  public export
  data DividedTakes : DivTag -> Noun bs k -> Type where
    DamageDivided : {auto 0 rk : DamageRecipient n} -> DividedTakes DivDamage n
    CountersDistributed : {auto 0 zn : OnBattlefield (nounZone n)} ->
                          DividedTakes DivCounters {k = Object} n

  ||| WHAT an exposure clause exposes — the two complements English writes
  ||| after "look at" and "reveal", measured rather than guessed. A card
  ||| GROUP is the dominant one and the one this chapter is built around
  ||| ("look at the top <n> cards of your library"). A whole HAND is the
  ||| other, and it is a zone rather than a group of cards ("reveals their
  ||| hand", "look at target player's hand"). Oracle never spells that one
  ||| out as a group — "reveal all cards in your hand" is zero lines — so
  ||| the zone phrase is the construction and not an abbreviation of one.
  public export
  data Exposed : Bindings -> Type where
    -- spelling: ["<Param(0)>"] (the card phrase itself, e.g. "the top four
    -- cards of your library"), kind: Nominal
    ExposedCards : (n : Noun bs Object) -> Exposed bs
    -- spelling: ["<Param(0)>"] (the zone phrase under its possessive, e.g.
    -- "their hand", "target player's hand"), kind: Nominal
    ExposedZone : (z : ZoneExpr bs) ->
                  {auto 0 ok : ExposableZone (zoneSort z)} -> Exposed bs

  ||| What an exposure contributes to the discourse. The card group is a
  ||| mention like any other and the whole point of the clause. The HAND is
  ||| a hole, and measured: every corpus line that reads a revealed hand
  ||| back reads it as a ZONE ("that player exiles a card from IT"), which
  ||| is a zone anaphor this grammar has no word for — so the clause
  ||| contributes its possessor and nothing else (ledger).
  public export
  exposedIntro : {bs : Bindings} -> Exposed bs -> Bindings
  exposedIntro (ExposedCards n) = nomIntro n
  exposedIntro (ExposedZone z) = zoneDelta z ++ bs

  ||| The WITH-COUNTERS rider — "exile it with three time counters on it",
  ||| "return that card to the battlefield under its owner's control with a
  ||| +1/+1 counter on it". It is the third rider on the `MoveRiders`
  ||| bundle and NOT a structure of its own, and the reason is that the
  ||| corpus puts it in the same slot as the other two: it trails the
  ||| destination phrase, it combines with the controller override
  ||| (Daydream writes both in one placement), and no line writes it on
  ||| anything but a placement.
  |||
  ||| Finding 192 said an `Amount bs` "was never going to fit" a rider slot
  ||| at any price, and that was true of `TokenRider` for a reason that
  ||| does NOT hold here: `TokenRider` is shared with the unindexed
  ||| `TokenChars`, where `MoveRiders` has carried a `Noun bs Player` since
  ||| chapter thirty and is indexed already. So the amount is typed in the
  ||| patient's own announcement, which is what "with a number of time
  ||| counters on it equal to ITS mana value" needs.
  |||
  ||| "On it" is NOT optional and the measurement is total: every corpus
  ||| line ends "on it" and none stops at "counters". The pronoun is the
  ||| patient of the same placement, so it is part of this construction's
  ||| own spelling rather than a slot.
  public export
  data CounterRider : Bindings -> Type where
    -- spelling: ["with <Param(0)> <Param(1)> counter(s) on it"] (the count
    -- and the kind as PutCounters writes them, at one the article "a"; the
    -- trailing "on it" is fixed -- see above)
    MkCounterRider : (amt : Amount bs) -> (kind : CounterKind) ->
                     {auto 0 wc : WrittenCount amt} -> CounterRider bs

  ||| The ARRIVAL riders a placement writes — "put that card onto the
  ||| battlefield TAPPED", "tapped AND ATTACKING", "UNDER YOUR CONTROL".
  ||| A closed BUNDLE and not three constructors, because the riders
  ||| COMBINE: "put it onto the battlefield tapped under your control" is
  ||| one placement wearing two of them, and separate `MoveTapped` /
  ||| `MoveUnderControl` rows would have multiplied the verb by its
  ||| combinations.
  |||
  ||| The entry half is chapter nineteen's `TokenRider` list SHARED and its
  ||| `ridersOk` shapes verbatim — a token's with-clause and a placement's
  ||| adverbial name the same two riders in the same fixed order, which is
  ||| finding 179's sharing at a third site. The CONTROLLER is a separate
  ||| field and not a third rider word, because it takes a noun where the
  ||| other two are bare participles, and it is an OVERRIDE rather than a
  ||| required field: [CR#110.2a] already gives an instructed player
  ||| control of what they put onto the battlefield, so the unwritten slot
  ||| is the rule's default and the written ones are departures from it.
  ||| One controller, [CR#109.4]'s own singular (`CtrlSingular`).
  public export
  data MoveRiders : Bindings -> Type where
    -- spelling: (construction-owned -- the adverbials trailing the
    -- destination phrase, entry riders first, then the controller, then
    -- the counters: "onto the battlefield tapped and attacking under your
    -- control with a +1/+1 counter on it".
    -- Each entry rider is TokenRider's own word; the controller writes
    -- "under <Param(1)>'s control" with the possessive; the counter rider
    -- writes its own phrase. An empty bundle writes nothing at all.)
    MkMoveRiders : (entry : List TokenRider) ->
                   (ctrl : Maybe (Noun bs Player)) ->
                   {default Nothing counters : Maybe (CounterRider bs)} ->
                   {auto 0 ro : RidersOk entry} ->
                   {auto 0 one : CtrlSingular ctrl} -> MoveRiders bs

  ||| A control override names ONE player ([CR#109.4] — an object on the
  ||| battlefield has a controller, not controllers), the same demand
  ||| `ControlledBy` and `GainsControl` make of their possessor.
  public export
  data CtrlSingular : {0 bs : Bindings} -> Maybe (Noun bs Player) -> Type where
    NoOverride : CtrlSingular Nothing
    OneController : {0 n : Noun bs Player} ->
                    {auto 0 one : nounPlur n = OneOf} -> CtrlSingular (Just n)

  ||| The bundle's BATTLEFIELD half — the two entry participles and the
  ||| controller, which are what chapter thirty gated as one.
  public export
  fieldRidersWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  fieldRidersWritten (MkMoveRiders [] Nothing) = False
  fieldRidersWritten _ = True

  ||| …and its second half, asked apart because it answers a different
  ||| zone question.
  public export
  counterRiderWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  counterRiderWritten (MkMoveRiders _ _ {counters = Nothing}) = False
  counterRiderWritten (MkMoveRiders _ _ {counters = Just _}) = True

  ||| Does the bundle write anything? The empty bundle is the placement
  ||| with no adverbial after it, which is what every `Move` before
  ||| chapter thirty wrote.
  public export
  ridersWritten : {0 bs : Bindings} -> MoveRiders bs -> Bool
  ridersWritten r = fieldRidersWritten r || counterRiderWritten r

  ||| The bundle's zone admissibility is PER HALF, and this round is what
  ||| split it. The entry participles and the controller are
  ||| BATTLEFIELD-only, and the reason is that there is nothing for one to
  ||| say anywhere else: both entry riders are battlefield ARRIVALS
  ||| ([CR#110.5b] — permanents "enter the battlefield untapped … unless a
  ||| spell or ability says otherwise", which is the tapped rider's own
  ||| rule; [CR#506.3a] words the second one as an effect that "would put
  ||| a … permanent onto the battlefield attacking"), and [CR#109.4] gives
  ||| an object off the stack and off the battlefield no controller to
  ||| override. Zero corpus lines write one on any other destination
  ||| (`badMoveRidersToGraveyard`, `badMoveRidersToHand`).
  |||
  ||| The COUNTER rider answers `counterZone` instead — the same
  ||| two-zone table the put and remove verbs answer, and the same table
  ||| rather than a second one because it is the same fact about where a
  ||| counter may sit. That is what keeps the two halves from leaking
  ||| into each other: a tapped rider cannot ride an exile
  ||| (`badExileTapped`) and a counter rider cannot ride a move into a
  ||| graveyard or a hand (`badMoveCountersToGraveyard`), while the
  ||| battlefield takes both at once and exile takes only the second.
  public export
  ridersFitZone : {0 bs : Bindings} -> MoveRiders bs -> Zone -> Bool
  ridersFitZone r z =
    (not (fieldRidersWritten r) || sameZone z Battlefield) &&
    (not (counterRiderWritten r) || counterZone (Just z))

  ||| The bundle's zone demand as a witness, named apart so a pin says
  ||| which question refused.
  public export
  data RidersFit : {0 bs : Bindings} -> MoveRiders bs -> Zone -> Type where
    MkRidersFit : {0 r : MoveRiders bs} ->
                  {auto 0 ok : ridersFitZone r z = True} -> RidersFit r z

  ||| A COST — what the activator PAYS, which is a different thing from
  ||| what an ability DOES ([CR#602.1a]: "the activation cost is everything
  ||| before the colon … must be paid by the player who is activating
  ||| it"). The type is what closes the ledger's cost-GRAMMAR entry: the
  ||| colon used to accept any clause at all, so "Draw a card:" was
  ||| writable and no rule made it so.
  |||
  ||| Its payloads REUSE the clause machinery rather than parallel it —
  ||| [CR#118.1] makes a cost "an action or payment" the payer carries out,
  ||| so "Sacrifice a creature" is the sacrifice CLAUSE under `Do` and not
  ||| a second sacrifice verb. What is NOT a clause is the two SYMBOLS:
  ||| [CR#107.5] gives "{T}" the fixed meaning "tap this permanent", a
  ||| self-patient with no noun phrase in it, and the corpus writes the
  ||| symbol far more often than the English tap clause ("Tap an untapped
  ||| creature you control"), which is two surfaces and not one. Core keeps
  ||| the same split (`CostComponent::{Tap, Untap}` beside `Do(Action)`);
  ||| the settled Idris spec spells `{T}` as `Do (Tap This)` instead, and
  ||| that is a recorded DIVERGENCE — a semantic normalization this grammar
  ||| cannot make, the symbol being a different thing on the page.
  |||
  ||| What the grammar claims is the sentence's cost STRUCTURE and nothing
  ||| about the game: not payability ([CR#118.3]), not timing, not the
  ||| total-cost pipeline ([CR#601.2f], core's `CostChange`), not the
  ||| alternative-cost swap ([CR#118.9]). Those are engine boundaries and
  ||| stay ledgered.
  public export
  data Cost : Bindings -> Type where
    -- The mana component ([CR#202.1a]) — a symbol sequence, whose whole
    -- spelling is the symbols run together.
    -- spelling: ["<Param(0)>"] (the symbol list, no separator: "{1}{R}",
    -- "{X}{X}{G}" -- see ManaSymbol for the per-symbol rendering)
    Mana : (c : ManaCost) -> {auto 0 wr : ManaRun c} -> Cost bs
    -- "{T}" ([CR#107.5]) — the symbol, whose patient is the permanent
    -- with the ability and is never written.
    -- spelling: ["{T}"], kind: Cost
    TapSymbol : Cost bs
    -- "{Q}", the untap symbol — attested at eighteen cost components
    -- (Merrow Grimeblotter), and the reason it is a row beside `{T}`
    -- rather than a negation of one: [CR#602.5a] names the two together
    -- as the summoning-sickness pair, which is what core's
    -- `CostPredicate::IncludesTapSymbol` asks about.
    -- spelling: ["{Q}"], kind: Cost
    UntapSymbol : Cost bs
    -- Pay by PERFORMING an action ([CR#118.1] — "to pay a cost, a
    -- player carries out the instructions") — the clause itself,
    -- gated to the payable verbs (`CostAction`). Core's
    -- `CostComponent::Do` and the settled spec's `Do` alike.
    -- spelling: ["<Param(0)>"] (the clause's own sentence, capitalised as
    -- a cost component: "Sacrifice a creature", "Discard a card", "Pay 2
    -- life" -- the life payment's cost spelling is the verb "pay", where
    -- the same clause in sentence position writes "loses"; see CostAction)
    Do : (e : Effect bs) -> {auto 0 ok : CostAction e} -> Cost bs
    -- Two or more components at once, which oracle joins with commas
    -- ("{1}{B}, Pay 2 life:", Erebos, God of the Dead). Order is TEXTUAL
    -- and not an order of payment: [CR#601.2h] has the player pay the
    -- components "in any order", which is why the elements thread
    -- announcements and why nothing here should read a sibling's deed —
    -- no corpus line does.
    -- spelling: ["<Param(0)>"] (the components joined by ", " in written
    -- order; the whole sequence precedes the colon)
    Compound : {0 n : Nat} -> CostSeq n bs ->
               {auto 0 two : AtLeastTwo n} -> Cost bs

  ||| What a cost's phrases announce to the effect after the colon. A mana
  ||| amount and the two symbols name nobody; an action component names
  ||| whatever its clause named ("Sacrifice a creature:" — Bosh, Iron
  ||| Golem's cost is read by its effect). The activated ability filters
  ||| this through `publicOnly`, which is the survivorship question and
  ||| not this one.
  public export
  costIntro : {bs : Bindings} -> Cost bs -> Bindings
  costIntro (Mana _) = bs
  costIntro TapSymbol = bs
  costIntro UntapSymbol = bs
  costIntro (Do e) = effIntro e
  costIntro (Compound cs) = costsIntro cs

  ||| Is this component itself a compound?
  public export
  isCompound : {0 bs : Bindings} -> Cost bs -> Bool
  isCompound (Compound _) = True
  isCompound _ = False

  ||| A compound's ELEMENTS are components, never compounds — the same
  ||| one-meaning-one-spelling refusal `NotSeq` makes, and for the same
  ||| reason: nesting re-mints the right-nested tree the n-ary telescope
  ||| replaced, and core reaches the flat form by normalizing instead
  ||| (`Cost::normalize` splices a nested cost into the surrounding list).
  public export
  data NotCompound : Cost bs -> Type where
    MkNotCompound : {auto 0 ok : isCompound c = False} -> NotCompound c



  ||| Clauses. Constructor argument order IS textual order, and each
  ||| argument is typed in the context its predecessors built — the
  ||| telescope is the whole term, not a special clause-list feature.
  ||| Constructors are engine-basis members only (see the decision
  ||| record); keyword actions live in the macro layer below.
  public export
  data Effect : Bindings -> Type where
    -- "[src] deals [amt] damage to [to]" — the recipient is a player,
    -- the class word that names the damage class outright, or a
    -- damageable battlefield object ([CR#120.1,120.1a];
    -- `DamageRecipient`, asked of the noun — `badDamageGraveyardCard`,
    -- `badDamageToColor`, `badDamageArtifact`, `badDamageThis`), and
    -- the SOURCE is singular or distributive (`DamageSource`;
    -- `badGroupDamageSource`).
    -- spelling: ["<Param(0)> deals <Param(1)> damage to <Param(2)>"],
    -- kind: Sentence (matches constructors.ron's `DealDamage` entry exactly
    -- -- Reference/Count/Reference)
    DealDamage : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
                 (to : Noun (amtIntro amt) k) ->
                 {auto 0 ds : DamageSource src} ->
                 {auto 0 pm : PerMember to} ->
                 {auto 0 rk : DamageRecipient to} -> Effect bs
    -- "[a] fights [b]" ([CR#701.14a] -- only battlefield creatures fight
    -- [CR#701.14b]; `badFightGraveyard`, `badFightLand`). Primitive,
    -- confirmed: both damages dealt simultaneously, which no clause
    -- sequence reproduces (a sequence deals two ORDERED events -- see
    -- `karplusanYeti`), and no operative oracle text spells it out
    -- (reminder text only). No distinctness gate: a self-fight is defined
    -- ([CR#701.14c] -- twice its power to itself). Both slots are SINGULAR
    -- -- the plural form is the reciprocal frame "those creatures fight
    -- each other" (ledger) -- and participation reads the `combatant`
    -- grant, not a type name.
    -- spelling: ["<Param(0)> fights <Param(1)>"], kind: Sentence (the
    -- two-slot ACTION form; filter/Fight.ron studied for this pass only
    -- carries the bare-verb EventFilter twin "<Param(0)> fights", for
    -- "whenever ... fights" event matching -- no action-frame RON confirmed
    -- here, so this string is a draft, not verified against a real macro)
    Fights : (a : Noun bs Object) ->
             {auto 0 za : OnBattlefield (nounZone a)} ->
             {auto 0 ta : FightParticipant (nounTy a)} ->
             {auto 0 pa : nounPlur a = OneOf} ->
             (b : Noun (nomIntro a) Object) ->
             {auto 0 zb : OnBattlefield (nounZone b)} ->
             {auto 0 tb : FightParticipant (nounTy b)} ->
             {auto 0 pb : nounPlur b = OneOf} -> Effect bs
    -- "tap [n]" ([CR#701.26a] — tapping takes a battlefield object;
    -- `badTapGraveyard`) — core basis.
    -- spelling: ["tap <Param(0)>"], kind: Sentence
    Tap : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    -- "untap [n]" ([CR#701.26b] -- untapping, the tap row's paired
    -- inverse). The zonal demand is the tap row's own
    -- (`badUntapGraveyard`); that only a TAPPED permanent untaps
    -- ([CR#701.26a..701.26b]) is the resolving engine's fact, exactly as
    -- tap's own no-op case is -- the sentence is well-formed either way,
    -- so the grammar's demand stays zonal. The {T} and {Q} COST symbols
    -- are a different surface with an implicit self patient
    -- ([CR#107.5,107.6]) and stay their own `Cost` rows.
    -- spelling: ["untap <Param(0)>"], kind: Sentence
    Untap : (n : Noun bs Object) ->
            {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    -- "turn [n] face down" / "turn [n] face up" — the face transition
    -- REQUESTED, the third of [CR#110.5]'s four status categories to get
    -- a verb. One row over the face pair rather than two verbs, which is
    -- what the English says: the word is "turn" both ways and the value
    -- is the complement it takes, so the row is `BecomesStatus`' idiom
    -- moved to the clause side and the value spells itself. Both cells
    -- are written and neither needs a table — "Turn target creature face
    -- down." at 42 imperative lines, "Turn target face-down creature an
    -- opponent controls face up." (Break Open) and the ten "you may turn
    -- … face up" offers on the other side.
    -- [CR#708.2a] is the down cell's rule ("if a face-up permanent is
    -- turned face down … it becomes a 2/2 face-down creature with no
    -- text") and [CR#708.7] the up cell's; the characteristics a card
    -- lists instead of that default are its own second sentence and no
    -- part of this clause (Cyber Conversion's "It's a 2/2 Cyberman
    -- artifact creature").
    -- The gate is the tap row's and only the tap row's: a permanent is
    -- what turns ([CR#708.7] — "spells normally can't be turned face
    -- up"), so the demand is zonal. No precondition rides the value, and
    -- [CR#110.5] is why the grammar cannot carry one: status is the
    -- permanent's state at a moment of the game and not a fact about the
    -- phrase, so "a face-down permanent can't be turned face down"
    -- ([CR#708.2b] — "nothing happens") is the resolving engine's no-op,
    -- exactly as tapping an already-tapped permanent is.
    -- spelling: ["turn <Param(1)> <Param(0)>"], kind: Sentence (Param(0)
    -- is the status value's own word -- see StatusVal -- so the row
    -- writes "turn target creature face down"; the value's PRENOMINAL
    -- hyphenation, "face-down creature", belongs to HasStatus and not
    -- here)
    ToFace : (v : StatusVal FaceC) -> (n : Noun bs Object) ->
             {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    -- "[n] phases out" / "[n] phases in" — phasing REQUESTED, and the
    -- corpus's one structural surprise in this family: there is no
    -- imperative form at all. "Phase out target creature" is written zero
    -- times; every line is the intransitive declarative "Target creature
    -- phases out", the patient as SUBJECT, which is [CR#702.26a]'s own
    -- wording for the turn-based action ("all phased-in permanents with
    -- phasing that player controls 'phase out'"). So the spelling is
    -- subject-first — `DealDamage`'s and `Fights`' shape, not `Tap`'s —
    -- while the clause is still a resolving effect like any other.
    -- Value-indexed over [CR#110.5]'s phase pair for the reasons the face
    -- row is, and with no attestation table for the reason the face row
    -- has none: both cells are written. The two are lopsided and the
    -- ledger records it rather than the type — 60 phase-out lines against
    -- a single effect-position phase-in, The Pandorica's "When The
    -- Pandorica becomes untapped or leaves the battlefield, that permanent
    -- phases in", whose coordinated trigger event this vocabulary cannot
    -- write. Evidence without a bench positive, the posture `TwoNextSteps`
    -- already holds.
    -- The gate is `Tap`'s, in both directions, and phasing is the one
    -- family where that needs saying: a phased-out permanent has not left
    -- the battlefield ([CR#702.26d] — the event "doesn't actually cause a
    -- permanent to change zones"), it is only treated as though it does
    -- not exist ([CR#702.26b]), so the phase-in subject is a battlefield
    -- object and the same demand fits it. Attached objects phase out
    -- INDIRECTLY with what they are attached to ([CR#702.26g]) and no
    -- oracle line writes that as a clause — engine bookkeeping, ledgered.
    -- spelling: ["<Param(1)> phases <Param(0)>"], kind: Sentence (Param(0)
    -- is the PARTICLE and not the status word: "out" for PhasedOut, "in"
    -- for PhasedIn, where the description's word for the resulting state
    -- is "phased out" -- see StatusVal)
    Phases : (v : StatusVal PhaseC) -> (n : Noun bs Object) ->
             {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    -- "remove [n] from combat" ([CR#506.4] — the rule lists every way a
    -- permanent is removed from combat and this row is the one that is a
    -- SENTENCE, "an effect specifically removes it from combat"; the
    -- other seven ways are consequences the engine draws from zone
    -- changes, control changes, phasing and regeneration).
    -- The gate is `Tap`'s and only `Tap`'s. Being attacking or blocking
    -- is dynamic combat state, not a fact about the phrase — the same
    -- argument [CR#110.5] forces on the status verbs — and the corpus
    -- agrees at the surface: every operative line puts "attacking or
    -- blocking" in the target DESCRIPTION ("Remove target attacking or
    -- blocking creature from combat"), where `Attacking` and `Blocking`
    -- already live and where the phrase can be read.
    -- WHAT IT UNDOES is [CR#506.4]'s own list: "a creature that's removed
    -- from combat stops being an attacking, blocking, blocked, and/or
    -- unblocked creature". What it does NOT undo is a declaration that
    -- already happened, which is [CR#506.4a]'s separate paragraph and is
    -- the reason the prohibition family is not this row in disguise: a
    -- post-declaration "can't block" leaves the creature blocking. That
    -- distinction is semantic and no phrase refuses it, so it is recorded
    -- and not pinned.
    -- spelling: ["remove <Param(0)> from combat"], kind: Sentence
    RemoveFromCombat : (n : Noun bs Object) ->
                       {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    -- "[who] becomes the monarch" / "[who] takes the initiative" -- a
    -- player GAINING a designation, and ONE row over the two because
    -- [CR#725.1] and [CR#726.1] say the same sentence twice ("the
    -- monarch/the initiative is a designation a player can have", gained
    -- when "an effect instructs a player to become the monarch" / "to take
    -- the initiative"). The VERB differs per value and the spelling
    -- carries it, which is the arrangement `Happened`'s past-verb table
    -- already uses.
    -- The SUBJECT RANGE is asymmetric and left ungated: the monarch is
    -- written with a third party 10 times against 25 self lines ("target
    -- opponent becomes the monarch"), while all 9 initiative lines are the
    -- self. Tolerated over-generation, the standing posture for a subject
    -- range, and recorded here rather than gated because gating would have
    -- required the row to inspect which noun it was handed.
    -- spelling: ["<Param(0)> become(s) the monarch" (Monarch),
    -- "<Param(0)> take(s) the initiative" (TheInitiative)], kind: Sentence
    -- (the verb is the designation's own -- see PlayerDesignation)
    TakesDesignation : (who : Noun bs Player) -> (d : PlayerDesignation) ->
                       Effect bs
    -- "goad [n]" (24 lines) -- [CR#701.15a]'s keyword action, whose EFFECT
    -- is the designation [CR#701.15b] defines. The verb is card text and
    -- the designation's own rules text is not: "attacks each combat if
    -- able and attacks a player other than the controller of the permanent
    -- … that caused it to be goaded" lives in [CR#701.15b] and in reminder
    -- text (33 lines by this round's closest measure), never as a printed
    -- operative clause, so the requirement row minted in chapter forty-six
    -- neither spells it nor needs to.
    -- spelling: ["goad <Param(0)>"], kind: Sentence
    Goad : (n : Noun bs Object) ->
           {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
    -- "it becomes [day/night]" -- the GAME gaining a designation
    -- ([CR#731.1] gives the phrase in its own words: "'it becomes day' and
    -- 'it becomes night' refer to the game gaining the day or night
    -- designation"). No subject slot, for `ItIsNow`'s reason.
    -- UNGATED where the check is gated: both directions are written, which
    -- is exactly why `timeCheckOk` is a fact about the check alone.
    -- spelling: ["it becomes <Param(0)>"], kind: Sentence
    BecomesTime : (t : TimeOfDay) -> Effect bs
    -- "[who] win(s)/lose(s) the game" -- the outcome an effect STATES
    -- ([CR#104.2b,104.3e]), 107 corpus occurrences across three frames:
    -- 87 imperative (the ability's own main clause, bare or if-gated --
    -- Battle of Wits' alternate win, the Pact family's fail-to-pay, the
    -- extra-turn tax), 17 triggered payloads over a third party bound by
    -- the same ability's trigger ("Whenever Phage deals combat damage to a
    -- player, that player loses the game"), and 3 direct third-party lines
    -- with no trigger at all (Door to Nothingness).
    -- The PATIENT is an ordinary player noun and the corpus ranges over
    -- the whole layer: you, that player, target player, enchanted player,
    -- each opponent, its owner, they, the chosen player. No vocabulary of
    -- its own is needed, which is the measurement this round came to make.
    -- It is immediate on resolution and suppressed by a matching
    -- `OutcomeGate` -- precedence, not consumption ([CR#101.2]) -- and a
    -- player who would win and lose at once loses ([CR#104.3f]). Both are
    -- the engine's arithmetic over this clause and no part of the phrase.
    -- spelling: ["<Param(1)> <Param(0)>"] (the verb phrase inflected for
    -- its subject -- see OutcomeVerb), kind: Sentence
    Concludes : (v : OutcomeVerb) -> (who : Noun bs Player) -> Effect bs
    -- "the game is a draw" ([CR#104.4c] -- "an effect may state that the
    -- game is a draw"), the third of the trio [CR#104] lets an effect
    -- state and the only one with no patient at all: a draw is the GAME's
    -- outcome, not a player's, which is why the row is nullary where its
    -- two siblings take a noun. `BecomesTime`'s shape one row up, and for
    -- the same reason -- there is one game and the sentence names it
    -- without a subject phrase.
    -- Two corpus lines, and neither is writable whole: Divine
    -- Intervention's trigger is an AGENTIVE removal ("when you remove the
    -- last intervention counter") over a counter kind this catalog does
    -- not carry, and Celestial Convergence's is the comparative victor.
    -- The row lands on the rule's own sentence with the bench gap
    -- recorded, which is `TwoNextSteps`' posture.
    -- spelling: ["the game is a draw"], kind: Sentence
    GameDrawn : Effect bs
    -- "Choose [n]." — the choice clause as surface for the mention it
    -- announces ([CR#601.2c] for targets; [CR#608.2d] otherwise). A
    -- recorded DIVERGENCE from core, whose choose binders are
    -- resolution-time and nontarget ([CR#115.1] keeps the words
    -- apart): here the fronted sentence scopes everything after it.
    -- spelling: ["choose <Param(0)>"], kind: Sentence
    Choose : {k : Kind} -> (n : Noun bs k) ->
             {auto 0 ch : Choosable n} -> Effect bs
    -- "[move] [n] [to zone]" — the zone-change primitive every keyword
    -- action's body bottoms out in ([CR#701.8a] shape). Destination
    -- only: the from-zone is the referent's fold-state, which this
    -- clause UPDATES (the retag). Owned destinations are owner-routed
    -- ([CR#400.3] — a card never enters another player's hand;
    -- `badMoveToTargetsHand`), so only the bare forms are writable
    -- until an owner-destination positive lands (Unsummon's "its
    -- owner's hand" waits with the owned-zone work).
    -- spelling: (construction-owned -- the bare zone-change primitive; no
    -- English word of its own. Spelled only through its wrapping verb tag:
    -- Composite Destroy/Sacrifice/Exile or Does _ Discard, e.g. Destroy's
    -- own "destroy <Param(0)>" per action/Destroy.ron)
    -- The ORDER rider rides the destination and not this verb, because
    -- English writes it there ("on the bottom of your library in a random
    -- order" is one adverbial phrase after another on the same placement);
    -- what the verb owns is the agreement between the rider and its
    -- patient's number (`ArrangementOk`, [CR#401.4]). Core splits the same
    -- fact into a second verb (`MoveGroup { group, arrangement, to }`)
    -- because its group term is a different sort; here plurality is a
    -- property of the one patient slot, so no second row is needed.
    -- The RIDERS are a defaulted slot for the reason the activated line's
    -- three restrictions are: every placement can carry them and almost
    -- none writes one. Gated to the battlefield (`RidersFit`), and the
    -- whole vocabulary is `MoveRiders`.
    -- The destination also asks about the PATIENT and not only about its
    -- own phrase: a battlefield placement takes a permanent card
    -- ([CR#110.4,110.4a]), so "put target instant card from a graveyard
    -- onto the battlefield" is unwritable (`Placeable`,
    -- `badInstantOntoBattlefield`). It reads the projected head type, so a
    -- phrase that writes no type word still places -- Oblivion Ring's
    -- "return the exiled card to the battlefield" names a card and not a
    -- type.
    -- And the patient is an OBJECT and not [CR#115.4]'s damage class:
    -- "any target" spans players, so nothing places it anywhere
    -- (`NotAnyTarget`, `badExileAnyTarget`). The zone demand cannot make
    -- that refusal here, this verb having none -- the placement is what
    -- SETS the zone.
    Move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
           {default (MkMoveRiders [] Nothing) riders : MoveRiders (nomIntro what)} ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur what) to} ->
           {auto 0 na : NotAnyTarget what} ->
           {auto 0 pl : Placeable (nounTy what) (zoneSort to)} ->
           {auto 0 rf : RidersFit riders (zoneSort to)} -> Effect bs
    -- "counter [what]" ([CR#701.6a] -- "to counter a spell or ability
    -- means to cancel it, removing it from the stack. It doesn't resolve
    -- and none of its effects occur"). The verb the stack carrier exists
    -- for. One noun slot and no zone destination, which is the difference
    -- from every other removal verb here. [CR#701.6a] does say where a
    -- countered spell goes -- "put into its owner's graveyard" -- but the
    -- sentence does not, and a `Move` the sentence never writes is the
    -- engine's fact and not this grammar's (`Composite`'s three tags each
    -- name a move oracle SPELLS; this verb names none).
    -- The complement is a SPELL and the demand says so by zone
    -- ([CR#112.1]) -- "counter target creature" is unwritable, the
    -- battlefield permanent having already resolved
    -- (`badCounterPermanent`). The ABILITY half of [CR#701.6a] is real
    -- English ("counter target activated or triggered ability") and is
    -- ledgered: an ability on the stack is an OBJECT [CR#109.1] this noun
    -- vocabulary has no word for.
    -- spelling: ["counter <Param(0)>"], kind: Sentence (the imperative
    -- leaves the agent unpronounced, as destroy and exile do)
    -- The demand is STRICT and not a `zoneFits` silence, which is the
    -- one place this verb parts from the file's other zone gates: what a
    -- countering needs is EVIDENCE that the phrase names a spell, and a
    -- phrase that projects no zone offers none ([CR#112.1]; `OnStack`,
    -- `badCounterAnyTarget`).
    CounterSpell : (what : Noun bs Object) ->
                   {auto 0 zn : OnStack (nounZone what)} -> Effect bs
    -- "[who] gains/loses [amt] life" ([CR#119.3]) — core basis (merged).
    -- spelling: ["<Param(0)> gains <Param(1)> life", "<Param(0)> loses
    -- <Param(1)> life"] (selects on the embedded LifeOp, Up/Down; mirrors
    -- constructors.ron's `GainLife` entry / the merged ChangeLife family),
    -- kind: Sentence
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Effect bs
    -- "[who] draw(s) [amt] card(s)" ([CR#121.1] -- a player draws by
    -- putting the top card of their library into their hand). Not a
    -- keyword action: [CR#701]'s list does not contain it, drawing being
    -- its own game action, so the clause is a row of its own beside
    -- `ChangeLife` rather than a `Composite` tag -- and it carries its
    -- SUBJECT the same way and for the same reason, both spellings being
    -- ordinary oracle ("Draw a card." with the imperative's unpronounced
    -- `You`; "Target player draws a card"; "Each player draws a card").
    -- The COUNT is the ordinary magnitude vocabulary and no parallel
    -- number path: a `Lit`, an `XVal`, a for-each amount, or a read.
    -- It introduces NOTHING but its subject, and that is measured rather
    -- than assumed: not one corpus line reads a drawn card back across a
    -- sentence boundary -- "Draw a card." followed by "it"/"that card" is
    -- written zero times, and the near misses are reminder parentheticals
    -- and Fblthp, whose "if it entered from your library" reads the
    -- CREATURE that entered. The drawn card IS read inside the
    -- coordination that reveals it ("Draw a card and reveal it. If it
    -- isn't a land card, discard it."), which is a verb-phrase
    -- coordination this grammar does not spell; that construction is what
    -- would reopen the question (ledger).
    -- spelling: ["<Param(0)> draw(s) <Param(1)> card(s)"], kind: Sentence
    -- (the imperative leaves the agent unpronounced; the noun "card"
    -- pluralises with the count, and at one the numeral is the article
    -- "a". A count that is READ rather than written extraposes -- "draw
    -- cards equal to <Param(1)>", the bare plural before the phrase --
    -- which is a linearization choice, unchecked here)
    Draw : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
           {auto 0 wc : WrittenCount amt} -> Effect bs
    -- "[who] look(s) at / reveal(s) [what]" -- the EXPOSURE clause
    -- ([CR#701.20a,701.20e]). One clause for both verbs because they are
    -- one operation with two audiences (see `ExposeVerb`), and its subject
    -- is a slot for the reason `Draw`'s is: both spellings are ordinary
    -- oracle ("Look at the top four cards of your library"; "Target
    -- opponent reveals their hand").
    -- Exposing MOVES nothing ([CR#701.20b]) -- the looked-at cards are
    -- still in the library, which is why the clause's own retag is absent
    -- and why the next sentence's placement has somewhere to move them
    -- FROM. What it contributes is the point of the whole chapter: a look
    -- or a reveal over a positioned slice ASSEMBLES a group the following
    -- sentences may reach into, which is the structural difference between
    -- "the rest" and every complement chapter twenty-two refused (finding
    -- 127 -- two announcements are two mentions and never a pair; one
    -- slice phrase is one group).
    -- spelling: ["<Param(1)> look(s) at <Param(2)>", "<Param(1)>
    -- reveal(s) <Param(2)>"] (selects on Param(0), the ExposeVerb; the
    -- imperative leaves the agent unpronounced), kind: Sentence
    Expose : (v : ExposeVerb) -> (who : Noun bs Player) ->
             (what : Exposed (nomIntro who)) -> Effect bs
    -- "[who] mill(s) [amt] card(s)" -- [CR#701.17a]: "that player puts
    -- that many cards from the top of their library into their graveyard".
    -- A row of its own beside `Draw` and for the same reasons: [CR#701]
    -- names it a keyword action but `VerbName`'s tags are the ones
    -- `Composite` spells over a `Move` body, and a mill is a move of a
    -- SLICE the sentence never names, so there is no patient phrase to
    -- wrap. It carries its subject like `Draw`.
    -- Unlike `Draw` it INTRODUCES its group, and the contrast is the
    -- rules' own rather than a corpus accident: [CR#701.17c] lets an
    -- effect that refers to a milled card find it "as long as that zone is
    -- a public zone", which a graveyard is ([CR#400.2]), while a draw puts
    -- its card in a hand, which is not -- so the same [CR#400.7j] that
    -- licenses the mill read denies the draw one, and the corpus agrees
    -- exactly. The READ FORMS are the among-restriction ("from among
    -- them") and the "this way" participle ("milled this way"), neither of
    -- which this vocabulary spells yet (ledger); the mention is honest
    -- before its readers arrive.
    -- spelling: ["<Param(0)> mill(s) <Param(1)> card(s)"], kind: Sentence
    -- (the imperative leaves the agent unpronounced; "card" pluralises
    -- with the count and at one the numeral is the article "a")
    Mill : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
           {auto 0 wc : WrittenCount amt} -> Effect bs
    -- "search [zone] for [description]" ([CR#701.23a] -- "look at all
    -- cards in that zone (even if it's a hidden zone) and find a card that
    -- matches the given description"). The rule gives the clause its three
    -- parts and this row is those three: who searches, which zone, and the
    -- description found.
    -- The description is a PREDICATE and not a noun phrase, because the
    -- zone is on the verb rather than in the phrase: written as a noun, "a
    -- creature card" would seed the battlefield ([CR#109.2]) and the
    -- clause would have to overwrite it. The found object's mention is the
    -- clause's own, placed in the zone it was found in -- which is what
    -- the following "it"/"that card" reads (Sylvan Scrying).
    -- [CR#701.23e] is why the reveal is a separate clause and not a rider
    -- here: "if the effect that contains the search instruction doesn't
    -- also contain instructions to reveal the found card(s), then they're
    -- not revealed" -- the exposure is written or it does not happen.
    -- COUNTED finds ("search your library for up to two basic land cards")
    -- and the type-free "a card" wait on counted untargeted groups and a
    -- card-headed predicate respectively (ledger).
    -- spelling: ["<Param(0)> search(es) <Param(1)> for <Param(2)>"] (the
    -- imperative leaves the agent unpronounced; the description takes the
    -- indefinite article and the zone's own "card" word), kind: Sentence
    -- WHICH zone phrase is two demands and not one, and chapter
    -- twenty-six is where they came apart: the SORT must be a zone a
    -- search may look through (`SearchableZone`), and the PHRASE must
    -- name that zone whole (`WholeZone`). Reading only the sort let
    -- "search the top of your library" through, arrangement rider and
    -- all, because `zoneSort` projects `Library` off a position exactly
    -- as it does off the pile ([CR#701.23a] looks at "all cards in that
    -- zone"; `badSearchLibraryPosition`).
    Search : (who : Noun bs Player) -> (z : ZoneExpr (nomIntro who)) ->
             (p : Predicate (nomIntro who) Object) ->
             {auto 0 sz : SearchableZone (zoneSort z)} ->
             {auto 0 wz : WholeZone z} ->
             {auto 0 hd : Headed p} ->
             {auto 0 af : AnyTargetFree p} ->
             {auto 0 zf : ZoneFree p} -> Effect bs
    -- "[whose] shuffle(s) [their] library" ([CR#701.24a] -- "randomize the
    -- cards within it so that no player knows their order"). The patient
    -- is a library and the slot names WHOSE, which is core's Law-3 reading
    -- of the same rule (`Shuffle(Selection)` over `LibraryOf(Reference)`,
    -- "the collection patient, not an agent"). English elides the object
    -- almost always -- "Then shuffle." dominates -- which is
    -- linearization's business, the searched library being the only one in
    -- scope.
    -- It DESTROYS discourse, and that is the row's one interesting fact:
    -- [CR#701.20d] says revealed cards that are reordered "stop being
    -- revealed and become new objects", and [CR#701.24a] leaves no player
    -- knowing the order, so a mention still sitting in the shuffled
    -- library cannot be referred to afterwards. `effIntro` drops exactly
    -- those (`badReadAfterShuffle`); a card the sentence already moved OUT
    -- is untouched, which is why every search writes its placement before
    -- its shuffle ([CR#701.24b] keeps the found cards out of the shuffle
    -- for the same reason).
    -- spelling: ["<Param(0)> shuffle(s) <Param(0)>'s library"] (the
    -- imperative leaves both the agent and the object unpronounced --
    -- "shuffle"), kind: Sentence
    Shuffle : (whose : Noun bs Player) -> Effect bs
    -- "[static effect] [duration]" -- the clause whose resolution
    -- establishes a continuous effect for the span it states ([CR#611.2a]
    -- -- it "lasts as long as stated"), which is core's
    -- `Continuously { effect, duration }` exactly
    -- (`deckmaste_core/src/effect.rs`). One envelope for every one of the
    -- constructions, because the adverbial is one slot: what varies is
    -- which static effect precedes it, and WHICH adverbials that
    -- construction writes is the corpus's answer, not the writer's
    -- (`SpanOk`, over `spanUse` and `absentOk`). The unwritten span is the
    -- [CR#611.2a] end-of-game default, spelled as an explicit `Nothing`
    -- per the no-defaults convention and legal only where its construction
    -- has an answer for it.
    -- spelling: ["<Param(0)> <Param(1)>"], kind: Sentence (Param(0) = the
    -- static effect's own clause, Param(1) = the trailing duration
    -- adverbial, omitted entirely when Nothing; mirrors core's
    -- Continuously struct field-for-field)
    Continuously : (se : StaticEffect bs) -> (span : Maybe (Duration (staticIntro se))) ->
                   {auto 0 nr : NotLetterRider se} ->
                   {auto 0 sp : SpanOk (staticKind se) span} -> Effect bs
    -- "[agent] create(s) [count] [characteristics] token(s) [arrival]"
    -- ([CR#111.1] -- a token represents a permanent no card represents;
    -- [CR#111.3] -- the creating effect defines its characteristics).
    -- Core's `Create { agent, count, token, riders }` field for field
    -- (`deckmaste_core/src/action.rs`), and the agent is an explicit slot
    -- there for a reason this grammar shares: "Its controller creates a
    -- 2/2 green Boar creature token" and "target opponent creates a tapped
    -- Treasure token" are real lines, so the creator is information, not
    -- always the imperative's unpronounced `You`.
    -- The COUNT is the ordinary magnitude vocabulary and no parallel
    -- number path -- a `Lit`, or the for-each amount (Elven Ambush) -- and
    -- core's slot is its own `Count` likewise. That count is also where
    -- the mention's grammatical number comes from (`amtPlur`).
    -- VERIFIED rather than assumed: the create WORDING is the only one
    -- current oracle uses. The older "put a … token onto the battlefield"
    -- survives on ZERO lines (the two matches are a "nontoken permanent"
    -- restriction and a Lander token's own quoted land-search ability).
    -- Token COPIES ("a token that's a copy of …", core's
    -- `TokenSpec::Copy`) and the PREDEFINED names ([CR#111.10], core's
    -- `TokenSpec::Named`) are the two other token positions and neither is
    -- here: copy machinery is its own axis and the predefined catalog its
    -- own registry (ledger).
    -- spelling: ["<Param(0)> create(s) <Param(1)> <Param(2)> token(s)
    -- <Param(3)>"], kind: Sentence (Param(2) = the characteristics in the
    -- fixed adjective order, see TokenChars; Param(3) = the arrival relative
    -- clause, omitted when empty, see TokenRider; the imperative leaves the
    -- agent unpronounced. Mirrors core's Create struct field for field)
    Create : (agent : Noun bs Player) -> (count : Amount (nomIntro agent)) ->
             (tok : TokenChars (amtIntro count)) -> (riders : List TokenRider) ->
             {auto 0 tt : TokenTyped tok} ->
             {auto 0 tp : TokenPt tok} ->
             {auto 0 sf : SubtypesFit tok} ->
             {auto 0 tc : TokenCanonical tok} ->
             {auto 0 wc : WrittenCount count} ->
             {auto 0 rr : RidersOk riders} -> Effect bs
    -- "Put [amt] [kind] counter(s) on [n]" ([CR#122.1]) -- core's
    -- `PutCounters(Reference, CounterRef, Count)`, with the arguments in
    -- TEXTUAL order as every clause here has them (core writes the
    -- reference first; the axes are the same three). AGENT-SILENT, and
    -- core says so in the same words: `PutCounters` is on its
    -- agent-carrying-none list, and the corpus agrees -- no line writes
    -- "[player] puts a +1/+1 counter on", the verb being the effect's own
    -- imperative.
    -- The recipient is an OBJECT on the battlefield. [CR#122.1] puts
    -- counters on players too, but English does not put them there with
    -- this verb: "put a … counter on [a player]" is written zero times and
    -- the player form takes a different verb entirely ("target player gets
    -- a poison counter"), so it waits on that verb (ledger). The ZONE
    -- demand is `CounterHolder`'s two-zone table and not the blanket
    -- battlefield one the other verbs carry: the graveyard is a real
    -- silence (`badPutCountersGraveyard`), exile is not (Jhoira of the
    -- Ghitu).
    -- spelling: ["put <Param(0)> <Param(1)> counter(s) on <Param(2)>"],
    -- kind: Sentence (Param(1) = CounterKind's own word; the noun "counter"
    -- pluralises with the count, and at one the numeral is the article "a")
    -- The KIND is scope-gated as of the round that gave the catalog its
    -- player half: this verb takes an object's counter and nothing else
    -- ([CR#122.1]'s two scopes, `counterScope`), so "put a poison counter
    -- on target creature" is refused by the kind rather than by the
    -- recipient (`badPutPoisonOnCreature`). The ledgered note above is
    -- discharged: the player form's verb now exists (`GetsCounters`).
    PutCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                  (on : Noun (amtIntro amt) Object) ->
                  {auto 0 wc : WrittenCount amt} ->
                  {auto 0 pm : PerMember on} ->
                  {auto 0 sc : counterScope kind = Object} ->
                  {auto 0 zn : CounterHolder (nounZone on)} -> Effect bs
    -- The DIVISION ([CR#601.2d]) -- one written amount split over the
    -- members of one group mention, the split announced as the spell is
    -- cast rather than chosen on resolution. That announcement is the
    -- whole reason the division is a structure and not an adverb: the
    -- targets and their shares are fixed together at [CR#601.2d], and
    -- [CR#115.7f] then holds the shares still even when the targets
    -- themselves are changed later ("the original division can't be
    -- changed"), which is a fact about a division as an object. The
    -- companion rule [CR#608.2d] runs the same split at RESOLUTION for
    -- untargeted recipients and is the same structure read at a different
    -- time; the group mention is what differs, so no second row is needed.
    -- The floor [CR#601.2d] states -- "each of these targets must receive
    -- at least one of whatever is being divided" -- is NOT typed here. It
    -- is a legality question about the announced numbers, and it goes to
    -- the legality layer beside [CR#701.14b]'s both-or-neither fight guard
    -- and [CR#701.12a]'s all-or-nothing exchange rule.
    -- The recipient is a GROUP MENTION and PLURAL, which is the division's
    -- own demand rather than a borrowed one: splitting needs members to
    -- split among, and every corpus line writes a counted target group or
    -- a plural read after "among" (`badDivideAmongSingular`,
    -- `badDivideAmongDescription`).
    -- spelling: [(text: "<Param(0).src> deals <Param(1)> damage divided as
    -- you choose among <Param(2)>", when: [(0, "DividedDamage")]),
    -- (text: "distribute <Param(1)> <Param(0).kind> counters among
    -- <Param(2)>", when: [(0, "DistributedCounters")])], kind: Sentence
    -- (core's `OneShotEffect::Distribute` narrowed to its two attested
    -- bodies -- see DividedVerb)
    Distribute : {k : Kind} -> (v : DividedVerb bs) ->
                 (amt : Amount (divIntro v)) ->
                 (among : Noun (amtIntro amt) k) ->
                 {auto 0 wc : WrittenCount amt} ->
                 {auto 0 pl : nounPlur among = ManyOf} ->
                 {auto 0 gm : GroupMention among} ->
                 {auto 0 tk : DividedTakes (divTag v) among} -> Effect bs
    -- "Remove [amt] [kind] counter(s) from [n]" -- the twin, and core's
    -- `RemoveCounters` beside `PutCounters` for the same reason: removal
    -- is not a negative put (it is cost-eligible where a put is not, and
    -- it can fail for want of counters), so the two are separate verbs
    -- there and separate rows here.
    -- What this row does NOT spell is the quantifier "all" ("Remove all
    -- counters from target creature") and the kind-blind "a counter",
    -- which are core's own `CounterSpec::AllKinds` and a bare-count
    -- reading; both want a quantity over KINDS that the written amount
    -- vocabulary has no term for (ledger).
    -- The zone demand is `PutCounters`' widened one, and it is the same
    -- table rather than a looser twin: [CR#702.62a]'s own second ability
    -- takes a counter OFF a card in exile, and the corpus writes it as
    -- main text too (Alaundo the Seer, All Hallow's Eve).
    -- spelling: ["remove <Param(0)> <Param(1)> counter(s) from <Param(2)>"],
    -- kind: Sentence (as PutCounters, with the preposition "from")
    RemoveCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                     (from : Noun (amtIntro amt) Object) ->
                     {auto 0 wc : WrittenCount amt} ->
                     {auto 0 sc : counterScope kind = Object} ->
                     {auto 0 zn : CounterHolder (nounZone from)} -> Effect bs
    -- "[who] get(s) [amt] [kind] counter(s)" -- the PLAYER's put verb, and
    -- a different WORD rather than a widened `PutCounters`: the corpus
    -- separates the two absolutely, objects never "getting" a counter and
    -- players never receiving "put", zero lines each way. So the scope
    -- table gates both verbs and neither has to describe its own
    -- recipients twice (`badGetsBoostCounter`).
    -- AGENT-SILENT like its object twin, and for a sharper reason: the
    -- recipient is the SUBJECT here, so there is no room for an agent
    -- phrase at all. [CR#702.90b] is where the rules and the oracle part
    -- company on the word -- infect "causes that source's controller to
    -- GIVE the player that many poison counters" -- and this grammar
    -- records what cards print, which is "gets" without exception.
    -- The count is WRITTEN, `PutCounters`' demand, and at one the numeral
    -- is the article ("a poison counter"); the recipient is an ordinary
    -- player phrase and needs no vocabulary of its own -- "you", "target
    -- player", "target opponent", "each player", "each opponent", and the
    -- anaphoric "that player" are all existing rows. The one recipient
    -- this round does NOT reach is "defending player" (2 lines), which
    -- wants a combat-role player predicate the grammar has no word for;
    -- recorded, not minted, the corpus discipline's own bar.
    -- spelling: ["<Param(0)> get(s) <Param(1)> <Param(2)> counter(s)"],
    -- kind: Sentence (Param(2) = CounterKind's own word; the noun
    -- "counter" pluralises with the count and at one the numeral is the
    -- article "a", exactly as PutCounters spells it)
    GetsCounters : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
                   (kind : CounterKind) ->
                   {auto 0 wc : WrittenCount amt} ->
                   {auto 0 sc : counterScope kind = Player} -> Effect bs
    -- "[who] loses all [kind] counters" / "[who] loses all counters" --
    -- the player's REMOVAL verb, and the one place a kind-blind reading
    -- is IN this vocabulary rather than ledgered with the quantifiers.
    -- The reason is that this verb is always-all: every one of the four
    -- corpus lines removes the whole holding and none removes a number,
    -- so the `Maybe` is not a quantity term in disguise -- it says
    -- whether the sentence NAMES a kind, not how many of one it takes.
    -- The `Nothing` cell is "Each opponent loses all counters" (Final Act)
    -- and "Target opponent loses all counters" (Suncleanser); the `Just`
    -- cell is "Target player loses all rad counters" (RadAway) and
    -- "Target player loses all poison counters" (Leeches).
    -- `CtrlSingular`'s and `BlockPartner`'s shape at a third site: the
    -- scope gate rides the written kind and asks nothing of the unwritten
    -- one (`PlayerCounterKind`).
    -- spelling: ["<Param(0)> lose(s) all <Param(1)> counters" (a kind
    -- named), "<Param(0)> lose(s) all counters" (none)], kind: Sentence
    -- (Param(1) = CounterKind's own word; "counters" is plural in both
    -- cells, the quantifier being "all")
    LosesAllCounters : (who : Noun bs Player) -> (kind : Maybe CounterKind) ->
                       {auto 0 pk : PlayerCounterKind kind} -> Effect bs
    -- the keyword-action tag ([CR#701]): the named verb deontics and
    -- replacements key on, wrapping its expansion body ([CR#701.8b] —
    -- only a Destroy-tagged move IS a destruction). The tag and body
    -- must agree (`TagBody` — a Destroy-tagged exile would cant the
    -- wrong verb [CR#702.12b]; `badDestroyTaggedExile`), and a tagged
    -- move also stamps its referent's provenance — the participle
    -- read's filter (finding 26).
    -- spelling: (construction-owned -- the keyword-action tag wrapper;
    -- spelled by its VerbName tag's own frame, e.g. Composite Destroy _
    -- = "destroy <Param(0)>" per action/Destroy.ron exactly), kind: Sentence
    Composite : (v : VerbName) -> (e : Effect bs) ->
                {auto 0 ok : TagBody v e} -> {auto 0 na : NonAgentive v} -> Effect bs
    -- "[subject] [verb phrase]" -- the declarative clause: the verb's
    -- performer in subject position, its phrase typed after it. Verbs the
    -- CR gives a player actor ([CR#701.21a,701.9a]-family) REQUIRE one --
    -- they have no `NonAgentive` row, so `Composite` refuses them
    -- (`badAgentlessSacrifice`) -- and their imperative supplies the
    -- unpronounced subject as an explicit `You`. Effect-verbs take a
    -- subject OPTIONALLY: oracle writes destroy and exile both ways ("You
    -- destroy four lands you control, then target opponent destroys four
    -- lands they control." -- Burning of Xinye). This is core's per-verb
    -- `who` slot factored to clause position -- a dependent context can't
    -- re-use the subject term at each inner slot the way the real macros
    -- ride their agent param -- and lowering redistributes it. Object
    -- sources (DealDamage's src) are the verb's own argument, not a
    -- subject. The clause carries its verb TAG directly, and the tag's
    -- body obligations ride `TagBody` here exactly as under `Composite`.
    -- Overgeneration accepted: a subject with no choice of its own ("You
    -- destroy target creature") is spellable, though oracle style writes
    -- the bare imperative there.
    -- spelling: (construction-owned -- subject + verb-tag clause, e.g.
    -- "<Param(0)> sacrifices <Param(2)>"/"<Param(0)> discards <Param(2)>";
    -- the verb's own lemma is VerbName's, conjugation is auto-inflection --
    -- see the sacrifice/discards macros in Experimental.Macros), kind:
    -- Sentence
    Does : (subj : Noun bs Player) -> (v : VerbName) ->
           (e : Effect (nomIntro subj)) ->
           {auto 0 tb : TagBody v e} -> Effect bs
    -- "[who] pay(s) [cost]" -- the payment as a CLAUSE, which is what the
    -- unless family needs and the only reason this row exists:
    -- [CR#118.12a] rewrites "[Do something] unless [a player does
    -- something else]" into "[A player may do something else]. If [that
    -- player doesn't], [do something]", so the may's BODY has to be able
    -- to say "pays {3}". The complement is a whole `Cost` and not a mana
    -- amount, which is measured: most of those complements are one symbol
    -- run, some are "N life", and the rest are the cost words this
    -- vocabulary does not spell (echo, upkeep, "its mana cost" -- ledger).
    -- The clause is NOT the cost type wearing a verb: a cost is paid to
    -- activate ([CR#602.1a]) where this is a sentence that resolves, and
    -- the same `Cost` value stands in both places exactly as core's
    -- `Action::Pay(Cost)` does.
    -- spelling: ["<Param(0)> pay(s) <Param(1)>"], kind: Sentence (the verb
    -- inflects with the subject -- "you pay {3}" against "its controller
    -- pays {3}"; a life component under this verb writes "3 life", the
    -- shared verb swallowing the component's own "Pay")
    Pay : (who : Noun bs Player) -> (c : Cost (nomIntro who)) ->
          {auto 0 pb : Payable c} ->
          {auto 0 ag : PayAgrees who c} -> Effect bs
    -- "[decider] may [effect]" -- the decider slot ([CR#608.2d]; the
    -- resolving default is the controller [CR#608.2c]). Decider and
    -- performer can differ, so the body is any clause.
    -- The two BRANCHES are the anaphoric "if you do" / "if you don't",
    -- and they are fields rather than `Condition` rows (core's `May { who,
    -- effect, if_did, if_not }`): the anaphor has no referent to condition
    -- ON, asking whether the preceding OPTIONAL ACTION was taken, which is
    -- neither a fact about the board nor a mention in the discourse.
    -- The arms are typed differently, and that asymmetry is the finding:
    -- `ifDid` runs only when the body ran, so it reads everything the body
    -- introduced; `ifNot` runs only when it did NOT, so the body's
    -- mentions never existed and it reads only what preceded the may
    -- (`badIfNotReadsMayBody`).
    -- What the branch reads is settled by rule: [CR#118.12] makes the
    -- offered action a COST paid on resolution and has the "if [a player]
    -- does" clause check "whether the player chose to pay an optional cost
    -- or started to pay a mandatory cost, regardless of what events
    -- actually occurred" -- not an event read at all, which is why the
    -- declined arm has nothing to mention.
    -- The MANDATORY twin ("[Do something]. If you do, …") is THIS NODE
    -- with the offer emptied, [CR#118.12] stating both shapes in one
    -- sentence and giving them one reader; `Nothing` is the bare
    -- instruction, whose "if you do" takes its pronoun from the BODY's own
    -- agent.
    -- spelling: ["<Param(0)> may <Param(1)>", "<Param(0)> may <Param(1)>.
    -- If <Param(0)> do, <Param(2)>.", "<Param(0)> may <Param(1)>. If
    -- <Param(0)> don't, <Param(3)>."] (the anaphor's pronoun and its verb
    -- agreement are the DECIDER's own -- "if you do" against Risk Factor's
    -- "if they don't" -- so auto-inflection supplies both from Param(0);
    -- mirrors core's May struct field-for-field. With Param(0) absent the
    -- offer is unwritten and the sentence is the bare body -- "<Param(1)>.
    -- If <agent of Param(1)> do, <Param(2)>." -- which is the [CR#118.12]
    -- mandatory shape), kind: Sentence
    May : (offer : Maybe (Noun bs Player)) -> (body : Effect (mayCtx offer)) ->
          (ifDid : Maybe (Effect (effIntro body))) ->
          (ifNot : Maybe (Effect (mayCtx offer))) -> Effect bs
    -- "[clause] if [condition]" -- the ordinary conditional, whose "if"
    -- has "only its normal English meaning" ([CR#603.4]).
    -- Argument order is TEXTUAL order and the binding flow is why: the
    -- trailing conditional's condition reads the clause's own mentions
    -- ("Destroy target artifact if its mana value is 2 or less" --
    -- Overload), so it is typed in the clause's post-context like every
    -- other trailing argument. The LEADING linearization is this same node
    -- whenever the condition uses nothing the clause introduced; the
    -- leading form that DOES read back is the target-announcing family and
    -- is refused (`condDelta`, `badMatchesTargetSubject`).
    -- The condition is WRITTEN after the clause and EVALUATED before it,
    -- so it is typed in `preIntro`: what the clause's phrases ANNOUNCED,
    -- with the announced zone, and nothing the clause did. Typing it in
    -- the post-state let "Destroy target creature if it's in a graveyard"
    -- through (`badTrailingPostStateZone`).
    -- The conditioned clause is a HOLE outward (`effIntro`): the condition
    -- may be false, and then this clause never ran and its phrase named
    -- nothing. The ELSE arm ("Otherwise, …") is a field rather than a
    -- `Sequentially` element, having no meaning without its "if", and it
    -- is `May`'s `ifNot` exactly -- typed in `bs`, the discourse BEFORE
    -- the conditional (`badOtherwiseReadsIfArm`), and a hole outward too.
    -- spelling: ["<Param(0)> if <Param(1)>", "If <Param(1)>, <Param(0)>",
    -- "If <Param(1)>, <Param(0)>. Otherwise, <Param(2)>."] (the leading
    -- order is available only when the condition reads nothing the clause
    -- introduced -- a linearization side condition, unchecked here, like the
    -- leading/trailing choice on Delayed -- and the else arm is available
    -- only in the leading order), kind: Sentence
    If : (e : Effect bs) -> (c : Condition (preIntro e)) ->
         (otherwise : Maybe (Effect bs)) -> Effect bs
    -- "…, where X is [amount]" -- the DEFINITION RIDER ([CR#107.3c]), a
    -- binder scoped to one ability: the definition is written last and
    -- binds over the body, which is `Intercepts`' typed-in-what-it-
    -- announced pattern linearized the other way round.
    -- The scope is the EFFECT and not the object, and that is measured
    -- rather than assumed. [CR#107.3i] makes X object-wide by default,
    -- but of 1,118 supported rider cards only NINE write X anywhere
    -- outside the rider's own ability, and every one of those nine
    -- accounts for itself: seven carry a SECOND rider (one per ability --
    -- The Archimandrite), one is the cost-X trap (Riptide Replicator)
    -- and one is a cycling cost feeding its own trigger (Shark Typhoon,
    -- [CR#107.3m]). So no card needs one rider to bind two abilities, and
    -- the narrower binder is the honest one.
    -- The letter STAYS readable through the rest of the effect
    -- (`effIntro`), which is [CR#107.3i] within the ability and is what
    -- the later-sentence readers want ("Draw X cards, where X is …. If
    -- you have an enduring story, this creature deals X damage to each
    -- opponent", Balin).
    -- The body is not required to READ the letter, exactly as a
    -- comparison mints its margin whether or not the statement reads it:
    -- what is written is the rider.
    -- The LETTER is an argument, which is how the second word arrives
    -- ([CR#107.3p]): three cards define a Y beside their X, they write it
    -- as ONE clause ("…, where X is the exiled creature card's power and
    -- Y is its toughness", Bioplasm; a joint "where X and Y are …" is
    -- written zero times), and that clause is two riders nested -- the
    -- inner definition typed in the outer letter's context, the body
    -- reading both. Which is why the row takes a word rather than a
    -- second constructor taking two definitions: the two letters are
    -- defined SEPARATELY on every card that has them, and nesting says so.
    -- spelling: ["<Param(2)>, where <Param(0)> is <Param(1)>"] (Param(0) =
    -- the letter's own word; the body spells that letter at each read
    -- site, and a rider nested immediately inside another joins its
    -- clause with "and" -- "where X is A and Y is B". Which clause of the
    -- body the rider trails is a linearization side condition, unchecked
    -- here as it is on If and Delayed), kind: Sentence
    WhereLetter : (w : LetterWord) -> (def : Amount bs) ->
                  {auto 0 xd : LetterDefinition def} ->
                  (body : Effect (Experimental.letterB w :: bs)) -> Effect bs
    -- the clause SEQUENCE — a card's sentence list and its "…, then
    -- …" alike ([CR#608.2c] orders sub-effects), mirroring core's
    -- `OneShotEffect::Sequentially`: n-ary, because a card writes n
    -- sentences and nothing in the ordering is binary. The discourse
    -- advances left to right, which the `Effects` telescope carries.
    -- At least TWO clauses: an empty sequence is no instruction at all
    -- (core admits `Sequentially([])` structurally — the workbench,
    -- spelling English, does not; `badEmptySequence`), and a
    -- one-clause sequence is a second spelling of that one clause
    -- (`badSingletonSequence`). Its elements are clauses and not
    -- sequences themselves (`NotSeq`, `badNestedSequence`) — the tree
    -- this replaced, refused rather than re-mintable.
    -- spelling: (construction-owned -- the clause-SEQUENCE list sugar over
    -- `Effects`; no connective word of its own ("X. Y." vs "X, then Y." is
    -- the renderer's choice); mirrors core's n-ary
    -- `OneShotEffect::Sequentially`), kind: TODO(reason: a multi-sentence
    -- body isn't one of the five FragmentKinds -- each element is its own
    -- Sentence)
    Sequentially : {0 n : Nat} -> Effects n bs ->
                   {auto 0 ok : AtLeastTwo n} -> Effect bs
    -- the SIMULTANEOUS batch -- one instruction whose parts happen at once
    -- (core's `OneShotEffect::Simultaneously`), an instruction the card
    -- writes as a single verb ("exchange control of A and B") whose parts
    -- have no order between them. Separate effects that merely resolve
    -- together are `Sequentially` and the stack, not this.
    -- The contrast with `Sequentially` is one word: the sequence threads
    -- `effIntro`, so each clause reads what its predecessors DID; this
    -- threads `preIntro`, so each reads only what they ANNOUNCED. Every
    -- element reads ONE game state ([CR#608.2f]: "each such action is
    -- processed simultaneously"), while the DISCOURSE accumulates, the
    -- announcements all having been made before any of it resolved. That
    -- is the pre-state reading an exchange depends on ([CR#701.12b] has
    -- each player gain control of what "was controlled by the other
    -- player"), and reading a sibling's retag, token or outcome is refused
    -- (`badSimultaneousReadsRetag`, `badSimultaneousReadsOutcome`).
    -- Outward it contributes the last element's `effIntro` over the
    -- announcement telescope, which IS the union for every row this
    -- container reaches today; a batch whose EARLIER element moved an
    -- object would lose that retag, and no corpus line writes one
    -- ([CR#701.12d]'s zone exchange is the ledger's).
    -- At least TWO elements, and elements that are neither batches
    -- (`NotSim`) nor SEQUENCES (`NotSeq`) -- an ordered list inside an
    -- unordered one contradicts its container. ALL-OR-NOTHING is
    -- [CR#701.12a]'s, a resolution fact about failed halves rather than a
    -- typing one, so it lives with the legality layer.
    -- spelling: (construction-owned -- the batch has no connective word of
    -- its own: English writes it as ONE verb, and which verb is the macro's
    -- ("exchange control of <a> and <b>", "<a> fights <b>"). Mirrors core's
    -- n-ary `OneShotEffect::Simultaneously`), kind: TODO(reason: a
    -- multi-clause body isn't one of the five FragmentKinds -- each element
    -- is its own Sentence, and no element is a sentence the card prints)
    Simultaneously : {0 n : Nat} -> SimEffects n bs ->
                     {auto 0 ok : AtLeastTwo n} -> Effect bs
    -- "Choose [q] — • [mode] • [mode] …" -- the MODAL clause ([CR#700.2]:
    -- a spell or ability is modal when it has "two or more options in a
    -- bulleted list preceded by instructions for a player to choose a
    -- number of those options"). The rule supplies both halves: the
    -- headcount is a QUANTITY, so it is the one quantity vocabulary and no
    -- parallel number path, and the modes are two or more (`AtLeastTwo`),
    -- the rule's own minimum. Which headcounts are attested is
    -- `modalHead`'s table.
    -- The MODE LIST IS A LIST AND NOT A TELESCOPE, which is the whole
    -- finding of this row: every mode is typed in `bs`, so a mode reads
    -- everything BEFORE the modal and nothing a sibling introduced. Both
    -- directions are measured -- the bullets that open with a pronoun all
    -- reach past the modal to the trigger before it, and zero bullets read
    -- a sibling. [CR#700.2a] says why: the modes are chosen at cast and an
    -- unchosen mode's targets are never announced ([CR#700.2c] treats the
    -- spell "as though it did not have those targets").
    -- OUTWARD it contributes nothing (`effIntro`), equally measured: of
    -- the modal cards with a non-bullet line after the list, all but two
    -- are keyword packaging, and Blood on the Snow is the positive proof,
    -- writing a DESCRIPTION covering both modes' outcomes exactly where an
    -- anaphor would have gone (`badModalReadsAcrossModes`,
    -- `badReadsAfterModal`).
    -- Keyword PACKAGING is not here: entwine, spree, escalate, the "same
    -- mode more than once" instruction ([CR#700.2d]), and the cost-algebra
    -- forms ([CR#700.2h,700.2i]).
    -- spelling: ["choose <Param(0)> — <Param(1)>"] (Param(0) = the
    -- headcount's own words, see Quantity's macros; Param(1) = the mode
    -- list, one bullet "•" per mode, each mode its own sentence or
    -- sentences. WHICH words a headcount writes depends on the mode count
    -- as well as the range -- a top that equals the list spells "or
    -- both" at two modes and "or more" above -- which is a linearization
    -- side condition, unchecked here as the leading/trailing conditional
    -- is), kind: Sentence
    Modal : (q : Quantity) -> (modes : List (Effect bs)) ->
            {auto 0 nz : NonZeroQ q} ->
            {auto 0 wf : WellFormedQ q} ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit q (modeCount modes)} ->
            {auto 0 mh : ModalHead q (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
    -- "[e] [when/at event]" -- the DELAYED triggered ability ([CR#603.7]
    -- -- "an effect may create a delayed triggered ability that can do
    -- something at a later time … will contain 'when', 'whenever', or
    -- 'at', although that word won't usually begin the ability"). The
    -- temporal adverbial stays on its clause (leading vs trailing position
    -- is linearization); the body reads the discourse as settled
    -- particulars transformed by the event (`delayedCtx`,
    -- [CR#603.7c,603.3d]).
    -- Its event is the SHARED vocabulary as of chapter twenty-eight, which
    -- is the ledger's own merge taken. What is NOT the same as the ability
    -- line is the container: a delayed trigger is CREATED by a resolving
    -- effect ([CR#603.7a]) and so is an `Effect` row typed in the
    -- discourse its creator built, where the card-text trigger is an
    -- `Ability` row typed at the empty context. Two constructions, one
    -- event vocabulary -- which is exactly the relation `Intercepts` and
    -- `HeldUntil` already have.
    -- The SPAN is the slot [CR#603.7b] names: "a delayed triggered ability
    -- will trigger only once — the next time its trigger event occurs —
    -- unless it has a stated duration, such as 'this turn'". So the
    -- unwritten span is the once-only default and "this turn" is the one
    -- adverbial the family writes (Graceful Reprieve), which is what the
    -- span tables now record (`admitsDelaySpan`,
    -- `badDelayedUntilEndOfTurn`).
    -- spelling: ["<Param(2)> <Param(0)><Param(1)>"] (trailing adverbial
    -- position; leading position swaps the order, linearization's choice.
    -- The event supplies its own header word -- "when <event>", "at the
    -- beginning of the next end step" -- with "next" derivable exactly as
    -- it is for a duration endpoint, and Param(1) writes the stated
    -- duration after it), kind: Sentence
    Delayed : (ev : GameEvent bs) ->
              {default Nothing span : Maybe (Duration bs)} ->
              Effect (delayedCtx ev) ->
              {auto 0 aw : Awaitable ev} ->
              {auto 0 one : eventSubjectPlur ev = OneOf} ->
              {auto 0 so : DelaySpanOk span} -> Effect bs
    -- "[replaced]. [replacement] instead." -- the SELF-replacement
    -- ([CR#614.15]: an effect of a resolving spell or ability that
    -- replaces part or all of that spell or ability's OWN effects, and the
    -- rule adds that the text creating one "is usually part of the ability
    -- whose effect is being replaced", which is why English writes it as
    -- the next sentence rather than as a standing shield).
    -- Distinguished from `Intercepts` beside it by what it names: the
    -- intercepting form names an EVENT PATTERN and waits, this form names
    -- the very clause before it and substitutes for it. The corpus divides
    -- on the word "would": the interception writes one and this form
    -- writes none.
    -- The replacement is typed in what the replaced clause ANNOUNCED and
    -- not in what it did, which is the whole content of the node.
    -- [CR#614.6] says a replaced event "never happens", so the replaced
    -- clause's outcome is not there to read ("that much" reads nothing --
    -- `badInsteadReadsReplacedOutcome`); [CR#601.2c] announces its targets
    -- all the same, so "that artifact" and "that creature" DO find it.
    -- Outward it contributes the same announcement and no outcome: exactly
    -- one of the two clauses ran and no later sentence can know which,
    -- which is `mayIntro`'s cut ([CR#118.12]) at a second site.
    -- NOT nested: [CR#614.5] gives a replacement effect "only one
    -- opportunity to affect an event or any modified events that may
    -- replace that event", so a replacement of a replacement is not a
    -- second sentence English writes (`badNestedInstead`).
    -- spelling: ["<Param(0)>. <Param(1)> instead."] (the "instead" trails
    -- the replacement clause; the fronted variant writes it first,
    -- "instead <Param(1)>", and the condition that usually governs the
    -- replacement is the replacement clause's own If node, not a slot
    -- here), kind: TODO(reason: a two-sentence body isn't one of the five
    -- FragmentKinds -- each half is its own Sentence)
    InsteadOf : (replaced : Effect bs) -> (repl : Effect (annIntro replaced)) ->
                {auto 0 na : NotInstead replaced} ->
                {auto 0 nb : NotInstead repl} -> Effect bs
    -- "[clause] until [event]" -- the [CR#610.3] rider, and it is NOT a
    -- duration: the rule makes this "one-shot effect" pair with "a second
    -- one-shot effect … created immediately after the specified event",
    -- which returns the object to its previous zone. So the clause
    -- resolves once and schedules its own undo, where a `Continuously`
    -- clause establishes something that lasts.
    -- WHICH clauses take the rider is the corpus's answer and it is one
    -- (`heldUntilOk`): almost every "until [object] leaves the
    -- battlefield" line is an exile, a few are the phase-out twin
    -- [CR#610.4] governs and this vocabulary has no word for, and the
    -- remaining three are continuous effects that belong to the `Duration`
    -- row instead (`badHeldUntilDestroy`).
    -- [CR#610.3c] settles what the undo does not need to say: the object
    -- "returns under its owner's control unless otherwise specified", so
    -- the rider needs no controller slot and the clause writes none.
    -- The event is typed in what the clause announced (`preIntro`), and
    -- the node contributes that same announcement OUTWARD rather than the
    -- exile's zone retag, because the object's zone depends on an event
    -- that has not happened (`badHeldUntilExileRetag`).
    -- spelling: ["<Param(0)> until <Param(1)>"] (trailing rider, no
    -- comma; the event is spelled in the finite mood -- see GameEvent),
    -- kind: Sentence
    HeldUntil : (e : Effect bs) -> (ev : GameEvent (annIntro e)) ->
                {auto 0 ok : HeldClause e} ->
                {auto 0 hd : Holdable ev} -> Effect bs
    -- "[clause]. When [the clause's agent] do, [trigger]" -- the REFLEXIVE
    -- triggered ability ([CR#603.12]: a resolving spell or ability "may
    -- allow or instruct a player to take an action and create a triggered
    -- ability that triggers 'when [a player] [does or doesn't]' take that
    -- action").
    -- It is a CONSTRUCTION over the enclosing clause and NOT a `GameEvent`
    -- row: "you do" is a PRO-VERB abbreviating the clause before it rather
    -- than describing a happening, so the trigger's event slot reads a
    -- CLAUSE where every event row reads the board. Core calls delayed and
    -- reflexive triggers "the same value" (`ability.rs`) and then has
    -- nothing to put in the `event` field that demands; this node keeps
    -- the clause. `reflexEncloseUse` is the enclosure table, and it looks
    -- THROUGH the offer, one action under two markings being what the rule
    -- describes.
    -- The TRIGGER BODY reads the enclosure's POST-state, which is the
    -- difference from `InsteadOf` and from `mayIntro`'s cut: the reflexive
    -- fires precisely BECAUSE the action was taken, so what the action did
    -- is there to read. Targets are SETTLED on the way in (`reflexCtx`):
    -- it is its own object on the stack ([CR#603.3]) choosing its own
    -- targets there ([CR#603.3d]).
    -- Outward it contributes the ENCLOSURE's discourse and none of the
    -- trigger's -- [CR#603.3] puts the ability on the stack "the next time
    -- a player would receive priority", so the rest of the resolution
    -- finishes first (`badAfterReflexiveReadsTrigger`).
    -- Three slots the family does NOT get, each measured: the trigger WORD
    -- (every corpus line writes "When"), the SPAN ([CR#603.12a] replaces
    -- [CR#603.7b] outright, making the count the event's multiplicity),
    -- and the intervening "if", which [CR#603.4] licenses and the ledger
    -- keeps at three lines.
    -- spelling: ["<Param(0)>. When <agent of Param(0)> do, <Param(1)>."]
    -- (the anaphor's pronoun and its verb agreement are the enclosure's
    -- own, exactly as `May`'s branches take theirs -- "when you do" for
    -- an imperative or a "you may", Yes Man's "when they do" for a named
    -- decider -- so no decider slot is needed here; the header word is
    -- fixed at "When"), kind: TODO(reason: a two-sentence body isn't one
    -- of the five FragmentKinds -- each half is its own Sentence)
    Reflexively : (body : Effect bs) -> (trig : Effect (reflexCtx body)) ->
                  {auto 0 en : ReflexEnclosure body} -> Effect bs

    -- "[n] doesn't untap during [possessor] next [count] untap step(s)"
    -- -- the TIMED untap restriction, a resolving one-shot clause
    -- ([CR#611.2a] -- a restriction created by resolution lasts as long
    -- as its text states, against the standing `DoesntUntap` line's
    -- source-bound span [CR#611.3b]). ONE clause covers all 67 corpus
    -- lines: 19 stand alone and 48 ride another action (38 after tap, 10
    -- after mana-addition, damage, or shroud clauses), so "rider" is
    -- placement the clause sequence already owns, and the 38/10
    -- period-versus-"and" split is the renderer's, not a field. What the
    -- restriction governs is [CR#502.3]'s turn-based untap -- the active
    -- player determines which permanents they control will untap, and
    -- effects may keep one or more from untapping.
    -- The SUBJECT is an ordinary battlefield noun, singular or
    -- distributive ("each attacking creature"), demanded exactly as
    -- `Tap`/`Untap` demand theirs (`badUntapNextGraveyard`); no plurality
    -- gate, three of the 67 subjects being plural/distributive.
    -- The POSSESSOR is derived, never a slot: [CR#109.5] makes "you"/
    -- "your" the controller's words, so a self subject writes "your"
    -- (12 lines) and a third party "its controller's" (55) -- the standing
    -- row's own derivation at a second site (finding 250).
    -- The COUNT is the one independent endpoint value, a closed two-row
    -- table over the written Nat (`NextUntapCount`): one and two are the
    -- whole attested vocabulary (66/1), three is unwritten English
    -- (`badUntapNextThree`). [CR#614.10a]'s skipped-step "next" is the
    -- engine's bookkeeping, no part of the phrase.
    -- spelling: ["<Param(0)> doesn't untap during <possessor> next untap
    -- step."] at count one, ["<Param(0)> doesn't untap during <possessor>
    -- next two untap steps."] at count two (the possessor is derived --
    -- "your" for the self subject, "its controller's" otherwise; count
    -- one writes no numeral and singular "step", count two the word "two"
    -- and plural "steps" -- style guide §4 "Fixed counts of things are
    -- words", §11 "This, next, and each"; the contraction is §1 "Voice
    -- and tense", "does not untap" written zero times), kind: Sentence
    DoesntUntapNext : (n : Noun bs Object) -> (steps : Nat) ->
                      {auto 0 ok : OnBattlefield (nounZone n)} ->
                      {auto 0 ct : NextUntapCount steps} -> Effect bs

  ||| Which clause may carry a [CR#610.3] "until [event]" rider. Full rows,
  ||| so a new clause declares whether the corpus hangs one on it. Exactly
  ||| one row is `True` and it is a nested pattern rather than a
  ||| constructor name: the rider goes on a zone change to EXILE and on
  ||| nothing else (Banisher Priest and its siblings). A destroy or a
  ||| graveyard move takes none, and neither does a bare `Move`: the corpus
  ||| writes the rider with the verb tag every time.
  public export
  heldUntilOk : {0 bs : Bindings} -> Effect bs -> Bool
  heldUntilOk (DealDamage _ _ _) = False
  heldUntilOk (DoesntUntapNext _ _) = False
  heldUntilOk (Distribute _ _ _) = False
  heldUntilOk (Fights _ _) = False
  heldUntilOk (Tap _) = False
  heldUntilOk (Untap _) = False
  heldUntilOk (ToFace _ _) = False
  heldUntilOk (Phases _ _) = False
  heldUntilOk (GetsCounters _ _ _) = False
  heldUntilOk (LosesAllCounters _ _) = False
  heldUntilOk (RemoveFromCombat _) = False
  heldUntilOk (TakesDesignation _ _) = False
  heldUntilOk (Goad _) = False
  heldUntilOk (BecomesTime _) = False
  heldUntilOk (Concludes _ _) = False
  heldUntilOk GameDrawn = False
  heldUntilOk (CounterSpell _) = False
  heldUntilOk (Choose _) = False
  heldUntilOk (Move _ _) = False
  heldUntilOk (ChangeLife _ _) = False
  heldUntilOk (Draw _ _) = False
  heldUntilOk (Expose _ _ _) = False
  heldUntilOk (Mill _ _) = False
  heldUntilOk (Search _ _ _) = False
  heldUntilOk (Shuffle _) = False
  heldUntilOk (Continuously _ _) = False
  heldUntilOk (Create _ _ _ _) = False
  heldUntilOk (PutCounters _ _ _) = False
  heldUntilOk (RemoveCounters _ _ _) = False
  -- the UNRIDDEN exile, and the bundle is part of the answer now that a
  -- rider exists to write there: zero corpus lines carry counters into a
  -- held exile ("exile … with … counters on it until …" is unwritten),
  -- and the two constructions are answering different questions about
  -- the same card — [CR#610.3] schedules the return where the counters
  -- wait for an ability to read them (`badHeldUntilWithCounters`).
  heldUntilOk (Composite Exile (Move _ _ {riders = MkMoveRiders [] Nothing
                                                    {counters = Nothing}})) = True
  heldUntilOk (Composite Exile (Move _ _ {riders = MkMoveRiders _ _
                                                    {counters = Just _}})) = False
  heldUntilOk (Composite Exile (Move _ _ {riders = MkMoveRiders (_ :: _) _})) = False
  heldUntilOk (Composite Exile (Move _ _ {riders = MkMoveRiders _ (Just _)})) = False
  heldUntilOk (Composite _ _) = False
  heldUntilOk (Does _ _ _) = False
  heldUntilOk (Pay _ _) = False
  heldUntilOk (May _ _ _ _) = False
  heldUntilOk (If _ _ _) = False
  heldUntilOk (WhereLetter _ _ _) = False
  heldUntilOk (Sequentially _) = False
  heldUntilOk (Simultaneously _) = False
  heldUntilOk (Modal _ _) = False
  heldUntilOk (Delayed _ _) = False
  heldUntilOk (InsteadOf _ _) = False
  heldUntilOk (HeldUntil _ _) = False
  heldUntilOk (Reflexively _ _) = False

  ||| WHY a clause is or is not a reflexive trigger's enclosure
  ||| ([CR#603.12]). Chapter twenty-eight's `PartUse` shape, and for its
  ||| reason: the Falses have four different grounds and a Bool would
  ||| have hidden which one each row was standing on.
  public export
  data EncloseUse
    = ||| No PLAYER takes the action, so "you do" has no subject to
      ||| inflect for. [CR#603.12] wants a resolving effect that "allow(s)
      ||| or instruct(s) A PLAYER to take an action".
      EncAgentless
    | ||| No single taken action for the pro-verb to abbreviate — a
      ||| sequence, a batch, a modal, a conditional, or a clause exactly
      ||| one of whose halves ran.
      EncNotOneAction
    | ||| The clause schedules its action rather than taking it, so
      ||| nothing "occurred earlier during the resolution"
      ||| ([CR#603.12]'s own check).
      EncNotYetTaken
    | ||| A player's own single action, and the corpus hangs no reflexive
      ||| on it.
      EncUnattested
    | ||| Real oracle enclosure this vocabulary cannot finish (ledger).
      EncUnclaimed
    | ||| Attested and built.
      EncReflexive

  ||| Which clause may be the enclosure a "When [you] do" reads. Full rows,
  ||| so a new clause declares whether English writes the anaphor over it,
  ||| and the Trues are counted from the corpus rather than reasoned to.
  |||
  ||| It looks THROUGH the offer, which is the rule's own wording read
  ||| literally: [CR#603.12] has a resolving effect "allow OR INSTRUCT a
  ||| player to take AN ACTION", one action under two markings, so a
  ||| branchless `May` asks its body's question rather than a question of
  ||| its own — which is why the may/imperative split is nowhere in this
  ||| table. What the look-through does NOT reach is a disjunctive offer
  ||| ("you may sacrifice a Food or pay {2}{W}"), which names two actions
  ||| under one may and has no clause disjunction to ask the question of.
  |||
  ||| The AGENTLESS rows are a rule the corpus corroborates, not a count
  ||| alone. [CR#120.1] makes "an object that deals damage … the source of
  ||| that damage", so a damage clause's agent is the source and not a
  ||| player; [CR#701.14a] has a fight instruct "a creature to fight
  ||| another creature"; and [CR#119.9] rewrites the life-gain trigger as
  ||| "whenever a source causes [a player] to gain life", making the player
  ||| the PATIENT and giving "do" nobody to stand for. The corpus agrees at
  ||| zero lines each. The one apparent counterexample proves the rule:
  ||| Elektra, Femme Fatale writes "you may HAVE her deal 2 damage to you.
  ||| When you do, …", where the having is yours and the dealing is hers —
  ||| a `Does` causative, which is a True row. The same asymmetry shows in
  ||| the life change itself: "you may pay 2 life. When you do" is three
  ||| lines and a `Pay`, paying being an action a player takes where
  ||| gaining life is not.
  |||
  ||| `Continuously` is agentless for the same reason and carries the one
  ||| `EncUnclaimed` cell as a nested row: the CONTROL grant is attested
  ||| exactly once and by ruling rather than inference (Yes Man, Personal
  ||| Securitron, whose 2024-03-08 ruling names the construction outright).
  ||| Its second sentence wants a quest counter `CounterKind` does not
  ||| carry, so the cell records the line it cannot write.
  public export
  reflexEncloseUse : {0 bs : Bindings} -> Effect bs -> EncloseUse
  reflexEncloseUse (DealDamage _ _ _) = EncAgentless
  reflexEncloseUse (DoesntUntapNext _ _) = EncAgentless
  reflexEncloseUse (Distribute _ _ _) = EncAgentless
  reflexEncloseUse (Fights _ _) = EncAgentless
  reflexEncloseUse (ChangeLife _ _) = EncAgentless
  reflexEncloseUse (Continuously (GainsControl _ _) _) = EncUnclaimed
  reflexEncloseUse (Continuously _ _) = EncAgentless
  -- 115: the sacrifice at seventy-one, the discard at forty, and four
  -- have-causatives, which is where the agentless rows' one apparent
  -- counterexample lives.
  reflexEncloseUse (Does _ _ _) = EncReflexive
  reflexEncloseUse (Pay _ _) = EncReflexive        -- 66, all of them offered
  reflexEncloseUse (Composite _ _) = EncReflexive  -- 51, every one an exile
  reflexEncloseUse (Tap _) = EncReflexive          -- 13
  reflexEncloseUse (Untap _) = EncUnattested
  -- A player's own single action, and no line writes the anaphor over
  -- it: "turn … face up. When you do, …" is zero lines in either
  -- direction, the ten offers ("you may turn a creature you control face
  -- up") all standing alone.
  reflexEncloseUse (ToFace _ _) = EncUnattested
  -- Agentless for the reason the clause is subject-first: no player
  -- turns the permanent, it phases, so "do" has nobody to stand for
  -- ([CR#603.12] wants an effect that instructs A PLAYER).
  reflexEncloseUse (Phases _ _) = EncAgentless
  -- both player counter verbs are agentless for `ChangeLife`'s reason
  -- ([CR#119.9]'s rewrite read across): the player is the PATIENT of a
  -- marker being placed or cleared, not a player taking an action, so
  -- "do" has nobody to stand for. Zero lines write the anaphor over
  -- either.
  reflexEncloseUse (GetsCounters _ _ _) = EncAgentless
  reflexEncloseUse (LosesAllCounters _ _) = EncAgentless
  -- a player's own single action with no anaphor on it: "remove … from
  -- combat. When you do, …" is zero lines.
  reflexEncloseUse (RemoveFromCombat _) = EncUnattested
  -- the designation clauses hang no anaphor: "becomes the monarch. When
  -- you do" and its siblings are zero lines. The game-scope row is
  -- additionally agentless -- nobody makes it day.
  reflexEncloseUse (TakesDesignation _ _) = EncUnattested
  reflexEncloseUse (Goad _) = EncUnattested
  reflexEncloseUse (BecomesTime _) = EncAgentless
  -- agentless for `ChangeLife`'s reason ([CR#119.9]'s rewrite read
  -- across): winning and losing HAPPEN to a player rather than being
  -- actions one takes, so "do" has nobody to stand for. The draw has no
  -- participant at all.
  reflexEncloseUse (Concludes _ _) = EncAgentless
  reflexEncloseUse GameDrawn = EncAgentless
  -- No line writes "counter target spell. When you do, …": the
  -- countering is mandatory wherever it appears, so there is no offer
  -- for the pro-verb to abbreviate.
  reflexEncloseUse (CounterSpell _) = EncUnattested
  reflexEncloseUse (Create _ _ _ _) = EncReflexive -- 8
  reflexEncloseUse (PutCounters _ _ _) = EncReflexive    -- 8
  reflexEncloseUse (RemoveCounters _ _ _) = EncReflexive -- 7
  reflexEncloseUse (Mill _ _) = EncReflexive       -- 5
  reflexEncloseUse (Move _ _) = EncReflexive       -- 3
  reflexEncloseUse (Expose _ _ _) = EncReflexive   -- 2
  reflexEncloseUse (Draw _ _) = EncReflexive       -- 1 ([CR#121.1]: a PLAYER draws)
  reflexEncloseUse (Choose _) = EncReflexive       -- 1
  reflexEncloseUse (Search _ _ _) = EncUnattested
  reflexEncloseUse (Shuffle _) = EncUnattested
  -- the OFFER, [CR#603.12]'s "allow" half at two hundred of the two
  -- hundred ninety-one lines, asking its body's question. Branchless,
  -- because a branched may has already read the same taken-ness by
  -- [CR#118.12]'s clause and no line writes both readers over one offer.
  reflexEncloseUse (May _ body Nothing Nothing) = reflexEncloseUse body
  reflexEncloseUse (May _ _ _ _) = EncNotOneAction
  reflexEncloseUse (If _ _ _) = EncNotOneAction
  reflexEncloseUse (WhereLetter _ _ _) = EncNotOneAction
  reflexEncloseUse (Sequentially _) = EncNotOneAction
  reflexEncloseUse (Simultaneously _) = EncNotOneAction
  reflexEncloseUse (Modal _ _) = EncNotOneAction
  reflexEncloseUse (InsteadOf _ _) = EncNotOneAction
  reflexEncloseUse (Reflexively _ _) = EncNotOneAction
  reflexEncloseUse (Delayed _ _) = EncNotYetTaken
  reflexEncloseUse (HeldUntil _ _) = EncNotYetTaken

  ||| Only the attested cell opens. `EncUnclaimed` answers `False` with
  ||| the rest, which is the whole point of keeping it a separate row.
  public export
  admitsReflexEnclosure : EncloseUse -> Bool
  admitsReflexEnclosure EncAgentless = False
  admitsReflexEnclosure EncNotOneAction = False
  admitsReflexEnclosure EncNotYetTaken = False
  admitsReflexEnclosure EncUnattested = False
  admitsReflexEnclosure EncUnclaimed = False
  admitsReflexEnclosure EncReflexive = True

  ||| The enclosure demand as a witness, so a pin says which question
  ||| refused (`CostAction`'s discipline).
  public export
  data ReflexEnclosure : Effect bs -> Type where
    MkReflexEnclosure :
      {auto 0 ok : admitsReflexEnclosure (reflexEncloseUse e) = True} ->
      ReflexEnclosure e

  ||| May this cost stand as the complement of the VERB "pay"? A cost is
  ||| the whole thing an ability charges; "pay" is one English verb, and
  ||| the two are not the same set. The complements were counted where the
  ||| verb appears most, after "unless": one symbol run or "N life", and
  ||| every other payment there writes its OWN verb ("unless you sacrifice
  ||| a land", "unless you discard a card"), never "pay". So a sacrifice is
  ||| a payment but not a payABLE, and saying otherwise would spell "you
  ||| pay sacrifice a creature" (`badPayBySacrificing`). The symbols are
  ||| refused for the same reason and more sharply: "{T}" is a cost that
  ||| can only be written before a colon. The compound is refused as
  ||| unattested.
  public export
  payableOk : {0 bs : Bindings} -> Cost bs -> Bool
  payableOk (Mana _) = True
  payableOk TapSymbol = False
  payableOk UntapSymbol = False
  payableOk (Do (ChangeLife _ (Down _))) = True
  payableOk (Do _) = False
  payableOk (Compound _) = False

  ||| The pay-complement demand as a witness.
  public export
  data Payable : Cost bs -> Type where
    MkPayable : {auto 0 ok : payableOk c = True} -> Payable c

  ||| May this phrase stand as a cost component's PATIENT? Two refusals,
  ||| both about the position rather than the verb. A participle read ("the
  ||| sacrificed artifact") names a sibling component's deed, and
  ||| [CR#601.2h] has the player pay the components "in any order", so no
  ||| component may presuppose another has been paid. And a TARGET is
  ||| announced as the ability is activated ([CR#601.2c]), before any cost
  ||| is paid at all, so a cost clause never carries the determiner.
  ||| Shallow: the ascription and the two group determiners pass their
  ||| complement through, and nothing looks inside a predicate.
  public export
  costNounOk : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  costNounOk This = True
  costNounOk (AsType t n) = costNounOk n
  costNounOk You = True
  costNounOk (PlayerGroup _) = True
  costNounOk (Each _) = True
  costNounOk (Indefinite _ _) = True
  costNounOk (TargetGroup _ _) = False
  costNounOk (AllOf _) = True
  costNounOk (EachOf grp) = costNounOk grp
  costNounOk (LibrarySlice _ _ _) = True
  costNounOk (SomeOf _ grp) = costNounOk grp
  costNounOk TheRest = True
  costNounOk It = True
  costNounOk They = True
  costNounOk Them = True
  costNounOk (Those _) = True
  costNounOk (That _) = True
  costNounOk (AttachHost _ _) = True
  costNounOk (TheVerbed _ _) = False
  costNounOk (ThoseVerbed _ _) = False
  costNounOk (ControllerOf _) = True
  costNounOk (OwnerOf _) = True

  ||| Is this phrase the activator? [CR#602.1a] says the activation cost
  ||| "must be paid by the player who is activating it", so a component
  ||| that names its payer names "you" and nobody else. Full rows.
  public export
  nounIsYou : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsYou You = True
  nounIsYou (PlayerGroup _) = False
  nounIsYou This = False
  nounIsYou (AsType _ _) = False
  nounIsYou (Each _) = False
  nounIsYou (Indefinite _ _) = False
  nounIsYou (TargetGroup _ _) = False
  nounIsYou (AllOf _) = False
  nounIsYou (EachOf _) = False
  nounIsYou (LibrarySlice _ _ _) = False
  nounIsYou (SomeOf _ _) = False
  nounIsYou TheRest = False
  nounIsYou It = False
  nounIsYou They = False
  nounIsYou Them = False
  nounIsYou (Those _) = False
  nounIsYou (That _) = False
  nounIsYou (AttachHost _ _) = False
  nounIsYou (TheVerbed _ _) = False
  nounIsYou (ThoseVerbed _ _) = False
  nounIsYou (ControllerOf _) = False
  nounIsYou (OwnerOf _) = False

  ||| May this clause stand as a payment? [CR#602.1a] makes a cost what the
  ||| ACTIVATOR pays, so the table is not "is this a legal sentence" but
  ||| "does oracle write this before a colon" — and the corpus answers per
  ||| verb, which is why this is a table and not a blanket yes. Sacrifice,
  ||| discard, remove-counters, exile, pay-life, tap, return, put-counters,
  ||| reveal and mill all appear as components; the refusals are measured
  ||| the same way and are zeroes — no line writes "Destroy …:" or "Draw
  ||| …:" or a shuffle or a search as a cost, which is the ledger's own
  ||| note that "you lose N life" and a delayed clause are not payments
  ||| ([CR#118.1]). Core's `Action::is_cost_eligible` lists the same verbs
  ||| and reaches them from the engine side.
  |||
  ||| The LIFE row is directional, and measured: "Pay N life" components
  ||| against zero gain-life ones, so `Down` pays and `Up` does not
  ||| (`badGainLifeCost`). Core admits the whole family, and a gain-life
  ||| cost does exist — [CR#119.7] speaks of "a cost that involves having
  ||| that player gain life" — but the cards that print one (Invigorate)
  ||| spell it as an ALTERNATIVE cost ([CR#118.9]), a base swap, never
  ||| before a colon.
  |||
  ||| Two demands ride ON TOP of the verb table, and both are about the
  ||| POSITION rather than the verb. The patient goes through `costNounOk`
  ||| ([CR#601.2h] order-freedom, [CR#601.2c] announcement). And the
  ||| placement row is narrowed by DESTINATION: every zone-change verb the
  ||| corpus writes before a colon removes or downgrades, and a bare
  ||| battlefield entry as a cost component is written zero times.
  public export
  costActionOk : {0 bs : Bindings} -> Effect bs -> Bool
  costActionOk (DealDamage _ _ _) = False
  costActionOk (DoesntUntapNext _ _) = False
  costActionOk (Distribute _ _ _) = False
  costActionOk (Fights _ _) = False
  costActionOk (Tap n) = costNounOk n
  costActionOk (Untap n) = costNounOk n
  -- Neither new clause is ever written before a colon: measured zero in
  -- both families.
  costActionOk (ToFace _ _) = False
  costActionOk (Phases _ _) = False
  costActionOk (GetsCounters _ _ _) = False
  costActionOk (LosesAllCounters _ _) = False
  costActionOk (RemoveFromCombat _) = False
  costActionOk (TakesDesignation _ _) = False
  costActionOk (Goad _) = False
  costActionOk (BecomesTime _) = False
  costActionOk (Concludes _ _) = False
  costActionOk GameDrawn = False
  -- Zero cost components counter a spell: [CR#602.1a] makes a cost what
  -- the ACTIVATOR pays, and cancelling somebody else's spell is not a
  -- payment (`badCounterAsCost`).
  costActionOk (CounterSpell _) = False
  costActionOk (Choose _) = False
  costActionOk (Move what to) =
    costNounOk what && not (sameZone (zoneSort to) Battlefield)
  costActionOk (ChangeLife _ (Down _)) = True
  costActionOk (ChangeLife _ _) = False
  costActionOk (Draw _ _) = False
  costActionOk (Expose Reveal _ _) = True
  costActionOk (Expose _ _ _) = False
  costActionOk (Mill _ _) = True
  costActionOk (Search _ _ _) = False
  costActionOk (Shuffle _) = False
  costActionOk (Continuously _ _) = False
  costActionOk (Create _ _ _ _) = False
  costActionOk (PutCounters _ _ on) = costNounOk on
  costActionOk (RemoveCounters _ _ from) = costNounOk from
  costActionOk (Composite Exile e) = costActionOk e
  costActionOk (Composite _ _) = False
  costActionOk (Does _ Sacrifice e) = costActionOk e
  costActionOk (Does _ Discard e) = costActionOk e
  costActionOk (Does _ _ _) = False
  costActionOk (Pay _ _) = False
  costActionOk (May _ _ _ _) = False
  costActionOk (If _ _ _) = False
  costActionOk (WhereLetter _ _ _) = False
  costActionOk (Sequentially _) = False
  costActionOk (Simultaneously _) = False
  costActionOk (Modal _ _) = False
  costActionOk (Delayed _ _) = False
  costActionOk (InsteadOf _ _) = False
  costActionOk (HeldUntil _ _) = False
  costActionOk (Reflexively _ _) = False

  ||| The payability demand as a witness, so a pin says which question
  ||| refused (`Headed`'s discipline).
  public export
  data CostAction : Effect bs -> Type where
    MkCostAction : {auto 0 ok : costActionOk e = True} -> CostAction e

  public export
  data HeldClause : Effect bs -> Type where
    MkHeldClause : {auto 0 ok : heldUntilOk e = True} -> HeldClause e

  ||| Is this clause itself a self-replacement? `NotSeq`'s shape at a
  ||| third site, and refused for [CR#614.5]'s reason: a replacement
  ||| effect gets "only one opportunity to affect an event or any
  ||| modified events that may replace that event", so neither half of
  ||| an `InsteadOf` is another one.
  public export
  isInstead : {0 bs : Bindings} -> Effect bs -> Bool
  isInstead (InsteadOf _ _) = True
  isInstead _ = False

  public export
  data NotInstead : Effect bs -> Type where
    MkNotInstead : {auto 0 ok : isInstead e = False} -> NotInstead e

  ||| How many modes a modal offers — `length` written in this mutual
  ||| block rather than borrowed from the Prelude, because the count sits
  ||| in the constructor's own type and the positivity checker reads only
  ||| the functions declared alongside the type it is checking.
  public export
  modeCount : {0 bs : Bindings} -> List (Effect bs) -> Nat
  modeCount [] = Z
  modeCount (_ :: es) = S (modeCount es)

  ||| Syntactic clause equality — `predEq`'s discipline one layer up, and
  ||| conservative in exactly its direction: `True` means "provably the
  ||| same clause", so a `False` is "not provably the same" and the gate
  ||| that reads it under-refuses. Most rows are that `False`, and the
  ||| reason is the telescope rather than laziness: a clause's later
  ||| arguments are typed in the context its earlier ones built, so two
  ||| clauses' payloads generally inhabit two different types and cannot be
  ||| compared at all. What CAN be compared is compared — the verb tag, a
  ||| same-context noun (`nounEqRef`), a destination zone, and the written
  ||| count of the imperative draw, whose subject is spelled `You`.
  public export
  effEq : {0 bs : Bindings} -> Effect bs -> Effect bs -> Bool
  effEq (DealDamage _ _ _) _ = False
  effEq (DoesntUntapNext n s) (DoesntUntapNext m t) = nounEqRef n m && s == t
  effEq (DoesntUntapNext _ _) _ = False
  effEq (Distribute _ _ _) _ = False
  effEq (Fights _ _) _ = False
  effEq (Tap a) (Tap b) = nounEqRef a b
  effEq (Tap _) _ = False
  effEq (Untap a) (Untap b) = nounEqRef a b
  effEq (Untap _) _ = False
  effEq (ToFace v a) (ToFace w b) = sameStatusVal v w && nounEqRef a b
  effEq (ToFace _ _) _ = False
  effEq (Phases v a) (Phases w b) = sameStatusVal v w && nounEqRef a b
  effEq (Phases _ _) _ = False
  effEq (GetsCounters _ _ _) _ = False
  effEq (LosesAllCounters a Nothing) (LosesAllCounters b Nothing) = nounEqRef a b
  effEq (LosesAllCounters a (Just j)) (LosesAllCounters b (Just l)) =
    nounEqRef a b && sameCounter j l
  effEq (LosesAllCounters _ _) _ = False
  effEq (RemoveFromCombat a) (RemoveFromCombat b) = nounEqRef a b
  effEq (RemoveFromCombat _) _ = False
  effEq (TakesDesignation a d) (TakesDesignation b e) = nounEqRef a b && samePlayerDesignation d e
  effEq (TakesDesignation _ _) _ = False
  effEq (Goad a) (Goad b) = nounEqRef a b
  effEq (Goad _) _ = False
  effEq (BecomesTime a) (BecomesTime b) = sameTimeOfDay a b
  effEq (BecomesTime _) _ = False
  effEq (Concludes v a) (Concludes w b) = sameOutcomeVerb v w && nounEqRef a b
  effEq (Concludes _ _) _ = False
  effEq GameDrawn GameDrawn = True
  effEq GameDrawn _ = False
  effEq (CounterSpell a) (CounterSpell b) = nounEqRef a b
  effEq (CounterSpell _) _ = False
  effEq (Choose _) _ = False
  effEq (Move a s) (Move b t) = nounEqRef a b && sameZone (zoneSort s) (zoneSort t)
  effEq (Move _ _) _ = False
  effEq (ChangeLife _ _) _ = False
  effEq (Draw You a) (Draw You b) = boundEq a b
  effEq (Draw _ _) _ = False
  effEq (Expose _ _ _) _ = False
  effEq (Mill _ _) _ = False
  effEq (Search _ _ _) _ = False
  effEq (Shuffle _) _ = False
  effEq (Continuously _ _) _ = False
  effEq (Create _ _ _ _) _ = False
  effEq (PutCounters _ _ _) _ = False
  effEq (RemoveCounters _ _ _) _ = False
  effEq (Composite v e) (Composite w f) = sameVerb v w && effEq e f
  effEq (Composite _ _) _ = False
  effEq (Does _ _ _) _ = False
  effEq (Pay _ _) _ = False
  effEq (May _ _ _ _) _ = False
  effEq (If _ _ _) _ = False
  -- conservative, `If`'s answer: two riders are not provably the same
  -- clause, and the only consumer is a mode-repetition check.
  effEq (WhereLetter _ _ _) _ = False
  effEq (Sequentially _) _ = False
  effEq (Simultaneously _) _ = False
  effEq (Modal _ _) _ = False
  effEq (Delayed _ _) _ = False
  effEq (InsteadOf _ _) _ = False
  effEq (HeldUntil _ _) _ = False
  effEq (Reflexively _ _) _ = False

  ||| No mode repeats another. [CR#700.2] calls a spell modal when its
  ||| bulleted options are "preceded by instructions for a player to choose
  ||| a number of those options", and two identical options are one option
  ||| written twice — the choice between them decides nothing. [CR#700.2d]
  ||| settles it from the other side: a player choosing more than one mode
  ||| "normally can't choose the same mode more than once", and the cards
  ||| that lift that restriction say so in words rather than by printing
  ||| the mode twice. Structural, and conservative with it (`effEq`): what
  ||| it catches is the degenerate repetition (`badDuplicateModes`).
  public export
  anyEffEq : {0 bs : Bindings} -> Effect bs -> List (Effect bs) -> Bool
  anyEffEq e [] = False
  anyEffEq e (f :: fs) = effEq e f || anyEffEq e fs

  public export
  distinctModes : {0 bs : Bindings} -> List (Effect bs) -> Bool
  distinctModes [] = True
  distinctModes (e :: es) = not (anyEffEq e es) && distinctModes es

  ||| A clause sequence as a TELESCOPE, not a list of independent
  ||| clauses: each element is typed in the bindings its predecessors
  ||| introduced, so "Destroy target creature. Its controller discards
  ||| a card." can read the destroyed creature in the second sentence.
  ||| Length-indexed, which is all `Sequentially` needs to demand two.
  ||| Written with list syntax, so a card's sentences read as the card
  ||| writes them.
  public export
  -- spelling: (construction-owned -- list syntax for the Sequentially
  -- telescope; Nil/(::) are Idris list sugar, not English words. See
  -- Sequentially)
  data Effects : Nat -> Bindings -> Type where
    Nil : Effects Z bs
    (::) : (e : Effect bs) -> {auto 0 ns : NotSeq e} ->
           Effects n (effIntro e) -> Effects (S n) bs

  ||| Is this clause itself a sequence?
  public export
  isSeq : {0 bs : Bindings} -> Effect bs -> Bool
  isSeq (Sequentially _) = True
  isSeq _ = False

  ||| A sequence's ELEMENTS are clauses, not sequences. `Sequentially
  ||| [Sequentially [a, b], c]` re-mints the right-nested tree the n-ary
  ||| telescope was built to replace, and spells a three-sentence card a
  ||| second way (`badNestedSequence`) — the same one-meaning-one-spelling
  ||| refusal the singleton sequence gets, and the one core reaches by
  ||| flattening in `normalize` instead. What may still hold a sequence is
  ||| a clause slot that takes a BODY — a "may" arm, a delayed clause, a
  ||| tagged composite — where the nesting is the card's own bracketing.
  public export
  data NotSeq : Effect bs -> Type where
    MkNotSeq : {auto 0 ok : isSeq e = False} -> NotSeq e

  ||| A simultaneous batch as an ANNOUNCEMENT telescope: each element is
  ||| typed in the bindings its predecessors ANNOUNCED (`annIntro`) and not
  ||| in what they did (`effIntro`, which is `Effects` beside it). One line
  ||| of difference between the two types, and it is the whole semantic
  ||| contrast between the two containers. The thread was `preIntro` until
  ||| chapter twenty-six, which is nearly the same list and not the same
  ||| function: a `May` element's optional DEED crossed into its siblings
  ||| through it (`badSimultaneousReadsMayDeed`).
  -- spelling: (construction-owned -- list syntax for the Simultaneously
  -- telescope; Nil/(::) are Idris list sugar, not English words. See
  -- Simultaneously)
  namespace Sim
    public export
    data SimEffects : Nat -> Bindings -> Type where
      Nil : SimEffects Z bs
      (::) : (e : Effect bs) -> {auto 0 ns : NotSim e} -> {auto 0 nq : NotSeq e} ->
             SimEffects n (annIntro e) -> SimEffects (S n) bs

  ||| Is this clause itself a simultaneous batch?
  public export
  isSim : {0 bs : Bindings} -> Effect bs -> Bool
  isSim (Simultaneously _) = True
  isSim _ = False

  ||| A batch's ELEMENTS are clauses, not batches — `NotSeq`'s twin, and
  ||| refused for its reason: `Simultaneously [Simultaneously [a, b], c]`
  ||| spells a three-part instruction a second way (`badNestedSimultaneous`).
  public export
  data NotSim : Effect bs -> Type where
    MkNotSim : {auto 0 ok : isSim e = False} -> NotSim e

  ||| Does this noun phrase spell "any target"? Only a counted target
  ||| mention can — every other determiner demands an any-target-free
  ||| phrase, and the reads carry no phrase at all. Full rows, so a new
  ||| determiner declares its answer.
  public export
  nounIsAnyTarget : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounIsAnyTarget (TargetGroup _ p) = headIsAnyTarget p
  nounIsAnyTarget This = False
  nounIsAnyTarget (AsType t n) = nounIsAnyTarget n
  nounIsAnyTarget You = False
  nounIsAnyTarget (PlayerGroup _) = False
  nounIsAnyTarget (Each _) = False
  nounIsAnyTarget (Indefinite _ _) = False
  nounIsAnyTarget (AllOf _) = False
  nounIsAnyTarget (EachOf grp) = nounIsAnyTarget grp
  nounIsAnyTarget (LibrarySlice _ _ _) = False
  nounIsAnyTarget (SomeOf _ grp) = nounIsAnyTarget grp
  nounIsAnyTarget TheRest = False
  nounIsAnyTarget It = False
  nounIsAnyTarget They = False
  nounIsAnyTarget Them = False
  nounIsAnyTarget (Those _) = False
  nounIsAnyTarget (That _) = False
  nounIsAnyTarget (AttachHost _ _) = False
  nounIsAnyTarget (TheVerbed _ _) = False
  nounIsAnyTarget (ThoseVerbed _ _) = False
  nounIsAnyTarget (ControllerOf _) = False
  nounIsAnyTarget (OwnerOf _) = False

  ||| The class word refused, as a witness — [CR#115.4] confines "any
  ||| target" to a creature, a player, a planeswalker or a battle, so
  ||| every carrier that is not the damage recipient's has to say no to
  ||| it. The zone projection cannot say it alone: the phrase places its
  ||| referent nowhere, and a silence passes a `zoneFits` demand
  ||| (`badExileAnyTarget`).
  public export
  data NotAnyTarget : Noun bs k -> Type where
    MkNotAnyTarget : {auto 0 ok : nounIsAnyTarget n = False} -> NotAnyTarget n

  ||| Does this phrase carry the TARGET determiner? The noun-level twin
  ||| of `anyTargetedAt`, for the slots that ask about one phrase rather
  ||| than about everything in scope. The ascription and the two group
  ||| determiners pass their complement through; nothing looks inside a
  ||| predicate, a target mention inside a relative clause being the
  ||| anchor construction and not this phrase's marking. Full rows.
  public export
  nounTargeted : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  nounTargeted (TargetGroup _ _) = True
  nounTargeted This = False
  nounTargeted (AsType _ n) = nounTargeted n
  nounTargeted You = False
  nounTargeted (PlayerGroup _) = False
  nounTargeted (Each _) = False
  nounTargeted (Indefinite _ _) = False
  nounTargeted (AllOf _) = False
  nounTargeted (EachOf grp) = nounTargeted grp
  nounTargeted (LibrarySlice _ _ _) = False
  nounTargeted (SomeOf _ grp) = nounTargeted grp
  nounTargeted TheRest = False
  nounTargeted It = False
  nounTargeted They = False
  nounTargeted Them = False
  nounTargeted (Those _) = False
  nounTargeted (That _) = False
  nounTargeted (AttachHost _ _) = False
  nounTargeted (TheVerbed _ _) = False
  nounTargeted (ThoseVerbed _ _) = False
  nounTargeted (ControllerOf _) = False
  nounTargeted (OwnerOf _) = False

  ||| The nontarget demand as a witness, so a pin says which question
  ||| refused.
  public export
  data Nontarget : Noun bs k -> Type where
    MkNontarget : {auto 0 ok : nounTargeted n = False} -> Nontarget n

  ||| Is this phrase writable as an EVENT's subject? One refusal: the bare
  ||| self-reference. [CR#603.6a] quotes the template as "When [this
  ||| object] enters" and the corpus spells that object with its type word
  ||| without exception — "When this enters" is zero lines against
  ||| thousands, and the same zero-against-hundreds for dies, attacks,
  ||| blocks and deals combat damage. The ascription is not decoration:
  ||| [CR#109.2] places a description that includes a card type on the
  ||| battlefield, which is exactly the evidence these events demand, where
  ||| bare `This` is the source as an object ("this spell") and stands
  ||| nowhere the grammar tracks (`badEntersBareThis`).
  public export
  selfSortedOk : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  selfSortedOk This = False
  selfSortedOk (AsType _ _) = True
  selfSortedOk You = True
  selfSortedOk (PlayerGroup _) = True
  selfSortedOk (Each _) = True
  selfSortedOk (Indefinite _ _) = True
  selfSortedOk (TargetGroup _ _) = True
  selfSortedOk (AllOf _) = True
  selfSortedOk (EachOf _) = True
  selfSortedOk (LibrarySlice _ _ _) = True
  selfSortedOk (SomeOf _ _) = True
  selfSortedOk TheRest = True
  selfSortedOk It = True
  selfSortedOk They = True
  selfSortedOk Them = True
  selfSortedOk (Those _) = True
  selfSortedOk (That _) = True
  selfSortedOk (AttachHost _ _) = True
  selfSortedOk (TheVerbed _ _) = True
  selfSortedOk (ThoseVerbed _ _) = True
  selfSortedOk (ControllerOf _) = True
  selfSortedOk (OwnerOf _) = True

  ||| The event subject's demand as a witness, so a pin says which
  ||| question refused.
  public export
  data SelfSorted : Noun bs k -> Type where
    MkSelfSorted : {auto 0 ok : selfSortedOk n = True} -> SelfSorted n

  ||| Who can take damage ([CR#120.1,120.1a] — battles, creatures,
  ||| planeswalkers, players; never a quality, never an off-battlefield
  ||| card, never a noncreature artifact or land) — asked of the NOUN, the
  ||| way `DiscardOk` asks its verb's question. Three rows: any player; the
  ||| class word, which NAMES [CR#115.4]'s damage class and so answers for
  ||| itself without a zone or a head type; and every other object phrase,
  ||| which must stand on the battlefield under a damageable head. The
  ||| middle row is what lets the class word's zone projection stay honest
  ||| — "any target" places its referent nowhere, so `nounZone` gives it
  ||| none and the battlefield verbs refuse it (`badDestroyAnyTarget`) —
  ||| where a zone-FREE object row would have bought the same refusal at
  ||| the price of admitting bare `This` (`badDamageThis`).
  public export
  data DamageRecipient : Noun bs k -> Type where
    PlayerTakes : DamageRecipient {k = Player} n
    AnyTargetTakes : {auto 0 ok : nounIsAnyTarget n = True} ->
                     DamageRecipient {k = Object} n
    ObjectTakes : {auto 0 field : OnBattlefield (nounZone n)} ->
                  {auto 0 dm : DamageableTy (nounTy n)} ->
                  DamageRecipient {k = Object} n

  ||| The hand half of discard's implicit restriction, asked of the
  ||| NOUN rather than of a zone ([CR#701.9a] — a discard moves a card
  ||| from a hand). Two rows, and only two: the bare self-reference,
  ||| which is the source as an OBJECT ("Discard this card") and so
  ||| projects no zone at all — cycling's own cost is the whole
  ||| justification for it ([CR#702.29a]; `cyclingCost`) — and any noun
  ||| whose fold-state actually stands in a hand. An untracked zone no
  ||| longer passes on its own strength: a READ that reaches an
  ||| unplaced referent is not thereby a hand card (`badDiscardIt`),
  ||| which is what the old zone-level spelling could not say.
  public export
  data DiscardOk : Noun bs Object -> Type where
    DiscardThis : DiscardOk This
    DiscardTracked : {auto 0 z : nounZone n = Just Hand} -> DiscardOk n

  ||| Which nouns take a card-type ascription — the closed table `AsType`
  ||| reads instead of carrying its scope in its row. One entry: the
  ||| SOURCE, whose sorted reading ("this creature", "this artifact") is
  ||| the only one the corpus writes. Every other noun either says its own
  ||| type in the predicate it carries or is a read whose antecedent
  ||| already fixed the head, re-sorting a read being the demonstrative's
  ||| job (`That`). The table is what keeps `AsType`'s [CR#109.2]
  ||| battlefield projection honest: the rule speaks of a description
  ||| carrying a card type and NO zone word, which is exactly what the
  ||| source under a type noun is.
  public export
  data Ascribable : Noun bs Object -> Type where
    AscribeThis : Ascribable This

  ||| A keyword tag's legal expansion body ([CR#701.8a] family): the tag
  ||| and its move agree, so no term can pair a verb's deontic identity
  ||| with another verb's motion — and each tag demands its verb's SOURCE
  ||| zone of the moved noun ([CR#701.8a] destruction moves a battlefield
  ||| permanent, [CR#701.9a] discarding a hand card; exile is zone-blind).
  ||| The demand lives on the RELATION, so raw spellings prove exactly what
  ||| the macros prove — and a forged provenance stamp is unwritable
  ||| (`badCompositeDestroyGraveyard`, `badDoesDiscardBattlefield`).
  public export
  -- The bundle is part of the INDEX, so a tagged placement declares
  -- which adverbials its verb writes. Three rows fix the empty bundle,
  -- which is what a destroy, a sacrifice and a discard write and all the
  -- corpus gives them; the exile has a second row because it writes one
  -- rider and exactly one. Leaving the bundle a free variable was the
  -- other design and it is not free: `Move` carries a `RidersFit` proof
  -- over the bundle, so abstracting the bundle drags an unsolved proof
  -- into every index.
  data TagBody : VerbName -> Effect bs -> Type where
    DestroyB : {auto 0 z : OnBattlefield (nounZone n)} ->
               {auto 0 na : NotAnyTarget n} ->
               TagBody Destroy (Move n (ZoneAt Graveyard Bare) {na})
    SacrificeB : {auto 0 z : OnBattlefield (nounZone n)} ->
                 {auto 0 na : NotAnyTarget n} ->
                 TagBody Sacrifice (Move n (ZoneAt Graveyard Bare) {na})
    ExileB : {auto 0 na : NotAnyTarget n} ->
             TagBody Exile (Move n (ZoneAt Exile Bare) {na})
    -- "exile [n] with [amt] [kind] counter(s) on it" — the one ridden
    -- placement a keyword-action tag writes. The entry participles and
    -- the controller are absent from the shape rather than refused by a
    -- gate, and the corpus is why: "exile it tapped" and "exile it under
    -- your control" are zero lines apiece, [CR#110.5b] and [CR#109.4]
    -- having nothing to say about a card in exile (`badExileTapped`).
    ExileWithCountersB : {0 amt : Amount (nomIntro n)} ->
                         {0 kind : CounterKind} ->
                         {0 wc : WrittenCount amt} ->
                         {auto 0 na : NotAnyTarget n} ->
                         TagBody Exile
                                 (Move n (ZoneAt Exile Bare) {na}
                                       {riders = MkMoveRiders [] Nothing
                                          {counters = Just (MkCounterRider amt kind {wc})}})
    DiscardB : {auto 0 d : DiscardOk n} ->
               {auto 0 na : NotAnyTarget n} ->
               TagBody Discard (Move n (ZoneAt Graveyard Bare) {na})

  ||| Verb agentivity, one table read by the rows it LACKS: an actor is
  ||| required exactly where there is no row here. The CR gives sacrifice
  ||| and discard a player actor ([CR#701.21a,701.9a]), so their tags are
  ||| absent and spell only under `Does` (`badAgentlessSacrifice`). Destroy
  ||| and exile have rows because they are actor-OPTIONAL — the bare
  ||| imperative and the subjected form are both real oracle text — so
  ||| `Does` demands nothing of the verb and only `Composite` reads this
  ||| table. A new verb declares its row or its absence, and that choice IS
  ||| the answer.
  public export
  data NonAgentive : VerbName -> Type where
    DestroyNA : NonAgentive Destroy
    ExileNA : NonAgentive Exile

  ||| A damage subject is grammatically singular, or the DISTRIBUTIVE
  ||| "each" group, which spreads the singular frame over its members
  ||| ("Each creature you control deals 1 damage to that creature." —
  ||| Case of the Gateway Express; "Each creature deals 1 damage to
  ||| its controller."). A COLLECTIVE group subject ("two target
  ||| creatures deal …") is unattested: the corpus writes the shared
  ||| verb distributively or names one source.
  public export
  damageSrcOk : {bs : Bindings} -> Noun bs Object -> Bool
  damageSrcOk (Each p) = True
  damageSrcOk n = isOne (nounPlur n)

  ||| The damage subject gate's witness form (a distinctive search
  ||| name, like `Headed` and `FightParticipant`).
  public export
  data DamageSource : Noun bs Object -> Type where
    MkDamageSource : {auto 0 ok : damageSrcOk n = True} -> DamageSource n

  ||| Retag the binding a moved noun denotes: an introducing noun's own
  ||| fresh binding, or the unique binding a read resolved to (strict
  ||| uniqueness is what makes this well-defined). The retag writes the
  ||| new zone AND the moving verb's tag (provenance — `Nothing` for an
  ||| untagged move: the participle names the LAST verb event). Player
  ||| and quality clauses are identity — unreachable from card terms
  ||| (`Move` is Object-kinded), kept explicit for totality.
  public export
  setZone : Maybe VerbName -> Maybe Zone -> Binding -> Binding
  setZone p z (MkBinding det Object plur (ObjectP ty oldZn _ og)) =
    MkBinding det Object plur (ObjectP ty z (mkStamp p oldZn) og)
  setZone p z (MkBinding det Player plur PlayerP) = MkBinding det Player plur PlayerP
  setZone p z (MkBinding det (Quality q) plur QualityP) =
    MkBinding det (Quality q) plur QualityP
  setZone p z (MkBinding det Outcome plur (OutcomeP s)) =
    MkBinding det Outcome plur (OutcomeP s)
  setZone p z (MkBinding det Gap plur GapP) = MkBinding det Gap plur GapP
  setZone p z (MkBinding det (Letter w) plur LetterP) = MkBinding det (Letter w) plur LetterP

  public export
  setZoneHead : Maybe VerbName -> Maybe Zone -> Bindings -> Bindings
  setZoneHead p z [] = []
  setZoneHead p z (b :: bs) = setZone p z b :: bs

  public export
  setZoneIt : Maybe VerbName -> Maybe Zone -> Bindings -> Bindings
  setZoneIt p z [] = []
  setZoneIt p z (MkBinding det Object OneOf (ObjectP ty zn _ og) :: bs) =
    MkBinding det Object OneOf (ObjectP ty z (mkStamp p zn) og) :: bs
  setZoneIt p z (b :: bs) = b :: setZoneIt p z bs

  public export
  setZoneThem : Maybe VerbName -> Maybe Zone -> Bindings -> Bindings
  setZoneThem p z [] = []
  setZoneThem p z (MkBinding det Object ManyOf (ObjectP ty zn _ og) :: bs) =
    MkBinding det Object ManyOf (ObjectP ty z (mkStamp p zn) og) :: bs
  setZoneThem p z (b :: bs) = b :: setZoneThem p z bs

  public export
  setZoneThose : Maybe VerbName -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneThose p w z [] = []
  setZoneThose p w z (b :: bs) =
    case (b.plur, wordNow w b) of
      (ManyOf, True) => setZone p z b :: bs
      _ => b :: setZoneThose p w z bs

  public export
  setZoneThat : Maybe VerbName -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneThat p w z [] = []
  setZoneThat p w z (b :: bs) =
    case (b.plur, wordNow w b) of
      (OneOf, True) => setZone p z b :: bs
      _ => b :: setZoneThat p w z bs

  public export
  setZoneVerbed : Maybe VerbName -> VerbName -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneVerbed p v w z [] = []
  setZoneVerbed p v w z (b :: bs) =
    if verbedMatch v w b then setZone p z b :: bs else b :: setZoneVerbed p v w z bs

  public export
  setZoneManyVerbed : Maybe VerbName -> VerbName -> NounWord -> Maybe Zone -> Bindings -> Bindings
  setZoneManyVerbed p v w z [] = []
  setZoneManyVerbed p v w z (b :: bs) =
    if verbedMatchMany v w b then setZone p z b :: bs
                             else b :: setZoneManyVerbed p v w z bs

  public export
  moveIntro : {bs : Bindings} -> {k : Kind} -> Maybe VerbName -> Noun bs k -> Maybe Zone -> Bindings
  moveIntro p nn@(Each pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(Indefinite m pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(TargetGroup q pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(AllOf pr) z = setZoneHead p z (nomIntro nn)
  -- a move of "each of [group]" retags the group, since a move of every
  -- member is a move of all of them; the determiner changes only what
  -- the clause's PER-MEMBER material may say ("return each of them to
  -- the battlefield under ITS owner's control"), never the retag. It
  -- delegates rather than repeating the four introducer branches,
  -- because the complement is a group mention either way.
  moveIntro p (EachOf grp) z = moveIntro p grp z
  moveIntro p nn@(LibrarySlice _ _ _) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(SomeOf _ _) z = setZoneHead p z (nomIntro nn)
  -- moving THE REST retags nothing, and the reason is the shape of the
  -- payload rather than a hedge: the binding it reads is the whole
  -- group's, part of which went somewhere else, and one zone field
  -- cannot hold two answers. No corpus line reads the group back after
  -- its complement moves, so the hole costs nothing that is written.
  -- It SPENDS the group besides (`groupSpent`, chapter twenty-six):
  -- the remainder is outstanding until it is placed and there is only
  -- one of it, so a second disposition on the same path names nothing.
  moveIntro p TheRest z = groupSpent bs
  moveIntro p It z = setZoneIt p z bs
  moveIntro p Them z = setZoneThem p z bs
  moveIntro p (That w) z = setZoneThat p w z bs
  moveIntro p (Those w) z = setZoneThose p w z bs
  moveIntro p (TheVerbed v w) z = setZoneVerbed p v w z bs
  moveIntro p (ThoseVerbed v w) z = setZoneManyVerbed p v w z bs
  moveIntro p This z = bs
  -- a moved attachment host mints NOTHING, which is `This`'s answer and
  -- not `AsType`'s: the sorted self mints because [CR#400.7] makes the
  -- moved object new and Flickering Spirit reads it back, and no line
  -- reads a moved host back at all -- an Aura that moves its own host
  -- stops being attached to it, so there is nothing for a later clause to
  -- pick up. Recorded rather than minted.
  moveIntro p (AttachHost _ _) z = bs
  -- a moved sorted self-reference mints the new object's binding
  -- ([CR#400.7]; see the constructor comment). The stamp's at-verb
  -- frame stays conservatively False — an ascribed-self cost participle TYPE
  -- word waits on a corpus witness, so the pre-move zone is passed as
  -- untracked here rather than read off `nounZone`.
  moveIntro p (AsType t n) z = MkBinding TheD Object OneOf (ObjectP (Just t) z (mkStamp p Nothing) Nothing) :: bs
  moveIntro p You z = bs
  moveIntro p (PlayerGroup _) z = bs
  moveIntro p They z = bs
  moveIntro p (ControllerOf n) z = nomIntro (ControllerOf n)
  moveIntro p (OwnerOf n) z = nomIntro (OwnerOf n)

  ||| The zone a noun's referent currently occupies, if tracked: reads
  ||| consult their unique binding, introducers their seed zone
  ||| ([CR#109.2] — a bare description means the battlefield), the player
  ||| nouns are untracked. The SORTED self-reference is a description that
  ||| includes a card type, so [CR#109.2] places it on the battlefield
  ||| exactly as it places "target creature" there; bare `This` is the
  ||| source as an object and stays untracked. The one counted mention that
  ||| takes no default is the class word: "any target" is not a description
  ||| of an object but the NAME of [CR#115.4]'s damage class, which spans
  ||| players, so [CR#109.2] has nothing to place and the phrase projects
  ||| no zone — which is how every battlefield-demanding verb comes to
  ||| refuse it through the ordinary gate (`badDestroyAnyTarget`,
  ||| `badTapAnyTarget`), damage taking it by its own recipient row.
  public export
  nounZone : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe Zone
  nounZone This = Nothing
  nounZone (AsType t n) = Just Battlefield
  nounZone You = Nothing
  nounZone (PlayerGroup _) = Nothing
  nounZone (Each p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (Indefinite m p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (TargetGroup q p) =
    if headIsAnyTarget p then Nothing else Just (zoneOr Battlefield (seedZone p))
  nounZone (AllOf p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (EachOf grp) = nounZone grp
  nounZone (LibrarySlice _ _ _) = Just Library
  nounZone (SomeOf _ grp) = nounZone grp
  nounZone TheRest = zoneOfGroup bs
  nounZone It = zoneOfIt bs
  nounZone They = Nothing
  nounZone Them = zoneOfThem bs
  nounZone (That w) = zoneOfThat w bs
  nounZone (AttachHost _ h) = attachHostZone h
  nounZone (Those w) = zoneOfThose w bs
  nounZone (TheVerbed v w) = zoneOfVerbed v w bs
  nounZone (ThoseVerbed v w) = zoneOfManyVerbed v w bs
  nounZone (ControllerOf n) = Nothing
  nounZone (OwnerOf n) = Nothing

  ||| The projected head type a noun's referent carries, if any —
  ||| introducers project their predicate's head, reads consult their
  ||| unique binding, the sorted self-reference names its own. What
  ||| verb slots that demand a type (fight takes creatures) consult.
  public export
  nounTy : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe CardType
  nounTy This = Nothing
  nounTy (AsType t n) = Just t
  nounTy You = Nothing
  nounTy (PlayerGroup _) = Nothing
  nounTy (Each p) = seedTy p
  nounTy (Indefinite m p) = seedTy p
  nounTy (TargetGroup q p) = seedTy p
  nounTy (AllOf p) = seedTy p
  nounTy (EachOf grp) = nounTy grp
  -- a position describes no card ([CR#401.2]) -- see the constructor.
  nounTy (LibrarySlice _ _ _) = Nothing
  nounTy (SomeOf _ grp) = nounTy grp
  nounTy TheRest = tyOfGroup bs
  nounTy It = tyOfIt bs
  nounTy They = Nothing
  nounTy Them = tyOfThem bs
  nounTy (That w) = tyOfThat w bs
  nounTy (AttachHost _ h) = attachHostTy h
  nounTy (Those w) = tyOfThose w bs
  nounTy (TheVerbed v w) = tyOfVerbed v w bs
  nounTy (ThoseVerbed v w) = tyOfManyVerbed v w bs
  nounTy (ControllerOf n) = Nothing
  nounTy (OwnerOf n) = Nothing

  ||| The grammatical number a noun phrase carries — what singular
  ||| reads (a possessive amount, a relational noun) demand of their
  ||| argument.
  public export
  nounPlur : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Plurality
  nounPlur This = OneOf
  nounPlur (AsType t n) = nounPlur n
  nounPlur You = OneOf
  nounPlur (PlayerGroup _) = ManyOf
  nounPlur (Each p) = ManyOf
  nounPlur (Indefinite m p) = OneOf
  nounPlur (TargetGroup q p) = quantPlur q
  nounPlur (AllOf p) = ManyOf
  nounPlur (EachOf grp) = ManyOf
  nounPlur (LibrarySlice _ amt _) = amtPlur amt
  nounPlur (SomeOf q _) = quantPlur q
  -- the complement is plural by its word: a group with one member left
  -- writes "the other" instead, which is the same relation under a
  -- different number and is shut for the reason finding 111 gave (a
  -- binding carries no cardinality, so the remainder's number cannot be
  -- computed). Ledgered with its count.
  nounPlur TheRest = ManyOf
  nounPlur It = OneOf
  nounPlur They = OneOf
  nounPlur Them = ManyOf
  nounPlur (That w) = OneOf
  nounPlur (AttachHost _ _) = OneOf
  nounPlur (Those w) = ManyOf
  nounPlur (TheVerbed v w) = OneOf
  nounPlur (ThoseVerbed v w) = ManyOf
  nounPlur (ControllerOf n) = OneOf
  nounPlur (OwnerOf n) = OneOf

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = outcomeB DamageDealt :: nomIntro to
  effIntro (Fights a b) = nomIntro b
  effIntro (Tap n) = nomIntro n
  effIntro (DoesntUntapNext n _) = nomIntro n
  effIntro (Untap n) = nomIntro n
  effIntro (ToFace _ n) = nomIntro n
  effIntro (Phases _ n) = nomIntro n
  effIntro (GetsCounters who amt _) = amtIntro amt
  effIntro (LosesAllCounters who _) = nomIntro who
  effIntro (RemoveFromCombat n) = nomIntro n
  effIntro (TakesDesignation who _) = nomIntro who
  effIntro (Goad n) = nomIntro n
  effIntro (BecomesTime _) = bs
  effIntro (Concludes _ who) = nomIntro who
  effIntro GameDrawn = bs
  -- The countering announces its patient and nothing else: [CR#701.6a]
  -- puts the countered spell into a graveyard, but the sentence never
  -- writes that move, so there is no retag for this clause to record.
  effIntro (CounterSpell what) = nomIntro what
  effIntro (Choose n) = nomIntro n
  effIntro (Move what to) = moveIntro Nothing what (Just (zoneSort to))
  effIntro (ChangeLife who (Up a)) = outcomeB LifeGained :: lifeIntro (Up a)
  effIntro (ChangeLife who (Down a)) = outcomeB LifeLost :: lifeIntro (Down a)
  -- no outcome mention: which direction a set-to went is not a fact the
  -- sentence states (see `LifeOp`).
  effIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  -- the subject and whatever its count read, and nothing else: the drawn
  -- card is not a mention (see `Draw`), and no OUTCOME binding either —
  -- `ThatMuch` reads a magnitude an earlier clause produced, and no
  -- corpus line reads a draw's ("draw that many cards" reads a count
  -- from somewhere else, never from a draw).
  effIntro (Draw who amt) = amtIntro amt
  -- exposing moves nothing ([CR#701.20b]), so the group is contributed
  -- exactly as the phrase wrote it -- still in the library, which is what
  -- the following placement moves it out of.
  effIntro (Expose v who what) = exposedIntro what
  -- the milled group, in the graveyard it was put into ([CR#701.17a]);
  -- public, so [CR#701.17c] lets later text find it. No head type: a mill
  -- takes cards off the top of a hidden pile and says nothing about them,
  -- the same silence `LibrarySlice` keeps.
  -- Its NUMBER is the clause's, which is the count and the SUBJECT
  -- together and not the count alone (`outputPlur`) -- the same derivation
  -- `Create` beside it uses: [CR#701.17a] has each milled-at player put
  -- that many cards into their own graveyard, so "each player mills a
  -- card" puts one card per player into the graveyards and the mention is
  -- plural even though the count says one (`badDistributedMillSingular`).
  effIntro (Mill who amt) =
    MkBinding TheD Object (outputPlur (nounPlur who) (amtPlur amt))
              (ObjectP Nothing (Just Graveyard) Nothing Nothing)
      :: amtIntro amt
  -- the found object, in the zone it was found in ([CR#701.23a]) -- the
  -- "it"/"that card" every search sentence goes on to place.
  effIntro (Search who z p) =
    MkBinding AD Object OneOf (ObjectP (seedTy p) (Just (zoneSort z)) Nothing Nothing)
      :: (predDelta p ++ nomIntro who)
  effIntro (Shuffle whose) = shuffledAway (nomIntro whose)
  effIntro (Continuously se _) = staticIntro se
  -- the created token enters the discourse as the indefinite mention its
  -- phrase is ("a … token"), on the battlefield ([CR#111.1]), under the
  -- head its type line writes, and with the number the CLAUSE writes --
  -- the count and the AGENT together, not the count alone. Elephant
  -- Resurgence reads the distributed one ("Each player creates a green
  -- Elephant creature token. Those creatures have …"), where the count is
  -- one and the mention is plural because the agent is (`outputPlur`).
  effIntro (Create agent count tok riders) =
    MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
              (ObjectP (tokenHeadTy tok) (Just Battlefield) Nothing (Just TokenOrigin))
      :: amtIntro count
  effIntro (PutCounters amt kind on) = nomIntro on
  -- the division contributes its GROUP, which is what the sentence
  -- after it reads ("Distribute two +1/+1 counters among up to two
  -- target creatures. They gain trample until end of turn."), and the
  -- damage row contributes the damage outcome beside it exactly as the
  -- undivided verb does.
  effIntro (Distribute (DividedDamage _) amt among) = outcomeB DamageDealt :: nomIntro among
  effIntro (Distribute (DistributedCounters _) amt among) = nomIntro among
  effIntro (RemoveCounters amt kind from) = nomIntro from
  effIntro (Composite v (Move what to)) = moveIntro (Just v) what (Just (zoneSort to))
  effIntro (Composite _ e) = effIntro e
  effIntro (Does s v (Move what to)) = moveIntro (Just v) what (Just (zoneSort to))
  effIntro (Does s v e) = effIntro e
  effIntro (Pay who c) = costIntro c
  effIntro (May d body did notd) = mayIntro body did notd
  -- NEITHER arm contributes: only one of them ever runs. The else arm was
  -- always a hole (it REPLACES the main line, `mayIntro`'s cut one
  -- construction over) and the conditioned clause is now one too, for the
  -- identical reason read the other way -- the condition may have been
  -- FALSE, and then the clause never happened and its phrase named
  -- nothing (`badConditionalArmAntecedent`). This row used to export
  -- `effIntro e`, which made a conditionally created token an
  -- unconditional `It` for every sentence after it. What survives the
  -- branch is the discourse that entered it, plus the condition's own
  -- contribution (nothing, on every row).
  -- The COST this row was named as carrying is PAID, and not here: a
  -- target announced inside the conditioned clause is announced whatever
  -- the condition's truth ([CR#601.2c]), so Overload's second sentence
  -- really does read the first sentence's "that artifact" across an `If`.
  -- That channel is `preIntro`'s. The half still owed is the LEADING
  -- conditional that announces a target its own consequent reads (Blood
  -- Lust; `badMatchesTargetSubject`), which needs a delta carrying a
  -- phrase's target half and nothing else (ledger).
  effIntro (If e c oth) = condDelta c ++ bs
  effIntro (WhereLetter _ def body) = effIntro body
  effIntro (Sequentially es) = effsIntro es
  -- the batch has RESOLVED by the time the next sentence reads it, so
  -- its discourse is its last element's — which over the announcement
  -- telescope carries every earlier element's phrases with it.
  effIntro (Simultaneously es) = simIntro es
  -- a modal names NOBODY the sentences after it can read: the modes are
  -- chosen at cast ([CR#700.2a]) and an unchosen one's targets are never
  -- announced ([CR#700.2c]), so no mode's phrase is guaranteed to have
  -- named anything, and Blood on the Snow writes a description where the
  -- anaphor would go rather than take the risk.
  effIntro (Modal q modes) = bs
  effIntro (Delayed ev e) = bs               -- a future clause mentions nothing NOW
  -- exactly one of the two clauses ran and no sentence after can know
  -- which, which is `mayIntro`'s cut at a second site — so neither
  -- clause's OUTCOME survives. What does survive is what the replaced
  -- clause's phrases announced ([CR#601.2c] announces them whatever
  -- resolution makes of the event, and [CR#614.6] is what makes the
  -- outcome unavailable: the replaced event never happened).
  -- the enclosure's discourse and NONE of the trigger's: the reflexive
  -- ability goes on the stack "the next time a player would receive
  -- priority" ([CR#603.3]), so the rest of this resolution finishes
  -- before it resolves and cannot mention what it will do
  -- (`badAfterReflexiveReadsTrigger`). `Delayed`'s "a future clause
  -- mentions nothing NOW" with a clause in front of it that DID run.
  effIntro (Reflexively body trig) = effIntro body
  effIntro (InsteadOf replaced repl) = annIntro replaced
  -- the undo is scheduled on an event that has not happened, so what
  -- this clause leaves behind is not settled yet: the exiled object may
  -- be in exile or back on the battlefield when a later sentence reads
  -- it. The announcement is all that is safe ([CR#601.2c]).
  effIntro (HeldUntil e ev) = annIntro e

  ||| What a clause has ANNOUNCED by the time its own trailing condition is
  ||| read — the pre-resolution twin of `effIntro`, and the whole of
  ||| chapter twenty-one's answer to the divergence `If` carried since
  ||| chapter eighteen: the condition is WRITTEN after the clause and
  ||| EVALUATED before it, so typing it in the clause's post-state let it
  ||| ask about a world the clause had not made yet. "Destroy target
  ||| creature if it's in a graveyard" typechecked exactly because the
  ||| destroy had already retagged its target (`badTrailingPostStateZone`).
  |||
  ||| Two kinds of row differ from `effIntro`, and both are the same
  ||| distinction: what a clause's PHRASES name is announced as the clause
  ||| is written ([CR#601.2c] for targets, and the choice and recipient
  ||| phrases with them), so it is readable; what the clause DOES is not.
  ||| So the three zone-writing rows announce their object without the
  ||| retag or the verb stamp, `Create` announces no token ([CR#111.1] — a
  ||| token is put onto the battlefield by an EFFECT), and the two
  ||| event-outcome rows leave no outcome behind. Every other row is its
  ||| `effIntro` answer verbatim, written out rather than delegated so that
  ||| a new clause has to declare its own pre-state.
  public export
  preIntro : {bs : Bindings} -> Effect bs -> Bindings
  preIntro (DealDamage src amt to) = nomIntro to
  preIntro (Distribute v amt among) = nomIntro among
  preIntro (Fights a b) = nomIntro b
  preIntro (Tap n) = nomIntro n
  preIntro (DoesntUntapNext n _) = nomIntro n
  preIntro (Untap n) = nomIntro n
  preIntro (ToFace _ n) = nomIntro n
  preIntro (Phases _ n) = nomIntro n
  preIntro (GetsCounters who amt _) = amtIntro amt
  preIntro (LosesAllCounters who _) = nomIntro who
  preIntro (RemoveFromCombat n) = nomIntro n
  preIntro (TakesDesignation who _) = nomIntro who
  preIntro (Goad n) = nomIntro n
  preIntro (BecomesTime _) = bs
  preIntro (Concludes _ who) = nomIntro who
  preIntro GameDrawn = bs
  preIntro (CounterSpell what) = nomIntro what
  preIntro (Choose n) = nomIntro n
  preIntro (Move what to) = nomIntro what
  preIntro (ChangeLife who (Up a)) = lifeIntro (Up a)
  preIntro (ChangeLife who (Down a)) = lifeIntro (Down a)
  preIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  preIntro (Draw who amt) = amtIntro amt
  -- an exposure ANNOUNCES its complement's phrase like any other clause;
  -- the group it names is the phrase's own, not something the clause did.
  preIntro (Expose v who what) = exposedIntro what
  -- the milled group is what the clause DID (it exists only once cards
  -- have moved), so the pre-state has only the count's reads.
  preIntro (Mill who amt) = amtIntro amt
  -- likewise the found object: nothing is found until the search runs.
  preIntro (Search who z p) = predDelta p ++ nomIntro who
  preIntro (Shuffle whose) = nomIntro whose
  preIntro (Continuously se _) = staticIntro se
  preIntro (Create agent count tok riders) = amtIntro count
  preIntro (PutCounters amt kind on) = nomIntro on
  preIntro (RemoveCounters amt kind from) = nomIntro from
  preIntro (Composite v (Move what to)) = nomIntro what
  preIntro (Composite _ e) = preIntro e
  preIntro (Does s v (Move what to)) = nomIntro what
  preIntro (Does s v e) = preIntro e
  preIntro (Pay who c) = nomIntro who
  preIntro (May d body did notd) = mayIntro body did notd
  -- THE ANNOUNCEMENT CHANNEL, chapter twenty-five's. This row used to be
  -- `condDelta c ++ bs` -- `effIntro`'s answer copied -- and the copy is
  -- what made Overload's second sentence unwritable. A conditioned clause
  -- is a hole for what it DID (finding 100: the condition may have been
  -- false, so it never ran) and that is `effIntro`'s row, unchanged. What
  -- it announced is another matter: [CR#601.2c] chooses targets as the
  -- spell is cast, "whatever clause spells them", and no condition is
  -- checked then -- so "Destroy target artifact if its mana value is 2 or
  -- less" really has announced an artifact by the time the next sentence
  -- says "that artifact", true condition or false.
  -- The channel is `annIntro`'s now, chapter twenty-six: `preIntro` was
  -- never announcement-only, so copying it here exported a nested may's
  -- deed and a nested sequence's outcome along with the targets
  -- (`badConditionalInsteadReadsMayOutcome`).
  preIntro (If e c oth) = condDelta c ++ annIntro e
  preIntro (WhereLetter _ def body) = preIntro body
  preIntro (Sequentially es) = preIntros es
  preIntro (Simultaneously es) = simPres es
  preIntro (Modal q modes) = bs
  preIntro (Delayed ev e) = bs
  -- the announcement of the clause that was replaced — the replacement
  -- may have announced phrases of its own, but only one of the two ran
  -- and the one that certainly did not is the replaced clause's EVENT,
  -- not its phrases ([CR#614.6] against [CR#601.2c]).
  preIntro (Reflexively body trig) = preIntro body
  preIntro (InsteadOf replaced repl) = annIntro replaced
  preIntro (HeldUntil e ev) = annIntro e

  ||| THE ANNOUNCEMENT CHANNEL — chapter twenty-six's, and the whole of
  ||| this chapter's answer to a distinction chapters twenty-one through
  ||| twenty-five kept borrowing `preIntro` for. What a clause's PHRASES
  ||| have named by the time the spell is cast ([CR#601.2c] announces every
  ||| target as the spell is cast, whatever resolution later makes of it)
  ||| and nothing whatever about what the clause DID.
  |||
  ||| `preIntro` is not that function and never was. It is the broader
  ||| PRE-RESOLUTION summary ordinary sequential prose needs — the twin of
  ||| `effIntro` one moment earlier — and on a FLAT clause the two agree
  ||| exactly, which is why three constructions could type their
  ||| announcement-only slots on it and look right. They came apart on
  ||| composites, in three directions and all of them measured: a `May`'s
  ||| row is `mayIntro`, so the optional body's DEED crossed into a
  ||| simultaneous sibling that had not seen it happen
  ||| (`badSimultaneousReadsMayDeed`); a `Sequentially`'s row is its last
  ||| clause's pre-state over the DEED telescope, so an earlier step's
  ||| outcome reached a replacement for an event [CR#614.6] says never
  ||| happened (`badInsteadReadsReplacedSequenceOutcome`); and `If`
  ||| forwarded either of them recursively.
  |||
  ||| So the rows differ from `preIntro` in exactly the places a clause
  ||| CONTAINS another clause: `May` announces its BODY's phrases and
  ||| nothing its arms did, `If` its conditioned clause's, `InsteadOf` and
  ||| `HeldUntil` the clause they wrap. A `Sequentially` is a hole — not a
  ||| hedge, a consequence of the telescope: its elements are typed over
  ||| each other's `effIntro`, so no later element's announcement can be
  ||| lifted out without the deeds it was typed in. A BATCH is not, because
  ||| its telescope is this function's (`annSims`).
  public export
  annIntro : {bs : Bindings} -> Effect bs -> Bindings
  annIntro (DealDamage src amt to) = nomIntro to
  annIntro (Distribute v amt among) = nomIntro among
  annIntro (Fights a b) = nomIntro b
  annIntro (Tap n) = nomIntro n
  annIntro (DoesntUntapNext n _) = nomIntro n
  annIntro (Untap n) = nomIntro n
  annIntro (ToFace _ n) = nomIntro n
  annIntro (Phases _ n) = nomIntro n
  annIntro (GetsCounters who amt _) = amtIntro amt
  annIntro (LosesAllCounters who _) = nomIntro who
  annIntro (RemoveFromCombat n) = nomIntro n
  annIntro (TakesDesignation who _) = nomIntro who
  annIntro (Goad n) = nomIntro n
  annIntro (BecomesTime _) = bs
  annIntro (Concludes _ who) = nomIntro who
  annIntro GameDrawn = bs
  annIntro (CounterSpell what) = nomIntro what
  annIntro (Choose n) = nomIntro n
  annIntro (Move what to) = nomIntro what
  annIntro (ChangeLife who (Up a)) = lifeIntro (Up a)
  annIntro (ChangeLife who (Down a)) = lifeIntro (Down a)
  annIntro (ChangeLife who (Set a)) = lifeIntro (Set a)
  annIntro (Draw who amt) = amtIntro amt
  annIntro (Expose v who what) = exposedIntro what
  annIntro (Mill who amt) = amtIntro amt
  annIntro (Search who z p) = predDelta p ++ nomIntro who
  annIntro (Shuffle whose) = nomIntro whose
  annIntro (Continuously se _) = staticIntro se
  annIntro (Create agent count tok riders) = amtIntro count
  annIntro (PutCounters amt kind on) = nomIntro on
  annIntro (RemoveCounters amt kind from) = nomIntro from
  annIntro (Composite v (Move what to)) = nomIntro what
  annIntro (Composite _ e) = annIntro e
  annIntro (Does s v (Move what to)) = nomIntro what
  annIntro (Does s v e) = annIntro e
  -- the BODY's phrases, on every row. [CR#601.2c] announces a target
  -- written inside an optional clause whether or not the player takes the
  -- offer; what the body DID is `mayIntro`'s business. The ARMS contribute
  -- nothing here, and the reason is structural: an arm is typed over the
  -- body's `effIntro`, so its own announcement cannot be taken without the
  -- body's deeds coming with it. Under-reporting, in the safe direction.
  annIntro (Pay who c) = nomIntro who
  annIntro (May d body did notd) = annIntro body
  annIntro (If e c oth) = condDelta c ++ annIntro e
  -- the rider is TRANSPARENT to the discourse and keeps its letter in it:
  -- the body's announcements are the clause's, and the letter stays
  -- readable through the rest of the ability ([CR#107.3i]).
  annIntro (WhereLetter _ def body) = annIntro body
  annIntro (Sequentially es) = bs
  annIntro (Simultaneously es) = annSims es
  annIntro (Modal q modes) = bs
  annIntro (Delayed ev e) = bs
  annIntro (Reflexively body trig) = annIntro body
  annIntro (InsteadOf replaced repl) = annIntro replaced
  annIntro (HeldUntil e ev) = annIntro e

  ||| A BATCH's announcements: the telescope's own end. `SimEffects`
  ||| threads this function, so walking to the last element reaches
  ||| every element's phrases at once — the accumulation `effsIntro`
  ||| performs for deeds, performed for announcements.
  public export
  annSims : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  annSims [] = bs
  annSims (e :: es) = annSims es

  ||| What a clause contributes BEYOND its announcement — the deed half of
  ||| `effIntro`, written as a delta so a batch can fold every element's in
  ||| (`simIntro`). Chapter twenty-two folded only the LAST element's
  ||| `effIntro` and recorded the simplification; the general case is the
  ||| union, and the case that shows it is two deeds of one kind — "create
  ||| a token" beside "create a token" leaves TWO tokens ([CR#608.2f]
  ||| processes a batch's actions simultaneously, so both happened), and a
  ||| following "it" must refuse as ambiguous rather than silently pick the
  ||| last (`badBatchTwoCreatesThenIt`, `badBatchTwoOutcomesThenThatMuch`).
  |||
  ||| It is the DELTA and not the whole answer because the announcement
  ||| telescope already carries each element's phrases: `effIntro`'s answer
  ||| folded whole would count them twice, and a doubled mention breaks the
  ||| uniqueness gates every read makes.
  |||
  ||| Two kinds of row answer `[]` for a reason worth stating. A clause
  ||| that RETAGS (the three zone-writing rows) or one that REMOVES (a
  ||| shuffle) does not add a binding at all, so it has no delta; the reads
  ||| INSIDE such a batch are refused anyway (`badSimultaneousReadsRetag`).
  ||| And a CONTAINER answers `[]` because what it leaves behind is not a
  ||| delta over its own announcement: a may's arms and a conditional's
  ||| branches are `mayIntro`'s cut, a sequence and a batch are refused as
  ||| batch elements outright (`NotSeq`, `NotSim`), and a replacement
  ||| contributes its announcement and no outcome by [CR#614.6].
  public export
  deedDelta : {bs : Bindings} -> Effect bs -> List Binding
  deedDelta (DealDamage src amt to) = [outcomeB DamageDealt]
  deedDelta (Distribute (DividedDamage _) amt among) = [outcomeB DamageDealt]
  deedDelta (Distribute (DistributedCounters _) amt among) = []
  deedDelta (Fights a b) = []
  deedDelta (Tap n) = []
  deedDelta (DoesntUntapNext _ _) = []
  deedDelta (Untap n) = []
  deedDelta (ToFace _ _) = []
  deedDelta (Phases _ _) = []
  deedDelta (GetsCounters _ _ _) = []
  deedDelta (LosesAllCounters _ _) = []
  deedDelta (RemoveFromCombat _) = []
  deedDelta (TakesDesignation _ _) = []
  deedDelta (Goad _) = []
  deedDelta (BecomesTime _) = []
  deedDelta (Concludes _ _) = []
  deedDelta GameDrawn = []
  deedDelta (CounterSpell _) = []
  deedDelta (Choose n) = []
  deedDelta (Move what to) = []
  deedDelta (ChangeLife who (Up a)) = [outcomeB LifeGained]
  deedDelta (ChangeLife who (Down a)) = [outcomeB LifeLost]
  deedDelta (ChangeLife who (Set a)) = []
  deedDelta (Draw who amt) = []
  deedDelta (Expose v who what) = []
  deedDelta (Mill who amt) =
    [MkBinding TheD Object (outputPlur (nounPlur who) (amtPlur amt))
               (ObjectP Nothing (Just Graveyard) Nothing Nothing)]
  deedDelta (Search who z p) =
    [MkBinding AD Object OneOf (ObjectP (seedTy p) (Just (zoneSort z)) Nothing Nothing)]
  deedDelta (Shuffle whose) = []
  deedDelta (Continuously se _) = []
  deedDelta (Create agent count tok riders) =
    [MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
               (ObjectP (tokenHeadTy tok) (Just Battlefield) Nothing (Just TokenOrigin))]
  deedDelta (PutCounters amt kind on) = []
  deedDelta (RemoveCounters amt kind from) = []
  deedDelta (Composite v (Move what to)) = []
  deedDelta (Composite _ e) = deedDelta e
  deedDelta (Does s v (Move what to)) = []
  deedDelta (Does s v e) = deedDelta e
  deedDelta (Pay who c) = []
  deedDelta (May d body did notd) = []
  deedDelta (If e c oth) = []
  deedDelta (WhereLetter _ _ _) = []
  deedDelta (Sequentially es) = []
  deedDelta (Simultaneously es) = []
  deedDelta (Modal q modes) = []
  deedDelta (Delayed ev e) = []
  deedDelta (Reflexively body trig) = deedDelta body
  deedDelta (InsteadOf replaced repl) = []
  deedDelta (HeldUntil e ev) = []

  ||| A sequence's pre-state is its LAST clause's: every earlier clause
  ||| has resolved by the time the trailing condition is read.
  public export
  preIntros : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  preIntros [] = bs
  preIntros (e :: []) = preIntro e
  preIntros (e :: es) = preIntros es

  ||| The context a may's BODY and its declined arm are typed in. An
  ||| offered may writes its decider first, so the body reads it ("Target
  ||| opponent may sacrifice a creature") and so does the "if they don't"
  ||| arm reaching back to the offerer. The MANDATORY twin writes no
  ||| decider at all, so there is nothing extra to read and the body is
  ||| typed where the sentence stands — the whole difference between the
  ||| two [CR#118.12] shapes, in one function.
  public export
  mayCtx : {bs : Bindings} -> Maybe (Noun bs Player) -> Bindings
  mayCtx Nothing = bs
  mayCtx (Just d) = nomIntro d

  ||| What a may-clause leaves behind: the MAIN LINE's discourse — the
  ||| body's, or the if-you-do arm's when that arm is the only branch,
  ||| since it continues the body rather than replacing it.
  |||
  ||| The if-you-DON'T arm contributes nothing, and the principle is the
  ||| one English marks: the arm that continues the main line flows out,
  ||| the arm that REPLACES it is a hole. "If you don't" is written
  ||| precisely to mark the departure, and only one of the two ever
  ||| happens, so a mention inside it names nobody the sentences after the
  ||| may can read back — `predDelta (Or _) = []` one layer up. The body's
  ||| own mentions flow out even though the may may be declined: a declined
  ||| may skips at runtime, not in scope, and Through the Breach reads the
  ||| body's creature in its very next sentence.
  |||
  ||| BOTH arms at once is the fourth row: the taken arm flows out only
  ||| when there is no declined arm beside it. Crovax the Cursed writes the
  ||| pair, and [CR#118.12] is why the join is the BODY and not either arm
  ||| — the branch records whether the player chose to pay, "regardless of
  ||| what events actually occurred", so exactly one arm ran and the
  ||| sentences after the may cannot know which (`badBothArmsAntecedent`).
  public export
  mayIntro : {bs : Bindings} -> (body : Effect bs) ->
             Maybe (Effect (effIntro body)) -> Maybe (Effect bs) -> Bindings
  mayIntro body Nothing Nothing = effIntro body
  mayIntro body (Just did) Nothing = effIntro did
  mayIntro body Nothing (Just notd) = effIntro body
  mayIntro body (Just did) (Just notd) = effIntro body

  ||| What a REFLEXIVE trigger's body reads ([CR#603.12]): the enclosure's
  ||| whole post-state, with its targets settled. Both halves are the
  ||| construction's content.
  |||
  ||| The POST-state, where `InsteadOf` beside it takes only the
  ||| announcement: [CR#614.6] makes a replaced event never happen, so that
  ||| node's replacement has no outcome to read, and this one fires
  ||| precisely BECAUSE the action was taken — [CR#603.12] triggers it
  ||| "based on whether the trigger event or events occurred earlier during
  ||| the resolution". So the sacrificed creature is in its graveyard when
  ||| the body reads it (`badReflexiveTapsSacrificed`), and "that much"
  ||| finds the outcome `badInsteadReadsReplacedOutcome` refuses.
  |||
  ||| SETTLED, for `delayedCtx`'s reason at a second site: this is its own
  ||| object on the stack ([CR#603.3]) choosing its own targets there
  ||| ([CR#603.3d]), so a target the enclosing spell announced is a
  ||| particular by the time this body speaks of it.
  public export
  reflexCtx : {bs : Bindings} -> Effect bs -> Bindings
  reflexCtx body = settleTargets (effIntro body)

  ||| What a whole sequence contributes: its last clause's discourse,
  ||| the telescope having threaded every predecessor's through.
  public export
  effsIntro : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  effsIntro [] = bs
  effsIntro (e :: es) = effsIntro es

  ||| What a whole BATCH contributes: EVERY element's deed over the
  ||| announcement telescope — the union, chapter twenty-six's correction
  ||| to chapter twenty-two's recorded simplification. It used to be the
  ||| LAST element's `effIntro`, which was the union for every row the
  ||| container reached then and is not the union in general: two creates
  ||| in one batch leave two tokens, and reading the deed off the end made
  ||| the second one the only one there was ([CR#608.2f]: both actions are
  ||| processed, so both happened). The deeds go in as DELTAS
  ||| (`deedDelta`) because the telescope has already carried each
  ||| element's announced phrases.
  public export
  simIntro : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  simIntro [] = bs
  simIntro (e :: es) = deedDelta e ++ simIntro es

  ||| A batch's PRE-state: what its elements have announced and nothing
  ||| they did — the last element's own `preIntro` over the same
  ||| telescope, which is `preIntros` one container over.
  public export
  simPres : {bs : Bindings} -> {0 n : Nat} -> SimEffects n bs -> Bindings
  simPres [] = bs
  simPres (e :: []) = preIntro e
  simPres (e :: es) = simPres es

  ||| When an ability may be activated ([CR#602.5d,602.5e]). A CLOSED pair
  ||| — the CR names exactly two "Activate only as a [card type]"
  ||| restrictions and both are printed — so both rows stand though only
  ||| the first has a bench witness. `Nothing` on the slot is the
  ||| unrestricted default, which is instant speed ([CR#117.1b] — "a player
  ||| may activate an activated ability any time they have priority"); the
  ||| settled Idris spec spells that default as an `AsInstant` value
  ||| instead, and the difference is only where the absence is written.
  |||
  ||| What is NOT here is the turn-part window ("Activate only during your
  ||| upkeep", core's `Timing::DuringTurn`/`DuringStep`). Its vocabulary
  ||| nearly exists — `TurnPart` and `Whose` are chapter seventeen's — and
  ||| the line that breaks it is exact: "Activate only during any upkeep
  ||| step" writes a possessor `Whose` has no word for, neither yours nor a
  ||| named player's. Measured and ledgered.
  ||| WHICH turn-part window an activation restriction names -- the
  ||| second reader of the part-and-possessor grid, keyed like `partUse`
  ||| and answering differently, which is why the two tables both exist.
  ||| Full rows over (part x possession).
  |||
  ||| The differences from the header table are the whole content: this
  ||| reader opens `AnOpponents`, which the header writes ZERO times ("At
  ||| the beginning of an opponent's upkeep" is unwritten while "Activate
  ||| only during an opponent's turn" is three lines), and it opens the
  ||| bare TURN, which no header names at all -- 68 lines of "Activate only
  ||| during your turn" against a `partUse` row that is `PartUnattested`
  ||| throughout. The header reader in turn opens six parts this one does
  ||| not.
  ||| It also resolves the file's own named unspellable: "Activate only
  ||| during any upkeep step" was recorded as writing "a possessor `Whose`
  ||| has no word for", and `EachPlayers` is that word. The window writes
  ||| the quantifier as "any" where the header writes "each" or drops it,
  ||| which is a determiner variant of one cell and not a cell of its own.
  public export
  windowOk : TurnPart -> Maybe Owner -> Bool
  windowOk Turn Nothing = False
  windowOk Turn (Just Yours) = True
  windowOk Turn (Just ThatPlayers) = False
  windowOk Turn (Just EachPlayers) = False
  windowOk Turn (Just EachOpponents) = False
  windowOk Turn (Just EachYours) = False
  windowOk Turn (Just AnOpponents) = True
  windowOk Upkeep Nothing = False
  windowOk Upkeep (Just Yours) = True
  windowOk Upkeep (Just ThatPlayers) = False
  windowOk Upkeep (Just EachPlayers) = True
  windowOk Upkeep (Just EachOpponents) = False
  windowOk Upkeep (Just EachYours) = False
  windowOk Upkeep (Just AnOpponents) = True
  windowOk EndStep Nothing = False
  windowOk EndStep (Just Yours) = False
  windowOk EndStep (Just ThatPlayers) = False
  windowOk EndStep (Just EachPlayers) = False
  windowOk EndStep (Just EachOpponents) = False
  windowOk EndStep (Just EachYours) = False
  windowOk EndStep (Just AnOpponents) = False
  windowOk Combat Nothing = True
  windowOk Combat (Just Yours) = False
  windowOk Combat (Just ThatPlayers) = False
  windowOk Combat (Just EachPlayers) = False
  windowOk Combat (Just EachOpponents) = False
  windowOk Combat (Just EachYours) = False
  windowOk Combat (Just AnOpponents) = False
  windowOk UntapStep Nothing = False
  windowOk UntapStep (Just Yours) = False
  windowOk UntapStep (Just ThatPlayers) = False
  windowOk UntapStep (Just EachPlayers) = False
  windowOk UntapStep (Just EachOpponents) = False
  windowOk UntapStep (Just EachYours) = False
  windowOk UntapStep (Just AnOpponents) = False
  windowOk EndOfCombat Nothing = False
  windowOk EndOfCombat (Just Yours) = False
  windowOk EndOfCombat (Just ThatPlayers) = False
  windowOk EndOfCombat (Just EachPlayers) = False
  windowOk EndOfCombat (Just EachOpponents) = False
  windowOk EndOfCombat (Just EachYours) = False
  windowOk EndOfCombat (Just AnOpponents) = False
  windowOk FirstMain Nothing = False
  windowOk FirstMain (Just Yours) = False
  windowOk FirstMain (Just ThatPlayers) = False
  windowOk FirstMain (Just EachPlayers) = False
  windowOk FirstMain (Just EachOpponents) = False
  windowOk FirstMain (Just EachYours) = False
  windowOk FirstMain (Just AnOpponents) = False
  windowOk PostcombatMain Nothing = False
  windowOk PostcombatMain (Just Yours) = False
  windowOk PostcombatMain (Just ThatPlayers) = False
  windowOk PostcombatMain (Just EachPlayers) = False
  windowOk PostcombatMain (Just EachOpponents) = False
  windowOk PostcombatMain (Just EachYours) = False
  windowOk PostcombatMain (Just AnOpponents) = False
  windowOk DrawStep Nothing = False
  windowOk DrawStep (Just Yours) = False
  windowOk DrawStep (Just ThatPlayers) = False
  windowOk DrawStep (Just EachPlayers) = False
  windowOk DrawStep (Just EachOpponents) = False
  windowOk DrawStep (Just EachYours) = False
  windowOk DrawStep (Just AnOpponents) = False

  public export
  data WindowOk : TurnPart -> Maybe Owner -> Type where
    MkWindowOk : {auto 0 ok : windowOk p w = True} -> WindowOk p w

  public export
  -- spelling: ["Activate only as a sorcery", "Activate only as an instant",
  -- "Activate only during <Param(1)> <Param(0)>"] (a full sentence after
  -- the effect, [CR#602.1b] putting activation instructions last;
  -- conjoined with a sibling restriction by "and only". The window row
  -- writes the possessor's determiner and the part's own word -- see
  -- Owner and TurnPart -- with "any" as the quantifier's spelling here
  -- where the header writes "each"; the bare-Turn cell writes "during
  -- your turn" with no part word beyond it), kind: Sentence
  data Timing : Type where
    AsSorcery : Timing
    AsInstant : Timing
              -- the turn-part WINDOW, core's
              -- `Timing::DuringTurn(WhoseTurn)`/`DuringStep(PhaseStep,
              -- WhoseTurn)` under one row because this vocabulary's
              -- `TurnPart` already carries the turn itself as a part
              -- ([CR#500.1] -- a turn consists of phases, and the whole
              -- turn is what "during your turn" names). Core splits the
              -- two because its `PhaseStep` does not include the turn;
              -- the split is core's type's and not the language's.
              -- Gated by `windowOk`, NOT by `partUse`: the two readers
              -- disagree on six parts and two possessors, measured.
    DuringPart : (p : TurnPart) -> (w : Maybe Owner) ->
                 {auto 0 wk : WindowOk p w} -> Timing

  ||| How often an ability may be activated ([CR#602.5b]) — "Activate only
  ||| once each turn" (eighty-six lines) and the per-game "Activate only
  ||| once" (eight). `Nothing` is unlimited. Core keeps a LIST here and a
  ||| third row with it (`LoyaltyOncePerTurn`, [CR#606.3]'s shared cap);
  ||| the list is a divergence measured rather than copied — no corpus
  ||| line writes two use limits on one ability, the conjunctions being
  ||| limit-with-window ("only during your upkeep and only once each
  ||| turn") or limit-with-guard — and the loyalty row waits with the
  ||| loyalty COST surface it is inseparable from (the bracketed "[+1]",
  ||| eight hundred thirty-two components, a symbol vocabulary of its own).
  public export
  -- spelling: ["Activate only once each turn", "Activate only once"]
  -- (a full sentence, placed like Timing's; the per-game form writes no
  -- period of its own -- see [CR#702.177a] exhaust)
  data UsageLimit = OncePerTurn | OncePerGame

  ||| An ABILITY ([CR#113]) — the first type above `Effect`, and the
  ||| container a card's printed lines are made of. Three of [CR#113.3]'s
  ||| four categories are rows here — the activated ([CR#113.3b,602]), the
  ||| triggered ([CR#113.3c,603]) and the static ([CR#113.3d,604]) — and
  ||| the fourth, the spell ability ([CR#113.3a]), belongs with the card
  ||| container because [CR#113.3a] defines it by what its CARD is ("any
  ||| text on an instant or sorcery spell is a spell ability unless …"),
  ||| which is a fact no ability line carries about itself. Each arrives as
  ||| a row rather than as a type of its own — which is what core and the
  ||| settled Idris spec both do.
  |||
  ||| What the three rows have in common is what makes the container one
  ||| container: none of them is indexed by a discourse, because an ability
  ||| line is context-closed. What they DIVIDE on is a discipline the round
  ||| had to state: the ACTIVATED line's effect reads the cost's survivors,
  ||| the TRIGGERED line's reads the event's after-discourse, and the
  ||| STATIC line reads nothing at all, being a statement rather than an
  ||| instruction ([CR#604.1] — "written as statements, and they're simply
  ||| true").
  |||
  ||| The keyword row was this type's whole content before, and keeping it
  ||| is the point: what a `Gains` clause grants and what a card prints are
  ||| one category ([CR#113.3] lists them together), so the container is
  ||| grown rather than parallelled. Core is unindexed too; the spec
  ||| indexes its `Ability` so a keyword DESUGARING can reference an
  ||| anaphor, and this file has no keyword desugaring.
  public export
  data Ability : Type where
    -- spelling: (construction-owned -- pass-through; the word is entirely
    -- KeywordAbility's Keyword argument's own, see Keyword)
    KeywordAbility : Keyword -> Ability
    -- "[cost]: [effect]" ([CR#602.1] spells the whole line, activation
    -- instructions and all: "[Cost]: [Effect.] [Activation instructions
    -- (if any).]"). The effect is typed in the cost's PUBLIC-zone
    -- survivors, which is the juncture this file has had since the colon
    -- was first minted; what is new is that the cost is a `Cost`.
    -- The three restriction slots are separate for a corpus reason, not a
    -- core-imitating one: oracle CONJOINS them ("Activate only during your
    -- upkeep and only once each turn"), so one restriction row would have
    -- had to spell a conjunction of unlike things.
    -- The GUARD is typed at the empty context and not in the cost's
    -- survivors, which is [CR#602.5]'s own placement: a restriction on use
    -- is checked before the ability is activated at all, where the cost is
    -- not paid until [CR#601.2h], so nothing the cost names can be read by
    -- the condition that decides whether the cost may be paid. It reuses
    -- `Condition` verbatim, chapter eighteen's seam ([CR#603.4] keeps the
    -- ordinary "if" and the trigger's intervening-"if" apart by CARRIER).
    -- spelling: ["<Param(0)>: <Param(1)>"] plus one sentence per written
    -- restriction, appended after the effect in the order
    -- window/limit/guard and joined by "and only" when more than one is
    -- written ([CR#602.1b]: activation instructions "appear last, after
    -- the ability's effect"), kind: Ability
    -- The self-tap symbols are UNIQUE across a compound cost: [CR#107.5]
    -- says "a permanent that's already tapped can't be tapped again to
    -- pay the cost", so "{T}, {T}:" spends one state twice and "{T},
    -- {Q}:" is the same impossibility written the other way round
    -- (`CostTapOnce`, `badDoubleTapCost`).
    -- …and every component that names a payer names the ACTIVATOR
    -- ([CR#602.1a]; `CostPaidByYou`, `badForeignPayerCost`). Here rather
    -- than on the clause, because the same life component is a resolving
    -- PAYMENT under `Pay`, where the sentence names its own payer.
    Activated : (cost : Cost []) ->
                (eff : Effect (publicOnly (costIntro cost))) ->
                {auto 0 tp : CostTapOnce cost} ->
                {auto 0 py : CostPaidByYou cost} ->
                {default Nothing window : Maybe Timing} ->
                {default Nothing limit : Maybe UsageLimit} ->
                {default Nothing guard : Maybe (Condition [])} -> Ability
    -- "[When/Whenever/At] [event], [if condition,] [effect]" ([CR#603.1]
    -- spells the whole line: "[When/Whenever/At] [trigger condition or
    -- event], [effect]. [Instructions (if any).]").
    -- The WORD is a slot and not a derivation, because no rule assigns it
    -- and the corpus assigns only one of the three absolutely
    -- ([CR#603.2b]; `TriggerWord`, `triggerWordOk`). What the corpus DOES
    -- assign is recorded at `TriggerWord` rather than gated here, its
    -- exceptions turning on what the EFFECT does.
    -- The EFFECT is typed in the event's AFTER-discourse and not in what
    -- its phrases announced, which is the one place this row parts from
    -- `Intercepts` over the same vocabulary: [CR#603.6] has a zone-change
    -- trigger "look for the object in the zone that it moved to", so
    -- "Whenever a creature dies, return it to the battlefield" reads a
    -- card in a graveyard (`eventAfter`; `badTriggerReadsPreEventZone`).
    -- The INTERVENING "if" reuses `Condition` verbatim, and [CR#603.4] is
    -- why it is this row's slot rather than an ordinary trailing
    -- conditional: the rule applies "only to an 'if' that immediately
    -- follows a trigger condition" and checks it twice, once as the event
    -- occurs and again on resolution. It is typed in the event's
    -- after-discourse too, which the corpus settles.
    -- The marking word is "if" and there is NO second cell, which is the
    -- rule's own restriction rather than a measurement this vocabulary
    -- made: [CR#603.4] applies to "an 'if' that immediately follows a
    -- trigger condition" and says in the same breath that "the word 'if'
    -- has only its normal English meaning anywhere else". A "while" clause
    -- is therefore outside the rule, and the corpus agrees that it is a
    -- different construction: sixty header-internal "while" lines, none of
    -- them comma-marked, attaching the clause to the EVENT rather than
    -- between two commas after it -- "Whenever this creature attacks while
    -- saddled", "while you're the monarch", "while it's exiled" -- and
    -- three of them naming an action in progress rather than a state at
    -- all ("while you're activating a craft ability", "while casting a
    -- spell with emerge", "while scrying"). Its own family, ledgered.
    -- The line takes NO duration ([CR#611.3b]: an ability line is not an
    -- effect, so there is nothing for a duration to bound).
    -- spelling: ["<Param(0)> <Param(1)>, <Param(2)>",
    -- "<Param(0)> <Param(1)>, if <Param(3)>, <Param(2)>"] (the header word,
    -- the event's own finite clause, a comma, and the effect; the
    -- intervening condition sits between two commas when written --
    -- see [CR#603.4]), kind: Ability
    -- The HEADER announces no target ([CR#115.1d] chooses a triggered
    -- ability's targets "as the ability is put on the stack", which is
    -- after its event) — the demand is this carrier's alone, the delayed
    -- and reflexive rows writing the marked subject the corpus gives
    -- them (`HeaderNontarget`, `badTargetedDeathHeader`).
    Triggered : (word : TriggerWord) -> (ev : GameEvent []) ->
                {default Nothing intervening : Maybe (Condition (eventAfter ev))} ->
                (eff : Effect (interveningIntro intervening)) ->
                {auto 0 tr : Triggerable ev} ->
                {auto 0 hn : HeaderNontarget ev} ->
                {auto 0 wo : TriggerWordOk ev word} -> Ability
    -- "[statement]" -- the STATIC ability ([CR#113.3d,604.1]: "static
    -- abilities do something all the time rather than being activated or
    -- triggered. They are written as statements, and they're simply
    -- true"). Core's `Ability::Static(StaticEffect)` exactly, and the row
    -- that finally houses the families three chapters refused as
    -- effect-sentences: the durationless "can't" (Pacifism), the standing
    -- interception, the standing shield, the entry rider ([CR#603.6d]) and
    -- the conditional static.
    -- NO duration slot, and [CR#611.3b] is why in as many words: the
    -- effect "applies at all times that the permanent generating it is on
    -- the battlefield or the object generating it is in the appropriate
    -- zone". A `Continuously` clause states its span because it outlives
    -- its own resolution; a static ability has nothing to outlive.
    -- Two gates. WHICH statements English writes as a line is
    -- `staticAsAbility`, and the one refusal is the control grant, whose
    -- stative form is a different verb (`badStaticGainsControl`). And a
    -- static ability does not TARGET: [CR#115.1a..115.1e] enumerate what
    -- can -- an instant or sorcery spell, an activated ability, a
    -- triggered ability, and the keyword abilities that represent those --
    -- and [CR#115.1b] says of the nearest case outright that "an Aura
    -- permanent doesn't target anything; only the spell is targeted". So
    -- the line refuses a target-determined subject (`Untargeting`,
    -- `badStaticTargets`), which is the sharpest single difference between
    -- this row and the other two: the trigger targets ([CR#115.1d]) and
    -- the activated ability targets ([CR#115.1c]), and only the statement
    -- cannot.
    -- spelling: ["<Param(0)>"] (pass-through; the statement supplies its
    -- own sentence, in the STATIVE aspect the line takes -- see
    -- staticAsAbility), kind: Ability
    Static : (se : StaticEffect []) ->
             {auto 0 ln : StaticLine se} ->
             {auto 0 ut : Untargeting se} -> Ability
    -- "[instruction]" -- the SPELL ability, [CR#113.3]'s fourth category
    -- and the last row this container was missing. [CR#113.3a] defines it
    -- by two things and this row carries both: it is "followed as
    -- instructions while an instant or sorcery spell is resolving", so the
    -- payload is a plain `Effect` and nothing else -- no cost, no event,
    -- no header word -- and the qualification is a fact about the CARD,
    -- which no line carries about itself. That is why the row lives here
    -- and its gate lives on the container (`cardLineOk`): the same
    -- sentence "Draw two cards." is a spell ability on Divination and,
    -- word for word, an activated ability's effect after a colon.
    -- Typed at the EMPTY context like its three siblings, which for this
    -- row is the rule: [CR#608.2c] has the controller follow the spell's
    -- instructions "in the order written", and the first of them is the
    -- first thing on the card with nothing said before it.
    -- spelling: ["<Param(0)>"] (pass-through; the effect supplies its own
    -- sentence in the imperative, which is what an instruction is),
    -- kind: Ability
    Spell : (eff : Effect []) -> Ability

  ||| The static line's two demands as witnesses, named apart so a pin
  ||| says which question refused.
  public export
  data StaticLine : StaticEffect bs -> Type where
    MkStaticLine : {auto 0 ok : staticLineOk se = True} -> StaticLine se

  public export
  data Untargeting : StaticEffect bs -> Type where
    MkUntargeting : {auto 0 ok : anyTargetedAt (staticIntro se) = False} -> Untargeting se

  ||| An ordinary trigger's HEADER announces no target, and [CR#115.1d] is
  ||| the reason rather than a count: a triggered ability's targets "are
  ||| chosen as the ability is put on the stack", which happens because the
  ||| event already occurred, so the header cannot be where one is
  ||| announced. The ability targets perfectly well one clause later, which
  ||| is what makes this the header's own demand and not the container's.
  ||| The DELAYED carrier keeps the marked subject and is the corpus's
  ||| whole witness for one — Graceful Reprieve's "When target creature
  ||| dies this turn, …", where the spell that created the delay announced
  ||| the target as it was cast (`badTargetedDeathHeader`).
  public export
  data HeaderNontarget : GameEvent bs -> Type where
    MkHeaderNontarget : {auto 0 ok : anyTargetedAt (eventIntro ev) = False} ->
                        HeaderNontarget ev

  ||| May this ability be GRANTED by a clause? Only the keyword. English
  ||| grants an activated ability by QUOTING it — "Enchanted land has
  ||| \"{T}: Add {B}\"" — which is a construction this grammar has no
  ||| quotation for, where "gains flying" is a bare keyword. Growing the
  ||| container is what made the refusal necessary: the row exists now, so
  ||| the grant site has to say no to it (`badGainsActivated`).
  public export
  grantableAb : Ability -> Bool
  grantableAb (KeywordAbility _) = True
  grantableAb (Activated _ _) = False
  -- Neither of the two new lines is grantable either, and the refusal is
  -- the QUOTATION's a second and third time: English grants a triggered
  -- ability by quoting it ("Enchanted creature has 'When this creature
  -- dies, …'") exactly as it grants an activated one, and it grants a
  -- static ability by quoting it too ("as long as enchanted permanent is
  -- an Equipment, it has 'Equipped creature gets +1/+1 and has
  -- trample'"). One construction, three refusals
  -- (`badGainsTriggered`, `badGainsStatic`).
  grantableAb (Triggered _ _ _) = False
  grantableAb (Static _) = False
  -- The spell ability is not grantable either, and this refusal is a
  -- CATEGORY error rather than a missing quotation: [CR#113.3a] makes a
  -- spell ability something an instant or sorcery SPELL has while it
  -- resolves, and a `Gains` clause grants to a permanent on the
  -- battlefield ([CR#604.1]) — there is nothing there for one to be
  -- (`badGainsSpellAbility`).
  grantableAb (Spell _) = False

  ||| The grant demand as a witness, named apart so a pin says which
  ||| question refused.
  public export
  data Grantable : Ability -> Type where
    MkGrantable : {auto 0 ok : grantableAb ab = True} -> Grantable ab

  ||| A cost's components in written order, length-indexed so `Compound`
  ||| can demand two. Its own namespace for `SimEffects`' reason — a third
  ||| telescope would collide on the list sugar — and LAST in the block for
  ||| a mechanical one: a namespace ends the enclosing mutual scope's
  ||| forward reach, so everything that needs to name a later type has to
  ||| stand above it.
  -- spelling: (construction-owned -- list syntax for the Compound
  -- telescope; Nil/(::) are Idris list sugar, not English words)
  namespace Paid
    public export
    data CostSeq : Nat -> Bindings -> Type where
      Nil : CostSeq Z bs
      (::) : (c : Cost bs) -> {auto 0 nc : NotCompound c} ->
             CostSeq n (costIntro c) -> CostSeq (S n) bs

  ||| What a whole component sequence contributes: its last element's
  ||| discourse, the telescope having threaded every predecessor's.
  public export
  costsIntro : {bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bindings
  costsIntro [] = bs
  costsIntro (c :: cs) = costsIntro cs

  ||| Is this component a SELF-TAP payment? [CR#107.5] gives "{T}" the
  ||| fixed meaning "Tap this permanent" and [CR#602.5a] names "{Q}" as
  ||| its twin over the same tap state, so the two symbols spend one
  ||| resource between them.
  public export
  selfTapPayment : {0 bs : Bindings} -> Cost bs -> Bool
  selfTapPayment (Mana _) = False
  selfTapPayment TapSymbol = True
  selfTapPayment UntapSymbol = True
  selfTapPayment (Do _) = False
  selfTapPayment (Compound _) = False

  ||| How many of them a component sequence writes.
  public export
  selfTapCount : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Nat
  selfTapCount [] = Z
  selfTapCount (c :: cs) =
    (if selfTapPayment c then S Z else Z) + selfTapCount cs

  ||| At most ONE. [CR#107.5] says in as many words that "a permanent
  ||| that's already tapped can't be tapped again to pay the cost", and
  ||| [CR#118.3] makes a cost unpayable without the resources, so "{T},
  ||| {T}:" charges the same permanent twice for a state it has once —
  ||| and "{T}, {Q}:" is the same impossibility written the other way
  ||| round. No corpus line writes either (`badDoubleTapCost`).
  public export
  selfTapOnce : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  selfTapOnce cs = leNat (selfTapCount cs) 1

  ||| …asked of a whole cost, so the demand can ride the ABILITY LINE
  ||| rather than the `Compound` constructor. That placement is not
  ||| cosmetic: an auto-implicit on the constructor is solved before its
  ||| own component list's errors surface, so every malformed component
  ||| would have reported THIS question instead of its own
  ||| (`badNestedCompound`, `badCostReadsSiblingDeed`) — and the pin
  ||| discipline is the whole reason these are separate witnesses.
  ||| `Activated` is the only carrier a compound reaches: the pay clause
  ||| refuses one outright (`payableOk`).
  public export
  costTapOnce : {0 bs : Bindings} -> Cost bs -> Bool
  costTapOnce (Mana _) = True
  costTapOnce TapSymbol = True
  costTapOnce UntapSymbol = True
  costTapOnce (Do _) = True
  costTapOnce (Compound cs) = selfTapOnce cs

  ||| The self-tap uniqueness demand as a witness.
  public export
  data CostTapOnce : Cost bs -> Type where
    MkCostTapOnce : {auto 0 ok : costTapOnce c = True} -> CostTapOnce c

  ||| Does every component of this cost name the ACTIVATOR as its payer?
  ||| [CR#602.1a] says an activation cost "must be paid by the player who
  ||| is activating it", so a component that names a player names "you":
  ||| Erebos, God of the Dead's "Pay 2 life" and every other life component
  ||| charges the activator, and none charges an opponent.
  ||| The demand rides the ABILITY LINE and not the clause, which is the
  ||| whole of what makes it right: the same `Do (ChangeLife …)` component
  ||| stands under the `Pay` CLAUSE, where the payer is whoever the
  ||| sentence names ("unless its controller pays", "unless that player
  ||| pays"). One position demands the activator; the other writes its
  ||| payer down (`badForeignPayerCost`).
  ||| The two symbols and the mana run name no player at all; the action
  ||| rows that do are the life payment and the two subjected verbs, and
  ||| the inner catch-all is `payableOk`'s discipline.
  public export
  costPaidByYou : {0 bs : Bindings} -> Cost bs -> Bool
  costPaidByYou (Mana _) = True
  costPaidByYou TapSymbol = True
  costPaidByYou UntapSymbol = True
  costPaidByYou (Do (ChangeLife who _)) = nounIsYou who
  costPaidByYou (Do (Does subj _ _)) = nounIsYou subj
  costPaidByYou (Do _) = True
  costPaidByYou (Compound cs) = costsPaidByYou cs

  ||| The same question over a whole component sequence.
  public export
  costsPaidByYou : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  costsPaidByYou [] = True
  costsPaidByYou (c :: cs) = costPaidByYou c && costsPaidByYou cs

  ||| The activator-pays demand as a witness.
  public export
  data CostPaidByYou : Cost bs -> Type where
    MkCostPaidByYou : {auto 0 ok : costPaidByYou c = True} -> CostPaidByYou c

  ||| Under the pay CLAUSE the component's payer agrees with the verb's
  ||| subject. The verb spells the subject once — "you pay 3 life", "its
  ||| controller pays {1}" — so a component naming a different player
  ||| spells nobody the sentence wrote (`badMismatchedPayer`).
  ||| One direction only, and the reason is the telescope rather than
  ||| laziness: the subject and the component's own player slot live in
  ||| two different contexts and cannot be compared at all (`effEq`'s
  ||| problem). What CAN be stated is that an imperative "you pay" takes a
  ||| component naming you, which is the frame a hundred forty-five
  ||| "unless you pay" lines write; a named subject's component is left to
  ||| the anaphor work the payer family is ledgered with.
  public export
  payAgreesOk : {0 bs : Bindings} -> {0 cs : Bindings} ->
                Noun bs Player -> Cost cs -> Bool
  payAgreesOk who c = not (nounIsYou who) || costPaidByYou c

  ||| The pay clause's agreement demand as a witness.
  public export
  data PayAgrees : Noun bs Player -> Cost cs -> Type where
    MkPayAgrees : {auto 0 ok : payAgreesOk who c = True} -> PayAgrees who c

  ||| Can every component of this cost be paid from OFF the battlefield?
  ||| The two symbols cannot: [CR#107.5] gives "{T}" the fixed meaning "tap
  ||| this permanent" and [CR#602.5a] pairs "{Q}" with it, and only a
  ||| permanent taps. Everything else can — a symbol run is paid from the
  ||| mana pool and an action component names its own patient. The
  ||| classification is the rules' own axis: [CR#113.6j] says an activated
  ||| ability "that has a cost that can't be paid while the object is on
  ||| the battlefield functions from any zone in which its cost can be
  ||| paid", which is the rule cycling runs on. The card container is the
  ||| reader ([CR#110.4] keeps an instant or sorcery card off the
  ||| battlefield): zero Instant or Sorcery cards carry a top-level "{T}"
  ||| ability, and cycling's "{2}{W}, Discard this card" is what one really
  ||| looks like (`badTapSorcery`).
  public export
  costOffBattlefield : {0 bs : Bindings} -> Cost bs -> Bool
  costOffBattlefield (Mana _) = True
  costOffBattlefield TapSymbol = False
  costOffBattlefield UntapSymbol = False
  costOffBattlefield (Do _) = True
  costOffBattlefield (Compound cs) = costsOffBattlefield cs

  ||| The same question over a whole component sequence.
  public export
  costsOffBattlefield : {0 bs : Bindings} -> {0 n : Nat} -> CostSeq n bs -> Bool
  costsOffBattlefield [] = True
  costsOffBattlefield (c :: cs) = costOffBattlefield c && costsOffBattlefield cs

-- ===== The card container =====


||| Is a supertype already in a list? `colorMember`'s shape over the
||| other catalog list.
public export
supertypeMember : Supertype -> List Supertype -> Bool
supertypeMember s [] = False
supertypeMember s (t :: ts) = sameSupertype s t || supertypeMember s ts

||| A card's supertypes are DUPLICATE-FREE. [CR#205.4b] makes a supertype
||| a property an object HAS or LACKS — an object that "gains or loses a
||| supertype … retains any OTHER supertypes it had", which is a set's
||| arithmetic and not a list's — so a word printed twice is one fact
||| written twice, which is `colorsDistinct`'s refusal at the catalog list
||| beside it. Unordered, for the colors' reason too: [CR#205.4a] fixes no
||| order among them, and Snow-Covered Forest carries both Basic and Snow.
public export
supersDistinct : List Supertype -> Bool
supersDistinct [] = True
supersDistinct (s :: ss) = not (supertypeMember s ss) && supersDistinct ss

||| The supertype list's demand as a witness — the field carried none at
||| all until this round, which is how "Legendary Legendary Creature"
||| came to be writable (`badDuplicateSupertype`).
public export
data CardSupers : List Supertype -> Type where
  MkCardSupers : {auto 0 ok : supersDistinct ss = True} -> CardSupers ss

||| Which of the two things a card's TEXT can be, read off its type
||| line. [CR#113.3a] makes this the container's central question and
||| asks it of the card and never of the line: "any text on an instant
||| or sorcery spell is a spell ability unless it's an activated
||| ability, a triggered ability, or a static ability that fits the
||| criteria described" [CR#113.6].
public export
data CardClass = PermanentCard | SpellCard

||| A line naming ANY non-permanent type is a spell card. The
||| disjunction rather than a conjunction is [CR#205.1a]'s doing — "an
||| object with either the instant or sorcery card type retains that
||| type" — so the spell type is the one that cannot be argued away.
||| A typeless line answers `PermanentCard` and is unreachable through
||| the container, which demands a non-empty line (`CardLine`).
public export
cardClassOf : List CardType -> CardClass
cardClassOf [] = PermanentCard
cardClassOf (t :: ts) = if permanentType t then cardClassOf ts else SpellCard

||| Does a line name a permanent type at all? Its spell twin below.
public export
anyPermanentType : List CardType -> Bool
anyPermanentType [] = False
anyPermanentType (t :: ts) = permanentType t || anyPermanentType ts

public export
anySpellType : List CardType -> Bool
anySpellType [] = False
anySpellType (t :: ts) = not (permanentType t) || anySpellType ts

||| The two families do not share a line. [CR#110.4] rules it out in as
||| many words — "instant and sorcery cards can't enter the battlefield
||| and thus can't be permanents" — and [CR#110.4a] lists the six types
||| that can, so a line naming one of each names a card that would have
||| to be a permanent and not be one. The ORDER check cannot see this:
||| the ranks put the two spell types last, so [Land, Creature, Instant]
||| is perfectly ascending and was accepted (`badMixedPermanentSpellLine`).
||| One demand, both directions, because the illegality is the PAIRING
||| and not either word.
public export
typesCombinable : List CardType -> Bool
typesCombinable tys = not (anyPermanentType tys && anySpellType tys)

||| WHICH keywords a card of each class may carry. The row is shut on
||| spell cards on a measurement rather than a rule: no Instant or Sorcery
||| card in the supported corpus prints any of the seven keywords this
||| file carries as a bare keyword line, zero lines against the permanent
||| cards that print them everywhere (`badKeywordOnInstant`).
||| `cardAbilityOk` delegates here rather than answering with a wildcard,
||| so a new keyword cannot inherit a decision nothing measured for it.
public export
keywordCardOk : CardClass -> Keyword -> Bool
keywordCardOk PermanentCard Haste = True
keywordCardOk PermanentCard Flying = True
keywordCardOk PermanentCard Trample = True
keywordCardOk PermanentCard Vigilance = True
keywordCardOk PermanentCard Deathtouch = True
keywordCardOk PermanentCard DoubleStrike = True
keywordCardOk PermanentCard FirstStrike = True
keywordCardOk SpellCard Haste = False
keywordCardOk SpellCard Flying = False
keywordCardOk SpellCard Trample = False
keywordCardOk SpellCard Vigilance = False
keywordCardOk SpellCard Deathtouch = False
keywordCardOk SpellCard DoubleStrike = False
keywordCardOk SpellCard FirstStrike = False

||| WHICH ability rows a card of each class may carry — the container's
||| gate. Full rows in both directions, so a new ability row and a new card
||| class each have to declare their answer.
|||
||| The SPELL row is [CR#113.3a] exactly: a spell ability is followed
||| "while an instant or sorcery spell is resolving", so a permanent card's
||| text is never one, and a spell card's text is one by DEFAULT — the
||| rule's "unless" naming the exceptions rather than the rule. So the
||| split is not "permanent cards hold `List Ability` and spells hold an
||| `Effect`": both hold a list of lines, and what differs is which rows
||| the list may contain.
|||
||| ACTIVATED and TRIGGERED are open on both sides — cycling is an
||| activated ability printed on sorceries, "When you cycle this card, …"
||| a triggered one — but the activated cell asks the COST, [CR#113.6j]
||| letting such an ability function off the battlefield exactly when its
||| cost can be paid there and [CR#110.4] keeping a spell card off it. So
||| cycling's "{2}{W}, Discard this card" passes and a top-level "{T}"
||| line does not ([CR#107.5]; `badTapSorcery`).
|||
||| STATIC is shut on spell cards by [CR#113.6]'s criteria rather than a
||| count: "abilities of an instant or sorcery spell usually function only
||| while that object is on the stack", where every `StaticEffect` row here
||| establishes a continuous effect on the BATTLEFIELD. The two exception
||| families ([CR#113.6d,113.6e]) this grammar does not spell, and the play
||| PERMISSION it does ([CR#604.6]) is ledgered rather than carved out of
||| this cell (`badStaticOnSorcery`).
public export
cardAbilityOk : CardClass -> Ability -> Bool
cardAbilityOk PermanentCard (KeywordAbility k) = keywordCardOk PermanentCard k
cardAbilityOk PermanentCard (Activated _ _) = True
cardAbilityOk PermanentCard (Triggered _ _ _) = True
cardAbilityOk PermanentCard (Static _) = True
cardAbilityOk PermanentCard (Spell _) = False
cardAbilityOk SpellCard (KeywordAbility k) = keywordCardOk SpellCard k
cardAbilityOk SpellCard (Activated c _) = costOffBattlefield c
cardAbilityOk SpellCard (Triggered _ _ _) = True
cardAbilityOk SpellCard (Static _) = False
cardAbilityOk SpellCard (Spell _) = True

||| Every line of a card's text is admissible for its own type line.
public export
cardTextOk : List CardType -> List Ability -> Bool
cardTextOk tys [] = True
cardTextOk tys (a :: as) = cardAbilityOk (cardClassOf tys) a && cardTextOk tys as

||| A card's P/T slot against its types — `tokenPtOk`'s table at a second
||| site and one-directional for its reason. [CR#208.1] says a CREATURE
||| card "has two numbers separated by a slash printed in its lower right
||| corner", so the creature type demands them; the converse is not
||| demanded, [CR#301.7]'s Vehicle printing a power and toughness without
||| the creature type. What the slot does NOT carry is [CR#208.2]'s star:
||| a characteristic-defining ability setting power and toughness is an
||| ability line this container has no row for (ledger).
|||
||| The pair is SIGNED, and the reason is a card rather than a taste:
||| [CR#107.1b] says "it's possible for a game value, such as a creature's
||| power, to be less than zero", and Char-Rumbler prints "-1/3" in the
||| corner [CR#208.1] describes. Stored as `Nat` the container did not
||| refuse that card — Idris saturates a negative literal, so the printed
||| -1 was silently kept as 0 and the record misrepresented the card it
||| was holding (`charRumbler`). The TOKEN's pair stays `Nat`
||| (`TokenChars`): a token's characteristics are what the creating effect
||| defines ([CR#111.3]), and no clause here spells a negative one.
public export
cardPtOk : List CardType -> Maybe (Integer, Integer) -> Bool
cardPtOk tys pt = case (lineHasType Creature tys, pt) of
  (True, Nothing) => False
  _ => True

||| A land card writes NO mana cost, and [CR#202.1b] is the rule:
||| "some objects have no mana cost. This normally includes all land
||| cards", the absence representing an unpayable cost [CR#118.6]. The
||| demand is one-directional in the other direction from the P/T one —
||| a nonland card may also write none (Evermind, the suspend-only
||| cards), so only the land row is closed (`badLandWithManaCost`).
public export
cardCostOk : List CardType -> Maybe ManaCost -> Bool
cardCostOk tys cost = case (lineHasType Land tys, cost) of
  (True, Just _) => False
  _ => True

||| The three demands as witnesses, named apart so a pin says which one
||| refused (the `TokenTyped`/`TokenPt` discipline).
public export
data CardLine : TypeLine -> Type where
  MkCardLine : {auto 0 ne : lineNonEmpty l = True} ->
               {auto 0 ord : typesOrdered l.tys = True} ->
               {auto 0 cmb : typesCombinable l.tys = True} ->
               {auto 0 sf : subsFitLine l.subs l.tys = True} -> CardLine l

public export
data CardText : TypeLine -> List Ability -> Type where
  MkCardText : {auto 0 ok : cardTextOk l.tys as = True} -> CardText l as

public export
data CardPt : TypeLine -> Maybe (Integer, Integer) -> Type where
  MkCardPt : {0 stats : Maybe (Integer, Integer)} ->
             {auto 0 ok : cardPtOk l.tys stats = True} -> CardPt l stats

public export
data CardCost : TypeLine -> Maybe ManaCost -> Type where
  MkCardCost : {auto 0 ok : cardCostOk l.tys c = True} -> CardCost l c

||| A CARD — the outermost container, and the first type here that is not
||| a construction at all. [CR#200.1] lists fifteen parts; this record
||| carries the five the grammar has anything to say about (name, mana
||| cost, type line, text box, power and toughness) and leaves the rest to
||| the printer.
|||
||| What the container BUYS is the question no ability line can answer
||| about itself. [CR#113.3a] makes the spell ability a fact about the
||| card ("any text on an instant or sorcery spell …"), so `Ability.Spell`
||| could exist as a row but could not be gated until something knew the
||| type line; `cardAbilityOk` is that gate and the reason this type
||| exists at all.
|||
||| Core's shape is `deckmaste_semantics/src/card.rs`'s `CardFace`, and
||| the divergences are three and each is measured. Core keeps
||| `types`/`subtypes` as two flat fields where this carries chapter
||| nineteen's `TypeLine` record, the same two lists under one name and
||| shared with the two clause readers. Core carries `color_indicator`,
||| `loyalty` and `defense`; none is written here — the colour indicator
||| is a printed dot and not language ([CR#204.1]), and the loyalty pair
||| is ledgered with the planeswalker type. And core's `Card` wraps
||| `CardFace` in a two-faced enum ([CR#712.8] gives each face its own
||| characteristics); a face is what this record is, and the layouts are
||| the engine's.
|||
||| The NAME is a `String` and the one field with no grammar in it,
||| carried for `TokenChars`' reason at a second site: [CR#201.1] makes it
||| a printed part, and modern oracle templating writes "this creature"
||| rather than the name, so nothing here reads it back.
public export
-- spelling: (construction-owned -- the card's printed layout rather than
-- a sentence: the name, then the mana cost, then the type line
-- (supertypes, card types, an em dash and the subtypes when written),
-- then the text box's lines one per ability, then the P/T pair as
-- "<power>/<toughness>". Never spelled inside another construction)
record Card where
  constructor MkCard
  name : String
  cost : Maybe ManaCost
  supers : List Supertype
  line : TypeLine
  text : List Ability
  pt : Maybe (Integer, Integer)

-- (The container's five demands are NOT gathered into one witness, and
-- the reason is the pin discipline: a gathering type would report its
-- own name whichever demand refused, where five separate ones let a
-- `failing` block say which question the card failed. They are threaded
-- to the construction site by the `card` macro instead. The fifth is the
-- SUPERTYPE list's, which carried no witness at all until this round.)
