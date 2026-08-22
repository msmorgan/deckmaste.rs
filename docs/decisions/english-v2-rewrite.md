# English v2 rewrite

## Decision

The English construction stack is replaced by a fresh implementation in which
**the construction declaration owns its generated type**. A declaration states
what a construction is grammatically; the compiler generates the AST type, the
parser rules, the exact renderer, and the traversal from that single source.
There is no binding to pre-existing types, no adapter layer, and no handwritten
canonical AST. The old crates are deleted wholesale at cutover; nothing
bridges the two implementations at any point.

Decided in the `english-construction-rewrite-design` dialogue (2026-08-12 to
2026-08-14) on the strength of one measurement: the old canonical AST is a
transcript, not a canonicalizer — across 242 construction declarations (34 of
them internal) and 66 target types, 203 bind through construction-specific
constructor/destructurer adapter pairs, exactly one constructor is shared
corpus-wide, and the sole genuine convergence is the coordination frame. A generated per-construction AST
therefore loses nothing the handwritten AST provided, and every adapter
construct exists only to bridge a gap this design removes.

## Source hierarchy and support corpus

- The [Oracle text style guide](../oracle-style-guide.md) is the primary
  grammar authority; the Oracle corpus is conformance evidence; the CR is
  consulted where a grammatical distinction depends on rules meaning. Legacy
  migrations, old parser acceptance, recovered trees, and their tests carry
  no evidentiary weight.
- The engine's support corpus is **every Vintage-playable card** (Legal or
  Restricted, not Banned). Verification gates ratchet against this corpus;
  the Modern-legal census remains prioritization data only.
- Parser acceptance tracks grammatical Oracle English, not Magic legality.
  "Whenever a player connives, that creature deals X damage to it." is
  multiply rules-invalid and must parse; discourse resolution and rules
  legality live in layers above. That parse succeeding is part of this spec —
  the most dangerous future "fix" would be teaching the parser Magic.

## Crates

~~~text
deckmaste_construction        proc-macro façade over the core lib
deckmaste_construction_core   declaration parsing, validation, codegen
        │ generates code inside
        ▼
deckmaste_english_v2     generated AST + grammar, Earley chart parser,
                         exact renderer, vocab/lexeme/codec tiers,
                         catalog loader

deckmaste_data -> deckmaste_catalogs -> {deckmaste_english, deckmaste_english_v2, xtask}
deckmaste_migrations -> deckmaste_data (temporary)
~~~

Runtime support the proc-macro cannot export (chart engine, forest, codec
traits, loader) lives in `deckmaste_english_v2` itself; generated code only
ever expands inside that crate and references it by `crate::` paths. A
separate runtime crate is minted only if a second macro consumer ever exists.
The compiler is itself split core + façade — `deckmaste_construction_core` is
an ordinary library (declaration parsing, validation, codegen over token
streams; unit-testable, and the engine behind `xtask english_v2 expand`) and
`deckmaste_construction` the thin `proc-macro = true` wrapper — because a
proc-macro crate can export nothing but macros.
At cutover, `deckmaste_spelling` and xtask display paths switch to v2 and the
old `deckmaste_english` / `deckmaste_construction_compiler` crates are
deleted; v2 then takes the `deckmaste_english` name. Shared snapshot models
live in the stable low-level `deckmaste_data` crate. The stable
`deckmaste_catalogs` crate owns extraction, the canonical inventory, line-file
I/O, directory comparison, and a removable legacy adapter; both English
implementations and xtask consume that crate. `deckmaste_migrations` now uses
`deckmaste_data` temporarily for its surviving extraction work instead of
owning the shared models. Thus v2 depends only on the stable catalog layer and
remains independent of every crate scheduled for deletion.

## The declaration language

The vocabulary is the reduced grammatical surface from the 2026-08-12 DSL
classification, with these deltas:

- **Kept:** `construction ID: Category`, named role fields, named `element`
  products, `lex`, `identity`, `opt`, `seq`, abstract sum/product, `require`
  predicates, `derive` feature flow, `form`/`when`/`otherwise`, and the
  literal / role / `lex(..)` / `identity(..)` form atoms.

