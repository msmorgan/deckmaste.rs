---
needs: []
---
# Let a fixed keyword export its verb: this creature mutates / crews a Vehicle / exploits a creature

Priority: high. One mechanism, a batch of terms.

## Why

`english-v3-vocabulary-batch-1` found keyword terms that pass the
vocabulary-only test (their witness reads once the word is known) but have no
declaration path: the keyword is declared through `grammar: FixedKeyword(…)`
in its `plugins_v2` definition, and that row has no way to export the verb
the keyword is also used as. Unread faces touching each (blocker report,
trunk `vwlkmvns`): *crew(s/ed)* 36, *mutate(s)* 33, *paired* 27, *exploit(s)*
24, *saddle(s/d)* 16, *escape(s/d)* 16, *haunt(s)* 9; about 160 in all
(surface counts, not gain forecasts). By owner ruling these terms are declared
by the macro that owns them, never as free-standing core words.

## Goal

A `FixedKeyword` grammar row can declare the verb use of its keyword (lemma,
attested frames, attested forms by the regular paradigm), and the lexical
loader exports it as a verb lexeme owned by that definition. Then declare the
verb for each term above whose witness reads.

## Analysis

Orchestrator rulings 2026-10-10:

- KV1. Extend the existing grammar row (`crates/deckmaste_construction_core/src/macro_def.rs`,
  the `FixedKeyword` fields near `participial_adjective`) with one optional
  field for the verb use, reusing the verb-declaration shape `verbs.ron`
  already has (frames by the existing frame kinds). No new frame kind, Role
  or Construction; no grammar (`declarations.rs`) edit.
- KV2. The exported verb is one lexeme per keyword, owned by the definition.
  Where the keyword already exports a participial Adjective (*crewed*,
  *paired*), the passive/perfect participle of the new verb must not give any
  face two Readings for one constituency: if both would read the same string,
  keep the verb and retire the adjective export for that keyword, re-spelling
  its tests.
- KV3. Vocabulary-only test per term (ruling V2 of batch 1): substitute a
  declared verb with the same frame; if the substituted unit reads, declare;
  if not, list the term with the missing piece and move on. Do not STOP for a
  single term.
- KV4. Attested frames only (`cargo xtask english-v3 grep`): for example
  intransitive *mutates*, transitive *crews a Vehicle* / *exploits a
  creature*, passive *becomes crewed* / *is paired with*.
- KV5. Plugin bodies are untouched; only `grammar:` fields change.
  `cargo xtask facts check` after; `cargo xtask lean check` is not needed
  unless a non-`grammar:` field changed (it must not).
- KV6. Pre-integrate review ruling 2026-10-10: soulbond's active frame
  (Object + *with* + Object) is unattested in the analysed corpus; the new
  `You pair this creature with another creature.` Reading is accepted as the
  base of the attested passive (*is paired with*, *paired with it*), in the
  same way mechanically derived forms are exempt from pruning. This is a
  KV4 deviation by ruling, not a corpus attestation.

## Witnesses

Two attested units per declared term, chosen under KV3 and recorded with
before 0 and after ≥ 1 with the verb analysis.

## Method

Standard constraints apply. Bounded census: every supported face matching any
declared term's surface forms plus 300 random covered controls; zero lost;
zero increases on covered faces (KV2). Full suites and clippy for
`deckmaste_construction_core`, `deckmaste_lexical_source`,
`deckmaste_english_v3` and any other crate consuming the changed data. Tests
assert the exported lexeme's frames and each witness's verb constituent.

## Out of scope

- Predicate and event helper terms (*historic*, *outlaw*, *commit a crime*):
  a different loader exclusion (`BUILTIN_FAMILIES`), its own ticket.
- Terms with no owning definition (*modified*, *party*, *multikicker*).

## Landing record

