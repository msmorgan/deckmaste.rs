Close the tripwire's residue (tripwire-landing-review M1/M2/M3/L1).
- M1: rules-bearing preservation was narrowed from unconditional to
  mid-line-only; a guarded spelling at LINE END ("…return it to the
  battlefield (front face up)") now strips again silently. Preservation
  is positional-agnostic: a classified rules-bearing group is preserved
  wherever it appears; the tripwire classifies every group wherever it
  appears.
- M2: `is_mid_line` reads the post-strip prefix, so a group behind a
  stripped leading reminder escapes classification ("(A leading
  reminder.) (a wholly novel gloss) trample." normalizes silently).
  Classify against the ORIGINAL line, before any stripping.
- M3: empty "()" flipped from pass-through to error with its pin deleted
  and no deviation recorded — decide explicitly (an empty group is not
  reminder text; error is defensible) and pin it with a test either way.
- L1: restore the adjacent-parenthetical no-double-space pin verbatim.
Identity stability: zero changes to any normalized text on the current
corpus (all 17 rules-bearing uses are mid-line today); lock unchanged.
Deviations section must list every test added/deleted. Standard
constraints apply.

## Landing record

Parenthetical classification now uses each group's position in the original
line, before any earlier group is stripped. The five classified rules-bearing
spellings are preserved byte-exactly at the beginning, middle, or end of a
line. An unknown true mid-line group following a leading reminder therefore
returns a card-naming normalization error instead of escaping the tripwire.

Empty `()` is explicitly an error when it is mid-line: it is not reminder text,
so silently stripping it would weaken the exhaustive classifier. Adjacent
classified reminders retain exactly one surrounding space.

| gate | before | after |
| --- | ---: | ---: |
| corpus units | 32,641 | 32,641 |
| selected and covered identities | 15,932 | 15,932 |
| construction declarations | 398 | 398 |
| coverage lock | current, 15,932 identities | current, unchanged |

The full-corpus identity probe reports zero normalized-text changes and still
finds exactly 17 rules-bearing uses that differ from the raw strip. The
coverage gate reports zero selected-uncovered units, ties, internal failures,
exceptions, round-trip mismatches, ownership failures, gaps, overlaps,
synthetic claims, or provenance-plan mismatches. The coverage lock remains
byte-unchanged (SHA-256
`a466fcf1289b7607095d050bab1bf69388e47dbe65a371409cda6881aa8fbe6a`).

Positive gates: `cargo fmt --all -- --check`; all 415 non-ignored xtask unit,
binary, integration, and doc tests; strict all-target xtask Clippy; and
`cargo xtask english_v2 coverage --check`.

### Deviations and additions

- Added `unknown_mid_line_parenthetical_after_a_leading_reminder_is_an_error`
  for M2 and `empty_mid_line_parenthetical_is_a_card_named_error` for the M3
  decision. No tests were deleted.
- Expanded `rules_bearing_parentheticals_survive_byte_exactly` to pin every
  allowlisted spelling at line start, mid-line, and line end. Restored the
  adjacent-parenthetical no-double-space assertion inside
  `reminder_stripping_pins_surrounding_space_and_malformed_input_contracts`,
  using two classified reminder spellings so the exhaustive tripwire is not
  weakened.
- Assurance counts: restored 1 assertion; re-spelled 0 tests; ignored 0 tests
  with new blockers; added 2 tests; removed 0 tests.
- No constructions were added or deleted. No STOP was taken.
