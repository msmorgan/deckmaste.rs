---
needs: []
---
**Delete the flavor-word declarations.** Ruling (user, 2026-09-07). A
flavor word is an open slot: `flavorWord (word : FlavorWordLabel)
(ability : Ability)` in `Macros.lean`, ported as the builtin `flavorWord`
helper, so a card writes `flavorWord("Kowabunga", ability)` with the label
as a string. Nothing about a flavor word is enumerable, so the 630
per-word declarations under `plugins_v2/builtin/macros/flavor_words/`
model nothing. Delete the family, the `FlavorWord` meta,
`DeclarationKind::FlavorWord` and the `flavor_words` entry in
construction_core's `BUILTIN_FAMILIES`, and whatever in the v2 reader or
`xtask` registers or counts the family. The corpus test states the new
count.

english_v2 breakage this causes is accepted: English v3 is the horizon
version and english_v2 is not corrected for this. Record what broke
(coverage-lock identities that stopped parsing, failing tests by name)
in the landing record and route it to the v3 tickets, not to english_v2
fixes. The tests that fail because their subject is gone are re-spelled
against `deckmaste_lexical`'s open slot where a v3 subject exists, and
otherwise listed with the ticket that will re-cover them.

`deckmaste_lexical` (English v3's lexical crate) is in scope: an italic run
before an ability that is not a declared ability word is a flavor word,
captured verbatim as its label; no `Source` of any `SourceKind` points at
a flavor-word list, and `english-v3-lexical-inventory` excludes the
family from its input. Pin it: an italic run absent from every
declaration analyses as a flavor word, and a declared ability word does
not. Standard constraints apply, except the coverage lock: a decrease
whose every lost identity is listed and routed is this ticket's expected
result.
