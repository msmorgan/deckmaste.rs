---
needs: [grammar-document-cleanup]
---
# Per-sentence enumeration: the re-baseline landing

Change the enumeration and validation unit of the census from the paragraph (document) to the sentence, and change nothing else in the same landing. Reading counts for a paragraph are today the product of its sentences' counts: Necromentia's sentences probe at 2, 50 and 361 readings and the paragraph has 36,100, which took 0.042 s of chart time and 13.7 s of validation. Across paragraphs the census sums. The product is an enumeration artifact, not a grammar property, and it hides where ambiguity lives (79 faces exceed 1,000 readings; the top 100 faces hold 49% of 724,921 readings).

## Change

- Enumerate and validate each sentence of a document on its own; the census records per-sentence counts and digests, and a face's count is derived from them.
- The covered and unread sets, and the lexical and chart results per sentence, stay as they are.
- Output stays under `target/english-v3/`; nothing the tool emits is tracked.

## Gate

This is the one re-baseline: reading digests are recomputed per sentence and so differ from the old ones by design. The check replacing digest identity is that for every paragraph, its old reading count equals the product of its new per-sentence counts, with the covered set and unread set identical. Nothing else may change in the landing.

## Report

Three-line note: census delta (faces, total Readings now counted per sentence; none lost); the size line and judgment-set line do not apply yet (instruments arrive in `grammar-size-report-and-ratchet` and `grammar-judgment-set`).
