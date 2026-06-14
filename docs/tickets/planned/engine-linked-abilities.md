---
needs: []
---
`Reference::Linked(Ident)` resolution. Needs a linked-ability information store
([CR#607]) — an ability that refers to what an earlier linked ability did or
which objects it affected (e.g. exile-then-return pairs). Seamed by
`engine-resolve-selections` (the `todo!` in `eval_reference`, resolve.rs).
