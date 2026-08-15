---
needs: [english-v2-earley-engine]
---
**Harden the landed Earley engine into the shape stage 4's generator can
target: re-key the chart on rule identity, split the grammar module's four
jobs, and sweep the review findings.** Follow-up to the 2026-08-15 stage-3
review; the declaration-compiler ticket (stage 4, not yet minted) will depend
on this. Every item is pinned — if something cannot be completed as written,
STOP and report. All work in `crates/deckmaste_english_v2`; authority
`docs/decisions/english-v2-rewrite.md`.

1. **Re-key the engine on rule identity (construction × form).** Today
   packing keys on construction (engine.rs ~:140) and position lookup takes
   the first rule matching a construction (grammar.rs ~:684), baking in a
   one-rule-per-construction bijection the ADR's own multi-form example
   (`quantity_at_least`, two forms) breaks: two forms would mispack into one
   node and feed selection the wrong RHS. Fix: chart packing and position
   lookup key on the RULE; construction identity re-enters at
   selection/materialization (two surviving rules of one construction are
   distinct readings until their materialized values are compared). DELETE
   the bijection lock-in test (grammar.rs ~:1319-1327); replace it with one
   permitting multiple rules per construction while still requiring exactly
   one build arm per rule. Zero behavior change on the shipped tables (still
   1:1 today) — every existing test stays green unchanged.
2. **Split grammar.rs's four jobs** (rule table ~:114-261, scanner
   ~:311-555, materializer ~:556-749, build dispatch ~:750-1015) so the
   generator-targeted artifacts live alone: the rule TABLE in its own module
   and the BUILD dispatch in its own module — those two are what stage 4
   emits diff-identical — with scanner and materializer moving to natural
   engine-runtime homes (e.g. parser/scan.rs, parser/materialize.rs). Pure
   code motion: no behavior change, tests green unchanged.
3. **Sweep, each item as cited by the review:**
   - Field-exhaustiveness for `SelfReferenceNp` (lost at render.rs ~:344,
     :378 and visit.rs ~:304 when the field went private): restore the
     adding-a-field-breaks-the-build property structurally if a cheap shape
     exists (e.g. one exhaustive destructuring accessor returning every
     part); otherwise document the sanctioned exception at the type. Do NOT
     weaken the checked constructor to get it.
   - Delete the dead `SeedPolicy::AllRules` machinery (engine.rs ~:46-50,
     :269-275, :290-302, :354-397) — it anticipates the inspect/probe stage,
     which re-adds it with an actual consumer; the `#[allow(dead_code)]` is
     the tell.
   - Failure spans get real extent: the span covers the furthest column's
     unconsumed/offending token where one exists, zero-width only at end of
     input. Update the pinned span values in the failure tests; the SHAPE
     (span + structured expectation set) is unchanged.
   - mod.rs ~:56: the `.expect` on selection's result becomes a structured
     internal error — its unreachability premises (no nullable rules,
     monotone validation) both change at stage 5.
   - `ParseError`'s `Display` stops interpolating `Debug`
     (`{expectations:?}`, error.rs ~:56-60) — expectations render through
     their own `Display` — and the message-equality assertions pinning
     implementation-transcribed strings (tests/parser.rs ~:61) are retired
     or re-pinned post-prettify. The compile-checked trait facts stay;
     message text is not contract.

OUT OF SCOPE, routed to stage-4 authoring: the demonstrative build arm's
`(That, Singular)/(Those, Plural)` sub-table (a declaration-surface question
— the engine encodes what declarations would express as two forms); the
requeue fixed point's full-sweep cost and the materializer cycle machinery
(stage-5 perf/nullable questions; currently cheap and correct).

Acceptance: all suites green with the only test-content changes being the
itemized ones (bijection test replaced, span values updated, message
assertions retired); both round-trip laws still byte-exact on the five
sentences; a NEW test proves multi-form re-keying works — two rules for one
construction in TEST-ONLY tables, both readings surviving to selection and
materializing distinct values, no mispacking; `cargo clippy
--all-targets -- -D warnings` clean; dependencies unchanged (catalog layer
only); no declaration-compiler machinery. Standard constraints apply.
