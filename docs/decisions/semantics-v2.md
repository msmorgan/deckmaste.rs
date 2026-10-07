# Semantics v2 — the encoded-English shape

Decision: 2026-09-06 (draft 2026-08-07). §§1–7 fix the target shape of
the semantics layer — the form the macro and card rewrite aims at — and the
rules for growing it; §§10–16 fix how that layer lands as Rust and how v1
retires. It realizes the divergence trajectory of
[semantics-spelling-lowering §17](semantics-spelling-lowering.md): semantics
drifts toward English constructions, core toward explicit slot reference.
The evidence artifact is the Lean workbench (`lean/Semantics/`, per
[Lean is the workbench](lean-is-the-workbench.md)); Idris paths in the
history of this document are reference only. §3's joined-kind payload was the
last item open; `semantics-v2-crate` settled it (2026-09-06) and §3 records the
answer.

## 1. The layer contract

Semantics v2 encodes the raw Magic card grammar — a reduced but equivalent
form of English — translated from the raw English AST. Constructions occupy
their surface positions: no prenex target slot lists, no hoisted binders, no
positional slot reads. All rearrangement, hoisting, and indexing is lowering's
job. The explicit indexed-binder form of [semantics-spelling-lowering
§7](semantics-spelling-lowering.md) stops being the semantics-layer normal
form; it is core's normal form, produced by lowering.

## 2. Binding and scope

Every constructor argument position is typed in the discourse context of
everything textually earlier: the context threads left to right through the
term, and constructor argument order IS textual order. One binding context
with one read discipline covers targets, choices, and groups; the separate
announce channel disappears. Proven consequences:

- Hoisting itself created the classic read-back problems (an announced slot
  referring to a sibling slot); in situ, those references resolve with no
  positional escape hatch because the later binding does not exist yet at the
  read point.
