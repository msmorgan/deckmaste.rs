---
needs: []
---
**Generate the Lean workbench's facts tables from the registry declarations.**
`lean/Semantics/Check/Words.lean` and `Check/Keywords.lean` carry four
hand-written tables the checker's laws read: `actFacts` (keyword actions, the
deed table), `keywordFacts`, `counterFacts`, and `designationTable`, plus the
small `subtypeFacts` (Saga, Adventure, Room frames). Each is a second copy of
knowledge the registry owns under `plugins/builtin_v2/macros/stubs/`
(`keyword_actions`, `keyword_abilities`, `counter_kinds`, `designations`,
`subtypes`), and the copies drift: the 2026-09-05 review found the designation
rows wrong about carrier (creature-held where the CR says permanent), casing
(`Alpha sector` versus the CR's `alpha sector`), and conferrer (Storied,
Monstrosity). That is the drift
[conferrals-come-from-registries](../../decisions/conferrals-come-from-registries.md)
forbids: "a new declared name must not require a corresponding hard-coded
behavior branch".

The Idris workbench already had this fixed for one table: `cargo xtask facts
generate` writes `idris/src/Experimental/FactsGen.idr` (the keyword facts) from
the keyword-ability stubs plus a hand-kept gate-column overlay in
`crates/xtask/src/facts.rs`, and `facts check` fails when the committed module
is stale. Do the same for Lean, for all four tables:

- `cargo xtask facts generate` also writes `lean/Semantics/Check/Facts.lean`
  (one generated module; `Check/Words.lean` and `Check/Keywords.lean` import it
  and lose their literal tables). Kernel `decide` still evaluates the tables, so
  they stay Lean data; only their authorship moves.
- Columns the stubs do not carry (deed roles, `stepwise`, `feature`,
  `counterfactual`, `opponentsLibrary`, the keyword regime and gate columns) stay
  in xtask's overlay, keyed by label, as they are for the Idris table today. A
  stub with no overlay row and a column the checker needs is a generation error,
  as it is now.
- `facts check` covers the Lean module, and `./scripts/build` keeps running the
  `Proofs/Tables.lean` uniqueness pins over the generated tables.
- `KeywordFacts.confers` / `ActFacts.confers` (a designation's conferrer, landed
  2026-09-05) come from the stubs' `DesignationDecl` links, not the overlay. Two
  conferrers name a *set* of designations (Space Sculptor: alpha/beta/gamma
  sector; Unlock: left/right half), so the column is a list, not an `Option`.

Decisions already made that bind this: the Lean is the successor workbench and
the Idris is the reference only (the Idris `FactsGen` can be dropped when the
Idris goes); laws read declared features, never a lexeme, so any string the
generator emits is a table key, not something a guard may name; Vintage-playable
Magic only, so the registry's scope is the table's scope and no row is reserved
for anything outside it.

## Landing record

Change `uomrqrvvtnmzllxkuvtpznlssnywtzyx`; English lock `covered`: 20,254.

**PROVE:** `Facts.lean` is generated from normalized builtin_v2 declarations
and checker-column overlays. `FactTypes.lean` owns the shared record types;
Words and Keywords retain their interpreting laws and lose the literal tables.
Both `facts generate` and `facts check` cover Lean and reference Idris output.
Missing keyword/action/designation overlays and undeclared conferrers fail
generation. Parameter shapes are read through the declaration parser, including
multiline fields. `DesignationDecl.conferrers` survives Rust lowering and
supplies list-valued conferrals for keyword abilities, keyword actions and
closed core deeds.

A kernel-evaluated comparison against the parent tables found no lost keys
and identical facts for all 65 actions, 56 named counters, 22 designations
and 3 frame subtypes. Of 197 keyword rows, only Space Sculptor changes: its
three sector conferrals now come from the declaration. Core Unlock's facts
also expose both declared door conferrals. All six original table-integrity
pins remain. The full Lean build passed 69 jobs without warnings; LSP
reports no diagnostics, and the audited sector-conferral theorem uses only
`propext`.

Assurance: 13 tests added (7 generator mutation/failure tests, 1 full-value
lowering test, 5 Lean conferral pins); 10 existing tests re-spelled for the
extended declaration census, quoted enum labels, new record field, and
redundant wildcard cleanup; 0 restored, newly ignored, or removed. The
lowering ability test module was relocated byte-for-byte after production
items to satisfy Clippy. Every original assertion is retained.

