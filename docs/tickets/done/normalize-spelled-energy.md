---
needs: []
---
Graduate the spelled-number energy cards ("Pay **six** {E}", "**eight** {E}",
"**fifty** {E}") by normalizing spelled-number-`{E}` into a `{E}`-run so the
`PayEnergy`/`GainEnergy` macros (which match a run of `{E}` glyphs) parse them.
~124 `{E}` cards remain `.ron.todo` after `macro-payenergy-symbol-repeat`; the
spelled-number subset is one blocker (others: dynamic "for each", crew, ETB
triggers, "add one mana of any color"). Corpus convention: ≤5 energy prints as
repeated `{E}` glyphs, >5 spells the number + a single `{E}` (unreadable to
count as glyphs past five).

## Two-layer normalization (both required)
The transform must be a SHARED spelled-number-`{E}` → `{E}`-run rewrite applied in
both places (they're separate functions today — factor one shared helper):
1. **Extract/parse** — `crates/deckmaste_migrations/src/extract.rs` (beside
   `academyruins::normalize_quotes`): rewrite the oracle line BEFORE it becomes
   `Unparsed(...)`, so `"Pay six {E}"` → `"Pay {E}{E}{E}{E}{E}{E}"` and the
   `${0*\{E\}}` matcher folds it into `PayEnergy(6)`. Without this the card never
   graduates.
2. **`fidelity::normalize`** (`crates/deckmaste_plugin/src/fidelity.rs:337`, runs
   on BOTH rendered and oracle lines before the diff) — so the diff matches
   regardless of whether the render emits glyphs or the spelled form.

Rewrite spec: `<spelled-number> {E}` → that many `{E}`, ONLY when a single `{E}`
follows the number (don't touch "six cards", "eight damage", etc.). Cover the
spelled numbers that actually appear (one..fifty+ — audit the corpus). Bounded to
the `{E}` symbol.

## Render threshold (readability)
Because `fidelity::normalize` collapses both sides, a large `PayEnergy(N)` would
PASS the gate rendered as N literal glyphs — but the card would read as e.g.
fifty `{E}` symbols. Add a `>5` render threshold to the `PayEnergy`/`GainEnergy`
render (and/or the `${0*\{E\}}` construct): N≤5 → `{E}`×N, N>5 → spelled "N {E}"
(match the oracle convention). Separable from the normalization but wanted for
readable graduated cards.

## Verify
Report how many previously-`.todo` energy cards now graduate (the spelled-number
subset), that fidelity + idris-check pass on them, and that the render threshold
produces the spelled form for N>5. Cards still blocked by OTHER mechanics stay
out of scope. Surfaced by `macro-payenergy-symbol-repeat`.
