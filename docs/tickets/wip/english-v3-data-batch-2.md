---
needs: []
---
# Data batch 2: missing frames, nouns, compounds and plugin frame data that existing routes already read

Priority: high. Batch data ticket (owner decision 2026-10-10: batching applies
to declarations an existing Construction already reads). Source: the blocker
report at trunk `vwlkmvns` (5,792 unread faces) and its 2026-10-10 triage;
every item below has a measured fail/control probe pair. Face figures are
sole-blocker upper bounds, not forecasts.

## Goal

Make each item read through routes that exist today, by declaration only.

| # | Declaration | Failing probe | Control that reads | Faces |
|---|---|---|---|---:|
| 1 | Verb *end*: frames `[]` and `[ObjectNounPhrase]` | "End the turn." / "You don't lose this mana as steps end." | "Sacrifice the turn." | ~29 |
| 2 | Common noun *flip* (the verb exists) | "If you lose the flip, draw a card." | "If you win the game, draw a card." | 43 |
| 3 | Common noun *total* and the compound *life total*, if batch 1 left a gap | "X is your life total." | "…equal to your life." | ≤44 |
| 4 | Compound *mana ability* | "It is a mana ability." | "It is an ability." | 40 |
| 5 | *remove*: add `Predicate([ObjectNounPhrase])` | "Add {R} for each counter removed this way." | "…each creature sacrificed this way." | 35 |
| 6 | Source Complement admits the *combat* turn part | "Remove it from combat." | "Remove a counter from this land." | 17 |
| 7 | *put*: frame Object + *back* (optionally followed by the manner or locative phrase its other frames take) | "Put them back." | "Put them on top of your library in any order." | 32 |
| 8 | Locative pro-form *there* as a Locative Complement | "It was put there." | "It was put onto the battlefield this turn." | 31 |
| 9 | *game*: `NominalAdjunctClass: Temporal` | "You cast a spell this game." | "You cast your commander this turn." | 20 |
| 10 | *any* as a temporal-adjunct dependent (bare *any time*) | "You may look at the top card of your library any time." | "…at any time." | ~35 |
| 11 | *devotion*: noun Complement *to <colour(s)>* | "Your devotion to black is 5." | "Its power is 5." | 20 |
| 12 | `transform.ron`: frames `[]`, Object and *into* NP | "When this permanent transforms, draw a card." | "When this permanent becomes …" | 37 |
| 13 | `surveil.ron`: add the bare frame `[]` | "Whenever you surveil, …" | "Whenever you scry, …" | ~10 |
| 14 | `typecycling.ron`: supertype + type stem (*basic land*) | "Basic landcycling {2}" | "Landcycling {2}" | 34 |

## Analysis

Orchestrator rulings 2026-10-10:

- D1. An item is in scope only while it stays a declaration. If one turns
  out to need a Construction, a frame kind, a Role, a feature or a loader
  change, drop it to the follow-up list with the evidence and continue; do
  not STOP for a single item.
- D2. Item 5: "A counter was removed from this land." and "This creature is
  removed from combat." also read 0. Diagnose in one paragraph whether the
  passive of *remove*'s Source frame is a data gap (fix it here) or a grammar
  gap (follow-up list).
- D3. Items 12–14 edit only `grammar:` fields of `plugins_v2` definitions;
  plugin bodies are untouched. `cargo xtask facts check` after.
- D4. Attested frames and categories only; the short declaration form;
  mechanically derived forms are exempt from the pruning rule (R16e).
- D5. One lexical reading per use: no Reading-count increase on a previously
  covered face. An increase is a defect: narrow the declaration or drop the
  item.

## Method

Standard constraints apply. Deltas: one bounded census after all
declarations on every supported face matching any item's surface pattern
(`cargo xtask english-v3 grep -i --names`, unioned) plus 300 random covered
controls, `--workers 6 --samples-per-face 0` under `/usr/bin/time -v`, compared
with `cargo xtask english-v3 diff` against the claim parent. Zero lost, zero
increases on covered faces. Tests: one table-driven test, each row pinning
the new declaration's role (not the whole unit), plus lexical-source tests per
declaration class. Full suites and clippy for every crate whose code or
consumed data changed. No grammar edits.

## Landing record

Six items as specified in `docs/tickets/README.md`; item 1 adds a per-item
table (item, where declared, faces gained) and item 4 the follow-up list.

