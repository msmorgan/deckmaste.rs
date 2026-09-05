---
needs: [english-v2-scope-device-collapse]
---
**A frame-declared preposition has no Predicate-Adjunct derivation in that
frame's clause.** `english-v2-role-preemption-depth` (B6) eliminated only the
noun-Postmodifier derivation of a preposition the head verb's frame declares;
the Predicate-Adjunct derivation survives, so `Search your library for a card.`
still carries two candidates — the declared object/`for`/object frame and a
transitive Predicate plus a generic Prepositional Predicate Adjunct — and the
frame wins only by specificity (decisive position 2). Principle (ii) of the
scope-device design (right-periphery frame-role preemption) is meant to reach
this competitor too.

Pinned scope: widen B6's preemption so that, in a clause built by a Verb
Frame declaring role P with marker preposition p, a right-peripheral PP headed
by p has neither a Postmodifier nor a Predicate-Adjunct derivation. The
mechanism is B6's existing frame-role accessor and preemption site, read from
declared frame data — never a guard naming a preposition, verb, construction,
or card. A verb without a `for` role keeps its adjunct `for` (witness:
`Draw a card for each Island you control.`). Expect specificity-resolved →
unique movement on the search/for family; DISCLOSE every changed selection.

Authority: design doc `b7-scope-device-design.md` §C.4a (2026-09-05), which
routes this widening here rather than into phase 3 of the collapse. Runs after
`english-v2-scope-device-collapse` lands so the census diff is attributable.

## Landing record

### PROVE

- Measured tree: change `nqtnzsxz` (the reviewed tip, feature commit `okupnymz` plus one review correction), lock `covered = 20,254`; matched parent: claim fork `upyluyyr`, lock `covered = 20,254`. Every figure below was re-measured on this refreshed pair; the implementer's original figures were stamped on a `20,054`-covered base that trunk has since left, and are superseded.
- Coverage had no loss and no gain: `20,254 → 20,254`, lock delta `+0/-0`. In report mode the tool printed no delta row and left `english-v2-coverage.lock` untouched in the tree. Newly covered identities: none. Identities no longer covered: none.
- Structural laws hold on the measured tree: `selected_uncovered_units=0`, `unresolved_ties=0`, `internal_failures=0`, `roundtrip_mismatch_units=0`, `ownership_failure_units=0`, `traversal_failure_units=0`, `leaf_traversal_failure_units=0`, `gap_spans=0`, `overlap_spans=0`, `synthetic_claims=0`, `provenance_plan_mismatches=0`. The standalone roundtrip gate reports `20,254` parse-accepted, `20,254` clean, `0` mismatched.
- No word-naming guard was added. The one new checker, `predicate_adjunct_is_prepositional_and_not_declared_role`, reads the right-peripheral Prepositional Phrase's marker and the Clause head verb's declared role markers through the generated `role_prepositions()` accessor and the `visit_verb_frame_role_preposition` visitor callback; it names no lexeme, preposition, verb, construction or card. `coverage` classifies it `structural_predicate` and reports `licensing_checker_permitted=24`, `licensing_checker_forbidden=0`. Declaration loading and the reverse-dependency tests report no `environment.rs` load error.
- Acceptance witnesses (probed on the measured tree): `Search your library for a card.` resolves `unique` through `ObjectForObjectLexicalVerbPhraseDeclaredObjectForObjectLexicalVerbPhrase`, with no Predicate-Adjunct construction on the selected path. `Draw a card for each Island you control.` resolves `unique` and keeps `PredicateAdjunctPredicatePrepositionalPredicateAdjunctPredicate`, because Draw declares no `for` role.
- Negative witnesses for markers other verbs declare as roles, all keeping their Predicate-Adjunct derivation: `Sacrifice it at the beginning of the next end step.` (`at`, declared by Look), `You may play an additional land on each of your turns.` (`on`, declared by Put and Enter), `Search your library for a card, then draw a card for each Island you control.` (the second Clause's free `for` survives beside the first Clause's declared frame).
- Whole-corpus selection-neutrality proof, parent `upyluyyr` against tree `nqtnzsxz` under identical `--json --require-resolved --workers 8` flags, all 32,641 units compared: `0` units changed accepted/failed status, `0` units changed selected construction path, `450` units changed resolution and every one of them `specificity → unique`. In all 540 units whose candidate set shrank, every removed candidate's construction path contains a `PredicateAdjunct` construction and no candidate was added anywhere. No newly unique reading is wrong: each of the 450 selects the frame reading it already selected, with the Prepositional Phrase filling the declared role.
- The corpus contains no verb whose declared role marker also heads a genuine adjunct elsewhere in the same Clause: no unit lost parse acceptance and no unit changed its selected path, which is the direct evidence. (`Smuggler Captain` is the apparent repeated-marker case; its second Prepositional Phrase is inside the object's embedded relative Clause.)

Gate artifacts, all foreground on the measured tree after `kata refresh`:

```text
cargo xtask gate --changed
cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask
test result: ok. 424 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 44 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 158 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 116 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 470 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
cargo clippy -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_english_v2 -p xtask --all-targets -- -D warnings
Finished dev profile; strict clippy emitted no warning
```

```text
DECKMASTE_COVERAGE_LOCK=report cargo xtask english_v2 coverage --check --workers 8
summary {"total_units":32641,"selected_units":20254,"covered_units":20254,"selected_uncovered_units":0,"parse_failures":12387,"unresolved_ties":0,"internal_failures":0,"roundtrip_mismatch_units":0,"ownership_failure_units":0,"licensed_vocab_lexicon_homographs":2,"form_literal_vocab_overlaps":9,"licensing_checker_permitted":24,"licensing_checker_forbidden":0,"traversal_failure_units":0,"leaf_traversal_failure_units":0,"gap_spans":0,"overlap_spans":0,"synthetic_claims":0,"provenance_plan_mismatches":0} lock_mode=report
cargo xtask english_v2 ambiguity --json --require-resolved --workers 8
tree    `nqtnzsxz`: total=32641 selected=20254 unique=17198 specificity_resolved=3056 unresolved_ties=0 parse_failures=12387 internal_failures=0
parent  `upyluyyr`: total=32641 selected=20254 unique=16748 specificity_resolved=3506 unresolved_ties=0 parse_failures=12387 internal_failures=0
cargo xtask english_v2 roundtrip --require-clean --workers 8
roundtrip: parse accepted=20254 clean=20254 mismatched=0 not parse accepted=12387
cargo xtask catalogs check
catalogs are up to date
cargo xtask cite check --list-noncompliant
0 non-compliant citation-looking string(s)
cargo xtask cite check
checked 14488 citations against cr.txt (eff. 2026-08-07); 0 stale
```

### DISCLOSE

- Selection census (`--require-resolved --workers 8`, matched flags, both sides measured in this workspace): `upyluyyr` has `16,748` unique and `3,506` specificity-resolved; `nqtnzsxz` has `17,198` unique and `3,056` specificity-resolved. Exactly `450` units moved specificity-resolved → unique. Another `90` specificity-resolved units lost invalid candidates but retained unrelated ambiguity; their surviving decisive positions are unrelated construction pairs (trigger prefix against ability body, nominal coordination against genitive determiner, pro-verb against base verb phrase, and similar), never a frame-against-adjunct pair. The specificity share fell, so no construction pair needs naming on that account.
- The specificity → unique movement is `448` Search/`for` units and `2` coordinated Put/`on` units. The mechanism is generic and declaration-driven; trunk produced no Attach/`to` or Shuffle/`into` movement for this diff, so no inventory pin was fitted to manufacture one.
- Assurance accounting: restored/widened `1` (the compiled-consumer `run_role_preemption` assurance, widened with a visitor probe of the generated accessor and re-based on a two-frame synthetic declaration; every pre-existing assertion in it is unchanged); re-spelled `4` (the compile-fail private-field `.stderr`, the `declaration-verb-expansion` golden pin, the `Search your library for a card.` selection assurance in `parser/materialize.rs`, and the licensing-checker census pin in `xtask`); ignored `0`; added `1` (`declared_frame_prepositions_preempt_only_matching_predicate_adjuncts`); removed `0`. The re-spelled Search assurance keeps the same card and still asserts a successful selection; only the recorded resolution moves `Specificity → Unique`, which is exactly the outcome the design doc requires of this widening.
- Deviations and additions:
  - B6's declared accessor `head.verb_frame_role_prepositions` reads the *codec's* static frame tail, which is empty for the bare transitive codec that builds the competing derivation, so it cannot answer this question. The widening therefore adds a per-declaration accessor beside it: a generated `verb_inventory_role_prepositions` helper, a `role_prepositions` field and `role_prepositions()` accessor on every generated declaration-verb terminal, and a `visit_verb_frame_role_preposition` callback on the generated `Visitor`. This is the same frame data read one layer lower; no new declaration syntax and no new construction.
  - The preemption is keyed to the **declared frame set of the Clause's head verb** — every frame that verb declares — not to the single frame that built the Clause. Keying it to the building frame would preempt nothing, because the competing derivation is by construction the one that does *not* use the declaring frame; the design doc's own acceptance (`Search your library for a card.` must resolve `unique`) is only satisfiable on the head-verb reading, and its self-limiting clause is stated as "a verb declaring no such role is untouched". It is not the "some frame anywhere declares p" reading: a verb that declares no such role keeps its adjunct, as the negative witnesses above show.
  - The Clause's head verb is the **first** declaration verb reached walking the adjunct host. Verb Phrase and Predicate coordination sit above these constructions, so in the corpus each host holds one verb; where a host could hold more, the rule under-preempts rather than over-preempts. `CostComparisonPredicate` and `AlternativePredicate` hosts contribute no frame at all.
  - `existential_predicate_adjunct` keeps the plain `predicate_adjunct_is_prepositional` check: a there-Clause has no Verb Frame to read. That is why the permitted licensing-checker total rises `23 → 24` rather than staying flat.
  - `predicate_grammar.rs` gains a synthetic `Island` subtype declaration, present only so the required Draw negative witness can be spelled.
  - The `kata refresh` onto the landed preterite work conflicted in the generated declaration-verb terminal, its golden pin and the private-field compile-fail fixture. Resolved by keeping trunk's `Box<VerbInventoryRef>` and `inflectional_forms` (and its manual `Debug`, which is why the terminal's debug rendering is unchanged by this landing) and adding `role_prepositions` beside them in both constructors; the golden pin and the `.stderr` were regenerated from the merged emit.
