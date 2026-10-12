---
needs: [grammar-document-cleanup]
---
# Per-sentence enumeration: the re-baseline landing

**Not claimable until the owner has approved the written decomposition definition below and it has been pasted into this ticket.** The definition is PENDING.

Change the enumeration and validation unit of the census from the document segment to the sentence, and change nothing else in the same landing. Reading counts are today the product of the counts of a document's segments: Necromentia's sentences probe at 2, 50 and 361 readings and the paragraph has 36,100, which took 0.042 s of chart time and 13.7 s of validation. The product is an enumeration artifact, not a grammar property, and it hides where ambiguity lives (79 faces exceed 1,000 readings; the top 100 faces hold 49% of 724,921 readings).

## What the census does today

- It segments a document at Ability, not at Paragraph: `census_segments` (`crates/xtask/src/english_v3/mod.rs:331`) takes `.segments(&Category::Ability)` of the forest and reports `Category::Ability`.
- A face's count is the sum of its lines' counts, and zero if any line is unread (`crates/xtask/src/english_v3/report.rs:229`).
- Sentences are not the only material under an Ability: an activated ability carries a separate `Cost` (`crates/deckmaste_english_v3/src/declarations.rs:2675`), a keyword ability need not contain a Sentence, and a modal item has a Clause header and nested mode paragraphs (`Mode` at `declarations.rs:2987`). Empty documents and type lines already have explicit exceptions.
- `Forest::segments` unions alternatives at identical ordered spans (`crates/deckmaste_english_v3/src/forest.rs:179`); it does not prove that the readings of a unit combine as an independent Cartesian product.

## Definition to be approved (PENDING)

A short written definition, approved by the owner and pasted here before this ticket is claimable, must settle:

- the units of decomposition and how a unit is found in the forest;
- the residual non-sentence units (costs, keyword abilities, modal headers, nested mode paragraphs) and how each is enumerated and validated;
- the nested-document boundary, for quoted abilities and mode paragraphs inside a sentence;
- face aggregation, including the rule for an unread unit;
- where the owner's product equality applies, stated precisely, and what is done where the product does not hold; an extension of the equality goes back to the owner, it is not weakened here;
- report and digest versioning, so an old report is refused rather than compared;
- migration of the incremental cache;
- that admission stays whole-root: every unit is also validated inside its surrounding costs, headers and wrappers.

## Change

- Enumerate and validate the units the definition names; the census records per-unit counts and digests, and a face's count is derived from them.
- The covered and unread sets, and the lexical and chart results, stay as they are.
- Output stays under `target/english-v3/`; nothing the tool emits is tracked.
- The landing updates the census description in `crates/deckmaste_english_v3/README.md`.

## Gate

This is the one re-baseline: reading digests are recomputed per unit and so differ from the old ones by design. The check replacing digest identity is the product equality as the definition states it, with the covered set and unread set identical. Nothing else may change in the landing.

## Report

Three-line note: census delta (faces, total Readings now counted per unit; none lost); the size line and judgment-set line do not apply yet (instruments arrive in `grammar-size-report-and-ratchet` and `grammar-judgment-set`).
