---
needs: []
design: true
---
**[design] Establish enforceable structural-recovery accounting for the English
parser.** This ticket supersedes the measurement and completion contract from
`english-unknown-min3`. It deliberately does not claim that supported-corpus
coverage is complete; the remaining grammar work is tracked by
`english-predicate-frames` and `english-structural-recovery-zero`.

The old campaign measured `UnknownPhrase` leaf length. That metric can improve
when one opaque span is merely split into shorter leaves, while a long licensed
catalog identity is not necessarily a grammar failure. Its checks also could
not enforce their stated result: `cargo xtask english unknown --min-words 4
--limit 0` exited successfully for any count, source-independent rendering was
satisfied by copied fallback spelling, and the release corpus test succeeded
when its required corpus was unavailable.

## Delivered contract

- Parse-failure recovery is represented separately from explicitly licensed
  lexical opacity.
- Recovery is counted once at its outermost responsible grammatical role:
  clause, nominal material, activation cost, keyword argument, modal header, or
  embedded rules.
- Recovered source-token totals are the primary invariant; occurrence count and
  maximum span length remain diagnostics. Splitting a recovered span cannot
  improve the token total.
- `cargo xtask english recovery --json` exposes a machine-readable live census,
  and `--require-complete` fails while structural recovery remains.
- Corpus snapshots are ephemeral working data. No card text, face identity,
  hash, or aggregate baseline is stored in the repository.
- The release supported-card test fails when its required corpus cannot be
  loaded.

The initial live census covered 31,685 unique supported faces and 821,355
source tokens. It reported 14,622 structural-recovery spans containing 241,092
source tokens, separately from 630 one-token opaque nouns. These values are an
ephemeral starting measurement, not a checked-in ratchet or acceptance
baseline.

## Follow-ups

- `english-predicate-frames` replaces predicate spelling checks and globally
  permissive dependent attachment with shared declarative lexical frames.
- `english-structural-recovery-zero` uses this census as the live work queue,
  completes the remaining metadata migrations and parser cleanup, and owns the
  durable zero-structural-recovery gate.

## Completion

- Recovery and lexical opacity are distinct in the syntax and reporting APIs.
- The census reports stable role/token totals in human- and machine-readable
  forms and has an enforceable nonzero exit mode.
- A focused test proves recovery-span splitting cannot lower recovered tokens.
- Required release-corpus absence is a failure rather than a skipped success.
- Parser, public API, and xtask tests pass.