1. **Census delta**: final bounded census on the closed source at `mpssvszw`, compared with the original claim-parent source (empty `surmqztl`, parent `yymumnyr`): **461 → 590 covered faces**, **51,561 → 14,730 Readings**. Native `diff --lines`: **129 gains, 0 lost, 194 decreases, 1 increase, 106 digest-only**, zero source-hash changes and issues. Lost faces: none. The own implementation at `surmqztl` added **99 faces**, with **0 lost/decreases/increases/digest-only** on previously covered faces; the first vocabulary refresh added The Last Ride (0 → 3). The review refresh adds **29 faces** after [dash-cost lines](../done/english-v3-keyword-dash-cost-line.md) (`kxwsyulpnrps`, completion `ovvlmoxprwns`) and [trailing Adjunct height](../done/english-v3-trailing-adjunct-height.md#landing-record) (`ukytykwrvlyr`, completion `ontmkkrswzow`) landed. Its refresh-only diff reports **226 decreases, 1 increase, 122 digest-only**, zero lost. All refresh decreases are attributed to `ukytykwrvlyr`'s accepted retired height classes A/B/C; they are not removed Readings caused by the keyword export. The increase is Malamet Veteran **7 → 9**, with the Existential Predicate PP carrier accepted under that landing's T1/T5 rulings; the current sampled document has exact roundtrips and zero issues. Identity manifest: all **394** supported faces matching candidate forms plus **300** covered controls, seed **20261008**, union **691** faces, **0** samples per face (`surmqztl` selector, unchanged at `mpssvszw`). Evidence under `target/english-v3/english-v3-keyword-verb-export/`: `manifest.json`, `before.json`, `after.json`, `diff.json`, `after-review-refresh.json`, `diff-review-final.json`, `diff-review-refresh-only.json`, `malamet-trunk-t5.json`.

2. **Witnesses** (`surmqztl`, before/after probes; complete enumeration). All substitution controls read before implementation. Each declared verb retains its definition as lexical owner and uses regular inflection.

   | Term / owner | Attested units (cards) | Before → after | Declared frames |
   |---|---|---|---|
   | crew / crew | `this creature crews a Vehicle` (Veteran Motorist); `This creature crews Vehicles` (Hotshot Mechanic) | 0 → 1; 0 → 1 | Predicate: Object NP |
   | mutate / mutate | `this creature mutates` (Dreamtail Heron); `this creature has mutated` (Huntmaster Liger) | 0 → 1; 0 → 1 | Intransitive Predicate |
   | pair / soulbond | `this creature is paired with another creature` (Wingcrafter); `it's paired with a creature` (Joint Assault) | 0 → 1; 0 → 1 | Predicate: Object NP + with + NP; existing Passive frame: with-marked NP Complement |
   | exploit / exploit | `this creature exploits a creature` (Fell Stinger); `it exploited that creature` (Silumgar Scavenger) | 0 → 1; 0 → 1 | Predicate: Object NP |
   | saddle / saddle | `This creature saddles Mounts` (Deathless Pilot); `saddles a Mount` (Reckless Velocitaur) | Before 0 / 0; deferred | Existing adjective must survive; see item 4 |
   | escape / escape | `This creature escapes with a +1/+1 counter on it` (Phoenix of Ash); `it escaped` (Kroxa) | 0 → 2; 0 → 1 | Intransitive Predicate; both licensed PP attachments retained |
   | haunt / haunt | `the creature this card haunts dies` (Cry of Contrition); `the creature it haunts dies` (Orzhov Pontiff) | 0 → 1; 0 → 1 | Predicate: Object NP, realized with an Object Gap |

   Additional attested modifier: `the exploited creature` (Henry Wu, InGen Geneticist), 0 → 1 when its declared Past Participial Premodifier licence is added (`surmqztl`).

3. **Spot checks** (`surmqztl`): ten newly covered complete faces, each with an exact roundtrip and zero issues in its inspected Reading. Dreamtail Heron and Vulpikeet: intransitive *mutates* trigger; Veteran Motorist and Speedway Fanatic: transitive *crews a Vehicle* trigger; Wingcrafter and Nearheath Pilgrim: passive *paired with another creature*, with selected Complement; Fell Stinger and Qarsi Sadist: transitive *exploits a creature* trigger; Cry of Contrition and Benediction of Moons: *haunts* in an Object-gap Relative Clause. Samples are `target/english-v3/english-v3-keyword-verb-export/spot-*.json`. An additional refresh sample at `surmqztl` checks The Last Ride: the gets clause with supplementary where clause, life-payment Activated Ability and unchanged Crew keyword all retain an exact roundtrip and zero issues.

4. **Deviations and STOPs**: no new Construction, Role or frame kind, no `declarations.rs` edit, no plugin body changes, no glossary gap, no STOP. Refresh produced one test-registration conflict; both the trunk vocabulary module and this keyword module were retained, and refresh then succeeded. Under the project rulings [KV1–KV6](#analysis), the existing core frame-kind schema is shared with the keyword export. `core.ron` supplies the existing Passive frame for pair's selected *with* Complement and the attested Past Participial Premodifier licence for exploit. Saddle is deferred under KV3/KV2: adding its verb changes `is saddled` from 1 to 2 Readings; retiring the adjective makes `becomes saddled` drop from 1 to 0 (`surmqztl` probes), threatening already-covered Stubborn Burrowfiend; Kolodin, Triumph Caster; Guidelight Matrix; and Alacrian Armory. Its active witnesses temporarily read, but the export was removed before the landing census and the original adjective retained. Follow-ups: saddle's state/verb overlap and *becomes crewed* → [systemic residuals](../planned/english-v3-systemic-residuals.md#keyword-verb-and-state-complement-residue-2026-10-10). *Becomes crewed* is a KV3 failing term: the 4 attested faces (Mindlink Mech, Mighty Servant of Leuk-o, Mobilizer Mech, Ghost Ark) remain 0 before and after (`surmqztl`, pre-integrate review); *become* takes an adjectival Complement, crew exports no state adjective, and adding one would duplicate *is crewed* as for saddle. KV2's retirement clause had no subject: none of crew, mutate, soulbond, exploit, escape or haunt had a `participial_adjective` export, hence 0 re-spelled tests. *Crewed by X* reads 2 through the same pre-existing By-phrase alternation as trunk's *blocked by X*. Under KV6, soulbond's unattested active Object + *with* + Object frame is accepted as the base of the attested passive; `You pair this creature with another creature.` now reads 1 (`surmqztl`). This is a KV4 deviation by ruling. Ordinary consequences of exporting Verb Lexemes include `Crew a Vehicle.`, `Mutate.`, `Exploit a creature.` and `This Vehicle is crewed.` reading. [Keyword dash-cost lines](../done/english-v3-keyword-dash-cost-line.md) landed on trunk as `ovvlmoxprwns`; the supplied `Escape—{3}{R}{R}, Exile three other cards from your graveyard.` line now has **3 complete Readings**, all exact roundtrips, zero issues (`mpssvszw`, `escape-dash-review.json`). Its prior dash-line blocker is discharged; the new full-face gains are recorded in item 1. The original escape fragment still has its 2 complete Readings after refresh (`escape-fragment-final.json`). The incoming trailing-Adjunct Reading retirements and T5 carrier increase are attributed in item 1 to their trunk landing, not to this implementation. Beyond the witness/export assertions, two schema tests check validation/serialization, an independent mutate test checks both roundtrip laws, and the deferred saddle regression test preserves the existing state analyses. The unsupported selected escape-with-on candidate was removed from its grammar data; the attested intransitive route remains.

5. **Tests** (`surmqztl`): `cargo test -p deckmaste_english_v3 -p deckmaste_construction_core -p deckmaste_lexical_source` passes all full suites; `cargo clippy -p deckmaste_english_v3 -p deckmaste_construction_core -p deckmaste_lexical_source --tests -- -D warnings` passes. Initial schema roundtrip test failure was corrected by using matching macro RON options for serialization and reading, retaining the exact value assertion. Initial clippy findings were fixed by borrowing the core declaration, extracting keyword verb export and using let-else in the witness test; full suites and clippy were rerun and passed. `cargo xtask facts check` passes; cite checks report 0 noncompliant strings and 0 stale citations, and bless registers no new rules. After the grammar/data-bearing refresh, the same full test and clippy commands pass at `surmqztl`; logs are `tests-refreshed.log` and `clippy-refreshed.log`. Assurance counts: restored 0 / re-spelled 0 / ignored 0 / added 7 / removed 0. No removals to justify. The existing landwalk metadata assertion gains only the new optional field's `None` value. Review at `mpssvszw` adds `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings`: both pass (407 unit, 4 binary and 14 integration tests). The inherited ignored `macros::templates::tests::macro_schema_census_count_matches_21` is the live-corpus/census cross-check marked run-on-demand; no ignore was added. Logs: `xtask-tests-review.log`, `xtask-clippy-review.log` in the same evidence directory. `deckmaste_semantics_v2` need not be run because its reader discards grammar metadata. After the review grammar/data-bearing refresh at `mpssvszw`, `cargo test -p deckmaste_english_v3 -p deckmaste_construction_core -p deckmaste_lexical_source` and `cargo clippy -p deckmaste_english_v3 -p deckmaste_construction_core -p deckmaste_lexical_source --tests -- -D warnings` both pass in full, with zero failures or ignored tests. Logs: `tests-review-refreshed.log`, `clippy-review-refreshed.log`.

6. **Timing** (`mpssvszw`): the final bounded census took **11.23 s**, **6 workers**, under `/usr/bin/time -v`.
