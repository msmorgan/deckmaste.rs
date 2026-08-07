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
||| 4. **A boundary is a view derived from the determiner** (revised in
|||    chapter four). "…at the beginning of the next end step" stays
|||    attached to its clause as English attaches it; `Delayed` marks
|||    the clause future, and its body reads the full discourse as
|||    SETTLED PARTICULARS (`settleTargets`): [CR#603.7c] refers to
|||    particular objects determiner-blind — Turn to Mist returns "that
|||    card" whose referent was a target, Junkyo Bell delays sacrificing
|||    a live target — while announcing stays local (the delayed
|||    ability targets in its own event, [CR#603.3d,601.2c]; Swooping
|||    Pteranodon does both in one sentence), so the "other"
|||    presupposition stops at the boundary (`badDelayedOther`).
|||    Staleness is the carrier/zone question (`badStale` fails the
|||    sacrifice zone demand, not a read gate; the fire-time zone
|||    expectation of [CR#603.7c] is runtime — a no-op, not an
|||    illegality). No timing tag is stored.
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
||| Chapter four, reference machinery (evidence: Suspended Sentence,
||| Turn to Mist, Flickering Spirit; Junkyo Bell and Swooping
||| Pteranodon ground the boundary revision):
|||
||| 14. **Relative clauses fold their mentions.** `nounDelta`/`predDelta`
|||    make a phrase's introductions one computation: "target creature
|||    an opponent controls" leaves the opponent readable ("That
|||    player…", `suspendedSentence`), and two inner opponents are
|||    refused as ambiguous (`badInnerAmbig`) — the uniqueness gate
|||    reaches inside predicates.
||| 15. **The delayed boundary settles; it does not drop.** Chapter
|||    one's target filter modeled the wrong axis — see revised finding
|||    4: reads pass as settled particulars, while the "other"
|||    presupposition and fresh announcing stay local to the delayed
|||    ability.
||| 16. **The self-reference moves like anything else.** An
|||    effect-position `ThisOf` move mints the new object's binding
|||    mid-sentence ([CR#400.7]) — Flickering Spirit's "it" reads its
|||    own exile.
|||
||| Chapter five, temporal grammar (evidence: Jump, Giant Growth, Bond
||| of Revival, Graceful Reprieve, Vraska's Stoneglare, Phthisis,
||| Karplusan Yeti):
|||
||| 17. **Duration is trailing-adverbial data.** `Gain`/`Gets` carry
|||    the stated duration ([CR#611.2a]); the unstated form is an
|||    explicit `Nothing` (lasts until end of game) per the
|||    written-Nothing convention — no defaults.
||| 18. **An event query transforms the delayed context.** `Delayed`
|||    names what it waits for; a time query settles only, and "when
|||    [target] dies this turn" both announces its watched referent and
|||    retags it to the graveyard ([CR#700.4]) — the event is a zone
|||    transition, so `gracefulReprieve`'s "that card" resolves and
|||    "that creature" is refused (`badDeadCreatureRead`).
||| 19. **Last-known reads are the standing semantics.** Reads ignore
|||    zone, so a dead referent's characteristics stay readable
|||    (`vraskasStoneglare`; `phthisis` threads "power plus toughness"
|||    as amount arithmetic); which VALUES those reads see is runtime —
|||    the ability layer's story.
||| 20. **`Fights` is confirmed primitive.** No operative oracle text
|||    spells the mutual-damage expansion (reminder text only), and the
|||    sequential family (`karplusanYeti`) is not equivalent — its two
|||    clauses admit state-based actions between them, where
|||    [CR#701.14a] deals one simultaneous event.
|||
||| Chapter six, group reference and qualities (evidence: Fulgent
||| Distraction, Continue?, Sudden Demise, Kindred Dominance; the
||| amounts axis stayed evidence-only — see the not-settled list):
|||
||| 21. **Groups are bindings like any other.** A counted target
|||    mention ("two target creatures", "up to four …") binds once,
|||    ManyOf, its numeral term-level surface data no context consumer
|||    reads ([CR#601.2c] fixes the count at announce, and the one read
|||    that wants a number wants the ACTUAL count, not the bound).
|||    `Them`/`Those c` are the strict-unique plural twins of the
|||    singular reads (`badThemAmbig`), riding the same carrier and
|||    retag machinery.
||| 22. **The fronted choose-sentence is a scope divergence.** "Choose
|||    two target creatures." announces in its own clause and scopes
|||    everything after; core's choose binders are resolution-time and
|||    nontarget ([CR#115.1]), so the divergence is recorded on the
|||    constructor, not mirrored away.
||| 23. **Qualities are a kind, not a carrier.** `Kind` grows one
|||    parameterized constructor (`Quality`); a chosen color/type
|||    enters the discourse like any mention and `OfChosen` demands it
|||    uniquely per sort (`badChosenWrongSort`). A quality mention has
|||    no carrier word — `carrier` returns `Maybe`, junk-free — and
|||    kind matching routes through `sameKind`, whose deliberate lack
|||    of a catch-all makes the NEXT kind a totality error instead of a
|||    silent zero. "All" is a surface determiner distinct from
|||    distributive "each" (the guide separates them; the CR does not),
|||    stored as `AllD`.
||| 24. **"That much" / "that many" read event results, not amounts.**
|||    The corpus antecedents are quantities of what HAPPENED — cards
|||    moved (Asmodeus the Archfiend), aggregate life lost (the Extort
|||    reminder), an intervening-if's count (Feast of the Victorious
|||    Dead), payment and cost quantities (Harnessed Lightning, Arcee)
|||    — runtime facts, not surface projections of any clause's amount
|||    term, so the read is NOT typed this chapter; the sole
|||    written-amount coincidence (Foul-Tongue Shriek) would prove the
|||    wrong rule.
|||
||| Not settled yet: the kind union ("any target" spans objects and
||| players [CR#115.4,115.1] — elided to `Object`);
||| owned-zone mentions beyond `You` ("its owner's hand" — destination
||| and predicate-inner mentions do not fold yet); controller
||| fold-state (the controller half of verb restrictions, entangled
||| with [CR#109.4]); `May`'s if-you-do / if-not branches (the Risk
||| Factor / Rakdos, Patron of Chaos else-clauses); definite
||| descriptive reads ("the sacrificed creature", "the exiled card" —
||| also what `AAtRandom`'s positive waits on: Pyromancy reads "the
||| discarded card") and the chosen-OBJECT forms ("the chosen
||| creatures"); more event queries (upkeep / end-of-combat /
||| leaves-the-battlefield — Slaughter Pact, Mirror Match, and
||| Kjeldoran Elite Guard wait on pay, tokens, and an unknown-zone
||| retag); the player pronoun "they"
||| (corpus-attested only inside trigger and unless clauses — Havoc,
||| Tergrid's Lantern — so `They`'s positive waits on those
||| constructions), player groups ("each opponent … they"
||| distributives), and "the rest"; event-outcome referents ("that
||| much" / "that many" / "this way" — RULED a third context data
||| class, sort-projected with runtime values, per the decision
||| record §3; finding 24 holds the evidence, machinery is a coming
||| chapter); "any number of" / "X" target groups
||| (corpus-frequent; constructors wait on verified whole cards); an
||| up-to-N group as an "other" witness ([CR#115.6] — it may denote
||| zero; no corpus line pairs them yet);
||| static abilities and "for as long as" durations ([CR#611.2b],
||| Kitesail Corsair); last-known VALUES (reads ignore zone — finding
||| 19 — but [CR#109.4] gives off-battlefield objects no controller,
||| so the value story belongs to the ability layer); Token / Spell
||| / stack-object / Amount carriers ("that much"; bare `This` stays
||| untracked, and "this spell" / "this card" carriers with it); "the
||| chosen [quality]" (quality-kind bindings); and coordination ellipsis (Arc
||| Trail's shared verb is spelling's business — here it is a clause
||| sequence). The context-as-phrase-telescope collapse (bindings storing
||| the mention terms themselves, every projection computed) stays open as
||| a possible later simplification — pressure that grows as kinds
||| multiply (quality bindings carry dead ty/zone fields).
module Experimental

%default total

-- ===== Vocabulary =====

||| Card types, as catalog atoms ([CR#205.2a]; only what the chapters
||| need — the real set is an open catalog, not an engine enum).
public export
data CardType = Creature | Artifact | Land

||| Quality sorts — the choosable characteristics ([CR#105.1,302.3];
||| only what the chapters need).
public export
data QualitySort = Color | CreatureType

public export
sameQ : QualitySort -> QualitySort -> Bool
sameQ Color Color = True
sameQ CreatureType CreatureType = True
sameQ _ _ = False

||| What a binding can bind ([CR#115.1] — targets are objects and/or
||| players; the union kind is deferred with the carrier lattice) —
||| plus chosen qualities ("Choose a color"), which enter the same
||| discourse.
public export
data Kind = Object | Player | Quality QualitySort

||| Kind equality — deliberately WITHOUT a catch-all: adding a Kind
||| makes this a totality error, not a silent zero in the counters.
public export
sameKind : Kind -> Kind -> Bool
sameKind Object Object = True
sameKind Object Player = False
sameKind Object (Quality _) = False
sameKind Player Object = False
sameKind Player Player = True
sameKind Player (Quality _) = False
sameKind (Quality _) Object = False
sameKind (Quality _) Player = False
sameKind (Quality a) (Quality b) = sameQ a b

||| Singular mention or group mention — the guard that keeps "it" from
||| resolving to a plural antecedent.
public export
data Plurality = OneOf | ManyOf

||| The introducing word of a mention — a SURFACE projection ("target",
||| "a", "each", "all", or a definite/derived mention). Rules facts
||| (the settled-target boundary, the "other" presupposition) are
||| functions of it, never stored alongside it.
public export
data Determiner = TargetD | AD | EachD | AllD | TheD

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
||| stops being one when it leaves). A quality mention has no carrier
||| word — `Nothing`, never a junk value.
public export
carrier : Binding -> Maybe Carrier
carrier (MkBinding _ Player _ _ _) = Just PlayerC
carrier (MkBinding _ Object _ ty (Just Battlefield)) = Just (maybe AnyPerm Perm ty)
carrier (MkBinding _ Object _ ty (Just _)) = Just CardC
carrier (MkBinding _ Object _ ty Nothing) = Just AnyPerm
carrier (MkBinding _ (Quality _) _ _ _) = Nothing

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
compatMaybe : Carrier -> Maybe Carrier -> Bool
compatMaybe c Nothing = False
compatMaybe c (Just c') = compatC c c'

||| Does the wanted carrier reach this binding's carrier (if any)?
public export
carrierIs : Carrier -> Binding -> Bool
carrierIs c b = compatMaybe c (carrier b)

public export
kindOfC : Carrier -> Kind
kindOfC PlayerC = Player
kindOfC _ = Object

||| Singular mentions of a kind, counted — the wildcard pronoun's
||| obligation is `= 1`: zero is an unbound anaphor, two an ambiguous one
||| (the uniqueness gate the controlled language relies on). Kind
||| matching routes through `sameKind` so a new Kind cannot silently
||| count as zero.
public export
countOnes : Kind -> Bindings -> Nat
countOnes k [] = Z
countOnes k (MkBinding _ k' OneOf _ _ :: bs) =
  if sameKind k k' then S (countOnes k bs) else countOnes k bs
countOnes k (_ :: bs) = countOnes k bs

||| Singular mentions whose CURRENT carrier the wanted carrier reaches —
||| the sorted demonstrative's obligation is `= 1` (strict uniqueness
||| after the filter; there is no nearest-wins).
public export
countCarrier : Carrier -> Bindings -> Nat
countCarrier c [] = Z
countCarrier c (b :: bs) =
  case (b.plur, carrierIs c b) of
    (OneOf, True) => S (countCarrier c bs)
    _ => countCarrier c bs

||| Chosen-quality mentions of a sort, counted — "the chosen color"
||| demands exactly one.
public export
countQuality : QualitySort -> Bindings -> Nat
countQuality q [] = Z
countQuality q (MkBinding _ k OneOf _ _ :: bs) =
  if sameKind (Quality q) k then S (countQuality q bs) else countQuality q bs
countQuality q (_ :: bs) = countQuality q bs

||| Group mentions of a kind, counted — the plural wildcard's
||| obligation is `= 1`, the ManyOf twin of `countOnes`.
public export
countManys : Kind -> Bindings -> Nat
countManys k [] = Z
countManys k (MkBinding _ k' ManyOf _ _ :: bs) =
  if sameKind k k' then S (countManys k bs) else countManys k bs
countManys k (_ :: bs) = countManys k bs

||| Group mentions whose CURRENT carrier the wanted carrier reaches —
||| the sorted plural demonstrative's gate.
public export
countManyCarrier : Carrier -> Bindings -> Nat
countManyCarrier c [] = Z
countManyCarrier c (b :: bs) =
  case (b.plur, carrierIs c b) of
    (ManyOf, True) => S (countManyCarrier c bs)
    _ => countManyCarrier c bs

||| Is any target-determined mention of this kind in scope? — the
||| presupposition of the modifier "other" ([CR#115.4]), stated entirely
||| in target vocabulary.
public export
anyTargeted : Kind -> Bindings -> Bool
anyTargeted k [] = False
anyTargeted k (MkBinding TargetD k' _ _ _ :: bs) =
  if sameKind k k' then True else anyTargeted k bs
anyTargeted k (_ :: bs) = anyTargeted k bs

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
settleTargets (MkBinding TargetD k plur ty zn :: bs) =
  MkBinding TheD k plur ty zn :: settleTargets bs
settleTargets (b :: bs) = b :: settleTargets bs

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
  case (b.plur, carrierIs c b) of
    (OneOf, True) => b.zone
    _ => zoneOfThat c bs

||| The current zone of the plural wildcard's group referent.
public export
zoneOfThem : Bindings -> Maybe Zone
zoneOfThem [] = Nothing
zoneOfThem (MkBinding det Object ManyOf ty zn :: bs) = zn
zoneOfThem (b :: bs) = zoneOfThem bs

||| The current zone of a sorted plural demonstrative's group referent.
public export
zoneOfThose : Carrier -> Bindings -> Maybe Zone
zoneOfThose c [] = Nothing
zoneOfThose c (b :: bs) =
  case (b.plur, carrierIs c b) of
    (ManyOf, True) => b.zone
    _ => zoneOfThose c bs

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
data Keyword = Haste | Flying | Trample

public export
data Ability = KeywordAbility Keyword

||| Durations, as the trailing adverbial writes them ([CR#611.2a] — a
||| resolution-generated continuous effect "lasts as long as stated";
||| with no stated duration it lasts until end of game, which is the
||| explicit `Nothing` spelling per the no-defaults convention).
||| "for as long as" durations ([CR#611.2b]) are a later chapter.
public export
data Duration = UntilEndOfTurn | UntilYourNextTurn

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
    -- head noun "color" / "creature type" — the choosable quality
    -- ([CR#105.1,302.3]).
    QualityNoun : (q : QualitySort) -> Predicate bs (Quality q)
    -- "of the chosen [quality]": reads the unique chosen quality (the
    -- guide's stored-quality naming; choice made at resolution
    -- [CR#608.2d]). The chosen-OBJECT twin ("the chosen creatures")
    -- waits with the definite reads.
    OfChosen : (q : QualitySort) -> {auto 0 ok : countQuality q bs = 1} -> Predicate bs Object
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
  bindFor det plur {k = Quality q} p = MkBinding det (Quality q) plur Nothing Nothing

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
    -- "[n] target [pred]s" / "up to [n] target [pred]s": counted group
    -- mentions — one ManyOf binding; the numeral is surface data no
    -- read consults ([CR#601.2c] distinctness is announce business).
    TargetGroup : (n : Nat) -> Predicate bs k -> Noun bs k
    TargetUpTo : (n : Nat) -> Predicate bs k -> Noun bs k
    -- "all [pred]s": the set-level group — a surface determiner the
    -- guide keeps distinct from distributive "each" (the CR fixes both
    -- sets at resolution and separates them no further).
    AllOf : Predicate bs k -> Noun bs k
    -- "it" / "its": the wildcard pronoun — exactly one singular Object
    -- mention may precede. Zero = unbound, two = ambiguous; both
    -- unspellable.
    It : {auto 0 ok : countOnes Object bs = 1} -> Noun bs Object
    -- "they" for a player (singular; player groups are a later chapter).
    They : {auto 0 ok : countOnes Player bs = 1} -> Noun bs Player
    -- "them": the plural wildcard — exactly one group mention of the
    -- kind may precede (the ManyOf twin of `It`).
    Them : {auto 0 ok : countManys Object bs = 1} -> Noun bs Object
    -- "those [carrier]s": the sorted plural demonstrative — exactly
    -- one group mention whose CURRENT carrier answers to the noun.
    Those : (c : Carrier) -> {auto 0 ok : countManyCarrier c bs = 1} -> Noun bs (kindOfC c)
    -- "that [carrier]": the sorted demonstrative — exactly one mention
    -- whose CURRENT carrier answers to the noun may precede.
    That : (c : Carrier) -> {auto 0 ok : countCarrier c bs = 1} -> Noun bs (kindOfC c)
    -- "[object]'s controller" / "its owner": relational nouns — a NEW
    -- player referent derived from an object mention ([CR#108.3,109.4]).
    ControllerOf : Noun bs Object -> Noun bs Player
    OwnerOf : Noun bs Object -> Noun bs Player

  ||| The bindings a noun phrase prepends to the discourse — its own
  ||| head first (determined mentions bind, reads don't), then the
  ||| mentions its predicate introduces in textual order: relative
  ||| clauses FOLD, so "target creature an opponent controls" leaves
  ||| both the creature and the opponent readable.
  public export
  nounDelta : {bs : Bindings} -> {k : Kind} -> Noun bs k -> List Binding
  nounDelta This = []
  nounDelta (ThisOf t) = []
  nounDelta You = []
  nounDelta (Target p) = bindFor TargetD OneOf p :: predDelta p
  nounDelta (Each p) = bindFor EachD ManyOf p :: predDelta p
  nounDelta (A p) = bindFor AD OneOf p :: predDelta p
  nounDelta (ATheirChoice p) = bindFor AD OneOf p :: predDelta p
  nounDelta (AAtRandom p) = bindFor AD OneOf p :: predDelta p
  nounDelta (TargetGroup n p) = bindFor TargetD ManyOf p :: predDelta p
  nounDelta (TargetUpTo n p) = bindFor TargetD ManyOf p :: predDelta p
  nounDelta (AllOf p) = bindFor AllD ManyOf p :: predDelta p
  nounDelta It = []
  nounDelta They = []
  nounDelta Them = []
  nounDelta (That c) = []
  nounDelta (Those c) = []
  nounDelta (ControllerOf n) = MkBinding TheD Player OneOf Nothing Nothing :: nounDelta n
  nounDelta (OwnerOf n) = MkBinding TheD Player OneOf Nothing Nothing :: nounDelta n

  ||| The mentions a predicate's clauses introduce, textual order.
  public export
  predDelta : {bs : Bindings} -> {k : Kind} -> Predicate bs k -> List Binding
  predDelta (ControlledBy n) = nounDelta n
  predDelta (InZone z) = zoneDelta z
  predDelta (And ps) = predDeltaAll ps
  predDelta (Not p) = predDelta p
  predDelta _ = []

  public export
  predDeltaAll : {bs : Bindings} -> {k : Kind} -> List (Predicate bs k) -> List Binding
  predDeltaAll [] = []
  predDeltaAll (p :: ps) = predDelta p ++ predDeltaAll ps

  public export
  zoneDelta : {bs : Bindings} -> ZoneExpr bs -> List Binding
  zoneDelta (HandOf n) = nounDelta n
  zoneDelta (GraveyardOf n) = nounDelta n
  zoneDelta _ = []

  ||| What a noun contributes to the discourse that follows it.
  public export
  nomIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Bindings
  nomIntro n = nounDelta n ++ bs

  ||| An amount expression — where "equal to its power" lives, so it
  ||| threads like everything else.
  public export
  data Amount : Bindings -> Type where
    Lit : Nat -> Amount bs
    PowerOf : Noun bs Object -> Amount bs      -- "[its/…] power"
    ToughnessOf : Noun bs Object -> Amount bs  -- "[its/…] toughness"
    -- "X" — announced with the cost ([CR#107.3a,107.3i]): a fixed
    -- value by resolution, not a discourse referent.
    XVal : Amount bs
    -- "[a] plus [b]" — the second operand reads after the first.
    Plus : (a : Amount bs) -> Amount (amtIntro a) -> Amount bs

  public export
  amtIntro : {bs : Bindings} -> Amount bs -> Bindings
  amtIntro (Lit n) = bs
  amtIntro (PowerOf nom) = nomIntro nom
  amtIntro (ToughnessOf nom) = nomIntro nom
  amtIntro XVal = bs
  amtIntro (Plus a b) = amtIntro b

  ||| Life-total change operands ([CR#119.3]; `Set` is a later chapter).
  public export
  data LifeOp : Bindings -> Type where
    Up : Amount bs -> LifeOp bs     -- "gains [amt] life"
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
    NextEndStep : EventQuery bs                     -- "at the beginning of the next end step"
    DiesThisTurn : Noun bs Object -> EventQuery bs  -- "when [n] dies this turn" ([CR#700.4])

  ||| The context a delayed body reads: settled particulars, with the
  ||| event's own transition applied — dying retags the watched
  ||| referent to the graveyard exactly as a move would ([CR#700.4]).
  public export
  delayedCtx : {bs : Bindings} -> EventQuery bs -> Bindings
  delayedCtx NextEndStep = settleTargets bs
  delayedCtx (DiesThisTurn n) = settleTargets (moveIntro n Graveyard)

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
    -- "[a] fights [b]" ([CR#701.14a]). Primitive, confirmed: the
    -- expansion is a single simultaneous event, which no clause
    -- sequence reproduces (state-based actions can intervene between
    -- sentences — see `karplusanYeti`), and no operative oracle text
    -- spells it out (reminder text only).
    Fights : (a : Noun bs Object) -> (b : Noun (nomIntro a) Object) -> Effect bs
    -- "tap [n]" ([CR#701.26a]) — core basis.
    Tap : Noun bs Object -> Effect bs
    -- "Choose [n]." — the choice clause as surface for the mention it
    -- announces ([CR#601.2c] for targets; [CR#608.2d] otherwise). A
    -- recorded DIVERGENCE from core, whose choose binders are
    -- resolution-time and nontarget ([CR#115.1] keeps the words
    -- apart): here the fronted sentence scopes everything after it.
    Choose : {k : Kind} -> Noun bs k -> Effect bs
    -- "[move] [n] [to zone]" — the zone-change primitive every keyword
    -- action's body bottoms out in ([CR#701.8a] shape). Destination
    -- only: the from-zone is the referent's fold-state, which this
    -- clause UPDATES (the retag).
    Move : (what : Noun bs Object) -> (to : ZoneExpr (nomIntro what)) -> Effect bs
    -- "[who] gains/loses [amt] life" ([CR#119.3]) — core basis (merged).
    ChangeLife : (who : Noun bs Player) -> (op : LifeOp (nomIntro who)) -> Effect bs
    -- "[n] gains [ability] [duration]" — establishes a continuous
    -- effect for the stated duration ([CR#611.2a]).
    Gain : Noun bs Object -> Ability -> Maybe Duration -> Effect bs
    -- "[n] gets [+p/+t] [duration]" — the stat-modifying continuous
    -- effect, same duration discipline.
    Gets : Noun bs Object -> (pow : Integer) -> (tou : Integer) ->
           Maybe Duration -> Effect bs
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
    -- "[e] [when/at event-query]" — the temporal adverbial stays on
    -- its clause (leading vs trailing position is linearization); the
    -- body reads the discourse as settled particulars transformed by
    -- the event (`delayedCtx`, [CR#603.7c,603.3d]).
    Delayed : (ev : EventQuery bs) -> Effect (delayedCtx ev) -> Effect bs

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
  setZoneThem : Zone -> Bindings -> Bindings
  setZoneThem z [] = []
  setZoneThem z (MkBinding det Object ManyOf ty zn :: bs) =
    MkBinding det Object ManyOf ty (Just z) :: bs
  setZoneThem z (b :: bs) = b :: setZoneThem z bs

  public export
  setZoneThose : Carrier -> Zone -> Bindings -> Bindings
  setZoneThose c z [] = []
  setZoneThose c z (b :: bs) =
    case (b.plur, carrierIs c b) of
      (ManyOf, True) => setZone z b :: bs
      _ => b :: setZoneThose c z bs

  public export
  setZoneThat : Carrier -> Zone -> Bindings -> Bindings
  setZoneThat c z [] = []
  setZoneThat c z (b :: bs) =
    case (b.plur, carrierIs c b) of
      (OneOf, True) => setZone z b :: bs
      _ => b :: setZoneThat c z bs

  public export
  moveIntro : {bs : Bindings} -> {k : Kind} -> Noun bs k -> Zone -> Bindings
  moveIntro (Target p) z = setZoneHead z (nomIntro (Target p))
  moveIntro (Each p) z = setZoneHead z (nomIntro (Each p))
  moveIntro (A p) z = setZoneHead z (nomIntro (A p))
  moveIntro (ATheirChoice p) z = setZoneHead z (nomIntro (ATheirChoice p))
  moveIntro (AAtRandom p) z = setZoneHead z (nomIntro (AAtRandom p))
  moveIntro (TargetGroup n p) z = setZoneHead z (nomIntro (TargetGroup n p))
  moveIntro (TargetUpTo n p) z = setZoneHead z (nomIntro (TargetUpTo n p))
  moveIntro (AllOf p) z = setZoneHead z (nomIntro (AllOf p))
  moveIntro It z = setZoneIt z bs
  moveIntro Them z = setZoneThem z bs
  moveIntro (That c) z = setZoneThat c z bs
  moveIntro (Those c) z = setZoneThose c z bs
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
  nounZone (TargetGroup n p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (TargetUpTo n p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (AllOf p) = Just (zoneOr Battlefield (seedZone p))
  nounZone It = zoneOfIt bs
  nounZone They = Nothing
  nounZone Them = zoneOfThem bs
  nounZone (That c) = zoneOfThat c bs
  nounZone (Those c) = zoneOfThose c bs
  nounZone (ControllerOf n) = Nothing
  nounZone (OwnerOf n) = Nothing

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = nomIntro to
  effIntro (Fights a b) = nomIntro b
  effIntro (Tap n) = nomIntro n
  effIntro (Choose n) = nomIntro n
  effIntro (Move what to) = moveIntro what (zoneSort to)
  effIntro (ChangeLife who op) = lifeIntro op
  effIntro (Gain n _ _) = nomIntro n
  effIntro (Gets n _ _ _) = nomIntro n
  effIntro (Composite _ e) = effIntro e
  effIntro (Does s e) = effIntro e
  effIntro (May d e) = effIntro e            -- a declined May skips at runtime, not in scope
  effIntro (AndThen e1 e2) = effIntro e2
  effIntro (Delayed ev e) = bs               -- a future clause mentions nothing NOW

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

-- "[n] gains haste [duration]"
public export
gainsHaste : Noun bs Object -> Maybe Duration -> Effect bs
gainsHaste n d = Gain n (KeywordAbility Haste) d

-- "[who] loses [amt] life"
public export
losesLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
losesLife who amt = ChangeLife who (Down amt)

-- "[who] gains [amt] life"
public export
gainsLife : (who : Noun bs Player) -> Amount (nomIntro who) -> Effect bs
gainsLife who amt = ChangeLife who (Up amt)

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
-- referent survives the delay [CR#603.7c], and the trailing adverbial
-- is the `Delayed` mark on its clause.
throughTheBreach : Effect []
throughTheBreach = AndThen (May You (Move (A (And [creature, InZone (HandOf You)])) BattlefieldZ))
                           (AndThen (gainsHaste (That (Perm Creature)) Nothing)
                                    (Delayed NextEndStep (sacrifice You (That (Perm Creature)))))

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

-- "Destroy target creature an opponent controls. That player loses 3
-- life." (Suspended Sentence; its self-exile clause and Suspend lines
-- elided) — the relative clause's inner mention folds: the indefinite
-- opponent enters the discourse from INSIDE the target's predicate,
-- and the sorted demonstrative reads it.
suspendedSentence : Effect []
suspendedSentence = AndThen (destroy (Target (And [creature, ControlledBy anOpponent])))
                            (losesLife (That PlayerC) (Lit 3))

-- "Exile this creature, then return it to the battlefield under its
-- owner's control." (Flickering Spirit's activated ability; its
-- mana-only cost and "under its owner's control" elided) — the sorted
-- self-reference moved by the EFFECT: the exile mints the new object's
-- binding mid-sentence ([CR#400.7]) and "it" reads it.
flickeringSpirit : Effect []
flickeringSpirit = AndThen (exile (ThisOf Creature)) (Move It BattlefieldZ)

-- "Exile target creature. Return that card to the battlefield under
-- its owner's control at the beginning of the next end step." (Turn to
-- Mist; "under its owner's control" elided) — the delayed clause reads
-- the TARGET-determined referent as a settled particular under its
-- retagged carrier ([CR#603.7c] is determiner-blind); the fire-time
-- zone expectation stays runtime (a mismatch is a no-op, not an
-- illegality).
turnToMist : Effect []
turnToMist = AndThen (exile (Target creature))
                     (Delayed NextEndStep (Move (That CardC) BattlefieldZ))

-- "Target creature gains flying until end of turn." (Jump) — the
-- duration as trailing-adverbial data ([CR#611.2a]).
jump : Effect []
jump = Gain (Target creature) (KeywordAbility Flying) (Just UntilEndOfTurn)

-- "Target creature gets +3/+3 until end of turn." (Giant Growth)
giantGrowth : Effect []
giantGrowth = Gets (Target creature) 3 3 (Just UntilEndOfTurn)

-- "Return target creature card from your graveyard to the
-- battlefield. It gains haste until your next turn." (Bond of
-- Revival) — an owned-zone source and the cross-turn duration; the
-- return is just a Move, and "it" reads the retagged referent.
bondOfRevival : Effect []
bondOfRevival = AndThen (Move (Target (And [creature, InZone (GraveyardOf You)])) BattlefieldZ)
                        (gainsHaste It (Just UntilYourNextTurn))

-- "When target creature dies this turn, return that card to the
-- battlefield under its owner's control." (Graceful Reprieve; "under
-- its owner's control" elided) — the event query transforms the
-- delayed context: the when-clause announces the watched target and
-- dying retags it to the graveyard ([CR#700.4]), so "that card" is
-- the carrier that resolves.
gracefulReprieve : Effect []
gracefulReprieve = Delayed (DiesThisTurn (Target creature))
                           (Move (That CardC) BattlefieldZ)

-- "Destroy target creature. You gain life equal to its toughness."
-- (Vraska's Stoneglare; its tutor clause elided) — a last-known read:
-- reads ignore zone, so the dead referent's characteristics stay
-- readable; which values they see is runtime (the ability layer's
-- story).
vraskasStoneglare : Effect []
vraskasStoneglare = AndThen (destroy (Target creature))
                            (gainsLife You (ToughnessOf It))

-- "Destroy target creature. Its controller loses life equal to its
-- power plus its toughness." (Phthisis; its Suspend line elided) —
-- the relational noun over the dead referent, and amount arithmetic
-- threading left to right.
phthisis : Effect []
phthisis = AndThen (destroy (Target creature))
                   (losesLife (ControllerOf It) (Plus (PowerOf It) (ToughnessOf It)))

-- "This creature deals damage equal to its power to target creature.
-- That creature deals damage equal to its power to this creature."
-- (Karplusan Yeti's activated ability; its {T} cost elided, and the
-- source-referring "its" is spelled as the self-reference — source
-- mentions don't bind) — the SEQUENTIAL cousin of fight: two one-shot
-- damage clauses, not [CR#701.14a]'s single simultaneous event
-- (state-based actions can intervene between the sentences), which is
-- why `Fights` stays primitive.
karplusanYeti : Effect []
karplusanYeti = AndThen (DealDamage (ThisOf Creature) (PowerOf (ThisOf Creature)) (Target creature))
                        (DealDamage (That (Perm Creature)) (PowerOf It) (ThisOf Creature))

-- "Choose two target creatures. Tap those creatures, then unattach
-- all Equipment from them." (Fulgent Distraction; the unattach clause
-- elided) — a counted group mention, read back by the sorted plural
-- demonstrative.
fulgentDistraction : Effect []
fulgentDistraction = AndThen (Choose (TargetGroup 2 creature))
                             (Tap (Those (Perm Creature)))

-- "Choose up to four target creature cards in your graveyard that
-- were put there from the battlefield this turn. Return them to the
-- battlefield." (Continue?; its look-back restrictive clause elided —
-- event-history predicates are unminted) — the bounded group, an
-- owned-zone predicate, and the plural wildcard riding the return's
-- retag.
continueSpell : Effect []
continueSpell = AndThen (Choose (TargetUpTo 4 (And [creature, InZone (GraveyardOf You)])))
                        (Move Them BattlefieldZ)

-- "Choose a color. Sudden Demise deals X damage to each creature of
-- the chosen color." (Sudden Demise) — a quality mention: the chosen
-- color enters the discourse like any mention ([CR#105.1]) and the
-- predicate-internal read demands it uniquely; X is the announced
-- cost variable ([CR#107.3a]).
suddenDemise : Effect []
suddenDemise = AndThen (Choose (A (QualityNoun Color)))
                       (DealDamage This XVal (Each (And [creature, OfChosen Color])))

-- "Choose a creature type. Destroy all creatures that aren't of the
-- chosen type." (Kindred Dominance) — the set-level "all" determiner
-- over a negated quality read.
kindredDominance : Effect []
kindredDominance = AndThen (Choose (A (QualityNoun CreatureType)))
                           (destroy (AllOf (And [creature, Not (OfChosen CreatureType)])))

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

-- The delay does not launder a dead referent: sacrifice's zone demand
-- reads fold-state through the boundary, and the destroyed target sits
-- in the graveyard ([CR#701.21a]; contrast Junkyo Bell, which legally
-- delays sacrificing a LIVE target — the distinction is the referent's
-- zone, never its determiner).
failing "OnBattlefield"
  badStale : Effect []
  badStale = AndThen (destroy (Target creature)) (Delayed NextEndStep (sacrifice You It))

-- "Another target" inside a delayed clause can only be distinct from
-- the DELAYED ability's own targets — it announces in its own event
-- ([CR#603.3d,601.2c]), so the outer clause's settled targets are no
-- witness for the presupposition. (Soundness-derived: the corpus
-- writes no such line; Swooping Pteranodon shows the two halves — a
-- fresh "target land" announced at delay time reading back "that
-- creature" from the outer clause.)
failing "anyTargeted"
  badDelayedOther : Effect []
  badDelayedOther = AndThen (DealDamage This (Lit 2) (Target AnyTarget))
                            (Delayed NextEndStep (DealDamage This (Lit 1) (Target anyOtherTarget)))

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

-- After the watched target dies, it no longer answers to "creature":
-- the event retag flips the carrier ([CR#700.4,110.1]) — "that card"
-- is the spelling that resolves (see `gracefulReprieve`).
failing "countCarrier"
  badDeadCreatureRead : Effect []
  badDeadCreatureRead = Delayed (DiesThisTurn (Target creature))
                                (Move (That (Perm Creature)) BattlefieldZ)

-- "The chosen type" with only a color chosen: the quality read is
-- sort-filtered — no witness.
failing "countQuality"
  badChosenWrongSort : Effect []
  badChosenWrongSort = AndThen (Choose (A (QualityNoun Color)))
                               (destroy (AllOf (And [creature, Not (OfChosen CreatureType)])))

-- Two group mentions leave "them" ambiguous — the plural wildcard has
-- the same strict-uniqueness gate as the singular.
failing "countManys"
  badThemAmbig : Effect []
  badThemAmbig = AndThen (Choose (TargetGroup 2 creature))
                         (AndThen (Choose (TargetGroup 2 creature))
                                  (Tap Them))

-- Two predicate-inner opponents leave "that player" ambiguous — the
-- uniqueness gate reaches inside relative clauses too. (Not
-- oracle-legal text; the guide would repeat the noun.)
failing "countCarrier"
  badInnerAmbig : Effect []
  badInnerAmbig = AndThen (Fights (Target (And [creature, ControlledBy anOpponent]))
                                  (Target (And [creature, ControlledBy anOpponent])))
                          (losesLife (That PlayerC) (Lit 1))
