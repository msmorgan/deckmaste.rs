||| The macro layer: one definition per English phrase shape.
||| Experimental's analogue of the Macros-over-Semantics split.
module Experimental.Macros

import public Experimental

%default total

-- "target [pred]" — the singular counted mention. One constructor
-- serves every count ([CR#601.2c] announces them all alike); at one
-- the numeral is what rendering leaves unwritten ("target creature",
-- never "one target creature").
public export
target : (p : Predicate bs k) -> {auto tk : Targetable k} ->
         {auto 0 hd : Headed p} -> Noun bs k
target p = TargetGroup 1 p {tk} {hd}

-- "creature"
public export
creature : Predicate bs Object
creature = HasType Creature

-- "creature you control"
public export
creatureYouControl : Predicate bs Object
creatureYouControl = And [creature, ControlledBy You]

-- "creature you don't control"
public export
creatureYouDontControl : Predicate bs Object
creatureYouDontControl = And [creature, Not (ControlledBy You)]

-- "an opponent"
public export
anOpponent : Noun bs Player
anOpponent = A Opponent

-- "any other target" — the macro CARRIES its phrase's presupposition
-- (an earlier target) in its type.
public export
anyOtherTarget : {auto 0 ok : anyTargeted Object bs = True} -> Predicate bs Object
anyOtherTarget = And [AnyTarget, Other]

-- "destroy [n]" — mirrors plugins/builtin/macros/action/Destroy.ron:
-- the Destroy tag over the battlefield→graveyard move [CR#701.8a];
-- only a battlefield permanent is destroyable (`badDestroyGraveyard`).
public export
destroy : (n : Noun bs Object) -> {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
destroy n = Composite Destroy (Move n GraveyardZ) {ok = DestroyB {z = ok}}

-- "exile [n]" ([CR#701.13a]) — speculative pending its real macro.
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
public export
discards : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) ->
           {auto 0 dk : DiscardOk n} -> Effect bs
discards agent n = Does agent Discard (Move n GraveyardZ) {tb = DiscardB {d = dk}}

-- "[agent] discard(s) a card" — the common phrase, spelled sort-only
-- (an owned-hand expansion needs a subject-read noun the vocabulary
-- lacks — not-settled).
public export
discardsACard : (agent : Noun bs Player) -> Effect bs
discardsACard agent = discards agent (A (InZone HandZ))

-- "[n] gains haste [duration]"
public export
gainsHaste : (n : Noun bs Object) -> Maybe Duration ->
             {auto 0 ok : OnBattlefield (nounZone n)} -> Effect bs
gainsHaste n d = Gain n (KeywordAbility Haste) d

-- "[who] loses [amt] life"
public export
losesLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
losesLife who amt = ChangeLife who (Down amt)

-- "[who] gains [amt] life"
public export
gainsLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
gainsLife who amt = ChangeLife who (Up amt)
