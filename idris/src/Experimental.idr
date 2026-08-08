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
data Determiner = TargetD | AD | EachD | AllD | TheD

||| Zone sorts, minimally ([CR#400.1] family) — the fold-state tag a
||| binding carries. Ownership is not stored here; it lives in the
||| surface `ZoneExpr` where English writes it.
public export
data Zone = Battlefield | Graveyard | Exile | Hand

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

||| Currently on the battlefield, strictly — the typed noun's
||| demonstrative demand ([CR#110.1]); untracked does not qualify.
public export
onFieldZone : Maybe Zone -> Bool
onFieldZone Nothing = False
onFieldZone (Just Battlefield) = True
onFieldZone (Just Graveyard) = False
onFieldZone (Just Exile) = False
onFieldZone (Just Hand) = False

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

public export
-- spelling: (construction-owned -- pass-through; the word is entirely
-- KeywordAbility's Keyword argument's own, see Keyword)
data Ability = KeywordAbility Keyword

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

||| Durations, as the trailing adverbial writes them ([CR#611.2a] — a
||| resolution-generated continuous effect "lasts as long as stated";
||| with no stated duration it lasts until end of game, which is the
||| explicit `Nothing` spelling per the no-defaults convention).
||| "For as long as" durations ([CR#611.2b]) are the row this type will
||| grow next and cannot grow yet, which the conditions chapter
||| measured rather than assumed: core spells it
||| `Duration::ForAsLongAs(Condition)` (`continuous.rs`), the condition
||| type it needs now exists here, and the row would EXTEND this one
||| (a third alternative beside `ThisTurn` and `Until`) rather than
||| parallel it. What is missing is a clause to attach it to. Two
||| hundred and ten corpus lines write the adverbial; of those, forty-two
||| are control grants and the rest reach for vocabulary this grammar
||| lacks — four keyword grants, all of "indestructible"; four stat
||| deltas, all ending "for as long as this artifact remains tapped";
||| nine restrictions, eight of them coordinated deeds or deeds with no
||| clause here. The single line that is one deed under one condition
||| ("Up to one target creature can't block for as long as you control
||| this Saga", There and Back Again) needs a type word `CardType` does
||| not have. So the row waits on its clauses, not on its shape, and
||| `spanUse` would have to be answered for a whole family rather than
||| a cell when it lands. The event-ended family ("until [this
||| creature] leaves the battlefield", the O-Ring shape) waits on the
||| events axis in the same way (ledger).
|||
||| "This turn" is its own row rather than an `Until` form because it is
||| not one: it names the current turn as a whole, without a boundary
||| word, and it is the adverbial the one-shot restrictions write where
||| the grants write "until end of turn" — the same span, a different
||| construction's word (`spanUse`).
public export
-- spelling: ["this turn", "until <Param(0)>"] (row order: ThisTurn/Until;
-- Until's own word is the bare "until" and the endpoint supplies the rest --
-- see DurationEnd), kind: TODO(reason: trailing-adverbial fragment -- not
-- one of Nominal/Sentence/Cost/KeywordLine/Ability)
data Duration = ThisTurn | Until DurationEnd

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
||| exactly, and only the cross-turn span is written by everything.
public export
data SpanUse = Unattested | Unclaimed | BothGrants | KeywordGrantOnly
             | RestrictionsOnly | EveryStatic

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
spanUse : Duration -> SpanUse
spanUse ThisTurn = RestrictionsOnly
-- "until [poss] next [part]" — the start boundary writes no boundary
-- word, and takes possession or the definite article, never nothing.
spanUse (Until (StartOf Turn Nothing)) = Unattested
spanUse (Until (StartOf Turn (Just Yours))) = EveryStatic
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
spanUse (Until (EndOf Turn Nothing)) = BothGrants
spanUse (Until (EndOf Turn (Just Yours))) = Unclaimed
spanUse (Until (EndOf Turn (Just ThatPlayers))) = Unattested
spanUse (Until (EndOf Upkeep Nothing)) = Unattested
spanUse (Until (EndOf Upkeep (Just Yours))) = Unclaimed
spanUse (Until (EndOf Upkeep (Just ThatPlayers))) = Unattested
spanUse (Until (EndOf EndStep Nothing)) = Unattested
spanUse (Until (EndOf EndStep (Just Yours))) = Unattested
spanUse (Until (EndOf EndStep (Just ThatPlayers))) = Unattested
spanUse (Until (EndOf Combat Nothing)) = BothGrants
spanUse (Until (EndOf Combat (Just Yours))) = Unclaimed
spanUse (Until (EndOf Combat (Just ThatPlayers))) = Unattested
spanUse (Until (EndOf UntapStep Nothing)) = Unattested
spanUse (Until (EndOf UntapStep (Just Yours))) = Unattested
spanUse (Until (EndOf UntapStep (Just ThatPlayers))) = Unattested

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
public export
data StaticKind = PtDelta | KeywordGrant | DeedRestriction

||| The attestation table's other half: which classes of adverbial each
||| construction writes. Full rows in both directions. The shape of it
||| is the finding — `EveryStatic` is the only class every row admits,
||| and the two current-turn words divide the grants from the
||| restrictions with nothing shared, which is why the detain family's
||| cross-turn span ("Up to one target creature can't attack or block
||| until your next turn") is the one place a restriction and a grant
||| write the same words.
public export
admitsSpan : StaticKind -> SpanUse -> Bool
admitsSpan PtDelta Unattested = False
admitsSpan PtDelta Unclaimed = False
admitsSpan PtDelta BothGrants = True
admitsSpan PtDelta KeywordGrantOnly = False
admitsSpan PtDelta RestrictionsOnly = False
admitsSpan PtDelta EveryStatic = True
admitsSpan KeywordGrant Unattested = False
admitsSpan KeywordGrant Unclaimed = False
admitsSpan KeywordGrant BothGrants = True
admitsSpan KeywordGrant KeywordGrantOnly = True
admitsSpan KeywordGrant RestrictionsOnly = False
admitsSpan KeywordGrant EveryStatic = True
admitsSpan DeedRestriction Unattested = False
admitsSpan DeedRestriction Unclaimed = False
admitsSpan DeedRestriction BothGrants = False
admitsSpan DeedRestriction KeywordGrantOnly = False
admitsSpan DeedRestriction RestrictionsOnly = True
admitsSpan DeedRestriction EveryStatic = True

||| Whether a construction can write NO duration at all. A grant can:
||| the unwritten span is [CR#611.2a]'s end-of-game default, which the
||| guide permits where the effect is "intentionally indefinite under
||| the rules" (Through the Breach's bare "It gains haste."). A
||| restriction cannot — a durationless "can't" is the STATIC ability
||| line ("Enchanted creature can't attack", Pacifism), a different
||| construction and the parked ability layer's, so the whole clause is
||| unwritable here rather than the span being optional
||| (`badStaticCant`).
public export
absentOk : StaticKind -> Bool
absentOk PtDelta = True
absentOk KeywordGrant = True
absentOk DeedRestriction = False

||| The `Continuously` clause's duration slot as a witness, reading both
||| tables: the stated absence against `absentOk`, a written adverbial
||| against the construction's own row in `spanUse`.
public export
data SpanOk : StaticKind -> Maybe Duration -> Type where
  SpanUnstated : {auto 0 ok : absentOk k = True} -> SpanOk k Nothing
  SpanStated : {auto 0 ok : admitsSpan k (spanUse d) = True} -> SpanOk k (Just d)

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
||| phrase that names it. The library will take its row when the
||| library lands; the shared zones can never take one, and a new
||| shared zone declares its absence by having no row to write.
public export
data Possessable : Zone -> Type where
  HandIsOwned : Possessable Hand
  GraveyardIsOwned : Possessable Graveyard

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

  ||| The sort a zone expression names — what fold-state records. The
  ||| scope is no part of it: whose hand a card is in does not change
  ||| that it is in a hand.
  public export
  zoneSort : ZoneExpr bs -> Zone
  zoneSort (ZoneAt z _) = z

  ||| An object/player criteria set — the noun phrase's modifier list,
  ||| FLAT: head noun and relative clauses are sibling constraints on one
  ||| referent, exactly as parsed (no rearrangement to figure out).
  public export
  data Predicate : Bindings -> Kind -> Type where
    -- spelling: ["<Param(0)>"] (Param(0) = CardType's own word -- see
    -- CardType), kind: Nominal (hasHead = True)
    HasType : CardType -> Predicate bs Object            -- head noun "creature"/…
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
  ||| (`badDoubleOther`). The cap is written FIRST so the conjunction
  ||| reduces for a phrase whose CONTEXT is abstract — `anyOtherTarget`
  ||| carries its anchor presupposition as a hypothesis, and `x && True`
  ||| would stay stuck on the neutral `x`.
  public export
  otherAnchorOk : {bs : Bindings} -> (k : Kind) -> List CardType ->
                  List (Predicate bs k) -> Bool
  otherAnchorOk k ts ps = atMostOne (countOthers (flattenPs ps)) &&
                          (if hasOtherAny ps then anchorFound k ts bs else True)

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
  isOther _ = False

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
  negatable AnyTarget = False

  public export
  data Negatable : Predicate bs k -> Type where
    MkNegatable : {auto 0 ok : negatable p = True} -> Negatable p

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
  anyTargetFree AnyTarget = False

  public export
  anyTargetFreeAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Bool
  anyTargetFreeAll [] = True
  anyTargetFreeAll (p :: ps) = anyTargetFree p && anyTargetFreeAll ps

  public export
  data AnyTargetFree : Predicate bs k -> Type where
    MkAnyTargetFree : {auto 0 ok : anyTargetFree p = True} -> AnyTargetFree p

  ||| May a counted target mention spell the class word? The answer is
  ||| the QUANTITY's, not the determiner's. At exactly one the phrase
  ||| IS the singular damage-class form [CR#115.4] defines — "Lightning
  ||| Bolt deals 3 damage to any target" — so the class word is exactly
  ||| what belongs there, and an up-to mention takes it at any bound
  ||| because the corpus writes that outright ("Fall of the Titans
  ||| deals X damage to each of up to two targets"). What is refused is
  ||| the EXACT group from two up and the unbounded "any number of":
  ||| [CR#115.4] names those plural forms in the same breath, but the
  ||| corpus writes them with structures the bare group mention does
  ||| not spell — a division ("divided as you choose among one or two
  ||| targets") or an each-of recipient — so the ban stands with that
  ||| ledgered axis (`badGroupAnyTarget`, `badAnyNumberAnyTarget`).
  ||| What the two permitting quantities license is the class word as
  ||| the phrase's HEAD, never the class word wherever it turns up: a
  ||| possessor buried in a relative clause spells "any target" under a
  ||| counted mention exactly as loudly as under "a"
  ||| (`badEmbeddedAnyTargetExact1`), so the embedded scan runs beneath
  ||| every quantity. The head case needs no scan of its own —
  ||| `AnyTargetLone` allows the class word no companion but "other",
  ||| and neither word carries a noun to bury one in.
  public export
  anyTargetOkAt : {0 bs : Bindings} -> {0 k : Kind} ->
                  Quantity -> Predicate bs k -> Bool
  anyTargetOkAt (Range (Just (S Z)) (Just (S Z))) p = headIsAnyTarget p ||
                                                      anyTargetFree p
  anyTargetOkAt (Range Nothing (Just _)) p = headIsAnyTarget p || anyTargetFree p
  anyTargetOkAt (Range _ _) p = anyTargetFree p

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

  ||| Destination legality for the move primitive ([CR#400.3] — cards
  ||| enter only their owner's hand/library/graveyard, so an owned
  ||| destination naming an arbitrary player is unwritable, and the
  ||| bare zone IS the owner-rooted destination; the possessive is
  ||| rendering's business, finding 34).
  public export
  data DestOk : ZoneExpr bs -> Type where
    BattlefieldOk : DestOk (ZoneAt Battlefield Bare)
    ExileOk : DestOk (ZoneAt Exile Bare)
    HandOkBare : DestOk (ZoneAt Hand Bare)
    GraveyardOkBare : DestOk (ZoneAt Graveyard Bare)

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
  predDelta _ = []

  public export
  predDeltaAll : {bs : Bindings} -> {k : Kind} -> List (Predicate bs k) -> List Binding
  predDeltaAll [] = []
  predDeltaAll (p :: ps) = predDelta p ++ predDeltaAll ps

  public export
  zoneDelta : {bs : Bindings} -> ZoneExpr bs -> List Binding
  zoneDelta (ZoneAt z (OwnedBy n)) = nounDelta n
  zoneDelta (ZoneAt z Bare) = []

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

  ||| Two bounds, compared as written. Conservative in `predEq`'s
  ||| direction and for its reason: the catch-all reads "not provably
  ||| the same value", and the constructor's own gate means the only
  ||| bounds that ever reach here are the two rows above.
  public export
  boundEq : {0 bs : Bindings} -> Amount bs -> Amount bs -> Bool
  boundEq (Lit a) (Lit b) = a == b
  boundEq XVal XVal = True
  boundEq _ _ = False

  ||| Life-total change operands ([CR#119.3]; `Set` is a later chapter).
  public export
  data LifeOp : Bindings -> Type where
    -- spelling: (construction-owned -- selects ChangeLife's verb "gains",
    -- not independently spelled; mirrors constructors.ron's `GainLife`
    -- entry, body ChangeLife(Param(0), Up(Param(1))))
    Up : Amount bs -> LifeOp bs     -- "gains [amt] life"
    -- spelling: (construction-owned -- selects ChangeLife's verb "loses";
    -- the `LosesLife`/Down counterpart per the GainLife comment above)
    Down : Amount bs -> LifeOp bs   -- "loses [amt] life"

  public export
  lifeIntro : {bs : Bindings} -> LifeOp bs -> Bindings
  lifeIntro (Up a) = amtIntro a
  lifeIntro (Down a) = amtIntro a

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
    -- discipline.
    -- spelling: ["<Param(0)> gains <Param(1)>"] (the trailing duration
    -- adverbial belongs to the Continuously envelope, not here),
    -- kind: Sentence
    Gains : (n : Noun bs Object) -> Ability ->
            {auto 0 ok : OnBattlefield (nounZone n)} -> StaticEffect bs
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

  ||| Which row a static effect is, for the span tables.
  public export
  staticKind : {0 bs : Bindings} -> StaticEffect bs -> StaticKind
  staticKind (Gets _ _ _) = PtDelta
  staticKind (Gains _ _) = KeywordGrant
  staticKind (Cant _ _ _) = DeedRestriction

  ||| What a continuous clause contributes to the discourse: its
  ||| subject, exactly as the one-shot clauses contribute theirs.
  public export
  staticIntro : {bs : Bindings} -> StaticEffect bs -> Bindings
  staticIntro (Gets n _ _) = nomIntro n
  staticIntro (Gains n _) = nomIntro n
  staticIntro (Cant n _ _) = nomIntro n

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
    Choose : {k : Kind} -> Noun bs k -> Effect bs
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
    Move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) ->
           {auto 0 ok : DestOk to} -> Effect bs
    -- "[who] gains/loses [amt] life" ([CR#119.3]) — core basis (merged).
    -- spelling: ["<Param(0)> gains <Param(1)> life", "<Param(0)> loses
    -- <Param(1)> life"] (selects on the embedded LifeOp, Up/Down; mirrors
    -- constructors.ron's `GainLife` entry / the merged ChangeLife family),
    -- kind: Sentence
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Effect bs
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
    Continuously : (se : StaticEffect bs) -> (span : Maybe Duration) ->
                   {auto 0 sp : SpanOk (staticKind se) span} -> Effect bs
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
    -- have served. The MANDATORY twin ("[Do something]. If you do, …",
    -- with no "may") is the same rule's first shape, a hundred and
    -- forty-two lines writing no "may" anywhere; it waits with the cost
    -- algebra that has to spell an unoffered payment. So does the ELSE
    -- sentence "Otherwise, …", which is `If`'s branch and not this
    -- one's.
    -- spelling: ["<Param(0)> may <Param(1)>", "<Param(0)> may <Param(1)>.
    -- If <Param(0)> do, <Param(2)>.", "<Param(0)> may <Param(1)>. If
    -- <Param(0)> don't, <Param(3)>."] (the anaphor's pronoun and its verb
    -- agreement are the DECIDER's own -- "if you do" against Risk Factor's
    -- "if they don't" -- so auto-inflection supplies both from Param(0);
    -- mirrors core's May struct field-for-field), kind: Sentence
    May : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          (ifDid : Maybe (Effect (effIntro body))) ->
          (ifNot : Maybe (Effect (nomIntro decider))) -> Effect bs
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
    -- A DIVERGENCE worth stating: the condition is evaluated before
    -- the clause it modifies takes effect, but it is WRITTEN after it,
    -- so the telescope types it against a discourse the clause has
    -- already updated — Overload's artifact is retagged to the
    -- graveyard by the destroy before "its mana value" is read here.
    -- Nothing in this vocabulary notices (mana value belongs to every
    -- object, [CR#202.3], and is read zone-free), but a zone-sensitive
    -- read in a trailing condition would, and it is the first thing to
    -- check when one lands.
    -- The condition contributes nothing (`condDelta`), so the clause's
    -- own contribution is unchanged by conditioning it — written in
    -- terms of `condDelta` rather than assuming it. The ELSE arm
    -- ("Otherwise, …", a hundred and seventy-five lines) is a third
    -- slot this node will take and does not have yet: every corpus
    -- else-arm read this pass needs vocabulary this grammar lacks
    -- (ledger).
    -- spelling: ["<Param(0)> if <Param(1)>", "If <Param(1)>, <Param(0)>"]
    -- (the leading order is available only when the condition reads nothing
    -- the clause introduced -- a linearization side condition, unchecked
    -- here, like the leading/trailing choice on Delayed), kind: Sentence
    If : (e : Effect bs) -> (c : Condition (effIntro e)) -> Effect bs
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
    -- "[e] [when/at event-query]" — the temporal adverbial stays on
    -- its clause (leading vs trailing position is linearization); the
    -- body reads the discourse as settled particulars transformed by
    -- the event (`delayedCtx`, [CR#603.7c,603.3d]).
    -- spelling: ["<Param(1)> <Param(0)>"] (trailing adverbial position;
    -- leading position swaps the order, linearization's choice -- see
    -- EventQuery), kind: Sentence
    Delayed : (ev : EventQuery bs) -> Effect (delayedCtx ev) -> Effect bs

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
  effIntro (Continuously se _) = staticIntro se
  effIntro (Composite v (Move what to)) = moveIntro (Just v) what (zoneSort to)
  effIntro (Composite _ e) = effIntro e
  effIntro (Does s v (Move what to)) = moveIntro (Just v) what (zoneSort to)
  effIntro (Does s v e) = effIntro e
  effIntro (May d body did notd) = mayIntro body did
  effIntro (If e c) = condDelta c ++ effIntro e
  effIntro (Sequentially es) = effsIntro es
  effIntro (Delayed ev e) = bs               -- a future clause mentions nothing NOW

  ||| What a may-clause leaves behind: the MAIN LINE's discourse — the
  ||| body's, or the if-you-do arm's when there is one, since that arm
  ||| continues the body rather than replacing it.
  |||
  ||| The if-you-DON'T arm contributes nothing, and the principle is the
  ||| one English marks: the arm that continues the main line flows out,
  ||| the arm that REPLACES it is a hole. "If you don't" is written
  ||| precisely to mark the departure, and only one of the two ever
  ||| happens, so a mention inside it names nobody the sentences after
  ||| the may can read back — `predDelta (Or _) = []` one layer up. The
  ||| body's own mentions flow out even though the may may be declined,
  ||| which is the ruling this clause has carried since it was minted:
  ||| a declined may skips at runtime, not in scope.
  public export
  mayIntro : {bs : Bindings} -> (body : Effect bs) ->
             Maybe (Effect (effIntro body)) -> Bindings
  mayIntro body Nothing = effIntro body
  mayIntro body (Just did) = effIntro did

  ||| What a whole sequence contributes: its last clause's discourse,
  ||| the telescope having threaded every predecessor's through.
  public export
  effsIntro : {bs : Bindings} -> {0 n : Nat} -> Effects n bs -> Bindings
  effsIntro [] = bs
  effsIntro (e :: es) = effsIntro es

-- ===== The activated-ability juncture =====

||| "[cost]: [effect]" ([CR#602.1]) — just the colon: the cost's
||| object-moving/tapping component as an ordinary clause, the effect
||| typed in the cost's public-zone survivors (`publicOnly`). Mana,
||| {T}, and activation instructions are elided the way positives elide
||| rider lines; the full ability layer stays parked.
public export
-- spelling: ["<Param(0)>: <Param(1)>"], kind: Ability (the activated-ability
-- line shape; TODO(reason: not directly confirmed against a real
-- Activated-shaped catalog entry among the artifacts studied this pass))
data Activated : Bindings -> Type where
  MkActivated : (cost : Effect bs) -> Effect (publicOnly (effIntro cost)) -> Activated bs
