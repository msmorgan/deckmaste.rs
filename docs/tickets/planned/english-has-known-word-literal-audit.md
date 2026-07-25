---
needs: []
---
**Audit `has_known_word` against the full literal-lexeme inventory.** The
opacity scanner's invariant predicate `has_known_word`
(`crates/deckmaste_english/src/grammar/mod.rs`) consults the vocabulary/verb/
catalog slots plus hand-written literals — originally only `plus`/`who`;
round pluralposs (2026-07-25) added `except`/`not` after both were swallowed
as opaque noun heads, completing wrong-but-round-tripping parses. The same
omission may exist for every other literal-keyed `EnglishLexicalSlot` (e.g.
`RatherThan`, existential `there`, and any literal scanned via
`one_token_match` outside the slots the predicate checks). Work: enumerate
`EnglishLexicalSlot` variants and every `one_token_match` literal against the
predicate's coverage; add the missing arms (invariant repair, no cost
changes); measure with a two-sided unknown-dump diff — faces that only parsed
by opacifying a known literal will surface as honest recoveries and must be
per-row attributed (pluralposs exposed 41 such faces for `except`/`not`
alone). Consider deriving the arm list from the scan table instead of
hand-maintaining parallel literals, so the invariant cannot silently drift
again.
