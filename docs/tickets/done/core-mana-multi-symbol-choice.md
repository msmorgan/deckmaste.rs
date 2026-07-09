---
needs: []
---
Multi-symbol mana-production choice (filterlands). `ManaSpec::OneOf` holds single
colors only — there's no representation for a choice AMONG multi-symbol outputs
(no mana-production `Modal`). So filterland abilities `{1}, {T}: Add {W}{W},
{W}{U}, or {U}{U}.` decline.

Add a choice-over-runs to the mana-production model (a `ManaSpec` variant holding
a `Vec` of multi-symbol options, resolved by a mana-mode decision at activation),
then the parse-mana-abilities production already in place will emit it.

~10+ filterlands one-away (plus the full filterland cycle multi-blocked). Flagged
by the parse-mana-abilities worker.
