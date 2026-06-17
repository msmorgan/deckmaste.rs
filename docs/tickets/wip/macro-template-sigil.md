---
needs: []
---
Foundation for `parse-via-macros`: replace the `{i}` template placeholder with
`${0}` / `${name}` so placeholders stop colliding with mana symbols (`{1}`,
`{2}`, `{W}`, `{X}`, `{T}`, …). A `{2}` placeholder is indistinguishable from
"two generic mana" in both directions, and the reverse parse matcher (built next
in `macro-parse-index`) cannot work until the sigil is unambiguous.

New placeholder grammar: `${0}`, `${1}`, … = positional `Param(i)`; `${name}` =
named `Param(name)`; single-brace `{…}` stays a literal game symbol; `~` =
subject (unchanged).

Mechanical, isolated — lands and integrates on its own before any parse logic:
migrate the ~15 slot-bearing macro `template`s, update `render::template::fill`
(the `{`…`}` scanner → `${`…`}`) and its `render_arg`, and the tests. Forward
render output is unchanged — a pure syntax swap, so the renderer snapshot suite
is the guard.
