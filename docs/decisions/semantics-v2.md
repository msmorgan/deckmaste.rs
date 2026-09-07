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
Graveyard)` keeps the tag that deontics key on — indestructible cants the
Destroy action and ignores an untagged move [CR#701.8a,702.12b,701.8b]. A
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
- Agents are explicit; core constructors take every field required. The
  imperative's unpronounced subject is supplied by the frame as an explicit
  `You` in the term. A macro PARAMETER may carry a default (§12, ruling
  2026-09-07); a constructor field may not.
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

## 11. Plugin format

`plugins_v2/` is the v2 plugin format. `plugins/builtin_v2` moves to
`plugins_v2/builtin`, with a symlink at the old path until english_v2 and
xtask read the new one. One declaration file per macro carries `name`,
`params`, `spelling`, `grammar`, and a semantic `body`: english_v2 reads the
spelling and grammar, semantics_v2 reads the params and body. The file is
the shared contract; neither crate depends on the other for it, and an xtask
drift test loads every declaration both ways. Today's bodyless declarations
(keyword actions, keyword abilities, ability words, turn parts) grow bodies
per family; no new bodyless declarations are added. A registry declaration
whose columns are the ones Lean's checker reads carries THOSE columns as its
body, typed by the `facts` mirror rather than by a semantics term: a
designation declares its `DesignationFacts` rows and a marker counter its
`CounterFacts` row, and `cargo xtask facts generate` writes
`lean/Semantics/Check/Facts.lean` from them (ruling, 2026-09-07,
`facts-generator-sheds-v1`). Cards, tokens, and the
three rules tables (state-based actions, conferrals, damage results) are
further kinds in the same tree, all semantics-language RON. `plugins_v2/canon`
is hand-authored RON first; translation from `deckmaste_english_v2`'s
`OracleText` is a later crate (`semantics-v2-english-translation`), with
RON as the cached form of that translation.

The builtin macro families live at `plugins_v2/builtin/macros/<family>/`,
alongside `macros/meta/`, whose declaration meta-macros the family files
invoke. They were under `macros/stubs/` while they were bodyless; they are
declarations now, and `read_builtin_v2` reads the family directories and
skips `meta/`.

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
  onto v2. Native dispatch always wins: an identifier the position's own
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
  forms are NEVER mixed: ron's value scanner refuses `C(a, b: c)` while
  deciding, before either visitor runs, so `exile x (agent := .you)` has no
  RON spelling and a binder that must be skipped takes the wholly named form.
  A constructor of ONE field is exempt and keeps its binder: inside a newtype
  variant ron reads `(x)` as a newtype rather than a one-element tuple, and
  its `handle_any_struct` then turns a bare identifier into a unit value,
  discarding the very name the argument is.
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

`macro_ron` is unchanged: a dumb expander whose `MacroDef<Metadata>` carries
consumer-typed, opaque metadata. Kinds are one per `SupportsMacros` enum and
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
`reading_positional_arguments`) and two `#[macro_ron(...)]` markers on the
mirror (`embed`, `literal`). A `MacroSet` with neither switch, over types
carrying neither marker, reads exactly as it did before. What a macro's own
BODY may write is what a card may write, because a body is read at the
position it expands to and goes through the same reader. A macro's own
argument list is unaffected: a signature is positional or named as it always
was, and `read_args` checks argument names against the signature rather than
against a constructor's binders.

## 13. Gate and bench

The `lean-card-soundness-gate` emitter writes each expanded card as
untracked generated Lean and an xtask gate proves `Card.check` empty by
`decide`; `Macros.lean` plays no part in the gate. The Idris emitter, checker
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
