# English v3 chart runtime

This crate implements recognition and lazy Reading materialization for the
[accepted lexical/grammar contract](../../docs/decisions/english-lexical-analysis.md).
It consumes `deckmaste_lexical::AnalyzedText` and the same immutable `Lexicon`.
The construction compiler generates parsing, admission, realization and traversal
for `grammar` in `src/declarations.rs`. The `slice` module is a bounded
compiler/runtime fixture. The runtime has no v2 parser, compiler or Semantics
dependency; integration tests load the authored lexical inventory.

## Nominal export authoring

The 21 ordinary `Nominal` constructions use the compiler's
[head inheritance and static defaults](../deckmaste_construction_v3_core/README.md#head-inheritance-and-defaults).
The `category Nominal(...)` interface marks each feature `head` or `local`;
constructions select their head by the field name `head` and state their changes.
Other Category interfaces remain strict. Features retain their existing domains
and defaults.

## Fragment probe

```sh
cargo xtask english-v3 probe --text "Draw a card." --category sentence
cargo xtask english-v3 probe --text "target tapped creature" --category noun-phrase --json --packed
```

`probe` analyzes exact free text with the declared lexicon, without corpus
selection or normalization. Category names are case-insensitive kebab-case;
the default is `document`. An invalid name lists available categories.
`--readings N` caps inspected distinct readings (default 4); zero reports only
recognition. Admitted forest roots are summary groups, not a reading count.
The report marks whether enumeration exhausted the forest; a capped count is
only the number inspected. Each inspected reading is checked for exact
realization, admission and structural/lexical traversal identity.

Text output includes lexical alternatives, vocabulary gaps and reading trees.
`--json` writes the same diagnostics as one JSON object on stdout, including
realized strings and validation issues. `--packed` adds chart counters and a
packed-node/family dump to either format. Missing vocabulary and no readings
are diagnostic results; internal or validation failures fail the command after
reporting them.

## Census diff

```sh
cargo xtask english-v3 diff BEFORE.json AFTER.json [--json OUT] \
  [--manifest OUT --random N --seed S] [--check no-loss|identical-digests]
```

`diff` compares two census reports face by face over the faces present in
both, so a bounded census diffs against a full one; faces present in only one
report are counted. It prints analysed-source hash changes, lost (covered to
no Reading), gained, decreases, increases, digest-only changes (same count,
different `readings_sha256`), unchanged, and covered faces and total Readings
before and after, then capped face lists with counts. `--json` writes every
list uncapped. `--manifest` writes an identity manifest of every count-changed
face plus `--random N` unchanged covered faces chosen by `--seed`, ready for
`--identity-manifest`. Differences alone do not fail the reporting mode.

`--check no-loss` fails for lost coverage, incomplete or limited enumeration,
validation issues, incompatible versions/input policies/source snapshots, changed
face sources, or missing/added/duplicate face identities. Both reports must have
complete enumeration metadata, equal `selection_status`, and the same nonempty
face population; matching
subsets can be checked while iterating. `--check identical-digests` also fails for
any changed face count or digest, including unread faces. These checks do not
replace the judgment set or analysis-key gate for linguistic changes.

```sh
cargo xtask english-v3 diff target/english-v3/inherit-before.json \
  target/english-v3/inherit-after.json --check identical-digests
```

## Corpus search

```sh
cargo xtask english-v3 grep PATTERN [-i] [--all-matches] [--count | --names] \
  [--limit N] [--field text|type-line] [--json OUT] \
  [--census FILE --unread|--covered]
```

`grep` searches the supported faces the census selects, over the analysed
source the census parses (reminder text removed; Type Lines unchanged), with a
Rust `regex` (`-i` for case-insensitive). It prints one line per matching face,
`Card Name [face name] | …context «match» context…` (`--all-matches` prints
every match), capped by `--limit` (default 200), then `N faces, M matches`;
`--count` prints only that line and `--names` only the sorted face names.
`--json` writes every face with byte offsets into the analysed source.
`--census FILE --unread` (or `--covered`) restricts to faces with no Reading
(or at least one) in that census. Use it, never `rg` over the Scryfall file,
for corpus attestation checks.

## Blocker report

```sh
cargo xtask english-v3 blockers --census FILE [--json OUT] [--limit N]
```

`blockers` diagnoses every unread face in a complete schema-9 text census
using the current grammar and lexicon and the census's worker count. It reuses
card-scoped census line analysis, with no retained tree samples. Unread faces
split on analysed-source newlines; covered faces use their separately recorded census Ability
spans, retaining quoted multiline abilities and modal lists. The summary
counts covered faces whose segments differ from newline splitting. A source
or coverage disagreement with the census fails loudly.

Each failing line has lowercased `word:` keys for unknown words, or an `at:`
key made from the previous and failing word tokens. Report-only tokenization
uses maximal runs of letters, digits, apostrophes and hyphens, whole brace
groups such as `{2/W}`, and individual non-space punctuation. A chart
position inside a token selects that token; whitespace and token boundaries
select the next token. Numerals retain their digits. `<start>` / `<end>`
identify input boundaries. JSON preserves the Unicode-scalar chart index and
line byte span, and records two tokens before and after the failing token as
`context`. These keys diagnose failures and never affect admission.
“Vocabulary-only” means every failing line has unknown words; it does not
claim that declaring those words would suffice to make the lines read.

The summary prints unread faces, single-failing-line faces, vocabulary-only
faces, line coverage and segmentation disagreements. Three tables rank the
keys by sole-blocker faces, then touched faces: the top 60 `word:` keys,
top 80 `at:` keys, and top 40 failing tokens (grouping `at:` keys regardless
of their previous token). Each row has at most three examples; each face is
counted once per row. `--limit N` overrides all three text limits; JSON's
`word_keys`, `at_keys` and `failing_tokens` tables are always complete.

## Raw corpus census

```sh
cargo xtask english-v3 --data data/scryfall/oracle-cards.jsonl \
  --card-name "Lightning Bolt" --export-subset target/english-v3/oracle-subset.jsonl \
  --output target/english-v3/report.json --workers 6
```

The command requires an explicit face selector or `--all`, applies that
selection before grammar work, and excludes Scryfall funny sets even when
Vintage-legal. Balanced parenthesized reminder text is stripped before lexical
analysis; its contents are outside the parsing scope. `--field text` (the default) parses rules text as a Document;
`--field type-line` parses Type Lines. Missing fields are analyzed as empty,
with null retained in the report. Metadata preserves both original fields;
`source_sha256` hashes the original selected field. `analyzed_source` and its
hash record the actual parser input; lexical offsets and roundtrip checks refer
to that input. Card names are preserved verbatim.

Enumeration is complete by default. `--reading-limit N` caps requests without
claiming uniqueness or an exact total before exhaustion. `--samples-per-face N`
limits stored trees without limiting validation. Each counted Reading passes
admission, exact realization and both traversal checks against materialization
traces. Construction traversal compares complete structural values directly;
SHA-256 fingerprints identify samples and census Reading sets; traversal still
compares complete values. Census enumeration deduplicates by a 32-byte digest
instead of retaining every AST and trace. The ordinary parser API retains exact
value equality. Materialization and declaration-admission memos clear at
100,000 entries; eviction only causes recomputation.
Pure materializations share immutable values for an identical production,
ordered child tuple. Generated materialization consumes no Summary or State;
the lexical leaf key retains the complete Word, including frame and countability
choices. The per-face `materializations` counters
separate requests from memo misses (the `unique_leaves`/`unique_builds` fields,
which may count a value again after eviction); the ordinary `readings.builds`
counter still counts all build requests. Every enumerated Reading is checked,
including roots combining shared children. This does not count forest paths in
place of enumerating them.

`RealizationContext` pairs an immutable lexical environment with independent
analysis of one text. It memoizes declaration-admission summaries by the full
grammatical value with the immutable grammar and lexical environment fixed. The renderer
checks each complete Reading and every contextual lexical boundary; it reuses
lexical analysis only when the rendered bytes match. Selected-frame validation
uses ordered declaration/frame indexes rather than scanning unrelated expansions.
A different rendering receives a fresh analysis. Independently authored
values use the same checks, without relying on a previous parse.

The JSON report includes provenance, lexical gaps, census, samples,
parser metrics, selection provenance, snapshot hashes and timings, and is written before validation errors fail the
command. Linguistic correctness and independently constructed-value roundtrips
require separate evidence.

The census parses the complete face in its original Category and card-name
context. It then enumerates outermost Sentences, activated Costs and modal
Clause headers; mode bodies and parenthetical paragraphs decompose recursively.
Keyword lines, Type Lines and EmptyDocument remain ordinary whole units, without
a retained reason. A Sentence is a stopping boundary: quoted granted abilities
and relative clauses stay inside it. Labels, separators, parentheses and
connections remain validated surrounding structure.

`Forest::decompose` compares fixed constructor/leaf structures and the actual
permitted tuples of completed nodes. It accepts an independent product only
when those tuple sets agree, rather than relying on equal spans or counts.
Regions without a proved fixed surrounding structure and product stay whole
at their smallest available Reading boundary, with span, Category, count,
proposed parts and a retained reason in the report.
Label-led regions are retained whole because label alternatives are not modelled
as surrounding-structure variants. This is a decomposition limitation, not a
claim that the label and body choices are non-independent. Modal Paragraphs with
alternative attachments of trailing sentences also remain whole: the last mode
and the enclosing Paragraph have different surrounding structures.
Compiler sequence helpers are not Reading boundaries; `helper_fallbacks` counts
forced whole-region fallbacks when a helper survives decomposition.
Unit Readings undergo exact realization and traversal checks in the original
lexical environment. Generated surrounding realization depends on fixed fields,
child admission summaries and
recursive child realization; all distinct boundary-summary tuples and the root
constraints are checked as well.

The `lines` array now holds ordered enumeration units with Category and byte
span. The separate `ability_spans` array preserves the Ability boundaries used
by blocker diagnostics. Face counts sum the unit counts and are labelled
**enumerated unit Readings**, never Document-tree counts. Ambiguity is separate:
several one-Reading units are still unambiguous. An incomplete or failed face
has no exact count or complete digest; its partial checked count is separate.
Read means syntactically admitted, not a linguistic correctness claim.

Schema 9, decomposition version 2, digest version 2 and context version 1
invalidate older incremental results. Within a unit, tree keys and sorted
fixed-width tree-set hashing retain the old encoding. Each face's
`readings_sha256` hashes versioned JSON encodings of the ordered unit descriptors
and tree-set digests, plus its validated surrounding structure and root
constraints. Surrounding encodings retain constructor names, Categories, forms,
fixed lexical values and literal separators; compiler helpers are transparent.
Root summaries are derived admission data and are not hashed: their slot count
and order are compiler storage details. Every reconstructed surrounding Reading
still passes admission and the root constraint. Unit Reading identities and
surrounding constructor structure remain in the digest.
Root-contract changes increment the context version. A label, header, wrapper or boundary change therefore changes the
digest even if the unit trees stay identical. `decomposition_version`,
`readings_digest_version` and `readings_digest_encoding` specify the encoding. Complete, issue-free faces have a digest, including the empty Reading
set; limited or failed enumeration has null. The digest is diagnostic under the
recorded tree's generated Debug encoding.

The single re-baseline bridge uses the recorded schema-8/digest-1 report:

```sh
cargo xtask english-v3 --all --workers 6 --samples-per-face 0 \
  --migration-from target/english-v3/presentence-before.json \
  --output target/english-v3/presentence-after.json
```

Migration enumerates original contextual Ability segments and accepted Paragraph
regions independently, reconstructs every product tuple, and compares complete
tree identities and tree-set digests. Checked multiplication must equal the
original count; duplicate reconstruction, overflow, incomplete enumeration,
coverage changes or lexical/chart changes fail the command. A retained whole
region proves unchanged identity and count; it is not evidence of sentence
factorization. Every retained unit and reason is preserved in the untracked
report. Later `diff` comparisons require the new baseline and matching schema,
decomposition, digest and context versions.

For lexical edits, add `--incremental-from target/english-v3/base.json` to the
same census command. A compatible schema-9 base records source hashes plus each
Lexeme's fingerprint and complete Word Forms. Changes to `core.ron`,
`verbs.ron` or `vocabulary.ron` select faces containing old or new affected forms,
using conservative case-folded substrings of the recorded parser input so bound
and multiword forms, including those joined by reminder removal, are included. Every selected face is fully checked; other face results are copied.
The output records `incremental`, `base_census`, affected Lexemes/forms, and
rechecked/reused identities. Copied counters and timing/CPU telemetry retain
their base provenance; `measurement_scope` distinguishes them from this run's
wall time. Changes to `declarations.rs`, runtime sources, dependencies, input,
field, support filter or enumeration policy disable reuse. Old, incomplete or
invalid bases also fall back to checking all selected faces. An incremental
report supports iteration; the landing authority remains one complete `--all`
census.

Stored samples rank by total Construction Cost, then by their fingerprint for
deterministic ties. Each Construction defaults to cost 1; a declaration can
override it with `cost 100;`. Lexical leaves contribute zero. Generated
`local_cost()` and `total_cost()` expose the weights and their checked sum.
Costs affect presentation only: admission, Reading identity, enumeration and
the complete no/one/multiple census remain unchanged. When enumeration is
limited, the samples are cheapest among the enumerated Readings, with no claim
that an unseen Reading costs more. Generic label fallback Constructions can
carry a higher cost than declared-label Constructions without discarding either.

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
leaves for separators, never a source-buffer echo. The landing note of an
English grammar ticket carries the measured counts.

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

## CGEL distinctions

The connected declarations distinguish ordinary pre-head adjective phrases from
complemented postpositive phrases (CGEL, Ch. 6 §3.3, pp. 550–553). An integrated
PP noun modifier attaches inside the Nominal, under the determiner; there is no
unrestricted external NP attachment (Ch. 5 §§13–14.2, pp. 436–447). Restricted
peripheral modifier constructions and exceptional short attributive comparatives
remain separate coverage work.

Locative copulas select a Locative Complement rather than treating their PP as
an adjunct or substituting an elliptical auxiliary analysis (Ch. 4 §5.2,
pp. 257–260). Auxiliary ellipsis remains available where context can recover it;
its presence does not replace the ordinary locative reading.

Prepositions can select NPs, finite clauses, or both, as declared by their
lexemes. Conditional *if* is a preposition; interrogative *if*, *whether*, and
relative *that* retain their subordinator analyses (Ch. 7 §1, p. 600).
Temporal *when*, *whenever*, and *while* have clause-complement PP readings;
CGEL also recognises fused-relative readings, which require a separate gap
construction (Ch. 12 §6.4, pp. 1078–1079).

Clause coordination accepts both comma and unpunctuated binary forms. Serial
NP, clause, and predicate coordination has an ordered series with a final
coordinator and an optional serial comma; its summaries retain agreement,
case, voice, and verb-form constraints (Ch. 15 §1.1, pp. 1275–1278). Coordination
inside a Nominal and coordination of other phrase categories remain coverage
work.

Plain verb form does not carry a lexical Finiteness value. Secondary Verb
Phrases constrain their heads by Word Form, and the imperative construction
supplies finite clause status (Ch. 3 §§1.8.1–2, pp. 88–90). Gerund-participles
and past participles remain nonfinite. Tensed lexical bundles retain their
Number, Person, Tense, and Finiteness correlation.

### Closed-word references and lexical classes

Closed words are named in forms with `lexical_word`: infinitival *to*, relative
*that*, restrictive *only*, *all*, *each*, *both*, *either*, *neither*, and the
specific prepositions *for*, *except*, *as*, *as though*, *by* and *with*.
The optional postmodifier uses `require dependent is Else`; both negator policies
use `require head is Not`. An absent optional element remains admissible.
The historical lexeme IDs, Categories and forms remain unchanged, including the
two distinct *to* lexemes and the atomic multiword *as though*.

The four prepositions *by*, *of*, *to* and *without* declare membership in
`GerundClausePreposition`; the gerund-complement construction requires that
class. Words have one home in lexical data, including forms, properties and
frames. The grammar only binds names to IDs and declares class domains.
Callers pass `grammar::lexical_registry()` to the lexical-source loader for
validation.

Membership flags replaced by references leave the feature map. Genuine lexical
properties, including table-fed judgments and default-driven permissions, remain
features. *So* and *exactly* retain their properties: their existing generic
Adverb inventories do not establish closed-word eligibility. Class memberships
and feature domains are checked during loading; normalization and frame order
are unchanged. The source adapter's retained metadata keys are listed in its
README.

`BareSingularUse` remains the modifier's permission to license a bare singular
NP, and `IdentityUse = Name` remains the Catalog entry's declared name use.
Replacing these with `in BareSingularModifier` and `in ProperName` would mostly
rename the same conditions, so they do not justify new plugin/catalog membership
plumbing in this ticket. No production class currently needs that route.

The data also declares `ComparativeQuantifier` (*fewer*, *less*, *more*),
`SuperlativeQuantifier` (*fewest*, *least*, *most*), `DependentGenitivePronoun`
(*her*, *his*, *its*, *their*, *your*), and `NoncorrelativeCoordinator`
(*and*, *and/or*, *but*, *or*, *then*). Requirements name these word classes;
separate case, agreement, selection and separator judgments still apply.
The superlative class spans Adjective and Determinative; each form retains its
own Category and index.

`ComparativeDegreeModifierUse`, `AuxiliaryUse`, `FusedHeadUse`,
`KeywordComplement` and `QuantitativeAdjectiveComplementUse` remain permissions.
Their proposed class names would repeat the construction's role rather than
identify a more informative class. In particular, `AuxiliaryUse` licenses only
three entries in the two postmodified auxiliary constructions, not every
auxiliary; `FusedHeadUse` licenses the particular *anywhere*/*this* path, not
all determinatives with fused-head uses.
