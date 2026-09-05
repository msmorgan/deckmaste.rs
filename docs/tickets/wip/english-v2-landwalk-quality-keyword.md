---
needs: []
---
**Landwalk as one keyword ability with a land-type quality.** The canonical
keyword-ability catalog is the Comprehensive Rules section-702 headings and
carries `Landwalk` alone; fused surfaces such as *islandwalk* are legacy atomic
variants that the catalog deliberately excludes
(`crates/deckmaste_catalogs/src/lib.rs::canonical_keyword_abilities_exclude_all_legacy_atomic_variants`
pins `"Islandwalk"` out of it). So english-v2 must reach the printed walk
surfaces through the `Landwalk` declaration plus a quality, never through
per-surface `KeywordAbility` stubs.

Minted by the `english-v2-lexical-inventory-2026-09-05` review under the
coordinator ruling of 2026-09-05 (Q1 = keep the exclusion, drop the row).

`[CR#702.14a]`: landwalk "appears within an object's rules text as
'[type]walk,' where [type] is usually a land type, but it can also be the card
type land plus any combination of land types, card types, and/or supertypes".
`[CR#702.14c]` gives the separated readings by example — "artifact landwalk",
"nonbasic landwalk", "snow swampwalk". (`[CR#702.14b]` is only "Landwalk is an
evasion ability" and does not bear on the surface, so the separated forms cite
`[CR#702.14a,702.14c]`, correcting the sub-rule letters named at mint time.)

What to build:

- A keyword-ability construction reading the `Landwalk` declaration with a
  land-type quality, realizing **both** shapes: the fused surface `<Type>walk`
  (*islandwalk*, *desertwalk*) and the separated `<Quality> landwalk`
  (*Nonbasic landwalk*, *Legendary landwalk*).
- **The quality is read from the type declarations.** A hard-coded list of the
  five basic land words is ruled against; so is a `require` or `checked by`
  naming any walk surface, land type or card. The basic-land subset is not the
  printed set, which is why the row that motivated this ticket was
  misclassified as a data line.
- The realization must round-trip byte-exact in both shapes, including the
  capitalized line-item form.

Expected units (counted with `jq` over `data/mtgjson/AtomicCards.json` at mint
time): the **146** identities the retired lexical row was covering, plus
`Desertwalk` 2, `Nonbasic landwalk` 3, `Legendary landwalk` 2. Out of scope
unless they fall out for free: the joke/silver printings `townwalk` 1,
`Planeswalkerwalk` 1, `Denimwalk` 1.

Open question, deliberately not resolved here: `macro-keyword-templates` says
"typed landwalk stays bare keyword names in the bespoke parser". Read in
context (`docs/keyword-policy.md` §4, where Landwalk carries
`quality: Default(Predicate, Type(Land))`) that sentence governs the **v1**
macro-template layer and its slot-reader codec, not the v2 grammar — the v1
shape is in fact the shape this ticket asks for. If the claimant reads it as a
constraint on v2, STOP and say so rather than resolving it.

Tier: **sol**. Standard constraints apply.

## Landing record

Measured after `kata refresh` on feature change `rrrzuukt`, with coverage lock
`covered = 19,947`. The fork-point ambiguity report and this tree's ambiguity
report used identical `--json --require-resolved --workers 8` flags. The one
full corpus pass before the final round was the explicitly required fork-point
baseline, captured at claim; development otherwise used the 194-unit affected
subset.

### PROVE

- **No silent loss.** The stable-identity ambiguity diff is exactly 102
  `parse_failure -> selected` transitions, 0 losses, 0 identities present on
  only one side, 0 changed selected paths among previously selected units, and
  0 changed resolution kinds among previously selected units. Coverage report
  mode printed `newly covered 102 corpus identities` and no no-longer-covered
  section: 19,845 -> 19,947, +102 / -0.
