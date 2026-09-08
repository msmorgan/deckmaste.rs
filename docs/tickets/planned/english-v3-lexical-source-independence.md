---
needs: [english-v3-lexical-model]
---
# Detach v3 lexical sources from English v2 tooling

Replace the private `xtask::english_v2` lexical-source path with a v3-owned
source module whose small interface returns normalized lexical declarations.
`deckmaste_lexical_model` remains data-only and `deckmaste_lexical` remains the
analysis/realization engine; source discovery, authoring-format adaptation and
inventory assembly sit behind a separately testable adapter. An English v3
consumer must not depend on xtask, `deckmaste_english_v2`, or the deletion-bound
v2 construction compiler to obtain a `Lexicon`.

Move the census to a neutral `cargo xtask lexical` command and remove the
`english_v2 lexical` entry point. Existing v2 declarations may be consumed only
through an explicitly transitional input adapter while the lexical-inventory
ticket consolidates their contents; they are not the durable interface or
source identity. Rust must not name an individual RON source file, directly or
by reconstructing a declaration identity from its filename. Discover source
trees generically and take identities from normalized declarations.

Carry the review follow-ups with the seam repair: replace the two plain
construction/readback model tests with behavioral serialization and consumer
tests, explicitly document that the new `FrameItem` export shape is not
backward-compatible with prototype artifacts, and replace the glob model
re-export with an explicit public surface.

Acceptance proves that the library-owned source interface assembles and indexes
the full current inventory, rejects unresolved supplemental overrides, and
reproduces the lexical census and independent realization/reanalysis law without
an exported-file handoff. Repository searches must show that the new durable
pipeline neither calls `xtask::english_v2` nor names a RON file. Preserve all
licensed readings and source provenance; do not broaden this repair into the
planned lexical inventory's vocabulary expansion. Standard constraints apply.
