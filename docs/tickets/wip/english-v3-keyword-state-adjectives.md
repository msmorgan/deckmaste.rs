---
needs: []
---
# Read a keyword verb's past participle as a state adjective: You may play that card for as long as it remains exiled.

Probably data-only: one class row in `lexicon/core.ron`. Scoped as a full
ticket because of the Reading-count increase it brings under *be*.

## Why

*remain* and *become* take a predicative Complement, and a keyword verb's
participle is an Adjective only where declared. *It remains tapped.* reads 1;
*It remains exiled.* reads 0. Blocker report at trunk `uoqyuxtnowpo`: key
`at:exiled .` 26 sole-blocker faces, but the construction is wider than the
key. `cargo xtask english-v3 grep -i '\b(remains?|becomes?)
(exiled|crewed|attached)\b'` finds **92** faces, all unread (*remain(s)
exiled* 77, *becomes crewed* 4, *become(s) / remain(s) attached* 13); 93
failing lines match, **85** of them the only failing line of their face.

## Goal

A keyword-action or keyword-ability verb can export its past participle as a
participial Adjective, so the predicative Complement of *remain* / *become*
reads: *it remains exiled*, *they remain exiled*, *this Vehicle becomes
crewed*, *becomes crewed for the first time each turn*.

## Analysis

CGEL Ch. 16 §10.1.3, p. 1436: "*They were married.* [ambiguous]"; "in the
verbal interpretation it is dynamic, describing an event, while in the
adjectival interpretation it is static, describing the state resulting from
some prior event". P. 1437: "Adjectival predicative complements are not
restricted to occurrence with *be* but are found also with such verbs as
*seem*, *look*, and *remain*"; "Replacement of *be* by *remain* also serves to
remove the ambiguity of [iii] in favour of an adjectival reading". Those back
an Adjective after *remain*, and two Readings after *be*. Extending the test
to *become* and choosing the declaration mechanism are project rulings.
Glossary: **Adjectival Passive**.

Existing mechanism: `participial_adjective_classes` in
`crates/deckmaste_lexical_source/lexicon/core.ron` (`features:
{"AdjectiveOrigin": "Participial"}, form: PastParticiple, members:
["lexeme:keyword_action/tap", "lexeme:keyword_action/untap",
"core-verb:Block"]`), loaded by `native::add_participial_adjectives`, which
derives `<owner>/adjective`. *remains tapped* reads through `core-verb:Remain`
(`Predicate([PredicativeComplement])`) → `AdjectivalComplement` →
`lexeme:keyword_action/tap/adjective`. Precedent for the *be* pair: "It is
tapped." reads **2** today (adjectival Complement and `PassivePredicate`),
"It is exiled." 1 (passive only), "This Vehicle is crewed." 1 (passive of
`lexeme:keyword_ability/crew/verb`). Attributive use is not doubled: NP "the
tapped creature" 1, "the exiled card" 1.

Orchestrator rulings 2026-10-10:

- KS1. One mechanism: class membership. Add the exile and crew verbs to the
  existing class (data). No new Construction, no frame on *remain* / *become*
  that takes a passive, no hand-written Adjective homograph. If the loader
  rejects the crew verb (it is a `FixedKeyword` `verb:` export, not a `Verb`
  declaration), extending the class to that owner kind is in scope; STOP if
  it needs more than that.
- KS2. The licence is the declared feature `AdjectiveOrigin = Participial` on
  the derived lexeme and the existing predicative-Complement frames. No test
  on a verb, participle or card.
- KS3. Exactly one Reading: *it remains exiled*, *they remain exiled*, *this
  Vehicle becomes crewed*, *this creature becomes saddled* (unchanged, 1).
  After *remain* / *become* only the adjectival analysis exists (CGEL
  p. 1437); a passive Reading there is a defect.
- KS4. Two Readings, both kept: *it is exiled*, *this Vehicle is crewed*
  (adjectival Complement + verbal passive), as "It is tapped." has today
  (CGEL p. 1436 [32iii]). This is the same pair the keyword-verb-export
  landing deferred for *is saddled* (1 → 2 on adding the saddle verb,
  `english-v3-systemic-residuals`, "saddle and *becomes crewed*"): it is the
  CGEL ambiguity, not a duplicate. With this ruling the saddle verb export is
  unblocked; it stays out of this ticket (KS7).