- **Structural laws.** `coverage --check` reported 32,641 total units; 19,947
  selected and covered; 0 selected-uncovered, unresolved ties, internal
  failures, exception resolutions or uses, roundtrip mismatches, ownership
  failures, traversal failures, leaf-traversal failures, gaps, overlaps,
  synthetic claims, or provenance-plan mismatches. Construction traversal was
  exact at 884,907 expected/visited nodes and leaf traversal exact at 308,886
  expected/visited leaves. `roundtrip --require-clean` independently reported
  19,947 accepted, 19,947 clean, 0 mismatched, and 12,694 not accepted.
- **Lexical ownership and declaration provenance.** Focused witnesses for
  fused, separated, and capitalized keyword-line forms render byte-exactly and
  visit both the `Landwalk` keyword-ability declaration and the applicable
  Type or land-Subtype declaration. Scanner claims divide the bytes between
  those declarations; the boundary operator contributes no lexical claim.
- **No word-naming.** The construction has no `require`, `checked by`, literal,
  or Rust predicate naming a surface, type, construction, or card. Coverage
  reported `licensing_checker_forbidden = 0`; the environment loaded, including
  its literal/vocabulary collision checks.

### DISCLOSE

- **Selection census.** Selected 19,845 -> 19,947; unique 15,678 -> 15,774;
  specificity-resolved 4,167 -> 4,173; unresolved ties and internal failures
  remained 0. The 102 gains comprise 96 unique and 6 specificity-resolved
  units. The six pre-existing rivalries outside the landwalk constituent were:
  `AbilityTriggered` over a preposed `PredicateAdjunctClauseTail` (Benthic
  Djinn, Sheoldred); declared `ObjectForObjectLexicalVerbPhrase` over a
  predicate-adjunct analysis (Cateran Slaver, Yavimaya Dryad); plural nominal
  coordination over modified coordination (Stonybrook Banneret); and bare
  relational reference over mass nominal (Witch Engine). Yavimaya Dryad also
  selected `ThenPredicateSequence` over `BareThenPredicateSequence`.
