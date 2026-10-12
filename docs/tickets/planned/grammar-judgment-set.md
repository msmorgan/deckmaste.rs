---
needs: [grammar-analysis-key]
---
# Judgment set, the judging agents, and three-number coverage

Build the instrument that says whether an analysis is right, and make the census report coverage so that a wrong analysis never counts as covered. The existing gates (parses, round-trips) cannot see a wrong analysis: "the card you have put into your hand" gets one reading, and it is `[the card [you have]] [put into your hand]`, a finite-gap relative plus a participial postmodifier; "the card you are putting into your hand" gets zero.

## The judgment set

- One tracked file, hand-written. One line per entry: text, category, expectation. No tree dumps. At most about 300 entries.
- Expectations: an expected analysis (a stated analysis the text must have), an expected failure (the text must have no reading), a known-wrong pass (the text parses today but only by a wrong analysis).
- Seeds: the reviewers' probes (for example "the card you have put into your hand", "While saddled, this creature has flying.", "Target creature gets -0/-9999 until end of turn.", "beyond the first", "You may choose both.", "Put one into your hand and the other into your graveyard."), the retired blocker-triage list (fused heads, gapped coordination, heavy-NP shift, "do so"), and the known-wrong list from the retired orchestration state (ten faces).
- A test runs the set against the current grammar and fails when an entry's expectation is not met. An entry is never edited to make a change pass without the owner.

## The judging agents

Entries are decided by two judging agents working independently on different models, neither seeing the other's answer.

- Given: the text and category, the question being asked (is this a grammatical analysis of the text; which of the candidate analyses are), the candidate analyses from the analysis-key tool in readable bracketed form with heads and functions, and access to CGEL through the cgel-reference skill and to `docs/oracle-style-guide.md`.
- Must return: a verdict per candidate (grammatical analysis, known-wrong, or cannot tell), the entry's expectation in the file's terms, and for each verdict a citation quoted verbatim: CGEL chapter, section and printed page, or the style guide's file and lines. A ruling without a verbatim citation is not a ruling.
- Recording: both answers and citations are kept per entry in an untracked record under `docs/evidence/`; the tracked line carries only text, category and expectation. Agreement with citations that the quoted text supports goes in without the owner. Disagreement, a missing or unsupported citation, or "cannot tell" goes to the owner in a batch showing both positions side by side.

## Three-number coverage

The census reports faces as read (no known-wrong analysis), read only by a known-wrong analysis, and unread. A known-wrong analysis never counts as covered, and no ruling may accept one. Known-wrong faces come from the judgment set's known-wrong passes and are listed in the census output under `target/english-v3/`.

## Gate

Reading digests identical on every face. Report in full: census delta (none; the three coverage numbers stated for the first time); size delta per declaration kind; judgment-set result (entries by expectation, agreed versus sent to the owner, test outcome).
