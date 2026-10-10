---
needs: [english-v3-per-line-readings]
---
# Trailing adjuncts attach at VP level: never over a sentence-initial trigger or intervening *if*; clause level only for clause coordination

Priority: medium. Owner decision (2026-10-09): three attachment classes are
restricted by declared feature, in this order, once the per-line census
(`english-v3-per-line-readings`) has landed:

1. this ticket;
2. `english-v3-stray-on-it-adjunct`;
3. `english-v3-temporal-pp-and-premodifier-scope`.

The retained-Reading contract in `docs/decisions/english-lexical-analysis.md`
("Chart admission and ambiguity", "Pruning rule for licensed functions and
attachments") still holds. A restriction may retire only Readings that
Oracle's conventions exclude. Every attested counter-example is a named
control and must keep reading.

## Why

A Clause-final Preposition Phrase or Nominal Adjunct (*this turn*, *until end
of turn*, an *if* PP) currently attaches at every height:

- at VP level, through the `PrepositionPredicate` and
  `NominalAdjunctPredicate` rows;
- at Clause level, through `ClausalPreposition`, including over an
  `InitialPreposition` host whose preposed dependent is the trigger
  (*Whenever …, …*);
- at VP level stacked over a predicate whose last dependent is an *if* PP.

The 2026-10-09 attachment decomposition measured this on eight high-count
faces. That evidence is the orchestrator's scratch and is not in the tree.

- **b-trig** (an Adjunct scoping over the sentence-initial trigger)
  multiplies a sentence's count by ×1.0–1.8 and occurs in 5.1% of sentences.
  The decomposition found no counter-example.
- **b-clause** (Clause-level attachment over a non-coordinated Clause) adds
  ×1.0–2.5. At either height the meaning is the same.

Oracle convention: a trailing Adjunct modifies the predicate it follows. Its
scope never includes the trigger condition or a preceding *if* condition. At
minting, 1,402 faces match
`cargo xtask english-v3 grep '\b(Whenever|When) [^.]*, [^.]* (if [^.]*|this turn|until end of turn)\.'`.
Two corpus examples:

- Belladonna Took: "Whenever a token you control enters, you gain 1 life if
  this is the first time this ability has resolved this turn." The *if* PP
  conditions *gain*; *this turn* belongs inside the *if* Clause.
- Vizier of Deferment: "When this creature enters, you may exile target
  creature if it attacked or blocked this turn." *this turn* belongs to
  *attacked or blocked*, not to the trigger or the *if*.

Same meaning at both heights: Veil of Summer, "Draw a card if an opponent has
cast a blue or black spell this turn."

CGEL (Ch. 6 §7, [5]–[7], p. 576, page-checked) lists temporal location and
duration among the VP-oriented adjuncts ([5v–vi]). It states that these are
"more closely associated with the VP constituents, and more likely to be
positioned in the VP or adjacent to the VP" ([7i]). Its clause-oriented list
([6]) has domain, modality, evaluation, speech-act-related and connective
adjuncts only. The passage is about AdvPs. Applying it to trailing PPs, to NP
adjuncts such as *this turn*, and to conditional *if* PPs is this project's
analysis and is not stated by CGEL. Cite it only for what it says.

## Goal

1. `ClausalPreposition` admits its dependent only when the host Clause is a
   Clause Coordination. The coordination may be direct, or reached through a
   stack of `ClausalPreposition`s over one. Clause-level attachment over a
   simple Clause or over `InitialPreposition` is retired. The VP-level rows
   carry the attachment instead.
2. A trailing Adjunct does not attach over a predicate whose last dependent
   is a clause-complement PP (*if*, *unless*, …). The Adjunct belongs inside
   that Clause.
3. Attachment above a modal is untouched (see Out of scope).

## Analysis

Line numbers were verified at minting against trunk `zunvpkot`, which does not
change `crates/deckmaste_english_v3/src/declarations.rs` relative to
`wnrmklzs`. Verify them against the claim tree.

- `InitialPreposition` `:2346–2355`, `ClausalPreposition` `:2357–2365`,
  `ClauseCoordination` `:2376–2382`. The Clause-series rows of
  `CoordinationSeriesEnd` start at `:2385`, with `ClauseSeries` at
  `:2412`/`:2453`/`:2490`.
- `PrepositionPredicate` schema `:1594–1601`, rows `:3827–3834`.
  `NominalAdjunctPredicate` schema `:1579–1581`, rows `:3714–3721`.
  `ClauseComplementPreposition` `:3308`.

Implement with declared features and tables only. Add no new Construction and
no guard naming a lexeme, Construction or card. If either item seems to need
one, STOP.

1. **Clause height.** Add a Clause feature recording whether the Clause is a
   coordination host, for example `{Coordinated, Simple}`:
   - `ClauseCoordination` and the Clause-series coordination rows export
     `Coordinated`;
   - `ClausalPreposition` forwards its `clause` value, so stacked Adjuncts
     over a coordination still read;
   - every other Clause Construction exports `Simple`. That includes
     `InitialPreposition`, so a trailing Adjunct can no longer scope over the
     trigger.

   `ClausalPreposition` requires `Coordinated`.
2. **Intervening *if*.** A PP-level feature marks a clause-complement PP. The
   source is `ClauseComplementPreposition`, or the head's declared
   `FiniteClauseComplement`/`VerblessClauseComplement` properties; it must
   not name *if*. The `PrepositionPredicate` rows export it as the host's
   last-dependent property. The `PrepositionPredicate` and
   `NominalAdjunctPredicate` rows then reject a modifier over a head
   carrying it, through a table.

Name both features from glossary terms. Add the entries to
`docs/contexts/oracle-english/CONTEXT.md` in the same landing; the glossary has
no term for adjunct height today.

## Witnesses

These are Sentence-category probes for single-sentence lines, so each line's
per-line count equals its sentence count. Before = probe at minting, using the
provisioned `cargo-xtask` with trunk `zunvpkot`'s lexicon. After = expected
count from a tree classifier over those Readings. The classifier drops every
Reading that has `ClausalPreposition` over a non-coordinated Clause, or a VP
Adjunct over a predicate ending in `ClauseComplementPreposition`. It is an
estimate, not a grammar run. If a sibling attachment ticket lands first,
re-probe the before counts.

Counts must FALL:

1. Goliath Daydreamer, "Whenever this creature attacks, you may cast a spell
   from among cards you own in exile with dream counters on them without
   paying its mana cost.": 1,194 → 323.
2. Goliath Daydreamer, "Whenever you cast an instant or sorcery spell from
   your hand, exile that card with a dream counter on it instead of putting it
   into your graveyard as it resolves.": 1,050 → 140.
3. Belladonna Took, the sentence above: 84 → 24. Of the 84, 48 carry a
   Clause-level attachment (24 of them over the trigger) and 12 are VP over
   *if* only.
4. Vizier of Deferment, the sentence above: 25 → 15.
5. Veil of Summer, the sentence above: 5 → 2.

Also expected: Wyluli Wolf, "Target creature gets +1/+1 until end of turn."
2 → 1; Suffocation, "Cast this spell only if you were dealt damage this turn
by a red instant or sorcery spell." 48 → 21; Scarlet Witch, Chaotic Avenger,
"Then you may cast a Hero or noncreature spell from among cards exiled with
Scarlet Witch without paying its mana cost." 410 → 164.

Controls that must NOT change:

- Suffocation, "Suffocation deals 4 damage to the controller of the last red
  instant or sorcery spell that dealt damage to you this turn.": 182.
- Teething Wurmlet, "If this is the first time this ability has resolved this
  turn, put a +1/+1 counter on this creature.": 24. The *if* PP is preposed.
- Skyrider Patrol, "When you do, put a +1/+1 counter on another target
  creature you control, and that creature gains flying until end of turn."
  goes 14 → 8 overall, but its 4 Readings that attach *until end of turn* over
  the Clause Coordination must all stay.
- Nissa of Shadowed Boughs, "You may have it become a 3/3 Elemental creature
  with haste and menace until end of turn." goes 22 → 15 overall. Of its 7
  Readings with the Adjunct above *may*, the 6 that have no Clause-level
  attachment must stay.

## Out of scope

- **b-modal.** Temporal Adjuncts above *may* are attested: Kaito, Dancing
  Shadow, "If you do, you may activate loyalty abilities of Kaito twice this
  turn rather than only once."; Urza Assembles the Titans, "You may activate
  the loyalty abilities of planeswalkers you control twice this turn rather
  than only once." `PrepositionPredicate` over `BareAuxiliaryPredicate`
  stays. Kaito's sentence reads 0 at minting, so it is not a count control.
- Preposed Adjuncts (`InitialPreposition` itself) and `Supplementation`
  (*, where X is …*).
- The other attachment classes, which have their own tickets.

## Method

Standard constraints apply. Deltas:

1. **Probe first.** Probe every witness and control before the change. Write
   them as tests. Run the tests once, at the end.
2. **Bounded census.** Selector: every face whose base per-line report lists
   `ClausalPreposition` in `constructions`, or lists
   `ClauseComplementPreposition` together with `PrepositionPredicate` or
   `NominalAdjunctPredicate`. Pass it as an `--identity-manifest`, unioned
   with 300 random covered faces chosen by `cargo xtask english-v3 diff
   --manifest … --random 300 --seed <S>`; record the seed. Run before (claim
   parent) and after on the same manifest. If the selector exceeds half of
   the supported faces, run `--all` instead and say so.
3. **Compare.** Use `cargo xtask english-v3 diff --lines` before → after.
   - Zero lost faces. A lost face is an attested use the restriction would
     wrongly exclude: STOP and name it.
   - Name every decreased face with the Reading shape it lost. Count the
     decreases by class (Clause-level over the trigger; Clause-level over a
     simple Clause; VP over *if*) in the record, and by name for the top 20.
4. **Measure.** Run each census under `/usr/bin/time -v` with `--workers 6`.
   Record wall time and peak RSS.

## Landing record

1. **Census delta.** Pre-refresh claim-parent grammar `rmwotzqr` → pre-refresh implementation `ukytykwr` (both census reports identify the measuring workspace change `ukytykwr`). Same identity manifest: 13,273 faces with `ClausalPreposition`, or `ClauseComplementPreposition` together with `PrepositionPredicate`/`NominalAdjunctPredicate`, plus 300 disjoint random covered faces, seed 20261008; **13,573 faces**. Per-line schema v8 totals: **846,693 → 428,899 Readings**, covered **13,573 → 13,573**. Native `diff --lines`: **0 lost, 0 gained, 11,689 decreases, 29 increases accepted under T5, 1,224 digest-only, 631 unchanged**; zero source-hash changes and census issues. Evidence: `target/english-v3/english-v3-trailing-adjunct-height/{before.json,after-rulings.json,diff-rulings.json}`.

   Retired classes: **A** = Clause-level over `InitialPreposition` (trigger or other preposed Adjunct/condition); **B** = Clause-level over simple Clause; **C** = VP trailer over a finite clause-complement PP, excluding connective orientation (T2–T3). Typed baseline classification retires **423,333** Readings: disjoint A-first counts **A 117,768; B 295,850; C 9,715**. Inclusive counts, which overlap, are A 117,768; B 337,051; C 14,288. Masks A-only 75,306; B-only 292,859; A+B 40,880; C-only 9,715; A+C 1,261; B+C 2,991; A+B+C 321. T1 adds **5,539** Predicate carrier Readings across 214 faces, giving the net removal **417,794**. Native diff of the retained-class projection against the census has **zero decreases/lost** and exactly those 5,539 additional carrier Readings. The native classifier-count validation has 11,721 unchanged faces and zero count differences. The complete named face/class ledger is `decreases-by-class.json` (11,689 faces), with overlap masks retained in `class-ledger.json`; every decreased face has at least one retired class.

   Top 20 decreases (`rmwotzqr` → `ukytykwr`; class letters above):

   | Face | Before → after | Retired classes |
   | --- | ---: | --- |
   | Saint Traft and Rem Karolus | 40,962 → 6,914 | A, B, C |
   | Scarlet Witch, Chaotic Avenger | 40,591 → 8,857 | A, B |
   | Rashmi and Ragavan | 31,872 → 8,256 | B, C |
   | Whiskervale Forerunner | 25,920 → 11,520 | B |
   | Tetsuo, Imperial Champion | 37,940 → 24,948 | B |
   | Last Stand | 14,168 → 2,898 | B |
   | Bring to Light | 25,308 → 15,456 | B |
   | Sea Gate Stormcaller | 9,241 → 1,057 | A, B |
   | Necromentia | 43,560 → 36,100 | B |
   | Ensnared by the Mara | 7,515 → 2,160 | B, C |
   | Sanar, Innovative First-Year | 5,760 → 960 | A, B |
   | Vito, Fanatic of Aclazotz | 5,761 → 1,345 | A, B, C |
   | Ziatora's Envoy | 9,200 → 5,168 | B |
   | Grave Betrayal | 4,650 → 676 | A, B |
   | Mysterious Stranger | 4,681 → 841 | A, B |
   | Semester's End | 3,570 → 252 | A, B, C |
   | Etali, Primal Conqueror | 3,749 → 597 | A, B |
   | Spectral Arcanist | 3,559 → 487 | A, B |
   | Winter, Cynical Opportunist | 3,754 → 782 | A, B |
   | The Jolly Balloon Man | 3,809 → 925 | B |

   All 14 initially lost faces are restored (`rmwotzqr` → `ukytykwr`):

   | Face | Before → after | Ruling |
   | --- | ---: | --- |
   | Cyclonus, Cybertronian Fighter | 14 → 8 | T1 |
   | Great Train Heist | 20 → 14 | T1 |
   | Grim Reaper's Sprint | 22 → 12 | T1 |
   | Marauding Maulhorn | 6 → 2 | T2 |
   | Moraug, Fury of Akoum | 34 → 14 | T1 |
   | Overload | 13 → 2 | T3 |
   | Prohibit | 13 → 2 | T3 |
   | Raiyuu, Storm's Edge | 81 → 41 | T1 |
   | Reckless Cohort | 6 → 2 | T2 |
   | Shadow of the Second Sun | 7 → 4 | T1 |
   | Spelltwine | 1005 → 278 | T2 |
   | Sphinx of the Second Sun | 7 → 4 | T1 |
   | Tifa, Martial Artist | 241 → 121 | T1 |
   | Y'shtola Rhul | 240 → 60 | T1 |

2. **Witnesses.** All 12 ticket witnesses were probed before implementing and again on `ukytykwr`, with no admission/realization issues; after counts, in ticket order: Goliath attack **323**, Goliath cast **140**, Belladonna Took **24**, Vizier of Deferment **15**, Veil of Summer **2**, Wyluli Wolf **1**, Suffocation cast **12**, Scarlet Witch **164**, Suffocation damage control **182**, Teething Wurmlet control **24**, Skyrider Patrol **8**, Nissa of Shadowed Boughs **15**. The two unchanged controls retain all 182/24 Readings. Belladonna is 80 at this claim parent (the older ticket estimate 84 predates trunk changes); the current cast witness independently has 12 retained Readings, rather than the ticket estimate 21. Evidence: `before-probe-*.json` and `rulings-probe-*.json`.

3. **Spot checks.** No faces are newly covered by this restriction. Ten retained card checks on `ukytykwr`: Goliath (both triggers retain VP trailers); Belladonna (temporal Adjunct inside finite *if*); Vizier (duration inside finite *if*); Veil (this-turn trailer inside finite *if*); Wyluli (duration inside finite *if*); Suffocation (cast restriction corrected and damage control unchanged); Scarlet Witch (inside-condition temporal trailer); Teething Wurmlet (preposed conditional control unchanged); Skyrider Patrol (Clause Coordination permits Clause-level duration; independent values preserve four coordinated-scope Readings); Nissa (duration above *may* preserved; independent values preserve six such Readings).

4. **Deviations and STOPs.** Orchestrator rulings **T1–T5, 2026-10-10**, are project rulings. The initial 14-face STOP is resolved by T1–T3. T1 rehomes `ExistentialClause` into `ExistentialPredicate` and adds its result to the existing PP/Nominal Adjunct Predicate rows using the same declared licences; `FiniteClause` hosts the Dummy Pronoun and Predicate. This adds no net Construction and retires the old existential Clause route, with no Clause-level exception or noun-postmodifier workaround. Existing existential and Nominative Pronoun value fixtures are re-spelled against the replacement. T2 distinguishes Finite/Verbless on the declared Clause-complement side; only Finite blocks an ordinary trailer. T3 exempts declared connective orientation: *instead* declares `ConnectiveAdjunctUse`, forwarded through focusing and coordination. CGEL Ch. 6 §7.1, p. 576, [6v] classifies connective **AdvP** Adjuncts as Clause-oriented; its application to the intransitive PP here is **ruling T3**. T4 authorizes the 32 exact count re-spells below and requires both new controls. T5 resolves the 29-increase STOP: Predicate attachment over an existential is a grammatical analysis distinct from NP-internal postmodification; no additional carrier eligibility constraint is added.

   The completion commit `ukytykwr` was refreshed onto trunk changes `umzuyqxm` (division), `wlstpkpu` (property equality), and `xmyznkll` (last-turn adjuncts). Refresh exited 69 with 37 declaration hunks and one test-registry hunk; harmony repaired the owned change, retaining both feature sets and all tests. Retry exited 0 (no-op), with no remaining conflicts. Post-refresh checks use `konuvksr` and the native 2,000-face sample. Against the pre-refresh landing, the sample has **0 lost/gained/decreases/increases**, **296 digest-only**, **1,704 unchanged**, and **72,815 → 72,815 Readings**; there are no refresh-introduced removed Readings to attribute. The digest changes accompany the trunk constructor re-spells in `umzuyqxm` and `wlstpkpu`. Against the original claim-parent sample: **163,869 → 72,815 Readings**, **1,731 decreases, 5 T5 increases, 191 digest-only, 73 unchanged**, covered **2,000 → 2,000**, zero source-hash changes and census issues. Evidence: `refresh-diff.json`, `refresh-trunk-diff.json`, `refresh-after.json`. Full suite/clippy outcomes follow in item 5.

   Glossary gaps filled: `ClauseCoordinationHost`, `ClauseComplementBoundary`, `Existential Predicate`; the existing Connective Adjunct entry documents the declared feature and distinguishes the CGEL passage from T3. Seven new integration tests cover heights, both independent controls, stacking, the 14 restored faces and the existential carrier. The new stacking test uses an attested Skyrider base after the initial synthetic Unnerving Assault candidate proved unread at baseline; no historical test is removed. The two existing *as long as* full-value fixtures are re-spelled to their VP-level replacements: Aeronaut Tinkerer **2 → 1**, retiring B; Master Thief **3 → 1**, retiring A and B.

   All 29 increased faces below gain only PP attachment at Predicate level over `ExistentialPredicate` (no Nominal Adjunct gain occurs in this set). The native line diff identifies the changed lines. Typed per-line enumeration excludes those carriers and matches the baseline retained class-zero count on **every** face: native diff **29 unchanged**, **0 lost/gained/decreases/increases**, **6,625 → 6,625** (`t5-shape-validation.json`); `t5-line-attestation.json` ties each positive native line delta to its typed Predicate carrier. Together with the grammar’s only additive carrier rows, this accounts for every gain without Debug-tree scanning. Five native probes: Keen-Eyed Curator, Profane Procession, Calculated Dismissal, Charitable Levy and Deathbringer Regent (`t5-probe-0..4.json`); all complete without issues.

   | Face | Before → after | Line (1-based); confirmed existential Predicate PP |
   | --- | ---: | --- |
   | Bygone Marvels | 61 → 65 | 1: in your graveyard |
   | Calculated Dismissal | 8 → 9 | 2: in your graveyard |
   | Charitable Levy | 206 → 272 | 2: on it |
   | Cleansing Meditation | 33 → 49 | 2: in your graveyard |
   | Convert to Slime | 161 → 221 | 2: in your graveyard |
   | Crashing Centaur | 6 → 7 | 2: in your graveyard |
   | Dark Petition | 13 → 14 | 2: in your graveyard |
   | Deathbringer Regent | 7 → 9 | 2: on the battlefield |
   | Decaying Soil | 16 → 20 | 2: in your graveyard |
   | Demoralize | 14 → 19 | 2: in your graveyard |
   | Fear of Missing Out | 131 → 179 | 2: in your graveyard |
   | Gilded Cerodon | 9 → 12 | 1: in your graveyard |
   | Jace, Vryn's Prodigy | 104 → 126 | 1: in your graveyard |
   | Keen-Eyed Curator | 19 → 24 | 1: with this creature |
   | Kirtar's Wrath | 149 → 169 | 2: in your graveyard |
   | Magmatic Channeler | 15 → 16 | 1: in your graveyard |
   | Mournwillow | 49 → 67 | 2: in your graveyard |
   | Null Summoner | 32 → 42 | 2: in your graveyard |
   | Olórin's Searing Light | 157 → 198 | 2: in your graveyard |
   | Osseous Sticktwister | 5249 → 7217 | 2: in your graveyard |
   | Profane Procession | 2 → 3 | 1: with Profane Procession |
   | Psychic Rebuttal | 95 → 101 | 2: in your graveyard |
   | Putrid Imp | 6 → 7 | 2: in your graveyard |
   | Repentant Vampire | 7 → 9 | 3: in your graveyard |
   | Snooping Newsie | 5 → 7 | 2: in your graveyard |
   | The Millennium Calendar | 10 → 11 | 3: on The Millennium Calendar |
   | The Swarmweaver | 152 → 204 | 2: in your graveyard |
   | Toxic Stench | 6 → 7 | 2: in your graveyard |
   | Wickerfolk Thresher | 32 → 44 | 1: in your graveyard |

5. **Tests.** Pre-refresh on `ukytykwr`: `cargo test -p deckmaste_english_v3` — PASS, 712 tests (0 failed/ignored), 23.11 s suite runtime; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` — PASS (53.76 s, no warnings); `cargo test -p deckmaste_lexical_source` — PASS, 28 unit + 36 integration tests (0 failed/ignored), 3.69 s / 1.31 s suite runtimes; `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` — PASS (22.98 s, no warnings). No other crate code/data changed. No Lean/xtask suite. Assurance: restored **2** new control tests; re-spelled **32** historical count tests (same card/unit, all positive), plus the existential replacement/value fixtures and the two existing *as long as* value tests; ignored **0**, added **7**, removed **0**. The initial restriction-only suite’s 34 failures were the 32 count tests plus those two controls; all subjects remain.

   Post-refresh on `konuvksr`: `cargo test -p deckmaste_english_v3` — PASS, **737 tests**, 0 failed/ignored/filtered, 12.33 s suite runtime; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` — PASS, 27.13 s, no warnings; `cargo test -p deckmaste_lexical_source` — PASS, **28 unit + 36 integration tests**, 0 failed/ignored/filtered, 3.24 s / 1.22 s suite runtimes; `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` — PASS, 0.99 s, no warnings. Both new controls and all seven added tests pass in the full suite. Logs: `test-refreshed-fixed.log`, `clippy-refreshed.log`, `test-lexical-source-refreshed.log`, `clippy-lexical-source-refreshed.log`. Formatting passes for both owned crates.

   T4 count re-spells (`rmwotzqr` → `ukytykwr`); letters name the retired classes from item 1. Each loop entry is retained, including unchanged controls:

   | Existing test | Count re-spell(s) | Class attribution |
   | --- | --- | --- |
   | `arithmetic_nominal_operand::haunting_apparition_keeps_the_modified_card_count` | 6 → 5 | B |
   | `as_clause_np_licensing::the_negative_probe_has_no_finite_as_clause` | 5 → 2 | B |
   | `at_least_cardinal::svyelun_keeps_the_bounded_cardinal_in_determiner_function` | 2 → 1 | B |
   | `at_least_cardinal::synchronized_eviction_keeps_the_relative_clause_on_the_nominal` | 3 → 2 | B |
   | `become_adjectival_passive::talruum_champion_keeps_the_selected_by_complement_inside_the_adjective` | 9 → 3 | A, B |
   | `as_clause_np_licensing::comparison_and_duration_survive_without_the_finite_as_alternatives` | 30 → 15; 96 → 30; 30 → 15; 96 → 30 | B; A, B; B; A, B |
   | `card_noun_premodifier::crop_sigil` | 25 → 11 | B: 14 and C: 3 retired; T1 adds 3 existential carriers (25 − 17 + 3 = 11) |
   | `become_adjectival_passive::vedalken_ghoul_trained_cheetah_and_somberwald_alpha_preserve_the_adjective` | 3 → 1; 3 → 1 | A, B; A, B |
   | `card_noun_premodifier::deluge_of_doom` | 24 → 12 | B |
   | `card_noun_premodifier::open_the_gates_preserves_coordination_without_card_as_a_coordinated_premodifier` | 23 → 20 | B |
   | `clause_final_predicative_as::selected_and_comparative_controls_gain_no_as_adjunct` | 28 → 22; 6 → 5 | B; B |
   | `coordinated_subject_agreement::alternative_coordination_preserves_singular_agreement` | 2 → 1 | B |
   | `coordinated_subject_agreement::shared_determiner_nominal_subject_requires_plural_agreement` | 4 → 3 | B |
   | `enter_as_copy_complement::existing_enter_frames_and_adjunct_exclusion_survive` | 2 → 1 (Activate as a sorcery) | B |
   | `except_matrix_complements::shifting_sliver_control_retains_its_complete_reading` | 9 → 7 | B |
   | `enter_as_copy_complement::master_biomancer_keeps_licensed_unlike_complement_coordination` | 96 → 72 | B |
   | `enter_as_copy_complement::geths_summons_keeps_finite_as_as_an_adjunct` | 48 → 24 | A, B |
   | `focusing_only::bare_as_keeps_its_preposed_licence_and_gains_final_position` | 2 → 1 (Activate as a sorcery) | B |
   | `from_anywhere::generous_soul` | 12 → 4 | A, B |
   | `from_anywhere::brinebound_gift` | 12 → 4 | A, B |
   | `from_anywhere::yawgmoths_agenda` | 12 → 4 | A, B |
   | `from_anywhere::rest_in_peace` | 12 → 4 | A, B |
   | `gerund_preposition_complements::jazal_goldmane_keeps_its_nominal_complement` | 4 → 2 | B |
   | `get_object_frames::vexing_radgull` | 9 → 3 | A, B |
   | `library_positions::stand_together_retains_coordinated_object_and_locative_complements` | 6 → 5 | B |
   | `name_lexeme_scope::shortened_names_keep_one_card_identity_and_roundtrip_in_its_scope` | 4 → 3 | B |
   | `overgeneration_prune::a_filled_passive_rejects_a_second_internalised_complement_adjunct` | 3 → 2; 9 → 7 | B; B |
   | `perfect_relative_adverbial_gap::frequency_and_lexical_have_controls_keep_their_readings` | 32 → 24; 280 → 196 | A, B; A, B |
   | `postpositive_comparative_property::existing_routes_keep_the_claim_parent_counts` | 3 → 2 | B |
   | `perfect_relative_adverbial_gap::resolved_witnesses_use_temporal_gaps_and_perfect_auxiliaries` | 80 → 24; 80 → 24; 432 → 72 | A, B, C; A, B, C; A, B |
   | `produce_verb::pale_moon` | 12 → 3 | A, B |
   | `then_sequencing::erode_retains_clause_series_and_shared_subject_readings_for_both_markers` | 16 → 14; 16 → 14 | B; B |

   Refresh introduced `property_equal_to::ironclaw_curse_keeps_its_coordinated_governor_readings` from trunk `wlstpkpu`. Its Document count is re-spelled **8 → 6**: exactly two class B Readings attached the PP “with power equal to or greater than the enchanted creature's toughness” over the simple Declarative Clause. The typed construction check rejects both retired values and admits/realizes all six retained values; the coordinated-governor and Property-value constituent checks remain. Evidence: `ironclaw-refresh.json` and `ironclaw-refresh-probe.json`. This is one additional inherited count re-spell after refresh, beyond the original 32; no test is removed or ignored. The first refreshed full English suite found this count failure (736 passed, 1 failed); the corrected full run and final clippy outcomes are recorded below.

   The postpositive-property control is renamed from `existing_routes_keep_the_claim_parent_counts` to `existing_routes_keep_their_predicate_readings`; the same two units and exact count assertions remain.

   Erode’s exact scope counts are re-spelled **(10 Clause-series, 6 shared-subject) → (8, 6)** for both markers: the two B Readings retire, with the same scope-value checks. The existential bare-and-extended unit is **2 → 3**, retaining the third Predicate-level PP value under T5. Citation checks: pre-refresh 0 noncompliant strings, 0 stale among 17,252 citations; post-refresh 0 noncompliant strings, 0 stale among 17,385 citations; `jj --no-pager diff --git | cargo xtask cite audit --diff` selects 0 new citation sites. No CR registrations to bless.

6. **Timing.** One full claim-parent census: **8:02.08 wall, 6 workers** (32,828 supported faces; 914,828 Readings). The landing census is bounded, timed with `/usr/bin/time -v` and a prebuilt binary: **5:47.87 wall, 6 workers**, 13,573 faces, peak RSS 10,867,832 KiB. No second full census. The implementation was closed in `ukytykwr` and the mandatory refresh completed successfully; post-refresh verification and the inherited count re-spell are recorded in `konuvksr`. The refreshed 2,000-face manifest is drawn natively from the same 13,573 faces, seed 20261008; its census is **0:25.68 wall, 6 workers**, peak RSS **3,851,444 KiB**, using the timed prebuilt binary. Integration and workspace retirement remain for explicit orchestrator authorization.
