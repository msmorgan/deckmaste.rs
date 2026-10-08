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
agreement with the finite verb, realised byte-exactly. The singular Reading
of the same string is retired. Alternative (*or*) coordination keeps its
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
retiring it here rests on the corpus, which attests only plural agreement
with a coordinated Nominal Subject. If the full census shows a face that
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

The six items in CLAUDE.md. Also give the number of faces whose Reading count
fell because the singular Reading was retired.
