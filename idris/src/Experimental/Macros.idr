||| The macro layer: one definition per English phrase shape.
||| Experimental's analogue of the Macros-over-Semantics split.
module Experimental.Macros

import public Experimental

%default total

-- The named quantities, macros over the one `Range` primitive exactly
-- as core's are (`plugins/builtin/macros/quantity/`): "[n]" for the
-- exact count, "up to [n]" for core's `AtMost` under the oracle's own
-- word, "any number of" for the unbounded range. Core's `AtLeast` and
-- `Between` waited on a corpus line that needed them and the MODAL
-- headcount is it: "one or more" is `atLeast` and "one or both" is the
-- one-to-two `Between`, nineteen and fifty-two cards. What still waits
-- is the TARGET-position spelling of either ("one or two target
-- creatures"), which no bench card writes.

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

-- "[n] or more" — core's `AtLeast` under the oracle's own word, which
-- the modal headcount is what finally needed: "Choose one or more —"
-- heads nineteen cards, over three, four, or five modes, and it is the
-- form that appears exactly where "one or both" cannot, the list running
-- past two.
-- spelling: ["<Param(0)> or more"] (matches core's `AtLeast` over the same
-- Range primitive)
public export
atLeast : Nat -> Quantity
atLeast n = Range (Just n) Nothing

-- "one or both" — the range from one to two under the word that names
-- the whole of a two-item list. Fifty-two modal cards write it and every
-- one of them offers EXACTLY two modes, which is what "both" says; the
-- same range over three modes would have to write "one or two", and the
-- corpus writes that zero times, all scopes. That agreement is a
-- linearization side condition and not a gate — the numeric relation is
-- `ModesFit`'s, and which WORDS a top-of-the-list range spells is the
-- renderer's, "or both" at two and "or more" above.
-- spelling: ["one or both"] (core's `Between(1, 2)` under the two-item
-- word; see `atLeast` for the unbounded sibling)
public export
oneOrBoth : Quantity
oneOrBoth = Range (Just 1) (Just 2)

-- "one or [n] target [pred]s" / "one, two, or [n] target [pred]s" — the
-- ENUMERATED range from one, and the DIVISION's own way of writing a
-- target count ([CR#601.2d] has the caster announce the division, and
-- the phrase spells the choice of how many to divide among). The corpus
-- writes exactly two widths: "one or two targets" (twelve lines, Forked
-- Bolt and Chandra's Pyrohelix) and "one, two, or three targets" (nine,
-- Arc Lightning), plus their creature-restricted twins ("among one or
-- two target creatures", ten; "among one, two, or three target …",
-- twenty-six). It is the same range `oneOrBoth` spells at two, under the
-- word the target count uses rather than the word a two-item mode list
-- uses — which is why they are two macros over one primitive and not one
-- macro with two names.
-- spelling: ["one, … or <Param(0)>"] (core's `Between(1, n)`; the
-- enumeration is the renderer's, "or both" being the modal list's word
-- and "one or two" the target count's)
public export
oneThrough : Nat -> Quantity
oneThrough n = Range (Just 1) (Just n)

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

-- "the stack" as a zone phrase — never a destination ([CR#405.1] puts a
-- cast card there by the casting itself, and `DestOk` has no row), only
-- the zone a description names.
-- spelling: (construction-owned -- the zone word is UNSPELLED wherever a
-- description carries it, the carrier noun "spell" standing for the
-- whole phrase [CR#112.1])
public export
stackZ : ZoneExpr bs
stackZ = ZoneAt Stack Bare

-- "[a] spell" — the stack's carrier noun ([CR#112.1]: "a spell is a card
-- on the stack"), which is why the phrase is a ZONE clause and not a
-- head word of its own: the same object is a card everywhere else and a
-- spell only here, exactly as `InZone graveyardZ` writes "card in a
-- graveyard".
-- spelling: ["spell"], kind: Nominal (InZone stackZ, whose zone word is
-- unspelled -- see Zone.Stack)
public export
spell : Predicate bs Object
spell = InZone stackZ

-- A whole CARD, with the container's four demands threaded to the call
-- site — the record itself carries no gates, records having no room for
-- one, so this is where a card term proves itself well formed.
-- spelling: (construction-owned -- pass-through to Card's own printed
-- layout; see Card)
public export
card : (name : String) -> (cost : Maybe ManaCost) -> (supers : List Supertype) ->
       (line : TypeLine) -> (text : List Ability) ->
       (stats : Maybe (Integer, Integer)) ->
       {auto 0 ln : CardLine line} ->
       {auto 0 sp : CardSupers supers} ->
       {auto 0 tx : CardText line text} ->
       {auto 0 pts : CardPt line stats} ->
       {auto 0 mc : CardCost line cost} ->
       Card
card name cost supers line text stats = MkCard name cost supers line text stats

-- "counter [n]" ([CR#701.6a])
-- spelling: ["counter <Param(0)>"], kind: Sentence (CounterSpell -- see
-- Effect.CounterSpell)
public export
counterSpell : (n : Noun bs Object) ->
               {auto 0 zn : OnStack (nounZone n)} -> Effect bs
counterSpell n = CounterSpell n {zn}

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
handOf : (n : Noun bs Player) -> {auto 0 pn : Possessor n} -> ZoneExpr bs
handOf n = ZoneAt Hand (OwnedBy n {ps = HandIsOwned} {pn})

-- "[player]'s graveyard" — the owned form, as `handOf`.
-- spelling: ["<Param(0)>'s graveyard"], kind: Nominal (see handOf)
public export
graveyardOf : (n : Noun bs Player) -> {auto 0 pn : Possessor n} -> ZoneExpr bs
graveyardOf n = ZoneAt Graveyard (OwnedBy n {ps = GraveyardIsOwned} {pn})

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

-- "exiled with this artifact" — the LINKAGE read at its commonest
-- source word ([CR#406.6,607.2a]); the creature and enchantment sources
-- are the same predicate under `thisCreature`/`thisEnchantment`.
-- spelling: ["exiled with this artifact"], kind: Nominal (hasHead = True;
-- ExiledWith thisArtifact -- see Predicate.ExiledWith)
public export
exiledWithThisArtifact : Predicate bs Object
exiledWithThisArtifact = ExiledWith thisArtifact

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

-- "instant"
-- spelling: ["instant"], kind: Nominal (hasHead = True; HasType Instant)
public export
instant : Predicate bs Object
instant = HasType Instant

-- "sorcery"
-- spelling: ["sorcery"], kind: Nominal (hasHead = True; HasType Sorcery)
public export
sorcery : Predicate bs Object
sorcery = HasType Sorcery

-- "instant and sorcery" / "instant or sorcery" — the corpus's most-written
-- type union (119 lines under the plural head, 464 more under a singular
-- determiner), and ONE term under both words: which coordinator is written
-- is `Or`'s environment-derived spelling and not a fact about this phrase.
-- spelling: (construction-owned -- see Predicate.Or's coordinator rule)
public export
instantOrSorcery : Predicate bs Object
instantOrSorcery = Or [instant, sorcery]

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

-- "creature your opponents control" -- the UNION read, and a third
-- spelling beside the two above rather than a transform of either: the
-- corpus writes it 170 times with the creature head, and only one card
-- writes it beside "you don't control", so they are distinct phrases and
-- not free variants (294 lines for the negated self).
-- spelling: ["creature your opponents control"], kind: TODO(reason: see
-- creatureYouControl)
public export
creatureYourOpponentsControl : Predicate bs Object
creatureYourOpponentsControl = And [creature, ControlledBy (PlayerGroup YourOpponents)]

||| "tapped" — [CR#110.5]'s tap value as the ordinary prenominal word.
public export
tapped : Predicate bs Object
tapped = HasStatus Tapped

||| "untapped" — the paired value's own word, not a negation.
public export
untapped : Predicate bs Object
untapped = HasStatus Untapped

||| "face-down" — [CR#110.5]'s face value as the ordinary prenominal
||| word, hyphenated where the tap pair is not. The description reader of
||| the same value `ToFace` changes and `IsTurnedFace` observes.
public export
faceDown : Predicate bs Object
faceDown = HasStatus FaceDown

||| "nontoken" — the description-side negation of the token head.
public export
nontoken : Predicate bs Object
nontoken = Not IsToken

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

-- The ANCHORED complement's surfaces — one word, "other", over the
-- referent the clause names, and one macro per attested head. Each
-- threads the anchor's own obligation (a read, never a mention) to its
-- caller. The determiner decides what the complement DOES: under `Each`
-- or `AllOf` it is the group complement ("each other creature", "other
-- creatures you control"), under a counted target it is the singular
-- selector spelled "another" ("this creature fights another target
-- creature", Brash Taunter).

-- "other creature [than n]" — the anchor unpronounced.
-- spelling: ["other creature"] (register variant "another creature"),
-- kind: Nominal (And [creature, OtherThan n] -- see Predicate.OtherThan)
public export
otherCreature : (n : Noun bs Object) -> {auto 0 ca : ComplementAnchor n} ->
                {auto 0 ty : anchorTyFits [Creature] (nounTy n) = True} ->
                Predicate bs Object
otherCreature n = And [creature, OtherThan n {ca}]

-- "other creature(s) you control [than n]" — the corpus's commonest
-- complement phrase (a hundred and thirty-seven plural lines, seventy-five
-- of the distributive "each other creature you control").
-- spelling: ["other creature you control"], kind: Nominal
public export
otherCreatureYouControl : (n : Noun bs Object) -> {auto 0 ca : ComplementAnchor n} ->
                          {auto 0 ty : anchorTyFits [Creature] (nounTy n) = True} ->
                          Predicate bs Object
otherCreatureYouControl n = And [creature, ControlledBy You, OtherThan n {ca}]

-- "other player" — the player-kind complement, anchored to `You`
-- ([CR#102.1] makes "player" the universal word, so the exclusion is
-- what makes it an opponent-or-teammate read). Fifty-three corpus
-- lines write "each other player".
-- spelling: ["other player"], kind: Nominal (And [AnyPlayer, OtherThan You])
public export
otherPlayer : Predicate bs Player
otherPlayer = And [AnyPlayer, OtherThan You]

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
destroy : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
          {auto 0 na : NotAnyTarget n} -> Effect bs
destroy n = Composite Destroy (Move n graveyardZ {na}) {ok = DestroyB {z = ok} {na}}

-- "exile [n]" ([CR#701.13a]) — speculative pending its real macro.
-- spelling: ["exile <Param(0)>"], kind: Sentence (speculative -- no
-- action/Exile.ron artifact studied this pass; drafted by direct analogy
-- to Destroy.ron's shape)
public export
exile : (n : Noun bs Object) -> {auto 0 na : NotAnyTarget n} -> Effect bs
exile n = Composite Exile (Move n exileZ {na}) {ok = ExileB {na}}

-- "exile [n] with [amt] [kind] counter(s) on it" — a hundred and two
-- lines, forty-seven of them the suspend family's keyword reminder line
-- and fifty-five real ability lines, of which the self-exiling delay
-- family ("Exile Arc Blade with three time counters on it") is the
-- biggest block. The rider rides the PLACEMENT,
-- so this is `exile` with a bundle rather than a second verb.
-- spelling: ["exile <Param(0)> with <Param(1)> <Param(2)> counter(s) on
-- it"], kind: Sentence (Composite Exile over Move with a CounterRider --
-- see MoveRiders)
public export
exileWithCounters : (n : Noun bs Object) -> (amt : Amount (nomIntro n)) ->
                    (kind : CounterKind) ->
                    {auto 0 wc : WrittenCount amt} ->
                    {auto 0 na : NotAnyTarget n} -> Effect bs
exileWithCounters n amt kind =
  Composite Exile
            (Move n exileZ
                  {riders = MkMoveRiders [] Nothing
                                         {counters = Just (MkCounterRider amt kind {wc})}})
            {ok = ExileWithCountersB}

-- "return [n] to the battlefield under its owner's control with [amt]
-- [kind] counter(s) on it" — the counter rider's OTHER destination,
-- eighty-three lines, and the shape that shows the rider is the
-- placement's and not the exile verb's.
-- spelling: ["return <Param(0)> to the battlefield under <Param(1)>'s
-- control with <Param(2)> <Param(3)> counter(s) on it"], kind: Sentence
public export
returnToBattlefieldWithCounters :
  (n : Noun bs Object) -> (who : Noun (nomIntro n) Player) ->
  (amt : Amount (nomIntro n)) -> (kind : CounterKind) ->
  {auto 0 one : nounPlur who = OneOf} ->
  {auto 0 wc : WrittenCount amt} ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 na : NotAnyTarget n} ->
  Effect bs
returnToBattlefieldWithCounters n who amt kind =
  Move n battlefieldZ {pl} {na}
       {riders = MkMoveRiders [] (Just who) {one = OneController {one}}
                              {counters = Just (MkCounterRider amt kind {wc})}}

-- "put [n] onto the battlefield" — the placement with no adverbial after
-- it, which is what `Move … battlefieldZ` has always spelled; named so
-- the three ridden forms below read as its siblings.
-- spelling: ["put <Param(0)> onto the battlefield"], kind: Sentence
-- (Move … battlefieldZ with the empty rider bundle -- see MoveRiders)
public export
putOntoBattlefield : (n : Noun bs Object) ->
                     {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                     {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                     {auto 0 na : NotAnyTarget n} ->
                     Effect bs
putOntoBattlefield n = Move n battlefieldZ {pl} {na}

-- "put [n] onto the battlefield tapped" — three hundred fifteen lines,
-- the rider family's centre ([CR#110.5b] is the rule it overrides).
-- spelling: ["put <Param(0)> onto the battlefield tapped"], kind: Sentence
public export
putOntoBattlefieldTapped : (n : Noun bs Object) ->
                           {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
                           {auto 0 pl : Placeable (nounTy n) Battlefield} ->
                           {auto 0 na : NotAnyTarget n} ->
                           Effect bs
putOntoBattlefieldTapped n =
  Move n battlefieldZ {pl} {na} {riders = MkMoveRiders [EntersTapped] Nothing}

-- "put [n] onto the battlefield tapped and attacking" — nineteen lines
-- ([CR#506.3a]); the token twin is `createTappedAttacking`, and both
-- read chapter nineteen's `ridersOk` shapes.
-- spelling: ["put <Param(0)> onto the battlefield tapped and attacking"],
-- kind: Sentence
public export
putOntoBattlefieldTappedAttacking :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 na : NotAnyTarget n} ->
  Effect bs
putOntoBattlefieldTappedAttacking n =
  Move n battlefieldZ {pl} {na} {riders = MkMoveRiders [EntersTapped, EntersAttacking] Nothing}

-- "put [n] onto the battlefield under your control" — a hundred
-- twenty-seven of the hundred thirty-five control lines, the override of
-- [CR#110.2a]'s default made explicit.
-- spelling: ["put <Param(0)> onto the battlefield under your control"],
-- kind: Sentence
public export
putOntoBattlefieldUnderYourControl :
  (n : Noun bs Object) ->
  {auto 0 arr : ArrangementOk (nounPlur n) (battlefieldZ {bs = nomIntro n})} ->
  {auto 0 pl : Placeable (nounTy n) Battlefield} ->
  {auto 0 na : NotAnyTarget n} ->
  Effect bs
putOntoBattlefieldUnderYourControl n =
  Move n battlefieldZ {pl} {na} {riders = MkMoveRiders [] (Just You)}

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
            {auto 0 ok : OnBattlefield (nounZone n)} ->
            {auto 0 na : NotAnyTarget n} -> Effect bs
sacrifice agent n =
  Does agent Sacrifice (Move n graveyardZ {na}) {tb = SacrificeB {z = ok} {na}}

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
           {auto 0 dk : DiscardOk n} ->
           {auto 0 na : NotAnyTarget n} -> Effect bs
discards agent n =
  Does agent Discard (Move n graveyardZ {na}) {tb = DiscardB {d = dk} {na}}

-- "[agent] discard(s) a card" — the common phrase, spelled sort-only
-- (an owned-hand expansion needs a subject-read noun the vocabulary
-- lacks — not-settled).
-- spelling: ["<Param(0)> discard(s) a card"], kind: Sentence (sort-only
-- expansion of `discards` at `a (InZone handZ)`; not an independent frame)
public export
discardsACard : (agent : Noun bs Player) -> Effect bs
discardsACard agent = discards agent (a (InZone handZ))

-- The two divided surfaces, over the one `Distribute` primitive
-- ([CR#601.2d,115.7f] — "divide or distribute" is one mechanic and the
-- rules quote both words for it). One macro per verb, because English
-- gives each its own idiom and no third exists.

-- "[src] deals [amt] damage divided as you choose among [group]" (Arc
-- Lightning, Forked Bolt, Boulderfall) — sixty-nine corpus lines, the
-- adverbial riding the amount and the recipient taking "among" where an
-- undivided damage clause takes "to".
-- spelling: ["<Param(0)> deals <Param(1)> damage divided as you choose
-- among <Param(2)>"], kind: Sentence (the DividedDamage row of
-- Distribute -- see DividedVerb)
public export
dealsDivided : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
               (among : Noun (amtIntro amt) k) ->
               {auto 0 ds : DamageSource src} ->
               {auto 0 wc : WrittenCount amt} ->
               {auto 0 pl : nounPlur among = ManyOf} ->
               {auto 0 gm : GroupMention among} ->
               {auto 0 rk : DamageRecipient among} -> Effect bs
dealsDivided src amt among =
  Distribute (DividedDamage src {ds}) amt among {wc} {pl} {gm}
             {tk = DamageDivided {rk}}

-- "Distribute [amt] [kind] counters among [group]" (Armament Corps,
-- Case of the Trampled Garden) — forty-six corpus lines, and the verb
-- itself is what marks the division here. Agent-silent, as `PutCounters`
-- is and for its reason.
-- spelling: ["distribute <Param(0)> <Param(1)> counters among
-- <Param(2)>"], kind: Sentence (the DistributedCounters row of
-- Distribute -- see DividedVerb)
public export
distributeCounters : (amt : Amount bs) -> (kind : CounterKind) ->
                     (among : Noun (amtIntro amt) Object) ->
                     {auto 0 wc : WrittenCount amt} ->
                     {auto 0 pl : nounPlur among = ManyOf} ->
                     {auto 0 gm : GroupMention among} ->
                     {auto 0 zn : OnBattlefield (nounZone among)} -> Effect bs
distributeCounters amt kind among =
  Distribute (DistributedCounters kind) amt among {wc} {pl} {gm}
             {tk = CountersDistributed {zn}}

-- The duration adverbials, one macro per phrase the corpus writes, over
-- the decomposed endpoint. The structure carries the axes (boundary,
-- part, possession); these names carry the WORDS, and which of them a
-- construction may write is the span tables' answer (`spanUse`), not
-- the caller's.

-- "this turn" — the restrictions' current-turn adverbial.
-- spelling: (construction-owned -- pass-through to Duration.ThisTurn)
public export
thisTurn : Duration bs
thisTurn = ThisTurn

-- "until end of turn" — the grants' current-turn adverbial; bare, no
-- article ([CR#514.2] sweeps it in cleanup).
-- spelling: (construction-owned -- the bare end-of-turn endpoint, see
-- DurationEnd for why the boundary word and the article are its own)
public export
untilEndOfTurn : Duration bs
untilEndOfTurn = Until (EndOf Turn Nothing)

-- "until your next turn" — the cross-turn span, the one every
-- construction writes (the detain family's included).
-- spelling: (construction-owned -- the possessed start-of-turn endpoint)
public export
untilYourNextTurn : Duration bs
untilYourNextTurn = Until (StartOf Turn (Just Yours))

-- "until end of combat" ([CR#511.2]) — Glyph of Destruction.
-- spelling: (construction-owned -- the bare end-of-combat endpoint)
public export
untilEndOfCombat : Duration bs
untilEndOfCombat = Until (EndOf Combat Nothing)

-- "until your next upkeep" ([CR#503]) — Gabriel Angelfire; the one
-- endpoint the keyword grant writes alone.
-- spelling: (construction-owned -- the possessed start-of-upkeep endpoint)
public export
untilYourNextUpkeep : Duration bs
untilYourNextUpkeep = Until (StartOf Upkeep (Just Yours))

-- "until your next end step" ([CR#513]) — the impulse family's own
-- endpoint and nobody else's (`PermissionOnly`), eight lines of "You
-- may play those cards until your next end step".
-- spelling: (construction-owned -- the possessed start-of-end-step endpoint)
public export
untilYourNextEndStep : Duration bs
untilYourNextEndStep = Until (StartOf EndStep (Just Yours))

-- "as long as [condition], [statement]" ([CR#611.3a]) — the conditional
-- static, named for the words it writes.
-- spelling: ["as long as <Param(0)>, <Param(1)>"], kind: Sentence
-- (Conditionally -- see StaticEffect.Conditionally)
public export
asLongAs : (c : Condition bs) -> (se : StaticEffect (condIntro c)) ->
           {auto 0 nn : NotConditional se} -> StaticEffect bs
asLongAs c se = Conditionally c se {nn}

-- "[statement] unless [condition]" — the SAME wrapper under its second
-- marking word, which is why the macro takes the POSITIVE condition and
-- negates it: "unless" carries the negation, so what the caller writes
-- is what the card prints. A hundred fifty lines ("can't … unless" a
-- hundred eleven, "enters tapped unless" thirty-nine).
-- spelling: ["<Param(1)> unless <Param(0)>"], kind: Sentence
-- (Conditionally (NotCond …) … {marking = Unless} -- see CondMarking)
public export
-- the UNLESS marking's body stays at the incoming context, and that falls
-- out of the threading rather than being stipulated: the wrapper negates,
-- so the condition it hands `Conditionally` is a `NotCond`, and
-- `condIntro` mints for a self-subject `Matches` and for a comparison's
-- margin. A negated condition is neither -- `condIntro (NotCond c)` is the
-- incoming context by construction, and the comparison row is unreachable
-- under a negation for a second reason (`condNegatable` refuses the
-- comparison frame outright) -- so this signature writes `bs` directly
-- rather than an expression that always reduces to it. The corpus agrees
-- twice over: no "can't … unless" line pronominalises its condition's
-- subject, and none reads a margin either.
unlessSo : (c : Condition bs) -> (se : StaticEffect bs) ->
           {auto 0 ng : CondNegatable c} ->
           {auto 0 nn : NotConditional se} -> StaticEffect bs
unlessSo c se = Conditionally (NotCond c) se {marking = Unless} {nn}

-- "[n] enters tapped" ([CR#603.6d]) — the entry rider as a line, the
-- one rider a permanent's own text writes.
-- spelling: ["<Param(0)> enters tapped"], kind: Sentence
-- (EntersRider … EntersTapped -- see StaticEffect.EntersRider)
public export
entersTapped : (n : Noun bs Object) ->
               {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
               StaticEffect bs
entersTapped n = EntersRider n EntersTapped {zn}

-- "[n] enters with [k] [kind] counters on it" ([CR#603.6d,614.1d]) —
-- the other entry replacement, three hundred eighty-six lines.
-- spelling: ["<Param(0)> enters with <Param(1)> <Param(2)> counter(s) on
-- it"], kind: Sentence (EntersWithCounters -- see
-- StaticEffect.EntersWithCounters)
public export
entersWithCounters : (n : Noun bs Object) -> (k : Nat) -> (kind : CounterKind) ->
                     {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                     {auto 0 wc : WrittenCount {bs} (Lit k)} ->
                     StaticEffect bs
entersWithCounters n k kind = EntersWithCounters n (Lit k) kind {zn}

-- "[n] gets [+p/+t] [duration]" — the stat change and its adverbial,
-- the envelope's two halves under one name. Each grant macro threads
-- the clause's demands to its caller: a battlefield subject, and a span
-- its own construction writes.
-- spelling: ["<Param(0)> gets <Param(1)>/<Param(2)>"] (optional trailing
-- duration), kind: Sentence (Continuously (Gets …) d -- see
-- StaticEffect.Gets)
public export
gets : (n : Noun bs Object) -> (pow : PtShift (nomIntro n)) ->
       (tou : PtShift (nomIntro n)) ->
       (d : Maybe (Duration (nomIntro n))) ->
       {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
       {auto 0 ps : PumpSigns pow tou} ->
       {auto 0 sp : SpanOk PtDelta d} -> Effect bs
gets n pow tou d = Continuously (Gets n pow tou {ps}) d

-- "[n] gains [ability] [duration]"
-- spelling: ["<Param(0)> gains <Param(1)>"] (optional trailing duration),
-- kind: Sentence (Continuously (Gains …) d -- see StaticEffect.Gains)
public export
gains : (n : Noun bs Object) -> (a : Ability) -> (d : Maybe (Duration (nomIntro n))) ->
        {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
        {auto 0 gr : Grantable a} ->
        {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gains n a d = Continuously (Gains n a) d

-- "[n] gains haste [duration]"
-- spelling: ["<Param(0)> gains haste"] (optional trailing duration), kind:
-- Sentence (gains n (KeywordAbility Haste) d -- see Keyword,
-- StaticEffect.Gains)
public export
gainsHaste : (n : Noun bs Object) -> (d : Maybe (Duration (nomIntro n))) ->
             {auto 0 ok : ZoneFits (nounZone n) (Just Battlefield)} ->
             {auto 0 sp : SpanOk KeywordGrant d} -> Effect bs
gainsHaste n d = gains n (KeywordAbility Haste) d

-- The counter-kind WORDS, one macro per kind English writes, over the two
-- products `CounterKind` is: the stat pair and the keyword name. These are
-- the layer a catalog belongs in once the vocabulary beneath it is a
-- product — a new kind word is a line here, not a decision in the grammar.
-- The RON side owns what each kind DOES; a macro here neither models that
-- nor fabricates a card line for it.
-- spelling: ["+1/+1"] (the construction-owned kind word before
-- "counter(s)"; `BoostCounter (Up 1) (Up 1)` -- see CounterKind)
public export
plusOnePlusOne : CounterKind
plusOnePlusOne = BoostCounter (Up 1) (Up 1)

-- spelling: ["-1/-1"] (as plusOnePlusOne, the stat pair written down)
public export
minusOneMinusOne : CounterKind
minusOneMinusOne = BoostCounter (Down 1) (Down 1)

-- spelling: ["flying"] (the construction-owned kind word before
-- "counter(s)"; `KeywordCounter Flying` -- see CounterKind)
public export
flyingCounter : CounterKind
flyingCounter = KeywordCounter Flying

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
cantAttack : (n : Noun bs Object) -> (span : Maybe (Duration (nomIntro n))) ->
             {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
             {auto 0 dp : DeedParticipant Attack Agent (nounTy n)} ->
             {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantAttack n span = Continuously (Deontic n Forbid Attack Agent Nothing {zn} {dp}) span {sp}

-- "[n] can't block [duration]" ([CR#509.1b]) — Blindblast, Blinding
-- Flare.
-- spelling: ["<Param(0)> can't block <Param(1)>"], kind: Sentence
public export
cantBlock : (n : Noun bs Object) -> (span : Maybe (Duration (nomIntro n))) ->
            {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
            {auto 0 dp : DeedParticipant Block Agent (nounTy n)} ->
            {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBlock n span = Continuously (Deontic n Forbid Block Agent Nothing {zn} {dp}) span {sp}

-- "[n] can't be blocked [duration]" ([CR#509.1b]) — Infiltrate; the
-- passive of the same deed, the subject standing in core's `on` slot.
-- spelling: ["<Param(0)> can't be blocked <Param(1)>"], kind: Sentence
public export
cantBeBlocked : (n : Noun bs Object) -> (span : Maybe (Duration (nomIntro n))) ->
                {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
                {auto 0 dp : DeedParticipant Block Patient (nounTy n)} ->
                {auto 0 sp : SpanOk DeedRestriction span} -> Effect bs
cantBeBlocked n span = Continuously (Deontic n Forbid Block Patient Nothing {zn} {dp}) span {sp}

-- The control grant and its adverbial, the envelope's two halves under
-- one name — `gets`/`gains` for the layer-2 verb. Two macros for the
-- two subject spellings oracle writes: the imperative with its
-- unpronounced `You` (two hundred and forty-six lines) and the written
-- subject (seventy-three).

-- "gain control of [n] [duration]" ([CR#613.1b]) — Act of Treason,
-- Mind Flayer.
-- spelling: ["gain control of <Param(0)>"] (optional trailing duration),
-- kind: Sentence (Continuously (GainsControl You n) d -- see
-- StaticEffect.GainsControl)
public export
gainControl : (n : Noun bs Object) -> (d : Maybe (Duration (nomIntro n))) ->
              {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
              {auto 0 sp : SpanOk ControlGrant d} -> Effect bs
gainControl n d = Continuously (GainsControl You n {zn}) d {sp}

-- "[who] gains control of [what] [duration]" — the written-subject
-- form, and what the exchange's halves are built from.
-- spelling: ["<Param(0)> gains control of <Param(1)>"] (optional trailing
-- duration), kind: Sentence
public export
gainsControl : (who : Noun bs Player) -> (what : Noun (nomIntro who) Object) ->
               (d : Maybe (Duration (nomIntro what))) ->
               {auto 0 zn : ZoneFits (nounZone what) (Just Battlefield)} ->
               {auto 0 sp : SpanOk ControlGrant d} -> Effect bs
gainsControl who what d = Continuously (GainsControl who what {zn}) d {sp}

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

-- The four may-clause surfaces, macros over the one `May` primitive
-- exactly as core's branchless and branching mays are one node
-- (`deckmaste_core/src/effect.rs`). What varies is which anaphoric
-- sentence follows, and the branch fields are where it goes; the
-- pronoun and its agreement are the decider's own.

-- "[decider] may [effect]" — the branchless offer.
-- spelling: ["<Param(0)> may <Param(1)>"], kind: Sentence
public export
may : (decider : Noun bs Player) -> Effect (nomIntro decider) -> Effect bs
may d body = May (Just d) body Nothing Nothing

-- "[decider] may [effect]. If [decider] do, [effect]." — the taken
-- branch, which reads everything the body introduced.
-- spelling: ["<Param(0)> may <Param(1)>. If <Param(0)> do, <Param(2)>."],
-- kind: Sentence (the anaphor's pronoun is Param(0)'s -- "if you do",
-- "if they do")
public export
mayThen : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (effIntro body) -> Effect bs
mayThen d body did = May (Just d) body (Just did) Nothing

-- "[decider] may [effect]. If [decider] don't, [effect]." — the
-- declined branch, which reads only what preceded the may.
-- spelling: ["<Param(0)> may <Param(1)>. If <Param(0)> don't,
-- <Param(2)>."], kind: Sentence (see mayThen)
public export
mayElse : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (nomIntro decider) -> Effect bs
mayElse d body notd = May (Just d) body Nothing (Just notd)

-- "[decider] may [effect]. If [decider] do, [effect]. If [decider]
-- don't, [effect]." — both branches, each typed as its own surface types
-- it: the taken arm reads the body, the declined arm reads only what
-- preceded the may. Crovax the Cursed writes it.
-- spelling: ["<Param(0)> may <Param(1)>. If <Param(0)> do, <Param(2)>. If
-- <Param(0)> don't, <Param(3)>."], kind: Sentence (see mayThen)
public export
mayThenElse : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
              Effect (effIntro body) -> Effect (nomIntro decider) -> Effect bs
mayThenElse d body did notd = May (Just d) body (Just did) (Just notd)

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
creatureTokOf : (pow : Amount bs) -> (tou : Amount bs) -> List Color -> List Subtype ->
                TokenChars bs
creatureTokOf pow tou cs ss = MkToken (Just (pow, tou)) cs (MkTypeLine ss [Creature]) [] Nothing

-- "[p]/[t] [colors] [subtypes] creature token" at WRITTEN numbers -- the
-- overwhelming majority, and `entersWithCounters`' shape: the numeral is
-- taken as a numeral and wrapped, so a call site that writes a number
-- writes a number.
-- spelling: (construction-owned -- see creatureTokOf), kind: Nominal
public export
creatureTok : (pow : Nat) -> (tou : Nat) -> List Color -> List Subtype -> TokenChars bs
creatureTok pow tou cs ss = creatureTokOf (Lit pow) (Lit tou) cs ss

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
create : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
         {auto 0 tt : TokenTyped tok} ->
         {auto 0 tp : TokenPt tok} ->
         {auto 0 sf : SubtypesFit tok} ->
         {auto 0 tc : TokenCanonical tok} ->
         {auto 0 wc : WrittenCount count} -> Effect bs
create count tok = Create You count tok [] {tt} {tp} {sf} {tc} {wc} {rr = MkRidersOk}

-- "Create [count] [chars] token(s) that's tapped and attacking"
-- ([CR#508.4]) — the arrival pair, which is the only shape the corpus
-- writes attacking in (sixty-six create-token lines; attacking without
-- tapped, none).
-- spelling: ["create <Param(0)> <Param(1)> token(s) that's tapped and
-- attacking"], kind: Sentence
public export
createTappedAttacking : (count : Amount bs) -> (tok : TokenChars (amtIntro count)) ->
                        {auto 0 tt : TokenTyped tok} ->
                        {auto 0 tp : TokenPt tok} ->
                        {auto 0 sf : SubtypesFit tok} ->
                        {auto 0 tc : TokenCanonical tok} ->
                        {auto 0 wc : WrittenCount count} -> Effect bs
createTappedAttacking count tok =
  Create You count tok [EntersTapped, EntersAttacking] {tt} {tp} {sf} {tc} {wc}
         {rr = MkRidersOk}

-- "[n] becomes [type line] in addition to its other types [duration]"
-- ([CR#205.1b]) — the static effect and its adverbial under one name, the
-- `gets`/`gains` shape for the type change.
-- spelling: ["<Param(0)> becomes <Param(1)> in addition to its other types"]
-- (optional trailing duration), kind: Sentence (Continuously (BecomesAlso …) d
-- -- see StaticEffect.BecomesAlso)
public export
becomes : (n : Noun bs Object) -> (added : TypeLine) ->
          (d : Maybe (Duration (nomIntro n))) ->
          {auto 0 zn : ZoneFits (nounZone n) (Just Battlefield)} ->
          {auto 0 ne : LineNonEmpty added} ->
          {auto 0 nw : AddsSomething (nounTy n) added} ->
          {auto 0 af : AddedFits (nounTy n) added} ->
          {auto 0 sp : SpanOk TypeAddition d} -> Effect bs
becomes n added d = Continuously (BecomesAlso n added {zn} {ne} {nw} {af}) d {sp}

-- The draw surfaces, over the one `Draw` primitive ([CR#121.1]). The
-- imperative's unpronounced subject is `You` spelled explicitly, which is
-- the `create`/`sacrifice` pattern, and the count is the ordinary amount
-- vocabulary.

-- "Draw a card." — the single commonest sentence in the corpus (a
-- thousand nine hundred sixty-three lines).
-- spelling: ["draw a card"], kind: Sentence
public export
drawACard : Effect bs
drawACard = Draw You (Lit 1)

-- "Draw [n] cards." — the counted imperative ("Draw two cards", two
-- hundred seventy-four lines; "Draw three cards", a hundred twenty-eight).
-- spelling: ["draw <Param(0)> cards"], kind: Sentence
public export
drawCards : (n : Nat) -> {auto 0 wc : WrittenCount {bs} (Lit n)} -> Effect bs
drawCards n = Draw You (Lit n) {wc}

-- "[who] draws a card" — the subjected form ("Target player draws a
-- card", twenty lines; "Each player draws a card", twenty-nine).
-- spelling: ["<Param(0)> draws a card"], kind: Sentence (auto-inflection
-- supplies the agreement)
public export
drawsACard : (who : Noun bs Player) -> Effect bs
drawsACard who = Draw who (Lit 1)

-- The modal headcount surfaces, one macro per phrase the corpus writes
-- over the one `Modal` primitive ([CR#700.2]). Each is the quantity
-- vocabulary in the modal's head slot and nothing more; the gates travel
-- to the caller, an abstract mode list being unable to discharge them.

-- "Choose one — • … • …" — four hundred seventy cards, over two, three,
-- or four modes.
-- spelling: ["choose one — <Param(0)>"], kind: Sentence
public export
chooseOne : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly 1) (modeCount modes)} ->
            {auto 0 mh : ModalHead (exactly 1) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOne modes = Modal (exactly 1) modes {tw} {mf} {mh} {dm}

-- "Choose two — • … • …" — thirty-two cards, over three or four modes.
-- spelling: ["choose two — <Param(0)>"], kind: Sentence
public export
chooseTwo : (modes : List (Effect bs)) ->
            {auto 0 tw : AtLeastTwo (modeCount modes)} ->
            {auto 0 mf : ModesFit (exactly 2) (modeCount modes)} ->
            {auto 0 mh : ModalHead (exactly 2) (modeCount modes)} ->
            {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseTwo modes = Modal (exactly 2) modes {tw} {mf} {mh} {dm}

-- "Choose one or both — • … • …" — fifty-two cards, every one over
-- exactly two modes (see `oneOrBoth`).
-- spelling: ["choose one or both — <Param(0)>"], kind: Sentence
public export
chooseOneOrBoth : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit Macros.oneOrBoth (modeCount modes)} ->
                  {auto 0 mh : ModalHead Macros.oneOrBoth (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrBoth modes = Modal Macros.oneOrBoth modes {tw} {mf} {mh} {dm}

-- "Choose one or more — • … • …" — nineteen cards, over three, four, or
-- five modes (see `atLeast`).
-- spelling: ["choose one or more — <Param(0)>"], kind: Sentence
public export
chooseOneOrMore : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit (atLeast 1) (modeCount modes)} ->
                  {auto 0 mh : ModalHead (atLeast 1) (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseOneOrMore modes = Modal (atLeast 1) modes {tw} {mf} {mh} {dm}

-- "Choose any number — • … • …" — the unbounded head, and the one where
-- the floor is genuinely zero: [CR#107.1c] says that a player instructed
-- to choose "any number" "may choose any positive number or zero", so a
-- controller may take none of the modes at all. [CR#700.2] asks only
-- that the list be preceded by "instructions for a player to choose a
-- number of those options" and imposes no minimum of its own, and the
-- quantity says the same thing structurally (`anyNumber` is the range
-- open at both ends). Two corpus lines head a bulleted list with it —
-- Rankle, Master of Pranks and Rankle and Torbran.
-- spelling: ["choose any number — <Param(0)>"], kind: Sentence
public export
chooseAnyNumber : (modes : List (Effect bs)) ->
                  {auto 0 tw : AtLeastTwo (modeCount modes)} ->
                  {auto 0 mf : ModesFit Macros.anyNumber (modeCount modes)} ->
                  {auto 0 mh : ModalHead Macros.anyNumber (modeCount modes)} ->
                  {auto 0 dm : distinctModes modes = True} -> Effect bs
chooseAnyNumber modes = Modal Macros.anyNumber modes {tw} {mf} {mh} {dm}

-- "if [subject] don't/doesn't [condition]" / "if [subject] isn't
-- [predicate]" — the negated condition ([CR#701.47a] writes both of
-- amass's branches with it).
-- spelling: (construction-owned -- negates its inner condition's own
-- frame; see Condition.NotCond)
public export
notSo : (c : Condition bs) -> {auto 0 ng : CondNegatable c} -> Condition bs
notSo c = NotCond c {ng}

-- "it's [pred]" — the reference-matches condition over the singular
-- object read, which is the subject every corpus line writes it with
-- ("if it's a creature card", "if it's attacking").
-- spelling: ["it's <Param(0)>"], kind: TODO(reason: condition fragment --
-- not one of Nominal/Sentence/Cost/KeywordLine/Ability)
public export
itsA : (p : Predicate bs Object) -> {auto 0 ok : countOnes Object bs = 1} ->
       {auto 0 sy : PredSays p} ->
       {auto 0 zc : ZoneFits (zoneOfIt bs) (seedZone p)} ->
       {auto 0 af : AnyTargetFree p} -> Condition bs
itsA p = Matches (It {ok}) p {sy} {zc} {af}

-- "it isn't [pred]" — `itsA` negated, the frame amass's last sentence
-- writes ("If it isn't a Zombie, …", [CR#701.47a]) and real card text
-- with it ("If it isn't a creature, it becomes a 0/0 creature in addition
-- to its other types.").
-- spelling: ["it isn't <Param(0)>"], kind: TODO(reason: condition
-- fragment, see itsA)
public export
itIsntA : (p : Predicate bs Object) -> {auto 0 ok : countOnes Object bs = 1} ->
          {auto 0 sy : PredSays p} ->
          {auto 0 zc : ZoneFits (zoneOfIt bs) (seedZone p)} ->
          {auto 0 af : AnyTargetFree p} ->
          {auto 0 nf : predNegFree p = True} -> Condition bs
itIsntA p = NotCond (itsA p {ok} {sy} {zc} {af}) {ng = MkCondNegatable {ok = nf}}

-- The LIBRARY surfaces, over `ZoneAt Library` (the zone whole) and
-- `LibraryAt` (a position in it). The split is core's `Zone::Library`
-- against `Destination::Library(Anchor)` and it is English's too: a
-- search looks through the zone, a placement names a place in it.

-- "[player]'s library" — the zone whole, what a search reads and a
-- shuffle randomizes ([CR#401.2]).
-- spelling: ["<Param(0)>'s library"], kind: Nominal (see handOf)
public export
libraryOf : (n : Noun bs Player) -> {auto 0 pn : Possessor n} -> ZoneExpr bs
libraryOf n = ZoneAt Library (OwnedBy n {ps = LibraryIsOwned} {pn})

-- "your library" — eight hundred twenty-five search lines' own phrase.
-- spelling: ["your library"], kind: Nominal
public export
yourLibrary : ZoneExpr bs
yourLibrary = libraryOf You

-- "on top of your library" — the placement with no order stated (a
-- hundred twenty-seven lines).
-- spelling: ["on top of your library"], kind: Nominal
public export
onTopZ : ZoneExpr bs
onTopZ = LibraryAt OnTop Nothing Bare

-- "on the bottom of your library" — four hundred eighteen lines, the
-- dominant library placement.
-- spelling: ["on the bottom of your library"], kind: Nominal
public export
onBottomZ : ZoneExpr bs
onBottomZ = LibraryAt OnBottom Nothing Bare

-- "on top of your library in [any/a random] order" — the plural
-- placement with its [CR#401.4] rider (fifty-three lines at "in any
-- order").
-- spelling: ["on top of your library <Param(0)>"], kind: Nominal
public export
onTopIn : Arrangement -> ZoneExpr bs
onTopIn a = LibraryAt OnTop (Just a) Bare

-- "on the bottom of your library in [any/a random] order" — a hundred
-- four lines at "in any order", two hundred sixty-six at "in a random
-- order".
-- spelling: ["on the bottom of your library <Param(0)>"], kind: Nominal
public export
onBottomIn : Arrangement -> ZoneExpr bs
onBottomIn a = LibraryAt OnBottom (Just a) Bare

-- "the top [n] cards of your library" — the assembled slice (four
-- hundred forty-nine "look at" lines, ninety-five "reveal" ones).
-- spelling: ["the top <Param(0)> cards of your library"], kind: Nominal
public export
topCards : (n : Nat) -> {auto 0 wc : WrittenCount {bs} (Lit n)} -> Noun bs Object
topCards n = LibrarySlice OnTop (Lit n) You {wc}

-- "the top card of your library" — five hundred thirty-nine lines.
-- spelling: ["the top card of your library"], kind: Nominal
public export
topCard : Noun bs Object
topCard = topCards 1

-- "the bottom card of your library" — ONE line (Grenzo, Dungeon
-- Warden), and the whole of the bottom slice's attestation.
-- spelling: ["the bottom card of your library"], kind: Nominal
public export
bottomCard : Noun bs Object
bottomCard = LibrarySlice OnBottom (Lit 1) You

-- "[q] of [group]" — the partitive over a group mention.
-- spelling: ["one of <Param(0)>"], kind: Nominal
public export
oneOf : (grp : Noun bs Object) -> {auto 0 gm : GroupMention grp} -> Noun bs Object
oneOf grp = SomeOf (exactly 1) grp {gm}

-- "[n] of [group]" — the counted partitive ("two of them",
-- twenty-five lines).
-- spelling: ["<Param(0)> of <Param(1)>"], kind: Nominal
public export
someOf : (n : Nat) -> (grp : Noun bs Object) -> {auto 0 gm : GroupMention grp} ->
         {auto 0 nz : NonZeroQ (exactly n)} ->
         {auto 0 wf : WellFormedQ (exactly n)} -> Noun bs Object
someOf n grp = SomeOf (exactly n) grp {gm} {nz} {wf}

-- The EXPOSURE surfaces, over the one `Expose` primitive
-- ([CR#701.20a,701.20e]). The imperative's unpronounced subject is
-- `You` spelled explicitly, as the draw macros do.

-- "Look at [cards]." — the one-player exposure.
-- spelling: ["look at <Param(0)>"], kind: Sentence
public export
lookAt : (n : Noun bs Object) -> Effect bs
lookAt n = Expose LookAt You (ExposedCards n)

-- "Reveal [cards]." — the all-players exposure.
-- spelling: ["reveal <Param(0)>"], kind: Sentence
public export
revealCards : (n : Noun bs Object) -> Effect bs
revealCards n = Expose Reveal You (ExposedCards n)

-- "Look at [player]'s hand." — the peek (twelve lines naming a
-- player, eleven an opponent).
-- spelling: ["look at <Param(0)>'s hand"], kind: Sentence
public export
lookAtHandOf : (n : Noun bs Player) -> {auto 0 pn : Possessor n} -> Effect bs
lookAtHandOf n = Expose LookAt You (ExposedZone (handOf n {pn}))

-- "[who] reveals their hand." — a hundred twenty-eight lines; the
-- possessive reads the subject the clause just named.
-- spelling: ["<Param(0)> reveals their hand"], kind: Sentence
public export
revealsTheirHand : (who : Noun bs Player) ->
                   {auto 0 ok : countOnes Player (nomIntro who) = 1} -> Effect bs
revealsTheirHand who = Expose Reveal who (ExposedZone (handOf (They {ok})))

-- The MILL surfaces, over the one `Mill` primitive ([CR#701.17a]).

-- "Mill [n] cards." — the imperative (twenty-six lines at the sentence
-- head; four hundred seventy-six writing the verb at all).
-- spelling: ["mill <Param(0)> cards"], kind: Sentence
public export
millCards : (n : Nat) -> {auto 0 wc : WrittenCount {bs} (Lit n)} -> Effect bs
millCards n = Mill You (Lit n) {wc}

-- "[who] mills [n] cards." — the subjected form ("Target player mills
-- ten cards", Glimpse the Unthinkable).
-- spelling: ["<Param(0)> mills <Param(1)> cards"], kind: Sentence
public export
millsCards : (who : Noun bs Player) -> (n : Nat) ->
             {auto 0 wc : WrittenCount {bs = nomIntro who} (Lit n)} -> Effect bs
millsCards who n = Mill who (Lit n) {wc}

-- The SEARCH and SHUFFLE surfaces ([CR#701.23a,701.24a]).

-- "Search your library for [description]." — eight hundred twenty-five
-- lines' own frame.
-- spelling: ["search your library for <Param(0)>"], kind: Sentence
public export
searchLibraryFor : (p : Predicate bs Object) ->
                   {auto 0 hd : Headed p} ->
                   {auto 0 af : AnyTargetFree p} ->
                   {auto 0 zf : ZoneFree p} -> Effect bs
searchLibraryFor p = Search You yourLibrary p {hd} {af} {zf}

-- "Then shuffle." — seven hundred ninety-four lines, the object elided.
-- spelling: ["shuffle"], kind: Sentence
public export
shuffle : Effect bs
shuffle = Shuffle You

-- "[whose] life total becomes [n]" — the set-to (twenty-nine corpus
-- lines; the rules it answers to are on `LifeOp`).
-- spelling: ["<Param(0)>'s life total becomes <Param(1)>"], kind: Sentence
public export
lifeTotalBecomes : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
lifeTotalBecomes who a = ChangeLife who (Set a)

-- The REPLACEMENT and PREVENTION surfaces, macros over the two new
-- `StaticEffect` rows under the envelope every continuous clause
-- already shares. Each threads its construction's own span demand to
-- the caller, exactly as the grant macros do: the span tables answer
-- which adverbial a shield may write, and the caller never does.

-- "if [event], [replacement] instead [duration]" — the repeated
-- interception ([CR#614.1a]), the word English uses where the event can
-- happen only once.
-- spelling: ["if <Param(0)>, <Param(1)> instead"] (optional trailing
-- duration), kind: Sentence (Continuously (Intercepts … Repeatedly) d --
-- see StaticEffect.Intercepts)
public export
ifWouldInstead : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                 (d : Maybe (Duration (eventIntro ev))) ->
                 {auto 0 ok : Interceptable ev} ->
                 {auto 0 uo : ReplUseOk ev Repeatedly} ->
                 {auto 0 sp : SpanOk Replacement d} -> Effect bs
ifWouldInstead ev repl d = Continuously (Intercepts ev repl Repeatedly {ok} {uo}) d {sp}

-- "the next time [event], [replacement] instead [duration]" — the
-- single-use shield ([CR#614.3]'s "used up" ending), the word English
-- uses where the event repeats.
-- spelling: ["the next time <Param(0)>, <Param(1)> instead"] (optional
-- trailing duration), kind: Sentence (Continuously (Intercepts …
-- NextTimeOnly) d)
public export
nextTimeWouldInstead : (ev : GameEvent bs) -> (repl : Effect (eventIntro ev)) ->
                       (d : Maybe (Duration (eventIntro ev))) ->
                       {auto 0 ok : Interceptable ev} ->
                       {auto 0 uo : ReplUseOk ev NextTimeOnly} ->
                       {auto 0 sp : SpanOk Replacement d} -> Effect bs
nextTimeWouldInstead ev repl d =
  Continuously (Intercepts ev repl NextTimeOnly {ok} {uo}) d {sp}

-- "prevent all [kind] damage that would be dealt [scope] [duration]"
-- ([CR#615.1a]) — Fog at `Everywhere`, Indestructible Aura at a target.
-- spelling: ["prevent all <Param(0)> damage that would be dealt
-- <Param(1)>"] (optional trailing duration), kind: Sentence
-- (Continuously (Prevents … AllOfIt …) d -- see StaticEffect.Prevents)
public export
preventAll : (kind : DamageKind) -> (scope : DamageScope bs) ->
             (d : Maybe (Duration (scopeIntro scope))) ->
             {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventAll kind scope d = Continuously (Prevents kind AllOfIt scope) d {sp}

-- "prevent the next [n] [kind] damage that would be dealt [scope]
-- [duration]" ([CR#615.7]'s numbered shield) — Healing Salve.
-- spelling: ["prevent the next <Param(1)> <Param(0)> damage that would be
-- dealt <Param(2)>"] (optional trailing duration), kind: Sentence
public export
preventNext : (kind : DamageKind) -> (amt : Amount bs) ->
              (scope : DamageScope (amtIntro amt)) ->
              (d : Maybe (Duration (scopeIntro scope))) ->
              {auto 0 wc : WrittenCount amt} ->
              {auto 0 sp : SpanOk Prevention d} -> Effect bs
preventNext kind amt scope d = Continuously (Prevents kind (TheNext amt {wc}) scope) d {sp}

-- "to [n]" — the shield's recipient phrase, reading the damage clause's
-- own recipient table.
-- spelling: ["to <Param(0)>"], kind: TODO(reason: prepositional-phrase
-- fragment -- not one of the five FragmentKinds)
public export
shieldingIt : {k : Kind} -> (n : Noun bs k) ->
              {auto 0 rk : DamageRecipient n} -> DamageScope bs
shieldingIt n = ToRecipient n {rk}

-- "exile [n] until [event]" — the [CR#610.3] rider on the one clause
-- the corpus hangs it on (Banisher Priest, Banishing Light).
-- spelling: ["exile <Param(0)> until <Param(1)>"], kind: Sentence
-- (HeldUntil (exile n) ev -- see Effect.HeldUntil)
public export
exileUntil : (n : Noun bs Object) -> {auto 0 na : NotAnyTarget n} ->
             (ev : GameEvent (preIntro (exile n))) ->
             {auto 0 hd : Holdable ev} -> Effect bs
exileUntil n ev = HeldUntil (exile n) ev {ok = MkHeldClause} {hd}

-- "[replaced]. [replacement] instead." — the self-replacement
-- ([CR#614.15]); the replacement reads what the replaced clause
-- ANNOUNCED and nothing it did.
-- spelling: ["<Param(0)>. <Param(1)> instead."], kind: TODO(reason:
-- two-sentence body -- see Effect.InsteadOf)
public export
insteadOf : (replaced : Effect bs) -> (repl : Effect (annIntro replaced)) ->
            {auto 0 na : NotInstead replaced} ->
            {auto 0 nb : NotInstead repl} -> Effect bs
insteadOf replaced repl = InsteadOf replaced repl {na} {nb}

-- The mana symbols as one-word names, so a cost reads the way the card
-- prints it. Each is a projection of `ManaSymbol`'s compositional shape
-- and none is a construction of its own — the symbol vocabulary is the
-- port, these are its spellings.

-- "{n}" — the generic numeral ([CR#107.4b]).
-- spelling: ["{<Param(0)>}"], kind: Cost (component-owned -- see ManaSymbol)
public export
generic : Nat -> ManaSymbol
generic n = Simple (Generic n)

-- "{W}".."{G}" — a colored pip ([CR#107.4a]).
-- spelling: ["{<Param(0) code>}"], kind: Cost (see ManaSymbol)
public export
pip : Color -> ManaSymbol
pip c = Simple (Specific (OfColor c))

-- "{C}" — the colorless pip ([CR#107.4c]).
-- spelling: ["{C}"], kind: Cost (see ManaSymbol)
public export
colorlessPip : ManaSymbol
colorlessPip = Simple (Specific Colorless)

-- "{W/U}" — the two-color hybrid ([CR#107.4e]).
-- spelling: ["{<Param(0) code>/<Param(1) code>}"], kind: Cost
public export
hybridPip : (a : Color) -> (b : Color) ->
            {auto 0 ds : HalvesDistinct (Specific (OfColor a)) b} -> ManaSymbol
hybridPip a b = Hybrid (Specific (OfColor a)) b {ds}

-- "{2/B}" — the monocolored hybrid, whose left half is a generic amount
-- ([CR#107.4e]).
-- spelling: ["{<Param(0)>/<Param(1) code>}"], kind: Cost
public export
monoHybridPip : Nat -> Color -> ManaSymbol
monoHybridPip n c = Hybrid (Generic n) c

-- "{W/P}" — the Phyrexian pip ([CR#107.4f]).
-- spelling: ["{<Param(0) code>/P}"], kind: Cost
public export
phyrexianPip : Color -> ManaSymbol
phyrexianPip c = Phyrexian c Nothing

-- "Pay [n] life" as a cost component ([CR#118.3b]) — the life clause
-- under `Do`, which is what makes it the SAME payment the sentence
-- "you lose N life" describes and not a second verb (core spells it
-- `Do(ChangeLife(You, Down(n)))` too).
-- spelling: ["Pay <Param(1)> life"], kind: Cost (the cost spelling of
-- ChangeLife's Down; the sentence frame writes "<Param(0)> loses
-- <Param(1)> life" for the same clause)
public export
-- No payer demand here: [CR#602.1a] charges the ACTIVATOR, but this
-- component also stands under the `Pay` clause, where the sentence names
-- its own payer ("unless that player pays 3 life"). The activation
-- position asks the question instead (`CostPaidByYou`).
payLife : (who : Noun bs Player) -> (n : Nat) -> Cost bs
payLife who n = Do (ChangeLife who (Down (Lit n)))

-- "[body]. If [body's agent] do, [effect]." — the MANDATORY twin of
-- `mayThen` ([CR#118.12], a hundred forty-two lines): the same node with
-- no offer, so the first sentence is an instruction and the arm still
-- asks whether the payment was started.
-- spelling: ["<Param(0)>. If <agent of Param(0)> do, <Param(1)>."],
-- kind: Sentence (the anaphor's pronoun is the body's own agent -- "if
-- you do" for an imperative, "if they do" for a named subject)
public export
doThen : (body : Effect bs) -> Effect (effIntro body) -> Effect bs
doThen body did = May Nothing body (Just did) Nothing

-- "[body]. If [body's agent] don't, [effect]." — the declined arm of the
-- same mandatory node, which reads only what preceded the instruction.
-- spelling: ["<Param(0)>. If <agent of Param(0)> don't, <Param(1)>."],
-- kind: Sentence (see doThen)
public export
doElse : (body : Effect bs) -> Effect bs -> Effect bs
doElse body notd = May Nothing body Nothing (Just notd)

-- "[decider] may [body]. When [decider] do, [trigger]." — the REFLEXIVE
-- trigger over an OFFER ([CR#603.12]'s "allow" half, two hundred lines of
-- the family's two hundred ninety-one). The offer is `May`'s own
-- branchless node, so this macro mints no second decider slot and the
-- taken-ness the branches read is the taken-ness the trigger reads; what
-- differs is that this one is an ABILITY and waits on the stack
-- ([CR#603.3]) where `mayThen`'s arm runs inside the same resolution.
-- The bare-instruction enclosure ([CR#603.12]'s "instruct" half, ninety-
-- one lines) is `Reflexively` itself with no wrapper, which is `doThen`'s
-- relation to `mayThen` one construction over.
-- spelling: ["<Param(0)> may <Param(1)>. When <Param(0)> do, <Param(2)>."],
-- kind: Sentence (the anaphor's pronoun and agreement are Param(0)'s, as
-- on every arm of the may -- "when you do", Yes Man's "when they do")
public export
mayWhen : (decider : Noun bs Player) -> (body : Effect (nomIntro decider)) ->
          Effect (settleTargets (effIntro body)) ->
          {auto 0 ok : admitsReflexEnclosure (reflexEncloseUse body) = True} ->
          Effect bs
mayWhen d body trig =
  Reflexively (May (Just d) body Nothing Nothing) trig {en = MkReflexEnclosure {ok}}
