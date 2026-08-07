||| The semantics-target workbench: what shape should the semantics layer
||| be, sitting between English and core? This module is DELIBERATELY
||| divorced from `Semantics` — no import, no reuse of its types — because
||| the verifier is what the original reasoning tool grew into under
||| direct-runnability pressure, and the destination has to be designable
||| without that baggage. Nothing here is emitted, mirrored, or run; the
||| typechecker is the only consumer. Grow the target shape in this file;
||| the eventual macro/card rewrite aims at what accumulates here.
||| (`Bridge` holds worked new-form ⇄ verifier-form pairs — the seed of the
||| lowering changes that accompany that rewrite.)
|||
||| THE RULES OF THUMB (owner-settled in the 2026-08-07 design discussion):
|||
||| - We translate from the raw English AST; this grammar encodes the raw
|||   MTG card grammar, a REDUCED BUT EQUIVALENT form of English. Nothing
|||   is rearranged or lifted out of surface position — `Target`, `Each`,
|||   and friends occupy the argument positions the words occupy.
|||   Rearrangement (hoisting, indexing) is LOWERING's job.
||| - Each macro must match one specific English phrase shape — that
|||   identity is exactly what makes frames work. The macro layer is plain
|||   lowercase definitions, one per phrase.
||| - Top-level constructors are minted only for what cannot feasibly be
|||   de-macroed further (`HasType`-grade primitives). Anything sayable as
|||   a definition over smaller primitives is a definition.
||| - No default arguments. An argument a phrase has, the term spells;
|||   leaving one out is a chapter-scope decision (noted as such), never a
|||   `{default …}` implicit.
|||
||| Chapter one, the binding spine — findings (each backed by a positive
||| or a `failing` negative below):
|||
||| 1. **In-situ nominals under linear threading.** Every argument
|||    position is typed in the discourse context of everything textually
|||    earlier (`nomIntro`/`amtIntro`/`effIntro` fold mentions left to
|||    right), so the term IS the sentence shape. One binding context, one
|||    read discipline; announcement [CR#601.2c] vs resolution [CR#608.2d]
|||    is a `When` INDEX on the binding, not a structural split.
||| 2. **Hoisting is what created the fight problem.** With all slots
|||    lifted to one prenex list, same-sort slots forced positional index
|||    reads past the anaphor uniqueness gate. In situ, Rabid Bite's "its"
|||    (`rabidBite`) is read at a point where only slot A precedes — it
|||    resolves by plain uniqueness, the way the English does — while a
|||    genuinely ambiguous pronoun is still refused (`badIt`), and a
|||    same-sort second slot is just another position (`preyUpon`,
|||    [CR#701.14a]).
||| 3. **"Other" is a modifier with a presupposition.** Oracle's "any
|||    other target" ([CR#115.4]) is the modifier `Other` inside one
|||    nominal's flat modifier set — distinct-from-every-earlier-announced
|||    mention, with an at-least-one-antecedent obligation. Forward and
|||    self references are UNSPELLABLE (`badOther`): textual precedence
|||    replaces the old sibling-index `Distinct` constructor and its range
|||    gate, and the strictly-earlier direction of
|||    [[idris-distinct-position-proof]] holds by construction.
|||    Unconstrained slots still legally share an object
|||    ([CR#601.2c,115.3] — one choice per instance), which is why the
|||    edge is opt-in predicate content, not a default.
||| 4. **A boundary is a filter on the temporal index.** "…at the
|||    beginning of the next end step" stays attached to its clause as
|||    English attaches it; `Delayed` marks the clause future, and its
|||    body's context keeps precisely the `survivors` — resolution-chosen
|||    particular objects persist for the delayed trigger ([CR#603.7c],
|||    `throughTheBreach`), announced slots do not (the trigger announces
|||    its own targets when it goes on the stack, [CR#603.3d]; `badStale`).
|||
||| Not settled in this chapter: the kind union ("any target" spans
||| objects and players [CR#115.4,115.1] — elided to `Object`); sorts and
||| head nouns for `That creature`-style sorted anaphors (`It` stands in);
||| threading of mentions introduced INSIDE predicates ("…an opponent
||| controls. That player…"); zones and event queries (`Put`/`Delayed`
||| carry neither yet); plural reads ("they") beyond the plurality guard;
||| the `May` decider; coordination ellipsis (Arc Trail's shared verb is
||| spelling's business — here it is a clause sequence); and the openness
||| of `When` (as-enters and trigger-time choices look like further index
||| values, not new machinery).
module Experimental

%default total

-- ===== Vocabulary =====

||| Card types, as catalog atoms ([CR#205.2a]; only what chapter one
||| needs — the real set is an open catalog, not an engine enum).
public export
data CardType = Creature

||| What a binding can bind ([CR#115.1] — targets are objects and/or
||| players; the union kind is deferred with the sort lattice).
public export
data Kind = Object | Player

||| Singular mention or group mention — the guard that keeps "it" from
||| resolving to a plural antecedent.
public export
data Plurality = OneOf | ManyOf

||| When a binding's value is chosen: at announcement, as the spell or
||| ability is put on the stack ([CR#601.2c,603.3d]), or during
||| resolution ([CR#608.2d]).
public export
data When = AtAnnounce | AtResolve

||| One discourse mention: when its value is chosen, what kind of thing,
||| singular or group.
public export
record Binding where
  constructor MkBinding
  when : When
  kind : Kind
  plur : Plurality

||| The one context: a nearest-first list of mentions.
public export
Bindings : Type
Bindings = List Binding

||| Singular mentions of a kind, counted — the pronoun read's obligation
||| is `= 1`: zero is an unbound anaphor, two an ambiguous one (the
||| uniqueness gate the controlled language relies on).
public export
countOnes : Kind -> Bindings -> Nat
countOnes k [] = Z
countOnes Object (MkBinding _ Object OneOf :: bs) = S (countOnes Object bs)
countOnes Player (MkBinding _ Player OneOf :: bs) = S (countOnes Player bs)
countOnes k (_ :: bs) = countOnes k bs

||| Is any announced mention of this kind in scope? — the presupposition
||| of the modifier "other".
public export
announcedAny : Kind -> Bindings -> Bool
announcedAny k [] = False
announcedAny Object (MkBinding AtAnnounce Object _ :: bs) = True
announcedAny Player (MkBinding AtAnnounce Player _ :: bs) = True
announcedAny k (_ :: bs) = announcedAny k bs

||| A future clause's context ([CR#603.7c]): resolution-chosen bindings
||| (a produced or chosen particular object) survive; announced slots do
||| not — the delayed trigger announces its own ([CR#603.3d]).
public export
survivors : Bindings -> Bindings
survivors [] = []
survivors (MkBinding AtAnnounce _ _ :: bs) = survivors bs
survivors (MkBinding AtResolve k p :: bs) = MkBinding AtResolve k p :: survivors bs

-- ===== The grammar (mutual: types thread contexts through VALUES) =====

mutual
  ||| An object/player criteria set — the nominal phrase's modifier list,
  ||| FLAT: head noun and relative clauses are sibling constraints on one
  ||| referent, exactly as parsed (no rearrangement to figure out).
  public export
  data Pred : Bindings -> Kind -> Type where
    HasType : CardType -> Pred bs Object                -- head noun "creature"/…
    ControlledBy : Nominal bs Player -> Pred bs Object  -- zero relative "[player] controls"
    Opponent : Pred bs Player                           -- head noun "opponent" (of You — team form [CR#102.3] deferred)
    And : List (Pred bs k) -> Pred bs k                 -- sibling modifiers, one referent
    Not : Pred bs k -> Pred bs k                        -- "don't"/"non-" on a modifier
    -- the modifier "other" ([CR#115.4]): distinct from every earlier
    -- announced mention of this kind; presupposes one exists.
    Other : {auto 0 ok : announcedAny k bs = True} -> Pred bs k
    -- "any target" ([CR#115.4]: creature, player, planeswalker, or
    -- battle). NOT yet de-macroable: needs `Or` and the object/player
    -- kind join (the verifier's `anyTarget` macro shows the target
    -- shape); primitive only until this chapter grows those.
    AnyTarget : Pred bs Object

  ||| A nominal in its argument position — the determiner layer of the
  ||| phrase, deciding how (and whether) the referent enters the
  ||| discourse.
  public export
  data Nominal : Bindings -> Kind -> Type where
    This : Nominal bs Object    -- the source, by self-name or "this …" [CR#113.7]
    You : Nominal bs Player     -- "you" [CR#109.5]
    Target : Pred bs k -> Nominal bs k   -- "target …" / "any target": announced [CR#601.2c]
    Each : Pred bs k -> Nominal bs k     -- "each …": a group, resolution-time [CR#608.2]
    A : Pred bs k -> Nominal bs k        -- "a …": indefinite choice/product [CR#608.2d,400.7]
    -- "it" / "its": the anaphor — exactly one singular Object mention
    -- may precede. Zero = unbound, two = ambiguous; both unspellable.
    It : {auto 0 ok : countOnes Object bs = 1} -> Nominal bs Object

  ||| What a nominal contributes to the discourse that follows it.
  public export
  nomIntro : {bs : Bindings} -> {k : Kind} -> Nominal bs k -> Bindings
  nomIntro This = bs
  nomIntro You = bs
  nomIntro (Target p) = MkBinding AtAnnounce k OneOf :: bs
  nomIntro (Each p) = MkBinding AtResolve k ManyOf :: bs
  nomIntro (A p) = MkBinding AtResolve k OneOf :: bs
  nomIntro It = bs

  ||| An amount expression — where "equal to its power" lives, so it
  ||| threads like everything else.
  public export
  data Amount : Bindings -> Type where
    Lit : Nat -> Amount bs
    PowerOf : Nominal bs Object -> Amount bs   -- "[its/…] power"

  public export
  amtIntro : {bs : Bindings} -> Amount bs -> Bindings
  amtIntro (Lit n) = bs
  amtIntro (PowerOf nom) = nomIntro nom

  ||| Clauses. Constructor argument order IS textual order, and each
  ||| argument is typed in the context its predecessors built — the
  ||| telescope is the whole term, not a special clause-list feature.
  public export
  data Eff : Bindings -> Type where
    -- "[src] deals [amt] damage to [to]"
    DealDamage : {k : Kind} -> (src : Nominal bs Object) -> (amt : Amount (nomIntro src)) ->
                 (to : Nominal (amtIntro amt) k) -> Eff bs
    -- "[a] fights [b]" ([CR#701.14a]). De-macroable in principle to the
    -- mutual-damage expansion; the reciprocal "the other" is a later
    -- chapter, so it stays primitive for now.
    Fights : (a : Nominal bs Object) -> (b : Nominal (nomIntro a) Object) -> Eff bs
    Destroy : Nominal bs Object -> Eff bs      -- "destroy [n]" [CR#701.8a]
    Sacrifice : Nominal bs Object -> Eff bs    -- "sacrifice [n]" [CR#701.21a]
    Put : Nominal bs Object -> Eff bs          -- "put [n] onto the battlefield" (zones elided)
    GainsHaste : Nominal bs Object -> Eff bs   -- "[n] gains haste" (the ability grammar is a later chapter)
    May : Eff bs -> Eff bs                     -- "you may [e]" (decider elided)
    -- sentence/clause sequence: the discourse advances left to right.
    AndThen : (e1 : Eff bs) -> (e2 : Eff (effIntro e1)) -> Eff bs
    -- "[e] at the beginning of the next end step" — the temporal
    -- adverbial stays on its clause; the body sees only `survivors`
    -- ([CR#603.7c,603.3d]). Event queries are a later chapter.
    Delayed : Eff (survivors bs) -> Eff bs

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Eff bs -> Bindings
  effIntro (DealDamage src amt to) = nomIntro to
  effIntro (Fights a b) = nomIntro b
  effIntro (Destroy n) = nomIntro n
  effIntro (Sacrifice n) = nomIntro n
  effIntro (Put n) = nomIntro n
  effIntro (GainsHaste n) = nomIntro n
  effIntro (May e) = effIntro e              -- a declined May skips at runtime, not in scope
  effIntro (AndThen e1 e2) = effIntro e2
  effIntro (Delayed e) = bs                  -- a future clause mentions nothing NOW

-- ===== The macro layer: one definition per English phrase shape =====

-- "creature"
public export
creature : Pred bs Object
creature = HasType Creature

-- "creature you control"
public export
creatureYouControl : Pred bs Object
creatureYouControl = And [creature, ControlledBy You]

-- "creature you don't control"
public export
creatureYouDontControl : Pred bs Object
creatureYouDontControl = And [creature, Not (ControlledBy You)]

-- "an opponent"
public export
anOpponent : Nominal bs Player
anOpponent = A Opponent

-- "any other target" — the macro CARRIES its phrase's presupposition
-- (an earlier announced mention) in its type.
public export
anyOtherTarget : {auto 0 ok : announcedAny Object bs = True} -> Pred bs Object
anyOtherTarget = And [AnyTarget, Other]

-- ===== Positives (must typecheck) =====

-- "Lightning Bolt deals 3 damage to any target."
bolt : Eff []
bolt = DealDamage This (Lit 3) (Target AnyTarget)

-- The design discussion's normative sketch:
-- "… deals 3 damage to each creature an opponent controls."
eachSweep : Eff []
eachSweep = DealDamage This (Lit 3) (Each (And [creature, ControlledBy anOpponent]))

-- "Target creature you control deals damage equal to its power to target
-- creature you don't control." — THE in-situ dividend: "its" is read
-- where only slot A precedes, so plain uniqueness resolves it; slot B
-- doesn't exist yet. Hoisted, this exact card needed positional reads.
rabidBite : Eff []
rabidBite = DealDamage (Target creatureYouControl)
                       (PowerOf It)
                       (Target creatureYouDontControl)

-- "Target creature you control fights target creature you don't
-- control." — two same-sort slots are just two argument positions.
preyUpon : Eff []
preyUpon = Fights (Target creatureYouControl) (Target creatureYouDontControl)

-- "Arc Trail deals 2 damage to any target and 1 damage to any other
-- target." — "other" reaches back across the clause boundary; no index,
-- no Distinct.
arcTrail : Eff []
arcTrail = AndThen (DealDamage This (Lit 2) (Target AnyTarget))
                   (DealDamage This (Lit 1) (Target anyOtherTarget))

-- "You may put a creature card from your hand onto the battlefield. That
-- creature gains haste. Sacrifice that creature at the beginning of the
-- next end step." (card/hand elided) — the indefinite's particular
-- object outlives its sentence AND the delay [CR#603.7c].
throughTheBreach : Eff []
throughTheBreach = AndThen (May (Put (A creature)))
                           (AndThen (GainsHaste It) (Delayed (Sacrifice It)))

-- ===== Negatives (each `failing` block must NOT typecheck) =====

-- "other" with nothing announced before it: the presupposition has no
-- witness. Forward and self references are unspellable the same way —
-- there is no context in which a later mention precedes.
failing "announcedAny"
  badOther : Eff []
  badOther = DealDamage This (Lit 1) (Target anyOtherTarget)

-- A genuinely ambiguous pronoun: two singular Object mentions precede
-- "it", so the uniqueness gate refuses. (Not oracle-legal text — which
-- is the point: the controlled language never writes this.)
failing "countOnes"
  badIt : Eff []
  badIt = AndThen (Fights (Target creature) (Target creature)) (Destroy It)

-- A group is not a singular antecedent: "each creature … it" has no
-- referent for "it" (the plurality guard; "they" is a later chapter).
failing "countOnes"
  badTheyIt : Eff []
  badTheyIt = AndThen (DealDamage This (Lit 3) (Each creature)) (Destroy It)

-- An announced target does not survive into a delayed clause: it is not
-- among the `survivors` ([CR#603.7c]); the delayed trigger announces its
-- own targets when it goes on the stack ([CR#603.3d]).
failing "survivors"
  badStale : Eff []
  badStale = AndThen (Destroy (Target creature)) (Delayed (Sacrifice It))
