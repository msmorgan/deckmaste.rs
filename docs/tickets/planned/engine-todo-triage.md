---
needs: []
---
**Classify the engine's `todo!()`/`unimplemented!()` population and turn it
into an explicit failure policy.** There are 76 grep hits in
`crates/deckmaste_engine/src` as of 2026-08-03; regenerate at claim time and
separate production sites from inline-test fixtures. Sixteen are the old
`todo!("P0.…")` SEAM backlog named in the tickets README.

The inventory must distinguish four cases that require different behavior:

1. **Malformed authored reference or missing authored object.** Fizzle only
   when the site falls under `docs/decisions/invalid-authoring-fizzles.md`:
   resolve to no applicable object, emit no game facts, and retain a
   diagnostic suitable for validation/logging.
2. **Valid but unsupported Magic mechanic.** Keep a loud, mechanic-specific
   diagnostic until its named implementation ticket lands. Do not silently
   turn a legal card behavior into a no-op.
3. **Internal engine invariant.** Keep or strengthen the assertion/panic and
   state the invariant it protects; these are explicitly outside the fizzle
   decision.
4. **Small implementation.** Mint a focused ticket, or record why a genuinely
   atomic correction is better handled ad hoc. Do not let opportunistic
   implementation make this inventory unbounded.

Examples to classify include `legal.rs:89` (unevaluated deontic legality),
`target.rs:409` (phased-in status), and `replace.rs:142` (uninterpreted
enters-replacement effect). Existing tickets such as
`review-low-severity-followups` and `engine-unbound-eventobject-panics` retain
ownership of their named instances; link rather than duplicate them.

Deliverables:

- a complete table in this ticket's eventual done record: location,
  reachability, class, governing decision/rule, and owner ticket;
- focused child tickets for every class-2 site and every non-ad-hoc class-4
  site, with no anonymous `P0.Wn` owner left;
- one engine-crate policy paragraph distinguishing invalid-authoring fizzle,
  unsupported mechanics, and invariant failure; and
- a final grep proving every remaining production `todo!`/`unimplemented!`
  has a durable diagnostic and a ticket slug.

This ticket performs classification and policy work, not a bulk behavior
rewrite.
