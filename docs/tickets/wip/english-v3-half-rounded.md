---
needs: []
---
# Read the fraction predeterminer *half* and its rounding supplement: You lose half your life, rounded up.

## Why

*half*, *rounded* and the verb *round* are not in the vocabulary, so every
line with them reads 0. Blocker report at trunk `uoqyuxtn`
(`target/english-v3/blockers-wave7.txt`): `word:half` / `word:rounded`.
`cargo xtask english-v3 grep 'rounded (up|down)'` finds **77** faces, all
unread, **68** with that line as their only failing line;
`grep -i '\bhalf\b'` finds **84** faces, all unread, **73** sole. 73 of the 77
*rounded* faces also have *half* in the same line, so the faces need both
parts; that is why this is one ticket.

Shapes in the failing lines (approximate matches): *half their / your life* 34, *half
their library* 9, *half the creatures / permanents / cards … they control /
in their hand* 14, *half the number of …* 7, *half their / your starting life
total* 9, *half that damage* 3, *half X damage / life / cards / … counters* 8,
*half that many …* 3, *half that player's / card's / creature's …* 5,
*half of their library* 1. *rounded up* 56, *rounded down* 27; 69 of the 83
are sentence-final, the rest precede a comma, a colon or a closing quote.

## Goal

*half* reads as a predeterminer Modifier of a definite NP wherever that NP
reads today, and *, rounded up* / *, rounded down* reads as a supplement
after the clause or cost that contains the fraction, with byte-exact
realisation.

## Analysis

CGEL Ch. 5 §12(b), p. 434, [3i]: "*John had already wasted* [*half his share
of the legacy*]"; "Predeterminers expressing fractions have the form of NPs.
*Half*, *quarter*, *third*, etc., are nouns"; of *half*, "it can occur
without a determiner"; and "There is an alternative construction in which the
fractional noun functions as head of the topmost NP, with a following
partitive complement: *half of his share*". Ch. 14 §9, p. 1265, [4iib]:
"*Born in Aberdeen, Sue had never been further south than Edinburgh*"; "The
underlined non-finites are supplements with the main clause as anchor". Those
back the predeterminer function, the noun Category, bare *half* and the
existence of a past-participial supplement anchored on a clause. Everything
else here is a project ruling, including the trailing position of the
rounding supplement.

Existing routes (`crates/deckmaste_english_v3/src/declarations.rs`):
`MultiplierNounPhrase` (`[modifier: lexical(Adverb) | lexical(Noun), " ",
head: NounPhrase]`, `require modifier.PredeterminerUse = Multiplier`,
`require head.Definiteness = Definite`) with feature `PredeterminerUse { No,
Multiplier }`; `lexeme:Multiplier/Double` in `lexicon/core.ron` is the model
noun lexeme. The comma supplement is the `Supplementation` schema
(`[anchor: node, ", ", supplement: PrepositionPhrase, "."]`, instances on
`Sentence` over `Clause` and on `Ability` over `KeywordPhrase`) and
`MedialSupplementation`; `ParticipialDepictive` and
`ParticipialPostmodifiedNominal` are the existing bare-passive consumers.
The multiplier landing (`docs/tickets/done/english-v3-multiplier-predeterminer.md`)
left *half* out.

Orchestrator rulings 2026-10-10:

- HR1. *half* is a Noun lexeme in `lexicon/core.ron` declaring a fraction
  value of `PredeterminerUse`; `MultiplierNounPhrase` is refined to read the
  declared predeterminer value (a table over `PredeterminerUse`), not a second
  Construction. Rename the Construction if its name no longer fits; the
  glossary entries **Predeterminer** and **Predeterminer Use** are amended in
  the landing.
- HR2. The host is definite, as for multipliers, read from the host's
  exported `Definiteness`. *half X damage*, *half X life*, *half X cards*
  (8 lines) have no definite host: in scope only if they fall out of a host
  the grammar already calls definite or scalar (MP4 of the multiplier
  ticket); otherwise list them with counts and leave them unread.
- HR3. *round* is a core verb in `lexicon/verbs.ron` with the particle frames
  the corpus attests (*up*, *down*; both lex today as
  `vocab:Adverb/Up|Down` and `vocab:Preposition/Up|Down`). Its past
  participle is an ordinary bare passive.
- HR4. The rounding supplement refines `Supplementation` /
  `MedialSupplementation`: the supplement slot admits a bare-passive
  participial phrase whose head verb declares the supplement use. The licence
  is a declared feature of the verb lexeme, never the word *rounded*. The
  anchor is the clause (project ruling; CGEL p. 1265 supports a clause
  anchor, not this position). No NP-internal postmodifier Reading of
  *life, rounded up* is added.
- HR5. One analysis per string. *half their life* reads only as
  predeterminer + NP: not as a head noun *half* with a bare NP dependent, and
  not through `PartitiveNounPhrase` (that is *half of their library*, 1 face:
  in scope only if the existing partitive route reads it once the noun
  exists). *It deals double that damage instead.* and *Copy that spell
  twice.* keep exactly their Readings.
