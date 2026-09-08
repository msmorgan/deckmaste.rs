---
needs: []
---
# Replace AtomicCards ingestion with Scryfall Oracle Cards and bounded selection

Make [Scryfall Oracle Cards](https://api.scryfall.com/bulk-data/oracle-cards)
the repository's card-snapshot source. Stop requiring or reading
`data/mtgjson/AtomicCards.json` in live ingestion paths. The source switch must
make small development batches straightforward; replacing one whole-file
deserialization with another does not meet the request.

The descriptor inspected on 2026-09-08 exposes `jsonl_download_uri` for a
gzipped JSON Lines export, one card object per Oracle ID. Resolve that field
from the endpoint at refresh time; do not hard-code the dated download URL or
assume the old `download_uri`/JSON-array shape. Follow Scryfall's
[bulk-data contract](https://scryfall.com/docs/api/bulk-data) and verify current
fields at implementation. Cache the descriptor and export under ignored
`data/`, recording update time, schema/version policy and content hashes.
Publish a refreshed snapshot atomically only after download, decompression
and record validation succeed. Ordinary consumers use a pinned local snapshot
offline; they neither refresh silently nor make one API request per card.

Pinned shape: stream gzip/JSONL records with bounded buffering. Keep snapshot
models and decoding in the surviving `deckmaste_data` layer, catalog extraction
in `deckmaste_catalogs`, and orchestration/selection in xtask. Expose card/face
records without an AtomicCards-shaped compatibility envelope or a complete
in-memory name map. Do not put new ingestion infrastructure in deletion-bound
grammar crates or require the grammar to understand Scryfall's transport.

Preserve full-card identity and ordered Scryfall face records: consume top-level
`oracle_text`/`type_line` for single-face objects and the appropriate
`card_faces` fields for multiface objects without duplicating combined text.
Pin field inheritance per supported layout, including face-level Oracle IDs
where supplied, missing versus empty text/cost, mana value, colors, stats and
related meld records. Map every field actually consumed by extraction and
catalog generation; Scryfall does not supply the MTGJSON type arrays in the
same shape, so derive needed labels from declared catalogs/type-line structure,
preserving multiword subtypes. Do not silently fill unavailable data.
Keep Scryfall's `card_faces` representation distinct from the Game Model's
Card Face/Alternative Characteristics distinction, including flip and
adventurer cards.

Use Oracle identity plus a documented face discriminator for durable corpus
keys; printing `id`, display name and text hash are separate provenance/content
fields. Apply the existing Vintage legal-or-restricted scope using Scryfall's
legalities before grammar work. Preserve supported multiface/reversible/meld
relationships without admitting unrelated unsupported records. A changed
printing representative or text revision must not manufacture a new Oracle
identity. Ambiguous old-to-new identity joins are reported, never guessed.

Analysis commands accept explicit Oracle IDs, card names or an identity
manifest, and reproducible predicate-selected subsets. Apply selection before
expensive lexical/grammar analysis; bound worker queues and do not collect the
whole export merely to select a few records. Require a selection or explicit
`--all` for corpus analysis. Keep complete final verification available, and
stamp reports with snapshot hash, selection, face identities, worker count and
complete/subset/limited status so a small batch cannot masquerade as a census.
Missing requested identities fail visibly. Persist/export subsets as JSONL so
agents never need to rewrite a giant nested JSON document for iteration.

Migrate the live readers, defaults and fixture detection in `xtask::raw_corpus`,
`english_v3`, `lexical`, catalog commands, `derive_cards`, migration extraction
and generation, plus still-live v1/v2 corpus/provenance adapters. Adapt legacy
input plumbing only; do not redesign their grammar. Update `scripts/fetch_data`
and its Scryfall downloader, provisioning, current command help and agent-facing
subset instructions. Remove AtomicCards fetches and orphaned loaders after the
consumer inventory is empty; unrelated MTGJSON rules/reference/printing sources
are outside this replacement. Keep historical reports and done records intact.

Acceptance: run each inventoried live path with AtomicCards unavailable and
network disabled after snapshot preparation. Small synthetic fixtures cover
single-face, split/adventure/flip, transform/modal double-face, reversible and meld
mapping; legal/restricted versus unsupported; empty text; Unicode/newlines;
duplicate identities; malformed/truncated records; and failed-refresh recovery.
An instrumented many-record fixture proves bounded ingestion and that selecting
two records analyzes only their selected faces. The same selection produces
identical data/analysis results with one and multiple workers and after input
record reordering. Preserve meaningful regression assertions when re-spelling
fixtures for the new format.

On pinned old/new snapshots, reconcile every supported face and inherited
obligation: identity remapping, upstream text/legality changes, face grouping
changes and real losses are separate named outcomes. Compare grammar results
on unchanged text independently of source drift; never bless a smaller census
without explaining every loss. Verify byte-exact source preservation and both
Reading roundtrip laws under the
[English ADR](../../decisions/english-lexical-analysis.md#source-and-roundtripping).
Report peak memory and small-selection/full-scan work separately. Standard
constraints apply.
