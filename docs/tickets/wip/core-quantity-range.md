---
needs: []
---
**Core: collapse `Quantity` to `Range(Option<Count>, Option<Count>)`, with
`Exactly` / `AtLeast` / `AtMost` / `Between` / `AnyNumber` as builtin macros.**
From the 2026-06-28 idris↔rust model audit follow-up. Reverses the audit's
"Quantity Range-ification — negative" note.

Today `Quantity` (`crates/deckmaste_core/src/quantity.rs`) is five variants —
`Exactly(Count)`, `AtLeast(Count)`, `AtMost(Count)`, `Between(Count, Count)`,
`AnyNumber`. Every consumer that needs the lower/upper bound (target counts,
choice counts, "does this permit ≥1") matches all five and re-derives the bounds.

Idris models it as one `Range (Maybe Count) (Maybe Count)` (`idris/src/Core.idr`).
The audit first marked porting this **negative**, purely because
`Range(Some(1), Some(1))` reads worse than `Exactly(1)`. That objection
dissolves once the named forms are *macros* over the single primitive — the macro
layer exists precisely to put readable names on canonical shapes (cf. the builtin
`Creature`, `AnyTarget`, `Dies`).

Proposed:

1. Change `Quantity` to `Range(Option<Count>, Option<Count>)` and add an
   `Expanded(Expansion<Quantity>)` variant; register `Quantity` as a
   **remembering** macro kind (it is already a registered kind in
   `crates/deckmaste_core/src/ron.rs` — this adds the remember policy). Without
   the remember policy an invocation would serialize back as the ugly `Range(…)`.
2. Add builtin `Quantity`-position macros, each with a rules-text template:
   `Exactly(Count) → Range(Some, Some)` (same arg in both bounds),
   `AtLeast(Count) → Range(Some, None)`, `AtMost(Count) → Range(None, Some)`,
   `Between(Count, Count) → Range(Some, Some)`, `AnyNumber → Range(None, None)`.
3. Update engine consumers to read the `(lo, hi)` pair — lower/upper bound become
   field reads instead of a five-way match.

Surface RON is **unchanged** (`Exactly(1)`, `AnyNumber`, `AtMost(2)` parse and
round-trip identically), so existing cards don't churn — only the Rust type, the
parser path, and consumers change.

Soundness note: the named variants give no structural guarantee the `Range` form
loses — inverted bounds (`Between(5, 2)` ≡ `Range(Some 5, Some 2)`) and a
degenerate "up to 0" are equally constructible either way, so ordered/non-zero
validity stays a runtime check (the Idris `OrderedRange`/`NonZeroQ` proofs erase).
No regression. Caveats: `Exactly`/`Between` substitute their count into both
bound slots (a literal or deterministic count reads identically in both); and
`Expanded` equality is provenance-sensitive (a hand-built `Range` won't compare
equal to an `Exactly(…)` invocation — true of every remembering kind).

Verdict: **neutral → mild improvement** (one canonical bound representation,
simpler consumers, identical surface RON, no card migration). Effort: **S–M**
(the type + `Expanded`/remember wiring + 5 builtin macros + consumers; no card
churn). Related: same "macros recover readability over a canonical primitive"
pattern as `core-modification-ops`.
