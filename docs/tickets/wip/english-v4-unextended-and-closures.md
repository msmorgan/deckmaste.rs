---
needs: []
---
# One mechanism per meaning of `unextended`; retire the chart kills that ranking makes redundant

## Owner's ruling (round 8, treatment item 3, Nodule 2)

Stop overloading `unextended` with three meanings. Replace or retire the
`unextended_first` and `fused_head` chart kills wherever ranking already puts
the intended Reading in the cheapest tier, which also lets the full re-parse
fallback in `admit_as` (`reading.rs`) be deleted. Admission stops only
analyses no English grammar admits (`docs/decisions/english-v4.md` §4.4).

## Evidence

Re-grepped on `qynmxmro`. Line numbers drift.

`ProductionDecl::unextended` (`crates/deckmaste_syntax/src/grammar.rs:109–122`)
has three meanings. `Compiler::unextended` (`compile.rs:774–842`) picks one by
the child's position:

| Meaning | How it is applied | Users (`english.ron`) |
|---|---|---|
| A child right before a selected `PP` Complement does not end in one of that Complement's prepositions | at chart time, through `Summary::final_preposition` (`step.rs:485`), `State::final_preposition` (`step.rs:582`), `track_final_preposition` (`step.rs:718–744`) and `CompiledGrammar::unextended_prepositions` (`compile.rs:101–103`) | `LocatedExistentialBe` (`:594–595`), the Displaced Subject before its Extension |
| A last child does not end in a `PP` that a sibling Production, the same but for a trailing selected `PP`, takes as its Complement | the same chart-time machinery | `ExistentialBe` (`:592–593`) |
| A first child followed by a Constituent does not end where one Constituent of its Category spans it and the next child (`unextended_first`, `step.rs:189–192`) | a chart closure, `CompiledGrammar::unextended_items` (`closure.rs:100–121`), registered in `chart_closed` (`compile.rs:104–106, 806–807`) | `PreposedPrepositionAndTemporal` (`:772`), `PreposedPrepositionPair` (`:783`) |

The other chart kill is `fused_head` (`grammar.rs:123–130`; `compile.rs:844–873`;
`step.rs:193–196` `fused_twin`). `headed_twins` (`closure.rs:123–166`) removes
the fused-head Constituent wherever its twin is followed by a child named
`head`. Its one user is `FusedSubjectDeterminerHead` (`english.ron:1154`;
*during [its controller's next untap step]*). It was landed in `pwyvozyw`.

`admit_as` (`reading.rs:790–840`) re-parses a Reading's realization, through
`self.run`, when the Reading uses a `chart_closed` Production
(`chart_closed_production`, `:844–852`). The same re-parse also decides
lexical atomicity: whether a declared multiword noun's words straddle a
constituent boundary (english-lexical-analysis §2, "Settled analyses"). So
retiring the closures removes only one of the fallback's two callers.

Settled analyses that bear on this:

- The existential rule is a settled analysis, not a canonicalization: "A
  locative or temporal Preposition Phrase right after an existential's
  Displaced Subject is its Extension … never also the Displaced Subject's
  postmodifier or an Adjunct" (english-lexical-analysis §2). It stays. Its
  mechanism changes.
- The six structural canonicalizations in english-lexical-analysis §2 are
  commuting modifiers, same-coordinator grouping, imperative clause vs VP
  coordination, same-head complement priority, adjuncts below a non-modal
  auxiliary, and fronted dependent outermost. They are not chart kills and
  stay, unless this ticket shows one of them is.

Tests pinning the current mechanisms: `crates/deckmaste_syntax/tests/unextended.rs`
(3 tests) and `crates/deckmaste_syntax/tests/chart_closures.rs` (5 tests).
Judgments exercising them include `combat_intervals.txt:2–3`,
`preposed_preposition_pair.txt:2–3`, `trigger_clauses.txt:17`,
`subordinate_clauses.txt:29` (North Star), `keyword_pp.txt:12`,
`coordination.txt:8`, `genitive_heads.txt:5` and `overgeneration_prune.txt:10`.

## Change

1. **Existential (meanings 1 and 2).** Give the Extension rule its own named
   mechanism, stated as what it is: the Displaced Subject does not end in a
   Preposition Phrase that its Production, or a sibling Production, selects as
   the Extension. One declaration replaces both positional readings of
   `unextended`. It reuses the existing `final_preposition` tracking: no new
   `Summary`, `State` or `ProductionDecl` field.
2. **Preposed PP (meaning 3) and fused head.** For each Production, disable
   the closure and measure. Report per unit whether the intended Reading is in
   the cheapest tier without it.
   - Where it is, retire the closure. The other Readings stay as costlier
     alternatives, and a Reading count may rise.
   - Where it is not, first look for a structural Ranking Key fix (§4.3).
     Keep the closure only if the Reading it removes is one no English grammar
     admits, and say why, with a CGEL or `docs/oracle-style-guide.md` citation.
     Ranking a closure's Reading behind another is not a reason to keep it.
3. **`closure.rs`.** Delete each pass whose last user is retired. Delete the
   file and `chart_closed` once none is left.
4. **`admit_as`.** Delete the chart-closure branch of the re-parse fallback
   once nothing is `chart_closed`. Then decide lexical atomicity without a full
   re-parse if that can be done structurally. If it cannot, record why: the
   fallback then stays for atomicity alone, and its doc comment says so.
5. Rewrite or retire the tests above to match, by name, each with its reason.
   Update the `deckmaste_syntax` README (`:280–300`) and the glossary entry
   **Chart Closure** (`docs/contexts/oracle-english/CONTEXT.md`) if the term no
   longer names anything.

## Done when

- `unextended` has no positional overloading: each remaining meaning has its
  own declaration and doc comment.
- Each chart kill is retired or justified in the landing as removing a Reading
  no English grammar admits. Each retirement has its per-unit measurement.
- The full re-parse in `admit_as` is gone, or kept for lexical atomicity alone
  with its reason recorded.
- No face is lost.
- Every judgment holds.
- `cargo xtask english-v4 strata --check` passes.
- The landing reports the cheapest tier: ambiguous units with a tier of one,
  the tiers' sum and the largest tier, before and after. No judgment-set entry
  goes from an all-endorsed cheapest tier to one that is not.
- No new `Summary`, `State` or `ProductionDecl` field, and no new `closure.rs`
  pass.

Standard constraints apply.

Census delta (measured on `<change>` → this landing): faces read, total Readings and Surplus before and after; lost faces by name, or "none".
Judgment set: the result of running it.
