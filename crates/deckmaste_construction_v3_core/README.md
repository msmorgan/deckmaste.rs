# V3 construction compiler

`deckmaste_construction_v3_core::compile` is a normal Rust function returning
`Result<TokenStream, syn::Error>`. The `deckmaste_construction_v3::constructions!`
proc macro only converts its errors to compiler diagnostics. Neither crate
depends on an English runtime, a v2 crate, lexical catalogs, or morphology.
Generated consumers depend on `deckmaste_english_v3` and `deckmaste_lexical`.

```rust
use deckmaste_construction_v3::constructions;

constructions! {
    pub mod example {
        category Nominal(number);
        category Pair();
        construction Noun: Nominal {
            form [head: lexical(Noun)];
            export number = head.number;
        }
        construction Pair: Pair {
            form [left: Nominal, " ", right: Nominal];
            agree left.number = right.number;
        }
    }
}
```

`example::Grammar::default()` supplies chart Productions and admission.
Pass precomputed lexical analyses to `deckmaste_english_v3::parse`, then request
`grammar.readings(&forest)`. Creating the iterator constructs no AST. The chart's
lower-level `forest.readings(&grammar)` also exposes materialization metrics;
its generated intermediate `Value` wrapper has a `Reading` variant at every
public Category root. There is no scanner callback or preferred Reading.

## Lowered form versus authoring layer

The chart consumes complete, finite summaries. Authors can now declare Category-interface
propagation and state only a Construction's changes; the compiler fills its
exports before validating and lowering them. Generic coordination elaborates
grammar-owned templates with per-interface combiners. Boolean requirements,
policy composition and typed lexical classes remain future authoring work. Policies still cannot use other policies,
and repetition does not automatically fold member features. The governing record
is `docs/decisions/english-lexical-analysis.md`.

## Head inheritance and defaults

```rust
feature Permission { No, Yes } default No;
feature Modified { No, Yes } default No;
category Nominal(head number, head Permission, local Modified);
category Clause(Permission); // strict: explicit export required
construction ModifiedNominal: Nominal {
    form [modifier: lexical(Adjective), " ", head: Nominal];
    export Modified = Yes;
}
```

Propagation belongs to each Category interface entry: `head Feature` inherits,
`local Feature` takes its declared default, and an unqualified entry requires an
explicit export. Features retain their domains and optional defaults. This
per-entry syntax also accepts an independent coordination combiner, described below.

A field named `head` is the head by convention. `head other;` selects another
field; `head none;` declares explicit headlessness. A Category with any head-mode
entry requires a head field or directive, even when all exports are explicit.
A selected head must be a singular lexical or constituent field. `head none;`
requires explicit exports for **every head-mode entry**, even with defaults;
local defaults still apply. A schema or its instance can declare the head, but
two directives conflict. A schema directive applies to all its instances;
`head none;` has no effect for interfaces without head-mode entries. Policies
supply equations, not heads.

After schema binding and policy expansion, **each form** fills its omitted exports
against its own head's concrete Category. A head-mode feature exposed by that
Category (including computed surface features) is copied; otherwise its declared
default is used. A lexical head may inherit **only features with declared
defaults**: an absent lexical property already takes that default in lexical
projection. Without a declared default, a lexical head requires an explicit
export, including for builtin features. Typed lexical classes will refine this
rule later. Constituent fallback is static, never a runtime repair of a missing
inherited value. A local entry always uses its declared default, never the head.
Without an applicable source or default, compilation names the Construction and
missing feature.

Explicit exports from a Construction, form, schema or policy override inference;
two explicit exports conflict. Requirements, agreements and table completeness
checks remain unchanged. Read the Category line and the Construction to determine
omitted exports, expanding policies and schema bindings where used. No duplicate
export inventory or inspection command is maintained.

Elaboration produces the existing reference/constant equations. The compiler still
checks complete, unique interface exports, accessible references, domains, table
arities, contradictory requirements, surface retention and recursion. It changes
neither field order nor form indices, Reading shapes, admission, chart or runtime.

## Coordination templates and combiners

