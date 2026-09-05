---
needs: []
---
**One round's lexical gaps, batched.** Every row below is a *data* line: a
member added to an inventory the grammar already reads, reaching consumer
constructions that are already standalone `lex(...)`/`verb(head)` forms, adding
zero constructions, sums, seams or features. Minted as a batch by coordinator
ruling (2026-09-05) after `english-v2-tail-color-property-adjectives` spent a
whole claim/executor/review/integrate cycle on a single vocabulary member.

Measured on change `ptoxwkmmrqno` (19,469 / 32,641 covered, 13,172 parse
failures, first-failure byte attribution; re-measure at claim). **545 units**
across six rows.

Every row was verified by probe against a *control* — a sentence identical in
shape whose lexeme already exists. The control selecting is what proves the
consumer is reachable and the gap is the member alone.

| surface | units | inventory home | consumers (all standalone forms) | licence | witness / control |
|---|---:|---|---|---|---|
| `kicked` | 194 | keyword-action/verb lexeme with a participle; no `Kick` identity exists anywhere (`core_verbs.ron` has none) | `predicative_declared_participle` (`:1777-1781`, `form = verb(head)`) | *kicked* is the participle of the kicker keyword action | `Activate only if this creature was kicked this turn.` fails; control `…was exiled this turn.` **selects** |
| `swampwalk`, `islandwalk`, `forestwalk`, `mountainwalk`, `plainswalk` | 127 | `plugins/builtin_v2/macros/stubs/keyword_abilities/` — `Landwalk.ron` exists, the printed fused surfaces do not | `bare_keyword_line_item` (`:4878-4882`, `form = lex(keyword)`) | each is a printed keyword ability in its own right; `macro-keyword-templates` states typed landwalk "stays bare keyword names", i.e. not templated | corpus: Anaconda `Swampwalk`, Boggart Loggers `Forestwalk` |
| `share` / `shares` | 91 | verb identity, `core_verbs.ron` | `finite_subject_gap_relative_clause` (`:4420-4423`, `form = licensed("that") verb(head)`) and the transitive subject-gap relative | ordinary transitive verb | `Destroy target creature that shares a color with it.` fails; control `…that controls a land.` **selects** |
| `win` | 77 | verb identity, `core_verbs.ron` | finite clause predicate | ordinary verb | `If you win, draw a card.` fails; control `If you control a Goblin, draw a card.` **selects** |
| `unlock` | 30 | verb identity, `core_verbs.ron` | same as `win` | ordinary verb | corpus: Bottomless Pool // Locker Room `When you unlock this door, …` |
| `exert` | 26 | verb identity, `core_verbs.ron`; the `Exert` keyword-**action** stub covers only the declaration line, not the predicative verb | finite/bare predicate | ordinary verb | corpus: Ahn-Crop Champion `You may exert this creature as it attacks.` |

Affected subset. Surfaces: `\b(kicked|swampwalk|islandwalk|forestwalk|mountainwalk|plainswalk|shares?|win|unlock|exert)\b`.
Touched constructions: `predicative_declared_participle`, `reduced_relative_modifier`,
`bare_keyword_line_item`, `finite_subject_gap_relative_clause` and the transitive
subject-gap relative, plus every card whose parent-tip selected path contains one
of them. Witnesses: the six rows' cards. Negatives: any card using `exiled`,
`controls`, `landwalk`, or a bare keyword line must not move.

Routed OUT of this batch, each with the reason and the owner — do not add them here:

- `cycling` fused surfaces (`landcycling`, `plainscycling`, …), 91 units →
  `macro-keyword-templates`, which design-gates typed cycling explicitly
  ("a bounded slot-reader codec … a codec change no existing macro uses").
- `exploits` and the other ability-derived verbs, 23 units →
  `english-ability-derived-verb-batch`, which names `exploit` and records a
  prior over-fire regression.
- `dealt by` / `controlled by`, 154 units → `english-v2-remaining-prepositions`.
  Verified: `Prevent all combat damage that would be dealt by target creature
  this turn.` fails **at `by`**, not at `dealt`, and `Deal` already declares its
  participle. Adding lexemes here would gain nothing.
- `addition` (`in addition to its other types`), 170 units →
  `english-v2-locative-licence-set`. The noun is missing *and*
  `nominal_preposition_is_licensed` refuses every `to`-postmodifier; the member
  alone cannot land it.
- `died`, `attacked`, `entered`, `left`, `gained`, … → the preterite has no
  finite realization at all, which is structural:
  `english-v2-tail-preterite-finite-clause`. `Attack` already has its frames and
  `…if a creature attacked this turn.` still fails, so these are not lexeme gaps.
- `named` — dropped. Zero units under this round's bucketing key and the
  consumer was not verified; re-derive before proposing it again.

`english-v2-rename-color-vocabulary` is **not** folded in: its own body leaves
the replacement name and any glossary amendment to be chosen at implementation,
which is a design dimension, not a data line.

Ruled against: a construction, sum, feature or seam added for any row — a row
needing one is misclassified and must be split out and reported; a `require` or
`checked by` naming any of these surfaces; narrowing an existing form to keep a
number.

Acceptance: the standard landing record; each row's witness selects and its
control still selects with an unchanged analysis; **the construction count is
unchanged** (that is the batch's defining property — a change to it means a row
was structural); both byte-exact laws green with total ownership, zero ties.
Report gains per row, since a row that gains nothing was misdiagnosed.

Baseline: change `ptoxwkmmrqno`, 19,469 covered of 32,641, 13,172 parse
failures, 0 ties, 0 internal failures. Re-measure at claim.

Tier: **terra**. Standard constraints apply.

## Landing record

### PROVE

- The implementation was refreshed with `kata refresh` before the final
  measurements. All figures below are stamped to feature change `qsssoovy`
  with lock `covered = 19,939`.
- The affected subset was built from the ticket's exact surface expression and
  contained 801 card names and 743 corpus units. Its final passes reported 334
  selected and covered units, 409 parse failures, 0 unresolved ties, 0 internal
  failures, and 334 / 334 byte-exact roundtrips.
- `cargo fmt --all` completed. `cargo xtask gate --changed --run` printed
  and ran the reverse-dependency closure:
  `cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask`.
  The same closure's strict command,
  `cargo clippy -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings`,
  completed cleanly.
- `DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check --workers 8`
  reported 32,641 total units, 19,939 selected and covered, 12,702 parse
  failures, 0 selected-uncovered units, 0 unresolved ties, 0 internal
  failures, 0 exception resolutions or uses, 0 roundtrip mismatches, 0
  ownership failures, 0 traversal failures, and 0 leaf-traversal failures.
  The printed lock delta was `newly covered 375 corpus identities`; no
  no-longer-covered section was emitted, hence +375 / -0. The report-mode
  `--bless` then wrote the lock to exactly 19,939.
- Refreshed `ambiguity --require-resolved --json --workers 8` reported 19,939
  selected, comprising 15,611 unique and 4,328 specificity-resolved
  selections, with 0 unresolved ties and 0 internal failures. The exact
  per-unit diff against the refreshed parent tip contained 375 changes, all
  `parse_failure -> selected`; no previously selected identity changed
  analysis and no identity was lost.
- Refreshed `roundtrip --require-clean --workers 8` reported 19,939 accepted,
  19,939 clean, 0 mismatches, and 12,702 not accepted.
- Structural-law totals were exact: 884,581 expected nonterminal nodes =
  884,581 visited constructions; 308,432 expected leaves = 308,432 visited
  leaves. The construction declaration count remained 394 -> 394, the
  ticket's defining acceptance.

#### Witness-and-control results

| row | result after member addition |
|---|---|
| `kicked` | The corpus-authoritative conditional modal selected and rendered byte-exact. The ticket witness `Activate only if this creature was kicked this turn.` still stopped at bytes 47..51 (`this`), while its `exiled` control retained its selected analysis. The lexical member is not forced through the remaining duration-host shape; recovery is routed to live `english-v2-bare-duration-adjunct-licence`. |
| fused landwalk identities | Bare `Swampwalk` and the ticket's other fused keyword witnesses selected as bare keyword lines; the ordinary bare-keyword controls retained their analyses. |
| `share` / `shares` | `Destroy target creature that shares a color with it.` selected; the `controls a land` control retained its selected analysis. |
| `win` | `If you win, draw a card.` selected; the `If you control a Goblin` control retained its selected analysis. |
| `unlock` | `When you unlock this door, draw a card.` advanced through `unlock` and stopped at bytes 21..25 (`door`). The row gained no corpus identities and was not forced. `workbench-room-halves` owns the half-level referent, door noun, and unlock-trigger shape; English-v2 parity remains routed to that recorded structural owner. |
| `exert` | The verb member gained 6 corpus identities. `You may exert this creature as it attacks.` advanced through `exert this creature` and stopped at bytes 31..33 (`as`); the remaining postposed subordinate clause is routed to `english-v2-subordinate-clause`. |

### DISCLOSE

#### STOP and coordinator-authorized resolution

- The earlier STOP was valid but its six-row rollback was overbroad. A
  row-by-row bisection of
  `plain_modal_rejects_every_deferred_or_malformed_surface_as_an_ordinary_failure`
  proved that only the `kicked` row made the generic negative select:
  all six rows were red, the other five without `kicked` were green, and
  `kicked` alone was red. The exact sentence was:
  `Choose one. If this spell was kicked, choose any number instead.\n• You gain 1 life.\n• You gain 2 life.`
- The sentence is grammatical Oracle English and is not a wrong reading.
  AtomicCards prints that same conditional-choice header on Inscription of
  Abundance, Inscription of Insight, and Inscription of Ruin. The Oracle style
  guide's “Modal choices” section says: “Bullet groups also use
  complete-sentence headers when the choice count is conditional.” Under the
  source-of-truth principle, the reading stays.
- Per the coordinator's 2026-09-05 ruling, that one assertion was re-spelled as
  a positive, byte-exact witness and the `kicked` row was retained. This is
  the authorized negative-to-positive flip. No narrowing, exception, named
  guard, new construction, or structural accommodation was added.
- There is no remaining STOP: every gain below was read; no newly covered
  identity is a negative oracle or has a wrong selected analysis, and there
  are no ties or losses.

#### Selection census and row attribution

- Parent -> feature: unique 15,353 -> 15,611;
  specificity-resolved 4,211 -> 4,328. The specificity share rose because 117
  of the gains are specificity-resolved. The contributing construction pairs
  are `finite_condition` over `preposed_if_clause_tail` for the 63
  `kicked` / `win` gains, the
  `subject_relative_qualified_reference` /
  `prepositional_qualified_reference` nesting rivalry for the 44
  `share` gains, and `triggered_ability` over `preposed_clause_tail` for
  the 10 fused-landwalk gains.
- Gains by ticket row: `kicked` 139; fused landwalk identities 146;
  `share` / `shares` 65; `win` 19; `unlock` 0; `exert` 6. Total:
  375.
- Coverage emitted 21 permitted licensing checkers and 0 forbidden licensing
  checkers. The environment load checks are green, and the diff adds no
  word-, lexeme-, construction-, card-, verb-, noun-, or preposition-naming
  guard.

#### Deviations and additions

- One existing modal negative assertion was re-spelled as the positive witness
  above, under the coordinator-authorized grammaticality ruling.
- Two aggregate-count assertions were re-spelled from 195 to 200 built-in
  keyword-ability stubs, and the generated-fact prose was re-spelled to the
  same catalog count. These are consistency pins surfaced by the changed
  closure, not fitted ceilings.
- One existing quoted-interior negative kept rejecting; its first-failure
  attribution moved from the newly licensed `shares` span to `feelings` and
  was re-spelled accordingly.
- The five fused keyword identities were added to xtask's existing facts
  inventory and the Idris facts table was regenerated. This preserves the
  existing cross-consumer inventory; it adds no rule or construction.
- No construction or test was added or deleted. No sum, seam, feature, frame
  family, or guard was added.

#### Newly covered identities and selected analyses

All 375 gains were individually read. This is also the complete set of
selection changes; there were no losses and no analysis changes among
previously selected identities.

<details>
<summary>375 gained corpus identities</summary>

