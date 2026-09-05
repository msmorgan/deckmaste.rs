---
needs: []
---
**The fused keyword surface lowercases its Keyword Quality in running text.**
`english-v2-landwalk-quality-keyword` landed the Bound Keyword Surface: the
Keyword Quality is read from the type declarations and the keyword stem is
bound to it, so *Islandwalk*, *Snow landwalk* and *artifact landwalk* all
analyze as one keyword-line item. Printed Oracle text writes that same fused
word in lowercase wherever it is not line-initial — *Enchanted creature has
mountainwalk.*, *Target creature gains islandwalk until end of turn.*,
*Creatures with forestwalk can be blocked…* — and the land Subtype declaration
has no running-position lowercase reading, so those units stop at the fused
word.

Measured on the corpus at that landing: 199 faces carry a basic fused walk
surface, 123 carry a capitalized one, and **76 carry only lowercase ones**.
Sampled failures stop inside the fused word, not elsewhere in the text
(Burrowing at bytes 40..52, Crevasse 15..27, Deadfall 15..25, Coral Barrier
80..90).

What to decide and build: how a declaration-backed Keyword Quality realizes
and reads in the lowercase running-position fused form, without a per-surface
row and without a guard naming a type or a card. The candidate shape is a
case rule on the Bound Keyword Surface construction — the fused word is one
lowercase orthographic word except where sentence casing capitalizes its first
letter — not a second lowercase surface on each type declaration. Rendering
must stay byte-exact in both cases, and the capitalized readings this landing
covers must not change their selected analysis.

Vocabulary: `docs/contexts/oracle-english/CONTEXT.md` (Bound Keyword Surface,
Keyword Quality). `[CR#702.14a]` gives the surface; `[CR#702.14c]` gives the
separated and stacked readings.

Tier: **sol**. Standard constraints apply.

## Landing record

Measured after `kata refresh` on feature change `tttnptln`, against fork point
`wlpmzlsm`. The fork-point and feature ambiguity reports used identical
`--json --require-resolved --workers 8` flags. Development used the affected
surface/current-analysis subset and focused tests; the only pre-gate full
corpus pass was the required fork-point ambiguity baseline.

### PROVE

- **Decision and mechanism.** Declaration-backed words already expose an
  initial surface alongside their running surface, and the scanner selects
  between them by case position. The Bound Keyword Surface now reuses that
  mechanism: when a right-adjacent declaration term is sealed as
  `BoundSuffix`, the scanner can read the quality's initial declaration surface
  in running position and the writer applies its existing `CasePosition` to the
  bound orthographic word. No host, keyword construction, declaration row, or
  per-type lowercase surface was added: the 52 gains select under four
  pre-existing hosts — `GrantedAbilityLexicalVerbPhraseGrantedAbilityLexicalVerbPhrase`
  (39, `has`/`gains`), `PrepositionalPhrasePrepositionalPhrase` (5, `with`),
  `KeywordLineKeywordLine` (6, a lowercase item after a `;`-separated keyword),
  and `BareKeywordLineItemBareKeywordLineItem` (1).
- **Required analyses.** The stable-identity diff is 52
  `parse_failure -> selected` transitions, 0 losses, 0 identities present on
  only one side, and 0 changed selected paths among all 20,002 prior
  selections. All 52 gains contain
  `BoundQualityKeywordLineItemBoundQualityKeywordLineItem`, the `Landwalk`
  keyword-ability declaration, its `lexeme:keyword_ability/Landwalk/bound_suffix`
  ownership of `walk`, and the declared Keyword Quality shown below. The 102
  previously selected Bound Keyword Surfaces have 0 changed paths. The
  lowercase fused-surface wrong-path count is 0.
- **Byte-exact rendering and ownership.** Focused witnesses cover `has
  mountainwalk`, `gains islandwalk until end of turn`, and `with forestwalk`;
  each parses, renders byte-exactly, selects the Bound Keyword Surface, and
  visits both `Landwalk` and its declared land Subtype. The pre-existing
  `Denimwalk` negative remains a parse failure. `roundtrip --require-clean`
  independently reported 20,054 accepted, 20,054 clean, 0 mismatched, and
  12,587 not accepted.
