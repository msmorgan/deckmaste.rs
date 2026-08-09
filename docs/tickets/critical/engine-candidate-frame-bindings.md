---
needs: [engine-candidate-frame-context]
---
**Engine: thread the full resolution frame through candidate matching.** The
completed `engine-candidate-frame-context` audit downgraded non-watcher
`Predicate::Ref` to a debug assertion after finding its two sampled corpus
shapes unreachable. The Foundations audit falsifies that invariant: ordinary
per-player choices, target-relative filters, trigger-relative targets, and
whole-zone selections reach it with faithful core terms.

Replace the watcher-only matcher context with enough of `Frame` to resolve
`Target(n)`, `It`, `That`, `EventActor`, and `EventPatient` while preserving
candidate-relative meaning. Carry it through chooser enumeration,
`SelectAll`, target announcement/recheck, and any history-query caller that
uses the same matcher. A missing semantic binding may fizzle under the existing
invalid-input policy; a binding present in the caller must never be discarded
into debug-panic/release-false behavior.

Foundations witnesses span two families:

- player/object-relative selection: Angel of Finality; Arbiter of Woe;
  Blasphemous Edict; Bloodtithe Collector; Burglar Rat; Deadly Brew; Duress;
  Liliana, Dreadhorde General; Painful Quandary; Perforating Artist; Pilfer;
  River’s Rebuke; Tribute to Hunger; and Tinybones, Bauble Burglar; and
- cross-target/event filters: Fiery Annihilation; Run Away Together; Steel
  Hellkite; Trygon Predator.

Add representative tests for every binding kind and caller family, in debug and
release-equivalent behavior. Remove or narrow the old “no corpus filter does
this today” assertion and record this Foundations counterexample in the
closeout.
