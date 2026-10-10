# Restore named bullet lists after sentence lines

## Why

The named-modes landing `pxtwrtvu` retired the wrong Flavor Word analysis
whose label included the bullet. Its dash-header selector missed ten covered
anchor-word faces with sentence-line headers, so those faces silently lost all
Readings. The regression needs a replacement named-Mode analysis.

## Goal

Restore the nine Siege faces and Typhoid Mary, Fractured through the existing
modal/list route. Keep the bullet outside every label, preserve the named-mode
suite, and verify all supported bulleted faces plus random covered controls.
Standard constraints apply.

## Orchestrator rulings (2026-10-10)

- **NM-R3:** A named bullet list may follow a complete ordinary Sentence line
  as well as a dash header. Extend the existing modal/list route by the smallest
  refinement, with every item a named Mode. Do not restore bullet-in-label
  Flavor Words, name a word/card in a guard, or add a Construction when an
  existing form suffices.
- **NM-R4:** Restore all ten faces with the named-Mode analysis on every bullet.
  Counts may differ from the retired wrong analyses; report each count and the
  grammatical alternatives that multiply.
- **NM-R5:** Select every supported `(?m)^• ` face plus 300 random covered
  controls (seed 20261008). Compare with `baseline-wave8mid.json` at
  `spqtlyqwwvot`, restricted to the manifest, or rerun the claim parent.
  Require all ten restored, zero lost, and disclose every other changed shape.
  Audit `diff-wave7-wave8mid.json` for other losses/decreases attributable to
  named modes. Full English suite and clippy; one structural test for each
  restored family. Refresh code/data requires rebuild, bounded census and full
  suite. Stop before integration.

## Landing record

1. **Census delta** (`spqtlyqw` before, `lqmpuwls` after): all 730 supported
   `(?m)^• ` faces plus 300 random covered controls, seed 20261008, with five
   overlaps: **1,025 distinct faces**. Native diff: covered **796 → 808** after refresh;
   Readings **34,949 → 35,070** after refresh; ten own valid gains plus two
   trunk gains, lost **none**, decreases **0**,
   increases **0**, issues **0**. The **782 digest-only** changes add the explicit Debug `category` field
   to existing Paragraph, ParagraphContinuation, Modes, ModeContinuation and
   ModalItem values because these now use shared schemas, while retaining
   the same Category assignments and grammatical alternatives, so Reading
   counts are unchanged. Refresh brings
   `nxxknqvk` (remove Source attachment): Bill Ferny, Bree Swindler 0 → 12
   uses *from combat* as a Source Complement of *remove*; Pyramids 0 → 48
   uses the Source-free Object frame with *marked on it* as a bare passive
   Postmodifier. Both retain ordinary, unlabelled Modes under dash headers.
   The native refresh-only diff finds all ten restorations unchanged and no
   losses, decreases, increases or further digest changes. The accepted-review
   refresh brings keyword-state-adjective changes (`ptvtlvss`, `qkvxplko`,
   `tqsuwzum`, `onvptpwn`) and Saddle grammar data; its native diff is
   unchanged on all 1,025 faces (`after-accepted-refresh.json`,
   `accepted-refresh-diff.json`, `final-accepted-diff.json`). Integration was
   refused because default moved; the retry refresh brings `wwzpptst` and its
   follow-up checks (play-with-absolute), again leaving all 1,025 faces
   unchanged (`after-retry-refresh.json`, `retry-refresh-diff.json`,
   `final-retry-diff.json`). The native
   comparison with `baseline-wave8mid.json` finds the before subset identical
   on all 1,025 faces. Evidence: `target/english-v3/english-v3-named-modes-regression/`
   (`manifest.json`, `before.json`, `after-refresh.json`, `final-diff.json`,
   `refresh-diff.json`, `baseline-vs-before.json`). The supplied `diff-wave7-wave8mid.json` contains
   exactly these ten lost faces and zero decreases: **no other lost or
   decreased face is attributable to named modes**.
2. **Witnesses** (`spqtlyqw` probe counts all zero; `lqmpuwls` counts below):
   each restored face is now one Ability. Its count is the product of its one
   header Reading and the independently retained Mode-body Readings, rather
   than the old wrong analysis's sum of separate Ability-line counts.

   | Face | Old wrong count | Restored count | Header × Mode bodies |
   |---|---:|---:|---|
   | Barrensteppe Siege | 3 | 1 | 1 × 1 × 1 |
   | Citadel Siege | 4 | 2 | 1 × 1 × 2 |
   | Frontier Siege | 5 | 4 | 1 × 2 × 2 |
   | Frostcliff Siege | 6 | 6 | 1 × 3 × 2 |
   | Glacierwood Siege | 8 | 12 | 1 × 4 × 3 |
   | Hollowmurk Siege | 3 | 1 | 1 × 1 × 1 |
   | Monastery Siege | 14 | 30 | 1 × 3 × 10 |
   | Outpost Siege | 4 | 2 | 1 × 2 × 1 |
   | Palace Siege | 4 | 2 | 1 × 1 × 2 |
   | Typhoid Mary, Fractured | 4 | 1 | 1 × 1 × 1 × 1 |

   Old counts are those of the supplied pre-wave diff (`mqrlzszkuszn`);
   each body factor follows label order in item 3. All probe enumerations
   complete with zero issues. Controls remain plain modal 1, named modal 1,
   ordinary Flavor Word head 1, standalone named bullet 0.
