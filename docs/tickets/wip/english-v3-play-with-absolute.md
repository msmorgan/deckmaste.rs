---
needs: []
---
# Read intransitive *play* with a *with* + subject–predicate Adjunct: Players play with their hands revealed.

## Why

Blocker report at trunk `uoqyuxtnowpo`: key `at:play with` 23 sole-blocker
faces. `cargo xtask english-v3 grep -i '\bplays? with\b'` finds **26** faces;
25 are unread (the covered one is the card name *Play with Fire*), 25 failing
lines match and **all 25** are the only failing line of their face: *Play
with the top card of your library revealed.* 12, *Players / Your opponents
play with their hands revealed.* 5, *Players play with the top card of their
libraries revealed.* 3, others 5.

Two things are missing, and only the first shows in the probes:

1. *play* has no intransitive frame: "Players play." 0 (`keyword_actions/
   play.ron` declares `frame_set: Transitive`).
2. The *with* phrase is a subject + predicate clause (*their hands* +
   *revealed*), and the grammar has no such Complement of *with*. What it has
   instead reads a neighbouring string by another analysis: "Players attack
   with their hands revealed." 1 and "Players draw cards with their hands
   revealed." 3, every Reading taking *their hands revealed* as an NP whose
   head *hands* has a participial postmodifier (`ParticipialPostmodifiedNominal`).

Adding only the frame would cover the 25 faces through that NP analysis. It
says "with those hands that are revealed", not "with their hands in the
revealed state": the wrong reading for these cards.

## Goal

1. *play* reads with no Object: *Players play …*, imperative *Play …*, *that
   player plays …*, bare *play* under causative *have*.
2. *with* takes a clause Complement with an overt Subject and a predicative
   past participle or Adjective: *with their hands revealed*, *with the top
   card of your library revealed*, *with that card revealed in their hand*.
   Under PW-R1 (orchestrator ruling, 2026-10-10), the marked *with* role
   is a selected Complement of the verb, not a general Adjunct.

## Analysis

CGEL Ch. 14 §10(a), p. 1267, "Complement to *with* and *without*": "*They
were standing against the wall* [*with their hands above their heads*]"; "The
underlined clauses have subject + predicate structure, but with no verb in
the predicate. *With* and *without* do not license finite complements, but
non-finites are found in addition to the verbless forms (see §8.3 above)".
That backs *with* + a subject–predicate clause, verbless or non-finite.
Treating *revealed* as the predicate of that clause, and everything about
licences, is a project ruling. Glossary: **Verbless Clause**.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`VerblessClause` (`[predicate: lexical(Adjective)]`, `predicate.
VerblessPredicateUse = Yes`: subjectless, for *if able*) and
`VerblessClauseComplementPreposition` (`head.VerblessClauseComplement = Yes`,
declared today by `vocab:Preposition/If` in `lexicon/core.ron`), which
already cites p. 1267. Owner of the verb:
`plugins_v2/builtin/macros/keyword_actions/play.ron` (`grammar: Verb(bare:
"play", frame_set: Transitive)`).

Orchestrator rulings 2026-10-10:

- PW1. Data: give *play* an intransitive frame through its owning plugin
  definition (`frame_set: Custom` with the empty and the Object frame). Not
  in core, not a second verb.
- PW2. Refine `VerblessClause` / `VerblessClauseComplementPreposition`: a
  form with an overt Subject NP followed by its predicate (past-participial
  or adjectival). No new Preposition Construction beside it unless the
  existing one cannot carry a second form; say so in the landing record if
  one is added.
- PW3. The licence is declared on the Preposition (which clause Complement it
  takes: subjectless reducible for *if*, subject + predicate for *with*) and
  read by the Construction. No test on *with*, *play*, *revealed* or a card.
  If the existing two-valued `VerblessClauseComplement` cannot express the
  difference, extending that feature is in scope.
- PW4. Both analyses of *with their hands revealed* are grammatical strings
  of English and the NP-postmodifier one already exists for other verbs. Do
  not suppress it with a guard. A witness counts as gained only when the
  subject–predicate analysis is among its Readings; the expected count per
  witness is therefore 2 (clause Complement, NP with postmodifier), and the
  landing record names both shapes. A face covered by the NP analysis alone
  is recorded as covered by a wrong analysis, not as a gain.
- PW5. One analysis per string otherwise. The twins to avoid: (a) *You may
  play that card.*, *Play lands.* keep exactly their Readings: the
  intransitive frame adds none where an Object is present, and *with* +
  clause does not attach inside an Object NP; (b) *enters with a +1/+1
  counter on it* keeps exactly 2: note that *a +1/+1 counter on it* is itself
  a subject + PP predicate candidate (CGEL's *with his hands in his
  pockets*), so a new Reading there is the predictable increase: report the
  count and shape on covered faces, and STOP if it exceeds one extra Reading
  per *with* phrase; (c) the card name *Play with Fire* keeps its name
  Reading only.
