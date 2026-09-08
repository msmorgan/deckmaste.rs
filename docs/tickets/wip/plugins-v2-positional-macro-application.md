---
needs: []
---
**A named-signature macro may be applied positionally.** `plugins-v2-cosmetic-conversion`
found that `hasType(Creature)` and `colorIs(Red)` are refused
(`Expected colon`): `read_args` reads `Params::Named` through a map
visitor, so the one-argument positional classification the dialect gives
constructors never reaches macro invocations. `Params::Named` is an
unordered map, so positional application needs a declared parameter order:
give `MacroDef` an ordered parameter list (declaration order in the file is
the order; the drift test's binder order is the contract for ported
helpers) and let `read_args` accept a sequence mapped by that order, with
the mixed form still refused. Then rewrite the 176 named one-field macro
invocations in card source positionally (source-to-source, comments
untouched, `lean/Generated` byte-identical as the oracle). Standard
constraints apply.
