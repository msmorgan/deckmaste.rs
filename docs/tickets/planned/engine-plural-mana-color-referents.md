---
needs: [engine-payment-obligation-window]
design: true
---
Add a plural color-union referent for mana production such as Mox Amber.

`ManaSpec::AmongColorsOf` currently accepts one `Reference`. The payment-window
acceptance fixture intentionally needs only one qualifying colored legendary
permanent and then zero, so it uses `Single(SelectAll(...))`: exact for that
matrix, but fail-closed when two or more qualifying permanents exist. A real
Mox Amber must instead offer the union of colors among every legendary creature
and planeswalker its controller controls, without an object-choice prompt.

Settle whether mana specs consume a `Selection` directly or gain a dedicated
plural color-source form. Preserve deterministic color ordering, produce no
mana when the set or its color union is empty, and add zero-, one-, and
multi-permanent tests including overlapping and distinct colors. Replace the
synthetic fixture's singular shortcut only after that representation exists.
