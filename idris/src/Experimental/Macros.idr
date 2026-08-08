||| The macro layer: one definition per English phrase shape.
||| Experimental's analogue of the Macros-over-Semantics split.
module Experimental.Macros

import public Experimental

%default total

-- The named quantities, macros over the one `Range` primitive exactly
-- as core's are (`plugins/builtin/macros/quantity/`): "[n]" for the
-- exact count, "up to [n]" for core's `AtMost` under the oracle's own
-- word, "any number of" for the unbounded range. Core's `AtLeast` and
-- `Between` ("one or two targets") spell over the same primitive; they
-- wait on a corpus line that needs them.

-- "[n] target [pred]s" — the exact count
-- spelling: (construction-owned -- quantity wording; verified real family:
-- crates/deckmaste_english/src/constructions/quantity.rs's "quantity_exact"
-- combinator (name+semantics match: exact count, singular iff n=1))
public export
exactly : Nat -> Quantity
exactly n = Range (Just n) (Just n)

-- "up to [n] target [pred]s"
-- spelling: (construction-owned -- quantity wording; matches
-- "quantity_up_to" in constructions/quantity.rs, verified by exact name)
public export
upTo : Nat -> Quantity
upTo n = Range Nothing (Just n)

-- "any number of target [pred]s"
-- spelling: (construction-owned -- quantity wording; TODO(reason: no
-- confirmed combinator name for the unbounded "any number of" form among
-- quantity.rs's registered names within this pass's scope))
public export
anyNumber : Quantity
anyNumber = Range Nothing Nothing