1. **Census delta** — before refresh, `vwlkmvns` → `owzsyvqn`, bounded to
   11,743 identities:
   11,545 surface matches plus 300 covered controls (seed 20261008; 198
   controls outside the surface). Covered faces 9,084 → 9,394; total Readings
   444,958 → 463,064. Gained 310; lost none; decreases 0; increases 0;
   digest-only 0. Before refresh, the claim parent had the same grammar and
   lexicon as the full `vwlkmvns` census. Evidence:
   `target/english-v3/english-v3-data-batch-2/`
   (`manifest.json`, `after.json`, `diff.json`). Surface selector:
   `\b(end(?:s|ed|ing)?|flips?|totals?|mana abilit(?:y|ies)|remov(?:e[sd]?|ing)|combat|back|there|games?|any|devotion|transform(?:s|ed|ing)?|surveil(?:s|ed|ing|led|ling)?|[a-z]*cycling)\b`.

   After refresh, `vwlkmvns` → `kpvllwpt`, the same 11,743 identities:
   covered 9,084 → 9,596; total Readings 444,958 → 489,188; gained 512;
   lost/decreases/increases/digest-only all 0. The built-in comparison of
   `owzsyvqn` before refresh → `kpvllwpt` after refresh gains 202 faces,
   loses none and changes no previously covered Reading set. Those 202
   gains come from the incoming vocabulary changes (`nouqyyouuops`,
   `qmqmopsronzl`); there are no trunk Reading decreases to attribute.
   Evidence: `after-refresh.json`, `diff-refresh.json`,
   `diff-refresh-vs-own.json`.

   Per-item figures on `owzsyvqn` before refresh and `kpvllwpt` after refresh
   count gained faces attesting the declared
   surface, from supported-corpus `english-v3 grep` identities intersected
   with the built-in diff's gained identities. These overlap; they are not
   separate ablation censuses or additive attribution. Narrowed patterns and
   counts are in `item-gain-counts.json`.

   | Item | Where declared | Faces gained with this surface, before → after refresh |
   |---|---|---:|
   | 1: *end* | `verbs.ron`, bare and Object frames | 35 → 39 |
   | 2: noun *flip* | `vocabulary.rs`, Count noun | 44 → 47 |
   | 3: *life total* | `core.ron`, native multiword Count noun; inherited from `nouqyyouuops` after refresh; standalone *total* deferred | 32 → 42 |
   | 4: *mana ability* | `core.ron`, compound recipe with *ability* head | 34 → 36 |
   | 5: passive Source frame | `core.ron`, explicit Passive frame; bare transitive deferred | 9 → 11 |
   | 6: *combat* Source Complement | deferred; trial Location licence retired | 0 → 0 |
   | 7: *put back* | `core.ron` Preposition; `verbs.ron` back + locative frame; existing bare/manner frames | 34 → 36 |
   | 8: locative *there* | `core.ron`, intransitive Preposition | 13 → 13 |
   | 9: temporal *game* | `core.ron`, Temporal noun adjunct class | 16 → 17 |
   | 10: bare *any time* | `core.ron`, Temporal dependent licence | 42 → 42 |
   | 11: *devotion to* | deferred | 0 → 0 |
   | 12: *transform* | plugin `grammar:` bare and *into* frames; Object frame retained | 37 → 38 |
   | 13: bare *surveil* | plugin `grammar:` bare frame; measure frame retained | 20 → 28 |
   | 14: *basic landcycling* | deferred | 0 → 0 |

2. **Witnesses** — probes before declarations on claim parent `pwpypkvo`
   confirmed every failing witness except the already-supported `Transform
   it.` (1 Reading); it was retained as a control. Final counts on
   `owzsyvqn`: “End the turn.” 1; “You don't lose this mana as steps end.” 3;
   “If you lose the flip, draw a card.” 1; “X is your life total.” 1;
   “It is a mana ability.” 1; “Add {R} for each counter removed this way.” 0
   (deferred); “A counter was removed from this land.” 1; “Remove it from
   combat.” 0 (deferred); “This creature is removed from combat.” 0
   (deferred); “Put them back.” 1;
   “Put them back in any order.” 1; “Put them back on top of your library in
   any order.” 1; “It was put there.” 1; “You cast a spell this game.” 2;
   “You may look at the top card of your library any time.” 8; “Your devotion
   to black is 5.” 0 (deferred); “When this permanent transforms, draw a
   card.” 1; “Transform it.” 1; “It transforms into a creature.” 1;
   “Whenever you surveil, draw a card.” 1; “Basic landcycling {2}” 0
   (deferred). All paired controls retain their before counts, including
   “Remove a counter from this land.” 1. Evidence: `before-probes.log`,
   `pass-1-probes.log`, `pass-2-probes.log`, `pass-3-probes.log` and the
   corresponding probe JSON files. Re-probed all ticket fail/control pairs
   on `kpvllwpt` after refresh: every count is unchanged
   (`probes-refresh.log`).

