---
needs: [english-v2-grammar-migration-design]
---
> **Migration routing (2026-09-05).** This unclaimed ticket waits on
> `english-v2-grammar-migration-design` under the
> [Lean design decision](../../decisions/english-lean-design-workbench.md).
> That design task must reconcile and repin this ticket before it becomes
> executable. The prior body below preserves examples, regression and
> re-coverage obligations, and proposed mechanisms; its old sequence,
> implementation prescriptions, and coverage-ratchet acceptance do not
> override the new design process or the current landing contract.

# Type `without`, `by`, `than` and `as` as prepositions

**R8 — Group R.** The unclosed remainder of the Plan 09 taxonomy audit's HIGH
finding that `with / without / by / as / between` cannot form prepositional
phrases. `english-v2-with-preposition` (done, 2026-09-04) closed the `with` half
and is the worked example to follow.

Defect. `vocab Preposition`
(`crates/deckmaste_english_v2/src/constructions.rs:38`) has fifteen members and
none of these four, so each appears as a bare form literal in a construction
minted to spell it:

- `without` — `verb(head) object "without" "paying" complement` (`:2024`),
  `keyword_without_subject_modifier = "without" lex(ability)` (`:4811`)
- `by` — `participial_by_complement = verb(head) "by" agent` (`:1741`),
  `participial_except_by_complement = verb(head) "except" "by" agent` (`:1748`)
- `than` — `:1992, 2014, 3663, 3671, 3682, 3911, 3987`
- `as` — `:1486, 1490, 1495, 1499, 1504, 1516, 1608, 1880, 4304`

Pinned shape. One member per word, each with a declared attachment class
justified from English under R1's default (adjunct- and postmodifier-capable
unless a stated fact narrows it), and each with its complement kind. Retire the
fused constructions whose only reason to exist is the missing member, re-spelling
their coverage through the general PP and its noun-side licence, exactly as the
`with` landing did. `as` is the hardest: several of its sites are a subordinator
(`as long as`, `as though`) rather than a preposition — split those explicitly
and say which is which in the record; `english-v2-subordinate-clause` owns the
subordinator half and must not be duplicated here. `than` is a comparative
complementizer at some sites for the same reason.

Land the four as separate changes if the grammar lane allows; one ticket because
they are one family and one method.

Fences. Declaring an attachment class by corpus count. Keeping a fused
construction "for now" beside the PP. A `checked by` naming any of the four
words. Adding complement arms only where a census shows them — every arm the
preposition governs in Oracle English is declared, and the census goes in the
record.

Glossary: Preposition, Subordinator, Complementizer, Prepositional Phrase,
Adjunct, Postmodifier. Record any gap.

Baseline, measured on change `oulzkkoqmvuv` (388 constructions, 17,052 / 32,641
covered) — re-measure at claim. Standard constraints apply.
