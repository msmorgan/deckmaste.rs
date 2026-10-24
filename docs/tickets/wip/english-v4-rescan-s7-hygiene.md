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

## Findings

- **Light-Paws: not fixed.** Its 840 added Readings are four shapes of
  the Scorching Lava coordination. In only 168 of them is the last conjunct
  a Linked Coordination. So option (i), an uncoordinated last conjunct,
  leaves 672. Option (ii) has no CGEL support. Ch. 20 §3.2.1
  (pp. 1739–1740) sets no condition on the clause after a comma-less
  *and*, and a fronted *with*-PP Adjunct of an imperative is English. The
  Readings are grammatical, so they are left for semantics.
- **Heavy shift: not fixed.** CGEL states no clausal-intervener ban. On
  p. 280 it says "there are no relevant restrictions on what can come
  between the verb and an NP that is heavy". On p. 1383 relative weight
  governs felicity, not grammaticality. Excluding clausal Adjuncts would
  reject English.
- **De-hax 1: confirmed single holder.** `SpecifyingClause` takes a
  `Subtype` value. In the lexicon and corpus, only *type* has subtype names
  as its values. *name* takes card names, and *word* takes colors.
  *subtype* and *supertype* appear in neither. The CGEL Ch. 4 §5.5
  citation is the right one for specifying *be*.
- **De-hax 2: not changed.** The features hold: *fewest* is Count, *least*
  is Mass and *most* is unvalued. But `Require` would value *most* and
  fuse it in *tied for most*. `Carry` Count would pick out the same one
  member for the wrong reason: the class excludes the superlatives that
  are also degree words, not the mass ones.
- **De-hax 3: not merged.** On p. 428 (Ch. 5 §10.1.1), *them all* /
  *them both* are compound pronouns, and a full NP is impossible there
  (∗*They've invited the girls all*). Dream Cache's full-NP *both* before
  a locative is a separate construction, so no single rule covers both.
- **De-hax 4: done.** The profiler reads a verb with the auxiliaries when
  its lemma is that of a lexicon auxiliary (`VerbKind: Auxiliary`). That
  covers main-verb *do*, which shares its lemma with *don't*. The census
  is unchanged.
- **De-hax 5: confirmed.** `SumAdjective` is CGEL's postpositive sum
  adjective (Ch. 5 §14.2(b), pp. 445–446). *net* and *gross* are absent
  from the corpus.

## Done when

- Each item is fixed or reported as not principled, with the reason.
- Units that lose Readings lose only wrong or duplicate ones. No face is lost.

Standard constraints apply.
