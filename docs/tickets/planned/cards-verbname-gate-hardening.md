---
needs: []
---
Finish recovering the safety the closed `CauseVerb` enum gave before it
became the open `VerbName` newtype. The corpus membership gate
(`cause_verbs_are_entailment_rows`, no_dead_grammar.rs) now tolerates RON
field reorder / anonymous-struct / quoted spellings (fixed 2026-07), but
three gaps remain:

1. **Scan scope.** The gate scans `plugins/{canon,testing,builtin}` only —
   any future plugin dir (noncanon-style) escapes the typo check entirely.
   Derive the scanned set from the workspace's actual plugin roots instead
   of a hard-coded list.

2. **No liveness direction.** Retiring the old `CauseVerb` allowlist lost
   per-verb liveness: the gate checks used ⊆ known, but nothing flags known
   verbs that are never used — the `Play` and `Explore` entailment rows are
   now permanently dead data no test can notice. Add the reverse check
   (known ⊆ used ∪ explicit-deferred-with-reason), i.e. the documented
   DEFERRED markers the old allowlist carried.

3. **Three hand-spelled verb vocabularies.** There is no canonical
   `KeywordAction → VerbName` accessor: the emit literals
   (`resolve/action.rs` `("Scry", ..)` etc.), the turn-based emit
   (`step.rs` `VerbName::from("Draw")`), and the match literals
   (`eval.rs` `named("Scry")` …) are aligned only by convention — a typo'd
   literal on a future atom compiles fine and its trigger silently never
   matches. Add `KeywordAction::verb_name()` (and a pattern-side twin) and
   route all three sites through it.

Also note the `VerbName` doc claim "each verb's fact form is one CR-cited row
of the emitted entailment table" is false for the Act-namespace verbs
(Scry/Surveil/Fateseal/Draw have no entailments row) — either emit rows for
them or scope the claim.