```text
newly covered	009dc31de6f04aa2aa1ef154aca693b052e2e3ea573d6058bca27b0904d4826e	card "Kor Sanctifiers"	selected_analysis "Kicker {W}\nWhen this creature enters, if it was kicked, destroy target artifact or enchantment."
newly covered	00c83bcb910dabfcaaf321d6dc2491743800d95bb09385fb4b9bd17c76e83547	card "Benalish Sleeper"	selected_analysis "Kicker {B}\nWhen this creature enters, if it was kicked, each player sacrifices a creature of their choice."
newly covered	015e0bcb8b5e245522a7d0e10d4da07f70de2f11c53e6fb36fa739929c04ed2c	card "Scout the Wilderness"	selected_analysis "Kicker {1}{W}\nSearch your library for a basic land card, put it onto the battlefield tapped, then shuffle. If this spell was kicked, create two 1/1 white Soldier creature tokens."
newly covered	01cc0d23391ee909e0d86595874e8215db2ec68efc436e8d22dd3bf32d51fc55	card "Dwarven Landslide"	selected_analysis "Kicker—{2}{R}, Sacrifice a land.\nDestroy target land. If this spell was kicked, destroy another target land."
newly covered	02f418eebec75723d7fe5f610c2e329f1c06f65a02e6d1f09fb2fc1308aa9113	card "Orbweaver Kumo"	selected_analysis "Reach\nWhenever you cast a Spirit or Arcane spell, this creature gains forestwalk until end of turn."
newly covered	03963fbb6bd515b002c960a8fa9c0050e83261163ee395e90d3ed67ee0f682ef	card "Viashino Branchrider"	selected_analysis "Kicker {2}{G}\nHaste\nIf this creature was kicked, it enters with two +1/+1 counters on it.\n{2}{R}: This creature gets +2/+0 until end of turn."
newly covered	03a72c079cb7dce13bcc16091b7e7ab22e216209a9bb38c46f7a7b6f443be558	card "Littjara Kinseekers"	selected_analysis "Changeling\nWhen this creature enters, if you control three or more creatures that share a creature type, put a +1/+1 counter on this creature, then scry 1."
newly covered	03eeed51fd64162500ebd040eacee107d4a1d295d60cc082b1d9d3e2d00853cf	card "Pygmy Allosaurus"	selected_analysis "Swampwalk"
newly covered	041aaa973f03f0ede664c3e8f613c70f64abff2d8cee8cd0cab34bf9c3fea671	card "Zodiac Horse"	selected_analysis "Islandwalk"
newly covered	049af2159b1d650b5a791c6375ab9f94b4c80045a12a8bac2dc5286218bbcca5	card "Farbog Explorer"	selected_analysis "Swampwalk"
newly covered	053cc5704c0d1f0b802739b4c27059f51b5cb45fa34948d26e2b6aa3bcc2f8b4	card "Semblance Anvil"	selected_analysis "Imprint — When this artifact enters, you may exile a nonland card from your hand.\nSpells you cast that share a card type with the exiled card cost {2} less to cast."
newly covered	058cb9d907ed2e9ff0efa1aeffb346ba6169dc1665f150570193d37486deca0e	card "Colos Yearling"	selected_analysis "Mountainwalk\n{R}: This creature gets +1/+0 until end of turn."
newly covered	05a6a0eec74e7e502ceb5c110317c77e8d1e15ee15e18144bca30b0aff5a471e	card "Llanowar Elite"	selected_analysis "Kicker {8}\nTrample\nIf this creature was kicked, it enters with five +1/+1 counters on it."
newly covered	06580586044dba3f4a695be427ea074ffb2cbb6ccd7ba7aeac221a9d5b506d29	card "Territorial Allosaurus"	selected_analysis "Kicker {2}{G}\nWhen this creature enters, if it was kicked, it fights another target creature."
newly covered	07ef6fa6c759b259fd22109ec455243a6a1bfc76987ccb1c26e1fb88143095ae	card "Crown of Awe"	selected_analysis "Enchant creature\nEnchanted creature has protection from black and from red.\nSacrifice this Aura: Enchanted creature and other creatures that share a creature type with it gain protection from black and from red until end of turn."
newly covered	084a6280c3aaf8dd2f20523ee11b5f8b2f002d41cc6bd7d63d2f5afc28ff5570	card "Leave No Trace"	selected_analysis "Radiance — Destroy target enchantment and each other enchantment that shares a color with it."
newly covered	08c965f197f3f32a3a42135dd1117c93524d37d73377b310cfc6241009a65cbe	card "Crown of Vigor"	selected_analysis "Enchant creature\nEnchanted creature gets +1/+1.\nSacrifice this Aura: Enchanted creature and other creatures that share a creature type with it get +1/+1 until end of turn."
newly covered	0bfde3d89a4079428bfc327bbc11c7668fe11bc7757fccfc608c9738c8aacb24	card "Bull Hippo"	selected_analysis "Islandwalk"
newly covered	0df17661d03bf5ea7f8b59ed0e66ad9720eda4a1c799ce1d6ea56e4277c6d511	card "Burst Lightning"	selected_analysis "Kicker {4}\nBurst Lightning deals 2 damage to any target. If this spell was kicked, it deals 4 damage instead."
newly covered	0e35cd60bd8d36a42575804a10302545f3ffffa354cb2bf1796f7df52c03e39a	card "Falling Timber"	selected_analysis "Kicker—Sacrifice a land.\nPrevent all combat damage target creature would deal this turn. If this spell was kicked, prevent all combat damage another target creature would deal this turn."
newly covered	0eda53decd0bffbb99da390e1b3fe9a6a19d54ffe73e5bfcb658bacb1a1fc175	card "Eladamri, Lord of Leaves"	selected_analysis "Other Elf creatures have forestwalk.\nOther Elves have shroud."
newly covered	0f9d77f7365ab393f56b23890330592e88b4425d6417ad3c93f317e06ec7a27d	card "Phyrexian Warhorse"	selected_analysis "Kicker {W}\nWhen this creature enters, if it was kicked, create a 1/1 white Soldier creature token.\n{1}, Sacrifice another creature: This creature gets +2/+1 until end of turn."
newly covered	0ffbbd1f6165491aa58d65a0bd359d49a3636ee94d5d3bc3e27ef99679a959a7	card "Goblin Bushwhacker"	selected_analysis "Kicker {R}\nWhen this creature enters, if it was kicked, creatures you control get +1/+0 and gain haste until end of turn."
newly covered	10d92b5dd8bd8ee8008723d2a597b458d5101ff2e5d1d5b7a59bc5c13c9334c8	card "Yavimaya Iconoclast"	selected_analysis "Kicker {R}\nTrample\nWhen this creature enters, if it was kicked, it gets +1/+1 and gains haste until end of turn."
newly covered	11217b08454842f55d001fe65bbda8ad6606c4f3591034fd482dd00188f3467b	card "Emerald Oryx"	selected_analysis "Forestwalk"
newly covered	113841015cb555512104f5eb2e9497ba94122e8909512a039c454bdd8e3b2f17	card "Hope Tender"	selected_analysis "{1}, {T}: Untap target land.\n{1}, {T}, Exert this creature: Untap two target lands."
newly covered	11467cc9c2f156618be9c8bb07d688febaec4b76688cf5a9cdb2f8e7968b09f3	card "Scorch Rider"	selected_analysis "Kicker {1}{R}\nWhen this creature enters, if it was kicked, it gains haste until end of turn."
newly covered	115d04867395150dd006d993c67d24d5dc309cd59c9ff3bc2a4fc7c3580a1f3b	card "Elvish Hydromancer"	selected_analysis "Kicker {3}{U}\nWhen this creature enters, if it was kicked, create a token that's a copy of target creature you control."
newly covered	1287ad856e059b836084478474fc55f7a1eb0bc020f1b0c12e2182396fb94918	card "Ivy Dancer"	selected_analysis "{T}: Target creature gains forestwalk until end of turn."
newly covered	12c8e10f9741d46541e2f7f4830533a3c3121e266be6bf6a18c355cb2ad8cef3	card "Leaf-Crowned Elder"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, you may play that card without paying its mana cost."
newly covered	137c4f338d5c599d2aa6fbd4307ae68f3621a024a3a53f550aa2e7b70af481a6	card "Wandering Graybeard"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, you gain 4 life."
newly covered	14985c9dcc5f4d35b1743cb2608583d8c794679ff52e613fb18dfd3b072e897a	card "Zodiac Rat"	selected_analysis "Swampwalk"
newly covered	14d852bdf4a7064e13fdca092964ca0724da27ec83a37839c2ec787014e3db63	card "Weatherseed Elf"	selected_analysis "{T}: Target creature gains forestwalk until end of turn."
newly covered	169529e2596b3c936ebf161ddf774da3a5dec9d05a4266729370cd0201cc989a	card "Desolation Angel"	selected_analysis "Kicker {W}{W}\nFlying\nWhen this creature enters, destroy all lands you control. If it was kicked, destroy all lands instead."
newly covered	18c414affb05f47fcc7066b9d60208292fa7aacb4b644f5e886a3527ab7128c2	card "Invasion of New Capenna // Holy Frazzle-Cannon (Holy Frazzle-Cannon)"	selected_analysis "Whenever equipped creature attacks, put a +1/+1 counter on that creature and each other creature you control that shares a creature type with it.\nEquip {1}"
newly covered	195aaccd33c1f9ae792e9e89bab83475bcf7c46fb50b726812a8b61c47dc6a1e	card "Rowdy Crew"	selected_analysis "Trample\nWhen this creature enters, draw three cards, then discard two cards at random. If two cards that share a card type are discarded this way, put two +1/+1 counters on this creature."
newly covered	19aa8d84247ddaf64e4eefdf524cee3d0d8a5708a967a13e4860e2e01e26b955	card "Zodiac Snake"	selected_analysis "Swampwalk"
newly covered	1a3c30023977674bf25f47e46d041e34ad5de09948c76717fd3797f3a67463d3	card "Conjurer's Mantle"	selected_analysis "Equipped creature gets +1/+1 and has vigilance.\nWhenever equipped creature attacks, look at the top six cards of your library. You may reveal a card that shares a creature type with that creature from among them and put it into your hand. Put the rest on the bottom of your library in a random order.\nEquip {1}"
newly covered	1aa9e5b36cbfb32b5b75bba12a0e18c83339854ff2070c1857e9b5a15fc24242	card "Sylvan Echoes"	selected_analysis "Whenever you clash and win, you may draw a card."
newly covered	1b796fe5d15a32d4757f20b3350b9102e51065e163bf20a2db835ed796db3d25	card "Ghitu Chronicler"	selected_analysis "Kicker {3}{R}\nWhen this creature enters, if it was kicked, return target instant or sorcery card from your graveyard to your hand."
newly covered	1d48057736f7a959923aec0e97d9356c28a89bb910c740c72b2cbd21e643b843	card "Cleaving Skyrider"	selected_analysis "Flash\nKicker {2}{R}\nFlying\nWhen this creature enters, if it was kicked, it deals X damage to any target, where X is the number of attacking creatures."
newly covered	1e61c849e5d256261ade73a6e212c484f9d9444ce23f93186b700e48dac6d861	card "Nightshade Schemers"	selected_analysis "Flying\nKinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, each opponent loses 2 life."
newly covered	1e7747d51c373d65ec7a5b027f2cd3e89ade9233f49490008548ff3e7fa4c9af	card "Ravaging Riftwurm"	selected_analysis "Kicker {4}\nVanishing 2\nIf this creature was kicked, it enters with three additional time counters on it."
newly covered	1ebec9794ff48ac340c9278216eb19e104fbf9c78f352bf8bfad50ae273afecd	card "Sowing Mycospawn"	selected_analysis "Devoid\nKicker {1}{C}\nWhen you cast this spell, search your library for a land card, put it onto the battlefield, then shuffle.\nWhen you cast this spell, if it was kicked, exile target land."
newly covered	1f0adc6117af65de7180c4d42d433cf0bd3459b2d969c59839bab61082d5dfee	card "Stoneforge Masterwork"	selected_analysis "Equipped creature gets +1/+1 for each other creature you control that shares a creature type with it.\nEquip {2}"
newly covered	215fd60b261c13bc6c7979b1bbfb9197acb1bcb9a7f67944000e148214ee8422	card "Tattermunge Duo"	selected_analysis "Whenever you cast a red spell, this creature gets +1/+1 until end of turn.\nWhenever you cast a green spell, this creature gains forestwalk until end of turn."
newly covered	216c8e89011c0dc3386725597a2357e9f45855850d78e24c5ad79a82150db4cf	card "Krosan Druid"	selected_analysis "Kicker {4}{G}\nWhen this creature enters, if it was kicked, you gain 10 life."
newly covered	23e36df0bb11eb11c6c7df016fdd0e8d29302829ab164c86200bdbb98704931f	card "Segovian Leviathan"	selected_analysis "Islandwalk"
newly covered	24ca5976be68288164ce21625000fb809b4ffed0e6c6990e23816cc6e9eaf0b3	card "Jedit Ojanen, Mercenary"	selected_analysis "Whenever Jedit Ojanen or another legendary creature you control enters, you may pay {G}. If you do, create a 2/2 green Cat Warrior creature token with forestwalk."
newly covered	2657b07184edbbff3203269c18aa13b138749549971199d1bde9a533cc1f1ca1	card "Leaf Dancer"	selected_analysis "Forestwalk"
newly covered	29b122219fd6fd4136a4af5a28ca45a87956c52fac322856ebdb008aff45a568	card "Spreading Plague"	selected_analysis "Whenever a creature enters, destroy all other creatures that share a color with it. They can't be regenerated."
newly covered	29c48a5a15860dfbf316e300c208cccd763835cf4e1cc341074003f5de0af04a	card "Elvish Pathcutter"	selected_analysis "{2}{G}: Target Elf creature gains forestwalk until end of turn."
newly covered	29cff680b63be598d97630b7b3ddfca91f4ae9ce8e6808a5baa7127f2f6a3b3a	card "Inkwell Leviathan"	selected_analysis "Trample\nIslandwalk\nShroud"
newly covered	2b2305b921ac80f336012ec56e721dd4949ed28d06abffefab7629f50e40f3d9	card "Bog Smugglers"	selected_analysis "Swampwalk"
newly covered	2b9c7ae56b3f5a03389036c7cc54e986516c4935032a9830cf8795f119948462	card "Verduran Emissary"	selected_analysis "Kicker {1}{R}\nWhen this creature enters, if it was kicked, destroy target artifact. It can't be regenerated."
newly covered	2dbf4c4fdb7f4d304616ebe875322fc1caf99b728ef38ed64a50b18bf497bdf9	card "Orim's Thunder"	selected_analysis "Kicker {R}\nDestroy target artifact or enchantment. If this spell was kicked, it deals damage equal to that permanent's mana value to target creature."
newly covered	322ea77c23325cf171ba9abc89c47c608d50519d5270165f173d1d7219d0e743	card "Rootwater Commando"	selected_analysis "Islandwalk"
newly covered	3282b980b2ba3214328bf1cf00cf66440c6a25a182a47bd374a151c5b8842cb6	card "Vayne's Treachery"	selected_analysis "Kicker—Sacrifice an artifact or creature.\nTarget creature gets -2/-2 until end of turn. If this spell was kicked, that creature gets -6/-6 until end of turn instead."
newly covered	333dc546ea29f942f8a290c77de2a24105bbfb3764cbb4d6c0141c00a37a3fc3	card "Mire Boa"	selected_analysis "Swampwalk\n{G}: Regenerate this creature."
newly covered	3368f3bbc496a24971d6e63d0ef80fbe2d0299f0bb77b6a21e6592e0ff5d9ce5	card "Vampire's Bite"	selected_analysis "Kicker {2}{B}\nTarget creature gets +3/+0 until end of turn. If this spell was kicked, that creature gains lifelink until end of turn."
newly covered	3411a90bd701261e29fa8d5caa2c0e5803d990e96a13c906430a1d97d8441f32	card "Cold-Eyed Selkie"	selected_analysis "Islandwalk\nWhenever this creature deals combat damage to a player, you may draw that many cards."
newly covered	3421c6b956fb276247ab99211c932b818e1a019e3dab3614fc40e96d2b5bfaad	card "Basri, Tomorrow's Champion"	selected_analysis "{W}, {T}, Exert Basri: Create a 1/1 white Cat creature token with lifelink.\nCycling {2}{W}\nWhen you cycle this card, Cats you control gain hexproof and indestructible until end of turn."
newly covered	3490eeaf6daecae20586ad34abb231f90794d3c510175f458d2e468105cc0d64	card "Nature's Cloak"	selected_analysis "Green creatures you control gain forestwalk until end of turn."
newly covered	35aad87bbb700f938f634fc5cd479f8fecf38310d28c35f7869c4e0ecd581655	card "Vine Gecko"	selected_analysis "The first kicked spell you cast each turn costs {1} less to cast.\nWhenever you cast a kicked spell, put a +1/+1 counter on this creature."
newly covered	35fb680b7bd424da9a0df5f3441e7fc19dc5aa9743dcd9e15c96642944bd5272	card "Shivan Fire"	selected_analysis "Kicker {4}\nShivan Fire deals 2 damage to target creature. If this spell was kicked, it deals 4 damage instead."
newly covered	36360384b060fe29c388c814fa53e91e76c2b3160b5b594af9bfd7ee2f93c140	card "Research the Deep"	selected_analysis "Draw a card. Clash with an opponent. If you win, return Research the Deep to its owner's hand."
newly covered	363bab7519b2f8ad875e9de8d13f664139d8def08facf1b57c45bf21ab0095ce	card "Descendants' Path"	selected_analysis "At the beginning of your upkeep, reveal the top card of your library. If it's a creature card that shares a creature type with a creature you control, you may cast it without paying its mana cost. If you don't cast it, put it on the bottom of your library."
newly covered	37a996bc03a89801e0b69a4e2c0f54b189a14164debff6c5a8d0c509a0d37697	card "Academy Drake"	selected_analysis "Kicker {4}\nFlying\nIf this creature was kicked, it enters with two +1/+1 counters on it."
newly covered	37db21facbd622609eec4907b125e469f77424525a3feb94139e7d98a134683d	card "Halimar Wavewatch"	selected_analysis "Level up {2}\nLEVEL 1-4\n0/6\nLEVEL 5+\n6/6\nIslandwalk"
newly covered	37f87c601c2566eb99aeb5368845cb08b90cd6d0f9eaaf8b45199cf5c788f034	card "Gilt-Leaf Ambush"	selected_analysis "Create two 1/1 green Elf Warrior creature tokens. Clash with an opponent. If you win, those creatures gain deathtouch until end of turn."
newly covered	394864f148edabd19e9a77d28babec4b8151a912ee09d49a1a8e873d9b2dadec	card "Kitesail Cleric"	selected_analysis "Kicker {2}{W}\nFlying\nWhen this creature enters, if it was kicked, tap up to two target creatures."
newly covered	39c38ab36a90336f32dfe9289bd370b9360448b778032c8902700c82c693610a	card "Rally the Righteous"	selected_analysis "Radiance — Untap target creature and each other creature that shares a color with it. Those creatures get +2/+0 until end of turn."
newly covered	39f4d66b8ad667585aa00c1cd56959d78fe873d7ca94e0080753304624972696	card "Weight of Conscience"	selected_analysis "Enchant creature\nEnchanted creature can't attack.\nTap two untapped creatures you control that share a creature type: Exile enchanted creature."
newly covered	3c3998e7a15a6eadebe759c6a46045eab4633cb1b1747c652d38a9a30768af08	card "Caligo Skin-Witch"	selected_analysis "Kicker {3}{B}\nWhen this creature enters, if it was kicked, each opponent discards two cards."
newly covered	3d4aab25fba2bff2bfc89f03c19a93b6991a5fdea6d9ae3621899bb04362a8f2	card "Moor Fiend"	selected_analysis "Swampwalk"
newly covered	3ddf1187159e67186e46daac898f6b00d3a1d0f3ec3597f5d0184e4bdfc91f99	card "Fight with Fire"	selected_analysis "Kicker {5}{R}\nFight with Fire deals 5 damage to target creature. If this spell was kicked, it deals 10 damage divided as you choose among any number of targets instead."
newly covered	3e512c2933f9ecea0bd934b2b90bd715c5433a26e2c4737777cf05a4abfb2b17	card "Zodiac Goat"	selected_analysis "Mountainwalk"
newly covered	4058ef187e439ed4c00683b84c0dad80773e1e1686e4ea47263bfa4c76f6c0d2	card "Cinderclasm"	selected_analysis "Kicker {R}\nCinderclasm deals 1 damage to each creature. If it was kicked, it deals 2 damage to each creature instead."
newly covered	405b42903282f35b8948ea6b65c371649b40ee5936fd1701fd1dbeba786f7245	card "Weed Strangle"	selected_analysis "Destroy target creature. Clash with an opponent. If you win, you gain life equal to that creature's toughness."
newly covered	412a476810b51dc52ee0864c60d1416d0305e75256b8e3a1fe079e964fcb8e43	card "Lumbering Satyr"	selected_analysis "All creatures have forestwalk."
newly covered	412cd75ec057d4df73e90acb9b2294309812edeccb36eaab0d9ecc7f21ebd002	card "Devouring Deep"	selected_analysis "Islandwalk"
newly covered	4260ed7401bdb2fcf3cfe5d10f05c7f2a44a328e4ba7bf3d6a7bb2785b4750da	card "Marsh Goblins"	selected_analysis "Swampwalk"
newly covered	426401300f49f834a621592058cd5f236332efa9ade7326899f1db40a8cafa9e	card "Lord of Atlantis"	selected_analysis "Other Merfolk get +1/+1 and have islandwalk."
newly covered	436ac5ac52958c91cceb8eee4eaedcecee8347bdca541f641e13f861ba8d3f1c	card "Wojek Siren"	selected_analysis "Radiance — Target creature and each other creature that shares a color with it get +1/+1 until end of turn."
newly covered	437ca9817794d61cd3cbbbfef76c496dffed62d12807ae1ff193085f7975bcaf	card "Goblin Scouts"	selected_analysis "Create three 1/1 red Goblin Scout creature tokens with mountainwalk."
newly covered	4387f7933e11bab572314738fd02d03042b55ee0ba0f91085d95a542bff55b7c	card "Timely Interference"	selected_analysis "Kicker {1}{R}\nTarget creature gets -1/-0 until end of turn. If this spell was kicked, that creature blocks this turn if able.\nDraw a card."
newly covered	45deefefa414997e44a5ee1f3457c4917fb6ead6a9c64db4d91d169a90bb283c	card "Firebending Lesson"	selected_analysis "Kicker {4}\nFirebending Lesson deals 2 damage to target creature. If this spell was kicked, it deals 5 damage to that creature instead."
newly covered	465432b7c64e6c82c629fab8b49a90fd4853573cd45d97aec9b7415c642f4622	card "Sewer Shambler"	selected_analysis "Swampwalk\nScavenge {2}{B}"
newly covered	46d3e2c607eeedb2a39fe335ca9aa8499fa796e03e3fab01d7cec4e353c9a409	card "Sol'kanar the Swamp King"	selected_analysis "Swampwalk\nWhenever a player casts a black spell, you gain 1 life."
newly covered	46e40a24b75d178aca2ea6a1dfb7ac6808b4c2c94c002bff2fcf3026c8621bc3	card "Zombie Master // Zombie Master (Zombie Master)"	selected_analysis "Other Zombie creatures have swampwalk.\nOther Zombies have \"{B}: Regenerate this permanent.\""
newly covered	4731656ec9f7b26d5631ad020dc62a9692786c9aa7663df925e69b35c4797157	card "Might of Murasa"	selected_analysis "Kicker {2}{G}\nTarget creature gets +3/+3 until end of turn. If this spell was kicked, that creature gets +5/+5 until end of turn instead."
newly covered	47bd74ec3c4d8b0ee08d7a3620e78400b788756779dda34a7be64d5589ea9a1a	card "Return from Extinction"	selected_analysis "Choose one —\n• Return target creature card from your graveyard to your hand.\n• Return two target creature cards that share a creature type from your graveyard to your hand."
newly covered	47d38caa8d7246ce8c00b271660e90fd8f79b43456a12018f9040e03b8c7bc6e	card "Gift of Growth"	selected_analysis "Kicker {2}\nUntap target creature. It gets +2/+2 until end of turn. If this spell was kicked, that creature gets +4/+4 until end of turn instead."
newly covered	481186910dca9600dfde8ca97bba93b1ab40205e6c5c52c7e77eb19a0aef8515	card "Tear Asunder"	selected_analysis "Kicker {1}{B}\nExile target artifact or enchantment. If this spell was kicked, exile target nonland permanent instead."
newly covered	487a68e7856a41456329449a4ead20814f4d3854218e9475ecc9737aedf9a554	card "Goblin King"	selected_analysis "Other Goblins get +1/+1 and have mountainwalk."
newly covered	4a3e3b99bda3cd6c3ac805a4cf023747e9914abd0e6475b8f3af0a0d67743c0e	card "Cave People"	selected_analysis "Whenever this creature attacks, it gets +1/-2 until end of turn.\n{1}{R}{R}, {T}: Target creature gains mountainwalk until end of turn."
newly covered	4b8462d29c7fe0f88355d41743be9ba01437e2b9c495afa6a47b11f44cb14941	card "Heartwood Treefolk"	selected_analysis "Forestwalk"
newly covered	4c41ba0f1959e4e263c120553862b79dcd290a6cb134f62eb33feec40e2a3dd3	card "Wolf-Skull Shaman"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, create a 2/2 green Wolf creature token."
newly covered	4d1205ce07546bba04ebd36b288e170c96c7154a306c63b65b88dabbb7cbbd50	card "Juniper Order Rootweaver"	selected_analysis "Kicker {G}\nWhen this creature enters, if it was kicked, put a +1/+1 counter on target creature you control."
newly covered	4e40994835b9f0a1940804e0a2279d57df64704569e4f61b59fba6ee5879a9a0	card "Hypnotic Cloud"	selected_analysis "Kicker {4}\nTarget player discards a card. If this spell was kicked, that player discards three cards instead."
newly covered	4e61bcf539a02e2311540698b56a9842ae8514fe20d5a21823fca3d46ed07a8c	card "Shared Animosity"	selected_analysis "Whenever a creature you control attacks, it gets +1/+0 until end of turn for each other attacking creature that shares a creature type with it."
newly covered	4e6a397dfeae542201d47eae5ed34d3873fa764dcb42751bc810eef348df9c94	card "Dryad's Favor"	selected_analysis "Enchant creature\nEnchanted creature has forestwalk."
newly covered	4ec2bc350dc5ccc18600482bfcd33030a175b225e3b9e68ffe4ce4ea0a11df72	card "Coral Barrier"	selected_analysis "Defender\nWhen this creature enters, create a 1/1 blue Squid creature token with islandwalk."
newly covered	4ec405662522cc8c47b6287959c47975d753ab8359ad0075909286faaac79c25	card "Nighthaze"	selected_analysis "Target creature gains swampwalk until end of turn.\nDraw a card."
newly covered	50007460668ec39d50184ca6478c117eac667298028d414b6023f700a24e02b1	card "Veldrane of Sengir"	selected_analysis "{1}{B}{B}: Veldrane gets -3/-0 and gains forestwalk until end of turn."
newly covered	50442fb71424557b0506e5a541f4c679ad5b5a1c94a860906ba355d683aa35cf	card "Wojek Apothecary"	selected_analysis "Radiance — {T}: Prevent the next 1 damage that would be dealt to target creature and each other creature that shares a color with it this turn."
newly covered	50eaf26e6f69563cbd31939da001cec60c0e663268bf67448d29d865163c73cc	card "Illuminated Folio"	selected_analysis "{1}, {T}, Reveal two cards from your hand that share a color: Draw a card."
newly covered	5283483d2efa7040758ac00e2c02f32a31b83d55c7165f43d8fb526719bf2385	card "Cliff Threader"	selected_analysis "Mountainwalk"
newly covered	53861d226c897fb7724a4258fc43ade9f31f54020d056bc01e225b3993d34766	card "Gnarlid Colony"	selected_analysis "Kicker {2}{G}\nIf this creature was kicked, it enters with two +1/+1 counters on it.\nEach creature you control with a +1/+1 counter on it has trample."
newly covered	54034ba3805ca90524b984ed9d5b366ad11a090314e923f6441c45110470eb0b	card "Ink Dissolver"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, each opponent mills three cards."
newly covered	55d970567fba6934a66fbc8a8a9b64bfc298a8dd6f9412ae71c0e6db2d7c42ee	card "Marsh Threader"	selected_analysis "Swampwalk"
newly covered	566812a782856dc3f81a591915a57a988034a939f431967dbd45bd340e8ff508	card "Explosive Growth"	selected_analysis "Kicker {5}\nTarget creature gets +2/+2 until end of turn. If this spell was kicked, that creature gets +5/+5 until end of turn instead."
newly covered	566f8339a74a461d8dd373f3eecb410402004b05d7892d7410cd7ae7de0982b9	card "Pale Bears"	selected_analysis "Islandwalk"
newly covered	56807d69de96c412affd3b96ad0942b1c9667d15a9f281bd9b4aef5db39f1f0f	card "Glissa's Courier"	selected_analysis "Mountainwalk"
newly covered	57b0f2ba1a6ad3467b72b60e5474e24727bf5a68a876703ff6a7b98749050bad	card "Kavu Primarch"	selected_analysis "Kicker {4}\nConvoke\nIf this creature was kicked, it enters with four +1/+1 counters on it."
newly covered	57f0516588096b71e139dfb27aab8e7199ad8e5b88bcb07e86aa147579f97531	card "Savage Offensive"	selected_analysis "Kicker {G}\nCreatures you control gain first strike until end of turn. If this spell was kicked, they get +1/+1 until end of turn."
newly covered	583ac6f54dd3f4230615f6f5bf604bde738d50bd66716d4604a190114ed891e4	card "Benthic Djinn"	selected_analysis "Islandwalk\nAt the beginning of your upkeep, you lose 2 life."
newly covered	588314f44e1abda1f88f03f9ee7ffc9cd1964d6ef0b1c757c62a3da340db803b	card "Sergeant-at-Arms"	selected_analysis "Kicker {2}{W}\nWhen this creature enters, if it was kicked, create two 1/1 white Soldier creature tokens."
newly covered	58a19b6463e8244d755b2a4e0c2c5dbb7cde64187420334f25dc9e172c027979	card "Shatterskull Charger"	selected_analysis "Kicker {2}\nTrample, haste\nIf this creature was kicked, it enters with a +1/+1 counter on it.\nAt the beginning of your end step, if this creature doesn't have a +1/+1 counter on it, return it to its owner's hand."
newly covered	58b78cc1e7a6b66c8b26a2f8c36a48b1e2983bfa083d8bd5de145295baec858e	card "Whoosh!"	selected_analysis "Kicker {1}{U}\nReturn target nonland permanent to its owner's hand. If this spell was kicked, draw a card."
newly covered	5921882f65a463e76ef00bad54ca4a3d5fb26082491efe132464b384361f4648	card "Daring Thief"	selected_analysis "Inspired — Whenever this creature becomes untapped, you may exchange control of target nonland permanent you control and target permanent an opponent controls that shares a card type with it."
newly covered	59c69f08b6f26965758904020c8c9e34287655b306315a2cb48aa2a5333fa6e9	card "Rishadan Dockhand"	selected_analysis "Islandwalk\n{1}, {T}: Tap target land."
newly covered	5a2050ad634805d2dd08140ebb85edace4c3731862152c49b4af123572fe852f	card "Cave Sense"	selected_analysis "Enchant creature\nEnchanted creature gets +1/+1 and has mountainwalk."
newly covered	5a63df827ad5b7b71e2d7300de431219fb6cc5991a19d6fd0dc68564c2b3064b	card "Ardent Soldier"	selected_analysis "Kicker {2}\nVigilance\nIf this creature was kicked, it enters with a +1/+1 counter on it."
newly covered	5bb8c5387b66faa71213cb22f752402b4b1ba819fb7e16f13931d90560936a56	card "Field Research"	selected_analysis "Kicker {2}{U}\nDraw two cards. If this spell was kicked, draw three cards instead."
newly covered	5c0eccb43dc0ba09dc4b66839e7656ac33cfa9a95bb50eef8559c87d7e2773ed	card "Stalker Hag"	selected_analysis "Swampwalk, forestwalk"
newly covered	5e990b3c6e1cb85a9f218f3f238ce9f4e4559f48509de7ee9ed1ac54598d9423	card "Bog-Strider Ash"	selected_analysis "Swampwalk\nWhenever a player casts a Goblin spell, you may pay {G}. If you do, you gain 2 life."
newly covered	5f14357e4538c5ef234e010c7c4a168986eb816480594788819628aaad5cf785	card "Kithkin Zephyrnaut"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, this creature gets +2/+2 and gains flying and vigilance until end of turn."
newly covered	60384d9c8d4348e4c36f2ca126408f3e3de39c0b7d7ea34ae6e5e2f40811e9ea	card "Elemental Appeal"	selected_analysis "Kicker {5}\nCreate a 7/1 red Elemental creature token with trample and haste. Exile it at the beginning of the next end step. If this spell was kicked, that creature gets +7/+0 until end of turn."
newly covered	60d4d93affb54f5d0b43923a9d0cc32719a8b36e2a15dbcb22986371ad17ce2d	card "Sewerdreg"	selected_analysis "Swampwalk\nSacrifice this creature: Exile target card from a graveyard."
newly covered	6168d2be0f71bda8ef3bb46ef32af3f51001d7f88b7376db2d7ade7f0201530a	card "Whirlpool Whelm"	selected_analysis "Clash with an opponent, then return target creature to its owner's hand. If you win, you may put that creature on top of its owner's library instead."
newly covered	627338eb398efae04e4c0017b81488bc08a374c7ba2ef7cc11e474a131db8973	card "Zodiac Monkey"	selected_analysis "Forestwalk"
newly covered	665f86d892bcaeede12e014a91651abc47ed85bdc5550e8fc0ccebb73c2938cc	card "Hurloon Battle Hymn"	selected_analysis "Kicker {W}\nHurloon Battle Hymn deals 4 damage to target creature or planeswalker. If this spell was kicked, you gain 4 life."
newly covered	66d19ff8b8281d7d6691b94b59d4743ac59ef700ac281554e7cccc7b0b665e31	card "Mold Shambler"	selected_analysis "Kicker {1}{G}\nWhen this creature enters, if it was kicked, destroy target noncreature permanent."
newly covered	67086b4c8d7703deec5dfdb1496b96efa3e1e0262b1bcb4a4798656b3da58509	card "Cemetery Protector"	selected_analysis "Flash\nWhen this creature enters, exile a card from a graveyard.\nWhenever you play a land or cast a spell, if it shares a card type with the exiled card, create a 1/1 white Human creature token."
newly covered	67c570fae03fbb46f284582046394f6d0a76c4dff38ebd364849ec1807fcbd98	card "Legerdemain"	selected_analysis "Exchange control of target artifact or creature and another target permanent that shares one of those types with it."
newly covered	69790c0351df4f8949ea4277cde37d79b2d8ca86968b87d95af97246df2f699e	card "Cunning Geysermage"	selected_analysis "Kicker {2}{U}\nWhen this creature enters, if it was kicked, return up to one other target creature to its owner's hand."
newly covered	6a179833d0a74addf99e4e255f1b342218e3ce36247fe0a2faeb0bcee4b96272	card "Orim's Touch"	selected_analysis "Kicker {1}\nPrevent the next 2 damage that would be dealt to any target this turn. If this spell was kicked, prevent the next 4 damage that would be dealt to that permanent or player this turn instead."
newly covered	6ad77da1943ff5a7b36b65b0a806ef36d798728d94a3c3d0649a4cbe1b3bb3d5	card "Tazeem Roilmage"	selected_analysis "Kicker {4}\nWhen this creature enters, if it was kicked, return target instant or sorcery card from your graveyard to your hand."
newly covered	6b63fe2834509f4fdffa78e0b6d08cd9f596c73b54bed4ce31a9bff7b4e62755	card "Stonybrook Banneret"	selected_analysis "Islandwalk\nMerfolk spells and Wizard spells you cast cost {1} less to cast."
newly covered	6c42895e6bf4deb13627defd1d7182edf8d08954b044f7b72024d9b3e8696316	card "Oasis Ritualist"	selected_analysis "{T}: Add one mana of any color.\n{T}, Exert this creature: Add two mana of any one color."
newly covered	6d0af54f3b61bdece83f83090bff4f1d9e6a918534df9b6aee7633ab49f8e8dd	card "Wrexial, the Risen Deep"	selected_analysis "Islandwalk, swampwalk\nWhenever Wrexial deals combat damage to a player, you may cast target instant or sorcery card from that player's graveyard without paying its mana cost. If that spell would be put into a graveyard, exile it instead."
newly covered	6d1e0461dafffbc20ecf1bbd85cf7532469565858419e24656221ecda8b324a2	card "Hidden Path"	selected_analysis "Green creatures have forestwalk."
newly covered	6d5ea1c331598ca1bf9ebeabde9405469dc32d307b9fb14ca9b4f0f4706c66ef	card "Harbor Serpent"	selected_analysis "Islandwalk\nThis creature can't attack unless there are five or more Islands on the battlefield."
newly covered	6dc71085fe43af409f2fbe61d4a28d9b0a38fc77392975714a157efe53f10886	card "Lynx"	selected_analysis "Forestwalk"
newly covered	6df4612932e9a9669aed2c7eced36cad51beb0d56511f5227e8be5d6ea1a7d0c	card "Tolarian Geyser"	selected_analysis "Kicker {W}\nReturn target creature to its owner's hand. Draw a card. If this spell was kicked, you gain 3 life."
newly covered	6ec0297538fbc9a7be5ae03f4903ba35b5e3a6c3a15d0db166cc117975ec047d	card "Plague Beetle"	selected_analysis "Swampwalk"
newly covered	6f0390133f69bfbe2eb4d1eb4eb659e3eda8d29ae1b92fe3697c780d0b848b2a	card "Raiding Nightstalker"	selected_analysis "Swampwalk"
newly covered	6f057a235a88aa5df39f5fb24a8f8137e148b288ea74357143308b001f47eb5d	card "Zodiac Rabbit"	selected_analysis "Forestwalk"
newly covered	6f773fefbc9783395b4cb17d0ab1eef285482aa92456405727dcf184a0682ec4	card "Goblins of the Flarg"	selected_analysis "Mountainwalk\nWhen you control a Dwarf, sacrifice this creature."
newly covered	7080d028b5d9dbe010582845dfd006cdf8acccd8caa806f515c947624519a18c	card "Probe"	selected_analysis "Kicker {1}{B}\nDraw three cards, then discard two cards. If this spell was kicked, target player discards two cards."
newly covered	71cd39795fcb45b80c005acc48aee07afda7316ab3a3fd220bf5655a66b0455c	card "Anurid Murkdiver"	selected_analysis "Swampwalk"
newly covered	73c1a18e78bbdb795b48d8b1bf7fc0b83c365ce20aec4c715649ca5581deac55	card "Vine Dryad"	selected_analysis "You may exile a green card from your hand rather than pay this spell's mana cost.\nFlash\nForestwalk"
newly covered	744081a2f11e8a93c4d79fb2e94900b09da08537f23faa9720089ddfee79667b	card "Broken Ambitions"	selected_analysis "Counter target spell unless its controller pays {X}. Clash with an opponent. If you win, that spell's controller mills four cards."
newly covered	75b978c15646220a039e4fd386f011361d25a8229a8c356860821ebd8445b3b2	card "Steward of Solidarity"	selected_analysis "{T}, Exert this creature: Create a 1/1 white Warrior creature token with vigilance."
newly covered	76dda8bf919592b17ef034cd68343d3cde68e656ef6cf9c028a1eaf084f7f21d	card "Bog Raiders"	selected_analysis "Swampwalk"
newly covered	78dd023eee3d80b1f18a27b57857d3d4a7819f6d4961d6b5208f6d712963af3a	card "Urborg Emissary"	selected_analysis "Kicker {1}{U}\nWhen this creature enters, if it was kicked, return target permanent to its owner's hand."
newly covered	7a2d78a735c96c954c54e1f6850b8bf8f65b8c4fc6f3b2b4821fdd28b66ca02b	card "Elvish Champion"	selected_analysis "Other Elf creatures get +1/+1 and have forestwalk."
newly covered	7a85961c32087723ff147326837dad209025b4a378d8e7aec58e2af89dd23a80	card "Jilt"	selected_analysis "Kicker {1}{R}\nReturn target creature to its owner's hand. If this spell was kicked, it deals 2 damage to another target creature."
newly covered	7a9ff1d9a306d54df435465dcd93c71016aaa6ebaced4da1606d284c412025a9	card "Keldon Overseer"	selected_analysis "Kicker {3}{R}\nHaste\nWhen this creature enters, if it was kicked, gain control of target creature until end of turn. Untap that creature. It gains haste until end of turn."
newly covered	7c3e27225b39ae6f712adf28c52979472a166e41c14f51e77bd8a855df2a8456	card "Waterspout Weavers"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, each creature you control gains flying until end of turn."
newly covered	7d8b621936b1597157883661430866f56437ca2f5970f084393b39e0430c96b5	card "Fires of Victory"	selected_analysis "Kicker {2}{U}\nIf this spell was kicked, draw a card. Fires of Victory deals damage to target creature or planeswalker equal to the number of cards in your hand."
newly covered	7d9e8343dc6d87e61d788f70b54137a57b9eb5d4836b0e83bb1cb4c03b21a2ea	card "Piracy Charm"	selected_analysis "Choose one —\n• Target creature gains islandwalk until end of turn.\n• Target creature gets +2/-1 until end of turn.\n• Target player discards a card."
newly covered	7df89f6a0dbd624d1c932dbf1d6e7bb9e51f030b0a9ac6e1ad99fa1265ddde21	card "Confusion in the Ranks"	selected_analysis "Whenever an artifact, creature, or enchantment enters, its controller chooses target permanent another player controls that shares a card type with it. Exchange control of those permanents."
newly covered	7e2066decb85bfe57e5a0fa53551bcfe3218eaf3a4f01eef4dc91392234a0889	card "Runic Shot"	selected_analysis "Kicker {U}\nDestroy target tapped creature. If this spell was kicked, scry 2."
newly covered	7e67d22c86c67b1cface3dc5c090ca375396b82ffaf7c037d272af44f77dba06	card "Woodland Guidance"	selected_analysis "Return target card from your graveyard to your hand. Clash with an opponent. If you win, untap all Forests you control.\nExile Woodland Guidance."
newly covered	7e9d059a28010450362e487fb14a91120ace34fad4fc2356d30f5af5f4af82f2	card "Leshrac's Rite"	selected_analysis "Enchant creature\nEnchanted creature has swampwalk."
newly covered	7e9e910d2c7ea6eb087be2ca8966fae9a1048c7d79eddfb58e06e5f8c13cc75e	card "Zombie Master"	selected_analysis "Other Zombie creatures have swampwalk.\nOther Zombies have \"{B}: Regenerate this permanent.\""
newly covered	7ea968e77bf057085661c561a77c2daa706fd9288f54a135d97a22a040c2a740	card "Excavation Elephant"	selected_analysis "Kicker {1}{W}\nWhen this creature enters, if it was kicked, return target artifact card from your graveyard to your hand."
newly covered	7f12b8ff2f1dd2342f4491d6ba31583f198852ce2fc6292c4bb25a05c4886cb6	card "Urborg Skeleton"	selected_analysis "Kicker {3}\n{B}: Regenerate this creature.\nIf this creature was kicked, it enters with a +1/+1 counter on it."
newly covered	80705a21002fa55d433fff9e6c0c3f685f82a275a921c4ce758bfb1652ad76d4	card "Pyroclast Consul"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, this creature deals 2 damage to each creature."
newly covered	8133afef5e1ab0622ed7b81e56b6a07da8faec6ded7e66ea3b1d6e6c94e095bd	card "Slithery Stalker"	selected_analysis "Swampwalk\nWhen this creature enters, exile target green or white creature an opponent controls.\nWhen this creature leaves the battlefield, return the exiled card to the battlefield under its owner's control."
newly covered	81be71cbb720990c299e6ecf40032c4eae1f30b7f023c3cb01ce98186e8c7a61	card "Skyclave Sentinel"	selected_analysis "Kicker {4}\nFlying, defender\nIf this creature was kicked, it enters with two +1/+1 counters on it.\nAs long as this creature has a +1/+1 counter on it, it can attack as though it didn't have defender."
newly covered	81cd379602089aebd7c528d4f7919e91f13987e64060947c3b1a4afc6ae9e6e6	card "Yavimaya Dryad"	selected_analysis "Forestwalk\nWhen this creature enters, you may search your library for a Forest card, put it onto the battlefield tapped under target player's control, then shuffle."
newly covered	82c735f65ae63995c48920c079146e57c68b670ed4192060803b4f673f5851cd	card "Spring Cleaning"	selected_analysis "Destroy target enchantment. Clash with an opponent. If you win, destroy all enchantments your opponents control."
newly covered	8301ab4bfd6e5bfdf116101e2bcb7f0c44801abee0e06417cacecda12f513836	card "Unbury"	selected_analysis "Choose one —\n• Return target creature card from your graveyard to your hand.\n• Return two target creature cards that share a creature type from your graveyard to your hand."
newly covered	8394f40ab596d94d28b1f9d2626d00fd77ec3bc6acaa47c8dc36351bd9ac46dc	card "Lash Out"	selected_analysis "Lash Out deals 3 damage to target creature. Clash with an opponent. If you win, Lash Out deals 3 damage to that creature's controller."
newly covered	8445da2df40d7670ef9b345e8d59e138d9ee3090fb6f98c0a6be821ef505f585	card "Shifting Loyalties"	selected_analysis "Exchange control of two target permanents that share a card type."
newly covered	848ae6862cbce0ae97a1781209dd5534b6956c7f8cc47e9a746ff052183360ed	card "Deeptread Merrow"	selected_analysis "{U}: This creature gains islandwalk until end of turn."
newly covered	84fc37fe62de412ba73b87c1b758470362826d33f6698ce9933637f9f1570248	card "Hillcomber Giant"	selected_analysis "Mountainwalk"
newly covered	85e2ad4f170dde0315bdded04cf369ab18622c6f7dbbf1bcc2f8c3ad9fd9f411	card "Wild Ox"	selected_analysis "Swampwalk"
newly covered	862c7e3ed9135c4f73413d546b39178e5d1f3bb0f9902a516708aa7c135f12bf	card "Bog Hoodlums"	selected_analysis "This creature can't block.\nWhen this creature enters, clash with an opponent. If you win, put a +1/+1 counter on this creature."
newly covered	869d3f166a370d16f96f98bfe33344442d72fdc8ddbe71e5d73b5219de2d7854	card "Joint Exploration"	selected_analysis "Kicker {G}\nScry 2, then draw a card. If this spell was kicked, you may put a land card from your hand onto the battlefield."
newly covered	871c25b76c8e32a4021fc53c6e29c0da6142cfb73fedaf427a53114b0a2df955	card "Ghastly Gloomhunter"	selected_analysis "Kicker {3}{B}\nFlying, lifelink\nIf this creature was kicked, it enters with two +1/+1 counters on it."
newly covered	874a291c53a3ddc510700701c634b99013157dbd3020050dea6a334313eb2e90	card "Oran-Rief Recluse"	selected_analysis "Kicker {2}{G}\nReach\nWhen this creature enters, if it was kicked, destroy target creature with flying."
newly covered	87bf91530bbcd54f1e699d61086ca2837eb920446e29eebbc4a83269ec403066	card "Whispering Shade"	selected_analysis "Swampwalk\n{B}: This creature gets +1/+1 until end of turn."
newly covered	87d4edf2e8111bf1a628f56583fbaf37b068489c6a9b60c2d158f4258330239b	card "Marsh Casualties"	selected_analysis "Kicker {3}\nCreatures target player controls get -1/-1 until end of turn. If this spell was kicked, those creatures get -2/-2 until end of turn instead."
newly covered	88375576b61a7173681db305fc887917da62c97b2ea4a6cb69cbf3ffaa164a49	card "Coward // Killer (Killer)"	selected_analysis "Killer deals 3 damage to target creature and each other creature that shares a creature type with it."
newly covered	885cccc5dbaee1052cdec09eeb44464e780673cd9e9d88c6f94baacbd7e2840f	card "Enslaved Scout"	selected_analysis "{2}: This creature gains mountainwalk until end of turn."
newly covered	88b7c5a7ea35ef8ef4a756e084df2914e6ddcaf9029f4763ef49a0743b0c250c	card "Pride Sovereign"	selected_analysis "This creature gets +1/+1 for each other Cat you control.\n{W}, {T}, Exert this creature: Create two 1/1 white Cat creature tokens with lifelink."
newly covered	894be821c0ba0efa3fc28154339bb01491c61ac4f8de7783c819db67f5b0df67	card "Kor Aeronaut"	selected_analysis "Kicker {1}{W}\nFlying\nWhen this creature enters, if it was kicked, target creature gains flying until end of turn."
newly covered	8ac76fc8a9df6aaf9819d55f0434db7f38d9791c18de663f5281f7a93afdd994	card "Aggressive Sabotage"	selected_analysis "Kicker {R}\nTarget player discards two cards. If this spell was kicked, it deals 3 damage to that player."
newly covered	8b537340d6b270bdc2467860467694487721f6766a2413020994d8b5f5c657e0	card "Revive the Fallen"	selected_analysis "Return target creature card from a graveyard to its owner's hand. Clash with an opponent. If you win, return Revive the Fallen to its owner's hand."
newly covered	8b9b5925e5004499f6dec7bdbc5c438280bdc529cfa6c4741d4cd5bb90073e00	card "Boggart Loggers"	selected_analysis "Forestwalk\n{2}{B}, Sacrifice this creature: Destroy target Treefolk or Forest."
newly covered	8bdba3c49544d2490b863594ade365c2fb666c0c5492e94758a176163ccc912d	card "Stall for Time"	selected_analysis "Kicker {1}{U}\nTap up to two target creatures. If this spell was kicked, put a stun counter on each of those creatures.\nDraw a card."
newly covered	8c06ecff18ebe25a154a9d55d5431a5bcfa5e14c18f7adc537ee09d611fc917d	card "Anaconda"	selected_analysis "Swampwalk"
newly covered	8cb869c90b3ccd15315b07eff3e2d0107cbbdc5d7886ddf8d121f33922d1a12e	card "Springjack Knight"	selected_analysis "Whenever this creature attacks, clash with an opponent. If you win, target creature gains double strike until end of turn."
newly covered	8e57568696726031cb4f113fbca8b36c233b7cc7e11e490d5e7c167b3b501831	card "Street Wraith"	selected_analysis "Swampwalk\nCycling—Pay 2 life."
newly covered	8eb9b45593f7f61a553026761e452a3c043cd0630452e6de271ad2022a0af61f	card "Pollen Lullaby"	selected_analysis "Prevent all combat damage that would be dealt this turn. Clash with an opponent. If you win, creatures that player controls don't untap during the player's next untap step."
newly covered	8f09200b8dcedf24b3068a94d8f70d22d2a2844d998f52f6d697673bf48cf503	card "Wojek Embermage"	selected_analysis "Radiance — {T}: This creature deals 1 damage to target creature and each other creature that shares a color with it."
newly covered	8f84a5f49b1515cf8c826ecfd3755163b544f3b1186ebda41326d79524ecf033	card "Rona's Vortex"	selected_analysis "Kicker {2}{B}\nReturn target creature or planeswalker you don't control to its owner's hand. If this spell was kicked, put that permanent on the bottom of its owner's library instead."
newly covered	901ac917e88051ab407e964979607436d5f58e7b36dcb9244f3f7a60ba89de99	card "Orim's Chant"	selected_analysis "Kicker {W}\nTarget player can't cast spells this turn. If this spell was kicked, creatures can't attack this turn."
newly covered	905cd271cf25df87be231158d4ed365e1ed47e13cc012d11306aa90f99c83245	card "Roost of Drakes"	selected_analysis "Kicker {2}{U}\nWhen this enchantment enters, if it was kicked, create a 2/2 blue Drake creature token with flying.\nWhenever you cast a kicked spell, create a 2/2 blue Drake creature token with flying."
newly covered	909665fa2ead303bd463f6a71136bcd5811939fd44404ade75e213e7caec1b03	card "Blistergrub"	selected_analysis "Swampwalk\nWhen this creature dies, each opponent loses 2 life."
newly covered	90ab594eea0818ac1c76a9055372cca08c0e42fd039a5c10376ae0642d47baf6	card "Bog Badger"	selected_analysis "Kicker {B}\nWhen this creature enters, if it was kicked, creatures you control gain menace until end of turn."
newly covered	90d22701107a5879a5683d122dcbc27e0475db3f623a85c370c1e7bd18589607	card "Benthic Behemoth"	selected_analysis "Islandwalk"
newly covered	91e6d09df457c8307756d3e7e63a0a4baef329fcaad31b816c6bfc77fbad575f	card "Tourach, Dread Cantor"	selected_analysis "Kicker {B}{B}\nProtection from white\nWhenever an opponent discards a card, put a +1/+1 counter on Tourach.\nWhen Tourach enters, if it was kicked, target opponent discards two cards at random."
newly covered	9234c9e5e845d9a1f6bccb1e96d930c01b3e1bacb56f63e262d07f54c37fdcb7	card "Sheoldred, Whispering One"	selected_analysis "Swampwalk\nAt the beginning of your upkeep, return target creature card from your graveyard to the battlefield.\nAt the beginning of each opponent's upkeep, that player sacrifices a creature of their choice."
newly covered	93cbfaff64bfe0f3af249201609e834a2550cc69e929cec84a4745a1dae57d7e	card "Krosan Constrictor"	selected_analysis "Swampwalk\n{T}: Target black creature gets -2/-0 until end of turn."
newly covered	94b66db91641482b05280864cc76d7eed90ece41fa0545f7b9c498ff5c5759d2	card "Ertai's Trickery"	selected_analysis "Counter target spell if it was kicked."
newly covered	96579ca1eb032affa93b7a43544c66f6e875e4e57629c540c310aa5af1a952a9	card "Role Reversal"	selected_analysis "Exchange control of two target permanents that share a permanent type."
newly covered	96690d5d79abfe6be5e70265a3a6d5ea7dad3df1486be955f70a3b3b388556ae	card "Merfolk Raiders"	selected_analysis "Islandwalk\nPhasing"
newly covered	9701d97215ee3c25e591a5b457bf1f8daaf24cf949ddeeb722a5a508eb864406	card "Secret Tunnel"	selected_analysis "This land can't be blocked.\n{T}: Add {C}.\n{4}, {T}: Two target creatures you control that share a creature type can't be blocked this turn."
newly covered	979e7a234637a32478f5219e21c7ebaf2d6cf5f5005cb653de13324b8fe421aa	card "Release the Ants"	selected_analysis "Release the Ants deals 1 damage to any target. Clash with an opponent. If you win, return Release the Ants to its owner's hand."
newly covered	97c17b828fa481542b0324737162448312705ee9fd216b8decef65ab91fe3329	card "Josu Vess, Lich Knight"	selected_analysis "Kicker {5}{B}\nMenace\nWhen Josu Vess enters, if it was kicked, create eight 2/2 black Zombie Knight creature tokens with menace."
newly covered	97eee41286979d5793bc9211180951aede2ce1a0b98b458cf40fc1e04be05c83	card "Koth's Courier"	selected_analysis "Forestwalk"
newly covered	980e56f370093fd2dceb311b58b896fca01df97a70b7569335bcd2eaa8d7db48	card "Aysen Highway"	selected_analysis "White creatures have plainswalk."
newly covered	98d2c7c71340104be696700ee0536fc655483886be8b40ec195c71253a4f64d1	card "Agonizing Demise"	selected_analysis "Kicker {1}{R}\nDestroy target nonblack creature. It can't be regenerated. If this spell was kicked, Agonizing Demise deals damage equal to that creature's power to the creature's controller."
newly covered	9a1a2280e9c889f86e05ec338301da1e7b95a4481f1c369c3a09b6e7d8e010dd	card "Fishliver Oil"	selected_analysis "Enchant creature\nEnchanted creature has islandwalk."
newly covered	9ae2fd52f232d0195ef37aba9bd1a22ee8704cda10d098279619fe0275b7e5a3	card "Tempest Owl"	selected_analysis "Kicker {4}{U}\nFlying\nWhen this creature enters, if it was kicked, tap up to three target permanents."
newly covered	9ca6820709ea0ce0ae5472d03bfc3e37d8649a312e98338bf4fccd70944af29d	card "Lost Soul"	selected_analysis "Swampwalk"
newly covered	9d18fff1fffe520f82853f0c3e989da465c588ad07e363ed5e826ab9917558ac	card "Sun-Blessed Healer"	selected_analysis "Kicker {1}{W}\nLifelink\nWhen this creature enters, if it was kicked, return target nonland permanent card with mana value 2 or less from your graveyard to the battlefield."
newly covered	9dddd7babdb0a3d7155f86d25824329216dae01332fbe39a4cf08e25fc2db2f8	card "Guard Dogs"	selected_analysis "{2}{W}, {T}: Choose a permanent you control. Prevent all combat damage target creature would deal this turn if it shares a color with that permanent."
newly covered	9df7bb0ab55d4b81f9ce6c2d43054ef7ccad8101d02f527db77648dd1eeeade5	card "Mega Flare"	selected_analysis "Kicker {3}{R}{R}\nIf this spell was kicked, create a 6/6 red Dragon creature token with flying.\nFor each opponent, choose up to one target creature that player controls. Mega Flare deals damage equal to the greatest power among creatures you control to each of the chosen creatures."
newly covered	9e5ddaff16d2b8883ab4415fa772067936e28a718a953836c3c5133112c48943	card "Slinking Serpent"	selected_analysis "Forestwalk"
newly covered	9f0e5166153eb0ac3086648881833ab1cc3f157868e2776e1cb7c97eb90ef85a	card "Reins of the Vinesteed"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2.\nWhen enchanted creature dies, you may return this card from your graveyard to the battlefield attached to a creature that shares a creature type with that creature."
newly covered	9f57f57c929e2c78ac61ec73fd20fe3c98851aa423a85c847e3a4014677782f4	card "Lullmage's Familiar"	selected_analysis "{T}: Add {G} or {U}.\nWhenever you cast a kicked spell, you gain 2 life."
newly covered	9fa96d368cd9f3c3b61cceac545143d4f6b288221f40b670cccecc484eaf59df	card "Zodiac Pig"	selected_analysis "Swampwalk"
newly covered	a110e345644436e2bd077a1bf8d8f4a43403ce04a45dd1b50ebbdb06d2e2abf3	card "Cloudstone Curio"	selected_analysis "Whenever a nonartifact permanent you control enters, you may return another permanent you control that shares a permanent type with it to its owner's hand."
newly covered	a1ee743dc24a3613f459ae50dfbe7619fb6db4ec43e8c94ba5d2db90c9facad1	card "Erhnam Djinn"	selected_analysis "At the beginning of your upkeep, target non-Wall creature an opponent controls gains forestwalk until your next upkeep."
newly covered	a2f1e568b646bdb2b89c4a419ab12d326226e8c567bf9e2fe71aef7c93a8461d	card "Streambed Aquitects"	selected_analysis "{T}: Target Merfolk creature gets +1/+1 and gains islandwalk until end of turn.\n{T}: Target land becomes an Island until end of turn."
newly covered	a6acc1b12b00ca84693b00baadf1efff5b1fc954810d451e39cc47a4ddde564c	card "Battlewing Mystic"	selected_analysis "Kicker {R}\nFlying\nWhen this creature enters, if it was kicked, discard your hand, then draw two cards."
newly covered	a6b6a2749e1adcc647915ec76c017ed56cdc8640e99a2f35329da9cd2355aef1	card "Volcanic Strength"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2 and has mountainwalk."
newly covered	a6bfdf7ba5148d7eadffd5fbffe6a0612ce1cdc9e408283a7d0d955d034c0af5	card "Unseen Walker"	selected_analysis "Forestwalk\n{1}{G}{G}: Target creature gains forestwalk until end of turn."
newly covered	a70093383ed309877a7e43e11df19added356a7a6be226b1c29c629f8a019867	card "River Merfolk"	selected_analysis "{U}: This creature gains mountainwalk until end of turn."
newly covered	a722fca5a8a933052866cfec5c71fdf294c0e31e1d2c6dcf5bfc5724c0ecf91a	card "Desolation Giant"	selected_analysis "Kicker {W}{W}\nWhen this creature enters, destroy all other creatures you control. If it was kicked, destroy all other creatures instead."
newly covered	a75030b5b14694ad7220d3739464621ae539298fc4cb93ffc2d0042cba62e333	card "Wormwood Dryad"	selected_analysis "{G}: This creature gains forestwalk until end of turn and deals 1 damage to you.\n{B}: This creature gains swampwalk until end of turn and deals 1 damage to you."
newly covered	a78251f82a76eb011108ef52374ec9534b3b0c56ca774b1790d52beedee62b6d	card "Wormwood Treefolk"	selected_analysis "{G}{G}: This creature gains forestwalk until end of turn and deals 2 damage to you.\n{B}{B}: This creature gains swampwalk until end of turn and deals 2 damage to you."
newly covered	a7af914c41da9f1c6db44b494b48ba9bd77110a05982ada0673ef14c3cfbffe6	card "Cateran Slaver"	selected_analysis "Swampwalk\n{5}, {T}: Search your library for a Mercenary permanent card with mana value 5 or less, put it onto the battlefield, then shuffle."
newly covered	a7cea01f932587b69de7083c35158068d3c41237fec1d75c90926fc14b41fabc	card "Chasm Skulker"	selected_analysis "Whenever you draw a card, put a +1/+1 counter on this creature.\nWhen this creature dies, create X 1/1 blue Squid creature tokens with islandwalk, where X is the number of +1/+1 counters on this creature."
newly covered	a8401e916d73f21fa4003ff5057280192e5aa9a7b7cc76d2ba1bec93af7a64a5	card "Balduvian Atrocity"	selected_analysis "Kicker {R}\nMenace\nWhen this creature enters, if it was kicked, return target creature card with mana value 3 or less from your graveyard to the battlefield. It gains haste. Sacrifice it at the beginning of the next end step."
newly covered	a844915a9feca96bdca0f38ddd9f17d246b92196785886522c213521caa9cfde	card "Goblin Mountaineer"	selected_analysis "Mountainwalk"
newly covered	a8aa9bb6b7ed6db9ea7b3a930e4d5e63b715f645d6fa655fb031d857335f129c	card "Urborg Repossession"	selected_analysis "Kicker {1}{G}\nReturn target creature card from your graveyard to your hand. You gain 2 life. If this spell was kicked, return another target permanent card from your graveyard to your hand."
newly covered	a99c29a42097cc0ff53dc43284b297b4c224f49af0a0b86f4bb92b56998000d4	card "Mountain Goat"	selected_analysis "Mountainwalk"
newly covered	a9dc208cfb98955369ffd0c63dbfeaa0cf04583d423eb7d1d1df05170971892d	card "Rock Badger"	selected_analysis "Mountainwalk"
newly covered	a9ff21637b6d95cc94cd78cc62f066510865a0b372054ce1bffda1586a21ab0c	card "Witch Engine"	selected_analysis "Swampwalk\n{T}: Add {B}{B}{B}{B}. Target opponent gains control of this creature."
newly covered	aa0d990831e3e512fc296dd052fad0ba5a2ba478a837b2f189ff8a69222b3bca	card "Oaken Brawler"	selected_analysis "When this creature enters, clash with an opponent. If you win, put a +1/+1 counter on this creature."
newly covered	ab28fb25e94bc324ad3ac2c01a88b3215ea3b2b56a624b378d590d51b1c8b7e6	card "Wild Onslaught"	selected_analysis "Kicker {4}\nPut a +1/+1 counter on each creature you control. If this spell was kicked, put two +1/+1 counters on each creature you control instead."
newly covered	aba38453efad2ea0f9cf51886de5342026a021b44f38bf926a75b3c5e31e98e9	card "In Thrall to the Pit"	selected_analysis "Kicker {2}{B}\nGain control of target creature until end of turn. Untap that creature. It gains haste until end of turn. If this spell was kicked, sacrifice that creature at the beginning of the next end step."
newly covered	ac75c8d4ec59df90eb6717ceb6f548ea7974e18e7437fa88c36ae37438535b6d	card "Phyrexian Missionary"	selected_analysis "Kicker {1}{B}\nLifelink\nWhen this creature enters, if it was kicked, return target creature card from your graveyard to your hand."
newly covered	ad093d6562b9dc2b5bb88e06aac10dc9c9bbfff7c967b7daee1e25972565dc2a	card "Burrowing"	selected_analysis "Enchant creature\nEnchanted creature has mountainwalk."
newly covered	ad9eea969bb24153cac72f9b7fb8f4b167c30fe476edc631d49e0feb5d60dbd0	card "Thought Prison"	selected_analysis "Imprint — When this artifact enters, you may have target player reveal their hand. If you do, choose a nonland card from it and exile that card.\nWhenever a player casts a spell that shares a color or mana value with the exiled card, this artifact deals 2 damage to that player."
newly covered	adcdcd4baa516fb6f3d699732ed84d4bdaec857308b3bc8032af9da6f237aea4	card "Surge of Zeal"	selected_analysis "Radiance — Target creature and each other creature that shares a color with it gain haste until end of turn."
newly covered	ae522060b949565b3fcdca600fbc4ec8675d6bc96d1b5bf51ebd917dfec2cbec	card "Cragplate Baloth"	selected_analysis "Kicker {2}{G}\nThis spell can't be countered.\nHexproof, haste\nIf this creature was kicked, it enters with four +1/+1 counters on it."
newly covered	ae7ab07dec8ccf350a975cf9f4d2457ff01b45261d15ab41425a260826a831fd	card "Dwarven Grunt"	selected_analysis "Mountainwalk"
newly covered	aee4e72d6109f2f4ea8956f81613c5e2d97022b8386317c397708246d471545f	card "Canopy Surge"	selected_analysis "Kicker {2}\nCanopy Surge deals 1 damage to each creature with flying and each player. If this spell was kicked, it deals 4 damage to each creature with flying and each player instead."
newly covered	af309e529792a0b20abfa07bdb0fc7a7f1432374aeda0fccfc3ea6613d050231	card "Phyrexian Scuta"	selected_analysis "Kicker—Pay 3 life.\nIf this creature was kicked, it enters with two +1/+1 counters on it."
newly covered	af7a51789ab90068fb14bd9caabbbcb8297b2f45c0b9b859263bb646bcef3dcd	card "Jukai Messenger"	selected_analysis "Forestwalk"
newly covered	af7d766adb96630e62b0a11a08fbaa1133cfaec9041f9d5da60c1b49e1023864	card "Dauntless Unity"	selected_analysis "Kicker {1}{W}\nCreatures you control get +1/+1 until end of turn. If this spell was kicked, those creatures get +2/+1 until end of turn instead."
newly covered	afb8fc3a50955e6f1919635974d1287baea5e4917b13fc1b7c15d157e1998794	card "Torch Slinger"	selected_analysis "Kicker {1}{R}\nWhen this creature enters, if it was kicked, it deals 2 damage to target creature."
newly covered	b03ccc80a7dde2af2274ed064904b368dc1aa0ee65333481811a1ac926e2d02d	card "Zendikar Farguide"	selected_analysis "Forestwalk"
newly covered	b09456b45aa9ac689b5ee2e7946158d65c0d7419f5a111c7ac061d708e921848	card "Roil Eruption"	selected_analysis "Kicker {5}\nRoil Eruption deals 3 damage to any target. If this spell was kicked, it deals 5 damage instead."
newly covered	b09e2775cd4fa9ba8167a2ad612d1d3b6ff96bdcdcff211a2f3493e042cf842f	card "Funeral Charm"	selected_analysis "Choose one —\n• Target player discards a card.\n• Target creature gets +2/-1 until end of turn.\n• Target creature gains swampwalk until end of turn."
newly covered	b0cba4d02496f55c30c2f9260870ab46f63978c81592f7f65810413b520c2d21	card "Jedit Ojanen of Efrava"	selected_analysis "Forestwalk\nWhenever Jedit Ojanen attacks or blocks, create a 2/2 green Cat Warrior creature token with forestwalk."
newly covered	b1bcf1074ad78824ffa9c66807c6dc180771fe7e0ff14ced56849b4796fa831a	card "Zodiac Rooster"	selected_analysis "Plainswalk"
newly covered	b2511dbfe43e8222c80e66b913ba71ffa8fd04cafd46be21faf3831417d6fbf4	card "Heartstabber Mosquito"	selected_analysis "Kicker {2}{B}\nFlying\nWhen this creature enters, if it was kicked, destroy target creature."
newly covered	b36c1fefa4a7099bb2237905ca30b6a3e083028a8812398ec368dfff5be97fec	card "Phyrexian Espionage"	selected_analysis "Kicker {1}{B}\nDraw two cards. If this spell was kicked, each opponent discards a card."
newly covered	b421999c66bb597016a3950e7d4a27dc3844c5e238a49fe74fc8b1221b1ea6af	card "Crown of Fury"	selected_analysis "Enchant creature\nEnchanted creature gets +1/+0 and has first strike.\nSacrifice this Aura: Enchanted creature and other creatures that share a creature type with it get +1/+0 and gain first strike until end of turn."
newly covered	b4df2c3eabcde53afd0f924676261547e45561d10fa4e33044a1bc844f081d59	card "Strength of the Coalition"	selected_analysis "Kicker {2}{W}\nTarget creature you control gets +2/+2 until end of turn. If this spell was kicked, put a +1/+1 counter on each creature you control."
newly covered	b5546f59e79b223751f7afc25d58a206758563901af9178f29967fe1ac61d027	card "Cleansing Beam"	selected_analysis "Radiance — Cleansing Beam deals 2 damage to target creature and each other creature that shares a color with it."
newly covered	b5f8ada83e13b4e15f659d48341cbc3e94342a0994458eb51f9a5055b8b78c41	card "Aether Figment"	selected_analysis "Kicker {3}\nIf this creature was kicked, it enters with two +1/+1 counters on it.\nThis creature can't be blocked."
newly covered	b6be9523ea13a4e9a14d7bf810d5976c66458dc20fc7d7ca745bb05d48360655	card "Blink of an Eye"	selected_analysis "Kicker {1}{U}\nReturn target nonland permanent to its owner's hand. If this spell was kicked, draw a card."
newly covered	b6f0d4dc447bceea34f255eb904780cf7c1d0c4e113239ba25986b99e87b8f08	card "Merfolk Falconer"	selected_analysis "Flying\nWhenever you cast a kicked spell, scry 2."
newly covered	b74cb5ce46361ca3d78e424bb36bb435e3b2758767df7c4c3aa03b9084281ed0	card "Adder-Staff Boggart"	selected_analysis "When this creature enters, clash with an opponent. If you win, put a +1/+1 counter on this creature."
newly covered	b7c2cf05a921ed272521a5fb19c1b99ff51c5a7736c6e6c796047fd3b185e8c7	card "Lurking Crocodile"	selected_analysis "Bloodthirst 1\nIslandwalk"
newly covered	b7f4bf41e8031cd380a7e375d701d5aed49b5738b04174927a78d4210889b101	card "Tolarian Emissary"	selected_analysis "Kicker {1}{W}\nFlying\nWhen this creature enters, if it was kicked, destroy target enchantment."
newly covered	b8e91a4bb07a954cad94cb4a59e998267b56a5fd6263d322ac81c8748542cd01	card "Shivan Emissary"	selected_analysis "Kicker {1}{B}\nWhen this creature enters, if it was kicked, destroy target nonblack creature. It can't be regenerated."
newly covered	baf961e022a1efcd888070971b19363216e210d07910973d013554c53b7deae3	card "Amareth, the Lustrous"	selected_analysis "Flying\nWhenever another permanent you control enters, look at the top card of your library. If it shares a card type with that permanent, you may reveal that card and put it into your hand."
newly covered	bed621499880dd6a98a3d1b7ab4f8ec2ddb56d75c00280d92a9ca4fa721a1bc9	card "Goblin Spelunkers"	selected_analysis "Mountainwalk"
newly covered	bef7623960585f4c6a6899dd0581cb0277fdd627eb52a3029c3c1f4e4dc91818	card "Counterlash"	selected_analysis "Counter target spell. You may cast a spell that shares a card type with it from your hand without paying its mana cost."
newly covered	c0102616123eb466226419c366a36ffed1297da2972d10c4060dc4b025dead6c	card "Dirtwater Wraith"	selected_analysis "Swampwalk\n{B}: This creature gets +1/+0 until end of turn."
newly covered	c0514ab9467355a49e0b588d08c8503250dfc1eb5802435acfca15f44e541486	card "Paperfin Rascal"	selected_analysis "When this creature enters, clash with an opponent. If you win, put a +1/+1 counter on this creature."
newly covered	c0f295f61b3987b1a5742e5b9c4da4d076b39076010ca1f4be25c7f986228879	card "Zodiac Ox"	selected_analysis "Swampwalk"
newly covered	c15c29a32c27274b3574d70c0db79d3228ba02e88490686516cd29d090b24476	card "Conqueror's Pledge"	selected_analysis "Kicker {6}\nCreate six 1/1 white Kor Soldier creature tokens. If this spell was kicked, create twelve of those tokens instead."
newly covered	c18c7baf089e565fad89c612aa4245645b4905ba52c59c738b7f45392ef5fe64	card "Bilbo, Luckwearer // Burglar's Plot (Burglar's Plot)"	selected_analysis "Exchange control of two target nonland permanents that share a card type."
newly covered	c1bf5b6dfb2a86aee6ef63899fb56df7c370d4a93c6de08e3d4fab44fc023173	card "Raise the Draugr"	selected_analysis "Choose one —\n• Return target creature card from your graveyard to your hand.\n• Return two target creature cards that share a creature type from your graveyard to your hand."
newly covered	c1c463d46d55f3cfb77fb994ad2a701285ceff2db347416079f6542ff8962636	card "Cemetery Gatekeeper"	selected_analysis "First strike\nWhen this creature enters, exile a card from a graveyard.\nWhenever a player plays a land or casts a spell, if it shares a card type with the exiled card, this creature deals 2 damage to that player."
newly covered	c293e4d74be892aacdac93089632ce47debcad544adc337585e7674567cd6893	card "Shalai's Acolyte"	selected_analysis "Kicker {1}{G}\nFlying\nIf this creature was kicked, it enters with two +1/+1 counters on it."
newly covered	c2d0662dcc6b13b4b3b5b021de4ed75e3260297dea01b14ce668b48b8e252a12	card "Magma Burst"	selected_analysis "Kicker—Sacrifice two lands.\nMagma Burst deals 3 damage to any target. If this spell was kicked, it deals 3 damage to another target."
newly covered	c30ddd3aa5a956becb325e4f756337b1f480cabc3de771a54c910f383c1be109	card "Bloodstone Goblin"	selected_analysis "Whenever you cast a spell, if that spell was kicked, this creature gets +1/+1 and gains menace until end of turn."
newly covered	c48a2f8e83646ccef3fd2de7776b73db222b58902e66b6b3ebc3d9211586a22d	card "Vicious Offering"	selected_analysis "Kicker—Sacrifice a creature.\nTarget creature gets -2/-2 until end of turn. If this spell was kicked, that creature gets -5/-5 until end of turn instead."
newly covered	c4c03c14b41d0fe57c6f8e6aaf80a36e30433dbe801353cab3838e08adfef1cc	card "Alpha Status"	selected_analysis "Enchant creature\nEnchanted creature gets +2/+2 for each other creature on the battlefield that shares a creature type with it."
newly covered	c71654c233a5eeaa10bd9c52009e94b7700aac47802c5028b9a1f18bda86563c	card "Gatekeeper of Malakir"	selected_analysis "Kicker {B}\nWhen this creature enters, if it was kicked, target player sacrifices a creature of their choice."
newly covered	c77075fd929652f97d091acbdfa124e376c0dce4a1ae3447e78319ff6df5ba2d	card "Grayscaled Gharial"	selected_analysis "Islandwalk"
newly covered	c81c37ebdfb3f47090d662c27c1e44d67baf7f8e0c8e4a84b9ed30719b840454	card "River Boa"	selected_analysis "Islandwalk\n{G}: Regenerate this creature."
newly covered	c906ee577bcd2e1ae0313f81b4724a02a99c198ae21e0895e93b96c3d95e765a	card "Holistic Wisdom"	selected_analysis "{2}, Exile a card from your hand: Return target card from your graveyard to your hand if it shares a card type with the card exiled this way."
newly covered	c97c1408d60599507b26db4a4a8611adcde397eb1d0cd6f354776b21d75af75a	card "Unstable Footing"	selected_analysis "Kicker {3}{R}\nDamage can't be prevented this turn. If this spell was kicked, it deals 5 damage to target player or planeswalker."
newly covered	cb0a03f3073d60727f856559f3dca17a757ac9a284e10788248f81bec5faf251	card "Dismantling Blow"	selected_analysis "Kicker {2}{U}\nDestroy target artifact or enchantment. If this spell was kicked, draw two cards."
newly covered	cb27543c21e6ef16abf677ba5a9a121608d8f556d35bd22bec58da2f539b58e6	card "Fervent Paincaster"	selected_analysis "{T}: This creature deals 1 damage to target player or planeswalker.\n{T}, Exert this creature: It deals 1 damage to target creature."
newly covered	cb9ca23f32c5d421578e319463534f28ebe107cb0907b998d823a27c6c200256	card "Mudbutton Clanger"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, this creature gets +1/+1 until end of turn."
newly covered	cba3db0aa437438e9709560799c5954820751e082efaec05b5a2bd80ea754c1f	card "Final Flourish"	selected_analysis "Kicker—Sacrifice an artifact or creature.\nTarget creature gets -2/-2 until end of turn. If this spell was kicked, that creature gets -6/-6 until end of turn instead."
newly covered	cc23ea2a41c434677b1320379a0879b3df15a867ae30abc7a86be51a8b3361b1	card "Citanul Woodreaders"	selected_analysis "Kicker {2}{G}\nWhen this creature enters, if it was kicked, draw two cards."
newly covered	cc56b098d9a73878e5f4ae1f83aa8da5f02b710d5d5dd67c921337b787d06618	card "Stomped by the Foot"	selected_analysis "Kicker—Sacrifice an artifact or creature.\nTarget creature gets -2/-2 until end of turn. If this spell was kicked, that creature gets -5/-5 until end of turn instead."
newly covered	cd59ed675d70b62d083c2a0d89b969af8bc6e510fdbc79a05bd8a82dc352ba04	card "Mountain Yeti"	selected_analysis "Mountainwalk\nProtection from white"
newly covered	cd74762e2a02595239e0bc24dc0c3f75f07b9236f043ce539598d936ae492b12	card "Crown of Suspicion"	selected_analysis "Enchant creature\nEnchanted creature gets +2/-1.\nSacrifice this Aura: Enchanted creature and other creatures that share a creature type with it get +2/-1 until end of turn."
newly covered	cdfe2f41c2949b080b66c82322f444c4beb4f05efbeb7e9aeda918dc92bac90f	card "Bubble Snare"	selected_analysis "Kicker {2}{U}\nEnchant creature\nWhen this Aura enters, if it was kicked, tap enchanted creature.\nEnchanted creature doesn't untap during its controller's untap step."
newly covered	ce351b9f674f357f2d81b7a04926d5e6d775eae79de0e70a9fae1ae58f69f38f	card "Sokenzan Bruiser"	selected_analysis "Mountainwalk"
newly covered	cfbd57507d5ab820a1b9e5a1b240eec6b73a7909892ab9ca470f06d38af3209e	card "Martyr's Bond"	selected_analysis "Whenever this enchantment or another nonland permanent you control is put into a graveyard from the battlefield, each opponent sacrifices a permanent of their choice that shares a card type with it."
newly covered	d15d8c24bee694f9de777c776f988cc7261e2b668a414d8f45e152aa9fcf17a8	card "Bog Wraith"	selected_analysis "Swampwalk"
newly covered	d18e9506b995f080c51bf4aaff5478f8a4063af404bb177399b883ede40da941	card "Somberwald Dryad"	selected_analysis "Forestwalk"
newly covered	d1fb927bb5239d3ed82bd9dc9a07e559b28f8b2d9c049c620e5071bb980482be	card "Dwarven Pony"	selected_analysis "{1}{R}, {T}: Target Dwarf creature gains mountainwalk until end of turn."
newly covered	d29aaf6b41b9bf3c81d57d2be77e3cc67cc65d492faa8a09aa190fe7931445ae	card "The Trickster-God's Heist"	selected_analysis "I — You may exchange control of two target creatures.\nII — You may exchange control of two target nonbasic, noncreature permanents that share a card type.\nIII — Target player loses 3 life and you gain 3 life."
newly covered	d301a839e7f32d4bd85c31aa2ea485028afdd3e0c56ef1a722a2c902c4e2215b	card "Mistform Warchief"	selected_analysis "Creature spells you cast that share a creature type with this creature cost {1} less to cast.\n{T}: This creature becomes the creature type of your choice until end of turn."
newly covered	d32348915d9532f9f1b6f7c5e399f8c67129335b34b2665f8258f6254e19fa76	card "Gigantiform"	selected_analysis "Kicker {4}\nEnchant creature\nEnchanted creature has base power and toughness 8/8 and has trample.\nWhen this Aura enters, if it was kicked, you may search your library for a card named Gigantiform, put it onto the battlefield, then shuffle."
newly covered	d34070c7fd76fd8676f9edd91e02ee05efbff3c7a7890ab94e69597cca7b74c7	card "Part Water"	selected_analysis "X target creatures gain islandwalk until end of turn."
newly covered	d3dd4665ab4bf9e1a237ef3470b39a5368ca93adedda627c2e9b9d7b059c8d88	card "Crown of Ascension"	selected_analysis "Enchant creature\nEnchanted creature has flying.\nSacrifice this Aura: Enchanted creature and other creatures that share a creature type with it gain flying until end of turn."
newly covered	d4164c443a828e68f1c15bbe50355f47d3530d23f467e722606b210315e1b538	card "Benalish Emissary"	selected_analysis "Kicker {1}{G}\nWhen this creature enters, if it was kicked, destroy target land."
newly covered	d46f1015fb97232cea3d0eb9b025f869a56b6401e15a63be712c573ff13b74d6	card "Choking Miasma"	selected_analysis "Kicker {G}\nIf this spell was kicked, put a +1/+1 counter on a creature you control.\nAll creatures get -2/-2 until end of turn."
newly covered	d54f9b40448f93fbbf6795abc78cb14ba0a4a50bd76785c16c0a6365635edb3e	card "Nullpriest of Oblivion"	selected_analysis "Kicker {3}{B}\nLifelink\nMenace\nWhen this creature enters, if it was kicked, return target creature card from your graveyard to the battlefield."
newly covered	d56807c5ae08a6c2276b41611bfdeb348825b0a3a76d03f6f63ae79a862c3dde	card "Fatal Grudge"	selected_analysis "As an additional cost to cast this spell, sacrifice a nonland permanent.\nEach opponent chooses a permanent they control that shares a card type with the sacrificed permanent and sacrifices it.\nDraw a card."
newly covered	d5a464777830a09e9544507dbaeec5652552716be1a84020d097b1f72cd0c4bc	card "Master of the Pearl Trident"	selected_analysis "Other Merfolk creatures you control get +1/+1 and have islandwalk."
newly covered	d5fcbc7d4ff4f7a61d7a48bae3ae68791777374ca9facf7b563a370cfc024ab8	card "Bog Down"	selected_analysis "Kicker—Sacrifice two lands.\nTarget player discards two cards. If this spell was kicked, that player discards three cards instead."
newly covered	d6588e485d136ccb38c262998e6cb8d0452aaaa3813ba562838836e428d5a9c0	card "Incite Hysteria"	selected_analysis "Radiance — Until end of turn, target creature and each other creature that shares a color with it gain \"This creature can't block.\""
newly covered	d66d331d9bdae27fe11d543a9d2eada883030c02933860c6f85e99114d8c37a8	card "Vines of Vastwood"	selected_analysis "Kicker {G}\nTarget creature can't be the target of spells or abilities your opponents control this turn. If this spell was kicked, that creature gets +4/+4 until end of turn."
newly covered	d6bd4ab11ab77ad049d15e07bf632e6ea751375583937f0e2d27b600c87d57f9	card "Jaded Response"	selected_analysis "Counter target spell if it shares a color with a creature you control."
newly covered	d726c9c083e4edb8402ca136bef967d87dc200c2185ecfea4704aa9b24cfe29d	card "Faces of the Past"	selected_analysis "Whenever a creature dies, tap all untapped creatures that share a creature type with it or untap all tapped creatures that share a creature type with it."
newly covered	d773342806f38490b9566627fcd481af0d5ef8d8c4f1ba687fbf99d24a47c800	card "Winnower Patrol"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, put a +1/+1 counter on this creature."
newly covered	d89407b62622c0514be9894a8fc8373ab605e12204261cb16640cf25d91b3149	card "Saproling Migration"	selected_analysis "Kicker {4}\nCreate two 1/1 green Saproling creature tokens. If this spell was kicked, create four of those tokens instead."
newly covered	da8671c5adbba8568cb14007b9c7fc6de2af4c62884706b4b7360a730251cdc1	card "Squeaking Pie Grubfellows"	selected_analysis "Kinship — At the beginning of your upkeep, you may look at the top card of your library. If it shares a creature type with this creature, you may reveal it. If you do, each opponent discards a card."
newly covered	dd1963f762b163c7e17212cf110d77fed2242202853b438db955a60939dc5320	card "Into the Roil"	selected_analysis "Kicker {1}{U}\nReturn target nonland permanent to its owner's hand. If this spell was kicked, draw a card."
newly covered	dda56947dc8a7d09ed5852b004820a7d6f01b2f8ba8277a85062e503ab50a710	card "Bog Tatters"	selected_analysis "Swampwalk"
newly covered	ddef287d668490ef3877e0107d56c2c0c8d846ab57211178341374852abdab4a	card "Elite Cat Warrior"	selected_analysis "Forestwalk"
newly covered	de5364d80e7e024ea2600162af8f3f5905e72df33723b3d967fc88e3da76208e	card "Untamed Kavu"	selected_analysis "Kicker {3}\nVigilance, trample\nIf this creature was kicked, it enters with three +1/+1 counters on it."
newly covered	debcc4aedc80ff5ef904ec648adb382137a0cee2d38ffc122541835b99496667	card "Inscription of Ruin"	selected_analysis "Kicker {2}{B}{B}\nChoose one. If this spell was kicked, choose any number instead.\n• Target opponent discards two cards.\n• Return target creature card with mana value 2 or less from your graveyard to the battlefield.\n• Destroy target creature with mana value 3 or less."
newly covered	e0156fd9dae2fa072c2cafc5e17974cd4f38d6790cf90219cf5c624a2dc70cd7	card "Marsh Boa"	selected_analysis "Swampwalk"
newly covered	e10d710d2c32b66ff9db1c8d9ec2bca25a8236c20f3e45e972b4326591ea42e5	card "Viscid Lemures"	selected_analysis "{0}: This creature gets -1/-0 and gains swampwalk until end of turn."
newly covered	e14ce5ff95cc8003cb0530df793e0e3ea3c79eebe57a007ee8d89092957eb7c1	card "Righteous Avengers"	selected_analysis "Plainswalk"
newly covered	e29a316fd94c5834e0bf40d11c9d217c81461c9b8bee21b774de903e5aa28f2b	card "Baloth Gorger"	selected_analysis "Kicker {4}\nIf this creature was kicked, it enters with three +1/+1 counters on it."
newly covered	e2edae9dc3af5e7c9eab186df9fe75e5ed0e64d666bcb25744ca45a1b5f0778b	card "Arctic Merfolk"	selected_analysis "Kicker—Return a creature you control to its owner's hand.\nIf this creature was kicked, it enters with a +1/+1 counter on it."
newly covered	e30c3f8fac880f3bb0de303db57a0ff4b3adf452afd239674ce4778c74822d3b	card "Sprouting Goblin"	selected_analysis "Kicker {G}\nWhen this creature enters, if it was kicked, search your library for a land card with a basic land type, reveal it, put it into your hand, then shuffle.\n{R}, {T}, Sacrifice a land: Draw a card."
newly covered	e653038446f217f15f18e6b9562944c4541a8fa84bdc90982f786ac9133c0326	card "Rushing River"	selected_analysis "Kicker—Sacrifice a land.\nReturn target nonland permanent to its owner's hand. If this spell was kicked, return another target nonland permanent to its owner's hand."
newly covered	e745acd77463dd2bf5cfdfc1efe1f2866ff37223f9a661a84d94fd411026ead2	card "Cavern Crawler"	selected_analysis "Mountainwalk\n{R}: This creature gets +1/-1 until end of turn."
newly covered	e7704ae473f54f2d28027dce49e135cdd6dca25ca689cb8afe9b3a21445be824	card "Woodlot Crawler"	selected_analysis "Forestwalk\nProtection from green"
newly covered	e78a64d566b341f48f31293c2aec06a838556af5e64fa9b2a44acae306fa2aec	card "Stronghold Confessor"	selected_analysis "Kicker {3}\nMenace\nIf this creature was kicked, it enters with two +1/+1 counters on it."
newly covered	e7af6338fbc464c082cd7a5397fa72eefa3b83c0378e60f3737ad874a8a1ccca	card "Merfolk Assassin"	selected_analysis "{T}: Destroy target creature with islandwalk."
newly covered	e7cf56760dc98a18c99fc8950b028c05847138a762947559fe9bd37e96a3dcfa	card "Kavu Aggressor"	selected_analysis "Kicker {4}\nThis creature can't block.\nIf this creature was kicked, it enters with a +1/+1 counter on it."
newly covered	e9c280802aec03df798797871710a79af980c5de85113a361298ba2af016d61f	card "Shore Snapper"	selected_analysis "{U}: This creature gains islandwalk until end of turn."
newly covered	eb6e9fba18f6cff6d809251e6fd4ec8759c098cb49572fe52e53b788a4efb7d5	card "Zodiac Dog"	selected_analysis "Mountainwalk"
newly covered	ec55c9f9b01366902712935617bef6df2ea31a957c73fd9c8991926b57023924	card "Titan's Revenge"	selected_analysis "Titan's Revenge deals X damage to any target. Clash with an opponent. If you win, return Titan's Revenge to its owner's hand."
newly covered	ec88ed8a3ba6017871db6d7ec3e975796aa4be1eedb6399fa3de0629948c4a8c	card "Ringskipper"	selected_analysis "Flying\nWhen this creature dies, clash with an opponent. If you win, return this card to its owner's hand."
newly covered	ef5bb43a0d03161d1ffc1ecf6fd1bf6b4daed37f790118514991e0f7b0af689f	card "Vastwood Surge"	selected_analysis "Kicker {4}\nSearch your library for up to two basic land cards, put them onto the battlefield tapped, then shuffle. If this spell was kicked, put two +1/+1 counters on each creature you control."
newly covered	ef6e401f92f2bfefc44e3127607f9ede39f211f114eaf0aabb0b09074f96d553	card "Zombie Trailblazer"	selected_analysis "Tap an untapped Zombie you control: Target land becomes a Swamp until end of turn.\nTap an untapped Zombie you control: Target creature gains swampwalk until end of turn."
newly covered	eff83e30bdf4c4aca1e8e63b52433a6479157a8438c3b0ccee039341a8a824ab	card "Common Cause"	selected_analysis "Nonartifact creatures get +2/+2 as long as they all share a color."
newly covered	f097c18eda4ca891bb8fbbc6dc10ac078c830fb5973ce174942bbb70a8c9e686	card "Restless Bones"	selected_analysis "{3}{B}, {T}: Target creature gains swampwalk until end of turn.\n{1}{B}: Regenerate this creature."
newly covered	f1364a29f56130b863387075cdba941507483c8dd447a88dbd78b93be4c7fcc7	card "Goblin Ruinblaster"	selected_analysis "Kicker {R}\nHaste\nWhen this creature enters, if it was kicked, destroy target nonbasic land."
newly covered	f28aa00225e32719fcdba29ae0b9fb5dafcea01806ce1837b158f9fa8b7d3e19	card "Ghitu Amplifier"	selected_analysis "Kicker {2}{U}\nWhen this creature enters, if it was kicked, return target creature an opponent controls to its owner's hand.\nWhenever you cast an instant or sorcery spell, this creature gets +2/+0 until end of turn."
newly covered	f419498a6f7cc02caab81adc5cc32e8974cf095fa692ed170500782e6ee11a12	card "Canyon Wildcat"	selected_analysis "Mountainwalk"
newly covered	f5125b2843f77a315859ebe51605f68996955e0a703a27276b3d79b366d2e975	card "Odylic Wraith"	selected_analysis "Swampwalk\nWhenever this creature deals damage to a player, that player discards a card."
newly covered	f53cf3908ff86761948bbd19f5fc0c560a1416503d74bbab6eff76adf6f1d26c	card "Boggart Arsonists"	selected_analysis "Plainswalk\n{2}{R}, Sacrifice this creature: Destroy target Scarecrow or Plains."
newly covered	f58e04fdb232d9180f223c29f093a6b0b9c95656281ac233cca20bf37ee44072	card "Rite of Replication"	selected_analysis "Kicker {5}\nCreate a token that's a copy of target creature. If this spell was kicked, create five of those tokens instead."
newly covered	f5c5d19a9e61eaaf0936f0c68e83cb87e40b7c3e2610535295211597307a4321	card "Warthog"	selected_analysis "Swampwalk"
newly covered	f6dcd6a4e28ace0b5ebb212fdff211c6ee64a1f85f757e89a744d8c074baa3b5	card "Shanodin Dryads"	selected_analysis "Forestwalk"
newly covered	f819f13371a4f5eeda14d95b4fd91d0a8309c6c20498db5ff4bc243f7d6e24cc	card "Vug Lizard"	selected_analysis "Mountainwalk\nEcho {1}{R}{R}"
newly covered	f8a9dc1ed94ede3da5af9e40c4cd3b36138a19d2ac143f5a5eb0ddefb851231b	card "Endemic Plague"	selected_analysis "As an additional cost to cast this spell, sacrifice a creature.\nDestroy all creatures that share a creature type with the sacrificed creature. They can't be regenerated."
newly covered	f8b0f8decfa9e1c82042469b78d30756e9f321dee5bd1a8b8a3ec3247f0d5135	card "Skyclave Relic"	selected_analysis "Kicker {3}\nIndestructible\nWhen this artifact enters, if it was kicked, create two tapped tokens that are copies of this artifact.\n{T}: Add one mana of any color."
newly covered	f8fef26fc4b6d8179a093c5ae39b7cf2eb17bd43266d3c5aa3768463c084cd56	card "Zodiac Tiger"	selected_analysis "Forestwalk"
newly covered	f94256c31e36fcf7a69d7c77775a3ad0b6929f485171f41af4e555bd73c33e69	card "Cat Warriors"	selected_analysis "Forestwalk"
newly covered	fad5c99756068ac707694b06defbb2de0326f3ee7b4dc3c462d39f4d7782e487	card "Protect the Negotiators"	selected_analysis "Kicker {W}\nIf this spell was kicked, create a 1/1 white Soldier creature token.\nCounter target spell unless its controller pays {1} for each creature you control."
newly covered	fafbec44e716afbe40ab4986ed3c7cf52feb09b916a8fdc7f5131f9bf78416a4	card "Rushwood Dryad"	selected_analysis "Forestwalk"
newly covered	fb157b5535d0768876ac9dec6f5e0ced2b93bc866ac262963c96233e870f5714	card "Willow Dryad"	selected_analysis "Forestwalk"
newly covered	fb70cffd79c9f1490d1ff82889285af3ca03ec3a2f496f234eccfebd506a352b	card "Pixie Illusionist"	selected_analysis "Kicker {3}{G}\nFlying\nIf this creature was kicked, it enters with two +1/+1 counters on it.\n{T}: Target land you control becomes the basic land type of your choice until end of turn."
newly covered	fc3c8101a6e618222c2885799ae425c5929cf2cf63025bcae5c7db9455ef4e5f	card "Myriad Landscape"	selected_analysis "This land enters tapped.\n{T}: Add {C}.\n{2}, {T}, Sacrifice this land: Search your library for up to two basic land cards that share a land type, put them onto the battlefield tapped, then shuffle."
newly covered	fcae7b76828c88e5f00b044a09bf00488e8a19bee467cb8ff0f02790abc70453	card "Mirri, Cat Warrior"	selected_analysis "First strike, forestwalk, vigilance"
newly covered	fd907b9e4c1e3cb1d013d734f8818c2544c50d5e8686ddcff1aa7de342d0ee0c	card "Pincer Spider"	selected_analysis "Kicker {3}\nReach\nIf this creature was kicked, it enters with a +1/+1 counter on it."
newly covered	ff4daf7dc2784053f7b2e4626f8a3679d5b1d4bf3a1506196832451531396f3e	card "River Bear"	selected_analysis "Islandwalk"
```