- KS5. One analysis per string, the twins to avoid: (a) attributive *the
  exiled card*, *exiled cards* keep exactly their Readings (216 faces match
  `\bexiled (card|cards|creature|permanent|spell)`); (b) postnominal *cards
  exiled with this creature*, *exiled this way* gain no Adjective-Phrase twin
  of the participial postmodifier; (c) *exile it face down* and the
  ResultativeComplement frame are untouched; (d) *was crewed by two
  creatures* keeps its passive Readings (3 today): an Adjective with a *by*
  Complement is a new analysis, reported if it appears.
- KS6. Newly covered faces are gains. Expected increases on covered faces,
  exactly one shape: finite *be* + *exiled* / *crewed* gains the adjectival
  Reading (34 covered faces match `\b(is|are|was|were|be|been|isn.t|aren.t|
  wasn.t|it.s|they.re) exiled\b`; 0 covered faces for *crewed*, whose one
  *was crewed* face is unread). Report the
  count; any other shape is a defect to fix before landing.
- KS7. Out: declaring *saddle* as a verb; *attached* (KS8).
- KS8. *becomes attached to …* / *remains attached* (13 faces) joins only if
  the class member reads it with no further declaration: the Adjective then
  needs the verb's *to* Complement. Otherwise list the count and leave it.

Additional orchestrator rulings, 2026-10-10 (project rulings):

- KS-R1 amends KS6 to two expected shapes: finite *be* + Adjective and
  *have* + Object + Adjective. The seven Have faces are accepted: the
  existing predicative-Complement frame already gives *tapped* this state
  analysis. Counts are reported from the complete claim-parent diff below.
- KS-R2 accepts the crew Adjective + *by* Adjunct by existing-class parity.
  On the rebuilt claim-parent `yqukrnss` tree,
  `cargo xtask english-v3 probe --text "It was tapped by two creatures."
  --category document --readings 32 --json` enumerates four Readings:
  one adjectival *be* + *by* Adjunct, three verbal passives. The new crew
  assertion is re-spelled to preserve both roles. Evidence:
  `target/english-v3/english-v3-keyword-state-adjectives/parent-tapped-by.json`.
  No Adjective-selected *by* Complement is introduced.
- KS-R3 (accepted review, 2026-10-10) approves the small grammar edit as a
  deviation from the ticket's data-only hope: `AdjectivePremodifierUse`
  reads a declared feature, names no lexeme, and adds no Construction.
  It prevents the forbidden attributive twin.
- The orchestrator's confirmed state/passive pair explicitly authorizes the
  deferred saddle verb export here as pure data, superseding KS7's exclusion.

## Witnesses

Probed 2026-10-10 on the `uoqyuxtnowpo` binary (`--readings 3 --as-card`).
Each line is its face's only failing line and reads 0; the control is the
same line with *tapped* for the participle.

- Gustha's Scepter: "{T}: Exile a card from your hand face down. You may look
  at it for as long as it remains exiled." 0 (*remains tapped* 3)
- Fiend of the Shadows: "… that player exiles a card from their hand. You may
  play that card for as long as it remains exiled." 0 (3)
- Savvy Trader: "… exile target permanent card from your graveyard. You may
  play that card for as long as it remains exiled." 0 (3)
- Jester's Scepter: "… You may look at those cards for as long as they remain
  exiled." 0 (3)
- Mobilizer Mech: "Whenever this Vehicle becomes crewed, up to one other
  target Vehicle you control becomes an artifact creature until end of
  turn." 0 (*becomes tapped* 3)

Sentence probes: "It remains exiled." 0; "They remain exiled." 0; "This
Vehicle becomes crewed." 0; "This Vehicle becomes crewed for the first time
each turn." 0 (*becomes tapped …* inside a *whenever* clause 3). Controls
that must not change: "It remains tapped." 1; "It is tapped." 2; "This
creature becomes saddled." 1; "This creature is saddled." 1; "It remains on
the battlefield." 1; "You may play that card for as long as it remains in
exile." 2; NP "the exiled card" 1. Controls that change by KS4 only: "It is
exiled." 1 → 2; "This Vehicle is crewed." 1 → 2.