English coordination structure stays in grammar-owned schemas. The compiler
binds their member Categories and derives feature equations and helper summaries.
Nominal, NounPhrase, adjective, adverb and preposition coordination use this facility. Correlative
adjectives bind AdjectivePhrase members to CorrelativeAdjectivePhrase results
and explicitly name CorrelativeAdjectiveSeries. AdverbPhrase uses one admission
requirement, copies UnmarkedConjunctLicence on ordinary forms, and omits it from
correlative helpers; completed correlatives retain the feature default `always Yes`.
PrepositionPhrase retains its ordinary-adjunct admission requirement and combines
LocativeUse with `all`, overriding the feature default. SelectedPrepositionHead
omits HeadCoordination from helpers; completed coordination supplies `always Yes`.
Their multi-valued judgments remain named tables, including the four-argument
complement-function judgment. Other families still use explicit instances until
their conversion is verified.

FinitePredicate, SecondaryVerbPhrase and their selected heads use the grammar-owned
Clausal profile: the same structural schemas without Standard’s GeneralCoordination
requirement. Finite members require a free right-hand clitic host while retaining
the left host. Predicate helpers omit result boundary features; selected-head
helpers omit HeadCoordination. SecondaryPredicateSeries keeps its explicit name.
FiniteObjectGap, BareObjectGap and PastParticipleObjectGap use the same profile and
boundary omissions. The past-participle family selects only binary/serial forms.
Clause serial/either forms retain a left-derived licence except at correlative
completion; exact-schema contexts identify the four forms that require a licensed
following series. CoordinatedFiniteClause binds FiniteClause members and explicitly
names FiniteClauseSeries and CorrelativeFiniteClauseSeries. Both use Clausal;
ClauseCoordination’s separate binary punctuation remains handwritten.

```rust
feature Permission { No, Yes } default No combine all;
category Nominal(head number combine same, head Permission);

schema Pair { head none; form [left: node, " and ", right: node]; }
schema End { head none; form [left: node, ", and ", right: node]; }
schema Continue { head none; form [left: node, ", ", rest: node]; }
schema Complete { head none; form [left: node, ", ", rest: node]; }
coordination_forms Standard {
    binary Pair;
    serial End, Continue, Complete;
}
coordinable Nominal using Standard { forms binary, serial; }
```

A feature's `combine` supplies the default for coordination, independently of
`default` and ordinary `head`/`local` propagation. A Category interface annotation
overrides that default; an `on` context overrides the Category choice. Every
combined entry needs a combiner. A Category annotation without a `coordinable`
block is an error. Ordinary constructions retain their existing export rules.

- `all`: Yes exactly when both inputs are Yes; otherwise No.
- `any`: Yes when either input is Yes; otherwise No.
- `same`: require equality and retain the value.
- `always No` (or another value): export a constant without reading inputs.
  Values named `all`, `any` or `same` remain unambiguous after `always`.
- `left.CliticHost` (or another reference): copy that input.
- A named table, such as `coordinated_oblique`: infer `(left.F, right.F)` for
  inputs `(F, F)`, or `(coordinator.CoordinationKind, left.F, right.F)` for inputs
  `(CoordinationKind, F, F)`, where F is the combined feature. Other signatures
  need explicit arguments, for example
  `determined_np_coordinate_number(coordinator.CoordinationKind, left.number,
  right.number, left.DeterminerAgreementNumber, right.DeterminerAgreementNumber)`.

**`all`, `any` and `same` are strict: an absent input rejects.** No/absent does
not satisfy `all`, and Yes/absent does not satisfy `any`. They are feature-valued
operations, not three-valued predicates. `all`/`any` require an exact Yes/No
domain. Named tables retain first-match, missing-row and absent-argument behavior.
Generated tables use the reserved `coordination_combine_` prefix.

References use `left`, `right` and `coordinator`. At a serial tail `right` is the
`rest` helper summary, and `coordinator` is its carried coordinator summary.
At a binary or end join it is the schema's `coordinator` field. Generic schema
fields use `left`/`right` or `left`/`rest`; markers, separators, lexical
restrictions, ordered forms and per-form requirements stay in the schemas.

