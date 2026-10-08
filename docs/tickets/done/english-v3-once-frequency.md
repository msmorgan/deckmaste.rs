---
needs: [english-v3-focusing-only]
---
# Read bounding frequency *once* with a distributive NP: only once each turn

## Why

*Once each turn* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), *only once each turn*
touches **281** unread faces and is the sole cause on **80**; *once* without
*only* occurs on 11 (recon bucket "activation/timing restriction", read for
this string). Counts are unread faces *touched* (at least one localised failing
unit matches) / *sole* (every failing unit matches and no other recon STRONG
bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "Draw a card each turn." 1; "Draw a card once." 0;
"Draw a card once each turn." 0; "Draw a card twice." 0; "Activate only once." 0.
The *each turn* NP Adjunct reads; *once* does not.

## Goal

*once* (and *twice*) read as a bounding frequency Adjunct, optionally followed
by a distributive temporal NP (*once each turn*), clause-finally. Combined with
`english-v3-focusing-only` (the `needs:` edge), *Activate only once each turn.*
and *Do this only once each turn.* read with *only* focusing the frequency
Adjunct.

## Analysis

The bounding frequency expressions are the adverbs *once* and *twice*, NPs
headed by *times*, and *on* + *occasion* PPs (CGEL, Ch. 8, §9, p. 715). *Once*
can also express temporal location (*I once liked this kind of music*; p. 715,
[8ib]); only the frequency use is attested in these witnesses, so the
temporal-location use is not added (pruning rule). How *each turn* combines
with *once* (a separate NP Adjunct or a dependent of *once*) is for the landing
to settle and record.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket except the restriction bucket (`xxknlzypsnwy`):

- Kozilek's Translator: "Activate only once each turn."
- Jeskai Devotee: "Activate only once each turn."
- Basking Rootwalla: "Activate only once each turn."
- Twinblade Slasher: "Activate only once each turn."
- Ondu Spiritdancer: "Do this only once each turn."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-once-frequency-before.json` on the
   claim parent, stamped with its change id and covered count; after: the same
   command to `target/english-v3/english-v3-once-frequency-after.json` on the
   final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *once* read as a temporal-location Adjunct or as a subordinator
   (*once X happens*), or *each turn* read as the Object, is a defect. A wrong
   analysis that parses is a defect, not a gain.
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
12. Tests: `cargo test -p <crate>` and clippy for every crate changed; Lean checks only if Lean sources or plugin definitions changed.

## Out of scope

- Subordinator *once* with a clause, *the first time each turn*, *N times*,
  *twice that many*.
- *Activate only once each turn and only if …*: coordination of the two
  restrictions is ordinary coordination; note it, do not build it.

## Landing record

Completed at `qsrqtwknwxqtrpyrmknwollqqrrnssyo` / covered **21,916**: **215 newly covered faces**,
**zero lost faces**, all nine exposed faces restored to their claim-parent counts
and Reading digests. Four other faces retire **32 inspected wrong Readings**
under the orchestrator's RelativeUse ruling; no valid Reading was removed.
The first full census is void; the authorized replacement is the final census.

### Analysis and implementation

Once and Twice keep their existing lexical identities and Adverb category;
`VPFinalAdjunct = Yes` admits their bounding-frequency use as an Adverb Phrase
in an `AdverbPredicate`, and `ClauseInitialAdjunct = No` leaves temporal-location
and subordinator uses outside this landing. CGEL Ch. 8 §9, pp. 714–715 supports
the bounding-frequency classification and attests both adverbs; it also describes
how combination with an interval makes the overall frequency non-bounding.

The ticket expressly leaves constituency of *once each turn* to this landing.
The project analysis is **two separate Adjuncts**: the frequency Adverb Phrase
and the existing distributive temporal NP. Only modifies the frequency Adverb
Phrase; *each turn* is a `NominalAdjunctPhrase` on the predicate, never an Object
or a dependent inside the Adverb Phrase. This is this landing's project decision,
not a claim that CGEL prescribes that exact constituency. CGEL Ch. 6 §7.3.1,
p. 587 [42ii,iv] supports restrictive focusing modification of AdvPs and PPs.
Twice is attested by Nadu, Winged Wisdom (*only twice each turn*) and Agent Frank
Horrigan (*proliferate twice*); both Once and Twice are enabled.

`FocusedPhrase` is a shared PP/Adverb Phrase schema. It replaces, rather than
parallels, `FocusedPrepositionPhrase`; the PP tests retain their exact expected
structure with the replacement constructor. The original Once/Twice owners are
reused on the lexicon side; no alternative FrequencyPhrase or word-specific
frequency construction is installed. The refreshed trunk change
`sqzukyvnqplmsyxxrprltlkztpwtzkrs` supplies the no-self-stacking `FocusedUse`
permission, retained for PPs and generalized to AdvPs. The existing stacked-PP
negative and the new *only only once* negative pass.

Ondu Spiritdancer additionally needs independent host ingredients: a transitive
Predicate/Object frame for Do and a Fused Determiner-Head NP for This. CGEL Ch. 5
§§9.1–9.2, pp. 410–415, particularly §9.2(c), supports singular demonstrative
fused heads. The declared `FusedHeadUse` permission represents that distribution;
This remains a Determinative, not a Pronoun. Do's existing pro-verb and auxiliary
frames retain their indices; the new Object frame is appended. The unfocused
Remorseless Punishment witness needs native Repeat/Object and count-noun Process
entries; both are attested in *Repeat this process once.* No plugin body,
verbs.ron or Lean source changed.

### Structural proof and assurance

Every counted census Reading passes declaration admission, lexical ownership
and context, byte-exact realization, and full node/Word traversal comparison
against materialization traces. The census does not establish independent
linguistic correctness. The new independently constructed frequency-focus tests
assert full value equality after realization and parse-back, exact node traversal
and lexical leaf identity for both adverbs, and rejection of invalid heads and
modifiers. The independent fused-head and transitive Do test likewise compares
complete constructed values. Existing roundtrip tests continue to pass.

All grammatical Readings remain retained. Unique and multiple counts below
replace the retired destructive-selection/specificity census; no ordering or
tie elimination is introduced. Census duplicates, cyclic derivations, internal
failures, failed or limited enumerations, and validation issues are zero.
The guards use declared `FocusingModifierUse`, `FocusedUse`, `VPFinalAdjunct`,
`FusedHeadUse`, `DeterminerUse`, `RelativeUse` and existing PP permissions. **Zero new word-naming
guards**; environment load-error tests pass. V3 emits neither a coverage lock
nor legacy permitted/forbidden licensing-checker totals: these are **not
applicable**, rather than invented zeroes. The derived gate is retired.

Assurance relative to the claim parent: restored **0**, re-spelled **2**, added
**11**, ignored **0**, removed **0** test functions. The re-spelled tests are
`focusing_only::focus_does_not_supply_missing_constituents_or_other_functions`
(the former Once rejection becomes a structural positive, all other negatives
remain) and
`focusing_only::independently_constructed_focus_preserves_structure_and_lexical_identity`
(the replacement schema retains complete value comparisons). During iteration,
the unchanged auxiliary-ellipsis roundtrip test exposed an accidentally shifted
Do frame index. The fix appends the new frame in the data; no test was weakened
or changed to hide it. This intermediate repair is not a restored test relative
to the claim parent.

### Deviations and additions

- Generalize the PP focusing construction into one shared PP/AdvP schema, retire
  its superseded constructor, and re-spell its existing tests; no duplicate route.
- Add the Fused Determiner-Head NP construction and lexical This permission,
  plus the transitive Do frame: required to satisfy the named Ondu witness.
- Add the attested Repeat/Object and Process entries for the unfocused witness,
  beyond the five mandatory witnesses; this tests frequency without Only.
- Add independent construction, negative placement/constituency, Twice and
  host-structure tests beyond the five mandatory witness tests. No construction
  or test subject is removed beyond the replaced PP focusing constructor.
- Generalize the refreshed trunk `FocusedUse` guard to AdvPs while preserving
  the PP ruling and its regression test.
- Apply the ruled RelativeUse permission to ordinary NominativePronoun admission
  and add the exact single-relative regression test; the orchestrator expressly
  authorizes this shared-boundary repair beyond the original ticket.
- Resolve glossary gaps Fused Determiner-Head, Fused Head Use and Relative Use; extend the
  project Focused Use definition to AdvPs with explicit project attribution.
  No new CR citations. CGEL citations support only the passages described above.

### STOP and resolution

The relative-boundary STOP is resolved by the orchestrator ruling of 2026-10-08:
“apply your proposed-relative-boundary.patch as is” and “the earlier census is
void.” The patch is applied exactly: RelativeUse has a No default and ordinary
NominativePronoun admission requires RelativeUse No. This is the generic lexical
head constraint within the ordinary nominative CasePhrase projection. A relative
pronoun is a marker of its relative construction, never the head of an ordinary
nominative pronoun phrase. The new regression test independently constructs the
complete expected subject relative with auxiliary Do and its elided complement,
and asserts that *each player who does* has exactly one NP Reading. Both the
scope expansion and one replacement full census are expressly authorized.
The earlier frame-index regression and ordinary refresh conflict were likewise
resolved before the replacement full census. *Once/twice that many*, subordinator Once,
*the first time each turn* and *N times* are not implemented. Coordination of
*only once each turn and only if …* remains an ordinary coordination obligation
for `english-v3-systemic-residuals`; its absence is not evidence against either
frequency Adjunct's category.

### Validation

- `cargo test -p deckmaste_english_v3 -p deckmaste_lexical_source`: passed,
  **394 English integration tests** and **56 lexical-source tests** (23 unit,
  33 integration), zero failed or ignored (`tests-repair.log`).
- `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`:
  passed (`clippy-repair.log`).
- `cargo +nightly fmt -p deckmaste_english_v3 -p deckmaste_lexical_source -- --check`:
  passed (`fmt-final.log`).
- `cargo xtask cite check --list-noncompliant`, `cargo xtask cite check`, and
  the actual feature `jj diff --git` piped to `cargo xtask cite audit --diff`:
  **0** noncompliant, **0** stale, **0** changed CR sites; no bless needed
  (`cite-list-final.log`, `cite-check-final.log`, `cite-audit-final.log`).
- One authorized replacement full, non-incremental feature census:
  `cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-once-frequency/after.json`.
  Iteration and Reading inspection use selectors only. No xtask tests, Lean
  checks or derived gate were run.

### Final census and no-silent-loss proof

All following final figures are measured at `qsrqtwknwxqtrpyrmknwollqqrrnssyo` / covered **21,916**. The
claim-parent-equivalent baseline is `urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` / covered
**21,701**, original claim parent
`vozmwtpxlwsypukuktrunnuwsppmkrlu`; `baseline-provenance.json` and
`parent-equivalence-proof.json` document source equivalence. Refresh imports
`sqzukyvnqplmsyxxrprltlkztpwtzkrs` (focusing self-stacking) and unrelated
Lean facts work `tztouvwnytpvunusopwzpzmpxyxvqpln` plus comment-only plugin
work `slktvwupsopvqvprvtppqsmotwqppwrz`. The supported input hash remains
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`. Neither refresh imports a Reading decrease.

| Census | Claim parent | Finished feature |
| --- | ---: | ---: |
| Supported faces | 32,828 | 32,828 |
| No Reading | 11,127 | 10,912 |
| One Reading | 7,472 | 7,516 |
| Multiple Readings | 14,229 | 14,400 |
| Exact Readings | 782,215 | 943,065 |

Covered **21,701 → 21,916**: **215 new faces**,
**zero lost faces**, **four named Reading-count decreases**, **zero count increases
on previously covered faces**. The decreases retire 32 wrong analyses under the
RelativeUse ruling; every retained value on those four faces is unchanged.
No grammatical Reading or face coverage is lost. `comparison.json` compares every supported face by identity,
source and exact Reading count. The nine exposed faces return to the parent
counts and complete Reading digests.

| Exposed face | Parent = repaired | Void attempt |
| --- | ---: | ---: |
| Tempt with Immortality | 250 | 500 |
| Step Between Worlds | 1 | 2 |
| Tempt with Glory | 2 | 4 |
| Tempt with Bunnies | 12 | 24 |
| Tempt with Reflections | 16 | 32 |
| Turtles in Time | 2 | 4 |
| The Second Doctor | 4 | 8 |
| Tempt with Vengeance | 896 | 1,792 |
| Tempting Contract | 1 | 2 |

### Removed Readings: inspected and retired under the ruling

| Face | Parent | Finished | Removed wrong Readings | Classification and routing |
| --- | ---: | ---: | ---: | --- |
| Angelic Arbiter | 9 | 3 | 6 | Relative pronoun as ordinary nominative NP head; wrong analysis retired under the 2026-10-08 ruling; face remains covered, no re-coverage obligation |
| Scythe Specter | 156 | 138 | 18 | Relative pronoun as ordinary nominative NP head; wrong analysis retired under the 2026-10-08 ruling; face remains covered, no re-coverage obligation |
| Hellish Rebuke | 9 | 3 | 6 | Relative pronoun as ordinary nominative NP head; wrong analysis retired under the 2026-10-08 ruling; face remains covered, no re-coverage obligation |
| Fire and Brimstone | 5 | 3 | 2 | Relative pronoun as ordinary nominative NP head; wrong analysis retired under the 2026-10-08 ruling; face remains covered, no re-coverage obligation |

`retirement-ledger.json` identifies every one of the **32** removed Readings by
value fingerprint, full tree and offending-node path. All contain
ZeroObjectRelativeClause → CasePhrase → NominativePronoun with the relative
pronoun Who as the ordinary Subject. This is exactly the distribution forbidden
by the orchestrator ruling, not a valid Reading suppressed to increase uniqueness.
Selector-only reconstruction uses the cached parent grammar and a reconstructed
parent lexical inventory with the exact accepted baseline lexical hash. Every
reconstructed before/after face count and complete Reading digest matches its
full census; all remaining value fingerprints are a subset of the parent set.
The removed sets are **6** Angelic Arbiter, **18** Scythe Specter, **6** Hellish
Rebuke, and **2** Fire and Brimstone. No decrease is attributed to refresh.
No additional full census or grammar edit was needed for this inspection.


The only other changes on previously covered faces are **215 same-count
Reading-digest changes**, named in `digest-only-changes.json`: these are the
existing PP-focusing faces whose generated value is re-spelled from
FocusedPrepositionPhrase to FocusedPhrase(category PrepositionPhrase). The
head, modifier, constituency and lexical leaves remain the same; the existing
independent PP-focus test is re-spelled against that exact value. This is a
constructor representation change, not a retired grammatical Reading.

### Selected gain analyses

The explicit 224-face selector `repair-selector.json` samples every gain
and the nine previously covered exposed faces.
For every sampled face its exact count and complete Reading digest match the
replacement full census; `gain-analysis-ledger.json` records the selected value
fingerprint, lexical owners and construction ancestry. Once/Twice in those
selected analyses are Adverb Phrases attached through AdverbPredicate, never
a subordinator or temporal-location reading. In the interval witnesses the
temporal NP attaches separately through NominalAdjunctPredicate, never as the
Object. The sample is an inspection view; other grammatical Readings remain
retained. All gains are named below with their selected analysis; counts share
the final-tree stamp above.

| Face identity | Name | Readings | Selected analysis |
| --- | --- | ---: | --- |
| `3d02ee2f-b7bd-40b0-b9be-2420991b2be0#card` | Abzan Devotee | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `0cd77ee1-408c-4c51-ac87-24f4fc50083d#card` | Academy Wall | 4 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d8ce3f18-26fe-47e8-be8b-0d89d7de7038#card` | Acrobatic Cheerleader | 16 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `aedcde3b-3ed5-443c-a81c-03d785153e55#card` | Agent Frank Horrigan | 5 | Twice frequency AdvP Adjunct |
| `ff321c13-03ae-4cb9-b371-1242290f8433#card` | Aggressive Mining | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `5a98decb-30ba-4c34-a88a-424b7f4e03bc#card` | Akki Avalanchers | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `6777863c-e52c-4a7c-b8d2-a16b1438b3d2#card` | Amzu, Swarm's Hunger | 30 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `f0950c95-a297-489d-b166-74bba6af5376#card` | Ancient Cornucopia | 8 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `1829f1dc-1fa9-4361-b318-d4dee280e6fd#card` | Anje, Maid of Dishonor | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `b0ed9fbe-324d-467a-a326-c6f85a4c688a#card` | Ant-Man, Colony Commander | 8 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `bc92f0ba-2cf0-453c-a0d3-98a8784fe55b#card` | Ashling the Pilgrim | 1,800 | This fused Determiner-Head NP |
| `91520db8-2d50-4c17-9f0e-2f92f63285f7#card` | Avizoa | 4 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `a82d9c9b-ec17-4f51-a6bf-5984ca4433e5#card` | Azimaet Drake | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `5a88f616-986d-4e97-9457-26c783489123#card` | Barbarian Bully | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `e25b516c-bf95-42bc-8b1e-04617a3d28df#card` | Baron Bertram Graywater | 9 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `c91e64d4-e687-4ccd-a56e-796cff6e052c#card` | Baron Strucker, HYDRA Overlord | 8 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `c6cd520c-106b-4ee5-bc02-6c5fbdbc78dd#card` | Barrels of Blasting Jelly | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `b2ec080f-ad5d-4274-91e3-bc6ac89fc3e4#card` | Basking Rootwalla | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `227cbba1-700b-4255-b730-b324ab3dec34#card` | Beetleform Mage | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `90194ff1-db61-463f-b5a3-15cd85311d0e#card` | Beledros Witherbloom | 5 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `011da9c5-aa8a-4fa0-b1f2-62b9f3760476#card` | Belladonna Took | 672 | This fused Determiner-Head NP |
| `d43f5977-0532-4653-bc57-ce29f9a531c9#card` | Blazing Rootwalla | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `32845ed6-2328-462e-be3a-579589748588#card` | Blood Hypnotist | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `b7040175-7411-43a8-9b46-dba1d7c2135d#card` | Bloombrute | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `2b548f9a-c160-427d-9bb4-1fe940f8ab28#card` | Boreal Centaur | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `578e0577-0ad8-4bfb-a317-9684c508770a#card` | Brutal Deceiver | 16 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `1bcc8ca6-5fd0-4fc9-8f0c-f81d8abf9249#card` | Calix, Guided by Fate | 84 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `8e90300b-c0da-4484-b065-e2fafa4e1b15#card` | Call the Bloodline | 8 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `e446bfa6-54b8-4dfa-98d3-023d12b7543d#card` | Callous Deceiver | 16 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `8958a6fe-76ef-45e3-8522-6bc13486b447#card` | Chance-Met Elves | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `bb300e38-32d7-4152-9a82-88d43af99f19#card` | Chronatog | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `08a40717-c272-42e6-adb1-0148614b1202#card` | Cloaked Cadet | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `482bd55a-ba73-4722-8643-94fa12818515#card` | Contagion Engine | 1 | Twice frequency AdvP Adjunct |
| `2469c0b7-a045-4733-bf53-28f3fdbe3797#card` | Cosmic Crucible | 200 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `ce7d672a-7cb9-40e5-a8ad-dd3ce0f8ecd3#card` | Crazed Armodon | 6 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `ac950d10-e373-42d3-907b-ee56e45a2084#card` | Crossbones, Malicious Mercenary | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `fd077d1a-f395-41c2-b1df-dd5e2f723abf#card` | Cursed Wombat | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `5707c278-dab9-43bc-b3a3-a6b7923511f4#card` | Cutthroat Centurion | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `43192553-0db9-48ec-ae88-a2605a6608d8#card` | Cutthroat Contender | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `f13a8621-4d88-4c1f-8b6e-80f8ef383cd2#card` | Dai Li Censor | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `f7b754d3-9863-420d-895d-00e70b8b078a#card` | Danitha, Sword of Hope | 4 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `a78ab15e-72e7-4d2b-bc42-b2960147ea7b#card` | Darkthicket Wolf | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `f97de425-c7ab-4688-8606-159370d179ee#face:0` | Defiled Crypt | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `667ece0e-a474-4e11-8ac4-6c8fc354457c#card` | Demonlord Belzenlok | 285 | Repeat + this process Object |
| `ed9d4bb1-7335-45fd-b2ea-97b1f6491d7a#card` | Desolation Prowler | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `e4ecfccd-b8b4-4b18-b989-30de79b4971e#card` | Detective's Satchel | 40 | Twice frequency AdvP Adjunct |
| `f61c1dc4-2f09-4b50-957f-ee656c659072#card` | Dina, Essence Brewer | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d5e5cf55-eb9e-4f01-9056-791215411b27#card` | Dionus, Elvish Archdruid | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d558b465-75c6-450e-b1f2-5843ad67de71#card` | Dire Wolf Prowler | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `bd55bcc4-f0c5-4c09-8b3b-2e25e9fbd9ee#card` | Drake Hatchling | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `988aaf54-2e46-4afc-ae37-b0a01191a1f1#card` | Dream Coat | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `30367a59-a812-41d4-a451-80d9071dd4a4#card` | Dusk Legion Duelist | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `2b719057-1bc0-4c3a-97c3-8377844570dd#card` | Dying to Serve | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `e43c31ee-1bac-4f96-8b74-334b3238ab73#card` | Earth Kingdom General | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `26ff3da5-4e02-41d1-a52f-3f8e87b01a77#face:0` | Ecstatic Awakener | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `3f4d6f91-95ad-4687-8899-5a21a0abb49e#card` | Elrond, Moon-Reader | 64 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d7035db0-4bde-4ba3-9028-dd14191c8126#card` | Elvish Archivist | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `bda83ef4-e345-495d-878c-4da171f997ba#card` | Elvish Warmaster | 12 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `634475d9-a1d7-4146-9a37-645d3d162af1#card` | Emet-Selch of the Third Seat | 312 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `98a389f4-2905-47f3-b60e-3d4afb3e5cb0#card` | Enduring Innocence | 65 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `a1640d9a-367f-4b90-bd4d-18f2c9244ccf#card` | Erstwhile Trooper | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `9ad730b5-8950-4215-9319-387e2970dd22#card` | Exemplar of Light | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `cffec16f-373b-43e7-a04c-4452987d7e02#card` | Ezuri, Stalker of Spheres | 1 | Twice frequency AdvP Adjunct |
| `04ac8b2c-95e9-40e9-8cc0-af9b075a94c0#card` | Fang, Fearless l'Cie | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `df516046-7f83-4d64-aa0d-d687631d919a#card` | Fault Riders | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `c3b8ea58-d84f-4c60-9f72-e4ec389eb93c#card` | Feral Deceiver | 16 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d14fdc1d-62bc-4e9c-b79b-8c9c3232f0bc#card` | Festerleech | 6 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `fd3bcc9b-7d84-478e-aef5-2e44610107c7#card` | Fire Drake | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `5ef8925c-5f43-49ca-83f0-e2ca8e655600#card` | Flying Octobot | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `11ceb2db-021e-422d-976e-7bfe3b67a8aa#card` | Foraging Wickermaw | 3 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `c234d2ab-21d5-41a9-9ab8-47a8bd53a686#card` | Forge Boss | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `07ba4fb4-f95b-49ca-be6c-6cdda0f1f030#card` | Frilled Oculus | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `3315aa42-d3ae-425e-9c12-191d26e67a0a#card` | Frilled Sandwalla | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `90772e8f-9b39-4cdd-864f-5e2c917f645e#card` | Frostwalla | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `0c4603be-d71a-4d33-b62c-04ead1987dbe#card` | G'raha Tia | 6 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `f558c758-8f12-45f9-8683-eee33ba1fb0d#card` | G'raha Tia, Scion Reborn | 6 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `9ee0d4a0-fd8d-43c4-a2aa-c4e6af5beecc#card` | Generous Pup | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `e3367ed8-16a3-4b04-bc6a-e0a3c794e6d3#card` | Ghor-Clan Bloodscale | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `9b10e0f7-9414-4769-85d7-c1cd1c74a9a6#card` | Glissa, Herald of Predation | 4 | Twice frequency AdvP Adjunct |
| `13ee85c3-a16a-41ac-bbaa-c7549c700a03#card` | Glistening Dawn | 2 | Twice frequency AdvP Adjunct |
| `f0ad4a60-ce7f-4117-b8b0-266ffd0447c2#card` | Graverobber Spider | 8 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `9ea58a77-9b2e-4ae2-8a31-a51804900a3b#card` | Gravestone Strider | 6 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `97560adc-814e-450c-9ca7-f9364e910a0b#card` | Grindstone | 2 | Repeat + this process Object |
| `fb8f7047-280b-464f-aca2-6add3e21df8f#card` | Harnesser of Storms | 8 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d83bf132-e67e-4af0-b236-4f7a32fa3dec#card` | Harsh Deceiver | 28 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `56208bdc-2eed-4ccc-8c34-c0eba9db0aa9#card` | Harvestrite Host | 252 | This fused Determiner-Head NP |
| `49f92c0b-16ac-4cf0-8ec3-e3638990ee75#card` | Haunted Screen | 11 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d5b9c62d-fe98-4cbc-b4b6-012702bf6657#face:0` | Heir of Falkenrath | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `60040023-7bf0-42e0-ab30-46a730675564#card` | Hoarder's Greed | 27 | Repeat + this process Object |
| `4fad3816-41af-4fa4-85b4-ba37614ec848#face:0` | Hollow Scavenger | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `de83b1a4-0ce0-4cc1-aff3-6d0239d92302#card` | Hostile Investigator | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `a5ca7bd9-0964-405f-adb9-7c27153595e6#card` | Increasing Vengeance | 150 | Twice frequency AdvP Adjunct |
| `059788eb-2c55-4da2-bf54-aa2fa7954169#card` | Inner-Flame Igniter | 72 | This fused Determiner-Head NP |
| `4f31eea0-61b9-467d-8393-0ab9f1b6e882#card` | Insatiable Frugivore | 66 | Repeat + this process Object |
| `f9ee4c80-20fb-4322-be02-1c78d37d7a99#card` | Invoke Despair | 10 | Repeat + this process Object |
| `44594004-52e7-4ce6-8eb0-61f72fd9c73b#card` | Irreverent Gremlin | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `6dcdd313-6df7-4d44-a4cb-0346a4fdb4e1#card` | Jeskai Devotee | 24 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `f5daadc1-98ff-480a-82bb-fe7bfaa7b60e#card` | Jin-Gitaxias, Progress Tyrant | 40 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `619e0686-e88c-4238-8364-75395e733533#card` | Kambal, Profiteering Mayor | 10 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `5973d8f6-242c-4014-b66e-aa5c78440be7#card` | Kami of Jealous Thirst | 9 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `062ffa63-1985-4409-b3ad-d42407ee755a#card` | Kingpin, Wilson Fisk | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `a3275d80-112b-4839-a014-e44ef48d2e6b#card` | Kishla Skimmer | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `f2edb543-de33-4eed-892c-98038eec96e2#card` | Knight of Valor | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `39145960-a212-46b4-addb-4b187f6bf6fd#card` | Knight of Wundagore | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `633e91ea-84d4-42ca-a8eb-603db340ebe7#card` | Knight of the Skyward Eye | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `6766c5e4-baa7-4db4-a900-c08fbcd98f5d#card` | Kozilek's Translator | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `976db753-9887-448a-8e67-ec7937b6505c#card` | Kraven's Cats | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `94806688-329d-429d-8126-2c9187e71f54#card` | Kyscu Drake | 18 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `ddc12487-f577-4def-acdb-491f7ba6dc51#card` | Legolas, Counter of Kills | 18 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `64337db7-a66b-42c5-89fd-c9dfc203c0dd#card` | Linessa, Zephyr Mage | 40 | Repeat + this process Object |
| `9963b6a9-d715-430e-9b5e-576fb688d944#card` | Locust Swarm | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `c112852f-61d5-4b94-b864-c7d0b5333b29#card` | Loki, God of Mischief | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `f8c2a972-e38f-47e5-a355-6f30ad09b1ae#card` | Losheel, Clockwork Scholar | 64 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `1196a4cb-544c-4e7b-91cd-f0820b21a80d#card` | Lucy MacLean, Positively Armed | 18 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `75810ce7-328c-4512-8e36-fd3badbd3997#card` | MACH-1, Swooping Scoundrel | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `7c699180-d18e-4599-acd1-27b951d82136#card` | Mana Bloom | 20 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `24ee57b5-a505-4264-97ca-8b8dfb12298e#card` | Mardu Devotee | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `6d44c87b-8945-4c39-b034-7690cf483992#card` | Mary Jane Watson | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `1c248187-0be3-4a03-a817-14f68f638ae7#card` | Mavinda, Students' Advocate | 96 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `2c2ef1ed-2f93-48c5-bf86-e46318599aeb#card` | Mechan Assembler | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `a86117c1-b83d-4350-9a10-a23b7e67e1bd#card` | Mikey & Leo, Chaos & Order | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `fb7070ab-c234-4d54-8cdc-caf106bf24a6#card` | Mild-Mannered Librarian | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `03af9d24-2df6-4318-82f2-8abdd22206f1#card` | Mindful Biomancer | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `4eba0e29-985b-4763-9a70-0946ba044b43#card` | Mishra, Excavation Prodigy | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `fcae4f9d-8f81-42f9-93a3-82a482851bdd#card` | Moonstone Harbinger | 11 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `322f44f0-e6da-4ee0-b474-e7d5e9a461c5#card` | Morbid Opportunist | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `25636214-c6ea-4655-a5e8-98e735da5b05#card` | Ms. Bumbleflower | 96 | This fused Determiner-Head NP |
| `0ff11a61-6fe3-4c70-aba8-417023a5e54e#card` | Nadu, Winged Wisdom | 4 | Twice frequency AdvP Adjunct; Only focusing that AdvP |
| `07155470-c5ab-41ae-ade3-bdc06d357379#card` | Nanoform Sentinel | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `1b8de1c5-1e23-4c84-bc5a-a14853ad8c7a#card` | Nimrodel Watcher | 7 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `367ac7e2-5056-48f3-a296-87d6cb1f7b54#card` | Nykthos Paragon | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `cd133d30-51ff-4114-a7d7-029345f0f0d7#card` | Omnath, Locus of Creation | 33,600 | This fused Determiner-Head NP |
| `d2ae893f-52af-4645-8712-d62fb78f901e#card` | Ondu Spiritdancer | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `c018f072-8f6d-4835-a717-ed4ecba70d97#card` | Oni-Cult Anvil | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `b828ba28-e98d-498b-9ab4-d4aa7143e407#card` | Pantlaza, Sun-Favored | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `e245f2f6-2b5b-41bc-88b3-5b41ca813c5e#card` | Patron of the Orochi | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `e20b80f2-9913-4886-a61b-04f3fd27c2ad#card` | Plated Rootwalla | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `9780d975-acbe-40c6-b444-1e1e6b772788#card` | Poetic Ingenuity | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `821f4ff3-ebc7-4f47-ac35-d00093b6d35b#card` | Possessed Goat | 34 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `054c6250-ac96-42f6-931d-925761ab7764#card` | Power Plant Worker | 6 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `4ca66b33-28c2-4d0d-8a44-9d891ab13e18#card` | Professor Hojo | 56 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `8dfd9baf-c3cc-4d2f-a0b0-b323b6480562#face:1` | Prowl, Pursuit Vehicle | 192 | This fused Determiner-Head NP |
| `17f78719-d86a-4a00-b83f-7a06bec844f7#card` | Pulsating Illusion | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `16b723bc-aab9-40f2-8e1b-b9c2bd2af3bc#card` | Putrid Leech | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `3ecaefc8-ead2-47a3-a7ea-b030faab65a7#card` | Quirion Ranger | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `fd9aeda1-9e44-4bc6-aa0d-19d58918d1f4#card` | Rally the Horde | 896 | Repeat + this process Object |
| `3ed41d2d-211b-4013-8562-8c64d54cc43a#card` | Ramos, Dragon Engine | 6 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `42e07167-b490-46ad-9b5c-b483a4bb64e3#card` | Remorseless Punishment | 6 | Once frequency AdvP Adjunct; Repeat + this process Object |
| `ec8a4b26-633a-4e88-aa8a-82e9704b3439#card` | Resonance Technician | 240 | Twice frequency AdvP Adjunct |
| `47b29cf9-8ef3-4b6a-a9a3-3ab822c5dea4#card` | Rinoa, Angel Wing | 16,192 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `4dda5f21-11d6-40ce-be24-7c9529272c25#card` | Rootwalla | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d73f02e0-53fa-4437-ab3a-89a9a1db27c5#card` | Rose Room Treasurer | 336 | This fused Determiner-Head NP |
| `3ceca713-38de-4428-a4f1-2146f6e98393#card` | Rumor Gatherer | 144 | This fused Determiner-Head NP |
| `3a1be8b3-16e2-4fae-bea7-3a98f2f9db38#card` | Saint Traft and Rem Karolus | 86,016 | This fused Determiner-Head NP |
| `a96814fe-dc74-4d02-a44e-f7daa794b647#card` | Salvaged Manaworker | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `5a4fabfe-a641-49b7-8134-9d9edeb8bf1b#card` | Savage Knuckleblade | 8 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `cf7bb5d4-c33f-4879-a793-6d6f2a4d94be#card` | Scarecrow Guide | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d1961110-575b-4a1b-9cee-db0e1f0fdbc1#card` | Scryb Ranger | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `929117cd-caa6-402b-9d7b-1f684d75695c#card` | Scythecat Cub | 576 | This fused Determiner-Head NP |
| `675de007-a71e-4036-8204-140c67871b7f#card` | Sea Gate Stormcaller | 9,240 | Twice frequency AdvP Adjunct |
| `55c7ce94-3cd3-42f3-9dd8-0c118fa626c8#card` | Secrets of the Key | 6 | Twice frequency AdvP Adjunct |
| `8a4d505e-b884-4b8b-93d5-495992f3858e#card` | Sengir Connoisseur | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `70113003-be5e-406a-9aec-cb480468c36d#face:0` | Sephiroth, Fabled SOLDIER | 384 | This fused Determiner-Head NP |
| `04936cc9-c915-4182-b84a-3491d26be548#card` | Sepulcher Ghoul | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `fbab033e-3a8f-4c08-a445-46ae34c283eb#card` | Setessan Griffin | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `c736935a-e006-4a67-b942-e8ccf1cd9abb#card` | Sharae of Numbing Depths | 6 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `4b9f6a6e-c168-4732-b454-2272d56636da#card` | Shire Scarecrow | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `2c181479-6df0-44ef-a7ce-d1d592735bc6#card` | Snarling Wolf | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `7ea8426a-3b66-4fd0-9863-2b68d379671f#card` | Soulbright Flamekin | 24 | This fused Determiner-Head NP |
| `762679dd-4345-43c3-929b-15959443aeb0#card` | Soulbright Seeker | 72 | This fused Determiner-Head NP |
| `c5173332-68ad-456e-ad85-dd7475464d94#card` | South Pole Voyager | 24 | This fused Determiner-Head NP |
| `158db438-52ba-4c30-b7bf-d1c4f76da917#card` | Sphinx's Tutelage | 8 | Repeat + this process Object |
| `fde2653f-5270-4b8f-9642-0835dbb076c2#card` | Spiteful Banditry | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `bfbc12ec-ecc0-4e23-a9d1-d13babd99a46#card` | Spitting Drake | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `2659cc53-d20d-476f-bc64-4b35a4288d0e#card` | Stalking Drone | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `e7bd3882-a0f2-4706-80e7-1d3ec455801f#card` | Stick, Fearless Mentor | 12 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `67b87332-44e1-4948-9269-5fc8f6107d0e#card` | Stromkirk Condemned | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `5c0f63ad-4bb1-479e-9760-5421265da40b#card` | Struggle for Sanity | 72 | Repeat + this process Object |
| `70ea1a9d-ec52-4c11-8c4d-0d957e5491ad#card` | Suki, Courageous Rescuer | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `ee7b3fc0-6128-4bf6-b3e7-7371e6f93c4b#card` | Sultai Devotee | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `c47ae79c-4e07-47f2-8920-2ab62b471e4f#card` | Sunbathing Rootwalla | 54 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d53d5d03-b180-4bdb-8801-260f8a75644d#card` | Tamiyo Meets the Story Circle | 104 | Twice frequency AdvP Adjunct |
| `76cb1912-3791-44b8-b594-b92dab21bafc#card` | Tannuk, Memorial Ensign | 48 | This fused Determiner-Head NP |
| `70dd5013-f18f-4501-882f-70590c424e20#card` | Tataru Taru | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `a44ceede-9934-4d4e-8b8c-e087b3ef74d4#card` | Teething Wurmlet | 48 | This fused Determiner-Head NP |
| `e444f072-ce24-4fd1-99eb-d20d12bc119a#card` | Temur Devotee | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `4ac94586-3ac7-4ade-af31-ad0d0e3ffc4a#card` | Terrasymbiosis | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `d4f5748f-bb0e-4c48-8802-45a7ad5a80a4#card` | Teysa, Opulent Oligarch | 96 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `39b67a4d-6a87-41f0-a86f-b66671ccc20d#card` | The Reaper, King No More | 25 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `c867a778-97ef-4872-b890-b088b96a1281#face:1` | The Sensational She-Hulk | 6 | Once frequency AdvP Adjunct; Only focusing that AdvP; This fused Determiner-Head NP; Do + NP Object |
| `1b8870fd-2d8a-42af-8173-b747d8d4c04f#card` | They Came from the Pipes | 2 | Twice frequency AdvP Adjunct |
| `b2f9e07b-64b7-40b3-9a5e-5f1d59b35af7#card` | Thopter Shop | 16 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `6cb9d153-4eaa-4f32-8930-bd68cd99c128#card` | Three Tree Mascot | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `25c983e0-a8c9-4784-91a4-8fe04c6df882#card` | Tocasia's Welcome | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `5d1d9914-3ef2-45c5-b1cb-cb85d2f7394c#card` | Tolls of War | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `84f1e596-f56f-48c9-9bff-2c0bb2cea2d3#card` | Touch of Vitae | 4 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `05d7222b-cc08-4f1f-8e35-6058599572ba#card` | Twilight Diviner | 12 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `2375a9c6-28ef-4c8f-909c-0aa7e5fc085c#card` | Twinblade Slasher | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `ce1bf2c4-93d5-41e3-a450-6b95fad7dd21#card` | Tyrant of Discord | 4 | Repeat + this process Object |
| `4f142d99-8146-4b16-a2cc-ac03da19b960#card` | Ursine Champion | 5 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `8ad7fdd2-bff5-4434-a3f9-1f5ce0ed573b#card` | Urza, Powerstone Prodigy | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `03ebc100-68ac-4d53-adc2-d537212f7d31#card` | Venom Connoisseur | 432 | This fused Determiner-Head NP |
| `9b1ae6d8-0cf3-4a67-9241-37f4bc5ddbdd#card` | Venser, Fervent Forger | 750 | Twice frequency AdvP Adjunct |
| `34bdf2f1-0cc9-4588-9a3e-5cfe5cde815d#card` | Viashino Slaughtermaster | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `9768d43b-8b3f-4d03-ac1b-e603af832875#card` | Vito, Fanatic of Aclazotz | 6,048 | This fused Determiner-Head NP |
| `b2f2645f-5f74-456a-bd02-83169d8b8a7e#card` | Vraan, Executioner Thane | 4 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `3a21a6ae-b2f2-4f0c-acfd-5f3e8d63fd2f#card` | Wall of Roots | 1 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `d1ec1e4e-faa6-4afc-9e00-a3327105c11b#card` | Wavesifter | 1 | Twice frequency AdvP Adjunct |
| `605c1ee0-5e8a-4e0a-a99b-42a38873f822#card` | Welcoming Vampire | 5 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `f8cebb33-9235-4f63-8171-15af8b8e1436#card` | Well Rested | 7 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `321e101c-a426-4d18-be0b-c26c5348b70d#card` | Whispering Wizard | 16 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `dbdf076c-18ce-45e3-8f70-0a32391ea4b6#card` | Wild Aesthir | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `67a52a74-9474-4f4e-8785-6ff54078a8ca#card` | Wirewood Symbiote | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `23d3ff11-bf19-4315-b53b-3e4f20c6a89b#card` | Wolfsbane, Highland Hero | 2 | Once frequency AdvP Adjunct; Only focusing that AdvP |
| `5bc66b18-22ac-4138-b527-fa711116e298#card` | Zimone and Dina | 16 | Once frequency AdvP Adjunct; Repeat + this process Object |

At least ten complete selected values were inspected directly: Kozilek's
Translator, Jeskai Devotee, Basking Rootwalla and Twinblade Slasher (Activate,
Only focusing Once, separate each-turn NP); Ondu Spiritdancer (Do, fused This
Object, same two separate Adjuncts); Nadu, Winged Wisdom (finite trigger,
Only focusing Twice, separate each-turn NP); Remorseless Punishment (Repeat
with this-process Object and unfocused Once); Abzan Devotee, Academy Wall,
Acrobatic Cheerleader and Aggressive Mining (Only focusing Once, with the
attested separate interval when present); Agent Frank Horrigan (unfocused
Twice modifying proliferate). Additional host-only values inspected include
Belladonna Took (fused This as Subject), Sphinx's Tutelage and Grindstone
(Repeat with this-process Object), and Invoke Despair (Repeat Object with the
existing for-PP). No selected gain assigns each turn to an Object. The 29
host-only gains without a frequency adverb are explicitly disclosed in the
table, rather than counted as frequency-only gains.

### Provenance and performance advisory

On `qsrqtwknwxqtrpyrmknwollqqrrnssyo` / covered **21,916**: **202 ordinary Constructions +
49 shared schemas = 251**,
**706 compiled Productions**, **329 schema category instances**,
**146 Categories**, **3,414 nonblank declaration lines**.
Baseline: **202 + 48 = 250**, **704** Productions, **327** instances, **146**
Categories, **3,360** nonblank lines; refreshed parent **3,379** lines. The
fused-head NP is a distinct grammatical construction needed by Ondu. The
ultimate completed-grammar ceiling remains 250 / 2,800; the present partial
grammar exceeds it, disclosed rather than concealed by merging unrelated work.

Named lists `inventory-before.json` / `inventory-after.json`: spellings
**38,380 → 38,392**; unchanged **1,139** homographs and **1,092** case-folded
homographs; unchanged **23** form literals; **zero** form-literal/vocabulary
overlaps (the named overlap list is empty). Added spellings: Process, Processes,
Repeat, Repeated, Repeating, Repeats and each lower-case counterpart. Full
lexical inventory hash `b0b0b7a98db683ad92f7c61e8c98791fbced1d61b5e04882b685d9bf8b57018f`
matches the measured inventory.

Before at `urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` / covered **21,701**:
corpus wall **88,681,432,200 ns**, setup **2,387,749,763 ns**,
checked-text thread CPU **383,972 ns/B**, **12** workers,
host load `[18.49072265625, 25.17041015625, 24.150390625]`.
Finished feature at `qsrqtwknwxqtrpyrmknwollqqrrnssyo` / covered **21,916**: corpus wall
**136,415,517,053 ns**, setup **2,263,243,075 ns**,
checked-text thread CPU **532,155 ns/B**, **12** workers,
host load `[15.26708984375, 13.7666015625, 11.27001953125]`. Both exceed the **16,260,000,000 ns**
quiet-host ceiling; these loaded-host measurements are advisory, not a gate.

Evidence remains under `target/english-v3/english-v3-once-frequency/`; no
process evidence or helper code is tracked in crates. Coordination of
*only once each turn and only if …* remains with the live
`english-v3-systemic-residuals` ticket for cause audit and bounded routing.

### Review notes (2026-10-08)

- Constituency support (hedged). CGEL p. 715, on the frequency expressions
  in its [9iii] list: "In the *on three occasions each year* type, *each
  year* is probably a separate adjunct rather than a postmodifier (as
  suggested by its position in *Each year we had lunch together on about
  three occasions*), but it still serves to cancel the bounding effect". The
  same page names "NPs introduced by *a* or else PPs with *per* as head" as
  "the clear postmodifiers". This supports the two-Adjunct constituency for
  *only once each turn*, with the hedge CGEL itself gives ("probably").
- Gain reconciliation. The 215 gained identities against the 80 sole / 281
  touched estimate: 186 frequency gains (non-sole faces whose other blockers
  had already been read) plus 29 host-only gains (19 fused-*this*, 10
  Repeat). 129 uncovered faces still contain *only once* or *only twice*.
- The `require head.DeterminerUse = Singular` in `declarations.rs` (about
  line 2135) is redundant with the other constraints on that construction
  and is harmless; no change owed.
