---
needs: []
design: true
---
**[design] Re-evaluate and design the clean replacement for the English
construction parser.**
The current construction compiler accumulated migration adapters, inverse and
projection compatibility, policy metadata, and recovery-preservation machinery
inside the parser-generator core. Design its replacement before porting more
grammar.

This ticket is intentionally independent of completing the current recovery
sweep. Derive the replacement grammar primarily from
[`docs/oracle-style-guide.md`](../../oracle-style-guide.md), then test it against
the current authoritative Oracle corpus. Consult the CR where a grammatical
distinction depends on rules meaning. The legacy regex migrations, old parser
acceptance, recovered trees, and their tests are not grammar evidence or a
corpus of prior examples. Inspect the old implementation for independently
useful mechanisms and engineering lessons, including catalog and vocabulary
design, source/span handling, rendering and diagnostic infrastructure, and
failure modes worth avoiding. Salvage an idea only after justifying it against
the rewrite's requirements; never retain it because old output or compatibility
depends on it.

## Do not inherit decisions by default

Begin by auditing the tracked English-parser architecture decisions and their
premises. Classify each as reaffirmed, amended, superseded, or still open. A
decision is not binding merely because previous work was organized around it,
and sunk implementation cost is not evidence for retaining it. Record the new
rationale in tracked architecture documentation so the rewrite does not depend
on conversational history.

In particular, reopen rather than assume:

- whether Oracle text compiles directly to the project's canonical semantic IR
  or first to a distinct linguistic AST;
- which representation is authoritative after parsing;
- how much exact surface information belongs in semantic values;
- whether inverse rendering is defined for every semantic value or only parsed
  values;
- whether packed ambiguity is part of the public model or development tooling;
- whether any recovery/partial-parse representation should exist;
- whether serialization is needed at any actual boundary; Serde must not be
  used as a tree-walking or control-flow mechanism;
- which old consumers and public APIs deserve continuity at cutover.

## Current direction to evaluate

The following is the proposed starting direction from the design conversation,
not an immutable specification. The design ticket may amend it when a simpler
coherent model better meets the project objective:

- Create `deckmaste_construction` from scratch. It owns the proc-macro entry
  point as well as declaration parsing, validation, and code generation; do not
  preserve the current compiler/macro split. Resolve explicitly how generated
  code obtains any ordinary runtime support that a proc-macro crate cannot
  export.
- Create `deckmaste_english_v2` for the new canonical English AST, feature
  model, lexer/catalog integration, construction macro invocations, lowering,
  selection, and rendering. Use the new macro in the first vertical slice.
- Create `cargo xtask english_v2 ...` commands that provide the inspection
  and full-corpus verification capabilities needed to develop the replacement.
- Prefer an exact parse API that may fail. Evaluate explicitly whether any
  recovered/partial result belongs in a separate tooling type; a failed input
  must not silently become a successful semantic AST through opaque clauses or
  invented structure.
- Preserve complete grammatical alternatives during recognition unless the
  design dialogue establishes a simpler representation with equivalent
  ambiguity inspection. Keep selection policy distinct from grammatical
  recognition.
- Keep failure tooling first-class: expose useful maximal constituents, spans,
  expected categories, rejected feature transitions, and bounded diagnostics.
  Decide its precise representation during the design.
- Old recovery counts, AST layouts, and internal APIs are differential evidence,
  never acceptance baselines. Do not add compatibility adapters between the old
  and new parser implementations.
- Cut over by replacing the old construction and English crates once the new
  path satisfies its gates; do not leave a permanent dual-parser bridge.

## Settle in the design dialogue

1. **Minimal macro language.** Write a one-page grammar and semantics for the
   smallest useful declaration algebra. Start from typed categories, recursive
   holes, sums/products/sequences, lexical fields, feature admission, reversible
   surface forms, and packed alternatives. Derive the necessary constructions
   from the style guide and current Oracle corpus. Every additional facility
   needs an independent grammar reason—not an old-layout or migration reason.
2. **Canonical AST ownership.** Decide which values the macro generates and
   which English types remain handwritten. Make checked semantic owners the
   normal output; do not begin with constructor/destructor adapters or lenses
   around a legacy AST.
3. **Bidirectionality.** Define the laws for parse, AST construction, inverse
   rendering, exact punctuation, card identity, and catalog-backed lexemes.
   Specify whether multiple surface forms map to one semantic value and how an
   exact originating form is retained when semantically necessary.
4. **Parse and ambiguity API.** Specify the forest representation, complete-root
   result, alternative enumeration, deterministic selection, limits, and
   registration-order invariance. Distinguish grammatical ambiguity from
   implementation nondeterminism.
5. **Honest failure diagnostics.** Design a bounded diagnostic API over failed
   recognition/lowering attempts. It may expose partial structures for tools,
   but those structures must use a diagnostic type that cannot be confused with
   the canonical English AST or satisfy a successful corpus parse gate.
6. **English semantics boundary.** Re-evaluate and define the relationship among
   any linguistic AST, exact rendering, persisted representation, and playable
   semantics. State which representation is authoritative and why. Parsing and
   round-tripping all Oracle text is not sufficient for cutover if the chosen
   representation cannot support the project's playable-card objective.
7. **Verification tooling.** Specify `xtask english_v2` contracts for
   `inspect`, `probe`, exact full-corpus parse, byte-exact roundtrip, packed
   ambiguity census, structured failure reports, performance limits, and
   old-versus-new mover reports. Mover reports diagnose differences; they do not
   require preserving the old result.
8. **Cutover and deletion.** Define dependency gates, shadow-running, downstream
   consumer migration, performance expectations, and the exact point at which
   the current crates and their compatibility machinery are deleted.

## Required design output

- A tracked architecture decision with the crate/dependency diagram, core data
  types, public APIs, invariants, rejected alternatives, and an explicit audit
  of which prior decisions it reaffirms or supersedes. It must record the source
  hierarchy: style guide first, current Oracle corpus as conformance evidence,
  rules meaning where necessary, and no evidentiary role for legacy migrations.
- A representative vertical slice—at least ability, sentence, clause, noun
  phrase, and verb phrase—shown as proposed macro invocations with the resulting
  AST and exact inverse rendering. A disposable prototype is allowed when prose
  cannot settle a question, but this ticket does not port the corpus grammar.
- A salvage ledger that evaluates ideas independently. Copy no implementation
  wholesale. Begin by examining catalogs, vocabulary and lexical indexing,
  source/span tracking, renderer organization, ambiguity representation, and
  diagnostic tooling for reusable mechanisms. Also explicitly justify retaining
  or rejecting typed categories, sequence invariants, feature combinators,
  inverse forms, dominance, lenses, bind adapters, projection metadata,
  evidence/witness metadata, and recovery nodes. Prefer ordinary typed traversal
  APIs; evaluate Serde only for a real serialization boundary, never as a tree
  walker. Old acceptance, parse trees, recovery output, and migration behavior
  are never salvage criteria.
- A small implementation sequence with measurable vertical milestones. Do not
  mint additional tickets from that sequence without explicit user approval.

The design is complete only after the user approves the macro surface, AST
shape, failure/ambiguity contract, verification gates, and cutover plan.

Standard constraints apply.