Profiles declare `binary Schema;`, `serial End, Continue, Result;`,
`correlative_helper End, Continue;`, and `both`, `either` or `neither` followed
by a binary schema and an optional serial result schema. Both `forms` and `on`
use these role names. Selecting `either`, for example, selects precisely the
binary and serial forms declared for that pair, automatically adding the shared
correlative helpers when needed. `schema(Name)` is the exact-schema escape.
No undeclared form or correlative pair is invented.

Contexts also include `helper` (end/continuation), `result` (binary/completion),
`correlative`, `overt` (joins with a coordinator field), and `ordinary_overt`
(the ordinary binary and serial end). Profiles may place shared requirements
in these contexts: English Standard states
`on ordinary_overt { require coordinator.GeneralCoordination = Yes; }` once.
A profile context must match a template, but need not match every Category's
selection. A Category context must match a selected form.

```rust
coordinable Nominal using Standard {
    forms binary, serial;
    on serial { require left.UnmarkedConjunctLicence = Yes; }
    on helper { combine SomeSummary = always No; omit CompletedOnlySummary; }
    on schema(End) { require right.SomePermission = Yes; }
}
```

This illustrates scopes; the extra features need declarations in a real grammar.
An exact-schema override wins over role-context overrides. Overlapping overrides
at the same specificity reject; requirements and agreements accumulate. There
is no block-level combiner tier, Boolean requirement syntax or policy composition.

Helpers start with the Category's combined entries. Helper contexts can override,
add or omit entries without repeating regular ones; completed entries cannot be
omitted. Schema exports add helper entries, such as CorrelativeCoordinator.
Coordinator features read through tails are carried automatically from the end.
End and continuation must expose the same summary. Helper names default to
`<Category>Series` and `Correlative<Category>Series`; `helper Name;` and
`correlative_helper Name;` name exceptions.

`coordinable Result of Member using Standard { ... }` binds conjuncts to Member
and completed forms to Result; without `of`, both use the named Category.
Result's interface supplies the combiners. Helper names derive from Result unless
explicitly named. This supports correlative adjectives, coordinated finite
clauses and focused adjuncts without changing their constructor contracts.
Elaboration retains constructor names, field order, form indices and `category`
discriminators. Handwritten instances can coexist for other Categories.

The following English structures intentionally remain handwritten:

- Focused adjunct coordination: six result-only function features would each need
  a constant override and a helper omission. FocusedAdjunctConcord and
  FocusedAdjunctFunction state the summary and result function more clearly.

- ClauseCoordination's binary punctuation: two forms and a BareCoordination
  restriction, with its own constructor shape.
- Irregular premodifier paths: unlike Categories and separate asyndetic/syndetic
  restrictions are clearer as explicit constructions.
- SelectedComplementTail: `share_segments` is not a feature-summary combiner.
- NumeralComparativeCoordination, AdjectivalScalarCoordination and
  CardinalDeterminativeCoordination: heterogeneous members with specialized
  lexical and distribution requirements.
- MedialSupplementation: the anchor, supplement and continuation have a
  different structure from these coordination templates.

## Frame declarations

- `frame Name = ...;` declares a verb frame the grammar recognises. A lexical
  frame is usable only if it is structurally identical to a declared one.
- `form [head: lexical(Verb), complements: selected_frame(head, Family)];`
  expands the head's declared frames into complement sequences. A
  `selected_frame` form has exactly these two fields.
- `frame_category Atom = Category [where Feature = Value];` maps a frame atom
  to the grammar Category that realises it, optionally restricted by one
  feature value.
- `frame_adjunct` and `frame_postposing` declare, for a frame family, an
  adjunct admitted between complements and a complement that may be postposed
  after listed ones.

## Shared construction schemas

A schema owns one AST construction, its fields, surface forms, cost and common
feature equations. Category instances bind generic `node` fields and add their
own checked equations:

