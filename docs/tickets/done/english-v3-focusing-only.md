---
needs: []
---
# Read restrictive focusing *only* on licensed clause-final Adjuncts

## Why

Focusing *only* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), *only* before a
clause-final Adjunct (*only during*, *only if*, *only before*, *only as a
sorcery*) touches **930** unread faces and is the sole cause on **472** (recon
buckets "activation/timing restriction" 1,138 / 84 and "only if / only during /
only before" 938 / 30, read together). Counts are unread faces *touched* (at
least one localised failing unit matches) / *sole* (every failing unit matches
and no other recon STRONG bucket does). They are surface counts, not gain
forecasts.

Probes (admitted roots): "Draw a card during your turn." 1; "Draw a card only
during your turn." 0; "Draw a card if you control a Zombie." 1; "… only if you
control a Zombie." 0; "Activate during your turn." 1; "Activate only during
your turn." 0; "Activate as a sorcery." 0; "Draw a card only." 0.

This ticket takes over from `english-v3-systemic-residuals` the
only-if/only-during item ("Only-if/only-during has no other live owner and
stays here") and the orchestrator resolution of 2026-10-06 that left *Activate
only as a sorcery* with that residual, including its deferred widening: the
position licence for clause-final predicative *as* (the fixed-cost landing
licensed only the preposed, comma-separated Adjunct).

## Goal

*only* reads as a restrictive focusing modifier of the licensed clause-final
Adjunct it precedes: temporal *during/before* PPs and conditional *if* clauses,
in imperative and finite clauses. Coordinated restrictions (*only during …
and only if …*) compose through ordinary coordination.

Orchestrator ruling, 2026-10-07, resolves the STOP below: **do not widen
clause-final predicative *as***. The 2026-10-06 deferral stands until both the
*same … as* equality comparative Complement and Enter's selected *as a copy of*
Complement have replacement analyses. Restore the preposed licence entirely;
*Activate only as a sorcery* and its kin remain unread. This supersedes the
original assignment of that widening to this ticket. The ruling authorizes one
replacement full census on the reverted feature and landing of focusing alone.

## Analysis

*Only* is a restrictive focusing modifier (CGEL, Ch. 6, §7.3.1, p. 587, [41]).
Focusing modifiers combine with a wide range of constructions, among them PPs
(*only in America*), declarative content clauses, *to*-infinitivals and
imperative clauses (p. 587, [42]); the relation between the modifier's linear
position and its focus is treated at p. 589. The predicative *as* PP is the one
recorded in the fixed-cost landing; its clause-final position remains deferred by the
orchestrator ruling of 2026-10-07, not by a CGEL claim.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket except the two restriction buckets (`xxknlzypsnwy`):

- Mind Slash: "Activate only as a sorcery." — retained as a negative deferral witness under the 2026-10-07 ruling.
- Celestial Enforcer: "Activate only if you control a creature with flying."
- Hammer of Bogardan: "Activate only during your upkeep."
- Flywheel Racer: "Activate only if this permanent is a creature."

*Cast this spell only during …* faces are mostly multi-cause at this stamp
(Surprise Deployment's "Cast this spell only during combat." also needs
`english-v3-combat-interval-noun`); use them as composition checks, not gain
claims.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Reuse the source-equivalent claim-parent census in
   `target/english-v3/english-v3-focusing-only/before.json`. The first full run
   exposed the STOP; the orchestrator authorizes one replacement full run:
   `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-focusing-only/after.json`.
   Iterate only on selector subsets; evidence stays under `target/`.

2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. *only* attached to the verb or Object when the focus is the
   following Adjunct, or *as a sorcery* read as an Object-oriented depictive,
   is a defect. A wrong analysis that parses is a defect, not a gain.
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

- Frequency *once each turn* (*Activate only once each turn*, 80 sole):
  `english-v3-once-frequency`, which needs this ticket.
- *before attackers are declared*: *declared* is a vocabulary gap at
  `xxknlzypsnwy`; note the faces, do not add the verb here.
- Focusing *only* on NPs inside Determiners or on Subjects (*Only creatures
  …*), and additive *also*/*even*.
- Spend restrictions (*Spend this mana only to cast …*): they also need a
  purpose infinitival Adjunct that does not read (*Pay {1} to draw a card.* 0);
  count them as residue.

## Landing record

### STOP and orchestrator resolution (2026-10-07)

The rejected widening at `pnqvxvwqwkxqnqzrwsyppzyonqxlunrw` / raw covered
**22,124** admitted 638 previously unread faces, including 78 without focusing.
It also increased Deputy of Detention **96 → 396**, Banishment **96 → 396** and
Phyrexian Processor **2 → 5**. These are exposure counts, not validated gains.
The selected Counterbore Reading (**0 → 39**) turns *same name as* into an Adjunct;
Clone (**0 → 18**) turns Enter's selected *as a copy* into an Adjunct; Aven Shrine
(**0 → 9**) has the comparative defect. The two new exclusion tests failed on
that attempt. Evidence: `rejected-as-widening.json`, `rejected-as-comparison.json`,
`as-without-focus-gains.json`, `as-audit.json`, named `*-as-tree.txt` and
`as-regressions-red.log`, under `target/english-v3/english-v3-focusing-only/`.

**Orchestrator ruling, 2026-10-07:** do not extend scope to replacement analyses;
revert clause-final predicative-*as* widening entirely, retain focusing and its
four witnesses, and land. The 2026-10-06 fixed-cost-phrases deferral stands until
both **Same–as equality comparative Complement** and **Enter selected
predicative-as Complement** exist. These named prerequisites and the measured
attempt are recorded in `english-v3-systemic-residuals`. Mind Slash and other
*only as a sorcery* faces stay unread. One replacement full census is authorized.
This is project authority, not a CGEL claim.

`ruling-reversion-source-proof.json` proves the predicative-preposition declaration
is byte-identical to the claim parent and `core.ron` differs only by Only's
`FocusingModifierUse` addition. Finite-clause *as* permissions are unchanged.
No superseded widening declaration or lexical route remains. The existing
preposed-comma exclusion test is restored in full; the independent focused-PP
roundtrip test is re-spelled against temporal *during*, retaining exact expected
values and traversal identities. The required four witnesses remain, with Mind
Slash now asserting the authorized deferral.

### Final measurement and assurance

Claim parent **`pvmzprszmnptxmxwztuyrqtlntlrryqk`** reuses the source-equivalent
full census at **`nplzxtwkzkyslwmlysqrnpmmmuvnknxv` / covered 21,486**
(`baseline-provenance.json`); its only source differences were unrelated xtask
Lean-definition/facts checks. The replacement **full** census is measured at
**`urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` / covered 21,701**. Both enumerate the same
32,828 face identities with byte-identical analyzed sources. `comparison.json`
compares every face, not aggregate percentages.

| Measure | Claim-parent census | Finished focusing feature |
| --- | ---: | ---: |
| Covered | 21,486 | 21,701 |
| No Reading | 11,342 | 11,127 |
| One Reading | 7,472 | 7,472 |
| Multiple Readings | 14,014 | 14,229 |
| Exact Readings | 737,600 | 782,215 |
| Undetermined / issues / internal failures / cyclic derivations / duplicates | 0 | 0 |

**215 newly covered faces; zero lost faces, zero Reading-count decreases, zero
increases on previously covered faces.** There is no baseline loss to route and
no trunk-attributed decrease. Counterbore, Clone, Aven Shrine and Mind Slash
are **0 → 0**. Deputy of Detention is **96 → 96**, Banishment **96 → 96**, and
Phyrexian Processor **2 → 2**, removing the rejected attempt's additions.
All **426** reductions relative to the rejected tree are named and justified in
`ruling-reversion-ledger.json`: **129,921** unlicensed attempted Readings retired,
**423** attempted-only faces again unread, and the three still-covered faces
just named. Recoverage is routed to `english-v3-systemic-residuals` and its two
named prerequisites; none is credited as a focusing gain.

Every counted Reading passes declaration admission, lexical ownership/context,
byte-exact realization and node/word traversal comparison against materialization
traces. The corpus command does not itself establish linguistic correctness or
the independently constructed-value roundtrip law. The independent temporal
focusing test constructs the complete expected Reading without parsing a template,
asserts exact realization and parse-back equality, compares complete node and Word
traversals, and rejects a non-focusing modifier and an unlicensed PP head.
Existing independently constructed nominal, frame and agreement outcomes remain.
All Readings are retained: the unchanged unique count and increased multiple count
above replace the retired destructive selection census. No priority, specificity
ordering or destructive tie resolution was added; specificity-resolved totals
are not emitted by v3.

The new guards read declared `FocusingModifierUse`, `PrepositionFunctionLicence`,
`SelectedPrepositionUse`, Number and Countability. **Zero new word-naming guards**;
the existing environment load-error tests pass. The active v3 census emits no
legacy permitted/forbidden licensing-checker total and has no coverage lock;
those legacy fields are **not applicable**, not invented zeroes. The census,
its source hashes and explicit admission/structure tests are the v3 evidence.

### Selected gain analyses

Post-landing reconciliation (orchestrator review, 2026-10-08): the **215** validated
gains do not fulfill the earlier **472** sole-cause estimate. That estimate was
prioritization evidence, not a verified gain forecast; the retained *as* deferral
and independent host gaps limit what this implementation can discharge. **984**
unread faces still contain the restriction strings. The overlapping categories
are *as a sorcery* **501**, deferred until both **Same–as equality comparative
Complement** and **Enter selected predicative-as Complement** land; *once each
turn* **281**, owned by the in-flight `english-v3-once-frequency`; *only if*
**125** and *only during* **106**, on multi-cause hosts; and *declared* **45** and
*only before* **8**, with vocabulary gaps. These categories are not additive.
This is review evidence for the accepted landing, not a new full census.

The 641-face selector audit samples all **215** final gains plus the rejected
attempt's remaining candidates. Every audited face's count and complete Reading
digest matches the full census. `gain-analysis-ledger.json` names each gained
identity, its complete source, selected fingerprint and focusing dependencies.
The sample is an inspection view; other admitted Readings remain retained.
In every selected gain Only is an Adverb Modifier of a PP in final Adjunct
function, never a Determiner, Object Modifier or verb-focusing Modifier.
The observed immediate parents are `ClausalPreposition`, `PrepositionPredicate`
or ordinary PP `Coordination`; conditional Finite Clauses are inside the PP.

Manual spot-checks at `urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` / covered **21,701**
(`manual-spot-checks.json` and named `*-reverted-tree.txt`):

| Face | Readings | Selected analysis |
| --- | ---: | --- |
| Celestial Enforcer | 7 | Only modifies If's conditional PP; Control's creature-with-flying NP stays inside its Finite Clause. |
| Flywheel Racer | 2 | Only modifies If's PP containing the copular clause *this permanent is a creature*. |
| Hammer of Bogardan | 6 | Only modifies During's PP with possessive *your upkeep* Complement. |
| Quakebringer | 8 | Only modifies a coordinated conditional PP headed by If; both condition branches stay inside the focused PP. |
| Defiant Stand | 28 | During and If PPs each have their own Only Modifier, combined by ordinary PP coordination. |
| Humble Defector | 2 | Only modifies During's PP with *your turn* Complement. |
| Tainted Isle | 2 | Only modifies If's PP containing *you control a Swamp*. |
| Desert | 2 | Only modifies During's PP with *the end of combat step* Complement. |
| Rimewind Cryomancer | 2 | Only modifies If's PP; *four or more snow permanents* belongs to Control's Object. |
| Feast of Blood | 2 | Only modifies If's PP with the Vampire-control condition. |
| Berserk | 42 | Only modifies Before's PP with *the combat damage step* Complement. |

### Deviations, additions and residue

- Added `FocusedPrepositionPhrase` and the declared `FocusingModifierUse` feature
  on the existing Only lexical owner; no new Lexeme, Word Form or spelling.
  Guards license PP Adjunct function and exclude selected-PP use; the wrapper
  exports no preposed, noun-modifying or complement licence. Existing ordinary
  clause/VP attachment and PP coordination consumers are reused.
- Consolidated `BarePlural` and `BareMass` into `BareNounPhrase`: their identical
  bidirectional form differs only in the two allowed Number/Countability pairs.
  The shared table permits exactly plural/count and singular/mass, preserving
  Determiner Requirement and all other equations. This avoids construction
  growth for a clear new grammatical distinction. This merge was outside the
  ticket's letter and was done to hold the construction count. It was proven
  Reading-identical: **4,605** faces changed their digest only, with unchanged
  Reading counts and analyses. Both superseded declarations
  are retired; no lexical frame names either old route, and lexical count/mass
  declarations remain authoritative. No duplicate constituency route is kept.
- Relative to the claim parent, restored **0** tests: the test described in
  `test-accounting-reverted.json` as restored once during iteration is unchanged
  in the landed feature. Re-spelled **18** existing tests and
  **8** helpers at **28** constructor sites, added **12** tests, ignored **0**,
  removed **0** (`test-accounting-reverted.json`). The three new focusing tests
  adjusted by the ruling keep their subjects: Mind Slash is a negative deferral
  witness; bare As retains one standalone PP Reading and zero final-clause
  Readings; the independent focus law uses a licensed temporal PP. No value
  assertion was replaced by a discriminant check.
- Only-if/only-during/only-before restrictions on already licensed hosts are
  discharged here. Frequency *only once each turn* remains with
  `english-v3-once-frequency`; purpose infinitivals now have the named live owner
  `english-v3-purpose-infinitival`, minted by the imported wave-3 ticket batch.
  Missing *declared*,
  possessive *own* and the remaining bare-combat boundary mismatch remain
  residuals, not additions in this ticket. Dosan and Maddening Imp retain
  negative whole-face diagnostics for those independent gaps. Final predicative
  As is explicitly deferred under the resolved STOP above.
- Glossary gaps found and filled: Focusing Modifier, Focusing Modifier Use and
  Bare Noun Phrase. CGEL backs only the stated PP modification and bare-NP
  passages; position deferral is cited as an orchestrator ruling. No new CR
  citation or invented rule number was added.

### Validation and provenance report

Passed on the reverted feature:

- `cargo test -p deckmaste_english_v3 -p deckmaste_lexical_source`: **383 English
  tests and 56 lexical-source tests** (23 unit, 33 integration), zero failed or
  ignored (`tests-reverted.log`).
- `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source --tests -- -D warnings`:
  passed (`clippy-reverted.log`).
- `cargo +nightly fmt -p deckmaste_english_v3 -p deckmaste_lexical_source -- --check`:
  passed (`fmt-reverted.log`).
- `cargo xtask cite check --list-noncompliant`: **0 noncompliant**;
  `cargo xtask cite check`: **0 stale**. The actual feature diff piped to
  `cargo xtask cite audit --diff` contains **0 changed citation sites**;
  no bless was needed (`cite-list-final.log`, `cite-check-final.log`,
  `cite-audit-final.log`).
- Full replacement census: exactly
  `cargo xtask english-v3 --all --workers 12 --samples-per-face 0 --output target/english-v3/english-v3-focusing-only/after.json`;
  complete, no limit or incremental reuse. The first full census is retained as
  the rejected attempt; the second is expressly authorized by the ruling.
  No further full run, xtask tests, Lean check or derived gate was run.

At `urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` / covered **21,701**: **202 constructions
+ 48 shared schemas = 250** before and after; **704 compiled productions** before
and after; **327 schema category instances** and **146 declared Categories**
before and after (`schema-instance-counts.json`). Nonblank declaration lines
**3,351 → 3,360**, disclosed rather than
fitted to the final-breadth target. Named inventories in `inventory-before.json`
and `inventory-reverted.json` are identical: **38,380** spellings, **1,139**
homographs, **1,092** case-folded homographs, **23** form literals, and **zero**
form-literal/vocabulary overlaps (the named overlap list is empty). Full lexical
inventory hash `4fd28826c2e5e52c68d022c1a4e2a1e192370d06deae737eb5390b2cb849ed98`
matches the measured inventory (`inventory-reverted-provenance.json`).

Performance advisory (integer ns, not a gate): baseline at
`nplzxtwkzkyslwmlysqrnpmmmuvnknxv` / covered **21,486**, with 12 workers and
host load **[13.8583984375, 13.33056640625, 15.5107421875]**, corpus wall
**158,351,420,863 ns**, thread CPU **518,957 ns/B**. Final at
`urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` / covered **21,701**, with 12 workers and
host load **[18.49072265625, 25.17041015625, 24.150390625]**, corpus wall
**88,681,432,200 ns**, setup **2,387,749,763 ns**, thread CPU **383,972 ns/B**.
The corpus wall exceeds the quiet-host **16,260,000,000 ns** ceiling; neither run
was a quiet-host comparison. Compilation preceding the command's corpus phase
is excluded from those report timings. Do not infer an isolated speedup from
these differently loaded hosts.

The pre-census refresh imported an unrelated Lean-facts checker edit and
Ascend's comment-only change `kqnwnynmsqqollmookxrzwysmurxvzns`; spelling and
lexical declarations are unchanged (`reverted-refresh-source-proof.json`).
The measured numbers remain stamped to their measured tree. Evidence is retained
under `target/english-v3/english-v3-focusing-only/` in default before drop.
The post-census refresh at review change `kpwzlsustmpunotkvysuouxqnoqmsuuz`
changed **zero** census snapshot sources (`final-refresh-source-proof.json`);
imported ticket/context documentation does not alter grammar, lexicon or corpus.
No post-refresh full census was needed.
The subsequent refresh imported `nmunomuownlmrlptqmkknwoxstzzztwm` (shared
spelling-template `:word` hint) and `qrzuzkqwwqpzupkqmuzzmmlvyssnwrwt` (seven
shared spelling templates). English declarations and plugin `grammar:` fields
are unchanged; lexical export explicitly ignores shared spelling templates.
The freshly loaded **complete lexical inventory is identical**, including its
full census hash and every named spelling/homograph map
(`lexical-inventory-refreshed.json`, `integration-source-proof.json`).
`cargo test -p deckmaste_lexical_source` again passes **56** tests, zero failed or
ignored, and `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings`
passes (`tests-lexical-refreshed.log`, `clippy-lexical-refreshed.log`). Unchanged
English grammar/runtime and identical lexical inputs preserve its test and full
census results; no extra full census or Lean check was run.

### Complete gained-face inventory

All rows below are measured at `urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` / covered
**21,701**, with before count **0**. Analysis codes refer to Only modifying the
following final Adjunct PP: **C** conditional If (Finite Clause Complement),
**T** temporal During (nominal Complement), **B** temporal Before (nominal
Complement). Combined codes have separate focused PPs in ordinary coordination;
**C (PP coordination)** focuses the conditional PP coordination as a whole.
The complete selected fingerprints and dependencies are in `gain-analysis-ledger.json`.

| Face identity | Readings | Selected focus |
| --- | ---: | --- |
| Ambitious Farmhand (`b85e7ab2-6f57-4b65-8c16-034b342996f0#face:0`) | 308 | C |
| Ancient Hellkite (`de672a4b-07eb-4be8-a03f-23dfbb7ac0fa#card`) | 2 | C |
| Angus Mackenzie (`d5e6ac6b-5786-4d98-8037-0867e365fc93#card`) | 20 | B |
| Arcum's Sleigh (`40960ec8-e3be-45f2-8d27-316d82c69123#card`) | 8 | T (PP coordination) + C |
| Argent Sphinx (`7220c391-8d0e-4f43-94fc-ff77ffa2ee8d#card`) | 48 | C |
| Assassin's Blade (`76da2150-34b9-4483-99df-131e1c5468d5#card`) | 14 | T (PP coordination) + C |
| Augur il-Vec (`34e05dc1-35af-402d-96ae-e249b3738493#card`) | 2 | T |
| Augur of Skulls (`723398f2-eaf2-4615-9078-418b9f540a33#card`) | 2 | T |
| Aven Augur (`e68d1b86-3996-4bbf-9086-a76a7f9ce2a4#card`) | 4 | T |
| Barad-dûr (`88159872-d37d-4847-b048-e4a9af6437bd#card`) | 6 | C |
| Baxter Building (`71bc69a5-7cec-4abd-b97d-13f8e1f9afac#card`) | 28 | C |
| Berserk (`8b67d192-9a05-4a47-82ae-5fc4b7834d88#card`) | 42 | B |
| Bitter Work (`e9a24ed8-e844-4c50-b386-bc28da9989b1#card`) | 20 | T |
| Black Carriage (`6bf5dcf1-5c93-42f4-adaf-a6f2e5807596#card`) | 12 | T |
| Blazemire Verge (`977c2f33-b622-4172-9efb-7f523becd32b#card`) | 2 | C |
| Bleachbone Verge (`2b8144a0-08d2-4c28-9fd7-5d90f90105e4#card`) | 2 | C |
| Blitzball (`75719f87-ce1a-40d6-ac5a-5ebf0c936c84#card`) | 14 | C |
| Blizzard (`698feaaa-2fff-4809-81d1-4caad44c6c0a#card`) | 12 | C |
| Blood Frenzy (`8d3d5e87-8b12-4549-a46e-70a103dbcf44#card`) | 8 | B |
| Bloodline Keeper (`434e82b5-abd4-41ac-a5b1-2dd01c667374#face:0`) | 14 | C |
| Bloodshot Trainee (`87ab1c79-79f5-48a9-af1a-7c876684c8d9#card`) | 2 | C |
| Bloodsoaked Champion (`cad09970-14c8-4d80-82fe-6c855efb0191#card`) | 30 | C |
| Bonders' Enclave (`f33ce38a-34ec-4b65-a0fc-160484a02007#card`) | 7 | C |
| Bonecache Overseer (`be3c7e4c-81fd-49e7-8fb5-5eb83994dafd#card`) | 10 | C |
| Boxing Ring (`0e676904-17c3-40f2-9b5b-eb29cacc336f#card`) | 26 | C |
| Brackish Trudge (`c775d3ef-2184-4425-8c8e-2bbc49687f76#card`) | 9 | C |
| Caged Zombie (`430b3e61-09a5-4390-b377-89830cacf66e#card`) | 3 | C |
| Candletrap (`089788ce-d06c-4a85-915c-82e2cb5b0103#card`) | 168 | C |
| Carrionette (`24315f60-731a-40bb-85ba-eedba1b011ff#card`) | 4 | C |
| Cauldron Dance (`e9784eb4-96f2-409e-a7b5-eb6478644943#card`) | 160 | T |
| Celestial Enforcer (`81f5c70f-fbb1-4b3a-a048-1f4602adfa0e#card`) | 7 | C |
| Chained Brute (`3835cb81-b6e4-4be8-9d49-8307ddc2e1a2#card`) | 12 | T |
| Champion's Victory (`8df38d95-1707-4f83-9d64-41f0be2a3b3b#card`) | 28 | T (PP coordination) + C |
| Choking Vines (`3b6b565e-c0b5-48f9-b3e0-4d5d4b065402#card`) | 2 | T |
| Cinder Crawler (`3850479f-1c1e-4a72-ae47-6da2997202e2#card`) | 8 | C |
| Cinderhaze Wretch (`df6d9787-b423-4067-adf4-c09dcee1329e#card`) | 2 | T |
| Cleaving Reaper (`70c268af-3fb4-42b8-a87a-d06d33c1c641#card`) | 42 | C |
| Coffin Puppets (`e49771ab-cabb-4a5f-aef3-a0ac8147e6bf#card`) | 12 | T (PP coordination) + C |
| Colossus of Sardia (`9be9625e-b98b-416b-aac4-9f7b2dfbd39d#card`) | 12 | T |
| Command of Unsummoning (`fd57dfa5-f29d-4b79-8748-c34a73efb7f0#card`) | 28 | T + C |
| Companion of the Trials (`3aa654cb-c0a0-48e7-901a-c690120b1510#card`) | 2 | C |
| Crown of Doom (`fed77e0a-a4a6-4409-9cab-bf0fc2cd9f9b#card`) | 12 | T |
| Cryptex (`8186a992-9065-4a7f-bb00-1c20e01e8fb6#card`) | 18 | C |
| Cryptic Caves (`b2eb7a64-a307-4a78-a25d-63fb3ae1e237#card`) | 2 | C |
| Cult Conscript (`e5762815-5fc3-4ae7-abee-69f27c5e4e90#card`) | 12 | C |
| Dark Fortress (`40760bfa-a423-487c-ba29-043b2d00c736#card`) | 8 | C |
| Dawnhart Mentor (`e12fb44a-524e-4603-a89c-fad8204fdb2b#card`) | 42 | C |
| Deep Wood (`3f01f627-9fbd-470b-8001-974784ccf421#card`) | 476 | T + C |
| Defenders of Humanity (`7662af07-1ebc-4545-bffd-7a2e0c513b71#card`) | 288 | C + T |
| Defiant Stand (`05e61628-cdae-4664-b866-be438fb3a6ba#card`) | 28 | T + C |
| Desert (`195107ad-879d-4b02-a44a-a3ba70fedf88#card`) | 2 | T |
| Desiccated Naga (`ab172e3e-e71e-4627-9b9a-a9295b891533#card`) | 2 | C |
| Disrupting Scepter (`cd30a128-4b23-476e-8066-2272fdc395d9#card`) | 2 | T |
| Dwarven Armory (`aba53a54-8666-430d-899e-45ad9f783a4a#card`) | 2 | T |
| Dwarven Weaponsmith (`a3541870-3dc9-4571-be98-c0a2b6c468fb#card`) | 2 | T |
| Earthlore (`91fbffd1-0886-45fa-bf3e-4c9864458431#card`) | 8 | C |
| Eightfold Maze (`9097457c-ceb3-4944-8f01-c28ddefbb270#card`) | 14 | T + C |
| Emberwilde Augur (`39bf60b2-eeee-48f3-b7b0-6cc2e038afb1#card`) | 2 | T |
| Endless Atlas (`cb923c4d-1d2e-4039-90bb-72d1981b9738#card`) | 7 | C |
| Essence Anchor (`e28a3f0e-40bd-4293-b5eb-ec9d185c1e1c#card`) | 6 | T + C |
| Essence Reliquary (`528039f3-e8e6-499d-add4-43cf2d731b65#card`) | 8 | T |
| Falkenrath Pit Fighter (`46d6bc30-9a21-4476-947b-87951cd9c882#card`) | 3 | C |
| Fanatic of Rhonas (`7973820b-fdaf-46ec-9e3e-d4c0e77b5067#card`) | 14 | C |
| Feast of Blood (`fed05740-01b2-4c7a-8b97-55e64837c07f#card`) | 2 | C |
| Feasting Troll King (`6f0c6876-dcae-46e5-8c62-6929733c3525#card`) | 12 | T |
| Festival (`8e594801-4a8d-4dcd-b328-7eb3de65cf90#card`) | 6 | T |
| Firemane Angel (`837e4452-3002-41e3-890b-3d54b469e7b9#card`) | 12 | T |
| Fixer, Techno Terror (`a47b71df-9357-4118-90c0-f86fca6d362b#card`) | 7 | C |
| Fleshformer (`67b8b67d-fda3-4a62-9bfb-51a171638502#card`) | 12 | T |
| Fleshless Gladiator (`8e33d6e5-30a2-48b4-826c-09aa75bbba27#card`) | 8 | C |
| Floodfarm Verge (`f1e9abfb-c3c8-483e-b446-5c2afc9f6394#card`) | 2 | C |
| Flywheel Racer (`0399b932-2a59-471c-828e-2b16614e0dae#card`) | 2 | C |
| Fog Patch (`eb8ff6c1-f9b4-4dae-8f46-0f1ad4c05355#card`) | 2 | T |
| Forgestoker Dragon (`53de1e6f-9471-4cdc-ac67-66d47da4d7eb#card`) | 6 | C |
| Fungus Elemental (`3d3c5dd5-ad4e-48ca-92b1-91e1f4ad5c3b#card`) | 5 | C |
| Gathering Place (`36b58705-c5a5-4547-8d8b-a7c35e1f69ae#card`) | 8 | C |
| Gerrard Capashen (`27521e70-3a1f-4819-a111-c38216ffe7c3#card`) | 24 | C |
| Gilt-Blade Prowler (`63e86054-5e9c-48fe-92a1-b71aeb6ee57f#card`) | 5 | C |
| Gleaming Bastion (`7785ffd4-f169-475d-9558-ce4877b3378a#card`) | 8 | C |
| Glint-Horn Buccaneer (`64ad5657-78e9-4f34-8877-18c4f51fff9a#card`) | 2 | C |
| Glistening Sphere (`15679808-baba-4a11-b111-f313ce24197f#card`) | 2 | C |
| Gloomlake Verge (`d71bda4c-3dee-4398-8fd0-f77d8743b887#card`) | 2 | C |
| Glory (`3853ef6a-6771-4708-8ed5-d667f646de5a#card`) | 8 | C |
| Goblin Bird-Grabber (`780f8be6-d477-43ca-92ba-0fe480bbd54d#card`) | 14 | C |
| Greenhilt Trainee (`1d7d340d-fe6f-4314-97ba-e0bf36a4ef6c#card`) | 4 | C |
| Grim Wanderer (`614feadd-b74f-4d9c-a0a3-3a63724956af#card`) | 3 | C |
| Gutterbones (`76ae3779-c821-4cd3-8d0f-8c42d1206ce4#card`) | 18 | T (PP coordination) + C |
| Gwendlyn Di Corci (`de44e7c8-01f9-44f9-9716-9e8a5b82b1ff#card`) | 4 | T |
| Hakim, Loreweaver (`43e99cba-a6e3-4abc-a92b-aaf5839c72d2#card`) | 20 | T (PP coordination) + C |
| Hammer of Bogardan (`3089bdd7-87e6-4d0e-ad98-c32e6f861c30#card`) | 6 | T |
| Haunt of the Dead Marshes (`0b683611-e46f-4105-b336-014d1556c8de#card`) | 8 | C |
| Heart Wolf (`2f50c45f-058d-41fb-9965-08c2e0edc8ac#card`) | 6 | T |
| Heavy Fog (`a2006755-8812-4aad-8567-e8df6e8923da#card`) | 476 | T + C |
| Heidar, Rimewind Master (`58ac4cce-506e-4d6e-a19c-0e875fbfcd81#card`) | 4 | C |
| Hell's Caretaker (`2f0d797e-d897-453d-92b6-a90e1a548dc5#card`) | 10 | T |
| Hewed Stone Retainers (`a27938d2-9a15-4a87-9e77-dea37347cf62#card`) | 5 | C |
| Hidden Lair (`7069d241-4e66-40bf-afd1-551a4a5457f0#card`) | 8 | C |
| Humble Defector (`1582e16d-4825-4cc5-b12c-449c45e4e449#card`) | 2 | T |
| Hushwood Verge (`cce328b9-6100-417e-9ddf-808bbe3e3bc5#card`) | 2 | C |
| Idol of Oblivion (`c232d974-9c65-4bce-b045-e5a7cc0c62e0#card`) | 3 | C |
| Illusory Angel (`6ee32b5e-3b29-47ff-87f8-913ec05f6b2f#card`) | 5 | C |
| Inventors' Fair (`91d4a5fe-fd6d-4b14-a63f-61b4d0ecd9c4#card`) | 14 | C |
| Jade Statue (`96162b22-41e7-4559-aa1b-c18f42abcb52#card`) | 12 | T |
| June, Bounty Hunter (`1779869a-5e54-4ab5-86b8-d4d20e2e1ba4#card`) | 46 | T |
| Just Fate (`2619692a-07a0-4f50-88b0-4cec6dbc8600#card`) | 14 | T + C |
| Kelpie Guide (`8fc1e8b1-3afc-4e9d-a3ae-7bd9bcdcb465#card`) | 4 | C |
| Kitsa, Otterball Elite (`16414460-11ce-481d-ab54-6f0e0ea7d83d#card`) | 100 | C |
| Kjeldoran Elite Guard (`a1f6df2a-fc97-44d4-a0d1-902d8df94002#card`) | 4 | T |
| Kjeldoran Guard (`bd699cf8-56c7-4375-920e-8dd22c83a82c#card`) | 8 | T (PP coordination) + C |
| Kongming's Contraptions (`c3327ffe-f47f-4f5b-b929-a7706ef4932d#card`) | 14 | T + C |
| Krovikan Plague (`609c3b1c-d9ad-4616-b685-52b85528fde8#card`) | 48 | C |
| Kuldotha Phoenix (`884260cf-b3f3-43a4-82b6-4f6a0b19cc75#card`) | 24 | T (PP coordination) + C |
| Lagomos, Hand of Hatred (`12c98c1e-7e02-473d-b494-ad8a0731fe82#card`) | 336 | C |
| Lavinia, Foil to Conspiracy (`b5b26cea-9be3-46f7-9603-a6eee0dd107e#card`) | 8 | T |
| Leechridden Swamp (`d83c86c1-126d-49e9-9b13-9e55784c49c5#card`) | 2 | C |
| Lesser Werewolf (`5542afeb-285f-4889-9d09-46ce20d0d189#card`) | 18 | T |
| Life Chisel (`2493fe65-2ecb-418f-8e4e-797f83475c73#card`) | 4 | T |
| Llanowar Augur (`b3052531-f7cf-44a7-9492-eb155d75268c#card`) | 6 | T |
| Luminarch Ascension (`90076bf5-aa9a-4a6e-9035-9aa97fd5561e#card`) | 196 | C |
| Lure of Prey (`3c83ea11-7141-41a2-ba77-1ed2722dfe74#card`) | 9 | C |
| M.O.D.O.K. (`fe7cc414-42d0-4dad-8516-7a0be0860a67#card`) | 2 | T |
| Madblind Mountain (`0ee0b090-3f1e-49d6-bcad-91e0cf1d12ae#card`) | 2 | C |
| Marjhan (`3fdee2ab-7ec6-4fc6-ad99-f04571f94583#card`) | 180 | T |
| Master's Manufactory (`2dfdd358-e3cd-4bf5-871b-37689b7951e9#face:1`) | 7 | C |
| Merfolk Windrobber (`be8b9bef-a98d-40a8-bedd-d78f1f08f93b#card`) | 21 | C |
| Metallurgic Summonings (`7d5149c6-0e7e-438c-a554-5235c85e2426#card`) | 40 | C |
| Minas Tirith (`7b0d7e62-0287-454a-8702-b0bfa7b41245#card`) | 8 | C |
| Mirrex (`5502741a-e3b9-454e-8121-4360a6db6750#card`) | 35 | C |
| Mistveil Plains (`bb5c1817-ac22-4779-9005-251bc354f181#card`) | 2 | C |
| Moonring Island (`cf620c66-7db1-4db8-ae56-ee4bc2f77d74#card`) | 8 | C |
| Mox Jasper (`d5fca77c-377f-41b4-a9e9-e91a89d351cc#card`) | 2 | C |
| Mox Opal (`de2440de-e948-4811-903c-0bbe376ff64d#card`) | 4 | C |
| Mtenda Griffin (`273f0353-4936-491b-82da-6d4d7f3259a6#card`) | 40 | T |
| Necrologia (`925f11c3-fd32-4aa7-a130-6554b81ad3eb#card`) | 2 | T |
| Necrosavant (`6732a587-e3cc-4c30-9267-f8949dd8724a#card`) | 6 | T |
| Nemesis Phoenix (`bd0dcb8c-d550-4034-8597-0aba6f6e3b3f#card`) | 4 | T (PP coordination) + C |
| Nim Devourer (`89075fbe-cd8a-4054-abab-fcaa2aedddfb#card`) | 24 | T |
| Nimbus Maze (`d7e1d4eb-1d4e-460e-9304-7db9ab50ccb5#card`) | 4 | C |
| Path to Redemption (`ac94860b-70b8-4624-9758-4bb024a56f60#card`) | 4 | T |
| Portal of Sanctuary (`c30491b5-08fe-4694-9ac8-53be7b4b92ce#card`) | 4 | T |
| Potioner's Trove (`d604c75e-9d81-49de-80bc-08f4ef3ee975#card`) | 10 | C |
| Professor Zei, Anthropologist (`59a10335-72df-44a5-a514-5bac392cd282#card`) | 24 | T |
| Quakebringer (`bbe41a21-f8ad-4e5c-88d8-2a41dfd32d13#card`) | 8 | C (PP coordination) |
| Rag Man (`8132c521-44e2-4fc0-b3de-e5b92d04463a#card`) | 6 | T |
| Rally the Troops (`555927b3-bbc8-48b7-9a04-e8ebaabccf3e#card`) | 14 | T + C |
| Remove (`a1ed0f7e-56c3-4866-89c4-77b81a952955#card`) | 28 | T (PP coordination) + C |
| Repeating Barrage (`a08c599e-ee75-4d2b-ae9b-0987c233dd17#card`) | 30 | C |
| Rescue Girl, First Responder (`df6fc0b4-333b-4f6c-804c-ba6c8e29c88e#card`) | 8 | T |
| Reset (`4faa5a70-ac0c-4a29-8bec-218a35a29fdf#card`) | 3 | T |
| Rimewind Cryomancer (`23c881f8-37c7-4899-890e-9f6e26f9f560#card`) | 2 | C |
| Rimewind Taskmage (`f5d9248e-9cff-4f80-b6ed-7d368099e97b#card`) | 2 | C |
| Rivendell (`2550099d-b3e2-4eb6-9f36-0fc412828ca6#card`) | 4 | C |
| Riverpyre Verge (`510a6ac5-f098-4145-ac07-771b1b6f7cdf#card`) | 2 | C |
| Sacred White Deer (`2d7bef31-3447-4de6-9d0e-bd6749e7ecd5#card`) | 2 | C |
| Sapseep Forest (`8d4dcab0-86e5-4ff8-a90f-78a062664e16#card`) | 2 | C |
| Scepter of Fugue (`f0b1dcdc-4fbb-4bcb-a4b0-1aa5928347f9#card`) | 2 | T |
| Sceptre of Eternal Glory (`ce08aff3-efe6-4186-9557-274e00a7ca38#card`) | 7 | C |
| Scorching Winds (`db025c94-1182-412e-9665-b5ae89d26616#card`) | 14 | T (PP coordination) + C |
| Sea Troll (`feabe30a-3c28-43a1-b538-b0207758520c#card`) | 37 | C |
| Seeker of Insight (`a0b2d446-fe90-40f8-8674-7421c425b489#card`) | 10 | C |
| Shard Phoenix (`3b629793-0efb-4d60-9d82-38b7b400abfd#card`) | 18 | T |
| Shi'ar Soldier (`f20ec8e2-d6e4-4707-9b86-107dbc09ac63#card`) | 8 | T |
| Sinew Dancer (`a605864a-f46f-4ebc-8e56-ce7322c918b0#card`) | 2 | C |
| Skarrgan Firebird (`42a96276-fddc-435e-bd47-04f9c6001ec7#card`) | 15 | C |
| Skirsdag High Priest (`c6064c08-e4e0-49f4-9f0b-55cdecf443af#card`) | 84 | C |
| Skitterskin (`ff3f5b1e-9d53-4258-84ee-cfecf685dd05#card`) | 2 | C |
| Skyblade's Boon (`a03a9ddb-595a-4dff-b7cb-7ad60655f1ee#card`) | 12 | C |
| Skyshroud Condor (`c7c4c67e-c4db-416b-b2e8-f0e59aa80bba#card`) | 5 | C |
| Spinal Embrace (`4cf3fb65-9107-428a-8853-029ec97112b5#card`) | 8 | T |
| Spire of Industry (`55a519b4-61cb-448a-875b-4d6dbe00580f#card`) | 2 | C |
| Steadfast Unicorn (`c823d38c-e73c-4e09-ab20-c5e7de1f1c64#card`) | 12 | T |
| Stella Lee, Wild Card (`8d0da2f9-0e05-4a94-8d42-fd57d06b09ba#card`) | 1000 | C |
| Suffocation (`57aeb106-dc4f-4069-b596-f997263dc0cf#card`) | 34944 | C |
| Sunbillow Verge (`a202276b-1f1b-4277-95ee-26877a204f5e#card`) | 2 | C |
| Sungold Sentinel (`410f5067-3380-4afb-9881-4b3bf0268493#card`) | 3220 | C |
| Surprise Deployment (`61a1748c-e772-4338-9edd-720cd5d2fc12#card`) | 12 | T |
| Svyelunite Priest (`fb7adbee-a3b0-42bf-988f-9d4faf6bd006#card`) | 4 | T |
| Tablet of Compleation (`6eca2ac9-16ba-4a11-b6ee-3caf2d7d8254#card`) | 49 | C |
| Tainted Field (`439de49b-1091-4688-9ffb-80a025df31c2#card`) | 2 | C |
| Tainted Isle (`0222414f-98b5-458a-a0fd-831a66cd8b07#card`) | 2 | C |
| Tainted Peak (`b2bae7fc-0668-4b34-9cd6-0d80aea52275#card`) | 2 | C |
| Tainted Wood (`fa6d05a1-3df4-4751-b1a0-8d9693faec73#card`) | 2 | C |
| Talara's Battalion (`08c7a6f5-7e25-4f8d-9e4a-aec7248e45fa#card`) | 5 | C |
| Tapestry of the Ages (`9248f9b9-ddaa-459c-91e6-9045a65b0f07#card`) | 5 | C |
| Tears of Rage (`564ffd21-bffb-488d-a496-cca29f344cdc#card`) | 16 | T |
| Tectonic Edge (`4927150d-7ff6-4232-b20e-d2ea245ac710#card`) | 2 | C |
| Teleport (`9efcf3d1-a0be-4a34-90c6-85a46b48fb83#card`) | 10 | T |
| Temple of the False God (`cfdd5dc6-593e-495a-8cfe-3a56b3c4c7df#card`) | 2 | C |
| Thornspire Verge (`e861bc08-4f0b-4d22-9b85-9d20227fd5b4#card`) | 2 | C |
| Tideshaper Mystic (`3eec0ca6-e461-4583-9989-e4f6b15a53f8#card`) | 30 | T |
| Training Compound (`99c70f4e-de8a-426d-99aa-17b2f87625ba#card`) | 8 | C |
| Treetop Defense (`f215d0f9-a53e-431a-a70d-9dc4e3caa41e#card`) | 28 | T (PP coordination) + C |
| Trenzalore Clocktower (`69143645-97b6-4c7c-9fa2-844fb3b99822#card`) | 4 | C |
| Undead Gladiator (`4da2425d-2118-4116-855d-2ef80693a7b7#card`) | 6 | T |
| Undercity Scrounger (`2750451f-348e-452f-88d0-bc921c46e8cc#card`) | 3 | C |
| Underhanded Designs (`870185fc-7b05-47d3-95ff-97fbf12796d4#card`) | 4 | C |
| Urtet, Remnant of Memnarch (`6222fccf-fc08-4190-8d40-a56d6d1423df#card`) | 4 | T |
| Urza's Workshop (`71099427-e110-488f-ab29-7867241fc7f0#card`) | 24 | C |
| Vedalken Certarch (`337533fb-2783-493c-856f-c41a83cca443#card`) | 4 | C |
| Vengeful Devil (`ba771caa-2cfb-4373-bbd9-6160b0543010#card`) | 6 | C |
| Veteran's Voice (`68d25ba4-d710-4090-bceb-5cfbd6bb63a1#card`) | 16 | C |
| Vivien's Jaguar (`32412a41-2929-4417-90bc-21c5659eeb72#card`) | 6 | C |
| Vona, Butcher of Magan (`b1c56e23-eae6-47a1-a1cd-3a2d01621768#card`) | 2 | T |
| Wake the Dead (`69bf79c3-87d1-4244-8d58-989307c91a12#card`) | 42 | T |
| Walker of Secret Ways (`451a3f52-9810-4137-93b1-9594e7727c29#card`) | 12 | T |
| Wand of Ith (`3ec1ccd7-4970-4032-be7a-9e76ca6a139e#card`) | 54 | T |
| Warrior's Stand (`1f08072d-7dec-4f0d-b465-dda62eb17630#card`) | 28 | T + C |
| Wastewood Verge (`c6e0574c-3e2b-4c40-b17a-05bce3d49309#card`) | 2 | C |
| Whisperer of the Wilds (`c1e456b6-5d2b-4c42-9464-54a8f9f8f3ba#card`) | 14 | C |
| Willowrush Verge (`c52eaa87-9251-4a47-83fd-04e582ade612#card`) | 2 | C |
| Wishclaw Talisman (`81c70ae7-3c18-4c9b-8505-e4db9e0e6518#card`) | 70 | T |
| Xerex Strobe-Knight (`2e150f97-5688-4559-94b8-86c33fda28a0#card`) | 35 | C |
| Yes Man, Personal Securitron (`16b8eab0-ccbd-4dbf-9998-82a4d85d0f77#card`) | 432 | T |
| Zhalfirin Decoy (`b22e55ef-fb74-4991-a43b-9d3ce06c23e5#card`) | 14 | C |
| Zuran Enchanter (`ee0f883f-d7c9-4acf-a78f-f733b6f268d3#card`) | 2 | T |


### Post-landing follow-up (2026-10-08)

The accepted review found two wrong Readings of *Activate only only during your
turn.* `FocusedUse` now distinguishes a phrase with its own Focusing Modifier:
`FocusedPrepositionPhrase` exports Yes and requires its head to export No.
Ordinary PP constructors and coordination export No; focus on a whole
coordination remains distinct from focus on its members. The guard reads a
declared feature, with no construction-name or word-name condition. The negative
is added to `focus_does_not_supply_missing_constituents_or_other_functions`.
The existing independent focusing roundtrip and lexical/traversal identity
assertions remain intact. The glossary now defines **Focused Use** as a project
term, attributed to this review rather than to CGEL. No new STOP or ruling
contradiction arose.

The reconciliation paragraph above records the review's 984 unread faces and
its overlapping restriction categories, including both prerequisites of the
predicative-*as* deferral. The nominal merge is explicitly disclosed as outside
the original ticket's letter, Reading-identical on 4,605 digest-only changes,
and undertaken to hold the construction count. The original assurance count now
states that the intermediate restoration left that test unchanged relative to
the claim parent.

Follow-up assurance: restored **0**, re-spelled **0**, ignored **0**, removed
**0**, added **0** test functions; **1** existing test strengthened with **1**
negative case. `cargo test -p deckmaste_english_v3` passed **383** tests, none
failed or ignored (`tests.log`); `cargo clippy -p deckmaste_english_v3 --tests --
-D warnings` passed (`clippy.log`); `cargo fmt --all -- --check` passed
(`fmt.log`). No lexicon data changed, no lexical-source package tests were
needed, and no Lean checks or xtask tests were run. Follow-up evidence is under
`target/english-v3/english-v3-focusing-followup/`.


The authorized **215-face selector** at
`sqzukyvnqplmsyxxrprltlkztpwtzkrs` / subset covered **215**, referenced to the
accepted full census `urxxrvzrurrtxvvqpmotlxvvtoyvwsrw` / covered **21,701**,
retains **44,615** exact Readings. `comparison.json` compares every selected face:
**0** count changes, **0** digest changes, **0** source changes, **0** losses,
and **0** issues, duplicates, cyclic derivations or internal failures. Every
counted Reading passes declaration admission, lexical ownership/context,
byte-exact realization and node/word traversal comparison. The independent
constructed-value roundtrip remains covered by the package tests. No full
census was run for this follow-up.

Command: `cargo xtask english-v3 --identity-manifest
target/english-v3/english-v3-focusing-followup/gained-selector.json --workers 12
--samples-per-face 0 --output
target/english-v3/english-v3-focusing-followup/after.json` (`selector.log`).
Selector corpus wall time **12,309,285,976 ns**; checked-text thread CPU
**665,610 ns/B**, **12** workers, host load `[7.4306640625, 10.130859375,
10.47119140625]`. These subset timings describe this tree and scope; the
whole-corpus performance advisory above remains the accepted full-run record.

On that measured tree the declared construction/schema count remains
**202 + 48 = 250** and Categories remain **146**. Nonblank declaration lines
are **3,379**. No lexical declaration, spelling, form literal or vocabulary
inventory changed; the lexical inventory hash matches the accepted full census.
The named homograph and overlap inventories above remain applicable. The new
licensing guard reads a declared feature; zero word-naming conditions were
introduced, and the existing environment load-error tests passed.
