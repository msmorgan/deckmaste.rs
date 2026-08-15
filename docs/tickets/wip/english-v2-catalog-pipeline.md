---
needs: [english-construction-rewrite-design]
---
**Build the shared catalog pipeline and the `english_v2` loader for open-class
identity slots.** Stage 1 of the rewrite's implementation sequence.

Decisions already made (2026-08-13 design dialogue; the ADR produced by
`english-construction-rewrite-design` is the authority if anything here
drifts):

- A catalog is a pure word list feeding an `identity` slot in the new
  grammar. Anything needing per-entry grammar (e.g. keywords with their own
  syntax) becomes a construction instead, and the card's own name is a
  parse-context parameter passed at parser construction — neither belongs in
  a catalog.
- Catalogs are sorted, deduplicated plain-text line files with no headers. A
  fixed pair of upstream snapshots produces byte-identical output, and
  regenerate-and-compare is the complete currency check.
- The canonical inventory in `data/gen/catalogs` is exactly
  `ability-words.txt`, `artifact-types.txt`, `battle-types.txt`,
  `card-names.txt`, `card-types.txt`, `counter-kind-phrases.txt`,
  `creature-types.txt`, `enchantment-types.txt`, `keyword-abilities.txt`,
  `keyword-actions.txt`, `land-types.txt`, `planeswalker-types.txt`,
  `spell-types.txt`, and `supertypes.txt`.
- The CR is authoritative for every canonical file except `card-names.txt`.
  Card names come from MTGJSON `AtomicCards.json` faces: `faceName` is used
  when present and `name` otherwise, and `vintage_playable()` accepts only
  Vintage `Legal` or `Restricted` faces. `Banned`, `Not Legal`, null, and
  missing legalities are excluded regardless of layout.
- `counter-kind-phrases.txt` contains only the CR-defined multi-token keyword
  counter phrases [CR#122.1b]. Single-token kinds remain opaque, `+1/+1` and
  similar kinds are codecs, and MTGJSON is not counter-kind authority.
- The parser binds catalogs at instance construction — they are inputs, not
  compiled-in globals.
- Shared snapshot models live in `deckmaste_data`; extraction, the inventory,
  line-file I/O, exact directory comparison, and the compatibility adapter
  live in stable `deckmaste_catalogs`. The dependency contract is:

  ~~~text
  deckmaste_data -> deckmaste_catalogs -> {deckmaste_english, deckmaste_english_v2, xtask}
  deckmaste_migrations -> deckmaste_data (temporary)
  ~~~

  V2 therefore depends on a stable low-level crate and no crate scheduled for
  deletion.
- Legacy consumers retain a removable `deckmaste_catalogs::legacy` module and
  a separate `data/gen/catalogs-legacy` cache. Its twelve generated files are
  the canonical inventory minus `card-names.txt` and
  `counter-kind-phrases.txt`; canonical and legacy output never mix.
- `cargo xtask catalogs generate` replaces the canonical directory;
  `catalogs check` regenerates into a temporary directory and reports changed,
  missing, or unexpected canonical files without mutation; `catalogs text`
  replaces only the twelve-file legacy cache.

Deliverables:

1. `deckmaste_data` models the shared local CR, MTGJSON, and Scryfall inputs;
   migrations uses that layer directly.
2. `deckmaste_catalogs` generates and loads the exact canonical inventory,
   compares directories without mutation, and isolates the removable legacy
   inventory and variant behavior.
3. `deckmaste_english`, `deckmaste_english_v2`, and xtask use the shared
   catalog crate. V2 exposes the loader that will be handed to parser
   construction; its grammar remains vertical-slice work.
4. The `cargo xtask catalogs generate|check|text` family writes and validates
   the two disjoint output directories with the behavior above.

Acceptance: fixed upstream bytes generate byte-identical sorted line files;
all fourteen canonical files load; Vintage filtering and face-name selection
are covered; only CR-defined multi-token counter phrases are emitted;
`catalogs check` detects changed, missing, and unexpected entries without
mutation; the twelve compatibility files remain isolated and replaceable;
v2 has no deletion-bound dependency; focused, workspace, strict-clippy, and
citation gates pass. Standard constraints apply.
