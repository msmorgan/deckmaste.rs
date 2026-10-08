---
needs: []
---
# Read an adverb Adjunct in central position: It's still a land.

## Why

*still* and *also* never read. Both are declared Adverbs, but the grammar
admits adverb Adjuncts only in front and end position. On census `xnkuznpo`
(docs-only descendant of `vvxtllnq`; 32,828 supported faces, 23,459 covered,
9,369 unread; wave-4 recon of 2026-10-08), the recon bucket "medial adverb
Adjunct (*still* / *also*)" touches **209** unread faces and is the sole cause
on **168**. Neither word occurs in any text the grammar reads. Counts are
unread faces *touched* / *sole* as defined in the recon; surface counts, not
gain forecasts.

Probes (admitted roots): "It's still a land." 0, while "It's a land." 1;
"It is still a land." 0; "Creatures you control also gain protection from the
chosen color until end of turn." 0, while the string without *also* reads 1.

## Goal

An adverb whose declared position licence allows it reads in central
position: after a finite auxiliary or copula (*it's still a land*, *that's
still a land*) and before a lexical verb (*you also gain*, *it also gets*),
realised byte-exactly. Each adverb's admissible positions are a declared
lexical property, read the same way `VPFinalAdjunct` and
`ClauseInitialAdjunct` are read today.

## Analysis

CGEL, Ch. 8 §20.1, p. 779 distinguishes front, central and end position for
adjuncts; §20.2, p. 782 records the preference for post-auxiliary over
pre-verbal position (*He is already in hospital*). *still* is described in
Ch. 8 §8, p. 710 (polarity-sensitive aspectual adjuncts).

Existing route: `vocab:Adverb/Still` and `vocab:Adverb/Also`
(`core.ron:890,897`) declare `VPFinalAdjunct: No` and
`ClauseInitialAdjunct: No`, so neither existing consumer admits them:
`AdverbPredicate` (`declarations.rs`, near line 1386) is end position only and
`InitialAdverb` (`:3911`) is front position only. This ticket adds the central
position, with a declared licence feature in the same family. It needs one
consumer for the post-auxiliary/copula site and one for the pre-verbal site,
or one schema with two forms. State which in the landing record and count it
against the construction ceiling.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xnkuznpo`, 0 Readings each):

- Vastwood Animist: "It's still a land." (control "It's a land." 1)
- Thelonite Druid: "They're still lands." (control "They're lands." 1)
- Verdant Touch: "Target land becomes a 2/2 creature that's still a land."
  (control without *still* 1)
- Aven Warcraft: "Creatures you control also gain protection from the chosen
  color until end of turn." (control without *also* 1)
- Might of the Meek: "It also gets +1/+0 until end of turn if you control a
  Mouse." (control without *also* 1)

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
   Reading. An adverb attached as a modifier of the following NP or Predicative Complement (rather than as a clause Adjunct), or a central-position Reading for an adverb the corpus does not attest there, is a defect. A wrong analysis that parses is a defect, not a gain.
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

- *no longer* (*longer* is a vocabulary gap).
- Licensing other adverbs in central position unless the corpus attests them
  on a face this ticket covers (pruning rule).
- Focusing *only* in central position (already handled by its own route).

## Landing record

The six items in CLAUDE.md. Also list each Adverb given the central-position
licence with its corpus face count.