`require len` remains the only cardinality language; no dedicated `nonempty`
type or declaration returns. A `seq` may independently be `separated by` a
uniform or total positional fixed surface and `terminated by` a fixed surface.
Separation emits only between adjacent members; termination emits after every
member, including the last. The declaration and member count/edge position
derive these surfaces and their lexical ownership; no separator field, value,
form tag, or token node is stored. Oracle sentences are period-terminated,
sentence edges are ASCII-space-separated, and document blocks are LF-separated.
An empty Oracle text is a valid document — a vanilla creature is the zero-block
`OracleText`, rendering to zero bytes and trivially byte-exact — so the document
root's block sequence admits length zero; only non-root sequences may require
members.
This follows the [Oracle text style guide §1](../oracle-style-guide.md#1-write-rules-instructions-not-conversational-prose)
and [§3](../oracle-style-guide.md#3-punctuation-and-glyphs).
- **Added:** inflected atoms — `verb(lexeme)` and `noun(role)` render their
  inflection from derived feature context; and the terminal declarations
  `vocab`, `lexeme`, `codec` (§Terminals).
- **Renamed:** a recursive constituent is a bare category-typed role. The
  word **hole** is reserved for the deferred macro-hole feature.
- **Deleted wholesale:** `group`/`backend`, `own`/`bind` and every `via`
  constructor/destructurer adapter, lens declarations/applications/edits,
  projectors, whole-value `check`/`inverse check`, `evidence`/`witness`
  metadata, explicit ordinals, `selection` policy metadata,
  `serialize`/`deserialize` opt-ins, and declared `dominates`/`dominated by`
  (§Selection). Each existed to bridge to types the declaration no longer
  binds; with the type generated from the declaration, they have nothing to
  adapt.

Old versus new for a real construction:

~~~text
// old (constructions/quantity.rs)                // new
construction quantity_at_least: Quantity {        construction quantity_at_least: Quantity {
    bind Quantity via make_at_least,                  bound: lex Numeral,
        at_least_parts {                              comparative: opt lex ComparativeWord,
        bound: lex QuantityValue via Numeral,
        comparative: opt lex ComparativeWord,         form at_least   when comparative.is_none()
    }                                                     = "at least" lex(bound);
    derive features = quantity_at_least(..);          form or_greater otherwise
    witness numeral = stored bound;                       = lex(bound) "or" lex(comparative);
    witness comparative_word = stored comparative;}
    form at_least @ 0 when check(is_at_least)
        = "at least" lex(bound);
    form or_comparison @ 1 when check(is_or_comparison)
        = lex(bound) "or" lex(comparative);
    selection unique;
}
~~~

Every deleted line bridged to a sealed handwritten type; the old declaration
already stored everything the new guards read.

### Stage-4 MVP subset

Settled 2026-08-15 after the stage-4 preplan audit surfaced expressibility
gaps between the frozen golden and the ticket's original allow-list (the
golden contains codec-, identity-, and require-shaped facts the allow-list
banned; the STOP fence held). The compiler MVP implements exactly this
subset:

- **One grammar-wide invocation.** Every declaration — constructions, vocab,
  lexemes, terminal bindings, roots — lives in a single function-like
  invocation, so validation sees the whole declaration set and each
  aggregate item (category enums, the visitor, rule ids, `RULES`, `build`)
  has one unambiguous producer. Separate per-tier macros cannot validate one
  another and are rejected.
- **Terminal bindings.** `codec` and `identity` declarations appear in
  binding-only form: they generate nothing, but declare an existing runtime
  terminal — its value type, its `Lexical` variant, its runtime render
  function and build leaf, and its traversal schema (parts and visit order,
  e.g. signed-number visits sign then whole value; catalog identity visits
  the identity then its spelling) — so construction roles can reference it
  (`lex SignedNumber`) and codegen can emit the golden's render/walk/build
  references without a type-name switch inside the compiler.
- **Checked constructions.** A construction marked `checked` emits the
  golden's non-public field visibilities, and its build arm calls the
  hand-written checked constructor instead of literal struct construction;
  the constructor-call binding (argument mapping, including `Triggered::new`'s
  interim `vec![effect]` wrapping) is written in the declaration. This is
  the MVP stand-in for `require`-generated constructors; full `require`
  codegen replaces it at buildout.
- **Role refinements.** `require <role> is <Variant>` is the one `require`
  form the MVP implements: it emits the golden's category-variant and
  vocab-value narrowing in `build` (`Triggered.event` accepts
  `Clause::Event` only; `WithWhere.clause` accepts `Clause::Where` only;
  `CountNp.controller` accepts `Pronoun::You` only). Every other `require`
  form remains a named hard error.
- **Feature equations.** Agreement/number derivation uses the equation
  grammar from the stage-4 architecture plan: constant, from-role, and
  exhaustive match-over-vocab equations with a directional IR — sufficient
  for imperative bare agreement, subject→predicate inheritance and equality,
  fixed third-person `Be`, demonstrative word→number (one rule with
  `NounNumber::Either` plus the exhaustive build check), and the count
  construction's fixed pairing.
- **Roots.** A `root` declaration names a category as a parse/render entry
  point: standalone `Render` impl, terminal punctuation, end-of-input in its
  rules.
- Everything else in this section's vocabulary (`opt`, `seq`, general
  `require` predicates, `when`/`otherwise` multi-form, generative `codec`
  bodies, morphology, scanners) remains a named hard error until stage 5.

