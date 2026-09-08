# English v3 chart runtime

This crate implements recognition and lazy Reading materialization for the
[accepted lexical/grammar contract](../../docs/decisions/english-lexical-analysis.md).
It consumes `deckmaste_lexical::AnalyzedText` and the same immutable `Lexicon`.
The fresh construction compiler generates `Grammar` and `Materializer`
implementations for the interacting `slice` module and the connected `grammar`
candidate in `src/declarations.rs`. The runtime has no v2 parser, compiler or
Semantics dependency. The source-loader dependency is confined to integration
tests, which exercise the candidate against the authored lexical inventory.

The slice remains a bounded compiler/runtime witness. The connected candidate
currently composes flat paragraph/document sequences, finite and imperative
clauses, noun phrases, selected verb frames, auxiliaries, prepositions, relative
Subjects/Objects, binary coordination and locally selected ellipsis. Keyword
lines, symbol/action cost lists, activated paragraphs, ability-word heads,
parentheticals, literal quotations and modal lists now share those constituents.
Its Reading
values preserve lexical forms, tense and selected frames even when a parent has
no reason to inspect those features. Auxiliary complements own their separator,
so omission contributes neither a word nor a trailing space.

Whole-grammar activation is still in progress. The remaining family audit
precedes the full supported-corpus baseline. The
`english-v3-systemic-residuals` follow-up owns general failures exposed by that
experiment, including unresolved article/onset, capitalization, frame,
coordination, extraction and document variants. The activation accounts for
these with their inherited witnesses rather than requiring systemic closure.
`tests/grammar.rs` checks composition
with the real source inventory, invalid agreement/Case/frame combinations, and
an independently constructed auxiliary-ellipsis Reading. No whole-corpus
baseline has been measured.

The measure declarations distinguish cardinal determiners and ordinal modifiers
from scalar digit notation, following style guide §4, “Numbers, quantities, and
comparisons”. Scalar expressions include declared variables and arithmetic;
prepositions, measured noun phrases, verb frames and comparison adjectives
consume them. Framed adjectives cannot use the bare-adjective rule to discard a
required Complement. Both Arabic notation choices survive for small values
whose surfaces need no commas; prose magnitudes of four or more digits require
grouping. No arithmetic value or discourse variable is evaluated by admission.

Keyword declarations supply their parameter class. Bare, numeric and symbolic
cost uses have distinct Productions; a required parameter cannot disappear via
the bare-keyword rule. Comma/semicolon keyword continuations and cost/mode lists
retain ordered flat collections. Quoted and keyword Objects use selected verb
frames. The Target Verb has its own regular paradigm and transitive frame,
independent of the Targeting Marker; both can occur in one relative clause.

Type Lines use declared catalog roles for supertypes, card types and subtypes.
Their flat groups preserve order and lexical identity, with supertypes before
the nonempty card-type group and an em dash before a nonempty subtype group.
These Productions implement the current Lean document relationship. Further
ordering and subtype-compatibility obligations remain part of the systemic
handoff; the grammar does not sort the input or claim those constraints.

Generated admission, realization and traversal dispatch to separate methods for
each Construction. This keeps recursive stack frames from reserving temporary
space for the entire grammar's match body. A nested, independently constructed
paragraph exercises exact realization and reparsing on the normal test stack;
no larger worker stack is required to pass it.

## Raw corpus census

The generated connected grammar has a corpus command:

```sh
cargo xtask english-v3 --data /tmp/AtomicCards-subset.json \
  --output /tmp/english-v3-report.json --workers 1
```

It shares the independent lexical command's supported-face filter and stable
group/index identity. It analyzes raw text, including reminders, without
normalization. Missing text is analyzed as an empty Document while its JSON
`null` remains distinct from a present empty string. `--field type-line` runs the
same checks with the Type Line root over each supported face's raw type line.
The default is `--field text`. Reports record the selected field; `source_sha256`,
unknown-word offsets, census and timing metrics refer to that field. Original
`raw_text` and `type_line` metadata keep their meanings in either mode. A missing
Type Line has no Reading because its required card-type group is absent.

