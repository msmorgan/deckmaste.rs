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
the same Category, policy and role validation as fully written rows; defaults
do not infer feature equations.

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
  parents may inspect. Every Construction of that Category must export all of
  those features exactly once.
- Every `construction Name: Category` has one or more `form [...]` declarations.
  Named fields are `lexical(Category)`, a syntactic Category, `optional(Category)`
  or `repeat(Category, "separator")`. Repetition includes the empty sequence.
  Literals express the exact intervening surface, including whitespace,
  punctuation, empty strings and line breaks.
- Each alternative form must contain the same named fields, types and
  cardinalities, each exactly once. Forms may change literals and field order.
  Adjacent literals are concatenated and empty literals disappear during
  normalization. Identical normalized forms with the same form requirements
  create duplicate Productions but the same canonical Reading form identity.
- A form may append feature requirements before its semicolon, for example
  `form [subject: Subject, " ", predicate: Predicate]
  require predicate.CliticHost = Free;`. Direct and table requirements apply
  only to that form and combine with the Construction's common equations in
  parsing and independent admission. Schema instances inherit these
  requirements with their forms. Forms cannot add exports or agreements.
- `require field.feature = Value` constrains a declared feature;
  `agree left.feature = right.feature` equates values of one domain;
  `export feature = field.feature` makes that value available to parents.
  `export feature = Value` supplies a typed constant, such as plural Number
  derived by additive Coordination. Constants use the same finite registers
  in chart admission and checked construction as copied features.
  Optional/repeated fields have no singular feature value. Constrain their
  element Construction instead. Referencing a feature absent from a child's
  Category interface, omitting an export, crossing domains or imposing
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
builtin feature domains retain their existing absence behavior. Defaults do not
replace required exports from child Categories.

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
never a lexeme name or input word. Frames not in this grammar's inventory remain
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