Terminal bindings, checked-constructor bindings, and root declarations are
counted escape hatches (§Guardrails). A known hole this subset records
rather than fixes: the golden's public AST still admits values the parser
cannot rebuild (a `WithWhere` holding an event clause, a count controlled by
`it`), narrowing `parse(render(v))` to slice acceptance; the structural fix
— require-generated checked constructors or narrowed role types — is
stage-5 work.

## Generated code contract

- **Struct per construction, wrapped in its category enum** (the `syn`
  pattern): `pub struct DealDamage { pub amount: Amount, pub to: NounPhrase }`
  with `VerbPhrase::DealDamage(DealDamage)`. Constructions are nameable types
  so frames, mappings, and the future hole matcher can put one in a
  signature.
- `require` clauses become checked constructors; fields without invariants
  stay public.
- Also generated per declaration: parse rules per form, the exact renderer,
  `Eq`/`Debug`, and arms of a **generated total traversal visitor** — the
  visitor is part of the codegen contract (the future hole matcher is a
  second visitor over it, not a parallel AST).
- **Static round-trip enforcement** (replacing the old dynamic
  `frame_reassembles` checks): every form must place every non-optional role,
  or the declaration is rejected; a multi-form construction's guards must
  select a form deterministically and exhaustively from stored fields — if
  they cannot, the author either splits the construction or stores an
  explicit form tag (a counted event, §Guardrails); a form containing an
  inflected atom with no feature derivation in scope is a compile error.

## Terminals

Every terminal class is a bidirectional value⇄text codec — the construction
contract one level down. Three declaration tiers plus catalogs:

1. **`vocab`** — declared closed word classes (`TriggerWord`,
   `ComparativeWord`, determiners, pronouns): generated enum + scanner +
   renderer + exhaustive round-trip test.
2. **`lexeme`** — inflected lemmas rendered through a default English
   morphology function with declared irregular exceptions; parse accepts any
   inflection and stores the lemma only (inflection is derived).
3. **`codec`** — handwritten scanners for genuinely algorithmic terminals
   (mana symbols, P/T expressions). Small, counted, and structured:
   atoms with internal structure yield fields, never fused strings. Whether a
   codec's spelling is context-derived or stored is a per-codec measurement
   against the style guide, never an assumption.