-- "target [pred]" — the singular counted mention. One constructor
-- serves every quantity ([CR#601.2c] announces them all alike); at
-- exactly one the numeral is what rendering leaves unwritten ("target
-- creature", never "one target creature").
-- spelling: [(text: "target <Param(0)>", when: [(quantity, "Exactly(1)")])],
-- kind: Nominal (this macro IS the Exactly(1) branch of constructors.ron's
-- own `Target` entry -- verified line-for-line against that file)
-- The class-word obligation travels as a hypothesis: at exactly one the
-- quantity permits "any target" as the phrase's HEAD but still refuses
-- one buried in a possessor, which an abstract predicate cannot answer
-- here — so the macro demands what `TargetGroup` demands and each call
-- site discharges it.
public export
target : (p : Predicate bs k) -> {auto tk : Targetable k} ->
         {auto 0 hd : Headed p} ->
         {auto 0 af : AnyTargetAtCount (exactly 1) p} -> Noun bs k
target p = TargetGroup (exactly 1) p {tk} {hd} {af}

-- The zone phrases, one macro per surface English writes over the two
-- axes `ZoneAt` carries: the zone word bare, and the zone word under a
-- possessive. Core names a zone the same bare way wherever it names one
-- (`StatePredicate::InZone(Zone)`, `Destination::Zone(Zone)`) and keeps
-- ownership beside it as its own relation (`RelationPredicate::Owner`,
-- `filter.rs`); these six are that pair multiplied out into the phrases
-- the corpus actually writes.

-- "the battlefield" — shared ([CR#400.1]), so it takes no possessive.
-- spelling: ["the battlefield"], kind: Nominal
public export
battlefieldZ : ZoneExpr bs
battlefieldZ = ZoneAt Battlefield Bare

-- "exile" — shared likewise.
-- spelling: ["exile"], kind: Nominal
public export
exileZ : ZoneExpr bs
exileZ = ZoneAt Exile Bare

-- "hand" — the sort-only form: a per-player zone written with no
-- owner, which is what a macro expansion needs when the card text
-- named none (see `ZoneScope`).
-- spelling: ["hand"], kind: Nominal (bare sort-only form -- macro
-- expansions whose English wrote no owner)
public export
handZ : ZoneExpr bs
handZ = ZoneAt Hand Bare

-- "graveyard" — the sort-only form, as `handZ`.
-- spelling: ["graveyard"], kind: Nominal (bare sort-only form, see handZ)
public export
graveyardZ : ZoneExpr bs
graveyardZ = ZoneAt Graveyard Bare

-- "[player]'s hand" — the owned form. The possessor is singular, and
-- the demand travels to the caller as everywhere else.
-- spelling: ["<Param(0)>'s hand"], kind: Nominal (e.g. "your hand",
-- "its owner's hand")
public export
handOf : (n : Noun bs Player) -> {auto 0 one : nounPlur n = OneOf} -> ZoneExpr bs
handOf n = ZoneAt Hand (OwnedBy n {ps = HandIsOwned} {one})

-- "[player]'s graveyard" — the owned form, as `handOf`.
-- spelling: ["<Param(0)>'s graveyard"], kind: Nominal (see handOf)
public export
graveyardOf : (n : Noun bs Player) -> {auto 0 one : nounPlur n = OneOf} -> ZoneExpr bs
graveyardOf n = ZoneAt Graveyard (OwnedBy n {ps = GraveyardIsOwned} {one})

-- The sorted self-reference, one macro per type word the corpus
-- writes it with: the source ascribed a card type ([CR#109.2] then
-- reading the PERMANENT), which is `AsType` over `This` and nothing
-- more. Bare `This` remains its own word — "this spell", cycling's
-- "Discard this card" ([CR#702.29a]).

-- "this creature"
-- spelling: ["this creature"], kind: Nominal
public export
thisCreature : Noun bs Object
thisCreature = AsType Creature This

-- "this artifact"
-- spelling: ["this artifact"], kind: Nominal
public export
thisArtifact : Noun bs Object
thisArtifact = AsType Artifact This

-- "this enchantment"
-- spelling: ["this enchantment"], kind: Nominal
public export
thisEnchantment : Noun bs Object
thisEnchantment = AsType Enchantment This

-- "this land"
-- spelling: ["this land"], kind: Nominal
public export
thisLand : Noun bs Object
thisLand = AsType Land This

-- "creature"
-- spelling: ["creature"], kind: Nominal (hasHead = True; HasType Creature)
public export
creature : Predicate bs Object
creature = HasType Creature

-- "artifact" — `creature`'s siblings, one per type word the
-- coordinated phrases spell ("artifact or enchantment", "artifact,
-- creature, or land").
-- spelling: ["artifact"], kind: Nominal (hasHead = True; HasType Artifact)
public export
artifact : Predicate bs Object
artifact = HasType Artifact

-- "enchantment"
-- spelling: ["enchantment"], kind: Nominal (hasHead = True;
-- HasType Enchantment)
public export
enchantment : Predicate bs Object
enchantment = HasType Enchantment

-- "land"
-- spelling: ["land"], kind: Nominal (hasHead = True; HasType Land)
public export
land : Predicate bs Object
land = HasType Land

-- "creature you control"
-- spelling: ["creature you control"], kind: TODO(reason: head noun +
-- non-head modifier conjunction, per hasHead/And -- see Predicate)
public export
creatureYouControl : Predicate bs Object
creatureYouControl = And [creature, ControlledBy You]

-- "creature you don't control"
-- spelling: ["creature you don't control"], kind: TODO(reason: see
-- creatureYouControl; "don't" is Not's construction-owned transform)
public export
creatureYouDontControl : Predicate bs Object
creatureYouDontControl = And [creature, Not (ControlledBy You)]

-- The indefinite, one macro per choice-mode marking the corpus writes:
-- the article is the same determiner throughout (`Indefinite`), and
-- what differs is the adverbial after the noun. Core keeps the axes
-- apart the same way — `Binder::ChooseOne`'s `by` slot names the
-- chooser over one filter, and the chooserless form is
-- `Selection::Random` (`binder.rs`, `selection.rs`).

-- "a [pred]" — the unmarked indefinite: the rules supply the chooser
-- the text does not name ([CR#608.2d,400.7]).
-- spelling: ["a <Param(0)>"], kind: Nominal (auto-inflects to "an" before
-- a vowel sound)
public export
a : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
    {auto 0 hd : Headed p} ->
    {auto 0 af : AnyTargetFree p} -> Noun bs k
a p = Indefinite Unmarked p {ph} {hd} {af}

-- "a [pred] of their choice" — the chooser marked by the possessive
-- pronoun, which is why the macro carries the antecedent obligation to
-- its caller (`badUnboundTheirChoice`).
-- spelling: ["a <Param(0)> of their choice"], kind: Nominal
public export
aTheirChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
               {auto 0 ch : countChoosers bs = 1} ->
               {auto 0 hd : Headed p} ->
               {auto 0 af : AnyTargetFree p} -> Noun bs k
aTheirChoice p = Indefinite (TheirChoice {ch}) p {ph} {hd} {af}

-- "a [pred] at random" — the markedly chooserless variant
-- ([CR#701.9b]; Pyromancy).
-- spelling: ["a <Param(0)> at random"], kind: Nominal
public export
aAtRandom : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
            {auto 0 hd : Headed p} ->
            {auto 0 af : AnyTargetFree p} -> Noun bs k
aAtRandom p = Indefinite AtRandom p {ph} {hd} {af}

-- "an opponent"
-- spelling: ["an opponent"], kind: Nominal (the unmarked indefinite over
-- Opponent; "a" auto-inflects to "an" before a vowel)
public export
anOpponent : Noun bs Player
anOpponent = a Opponent

-- "any other target" — the macro CARRIES its phrase's presupposition
-- (an earlier target) in its type.
-- spelling: ["any other target"], kind: Nominal (And [AnyTarget, Other] --
-- the sole corpus companion Arc Trail spells, finding 40)
public export
anyOtherTarget : {auto 0 ok : anyTargeted Object bs = True} -> Predicate bs Object
anyOtherTarget = And [AnyTarget, Other]

-- The stat reads, macros over the one `StatOf` primitive as core's are
-- over `Count::StatOf(Reference, Stat)` (`count.rs`): one axis, one
-- constructor, and the three phrases English writes for it. Each keeps
-- the singular agreement its constructor demands — one object's own
-- numbers ([CR#208.1,202.3]; `badGroupPower`) — by threading the proof
-- through to the caller.

-- "[its/…] power" ([CR#208.1])
-- spelling: ["<Param(0)>'s power"], kind: TODO(reason: amount fragment --
-- not one of Nominal/Sentence/Cost/KeywordLine/Ability; see Amount.Lit)
public export
powerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
powerOf n = StatOf Power n {one}

-- "[its/…] toughness" ([CR#208.1])
-- spelling: ["<Param(0)>'s toughness"], kind: TODO(reason: amount
-- fragment, see powerOf)
public export
toughnessOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
toughnessOf n = StatOf Toughness n {one}

-- "[its/…] mana value" ([CR#202.3])
-- spelling: ["<Param(0)>'s mana value"], kind: TODO(reason: amount
-- fragment, see powerOf)
public export
manaValueOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
manaValueOf n = StatOf ManaValue n {one}

-- The for-each amount, as the two operations core keeps apart: count
-- the described set, then scale it by the written per-unit
-- (`Count::CountOf` and `Count::Times`, `count.rs`). The adverbial
-- surface is the macro's; the demands are the primitives' — a
-- noun-headed domain (`Headed`) and an any-target-free one
-- (`AnyTargetFree`) from the count, a written per-unit of at least one
-- (`AtLeastOne`) from the scaling — and each travels to the call site
-- as a hypothesis, since an abstract predicate cannot discharge it
-- here (the `target` pattern).

-- "[n] [unit] for each [pred]" — the counted-set amount with its
-- per-unit numeral written ("loses 1 life for each attacking creature
-- you control", Foul-Tongue Shriek). The unit word itself ("life",
-- "card") comes from the embedding verb, not from here.
-- spelling: ["<Param(0)> for each <Param(1)>"], kind: TODO(reason:
-- amount fragment, see powerOf)
public export
nForEach : {k : Kind} -> (n : Nat) -> (p : Predicate bs k) ->
           {auto 0 hd : Headed p} -> {auto 0 nz : AtLeastOne n} ->
           {auto 0 af : AnyTargetFree p} -> Amount bs
nForEach n p = Times n (CountOf p {hd} {af}) {nz}

-- "[a/one] [unit] for each [pred]" — the common per-unit, one.
-- spelling: ["<Param(0)> for each <Param(1)>"] (at one the numeral is
-- what an article may spell instead -- "draw a card for each opponent
-- who lost life this turn"), kind: TODO(reason: amount fragment, see
-- powerOf)
public export
forEach : {k : Kind} -> (p : Predicate bs k) ->
          {auto 0 hd : Headed p} ->
          {auto 0 af : AnyTargetFree p} -> Amount bs
forEach p = nForEach 1 p {hd} {af}

-- "destroy [n]" — mirrors plugins/builtin/macros/action/Destroy.ron:
-- the Destroy tag over the battlefield→graveyard move [CR#701.8a];
-- only a battlefield permanent is destroyable (`badDestroyGraveyard`).
-- spelling: ["destroy <Param(0)>"], kind: Sentence (verified line-for-line
-- against action/Destroy.ron: template "destroy ${0}", params: [Reference],
-- body Composite(name: Destroy, body: Move(Param(0), Graveyard)) -- this
-- macro IS that def)
public export
destroy : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
destroy n = Composite Destroy (Move n graveyardZ) {ok = DestroyB {z = ok}}

-- "exile [n]" ([CR#701.13a]) — speculative pending its real macro.
-- spelling: ["exile <Param(0)>"], kind: Sentence (speculative -- no
-- action/Exile.ron artifact studied this pass; drafted by direct analogy
-- to Destroy.ron's shape)
public export
exile : Noun bs Object -> Effect bs
exile n = Composite Exile (Move n exileZ) {ok = ExileB}

-- "[agent] sacrifice(s) [n]" ([CR#701.21a]) — one macro per lemma: the
-- imperative spells `You` explicitly, inflection is the frame's. The
-- performer is the sacrificed permanent's controller [CR#701.21a]; no
-- CR rule names a sacrifice chooser, so "of their choice" is surface
-- marking (`aTheirChoice`'s mode), not derivation. The zone half of the
-- implicit restriction is demanded (`OnBattlefield`); the controller
-- half needs fold-state the context does not carry (not-settled).
-- Core's `Sacrifice` variant is the whittling candidate this expands.
-- spelling: ["<Param(0)> sacrifice(s) <Param(1)>"], kind: Sentence (core
-- whittling candidate, no real macro studied this pass; construction
-- mirrors Does's subject+tag+Move shape)
public export
sacrifice : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
            {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
sacrifice agent n = Does agent Sacrifice (Move n graveyardZ) {tb = SacrificeB {z = ok}}

-- "[agent] discard(s) [n]" — the hand→graveyard move [CR#701.9a] with
-- the subject in clause position. The CR routes by the card's OWNER;
-- owner≡agent is this macro's elision, the same one the real macro
-- makes with its agent-param hand filter. The noun carries its own
-- choice marking: the plain indefinite is the affected player's
-- choice by default [CR#701.9b], "at random" the markedly chooserless
-- variant (`aAtRandom` — Pyromancy).
-- spelling: ["<Param(0)> discard(s) <Param(1)>"], kind: Sentence (no real
-- action/Discard.ron artifact studied this pass; mirrors Does's
-- subject+tag+Move shape, same as sacrifice)
public export
discards : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
           {auto 0 dk : DiscardOk n} -> Effect bs
discards agent n = Does agent Discard (Move n graveyardZ) {tb = DiscardB {d = dk}}

-- "[agent] discard(s) a card" — the common phrase, spelled sort-only
-- (an owned-hand expansion needs a subject-read noun the vocabulary
-- lacks — not-settled).
-- spelling: ["<Param(0)> discard(s) a card"], kind: Sentence (sort-only
-- expansion of `discards` at `a (InZone handZ)`; not an independent frame)
public export
discardsACard : (agent : Noun bs Player) -> Effect bs
discardsACard agent = discards agent (a (InZone handZ))

-- The duration adverbials, one macro per phrase the corpus writes, over
-- the decomposed endpoint. The structure carries the axes (boundary,
-- part, possession); these names carry the WORDS, and which of them a
-- construction may write is the span tables' answer (`spanUse`), not
-- the caller's.

-- "this turn" — the restrictions' current-turn adverbial.
-- spelling: (construction-owned -- pass-through to Duration.ThisTurn)
public export
thisTurn : Duration
thisTurn = ThisTurn

-- "until end of turn" — the grants' current-turn adverbial; bare, no
-- article ([CR#514.2] sweeps it in cleanup).
-- spelling: (construction-owned -- the bare end-of-turn endpoint, see
-- DurationEnd for why the boundary word and the article are its own)
public export
untilEndOfTurn : Duration
untilEndOfTurn = Until (EndOf Turn Nothing)

-- "until your next turn" — the cross-turn span, the one every
-- construction writes (the detain family's included).
-- spelling: (construction-owned -- the possessed start-of-turn endpoint)
public export
untilYourNextTurn : Duration
untilYourNextTurn = Until (StartOf Turn (Just Yours))

-- "until end of combat" ([CR#511.2]) — Glyph of Destruction.
-- spelling: (construction-owned -- the bare end-of-combat endpoint)
public export
untilEndOfCombat : Duration
untilEndOfCombat = Until (EndOf Combat Nothing)

-- "until your next upkeep" ([CR#503]) — Gabriel Angelfire; the one
-- endpoint the keyword grant writes alone.
-- spelling: (construction-owned -- the possessed start-of-upkeep endpoint)
public export
untilYourNextUpkeep : Duration
untilYourNextUpkeep = Until (StartOf Upkeep (Just Yours))

-- "[n] gets [+p/+t] [duration]" — the stat change and its adverbial,
-- the envelope's two halves under one name. Each grant macro threads
-- the clause's demands to its caller: a battlefield subject, and a span
-- its own construction writes.
-- spelling: ["<Param(0)> gets <Param(1)>/<Param(2)>"] (optional trailing
-- duration), kind: Sentence (Continuously (Gets …) d -- see
-- StaticEffect.Gets)
public export
gets : (n : Noun bs Object) -> (pow : Integer) -> (tou : Integer) ->
       (d : Maybe Duration) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
       {auto 0 sp : SpanOk PtDelta d} -> Effect bs
gets n pow tou d = Continuously (Gets n pow tou) d

-- "[n] gains [ability] [duration]"
-- spelling: ["<Param(0)> gains <Param(1)>"] (optional trailing duration),
-- kind: Sentence (Continuously (Gains …) d -- see StaticEffect.Gains)
public export
gains : (n : Noun bs Object) -> (a : Ability) -> (d : Maybe Duration) ->
        {auto 0 ok : OnBattlefield (nounZone n)} ->
        {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gains n a d = Continuously (Gains n a) d

-- "[n] gains haste [duration]"
-- spelling: ["<Param(0)> gains haste"] (optional trailing duration), kind:
-- Sentence (gains n (KeywordAbility Haste) d -- see Keyword,
-- StaticEffect.Gains)
public export
gainsHaste : (n : Noun bs Object) -> (d : Maybe Duration) ->
             {auto 0 ok : OnBattlefield (nounZone n)} ->
             {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gainsHaste n d = gains n (KeywordAbility Haste) d

-- The one-shot restrictions, one macro per verb phrase English
-- writes: the deed word inflected by the voice its subject's part
-- calls for. Core marks the same distinction by which slot carries the
-- reference (`DeonticAction::Block { by, on }`, `deontic.rs`); here it
-- is the `Role` argument, and these three names are the whole attested
-- surface. Each threads the clause's demands — battlefield subject,
-- the deed's grant in that voice, the restriction's own adverbial — to
-- its caller. The span slot is `Maybe` like the grants', so that the
-- durationless "can't" is REFUSED for its reason rather than by its
-- shape: `absentOk DeedRestriction = False` says a "can't" with no
-- adverbial is the static ability line (`badStaticCant`).

-- "[n] can't attack [duration]" ([CR#508.1c]) — Change of Heart.
-- spelling: ["<Param(0)> can't attack <Param(1)>"], kind: Sentence
public export
cantAttack : (n : Noun bs Object) -> (span : Maybe Duration) ->
             {auto 0 zn : OnBattlefield (nounZone n)} ->
             {auto 0 dp : DeedParticipant Attack Agent (nounTy n)} ->
             {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantAttack n span = Continuously (Cant n Attack Agent {zn} {dp}) span {sp}

-- "[n] can't block [duration]" ([CR#509.1b]) — Blindblast, Blinding
-- Flare.
-- spelling: ["<Param(0)> can't block <Param(1)>"], kind: Sentence
public export
cantBlock : (n : Noun bs Object) -> (span : Maybe Duration) ->
            {auto 0 zn : OnBattlefield (nounZone n)} ->
            {auto 0 dp : DeedParticipant Block Agent (nounTy n)} ->
            {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBlock n span = Continuously (Cant n Block Agent {zn} {dp}) span {sp}

-- "[n] can't be blocked [duration]" ([CR#509.1b]) — Infiltrate; the
-- passive of the same deed, the subject standing in core's `on` slot.
-- spelling: ["<Param(0)> can't be blocked <Param(1)>"], kind: Sentence
public export
cantBeBlocked : (n : Noun bs Object) -> (span : Maybe Duration) ->
                {auto 0 zn : OnBattlefield (nounZone n)} ->
                {auto 0 dp : DeedParticipant Block Patient (nounTy n)} ->
                {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBeBlocked n span = Continuously (Cant n Block Patient {zn} {dp}) span {sp}

-- "[who] loses [amt] life"
-- spelling: ["<Param(0)> loses <Param(1)> life"], kind: Sentence
-- (ChangeLife who (Down amt) -- the `LosesLife`/Down sibling of
-- constructors.ron's `GainLife` entry)
public export
losesLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
losesLife who amt = ChangeLife who (Down amt)

-- "[who] gains [amt] life"
-- spelling: ["<Param(0)> gains <Param(1)> life"], kind: Sentence
-- (ChangeLife who (Up amt) -- verified line-for-line against
-- constructors.ron's `GainLife` entry exactly)
public export
gainsLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
gainsLife who amt = ChangeLife who (Up amt)

-- The three may-clause surfaces, macros over the one `May` primitive
-- exactly as core's branchless and branching mays are one node
-- (`deckmaste_core/src/effect.rs`). What varies is which anaphoric
-- sentence follows, and the branch fields are where it goes; the
-- pronoun and its agreement are the decider's own.

-- "[decider] may [effect]" — the branchless offer.
-- spelling: ["<Param(0)> may <Param(1)>"], kind: Sentence
public export
may : (decider : Noun bs Player) -> Effect (nomIntro decider) -> Effect bs
may d body = May d body Nothing Nothing

-- "[decider] may [effect]. If [decider] do, [effect]." — the taken
-- branch, which reads everything the body introduced.
-- spelling: ["<Param(0)> may <Param(1)>. If <Param(0)> do, <Param(2)>."],
-- kind: Sentence (the anaphor's pronoun is Param(0)'s -- "if you do",
-- "if they do")
public export
mayThen : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (effIntro body) -> Effect bs
mayThen d body did = May d body (Just did) Nothing

-- "[decider] may [effect]. If [decider] don't, [effect]." — the
-- declined branch, which reads only what preceded the may.
-- spelling: ["<Param(0)> may <Param(1)>. If <Param(0)> don't,
-- <Param(2)>."], kind: Sentence (see mayThen)
public export
mayElse : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (nomIntro decider) -> Effect bs
mayElse d body notd = May d body Nothing (Just notd)

-- The token characteristics, as the two shapes the corpus writes them in:
-- the plain creature token, which is the overwhelming majority, and the
-- record itself for everything that adds a slot (a compound type line, a
-- with-clause, a name). `TokenChars` is the primitive; these carry the
-- words.

-- "[p]/[t] [colors] [subtypes] creature token" — the common bundle
-- ([CR#111.3]). The empty color list is the word "colorless"; the
-- abilities and name slots are the ones this shape leaves unwritten.
-- spelling: (construction-owned -- the adjective run in oracle's fixed
-- order, see TokenChars), kind: Nominal
public export
creatureTok : (pow : Nat) -> (tou : Nat) -> List Color -> List Subtype -> TokenChars
creatureTok pow tou cs ss = MkToken (Just (pow, tou)) cs (MkTypeLine ss [Creature]) [] Nothing

-- "[subtypes]" — the type line a subtype-only addition writes ("becomes a
-- Zombie in addition to its other types", [CR#701.47a]).
-- spelling: (construction-owned -- see TypeLine)
public export
subtypesOnly : List Subtype -> TypeLine
subtypesOnly ss = MkTypeLine ss []

-- "[types]" — the type line a card-type-only addition writes ("becomes an
-- artifact in addition to its other types", Neurok Transmuter).
-- spelling: (construction-owned -- see TypeLine)
public export
typesOnly : List CardType -> TypeLine
typesOnly ts = MkTypeLine [] ts

-- "Create [count] [chars] token(s)" — the imperative, whose unpronounced
-- subject is `You` spelled explicitly (the `sacrifice`/`discards` pattern),
-- and with no arrival rider. The token's own demands travel to the caller
-- as hypotheses, since an abstract bundle cannot discharge them here.
-- spelling: ["create <Param(0)> <Param(1)> token(s)"], kind: Sentence
public export
create : (count : Amount bs) -> (tok : TokenChars) ->
         {auto 0 tt : TokenTyped tok} ->
         {auto 0 tp : TokenPt tok} ->
         {auto 0 sf : SubtypesFit tok} -> Effect bs
create count tok = Create You count tok [] {tt} {tp} {sf} {rr = MkRidersOk}

-- "Create [count] [chars] token(s) that's tapped and attacking"
-- ([CR#508.4]) — the arrival pair, which is the only shape the corpus
-- writes attacking in (sixty-six create-token lines; attacking without
-- tapped, none).
-- spelling: ["create <Param(0)> <Param(1)> token(s) that's tapped and
-- attacking"], kind: Sentence
public export
createTappedAttacking : (count : Amount bs) -> (tok : TokenChars) ->
                        {auto 0 tt : TokenTyped tok} ->
                        {auto 0 tp : TokenPt tok} ->
                        {auto 0 sf : SubtypesFit tok} -> Effect bs
createTappedAttacking count tok =
  Create You count tok [EntersTapped, EntersAttacking] {tt} {tp} {sf} {rr = MkRidersOk}

-- "[n] becomes [type line] in addition to its other types [duration]"
-- ([CR#205.1b]) — the static effect and its adverbial under one name, the
-- `gets`/`gains` shape for the type change.
-- spelling: ["<Param(0)> becomes <Param(1)> in addition to its other types"]
-- (optional trailing duration), kind: Sentence (Continuously (BecomesAlso …) d
-- -- see StaticEffect.BecomesAlso)
public export
becomes : (n : Noun bs Object) -> (added : TypeLine) -> (d : Maybe Duration) ->
          {auto 0 zn : OnBattlefield (nounZone n)} ->
          {auto 0 ne : LineNonEmpty added} ->
          {auto 0 af : AddedFits (nounTy n) added} ->
          {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomes n added d = Continuously (BecomesAlso n added {zn} {ne} {af}) d {sp}

-- "it's [pred]" — the reference-matches condition over the singular
-- object read, which is the subject every corpus line writes it with
-- ("if it's a creature card", "if it's attacking").
-- spelling: ["it's <Param(0)>"], kind: TODO(reason: condition fragment --
-- not one of Nominal/Sentence/Cost/KeywordLine/Ability)
public export
itsA : (p : Predicate bs Object) -> {auto 0 ok : countOnes Object bs = 1} ->
       {auto 0 af : AnyTargetFree p} -> Condition bs
itsA p = Matches (It {ok}) p {af}
