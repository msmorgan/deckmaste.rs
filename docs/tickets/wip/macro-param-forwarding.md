---
needs: []
---
Support forwarding a macro's own `Param(n)` into a nested macro invocation.
Today `CastFromGraveyard(Param(0))` inside Flashback's body fails at expansion
("Expected opening `[`"), so param-carrying sub-macros can't be factored — only
zero-param and concrete-arg sub-macros compose, and keywords keep their
param-carrying half inline (e.g. Flashback's `May(Cast(cost:
Components(Param(0))))`, `PumpThisUntilEot`).

Ruling (2026-07-12): the no-forwarding state was an implementation shortcut,
not doctrine. The macro language deliberately bans conditionals, recursion, and
meta-features — plain declarative param forwarding is none of those and should
work.

Implementation lives in `crates/macro_ron` (nested-param-scope during
expansion): substitute the outer macro's params into a nested invocation's args
before/while expanding the inner macro. Keep the existing bans intact:
recursion stays a load error (`MAX_DEPTH`), no conditionals, no wildcard kinds.
Follow-up candidates once landed: factor Flashback's cast half; revisit
`PumpThisUntilEot`.
