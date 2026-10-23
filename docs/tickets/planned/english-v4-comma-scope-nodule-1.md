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
  - Reject it only where the phrase attaches before a comma (a `Before`
    Adjunct delimited by `,`, or a Preposed one): a preposed subordinate
    clause ends at its comma, so it cannot contain an unbracketed comma-set
    post-dependent (`*When [C1, if C2], C3`). A sentence-final one runs to the
    period and can.
  - Keep `Summary::delimited`.
  - Net: `Summary` −`supplemented`, −`delimited_complement`, −`post_adjunct`,
    +`internal_comma`; `State` −1 field. Update the `ProductionDecl` reserved
    child-name contract in `grammar.rs` and the syntax README to match.

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

Census delta: not yet measured.
Judgment set: not yet run.