Enumeration is complete by default. `--reading-limit N` requests a bounded
prefix for inspection; the report never calls a capped single Reading unique.
Two checked distinct Readings establish multiplicity even without exhaustion,
but exact totals require exhaustion without errors. Duplicate Derivations,
cycles, materialization failures and validation failures have separate counters
or diagnostics. Every counted Reading passes declaration admission, lexical
ownership/context, byte-exact realization and traversal comparison. The report
is written before validation issues make the command fail.

Traversal evidence comes from a materializer decorator, independently of the
generated visitors. It records complete subtree fingerprints and lexical values
in production order, then compares them with node and word visitation. The
fingerprints use generated Debug encoding and are diagnostic within the stamped
source tree, not a persistent serialization format. These checks add work to
materialization; their cost is included in validation time and thread CPU.

JSON retains input and lexical-inventory hashes, the current change ID when jj
is available, each face's identity and raw text, unknown words with byte ranges,
checked Reading samples, construction occurrences, chart/forest and enumeration
counters, stage timings, host load and worker count. Failure groups separate
observed lexical gaps from unresolved grammatical or lexical causes; a missing
Reading alone does not identify its linguistic cause. `--samples-per-face N`
limits stored trees without limiting enumeration or checks.

The command does not establish independent linguistic correctness or the
constructed-value roundtrip law. Those still require independent expected
values and the inherited witness audit. A corpus census, cause-grouped residual
analysis and whole-family acceptance remain outstanding until the complete
connected grammar is ready for the full supported snapshot.

## Admission and packing

An Earley item is `(production, dot, origin, end, state)`. There is exactly one
intermediate node for each item, including the empty prefix. Each alternative
is either an empty edge or a binary `(prefix, child)` edge. Child vectors are
never copied or enumerated through the agenda. A completed node is keyed by
`(category, origin, end, summary)` and holds `(production, final prefix)`
families. Productions with the same future distribution can therefore share a
completed node even when they construct different Readings.

The grammar supplies pure, finite state transitions. Its summary equality is
future admissibility: every possible parent must treat equal summaries alike.
Prefix states preserve distinctions affecting suffix admission, resulting
summaries, and construction-local grammatical feature choices. A child vector
or a set of full AST alternatives is not a summary. Correlated alternatives
must remain whole tuples or an equally exact constraint relation; independent
sets of Person and Number do not suffice. Lexical feature absence is not a
wildcard. Open agreement, selected frame slots, extraction, sharing and
recoverability must survive until the context that discharges them. Root
admission checks obligations that cannot remain open at the requested Category.

These are obligations on the generated grammar, not a theorem the generic Rust
trait can enforce. The test grammars demonstrate both positive admission and
crossed-feature exclusion, carry each open dependency through an intervening
constituent, and carry a declared lexical frame to its governing parent. They
are synthetic runtime witnesses, not a replacement for the connected English
judgment or the compiler's obligation checks.

The chart projects occurrences into grammatical summaries once and indexes
them by scalar position and lexical Category. Overlapping, bound, homographic
and multiword occurrences stay available. It does not tokenize or infer POS.
Validation checks each supplied occurrence against the lexical realization
index and its tokens, without rerunning analysis. Word identity, provenance,
capitalization and spelling variant stay on lexical leaves; exact declared
literal text stays on literal leaves. Positions in chart keys serve recognition
only and are never passed to the grammatical admission functions.

Every new item is scheduled once. Waiters join each compatible completed node
once, whether the waiter or the child arrives first. New edges/families on an
existing node require no repeat completion: its parents already reference that
node. This also handles nullable rules whose child families grow after a parent
has completed. Empty literals are zero-width declared leaves; empty productions
have an empty prefix and no children.

## Reading identity and demand

