||| The semantics-target workbench: what shape should the semantics layer
||| be, sitting between English and core? This module is DELIBERATELY
||| divorced from `Semantics` — no import, no reuse of its types — because
||| the verifier is what the original reasoning tool grew into under
||| direct-runnability pressure, and the destination has to be designable
||| without that baggage. Nothing here is emitted, mirrored, or run; the
||| typechecker is the only consumer. Grow the target shape in this file;
||| the eventual macro/card rewrite aims at what accumulates here.
||| (`Bridge` holds worked new-form ⇄ verifier-form pairs — the seed of the
||| lowering changes that accompany that rewrite; parked out of the build
||| for now so chapters here don't drag the old grammar along.)
|||
||| The draft contract is `docs/decisions/semantics-v2.md` — the layer
||| boundary, the context-data rule, vocabulary, the constructor/macro
||| boundary, and the macro discipline live THERE; this module is its
||| evidence bench and does not restate it.
|||
||| Chapter one, the binding spine — findings (each backed by a positive
||| or a `failing` negative below):
|||
||| 1. **In-situ nouns under linear threading.** Every argument position
|||    is typed in the discourse context of everything textually earlier
|||    (`nomIntro`/`amtIntro`/`effIntro` fold mentions left to right), so
|||    the term IS the sentence shape. One binding context, one read
|||    discipline — the announce list is gone.
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
|||    noun's flat modifier set — distinct-from-every-earlier-target, with
|||    an at-least-one-antecedent obligation (`anyTargeted`). Forward and
|||    self references are UNSPELLABLE (`badOther`): textual precedence
|||    replaces the old sibling-index `Distinct` constructor and its range
|||    gate. Unconstrained slots still legally share an object
|||    ([CR#601.2c,115.3] — one choice per instance), which is why the
|||    edge is opt-in predicate content, not a default.
||| 4. **A boundary is a filter derived from the determiner.** "…at the
|||    beginning of the next end step" stays attached to its clause as
|||    English attaches it; `Delayed` marks the clause future, and its
|||    body's context keeps precisely the `survivors` — choice-determined
|||    particular objects persist for the delayed trigger ([CR#603.7c],
|||    `sneakAttack`), target-determined ones do not (the trigger
|||    announces its own targets when it goes on the stack, [CR#603.3d];
|||    `badStale`). No timing tag is stored: staleness is a function of
|||    the determiner the author wrote.
|||
||| Chapter two, anaphora and carriers (category inventory:
||| oracle-style-guide.md §"Names, self-reference, pronouns, and anaphora"
||| and §"Describing objects, players, and targets"):
|||
||| 5. **The carrier is derived, never stored.** A binding records phrase
|||    projections (determiner, kind, plurality, head type) plus one piece
|||    of fold-state — the referent's current zone, which `Move` retags in
|||    place. The carrier word is a FUNCTION of those ([CR#109.2,110.1]):
|||    Cloudshift's exile makes "that card" resolve and "that creature"
|||    unspellable (`cloudshift`, `badStaleCarrier`), and the return trip
|||    restores the typed battlefield noun for free because the head type
|||    was never lost (`sneakAttack`'s "that creature" after the
|||    hand-to-battlefield move).
||| 6. **Reads are strict-unique after their filter.** `It`/`They` demand
|||    exactly one kind-compatible singular antecedent; `That c` demands
|||    exactly one after the carrier filter. No nearest-wins tiebreak
|||    exists at this layer — the guide's "repeat a noun instead of
|||    stacking ambiguous pronouns" IS the type discipline.
||| 7. **Relational nouns introduce; re-mentions don't.** "its controller"
|||    is `ControllerOf It` — a new Player referent enters the discourse
|||    (readable as "that player"), while pronoun/demonstrative reads add
|||    no binding, so a re-mentioned referent never becomes its own
|||    ambiguity (`bitterDownfall`, `dealsThenDiscards`).
||| 8. **Keyword actions are Composite-tagged macros.** `destroy` mirrors
|||    `plugins/builtin/macros/action/Destroy.ron` — the tag deontics key
|||    on plus the `Move` body [CR#701.8a,701.8b]; `sacrifice` is spelled
|||    the same way and recorded as a core whittling candidate
|||    [CR#701.21a]; `exile` is speculative pending its real macro. The
|||    retag concentrates in `Move`, so every zone-change verb inherits
|||    carrier tracking through its body (`bitterDownfall`'s "its
|||    controller" still resolves after the destroy).
|||
||| Not settled yet: the kind union ("any target" spans objects and
||| players [CR#115.4,115.1] — elided to `Object`); threading of mentions
||| introduced INSIDE predicates ("…an opponent controls. That player…" —
||| `ControlledBy (A Opponent)` does not yet fold its inner mention);
||| subject/agent threading for declarative verbs (the `May` decider,
||| `sacrifice`'s agent, `discardsACard`'s owner linkage — one clause-
||| structure question, which also owns cost-introduced mentions:
||| "Sacrifice this artifact: It deals 2 damage to any target" (Pyrite
||| Spellbomb) reads the cost's mention as the pronoun's antecedent); zone ownership ("your hand", "its owner's hand")
||| and event queries (`Delayed` carries none); plural reads beyond the
||| singular player "they" ("those creatures", "them", "the rest");
||| duration and the continuous-effect grammar (`Gain` elides "until end
||| of turn"); last-known-information reads (`bitterDownfall`'s "its
||| controller" resolves after the destroy because reads ignore zone —
||| deliberate, but [CR#109.4] gives off-battlefield objects no
||| controller, so the LKI story belongs to the ability layer);
||| simultaneity (blocks de-macroing `Fights`); Token / Spell
||| / stack-object / Amount carriers ("that much"); "the chosen [quality]"
||| (quality-kind bindings); sorted self-reference ("this creature" — the
||| carrier of `This` is unverified); and coordination ellipsis (Arc
||| Trail's shared verb is spelling's business — here it is a clause
||| sequence). The context-as-phrase-telescope collapse (bindings storing
||| the mention terms themselves, every projection computed) stays open as
||| a possible later simplification.
module Experimental

%default total

-- ===== Vocabulary =====

||| Card types, as catalog atoms ([CR#205.2a]; only what the chapters
||| need — the real set is an open catalog, not an engine enum).
public export
data CardType = Creature

||| What a binding can bind ([CR#115.1] — targets are objects and/or
||| players; the union kind is deferred with the carrier lattice).
public export
data Kind = Object | Player

||| Singular mention or group mention — the guard that keeps "it" from
||| resolving to a plural antecedent.
public export
data Plurality = OneOf | ManyOf

||| The introducing word of a mention — a SURFACE projection ("target",
||| "a", "each", or a definite/derived mention). Rules facts (delayed
||| staleness, the "other" presupposition) are functions of it, never
||| stored alongside it.
public export
data Determiner = TargetD | AD | EachD | TheD

||| Zones, minimally ([CR#400.1] family; only what movement needs so
||| far — ownership of hands/graveyards is a later chapter).
public export
data Zone = Battlefield | Graveyard | Exile | Hand

||| The carrier word — the noun a referent currently answers to
||| ([CR#109.2,110.1]): a typed or untyped battlefield permanent, a card
||| in a non-battlefield zone, or a player. Token, spell, and stack-object
||| carriers are later chapters.
public export
data Carrier = PlayerC | Perm CardType | AnyPerm | CardC

||| One discourse mention: its determiner, kind, plurality, projected
||| head type, and current zone (the ONE piece of fold-state — `Move`
||| updates it; everything else is a projection of the phrase).
public export
record Binding where
  constructor MkBinding
  det : Determiner
  kind : Kind
  plur : Plurality
  ty : Maybe CardType
  zone : Maybe Zone

||| The one context: a nearest-first list of mentions.
public export
Bindings : Type
Bindings = List Binding

||| The carrier is DERIVED: kind and projected type plus current zone
||| ([CR#110.1] — a permanent is a card or token on the battlefield, and
||| stops being one when it leaves).
public export
carrier : Binding -> Carrier
carrier (MkBinding _ Player _ _ _) = PlayerC
carrier (MkBinding _ Object _ ty (Just Battlefield)) = maybe AnyPerm Perm ty
carrier (MkBinding _ Object _ ty (Just _)) = CardC
carrier (MkBinding _ Object _ ty Nothing) = AnyPerm

public export
sameCT : CardType -> CardType -> Bool
sameCT Creature Creature = True

||| Which carrier nouns a wanted carrier reaches: exact for typed nouns
||| ([CR#205.2a]), "permanent" reaches any battlefield noun
||| ([CR#110.1]), "card" only a non-battlefield object ([CR#108.2]).
public export
compatC : (want : Carrier) -> (have : Carrier) -> Bool
compatC PlayerC PlayerC = True
compatC (Perm a) (Perm b) = sameCT a b
compatC AnyPerm (Perm _) = True
compatC AnyPerm AnyPerm = True
compatC CardC CardC = True
compatC _ _ = False

public export
kindOfC : Carrier -> Kind
kindOfC PlayerC = Player
kindOfC _ = Object

||| Singular mentions of a kind, counted — the wildcard pronoun's
||| obligation is `= 1`: zero is an unbound anaphor, two an ambiguous one
||| (the uniqueness gate the controlled language relies on).
public export
countOnes : Kind -> Bindings -> Nat
countOnes k [] = Z
countOnes Object (MkBinding _ Object OneOf _ _ :: bs) = S (countOnes Object bs)
countOnes Player (MkBinding _ Player OneOf _ _ :: bs) = S (countOnes Player bs)
countOnes k (_ :: bs) = countOnes k bs

||| Singular mentions whose CURRENT carrier the wanted carrier reaches —
||| the sorted demonstrative's obligation is `= 1` (strict uniqueness
||| after the filter; there is no nearest-wins).
public export
countCarrier : Carrier -> Bindings -> Nat
countCarrier c [] = Z
countCarrier c (b :: bs) =
  case (b.plur, compatC c (carrier b)) of
    (OneOf, True) => S (countCarrier c bs)
    _ => countCarrier c bs

||| Is any target-determined mention of this kind in scope? — the
||| presupposition of the modifier "other" ([CR#115.4]), stated entirely
||| in target vocabulary.
public export
anyTargeted : Kind -> Bindings -> Bool
anyTargeted k [] = False
anyTargeted Object (MkBinding TargetD Object _ _ _ :: bs) = True
anyTargeted Player (MkBinding TargetD Player _ _ _ :: bs) = True
anyTargeted k (_ :: bs) = anyTargeted k bs

||| A future clause's context ([CR#603.7c]): choice-determined bindings
||| (a chosen or produced particular object) survive; target-determined
||| ones do not — the delayed trigger announces its own ([CR#603.3d]).
||| Derived from the determiner; no timing tag exists.
public export
survivors : Bindings -> Bindings
survivors [] = []
survivors (MkBinding TargetD _ _ _ _ :: bs) = survivors bs
survivors (b :: bs) = b :: survivors bs

-- ===== Abilities (the granted-ability vocabulary, minimally) =====

||| Keyword abilities, as macro NAMES mirroring
||| `plugins/builtin/macros/keyword/` — parameterized keywords spell
||| their parameters explicitly (e.g. a from-quality as `Maybe`, written
||| `Nothing` in the plain form), never as defaults.
public export
data Keyword = Haste

public export
data Ability = KeywordAbility Keyword

||| Keyword-action tags ([CR#701]) — core's `Composite` verb names.
||| `Destroy` and `Discard` mirror `plugins/builtin/macros/action/`;
||| `Sacrifice` is a core whittling candidate (see the decision record);
||| `Exile` is speculative pending its real macro definition. (Its own
||| namespace: the tag `Exile` and the zone `Exile` are distinct words.)
namespace Verb
  public export
  data VerbName = Destroy | Sacrifice | Exile | Discard

-- ===== The grammar (mutual: types thread contexts through VALUES) =====

mutual
  ||| An object/player criteria set — the noun phrase's modifier list,
  ||| FLAT: head noun and relative clauses are sibling constraints on one
  ||| referent, exactly as parsed (no rearrangement to figure out).
  public export
  data Predicate : Bindings -> Kind -> Type where
    HasType : CardType -> Predicate bs Object            -- head noun "creature"/…
    AnyPlayer : Predicate bs Player                      -- head noun "player" (any player, [CR#102.1])
    Opponent : Predicate bs Player                       -- head noun "opponent" (of You — team form [CR#102.3] deferred)
    ControlledBy : Noun bs Player -> Predicate bs Object -- zero relative "[player] controls"
    InZone : Zone -> Predicate bs Object                 -- zone clause "in/from [zone]" ([CR#109.2a]; ownership deferred)
    And : List (Predicate bs k) -> Predicate bs k        -- sibling modifiers, one referent
    Not : Predicate bs k -> Predicate bs k               -- "don't"/"non-" on a modifier
    -- the modifier "other"/"another" ([CR#115.4]): distinct from every
    -- earlier target of this kind; presupposes one exists.
    Other : {auto 0 ok : anyTargeted k bs = True} -> Predicate bs k
    -- "any target" ([CR#115.4]: creature, player, planeswalker, or
    -- battle). NOT yet de-macroable: needs `Or` and the object/player
    -- kind join; primitive only until a chapter grows those.
    AnyTarget : Predicate bs Object

  ||| The head type a predicate projects onto its referent — what "that
  ||| creature" remembers across a zone change.
  public export
  seedTy : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe CardType
  seedTy (HasType t) = Just t
  seedTy (And ps) = seedTyAll ps
  seedTy _ = Nothing

  public export
  seedTyAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe CardType
  seedTyAll [] = Nothing
  seedTyAll (p :: ps) = case seedTy p of
    Just t => Just t
    Nothing => seedTyAll ps

  ||| The zone a predicate places its referent in — a bare description
  ||| means the battlefield ([CR#109.2]); a zone clause says otherwise.
  public export
  seedZone : {0 bs : Bindings} -> {0 k : Kind} -> Predicate bs k -> Maybe Zone
  seedZone (InZone z) = Just z
  seedZone (And ps) = seedZoneAll ps
  seedZone _ = Nothing

  public export
  seedZoneAll : {0 bs : Bindings} -> {0 k : Kind} -> List (Predicate bs k) -> Maybe Zone
  seedZoneAll [] = Nothing
  seedZoneAll (p :: ps) = case seedZone p of
    Just z => Just z
    Nothing => seedZoneAll ps

  public export
  zoneOr : Zone -> Maybe Zone -> Zone
  zoneOr z Nothing = z
  zoneOr z (Just w) = w

  ||| Build the binding a determined mention introduces: projections of
  ||| the phrase only.
  public export
  bindFor : Determiner -> Plurality -> {k : Kind} -> Predicate bs k -> Binding
  bindFor det plur {k = Object} p =
    MkBinding det Object plur (seedTy p) (Just (zoneOr Battlefield (seedZone p)))
  bindFor det plur {k = Player} p = MkBinding det Player plur Nothing Nothing

  ||| A noun in its argument position — the determiner layer of the
  ||| phrase, deciding how (and whether) the referent enters the
  ||| discourse.
  public export
  data Noun : Bindings -> Kind -> Type where
    This : Noun bs Object       -- the source, by self-name or "this …" [CR#113.7]
    You : Noun bs Player        -- "you" [CR#109.5]
    Target : Predicate bs k -> Noun bs k  -- "target …" / "any target": announced [CR#601.2c]
    Each : Predicate bs k -> Noun bs k    -- "each …": a group, resolution-time [CR#608.2]
    A : Predicate bs k -> Noun bs k       -- "a …": indefinite choice/product [CR#608.2d,400.7]
    -- "it" / "its": the wildcard pronoun — exactly one singular Object
    -- mention may precede. Zero = unbound, two = ambiguous; both
    -- unspellable.
    It : {auto 0 ok : countOnes Object bs = 1} -> Noun bs Object
    -- "they" for a player (singular; plural groups are a later chapter).
    They : {auto 0 ok : countOnes Player bs = 1} -> Noun bs Player
    -- "that [carrier]": the sorted demonstrative — exactly one mention
    -- whose CURRENT carrier answers to the noun may precede.
    That : (c : Carrier) -> {auto 0 ok : countCarrier c bs = 1} -> Noun bs (kindOfC c)
    -- "[object]'s controller" / "its owner": relational nouns — a NEW
    -- player referent derived from an object mention ([CR#108.3,109.4]).
    ControllerOf : Noun bs Object -> Noun bs Player
    OwnerOf : Noun bs Object -> Noun bs Player

  ||| What a noun contributes to the discourse that follows it.
  ||| Determined mentions introduce a binding; pronoun and demonstrative
  ||| READS introduce nothing (a re-mention is not a new referent);
  ||| relational nouns introduce their derived referent.
  public export
  nomIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  nomIntro This = bs
  nomIntro You = bs
  nomIntro (Target p) = bindFor TargetD OneOf p :: bs
  nomIntro (Each p) = bindFor EachD ManyOf p :: bs
  nomIntro (A p) = bindFor AD OneOf p :: bs
  nomIntro It = bs
  nomIntro They = bs
  nomIntro (That c) = bs
  nomIntro (ControllerOf n) = MkBinding TheD Player OneOf Nothing Nothing :: nomIntro n
  nomIntro (OwnerOf n) = MkBinding TheD Player OneOf Nothing Nothing :: nomIntro n

  ||| An amount expression — where "equal to its power" lives, so it
  ||| threads like everything else.
  public export
  data Amount : Bindings -> Type where
    Lit : Nat -> Amount bs
    PowerOf : Noun bs Object -> Amount bs   -- "[its/…] power"

  public export
  amtIntro : {bs : Bindings} -> Amount bs -> Bindings
  amtIntro (Lit n) = bs
  amtIntro (PowerOf nom) = nomIntro nom

  ||| Life-total change operands ([CR#119.3]; `Set` is a later chapter).
  public export
  data LifeOp : Bindings -> Type where
    Up : Amount bs -> LifeOp bs     -- "gains [amt] life"
    Down : Amount bs -> LifeOp bs   -- "loses [amt] life"

  public export
  lifeIntro : {bs : Bindings} -> LifeOp bs -> Bindings
  lifeIntro (Up a) = amtIntro a
  lifeIntro (Down a) = amtIntro a

  ||| Clauses. Constructor argument order IS textual order, and each
  ||| argument is typed in the context its predecessors built — the
  ||| telescope is the whole term, not a special clause-list feature.
  ||| Constructors are engine-basis members only (see the decision
  ||| record); keyword actions live in the macro layer below.
  public export
  data Effect : Bindings -> Type where
    -- "[src] deals [amt] damage to [to]"
    DealDamage : {k : Kind} -> (src : Noun bs Object) -> (amt : Amount (nomIntro src)) ->
                 (to : Noun (amtIntro amt) k) -> Effect bs
    -- "[a] fights [b]" ([CR#701.14a]). De-macroable in principle to the
    -- mutual-damage expansion; simultaneity is not yet mintable, so it
    -- stays primitive for now (classification unconfirmed).
    Fights : (a : Noun bs Object) -> (b : Noun (nomIntro a) Object) -> Effect bs
    -- "tap [n]" ([CR#701.26a]) — core basis.
    Tap : Noun bs Object -> Effect bs
    -- "[move] [n] [to zone]" — the zone-change primitive every keyword
    -- action's body bottoms out in ([CR#701.8a] shape). Destination
    -- only: the from-zone is the referent's fold-state, which this
    -- clause UPDATES (the retag).
    Move : (what : Noun bs Object) -> (to : Zone) -> Effect bs
    -- "[who] gains/loses [amt] life" ([CR#119.3]) — core basis (merged).
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Effect bs
    -- "[n] gains [ability]" — duration and the continuous-effect grammar
    -- are a later chapter.
    Gain : Noun bs Object -> Ability -> Effect bs
    -- the keyword-action tag ([CR#701]): the named verb deontics and
    -- replacements key on, wrapping its expansion body ([CR#701.8b] —
    -- only a Destroy-tagged move IS a destruction).
    Composite : VerbName -> Effect bs -> Effect bs
    May : Effect bs -> Effect bs               -- "you may [e]" (decider deferred)
    -- sentence/clause sequence: the discourse advances left to right.
    AndThen : (e1 : Effect bs) -> (e2 : Effect (effIntro e1)) -> Effect bs
    -- "[e] at the beginning of the next end step" — the temporal
    -- adverbial stays on its clause; the body sees only `survivors`
    -- ([CR#603.7c,603.3d]). Event queries are a later chapter.
    Delayed : Effect (survivors bs) -> Effect bs

  ||| Retag the binding a moved noun denotes: an introducing noun's own
  ||| fresh binding, or the unique binding a read resolved to (strict
  ||| uniqueness is what makes this well-defined). Player nouns and the
  ||| untracked source pass through.
  public export
  setZone : Zone -> Binding -> Binding
  setZone z b = { zone := Just z } b

  public export
  setZoneHead : Zone -> Bindings -> Bindings
  setZoneHead z [] = []
  setZoneHead z (b :: bs) = setZone z b :: bs

  public export
  setZoneIt : Zone -> Bindings -> Bindings
  setZoneIt z [] = []
  setZoneIt z (MkBinding det Object OneOf ty zn :: bs) =
    MkBinding det Object OneOf ty (Just z) :: bs
  setZoneIt z (b :: bs) = b :: setZoneIt z bs

  public export
  setZoneThat : Carrier -> Zone -> Bindings -> Bindings
  setZoneThat c z [] = []
  setZoneThat c z (b :: bs) =
    case (b.plur, compatC c (carrier b)) of
      (OneOf, True) => setZone z b :: bs
      _ => b :: setZoneThat c z bs

  public export
  moveIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Zone -> Bindings
  moveIntro (Target p) z = setZoneHead z (nomIntro (Target p))
  moveIntro (Each p) z = setZoneHead z (nomIntro (Each p))
  moveIntro (A p) z = setZoneHead z (nomIntro (A p))
  moveIntro It z = setZoneIt z bs
  moveIntro (That c) z = setZoneThat c z bs
  moveIntro This z = bs
  moveIntro You z = bs
  moveIntro They z = bs
  moveIntro (ControllerOf n) z = nomIntro (ControllerOf n)
  moveIntro (OwnerOf n) z = nomIntro (OwnerOf n)

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = nomIntro to
  effIntro (Fights a b) = nomIntro b
  effIntro (Tap n) = nomIntro n
  effIntro (Move what to) = moveIntro what to
  effIntro (ChangeLife who op) = lifeIntro op
  effIntro (Gain n _) = nomIntro n
  effIntro (Composite _ e) = effIntro e
  effIntro (May e) = effIntro e              -- a declined May skips at runtime, not in scope
  effIntro (AndThen e1 e2) = effIntro e2
  effIntro (Delayed e) = bs                  -- a future clause mentions nothing NOW

-- ===== The macro layer: one definition per English phrase shape =====

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
-- the Destroy tag over the battlefield→graveyard move [CR#701.8a].
public export
destroy : Noun bs Object -> Effect bs
destroy n = Composite Destroy (Move n Graveyard)

-- "exile [n]" ([CR#701.13a]) — speculative pending its real macro.
public export
exile : Noun bs Object -> Effect bs
exile n = Composite Exile (Move n Exile)

-- "sacrifice [n]" ([CR#701.21a]) — imperative form; the agent (and the
-- choice-filter conjuncts `InZone Battlefield`/`ControlledBy agent` of
-- the indefinite case) belong to the clause-structure chapter. Core's
-- `Sacrifice` variant is the whittling candidate this expands.
public export
sacrifice : Noun bs Object -> Effect bs
sacrifice n = Composite Sacrifice (Move n Graveyard)

-- "[agent] discards a card" — the hand-zone choice; whose hand, and the
-- agent-as-chooser linkage, are the clause-structure/ownership chapters
-- (the agent argument is the phrase's subject, held for that work).
public export
discardsACard : Noun bs Player -> Effect bs
discardsACard agent = Composite Discard (Move (A (InZone Hand)) Graveyard)

-- "[n] gains haste"
public export
gainsHaste : Noun bs Object -> Effect bs
gainsHaste n = Gain n (KeywordAbility Haste)

-- "[who] loses [amt] life"
public export
losesLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
losesLife who amt = ChangeLife who (Down amt)

-- ===== Positives (must typecheck) =====

-- "Lightning Bolt deals 3 damage to any target."
bolt : Effect []
bolt = DealDamage This (Lit 3) (Target AnyTarget)

-- The design discussion's normative sketch:
-- "… deals 3 damage to each creature an opponent controls."
eachSweep : Effect []
eachSweep = DealDamage This (Lit 3) (Each (And [creature, ControlledBy anOpponent]))

-- "Target creature you control deals damage equal to its power to target
-- creature you don't control." — THE in-situ dividend: "its" is read
-- where only slot A precedes, so plain uniqueness resolves it; slot B
-- doesn't exist yet. Hoisted, this exact card needed positional reads.
rabidBite : Effect []
rabidBite = DealDamage (Target creatureYouControl)
                       (PowerOf It)
                       (Target creatureYouDontControl)

-- "Target creature you control fights target creature you don't
-- control." — two same-sort slots are just two argument positions.
preyUpon : Effect []
preyUpon = Fights (Target creatureYouControl) (Target creatureYouDontControl)

-- "Arc Trail deals 2 damage to any target and 1 damage to any other
-- target." — "other" reaches back across the clause boundary; no index,
-- no Distinct.
arcTrail : Effect []
arcTrail = AndThen (DealDamage This (Lit 2) (Target AnyTarget))
                   (DealDamage This (Lit 1) (Target anyOtherTarget))

-- "Exile target creature you control, then return that card to the
-- battlefield." (Cloudshift; "under your control" elided with zone
-- ownership) — the exile RETAGS the referent's zone, so the carrier the
-- demonstrative must use flips from creature to card mid-sentence
-- ([CR#110.1]), and the return trip is just another Move.
cloudshift : Effect []
cloudshift = AndThen (exile (Target creatureYouControl))
                     (Move (That CardC) Battlefield)

-- "You may put a creature card from your hand onto the battlefield. That
-- creature gains haste. At the beginning of the next end step, sacrifice
-- that creature." (Sneak Attack; per-card condition elided) — the
-- hand-to-battlefield move RESTORES the typed carrier (the head type was
-- projected at introduction, never lost), and the choice-determined
-- referent survives the delay [CR#603.7c] while a target would not.
sneakAttack : Effect []
sneakAttack = AndThen (May (Move (A (And [creature, InZone Hand])) Battlefield))
                      (AndThen (gainsHaste (That (Perm Creature)))
                               (Delayed (sacrifice (That (Perm Creature)))))

-- "Destroy target creature. Its controller loses 2 life." (Bitter
-- Downfall; its cost-reduction line elided) — the relational noun:
-- `ControllerOf It` derives a NEW player referent from the destroyed
-- object (whose "its" still resolves — the retag moved it to the
-- graveyard, it didn't unmention it).
bitterDownfall : Effect []
bitterDownfall = AndThen (destroy (Target creature))
                         (losesLife (ControllerOf It) (Lit 2))

-- "Tap target creature. It deals damage equal to its power to another
-- target creature." (Deadshot) — pronoun as source, and "another" is
-- the same `Other` modifier "any other target" uses.
deadshot : Effect []
deadshot = AndThen (Tap (Target creature))
                   (DealDamage It (PowerOf It) (Target (And [creature, Other])))

-- "… deals 3 damage to target player. That player discards a card." —
-- the sorted demonstrative at Player kind, and a keyword-action macro
-- (Discard) whose body moves a hand-zone choice.
dealsThenDiscards : Effect []
dealsThenDiscards = AndThen (DealDamage This (Lit 3) (Target AnyPlayer))
                            (discardsACard (That PlayerC))

-- Normative sketch for the singular player "they":
-- "Target player loses 1 life. They lose 1 life."
theySketch : Effect []
theySketch = AndThen (losesLife (Target AnyPlayer) (Lit 1))
                     (losesLife They (Lit 1))

-- ===== Negatives (each `failing` block must NOT typecheck) =====

-- "other" with no target before it: the presupposition has no witness.
-- Forward and self references are unspellable the same way — there is
-- no context in which a later mention precedes.
failing "anyTargeted"
  badOther : Effect []
  badOther = DealDamage This (Lit 1) (Target anyOtherTarget)

-- A genuinely ambiguous pronoun: two singular Object mentions precede
-- "it", so the uniqueness gate refuses. (Not oracle-legal text — which
-- is the point: the controlled language never writes this.)
failing "countOnes"
  badIt : Effect []
  badIt = AndThen (Fights (Target creature) (Target creature)) (Tap It)

-- A group is not a singular antecedent: "each creature … it" has no
-- referent for "it" (the plurality guard).
failing "countOnes"
  badTheyIt : Effect []
  badTheyIt = AndThen (DealDamage This (Lit 3) (Each creature)) (Tap It)

-- An announced target does not survive into a delayed clause: it is not
-- among the `survivors` ([CR#603.7c]); the delayed trigger announces its
-- own targets when it goes on the stack ([CR#603.3d]).
failing "survivors"
  badStale : Effect []
  badStale = AndThen (destroy (Target creature)) (Delayed (sacrifice It))

-- After the exile, the referent no longer answers to "creature": its
-- carrier is derived from the RETAGGED zone ([CR#110.1]), so the typed
-- demonstrative has no antecedent — the carrier-word rule as a type
-- error ("that card" is the spelling that resolves; see `cloudshift`).
failing "countCarrier"
  badStaleCarrier : Effect []
  badStaleCarrier = AndThen (exile (Target creatureYouControl))
                            (Move (That (Perm Creature)) Battlefield)
