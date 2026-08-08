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

-- "an opponent"
-- spelling: ["an opponent"], kind: Nominal (A Opponent; "a" auto-inflects
-- to "an" before a vowel)
public export
anOpponent : Noun bs Player
anOpponent = A Opponent

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
destroy n = Composite Destroy (Move n GraveyardZ) {ok = DestroyB {z = ok}}

-- "exile [n]" ([CR#701.13a]) — speculative pending its real macro.
-- spelling: ["exile <Param(0)>"], kind: Sentence (speculative -- no
-- action/Exile.ron artifact studied this pass; drafted by direct analogy
-- to Destroy.ron's shape)
public export
exile : Noun bs Object -> Effect bs
exile n = Composite Exile (Move n ExileZ) {ok = ExileB}

-- "[agent] sacrifice(s) [n]" ([CR#701.21a]) — one macro per lemma: the
-- imperative spells `You` explicitly, inflection is the frame's. The
-- performer is the sacrificed permanent's controller [CR#701.21a]; no
-- CR rule names a sacrifice chooser, so "of their choice" is surface
-- marking (`ATheirChoice`), not derivation. The zone half of the
-- implicit restriction is demanded (`OnBattlefield`); the controller
-- half needs fold-state the context does not carry (not-settled).
-- Core's `Sacrifice` variant is the whittling candidate this expands.
-- spelling: ["<Param(0)> sacrifice(s) <Param(1)>"], kind: Sentence (core
-- whittling candidate, no real macro studied this pass; construction
-- mirrors Does's subject+tag+Move shape)
public export
sacrifice : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
            {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
sacrifice agent n = Does agent Sacrifice (Move n GraveyardZ) {tb = SacrificeB {z = ok}}

-- "[agent] discard(s) [n]" — the hand→graveyard move [CR#701.9a] with
-- the subject in clause position. The CR routes by the card's OWNER;
-- owner≡agent is this macro's elision, the same one the real macro
-- makes with its agent-param hand filter. The noun carries its own
-- choice marking: the plain indefinite is the affected player's
-- choice by default [CR#701.9b], "at random" the markedly chooserless
-- variant (`AAtRandom` — Pyromancy).
-- spelling: ["<Param(0)> discard(s) <Param(1)>"], kind: Sentence (no real
-- action/Discard.ron artifact studied this pass; mirrors Does's
-- subject+tag+Move shape, same as sacrifice)
public export
discards : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
           {auto 0 dk : DiscardOk n} -> Effect bs
discards agent n = Does agent Discard (Move n GraveyardZ) {tb = DiscardB {d = dk}}

-- "[agent] discard(s) a card" — the common phrase, spelled sort-only
-- (an owned-hand expansion needs a subject-read noun the vocabulary
-- lacks — not-settled).
-- spelling: ["<Param(0)> discard(s) a card"], kind: Sentence (sort-only
-- expansion of `discards` at A (InZone HandZ); not an independent frame)
public export
discardsACard : (agent : Noun bs Player) -> Effect bs
discardsACard agent = discards agent (A (InZone HandZ))

-- "[n] gains haste [duration]" — the duration slot is the grant's own
-- adverbial, so the macro carries `GrantSpan` through to its caller.
-- spelling: ["<Param(0)> gains haste"] (optional trailing duration), kind:
-- Sentence (Gain n (KeywordAbility Haste) d -- see Keyword, Effect.Gain)
public export
gainsHaste : (n : Noun bs Object) -> (d : Maybe Duration) ->
             {auto 0 ok : OnBattlefield (nounZone n)} ->
             {auto 0 sp : GrantSpan d} -> Effect bs
gainsHaste n d = Gain n (KeywordAbility Haste) d

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
