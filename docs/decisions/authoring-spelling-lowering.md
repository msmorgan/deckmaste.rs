# Authoring, spelling, lowering — the card-grammar split

Settled 2026-08-02, after the action-role-reshape landed. This decision is
the design contract for the authored-grammar program; the ticket map in §15
implements it. Like every decision doc: it records intended design, current
code wins on incidental drift, and changing it requires explicit review.

## 1. Summary

One authored grammar, two derived projections, three new crates:

```
                deckmaste_spelling            deckmaste_lowering
deckmaste_english ◄════════════════► deckmaste_authoring ────────► deckmaste_core
  English AST       two-way ranked      the authored        one-way    pure engine
  text ⇄ AST only   relation            rules grammar       total      AST; plain
                    (frames ENGINE      + macro layer       compile    serde; NOT
                    lives here;         + Idris mirror                 Idris-mirrored
                    frames: DATA
                    rides authoring
                    defs)
```

- **`deckmaste_authoring`** is the single source of truth: the authored
  rules grammar every container is written in — card files, token files,
  and the `rules/` engine tables. It owns the types, the kind registry and
  `SupportsMacros` machinery, identity-macro registration, the collision
  diagnostic, and the scope-calculus and sugar forms. The Idris mirror
  attaches HERE (§10).
- **`deckmaste_spelling`** is the authored ⇄ English relation:
  `deckmaste_frames` renamed and absorbed. "Frames" survives as the name of
  the lexicon's entries — the mechanism inside spelling.
- **`deckmaste_lowering`** is the one-way compile authored → core: sugar
  desugaring, scope elaboration, normalization, the type mapping, authored
  invariant enforcement, and the divergence ledger.
- **`deckmaste_core`** becomes a pure engine AST: no macro machinery, plain
  serde, no Idris obligations (until an engine-side dependent invariant
  earns a thin mirror back).
- **`deckmaste_card`** (engine-side, sitting directly above core): the
  engine's unit of card definitions — `Card`/`CardFace`/`Token`, layouts,
  type lines, the packaging of primitives into a playable unit — split out
  of core so the loose primitives are testable independently and the
  card-shapes program (transform/saga/adventure/split) has a home. Card
  depends on core, never the reverse; lowering targets both. The authoring
  crate carries its own authored container types; there are deliberately
  two card-type families, one per side of `lower`. The seam between card
  and core is a dependency-inverted interface: core owns
  `trait BaseCharacteristics` (implemented by card's types, and by core's
  own `Token` and face-down bundle) plus
  `CardRef<T: BaseCharacteristics>` at the object/state boundary; grammar
  enums stay monomorphic (`Token`/`TokenSpec` remain core — the grammar
  creates tokens).
- **`deckmaste_cards` is renamed to `deckmaste_plugin`** (Stage 0): it was
  always the loader plus riders; the riders (emitter, validation, fidelity,
  the dying legacy renderer) may split further at the claimant's
  discretion.

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
   load. Every step is therefore reversible.
4. **Productive compositions are machinery, never vocabulary.** A pattern
   that composes with an open class (targeting × verbs, `May` × effects)
   gets a mechanism; only closed idioms (`Mill`, `DestroyNoRegen`) get
   words.
5. **Proof follows purpose.** The Idris gate's obligations are
   author-mistake proofs, so the mirror attaches to the authored grammar,
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
  variants across 6 files) — the authored grammar is container-neutral, not
  card-shaped.
- The frames constructor catalog has exactly 7 entries — retirement is
  cheap now and only now.
- Canon already spells the explicit indexed targeting form (Lightning
  Bolt), so keeping it canonical costs zero migration.
- Card-level ground truth was verified via the mtg-rules toolkit for every
  oracle text relied on: Lightning Bolt, Terminate, Rabid Bite, Seeds of
  Strength, Arc Trail, Arc Lightning, Pyrotechnics, Fling, Do or Die,
  Otherworldly Journey, Ephemerate, Cloudshift.

## 3. Architecture: one authority, two projections

**Authored form is the source; everything else is derived.** English text
is an input to recovery and an output of rendering, never stored truth.
Core is a compiled artifact — cached at most, keyed by content digest,
never treated as source. The spelling relation is **ranked, not
bijective**: several authored spellings may word identically and one
sentence may recover to several candidates; ambiguity surfaces as
candidates, never resolved by assembly order. "Faithful" means semantic
round-trip plus canonical wording; byte-exact replay is out of scope (an
optional non-semantic surface witness can be added later if it is ever
genuinely needed).

