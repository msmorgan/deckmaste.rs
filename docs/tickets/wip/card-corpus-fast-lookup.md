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
