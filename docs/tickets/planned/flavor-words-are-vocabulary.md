---
needs: []
---
**Flavor words go back to being stubs.** Ruling (user, 2026-09-07). The
630 files under `plugins_v2/builtin/macros/flavor_words/` are not macros:
a flavor word is an open slot, `flavorWord (word : FlavorWordLabel)
(ability : Ability)` in `Macros.lean`, ported as the builtin `flavorWord`
helper, so a card writes `flavorWord("Kowabunga", ability)` with the label
as a string. The files are english_v2's recognition vocabulary (spelling
plus a `FixedTerm` grammar) and stay exactly as they are in content;
editing english_v2 is out of scope; correcting `deckmaste_lexical`, the
English v3 lexical crate, is in scope.

Move the family back to `plugins_v2/builtin/macros/stubs/flavor_words/`,
where it lived before `plugins-v2-dialect`'s rename, and re-point
construction_core's `BUILTIN_FAMILIES` entry for that one family to the
stubs path (a path constant only; no other construction_core or
english_v2 change). The v2 reader (`deckmaste_semantics_v2::reader`)
never reads `stubs/`, so no flavor-word declaration enters the macro
table or the corpus count; the corpus test states the new count. The
`FlavorWord` meta is untouched.

Second part, `deckmaste_lexical`: a flavor word is an open slot there too.
An italic run before an ability that is not a declared ability word is a
flavor word, captured verbatim as its label; the stubs are never a lexical
inventory source (no `Source` of any `SourceKind` points at
`stubs/flavor_words/`), and `english-v3-lexical-inventory` reads them out
of its "existing plugin declarations" input. Pin it: an italic run absent
from every declaration analyses as a flavor word, and a declared ability
word does not. Standard constraints apply.