3. **Spot checks** — `owzsyvqn`, one selected Reading per face, byte-exact
   realization and no issues (`spot-0.json` through `spot-9.json`): Time Stop,
   imperative *end* + Object “the turn”; Sundial of the Infinite, the same
   predicate inside an activated ability; Bottle of Suleiman, Count noun
   *flip* as *win*/*lose* Object; Burning-Tree Shaman, compound *mana ability*
   as the copular complement inside a relative; Channel, Temporal “any
   time” with an adverbial-gap relative and *activate* Object “a mana
   ability”; Halimar Depths, *back* Locative Complement plus “any order”
   Manner Complement; Sage Owl, the same two complements in its trigger;
   Ancestral Knowledge, *back* marker, “on top of your library” Locative
   Complement and “any order” Manner Complement; Abolisher of Bloodlines,
   *transform* with selected *into* + proper-name NP; Empyrial Storm,
   Temporal “this game” within the commander-casting relative.

4. **Deviations and STOPs** — no STOPs or glossary gaps. D1 follow-ups:
   standalone *total*, bare transitive *remove*, *combat* Source Complement,
   *devotion to*, and *basic landcycling*. The trial *total* noun gave
   Cryptborn Horror 51 → 66 and
   Tainted Sigil 15 → 21 Readings, through new object-gap relative analyses
   of “the total life lost …”; it was dropped under D5, leaving *life total*
   as a native multiword declaration. The one bounded scan was repaired
   with the built-in incremental census on the identical manifest (244
   faces rechecked, 11,499 reused); its final built-in diff has no covered
   increases. This is no claim that CGEL rejects those relatives: the
   preservation constraint is the project's D5 ruling.

   D2 diagnosis: *remove*'s active Source frame already existed, but its
   passive realization lacked an explicit Passive + Source frame: a data
   gap fixed here. A trial bare Object frame made “Remove a counter from
   this land.” increase 1 → 4 via competing Source attachments; that frame
   was retired and the active control restored to 1. The bare-passive
   “removed this way” witness remains unread and needs grammar narrowing
   before the bare Object frame can satisfy D5. *Devotion to* needs a
   nominal Complement marker beyond the currently declared None/Of and the
   existing *of* route. *Basic landcycling* needs a compound supertype/type
   stem beyond the bound-stem schema and single-stem grammar. Neither is
   implemented in this data batch. The trial *combat* Location licence also
   admitted bare “combat” as an ordinary Noun Phrase, failing the existing
   `combat_intervals::combat_damage_keeps_its_premodifier_analysis` test.
   It was retired, leaving that test unchanged and both *from combat*
   witnesses unread. Supporting Source without admitting the ordinary bare
   NP needs grammar narrowing. The same-manifest incremental repair
   rechecked 1,817 faces and reused 9,926; the parent comparison loses no
   previously covered face or Reading.

   *Back*'s obsolete Adverb declaration was replaced with the intransitive
   Preposition declaration; no grammar route named *back* existed to retire.
   The generic Adverb routes remain for their other members. Dummy *there*
   remains distinct from locative *there*. CGEL Ch. 7 §2.4, pp. 614–615
   supports only the Preposition classification of *back* and locative
   *there*; the frames and licences follow this ticket's project rulings.
   Plugin bodies were untouched; `cargo xtask facts check` passed on
   `owzsyvqn`. No grammar, loader, frame-kind or feature-schema edits.
   Refresh brought vocabulary batch 1 (`nouqyyouuops`, review rulings
   `qmqmopsronzl`). Its *life total* declaration is byte-identical to this
   batch's declaration. Refresh merged both locations; this workspace's
   duplicate was retired, keeping trunk's declaration and the tests of its
   values. Item 3's pre-refresh count remains in the table; it is inherited
   coverage after refresh, not an additional declaration in this landing.

5. **Tests** — `owzsyvqn`: `cargo test -p deckmaste_english_v3` passed
   (731 integration tests, no failures or ignores), including the final
   full re-check after moving the unchanged role table into a constant for clippy.
   `cargo test -p deckmaste_lexical_source` passed (28 unit and 40 integration
   tests, no failures or ignores). `cargo clippy -p deckmaste_english_v3
   --tests -- -D warnings` and `cargo clippy -p deckmaste_lexical_source
   --tests -- -D warnings` passed. Added 5 tests (one
   English table with 15 role rows, three preservation negatives and
   independent admitted values; four
   lexical-source tests for frames, nouns, compound inheritance and feature
   licences). Restored 0 / re-spelled 2 / newly ignored 0 / removed 0.
   `generic_predicates::sun_droplet_remove_retains_object_and_marked_source_roundtrip`
   now compares the complete active and passive frames instead of the old
   one-frame count; its Sun Droplet Reading, realization and word-value
   assertions are unchanged.
   `keyword_payload_features::complete_put_and_look_frames_preserve_their_typed_np_slots`
   expects seven *put* frames rather than six; its complete Boldwyr
   Heavyweights and Genesis Ultimatum frame values are unchanged. Initial
   suite failures exposed the retired combat licence and these two obsolete
   inventory counts; all were corrected before landing. No existing tests
   were removed or weakened.
   After refresh on `kpvllwpt`, the same full `cargo test -p
   deckmaste_english_v3` command passed 732 integration tests; `cargo test
   -p deckmaste_lexical_source` passed 35 unit and 40 integration tests.
   No failures or ignores in either suite. The same lexical-source clippy
   command passed again after correcting the test name to “declared plural”;
   its assertions are unchanged.

6. **Timing** — `owzsyvqn`: 4m05.31s for the bounded census, 6 workers,
   `--samples-per-face 0`, measured with `/usr/bin/time -v` (`census.time`).
