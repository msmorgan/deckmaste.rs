---
needs: []
---

# english-v4: re-scan S7 hygiene

Queued fixes from the reviewer's diagnoses of 2026-10-24. Each item is a
hypothesis; verify it on its witness before acting on it. A fix that is an
exclusion aimed at one witness is not made: report it and keep the reading.

## Change

- **Light-Paws.** The Scorching Lava Clause coordination (finite first
  conjunct, `last.FrontPosition` Occupied) reads Light-Paws' *with*-PP as a
  comma-set fronted Adjunct of *put* (750 → 1,590 Readings, from
  `znmzqpzo`). Restrict it structurally: preferably the last conjunct is
  uncoordinated (Light-Paws' is a clause chain), else the fronted dependent
  is clausal, only with CGEL support.
- **Heavy shift past a finite-clause Adjunct.** *Activate only if two
  creatures entered the battlefield under your control.* reads *Activate [the
  battlefield under your control] [only if …]* (~22 units, from
  `spkrozto`). Heavy-NP shift does not cross a finite or verbless
  subordinate clause (CGEL Ch. 16 §4, pp. 1382–1384): exclude clausal
  Adjuncts from the interleaved site before a shifted Object.
- **De-hax 1, `Variable: Cited`.** One holder (`noun:type`). Give the value
  to the metalinguistic and taxonomy variable nouns that take a cited value
  in a specifying clause (CGEL Ch. 4 §5.5); Artificial Evolution's zero
  growth holds. Leave *name* out if it duplicates *named [Name]* trees.
- **De-hax 2, `CountSuperlativeQuantifier`.** One member (`det:fewest`).
  Replace it with `SuperlativeQuantifier` plus a Count requirement if the
  Countability features support it; *tied for most* stays unfused.
- **De-hax 3, floated *both* / *all*.** `FloatedObjectBoth` (Dream Cache)
  and `FloatedObjectAll` (Strixhaven Stadium) are one-card productions.
  Merge them only into a CGEL-grounded general construction that adds no
  trees and keeps `* Remove the counters all` out.
- **De-hax 4, `profile.rs` auxiliaries.** The profiler names the lemma *do*
  and the file `closed_class.ron` in Rust. Move auxiliaryhood to a lexical
  property read by the profiler; the census stays unchanged.
- **De-hax 5, `SumAdjective`.** One holder (*total*). Confirm it is a real
  single-holder construction.

## Done when

- Each item is fixed or reported as not principled, with the reason.
- Units that lose Readings lose only wrong or duplicate ones. No face is lost.

Standard constraints apply.
