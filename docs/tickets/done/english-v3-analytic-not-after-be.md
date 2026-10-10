---
needs: []
---
# Read analytic *not* after a cliticised *be*: If it's not your turn, you may exile a blue card from your hand rather than pay this spell's mana cost.

## Why

Negation reads only as a negative verb form (*isn't*, *aren't*, *doesn't*).
The word *not* is declared (`vocab:PredicateNegator/Not`,
`lexicon/vocabulary.rs`) and no Construction uses it: `cargo xtask english-v3
grep -i '\bnot\b'` finds **111** faces and **0** are covered. Blocker report
at trunk `uoqyuxtnowpo`: failing token *not* on 79 sole-blocker faces, key
`at:it's not` 23.

This ticket takes the clitic class. `grep -i "\b(it|that|they|you|he|she|
there)'(s|re) not\b"` finds **38** faces, all unread; 39 failing lines match,
**33** of them the only failing line of their face: *it's not your / their /
that player's turn* 15, *it's not legendary* 8, *it's not a …* 7, *that's
not* 3, *they're not* 2, *you're not* 2, *it's not attacking / suspected* 2.

The other classes at *not* (sole-blocker lines), for the record:

| Class | Faces | Sole lines | Where |
|---|---:|---:|---|
| *choose not to* VP | 45 | 34 | another wave-8 ticket |
| clitic *be* + *not* | 38 | 33 | this ticket |
| noun + *not* + participle / PP (*cards not cast this way*, *permanent not named X*, *creature not chosen this way*, *card not on the battlefield*) | 23 | 19 | Out of scope |
| *has not been* | 2 | 2 | NB5 |
| *but not more …* / *but not from …* | 3 | 1 | Out of scope |
| *pack not being* | 1 | 0 | Out of scope |

## Goal

A finite clitic form of *be* (*'s*, *'re*) followed by *not* and then the
ordinary Complement of *be* reads as a negative clause: predicative NP (*not
your turn*, *not a creature*, *not the monarch*), AdjP (*not legendary*,
*not historic*), participle (*not attacking*, *not suspected*), PP (*not in
your commander's color identity*).

## Analysis

CGEL Ch. 9 §1, p. 788: "Analytic negation is marked by words whose sole
syntactic function is to mark negation, i.e. *not* …"; "Synthetic verbal
negation is marked inflectionally, by negative verb-forms". Ch. 9 §2, p. 800: "Analytic
and synthetic primary negation forms are not fully interchangeable. The main
difference is one of style level." Those back two forms of the same verbal
negation. That *not* attaches to the auxiliary rather than to the Complement
is CGEL's verbal-negation analysis as a whole (Ch. 9 §2), not a quoted
passage: treat NB3 as a project ruling.

Existing routes: `core-verb:BeNegative` (lemma *isn't*, `Polarity =
Negative`, `Contraction = Yes`, `lexicon/core.ron`) is the synthetic form;
the clitic *be* forms read through the `Clitic` / `CliticHost` features and
the `clitic_host` table (`crates/deckmaste_english_v3/src/declarations.rs`).
The synthetic and clitic routes cannot combine (*it'sn't*), which is why Oracle
writes *it's not*. Uncontracted *is not* / *does not* / *can not* are
unattested (`\b(is|are|was|were|do|does|did|can|may|must) not\b` 0 faces).

Orchestrator rulings 2026-10-10:

- NB1. One route: *not* as the negation marker of a finite auxiliary head,
  placed after it and before its Complement. Refine the existing *be*
  predicate Constructions (a negated form or a schema over them); do not
  mint a negative clitic lexeme (*'s not* as one word) and do not copy the
  `BeNegative` frames.
- NB2. The licence reads declared features: the `PredicateNegator` Category,
  and the head's `Polarity` (not already Negative) and auxiliary status. No
  test on *be*, *it*, *turn* or a card. The resulting predicate exports
  `Polarity = Negative`, as the synthetic form does, so anything downstream
  that reads polarity sees one value for both forms.
- NB3. One analysis per string. The twins to avoid: (a) *it's not your turn*
  has exactly as many Readings as *it isn't your turn* in the same context
  (clause negation only): no second Reading with *not* inside the Complement
  (*[not your turn]*, *[not legendary]*); (b) *isn't* / *aren't* strings keep
  exactly their Readings; (c) *it's* + possessive or perfect uses of *'s* are
  untouched.
- NB4. Scope by licence, not by host word. If the declared-feature licence
  also admits uncontracted *is not* (unattested), that is acceptable: the
  pruning rule forbids a check whose only job is to block it, and equally
  forbids extra machinery to reach it. Report what the licence admits.
- NB5. *has not been* (2 faces) is in scope only if NB1's route reads it with
  no further declaration; otherwise list it.
- NB6. Newly covered faces are gains. No Reading-count increase on a
  previously covered face is expected: no covered face contains *not*. Any
  increase is reported with its shape before landing.

## Witnesses

Each is the face's only failing line. Probed 2026-10-10 on the `uoqyuxtnowpo`
binary (`--readings 3 --as-card`): witness 0; control is the same line with
the negative verb form.

- Force of Negation: "If it's not your turn, you may exile a blue card from
  your hand rather than pay this spell's mana cost." 0 (*it isn't* 3)
- Eddymurk Crab: "This creature enters tapped if it's not your turn." 0 (1)
- Glademuse: "Whenever a player casts a spell, if it's not their turn, that
  player draws a card." 0 (1)
- Arcades Sabboth: "Each untapped creature you control gets +0/+2 as long as
  it's not attacking." 0 (2)
- Desynchronization: "Return each nonland permanent that's not historic to
  its owner's hand." 0 (*that isn't* 2)

Sentence probes, 0 today: "It's not your turn."; "It's not a creature.";
"It's not legendary."; "They're not legendary."; "You're not the monarch."
Controls that must not change: "It isn't your turn." 1; "It's your turn." 1;
"It isn't a creature." 1; "They aren't legendary." 1; "You aren't the
monarch." 1; "It doesn't untap." 1.

## Method

Standard constraints apply. Bounded census, never `--all`: every supported
face matching `\bnot\b|\b(it|that|they|you|there)'(s|re)\b` (1,161 faces)
plus 300 random covered controls, `--workers 6 --samples-per-face 0`,
compared with `cargo xtask english-v3 diff` against the claim parent. Zero
lost; NB6 for increases. Full `cargo test -p` and `cargo clippy -p … --tests
-- -D warnings` for `deckmaste_english_v3` (and `deckmaste_lexical_source` if
a feature is declared there), once, at the end: a rebuild after a grammar
edit takes about 3 minutes and the test binary about 7, so batch edits. Tests
assert the role (*not* is the negation marker of the *be* head; the predicate
is Negative; the Complement is the ordinary Complement; the NB3 pairs have
equal counts), never an incidental count or a Debug-printed tree. Attestation
by `cargo xtask english-v3 grep` only.

## Out of scope

- *choose not to* VP (45 faces): its own wave-8 ticket; negation of a
  *to*-infinitival, not of a finite auxiliary.
- Postnominal *not* + participle / PP (23 faces, 19 sole lines): negation
  inside a reduced postmodifier. "Destroy each creature not chosen this way."
  0 against "… chosen this way." 2. Own ticket.
- *but not more than …* (Drain Life, Soul Burn), *pack not being*.
- Faces whose except clause has its own blocker (*a copy of …, except it's
  not legendary and …*): count only when the whole face reads.

## Landing record

1. **Census delta** (`spzllmsloltx`, completed grammar against claim-parent
   `xxytwsys` declarations and lexicon): 1,456 compared faces; covered
   1,190 → 1,220; total Readings 86,235 → 92,182. After refresh
   (`kuqmnwsprmor`), covered 1,190 → 1,222; Readings 86,235 → 92,231.
   Lost: none. Gained 32: 30 from this change and two incoming named-mode
   gains from `pxtwrtvu` (Sauron, Dino Devotee, 37 Readings; What Must Be
   Done, 12 Readings);
   decreases 0; increases on previously covered faces 0; digest-only 34,
   all unchanged-count Secondary Negation Readings whose marker now uses
   the shared Negator schema. No wrong-analysis gains found; 0 census issues.
   Selector: case-insensitive `\bnot\b|\b(it|that|they|you|there)'(s|re)\b`
   (1,161 faces), plus 300 random covered controls, seed 20261008 (5 overlap).
   Official `english-v3 diff --lines` evidence is
   `target/english-v3/english-v3-analytic-not-after-be/{before,after,diff,manifest}.json`.
   The copied executable stamped `mqrlzszkuszn` and was stale; the parent was
   rebuilt and remeasured, rather than attributing its sibling gains here.
   Refresh comparisons: `{refresh-after,refresh-diff,final-diff}.json` in
   the same evidence directory; refresh added no losses, decreases, increases
   or digest-only changes. Final refresh (`zkvnqmuussok`) added six gains
   from incoming Source-attachment code `nxxknqvk`: Bottomless Vault,
   Dwarven Hold, Hollow Trees, Icatian Store and Sand Silos (11 each),
   Lost in the Woods (2). Final claim-parent comparison: covered
   1,190 → 1,228; Readings 86,235 → 92,288; gained 38 (30 own, eight
   trunk), lost/decreases/increases 0, digest-only 34. Evidence:
   `second-refresh-{after,diff}.json` and `final-diff.json`.
   Accepted-review refresh (`nxwppuwuqkpq`) brought keyword-state code
   `ptvtlvss` / `qkvxplko` / `tqsuwzum`: three further trunk gains,
   Court of Locthwain (721), Opposition Agent (8), Planeswalker's Mischief
   (10). Eight trunk increases have the KS6/KS-R1 finite-*be* + *exiled*
   Adjective Complement shape alongside the retained verbal passive:
   Arcane Artisan 17 → 23; Azor's Gateway 36 → 54; Dinosaurs on a Spaceship
   18 → 30; Market Gnome 12 → 15; Riftmarked Knight 14 → 26; Uvilda, Dean
   of Perfection 68 → 272 (both quoted *be exiled* clauses); Veiling Oddity
   11 → 21; Watcher of Hours 3 → 4. Own gains remain 30; own increases 0.
   Final totals: covered 1,190 → 1,231; Readings 86,235 → 93,293;
   gains 41 (30 own, 11 trunk), lost/decreases 0, increases 8 (all trunk),
   digest-only 34. `accepted-refresh-{after,diff}.json`, `final-diff.json`.
   Play-with refresh (`mwvnkpvkrnzw`) added only Enduring Renewal (7),
   from trunk `wwzpptst`'s selected play-with Construction. Refresh-only
   lost/decreases/increases/digest-only 0. Latest claim-parent totals:
   covered 1,190 → 1,232; Readings 86,235 → 93,300; gains 42 (30 own,
   12 trunk), lost/decreases 0, increases 8 (keyword-state trunk),
   digest-only 34. `play-refresh-{after,diff}.json`, `final-diff.json`.
   Named-modes regression refresh (`ypqnvzptrwop`) preserves all 1,456
   face counts: gained/lost/increases/decreases 0, 1,207 digest-only changes
   from trunk `lqmpuwls`'s Paragraph/continuation schema instances and
   ability/mode declared-feature interfaces. Totals remain 1,232 covered,
   93,300 Readings; own gains 30. Latest claim-parent diff: 42 gains,
   eight trunk increases, 0 lost/decreases, 1,157 digest-only changes.
   Evidence: `modes-refresh-{after,diff}.json`, `final-diff.json`.
2. **Witnesses** (`spzllmsloltx`, complete Document probes): Force of Negation
   0 → 3, Eddymurk Crab 0 → 1, Glademuse 0 → 1, Arcades Sabboth 0 → 2,
   Desynchronization 0 → 2. Their synthetic controls remain 3/1/1/2/2.
   The five sentence witnesses each change 0 → 1; all six sentence controls
   remain at 1. Counts were re-probed on the rebuilt claim-parent grammar.
   Both refreshes (`kuqmnwsprmor`, `zkvnqmuussok`) retain per-line
   witness counts 3/1/1/2/2.
3. **Spot checks** (`spzllmsloltx`): ten gained faces, with byte-exact
   probe Readings and Primary Negation on the auxiliary, preceding its
   ordinary Complement: Force of Negation, Eddymurk Crab and Glademuse
   (NP, *your/their turn*); Arcades Sabboth (participial, *attacking*);
   Desynchronization (AdjP, *historic*); Court of Vantress (NP, *the monarch*);
   Otherworldly Escort (NP, *a Spirit*); The Peregrine Dynamo (NP,
   *a commander*); Ember Island Production (AdjP, *legendary*, within its
   except clauses); Captain Marvel, Apex Avenger (NP, *a Kree*).
4. **Deviations and STOPs** (`spzllmsloltx`): no STOPs. Added
   NegatedAuxiliaryHead and its selected-predicate and participial-predicate
   forms over the existing frames; `selected_frame` requires a completed
   head immediately before the frame, and *attacking* uses the auxiliary
   participial frame. Refined the existing SecondaryNegator into a spacing
   wrapper over one shared atomic Negator schema with separate declared
   Primary/Secondary licences; no duplicate negator lexeme or copied negative
   frames. Propagated finite-head Polarity through existing interfaces,
   including coordinate anchors, whose members retain their own polarity.
   `FiniteConcord` and `FiniteGapConcord` export the LEFT member's Polarity:
   a mixed coordination (*is X and isn't Y*) carries the left value. Nothing
   reads that exported value today.
   Added the missing Negative property to HaveNegative for that export.
   Glossary gaps filled: Primary Negation, Primary Negator and shared Negator,
   Primary Negation's term cites page-checked CGEL, Ch. 9 §2.1, p. 799;
   its attachment site and the shared Negator representation cite NB/NI
   project rulings. NB4: uncontracted *is not* reads through the
   same licence. NB5: Cyclopean Tomb and Illusionary Mask (*has not been*)
   remain unread; no additional auxiliary declaration was made to reach them.
   Final refresh conflicted in category interfaces. Resolution preserved both
   Polarity and incoming SourceComplementSelection; NegatedAuxiliaryHead now
   exports the latter from its lexical head to satisfy the incoming shared
   agreement policy. No incoming frame, licence or test was retired.
5. **Tests** (`spzllmsloltx`): `cargo test -p deckmaste_english_v3` passed
   all 852 tests; `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings`
   passed. `cargo test -p deckmaste_lexical_source` passed all 83 tests;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed.
   Post-refresh (`kuqmnwsprmor`): `cargo build -p xtask` passed;
   `cargo test -p deckmaste_english_v3` passed all 863 tests;
   `cargo clippy -p deckmaste_english_v3 --tests -- -D warnings` passed;
   `cargo test -p deckmaste_lexical_source` passed all 83 tests;
   `cargo clippy -p deckmaste_lexical_source --tests -- -D warnings` passed;
   `cargo test -p deckmaste_construction_core` passed all 473 tests;
   `cargo clippy -p deckmaste_construction_core --tests -- -D warnings` passed;
   `cargo test -p deckmaste_lexical` passed all 60 tests;
   `cargo clippy -p deckmaste_lexical --tests -- -D warnings` passed;
   `cargo test -p xtask english_v3` passed all 56 selected tests.
   Final refresh (`zkvnqmuussok`): `cargo build -p xtask` passed;
   the same four full crate test and clippy commands passed again:
   English 876, lexical-source 83, construction-core 473, lexical 60.
   `cargo test -p xtask english_v3` passed all 56 selected tests again.
   Accepted-review refresh (`nxwppuwuqkpq`): `cargo build -p xtask` and
   the same four full crate test and clippy commands passed again:
   English 881, lexical-source 83, construction-core 473, lexical 60.
   `cargo test -p xtask english_v3` passed all 56 selected tests again.
   Play-with refresh (`mwvnkpvkrnzw`): `cargo build -p xtask` and
   the same four full crate test and clippy commands passed again:
   English 891, lexical-source 83, construction-core 473, lexical 60.
   `cargo test -p xtask english_v3` passed all 56 selected tests again.
   Named-modes refresh (`ypqnvzptrwop`): `cargo build -p xtask` and
   the same four full crate test and clippy commands passed again:
   English 893, lexical-source 83, construction-core 473, lexical 60.
   `cargo test -p xtask english_v3` passed all 56 selected tests again.
   Restored 0 / re-spelled 2 / ignored 0 / added 4 / removed 0. The two
   re-spelled infinitival tests retain their original cards, roles and
   independently constructed roundtrip outcomes. The initial English run
   exposed double spacing in the new route; the declaration was fixed and
   the full suite rerun. An initial clippy compile found a shadowed helper
   binding in the re-spelling; it was renamed. Citation checks: 0
   noncompliant, 0 stale; piped diff audit found no new citation sites.
6. **Timing** (`ypqnvzptrwop`): final bounded census 55.80 s wall,
   6 workers, prebuilt executable, `--samples-per-face 0`, under
   `/usr/bin/time -v`. No `--all` or Lean check was run.
