import Semantics.Definitions.CardTypes
import Semantics.Definitions.Counters
import Semantics.Definitions.Designations
import Semantics.Definitions.Relations
import Semantics.Definitions.Subtypes
import Semantics.Definitions.Phrasings
import Semantics.Definitions.CardTypeRules
import Semantics.Definitions.Actions
import Semantics.Definitions.Keywords

/-!
# Semantics.Definitions

Rules-defined vocabulary (`Definition`s, `semantic_definition`) for `semantics-v3`
(`docs/decisions/semantics-v3.md` §§1–3): CR §701 keyword actions, CR §702 keyword abilities, and
the counters, tokens and designations the rules define. A Definition evaluates in its own
Reference Scope (`ownScope` + `inCaller`).

This module re-exports the submodules. `CardTypes` is the one leaf catalog below the checker: its
values live in `namespace Semantics` (`CardType.creature`), over `Semantics.Words` alone, because
the checker compares card types by value. The rest are in `namespace Semantics.Definitions`, above
`Semantics.Templates`: the leaf catalogs (`Counters`, `Designations`, `Relations`, `Subtypes`) are
spelled over primitives; `Phrasings` holds the Templates that name a card type or a leaf catalog
entry, spelled over `Semantics.Templates` and the leaf catalogs; `CardTypeRules` holds what each
card type confers; and the composite Definitions (`Actions`, `Keywords`) are spelled over all of
them.
-/
