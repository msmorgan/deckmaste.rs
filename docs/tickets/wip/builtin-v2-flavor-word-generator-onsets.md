Stop the flavor-word generator guessing onsets (flavor-generator review
H1/M1). `render_stub` writes `onset: Consonant` for any spelling the onset
recipe cannot classify — forbidden by the builtin-v2 spelling/grammar ADR
(authored, attested overrides only; never inferred). Replace with an
explicit authored `(surface, onset)` override table (today: "... Catch" and
"10,000 Needles", both Consonant, attested) and a HARD ERROR for any
unclassifiable spelling absent from it; `--check` verifies the override
table against the nursery too, and a hand-authored override is never
reverted. Add a test that runs `flavor-words --check` (and the
one-shared-predicate equality) so a census/predicate edit cannot silently
regenerate the nursery green; wire `--check` into the standard gate list.
Zero grammar/lock change. Standard constraints apply.
