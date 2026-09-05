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
                         immutable declaration/provider environment

deckmaste_data -> deckmaste_catalogs -> {deckmaste_english, xtask}
deckmaste_migrations -> deckmaste_data (temporary)

deckmaste_english_v2 -> macro_ron -> deckmaste_features
                                current cutover debt; edge must be shed
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
live in the stable low-level `deckmaste_data` crate. The old
`deckmaste_features` crate is v1 grammatical-feature vocabulary and is deleted
at cutover together with the old English/compiler/macro machinery; Plan 07
adds no new or direct dependency on it, and v2's dependency set is unchanged.
Plan 08 likewise adds no crate edge and does not revive
`deckmaste_features` or v1 feature vocabulary.
The current `deckmaste_english_v2 -> macro_ron -> deckmaste_features` edge is
cutover debt: the surviving `macro_ron`/v2 normalization path must shed that
transitive dependency before `deckmaste_features` is deleted. The stable
`deckmaste_catalogs` crate owns extraction, the canonical inventory, line-file
I/O, directory comparison, and a removable legacy adapter. Legacy English
consumes it directly; xtask is the sole adapter that turns a named catalog
source into frozen typed provider rows for v2. Parser consumers never load or
depend directly on `deckmaste_catalogs`. `deckmaste_migrations` now uses
`deckmaste_data` temporarily for its surviving extraction work instead of
owning the shared models.

## The declaration language

The vocabulary is the reduced grammatical surface from the 2026-08-12 DSL
classification, with these deltas:

- **Kept:** `construction ID: Category`, named role fields, named `element`
  products, `lex`, `identity`, `opt`, `seq`, abstract sum/product, `require`
  predicates, construction-wide and role-level `derive` feature flow,
  `form`/`when`/`otherwise`, and the literal / role / `lex(..)` /
  `identity(..)` form atoms.

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
Nominal coordination uses the same positional sequence algebra with a
minimum of two members. Separate semantic constructions represent `and`,
`or`, and `and/or`; arity and edge position derive pair, first, middle, and
last separator surfaces. The AST stores members and the semantic conjunction,
never commas, spaces, coordinator spelling, or another separator value.
Nominal-core coordination under one selector and coordination of complete noun
phrases are distinct generated categories, so their scope and derived number
cannot be silently rebracketed.

`target` is an English determiner, not a rules-semantic noun-phrase category.
The generated English AST therefore records its surface syntax through the
generic `DeterminerPhrase` category; whether that phrase denotes a targeted
game object is established only by a later rules projection. An unquantified
`target` determiner selects a singular nominal and derives singular number and
agreement. Plural target-determiner syntax remains available only beneath an
explicit higher quantifier (`two`, `X`, `up to three`, `any number of`, and
their declared peers), and cannot enter the bare-reference path. Complete
noun-phrase coordination sequences generic determiner phrases, preserving each
repeated determiner and each singular member. These facts do not depend on noun
spelling, invariant plural morphology, or declaration-specific lexical-number
exceptions.

Feature equations may target the construction or a named role. Role-level
constants and from-role equations let an attributive noun be singular while
its head inherits the enclosing phrase's number; each feature-consuming atom
must resolve exactly one provider. A finite form guard may read a sealed finite
derived feature only when the compiler can enumerate its domain and include it
in the same exhaustive/disjoint guard proof as stored features. It is not a
callback or an open predicate.

**Plan 08 homogeneous-sequence amendment (2026-08-24):** an `Agreement`
equation may target every member of a sequence (`derive members.agreement =
...`) or relay one homogeneous member value outward (`derive agreement =
members.agreement`). This is available only for a statically nonempty sequence
of direct feature-bearing category values. Generated scanning, build,
checked-construction, rendering, and feature helpers apply or compare the
feature across every member, including middle and final positions. The
compiler carries one transient feature beside its private sequence and
abstract-sum build carriers; it adds no public AST feature vector, wrapper,
tag, or spelling field. Possibly empty sequences, lexical or identity items,
categories without the feature, unsupported feature domains, and mixed
sequence-feature reads are declaration errors. Contextual categories whose
feature is chosen only by an enclosing construction remain feature-free in the
public AST; their generated parser carriers prove homogeneous realized input,
and their renderer receives the enclosing feature once and applies it to all
members. When an explicit sum mixes intrinsic and contextual Agreement
alternatives, a generated constraint helper treats contextual alternatives as
accepting the supplied Agreement and checks intrinsic alternatives against it.
Constraint authority and its source are classified per construction variant:
the category helper only dispatches those exact variant results, so an
intrinsic sibling cannot inherit a contextual acceptance arm and an unrelated
checked sibling cannot inherit a relay source requirement. Homogeneous outward
relay preserves that constraint through materialization and requires every
enclosing Agreement writer to satisfy it, without storing the chosen feature
in the public AST.

**Plan 08 homogeneous-Number amendment (2026-09-04):** by coordinator ruling,
grammatical `Number` has the same homogeneous-sequence propagation and the same
declaration-error discipline as `Agreement`: `derive members.number = ...`
imposes one Number on every member, while `derive number = members.number`
relays one homogeneous Number outward. Number remains an alternative to
Agreement, first-member Onset, and last-member possessive ending for a sequence;
mixed sequence-feature reads remain declaration errors.

**Plan 08 per-feature sequence amendment (2026-09-04, second amendment):** by
coordinator ruling, “alternative” and “mixed” above refer to contending
equations for the same feature on one sequence role. Distinct features on that
role are permitted when each has exactly one equation, so homogeneous Number
may relay beside first-Conjunct Onset and last-member possessive ending. The
compiler's private sequence carrier therefore holds one transient value per
feature; a second equation contending for the same feature remains a
declaration error.

**Plan 08 unified-membership clarification (2026-09-04):** re-spelling former
Category membership as a `nominal_form` requirement is the unified shape's
replacement for the deleted Category, not a narrowing.

Agreement-constrained construction roles use the same checked-public-boundary
policy as `require`: the constrained role and every stored role used to derive
its expected Agreement are private, with generated copy or borrow accessors.
This prevents a consumer from constructing a valid product and then mutating a
dependency so that the stored relation no longer holds. Abstract products may
store an Agreement-bearing sum only when every reachable alternative supplies
intrinsic Agreement. Their generated renderer derives the selected value's
Agreement independently for a required field, a present optional field, and
each sequence member. A sum with any contextual alternative instead requires
an external Agreement writer; because abstract products have no feature-writer
syntax, each of those three field shapes is rejected at the authored product
role rather than deferred to code generation.