- **Structural laws.** Coverage reported 32,641 total units; 20,054 selected
  and covered; 0 selected-uncovered, unresolved ties, internal failures,
  exception resolutions or uses, roundtrip mismatches, ownership failures,
  traversal failures, leaf-traversal failures, gaps, overlaps, synthetic
  claims, or provenance-plan mismatches. Construction traversal was exact at
  890,915 expected/visited nodes and leaf traversal exact at 311,251
  expected/visited leaves. The environment and its collision checks loaded.
- **No word-naming guard.** Production behavior is licensed only by the sealed
  `BoundSuffix` feature and boundary direction. No predicate, `require`,
  construction, declaration row, or exception names a word, type, lexeme,
  construction, or card. Coverage reported 0 forbidden licensing checkers.

### DISCLOSE

- **Selection census.** Selected 20,002 -> 20,054; unique 16,631 -> 16,680;
  specificity-resolved 3,371 -> 3,374; unresolved ties, internal failures,
  exception resolutions, and exception uses remained 0. The gains comprise 49
  unique and 3 specificity-resolved units: Erhnam Djinn, Orbweaver Kumo, and
  Wrexial, the Risen Deep. None is a negative oracle.
- **Deviation and additions.** The existing case mechanism could not tell a
  generic suffix-direction `Bound` atom that its preceding declaration term
  and following declared suffix form one cased orthographic word. The sealed
  compiler feature is therefore extended once, generally: a suffix-direction
  boundary immediately before a declaration term carrying `BoundSuffix`
  emits `bind_declared_suffix`. The English writer records the preceding
  word's case position, lowercases its first scalar only in continuation, and
  adjusts collected byte spans if Unicode case mapping changes the byte width.
  The scanner indexes declared bound-suffix surfaces and admits the existing
  initial declaration reading only at that feature-backed adjacent boundary.
  Added one compiler emission test, one writer case test, and one three-witness
  end-to-end test. Re-spelled the obsolete `Snow Swampwalk` witness as `Snow
  swampwalk` to preserve the same orthographic law.
- **Gate rerun.** The first refreshed closure run passed tests but strict
  clippy found one function-length finding and two cast findings in the new
  code. The helper extraction and checked signed conversions fixed them; the
  permitted post-fix closure rerun was green. The second `kata refresh` was a
  no-op, so the refreshed base did not move between fix and final gates.
- **STOPs:** none. There is no tie, wrong newly covered analysis, newly covered
  negative oracle, unexplained loss, roundtrip mismatch, or word-naming guard.
- **Glossary gaps:** one, closed by this landing. The **Bound Keyword Surface**
  entry in `docs/contexts/oracle-english/CONTEXT.md` named the surface but
  stated no case law for the word it forms, which is the whole subject of this
  ticket; the entry now says quality and bound surface are cased together as
  one word, capitalized only sentence- or line-initially. **Keyword Quality**
  needed no change.

#### Newly covered identities and selected analyses

Every identity below selects
`BoundQualityKeywordLineItemBoundQualityKeywordLineItem` with the `Landwalk`
declaration's `BoundSuffix`; the value is its selected declared Keyword
Quality, followed by the resolution kind. Compound and mixed faces list every
quality used by the selected Bound Keyword Surfaces.

