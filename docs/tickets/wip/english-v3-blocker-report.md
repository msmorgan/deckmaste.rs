---
needs: []
---
# Report why each unread face is unread: `cargo xtask english-v3 blockers`

Priority: high. Tooling only; no grammar or lexicon change.

## Why

The census records nothing about an unread face except its unknown words:
`lines` is empty, and no failing line, sentence or position is recorded. Work
has therefore been chosen by regex recon, which left 3,417 unread faces in no
bucket (wave-5 recon, 2026-10-08) and can only find constructions somebody
already suspected. At the per-line baseline (`sskyxnyvmruk`, 32,828 supported
faces, 6,292 unread) 2,027 unread faces carry at least one unknown word, and
nobody knows how many of them are unread for that reason alone. Owner
decision 2026-10-10: build the report before minting more construction
tickets.

## Goal

A subcommand `cargo xtask english-v3 blockers --census FILE [--json OUT]
[--limit N]` that, for every unread face in a schema-8 census, says which
units fail and where, and ranks the causes.

1. **Per-line verdict.** Split the face's analysed source into its ability
   lines and analyse each line on its own, with the face's card scope (the
   same scope `card_scope::for_face` gives the census and `probe --as-card`).
   Record for each line: reads or not, Reading count if it reads, and its
   unknown words.
2. **Localisation of a failing line.** For a line that does not read and has
   no unknown word, record the furthest input position the chart reached
   (the largest token index at which any item was scanned or completed from
   the line start), with the token at that position and the one before it.
   If the chart does not expose that today, expose it read-only from the
   chart statistics; do not change parsing.
3. **Blocker key.** Each failing line gets one key: `word:<lemma or lowercased
   text>` for each unknown word, otherwise `at:<previous token> <failing
   token>` from item 2. Keys are data for ranking, not grammar.
4. **Ranking.** The summary lists blocker keys by the number of faces they
   touch and by the number of faces where they are the *sole* blocker (the
   face has exactly one failing line and that line has exactly one key), with
   up to three example faces each. It also prints: unread faces, faces with
   exactly one failing line, faces whose every failing line is
   vocabulary-only, and line-level coverage over the whole census (lines that
   read / lines).
5. **JSON.** `--json` writes the per-face records (face id and name, per-line
   verdicts, keys) and the ranked table, so tickets can cite it and a later
   run can be compared.

## Analysis

Orchestrator rulings 2026-10-10:

- B1. Reuse the census's own line analysis (`analyze_line` and the segment
  logic that `english-v3-per-line-readings` added); the report must agree
  with the census: a face it calls fully reading must be covered in the
  census, and a covered face has no failing line. Assert that on the run and
  fail loudly on a mismatch.
- B2. Line splitting for an unread face cannot come from a Document parse
  (there is none). Split on the analysed source's newlines; where the census
  segments a covered face differently (quoted multi-line abilities, modal
  bullet lists), record the disagreement count in the summary rather than
  special-casing by card.
- B3. The report is diagnostic. It adds no field to the census schema and no
  gate; nothing may read it to admit or reject a Reading.
- B4. No Readings are materialised beyond what a line's count needs; run with
  the same worker pool as the census and stay under the census's peak memory.

- B5 (review ruling, 2026-10-10). Report keys use word tokens: maximal runs
  of letters, digits, apostrophes and hyphens; brace groups are single tokens;
  other non-space characters are individual tokens. The token containing the
  furthest position fails; whitespace and boundaries select the next token.
  Lowercase keys, preserving numerals. Each failing line records two tokens
  before and after it as context. Print three separately ranked tables: top
  60 vocabulary keys, top 80 position keys, top 40 failing tokens (ignoring
  previous tokens), with all three complete in JSON. Parsing is unchanged.

## Witnesses

Probe-level expectations, each written as an xtask test on a synthetic
two- or three-line card (never a pinned corpus count):

