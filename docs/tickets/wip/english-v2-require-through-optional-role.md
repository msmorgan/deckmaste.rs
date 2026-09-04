---
needs: [english-v2-pp-construction]
---
`require` cannot read a feature through an `opt Category` role: the generated
invariant receives `&Option<T>` and code generation has no unwrap for it.
Teach the invariant emitter to map over an optional category role (absent
satisfies the predicate), then dissolve `SourcePhrase` and `ControlPhrase`
back into the general prepositional phrase under a preposition-class guard —
they exist only because a codec tail cannot carry an optional literal and the
slots could not otherwise be fenced. Standard constraints apply.

## Landing record

Measured on change `owkxqznkysrxxpuunltksnzyvmlkxvom` with 16,771 covered
lock identities. The measured lock has 49,421 lines and SHA-256
`2cf7f9b716e13b27d626c60638d1266ca6cdc083bbcca87cb1435b4b00cd65ca`.

| gate | refreshed parent | measured tree |
| --- | ---: | ---: |
| selected and covered units | 16,768 | 16,771 |
| ordinary parse failures | 15,873 | 15,870 |
| unique selections | 11,009 | 11,515 |
| specificity-resolved selections | 5,759 | 5,256 |
| unresolved ties | 0 | 0 |
| construction declarations | 399 | 397 |
| coverage-lock identities | 16,768 | 16,771 |

- Coverage and lock state: the lock is **+3/-0 identities**, from 49,418
  lines and SHA-256
  `a4fdc96b7bcb2770104e6cb0cda1f0a3f5cd63d529d30e1c5d1147e714fb3dd1`
  to the measured state above. The additions are Cavalier of Thorns, Animal
  Magnetism, and Genesis Ultimatum, all admitted by the dissolved general
  selected-role tail in coordinated movement clauses. No retirement manifest
  was created or used. Selected-uncovered units, internal failures, exception
  resolutions, exception uses, round-trip mismatches, ownership failures,
  gaps, overlaps, synthetic claims, and provenance-plan mismatches are all
  zero; traversal failures are also zero. Literal/lexicon collisions decrease
  60 -> 59 with the two wrapper
  construction surfaces removed.
- Selection census: 504 already-covered units move from specificity resolution
  to unique selection after the wrapper paths disappear. Of the three newly
  covered units, Cavalier of Thorns and Animal Magnetism select uniquely and
  Genesis Ultimatum remains specificity-resolved, producing the table's net
  +506 unique/-503 specificity shift. Exact rendering and locked identity are
  preserved for every previously covered unit.
- Amendment audit: the optional source role is the general
  `PrepositionalPhrase` fenced by the declared `SourceComplement` feature. A
  hard object-build constraint removes the same source-class noun-postmodifier
  derivation inside frames that declare that role; it does not add a
  specificity weight and names no lexeme, verb, noun, construction, or card
  identity. The paired probes select one candidate for both `Return target
  creature card from your graveyard to your hand.` and the same frame with the
  optional source absent; the `Destroy ... from your graveyard.` counterpart
  retains its single noun-postmodifier derivation. The control role is the
  general PP fenced by its declared `SelectedOnly` class, which also preserves
  the coordinated `... onto ... and the rest into ...` readings represented by
  the three coverage additions.
- Positive artifacts after refresh: `cargo fmt --all -- --check`; `cargo
  clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`;
  the complete `deckmaste_construction_core` suite; all 100
  `predicate_grammar` tests; `cargo xtask english_v2 ambiguity
  --require-resolved --json`; `cargo xtask english_v2 coverage --check
  --json`; and `cargo xtask cite check` all exited zero.
- Performance advisory: the final 24-worker coverage gate took 20.967337467 s
  against the 16.26 s quiet-host ceiling and reported 103.762 thread-CPU
  microseconds per accepted byte over 16,771 units / 1,523,802 bytes. Host load
  was 18.10/20.12/26.83, so the command emitted the specified non-failing
  busy-host warning; this is advisory rather than a quiet-host breach. The
  ambiguity gate likewise completed with zero ties at 41.013654664 s and
  143.966 thread-CPU microseconds per accepted byte under load
  21.13/20.77/27.21.
- Assurance census: restored 0; re-spelled 0; ignored with blockers 0; added 1
  test function; removed 0. The new test authenticates unique source-role
  attachment, satisfaction when the optional role is absent, and preservation
  of the noun-postmodifier reading when the verb frame does not declare the
  source. The existing compiler fixture is strengthened with absent, allowed,
  and rejected optional-role feature assertions.
- Deviations and additions: exactly the ticket's two wrapper constructions,
  `SourcePhrase` and `ControlPhrase`, are deleted; no construction is added.
  The recorded 2026-09-03 amendment adds the declared-feature-driven hard
  source-preemption walk and its one regression test beyond the ticket's
  original wording. The lock gains the three identities listed above; there
  are no other test additions or deletions. The final refresh incorporated the
  concurrent repeatable-NP-postmodifier landing, so the source-preemption walk
  was adapted from the retired fixed stages to that landing's general
  `PostmodifiedReference` spine.
- STOPs: the original `from ...` frame-role versus noun-postmodifier attachment
  STOP was resolved by the recorded amendment “selected roles preempt
  postmodifiers (2026-09-03).” The implementation follows that authority as
  hard candidate elimination derived from the frame's declared feature
  valence, not as a preference. The complete final census has zero unresolved
  ties, so no fresh STOP remains. The final refresh recorded one mechanical
  conflict in the literal/lexicon-collision test: the parent moved 59 -> 60
  while this feature moved 59 -> 58. Harmony resolved the composed value to
  59, after which refresh was a no-op and the complete gate set passed.