- `980e56f370093fd2dceb311b58b896fca01df97a70b7569335bcd2eaa8d7db48` — Aysen Highway — plains — unique
- `ad093d6562b9dc2b5bb88e06aac10dc9c9bbfff7c967b7daee1e25972565dc2a` — Burrowing — mountain — unique
- `4a3e3b99bda3cd6c3ac805a4cf023747e9914abd0e6475b8f3af0a0d67743c0e` — Cave People — mountain — unique
- `5a2050ad634805d2dd08140ebb85edace4c3731862152c49b4af123572fe852f` — Cave Sense — mountain — unique
- `a7cea01f932587b69de7083c35158068d3c41237fec1d75c90926fc14b41fabc` — Chasm Skulker — island — unique
- `4ec2bc350dc5ccc18600482bfcd33030a175b225e3b9e68ffe4ce4ea0a11df72` — Coral Barrier — island — unique
- `848ae6862cbce0ae97a1781209dd5534b6956c7f8cc47e9a746ff052183360ed` — Deeptread Merrow — island — unique
- `4e6a397dfeae542201d47eae5ed34d3873fa764dcb42751bc810eef348df9c94` — Dryad's Favor — forest — unique
- `d1fb927bb5239d3ed82bd9dc9a07e559b28f8b2d9c049c620e5071bb980482be` — Dwarven Pony — mountain — unique
- `0eda53decd0bffbb99da390e1b3fe9a6a19d54ffe73e5bfcb658bacb1a1fc175` — Eladamri, Lord of Leaves — forest — unique
- `7a2d78a735c96c954c54e1f6850b8bf8f65b8c4fc6f3b2b4821fdd28b66ca02b` — Elvish Champion — forest — unique
- `29c48a5a15860dfbf316e300c208cccd763835cf4e1cc341074003f5de0af04a` — Elvish Pathcutter — forest — unique
- `885cccc5dbaee1052cdec09eeb44464e780673cd9e9d88c6f94baacbd7e2840f` — Enslaved Scout — mountain — unique
- `a1ee743dc24a3613f459ae50dfbe7619fb6db4ec43e8c94ba5d2db90c9facad1` — Erhnam Djinn — forest — specificity
- `9a1a2280e9c889f86e05ec338301da1e7b95a4481f1c369c3a09b6e7d8e010dd` — Fishliver Oil — island — unique
- `b09e2775cd4fa9ba8167a2ad612d1d3b6ff96bdcdcff211a2f3493e042cf842f` — Funeral Charm — swamp — unique
- `487a68e7856a41456329449a4ead20814f4d3854218e9475ecc9737aedf9a554` — Goblin King — mountain — unique
- `437ca9817794d61cd3cbbbfef76c496dffed62d12807ae1ff193085f7975bcaf` — Goblin Scouts — mountain — unique
- `6d1e0461dafffbc20ecf1bbd85cf7532469565858419e24656221ecda8b324a2` — Hidden Path — forest — unique
- `1287ad856e059b836084478474fc55f7a1eb0bc020f1b0c12e2182396fb94918` — Ivy Dancer — forest — unique
- `b0cba4d02496f55c30c2f9260870ab46f63978c81592f7f65810413b520c2d21` — Jedit Ojanen of Efrava — forest — unique
- `24ca5976be68288164ce21625000fb809b4ffed0e6c6990e23816cc6e9eaf0b3` — Jedit Ojanen, Mercenary — forest — unique
- `c92a3c5595db06e6e3784d95788dea3388cd19650f476009f9f797ba80bffc1c` — Legions of Lim-Dûl — snow swamp — unique
- `7e9d059a28010450362e487fb14a91120ace34fad4fc2356d30f5af5f4af82f2` — Leshrac's Rite — swamp — unique
- `426401300f49f834a621592058cd5f236332efa9ade7326899f1db40a8cafa9e` — Lord of Atlantis — island — unique
- `412a476810b51dc52ee0864c60d1416d0305e75256b8e3a1fe079e964fcb8e43` — Lumbering Satyr — forest — unique
- `d5a464777830a09e9544507dbaeec5652552716be1a84020d097b1f72cd0c4bc` — Master of the Pearl Trident — island — unique
- `e7af6338fbc464c082cd7a5397fa72eefa3b83c0378e60f3737ad874a8a1ccca` — Merfolk Assassin — island — unique
- `fcae7b76828c88e5f00b044a09bf00488e8a19bee467cb8ff0f02790abc70453` — Mirri, Cat Warrior — forest — unique
- `3490eeaf6daecae20586ad34abb231f90794d3c510175f458d2e468105cc0d64` — Nature's Cloak — forest — unique
- `4ec405662522cc8c47b6287959c47975d753ab8359ad0075909286faaac79c25` — Nighthaze — swamp — unique
- `02f418eebec75723d7fe5f610c2e329f1c06f65a02e6d1f09fb2fc1308aa9113` — Orbweaver Kumo — forest — specificity
- `d34070c7fd76fd8676f9edd91e02ee05efbff3c7a7890ab94e69597cca7b74c7` — Part Water — island — unique
- `7d9e8343dc6d87e61d788f70b54137a57b9eb5d4836b0e83bb1cb4c03b21a2ea` — Piracy Charm — island — unique
- `f097c18eda4ca891bb8fbbc6dc10ac078c830fb5973ce174942bbb70a8c9e686` — Restless Bones — swamp — unique
- `ef0516af0f8a34e6d2714a2dcb5e2125948c2b310a30e235958a559d2810ad31` — Rime Dryad — snow forest — unique
- `a70093383ed309877a7e43e11df19added356a7a6be226b1c29c629f8a019867` — River Merfolk — mountain — unique
- `e9c280802aec03df798797871710a79af980c5de85113a361298ba2af016d61f` — Shore Snapper — island — unique
- `5c0eccb43dc0ba09dc4b66839e7656ac33cfa9a95bb50eef8559c87d7e2773ed` — Stalker Hag — forest + swamp — unique
- `a2f1e568b646bdb2b89c4a419ab12d326226e8c567bf9e2fe71aef7c93a8461d` — Streambed Aquitects — island — unique
- `215fd60b261c13bc6c7979b1bbfb9197acb1bcb9a7f67944000e148214ee8422` — Tattermunge Duo — forest — unique
- `a6bfdf7ba5148d7eadffd5fbffe6a0612ce1cdc9e408283a7d0d955d034c0af5` — Unseen Walker — forest — unique
- `50007460668ec39d50184ca6478c117eac667298028d414b6023f700a24e02b1` — Veldrane of Sengir — forest — unique
- `e10d710d2c32b66ff9db1c8d9ec2bca25a8236c20f3e45e972b4326591ea42e5` — Viscid Lemures — swamp — unique
- `a6b6a2749e1adcc647915ec76c017ed56cdc8640e99a2f35329da9cd2355aef1` — Volcanic Strength — mountain — unique
- `14d852bdf4a7064e13fdca092964ca0724da27ec83a37839c2ec787014e3db63` — Weatherseed Elf — forest — unique
- `a75030b5b14694ad7220d3739464621ae539298fc4cb93ffc2d0042cba62e333` — Wormwood Dryad — forest + swamp — unique
- `a78251f82a76eb011108ef52374ec9534b3b0c56ca774b1790d52beedee62b6d` — Wormwood Treefolk — forest + swamp — unique
- `6d0af54f3b61bdece83f83090bff4f1d9e6a918534df9b6aee7633ab49f8e8dd` — Wrexial, the Risen Deep — island + swamp — specificity
- `7e9e910d2c7ea6eb087be2ca8966fae9a1048c7d79eddfb58e06e5f8c13cc75e` — Zombie Master — swamp — unique
- `46e40a24b75d178aca2ea6a1dfb7ac6808b4c2c94c002bff2fcf3026c8621bc3` — Zombie Master // Zombie Master, face Zombie Master — swamp — unique
- `ef6e401f92f2bfefc44e3127607f9ede39f211f114eaf0aabb0b09074f96d553` — Zombie Trailblazer — swamp — unique