`Forest::readings(materializer)` does no AST construction. Each `next()` walks
packed edges using explicit continuations. The materializer receives ordered
children, the completed summary and final prefix state. Lexical leaves receive
their correlated grammatical summary. This preserves grammatical distinctions
that have identical surface forms, including construction-local choices no
longer visible to an outer parent.

The generated Reading type's equality/ordering must preserve lexical identity,
constituent structure, grammatical feature values and retained spelling
variants. It excludes computed source positions and mere derivation identity.
The iterator deduplicates materialized root values by that equality. No value is
materialized to decide chart admission. Two productions constructing the same
value count as duplicate derivations; two homographs or attachment structures
are separate Readings even if realization is identical.

Materializers cannot defer admission checks: their errors report internal
construction failures, not ordinary grammatical rejection. A cycle on the
active completed-node path yields `MaterializeError::Cycle`; subsequent calls
can still enumerate finite sibling paths. Reusing a nullable node as two
siblings is not a cycle. Encountering a cycle is an incomplete enumeration,
never evidence that the returned finite subset is the complete Reading set.
The future compiler must reject unsafe recursive/nullable shapes. Recognition
terminates on a finite state space even if that forest is cyclic; it cannot
force an arbitrary user-supplied grammar with unbounded states to terminate.

Enumeration has no polynomial-delay or polynomial-total-space promise. The
forest can encode exponentially many Readings or duplicate derivations. Asking
for the next *distinct* Reading may exhaust many duplicates. Saved continuations
clone partial constructed values; the deduplication set retains values already
returned. Materialization work and storage must be measured separately from
recognition. `ReadingMetrics` reports calls, completed derivations, unique
Readings, duplicates, cycles, internal failures and construction builds.

## Bound supplied by this implementation

Let `n` be the number of Unicode scalar tokens, `R` productions, `D` the sum of
their dotted positions, and `C` nonterminal Categories. Let `S` bound the number
of prefix states for a fixed production/dot/span, `Q` the summaries for a fixed
Category/span, `B` the number of transition outputs, and `L` the total indexed
lexical alternatives **after** grammatical projection. `A`, reported as
`lexical_alternatives`, is the lexical engine's input occurrence count; one
occurrence may project to several correlated summaries, so `L` can exceed `A`.
`lexical_projections` measures `L` explicitly.

- Items and intermediate nodes: `O(D S n²)`.
- Completed nodes: `O(C Q n²)`; completed families: `O(R S n²)`.
- Waiter/completed-node pairs: `O(D S Q n³)`.
- Lexical scan pairs: `O(D S n L)`.
- Intermediate edges: `O(B D S (Q n³ + n L) + D S n²)`.

Those are count bounds. Time additionally pays for lexical validation and
projection, literal matching, callbacks, cloning and comparison of states and
summaries, and `BTreeMap`/`BTreeSet` logarithmic insertion/lookup factors. A
literal of length `t` can cost `O(t)` per waiting item. Callback and key costs
are not assumed constant when features, frame lengths or dependency constraints
grow with input. Space includes the packed counts above, lexical/literal leaves,
indexes and the sizes of stored states and summaries. Lexical indexing/validation
also pays for occurrence spelling lengths and immutable-lexicon lookup.

For fixed finite grammar/feature domains, bounded transition costs, and
`L = O(n²)`, these count bounds specialize to cubic packed-edge/completion
growth (with logarithmic map factors in runtime). Unbounded feature cardinality
or constraints can destroy that specialization. Earley ancestry alone does not
establish a cubic parser. The long-prefix witness varies production arity too:
its linear measured counts are for the declared sequence grammar, not a general
worst-case promise.

## Reproduce acceptance

```sh
cargo test -p deckmaste_english_v3 --test packed -- --nocapture --test-threads=1
```

