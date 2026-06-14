---
needs: []
---
`MacroSet::insert`/`replace` (`set.rs:439`/`493`) and `declare`/`redeclare`
(`set.rs:472`/`511`) are copy-paste pairs differing only in the duplicate-key check;
back both with a private `register(def, allow_overwrite)` plus a `decl_def(...)` builder.
Minor: the boxed-payload construct closure in the derive is duplicated between
`embed_construct` (`generate.rs:101`) and `fall_throughs` (`generate.rs:347`) — share a
`newtype_construct` helper. Pure refactor.
