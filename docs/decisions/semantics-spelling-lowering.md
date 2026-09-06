# Semantics, spelling, lowering — the card-grammar split

> Workbench succession (2026-09-05): current workbench evidence lives in
> `lean/`; Idris paths below are historical. See
> [Lean is the workbench](lean-is-the-workbench.md).

Settled 2026-08-02, after the action-role-reshape landed. This decision is
the design contract for the semantics-grammar program; the ticket map in §15
implements it. Like every decision doc: it records intended design, current
code wins on incidental drift, and changing it requires explicit review.
Clarified 2026-08-04 to distinguish Oracle text's upstream authority from the
semantic form's downstream canonicality; no type boundary changed.
Renamed 2026-08-05 from “authoring” to “semantics”: most terms are recovered
from Oracle rather than written by hand, while all terms share their role as
the canonical meaning representation. This is a vocabulary and crate rename;
the architecture and type boundary are unchanged.
Amended 2026-08-06: §17 records the divergence trajectory — semantics
drifts toward English constructions, core toward explicit slot reference
(`core-reference-slots`; superseded 2026-09-02 by [Core is explicit regions](core-explicit-regions.md)) — and the certifier/resolver role split; no type
boundary changes today.
Amended 2026-08-07 as `idris-mirror-semantics` landed: §10 records the
reattachment as executed — the mirror is `idris/src/Semantics.idr` (renamed
from `Core.idr` per principle 1), the emitter walks semantic terms, and the
riders-battlefield obligation is resolved. No design change; the section now
describes the code.

## 1. Summary

One semantics grammar, two derived projections, three new crates:

```
                deckmaste_spelling            deckmaste_lowering
deckmaste_english ◄════════════════► deckmaste_semantics ────────► deckmaste_core
  English AST       two-way ranked      the semantics       one-way    pure engine
  text ⇄ AST only   relation            rules grammar       total      AST; plain
                    (frames ENGINE      + macro layer       compile    serde; NOT
                    lives here;         + Idris mirror                 Idris-mirrored
                    frames: DATA
                    rides semantics
                    defs)
```

- **`deckmaste_semantics`** is the canonical semantic form: the normalized,
  information-complete rules grammar persisted in every card-content container
  — card files, token files, and the `rules/` engine tables. For existing Magic
  cards, Oracle text is the authoritative source input and recovery compiles it
  into this form; downstream rendering and lowering then treat the semantic term
  as their single source of truth. The crate owns the types, the kind registry
  and `SupportsMacros` machinery, identity-macro registration, the collision
  diagnostic, the scope-calculus and sugar forms, and normalization (desugar +
  scope elaboration, semantics → semantic normal form).
- **`deckmaste_spelling`** is the semantics ⇄ English relation:
  `deckmaste_frames` renamed and absorbed — the ENGINE (compile/unify/
  render). "Frames" survives as the name of the lexicon's entries, and the
  frame SCHEMA (`FrameSpec` and kin) deliberately stays in neutral
  `macro_ron`: defs carry `frames:` and `macro_ron` parses defs, so moving
  the schema into spelling would cycle schema and consumer.
- **`deckmaste_lowering`** is the one-way compile semantics → core: it
  invokes semantics' normalization, then owns the cross-grammar type
  mapping, semantic-invariant enforcement, and the divergence ledger.
- **`deckmaste_core`** becomes a pure engine AST: no macro machinery, plain
  serde, no Idris obligations (until an engine-side dependent invariant
  earns a thin mirror back).
- **`deckmaste_card`** (engine-side, sitting directly above core): the
  engine's unit of card definitions — `Card`/`CardFace` and the face
  layouts, the packaging of primitives into a playable unit — split out of
  core so the loose primitives are testable independently and the
  card-shapes program (transform/saga/adventure/split) has a home. A face's
  type-line FIELDS move with it; the `Type`/`Subtype`/`Supertype`/`TypeDef`
  vocabulary those fields hold stays in core. Card depends on core, never
  the reverse; lowering targets both. Core carries
  NO characteristics abstraction: the grammar keeps only the
  characteristic value vocabulary plus the effect-defined bundles
  (`Token`/`TokenSpec` and the face-down bundle remain core — the grammar
  creates those objects). The base/computed seam is ENGINE-owned:
  `trait BaseCharacteristics` lives in the engine, implemented there for
  `card`'s types and core's bundles (local trait, foreign types — the
  orphan rule permits it), with `CardRef<T: BaseCharacteristics>`
  engine-local at the object/state boundary; grammar enums stay
  monomorphic. Neither the trait nor `CardRef` is built yet:
  `card-crate-split` landed as a pure move and enforced only the negative
  half of the contract — that core gains no characteristics abstraction.
  The positive half is owned by `engine-base-characteristics`, which also
  records that only two of the three named base sources are live today
  (`FaceDownCharacteristics` has no reader until `engine-face-down`).
  A future shared characteristics-atoms
  crate below both grammars (killing atom mirroring) is booked as a
  design-gated follow-up (`characteristics-atoms-crate`), deliberately
  NOT part of Stage 1.
