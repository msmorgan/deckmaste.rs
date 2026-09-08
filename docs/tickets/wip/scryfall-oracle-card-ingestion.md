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

## Landing record

### Stamp

- Landed change: `tlrxopor`.
- Replacement coverage lock: 19,953 covered identities on Scryfall snapshot
  `ba4952fd7eae58b49f5dd8277968169902f79f8a289644c1ba2b4376cc53bb22`.
- Snapshot descriptor: updated `2026-09-08T09:01:57.053+00:00`, 38,634
  records, 24,535,940 compressed bytes, 202,887,682 JSONL bytes. The compressed
  SHA-256 is
  `70e3b5d9f4e5c9ee0c667f887fcca6afb99d441f284ba2cb0ba85c897adf956a`.

### Prove

The [coverage transition appendix](../../research/scryfall-coverage-reconciliation.md)
names and classifies all 309 identities retired from the old lock. Every one is
a source transition caused by moving from MTGJSON-derived normalized identities
to durable Scryfall Oracle face identities; no grammar regression or recoverage
ticket remains. Independent old/new face reconciliation started with 32,641 old
supported faces and 32,568 new supported faces. It found 32,553 exact identity
joins, 88 deterministic remaps, zero ambiguous joins, 675 upstream text changes,
zero legality changes, 148 face-grouping changes, zero real losses, and zero
unexplained additions. On 31,966 unchanged-text joins, grammar outcomes were
identical; no unchanged text changed result merely because its source context
changed.

The complete English v2 census preserved source bytes and satisfied both Reading
roundtrip laws. It recorded zero roundtrip mismatches, ownership failures,
construction-traversal failures, leaf-traversal failures, unresolved ties, and
internal failures. Expected and visited leaves were both 308,259; nonterminal
and visited constructions were both 883,487. All 24 active construction
checkers use permitted vocabulary classes, with zero forbidden classes and zero
environment-load errors.

The prepared local snapshot was used with the network namespace disabled and
`data/mtgjson` replaced by an empty temporary mount. In that environment,
catalog generation/check/text, card derivation, English v1 extraction and
migration generation, English v2 parse/coverage/roundtrip/ambiguity, lexical
analysis, and English v3 analysis all completed against the local Scryfall
JSONL. Card derivation emitted 41,851 face records. Ordinary analysis commands
therefore have no live AtomicCards or network dependency.

### Disclose

The new lock admits these eight newly covered identities and selected analyses:

- `0a3c7b103be9f148c447108815dcfc77f8060929406587b2031c9b7cb3c56c9d`
  — Gauntlets of Chaos: `{5}, Sacrifice this artifact: Exchange control of target artifact, creature, or land you control and target permanent an opponent controls that shares one of those types with it. If those permanents are exchanged this way, destroy all Auras attached to them.`
- `19741565f8ff846cbaede5d09769349c1a489d77ee21617104beeebaea088752`
  — Mishra, Lost to Phyrexia:

  ```text
  Whenever Mishra enters or attacks, choose three —
  • Target opponent discards two cards.
  • Mishra deals 3 damage to any target.
  • Destroy target artifact or planeswalker.
  • Creatures you control gain menace and trample until end of turn.
  • Creatures you don't control get -1/-1 until end of turn.
  • Create two tapped Powerstone tokens.
  ```

- `27fc29aa3f590ae64c02a47bc5477ced8451ee954fb97eb7fa320808a229fe7a`
  — Brisela, Voice of Nightmares:

  ```text
  Flying, first strike, vigilance, lifelink
  Your opponents can't cast spells with mana value 3 or less.
  ```

- `378b1f9c16a330dc702fac707eb25fa512a2bb90a5e19c1bdece77576eaea9e0`
  — Elixir:

  ```text
  This artifact enters tapped.
  {5}, {T}, Exile this artifact: Shuffle all nonland cards from your graveyard into your library. You gain life equal to the number of cards shuffled into your library this way.
  ```

- `3ec28f08eecfa7b01da728d48b6557a9c7af08247aa9a1eece0cd8c0617705b5`
  — Chittering Host:

  ```text
  Haste
  Menace
  When this creature enters, other creatures you control get +1/+0 and gain menace until end of turn.
  ```

- `a59d647b8779756370e6e811d59dfe6a75dda63e742f99b8f5297a201653d612`
  — Midnight Scavengers: `When this creature enters, you may return target creature card with mana value 3 or less from your graveyard to your hand.`
