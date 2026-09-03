Close the line-initial twin of the tripwire escape (tripwire-fixes
review M1). "(a wholly novel gloss) trample." still normalizes to
"trample." with exit 0: the tripwire classifies mid-line groups only, and
a novel group at line START followed by text escapes. Relax the predicate
so any group followed by non-whitespace text on its line is classified
(`!after.trim().is_empty()`); line-FINAL groups (10,414 in the corpus,
the ordinary trailing-reminder shape) stay outside classification — the
ticket's earlier "every group wherever it appears" clause is superseded
by this reading, which is the only one consistent with corpus load and
identity stability. Zero non-classified line-initial-with-text groups
exist in the corpus, so identities cannot move; prove it. Also retire
`previous_strip_reminder_text`: with preservation restored it equals the
shipped scanner, making "the normalizer equals the pre-tripwire
normalizer" a standing law — replace it with the positive probes that
now exist. Standard constraints apply.

## Landing record

The tripwire now classifies every parenthetical followed by non-whitespace
text on its line, including a group at line start. The direct
`(a wholly novel gloss) trample.` probe returns a card-naming normalization
error. Unknown line-final groups remain outside classification and continue to
strip as reminder text, while classified rules-bearing groups remain preserved
byte-exactly in every position.

The Vintage-playable corpus contains zero line-initial parentheticals followed
by text. The production inventory still passes every card through the scanner,
and the new explicit zero-count invariant proves that the predicate change
moves no current normalized identity.

| gate | before | after |
| --- | ---: | ---: |
| corpus units | 32,641 | 32,641 |
| selected and covered identities | 15,966 | 15,966 |
| line-initial parentheticals followed by text | 0 | 0 |
| construction declarations | 378 | 378 |
| coverage lock | current, 15,972 lines | current, unchanged |

The coverage lock remains byte-unchanged (SHA-256
`b6a4a064882ecde7dca01b41345c805e6bfba7206fb6b58a63af807bcc91406e`).
The refreshed coverage check reports zero selected-uncovered units, unresolved
ties, internal failures, exception uses, round-trip mismatches, ownership
failures, gaps, overlaps, synthetic claims, or provenance-plan mismatches. Its
two pre-existing literal/lexicon collisions are unchanged.

Positive gates: all 419 non-ignored xtask library, CLI, determinism, and doc
tests; strict all-target xtask Clippy; `cargo xtask english_v2 coverage
--check`; and the full workspace test suite after refresh.

### Deviations and additions

- Added `unknown_line_initial_parenthetical_is_a_card_named_error` and the
  Vintage snapshot zero-count invariant. Removed
  `exhaustive_tripwire_preserves_every_current_normalized_identity` and its
  `previous_strip_reminder_text` shadow scanner: the shadow had become the
  shipped scanner duplicated in test code, so its equality assertion was no
  longer an independent oracle. The classified preservation, stripping,
  inventory, and new line-initial probes now provide positive evidence.
- Re-spelled the leading-reminder inputs in
  `reminder_stripping_pins_surrounding_space_and_malformed_input_contracts`
  and `unknown_mid_line_parenthetical_after_a_leading_reminder_is_an_error`
  from unclassified synthetic placeholders to the classified `(the Fridge)`
  spelling. Their asserted spacing and later-unknown-group behaviors are
  unchanged.
- Assurance counts: restored 0 tests; re-spelled 2 tests; ignored 0 tests with
  new blockers; added 2 tests; removed 1 test, named and justified above.
- No constructions were added or deleted. No CR citations changed. No STOP was
  taken.
