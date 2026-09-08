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
  normalization. Identical normalized forms create duplicate Productions but
  the same canonical Reading form identity.
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

`frame Name = "<RON Frame>";` declares a complete data-only lexical frame
signature. It preserves frame kind, ordered items, grammatical relations,
Categories, marker identities and optional/marked nesting. For example:

```text
frame Object = "(kind: \"transitive\", items: [Argument((relation: Complement, category: \"Nominal\"))])";
```

A head Construction can export `frame = head.frame`; a governing Construction
can `require head.frame = Object`. Matching reads the declared lexical property,
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
