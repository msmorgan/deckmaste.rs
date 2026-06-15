---
needs: []
---
**[design]** Lookup primitive for `parse-via-macros`: a kind-scoped reverse index
over the registered macros — template-shape → macro — so a parser that knows the
macro kind it's targeting (KeywordAbility, Effect, Filter, …) can find the macro
whose template matches a parsed fragment and emit its invocation. Build it from
the already-registered macros (builtin/canon/… prefixes); expose lookup +
arg-binding. Gated on the `parse-via-macros` dialogue: what "template shape" means
as a match key when the template is a render/expand template, and how
raw-RON-source args map onto parsed fragments. No parser is rerouted here — that's
the children.
