# deckmaste RON language server

An early language server for deckmaste's macro-aware RON dialect. The first
slice supports go-to-definition from identifiers in an open RON document to:

- matching macro definitions below `plugins/*/macros/**/*.ron`; and
- matching Rust structs, enums, and enum variants in `deckmaste_core`.

Document highlights understand the lexical `It` scopes introduced by `Each`,
`Distribute`, `RevealUntil`, and `Where`. Selecting either the binder name or a
bound `It` highlights the source and all uses in that scope; nested binders
shadow outer ones.

Run it over stdio from the repository root:

```sh
cargo run -p ron_lsp
```

Configure an editor LSP client to launch that command for both `*.ron` and
`*.ron.todo`. The server uses the client's workspace root to build its index.

The index intentionally returns every matching definition for now. Macro names
are kind-scoped (for example, `AnyTarget` is both a `Predicate` and a
`TargetSpec` macro); expected-kind inference will narrow those results in a
later slice.
