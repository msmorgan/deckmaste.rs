---
needs: []
---
# By-name card lookup by substring scan; precomputed nickname catalog

Normal priority (user decision 2026-10-07; replaces `lexicon-nickname-precompute` and a rejected SQLite-sidecar design).

## Why

A startup profile on 2026-10-07 (instrumentation kept in workspace `english-v3-load-profile`, commit `mqwtzryt`, not integrated) shows a single-card `cargo xtask english-v3 --card-name …` run spends about 2,400,000,000 ns before parsing the card, which itself takes about 9,000,000 ns. Two full parses of the 203,205,300-byte `data/scryfall/oracle-cards.jsonl` (38,705 records) dominate it, at opt-level 2 after the 2026-10-07 profile fix (3,700,000,000 ns before it):

- about 750,000,000 ns in `crates/xtask/src/raw_corpus.rs` (`CorpusSelectionArgs::load`, line 75, into `load_selected`, line 176, which runs `OracleCardReader` over every record at line 191) to select one card by name;
- about 640,000,000 ns in `crates/deckmaste_lexical_source/src/card_names.rs` (`load`, lines 13-41, called from `src/lib.rs:53`), which streams the same file again to derive 1,806 legendary nicknames from 36,821 titles and 4,263 legendary units.

## Part A: by-name selection by substring scan (xtask)

