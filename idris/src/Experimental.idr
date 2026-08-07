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
|||    `throughTheBreach`), target-determined ones do not (the trigger
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
|||    was never lost (`throughTheBreach`'s "that creature" after the
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
|||    ambiguity (`bitterDownfall`, `immersturmSkullcairn`).
||| 8. **Keyword actions are Composite-tagged macros.** `destroy` mirrors
|||    `plugins/builtin/macros/action/Destroy.ron` — the tag deontics key
|||    on plus the `Move` body [CR#701.8a,701.8b]; `sacrifice` is spelled
|||    the same way and recorded as a core whittling candidate
|||    [CR#701.21a]; `exile` is speculative pending its real macro. The
|||    retag concentrates in `Move`, so every zone-change verb inherits
|||    carrier tracking through its body (`bitterDownfall`'s "its
|||    controller" still resolves after the destroy).
|||
||| Chapter three, clause structure (evidence: Diabolic Edict, Innocent
||| Blood, Cry of Contrition, Pyrite Spellbomb, Immersturm Skullcairn;
||| Browbeat/Risk Factor for the decider split):
|||
||| 9. **Subjects are the factored who-slot.** Verbs the CR gives a
|||    player actor ([CR#701.21a,701.9a]) put their performer in clause
|||    position (`Does`), typed before their phrase; the imperative
|||    supplies an explicit `You`; effect-verbs stay subjectless, and an
|||    object source (DealDamage's src) is the verb's own argument. A
|||    dependent context cannot re-use the subject term at each inner
|||    slot the way the real macros ride their agent param, so the slot
|||    factors to the clause and lowering redistributes it.
||| 10. **The may-decider is a slot, not the performer.** `May` names
|||    its decider ([CR#608.2d]; resolving default the controller
|||    [CR#608.2c]) over an arbitrary clause — "[player] may have
|||    [source] deal …" separates decider from performer, killing the
|||    auxiliary-subject reading (`throughTheBreach`'s "You may put").
||| 11. **Choice method is surface data.** "of their choice" / "at
|||    random" are marked indefinites (`ATheirChoice`, `AAtRandom`)
|||    mirroring the real macros' explicit chooser slot and its absence
|||    in the random variant ([CR#701.9b]); no CR rule derives a
|||    chooser, and the corpus has no bare "Target player sacrifices a
|||    creature" (`diabolicEdict`, `innocentBlood`).
||| 12. **The colon is a public-zone filter.** An activated effect reads
|||    its cost's mentions through `publicOnly` ([CR#400.2], current
|||    zone): a tapped cost-mention survives unmoved, the moved sorted
|||    self-reference (`ThisOf`) mints the new object's binding
|||    ([CR#400.7]) that the effect's "It" reads ([CR#400.7j] —
|||    `pyriteSpellbomb`, `immersturmSkullcairn`), a bounce-to-hand is
|||    unreadable past the colon (`badHiddenCost`), and two cost moves
|||    make a bare pronoun ambiguous (`badTwoCostMentions` — real text
|||    switches to definite descriptions there).
||| 13. **Owned zones where English owns them.** `ZoneExpr` spells
|||    "your hand" (`HandOf You`); expansions keep sort-only forms so
|||    the CR's owner-routing ([CR#701.8a,701.9a]) never injects phantom
|||    mentions; and sacrifice demands its referent stand on the
|||    battlefield (`OnBattlefield`, `badSacrificeExiled`) — the zone
|||    half of the verb's implicit restriction, controller half parked.
|||
||| Not settled yet: the kind union ("any target" spans objects and
||| players [CR#115.4,115.1] — elided to `Object`); threading of mentions
||| introduced INSIDE predicates ("…an opponent controls. That player…" —
||| `ControlledBy (A Opponent)` does not yet fold its inner mention);
||| owned-zone mentions beyond `You` ("its owner's hand" — destination
||| and predicate-inner mentions do not fold yet); controller
||| fold-state (the controller half of verb restrictions, entangled
||| with [CR#109.4]); `May`'s if-you-do / if-not branches (the Risk
||| Factor / Rakdos, Patron of Chaos else-clauses); definite
||| descriptive reads ("the sacrificed creature", "the exiled card" —
||| also what `AAtRandom`'s positive waits on: Pyromancy reads "the
||| discarded card"); event queries (`Delayed` carries none); the player pronoun "they"
||| (corpus-attested only inside trigger and unless clauses — Havoc,
||| Tergrid's Lantern — so `They`'s positive waits on those constructions)
||| and plural reads ("those creatures", "them", "the rest");
||| duration and the continuous-effect grammar (`Gain` elides "until end
||| of turn"); last-known-information reads (`bitterDownfall`'s "its
||| controller" resolves after the destroy because reads ignore zone —
||| deliberate, but [CR#109.4] gives off-battlefield objects no
||| controller, so the LKI story belongs to the ability layer);
||| simultaneity (blocks de-macroing `Fights`); Token / Spell
||| / stack-object / Amount carriers ("that much"; bare `This` stays
||| untracked, and "this spell" / "this card" carriers with it); "the
||| chosen [quality]" (quality-kind bindings); and coordination ellipsis (Arc
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
data CardType = Creature | Artifact | Land

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

||| Zone sorts, minimally ([CR#400.1] family) — the fold-state tag a
||| binding carries. Ownership is not stored here; it lives in the
||| surface `ZoneExpr` where English writes it.
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
sameCT Artifact Artifact = True
sameCT Land Land = True
sameCT _ _ = False

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

||| Zone visibility ([CR#400.2] — library and hand are hidden zones).
public export
publicZone : Zone -> Bool
publicZone Battlefield = True
publicZone Graveyard = True
publicZone Exile = True
publicZone Hand = False

||| The cost boundary's filter: a mention a cost leaves in a hidden
||| zone is unreadable past the colon; unmoved mentions (a tapped cost
||| creature) and publicly-moved ones survive. [CR#400.7] fires only on
||| a zone change and [CR#400.7j] is its public-zone exception, so the
||| filter keys on the CURRENT zone, not on having moved. Players and
||| untracked mentions pass.
public export
publicOnly : Bindings -> Bindings
publicOnly [] = []
publicOnly (b :: bs) =
  case b.zone of
    Just z => if publicZone z then b :: publicOnly bs else publicOnly bs
    Nothing => b :: publicOnly bs

||| The current zone of the wildcard pronoun's referent — the unique
||| singular object mention (uniqueness is `It`'s own gate).
public export
zoneOfIt : Bindings -> Maybe Zone
zoneOfIt [] = Nothing
zoneOfIt (MkBinding det Object OneOf ty zn :: bs) = zn
zoneOfIt (b :: bs) = zoneOfIt bs

||| The current zone of a sorted demonstrative's referent.
public export
zoneOfThat : Carrier -> Bindings -> Maybe Zone
zoneOfThat c [] = Nothing
zoneOfThat c (b :: bs) =
  case (b.plur, compatC c (carrier b)) of
    (OneOf, True) => b.zone
    _ => zoneOfThat c bs

||| The zone half of sacrifice's implicit restriction ([CR#701.21a] —
||| only a permanent can be sacrificed): the referent's tracked zone
||| must be the battlefield, or untracked. The controller half needs
||| fold-state the context does not carry (not-settled).
public export
data OnBattlefield : Maybe Zone -> Type where
  Untracked : OnBattlefield Nothing
  OnField : OnBattlefield (Just Battlefield)

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
  ||| A zone as English writes it: the shared zones bare ([CR#400.1] —
  ||| battlefield and exile are shared), the per-player zones either
  ||| owned ("your hand", "its owner's hand") or bare sort-only — the
  ||| bare forms are for macro expansions whose English wrote no owner
  ||| (the CR routes those per-object, e.g. destroy's "its owner's
  ||| graveyard" [CR#701.8a], without the card text mentioning the owner
  ||| — an owned expansion form would inject a phantom mention into the
  ||| discourse). Mentions inside a destination expression do not yet
  ||| enter the discourse (no current positive writes one).
  public export
  data ZoneExpr : Bindings -> Type where
    BattlefieldZ : ZoneExpr bs
    ExileZ : ZoneExpr bs
    HandZ : ZoneExpr bs
    GraveyardZ : ZoneExpr bs
    HandOf : Noun bs Player -> ZoneExpr bs
    GraveyardOf : Noun bs Player -> ZoneExpr bs

  ||| The sort a zone expression names — what fold-state records.
  public export
  zoneSort : ZoneExpr bs -> Zone
  zoneSort BattlefieldZ = Battlefield
  zoneSort ExileZ = Exile
  zoneSort HandZ = Hand
  zoneSort GraveyardZ = Graveyard
  zoneSort (HandOf _) = Hand
  zoneSort (GraveyardOf _) = Graveyard

  ||| An object/player criteria set — the noun phrase's modifier list,
  ||| FLAT: head noun and relative clauses are sibling constraints on one
  ||| referent, exactly as parsed (no rearrangement to figure out).
  public export
  data Predicate : Bindings -> Kind -> Type where
    HasType : CardType -> Predicate bs Object            -- head noun "creature"/…
    AnyPlayer : Predicate bs Player                      -- head noun "player" (any player, [CR#102.1])
    Opponent : Predicate bs Player                       -- head noun "opponent" (of You — team form [CR#102.3] deferred)
    ControlledBy : Noun bs Player -> Predicate bs Object -- zero relative "[player] controls"
    InZone : ZoneExpr bs -> Predicate bs Object          -- zone clause "in/from [zone]" ([CR#109.2a])
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
  seedZone (InZone z) = Just (zoneSort z)
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
    This : Noun bs Object       -- the source, by self-name or "this spell" [CR#113.7]
    -- the sorted self-reference "this artifact"/"this land"/"this
    -- creature": the source under its type noun. Unmoved it introduces
    -- nothing (like `This`); MOVED it mints a fresh binding — the move
    -- makes it a new object [CR#400.7], which is why cost-position
    -- "Sacrifice this artifact" leaves a referent the effect's "It" can
    -- read ([CR#400.7j] is the exception letting the effect find it).
    ThisOf : CardType -> Noun bs Object
    You : Noun bs Player        -- "you" [CR#109.5]
    Target : Predicate bs k -> Noun bs k  -- "target …" / "any target": announced [CR#601.2c]
    Each : Predicate bs k -> Noun bs k    -- "each …": a group, resolution-time [CR#608.2]
    A : Predicate bs k -> Noun bs k       -- "a …": indefinite choice/product [CR#608.2d,400.7]
    -- "a … of their choice" / "a … at random": the indefinite with its
    -- choice method marked in the text — chooser and method are surface
    -- facts (the guide's chooser marking; a random discard has no
    -- chooser [CR#701.9b]), mirroring the real macros' explicit `by`
    -- slot and its absence in the at-random variant.
    ATheirChoice : Predicate bs k -> Noun bs k
    AAtRandom : Predicate bs k -> Noun bs k
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
  nomIntro (ThisOf t) = bs
  nomIntro You = bs
  nomIntro (Target p) = bindFor TargetD OneOf p :: bs
  nomIntro (Each p) = bindFor EachD ManyOf p :: bs
  nomIntro (A p) = bindFor AD OneOf p :: bs
  nomIntro (ATheirChoice p) = bindFor AD OneOf p :: bs
  nomIntro (AAtRandom p) = bindFor AD OneOf p :: bs
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
    Move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) -> Effect bs
    -- "[who] gains/loses [amt] life" ([CR#119.3]) — core basis (merged).
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Effect bs
    -- "[n] gains [ability]" — duration and the continuous-effect grammar
    -- are a later chapter.
    Gain : Noun bs Object -> Ability -> Effect bs
    -- the keyword-action tag ([CR#701]): the named verb deontics and
    -- replacements key on, wrapping its expansion body ([CR#701.8b] —
    -- only a Destroy-tagged move IS a destruction).
    Composite : VerbName -> Effect bs -> Effect bs
    -- "[subject] [verb phrase]" — the declarative clause: an agentive
    -- verb's performer in subject position, its phrase typed after it.
    -- Only verbs the CR gives a player actor take a subject
    -- ([CR#701.21a,701.9a]-family); effect-verbs (destroy, damage) stay
    -- subjectless imperatives, and the imperative of an agentive verb
    -- supplies its unpronounced subject as an explicit `You`. This is
    -- core's per-verb `who` slot factored to clause position — a
    -- dependent context can't re-use the subject term at each inner
    -- slot the way the real macros ride their agent param — and
    -- lowering redistributes it; `ChangeLife` carries its `who` the
    -- same way. Object sources (DealDamage's src) are the verb's own
    -- argument, not a subject.
    Does : (subj : Noun bs Player) -> Effect (nomIntro subj) -> Effect bs
    -- "[decider] may [effect]" — the decider slot ([CR#608.2d]; the
    -- resolving default is the controller [CR#608.2c]). Decider and
    -- performer can differ ("[player] may have [source] deal … to
    -- them"), so the body is any clause, not the decider's own verb
    -- phrase; if-you-do/if-not branches are later growth.
    May : (decider : Noun bs Player) -> Effect (nomIntro decider) -> Effect bs
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
  moveIntro (ATheirChoice p) z = setZoneHead z (nomIntro (ATheirChoice p))
  moveIntro (AAtRandom p) z = setZoneHead z (nomIntro (AAtRandom p))
  moveIntro It z = setZoneIt z bs
  moveIntro (That c) z = setZoneThat c z bs
  moveIntro This z = bs
  -- a moved sorted self-reference mints the new object's binding
  -- ([CR#400.7]; see the constructor comment).
  moveIntro (ThisOf t) z = MkBinding TheD Object OneOf (Just t) (Just z) :: bs
  moveIntro You z = bs
  moveIntro They z = bs
  moveIntro (ControllerOf n) z = nomIntro (ControllerOf n)
  moveIntro (OwnerOf n) z = nomIntro (OwnerOf n)

  ||| The zone a noun's referent currently occupies, if tracked: reads
  ||| consult their unique binding, introducers their seed zone
  ||| ([CR#109.2] — a bare description means the battlefield), the
  ||| source and player nouns are untracked.
  public export
  nounZone : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Maybe Zone
  nounZone This = Nothing
  nounZone (ThisOf t) = Nothing
  nounZone You = Nothing
  nounZone (Target p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (Each p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (A p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (ATheirChoice p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (AAtRandom p) = Just (zoneOr Battlefield (seedZone p))
  nounZone It = zoneOfIt bs
  nounZone They = Nothing
  nounZone (That c) = zoneOfThat c bs
  nounZone (ControllerOf n) = Nothing
  nounZone (OwnerOf n) = Nothing

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = nomIntro to
  effIntro (Fights a b) = nomIntro b
  effIntro (Tap n) = nomIntro n
  effIntro (Move what to) = moveIntro what (zoneSort to)
  effIntro (ChangeLife who op) = lifeIntro op
  effIntro (Gain n _) = nomIntro n
  effIntro (Composite _ e) = effIntro e
  effIntro (Does s e) = effIntro e
  effIntro (May d e) = effIntro e            -- a declined May skips at runtime, not in scope
  effIntro (AndThen e1 e2) = effIntro e2
  effIntro (Delayed e) = bs                  -- a future clause mentions nothing NOW

-- ===== The activated-ability juncture =====

||| "[cost]: [effect]" ([CR#602.1]) — just the colon: the cost's
||| object-moving/tapping component as an ordinary clause, the effect
||| typed in the cost's public-zone survivors (`publicOnly`). Mana,
||| {T}, and activation instructions are elided the way positives elide
||| rider lines; the full ability layer stays parked.
public export
data Activated : Bindings -> Type where
  MkActivated : (cost : Effect bs) -> Effect (publicOnly (effIntro cost)) -> Activated bs

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

-- "an opponent" (held for the predicate-inner-mention chapter)
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
destroy n = Composite Destroy (Move n GraveyardZ)

-- "exile [n]" ([CR#701.13a]) — speculative pending its real macro.
public export
exile : Noun bs Object -> Effect bs
exile n = Composite Exile (Move n ExileZ)

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
sacrifice agent n = Does agent (Composite Sacrifice (Move n GraveyardZ))

-- "[agent] discard(s) a card" — the hand→graveyard move [CR#701.9a]
-- with the subject in clause position. The CR routes by the card's
-- OWNER; owner≡agent is this macro's elision, the same one the real
-- macro makes with its agent-param hand filter. The discarded card is
-- the affected player's choice by default [CR#701.9b], spelled
-- sort-only here (an owned-hand expansion needs a subject-read noun
-- the vocabulary lacks — not-settled).
public export
discardsACard : (agent : Noun bs Player) -> Effect bs
discardsACard agent = Does agent (Composite Discard (Move (A (InZone HandZ)) GraveyardZ))

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

-- "Barrage of Boulders deals 1 damage to each creature you don't
-- control." (Ferocious rider line elided) — the corpus has no "each
-- creature an opponent controls": opponent-scoped sweeps say "you don't
-- control" or plural "your opponents control" (a later chapter's noun).
barrageOfBoulders : Effect []
barrageOfBoulders = DealDamage This (Lit 1) (Each creatureYouDontControl)

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
                     (Move (That CardC) BattlefieldZ)

-- "You may put a creature card from your hand onto the battlefield. That
-- creature gains haste. Sacrifice that creature at the beginning of the
-- next end step." (Through the Breach; Splice line elided — real Sneak
-- Attack says "the creature", a definite read this chapter doesn't mint)
-- — the hand-to-battlefield move RESTORES the typed carrier (the head
-- type was projected at introduction, never lost), the choice-determined
-- referent survives the delay [CR#603.7c] while a target would not, and
-- the trailing adverbial is the `Delayed` mark on its clause.
throughTheBreach : Effect []
throughTheBreach = AndThen (May You (Move (A (And [creature, InZone (HandOf You)])) BattlefieldZ))
                           (AndThen (gainsHaste (That (Perm Creature)))
                                    (Delayed (sacrifice You (That (Perm Creature)))))

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

-- "Sacrifice this land: It deals 3 damage to target player. That
-- player discards a card." (Immersturm Skullcairn; its mana and {T}
-- cost components, the land's other lines, and its timing line elided)
-- — the cost's sacrificed self is the effect's "It": the cost move
-- mints the referent ([CR#400.7]) and it survives the colon publicly
-- ([CR#400.7j]); then the sorted demonstrative at Player kind and a
-- keyword-action macro (Discard) whose body moves a hand-zone choice.
immersturmSkullcairn : Activated []
immersturmSkullcairn = MkActivated (sacrifice You (ThisOf Land))
                                   (AndThen (DealDamage It (Lit 3) (Target AnyPlayer))
                                            (discardsACard (That PlayerC)))

-- "{R}, Sacrifice this artifact: It deals 2 damage to any target."
-- (Pyrite Spellbomb, first ability; {R} and the card's second ability
-- elided) — the smallest cost-antecedent pair: the sorted
-- self-reference moved by the cost is the only mention "It" can reach.
pyriteSpellbomb : Activated []
pyriteSpellbomb = MkActivated (sacrifice You (ThisOf Artifact))
                              (DealDamage It (Lit 2) (Target AnyTarget))

-- "Target player sacrifices a creature of their choice." (Diabolic
-- Edict) — the declarative clause: the target subject introduces, the
-- verb phrase is typed after it, and "of their choice" is the marked
-- own-choice method on the indefinite (the corpus has no bare "Target
-- player sacrifices a creature").
diabolicEdict : Effect []
diabolicEdict = sacrifice (Target AnyPlayer) (ATheirChoice creature)

-- "Each player sacrifices a creature of their choice." (Innocent
-- Blood) — a group subject: the same clause shape over "each player".
innocentBlood : Effect []
innocentBlood = sacrifice (Each AnyPlayer) (ATheirChoice creature)

-- "Target player discards a card." (Cry of Contrition, first line; its
-- Haunt lines elided) — the declarative discard whose imperative twin
-- is the same macro with `You`.
cryOfContrition : Effect []
cryOfContrition = discardsACard (Target AnyPlayer)

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
  badStale = AndThen (destroy (Target creature)) (Delayed (sacrifice You It))

-- After the exile, the referent no longer answers to "creature": its
-- carrier is derived from the RETAGGED zone ([CR#110.1]), so the typed
-- demonstrative has no antecedent — the carrier-word rule as a type
-- error ("that card" is the spelling that resolves; see `cloudshift`).
failing "countCarrier"
  badStaleCarrier : Effect []
  badStaleCarrier = AndThen (exile (Target creatureYouControl))
                            (Move (That (Perm Creature)) BattlefieldZ)

-- A hidden-zone cost mention is unreadable past the colon: the card
-- bounced to hand is not among the public survivors ([CR#400.2] —
-- hand is hidden; no [CR#400.7] exception reaches it, and
-- Soratami Cloudskater-family costs write no such read back).
failing "publicOnly"
  badHiddenCost : Activated []
  badHiddenCost = MkActivated (Move (A creature) HandZ) (Tap It)

-- Two cost moves leave two candidate antecedents ("Discard a card,
-- Sacrifice a creature: …" — Falkenrath Pit Fighter-family): a bare
-- "It" past the colon is ambiguous. Real costs of this shape read
-- back with definite descriptions ("the sacrificed creature" — a
-- later chapter's noun), never a bare pronoun.
failing "countOnes"
  badTwoCostMentions : Activated []
  badTwoCostMentions = MkActivated (AndThen (discardsACard You)
                                            (sacrifice You (A creature)))
                                   (Tap It)

-- The zone half of sacrifice's implicit restriction as a type error:
-- an exiled referent is not sacrificeable [CR#701.21a].
failing "OnBattlefield"
  badSacrificeExiled : Effect []
  badSacrificeExiled = AndThen (exile (Target creature)) (sacrifice You It)
