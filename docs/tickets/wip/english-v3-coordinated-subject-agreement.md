---
needs: []
---
# Give a coordinated Nominal plural agreement: its power and toughness are equal to …

## Why

Defect. A Subject whose head is an *and*-coordination of Nominals under one
Determiner (*its power and toughness*) agrees as singular: "Its power and
toughness is 2." reads 1 while "Its power and toughness are 2." reads 0. The
singular Reading is a wrong analysis that parses; the plural, which is the
only agreement the supported corpus attests, does not read.

On census `xnkuznpo` (docs-only descendant of `vvxtllnq`; 32,828 supported
faces, 23,459 covered, 9,369 unread; wave-4 recon of 2026-10-08), the recon
bucket "coordinated property subject plural concord (*power and toughness
are*)" touches **178** unread faces and is the sole cause on **10**. Those 10
are all verbless *with power and toughness each equal to …* units (Karn's
Touch, March of the Machines, Porcelain Gallery …) whose controls without
*each* also read 0, so this fix alone covers none of them. The real exposure
is the finite clause: 164 unread faces carry *power and toughness are* or
*aren't*, 162 of them as *are each* (Lord of Extinction, Gaea's Liege, Angry
Mob). On 117 of those faces, every failing unit is that clause or a floated
*each* match, and no other recon STRONG bucket matches. Those faces also need
`english-v3-floated-each-quantifier`, which `needs` this ticket. Together the
two recon buckets touch 278 unread faces and are the sole cause on 197; that
figure includes the 10 verbless faces above. The 117 figure is a
regex pass over the recon's failing units, not a recon bucket. Counts are
surface counts, not gain forecasts.

The supported corpus has no *power and toughness is* (0 tokens). NP-level
coordination already resolves correctly: "Target creature you control and
target creature an opponent controls gain indestructible until end of turn."
reads 1, while the *gains* version reads 0.

Probes (admitted roots, `--category Sentence --readings 1`): "Its power and
toughness are equal to this creature's power and toughness." 0, while "Its
power and toughness is equal to this creature's power and toughness." 1 and
"Its power is equal to this creature's power." 1; "Lord of Extinction's
power and toughness are equal to the number of cards in all graveyards." 0,
while the *is* version reads 1.

## Goal

An *and*-coordination of Nominals that heads a Subject NP takes plural
agreement when its shared Determiner does not fix singular Agreement Number,
realised byte-exactly. A singular-selecting Determiner fixes singular NP
agreement under the owner ruling below. The singular Reading of the
possessive *its power and toughness* Subject is retired. Alternative (*or*) coordination keeps its
existing resolution (*target creature or Vehicle gets*, attested).

## Analysis

CGEL, Ch. 5 §18.4(a), p. 507: a subject that is an *and*-coordination of NPs
generally takes a plural verb, whatever the number of the coordinates,
because the coordination denotes a set of at least two members. Pp. 507–508,
[26ii] and [27ii], extend this to coordinations of nouns under one
determiner: they take a singular verb only when the whole denotes a single
entity (*The hammer and sickle was*; *Our chef and chauffeur has*). With two
referents they take a plural verb (*Our chef and chauffeur have decided*).
CGEL therefore licenses the singular for a single-referent coordination;
retiring it for the possessive coordinated property Subject rests on its
corpus attestations. Distributive shared Determiners are retained under the
owner ruling below. If the full census shows a face that
needs the singular, STOP (pruning rule).

Existing route: `NominalConcord` (`declarations.rs:1042`), used by the
`NominalSeries` instances (`:2067,2108,2145`), requires `agree left.number =
Right.number` and exports `number = left.number`. Two singular Nominals
therefore yield a singular Nominal. NP coordination (`NounPhraseSeries` with
`NounCoordinationAgreement`) resolves number through `coordinate_number`
(`:487`). The fix gives coordinated Nominals the same resolution for clause
agreement through declared features. Keep each coordinate's own number for
selection by the Determiner. Nominal coordinations under *each* are attested
(*each creature and planeswalker they control*, Goblin Chainwhirler, 8
occurrences). They keep their Readings, so a singular-selecting
Determiner must still accept the coordinated Nominal.