- a face whose second line contains an undeclared word: line 1 reads, line 2
  fails with key `word:<that word>`, sole blocker;
- a face whose only line is declared vocabulary in an ungrammatical order:
  key `at:<prev> <token>` at the first token the chart cannot pass;
- a covered face: no failing line;
- two failing lines: listed under both keys, sole blocker for neither.

## Method

Standard constraints apply. Deltas:

1. No grammar, lexicon or census-schema change. `cargo test -p xtask
   english_v3` and `cargo clippy -p xtask --tests -- -D warnings`; if the
   chart statistic needs a field in `deckmaste_english_v3`, that crate's full
   suite and clippy too.
2. Run it once on a current full census. If none of the claim parent exists,
   run one `--all --workers 6 --samples-per-face 0` census under
   `/usr/bin/time -v` (about 4 minutes) and keep it at
   `/home/msmorgan/Projects/deckmaste.rs/target/english-v3/baseline-blockers.json`
   with the report beside it as `blockers.json`.
3. Tests assert relations (which line fails, which key, sole or not), never
   incidental numbers or Debug-printed trees.
4. Document the command in the same place `diff` and `grep` are documented.

## Out of scope

- Sentence-level splitting inside a line, suggestions of fixes, and any
  clustering smarter than the two key kinds above. Mint a follow-up if the
  `at:` keys prove too coarse.

## Landing record