4. **Catalogs** — open-class identities are regenerated as sorted, deduplicated
   plain-text word lists (one entry per line, no headers) and loaded at parser
   construction. The canonical `data/gen/catalogs` inventory is exactly:

   ~~~text
   ability-words.txt          artifact-types.txt       battle-types.txt
   card-names.txt             card-types.txt           counter-kind-phrases.txt
   creature-types.txt         enchantment-types.txt    keyword-abilities.txt
   keyword-actions.txt        land-types.txt           planeswalker-types.txt
   spell-types.txt            supertypes.txt
   ~~~

   The CR is authoritative for all canonical files except
   `card-names.txt`. Card names come from the local MTGJSON
   `AtomicCards.json`: every face for which `vintage_playable()` is true uses
   `faceName` when present and the full `name` otherwise. That predicate means
   Vintage `Legal` or `Restricted` only; `Banned`, `Not Legal`, null, and
   missing legalities are excluded, independently of card layout. Counter
   kinds remain opaque single tokens except for
   `counter-kind-phrases.txt`, which contains only the multi-token keyword
   counter phrases enumerated by the CR [CR#122.1b]; `+1/+1` and similar
   forms are codec atoms, and MTGJSON is not counter-kind authority.

   A catalog is a pure word list; anything needing per-entry grammar becomes
   a construction, and the card's own name is still a parse-context parameter
   rather than a membership gate. **A catalog exists only where structure
   cannot determine the reading** — to license multi-token spans or resolve
   genuine ambiguity — never to police membership in an open productive
   class; membership policing is legality's business, layers above.
   Casing-in-prose is likewise a grammar concern, not a catalog property:
   any per-kind casing rule (card types lowercase in running text, subtypes
   keeping their printed case) lives on the English side as a total mapping
   over catalog kinds — the catalog layer stays a pure word-list store with
   no presentation properties.

   The old consumers use the separate `data/gen/catalogs-legacy` cache, whose
   generated inventory is exactly the same list minus `card-names.txt` and
   `counter-kind-phrases.txt`. `deckmaste_catalogs::legacy` owns that
   compatibility path, including CR-authorized keyword variants observed on
   Vintage-playable faces, and is removable with the legacy consumers at
   cutover. Canonical and legacy files never share an output directory.

**Terminal generation (stage-5 Plan 03 ruling, 2026-08-17):** vocab,
lexeme, and identity terminals — and the graduated codecs below — are
generated by a **closed set of compiler-owned recipes** (English verb/noun
morphology, structured signed-decimal, terminal choice, context identity,
catalog identity), never by declaration-bound arbitrary callbacks and never
by a general scanner/renderer combinator language. Each recipe derives both
halves of the codec and its round-trip tests from one surface table; the
recipe set itself is closed — adding a recipe is a reviewed compiler
change, not declaration-side vocabulary. Morphology is a named recipe over
one feature axis with declared irregular overrides, and recipes stay
**strictly regular**: only exceptionless mechanical transformations
(append-`s`; append-`ed` when past forms arrive). A pattern with even one
attested exception — `-f` plurals read Dwarves but also Lhurgoyfs — is
lexical and lives in declared per-word overrides, which are expected to
grow with the lexicon; the counted list is their review surface, not a
recipe-extension pressure gauge (ruling 2026-08-20, superseding the
earlier extend-the-recipe reading of a rising count). This ruling graduates signed-decimal numerals
out of tier 3's handwritten examples; mana symbols and P/T expressions
remain handwritten codecs until each earns its own reviewed recipe. The
stage-4 binding-only `codec`/`identity` forms are migration scaffolding
retired at the Plan 03 gate — generated recipes are ordinary declarations,
not escape hatches, and no dual terminal authority survives. A recipe
accepts its **full structural domain** — every surface its shape admits
under the style guide's canonical form — never an observed-corpus subset;
the corpus is conformance evidence and ratchet fuel, not an acceptance
filter (the same rule §Source-hierarchy sets for sentences, one level
down). Spelled-out number words, when they arrive, are a generative recipe
over English number grammar for this reason, never a word list.
Representation bounds (a declared magnitude type) are declaration-side
facts that fail loudly, not hidden limits. Applied 2026-08-20 (stage-5
Plan 03): the noun terminal's structural domain is type-and-subtype
membership — the noun recipe spans both declaration kinds (472
declarations, not the 10 card types the slice began with) — and the
corpus consequence, acceptance 47→48 ("Destroy target Spirit." now
parses), is this ruling working as intended: the prior count was the
artifact of an artificially narrowed terminal, and the corpus is
conformance evidence, not an acceptance filter.

**Lexical coverage gate:** every token of every accepted corpus sentence must
be claimed by a form literal, vocab, lexeme, codec, or identity — an
unclaimed token fails at the lexical layer, loudly.

## Parsing, selection, and failure

- **Engine: a freshly written Earley-family chart parser.** Forced by three
  decisions: the selection pass needs every surviving reading (charts produce
  forests natively; PEG's ordered choice is a banned silent pick; LR/GLR
  wants conflict-engineering on a grammar that regenerates constantly); the
  failure surface below is literally the furthest chart column and its live
  items; and the terminal tiers plug in as the scan step. The old chart
  core's design is a salvage keep — the disease was around it, not in it.
- **Selection:** no authored dominance edges anywhere. The parser preserves
  all surviving readings; one post-parse selection pass picks by computed
  structural specificity (derived from the declarations), plus an explicit,
  countable exception table for rivalries the rule mis-orders. A tie neither
  can break is a **hard error naming both constructions** — in production and
  development alike. Policy lives in one derived rule plus one counted list,
  never smeared across declarations.
- **Failure:** a failed parse returns a structured error — the span of
  furthest progress plus the set of expectations live at that point. There is
  no recovery representation in the canonical AST; any partial-analysis
  tooling uses a distinct diagnostic type that cannot satisfy a corpus gate.

## Bidirectionality contract

Byte-exact, both directions, from day one:

~~~text
render(parse(s)) == s        for every accepted sentence s
parse(render(v)) == v        for every constructed value v
~~~

The second law doubles as an ambiguity detector. **Derive, don't store:**
verb agreement (an inherited render attribute derived at clause/sentence
nodes), noun number (per-construction, from determiner/quantifier content),
capitalization (positional: ability-initial and after terminal periods — not
after the trigger comma), whitespace (single space; punctuation binds left),
and numeral spelling (style-guide contextual rule) are all computed at
render. Stored: only non-derivable surface facts — and the measured corpus
tax of exactness over normalization is roughly one two-variant enum (the
abbreviated-vs-full self-reference spelling, which the old AST also kept even
though the CR licenses collapsing it; that store-the-surface stance is
inherited deliberately). Exactness is cheaper than normalizing: the parser
must recognize every spelling anyway, so remembering which form fired is one
field, while normalizing costs an extra pass plus a weaker conformance-style
test. **Both laws are context-threaded:** `render(v, ctx)` and `parse(s,
ctx)` share one parse context (today: the card's own name; nothing else yet).
Context-derivable text — the self-name above all — is never stored in a
node: a self-reference stores only its spelling variant, and the renderer
reads the name from ctx. A context-free render signature forces name-in-node
and was the root cause of the one HIGH finding in the stage-2 review.

## Guardrails

Structural, not disciplinary:

1. **No raw-text or recovery nodes.** Uncovered input fails loudly.
   Round-trip exactness (on the accepted set) and corpus coverage are two
   separate honest metrics — never one gameable one.
2. **Every escape hatch is countable.** The optional per-category
   bidirectional mapping layer (`map` declarations in their own files),
   handwritten codecs, stored form tags, stored-spelling codecs,
   selection-exception entries, morphology-irregular overrides, and the
   stage-4 MVP's terminal bindings, checked-constructor bindings, and root
   declarations are each enumerable lists reviewed in code review; a rising
   count is the tumor marker.
3. **Corpus irregularities are quarantined as data** — a known-uncovered
   list, never grammar special cases.

## Holes: deferred, with insurance

Ordering is parse-first: read and render Oracle English completely before any
macro-hole capability. Deferral is safe because the failure cascade the old
system built (spelling-located witnesses, linearity bans, scrubbing passes)
traces entirely to its one-way pipeline, and the one structurally-located
hole it had worked cleanly, repetition included. Day-one insurance: structured
lexical atoms (sub-lexical hole class), the generated traversal contract (the
matcher is a second visitor), and generated types (a hole variant is a later
regeneration flag, not a migration). The likely macro representation is a
frame as a function from fillers to AST — holes are parameters, repetition is
using an argument twice — so the parser may never need to see a hole at all.
Repeated referring expressions in ordinary English (one `X`, three
occurrences, a `where` binder) are handled today by the AST itself: a
variable is the same leaf value at every occurrence, and the `where` clause
is a syntactic binder node, recognized but never resolved.

## Vertical slice (the exhibit)

Declarations sufficient for two sentences, the generated Rust, the parsed
values, and the byte accounting are recorded as the paper slice of
2026-08-13; the stage-2 ticket (`english-v2-vertical-slice`) implements it.
Compressed exhibit — the flagship value:

~~~rust
Ability::Triggered(Triggered {
    trigger: TriggerWord::Whenever,
    event: Clause::Event(EventClause {
        subject: NounPhrase::Common(Common { article: Article::A, head: Noun::Player }),
        predicate: VerbPhrase::Connive(Connive),
    }),
    effects: vec![Sentence::Declarative(Declarative {
        subject: NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::That, head: Noun::Creature }),
        predicate: VerbPhrase::DealDamage(DealDamage {
            amount: Amount::Var(VarAmount { var: Variable::X }),
            to: NounPhrase::Pronoun(PronounNp { word: Pronoun::It }),
        }),
    })],
})
~~~