- For `--card-name`, find candidate records by scanning the raw JSONL bytes for the literal field as the file spells it: compact JSON, `"name":"<name>"` with no space after the colon. Encode the needle with the same JSON escaping the file uses (names containing `"` or `\`). Double-faced and split cards carry the top-level name `A // B` (for example `"name":"Galathul Galecaller // Corvid Squall"`); a face name such as `Corvid Squall` appears only inside `card_faces`, and `--card-name` matches display (face) names as well as group names (`raw_corpus.rs:204-205`), so the face-name needle must still find its record.
- The scan is only a candidate filter. `"name":"Lightning Bolt"` occurs on 6 lines of the current file: the same string appears inside other records' `all_parts` related-card entries and inside `card_faces`. Deserialize every candidate line through `OracleCardReader`'s validation and select by the exact top-level `name`, or by the face's name when the request addresses a face, using the same match rules `load_selected` applies today. Never select by first hit.
- When no candidate line matches exactly, fall back to the full stream, so a wrong or missing name errors exactly as today (`ensure_found`, `raw_corpus.rs:247`).
- `--all` keeps streaming. `--oracle-id`, `--face-id` and identity manifests may use the same scan with the Oracle ID as the needle; state whether they do.
- No new dependency. `memchr` is in `Cargo.lock` transitively (via `regex`), not a declared workspace dependency; either declare it directly (same locked version, no new crate) with a one-line justification, or use std.
- Expected: the about 750,000,000 ns selection parse drops to the cost of one byte scan of the file. That must be measured; it is likely under 100,000,000 ns from the page cache.

## Part B: precomputed nickname catalog (lexicon)

- The generation step writes the derived nickname list, plus anything else `card_names.rs` needs, as a generated catalog under `data/gen/catalogs/`, keyed to the corpus file's size and SHA-256 with a stale check that fails with a clear "regenerate with `cargo xtask …`" message. Note: `data/` is wholly gitignored, and `data/gen/catalogs` is produced today by `cargo xtask catalogs` (`crates/xtask/src/catalogs.rs`, default output `data/gen/catalogs`), loaded by `deckmaste_catalogs::CatalogSet`; `cargo xtask generate` generates plugin cards. Put the nickname catalog in `cargo xtask catalogs` and follow its conventions.
- `card_names::load` reads the catalog instead of streaming the JSONL. `--all` must then parse the corpus once, not twice.
- The nickname set and the lexical inventory SHA-256 are identical, proven by test.

## Method

Standard constraints apply, plus:

- Land as a series: Part A; Part B generator; Part B lexicon consumer.
- Tests: a name occurring in another record's `all_parts` is not selected, with Lightning Bolt as the fixture (the scan finds 6 candidate lines; exactly the one Lightning Bolt record is selected); a face-name lookup on a double-faced card selects that face; a missing name still errors via the full-stream fallback; scan selection equals full-stream selection for a sample of names including escaped ones; catalog nicknames equal the streamed computation on the current corpus; a stale catalog is detected.
- The selected faces are unchanged, and the full `--all` census is identical face by face against the claim parent.
- Provisioning: `scripts/provision-workspace` symlinks `data` from `default`, so the catalog under `data/gen` is shared by every workspace. State whether generation belongs in the provisioning hook or only in `cargo xtask catalogs`, and what a fresh `default` checkout must run.
- Timings before and after as integer ns for `--card-name` and `--all`, with host load and worker count. Evidence under `target/english-v3/`.

## Out of scope

The macro_ron double parse (`macro-ron-single-pass`). Any change to the corpus file itself. A sidecar database or other index file for selection.

## Landing record

Standard tiers. Plus: the dependency statement, the tests above, the inventory SHA and census comparison against the claim parent, and the before/after timings.

**PROVE — preservation.** Baseline executable/source tree: `smnmpwpryoxuyyyvrwlutzxuupkuyzll` (claim parent); consumer tree: `ztnqlyknzkxmnmlnqlunplmsvsuuwtor`. Both have corpus `covered` 21,072; an English coverage-lock `covered` stamp is not applicable because v3 consumes no English coverage lock. All figures below use these stamps unless explicitly qualified and refer to their pre-refresh trees retained at operation `4077a67b3503`; the stable change IDs were subsequently rebased. The baseline report's `change_id` is sampled at completion, after implementation began; `measurement-provenance.json` records the actual baseline binary/source tree rather than that late working-copy head.

The complete `--all` census is identical face by face over **32,828 faces**: **11,756 no Reading, 7,464 one, 13,608 multiple, 0 undetermined**. Identity losses `[]`; gains `[]`; changed analyses `[]`; newly covered identities/analyses `[]`. Comparison includes every non-timing face field, complete reading counts, chart/materialization metrics, construction inventories, source metadata, checked trees, lexical identities/provenance and traversal samples. Both runs enumerate without a reading limit and retain two samples per face. The normalized face-hash inventory SHA-256 is `f75bf34f4fbaf2d8694df96c0e04eb30dcf2c5c35fb56e858fc829a81149f7b9`. Validation issues and internal reading failures are zero. Corpus validation preserves byte-exact realization, lexical ownership and traversal; package tests retain independently constructed-value roundtrips.

The generator-stage test compares the new nickname derivation directly with the old streamed loader on the pinned corpus. Both yield **1,806 nicknames**. The consumer inventory test reconstructs the nickname forms from a fresh stream and compares the complete lexical inventory digest. Before and after lexical inventory SHA-256: `3f6533411916afd863234508e82c7d3b249a5b7ef48a960c692d4d81877ab642`. All 14 existing catalog files are byte-identical after generation. The new artifact is keyed to **203,205,300 bytes** and corpus SHA-256 `49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`.

No grammatical admission/licensing checker or word-naming guard is added: forbidden additions 0, permitted additions 0. Grammar declarations and authored vocabulary are unchanged; existing loader-error and admission assertions remain in the gate. The v3 command does not emit the retired v2 permitted-checker inventory, so that total is not remeasured here.

The derived gate is:

```sh
cargo test -p deckmaste_catalogs -p deckmaste_english -p deckmaste_lexical_source -p deckmaste_migrations -p deckmaste_spelling -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
```

`cargo xtask gate --changed --run --clippy` ran that test closure: **2,622 passed, 0 failed, 1 existing ignored**. The original Clippy phase reported doc-markdown and a redundant result wrapper; both were corrected, then strict Clippy over the same eight packages passed. The ignored test remains `macros::templates::tests::macro_schema_census_count_matches_21`, with its existing blocker “cross-checks the live corpus against the census; run on demand”. The final selector-module rerun verifies the formatting/encoding refinements. Stable fmt check, catalog check, citation check (0 stale, 0 noncompliant), and the piped citation audit pass; the moved shortened-name citation was read against its rule text.

Assurance counts attributable to this ticket: restored 0; re-spelled 3; newly ignored 0; added 10; removed 0. The three existing nickname tests move from lexical source to catalogs with unchanged assertions; their alias lookup now calls the generator. They are `oracle_short_names_preserve_multiword_prefixes_and_use_the_numeral_codec`, `only_standalone_short_references_in_pinned_oracle_text_become_aliases`, and `embedded_fragments_are_not_short_reference_attestations`. New integration modules are registered in package `tests/main.rs`, under `tests/suite/`; no additional top-level integration binaries were introduced.

**DISCLOSE — selection, tests and scope.** Only name-only selections use the candidate scan, including name-only identity manifests. `--all`, Oracle IDs, face identities, predicates, and mixed selector requests retain streaming. Exact group/display-name matching, supported-card filtering, deterministic ordering and subset export are shared with the full reader. Missing names or rejected candidates fall back to the full stream and its existing errors. Name lookup reads the snapshot into memory; streaming selections remain bounded by records.

The pinned Lightning Bolt needle finds six candidate lines. Exact matching selects one ordinary Lightning Bolt record and the legitimate Lightning Bolt face of `Emeritus of Conflict // Lightning Bolt`, exactly as the claim-parent stream does. Related-card mentions and the unsupported art record are excluded. The ticket's Corvid Squall example is not Vintage-legal in this snapshot; the positive real face lookup uses `Insectile Aberration` from `Delver of Secrets // Insectile Aberration`. The synthetic face/group tests additionally preserve full-group lookup and quote, backslash and Unicode name escaping. Missing-name tests preserve the original `ensure_found` error and prove the full-reader fallback. No supported face was dropped to fit an example.

Deviations and additions: zero constructions added/deleted. Five selector tests cover false hits/export, escaped names/group matching, a real face, fallback, and mixed compact/whitespace/Unicode/slash JSON encodings. The last closes an additional loss case: a compact hit must not hide another exact name encoded differently. Three catalog integration tests cover map preservation, changed size and same-size changed bytes, and missing/malformed/old-version artifacts. One generator/stream equality unit test and one whole-inventory integration test complete the 10 additions. The catalog directory writer is shared so the new artifact is published with the canonical files while retaining the previous output-validation order and rollback tests. Catalog checking stages under ignored `target/`. An accidental broad nightly format pass was restored before closing the generator change; only task files remain changed. No ruling contradiction or unresolved STOP occurred; no new glossary concept is needed (the spellings remain Spelling Variants).

Dependency statement: `memchr` is declared directly at the existing locked version 2.8.2 for the substring scan; no crate/version is added to the lock. Catalog generation reuses existing `serde`, `sha2`, and `deckmaste_lexical` dependencies, keeping the original Numeral codec and nickname algorithm. No selection index or database is introduced.

Generation belongs only in **`cargo xtask catalogs generate`**, not the provisioning hook. A fresh default checkout runs that command after installing its rules/Oracle snapshots and after any Oracle snapshot replacement. Missing/stale artifacts fail with that regeneration command. Generation was run here; `data/gen/catalogs/card-name-nicknames.json` is shared through provisioned workspaces' `data` symlink. `--all` deserializes the corpus once; nickname loading hashes source bytes for freshness without parsing them.

**REPORT — provenance and performance advisory.** Ordinary dev profile; timings exclude compilation. Full-corpus wall times use different worker counts and host states, so their speed difference is not attributed to this IO change. Initial loaded-host after measurements are retained alongside the quieter repeats.

| Measurement / tree stamp (`covered`: corpus 21,072; lock not applicable) | Command wall | Setup wall | Workers | Host load at start (1 / 5 / 15 min) |
|---|---:|---:|---:|---|
| Before card / `smnmpwpr` | 2,393,609,529 ns | 2,214,320,372 ns | 1 | `7.858 / 14.072 / 15.493` |
| After card, initially loaded / `ztnqlykn` | 3,403,614,613 ns | 2,898,057,774 ns | 1 | `35.073 / 28.420 / 23.725` |
| After card, warmed 1 / `ztnqlykn` | 1,361,129,752 ns | 1,172,549,072 ns | 1 | `5.060 / 7.217 / 13.634` |
| After card, warmed 2 / `ztnqlykn` | 1,351,716,185 ns | 1,172,371,595 ns | 1 | `4.975 / 7.163 / 13.583` |
| After card, warmed 3 / `ztnqlykn` | 1,344,094,656 ns | 1,161,296,626 ns | 1 | `4.975 / 7.163 / 13.583` |
| Before all / `smnmpwpr` | 1,407,009,189,283 ns | 2,535,304,690 ns | 1 | `7.469 / 13.888 / 15.426` |
| After all / `ztnqlykn` | 231,640,060,406 ns | 4,227,750,134 ns | 8 | `35.073 / 28.420 / 23.725` |

The quieter card median is **1,351,716,185 ns**, against the baseline **2,393,609,529 ns** (one worker each; advisory across the listed loads). The final paired selector benchmark, one worker at host load `4.606 / 8.458 / 15.895`, retains identical faces: candidate scan **92,472,138 ns**, complete candidate selection **243,161,207 ns**, full-stream selection **776,285,896 ns**. All three final candidate scans are below 100,000,000 ns; the scan deserializes six records instead of 38,705. Earlier loaded-host timings are retained in the evidence.

Before full census (`smnmpwpr`, corpus covered 21,072; lock not applicable): corpus wall **1,396,362,037,710 ns**, **1 worker**, host load `7.469 / 13.888 / 15.426`, checked-text thread CPU **462,366 ns/B**. Command wall exceeds the **16,260,000,000 ns** quiet-host advisory ceiling.

After full census (`ztnqlykn`, corpus covered 21,072; lock not applicable): corpus wall **215,542,780,690 ns**, **8 workers**, host load `34.987 / 28.513 / 23.781`, checked-text thread CPU **554,531 ns/B**. Command wall exceeds the **16,260,000,000 ns** quiet-host advisory ceiling.

Grammar census, unchanged on the two measured trees: 205 ordinary constructions, 45 shared schemas, 45 instances; 210 construction names occur in the exhaustive Reading census. Homograph additions/removals `[]`/`[]`; form-literal/vocabulary-overlap additions/removals `[]`/`[]`. Their inherited inventories are not independently reclassified for this IO change; declaration identity and the complete lexical inventory SHA are preserved. No English coverage lock is consumed or changed.

All measurements, source snapshots, comparison scripts and reports are under this workspace's ignored `target/english-v3/card-corpus-fast-lookup/`, never `/tmp` or tracked source directories. They are retained under default's corresponding ignored target directory when the workspace is retired. `census-comparison.json` hashes every face while streaming the two large reports; `catalog-comparison.json`, `measurement-provenance.json`, the paired selector timings, and the gate logs retain the supporting evidence.

Refresh incorporated `nzvnsktk` / `ossltroo` (the separately landed Where supplement) and the coordinator claim `suprnuvn` without conflicts. The incoming declaration edit is outside this IO change; the corpus measurements above retain their pre-refresh provenance. On the refreshed tree, `cargo test -p deckmaste_english_v3 -p xtask` passed **757 tests, 0 failed, 1 existing ignored**, including all **366 English v3 integration tests**. Strict Clippy for those packages with all targets and warnings denied, stable fmt, and citation checks also passed. The refresh logs are retained with the evidence.

A second refresh incorporated the separately landed `semantics-v2-deed-is-a-name` series (`lpulmpkm`, `mykqutqm`, `nzkqnwxo`, `lozxzott`, `rwxsynww`, completion `yurpoqvz`) and claim `twnmsonp`, again without conflicts. It changes xtask consumers and Lean/plugin data, so `cargo test -p xtask` was rerun: **391 passed, 0 failed, 1 existing ignored**, including all 14 integration tests and the cold Lean build. Strict all-target xtask Clippy and stable fmt passed. The English v3 package was unchanged by this refresh; its preceding 366-test result is retained. This refreshed tree is retained at operation `c80a207f54a7`; the two incoming landings do not alter the pre-refresh corpus measurement stamps.