Recovery (English → authored) uses transient, in-memory handles for
binder/discourse linking inside its derivation; the derivation is erased to
an authored term and no handle is ever serialized (principle 2).

## 4. The authoring grammar and the parse-position ban

**One grammar, many containers, restriction per container.** Cards and
tokens are macro-vocabulary-only; the `rules/` tables and macro definition
bodies may spell primitives freely; test fixtures choose per fixture.

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
- **The restriction bit follows textual provenance, not the expansion
  frame.** It is recorded when argument text is captured and restored when
  that text is re-read; body re-reads are exempt however reached. (The
  `Ctx::frameless()` param-re-read path is exactly why frame-gating alone
  is wrong.)
- **Entry points**: `Plugin::card`/`token` and the migrations graduation
  path share one restricted entry; the ~230 other `read_str` callers are
  untouched — restriction is opt-in at the entry, never a global default.
- **Structural binding forms carve-out**: `Targeted`, `Target(n)`,
  `Targets(n)`, `Distinct` are designated structural grammar — the
  authoring layer is "a typed macro-invocation tree plus a small built-in
  indexed scope calculus," and pretending the calculus is macros would only
  hide its binding behavior.
- **Straggler registration**: `Color` (currently unregistered as a kind — a
  macro of kind Color is undefinable today), `Supertype` (no
  `SupportsMacros`), and the `Card` root gain registration and trivial defs
  so the ban is uniform. The closed `Type` enum needs nothing (it appears
  only in machinery positions post type-flip — the landed migration of
  card type lines from the closed `Type` enum to `TypeDef` macros);
  counters already conform (name vocabulary behind their hand
  deserializer).
- Negative fixtures prove a bare-primitive card fails with a useful error.

## 5. Identity macros: scaffolded once, then hand-owned