- STOPs: none. Newly covered negative or wrong analyses: none. Wrong newly unique analyses: none. Unexplained losses: none.
- Glossary gaps: none.
- Citations: unchanged; the cite gates were run anyway and are clean.

Changed selections (all 450 identities):

#### `ObjectForObjectLexicalVerbPhraseDeclaredObjectForObjectLexicalVerbPhrase` vs `PredicateAdjunctPredicatePrepositionalPredicateAdjunctPredicate` → `TransitiveLexicalVerbPhraseTransitivePredicate` + `PredicateAdjunctPrepositionalPredicateAdjunct` (448 units)

- `06e34267add0f5af6916ea8aefdb9c1f7b9ae7d07fb4293be3e070395d0b235f` — Academy Rector
- `60ba87443c469bbc7213bc695478cc1cc91e86f7baba7d93f26b5cd166fc73b7` — Acquire
- `83ccbef11e4dcd85d5f17181abce99b8c0a46802dfcd74f20f30cf35177ef8b6` — Ajani's Aid
- `af0ec4a83714e5ba243268782e252a17e821ac39710ee5b5539837eb4722c5f2` — Alpine Houndmaster
- `0912aa6f89637c383de27b9a780eb015c4691804117a8280c9bfcb6cc8f1db58` — Altar of Bone
- `f8bbde92d42c5047ec581aa77be5d4349edbb644372e615584e1ac6d0c7c4f06` — Amrou Scout
- `9e6a27e4fc34694e37da6b292c65d31df2babd2520b279342ddf4465f4f2880b` — Angel's Herald
- `54e2d49e4859b0a3ebd364239a296f13c23ad25b20e937d03cfb1004bc976851` — Angrath's Fury
- `c5abc86714ff8fe63013bb80f5db9e23710a2b2e1305d5d888cdce4d4e8295f1` — Arachnus Spinner
- `9df788f0eed7787c1782c317f869ec78f40ec0b9b10d6fa63e2fdb007acaff06` — Arcum Dagsson
- `fc3c8c26273674da3b9d558eef4efc1c7bb5b793c01ca1005af6c0ab541d5a82` — Arena Rector
- `70a14e39ce7ea7bdde0487970e274e30164741bd81734411b55114924cb73fa3` — Armillary Sphere
- `5300313a228efca278fe69ed223c629facaa7048806bbe4c5f227290e4f700cd` — Artificer's Intuition
- `8c5208115a67729b771f3ee9245d77f0ea66cc767103955a0f427e78cad13d56` — Ashiok's Forerunner
- `79ce1ff1372bae473d08c7f7b8ca553cce5dd709e74863379c1d83e614a86e84` — Assassin's Trophy
- `abf7042fdaac003226a78f4a7d4323f667fe60cb36907ced75a0f7bdd2dd4de8` — Atalan Jackal
- `1621bbb2d64ec0fc2cb29e681255795abc968f738d9d1d49a2a96c0ea509d198` — Auditore Ambush
- `5aa551409382a05bf7b0662f62653754468dbf3a82c1825ab1ae2f8f1ab0eb78` — Aurochs Herd
- `e381c60f9a327b532088724c18fac73e34b056fdbe6f107d0d6ef0fee4ef817a` — Avarax
- `3aca964a5450fd453f052c35b3b16130aa0005c4a73e8fb358cabcebdab26156` — Avengers Disassembled
- `0a795c6fc3ca7a0466dac020e5ddb41196c530c487e4ed284b95a1f24d93d24b` — Axgard Armory
- `9629493f85583bfc74def2e399995cb7c0afdc0132159db4053c24d02bbdbdcc` — Bant Panorama
- `9ba2f7370fb74d9d502f3d68a37bc38fcb7fd8046f78fcf7b6ad26715ce155a3` — Basri's Aegis
- `ac00a1a00522b6159f98a5264e9f169fa1c5d568703205544b553a8f5fbea4dc` — Beanstalk Giant // Fertile Footsteps
- `447c3ad08e460a9f4f90e94fae1a5cf3c9bc0440dee010e608f719bb78421d77` — Behemoth's Herald
- `b27d6479266ee22408b6ba8968fe6be95ee13798c6e67c7267a5a1fa180b9f25` — Behold the Beyond
- `5af8d082c149f2bd1509f24256fe578bce366ce51649ff755bb6683ad6e86817` — Beneath the Sands
- `65501c4eaeeac1739523bee3fb48904e2c5f47f8300565b9f92ae9d63a8c8d80` — Binding the Old Gods
- `abb77a2b5c98aab6099154330c279fc4952db0eb90d245f6e1fa00740409cb5e` — Bitter Ordeal
- `eda3022087dbe5d66179c86e7fdfe116c60e321e6fafb6ccdb9c94596154769f` — Bitterheart Witch
- `c759dadbe9fc67d3e2def6dc1d9beb5224fb80bd79374fcfd5125346842bb945` — Bitterthorn, Nissa's Animus
- `5a41538aec8f7be9712590e04d1b05c9a29cbd599d0203cc93bf9198e3f2494f` — Blighted Woodland
- `a7b9cb81e42d1b4a3d115885de7b9a4b9d4e3349504157c149d3bfff8b26bfda` — Blightspeaker
- `55c9208032912d5df27a05a5c3692a6bacc7f9a8258edb01c2494430444e0e1b` — Bog Glider
- `e6abb930ff695698ffdfacff6d7ad76cbec6b760a0e629311b0c6ac33c15f73d` — Boonweaver Giant
- `c331b3df318244243da55ae93cef951208d934b4b78ded988960fd3ba282458a` — Borderland Ranger
- `4ce0e13fde8af3f2cf302b986ffa3fd0bb107e39f694884ebfc0e5c2473db4a5` — Boseiju Reaches Skyward // Branch of Boseiju
- `8ffda2c7fd4b0c1dcee7f9fd0ce5105db2ff2c61f12a1556c28009dfd69a73e3` — Boseiju, Who Endures
- `c6b74d419eb7bbc12422912cd13b158b303aaf1e602f550c1c6ddc76b91faba8` — Bountiful Landscape
- `84c9151391423c3397ca2b7abde2656516665b4b501f62749a61ccc96cbb4e18` — Braidwood Sextant
- `58350bba5746beb580a0bde03ac6364ee15cf7b9195bad98439f29ab91edbb64` — Bribery
- `95385cd2767e10b8d50ac3ef33dde52b67668e5a642711d52a0bee1c7a159ed9` — Buried Alive
- `dff212af6197f78850f42184845569edfe60a2035dfcfe2952816eab88016e3a` — Burnished Hart
- `277cd39b529b82c30ea92d489197579d2d5583e919989b86c35211f4cb269cac` — Bushwhack
- `7d3ab41a65cc1f66e0e5b3ea952cefbd1873ad3ebc7e7f341f6a7b31c5bfe56a` — Call the Gatewatch
- `c0476f797d983c6a3d385e1534b2d4e7bfd9b2a45d563399720cb3682cf0917e` — Call the Mountain Chocobo
- `c288cdebf51159235e97a7f93e68b5eaba48ed771160e347db87b9f5a61ee137` — Captain America, Liberator
- `9d25e2d738d86a8dc99f579532e0dd00a242819db3738547ed0eed1450db0e2d` — Captain Sisay
- `e64867240e03a74e76a39701e552cb05b05bdffdbdbe389e1e09954e1bac9cdf` — Casal, Lurkwood Pathfinder // Casal, Pathbreaker Owlbear
- `c1f86c8c32329362ca43ecccf287acc3080460e36c1e0f37599f2f1c272e7b77` — Cateran Brute
- `b39c0ca25acf035f68578ca8bd6abef9ae5de5f5ea8327fb1ed1a0851ea659ce` — Cateran Enforcer
- `e528210fc6b742d09986bee137b1e61dfea32f9246720c2eaa77373a6959f393` — Cateran Kidnappers
- `0f3bb629ef8778a4da2c84321f96027fc8699aa779993408b6201800fcdb2164` — Cateran Overlord
- `8e8b52d551466f7758ffc0f0217d5f61182892537004bb154881b31f338f97d5` — Cateran Persuader
- `a7af914c41da9f1c6db44b494b48ba9bd77110a05982ada0673ef14c3cfbffe6` — Cateran Slaver
- `0b93e5e7447b7c489396c0b0b23a1ddad4d25a1190ff402abeed4790d17f94ec` — Cateran Summons
- `3296034e37bd0a2fd447d28817c746180c50204a522d28fab24224cb08be0a59` — Centaur Rootcaster
- `7908b8fe581d6d4ed1c49c11d3e9aed69eedf9589577255e6e075e3cbdc61ae5` — Chandra's Firemaw
- `3f8aa022c37beef9d674c918289cb74d458fcac8dddcbce5af54707554aecf08` — Chandra's Flame Wave
- `fe6b2f91ca2db593e2224bb0c6dd2935a9f8e83019ea5382a193468eeef2c474` — Chandra's Outburst
- `51d848a3f3924dedf6c9361c32acd7cc15286e618ad8ccd6fa8af9d5fcec06ca` — Changeling Wayfinder
- `09dd85fb37c70b88015e8964d5648bc4d805b5fb6f4b99453fdd4fc9180090a6` — Chord of Calling
- `d70510f33a58d9312e02cd233c8f7e2329b840babc54c73a4f1368cfa6fe9ab0` — Citanul Flute
- `38e53960c64b9f72402f9cfb70c70688ea1bc4204ff4111a116d97952ef454ab` — Civic Wayfinder
- `787a7bb4dd210c2587ff8b2b7a635070f4e03ca91d571f4a8dcb91129b207ede` — Cleansing Wildfire
- `a9721005ffb0a91ed9439d2ed8aa31321ea212d5a30309b1a68e29db4a7503c5` — Colossal Rattlewurm
- `a819b4dcb375ecb685badf3b7c850d09e9db7f8e11510ab235cacc85e4253fd1` — Conflux
- `27b773a50c4c428b5e5d2d4a347dde1fc1fcf0a8eb21910cae710fbb8acb1917` — Contaminated Landscape
- `60a65623da9db87bd0982a62846331d2aa1e7be9105a5ca92b9c4313dc3241e8` — Corpse Connoisseur
- `8b21b051cfe167cd2673dec626d48dd2e223d1788a7c0e00a298724ce1b2d4cd` — Corpse Harvester
- `96e9c1410c0bafab7b6124090f2327f5f43046b6e8da1328a704e1d05232634a` — Courier of Comestibles
- `736bdc0f1640a42eaff5b43c0f1956a1571671f4706a8ba9937bad44efb04d6a` — Crop Rotation
- `dac3cd4abca677546fc01dd44944fdf766f8bf8ab93a61100bb31ec25c9efc24` — Cycle of Renewal
- `b04b7ec64c900f145965d8d0f64d12bff7d03cee949e402baf9f4f81f11f5b45` — Dark Petition
- `1eb135d0dc0578598a4f2563729af62ebdf1f99e91ad96c2591491306df3c55c` — Dark Supplicant
- `ddaa1f492cbe669342071436cbf24d67e59eab573a81f56c1cda97495ccc8f05` — Daru Cavalier
- `741a21c42d776afea0e84b2fdfeed037825e7ed69b711ab219568aeec63dcf4a` — Dawntreader Elk
- `c45f6b2d7dbff22f0c37532c126a76fe3831f890f285161a0fecd2ee8eca2fe0` — Deathsprout
- `d51dcc2493281c8536cf00784a88361486ec21926856631d647b9fc61b978fa1` — Deceptive Landscape
- `7ab6141e7ac009060f8033342f49c18ed478fca36abaf6d7df809693f3424b6e` — Deep Reconnaissance
- `71d24ef3558f4d37b2f189b84ca235fa514a9e3414db587cc0530019e63efba2` — Defiant Falcon
- `3a84f5f44a0c4f965d7dda1c40730a9a137d364bffce2fe6aaad87128a83db82` — Delivery Moogle
- `7ef3fa7d8e05846103d1203ea88459f1962c915dbc60e88c5bd1ffb4cb6c9698` — Demolition Field
- `60476dcf255756052e5a96763c0d5487f62daaedfa7b0aa47edb4fd0a904b817` — Demon's Herald
- `25496d639acd18fe8cdd49bbb73e5abfbf2684be50a8f9be09925d47ac40b3c4` — Demonic Bargain
- `7ad54199af10dd29a076a9936d810564690828af4e27f37dfe5e43b3b1e4b031` — Demonic Collusion
- `7f2f1714198c9dded9204455da2122c8c018c3dd1e8079e6191307c8c1813b00` — Demonic Tutor
- `0b73218e8b09f45ba14a998ac7fc679c3faa3fef2f97c02e04383691083d69c9` — Denying Wind
- `f18f227c0deda0dfa4dd2ebc214851e8e6c8fcc6f150eb932f77f18bb133291c` — Diabolic Intent
- `9d337a94c69065467bf4eb2e525461544bfaa9cb01c9ee537541c27cfca58cbc` — Diabolic Tutor
- `ccd05401be058da6437ff9886e670eda76a3cb8b360030ccec90bd481196cf76` — Dina's Guidance
- `cdcd9f869e789ce716f9e40f1b1049bc7ff104441a6f0cd5dca0e38eeb5030d6` — Disciples of Gix
- `885241601d7452d966b4ac472beaa857175dc3f144dc4b5e83ad494b333da5d6` — Distant Memories
- `5d803f3348ec0319271ea4b58c35f02289b5e7d0158504c56b785d3db61dcb0d` — Domri's Nodorog
- `39b1bee4ea16496faf630d45325966961219d38c0476ff6e6bea03b5d1f59c48` — Dovin's Dismissal
- `8e62c3dcc24fbad4a4bdfa4c7debe2b361124d78bcb790d1c1f96228a19fd0d8` — Down in the Valley
- `284ca69813ed88a215feddfeb332c325e41aca70dbb0743dc1d6d90db75a35ee` — Draconic Muralists
- `733f5d85721a2854b74044db0a51d936b094ad80f1ab9ca0afa728b36704e655` — Dragon's Herald
- `1c2a04cf3aaad8ebf504fc786224b2c7babcfff053bd9590bba056e012a8b745` — Dragonstorm
- `d5accf515e47b8c38bf775875f1f08687e7626a78e5b57b026ee8fa0f7c8053b` — Dreamscape Artist
- `f3880d74c66e74a576bf67b40b0f2376cf263f5725a59e24e3fccd5b64ea8c4e` — Edge of Autumn
- `c22ed77711793a3f3bce8b76b1228b888772c8c8f823ab8b714e08ce2323978b` — Eerie Procession
- `20454a87dc5571fe4f63ca34059386c915056e4cdbcb04896026935c7e5a52a1` — Eladamri's Call
- `cae1cc612fb7692548b3336993f40577cb1842638ab39c8d54ea782b4e7fe7ff` — Elspeth's Devotee
- `81a91fe62dd8879d3355a549d149c2c4ddcc9ca9eba6283c0b1c06d090c8be81` — Elvish Clancaller
- `4eca97cd0f56e4028738fc1f2c12627ffea38b7486aa78a748f9b912f2c96051` — Elvish Reclaimer
- `553cb2212935d8cbc5740d80d01302d716b1e059e5da6a240e5ef20f6e40f81f` — Embermage Goblin
- `c9113547942244c2689528aff3aa0d4b01529b01cb7cf40c16a28816ac524602` — Embodiment of Spring
- `ecd594bb73b21c92a5d6777001bfb2f707ef30d6a27dfc5c7a7944cf90609645` — Emeritus of Woe // Demonic Tutor
- `53b1703bcb3f5362c52a050f2cac5213e97442504fc2cdd7ed4940d641ef2047` — Encroaching Dragonstorm
- `bc40d15cf708233a740688670d5d93e0a3aa9c1af19c385ca00e939f14530ec6` — Enduring Ideal
- `f3344f837279035e67bce12d900c8b96b75d932b5af2a115215f5901e01005d3` — Entomb
- `75c15414fe36e94639d8ec8cd0defa72ee0a1c66f0f48aabb7ad840f80729244` — Environmental Sciences
- `4d7448cee2999c40a8bae6e221ecdd9d739a835abf9f9607b28790b264d7839e` — Environmental Scientist
- `0b8118e3c65bebd279257358554923ab513ac0fee19a5b0084fef1d350aff28d` — Erode
- `8850fa0471bce4562bca4f0972206a78970b85beca0e480643be3fd663f73ebf` — Escape Tunnel
- `ad64badf7b0bbd4fb5a2a5049056da44f4b64eab05a246106bc80a245a1a9947` — Esper Panorama
- `dec71b9adbb1b434f3fe9ccaf99fb82ce969190cc1cdb8fcb0e14f2d97a44e5f` — Ethereal Elk
- `44cb21a2a3f4518b8764edbac83cff1a4efb32a8769afb4c9d24eabe009aa115` — Everbark Shaman
- `84a35637d465e1441d6fea88377c034948f578cd11876ea75918e6bc9a6a8ae3` — Everything Pizza
- `8b29f31da5a4e98dbffe5ddec56b7dfa34b7ad21811bb8fbaecca70c0ffa6412` — Evolution Charm
- `f1bf35a3f5cf5fccd93d7606f69a7a1997b731814b533fb9de95802d8dcebf65` — Evolving Wilds
- `75342a6c6b7a636de2ec3fd6d82590839aed65f27069c009e6d49d3e6e66654e` — Expedition Map
- `d2dd071568131057288d05f0c7c252b8b481f8c57dbd0369602ef007d75038e3` — Explosive Vegetation
- `f4fa1ac872efdc2cf83c168983c68110c69c7491ce6becc4ee421e438d0b148e` — Extract
- `6a1d18db92367ed0edb778718445a5640f468c76706d1ce819111e41f568a255` — Eye of Ugin
- `bb53fda8b219f9edbc4b634937a86ea2beed77cd6c97541437f0045af1608c41` — Fabled Passage
- `3c7a523a84d40cf1b1bb105da4ad4d237c7876a8db7c1fe7b6f2c7c004b49505` — Fabricate
- `f1bd3bbb45958e7db4466093824de0386bcb4fc7c54062f001950f86a2f25c8a` — Fang-Druid Summoner
- `9fea8472562ac4ed0cb454b7e67d4ca21f9f532a8a49c905eb1797f1c099878a` — Farfinder
- `841c03bf85bff8071e4dfdf13afca70a095e87eda8ed13182e01bfbd5549f1b8` — Farhaven Elf
- `827ddc49c2ebc8cb12b73869115442b18da55c28e28ae3d9b50ce6b59878720c` — Farseek
- `e21c146e8772cf0e6e07e1489a1afd0860631c3b6b8e31815d2e517eb84d6e6b` — Fauna Shaman
- `210148ff3642ba0c901845dd938621f4494e7646167cda1447283e15f9d1bbd1` — Fierce Empath
- `101d5df0bfc33045a1b3f6c6167f8f47227d0eaf49383ff2d13935516d0827cb` — Flagstones of Trokair
- `01007a819b567329915af640a34e81804711e01e96eb3b19f85c1cf4b1e8def9` — Font of Fertility
- `5d8ec9e3bc0e9cbd8777109b707403fc1310960ba8bbfd7b1541a0ddfc95eb57` — Foreboding Landscape
- `a67a5f444737916855626d2634f3c6671d63986cedfbe2ce6ea80a36e28d7080` — Foresight
- `5310fe593725c059e1ad46af321e0b3ebff040ca9b5098ae10902aa4b464f25c` — Forging the Tyrite Sword
- `ce65dc29ce5a8e8f06153b8c7a58644887afcade158ad2eb3f92c0fbc6b60c06` — Formidable Speaker
- `015700ac5cc458ad632397c09e46f124019e8e8d7fa29afe18488dc9ef9a894e` — Frenzied Tilling
- `06bd44492fe306cc7199e7aef0d53e0d1e1fbaffdb751fa1804c3e295bbda52f` — Frontier Guide
- `9fb0152a754912c4249cfd0bc291253ade9bc449bc3425790c8e66b409005202` — Gaea's Balance
- `5ac548fae270adeb340ebb124c7379935c864440f5a95af15c5a98c6186c59c8` — Gaea's Bounty
- `c6ef81de7294c9fcbdcbad51cf022b1940fb39927d935dfef7763cd6411f9692` — Gamble
- `ccbe77821f35403711b19ac4fe8eaf4ec5fe32ebca5f46d508da08f6ce57d60a` — Garruk Relentless // Garruk, the Veil-Cursed
- `74b301b1711ed3e2d07517eb111e3bf116214ec9ca2aeb72291041123f39eea9` — Garruk's Warsteed
- `08bd4f60b9b319cebf407c9bcca8d62b82d8b5f2fdb497409d69dfedf863cfb3` — Gatecreeper Vine
- `0f6b7ef118c8d569d86bb4c52d58cb12fa6de91d3820c4098b3a14d71d9438a1` — Gem of Becoming
- `beb0a491cba07860381de5fcd3be5df069bf0d183f78e31e7f58128fe634a713` — General Tazri
- `7b13116eba002d607e7fad02c46d7c4340f805701a049d53391cfc0de9e49511` — Geomancer's Gambit
- `b694fb2b9e77ffbe2d5a6af299070a6716f10c050276b2771937395176165145` — Ghost Quarter
- `3644881835ed676ff109f5fb20e9f4ecf270a668b03036eb9fd3eb3a6df6d4f3` — Gideon's Battle Cry
- `44955eeb43554a789e477a913ca2970b8f7475ac49a89a1a99199d04a675cc61` — Gideon's Resolve
- `1d823017f4d2f38645941c0f7811ddbdaa4b7ca641ed44079a6c190d2a588940` — Gift of Estates
- `389359039f51e43c73f0521bc5b2fb62ff50bd0a17ef1c899dad4bc1560ae8ad` — Gladiolus Amicitia
- `5834f78534f9fe91f68c68ec4547a6ad0a83a5a03b2980d4f1def85d01861fc5` — Glimpse the Core
- `e9d77ef8850d7d194d13c9c860a97664317689976249b680e0f59671db50c939` — Go Forth
- `1846dcd95bb12e79d163f0e1ede9e451099d52bc7fcaff94878fec8ce9ad087b` — Goblin Engineer
- `02036687e2774a5a7b99fcfc27e057c3dea494d6d68a1dce161ed7a5b30172e1` — Goblin Matron
- `2b91de3d89f847b0c660a101a71608534f8ab851e458e861c09697fb35436608` — Goldmane Griffin
- `8745a3618953400ffc0f35dd3672768a0b36118b3930c6e5866058efea2cc8c3` — Grasping Current
- `b4c44f6b7913ea22ef4f7a9869622349ccdf344dbfdbf23fb1e1548c1a946146` — Gravebreaker Lamia
- `8042c114d1394e3201004774ce3450f94ab1e54c7787872548cdf00d6d2b363a` — Greater Tanuki
- `c43817db0cabbbd0f9424e83b526a91733d996aad8f7c6d507cb2ed17b55f2a5` — Green Sun's Zenith
- `5097070343e1de8c9b932f2c3cc6ae0ca8c9a2e65d635b79115d67025817da8b` — Greenseeker
- `d9a770721b79ecfa24a661a6fd39817df5bcffc8852d76ce1dd920ca4c01a402` — Grim Tutor
- `f7d2cd35c72e3e0e04a52cb526888cc7bd721777978a7bcc5b2632367adab9dc` — Grixis Panorama
- `b617f1a9ff76d747b8596d4302a6e8b7d5483958b2df965e6c12d6c382cd2937` — Growth Spasm
- `8ef92392b8a4e3d6487dc048bc565af17bdc3cd25e9f68d45f3ae7ee266d5dc4` — Harrow
- `1e2313b2f7ee45c43083730ae953ad9fb99c897568fbf4f0b7e20bb9ef4f711a` — Heliod's Pilgrim
- `c384c252018c84afa971ebab83ba7cfb0b83e708887dbd10b8e6cc24800a5e48` — Herd Migration
- `08e95854b428cbdd675ce0e5169a6864e1f4e7f0565d1bfcde0250288e659ad4` — Higure, the Still Wind
- `d71c1e02167dcb2059b430f5e23dc9ee4809b406879cbe34d2dcaf441a59029c` — Hoarding Dragon
- `85c6801e45a399bfe16a090656383662d638755956093d58c1f2ef9442c349c3` — Honored Knight-Captain
- `42575620e35a704bc3fe5e768bfec23f4c2d18a8d9e3a50f40d231d4970b1295` — Horizon Spellbomb
- `df57e325b83d22a77c1c681a44eef5444f82b0930bea27bc7178d33fd7d853a1` — Hour of Promise
- `86880e2d8308147ce24293746471b3a068651e742f248da2a5e210cdebf57d61` — Hunting Cheetah
- `657910a3806da02a4e2908e4763ca03b20c8ac8d7a28f0d29c2cb2844a07c980` — Idyllic Tutor
- `e7075f49743b4e461c8ffe346637118f60abc2239a9fe53b2a24431a0448eaec` — Ignite the Beacon
- `ecdec34629b2f425c3c4c940f0ac149a69856c939c8676e702a4fb16fc1c1631` — Illicit Shipment
- `22d4679bd9c7738efec4f0a93feadc416ea9e3a1dcf7d9708c0c9575bbc20de0` — Imperial Hellkite
- `1534a64022c7a3112fd346f55a31e4b1bd59063f230f0ff575bb4f0c33652195` — Imperial Recruiter
- `ae81c22e69e006048437c408db91b4903994982c5740d65e8a528f5e0981f1bf` — Inevitable Betrayal
- `5b7e20904d49795618636132d3e958c8d4f6bf6d1edde3e3eb38d207b6813670` — Into the North
- `a0beb8e22e477e36339fd87c1854069ff3183cab21f19e045827789f5fca8a0b` — Invasion of Ikoria // Zilortha, Apex of Ikoria
- `c6de176deefa654337dc3cff8e725719bb84e26e3c5c138231c49736a2194655` — Invasion of Theros // Ephara, Ever-Sheltering
- `3f926d9a547aca40635a75b132ca018c0cda43064c498d14aeaca41d9a680961` — Invasion of Zendikar // Awakened Skyclave
- `bfb527e320fd81787fac0d203629bc18693b4da4664fce514ba67997a1fdd1cc` — Invert // Invent
- `3ba4e573b550fb63e1bb4e31313c53724a6d36e553b577c8eef3ce96dbf9fe8b` — Isperia the Inscrutable
- `9c00a72ca33c983949e9f134a1d56a6a177cf0d307a6b183105012c38e065710` — Jace's Ruse
- `509f2091e2aa058fe186ef80d09fe2afe0c167ce221ce6863bf53e4d03b188a7` — Jester's Cap
- `28ec5d76b35f3498852fa1c7e5b4072d010b67fb5ba2bdf62713fd6e803b1eca` — Journey for the Elixir
- `54f5bd29ff59d988b8dac7d0dd8bfb3817283c2e7f7a3b5db90a394e70bdbcc9` — Journey of Discovery
- `425754f09534cb8d3665a17bd3fded1de544b11af011348a2bf0b12dfccab814` — Journeyer's Kite
- `933d48f0f18c89c6aaa86dd13938637164c97f9009a5089a77192ff31fa4f7ae` — Jund Panorama
- `4cea195af255ac0d365eba4751d1298ca14c56be71f7e7ce19c5c4c0f7963b0e` — Jungle Wayfinder
- `b7f52fafcfba78a55a0bdd9379990ffa17d78645953616c77393941c0a5bf50c` — Kassandra, Eagle Bearer
- `1d12aa1175fb5b7764ad687d3a92b1f86f2c236d0b0372fb11949445278d2466` — Kayla's Command
- `3389c16e3c1e62779f6965ec3954efe76032c98d9230af01a550a130d2be46d2` — Knight of the Reliquary
- `175a11119270bf3751d50d68bb394af5a09f01dc71ca4cb206c52030b4dedbb9` — Kor Cartographer
- `b70c42426c8eae1a777ea9c4e8bd30a075de5fc1e9dd687627905285ed78bf63` — Krenko's Buzzcrusher
- `2549819445553ba5a0223addbb0d5aadba14d5ed5f9d641c9bb075fd9bf5254f` — Krosan Tusker
- `5ca75d6d74fe4dba13e60c343a3121eb372dc30e105420a2637fd10d9f58ae1d` — Krosan Verge
- `49e91104df6983243823081aefcf34fa7a34e33563b525c7dfc56f382759ee73` — Kuldotha Forgemaster
- `ce06282efcffe861242d66ed0b88f577405da86ef11e5d856686c3e9fe784b0b` — Kyscu Drake
- `e109b10f76c6fd70ca187d1abd8ca1a62f4d6607af95d142abfeb99b8b7a2573` — Lay of the Land
- `484c4b493dfb7f3fc58265b08e8ed9f644aa2d84ba104e4403283e029a0f4681` — Liberating Combustion
- `ab4d2a180726512d8d6a0383fd7b2021781b4ce0ac14998ea57888d217aa79d1` — Library of Lat-Nam
- `c9b9a05a22f50e0b4a5a926068a95a4074da8a9b98c613726612470ed8f269c2` — Lifespinner
- `0823e818a41cc9b1cd9087fe84eedd3bcf631dd7dc396e8724dba90a996f2a63` — Liliana's Influence
- `2370d0c4b15b17aa74ff6c8e567a8ae5db65c3ad67ee2408d018a931d07641b4` — Liliana's Scorn
- `0d2064cc4c8b3cd05ffb01ea0ccb2e2faccd1cf7cae40bd9407cedd4f887b82a` — Liliana's Shade
- `6c28108d5ac83b60c926982f2a479857bf5df97f30d39f973f18e4d35ae65e8a` — Lin Sivvi, Defiant Hero
- `d6d8a2bed836de6c9620133b9e0443742565c8079009c87399975f406753dbe2` — Llanowar Sentinel
- `168a9e4734945641dc0e718b13b6580b1fe4eef7aae517bc53bf21a5b9e531c4` — Lotuslight Dancers
- `dca8eb9fdb401226a2344f3ea8619dc484ae6d8cb7c3476df6afb729ff3b384d` — Magitek Infantry
- `b5a8cd601bffdbbbb1813f7101f286ed642c58829d7d5737115b8649d90f1539` — Magus of the Order
- `0ee92cc84b250b4999ae8ead9834f204853a04629fee00d5c81c446e2faec589` — Manipulate Fate
- `3fbaf3715b22c8e71592eadfdf87fbe86c4d3e6d7de1778c165f16af4223c328` — Many Partings
- `219987d97a2c202b28c56d48dd56202288d60d3be0c0ff0eaeff313e2b9c86cc` — Merchant Scroll
- `6e7ef46eae9eb5545f462ea11ffd897ea1ccdb2dea76ac9cef63036c67cc2289` — Migration Path
- `c4396e449677614abb4b871dc10999b823e9469a4e6f3143dfd3496306ae8eb6` — Moggcatcher
- `0858699096c898ff07a958c4dc2c148deb1a8886b39f55f7c11810595e75a160` — Molten Man, Inferno Incarnate
- `4c44d85b77d81156bec58a66fa3afba0bc4313b0402c38b2eb393bdece7764fb` — Mwonvuli Acid-Moss
- `42bc787c6b9442e0a421d0fe80e49ce09c1da8edc6bca9010b577fce8db02b67` — Mycosynth Wellspring
- `89935a333d636911d8d10f3842a7c3425bbd09c8512789e23160fea5125cf6ee` — Myr Kinsmith
- `46b215a73ea5b701b6a771e457c96059c6d938d2effd840438d14a21b1494fbb` — Myr Turbine
- `fc3c8101a6e618222c2885799ae425c5929cf2cf63025bcae5c7db9455ef4e5f` — Myriad Landscape
- `ac4a400b478c7cf1a4c0ed7e528a4dafaa73c0403587904dd18e85b15ef4104f` — Mystical Teachings
- `2b9fbf6e96826fb9d8ce0c305c9875465400546d8a36585d43dc261572b46406` — Natural Connection
- `20423ac5727bc5498aeca288633ee4017bfc140acbabf025db68fb325cb5dc1a` — Natural Order
- `a96f36117d60e5287db6355574fa981730979c48582aae7c5324b0fd0455c9c8` — Nature's Lore
- `55ae34d8dec448fe14bd883bd401b0d8622f7d4e0ee469f8c6c4aaf235b0f5de` — Nature's Rhythm
- `e6c71fa1e530bb36a7406bdc228ee85bf32f94aede869a323bd750f0db477e59` — Naya Panorama
- `aabafde69b13f842df42d122aa30493b7f82de4cdc1f229a52b7397a410cc155` — Nervous Gardener
- `29de2863663b8b7bbfb5c3fe8a8b6a110d04002306fec39712e7441a9147250c` — Neverwinter Dryad
- `e4c862771b4a328b82ff8db4c48b96cc84a4257a7eb4d6b6bb97ac1def0568cb` — New Generation's Technique
- `bb4a63547aba54a2212430dc2c22d73e4a0e4080f79a0acab098c0f5a9826925` — Niambi, Faithful Healer
- `2afc355c059ce90a3a43706b69812ec9f6a2ff2b4bfa6439619315fbe777f3f0` — Nissa's Encouragement
- `98f42712773034a413a18754036fbf102a72d3e910389340ae66ef70857f733b` — Nissa's Expedition
- `86130c4f75cb62eded1de44a5f2c945ecfddddaa3334058bb2346d028bbbd5c7` — Nissa's Renewal
- `5271e504a7500a09f648adcbc2357179050a17231dfeb9bbe7f3de7417c11963` — Oashra Cultivator
- `25dbc677b781cd02131f3ae3d1968866ce7121361240f5db5f8a7d631599ebb2` — Old-Growth Dryads
- `300cbc30ac875f321e14a1c64706fbf460cb7f8ae544f250cb74d0ab22e3cd5a` — Omen of the Hunt
- `3deab3e272e5be6c03adec5ceb395b750fe479d86f3a533317a72024db00c303` — Ominous Parcel
- `ad263ca4f4f95abde8be043e47899f40085715122b1e71e336308d9ddfeeb03c` — Ondu Giant
- `fe48f0aede49f0479a8a7dde79af26506133927d3933aec48b584f70cbd22d4b` — One with Nature
- `bbd8be3b94cbf3323811b9fbe220687512b04ae69dc6df8eb4dd503787989db4` — Outcaster Greenblade
- `62397c669d4194259e837cd39a8b00ccd0ed55efa8a14c654bf94cb79ac27b35` — Path to Exile
- `3556285c20dc25b27014d1d9365190ba7806ed7a714e330289d5cfd192b8f4d4` — Path to the Festival
- `0980fbbe80df25ae6167aff1f7b328c93bd3b724c92ed79c21ad21c9b92ebdf9` — Path to the World Tree
- `67c8abde207d22dd2590579954197be77dd639d00a564f3ccb5ba4cd58ef111a` — Pattern of Rebirth
- `41a5dc877fdc0561883b168ddb86a3f195324f007483cabe4fd737f303d8daa3` — Perilous Forays
- `82c4539d9521eb237f96f3768265a138febe429dbc3397c92d83a862d6866b61` — Perilous Landscape
- `e57343592684d6dd1767bf641b6854c845d5bb4bc749b20aea5e85cbb0bc9437` — Pilgrim of the Ages
- `0add1e6377b969b8bcca2604a38b3ae55df213fab9a4c5882630b438ec3b46ba` — Pilgrim's Eye
- `2d678b705a3fefd9bd4b9851c0693b4d0cf57415d2e66bb0c41bf05d350d6194` — Planar Bridge
- `c3cc1be8d5f758cee8cae489b54e404dacc82edfff4f32009429dc9cb48bef47` — Planar Engineering
- `2c1878a96ddafc957086f16535e6517349d4fd3374f861ba8c586c72675ccb5d` — Planar Portal
- `28fc9acad0363a2a853e2f85522b83cebcecb8ebf128b6309ea5675c0e5c8933` — Plasmancer
- `495b37f1635eeed674eee1e543b6f4b60fbe26010daf8b98b7c5137acddf35d5` — Plea for Guidance
- `aae97229229502a92c58ae9e26809a8d83471db85823d20260dda46d28e19ace` — Price of Freedom
- `848e8daef69f6936a7fa0eb027a306adc33dc84f0d93627d1dee135b8c12598c` — Primal Command
- `de248c3a29c45e38edf4724416d5762ebe56693681a4cc3eb77cec90a1d65a22` — Primal Druid
- `f0c913dbaf008dc80d3360ecd407ec71a2c6257cba76836e44826a4309da121b` — Primeval Herald
- `f24e262a97decad4e4f5f6882bbd4364f2e06ff2508cfbd51e96060f46da90b0` — Primeval Titan
- `b23a8713d7db6a23fdb534b14d0fed5ee825acdf767ce03f2ac4d25fc4f346af` — Prismatic Vista
- `d11e0209ca5ca21416ccb56dd4578177367742a35115b881288dd9a8fec79e22` — Proctor's Gaze
- `3fae7dedad6d3e6cdb092fe11ac2eeba3cd8522144cf9296e963a6066884d572` — Profane Tutor
- `a505e8a7851b12a08b0547b3d069bbad7a390feef624df948321e7672ea507e5` — Promising Vein
- `5aca8f4a11122186a36fff36d1da5692a960f4f29d720b931a6560751cb131cf` — Purestrain Genestealer
- `e1e0fb652940d75f64dd8bd67ba2f417a3bb63d7c55ad0d740638406776f46cc` — Quiet Speculation
- `79fcd99e9946e63d78860e169c1844da28d167e163acccc5c3d879b28d4a7ed3` — Quirion Trailblazer
- `937bcb4350d3b8376f44e37109f7ed6d670136d16953f83f08467601b3e2b14a` — Ral's Dispersal
- `e1df97bcc3c1c72d3a80e8aacc436757f2d984e33797f3a0729f3d773ed4c81f` — Ramosian Captain
- `6b67f9b78709a6d6169dcb9f8cd062dbcb9251027fa084a8f2869d5fb4ec8de7` — Ramosian Commander
- `e3b490684cfeb82e47405a0c0efaf70680ff2ab55ecccb6da6ce7df627d73442` — Ramosian Lieutenant
- `370a437216b665d357d695595d32a7f0c4fc4b1f0761d74d7e221c783faa8ead` — Ramosian Sergeant
- `adbc9f6cfabf1af3ffb211be7654beebd3fc91552af2209766e0c0aa329876e5` — Ramosian Sky Marshal
- `a382c0368b2c85955a12ab5b9de87caa7e8f510edd31841e458c65229f07d193` — Rampant Growth
- `b7257b3cd9c76a7b4081a24c08a97dab5e768b937dfde91f802bfc4c1f55cc65` — Rampart Architect
- `7b6eb1b34545f64e55de65370aee5c81f39efdee1fb7aa8eb8e49cfe2907540a` — Ranger of Eos
- `805875a82127418846ed7c3fe0ef78c5667706202305a517615727e3961f0494` — Ranger's Path
- `44316589aadce824e04689bc773dd7991303b470ed6b02892148d2915fdbf256` — Ranger-Captain of Eos
- `84f406c60d10e07db061d83018616b20abbffd23f1cdfb65d9433a9dc034344d` — Rathi Assassin
- `7bb0a20a41151a47aa13807bad3182e1c0f2859b31448428ddd41ad721ff5015` — Rathi Fiend
- `9a14f91ef94ef4c855a59e4296c2bbf04e0331eeb50106610b3c0c8e7cc96f8e` — Rathi Intimidator
- `1851e183010f34c5926ba29c7bfcbe1eeaa3de3899a5994d4208b001d9557545` — Raven Clan War-Axe
- `b1293c8fc0d86ea509d29930c4677eee8a6ddd90b457ac9f73966b89bde4b8ab` — Razaketh's Rite
- `ca90f451ec7f2eeb4ee1259862283242a385c7b365a4d7e1130c46b2874d02f7` — Razaketh, the Foulblooded
- `39631f49f09426e7d1c3c9453bb1ab47fe3fc17b723e1a16c0d300cf8ee8f9bb` — Reap and Sow
- `785f1d55896e938884c89f4bb99b9e8b8fc4a26fb109f9eab13b882f46a5028d` — Reckless Handling
- `24f72cc7b1c24d8910d1ce48632c31e433a3ff8879c79bbddba7f4ed0f4c2687` — Recruiter of the Guard
- `8ee17452f6a7edc93bb7bfc156a04cd367fbce0042eaa97bd15091b0c19fa3e8` — Relic Seeker
- `fb746d5f8d183f04bae57b4ea9e15ebe3f0af0112cab3267f258f78650ca7cf8` — Renegade Map
- `2c9d7a08dff70baa684d1b073405ca0cc4295ecd9196e4a4045a27b75bd795e7` — Renewal
- `6df3d2fbf20e514572eaad8724f93a45d0aa5e0b83ee819f50c276277f2de791` — Reshape
- `9a3bdef12cf2629270b05bc0c84e61eeab42210b158357982c5a1e345716e440` — Reshape the Earth
- `7aaaf326d4256b63b891bdc5c1dfdc50f9cd3c5942b747c9e56c08bbaa2f81fe` — Return from the Wilds
- `96b7f302eb3ed9c3e840b3161e5dc043ecfbd80b76856ccfbf993a69624150df` — Rhythmic Water Vortex
- `a047b85a36eb5fd4cee72075b719461c016478c29a68ae08297c3af52c6c9104` — Road // Ruin
- `dd45a809723d9a49ae8a7e4118e49b26308d0017b28b46b852daca30d8a5a459` — Roamer's Routine
- `ea7d6363915458938d53650212e65dea7787b7f8bc3733f732060e6e1151084f` — Roiling Regrowth
- `f82d9902ee7f898cdd3152143cee696518954917278616431b2c99c486b32d4c` — Rowan's Stalwarts
- `09cb0a5eb6619833263fad679830821a90f97e8aaf9a998a0a904043447f0054` — Rune-Scarred Demon
- `396ef0833d44c1708f62e60de67b95c63a2198e5a0044d0b7b5ff0127da6a279` — Runeforge Champion
- `b4645c956959c78ea16322c03c202cc57b9b122c5eb1e42b1860fd429c4067f3` — Sagu Wildling // Roost Seek
- `0745f4e42d7f2055e129f99e272acb2368fffceb4fc597f36cf568e993b0d176` — Sakura-Tribe Elder
- `442ef949678fe415888704a8d3456908e19f0cc85fcb74324b1da88b7c40e10e` — Sanctum of Ugin
- `9cdb2ea101becd2172cd77910a98e4ae179393bb4c158f5ea0bc46de7da379be` — Sandworm
- `a9f269c5971ada0bd10e25790de5ea889e5ac28108236dbe012961478c01c13e` — Sarkhan's Triumph
- `560ddf7a26c085dddf1afe667388de1dab234df267d241bbe90d09e28d50ba43` — Savage Order
- `20a2493c48d465235e71e3e9d60134d5ad0799c08091a82a305bb2960d88f435` — Sazh Katzroy
- `fb8ce54500ffcc1132f3b2404dee33a8877da7f98187599e0e6288893a3e27db` — Scour for Scrap
- `015e0bcb8b5e245522a7d0e10d4da07f70de2f11c53e6fb36fa739929c04ed2c` — Scout the Wilderness
- `88c4b09f7083f420a12a172b974b3c8aa28a83f8b6423ff13eb4891eb3ff20c6` — Scrapyard Recombiner
- `563fe608a4620b4daaf2b3eff45daccd8aafe952950aabc75e1e14b9b171ef5a` — Screaming Seahawk
- `dc8e9d7dfd70ee48743bb896f38bd48241d802b53b937b756e3c6f7a2768834b` — Seahunter
- `cdae65c23d8122fb0263a737651dbecf10f112340eceff71e86973c467010648` — Search for Tomorrow
- `98e1bc2ee0bc93c6f6600dd395404a29bad991b25882abaa7e56437bd65a9679` — Seedguide Ash
- `d07b6bd384dbf97bc324deeb6b9e518b14265e9cbd9a208e9a05c6f673c87b04` — Seek the Horizon
- `ce1a335fa8aa08e9ec3e2ce4f58c37785bb4bd150a6d6b1a9d83de96f5e62db2` — Seething Landscape
- `30727b694d82da282e3d3606cef0beea1009a4cd25c2962c08d1442a906e2762` — Self-Assembler
- `8350c16c44367e76e8bc4e7c67829b614f27b57e15c943b9fb66bf5c53ac03f7` — Settle the Wreckage
- `b6517413cf89372bb8e67232a291d06590ea1e751bd6ce2faf859fac0b9f7a4d` — Shadow-Rite Priest
- `f29e1af59cde4f6c663cd25fe7e734eb78e8cb427200b908cba7f53a98621ca2` — Shard Convergence
- `8275de836e8006860ee9570beda0166f1ad5784b8f57e8ac7cd5b84e78a015aa` — Shared Roots
- `6196786e5ae371cb1e3f36770978e83c212f7e386dda35757eabc63791168846` — Shattered Landscape
- `668968a69af5974b62535bd08a975e76368bd0eac6baafbe88dcaf6bcbbcfd3f` — Shefet Monitor
- `d35c8e786fab3f52de0027d57a383b466636c3ea3b27922b0770d2fdada34f2b` — Sheltering Landscape
- `8c2506f6467b5da69ab380aa4a35d3f16b3c6fe33877034d3e757a6987cac31b` — Shield-Wall Sentinel
- `d9a987293a34838683fe6554cf1aa8dfe635fe6356a48554c235d408919ff29d` — Shire Terrace
- `783e318cd7a10a05a93d3e11e12baf96330293df06efe0c8ca44666738fa50e0` — Silkwing Scout
- `d4d9b59e342e6903cbeaee2824c9c6adcbe87414dfd4a37062354798a65d6e9f` — Silver Surfer, Galactus's Herald
- `588b7dca853b4382b95b85b169959e0e39a44fd447baac62d89e0b478a0881b8` — Silverglade Elemental
- `c31b5394602110a23efb8d10183a1f57c9376ba4f671cd5374da2c96699fcea2` — Silverglade Pathfinder
- `ac62ad5fd285eac4d613f625b49bcedf0fa65a198cb4a74ac97c6bfdd4cd80a7` — Skittering Surveyor
- `522ba057261d8620605979aea32c58279f5c9e81b1b799b68dcb104d27c93ae7` — Skyshroud Claim
- `36db0b173dda0140bf82931e25316cc0fd8e7d1cce14d076d44700fceef9ce8d` — Skyshroud Poacher
- `d4ff40e071565578b487632d1cc37558735cc637db1e22e1a59c933563c73bf8` — Sliver Overlord
- `f0e1dc71f28f255f4b4c8cf321769a5714c3c054789a211d700a55e1136b5090` — Solemn Simulacrum
- `fa1815b78953c4c806369e34ed0572f7dd5f09c92e9aaf48bb615a89af939822` — Sorin's Guide
- `c45716e3a8546f16e4b2f54a4e125502c840083b4af64c550d50cbf70db06c69` — Sphinx Summoner
- `ed89aa5f43fabb63670191722ae23280479775c817db25cf41b4f2f516009e1a` — Sphinx's Herald
- `53f58cdb9db511c7e78ff0951ebba3d393ea80eba592c8243c7de062ca0eebf4` — Spider-Man, Brooklyn Visionary
- `bb56cb70029631de0f24fe57be80e093ba9c3bb600afbbb4fd05aae88ef7a159` — Spinewoods Armadillo
- `cae14710a785049d303da9e93752a8ef321072c3a88676de31e20dcd0422c3a7` — Spirit of the Aldergard
- `8dde64a40203b08cb88c3f987629f4555ed0c00be78a5722e7acd7aac8d16905` — Splinter's Technique
- `9c8d9e4458dbe86f920b14c7ae11d75316357c2da67ea52e8475fe417d11d70f` — Spoils of Victory
- `62bc3ec1dc5420fcdc56c1a60ac1f11234212b15aa95225dc94728d72841e3c2` — Spring // Mind
- `913289ffec783d62d5e94443c87c080aee751db342e9389274f5c738cebe0211` — Springbloom Druid
- `2086f02e161acd05adc0798b2a03db5c027dacc5534c3bc07b41d838bea1babd` — Sprouting Vines
- `28813106d6448f15b9e7da2932f58ebe915620066b7cf445c291e449ff3ab145` — Starfield Shepherd
- `2e3514038bd0a66a145e89671c291419fb3c222f07687424a3cc5473e6b62e5e` — Steelshaper Apprentice
- `db5bf8eb5242d017531924172032ced7812d47d1ec2781f69aa2f2c68a266015` — Steelshaper's Gift
- `dce5daa63c75b91516f57ae4edf1f89a95ef1f269020d2f0f36da24e03afd624` — Stoneforge Mystic
- `1429df627befc33faa2ccbed580d46026b3182c75af9250cc5d696e291cde9f5` — Stonehewer Giant
- `dc1c781aa3cabbb6ecd22aab9b4fcf08b7c5d07ccb98bed22d1c55e8b6956aa5` — Strixhaven Skycoach
- `6dd5686093ff3b232435b2452f0b968199ebd2cf193c9e8a68ca6142fa0564cb` — Studious First-Year // Rampant Growth
- `d549dcf4b85e30158925af3b91839b72424211f69b5fff721fd5baa5e0c83b05` — Subway Train
- `9d0aa024c38ed36e5e728871b1e90c78137e6623266b66b27f2fd9f892c041ad` — Sun-Blessed Mount
- `abc59a321c1528cc10ee360b16d5a1aa46d8cc63d2039c3228c04f40df562722` — Sunblade Samurai
- `335a99d08aef0816068286d097e7383d7193e2fcda38e153c53eb32d99a1b369` — Sunforger
- `8355dbe6c66fc680a860d1eef3093a0d05591904de5dd58b29a51a45897f5cb1` — Supply // Demand
- `d64ebaf56114a32d2178d25cf9807d6a77588fccc642faf2376c2ab3a4412b14` — Supreme Inquisitor
- `d6ab26fbf4441f92e27fb72687b015b5e16ed65c5350f2df7ccce5b1b7d57244` — Survival of the Fittest
- `696af6805a9f7f760c62c4a827afae31427867e24e9a3d0a10f51ae8746476fa` — Sword of the Animist
- `00e7f0f770bcf19dc9127e9930b7fd7a69fee9888f55ba5c184235d9f5ebd938` — Sylvan Ranger
- `f586f104da8f6a92a968003c479380e9e672afbc121fa2fb625191baa179f118` — Sylvan Scrying
- `117fbead032d5245e994fb869be4c3cf1faf29d6461b254adbf971f77381b6b9` — Taj-Nar Swordsmith
- `24a76c697b28eac01caeffa7e56c20d8a57a116c4858e91468e76e5dea5e6eb8` — Teferi's Wavecaster
- `1ba2783dee7e523bed97b9e7e994f0c7e5035de900de4afaa429f074b4e0bfbb` — Tend the Sprigs
- `64e4dce9bff766c3c41ea5421312f326f335950edea1d91a047a3f76d893247c` — Terminal Moraine
- `6e3cf72745d17f4191677a5847fe09cfe501a8cd03b5a47e4db64cef495320be` — Terramorph
- `b91c003aabbf45510e1dcb778684015195ade0a1d7a8b982dda97d8910101bf2` — Terramorphic Expanse
- `5e63fd18fa55df8a88a79294570c2b29b27e18d90c74d4f49e1d57cd32f5823c` — Tezzeret's Betrayal
- `bc37306a0d401a0c7255be82a209d177ed0a3953be67a86c288064dff1e88970` — Thalia's Lancers
- `8e12b147e0ea474a910060a1da70bf09ebc4809c7e47509edc868ea394f55ac0` — The Arkenstone // Seek the Heart
- `b6a865207d9ba50bbbb4fd32727714d0a420bddc01af87e42ac4219a2e8b3fa6` — The Birth of Meletis
- `509b34d3ce4ce55cd5785b03cb6388d12621a99284301d795323e0d4443fa6bd` — The First Doctor
- `dd233695ece745aff76f1645a999264982bd733005c1253089989046027544c7` — The Huntsman's Redemption
- `f7afff23eb09ed463cec2fceb53d3322c5b8926faa27ac208fefafccd7096c18` — The Masters of Evil
- `90eac471fd7fcad5b3c1d457f01fd4cf575b02211e36b4ba319cce3d4b492602` — They Went This Way
- `8472736ff9d13def45170018e6a0e29c2fdde8a53b89c0798948ce9759681ab7` — Thirsting Roots
- `f6754e45bc17e1b01ed3e4becf15122c68f4bab36ffce845e109536122e64fd9` — Threats Around Every Corner
- `8c07322e0ee48f2d5e110dfa9c7cbb32040d147b4613cd516254d9aaa8351eb9` — Three Visits
- `359cbb3f6fa5a59f4d222cfe274248f5b973b9590ff645607adf866df5a69bed` — Thrór's Map
- `8f0754327d607dbd577d023f110c56b2a8577570fa1c4bdd6963f167d38b0525` — Thunderherd Migration
- `90f8eb9fe6ea3991c9639b801ca4ba0e5096151c59271b5797736288aa239c6e` — Time of Need
- `8e3b2906a0a7de6960135a0a4e2e7d0fb3832b94b854e3d36cff81feff744fc3` — Tinker
- `e4c28203d1513820248b755bd0cf29ca910ef90dc70bbea811c1c36f999eaf92` — Titania's Command
- `b2c2db4c633c049382cfb78d39682e3ff05ae5555641add3ab869a0ba598c5d6` — Tithe
- `93029a65d7cf7271244632bcc606098b2d716a65eba988e1966f6e9a8cc48475` — Tooth and Nail
- `d91978d500906f1aa55989be6ce07591d60154b6dea5982d8dc6633164251c7e` — Topiary Stomper
- `2aef6fd65252968295acf88fe59dedbbaa2895534c12000fca295565d8d9eebd` — Totem-Guide Hartebeest
- `72aad856036b1880b0bbfb8a607ff90d6096cbdcb55b91252c54cfb50ec327b7` — Tower Winder
- `5f44c986d900e8add1c39f5b1a8a7b665026604a5b1ada84ba42e68802cfca6a` — Tranquil Landscape
- `18c09ef311c249fe1df9df6fb112294a255e0746037b81ec66193f889b32932e` — Trapmaker's Snare
- `8ad6083066966325c6c63ea3a9acb1b82cedd9459262ca695a13fd15d0c9033e` — Traveler's Amulet
- `1f953e14e53291b7d90b8f0904c8a97686d99c50244b27434772e5b3c2d58220` — Treasure Mage
- `08e7a2dc300b74fde66ccaec6bb5a992c15641cfc00cff6dcfe69ca0e01d2dc1` — Tribute Mage
- `f871d39ab307e4e9e8777cc0f22d7ec9d33d4c0a753b790ae6684eb71118cab8` — Trinket Mage
- `988f0b0b85863c1ea05281403931f2da6d041f6c55e103e3853dc91818e4afa1` — Trophy Mage
- `e4114bf7edf6eda31306640f62cae0a68b6386560e3a1d92072a0b976edb8c74` — Trustworthy Scout
- `25e898a9b0d42abfb47d298172f1e2b4c2b657ee2f1815b00d686c90c6e1c25a` — Twice Upon a Time // Unlikely Meeting
- `8d58498a843c95bc063f74a8b9ffa5f826acf93897d590f658512f889e488348` — Twisted Landscape
- `b79d4ca8974d200d3563a3a6d8ab37df61adfcd622a577d36f6812c7a5b8115e` — Unlucky Cabbage Merchant
- `7a49d257ebdb0851e6eea7ad8f390e07e01d7369767bd5b2abe57fcdae62f394` — Unmarked Grave
- `6115a7ce8a1668ccf04f4a310586e09430c44c8f1315aedc4ff7d6e4ebc0232c` — Untamed Wilds
- `bc08414503c8f7ca85e00dcb0aad805fc03fc851de9866f48f039f1a54e04cae` — Urza's Cave
- `ef5bb43a0d03161d1ffc1ecf6fd1bf6b4daed37f790118514991e0f7b0af689f` — Vastwood Surge
- `139c51850e0f59abdc5fa1755f6c28e3914e14d15c1d6a3beef236f602b5fd81` — Verdant Confluence
- `1abf978f89980c75b2a2604c433ccdc2f91a782ac4a08ac37453756264bbe8b8` — Verdant Crescendo
- `f7e1c7c78a5b9241da412160ae3dc56ba1adc18f92e3d88e6db38610189887db` — Veteran Explorer
- `4a7a5559a44a7c6f9a5a9b622709a25f5daeb5d54f88131e56504b6e2bd719db` — Vibrant Cityscape
- `c5bca1a930a981696c8f9912a23aa8a502be18b3a99c4035c3b1c57baf52de67` — Vile Entomber
- `a0c8c7c22bbfb64b73428592211dd9dd16f9d823588150c184abc13784878fc2` — Viridian Emissary
- `a7adf9e648ef1eb5ad511d5bce989e92a964be1f194f2d7944db3477a6c311d5` — Volatile Fault
- `5212c3d3cc5ef9cf088011f8135cbfeb4cfaf8807c50b2e248228ae9f6f7f1e3` — Vraska's Scorn
- `510c66e49b93adfc8ecdaec34ae841a917865645b293690ded753df9b36aec36` — Vraska's Stoneglare
- `a751f778a145faa3cfb82b7946c9d56ec6f31b07f51d8342c765abe1b669ae63` — Wanderer's Twig
- `b2d9fdc9e81ae5007d6807a8d9ed20269dbe57eda8e1e9cb1040cd9ccc004a9b` — Wargate
- `2d4609d9fc6bacd3caaf2e1d30f28e3fa7e213ff0ac94da75c680671d842e618` — Warped Landscape
- `96dcc2a89352f8037abc842e265a36bf52b7810c6e8c093178acad8eb027c46c` — Waterlogged Teachings // Inundated Archive
- `ea217f47b734eb6d1d42acd4e0bdc1f96ffd443295dd4a992bcc6a2d3e200172` — Wayfarer's Bauble
- `e5def5104dd26466f1641c64d0a45923206efbc96ac025308ac2c4a6b618975e` — Weathered Wayfarer
- `4ea0a021c8d852e2080493175c2fb92c8bfea434d5b3fa4bb13dfaa91e8e3508` — Welkin Hawk
- `f0fa98534922272fa53e43a46389e0693557bb6956a4d4ce4a6c58b349d73de1` — Whir of Invention
- `531329c99d318f6ec8bd8db83c667f06e73a13cdcd44347917b287698e17b0bb` — Whisper Squad
- `68392697d41e07573cab99272b68c8b2226f449a2d486b5b41f40accc2c925d1` — White Orchid Phantom
- `66b8f411216ee3a3d05640381586a7f7b4250ec73f9b6eb19e011f28d250a5cc` — Wight of the Reliquary
- `b26442715836f0cd8749c3ee7a60e108088b2574f0e4761a1614cae6b7b06819` — Wild Wanderer
- `a69a76d516d49274ee6119372c8ff74c9017ccadc445b444377a3357fe67b470` — Wild-Field Scarecrow
- `8b6cda5aecc314f7fcc9bab34e5ffb8b2d05f076d1b7eb4964e25e0d90be1114` — Winds of Abandon
- `233bf9c864a6fc8d4d2e15b762c1900723a61360ad98117506d85078d8f4f544` — Wirewood Herald
- `cab4cd2b58e4eea817fb1b0ea4ed555663b3156f3262f7c6de41b45ea5c414ef` — Wood Elves
- `536dbbf14324ebc2a450e5bf5e01060be0a1be3611df2033a2743a9cbc9dc681` — Woodland Bellower
- `fcd7cf96a71d60dea1c0f73b828fedd4f96f4a7ac9999f9270a8c8039ae1cabf` — World Map
- `86a2f68e7fa08ebb7aaf43369c263fd5c45ce3be294e2e3996ca2f453f92684b` — Wretched Throng
- `e2b4ef454b8431df437092a651b76e41dabe75fccda13605a46b5e62a325ec19` — Yanling's Harbinger
- `27810d037cc0ac7cfeb8a1c8259e8d3307f0a96e39e5ec1bb0aa9d4d831d2b9c` — Yasharn, Implacable Earth
- `81cd379602089aebd7c528d4f7919e91f13987e64060947c3b1a4afc6ae9e6e6` — Yavimaya Dryad
- `3d614dca5676c2f060d086ba2203ae4ef3b4fff2054682f48bce645662b35aee` — Yavimaya Elder
- `23365fe27690ad025f6cfea056ee12d6ac8db5b0d09b2cf2efd9bcbd483b03dd` — Yavimaya Granger
- `3d75520661bec94f63f0ec866fc986802e4680d044fadfa9867173960cabfca8` — You Happen On a Glade
- `c493efcdc199acac1fc537a4ce71660667628afb29bcfe012d62c6075d587055` — Zirilan of the Claw
- `9aa6ff020f9e7df9f05e04d7381afc89ef78c76704b30663751ae62024f6c067` — Zur the Enchanter

