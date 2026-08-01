---
needs: []
---
**Make the opacity-reserved-literal invariant compiler-forced.** Round
litaudit (2026-07-25) consolidated `has_known_word`'s literal coverage into
`OPACITY_RESERVED_LITERALS` (grammar/mod.rs) — a single predicate-side source
of truth pinned by `every_reserved_literal_is_a_known_word` — but the
genuinely drift-proof form is an exhaustive
`const fn literal_surfaces(slot: EnglishLexicalSlot) -> &'static [&'static str]`
consumed by BOTH the ~20 literal dispatch arms in the big `match slot`
scanner AND the predicate, so adding a literal-scanned slot without declaring
its surface fails to compile. Deferred from litaudit because rewriting the
dispatch arms (each with per-arm feature/meaning shapes) is a behavior-risk
surface disproportionate to a repair round, and declaring the helper without
the dispatch consuming it merely relocates the drift. Caution from litaudit:
membership is semantic, not mechanical — `out` must NOT be reserved (an
existing test depends on its opaque-noun fallback for unlicensed verb pairs)
and `you've` has second-order `retain` blast radius — so the derivation must
support per-slot opt-out with a stated reason, not blanket enrollment.

## Completion

`EnglishLexicalSlot::literal_surfaces` is now the exhaustive fixed-surface
declaration for every lexical slot. Exact-token, ordered-sequence, alternative,
combat-step, coin-result, contracted-subject, and existential scanners consume
that declaration rather than repeating their spellings. Because the function's
nonliteral arm names every remaining enum variant, adding a new lexical slot
requires an explicit fixed-surface decision at compile time.

`has_known_word` now walks the same typed literal slots and surfaces; the old
predicate-only `OPACITY_RESERVED_LITERALS` string table is gone. Per-surface
reservation metadata preserves the measured litaudit boundary exactly:
`out`, `both`, `half`, `rounded`, `rather`, `there's`, and the unaudited
subject contractions remain explicit opacity-visible opt-outs, while the
previously measured surfaces remain reserved. Tests enumerate every declared
literal slot, verify every reserved surface is known, and pin the opt-outs.

The supported-corpus census is byte-for-byte unchanged: 3,480 structural
recovery spans / 64,217 source tokens, 792 noun-opacity occurrences, and 622
flavor-header-opacity occurrences.

All 574 library tests and 112 public-API tests pass. The supported corpus
round-trips 31,685/31,685 clean with zero mismatches or render errors. Clippy
reports only the repository's existing unrelated warnings. The citation audit
reaches only its two pre-existing malformed examples in the planned
comment-discipline ticket; this change adds no citation site.