```rust
schema Coordination {
    form [left: node, " and ", right: node];
}
policy SameNumber {
    agree left.number = right.number;
    export number = left.number;
}
instance Coordination: Nominal {
    bind left, right = Nominal;
    use SameNumber;
}
```

Instances can share a checked body through category and policy rows:

```rust
instance Coordination<Result, Member, Agreement>: [
    (Nominal, Self, SameNumber),
    (CountNominal, Self, SameNumber),
] {
    bind left, right = Member;
    use Agreement;
}
```

Category parameters may bind their schema fields in the header:

```rust
instance Coordination<Result, Member: (left, right), Agreement>: [
    (Nominal, Self, SameNumber),
    (CountNominal, Self, SameNumber),
] {
    use Agreement;
}
```

`Head: head` replaces `bind head = Head`; grouped targets replace the equivalent
shared-category binding. Defaults follow the targets, for example
`Member: (left, right) = Self`. Targets must be declared generic node fields;
unknown, duplicate, lexical or fixed-category targets reject. Policy parameters
cannot bind fields. Header bindings expand to the same checked contracts and
Reading fields as body bindings.

The first parameter selects the result Category. Other parameters have exactly
one declared role: a Category in bindings or a policy in `use`. Each row supplies
those explicit Categories and policies; unused or mixed-role parameters are
errors. `Self` in a Category column aliases the row's concrete result Category;
it cannot replace the result itself or a policy. Additional Category parameters
can distinguish a serial
member Category from its intermediate-series Category. Each row must have the
same arity as the unique parameter list. Rows expand into independently checked
category contracts, preserving one shared Reading variant.

Trailing parameters may declare explicit defaults, for example
`<Result, Member = Self, Agreement = SameNumber>`. A row `(Nominal)` then supplies
those authored defaults; longer prefix rows override the defaulted columns.
Required parameters must precede defaulted parameters. Completed rows undergo
the same Category, policy and role validation as fully written rows; parameter defaults do not themselves infer feature equations.

Each schema permits one instance per result Category. Its generated Reading
variant stores `category: Category` alongside `form` and the shared fields.
Category is part of structural identity; admission checks the selected
instance's child Categories and feature equations. Materialization retains
that Category, while realization, lexical traversal and construction cost use
the schema once. Ordinary `construction` declarations keep their existing AST
representation.

Generic fields can also be `optional(node)` or `repeat(node, "separator")`.
Every generic field needs exactly one binding; grouped bindings name fields
sharing a Category. Bindings cannot change cardinality or lexical fields.
Schema surface forms and costs cannot be overridden by instances. Schema
feature equations are inherited and validated with the instance's equations;
conflicting requirements or duplicate feature exports remain errors.

`policy Name { ... }` declares reusable feature equations. Multiple `use Name;`
directives are allowed in schemas, instances and ordinary constructions.
Policies cannot declare forms, bindings, surface obligations, costs or nested
policy uses. Reuse supplies equations, never a new AST construction.

Policies may parameterize equation field references:

```rust
policy FiniteConcord<Other> {
    agree left.number = Other.number;
    agree left.person = Other.person;
    export number = left.number;
    export person = left.person;
}
```

`use FiniteConcord(right);` and `use FiniteConcord(rest);` instantiate the same
explicit equations with different declared caller fields. Only field positions
in references substitute; feature names, tables, constants and Categories do
not. Parameter names must be unique, argument arity must match, and every
argument and fixed policy reference must name a caller field. The ordinary
Category-interface and feature-domain checks still apply after substitution.
Field parameters have no defaults and policies cannot call other policies.

A row-selected policy composes with arguments, as in `use Agreement(right);`.
Every policy named in that column must accept that argument count. An explicitly
empty `policy NoConcord<Other> {}` is allowed; even its unused argument must name
a declared field. No-argument policies retain `use Name;` syntax.

The chart still instantiates concrete category-sensitive Productions. Count
shared AST constructions separately from category instances and chart
Productions (`Grammar::productions()` through the runtime trait). Generic
repetition retains the ordinary sequence contract: it does not automatically
fold member features or enforce Oxford-list cardinality. Such constraints must
be expressed by category-sensitive grammar rules and summaries.

