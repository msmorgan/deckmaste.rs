---
needs: []
---
**Generate the Lean workbench's facts tables from the registry declarations.**
`lean/lean/Semantics/Check/Words.lean` and `Check/Keywords.lean` carry four
hand-written tables the checker's laws read: `actFacts` (keyword actions, the
deed table), `keywordFacts`, `counterFacts`, and `designationTable`, plus the
small `subtypeFacts` (Saga, Adventure, Room frames). Each is a second copy of
knowledge the registry owns under `plugins/builtin_v2/macros/stubs/`
(`keyword_actions`, `keyword_abilities`, `counter_kinds`, `designations`,
`subtypes`), and the copies drift: the 2026-09-05 review found the designation
rows wrong about carrier (creature-held where the CR says permanent), casing
(`Alpha sector` versus the CR's `alpha sector`), and conferrer (Storied,
Monstrosity). That is the drift
[conferrals-come-from-registries](../../decisions/conferrals-come-from-registries.md)
forbids: "a new declared name must not require a corresponding hard-coded
behavior branch".

The Idris workbench already had this fixed for one table: `cargo xtask facts
generate` writes `idris/src/Experimental/FactsGen.idr` (the keyword facts) from
the keyword-ability stubs plus a hand-kept gate-column overlay in
`crates/xtask/src/facts.rs`, and `facts check` fails when the committed module
is stale. Do the same for Lean, for all four tables:

- `cargo xtask facts generate` also writes `lean/lean/Semantics/Check/Facts.lean`
  (one generated module; `Check/Words.lean` and `Check/Keywords.lean` import it
  and lose their literal tables). Kernel `decide` still evaluates the tables, so
  they stay Lean data; only their authorship moves.
- Columns the stubs do not carry (deed roles, `stepwise`, `feature`,
  `counterfactual`, `opponentsLibrary`, the keyword regime and gate columns) stay
  in xtask's overlay, keyed by label, as they are for the Idris table today. A
  stub with no overlay row and a column the checker needs is a generation error,
  as it is now.
- `facts check` covers the Lean module, and `./scripts/build` keeps running the
  `Proofs/Tables.lean` uniqueness pins over the generated tables.
- `KeywordFacts.confers` / `ActFacts.confers` (a designation's conferrer, landed
  2026-09-05) come from the stubs' `DesignationDecl` links, not the overlay. Two
  conferrers name a *set* of designations (Space Sculptor: alpha/beta/gamma
  sector; Unlock: left/right half), so the column is a list, not an `Option`.

Decisions already made that bind this: the Lean is the successor workbench and
the Idris is the reference only (the Idris `FactsGen` can be dropped when the
Idris goes); laws read declared features, never a lexeme, so any string the
generator emits is a table key, not something a guard may name; Vintage-playable
Magic only, so the registry's scope is the table's scope and no row is reserved
for anything outside it.
