---
needs: []
---
# Read Enter's selected *as a copy of* Complement: enter as a copy of a creature you control

## Why

No *enter as a copy of* sentence reads. On census `xnkuznpo` (docs-only
descendant of `vvxtllnq`; 32,828 supported faces, 23,459 covered, 9,369
unread; wave-4 recon of 2026-10-08), the recon bucket "enter as a copy of"
touches **63** unread faces and is the sole cause on **51**. Of the 94 failing
", except it …" units, 40 sit on this host. It is also one of the two
prerequisites of `english-v3-clause-final-predicative-as` (orchestrator ruling
2026-10-07). Counts are unread faces *touched* / *sole* as defined in the
recon; surface counts, not gain forecasts.

Probes (admitted roots): "You may have this creature enter as a copy of a
creature you control." 0, while "You may have this creature enter tapped." 1;
"Creatures you control enter as a copy of this creature." 0, while
"Creatures you control enter tapped." 1.

## Goal

*enter* selects an *as* + NP predicative Complement (*as a copy of …*), also
after a depictive (*enter tapped as a copy of …*), realised byte-exactly. The
*as* phrase is a Complement of *enter*, never a trailing Adjunct, per the
predicative-*as* orchestrator ruling of 2026-10-06 recorded in
`english-v3-systemic-residuals` ("Enter selected predicative-as Complement").

## Analysis

CGEL, Ch. 4 §5.1, p. 255, [9]: predicative obliques are commonly marked by
*as* (*That counts as excellent*, *She served as treasurer*). That backs an
*as*-marked predicative Complement. Its selection by *enter* is the 2026-10-06
ruling.

Existing route: Enter's frames (`verbs.ron:63–69`) have no predicative
Complement. `vocab:Preposition/As` declares `PrepositionComplement:
PredicativeNounPhrase` (`core.ron:181`), and
`PredicativeComplementPreposition` (`declarations.rs:2689`) builds the PP.
This ticket adds the Enter frame and its consumer, reusing that PP.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Essence of the Wild: "Creatures you control enter as a copy of this
  creature." (control "Creatures you control enter tapped." 1)
- Mirror Image: "You may have this creature enter as a copy of a creature you
  control." (control "You may have this creature enter tapped." 1)
- Vesuva: "You may have this land enter tapped as a copy of any land on the
  battlefield."
- Body Double: "You may have this creature enter as a copy of any creature card
  in a graveyard."
- Masterwork of Ingenuity: "You may have this Equipment enter as a copy of any
  Equipment on the battlefield."

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
2. Write each witness as a test first.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record; a decrease inherited from trunk is attributed by change
   id.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. An *as a copy of* phrase read as an Adjunct, or as a postmodifier of the Subject, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- The *, except it …* content clause after the copy: count newly covered
  faces such as Quicksilver Gargantuan if they read, but do not change the
  except route here.
- *becomes a copy of*, *create a token that's a copy of* hosts.
- Plural agreement in Hulking Metamorph's "… and its power and toughness are
  equal to this creature's power and toughness"
  (`english-v3-coordinated-subject-agreement`).
- Clause-final predicative *as*: `english-v3-clause-final-predicative-as`.

## Landing record

The six items in CLAUDE.md. Also give the count of newly covered faces that
carry ", except it …" after *enter as a copy of*.