3. **Spot checks** (`lqmpuwls`, all ten gains): Document → ModalItem at Ability
   → SentenceParagraph header and NamedModes; every bullet is Mode →
   FlavorWordHead → OrdinaryAbility → Paragraph. Labels exclude the bullet:
   Barrensteppe (Abzan / Mardu); Citadel (Khans / Dragons); Frontier
   (Khans / Dragons); Frostcliff (Jeskai / Temur); Glacierwood (Temur / Sultai);
   Hollowmurk (Sultai / Abzan); Monastery (Khans / Dragons); Outpost
   (Khans / Dragons); Palace (Khans / Dragons); Typhoid Mary (Mary /
   Typhoid Mary / Bloody Mary). No wrong-analysis gain.
4. **Deviations and STOPs** (`lqmpuwls`): added **two features** (SentenceLineList, ModeListLayout), **four Categories**
   (SentenceParagraph, SentenceContinuation, NamedModes, NamedModeContinuation),
   **five schema instances** (one each of Paragraph, ParagraphContinuation,
   Modes, ModeContinuation and ModalItem), and **zero Constructions or Frames**;
   there are no lexical edits. These five existing Constructions become shared
   schemas and retain their original instances. The Construction/shared-schema
   total at accepted review was unchanged at **301** (235 ordinary Constructions
   + 66 shared schemas), already above the ultimate **250** ceiling; this
   refinement adds zero toward that ceiling. The integration-retry refresh
   brings two ordinary Constructions from `wwzpptst` (play-with-absolute),
   taking the trunk total to **303** (237 ordinary + 66 shared schemas);
   neither addition belongs to named modes. Named-list and complete-sentence instances express NM-R3. LabelKind checks
   require every named member; ModeListLayout distinguishes the two header
   forms. SentenceLineList prevents the whole sentence-line Ability from
   becoming a labelled Mode body. The first draft allowed nested lists in
   Mode bodies and increased 41 covered faces; it was corrected before tests,
   and the final diff has zero increases. The two layout terms are added to
   the Oracle English glossary, citing NM-R3 as a project ruling. Eight
   existing fixture values receive explicit Paragraph Category tags; their
   subjects and assertions remain unchanged. No STOP remains unresolved.
5. **Tests** (`lqmpuwls`): `cargo build -p xtask` passed;
   `cargo test -p deckmaste_english_v3` passed **861 tests across all six
   binaries**, run once after implementation; and
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed.
   After refresh, `cargo build -p xtask`, the bounded census and full
   `cargo test -p deckmaste_english_v3` passed again (**874 tests**);
   `cargo test -p deckmaste_lexical_source` passed **83 tests**.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Refresh brought grammar, lexical-source data and tests from `nxxknqvk`;
   there were no conflicts. After the accepted-review refresh, the same build,
   bounded census, full English-v3 suite (**879 tests**), lexical-source suite
   (**83 tests**) and clippy commands passed again; additionally
   `cargo test -p deckmaste_semantics_v2`,
   `cargo clippy -p deckmaste_semantics_v2 --tests -- -D warnings` and
   `cargo test -p xtask english_v3` passed for incoming plugin grammar data.
   The integration-retry refresh repeats those exact commands: English-v3
   **889 tests**, lexical source **83**, plugin reader **105**, xtask English-v3
   **56**, and all three clippy checks pass; build and bounded census also pass.
   Two family tests assert labels and Paragraph
   bodies, reject mixed lists,
   and independently construct the sentence-line ModalItem. The existing
   named-mode module is unchanged. Restored 0 / re-spelled 0 / ignored 0 /
   added 2 / removed 0 tests; no removals. Fixture tag adaptations preserve
   existing expectations. Citation checks find zero stale and zero
   noncompliant sites; the piped citation audit selects zero changed sites;
   no new CR rule citation requires blessing. Logs are under the evidence path.
6. **Timing** (`lqmpuwls`): the bounded after-refresh census took **33.19 s wall**
   under `/usr/bin/time -v`, six workers, zero samples per face; the accepted-review
   refresh re-census took **34.65 s wall**, and the integration-retry re-census
   took **31.03 s wall**, with the same settings. No full census
   or Lean check was run.
