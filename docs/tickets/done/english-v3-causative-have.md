---
needs: []
---
# Read causative *have* with an Object and bare infinitival: you may have target player mill two cards

## Why

Causative *have* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), *(may) have* + NP + bare
infinitival touches **253** unread faces and is the sole cause on **101**
(recon bucket "causative have": 307 / 114, which also counts non-causative
*have all activated abilities*). Counts are unread faces *touched* (at least
one localised failing unit matches) / *sole* (every failing unit matches and no
other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "You may have target player mill two cards." 0; "Have
target player mill two cards." 0. *Have* declares `Predicate([Role("Object"),
Role("VerbPhrase")])` in `verbs.ron`; it is the `core-verb:Have` row
`Complement(Object), Complement(VerbPhrase)` of the unsupported-inventory table
in the done `english-v3-generic-frame-consumption` ("unsupported slot category
Object (Complement)"). This ticket takes that row over from
`english-v3-systemic-residuals`.

## Goal

*have NP VP* reads with *have* as a causative catenative: the NP is the Object
and the bare infinitival clause is the catenative Complement, with the Object
understood as its Subject (*have target player [mill two cards]*). It reads
under *may*, in imperatives and in finite clauses. The legacy `Role("Object"),
Role("VerbPhrase")` frame is replaced by typed slots the generic consumer
admits, and retired (Method 9).

## Analysis

Only a small number of catenatives take bare infinitivals; among the causatives
they are *have*, *let* and *make* (CGEL, Ch. 14, §5.6.2, p. 1244). CGEL,
Ch. 14, §5.4, p. 1236, treats *have* in the complex construction with a raised
Object. Applying that account to *have target player mill …*, the
intervening NP is the Object of *have* and is understood as Subject of the
bare infinitival Complement; it is not an overt Subject of a finite clause.

## Witnesses

Four retained witnesses come from recon `xxknlzypsnwy`; Extractor Demon is
attested in the gained faces of the fresh census, replacing Mirror Image per
the orchestrator ruling of 2026-10-07:

- Quill-Slinger Boggart: "Whenever a player casts a Kithkin spell, you may have
  target player lose 1 life."
- Rage Forger: "Whenever a creature you control with a +1/+1 counter on it
  attacks, you may have that creature deal 1 damage to target player or
  planeswalker."
- Joraga Bard: "Whenever this creature or another Ally you control enters, you
  may have Ally creatures you control gain vigilance until end of turn."
- Extractor Demon: "Whenever another creature leaves the battlefield, you may
  have target player mill two cards."
- Ebon Dragon: "When this creature enters, you may have target opponent discard
  a card."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 6 --samples-per-face 0
   --output target/english-v3/refreshed-base-census.json` on the
   claim parent, stamped with its change id and covered count; after: the same
   command to `target/english-v3/refreshed-final-census.json` on the
   final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the refreshed-base census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. The bare infinitival read as a reduced relative on the Object, or
   *have* read as possessive with an Adjunct, is a defect. A wrong analysis
   that parses is a defect, not a gain.
6. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
7. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
8. A new frame or construction must not overlap an existing one on the same
   string (two labels for one constituency is a spurious duplicate, not an
   ambiguity); when it supersedes one, retire the old route and re-spell its
   tests.
9. Retire a superseded route on both the lexicon and the grammar side; do not
   leave unreachable declarations.
10. A CGEL citation may back only what the cited passage itself says; a project
    or orchestrator ruling is cited as a ruling, never attributed to CGEL.
11. Timings as integer ns and ns/B, with host load and worker count.
12. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Possessive and granted-ability *have* (*has flying*, *have all activated
  abilities of …*): existing frames, untouched.
