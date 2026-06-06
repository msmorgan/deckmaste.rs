# Basic Lands Migration (`_005`) — Design

2026-06-06

## Goal

Implement migration `_005_basic_lands`: read the card todo files, find the
basic lands, and overwrite each todo with a proper card definition like those
in `plugins/builtin/cards`. Alongside it, the supporting infrastructure this
exposes a need for: a shared todo-reading module (todos are the entry point
for all future migrations), builtin-as-prelude plugin loading, a validation
pass that runs migration output through the real reader, and an `xtask`
crate to host project automation.

## Decisions already made

- **Wastes and Snow-Covered Wastes stay todos.** They have no basic land
  type, so their `{T}: Add {C}.` is real printed ability text — and the
  `Ability` model can't express activated mana abilities yet (the same
  blocker that shelved the tokens migration). The migration's text-empty
  criterion excludes them naturally.
- **Detection parses todo structure** (ron/serde), not a hardcoded name list
  or MTGJSON lookup. Works for any plugin; no data-file dependency.
- **Write side uses `format!` string templates, not serde mirrors.** The
  migration source shows the output file verbatim. Serde stays for the read
  side; validation comes from the reader-side checks, not a write schema.
- **builtin is a prelude to all other plugins**: its macros and subtype
  declarations are in scope when any other plugin loads.

## 1. Shared card-todo module (read side)

Move `CardFile`, `CardFaceTodo`, and `Stat` from `_004_card_todos.rs` to a
sibling module `migrations/card_todo.rs`. Add `Deserialize` derives, with
`#[serde(default)]` on every field that has a skip attr (the skip attrs
remain load-bearing — see ron `implicit_some` notes). `_004` keeps
serializing through these structs unchanged; `_005` and future migrations
deserialize todos through them. Todo files quote everything, so plain
ron/serde suffices — no macro awareness on this path.

Todo-stub detection moves to `deckmaste_core::plugin` as
`pub fn is_todo_source(source: &str) -> bool`, implemented with
`lines().any(|l| l.trim_start().starts_with("Todo("))` so core gains no
regex dependency. That module's charter is already "conventions shared by
readers and writers"; the validation pass needs the same classification the
migrations use. The migrations' path-based `is_todo` becomes a thin wrapper.

## 2. Migration `_005_basic_lands`

For each `.ron` under the plugin's `cards/`:

1. Skip unless the content is a todo stub (`is_todo_source`).
2. Parse into `CardFile` via `deckmaste_core::ron::options()`.
3. Convert when **all** hold:
   - exactly one face, `layout == "normal"`
   - `types == ["Land"]` and `supertypes` contains `"Basic"`
   - `text` is empty
4. Render via template, multi-element arrays inline:

   ```rust
   let definition = format!(
       "\
   Normal(
       name: {name:?},
       supertypes: [{supertypes}],
       types: [Land],
       subtypes: [{subtypes}],
   )
   ",
       name = face.name,
       supertypes = face.supertypes.join(", "),
       subtypes = face.subtypes.join(", "),
   );
   ```

   `{name:?}` (Rust debug escaping) is adequate for card names; supertypes
   and subtypes are emitted as bare idents, matching the builtin style.
5. Guard: the rendered output must parse as RON
   (`ron::value::RawValue::from_ron`) before writing. Real validation is the
   reader-side pass (§4).
6. Write with trailing newline; `eprintln!` the path like `_004` does.

Expected effect on `plugins/wizards`: exactly 10 files converted (Plains,
Island, Swamp, Mountain, Forest + their Snow-Covered variants), Wastes and
Snow-Covered Wastes untouched. Idempotent for free: converted files no
longer match the todo-stub check.

## 3. Builtin as prelude (reader)

`MacroSet` is `Clone`, so prelude loading is cheap composition:

- `Plugin::load_with_prelude(prelude: &Plugin, root)` starts from a clone of
  the prelude's macros and subtypes, then loads `root`'s `macros/` and
  `types/` on top. **Last plugin wins:** a macro or subtype declaration in
  `root` that collides with a name already in scope *overrides* it —
  plugins are layers, and a later layer may rebalance an earlier one (the
  Arena Alchemy model). Two same-kind definitions of one name *within a
  single plugin* are still a `DuplicateMacro` error: file order inside a
  plugin is alphabetical happenstance, so "last" would be meaningless
  there. (`_003`'s generated subtype set overlapping builtin's declarations
  — Plains, Island, Swamp, Mountain, Forest, Bear, byte-identical — is the
  immediate case: those redeclarations now simply win, silently.)
- Card files don't merge across plugins today (`Plugin::card` reads one
  plugin's `cards/`), so card-level overriding becomes meaningful only when
  a multi-plugin reader exists; the layering rule above is the precedent it
  will follow.