Claim-parent re-probes (2026-10-10, `ptvtlvss` before edits, identical to the
claimed `yqukrnss` tree): all five card witnesses remain 0, so none was dropped.
Their substituted *tapped* controls enumerate 4 / 12 / 6 / 6 / 3 Readings,
respectively (limit 16, enumeration complete). The sentence and nominal controls
match the earlier counts except the complete *was crewed by two creatures*
probe, which enumerates 6 rather than the earlier limit-3 observation.
The active saddle witnesses both read 0. Evidence is under
`target/english-v3/english-v3-keyword-state-adjectives/before-probe-*.json`.

## Method

Standard constraints apply. Lexicon data loads at runtime: no rebuild for the
class row. Bounded census, never `--all`: every supported face matching
`\b(exiled|crewed|saddl(e|es|ed|ing))\b` (657 faces after the
authorized saddle extension) plus 300 random
covered controls, `--workers 6 --samples-per-face 0`, compared with `cargo
xtask english-v3 diff` against the claim parent. Zero lost; KS6 for
increases, each classified by shape. Full `cargo test -p` and `cargo clippy
-p … --tests -- -D warnings` for `deckmaste_lexical_source` and
`deckmaste_english_v3` once, at the end (a rebuild after a grammar edit takes
about 3 minutes, the test binary about 7). Tests assert the role (the
Complement of *remain* / *become* is the derived Adjective; the KS3 and KS5
negatives), never an incidental count or a Debug-printed tree. Attestation by
`cargo xtask english-v3 grep` only. On landing, close the "saddle and
*becomes crewed*" entry of `english-v3-systemic-residuals` or rewrite it to
what remains.

## Out of scope

- *Whenever this creature attacks while saddled, …*: verbless *while* +
  Adjective. 26 faces, all unread; "… attacks while tapped, …" reads 0 too, so
  it is not an adjective gap. Own ticket.
- *becomes unattached* (4 faces), *stays* (unknown word), *becomes suspected*
  (1), *remains unchanged* (1).
- *it was crewed by exactly two creatures* (*exactly* is an unknown word).
- *becomes plotted* / *prepared* / *renowned*: declared adjectives, covered.

## Landing record

