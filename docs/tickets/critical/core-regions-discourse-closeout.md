---
needs: []
---
**Finish the discourse stage's architectural half: no nearest-antecedent
search at runtime, no shared choice slot, real lowering diagnostics.** The
stage-2 landing (`core: complete discourse regions`, 2026-09-02) delivered
the instruction-with-dest vocabulary, region-shaped loops and predicates,
per-region-kind params, and zero literal register reads in engine
production code. Four contract items did not land. Standard constraints
apply.

## Scope

1. **Runtime R1 survives.** `engine/src/activation.rs` grew
   `activation_latest_object` and `activation_latest_number`, which scan
   the register file backwards for the newest value of a compatible kind.
   That is nearest-antecedent resolution, the thing
   [Core is explicit regions](../../decisions/core-explicit-regions.md)
   moves to lowering, reintroduced positionally. Three production callers:
   the shield subject and the produced-`That` snapshot in
   `engine/src/resolve/effect.rs`, and ward-{X} pricing in the same file.
   Lowering assigns each a param or def; the scan and both helpers delete.
   This is also why the zero-literal-`RefId` gate passes while these paths
   still read by position.
2. **The shared `chosen` slot survives** with its clearing discipline, in
   `engine/src/activation.rs` and written from `engine/src/payment/
   fulfill.rs`, read behind a presence check in `engine/src/resolve/
   effect.rs` and `engine/src/resolve/query.rs`. Absorbed ticket
   `engine-chosen-slot-provenance` exists to delete exactly this: the
   deciding instruction writes its own dest, and no reader tests a
   presence flag. Its fixture (a chooser nested under another chooser,
   `Random`, or `AmongNoted`) is owned by
   [[core-regions-test-restoration]].
3. **The `noted`/`noting` collection** is still written by `BeginNote` and
   `EndNote` with no remaining reader. Delete it or give it its reader.
4. **Lowering diagnostics are a bare assertion.** R1/R2 refusal panics
   through `assert!`/`expect` with no card name. Stage 2's ticket requires
   a per-card diagnostic carrying card context, and the ADR makes
   ambiguity a compilation error with provenance, not an eval-time
   refusal.
5. **The resolver/certifier differential gate does not exist.** The ADR's
   law 12 and `semantics-spelling-lowering.md` §17 pair lowering's
   computed resolution against the Idris mirror's certification over the
   canon slice. No xtask target compares them. Build it, or record in the
   ADR why the pairing is dropped.

6. **The magnitude anaphor never gets nearest-wins.** A card fixing two
   magnitudes and then reading a bare "that much" is refused by the
   uniqueness gate at lowering rather than resolving to the nearer
   antecedent, because the amount channel is the only anaphor channel
   with no site preference. The absorbed `engine-that-much-frame-scoped`
   ticket's wording, that such a card reads the semantic antecedent,
   implies nearest-wins was intended. The restoration landing pinned the
   refusal as the actual behaviour rather than changing the resolver, so
   this is a live contradiction between a pinned test and an absorbed
   ticket's intent. Settle it: either the amount channel gains a site
   preference and the pin flips, or the refusal is correct and the
   absorbed wording was wrong, recorded here.

## Gates

Standard constraints apply. A grep proves no backwards register scan
survives in `deckmaste_engine`, and no reader tests a `chosen` presence
flag. A deliberately ambiguous fixture card produces a lowering
diagnostic naming the card, not a panic. The differential gate runs over
canon and reports zero disagreements.