The tests print lexical alternatives, items, intermediate and completed nodes,
both kinds of families, completion/scan work and requested materializations.
They check homographs and duplicate derivations, nullable and late child growth,
ambiguous attachment, long ambiguous prefixes, agreement and correlated
counterexamples, open dependencies and lexical frames, multiword/bound overlap,
cycle reporting, and independently constructed spelling/feature values. Exact
realization assertions use the lexical engine for words and declared literal
leaves for separators, never a source-buffer echo. The ticket landing record
contains the measured counts and validation on its stamped tree.

## Generated interacting slice

`src/slice.rs` is the declaration authority for 15 Constructions. One macro
expansion supplies their types, Productions, correlated admission summaries,
lazy materializers, realization and traversals. `src/slice/vocabulary.rs`
declares the bounded lexical environment through the lexical engine's public
constructors, morphology defaults and irregular replacements. It does not use
v2 grammar or source adapters. The handwritten grammars in `tests/packed.rs`
remain runtime counterexamples; the inspection path uses generated rules only.

The shared nominal and Noun Phrase rules feed selected verb and preposition
Complements, finite agreement, imperatives, nominal headers and newline-separated
Documents. A Preposition Phrase can attach to the verb or the object Noun Phrase.
The two structures survive together. Present/past tense and the two grammatical
Numbers of `you` remain correlated with the finite head. Determiners agree in
Number with count nominals; missing features never license a constituent.

The slice intentionally admits an empty Document and a header with no sentences.
It handles one nominal Premodifier and at most one prepositional Adjunct at each
of the nominal and predicate sites, with a Basic Noun Phrase inside that Adjunct.
It is not a complete judgment for arbitrary Oracle text. Header nouns and the
noun/verb `cast` witness are linguistic fixtures, not a new card-coverage claim.
`one` retains noun, Determinative and Numeral lexical alternatives; the slice's
Determiner rule consumes the Determinative analysis. Multiword `first strike`
competes with the separate adjective and noun analysis. `learned`/`learnt` are
explicit replacing spelling alternatives. None of these spellings appears in an
admission guard or grammar literal.

Both roundtrip laws apply to every finite admitted value, including arbitrarily
long finite Documents. The compositional argument is the compiler's shared IR:
each valid Construction realizes its declared Production in surface order;
inductively each child contributes a derivation with the same features; the
same equations admit their composition; lexical context validation supplies the
same lexical values; materialization reconstructs the original fields and
canonical form. Finite consuming repetition extends this argument to Documents.
This is an implementation argument, not a machine-checked proof of Rust.

Independent exhaustive test families cover all 72 nominal values, all 180 Basic
Noun Phrases and 360 prepositional Complements over those nominals, 720 imperative
compositions spanning both optional attachment sites, 112 finite clauses spanning
every declared compatible subject/verb bundle, spelling and capitalization, and
62 headed/unheaded tense-ambiguous Documents through four sentences. These
families compare complete expected Reading sets, including every Reading's exact
realization. Separate witnesses assert full lexical and structural traversal,
wrong agreement and frames, jointly invalid feature combinations, and two
Derivations denoting exactly one Reading. Removing precomputed lexical
alternatives removes their Readings, demonstrating that recognition does not
rescan input. There is no exhaustive enumeration claim for the infinite set of
all finite Documents.

```sh
cargo test -p deckmaste_english_v3 --test slice -- --nocapture --test-threads=1
cargo run -p deckmaste_english_v3 --example inspect -- \
  'You cast one spell with one counter.' --readings 2
cargo run -p deckmaste_english_v3 --example inspect -- \
  'first strike' --category nominal --packed --readings 0
cargo build -p deckmaste_english_v3 --timings
```

Inspection reports lexical alternatives, vocabulary gaps, chart work and a
bounded prefix of Readings. `--packed` writes shared completed/intermediate nodes,
families and leaves with references, without expanding Derivations. With
`--readings 0`, both inspection and recognition construct zero ASTs. Reaching a
request limit does not claim exhaustion or report a total Reading count. A
request for the next distinct Reading can still traverse duplicate Derivations;
the iterator's separate counters disclose that cost. The 4/16/64-sentence test
compares recognition work and two requested Readings for growing ambiguity.