## One validated representation

The syntax tree is validated into an IR before any projection is emitted:

- Every `category Name(feature, ...)` declares precisely the information its
  parents may inspect. Every lowered Construction exports all of
  those features exactly once; head inheritance and defaults fill omitted author exports.
- Every `construction Name: Category` has one or more `form [...]` declarations.
  Named fields are `lexical(Category)`, a syntactic Category, `optional(Category)`
  or `repeat(Category, "separator")`. Repetition includes the empty sequence.
  Literals express the exact intervening surface, including whitespace,
  punctuation, empty strings and line breaks.
- Each alternative form must contain the same named fields, types and
  cardinalities, each exactly once. Forms may change literals and field order.
  Adjacent literals are concatenated and empty literals disappear during
  normalization. Identical normalized forms with the same form equations
  create duplicate Productions but the same canonical Reading form identity.
- A form may append feature requirements or exports before its semicolon, for
  example
  `form [subject: Subject, " ", predicate: Predicate]
  require predicate.CliticHost = Free;`. Direct and table requirements apply
  only to that form and combine with the Construction's common equations in
  parsing and independent admission. Schema instances inherit these
  equations with their forms. After elaboration each form exports its complete Category
  interface exactly once, with common exports and form exports checked together.
  Forms
  cannot add agreements.
- `require field.feature = Value` constrains a declared feature;
  `agree left.feature = right.feature` equates values of one domain;
  `export feature = field.feature` makes that value available to parents.
  `export feature = Value` supplies a typed constant, such as plural Number
  derived by additive Coordination. Constants use the same finite registers
  in chart admission and checked construction as copied features.
  Optional/repeated fields have no singular feature value. Constrain their
  element Construction instead. Referencing a feature absent from a child's
  Category interface, leaving an export unresolved, crossing domains or imposing
  contradictory constants is a compile-time error.

Builtin feature domains are `number`, `person`, `tense`, `finiteness`, `case`,
`form`, `countability` (`Count`/`Mass`), `frame`, `numeral_kind`,
`numeral_size`, `framing`, `onset`, and `article_onset`. The first six use the lexical model's distinct
enums. A declaration can add a finite distribution feature:
`feature extraction { Open, Closed }` reads the correspondingly named lexical
property. Undeclared or absent values never act as wildcards. Agreement keeps
whole lexical feature bundles correlated.

A custom feature may explicitly provide an absent-property default:
`feature Class { None, Special } default None;`. Only an absent lexical property
uses that default; an authored value outside the declared domain remains absent
from projection and cannot satisfy an obligation. Features without defaults and
builtin feature domains retain their existing absence behavior. Separately, an
explicit `head` or `local` Category-interface qualifier enables the static export fill
described above; it does not change lexical absence behavior.

`onset` and `article_onset` share the `Consonant`/`Vowel` domain. The Lexicon
normalizes pronunciation and the selected article variant before admission.
Every Category automatically exposes `onset`, derived from its first pronounced
constituent in the selected form's order, including modifiers and collection
elements. An unknown first onset stays unknown. A Construction can declare
`onset Consonant;` or `onset Vowel;` for pronounced literal notation such as a
sign; other literals contribute no pronunciation. Agreement can therefore
compare `determiner.article_onset` with `head.onset` without naming words.

The module declaration `capitalization Positional;` enables positional casing
admission. Without it, a generic grammar leaves casing unconstrained.
`boundary Initial;` and `boundary Interior;` require the corresponding capability
and close that constituent's casing scope. A sentence and a quoted fragment can
thus declare different boundaries even when both end with a period. These checks
preserve the lexical variant, including exact names and bound forms.

Every Category also exposes `terminal_punctuation` (`None`, `FullStop`,
`QuotedFullStop`), derived from the right edge of the declared surface. Lexical
constituents reset it, whitespace preserves it, and closing quotation marks
retain a preceding full stop as `QuotedFullStop`. Optional and repeated fields
compose the same feature in recognition and independently checked values. A
sentence declaration can suppress its matrix full stop only when its Clause
already ends in a quoted full stop (CGEL, Ch. 20 §6, p. 1755).

