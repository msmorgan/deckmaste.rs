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
||| 3. **"Other" is a modifier with a presupposition** (its anchor set
|||    completed by the second-wave audit, head-TYPED by the third —
|||    finding 44). Oracle's "any other
|||    target" ([CR#115.4]) is the modifier `Other` inside one noun's
|||    flat modifier set — distinct from its ANCHOR, with the
|||    obligation that an anchor exist. The anaphoric anchor is any
|||    earlier same-kind target mention (`anyTargeted`; up-to
|||    mentions count, finding 35) of a compatible head type
|||    (`OtherAnchored`); the second mode anchors on the
|||    SOURCE ("Olivia Voldaren deals 1 damage to another target
|||    creature", Red Hulk's "…to any other target" — no earlier
|||    target exists, "other" excludes the source), which waits on the
|||    source entering the discourse (ledger). Anchorless "other" stays UNSPELLABLE
|||    (`badOther`): textual precedence replaces the old
|||    sibling-index `Distinct` constructor and its range gate.
|||    Unconstrained slots still legally share an object
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
||| Browbeat/Risk Factor for the decider split. The clause SEQUENCE is
||| n-ary, as core's `OneShotEffect::Sequentially` is: a card writes a
||| LIST of sentences, and the binary `AndThen` this replaced imposed a
||| right-nested tree no card has. Its elements are a TELESCOPE, each
||| typed in what its predecessors introduced — finding 1's threading
||| made a constructor — and it runs two clauses at minimum, one being
||| the clause itself and none being no instruction at all
||| (`badSingletonSequence`, `badEmptySequence`). Its elements are
||| clauses and not sequences: nesting one re-mints the very tree the
||| list replaced, spelling a three-sentence card twice
||| (`NotSeq`, `badNestedSequence`), while the slots that take a BODY
||| — a "may" arm, a delayed clause, a tagged composite — go on
||| holding whatever the card brackets there):
|||
||| 9. **Subjects are the factored who-slot.** Verbs the CR gives a
|||    player actor ([CR#701.21a,701.9a]) put their performer in clause
|||    position (`Does`), typed before their phrase, and REQUIRE it;
|||    the imperative supplies an explicit `You`; effect-verbs take a
|||    subject OPTIONALLY (finding 38's correction — destroy and exile
|||    spell both ways), and an
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
|||    written-Nothing convention — no defaults. WHICH duration a
|||    clause may state is not free, though: it is the construction's
|||    own word (finding 55).
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
|||    sequential family (`karplusanYeti`) is not equivalent — it deals
|||    two ORDERED damage events where [CR#701.14a] deals both
|||    SIMULTANEOUSLY (event granularity is observer-dependent,
|||    [CR#700.1]); the order is observable to triggers and
|||    replacements, while state-based actions see neither
|||    mid-resolution ([CR#704.3,704.4] — the audit's correction of
|||    this finding's original rationale).
|||
||| Chapter six, group reference and qualities (evidence: Fulgent
||| Distraction, Continue?, Sudden Demise, Kindred Dominance; the
||| amounts axis stayed evidence-only — see the not-settled list):
|||
||| 21. **Groups are bindings like any other.** A counted target
|||    mention ("two target creatures", "up to four …") binds once, and
|||    its quantity is ARITY data: the only thing any consumer reads off
|||    it is grammatical number (`quantPlur`), never a membership fact
|||    ([CR#601.2c] fixes the count at announce, and the one read that
|||    wants a number wants the ACTUAL count, not the bound).
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
||| Chapter nine, verb frames and agreement (an adversarial audit —
||| Codex and agy probed for terms that typecheck but should not;
||| every accepted finding is now a gate plus a pinned negative):
|||
||| 29. **Verb slots demand their frames.** Damage recipients are
|||    players or battlefield objects, never qualities ([CR#120.1] —
|||    `badDamageGraveyardCard`, `badDamageToColor`); tap, fight,
|||    destroy, and the dying query take battlefield referents, fight
|||    creatures specifically
|||    ([CR#701.26a,701.14b,701.8a,700.4] — `badTapGraveyard`,
|||    `badFightGraveyard`, `badFightLand`, `badDestroyGraveyard`,
|||    `badDiesInGraveyard`, `badGetsGraveyard`); discard takes a hand
|||    card ([CR#701.9a], `badDiscardBattlefield`); "target" takes
|||    objects and players only ([CR#115.1], `badTargetColor`); move
|||    destinations are owner-routed, so only bare zone destinations
|||    are writable ([CR#400.3], `badMoveToTargetsHand`); and a tag
|||    agrees with its body — a Destroy-tagged exile is unspellable
|||    ([CR#701.8b,702.12b], `badDestroyTaggedExile`). The participle
|||    type word likewise demands its referent stood on the
|||    battlefield AT the verb (the stamp's `wasField`): "the
|||    discarded creature" is unwritten (`badDiscardedCreatureWord`).
||| 30. **Reads agree in number.** Possessive amounts and relational
|||    nouns take singular arguments ([CR#208.1] — power/toughness
|||    are one object's numbers; aggregates are explicit, "the total
|||    power of the sacrificed creatures", Soulblast; owners are per
|||    card [CR#108.3], "their owners' hands", Aether Burst, being
|||    the plural relational, future vocabulary): `nounPlur` projects
|||    grammatical number (`badGroupPower`, `badGroupOwner`). Counted
|||    mentions refine: the quantity is a written one, so its maximum
|||    is at least one (`badZeroGroup`), and the number it projects is
|||    its own — "up to one" and the exact one both bind singular (Ty
|||    Lee, Chi Blocker's "It" remention; `quantPlur` reads the maximum,
|||    which is what both spellings share).
|||    This finding's original refusal of up-to mentions as "other"
|||    witnesses is repealed by finding 35.
||| 31. **Marked clauses carry their obligations.** "Of their choice"
|||    demands exactly one player antecedent — a subject or one
|||    distributive group ([CR#608.2c..608.2d];
|||    `badUnboundTheirChoice`). This finding's companion claim — that
|||    `Does` admits only agentive clauses — is corrected by finding
|||    38: the actor is REQUIRED for sacrifice and discard
|||    ([CR#701.21a,701.9a]) and OPTIONAL for destroy and exile.
|||
||| Chapter ten, event outcomes (evidence: Foul-Tongue Shriek; the
||| recon corpus — "that much" antecedents are damage, life, and mana
||| both produced AND spent (Tellah, Great Sage: "If eight or more
||| mana was spent to cast that spell, sacrifice Tellah and it deals
||| that much damage to each opponent." — the if-condition supplies
||| the scalar, the third-wave audit's correction of this header);
||| "that many" consumers are counters, draws, and tokens; "this way"
||| is the largest family of all):
|||
||| 32. **Event outcomes are the third binding class, realized.** An
|||    event clause prepends an outcome mention — kind `Outcome`,
|||    payload its surface-projected SORT alone (damage dealt, life
|||    gained/lost); the magnitude stays runtime (§3's ruling).
|||    `ThatMuch` reads the unique outcome in scope as an amount,
|||    SORT-BLIND — the corpus crosses damage→life, count→life,
|||    damage→mana — under the same strict uniqueness as every read
|||    (`badThatMuchUnbound`, `badThatMuchAmbig`). Only clauses
|||    introduce outcomes: no determiner phrase binds one (`Phrasal`
|||    keeps `bindFor` total), and every existing read's kind filter
|||    excludes them for free — the payload architecture absorbing a
|||    fourth kind without touching object data.
||| 33. **Countability belongs to the read site.** "That much" vs
|||    "that many" is the reading phrase's choice, not stored data —
|||    both consume the same event class; the count reads and the
|||    "this way" participant subsets wait on their consumer
|||    vocabulary. Overgeneration accepted: an outcome follows a
|||    literal-amount event too, though oracle style repeats the
|||    literal instead of writing "that much" — linearization's
|||    business, like extraposition; and Fights/Move events stay
|||    outcome-silent until a read wants them.
|||
||| Chapter eleven, owner-rooted zones (evidence: Unsummon; the
||| morning rulings):
|||
||| 34. **The destination possessive is derived, not stored.** A card
|||    reaches only its owner's hand/library/graveyard — [CR#400.3]
|||    redirects any other — so "to its owner's hand" adds nothing to
|||    the zone sort: the bare zone IS the owner-rooted destination,
|||    `DestOk`'s bare-zone gate is the complete destination grammar
|||    for those zones (an owned destination naming ANY chooser stays
|||    unwritable — `badMoveToTargetsHand`), and rendering re-adds the
|||    possessive ("your hand" only where the owner is contextually
|||    You, as in search effects). PREDICATE possessives contrast: "a
|||    card in an opponent's graveyard" FILTERS by owner — the same
|||    rule fuses zone and ownership into one fact — so
|||    `HandOf`/`GraveyardOf` stay real inside predicates. And the
|||    other parked gate closes the opposite way: a self-fight is
|||    DEFINED — [CR#701.14c] has it deal twice its power to itself —
|||    so distinctness is per-card templating ("another", the `Other`
|||    modifier) and `Fights` takes no distinctness gate, ever.
|||
||| Chapter twelve, the second wave (evidence: Phantom Blade; the
||| audits' verified corrections, corpus- and CR-checked one by one):
|||
||| 35. **"Other"'s witness is the mention, not the denotation.**
|||    Phantom Blade pairs an up-to-one target with "up to one OTHER
|||    target creature": the anchor SLOT exists even when its
|||    denotation may be empty ([CR#115.6]), distinctness over an
|||    empty anchor is vacuous, and distinctness stays predicate
|||    content ([CR#601.2c]). `anyTargeted` counts up-to mentions;
|||    finding 30's contrary refusal is repealed. That the two spellings
|||    are one witness is why they now share one determiner tag — the
|||    emptiness is the quantity's, not the word's.
||| 36. **Grammatical number gates the argument slots.** The binary
|||    fight frame takes two singular combatants ([CR#701.14a];
|||    every corpus "X fights Y" line is singular — the plural form
|||    is the RECIPROCAL frame "those creatures fight each other",
|||    its own future construction); possessors are singular (one
|||    controller [CR#109.4]; per-player zones [CR#400.1] — the
|||    union read "creatures your opponents control" waits with the
|||    player groups, "their owners' hands" with the plural
|||    relationals); the minted dies-watcher is singular; and a
|||    counted mention carries its number in its QUANTITY. One
|||    constructor serves every quantity (`TargetGroup`, mirroring
|||    core's single `TargetSpec::Target(Quantity, Predicate)` announce
|||    form) and the named forms are macros over the one `Range`
|||    primitive exactly as core's are — `target p = TargetGroup
|||    (exactly 1) p`, `upTo n`, `anyNumber` — so both the singular and
|||    the up-to phrase are spellings of one mention, and two-up
|||    survives as a RENDERING rule rather than a type floor: the
|||    numeral is unwritten at one ("target creature", never "one target
|||    creature") and written from two ("two target creatures"). What
|||    the type still refuses is a quantity that admits nothing or
|||    reads backwards: the MAXIMUM is at least one (`NonZeroQ`,
|||    `badZeroGroup`), which kills "up to zero" in the same stroke as
|||    "zero", and the range runs UPWARD from a written minimum of at
|||    least one (`WellFormedQ`) — a descending pair names an empty
|||    interval and would read as plural besides, and a zero minimum
|||    spells what "any number of" already spells, [CR#107.1c] having
|||    "any number" permit zero outright (`badDescendingRange`,
|||    `badZeroLowerRange`). Fight
|||    participation reads a GRANT, not a type name: `combatant` is
|||    the stand-in for a TypeDef-declared combat-participant grant
|||    — distinct from May(Attack)/May(Block), fight keying on type
|||    membership and dealing non-combat damage [CR#701.14d] — so a
|||    future type joins by declaration, not by gate rewrites.
||| 37. **Obligations live on relations, not phrases.** The tag-body
|||    witness carries each verb's SOURCE-zone demand ([CR#701.8a]
|||    destruction moves a battlefield permanent, [CR#701.9a]
|||    discarding a hand card; exile zone-blind), so raw
|||    `Composite`/`Does` spellings prove exactly what the macros
|||    prove (`badCompositeDestroyGraveyard`,
|||    `badDoesDiscardBattlefield`) and a forged provenance stamp is
|||    unwritable — only a legal tagged move writes one. The damage
|||    recipient likewise gained its head-type demand ([CR#120.1a];
|||    `badDamageArtifact`).
||| 38. **Agentivity is one table, read by its MISSING rows** (the
|||    third-wave audit's correction: the original claimed two tables
|||    and refused a subject on destroy — an actor is required
|||    exactly where `NonAgentive` has no row). `Does` carries the
|||    verb tag itself and demands nothing of the verb; `Composite`
|||    demands `NonAgentive`, so sacrifice and discard cannot shed
|||    their actor ([CR#701.21a,701.9a]; `badAgentlessSacrifice`)
|||    while destroy and exile take one OPTIONALLY — real oracle text
|||    writes both, "You destroy four lands you control, then target
|||    opponent destroys four lands they control." (Burning of Xinye)
|||    and "Each player exiles two cards from their hand." (The
|||    refused negative went with it; the Burning of Xinye positive
|||    waits on counted untargeted groups.) Overgeneration accepted:
|||    a choice-free subject form ("You destroy target creature") is
|||    spellable. The old conservative `clauseVerb` catch-all
|||    dissolves with nothing left to leak.
||| 39. **Phrases are well-formed or unwritable.** Determiner
|||    phrases demand a positive HEAD (`Headed` — "choose a
|||    noncolor" and "target non-player" head nothing,
|||    [CR#105.1,608.2d]); a conjunction's explicit zones must agree
|||    (`ZoneCoherent` — an object is in one zone, and the gate no
|||    longer depends on conjunct order); and negation is a binding
|||    HOLE (`Not`'s inner mentions do not fold: "a creature an
|||    opponent doesn't control" names no opponent — a positive
|||    relation names the one controller [CR#109.4], a negated one
|||    selects nobody; `badNegatedAntecedent`).
|||
||| Chapter thirteen, the third wave (both ducks probing for terms
||| that typecheck but should not; every accepted finding is a gate
||| plus a pinned negative, and each negative is probed at the
||| SMALLEST construct carrying its gate — an auto-search failure
||| nested inside another auto stalls the outer search and misreports,
||| so the bench writes bare `Predicate`/`Amount`/`Noun` probes
||| wherever one exists):
|||
||| 40. **"Any target" is a lone class word.** The guide reserves it
|||    for the rules-defined damage target class ([CR#115.4]) and
|||    forbids it as a synonym for "any object". Four refusals
|||    follow: it takes no modifier but "other" (Arc Trail's "any
|||    other target" is the sole corpus companion — `AnyTargetLone`,
|||    `badAnyTargetInGraveyard`); it is never negated, there being no
|||    complement class to name (`Negatable`, which also refuses
|||    double negation — `badNegatedAnyTarget`); and it is ITSELF the
|||    targeting form, so the non-targeting determiners and the
|||    for-each domain demand an any-target-free phrase — "a any
|||    target" and "each any target" are unwritable (`AnyTargetFree`,
|||    `badAnyTargetUnderA`); and it DESCRIBES no object, so
|||    [CR#109.2] has nothing to place on the battlefield and the
|||    phrase projects no zone — which is what refuses it to every
|||    battlefield-demanding verb (`nounZone`, `badDestroyAnyTarget`,
|||    `badTapAnyTarget`) while damage still takes it, by a recipient
|||    row read off the NOUN (`DamageRecipient`, `badDamageThis`).
|||    What is stated here is the SINGULAR form's rule:
|||    [CR#115.4] names "another target," "two targets," and similar
|||    plural damage-class forms in the same breath, and those carry
|||    their own structures (a division, an each-of recipient —
|||    ledger), so "lone class word" constrains "any target" itself,
|||    not the family it belongs to. That makes the third refusal a
|||    question of the QUANTITY once the counted mention is one
|||    constructor (finding 36): the class word belongs at exactly
|||    one, where the phrase IS the singular form ("Lightning Bolt
|||    deals 3 damage to any target"), and at any up-to bound, which
|||    the corpus writes outright ("each of up to two targets", Fall
|||    of the Titans); the exact group from two up and the unbounded
|||    "any number of" are refused until the plural structures land
|||    — `AnyTargetAtCount`, `badGroupAnyTarget`,
|||    `badAnyNumberAnyTarget`. What those two quantities permit is the
|||    class word as the phrase's HEAD, not the word wherever it hides:
|||    a possessor buried in a relative clause spells it under a
|||    counted mention exactly as it does under "a"
|||    (`badEmbeddedAnyTargetExact1`), so the embedded scan runs
|||    beneath every quantity.
||| 41. **The sorted self-reference stands on the battlefield.** "This
|||    creature" / "this enchantment" is a description including a
|||    card type, so [CR#109.2] denotes the PERMANENT: `ThisOf`
|||    projects the battlefield where bare `This` — the source as an
|||    object ("this spell", cycling's "Discard this card") — projects
|||    nothing. Discarding the sorted form is thereby unwritable
|||    ([CR#701.9a] moves a HAND card; `badDiscardThisCreature`), and
|||    the cost that justifies discard's bare-`This` row is cycling's
|||    own ([CR#702.29a]; `cyclingCost`) — the row now sits on the NOUN
|||    (`DiscardOk`), so it exempts that one word rather than every
|||    referent whose zone happens to be untracked (`badDiscardIt`).
|||    The audit then closed the
|||    opposite permissive row for good: `OnBattlefield`'s untracked
|||    constructor existed ONLY for the sorted self-reference, so
|||    deleting it left the whole bench standing — every
|||    battlefield-demanding slot now demands the battlefield, full
|||    stop. (`Enchantment` joined the type catalog in the same pass:
|||    Pyromancy's "This enchantment deals …" had been spelled with
|||    bare `This` as a workaround for the missing row.)
||| 42. **A phrase may not contradict itself.** Zone negation is real
|||    oracle — "Each Vampire creature card you own that isn't on the
|||    battlefield has madness." (Falkenrath Gorger) — so `Not (InZone
|||    …)` stays writable; what a conjunction cannot do is rule OUT
|||    the zone it places its referent in, [CR#109.2]'s battlefield
|||    default included (`ZoneCoherent`, `badNotOnBattlefield`), nor
|||    hold a member that syntactically negates a sibling
|||    (`ContradictionFree`, `badQualityContradiction`). Both scans
|||    FLATTEN nested conjunctions, so a clash one level down is no
|||    laundering (`badNestedContradiction`). The member equality they
|||    share is deliberately conservative on the rows carrying a noun:
|||    "not provably the same" under-refuses rather than over-refuses.
|||    It is NOT conservative on the rows carrying structure, though it
|||    began that way: a conjunction compares member by member, so the
|||    same phrase written twice is recognized wherever the equality is
|||    consulted — including as an alternative repeated word for word,
|||    which the blanket row waved through (`badRepeatedStructuredDisjunct`).
||| 43. **Status words seed the zone they presuppose.** Attackers are
|||    declared from creatures their controller controls
|||    ([CR#508.1a]), and leaving the battlefield removes a permanent
|||    from combat — it "stops being an attacking … creature"
|||    ([CR#506.4]) — so `Attacking` projects the battlefield exactly
|||    as a zone clause does. The nonsense that follows ("an
|||    attacking creature in your hand") is then refused by the
|||    EXISTING coherence gate rather than a new one
|||    (`badAttackingInHand`) — the cheapest shape a finding can take:
|||    a projection made honest, and the refusal falls out.
||| 44. **"Other" is typed by its own head.** The anchor obligation
|||    splits in two: `Other`'s own gate demands a same-KIND target
|||    mention, and the conjunction it sits in supplies the head TYPE
|||    that mention must be compatible with (`OtherAnchored`; an
|||    untyped head — Arc Trail's "any other target", a player-kind
|||    "other" — accepts any same-kind anchor, which is exactly
|||    finding 3's gate). The head is a SET, once a coordination can
|||    write one: "another target creature or land" fixes no type on
|||    its referent (finding 50) but offers TWO heads to anchor
|||    against, and reading that silence as untypedness let an earlier
|||    artifact anchor it (`badDisjunctiveOtherCrossHead`) — silence as
|||    permission, the third time. The guide reserves "another" for excluding
|||    the source or first referent and writes two separately
|||    described roles WITHOUT it ("target creature and target
|||    planeswalker"); the corpus pairs "other" only with overlapping
|||    heads. Sharing one object across disjoint-headed slots stays
|||    rules-LEGAL ([CR#601.2c]'s artifact-land example), which is
|||    precisely why this is templating enforced where the head type
|||    is known (`badOtherCrossHead`).
||| 45. **The counted-set amount is a noun phrase.** Every corpus
|||    for-each domain is noun-HEADED — "for each you control" names
|||    no set to count (`Headed`, `badForEachHeadless`) — and its
|||    per-unit is a written numeral, hence at least one
|||    (`AtLeastOne`, `badForEachZero`): the same surface-numeral
|||    discipline finding 36 gives counted groups. The comparisons
|||    that legitimately carry zero READ a count rather than write
|||    one, so `Lit` stays ungated.
||| 46. **Damage subjects distribute; they do not collect.** A group
|||    source is legal exactly when the group is distributive: "Each
|||    creature you control deals 1 damage to that creature." (Case of
|||    the Gateway Express) spreads the singular deal frame over its
|||    members, while a COLLECTIVE plural subject ("two target
|||    creatures deal …") is unattested — oracle either distributes or
|||    names one source (`DamageSource`, `badGroupDamageSource`). The
|||    distributive lines carrying a per-member "its" ("Each creature
|||    deals 1 damage to its controller.") wait on dependent iteration,
|||    which is why the bench positive reads its recipient back as a
|||    demonstrative instead.
|||
||| Chapter fourteen, disjunction (evidence: Disenchant, Icy
||| Manipulator, Rats of Rath, Arrows of Justice; the corpus counts and
||| the bracketed parses are this chapter's whole argument):
||| 47. **A disjunction is a PREDICATE, not a second noun.** Disenchant
|||    writes "target" once — "Destroy target artifact or enchantment."
|||    — so the phrase announces ONE target ([CR#601.2c], whose own
|||    example turns on a spell using the word in two places), and the
|||    parse agrees about where the coordination sits:
|||    `<<target> <<artifact> <or <enchantment>>>>`, inside the
|||    determined phrase rather than beside it. That places "or" among
|||    the descriptions, where the kind index already forces the
|||    alternatives to describe the same sort of thing and no gate has
|||    to say so. `Or` carries a member LIST for the reason `And` does,
|||    and core's `Predicate::Or` does (`filter.rs`, a slice beside
|||    `And`'s): Icy Manipulator's third alternative then costs no
|||    machinery, only the guide's serial comma. Two-alternative
|||    "target X or Y" runs to about a thousand corpus lines and the
|||    Oxford form to eighty-eight, so this is a first-rank
|||    construction, not a curiosity.
||| 48. **Modifier scope falls out of the nesting.** "Destroy target
|||    artifact, creature, or land you control." (Rats of Rath) is
|||    `And [Or […], ControlledBy You]` — the relative clause a SIBLING
|||    of the coordination, which is exactly how the parse brackets it
|||    (`<<target> <<artifact><, creature><, or land>> <<you>
|||    <control>>>`). One hundred sixty-six corpus lines write the
|||    shared trailing modifier and it needed nothing new; the other
|||    reading, the clause repeated per alternative, is what the guide
|||    reserves for "alternatives with different domains or modifiers".
||| 49. **The member scans must not flatten through "or".**
|||    `flattenPs` flattens conjunctions only. Splicing alternatives
|||    into the surrounding conjunction would read Arrows of Justice's
|||    "attacking or blocking creature" as "attacking AND blocking" and
|||    refuse ninety-five corpus lines, so the coherence gates see a
|||    disjunction through its PROJECTIONS instead — and those project
|||    what the alternatives AGREE on. Both status words stand on the
|||    battlefield and both presuppose a creature ([CR#506.3] names
|||    them in one breath), so the disjunction does too, and the
|||    nonsense around it is refused by the gates that were already
|||    there (`badAttackingOrBlockingInGraveyard`,
|||    `badNoncreatureAttackingOrBlocking`) — finding 43's shape a
|||    second time. Not flattening puts the whole weight on those
|||    projections, so they have to reach as far as the flattening
|||    would have: a presupposition written inside a nested
|||    CONJUNCTION, which an alternative may hold, is written all the
|||    same, and the type seed had no such row until an "attacking
|||    artifact or blocking land" laundered one past the negation
|||    (`badWrappedStatusLaunder`).
||| 50. **A disjunctive head is untyped, and the untyped row had to
|||    go.** "Artifact or enchantment" fixes no card type, so `seedTy`
|||    projects none — the honest silence. `DamageableTy` used to read
|||    a missing type word as no evidence of an illegal one, which was
|||    defensible only while nothing projected `Nothing` deliberately;
|||    a disjunction does, and the row would have made every
|||    disjunction damageable, including the very types [CR#120.1a]
|||    says damage cannot reach (`badDamageDisjunctHead`). Deleted, and
|||    `DamageableTy` is now the single creature row. Reading a
|||    phrase's silence as permission is the recurring bug; the last
|||    chapter found the same one in [CR#109.2]'s zone default.
||| 51. **Alternatives are parallel, and three words are not
|||    alternatives at all.** Parallel in RANK — a head noun and a bare
|||    status word cannot stand in each other's place, which the guide
|||    says outright ("Repeat the carrier when the alternatives have
|||    different domains or modifiers") — and parallel in PLACE, since
|||    the phrase places its referent once. Parallel in what they
|||    COMMIT, too, which the place rule only looked like: comparing
|||    the alternatives through [CR#109.2]'s default made a committed
|||    zone and a silent one agree, and the disagreement then surfaced
|||    downstream as a projection of nothing, where a conjunction could
|||    place the phrase somewhere else entirely
|||    (`badPartialZoneJoin`). Comparing the SEEDS, silence included,
|||    is what makes the joins honest: a projection of nothing now
|||    means every alternative was silent. The three words that cannot
|||    be alternatives share one reason: each is written once for the
|||    whole coordination. "Any target" IS the union [CR#115.4] fixes
|||    by disjunction ("creatures, players, planeswalkers, or
|||    battles"), so coordinating it re-opens a closed class; "other"
|||    fills its one selector slot for the coordination entire ("Another
|||    target Wolf or Werewolf you control"); and an `Or` inside an `Or`
|||    is the flat coordination written with brackets oracle cannot
|||    print — core reaches the same shape by flattening associatively
|||    in `normalize`, where the workbench refuses the second spelling.
|||    Each alternative is read through `flattenPs`, so a singleton
|||    conjunction launders none of the three
|||    (`badAnyTargetInOrLaundered`) — the lesson finding 42's negation
|||    row learned, applied to a new list-carrying constructor before
|||    it could be exploited.
||| 52. **Negation does not reach a coordination.** The corpus spells
|||    "non-(A or B)" zero times, while the De Morgan the writer does
|||    instead is plentiful and comma-chained: "noncreature, nonland
|||    card", "Target nonattacking, nonblocking creature". `Not (Or …)`
|||    is unwritable on that evidence (`badNegatedDisjunction`), and
|||    the second half of `rawNonattacking`'s card now spells, chained
|||    exactly as the writer chains it.
||| 53. **A disjunction is a binding HOLE.** Exactly one alternative is
|||    realized and the phrase never says which, so a possessor written
|||    inside one names nobody the next sentence can read
|||    (`badDisjunctAntecedent`) — negation's rule (finding 39's
|||    `predDelta`) for a neighbouring reason. What is unaffected is
|||    the phrase's OWN binding: the determiner mints that at the noun
|||    layer, which is why "Destroy target artifact or enchantment"
|||    still leaves an "it" behind.
|||
||| Chapter fifteen, the one-shot deontic (evidence: Infiltrate, Change
||| of Heart, Blindblast, Blinding Flare; the duration corpus split is
||| the chapter's sharpest single fact):
||| 54. **A "can't" with a duration is a CLAUSE; a "can't" without one
|||    is an ability.** "Target creature can't block this turn."
|||    resolves and creates a continuous effect for the span it states
|||    ([CR#611.2a]) — core's `Continuously { effect:
|||    Deontic(Cant(…)), duration }` (`deckmaste_core/src/effect.rs`,
|||    `deontic.rs`) with exactly the two halves English writes. What
|||    the effect creates is a restriction the declare steps check
|||    ([CR#508.1c] for attacking, [CR#509.1b] for blocking and for
|||    being blocked), and it beats any permission it meets
|||    ([CR#101.2]). The static line — "Enchanted creature can't
|||    attack", Pacifism — is durationless, continuous, and the parked
|||    ability layer's; here it is unwritable because the span slot is
|||    not optional, so the refusal is the ABSENT SLOT rather than a
|||    gate (`badStaticCant`). That is the whole scope line, and it is
|||    the smallest bite that spells a real card end to end.
||| 55. **The duration adverbial is the CONSTRUCTION's word, not the
|||    writer's.** "This turn" and "until end of turn" name the same
|||    span, and the corpus does not let a clause choose between them:
|||    two hundred ninety-six one-shot restriction lines say "this
|||    turn" and not one says "until end of turn", while two hundred
|||    sixty-six "gets +1/+1" lines and one hundred ninety-seven "gains
|||    haste" lines say "until end of turn" and not one says "this
|||    turn". The guide prints both wordings a section apart and
|||    assigns neither, so the corpus is what assigns them, and the
|||    workbench carries the assignment as two complementary tables
|||    (`restrictionSpan`, `grantSpan`). The cross-turn span is the one
|||    word both constructions write — detain's "can't attack or block
|||    until your next turn" against Bond of Revival's "It gains haste
|||    until your next turn" — which is what keeps the tables from
|||    being a single flipped bit. Minting the row was also this
|||    chapter's one chance to open a hole, a new duration being
|||    available to every construction that takes one, so the grant
|||    side was gated in the same edit rather than after the fact
|||    (`badGainsThisTurn`, `badGetsThisTurn`) — wave five's coupling
|||    rule, applied before the hole existed.
||| 56. **The role rides the word, because the clause has one subject.**
|||    Core tells "can't block" from "can't be blocked" by which SLOT
|||    carries the reference (`DeonticAction::Block { by, on }`), and
|||    the predecessor grammar does the same (`Enact Block <agent>
|||    <patient>`, `Semantics.idr`). A clause has ONE subject and no
|||    second slot to put the distinction in, so English marks it in
|||    the VOICE, and the deed word carries it here: `Attack`, `Block`,
|||    `BeBlocked` are literally the verb phrase after "can't". The
|||    vocabulary is CLOSED and every table over it is written out, so
|||    a new deed is a totality error that has to declare which types
|||    carry its grant before it can be written — the `combatant`
|||    precedent, one chapter on.
||| 57. **The deed's grant table is not the fight table.** [CR#506.3]
|||    — "Only a creature can attack or block" — answers all three
|||    rows, the passive included, because what a blocker blocks is an
|||    attacking creature ([CR#509.1a]). That is the same verdict
|||    `combatant` gives and a DIFFERENT question: the fight chapter
|||    minted that table precisely because fight keys on type
|||    membership and deals non-combat damage ([CR#701.14b,701.14d]),
|||    where these deeds are combat proper and read the grants
|||    Creature.ron actually confers. So a second table, `deedType`,
|||    written out in both directions like `sameKind`. It has no
|||    untyped row: a coordinated subject fixes no type (finding 50)
|||    and therefore cannot prove participation
|||    (`badCantDisjunctSubject`) — silence read as permission, refused
|||    a third time.
||| 58. **Everything else the restriction needed was already there.**
|||    The class word places nothing, so the battlefield demand refuses
|||    it with no rule of its own (`badCantAnyTarget`), exactly as it
|||    refuses destroy and tap; a graveyard card is refused by the same
|||    gate ([CR#506.4] removes a permanent that leaves the
|||    battlefield from combat; `badCantInGraveyard`). Grammatical
|||    number is NOT demanded, and the corpus is why: "Any number of
|||    target creatures can't block this turn." (Blinding Flare) and
|||    "Other creatures can't attack this turn." (Intimidation Bolt)
|||    restrict groups, so a plurality gate would have been invention.
|||    The subject enters the discourse as any clause's
|||    does, an anaphoric subject reads back into it (Blindblast's
|||    "That creature"), and the clause nests under `May`, `Delayed`,
|||    `Sequentially`, and a colon without a gate misfiring — audited,
|||    with nothing to fix. A chapter that adds one constructor and
|||    four gates and finds the rest already standing is the shape the
|||    projections were built for.
|||
||| Engine-boundary deferrals (deliberate, and to stay so): the
||| workbench spells the ENGLISH; committed event structure is
||| core's. The per-combatant fight fact is the type case — core
||| commits `Fight` once per subject, a self-fight being ONE subject
||| ([CR#701.14a,701.14c]; `deckmaste_core/src/event.rs`,
||| `plugins/builtin/macros/filter/Fight.ron`) — and the clause here
||| stays event-silent until triggers and the "this way" reads
||| consume it. The same boundary holds the damage pipeline
||| (prevention and replacement), trigger firing and state-based
||| actions, and every runtime magnitude (§3: sorts stored, values
||| never).
|||
||| Not settled yet: the kind union ("any target" spans objects and
||| players [CR#115.4,115.1] — elided to `Object`). The elision's LEAK
||| is closed, and by fixing a projection rather than patching a gate:
||| the class word describes no object, so [CR#109.2]'s battlefield
||| default has nothing to place and the phrase projects no zone,
||| which is how every battlefield-demanding verb comes to refuse it
||| through the demand it already carried (`badDestroyAnyTarget`,
||| `badTapAnyTarget`); the binding the mention introduces records the
||| same silence, so reading it back inherits nothing better
||| (`badDestroyAnyTargetRemention`). Damage keeps the phrase by a
||| recipient row of its own — `DamageRecipient` asks the NOUN now, as
||| `DiscardOk` does — which is what a zone-free object row could not
||| have done without admitting bare `This`, the source as an object
||| (`badDamageThis`). What is left is the union proper, and it is a
||| KIND question rather than a zone one: `AnyTarget` is typed
||| `Predicate bs Object`, so the mention binds an OBJECT, the reads
||| that reach it are the object pronouns ("it", never "that
||| player"), and its recipient row asserts damageability without
||| saying which kind was chosen. Core settles none of this by
||| example: it has no kind index at all — one `Reference` spanning
||| both, with `DealDamage(Reference, Count, Reference)` taking the
||| same type in the source and patient slots
||| (`deckmaste_core/src/action.rs`, `reference.rs`: "Players are
||| objects") — whereas the kind index HERE is what buys the anaphora
||| discipline (uniqueness counted per kind). So the union wants `Or`
||| at the predicate level and a kind join the reads can project
||| through, not the deletion of kinds — which is what the `AnyTarget`
||| constructor has claimed since it was minted. Half of that has
||| arrived: chapter fourteen built `Or`, and settled that the class
||| word is expressly NOT one of its alternatives, [CR#115.4]'s union
||| being closed already (`badAnyTargetInOr`). What is still missing
||| is the JOIN: `Or`'s alternatives share one `Kind` index by
||| construction, so "target player or planeswalker" (Chandra Nalaar)
||| cannot be written as a disjunction of a player description and an
||| object one, and the reads have no joined kind to project through
||| — the union proper, now stated as the one thing it needs;
||| owned-zone PREDICATE mentions beyond `You` ("a card in an
||| opponent's graveyard" — [CR#400.3] makes the possessive an owner
||| FILTER, finding 34; the inner noun does not fold yet); controller
||| fold-state (the controller half of verb restrictions, entangled
||| with [CR#109.4]; sharpened by audit — `ControllerOf` accepts a
||| graveyard-introduced object, but [CR#109.4] gives off-battlefield
||| objects no controller, and last-known information preserves only
||| what existed ([CR#608.2h] is zone-general, not battlefield-only —
||| the audit's correction), so the read wants zone/provenance
||| evidence); the
||| library zone and its ORDERED positions ("into its owner's library
||| second from the top", "on the bottom of its owner's library" — a
||| sequence structure no current zone carries, arriving with the
||| draw cluster); miracle's reveal condition ("the first card you've
||| drawn this turn" [CR#702.94a] — a turn-scoped draw-ordinal
||| memory, same cluster); the up-to-one singular
||| remention positive (Ty Lee, Chi Blocker — its second sentence is
||| the UNTAP deed under a "for as long as" duration ([CR#611.2b]), so
||| it waits on both, chapter fifteen's restriction clause having
||| landed with neither); `May`'s if-you-do / if-not branches (the Risk
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
||| the colon, constructor unminted); the cost GRAMMAR (the colon
||| accepts ANY clause as a cost, but [CR#602.1a] makes a cost what
||| the activator pays — delayed clauses and "you lose N life" are
||| not payments [CR#118.1,119.4], and a cost's OUTCOME mention leaks
||| through `publicOnly` to the effect; restrict the pre-colon sort
||| with the cost-participle work); hidden-zone identity (a move into
||| hand keeps its binding readable, but [CR#400.7] mints a new
||| object and [CR#400.7j] lets the effect re-find it only in a
||| PUBLIC zone — introduction-in-hand via predicate stays legal,
||| retention across a hidden-bound move must not; wants a
||| trackedness distinction the payload does not yet carry); more event queries (upkeep / end-of-combat /
||| leaves-the-battlefield — Slaughter Pact, Mirror Match, and
||| Kjeldoran Elite Guard wait on pay, tokens, and an unknown-zone
||| retag); the player pronoun "they"
||| (corpus-attested only inside trigger and unless clauses — Havoc,
||| Tergrid's Lantern — so `They`'s positive waits on those
||| constructions), player groups ("each opponent … they"
||| distributives), and "the rest"; event-outcome residues (chapter
||| ten built the scalar "that much" read; "that many" count reads
||| wait on their consumers — draw, tokens, counters are the
||| corpus's big three — "this way" participant-subset participles
||| are the `TheVerbed` cousin and the largest family, and more
||| sorts wait with them: mana produced, Sakiko; card counts,
||| Asmodeus; and condition-supplied scalars — Tellah, Great Sage's
||| mana SPENT is not a clause outcome at all, so it arrives with the
||| conditions axis); "any number of" / "X" target groups
||| (corpus-frequent; the QUANTITY spells the unbounded form now, while
||| the variable one waits on bounds that admit a variable — core's
||| range is over a `Count`, whose `X` row this one's `Nat` has no
||| answer to — and both wait on verified whole cards for a positive),
||| and
||| with them the PLURAL damage-class forms — [CR#115.4] names
||| "another target," "two targets," and similar expressly alongside
||| "any target", and the corpus writes them either as a division
||| ("deals 2 damage divided as you choose among one or two targets",
||| Chandra's Pyrohelix) or as an each-of recipient ("Fall of the
||| Titans deals X damage to each of up to two targets"); the bare
||| up-to noun already spells (`TargetGroup (upTo n) AnyTarget`), as
||| does the unbounded quantity itself (`anyNumber`), while the
||| division and each-of RECIPIENT structures and a verified whole-card
||| positive wait with this axis, and the exact-count mention admits
||| the class word only at ONE — the singular form itself — refusing it
||| from two up, and from the unbounded quantity, until they land
||| (`AnyTargetAtCount`);
||| counted UNTARGETED groups ("four lands you control" — Burning of
||| Xinye's subject-destroy positive waits on them); the
||| reciprocal fight frame ("those creatures fight each other" — one
||| exactly-two-membered plural subject, corpus-attested, a distinct
||| construction from binary `Fights`); union-read plural possessors
||| ("creatures your opponents control", corpus-attested — waits with
||| the player groups); the self-exclusion "other" anchor (Olivia
||| Voldaren, Red Hulk — "another"/"any other" excluding the SOURCE;
||| the sorted self-reference is typed and battlefield-projected
||| already (finding 41), so what this waits on is the source ENTERING
||| the discourse: `This`/`ThisOf` introduce no binding, and the anchor
||| search reads bindings);
||| static "as long as" conditions ([CR#611.3], Kitesail Corsair —
||| the card has no "for") and effect-created "for as long as"
||| durations ([CR#611.2b]); last-known VALUES (reads ignore zone — finding
||| 19 — but [CR#109.4] gives off-battlefield objects no controller,
||| so the value story belongs to the ability layer); Token / Spell
||| / stack-object / Amount carriers ("that much"; bare `This` stays
||| untracked, and "this spell" / "this card" carriers with it); "the
||| chosen [quality]" (quality-kind bindings); coordinated verb
||| COMPLEMENTS ("deals 2 damage to any target and 1 damage to any
||| other target", Arc Trail — one verb distributing over paired
||| amount+recipient complements; the bench transcribes them as a
||| two-clause `Sequentially`, a named stand-in that mis-orders nothing
||| binding-wise but serializes what the card states as one
||| instruction); amount EXTRAPOSITION (`DealDamage` fixes
||| amount-before-recipient order while oracle writes both — "deals
||| damage equal to its power to target creature" against "deals damage
||| to any target equal to the mana value of the discarded card",
||| Pyromancy; amounts introduce no bindings, so the fixed order is
||| binding-neutral and the divergence is linearization's, like the
||| coordination above); event-history restrictive clauses ("that were
||| put there from the battlefield this turn", Continue? — no
||| vocabulary, so `continueSpell` elides the clause, which widens the
||| domain it expresses until the axis arrives); and CONTROL assignment
||| on battlefield moves ([CR#110.2a] defaults an instructed put to the
||| instructed player, so Cloudshift's "under your control" is the
||| derivable default, while "under its owner's control" is an OVERRIDE
||| the vocabulary cannot spell — four positives elide it, each named
||| meaning-carrying in its comment: Turn to Mist, Flickering Spirit,
||| Graceful Reprieve, Voyager Staff); and the rest of the DEONTIC
||| surface, chapter fifteen having taken the one-shot restriction
||| clause and nothing else. The STATIC form is the largest piece:
||| "Enchanted creature can't attack" (Pacifism) is durationless and
||| continuous, so it belongs to the ability layer with the other
||| static abilities rather than to any clause. Beside it: deed
||| COORDINATION, one "can't" over two deeds ("Target creature can't
||| attack or block this turn.", six lines, and the detain family's
||| cross-turn form — `Or` coordinates PREDICATES, and a deed is not
||| one); PLAYER subjects ("Target player can't cast spells this
||| turn"), which want the cast deed and a player-subject clause, the
||| kind index making them structurally unwritable today; the deeds
||| with no clause yet — "can't be regenerated" (one hundred forty-two
||| lines, and usually with NO stated duration, so it wants
||| [CR#611.2a]'s end-of-game default in a span slot that currently
||| refuses to be empty), "can't be countered" (thirty-three), and
||| "doesn't untap" (one hundred twenty-two, Ty Lee above) — each
||| needing its own `deedType` row and the last two a subject this
||| vocabulary cannot describe; the EXCEPT rider ("can't be blocked
||| except by Walls", one hundred ninety-one lines), a restriction
||| carrying an exception clause rather than a plain one; the other
||| deontic polarities (core keeps `May`, `Must`, and `Gate` beside
||| `Cant` — requirements are the "attacks if able" family
||| [CR#508.1d,509.1c], arbitrated rather than subtracted); and the
||| bare GENERIC PLURAL subject ("Creatures without flying can't block
||| this turn.", Falter), which is not the "all" determiner `AllOf`
||| spells and has no noun of its own.
||| The context-as-phrase-telescope collapse (bindings storing
||| the mention terms themselves, every projection computed) stays open as
||| a possible later simplification — less pressing since the payload
||| split gave each kind exactly its own data.
|||
||| The representability frontier (the second-wave audit's merged map;
||| every gap evidenced by real oracle text plus the core taxonomy,
||| none spellable today). New structural axes: attachment — ONE
||| relation ([CR#701.3a]; core's `AttachedTo`), with
||| "enchanted"/"equipped" as derived reads of an Aura/Equipment edge
||| ([CR#303.4b,301.5a]); the two keywords are NOT parallel — Enchant
||| statically restricts an Aura's legal target and host ([CR#702.5a]
||| — a deontic grant over the attach basis, not edge structure),
||| while Equip is the activated ability that does the attaching
||| ([CR#702.6a]);
||| object status — tapped/flipped/face-down/phased as a per-object
||| state dimension ([CR#110.5]; no Untap verb, no Tapped predicate:
||| "destroy target tapped creature" is unspellable); pile partitions
||| (Death or Glory partitions the GRAVEYARD, so this is not the
||| library gap — [CR#700.3a] puts each object in exactly one pile
||| "unless the effect specifies otherwise" and [CR#700.3d] lets a
||| pile be empty, while [CR#700.3c] only keeps the piles in the zone
||| they came from; labels, choice, and complement are the card's own
||| instructions); modal clauses ("Choose one —",
||| [CR#700.2,115.8] — effect-row alternatives with branch-local
||| targets); truth-valued conditions and branching (Galvanic Blast's
||| "instead if"; [CR#603.4] separates intervening-if from English
||| "if"); linked-ability memory (Cold Storage, [CR#607.2a] —
||| source-keyed exile notes across abilities, beyond any one
||| discourse); divided amounts (Chandra's Pyrohelix,
||| [CR#601.2d,115.7f] — an allocation announced and locked);
||| dependent iteration (Killing Wave's "for each creature, its
||| controller…" — a per-member singular frame no plural noun
||| provides); aggregation and extremal selection (Crackling Doom's
||| "greatest power among"); data-dependent repetition (Torment of
||| Hailfire's "repeat this process X times"); turn-schedule
||| insertion (Relentless Assault, [CR#500.8]); standing triggered
||| abilities ([CR#603.1] — the ability SHAPE beyond delayed
||| queries); counters as per-HOLDER state — objects AND players
||| ([CR#122.1]; poison and rad ride the player), with put/remove
||| verbs and count reads; mana production and payment
||| ([CR#106.4]); and control ASSIGNMENT on a battlefield move — a
||| controller argument the move primitive does not carry, since
||| [CR#110.2a] gives the instructed player by default and only an
||| explicit "under [player]'s control" overrides it. Leaf
||| vocabulary: Transform ([CR#701.27a]);
||| life-total Set ([CR#119.5]); the non-additive continuous family
||| (gain control, "becomes", lose abilities, set base P/T — the
||| layer words).
|||
||| The English-AST frontier — §8's other half, the parser's own
||| construction inventory read against this file rather than the gaps
||| the chapters walked into. Deferred deliberately, each waiting on
||| an axis already named above: the general prepositional, adverbial,
||| and subordinating OPERATORS — particular relations are ledgered
||| one at a time (the "if" and "as long as" conditions, the "when"
||| events), but the operator carrying any of them is not, and the
||| counterfactual "as though" has no ledger entry at all ("This
||| creature can attack this turn as though it didn't have
||| defender"); the bounded and
||| alternative quantity WORDINGS — `Quantity` is already a range with
||| both bounds optional, so "at least three", "three or more", and
||| "more than two" are ONE bound pair under three words (a count
||| against a literal, not the comparative family's value
||| comparisons), which puts the gap on the spelling rather than the
||| structure: the semantics is right to collapse them and the
||| renderer has to undo the collapse; the two that are not bounds are
||| "one or two" (a disjunction of exact counts, waiting with the
||| divided damage above) and "both" ("both creatures have
||| deathtouch" — a definite plural presupposing a two-membered
||| antecedent); activation restrictions and frequency riders
||| ("Activate only as a sorcery", "Activate only during your turn and
||| only once each turn") — a legality rider with a turn-scoped count,
||| the ability layer's rather than any clause's; nonfinite and
||| elliptical clauses — infinitive, gerund, and ellipsis have nothing
||| to lower to while every effect here is a saturated clause; the
||| passive and its reduced-recipient forms ("Whenever this creature
||| is dealt damage, it deals that much damage to the chosen player",
||| Stuffy Doll) — recipient fronted and agent dropped, arriving with
||| the event queries above rather than as a clause shape of its own;
||| exception riders ("except it's legendary", "except it's not
||| legendary") — one characteristic of what the clause just made,
||| overridden after the fact, so they want the copy/token vocabulary
||| and the layer words together; partitives ("Put one of them into
||| your hand and the rest on the bottom of your library in any
||| order") — selection from an established group, twin of "the rest"
||| above and of the pile partitions below; set-exception noun phrases
||| ("choose a card type other than creature", Arachne, Psionic
||| Weaver) — a complement
||| over a QUALITY domain, which `Other`'s object distinctness is not,
||| waiting with the chosen-quality bindings; arithmetic and rounding
||| ("Target opponent loses half their life, rounded up") — `Plus` is
||| the only composition, and subtraction, halving, and a rounding
||| mode have no constructor; quoted abilities (Master of the Hunt's
||| token has "bands with other creatures named Wolves of the
||| Hunt.") — an ability as a VALUE and a verb that installs it, the
||| layer words' carrier question in a new place; the causative ("You
||| may have it become a 3/3 Elemental creature with haste and menace
||| until end of turn") — `have` over a bare-infinitive complement,
||| and only its narrowest "may have [player] deal" reading is `May`
||| today; coin flips and their result clauses ("You
||| and target opponent each flip a coin", Mana Clash) — a random
||| outcome to bind and a side predicate to read it back; the
||| rules-defined bundles, which SPLIT — historic ([CR#700.6]),
||| modified, party, and outlaw name sets the CR fixes, landing with
||| the predicate vocabulary, while devotion is a measured value
||| taking a mandatory "to [color]" ([CR#700.5]; "each opponent
||| loses X life, where X is your devotion to black", Gray Merchant
||| of Asphodel), landing with the amounts;
||| the COORDINATION families chapter fourteen did not take, each
||| waiting on a constituent this file has no term for: CLAUSE
||| coordination in its six spellings (`ClauseCoordination`, its comma
||| and asyndetic variants, and the three copular-noun-prepositional
||| ones) waits on no new structure — `Sequentially` composes clauses
||| already — but on the DISTINCTION the guide draws between
||| coordinated and sequenced results ("and" for results with no
||| emphasized ordering, "then" when the order matters), which the
||| clause list currently collapses, the Arc Trail entry above being
||| the same collapse seen from one card; QUOTED-ability coordination
||| (`VerbPhraseQuotedAbilityCoordination`,
||| `VerbPhraseAbilityQuotedCoordination`) waits on the quoted ability
||| as a VALUE, above; the coordinated-MODIFIER trio
||| (`CoordinatedModifierConjoined`, `CoordinatedModifierOxford`,
||| `NominalCoordinatedModifier`), the coordinated-ADJECTIVE trio
||| (`VerbPhraseCoordinatedAdjective`,
||| `CopularRemainderCoordinatedAdjective`,
||| `RelativeContractedCopularCoordinatedAdjective`), and the
||| postpositive-adjective spellings
||| (`nominal_postpositive_adjective_conjoined` with its Oxford and
||| prepositional twins) all wait on the same missing constituent, an
||| ADJECTIVE phrase: the status words here are predicates, and
||| nothing in this file is adjective-shaped; and
||| `PrepositionalPhraseSiblingCoordinated` waits on the general
||| prepositional operator named at the top of this list. Two
||| disjunctions are likewise named rather than built — the
||| CROSS-ZONE one, which the single-valued zone projection refuses
||| outright rather than mis-place ("an Equipment card from your hand
||| or graveyard", seventeen corpus lines under a shared preposition;
||| `badCrossZoneDisjunction`), and the KIND-crossing one, which is
||| the kind-union entry above wearing a coordinator. And `and/or` is
||| a THIRD coordinator, not a spelling of this one: the parser keeps
||| `Conjunction::AndOr` beside `And` and `Or` in its coordination
||| constructions, and the guide gives it its own rule — either
||| category alone or both together qualify, "instant and/or sorcery
||| cards"; and the striated ability
||| frames — class levels (Monk Class's "{W}{U}: Level 2"), saga
||| chapters (History of Benalia's "I, II —"), level bands (Kargan
||| Dragonlord's "LEVEL 4-7"), station thresholds (The Eternity
||| Elevator's "20+ |"), and die-roll rows — one shape under five
||| names: a keyed band of the CARD granting an ability set, sometimes
||| with a P/T box, once a counter, level, or roll reaches its key, so
||| the counters are the frontier's per-holder state while the frame
||| itself sits above every clause.
|||
||| One gap is the AUTHORING side rather than the semantic one: a
||| fragment is authorable at Nominal, Sentence, Cost, KeywordLine, or
||| Ability and at no other category, so an `Amount`, a `Duration`, or
||| an `EventQuery` has no category to be authored AT — which is why
||| the spelling pass marked those constructors construction-owned or
||| TODO instead of giving them a fragment of their own.
|||
||| Two families stay unmapped deliberately, being the next CHAPTERS
||| rather than deferrals: comparatives, and the generic definite and
||| possessive noun phrases (which "both" above waits on). The deontic
||| auxiliaries were a third and have SPLIT the way coordination did:
||| chapter fifteen took the one-shot restriction clause, and what is
||| left is the STATIC reading ("Enchanted creature can't attack or
||| block", Pacifism) with the ability layer, plus the rest of the
||| auxiliary slot itself — the parser's slot is general, twelve of
||| them stacking, and the deontic reading was only ever one slice of
||| it. Coordination was the fourth and has SPLIT: chapter fourteen
||| took its noun-phrase half — `noun_phrase_coordination` and
||| `shared_determiner_nominal`, which is Disenchant's alternatives
||| and Rats of Rath's shared modifier — and the rest is deferred by
||| axis rather than unmapped, in the frontier entry above.
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
-- macro's own word (see cardtype/Creature.ron), consumed by HasType/ThisOf,
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

||| The word check against a STAMPED object mention, under the verb's
||| frame: a TYPE word demands the referent stood on the battlefield
||| AT the verb (a bare type word denotes a permanent [CR#109.2] —
||| "the discarded creature" is unwritten; hands lose cards, not
||| creatures) plus its projected type; the CARD word checks the
||| current zone; no participle reads a player.
public export
stampWordOk : VerbName -> NounWord -> Stamp -> Maybe CardType -> Maybe Zone -> Bool
stampWordOk v (TypeW t) (MkStamp v' wasF) ty zn = sameVerb v v' && wasF && tyIs t ty
stampWordOk v CardW (MkStamp v' _) ty zn = sameVerb v v' && isCardZone zn
stampWordOk v PlayerW st ty zn = False

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
||| ([CR#109.2], `nounZone (ThisOf …)`); bare `This` is the source as an
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

||| Durations, as the trailing adverbial writes them ([CR#611.2a] — a
||| resolution-generated continuous effect "lasts as long as stated";
||| with no stated duration it lasts until end of game, which is the
||| explicit `Nothing` spelling per the no-defaults convention).
||| "for as long as" durations ([CR#611.2b]) are a later chapter.
||| Two of these name the SAME span in different words, and which word
||| a clause writes is its construction's, not its author's — see
||| `grantSpan` and `restrictionSpan`.
public export
-- spelling: ["until end of turn", "until your next turn", "this turn"] (row
-- order: UntilEndOfTurn/UntilYourNextTurn/ThisTurn), kind: TODO(reason:
-- trailing-adverbial fragment -- not one of Nominal/Sentence/Cost/
-- KeywordLine/Ability)
data Duration = UntilEndOfTurn | UntilYourNextTurn | ThisTurn

||| Which duration a GRANT or a stat change writes. The two current-turn
||| adverbials are not interchangeable: "until end of turn" runs to two
||| hundred sixty-six corpus lines on "gets +1/+1" and one hundred
||| ninety-seven on "gains haste", and "this turn" to none of either,
||| while the one-shot restrictions invert the split exactly
||| (`restrictionSpan`). The guide lists both wordings and
||| assigns neither — "Use `until end of turn` for the ordinary
||| current-turn duration" sits a section away from "Use `this turn` for
||| the current turn" — so the corpus is what assigns them. Full rows: a
||| new duration declares its answer on both tables.
public export
grantSpan : Duration -> Bool
grantSpan UntilEndOfTurn = True
grantSpan UntilYourNextTurn = True
grantSpan ThisTurn = False

||| The grant's duration slot as a witness. `Nothing` is the stated
||| absence — no duration written, so the effect lasts until end of game
||| ([CR#611.2a]), which the guide permits where the effect is
||| "intentionally indefinite under the rules" (Through the Breach's
||| bare "It gains haste.").
public export
data GrantSpan : Maybe Duration -> Type where
  Indefinite : GrantSpan Nothing
  Stated : {auto 0 ok : grantSpan d = True} -> GrantSpan (Just d)

-- ===== Deontic restrictions (the one-shot "can't" vocabulary) =====

||| The deed a restriction denies, with the ROLE its subject plays in
||| it. Core and the predecessor grammar both mark that role by which
||| SLOT carries the reference — `DeonticAction::Block { by, on }`
||| (`deckmaste_core/src/deontic.rs`), `Enact Block <agent> <patient>`
||| (`Semantics.idr`) — but a clause with ONE subject has no second slot
||| to put it in, so English marks it in the VOICE and the deed word
||| carries it here: "can't block" against "can't be blocked". Closed
||| and row-enumerated, and every table over it is written out, so a new
||| deed is a totality error that must declare which types carry its
||| grant before it can be written at all. These three lead the one-shot
||| restrictions in the corpus — "can't be blocked this turn" one hundred
||| seventy-three lines, "can't block this turn" one hundred fifteen,
||| "can't attack this turn" eight — and the rest of the deontic surface
||| is either the parked ability layer or a deed with no clause of its
||| own yet.
public export
-- spelling: ["attack", "block", "be blocked"] (row order: Attack/Block/
-- BeBlocked -- the verb phrase after "can't"; two active infinitives and
-- one passive, consumed by Effect.Cant, never spelled alone)
data Deed = Attack | Block | BeBlocked

||| Which card types carry a deed's grant — the stand-in for reading
||| `May(Attack)`/`May(Block)` off the TypeDef declaration
||| (`plugins/builtin/macros/cardtype/Creature.ron`), and a DIFFERENT
||| table from `combatant`, which the fight chapter minted precisely
||| because fight keys on type membership and deals non-combat damage
||| ([CR#701.14b,701.14d]) where these deeds are combat proper.
||| [CR#506.3] — "Only a creature can attack or block" — answers all
||| three rows, the passive included: what a blocker blocks is an
||| attacking creature ([CR#509.1a]). Written out in BOTH directions,
||| `sameKind`-style, so a new deed and a new card type are each a
||| totality error rather than a silent `False`.
public export
deedType : Deed -> CardType -> Bool
deedType Attack Creature = True
deedType Attack Artifact = False
deedType Attack Land = False
deedType Attack Enchantment = False
deedType Block Creature = True
deedType Block Artifact = False
deedType Block Land = False
deedType Block Enchantment = False
deedType BeBlocked Creature = True
deedType BeBlocked Artifact = False
deedType BeBlocked Land = False
deedType BeBlocked Enchantment = False

||| The deed's demand on its subject's projected head, as a witness —
||| `FightParticipant`'s shape for the combat grants. There is no
||| `Nothing` row: an UNTYPED head cannot prove participation, so a
||| disjunctive subject, which honestly fixes no type (finding 50), is
||| refused rather than waved through on its silence.
public export
data DeedParticipant : Deed -> Maybe CardType -> Type where
  Participant : {auto 0 ok : deedType d t = True} -> DeedParticipant d (Just t)

||| Which duration a RESTRICTION writes — `grantSpan`'s complement, and
||| complementary is what the corpus makes it. Every one-shot
||| single-deed restriction says "this turn" (two hundred ninety-six
||| lines across the three deeds) and not one says "until end of turn".
||| The cross-turn span is written too, in the detain family ("Up to one
||| target creature can't attack or block until your next turn"), where
||| what this vocabulary is missing is the deed coordination (ledger),
||| not the adverbial. Full rows, as on the grant side.
public export
restrictionSpan : Duration -> Bool
restrictionSpan ThisTurn = True
restrictionSpan UntilYourNextTurn = True
restrictionSpan UntilEndOfTurn = False

||| The restriction's duration slot as a witness. There is no absent
||| row: a durationless "can't" is the STATIC ability line, a different
||| construction (see `Cant`).
public export
data RestrictionSpan : Duration -> Type where
  Spanned : {auto 0 ok : restrictionSpan d = True} -> RestrictionSpan d

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
    -- spelling: ["the battlefield"], kind: Nominal
    BattlefieldZ : ZoneExpr bs
    -- spelling: ["exile"], kind: Nominal
    ExileZ : ZoneExpr bs
    -- spelling: ["hand"], kind: Nominal (bare sort-only form -- macro
    -- expansions whose English wrote no owner; see the constructor comment)
    HandZ : ZoneExpr bs
    -- spelling: ["graveyard"], kind: Nominal (bare sort-only form, see HandZ)
    GraveyardZ : ZoneExpr bs
    -- owned zones are per-player ([CR#400.1]): the possessor is
    -- singular — the plural surface is the plural relational
    -- ("their owners' hands", ledger).
    -- spelling: ["<Param(0)>'s hand"], kind: Nominal (e.g. "your hand",
    -- "its owner's hand" -- the owned-zone possessive form)
    HandOf : (n : Noun bs Player) -> {auto 0 one : nounPlur n = OneOf} -> ZoneExpr bs
    -- spelling: ["<Param(0)>'s graveyard"], kind: Nominal (see HandOf)
    GraveyardOf : (n : Noun bs Player) -> {auto 0 one : nounPlur n = OneOf} -> ZoneExpr bs

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
    -- and fills its one slot at most once (`OtherAnchored`), and the
    -- class word "any target" takes no modifiers but "other" and is
    -- itself written exactly once (`AnyTargetLone`).
    -- spelling: (construction-owned -- flat modifier-list juxtaposition, not
    -- itself a word; kind follows whether a member hasHead)
    And : (ps : List (Predicate bs k)) -> {auto 0 zc : ZoneCoherent ps} ->
          {auto 0 cf : ContradictionFree ps} -> {auto 0 oa : OtherAnchored ps} ->
          {auto 0 at : AnyTargetLone ps} -> Predicate bs k
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
    -- the sorted self-reference "this artifact"/"this land"/"this
    -- creature": the source under its type noun. Unmoved it introduces
    -- nothing (like `This`); MOVED it mints a fresh binding — the move
    -- makes it a new object [CR#400.7], which is why cost-position
    -- "Sacrifice this artifact" leaves a referent the effect's "It" can
    -- read ([CR#400.7j] is the exception letting the effect find it).
    -- spelling: ["this <Param(0)>"], kind: Nominal (Param(0) = CardType's
    -- word; the sorted self-reference, [CR#109.2])
    ThisOf : CardType -> Noun bs Object
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
    -- spelling: ["a <Param(0)>"], kind: Nominal (auto-inflects to "an"
    -- before a vowel sound)
    A : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
        {auto 0 hd : Headed p} ->
        {auto 0 af : AnyTargetFree p} -> Noun bs k       -- "a …": indefinite choice/product [CR#608.2d,400.7]
    -- "a … of their choice" / "a … at random": the indefinite with its
    -- choice method marked in the text — chooser and method are surface
    -- facts (the guide's chooser marking; a random discard has no
    -- chooser [CR#701.9b]), mirroring the real macros' explicit `by`
    -- slot and its absence in the at-random variant. "Their" is a
    -- possessive pronoun: it demands exactly one player antecedent —
    -- a subject or one distributive group (`badUnboundTheirChoice`).
    -- spelling: ["a <Param(0)> of their choice"], kind: Nominal
    ATheirChoice : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
                   {auto 0 ch : countChoosers bs = 1} ->
                   {auto 0 hd : Headed p} ->
                   {auto 0 af : AnyTargetFree p} -> Noun bs k
    -- spelling: ["a <Param(0)> at random"], kind: Nominal
    AAtRandom : (p : Predicate bs k) -> {auto ph : Phrasal k} ->
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
    -- demonstrative would be ambiguous (finding 26).
    -- spelling: ["the <Param(0)> <Param(1)>"] (Param(0) = VerbName's lemma,
    -- rendered as its past participle by auto-inflection; Param(1) =
    -- NounWord's own word), kind: Nominal
    TheVerbed : (v : VerbName) -> (w : NounWord) ->
                {auto 0 ok : countVerbed v w bs = 1} -> Noun bs Object
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
  nounEqRef (ThisOf _) _ = False
  nounEqRef You You = True
  nounEqRef You _ = False
  nounEqRef (Each _) _ = False
  nounEqRef (A _) _ = False
  nounEqRef (ATheirChoice _) _ = False
  nounEqRef (AAtRandom _) _ = False
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
  nounAnyTargetFree (ThisOf _) = True
  nounAnyTargetFree You = True
  nounAnyTargetFree (Each p) = anyTargetFree p
  nounAnyTargetFree (A p) = anyTargetFree p
  nounAnyTargetFree (ATheirChoice p) = anyTargetFree p
  nounAnyTargetFree (AAtRandom p) = anyTargetFree p
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
  zoneAnyTargetFree BattlefieldZ = True
  zoneAnyTargetFree ExileZ = True
  zoneAnyTargetFree HandZ = True
  zoneAnyTargetFree GraveyardZ = True
  zoneAnyTargetFree (HandOf n) = nounAnyTargetFree n
  zoneAnyTargetFree (GraveyardOf n) = nounAnyTargetFree n

  ||| Destination legality for the move primitive ([CR#400.3] — cards
  ||| enter only their owner's hand/library/graveyard, so an owned
  ||| destination naming an arbitrary player is unwritable, and the
  ||| bare zone IS the owner-rooted destination; the possessive is
  ||| rendering's business, finding 34).
  public export
  data DestOk : ZoneExpr bs -> Type where
    BattlefieldOk : DestOk BattlefieldZ
    ExileOk : DestOk ExileZ
    HandOkBare : DestOk HandZ
    GraveyardOkBare : DestOk GraveyardZ

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
  nounDelta (Each p {ph}) = bindFor EachD ManyOf ph p :: predDelta p
  nounDelta (A p {ph}) = bindFor AD OneOf ph p :: predDelta p
  nounDelta (ATheirChoice p {ph}) = bindFor AD OneOf ph p :: predDelta p
  nounDelta (AAtRandom p {ph}) = bindFor AD OneOf ph p :: predDelta p
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
    -- spelling: ["<Param(0)>"] (bare numeral), kind: TODO(reason: amount
    -- fragment -- not one of Nominal/Sentence/Cost/KeywordLine/Ability)
    Lit : Nat -> Amount bs
    -- "[its/…] power" / "toughness" / "mana value"
    -- ([CR#208.1,202.3]): one object's own numbers, so the argument
    -- is singular — a group's aggregate is written explicitly ("the
    -- total power of the sacrificed creatures", Soulblast) and is
    -- future vocabulary (`badGroupPower`).
    -- spelling: ["<Param(0)>'s power"], kind: TODO(reason: amount fragment,
    -- see Lit)
    PowerOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
    -- spelling: ["<Param(0)>'s toughness"], kind: TODO(reason: amount
    -- fragment, see Lit)
    ToughnessOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
    -- spelling: ["<Param(0)>'s mana value"], kind: TODO(reason: amount
    -- fragment, see Lit)
    ManaValueOf : (n : Noun bs Object) -> {auto 0 one : nounPlur n = OneOf} -> Amount bs
    -- "[per] [unit] for each [pred]" — the counted-set amount
    -- ("loses 1 life for each attacking creature you control");
    -- mentions inside the predicate fold as everywhere. The domain is
    -- a real noun phrase: every corpus for-each domain is noun-HEADED
    -- (`Headed` — "for each you control" heads nothing), and the
    -- per-unit is a written numeral, so it is at least one
    -- (`AtLeastOne`; the comparisons that legitimately carry zero read
    -- a count rather than write one, and `Lit` stays ungated).
    -- spelling: ["<Param(0)> for each <Param(1)>"], kind: TODO(reason:
    -- amount fragment, see Lit; the surrounding unit word, e.g. "life",
    -- comes from the embedding verb, not from ForEach itself)
    ForEach : {k : Kind} -> (per : Nat) -> (p : Predicate bs k) ->
              {auto 0 hd : Headed p} -> {auto 0 nz : AtLeastOne per} ->
              {auto 0 af : AnyTargetFree p} -> Amount bs
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
  amtIntro (PowerOf nom) = nomIntro nom
  amtIntro (ToughnessOf nom) = nomIntro nom
  amtIntro (ManaValueOf nom) = nomIntro nom
  amtIntro (ForEach per p) = predDelta p ++ bs
  amtIntro ThatMuch = bs
  amtIntro XVal = bs
  amtIntro (Plus a b) = amtIntro b

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
    -- "[n] gains [ability] [duration]" — establishes a continuous
    -- effect for the stated duration ([CR#611.2a]); the one-shot form
    -- modifies a battlefield object (graveyard-reaching grants are
    -- static abilities, a later chapter). The duration is the GRANT's
    -- adverbial, never the restriction's (`GrantSpan`;
    -- `badGainsThisTurn`).
    -- spelling: ["<Param(0)> gains <Param(1)>"] (optional trailing duration
    -- adverbial, see Duration), kind: Sentence
    Gain : (n : Noun bs Object) -> Ability -> (span : Maybe Duration) ->
           {auto 0 ok : OnBattlefield (nounZone n)} ->
           {auto 0 sp : GrantSpan span} -> Effect bs
    -- "[n] gets [+p/+t] [duration]" — the stat-modifying continuous
    -- effect, same duration and battlefield discipline
    -- (`badGetsGraveyard`, `badGetsThisTurn`).
    -- spelling: ["<Param(0)> gets <Param(1)>/<Param(2)>"] (signed pow/tou
    -- pair, e.g. "+1/+1"; optional trailing duration), kind: Sentence
    Gets : (n : Noun bs Object) -> (pow : Integer) -> (tou : Integer) ->
           (span : Maybe Duration) -> {auto 0 ok : OnBattlefield (nounZone n)} ->
           {auto 0 sp : GrantSpan span} -> Effect bs
    -- "[n] can't [deed] [duration]" — the one-shot DEONTIC: a clause
    -- whose resolution creates a continuous effect denying its subject
    -- a deed for the stated span ([CR#611.2a]), which is core's
    -- `Continuously { effect: Deontic(Cant(…)), duration }`
    -- (`deckmaste_core/src/effect.rs`, `deontic.rs`) with the two
    -- halves English writes. The deed is the restriction the declare
    -- steps check ([CR#508.1c] for attacking, [CR#509.1b] for blocking
    -- and for being blocked), and it beats any permission it meets
    -- ([CR#101.2]). Three demands, each already a shape this grammar
    -- makes elsewhere: the subject stands on the battlefield (combat is
    -- fought there — [CR#506.4] takes a permanent that leaves out of
    -- combat; `badCantInGraveyard`), carries the deed's grant
    -- ([CR#506.3]; `badCantAttackLand`, `badCantDisjunctSubject`), and
    -- the span is the RESTRICTION's adverbial, not the grant's
    -- (`badCantUntilEndOfTurn`). A durationless "can't" is the STATIC
    -- ability line ("Enchanted creature can't attack", Pacifism) — a
    -- different construction, and unwritable here because the slot is
    -- not optional. The subject may be plural — "Other creatures can't
    -- attack this turn." (Intimidation Bolt) is one sentence of many —
    -- so no grammatical number is demanded.
    -- spelling: ["<Param(0)> can't <Param(1)> <Param(2)>"], kind: Sentence
    -- (Param(1) = Deed's own verb phrase, Param(2) = Duration's own
    -- adverbial; mirrors core's Continuously-over-Cant pair -- no single
    -- RON constructor entry confirmed for the fused clause this pass)
    Cant : (n : Noun bs Object) -> (deed : Deed) -> (span : Duration) ->
           {auto 0 zn : OnBattlefield (nounZone n)} ->
           {auto 0 dp : DeedParticipant deed (nounTy n)} ->
           {auto 0 sp : RestrictionSpan span} -> Effect bs
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
    -- phrase; if-you-do/if-not branches are later growth.
    -- spelling: ["<Param(0)> may <Param(1)>"], kind: Sentence
    May : (decider : Noun bs Player) -> Effect (nomIntro decider) -> Effect bs
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
  nounIsAnyTarget (ThisOf _) = False
  nounIsAnyTarget You = False
  nounIsAnyTarget (Each _) = False
  nounIsAnyTarget (A _) = False
  nounIsAnyTarget (ATheirChoice _) = False
  nounIsAnyTarget (AAtRandom _) = False
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
               TagBody Destroy (Move n GraveyardZ)
    SacrificeB : {auto 0 z : OnBattlefield (nounZone n)} ->
                 TagBody Sacrifice (Move n GraveyardZ)
    ExileB : TagBody Exile (Move n ExileZ)
    DiscardB : {auto 0 d : DiscardOk n} ->
               TagBody Discard (Move n GraveyardZ)

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
  moveIntro p nn@(A pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(ATheirChoice pr) z = setZoneHead p z (nomIntro nn)
  moveIntro p nn@(AAtRandom pr) z = setZoneHead p z (nomIntro nn)
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
  -- frame stays conservatively False — a ThisOf-cost participle TYPE
  -- word waits on a corpus witness, so the pre-move zone is passed as
  -- untracked here rather than read off `nounZone`.
  moveIntro p (ThisOf t) z = MkBinding TheD Object OneOf (ObjectP (Just t) (Just z) (mkStamp p Nothing)) :: bs
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
  nounZone (ThisOf t) = Just Battlefield
  nounZone You = Nothing
  nounZone (Each p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (A p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (ATheirChoice p) = Just (zoneOr Battlefield (seedZone p))
  nounZone (AAtRandom p) = Just (zoneOr Battlefield (seedZone p))
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
  nounTy (ThisOf t) = Just t
  nounTy You = Nothing
  nounTy (Each p) = seedTy p
  nounTy (A p) = seedTy p
  nounTy (ATheirChoice p) = seedTy p
  nounTy (AAtRandom p) = seedTy p
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
  nounPlur (ThisOf t) = OneOf
  nounPlur You = OneOf
  nounPlur (Each p) = ManyOf
  nounPlur (A p) = OneOf
  nounPlur (ATheirChoice p) = OneOf
  nounPlur (AAtRandom p) = OneOf
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
  effIntro (Gain n _ _) = nomIntro n
  effIntro (Gets n _ _ _) = nomIntro n
  effIntro (Cant n _ _) = nomIntro n
  effIntro (Composite v (Move what to)) = moveIntro (Just v) what (zoneSort to)
  effIntro (Composite _ e) = effIntro e
  effIntro (Does s v (Move what to)) = moveIntro (Just v) what (zoneSort to)
  effIntro (Does s v e) = effIntro e
  effIntro (May d e) = effIntro e            -- a declined May skips at runtime, not in scope
  effIntro (Sequentially es) = effsIntro es
  effIntro (Delayed ev e) = bs               -- a future clause mentions nothing NOW

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