`Onset::{Consonant,Vowel}` is a v2-owned sealed compiler feature, emitted and
carried through generated feature plans exactly like `Number`. It is a
property of each realized terminal surface. The v2 normalization path owns a
bounded pronunciation recipe and accepts an explicit optional per-form
override authored as attested stub data. The first-character helper is only
that recipe's orthographic fallback, never onset authority. Normalized rows
freeze effective onset as data; a spelling the recipe cannot classify is a
normalization error unless that realized form has an authored, attested
override. Wrappers forward the first actually realized child's onset.
Indefinite articles are exhaustive guarded forms over that derived value (`an`
for `Vowel`, `a` otherwise); article choice is not stored.

The narrow adjacency atoms `prefix(fixed, value)` and
`suffix(value, fixed)` contain exactly one nonempty, whitespace-free fixed
affix and one ordinary value atom. They suppress only their internal word
boundary and preserve adjacent, disjoint lexical claims. Nesting, callbacks,
alternatives, repetition, and two value atoms are rejected. A
`prefix(fixed, value)` atom gets its onset from the fixed realized prefix when
that prefix has a lexical onset, so every `non...`/`non-...` form is
consonantal rather than inheriting the value's onset. A punctuation-only
prefix such as `+`, U+2212, or `2/` is adjacent structural spelling: it need
not have a lexical onset, and the complete realized form forwards the value's
onset. A construction that requires onset still rejects when neither affix nor
value provides one. The sealed `PossessiveEnding::{EndsInS,Other}` feature derives from the
last actually realized possessor surface. English possessives have exactly
three guarded forms: singular takes `'s`; plural `EndsInS` takes `'`; the
plural remainder takes `'s`. Neither affix punctuation nor a possessive form
tag is stored.

**Plan 08 declaration extensions (2026-08-23):** `unsigned_decimal` is the
canonical nonzero unsigned decimal generated terminal recipe. The declaration
states its representation bound; the compiler owns the canonical digit
surface, scanner, renderer, and round-trip evidence. `circumfix(prefix,
value, suffix)` is one fixed two-sided boundary around one required declared
role. That role may be a singular category or terminal, or a sequence whose
own separator and terminator remain authoritative. The circumfix bytes are
derived and separately owned. Nesting, callbacks, optional roles,
whitespace-bearing affixes, and literal-only payloads are rejected. A
standalone-root renderer's private dispatch helper uses the compiler-reserved
`__deckmaste_construction_internal_render_root_` namespace, which authored
declaration identifiers cannot enter. This keeps a generated `Ability` root
and generated `AbilityBody` category distinct without aliases or a public/API
compatibility surface. Render capability sealing follows every existing role
wrapper to its category leaf, so an `opt Category` receives the same parse
context as a direct category while `None` remains a no-byte branch. An empty
`abstract sum` is a valid generated uninhabited public enum: it has no
sentinel, tag, alias, or surface, and is useful as an optional structural slot
that a later finite grammar extension may populate. An optional `vocab` role
used by a finite form guard has the sealed domain `{Absent}` plus every
declared present vocabulary variant. Membership accepts only matching present
variants; `.is_some()` and `.is_none()` select presence and absence across the
same domain. The same presence predicates are available to `require`, where
they become checked-constructor invariants and may compose with present-value
membership. This changes no public storage: the field remains `Option<Vocab>`,
with no sentinel, form tag, or arbitrary Rust predicate.

The fixed-surface annotation `sentence_initial(surface)` applies only to an
exact form literal or sequence separator. It emits no AST field or form tag;
the annotated lexical owner and the existing structural-separator owner both
apply `StructuralTransition::SentenceInitial` through the same scanner and
renderer state transition. This keeps an activation colon-space distinct from
an identical continuation literal, and an activation-cost comma-space distinct
from other comma-space separators, without category-name or punctuation-wide
inference. Empty, nested, duplicated, nonliteral form targets and sequence
terminators are rejected. The English lexical boundary inventory admits the
exact `:`, `]`, and `}` delimiters required by these form and circumfix
surfaces, without treating arbitrary punctuation as a word boundary.

The exact form annotation `structural(surface)` owns a nonempty, unnested
literal through the same structural scanner and renderer path while applying
`StructuralTransition::Preserve`. It exists for a constituent-owned boundary
whose following lexical item must retain the current case position; it emits no
AST field or form tag and names no English vocabulary item.

Fixed surfaces carry one sealed case transition: `Preserve`,
`SentenceInitial`, or `Continuation`; the states cannot be combined. The
annotation `continuation(surface)` is the sequence-separator counterpart to
`sentence_initial(surface)`: after consuming one exact nonempty, unnested
fixed separator, scanner and renderer both pass `CasePosition::Continuation`
to the following member. The construction compiler validates that structural
placement without knowing any English word or punctuation. English v2's sole
production use is `continuation(" Then ")`: the preceding member's terminator
retains ownership of the sentence-ending period, the separator owns the exact
ASCII connective surface, and the following predicate begins in continuation
case. The annotation emits no AST field, form tag, connective spelling, parser
branch, or runtime case escape. Form atoms, sequence terminators, empty
surfaces, and nested or duplicated transition annotations are rejected.

An `abstract sum` may be the sole generated authority for a construction
category with the same name when its alternatives map every construction
element type in that category exactly once. The alternative name authors the
public enum variant independently of the element type, so `Clause:
CostClause` produces `Category::Clause(CostClause)`. The construction forms
still build and render their generated element products; the sum contributes
the one public category enum and the one wrapping rule family. No inferred
second enum, wrapper, stored tag, duplicate rule identifier, or duplicate
render/walk/visitor family is emitted. A missing category member, duplicate
mapping, foreign element type, or ordinary name collision outside this exact
ownership relation is a declaration error.

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
  vocab-value narrowing in `build` (`WithWhere.clause` accepts
  `Clause::Where` only; `CountNp.controller` accepts `Pronoun::You` only).
  The historical `Triggered.event` / `Clause::Event` example is superseded;
  the current finite trigger branch is `TriggerPrefix::Finite(FiniteClause)`.
  Every other `require` form remains a named hard error.
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
- `require` clauses and Agreement-constrained roles become checked
  constructors. Each constrained field and every stored field that derives
  its expected Agreement is private with a generated accessor; fields outside
  those invariant dependencies stay public.
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
4. **Catalog sources** — open-class inventories are regenerated as sorted,
   deduplicated plain-text word lists (one entry per line, no headers). Parser
   consumers never load those files. Where a construction declares a
   `catalog_identity`, xtask's named adapter converts the requested immutable
   inventory into typed provider rows before parser construction. The
   canonical `data/gen/catalogs` inventory is exactly:

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
morphology, structured signed-decimal, canonical `english_cardinal`, canonical
`unsigned_decimal`, terminal choice, context identity, and
`catalog_identity`), never by declaration-bound arbitrary callbacks and never
by a general scanner/renderer combinator language. `english_cardinal` is an
algorithmic English-number grammar over a declared unsigned magnitude;
`unsigned_decimal` is the canonical digit grammar for unsigned scalars; and
`catalog_identity` carries a canonical identity from one named immutable typed
provider. None is a corpus word list. Each recipe derives both
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
not escape hatches, and no dual terminal authority survives. A recipe accepts
its **full structural domain** — every surface its shape admits
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

