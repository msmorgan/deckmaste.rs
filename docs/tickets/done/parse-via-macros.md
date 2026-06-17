---
needs: []
---
**[design]** Make the macro library participate in card parsing, so the parser
derives from the same declarative source the renderer and the authored cards
already use — one macro grammar driving authoring-expansion, rendering, AND
parsing, instead of the parser and renderer each re-encoding what every mechanic
looks like.

The renderer already runs that grammar one way: core → English via macro
templates (the `template::expanded` path; see `render-template-first` /
`render-template-grammar-args`). This is the reverse — English → core — with the
parser consulting the registry: it knows what *kind* of macro it's targeting and
looks registered macros up by template (a kind-scoped reverse index from
template-shape → macro), emitting the matching macro invocation instead of
building core nodes by hand.

Why it's [design], not plumbing: rendering is deterministic (one canonical
output); parsing real oracle text is not — synonyms, word order, optional
clauses, punctuation — and the macro language is deliberately declarative (no
control flow), so template-matching alone can't absorb that. The realistic shape
is a **hybrid that grows**: macros drive the regular mechanics, the bespoke
parsers keep the irregular tail, macro coverage expands as mechanics get clean
parseable templates. No big-bang rewrite.

Settle first: (a) how a macro template serves as a *parse* pattern — does a macro
declare a parse side, or is it derived from the render/expand template (ties into
`ExpansionArgs` storing args as raw RON source); (b) the reverse-index shape +
kind-scoping; (c) where the macro path hands off to the bespoke parser without
double-handling.

Line: `macro-parse-index` (lookup primitive) → `parse-keywords-via-macros` (MVP —
keywords are the most regular) → `parse-params-via-macros` (arg-bearing macros).

## DESIGN SETTLED (2026-06-17)

Decisions:
- (a) Parse pattern = **derive-only** from the render `template`; no `parse:`
  field (escape hatch only if a real case forces it). A template-less macro does
  not participate.
- (b) Reverse index = kind-scoped `template → macro`; a `ParsePattern` of literal
  segments + typed slots; specificity-ordered, case-folded, word-boundary
  anchored. → `macro-parse-index`.
- (c) Handoff = a generic `macro_template` parser placed **first** in the resolve
  `REGISTRY`; first-match-wins; unmatched lines fall through to the bespoke tail.
  It doubles as a typed sub-reader for the frame parsers.

Enablers:
- Placeholder sigil `{i}` → `${0}` / `${name}` (kills the mana-symbol collision).
  → `macro-template-sigil` (foundation, lands first).
- **Typed params** (the under-used feature this work vindicates): retype lazy
  `params: [Any]` slots; slot type read from `MacroDef.params` — no body-AST
  reflection.
- **One bidirectional slot codec** keyed on declared type (parse `read` + render
  `show`) — subsumes the old `render-template-grammar-args`. → `macro-slot-codec`.
- Codec-driven matching: slots bounded by grammatical validity, not regex greed.

Build line: `macro-template-sigil` → `macro-parse-index` (nullary) →
`parse-keywords-via-macros` (MVP) + `macro-slot-codec` (typed codec; closes
`render-template-grammar-args`) → `parse-params-via-macros` (full).

Big deletion target: `KEYWORD_NAMES`, `match_keyword_prefix`, and `render_arg`'s
per-keyword argument shapes collapse into index + codec.