- "Any other target" is a modifier carrying the presupposition that an
  earlier target exists — distinctness is predicate content, not a slot-list
  side condition [CR#601.2c,115.4].
- A delayed clause reads the discourse as settled particulars:
  [CR#603.7c] refers to particular objects determiner-blind, so announced
  targets stay readable past the boundary; what stays local is announcing
  itself — the delayed ability targets in its own event [CR#603.3d,601.2c],
  so the "other" presupposition stops at the boundary. Staleness is the
  carrier/zone question, and the fire-time zone expectation stays runtime
  (a mismatch is a no-op, not an illegality).
- The activated-ability colon is the same discipline: the effect is typed
  against the cost's mentions filtered to public current zones
  [CR#400.2,400.7j] — an unmoved (tapped) cost mention passes, a
  hidden-zone one does not.

## 3. Context data: surface projections and fold-state only

A binding may record two kinds of data: projections of the mention phrase
(determiner, kind, plurality — everything recoverable from the text) and
fold-state accumulated by threading the effects (the referent's current
zone/carrier, and the keyword-action tag it last moved under plus
whether it stood on the battlefield at that move — the provenance a
definite participle read filters by). A third class is built: event-outcome
referents — an event-producing clause introduces a referent for what it
did, whose sort projects from the clause's surface while its value stays
runtime; "that much" reads its quantity sort-blind (the workbench's
findings 32–33), and the "that many" / "this way" reads follow with
their consumer vocabulary. The value itself is never stored, and
countability is the read site's, not the record's. The record is kind-indexed: a binding can
record only the data its kind can have (an object's head type and zone;
a player or quality has neither), so an ill-sorted binding — "a player
in your hand" — is unrepresentable rather than merely unused. A binding at a
joined kind is pair-carrying: it holds both halves' records side by side, so
its kind is the join of the two halves' kinds and it does carry its
antecedent's kind pair; the half-reading anaphor projects one side rather than
collapsing the pair. Settled 2026-09-06 by `semantics-v2-crate` on the
workbench's own execution shape (`Payload.join`, whose kind is the join of its
halves' and whose zone, type, size and provenance projections read the halves
in turn) and on the census [The kind index joins; union marking is
spelling](kind-index-joins-union-marking-is-spelling.md) already recorded — the
demonstrative echo copies its antecedent's kind pair 33 of 33 times, so a flat
join would reconstruct on every echo what the pair-carrying record keeps.
Nothing else about the union family moves: the marked union constructions stay
spelling-boundary knowledge, as that decision says.
Never rules classifications: timing
(announce vs. resolution), staleness, and agreement are always functions
of the stored surface facts.
The same derive-don't-store rule applies across layers — inflection derives
from the subject, a delayed clause's settled-particular reading derives from
the determiner the author wrote (staleness itself is object identity plus the fire-time zone
expectation [CR#603.7c], as §2 states), and a
hand/library/graveyard destination's owner derives from the moved card itself
([CR#400.3] — the possessive is mandatory surface with fully derivable
content: the style guide's return templating always writes it, and rendering
re-adds it). A move's origin is not derived: the term states it, as a zone or
as `wherever` when the text names none ("from wherever it is" [CR#701.13a]).

## 4. Anaphora and carriers

The category inventory to cover is oracle-style-guide.md §"Names,
self-reference, pronouns, and anaphora" and §"Describing objects, players,
and targets" (heading-anchored evidence of surface convention). Settled
spellings:

- Wildcard pronouns (`It`; player `They`) demand a unique compatible
  antecedent. The sorted demonstrative (`That <noun word>`) demands
  uniqueness after its word filter. There is no nearest-wins tiebreak at the
  semantics layer; the guide's own editorial rule — repeat a noun rather than
  stack ambiguous pronouns — is the type discipline.
  (Superseded as the target by the §7 ruling of 2026-10-07, "Anaphor
  resolution: measured rules replace the strict rule as the target"; this
  remains the checker's behaviour until `lean-drt-anaphora-refactor` lands.)
- "its controller" is `ControllerOf It`: relational nouns compose over any
  noun; there are no fused pronoun forms.
- The carrier word is a typing discipline [CR#109.2,110.1]. Bindings carry
  the referent's current carrier as fold-state; movement re-carriers in place
  ("exile target creature … return that card": after the exile, a card read
  resolves and a creature read must not); and verb slots demand
  the carriers Magic assigns them (destroy takes a battlefield noun).
- The noun-word vocabulary is decomposed, never fused: type words are
  macro-declared catalog atoms (a `TypeDef` confers a type's rules grants,
  even permanence), while the card and player words are the engine's own.
  One vocabulary serves the demonstrative (checked against the referent's
  current state) and the definite participle read ("the exiled card",
  "the sacrificed artifact" — checked against the verb event, filtered by
  provenance fold-state).

## 5. Vocabulary

Type and constructor names come from the repo's own Magic vocabulary: the
oracle style guide's metalanguage (`Noun`, `Carrier`, determiners) and core's
names (`Predicate`, `Effect`, `Object`, `Player`). Linguistics terms of art
(`Nominal`, `Pred`, `Sort`) are not used where the guide or core already has
the word.

## 6. The constructor/macro boundary

Top-level constructors are the engine primitive basis and nothing else. The
classification is consulted per word, never re-derived: the core taxonomy
(`cargo xtask map enums`) plus the v2 macro declarations
(`plugins_v2/builtin/macros/`, §11) are the record of the whittling. Keyword actions,
keyword abilities, and common phrasings generally are phrase-shaped macros over
the basis, mirroring the real definitions' name, params, and body. A core shape
is judged by lowerability and by its algebra; meta constructs are legitimate in
the core. Phrasing knowledge belongs to the card language or the spelling
declarations. The hand-written Lean macro layer
(`lean/Semantics/Macros.lean`) is a stand-in to be generated from those
declarations eventually (`lean-macros-from-ron`). The
`Composite` tag is itself basis: `destroy x = Composite Destroy (Move x
Battlefield Graveyard)` keeps the tag that deontics key on — indestructible
cants the Destroy action and ignores an untagged move [CR#701.8a,702.12b,701.8b]. A
keyword-ability macro carries the keyword's own definition as its
`Ability.keyword` body (ruling, 2026-09-06): a card may write the keyword
bare, and a written definition is an ability of one of the categories the
keyword's registry row declares [CR#702.1]. A
word absent from both records gets a speculative workbench definition,
tagged as such pending the real macro. Where the workbench's expansion story
shows a core variant decomposable — `Sacrifice` is a Composite-taggable
choose-and-move whose restrictions are predicate conjuncts [CR#701.21a] —
the divergence is recorded as a core whittling candidate rather than
mimicked.

## 7. Macro discipline

- One macro per verb lemma. Frames understand inflection, so surface
  agreement variants are one definition; the old macro layer's agreement
  pairs (draw/draws) do not carry into v2.
- Core constructors take every field required. A macro PARAMETER may carry
  a default (§12, ruling 2026-09-07); a constructor field may not.
- An instruction's performer is implicit: it is the ACTOR, the controller of
  the spell or ability [CR#109.5], and another player performs an
  instruction only through the handoff `act(player, instruction)` (Lean
  `Instruction.act`), the one place a performer is written. The noun phrase
  `actor` (Lean `NounPhrase.actor`) reads whoever performs the instruction
  it is part of: outside every handoff it is checked exactly as `you`, the
  innermost handoff wins, a distributive player ("each opponent") performs
  the whole body member by member, and a nested ability (granted, token,
  emblem) starts again with its own controller [CR#109.5]. The agent fields
  the constructors carried were filled with `actor` by helpers until
  `semantics-v2-drop-agent-fields` deleted them (ruling, 2026-10-05,
  `semantics-v2-actor-handoff`, superseding "Agents are explicit" and "The
  imperative's unpronounced subject is supplied by the frame as an explicit
  `You` in the term."). The owner: "Perhaps instructions have an implicit
  performer, which can be handed off … Maybe the explicit actors version is
  more a concern for lowering/core." The ruling does not contradict
  [macros-are-declarative](macros-are-declarative.md): the handoff is a
  model node, and the expander gains no ambient or inherited value. Nor does
  it contradict
  [card-authoring-binds-no-implicits](card-authoring-binds-no-implicits.md):
  constructors still take every field, and a helper wrote `actor` into an
  agent field as an ordinary argument rather than leaving a slot defaulted;
  with the fields deleted there is no slot to fill.
  Amendment (2026-10-05, owner, `semantics-v2-deed-performer-roles`): a
  permanent may be handed an instruction too, since the rules instruct the
  permanent itself ("instruct a permanent to explore" [CR#701.44a,701.63a]);
  owner: "CR literally says the permanent is instructed to explore right? I
  guess we're doing that. whole hog it is". The handoff's performer is a
  player or a permanent (`act(performer, instruction)`; a card in another
  zone or a spell is refused), and inside a handoff to a permanent `actor` is
  that permanent: the steps the rule gives "that permanent's controller" are
  handed on, `act(controllerOf(actor), …)`, inside which `actor` is the
  controller and the permanent stays in view ("that permanent"). An action
  a permanent performs (explore, endure, adapt, harness, monstrosity) written
  with no handoff to a permanent is performed by the source permanent, as an
  action with no handoff is performed by the controller.
  Amendment (2026-10-06, `semantics-v2-drop-agent-fields`): the agent fields
  are deleted. No instruction constructor carries a performer, except
  `choose`'s optional chooser and `insertPart`'s optional taker of an added
  turn or phase, which stay while their removal waits on the owner: each
  would change whether a bench sentence checks (a choice whose chooser is not
  recorded may not be of "one of them"; an added turn must have a taker).
  (Decided 2026-10-06: both are deleted; see the ruling below.)
  Every rule that read a field reads the performer in context instead: the
  actor, or, for the one instruction a handoff to a group hands each member,
  that group, so "each player votes" and "each opponent sacrifices a
  creature" are judged as the deleted plural agent judged them. A deed whose
  facts row names no performer ("fight" [CR#701.14a]) records none, and is
  refused when a handoff names one. Core constructors still take every field
  required: a field was removed, no default was added.
- Agentive verbs — those the CR gives a player actor — put that performer
  in clause position, the factored form of the real macros' agent
  parameter: a dependent context cannot re-use the subject term at each
  inner slot, so the slot rides the clause and lowering redistributes it.
  The clause does not write its performer: it is the actor (ruling,
  2026-10-05, superseding "require it"). Effect-verbs take that performer
  OPTIONALLY, since oracle text writes destroy and exile both with a subject
  and as bare imperatives; object sources are the verb's own argument.
- Choice method is surface data: "of their choice" and "at random" are
  marked indefinites mirroring the real macros' explicit chooser slot and
  its absence in the random variant; no CR rule derives a chooser.
- Each macro matches exactly one English phrase shape — the property frames
  compile against.
- A verb's implicit restrictions must be expressible both ways — as predicate
  conjuncts on a choice ("sacrifice a creature" chooses among `InZone
  Battlefield`, `ControlledBy` the actor) and as fold-state demands on a
  definite referent ("sacrifice it" requires the referent currently
  battlefield-and-controlled) — and the two must agree. A restriction that
  cannot take both forms is un-de-macroed verb structure hiding in a filter.
  An indefinite patient that names no zone ("discard a card") is selected
  where the deed's patient lives, so the verb supplies its zone (ruling,
  2026-10-05); the controller or possessor half is not yet checked, because
  an object binding records no possessor
  (`semantics-v2-bindings-carry-no-possessor`).
- **One stat change carries both deltas.** "<subject> gets +P/+T" and
  "<subject> has base power and toughness P/T" are one node,
  `StaticSpec.ptModification subject power toughness` (mirror
  `PtModification`, RON alias `ptModification`), with the subject written
  once; each delta is the one a `modification` of that stat would carry
  [CR#613.4b,613.4c]. A change to one stat stays `modification`. The
  two-modification spelling needed its toughness half to read the subject
  back as a computed pronoun (Lean `itsOther`), which a RON template cannot
  write, so a RON `gets` wrote the subject twice: two targets where the card
  has one (ruling, 2026-10-05, `semantics-v2-gets-both-deltas`).
- **A counter's kind is a name; a counter definition states what one counter
  confers.** `CounterKind` has one constructor, `named name` (mirror
  `CounterKind::Named { name }`), whose name is the counter declaration's
  name verbatim (`p1p1Counter`, `flyingCounter`, `chargeCounter`): counters
  with the same name are interchangeable [CR#122.1], and the shapes that
  restated a definition (`Boost(power, toughness)`, `Keyword(keyword)`,
  `Named(label)`) are gone. It is a type of its own rather than a `String`, so
  `denoted_by` (§12) still reads a bare declaration name at a counter
  position. A kind is valid when the generated counter facts have a row of
  that name, and the holder is read from that row for every kind. The
  declaration does not write its kind: the `CounterKind` meta-macro builds the
  `Counter` definition from the declaration's `name`, `holder` (an object
  unless written) and `confers`, as the subtype metas build theirs, so
  nothing in a declaration refers to the counter it declares. `confers` is
  what ONE counter confers and the model applies it once per counter held:
  the +1/+1 counter confers "+1/+1" (`boost(up(1), up(1))`, one
  `ptModification`) [CR#122.1a,613.4c] and a keyword counter its keyword
  (`grants(keyword("Flying"))`) [CR#122.1b,613.1f]; no conferral reads the
  count of its own kind. Considered and rejected: a "this kind of counter"
  form, and a "this macro" expander hole, which would be a new expander
  feature ([macros-are-declarative](macros-are-declarative.md)). The
  declaration's `spelling` stays the compound's stem ("+1/+1", "charge").
  The annihilation of +1/+1 and -1/-1 counters [CR#704.5q] is not derived
  from the kind and stays with `semantics-v2-counter-annihilation-sba`.
  (Ruling, 2026-10-05, owner, `semantics-v2-counter-kind-is-a-name`; the
  type's shape is the orchestrator's call; the helper names `boost` and
  `grants` were accepted by the owner the same day.)
- **A designation is a name.** `DesignationLabel` has one constructor,
  `named name` (mirror `DesignationLabel::Named { name }`), whose name is the
  designation declaration's name verbatim (`goaded`, `monarch`, `ringBearer`,
  `leftHalfUnlocked`), never the phrase a card prints: "the monarch" and
  "Ring-bearer" stay the declaration's `spelling`. It is a type of its own
  rather than a `String` alias, so `denoted_by` (§12) reads a bare
  declaration name at a designation position (`gainDesignation(this,
  goaded)`, `hasDesignation(monarch)`). The declaration does not write its
  label: the `Designation` meta-macro builds the `Designation` definition from
  the declaration's `name` and its columns (`scope`, `effectful`, and `zone`,
  `type` and `half`, each `None` unless written). A declaration whose spelling
  covers several designations lists them as `members`, each its own name with
  the declaration's columns: `dayNight` ("day or night") lists `day` and
  `night` [CR#731.1], each of which reads bare, while `dayNight` itself names
  no designation and does not. The loader registers the members
  (`deckmaste_semantics_v2::designations`), and the facts generator writes
  one `DesignationFacts` row per designation, labelled with its name; the
  checker looks a designation up by that name. (Ruling, 2026-10-05, owner,
  `semantics-v2-designation-is-a-name`: "I'd prefer to avoid stringly typed
  so yes", and, on each label being its own bare name, "yes."; the type's
  shape is the orchestrator's call.)
- **The checker's charter: references resolve and terms are well-kinded.**
  The Lean checker (`lean/Semantics/Check/`) admits a written term as a
  faithful reading of the card's English. Its charter is that references
  resolve (anaphora, outcomes, choices, stamps, scope windows, plurality, the
  handoff performer) and that terms are well-kinded. It is not a rules
  engine: an instruction the game cannot carry out is still a faithful
  reading, because the rules say any part of an instruction that is
  impossible to perform is ignored and an effect that attempts the impossible
  does only as much as possible [CR#101.3,609.3]. Consequences:
  (a) no new refusal that asserts a game rule about zones, timing windows,
  costs and deontics, tokens, grants or the card frame lands without a dated
  ruling; (b) the existing ones are retired bucket by bucket by
  `lean-checker-sweep-zone`, `lean-checker-sweep-timing`,
  `lean-checker-sweep-costs` and `lean-checker-sweep-game-rules`, each
  retirement recording whether any retired pin ever encoded a real card
  defect; (c) zone state on bindings stays, because `that(Permanent)` against
  `that(Card)` resolves on it (the proposal to strip zone and provenance from
  bindings was rejected 2026-10-05, `lean-checker-binding-ids`
  "Provenance"); (d) the card-frame rules may survive the sweep at the
  owner's call, since they are not instructions and do not move with the
  model. This supersedes `lean/CONTRACTS.md`'s "Its authority is rules
  correctness and internal consistency" on the question of scope; the header
  there is amended to match. Evidence: `Refusal` has 234 constructors
  (`lean/Semantics/Check/Refusal.lean`), of which 50 are reference resolution
  (282 pin lines under `lean/Semantics/Proofs/`), 83 term shape (208 pin
  lines), 33 zone and object state (174), 10 timing and window (25), 15 cost
  and deontic (60) and 43 other game rules (132). The breadth is inherited:
  the Idris prototype (`idris/src/Experimental/*.idr`) carried about 1,100
  obligation sites and the Lean type is its port ("One constructor per Idris
  obligation"); Lean added `handoffPerformer`, `opponentsLibrary`,
  `keywordCostPaidByYou`, `tokenNamed`, `definitionHolder`,
  `definitionNamed`, `definitionScoped`, `amountParameter`, `lexicalScope`
  and `bindingless`. (Ruling, 2026-10-06, owner: "what I truly care about is
  that anaphora and other backreferences resolve; other concerns keep
  accruing by accident".)
- **A macro's referent parameter is bound at its first mention; later
  mentions refer back.** A referent parameter (`Subject`, `NounPhrase`) of a
  registry macro is introduced by the body's first mention of it; every later
  mention is a back-reference to that binding, resolved by binding identity,
  not by pronoun search. Substitution by copy stays for a parameter mentioned
  once, which is 223 of the 228 macros with a referent parameter: they expand
  byte-identically. Rejected: binding at the call, because it hoists the
  argument out of its clause (`act(each(opponent), discard(a(card)))` would
  choose one card for every opponent, and a choice under `may` or `doIf`
  would be made whether or not the branch runs). Rejected: an opt-in
  `Capture(Subject)` parameter annotation. The v1 `Target(N)` device is not
  re-adopted. (Ruling, 2026-10-06, owner,
  `semantics-v2-macro-capture-and-plurality`: "really painted myself into a
  corner here didn't I. I guess B'.")
- **Deeds and keyword labels are declared names.** A keyword action is
  referenced by its declaration name read bare where a deed is wanted
  (`theVerbed(exile, Many)`, `enact(discard, …)`), and a keyword ability
  likewise where a keyword label is wanted (`grants(flying)`); the loader
  refuses an undeclared name at load time. Lean keeps `Deed.action name` and
  `Deed.ofAbility name` (`lean/Semantics/Words.lean`), as it keeps
  `CounterKind.named`. The fifteen core deeds (`CoreDeed`) stay a closed enum.
  This is the shape of the 2026-10-05 rulings "a counter's kind is a name"
  and "a designation is a name". The stamped pronouns `itVerbed` and
  `themVerbed` retire into `theVerbed` with an optional noun word; `it` and
  `them` are not overloaded. (Ruling, 2026-10-06, owner,
  `semantics-v2-deed-is-a-name`: "I don't want to overload it and them like
  that, it'd take new macro_ron features".)
- **A keyword body has its own reference scope; helper macros are
  transparent.** Every keyword action body and every keyword ability body is
  read in its own reference scope, seeing only its parameters and what it
  introduces. The scope opens at the loader's `Enact` wrapper, the keyword
  action boundary, never at an `act` handoff: `act(they, discard(that(Card)))`
  must see the choice made before it. A helper macro reads the scope of the
  body that calls it. A helper that bakes in a pronoun where `verb(it)` says
  the same thing is retired to the general form; a helper that carries its
  own `spelling` stays. (Ruling, 2026-10-06, owner,
  `semantics-v2-keyword-body-reference-scope`, settling its open items;
  `sacrificeIt` is "`add1 = (+) 1`".)
- **`choose` has no chooser and an added turn part no taker.** `choose`'s
  optional chooser and `insertPart`'s optional taker are deleted: both are
  the actor in context, as every other performer is since the 2026-10-05
  handoff ruling, with no performer exception. The part's kind carries the
  difference the taker carried: a turn is taken by the player the
  instruction was handed to [CR#500.7], and a phase or step is added to the
  current turn [CR#500.8,500.9]. The RON helpers become `extraTurn`,
  `additionalPhase(…)` and `additionalStep(…)` over one renamed node.
  (Ruling, 2026-10-06, owner, `semantics-v2-chooser-and-taker-fields`:
  "insertPart? weirdass name".)
- **Gift and the additional-cost keywords share one mechanism.** Gift has one
  definition whose second ability is guarded by the card's class
  [CR#702.174a,702.174b]. The second ability reads the first through the
  paid-cost facets the registry already uses for kicker, as the rules link
  them [CR#607.2d,607.2i,702.33e], with a third facet: the player chosen as
  the named cost was paid. A keyword definition is a list of parts, each
  carrying the regime the rules give it
  [CR#702.174a,702.175a,702.157a,702.153a]; kicker is the one-part case
  [CR#702.33a]. (Ruling, 2026-10-06, owner,
  `semantics-v2-linked-choice-readback`,
  `semantics-v2-keyword-definition-by-card-class`,
  `lean-keyword-definition-regimes`: "maro might say that these abilities are
  basically kicker, so we should use a consistent mechanism across the lot.")
- **The bearer exemption stays narrow until the zone sweep.** The exemption
  in `Conferral.checkProperty` (`lean/Semantics/Check/Rules.lean`) covers only
  a two-stat `ptModification` on `this`, until `lean-checker-sweep-zone`
  retires the battlefield demand altogether. No counter kind needs a wider
  rule: of 73 counter declarations only `p1p1Counter` and `m1m1Counter`
  confer a stat change, both through `boost`. (Ruling, 2026-10-06.)
- **Anaphor resolution: measured rules replace the strict rule as the
  target.** Accessibility comes first. A mention is inaccessible to a read
  when the consuming verb cannot apply to it by zone or kind: a creature that
  died cannot take a counter, and a non-permanent cannot be sacrificed
  [CR#109.2,701.21a]. This exclusion runs before any resolution rule. The
  checker's bindings already carry zone and kind, so it is a condition on the
  referent in the discourse-representation sense. The clause-mates of an
  anaphor are the co-arguments of its verb only. A possessor ("its power") and
  the object of a small clause ("counters on it") are not clause-mates, which
  keeps the pin `badItAcrossOwnSlot` and the Weeping Angel corpus case. Among
  the accessible referents, `it` and "that N" keep different rules. Bare
  `it` takes the referent in the nearest clause, with grammatical role
  (subject, then object, then other) as the tie-break, and a true tie still
  refuses. On the 61 ambiguous sites in a 150-site sample this is 60 correct
  under both coders' features: 60 / 1 / 0 (correct / wrong / refused) with
  coder 2's, Grip of Phyresis the one wrong, and 60 / 0 / 1 with coder 1's,
  Cocoon refused. "That N" first removes the anaphor's clause-mates and whatever a
  pronoun in its own clause resolved to, then takes the most recent of what
  remains. On the 87 ambiguous sites this is 76 correct, 11 wrong and
  0 refused. Both coders' keys put it wrong on the same ten cards; the
  eleventh is Noetic Scales for coder 1 and Tragic Banshee for coder 2, a
  fold-state site the accessibility exclusion above settles. The 11 "that N" sites are known-wrong: Tahngarth, First Mate; Unpredictable
  Cyclone; Eriette, the Beguiler; Runesword; Scythe of the Wretched; Bronze
  Bombshell; Gisela, Blade of Goldnight; Solphim, Mayhem Dominus; Ram Through;
  Mangara's Equity; Noetic Scales. Each resolves silently to the wrong
  referent, and the round-trip cannot catch it because the rendered English
  is identical. Each needs an explicit selector when its card is written, and
  the refactor pins all 11. This is the target: it supersedes the strict
  statement in `lean/CONTRACTS.md` ("Grammatical references and internal
  windows") and §4's "no nearest-wins tiebreak". The checker keeps its
  current behaviour until `lean-drt-anaphora-refactor` lands. Evidence:
  `docs/tickets/done/semantics-v2-anaphor-resolution-heuristics.md`
  ("Measured 2026-10-06", "Blind second coding and agreement") and the ignored
  local directories `docs/evidence/anaphora-measurement-2026-10-06/`,
  `docs/evidence/anaphora-blind-2026-10-07/` and
  `docs/evidence/anaphora-agreement-2026-10-07/`. (Ruling, 2026-10-07, owner,
  `semantics-v2-anaphor-resolution-heuristics`. The accessibility and
  clause-mate judgments are provisional: "ok to both I guess. we'll try this
  way for now anyhow". The rules were accepted with "ok".)
- Predicates are flat sibling modifier sets on one referent ("a creature an
  opponent controls" is two modifiers on one object predicate); zone
  membership (`InZone`) is an ordinary conjunct.

## 8. Process

The workbench grows chapter by chapter in Lean: pick one structural
question, ground it in real parses (`cargo xtask english inspect` /
`bracket`, corpus lines) and the style guide, prove the shape with positive
pins and pinned negative refusal lists, record findings in the module doc.
Reference machinery comes before vocabulary breadth. The exhaustive
English-AST-to-workbench crosswalk once planned as the completion audit is
retired (`semantics-v2-lean-spec-crosswalk`, 2026-09-06); completion is
parity (§14).

## 9. Affected contracts and tickets

- [semantics-spelling-lowering](semantics-spelling-lowering.md): §7's indexed
  scope calculus becomes core-only normal form (see §1 above); §17's
  trajectory is realized by this contract; §9's lowering contract carries
  over to `deckmaste_lowering_v2` unchanged.
- [idris-is-a-soundness-gate](idris-is-a-soundness-gate.md): superseded for
  `plugins_v2/` by the Lean gate of §13, which `lean-card-soundness-gate`
  landed (2026-09-06); v1 canon stays on the Idris gate until
  `semantics-v1-cutover`.
- [english-v2-rewrite](english-v2-rewrite.md): its cutover and §14's are
  coordinated by `semantics-v1-cutover`; `macro_ron` is retained by both.
- [workbench-ron-shaped-and-label-rulings](workbench-ron-shaped-and-label-rulings.md):
  the direction of travel stays RON → workbench; §11 names the RON.
- The ticket chain: `semantics-v2-crate` → `lean-card-soundness-gate` →
  `lean-rules-tables` and the four `semantics-v2-macro-bodies-*` families →
  `plugins-v2-canon` (with `lean-hand-bench-retirement`) → `lowering-v2` →
  `semantics-v2-parity` → `semantics-v1-cutover`; `lean-constructor-collapse`
  follows parity. Parked: `lean-macros-from-ron`,
  `semantics-v2-english-translation`.

## 10. Rust representation

`crates/deckmaste_semantics_v2` mirrors the Lean syntax
(`lean/Semantics/{Words,Events,Phrase,Triggers,Abilities,Card,Rules}.lean`)
and the registry fact columns (`lean/Semantics/Check/FactTypes.lean`, the
`facts` module) constructor-for-constructor and field-for-field. Lean is the
specification: a Lean change is a Rust change, never the reverse, and a drift
test in the crate fails on any name the two sides disagree on. The crate does
no law checking. The Lean gate (§13) is the only checker; `deckmaste_lowering_v2`
may fail to lower a card that breaks a law, without going out of its way to
validate. The crate depends on `macro_ron` and on nothing deletion-bound
(`CLAUDE.md` "Crate fates"); in particular it never depends on
`deckmaste_construction_core` or `deckmaste_spelling`. It exposes as plain
functions the structural reads lowering needs (binding resolution, kind
projection) and nothing that refuses.

`Check/FactTypes.lean` holds the row types of the tables `cargo xtask facts
generate` writes, and only those; the mirror contract covers exactly them. A
fact type the checker owns rather than the registry is declared beside its
hand-written table in `Check/Words.lean`, outside the mirror and with no Rust
counterpart — `ActFacts` and its columns (`DeedRole`, `ReferentSort`,
`EntityDomain`, `ObjectClass`, `PremiseSort`, `DeedFeature`) are the case, all
three of their tables (`coreDeedFacts`, `abilityDeedFacts`, `actFacts`) being
hand-written Lean. Moving a fact type between the two homes is therefore a
change to what the drift test governs, and belongs here rather than in a
generator ticket.

## 11. Plugin format

`plugins_v2/` is the v2 plugin format. `plugins/builtin_v2` moves to
`plugins_v2/builtin`, with a symlink at the old path until all declaration readers use the new one. One declaration file per macro carries `name`,
`params`, `spelling`, `grammar`, and a semantic `body`: the lexical-source adapter reads the
spelling and grammar, semantics_v2 reads the params and body. A keyword
declaration's file writes only the part of its definition that does not follow
from its name and signature — a keyword ability's list of abilities (omitted
when empty), a keyword action's instruction, never who performs it — and
semantics_v2 builds the `Keyword(...)` or `Enact(verb: Action(...), ...)`
wrapper around it (`deckmaste_semantics_v2::keywords`), the deed performed by
the actor in context (§7, ruling 2026-10-05; under a handoff to a permanent
the actor is that permanent, amendment of the same date); a file writes
`keyword_params` or `deed: None` where its definition does not take the
derived arguments or the deed.
The file is
the shared contract; neither crate depends on the other for it, and an xtask
drift test loads every declaration both ways. Today's bodyless declarations
(keyword actions, keyword abilities, ability words, turn parts) grow bodies
per family; no new bodyless declarations are added. A registry declaration's
body is the DEFINITION of what it declares — `Semantics.Definition`, one
`Rules.lean` constructor per registry family — and `cargo xtask facts generate`
DERIVES `lean/Semantics/Check/Facts.lean` from it: every counter definition is
a `CounterFacts` row labelled with its kind, its declaration's name (§7,
ruling 2026-10-05), a subtype definition a `SubtypeFacts`
row, a designation definition a `DesignationFacts` row labelled with its
name, the declaration's or each listed member's (§7, ruling 2026-10-05), and a keyword
ability's own `Ability.keyword` term supplies its word and the categories its
definition is written in. Rust owns no mapping into the RON files (ruling,
2026-09-07, `semantics-v2-definition-bodies`, superseding
`facts-generator-sheds-v1`'s "a declaration carries its own facts columns as
its body"). An xtask overlay survives only for a column no declaration
carries, and each survivor is named in that ticket's landing record: the
keyword-ability gate columns, the keyword-action checker columns, and the
subtype frame column. Cards, tokens, and the
three rules tables (state-based actions, conferrals, damage results) are
further kinds in the same tree, all semantics-language RON. `plugins_v2/canon`
is hand-authored RON first; translation from the retained `deckmaste_english_v3` Readings is a later crate (`semantics-v2-english-translation`), with
RON as the cached form of that translation.

The builtin macro families live at `plugins_v2/builtin/macros/<family>/`,
alongside `macros/meta/`, whose declaration meta-macros the family files
invoke. They were under `macros/stubs/` while they were bodyless; they are
declarations now.

**Case is the mark.** A declaration's name is Lean's own macro name, verbatim
and camelCase — `flying`, `theRingTemptsYou`, `willOfTheCouncil` — while the
mirror's constructors stay PascalCase, so `draw` is the macro and `Draw(…)` the
constructor, exactly as Lean's `.draw`/`draw` separates the two (ruling,
2026-09-07). The file stem is still the name. Nothing else moves: spelling,
grammar and every semantic label a body writes are unchanged, and the emitted
Lean is byte-identical across the rename. The keyword declarations take a name
as a LABEL rather than as an identity and capitalise it at that boundary — a
keyword ability's label is its name capitalized, a keyword action's the words
its name spells, each capitalized (`deckmaste_semantics_v2::keywords`, which
`cargo xtask facts` reads them through) — and one reader stopped taking it at
all: an ability word's italic label is now its
SPELLING, which is what the bench writes (`abilityWord "will of the council"`).
`lean-macros-from-ron` is an identity on names, with one mechanical
exception: a RON name that is a Lean keyword is spelled in Lean with a
trailing underscore — `exists` ↔ `exists_` — the escape the mirror's
constructors already take (`lean_emit::LEAN_ESCAPED`). RON is not bound by
Lean's reserved words, so the declaration keeps the plain name (ruling,
2026-10-04).

Nineteen helpers are named apart from Lean's: `aRandom` (Lean `aAtRandom`), `card` (Lean `isCard`; the owner, 2026-10-05), `random` (Lean `countedAtRandom`), `revealHand` (Lean `Actor.revealHand`, the namespace dropped), `mayCastFrom` (Lean `Actor.mayCastFrom`, the namespace dropped; RON adds a `paying` parameter defaulting to Lean's `ItsOwnCost` and an `exclusive` one defaulting to Lean's `false`), and `exileFrom`, `selectRandom`, `theirHand` and the six exchange helpers `exchangeControl`, `exchangeLifeTotals`, `exchangeCards`, `exchangeZones`, `exchangeValues` and `exchangeTextBoxes` (no Lean macro; the bench writes the constructor alias `Primitives.Instruction.exchange` with the `Exchanged` arm, and each helper expands to that same term), and `getsBoth` (no Lean macro; "<subject> gets +X/+X", the one delta written for both stats of `StaticSpec.ptModification`, §7), and `artifactCreatureToken`, `createTokenCopy`, `discards` and `mayOrElse` (no Lean macro; each expands to the term the bench writes in its place: the creature token bundle with `artifact` beside `creature`, `create` of a `copyOf` token, `verbedEvent` with the `Discard` deed, and `withContinuation (.optional actor)` with only its "if they don't" branch).

The nursery is SHARED, and `read_builtin_v2` takes its nine spelled families
by name — `ability_words`, `counter_kinds`, `designations`, `flavor_words`,
`keyword_abilities`, `keyword_actions`, `subtypes`, `turn_parts`, `types` —
ignoring every other directory there rather than refusing what it cannot
place. Beside them live `macros/meta/` and the HELPER macro families (§12),
which are declarations of no construction at all: semantics_v2 reads every
`.ron` under `macros/`, construction_core reads the nine, and the xtask drift
test compares the two readings over the nine.

### 11.1 What a card may write

The dialect is what the Lean bench writes, spelled in RON. Four rulings
(2026-09-07, `plugins-v2-dialect`), each a reader rule rather than a
per-type convenience, and each off by default in `macro_ron` — v2's
`MacroSet` turns them on, v1's does not:

- **Unknown fields are refused.** A named argument a constructor does not
  declare is an error naming both, never a skipped key. serde's default —
  ignore what you do not recognize — read a misspelling as an omission, so
  Fading and Impending wrote `amount:` at a `quantity` field and got a
  count-less removal, and `conferral:` arguments left over from a retired
  signature passed unnoticed. `MacroSet::denying_unknown_fields` turns the
  check on; v2's macro set does.
- **A constructor that carries only its payload may be left unwritten.** A
  constructor of exactly one field whose type is another syntax type is an
  INJECTION: it adds no information, so the reader supplies it. `Green` at a
  mana-symbol position is `Simple(symbol: Specific(color: Of(color: Green)))`,
  one hop at a time. The mechanism is v1's `#[macro_ron(embed)]` (see
  `deckmaste_semantics`'s `mana.rs` for why an untagged fall-through rather
  than `serde(untagged)`), extended one way the mirror needs: the embed
  variant may be a STRUCT variant of one field, so it keeps the binder name
  that is the contract with the Lean constructor. Everything else about
  `embed` is unchanged, the ONE-PER-ENUM rule above all — two untagged
  fall-throughs at one position would race for the same identifier, and the
  derive refuses a second. The chain is hand-chosen, never enumerated from
  the mirror: today `ManaSymbol::Simple`, `SimpleManaSymbol::Specific`,
  `ColorOrColorless::Of` and `ColorTerm::Lit`, which is v1's chain mapped
  onto v2; `CounterKindSource::Printed`, so a bare `p1p1Counter` is a
  printed counter kind (ruling, 2026-10-03: "p1p1Counter should read bare,
  with Printed the default constructor"); `NounWord::Type`, so "that
  creature" is `that(Creature)` (ruling, 2026-10-04); and `Deed::Core`, so a
  deontic rule's deeds read `[Block]` for `[Core(deed: Block)]`. Native dispatch always wins: an identifier the position's own
  grammar or macro namespace claims is never routed.

  An injection also WRITES bare, so a card file spells the colour and
  nothing else. The elided constructor is not lost on the way out: the
  variant's payload is written through a newtype struct named
  `Type.Variant`, which ron's `unwrap_newtypes` extension drops, while a
  structural consumer that must name every constructor — the Lean emitter
  (§13) — reads that name and restores the application.
- **A constructor may be applied positionally.** Arguments in the
  constructor's declared binder order, which is what Lean writes:
  `Hybrid(Generic(amount: 1), Red)` for
  `Hybrid(left: Generic(amount: 1), right: Red)`. The reader does not rewrite
  the text: it hands the position to ron's own `deserialize_any`, whose
  lookahead decides which form is written and calls `visit_map` or
  `visit_seq` — both of which serde's derived struct visitor implements, so
  the binder names and every forwarded `#[serde(...)]` still govern. The two
  forms are NEVER mixed: `C(a, b: c)` is refused, so `choose x (disclosure := .secretly)`
  has no RON spelling and a binder that must be skipped takes the wholly named
  form.

  A constructor of ONE field has nothing for ron's lookahead to see — inside a
  newtype variant `(x)` is a newtype tuple, and `handle_any_struct` turns the
  bare identifier into a unit value, discarding the very name the argument is
  — so the reader classifies the argument list itself: `macro_ron`'s value
  capture takes the list, and a list that opens with a binder re-reads as the
  named struct while any other re-reads as a one-element tuple. `CountOf(x)`
  and `CountOf(group: x)` are the same value. Nothing is rewritten; the only
  text this makes is the pair of parentheses the capture left behind. The
  capture at that position does not go through ron's own `RawValue`, whose
  `Deserialize` re-parses what it captured and refuses anything that is not a
  standalone value — a fused argument list is not one.
- **A named-signature macro may be applied positionally.** Its parameters
  retain declaration order from the definition file, and a positional call
  maps its argument sequence onto that order: `hasType(Creature)` is the same
  invocation as `hasType(type: Creature)`. Named substitution still resolves
  `Param(type)`, while remembered invocation provenance retains the call's
  positional spelling. The two forms are never mixed, under the same rule as
  constructor application.
- **A card may write only macros.** At a `semantic_expression` kind — the
  seventeen `crate::ron::EXPRESSION_KINDS` mirror Lean's own attribute — a
  constructor's name has no native candidacy in a CARD, and the macro of that
  name stands in its place: `keyword(label: "Flying")`, never
  `Keyword(keyword: "Flying", …)`. This is Lean's `Authoring.Form.onlyMacros`,
  which refuses a `.raw` constructor of a `semantic_expression` type and admits
  a macro call, a literal leaf, or a parameter. The mechanism is v1's
  restricted read (`macro_ron::MacroSet::read_str_restricted`, spec §4): the
  restriction is opt-in at the ENTRY, and only the card containers read that
  way, so a macro's own body — the basis those macros are written over — keeps
  every constructor. An argument keeps the restriction of the text it was
  written in, so a card cannot launder a constructor through a macro's
  argument. Two carve-outs keep the rule Lean's rather than wider: every
  non-expression registered kind marks its own variants natively spellable (a
  colour, a subtype, a turn part, the mana chain), and the two literal leaves
  (`Amount::Lit`, `SimpleManaSymbol::Generic`) stay written-out-able, which is
  what `semantic_literal` buys in Lean. A hand-built kind carries no dispatch
  set to read variants off, so `HeaderPossessor`, the one left, names its
  variants in `ron.rs`; `Subtype`, `CounterKind` and `DesignationLabel`
  derive theirs.
- **A numeral reads at its leaf.** Lean marks `Amount.lit` and
  `SimpleManaSymbol.generic` `semantic_literal`; the mirror marks the same
  two `#[macro_ron(literal)]`, the marker v1 already uses for `StatValue`'s
  bare `-1`. A struct-variant literal carries its binder to the splice, so
  `3` reads as `Lit(value: 3)`. A numeral also falls through an INJECTION to
  a leaf further down — a position that embeds another type and has no
  numeral leaf of its own passes the numeral on — so `[2, Green, Blue]` is a
  mana cost and `power: 3` is an amount. Like an injection, a numeral leaf
  writes bare and carries its constructor to the Lean emitter the same way.

Which constructors are injections and which are numeral leaves is declared on
the mirror, beside the Lean constructor each one mirrors; `macro_ron` carries
no policy of its own. The reader rules are not per-type conveniences but they
are per-consumer: a `MacroSet` that declares none of them reads exactly as it
did before, which is what keeps v1 — the embed mechanism's other user —
unchanged.

## 12. Macro system

`macro_ron` remains a dumb expander whose `MacroDef<Metadata>` carries
consumer-typed, opaque metadata and its named parameters in file declaration
order. Kinds are one per `SupportsMacros` enum and
exist only to disambiguate same-name macros at different usage sites; Lean's
`MacroParameters` classes are a proof device, not the kind set. Macros
invoke other macros up to the expander's depth limit; there is no
self-recursion. §7's discipline (one macro per phrase shape, term-for-term
expansion) is unchanged, except that a macro PARAMETER may now carry a
default: the no-default-slots rule was an Idris carryover about core
constructors, and Idris retires with v1 (ruling, 2026-09-07). A helper macro
declares Lean's named defaults as defaults on the parameter, and generated
Lean carries them natively. Core constructors take every field required, as
[card-authoring-binds-no-implicits](card-authoring-binds-no-implicits.md)
says.

`macro_ron` gains the reader rules §11.1 records — the unknown-field refusal,
injections, positional application, numeral leaves — all off by default and
all consumer-declared: two `MacroSet` switches (`denying_unknown_fields`,
`reading_positional_arguments`) and three `#[macro_ron(...)]` markers on the
mirror (`embed`, `literal`, `denoted_by`). A `MacroSet` with neither switch,
over types carrying none of the markers, reads exactly as it did before.
`denoted_by(Spelling, term = path)` is the type-level one: it declares that a
Registry Definition's spelling also reads at the type's position and denotes
the term `path` finds in it — a definition's name denotes its term, so
`p1p1Counter`, whose body is its `Definition::Counter` node, reads at a
`CounterKind` position as that node's `kind`, which is the name itself
(`Named(name: "p1p1Counter")`, §7), a subtype declaration reads
at a `Subtype` position as its `subtype`, and a designation declaration
(`goaded`), or a member a declaration lists (`day`), reads at a
`DesignationLabel` position as its `label`, which is that name (§7, ruling
2026-10-05). The spelling is not one of the type's variants, so a card writing
the node raw is refused by name; the projection is the mirror's
(`Definition::counter_term`, `Definition::subtype_term`,
`Definition::designation_term`), and `macro_ron` knows nothing of it (ruling,
2026-10-03; the derive marker, 2026-10-04). What a macro's own
BODY may write is what a card may write, because a body is read at the
position it expands to and goes through the same reader. A macro's declared
signature still determines its hole vocabulary: positional signatures resolve
`Param(i)`, while named signatures resolve `Param(name)`. With positional
argument reading enabled, the latter accepts either a wholly named call or a
sequence mapped by declaration order; mixed calls remain invalid.

### 12.1 The helper macro layer

`lean/Semantics/Macros.lean`'s 421 `semantic_macro`s are the phrasings a card
writes over the constructor basis. 316 of them are declarations under
`plugins_v2/builtin/macros/<family>/` — the families are the Lean file's own
sections (`pronouns`, `quantities`, `determiners`, `zones`, `predicates`,
`nouns`, `mana`, `durations`, `amounts`, `instructions`, `events`,
`abilities`, and `conferrals` for `boost` and `grants` below) — each a plain meta with `name`, `kinds`, `params` and `body` and
no spelling or grammar. Lean's `counters` section (`p1p1Counter`,
`m1m1Counter`, `flyingCounter`, renamed from `plusOnePlusOne` and
`minusOneMinusOne` on 2026-10-05) has no declarations under these families:
the counter declarations of the same names are those kinds and read bare at a
counter position. Its two conferral helpers, `boost` and `grants`, are
declarations under `macros/conferrals/`, the family a counter declaration's
`confers` is written in. The name is the Lean name verbatim (§11), the kind is
the Lean return type, and the signature is NAMED, with Lean's own defaults as
parameter defaults. A RON signature may default a binder further, and move a
defaulted binder last, so that a positional call can leave it out; and a binder
Lean escapes as a keyword (`by_`, `from_`, `while_`, `as_`) takes its plain
name. A list-typed parameter registers under the PLURAL of its
element type, v1's convention (`Abilities` for `Vec<Ability>`); a Lean
`abbrev` registers under the alias's own name, since that is what the
signature writes. `lean-macros-from-ron` generates `Macros.lean` from these
and around the rest.

The 29 that were blocked because their name is a constructor of their own kind
are unblocked by §11's case rule and 27 of them ported here: Lean's `.draw` /
`draw` distinction is now RON's `Draw` / `draw`, so an alias macro is an
ordinary declaration. The other two of the 29, `shuffle` and `vote`, are the
next bucket — their keyword-action declarations already own the name, with a
body that is the constructor itself, which names no performer (§7, amendment
2026-10-06).

The other 103 stay Lean-only, in eight buckets. (Counts recounted
2026-10-05 at `semantics-v2-actor-handoff`, and again at
`plugins-v2-implicit-actor-spelling`, where `revealTheirHand` left the
declarations, and at `plugins-v2-keyword-helper-additions`, where
`returnToBattlefieldWithCounters` left them and `mayCastFrom` joined them:
every line of `Macros.lean`
beginning `semantic_macro`, an `Actor` namespace prefix kept; a macro is a
declaration when a file of the same name, a trailing `_` dropped, exists under
one of the twelve families above, `macros/conferrals/` or `macros/conditions/`, or when §11 names
its helper apart. 426 = 316 declarations + the two `counters` macros + 108;
`semantics-v2-counter-kind-is-a-name` renamed the `counters` macros and added
`boost` and `grants`, both declared, and the recount after the merge with
`plugins-v2-keyword-helper-additions` found 316 declarations. Six macros the earlier count left here had ported since and are gone from
their buckets: `chooseModes`, `createTappedAttacking`, `dealsCombatDamage`,
`doUnless`, `forEach`, `leavesZone`. Recounted 2026-10-06 at
`semantics-v2-drop-agent-fields`, which retired five `Actor` helpers: 421 =
316 declarations + the two `counters` macros + 103.)

- **A spelled declaration already owns the identity (17).** The keyword
  families keep their own declarations: `companion`, `destroy`, `discard`,
  `exile`, `fight`, `flying`, `flyingCounter`, `levelUp`, `mill`,
  `proliferate`, `regenerate`, `sacrifice`, `shuffle`, `tap`, `transform`,
  `untap`, `vote`. A ported body that calls one of these calls the
  DECLARATION, under its declaration's positional signature.
- **It calls a `Primitives.*` helper (24).** `Semantics.Macros.Primitives`
  holds hand-written macros beside the constructor wrappers
  `declare_semantic_primitives` generates; a wrapper is the constructor and
  converts as one, but the hand-written helpers are a second Lean-side layer
  this port does not cover: `addPart`, `addPartThen`, `attachChoosing`,
  `attachToIt`, `become`, `becomeColor`, `create`,
  `cumulativeUpkeepExpansion`, `doesntUntap`, `entersChoosing`,
  `entersChoosingFrom`,
  `entersChoosingPlayer`, `entersChoosingPlayerSecretly`,
  `getAdditionalPart`, `leavesBattlefield`, `mayDeclineUntap`,
  `offer`, `offerWhen`, `putIntoFrom`, `splitOverPermanent`,
  `splitOverPlaneswalker`, `untapsDuring`, `youAnd`, `youOr`.
- **It computes (22).** Not a substitution bundle: a `let`, a `match`, a
  `.map` over an argument, an anonymous constructor, a plurality read off a
  subject. `agentRef`, `amass`, `chooseSpree`,
  `controllerSacrifices`, `dealDamageOwnPower`, `itCondSubject`, `itPrior`,
  `itsOther`, `joinedHead`, `joinedHeadWhile`, `lookAndSort`,
  `lookAndSortInto`, `lookedCards`, `lookedTop`, `loseCounters`, `modular`,
  `ownSubject`, `partyOf`, `requireBlockIt`, `rollRow`, `sacrificeIt`,
  `scaledMana`. These are routed to
  `semantics-v2-macro-capture-and-plurality`. The capture question is
  answered (§7, ruling 2026-10-06): a referent parameter is bound at the
  body's first mention and every later mention refers back to that binding;
  the RON macro language grows no capture annotation.
- **It calls one of the above (27).** A macro that does not port takes its
  callers with it. Some of these were blocked only by the 27 that have now
  ported and are available to a later port; the bucket is not re-derived here,
  because deciding it needs the converter this landing did not run: `after`, `at_`, `bushido`, `bushidoExpansion`,
  `cumulativeUpkeep`, `cycling`, `cyclingExpansion`, `fateseal`,
  `fullParty`, `fullPartyOf`, `get`, `getsBase`, `getsPt`, `levelBand`,
  `loseAllCounters`, `party`, `partySize`, `partySizeOf`, `prototypeAlt`,
  `renown`, `renownExpansion`, `scry`, `storm`, `stormExpansion`, `surveil`,
  `when`, `whenever`. Three of them no longer call one: since
  `semantics-v2-gets-both-deltas` (2026-10-05) `getsPt` writes the one node
  `StaticSpec.ptModification` (§7), and `getsBase` and `get` call `getsPt`,
  so none reaches `itsOther`. They stay counted here until a port takes them;
  RON's `gets` and `getsBoth` already write the same node.
- **It expands to no single position (4).** A macro occupies one position, and
  `List X` / `Option X` is not one: `copyCharacteristics`, `modularExpansion`,
  `partyRoles`, `stat`.
- **It is defined by pattern matching on an argument (3).** `agentPlur`,
  `itOrThem`, `sameWindow`.
- **It is a performer helper in the `Actor` namespace (4).** These spell what
  the unprefixed macros do not (§7, ruling 2026-10-05). Two have a RON
  declaration of the unprefixed name that writes the same term (`choose`,
  which records its chooser, and the keyword action `discard`, from the
  performer's own hand); the method keeps the `Actor` prefix, so they stay
  counted here while the Lean bench's unprefixed macros of those names
  differ. The other two have none: `amass` writes the literal reminder text
  rather than `itPrior`, and `army` is a subtype and no predicate is added
  (owner, 2026-10-05): `Actor.amass`, `Actor.army`, `Actor.choose`,
  `Actor.discard`. `Actor.create`, `Actor.draw`, `Actor.gainLife`,
  `Actor.loseLife` and `Actor.sacrifice` were retired at
  `semantics-v2-drop-agent-fields` (2026-10-06): with the agent fields gone
  each wrote the term of the unprefixed macro. `Actor.mayCastFrom`
  left this bucket at `plugins-v2-keyword-helper-additions`: Lean has no
  explicit-agent macro of that name, so RON's `mayCastFrom` is its helper,
  named apart in §11 as `revealHand` is.
- **Its RON helper was retired (2).** `revealTheirHand`, for a handoff: a
  card writes `act(player, revealHand)` (§7, ruling 2026-10-05).
  `returnToBattlefieldWithCounters`, for having no caller: persist and
  undying write `returnToBattlefield` with a `WithCounters` rider; the bench
  still calls the Lean macro.

A binder Lean declares `Option T` with a `:= none` default takes the param
type `Any`, not `T`: a param type validates the DEFAULT as well as the
argument, and `None` is not a `T`. Four ported declarations carried
`Default(NounPhrase, None)` and so could never be invoked without passing the
binder they defaulted; `every_ported_alias_expands` is what caught it.

Beside the ported phrasings is the ALIAS layer: one identity macro per
constructor of a `semantic_expression` type, `clearDamage(subject: …)` for
`ClearDamage`, which is what `declare_semantic_primitives` generates in Lean
and what makes §11.1's macro-only rule satisfiable — a card that may write
only macros needs a macro for every constructor. Since
`semantics-v2-drop-agent-fields` (2026-10-06) no constructor but `choose` and
`insertPart` has an agent field (§7), so the aliases take Lean's signatures:
`skipPart(part: …, count: …)` is `SkipPart(part, count)`, and `changeLife`,
`enact`, `returnTo`, `returnToBattlefield`, `meldInto` and
`exileWithCounters` take no performer. One helper keeps an agent parameter,
`insertPart`, whose agent is optional and `None` where no player takes the
added part, as in Lean's `Option NounPhrase := none`; `choose` takes none
and records the actor as its chooser. Of the aliases, 244 are declarations; the rest of
the basis is already covered by a phrasing macro of the same name (the 27
above among them, whose narrower signature is the one a card writes, and
`exists`, whose `Predicate` signature is Lean's `exists_` phrasing — the
`NounPhrase` form gets its own macro when a card first needs one), and Lean
tags `NounPhrase.pro`, `NounPhrase.gap`, `Amount.parameter` and each type's
`withBindings`/`inCaller` `internal_expansion`, so it mints no alias for them
either. `gap` is the one exception this landing made: `Storm Fleet Spy` writes
a gap outright, and a card with no way to write what it means is worse than a
deviation from the tag.
`Instruction.Exchange` has no alias, because the bodyless keyword
action `exchange` owns the name; a card writes it through one helper per
`Exchanged` arm, each the constructor with that arm (`exchangeControl`,
`exchangeLifeTotals`, `exchangeCards`, `exchangeZones`, `exchangeValues`,
`exchangeTextBoxes`; owner, 2026-10-05), so the all-or-nothing rule
[CR#701.12a] stays on the one node.
A `Condition` alias lives under `macros/conditions/`,
which is a family Lean's `Macros.lean` has no section for — `Predicate` and
`Condition` share three constructor names (`Not`, `And`, `Or`) and one file
holds one macro.

Every mirror type a ported signature names is a registered param type
(`deckmaste_semantics_v2::ron::param_types`), and every position a ported
body expands to is a registered kind — including four that are not
`semantic_expression` types and so carry no dispatch set of their own
(`CharacteristicBundle`, `HeaderPossessor`, `LevelBand`, `PrototypeFrame`),
registered by hand the way `Delta` is.

## 13. Gate and bench

The `lean-card-soundness-gate` emitter writes each expanded card as
untracked generated Lean and an xtask gate proves `Card.check` empty by
`decide`; `Macros.lean` plays no part in the gate. `cargo xtask
definition-check` proves `Definition.check` empty the same way for every
Registry Definition `plugins_v2/builtin` declares. The Idris emitter, checker
and baselines retire with Idris and v1 (§14). The hand-written `lean/Semantics/Cards/` bench
is a stand-in: as each card lands in `plugins_v2/canon` the emitted term
supersedes its hand spelling (`lean-hand-bench-retirement`), and canon cards
without Lean versions get them by emission. Pins in `Proofs/` that are
theorems about constructions rather than cards stay. The workbench is not a
second authoring surface.

## 14. Parity and cutover

v1 retires when card translation reaches parity, measured by
`semantics-v2-parity`: a per-card comparison of v2 and v1 lowerings, v1 as a
starting point and not an oracle (not everything in canon is known to lower
correctly), each disagreement adjudicated by hand and recorded, then the
whole-game slow tests on v2. At cutover `deckmaste_semantics_v2` takes the
`deckmaste_semantics` name and `plugins_v2/` becomes `plugins/`. Deletion
set, enumerated from `cargo metadata` at cutover with differences from this
list reported: `deckmaste_semantics`, `deckmaste_lowering`,
`deckmaste_legacy_render`, `deckmaste_plugin` (`deck.rs` and `provenance.rs`
rehomed; `energy.rs` dropped as vestigial, its `deckmaste_migrations` call
site going with it), `idris/`, `plugins/builtin`, `plugins/canon`, and the
splice machinery of `deckmaste_spelling` already slated by
[english-v2-rewrite](english-v2-rewrite.md). `deckmaste_migrations` sheds
`deckmaste_legacy_render` and survives.

## 15. Registries

The `plugins_v2/builtin` declarations are the sole v2 registries and already
generate `lean/Semantics/Check/Facts.lean`. v1's registries under
`plugins/builtin/macros` are frozen until deletion; anything v1 has that a v2
declaration lacks is added as a v2 declaration, never the other way.

## 16. Inert vocabulary

Ruling (user, 2026-09-06). The `Words.lean` and `Events.lean` enums no
checker function reads (`Disclosure`, `CoinFace`, `RoundMode`, `Parity`,
`ArithOp`, `SpecialAction`, and the rest, 35 at the time of the ruling) are
engine-facing vocabulary the semantics layer carries opaquely. They were
carried vocabulary in the Idris reference too. They owe no admission law and
are not twins to prune: the as-written rule keeps each printed word its own
constructor. The same holds of payload fields no law projects.
