---
needs: [english-v3-licence-plumbing-economy]
---
# Convert every Category interface to head and local modes

Convert the whole grammar onto the head-inheritance mechanism: annotate each Category's interface entries as `head` or `local`, declare coordination and series headless with `head none;`, remove the exports the compiler now derives, and remove policies that only forwarded or defaulted features. No behaviour change. Gate: every reading digest identical on every face.

The compiler capabilities and the dead-declaration pass first planned for this ticket are `grammar-dead-declarations-and-requirements`.

## What landed

All 140 nonempty Category interfaces now use head/local modes. The 714
interface slots, 198 Categories, 326 constructions and 927 lowered rules are
unchanged. Coordination and series are explicitly headless and retain their
exports. Reading fields, form order and admission constraints are unchanged.

Each family passes the full-corpus `--check identical-digests` gate against
the same baseline taken before the first edit:

| Family | Change | Lines | Exports | Policies | Tables |
|---|---|---:|---:|---:|---:|
| Baseline | | 7,456 | 1,872 | 132 | 216 |
| Noun phrases | `lskylzln` | 7,168 | 1,544 | 131 | 216 |
| Preposition phrases | `ouxxktwm` | 7,013 | 1,367 | 130 | 216 |
| Predicates and verb phrases (47 interfaces) | `pnpssukm` | 6,909 | 1,122 | 125 | 216 |
| Clauses (13 interfaces) | `wukqwkzq` | 6,910 | 1,092 | 123 | 216 |
| Remaining interfaces (55) | `nrtousno` | 6,897 | 991 | 119 | 216 |

Removed policies and why their uses are no longer needed:

- `NominalHeadProperties`: its sole assignment is the local default.
- `PrepositionHeadPermissions`: defaulted lexical forwarding is inferred;
  the non-defaulted PrepositionFunctionLicence remains explicit at its callers.
- `PredicateHostAgreement`, `FiniteGapHostProjection`, `BareGapHostProjection`,
  `SecondaryAdjunctProjection`, `ManaPassiveProjection`: every assignment is
  inferred from the syntactic head at every former caller.
- `SourceSelectionAnchor`, `ImperativeSupplement`: their anchor forwarding is
  inferred after declaring `head anchor;`.
- `CompleteSentence`, `UnitFreeMeasure`, `NoPropertyEquality`: their assignments
  are existing defaults when the named head lacks those slots.
- `PredicateHeadAgreement`: its last caller has a lexical Pronoun head;
  its number/person exports are inlined because neither has a declared default.

No tables, interface slots or rules have been deleted. The 91 unread-slot
candidates remain unmodified; unread case exports still constrain admission
through their table lookups. All 24 AND and 11 OR tables have export users.
No compiler capability is added in this prefix.

Census delta (`nrtousno`, prefix): faces read 30,794 → 30,794; total Readings 482,041 → 482,041; lost faces: none.
Judgment set (`nrtousno`): does not apply yet.