Every authoring-reachable variant gets a wrapping macro def under its
variant's own name (mirror-by-default — this is what makes the ban
near-zero-churn: invocation syntax is identical to variant syntax, so canon
re-parses byte-identically). **Reachable** means: variants of kinds in the
registry reachable from the restricted-container root types via field
types, flatten, and embed — excluding the name-erasing loader-tag kinds
(`Macro`, `KeywordAction`, and kin). Defs are **generated once as
scaffolds, then hand-owned**: they must carry hand-written `frames:` (§8),
which wipe-first regeneration would destroy, and hand ownership is what
makes a core-side rename a one-line body edit with canon untouched. A
**coverage gate** replaces regeneration: every reachable variant has
exactly one covering def; arities verified; missing defs scaffolded on
demand. Registration kinds are computed from the flatten graph (a variant
spellable at a flatten host's positions registers its wrapper there too).

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
exemption is **unforgeable generated metadata** (`IdentityOf(kind,
variant)`-style), never body-equality. Per-kind, because same-name
different-kind reuse is intentional practice (`Draw` the verb vs `Draw` the
event filter). Current corpus has zero collisions — clean slate. This
retires `action.rs`'s comment-enforced naming rule. A mandatory `core::`
body namespace is consciously NOT shipped; it remains a possible future
escape hatch if a non-identity collision is ever genuinely wanted.

## 7. Targeting and the scope calculus

**Core invariant (the design's fixed star): announced targets are never
anaphors.** They are read only through the indexed channel; `It`'s
antecedent-stack resolution (R1 = nearest singular antecedent, R2 = refuse
when a second compatible antecedent makes it a guess) excludes them;
post-move re-mentions are the separate `That(Sort)` product channel.
Current code deviates in two known, ticket-owned places: the engine
carries a deliberate lone-target `It` compatibility arm
(`engine-it-target-fallback-removal`), and the doc comment on `Targeted`
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
  Discrimination from index reads is syntactic: at a Reference/Selection
  position, `Target(<numeral>)` / `Targets(<numeral>)` is an index read;
  any other argument shape is the sugar. Sugar is legal only in
  card-authored provenance text — a macro body may FORWARD sugar through a
  `Param` hole but may never introduce it (bodies are scope-free).
- **Occurrence counting has no "primary/secondary" distinction**: every
  authored `Target(n)`/`Targets(n)` site counts, including characteristic
  reads (`PowerOf(Target(0))` is a site). English `Mention` occurrences
  and expanded-core reads do NOT count (an idiom's single argument may
  expand to many core reads and stays sugar-eligible). A sugared slot is
  unreferenceable by construction — no index exists in the authored text —
  so the one-site rule is enforced by syntax; needing a second site means
  writing the explicit form. Worked verdict: Rabid Bite's slot 0 has two
  authored sites (agent position and the power read), so it must be
  explicit; its slot 1 has one site and may append inline under the
  mixing rule below.
- **Direction labels on the two scope rules** (they are not in tension —
  they govern different directions): *acceptance-time validation* — a
  sugared introduction is accepted iff it is that slot's only authored
  site, and a `Distinct` edge cannot attach to a sugared slot at all
  (authored indices must be `< E`); *canonical-writer contraction* (the
  future formatter direction) — a slot contracts to inline form only if it
  has one site and no `Distinct` edge, and a cross-constrained connected
  component stays wholly explicit.
- **Mixed scopes are legal via the explicit-prefix rule**: explicit slots
  permanently own `0..E`; inline sites append in a specified traversal
  (textual) order; authored indices must be `< E`; the validator rejects an
  authored index landing on a generated slot.
- **The elaborator is the only scope introducer.** Idiom bodies are
  scope-free with spec-typed parameters. Elaboration normalizes to the
  engine's single top-level `Targeted` per spell/ability and stops
  hoisting at genuine announcement boundaries (modal modes, delayed
  triggers — per-mode target scopes are never flattened into an
  unconditional outer list [CR#601.2c]).
- **`Distinct` is a set-level disjointness constraint** (two unconstrained
  slots may legally choose the same object), checked after selection and
  at legality recheck. No new constructor; canonical well-formedness:
  sibling indices strictly earlier, sorted, deduplicated, each undirected
  edge stored on the later slot. (The Idris side currently proves range
  only; the position-strengthening is `idris-distinct-position-proof`.)
- **Re-mentions**: same-slot re-mentions are `Target(n)` in the full form
  or owned inside idiom defs; cross-zone re-mentions are `That(Sort)`.
  Surface pronouns do not map one-to-one onto channels — Ephemerate says
  "return it" and Cloudshift "return that card" over the same product
  structure; pronoun choice is a spelling-side realization decision and
  parse-side disambiguation needs the zone-aware discourse environment.
  Canonical test pair: Ephemerate / Cloudshift; same-slot case: Terminate;
  slot-read case: Rabid Bite; repeated-clause case: Seeds of Strength
  (each syntactic introduction is a fresh slot); relative case: Arc Trail
  (`Distinct`); nested-predicate case: Do or Die (an inline introduction
  inside a filter is legal; the desugar traversal collects it — under the
  explicit form nothing needs discovering at all).

## 8. Spelling (the authored ⇄ English relation)

- **The constructors catalog retires.** Its 7 entries migrate into their
  words' `frames:`; the lexicon becomes single-origin; macro entries absorb
  the constructor-only capabilities (body patterns, `announcement:`).
  Compound entries for productive compositions are the frames engine's
  job, not vocabulary (principle 4): the bodied `GainLife` entry's frames
  land on the life word's def, and `TargetedDealDamage` survives only as a
  non-author-facing spelling-side test fixture (a differential baseline)
  until compositional targeting covers it, when `target-sugar-elaboration`
  deletes it.
- **`template:` fields sunset with the legacy renderer** (they are already
  build-gated projections of `frames[0]`).
- **Requirements handed to the english effort** (interface, not
  implementation): `Argument` vs `Mention(Full | Pronoun | Demonstrative)`
  occurrence classes on frame specs and compiled frames — the `${0:pro}`
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

## 9. Lowering (the authored → core compile)

Phases of the one pass: desugar (sugar → structural forms) → scope
elaboration (collect, order, index; §7 rules) → normalization (single
top-level `Targeted`) → type mapping (authored types → core/card types) →
authored-invariant enforcement at the point where violation would produce
wrong core. Normalization (authored → authored normal form) is an
authoring-crate transformation — sugar is authoring's own feature — so the
Idris emitter can reach the normal form without depending on lowering;
lowering invokes it and owns the cross-grammar mapping.

- **The mapping starts as an exact mirror** (identity-shaped arms,
  generated), because the fork duplicates the freshly-reshaped grammar.
- **Divergence discipline**: shapes stay mirrored unless a divergence earns
  its mapping complexity; every non-identity arm carries its justification
  in place — the crate IS the divergence ledger. Authored terms have no
  independent semantics: a term means its image under `lower`.
- **Correctness story** (because the compiler forces totality, not
  correctness): generated identity arms while mirrored; a debug-only raise
  map enabling round-trip property tests on the mirrored subset;
  per-variant mapping tests for every diverged arm; downstream gates
  (engine suites, fidelity) as the backstop. The round-trip property is
  plain structural equality — the calculus is indexed and index
  assignment is deterministic, so no renaming equivalence is needed; if a
  future contraction pass reorders slots, equivalence is then "equal up to
  consistent slot renumbering with `Distinct` edges re-anchored," and that
  definition lands with that pass.
- The error taxonomy (authoring parse errors / lowering errors / core
  validation) gets its one deliberate pass here, where the layers meet.

## 10. Idris: the mirror attaches to authoring

The mirror's existing obligations are author-mistake proofs — unbound-
anaphor soundness (the R1/R2 gates), target-read range and cardinality,
and `Distinct` range checking (position-strengthening is
`idris-distinct-position-proof`) — so the mirror follows its purpose
(principle 5). One long-claimed obligation is NOT real: the
riders-battlefield-only rule exists solely as prose on `EnterRider` in
`action.rs` ("rejected by the Idris re-emit gate" — it is not), and the
emitter passes rider shapes through or gaps on them without destination
checks. Minting that proof — or correcting the prose — rides
`idris-mirror-authoring`.

- The mirror models the **authoring kernel**: the post-expansion,
  post-desugar normal form (macros and sugar erased exactly as loading
  erases them). On day one this is shape-equivalent to today's `Core.idr`,
  so reattachment is a rename plus repointing the emitter at authored
  terms — which also frees the emitter from depending on lowering (§9:
  normalization is authoring-side).
- Sugar-level syntactic rules (occurrence criterion, prefix rule) are
  desugar-time Rust checks; Idris proves semantic invariants on the normal
  form. English-ward divergence brings its proof obligations with it.
- **`deckmaste_core` carries no proofs**, deliberately: its divergences are
  engine-driven and Rust-testable. If a genuinely dependent engine-side
  invariant appears, a thin core mirror can return for that obligation.
- Known gate-coverage debt is unchanged by the move and more naturally
  closed after it (the emitter walks stored artifacts directly): the canon
  emitter gap and ungated wizards corpus tracked by
  `validate-unbound-anaphor-lint`.

## 11. Staging

Stage numbers are thematic groupings; the wave map below (equivalently,
the ticket `needs:` graph) is the schedule — a later-stage ticket may
legally land before an earlier-stage one when its needs are met. Every
landing keeps all gates green (zero behavior change until Stage 2 begins
the visible policy).

- **Stage 0 — free the names**: `deckmaste_cards` → `deckmaste_plugin`
  (claimant may split loader vs riders further); `deckmaste_card` split
  out of core (sequenced flexibly). No semantic change.
- **Stage 1 — the fork**: `deckmaste_authoring` created as a mirror of
  core's grammar WITH the macro machinery; `deckmaste_lowering` with the
  generated identity mapping; loaders and migrations repoint (parse
  authoring, lower to core); Idris mirror reattaches; THEN core is stripped
  of macro machinery, with the test-fixture sweep (the priced big-boring
  item: every `#[cfg(test)]` macro-aware spelling of core values repoints
  or re-spells).
- **Stage 2 — the author surface**: the ban, identity-macro scaffold +
  coverage gate, straggler registration, negative fixtures; the collision
  diagnostic may land any time (independently early).
- **Stage 3 — spelling consolidation**: `deckmaste_frames` →
  `deckmaste_spelling`; catalog merge; capability unification.
- **Stage 4 — sugar and scope**: inline target sugar + elaboration rules.
- **Throughout**: laws first — expansion equality and the structural
  round-trip property are owned by `lowering-crate`;
  canonical-vs-exact-rendering semantics by the spelling side — and the
  legacy renderer + regex migrations run as shadow oracles with ratcheted
  coverage until each feature family crosses its gates (ratchets and kill
  criteria are owned by each family's landing ticket; the comparison
  harness rides fidelity in the plugin crate) — measurements, not
  authorities — then die.

Parallelism contract (the ticket `needs:` graph encodes this):

```
wave 1:  plugin-crate-split ∥ card-crate-split ∥ macro-collision-diagnostic
         ∥ the planned soundness tickets (engine-it-target-fallback-removal,
           idris-distinct-position-proof, post-reshape-comment-rot)
spine    authoring-crate-fork → lowering-crate → plugin-repoint
(waves     (side lane, off the pinch path: spelling-crate-rename —
 2–4):      needs only the fork)
pinch:   plugin-repoint  (the one true serialization point; freeze window)
wave 5:  macro-author-surface ∥ idris-mirror-authoring
wave 6:  core-demacro ∥ target-sugar-elaboration ∥ frames-catalog-merge
```

The two Stage-0 renames conflict textually (workspace-wide import sweeps),
not semantically — integrate them back-to-back or fold both into one
workspace. Steady state after Stage 1, contention is sharded by crate:
english rounds in spelling + authoring data, def/frames work in authoring,
engine correctness in core/card, with the dependency DAG making
cross-interference a build error.

## 12. Migration inventory and blast radius

- Canon churn ≈ 0 at every stage (mirrored names; canon already spells the
  explicit targeting form).
- The Stage-1 fixture sweep is the largest single line-item (engine
  resolve/trigger test modules, core mana/filter tests, the plugin-crate
  fixture helpers, integration suites).
- `Expanded` / `remembers_expansion` invocation provenance relocates from
  core values to authored values (a real sub-project inside Stage 1, not a
  rename) — including the ~97 production `Expanded(…)` match sites across
  ~14 `deckmaste_engine` modules, which stop existing once core values
  carry no wrappers.
- Blame/history lineage for the grammar types breaks at the fork; the fork
  commit message must state the provenance.
- Existing `idris-*` and macro-machinery tickets re-aim at the authoring
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
5. The formal gate shifted from "core well-formed" to "authored
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
- **A separate semantic IR beside authoring and core**: a third authority;
  the needed intermediate is only the transient derivation.
- **Mandatory `core::` qualification in bodies**: the diagnostic wins under
  this policy.
- **Wipe-first identity defs with generator-input surface text**:
  reintroduces the pin-table disease (principle 2's external mapping
  files) and destroys hand `frames:`.
- **Storing expanded core beside authored terms**: they will drift; core is
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
  identity-macro line is superseded by §5.
- Ticket map — Stage 0: `plugin-crate-split` (+ `card-crate-split`,
  sequenced flexibly). Stage 1: `authoring-crate-fork`, `lowering-crate`,
  `plugin-repoint`, `idris-mirror-authoring`, `core-demacro`. Stage 2:
  `macro-author-surface` (rewritten), `macro-collision-diagnostic`.
  Stage 3: `spelling-crate-rename`, `frames-catalog-merge`. Stage 4:
  `target-sugar-elaboration`. Follow-ups:
  `engine-it-target-fallback-removal`, `idris-distinct-position-proof`,
  `spelling-engine-requirements`, `post-reshape-comment-rot`, and the
  re-aimed `core-remove-default-args`.

## 16. Verification obligations

1. Every stage's zero-behavior-change claim is gated: canon + workspace
   suites, `cargo xtask idris-check plugins/canon` no regressions,
   fidelity green, before and after each stage lands — with "zero change"
   measured at the two §5 levels (stored-byte round-trip; lowered core).
2. The engine `It`→lone-target compatibility arm is deliberate, guarded,
   and self-documented — its removal requires the corpus re-spell sweep
   first (`engine-it-target-fallback-removal`).
3. The raise-map round-trip property (structural equality on the mirrored
   subset) runs in CI while any mirrored arm exists; `plugin-repoint`'s
   provenance relocation is the moment the property's scope first
   shrinks, and that ticket owns adjusting it.
4. The coverage gate (variant ↔ identity def) and the collision diagnostic
   get negative fixtures each.
5. The Ephemerate/Cloudshift pronoun pair and the §7 card list become
   spelling test fixtures when their features land.
6. Sugar acceptance is proven non-semantic: for every sugared fixture,
   `lower(sugar_form) == lower(explicit_form)` byte-for-byte on core.
7. The per-container restriction matrix (cards/tokens restricted; rules
   tables and bodies free) gets an explicit fixture per container kind.
