---
needs: [english-construction-rewrite-design, english-v2-catalog-pipeline]
---
**Hand-write the construction compiler's target output for the first vertical
slice of `english_v2`: the AST, exact renderer, and total visitor for five
grammatical categories — NO parser of any kind.** Stage 2 of the rewrite's
implementation sequence. Parsing arrives only with `english-v2-earley-engine`
(stage 3); do not write an interim parser, however small — an ordered-choice
or recursive-descent stopgap is exactly the architecture the ADR bans, and a
previous run of this ticket failed by inventing one under an under-specified
brief. If something below cannot be completed as written, STOP and report;
do not fill gaps with judgment calls.

This ticket writes BY HAND, in `deckmaste_english_v2`, the code the future
construction compiler must generate, so codegen has a golden target before it
exists. Every dimension is pinned by `docs/decisions/english-v2-rewrite.md`:

- **AST** (§Generated code contract): transcript style — one struct per
  construction, wrapped in its category enum (`pub struct DealDamage
  { amount, to }`; `VerbPhrase::DealDamage(DealDamage)`). Categories:
  `Ability`, `Sentence`, `Clause`, `NounPhrase`, `VerbPhrase`, plus `Amount`.
  Constructions: exactly those the sentence set below needs. No
  recovered-text or raw-string nodes anywhere.
- **Checked construction**: shapes with invariants get checked constructors
  and non-public fields, so values the renderer accepts but the grammar
  cannot re-read are unrepresentable — triggered ability holds exactly one
  effect for now (comment: multi-effect awaits grammar buildout);
  catalog-bound identity spellings are validated at construction against the
  bound catalog/context. Invariant-free shapes stay plain public fields.
- **Renderer** (§Bidirectionality): total and exact over constructible
  values. Derive, don't store — verb agreement (inherited attribute derived
  at clause/sentence nodes), noun number (a/an ⇒ singular; that ⇒ singular /
  those ⇒ plural; bare target ⇒ singular), capitalization (positional:
  ability-initial and after terminal periods only, NOT after the trigger
  comma), whitespace (single space; punctuation binds left). Stored: only
  non-derivable surface facts (self-reference abbreviated-vs-full spelling).
  Render no path the sentence set cannot exercise — no speculative
  multi-sentence joining.
- **Visitor**: a total traversal over every AST type — exhaustive `match`
  everywhere, no `if let` or wildcard arms, so a future variant is a compile
  error — with callbacks for constituents AND closed-class leaves (vocab
  words, lexemes, amounts, identities).
- Lexical atoms with internal structure store their parts (sign +
  magnitude), never fused strings. The word "hole" is reserved for the
  future macro feature; constituents are plain typed roles. Anaphora are
  unresolved syntax; the `where` clause is a syntactic binder node; a
  variable is the same leaf value at every occurrence of its name.

Sentence set (render targets — byte-exact PARSE verification lands with the
engine ticket; here each sentence must be constructible through public
constructors and render to exactly these bytes):

1. "Destroy target creature."
2. "Whenever a player connives, that creature deals X damage to it."
3. "You gain X life, where X is the number of creatures you control with
   power 2 or less."
4. One real corpus sentence (implementer's choice) using the card's
   self-reference — binding the self-name context parameter plus at least
   one catalog through the catalog-pipeline loader, storing
   abbreviated-vs-full spelling.

Tests: exact-bytes render assertion per sentence; constructed-value tests
building each through public constructors only; at least one constructor
rejection asserted (e.g. zero effects).
Acceptance: those tests green; `deckmaste_english_v2` dependencies unchanged
(catalog layer only); no parser module exists. Standard constraints apply.