renders to exactly `"Whenever a player connives, that creature deals X damage
to it."` — every byte attributed to a stored field, a form literal, or a
derived rule (inflection, capitalization position, punctuation attachment,
spacing), and the same `verb(..)` atoms render bare in imperatives. Working
the slice on paper surfaced the four derived-rule commitments listed in
§Bidirectionality; the byte-accounting table is the ADR's acceptance
demonstration that "exact inverse rendering" is a mechanical property, not an
aspiration.

## Verification gates and tooling

`cargo xtask english_v2 …` contracts:

- **`parse` / full-corpus gate** — exact parse over the Vintage-playable
  corpus; coverage is a ratchet, failures are enumerable.
- **`roundtrip`** — byte-exact both laws; `--require-clean` for the accepted
  set.
- **`coverage`** — the lexical coverage gate (§Terminals): byte-exact
  ownership partition of every accepted sentence, and owner of the coverage
  lock (an identity locks only when selected, byte-exact, and totally
  owned); `parse` keeps census duty only.
- **`catalogs generate`** — derives the complete canonical inventory from the
  local CR and `AtomicCards.json`, then replaces `data/gen/catalogs` with the
  exact deterministic line files.
- **`catalogs check`** — regenerates the canonical inventory in a temporary
  directory and compares exact contents against `data/gen/catalogs`, reporting
  changed, missing, and unexpected files without mutating the checked tree.
- **`catalogs text`** — regenerates the twelve compatibility files into the
  separate `data/gen/catalogs-legacy` cache. It never writes the canonical
  directory. These three commands are the `cargo xtask catalogs` family.
- **`ambiguity`** — census of selection decisions; unresolvable ties must be
  zero; exception-table entries enumerated for review.
