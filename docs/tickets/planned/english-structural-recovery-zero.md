---
needs: [english-predicate-frames]
design: true
---
**[design] Finish supported-corpus English coverage at zero structural
recovery.** Use the role/token census established by
`english-structural-coverage` as the live queue. Completion means every one of
the 31,685 unique supported faces parses without whole-sentence/clause,
modal-header, embedded-rules, activation-cost, keyword-argument, or nominal
recovery. Licensed lexical opacity may remain only at a documented lexical
boundary and is reported separately.

## Remaining migrations

- Replace the `protection from` / `affinity for` name checks and first-token
  plausibility heuristic with declarative keyword-argument signatures. Cover no
  argument, quantity, structured cost or symbol sequence, noun phrase, PP with
  permitted preposition, and embedded ability shapes, both alone and in
  comma/semicolon-separated keyword lists. Unsupported shapes recover at the
  keyword-argument role.
- Replace raw-string `Cost::SymbolList` with the existing structured
  `OracleSymbol`/symbol-sequence representation, and let quoted or embedded
  abilities occupy licensed grammatical slots instead of special-casing only
  `with "..."`.
- Record adjective comparison capability/kind in vocabulary metadata instead of
  matching `fewer | greater | less | more | other` in grammar control flow.
- Carry canonical lemmas/forms of CR-derived keyword actions in catalog or
  vocabulary metadata instead of rewriting only `prepared` during resolution.
- Audit named regular `Vocab` entries after the metadata migrations and retain
  stable variants only for grammar semantics or irregular forms. Coordinate a
  mechanical module split with `english-split-word-module`.
- Confirm no tooling consumes the inactive `ast.rs`, `parser.rs`, `render.rs`,
  `source_debug.rs`, or `catalogs.rs` tree, then delete it.

## Campaign workflow

Take coherent, high-frequency grammar families from the live census. Each
completed slice must add a reusable typed production, populate shared metadata,
or document a genuine lexical exception, with causal positive/negative tests,
source-free rendering, and lower role/token totals. Before/after snapshots stay
ephemeral; no card text, identities, hashes, or aggregate baselines belong in
the repository.

Do not reach zero by splitting recovery spans, adding opaque semantic leaves,
or weakening lexical-slot constraints. Measure release-parser behavior when
chart search changes materially.

## Completion

- Structural recovery is zero in every reported role for every supported face;
  `cargo xtask english recovery --require-complete` succeeds.
- Keyword arguments and the remaining lexical facts use declarative shared
  metadata, and the inactive parser tree is gone.
- Supported-card round-trip remains an independent source-free invariant and
  required corpus loading remains enforceable.
- English, xtask, release-corpus, workspace, formatting, and clippy gates pass
  under the project's deliberate unboxed-grammar-enum policy.
