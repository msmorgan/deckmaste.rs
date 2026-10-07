---
needs: []
---
# Give clitic be and have the frames of their full forms

## Why

Clitic auxiliaries on pronoun hosts (*it's*, *that's*, *you've*, *they're*,
*you're*) fail where the full form reads. On change `wlvwtnppyovn` (32,828
supported faces, 13,716 covered, 19,112 unread) they occur in a failing unit
of 1,318 unread faces and are the only recognised cause on 352. These are
surface-bucket counts, not gain forecasts.

Minimal pairs on that change (`cargo xtask english-v3 probe --readings 0`,
admitted roots):

| Contracted | Full |
|---|---|
| "If it's a land card, you may put it onto the battlefield tapped." 0 | "If it is …" 1 |
| "If it's tapped, put a stun counter on it." 0 | "If it is tapped, …" 1 |
| "Create a token that's a copy of target creature." 0 | "… that is a copy …" 1 |
| "Exile target creature that's attacking you." 0 | "… that is attacking you." 1 |
| "This creature can't attack unless you've cast a creature spell this turn." 0 | "… unless you have cast …" 1 |

Negative inflections already read: "This creature can't block." 1 and "This
creature isn't legendary." 1. Check *didn't*, *doesn't*, *wasn't* the same way
before scoping them in.

The likely cause is in the `frame_additions` block of
`crates/deckmaste_lexical_source/lexicon/core.ron`. It gives
`core-verb:BeContracted` (and `BeNegative`) only a `LocativeComplement`
Predicate frame and a `ParticipialPredicate` Auxiliary frame. The full *be*
also takes NP and AdjP predicative Complements. The contracted-*be* Predicate
frame was added by the generic-frame-consumption landing (its STOP 2, Urza's
Ruinous Blast) to stop an auxiliary-stranding misparse. Keep that witness's
two exact Readings. `core-verb:HaveContracted` has only a
`PastParticiplePredicate` Auxiliary frame, yet "you've cast" still fails, so
the *have* cause is something else. Find it.

## Goal

Each clitic form has exactly the frames of the full-form paradigm cell it
realises, with clitic Readings forming a stranding-free subset of the
uncontracted twin's Readings. Where a frame is deliberately withheld from the
clitic, the withholding must come from a declared feature with a CGEL basis,
not from an ad hoc omission.

## Analysis