- HR6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected (all three words are new); report any
  with its shape before landing.

- HR-R1 (orchestrator ruling, 2026-10-10). Retain both
  `[half [that player's life total]]` and `[[half that player]'s life total]`.
  Both are grammatical bracketings: a fractional Predeterminer may modify a
  definite NP that becomes a Genitive Determiner. Tests assert the intended
  whole-possessive-NP constituency is present by value, without excluding the
  other Reading. Do not add a feature solely to suppress the second scope.

## Witnesses

Each is the face's only failing line at `uoqyuxtn`. Probed 2026-10-10 with
the `uoqyuxtn` binary (`--readings 3`, `--as-card` where the name occurs):
witness 0, control as shown (3+ = enumeration capped). Re-probe on the claim
parent first.

- Grievous Wound: "Whenever enchanted player is dealt damage, they lose half
  their life, rounded up." (control "… they lose 2 life." 1)
- Cut Your Losses: "Target player mills half their library, rounded down."
  (control "Target player mills three cards." 1)
- Endless Ranks of the Dead: "At the beginning of your upkeep, create X 2/2
  black Zombie creature tokens, where X is half the number of Zombies you
  control, rounded down." (controls "… where X is twice the number of Zombies
  you control." 2; "… where X is the number of Zombies you control." 2)
- The Endstone: "At the beginning of your end step, your life total becomes
  half your starting life total, rounded up." (control "… becomes your
  starting life total." 1)
- Heartless Hidetsugu (document): "{T}: Heartless Hidetsugu deals damage to
  each player equal to half that player's life total, rounded down." (control
  without "half … , rounded down" 3+)
- Malignus: "Malignus's power and toughness are each equal to half the
  highest life total among your opponents, rounded up." (control without
  "half … , rounded up" 2)

Also 0 today: "That player loses half their life."; NP "half that player's
life total" (NP "twice that player's life total" 2). Controls that must not
change: "It deals double that damage instead." 1; "Copy that spell twice." 1;
"Put twice that many +1/+1 counters on it." 1; "Prevent double that damage."
1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching

    \b(half|third|tenth|twice|double|triple|rounded?)\b

(704 faces, case-insensitive) plus 300 random covered controls (`cargo xtask
english-v3 diff --manifest … --random 300 --seed 20261008`), `--workers 6
--samples-per-face 0`, compared with `cargo xtask english-v3 diff` against
the claim parent. Zero lost; HR6 for increases. Full `cargo test -p` and
`cargo clippy -p … --tests -- -D warnings` for `deckmaste_english_v3` and
`deckmaste_lexical_source`, once, at the end: a rebuild after a grammar edit
takes about 3 minutes and the test binary about 7, so batch edits; lexicon
data loads at runtime. Tests assert the role (the fraction as predeterminer
Modifier, the definite host, the participial supplement and its anchor, the
HR5 negatives), never an incidental count or a Debug-printed tree.
Attestation by `cargo xtask english-v3 grep` only.

## Out of scope

- *a third of their life*, *a tenth of the creatures …* (2 faces; no *half*):
  fraction noun as head with a partitive Complement. *a third of their life*
  reads 0 without the supplement too.
- *Round up each time.* / *Round down each time.* (4 faces, each also needs
  *half*): in scope only if the HR3 verb declaration reads the imperative
  with no further declaration; otherwise list.
- *divided evenly, rounded down, among …* (2 faces; *evenly* unknown).
- *less than or equal to half your starting life total* (4 faces): in scope
  only as a by-product of HR1.
- *half X …* beyond HR2.

## Landing record

1. **Census delta** (`soxvyspk`): bounded manifest, 995 faces; covered
   748 → 814, total Readings 25,118 → 25,855. Lost none; gained 66;
   decreases 0; previously covered increases 0; digest-only 79 (the
   `MultiplierNounPhrase` → `PredeterminerNounPhrase` rename). Wrong-analysis
   gains found: none. `before.json` was measured on the empty `soxvyspk`
   equivalent to claim parent `mrsslony`; `after.json` and `diff.json` are
   under `target/english-v3/english-v3-half-rounded/`. Selector:
   `\b(half|third|tenth|twice|double|triple|rounded?)\b`, case insensitive,
   704 attested faces plus 300 random covered faces, seed 20261008 (995
   unique). Standard `english-v3 diff --lines` supplies the comparison.
   After refresh (`ztqopwrv`, empty child of `soxvyspk`): covered 816,
   Readings 25,912; combined diff: gained 68, lost/decreases/increases 0,
   digest-only 79. The additional gains are Complete the Circuit (20
   Readings, preverbal next from `zunolrnvsuqr`) and Personal Incarnation
   (37 Readings, NP focus from `zqnqzmnumpyr`). The second refresh's
   `koswtswwympz` copy-exception licence leaves all 995 face counts and
   digests unchanged. Final evidence: `after-refresh-2.json`,
   `diff-final.json`, and separate `diff-refresh*.json` comparisons.
