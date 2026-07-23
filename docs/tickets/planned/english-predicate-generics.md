---
needs: [english-structural-recovery-zero]
---
**Generic predicate shell, Deontic promotion, and elevated VP coordination.**
Settled design (2026-07-23, with the user):

- `Predicate<K> { head: PredicateHead, kind: K, elements }` for the
  head-bearing kinds — payload `Transitive { pre_object_elements, object }`;
  `Intransitive` and `Passive` as unit payloads (type-level tags over
  identical data; `ObjectGapPredicate` joins the shell). `Copular` and
  `Proform` stay outside the shell (no `PredicateHead`); an `AnyPredicate`
  enum over the instantiations plus the two outsiders covers heterogeneous
  positions.
- Modality moves into the predicate layer: `Deontic { modal, inner:
  Option<Box<AnyPredicate>> }` becomes a predicate kind, collapsing
  `IndependentClause::Deontic` and `RelativeBody::ModalSubjectGap`.
- The clause layer threads the type: per-kind `IndependentClause` variants
  wrap `Clause<K> { subject, predicate: Predicate<K> }`, so the kind tag
  exists once instead of twice.
- Shared-subject coordination elevates the subject:
  `PredicateCoordination { subject, first: AnyPredicate, rest }` with
  uniform predicate members (modality included via the promotion);
  `CoordinatedClauseMember::SharedPredicate` dies. Full-clause coordination
  (per-member subjects) is untouched.
- Agreement propagates from the elevated subject into every member's finite
  slot — written once against the shell head, plus one arm for the copula's
  inflection site. Motivating defects (verified 2026-07-23): Lord of
  Atlantis's shared `have` carries `Infinitive` (should be
  `Present { person: Third, number: Plural }`), and the shared subject is
  buried inside the first conjunct — arbitrarily deep when `first` is
  `Complex` (Aerial Engineer).

Representation-only: recovery census byte-identical and round-trip stays at
zero. The agreement half is invisible to the round-trip gate (the spellings
are identical), so it is held by feature-level tests asserting verb slots:
Lord of Atlantis (plural `have`), Aerial Engineer (singular `has`), and one
mixed transitive+deontic coordination witness. Standard constraints apply.