### REPORT

- Coverage lock in report mode: 20,002 -> **20,054**, +52 / -0. Both
  `coverage --check` and `coverage --bless` printed `newly covered 52 corpus
  identities`; neither printed a no-longer-covered section. `--bless` wrote
  the lock to exactly this tree's selected covered set.
- Construction declarations: 397 -> **397**. No construction, frame, or
  `require` was added to hold an inventory value. Permitted licensing checkers:
  23; forbidden: 0.
- Licensed vocabulary/lexeme homographs, reported not fitted: 2 —
  `AttributiveAdjective::Untap` beside the `Untap` keyword-action Verb
  declaration; `TargetingMarker::Target` beside `CommonNoun::Target`.
- Form-literal/vocabulary overlaps, reported not fitted: 9 — `additional` at
  `additional_cost` atom 2; `to` at `up_to_quantifying_determiner` atom 1;
  `the` and `next` at `definite_next_mass_quantity_reference` atoms 0 and 1;
  `to` at `scalar_less_than_or_equal_to` atom 4; `the` at
  `number_of_scalar_value` atom 0; `the` at `greatest_scalar_value` atom 0;
  `other` at `other_than_qualified_reference` atom 1; `the` at
  `positional_partitive` atom 0. Longest form literal: 11 bytes.
