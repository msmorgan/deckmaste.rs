---
needs: [english-structural-recovery-zero]
---
**Generic predicate shell, Deontic promotion, and elevated VP coordination.**
The implementation refines the 2026-07-23 sketch in one important respect:
predicate coordination has no distinguished first member. Its landed shape is
`IndependentClause::Predicated(subject, PredicateExpression::Coordinated(
Coordination<Predicate>))`; `Coordination<T>` stores a uniform `Vec<T>` of
conjuncts and a separate aligned vector of junctions. Its fields are private,
and construction preserves `junctions.len() + 1 == conjuncts.len()`.

- `HeadedPredicate<K> { head: PredicateHead, kind: K, elements }` is the
  shared shell for head-bearing kinds. `Transitive` carries
  `pre_object_elements` and `object`; `Intransitive`, `Passive`, and
  `ObjectGap` provide their kind-specific payloads. `Copular` and `Proform`
  remain outside the shell because they do not have a `PredicateHead`; the
  heterogeneous `Predicate` enum covers all kinds.
- Modality is available in the predicate layer as `Predicate::Deontic`, so a
  modal is a uniform coordination member. Modal relative subject gaps also use
  that form, eliminating `RelativeBody::ModalSubjectGap`. The established
  uncoordinated finite-clause leaves remain as compact representations and are
  promoted when coordination requires a heterogeneous predicate value.
- A shared subject is owned once by `IndependentClause::Predicated` and scopes
  over the entire predicate expression. `CoordinatedClauseMember` can now hold
  only complete independent clauses; the old `SharedPredicate` and
  `SharedDeontic` continuations are gone. Full-clause coordination, where each
  conjunct has its own subject, stays separate.
- `Predicate::Attached` keeps a trailing dependent with the conjunct it
  modifies (`P1 unless C or P2`, `P1, where C, then P2`), while fronted
  attachments continue to scope over the whole clause. A changed overt
  connective starts a new clause group, so `C1 and C2, or P3` does not
  misattach `P3` under C2's subject.
- Agreement propagates from the host finite slot into each subjectless
  non-modal conjunct when the parsed base and finite forms are syncretic. An
  already finite form remains intact; a surface-changing reinterpretation
  such as noun-like `target` -> `targets` is rejected. The shared shell handles
  lexical predicates, with dedicated copular and proform arms; deontic inners
  remain infinitival.
  Motivating defects: Lord of Atlantis's shared `have` previously carried
  `Infinitive` instead of third-person plural present, and the shared subject
  was buried inside the first conjunct — arbitrarily deep when that conjunct
  was itself complex.

Representation-only: recovery census byte-identical and round-trip stays at
zero. The agreement half is invisible to the round-trip gate (the spellings
are identical), so it is held by feature-level tests asserting verb slots:
Lord of Atlantis (plural `have`), Aerial Engineer (singular `has`), and one
mixed transitive+deontic coordination witness. Standard constraints apply.

## Completion

- Shared-subject declaratives and imperatives lower to a clause-owned subject
  plus `Coordination<Predicate>`; mixed `S1 P1 and S2 P2 and P3` attaches the
  final predicate under `S2`. Two- and multi-member coordination, asyndetic
  commas, modal members, VP ellipsis, per-conjunct dependent scope, connective
  group boundaries, and full subject-bearing coordination have causal
  structure and source-free rendering tests.
- The generic headed-predicate shell is shared by transitive, intransitive,
  passive, and object-gap predicates. Modal relative gaps and coordinated
  modals use `Predicate::Deontic`.
- Feature tests assert plural `have`, singular `has`, an infinitival inner
  predicate under a coordinated modal, and surface-preserving agreement on
  ambiguous bare tails. The English suite passes (576 unit tests and 112
  public-API tests); all 31,685 supported faces round-trip with zero mismatches
  or render errors.
- The live recovery census is byte-for-byte identical to the parent: 3,480
  structural spans / 64,217 source tokens (3,345 / 63,129 clause; 42 / 175
  activation cost; 43 / 264 keyword argument; 4 / 67 modal header; 46 / 582
  embedded rules), plus 792 noun and 622 flavor-header opacity spans. The
  diff-scoped citation audit selected no citation sites.
