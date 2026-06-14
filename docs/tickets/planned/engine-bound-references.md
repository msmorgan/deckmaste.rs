---
needs: []
---
`Reference::Bound(Ident)` resolution. Needs a named-role binding store: roles
bound by an event pattern or instruction during resolution ([CR#608.2]), read
back by later instructions of the same effect. Seamed by
`engine-resolve-selections` (the `todo!` in `eval_reference`, resolve.rs).
