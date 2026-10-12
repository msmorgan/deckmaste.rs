---
needs: [grammar-size-report-and-ratchet]
---
# Analysis key in the census

Give every reading a second identity, its analysis key, and record it in the census. The tree stays the Reading; the key says which head each dependent attaches to, and in what function. This is the instrument that decides what a later change may remove: the approved cases (order of applying modifiers to one head, grouping within a same-coordinator chain, clause versus verb-phrase coordination with nothing to tell them apart) are those where trees differ and the key does not.

## Change

- Define the key and compute it for each enumerated reading: per word, its lexeme and inflected form; per dependent, its head and its function. Write the definition down in the ticket's landing so the owner can review it. It must not erase differences that are not approved for removal: a differing lexeme or form, a different head, a different function stay distinct keys, so "Destroy target creature with flying." keeps its two readings and "cast" keeps present and preterite.
- The census records, per line, the number of trees and the number of distinct keys, and `diff` reports, per line, the set of keys before and after.
- One census run then gives the answer to how much of the 724,921 readings is derivational artifact. Report it.
- Output stays under `target/english-v3/`.

## Gate

Reading digests identical on every face. Report: census delta (none); size delta per declaration kind from the size tool; the judgment-set line does not apply yet.
