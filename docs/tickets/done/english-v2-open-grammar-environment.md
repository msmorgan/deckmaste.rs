---
needs: [builtin-v2-declaration-schema]
---
Build the open identity and immutable parser-environment foundation required
by Stage 5 and the
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
This ticket consumes normalized grammar rows; it does not read catalogs,
discover plugin files, resolve dependency closures, or compile semantic
spelling frames.

Represent open declaration identity with category-safe owned or interned
carriers. Keyword actions, keyword abilities, each supported subtype category,
types, counter kinds, and designations must remain distinct domains even when
their local names or surfaces coincide. Do not mint an exhaustive Rust enum
member per registry entry and do not collapse all families into an unchecked
string. Closed grammar recipes, AST categories, and feature axes may remain
finite enums.

Compile normalized rows plus closed core grammar into one immutable
`ParserEnvironment` (name illustrative). It owns or shares every runtime ID
and surface needed after construction and exposes efficient indexes in both
directions:

- surface and grammatical position to every compatible lexical reading;
- declaration identity plus realized features to the exact render surface;
- declaration identity to its validated recipe, valence, and provenance.

Surface collisions retain every reading. Neither insertion order, catalog
order, nor a stable sort is a winner-selection rule. The parser's ordinary
packed-forest ambiguity and later frame selection remain responsible for
distinguishing readings. Static semantic constructions such as Destroy and
Connive request a declaration ID from this environment; they do not declare
the ID, own a duplicate surface table, or require an exhaustive identity
variant.

Make scanner, parser, renderer, diagnostics, construction materialization, and
lexical ownership consume only the immutable environment. Parse-context
identities such as card names remain explicit parse context, not open registry
rows. There is no process-global or thread-local vocabulary. Public provenance
accessors return `&str`; their backing storage may be owned, shared, or
interned, but no public `&'static str` promise may prevent later provider
migration.

This is the Stage 5 ABI seam. Amend the active buildout before its Task 4 so
the aggregate terminal ABI carries open category-safe declaration identity;
Task 5 receives this environment rather than `ParserCatalogs`; Task 6 seals
the normalized surface rows and dumb morphology; Task 9 constructs registry
terminals from environment membership rather than a catalog-owned final type;
and Task 10 retains migration-capable owner IDs. Tasks 1 through 3 are already
complete and should change only if an explicit incompatibility is found.

Acceptance builds environments entirely from synthetic normalized fixtures,
including a novel action, subtype, counter kind, and two cross-category
homonyms absent from official catalogs. Prove bidirectional scan/render
round-trips, collision preservation, category isolation, deterministic
diagnostics, no `ParserCatalogs` access below the provider boundary, no
per-member generated construction or enum variant, and no public static
lifetime in a registry owner ID. Standard constraints apply.

## Completion evidence

Completed by Stage 5 Plan 03 Tasks 4–10. Integrated declaration prerequisites:

- schema: `pzqtovourwwrwqkykkprklosrrtvwkor`;
- keyword actions: `tuuqyzvnswxmwmovkzzpynxlnsulwolr`,
  `zmmzoyvpmtlwwzqllnluyzzqukokwovz`, and
  `wqqxollnqlyqqmmxwumwktrlwvzttuko`;
- creature subtypes: `zzuqpzsnkkvnlwmrsnnunnnrpzonowyy`, fixed by
  `xkwpmqvunvupmxxkvuqomkovmksopqtk`;
- noncreature subtypes: `uzvtlxvnqskrlxlwtznqxrplltqpozmz`;
- card types: `zrzqtxtmxryruvnuouomyuzooyzqkxrz`, assurance fixed by
  `mrtnuvopvlsxwzoqzwpytnmxluunrluw`.

Synthetic fixtures prove novel open identities, category-safe homonyms,
collision preservation, scan/render symmetry, shuffled-input determinism, and
owned owner IDs. Final source scans find no catalog authority below the
provider, no per-member open-family enum, and no public static-lifetime owner
identity.

Final corpus evidence: 32,285 total / 48 accepted / 32,237 ordinary failures,
0 ambiguities and 0 internal failures; all 48 accepted units round-trip exactly
and select uniquely. The prior 47 accepted IDs remain unchanged, with only the
separately reviewed Rend Spirit ID
`5a0bd9563d2e05ca394ee7bedc5e55f386f82ee16f4227c410565066c6585660`
added. Probe and inspect JSON for Rend Spirit are byte-deterministic.

Completion change: `xpomwntnkutrrrlsokukxxmmzlnvykxt`
(`english v2: finish open grammar environment`).

Independent-review Fix Round 1 is
`pxskwqrnumurluykxmrwosupzuqmuxlm`: structured raw owner identities defer all
stable-label construction past cap-zero projections; impossible inspection
corruption travels through the real selected-analysis and xtask diagnostic
path; corresponding-slice and missing-owner validation are pinned at their
production boundaries; and the one-run gate distinguishes one stage invocation
from three candidate evaluations. The full workspace, fmt, denied Clippy,
source, and unchanged 32,285 / 48 / 32,237 corpus gates are GREEN.