- `b01f875e71f6a13db066164250e757346e2a8c41e91036e78676bf16cfd3e9f0`
  — Bruna, the Fading Light:

  ```text
  When you cast this spell, you may return target Angel or Human creature card from your graveyard to the battlefield.
  Flying, vigilance
  ```

- `bfbc8c54c21825e6f98f82ca75507b227fd32f1392bf4b461164e1bf3e1f1e2f`
  — Ragnarok, Divine Deliverance:

  ```text
  Reach, vigilance, menace, trample, haste
  When Ragnarok dies, destroy target permanent and return target nonlegendary permanent card from your graveyard to the battlefield.
  ```

The appendix records their exact multiline source text. The last complete
parent census had 16,748 unique and 3,506 specificity-resolved analyses; this
landing has 16,965 unique and 2,988 specificity-resolved analyses, with zero
exception-resolved analyses, unresolved ties, or internal failures. The
specificity count and share fell, so this transition does not require a new
construction pair. The permitted construction count remains 24.

No grammar construction or glossary term was added. Supporting additions are
the snapshot prepare/reconcile commands and the transition appendix. Two
source-boundary regressions exposed by the full gate were corrected: loyalty
cost rendering now preserves Scryfall's `+N`/`−N`/`0` Oracle spelling, and the
legacy `Blink` inspection assertion now selects the supported face rather than
same-name unsupported auxiliary rows. The CLI help contract now accurately
describes Vintage-playable card-face names. A citation audit also replaced the
wrong-topic variable-loyalty citation with `[CR#107.7]`. No STOP or ruling
conflict remains.

Test assurance changed as follows: 10 logical regression tests were re-spelled
for Scryfall records and supported-face semantics; 10 new logical tests cover
the Scryfall reader, pre-analysis selection and manifests, missing selectors,
bounded 5,000-record selection, atomic snapshot recovery, and one-worker versus
multi-worker determinism. Zero meaningful tests were removed, restored, or
newly ignored.

### Report

- Coverage lock: 20,254 old covered identities to 19,953 new covered identities.
  The 395 construction declarations are unchanged.
- Homographs: vocabulary `AttributiveAdjective::Untap` beside the `untap` Verb
  keyword-action declaration, and vocabulary `TargetingMarker::Target` beside
  Noun lexeme `CommonNoun::Target`.
- Form literal/vocabulary overlaps: `additional` in `additional_cost` atom 2;
  `to` in `up_to_quantifying_determiner` atom 1; `the` and `next` in
  `definite_next_mass_quantity_reference` atoms 0 and 1; `to` in
  `scalar_less_than_or_equal_to` atom 4; `the` in `number_of_scalar_value` atom
  0; `the` in `greatest_scalar_value` atom 0; `other` in
  `other_than_qualified_reference` atom 1; and `the` in `positional_partitive`
  atom 0.
- Complete coverage performance: 159 seconds plus 449 milliseconds with four workers,
  131,867 ns per accepted byte, versus the 16.260-second advisory ceiling;
  host loads were 3.11/2.64/2.17. The result is accepted as a recorded source
  migration cost, with all semantic laws green.
- Lexical memory/work comparison: a two-face `Lightning Bolt` selection took
  about 2 seconds with 165,696 KiB peak RSS; the 32,568-face full scan took
  about 10 seconds with 1,024,692 KiB peak RSS.
- Full English v2 coverage: 19,953 covered, 12,615 explicit parse failures,
  zero ties or internal failures. Strict roundtrip and strict ambiguity checks
  passed. Strict parse emitted a complete report and exited nonzero as designed
  because the existing grammar does not yet parse those 12,615 faces.
- `cargo xtask gate --changed --run` passed the complete derived crate closure,
  including xtask's 501 passed, 0 failed, 1 ignored tests and the external Lean
  checker tests.
- Strict `cargo clippy` passed for every crate selected by the gate with
  `--all-targets -- -D warnings`; `cargo fmt --all --check` passed.
- `cargo xtask cite check --list-noncompliant` reported zero sites;
  `cargo xtask cite check` checked 15,847 citations with zero stale citations;
  the changed-diff citation audit passed.
- `fish tests/fetch_data_test.fish` passed all fetch, cache, validation, and
  failed-refresh recovery cases.
