---
needs: []
---
`Reference::Bound(Ident)` resolution. Needs a named-role binding store: roles
bound by an event pattern or instruction during resolution ([CR#608.2]), read
back by later instructions of the same effect. Seamed by
`engine-resolve-selections`: `eval_reference` in
`crates/deckmaste_engine/src/resolve/query.rs` currently turns an unbound read
into a null reference because the named-role store is not wired.
