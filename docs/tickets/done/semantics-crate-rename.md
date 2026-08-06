---
needs: []
---
**Rename `deckmaste_semantics` → `deckmaste_semantics` and sweep the
"authoring" crate name project-wide.** Settled 2026-08-05: "authoring" names a
provenance that is false for most of the crate's content — the bulk is compiled
from Oracle text by recovery; only the rules tables, macros, and builtins are
hand-written. What every term shares is what it *is*: the canonical meaning
representation. `semantics` also completes the syntax/semantics split with
`deckmaste_english` (English-text syntax only, no card behavior) bridged by
`deckmaste_spelling`.

Scope — every reference to the crate, and to "authoring"/"authored" where it
names the grammar or its form, **except tickets under `done/`** (historical
records stay as written):

- **Crate:** directory, `[package] name`, every dependent `Cargo.toml` and
  `use`/path reference (10 dependent crates incl. xtask); `Cargo.lock` follows
  from a build.
- **Terminology naming the form:** "semantics grammar" → "semantics grammar",
  "semantic term" → "semantic term", "semantic normal form" → "semantic
  normal form", in code identifiers, comments, and docs alike.
- **The two-sense rule** for every other hit: a phrase naming the crate,
  grammar, or canonical form renames; the plain activity sense — hand-writing
  content — stays (e.g. the "keyword authoring" priority tier,
  `cards-untap-skip-authoring`, `docs/memory/authoring/`). Judgment sites get
  a read, not a mechanical replace; `docs/decisions/invalid-semantic-input-fizzles.md`
  is a known one (it is about malformed content in the form, whatever its
  provenance — retitle only if the reworded doc still reads true).
- **Docs:** `docs/decisions/semantics-spelling-lowering.md` renames to
  `semantics-spelling-lowering.md`; update every citer (crate doc comments,
  `CLAUDE.md`, decisions README, other decisions/tickets) and add a dated note
  in the ADR recording this rename decision. Update `docs/memory/` notes that
  name the crate in place (gitignored, shared live — not part of the commit).
- **Inventory discipline:** sweep with `rg -i authoring` from the repository
  root and classify every hit under the two-sense rule — the sweep is the
  inventory; do not trust any pre-baked list.
- **Sequencing:** this is a workspace-wide import/text sweep — do not run it
  concurrently with another sweep-shaped claim; refresh before integrate.

Standard constraints apply.