**Plan 09 participle amendment (2026-08-25):** `Participle` is one sealed,
one-member morphology axis generated by the compiler-owned
`english_participle` recipe. The regular recipe is exactly append-`ed`; an
attested whole-form exception is an explicit per-lexeme `Participle` override.
A `declaration_verb` may select this axis only with a closed lexeme using the
same morphology. Its scanner filters open declaration readings by the exact
participle feature and exact declared valence, and its renderer immediately
consumes the fixed axis. Neither the public AST nor authored constructions
store a voice, tense, form tag, auxiliary sequence, or spelling. Agreement and
Participle axes are reciprocally rejected at the codec boundary.

**Plan 09 declaration-language and grammar-boundary amendment
(2026-08-26):** English-v2 produces a linguistic AST. It owns English syntax,
morphology, agreement, constituency, attachment, and lexical
subcategorization. A separate downstream layer owns game-semantic
interpretation. Oracle declarations may contribute open lexical vocabulary
and linguistically normalized valence; corpus examples are evidence for an
English construction, never authority for a game-semantic category or a
card-specific grammar island. Printed notation and document segmentation are
the only game-specific surfaces admitted directly by the grammar.

This amendment supersedes Plan 09 Task 2's three-atom declaration-tail
contract. `PredicativeComplement` is a fourth sealed tail atom, allowing a
verb declaration to select a resultative complement; the formerly
unrestricted any-verb resultative attachment is deleted. Repeated atom
categories require compiler-only positional labels, which are erased before
normalized frame-key matching and never enter generated AST or runtime data.
Compositional preposition phrases are shared constituents; only genuinely
verb-selected prepositions remain flat lexical subcategorization frames.

The shared predicate algebra separates a nonrecursive base verb frame from
typed adjunct layers. Coordination relays agreement into its members while
exposing the same contextual agreement outward. Finite auxiliaries select
bare complements; passive predicates compose an auxiliary/copula with a
declared participle and do not store voice, tense, or form tags. Relative
clauses express subject and object gaps as ordinary syntactic dependencies,
including finite, modal, copular, and passive subject-gap forms. Ordinary
noun phrases supply partitives, possessives, locatives, modifiers, and
predicative nominals rather than resource-, zone-, controller-, or
card-kind-specific categories.

**Target-as-noun amendment (2026-08-27):** `target` remains a determiner.
This supersedes the 2026-08-26 sentence that made admitting productive noun
uses conditional on the `common_noun_modifier` guard. Admitting *the target
of* and *new targets* is a lexicon change, not a grammar change: pin one
`Determinative` lexical category and one `Det + Nominal` frame — with the
multiword quantifying determiners as a second frame over the same nominal —
then add an ordinary `CommonNoun::Target` entry and let the noun uses fall out
of the frames. A singular count nominal in argument position already requires
a determiner, so the bare-compound rival parse is excluded by machinery the
grammar already has, and the guard's stated justification (preventing a
targeted-NP *semantic* category) belongs to the downstream layer this
amendment set assigns it to. A general rule never names a specific lexeme: if
the corpus produces a genuine tie, the residual restriction is declaration or
lexeme metadata — a compoundability feature on the noun entry, checked
generically by the compounding rule — never a `require` naming `target` or any
other word. Damage-, distribution-, and counter-specific escape hatches remain
forbidden.

Declaration-backed noun recipes have explicit, category-safe declaration-kind
domains and, for subtypes, exact subtype-family domains. More than one such
terminal may exist only under distinct generated value types, and a
`noun(role)` atom resolves its terminal statically from the role type rather
than scanning a catch-all union. Its public value stores declaration identity
only. The private parse leaf may retain the realized surface feature long
enough to check it against the role's derived `Number`; rendering then asks
the immutable environment for identity plus role number. Realized noun number
is never public AST state.