Prefix states and complete summaries carry finite onset and casing capabilities
alongside the feature registers. Composition retains the first constituent's
initial capability and requires subsequent constituents to permit interior use.
Optional and repeated fields compose the same summaries in the chart and checked
construction; no child AST is needed to decide admission. Distinct correlated
surface choices remain distinct states until no parent can distinguish them.

`numeral_kind` distinguishes `Cardinal`, `Ordinal`, `Arabic`, `GroupedArabic`
and `Roman`; the grouping policy remains distinct even for a small digit
surface without commas. `numeral_size` is `Small` below magnitude 1,000 and
`Large` otherwise, allowing grammar to constrain notation by its host. Cardinal
and Arabic numerals project grammatical Number (magnitude one is singular);
ordinal and Roman notation project no Number. Their numeric values are retained
in lexical leaves, never carried as an unbounded chart feature.

`framing` distinguishes lexical declarations with selected frames (`Framed`)
from those without them (`Unframed`). This lets a bare-head Construction reject
a head whose declared frame requires a Complement. An explicitly declared empty
frame is still Framed and can be selected by its complete `frame` signature.

Finite feature tables derive a parent feature from correlated child features:

```text
table common_case(case, case) -> case {
    (Nominative, Nominative) => Nominative,
    (Accusative, Accusative) => Accusative,
}
```

Use `export case = common_case(left.case, right.case);` in a Construction
whose children export Case. Each table declares its input and output domains;
the compiler rejects wrong arities, wrong domains and duplicate input tuples.
A missing tuple rejects the completed candidate before it becomes an admitted
Reading. Constant, copied and table-derived exports all satisfy the same
exactly-once Category interface requirement. Tables read only declared feature
values, never lexical identities, spellings or ASTs. Their input registers
survive until completion, preserving correlations and future admissibility;
both parsing and checked construction evaluate the same generated table.

A table can also constrain admission without exporting a parent feature:

```text
require permitted(head.frame, complement.number) = Yes;
```

The named table must declare the argument domains and the expected constant's
output domain. A missing input tuple or a different output rejects the candidate.
These relational guards work in constructions, schemas and policies, including
field-parameter substitution. Their input registers remain correlated through
completion, using the same table lookup as derived exports; they add no Category
summary feature or Reading field.

`frame Name = Kind(items...);` declares a complete data-only lexical frame
signature. It preserves the exact frame kind, ordered items, grammatical
relations, Categories, marker identities and optional/marked nesting:

```text
frame Transitive = Predicate(Object(NounPhrase));
frame ObjectToObject = Predicate(Object(NounPhrase), Preposition(To), Object(NounPhrase));
frame Predicative = Predicate(Complement(PredicativeComplement));
frame BareNominal = Nominal();
frame NominalSymbols = Nominal(Marked(Preposition, Of, Complement(CostSymbols)));
frame OptionalObject = Predicate(Optional(Object(NounPhrase)));
```

`Subject(Category)`, `Object(Category)` and `Complement(Category)` create
argument slots. `Vocabulary(Member)` creates a standalone marker;
`Marker(Vocabulary, Member)` also supports vocabulary names reserved by this
syntax. `Marked(Vocabulary, Member, Relation(Category))` combines a marker and
slot. `Optional(item)` nests any item, and `Literal("text")` retains legacy
literal data. Categories, vocabularies and members accept identifiers or string
literals. They are frame data, so they need not name grammar Categories.
Frame kinds remain open identifiers. The existing `frame Name = "<RON Frame>";`
syntax remains supported and produces identical typed data and generated code.

A head Construction can export `frame = head.frame`; a governing Construction
can `require head.frame = Transitive`. Matching reads the declared lexical property,
not an input word. Frames not in this grammar's inventory remain
lexical choices, but cannot satisfy a required or exported frame feature.
Duplicate frame signatures are rejected. As with any grammar declaration,
authors must declare the appropriate complementary constituents and constraints;
the compiler does not invent a linguistic judgment from Category names.