- Closure gate scope printed and ran:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`
  and
  `cargo clippy -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings`.
  Final tests and strict clippy were green. `cargo fmt --all` was clean.
  `cargo xtask catalogs check` printed `catalogs are up to date`. Cite gates
  were not run because no citation changed.
- Assurance counts: restored 0; re-spelled 1 existing witness; ignored 0;
  added 4 tests (3 by the implementer, 1 rejection test added in review);
  removed 0. The closure's one ignored test is pre-existing and untouched.
  The re-spelled witness is justified: real Oracle prints the stacked form
  lowercase (Legions of Lim-Dûl reads `Snow swampwalk`), so the retired
  `Snow Swampwalk` spelling was never valid Oracle and is not a second valid
  case to keep as a witness.
- Performance advisory, workers 8. Coverage `--check`: 171,726 ms,
  **218,347 ns/B**, host load 18.80 / 21.98 / 14.48, with 0 visible
  `cargo`/`rustc` processes at launch. Coverage `--bless`: 199,775 ms,
  244,772 ns/B, host load 19.22 / 24.33 / 17.25. Feature ambiguity: 147,144
  ms, 191,293 ns/B, host load 14.31 / 20.81 / 17.07. Roundtrip: 145,698 ms,
  176,489 ns/B, host load 19.59 / 20.50 / 17.54. Every pass exceeded the
  16,260 ms quiet-host ceiling under the reported load; this is advisory and
  was not fitted. The sandbox cannot observe sibling executor processes, so 0
  is only the directly visible cargo/rustc count. **True contention at the time
  of these passes, supplied by the coordinator: 2 concurrent codex executors
  plus 2 other Opus reviewers.** Review-side subset passes on the same tree
  measured host load 41.85 / 27.66 / 20.29 (coverage) and 33.74 / 26.56 /
  20.04 (ambiguity), consistent with that count.

### Review corrections

Reviewed on feature change `tttnptln` (lock `covered` 20,054), corrections
committed above it.

- **MEDIUM — the compiler Deviation had a positive test but no rejection
  test.** `right_adjacent` occurs exactly once in the whole english_v2 grammar
  (`bound_quality_keyword_line_item`), and that one use *is* declaration-backed,
  so nothing anywhere held the `declared_bound_suffix` gate closed: flipping it
  to always-true left every suite green. Added
  `a_bound_suffix_without_the_sealed_feature_keeps_plain_space_suppression` in
  `crates/deckmaste_construction_core/src/emit/render.rs` — the same grammar
  with the declaration term sealed `Fixed` instead of `BoundSuffix` must emit
  `writer.suppress_next_space()` and must not emit `bind_declared_suffix`. The
  test was mutation-checked: with the gate forced true it fails.
- **MEDIUM — the record misattributed the hosts.** It said the granted-ability
  host owns `has`, `gains` and `with`. Measured on this tree, the 52 gains
  select under four pre-existing hosts (39 / 5 / 6 / 1); `with` is a
  `PrepositionalPhrase` host and 7 units select under keyword-line hosts. The
  mechanism claim (no host added) is unchanged and holds. Record corrected.
- **MEDIUM — a glossary gap was reported as none.** The **Bound Keyword
  Surface** entry stated no case law for the word it forms. Amended in
  `docs/contexts/oracle-english/CONTEXT.md`; the DISCLOSE line now names it.
- **Contention stamp added** to the performance advisory (2 concurrent codex
  executors plus 2 other Opus reviewers), which the implementer's sandbox could
  not observe.

#### Disclosed known limit (LOW, not fixed here)

At a bound-suffix boundary the scanner still admits the **capitalized** quality
surface in running position, because a land Subtype's running surface is itself
capitalized: `Enchanted creature has Mountainwalk.` and the retired
`Snow Swampwalk` both still parse and select the Bound Keyword Surface, and the
writer then renders them lowercase, so the unit fails the ownership
byte-exactness check rather than being refused at scan. That is the loud
failure mode, not a silent one — `ownership_failure_units` is a structural law
the coverage gate holds at zero — and the corpus has no such unit: the only two
occurrences of a capitalized fused walk after a word (`with Islandwalk`,
`with Plainswalk`) are inside rulings text, not oracle text, so `roundtrip
--require-clean` is clean at 0 mismatched. Refusing the capitalized reading at
that boundary is a tightening the ticket did not ask for (it asked to add the
lowercase reading, not to remove the capitalized one) and it would move the
102 pre-existing Bound Keyword Surface selections, so it is disclosed rather
than done. Nothing is owed: no coverage is lost and no gate is weakened.

#### Review gates

Re-run once on review change `kzznpxol` (lock `covered` 20,054), after the
final `kata refresh`, which was a no-op — `jj diff --from wlpmzlsm --to
default@` is 0 files, so trunk did not move under this claim and no incoming
diff needed re-gating. The review commit's only Rust change is a `#[cfg(test)]`
test at `crates/deckmaste_construction_core/src/emit/render.rs:7449`;
production code is byte-identical to `tttnptln`, so the implementer's stamped
selection-neutrality proof (0 changed paths of 20,002; 0 of 102 prior Bound
Keyword Surfaces) carries to this tree without re-measurement.