1. **Report summary** (original run `sqnyzpnl`; full 32,828 supported faces;
   schema 8). B5 rerun at `rwplnpkk` failed the required B1 agreement check:
   Neoform (`420c6dcf-966d-4a4c-a0ef-23037ab8b325#card`) is unread in the
   original census and covered by incoming `wlstpkpu` (the refresh diff
   records 0 to 263 Readings). The existing report artifacts remain intact;
   the summary below is the original character-key report, pending a ruling
   reconciling B1 with reuse of the pre-refresh census. Error evidence:
   `target/english-v3/english-v3-blocker-report/b5-blockers-errors.txt`.
   The original run passed coverage agreement for every face. Report and census used
   six workers; report peak RSS 11,877,316 KiB, below census peak
   14,952,824 KiB (`sqnyzpnl`). Artifacts in the coordinator workspace:
   `target/english-v3/baseline-blockers.json`, `blockers.json`, `blockers.txt`.
   After refresh (`qyllylkq`), the bounded selector combined the 1,607
   supported faces matching `grep 'equal to' -i`, 300 covered controls
   (seed 20261008), and the ten spot checks: 1,901 distinct identities.
   The stock census diff reports lost 0 / gained 31 / decreases 0 /
   increases 1 / digest-only 0; covered faces 1,384 to 1,415 and Readings
   194,476 to 202,878. The gains and Woodcaller Automaton's increase
   (273 to 285) come from incoming equality-property change `wlstpkpu`.
   The blocker report on the bounded census has zero coverage mismatches
   and zero internal analysis failures. Evidence: `refresh-manifest.json`,
   `refresh-after.json`, `refresh-diff.json`, and `refresh-blockers.json`
   under the feature workspace's `target/english-v3/english-v3-blocker-report/`.

    ```text
    5886 unread faces; 5035 with exactly one failing line; 1673 vocabulary-only faces
    Line coverage: 52196 / 59428; 440 covered-face segmentation disagreements
    Blocker keys (sole-blocker faces, touched faces; examples):
    at:\s t: 245, 274; Guul Draz Vampire | Heartwarming Redemption | Doubling Season
    at:\s {: 195, 238; Nissa, Worldsoul Speaker | Lattice Library | Saltcrusted Steppe
    at:e .: 177, 220; Syndicate Heavy | Greater Werewolf | Giant Oyster
    at:. <end>: 152, 180; Goblin Barrage | Cabal Therapy | Bloodthorn Flail
    at:\s n: 146, 171; Soul-Scar Mage | Glademuse | Tawnos's Coffin
    at:d .: 140, 178; The Foretold Soldier | Glimpse of Freedom | Stenn, Paranoid Partisan
    at:r .: 137, 159; Worldly Counsel | Muxus, Goblin Grandee | Karn, Living Legacy
    at:\s a: 121, 130; Muck Drubb | Twice Upon a Time | Akawalli, the Seething Tower
    at:n .: 113, 129; Sanguine Indulgence | Flashback | Eye of Duskmantle
    at:\s o: 109, 144; Eldrazi Confluence | Kotis, Sibsig Champion | Doomsday Confluence
    at:\s i: 106, 138; Pest Rescuer | Wail of the Forgotten | Verrak, Warped Sengir
    at:t .: 100, 116; Observed Stasis | Gustcloak Runner | Stunning Strike
    at:\s c: 92, 100; Welkin Tern | Departed Soulkeeper | Cloud Elemental
    at:\s m: 88, 96; Rashmi, Eternities Crafter | No Witnesses | Asinine Antics
    at:m \s: 69, 74; Treasure Map | Ulrich of the Krallenhorde | Ulrich, Uncontested Alpha
    at:y .: 67, 74; Coalition Relic | Green Sun's Twilight | Fountain of Cho
    at:\s l: 64, 94; Stormchaser's Talent | Beza, the Bounding Spring | Kasmina, Enigma Sage
    at:\s p: 63, 74; Benevolent Hydra | Goblin Gathering | Hulkling, Burgeoning Bruiser
    at:s .: 63, 65; Jace, the Perfected Mind | Primal Adversary | Conflagrate
    at:\s d: 55, 65; Kotis, Sibsig Champion | One with the Multiverse | Forethought Amulet
    at:\s w: 54, 56; Gorex, the Tombshell | Illusionary Mask | Karn, Scion of Urza
    at:\s f: 53, 59; Walking Sponge | Rime Dryad | Tomb Tyrant
    at:\s b: 50, 54; Ponder | Sealed Fate | Frenzied Raider
    at:\s y: 49, 55; Phial of Galadriel | Convalescent Care | Hoarding Broodlord
    at:\s e: 46, 59; Aura Graft | Omo, Queen of Vesuva | Hired Claw
    at:s \s: 46, 53; Phenax, God of Deception | Erebos, God of the Dead | Magister Sphinx
    at:— <end>: 43, 196; Saheeli's Artistry | Dawnbringer Cleric | Astarion, the Decadent
    at:e \s: 42, 51; The Book of Exalted Deeds | The Sound of Drums | First Response
    at:\s R: 42, 50; Elrond, Lord of Rivendell | Uruk-hai Berserker | Horses of the Bruinen
    at:p ,: 42, 46; Goblin Bomb | Goblin Lyre | Ral, Monsoon Mage
    ```

2. **Witnesses** (original counts `sqnyzpnl`; B5 keys/tests `rwplnpkk`). Synthetic line Reading counts: undeclared
   second line `[1, 0]`, key `word:zzyzx`, sole blocker; declared words in
   wrong order (`Draw Draw.`) `[0]`, key `at:draw .`, sole blocker; covered
   two-line face `[1, 1]`, no failing line; two failing lines `[1, 0, 0]`,
   keys `word:zzyzx` and `word:qqxyz`, sole blocker for neither. Additional
   tests preserve face-level deduplication for repeated keys and covered
   modal-list segmentation with a reported newline disagreement.

3. **Spot checks** (`sqnyzpnl`). Ten faces checked line by line with
   `probe --as-card NAME --text LINE --readings 8 --json`: Ancestral Recall
   `[1]`; Lightning Bolt `[1]`; Black Lotus `[1]`; Counterspell `[1]`;
   Brainstorm `[2]`; Guul Draz Vampire `[0]`, `at:\s t`; Nissa, Worldsoul
   Speaker `[2, 0]`, `at:\s {`; Syndicate Heavy `[1, 0]`, `at:e .`; Goblin
   Barrage `[0, 1]`, `at:. <end>`; Hua Tuo, Honored Physician `[0]`,
   `word:declared`. Every probe enumerated completely, agreed with the
   report, and had zero issues; every inspected Reading realized exactly.
   Evidence: the feature workspace's
   `target/english-v3/english-v3-blocker-report/spotcheck-*.json`.