1. **Census delta**: `ptvtlvss` before edits (claim-parent `yqukrnss` tree)
   → current `ptvtlvss`: 705 → 752 covered faces; 100,275 → 107,758
   Readings. Bounded selection: 657 faces matching
   `\b(exiled|crewed|saddl(e|es|ed|ing))\b` plus 300 covered controls
   (seed 20261008), two overlaps: 955 faces. `diff --lines`: 0 lost,
   47 gained, 0 decreases, 42 increases, 0 digest-only; 0 validation issues.
   Lost faces: none. The finite-*be* state/passive pair affects 36 covered
   faces (35 *exiled*, one *isn't saddled*; Agatha also has the accepted
   Have increase): Agatha's Soul Cauldron; Arcane Artisan; Augusta, Order Returned; Azor's Gateway; Captain Marvel, Shooting Star; Caustic Bronco; Check for Traps; Colfenor's Urn; Cosima, God of the Voyage; Darigaaz Reincarnated; Deathbonnet Hulk; Deathgorge Scavenger; Dinosaurs on a Spaceship; Diregraf Scavenger; Dragonhawk, Fate's Tempest; Feral Appetite; Graveyard Trespasser; Ignis Scientia; Kaya, Orzhov Usurper; Lazav, Familiar Stranger; Mastermind Plum; Mysterious Stranger; Raven Eagle; Redemptor Dreadnought; Riftmarked Knight; Senu, Keen-Eyed Protector; Siren's Ruse; Smirking Spelljacker; Soul-Shackled Zombie; Soulherder; Specter's Shriek; Struggle for Sanity; Taster of Wares; Ugin's Labyrinth; Veiling Oddity; Watcher of Hours.
   Seven covered faces additionally gain the Have + Object +
   *exiled* predicative Complement shape accepted by KS-R1: Agatha's Soul Cauldron (195 → 207); Dark Impostor (39 → 41); Hazel's Brewmaster (47 → 49); Myr Welder (28 → 30); Patchwork Crawler (43 → 45); Spellweaver Helix (26 → 28); Steward of the Harvest (41 → 43).
   Agatha's Have line is 185 → 195 (+10 through existing surrounding
   ambiguities), and its finite-*be* line is 4 → 6; the other six Have
   lines gain two each. All increases have the two accepted shapes.
   Evidence: `target/english-v3/english-v3-keyword-state-adjectives/`
   (`before.json`, `after.json`, `diff.json`, `increased-lines.txt`).
   After refresh (`muztkomo`, same implementation `ptvtlvss`): 766 covered,
   114,320 Readings; original-parent diff 61 gained, 42 increases,
   0 lost/decreases/digest-only. Refresh-only diff: 14 gains, 0 other
   changes, 0 issues. Four preposed-frequency gains from trunk `lvpxmkkl`:
   Cemetery Illuminator, Evelyn, the Covetous, Hauken's Insight, Maralen,
   Fae Ascendant. Ten bare-keyword-mention gains from trunk `zvtzutzx`:
   Archmage's Newt, Blade of the Swarm, Gandalf of the Secret Fire,
   Jhoira of the Ghitu, Market Gnome, Monastery Raid, Runic Repetition,
   Sinister Concierge, Sorceress's Schemes, The Parting of the Ways.
   Own gains remain 47; no refreshed covered face loses any Reading.
   Official evidence: `refresh-after.json`, `refresh-diff.json`,
   `refresh-claim-diff.json`, `refresh-stat.txt` in the same directory.
   Second refresh (`tmrkxvvq`): 773 covered, 114,658 Readings; original
   parent diff 68 gained, 42 increases, 0 lost/decreases, 1 digest-only.
   The seven additional gains come from quoted-ability trunk `lrynurmu`
   combined with the state repair: Emrakul, the Exigent Doom (12),
   Glamorous Outlaw (10), Masked Bandits (10), Rakish Revelers (9),
   Shattered Seraph (10), Spara's Adjudicators (15), Uvilda, Dean of
   Perfection (272). Half-rounded trunk `soxvyspk` changes Corpseweft's
   digest through `MultiplierNounPhrase` → `PredeterminerNounPhrase`;
   its 12 Readings are preserved. Second-refresh-only diff: 7 gains,
   1 digest-only, 0 lost/decreases/increases, 0 issues. Evidence:
   `refresh-2-after.json`, `refresh-2-diff.json`, `refresh-2-claim-diff.json`,
   `refresh-2-stat.txt`. Own gains remain 47.
   Accepted-review refresh and export repair (`tqsuwzum`): 773 covered,
   114,658 Readings, unchanged totals; refresh-only diff has 0 gains,
   losses, decreases or increases and 79 digest-only changes from trunk
   `wqkrtoxr`'s absent optional infinitival negator field. Original-parent
   diff: 68 gains (47 own, 21 trunk), 42 approved increases, 0 lost/decreases,
   50 digest-only (the 49 remaining negator-field changes plus Corpseweft's
   trunk rename), 0 issues. Evidence: `integration-after.json`,
   `integration-diff.json`, `integration-claim-diff.json`,
   `review-refresh-stat.txt`. No refreshed Reading count decreases.
   Integration retry refresh (`lkorvrvl`) brought Ring trunk `sztumtmq`.
   The official bounded diff is entirely unchanged: all 955 face counts
   and digests retained; totals 773 covered / 114,658 Readings; 0 issues.
   Original-parent comparison remains 68 gains (47 own), 42 approved
   increases, 50 trunk-attributed digest-only, 0 losses/decreases.
   Evidence: `retry-after.json`, `retry-diff.json`, `retry-claim-diff.json`,
   `retry-refresh-stat.txt`.
   Post-divergence refresh brought Source attachment trunk `nxxknqvk`.
   Bounded diff: 0 lost/gained/decreases/digest-only, one increase:
   Dawnhand Dissident 1,154 → 3,273 (line 2: 1,151 → 3,270). This is
   trunk's RS12-accepted shape: Source PP following the bare-Object gerund
   *by removing N counters* also attaches to the matrix Predicate, as
   recorded in [that landing](english-v3-remove-source-attachment.md#landing-record).
   Totals: 773 covered, 116,777 Readings, 0 issues. Original-parent diff:
   68 gains (47 own, 21 trunk), 43 increases (42 own approved, one trunk
   RS12), 50 trunk digest-only, 0 lost/decreases. Evidence:
   `resume-after.json`, `resume-diff.json`, `resume-claim-diff.json`,
   `resume-refresh-stat.txt`.
2. **Witnesses** (`ptvtlvss` before/after): Gustha's Scepter 0 → 4;
   Fiend of the Shadows 0 → 12; Savvy Trader 0 → 6; Jester's Scepter
   0 → 6; Mobilizer Mech 0 → 3. Complete limit-16 sentence probes:
   *It remains exiled*, *They remain exiled*, *This Vehicle becomes crewed*
   each 0 → 1; the frequency-Adjunct sentence 0 → 4. *Remains tapped*
   stays 1; *is tapped* stays 2; *becomes saddled* stays 1;
   *is saddled*, *is exiled*, *is crewed* each 1 → 2. The battlefield
   control stays 1; *remains in exile* stays 2. *The exiled card* and
   *exiled cards* stay 1; *cards exiled with this creature* stays 2;
   *cards exiled this way* stays 1. The subjectless `finite-predicate`
   probe *was crewed by two creatures* is 6 → 8:
   all six passive Readings survive, plus two adjectival-Be Readings whose
   *by* phrase is an Adjunct, not an Adjective Complement (accepted by KS-R2 parity).
   The full sentence *It was crewed by two creatures.* reads 4, Reading
   for Reading equal to *It was tapped by two creatures.* after substituting
   the declared verb identity (review probes at `qkvxplko`). Both active
   saddle witnesses 0 → 1; both attached controls stay 0.
3. **Spot checks** (`ptvtlvss`): Gustha's Scepter; Fiend of the Shadows;
   Savvy Trader; Jester's Scepter; Author of Shadows; Brainstealer Dragon;
   Covetous Urge; Cunning Rhetoric — *remain* selects the derived
   `lexeme:keyword_action/exile/adjective`. Mobilizer Mech — *become*
   selects `lexeme:keyword_ability/crew/verb/adjective`. Canyon Vaulter —
   *saddles* uses `lexeme:keyword_ability/saddle/verb`. All ten newly
   covered faces have the intended inspected analysis and zero probe issues.
4. **Deviations and STOPs**: the data-only class expansion produced the
   forbidden attributive twin. Added **Adjective Premodifier Use** to the
   glossary and existing adjective/coordination and Premodifier declarations
   under KS5; the new class declares No, preserving the existing verbal
   Premodifier. Accepted grammar deviation by project ruling KS-R3
   (2026-10-10): it reads a declared feature, names no lexeme, and adds no
   Construction. The user's confirmation of 2026-10-10
   supersedes KS7's saddle exclusion: the saddle verb export is pure data
   in its existing definition; its state Adjective remains. The systemic
   residual entry records the completed repair. Attached is deferred
   (11 faces, 13 grep matches at `ptvtlvss`): the class recipe does not carry
   the needed *to* Complement. The STOP on Have and crew-*by* is resolved
   by KS-R1 and KS-R2 above. No unresolved STOP remains. Accepted review
   authorizes integration. Unattested accepted
   class-parity consequences, review probes at `qkvxplko`: *It remains
   exiled by a creature.* 1; *It enters exiled.* 1 (AdjectivalDepictive);
   *It becomes exiled.* 1. Each is Reading for Reading equal to its *tapped*
   control after verb-identity substitution. Evidence: `review-probe-*.json`.
   These are project-approved class consequences, not corpus attestations.
   Commit then refresh brought grammar/lexicon changes. Harmony resolved
   the suite_5 registration conflict by retaining every incoming module and
   this ticket's module exactly once; the repeated refresh succeeded.
   Incoming code landings: `lvpxmkkl`, `zunolrnv`, `zqnqzmnu`, `koswtsww`,
   `zvtzutzx`. The mandatory refreshed bounded census is recorded above;
   all mandatory full first-refresh suites/clippy passed. Final refresh
   brought `soxvyspk` half-rounded and `lrynurmu` quoted-ability coordination;
   rebuild, bounded census, second full suites and clippy all passed.
   Accepted-review refresh brought `zzwnrkot` do-so/able-to, `wqkrtoxr`
   negated infinitivals and `pxtwrtvu` named modes. The combined build found
   a missing `AdjectivePremodifierUse` export in trunk's new
   `InfinitivalAdjective`; propagated the head's declared value exactly as
   in other adjective constructions. No new Construction or lexeme guard.
   Corrected rebuild, bounded census and all full checks passed.
   Integration was refused because default moved; retry refresh brought
   `sztumtmq` Ring changes. Harmony retained Ring and keyword-state module
   registrations exactly once. The repeated refresh succeeded, rebuild and
   bounded census and all full retry checks passed.
   After the orchestrator repaired external divergence `lqmpuwls`, refresh
   conflicted in the category declarations with `nxxknqvk`. Harmony merged
   trunk's `WholeObjectSourcePresent` and this ticket's
   `AdjectivePremodifierUse`, retaining both declarations. Refresh then
   succeeded; the rebuild and bounded census passed.
5. **Tests** (`ptvtlvss`): `cargo test -p deckmaste_english_v3` passed all six
   test binaries (795 tests). `cargo clippy -p deckmaste_english_v3 --tests
   -- -D warnings` passed. `cargo test -p deckmaste_lexical_source` passed
   (39 unit + 43 integration); `cargo clippy -p deckmaste_lexical_source
   --tests -- -D warnings` passed. `cargo test -p xtask english_v3` passed
   (56 tests). `cargo build -p xtask` passed on both the parity and final trees. Earlier test compilation mistakenly used
   PassivePredicate as a Category; corrected to its Reading variant.
   The initial full suite's only failure was the crew-*by* negative, now
   re-spelled under KS-R2. Clippy then required collapsing two nested
   conditionals in the new assertion; fixed and full checks rerun green. Restored 0 / re-spelled 1 existing test
   (`deferred_saddle_keeps_the_attested_adjectival_state_routes` →
   `saddle_keeps_the_attested_adjectival_state_routes`, same cards and
   asserted state values, with the new passive tested separately) /
   ignored 0 / added 5 (including the corrected KS-R2 assertion) / removed 0.
   Post-refresh at `muztkomo`: `cargo build -p xtask` passed; the same
   five test/clippy commands above all passed, English across all six
   binaries (824 tests), lexical source 39 unit + 44 integration, xtask
   56 tests. Logs: `tests-english-refresh.log`, `clippy-english-refresh.log`,
   `tests-lexical-source-refresh.log`, `clippy-lexical-source-refresh.log`,
   `tests-xtask-refresh.log` under the evidence directory.
   Final second-refresh verification at `tmrkxvvq`: `cargo build -p xtask`
   and all five test/clippy commands above passed; English all six binaries
   (841 tests), lexical source 39 unit + 44 integration, xtask 56 tests.
   Logs use the same names with `-refresh-2.log`. The record is folded into
   implementation `ptvtlvss`; accepted review is recorded below.
   Review nits at `qkvxplko`: `cargo xtask facts check` passes (saddle
   `grammar:` edit). Added the requested negative assertion for *it remains
   destroyed* to the existing witness test (probe 0); no new test module or
   test removal. `cargo test -p deckmaste_english_v3` passed all six binaries
   (841 tests); `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed. Evidence: `tests-english-review.log`, `clippy-english-review.log`,
   `facts-review.log`.
   Final integration checks at `tqsuwzum`, including the export repair:
   `cargo build -p xtask` passed. All seven full test/clippy commands passed:
   the five commands above, plus `cargo test -p deckmaste_lexical` and
   `cargo clippy -p deckmaste_lexical --tests -- -D warnings` for incoming
   lexical code. English passed all six binaries (858 tests); xtask 56 tests.
   Logs: `integration-build-fixed.log`, `tests-english-integration.log`,
   `clippy-english-integration.log`, `tests-lexical-source-integration.log`,
   `clippy-lexical-source-integration.log`, `tests-lexical-integration.log`,
   `clippy-lexical-integration.log`, `tests-xtask-integration.log`.
   Final retry verification at `lkorvrvl`: `cargo build -p xtask` and all
   nine full test/clippy commands passed (the seven integration commands
   above, plus `cargo test -p deckmaste_construction_core` and
   `cargo clippy -p deckmaste_construction_core --tests -- -D warnings`).
   English passed all six binaries (864 tests). Logs use `-retry.log`;
   facts check also passes on refreshed plugin data (`facts-retry.log`).
   Post-divergence verification: `cargo build -p xtask` and the same nine
   full test/clippy commands passed. English passed all six binaries
   (877 tests); xtask passed 56 tests. Logs use `-resume.log`;
   facts check passed again (`facts-resume.log`).
   Citation checks: 0 noncompliant, 0 stale; piped diff audit selected 0
   citation sites, so none needed bless.
6. **Timing**: the corrected complete bounded census took 232,110 milliseconds,
   six workers, at `ptvtlvss`.