#### `VerbPhraseAndFrameComplementPairCoordination` vs `PredicateAdjunctPredicatePrepositionalPredicateAdjunctPredicate` → `VerbPhrasePutOn` + `PredicateAdjunctPrepositionalPredicateAdjunct` (2 units)

- `3dec1f463f44fe533fffdb309a7c36005b22d6d8e70bd85b862e94f7964c9199` — Ajani, the Greathearted
- `c99ec7be442c79322f5b15f08e6c0676bfeef77003708e1a8e74983d1742c046` — Filigree Vector

### REPORT

- Construction count: `397 → 397`; no construction, Verb Frame, or require was added to hold a census number.
- Licensed vocabulary/lexicon homographs, reported rather than fitted: `AttributiveAdjective::Untap` beside declaration keyword action `Untap`; `TargetingMarker::Target` beside `CommonNoun::Target`.
- Form-literal/vocabulary overlaps, reported rather than fitted: `additional_cost/additional`; `up_to_quantifying_determiner/to`; `definite_next_mass_quantity_reference/the`; `definite_next_mass_quantity_reference/next`; `scalar_less_than_or_equal_to/to`; `number_of_scalar_value/the`; `greatest_scalar_value/the`; `other_than_qualified_reference/other`; `positional_partitive/the`.
- Inventory pins: 2 licensed homographs, 9 form-literal/vocabulary overlaps, 24 permitted licensing checkers, zero forbidden licensing checkers, 397 constructions, and a 72-byte scanner carrier.
- True contention for every figure below: 1 concurrent codex executor and 0 other reviewers on the host.
- Coverage performance advisory on `nqtnzsxz`, `covered = 20,254`: `--check --workers 8` took 118 s against the 16,260 ms quiet-host ceiling, at 140,536 ns/B, with host loads 456/508/526 hundredths.
- Ambiguity performance advisory, the matched pair back to back on the same host at 8 workers with identical `--require-resolved` flags: parent `upyluyyr` took 126 s at 141,772 ns/B with loads 736/543/544 hundredths; tree `nqtnzsxz` took 117 s at 140,911 ns/B with loads 513/539/542 hundredths. The per-terminal role-preposition materialization costs nothing measurable: the tree is 1 per cent cheaper per byte than its parent, inside the noise of the load difference.
- Roundtrip performance advisory on `nqtnzsxz`: 115 s at 137,320 ns/B with 8 workers and loads 1,966/907/661 hundredths.