**DISCLOSE:** Forty grammarless counter declarations supply names that the
old Lean table already admitted but the registry lacked: BloodCounter,
BloodstainCounter, BountyCounter, BrickCounter, DepletionCounter,
DivinityCounter, DoomCounter, DreamCounter, EggCounter, FinalityCounter,
FloodCounter, FungusCounter, FuseCounter, GrowthCounter, HoneCounter,
HourCounter, IceCounter, InterventionCounter, KiCounter, LevelCounter,
OmenCounter, PageCounter, PlagueCounter, PlanCounter, QuestCounter,
RadCounter, RevCounter, ScreamCounter, SleightCounter, SlimeCounter,
SoulCounter, SpiteCounter, StashCounter, StorageCounter, StrikeCounter,
StudyCounter, SuspectCounter, TideCounter, WindCounter and WishCounter.
RadCounter declares player scope; the others declare object scope. The
existing dedicated power/toughness and keyword counter forms remain outside
the named-counter table. No English grammar recipe is added.

Deviations and additions: the missing registry declarations preserve existing
Lean acceptance; typed core-deed/conferrer metadata enables the requested
links without a Rust core/semantics dependency cycle; DayNight and Sector enum
members now use the quoted Ident representation the typed loader requires.
Existing keyword scope exemptions and missing-column overlays are retained.
The glossary defines Designation Conferrer. Downstream census assertions were
extended, and pre-existing lowering lint violations were fixed mechanically.
This change claims no new card acceptance or retirement, and introduces no
lexeme-name guard. No unresolved glossary gap or ruling contradiction was found.

The full-corpus comparison triggered a STOP for two identities still in the
coverage lock: `1d12969b573ddeab74744733349723633eaded98bf64b4cf0694a3a6b152af56`
(Start the TARDIS) and
`8285bf929bff5f9be66997c1eae3405964ca1e901f8467b557b0d51e0e21d49a` (TARDIS).
Both fail at “planeswalk”. Inspection resolved this as inherited scope
retirement: ancestor change `rqutzrtywuvmwtnlwlvkwoltzsqutszp` deliberately
removed `Planeswalk.ron` and other out-of-scope actions. Classification:
wrong analysis retired under the recorded Vintage-only scope, not a new loss
introduced here. The lock remains unchanged. The first corpus run used the
legacy ratchet default; a second full coverage run is required to exercise the
project's normal `DECKMASTE_COVERAGE_LOCK=report` policy. That invocation
correction, rather than an implementation iteration, is the reason for the
second full coverage pass.

**REPORT:** The refreshed `cargo xtask gate --changed --run` passed the
reverse-dependency closure:

```text
cargo test -p deckmaste_construction_core -p deckmaste_construction -p deckmaste_core -p deckmaste_card -p deckmaste_english_v2 -p deckmaste_semantics -p deckmaste_lowering -p deckmaste_plugin -p deckmaste_engine -p deckmaste_legacy_render -p deckmaste_migrations -p deckmaste_noncanon -p deckmaste_spelling -p deckmaste_tui -p deckmaste -p xtask
```

The closure ran 4,602 passing tests, with 6 pre-existing ignored tests and
no failures. The same closure passed `cargo clippy --all-targets -- -D warnings`.
`facts check` reports both modules current. Full-corpus roundtrip and ambiguity
checks passed: 20,252 selected units, 0 mismatches, 0 unresolved ties and
0 internal failures. Selection census: 17,197 unique and 3,055 resolved by
specificity, with 0 selection exceptions. The parent selection census was not
remeasured: this change introduces no grammar or selection rule, and claims
no selection delta. Report-mode coverage passed: 20,252 covered units, no
newly covered identities, and exactly the two inherited retirements listed
above. It reports zero ownership, construction-traversal, leaf-traversal,
roundtrip, unresolved-tie or internal failures. The unchanged lock records
20,254 covered identities.

Performance advisory: the final coverage command took 176 seconds (rounded) against
the 16.26-second quiet-host ceiling, using 4 workers; accepted per-byte
thread CPU was 142,846 ns/B. Host load was 5.03 / 6.36 / 5.75 (1 / 5 / 15
minutes), with other checks running on the host; this is not a quiet-host
performance claim. All measurements above are on the stamped change and lock
count at the top of this record. Citation checks report 0 noncompliant and
0 stale citations; all 24 changed citation sites were read against their
rule text.

The current English inventory contains 397 constructions, 24 permitted
licensing checkers, and 0 forbidden checkers. Homograph inventory:
`AttributiveAdjective::Untap` / keyword-action verb `Untap`, and
`TargetingMarker::Target` / noun `CommonNoun::Target`. Form-literal/vocabulary
overlaps: `additional` in `additional_cost`; `to` in
`up_to_quantifying_determiner`; `the` and `next` in
`definite_next_mass_quantity_reference`; `to` in
`scalar_less_than_or_equal_to`; `the` in `number_of_scalar_value`;
`the` in `greatest_scalar_value`; `other` in `other_than_qualified_reference`;
`the` in `positional_partitive`.

