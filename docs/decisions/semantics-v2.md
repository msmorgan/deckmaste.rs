# Semantics v2 — the encoded-English shape

> Workbench succession (2026-09-05): current workbench evidence lives in
> `lean/`; Idris paths below are historical. See
> [Lean is the workbench](lean-is-the-workbench.md).

Draft — not settled (2026-08-07). This decision fixes the target shape of
the semantics layer — the form the macro and card rewrite aims at — and the
rules for growing it. It realizes the
divergence trajectory of [semantics-spelling-lowering
§17](semantics-spelling-lowering.md): semantics drifts toward English
constructions, core toward explicit slot reference. While a draft, it moves
freely with the design discussion.
The evidence artifacts are the Idris workbench
(`idris/src/Experimental.idr`, machinery, with its evidence bench
`idris/src/Experimental/Cards.idr` — typechecking positives, pinned
`failing` negatives) and its translation guide (`idris/src/Bridge.idr` — new⇄old pairs
and the T-rule inventory; parked out of the build while chapters accumulate,
resuming with the lowering work); this document is the contract those
artifacts probe. The workbench is deliberately divorced from the current verifier and
from runnability; it becomes the real semantics validator only when v2
replaces the verifier's shape.

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
in your hand" — is unrepresentable rather than merely unused. The payload of a
binding at a joined kind remains open; it will be settled with the execution
shape described in [The kind index joins; union marking is
spelling](kind-index-joins-union-marking-is-spelling.md), including whether the
binding carries its antecedent's kind pair. Never rules classifications: timing
(announce vs. resolution), staleness, and agreement are always functions
of the stored surface facts.
The same derive-don't-store rule applies across layers — inflection derives
from the subject, a move's from-zone derives from the referent's fold-state,
a delayed clause's settled-particular reading derives from the determiner the
author wrote (staleness itself is object identity plus the fire-time zone
expectation [CR#603.7c], as §2 states), and a
hand/library/graveyard destination's owner derives from the moved card itself
([CR#400.3] — the possessive is mandatory surface with fully derivable
content: the style guide's return templating always writes it, and rendering
re-adds it).

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
(`cargo xtask map enums`) plus the builtin macro definitions
(`plugins/builtin/macros/`) are the record of the whittling. Keyword actions,
keyword abilities, and common phrasings generally are phrase-shaped macros over
the basis, mirroring the real definitions' name, params, and body. A core shape
is judged by lowerability and by its algebra; meta constructs are legitimate in
the core. Phrasing knowledge belongs to the card language or the spelling
declarations. The hand-written Idris macro layer
is a stand-in to be generated from those definitions eventually. The
`Composite` tag is itself basis: `destroy x = Composite Destroy (Move x
Graveyard)` keeps the tag that deontics key on — indestructible cants the
Destroy action and ignores an untagged move [CR#701.8a,702.12b,701.8b]. A
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
- Agents are explicit and there are no default arguments anywhere. The
  imperative's unpronounced subject is supplied by the frame as an explicit
  `You` in the term.
- Agentive verbs — those the CR gives a player actor — put that performer
  in clause position and require it, the factored form of the real macros'
  agent parameter: a dependent context cannot re-use the subject term at
  each inner slot, so the slot rides the clause and lowering redistributes
  it. Effect-verbs take that performer OPTIONALLY, since oracle text writes
  destroy and exile both with a subject and as bare imperatives; object
  sources are the verb's own argument.
- Choice method is surface data: "of their choice" and "at random" are
  marked indefinites mirroring the real macros' explicit chooser slot and
  its absence in the random variant; no CR rule derives a chooser.
- Each macro matches exactly one English phrase shape — the property frames
  compile against.
- A verb's implicit restrictions must be expressible both ways — as predicate
  conjuncts on a choice ("sacrifice a creature" chooses among `InZone
  Battlefield`, `ControlledBy` the agent) and as fold-state demands on a
  definite referent ("sacrifice it" requires the referent currently
  battlefield-and-controlled) — and the two must agree. A restriction that
  cannot take both forms is un-de-macroed verb structure hiding in a filter.
- Predicates are flat sibling modifier sets on one referent ("a creature an
  opponent controls" is two modifiers on one object predicate); zone
  membership (`InZone`) is an ordinary conjunct.

## 8. Process

The workbench grows chapter by chapter: pick one structural question, ground
it in real parses (`cargo xtask english inspect` / `bracket`, corpus lines)
and the style guide, prove the shape with typechecking positives and pinned
`failing` negatives, record findings in the module doc; Bridge pairs
catch up in batch when the lowering work resumes. Reference machinery comes
before vocabulary breadth. Completion is
two audits — every English AST construction has a settled v2 counterpart or
an explicit lowering note, and the old worked corpus transcribes cleanly —
after which this contract plus the Bridge T-rule inventory spec the
macro/card rewrite and the accompanying lowering changes.

## 9. Affected contracts and tickets

- [semantics-spelling-lowering](semantics-spelling-lowering.md): §7's indexed
  scope calculus becomes core-only normal form (see §1 above); §17's
  trajectory is realized by this contract.
- [idris-is-a-soundness-gate](idris-is-a-soundness-gate.md): unchanged today;
  the workbench sits outside the emit/check gate until v2 replaces the
  verifier's shape.
- Tickets informed: `target-sugar-elaboration`,
  `idris-distinct-position-proof` (its deep-scan obligation is subsumed
  structurally by the `Other` presupposition), `frames-catalog-merge`
  (capability unification).
