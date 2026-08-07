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
||| or a `failing` negative in `Experimental.Cards`):
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
|||    silent zero (the old fused-carrier derivation this finding named
|||    is decomposed in finding 28 — quality mentions now simply fail
|||    every noun word). "All" is a surface determiner distinct from
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
||| Chapter seven, the payload split (the ruling is recorded in the
||| decision record §3):
|||
||| 25. **Context data is kind-indexed.** `Payload k` gives each kind
|||    exactly its own data — objects a head type and zone fold-state,
|||    players and qualities nothing — so a binding cannot record what
|||    its kind cannot have: "a player in your hand" is unrepresentable
|||    in the CONTEXT (`badPlayerInHand`) exactly as it is in the
|||    surface grammar (`InZone` is Object-kinded), and a junk write (a
|||    zone retag on a player) is refused by construction instead of
|||    absorbed by a dead field. The per-kind matches keep the
|||    no-catch-all discipline: a new `Payload` constructor is a
|||    totality error in `carrier`/`bindingZone`/`pubB`/`setZone`, not
|||    a silent pass-through.
|||
||| Chapter eight, definite reads (evidence: Voyager Staff, Bosh, Iron
||| Golem, Pyromancy; corpus: participles of tagged verbs are frequent
||| — sacrificed/exiled/discarded — while untagged verbs' are near
||| absent: returned/destroyed/tapped):
|||
||| 26. **The participle is a provenance filter.** A tagged move
|||    stamps its verb on the binding it retags (`ObjectP`'s third
|||    component — object-only, which the payload split makes free); a
|||    bare move clears it, so the participle names the LAST verb
|||    event. "The [verbed] [noun]" (`TheVerbed`) reads the unique
|||    stamped mention: where the bare demonstrative is ambiguous —
|||    Voyager Staff's effect holds two card mentions, the sacrificed
|||    self and the exiled target — oracle switches to the participle,
|||    exactly as finding 12 predicted (`badBareCardRead`). Provenance
|||    follows the TAG channel: the dying retag stamps nothing
|||    ([CR#700.4] is an event, not a keyword action), matching the
|||    corpus absence of "the destroyed …" reads.
||| 27. **The participle noun keeps the word axes apart.** Ruling:
|||    card types/subtypes/supertypes are intrinsic to NEITHER core
|||    nor semantics — each is a macro-DECLARED catalog word carrying
|||    its rules grants (`cardtype/Creature.ron`, down to
|||    `permanent: true`). So `NounWord` splits the axes: a TYPE word
|||    checks the time-stable projected head type ("the sacrificed
|||    artifact" — Bosh: a graveyard card NOW, an artifact under the
|||    verb), the intrinsic CARD word checks current zone fold-state
|||    ("the exiled card" — Voyager Staff); both attested, uniqueness
|||    supplied by the verb filter.
||| 28. **The demonstrative shares the split word vocabulary.**
|||    `That`/`Those` take the same `NounWord`, anchored to the
|||    CURRENT state where the participle anchors to the verb event
|||    (`wordNow` vs `wordOk`), and the fused `Carrier` type dissolves
|||    — a type word checks projected type + battlefield, the
|||    intrinsic CARD and PLAYER words check zone and kind. Same
|||    refusals as before (the carrier negatives re-pin on
|||    `countWord`); no fused type-carrier value remains in the model.
|||
||| Not settled yet: the kind union ("any target" spans objects and
||| players [CR#115.4,115.1] — elided to `Object`);
||| owned-zone mentions beyond `You` ("its owner's hand" — destination
||| and predicate-inner mentions do not fold yet); controller
||| fold-state (the controller half of verb restrictions, entangled
||| with [CR#109.4]); `May`'s if-you-do / if-not branches (the Risk
||| Factor / Rakdos, Patron of Chaos else-clauses); the chosen-OBJECT
||| definites ("the chosen creatures" — V.A.T.S./Victimize wait on
||| "any number of" groups and if-you-do); plural participle reads
||| ("the exiled cards", Hide on the Ceiling — group twins of
||| `TheVerbed`); the PERMANENT word (demonstrative "that permanent"
||| and participle "the sacrificed permanent", Broadside Bombardiers —
||| the before-state question and its "or" disjunction; no bench card
||| spells either, so `NounWord` waits to grow it); the
||| additional-cast-cost juncture (Fling — "the sacrificed creature"
||| across a casting cost, the same public-survivors discipline as
||| the colon, constructor unminted); more event queries (upkeep / end-of-combat /
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
||| a possible later simplification — less pressing since the payload
||| split gave each kind exactly its own data.
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
data CardType = Creature | Artifact | Land

||| Quality sorts — the choosable characteristics ([CR#105.1,302.3];
||| only what the chapters need).
public export
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

||| Keyword-action tags ([CR#701]) — core's `Composite` verb names.
||| `Destroy` and `Discard` mirror `plugins/builtin/macros/action/`;
||| `Sacrifice` is a core whittling candidate (see the decision record);
||| `Exile` is speculative pending its real macro definition. (Its own
||| namespace: the tag `Exile` and the zone `Exile` are distinct words.)
namespace Verb
  public export
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
            (prov : Maybe VerbName) -> Payload Object
  PlayerP : Payload Player
  QualityP : Payload (Quality q)

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
||| the participle against the verb event's frame (`wordOk`). Token,
||| spell, and stack-object words are later chapters.
public export
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

public export
wordOk : NounWord -> Maybe CardType -> Maybe Zone -> Bool
wordOk (TypeW t) ty zn = tyIs t ty
wordOk CardW ty zn = isCardZone zn
wordOk PlayerW ty zn = False  -- no participle reads a player ("the sacrificed player" is unwritten)

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
wordNow CardW (MkBinding _ _ _ (ObjectP _ zn _)) = isCardZone zn
wordNow CardW (MkBinding _ _ _ PlayerP) = False
wordNow CardW (MkBinding _ _ _ QualityP) = False
wordNow PlayerW (MkBinding _ _ _ (ObjectP _ _ _)) = False
wordNow PlayerW (MkBinding _ _ _ PlayerP) = True
wordNow PlayerW (MkBinding _ _ _ QualityP) = False

public export
kindOfW : NounWord -> Kind
kindOfW (TypeW _) = Object
kindOfW CardW = Object
kindOfW PlayerW = Player

||| Does "the [verbed] [noun]" reach this binding? Singular, stamped
||| with that verb tag, noun word compatible.
public export
verbedMatch : VerbName -> NounWord -> Binding -> Bool
verbedMatch v w (MkBinding _ _ OneOf (ObjectP ty zn (Just v'))) =
  sameVerb v v' && wordOk w ty zn
verbedMatch v w (MkBinding _ _ OneOf (ObjectP _ _ Nothing)) = False
verbedMatch v w (MkBinding _ _ ManyOf (ObjectP _ _ _)) = False
verbedMatch v w (MkBinding _ _ _ PlayerP) = False
verbedMatch v w (MkBinding _ _ _ QualityP) = False

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
    MkBinding det Object plur (ObjectP (seedTy p) (Just (zoneOr Battlefield (seedZone p))) Nothing)
  bindFor det plur {k = Player} p = MkBinding det Player plur PlayerP
  bindFor det plur {k = Quality q} p = MkBinding det (Quality q) plur QualityP

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
    -- "those [noun word]s": the sorted plural demonstrative — exactly
    -- one group mention its word currently reaches.
    Those : (w : NounWord) -> {auto 0 ok : countManyWord w bs = 1} -> Noun bs (kindOfW w)
    -- "that [noun word]": the sorted demonstrative — exactly one
    -- mention its word currently reaches may precede (`wordNow` — the
    -- current-state anchoring; the participle read anchors the same
    -- words to the verb event instead).
    That : (w : NounWord) -> {auto 0 ok : countWord w bs = 1} -> Noun bs (kindOfW w)
    -- "the [verbed] [noun]" ("the exiled card", "the sacrificed
    -- artifact"): the definite participle read — exactly one mention
    -- stamped by that verb tag and reached by the noun word may
    -- precede. The disambiguator real text switches to where a bare
    -- demonstrative would be ambiguous (finding 26).
    TheVerbed : (v : VerbName) -> (w : NounWord) ->
                {auto 0 ok : countVerbed v w bs = 1} -> Noun bs Object
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
    -- "[n]'s mana value" / "the mana value of [n]" ([CR#202.3]).
    ManaValueOf : Noun bs Object -> Amount bs
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
  amtIntro (ManaValueOf nom) = nomIntro nom
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
  delayedCtx (DiesThisTurn n) = settleTargets (moveIntro Nothing n Graveyard)

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
    -- only a Destroy-tagged move IS a destruction). A tagged move also
    -- stamps its referent's provenance — the participle read's filter
    -- (finding 26).
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
  ||| uniqueness is what makes this well-defined). The retag writes the
  ||| new zone AND the moving verb's tag (provenance — `Nothing` for an
  ||| untagged move: the participle names the LAST verb event). Player
  ||| and quality clauses are identity — unreachable from card terms
  ||| (`Move` is Object-kinded), kept explicit for totality.
  public export
  setZone : Maybe VerbName -> Zone -> Binding -> Binding
  setZone p z (MkBinding det Object plur (ObjectP ty _ _)) =
    MkBinding det Object plur (ObjectP ty (Just z) p)
  setZone p z (MkBinding det Player plur PlayerP) = MkBinding det Player plur PlayerP
  setZone p z (MkBinding det (Quality q) plur QualityP) =
    MkBinding det (Quality q) plur QualityP

  public export
  setZoneHead : Maybe VerbName -> Zone -> Bindings -> Bindings
  setZoneHead p z [] = []
  setZoneHead p z (b :: bs) = setZone p z b :: bs

  public export
  setZoneIt : Maybe VerbName -> Zone -> Bindings -> Bindings
  setZoneIt p z [] = []
  setZoneIt p z (MkBinding det Object OneOf (ObjectP ty zn _) :: bs) =
    MkBinding det Object OneOf (ObjectP ty (Just z) p) :: bs
  setZoneIt p z (b :: bs) = b :: setZoneIt p z bs

  public export
  setZoneThem : Maybe VerbName -> Zone -> Bindings -> Bindings
  setZoneThem p z [] = []
  setZoneThem p z (MkBinding det Object ManyOf (ObjectP ty zn _) :: bs) =
    MkBinding det Object ManyOf (ObjectP ty (Just z) p) :: bs
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
  moveIntro p (Target pr) z = setZoneHead p z (nomIntro (Target pr))
  moveIntro p (Each pr) z = setZoneHead p z (nomIntro (Each pr))
  moveIntro p (A pr) z = setZoneHead p z (nomIntro (A pr))
  moveIntro p (ATheirChoice pr) z = setZoneHead p z (nomIntro (ATheirChoice pr))
  moveIntro p (AAtRandom pr) z = setZoneHead p z (nomIntro (AAtRandom pr))
  moveIntro p (TargetGroup n pr) z = setZoneHead p z (nomIntro (TargetGroup n pr))
  moveIntro p (TargetUpTo n pr) z = setZoneHead p z (nomIntro (TargetUpTo n pr))
  moveIntro p (AllOf pr) z = setZoneHead p z (nomIntro (AllOf pr))
  moveIntro p It z = setZoneIt p z bs
  moveIntro p Them z = setZoneThem p z bs
  moveIntro p (That w) z = setZoneThat p w z bs
  moveIntro p (Those w) z = setZoneThose p w z bs
  moveIntro p (TheVerbed v w) z = setZoneVerbed p v w z bs
  moveIntro p This z = bs
  -- a moved sorted self-reference mints the new object's binding
  -- ([CR#400.7]; see the constructor comment).
  moveIntro p (ThisOf t) z = MkBinding TheD Object OneOf (ObjectP (Just t) (Just z) p) :: bs
  moveIntro p You z = bs
  moveIntro p They z = bs
  moveIntro p (ControllerOf n) z = nomIntro (ControllerOf n)
  moveIntro p (OwnerOf n) z = nomIntro (OwnerOf n)

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
  nounZone (That w) = zoneOfThat w bs
  nounZone (Those w) = zoneOfThose w bs
  nounZone (TheVerbed v w) = zoneOfVerbed v w bs
  nounZone (ControllerOf n) = Nothing
  nounZone (OwnerOf n) = Nothing

  ||| What a clause contributes to the discourse that follows it.
  public export
  effIntro : {bs : Bindings} -> Effect bs -> Bindings
  effIntro (DealDamage src amt to) = nomIntro to
  effIntro (Fights a b) = nomIntro b
  effIntro (Tap n) = nomIntro n
  effIntro (Choose n) = nomIntro n
  effIntro (Move what to) = moveIntro Nothing what (zoneSort to)
  effIntro (ChangeLife who op) = lifeIntro op
  effIntro (Gain n _ _) = nomIntro n
  effIntro (Gets n _ _ _) = nomIntro n
  effIntro (Composite v (Move what to)) = moveIntro (Just v) what (zoneSort to)
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

-- "[agent] discard(s) [n]" — the hand→graveyard move [CR#701.9a] with
-- the subject in clause position. The CR routes by the card's OWNER;
-- owner≡agent is this macro's elision, the same one the real macro
-- makes with its agent-param hand filter. The noun carries its own
-- choice marking: the plain indefinite is the affected player's
-- choice by default [CR#701.9b], "at random" the markedly chooserless
-- variant (`AAtRandom` — Pyromancy).
public export
discards : (agent : Noun bs Player) -> (n : Noun (nomIntro agent) Object) -> Effect bs
discards agent n = Does agent (Composite Discard (Move n GraveyardZ))

-- "[agent] discard(s) a card" — the common phrase, spelled sort-only
-- (an owned-hand expansion needs a subject-read noun the vocabulary
-- lacks — not-settled).
public export
discardsACard : (agent : Noun bs Player) -> Effect bs
discardsACard agent = discards agent (A (InZone HandZ))

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