- **`inspect` / `probe`** — per-sentence forest, selected construction, the
  decisive specificity comparison, and pruned alternatives (the losers stay
  visible in debug tooling).
- **`movers`** — old-versus-new difference reports during the parallel
  period; diagnostic only, never a preservation baseline.
- **Counted-list report** — mapping-layer uses, handwritten codecs, stored
  form tags, stored-spelling codecs, exception entries, morphology
  irregulars, terminal bindings, checked-constructor bindings, root
  declarations.
- Performance stays bounded (full-corpus parse remains a routine local
  command), measured once real grammar exists; no premature targets.

## Cutover plan

`english_v2` grows fully decoupled: no shims, no dual support, no
compatibility adapters, old crates untouched and still wired to consumers.
Cutover is **one event**: consumers switch, old crates deleted wholesale in
the same change, v2 takes the `deckmaste_english` name. **Trigger: consumer
parity** — v2 covers what `deckmaste_spelling` and xtask actually consume —
with the corpus ratchet continuing past cutover. The structural-recovery
effort on the old parser is halted (its remaining rounds would improve code
scheduled for deletion); its open tickets are reconciled separately. The 164
Idris `spelling:` annotations are out of this effort's scope — they prototype
the `semantics` shape and are handled separately.

## Implementation sequence

1. **`english-v2-catalog-pipeline`** (minted) — factor shared snapshot models
   into `deckmaste_data`; implement canonical extraction, inventory, line I/O,
   comparison, and the removable compatibility path in
   `deckmaste_catalogs`; point both English crates and xtask at the stable
   catalog layer; move migrations onto `deckmaste_data`; add the v2 loader and
   the `catalogs generate|check|text` gates.
2. **`english-v2-vertical-slice`** (minted) — hand-written golden for the
   AST, exact renderer, and total visitor. NO parser of any kind exists in
   this stage; parsing belongs exclusively to stage 3.
3. **`english-v2-earley-engine`** (minted) — the Earley-family chart engine
   over hand-written grammar tables for the slice corpus: english_v2's only
   parser, ever. No interim parser exists at any point in the sequence;
   ordered-choice/PEG/recursive-descent stopgaps are banned outright.
4. **`english-v2-declaration-compiler`** (minted) — the compiler MVP
   generates the stage-2 AST/renderer/visitor and the stage-3 grammar tables
   diff-identical. Settled mechanics (2026-08-15): comparison is
   **item-level** — the generated-item emission set, not whole files; runtime
   residue (context, catalog loading, checked-constructor bodies, scanners,
   morphology tables, codec value types) stays hand-written — and byte-exact
   after both sides normalize through `syn` parse + `prettyplease::unparse`.
   Declarations are inline function-like macro invocations; a macro never
   reads a file. Vocab enums and their variant→word render maps generate from
   day one; word→variant scanners and morphology join at stage 5. The stage
   **ends with the flip**: english_v2 consumes the macro, and the golden's
   generated items and the proof harness are deleted in the same ticket — no
   dual grammar authority survives the stage. `xtask english_v2 expand`
   renders the generated items for human review thereafter. The MVP
   declaration subset — settled after the stage-4 preplan audit found the
   original allow-list could not express the golden — is §Stage-4 MVP
   subset under §The declaration language.
5. Grammar buildout — style-guide-driven construction porting with the corpus
   ratchet and gates as the measure; selection pass and exception table grow
   with the first real ambiguity.
6. Consumer parity — the spelling seam's requirements implemented against
   v2; shadow running with `movers` reports.
7. Cutover and deletion, per the plan above.

No tickets beyond stages 1–3 are minted without explicit user approval.

## Prior-decision audit

- **English grammar is derived** — *reaffirmed in principle, amended in
  mechanism.* Reaffirmed: one declaration as sole authority for parse,
  render, and build; the layered stack; no parallel grammar authority; the
  hole-class taxonomy and typed-AST-substitution-over-splice intents (now
  day-one properties rather than retrofits); the exactness domain including
  stored house-style variation. Amended: the AST is generated from the
  declaration rather than bound by it (no constructor/destructurer adapters,
  lenses, or witness metadata — surface facts are ordinary stored fields);
  build validation is checked constructors from `require`, not adapter
  machinery. Superseded: declared construction-local dominance and cost
  tie-breakers — replaced by the computed-specificity selection pass with a
  counted exception table and hard-error ties. This document is the contract
  for v2; the old document remains the record of the legacy implementation
  until cutover deletes it.
- **English productions ship their inverse** — *reaffirmed and strengthened*:
  the inverse is now compiler-enforced statically, not shipped per slice.
- **English clauses are structural** / **English coordination is structural**
  — *reaffirmed*; coordination as a shared structural node is precisely the
  one genuine convergence the measurement found.
