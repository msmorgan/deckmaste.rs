---
needs: []
---
`Reference::Linked(Ident)` resolution. Needs a linked-ability information store
([CR#607]) — an ability that refers to what an earlier linked ability did or
which objects it affected (e.g. exile-then-return pairs). Seamed by
`engine-resolve-selections`: `eval_reference` in
`crates/deckmaste_engine/src/resolve/query.rs` currently turns an unlinked read
into a null reference because the linked-ability store is not wired.