Supertypes remain generated closed vocabulary, not declaration nouns or
catalog identities. Oracle text that names its own source denotes that
particular object, including when it uses an approved shortened printed name
[CR#201.5c]. Every nonempty full face name is opaque context identity: the
parser neither structurally parses it nor validates its punctuation or word
onset. A distinct abbreviated arm exists only when authoritative per-face
MTGJSON metadata says `Legendary`; its spelling follows the established
families in order: first take the prefix before a comma (so `The Balrog,
Durin's Bane` shortens to `The Balrog`); only when there is no comma does a
leading `The ` block shortening; otherwise remove a canonical trailing Roman
numeral, take the prefix before the earliest ` the ` or ` of ` epithet, or
take the first word. Equal or empty results do not create an abbreviated arm.
The current Comprehensive Rules and exact Oracle text are correctness
authority for this behavior. English v1 contributes the shortening mechanism
only; it is neither a compatibility target nor an acceptance authority. The
grammar represents source self-reference as a bare per-parse-context identity
and stores only its full/abbreviated spelling choice; the name, shortening,
legendary flag, and separately normalized realization onset remain context
facts, never AST state. An independently mentioned card-name `catalog_identity` is admitted
only inside an explicit name-bearing construction such as `a card named Seven
Dwarves`; it is never a rival bare noun phrase for the current card's name.

**Plan 08 cross-envelope obligation (2026-08-23):** the dedicated legendary
self-reference ruling applies in every ability envelope. The only shortening
license is exact per-face Oracle `Legendary` metadata supplied by xtask;
punctuation or a name such as `+2 Mace` never licenses abbreviation.

Catalog providers do not weaken the normalized grammar boundary. Generated
metadata names each required typed provider, and construction fails on a
missing or duplicate provider. xtask alone reads `deckmaste_catalogs` and
adapts the requested canonical identities into frozen provider rows;
construction-core and every v2 scanner, parser, renderer, and environment
consumer read only those rows and generated lookup metadata. For card-name
rows, the adapter derives the exact catalog surfaces that the bounded onset
recipe cannot classify and requires two-way equality with a closed, reviewed
per-surface onset-override inventory. A new unreviewed surface and a stale
override both fail loudly; Unicode class, punctuation prefix, and other
spelling-wide fallbacks are not onset authority.

**Onset-override inventory scope amendment (2026-09-04):** the two-way
onset-override closure applies to both authored override inventories:
card-name rows and flavor-word rows.

**Lexical coverage gate:** every token of every accepted corpus sentence must
be claimed by a form literal, vocab, lexeme, codec, or identity — an
unclaimed token fails at the lexical layer, loudly.

## Plan 08 ability-and-logic grammar boundary

`Ability` envelopes share one public `AbilityBody`; triggered and activated
envelopes differ only in their envelope-specific structure around that body.
Triggered envelopes stage finite and temporal trigger complements separately.
An intervening `if` is not an ordinary condition: it is part of the triggered
envelope, while ordinary condition attachment is staged as a structural
operator. Cost surfaces are linguistic and v2-owned. Auxiliaries,
coordination, condition attachment, and `then` are structural operators, not
ad hoc literal strings or v1 feature vocabulary. This condition/coordination
staging keeps their AST and ownership decisions before Plan 08 effect grammar.

Plan 08 owns ordinary unlabelled plain modal U+2022 lists, moved here from
Plan 10. Its public data boundary uses this per-boundary assignment.

### Per-boundary structural byte and case ownership

`CasePosition::DocumentInitial` and `CasePosition::SentenceInitial` capitalize
the following lexical word; `CasePosition::Continuation` preserves its running
case. Each structural owner below claims every listed structural byte,
including its ASCII space where present, before passing the stated position to
the following node. The explicit word-prefix row names the lexical owner when
that prefix instead owns the ASCII space.

| Boundary bytes | Structural owner | Following `CasePosition` / capitalization |
| --- | --- | --- |
| Sentence period (`.`) | `Sentence` | Sets `CasePosition::SentenceInitial`; the next lexical word is capitalized. |
| Sentence-final quoted block (`.` then `"`) | The nested quoted `Sentence` owns `.`; the `QuotedAbility` closing affix owns `"` | The quoted interior's period also discharges the enclosing sentence terminator; no second period follows the closing quote. |
| Intersentence ASCII space | `SentenceSequence` | Preserves `CasePosition::SentenceInitial`; the following sentence's first lexical word remains capitalized. |
| Document LF | `OracleText`'s document-block sequence | Sets `CasePosition::DocumentInitial`; the next document block's first lexical word is capitalized. |
| Trigger → body comma | `Triggered` envelope | Sets `CasePosition::Continuation`; the body begins without capitalization. |
| Trigger → intervening-if comma | `Triggered` envelope | Sets `CasePosition::Continuation`; `if` begins without capitalization. |
| Intervening-if → body comma | `FiniteCondition` | Sets `CasePosition::Continuation`; the body begins without capitalization. |
| Trigger or condition → following ASCII space + word prefix | The immediately following lexical or form-literal word-prefix claim | Preserves `CasePosition::Continuation`; that one claim owns both the ASCII space and its word. |
| Cost-component comma-space | `ActivationCost` sequence | Sets `CasePosition::SentenceInitial`; the next cost component's first lexical word is capitalized. |
| Activation colon-space | `Activated` envelope | Sets `CasePosition::SentenceInitial`; the `AbilityBody`'s first lexical word is capitalized. |
| Modal header ASCII-space + U+2014 + LF | `Modal` header | Sets `CasePosition::SentenceInitial`; the first mode body is capitalized after its bullet. |
| Mode U+2022 + ASCII-space | `ModalMode` | Preserves `CasePosition::SentenceInitial`; the mode body's first lexical word is capitalized. |
| Intermode LF after preceding final period | `Modal` mode sequence | The preceding `Sentence` has set `CasePosition::SentenceInitial`; the next `ModalMode` preserves it through its bullet, so its body is capitalized. |

Every listed structural or lexical owner claims its assigned bytes. Trigger
and condition comma claims end at the comma; the immediately following
word-prefix claim owns the separator space and word. Claims cannot overlap.
Keyword-, ability-word-, reminder-, frame-coupled, labelled,
pawprint-weighted, repetition, and other advanced modal forms remain Plan 10.

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
  can break produces a typed runtime hard error naming both constructions, in
  production and development alike. During grammar implementation that same
  tie is also a hard design boundary: the worker stops and reports the complete
  ambiguity census, competing ASTs, and specificity evidence, and resolution
  requires an explicit design ruling. A worker must never silently restructure
  or narrow the grammar, add dominance, or add an exception merely to erase
  the tie. Policy lives in one derived rule plus one counted list, never
  smeared across declarations.
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
nodes), noun number (from selector content and role-level equations), article
choice (from the first realized child's effective onset), possessive suffix
(from number plus the last realized possessor surface), capitalization
(positional: ability-initial and after terminal periods — not after the trigger
comma), whitespace and bound-affix adjacency, coordination separators, and
numeral spelling are all computed at render. Declaration-noun values store
identity but not realized number; normalized terminal rows freeze effective
onset and other terminal realization facts outside the AST. Stored: only
non-derivable surface facts — and the measured corpus
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
An independently mentioned exact card identity is different: it is stored as
the value of an explicit name-bearing construction and rendered through its
frozen typed provider, never inferred as source self-reference.

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
Compressed exhibit — the current production shape:

~~~text
Triggered
  trigger = TriggerPrefix::Finite
    clause = FiniteClause
      subject + Predicate
  body = AbilityBody
~~~

`EventClause`, its `event` role, and `Clause::Event` are superseded production
API, not usable historical authority.

The finite witness renders to exactly `"Whenever a player connives, you gain X
life."` — every byte is attributed to a stored field, a form literal, or a derived rule
(inflection, capitalization position, punctuation attachment, spacing), and
the same `verb(..)` atoms render bare in imperatives. Working the slice on
paper surfaced the four derived-rule commitments listed in §Bidirectionality;
the byte-accounting table is the ADR's acceptance demonstration that "exact
inverse rendering" is a mechanical property, not an aspiration.

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
   `deckmaste_catalogs`; point legacy English and xtask at the stable catalog
   layer; move migrations onto `deckmaste_data`; have xtask alone adapt the
   required canonical identities into frozen typed provider rows for v2; give
   v2 an immutable declaration/provider environment, never a catalog loader or
   direct catalog dependency; and add the `catalogs generate|check|text` gates.
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
   residue (context, immutable declaration/provider-environment loading,
   checked-constructor bodies, scanners, morphology tables, codec value types)
   stays hand-written — and byte-exact after both sides normalize through
   `syn` parse + `prettyplease::unparse`.
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

New roadmap scope needs explicit approval; completion tickets and re-minted
remainder tickets within an approved stage are evidence, not expanded scope;
integrate one plan before claiming the next.

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


## Ruling: frame literals may not be lexicon surfaces (2026-09-02)

Two independent gaming audits found ~40% of selected units routing
through verb-specific frames whose tail literals were nouns ("damage",
"life", "mana of any", "card", "counter"), so those phrases were never
noun phrases — coverage bought one level below sentence memorization,
invisible to the byte-exact laws and the unit-count ratchet. Ruling: a
`Literal` tail atom or form literal whose surface equals a declared noun
or verb surface is a compile-time load error. Literals are for particles,
prepositions, punctuation, and closed grammatical words; content words
enter only through lexemes and declarations. The gate census reports a
structural-depth or literal/lexicon-collision metric beside unit counts.

### One noun inventory closes the content-word escape hatch

English v2 has one noun inventory and one `Noun` codec. Its closed inventory
is the union of the core `CommonNoun` seed and every noun grammar row
contributed by the `Type`, `Subtype`, and `TurnPart` declaration kinds.
Contributor kind and declaration identity remain provenance on the value;
they do not create parallel noun terminal categories. The construction
compiler therefore rejects a second declaration-noun provider instead of
letting multiple typed noun inventories coexist.

Turn-part names are declaration data, not core vocabulary. The builtin
`TurnPart` rows cover the CR-closed phase and step inventory: the five phases
[CR#500.1], beginning-phase steps [CR#501.1], combat-phase steps [CR#506.1],
and ending-phase steps [CR#512.1], together with the Oracle shorthand noun
surfaces that name those parts. They contribute through the same `Noun` codec
as every type, subtype, and core noun.

Number is the noun recipe's feature axis. Regular plurals are derived; an
attested exception is a per-word override, never a separately authored plural
inventory. Grammar may distinguish noun behavior only through declared
grammatical features such as countability, properness, compoundability, and
relationality. Topic or semantic classes such as “temporal” and “object” are
downstream concerns and may not become terminal categories.

This aggregation closes the remaining literal escape hatch: a content noun
has exactly one legal home in the noun inventory, so the frame-literal
collision tripwire detects evasion rather than compensating for an absent
provider path.


## Ruling: one noun inventory, multiple contributors (2026-09-02)

Root cause of the noun-as-literal class: the Type+Subtype noun-domain
ruling was hardened by the compiler into a single noun lexeme provider,
leaving ordinary English nouns (damage, life, card, counter, beginning,
end, mana) no legal home as nouns — literals and vocab were the only
doors. Ruling, mirroring the verb inventory: ONE noun inventory with
provenance as data — core-declared ordinary nouns as the seed,
Type/Subtype declarations as today, and step/phase names as a
declaration kind (a CR-closed set) contributing like Type/Subtype.
Grammatical distinctions (count/mass, relational, proper) are declared
features consumed by constructions; topical noun categories (a
`TemporalNoun`) are not admitted. Number remains recipe-derived. The
compiler's single-provider assumption is removed by generalizing to the
inventory pattern, never by admitting parallel typed noun domains.


## Ruling: derived attachment (2026-09-02)

Prerequisite for the general PP / adjunct / postmodifier constructions.
Verb-selected prepositions attach at the frame (declared valence). Every
other prepositional phrase or adverbial attaches to the NEAREST constituent
that licenses it — low attachment: an NP-postmodifier reading wins when
the nominal licenses that preposition class ("creature with flying", "card
in your graveyard"); otherwise the phrase is a predicate adjunct. Attachment
is derived structurally, never stored and never guarded per construction.
Oracle style avoids genuine attachment ambiguity by design, so two
surviving readings for the same bytes is a genuine selection tie and a
STOP-and-report — never a preference weight or a default.


## Amendment: preposition classes and noun complement licensing (2026-09-02)

The derived-attachment ruling above presupposed nominal-side preposition
licensing that did not exist; a general PP without it licenses every
preposition at every site, and negative oracles ("deals 2 damage to each
creatures") parsed via rescue adjunct readings. Two declared facts close
the gap, both data, neither a per-construction guard:
- Each Preposition member declares its class: adjunct-capable (locative,
  temporal, manner — in, on, under, at, during, until, ...) or
  selected-only (to, into, onto, of, ...). Only adjunct-capable PPs enter
  the free predicate-adjunct / NP-postmodifier attachment rule.
- Nouns declare complement prepositions (noun valence, parallel to verb
  valence): relational nouns license their complement PP ("the top of",
  "the controller of", "a copy of"); other nouns license none. A
  selected-only preposition therefore appears only under a licensing verb
  frame or noun valence.
Negative oracles (agreement/quantity violations) are part of the fidelity
standard: a landing that makes one parse is a defect, never a coverage
gain.


## Correction: cutover debt path (2026-09-02)

§Crates' `deckmaste_english_v2 -> macro_ron -> deckmaste_features` debt no
longer exists: the macro-ron fold-back removed `macro_ron` from
`deckmaste_english_v2`'s dependency closure (v2 depends directly on
`deckmaste_construction_core`, which owns the spelling/grammar metadata
type as the row consumer). `macro_ron`'s remaining `deckmaste_features`
edge is its legacy `frames.rs` re-exports, deleted with v1.


## Ruling: vocab surfaces and homograph exemptions (2026-09-02)

The frame-literal collision rule extends to `vocab` surfaces: a vocab
member whose surface equals a declared noun or verb surface is a load
error. A genuine homograph (attributive `target` beside noun `target`,
adjective `untap` beside verb `untap`) is admitted only by a per-member
declared feature on the vocab entry, emitted on the surface row and
consumed by the collision check — never by a vocabulary-name or category
carve-out in the checker. Adjectives are content words: their long-term
home is an adjective inventory (core seed + declaration contributions,
mirroring nouns and verbs); the attributive-adjective vocab is a
transitional home, not a permanent one.


## Amendment: preposition classes are corpus-measured; complements license locatives (2026-09-02)

The earlier amendment's adjunct-capable list (in, on, under, at, during)
was written from general English, not Oracle distribution; it is
superseded by measurement. Each Preposition member's attachment class is
declared from its corpus behaviour — `under` occurs only selected
("enters under your control") and is selected-only; `on`/`at`/`during`
are adjunct-capable where the corpus shows free adjuncts ("on your
turn", "at the beginning of…"). Second, admissibility of a locative or
temporal PP is a conjunction: the preposition's attachment class AND the
complement head noun's declared licence for that preposition — zone
nouns declare their locative preposition (`hand`→in, `battlefield`→on,
`graveyard`→in, `library`→in/on-top-of), turn-part nouns declare their
temporal ones — relayed from the head up the NP stages to the attachment
site, the same mechanism that licenses `of`. "Destroy target creature on
your hand" therefore rejects on the noun's licence while "on your turn"
attaches freely; "Under your control, draw a card" rejects because
`under` is selected-only. No preposition or noun is named in any
construction; all of it is declared data.


## Ruling: complement attestation is provenance, not admissibility (2026-09-04)

A general English construction is declared from the language, not filtered by
the current corpus. Attestation supplies provenance and measurements; a
grammatical variant with zero current witnesses remains declared. In
particular, `PrepositionalComplement` admits every complement shape that an
Oracle-English preposition governs. Corpus measurement still determines each
preposition's `PrepositionAttachment` class, and attachment admissibility
remains the conjunction of that class and the complement-taking head's
declared licence.


## Amendment: attachment class is a declared linguistic property (2026-09-04)

This supersedes the 2026-09-02 "preposition classes are corpus-measured"
amendment above, which is retained as the record of the superseded method. That
amendment reversed the direction of evidence: it made Oracle distribution the
authority for a grammatical class and general English merely a prior. The 40
failures sampled for the 2026-09-04 fallout audit show what that costs — 28 of
40 are a construction that exists and refuses the values nobody printed.

A Preposition member's `PrepositionAttachment` is a **declared linguistic
property on its vocabulary row**, justified per member by what the word does in
English, and **defaulting to adjunct-capable and postmodifier-capable**. A
member is narrowed below that default only by a stated linguistic fact about the
word, never by a witness count. Corpus counts are provenance: they belong in the
landing record beside the row they describe, and they may schedule work, decide
priority, and expose a defect — they may never decide admissibility.

The source-of-truth test for every such decision, in the coordinator's words:

> "is this grammatical English, or near-grammatical within our approximations?
> if so it stays."

**Attestation is provenance, never a filter.** Stated once more here as the
governing principle for every inventory in this grammar — constructions, sum
arms, category members, preposition rows, noun licences, verb frames, vocabulary
members, and value lists in a `require`. A declaration admits its full
linguistic domain regardless of how many cards exercise it; a general
construction with zero witnesses is kept; a value list must answer "which
English fact excludes the missing values?" and a count is not an answer. A
ticket, brief, or landing record that says "add arms only where the census shows
them" contradicts this ADR and is a STOP.

Two consequences already measured. `With` landed 2026-09-04 with a
corpus-measured `PostmodifierOnly` class, so a free `with` that a sentence means
as a verb-level adjunct has no adjunct site and attaches low: thirteen of that
landing's 230 gains are that shape. Reversing the measurement is
`english-v2-attachment-class-declared`; the survivors that remain genuinely
ambiguous are the recorded attachment-misselection class and belong to
`english-v2-underspecified-adjunct-attachment`. Nothing here changes the second
half of the 2026-09-02 amendment: admissibility remains the conjunction of the
preposition's declared class and the complement-taking head's declared licence,
and no construction, checker, or literal may name a preposition, noun, verb,
construction, or card.


## Amendment: what a landing proves, discloses, and reports (2026-09-04)

The Plan 09 metric set was designed for a corpus-driven incremental coverage
push and outlived the approach it measured; the fallout audit found roughly half
of executor spend going into the measurement apparatus, and every regression in
the 2026-09-02..04 window arriving as a coverage *gain* that passed
`coverage --check` green. The permanent contract is three tiers, and
`CLAUDE.md`'s landing-record bullet is its operational form.

**PROVE** (gates; a failure is a defect, not a number to fit):

- **No silent loss.** Every identity that stopped being covered is named and
  classified — wrong analysis retired / re-coverage owed to `<live ticket>` /
  regression — and routed. An *unexplained* loss is the defect; a decrease with
  a complete, classified, routed list is normal. `DECKMASTE_COVERAGE_LOCK=report`
  is the normal mode.
- **The structural laws.** Byte-exact roundtrip, lexical ownership (no gap,
  overlap, synthetic claim, or provenance mismatch), traversal identity
  (construction and leaf), zero unresolved ties, zero internal failures.
- **No word-naming.** No construction, checker, or form literal names a word,
  card, or mechanic: forbidden licensing checkers at zero, plus the environment
  load errors for an unlicensed literal/vocabulary collision and for a licensed
  form literal governing nothing.

**DISCLOSE** (obligations of the record; absence is a review finding):

- Every newly covered identity with its selected analysis. A wrong analysis that
  starts parsing is a STOP, never a coverage gain.
- The selection census before and after, with the construction pair named if the
  specificity-resolved share rose.
- The permitted licensing-checker count.
- A `Deviations and additions` list, and every STOP with its resolution.
- Glossary gaps: any term the landing needed that
  `docs/contexts/oracle-english/CONTEXT.md` does not define.

**REPORT** (provenance; never fitted to, never a gate):

- The lock `covered` count and the construction count.
- The homograph and form-literal/vocabulary overlap counts, as named inventories
  rather than bare numbers.
- Performance: coverage-command wall time against the 16.26 s quiet-host ceiling
  and the per-byte thread-CPU figure as an integer in ns/B, each stated with the
  host load and worker count.

The exact pins that made these provenance figures behave as targets —
`licensed_vocab_lexicon_homographs == 2`, `form_literal_vocab_overlaps <= 5`,
the builtin noun-morphology census `491 / 165 / 26 / 300`, `roots == 8`, the
card-name row count `32_548`, and the parenthetical census `17` / `136` — are
demoted to report-only by `xtask-legacy-pins-to-provenance`. Their real
invariants (the two-way onset-override closure, the census partition identity,
and the `environment.rs` load errors) stay as gates.


## Amendment: focus is non-iterating declared syntax (2026-09-05)

English focus does not iterate: a Focus construction requires its operand's
declared `Focus` feature to be `Unfocused` and derives `Focused` for its result.
Every unfocused member of a focus-bearing sum derives `Unfocused`, and the same
two-value feature applies to every focus category. The guard reads only this
declared grammatical feature; it never names an adverb, construction, verb, or
card. A Focus wrapper remains transparent to every other feature and
admissibility classification its host reads.


## Ruling: corpus timing ceiling and acceptance-cost telemetry (2026-09-02)

The gate criterion stays a per-command wall-clock ceiling of 16.26 s,
measured on a quiet host (the hardcoded `CORPUS_WALL_CEILING_SECONDS`
cites this section); under sibling-workspace load the warning is
advisory and a landing record states the load. The per-byte acceptance
cost proposed by the performance investigation is adopted as TELEMETRY,
not a gate: every corpus command reports it, computed from thread CPU
time (`CLOCK_THREAD_CPUTIME_ID`), never summed wall time, so it is
load-insensitive. Worker parallelism (`--workers`, default = host
threads) is part of the measured configuration and every timing claim
states its worker count; a speed-up claim decomposes parallelism from
constant-factor work. Persistent breach on a quiet host remains
STOP-and-report; re-blessing the ceiling is a coordinator ruling.


## Amendment: selected roles preempt postmodifiers (2026-09-03)

A verb frame that declares a preposition role — required or optional —
selects that preposition: within that frame's object position, the
noun-postmodifier derivation of the same preposition does not exist. This
is not a preference weight; it is derived from the frame's declared
valence (the ruling "verb-selected prepositions attach at the frame"
applied to optional roles). "Return target creature card from your
graveyard to your hand" attaches `from your graveyard` to Return's source
role; "Destroy target creature card from your graveyard" (Destroy declares
no `from`) attaches it to the noun. Absence of an optional role still
satisfies its predicate. Frame-role wrapper constructions that exist only
to spell a preposition (`SourcePhrase`, `ControlPhrase`) are dissolved.

Amended 2026-09-04: each declared role is filled by exactly the first eligible
Prepositional Phrase after the object in linear order, so preemption removes only
that candidate and preserves later same-preposition Postmodifiers inside governed
material.

Amended again 2026-09-04: the candidate is sought along the right periphery of
the governed material in form order, so a Prepositional Phrase inside a non-final
Conjunct is not a role candidate and keeps its Postmodifier derivation.

## Ruling: adjunct licences removed; attachment misselection is a recorded class (2026-09-03)

Verb valence rows carry no adjunct licence. A temporal, manner, or locative
adjunct attaches to any verb clause, matrix or embedded; the derived
low-attachment rule selects among survivors. The whitelist licence
(`AdjunctLicensed` / `NonprepositionalAdjunctLicensed`) rejected attested
sentences on every unlicensed verb and encoded no English fact; it is deleted
(`english-v2-adjunct-licence-removal`).

Consequence, recorded as a known class: right-peripheral adjunct attachment
is not grammar-decidable. Low attachment is a selection, not a fact. Seedborn
Muse — `Untap all permanents you control during each other player's untap
step` — selects `during…` under `control`; the card means it under `untap`.
Every consumer that reads an adjunct's attachment must treat it as
provisional until the resolution below lands.

Resolution (design, not yet scheduled): underspecified attachment. Candidates
that differ only in a right-peripheral adjunct's attachment site collapse into
one packed candidate; the adjunct node is hoisted to the highest site and
carries the set of admissible lower sites as a value. Semantics chooses. An
attachment-only tie is therefore one candidate, not a STOP; any other tie
remains a STOP. Ticket: `english-v2-underspecified-adjunct-attachment`.

## Amendment: one scope device, principles before packing (2026-09-04)

Widens the 2026-09-03 resolution above from right-peripheral Adjunct attachment
to the scope of every right-peripheral or shared Constituent, and orders it
against the principles that decide before it. Coordinator rulings Q1-Q4,
2026-09-04. Ticket: `english-v2-underspecified-adjunct-attachment`.

**Q1 — one class, one device.** Two candidates that differ only in where a
right-peripheral Constituent attaches, or in how far a Constituent realized once
beside a Coordination scopes over it, are one ambiguity class: *scope of a
right-peripheral or shared Constituent*. Adjunct height, Postmodifier scope over
a Coordination, a shared Determiner, a shared Modifier, a shared head and a
shared preposition are that one class, and get one device, not four.

**Q2 — declared principles decide before packing.** An ordered set of declared
English principles runs first; whatever survives it is packed. Each principle is
one general rule over derivation shapes and declared features, stated once. No
principle may be a corpus count, name a word, or be a dominance edge between
named constructions. The admitted set, in order:

1. A qualification Complement — scalar comparison, degree measure, power and
   toughness value, granted keyword line, quoted ability — predicates a property
   of a Nominal and has no verb-Adjunct site at all. Landed as the
   complement-kind site capability.
2. A frame-selected role preempts the Postmodifier derivation of the same
   preposition, at every preposition the Verb Frame declares and at every depth
   of the complement it governs. The 2026-09-03 amendment above, extended;
   `english-v2-role-preemption-depth`.
3. An identity claim outranks a lexeme claim over the same bytes. Landed as the
   Identity specificity tier.
4. A distributive measure attaches to the Predicate as its multiplier and has no
   Nominal-Postmodifier derivation.

Not admitted, and therefore packed: the scope of a shared Determiner over a
Coordination. English is ambiguous there and Semantics chooses.

**Q3 — representation.** The ambiguous Constituent is hoisted to its highest
admissible host and carries the set of admissible lower hosts as a derived value
on that host, in the manner of the zero Determiner: a declared field kind, no
new AST category, no bytes in any Realization. Every host renders the same
bytes, so Linearization and the roundtrip law are untouched, leaf traversal is
unchanged, and the canonical construction path is the hoisted one. Consumers —
the semantics workbench and the xtask diagnostic display — read the slot. A
first-class packed AST node holding alternative subtrees is deferred to the
re-layering wayfinder, not built here.

**Q4 — census.** A packed candidate is one candidate: not a tie, never a STOP.
The packed-unit count is a REPORT figure, listed with its identities. A tie that
is not a scope tie remains a STOP.

Placement amendment (2026-09-04): a mobile role is valid at a form edge,
immediately left of its declared scope sibling, or adjacent to a sequence role.

## Amendment: closed-class ownership and the licensed form atom (2026-09-04)

One closed-class word has one owner. Where a form literal spells a word a
vocabulary member already spells, the literal is deleted and the construction
consumes the member (`role: lex Vocabulary` plus `require role is Member`),
never a Rust `checked by` naming the word. Where the shared surface belongs to
two genuinely different words, the form atom declares that with the
fixed-surface annotation `licensed(surface)`: exactly one unnested literal,
accepted only in a form (never as a bound or circumfix value), lowered like an
ordinary literal, and emitted as `homograph_license` metadata on the form
surface row. It emits no AST field, form tag, or parser branch, and changes no
rendering. A licence that governs nothing is a load error: an environment
rejects a licensed form literal whose surface no vocabulary member owns, so
the licence cannot be used to hide an overlap that does not exist.

The homograph census is therefore two numbers, both gated by
`english_v2 coverage --check` beside the lock: licensed vocabulary/lexicon
homographs — the per-member declarations of the 2026-09-02 ruling above —
pinned exactly, with the admitted rows named in the failure; and unlicensed
form-literal/vocabulary overlaps as a ceiling that only decreases. The former
single `literal_lexicon_collisions` count mixed both and could be raised by
either cause.

## Amendment: Verb Frame and execution-context vocabulary (2026-09-04)

This amendment supersedes the verb-valence and base-frame vocabulary above
without changing any production, selection rule, or semantic contract. A
lexeme owns a `VerbFrameSet` containing `VerbFrame` schemas. The
amount-selecting role is `MeasureComplement`, and a realized lexical verb
together with its selected Complements is a `LexicalVerbPhrase` inside the
larger `VerbPhrase` category. `VerbFrameKey` is only the compiler
compatibility key between a declared Verb Frame and its realization.

The Game Model evaluation context is `ExecutionFrame`. It is not a Verb
Frame or a Lexical Verb Phrase, and English-v2 does not use that Game Model
name for syntax. Historical uses of “valence,” “Numerative,” “base verb
frame,” and unqualified engine “Frame” remain above as the record of the
superseded vocabulary.

## Amendment: one Targeting Marker with two projections (2026-09-04)

Targeting-sense prenominal *target* is one invariant lexical Targeting Marker,
not a determiner, adjective, or count noun. This supersedes the categorical
target-as-determiner claim above, the 2026-08-27 target-as-noun amendment's
statement that *target* remains a determiner, and the 2026-09-02 homograph
ruling's description of attributive *target*. It retains the syntax those
claims were trying to preserve: a thin determinative projection licenses bare
singular *target creature*, while a thin nominal-modifier projection licenses
*target* under *another*, numerals, *up to N*, and similar determiners, as well
as in zero-determined plural nominals. Both projections consume the same
Targeting Marker and contribute the same later projection to the Game Model
Target relation; neither projection is a second lexeme. Rules meaning remains
downstream: a target is chosen for a spell or ability according to its
requirements [CR#115.1]; an effect that changes a target may replace it only
with another legal target [CR#115.7a], while one that lets a player choose new
targets may leave any number of them unchanged [CR#115.7d].

The Target Noun *target/targets* and the Target Verb
*target/targets/targeted/targeting* remain independent lexical homographs of the
Targeting Marker, never projections of it. The Target Noun is declared and
licenses noun uses such as *choose new targets*; the Target Verb, and with it
verb uses such as *a spell that targets*, is not yet declared and is owned by
`english-v2-target-verb-subject-selection`. The declared homograph licence
governs the shared surface without any scanner or grammar guard naming the
marker, a construction, or a card.

## Amendment: Concord Class and Inflectional Form (2026-09-04)

This amendment supersedes every use of `Agreement` above as the name of the
compiler feature. The feature is `ConcordClass`, with the derived morphological
equivalence classes `Other` and `ThirdPersonSingular`. It is not grammatical
Agreement, underlying Person or Number, Finiteness, or an Inflectional Form.
The homogeneous-sequence and per-feature-carrier equations are consequently
`derive members.concord_class = ...` and
`derive concord_class = members.concord_class`; their propagation and licence
reader rules are unchanged.

The compiler's transient morphology inventory is the single Inflectional Form
dimension: plain, third-person-singular present, preterite,
gerund-participle, and past participle. Plain can occur without a Concord Class
in nonfinite use or with `Other` where a present finite paradigm selects it;
third-person-singular present combines with `ThirdPersonSingular`; preterite
can combine with either class; participial forms carry no Concord Class.
Finiteness remains a separate syntactic dimension. No authored form tag or
public AST form tag is stored, and the sealed one-member `Participle` compiler
axis remains the way participial lexical rows are selected. In particular,
there is no mixed Finiteness/Inflectional-Form pseudo-paradigm.

The exceptional copula constraints are retained until explicit Person and
Number land: *was* selects the `ThirdPersonSingular` path and *were* the
`Other` path, while both remain morphologically preterite. Those constraints
are pinned independently in finite passive predicates, finite copular
predicates, existential finite clauses, and copular subject-gap relative
clauses; the temporary selection split does not reclassify either word as a
present Inflectional Form.

## Amendment: an identity claim outranks a declared type over the same bytes (2026-09-04)

Selection compares candidates position by position over claim kinds. That
ordering gains a fourth tier between Typed Lexical and Literal: an **Identity**
— a spelling supplied by the parse context or by a catalog rather than by a
declared type. Identity outranks Typed Lexical; Literal still outranks
Identity.

The rule is a rules fact, not a preference. Text that refers to the object it
is on by name means just that particular object and no other object with that
name [CR#201.5], and a card's shortened name used that way is treated as its
full name [CR#201.5c]; the same holds for a name inside an ability one object
grants another [CR#201.5a]. So where a card's own name and a declared type
spell the identical bytes — `Nightmare`, `Daretti`, and every other card whose
name is also a creature or planeswalker type — the self-reference is the
correct reading and the type reading is wrong.

It is one ordering over every identity: no dominance edge between named
constructions, no selection exception, and no checker or requirement naming a
card, word, or construction. `Lexical::is_identity` is generated from the
declared identity inventory, so a new identity declaration joins the ordering
without further code.

Consequences measured on the corpus at the time of the ruling: the two
identities it was ruled for (`Daretti, Rocketeer Engineer`, `Nightmare`) select
their self-reference uniquely, and no other unit's selected analysis moves.
Where no identity claim covers the bytes the declared type still wins —
`This Vehicle's power is equal to the number of lands you control.` keeps its
proper-Noun possessive owner, and `a card named Alpine Watchdog` keeps its
catalog identity.