### Review corrections

- MEDIUM — every figure in the record was stamped on a `20,054`-covered base that trunk left when the preterite landing integrated. Fixed: refreshed, resolved the generated-terminal conflict, and re-measured coverage, the matched-parent/tree ambiguity pair, roundtrip, the closure test gate and strict clippy on the refreshed tree; every number and change-id stamp above is the re-measured one, and the changed-identity list is regenerated (447 → 450).
- MEDIUM — the record's assurance counts (`re-spelled 6`) counted two generated-debug-string re-spellings that the refresh dissolved: trunk's preterite landing gave the terminal a manual `Debug` that prints neither `inflectional_forms` nor `role_prepositions`, so `parser/mod.rs` and `parser/scan.rs` are untouched by this landing. Fixed: counts corrected to re-spelled `4`, and the merged `Debug` behaviour disclosed.
- MEDIUM — `Deviations and additions: none beyond the ticket's letter` understated the change. The ticket names "B6's existing frame-role accessor", but B6's accessor reads the codec's static frame tail and cannot see a declaration's other frames; the landing adds a per-declaration accessor, a terminal field and a `Visitor` callback, and keys the rule to the head verb's whole declared frame set. Fixed: all of it, plus the first-verb host rule, the two hosts that contribute no frame, the retained plain check on `existential_predicate_adjunct` and the synthetic `Island` declaration, are now listed with their justification. No ruling is contradicted: the design doc's operative acceptance is only satisfiable on the head-verb reading, so this is disclosure, not a resolved contradiction.
- MEDIUM — the perf advisory carried no true contention count (the sandbox `pgrep` is meaningless). Fixed: stamped, and a matched back-to-back parent/tree ambiguity pair added so the advisory can actually answer the cost question.
- LOW — `ClauseVerbFrame for PrepositionalPredicateAdjunctHost` skipped two of the sum's three variants with an `if let`, hiding the omission from the next reader and from the compiler. Fixed in `review: make the adjunct host frame walk exhaustive`: the walk is an exhaustive `match`, so a new host variant forces a decision. No behaviour change; the corpus figures above were measured after it.
