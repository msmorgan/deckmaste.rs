---
needs: []
---
# Comma scope, Nodule 1: one structural `internal_comma` bit

Implements owner rulings round 12 (2026-10-23), findings F1 and F2 (Nodule 1A
and 1B of the oncology re-scan). Both block Stratum 7.

## Change

In `crates/deckmaste_syntax/src/step.rs`:

- **F1, `comma_scope_excludes`.**
  - Keep `summary.fronted && post_host`: the `Pre[,] [Clause, Post]`
    canonicalization of `docs/decisions/english-lexical-analysis.md` §2.
  - Delete `Summary::supplemented`. Banning a Supplement-hosting Clause from an
    outer comma-set post-dependent is admission-time suppression; ranking
    (Supplement high attachment) holds the preference.
  - Restrict `first_conjunct` to undelimited coordinations
    (`self.coordinator.is_some() && !self.delimits`). In a comma-delimited
    clause coordination (*[If you do, A], and [if you don't, B]*) the first
    conjunct has its own fronted adjunct.
- **F2, `delimited_complement` and `post_adjunct`.** Replace both with one
  structural bit, `Summary::internal_comma`.
  - Set it when a clause is formed by a `delimits` coordination or attaches a
    comma-set post-dependent.
  - Propagate it through `undelimited` children and `SlotRelation::Complement`,
    removing the hardcoded `"complement"` child-name check in
    `Production::accept`.
  - Measured, that propagation alone admits a preposed clause with the comma
    in a first conjunct (Gilded Cerodon: *Whenever [[this creature attacks,
    if you control a Desert] or …], …*, +149 Readings on 30 units) or in an
    Adjunct inside it (Wargling). The rule's own statement is that the comma
    runs to the end of its sentence, so every phrase that contains it carries
    the bit. That subsumes `undelimited` and the Complement path, so
    `ProductionDecl::undelimited` and its compiled field are deleted, and the
    reserved name `complement` loses its only engine behaviour.
  - Reject it only where the phrase attaches before a comma (a `Before`
    Adjunct delimited by `,`, or a Preposed one): a preposed subordinate
    clause ends at its comma, so it cannot contain an unbracketed comma-set
    post-dependent (`*When [C1, if C2], C3`). A sentence-final one runs to the
    period and can.
  - Delete `Summary::delimited`, which nothing reads once `internal_comma`
    carries the fact (owner rulings round 13, item 1.3).
  - Net: `Summary` −`supplemented`, −`delimited_complement`, −`post_adjunct`,
    −`delimited`, +`internal_comma`; `State` −1 field. Update the `ProductionDecl` reserved
    child-name contract in `grammar.rs` and the syntax README to match.

- **Round 12b, finding A: a serial coordination's middle conjunct.** The
  serial tails (`serial-more`, `serial-end` and the polysyndetic links) name
  their non-final conjunct `first`, so the fronted-scope rule also banned a
  fronted dependent on a middle conjunct (*C1, [if X, C2], and C3*). A
  compiled `Production::tail` marks the tails, and only the initial conjunct
  of an undelimited coordination is ruled out.
- **Canonicalization #6 in the normal form.** `normal_form.rs` rewrites
  *[Pre[,] Clause], Post* to *Pre[,] [Clause, Post]* (the fronted dependent
  outermost, `docs/decisions/english-lexical-analysis.md` §2), and
  `seed_proof.rs` checks that equivalence.

## Acceptance

- Positive judgments for corpus witnesses of the new admissions: *, where X
  is …, rounded up/down*; a sentence-final subordinate clause containing a
  comma-set post-dependent; *[If you do, A], and [if you don't, B]*.
- A negative judgment for `*When [C1, if C2], C3`.
- Round-9 acceptance (`docs/decisions/english-v4.md` §4.4, item 9): no face
  lost, every positive assertion holds, nothing ungrammatical admitted,
  `cargo xtask english-v4 strata --check` passes, no pathological forest
  growth. F1 is expected to add about 20 to 60 Readings; F2's effect is
  reported separately.

Standard constraints apply.

Census delta (measured on `yrrlkusk` → `skwsrsll`): faces read 28,801 → 28,801; total Readings 245,781 → 250,368 and Surplus 106,817 → 109,721 (95 units grew, none shrank, no verdict changed); cheapest tier singleton in 15,624 of 18,344 → 15,637 of 18,353, summed 21,723 → 21,727, largest 8 → 8. By part: F1 Supplement deletion +28 on 7 units; F1 delimited first conjunct +1,462 on 45; F2 +2,711 on 41; round 12b A +386 on 8. Largest unit Evelyn, the Covetous 420 → 1,736. Lost faces: none.
Judgment set (`skwsrsll`): 3,060 enforced and 207 pending, all holding; S/A/P/F not computed (no in-repo tally).
