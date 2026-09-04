---
needs: [english-v2-np-postmodifiers, english-v2-lexeme-owned-verb-frames]
---
Route keyword-subject modifiers through the general staged reference stack
(separator-admission review M1/M3). `KeywordSubjectModifier` is a new
5-member hand-enumeration (`"with" Object`, `"without" lex(KeywordAbility)`,
…) where the ticket asked for the general reference stack: measured
asymmetry (unstamped, measured before `english-v2-np-postmodifiers` landed;
re-measure at claim) — `Enchant creature without flying` parses, `Enchant
creature with flying` does not; `without a counter on it`, stacked modifiers,
subject-gap relatives, and participials all reject. Delete the enumeration;
the keyword subject takes the same staged reference phrase every nominal
position takes (postmodifiers included, per np-postmodifiers). Also M3:
the F7 `VerbLexeme` predicate is vacuous (zero `lexeme:VerbLexeme/` rows
exist as measured, unstamped — re-measure at claim; neither `||` branch runs,
and its message "only the two intentionally irregular verb lexemes remain" is
false) — replace with a
set-equality assertion over the irregulars actually present. 2026-09-04: measurements marked for re-measure at claim (unstamped baselines
predating the np-postmodifiers landing). Ratchet up or
equal, -0 rows; landing record with measured-tree stamp, census,
Deviations. Standard constraints apply.