- **The card-loader crate is renamed to `deckmaste_plugin`** (Stage 0): it
  was always the loader plus riders. The rename landed alone; splitting the
  riders (the Idris emitter and validation) out is booked as
  `plugin-rider-split`. The dying legacy renderer and the fidelity harness
  that rides it move together into `deckmaste_legacy_render`, in
  `runtime-prose-link` (owner-settled 2026-08-02, superseding "stays put to
  die in place"): that ticket must retype every one of the renderer's
  `Expanded` match sites onto semantic terms regardless, so moving and
  retyping in one traversal is cheaper than two. Once those sites match
  semantic `Expanded`, which no ticket deletes, the renderer stops blocking
  `core-demacro`; `plugin-rider-split` is left splitting only the Idris
  emitter and validation, as it says.

Design principles that did the deciding, recorded because they generalize:

1. **Name things what they are.** No minted proper nouns; every crate and
   concept carries its literal role. ("Vocabulary" is reserved for the
   English side.)
2. **The canonical form may contain only what round-trips.** Anything one
   translation direction must invent and the other must discard is
   non-derivable decoration and cannot be stored. (This killed named binder
   variables, labels, pin tables — external variant→author-name mapping
   files — and persisting the author's sugar-vs-full choice as
   provenance.)
3. **Sugar never replaces structure.** The explicit form is always the
   meaning and always legal; every alignment feature desugars into it at
   load — so each alignment step is removable, and semantics are preserved
   under reduction to the explicit form. (The author's surface CHOICE is
   intentionally unrecoverable; what is preserved is meaning, not
   spelling.)
4. **Productive compositions are machinery, never vocabulary.** A pattern
   that composes with an open class (targeting × verbs, `May` × effects)
   gets a mechanism; only closed idioms (`Mill`, `DestroyNoRegen`) get
   words.
5. **Proof follows purpose.** The Idris gate's obligations are
   author-mistake proofs, so the mirror attaches to the semantics grammar,
   not to whichever type family historically hosted it.
6. **Formal budget goes where the entropy is.** The open-ended human/
   generated corpus gets the proof gate; the closed mechanical mapping gets
   generated arms and tests.

## 2. Evidence base

Settled through recon over the post-reshape tree plus independent external
design consultations and a three-reviewer cold-review panel, with every
load-bearing external claim re-verified against the repo or the card data
before adoption. Facts that shaped scope (counts from a name-based census:
capitalized idents in container files cross-checked against the defined
macro-name set and `cargo xtask map enums`; approximate by design and
re-derivable the same way):

- Canon cards spell 182 distinct bare variants across ~842 file-hits — the
  parse ban is a whole-corpus property, not a cleanup.
- The `rules/` tables are the densest bare-grammar spellers (23 distinct
  variants across 6 files) — the semantics grammar is container-neutral, not
  card-shaped.
- The frames constructor catalog has exactly 7 entries — retirement is
  cheap now and only now.
- Canon already spells the explicit indexed targeting form (Lightning
  Bolt), so keeping it canonical costs zero migration.
- Card-level ground truth was verified via the mtg-rules toolkit for every
  oracle text relied on: Lightning Bolt, Terminate, Rabid Bite, Seeds of
  Strength, Arc Trail, Arc Lightning, Pyrotechnics, Fling, Do or Die,
  Otherworldly Journey, Ephemerate, Cloudshift.

## 3. Architecture: one canonical form, two projections

**Oracle text is authoritative input; semantic form is canonical semantics.**
For the existing Magic corpus, recovery compiles Oracle text into one normalized,
information-complete form. Persisting that form as RON caches a compilation worth
performing once across roughly thirty thousand cards; it does not make RON a
competing upstream authority. Once compiled, the semantic term is the only input
to the two downstream projections: English rendering and core lowering. Core is
a compiled artifact — cached at most, keyed by content digest, never treated as
source.

**Information-complete does not mean exhaustive.** The canonical form is
deliberately minimal and human-legible: it preserves every distinction required
to render or lower the card, but no parser trace, redundant machine expansion,
or non-semantic authoring choice. A reader familiar with the card should be able
to diagnose a mistranslation from its compact RON without understanding the
compiler's internals. This debuggability is part of the representation contract,
not incidental prettiness.

The mechanism is the built-in macro library together with its construction
frames. Macros give recurring semantic constructions compact, rules-shaped
names; frames define how those names recover from and spell as English. Compiled
together as a lexicon, those frames are also the ingestion program: unification
against the parsed English tree chooses a macro, recovers its typed arguments,
and recursively assembles the canonical RON term. Frame coverage is therefore
card-ingestion coverage. The explicit expansion remains the meaning, so this
compression loses no structure and can always be reduced before lowering or
proof.

The authoring interface for future custom-content plugins remains open: prose,
direct RON, or both can be accepted, provided every path produces this same
normalized form before rendering or lowering. One narrow, declared exception:
engine STRATEGY configuration is authored directly in core terms via plain serde
— it is engine configuration, not card content, and sits outside the card-semantics
pipeline (see the §4 container table). The spelling relation is **ranked, not
bijective**: several authored spellings may word identically and one sentence may
recover to several candidates; ambiguity surfaces as candidates, never resolved
by assembly order. "Faithful" means semantic round-trip plus canonical wording;
byte-exact replay is out of scope (an optional non-semantic surface witness can
be added later if it is ever genuinely needed).

Recovery (English → semantics) uses transient, in-memory handles for
binder/discourse linking inside its derivation; the derivation is erased to
a semantic term and no handle is ever serialized (principle 2).

## 4. The semantics grammar and the parse-position ban

**One grammar, many containers, restriction per container.** The
normative container table (restriction mode inherits through NESTED
definitions by textual provenance — e.g. a token bundle defined inline in
a card file is card-authored text and restricted):

| Container | Loader path | Mode |
|---|---|---|
| `cards/` (canon, builtin, AND generated wizards) | restricted read API | restricted |
| `tokens/` | restricted read API | restricted |
| migration graduation candidates (`.ron.todo`) | restricted read API | restricted |
| `rules/{sba,grant,damage}` engine tables | rules loaders | free |
| macro definition files (signatures, bodies, defaults, guards) | macros loader | free |
| frames data (`frames:` on defs) | spelling compile | free |
| engine strategy RON | plain core serde | outside the program (engine config) |
| test fixtures | per fixture | fixture's choice |

- **Mechanism: native-variant-candidacy suppression** at the ident dispatch
  point (`EnumIntercept::visit_enum`'s native-first branch) — not
  match-then-reject. Suppression is what routes mirrored spellings to
  identity macros, keeps bare-nullary sugar (`You`, `This`, `It`) parsing,
  and produces the right error ("not author vocabulary") only when no macro
  exists. No value tree is ever built for a banned spelling. NOTE: there is
  a second native-candidacy consult in the untagged-embed pre-scan (its own
  `variants.contains` before `EnumIntercept` runs); suppression must cover
  both consults or a banned ident at an embed-hosting kind takes a
  confusing route.
- **Suppression applies at REGISTERED kinds only.** Restricted card syntax
  also reaches plain enums that are deliberately outside the macro system —
  closed operator/timing/format atoms such as `Cmp` (canon spells `Eq`),
  the phase/step enums (canon spells `Beginning(Upkeep)`), `FaceLayout` —
  and those parse natively by design. The Stage-2 deliverable includes a
  **closed reachability inventory**: every reachable `(kind, variant)`
  pair classified as identity-covered, native-calculus (the §4 carve-out
  below), or native-atom whitelist. Nothing may be unclassified.
- **The restriction bit follows textual provenance, not the expansion
  frame.** It is recorded when argument text is captured and restored when
  that text is re-read; body re-reads are exempt however reached. (The
  `Ctx::frameless()` param-re-read path is exactly why frame-gating alone
  is wrong.) The bit therefore rides on the ARGUMENT, not on the frame: a
  body that forwards a card-written argument into a nested macro passes the
  bit on with it, so provenance survives any number of hops. Two things the
  rule turns on that are easy to get backwards:
  - **Granularity is per argument, not per byte.** An argument a body
    assembles from its own text plus a forwarded hole (`Up(Param(0))`) is
    restricted as a whole — text-level splicing loses the seam, and the
    author's half has to decide, since the alternative is laundering it.
    Definition text over-restricted this way fails loudly at the
    identity-macro gate; it can never pass silently.
  - **Reader-synthesized text is not author text.** The bare-numeral literal
    splice (§4 sugar: `3` for `Literal(3)`) wraps the author's numeral in a
    constructor the reader invented, and that wrapper reads FREE. Restricting
    it would route every bare numeral through an identity macro and change
    what a remembering kind stores.
- **Entry points**: ONE restricted read API (a `read_semantic_card`/
  `read_semantic_token` pair or equivalent) shared by `Plugin::card`/
  `token`, the migrations graduation path, AND every typed production
  reader that currently bypasses the loader — validation, fidelity, the
  canon comparison path, and the Idris bulk emitter all `read_str` typed
  card values directly today. A grep gate forbids remaining direct
  `read_str::<Card|Token>` calls. The other ~230 `read_str` callers are
  untouched — restriction is opt-in at the entry, never a global default.
- **Structural binding forms carve-out**: `Targeted`, `Target(n)`,
  `Targets(n)`, `Distinct` are designated structural grammar — the
  semantics layer is "a typed macro-invocation tree plus a small built-in
  indexed scope calculus," and pretending the calculus is macros would only
  hide its binding behavior.
- **Straggler registration**: `Color` (currently unregistered as a kind — a
  macro of kind Color is undefinable today), `Supertype` (no
  `SupportsMacros`), and the `Card` root gain registration and trivial
  defs. The closed `Type` enum needs nothing (it appears only in machinery
  positions post type-flip — the landed migration of card type lines from
  the closed `Type` enum to `TypeDef` macros); counters already conform
  (name vocabulary behind their hand deserializer).
- Negative fixtures prove a bare-primitive card fails with a useful error.

## 5. Identity macros: scaffolded once, then hand-owned

Every semantics-reachable variant OUTSIDE the native whitelists gets a
wrapping macro def under its variant's own name (mirror-by-default — this
is what makes the ban near-zero-churn: invocation syntax is identical to
variant syntax, so canon re-parses byte-identically). **Reachable** means:
variants of kinds in the registry reachable from the restricted-container
root types via field types, flatten, and embed — excluding the
name-erasing loader-tag kinds (`Macro`, `KeywordAction`, and kin).
**Coverage is keyed per `(defining type, variant, dispatch kind,
signature)` row** — a flattened variant registers its wrapper at every
dispatch host; same-name different-kind entries (the two `Target`
constructors) are distinct rows. Coverage = every reachable row is
identity-covered, native-calculus, or native-atom (§4) — the whitelists
are named, so "every" is checkable.

Wrapper scaffolds **mirror arities AND any existing constructor
defaults** — byte-identical re-parse of canon is the invariant, so default
elision keeps working; removing the defaults themselves remains
`core-remove-default-args`'s separately-priced scope, not a side effect of
this program.

Defs are **generated once as scaffolds, then hand-owned**: they are the
home for hand-written `frames:` (§8), which wipe-first regeneration would
destroy, and hand ownership is what makes a core-side rename a one-line
body edit with canon untouched. (Frame COVERAGE is the english effort's
ratchet, not this program's gate — the coverage gate here checks
def-presence and signature only.) A **coverage gate** replaces
regeneration; missing defs are scaffolded on demand.

Wrapper interaction at remembering kinds: once identity macros exist,
previously-bare spellings at `remembers_expansion` kinds parse wrapped in
invocation provenance (`Expanded(…)`). This is invisible at the two levels
that matter — stored-byte round-trip (write-back restores the invocation)
and lowered core (`lower` erases wrappers) — and "zero behavior change" in
this program's gates is measured at exactly those two levels.

## 6. The collision diagnostic

Registering a macro whose name equals a variant of any kind it registers
under is an **error**, checked against the complete dispatch set
(`SupportsMacros::ALL_VARIANTS` — flattened compartments included), at both
ordinary insertion and plugin-layer replacement. The identity-wrapper
exemption's trust channel is a **compiled registry**: the scaffold
generator emits a Rust-side table of `(kind, variant, signature)` rows in
the semantics crate, and the loader confers the exemption when a def
matches a row — the marker is never serialized in RON (a serialized marker
would be forgeable; a serde-skipped one would be lost on reread). Per-kind,
because same-name different-kind reuse is intentional practice (`Draw` the
verb vs `Draw` the event filter). Current corpus has zero collisions —
clean slate. This retires `action.rs`'s comment-enforced naming rule. A
mandatory `core::` body namespace is consciously NOT shipped; it remains a
possible future escape hatch if a non-identity collision is ever genuinely
wanted.

## 7. Targeting and the scope calculus

**Core invariant (the design's fixed star): announced targets are never
anaphors.** They are read only through the indexed channel; `It`'s
antecedent-stack resolution (R1 = nearest singular antecedent, R2 = refuse
when a second compatible antecedent makes it a guess) excludes them;
post-move re-mentions are the separate `That(Sort)` product channel.
Current code deviates in two known, ticket-owned places: the engine
carries a deliberate lone-target `It` compatibility arm
(`core-regions-substrate`, which deletes it), and the doc comment on `Targeted`
itself still describes the pre-invariant anaphor reading
(`post-reshape-comment-rot`). Neither weakens the invariant as the design
target.

- **The explicit indexed binder is the canonical semantic normal form** —
  which canon already spells. Sugar is accepted input; whether a canonical
  writer later contracts eligible slots is a formatter decision; the
  author's choice is never persisted (principle 2).
- **Inline sugar**: `Target(spec)` at an exactly-one Reference position;
  `Targets(spec)` at a Selection position; never type-directed overloading
  (Idris treats only literal exactly-one as cardinality One).
  **The sugar family is named `Announce` (owner-settled 2026-08-02)**:
  `Announce(spec)` introduces an announced slot — it names the rules
  moment [CR#601.2c], kills the `Target(Target(…))` nesting the
  overloaded spelling would have had, and makes introduction-vs-read
  lexical rather than payload-shape-based (no numeral discriminator
  needed). The same-concept reuse with `LockPoint::Announce` is
  deliberate harmony. Two family members are direction-settled with
  final shape left to the Stage-4 plan: a READ companion for referring
  to a previously-announced slot (an `Announced(n)`-style spelling —
  note the rename depth question: canon already spells `Target(n)` at
  ~20 sites, so either both grammars rename with a small canon sweep, or
  `Target(n)` stays canonical and the companion is rejected — dual
  spellings are not an option under principle 2); and **distinctness
  encoding**: `Announce(Distinct([i…], spec))` already composes for
  edges naming explicit-prefix slots, and an "other"-flavored form
  (distinct-from-all-earlier-announcements, matching oracle's "any
  OTHER target" — style guide §6, "Describing objects, players, and
  targets") covers the fully-inline case that indexless sugar
  slots cannot express — the explicit indexed `Distinct` remains the
  general mechanism. Sugar is legal only in card-authored provenance
  text — a macro body may FORWARD sugar through a `Param` hole but may
  never introduce it (bodies are scope-free).
- **Occurrence counting is defined over the PRE-EXPANSION invocation
  AST**: each caller-side semantic site is one occurrence, regardless of
  how many times an idiom's body re-reads the parameter (`Fight` re-reads
  each of its two arguments several times and is the normative fixture —
  its caller writes each `Target(spec)` once, so both slots are
  sugar-eligible). This is an implementation commitment, not just a
  definition: the current reader expands during deserialization and
  re-reads raw argument text at every `Param`, so sugar recognition needs
  either a pre-expansion elaboration layer or origin-tagged sugar nodes
  propagated through expansion — priced as such in
  `target-sugar-elaboration`. Every semantic `Target(n)`/`Targets(n)`
  site counts (including characteristic reads — `PowerOf(Target(0))` is a
  site); English `Mention` occurrences and expanded-core reads do not. A
  sugared slot is unreferenceable by construction, so the one-site rule is
  enforced by syntax; needing a second site means writing the explicit
  form. Worked verdict: Rabid Bite's slot 0 has two semantic sites (agent
  position and the power read), so it must be explicit; its slot 1 has
  one site and may append inline under the mixing rule below.
- **Direction labels on the two scope rules** (they are not in tension —
  they govern different directions): *acceptance-time validation* — a
  sugared introduction is accepted iff it is that slot's only semantic
  site, and a `Distinct` edge cannot attach to a sugared slot at all
  (semantic indices must be `< E`); *canonical-writer contraction* (the
  future formatter direction) — a slot contracts to inline form only if it
  has one site and no `Distinct` edge, and a cross-constrained connected
  component stays wholly explicit.
- **Mixed scopes are legal via the explicit-prefix rule**: explicit slots
  permanently own `0..E`; inline sites append in the elaborator's
  traversal order, which is DEFINED as schema order — fields in
  declaration order, list elements in sequence — so named-field reordering
  in RON text does not change indices; semantic indices must be `< E`; the
  validator rejects a semantic index landing on a generated slot.
- **The elaborator is the only scope introducer.** Idiom bodies are
  scope-free with spec-typed parameters. Elaboration normalizes to the
  engine's single top-level `Targeted` per spell/ability and stops
  hoisting at genuine announcement boundaries — modal modes and delayed
  triggers open their own announcement scopes; per-mode target scopes are
  never flattened into an unconditional outer list
  [CR#601.2c,603.3d,603.7].
- **Cross-slot distinctness**: two unconstrained slots may legally choose
  the same object (once per instance [CR#601.2c]); within-slot uniqueness
  is rules-default and needs no encoding. The ULTIMATE core encoding of
  cross-slot constraints is predicate-embedded slot references — slot i's
  criteria may reference strictly-earlier slots
  (`Target(And([Not(Ref(Target(0))), Creature]))`) — making
  `TargetSpec::Distinct` derived and deletable (the semantic other-form
  lowers to the predicate shape; recheck-correctness comes free because
  the rules recheck targets against the criteria themselves). This is a
  STATIC, order-free constraint — distinct from the rejected dynamic
  not-already-targeted filter. Well-formedness: no `Target(j >= i)`
  anywhere in slot i's predicate tree (the generalized
  `idris-distinct-position-proof` obligation); plural edges use a
  group-membership predicate.
- **Re-mentions**: same-slot re-mentions are `Target(n)` in the full form
  or owned inside idiom defs; cross-zone re-mentions are `That(Sort)`.
  Surface pronouns do not map one-to-one onto channels — Ephemerate says
  "return it" and Cloudshift "return that card" over the same product
  structure; pronoun choice is a spelling-side realization decision and
  parse-side disambiguation needs the zone-aware discourse environment
  (surface conventions: [style guide](../oracle-style-guide.md) §5,
  "Pronouns" / "This, that, those, and the chosen").
  Canonical test pair: Ephemerate / Cloudshift; same-slot case: Terminate;
  slot-read case: Rabid Bite; repeated-clause case: Seeds of Strength
  (each syntactic introduction is a fresh slot); relative case: Arc Trail
  (`Distinct`); nested-predicate case: Do or Die (an inline introduction
  inside a filter is legal; the desugar traversal collects it — under the
  explicit form nothing needs discovering at all).

Worked examples (normative):

```ron
// Sugar (accepted input)             // Normal form (canonical)
DealDamage(This, 3, Target(AnyTarget))
                                      Targeted(
                                        targets: [AnyTarget],
                                        effect: DealDamage(This, 3, Target(0)))

// Mixed: explicit prefix + appended inline site (indices 0..E fixed first)
Targeted(
  targets: [AnyTarget],               Targeted(
  effect: Sequentially([                targets: [AnyTarget, TargetOne(Creature)],
    DealDamage(This, 2, Target(0)),     effect: Sequentially([
    Destroy(Target(TargetOne(Creature)))  DealDamage(This, 2, Target(0)),
  ]))                                     Destroy(Target(1))]))

// Idiom forwarding (Fight: one caller site per slot; body re-reads don't count)
Fight(Target(TargetOne(Creature)), Target(TargetOne(Creature)))
                                      Targeted(
                                        targets: [TargetOne(Creature), TargetOne(Creature)],
                                        effect: <Fight expansion over Target(0), Target(1)>)
```

## 8. Spelling (the semantics ⇄ English relation)

- **The constructors catalog retires.** Its 7 entries migrate into their
  words' `frames:`; the lexicon becomes single-origin; macro entries absorb
  the constructor-only capabilities (body patterns, `announcement:`).
  Compound entries for productive compositions are the frames engine's
  job, not vocabulary (principle 4): the bodied `GainLife` entry's frames
  land on the life word's def, and `TargetedDealDamage` survives only as a
  non-author-facing spelling-side test fixture (a differential baseline)
  until compositional targeting covers it, when `target-sugar-elaboration`
  deletes it.
- **`template:` fields sunset with the legacy renderer** (their projection
  from `frames[0]` is checkable via the opt-in
  `cargo xtask macro templates --check` — not currently wired into CI).
- **Requirements handed to the english effort** (interface, not
  implementation): `Argument` vs `Mention(Full | Pronoun | Demonstrative)`
  occurrence classes on frame specs and compiled frames (full-mention vs
  pronoun vs demonstrative conventions: style guide §5) — the `${0:pro}`
  predecessor generalized (one definition parameter, several licensed
  surface occurrences; frames stay linear except via declared occurrence
  roles); whole-scope recovery with transient handles; a discourse
  environment with R1/R2-style uniqueness that surfaces ambiguity; frames
  rooted at multi-sentence text for `May`-class words; a role key
  generalizing `FramePosition` (cost / effect / trigger consequent /
  condition / keyword argument); a semicolon junction in the English AST;
  eventually typed English-AST hole substitution replacing
  splice-and-reparse.
- **English-effort gating**: from the catalog merge onward, the english
  effort's frame-minting rounds write frames into defs, never the catalog;
  targeting-adjacent frames wait for the occurrence-class design.

## 9. Lowering (the semantics → core compile)

Phases of the one pass: normalization (semantics-side: desugar + scope
elaboration + single-`Targeted` normalization, §7 rules) → type mapping
(semantics types → core/card types) → semantic-invariant enforcement at the
point where violation would produce wrong core. Normalization is an
semantics-crate transformation — sugar is semantics' own feature — so the
Idris emitter reaches the normal form without depending on lowering;
lowering invokes it and owns the cross-grammar mapping.

- **The mapping starts as an exact mirror** (identity-shaped arms,
  generated), because the fork duplicates the freshly-reshaped grammar.
- **Divergence discipline**: shapes stay mirrored unless a divergence earns
  its mapping complexity; every non-identity arm carries its justification
  in place — the crate IS the divergence ledger. Semantic terms have no
  independent semantics: a term means its image under `lower`.
- **Correctness story** (because the compiler forces totality, not
  correctness): generated identity arms while mirrored; **one mapping test
  per variant, each naming the engine shape it expects**; and downstream
  gates (engine suites, fidelity) as the backstop.

  **The raise map is WITHDRAWN (owner-settled 2026-08-02, during
  `lowering-crate`).** The original story here was a debug-only raise map
  enabling a round-trip property (`raise(lower(t)) ≡α t`) over the
  mirrored subset. It is not worth building: its coverage is defined as
  the set of terms all of whose nodes map via identity arms, so it shrinks
  monotonically as arms diverge — and `runtime-prose-link` diverges the
  `Expansion` arms almost immediately (§12), which is the first bite out
  of it. A second mapping the size of the first, whose value decays from the day it
  lands loses to per-variant tests that keep working after divergence,
  which is exactly when a mapping test earns its keep.

  What replaces it: **one test per variant, stating the expected engine
  shape** — `assert_matches!(semantic.lower(), core::T::V(<nested…>))`.
  The pattern is written to the depth stable Rust can reach: it spells
  nested variants and `None`/numeric leaves exactly as the test's value
  builds them, and bottoms out at `_` only where no pattern can go
  (behind `Arc`/`Box`, and at `String`/`Arc<str>`/`Ident`, which have no
  matchable literal form). Values come from a well-foundedness fixpoint
  over the grammar, so no fixtures are hand-written.

  This is what catches a wrong-but-well-typed arm — §13.2's named blind
  spot. Note the spot is narrower than it looks: swapping two variants
  with differently-typed payloads is a COMPILE error, so the tests are
  covering the residue the type system leaves, chiefly variants sharing a
  payload type. A **serialization comparison**
  (`core::to_string(lower(x))` vs `semantics::to_string(x)`) was carried
  alongside for a while and measured redundant once the patterns were
  written to full depth — identical detection on a mutation sweep — so it
  survives for exactly one type: `ManaCost` wraps a private field, and
  stable Rust cannot match a tuple struct with private fields across a
  crate boundary.

  Known limit, honestly stated: two same-typed fields whose minimal values
  are equal (four `Option<StatValue>` on `CardFace`, all `None`) cannot be
  told apart by any assertion over minimal values. Closing that needs
  distinguishable values per field, not a different assertion.

  If a future contraction pass reorders slots, per-variant expectations
  are updated with that pass; the "equal up to consistent slot renumbering
  with `Distinct` edges re-anchored" equivalence lands there if it is ever
  needed.
- The error taxonomy (semantics parse errors / lowering errors / core
  validation) gets its one deliberate pass here, where the layers meet.

## 10. Idris: the mirror attaches to semantics

The mirror's existing obligations are author-mistake proofs — unbound-
anaphor soundness (the R1/R2 gates), target-read range and cardinality,
and `Distinct` range checking (position-strengthening is
`idris-distinct-position-proof`) — so the mirror follows its purpose
(principle 5). One long-claimed obligation was NOT real: the
riders-battlefield-only rule existed solely as prose on `EnterRider` in
`action.rs` ("rejected by the Idris re-emit gate" — it was not), and the
emitter passed rider shapes through or gapped on them without destination
checks. **Resolved in `idris-mirror-semantics` (2026-08-07), both ways at
once**: the mirror gained `EnteringOk`, an auto-implicit on `Move` that
makes `enteringAttacking` unrepresentable at any non-battlefield
destination (`Spec.idr` carries the paired positive and negative), and the
prose was corrected to say what that leaves unenforced — every other
`EnterRider` variant has no Idris carrier at all, so the emitter gaps on it
and nothing checks its destination. Mirroring the rest is
`idris-mirror-enum-gaps`.

- The mirror models the **semantics kernel**: the post-expansion,
  post-desugar normal-form value universe of the families the emitter
  emits — the container types (`Card`/`Token`) and the grammar reachable
  from them, exactly as emitted today. Excluded: strategy terms, the
  `MacroDef` machinery, frames data. The reattachment is
  **content-preserving, not shape-identical**: the emitted kernel and the
  idris-check pass set are unchanged (the emitter walks semantics mirrors
  of the same shapes it walked before), while the PRE-EXISTING Rust↔Idris
  drift — missing variants, arity differences the emitter bridges —
  remains exactly what `idris-mirror-enum-gaps` records: untouched,
  neither fixed nor worsened by the move.
- Sugar-level syntactic rules (occurrence criterion, prefix rule) are
  desugar-time Rust checks; Idris proves semantic invariants on the normal
  form. English-ward divergence brings its proof obligations with it.
- **`deckmaste_core` carries no proofs**, deliberately: its divergences are
  engine-driven and Rust-testable. If a genuinely dependent engine-side
  invariant appears, a thin core mirror can return for that obligation.
- Known gate-coverage debt is unchanged by the move and more naturally
  closed after it (the emitter walks stored artifacts directly): the canon
  emitter gap and ungated wizards corpus tracked by
  `core-regions-substrate`'s validator — and NOTE: batch `idris-check`
  currently reports failures without a failing exit status, so "no
  regressions" gates need the checked-in pass/gap baseline that
  `ci-idris-gate` owns.

## 11. Staging

Stage numbers are thematic groupings; the wave map below (equivalently,
the ticket `needs:` graph) is the schedule — a later-stage ticket may
legally land before an earlier-stage one when its needs are met. Every
landing keeps all gates green (zero behavior change until Stage 2 begins
the visible policy).

- **Stage 0 — free the names**: the card-loader crate → `deckmaste_plugin`
  (rename only; the rider split is `plugin-rider-split`); `deckmaste_card`
  split out of core — a cross-cutting, land-anytime migration
  (deliberately `needs: []`; whichever of it and the fork lands second
  adapts). No semantic change.
- **Stage 1 — the fork**: `deckmaste_semantics` created as a mirror of
  core's grammar WITH the macro machinery; `deckmaste_lowering` with the
  generated identity mapping; loaders and migrations repoint (parse
  semantics, lower to core); Idris mirror reattaches; THEN core is stripped
  of macro machinery, with the test-fixture sweep (the priced big-boring
  item: every `#[cfg(test)]` macro-aware spelling of core values repoints
  or re-spells).
- **Stage 2 — the author surface**: the ban, identity-macro scaffold +
  coverage gate + the closed reachability inventory, straggler
  registration, negative fixtures; the collision diagnostic lands after
  the surface settles (early-landing option withdrawn 2026-08-02 — the
  machinery it checks was moving underneath it; `needs:
  [macro-author-surface]`, exemption live against the compiled registry).
- **Stage 3 — spelling consolidation**: `deckmaste_frames` →
  `deckmaste_spelling`; catalog merge; capability unification.
- **Stage 4 — sugar and scope**: inline target sugar + elaboration rules.
- **Throughout**: laws first — expansion equality and the per-variant
  mapping tests are owned by `lowering-crate`;
  canonical-vs-exact-rendering semantics by the spelling side — and the
  legacy renderer + regex migrations run as shadow oracles with ratcheted
  coverage until each feature family crosses its gates (ratchets and kill
  criteria are owned by each family's landing ticket; the comparison
  harness rides fidelity in `deckmaste_legacy_render`) — measurements, not
  authorities — then die.

Parallelism contract (the ticket `needs:` graph encodes this):

```
wave 1:  plugin-crate-split ∥ card-crate-split
         ∥ the planned soundness tickets (engine-it-target-fallback-removal,
           idris-distinct-position-proof, post-reshape-comment-rot)
spine    authoring-crate-fork → lowering-crate → plugin-repoint
(waves     (side lane, off the pinch path: spelling-crate-rename —
 2–4):      needs only the fork)
pinch:   plugin-repoint  (the one true serialization point; freeze window)
wave 5:  macro-author-surface ∥ idris-mirror-semantics ∥ runtime-prose-link
wave 6:  core-demacro ∥ target-sugar-elaboration ∥ frames-catalog-merge
         ∥ macro-collision-diagnostic (resequenced 2026-08-02)
```

The two Stage-0 renames conflict textually (workspace-wide import sweeps),
not semantically — integrate them back-to-back or fold both into one
workspace. Steady state after Stage 1, contention is sharded by crate:
english rounds in spelling + semantics data, def/frames work in semantics,
engine correctness in core/card, with the dependency DAG making
cross-interference a build error.

## 12. Migration inventory and blast radius

- Canon churn ≈ 0 at every stage (mirrored names AND mirrored defaults;
  canon already spells the explicit targeting form).
- The Stage-1 fixture sweep is the largest single line-item (engine
  resolve/trigger test modules, core mana/filter tests, the plugin-crate
  fixture helpers, integration suites).
- `Expanded` / `remembers_expansion` invocation provenance relocates from
  core values to semantic values (a real sub-project inside Stage 1, not a
  rename) — the loader grows a dual-result contract (semantic term AND
  lowered core value; provenance erased exactly at `lower`), and the ~97
  production `Expanded(…)` match sites across ~14 `deckmaste_engine`
  modules stop existing once core values carry no wrappers. The relocation
  is sequenced in two landings so that no landing degrades (owner-settled
  2026-08-02). `plugin-repoint` builds the dual-result contract with the
  `Expansion` arms still identities: its lowered core is byte-identical to
  today's — gated by `lower(semantic_read(src)) == core_read(src)` over the
  corpus — and every consumer of invocation provenance keeps working
  untouched. `runtime-prose-link` then erases the wrappers in the same
  landing that moves the legacy renderer and fidelity into
  `deckmaste_legacy_render` and repoints them onto semantic terms (a further
  ~30+ `Expanded` match sites, outside the ~97-site engine count) and builds
  the provenance index — so prose that feeds on invocation templates never
  loses them, and the ~97 engine arms are deleted alongside the erasure that
  makes them dead. `core-demacro` deletes the variants those arms named,
  last.
- Blame/history lineage for the grammar types breaks at the fork; the fork
  commit message must state the provenance.
- Existing `idris-*` and macro-machinery tickets re-aim at the semantics
  mirror and the renamed crates — an open-ended set: grep the ticket tree
  at claim time rather than trusting any closed list here.

## 13. Downsides accepted, with mitigations

1. Per-mechanic tax rises (~seven touch-points per new capability).
   Accepted: exhaustive matches force the author-surface conversation per
   capability; NO mirror-generator meta-machinery (a schema layer would be
   a third defining place — principle 1's evil twin).
2. Wrong-but-well-typed mapping arms are invisible to the compiler.
   Mitigated by §9's correctness story.
3. Divergence governance is prose. Mitigated by the ledger-in-place
   convention and review culture; the failure modes (ratchet vs
   never-diverge) are named so reviews can watch for both.
4. Stage 1 is a coordination big-bang; concurrent efforts (the english
   frames rounds) queue behind the fork window.
5. The formal gate shifted from "core well-formed" to "semantic
   well-formed" — resolved deliberately in §10, with the thin-core-mirror
   escape hatch recorded.

## 14. Rejected alternatives (do not re-litigate)

- **Named binder variables / labels in stored form** (`@doomed`,
  `id: "x"`): names have no source in English — parse must invent, render
  must discard (principle 2); card files are data, not a language.
- **Pure inline storage with hoist inference for all cards**: cannot spell
  a re-mentioned target legally (stored `It` violates the
  targets-never-anaphors invariant; `That` is the wrong channel; labels are
  names).
- **Context/binder machinery inside the unifier**: parse-side
  transduction-in-the-matcher is the wall — no ancestor threading exists
  and every binder feature would compound there.
- **Per-verb compound frame entries**: O(verbs × wrappers) corpus
  explosion; the prototype's own type mismatch documented it.
- **A separate semantic IR beside semantics and core**: a third authority;
  the needed intermediate is only the transient derivation.
- **Mandatory `core::` qualification in bodies**: the diagnostic wins under
  this policy.
- **Wipe-first identity defs with generator-input surface text**:
  reintroduces the pin-table disease (principle 2's external mapping
  files) and destroys hand `frames:`.
- **Storing expanded core beside semantic terms**: they will drift; core is
  a disposable cache only.

## 15. Supersessions and ticket map

The superseded texts below are session-local working records (not in the
tracked tree); the deltas restated here are self-contained.

- The action-role-reshape design's statements that the `TargetedDealDamage`
  catalog entry is permanent are refined: the body stays structural but the
  entry's home and life expectancy change (§8 — spelling-side test fixture,
  then deleted). The reshape's plan-time ident-collision sweep obligation
  is retired for future types by §6's diagnostic.
- The macro-frames round-2 working assumption that the constructor catalog
  grows is superseded: the catalog is a migration source, not a
  destination.
- The macro-author-surface ticket's earlier "wipe-first, never hand-edited"
  identity-macro line is superseded by §5; its earlier claim that this
  program's canon migration subsumes `core-remove-default-args`'s card-RON
  churn is WITHDRAWN by §5's defaults-mirroring rule — that churn returns
  to that ticket.
- Ticket map — Stage 0: `plugin-crate-split` (+ `card-crate-split`,
  cross-cutting/anytime). Stage 1: `authoring-crate-fork`,
  `lowering-crate`, `plugin-repoint`, `idris-mirror-semantics`,
  `runtime-prose-link` (added 2026-08-02 — the render-side seam),
  `core-demacro`. Stage 2: `macro-author-surface` (rewritten),
  `macro-collision-diagnostic`. Stage 3: `spelling-crate-rename`,
  `frames-catalog-merge`. Stage 4: `target-sugar-elaboration`.
  Follow-ups: `plugin-rider-split`, `engine-base-characteristics`,
  `engine-ability-origin-refs` (maybe, trigger-gated),
  `core-regions-substrate`,
  `idris-distinct-position-proof`, `spelling-engine-requirements`,
  `post-reshape-comment-rot`, `ci-idris-gate` (the idris-check baseline),
  the re-aimed `core-remove-default-args`, and the design-gated
  `characteristics-atoms-crate` and [Core is explicit regions](core-explicit-regions.md) (§17).

## 16. Verification obligations

1. Every stage's zero-behavior-change claim is gated: canon + workspace
   suites, `cargo xtask idris-check plugins/canon` no regressions
   (against the `ci-idris-gate` baseline — the batch command currently
   exits 0 on failures, so a diffable baseline is part of the gate),
   fidelity green, before and after each stage lands — with "zero change"
   measured at the two §5 levels (stored-byte round-trip; lowered core).
2. The engine `It`→lone-target compatibility arm is deliberate, guarded,
   and self-documented — its removal requires the corpus re-spell sweep
   first (`core-regions-substrate`).
3. The per-variant mapping tests (§9) run in CI. Coverage is total and
   stays total: a new variant on either side is already a build error, and
   its test lands with it. When an arm diverges, its test is edited to the
   new expected shape in place — which is why each test names that shape
   rather than inferring it; `runtime-prose-link` is the first ticket to do
   this, for the `Expansion` arms it erases. (The raise-map round-trip
   property this obligation used to name is withdrawn — see §9.)
4. The coverage gate (reachability inventory: every row classified) and
   the collision diagnostic get negative fixtures each.
5. The Ephemerate/Cloudshift pronoun pair and the §7 card list become
   spelling test fixtures when their features land; `Fight` is the
   normative occurrence-counting fixture.
6. Sugar acceptance is proven non-semantic: for every sugared fixture,
   `lower(sugar_form) == lower(explicit_form)` byte-for-byte on core.
7. The per-container restriction table (§4) gets an explicit fixture per
   row, including a nested-definition inheritance case.

## 17. Addendum (2026-08-06): the divergence trajectory

Recorded so later design rounds inherit the direction instead of
re-deriving it. §1's fork is the initial condition, not the end state: the
two grammars drift in named directions, and each form spends its
explicitness budget on exactly what its consumer needs.

- **Semantics drifts toward English constructions.** The authored form
  becomes English with explicit grouping — English vocabulary and
  constructions, with only what English leaves implicit (bracketing,
  attachment, ordering) made explicit. The drift lands in the DATA layer,
  not the enums: English-shaped surface accumulates as macro defs and
  frames (`AnyTarget` — a kind-punned Predicate/TargetSpec pair plus its
  lexicon frame — and `Amass` are the existence proofs), while the
  taxonomy enums drift toward generic scaffold (binders, sequencing, the
  primitive verbs). Reference stays anaphoric here (`It`/`That(Sort)`):
  English is anaphoric (style guide §5, "Names, self-reference,
  pronouns, and anaphora"), and the spelling relation needs the anaphors.
  Grouping is what turns explicit.
- **Core drifts toward evaluation shape; its anaphoric channel is
  direction-settled for removal** (superseded 2026-09-02 by [Core is explicit regions](core-explicit-regions.md), which replaces the slot environment below with closed regions).
  The discourse reads mirrored at the fork (`It`/`That(Sort)` outside
  binders, R1/R2-resolved dynamically at engine eval time) are replaced by
  an explicit per-scope slot environment — absolute indices, telescope
  ordering (strictly-earlier references only), per-slot provenance —
  extending §7's indexed announce channel to every statement-level
  binding. Reference is what turns explicit; grouping stays only as
  structural as evaluation needs. Resolution happens ONCE, in lowering:
  the antecedent walk stops being engine machinery and becomes a
  compilation pass with per-card diagnostics. For §9's ledger this is a
  pre-registered earned divergence, the second after §7's derived
  `TargetSpec::Distinct`.
- **The certifier/resolver split** (sharpens §10; assumes
  `idris-mirror-semantics`' reattachment). The Idris mirror keeps the
  whole certification job on semantic input — it proves a card's anaphora
  resolve uniquely (R1/R2), reads sit in range, distinctness is
  well-placed — and lowering is the executable counterpart that computes
  the certified resolution. Two independent derivations of one rule,
  gate-compared: the soundness-gate pattern working as designed, not
  duplication drift. Coverage inverts in lowering's favor — the resolver
  runs on every card at lower time (wizards included) while the gate
  remains the canon-slice certifier. Core stays proof-free (§10), now
  well-scoped by construction; the thin-mirror escape hatch stays
  theoretical.
