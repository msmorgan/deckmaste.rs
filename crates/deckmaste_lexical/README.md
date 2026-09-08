# Independent lexical analysis

`Lexicon::new` freezes declared lexemes into analysis and realization indexes.
`analyze` returns every licensed lexical occurrence; `realize` accepts a lexical
value independently of source text. Shared grammar-facing values live in the
data-only [`deckmaste_lexical_model`](../deckmaste_lexical_model) crate and are
re-exported here; declaration provenance, morphology, binding, tokenization,
indexes, occurrences, numeral codecs, and realization stay in this deep module.
This crate has no construction, catalog-I/O, game-model or grammar-compiler
dependency. The governing contract is
[independent lexical analysis](../../docs/decisions/english-lexical-analysis.md).

## Input and morphology

A `Lexeme` carries its stable identity, category, source, grammatical properties
and frames. Each form declaration pairs one complete feature bundle with a
default realization or explicit replacing spelling alternatives. Person, Number,
Tense, Finiteness and Word Form remain separate. Equal spellings retain distinct
bundles and identities. Missing dimensions are not expanded into guessed values.
Finite verb bundles require all applicable agreement and tense dimensions.

The one default algorithm for each inflection is:

| Form | Default |
|---|---|
| Invariant, singular noun, plain verb, non-third-singular present | Lemma |
| Plural noun, third-singular present | Consonant + `y` → `ies`; final `s`, `x`, `z`, `ch`, `sh` → `es`; otherwise append `s` |
| Preterite, past participle | Consonant + `y` → `ied`; final `e` → `d`; otherwise append `ed` |
| Gerund-participle | Final `ie` → `ying`; drop final `e` except after `ee`, `oe`, `ye`; otherwise append `ing` |

Consonant doubling and lexical exceptions are explicit overrides; the analyzer
does not infer stress or paradigm classes. An override replaces the default for
the selected slot. Multiple valid spellings occupy explicit variant indices.
There is no fallback dictionary or unknown-word POS inference.

The `deckmaste_lexical_source` adapter reads the existing construction
vocabulary, authored plugin morphology and generated catalogs. Original owner
identities and source paths survive normalization. Optional RON exports are
inspection artifacts, not a v3 input interface: the prototype `FrameItem`
representation changed when correlated marked slots entered the shared model,
so regenerate old artifacts rather than expecting backward deserialization.
Catalog phrases remain whole entries; their internal words are not added as new
lexemes. Unmapped inventories and construction literals remain named source gaps.
V3-owned `deckmaste_lexical_source/lexicon/core.ron` declares closed-class
vocabulary, pronoun bundles, auxiliary paradigms and frame additions.
`lexicon/overrides.ron` supplies explicit irregular replacements under existing
owners and preserves the noun/determinative `one` entries.

Ordinary verbs acquire the regular paradigm before overrides. Old verb
`Unavailable` markers based on nonattestation are reported as retired source
policy; they do not suppress forms. The declared modal paradigms contain only their applicable finite forms;
no regular nonfinite paradigm is guessed for them. Supplemental adjectives are
loaded only when the keyword declaration explicitly requests that derivation;
the regular participle rule above and explicit replacements also drive their
realization.

## Occurrences and spelling

The lossless atomic sequence uses Unicode scalars. Lexical matches are edges
between scalar occurrences; this permits overlapping multiword forms and bound
prefixes/suffixes without committing to a word split. Whitespace and punctuation
remain in the sequence. Byte ranges are derived when requested and are absent
from lexical value identity. Raw and externally normalized text are kept distinct.
This ticket's corpus command analyzes raw text, including reminders.

Free forms require word edges; Unicode combining marks belong to those words.
`Binding` explicitly relaxes the left, right or both edges. Declared prefixes
expose a following host boundary and declared suffixes expose a preceding one,
including chains of bound forms. This is independent lexical segmentation:
Category, host and allomorph constraints remain grammatical admission work.
Multiword separators
must match the declared spelling exactly. `Capitalization::Initial` adds a
first-scalar uppercase realization when it differs from the declared spelling;
the reading records that choice. It does not license arbitrary case folding.
The grammar will constrain where an initial realization is appropriate.

`SurfaceStructure::Word` rejects embedded whitespace, separators and quotation
boundaries in lemmas and overrides. Internal hyphens and apostrophes are word
characters only in their permitted spelling positions. `Multiword` declares a
sequence of words with exact single-space separators. `Opaque` is reserved for
catalog, keyword and notation categories; a noun cannot escape word validation
by claiming an opaque spelling. Catalog and keyword entries retain their exact
atomicity, including internal punctuation, without declaring their constituent
words as lexemes.

Genitive clitics and negative prefixes have their own declared categories and
binding. Contracted auxiliary endings retain full finite bundles; straight and
curly apostrophes are explicit spelling variants. A genitive and a contracted
auxiliary may share the same ending without sharing grammatical identity. The
inventory does not decide which host or use is admissible.

The numeral codecs recognize cardinal, ordinal, Arabic and Roman notation
alongside ordinary lexical entries. They never claim exclusive ownership of
`one`, `I` or any other spelling. A numeric reading retains value, notation and
case. Checked realization rejects lossy Roman values: finite magnitudes through
3999 and the two integer extrema roundtrip; intermediate saturated values do not.

## Corpus accounting and inspection

```sh
cargo xtask lexical --data data/mtgjson/AtomicCards.json \
  --output /tmp/lexical-inventory.json --export /tmp/lexemes.ron
cargo run -p deckmaste_lexical --example inspect -- /tmp/lexemes.ron 'counters'
```

Use a supported-card subset during development. The report retains raw face
text, input digests, byte occurrences, overlapping material classes, unknown
words, unrecognized nonword material and unmapped source declarations. Ordinary
vocabulary, catalogs, keywords, numerals, symbols, affixes and clitics are
accounted separately. Schema 2 lists every indexed lexical owner with its
Category, properties, frame signatures and independently checked value count.
Every unknown word remains named; every nonlexical scalar is classified as a
structural separator, notation material or unresolved nonword. Those classes
are source accounting, not lexical or grammatical licenses. In particular,
classifying braces as notation does not license an unknown symbol inside them.
A word is covered only by a contiguous lexical path, including a containing
multiword entry; crossing overlaps alone cannot hide a remainder.

Every returned occurrence must realize exactly to its source substring. Every
indexed lexical value is also realized independently and recovered as the same
complete reading. Numeral property tests exercise independently generated values.
These are lexical laws: an indexed word or covered face is not evidence that
the construction grammar admits a card. Chart integration owns grammatical
composition, separator admission and generated construction roundtripping.