These are clitic versions of auxiliary verbs (CGEL, Ch. 18, §6.2,
pp. 1614–1616). The clitic forms of *are* and *have* (*they're*, *you're*,
*you've*) attach only to a preceding subject pronoun ([6]–[7], p. 1615). The
clitic forms of *is* and *has* are less restricted: their hosts include NP
Subjects and relative *that* (*the one that's up in the bedroom*, [8iii]).
*'s* is therefore ambiguous between *is* and *has*, as in *It's finished*
(p. 1615). Clitic auxiliaries are a reduced form of the same lexeme, a
property that only auxiliaries have (CGEL, Ch. 3, §2.1.5, p. 102). The clitic
inherits its verb's complementation; its host restriction is a
morphophonological fact, recorded on the lexeme, not in the frame list.

## Witnesses

- Risen Reef: "If it's a land card, you may put it onto the battlefield tapped."
- Shackle Slinger: "If it's tapped, put a stun counter on it."
- Mirrorpool: "{4}{C}, {T}, Sacrifice this land: Create a token that's a copy
  of target creature you control."
- Bounty Agent: "{T}, Sacrifice this creature: Destroy target legendary
  permanent that's an artifact, creature, or enchantment."
- Goblin Cohort: "This creature can't attack unless you've cast a creature
  spell this turn."
- Panglacial Wurm: "While you're searching your library, you may cast this card
  from your library."

Each witness's only failing unit is the one quoted, and each witness's
uncontracted twin reads (probed on `wlvwtnppyovn`).

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. First determine why "Create a token that's a copy of target creature."
   fails and why "you've cast" fails, and record both causes.
2. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/clitic-contractions-before.json` on the claim
   parent, stamped with its change id.
3. Write each witness and its uncontracted twin as tests first. The two must
   retain corresponding structures modulo the auxiliary leaf; the clitic
   admits the stranding-free subset of the full form's Readings.
4. Keep the Urza's Ruinous Blast exact two-Reading test passing unchanged.
5. Iterate on `--face-id` selectors; verify on `--all` at the end.
6. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
7. Spot-check every newly covered face's selected analysis. The landing record
   lists at least 10 by name with their Reading. A clitic *'s* read as a
   Genitive where the uncontracted twin has *is*, or any auxiliary-stranding
   gap, is a defect, not a gain.
8. No guard may name a lexeme, construction, verb, noun, preposition or card;
   read declared features only. Tests are re-spelled, never deleted. No new
   xtask subcommands, flags, fixtures or tooling.
9. May edit `grammar:` frame fields in `plugins_v2` keyword actions and lexicon
   frames in `crates/deckmaste_lexical_source/lexicon/*.ron`; plugin bodies
   untouched. Lexicon additions must be attested in the supported corpus
   (pruning rule).
10. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- *there's* and existential *there* (a separate unread bucket).
- Any cause the uncontracted twin also fails on. Damping Matrix's "…can't be
  activated unless they're mana abilities." fails uncontracted too ("unless
  they are mana abilities", 0 admitted roots).

## Landing record

In addition to the standard record: the two causes from Method step 1; a table
of clitic forms (`'s` as *is*, `'s` as *has*, `'re`, `'ve`, `'d`) with their
frames before and after; the Urza's Ruinous Blast result; timings as integer ns
and ns/B with host load and worker count.

### Causes and implementation

The claim is `pmpxvoxkswnwqkmpnkomymrzlrmnrmxv`. On its tree, contracted
*be* already has the full *be* frame signatures, including NP and AdjP
Predicative Complements. The ticket's earlier missing-frame hypothesis no
longer describes this tree. No frame addition is needed.

Both failures are separator failures. `SubjectRelativeClause` requires a
space between relative *that* and its Finite Predicate, preventing *that's a
copy*. `FiniteClause` requires a space between its Subject and Finite
Predicate, preventing *you've cast*. Lexical analysis already recognizes the
suffixes and their appropriate frames. Five existing clause constructors now
also declare a joined form: `FiniteClause`, `SubjectRelativeClause`,
`PronounSubjectRelative`, `ObjectRelativeClause`, and
`ZeroObjectRelativeClause`. The original spaced forms retain their indices.
Form-specific feature requirements now reject a spaced clitic or joined free
verb. The compiler combines these requirements with the Construction's common
equations in both parsing and independent admission. Lexical binding alone
relaxes word edges and deliberately leaves host restrictions to the grammar.

`Clitic`, `CliticHost`, and `SubjectStructure` declare the host distinction
from CGEL, Ch. 18 §6.2, pp. 1615–1616. Present clitics outside the third-person
singular require a pronoun that is itself the Subject; phrase coordination
does not inherit that licence. Predicate projections retain the host feature;
a subsequent coordinated Predicate cannot supply a new clitic host. Clitics
require overt auxiliary Complements and cannot supply an uncomplemented
coordinate head: auxiliary stranding requires the stressed strong form
(CGEL, p. 1614). Every new licensing guard reads declared features.

`PredicateFrameUse = No` makes the perfect clitic's withholding of lexical
*have* frames explicit. American English stative and dynamic *have* are
lexical uses, whereas perfect *have* is auxiliary (CGEL, Ch. 3 §2.5.6,
pp. 111–113). The generic Predicate and Object-gap consumers read that feature.
Frames, surface variants, lexical owners, and plugin bodies are unchanged.

| Clitic cell | Frames before | Frames after |
|---|---|---|
| `'s` = *is* | Predicate → PredicativeComplement; Predicate → LocativeComplement; Auxiliary → ParticipialPredicate | Same three full-*be* signatures |
| `'re` = *are* | Same three *be* signatures | Same, with a pronoun Subject host |
| `'s` = *has* | Auxiliary → PastParticiplePredicate | Same full-*have* auxiliary signature; lexical Predicate use explicitly unlicensed |
| `'ve` = *have* | Auxiliary → PastParticiplePredicate | Same, with a pronoun Subject host and lexical Predicate use explicitly unlicensed |
| `'d` = *had* | Auxiliary → PastParticiplePredicate | Same, with lexical Predicate use explicitly unlicensed |

The existing preterite clitic declarations are retained; this landing adds no
unattested forms. Negative inflections remain free Words. Baseline probes of
*doesn't*, *didn't*, and *wasn't* admit 6, 2, and 1 Readings respectively, so
they require no changes.

### PROVE — preservation and laws

The corrected baseline is `zxlxvukrspktolnnsrmoqopqtkmuskwo` (covered
**15,167**), after **Remove overlapping Pay symbol frame**, not `lpzxsvzm`.
The original `tnnokwxtrvkq` reports preceded that change and do not describe
the refreshed landing. The landed tree `wzutyrmomtpqtoxqquopnqqukqtnpqzy`
(covered **15,404**) has **230,566** Readings. A fresh full-corpus run on
2026-10-06 confirms that census on `ksnusstzlqkkpypksnnwotqqxuqwnyqz`
(covered **15,404**), the docs-only follow-up's working tree
on current trunk tip `vxszvmowynxunuoplppwyyymptxuuzlp`. Its grammar is
byte-identical to `wzutyrmo`; the intervening trunk change only claims the
sibling granted-ability ticket. The baseline report and fresh verification
share input SHA-256
`49dc966bda6ef588fc68e8d6972de25df1b660ed2ea654904584698263007ebb`.
The baseline lexical inventory SHA-256 is
`13aa0820f27162671f2e849fd35bad94970b5b3a628bc979ebd60c7b7299470c`;
the fresh inventory SHA-256 is
`ee58384138b8d8e1008cec37f3ad4ac265fe249a59f93d611e7bd9578be4cbe2`.
The clitic features change that serialization even though owners and surfaces
are unchanged.

Both censuses use `cargo xtask english-v3 --all --workers 12
--samples-per-face 0 --output <ignored-report.json>`, without a Reading limit.
The baseline report is the Pay de-duplication follow-up's after census; the
fresh report and identity comparison are local ignored evidence under
`target/english-v3/`. They are not portable links or required ticket inputs.

| Census | Before: `zxlxvukr`, covered 15,167 | After: `ksnusstzlqkk`, covered 15,404 |
|---|---:|---:|
| Supported faces | 32,828 | 32,828 |
| Covered faces | 15,167 | 15,404 |
| Unread faces | 17,661 | 17,424 |
| Exactly one Reading | 6,776 | 6,785 |
| Multiple Readings | 8,391 | 8,619 |
| Exact Readings checked | 217,223 | 230,566 |
| Incomplete / limited / failed faces | 0 / 0 / 0 | 0 / 0 / 0 |
| Validation issues / internal failures / duplicate Readings | 0 / 0 / 0 | 0 / 0 / 0 |

Lost face identities versus `zxlxvukr`: **[]**. Reading-count decreases versus
that baseline: **[]**. All **237** gained faces and **13,343** additional
Readings belong to the clitic landing. Compared instead with `lpzxsvzm`,
**75** faces have fewer Readings: all are attributable to `zxlxvukr`'s Pay
frame de-duplication, none to this landing. These retire duplicate derivations,
lose no covered identity, and create no clitic re-coverage obligation. The
census checks declaration admission, contextual lexical ownership, byte-exact
realization, and complete node/Word traversal identity for every counted
Reading. Independent constructed-value tests additionally retain both exact
copular and passive analyses of *it's tapped*, with exact realization and
node/Word identity, and reject incorrect boundary forms.

Forbidden word-naming licensing guards introduced: **0**. Grammar environment
and lexical-source loading succeed; the census reports zero internal failures.
English v3 preserves all Readings under the current lexical-analysis decision;
it has no destructive specificity selection, coverage-lock count, or legacy
permitted-checker total emitted by this command. Those legacy figures are not
substituted for the complete v3 census.

### DISCLOSE — selected analyses

The original inspection covered all **237** newly covered identities; each
selected clitic analysis has an overt auxiliary Complement. The named
inventory and selected-tree samples are local ignored evidence. The thirteen
analyses below stand on their own. Mapping a selected clitic Reading to its
full auxiliary with the same frame signature and constituent structure checks
the corresponding full-form Reading's admission; this does not establish
equality of the complete Reading sets.

Clitic Readings are a **stranding-free subset** of the full twin's Readings.
Brimaz, King of Oreskos's attack-trigger unit has **21** clitic versus **133**
full-form Readings; Onakke Oathkeeper's attack-restriction unit has **33**
versus **111**. The minimal pair "Create a token that's attacking." has **1**
versus **3** for "Create a token that is attacking." The extra full-form
Readings use `PassiveEllipsis` or `ProgressiveEllipsis` stranding under a
participial postmodifier. Excluding these from the clitic is principled:
auxiliary stranding requires the strong form (CGEL, Ch. 18 §6.2, p. 1614). The
full-form stranding Readings are themselves dubious and pre-existing; their
measured review is owed to `english-v3-systemic-residuals`, not counted as a
clitic coverage gain.

Representative newly covered selected Readings, confirmed on
`ksnusstzlqkk` (covered **15,404**; the grammar of `wzutyrmo`):

| Face | Selected analysis of the clitic unit |
|---|---|
| Risen Reef | *it's a land card*: copular SelectedPredicate with nominal Predicative Complement |
| Shackle Slinger | *it's tapped*: copular SelectedPredicate with adjectival Complement; the passive Reading is also retained |
| Mirrorpool | *that's a copy of target creature*: SubjectRelativeClause with copular nominal Complement |
| Bounty Agent | *that's an artifact, creature, or enchantment*: SubjectRelativeClause with coordinated nominal Complement |
| Goblin Cohort | *you've cast a creature spell*: PerfectAuxiliaryPredicate with complete past-participial Complement |
| Panglacial Wurm | *you're searching your library*: progressive ParticipialAuxiliaryPredicate |
| Dream Thief | *you've cast another blue spell*: PerfectAuxiliaryPredicate |
| Afterlife from the Loam | *they're Zombies*: copular SelectedPredicate with nominal Complement |
| Coiling Oracle | *it's a land card*: copular SelectedPredicate with nominal Complement |
| Call of the Wild | *it's a creature card*: copular SelectedPredicate with nominal Complement |
| Brimaz, King of Oreskos | *that's attacking* and *that's blocking that creature*: relative progressive ParticipialAuxiliaryPredicates |
| Captain's Claws | *that's tapped and attacking*: relative ParticipialAuxiliaryPredicate with mixed participial Complement |
| Laboratory Drudge | *you've cast a spell … or activated an ability …*: PerfectAuxiliaryPredicate with coordinated past-participial Complement |

The six specified witness tests retain their exact corresponding structures
modulo the auxiliary owner and surface form; their equal counts do not imply
complete-set equality for other twins. Frame signatures, rather than different
owner-local frame indices, are compared. The additional Myth Unbound fragment
*it's been cast* tests `'s` as perfect *has*. Urza's Ruinous Blast retains its
**two exact Readings**, with its existing test unchanged.

The earlier **352** sole-recognised-cause faces were a surface-bucket estimate,
not a gain forecast; the landing gains **237**. After landing, the review's
contracted-auxiliary surface bucket leaves **1,062** faces unread. That bucket
uses substring matching and also includes *you're*; exact word-boundary
occurrences of *it's*, *that's*, *you've*, or *they're* give **1,016** unread
faces in the fresh census. In a sample, the full-form twin also fails in most:
Llanowar Loamspeaker's "It's still a land." is **0/0** (clitic/full), blocked
by *still*; Leitmotif Composer, Korvold, Gleeful Glutton, Skyshroud Condor,
Discordant Spirit, and Yarus, Roar of the Old Gods are also **0/0**. Omen
Machine's clitic sentence reads, but its face is blocked elsewhere. A clitic
occurrence does not establish a clitic-only cause.

The real clitic-specific gap is Murmuration's "Draw a card for each spell
you've cast this turn.": recognition gives **0** clitic versus **1** full-form
admitted root. Full enumeration gives **0/9** Readings, so the recognition
count must not be called one Reading; none of the nine contains a perfect
auxiliary. The inspected full-form Reading is a pre-existing wrong analysis:
lexical *have* with a finite Object Gap plus a passive "cast this turn"
depictive Adjunct, with no perfect auxiliary. The clitic correctly withholds
lexical *have*. The review estimates about **56** unread faces in the "… you've
<V> this turn" group; the fresh census has **49** matching that strict
single-verb, immediate-*this turn* pattern. The approximate bucket is not a
completed grammatical-cause census. Perfect *have* with a relative Object Gap
is owed to `english-v3-systemic-residuals`; its landing must retire the wrong
full-form analysis when it supplies the perfect.

The census is unique/multiple, **not** unique/specificity-resolved; no
construction pair is selected away. All ambiguity is retained.

### Deviations and additions

- Orchestrator resolution (2026-10-06): the construction_v3_core compiler extension (form-level `require` clauses, canonical-form identity including requirements) is accepted as generic grammar infrastructure; disclosed and tested.
- No frame additions: the claim parent already supplies the required *be*
  frames. Joined forms on all five existing subject-boundary constructors
  implement the same host rule without inventing a new construction family.
- Host, overt-Complement, coordinate-head, and Predicate-use feature guards
  make the ticket's CGEL restrictions explicit and prevent auxiliary stranding.
- The extra independent boundary check exposed admission of *it 's tapped*.
  The compiler gained form-specific direct/table feature requirements so the
  original spaced form and added joined form can keep one Reading constructor
  while licensing different declared host features. A schema-instance test and
  two compiler tests cover both directions, invalid requirements, and canonical
  form normalization. Existing tests were kept unchanged.
- Beyond the six specified twin tests, four tests cover perfect `'s`, an
  independently constructed copular/passive pair, invalid hosts/stranding,
  and full/clitic frame-signature parity.
  Their names are `myth_unbound_clitic_has_keeps_the_perfect_passive_complement`,
  `independently_constructed_clitic_copula_and_passive_roundtrip`,
  `clitic_auxiliaries_require_overt_complements_and_licensed_hosts`, and
  `clitic_be_retains_all_full_be_frames_and_have_retains_the_perfect_use`.
- Compiler assurance adds
  `surface_requirements_reject_exports_and_invalid_features`,
  `surface_requirements_distinguish_forms_without_defeating_normalization`,
  and `surface_requirements_select_the_same_independent_values_in_both_directions`.
  The last extends the existing unit-test grammar with `SurfaceChecked` and
  `SurfaceAllowed` to check schema inheritance and both direct/table guards.
- The Oracle English glossary's Clitic entry was clarified; Clitic Host,
  Subject Structure, and Predicate Frame Use were gaps needed by the landing
  and are now defined with CGEL references. No Game Model term was introduced.
- No lexeme, surface variant, plugin body, tracked corpus fixture, xtask command,
  flag, or tooling was added or deleted. All census and inspection files live
  under the workspace's ignored `target/english-v3/`.

STOPs: **none**. The obsolete ticket hypothesis is an evidence correction,
not a contradiction with a recorded ruling. The generic-frame-consumption
Urza ruling and witness are preserved. An overbroad formatter pass was
restored completely on unrelated paths before final verification.

Assurance counts: **restored 0; re-spelled 0; newly ignored 0; added 13;
removed 0**. Existing ignored tests and their blockers remain unchanged.
The one inherited ignore is
`macros::templates::tests::macro_schema_census_count_matches_21`: it
cross-checks the live corpus against its census and is run on demand.

### REPORT — inventories and performance advisory

Stamped baseline `zxlxvukr`, covered **15,167**, and fresh verification
`ksnusstzlqkk`, covered **15,404**: **230** named Constructions before and
after (186 ordinary declarations and 44 schemas), 135 Categories before and
after; the final grammar compiles **625** productions. There are five added
surface forms and no added/deleted Constructions. The unchanged inventory has
**1,085** case-folded multi-owner homograph spellings. A fresh inventory
confirms these totals; the named inventory is local ignored evidence, not a
portable linked artifact. Exact form-literal/vocabulary overlap names: **[]**.
No lexical owner or spelling changed. The `'s` homograph retains *be*, *have*,
and Genitive owners; it is not collapsed into one owner.

| Measurement | Before: `zxlxvukr`, covered 15,167 | After: `ksnusstzlqkk`, covered 15,404 |
|---|---:|---:|
| Corpus wall time | 68,865,652,661 ns | 62,135,993,494 ns |
| Setup wall time | 7,785,738,794 ns | 8,313,602,154 ns |
| Reported setup + corpus wall time | 76,651,391,455 ns | 70,449,595,648 ns |
| Checked-text thread CPU telemetry | 294,268 ns/B | 248,127 ns/B |
| Aggregate thread CPU | 640,431,211,047 ns | 553,203,986,264 ns |
| Chart families | 99,960,922 | 107,010,739 |
| Workers | 12 | 12 |
| Host load (1 / 5 / 15 minutes) | 6.876 / 7.540 / 7.856 | 13.945 / 15.207 / 10.823 |

Both corpus times exceed the **16,260,000,000 ns** quiet-host advisory. Host
load and worker counts are disclosed alongside each measurement; these runs do
not establish quiet-host compliance. The setup-plus-corpus figures exclude
Cargo compilation and report writing; even the corpus phase alone exceeds the
command-time advisory. This is performance evidence, not a passing performance
claim.

### Validation

Gate invocation: `cargo xtask gate --changed --from pmpxvoxk --clippy --run`.
Derived test command: `cargo test -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p
deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask`.
Derived lint command: `cargo clippy -p deckmaste_construction_v3_core -p deckmaste_lexical_source -p
deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets --
-D warnings`. The complete test command passes: **744 passed, 0 failed,
1 inherited ignore**, across 83 suite results including doctests. Initial
clippy found `clone_on_copy` in the new test helper; copying the lexical
identity directly fixes it without changing grammar or test behavior. The
clitic suite and the full derived clippy command are rerun after that fix;
the clitic rerun passes **10 tests**, and the complete derived clippy command
passes with **exit 0**. The correction only removes a redundant clone of a
Copy identity; the previously passing complete test results remain applicable.
The complete test gate, clitic rerun, and final clippy logs are local ignored
evidence; these results describe the implementation validation.

Focused Rust formatting check passes. Citation noncompliance is **0**;
**16,059** registered citation sites have **0** stale rules. The piped
`jj diff --git | cargo xtask cite audit --diff` audits zero new rule sites:
this landing changes CGEL references, not Comprehensive Rules citations.

### Docs-only follow-up (2026-10-06)

`english-v3-clitic-record-fix` changes only this ticket and the systemic
residuals ticket. It corrects census provenance, twin-subset wording, the
remainder explanation and infrastructure acceptance, and routes the three
residuals. No code, lexicon, test or citation is changed. Follow-up assurance
counts: restored **0**, re-spelled **0**, ignored **0**, added **0**, removed
**0**. The fresh complete census and identity comparison pass. Citation check
from the workspace root reports **0** noncompliant and **0** stale sites;
`kata kanban check` reports **OK**. The implementation gate above is retained
as historical validation, not a newly run follow-up gate.