- Perfect *have* with an Object Gap (*spells you've cast*): owned by
  `english-v3-systemic-residuals`.
- *enter as a copy of …* inside the infinitival: deferred to
  `english-v3-systemic-residuals` by the orchestrator ruling of 2026-10-07.

## Landing record

### Implementation and resolved STOP

The unsupported Have frame is replaced in place by:

```ron
Predicate([
    Argument((relation: Object, category: "NounPhrase")),
    Argument((relation: Complement, category: "BarePredicate")),
])
```

The existing generic `SelectedPredicate` consumes its Object through
`AccusativePhrase` and its catenative Complement through `BarePredicate`, whose
head is a plain `SecondaryVerbPhrase`. The old `Role("Object")` and
`Role("VerbPhrase")` slots are retired from the lexicon. That unsupported frame
had no causative-specific grammar Production or declaration: no unreachable
route remains on the grammar side, and no competing label is introduced.
Granted-ability, predicative, possessive and perfect Have frames retain their
contents and positions. Other verbs' unsupported routes retain their owners.

**STOP:** Mirror Image exposed a ticket/ruling contradiction. The claim-parent
probe `enter as a copy of a creature you control` already admitted no
`SecondaryVerbPhrase` root. The ticket's instruction to reuse Enter therefore
had an unmet prerequisite. Widening the trailing predicative-*as* Adjunct would
contradict the [2026-10-06 orchestrator ruling](../done/english-v3-fixed-cost-phrases.md);
adding the missing Enter Complement exceeded this ticket's scope. Work stopped
and requested a ruling before resolving that contradiction.

**Resolution, orchestrator 2026-10-07:** “defer the Mirror Image witness; do not
extend scope.” The ruling requires the missing phrase to be “a Complement of
*enter*, never a trailing Adjunct.” Mirror Image's new regression witness is
re-spelled as Extractor Demon's attested trigger, drawn from the gained faces:
`Whenever another creature leaves the battlefield, you may have target player
mill two cards.` Its Object and complete embedded infinitival assertions are
retained through the same helper. The five-witness count is preserved. This
explicit scope ruling supersedes the original Mirror Image positive-test
obligation; it is not a weakened assertion or an ignored test. One bullet in
[systemic residuals](../planned/english-v3-systemic-residuals.md) owns the
verb-selected *as a copy of <NP>* Complement and names all 60 supported
*enter/enters as a copy of* faces, including Mirror Image. They all remain
unread; their membership is a surface observation, not a sole-cause claim.
Both orchestrator rulings are project authority, never attributed to CGEL.

### Census, provenance and no silent loss

All final production figures below are stamped **`vusnwtmuoytr` / covered
19,826**, unless a refreshed-baseline stamp is explicitly given. Evidence is
retained in ignored
[target/english-v3/english-v3-causative-have](../../../target/english-v3/english-v3-causative-have/).
The refreshed-base report is measured on `tlupskwqkwmv` / covered 19,682,
an empty diagnostic child of claim `pozynvyu`, with the exact base production
sources. The final report is measured on `vusnwtmuoytr` / covered 19,826,
with the typed Have frame. Their lexical inventory hashes are, respectively,
`f34ca4043c1b8ba59b19b264b575f70adfa031c889396373a62819d976c58658` and
`2b96cec97b9b2d3a35a6e3f93756e092135ecbc4dbb2899d298fac25920cfdd8`.

The refreshed base is `mtxtksrr` (`english-v3-causative-have-`). Final refresh
incorporated the chooser-and-taker landing, including shared plugin data,
xtask and Lean changes. The gate was rebuilt and rerun. Trunk's
`swptuusp` removes the out-of-scope Attraction subtype; `xttprnpy` regroups the
Lean commands, and `ouysnvuw` / `xxqksrzw` change semantic actor fields and turn
helpers. The dedicated refreshed-base census has identical per-face Reading
counts to the original before census (`ostqqnplnqrm` / covered 19,682).
**Trunk-attributed Reading decreases: none** (`trunk-refresh-comparison.json`).
Thus the comparison below uses a freshly measured base, without borrowing any
pre-refresh production totals. All face identities and analyzed-source hashes
agree. Refresh completed without conflicts and preserved other workspaces.

| Census stamp / covered | No Reading | One Reading | Multiple Readings | Exact Readings |
|---|---:|---:|---:|---:|
| Refreshed base `tlupskwqkwmv` / 19,682 | 13,146 | 7,294 | 12,388 | 560,894 |
| Final `vusnwtmuoytr` / 19,826 | 13,002 | 7,331 | 12,495 | 584,148 |

Aggregate packed-chart metrics on the same measured trees (not per-face peaks):

| Census stamp / covered | Items | Families | Completed nodes | Completion work |
|---|---:|---:|---:|---:|
| Refreshed base `tlupskwqkwmv` / 19,682 | 125,555,843 | 131,576,578 | 2,919,870 | 20,433,596 |
| Final `vusnwtmuoytr` / 19,826 | 126,367,237 | 132,421,288 | 2,937,331 | 20,556,916 |

Both runs completely enumerate all 32,828 supported faces, with zero failed or
limited enumerations, undetermined faces, validation issues, duplicate
Readings, cyclic derivations or internal failures. Gains: 144 identities and
23,254 Readings. Lost identities: **none**. Decreased Reading counts on
previously covered faces: **none**. Increased counts on previously covered
faces: **none**. All 19,682 retain their exact former counts; no identity is
silently removed or owed re-coverage. `comparison-refreshed-final.json` records
the identity-level comparison against the refreshed-base census.

The v3 contract retains every admitted grammatical Reading. Its One/Multiple
census is the selection census above; specificity-resolved selection is not an
active v3 policy, and this change adds none. No tie is discarded. Every counted
Reading passes declaration admission, lexical ownership/context, byte-exact
realization, and construction/Word traversal comparison with materialization
traces. The corpus command does not prove independent linguistic correctness
or the independently constructed-value roundtrip law; the structural tests
below and gain-tree inspection supply those separate checks.

A sampled retained tree for **every one of the 144 gained faces** has its new
Have frame inspected: Object NP followed by a `BarePredicate` catenative
Complement. Other Have frames in those documents remain distinct existing
uses (e.g. granted abilities), not duplicate labels for this constituency.
The detailed ten-face review additionally checks all 50 Readings: Renegade
Doppelganger (7), Browbeat (1), Distant Memories (7), Ebon Dragon (1), Wandering
Troubadour (5), Extractor Demon (1), Lava Blister (2), Quill-Slinger Boggart (1),
Rage Forger (15), Joraga Bard (10). None of these inspected causative nodes
absorbs the infinitival into a reduced relative or a possessive-Have Adjunct.
See `gains-spot-check-analysis.json`, `gains-spot-checks.json`,
`spot-check-analysis.json` and `spot-checks.json`, each measured on `vusnwtmuoytr`
with the full-tree covered count 19,826.

The five full-face witnesses gain Ebon Dragon 0 → 1, Quill-Slinger Boggart
0 → 1, Rage Forger 0 → 15, Joraga Bard 0 → 10 and Extractor Demon 0 → 1.
Mirror Image remains 0 → 0, routed under the ruling above. Real finite *has*
(Lava Blister) and *had* (Wandering Troubadour) also gain, 0 → 2 and 0 → 5.
The latter experiential use shares the complex catenative structure; CGEL,
Ch. 14 §5.4, p. 1236, discusses the non-causative “undergo” sense. No imperative
causative-Have instruction was found in the supported attestation scan; no
invented effect is presented as an authentic positive witness. The independent
imperative diagnostic `Have target player mill two cards.` completely
enumerates 1 Reading with zero issues (`imperative-final-probe.json`); it is a
form/admission probe, not a supported-corpus attestation.

### Assurance, gate and declarations

Nine new public-seam tests pass, including the five attested trigger/effect
witnesses, finite *has*/*had*, a hand-constructed Ebon Dragon constituent, and
Complement-form diagnostics. The independent constituent checks both
roundtrip laws with exact values, construction/leaf traversal identity, and
rejection of swapped slots by admission and realization. Joraga Bard retains
both matrix and embedded duration attachments. Gerund/preterite diagnostic
strings already have possessive/perfect Readings; their exact sets are
compared against a lexicon with the original unsupported Have frame, proving
this replacement adds none. Plain-form and *to*-infinitival diagnostics retain
their full-root outcomes. No assertion is weakened to a discriminant check.

Existing tests: restored 0, re-spelled 0, ignored 0, added 0, removed 0.
This ticket's suite: added 9, with 1 newly authored witness re-spelled
(Mirror Image → Extractor Demon) under the explicit scope ruling; restored 0,
ignored 0, removed 0. The original red witness evidence is retained. The swap
is the only deliberate departure from the original five requested witnesses.

`cargo xtask gate --changed --from english-v3-causative-have --clippy --run`
derives the reverse-dependency closure of the lexical source, including its
construction reader and consumers:

```text
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

Both derived commands pass again after refresh: the complete test closure and
clippy with warnings denied. The causative suite reports 9 passed, 0 failed
and 0 ignored.
Formatting (`cargo fmt --all -- --check`), focused nightly rustfmt, and derive
order checks pass. The additional whole-repository nightly check finds inherited
drift on 170 unchanged paths (named in `inherited-nightly-format-drift-refreshed.json`);
none is changed by this ticket. Citation noncompliance has
zero entries; `cite check` reports 16,112 sites and **0 stale**. The piped
`jj diff --git --from english-v3-causative-have | cargo-xtask cite audit --diff`
audits 0 changed CR sites; this change adds CGEL citations only, so no bless is
needed. Final gate, clippy, formatting and citation logs are retained with the
census evidence.

The final declaration file is byte-identical to the refreshed claim parent:
**3,227 non-blank lines → 3,227, net 0**, on `vusnwtmuoytr` / covered 19,826.
The inherited over-ceiling file is not enlarged. The construction inventory
remains 204 ordinary Constructions plus 43 shared schemas (247 Reading
variants), 315 category instances and 612 static Productions. Features added:
**none**. Tables added: **none**. Policies added: **none**. Constructions added:
**none**. One lexical frame is replaced, no extra frame is appended.

No admission guard is added; forbidden word-naming guards added: zero.
Lexical loading succeeds with zero errors; the 87 unmapped annotations are
unchanged. Active v3 has no coverage lock or `environment.rs`, and its census
emits no legacy permitted-licensing-checker total; the covered counts here are
v3 census counts, not fabricated lock figures. There is no compiler or policy
change to add a licensing checker.

**Deviations and additions:** the nine tests add finite-form, independent-value
roundtrip and Complement-form checks beyond the five witness tests, because
they exercise the generic frame's form/category constraints. No existing test
or Construction is deleted. The Mirror Image witness swap and 60-face Enter
Complement deferral follow the explicit STOP resolution above. No plugin body,
xtask command, production fixture or plan-scoped verifier is added.
The glossary gap is resolved by adding Bare Infinitival, Catenative
Construction and Catenative Complement with their defining CGEL passages
(Ch. 14 §§1.1–1.2, pp. 1173–1178). These are linguistic definitions, not new
implementation features; no unresolved glossary gap remains.

### Inventory and performance advisory

On `vusnwtmuoytr` / covered 19,826, the generated-form homograph inventory is
unchanged against the refreshed base at 585 named surfaces (excluding card-name
catalogs and capitalization alternatives); `inventory-refreshed-base.json` and
`inventory-refreshed-final.json` retain every surface's lexical owners.
Trunk `swptuusp` removes the homograph surface `Attraction` (owners before:
`catalog:artifact-types.txt/Attraction`, `lexeme:artifact_subtype/attraction`),
changing the original inventory of 586 to 585 before this ticket's comparison.
This ticket changes no homograph surface or lexical owner. Word-bearing
form literals: **none**. Form-literal/vocabulary overlaps: **none**. The full
named homograph surface list follows the gained-identity list below.

| Census stamp / covered | Wall time (ns) | Checked-text thread CPU | Host load (1/5/15 min) | Workers |
|---|---:|---:|---|---:|
| Refreshed base `tlupskwqkwmv` / 19,682 | 127,004,870,086 | 260,914 ns/B | 1.39 / 4.51 / 9.15 | 6 |
| Final `vusnwtmuoytr` / 19,826 | 130,739,481,125 | 275,990 ns/B | 23.04 / 12.43 / 11.36 | 6 |

Both exceed the 16,260,000,000 ns quiet-host advisory. These loaded-host runs
include competing jobs (the final census also overlaps the test gate); they
do not establish quiet-host performance. Full command:

```text
target/debug/cargo-xtask english-v3 --all --workers 6 --samples-per-face 0 --output target/english-v3/refreshed-final-census.json
```

The direct workspace-built xtask binary avoids Cargo's build lock while the
gate runs; no grammar Rust code changed. The paired before command used the
same options with `refreshed-base-census.json`. Subset checks used
explicit identity manifests, six workers and no Reading limit.

### Newly covered identities and selected analysis

All rows are stamped `vusnwtmuoytr` / covered 19,826. Before counts are 0 for each;
the following column gives the completely enumerated after count. Every row's
inspected new Have node selects **Object NP + bare-infinitival catenative
Complement** through `SelectedPredicate` (Have frame 2), preserving other
attachments and existing frames in the surrounding document. A sampled tree
per identity is in `gains-spot-checks.json`; no negative oracle is counted as a
gain.

| Identity | Face | Readings |
|---|---|---:|
| `008c92bc-3022-4d34-aa04-584576ff8f8e#card` | Deem Worthy | 1 |
| `073551ca-2e84-4d49-8484-9890718cc483#card` | Pandemonium | 6 |
| `08528c5e-9a01-42a4-a00b-c1ec5eaae64d#card` | Renegade Doppelganger | 7 |
| `0c0d6c7e-a7cc-49e0-bf84-3fc33dd10b90#card` | Deepfathom Echo | 15 |
| `0d063c0e-66bc-4a9c-98aa-a381b9cea120#card` | Farrel's Zealot | 1 |
| `113c3024-c3bd-4c3a-8c08-3828646d9ca2#card` | Forerunner of the Empire | 80 |
| `11cd0c42-4148-4d82-9172-b1a1967fe319#card` | Browbeat | 1 |
| `1253de62-6218-401c-9d98-a5ec8617e0d8#card` | Cosi's Ravager | 2 |
| `12c78cc4-7fbe-4639-bef7-bf959a0d6d68#card` | Distant Memories | 7 |
| `12e4d2bd-83e9-4120-a2e8-0645c0ed2387#card` | Ebon Dragon | 1 |
| `135e6545-4ec5-4136-9019-3fda92052b7a#card` | Lys Alana Bowmaster | 12 |
| `154104b7-4125-4276-8bc7-9a6edfe48cb9#card` | Ob Nixilis, the Fallen | 10 |
| `15e83068-6253-4c65-8679-7295f3dc2075#card` | Aether Charge | 1 |
| `16069935-4aeb-481f-98f2-46387775540f#card` | Noggle Hedge-Mage | 1 |
| `17b29350-4f37-4552-8192-4856b15345f9#card` | Smuggler's Share | 1,889 |
| `18c58328-518f-4228-bb1b-2433f3fb21f2#card` | Deceiver of Form | 28 |
| `1a02cb6d-965a-4eeb-b126-8e153d4832a2#card` | Snapping Thragg | 3 |
| `1a304703-6b48-4f3f-bd41-7f10ce0a751c#card` | Wandering Troubadour | 5 |
| `1bfdbb96-cc18-4609-ba17-d169967df8d0#card` | Geistcatcher's Rig | 6 |
| `1d6925f3-d132-4fa1-bde0-82d0128c319f#card` | Murasa Pyromancer | 23 |
| `1fb5c014-9366-4f9e-9728-303ff766002b#card` | Extractor Demon | 1 |
| `2071b712-2539-4797-8359-f9adb31889ae#card` | Hazoret's Favor | 66 |
| `2160c837-342b-409f-a1db-0ec5ac023ccf#card` | Vrondiss, Rage of Ancients | 12 |
| `2227bab2-36a0-47f1-a797-7b82323eca9d#card` | Death Pulse | 10 |
| `2d785e3e-8e19-4eea-b09f-9a30c792dba1#card` | Belligerent Yearling | 6 |
| `3260bede-d78d-41a6-8fe1-95ce14a2ba8c#card` | Lava Blister | 2 |
| `3494a575-f68e-4e78-9f6b-142cd8a0edea#card` | Combustible Gearhulk | 28 |
| `34fd52c1-3d81-4005-b826-eeefbd7fd874#card` | Grothama, All-Devouring | 85 |
| `35553c2a-0150-49e4-81da-f356c6a17253#card` | Sanguine Statuette | 45 |
| `364e8bd6-52e5-4397-8d2a-10c4706e5a45#card` | Hag Hedge-Mage | 1 |
| `3708eaae-9306-46b2-ae41-7ce6c50be3c7#card` | Mysteries of the Deep | 30 |
| `3b9bc41a-a43f-4be7-9452-f61f5ad5b482#card` | Sita Varma, Masked Racer | 228 |
| `3bd57758-00a6-4d49-b1fc-b14f8203da0b#card` | Commando Raid | 18 |
| `40c3007a-be6c-4400-9139-12b9ca098ac4#card` | Shyft | 1 |
| `40f5f8b9-5c2c-41d4-8b83-55a2e51ced64#card` | Exuberant Wolfbear | 98 |
| `4107d0e3-608b-4024-b433-40c5d8b63549#card` | Zealot il-Vec | 6 |
| `41087db6-34c4-4e2b-9f54-5e4488ca9c0b#card` | Blood Seeker | 1 |
| `41353001-eba4-45b5-8dd2-09d49b63b6b1#card` | Tunnel Ignus | 5 |
| `4489f362-9b7e-4dc0-a459-a990a487454e#card` | Absorb Identity | 12 |
| `463fc961-d34e-4f40-b383-5b78a0fcb5c8#card` | Slice and Dice | 1 |
| `4b3bc59c-5439-4ec8-b25e-7493fa1cd3fd#card` | Tomb Hex | 120 |
| `4ca5d2c7-621b-4d5e-9bf4-ff3ae271ba44#card` | Kinsbaile Balloonist | 5 |
| `4e1efd99-8bfc-4650-a430-a754578facbd#card` | Hollowsage | 1 |
| `4ec0b64d-c793-47a6-9c26-9ad4d9e9c182#card` | Lazav, Wearer of Faces | 15,696 |
| `5212f0a8-7341-4960-835b-2f4600ca1b2d#card` | Anointed Deacon | 5 |
| `533b107c-fd66-45ef-b74e-e6bd9b75ea8f#card` | Combustion Man | 9 |
| `575f032a-6ae8-4f4f-ab0b-983e7c76ad7f#card` | Glint Hawk Idol | 945 |
| `593b229b-579b-4052-8a00-dac787b149b1#card` | Quill-Slinger Boggart | 1 |
| `5967c3d3-2ef1-4496-b51c-23478fa77f09#card` | Dwarven Driller | 2 |
| `5c5d1a62-3b83-4812-82bb-d95b39b990b4#card` | Faerie Tauntings | 2 |
| `5cdc4654-b8c5-4599-a1d2-9e9d5f6509cb#card` | Duelist's Heritage | 5 |
| `5d258409-28d3-4eac-9f1a-3e494db818bf#card` | Cloak of Confusion | 9 |
| `5e2c1e0e-0a10-416a-9b50-96ee0cbbc24e#card` | Hagra Diabolist | 5 |
| `5ef498fa-31d5-4eca-873d-6b2b1f7f9a3c#card` | Tajuru Archer | 28 |
| `633849f6-fad7-49d7-8bf5-2a3a4510965f#card` | Highland Berserker | 10 |
| `644a8946-4890-4c2b-978f-02eedc95b5ad#card` | Book Burning | 1 |
| `669830ea-f55c-447a-aa3a-8965fa6d2a33#card` | Rage Forger | 15 |
| `68d11f97-da97-4738-bb3e-e6730bd014e8#card` | Death Match | 5 |
| `72dc807b-35fb-4d69-ae4c-04eed0646170#card` | Battle-Rattle Shaman | 5 |
| `731898d3-465b-4ecb-a346-b91881714ebc#card` | Saddled Rimestag | 14 |
| `73c44cf5-a601-4625-9550-e722fba2ec0b#card` | Tanazir Quandrix | 1,064 |
| `73f6619c-9d92-434e-9f06-52e0ff15cb5e#card` | Reptilian Reflection | 313 |
| `745efa76-7063-4971-9d1b-b2b4cb336a57#card` | Sin Prodder | 18 |
| `74b08a70-b0bb-4340-98a0-b1d5b7c9d2cc#card` | Searing Blaze | 60 |
| `7545adab-b6f8-496d-8c25-416c7ec85fa2#card` | Dire Undercurrents | 4 |
| `75f2545d-cd31-4404-953f-1110f646228b#card` | Elektra, Femme Fatale | 2 |
| `7661dda6-4e7b-42d0-8374-0a1fe65b83e4#card` | Kyren Sniper | 1 |
| `7ab18e48-f935-405b-8658-b49052d20529#card` | Clash of Realities | 1 |
| `7b3e05e1-31b4-46af-b3d0-f2467f8551e9#card` | Goblin Razerunners | 17 |
| `7c4fc524-4734-4fb8-a5ca-ac2617242c3f#card` | Hissing Iguanar | 1 |
| `7d9b7fca-9514-4aea-be51-b6c8debb3de4#card` | Skophos Maze-Warden | 18 |
| `7f61a2a6-2fe2-48ec-8e2e-74d147be0a0a#card` | Risk Factor | 1 |
| `822eb600-cfaf-4ea1-a016-0c1f0cfa6436#card` | Dreamspoiler Witches | 10 |
| `84a3ebc2-5c38-40ef-b5f4-62a92cbca493#card` | Jund Sojourners | 1 |
| `877c6bb4-487c-4eea-b4af-f28fe1515fb6#card` | Seascape Aerialist | 10 |
| `87a44a87-a2a5-4193-a163-e4846e5b8387#card` | Nath of the Gilt-Leaf | 5 |
| `87adda41-113b-4126-8978-25a258b31bdd#card` | Iroh, Tea Master | 20 |
| `88aa032d-dc11-4cae-81f3-ce66353963e0#card` | Affectionate Indrik | 1 |
| `8b3cec74-0d32-4762-b3e0-3f4e5eb6f179#card` | Ruination Rioter | 77 |
| `902e7d4b-7ab4-40b5-b3a5-c83ef604989a#card` | Primal Boost | 10 |
| `91c83b1e-32d9-4bb4-9a71-05fbb0cb8134#card` | Caustic Crawler | 10 |
| `942eefa3-6775-428b-84aa-48c3bd78d3b8#card` | Thorntooth Witch | 10 |
| `94d9878e-9786-438c-8b33-c1dff730b748#card` | Decoy Gambit | 44 |
| `9707ac4f-6493-47fe-9c31-4daf469b5c6e#card` | Vicious Shadows | 102 |
| `98914c0e-967f-4c8b-bd74-e2ce15c7fe95#card` | Talus Paladin | 9 |
| `997d2f38-5a50-451e-bd48-e36dcb24d967#card` | Boggart Shenanigans | 3 |
| `9b0abb58-4e2c-45dc-99b1-61a4cb79354b#card` | The Fantasticar | 252 |
| `9d2ad65f-fba0-45e9-a5e6-bec63a9db979#card` | Predatory Nightstalker | 1 |
| `9e00188a-0743-4a39-9229-977a0a115659#card` | Iceberg Cancrix | 2 |
| `9e9164be-3534-4445-bfad-00d16a50575c#card` | Workshop Elders | 4 |
| `a0b16b11-69d9-4e94-aae3-298aa44ce3d4#card` | Munitions Expert | 46 |
| `a4003cd1-e98c-4346-b685-89c7891f9920#card` | Brutal Nightstalker | 1 |
| `a648b9ed-810a-41db-af40-3b5b4658db90#card` | Breaking Point | 1 |
| `a7eef84a-0b56-41d5-a298-dc3cf199e125#card` | Joraga Bard | 10 |
| `a896cedb-f10a-48ac-82e7-6ae1e8b5a439#card` | Dwarven Scorcher | 2 |
| `aed10a6c-ef93-46a7-a5d7-c9cd2a4fd91a#card` | Atzocan Archer | 1 |
| `b1b67851-23d3-4799-ab29-15bce7bb2c39#card` | Somberwald Stag | 1 |
| `b3a479d4-3e36-496b-abf8-b83231628d76#card` | Gaze of Pain | 3 |
| `b40e2aae-3a1d-4bc2-bf67-e821ec800434#card` | Blightcaster | 10 |
| `b49baf80-237c-41ee-8a22-2b6a68a313e7#card` | Dwarven Vigilantes | 3 |
| `b52788ff-9956-400a-bf26-17e58f673af0#card` | Skullscorch | 3 |
| `b582e485-c024-47f0-ad6f-b918d32288ba#card` | Vexing Devil | 1 |
| `bb7c069b-8d3f-4ba2-874b-8ec575fd6dd4#card` | Jace's Erasure | 1 |
| `bd532f8d-789f-4ee1-a2f3-ae30e8282c76#card` | Gempalm Polluter | 15 |
| `be7b16ef-32aa-40d5-b287-c5e79d52d6b9#card` | Dirge of Dread | 10 |
| `c08e0958-8fa2-49c7-ab17-a70dfc1ef33a#card` | Terrapact Intimidator | 1 |
| `c1177f22-a1cf-4da3-a68d-ff954e878403#card` | Goblin Arsonist | 1 |
| `c3b6e114-7bc8-4890-97d9-edbeea1f425f#card` | Assault Suit | 14 |
| `c4d36522-3ace-4bfb-bf2d-1a366f458698#card` | Suture Priest | 1 |
| `c6ca1f21-3aaf-4a5e-938e-7daed67a9d6e#card` | Longhorn Firebeast | 1 |
| `c6d39445-dfc2-42c3-afd2-016fdf008f0e#card` | Slavering Nulls | 3 |
| `c7f08962-6c54-4fd5-a55e-7513e758af32#card` | Rest for the Weary | 30 |
| `c8625113-0ce4-4454-83a1-25c31b8bfb9a#card` | Disciple of the Vault | 3 |
| `c92f1d40-0441-4ede-9363-78821ff15f88#card` | Skirk Commando | 3 |
| `ca5711d1-8cc9-4540-a567-778b56d51f50#card` | Cyclops Gladiator | 18 |
| `cba6563e-96e5-4cd3-bddf-0cf1368f9e15#card` | Cartouche of Strength | 1 |
| `cc261487-060f-47e6-ac5e-a204254058b3#card` | Riddleform | 626 |
| `cddb3ec4-01a2-4f4d-a690-3486c73c3df8#card` | Eldrazi Mimic | 28 |
| `ce8fc1bb-5dd0-41b1-909a-bbfe6b165158#card` | Ronin Cliffrider | 1 |
| `d17a1109-3a7f-4a39-b602-155da819235f#card` | Bellowing Elk | 5 |
| `d955a26a-d474-4dff-9eba-69fcfe92d8ae#card` | Molten Influence | 4 |
| `da18d97c-85f3-4395-9af5-bca72e33ebfc#card` | Fourth Bridge Prowler | 5 |
| `da3b12d1-8867-4063-b50d-3b2f5ce0549f#card` | Dancing Sword | 9 |
| `dda70e25-b3c5-444d-9dff-1366771bf881#card` | Wicked Guardian | 1 |
| `ddbacb74-1f98-4607-a92e-d14973b9d0ef#card` | Groundswell | 120 |
| `de2c7724-647c-4ca2-be4d-f0e348a59262#card` | Reclusive Artificer | 23 |
| `de3ed52e-7cc2-49ea-96d9-4254aac46168#card` | Angel's Tomb | 55 |
| `e2bfac84-0e2f-4f3e-8d87-a555c26093a6#card` | Undead Executioner | 5 |
| `e6681d22-821a-47c8-9c0d-042a7d44f843#card` | Blazing Salvo | 2 |
| `eb448589-28e1-49d4-ba3e-7ddb4341fdc5#card` | Akoum Battlesinger | 10 |
| `ece3e379-c586-48bd-8742-472b5deccc35#card` | Solar Blast | 1 |
| `ee45b588-b424-49d3-9e37-270f2ae9490d#card` | Razormane Masticore | 3 |
| `f0fd78bb-cb83-4dfa-a910-0334d225f87c#card` | Attack-in-the-Box | 15 |
| `f427a1ed-d7ca-40e5-a759-a1096bad166d#card` | Spark Mage | 3 |
| `f86b9a01-62fc-4120-8914-5054ed48d37e#card` | Confounding Conundrum | 10 |
| `f8c0c555-c750-4842-8924-d634206bbb81#card` | Exuberant Firestoker | 2 |
| `f912c739-27a2-4068-bf0c-0891309639df#card` | Flameblade Angel | 4 |
| `fa716557-8e75-4617-8310-ab23a9ca1af1#face:0` | Slicer, Hired Muscle | 288 |
| `fab8c985-d7a9-41eb-9884-01128b97af5c#card` | Flaming Gambit | 36 |
| `fbdbea75-7f80-4704-b2cf-920d00dc1273#card` | Painsmith | 44 |
| `fc7b46af-6c07-455a-99c7-f1bccaa2a5f4#card` | Kederekt Parasite | 1 |
| `fca2fcab-4f17-448d-bf6d-f6c913159df8#card` | Where Ancients Tread | 2 |
| `fcca3c9d-a2b6-4f29-b43c-2f061074a332#card` | Scalding Salamander | 1 |
| `fd92b588-f273-4206-9887-34e6877fc14f#card` | Park Heights Pegasus | 57 |

### Named homograph surfaces

Unchanged refreshed-base/final inventory, final `vusnwtmuoytr` / covered 19,826;
lexical owners are listed in the paired ignored inventory artifacts.

`'s`; `Adamant`; `Addendum`; `Adventure`; `Advisor`; `Aetherborn`; `Ajani`; `Alien`; `Alliance`;
`Ally`; `Aminatou`; `Angel`; `Angrath`; `Antelope`; `Ape`; `Arcane`; `Archer`; `Archon`; `Arlinn`;
`Armadillo`; `Army`; `Artificer`; `Ashiok`; `Assassin`; `Assembly-Worker`; `Astartes`; `Atog`;
`Aura`; `Aurochs`; `Avatar`; `Azra`; `Background`; `Badger`; `Bahamut`; `Balloon`; `Barbarian`;
`Bard`; `Basilisk`; `Basri`; `Bat`; `Battalion`; `Bear`; `Beast`; `Beaver`; `Beeble`; `Beholder`;
`Berserker`; `Bird`; `Bison`; `Blinkmoth`; `Blood`; `Bloodrush`; `Boar`; `Bobblehead`; `Bolas`;
`Book`; `Bringer`; `Brushwagg`; `C'tan`; `Calix`; `Camarid`; `Camel`; `Capybara`; `Caribou`;
`Carrier`; `Cartouche`; `Case`; `Cat`; `Cave`; `Celebration`; `Centaur`; `Chandra`; `Channel`;
`Child`; `Chimera`; `Chroma`; `Citizen`; `Class`; `Cleric`; `Clown`; `Clue`; `Cockatrice`; `Cohort`;
`Comet`; `Constellation`; `Construct`; `Contraption`; `Converge`; `Council's dilemma`; `Coven`;
`Coward`; `Coyote`; `Crab`; `Crocodile`; `Curse`; `Custodes`; `Cyberman`; `Cyclops`; `Dack`;
`Dakkon`; `Dalek`; `Daretti`; `Dauthi`; `Davriel`; `Delirium`; `Dellian`; `Demigod`; `Demon`;
`Descend 4`; `Descend 8`; `Desert`; `Deserter`; `Detective`; `Devil`; `Dihada`; `Dinosaur`;
`Disappear`; `Djinn`; `Doctor`; `Dog`; `Domain`; `Domri`; `Dovin`; `Dragon`; `Drake`; `Dreadnought`;
`Drix`; `Drone`; `Druid`; `Dryad`; `Dwarf`; `Echidna`; `Eerie`; `Efreet`; `Egg`; `Elder`; `Eldrazi`;
`Elemental`; `Elephant`; `Elf`; `Elk`; `Ellywick`; `Elminster`; `Elspeth`; `Eminence`; `Employee`;
`Enrage`; `Equipment`; `Estrid`; `Eternal`; `Eye`; `Faerie`; `Fateful hour`; `Fathomless descent`;
`Ferocious`; `Ferret`; `Fish`; `Flagbearer`; `Flurry`; `Food`; `Forest`; `Formidable`;
`Fortification`; `Fox`; `Fractal`; `Freyalise`; `Frog`; `Fungus`; `Gamer`; `Gamma`; `Gargoyle`;
`Garruk`; `Gate`; `Germ`; `Giant`; `Gideon`; `Giraffe`; `Gith`; `Glimmer`; `Gnoll`; `Gnome`; `Goat`;
`Goblin`; `God`; `Gold`; `Golem`; `Gorgon`; `Grandeur`; `Graveborn`; `Gremlin`; `Griffin`; `Grist`;
`Guest`; `Guff`; `Hag`; `Halfling`; `Hamster`; `Harpy`; `Hedgehog`; `Hellbent`; `Hellion`; `Hero`;
`Heroic`; `Hippo`; `Hippogriff`; `Homarid`; `Homunculus`; `Horror`; `Horse`; `Huatli`; `Human`;
`Hydra`; `Hyena`; `I`; `Illusion`; `Imp`; `Imprint`; `Incarnation`; `Incubator`; `Infinity`;
`Infusion`; `Inhuman`; `Inkling`; `Inquisitor`; `Insect`; `Inspired`; `Island`; `Jace`; `Jackal`;
`Jared`; `Jaya`; `Jellyfish`; `Jeska`; `Join forces`; `Juggernaut`; `Junk`; `Kaito`; `Kangaroo`;
`Karn`; `Kasmina`; `Kavu`; `Kaya`; `Kinship`; `Kiora`; `Kirin`; `Kithkin`; `Knight`; `Kobold`;
`Kor`; `Koth`; `Kraken`; `Kree`; `Lair`; `Lamia`; `Lammasu`; `Lander`; `Landfall`; `Leech`; `Lemur`;
`Lesson`; `Leviathan`; `Lhurgoyf`; `Licid`; `Lieutenant`; `Liliana`; `Lizard`; `Llama`; `Lobster`;
`Locus`; `Lolth`; `Lukka`; `Magecraft`; `Manticore`; `Map`; `Masticore`; `Mercenary`; `Merfolk`;
`Metalcraft`; `Metathran`; `Mine`; `Minion`; `Minotaur`; `Minsc`; `Mite`; `Mole`; `Monger`;
`Mongoose`; `Monk`; `Monkey`; `Moogle`; `Moonfolk`; `Morbid`; `Mordenkainen`; `More Than Meets the
Eye`; `Mount`; `Mountain`; `Mouse`; `Mutagen`; `Mutant`; `Myr`; `Mystic`; `Nahiri`; `Narset`;
`Nautilus`; `Necron`; `Nephilim`; `Nightmare`; `Nightstalker`; `Niko`; `Ninja`; `Nissa`; `Nixilis`;
`Noble`; `Noggle`; `Nomad`; `Nymph`; `Octopus`; `Ogre`; `Oko`; `Omen`; `Ooze`; `Opus`; `Orb`; `Orc`;
`Orgg`; `Otter`; `Ouphe`; `Ox`; `Oyster`; `Pack tactics`; `Pangolin`; `Paradox`; `Parley`;
`Peasant`; `Pegasus`; `Pentavite`; `Performer`; `Pest`; `Phelddagrif`; `Phoenix`; `Phyrexian`;
`Pilot`; `Pincher`; `Pirate`; `Plains`; `Plan`; `Planet`; `Plant`; `Platypus`; `Porcupine`;
`Possum`; `Power-Plant`; `Powerstone`; `Praetor`; `Primarch`; `Prism`; `Processor`; `Qu`;
`Quintorius`; `Rabbit`; `Raccoon`; `Radiance`; `Raid`; `Ral`; `Rally`; `Ranger`; `Rat`; `Rebel`;
`Reflection`; `Renew`; `Repartee`; `Revolt`; `Rhino`; `Rigger`; `Robot`; `Rogue`; `Role`; `Room`;
`Rowan`; `Rune`; `Sable`; `Saga`; `Saheeli`; `Salamander`; `Samurai`; `Samut`; `Sand`; `Saproling`;
`Sarkhan`; `Satyr`; `Scarecrow`; `Scientist`; `Scion`; `Scorpion`; `Scout`; `Sculpture`; `Seal`;
`Secret council`; `Serf`; `Serpent`; `Serra`; `Servo`; `Shade`; `Shaman`; `Shapeshifter`; `Shard`;
`Shark`; `Sheep`; `Shi'ar`; `Shrine`; `Siege`; `Siren`; `Sivitri`; `Skeleton`; `Skrull`; `Skunk`;
`Slith`; `Sliver`; `Sloth`; `Slug`; `Snail`; `Snake`; `Soldier`; `Soltari`; `Sorcerer`; `Sorin`;
`Spacecraft`; `Spawn`; `Specter`; `Spell mastery`; `Spellshaper`; `Sphere`; `Sphinx`; `Spider`;
`Spike`; `Spirit`; `Splinter`; `Sponge`; `Spy`; `Squid`; `Squirrel`; `Starfish`; `Stone`; `Strive`;
`Surrakar`; `Survival`; `Survivor`; `Swamp`; `Sweep`; `Symbiote`; `Synth`; `Szat`; `Tamiyo`;
`Tasha`; `Teferi`; `Tempting offer`; `Tentacle`; `Tetravite`; `Teyo`; `Tezzeret`; `Thalakos`;
`Thopter`; `Threshold`; `Thrull`; `Tibalt`; `Tiefling`; `Time Lord`; `Tower`; `Town`; `Toy`; `Trap`;
`Treasure`; `Treefolk`; `Trilobite`; `Triskelavite`; `Troll`; `Turtle`; `Tyranid`; `Tyvar`; `Ugin`;
`Undergrowth`; `Unicorn`; `Urza`; `Urza's`; `Utrom`; `Valiant`; `Vampire`; `Varmint`; `Vedalken`;
`Vehicle`; `Venser`; `Vibranium`; `Villain`; `Vivid`; `Vivien`; `Void`; `Volver`; `Vraska`;
`Vronos`; `Wall`; `Walrus`; `Warlock`; `Warrior`; `Weasel`; `Weird`; `Werewolf`; `Whale`; `Will`;
`Will of the council`; `Windgrace`; `Wizard`; `Wolf`; `Wolverine`; `Wombat`; `Worm`; `Wraith`;
`Wrenn`; `Wurm`; `X`; `Xenagos`; `Yanggu`; `Yanling`; `Yeti`; `Zariel`; `Zombie`; `Zubera`; `as`;
`bottom`; `control`; `copies`; `copy`; `cost`; `costs`; `counter`; `counters`; `cycling`;
`deathtouch`; `decayed`; `die`; `double strike`; `draw`; `draws`; `exalted`; `exile`; `exiles`;
`first strike`; `flying`; `goaded`; `harnessed`; `haste`; `her`; `hexproof`; `his`; `if`;
`indestructible`; `it`; `legendary`; `less`; `lifelink`; `menace`; `name`; `names`; `one`; `reach`;
`shadow`; `solved`; `suspected`; `tapped`; `target`; `targets`; `that`; `then`; `time`; `to`; `top`;
`trample`; `turn`; `turns`; `untap`; `untapped`; `up`; `vigilance`; `you`; `’s`; `∞`.
