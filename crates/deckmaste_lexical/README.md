# Independent lexical analysis

`Lexicon::new` freezes declared lexemes into analysis and realization indexes.
`analyze` returns every licensed lexical occurrence; `realize` accepts a lexical
value independently of source text. This crate has no construction, catalog-I/O,
game-model or grammar-compiler dependency. The governing contract is
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

The xtask source adapters read the existing construction vocabulary, authored
plugin morphology and generated catalogs. Exported RON declarations can be loaded
directly by this crate. Original owner identities and source paths survive export.
Catalog phrases remain whole entries; their internal words are not added as new
lexemes. Unmapped inventories and construction literals remain named source gaps.
The supplement in `english_v2/src/lexical_supplement.ron` adds noun/determinative
`one` and supplies explicit irregular replacements under existing verb owners.

Ordinary verbs acquire the regular paradigm before overrides. Old verb
`Unavailable` markers based on nonattestation are reported as retired source
policy; they do not suppress forms. Existing modal inventories still need
explicit applicability declarations, and are reported rather than treated as
regular verbs. Supplemental keyword-derived adjectives remain an adapter gap.

## Occurrences and spelling

The lossless atomic sequence uses Unicode scalars. Lexical matches are edges
between scalar occurrences; this permits overlapping multiword forms and bound
prefixes/suffixes without committing to a word split. Whitespace and punctuation
remain in the sequence. Byte ranges are derived when requested and are absent
from lexical value identity. Raw and externally normalized text are kept distinct.
This ticket's corpus command analyzes raw text, including reminders.

Free forms require word edges; Unicode combining marks belong to those words.
`Binding` explicitly relaxes the left, right or both edges. Multiword separators
must match the declared spelling exactly. `Capitalization::Initial` adds a
first-scalar uppercase realization when it differs from the declared spelling;
the reading records that choice. It does not license arbitrary case folding.
The grammar will constrain where an initial realization is appropriate.

The numeral codecs recognize cardinal, ordinal, Arabic and Roman notation
alongside ordinary lexical entries. They never claim exclusive ownership of
`one`, `I` or any other spelling. A numeric reading retains value, notation and
case. Checked realization rejects lossy Roman values: finite magnitudes through
3999 and the two integer extrema roundtrip; intermediate saturated values do not.

## Corpus accounting and inspection

```sh
cargo xtask english_v2 lexical --data data/mtgjson/AtomicCards.json \
  --output /tmp/lexical-inventory.json --export /tmp/lexemes.ron
cargo run -p deckmaste_lexical --example inspect -- /tmp/lexemes.ron 'counters'
```

Use a supported-card subset during development. The report retains raw face
text, input digests, byte occurrences, overlapping material classes, unknown
words, unrecognized nonword material and unmapped source declarations. Ordinary
vocabulary, catalogs, keywords, numerals and symbols are accounted separately.
A word is covered only by a contiguous lexical path, including a containing
multiword entry; crossing overlaps alone cannot hide a remainder.

Every returned occurrence must realize exactly to its source substring. Every
indexed lexical value is also realized independently and recovered as the same
complete reading. Numeral property tests exercise independently generated values.
These are lexical laws: an indexed word or covered face is not evidence that
the construction grammar admits a card. Chart integration owns grammatical
composition, separator admission and generated construction roundtripping.