There is no arbitrary Rust admission hook. All declared obligations execute in
the chart. The compiler turns equations into finite unification registers and
reads only the referenced child features. A register needed only for checking
is released after its final use; exported registers survive completion. Equal
states therefore admit identical remaining suffixes, and equal Category
summaries are indistinguishable to every generated parent. No AST, lexical
identity, input spelling, source position or derivation history enters those
registers. Construction-local grammatical distinctions remain in the typed
children or retained surface form; there are no materializer-only checks.

## Values, realization and traversal

Generated `Category` and `Reading` enums retain Construction identity. Each
Reading variant has its declared fields (`Word`, `Box<Reading>`,
`Option<Box<Reading>>`, `Vec<Reading>`) and a canonical `form: usize`, indexing
the distinct normalized surface alternatives in declaration order. Its equality
excludes Production identity and source coordinates. Invalid form indexes or
wrong child Categories are rejected by `Reading::admit` and `Reading::realize`.
Both operate directly on a constructed value, without parsing its grammar.

`Word` holds the complete `LexicalReading` (including spelling variant and
capitalization), the selected count/mass use, and the selected frame's index in
that lexeme's declaration. Equal duplicate lexical frame entries select their
first index. These choices survive even when the outer Category does not
export them. Provenance resolves through the same immutable Lexicon and lexeme
identity; ASTs do not copy source declarations or token ownership.

Realization walks the IR's surface order and delegates each lexical spelling
to `Lexicon::realize`. A final independent lexical analysis checks that each
emitted word is licensed at its rendered boundary. This catches, for example,
two free words concatenated without a separator, while allowing declared bound
forms. This check does not run the grammar, select a Reading, generate
morphology, or reuse source text. Temporary output offsets are discarded; they
are never stored in a Reading or used for chart admission. Context checking
costs one lexical analysis of the completed output, in addition to structural
admission and lexical realization. Checking a subtree alone uses that subtree's
surface as its context; enclosing realization checks the complete context.

`visit` walks Constructions in preorder and `visit_words` walks lexical values,
both in the selected form's surface order. Realization and both traversals are
emitted from the same normalized pieces. Every materialized lexical leaf owns
its complete lexical value exactly once; literal spelling lives only in the
declaration. There is no opaque source-buffer leaf.

## Recursion, packing and evidence

Optional and repeated fields lower to helper Productions whose derivation
structure is erased when materializing `Option` and `Vec`. Helpers cannot be
requested as successful roots. Nullable Categories are computed to a fixed
point. A second graph connects a Production's result to a child when all other
symbols can be empty. Any cycle in that graph is rejected, including cycles
introduced by helpers. Thus recursive descent along a forest path must consume
surface before revisiting a Category: finite input has no cyclic materialization
path. Nullable acyclic structure, consuming recursion and nullable repetition
with a consuming separator remain representable.

For a fixed grammar every feature register has a finite declared domain.
Lexical frame-choice indexes range over the finite supplied Lexicon and do not
propagate into syntactic summaries. Prefix states carry neither child vectors
nor an unbounded constraint stack. These facts establish the finite-state and
future-admissibility assumptions required by the packed chart. They do not
promise polynomial Reading enumeration: ambiguity and duplicate derivations
can still be exponential. See the runtime's README for the count bounds.

```sh
cargo test -p deckmaste_construction_v3_core
cargo test -p deckmaste_construction_v3 --test generated -- --nocapture --test-threads=1
```

The public-macro consumer tests check both roundtrip directions against
independent values, alternative surface order, optional and repeated fields,
nullable structure, full correlated agreement and its negative combinations,
lexical frames and forwarded distribution features, homographs, spelling and
capitalization variants, countability, numeral notations, duplicate packing,
contextual lexical boundaries, traversal and lazy long-prefix materialization.
Compiler diagnostics separately reject lossy surfaces, inaccessible or
ill-typed feature obligations and unsafe recursion. These are synthetic
compiler witnesses. The generated interacting English slice and whole-grammar
activation remain their own tickets; this crate adds no production English
coverage or compatibility alias.
