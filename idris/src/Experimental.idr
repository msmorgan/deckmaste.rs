||| The semantics-target workbench: what shape should the semantics layer
||| be, sitting between English and core? Deliberately divorced from
||| `Semantics` — no import, no reuse of its types; nothing here is
||| emitted, mirrored, or run, the typechecker is the only consumer.
|||
||| The chapter-by-chapter findings, engine-boundary deferrals, and
||| representability/AST frontier notes that used to live in this doc
||| comment now live in `experiment-log.md`, right beside this file —
||| read there for the full record. (Named off the module on purpose:
||| Idris2 reads a same-named `.md` as literate source, so an
||| `Experimental.md` here would shadow this module.) Per-declaration
||| docs and `-- spelling:` notes stay here, beside the code they
||| describe.
module Experimental

%default total

-- ===== Vocabulary =====

||| Card types, as catalog atoms ([CR#205.2a]; only what the chapters
||| need). Ruling: types are intrinsic to NEITHER core nor semantics —
||| each is DECLARED by a `TypeDef` macro carrying its rules grants
||| (`plugins/builtin/macros/cardtype/Creature.ron` confers the combat
||| grants, and even permanence is its `permanent: true` field), so
||| this enum is the workbench's stand-in for reading those
||| declarations, like the keyword and verb macro names.
public export
-- spelling: (construction-owned catalog -- Creature="creature", Artifact=
-- "artifact", Land="land", Enchantment="enchantment"; each row is a TypeDef
-- macro's own word (see cardtype/Creature.ron), consumed by HasType/AsType,
-- never spelled alone)
data CardType = Creature | Artifact | Land | Enchantment

||| Fight participation, per type — the stand-in for reading a
||| combat-participant grant from the TypeDef declaration, distinct
||| from Creature.ron's May(Attack)/May(Block): fight keys on type
||| membership [CR#701.14a,701.14b] and deals non-combat damage
||| [CR#701.14d]. Full rows: a new type must declare its answer.
public export
combatant : CardType -> Bool
combatant Creature = True
combatant Artifact = False
combatant Land = False
combatant Enchantment = False

||| The projected-head gate the fight slots consume — an untyped head
||| cannot prove participation, and the witness carries the grant
||| table's verdict.
public export
data FightParticipant : Maybe CardType -> Type where
  Fighter : {auto 0 ok : combatant t = True} -> FightParticipant (Just t)

||| The numeric characteristics a phrase can NAME — one axis serving
||| both readers this grammar has: the bound a phrase compares against
||| (`Compare`) and the value an amount reads off an object (`StatOf`).
||| Core does the same with one `Stat` (`deckmaste_core/src/count.rs`),
||| and it carries five: power and toughness [CR#208.1], mana value
||| [CR#202.3], loyalty [CR#209.1], defense [CR#210.1]. The two missing
||| here are missing in BOTH frames, measured rather than assumed.
||| Bounded: "loyalty N or less" and "defense N or greater" appear zero
||| times each. Read: "defense" is never read off an object at all, and
||| loyalty is read once in the whole corpus — Drain Life's cap clause
||| ("but not more life than … the planeswalker's loyalty …"), which is
||| the phrasal-standard frame this grammar does not write either way
||| (see `Comparator`), while every "loyalty" line the corpus does write
||| counts COUNTERS ("the number of loyalty counters on him"), a
||| different reader. Full rows everywhere below, so a fourth
||| characteristic has to declare which heads carry it — and which
||| frames read it — before it can be written.
public export
-- spelling: (construction-owned catalog -- Power="power", Toughness=
-- "toughness", ManaValue="mana value"; each row is the characteristic's own
-- word, inside Compare's frame or after StatOf's possessive, never spelled
-- alone. Same three names core's `Stat` gives its own first three rows
-- (count.rs))
data Characteristic = Power | Toughness | ManaValue

||| Characteristic equality, per-row — `sameZone`'s discipline.
public export
sameChar : Characteristic -> Characteristic -> Bool
sameChar Power Power = True
sameChar Power _ = False
sameChar Toughness Toughness = True
sameChar Toughness _ = False
sameChar ManaValue ManaValue = True
sameChar ManaValue _ = False

||| The two comparators a WRITTEN bound takes. Core's `Cmp` has five
||| (`condition.rs`: `Eq`, `AtLeast`, `AtMost`, `Greater`, `Less`) and
||| the relation here is exactly its `AtMost` and `AtLeast`; what the
||| workbench cannot borrow is the other three, because English does
||| not spell them against a numeral. "Power less than 4" and "mana
||| value greater than 3" are written zero times; the strict
||| comparators appear only against a PHRASAL standard ("power less
||| than Yasova Dragonclaw's power", "mana value less than or equal to
||| the number of lands you control"), which is a different frame with
||| a different word order and waits on the ledger. The two rows are
||| therefore the whole bounded vocabulary, and a third is a totality
||| error.
public export
-- spelling: (construction-owned catalog -- OrLess="or less", OrGreater=
-- "or greater", each following its bound inside Compare's frame. The
-- comparative word is fixed by the HEAD CLASS, not chosen: every scalar
-- characteristic takes greater/less and never more/fewer, which the english
-- crate stores rather than derives (`ComparativeWord`, syntax/phrase.rs))
data Comparator = OrLess | OrGreater

public export
sameCmp : Comparator -> Comparator -> Bool
sameCmp OrLess OrLess = True
sameCmp OrLess _ = False
sameCmp OrGreater OrGreater = True
sameCmp OrGreater _ = False

||| Which card type a characteristic PRESUPPOSES of the object read —
||| the characteristic half of the closed table `deedType` writes for
||| deeds. Power and toughness belong to creatures: a noncreature
||| permanent has neither, and a noncreature object off the battlefield
||| has them only if they are printed on it ([CR#208.3]). Mana value
||| belongs to every object ([CR#202.3] defines it for one, and
||| [CR#202.3a] gives even a costless object the value zero), so it
||| presupposes nothing and the corpus agrees on both counts: no power
||| or toughness bound is written on any head but a creature's, while
||| mana value is bounded on cards, spells, permanents, artifacts,
||| planeswalkers, and enchantments alike. The Vehicle is the caveat
||| [CR#208.3] leaves open — a noncreature card CAN carry printed
||| power in a graveyard — and the corpus never writes it, so the
||| creature row is exact for what English spells; revisit if a
||| printed-P/T noncreature head ever turns up.
public export
comparedType : Characteristic -> Maybe CardType
comparedType Power = Just Creature
comparedType Toughness = Just Creature
comparedType ManaValue = Nothing

||| Quality sorts — the choosable characteristics ([CR#105.1,302.3];
||| only what the chapters need).
public export
-- spelling: (construction-owned catalog -- Color="color", CreatureType=
-- "creature type"; consumed by QualityNoun/OfChosen, never spelled alone)
data QualitySort = Color | CreatureType

public export
sameQ : QualitySort -> QualitySort -> Bool
sameQ Color Color = True
sameQ Color _ = False
sameQ CreatureType CreatureType = True
sameQ CreatureType _ = False

||| What a binding can bind ([CR#115.1] — targets are objects and/or
||| players; the union kind is deferred with the carrier lattice) —
||| plus chosen qualities ("Choose a color"), which enter the same
||| discourse, and event OUTCOMES (§3's third data class: what a
||| clause DID, readable as "that much"), which only clauses
||| introduce.
public export
data Kind = Object | Player | Quality QualitySort | Outcome

||| Kind equality — deliberately WITHOUT a catch-all: adding a Kind
||| makes this a totality error, not a silent zero in the counters.
public export
sameKind : Kind -> Kind -> Bool
sameKind Object Object = True
sameKind Object Player = False
sameKind Object (Quality _) = False
sameKind Object Outcome = False
sameKind Player Object = False
sameKind Player Player = True
sameKind Player (Quality _) = False
sameKind Player Outcome = False
sameKind (Quality _) Object = False
sameKind (Quality _) Player = False
sameKind (Quality a) (Quality b) = sameQ a b
sameKind (Quality _) Outcome = False
sameKind Outcome Object = False
sameKind Outcome Player = False
sameKind Outcome (Quality _) = False
sameKind Outcome Outcome = True

||| The surface-projected SORT of an event outcome — what kind of
||| thing the clause did; its magnitude stays runtime, never stored
||| (the §3 ruling). Only what this chapter's reads need; mana
||| produced, counters, and card counts are later sorts.
public export
data OutcomeSort = DamageDealt | LifeGained | LifeLost

||| Singular mention or group mention — the guard that keeps "it" from
||| resolving to a plural antecedent.
public export
data Plurality = OneOf | ManyOf

||| Grammatical number as a Bool, for the gates that only ask
||| "singular?" — per-row, so a new number is a totality error.
public export
isOne : Plurality -> Bool
isOne OneOf = True
isOne ManyOf = False

||| The number a DISTRIBUTED output takes: one thing per agent, over many
||| agents, is many things. A clause whose agent is the distributive
||| determiner runs once per member, so what the sentence AFTER it can
||| read back is the union of every agent's output — "Each player creates
||| a green Elephant creature token. THOSE CREATURES have …" (Elephant
||| Resurgence). The per-agent amount is one, and the discourse mention
||| is still plural. Both arguments are written out rather than folded
||| through `isOne`, so a new number is a totality error on the pair.
public export
outputPlur : (agent : Plurality) -> (perAgent : Plurality) -> Plurality
outputPlur OneOf OneOf = OneOf
outputPlur OneOf ManyOf = ManyOf
outputPlur ManyOf OneOf = ManyOf
outputPlur ManyOf ManyOf = ManyOf

||| How many objects a counted mention takes — core's `Quantity`
||| (`deckmaste_core/src/quantity.rs`): ONE primitive, a range with
||| both bounds optional (`Nothing` = unbounded that side, so "any
||| number of" is `Range Nothing Nothing` — the variable target count
||| [CR#601.2c] has its caster announce before choosing). The
||| readable named forms are macros over it in core and macros over it
||| here (`exactly`, `upTo`, `anyNumber` in `Experimental.Macros`,
||| answering core's `Exactly`/`AtMost`/`AnyNumber`; its `AtLeast` and
||| `Between` spell over the same primitive when a corpus line wants
||| them). A magnitude is not a quantity — that is `Amount`.
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
||| cannot target nothing). The UPPER bound carries the demand: a
||| statically zero maximum is unwritten English however it is spelled,
||| exactly ("zero target creatures") or as a bound ("up to zero") —
||| `badZeroGroup`. An ABSENT maximum is the unbounded "any number of",
||| and a zero LOWER bound is what every "up to" has, so neither is
||| touched.
public export
data NonZeroQ : Quantity -> Type where
  UnboundedAbove : NonZeroQ (Range lo Nothing)
  MaxAtLeastOne : NonZeroQ (Range lo (Just (S n)))

||| A written quantity also runs UPWARD, from a minimum of at least
||| one — the demands `NonZeroQ`'s upper bound cannot make, since a
||| range is two numbers and only one of them is up there. A DESCENDING
||| range admits nothing at all ("between three and two target
||| creatures" names an empty interval, and the plurality read off the
||| maximum would lie about it besides), and a ZERO minimum spells
||| nothing the unbounded form does not already say: [CR#107.1c] has
||| "any number" permit zero outright, so "zero or more target
||| creatures" is a second spelling of "any number of target
||| creatures" — and one the corpus never writes
||| (`badDescendingRange`, `badZeroLowerRange`). An absent minimum is
||| every "up to", untouched.
||| "No greater than", per row over the written numerals.
public export
leNat : Nat -> Nat -> Bool
leNat Z _ = True
leNat (S _) Z = False
leNat (S a) (S b) = leNat a b

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
||| is SINGULAR — "target creature" and "up to one target creature"
||| both rement as "it" (Ty Lee, Chi Blocker) — and every wider
||| quantity is a group. The MAXIMUM is what number reads; choosing
||| fewer [CR#115.6] is runtime's null read, not a grammar fact.
public export
quantPlur : Quantity -> Plurality
quantPlur (Range _ (Just (S Z))) = OneOf
quantPlur (Range _ _) = ManyOf

||| Numeral equality, over the one ordering primitive rather than a
||| second table of its own.
public export
eqNat : Nat -> Nat -> Bool
eqNat a b = leNat a b && leNat b a

||| Whether a modal headcount FITS the list of modes it heads
||| ([CR#700.2]) — two demands, and both are about the pair rather than
||| about either half, which is why they are one relation and not a
||| widening of `WellFormedQ`.
|||
||| The first is impossibility: a headcount cannot reach past the modes
||| offered ("choose three" of two options names nothing), so a WRITTEN
||| maximum is no greater than the count. The corpus never comes close —
||| every printed headcount is strictly under its list but for the two
||| open-topped forms, whose top IS the count and whose words say so
||| ("one or both" over exactly two, fifty-two cards; "one or more" over
||| three, four, or five, nineteen cards).
|||
||| The second is that a modal offers a CHOICE: [CR#700.2] calls a spell
||| modal when its bulleted options come "preceded by instructions for a
||| player to choose a number of those options", and a headcount that
||| fixes the whole list instructs nothing — "Choose two —" over two
||| modes has one answer. Zero cards write it (`badModalFixedWhole`).
||| An open top escapes this by construction, having a range to choose
||| in; so does every "up to", which may take fewer.
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
||| separate question: `modesFit` asks whether a headcount could name a
||| choice at all, this asks whether oracle spells it. The counts are
||| supported-corpus lines carrying the bulleted em-dash: "choose one —"
||| eighty-four, "choose two —" three, "choose three —" one (Mishra,
||| Eminent One), "choose up to one —" five, "choose one or both —"
||| four, "choose one or more —" four, "choose any number —" two (Rankle,
||| Master of Pranks; Rankle and Torbran). Written ZERO times, all
||| scopes: "choose up to two —", "choose up to three —", "choose four",
||| "two or more", "one or two". So the vocabulary is the three small
||| fixed counts, the one-capped "up to", the two open tops, and the
||| unbounded head — and an unattested head is refused rather than
||| inferred from the range algebra (`badModalUpToTwo`).
||| The two OPEN TOPS take their maximum from the list: "one or both"
||| over exactly two modes and "one or more" over more, which is what
||| those words say. A one-to-two range over three modes would have to
||| spell "one or two" and no card does, so the top is required to BE the
||| count rather than merely fit under it.
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
||| "a", "each", "all", or a definite/derived mention). Rules facts
||| (the settled-target boundary, the "other" presupposition) are
||| functions of it, never stored alongside it. Counted target mentions
||| share ONE tag whatever their quantity: "up to" once held its own on
||| the theory that a possibly-empty group [CR#115.6] was a different
||| word for the presupposition to see, but finding 35 repealed that
||| and no consumer ever told the two apart — the emptiness lives in
||| the quantity, as it does in core's single announce form.
public export
-- spelling: (construction-owned -- TargetD/AD/EachD/AllD/TheD mark WHICH Noun
-- constructor built a binding; the words live on Noun's own TargetGroup/A/
-- Each/AllOf/TheVerbed rows, not here. The english crate has its own
-- `Determiner` hole (constructions/coordination.rs's shared_determiner_nominal)
-- confirming the concept; TODO(reason: no single owning family name verified
-- for the per-word constructions within this pass's scope))
data Determiner = TargetD | AD | EachD | AllD | TheD | PartD

||| Zone sorts, minimally ([CR#400.1] family) — the fold-state tag a
||| binding carries. Ownership is not stored here; it lives in the
||| surface `ZoneExpr` where English writes it. The library is the one
||| ORDERED zone ([CR#401.2] — "a single face-down pile", whose order
||| players may neither inspect nor change), and the order is no part of
||| the SORT: it lives where English writes it, on the position phrase
||| (`LibraryAt`), exactly as ownership does. Core makes the same split
||| (`Zone::Library` beside `Destination::Library(Anchor)`).
public export
data Zone = Battlefield | Graveyard | Exile | Hand | Library

||| Zone equality, per-row: a new zone is a totality error on its
||| missing row, never a silent False.
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

||| A position IN the ordered library ([CR#401.2]) — the two words
||| English writes bare, and the whole of the attested position
||| vocabulary at offset zero: "on top of your library" (a hundred
||| twenty-seven destination lines, "the top card/cards of" twelve
||| hundred and more on the noun side) and "on the bottom of your
||| library" (four hundred eighteen). Core spells the axis once, with an
||| offset (`Anchor = FromTop(Count) | FromBottom(Count)`); English
||| writes the offset with a DIFFERENT phrase — the ordinal "second from
||| the top" (ten lines) and "third from the top" (fourteen), [CR#401.7]'s
||| own subject — so it is not this table's third row but an ordinal
||| vocabulary this file does not have (ledger).
public export
-- spelling: (construction-owned catalog -- each row is the position phrase
-- LibraryAt writes over its zone word: OnTop = "on top of <scope> library",
-- OnBottom = "on the bottom of <scope> library". Noun-side the same words
-- lead the slice ("the top <n> cards of <scope> library"). Never spelled
-- alone -- see LibraryAt and LibrarySlice)
data LibPos = OnTop | OnBottom

||| How a GROUP landing at a library position is arranged ([CR#401.4]):
||| the two riders English writes, "in any order" (two hundred
||| thirty-seven) and "in a random order" (three hundred thirty-two).
||| Core carries four (`Arrangement`, adding `SameOrder` and a
||| `ChosenOrder(Reference)`); English writes neither — "in the same
||| order" and "in an order of their choice" are zero lines each — so
||| this is a recorded NARROWING of core, `DividedVerb`'s discipline
||| (finding 126) applied to the order rider.
|||
||| The rider is not a default made explicit. [CR#401.4] already gives
||| the cards' owner the arrangement ("may arrange them in any order"),
||| so "in any order" RESTATES the rule and only "in a random order"
||| overrides it — which is why the absent rider and the any-order rider
||| mean the same thing and both are written.
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

||| Verb-tag equality, per-row: each row ends in its own catch-all, so
||| a NEW verb is a totality error on the missing row (its diagonal
||| cannot silently go False) without the full quadratic.
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

||| Per-kind mention data, kind-indexed so a binding can only record
||| what its kind can have: an object carries the projected head type
||| and its current zone (the ONE piece of fold-state — `Move` updates
||| it; everything else is a projection of the phrase); players and
||| qualities carry nothing. An ill-sorted binding ("a player in your
||| hand") is thereby unrepresentable — the refusal the surface grammar
||| makes (`InZone` is Object-kinded), extended to the representation.
public export
data Payload : Kind -> Type where
  ObjectP : (ty : Maybe CardType) -> (zone : Maybe Zone) ->
            (prov : Maybe Stamp) -> Payload Object
  PlayerP : Payload Player
  QualityP : Payload (Quality q)
  OutcomeP : (sort : OutcomeSort) -> Payload Outcome

||| One discourse mention: its determiner, kind, plurality, and its
||| kind's own data.
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

||| The zone a binding tracks — object fold-state; players and
||| qualities have none, structurally.
public export
bindingZone : Binding -> Maybe Zone
bindingZone (MkBinding _ _ _ (ObjectP _ zn _)) = zn
bindingZone (MkBinding _ _ _ PlayerP) = Nothing
bindingZone (MkBinding _ _ _ QualityP) = Nothing
bindingZone (MkBinding _ _ _ (OutcomeP _)) = Nothing

||| The projected head type a binding carries, if its kind can.
public export
bindingTy : Binding -> Maybe CardType
bindingTy (MkBinding _ _ _ (ObjectP ty _ _)) = ty
bindingTy (MkBinding _ _ _ PlayerP) = Nothing
bindingTy (MkBinding _ _ _ QualityP) = Nothing
bindingTy (MkBinding _ _ _ (OutcomeP _)) = Nothing

||| The mention an event clause prepends for what it did — sort from
||| the clause's surface, value runtime.
public export
outcomeB : OutcomeSort -> Binding
outcomeB s = MkBinding TheD Outcome OneOf (OutcomeP s)

||| Per-row catch-alls (here and in `sameQ`/`sameVerb`): a new
||| constructor is a totality error on its missing row, never a
||| silently-False diagonal.
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

||| Singular mentions of a kind, counted — the wildcard pronoun's
||| obligation is `= 1`: zero is an unbound anaphor, two an ambiguous one
||| (the uniqueness gate the controlled language relies on). Kind
||| matching routes through `sameKind` so a new Kind cannot silently
||| count as zero.
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

||| Group mentions of a kind, counted — the plural wildcard's
||| obligation is `= 1`, the ManyOf twin of `countOnes`.
public export
countManys : Kind -> Bindings -> Nat
countManys k [] = Z
countManys k (MkBinding _ k' ManyOf _ :: bs) =
  if sameKind k k' then S (countManys k bs) else countManys k bs
countManys k (_ :: bs) = countManys k bs

||| GROUP mentions of objects, counted — `countManys` minus the parts.
||| A partitive ("two of them") is plural and is NOT a group the
||| sentence may subtract from: it is what was subtracted. The
||| complement's presupposition counts the groups only, which is why it
||| cannot be `countManys` and why the partitive needed a determiner tag
||| of its own.
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
||| part taken out of it. Both halves are the phrase's own meaning —
||| with no group there is nothing to be the rest OF, and with nothing
||| taken "the rest" is the whole group and the sentence would have
||| written "them".
public export
theRestOk : Bindings -> Bool
theRestOk bs = eqNat (countGroups bs) 1 && not (eqNat (countParts bs) Z)

||| What disposing of a partition's REMAINDER leaves behind: the group
||| mention is SPENT. Chapter twenty-six's finding, and it is one word —
||| "the rest" names what is outstanding of a group, so once the rest has
||| been placed there is nothing outstanding and a second "the rest" in
||| the same breath names nothing (`badRestDisposedTwice`). The state the
||| phrase needs was already on the binding list: dropping the group
||| makes `theRestOk` false for every later clause on the SAME path,
||| while a branch arm typed in the discourse BEFORE the disposition
||| still sees it — which is exactly what the corpus writes, the only
||| three lines with two "the rest" phrases putting them in mutually
||| exclusive if/instead/otherwise arms.
||| The parts stay: a part that was taken is a referent that moved, and
||| later text may still read it. Group mentions the sentence assembled
||| some other way go with it, which costs nothing written — no corpus
||| line reads a second group back across a remainder disposal, and the
||| move already contributed nothing of its own (`moveIntro`).
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
zoneOfGroup (MkBinding _ Object ManyOf (ObjectP _ zn _) :: bs) = zn
zoneOfGroup (_ :: bs) = zoneOfGroup bs

||| …and its projected head type, by the same walk.
public export
tyOfGroup : Bindings -> Maybe CardType
tyOfGroup [] = Nothing
tyOfGroup (MkBinding PartD _ _ _ :: bs) = tyOfGroup bs
tyOfGroup (MkBinding _ Object ManyOf (ObjectP ty _ _) :: bs) = ty
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

||| The "other" presupposition's witness search, over the head types
||| the phrase OFFERS. The empty set is the genuinely untyped head (Arc
||| Trail's "any other target", a player-kind "other") and accepts any
||| same-kind anchor, which is exactly `anyTargeted`; a written head
||| demands a type-compatible anchor. A COORDINATED head offers one
||| type per alternative rather than none: "another target creature or
||| land" is anchored by an earlier creature or by an earlier land, and
||| by an earlier artifact it is not (`badDisjunctiveOtherCrossHead`).
||| Reading its silence as "untyped" was the same mistake finding 50
||| found in `DamageableTy`.
||| SOME listed head type has a compatible anchor. Its own empty list is
||| the exhausted search, not an untyped head — the two readings of `[]`
||| have to stay apart, or every typed head would fall through to
||| accepting anything (`badOtherCrossHead` caught exactly that).
public export
anchorFoundSome : Kind -> List CardType -> Bindings -> Bool
anchorFoundSome k [] ctx = False
anchorFoundSome k (t :: ts) ctx = anyTargetedTy k t ctx || anchorFoundSome k ts ctx

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

||| WHICH way an exposure clause shows a card, and it is the whole of
||| the axis: [CR#701.20a] has revealing "show that card to all
||| players", and [CR#701.20e] has looking follow "the same rules as
||| revealing a card, except that the card is shown only to the
||| specified player". So one operation, two audiences, and the
||| specified player is the clause's own subject — which is why the
||| audience is not a slot: "look at" says the looker sees it and
||| "reveal" says everyone does, and no card writes a third answer.
|||
||| Core draws the line in the same place from the other side, with one
||| `Reveal { what, to }` whose optional `to` "names a player instead =
||| 'look at'" (`deckmaste_core/src/action.rs`). A tag here rather than
||| an optional player, because English chooses a VERB and the verb
||| carries no second player: an audience slot would let the grammar
||| write "reveal it to target opponent", which oracle does not.
public export
-- spelling: (construction-owned catalog -- each row is the exposure clause's
-- own verb: LookAt = "<subject> look(s) at <what>", Reveal = "<subject>
-- reveal(s) <what>". Never spelled alone -- see Expose)
data ExposeVerb = LookAt | Reveal

||| Which zones English EXPOSES whole. Only the hand: a graveyard, the
||| battlefield, and exile are public already ([CR#400.2]), so revealing
||| one says nothing and no line does it; a library is hidden but too
||| big to show, and "reveal your library" is zero lines — what oracle
||| exposes of a library is a positioned SLICE, which is the card
||| complement and not this row. Full rows, so a new zone declares
||| whether it can be shown whole.
public export
exposableZone : Zone -> Bool
exposableZone Hand = True
exposableZone Battlefield = False
exposableZone Graveyard = False
exposableZone Exile = False
exposableZone Library = False

public export
data ExposableZone : Zone -> Type where
  MkExposableZone : {auto 0 ok : exposableZone z = True} -> ExposableZone z

||| Which zones English SEARCHES. [CR#701.23a] defines the action over
||| any zone and says the hidden ones expressly ("even if it's a hidden
||| zone"), and the corpus writes three: the library (eight hundred
||| twenty-five lines), the graveyard (ninety-five), and the hand
||| (twenty-one) — two of them hidden, which is the case the rule calls
||| out, and one public. The battlefield and exile are zero lines each — there is nothing
||| to search for in a zone everyone can already read. Full rows.
public export
searchableZone : Zone -> Bool
searchableZone Library = True
searchableZone Graveyard = True
searchableZone Hand = True
searchableZone Battlefield = False
searchableZone Exile = False

public export
data SearchableZone : Zone -> Type where
  MkSearchableZone : {auto 0 ok : searchableZone z = True} -> SearchableZone z

||| Colon-readability of one mention: tracked objects by zone
||| visibility; players, qualities, and untracked objects pass —
||| structurally, since only `ObjectP` has a zone at all.
public export
pubB : Binding -> Bool
pubB (MkBinding _ _ _ (ObjectP _ (Just z) _)) = publicZone z
pubB (MkBinding _ _ _ (ObjectP _ Nothing _)) = True
pubB (MkBinding _ _ _ PlayerP) = True
pubB (MkBinding _ _ _ QualityP) = True
pubB (MkBinding _ _ _ (OutcomeP _)) = True  -- what happened is a public fact

||| The cost boundary's filter: a mention a cost leaves in a hidden
||| zone is unreadable past the colon; unmoved mentions (a tapped cost
||| creature) and publicly-moved ones survive. [CR#400.7] fires only on
||| a zone change and [CR#400.7j] is its public-zone exception, so the
||| filter keys on the CURRENT zone, not on having moved.
public export
publicOnly : Bindings -> Bindings
publicOnly [] = []
publicOnly (b :: bs) = if pubB b then b :: publicOnly bs else publicOnly bs

||| A shuffle's filter: a mention still lying in the shuffled library is
||| gone from the discourse. [CR#701.24a] leaves the order known to no
||| player and [CR#701.20d] makes any revealed card that was reordered
||| "stop being revealed and become a new object" — so what the sentence
||| could name a moment ago is not there to be named. Mentions that
||| LEFT the library first are untouched, which is both what [CR#701.24b]
||| says of the found cards and what every search sentence relies on.
||| The filter keys on the CURRENT zone, `publicOnly`'s discipline
||| applied to a different question.
public export
notInLibrary : Binding -> Bool
notInLibrary (MkBinding _ _ _ (ObjectP _ (Just z) _)) = not (sameZone z Library)
notInLibrary (MkBinding _ _ _ (ObjectP _ Nothing _)) = True
notInLibrary (MkBinding _ _ _ PlayerP) = True
notInLibrary (MkBinding _ _ _ QualityP) = True
notInLibrary (MkBinding _ _ _ (OutcomeP _)) = True

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
zoneOfIt (MkBinding det Object OneOf (ObjectP ty zn _) :: bs) = zn
zoneOfIt (b :: bs) = zoneOfIt bs

||| The current zone of the plural wildcard's group referent.
public export
zoneOfThem : Bindings -> Maybe Zone
zoneOfThem [] = Nothing
zoneOfThem (MkBinding det Object ManyOf (ObjectP ty zn _) :: bs) = zn
zoneOfThem (b :: bs) = zoneOfThem bs

||| The noun-word vocabulary — ONE set of words for the sorted reads,
||| its axes kept apart (finding 27 — type words are declared catalog
||| atoms, never intrinsic sorts; the intrinsic words are the engine's
||| own). What differs per read is the ANCHORING: the demonstrative
||| checks its word against the referent's current state (`wordNow`),
||| the participle against the verb event's frame (`verbedMatch`).
||| Token, spell, and stack-object words are later chapters.
public export
-- spelling: (construction-owned catalog -- TypeW t = t's own CardType word,
-- CardW = "card", PlayerW = "player"; consumed by That/Those/TheVerbed, e.g.
-- That (TypeW Creature) = "that creature". Never spelled alone)
data NounWord = TypeW CardType | CardW | PlayerW

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

||| Build the stamp a retag writes: the moving verb's tag (if any)
||| plus whether the referent stood on the battlefield BEFORE the
||| move — the at-verb frame.
public export
mkStamp : Maybe VerbName -> Maybe Zone -> Maybe Stamp
mkStamp Nothing oldZn = Nothing
mkStamp (Just v) oldZn = Just (MkStamp v (onFieldZone oldZn))

||| The demonstrative's noun check — CURRENT-state anchoring, the
||| carrier discipline ([CR#109.2,110.1]): a type word demands the
||| referent currently answer to it (on the battlefield, projected
||| type matching), the CARD word a tracked non-battlefield object
||| ([CR#108.2]), the PLAYER word a player. Quality mentions answer
||| to no noun word.
public export
wordNow : NounWord -> Binding -> Bool
wordNow (TypeW t) (MkBinding _ _ _ (ObjectP ty zn _)) = onFieldZone zn && tyIs t ty
wordNow (TypeW t) (MkBinding _ _ _ PlayerP) = False
wordNow (TypeW t) (MkBinding _ _ _ QualityP) = False
wordNow (TypeW t) (MkBinding _ _ _ (OutcomeP _)) = False
wordNow CardW (MkBinding _ _ _ (ObjectP _ zn _)) = isCardZone zn
wordNow CardW (MkBinding _ _ _ PlayerP) = False
wordNow CardW (MkBinding _ _ _ QualityP) = False
wordNow CardW (MkBinding _ _ _ (OutcomeP _)) = False
wordNow PlayerW (MkBinding _ _ _ (ObjectP _ _ _)) = False
wordNow PlayerW (MkBinding _ _ _ PlayerP) = True
wordNow PlayerW (MkBinding _ _ _ QualityP) = False
wordNow PlayerW (MkBinding _ _ _ (OutcomeP _)) = False

public export
kindOfW : NounWord -> Kind
kindOfW (TypeW _) = Object
kindOfW CardW = Object
kindOfW PlayerW = Player

||| The PROVENANCE half of the participle read: is this mention the
||| one the named verb event stamped? That is the whole of what the
||| participle contributes as a determiner — "the sacrificed …" picks
||| out the referent of the LAST sacrifice (finding 26), and it says
||| nothing about which word may then describe it. Core keeps the same
||| axis to itself: `Reference::Bound`/`Linked` name a mention by the
||| role or the remembered link, where `Reference::That(Sort)` names
||| one by its sort (`reference.rs`).
public export
stampedBy : VerbName -> Stamp -> Bool
stampedBy v (MkStamp v' _) = sameVerb v v'

||| The WORD half, over that same stamped mention: a TYPE word demands
||| the referent stood on the battlefield AT the verb (the stamp's
||| `wasField` — a bare type word denotes a permanent [CR#109.2], so
||| "the discarded creature" is unwritten; hands lose cards, not
||| creatures) plus its projected type; the intrinsic CARD word checks
||| the CURRENT zone; no participle reads a player. Each row reads the
||| word's own anchor and nothing about the verb, which is the axis
||| separation: the same three words serve the demonstrative anchored
||| to the current state instead (`wordNow`, finding 28).
public export
verbedWordOk : NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
verbedWordOk (TypeW t) (MkStamp _ wasF) ty zn = wasF && tyIs t ty
verbedWordOk CardW st ty zn = isCardZone zn
verbedWordOk PlayerW st ty zn = False

||| The participle's two halves as the one check the scans want: the
||| provenance picks the mention, the word describes it.
public export
stampWordOk : VerbName -> NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
stampWordOk v w st ty zn = stampedBy v st && verbedWordOk w st ty zn

||| Does "the [verbed] [noun]" reach this binding? Singular, stamped,
||| noun word compatible (`stampWordOk`).
public export
verbedMatch : VerbName -> NounWord -> Binding -> Bool
verbedMatch v w (MkBinding _ _ OneOf (ObjectP ty zn (Just st))) = stampWordOk v w st ty zn
verbedMatch v w (MkBinding _ _ OneOf (ObjectP _ _ Nothing)) = False
verbedMatch v w (MkBinding _ _ ManyOf (ObjectP _ _ _)) = False
verbedMatch v w (MkBinding _ _ _ PlayerP) = False
verbedMatch v w (MkBinding _ _ _ QualityP) = False
verbedMatch v w (MkBinding _ _ _ (OutcomeP _)) = False

||| Mentions the definite participle read reaches — its obligation is
||| `= 1`, the same strict uniqueness as every other read.
public export
countVerbed : VerbName -> NounWord -> Bindings -> Nat
countVerbed v w [] = Z
countVerbed v w (b :: bs) =
  if verbedMatch v w b then S (countVerbed v w bs) else countVerbed v w bs

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
tyOfIt (MkBinding det Object OneOf (ObjectP ty zn pv) :: bs) = ty
tyOfIt (b :: bs) = tyOfIt bs

public export
tyOfThem : Bindings -> Maybe CardType
tyOfThem [] = Nothing
tyOfThem (MkBinding det Object ManyOf (ObjectP ty zn pv) :: bs) = ty
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
||| ([CR#109.2], `nounZone (AsType …)`); bare `This` is the source as an
||| object and never denotes a permanent, so nothing legal needs it.
||| The controller half needs fold-state the context does not carry
||| (not-settled).
public export
data OnBattlefield : Maybe Zone -> Type where
  OnField : OnBattlefield (Just Battlefield)

||| A reference's OWN zone against the zone its description reads —
||| `zonesAgree`'s agreement rule pointed at the one place the subject
||| is external to the phrase. A noun phrase places its referent itself,
||| so "creature card in your graveyard" has its zone story checked
||| inside the conjunction; a copular condition instead says something
||| about a referent placed somewhere else already, and "it's in a
||| graveyard" said of a battlefield mention describes nothing
||| (a description naming a zone means an object in THAT zone,
||| [CR#109.2a], so it picks out nothing the subject could be).
||| Silence on either side is no evidence and passes: an untracked
||| subject answers for nothing, and a description that names no zone
||| asks nothing of one.
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
||| object that's not a battle, a creature, or a planeswalker): one
||| row, for the one damageable type this vocabulary has a word for.
||| The class word "any target" does not arrive here — it names the
||| [CR#115.4] class itself and has its own recipient row — and the
||| untyped head no longer arrives EITHER. That row read a missing
||| type word as "no evidence of an illegal one", which was defensible
||| while nothing could project `Nothing` deliberately; a disjunctive
||| head does exactly that, and honestly ("artifact or enchantment"
||| fixes no type), so keeping the row would have made every
||| disjunction damageable — including the two types [CR#120.1a] names
||| as the ones damage can't reach (`badDamageDisjunctHead`). Reading
||| the phrase's silence as permission is what had to go. New rows
||| arrive with their types' declarations.
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
||| `Nothing` in the plain form), never as defaults.
public export
-- spelling: ["haste", "flying", "trample"] (row order: Haste/Flying/Trample),
-- kind: KeywordLine (bare keyword-ability line, no params/cost -- contrast
-- Madness.ron's parameterized "madness <Param(0)>")
data Keyword = Haste | Flying | Trample

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
||| hole in the rules rather than an unattested phrase. Core's sibling
||| `ColorOrColorless` is not ported: its whole job is the mana-symbol
||| channel (`from_code` reads the symbol letter "C"), which is a later
||| round, and the token's empty color list is where English writes
||| "colorless" here.
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

||| Color equality, per-row — `sameZone`'s discipline, so a new color is a
||| totality error rather than a silent `False`.
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
||| Idris spec agrees row for row (`Semantics.idr`). This is the second
||| closed vocabulary the workbench does NOT whittle to witnessed rows: a
||| mana symbol is a rules-fixed catalog and a missing row would be a hole
||| in the rules rather than an unattested phrase.
public export
-- spelling: (component-owned -- never spelled alone. Written inside a
-- ManaSymbol's braces: Generic n as the numeral "{2}", Specific as the
-- one-letter code "{W}"/"{C}"; the halves of a hybrid are joined by "/")
data SimpleManaSymbol = Generic Nat | Specific ColorOrColorless

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
data ManaSymbol = Simple SimpleManaSymbol
                | Hybrid SimpleManaSymbol Color
                | Phyrexian Color (Maybe Color)
                | Variable
                | SnowMana

||| A printed mana cost: the symbol sequence, in the order the card writes
||| it ([CR#202.1] — "a card's mana cost is indicated by mana symbols";
||| [CR#202.1a] makes paying it a per-symbol match). A LIST because that is
||| all oracle's spelling is, "{1}{R}" being two symbols run together, and
||| because core's `ManaCost` is the same newtype over a symbol slice. The
||| empty list is the {0} cost's neighbour and not the same thing:
||| [CR#118.5] makes "{0}" a payment of nothing, written `[Simple (Generic
||| 0)]`, where the empty list is core's "no mana cost" ([CR#202.1b],
||| unpayable) — a distinction the type keeps for free and no clause here
||| exercises.
public export
ManaCost : Type
ManaCost = List ManaSymbol

||| Subtypes, as catalog atoms ([CR#205.3m] — creatures and kindreds
||| share one open list of creature types). Witnessed rows, and
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
             | Coward | Demon | Plant | Dragon

||| Subtype equality, per-row catch-alls — `sameVerb`'s discipline.
public export
sameSub : Subtype -> Subtype -> Bool
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
sameSub Plant Plant = True
sameSub Plant _ = False
sameSub Dragon Dragon = True
sameSub Dragon _ = False

||| Which card type a subtype's set belongs to — [CR#205.1a] names the
||| sets themselves (creature types, land types, artifact types,
||| enchantment types, planeswalker types, spell types) and [CR#205.3m]
||| lists the creature types, which is every row here. Single-valued
||| because [CR#205.3m]'s set is shared by creature and kindred alone and
||| this vocabulary has no kindred row, so the creature answer is exact
||| for what English spells here.
||| Full rows: a subtype from another set (an artifact type like
||| Equipment, an enchantment type like Aura) is a totality error that
||| must declare its card type before anything can be written with it.
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
subtypeType Plant = Creature
subtypeType Dragon = Creature

||| Counter kinds ([CR#122.1] — "a marker placed on an object or player
||| that modifies its characteristics and/or interacts with a rule,
||| ability, or effect"). Core spells the kind as an open bare-identifier
||| reference into a plugin registry (`CounterRef`,
||| `deckmaste_core/src/counter.rs`), which is the same open-catalog shape
||| its subtypes take and the same one this file answers with witnessed
||| rows. The two stat counters are first-class because they lead the
||| corpus by an order of magnitude — "put a +1/+1 counter on" runs to
||| one thousand four hundred ninety-three lines and "put N +1/+1
||| counters on" to four hundred thirty-one, against eighty-eight and
||| twenty for -1/-1 — and each carries its own rule ([CR#122.1a]). A
||| NAMED counter earns a row only where a corpus line writes it as a
||| one-shot put or remove, which is why `Stun` is here (fifty-six lines;
||| Kaito, Bane of Nightmares' "Tap target creature. Put two stun counters
||| on it.", its replacement identity being [CR#122.1d]'s) and charge,
||| age, quest, loyalty, oil, level and the rest are not — their lines are
||| costs, upkeep triggers, and enters-with riders, which are other axes.
public export
-- spelling: ["+1/+1", "-1/-1", "stun"] (row order: PlusOnePlusOne/
-- MinusOneMinusOne/Stun -- the kind's own word, written between the count and
-- the noun "counter(s)"; never spelled alone)
data CounterKind = PlusOnePlusOne | MinusOneMinusOne | Stun

||| Counter-kind equality, per-row catch-alls.
public export
sameCounter : CounterKind -> CounterKind -> Bool
sameCounter PlusOnePlusOne PlusOnePlusOne = True
sameCounter PlusOnePlusOne _ = False
sameCounter MinusOneMinusOne MinusOneMinusOne = True
sameCounter MinusOneMinusOne _ = False
sameCounter Stun Stun = True
sameCounter Stun _ = False

||| The TYPE LINE a token defines and a type-addition clause adds
||| ([CR#205.1] — the line carries the card types and the subtypes).
||| ONE record for both readers, because English writes one phrase in one
||| order for both: "0/0 black Zombie Army creature token" and "becomes a
||| Spirit artifact creature in addition to its other types" put the
||| subtypes before the types alike, and the two clauses differ in what
||| surrounds the line, not in the line. Core keeps the halves apart at
||| both sites (`Token`'s `types`/`subtypes` fields, `Modification`'s
||| `CardTypes`/`Subtypes` ops) and this record is those two lists under
||| one name. Supertypes are a third list neither reader witnesses here
||| (ledger).
public export
record TypeLine where
  constructor MkTypeLine
  subs : List Subtype
  tys : List CardType

||| Is a card type among a line's types?
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

||| Every subtype in a line sits on a card type the same line names — the
||| [CR#205.1a] set-membership fact as a check on the phrase: "Zombie Army
||| creature token" writes the creature type its two subtypes belong to,
||| and a "Zombie artifact token" would name a subtype the object's types
||| cannot carry (`badZombieArtifactToken`).
public export
subsFitLine : List Subtype -> List CardType -> Bool
subsFitLine [] tys = True
subsFitLine (s :: ss) tys = lineHasType (subtypeType s) tys && subsFitLine ss tys

||| A token's defined characteristics ([CR#111.3] — "the spell or ability
||| that creates a token may define the values of any number of
||| characteristics for the token … A token doesn't have any
||| characteristics not defined by the spell or ability that created
||| it"), in the fixed adjective order oracle writes them: power and
||| toughness, colors, the type line, then the noun "token", then the
||| with-clause, then the name. Fire Navy Trebuchet spells the whole
||| order in one phrase — "a 2/1 colorless Construct artifact creature
||| token with flying named Ballistic Boulder" — which is what fixes the
||| with-before-named half nothing shorter could.
|||
||| Core's `Token` (`deckmaste_core/src/token.rs`) field for field, minus
||| what no corpus line here needs: supertypes (ledger) and the abilities
||| beyond bare keywords — the with-clause slot holds `Keyword` itself
||| rather than the `Ability` container, which is what every corpus token
||| line writes ("with flying"). The QUOTED form is real English and
||| ledgered with its count ("create a 1/1 red Mercenary creature token
||| with \"{T}: …\"", seven lines), and it is a quotation the container has
||| no row for; narrowing the field is what keeps the container from
||| admitting it silently. Color rides a LIST, exactly as core's
||| `color_indicator` does, and the empty list is where English writes the
||| word "colorless" ([CR#105.2c] — a colorless object has no color).
||| The name is a `Maybe`: [CR#111.4] synthesizes an unnamed token's name
||| from its subtypes, so the slot is written only when the creating
||| effect states one ("named Kobolds of Kher Keep").
public export
record TokenChars where
  constructor MkToken
  pt : Maybe (Nat, Nat)
  colors : List Color
  line : TypeLine
  abilities : List Keyword
  name : Maybe String

||| A token is a PERMANENT ([CR#111.1] — "a marker used to represent any
||| permanent that isn't represented by a card"), so its line names at
||| least one card type. The corpus agrees for every inline definition;
||| the type-less spelling is the PREDEFINED name ("create a Treasure
||| token", [CR#111.10]), which core gives its own `TokenSpec::Named` row
||| and which waits here with the predefined catalog (ledger).
public export
tokenTyped : TokenChars -> Bool
tokenTyped t = lineNonEmpty (MkTypeLine [] t.line.tys)

||| A CREATURE token writes power and toughness — the creature's own two
||| numbers ([CR#208.1]), which a token's creating effect has to define
||| because no card prints them for it ([CR#111.3]), and which every
||| corpus creature-token line does in fact write. The
||| converse is NOT demanded: a Vehicle token carries a P/T without the
||| creature type ("a 3/2 colorless Vehicle artifact token with crew 1",
||| [CR#301.7]), so a noncreature token's slot is left free rather than
||| forced empty — a one-directional table, and the direction is the one
||| the corpus fixes (`badCreatureTokenNoPt`).
public export
tokenPtOk : TokenChars -> Bool
tokenPtOk t = case (lineHasType Creature t.line.tys, t.pt) of
  (True, Nothing) => False
  _ => True

||| The three token demands as witnesses, named apart so a pin says which
||| one refused (the `Headed`/`FightParticipant` discipline).
public export
data TokenTyped : TokenChars -> Type where
  MkTokenTyped : {auto 0 ok : tokenTyped t = True} -> TokenTyped t

public export
data TokenPt : TokenChars -> Type where
  MkTokenPt : {auto 0 ok : tokenPtOk t = True} -> TokenPt t

public export
data SubtypesFit : TokenChars -> Type where
  MkSubtypesFit : {auto 0 ok : subsFitLine t.line.subs t.line.tys = True} ->
                  SubtypesFit t

||| Where a card type stands in a type line — a rank rather than a
||| comparison table, because the corpus fixes a TOTAL order over these
||| four words and a rank is that order written once. Measured pairwise
||| over supported oracle: "artifact creature" five hundred ninety-four
||| lines against "creature artifact" none, "artifact land" four against
||| none, "land creature" eleven against none (Dryad Arbor's token,
||| "1/1 green Forest Dryad land creature token"), "enchantment
||| creature" thirty-three, and "enchantment artifact creature" the one
||| line that places enchantment ahead of artifact. The Enchantment/Land
||| pair is written neither way and rides on transitivity, which is what
||| a rank buys and a pair table would have had to guess at. The one
||| counterexample is the Licid template's "becomes a creature
||| enchantment … instead of a creature" (Flanking Licid), a
||| pre-standardization wording against thirty-three the other way.
public export
typeRank : CardType -> Nat
typeRank Enchantment = 0
typeRank Artifact = 1
typeRank Land = 2
typeRank Creature = 3

||| Strictly ascending by rank — which is the ORDER and the
||| duplicate-freeness at once, a repeated word being the one thing a
||| strict order cannot admit. [CR#111.3] makes a token's stated
||| characteristics its text, so "creature artifact token" and "white
||| white Soldier" are not two spellings of a bundle but two bundles
||| oracle never writes (`badTokenTypeOrder`, `badTokenDuplicateColor`).
public export
ltNat : Nat -> Nat -> Bool
ltNat a b = leNat (S a) b

public export
typesOrdered : List CardType -> Bool
typesOrdered [] = True
typesOrdered (t :: []) = True
typesOrdered (t :: u :: ts) = ltNat (typeRank t) (typeRank u) &&
                              typesOrdered (u :: ts)

||| Colors are duplicate-free but NOT ordered, and the corpus is why:
||| Additive Evolution writes "a 0/0 green and blue Fractal creature
||| token", which the mana order would have spelled the other way round.
||| So the demand is only that no color is written twice ([CR#105.2] —
||| an object "can be one or more of the five colors, or … no color at
||| all", which is a property it has or lacks and never counts).
public export
colorMember : Color -> List Color -> Bool
colorMember c [] = False
colorMember c (d :: ds) = sameColor c d || colorMember c ds

public export
colorsDistinct : List Color -> Bool
colorsDistinct [] = True
colorsDistinct (c :: cs) = not (colorMember c cs) && colorsDistinct cs

||| The token bundle's surface form as one witness: its colors written
||| once each, its type line in the order oracle writes it.
public export
tokenCanonical : TokenChars -> Bool
tokenCanonical t = colorsDistinct t.colors && typesOrdered t.line.tys

public export
data TokenCanonical : TokenChars -> Type where
  MkTokenCanonical : {auto 0 ok : tokenCanonical t = True} -> TokenCanonical t

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
||| gains, so a clause that states only what the subject already is
||| states nothing: "target creature becomes a creature in addition to
||| its other types" is no instruction, and the corpus writes no line
||| where the added type is the subject's own head (Tezzeret's adds
||| creature to an ARTIFACT, Neurok Transmuter's adds artifact to a
||| CREATURE). The subject's head is the only thing the discourse knows
||| about it, so that is what "already" can mean here: a SUBTYPE is
||| never provably redundant (no mention carries its subtypes) and an
||| untyped subject entails nothing, both of which pass — the gate
||| under-refuses in `predEq`'s direction. [CR#701.47a] guards the
||| subtype case in the text instead, with a condition ("If it isn't a
||| [subtype], …") rather than a grammar rule (`badBecomesOwnType`).
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

public export
tokenHeadTy : TokenChars -> Maybe CardType
tokenHeadTy t = lastType t.line.tys

||| The parts of the turn a duration can name its endpoint by — the
||| turn itself and the steps and phases inside it. Deliberately NOT a
||| mirror of core's `PhaseStep` (`deckmaste_core/src/event.rs`, whose
||| `Beginning`/`Combat`/`Ending` trees enumerate all thirteen): these
||| are the parts an ENDPOINT is written against, and the corpus writes
||| a duration-class adverbial against only these. The draw step and
||| the two main phases carry none at all (ledger), so they have no row
||| here to answer for. This is the SHARED vocabulary — the trigger
||| headers the later chapter must spell ("at the beginning of your
||| upkeep") name the same parts, and a second enum would drift from
||| this one — so a new row is a totality error on `spanUse` and on
||| every table the triggers chapter adds.
public export
-- spelling: ["turn", "upkeep", "end step", "combat", "untap step"] (row
-- order: Turn/Upkeep/EndStep/Combat/UntapStep; the bare part word --
-- DurationEnd's boundary and possession supply everything around it),
-- kind: TODO(reason: adverbial fragment -- not one of Nominal/Sentence/
-- Cost/KeywordLine/Ability)
data TurnPart = Turn | Upkeep | EndStep | Combat | UntapStep

||| Whose part a possessed endpoint names. CLOSED and pronominal: the
||| corpus possesses a duration endpoint with a possessive DETERMINER
||| and nothing else — "your next turn", "that player's next end step" —
||| so this is a two-word vocabulary, not a noun slot. A noun-valued
||| possessor would make `Duration` bindings-indexed (every consumer
||| re-indexed) to buy a form no construction here writes; the one line
||| that needs one — "Target land becomes a Swamp until its controller's
||| next untap step" — is a base-type SETTING clause, a layer word this
||| grammar has no construction for, so its possessor waits with it
||| (ledger). `ThatPlayers` is the anaphoric row: it presupposes a
||| unique player antecedent the way `They` does, an obligation no
||| current table's cell opens (`spanUse`), so the demand itself waits
||| for the cell that needs it.
public export
-- spelling: ["your", "that player's"] (row order: Yours/ThatPlayers; the
-- possessive determiner alone, always followed by "next" -- see
-- DurationEnd), kind: TODO(reason: adverbial fragment)
data Whose = Yours | ThatPlayers

||| The endpoint an "until" adverbial names: a boundary of a turn part,
||| optionally possessed. Two axes, because the corpus writes both
||| independently — "until end of turn" against "until your next turn"
||| is the SAME part at opposite boundaries, and each boundary takes
||| possession or not.
|||
||| "Next" is DERIVABLE, not a parameter: a possessed endpoint is
||| always the next one ("your next turn" — there is no "your previous
||| upkeep" to distinguish it from), and a bare endpoint is always the
||| current turn's. The article goes with the possession the same way
||| ("until end of turn" has none; "until the end of your next turn"
||| keeps it), and the possessive can even land outside the part it
||| possesses — Brazen Cannonade writes "until end of combat on your
||| next turn", not a possessed combat. All of that is construction-
||| assigned surface for the spelling layer, which is why none of it is
||| structure here.
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
||| assigns neither ("Use `until end of turn` for the ordinary
||| current-turn duration" sits a section away from "Use `this turn` for
||| the current turn"), so the counts are the whole evidence.
|||
||| Two kinds of `False` are worth telling apart, so they are separate
||| rows: `Unattested` means no corpus line writes the phrase at all,
||| while `Unclaimed` means the phrase is real oracle English that this
||| grammar's constructions do not write — the endpoint exists, its
||| clause is somewhere else (a play permission, a base-P/T setting).
||| The rest name the observed splits, and they are strikingly clean:
||| the current-turn adverbials divide the grants from the restrictions
||| exactly, and the cross-turn span is the only one both families
||| write.
|||
||| Chapter nineteen SPLIT one of these rows rather than joining it. The
||| type-addition clause writes "until end of turn" and never "until end
||| of combat", so a class that named both endpoints at once could not
||| hold a fourth construction's answer: the two current-turn GRANT
||| endpoints, which three constructions could not tell apart, are two
||| classes as soon as a fourth writes one of them and not the other.
|||
||| Chapter twenty-two is the same event a second time, and it renamed
||| where chapter nineteen split. The CONTROL grant writes both
||| current-turn endpoints, so it joins two existing classes rather than
||| dividing them — `BothGrants` and `GrantsAndTypes` stopped being true
||| of themselves the way `EveryStatic` had, and were renamed for it.
||| What chapter twenty-two DID divide is `Unclaimed`:
||| "until the end of your next turn" was eighty-three lines of play
||| permissions and control grants with no construction to claim it, and
||| four of those lines are naked control grants, so the cell is
||| `ControlGrantOnly` and the class is the control grant's alone. The
||| ninth row is the new adverbial's own (`GrantsRestrictionsAndControl`
||| — everything but the type addition), and the end-step cells stay
||| `Unclaimed` on a measurement rather than an assumption: the one
||| corpus line pairing control with an end step writes it as a DELAYED
||| clause ("An opponent gains control of this land at the beginning of
||| the next end step"), which is not a duration adverbial at all.
||| Chapter twenty-five renamed three cells and split none. The two
||| SHIELD constructions — the replacement interception and the
||| prevention shield — both write "this turn", so the restriction's own
||| current-turn word stops being the restriction's alone
||| (`RestrictionsAndShields`); the interception ALSO writes the two
||| cross-clause endpoints the restriction and the grants divide
||| ("Until end of turn, if one or more tokens would be created under
||| your control, twice that many of those tokens are created instead",
||| seven lines; "Until your next turn, if that creature would deal
||| combat damage to one of your opponents, it deals triple that damage
||| to that player instead", two), so those two cells gain a name rather
||| than a row. Prevention writes NOTHING else: two hundred fifty-five
||| prevention lines carry "this turn" and not one carries "until end of
||| turn" or any possessed endpoint, which is the sharpest single-word
||| answer in the table.
public export
data SpanUse = Unattested | Unclaimed | GrantsAndControl | GrantsTypesControlAndReplacement
             | KeywordGrantOnly | RestrictionsAndShields | GrantsRestrictionsAndReplacement
             | ControlGrantOnly | GrantsRestrictionsAndControl

-- ===== Event patterns (the vocabulary the "would" clause and the
-- "until" clause share) =====

||| The events oracle names when it intercepts one or waits for one —
||| the vocabulary two constructions read and no rule enumerates. Closed
||| and row-enumerated, so a new event is a totality error on every
||| table below before anything can be written with it.
|||
||| Which events are HERE is the corpus's answer. Counted over the whole
||| supported corpus, with the "would" mood and the plain mood counted
||| separately because they are two constructions: "would die" a hundred
||| and one lines against no "until … dies" at all; "would leave the
||| battlefield" fifty-three against "until … leaves the battlefield"
||| ninety-one; "would be destroyed" thirty-one; "would be dealt
||| [damage]" three hundred eighty-five; "would draw" fifty. Everything
||| below that is ledgered with its count rather than minted — the
||| put-into-a-graveyard event (fifty-seven), the enter event (fourteen),
||| the create event (thirteen), the life-change pair (eight and one) —
||| because no construction here writes a clause over them.
|||
||| Core spells this space as ONE open filter type (`EventFilter`,
||| `deckmaste_core/src/event.rs`) shared by triggers, replacements,
||| durations, and condition lookbacks. That is right for an engine and
||| premature here: the trigger container does not exist yet, and the
||| two constructions that DO exist disagree about which events they
||| write — which is what the tables below record and a single filter
||| type could not.
public export
-- spelling: (construction-owned -- each row is a VERB PHRASE whose subject
-- and mood the reading construction supplies: the interception writes it
-- after "would" ("would die", "would be destroyed"), the held-until rider
-- writes it finite ("leaves the battlefield"). Spelled only through
-- GameEvent)
data EventName = Dies | LeavesBattlefield | IsDestroyed | IsDealtDamage
               | DrawsCard

||| WHICH constructions write a clause over a given event — `SpanUse`'s
||| shape asked of the event axis, and it tells the same two silences
||| apart. `EventUnattested` means no corpus line writes any clause over
||| the event in either mood; `EventUnclaimed` means the clause is real
||| oracle English and no construction HERE writes it.
|||
||| The two claimed rows and the two unclaimed ones are worth their
||| counts. `Dies` is `InterceptedOnly`: fifty-seven lines write "if
||| [subject] would die this turn, [replacement] instead" and
||| twenty-four of them write it in exactly one frame ("If that creature
||| would die this turn, exile it instead"), while nothing anywhere ends
||| a duration at a death. `LeavesBattlefield` is `HeldUntilOnly` from
||| the other side: eighty-six lines hang the [CR#610.3] rider on it and
||| its interception exists only INSIDE granted quoted abilities ("It
||| gains 'If this creature would leave the battlefield, exile it
||| instead'"), a container this grammar has no word for.
||| `IsDestroyed` is `EventUnclaimed` because its replacement is
||| regeneration's four-part instruction ([CR#614.8] — tap, remove from
||| combat, heal), none of which this vocabulary writes; `IsDealtDamage`
||| is `EventUnclaimed` because its replacement is the redirection
||| family ([CR#614.9] — "that damage is dealt to [other] instead"),
||| which wants a damage clause whose amount is the intercepted event's.
public export
data EventUse = EventUnattested | EventUnclaimed | InterceptedOnly
              | HeldUntilOnly | InterceptedAndHeld

public export
eventUse : EventName -> EventUse
eventUse Dies = InterceptedOnly
eventUse LeavesBattlefield = HeldUntilOnly
eventUse IsDestroyed = EventUnclaimed
eventUse IsDealtDamage = EventUnclaimed
eventUse DrawsCard = InterceptedOnly

||| May the would/instead clause intercept an event of this class? Full
||| rows in the answer axis, so a new `EventUse` declares both readers.
public export
admitsIntercept : EventUse -> Bool
admitsIntercept EventUnattested = False
admitsIntercept EventUnclaimed = False
admitsIntercept InterceptedOnly = True
admitsIntercept HeldUntilOnly = False
admitsIntercept InterceptedAndHeld = True

||| May a [CR#610.3] zone-change rider wait for an event of this class?
public export
admitsHold : EventUse -> Bool
admitsHold EventUnattested = False
admitsHold EventUnclaimed = False
admitsHold InterceptedOnly = False
admitsHold HeldUntilOnly = True
admitsHold InterceptedAndHeld = True

||| Which duration-adverbial class an event-ended "until" phrase falls
||| in — the event axis's own row of chapter seventeen's attestation
||| table, and the finding is that every row is a silence.
|||
||| `LeavesBattlefield` is `Unclaimed` and the composition is why. The
||| ninety-one lines that write "until [object] leaves the battlefield"
||| are eighty-six zone changes ([CR#610.3] — a one-shot exile that
||| schedules its own undo, NOT a continuous effect) and three phasings
||| ([CR#610.4], the same shape), leaving three genuine [CR#611.2a]
||| continuous durations: two base-TYPE settings ("Target land becomes a
||| Forest until this creature leaves the battlefield") and one
||| becomes-a-copy. All three are constructions this grammar lacks, so
||| the cell names them and claims nothing. Every other event is
||| `Unattested`: no corpus line ends a duration at a death, a
||| destruction, a damage event, or a draw — the twenty-three lines that
||| appear to write "until … dies" all cross a clause boundary ("until
||| end of turn, whenever another creature dies").
public export
eventSpan : EventName -> SpanUse
eventSpan Dies = Unattested
eventSpan LeavesBattlefield = Unclaimed
eventSpan IsDestroyed = Unattested
eventSpan IsDealtDamage = Unattested
eventSpan DrawsCard = Unattested

||| How many times an interception fires — [CR#614.3]'s two ways for a
||| replacement effect to end, "used up" against "duration expired",
||| and English marks the difference with the clause's own opening
||| word.
|||
||| The corpus assigns the word by EVENT and the split is total.
||| "The next time [subject] would die" is written zero times against
||| fifty-seven "if … would die this turn"; "if you would draw a card
||| this turn" is written zero times against nine "the next time you
||| would draw"; regeneration's own reminder text is "the next time
||| [permanent] would be destroyed this turn" ([CR#614.8]) twenty-five
||| times and never the conditional. The reason is the event's own
||| repeatability: a creature dies once, so an unlimited shield and a
||| single-use one are the same shield and English writes the shorter
||| word; a draw repeats, so the two differ and the writer must say
||| which. That makes this a gate rather than a spelling note
||| (`badNextTimeWouldDie`, `badIfWouldDraw`).
public export
-- spelling: (construction-owned -- the clause's opening word: Repeatedly
-- writes "if <event>", NextTimeOnly writes "the next time <event>".
-- Consumed by StaticEffect.Intercepts, never spelled alone)
data ReplUse = Repeatedly | NextTimeOnly

public export
replUseOk : EventName -> ReplUse -> Bool
replUseOk Dies Repeatedly = True
replUseOk Dies NextTimeOnly = False
replUseOk LeavesBattlefield Repeatedly = True
replUseOk LeavesBattlefield NextTimeOnly = False
replUseOk IsDestroyed Repeatedly = False
replUseOk IsDestroyed NextTimeOnly = True
replUseOk IsDealtDamage Repeatedly = True
replUseOk IsDealtDamage NextTimeOnly = True
replUseOk DrawsCard Repeatedly = False
replUseOk DrawsCard NextTimeOnly = True

-- ===== Prevention shields (the damage class and the shield's size) =====

||| WHICH damage a prevention shield stops. [CR#615.1] makes the shield
||| watch "a damage event that would happen", and the only qualifier
||| oracle puts on the noun is the combat/noncombat split
||| ([CR#510.2] — combat damage is what the combat damage step deals,
||| which is the whole of what the adjective names). Counts: "prevent all
||| damage" a hundred and three lines lead with it, "prevent all combat
||| damage" forty, "prevent all noncombat damage" six. Closed at three
||| because no fourth qualifier is written: no line prevents "all
||| trample damage" or "all excess damage".
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
||| to put it in, so English marks it in the VOICE: "can't block"
||| against "can't be blocked". That is an axis of its own (`Role`),
||| not a second deed word, so this enum stops at the verbs. Closed and
||| row-enumerated, and every table over it is written out, so a new
||| deed is a totality error that must declare which types carry its
||| grant, in which voice, before it can be written at all. These lead
||| the one-shot restrictions in the corpus — "can't be blocked this
||| turn" one hundred seventy-three lines, "can't block this turn" one
||| hundred fifteen, "can't attack this turn" eight — and the rest of
||| the deontic surface is either the parked ability layer or a deed
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

||| Which card types carry a deed's grant IN A GIVEN VOICE — the
||| stand-in for reading `May(Attack)`/`May(Block)` off the TypeDef
||| declaration (`plugins/builtin/macros/cardtype/Creature.ron`), and a
||| DIFFERENT table from `combatant`, which the fight chapter minted
||| precisely because fight keys on type membership and deals
||| non-combat damage ([CR#701.14b,701.14d]) where these deeds are
||| combat proper. [CR#506.3] — "Only a creature can attack or block" —
||| answers the active rows, and the passive of BLOCK too, because what
||| a blocker blocks is an attacking creature ([CR#509.1a]). The
||| passive of ATTACK is answered by the SECOND sentence of that same
||| rule: only a player, a planeswalker, or a battle can be attacked,
||| and not one of those is a card type this grammar spells. So that
||| whole row is False — and False for a reason worth
||| writing down, because the PHRASE is real oracle: "The Aetherspark
||| can't be attacked" writes it of a planeswalker, and "until your
||| next turn, you can't be attacked except by creatures with flying"
||| writes it of a player. The row waits on the planeswalker and battle
||| card types and on the ledgered player-subject restriction, not on a
||| corpus witness. Written out in every direction, `sameKind`-style,
||| so a new deed, a new voice, and a new card type are each a totality
||| error rather than a silent `False`.
public export
deedType : Deed -> Role -> CardType -> Bool
deedType Attack Agent Creature = True
deedType Attack Agent Artifact = False
deedType Attack Agent Land = False
deedType Attack Agent Enchantment = False
deedType Attack Patient Creature = False
deedType Attack Patient Artifact = False
deedType Attack Patient Land = False
deedType Attack Patient Enchantment = False
deedType Block Agent Creature = True
deedType Block Agent Artifact = False
deedType Block Agent Land = False
deedType Block Agent Enchantment = False
deedType Block Patient Creature = True
deedType Block Patient Artifact = False
deedType Block Patient Land = False
deedType Block Patient Enchantment = False

||| The deed's demand on its subject's projected head, as a witness —
||| `FightParticipant`'s shape for the combat grants, reading the voice
||| along with the verb. There is no `Nothing` row: an UNTYPED head
||| cannot prove participation, so a disjunctive subject, which
||| honestly fixes no type (finding 50), is refused rather than waved
||| through on its silence.
public export
data DeedParticipant : Deed -> Role -> Maybe CardType -> Type where
  Participant : {auto 0 ok : deedType d r t = True} -> DeedParticipant d r (Just t)

||| Which static effect a `Continuously` clause establishes, as the
||| span tables' key — the axis `spanUse` classifies its adverbials
||| against. One row per `StaticEffect` constructor (`staticKind`), so a
||| new static row is a totality error on both tables and must declare
||| which durations it writes before it can be written at all.
||| The two rows chapter twenty-five added are the first that modify no
||| OBJECT. [CR#611.2c] cuts the continuous effects in two — those that
||| "modify the characteristics or change the controller of any objects",
||| whose affected set is fixed when the effect begins, and those that
||| do neither and therefore "modify the rules of the game" — and the
||| rule's own worked example is a prevention effect ("Prevent all damage
||| creatures would deal this turn" applies to creatures that were not on
||| the battlefield when it began). That is why `Replacement` and
||| `Prevention` carry no subject noun where the other five do.
public export
data StaticKind = PtDelta | KeywordGrant | DeedRestriction | TypeAddition
                | ControlGrant | Replacement | Prevention

||| The attestation table's other half: which classes of adverbial each
||| construction writes. Full rows in both directions. The shape of it
||| is the finding — the cross-turn span is the one class both GRANTS and
||| the RESTRICTION admit, and the two current-turn words divide those
||| two families with nothing shared, which is why the detain family's
||| cross-turn span ("Up to one target creature can't attack or block
||| until your next turn") is the one place a restriction and a grant
||| write the same words. It is not the class every static admits, and
||| the class name says so since chapter twenty-one: the type addition is
||| out of it, which is the row that used to be called `EveryStatic` and
||| had stopped being true of it.
||| The type-addition row is chapter nineteen's, and it is what split
||| `BothGrants`. Two hundred and twenty corpus lines write "in addition
||| to its other types"; the great majority state no span at all (the
||| permanent change Memnarch glosses outright, "(This effect lasts
||| indefinitely.)"), eighteen end the clause at "until end of turn"
||| ("{U}: Target creature becomes an artifact in addition to its other
||| types until end of turn", Neurok Transmuter), and NOT ONE ends one at
||| end of combat. Coward // Killer settles the current-turn split from
||| inside a single sentence: "Target creature can't block this turn and
||| becomes a Coward in addition to its other types until end of turn"
||| gives the restriction "this turn" and the type addition "until end of
||| turn" in one breath, which is chapter seventeen's division confirmed
||| by a card that writes both halves at once. The cross-turn cell is
||| CLOSED, and chapter twenty-one closed it on the re-measurement
||| chapter nineteen asked for: two hundred sixty-four supported lines
||| write "in addition to its/their/his/her other types" and exactly
||| three of them carry "until your next turn", every one otherwise
||| explained. Rootwise Survivor's duration belongs to the SEPARATE haste
||| grant beside it ("That land becomes a 0/0 Elemental creature in
||| addition to its other types. It gains haste until your next turn.");
||| Absorbing Man's clause is a copy construction, the type addition
||| riding inside an "except" list; and Tezzeret, Cruel Machinist's
||| "becomes a 5/5 creature in addition to its other types" fuses a base
||| power/toughness setting this vocabulary has no word for onto the
||| addition. No line writes a NAKED type addition across turns, so the
||| honest value is `False` (`badTypeAdditionAcrossTurns`) and Tezzeret
||| waits on the compound base-P/T-setting construction (ledger).
||| The CONTROL-grant row is chapter twenty-two's, and it is the widest
||| of the five: control is the construction that writes every current-
||| turn grant endpoint AND the cross-turn one AND the new for-as-long-as
||| adverbial, and it is the only construction that writes "until the end
||| of your next turn" at all. Counts, joint patterns as everywhere in
||| this table: "until end of turn" a hundred and eleven ("Gain control
||| of target creature until end of turn", Act of Treason), "for as long
||| as" forty-two, "until the end of your next turn" four, "until end of
||| combat" one (Tahngarth, Talruum Hero's "you may have that opponent
||| gain control of Tahngarth until end of combat" — a single line, the
||| same weight chapter seventeen gave Glyph of Destruction). Written
||| ZERO times: "this turn" (the four lines matching are event clauses —
||| "that entered this turn", "that attacked you this turn" — not
||| adverbials), "until your next turn", "until your next upkeep", and
||| every end-step and that-player endpoint. So the restriction's own
||| current-turn word stays the restriction's alone, which is chapter
||| seventeen's division holding under a fifth construction.
||| The two SHIELD rows are chapter twenty-five's, and they part where
||| the earlier five never did — on how MANY adverbials each writes.
||| The replacement interception is nearly as wide as the control grant:
||| "this turn" is its ordinary word (fifty-seven "if … would die this
||| turn" lines alone), and it also writes the two cross-clause endpoints
||| ("Until end of turn, if …", seven; "Until your next turn, if …",
||| two). Prevention writes ONE: two hundred fifty-five prevention lines
||| carry "this turn" and the corpus writes "prevent … until end of
||| turn", "prevent … until end of combat" and "prevent … for as long
||| as" zero times each. Nothing writes an interception or a shield at
||| an upkeep, at a combat endpoint, or under a for-as-long-as
||| condition, all measured zero.
public export
admitsSpan : StaticKind -> SpanUse -> Bool
admitsSpan PtDelta Unattested = False
admitsSpan PtDelta Unclaimed = False
admitsSpan PtDelta GrantsAndControl = True
admitsSpan PtDelta GrantsTypesControlAndReplacement = True
admitsSpan PtDelta KeywordGrantOnly = False
admitsSpan PtDelta RestrictionsAndShields = False
admitsSpan PtDelta GrantsRestrictionsAndReplacement = True
admitsSpan PtDelta ControlGrantOnly = False
admitsSpan PtDelta GrantsRestrictionsAndControl = True
admitsSpan KeywordGrant Unattested = False
admitsSpan KeywordGrant Unclaimed = False
admitsSpan KeywordGrant GrantsAndControl = True
admitsSpan KeywordGrant GrantsTypesControlAndReplacement = True
admitsSpan KeywordGrant KeywordGrantOnly = True
admitsSpan KeywordGrant RestrictionsAndShields = False
admitsSpan KeywordGrant GrantsRestrictionsAndReplacement = True
admitsSpan KeywordGrant ControlGrantOnly = False
admitsSpan KeywordGrant GrantsRestrictionsAndControl = True
admitsSpan DeedRestriction Unattested = False
admitsSpan DeedRestriction Unclaimed = False
admitsSpan DeedRestriction GrantsAndControl = False
admitsSpan DeedRestriction GrantsTypesControlAndReplacement = False
admitsSpan DeedRestriction KeywordGrantOnly = False
admitsSpan DeedRestriction RestrictionsAndShields = True
admitsSpan DeedRestriction GrantsRestrictionsAndReplacement = True
admitsSpan DeedRestriction ControlGrantOnly = False
admitsSpan DeedRestriction GrantsRestrictionsAndControl = True
admitsSpan TypeAddition Unattested = False
admitsSpan TypeAddition Unclaimed = False
admitsSpan TypeAddition GrantsAndControl = False
admitsSpan TypeAddition GrantsTypesControlAndReplacement = True
admitsSpan TypeAddition KeywordGrantOnly = False
admitsSpan TypeAddition RestrictionsAndShields = False
admitsSpan TypeAddition GrantsRestrictionsAndReplacement = False
admitsSpan TypeAddition ControlGrantOnly = False
admitsSpan TypeAddition GrantsRestrictionsAndControl = False
admitsSpan ControlGrant Unattested = False
admitsSpan ControlGrant Unclaimed = False
admitsSpan ControlGrant GrantsAndControl = True
admitsSpan ControlGrant GrantsTypesControlAndReplacement = True
admitsSpan ControlGrant KeywordGrantOnly = False
admitsSpan ControlGrant RestrictionsAndShields = False
admitsSpan ControlGrant GrantsRestrictionsAndReplacement = False
admitsSpan ControlGrant ControlGrantOnly = True
admitsSpan ControlGrant GrantsRestrictionsAndControl = True
admitsSpan Replacement Unattested = False
admitsSpan Replacement Unclaimed = False
admitsSpan Replacement GrantsAndControl = False
admitsSpan Replacement GrantsTypesControlAndReplacement = True
admitsSpan Replacement KeywordGrantOnly = False
admitsSpan Replacement RestrictionsAndShields = True
admitsSpan Replacement GrantsRestrictionsAndReplacement = True
admitsSpan Replacement ControlGrantOnly = False
admitsSpan Replacement GrantsRestrictionsAndControl = False
admitsSpan Prevention Unattested = False
admitsSpan Prevention Unclaimed = False
admitsSpan Prevention GrantsAndControl = False
admitsSpan Prevention GrantsTypesControlAndReplacement = False
admitsSpan Prevention KeywordGrantOnly = False
admitsSpan Prevention RestrictionsAndShields = True
admitsSpan Prevention GrantsRestrictionsAndReplacement = False
admitsSpan Prevention ControlGrantOnly = False
admitsSpan Prevention GrantsRestrictionsAndControl = False

||| Whether a construction can write NO duration at all. A grant can:
||| the unwritten span is [CR#611.2a]'s end-of-game default, which the
||| guide permits where the effect is "intentionally indefinite under
||| the rules" (Through the Breach's bare "It gains haste."). A
||| restriction cannot — a durationless "can't" is the STATIC ability
||| line ("Enchanted creature can't attack", Pacifism), a different
||| construction and the parked ability layer's, so the whole clause is
||| unwritable here rather than the span being optional
||| (`badStaticCant`).
||| The type addition's answer is `True`, and it is the row where the
||| unwritten span is the NORM rather than the exception: most of the two
||| hundred and twenty "in addition to its other types" lines state no
||| duration, and Memnarch prints the reason in reminder text — "(This
||| effect lasts indefinitely.)" — which is [CR#611.2a]'s end-of-game
||| default said out loud on the card.
||| The control grant's answer is `True` on the same evidence and the
||| same reminder text: "Gain control of target creature." states no
||| duration at all (Beguiler of Wills, Dominate, Blue Sun's Twilight),
||| and Phyrexian Infiltrator prints the gloss — "(This effect lasts
||| indefinitely.)" — beside its own exchange.
||| Both SHIELD rows answer `False`, and for the deed restriction's
||| reason exactly: a durationless interception or shield is the STATIC
||| ABILITY line and not a clause at all. [CR#603.6d] says so for the
||| entry riders in as many words and [CR#611.3] for the rest — a
||| continuous effect from a static ability carries no duration because
||| it lasts while the ability functions. The corpus divides on the same
||| line: every one-shot interception and every one-shot shield states a
||| span ("If that creature would die this turn, exile it instead"),
||| while the forty-nine durationless "Prevent all …" lines are static
||| abilities to a line ("Prevent all combat damage that would be dealt
||| to enchanted creature") and the standing interceptions are the
||| permanent's own ability ("If a creature an opponent controls would
||| die, exile it instead"). So the whole clause is unwritable here
||| rather than the span being optional (`badStandingIntercept`,
||| `badStandingPrevention`), which is `badStaticCant` a third and
||| fourth time.
public export
absentOk : StaticKind -> Bool
absentOk PtDelta = True
absentOk KeywordGrant = True
absentOk DeedRestriction = False
absentOk TypeAddition = True
absentOk ControlGrant = True
absentOk Replacement = False
absentOk Prevention = False

||| How an indefinite phrase MARKS its choice method — the axis three
||| constructors used to spell three times. Choice method is surface
||| data (finding 11): no CR rule derives a chooser, so what the
||| grammar records is what the text writes, and the phrase itself is
||| one determiner throughout ("a …", article and all).
|||
||| Core keeps the same axis off the choice itself: `Binder::ChooseOne`
||| carries the filter and a separate `by` slot naming who chooses,
||| defaulting to the controller and OVERRIDDEN for the foreign chooser
||| ("that player sacrifices a creature of their choice",
||| [CR#608.2d,701.21a]) — `binder.rs` says so in those words — while
||| the chooserless form is a different constructor entirely
||| (`Selection::Random`, `selection.rs`). `Unmarked` is core's elided
||| `by`, `TheirChoice` its override, `AtRandom` its random sibling.
|||
||| `TheirChoice` carries the pronoun's own obligation: "their" is a
||| POSSESSIVE, so it needs exactly one player antecedent — a singular
||| subject or one distributive group (`countChoosers`,
||| `badUnboundTheirChoice`). A NOMINAL chooser slot mirroring core's
||| `by: Reference` ("of that player's choice") waits on the plural
||| player read the distributive antecedent would need; the possessive
||| pronoun is the whole attested marking here (ledger).
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

||| WHICH zones English writes a possessor for — the closed table
||| behind the owned zone phrase, and [CR#400.1] is the whole of it:
||| "Each player has their own library, hand, and graveyard. The other
||| zones are shared by all players." So "your hand" and "an
||| opponent's graveyard" are phrases and "your battlefield" is not,
||| and the reason is a fact about the ZONE rather than about the
||| phrase that names it. The library takes the row the rule names it in
||| ("your library", eight hundred twenty-five search lines alone); the
||| shared zones can never take one, and a new shared zone declares its
||| absence by having no row to write.
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
    -- the possessor is SINGULAR — the plural surface is the plural
    -- relational ("their owners' hands", ledger) — and its zone must
    -- be one a player has ([CR#400.1], `Possessable`).
    -- spelling: (construction-owned -- the possessive premodifier
    -- "<Param(0)>'s" before the zone word, e.g. "your hand", "its
    -- owner's hand")
    OwnedBy : (n : Noun bs Player) -> {auto 0 ps : Possessable z} ->
              {auto 0 one : nounPlur n = OneOf} -> ZoneScope bs z

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
    -- the library AT a position ([CR#401.2] — the one ordered zone), with
    -- the order rider a plural arrival takes ([CR#401.4]). A separate row
    -- rather than a `Maybe LibPos` on `ZoneAt`, and for core's own reason:
    -- a bare library is not a destination at all (`DestOk` has no row for
    -- `ZoneAt Library _`, which is core's `exclude(Library)` on
    -- `Destination` exactly), so the positioned form is the SINGLE
    -- canonical spelling and the two could never denote the same thing.
    -- `ZoneAt Library` remains the zone as a WHOLE — what a search looks
    -- through and a shuffle randomizes — and that is a different phrase,
    -- not a second spelling of a destination.
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
  ||| answer it — `zoneSort` reads `Library` off "the top of your
  ||| library" as readily as off "your library" — and one clause needs
  ||| the difference. [CR#701.23a] defines searching as looking at "all
  ||| cards in that zone", so a search takes the pile and never a place
  ||| in it; [CR#401.2] is why the two can be told apart at all, a
  ||| library being "a single face-down pile" whose positions are not
  ||| zones of their own. Full rows, so a new zone phrase declares
  ||| whether it names a whole zone.
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
    -- modifier per hasHead -- not a complete Nominal alone)
    OfChosen : (q : QualitySort) -> {auto 0 ok : countQuality q bs = 1} -> Predicate bs Object
    -- zero relative "[player] controls": the possessor is singular
    -- ([CR#109.4] — one controller; the union read "creatures your
    -- opponents control" is the player-groups vocabulary, ledger).
    -- spelling: ["<Param(0)> control"] (auto-inflection covers "controls"),
    -- kind: TODO(reason: non-head relative-clause modifier per hasHead)
    ControlledBy : (n : Noun bs Player) -> {auto 0 one : nounPlur n = OneOf} -> Predicate bs Object
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
    -- "with [characteristic] [n] or less/greater" — a numeric BOUND on
    -- one of the object's own numbers ([CR#208.1] power and toughness,
    -- [CR#202.3] mana value), and core's
    -- `CharacteristicPredicate::Stat(Stat, Cmp, Count)` in the same
    -- three parts (`filter.rs`, whose own example is
    -- `Stat(Power, AtLeast, 3)`). The bound is WRITTEN — a numeral or
    -- the announced X (`WrittenBound`) — because that is the frame
    -- this wording belongs to: a phrasal standard takes "less than
    -- (or equal to)" instead and waits on the ledger.
    -- spelling: (construction-owned -- the postnominal qualifier "with
    -- <Param(0)> <Param(2)> <Param(1)>", the comparator supplying its own
    -- trailing word; mirrors the english crate's
    -- `QuantityRepr::OrComparison(value, word)` against its separate
    -- `ComparisonComplement { Than | ThanOrEqualTo, standard }` for the
    -- phrasal frame, syntax/phrase.rs), kind: TODO(reason: non-head
    -- postnominal qualifier per hasHead)
    Compare : (c : Characteristic) -> (r : Comparator) -> (bound : Amount bs) ->
              {auto 0 wb : WrittenBound bound} -> Predicate bs Object
    -- spelling: ["in <Param(0)>"] (also "from <Param(0)>", see comment),
    -- kind: Nominal (hasHead = True; implicit head is the zone's carrier,
    -- e.g. "a card in your hand")
    InZone : ZoneExpr bs -> Predicate bs Object          -- zone clause "in/from [zone]" ([CR#109.2a])
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
    -- announces ONE target ([CR#601.2c]), and the determiner scopes
    -- over the whole coordination — the parse brackets it
    -- `<<target> <<artifact> <or <enchantment>>>>`. The kind index
    -- forces the alternatives to describe the same sort of thing
    -- without a gate. The obligations are listed in DECLARATION
    -- order: a coordination needs two alternatives (`TwoDisjuncts`),
    -- they are PARALLEL — the same grammatical rank, and committing
    -- their referent alike, zone and presupposed type both
    -- (`ParallelDisjuncts`) — a word that fills one
    -- phrase-level slot is not an alternative (`CoordinableDisjuncts`),
    -- and no alternative repeats another (`DistinctDisjuncts`).
    -- spelling: (construction-owned -- serial-comma coordination with a
    -- final "or", mirroring core's `Predicate::Or`; the guide puts the
    -- Oxford comma before the coordinator from three items up)
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
    -- the ANCHORED COMPLEMENT — "other" over a named referent: the
    -- described domain MINUS the anchor. "Each other creature you
    -- control", "other creatures you control get +1/+1", "each other
    -- player": one modifier word, and a different relation from `Other`
    -- above, which is why it is a second row and not a widening (the
    -- `Compare`/`CompareAmt` and `Not`/`NotCond` shape — one namespace,
    -- two frames, and the argument in the name). `Other` is the
    -- TARGETING distinctness a slot announces against the slots before
    -- it ([CR#115.4]'s "another target"), so its anchor is every earlier
    -- target and the discourse supplies it; this one subtracts ONE
    -- referent the clause names, and carries it.
    -- The anchor is a READ or a written TARGET, and singular
    -- (`ComplementAnchor`, chapter twenty-six). It used to be a read
    -- ONLY — the blanket `Bindingless` demand `Matches` makes of its
    -- subject — and that conflated two questions: an anchor written as
    -- "a creature" would indeed announce a referent the sentence never
    -- spelled (`badComplementAnchorAnnounces`), but "other than target
    -- player" spells its referent out loud and the corpus writes it
    -- twice (Death by Dragons, Terrifying Presence). The announcement
    -- travels with the phrase now (`predDelta`). Its KIND is the
    -- phrase's own, by the index — "each other player" excludes a
    -- player and cannot exclude a creature — and its NUMBER is
    -- singular, one referent being what this constructor subtracts.
    -- What it does NOT need is a count, and that is the finding: the
    -- definite sweep filed this family with "the rest" under one
    -- heading and they come apart. Subtracting a REFERENT from a
    -- DESCRIPTION needs no cardinality at all, so it lands now;
    -- subtracting a SUBSET from a GROUP ("the rest", "the other",
    -- "both") still needs a binding to record how many, and stays
    -- ledgered.
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

  ||| The head types a phrase OFFERS, as a SET — `seedTy`'s answer to
  ||| the question "other" asks. The two differ on a coordination only,
  ||| and they must: `seedTy` projects the ONE type the phrase fixes
  ||| onto its referent, and "artifact or enchantment" fixes none, but
  ||| it does not thereby offer nothing to anchor against — it offers
  ||| one head per alternative. The empty list is the head that really
  ||| is untyped (the class word, a zone clause's implicit card), and a
  ||| conjunction takes the first member that offers anything, exactly
  ||| as `seedTyAll` does.
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
  -- a controller relation says the same thing: only objects on the
  -- stack or on the battlefield have a controller, and everything else
  -- "isn't controlled by any player" ([CR#109.4]), so "a creature you
  -- control in your graveyard" describes nothing
  -- (`badControlledInGraveyard`). The stack is the caveat — [CR#109.4]
  -- grants stack objects a controller too, and `Zone` has no stack row
  -- today, so the battlefield seed is exact; revisit when one lands.
  seedZone (ControlledBy _) = Just Battlefield
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

  ||| The card type a modifier PRESUPPOSES of its referent — the type
  ||| twin of `seedZone`, and a different question from `seedTy`, which
  ||| projects the phrase's own HEAD. Only a creature can attack or
  ||| block ([CR#506.3]), so the status word presupposes the type
  ||| exactly as it presupposes the battlefield. It recurses through
  ||| BOTH list forms, exactly as `seedZone` does. A disjunction
  ||| presupposes what every alternative presupposes, which is how
  ||| "attacking or blocking" keeps demanding a creature ([CR#506.3]
  ||| names both words in one breath) though neither word survives
  ||| alone. A conjunction needs its row for the alternatives' sake:
  ||| the coherence scans see a top-level conjunction through
  ||| `flattenPs`, but one BURIED in an alternative is left whole, and
  ||| the presupposition written there is written all the same
  ||| ("attacking artifact or blocking land" demands a creature twice
  ||| over — `badWrappedStatusLaunder`, which read as unpresupposing
  ||| while this row was missing).
  public export
  seedType : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedType Attacking = Just Creature
  seedType Blocking = Just Creature
  -- a bound on a number presupposes an object that HAS that number,
  -- which is the status word's shape with a table in place of a fixed
  -- answer: power and toughness demand a creature ([CR#208.3]), mana
  -- value demands nothing ([CR#202.3]). What it does NOT presuppose is
  -- a zone, and that is the difference from the status words above:
  -- attacking is a battlefield designation, while a creature card in a
  -- graveyard keeps the power printed on it ([CR#208.1] prints it on
  -- the card; [CR#208.3] withholds it off the battlefield only from
  -- noncreature objects) and
  -- every object has a mana value wherever it stands — which is why
  -- "return target creature card with power 2 or less from your
  -- graveyard to the battlefield" is ordinary oracle and no zone seed
  -- may refuse it.
  seedType (Compare c _ _) = comparedType c
  -- a subtype word presupposes the card type whose closed set it comes
  -- from ([CR#205.1a]; [CR#205.3m] makes every row here a creature type),
  -- which is the status word's shape with a table in place of a fixed
  -- answer. It presupposes no ZONE: a Zombie card in a graveyard is
  -- still a Zombie, so the word places nothing and the phrase's own
  -- default speaks.
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
  hasHead (ControlledBy _) = False
  hasHead Attacking = False
  hasHead Blocking = False
  hasHead (Compare _ _ _) = False
  hasHead (InZone _) = True
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

  ||| Every member of a conjunction, nested conjunctions flattened —
  ||| the member scan the coherence gates share, so a clash one level
  ||| down is refused exactly as a sibling clash is. It flattens
  ||| conjunctions ONLY: a disjunction's alternatives are not siblings
  ||| of the conjunction around it (splicing them in would make
  ||| "attacking or blocking" read as "attacking and blocking"), so an
  ||| `Or` passes through whole and the gates see it through its
  ||| projections instead.
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
  ||| negation is real oracle — "Each Vampire creature card you own
  ||| that isn't on the battlefield has madness." (Falkenrath Gorger) —
  ||| so `Not (InZone …)` stays writable; what it cannot do is
  ||| contradict the zone the phrase actually places its referent in.
  ||| A negated modifier that merely PRESUPPOSES a zone rules out
  ||| nothing: presupposition projects through negation, so
  ||| "nonattacking creature" still stands on the battlefield — real
  ||| and plentiful oracle ("Target nonattacking, nonblocking creature
  ||| gets +0/+2 until end of turn."; `rawNonattacking`) that reading
  ||| the seed through `Not` would have refused.
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

  ||| The conjunction's whole zone story: the explicit zones agree with
  ||| each other, AND the phrase's EFFECTIVE zone — a bare description
  ||| means the battlefield ([CR#109.2]) — is not one the phrase rules
  ||| out. "creature that isn't on the battlefield" contradicts its own
  ||| default; "creature card in your graveyard that isn't on the
  ||| battlefield" does not.
  public export
  zonesOk : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  zonesOk ps = zonesAgree Nothing (flattenPs ps) &&
               not (zoneMember (zoneOr Battlefield (seedZoneAll (flattenPs ps)))
                               (negZones (flattenPs ps)))

  public export
  data ZoneCoherent : List (Predicate bs k) -> Type where
    MkZoneCoherent : {auto 0 ok : zonesOk ps = True} -> ZoneCoherent ps

  ||| Syntactic predicate equality — enough to spot a member that
  ||| contradicts a sibling, or an alternative that repeats one. Still
  ||| CONSERVATIVE on the rows carrying a noun, but no longer
  ||| VACUOUSLY so on the rows carrying STRUCTURE: `ControlledBy`
  ||| compares its possessor with `nounEqRef` and a conjunction its
  ||| members pointwise, so neither the syntactically identical
  ||| contradiction nor the syntactically identical alternative
  ||| launders through a blanket `False`. That `False` reads "not
  ||| provably the SAME referent", so the gate under-refuses rather
  ||| than over-refuses. Per-row catch-alls, so a new predicate form is
  ||| a totality error.
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
  predEq (ControlledBy a) (ControlledBy b) = nounEqRef a b
  predEq (ControlledBy _) _ = False
  predEq Attacking Attacking = True
  predEq Attacking _ = False
  predEq Blocking Blocking = True
  predEq Blocking _ = False
  -- all three parts, since all three are written words: the same
  -- characteristic, the same comparator, and the same bound. `boundEq`
  -- is conservative where the rest of this function is.
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
  ||| nothing either. Positive types do NOT clash with each other —
  ||| they stack, an attacking artifact being an artifact creature — so
  ||| only the negation raises. The flattened scan catches the nested
  ||| spelling of both.
  public export
  contradictionFree : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  contradictionFree ps = noNegatedPair (flattenPs ps) &&
                         not (anyTypeClash (negTypes (flattenPs ps))
                                           (seedTypes (flattenPs ps)))

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

  ||| The anchor obligation at the phrase level: `Other`'s own gate
  ||| demands a same-KIND target mention, and the conjunction it sits
  ||| in supplies the head type that mention must be compatible with.
  ||| The modifier also has ONE slot per phrase — the style guide's
  ||| selector order gives other/another a single position and no
  ||| corpus line doubles it — so a second "other" is unwritable
  ||| (`badDoubleOther`). The cap is written FIRST and the CONTEXT-reading
  ||| conjunct last, so the conjunction reduces for a phrase whose context
  ||| is abstract — `anyOtherTarget` carries its anchor presupposition as
  ||| a hypothesis, and `x && True` would stay stuck on the neutral `x`.
  ||| The two branches are exclusive by the cap above them, which is why
  ||| they are an `if` and not a third conjunct: the cap counts BOTH
  ||| spellings of the word, so a phrase that has the bare one has no
  ||| anchored complement to check, and each branch reduces to a single
  ||| stuck term instead of to a `x && True` that never does.
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

  ||| Every anchored complement in a modifier list excludes something
  ||| the phrase could have described. "Each other creature" anchored to
  ||| a LAND subtracts nothing and is written by no corpus line
  ||| (`badComplementCrossHead`) — the same evidence `anyTargetedTy`
  ||| reads for the bare word, that oracle pairs "other" only with
  ||| overlapping heads. An anchor that projects no head type at all
  ||| (bare `This`, "you") fits any head, exactly as an untyped mention
  ||| does there.
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

  ||| A phrase bounds a characteristic AT MOST ONCE. The corpus writes
  ||| no noun phrase with two bounds in it — the pairs that look like
  ||| one are two separate phrases ("Creatures you control with power 2
  ||| or less can't be blocked by creatures with power 3 or greater")
  ||| — so a second bound is the multiplicity error "other" and the
  ||| class word already answer for (`badDoubleComparison`). Writing it
  ||| as a CAP rather than as arithmetic is what makes it honest and
  ||| cheap at once: the empty-range pair ("power 2 or less" beside
  ||| "power 4 or greater") is refused because the phrase says it
  ||| twice, not because a range solver went looking, and the
  ||| satisfiable pair is refused on exactly the same evidence — that
  ||| oracle does not write it. An interval, when the corpus finally
  ||| wants one, is a construction with its own word, not two
  ||| qualifiers stacked.
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

  ||| Alternatives are PARALLEL: each one has to be able to stand where
  ||| the others stand. Three ways a phrase can fail that, and the guide
  ||| names them with one sentence — "Repeat the carrier when the
  ||| alternatives have different domains or modifiers". A head noun
  ||| and a bare modifier are not interchangeable ("artifact or
  ||| attacking" is unwritable, `badHeadlessDisjunct`); neither are a
  ||| hand card and a battlefield permanent, because the phrase places
  ||| its referent ONCE (the corpus writes cross-zone alternatives —
  ||| "an Equipment card from your hand or graveyard", seventeen lines
  ||| — under a shared preposition, which is a zone disjunction the
  ||| single-valued projection cannot carry, so it is refused here and
  ||| ledgered rather than mis-projected, `badCrossZoneDisjunction`);
  ||| and neither is an alternative that COMMITS beside one that stays
  ||| silent. The comparison is on the SEEDS themselves, silence
  ||| included, not on the defaults they fall back to: reading both
  ||| through [CR#109.2]'s battlefield made "attacking artifact or
  ||| land" look parallel, and the disagreement then reappeared as a
  ||| projection of NOTHING, which is what let the surrounding
  ||| conjunction place the phrase in a graveyard (`badPartialZoneJoin`).
  ||| Agreement here is what makes the joins honest downstream: a
  ||| `Nothing` out of `seedZoneJoin` now means every alternative was
  ||| silent, never that they disagreed.
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

  ||| May this predicate stand as ONE alternative? Three words cannot,
  ||| and for one reason between them: each is written once for the
  ||| whole phrase, so putting it on one side of an "or" spells
  ||| something the phrase already said. "Any target" is ITSELF the
  ||| class [CR#115.4] defines by disjunction — "creatures, players,
  ||| planeswalkers, or battles" — so coordinating it with an
  ||| alternative re-opens a closed union (`badAnyTargetInOr`); "other"
  ||| fills its one selector slot for the coordination entire
  ||| ("Another target Wolf or Werewolf you control", `badOtherInOr`);
  ||| and a disjunction inside a disjunction is the flat coordination
  ||| written with brackets oracle has no way to print
  ||| (`badNestedOr`) — core reaches the same shape by flattening
  ||| associatively in `normalize`, where the workbench refuses the
  ||| second spelling outright. The scan reads each alternative through
  ||| `flattenPs`, so a singleton conjunction wrapped around any of the
  ||| three launders none of them.
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
  ||| evidence and more sharply: no corpus line spells "non-(A or B)"
  ||| at all, while the De Morgan the writer does instead is plentiful
  ||| and comma-chained ("noncreature, nonland card"; "Target
  ||| nonattacking, nonblocking creature"), so `Not (Or …)` is
  ||| unwritable (`badNegatedDisjunction`) and the two-member `Or`
  ||| launders nothing either. "Any target" and "other" are not
  ||| negatable — the class word is never negated ([CR#115.4] defines it
  ||| positively) and "non-other" is unwritten — and neither is the
  ||| universal player word, which names one of the people in the game
  ||| ([CR#102.1]) and so has no complement class inside the kind, the
  ||| same argument the class word's row makes. Nor is a negation itself
  ||| negated: oracle spells no double negative.
  ||| Full rows: a new predicate form declares its answer.
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
  negatable (ControlledBy _) = True
  negatable Attacking = True
  negatable Blocking = True
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
  predSays (ControlledBy _) = True
  predSays Attacking = True
  predSays Blocking = True
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
  predNegFree (ControlledBy _) = True
  predNegFree Attacking = True
  predNegFree Blocking = True
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
  ||| ITSELF the targeting form — "a any target" and "each any target"
  ||| are unwritable — so only the targeting determiners admit it. The
  ||| scan reaches through EMBEDDED nouns, not just sibling predicates:
  ||| a relative clause's possessor ("a creature the controller of any
  ||| target controls") and an owned zone's possessor spell the class
  ||| word just as loudly under a non-targeting determiner
  ||| (`badAnyTargetEmbedded`). Full rows, like every other predicate
  ||| scan; the Pred → Noun → Pred descent is structural, so it
  ||| terminates.
  public export
  anyTargetFree : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Bool
  anyTargetFree (HasType _) = True
  anyTargetFree (HasSubtype _) = True
  anyTargetFree AnyPlayer = True
  anyTargetFree Opponent = True
  anyTargetFree (QualityNoun _) = True
  anyTargetFree (OfChosen _) = True
  anyTargetFree (ControlledBy n) = nounAnyTargetFree n
  anyTargetFree Attacking = True
  anyTargetFree Blocking = True
  -- the bound is a numeral or the announced X (`WrittenBound`), and
  -- neither carries a noun for the class word to hide in. That is the
  -- row's whole warrant, so `writtenBound` is where a widened bound
  -- vocabulary is forced to come back and re-answer this.
  anyTargetFree (Compare _ _ _) = True
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

  ||| May a counted target mention spell the class word? The answer is
  ||| the QUANTITY's, not the determiner's, and the line it draws is a
  ||| MINIMUM rather than a maximum. [CR#115.4] names "any target,"
  ||| "another target," "two targets," "or similar" as one family, and
  ||| the corpus writes every plural member of it with a range that
  ||| starts at one and leaves the count to the caster: "up to two
  ||| targets" and "up to three targets" (Fall of the Titans, Jaya's
  ||| Immolating Inferno), "one or two targets" (Forked Bolt, twelve
  ||| lines), "one, two, or three targets" (Arc Lightning, nine), "any
  ||| number of targets" (Boulderfall, eighteen under "among"). What it
  ||| never writes is a FIXED plural count — "two targets" as a phrase
  ||| of its own is zero lines, and so is "among two targets" — because
  ||| the class word's plural forms all belong to a structure that lets
  ||| the caster pick how many things to hit ([CR#601.2c] has a
  ||| variable target count announced before the targets;
  ||| [CR#601.2d] has the division announced with it). So the gate asks
  ||| for a minimum of one or none at all, and the one refusal left is
  ||| the exact group from two up (`badGroupAnyTarget`).
  |||
  ||| The unbounded quantity used to be refused beside it, on the
  ||| ground that its structure was unbuilt; the division built it, and
  ||| Boulderfall's "deals 5 damage divided as you choose among any
  ||| number of targets" is the positive that retired the ban. What
  ||| stops a plural class-word mention from standing as a BARE
  ||| recipient is no longer this gate but `PerMember`, which asks the
  ||| question where it belongs — of the clause that writes the amount.
  ||| What a permitting quantity licenses is the class word as the
  ||| phrase's HEAD, never the class word wherever it turns up: a
  ||| possessor buried in a relative clause spells "any target" under a
  ||| counted mention exactly as loudly as under "a"
  ||| (`badEmbeddedAnyTargetExact1`), so the embedded scan runs beneath
  ||| every quantity. The head case needs no scan of its own —
  ||| `AnyTargetLone` allows the class word no companion but "other",
  ||| and neither word carries a noun to bury one in.
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
                       Nothing)
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
    -- the TYPE ASCRIPTION: a noun read under a card-type word, which
    -- is what "this artifact"/"this land"/"this creature" is —
    -- `This`'s identity plus a type noun, two axes and not one
    -- constructor. Core keeps identity type-free (`Reference::This`
    -- carries nothing, `reference.rs`), so the type word is this
    -- layer's business, and the projections it earns are the type
    -- word's: it names the head type, and a description including a
    -- card type and no zone/card/spell/source word denotes a
    -- PERMANENT ([CR#109.2]) — so the ascribed phrase stands on the
    -- battlefield where the bare source ("this spell", cycling's
    -- "Discard this card" [CR#702.29a]) stands nowhere the grammar
    -- tracks. Unmoved it introduces what its argument introduces
    -- (nothing, for the source); MOVED it mints a fresh binding — the
    -- move makes it a new object [CR#400.7], which is why
    -- cost-position "Sacrifice this artifact" leaves a referent the
    -- effect's "It" can read ([CR#400.7j] is the exception letting the
    -- effect find it). WHICH nouns take an ascription is a closed
    -- table (`Ascribable`), not this row's business: the corpus writes
    -- the sorted reading of the source and of nothing else.
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
    -- the NON-targeting determiners each demand an `AnyTargetFree`
    -- phrase: "any target" is itself the targeting form, so "a any
    -- target" / "each any target" are unwritable ([CR#115.4]).
    -- spelling: ["each <Param(0)>"], kind: Nominal
    Each : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
           {auto 0 hd : Headed p} ->
           {auto 0 af : AnyTargetFree p} -> Noun bs k    -- "each …": a group, resolution-time [CR#608.2]
    -- "a …": the indefinite — one determiner ([CR#608.2d,400.7]),
    -- whose CHOICE METHOD is a separate axis the text marks or leaves
    -- unmarked (`ChoiceMode`; chooser and method are surface facts,
    -- the guide's chooser marking, and a random discard has no chooser
    -- [CR#701.9b]). Three constructors spelled that one determiner
    -- three times; core spells it once and hangs the chooser off a
    -- slot (`Binder::ChooseOne`'s `by`, `binder.rs`). The mode's own
    -- obligations ride the mode, not this row — "of their choice"
    -- needs its unique player antecedent (`badUnboundTheirChoice`),
    -- "at random" needs nothing.
    -- spelling: (construction-owned -- the indefinite article over its
    -- phrase, followed by whatever adverbial its mode writes; the macros
    -- own the surfaces: a/aTheirChoice/aAtRandom), kind: Nominal
    Indefinite : (m : ChoiceMode bs) -> (p : Predicate bs k) ->
                 {auto ph : Phrasal k} ->
                 {auto 0 hd : Headed p} ->
                 {auto 0 af : AnyTargetFree p} -> Noun bs k
    -- "[quantity] target [pred]": the counted target mention — one
    -- binding, announced [CR#601.2c], and only objects and players are
    -- targetable ([CR#115.1] — `badTargetColor`). ONE constructor
    -- serves every quantity, mirroring core's single announce form
    -- `TargetSpec::Target(Quantity, Predicate)`, so "target creature",
    -- "two target creatures" and "up to two target creatures" differ
    -- only in the range they carry. That quantity is ARITY data: the
    -- phrase's grammatical number reads it (`quantPlur`), it permits
    -- at least one (`badZeroGroup`) and runs upward from one
    -- (`badDescendingRange`, `badZeroLowerRange`), and at exactly one the phrase IS
    -- the singular "target [noun]" — the `target` macro, whose numeral
    -- rendering leaves unwritten. Distinctness stays announce business
    -- ([CR#601.2c]); the quantity never encodes it.
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
    -- "each of [group]": the distributive determiner over a GROUP
    -- MENTION rather than over a description — "Put a +1/+1 counter on
    -- each of up to two target creatures" (Ajani, Adversary of
    -- Tyrants), "Put a +1/+1 counter on each of them" (Nature's
    -- Panoply). `Each` distributes over whatever answers a phrase; this
    -- distributes over the members of one mention already made, so it
    -- announces nothing of its own and contributes its complement's
    -- binding unchanged. The complement is exactly the group forms the
    -- corpus writes after the word (`GroupMention`): a counted target
    -- mention ("each of up to two target creatures", "each of two
    -- target creatures", "each of any number of target creatures") or a
    -- plural read ("each of them", "each of those creatures"). It is
    -- NOT a second distributive ("each of each creature") and not the
    -- universal ("each of all creatures"), neither of which is written.
    -- The phrase stays PLURAL — its mention is the group's, and the
    -- next sentence reads it as one ("… deals 1 damage to each of up to
    -- two target creatures. Those creatures can't block this turn." —
    -- Sparkmage's Gambit). What the determiner buys is the per-member
    -- reading of the clause's own amount: the sentence writes "a
    -- counter" and every member gets one, which is what `PerMember`
    -- demands of a recipient and what a bare plural mention cannot say.
    -- spelling: ["each of <Param(0)>"], kind: Nominal
    EachOf : (grp : Noun bs k) ->
             {auto 0 pl : nounPlur grp = ManyOf} ->
             {auto 0 gm : GroupMention grp} -> Noun bs k
    -- "the top [n] card(s) of [whose] library" — the POSITIONED SLICE, a
    -- definite description whose members are fixed by where they lie
    -- rather than by what they are ([CR#401.2]: a library is a single
    -- face-down pile, so a position picks out cards and says nothing
    -- about them). Twelve hundred and more corpus lines write the top
    -- slice ("the top card of your library", five hundred thirty-nine;
    -- "the top <n> cards of your library", six hundred ninety-nine;
    -- seventy-one more over another player's), against ONE for the
    -- bottom (Grenzo, Dungeon Warden's "the bottom card of your
    -- library") — an asymmetry worth stating, because the bottom is the
    -- dominant DESTINATION (four hundred eighteen lines) and almost
    -- never a source.
    -- It projects NO card type, and that is the honesty this row exists
    -- to keep: the cards are in a hidden zone ([CR#400.2]) whose order
    -- and contents no player may inspect ([CR#401.2]), so the phrase
    -- describes a place and the grammar may not pretend to know what is
    -- there. Every type-demanding verb refuses it for free as a result,
    -- and the zone demand refuses it twice over (`badTapLibraryTop`).
    -- The count is the ordinary magnitude vocabulary: "the top card" is
    -- `Lit 1`, "the top four cards" a `Lit`, "the top X cards of your
    -- library" (eighty-three lines) is `XVal`.
    -- spelling: ["the <Param(0)> <Param(1)> card(s) of <Param(2)>'s library"]
    -- (Param(0) = LibPos's own word "top"/"bottom"; the noun "card"
    -- pluralises with the count, and at one the numeral is unwritten --
    -- "the top card", never "the top one card"), kind: Nominal
    LibrarySlice : (pos : LibPos) -> (amt : Amount bs) ->
                   (whose : Noun bs Player) ->
                   {auto 0 one : nounPlur whose = OneOf} ->
                   {auto 0 wc : WrittenCount amt} -> Noun bs Object
    -- "[q] of [group]" — the PARTITIVE determiner: a selection of some
    -- members out of a group mention already made ("Put one of them into
    -- your hand", sixty-one lines; "one of them" a hundred thirty-five,
    -- "two of them" twenty-five, "up to one of them" three). It is
    -- `EachOf`'s sibling and its opposite in the one way that matters:
    -- the distributive reaches every member and announces nothing, this
    -- one PICKS members and so announces the mention it picked — which
    -- is what makes the complement beside it ("the rest") a subtraction
    -- with something to subtract.
    -- It inherits the group's head type and zone, because a part of a
    -- group is in the group's place and answers the group's description;
    -- what it does not inherit is the group's number, which is its own
    -- quantity's (`quantPlur`).
    -- spelling: ["<Param(0)> of <Param(1)>"] (Param(0) = the Quantity's own
    -- numeral or bound, e.g. "one", "two", "up to one"), kind: Nominal
    SomeOf : (q : Quantity) -> (grp : Noun bs Object) ->
             {auto 0 gm : GroupMention grp} ->
             {auto 0 nz : NonZeroQ q} ->
             {auto 0 wf : WellFormedQ q} -> Noun bs Object
    -- "the rest": the group COMPLEMENT — the members of an assembled
    -- group that the preceding instruction did not take. Six hundred
    -- twenty-nine corpus lines write it as a complement (a further
    -- nineteen write the unrelated temporal idiom "for the rest of the
    -- game"), and five hundred ninety-nine of those subtract from a
    -- library slice a look or a reveal or an exile-from-the-top
    -- assembled.
    -- This is finding 111's subtract-a-subset half, and what unblocks it
    -- is not new counting machinery but the GROUP: chapter twenty-two
    -- could not subtract one announced target from another because two
    -- "target" instances are two mentions and never a pair
    -- ([CR#601.2c], finding 127), while a slice phrase names one group
    -- whose members a partitive then divides. So the gate asks for
    -- exactly that pair — one group mention, and at least one part taken
    -- from it (`theRestOk`; `badRestWithoutGroup`, `badRestWithoutPart`).
    -- The pair complement over never-assembled mentions stays refused,
    -- and the fight expansion with it.
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
    -- "the [verbed] [noun]" ("the exiled card", "the sacrificed
    -- artifact"): the definite participle read — exactly one mention
    -- stamped by that verb tag and reached by the noun word may
    -- precede. The disambiguator real text switches to where a bare
    -- demonstrative would be ambiguous (finding 26). Two axes kept
    -- apart as queries: the PROVENANCE picks the mention
    -- (`stampedBy`) and the WORD describes it (`verbedWordOk`), the
    -- pair being what the read's uniqueness counts. The word fixes
    -- the phrase's kind exactly as it does for `That`/`Those`; the
    -- player word yields a player phrase no participle can reach
    -- (finding 27).
    -- spelling: ["the <Param(0)> <Param(1)>"] (Param(0) = VerbName's lemma,
    -- rendered as its past participle by auto-inflection; Param(1) =
    -- NounWord's own word), kind: Nominal
    TheVerbed : (v : VerbName) -> (w : NounWord) ->
                {auto 0 ok : countVerbed v w bs = 1} -> Noun bs (kindOfW w)
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
  ||| referent the binding context already fixes ("you" is you, "it" is
  ||| the unique singular object), so two occurrences inside ONE phrase
  ||| denote the same thing. Everything else is False, INCLUDING two
  ||| target mentions: [CR#601.2c] chooses each "target" instance
  ||| separately, so two of them may denote different objects and must
  ||| never be equated. Per-row catch-alls, so a new noun form is a
  ||| totality error. (Declared after `Noun` because a function's TYPE
  ||| is elaborated in source order even inside `mutual`.)
  public export
  nounEqRef : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Noun bs k -> Bool
  nounEqRef This This = True
  nounEqRef This _ = False
  nounEqRef (AsType _ _) _ = False
  nounEqRef You You = True
  nounEqRef You _ = False
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
  nounEqRef (TheVerbed _ _) _ = False
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
  nounAnyTargetFree (TheVerbed _ _) = True
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

  ||| Destination legality for the move primitive ([CR#400.3] — cards
  ||| enter only their owner's hand/library/graveyard, so an owned
  ||| destination naming an arbitrary player is unwritable, and the
  ||| bare zone IS the owner-rooted destination; the possessive is
  ||| rendering's business, finding 34).
  |||
  ||| The LIBRARY row is the position phrase and never the bare zone: a
  ||| library is ordered ([CR#401.2]), so "put it into your library"
  ||| names no place to put it and oracle never writes it — every one of
  ||| the corpus's library placements spells a position ("on top of your
  ||| library", a hundred twenty-seven; "on the bottom of your library",
  ||| four hundred eighteen; the ordinal "third from the top", the
  ||| ledger's). `ZoneAt Library _` therefore has no row at all
  ||| (`badMoveToBareLibrary`), which is core's `exclude(Library)` on
  ||| `Destination` and, in core's own words, "the Idris `DestinationOk`
  ||| gate". The position takes the bare scope like every other
  ||| destination, [CR#400.3] routing the card to its owner's library
  ||| whatever the sentence's possessive says (finding 34).
  public export
  data DestOk : ZoneExpr bs -> Type where
    BattlefieldOk : DestOk (ZoneAt Battlefield Bare)
    ExileOk : DestOk (ZoneAt Exile Bare)
    HandOkBare : DestOk (ZoneAt Hand Bare)
    GraveyardOkBare : DestOk (ZoneAt Graveyard Bare)
    LibraryPosOk : DestOk (LibraryAt pos arrg Bare)

  ||| May THIS patient take THIS destination's order rider? [CR#401.4]
  ||| asks the question and answers it: an arrangement exists only when
  ||| an effect "puts TWO OR MORE cards in a specific position in a
  ||| library at the same time", so a singular placement has no order to
  ||| state. English agrees exactly — "in any order" and "in a random
  ||| order" appear after a plural patient every time they appear, and
  ||| after a singular one ("put it on the bottom of your library in any
  ||| order") zero times (`badSingularOrderRider`). The absent rider is
  ||| legal at either number: it is [CR#401.4]'s own default for a group
  ||| and vacuous for a single card.
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
    MkBinding TheD Object (amtPlur amt) (ObjectP Nothing (Just Library) Nothing)
      :: nounDelta whose
  -- the partitive announces the part it picked, inheriting the group's
  -- place and description and taking its number from its own quantity.
  nounDelta (SomeOf q grp) =
    MkBinding PartD Object (quantPlur q) (ObjectP (nounTy grp) (nounZone grp) Nothing)
      :: nounDelta grp
  nounDelta TheRest = []
  nounDelta It = []
  nounDelta They = []
  nounDelta Them = []
  nounDelta (That w) = []
  nounDelta (Those w) = []
  nounDelta (TheVerbed v w) = []
  nounDelta (ControllerOf n) = MkBinding TheD Player OneOf PlayerP :: nounDelta n
  nounDelta (OwnerOf n) = MkBinding TheD Player OneOf PlayerP :: nounDelta n

  ||| The mentions a predicate's clauses introduce, textual order.
  public export
  predDelta : {bs : Bindings} -> {k : Kind} -> Predicate bs k -> List Binding
  predDelta (ControlledBy n) = nounDelta n
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
  -- which is why this row was invisible under the old blanket demand;
  -- the two lines that write a target there ("Each player other than
  -- target player …", Death by Dragons) announce it like any other
  -- target phrase ([CR#601.2c]), and the announcement belongs to the
  -- phrase the complement modifies.
  predDelta (OtherThan n) = nounDelta n
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
    -- "[its/…] power" / "toughness" / "mana value"
    -- ([CR#208.1,202.3]): one object's own numbers, so the argument
    -- is singular — a group's aggregate is written explicitly ("the
    -- total power of the sacrificed creatures", Soulblast) and is
    -- future vocabulary (`badGroupPower`). WHICH number is read is the
    -- comparison chapter's `Characteristic`, not a row of its own:
    -- core reads every stat through one `Count::StatOf(Reference,
    -- Stat)` (`count.rs`), and three constructors here spelled one
    -- axis three times. The characteristic leads the arguments as it
    -- does in `Compare`, the workbench's other characteristic reader
    -- (core orders the pair the other way; the axis is the same).
    -- The surface phrases are the macros' (`powerOf`, `toughnessOf`,
    -- `manaValueOf`).
    -- spelling: ["<Param(1)>'s <Param(0)>"] (Param(0) = Characteristic's
    -- own word, its construction-owned catalog entry), kind: TODO(reason:
    -- amount fragment, see Lit)
    StatOf : (c : Characteristic) -> (n : Noun bs Object) ->
             {auto 0 one : nounPlur n = OneOf} -> Amount bs
    -- the CARDINALITY of a set the phrase describes — "the number of
    -- cards in your hand", and the domain half of every for-each
    -- amount; mentions inside the predicate fold as everywhere. Core
    -- keeps this orthogonal to multiplication (`Count::CountOf`
    -- against `Count::Times`, `count.rs`) and so does this: counting
    -- the set and scaling the result are two operations, not one
    -- constructor's two arguments. The domain is a real noun phrase:
    -- every corpus for-each domain is noun-HEADED (`Headed` — "for
    -- each you control" heads nothing).
    -- spelling: ["the number of <Param(0)>"] (the bare nominal read;
    -- the for-each adverbial is the `forEach`/`nForEach` macros' own
    -- surface over the same amount), kind: TODO(reason: amount
    -- fragment, see Lit)
    CountOf : {k : Kind} -> (p : Predicate bs k) ->
              {auto 0 hd : Headed p} ->
              {auto 0 af : AnyTargetFree p} -> Amount bs
    -- "[per] times [amt]" — the scaling half, whose left factor is a
    -- WRITTEN numeral. Core's multiplication is count-by-count
    -- (`Count::Times(Count, Count)`; no CR rule licenses a product,
    -- the rules only fixing that every number is an integer, so the
    -- shape answers to the corpus alone); every corpus
    -- multiplication writes a numeral against a phrase instead —
    -- "twice the number of cards in your hand", "three times the
    -- number of creatures tapped this way" (Burn at the Stake), and
    -- the per-unit of a for-each line ("loses 1 life for each
    -- attacking creature you control") — with no phrase-by-phrase
    -- product written anywhere, so the numeral is the factor's type
    -- and generalizing it waits on a line that needs it. A written
    -- numeral is at least one (`AtLeastOne`, `badForEachZero`), which
    -- is finding 45's discipline unmoved: the comparisons that
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
    -- "X" — announced with the cost ([CR#107.3a,107.3i]): a fixed
    -- value by resolution, not a discourse referent.
    -- spelling: ["X"], kind: TODO(reason: amount fragment, see Lit)
    XVal : Amount bs
    -- "[a] plus [b]" — the second operand reads after the first.
    -- spelling: ["<Param(0)> plus <Param(1)>"], kind: TODO(reason: amount
    -- fragment, see Lit)
    Plus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs

  public export
  amtIntro : {bs : Bindings} -> Amount bs -> Bindings
  amtIntro (Lit n) = bs
  amtIntro (StatOf c nom) = nomIntro nom
  amtIntro (CountOf p) = predDelta p ++ bs
  amtIntro (Times per a) = amtIntro a
  amtIntro ThatMuch = bs
  amtIntro XVal = bs
  amtIntro (Plus a b) = amtIntro b

  ||| The grammatical number a COUNTED creation writes, read off its
  ||| amount — `quantPlur`'s twin one vocabulary over. Exactly one is
  ||| singular ("Create a 1/1 green Wolf creature token … put a +1/+1
  ||| counter on IT"); every other written amount is a group, the plural
  ||| noun the corpus writes with it ("Create two … tokens", "create that
  ||| many … tokens"), whatever it evaluates to at resolution. Full rows,
  ||| so a new amount declares its number.
  public export
  amtPlur : {0 bs : Bindings} -> Amount bs -> Plurality
  amtPlur (Lit (S Z)) = OneOf
  amtPlur (Lit _) = ManyOf
  amtPlur (StatOf _ _) = ManyOf
  amtPlur (CountOf _) = ManyOf
  amtPlur (Times _ _) = ManyOf
  amtPlur ThatMuch = ManyOf
  amtPlur XVal = ManyOf
  amtPlur (Plus _ _) = ManyOf

  ||| May this amount stand as a comparison's BOUND? Only the two that
  ||| are WRITTEN as a value: the numeral, and the X announced with the
  ||| cost ([CR#107.3a]) — three hundred and more corpus lines for the
  ||| numeral against ninety-one for X, and nothing else at all. The
  ||| refusal is not that the other amounts are meaningless but that
  ||| they belong to the OTHER frame: a phrasal standard is written
  ||| "less than or equal to the number of lands you control", never
  ||| "the number of lands you control or less", so admitting one here
  ||| would spell a real comparison with a word order oracle does not
  ||| use (`badPhrasalBound`). Full rows, so a new amount declares
  ||| which frame it belongs to — and `anyTargetFree`'s comparison row
  ||| leans on the answer, the two writable bounds carrying no noun.
  public export
  writtenBound : {0 bs : Bindings} -> Amount bs -> Bool
  writtenBound (Lit _) = True
  writtenBound (StatOf _ _) = False
  writtenBound (CountOf _) = False
  writtenBound (Times _ _) = False
  writtenBound ThatMuch = False
  writtenBound XVal = True
  writtenBound (Plus _ _) = False

  public export
  data WrittenBound : Amount bs -> Type where
    MkWrittenBound : {auto 0 ok : writtenBound b = True} -> WrittenBound b

  ||| A count English WRITES is at least one. Drawing, creating, and
  ||| counter-placing all spell their number, and no corpus line spells
  ||| it zero — not as a numeral ("draw zero cards"), not as a
  ||| determiner ("create no tokens"), in any scope. [CR#121.1] makes a
  ||| draw the movement of a card, [CR#111.1] a token a marker put onto
  ||| the battlefield, and [CR#122.1] a counter a marker placed on
  ||| something; a zero of any of them instructs nothing, and English
  ||| writes nothing instead (`badDrawZero`, `badCreateZero`,
  ||| `badPutZeroCounters`).
  ||| It is the LITERAL SPELLING that is refused and not the value. An
  ||| amount that is READ can evaluate to zero and stay perfectly
  ||| written — X's value is its controller's to choose and announce
  ||| ([CR#107.3a]) and nothing floors it, a for-each domain can
  ||| be empty, "that much" can be nothing — so those rows pass and only
  ||| the written numeral is asked to be positive. That is `AtLeastOne`'s
  ||| discipline (`badForEachZero`) reaching the action counts, and it is
  ||| why `Lit` itself stays ungated: a bound of zero is a real
  ||| comparison ("with mana value 0 or less" measures rather than
  ||| instructs). A sum is written where BOTH its operands are.
  public export
  writtenCount : {0 bs : Bindings} -> Amount bs -> Bool
  writtenCount (Lit Z) = False
  writtenCount (Lit (S _)) = True
  writtenCount (StatOf _ _) = True
  writtenCount (CountOf _) = True
  writtenCount (Times _ a) = writtenCount a
  writtenCount ThatMuch = True
  writtenCount XVal = True
  writtenCount (Plus a b) = writtenCount a && writtenCount b

  public export
  data WrittenCount : Amount bs -> Type where
    MkWrittenCount : {auto 0 ok : writtenCount a = True} -> WrittenCount a

  ||| Two bounds, compared as written. Conservative in `predEq`'s
  ||| direction and for its reason: the catch-all reads "not provably
  ||| the same value", and the constructor's own gate means the only
  ||| bounds that ever reach here are the two rows above.
  public export
  boundEq : {0 bs : Bindings} -> Amount bs -> Amount bs -> Bool
  boundEq (Lit a) (Lit b) = a == b
  boundEq XVal XVal = True
  boundEq _ _ = False

  ||| Life-total change operands ([CR#119.3]), and the SET-TO beside the
  ||| two deltas. The third row is chapter twenty-two's ledgered
  ||| player-attribute set arriving: twenty-nine corpus lines write "life
  ||| total becomes" ("Your life total becomes 10", "Target player's life
  ||| total becomes 1", "Each player's life total becomes 7"), and it is
  ||| the operand the life EXCHANGE would need — [CR#701.12c] has each
  ||| player "gain or lose the amount of life necessary to equal the
  ||| other player's previous life total", which is a set realized as
  ||| whichever direction reaches it (finding 121).
  |||
  ||| That is also why the set contributes no OUTCOME mention where the
  ||| deltas each contribute one: "that much" reads a magnitude a clause
  ||| produced, and a set-to produces a gain for one player and a loss
  ||| for another depending on where their total stood. The sort is not
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

  ||| What a delayed clause waits for — time queries introduce nothing;
  ||| an object-event query names its watched referent ("when target
  ||| creature dies this turn", Graceful Reprieve — the when-clause is
  ||| where that target is announced).
  public export
  data EventQuery : Bindings -> Type where
    -- spelling: ["at the beginning of the next end step"], kind: TODO(reason:
    -- temporal-adverbial fragment -- not one of the five FragmentKinds)
    NextEndStep : EventQuery bs                     -- "at the beginning of the next end step"
    -- "when [n] dies this turn" ([CR#700.4] — dying IS the
    -- battlefield-to-graveyard transition, so the watched referent
    -- stands on the battlefield; `badDiesInGraveyard`).
    -- spelling: ["when <Param(0)> dies this turn"], kind: TODO(reason:
    -- temporal-adverbial fragment, see NextEndStep)
    DiesThisTurn : (n : Noun bs Object) ->
                   {auto 0 ok : OnBattlefield (nounZone n)} ->
                   {auto 0 one : nounPlur n = OneOf} -> EventQuery bs

  ||| The context a delayed body reads: settled particulars, with the
  ||| event's own transition applied — dying retags the watched
  ||| referent to the graveyard exactly as a move would ([CR#700.4]).
  public export
  delayedCtx : {bs : Bindings} -> EventQuery bs -> Bindings
  delayedCtx NextEndStep = settleTargets bs
  delayedCtx (DiesThisTurn n) = settleTargets (moveIntro Nothing n Graveyard)

  ||| May this amount stand as a comparison's SUBJECT — the thing
  ||| measured, on the left of "is"? The exact complement of
  ||| `writtenBound`, and for the same reason: a comparison in English
  ||| puts a READ of the game state against a WRITTEN value ("if its
  ||| mana value is 2 or less", "if there are four or more creature
  ||| cards in your graveyard"), and the two halves draw on disjoint
  ||| parts of the one `Amount` vocabulary. The two readers are the
  ||| object's own number and a set's cardinality; the two writables
  ||| are the numeral and the announced X, and neither is ever
  ||| measured — "if 3 is 4 or greater" is not a sentence, and no
  ||| corpus line compares a bare X either. What is neither is neither:
  ||| the scaled product, the sum, and the event-outcome read appear on
  ||| no side of a written comparison at all (`badCompareLiteral`,
  ||| `badCompareThatMuch`). Full rows, so a new amount declares which
  ||| side of a comparison it can stand on — both tables ask, and a new
  ||| row that answers `True` to both would be the first amount English
  ||| both writes and reads.
  public export
  readAmount : {0 bs : Bindings} -> Amount bs -> Bool
  readAmount (Lit _) = False
  readAmount (StatOf _ _) = True
  readAmount (CountOf _) = True
  readAmount (Times _ _) = False
  readAmount ThatMuch = False
  readAmount XVal = False
  readAmount (Plus _ _) = False

  public export
  data ReadAmount : Amount bs -> Type where
    MkReadAmount : {auto 0 ok : readAmount a = True} -> ReadAmount a

  ||| A noun phrase that contributes NOTHING to the discourse — every
  ||| read, and the bare source; never a determined mention. Re-keyed
  ||| onto `nounDelta` rather than mirroring the constructor list, so
  ||| the question asked is the one that matters ("does this phrase
  ||| bind?") and a new noun answers it by the delta it already has to
  ||| write.
  |||
  ||| What consumes it is the condition subject (`Matches`): a
  ||| condition introduces nothing (`condDelta`), so a phrase that
  ||| WOULD have introduced something can only be written there by
  ||| losing it silently. Refusing the phrase is the honest form of
  ||| that — and it is what makes the target-announcing conditional
  ||| ("If target creature has toughness 5 or greater, it gets +4/-4
  ||| until end of turn", Blood Lust) unwritable rather than
  ||| mis-written (`badMatchesTargetSubject`; ledger).
  public export
  data Bindingless : Noun bs k -> Type where
    MkBindingless : {auto 0 ok : nounDelta n = []} -> Bindingless n

  ||| Which phrase may ANCHOR a complement — chapter twenty-six's
  ||| replacement for the blanket `Bindingless` demand `OtherThan`
  ||| carried, and the correction is that "valid anchor" and
  ||| "introduces no binding" were never the same question. The old
  ||| demand refused the only two corpus lines whose anchor is written
  ||| as a target: "Each player other than target player creates a 5/5
  ||| red Dragon creature token with flying" (Death by Dragons) and
  ||| "Prevent all combat damage that would be dealt by creatures other
  ||| than target creature this turn" (Terrifying Presence). Those
  ||| targets are announced like any other ([CR#601.2c]), and the
  ||| announcement now travels with the phrase (`predDelta`) instead of
  ||| being refused for existing.
  |||
  ||| So the SHAPE question is asked on its own: an anchor is a READ,
  ||| or a phrase the text writes as a target. What stays out is the
  ||| determiner that would announce a referent the sentence never
  ||| spelled — "other than a creature" is unwritten English and
  ||| `Indefinite` still refuses it (`badComplementAnchorAnnounces`) —
  ||| and every determiner that names a set rather than a referent
  ||| (`Each`, `AllOf`, `EachOf`, the slice, the partitive, the
  ||| remainder), which subtract nothing an anchor can subtract.
  ||| Full rows, so a new noun declares whether it can anchor one.
  public export
  anchorPhrase : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  anchorPhrase This = True
  anchorPhrase (AsType t n) = anchorPhrase n
  anchorPhrase You = True
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
  anchorPhrase (TheVerbed _ _) = True
  anchorPhrase (ControllerOf _) = True
  anchorPhrase (OwnerOf _) = True

  ||| The complement anchor as a witness: the right SHAPE, and singular.
  ||| The number half is the second of chapter twenty-six's two
  ||| corrections and it is the one that keeps the family honest — the
  ||| constructor subtracts ONE anchored referent, and a plural read
  ||| passed there ("other than those creatures") is group subtraction,
  ||| the subset-complement family finding 111 deferred and that no
  ||| corpus line writes at all: `other than them/those/these` returns
  ||| zero lines, supported scope and all-cards scope alike
  ||| (`badPluralComplementAnchor`).
  public export
  data ComplementAnchor : Noun bs k -> Type where
    MkComplementAnchor : {auto 0 sh : anchorPhrase n = True} ->
                         {auto 0 one : nounPlur n = OneOf} ->
                         ComplementAnchor n

  ||| Which phrases a choice clause can SELECT — the introduction
  ||| discipline chapter twenty gave the indefinite article, asked of
  ||| "Choose". A choice binds a NEW referent out of a described set, so
  ||| the phrase has to describe one: the corpus writes "choose a/an …"
  ||| (four hundred ninety-one lines), "choose target …" (a hundred
  ||| sixty), "choose two/three …", "choose up to …", "choose any number
  ||| of …", "choose another …" — every one of them a selection from a
  ||| description — and writes "choose you", "choose it", and "choose
  ||| them" zero times each. Choosing an ALREADY DEFINITE participant is
  ||| not a choice at all: there is nothing to select among, and the
  ||| clause would announce a mention it did not bind
  ||| (`badChooseYou`). The distributive and universal determiners are
  ||| out for the neighbouring reason — "choose each creature" selects
  ||| nothing either — leaving the two introducing determiners exactly.
  ||| It reads the constructor rather than `nounDelta`, unlike
  ||| `Bindingless` beside it, because a relative clause's possessor
  ||| makes a read's delta nonempty without making the read a choice.
  ||| Full rows.
  public export
  choosable : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  choosable This = False
  choosable (AsType _ _) = False
  choosable You = False
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
  choosable (TheVerbed _ _) = False
  choosable (ControllerOf _) = False
  choosable (OwnerOf _) = False

  public export
  data Choosable : Noun bs k -> Type where
    MkChoosable : {auto 0 ok : choosable n = True} -> Choosable n

  ||| Which phrases name a GROUP the sentence can then reach INTO — the
  ||| complement "each of" takes and the complement a division is spread
  ||| "among". Measured rather than guessed: of the words the corpus
  ||| writes after "each of", the object-denoting ones are a counted
  ||| target mention ("each of up to two target creatures", twenty-nine
  ||| lines; "each of up to X target creatures", nine; "each of up to
  ||| three targets", four; "each of two target creatures", three; "each
  ||| of any number of target creatures", three) and a plural read
  ||| ("each of them", thirty-seven; "each of those creatures",
  ||| twenty-four; "each of those cards", four; "each of those tokens",
  ||| three). Nothing else: no "each of each …", no "each of all …", no
  ||| "each of a …" (`badEachOfDistributive`, `badEachOfAll`,
  ||| `badEachOfIndefinite`). The rest of the corpus's "each of" lines
  ||| are the temporal and quality phrases this vocabulary does not
  ||| reach ("each of your turns", "each of its colors").
  |||
  ||| The reason is what the determiner does: it distributes over
  ||| MEMBERS, so it needs a phrase whose members the sentence has
  ||| already fixed. A description has none until it resolves, which is
  ||| what the plain distributive `Each` is for; a definite singular has
  ||| one, which is no group at all. Full rows, so a new noun form
  ||| declares whether it names a group.
  public export
  groupMention : {0 bs : Bindings} -> {0 k : Kind} -> Noun bs k -> Bool
  groupMention (TargetGroup _ _) = True
  groupMention Them = True
  groupMention (Those _) = True
  groupMention This = False
  groupMention (AsType _ _) = False
  groupMention You = False
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
  groupMention (TheVerbed _ _) = False
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
  ||| outright — `Each` over a description ("put a +1/+1 counter on each
  ||| other creature you control", Bellowing Aegisaur; "deals 1 damage
  ||| to each creature you don't control", Barrage of Boulders) and
  ||| `EachOf` over a group mention ("on each of up to two target
  ||| creatures", Ajani, Adversary of Tyrants). A BARE plural recipient
  ||| answers neither, and the corpus never writes one: "put a … counter
  ||| on" reaches a counted group only through "each of" (zero lines at
  ||| two, three, or four; twenty-eight at "up to ONE target creature",
  ||| which is singular and already passes), and "deals … damage to"
  ||| likewise (`badBarePluralCounterRecipient`,
  ||| `badBarePluralDamageRecipient`). The plural READS are the same
  ||| story from the other side — "counters on them" is thirty-six
  ||| relative clauses ("cards with intel counters on them") and no
  ||| recipient, "damage to them" is the singular epicene player every
  ||| one of its twenty-five times ("At the beginning of each player's
  ||| upkeep, this enchantment deals 1 damage to them"), which is `They`
  ||| and singular already, and "damage to those …" is written zero
  ||| times (`badThemCounterRecipient`). And the universal
  ||| is out with them: "damage to all creatures" and "counter on all
  ||| creatures" are each zero lines, the corpus writing the sweep
  ||| distributively (`badAllOfDamageRecipient`).
  |||
  ||| The DIVISION is the other answer to the same question and is not
  ||| here: it says the magnitude is the group's, to be split
  ||| ([CR#601.2d]), and it writes its own clause (`Distribute`).
  ||| Shaped after `damageSrcOk` — two named rows and a number
  ||| catch-all — because it is that gate's mirror at the other end of
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

  ||| A truth-valued test a clause can be conditioned on — core's
  ||| `Condition` (`deckmaste_core/src/condition.rs`), and named after
  ||| it. The three rows here are core's first three exactly
  ||| (`Exists(Predicate)`, `Matches(Reference, Predicate)`,
  ||| `Compare(Count, Cmp, Count)`), which is not a coincidence: they
  ||| are the three questions English asks of the board without any
  ||| vocabulary beyond the phrase, the reference, and the amount this
  ||| grammar already has. Core's other rows all reach past that —
  ||| attachment, event history, cost tags, turn and phase — and each
  ||| arrives with the axis it names.
  |||
  ||| The word "if" here has "only its normal English meaning"
  ||| ([CR#603.4]'s own parenthetical): this is the ORDINARY
  ||| conditional, not the intervening-"if" of a triggered ability,
  ||| which that rule reserves for an "if" immediately following a
  ||| trigger condition and checks twice — once at the trigger event
  ||| ([CR#603.4]) and again on resolution ([CR#608.2a]). That
  ||| distinction is a CARRIER distinction and not a condition one, and
  ||| the seam is deliberate: the trigger layer reuses this type
  ||| verbatim for its intervening-"if" clause, so nothing here may
  ||| assume a one-shot reader. Nothing does — the rows ask about the
  ||| board, the bindings they are typed in are whatever their carrier
  ||| supplies, and `condDelta`'s opacity holds for any of them.
  public export
  data Condition : Bindings -> Type where
    -- "you control a creature with power 4 or greater" / "there is a
    -- …": at least one object answers the description ([CR#603.4]'s
    -- ordinary-English "if"). Core's `Exists(Predicate)` in one
    -- argument, and the same demands `CountOf` makes of a domain,
    -- because it is the same kind of slot: the phrase must be
    -- noun-HEADED ("if you control attacking" heads nothing —
    -- `badExistsUnheaded`) and must not spell the damage class, which
    -- names a target and describes no object (`badExistsAnyTarget`).
    -- No article of its own: the existential quantifier IS the
    -- indefinite English writes, and the count comparison below spells
    -- the other form ("one or more" — two corpus lines against three
    -- hundred and thirty-eight for the article, which is why both are
    -- rows and neither is sugar for the other; core keeps them apart
    -- likewise).
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
    -- "it's an artifact creature" — a REFERENCE answers a description:
    -- core's `Matches(Reference, Predicate)` with the kind index this
    -- grammar carries and core does not. (Core cites [CR#603.4] on this
    -- row; the rule licenses the ordinary-English "if" and says nothing
    -- about references answering descriptions, so the cite is not
    -- propagated here — the type's own comment carries the "if" point
    -- once.) The subject
    -- is a read and never a mention (`Bindingless`): the condition
    -- introduces nothing, so a determined phrase written here would be
    -- announced and then dropped. The predicate need not be headed —
    -- "if it's attacking" (five lines) and "if it's tapped" (two) are
    -- as real as "if it's a creature card" (two hundred sixty-five) —
    -- but the class word is refused as it is everywhere.
    -- spelling: ["<Param(0)> is <Param(1)>"] (auto-inflection supplies the
    -- copula and its contraction, "it's"/"they're"; the same predicate
    -- spelled postnominally in a noun phrase is spelled predicatively
    -- here), kind: TODO(reason: condition fragment, see Exists)
    Matches : {k : Kind} -> (n : Noun bs k) -> (p : Predicate bs k) ->
              {auto 0 bl : Bindingless n} ->
              {auto 0 sy : PredSays p} ->
              {auto 0 zc : ZoneFits (nounZone n) (seedZone p)} ->
              {auto 0 af : AnyTargetFree p} -> Condition bs
    -- "its mana value is 2 or less" — a measured amount against a
    -- written bound, which is core's `Compare(Count, Cmp, Count)` with
    -- the ONE comparator axis chapter sixteen already opened
    -- (`Comparator`) and the ONE amount vocabulary chapter sixteen
    -- already reads through (`StatOf`, `CountOf`). No parallel
    -- machinery: the postnominal qualifier "with power 4 or greater"
    -- and the predicative "if its power is 4 or greater" are two
    -- FRAMES over the same relation, and what differs between them is
    -- word order, not vocabulary. The two gates are the frame's two
    -- halves — a reader on the left (`ReadAmount`), a written value on
    -- the right (`WrittenBound`, the same witness `Compare` consumes).
    -- The STRICT comparators the condition frame also writes ("if your
    -- life total is less than 7") are a real gap and stay one: they
    -- would make `Comparator` a per-frame table, which is a chapter's
    -- worth of attestation and not a row (ledger).
    -- spelling: ["<Param(0)> is <Param(2)> <Param(1)>"] (the comparator
    -- supplies its own trailing word, and which word depends on what is
    -- measured: a stat takes "or greater"/"or less", a count takes "or
    -- more"/"or fewer" -- one relation, two registers. The count subject
    -- also spells existentially, "there are <Param(2)> or more <domain>",
    -- which is the form the corpus writes 173 times), kind: TODO(reason:
    -- condition fragment, see Exists)
    CompareAmt : (subj : Amount bs) -> (r : Comparator) -> (bound : Amount bs) ->
                 {auto 0 rd : ReadAmount subj} ->
                 {auto 0 wb : WrittenBound bound} -> Condition bs
    -- "if you don't control an Army creature" / "if it isn't a Zombie" —
    -- the CONDITION negation, core's `Not(Condition)`
    -- (`deckmaste_core/src/condition.rs`) and the ledger's own entry,
    -- opened here because [CR#701.47a] writes BOTH of amass's branches
    -- with it and the corpus writes it far past the carriers the ledger
    -- had counted: a hundred and twelve one-shot lines across the two
    -- rows it reaches ("if you don't control a/an …" fifty-nine, "if you
    -- control no …" twenty-four, "if there are no …" seventeen, "if no
    -- [creature/player/opponent] …" twelve on the existential side;
    -- "if it isn't …" twenty-four and "if it's not a …" thirty-seven on
    -- the reference side), and the sentence that fixes the shape is real
    -- card text and not the rule alone — "Put two +1/+1 counters on
    -- target artifact, creature, or land you control. Untap that
    -- permanent. If it isn't a creature, it becomes a 0/0 creature in
    -- addition to its other types."
    -- WHICH rows may be negated is a closed table and not this row's
    -- license (`CondNegatable`): the two frames above are written, the
    -- comparison frame is not — "if its power isn't 4 or greater" is
    -- written zero times, English saying it with the opposite comparator
    -- instead — and a negation of a negation is written zero times too.
    -- The negated condition still contributes nothing (`condDelta`); it
    -- is a hole for the reason every condition is one, and doubly so,
    -- the false branch being the one that runs.
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

  ||| Which condition frames English negates — a closed full-row table,
  ||| so a new condition declares whether its own frame takes a "not".
  ||| The existential and the reference frames do (`Exists`, `Matches`);
  ||| the comparison frame does not, because a bound has a NEGATIVE of
  ||| its own — "if its power isn't 4 or greater" is written zero times
  ||| and "3 or less" is what English writes there — and a negation
  ||| under a negation is written zero times likewise
  ||| (`badNegatedComparison`, `badDoubleNegatedCondition`).
  ||| The two open rows are open CONDITIONALLY, which is chapter
  ||| twenty-one's correction: the frame negates a POSITIVE phrase only.
  ||| A blanket `True` let the polarity launder — "if it isn't a
  ||| non-artifact" is the double negation the row below refuses,
  ||| written one construction down instead (`predNegFree`,
  ||| `badNegatedNegativeMatch`).
  public export
  condNegatable : {0 bs : Bindings} -> Condition bs -> Bool
  condNegatable (Exists p) = predNegFree p
  condNegatable (Matches n p) = predNegFree p
  condNegatable (CompareAmt subj r bound) = False
  condNegatable (NotCond c) = False

  public export
  data CondNegatable : Condition bs -> Type where
    MkCondNegatable : {auto 0 ok : condNegatable c = True} -> CondNegatable c

  ||| What a condition contributes to the discourse: NOTHING, on every
  ||| row. This is the disjunction hole (`predDelta (Or _) = []`) at
  ||| clause level and for the same reason — a condition may be false,
  ||| so a mention written inside one names nobody the sentences after
  ||| it can read back (`badConditionAntecedent`). It is a full-row
  ||| table rather than a constant so that a new condition has to
  ||| declare its answer, and so that `If`'s own contribution stays
  ||| written in terms of it rather than assuming it.
  |||
  ||| Corpus is not unanimous, and the exception is named rather than
  ||| smoothed over: a conditional whose if-clause announces a TARGET
  ||| does leave a referent behind, because [CR#601.2c] announces
  ||| targets as the spell is cast whatever clause spells them, so the
  ||| condition's truth never gated the announcement. Ten corpus lines
  ||| write "if target …", three of them read it back ("If target
  ||| creature has toughness 5 or greater, it gets +4/-4 until end of
  ||| turn", Blood Lust; Hidetsugu's Second Rite; Meddle). That family
  ||| is refused at the subject slot (`Bindingless`) rather than
  ||| admitted with a lie about its bindings, and it waits on the
  ||| ledger with the leading-"if" linearization it belongs to.
  public export
  condDelta : {bs : Bindings} -> Condition bs -> List Binding
  condDelta (Exists p) = []
  condDelta (Matches n p) = []
  condDelta (CompareAmt subj r bound) = []
  condDelta (NotCond c) = []


  ||| Durations, as the trailing adverbial writes them ([CR#611.2a] — a
  ||| resolution-generated continuous effect "lasts as long as stated";
  ||| with no stated duration it lasts until end of game, which is the
  ||| explicit `Nothing` spelling per the no-defaults convention).
  ||| "FOR AS LONG AS" durations ([CR#611.2b]) are the third row, opened
  ||| here on the clause chapter eighteen said they waited for. The rule
  ||| gives the row its shape and its own example is a control grant:
  ||| "Some continuous effects generated by the resolution of a spell or
  ||| ability have durations worded 'for as long as . . . .'", and Master
  ||| Thief's "gain control of target artifact for as long as you control
  ||| this creature" is the text [CR#611.2b] prints beside it. Core spells
  ||| it `Duration::ForAsLongAs(Condition)` (`continuous.rs`) and this row
  ||| is that, which is why the type is now INDEXED by the discourse: a
  ||| condition is typed in bindings, so every duration is. The condition
  ||| reads what the clause it trails has ANNOUNCED — chapter
  ||| twenty-one's `preIntro` distinction, here supplied by the
  ||| `Continuously` envelope's own telescope (`staticIntro se`) — which
  ||| is what Old Man of the Sea needs ("for as long as ... that
  ||| creature's power remains less than or equal to this creature's
  ||| power" reads the grant's own target).
  ||| The blocker chapter eighteen recorded was the missing clause and it
  ||| has fallen: forty-two of the two hundred and ten corpus lines are
  ||| control grants, and `GainsControl` is now a static row. The rest of
  ||| the count is unchanged — six stat deltas, four keyword grants (all
  ||| of "indestructible"), ten restrictions, and thirteen "becomes"
  ||| lines that are type SETTINGS and copies rather than the "in
  ||| addition to its other types" ADDITION this vocabulary spells (the
  ||| addition's own "for as long as" is written zero times with the
  ||| "becomes" verb; the three copular "is an Island in addition to its
  ||| other types for as long as it has a flood counter on it" lines are
  ||| the layer words' construction, not this one).
  ||| The event-ended row is chapter twenty-five's, and minting it is
  ||| what showed that the ledger's ninety-one-line family is TWO
  ||| constructions wearing one phrase. [CR#610.3] makes "exile [object]
  ||| until [event]" a ONE-SHOT zone change that schedules its own undo —
  ||| "a second one-shot effect is created immediately after the
  ||| specified event" — not a continuous effect with a duration at all,
  ||| and [CR#610.4] says the same of the phase-out twin. Eighty-six of
  ||| the ninety-one lines are that zone change and three more are the
  ||| phasing, which leaves three [CR#611.2a] continuous effects, every
  ||| one of them a clause this grammar has no word for. So the row is
  ||| here, its answer is `Unclaimed`, and the writable half of the
  ||| family is a clause rider instead (`HeldUntil`).
  ||| Core makes the opposite cut and its comment says so: `UntilEvent`
  ||| there is one `Duration` row for both, cited to [CR#610.3] with the
  ||| note that "the engine pairs the undo one-shot"
  ||| (`deckmaste_core/src/continuous.rs`). That is right for a runtime,
  ||| which has to schedule the undo either way, and wrong for a grammar,
  ||| where the two phrases sit in different slots of different clauses.
  |||
  ||| "This turn" is its own row rather than an `Until` form because it is
  ||| not one: it names the current turn as a whole, without a boundary
  ||| word, and it is the adverbial the one-shot restrictions write where
  ||| the grants write "until end of turn" — the same span, a different
  ||| construction's word (`spanUse`).
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
  ||| to. The vocabulary two constructions share: the would/instead
  ||| clause names one to intercept ([CR#614.1a]) and the zone-change
  ||| rider names one to wait for ([CR#610.3]), and the MOOD is the
  ||| reading construction's rather than the pattern's ("would die"
  ||| against "leaves the battlefield").
  |||
  ||| The subject is a full noun phrase and not a bare reference,
  ||| because both constructions target through it: "Prevent all damage
  ||| that would be dealt to target creature this turn" announces a
  ||| target inside the event pattern ([CR#601.2c]), and so does the
  ||| interception's own subject where it is written with "target".
  ||| Which row goes with which construction is `eventUse`'s answer and
  ||| not this type's, so a new event declares its readers before it can
  ||| be used by either.
  public export
  -- spelling: (construction-owned -- the reading construction supplies the
  -- mood and the opening word: Intercepts writes "if <subject> would
  -- <event>" or "the next time <subject> would <event>" per ReplUse,
  -- HeldUntil and Duration.UntilEvent write the finite "<subject>
  -- <event>s". Never spelled alone)
  data GameEvent : Bindings -> Type where
    -- "[n] dies" ([CR#700.4] — dying IS the battlefield-to-graveyard
    -- transition, so the watched object stands on the battlefield, which
    -- is `EventQuery`'s own demand one construction over;
    -- `badWouldDieInGraveyard`).
    WouldDie : (n : Noun bs Object) ->
               {auto 0 zn : OnBattlefield (nounZone n)} -> GameEvent bs
    -- "[n] leaves the battlefield" ([CR#603.6c] names the transition —
    -- from the battlefield to another zone) — the O-Ring endpoint.
    WouldLeave : (n : Noun bs Object) ->
                 {auto 0 zn : OnBattlefield (nounZone n)} -> GameEvent bs
    -- "[n] would be destroyed" ([CR#701.8a] — destruction moves a
    -- permanent from the battlefield to its owner's graveyard;
    -- [CR#701.8c] names regeneration as the effect that replaces the
    -- destruction event, which is this row's whole corpus).
    WouldBeDestroyed : (n : Noun bs Object) ->
                       {auto 0 zn : OnBattlefield (nounZone n)} -> GameEvent bs
    -- "damage would be dealt to [n]" ([CR#120.1] — damage is dealt to a
    -- player or a battlefield permanent), the event prevention watches
    -- and redirection replaces. Reads the same recipient row the damage
    -- CLAUSE reads (`DamageRecipient`), which is what keeps one damage
    -- vocabulary rather than two.
    WouldBeDealtDamage : {k : Kind} -> (to : Noun bs k) ->
                         {auto 0 rk : DamageRecipient to} -> GameEvent bs
    -- "[who] would draw a card" ([CR#121.1] — a draw moves the top card
    -- of a library to its owner's hand; [CR#614.11] makes draw
    -- replacement its own paragraph).
    WouldDraw : (who : Noun bs Player) -> GameEvent bs

  ||| Which row an event pattern is, for the event tables.
  public export
  eventName : {0 bs : Bindings} -> GameEvent bs -> EventName
  eventName (WouldDie _) = Dies
  eventName (WouldLeave _) = LeavesBattlefield
  eventName (WouldBeDestroyed _) = IsDestroyed
  eventName (WouldBeDealtDamage _) = IsDealtDamage
  eventName (WouldDraw _) = DrawsCard

  ||| What an event pattern contributes to the discourse: its subject
  ||| phrase, exactly as a clause contributes its own. The event has not
  ||| happened — it is what the construction is watching FOR — but the
  ||| phrase naming its subject is written and announced all the same
  ||| ([CR#601.2c]), which is why the replacement clause can say "it".
  public export
  eventIntro : {bs : Bindings} -> GameEvent bs -> Bindings
  eventIntro (WouldDie n) = nomIntro n
  eventIntro (WouldLeave n) = nomIntro n
  eventIntro (WouldBeDestroyed n) = nomIntro n
  eventIntro (WouldBeDealtDamage to) = nomIntro to
  eventIntro (WouldDraw who) = nomIntro who

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

  ||| Whether an interception may be written with THIS multiplicity
  ||| word, over the event it intercepts ([CR#614.3]'s two endings).
  public export
  data ReplUseOk : GameEvent bs -> ReplUse -> Type where
    MkReplUseOk : {auto 0 ok : replUseOk (eventName ev) u = True} -> ReplUseOk ev u

  ||| The attestation table: every duration this vocabulary can spell,
  ||| against the constructions that write it. FULL ROWS over (boundary x
  ||| part x possession) plus the bare current-turn adverbial — thirty-one
  ||| cells — so a new `TurnPart`, a new `Whose`, or a new boundary is a
  ||| totality error that must be answered with evidence before anything
  ||| can be written with it.
  |||
  ||| The counts behind the open cells (oracle corpus, joint patterns —
  ||| the adverbial has to belong to THAT clause, not merely share a line
  ||| with it): "until end of turn" writes one thousand five hundred
  ||| fifty-nine stat changes and one thousand three hundred ninety-one
  ||| keyword grants and NOT ONE single-deed restriction; "this turn"
  ||| inverts it exactly, two hundred ninety-six restrictions and neither
  ||| grant; "until your next turn" is written by all three (thirteen,
  ||| thirteen, twenty-four), the one span that is nobody's alone; "until
  ||| end of combat" takes the two grants only (Glyph of Destruction's
  ||| "+10/+0", one banding line); "until your next upkeep" takes the
  ||| keyword grant alone (Gabriel Angelfire, and one forestwalk line) and
  ||| no stat change at all.
  |||
  ||| The `Unclaimed` cells are where the frontier is: "until the end of
  ||| your next turn" is eighty-three lines of play permissions and
  ||| control grants, "until end of combat on your next turn" is Brazen
  ||| Cannonade's play permission, "until the end of your next upkeep" is
  ||| Halfdane's base-P/T setting, and the three end-step endpoints are
  ||| play permissions and a copy effect. Every one of them waits on a
  ||| construction, not on a duration.
  public export
  spanUse : {0 bs : Bindings} -> Duration bs -> SpanUse
  spanUse ThisTurn = RestrictionsAndShields
  -- "until [poss] next [part]" — the start boundary writes no boundary
  -- word, and takes possession or the definite article, never nothing.
  spanUse (Until (StartOf Turn Nothing)) = Unattested
  spanUse (Until (StartOf Turn (Just Yours))) = GrantsRestrictionsAndReplacement
  spanUse (Until (StartOf Turn (Just ThatPlayers))) = Unclaimed
  spanUse (Until (StartOf Upkeep Nothing)) = Unattested
  spanUse (Until (StartOf Upkeep (Just Yours))) = KeywordGrantOnly
  spanUse (Until (StartOf Upkeep (Just ThatPlayers))) = Unattested
  spanUse (Until (StartOf EndStep Nothing)) = Unclaimed
  spanUse (Until (StartOf EndStep (Just Yours))) = Unclaimed
  spanUse (Until (StartOf EndStep (Just ThatPlayers))) = Unclaimed
  spanUse (Until (StartOf Combat Nothing)) = Unattested
  spanUse (Until (StartOf Combat (Just Yours))) = Unattested
  spanUse (Until (StartOf Combat (Just ThatPlayers))) = Unattested
  spanUse (Until (StartOf UntapStep Nothing)) = Unattested
  spanUse (Until (StartOf UntapStep (Just Yours))) = Unattested
  spanUse (Until (StartOf UntapStep (Just ThatPlayers))) = Unattested
  -- "until (the) end of [part]" — the end boundary is the one that
  -- writes bare, and the bare forms are where the grants live.
  spanUse (Until (EndOf Turn Nothing)) = GrantsTypesControlAndReplacement
  spanUse (Until (EndOf Turn (Just Yours))) = ControlGrantOnly
  spanUse (Until (EndOf Turn (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf Upkeep Nothing)) = Unattested
  spanUse (Until (EndOf Upkeep (Just Yours))) = Unclaimed
  spanUse (Until (EndOf Upkeep (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf EndStep Nothing)) = Unattested
  spanUse (Until (EndOf EndStep (Just Yours))) = Unattested
  spanUse (Until (EndOf EndStep (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf Combat Nothing)) = GrantsAndControl
  spanUse (Until (EndOf Combat (Just Yours))) = Unclaimed
  spanUse (Until (EndOf Combat (Just ThatPlayers))) = Unattested
  spanUse (Until (EndOf UntapStep Nothing)) = Unattested
  spanUse (Until (EndOf UntapStep (Just Yours))) = Unattested
  spanUse (Until (EndOf UntapStep (Just ThatPlayers))) = Unattested
  -- "for as long as [condition]" — the one adverbial with no turn
  -- boundary in it at all ([CR#611.2b]), and the widest class in the
  -- table: the control grant writes it forty-two times, the stat delta
  -- six, the keyword grant four, the restriction ten, and the type
  -- ADDITION not once.
  spanUse (ForAsLongAs _) = GrantsRestrictionsAndControl
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

  ||| The continuous effects a resolving clause can establish
  ||| ([CR#611.2]) — the PART of the sentence that survives its
  ||| resolution, with the duration adverbial factored out onto the
  ||| envelope that carries it (`Continuously`). Core makes the same
  ||| cut: `Continuously { effect: StaticEffect, duration: Duration }`
  ||| (`deckmaste_core/src/effect.rs`, `continuous.rs`) puts every
  ||| lasting change behind one duration-bearing node, whether the
  ||| change is a characteristic modification (`StaticEffect::Modify`)
  ||| or a prohibition (`StaticEffect::Deontic`), and English agrees
  ||| with it: the adverbial is one slot, written once at the end of the
  ||| clause, and the three constructions below differ only in what
  ||| precedes it.
  |||
  ||| A SMALL slice deliberately: the stat delta, the keyword grant, and
  ||| the deed restriction are the rows this vocabulary has clauses for.
  ||| Core's other static rows — becoming a copy, losing abilities,
  ||| setting base power and toughness, conditional statics — arrive
  ||| with the ability layer, and each is a new `StaticKind` row that
  ||| the span tables will force to declare its adverbials.
  public export
  data StaticEffect : Bindings -> Type where
    -- "[n] gets [+p/+t]" — the stat-modifying continuous effect
    -- ([CR#613.4c] layer 7c, core's `Modification::PowerAndToughness…`).
    -- The one-shot form modifies a battlefield object
    -- (`badGetsGraveyard`); graveyard-reaching changes are static
    -- abilities, a later chapter.
    -- spelling: ["<Param(0)> gets <Param(1)>/<Param(2)>"] (signed pow/tou
    -- pair, e.g. "+1/+1"; the trailing duration adverbial belongs to the
    -- Continuously envelope, not here), kind: Sentence
    Gets : (n : Noun bs Object) -> (pow : Integer) -> (tou : Integer) ->
           {auto 0 ok : OnBattlefield (nounZone n)} -> StaticEffect bs
    -- "[n] gains [ability]" — the keyword grant, same battlefield
    -- discipline. The ability slot is the whole container now, and the
    -- gate is what keeps the widening honest: English grants an ACTIVATED
    -- ability by quoting it, which is not this clause's spelling
    -- (`Grantable`, `badGainsActivated`).
    -- spelling: ["<Param(0)> gains <Param(1)>"] (the trailing duration
    -- adverbial belongs to the Continuously envelope, not here),
    -- kind: Sentence
    Gains : (n : Noun bs Object) -> (ab : Ability) ->
            {auto 0 ok : OnBattlefield (nounZone n)} ->
            {auto 0 gr : Grantable ab} -> StaticEffect bs
    -- "[n] can't [deed, in a voice]" — the DEONTIC: a continuous effect
    -- denying its subject a deed, which is core's
    -- `Deontic(Cant(…))` under the same envelope
    -- (`deckmaste_core/src/deontic.rs`) with the two halves English
    -- writes. Verb and voice are separate arguments for core's own
    -- reason: core tells the readings apart by which SLOT carries the
    -- reference (`Block { by, on }`), so the deed and the part the
    -- subject plays in it are two facts, and fusing them into one word
    -- would spell "block"'s two voices as unrelated vocabulary. The
    -- deed is the restriction the declare steps check ([CR#508.1c] for
    -- attacking, [CR#509.1b] for blocking and for being blocked), and
    -- it beats any permission it meets ([CR#101.2]). Two demands, each
    -- a shape this grammar makes elsewhere: the subject stands on the
    -- battlefield (combat is fought there — [CR#506.4] takes a
    -- permanent that leaves out of combat; `badCantInGraveyard`) and
    -- carries the deed's grant IN THAT VOICE ([CR#506.3];
    -- `badCantAttackLand`, `badCantDisjunctSubject`, `badCantBeAttacked`).
    -- The third demand — that the span be the RESTRICTION's adverbial
    -- and not the grant's, and that it be written at all — is the
    -- envelope's (`SpanOk DeedRestriction`; `badCantUntilEndOfTurn`,
    -- `badStaticCant`). The subject may be plural — "Other creatures
    -- can't attack this turn." (Intimidation Bolt) is one sentence of
    -- many — so no grammatical number is demanded. The surface phrases
    -- are the macros' (`cantAttack`, `cantBlock`, `cantBeBlocked`).
    -- spelling: ["<Param(0)> can't <Param(1)+Param(2)>"], kind: Sentence
    -- (Params 1 and 2 spell ONE verb phrase, the deed word inflected by its
    -- voice -- Attack/Agent "attack", Block/Agent "block", Block/Patient
    -- "be blocked"; the duration adverbial is the Continuously envelope's.
    -- Mirrors core's Continuously-over-Cant pair -- no single RON
    -- constructor entry confirmed for the fused clause this pass)
    Cant : (n : Noun bs Object) -> (deed : Deed) -> (role : Role) ->
           {auto 0 zn : OnBattlefield (nounZone n)} ->
           {auto 0 dp : DeedParticipant deed role (nounTy n)} -> StaticEffect bs
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
                  {auto 0 zn : OnBattlefield (nounZone n)} ->
                  {auto 0 ne : LineNonEmpty added} ->
                  {auto 0 nw : AddsSomething (nounTy n) added} ->
                  {auto 0 af : AddedFits (nounTy n) added} -> StaticEffect bs
    -- "[who] gain(s) control of [what]" — the control-changing
    -- continuous effect ([CR#613.1b] layer 2; core's
    -- `Modification::SetController(Reference)`). ONE row for the whole
    -- English verb, span or none, which is a recorded DIVERGENCE from
    -- core: core keeps a one-shot `Action::GainControl` for the
    -- exchange family beside the layer-2 modification for the
    -- duration-bounded grants ("gain control until end of turn"), two
    -- constructors for what English writes with one verb and one
    -- optional trailing adverbial. Here the adverbial is already the
    -- `Continuously` envelope's slot, and the unwritten span is
    -- [CR#611.2a]'s end-of-game default the same way it is for every
    -- other static — which is what Phyrexian Infiltrator's reminder text
    -- says out loud, "(This effect lasts indefinitely.)" — so the split
    -- would spell one construction twice.
    -- The SUBJECT is a slot for the reason `ChangeLife` and `Draw` have
    -- one: both spellings are ordinary oracle. "Gain control of target
    -- creature" is the imperative with its unpronounced `You` (two
    -- hundred and forty-six lines), "[player] gains control of …" is
    -- written out (seventy-three), and the distributive subject is real
    -- too ("each player gains control of …", ten), so no grammatical
    -- number is demanded of it.
    -- The patient is a PERMANENT: [CR#110.2] gives every permanent a
    -- controller and [CR#109.4] gives objects that are neither on the
    -- stack nor on the battlefield none at all, so the battlefield
    -- demand tap, destroy, and "gets" already carry applies here too
    -- (`badGainControlGraveyard`). The stack half of [CR#109.4] is real
    -- English — "exchange control of target noncreature spell and target
    -- creature" — and waits with the spell carrier (ledger).
    -- spelling: ["<Param(0)> gain(s) control of <Param(1)>"] (the imperative
    -- leaves the agent unpronounced; the trailing duration adverbial belongs
    -- to the Continuously envelope, not here), kind: Sentence
    GainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
                   {auto 0 zn : OnBattlefield (nounZone what)} -> StaticEffect bs
    -- "if [event], [replacement] instead" / "the next time [event],
    -- [replacement] instead" — the REPLACEMENT effect as a continuous
    -- one, which is [CR#614.1]'s own first sentence ("Some continuous
    -- effects are replacement effects") and core's own filing
    -- (`StaticEffect::Replacement`, `deckmaste_core/src/continuous.rs`).
    -- The word is [CR#614.1a]'s: an effect that says "instead" is a
    -- replacement effect, and the corpus agrees at a thousand and
    -- thirty-nine lines.
    -- The intercepted event is a PATTERN and not a clause: nothing is
    -- being instructed there, so it takes `GameEvent` rather than an
    -- `Effect`, and which events are writable at all is the event
    -- table's answer (`Interceptable`; `badInterceptDestruction`).
    -- The replacement clause is typed in what the event ANNOUNCED, so
    -- "exile it" finds the creature the event named — and it is a HOLE
    -- outward, for the conditional arm's reason exactly ([CR#614.7]: if
    -- the intercepted event never happens the replacement "simply
    -- doesn't do anything", so nothing after the sentence may read it;
    -- `badInterceptReplacementAntecedent`).
    -- The multiplicity word is a slot because English makes it one and
    -- the event decides which: "if" for the once-only events, "the next
    -- time" for the repeatable ones ([CR#614.3]'s two endings; `ReplUse`,
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

  ||| HOW MUCH damage a shield stops — [CR#615.7]'s numbered shield
  ||| against the unnumbered one, and they are two rows rather than one
  ||| because the rule treats them differently: a numbered shield is
  ||| "used up" one damage at a time and an unnumbered one is not used up
  ||| at all, expiring only with its duration ([CR#615.3]). English marks
  ||| the same split with two determiners, "all" and "the next [N]", and
  ||| writes no third: no line prevents "half the damage" or "up to 3
  ||| damage".
  ||| The number is the ordinary magnitude vocabulary and no parallel
  ||| number path — eighty-five lines write a numeral ("Prevent the next
  ||| 3 damage that would be dealt to any target this turn", Healing
  ||| Salve) and the announced X is real too ("Prevent the next X damage
  ||| that would be dealt to you this turn"), which is exactly what
  ||| `Amount`'s written half already spells (`WrittenCount`).
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

  ||| WHO the shield stands in front of. Two rows, and the empty one is
  ||| not an omission: "Prevent all combat damage that would be dealt
  ||| this turn" (Fog, Holy Day, Darkness, Root Snare — five lines) names
  ||| no recipient at all and shields every damage event of its class,
  ||| which [CR#611.2c] calls modifying the rules of the game and gives
  ||| its own worked example of. The recipient row reads the damage
  ||| clause's own recipient table rather than a second one
  ||| (`DamageRecipient`), so what may be shielded is exactly what may be
  ||| damaged.
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
  staticKind (Gets _ _ _) = PtDelta
  staticKind (Gains _ _) = KeywordGrant
  staticKind (Cant _ _ _) = DeedRestriction
  staticKind (BecomesAlso _ _) = TypeAddition
  staticKind (GainsControl _ _) = ControlGrant
  staticKind (Intercepts _ _ _) = Replacement
  staticKind (Prevents _ _ _) = Prevention

  ||| What a continuous clause contributes to the discourse: its
  ||| subject, exactly as the one-shot clauses contribute theirs.
  ||| The two SHIELD rows have no subject and contribute what their
  ||| phrases announced instead: the intercepted event's own noun, and
  ||| the shield's recipient. Neither contributes what its clause WOULD
  ||| do, the replacement being a hole for [CR#614.7]'s reason.
  public export
  staticIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticIntro (Gets n _ _) = nomIntro n
  staticIntro (Gains n _) = nomIntro n
  staticIntro (Cant n _ _) = nomIntro n
  staticIntro (BecomesAlso n _) = nomIntro n
  staticIntro (GainsControl who what) = nomIntro what
  staticIntro (Intercepts ev repl use) = eventIntro ev
  staticIntro (Prevents kind size scope) = scopeIntro scope

  ||| Which clause a DIVISION writes. [CR#601.2d] and [CR#115.7f] both
  ||| name "divide or distribute" as one mechanic over one pair of
  ||| examples — "such as damage or counters" — and English writes that
  ||| one mechanic with two idioms and no third: the damage verb takes
  ||| an adverbial ("Arc Lightning deals 3 damage DIVIDED AS YOU CHOOSE
  ||| among one, two, or three targets", sixty-nine lines) and the
  ||| counter verb changes its own word ("DISTRIBUTE two +1/+1 counters
  ||| among one or two target creatures you control", Armament Corps,
  ||| forty-six lines). Nothing else is divided: "divided as you choose"
  ||| pairs with no other verb, and the two spellings never cross
  ||| ("counters divided as you choose" is zero lines, and so is
  ||| "distribute … damage"). So the table is closed at two, and a third
  ||| divided verb would be a row rather than a construction.
  |||
  ||| A recorded NARROWING of core, which takes a free `body` and reads
  ||| the share back through a `Count::Allotment` anaphor
  ||| (`deckmaste_core/src/effect.rs`, `count.rs`). That generality is
  ||| right for an engine and wrong for this grammar: no oracle line
  ||| writes a divided clause whose body is anything but one of these
  ||| two, and a free body would spell instructions English does not
  ||| have. The share stays implicit here for the same reason — the
  ||| words "divided as you choose" ARE the allotment, and no corpus
  ||| line mentions a member's share a second time.
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
  ||| counter verb demands the battlefield `PutCounters` demands, and
  ||| its `k = Object` is what stops a division of counters among
  ||| players.
  public export
  data DividedTakes : DivTag -> Noun bs k -> Type where
    DamageDivided : {auto 0 rk : DamageRecipient n} -> DividedTakes DivDamage n
    CountersDistributed : {auto 0 zn : OnBattlefield (nounZone n)} ->
                          DividedTakes DivCounters {k = Object} n

  ||| WHAT an exposure clause exposes — the two complements English
  ||| writes after "look at" and "reveal", measured rather than guessed.
  ||| A card GROUP is the dominant one and the one this chapter is built
  ||| around ("look at the top card of your library", two hundred twenty;
  ||| "look at the top <n> cards of your library", four hundred
  ||| forty-nine; "reveal the top card", ninety-nine; "reveal the top
  ||| <n> cards", ninety-five). A whole HAND is the other, and it is a
  ||| zone rather than a group of cards: "reveals their hand" (a hundred
  ||| twenty-eight), "look at target player's hand" and its opponent
  ||| twin (twelve and eleven),
  ||| "reveal your hand" (seven). Oracle never spells that one out as a
  ||| group — "reveal all cards in your hand" is zero lines — so the
  ||| zone phrase is the construction and not an abbreviation of one.
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
  ||| mention like any other and the whole point of the clause. The
  ||| HAND is a hole, and measured: every corpus line that reads a
  ||| revealed hand back reads it as a ZONE ("that player exiles a card
  ||| from IT", "you choose a card … from it"), which is a zone anaphor
  ||| this grammar has no word for — so the clause contributes its
  ||| possessor and nothing else, rather than inventing a card group
  ||| oracle never names (ledger).
  public export
  exposedIntro : {bs : Bindings} -> Exposed bs -> Bindings
  exposedIntro (ExposedCards n) = nomIntro n
  exposedIntro (ExposedZone z) = zoneDelta z ++ bs

  ||| A COST — what the activator PAYS, which is a different thing from
  ||| what an ability DOES ([CR#602.1a]: "the activation cost is everything
  ||| before the colon … must be paid by the player who is activating
  ||| it"). The type is what closes the ledger's cost-GRAMMAR entry: the
  ||| colon used to accept any clause at all, so "Draw a card:" was
  ||| writable and no rule made it so.
  |||
  ||| Its payloads REUSE the clause machinery rather than parallel it —
  ||| [CR#118.1] makes a cost "an action or payment" the payer carries
  ||| out, so
  ||| "Sacrifice a creature" is the sacrifice CLAUSE under `Do` and not a
  ||| second sacrifice verb. What is NOT a clause is the two SYMBOLS:
  ||| [CR#107.5] gives "{T}" the fixed meaning "tap this permanent", a
  ||| self-patient with no noun phrase in it, and the corpus writes the
  ||| symbol three thousand two hundred fifty-nine times against a
  ||| hundred seventy-one English tap clauses ("Tap an untapped creature
  ||| you control"), which is two surfaces and not one. Core keeps the
  ||| same split (`CostComponent::{Tap, Untap}` beside `Do(Action)`); the
  ||| settled Idris spec spells `{T}` as `Do (Tap This)` instead, and that
  ||| is a recorded DIVERGENCE — a semantic normalization this grammar
  ||| cannot make, the symbol being a different thing on the page.
  |||
  ||| What the grammar claims is the sentence's cost STRUCTURE and nothing
  ||| about the game: not payability ([CR#118.3] — "a player can't pay a
  ||| cost without having the necessary resources"), not timing, not the
  ||| total-cost pipeline ([CR#601.2f]'s increases and reductions, core's
  ||| `CostChange`), not the alternative-cost swap ([CR#118.9]). Those are
  ||| engine boundaries and stay ledgered.
  public export
  data Cost : Bindings -> Type where
    -- The mana component ([CR#202.1a]) — a symbol sequence, whose whole
    -- spelling is the symbols run together.
    -- spelling: ["<Param(0)>"] (the symbol list, no separator: "{1}{R}",
    -- "{X}{X}{G}" -- see ManaSymbol for the per-symbol rendering)
    Mana : ManaCost -> Cost bs
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
    -- "[a] fights [b]" ([CR#701.14a] — only battlefield creatures
    -- fight [CR#701.14b]; `badFightGraveyard`, `badFightLand`).
    -- Primitive, confirmed: both damages dealt simultaneously, which
    -- no clause sequence reproduces (a sequence deals two ORDERED
    -- events — see `karplusanYeti`), and no operative oracle text
    -- spells it out (reminder text only). No distinctness gate: a
    -- self-fight is defined ([CR#701.14c] — twice its power to
    -- itself); "another" is per-card templating (the `Other`
    -- modifier). Both slots are SINGULAR — the plural form is the
    -- reciprocal frame "those creatures fight each other" (ledger)
    -- — and participation reads the `combatant` grant, not a type
    -- name.
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
    -- order" is one adverbial phrase after another on the same
    -- placement); what the verb owns is the agreement between the rider
    -- and its patient's number (`ArrangementOk`, [CR#401.4]). Core splits
    -- the same fact into a second verb (`MoveGroup { group, arrangement,
    -- to }`) because its group term is a different sort; here plurality
    -- is a property of the one patient slot, so no second row is needed
    -- and none is minted.
    Move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
           {auto 0 ok : DestOk to} ->
           {auto 0 arr : ArrangementOk (nounPlur what) to} -> Effect bs
    -- "[who] gains/loses [amt] life" ([CR#119.3]) — core basis (merged).
    -- spelling: ["<Param(0)> gains <Param(1)> life", "<Param(0)> loses
    -- <Param(1)> life"] (selects on the embedded LifeOp, Up/Down; mirrors
    -- constructors.ron's `GainLife` entry / the merged ChangeLife family),
    -- kind: Sentence
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Effect bs
    -- "[who] draw(s) [amt] card(s)" ([CR#121.1] — a player draws by
    -- putting the top card of their library into their hand). Not a
    -- keyword action: [CR#701]'s list does not contain it, drawing being
    -- its own game action, so the clause is a row of its own beside
    -- `ChangeLife` rather than a `Composite` tag — and it carries its
    -- SUBJECT the same way and for the same reason, both spellings being
    -- ordinary oracle ("Draw a card." with the imperative's unpronounced
    -- `You`, a thousand nine hundred sixty-three lines; "Target player
    -- draws a card", twenty; "Each player draws a card", twenty-nine).
    -- The COUNT is the ordinary magnitude vocabulary and no parallel
    -- number path: "Draw two cards" (two hundred seventy-four) is a
    -- `Lit`, "Draw X cards" (seventy-six) is `XVal`, "Draw a card for
    -- each opponent who lost life this turn" (a hundred thirty-seven
    -- for-each lines) is the for-each amount, and "Draw cards equal to
    -- the sacrificed creature's power" (eighty-two) is a read.
    -- It introduces NOTHING but its subject, and that is measured rather
    -- than assumed: not one corpus line reads a drawn card back across a
    -- sentence boundary — "Draw a card." followed by "it"/"that card" is
    -- written zero times, and the three near misses are two reminder
    -- parentheticals and Fblthp, whose "if it entered from your library"
    -- reads the CREATURE that entered, not the card drawn. The drawn card
    -- IS read inside the coordination that reveals it ("Draw a card and
    -- reveal it. If it isn't a land card, discard it.", four lines),
    -- which is a verb-phrase coordination this grammar does not spell;
    -- that construction is what would reopen the question (ledger).
    -- spelling: ["<Param(0)> draw(s) <Param(1)> card(s)"], kind: Sentence
    -- (the imperative leaves the agent unpronounced; the noun "card"
    -- pluralises with the count, and at one the numeral is the article
    -- "a". A count that is READ rather than written extraposes -- "draw
    -- cards equal to <Param(1)>", the bare plural before the phrase --
    -- which is a linearization choice, unchecked here)
    Draw : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
           {auto 0 wc : WrittenCount amt} -> Effect bs
    -- "[who] look(s) at / reveal(s) [what]" — the EXPOSURE clause
    -- ([CR#701.20a,701.20e]). One clause for both verbs because they are
    -- one operation with two audiences (see `ExposeVerb`), and its
    -- subject is a slot for the reason `Draw`'s is: both spellings are
    -- ordinary oracle ("Look at the top four cards of your library" with
    -- the imperative's unpronounced `You`; "Target opponent reveals their
    -- hand", seventy-four lines; "Each player reveals the top card of
    -- their library", seventeen).
    -- Exposing MOVES nothing ([CR#701.20b]) — the looked-at cards are
    -- still in the library, which is why the clause's own retag is
    -- absent and why the next sentence's placement has somewhere to move
    -- them FROM.
    -- What it contributes is the point of the whole chapter: a look or a
    -- reveal over a positioned slice ASSEMBLES a group the following
    -- sentences may reach into, and that is the structural difference
    -- between "the rest" and every complement chapter twenty-two refused
    -- (finding 127 — two announcements are two mentions and never a
    -- pair; one slice phrase is one group).
    -- spelling: ["<Param(1)> look(s) at <Param(2)>", "<Param(1)>
    -- reveal(s) <Param(2)>"] (selects on Param(0), the ExposeVerb; the
    -- imperative leaves the agent unpronounced), kind: Sentence
    Expose : (v : ExposeVerb) -> (who : Noun bs Player) ->
             (what : Exposed (nomIntro who)) -> Effect bs
    -- "[who] mill(s) [amt] card(s)" — [CR#701.17a]: "that player puts
    -- that many cards from the top of their library into their
    -- graveyard". A row of its own beside `Draw` and for the same
    -- reasons: [CR#701] names it a keyword action but `VerbName`'s tags
    -- are the ones `Composite` spells over a `Move` body, and a mill is a
    -- move of a SLICE the sentence never names, so there is no patient
    -- phrase to wrap. It carries its subject like `Draw` ("Target player
    -- mills ten cards", a hundred eighteen lines; "Each player mills",
    -- thirty-four; the bare imperative, the rest of four hundred
    -- seventy-six).
    -- Unlike `Draw` it INTRODUCES its group, and the contrast is the
    -- rules' own rather than a corpus accident: [CR#701.17c] lets an
    -- effect that refers to a milled card find it "as long as that zone
    -- is a public zone", which a graveyard is ([CR#400.2]), while a draw
    -- puts its card in a hand, which is not — so the same [CR#400.7j]
    -- that licenses the mill read denies the draw one, and the corpus
    -- agrees exactly (sixty-one lines read a milled group back, zero read
    -- a drawn card). The READ FORMS are the among-restriction ("from
    -- among them", eighteen) and the "this way" participle ("milled this
    -- way", forty-three), neither of which this vocabulary spells yet
    -- (ledger); the mention is honest before its readers arrive.
    -- spelling: ["<Param(0)> mill(s) <Param(1)> card(s)"], kind: Sentence
    -- (the imperative leaves the agent unpronounced; "card" pluralises
    -- with the count and at one the numeral is the article "a")
    Mill : (who : Noun bs Player) -> (amt : Amount (nomIntro who)) ->
           {auto 0 wc : WrittenCount amt} -> Effect bs
    -- "search [zone] for [description]" ([CR#701.23a] — "look at all
    -- cards in that zone (even if it's a hidden zone) and find a card
    -- that matches the given description"). The rule gives the clause its
    -- three parts and this row is those three: who searches, which zone,
    -- and the description found.
    -- The description is a PREDICATE and not a noun phrase, because the
    -- zone is on the verb rather than in the phrase: written as a noun,
    -- "a creature card" would seed the battlefield ([CR#109.2]) and the
    -- clause would have to overwrite it. The found object's mention is
    -- the clause's own, placed in the zone it was found in — which is
    -- what the following "it"/"that card" reads ("Search your library for
    -- a land card, reveal it, put it into your hand, then shuffle",
    -- Sylvan Scrying).
    -- [CR#701.23e] is why the reveal is a separate clause and not a rider
    -- here: "if the effect that contains the search instruction doesn't
    -- also contain instructions to reveal the found card(s), then they're
    -- not revealed" — the exposure is written or it does not happen, so
    -- the grammar writes it as its own clause (three hundred eighty-eight
    -- search lines carry a reveal somewhere, ", reveal it," being two
    -- hundred eighty-two of them; the rest write none).
    -- COUNTED finds ("search your library for up to two basic land
    -- cards", a hundred six lines) and the type-free "a card" (ninety) wait
    -- on counted untargeted groups and a card-headed predicate
    -- respectively (ledger).
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
    -- "[whose] shuffle(s) [their] library" ([CR#701.24a] — "randomize the
    -- cards within it so that no player knows their order"). The patient
    -- is a library and the slot names WHOSE, which is core's Law-3
    -- reading of the same rule (`Shuffle(Selection)` over
    -- `LibraryOf(Reference)`, "the collection patient, not an agent").
    -- English elides the object almost always — "Then shuffle." is seven
    -- hundred ninety-four lines against seventeen for "shuffle your
    -- library" and four for "shuffles their library" — which is
    -- linearization's business, the searched library being the only one
    -- in scope.
    -- It DESTROYS discourse, and that is the row's one interesting fact:
    -- [CR#701.20d] says revealed cards that are reordered "stop being
    -- revealed and become new objects", and [CR#701.24a] leaves no player
    -- knowing the order, so a mention still sitting in the shuffled
    -- library cannot be referred to afterwards. `effIntro` drops exactly
    -- those (`badReadAfterShuffle`); a card the sentence already moved
    -- OUT is untouched, which is why every search writes its placement
    -- before its shuffle ([CR#701.24b] keeps the found cards out of the
    -- shuffle for the same reason).
    -- spelling: ["<Param(0)> shuffle(s) <Param(0)>'s library"] (the
    -- imperative leaves both the agent and the object unpronounced --
    -- "shuffle"), kind: Sentence
    Shuffle : (whose : Noun bs Player) -> Effect bs
    -- "[static effect] [duration]" — the clause whose resolution
    -- establishes a continuous effect for the span it states
    -- ([CR#611.2a] — it "lasts as long as stated"), which is core's
    -- `Continuously { effect, duration }` exactly
    -- (`deckmaste_core/src/effect.rs`). One envelope for all three
    -- constructions, because the adverbial is one slot: what varies is
    -- which static effect precedes it, and WHICH adverbials that
    -- construction writes is the corpus's answer, not the writer's
    -- (`SpanOk`, over `spanUse` and `absentOk`; `badGainsThisTurn`,
    -- `badCantUntilEndOfTurn`, `badGetsThisTurn`, `badStaticCant`).
    -- The unwritten span is the [CR#611.2a] end-of-game default, spelled
    -- as an explicit `Nothing` per the no-defaults convention and legal
    -- only where its construction has an answer for it.
    -- spelling: ["<Param(0)> <Param(1)>"], kind: Sentence (Param(0) = the
    -- static effect's own clause, Param(1) = the trailing duration
    -- adverbial, omitted entirely when Nothing; mirrors core's
    -- Continuously struct field-for-field)
    Continuously : (se : StaticEffect bs) -> (span : Maybe (Duration (staticIntro se))) ->
                   {auto 0 sp : SpanOk (staticKind se) span} -> Effect bs
    -- "[agent] create(s) [count] [characteristics] token(s) [arrival]"
    -- ([CR#111.1] — a token represents a permanent no card represents;
    -- [CR#111.3] — the creating effect defines its characteristics).
    -- Core's `Create { agent, count, token, riders }` field for field
    -- (`deckmaste_core/src/action.rs`), and the agent is an explicit slot
    -- there for a reason this grammar shares: "Its controller creates a
    -- 2/2 green Boar creature token" and "target opponent creates a
    -- tapped Treasure token" are real lines, so the creator is
    -- information, not always the imperative's unpronounced `You`.
    -- The COUNT is the ordinary magnitude vocabulary and no parallel
    -- number path: "Create two 1/1 white Soldier creature tokens" (Raise
    -- the Alarm) is a `Lit`, "Create a 1/1 green Elf Warrior creature
    -- token for each Elf you control" (Elven Ambush) is the for-each
    -- amount, and core's slot is its own `Count` likewise. That count is
    -- also where the mention's grammatical number comes from
    -- (`amtPlur`).
    -- VERIFIED rather than assumed: the create WORDING is the only one
    -- current oracle uses. Three thousand five hundred twenty-eight lines
    -- write "create … token"; the older "put a … token onto the
    -- battlefield" survives on ZERO of them (the two lines matching that
    -- pattern are a "nontoken permanent" restriction and a Lander token's
    -- own quoted land-search ability).
    -- Token COPIES ("a token that's a copy of …", core's
    -- `TokenSpec::Copy`) and the PREDEFINED names ("create a Treasure
    -- token", [CR#111.10], core's `TokenSpec::Named`) are the two other
    -- token positions and neither is here: copy machinery is its own axis
    -- and the predefined catalog its own registry (ledger).
    -- spelling: ["<Param(0)> create(s) <Param(1)> <Param(2)> token(s)
    -- <Param(3)>"], kind: Sentence (Param(2) = the characteristics in the
    -- fixed adjective order, see TokenChars; Param(3) = the arrival relative
    -- clause, omitted when empty, see TokenRider; the imperative leaves the
    -- agent unpronounced. Mirrors core's Create struct field for field)
    Create : (agent : Noun bs Player) -> (count : Amount (nomIntro agent)) ->
             (tok : TokenChars) -> (riders : List TokenRider) ->
             {auto 0 tt : TokenTyped tok} ->
             {auto 0 tp : TokenPt tok} ->
             {auto 0 sf : SubtypesFit tok} ->
             {auto 0 tc : TokenCanonical tok} ->
             {auto 0 wc : WrittenCount count} ->
             {auto 0 rr : RidersOk riders} -> Effect bs
    -- "Put [amt] [kind] counter(s) on [n]" ([CR#122.1]) — core's
    -- `PutCounters(Reference, CounterRef, Count)`, with the arguments in
    -- TEXTUAL order as every clause here has them (core writes the
    -- reference first; the axes are the same three). AGENT-SILENT, and
    -- core says so in the same words: `PutCounters` is on its
    -- agent-carrying-none list, and the corpus agrees — no line writes
    -- "[player] puts a +1/+1 counter on", the verb being the effect's own
    -- imperative.
    -- The recipient is an OBJECT on the battlefield. [CR#122.1] puts
    -- counters on players too, but English does not put them there with
    -- this verb: "put a … counter on [a player]" is written zero times
    -- and the player form takes a different verb entirely ("target player
    -- gets a poison counter", forty-seven lines), so it waits on that
    -- verb (ledger). The battlefield demand is the one destroy, tap, and
    -- "gets" already carry (`badPutCountersGraveyard`); no corpus line
    -- puts a counter on a card in a graveyard or in exile with this
    -- clause.
    -- spelling: ["put <Param(0)> <Param(1)> counter(s) on <Param(2)>"],
    -- kind: Sentence (Param(1) = CounterKind's own word; the noun "counter"
    -- pluralises with the count, and at one the numeral is the article "a")
    PutCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                  (on : Noun (amtIntro amt) Object) ->
                  {auto 0 wc : WrittenCount amt} ->
                  {auto 0 pm : PerMember on} ->
                  {auto 0 zn : OnBattlefield (nounZone on)} -> Effect bs
    -- The DIVISION ([CR#601.2d]) — one written amount split over the
    -- members of one group mention, the split announced as the spell is
    -- cast rather than chosen on resolution. That announcement is the
    -- whole reason the division is a structure and not an adverb: the
    -- targets and their shares are fixed together at [CR#601.2d], and
    -- [CR#115.7f] then holds the shares still even when the targets
    -- themselves are changed later ("the original division can't be
    -- changed"), which is a fact about a division as an object. The
    -- companion rule [CR#608.2d] runs the same split at RESOLUTION for
    -- untargeted recipients ("distribute that many +1/+1 counters among
    -- any number of creatures you control") and is the same structure
    -- read at a different time; the group mention is what differs, so
    -- no second row is needed here.
    -- The floor [CR#601.2d] states — "each of these targets must
    -- receive at least one of whatever is being divided" — is NOT typed
    -- here. It is a legality question about the announced numbers
    -- (a division of two among three targets is an illegal
    -- announcement, not an ungrammatical sentence), and it goes to the
    -- legality layer beside [CR#701.14b]'s both-or-neither fight guard
    -- and [CR#701.12a]'s all-or-nothing exchange rule.
    -- The recipient is a GROUP MENTION and PLURAL, which is the
    -- division's own demand rather than a borrowed one: splitting needs
    -- members to split among, and every corpus line writes a counted
    -- target group or a plural read after "among"
    -- (`badDivideAmongSingular`, `badDivideAmongDescription`).
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
    -- "Remove [amt] [kind] counter(s) from [n]" — the twin, and core's
    -- `RemoveCounters` beside `PutCounters` for the same reason: removal
    -- is not a negative put (it is cost-eligible where a put is not, and
    -- it can fail for want of counters), so the two are separate verbs
    -- there and separate rows here. Four hundred five corpus lines
    -- remove counters; the flagship is Chainbreaker's "{3}, {T}: Remove a
    -- -1/-1 counter from target creature."
    -- What this row does NOT spell is the quantifier "all" ("Remove all
    -- counters from target creature") and the kind-blind "a counter",
    -- which are core's own `CounterSpec::AllKinds` and a bare-count
    -- reading; both want a quantity over KINDS that the written amount
    -- vocabulary has no term for (ledger).
    -- spelling: ["remove <Param(0)> <Param(1)> counter(s) from <Param(2)>"],
    -- kind: Sentence (as PutCounters, with the preposition "from")
    RemoveCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                     (from : Noun (amtIntro amt) Object) ->
                     {auto 0 wc : WrittenCount amt} ->
                     {auto 0 zn : OnBattlefield (nounZone from)} -> Effect bs
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
    -- "[subject] [verb phrase]" — the declarative clause: the verb's
    -- performer in subject position, its phrase typed after it.
    -- Verbs the CR gives a player actor ([CR#701.21a,701.9a]-family)
    -- REQUIRE one — they have no `NonAgentive` row, so `Composite`
    -- refuses them (`badAgentlessSacrifice`) — and their imperative
    -- supplies the unpronounced subject as an explicit `You`.
    -- Effect-verbs take a subject OPTIONALLY: oracle text writes
    -- destroy and exile both ways ("You destroy four lands you
    -- control, then target opponent destroys four lands they
    -- control." — Burning of Xinye; "Each player exiles two cards
    -- from their hand."). This is core's per-verb `who` slot factored
    -- to clause position — a dependent context can't re-use the
    -- subject term at each inner slot the way the real macros ride
    -- their agent param — and lowering redistributes it; `ChangeLife`
    -- carries its `who` the same way. Object sources (DealDamage's
    -- src) are the verb's own argument, not a subject. The clause
    -- carries its verb TAG directly, and the tag's body obligations
    -- ride `TagBody` here exactly as under `Composite`.
    -- Overgeneration accepted: a subject with no choice of its own
    -- ("You destroy target creature") is spellable, though oracle
    -- style writes the bare imperative there.
    -- spelling: (construction-owned -- subject + verb-tag clause, e.g.
    -- "<Param(0)> sacrifices <Param(2)>"/"<Param(0)> discards <Param(2)>";
    -- the verb's own lemma is VerbName's, conjugation is auto-inflection --
    -- see the sacrifice/discards macros in Experimental.Macros), kind:
    -- Sentence
    Does : (subj : Noun bs Player) -> (v : VerbName) ->
           (e : Effect (nomIntro subj)) ->
           {auto 0 tb : TagBody v e} -> Effect bs
    -- "[who] pay(s) [cost]" — the payment as a CLAUSE, which is what the
    -- unless family needs and the only reason this row exists: [CR#118.12a]
    -- rewrites "[Do something] unless [a player does something else]" into
    -- "[A player may do something else]. If [that player doesn't], [do
    -- something]", so the may's BODY has to be able to say "pays {3}" —
    -- a hundred forty-five "unless you pay" lines and a hundred
    -- eighty-five "unless [someone] pays". The complement is a whole
    -- `Cost` and not a mana amount, which is measured: a hundred
    -- seventy-eight of those complements are one symbol run, twenty-four
    -- are "N life", and the rest are the cost words this vocabulary does
    -- not spell (echo, upkeep, "its mana cost" — ledger).
    -- The clause is NOT the cost type wearing a verb: a cost is paid to
    -- activate ([CR#602.1a]) where this is a sentence that resolves, and
    -- the same `Cost` value stands in both places exactly as core's
    -- `Action::Pay(Cost)` does.
    -- spelling: ["<Param(0)> pay(s) <Param(1)>"], kind: Sentence (the verb
    -- inflects with the subject -- "you pay {3}" against "its controller
    -- pays {3}"; a life component under this verb writes "3 life", the
    -- shared verb swallowing the component's own "Pay")
    Pay : (who : Noun bs Player) -> (c : Cost (nomIntro who)) ->
          {auto 0 pb : Payable c} -> Effect bs
    -- "[decider] may [effect]" — the decider slot ([CR#608.2d]; the
    -- resolving default is the controller [CR#608.2c]). Decider and
    -- performer can differ ("[player] may have [source] deal … to
    -- them"), so the body is any clause, not the decider's own verb
    -- phrase.
    -- The two BRANCHES are the anaphoric conditionals "if you do" and
    -- "if you don't" — a thousand and sixty-two corpus lines write the
    -- first after a "may", eighty-three the second — and they are
    -- fields on this node rather than `Condition` rows, which is
    -- core's shape exactly (`May { who, effect, if_did, if_not }`,
    -- `deckmaste_core/src/effect.rs`; a branchless may is the same node
    -- with both `None`). The reason is that the anaphor has no
    -- referent of its own to condition ON: "if you do" asks whether the
    -- immediately preceding OPTIONAL ACTION was taken, which is not a
    -- fact about the board and not a mention in the discourse, so
    -- making it a condition would need a channel recording what the
    -- last clause offered. The branch reads it structurally instead.
    -- The two arms are typed differently, and that asymmetry is the
    -- finding: `ifDid` runs only when the body ran, so it reads
    -- everything the body introduced ("You may sacrifice a creature.
    -- If you do, each opponent discards a card." — Braids's Frightful
    -- Return); `ifNot` runs only when the body did NOT, so the body's
    -- mentions never existed and it reads only what preceded the may
    -- (`badIfNotReadsMayBody`). Corpus agrees: every if-not arm read
    -- this pass reaches the DECIDER ("If they don't, they lose 2
    -- life") or the sentences before the may (Chandra's "that card"),
    -- never the may body's own phrase.
    -- What the branch actually reads is settled by rule and is worth
    -- stating exactly: [CR#118.12] makes the offered action a COST paid
    -- on resolution, and has the "if [a player] does" clause check
    -- "whether the player chose to pay an optional cost or started to
    -- pay a mandatory cost, regardless of what events actually
    -- occurred". So the branch is not an event read at all — which is
    -- why the declined arm has nothing to mention, the payment having
    -- never been started, and why no channel recording outcomes would
    -- have served. The ELSE sentence "Otherwise, …" is `If`'s branch and
    -- not this one's.
    -- The MANDATORY twin ("[Do something]. If you do, …", with no "may",
    -- a hundred and forty-two lines) is THIS NODE with the offer emptied,
    -- and that is chapter eighteen's ledger entry read the way its own
    -- rule reads: [CR#118.12] states both shapes in one sentence and gives
    -- them one reader, checking "whether the player chose to pay an
    -- optional cost OR STARTED TO PAY A MANDATORY COST, regardless of what
    -- events actually occurred". Every consequence therefore carries over
    -- unchanged — the arm asymmetry, the opacity of the join, the deed
    -- that never happened (the rule's own Standstill example is a
    -- mandatory sacrifice that could not be paid) — so a second row would
    -- have duplicated `mayIntro`, `annIntro`, `deedDelta` and the
    -- refusals with them. The offer is the ONLY difference, so it is the
    -- only thing that varies: `Nothing` is the bare instruction, whose
    -- "if you do" takes its pronoun from the BODY's own agent rather than
    -- from a decider the sentence never wrote.
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
    -- "[clause] if [condition]" — the ordinary conditional, whose "if"
    -- has "only its normal English meaning" ([CR#603.4]).
    -- Argument order is TEXTUAL order and the binding flow is why:
    -- oracle writes the trailing conditional six hundred and ninety
    -- times, and its condition reads the clause's own mentions
    -- ("Destroy target artifact if its mana value is 2 or less" —
    -- Overload; "its" is the artifact the main clause targeted), so
    -- the condition is typed in the clause's post-context like every
    -- other trailing argument in this grammar. The LEADING
    -- linearization ("If you control a creature, …", four hundred
    -- eighty-six lines) is this same node whenever the condition uses
    -- nothing the clause introduced, which is the ordinary case; the
    -- leading form that DOES read back is the target-announcing family
    -- and is refused (`condDelta`, `badMatchesTargetSubject`).
    -- The DIVERGENCE chapter eighteen wrote here has landed, and it is
    -- closed: the condition is evaluated before the clause it modifies
    -- takes effect but WRITTEN after it, so typing it in the clause's
    -- post-state let it read a world the clause had not made — a
    -- zone-sensitive trailing condition was named as the thing to watch
    -- for, and "Destroy target creature if it's in a graveyard" is it
    -- (`badTrailingPostStateZone`). The condition is typed in
    -- `preIntro` now: what the clause's phrases ANNOUNCED, with the
    -- announced zone, and nothing the clause did. Overload is unmoved —
    -- mana value belongs to every object [CR#202.3] and is read
    -- zone-free — which is what makes this a correction to the timing
    -- rather than to the textual-order design.
    -- The conditioned clause is a HOLE outward (`effIntro`), for the
    -- reason the else arm below is one: the condition may be false, and
    -- then this clause never ran and its phrase named nothing.
    -- The ELSE arm ("Otherwise, …", a hundred and seventy-five lines) is
    -- the third slot chapter eighteen designed and could not fill, opened
    -- here now that a card's both arms are writable: Unholy Annex's "If
    -- you control a Demon, each opponent loses 2 life and you gain 2
    -- life. Otherwise, you lose 2 life." A `Maybe` field on this node
    -- rather than a `Sequentially` element, because an else-arm has no
    -- meaning without the "if" that governs it where a sequence's
    -- elements are independent clauses — and it is `May`'s `ifNot`
    -- exactly, typed the same way and for finding 73's reason: the arm
    -- runs when the condition was FALSE, so the main clause never
    -- happened and its phrase never named anything. The arm is therefore
    -- typed in `bs`, the discourse BEFORE the conditional, and reads none
    -- of what `e` introduced (`badOtherwiseReadsIfArm`). It contributes
    -- nothing outward either (`effIntro`), the arm that REPLACES the main
    -- line being a hole exactly as `mayIntro` has it.
    -- Only the LEADING linearization spells it: "Otherwise" needs its
    -- "if" in front of it, so a trailing conditional with an else arm has
    -- no word order — one more linearization side condition this node
    -- does not check.
    -- spelling: ["<Param(0)> if <Param(1)>", "If <Param(1)>, <Param(0)>",
    -- "If <Param(1)>, <Param(0)>. Otherwise, <Param(2)>."] (the leading
    -- order is available only when the condition reads nothing the clause
    -- introduced -- a linearization side condition, unchecked here, like the
    -- leading/trailing choice on Delayed -- and the else arm is available
    -- only in the leading order), kind: Sentence
    If : (e : Effect bs) -> (c : Condition (preIntro e)) ->
         (otherwise : Maybe (Effect bs)) -> Effect bs
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
    -- the SIMULTANEOUS batch — one instruction whose parts happen at
    -- once, mirroring core's `OneShotEffect::Simultaneously`. Two senses
    -- of the word keep getting conflated and this container is only the
    -- first: ONE effect whose events happen simultaneously is this node,
    -- an instruction the card writes as a single verb ("exchange control
    -- of A and B") whose parts have no order between them. Separate
    -- effects that merely happen to resolve together is the OTHER sense
    -- — two clauses of one spell, or two spells resolving in a turn —
    -- and that is `Sequentially` and the stack, not this.
    -- The SHAPE is the semantic contrast with `Sequentially` beside it,
    -- and it is one word: the sequence threads `effIntro` left to right,
    -- so each clause reads what its predecessors DID; this threads
    -- `preIntro`, so each element reads only what its predecessors
    -- ANNOUNCED and nothing any of them did. That is chapter
    -- twenty-one's own distinction ([CR#601.2c] announces a phrase as
    -- the text is written, whatever resolution later makes of it), and
    -- it is what "one pre-state" means precisely: every element reads
    -- ONE game state — [CR#608.2f] is the general rule, "each such action
    -- is processed simultaneously" for one instruction spread over
    -- several objects, and its own example is a control grant (Blatant
    -- Thievery gains control of every target at once) — while the
    -- DISCOURSE still accumulates, because
    -- the announcements were all made before any of it resolved. An
    -- element reading a sibling's retag, its created token, or its
    -- damage outcome is refused (`badSimultaneousReadsRetag`,
    -- `badSimultaneousReadsOutcome`), which is exactly the pre-state
    -- reading an exchange depends on ([CR#701.12b] has each player gain
    -- control of what "was controlled by the other player" — both halves
    -- read the controllers as they stood BEFORE either half applied).
    -- OUTWARD it contributes the batch's whole discourse, and that is
    -- measured rather than assumed: Volatile Stormdrake reads its
    -- exchanged target in the very next breath ("exchange control of
    -- this creature and target creature an opponent controls. If you do,
    -- … sacrifice that creature …"), and Sudden Substitution does the
    -- same ("Then the spell's controller may choose new targets for
    -- it."), so a hole would be wrong. The answer is written as the last
    -- element's own `effIntro` over the announcement telescope, which
    -- IS the union for every row this container reaches today — the
    -- control grant introduces its phrase and retags nothing. A batch
    -- whose EARLIER element moved an object would lose that retag and
    -- force the union to be built rather than read off the end; no
    -- corpus line writes one (the zone-exchange family [CR#701.12d]
    -- names is the ledger's, not this row's).
    -- Hygiene mirrors the sequence's, and the arity demand is the same
    -- word: at least TWO elements, an empty batch being no instruction
    -- and a one-element batch a second spelling of that element
    -- (`badEmptySimultaneous`, `badSingletonSimultaneous`). Its elements
    -- are neither batches (`NotSim`, `badNestedSimultaneous` — the
    -- re-minted tree `NotSeq` refuses one construction over) nor
    -- SEQUENCES (`NotSeq`, `badSequenceInsideSimultaneous`): an ordered
    -- list inside an unordered one contradicts the container it sits in,
    -- and its `preIntro` would export only its last clause's
    -- announcements besides. No corpus line writes either.
    -- ALL-OR-NOTHING is [CR#701.12a]'s ("if the entire exchange can't be
    -- completed, no part of the exchange occurs") and core's doc says so
    -- too; it is a resolution fact about failed halves, not a typing
    -- one, so it lives with the legality layer the fight guard waits in.
    -- spelling: (construction-owned -- the batch has no connective word of
    -- its own: English writes it as ONE verb, and which verb is the macro's
    -- ("exchange control of <a> and <b>", "<a> fights <b>"). Mirrors core's
    -- n-ary `OneShotEffect::Simultaneously`), kind: TODO(reason: a
    -- multi-clause body isn't one of the five FragmentKinds -- each element
    -- is its own Sentence, and no element is a sentence the card prints)
    Simultaneously : {0 n : Nat} -> SimEffects n bs ->
                     {auto 0 ok : AtLeastTwo n} -> Effect bs
    -- "Choose [q] — • [mode] • [mode] …" — the MODAL clause
    -- ([CR#700.2]: a spell or ability is modal when it has "two or more
    -- options in a bulleted list preceded by instructions for a player to
    -- choose a number of those options"). The rule supplies both halves
    -- of the node's shape: the headcount is a QUANTITY, so it is the one
    -- quantity vocabulary and no parallel number path (`exactly`,
    -- `upTo`, and the new `atLeast`/`oneOrBoth` over the same `Range`
    -- primitive), and the modes are two or more (`AtLeastTwo`), which is
    -- the rule's own minimum and not a presumption — no one-mode modal
    -- exists in corpus, and by [CR#700.2] none could.
    -- The attested headcounts, by card: "Choose one —" four hundred
    -- seventy, "Choose two —" thirty-two, "Choose three —" one (Mishra,
    -- Eminent One, over six modes), "Choose one or both —" fifty-two,
    -- "Choose one or more —" nineteen, "Choose up to one —" seven. The
    -- brief's "up to two"/"up to three" are written ZERO times, all
    -- scopes; so are "choose four", "two or more", and "one or two".
    -- The MODE LIST IS A LIST AND NOT A TELESCOPE — the whole finding of
    -- this row, and the structural difference from `Effects` beside it.
    -- Every mode is typed in `bs`, the discourse the modal itself stands
    -- in, so a mode reads everything BEFORE the modal and nothing a
    -- sibling mode introduced. Both directions are measured. Inward: the
    -- six bullets that open with a pronoun all reach past the modal to
    -- the trigger that precedes it ("When Kogla and Yidaro enters, choose
    -- one — • It gains trample and haste until end of turn. • It fights
    -- target creature you don't control." — BOTH modes read the same
    -- outside antecedent, neither reads the other; likewise Blizzard
    -- Specter's "That player", Judith's "That spell", Kitsune Ace's "That
    -- Vehicle", Gylwain's and Pip-Boy 3000's "that creature"). Across:
    -- zero bullets read a sibling, and [CR#700.2a] says why — the modes
    -- are chosen at cast, and an unchosen mode's targets are never
    -- announced at all — [CR#700.2c] has the spell "treated as though it
    -- did not have those targets" — so a sibling's phrase may have named
    -- nobody at all.
    -- OUTWARD it contributes nothing (`effIntro`), which is the same fact
    -- pointed forward and is equally measured: of the forty-eight modal
    -- cards with a non-bullet line after the list, all but two are
    -- keyword packaging (entwine, equip, cycling, flashback, rebound,
    -- crew, suspend, reinforce), and neither survivor reads a mode.
    -- Thermal Flux's trailing line names nothing, and Blood on the Snow
    -- is the positive proof: "Choose one — • Destroy all creatures. •
    -- Destroy all planeswalkers. Then return a creature or planeswalker
    -- card … from your graveyard" writes a DESCRIPTION covering both
    -- modes' outcomes exactly where an anaphor would have gone. So this
    -- is `predDelta (Or _) = []` and `condDelta = []` one construction
    -- up, and for their reason (`badModalReadsAcrossModes`,
    -- `badReadsAfterModal`).
    -- What is NOT here, and is keyword PACKAGING rather than grammar:
    -- entwine (thirty-two cards — "Choose both if you pay the entwine
    -- cost"), spree (twenty-one), escalate (nine), and the "You may
    -- choose the same mode more than once" instruction ([CR#700.2d], six
    -- lines). The pawprint worth-of-modes form ([CR#700.2i]) and the
    -- per-mode additional cost ([CR#700.2h]) are the cost algebra's.
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
    -- "[e] [when/at event-query]" — the temporal adverbial stays on
    -- its clause (leading vs trailing position is linearization); the
    -- body reads the discourse as settled particulars transformed by
    -- the event (`delayedCtx`, [CR#603.7c,603.3d]).
    -- spelling: ["<Param(1)> <Param(0)>"] (trailing adverbial position;
    -- leading position swaps the order, linearization's choice -- see
    -- EventQuery), kind: Sentence
    Delayed : (ev : EventQuery bs) -> Effect (delayedCtx ev) -> Effect bs
    -- "[replaced]. [replacement] instead." — the SELF-replacement
    -- ([CR#614.15]: an effect of a resolving spell or ability that
    -- replaces part or all of that spell or ability's OWN effects, and
    -- the rule adds that the text creating one "is usually part of the
    -- ability whose effect is being replaced", which is why English
    -- writes it as the next sentence rather than as a standing shield).
    -- Distinguished from `Intercepts` beside it by what it names: the
    -- intercepting form names an EVENT PATTERN and waits, this form
    -- names the very clause before it and substitutes for it. The
    -- corpus divides on the word "would": the interception writes one
    -- (six hundred twenty-six lines pair "would" with "instead") and
    -- this form writes none (four hundred thirteen "instead" lines have
    -- no "would" anywhere).
    -- The replacement is typed in what the replaced clause ANNOUNCED
    -- and not in what it did, which is the whole content of the node.
    -- [CR#614.6] says a replaced event "never happens", so the replaced
    -- clause's outcome is not there to read ("that much" reads nothing
    -- — `badInsteadReadsReplacedOutcome`); [CR#601.2c] announces its
    -- targets all the same, so "that artifact" and "that creature" DO
    -- find it. Twenty-four corpus lines turn on exactly that
    -- ("Bot Bashing Time deals 6 damage to target creature. If that
    -- creature would die this turn, exile it instead.").
    -- Outward it contributes the same announcement and no outcome:
    -- exactly one of the two clauses ran and no later sentence can know
    -- which, which is `mayIntro`'s cut ([CR#118.12]) at a second site.
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
    -- "[clause] until [event]" — the [CR#610.3] rider, and it is NOT a
    -- duration: the rule makes this "one-shot effect" pair with "a
    -- second one-shot effect … created immediately after the specified
    -- event", which returns the object to its previous zone. So the
    -- clause resolves once and schedules its own undo, where a
    -- `Continuously` clause establishes something that lasts.
    -- WHICH clauses take the rider is the corpus's answer and it is one
    -- (`heldUntilOk`): eighty-six of the ninety-one "until [object]
    -- leaves the battlefield" lines are an exile, three are the
    -- phase-out twin [CR#610.4] governs and this vocabulary has no word
    -- for, and the remaining three are continuous effects that belong to
    -- the `Duration` row instead (`badHeldUntilDestroy`).
    -- [CR#610.3c] settles what the undo does not need to say: the object
    -- "returns under its owner's control unless otherwise specified", so
    -- the rider needs no controller slot and the clause writes none.
    -- The event is typed in what the clause announced (`preIntro`), so
    -- the exiled object and the watched permanent are two phrases in one
    -- sentence; and the node contributes that same announcement OUTWARD
    -- rather than the exile's zone retag, because the object's zone
    -- depends on an event that has not happened — it may be in exile or
    -- back on the battlefield when a later sentence reads it
    -- (`badHeldUntilExileRetag`).
    -- spelling: ["<Param(0)> until <Param(1)>"] (trailing rider, no
    -- comma; the event is spelled in the finite mood -- see GameEvent),
    -- kind: Sentence
    HeldUntil : (e : Effect bs) -> (ev : GameEvent (annIntro e)) ->
                {auto 0 ok : HeldClause e} ->
                {auto 0 hd : Holdable ev} -> Effect bs

  ||| Which clause may carry a [CR#610.3] "until [event]" rider. Full
  ||| rows, so a new clause declares whether the corpus hangs one on it.
  ||| Exactly one row is `True` and it is a nested pattern rather than a
  ||| constructor name: the rider goes on a zone change to EXILE and on
  ||| nothing else — "exile target creature an opponent controls until
  ||| this creature leaves the battlefield" (Banisher Priest) and its
  ||| eighty-five siblings. A destroy or a graveyard move takes none
  ||| (nothing returns from a graveyard "until"), and neither does a
  ||| bare `Move`: the corpus writes the rider with the verb tag every
  ||| time, which is what the exile macro spells.
  public export
  heldUntilOk : {0 bs : Bindings} -> Effect bs -> Bool
  heldUntilOk (DealDamage _ _ _) = False
  heldUntilOk (Distribute _ _ _) = False
  heldUntilOk (Fights _ _) = False
  heldUntilOk (Tap _) = False
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
  heldUntilOk (Composite Exile (Move _ _)) = True
  heldUntilOk (Composite _ _) = False
  heldUntilOk (Does _ _ _) = False
  heldUntilOk (Pay _ _) = False
  heldUntilOk (May _ _ _ _) = False
  heldUntilOk (If _ _ _) = False
  heldUntilOk (Sequentially _) = False
  heldUntilOk (Simultaneously _) = False
  heldUntilOk (Modal _ _) = False
  heldUntilOk (Delayed _ _) = False
  heldUntilOk (InsteadOf _ _) = False
  heldUntilOk (HeldUntil _ _) = False

  ||| May this cost stand as the complement of the VERB "pay"? A cost is
  ||| the whole thing an ability charges; "pay" is one English verb, and
  ||| the two are not the same set. The complements were counted where the
  ||| verb appears most: a hundred seventy-eight "unless [someone] pays"
  ||| lines take one symbol run, twenty-four take "N life", and every
  ||| other payment after "unless" writes its OWN verb ("unless you
  ||| sacrifice a land", "unless you discard a card" — seventy-one lines
  ||| across sacrifice, discard, exile and tap), never "pay". So a
  ||| sacrifice is a payment but not a payABLE, and saying otherwise would
  ||| spell "you pay sacrifice a creature" (`badPayBySacrificing`). The
  ||| symbols are refused for the same reason and more sharply: "{T}" is a
  ||| cost that can only be written before a colon, no sentence anywhere
  ||| spelling it as a verb phrase. The compound is refused as unattested
  ||| — no corpus line writes "pay" over a comma-joined cost.
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

  ||| May this clause stand as a payment? [CR#602.1a] makes a cost what
  ||| the ACTIVATOR pays, so the table is not "is this a legal sentence"
  ||| but "does oracle write this before a colon" — and the corpus answers
  ||| per verb, which is why this is a table and not a blanket yes. The
  ||| counts are components of symbol-led activation costs, so they
  ||| UNDERCOUNT the action-only costs ("Sacrifice this creature:", which
  ||| leads its own line): sacrifice eleven hundred eighty-four, discard
  ||| two hundred twenty, remove-counters two hundred twelve, exile a
  ||| hundred ninety-one, pay-life ninety-five, tap forty-four, return
  ||| twenty-nine, put-counters fourteen, reveal seven, mill five. The
  ||| refusals are measured the same way and are zeroes: no line writes
  ||| "Destroy …:" or "Draw …:" or a shuffle or a search as a cost, which
  ||| is the ledger's own note that "you lose N life" and a delayed clause
  ||| are not payments ([CR#118.1]). Core's `Action::is_cost_eligible`
  ||| lists the same verbs and reaches them from the engine side.
  |||
  ||| The LIFE row is directional, and measured: ninety-five "Pay N life"
  ||| components against zero gain-life ones, so `Down` pays and `Up` does
  ||| not (`badGainLifeCost`). Core admits the whole family, and a
  ||| gain-life cost does exist — [CR#119.7] speaks of "a cost that
  ||| involves having that player gain life" — but the cards that print
  ||| one (Invigorate) spell it as an ALTERNATIVE cost ([CR#118.9]), a
  ||| base swap, never before a colon. The divergence is the frame's.
  public export
  costActionOk : {0 bs : Bindings} -> Effect bs -> Bool
  costActionOk (DealDamage _ _ _) = False
  costActionOk (Distribute _ _ _) = False
  costActionOk (Fights _ _) = False
  costActionOk (Tap _) = True
  costActionOk (Choose _) = False
  costActionOk (Move _ _) = True
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
  costActionOk (PutCounters _ _ _) = True
  costActionOk (RemoveCounters _ _ _) = True
  costActionOk (Composite Exile _) = True
  costActionOk (Composite _ _) = False
  costActionOk (Does _ Sacrifice _) = True
  costActionOk (Does _ Discard _) = True
  costActionOk (Does _ _ _) = False
  costActionOk (Pay _ _) = False
  costActionOk (May _ _ _ _) = False
  costActionOk (If _ _ _) = False
  costActionOk (Sequentially _) = False
  costActionOk (Simultaneously _) = False
  costActionOk (Modal _ _) = False
  costActionOk (Delayed _ _) = False
  costActionOk (InsteadOf _ _) = False
  costActionOk (HeldUntil _ _) = False

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
  ||| clauses' payloads generally inhabit two different types and cannot
  ||| be compared at all. What CAN be compared is compared — the verb
  ||| tag, a same-context noun (`nounEqRef`), a destination zone, and the
  ||| written count of the imperative draw, whose subject is spelled
  ||| `You` and so puts both amounts in one context. Per-row catch-alls,
  ||| so a new clause is a totality error on its missing row.
  public export
  effEq : {0 bs : Bindings} -> Effect bs -> Effect bs -> Bool
  effEq (DealDamage _ _ _) _ = False
  effEq (Distribute _ _ _) _ = False
  effEq (Fights _ _) _ = False
  effEq (Tap a) (Tap b) = nounEqRef a b
  effEq (Tap _) _ = False
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
  effEq (Sequentially _) _ = False
  effEq (Simultaneously _) _ = False
  effEq (Modal _ _) _ = False
  effEq (Delayed _ _) _ = False
  effEq (InsteadOf _ _) _ = False
  effEq (HeldUntil _ _) _ = False

  ||| No mode repeats another. [CR#700.2] calls a spell modal when its
  ||| bulleted options are "preceded by instructions for a player to
  ||| choose a number of those options", and two identical options are
  ||| one option written twice — the choice between them decides nothing.
  ||| [CR#700.2d] settles it from the other side: a player choosing more
  ||| than one mode "normally can't choose the same mode more than once",
  ||| and the cards that lift that restriction say so in words ("You may
  ||| choose the same mode more than once", six lines) rather than by
  ||| printing the mode twice. Structural, and conservative with it
  ||| (`effEq`): what it catches is the degenerate repetition
  ||| (`badDuplicateModes`), not every semantic twin.
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
  ||| [Sequentially [a, b], c]` re-mints the right-nested tree the
  ||| n-ary telescope was built to replace, and spells a three-sentence
  ||| card a second way (`badNestedSequence`) — the same
  ||| one-meaning-one-spelling refusal the singleton sequence gets, and
  ||| the one core reaches by flattening in `normalize` instead. What
  ||| may still hold a sequence is a clause slot that takes a BODY — a
  ||| "may" arm, a delayed clause, a tagged composite — where the
  ||| nesting is the card's own bracketing rather than a second
  ||| spelling of the list.
  public export
  data NotSeq : Effect bs -> Type where
    MkNotSeq : {auto 0 ok : isSeq e = False} -> NotSeq e

  ||| A simultaneous batch as an ANNOUNCEMENT telescope: each element is
  ||| typed in the bindings its predecessors ANNOUNCED (`annIntro`) and
  ||| not in what they did (`effIntro`, which is `Effects` beside it).
  ||| One line of difference between the two types, and it is the whole
  ||| semantic contrast between the two containers. The thread was
  ||| `preIntro` until chapter twenty-six, which is nearly the same list
  ||| and not the same function: a `May` element's optional DEED crossed
  ||| into its siblings through it (`badSimultaneousReadsMayDeed`).
  ||| Length-indexed, which is all `Simultaneously` needs to demand two.
  ||| Written with list syntax, so a batch reads as its macro writes it.
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
  nounIsAnyTarget (TheVerbed _ _) = False
  nounIsAnyTarget (ControllerOf _) = False
  nounIsAnyTarget (OwnerOf _) = False

  ||| Who can take damage ([CR#120.1,120.1a] — battles, creatures,
  ||| planeswalkers, players; never a quality, never an off-battlefield
  ||| card, never a noncreature artifact or land) — asked of the NOUN,
  ||| the way `DiscardOk` asks its verb's question. Three rows: any
  ||| player; the class word, which NAMES [CR#115.4]'s damage class and
  ||| so answers for itself without a zone or a head type; and every
  ||| other object phrase, which must stand on the battlefield under a
  ||| damageable head. The middle row is what lets the class word's
  ||| zone projection stay honest — "any target" places its referent
  ||| nowhere, so `nounZone` gives it none and the battlefield verbs
  ||| refuse it (`badDestroyAnyTarget`) — where a zone-FREE object row
  ||| would have bought the same refusal at the price of admitting bare
  ||| `This`, the source as an object, which takes no damage
  ||| (`badDamageThis`).
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

  ||| Which nouns take a card-type ascription — the closed table
  ||| `AsType` reads instead of carrying its scope in its row. One
  ||| entry: the SOURCE, whose sorted reading ("this creature", "this
  ||| artifact") is the only one the corpus writes. Every other noun
  ||| either says its own type in the predicate it carries ("target
  ||| creature" is a `HasType` phrase, not an ascribed one) or is a
  ||| read whose antecedent already fixed the head, and re-sorting a
  ||| read is the demonstrative's job ("that creature", `That`). The
  ||| table is what keeps `AsType`'s [CR#109.2] battlefield projection
  ||| honest: the rule speaks of a description carrying a card type and
  ||| NO zone word, which is exactly what the source under a type noun
  ||| is — so a second row has to argue for that projection again
  ||| before it can be written.
  public export
  data Ascribable : Noun bs Object -> Type where
    AscribeThis : Ascribable This

  ||| A keyword tag's legal expansion body ([CR#701.8a] family): the
  ||| tag and its move agree, so no term can pair a verb's deontic
  ||| identity with another verb's motion — and each tag demands its
  ||| verb's SOURCE zone of the moved noun ([CR#701.8a] destruction
  ||| moves a battlefield permanent, [CR#701.9a] discarding a hand
  ||| card; exile is zone-blind). The demand lives on the RELATION,
  ||| so raw spellings prove exactly what the macros prove — and a
  ||| forged provenance stamp is unwritable, only a legal tagged move
  ||| writing one (`badCompositeDestroyGraveyard`,
  ||| `badDoesDiscardBattlefield`).
  public export
  data TagBody : VerbName -> Effect bs -> Type where
    DestroyB : {auto 0 z : OnBattlefield (nounZone n)} ->
               TagBody Destroy (Move n (ZoneAt Graveyard Bare))
    SacrificeB : {auto 0 z : OnBattlefield (nounZone n)} ->
                 TagBody Sacrifice (Move n (ZoneAt Graveyard Bare))
    ExileB : TagBody Exile (Move n (ZoneAt Exile Bare))
    DiscardB : {auto 0 d : DiscardOk n} ->
               TagBody Discard (Move n (ZoneAt Graveyard Bare))

  ||| Verb agentivity, one table read by the rows it LACKS: an actor
  ||| is required exactly where there is no row here. The CR gives
  ||| sacrifice and discard a player actor ([CR#701.21a,701.9a]), so
  ||| their tags are absent and spell only under `Does`
  ||| (`badAgentlessSacrifice`). Destroy and exile have rows because
  ||| they are actor-OPTIONAL — the bare imperative and the subjected
  ||| form are both real oracle text ("You destroy four lands you
  ||| control, then target opponent destroys four lands they
  ||| control." — Burning of Xinye; "Each player exiles two cards from
  ||| their hand."), so `Does` demands nothing of the verb and only
  ||| `Composite` reads this table. A new verb declares its row or its
  ||| absence, and that choice IS the answer.
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
  setZone : Maybe VerbName -> Zone -> Binding -> Binding
  setZone p z (MkBinding det Object plur (ObjectP ty oldZn _)) =
    MkBinding det Object plur (ObjectP ty (Just z) (mkStamp p oldZn))
  setZone p z (MkBinding det Player plur PlayerP) = MkBinding det Player plur PlayerP
  setZone p z (MkBinding det (Quality q) plur QualityP) =
    MkBinding det (Quality q) plur QualityP
  setZone p z (MkBinding det Outcome plur (OutcomeP s)) =
    MkBinding det Outcome plur (OutcomeP s)

  public export
  setZoneHead : Maybe VerbName -> Zone -> Bindings -> Bindings
  setZoneHead p z [] = []
  setZoneHead p z (b :: bs) = setZone p z b :: bs

  public export
  setZoneIt : Maybe VerbName -> Zone -> Bindings -> Bindings
  setZoneIt p z [] = []
  setZoneIt p z (MkBinding det Object OneOf (ObjectP ty zn _) :: bs) =
    MkBinding det Object OneOf (ObjectP ty (Just z) (mkStamp p zn)) :: bs
  setZoneIt p z (b :: bs) = b :: setZoneIt p z bs

  public export
  setZoneThem : Maybe VerbName -> Zone -> Bindings -> Bindings
  setZoneThem p z [] = []
  setZoneThem p z (MkBinding det Object ManyOf (ObjectP ty zn _) :: bs) =
    MkBinding det Object ManyOf (ObjectP ty (Just z) (mkStamp p zn)) :: bs
  setZoneThem p z (b :: bs) = b :: setZoneThem p z bs

  public export
  setZoneThose : Maybe VerbName -> NounWord -> Zone -> Bindings -> Bindings
  setZoneThose p w z [] = []
  setZoneThose p w z (b :: bs) =
    case (b.plur, wordNow w b) of
      (ManyOf, True) => setZone p z b :: bs
      _ => b :: setZoneThose p w z bs

  public export
  setZoneThat : Maybe VerbName -> NounWord -> Zone -> Bindings -> Bindings
  setZoneThat p w z [] = []
  setZoneThat p w z (b :: bs) =
    case (b.plur, wordNow w b) of
      (OneOf, True) => setZone p z b :: bs
      _ => b :: setZoneThat p w z bs

  public export
  setZoneVerbed : Maybe VerbName -> VerbName -> NounWord -> Zone -> Bindings -> Bindings
  setZoneVerbed p v w z [] = []
  setZoneVerbed p v w z (b :: bs) =
    if verbedMatch v w b then setZone p z b :: bs else b :: setZoneVerbed p v w z bs

  public export
  moveIntro : {bs : Bindings} -> {k : Kind} -> Maybe VerbName -> Noun bs k -> Zone -> Bindings
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
  moveIntro p This z = bs
  -- a moved sorted self-reference mints the new object's binding
  -- ([CR#400.7]; see the constructor comment). The stamp's at-verb
  -- frame stays conservatively False — an ascribed-self cost participle TYPE
  -- word waits on a corpus witness, so the pre-move zone is passed as
  -- untracked here rather than read off `nounZone`.
  moveIntro p (AsType t n) z = MkBinding TheD Object OneOf (ObjectP (Just t) (Just z) (mkStamp p Nothing)) :: bs
  moveIntro p You z = bs
  moveIntro p They z = bs
  moveIntro p (ControllerOf n) z = nomIntro (ControllerOf n)
  moveIntro p (OwnerOf n) z = nomIntro (OwnerOf n)

  ||| The zone a noun's referent currently occupies, if tracked: reads
  ||| consult their unique binding, introducers their seed zone
  ||| ([CR#109.2] — a bare description means the battlefield), the
  ||| player nouns are untracked. The SORTED self-reference is a
  ||| description that includes a card type, so [CR#109.2] places it on
  ||| the battlefield exactly as it places "target creature" there;
  ||| bare `This` is the source as an object ("this spell", cycling's
  ||| "Discard this card") and stays untracked. The one counted
  ||| mention that takes no default is the class word: "any target" is
  ||| not a description of an object but the NAME of [CR#115.4]'s
  ||| damage class, which spans players, so [CR#109.2] has nothing to
  ||| place and the phrase projects no zone — which is how every
  ||| battlefield-demanding verb comes to refuse it through the
  ||| ordinary gate (`badDestroyAnyTarget`, `badTapAnyTarget`), damage
  ||| taking it by its own recipient row instead.
  public export
  nounZone : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe Zone
  nounZone This = Nothing
  nounZone (AsType t n) = Just Battlefield
  nounZone You = Nothing
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
  nounZone (Those w) = zoneOfThose w bs
  nounZone (TheVerbed v w) = zoneOfVerbed v w bs
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
  nounTy (Those w) = tyOfThose w bs
  nounTy (TheVerbed v w) = tyOfVerbed v w bs
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
  nounPlur (Those w) = ManyOf
  nounPlur (TheVerbed v w) = OneOf
  nounPlur (ControllerOf n) = OneOf
  nounPlur (OwnerOf n) = OneOf

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = outcomeB DamageDealt :: nomIntro to
  effIntro (Fights a b) = nomIntro b
  effIntro (Tap n) = nomIntro n
  effIntro (Choose n) = nomIntro n
  effIntro (Move what to) = moveIntro Nothing what (zoneSort to)
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
  -- together and not the count alone (`outputPlur`, chapter
  -- twenty-six) — the same derivation `Create` beside it uses, and for
  -- the same reason: [CR#701.17a] has each milled-at player put that
  -- many cards into their own graveyard, so "each player mills a card"
  -- puts one card per player into the graveyards and the mention is
  -- plural even though the count says one. Locke, Treasure Hunter reads
  -- it plural in the next breath ("each player mills a card. … you may
  -- cast a spell from among those cards"); the singular `It` there is
  -- refused now (`badDistributedMillSingular`).
  effIntro (Mill who amt) =
    MkBinding TheD Object (outputPlur (nounPlur who) (amtPlur amt))
              (ObjectP Nothing (Just Graveyard) Nothing)
      :: amtIntro amt
  -- the found object, in the zone it was found in ([CR#701.23a]) -- the
  -- "it"/"that card" every search sentence goes on to place.
  effIntro (Search who z p) =
    MkBinding AD Object OneOf (ObjectP (seedTy p) (Just (zoneSort z)) Nothing)
      :: (predDelta p ++ nomIntro who)
  effIntro (Shuffle whose) = shuffledAway (nomIntro whose)
  effIntro (Continuously se _) = staticIntro se
  -- the created token enters the discourse as the indefinite mention its
  -- phrase is ("a … token"), on the battlefield ([CR#111.1] — tokens are
  -- put there), under the head its type line writes, and with the number
  -- the CLAUSE writes — which is the count and the AGENT together, not
  -- the count alone. Additive Evolution reads the undistributed singular
  -- in the next breath ("create a 0/0 green and blue Fractal creature
  -- token. Put three +1/+1 counters on it."); Elephant Resurgence reads
  -- the distributed one ("Each player creates a green Elephant creature
  -- token. Those creatures have …"), where the count is still one and
  -- the mention is plural because the agent is (`outputPlur`).
  effIntro (Create agent count tok riders) =
    MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
              (ObjectP (tokenHeadTy tok) (Just Battlefield) Nothing)
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
  effIntro (Composite v (Move what to)) = moveIntro (Just v) what (zoneSort to)
  effIntro (Composite _ e) = effIntro e
  effIntro (Does s v (Move what to)) = moveIntro (Just v) what (zoneSort to)
  effIntro (Does s v e) = effIntro e
  effIntro (Pay who c) = costIntro c
  effIntro (May d body did notd) = mayIntro body did notd
  -- NEITHER arm contributes: only one of them ever runs. The else arm
  -- was always a hole (it REPLACES the main line, `mayIntro`'s cut one
  -- construction over) and the conditioned clause is now one too, for
  -- the identical reason read the other way — the condition may have
  -- been FALSE, and then the clause never happened and its phrase named
  -- nothing (`badConditionalArmAntecedent`). Chapter twenty-one's
  -- finding: this row used to export `effIntro e`, which made a
  -- conditionally created token an unconditional `It` for every
  -- sentence after it. What survives the branch is the discourse that
  -- entered it, plus the condition's own contribution (nothing, on
  -- every row — written in terms of `condDelta` rather than assumed).
  -- The COST this row was named as carrying is PAID, and not here: a
  -- target announced inside the conditioned clause is announced whatever
  -- the condition's truth ([CR#601.2c]), so Overload's second sentence
  -- really does read the first sentence's "that artifact" across an
  -- `If`. That channel is `preIntro`'s and it is open there now, which
  -- leaves this row saying only what it should have said all along —
  -- what a conditional DID is unknown. The half still owed is the
  -- LEADING conditional that announces a target its own consequent
  -- reads (Blood Lust; `badMatchesTargetSubject`), which needs a delta
  -- carrying a phrase's target half and nothing else (ledger).
  effIntro (If e c oth) = condDelta c ++ bs
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
  effIntro (InsteadOf replaced repl) = annIntro replaced
  -- the undo is scheduled on an event that has not happened, so what
  -- this clause leaves behind is not settled yet: the exiled object may
  -- be in exile or back on the battlefield when a later sentence reads
  -- it. The announcement is all that is safe ([CR#601.2c]).
  effIntro (HeldUntil e ev) = annIntro e

  ||| What a clause has ANNOUNCED by the time its own trailing condition
  ||| is read — the pre-resolution twin of `effIntro`, and the whole of
  ||| chapter twenty-one's answer to the divergence `If` has carried in
  ||| its comment since chapter eighteen: the condition is WRITTEN after
  ||| the clause and EVALUATED before it, so typing it in the clause's
  ||| post-state let it ask about a world the clause had not made yet.
  ||| "Destroy target creature if it's in a graveyard" typechecked
  ||| exactly because the destroy had already retagged its target
  ||| (`badTrailingPostStateZone`).
  |||
  ||| Two kinds of row differ from `effIntro`, and both are the same
  ||| distinction: what a clause's PHRASES name is announced as the
  ||| clause is written ([CR#601.2c] for targets, and the choice and
  ||| recipient phrases with them), so it is readable; what the clause
  ||| DOES is not. So the three zone-writing rows announce their object
  ||| without the retag or the verb stamp, `Create` announces no token
  ||| ([CR#111.1] — a token is put onto the battlefield by an EFFECT, so
  ||| the resolution the condition gates is what makes it), and the two event-outcome rows leave no outcome behind
  ||| (nothing has been dealt or gained yet, so "that much" reads
  ||| nothing). Every other row is its `effIntro` answer verbatim,
  ||| written out rather than delegated so that a new clause has to
  ||| declare its own pre-state.
  public export
  preIntro : {bs : Bindings} -> Effect bs -> Bindings
  preIntro (DealDamage src amt to) = nomIntro to
  preIntro (Distribute v amt among) = nomIntro among
  preIntro (Fights a b) = nomIntro b
  preIntro (Tap n) = nomIntro n
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
  -- THE ANNOUNCEMENT CHANNEL, chapter twenty-five's. This row used to
  -- be `condDelta c ++ bs` — `effIntro`'s answer copied — and the copy
  -- is what made Overload's second sentence unwritable. A conditioned
  -- clause is a hole for what it DID (finding 100: the condition may
  -- have been false, so it never ran) and that is `effIntro`'s row,
  -- unchanged. What it announced is another matter: [CR#601.2c] chooses
  -- targets as the spell is cast, "whatever clause spells them", and no
  -- condition is checked then — so "Destroy target artifact if its mana
  -- value is 2 or less" really has announced an artifact by the time the
  -- next sentence says "that artifact", true condition or false. The two
  -- functions are exactly the pre/post distinction chapter twenty-one
  -- drew, and this row is the first place where they had been made to
  -- agree by hand.
  -- The channel is `annIntro`'s now, chapter twenty-six: `preIntro` was
  -- never announcement-only, so copying it here exported a nested may's
  -- deed and a nested sequence's outcome along with the targets
  -- (`badConditionalInsteadReadsMayOutcome`). The flat case — which is
  -- the case the channel was opened for — is unchanged, a flat clause's
  -- announcement being the same list under either name.
  preIntro (If e c oth) = condDelta c ++ annIntro e
  preIntro (Sequentially es) = preIntros es
  preIntro (Simultaneously es) = simPres es
  preIntro (Modal q modes) = bs
  preIntro (Delayed ev e) = bs
  -- the announcement of the clause that was replaced — the replacement
  -- may have announced phrases of its own, but only one of the two ran
  -- and the one that certainly did not is the replaced clause's EVENT,
  -- not its phrases ([CR#614.6] against [CR#601.2c]).
  preIntro (InsteadOf replaced repl) = annIntro replaced
  preIntro (HeldUntil e ev) = annIntro e

  ||| THE ANNOUNCEMENT CHANNEL — chapter twenty-six's, and the whole of
  ||| this chapter's answer to a distinction chapters twenty-one through
  ||| twenty-five kept borrowing `preIntro` for. What a clause's PHRASES
  ||| have named by the time the spell is cast ([CR#601.2c] announces
  ||| every target as the spell is cast, whatever resolution later makes
  ||| of it) and nothing whatever about what the clause DID.
  |||
  ||| `preIntro` is not that function and never was. It is the broader
  ||| PRE-RESOLUTION summary ordinary sequential prose needs — the
  ||| twin of `effIntro` one moment earlier — and on a FLAT clause the
  ||| two agree exactly, which is why three constructions could type
  ||| their announcement-only slots on it and look right. They came
  ||| apart on composites, in three directions and all of them measured:
  ||| a `May`'s row is `mayIntro`, so the optional body's DEED crossed
  ||| into a simultaneous sibling that had not seen it happen
  ||| (`badSimultaneousReadsMayDeed`, `badSimultaneousReadsMayOutcome`);
  ||| a `Sequentially`'s row is its last clause's pre-state over the
  ||| DEED telescope, so an earlier step's outcome reached a replacement
  ||| for an event [CR#614.6] says never happened
  ||| (`badInsteadReadsReplacedSequenceOutcome`); and `If` forwarded
  ||| either of them recursively
  ||| (`badConditionalInsteadReadsMayOutcome`).
  |||
  ||| So the rows differ from `preIntro` in exactly the places a clause
  ||| CONTAINS another clause: `May` announces its BODY's phrases and
  ||| nothing its arms did, `If` its conditioned clause's, `InsteadOf`
  ||| and `HeldUntil` the clause they wrap. A `Sequentially` is a hole —
  ||| not a hedge, a consequence of the telescope: its elements are
  ||| typed over each other's `effIntro`, so no later element's
  ||| announcement can be lifted out without the deeds it was typed in.
  ||| A BATCH is not, because its telescope is this function's
  ||| (`annSims`), which is what makes the announcements accumulate
  ||| across a batch while the deeds do not.
  |||
  ||| Every other row is `preIntro`'s answer verbatim, written out
  ||| rather than delegated so that a new clause has to declare its own
  ||| announcement.
  public export
  annIntro : {bs : Bindings} -> Effect bs -> Bindings
  annIntro (DealDamage src amt to) = nomIntro to
  annIntro (Distribute v amt among) = nomIntro among
  annIntro (Fights a b) = nomIntro b
  annIntro (Tap n) = nomIntro n
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
  -- written inside an optional clause whether or not the player takes
  -- the offer, so the body's phrases are announced; what the body DID
  -- is `mayIntro`'s business and not this function's. The ARMS
  -- contribute nothing here, and the reason is structural rather than a
  -- ruling: an arm is typed over the body's `effIntro`, so its own
  -- announcement cannot be taken without the body's deeds coming with
  -- it. Under-reporting, and in the safe direction.
  annIntro (Pay who c) = nomIntro who
  annIntro (May d body did notd) = annIntro body
  annIntro (If e c oth) = condDelta c ++ annIntro e
  annIntro (Sequentially es) = bs
  annIntro (Simultaneously es) = annSims es
  annIntro (Modal q modes) = bs
  annIntro (Delayed ev e) = bs
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

  ||| What a clause contributes BEYOND its announcement — the deed half
  ||| of `effIntro`, written as a delta so a batch can fold every
  ||| element's in (`simIntro`). Chapter twenty-two folded only the LAST
  ||| element's `effIntro` and recorded the simplification ("the union
  ||| for every row the container reaches today"); the general case is
  ||| the union, and the case that shows it is two deeds of one kind —
  ||| "create a token" beside "create a token" leaves TWO tokens
  ||| ([CR#608.2f] processes a batch's actions simultaneously, so both
  ||| happened), and a following "it" must refuse as ambiguous rather
  ||| than silently pick the last (`badBatchTwoCreatesThenIt`,
  ||| `badBatchTwoOutcomesThenThatMuch`).
  |||
  ||| It is the DELTA and not the whole answer because the announcement
  ||| telescope already carries each element's phrases: `effIntro`'s
  ||| answer folded whole would count them twice, and a doubled mention
  ||| breaks the uniqueness gates every read makes. So each row states
  ||| what the deed adds that the phrase did not — an outcome, a created
  ||| token, a found or milled group — and nothing else.
  |||
  ||| Two kinds of row answer `[]` for a reason worth stating. A clause
  ||| that RETAGS (the three zone-writing rows) or one that REMOVES
  ||| (a shuffle) does not add a binding at all, so it has no delta;
  ||| chapter twenty-two named this and the corpus still agrees — no
  ||| line writes a batch whose element moves an object, and the reads
  ||| INSIDE such a batch are refused anyway
  ||| (`badSimultaneousReadsRetag`). And a CONTAINER answers `[]`
  ||| because what it leaves behind is not a delta over its own
  ||| announcement: a may's arms and a conditional's branches are
  ||| `mayIntro`'s cut, a sequence and a batch are refused as batch
  ||| elements outright (`NotSeq`, `NotSim`), and a replacement
  ||| contributes its announcement and no outcome by [CR#614.6].
  public export
  deedDelta : {bs : Bindings} -> Effect bs -> List Binding
  deedDelta (DealDamage src amt to) = [outcomeB DamageDealt]
  deedDelta (Distribute (DividedDamage _) amt among) = [outcomeB DamageDealt]
  deedDelta (Distribute (DistributedCounters _) amt among) = []
  deedDelta (Fights a b) = []
  deedDelta (Tap n) = []
  deedDelta (Choose n) = []
  deedDelta (Move what to) = []
  deedDelta (ChangeLife who (Up a)) = [outcomeB LifeGained]
  deedDelta (ChangeLife who (Down a)) = [outcomeB LifeLost]
  deedDelta (ChangeLife who (Set a)) = []
  deedDelta (Draw who amt) = []
  deedDelta (Expose v who what) = []
  deedDelta (Mill who amt) =
    [MkBinding TheD Object (outputPlur (nounPlur who) (amtPlur amt))
               (ObjectP Nothing (Just Graveyard) Nothing)]
  deedDelta (Search who z p) =
    [MkBinding AD Object OneOf (ObjectP (seedTy p) (Just (zoneSort z)) Nothing)]
  deedDelta (Shuffle whose) = []
  deedDelta (Continuously se _) = []
  deedDelta (Create agent count tok riders) =
    [MkBinding AD Object (outputPlur (nounPlur agent) (amtPlur count))
               (ObjectP (tokenHeadTy tok) (Just Battlefield) Nothing)]
  deedDelta (PutCounters amt kind on) = []
  deedDelta (RemoveCounters amt kind from) = []
  deedDelta (Composite v (Move what to)) = []
  deedDelta (Composite _ e) = deedDelta e
  deedDelta (Does s v (Move what to)) = []
  deedDelta (Does s v e) = deedDelta e
  deedDelta (Pay who c) = []
  deedDelta (May d body did notd) = []
  deedDelta (If e c oth) = []
  deedDelta (Sequentially es) = []
  deedDelta (Simultaneously es) = []
  deedDelta (Modal q modes) = []
  deedDelta (Delayed ev e) = []
  deedDelta (InsteadOf replaced repl) = []
  deedDelta (HeldUntil e ev) = []

  ||| A sequence's pre-state is its LAST clause's: every earlier clause
  ||| has resolved by the time the trailing condition is read.
  public export
  preIntros : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  preIntros [] = bs
  preIntros (e :: []) = preIntro e
  preIntros (e :: es) = preIntros es

  ||| What a may-clause leaves behind: the MAIN LINE's discourse — the
  ||| body's, or the if-you-do arm's when that arm is the only branch,
  ||| since it continues the body rather than replacing it.
  |||
  ||| The if-you-DON'T arm contributes nothing, and the principle is the
  ||| one English marks: the arm that continues the main line flows out,
  ||| the arm that REPLACES it is a hole. "If you don't" is written
  ||| precisely to mark the departure, and only one of the two ever
  ||| happens, so a mention inside it names nobody the sentences after
  ||| the may can read back — `predDelta (Or _) = []` one layer up. The
  ||| body's own mentions flow out even though the may may be declined,
  ||| which is the ruling this clause has carried since it was minted:
  ||| a declined may skips at runtime, not in scope, and Through the
  ||| Breach reads the body's creature in its very next sentence.
  |||
  ||| BOTH arms at once is the fourth row, and chapter twenty-one's: the
  ||| taken arm flows out only when there is no declined arm beside it.
  ||| Crovax the Cursed writes the pair — "you may sacrifice a creature.
  ||| If you do, put a +1/+1 counter on Crovax. If you don't, remove a
  ||| +1/+1 counter from Crovax." — and [CR#118.12] is why the join is
  ||| the BODY and not either arm: the branch records whether the player
  ||| chose to pay, "regardless of what events actually occurred", so
  ||| exactly one arm ran and the sentences after the may cannot know
  ||| which. Selecting the if-you-do arm's mentions there was reading one
  ||| branch as if it were both (`badBothArmsAntecedent`).
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

  public export
  mayIntro : {bs : Bindings} -> (body : Effect bs) ->
             Maybe (Effect (effIntro body)) -> Maybe (Effect bs) -> Bindings
  mayIntro body Nothing Nothing = effIntro body
  mayIntro body (Just did) Nothing = effIntro did
  mayIntro body Nothing (Just notd) = effIntro body
  mayIntro body (Just did) (Just notd) = effIntro body

  ||| What a whole sequence contributes: its last clause's discourse,
  ||| the telescope having threaded every predecessor's through.
  public export
  effsIntro : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  effsIntro [] = bs
  effsIntro (e :: es) = effsIntro es

  ||| What a whole BATCH contributes: EVERY element's deed over the
  ||| announcement telescope — the union, chapter twenty-six's
  ||| correction to chapter twenty-two's recorded simplification. It
  ||| used to be the LAST element's `effIntro`, which was the union for
  ||| every row the container reached then (the control grant introduces
  ||| its phrase and does nothing else) and is not the union in general:
  ||| two creates in one batch leave two tokens, and reading the deed
  ||| off the end made the second one the only one there was, so a
  ||| following "it" resolved where the sentence is ambiguous
  ||| ([CR#608.2f]: both actions are processed, so both happened).
  ||| The deeds go in as DELTAS (`deedDelta`) because the telescope has
  ||| already carried each element's announced phrases; folding whole
  ||| `effIntro`s would count those twice.
  ||| The empty case is unreachable through `Simultaneously`
  ||| (`AtLeastTwo`) and is written for totality.
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
  ||| restrictions and both are printed, five hundred twenty-three lines
  ||| for the sorcery form and five for the instant one — so both rows
  ||| stand though only the first has a bench witness. `Nothing` on the
  ||| slot is the unrestricted default, which is instant speed
  ||| ([CR#117.1b] — "a player may activate an activated ability any time
  ||| they have priority"); the settled Idris spec spells that default as an
  ||| `AsInstant` value instead, and the difference is only where the
  ||| absence is written.
  |||
  ||| What is NOT here is the turn-part window ("Activate only during your
  ||| upkeep", thirty-two lines; "during your turn", forty-one; "during
  ||| your turn, before attackers are declared", nineteen; about a hundred
  ||| twenty in all, core's `Timing::DuringTurn`/`DuringStep`). Its
  ||| vocabulary nearly exists — `TurnPart` and `Whose` are chapter
  ||| seventeen's — and the two lines that break it are exact: "Activate
  ||| only during any upkeep step" writes a possessor `Whose` has no word
  ||| for, neither yours nor a named player's. Measured and ledgered.
  public export
  -- spelling: ["Activate only as a sorcery", "Activate only as an instant"]
  -- (a full sentence after the effect, [CR#602.1b] putting activation
  -- instructions last; conjoined with a sibling restriction by "and only"),
  -- kind: Sentence
  data Timing = AsSorcery | AsInstant

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
  ||| container a card's printed lines are made of. It carries the
  ||| activated row now and is shaped to GROW the others: the triggered
  ||| row ([CR#113.3c,603]) and the static row ([CR#113.3d,604]) are the
  ||| next round's, the spell row ([CR#113.3a]) belongs with the card
  ||| container, and each arrives as a row here rather than as a type of
  ||| its own — which is what core and the settled Idris spec both do
  ||| (`Ability::{Static, Activated, Triggered, Spell, Keyword}`).
  |||
  ||| The keyword row was this type's whole content before, and keeping it
  ||| is the point: what a `Gains` clause grants and what a card prints
  ||| are one category ([CR#113.3] lists them together), so the container
  ||| is grown rather than parallelled. It is UNINDEXED, which is the
  ||| claim that an ability line is context-closed — its cost is the first
  ||| thing on the line and reads no discourse before it. Core is
  ||| unindexed too; the spec indexes its `Ability` so a keyword
  ||| DESUGARING can reference an anaphor, and this file has no keyword
  ||| desugaring.
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
    -- The three restriction slots are separate for a corpus reason, not
    -- a core-imitating one: oracle CONJOINS them ("Activate only during
    -- your upkeep and only once each turn"; "Activate only if you control
    -- ten or more permanents and only as a sorcery"), so one restriction
    -- row would have had to spell a conjunction of unlike things.
    -- The GUARD is typed at the empty context and not in the cost's
    -- survivors, which is [CR#602.5]'s own placement: a restriction on
    -- use is checked before the ability is activated at all, where the
    -- cost is not paid until [CR#601.2h], so nothing the cost names can
    -- be read by the condition that decides whether the cost may be paid.
    -- It reuses `Condition` verbatim, chapter eighteen's seam ([CR#603.4]
    -- keeps the ordinary "if" and the trigger's intervening-"if" apart by
    -- CARRIER, and this is a third carrier of the same three questions).
    -- spelling: ["<Param(0)>: <Param(1)>"] plus one sentence per written
    -- restriction, appended after the effect in the order
    -- window/limit/guard and joined by "and only" when more than one is
    -- written ([CR#602.1b]: activation instructions "appear last, after
    -- the ability's effect"), kind: Ability
    Activated : (cost : Cost []) ->
                (eff : Effect (publicOnly (costIntro cost))) ->
                {default Nothing window : Maybe Timing} ->
                {default Nothing limit : Maybe UsageLimit} ->
                {default Nothing guard : Maybe (Condition [])} -> Ability

  ||| May this ability be GRANTED by a clause? Only the keyword. English
  ||| grants an activated ability by QUOTING it — "Enchanted land has
  ||| \"{T}: Add {B}\"" (twenty-five lines), "Equipped creature has
  ||| \"{T}: …\"" (ten), "All Slivers have \"{T}: …\"" (seven), and the
  ||| token with-clause form (seven) — which is a construction this
  ||| grammar has no quotation for, where "gains flying" is a bare
  ||| keyword. Growing the container is what made the refusal necessary:
  ||| the row exists now, so the grant site has to say no to it
  ||| (`badGainsActivated`).
  public export
  grantableAb : Ability -> Bool
  grantableAb (KeywordAbility _) = True
  grantableAb (Activated _ _) = False

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
