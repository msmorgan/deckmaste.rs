---
needs: []
---
Continue the bottom-up Oracle-text grammar campaign until every supported card
has no `UnknownPhrase` leaf longer than three words.

## Starting state

The 2026-07-21 derived snapshot contains 31,685 supported faces. All of them
structurally round-trip after the parser's input is dropped; rendering receives
only the printed card name and whether the card is legendary.

The current census has 24,177 unknown leaves, of which 18,793 exceed three
words. The maximum is 69 words. Length buckets are:

```text
0–1=4452  2=534  3=398  4–5=1387  6–10=4049  11–20=10263  21+=3094
```

The parallel corpus gate takes 29.666s with one Rayon worker and 4.604s with
the default pool on the development machine. The corresponding unknown census
takes 28.06s and 4.45s. Treat performance as a guard against order-of-magnitude
regressions, not a requirement that each grammar improvement be free.

## Parser invariants

- Supported means Vintage `Legal` or `Restricted`; Un-cards are excluded.
- Keep packed recognition keyed only by symbol, span, and finite grammatical
  features. Lower meanings after selecting the best derivation.
- Select parts of speech from grammar slots through the vocabulary and catalog
  registries; tokenization must not assign a global part of speech.
- Unknown recovery stays limited to noun, sentence, cost, and other explicitly
  restrictive slots. Do not replace spans with supposedly opaque semantic
  leaves to make the gate pass.
- Reminder text is stripped at the input boundary. Self references and the
  three typographic quote characters are normalized there as well.
- The structural gate must remain source-independent: the AST cannot retain or
  receive the original Oracle text for rendering.
- Preserve punctuation through syntax/rendering without dedicated punctuation
  AST leaves.

## Workflow

Use the command output as the live work queue rather than copying phrase lists
into this ticket:

```text
cargo xtask english unknown --min-words 4 --sort-count --exemplar
```

Take coherent high-frequency or short-bottom-up families, add causally
sensitive grammar and rendering tests, run the supported-card structural gate,
then rerun the census. Measure each slice on the release parser when it changes
chart search behavior. Commit accepted slices with the `english:` crate prefix.

The full gate is:

```text
cargo test --release -p xtask local_card_snapshot_structurally_round_trips_without_source_text
```

Set `RAYON_NUM_THREADS=1` when a single-worker comparison is needed.

## Completion

- The structural gate passes all supported faces without source text.
- `cargo xtask english unknown --min-words 4 --limit 0` reports zero phrases
  over three words.
- `cargo test -p deckmaste_english`, `cargo test -p xtask english::`, and the
  relevant workspace checks are green.
