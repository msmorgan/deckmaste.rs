---
needs: []
---
**Bare keyword-ability lines (`Trample`, `Flying`) yield zero rule-bearing
`ParseSelection`s — no constituent spans at all.** Every instrument built on
`ParseReport::provenance().selections()` is therefore blind to keyword-only
text: it sees no rule, no span, no tie, nothing, for a line that parsed
successfully. This affects `english bracket` and the new
`english adversarial` tool at minimum, and possibly others that walk
`selections()`.

Verified 2026-07-29: `cargo run -q --release -p xtask -- english bracket
"Siege Rhino" 2>/dev/null` (sibling `english-adversarial-corpus` workspace,
not yet integrated) prints:

```
<Trample>
<When <<<this> <creature>> <enters>>, <<<<<each> <opponent>> <<loses> <<3> <life>>>> and <<you> <<gain> <<3> <life>>>>>.>>
```

The `Trample` line renders as one flat, unstructured bracket — no internal
nesting — while the following ability sentence is fully decomposed into
nested constituent brackets down to individual words. The contrast is the
evidence: `bracket` renders exactly what `selections()` reports, and for the
keyword line it reports nothing to bracket.

Standard constraints apply.

## Completion

- Confirmed bare keywords are atomic catalog items and `<Trample>` is their complete structure; no synthetic grammar provenance landed, and keyword mining remains an adversarial-instrument concern.
