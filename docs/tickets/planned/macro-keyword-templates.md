---
needs: []
design: true
---
Template parameters for keyword macros. **Base goal already satisfied; only typed
cycling remains, and it is design-gated (rescoped 2026-07-09).**

The parameterization mechanism (macro `params`/`Default`/conditional-fragment
templates + the `TemplateIndex` slot codec, from `macro-typed-holes-contracts`)
landed, and the two straightforward named templates are implemented and tested
(`macro-first-wave`):
- **Ward** — `plugins/builtin/macros/keyword/Ward.ron`:
  `params: { cost: Cost, where_x: Default(Count, 0) }`.
- **Protection** — `Protection.ron`: `params: [Predicate]`.
Both round-trip (parse⇄render); `cargo test -p deckmaste_cards --lib` green.

## Remaining: typed cycling — a distinct design-gated follow-up

`core-intrinsic-keywords-policy` (done) settled this in `docs/keyword-policy.md
§5` / §15.1: typecycling `[CR#702.29e]` "mirrors Landwalk" and is a **separate**
item, NOT folded into the base body. Landing it needs THREE coupled pieces (only
the third is a genuine design decision; the first two are churn that buys nothing
without it):

1. **Signature migration** — Cycling is `params: [Cost]` (positional) and every
   graduated card authors it positionally (`Keyword(Cycling([Mana([Black])]))` —
   Barren Moor, Barkhide Mauler, …). A defaulted `type` param forces Cycling to a
   NAMED signature (`Default` is named-only, `macro_ron/src/param.rs`), which
   breaks every existing invocation + needs an emitter change + cycling-corpus
   regen.
2. **Behavioral body variant** — the param is inert unless the body becomes the
   Typecycling library-SEARCH body (per §5/§15.1), not `Draw(1)`. Landing the
   signature alone enables zero cards.
3. **Bounded slot-reader codec (the design decision)** — typed cycling glues the
   quality as a PREFIX to a literal ("slivercycling {2}", "mountaincycling {2}").
   The current slot reader (`migrations/src/parsers/macro_template.rs::slot_reader`)
   consumes to line-end and `quality_filter` (`keyword_ability.rs`) rejects any
   input with a space, so a predicate prefix before `cycling` with a trailing
   cost cannot be matched back. Round-trip needs a BOUNDED slot reader (stop
   before the next literal) — a codec change no existing macro uses. Landwalk
   sidesteps this by not templating its quality (typed landwalk stays bare
   keyword names in the bespoke parser); typed cycling wants the same deferral
   until the bounded codec is designed.

Claim only with a design pass on the bounded-slot-reader codec; then land all
three together (signature migration + search body + bounded codec). Surfaced by
the batch executor.
