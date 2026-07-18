---
needs: []
---
Chapter-ability structure for Sagas: I/II/III markers, ranges, and
read-ahead compatibility. Grammar prerequisite for [[engine-sagas]] and
the shape-saga layout.

Standard constraints apply. Baseline already in-tree: the `Chapter` ability
macro (`plugins/builtin/macros/ability/Chapter.ron`), `LoreCounter`, the
`Saga` subtype row, `Test Saga`, `Condition::Crossed`, and green engine
chapter firing (`saga_chapters_fire_on_crossed_thresholds_from_one_batch_fact`).
This ticket is the delta below.

## ① Chapter abilities are NOT `Innate`

A chapter ability is the card's printed, REMOVABLE ability — [CR#714.2]'s "a
chapter symbol is a keyword ability that represents a triggered ability", not
a rule of the object. [CR#714.2d] contemplates a Saga with no chapter
abilities (final chapter number 0), exactly what "loses all abilities"
produces. Drop the `Innate` wrapper: `Chapter` expands to a plain
`Expanded(Triggered(...))`, layer-6-removable. `Innate` stays only on the
`Saga` subtype-conferred rules (enters-with-lore-counter [CR#714.3a],
turn-based increment [CR#714.3c]). Update the `builtin.rs` expansion
assertion and the macro comment.

## ② Ranges via plural crossing

[CR#714.2c]: "{rN1}, {rN2}—[Effect]" fires the effect at each listed chapter
number. Real ranges are always contiguous (corpus: `I, II` / `II, III` /
`III, IV` / `II, III, IV` / `I, II, III, IV`). Generalize
`Condition::Crossed { value, threshold: CountBound }` →
`Crossed { value, thresholds: Vec<Count> }` — crossing is inherently "became
at least N" [CR#714.2b] and chapters are `Crossed`'s only consumer, always
`AtLeast`. `Chapter`'s `n` takes one-or-more numbers (`n: 1` single /
`n: [2, 3]` range), spliced into `thresholds`. The engine `Crossed` eval
iterates `thresholds`; a 1-element list fires identically to today, so the
single-chapter and Doubling-Season-batch tests stay green.

Crossing is load-bearing, not overkill: Doubling Season doubles lore
placement (official ruling — it affects permanents that enter with counters),
so a Saga you control enters with 2 lore and increments by 2, and each
chapter whose number the count "becomes equal to or greater than" triggers
(History of Benalia ruling, 2018-04-27); multiple chapters trigger from one
batch. The `before`/`after` fact channel `Crossed` reads is exactly what a
doubled 0→2 jump needs.

DEFERRED to [[engine-sagas]]: per-crossed-member firing when a single doubled
batch crosses TWO members of one range at once (History of Benalia + Doubling
Season → 2 Knights). Normal one-at-a-time play already fires each range
member correctly (separate placement events, [CR#714.2b] crossing per event).

## ③ Faithful render

`Chapter` currently renders nothing (its expansion falls through the render
catch-all). Add a render arm emitting `<roman thresholds> — <effect>`
("I — …", "II, III — …"): Roman markers from `thresholds`, effect rendered
structurally via `effect::effect`. Add an int→Roman helper plus a `:roman`
template codec (declines-to-structural like the existing `+` / `sing|plur`
codecs). Update the `render/mod.rs` deferred-Chapter note.

## ④ ReadAhead — deferred

Full ReadAhead ([CR#702.155,714.3b]) decomposition and firing is
[[engine-sagas]]. It will take the final chapter number as an authored
argument (a declarative macro cannot count the card's chapters) and decompose
to the as-enters "choose 1..N lore counters, enter with that many" ability;
that needs a bounded numeric-choice primitive (a 1..N bound on
`ChooseAndNote(_, Number)`) plus the [CR#702.155a] turn-of-entry exactly-N
trigger gate and the [CR#714.3a]-vs-[CR#714.3b] default-enter exclusion. This
ticket only keeps chapter numbers first-class in `thresholds` so
[[engine-sagas]] can derive final chapter number = max threshold.

## Tests

- `Chapter` expands to `Expanded(Triggered(...))` (no `Innate`); single
  `thresholds: [1]` and range `thresholds: [2, 3]`.
- Render: `Test Saga` chapters render "I — …" etc.; a range fixture renders
  "II, III — …".
- Existing engine chapter / Doubling-Season batch tests stay green.