- **Semantics, spelling, lowering** — *reaffirmed*; `spelling` owns macro,
  RON, discourse, and semantic concerns; english_v2 owns English grammar and
  its AST; the English AST is authoritative for surface structure only.
- **Macro templates are bidirectional** — *reaffirmed*; frames-as-functions
  is its natural v2 form.

## Construct salvage judgments

Per the design ticket's explicit list: **retained** — typed categories,
sequence invariants (as `require`), feature combinators (slice-first
vocabulary), inverse forms (`form`/`when`/`otherwise`, statically checked).
**Rejected** — lenses, bind adapters, projection metadata, evidence/witness
metadata, recovery nodes (all bridge machinery for a bound AST), declared
dominance (replaced by computed specificity), Serde as tree-walking (Serde
only at a real serialization boundary, of which the slice has none). Ordinary
typed traversal is the generated visitor. The full mechanism-level salvage
ledger (ideas kept, failure modes recorded, with citations into the legacy
code) is the appendix below.

## Non-goals

Carried forward: no GF toolchain; no second grammar authority; no
probabilistic or learned ranking; no HPSG/DAG engine; no binder or discourse
resolution in the parser. New: no rules-legality judgment in acceptance; no
recovery representation in the canonical AST; no compatibility bridge to the
legacy implementation at any point; no authored pairwise selection policy.

## Appendix: salvage ledger

Recorded 2026-08-14, before wholesale deletion of `deckmaste_english`, the
render/compile side of `deckmaste_spelling`, and
`deckmaste_construction_compiler`. Cites were verified against the tree at
recording time and will dangle after cutover deletion — they are the
historical record, not live pointers.

### Part A — ideas carried into the rewrite

**A1. Earley chart core: two-kind RHS positions.**
`Expected<N, L>` splits every rule position into `Nonterminal(N)` or
`Lexical(L)` — `deckmaste_english/src/chart.rs:34-38`, consumed by the single
dispatch at `:634-638` (`predict_and_advance` vs `scan_and_advance`).
Keep: one enum makes prediction and scanning the *only* two cases, and the
dispatch is exhaustive by construction rather than by convention.

**A2. Scan as injection, not as a token table.**
`LexicalMatch<F, M> { end, features, meaning, local_cost }` — `chart.rs:48-54`,
produced by the grammar's own `scan` (`:91-96`) and consumed at `:738-752`.
Keep: the lexicon is a *callback returning spans*, so multi-token and
ambiguous lexical items fall out for free; `end` (not "length 1") is why.

**A3. Seeding is a policy knob, not hardcoded.**
`fn seed` branches on `O::ALL_RULE_SEEDS` — `chart.rs:584-600`.
Keep: all-positions seeding for diagnostics vs start-symbol seeding for
production is one const, not a second parser.

**A4. Generated total traversal as a codegen contract.**
`trait LinearizationVisitor` —
`deckmaste_construction_compiler/src/runtime.rs:320`, with typed callbacks
`subtree<T>` (`:362`), `scalar<T>` (`:443`), `identity<T>` (`:472`) and
role/codec-projected twins (`:377`, `:455`, `:488`).
Keep: the *generator* guarantees totality over a declaration, and consumers
pick their level (concrete Rust type vs stable declared role). Precedent for
visitor-generation as the codegen deliverable.

**A5. Hole taxonomy derived from real frames, not invented.**
`enum HoleClass { Subtree, Numeral, PtHalf, SelfRef }` —
`deckmaste_spelling/src/compile.rs:77-99`, with its own doc stating the
distinctions are relational, not taxonomic (`:70-75`).
Keep: the requirement catalogue for the deferred hole feature —
whole-subtree, bare numeral (notation stays outside the hole), one half of a
P/T pair below phrase level, and self-reference.

**A6. Typed-hole vocabulary.**
`enum FieldKind` — `deckmaste_construction_compiler/src/model.rs:398-475`:
`Unit`, `TupleProduct`, `StructProduct`, `Identity`, `Subtree`, `Sum`,
`Product`, `Optional` — with `opt opt` made unrepresentable at parse
(`:470-471`).
Keep: an eight-way field vocabulary covering the whole declared surface, and
the "nesting rejected at parse, not at validate" trick.

**A7. Structural self-reference already worked.**
`SELF_WITNESS` substitution makes the parser build a
`NounPhraseKind::ThisCard` node (`deckmaste_spelling/src/witness.rs:64-70`),
and `HoleClass::SelfRef` is located *structurally* by that projection, not by
witness spelling (`compile.rs:95-98`). Multi-site holes are already modelled:
the hole keeps one `path` and consumers scan the tree for every hole site
(`compile.rs:126-135`).
Keep: proof that structural location dissolves the spelling-location
compensations, and that multi-site holes need a scan, not a path.