- PW6. Newly covered faces are gains (subject to PW4). Increases on
  previously covered faces are possible only in the PW5(b) shape (*with* + NP
  + PP or participle after a verb); report them with counts before landing.
  If the count is large, restricting the new form to a participial or
  adjectival predicate (no PP predicate) is the authorised narrowing, since
  only that is attested as an absolute here.

### Claim-parent probes and resolved STOP (2026-10-10)

Measured with the workspace-built binary on claim parent `qpqyvsou`, before
implementation: all five witnesses 0; the three intransitive sentence probes
0; the five unchanged controls 1 each; attack-with-revealed 1; counter-entry
2. No witness was dropped. Evidence is under
`target/english-v3/english-v3-play-with-absolute/`.

Draft `wwzpptst` adds the empty frame after the existing Object frame, declares
an `OvertSubject` value of `VerblessClauseComplement` on *with*, and retains
the subjectless `Yes` value on *if*. The existing preposition Construction
agrees its licence with the Complement's shape; two subject–predicate
Constructions supply an adjectival or bare-passive predicate. No PP predicate
is admitted and no lexeme-naming guard is added.

Initial STOP (resolved by PW-R1 below): the general implementation conflicts
with PW4's expected witness count and the unchanged *look* control. On
`wwzpptst`, Telepathy and Enduring Renewal have 2 Readings each. Field of
Dreams and Future Sight have 7 each: 2 subject–predicate clause Complements
(the Subject NP has two existing Modifier scopes) and 5 NP-postmodifier
analyses. Sen Triplets' complete printed line has 84 Document Readings; its
previously failing *This turn … revealed.* sentence has 4. These probes
enumerated completely. The *look … with it revealed* control rises 1 → 3:
the existing Object-NP analysis remains, plus clause Adjuncts at lexical-verb
and modal levels. Attack-with-revealed rises 1 → 2; counter-entry stays 2;
the other four unchanged controls stay 1. The three intransitive sentence
probes now have 1, 1 and 2 Readings respectively.

The manifest implements Method's 475 surface matches plus 300 sampled covered
*with* faces and 300 general covered controls (seed 20261008), deduplicated
to 1,064 faces. No census or test run has occurred after this STOP; no landing
record is claimed. Resolve whether these retained general Readings revise
PW4/PW5's counts or require a narrower grammatical ruling before continuing.

### PW-R1 — selected Complement (orchestrator ruling, 2026-10-10)

PW-R1 supersedes the general Adjunct route in PW2/PW3/PW5/PW6. Declare a
marked *with* role on the verb's frame whose Complement is the overt
subject–predicate clause. Only verbs declaring that frame can select it;
no general absolute Adjunct route and no word test in the grammar. Existing
`MarkedRole` frame atoms can express it, with a Category projection requiring
an overt Subject; no new frame kind is needed.

Zero Reading-count increases are allowed on covered faces without
`\bplays? with\b`. The *look … with it revealed* control must remain 1,
attack-with-revealed must remain 1, and counter-entry must remain 2. Witnesses
must include the selected-frame analysis for coverage to count as a gain;
report each count and its multipliers, retaining the existing NP-postmodifier
analysis beside it as PW4 permits. The initial STOP's expected counts are
superseded by this shape-based accounting.

Bounded selector: `\bwith\b[^.]* (revealed|tapped|attached|face (up|down))`
plus `\bplays? with\b`, plus 300 covered controls (seed 20261008). No full
corpus run or Lean checks. Tests assert the selected Complement role. Full
suites and clippy precede the six-item record, commit and refresh. Incoming
`crates/` or `plugins_v2/` changes require rebuild, bounded re-census and full
suites/clippy before the review STOP. Do not integrate until instructed.

## Witnesses

Each is the face's only failing line. Probed 2026-10-10 on the `uoqyuxtnowpo`
binary (`--readings 3 --as-card`): 0.

- Telepathy: "Your opponents play with their hands revealed."
- Enduring Renewal: "Play with your hand revealed."
- Field of Dreams: "Players play with the top card of their libraries
  revealed."
- Future Sight: "Play with the top card of your library revealed."
- Sen Triplets: "… This turn, that player can't cast spells or activate
  abilities and plays with their hand revealed. …" (whole line)

