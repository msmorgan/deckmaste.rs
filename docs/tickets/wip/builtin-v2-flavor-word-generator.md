Track the flavor-word nursery generator (flavor-word landing review M2).
The 630 `FlavorWord` stubs were produced by an untracked one-off; only the
census verifier exists (in tests/), so a newly printed flavor word must be
hand-added and repaired off a 630-element set-inequality diff. Add a
tracked xtask path (e.g. `cargo xtask english_v2 flavor-words --check /
--regenerate`) that derives the declaration set from the corpus census
with the SAME predicate the verifier uses (one shared implementation, not
two), writes stubs deterministically (sorted, byte-stable), and reports
the diff. Retire the hardcoded `assert_eq!(expected.len(), 630)` in favour
of check-mode equality against the generator. Zero grammar change;
standard constraints apply.
