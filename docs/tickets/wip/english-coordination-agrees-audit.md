---
needs: [english-coordination-residue]
---
**Tighten the two latent `coordination_agrees` over-fire arms.** Distinguish
productive imperative sequencing and subjectless modal members from the
agreement-less finite hosts and bare infinitives that those broad tuple arms
also admitted.

## Completion

- Replaced `(None, None, true)` with a structural nonfinite-host check. Removing
  the arm wholesale exposed 1,940 unresolved rows because it is the productive
  imperative-to-imperative path (`Shuffle, then scry`); checking the existing
  first-clause `finite` feature preserves all of them while rejecting an
  existential finite host.
- Replaced `(Some(_), None, false)` with a check for the continuation's existing
  `host_modal` feature. This preserves the landed subject-shared deontic family
  (`gets +1/+0 and can't be blocked`) without letting an ordinary infinitive
  masquerade as an agreeing finite predicate.
- Added direct negative tests for `That player sacrifices a creature, then draw
  a card` and `There is a creature, then draw a card`; the existing imperative
  chains, finite agreement, and six shared-deontic structural tests remain
  green.
- The finite-host gate has one sole corpus consumer, Titania, Voice of Gaea. Its
  prior bracket tree mis-scoped `there are ... and you` and already left a
  12-word clause recovery plus opaque `Argoth`; it now becomes one honest
  44-word clause recovery.
- The nonmodal-infinitive gate exposes 12 more unsound trees. They are fully
  attributed in `english-coordination-scope-host-residue`: shared auxiliary
  scope, subject/host shifts, last-member agreement, preverbal `then`, and
  coordinated infinitives.
- Net census movement is +12 clause recovery occurrences and +281 recovered
  source tokens. One formerly opaque noun token (`Argoth`) is now inside the
  honest Titania clause recovery; every other recovery and opacity cell is
  unchanged.