- `Plugin::load` stays as-is (it's what loads builtin itself).
- The builtin-sibling convention lives in one helper: given a plugin dir,
  if a sibling directory named `builtin` exists and isn't the plugin itself,
  load it as the prelude. Used by `cargo card`, `cargo xtask validate`, and
  the integration test.

## 4. Plugin validation

`pub fn validate_plugin(plugin_dir: &Path)` in `deckmaste_cards`: load the
plugin (builtin-sibling prelude per §3), then for every `cards/**/*.ron`
that is not a todo stub, `read_str::<Card>` must succeed. Failures are
collected (path + error) and reported together, not fail-fast.

Entry points:

- `cargo xtask validate [plugin_dir]` — defaults to `plugins/builtin`
  under the workspace root (resolved from `CARGO_MANIFEST_DIR`, so it works
  from any cwd); prints each failure and exits nonzero on any. The whole
  entry point — arg parsing, defaulting, reporting — lives in
  `deckmaste_cards` next to `card`'s, not in xtask. Wizards validation is
  always an explicit `cargo xtask validate plugins/wizards`.
- A `#[test]` in `deckmaste_cards` calls `validate_plugin` on
  `plugins/builtin` (resolved via `CARGO_MANIFEST_DIR`), so plain
  `cargo test` always validates the prelude's own cards — ~31k wizards
  file reads don't belong in every `cargo test`.

This pass is what closes the validation gap the template write-side leaves
open: migration output is checked by the real macro-aware reader.

## 5. xtask

New workspace member `crates/xtask` (clap subcommands), with alias
`xtask = "run --release -p xtask --"` in `.cargo/config.toml` — release
because `migrate` needs it (AllPrintings parsing under the dev profile takes
tens of seconds).

Subcommands dispatch **in-process** — the existing bins refactor to thin
wrappers over `pub` lib entry points (clap `parse_from` on forwarded args),
xtask imports the crates, and is a pure dispatcher: no subcommand logic of
its own.

- `cargo xtask validate [plugin_dir]` — §4's entry point in
  `deckmaste_cards`.
- `cargo xtask migrate <plugin_dir> [migration_number]` — calls
  `deckmaste_migrations`' entry point. The crate gains a lib target; its
  modules move under it, `main.rs` becomes the wrapper.
- `cargo xtask card <plugin_dir> <card_name>` — calls `deckmaste_cards`'
  entry point (moved from its `main.rs` into the lib).

The `migrate` and `card` aliases and bins stay; xtask is a unified front
door, not a replacement.

## 6. Testing & verification

- Unit: todo deserialization round-trip (`_004`-shaped source →
  `CardFile`), the basic-land filter predicate, exact rendered template for
  a plain and a snow land.
- `cargo test` — includes the builtin validation test (§4).
- End-to-end: `cargo xtask migrate plugins/wizards 5`; `jj diff --stat`
  shows exactly 10 changed files, Wastes untouched;
  `cargo xtask validate plugins/wizards` green;
  `cargo card plugins/wizards "Snow-Covered Plains"` expands cleanly;
  migrations 0–4 output unaffected (re-run produces no diff).

## Out of scope

- Wastes' activated mana ability (blocked on the real ability model, like
  the shelved tokens migration).
- ~~`_006_vanilla_creatures` (next migration; reuses the shared todo
  module).~~ — designed below, addendum of 2026-06-06.
- Any change to the macro language or the todo file format.

## Addendum (2026-06-06): `_006_vanilla_creatures`

Same machinery, next migration. Corpus measured at design time: 338
candidates in wizards — 311 pure `[Creature]`, 27 multi-type (Artifact /
Land / Kindred), 4 with color indicators (Dryad Arbor + the three
{0}-cost Kobolds), 0 with `*` stats, 0 without subtypes.

**Criteria** (user decision: all expressible, not just `types ==
["Creature"]`): exactly one face, `layout == "normal"`, `types` contains
`"Creature"`, `text` empty, `power` and `toughness` both present and
`Stat::Number`, `loyalty` and `defense` absent. The `Number` requirement
guards against `*` stats (CDA reminder text) ever slipping through.

**Template** — lines in card-file field order, each omitted when its
value is empty/absent:

```ron
Normal(
    name: "Grizzly Bears",
    mana_cost: [
        Generic(1),
        Green,
    ],
    types: [Creature],
    subtypes: [Bear],
    power: 2,
    toughness: 2,
)
```

- `mana_cost`: omitted when empty (Dryad Arbor); one line when single
  (`[Generic(6)]`); chopped one-per-line when multi — matching builtin's
  hand-written Grizzly Bears byte-for-byte and `_004`'s todo style (user
  decision).
- `color_indicator`, `supertypes`, `types`, `subtypes`: bare idents,
  inline (`[Artifact, Creature]`, `[Elf, Ranger]`) per the `_005`
  precedent; omitted when empty (`types` never is).
- `power`/`toughness`: the bare number.
- Mana symbols and colors are rendered through the shared ron config
  (`to_string_pretty` on the leaf value — tuple members stay inline by
  default), NOT hand-spelled: that keeps `Hybrid(Generic(2), White)`
  spacing canonical without a per-migration schema. The template still
  owns the file shape; ron only spells the leaf tokens.
- Post-render `RawValue::from_ron` guard + reader-side validation, exactly
  as `_005`.

**Expected effect on wizards:** 338 conversions; `cargo xtask validate
plugins/wizards` reports 348 valid (10 lands + 338 creatures), 0 invalid;
wizards' `Grizzly Bears.ron` diffs identical to builtin's; re-run writes
nothing.