</details>

### REPORT

- Wall clock: 2026-09-05 06:26 PDT to 2026-09-05 07:44 PDT.
- Coverage lock: 19,564 -> 19,939, +375 / -0, on change `qsssoovy`.
  Construction declarations: 394 -> 394.
- Licensed vocabulary/lexeme homographs, reported rather than fitted:
  `AttributiveAdjective::Untap` beside the keyword-action verb declaration
  `Untap`; `TargetingMarker::Target` beside noun lexeme
  `CommonNoun::Target`.
- Form-literal/vocabulary overlaps, reported rather than fitted: `additional`
  at `additional_cost` atom 2; `to` at
  `up_to_quantifying_determiner` atom 1; `the` at
  `definite_next_mass_quantity_reference` atom 0; `next` at that form atom
  1; `to` at `scalar_less_than_or_equal_to` atom 4; `the` at
  `number_of_scalar_value` atom 0; `the` at `greatest_scalar_value` atom
  0; `other` at `other_than_qualified_reference` atom 1; `the` at
  `positional_partitive` atom 0.
- Performance advisory, workers 8 throughout: coverage `--check` 142,293 ms,
  164,304 ns/B, host load 22 / 23 / 18; ambiguity 114,200 ms, 127,438 ns/B,
  host load 15 / 20 / 19; roundtrip 146,463 ms, 183,469 ns/B, host load
  28 / 25 / 21. The 16,260 ms quiet-host ceiling was exceeded under concurrent
  load and is reported, not fitted to. Concurrent-process count is unavailable
  from the sandbox; the reviewer stamps contention.
- Assurance counts: restored 11 payload paths; re-spelled 4 existing
  assertions; ignored 0; added 0; removed 0.
- Citation changes: none; no cite gate required. Glossary gaps: none. Decisions
  wanted: none.