**A8. A terminal standing for an arbitrarily large constituent.**
`MeaningKey::QuotedAbility(Span)` —
`deckmaste_english/src/grammar/mod.rs:2056-2059`: the quoted ability's
interior is carried as a hashable `Span` and reparsed at lowering.
Keep: a legitimate way to make an unbounded constituent behave as a chart
terminal — defer the recursion to lowering, carry only a locator.

**A9. Identity providers for contextual lexical identity.**
`form: identity ThisCardForm via ThisCard` / `via FullThisCard` —
`deckmaste_english/src/constructions/noun_phrase.rs:1114-1130`; same pattern
for pronoun cases at `:1077-1101`.
Keep: one value type, several *providers*, where the provider names the
context licensing the spelling — a value shared without collapsing the
contexts that produce it.

### Part B — failure modes remembered

**B1. Projection is a one-way seam; nothing comes back in.**
`project_fragment` (`deckmaste_english/src/projection.rs:55`) is the only
projection entry point; its output is consumed (spelling's unify, compile,
render), but there is no inbound arrow — per the render module's own
admission, "the construction projection is intentionally transient and
one-way. There is no way back from a patched projection to the typed
`Fragment`" (`deckmaste_spelling/src/render.rs:14-18`).
Remember: a missing inbound constructor is what every compensation below
pays for.

**B2. Splice-and-reparse rendering, admitted in the module doc.**
`deckmaste_spelling/src/render.rs:9-51` — substitution is textual into the
frame's authored sigil string, the assembled string is reparsed (`:213`), and
the reparse is compared back against the frame tree, failing as
`ReassembledDifferently` (`:107`, `:227-229`).
Remember: the round-trip check exists precisely because spliced text cannot
guarantee it re-brackets into the frame it came from.

**B3. Compensation cascade downstream of B2.**
Synthetic spelling-located holes (reserved `zz` stems, `witness.rs:62`,
`:170-181`; six reserved two-digit primes as numeric witnesses, `:72-76`);
the hole-linearity ban ("each param on exactly one constituent",
`compile.rs:476-480`) — an authoring restriction imposed by the substitution
mechanism; the text-offset ordinal bijection (`MarkerOccurrence`,
`compile.rs:355-363`, `:545-560`) guarded by
`is_within_grouped_arabic_numeral` (`:371`); and the matcher skipping witness
comparison for hole-bearing patterns
(`deckmaste_spelling/src/unify.rs:1085-1088`).
Remember: four independent mechanisms, each sound on its own, all paying for
B1.

**B4. Failure silently becoming a successful AST.**
`RecoveredText` (`deckmaste_english/src/syntax/phrase.rs:34-37`) sits inside
the canonical AST at three ability-layer roles (`syntax/ability.rs:298`,
`:634`, `:833`) and `syntax/phrase.rs:2156`; the design is stated at
`syntax/mod.rs:89-93` — the ability layer is total, so an unparsed run
becomes a node rather than an error.
Remember: totality bought by making failure a node means every consumer must
re-derive "did this actually parse", and most don't.

**B5. Adapter smear.**
203 of the 242 construction declarations bind through a ctor/destructurer
pair (`bind T via make_x, x_parts`) rather than the target's own fields; the
cause is sealed newtypes hiding one-variant-per-construction enums
(`Quantity(QuantityKind)`, `syntax/phrase.rs:301`, `:334-350`). The compiler's
own doc comments name the surface as adapters
(`deckmaste_construction_compiler/src/model.rs:237-239`, `:298-303`,
`:39-41`).
Remember: when the near-universal case needs an adapter, the adapter is the
real interface and the declared one is decoration.

**B6. Closed-set ceilings, hit twice independently.**
Five fragment categories in the parser (`deckmaste_english/src/fragment.rs:71-89`,
`:111-122`) and the same five declared separately as `FrameKind` in
`macro_ron/src/frames.rs:266-272`, joined by a hand-written 1:1 map
(`deckmaste_spelling/src/lexicon.rs:459-463`) plus two more exhaustive lists
(`:393-397`, `:408-412`).
Remember: two layers independently arriving at the same closed set, then
needing a translation table between them, is the smell — the set should have
one owner or none.

**B7. Monolithic hand-written rendering.**
`deckmaste_english/src/renderer.rs` — 8,133 lines, with per-identity dispatch
on stringly-typed provider names and `downcast_ref` + `expect` (`:1502-1505`,
`:1995-1998`).
Remember: what hand-written rendering grows into when the generator does not
own it, and why A4 has to cover rendering too.