4. **Deviations and STOPs**. Exposed the permitted read-only chart statistic
   `Metrics::furthest_position`; kept it out of the census schema. No
   grammar, lexicon, construction, frame or gate changed. Runtime tokens
   are Unicode scalars; B5 uses report-only word tokens to locate keys, with
   context kept outside the key. The original report minted
   `english-v3-blocker-word-context` because the `at:\s t` group combined
   *this*, *the* and *twice*; B5 resolves that follow-up within this ticket,
   so the superseded planned ticket is removed. Normalized the existing completed
   `english-v3-derived-punctuation-forms` dependency's quoted frontmatter
   ID; the adapter had treated the quotes as part of the ID. Kanban check
   now passes. Two extra tests cover deduplication and modal segmentation.
   Corrected one new test's mistaken frontier expectation against the packed
   probe: the chart reaches the second *Draw* and stops before the full stop.
   B5 re-spells that witness against the whole previous token, failing token
   and context, replacing the incidental scalar-position count. Three added
   tests cover Unicode/brace/numeral context, boundary selection and grouped
   failing-token deduplication. No glossary gap. STOP: the B5 full report
   fails B1 on Neoform after the grammar refresh; resolution pending. Tooling exception: no before/after grammar census or red witnesses
   for this implementation. Refresh brought equality-property grammar work
   (`wlstpkpu`); the bounded re-check found no lost faces or removed Readings.

5. **Tests** (`sqnyzpnl`). `cargo test -p xtask english_v3`: 53 passed (the
   initial run had 52 passes and the mistaken new expectation described
   above; the corrected run passes). `cargo test -p deckmaste_english_v3`:
   712 passed. `cargo clippy -p xtask --tests -- -D warnings` and
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`: both pass.
   Restored 0 / re-spelled 0 / ignored 0 / added 6 / removed 0; removals:
   none. B5 re-spells the wrong-order witness as described above. Citation checks: zero
   noncompliant and zero stale references; no newly cited rules.
   After refresh (`qyllylkq`), `cargo test -p xtask` in full passes:
   404 library, 4 binary, and 14 integration tests (422 total), zero
   failures. The existing ignored
   `macros::templates::tests::macro_schema_census_count_matches_21` remains
   ignored with its existing reason, "cross-checks the live corpus against
   the census; run on demand"; this change introduces no ignores.
   After refresh (`qyllylkq`), `cargo test -p deckmaste_english_v3` in full
   passes: 723 tests, zero failures and zero ignored. The pre-refresh
   clippy checks above passed; refresh verification follows the bounded
   census plus full-crate-test policy.
   B5 (`rwplnpkk`): `cargo test -p xtask english_v3` passes all 56 tests,
   zero failures and zero ignored. Added 3 / re-spelled 1 / restored 0 /
   ignored 0 / removed 0 in the review fix; cumulative added 9. The
   re-spelled wrong-order witness asserts the word-level key and context,
   as required by B5. `cargo clippy -p xtask --tests -- -D warnings`
   passes after replacing one new empty-context assertion with an exact
   empty-vector comparison, as clippy requires. No suppression or weakening.
   No additional census was run; the report rerun alone failed B1 as above.

6. **Timing** (`sqnyzpnl`). The one full census took **423340 ms**, six
   workers, prebuilt binary under `/usr/bin/time -v`, samples per face 0.
   `free -g` showed 82 GB available immediately before launch. Timing and
   verification logs are in the feature workspace's
   `target/english-v3/english-v3-blocker-report/`.
