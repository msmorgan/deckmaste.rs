# English v3 chart runtime

This crate implements recognition and lazy Reading materialization for the
[accepted lexical/grammar contract](../../docs/decisions/english-lexical-analysis.md).
It consumes `deckmaste_lexical::AnalyzedText` and the same immutable `Lexicon`.
The fresh construction compiler will generate `Grammar` and `Materializer`
implementations. The runtime has no v2 parser, compiler or Semantics dependency.
There is no production grammar or corpus-coverage claim at this stage.

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
