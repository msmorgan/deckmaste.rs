# deckmaste RON language server

A language server for deckmaste's macro-aware RON dialect (`*.ron`, `*.ron.todo`),
built on `lsp-server` + `lsp-types`. It backs both editor LSP clients and Claude
Code's built-in `LSP` tool.

## Capabilities

| LSP request | What it does |
|---|---|
| `textDocument/definition` | Jump from an identifier to matching macro definitions (`plugins/*/macros/**/*.ron`), card files, keyword/ability-word definitions, and Rust structs/enums/variants in `deckmaste_core`. |
| `textDocument/references` | All use sites of a macro / card / type across card and macro files. The reverse index is built lazily on the first request and cached, and tracks only known-symbol identifiers, so it stays bounded across a plugin's tens of thousands of cards. |
| `textDocument/hover` | For a macro, its `template` and `kinds`; for a card / keyword / type, its kind and container. |
| `textDocument/documentSymbol` | An outline of the open card: the card `name` as the root, one child per top-level `abilities` entry. |
| `workspace/symbol` | Fuzzy search over every indexed symbol by **name or container**. Because each symbol carries a `containerName` of `"<plugin>/<category>"`, a query like `wizards/cards` (or just `wizards`) **lists that plugin's contents**. |
| `textDocument/documentHighlight` | The lexical `It` scopes introduced by `Each`, `Distribute`, `RevealUntil`, and `Where`. Selecting the binder or a bound `It` highlights the source and all uses in that scope; nested binders shadow outer ones. |

Indexing is cheap at startup: cards are indexed by **filename** only (no file
reads), while the few hundred macro / keyword / ability-word / Rust files are
content-scanned. `goToImplementation` and call hierarchy are intentionally not
advertised — the card DSL has no matching notion.

## Running standalone

Over stdio, from the repository root (or any jj feature workspace):

```sh
cargo run -p ron_lsp
```

Configure an editor LSP client to launch that command for both `*.ron` and
`*.ron.todo`. The server builds its index from the client's workspace root.

## Claude Code integration

`scripts/ron-lsp` launches the server (`cargo run -q -p ron_lsp`, manifest pinned
to its own checkout so each jj workspace runs its own build; all logs go to
stderr to keep the stdio protocol clean). A checked-in Claude Code plugin at
`.claude/ron-lsp/` registers it for `.ron` / `.ron.todo` via `.lsp.json`:

```json
{
  "ron_lsp": {
    "command": "${CLAUDE_PLUGIN_ROOT}/../../scripts/ron-lsp",
    "transport": "stdio",
    "extensionToLanguage": { ".ron": "ron", ".ron.todo": "ron" },
    "startupTimeout": 60000
  }
}
```

The plugin doubles as its own local `directory` marketplace
(`.claude/ron-lsp/.claude-plugin/marketplace.json`) and is enabled from
`.claude/settings.json`. The repo `.gitignore` re-includes `.claude/ron-lsp/`
so the plugin is version-controlled even though `.claude/` is otherwise ignored.
After first checkout, run
`cargo build -p ron_lsp` once to warm the build (the wrapper's first launch
otherwise waits on a debug compile, which `startupTimeout` is sized to cover),
then use `/plugin` and `/reload-plugins` in a Claude Code session to confirm the
server registers. Once live, Claude's `LSP` tool drives all of the capabilities
above — e.g. `workspaceSymbol` with a plugin name lists that plugin's cards.

## Notes

The index returns every matching definition. Macro names are kind-scoped (for
example `AnyTarget` is both a `Predicate` and a `TargetSpec` macro); expected-kind
inference will narrow those results in a later slice.