- `cargo fmt --all -- --check`: clean.
- `cargo xtask gate --changed` printed, and this ran:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`
  — 39 test binaries, all `test result: ok`, 0 failed, 1 pre-existing ignored.
- `cargo clippy -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings`: clean.
- `coverage --check` (`DECKMASTE_COVERAGE_LOCK=report`): 32,641 total; 20,054
  selected and covered; 0 selected-uncovered, ties, internal failures,
  exception resolutions/uses, roundtrip mismatches, ownership failures,
  traversal or leaf-traversal failures, gaps, overlaps, synthetic claims,
  provenance-plan mismatches; 890,915/890,915 nodes; 311,251/311,251 leaves;
  23 permitted and 0 forbidden licensing checkers. No newly-covered and no
  no-longer-covered section printed: the lock is exactly this tree.
- `ambiguity --require-resolved`: 0 unresolved ties, 0 internal failures, 0
  exception resolutions, 0 exception uses, 12,587 parse failures.
- `roundtrip --require-clean`: 20,054 accepted, 20,054 clean, 0 mismatched.
- `cargo xtask catalogs check`: `catalogs are up to date`.
- `cargo xtask cite check --list-noncompliant`: 0 non-compliant.
  `cargo xtask cite check`: 14,490 citations, 0 stale. No citation was added or
  changed by the landing or the review.

Review-side performance advisory, workers 8, on review change `kzznpxol`,
under the true contention above: `coverage --check` 161,243 ms,
**205,323 ns/B**, host load 18.96 / 25.66 / 23.78; `ambiguity` 151,963 ms,
**181,929 ns/B**, host load 20.23 / 24.19 / 23.54; `roundtrip` 155,197 ms,
**151,470 ns/B**, host load 26.76 / 22.73 / 22.91. Each exceeds the 16,260 ms
quiet-host ceiling under that load; advisory, not fitted.
