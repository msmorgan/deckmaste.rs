import Semantics.Definitions.Counters
import Semantics.Definitions.Designations
import Semantics.Definitions.Relations
import Semantics.Definitions.Subtypes
import Semantics.Definitions.Actions
import Semantics.Definitions.Keywords

/-!
# Semantics.Definitions

Rules-defined vocabulary (`Definition`s, `semantic_definition`) for `semantics-v3`
(`docs/decisions/semantics-v3.md` §§1–3): CR §701 keyword actions, CR §702 keyword abilities, and
the counters, tokens and designations the rules define. A Definition evaluates in its own
Reference Scope (`ownScope` + `inCaller`).

This module re-exports the submodules, all in `namespace Semantics.Definitions`, in two layers:
the leaf catalogs (`Counters`, `Designations`, `Relations`, `Subtypes`) are spelled over
primitives and imported by `Semantics.Templates`, which names their entries; the composite
Definitions (`Actions`, `Keywords`) are spelled over `Semantics.Templates`.
-/
