---
needs: []
---
# Share one Reading test helper with a constituent assertion

Give the `deckmaste_english_v3` integration tests a single shared support module
and add an assertion that states an intended analysis without spelling a full
`Reading` value.

Two facts motivate it. The function `readings(text, category)` (parse, check each
Reading realizes back to the text, return the set) is copy-pasted into 33 of the
37 test files, each with its own `static LEXICON`. And nearly every test asserts
a hand-built `Reading` value, so a change to how a construction is represented
means re-spelling expectations across the suite; the archived attempt at
`english-v3-generic-frame-consumption` rewrote about fifty test files that way,
which is exactly when a wrong expectation slips in unnoticed.

Pinned shape:

- one support module (`tests/common/`) owning the lexicon and `readings`; the 33
  copies are replaced by it. Existing assertions are not rewritten.
- an assertion of the form "some Reading of TEXT at CATEGORY contains a
  Constituent of Category C spanning exactly SUBSTRING", taking several such
  constituents at once, all required of the same Reading. It names Categories
  and source text only: no construction, form index, frame choice or field
  layout. Example: the Noun Phrase "sources of the color of your choice" with
  "the color of your choice" as a Noun Phrase asserts the low Of attachment.
- where a text has two justifiable Readings the grammar is not meant to choose
  between (Seedborn Muse's ability is the user's example), the test makes two
  such assertions on the same text.

The helper is for new tests and for tests a later ticket must re-spell anyway.
It does not replace value-equality tests where the exact value is the point, and
it is not a fixture: no data file, no expectations outside ordinary test
functions, nothing generated from parser output.

Acceptance: the shared module in use by every test file that had a copy; the
constituent assertion with tests of its own, including one that fails for the
right reason (a span that is not a constituent in any Reading); no test removed
or weakened. Standard constraints apply.
