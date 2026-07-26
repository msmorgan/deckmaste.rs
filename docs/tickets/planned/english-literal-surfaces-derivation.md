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
support per-slot opt-out with a stated reason, not blanket enrollment. See
`recovery-harness/out/litaudit-plan.md` §2 and `litaudit-mechanic-report.md`
§3.