Sentence probes, 0 today: "Players play."; "Players play with their hands.";
"Play with the top card of your library." Controls that must not change:
"You may play that card." 1; "Play lands." 1; "Players can't play lands." 1;
"Players attack." 1; "You may look at that card with it revealed." 1.
Controls expected to change only by PW5(b)/PW6: "Players attack with their
hands revealed." 1; "It enters with a +1/+1 counter on it." 2.

## Method

Standard constraints apply; PW-R1 controls the selected role and supersedes the
original general Adjunct plan. Plugin `grammar:` data loads at runtime. Use the
PW-R1 selector and 300 covered controls, `--workers 6 --samples-per-face 0`,
with a prebuilt binary under `/usr/bin/time -v`; never `--all`. Compare using
`cargo xtask english-v3 diff --lines`. Check every gained *play with* line for
the selected-frame analysis. Zero lost or decreased Readings; zero increases
outside *play with*. Full suites and clippy for English and crates consuming
the changed plugin data; no Lean checks. Tests assert the selected Complement,
its Subject and predicate, and unchanged control values. Corpus attestation
uses `cargo xtask english-v3 grep`.

## Out of scope

- Temporal Aperture (*for as long as that card remains on top of your
  library, play with … revealed and you may play that card …*) and Stromgald
  Spy (*have defending player play with their hand revealed for as long as
  …*): count them only if the whole face reads.
- *with* + gerund-participial clause; *without* + clause.
- *play* + *as though* / *any time* Adjuncts.

## Landing record

1. **Census delta** (`wwzpptst`, parent-equivalent `qpqyvsou` → selected
   frames): 433 bounded faces; covered **389 → 413**, total Readings
   **35,813 → 36,216**. Lost none; gains **24**; decreases **0**; increases
   **0**, including outside *play with*; digest-only **0**; issues **0**;
   wrong-analysis-only gains **0**. Every gain has the selected analysis.
   Selector: PW-R1's `\bwith\b[^.]* (revealed|tapped|attached|face (up|down))`
   plus `\bplays? with\b` (134 faces), plus 300 covered controls from
   `english-v3 diff --manifest --random 300 --seed 20261008`, deduplicated.
   Evidence: `target/english-v3/english-v3-play-with-absolute/`:
   `selected-manifest.json`, `before-parent.json`, `after.json`,
   `parent-diff.json`, `gain-shapes.json`. The parent-equivalent run restores
   only the original *play* data: the new clause forms then have no Document
   route. The original `mqrlzszkuszn` census comparison also contains seven
   upstream gains: Backslide, Dermoplasm, Master of the Veil and Weaver of
   Lies (`zvtzutzx`, keyword mentions); Hedge Whisperer, Old Man of the Sea
   and Vedalken Shackles (`wqkrtoxr`, negated infinitivals). Its 13 digest-only
   changes also predate this feature, isolated in `ancestor-diff.json`.
   Post-refresh `after-refresh.json` / `refresh-diff.json`: all 433 faces
   unchanged; covered 413 and Readings 36,216; no gains, losses, decreases,
   increases, digest-only changes, source changes, or issues. The second
   refresh is also unchanged (`after-refresh-2.json`, `refresh-2-diff.json`).
   Third refresh (`after-refresh-3.json`, `refresh-3-diff.json`): covered
   413, Readings 36,234; 0 gains/losses/decreases/digest changes, 1 increase.
   Azor's Gateway 36 → 54 is inherited from `ptvtlvss` / `qkvxplko` /
   `tqsuwzum`: the finite-*are* + adjectival *exiled* Complement adds
   18 Readings alongside the 36 preserved prior Readings. This is the
   keyword-state-adjectives ticket's accepted shape under project ruling KS6,
   recorded there by name; it is not a *play with* increase. Its complete
   probe and supported-corpus attestation are `refresh-3-azor-probe.json`
   and `refresh-3-azor-attestation.json`. Own delta remains 24 gains with
   zero covered-face increases.
2. **Witnesses** (`qpqyvsou` before: all 0; `wwzpptst` after): Telepathy **2**
   and Enduring Renewal's witness **2**, each one selected clause and one
   retained NP participial Postmodifier. Field of Dreams **7** and Future
   Sight's witness **7**: two selected clauses (the Subject NP's two
   premodifier/postmodifier scopes) and five existing NP-postmodifier
   analyses, varying Modifier scope and the participle's nominal host.
   Sen Triplets' failing Sentence **3**: one selected clause and two NP
   analyses, with the PP scoping *plays* or the coordinated Predicate. Its
   complete printed line is **63** (21 selected / 42 NP), multiplied by the
   other Sentences' 21 combinations. Whole-face totals for Enduring Renewal
   and Future Sight are 7 and 24 because of their other Abilities. Controls:
   *look … with it revealed* 1, attack-with-revealed 1, counter-entry 2;
   transitive *play* and *Play with Fire* retain their complete values.
