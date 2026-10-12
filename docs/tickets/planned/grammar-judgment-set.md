---
needs: [grammar-analysis-key]
---
# Judgment set, the judging agents, and three-number coverage

Build the instrument that says whether an analysis is right, and make the census report coverage so that a wrong analysis never counts as covered. The existing gates (parses, round-trips) cannot see a wrong analysis: "the card you have put into your hand" gets one reading, and it is `[the card [you have]] [put into your hand]`, a finite-gap relative plus a participial postmodifier; "the card you are putting into your hand" gets zero.

## The judgment set

- One tracked file, hand-written. One line per entry: text, category, expectation. No tree dumps. At most about 300 entries.
- Three kinds of expectation, each checked by the test:
  - Required analysis: the text, parsed as the category, has a reading whose analysis key equals the stated analysis.
  - Forbidden analysis: no reading of the text has the stated analysis key.
  - Unmet obligation: a grammatical text the grammar cannot yet read, or a text read only by a stated wrong analysis. The wrong analysis, where there is one, is stated in the line.
- An analysis is written in the bracketed form the candidate renderer of `grammar-analysis-key` prints, and compared with the analysis keys the census computes, never with tree shape.
- Lifecycle: an unmet obligation never fails the gate; it is counted and listed. When a later landing satisfies one, that landing promotes it to a required analysis (and to a forbidden analysis for the stated wrong one), and says so in its note; an entry is never edited or removed silently, and never to make a change pass without the owner. A required or forbidden entry that fails is a gate failure.
- What gates this landing: every entry that holds today is recorded as required or forbidden and passes; every seed the current grammar cannot meet is recorded as an unmet obligation. The test fails on a required or forbidden failure and on any entry whose line cannot be parsed.
- Seeds: the reviewers' probes (for example "the card you have put into your hand", "While saddled, this creature has flying.", "Target creature gets -0/-9999 until end of turn.", "beyond the first", "You may choose both.", "Put one into your hand and the other into your graveyard."), the retired blocker-triage list (fused heads, gapped coordination, heavy-NP shift, "do so"), and the known-wrong list from the retired orchestration state (ten faces).
- A test runs the set against the current grammar as the lifecycle above describes.

## The judging agents

Entries are decided by two judging agents working independently on different models, neither seeing the other's answer.

- Given: the text and category, the question being asked (is this a grammatical analysis of the text; which of the candidate analyses are), the candidate analyses from the analysis-key tool in readable bracketed form with heads and functions, and access to CGEL through the cgel-reference skill and to `docs/oracle-style-guide.md`.
- Must return: a verdict per candidate (grammatical analysis, known-wrong, or cannot tell), the entry's expectation in the file's terms, and for each verdict a citation quoted verbatim: CGEL chapter, section and printed page, or the style guide's file and lines. A ruling without a verbatim citation is not a ruling.
- Recording: both answers and citations are kept per entry in an untracked record under `docs/evidence/`; the tracked line carries only text, category and expectation. Agreement with citations that the quoted text supports goes in without the owner. Disagreement, a missing or unsupported citation, or "cannot tell" goes to the owner in a batch showing both positions side by side.

## Three-number coverage

The census reports faces as read, read only by a known-wrong analysis, and unread. A face is read if a correct analysis is among its readings: wrong readings alongside a correct one are acceptable, since not every wrong reading can be eliminated. Only a face whose readings are all known-wrong is read only by a known-wrong analysis. There is no count of faces that merely carry some wrong reading. A known-wrong analysis never counts as covered, and no ruling may accept one.

- A reading is known-wrong if a judgment entry's stated wrong analysis (an unmet obligation or a forbidden analysis) matches it; any other reading of a read face counts as correct. A face whose admitted analyses nobody has judged counts as read, and the report says plainly that this is not a correctness claim.
- Matching inside a complete face: an entry is a fragment. A reading matches when it contains a node of the entry's category whose span text equals the entry's text and whose analysis key equals the stated analysis. An entry may name a card; without one it matches only fragments that do not depend on card-name scoping.
- `diff`, the blockers report and the later marginal-rate denominator (`grammar-coverage-resume`) use this same classification, and the lists of known-wrong faces are written under `target/english-v3/`.

## Gate

Reading digests identical on every face, enforced by the census check mode for an identical-digest landing. The landing updates the census description in `crates/deckmaste_english_v3/README.md`. Report in full: census delta (none; the three coverage numbers stated for the first time, earlier landings having reported plain read and unread); the size report and the tracked grammar line total lowered to the new total; judgment-set result (entries by kind, unmet obligations listed, agreed versus sent to the owner, test outcome).