Ruling (owner, 2026-10-08), resolving the census STOP: a distributive
singular-selecting Determiner (*each*, *every*, *a/an*) fixes the NP's
Agreement Number as singular over a coordinated Nominal. Determiners that
do not impose that singular Agreement Number, including possessives and
*the*/*that*, pass the Nominal's resolved Number through. Implement through
declared features, with no word guards. This supersedes the ticket's blanket
singular-retirement claim. CGEL, Ch. 5 §18.4(a), p. 508 [28] describes singular
agreement for coordinated NPs containing *each* or *every*; the shared
Determiner implementation and the broader selection rule are the owner's
ruling, not a claim attributed to that passage.

## Witnesses

Real faces whose failing unit has this clause (`xnkuznpo`, 0 Readings):

- Hulking Metamorph: "You may have this creature enter as a copy of an
  artifact or creature you control, except it's an artifact creature in
  addition to its other types, and its power and toughness are equal to this
  creature's power and toughness." The face also needs
  `english-v3-enter-as-copy-complement`, so test the attested clause "Its power
  and toughness are equal to this creature's power and toughness." (0 → ≥1)
  and its singular counterpart (1 → 0).
- Lord of Extinction: "Lord of Extinction's power and toughness are each
  equal to the number of cards in all graveyards." It reads once floated
  *each* lands.
- Gaea's Liege: "As long as Gaea's Liege isn't attacking, its power and
  toughness are each equal to the number of Forests you control." It reads
  once floated *each* lands.
- Gilt-Leaf Winnower: "When this creature enters, you may destroy target
  non-Elf creature whose power and toughness aren't equal." It stays unread
  for other causes: "… whose power isn't equal to 2." also reads 0.

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, scope).
Deltas:

1. Verification budget (binding): One full census (`--all --workers 12
   --samples-per-face 0`) on the finished feature for the record, compared
   face by face (per-face `readings_sha256` and counts) against the claim
   parent's census; iterate on selector subsets, never on --all. Tests:
   `cargo test -p` and `cargo clippy -p … --tests -- -D warnings` for every
   crate whose code or consumed data changed (`deckmaste_english_v3` always;
   `deckmaste_lexical_source` if core.ron/verbs.ron/vocabulary.rs changed).
   No xtask tests, no Lean checks, no post-refresh re-census unless refresh
   touched grammar, lexicon or core. New tests go under tests/suite/
   registered in tests/main.rs. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write the Hulking Metamorph clause and its singular counterpart as tests
   first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. A
   decrease caused by retiring the singular coordinated-Nominal Reading is a
   correction: name each face and its before/after count in the landing record.
   Any other decrease on a previously covered face is restored, or named and
   justified; a decrease inherited from trunk is attributed by change id.
5. Spot-check at least 10 faces whose Readings changed, plus any newly covered
   faces, and list them by name with their Reading. A coordinated Nominal
   Subject agreeing as singular, *or*-coordination turned plural, or a
   singular-selecting Determiner newly rejecting a coordinated Nominal is a
   defect. A wrong analysis that parses is a defect, not a gain.
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

## Out of scope

- Floated *each* (*are each equal to*, *each get*):
  `english-v3-floated-each-quantifier`.
- Verbless *with power and toughness each equal to …* clauses.
- Clause coordination that happens to put two nouns side by side (*draw a
  card and you lose 1 life*): already a separate route.

## Landing record

1. **Census delta** (`zpzympuw` → `szlklvns`): bounded selector,
   case-insensitive `power and toughness| and [\s\S]{0,40}? (?:are|aren't|is|isn't|has|have) `,
   reaches 1,018 supported faces; official diff manifest adds 300 random covered
   faces (seed 20261008), with four overlaps: 1,314 distinct faces. Covered
   693 → 696; Readings 84,643 → 84,773. Lost faces: none; decreases: none;
   increases on previously covered faces: none. Gains: Galion, Elvenking's
   Butler 0 → 24, Halfdane 0 → 76, Shape Stealer 0 → 30. **Zero corpus faces
   lost a singular Reading.** Wild Pair is digest-only (1,026 → 1,026): a fresh
   parent-runtime census on `mqrlzszkuszn` reproduces the baseline discrepancy
   and matches the feature digest; no feature-induced change. Official
   `cargo xtask english-v3 diff` evidence is under
   `target/english-v3/english-v3-coordinated-subject-agreement/`; all selected
   faces enumerate completely with roundtrip/traversal validation and no issues
   or internal failures.
2. **Witnesses** (`szlklvns`): Hulking Metamorph's isolated “Its power and
   toughness are equal to this creature's power and toughness.”: 1; singular
   *is*: 0; single-head control: 1. Lord of Extinction's agreement clause without
   floated *each*: 4; singular counterpart: 0. Complete Hulking Metamorph,
   Lord of Extinction, Gaea's Liege and Gilt-Leaf Winnower faces: 0, retaining
   their ticket-listed blockers (enter-as-copy, floated *each*, and unrelated
   relative predicate support). Shared *each creature and planeswalker they
   control*: 2. Brightcap Badger and Great Divide Guide complete faces: 2 each;
   altered *have* counterparts: 0. Alternative “Target creature or Vehicle
   gets +2/+0 until end of turn.”: 2; *get*: 0. Each witness was probed before
   implementation; independent construction preserves realization, parse
   membership, and complete node/word traversal equality.
3. **Spot checks** (`szlklvns`): Galion, Elvenking's Butler (24), Halfdane (76),
   and Shape Stealer (30): shared possessive/genitive Determiner over coordinated
   *base power and toughness*, plural Subject with *become*. Biomass Mutation
   (4) and Mirror Entity (6): coordinated measured properties in *have* Object;
   Ovinize (2), Diminish (4), Humble (2): the same properties in *has* Object;
   Turn to Frog (12) and Polymorphist's Jest (12): coordinated properties inside
   a *with* modifier. Only three newly covered faces exist; all three were
   inspected, with seven preserved controls completing ten checks. Also checked
   all four faces affected by the initial distributive regression and Wild Pair.
4. **Deviations and STOPs**: the initial blanket plural rule lost Brightcap
   Badger and Great Divide Guide and decreased Lorehold, the Historian and
   Silverquill, the Disputant. STOP resolved by the owner's 2026-10-08 ruling:
   declared singular-selecting Determiners retain singular agreement over
   coordinated Nominals; others export resolved plural. CGEL Ch. 5 §18.4(a),
   p. 508 [28], supports coordinated NPs containing *each/every*, while the
   shared-Determiner and *a/an* extension is explicitly the owner's ruling.
   Added Nominal Agreement Number and Determiner Agreement Number glossary
   entries; propagated the feature through modifiers, relatives and measured
   attributes without adding constructions or frames. Indefinite articles use
   their existing declared singular Number. Corpus attestation check
   (`zpzympuw`, 32,828 supported faces) found no “each creature and planeswalker
   they control have”; negative probe and test return 0 on `szlklvns`. No word
   guards or superseded lexical routes. Adopted default's card-context
   re-spelling of the Circu comparative test with all value assertions retained.
   The orchestrator's bounded-run override replaces Method's full-census rule;
   witnesses used probes before implementation and tests were written once.
5. **Tests** (`szlklvns`): `cargo test -p deckmaste_english_v3`
   ran once (527 passed, 3 failures subsequently fixed). Focused reruns
   `cargo test -p deckmaste_english_v3 --test main articles::`,
   `cargo test -p deckmaste_english_v3 --test main same_as_comparative::`, and
   `cargo test -p deckmaste_english_v3 --test main coordinated_subject_agreement::`
   pass (5, 3, 5 respectively). `cargo test -p deckmaste_lexical_source` passes
   (59). `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` and
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` pass;
   formatting check passes. Assurance: restored 2 article tests
   by removing the unnecessary feature requirement; re-spelled 1 Circu scoped
   name test; added 5; ignored 0; removed 0.
6. **Timing** (`szlklvns`): 20.564 seconds for the final bounded census,
   12 workers. No full census was run, per the binding override.
