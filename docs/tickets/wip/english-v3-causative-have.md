---
needs: []
---
# Read causative *have* with an Object and bare infinitival: you may have target player mill two cards

## Why

Causative *have* never reads. On change `xxknlzypsnwy` (32,828 supported faces,
17,322 covered, 15,506 unread; recon of 2026-10-07), *(may) have* + NP + bare
infinitival touches **253** unread faces and is the sole cause on **101**
(recon bucket "causative have": 307 / 114, which also counts non-causative
*have all activated abilities*). Counts are unread faces *touched* (at least
one localised failing unit matches) / *sole* (every failing unit matches and no
other recon STRONG bucket does). They are surface counts, not gain forecasts.

Probes (admitted roots): "You may have target player mill two cards." 0; "Have
target player mill two cards." 0. *Have* declares `Predicate([Role("Object"),
Role("VerbPhrase")])` in `verbs.ron`; it is the `core-verb:Have` row
`Complement(Object), Complement(VerbPhrase)` of the unsupported-inventory table
in the done `english-v3-generic-frame-consumption` ("unsupported slot category
Object (Complement)"). This ticket takes that row over from
`english-v3-systemic-residuals`.

## Goal

*have NP VP* reads with *have* as a causative catenative: the NP is the Object
and the bare infinitival clause is the catenative Complement, with the Object
understood as its Subject (*have target player [mill two cards]*). It reads
under *may*, in imperatives and in finite clauses. The legacy `Role("Object"),
Role("VerbPhrase")` frame is replaced by typed slots the generic consumer
admits, and retired (Method 9).

## Analysis

Only a small number of catenatives take bare infinitivals; among the causatives
they are *have*, *let* and *make* (CGEL, Ch. 14, §5.6.2, p. 1244). CGEL, Ch. 14, §5.4, p. 1236, treats *have* in the complex construction with a
raised Object. Applying that account to *have target player mill …*, the
intervening NP is the Object of *have* and is understood as Subject of the
bare infinitival Complement; it is not an overt Subject of a finite clause.

## Witnesses

Each witness's failing units all match this construction and no other recon
STRONG bucket (`xxknlzypsnwy`):

- Quill-Slinger Boggart: "Whenever a player casts a Kithkin spell, you may have
  target player lose 1 life."
- Rage Forger: "Whenever a creature you control with a +1/+1 counter on it
  attacks, you may have that creature deal 1 damage to target player or
  planeswalker."
- Joraga Bard: "Whenever this creature or another Ally you control enters, you
  may have Ally creatures you control gain vigilance until end of turn."
- Mirror Image: "You may have this creature enter as a copy of a creature you
  control."
- Ebon Dragon: "When this creature enters, you may have target opponent discard
  a card."

## Method

Standard constraints apply (CLAUDE.md landing record, assurance, gate scope,
cite check). Deltas:

1. Before: `cargo xtask english-v3 --all --workers 12 --samples-per-face 0
   --output target/english-v3/english-v3-causative-have-before.json` on the
   claim parent, stamped with its change id and covered count; after: the same
   command to `target/english-v3/english-v3-causative-have-after.json` on the
   final tree. Evidence lives under the workspace's ignored
   `target/english-v3/`, never `/tmp`.
2. Write each witness as a test first. Iterate on `--face-id` selectors; verify
   on `--all` at the end.
3. Zero lost faces. A lost face is a defect: fix it within the ticket first;
   STOP only if the fix fails or needs a ruling.
4. Compare per-face Reading counts against the claim-parent census. Any
   decrease on a previously covered face is restored, or named and justified in
   the landing record.
5. Spot-check at least 10 newly covered faces and list them by name with their
   Reading. The bare infinitival read as a reduced relative on the Object, or
   *have* read as possessive with an Adjunct, is a defect. A wrong analysis
   that parses is a defect, not a gain.
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
11. Timings as integer ns and ns/B, with host load and worker count.
12. Gate: `cargo xtask gate --changed --from <claim> --run`.

## Out of scope

- Possessive and granted-ability *have* (*has flying*, *have all activated
  abilities of …*): existing frames, untouched.
- Perfect *have* with an Object Gap (*spells you've cast*): owned by
  `english-v3-systemic-residuals`.
- *enter as a copy of …* inside the infinitival: reuse the existing Enter
  frames; do not build *as a copy* here.

## Scope STOP and implementation progress (2026-10-07)

Unfinished; not integrated. The five requested witness tests were red before
implementation. The typed Have frame replaces the unsupported legacy row:

```ron
Predicate([
    Argument((relation: Object, category: "NounPhrase")),
    Argument((relation: Complement, category: "BarePredicate")),
])
```

This uses the existing generic `SelectedPredicate` consumer and `BarePredicate`
Construction. The unsupported old frame had no causative-specific v3 grammar
Production to retire. Both obsolete slots are removed from Have's lexicon;
no grammar declaration or parallel label is added. Its existing granted-ability,
predicative, possessive and perfect frames retain their positions and contents.
Other verbs' legacy slots stay with their owners.

**STOP:** Mirror Image's full witness remains unread. On the claim-parent
production tree, the isolated `enter as a copy of a creature you control`
at `SecondaryVerbPhrase` already has zero admitted roots. The ticket's
instruction to reuse existing Enter frames therefore has an unmet prerequisite.
The [fixed-cost-phrases orchestrator resolution of 2026-10-06](../done/english-v3-fixed-cost-phrases.md)
licenses predicative-*as* Adjuncts only in preposed, clause-initial,
comma-separated position. `PredicativeComplementPreposition` projects only
that licence; there is no existing Enter frame consuming the trailing
predicative-*as* phrase. Widening the Adjunct licence would contradict the
recorded ruling, and building the missing Enter analysis reaches this ticket's
explicit out-of-scope item. That ruling is project authority, not a CGEL claim.
The full Mirror Image test is retained and failing, with no ignore or weakened
assertion. A scope question is pending: defer this full witness or authorize
the missing Enter analysis without changing the recorded Adjunct restriction.

Eight other tests pass: the four other requested witnesses, real finite *has*
(Lava Blister) and *had* (Wandering Troubadour) witnesses, an independently
constructed Ebon Dragon constituent with both roundtrip laws and exact node/
Word traversal, and a Complement-form diagnostic. Joraga Bard preserves both
matrix and embedded duration attachments. Participial diagnostics can already
read through the existing possessive/perfect routes; their exact Reading sets
are compared against the legacy-frame baseline, and the new frame adds none.
No existing test was changed. Added: 9; restored: 0; re-spelled: 0; ignored: 0;
removed: 0. The initial diagnostic assumptions about participial strings were
corrected against those unchanged Readings, rather than narrowing admission.

### Candidate evidence (not a final landing measurement)

All figures below are stamped with change `ostqqnplnqrm` and the measured
covered count. The before census measured the claim-parent production sources
with only the five new tests present; the candidate measured the replacement
frame. The lexical inventory hashes in the reports distinguish these snapshots
of the same mutable change. Both completely enumerate 32,828 identical
supported identities and parser inputs, with zero issues, failed or limited
enumerations, and undetermined faces.

| Snapshot / covered | No Reading | One Reading | Multiple Readings | Exact Readings |
|---|---:|---:|---:|---:|
| Before `ostqqnplnqrm` / 19,682 | 13,146 | 7,294 | 12,388 | 560,894 |
| Candidate `ostqqnplnqrm` / 19,826 | 13,002 | 7,331 | 12,495 | 584,148 |

Candidate gains: 144; lost faces: 0; decreased Reading counts on previously
covered faces: 0; increased counts on previously covered faces: 0. Every
previously covered face retains exactly its former count. This comparison is against the original base; the refreshed
base comparison is still required before landing. No specificity-based
selection is introduced. Byte-exact realization, lexical ownership and
construction/leaf traversal validation pass for every counted Reading.

The complete before/candidate census, comparison, probe, inventory and
spot-check artifacts are under this workspace's ignored `target/english-v3/`.
Ten newly covered faces have all 50 Readings inspected: Ebon Dragon (1),
Quill-Slinger Boggart (1), Rage Forger (15), Joraga Bard (10), Extractor Demon
(1), Wandering Troubadour (5), Lava Blister (2), Distant Memories (7), Renegade
Doppelganger (7), and Browbeat (1). Every inspected Have occurrence is
`SelectedPredicate` with an Object NP and `BarePredicate` Complement, not a
possessive analysis or a reduced relative absorbing the bare infinitival.
The *had* experiential use shares this complex catenative structure; CGEL,
Ch. 14 §5.4, p. 1236, also discusses the non-causative “undergo” sense.

Full-face witness counts: Ebon Dragon 0 → 1; Quill-Slinger Boggart 0 → 1;
Rage Forger 0 → 15; Joraga Bard 0 → 10; Mirror Image 0 → 0, blocked as above.
The attestation scan found no imperative causative-Have instruction; no invented
standalone effect is made an authentic positive regression witness.

Grammar economy on `ostqqnplnqrm` / covered 19,682 and 19,826:
claim-parent and candidate declarations each have 3,227 non-blank lines,
**net change 0**; 204 ordinary Constructions and 43 shared schemas remain
unchanged. Features added: none. Tables added: none. Policies added: none.
Constructions added: none. One lexical frame replaced, as shown above.
No admission guard is added, including forbidden word-named guards: zero.
Lexical loading succeeds with zero load errors; 87 unmapped annotations are
unchanged. The active v3 tooling has no coverage lock or `environment.rs`;
the covered counts here come from `english-v3`, and no legacy permitted
licensing-checker total is emitted by it.

The named generated-form homograph inventory is unchanged at 586 surfaces
(excluding card-name catalogs and capitalization alternatives), retained in
`target/english-v3/inventory-before.json` and `inventory-after.json`.
Word-bearing grammar form literals and their vocabulary overlaps are empty,
as on the unchanged claim-parent declarations. The glossary gains Bare
Infinitival, Catenative Construction and Catenative Complement, citing the
passages that define them (CGEL, Ch. 14 §§1.1–1.2, pp. 1173–1178).
These are linguistic terms, not new implementation features.

Performance advisory, six workers:

| Snapshot / covered | Corpus wall time (ns) | Checked-text thread CPU | Host load (1/5/15 min) |
|---|---:|---:|---|
| Before `ostqqnplnqrm` / 19,682 | 112,705,430,965 | 260,907 ns/B | 14.25 / 12.93 / 13.06 |
| Candidate `ostqqnplnqrm` / 19,826 | 121,072,113,809 | 268,912 ns/B | 11.79 / 14.82 / 13.93 |

Both exceed the 16,260,000,000 ns quiet-host advisory; these loaded-host
measurements do not establish quiet-host performance. Census commands used the
workspace's built `target/debug/cargo-xtask english-v3 --all --workers 6
--samples-per-face 0 --output ...` while Cargo held the build lock. No grammar
Rust code changed. Gate command derived by `cargo xtask gate --changed --from
pozynvyu --clippy --run`:

```text
cargo test -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask
cargo clippy -p deckmaste_lexical_source -p deckmaste_construction_v3 -p deckmaste_english_v3 -p xtask --all-targets -- -D warnings
```

The full gate cannot pass while the preserved Mirror Image witness fails.
Formatting passes. Citation noncompliance is empty; the citation checker
reports 0 stale. The diff citation audit selects 0 changed rule sites (no CR
citation changed). Final gate results, refresh, and a resolved Landing record
remain required before integration.

## Landing record

In addition to the standard record: the replacement Have frame and the retired
legacy role slots; Reading counts of each witness; before/after counts stamped
with change ids; timings as integer ns and ns/B with host load and worker
count.
