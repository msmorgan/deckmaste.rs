---
needs: [english-v2-open-grammar-environment]
---
Replace the narrow builtin bootstrap reader with the final dependency-closure
provider for the normalized grammar environment ratified by the
[builtin-v2 spelling/grammar ADR](../../decisions/builtin-v2-macro-spelling-and-grammar.md).
This ticket owns plugin discovery, merge policy, and environment construction;
it does not compile semantic spelling frames or English-authored cards.

Resolve the ordered dependency closure, including the plugin being loaded, and
read all v2 declaration headers before parsing any English source. Feed every
source through the neutral reader and normalized-row contract, then compile
the merged rows through the same environment builder used during bootstrap.
The scanner, parser, renderer, diagnostics, and constructions must not know
whether rows came from `builtin_v2`, a dependency, or the leaf plugin.

Apply one category-aware shadowing decision to semantic declaration identity,
grammar, and later spelling-frame visibility. A dependent plugin may replace a
dependency's same-kind identity; duplicate same-kind declarations within one
plugin are errors; equal local names in different registry categories remain
distinct. Preserve all surface collisions that survive identity shadowing.
Filesystem and discovery order never select a declaration or lexical reading,
and every diagnostic names plugin and source path.

Prove the closure provider produces equivalent normalized rows for a
builtin-only fixture. It may coexist temporarily with the narrow bootstrap
entry point so this ticket does not depend on completion of hundreds of
builtin inventory records, but runtime consumers still see the same
environment ABI and never a fallback merge. The final cutover ticket deletes
the bootstrap entry point atomically after the inventories land. Do not add an
adapter, catalog-enrichment pass, or second runtime vocabulary. Canonical
catalogs may be invoked only by the separate read-only coverage gate; they do
not supply identity membership, casing, morphology, valence, or surfaces here.

Acceptance uses synthetic plugins rather than the official vocabulary ceiling:
a dependency contributes a novel subtype and counter kind used by a dependent;
a leaf plugin contributes a novel verb; same-kind shadowing changes semantics
and grammar coherently; a same-plugin duplicate fails deterministically;
cross-kind homonyms survive; and a declaration absent from every official
catalog scans and round-trips. Compare bootstrap and closure-provider
environments for a builtin-only fixture and leave the provider ready for the
final atomic cutover. Standard constraints apply.