2. **Witnesses** (`soxvyspk`): all eight probed 0 before implementation.
   After, complete enumerations: Grievous Wound 1; Cut Your Losses 1;
   Endless Ranks of the Dead 2; The Endstone 1; Heartless Hidetsugu 17;
   Malignus 2; “That player loses half their life.” 1;
   “half that player's life total” 2. Controls before → after:
   Grievous 1 → 1; Cut 1 → 1; Endless twice 2 → 2 and plain number 2 → 2;
   Endstone 1 → 1; Hidetsugu 10+ (enumeration capped at 10) → 11 complete;
   Malignus 2 → 2; twice-genitive NP 2 → 2; double-damage, spell-twice,
   twice-that-many-counters and prevent-double-damage each 1 → 1.
3. **Spot checks** (`soxvyspk`, ten newly covered faces): Grievous Wound,
   fraction over their life, rounding anchored on the triggered effect's
   finite Clause; Cut Your Losses, fraction over their library, finite-Clause
   rounding; Endless Ranks of the Dead, fraction over the number of Zombies,
   rounding on the finite where-binding Clause; The Endstone, fraction over
   your starting life total, rounding on the becomes Clause; Heartless
   Hidetsugu, fraction over the whole genitive NP, plus HR-R1's retained
   genitive-possessor scope, rounding on the deals Clause; Malignus, fraction
   over the highest life total among your opponents, rounding on the finite
   Clause; Lurking Evil, fraction over your life, rounding anchored on the
   action Cost; Garza's Assassin, the same action-Cost analysis inside
   Recover; Aspect of Wolf, separate X/Y finite where-binding Clauses with
   their respective down/up Supplements; Infernal Contract, fraction over
   your life and rounding on the lose Clause. Full-face probes are
   `spot-0.json` through `spot-9.json` in the evidence directory.
4. **Deviations and STOPs**: `ParticipialSupplementation` is an embedded
   schema, separate from terminal PP `Supplementation`, because the latter's
   initial-capitalization boundary prevents lowercase triggered and adjunct
   Clauses. `ImperativeClause`/`SupplementAnchorImperative` supply a disjoint
   Clause anchor without duplicating declarative Readings or changing
   existing Clause summaries; the existing finiteness test remains unchanged.
   Refresh's category-interface conflict was resolved by preserving both
   trunk's NounPhraseSeries `FocusedUse` and this change's
   SecondaryPredicateSeries `ParticipialSupplementUse`; no route was removed.
   The licence is declared `ParticipialSupplementUse`, propagated through
   secondary projections and conjoined for coordination. Added glossary
   entries Participial Supplement Use and Imperative Clause; amended
   Predeterminer and Predeterminer Use. STOP over genitive scope resolved by
   **HR-R1 (orchestrator ruling, 2026-10-10)**: retain both bracketings and
   assert the intended constituency is present by value. The landed
   multiplier already has the same pair on claim-parent trunk (`mrsslony`):
   `[twice [that player's life total]]` and `[[twice that player]'s life total]`.
   No multiplier restriction was added. HR2 residuals (`soxvyspk`): half X
   remains unread on 7 faces/9 matches: Wan Shi Tong, Librarian; Banshee;
   Hydroid Krasis; Eternal Flame; The Goose Mother; Contaminated Drink;
   Brass Infiniscope. Bare “half of their library” remains unread; the
   attested Ulamog, the Defiler line instead reads “the top half of their
   library” through the existing determined, premodified, PP-postmodified
   Noun route; its face is newly covered. No new partitive route or
   NP-internal comma postmodifier was added.
5. **Tests** (`soxvyspk`): `cargo test -p deckmaste_english_v3` passed all
   801 tests; `cargo test -p deckmaste_lexical_source` passed all 82 tests.
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Restored 0 / re-spelled 10 existing multiplier tests for the construction
   rename / ignored 0 / added 11 (including the two HR-R1 role tests) /
   removed 0. Initial verification exposed a Clause-summary regression,
   fixed by the disjoint imperative anchor, and the two genitive-scope
   assertions, re-spelled under HR-R1. Citation checks: 0 noncompliant,
   0 stale; diff audit 0 newly cited sites. No CR registration was added.
   Required checks repeated after both code/data refreshes (`ztqopwrv`):
   `cargo build -p xtask`, full tests and clippy for both changed crates,
   all passed; final English suite 823 tests, lexical-source suite 82.
   Assurance counts above describe this ticket's changes, excluding tests
   brought in from trunk. Neither construction-core nor plugin definitions
   changed, so no xtask-test or Lean check was required.
6. **Timing** (`soxvyspk`): bounded census wall time 27.77 s under
   `/usr/bin/time -v`, 6 workers, samples per face 0. Required bounded
   refresh rechecks (`ztqopwrv`): 27.36 s, then 12.17 s, with the same
   manifest/workers/sampling. No full-corpus census.