3. **Spot checks** (`wwzpptst`): Goblin Spy, Lantern of Insight, Wizened
   Snitches, Revelation, Wandering Eye, Courser of Kruphix, Oracle of Mul
   Daya, Melek, Izzet Paragon, Conspicuous Snoop, Garruk's Horde. Each has a
   selected *with* frame whose Complement retains its Subject NP and
   bare-passive predicate. Temporal Aperture remains unread; Stromgald Spy
   reads as a whole face (26) and its causative constituent is tested.
4. **Deviations and STOPs**: the initial general Adjunct draft triggered the
   count/control STOP; PW-R1 resolved it with selected complementation.
   Retired its *with* lexical licence and general grammar route. Added empty
   and marked-clause frames after the original Object frame; no new frame
   kind, loader or compiler code. Added exactly two Constructions,
   `SubjectAdjectiveVerblessClause` and `SubjectParticipialClause`, charged
   against the 250-Construction ceiling: current refreshed parent 240 →
   landing 242 (8 below the ceiling). They distinguish adjectival and
   bare-passive predicates; the existing preposition route
   remains subjectless, and the frame Category projection requires an overt
   Subject. These are PW2/PW-R1 distinctions, not word guards. No glossary
   gap. The extra bounded data-only parent run separates ancestor changes
   from this feature without rebuilding the grammar. Refresh brought
   `sztumtmq` (Ring) and `pxtwrtvu` (named modes), including parsing, lexical,
   construction-core and plugin changes. The sole conflict was additive
   module registration in `suite_6`; both registrations were retained. A
   subsequent refresh succeeded as a no-op. The second refresh brought
   `nxxknqvk` (remove-source-attachment), including grammar and core lexicon
   data. Its additional `suite_6` registration conflict was resolved by
   retaining both modules; the retry succeeded as a no-op. A third,
   conflict-free refresh brought keyword-state-adjectives (`ptvtlvss`,
   `qkvxplko`, `tqsuwzum`), including grammar, lexicon and saddle plugin data.
5. **Tests** (`wwzpptst`): `cargo test -p deckmaste_english_v3 -p
   deckmaste_lexical_source -p deckmaste_construction_core -p
   deckmaste_semantics_v2` passed in full; its first build caught an ambiguous
   `.into()` in the new fixture, corrected before the successful full run.
   `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source -p
   deckmaste_construction_core -p deckmaste_semantics_v2 --tests -- -D warnings`
   passed. Post-refresh `cargo build -p xtask` passed; full
   `cargo test -p deckmaste_english_v3 -p deckmaste_lexical_source -p
   deckmaste_construction_core -p deckmaste_semantics_v2 -p deckmaste_lexical`
   and `cargo clippy -p deckmaste_english_v3 -p deckmaste_lexical_source -p
   deckmaste_construction_core -p deckmaste_semantics_v2 -p deckmaste_lexical
   --tests -- -D warnings` passed. `cargo test -p xtask english_v3`
   also passed (56 matching tests). The second refresh rebuilt xtask and
   repeated the same full five-crate suite and clippy commands successfully.
   Evidence: `refresh-2-build.log`, `refresh-2-tests.log`,
   `refresh-2-clippy.log`. No new construction-core/plugin changes required
   another xtask test run. Third refresh rebuilt xtask and repeated the
   same full five-crate suites and clippy successfully; `cargo test -p xtask
   english_v3` also passed again (56 matching tests). Third-refresh logs:
   `refresh-3-build.log`, `refresh-3-tests.log`, `refresh-3-clippy.log`,
   `refresh-3-xtask-tests.log`. First-refresh logs: `refresh-build.log`,
   `refresh-tests.log`, `refresh-clippy.log`, `refresh-xtask-tests.log`.
   `cargo xtask facts check` passed after the *play.ron* `grammar:` edit;
   generated facts are up to date (`facts-check.log`).
   Restored 0 / re-spelled 0 / ignored 0 / added 10 / removed 0.
   The new module is registered once in `suite_6`. Citation check: 0
   noncompliant and 0 stale; `jj --no-pager diff --git | cargo xtask cite audit
   --diff` selected 0 new sites, so no new CR registrations need blessing.
6. **Timing** (`wwzpptst`): landing bounded census **69.73 s** wall time,
   6 workers, `--samples-per-face 0`, prebuilt binary under `/usr/bin/time -v`.
   The same post-refresh bounded census took **91.13 s** wall time; the
   rebuild took **2m 14s**, and full suite compilation took **2m 26s**.
   Second-refresh rebuild: **2m 10s**; bounded census: **33.64 s**.
   Third-refresh bounded census: **42.01 s**.
