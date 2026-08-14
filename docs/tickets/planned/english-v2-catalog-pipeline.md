---
needs: [english-construction-rewrite-design]
---
**Build the catalog pipeline for `english_v2`'s open-class identity slots:
`cargo xtask catalogs generate`, generated data files with provenance, a
loader that binds catalogs to a parser instance, and a staleness check.**
Stage 1 of the rewrite's implementation sequence.

Decisions already made (2026-08-13 design dialogue; the ADR produced by
`english-construction-rewrite-design` is the authority if anything here
drifts):

- A catalog is a pure word list feeding an `identity` slot in the new
  grammar. Anything needing per-entry grammar (e.g. keywords with their own
  syntax) becomes a construction instead, and the card's own name is a
  parse-context parameter passed at parser construction — neither belongs in
  a catalog.
- Catalogs are generated DATA files (RON), never generated Rust enums: a set
  release must be a regeneration diff, not a code review.
- One authoritative upstream per catalog, named in its provenance header: the
  CR snapshot in `data/rules/` for subtype lists, card types, supertypes,
  ability words, and keyword names; the local MTGJSON snapshot for card names
  and counter kinds.
- Every generated file carries a provenance header (CR version/date or DB
  snapshot id). A staleness check fails when the fetched CR is newer than the
  provenance of any CR-derived catalog.
- The parser binds catalogs at instance construction — they are inputs, not
  compiled-in globals.
- Extraction code lives in `deckmaste_migrations` beside the existing
  snapshot parsers (reuse the legacy bare-text extractor where it fits); the
  catalog FILE-FORMAT types live in `deckmaste_english_v2` (consumer-owned,
  so generator and loader cannot drift). That is a tool-side dependency edge
  migrations→english_v2 only; english_v2 gains no dependency in return and
  stays a leaf.
- Command family: `cargo xtask catalogs generate` and `cargo xtask catalogs
  check` together as one group; the existing bare-text generation is renamed
  `cargo xtask catalogs text`.

Deliverables:

1. `cargo xtask catalogs generate`: deterministic regeneration of the initial
   inventory — subtype lists per class (creature, land, artifact, …), card
   types, supertypes, ability words, keyword names, card names, counter
   kinds.
2. The crate skeleton for `deckmaste_english_v2` containing only the catalog
   loader module (a catalog set handed to parser construction), with a smoke
   test loading every generated catalog. The grammar itself lands in the
   vertical-slice ticket.
3. The staleness check, wired so a CR bump without regeneration fails.

Acceptance: regeneration is byte-deterministic for a fixed upstream; every
catalog has provenance; the staleness check demonstrably fails on a stale
fixture; the loader smoke test passes. Standard constraints apply.
