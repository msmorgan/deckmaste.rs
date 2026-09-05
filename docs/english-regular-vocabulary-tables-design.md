# Regular vocabulary tables

## Goal

Give the English grammar broad lexical coverage without creating one Rust enum
variant and definition block for every ordinary word. The grammar must continue
to choose part of speech from its current slot; vocabulary data must not assign
one part of speech before parsing or make every word eligible for every slot.

## Approaches considered

1. Add thousands of named `Vocab` variants. This preserves the current model,
   but creates boilerplate and obscures the small set of words whose identity or
   morphology is actually special.
2. Store arbitrary strings directly in noun, verb, adjective, and adverb AST
   leaves. This is simple, but loses the shared lexical identity the AST and
   renderer need and invites opaque string leaves back into the grammar.
3. Keep named variants for special words and add a data-backed regular variant.
   This is the chosen approach. It preserves lexical identity while moving
   ordinary declarations into concise, auditable tables.

## Vocabulary data

The crate will contain a checked-in table with one `lemma<TAB>part_of_speech`
row per regular lexical use. A lemma may have several rows. Supported parts of
speech are:

- count noun;
- mass noun;
- count-or-mass noun;
- regular verb;
- adjective;
- adverb.

The table contains lemmas, not inflected surface forms. Existing noun and verb
morphology generates regular plurals, agreement forms, past forms,
participles, and gerunds. Closed-class grammatical words, catalog-only terms,
proper names, and irregular nouns and verbs stay in their existing dedicated
representations.

The initial candidate table comes from the supported Vintage corpus and is
classified by a bounded `gpt-5.3-codex-spark` extraction. It is input to local
validation, not an authority: malformed rows, catalog/name leaks, irregular
words, and unattested classifications are removed before integration.

## Runtime representation

`Vocab` gains a data-backed variant holding a `&'static str`. Existing named
variants remain available for grammar semantics and irregular definitions. The
lazy reverse-index build reads the static table and turns every row into a
slot-specific `WordMatch` for the same `Vocab` identity.

When a table lemma also has a named `Vocab` variant, the named identity is used.
Its explicit noun declension, inflectional form, countability, or initial-sound override
wins; the table may add another regular part of speech that the named definition
does not declare. This lets a word be promoted to or removed from the special
registry later without changing the AST shape.

The renderer uses the same table metadata and existing morphology, so regular
words round-trip without retaining source text or spans. No generic fallback
part of speech is added.

## Validation

Tests will require the checked-in table to be sorted, deduplicated, use only
known part-of-speech labels, and contain valid lowercase lemmas. Focused tests
will cover:

- a regular noun and plural;
- a mass noun;
- a regular verb and its generated forms;
- one word listed in multiple parts of speech;
- adjective and adverb slot isolation;
- a named special word overlapping the table;
- rendering without source text;
- adversarial unknown-boundary sentences that motivated complete vocabulary.

The full crate test suite, supported-card round-trip gate, and bounded
single-card performance checks remain required. This change does not alter the
chart engine or broaden unknown-phrase recovery.