- **Current attestation is provenance, not admissibility.** The current corpus
  has 185 basic fused units (96 selected, 89 not selected), 2 Desertwalk
  (1/1), 2 Nonbasic landwalk (2/0), and 2 Legendary landwalk (1/1). It also has
  one selected Snow landwalk and one selected artifact landwalk. The grammar
  admits a full declared Nominal as the Keyword Quality; [CR#702.14a] licenses
  land types, the card type land, card types and supertypes, so the admitted
  set is wider than the licensed one (*Bearwalk*, *Cardwalk* parse). Nothing
  narrows it, because no declared feature distinguishes those declaration kinds
  today and every corpus selection below is a licensed shape.
- **Joke printings.** Direct probes show Townwalk (declared land Subtype) and
  Planeswalkerwalk (declared Type) fall out for free through the same analysis;
  Denimwalk remains a parse failure because it has no declaration. No row,
  construction, or exception was added for any joke printing.
- **Permitted licensing checkers:** 23, unchanged; forbidden 0.
- **Deviations and additions.** Existing fixed-affix atoms could fuse bytes only
  by owning the suffix as a construction literal, which would lose the
  keyword declaration's provenance. The compiler therefore gains one sealed
  `BoundSuffix` surface feature on fixed keyword declarations and one general
  `right_adjacent(value)` boundary atom. `Landwalk.ron` declares its bound
  suffix once; the single new keyword-line construction reads that feature and
  the existing declaration-backed Nominal inventory. Exhaustive compiler,
  scanner, renderer, diagnostic, and AST projections were extended for that
  sealed feature. No per-surface row or guard was added.
- **STOPs:** none. Every gain below has the required Landwalk-with-quality
  selection, no negative oracle began parsing, and no tie occurred. The ticket's
  macro-template sentence was read as governing v1, as directed by the
  coordinator; it did not constrain v2.
- **Glossary gap, closed at review.** `docs/contexts/oracle-english/CONTEXT.md`
  had no term for a keyword surface whose declared quality and keyword stem are
  realized as one orthographic word. The review added **Bound Keyword Surface**
  citing `[CR#702.14a]` and extended **Keyword Quality** to name the third
  realization; `BoundSuffix` and `right_adjacent` remain compiler mechanics.

#### Newly covered identities and selected analyses

Every identity below selected a path containing
`BoundQualityKeywordLineItemBoundQualityKeywordLineItem` (the path printer
spells every node as its Category followed by its construction name) and the
`Landwalk` declaration's `BoundSuffix`. The heading gives the selected landwalk surface;
basic fused and Desertwalk forms use
`NominalBareSingularNominal / HeadNounSingularHead`, Nonbasic uses
`NominalModifiedSingularNominal / NominalModifierNonSupertypeModifier /
HeadNounSingularHead`, Legendary and Snow use the corresponding
`NominalModifierSupertypeModifier`, and artifact landwalk uses
`NominalModifierNounModifier`. Thus this is an identity-by-identity selected
analysis ledger, not a surface-only census.

**Desertwalk — 1**

- `aa7949bf75d160b52e7a2d9fb66f85c57c675ed8527db112a59bf7d4275f995d` — Hazezon, Shaper of Sand

**Forestwalk — 21**

- `8b9b5925e5004499f6dec7bdbc5c438280bdc529cfa6c4741d4cd5bb90073e00` — Boggart Loggers
- `f94256c31e36fcf7a69d7c77775a3ad0b6929f485171f41af4e555bd73c33e69` — Cat Warriors
- `ddef287d668490ef3877e0107d56c2c0c8d846ab57211178341374852abdab4a` — Elite Cat Warrior
- `11217b08454842f55d001fe65bbda8ad6606c4f3591034fd482dd00188f3467b` — Emerald Oryx
- `4b8462d29c7fe0f88355d41743be9ba01437e2b9c495afa6a47b11f44cb14941` — Heartwood Treefolk
- `af7a51789ab90068fb14bd9caabbbcb8297b2f45c0b9b859263bb646bcef3dcd` — Jukai Messenger
- `97eee41286979d5793bc9211180951aede2ce1a0b98b458cf40fc1e04be05c83` — Koth's Courier
- `2657b07184edbbff3203269c18aa13b138749549971199d1bde9a533cc1f1ca1` — Leaf Dancer
- `6dc71085fe43af409f2fbe61d4a28d9b0a38fc77392975714a157efe53f10886` — Lynx
- `fafbec44e716afbe40ab4986ed3c7cf52feb09b916a8fdc7f5131f9bf78416a4` — Rushwood Dryad
- `f6dcd6a4e28ace0b5ebb212fdff211c6ee64a1f85f757e89a744d8c074baa3b5` — Shanodin Dryads
- `9e5ddaff16d2b8883ab4415fa772067936e28a718a953836c3c5133112c48943` — Slinking Serpent
- `d18e9506b995f080c51bf4aaff5478f8a4063af404bb177399b883ede40da941` — Somberwald Dryad
- `73c1a18e78bbdb795b48d8b1bf7fc0b83c365ce20aec4c715649ca5581deac55` — Vine Dryad
- `fb157b5535d0768876ac9dec6f5e0ced2b93bc866ac262963c96233e870f5714` — Willow Dryad
- `e7704ae473f54f2d28027dce49e135cdd6dca25ca689cb8afe9b3a21445be824` — Woodlot Crawler
- `81cd379602089aebd7c528d4f7919e91f13987e64060947c3b1a4afc6ae9e6e6` — Yavimaya Dryad
- `b03ccc80a7dde2af2274ed064904b368dc1aa0ee65333481811a1ac926e2d02d` — Zendikar Farguide
- `627338eb398efae04e4c0017b81488bc08a374c7ba2ef7cc11e474a131db8973` — Zodiac Monkey
- `6f057a235a88aa5df39f5fb24a8f8137e148b288ea74357143308b001f47eb5d` — Zodiac Rabbit
- `f8fef26fc4b6d8179a093c5ae39b7cf2eb17bd43266d3c5aa3768463c084cd56` — Zodiac Tiger

**Islandwalk — 19**

- `90d22701107a5879a5683d122dcbc27e0475db3f623a85c370c1e7bd18589607` — Benthic Behemoth
- `583ac6f54dd3f4230615f6f5bf604bde738d50bd66716d4604a190114ed891e4` — Benthic Djinn
- `0bfde3d89a4079428bfc327bbc11c7668fe11bc7757fccfc608c9738c8aacb24` — Bull Hippo
- `3411a90bd701261e29fa8d5caa2c0e5803d990e96a13c906430a1d97d8441f32` — Cold-Eyed Selkie
- `412cd75ec057d4df73e90acb9b2294309812edeccb36eaab0d9ecc7f21ebd002` — Devouring Deep
- `c77075fd929652f97d091acbdfa124e376c0dce4a1ae3447e78319ff6df5ba2d` — Grayscaled Gharial
- `37db21facbd622609eec4907b125e469f77424525a3feb94139e7d98a134683d` — Halimar Wavewatch
- `6d5ea1c331598ca1bf9ebeabde9405469dc32d307b9fb14ca9b4f0f4706c66ef` — Harbor Serpent
- `29cff680b63be598d97630b7b3ddfca91f4ae9ce8e6808a5baa7127f2f6a3b3a` — Inkwell Leviathan
- `b7c2cf05a921ed272521a5fb19c1b99ff51c5a7736c6e6c796047fd3b185e8c7` — Lurking Crocodile
- `96690d5d79abfe6be5e70265a3a6d5ea7dad3df1486be955f70a3b3b388556ae` — Merfolk Raiders
- `566f8339a74a461d8dd373f3eecb410402004b05d7892d7410cd7ae7de0982b9` — Pale Bears
- `59c69f08b6f26965758904020c8c9e34287655b306315a2cb48aa2a5333fa6e9` — Rishadan Dockhand
- `ff4daf7dc2784053f7b2e4626f8a3679d5b1d4bf3a1506196832451531396f3e` — River Bear
- `c81c37ebdfb3f47090d662c27c1e44d67baf7f8e0c8e4a84b9ed30719b840454` — River Boa
- `322ea77c23325cf171ba9abc89c47c608d50519d5270165f173d1d7219d0e743` — Rootwater Commando
- `23e36df0bb11eb11c6c7df016fdd0e8d29302829ab164c86200bdbb98704931f` — Segovian Leviathan
- `6b63fe2834509f4fdffa78e0b6d08cd9f596c73b54bed4ce31a9bff7b4e62755` — Stonybrook Banneret
- `041aaa973f03f0ede664c3e8f613c70f64abff2d8cee8cd0cab34bf9c3fea671` — Zodiac Horse

**Legendary landwalk — 1**

- `a8bf9d66e5a1bee9a5f52476a174af86a482a961b99f00db509608ed93202432` — Ayumi, the Last Visitor

**Mountainwalk — 17**

- `f419498a6f7cc02caab81adc5cc32e8974cf095fa692ed170500782e6ee11a12` — Canyon Wildcat
- `e745acd77463dd2bf5cfdfc1efe1f2866ff37223f9a661a84d94fd411026ead2` — Cavern Crawler
- `5283483d2efa7040758ac00e2c02f32a31b83d55c7165f43d8fb526719bf2385` — Cliff Threader
- `058cb9d907ed2e9ff0efa1aeffb346ba6169dc1665f150570193d37486deca0e` — Colos Yearling
- `ae7ab07dec8ccf350a975cf9f4d2457ff01b45261d15ab41425a260826a831fd` — Dwarven Grunt
- `56807d69de96c412affd3b96ad0942b1c9667d15a9f281bd9b4aef5db39f1f0f` — Glissa's Courier
- `a844915a9feca96bdca0f38ddd9f17d246b92196785886522c213521caa9cfde` — Goblin Mountaineer
- `bed621499880dd6a98a3d1b7ab4f8ec2ddb56d75c00280d92a9ca4fa721a1bc9` — Goblin Spelunkers
- `6f773fefbc9783395b4cb17d0ab1eef285482aa92456405727dcf184a0682ec4` — Goblins of the Flarg
- `84fc37fe62de412ba73b87c1b758470362826d33f6698ce9933637f9f1570248` — Hillcomber Giant
- `a99c29a42097cc0ff53dc43284b297b4c224f49af0a0b86f4bb92b56998000d4` — Mountain Goat
- `cd59ed675d70b62d083c2a0d89b969af8bc6e510fdbc79a05bd8a82dc352ba04` — Mountain Yeti
- `a9dc208cfb98955369ffd0c63dbfeaa0cf04583d423eb7d1d1df05170971892d` — Rock Badger
- `ce351b9f674f357f2d81b7a04926d5e6d775eae79de0e70a9fae1ae58f69f38f` — Sokenzan Bruiser
- `f819f13371a4f5eeda14d95b4fd91d0a8309c6c20498db5ff4bc243f7d6e24cc` — Vug Lizard
- `eb6e9fba18f6cff6d809251e6fd4ec8759c098cb49572fe52e53b788a4efb7d5` — Zodiac Dog
- `3e512c2933f9ecea0bd934b2b90bd715c5433a26e2c4737777cf05a4abfb2b17` — Zodiac Goat

**Nonbasic landwalk — 2**

- `f6481ed6b2a36004e87bc614e652963523836278c30c5977090104fa73124810` — Dryad Sophisticate
- `7396592af332987683d0fa435c325ebb024a7561e505e2411a9b90d996789493` — Trailblazer's Boots

**Plainswalk — 3**

- `f53cf3908ff86761948bbd19f5fc0c560a1416503d74bbab6eff76adf6f1d26c` — Boggart Arsonists
- `e14ce5ff95cc8003cb0530df793e0e3ea3c79eebe57a007ee8d89092957eb7c1` — Righteous Avengers
- `b1bcf1074ad78824ffa9c66807c6dc180771fe7e0ff14ced56849b4796fa831a` — Zodiac Rooster

**Snow landwalk — 1**

- `4f234547fdedac94aa51c0d2cf369ef35319fde4f9a28058f704ebdc46c7beb8` — Zombie Musher

**Swampwalk — 36**

- `8c06ecff18ebe25a154a9d55d5431a5bcfa5e14c18f7adc537ee09d611fc917d` — Anaconda
- `71cd39795fcb45b80c005acc48aee07afda7316ab3a3fd220bf5655a66b0455c` — Anurid Murkdiver
- `909665fa2ead303bd463f6a71136bcd5811939fd44404ade75e213e7caec1b03` — Blistergrub
- `76dda8bf919592b17ef034cd68343d3cde68e656ef6cf9c028a1eaf084f7f21d` — Bog Raiders
- `2b2305b921ac80f336012ec56e721dd4949ed28d06abffefab7629f50e40f3d9` — Bog Smugglers
- `dda56947dc8a7d09ed5852b004820a7d6f01b2f8ba8277a85062e503ab50a710` — Bog Tatters
- `d15d8c24bee694f9de777c776f988cc7261e2b668a414d8f45e152aa9fcf17a8` — Bog Wraith
- `5e990b3c6e1cb85a9f218f3f238ce9f4e4559f48509de7ee9ed1ac54598d9423` — Bog-Strider Ash
- `a7af914c41da9f1c6db44b494b48ba9bd77110a05982ada0673ef14c3cfbffe6` — Cateran Slaver
- `c0102616123eb466226419c366a36ffed1297da2972d10c4060dc4b025dead6c` — Dirtwater Wraith
- `049af2159b1d650b5a791c6375ab9f94b4c80045a12a8bac2dc5286218bbcca5` — Farbog Explorer
- `93cbfaff64bfe0f3af249201609e834a2550cc69e929cec84a4745a1dae57d7e` — Krosan Constrictor
- `9ca6820709ea0ce0ae5472d03bfc3e37d8649a312e98338bf4fccd70944af29d` — Lost Soul
- `e0156fd9dae2fa072c2cafc5e17974cd4f38d6790cf90219cf5c624a2dc70cd7` — Marsh Boa
- `4260ed7401bdb2fcf3cfe5d10f05c7f2a44a328e4ba7bf3d6a7bb2785b4750da` — Marsh Goblins
- `55d970567fba6934a66fbc8a8a9b64bfc298a8dd6f9412ae71c0e6db2d7c42ee` — Marsh Threader
- `333dc546ea29f942f8a290c77de2a24105bbfb3764cbb4d6c0141c00a37a3fc3` — Mire Boa
- `3d4aab25fba2bff2bfc89f03c19a93b6991a5fdea6d9ae3621899bb04362a8f2` — Moor Fiend
- `f5125b2843f77a315859ebe51605f68996955e0a703a27276b3d79b366d2e975` — Odylic Wraith
- `6ec0297538fbc9a7be5ae03f4903ba35b5e3a6c3a15d0db166cc117975ec047d` — Plague Beetle
- `03eeed51fd64162500ebd040eacee107d4a1d295d60cc082b1d9d3e2d00853cf` — Pygmy Allosaurus
- `6f0390133f69bfbe2eb4d1eb4eb659e3eda8d29ae1b92fe3697c780d0b848b2a` — Raiding Nightstalker
- `465432b7c64e6c82c629fab8b49a90fd4853573cd45d97aec9b7415c642f4622` — Sewer Shambler
- `60d4d93affb54f5d0b43923a9d0cc32719a8b36e2a15dbcb22986371ad17ce2d` — Sewerdreg
- `9234c9e5e845d9a1f6bccb1e96d930c01b3e1bacb56f63e262d07f54c37fdcb7` — Sheoldred, Whispering One
- `8133afef5e1ab0622ed7b81e56b6a07da8faec6ded7e66ea3b1d6e6c94e095bd` — Slithery Stalker
- `46d3e2c607eeedb2a39fe335ca9aa8499fa796e03e3fab01d7cec4e353c9a409` — Sol'kanar the Swamp King
- `8e57568696726031cb4f113fbca8b36c233b7cc7e11e490d5e7c167b3b501831` — Street Wraith
- `f5c5d19a9e61eaaf0936f0c68e83cb87e40b7c3e2610535295211597307a4321` — Warthog
- `87bf91530bbcd54f1e699d61086ca2837eb920446e29eebbc4a83269ec403066` — Whispering Shade
- `85e2ad4f170dde0315bdded04cf369ab18622c6f7dbbf1bcc2f8c3ad9fd9f411` — Wild Ox
- `a9ff21637b6d95cc94cd78cc62f066510865a0b372054ce1bffda1586a21ab0c` — Witch Engine
- `c0f295f61b3987b1a5742e5b9c4da4d076b39076010ca1f4be25c7f986228879` — Zodiac Ox
- `9fa96d368cd9f3c3b61cceac545143d4f6b288221f40b670cccecc484eaf59df` — Zodiac Pig
- `14985c9dcc5f4d35b1743cb2608583d8c794679ff52e613fb18dfd3b072e897a` — Zodiac Rat
- `19aa8d84247ddaf64e4eefdf524cee3d0d8a5708a967a13e4860e2e01e26b955` — Zodiac Snake

**Artifact landwalk — 1**

- `e9f9c26ba267116a15a5a206264d2b417cf97852e5a58cbad470151de1295bbc` — Vectis Gloves

### REPORT

- Coverage lock: 19,845 -> **19,947**, +102 / -0, on change
  `rrrzuukt`. Construction declarations: 395 -> **396**; the one addition is
  the ticket's keyword-line construction.
- Licensed vocabulary/lexeme homographs, reported not fitted: 2 —
  `AttributiveAdjective::Untap` beside the `Untap` keyword-action Verb
  declaration; `TargetingMarker::Target` beside `CommonNoun::Target`.
- Form-literal/vocabulary overlaps, reported not fitted: 9 — `additional` at
  `additional_cost` atom 2; `to` at `up_to_quantifying_determiner` atom 1;
  `the` and `next` at `definite_next_mass_quantity_reference` atoms 0 and 1;
  `to` at `scalar_less_than_or_equal_to` atom 4; `the` at
  `number_of_scalar_value` atom 0; `the` at `greatest_scalar_value` atom 0;
  `other` at `other_than_qualified_reference` atom 1; `the` at
  `positional_partitive` atom 0.
- Gate scope: `cargo xtask gate --changed --run --clippy` printed and ran
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p
  deckmaste_english_v2 -p xtask`, then strict clippy over the same four crates.
  The first test attempt exposed one stale expected diagnostic; the next clippy
  attempt exposed five local style findings. Each was fixed, and the final
  closure test and clippy run were clean. `cargo fmt --all` and `cargo xtask
  catalogs check` were clean.
- Assurance counts: restored 0; re-spelled 4 existing assertions (the bound
  atom parser and validator shape tests, the built-in keyword supplemental-row
  count, and the declaration-term accepted-feature diagnostic); ignored 0;
  added 2 tests (Landwalk declaration surface provenance and end-to-end fused /
  separated keyword lines); removed 0. The one ignored test in the closure is
  pre-existing and untouched.
- Performance advisory, workers 8. Coverage `--check`: 111,509 ms,
  **130,306 ns/B**, host load 5.73 / 7.18 / 6.98, 0 concurrent `cargo`/`rustc`
  processes at launch. Coverage `--bless`: 106,765 ms, 127,245 ns/B, host load
  4.76 / 6.68 / 6.84. Ambiguity: 113,843 ms, 135,024 ns/B, host load 7.90 /
  7.64 / 7.20. Roundtrip: 107,946 ms, 121,690 ns/B, host load 3.86 / 5.62 /
  6.46. Every pass exceeded the 16,260 ms quiet-host ceiling under the reported
  load; this is provenance, not a fitted gate. **True contention** (the
  implementer's sandboxed process count is meaningless): 2 concurrent codex
  executors and 0 other Opus reviewers were running against this host.

### Review corrections

Applied by the landing reviewer on this workspace; gates below were re-run
after them.

- **MEDIUM — the record misattributed the unselected fused units.** "89
  stopped elsewhere" implied the landwalk constituent was never the blocker.
  It usually is: printed Oracle text writes the fused word in lowercase
  wherever it is not line-initial, and the land Subtype declaration has no
  running-position lowercase reading, so the parse stops inside the fused
  word. Measured on this tree: 199 faces carry a basic fused walk surface, 123
  carry a capitalized one, 76 carry only lowercase ones; sampled failures stop
  at the fused word (Burrowing bytes 40..52, Crevasse 15..27, Deadfall 15..25,
  Coral Barrier 80..90), not elsewhere. Fixed by restating the paragraph and
  routing the remainder to the new planned ticket
  `english-v2-running-case-fused-keyword-quality`. No identity regressed: every
  such unit was already a parse failure at the fork point, so this landing is
  still +102 / -0.
- **MEDIUM — construction misnamed.** `QualityPrefixKeywordLineItem` called
  the Keyword Quality a prefix, but the quality is free material and the
  keyword stem is the bound element; `prefix(...)` already means a fixed affix
  in this compiler, and the row's own lexeme codec is
  `BoundQualityKeywordAbility`. Renamed to `BoundQualityKeywordLineItem`
  (element `BoundQualityKeywordLineItemValue`, sum member `BoundQualified`,
  construction `bound_quality_keyword_line_item`) across `constructions.rs`,
  `ast.rs`, the xtask diagnostic projection and the keyword-line test. The
  doubled name in a selected path is the path printer's Category-plus-
  construction spelling, shared with every other node
  (`KeywordLineKeywordLine`, `AsClauseTailAsClauseTail`) — not a nested pair.
- **MEDIUM — a general law carried a per-declaration carve-out.**
  `builtin_v2_keyword_abilities.rs` had re-spelled its universal
  surface-count assertion to `usize::from(name == "Landwalk") + 1`. Re-spelled
  name-free: exactly one `Fixed` surface, first in the row, with every further
  surface a `BoundSuffix`. Landwalk's own count stays pinned by
  `landwalk_declares_one_bound_suffix_surface`.
- **MEDIUM — the new boundary atom shipped without rejection tests, and
  without a well-formedness rule.** A form-final `right_adjacent(...)` would
  leak its `suppress_next_space` into whatever the parent form renders next,
  so `validate_bound_form_atom` now rejects a boundary-only atom in final
  position ("a boundary-only atom must be followed by the material it binds"),
  and the two existing bound-atom rejection tables gained `right_adjacent`
  rows for a literal value and for optional/sequence role values.
- **LOW — indentation.** Two mechanically inserted `BoundSuffix` match arms
  (`emit/render.rs`, `emit/runtime.rs`) sat at the wrong indent inside blocks
  `cargo fmt` declines to reformat; realigned by hand.
- **Glossary.** Added **Bound Keyword Surface** to
  `docs/contexts/oracle-english/CONTEXT.md` citing `[CR#702.14a]` and extended
  **Keyword Quality** with the third realization, closing the disclosed gap.
- **Review assurance counts:** restored 0; re-spelled 2 (the universal
  keyword-ability surface assertion; the keyword-line test renamed with its
  construction); ignored 0; added 1 test
  (`boundary_only_atoms_reject_a_form_final_position`) plus 4 rejection rows
  inside 2 existing tests; removed 0.
- **Review deviations:** one validation rule and one glossary entry beyond the
  ticket's letter, both justified above. Nothing under `data/gen` was touched
  and `cargo xtask catalogs check` stayed clean.

Verified in review, not re-measured: the ledger's 102 identities are exactly
the 102 hashes the lock gained; 16 identities across every surface bucket
(Desert, Forest, Island, Legendary, Mountain, Nonbasic, Plains, Snow, Swamp,
artifact, plus the two grant sites and the three specificity-resolved units)
were re-read with `ambiguity --json` and each selects the Landwalk declaration
with a Type, Supertype or land Subtype quality; `walk` is claimed by
`lexeme:keyword_ability/Landwalk/bound_suffix` and appears in no vocabulary.

**Review gates**, all foreground, stamped on feature change `rrrzuukt` with
coverage lock `covered = 19,947`. `cargo xtask gate --changed --run --clippy`
printed and ran `cargo test -p deckmaste_construction_core -p
deckmaste_construction -p deckmaste_english_v2 -p xtask` — 1,419 passed, 0
failed, 1 ignored (pre-existing) — then strict clippy over the same four
crates with no findings. `coverage --check` (`DECKMASTE_COVERAGE_LOCK=report`):
32,641 total units, 19,947 selected and covered, 0 selected-uncovered, ties,
internal failures, roundtrip mismatches, ownership/traversal/leaf failures,
gaps, overlaps, synthetic claims or provenance mismatches; 884,907
expected/visited nodes; 308,886 expected/visited leaves; permitted licensing
checkers 23, forbidden 0; no lock delta section. `ambiguity
--require-resolved`: selected 19,947, unique 15,774, specificity-resolved
4,173, unresolved ties 0, internal failures 0. `roundtrip --require-clean`:
19,947 accepted, 19,947 clean, 0 mismatched, 12,694 not accepted. `cite check
--list-noncompliant` reported 0; `cite check` 14,498 citations, 0 stale; `jj
diff --git | cite audit --diff` audited 6 sites, each read against its rule.
`cargo fmt --all` and `cargo xtask catalogs check` clean.

Review performance advisory, workers 8, host load reported per pass:
`coverage --check` 108,064 ms, **130,699 ns/B**, load 6.13 / 8.04 / 5.84;
`ambiguity` 116,754 ms, **139,757 ns/B**, load 9.28 / 9.59 / 6.78;
`roundtrip` 106,418 ms, **125,078 ns/B**, load 10.01 / 9.68 / 7.13. Each
exceeds the 16,260 ms quiet-host ceiling under 2 concurrent codex executors
and 0 other Opus reviewers; provenance, not a gate. The `kata refresh` that
followed these gates brought in ticket-file moves only (two claim commits and
one `wip`/`planned` rename), which reach no crate, so nothing was re-run.
