---
needs: [english-v2-vertical-slice]
---
**Harden the landed vertical slice into a freezable codegen golden: thread
the render context, store only non-derivable facts, delete phantom leaves,
and unify the generator-shaped idioms.** Follow-up to the 2026-08-15 stage-2
review; stage 3 (`english-v2-earley-engine`) freezes against this golden, so
this lands first. Every item is pinned by
`docs/decisions/english-v2-rewrite.md` (§Bidirectionality now pins
context-threading); if something cannot be completed as written, STOP and
report. All work in `crates/deckmaste_english_v2`.

1. **Context-threaded rendering.** `render` takes the parse context (the
   card's own name — the same context stage 3's `parse(s, ctx)` will
   share). `SelfName { full, abbreviated }` (ast.rs ~:266-269) is DELETED: a
   self-reference stores only its spelling variant
   (`SelfReferenceSpelling::Abbreviated | Full`); the renderer derives the
   name text from ctx, using the same abbreviation rule the constructor
   used. This also removes the render-side `.expect("checked constructor")`
   (render.rs ~:191) — the cross-field invariant disappears with the fields.
2. **One spelling source per surface word.** A word a construction's form
   writes is a form literal ONLY — the visitor must not synthesize lexeme
   leaves for it. Remove the visitor-synthesized lexemes for words the
   renderer emits as literals ("damage", "life", "the", "number", "of",
   "power": render.rs ~:139-141, :157, :164, :203 vs visit.rs ~:278, :325,
   :345, :353). Rule: a surface word is either a stored lexeme field (then
   the renderer reads it) or a form literal (then the visitor never sees
   it); in this slice's sentences all of the above are literals.
3. **Delete the phantom `Comparative` vocab** (ast.rs ~:205-208; synthesized
   at visit.rs ~:327; renderer hardcodes "or"/"less" at ~:205-206). "or
   less" is a form literal until a second comparative form actually exists —
   no enum, no leaf callback. Transcript honesty cuts speculative vocab too.
4. **Per-kind catalog casing.** Replace the unconditional
   `singular.to_lowercase()` (render.rs ~:229) with a casing property on the
   catalog kind: card types lowercase in running prose; subtypes keep their
   printed case (corpus: "an Equipment card", "any number of Auras").
   Implement for the kinds the slice touches; the property rides the kind so
   grammar buildout extends it per catalog.
5. **Idiom uniformity for codegen.** Renderer matches become
   field-exhaustive like visit.rs — no `..` rest patterns (render.rs ~:292,
   :296, :303, :322, :326, :332); a new field must be a compile error in
   BOTH modules. `SignedNumber` (ast.rs ~:210-231) loses its ceremony —
   plain public fields, it carries no invariant.

Tests: every existing slice test stays green with the rendered BYTES
unchanged (only signatures and storage move); the constructed-value tests
build self-references without any name string; add one test rendering the
same self-reference value under two different card contexts and asserting
the outputs differ in exactly the name. Acceptance: tests green; no card
name text stored anywhere in the AST (grep); renderer and visitor both
field-exhaustive; dependencies unchanged; still no parser module. Standard
constraints apply.
