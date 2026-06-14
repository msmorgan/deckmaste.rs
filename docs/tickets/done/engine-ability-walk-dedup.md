---
needs: []
---
`legal.rs` carries two copies of the same `Static` / `Keyword(Composite)` / `Expanded`
ability-tree descent: `in_ability`/`in_keyword`/`in_static` as `Fn -> bool` "any"
(`legal.rs:34`, backing `object_has_static` / `statics_present`) and again as `FnMut`
"visit each" (`legal.rs:473`, `statics_on`). Keep one `statics_on(visitor)` walker and
make the boolean form a thin short-circuiting wrapper over it. Pure refactor.
