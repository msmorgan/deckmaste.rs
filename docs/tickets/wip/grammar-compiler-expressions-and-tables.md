---
needs: [english-v3-licence-plumbing-economy]
---
# Boolean requirements, policy composition, wildcard tables, unread slots removed

Second half of replacing the authoring layer: finish the compiler capabilities the prototype did not need, then convert the whole grammar to use them.

## Change

- Two kinds of expression are different things. A Boolean predicate in `require` accepts or rejects a combination; a feature-valued expression in `export` computes a value. This ticket delivers the Boolean predicates only: `require` accepts `and`, `or`, `not` and `in` (today an equation is an export of the same feature, a constant, a table call, `agree`, or a `require` of a constant or table result: `crates/deckmaste_construction_v3_core/src/parse.rs:127–137`). That removes the hand-written two-argument `{No, Yes}` tables (24 AND and 11 OR) wherever their result is only required. A table whose result is exported stays as it is, for example the computed permissions at `crates/deckmaste_english_v3/src/declarations.rs:2028`; replacing those with combiners belongs to `grammar-generic-coordination`, since feature-valued expressions are not delivered here. The conversion reports how many of the 35 tables fall on each side.
- Policies compose: a policy may use other policies. Today a policy admits only feature equations (`parse.rs:620–648`, check at `:634–645`) and `use` is rejected in policies; 135 policies span 1,143 lines, 61 referenced once, 2 never. Precedence between a policy and an enclosing construction's own equations, absent-value behaviour (a feature the used policy reads that the user does not export), rejection of policy cycles, and conflicting overrides must be defined and each tested in `crates/deckmaste_construction_v3_core`.
- Wildcard table rows already exist: `_` parses as `TablePattern::Any` (`parse.rs:404`), lowers at `crates/deckmaste_construction_v3_core/src/ir.rs:665` and is evaluated first-matching-row in `runtime.rs.txt:318`. This is conversion, not a compiler change: 216 tables span 1,288 lines (about 1,207 arms), and a 75-row table such as `coordinated_property_value_position` (only additive first plus second yields a pair, every other row yields none) shrinks to a few rows. The complete domain is preserved, so a missing row must not reject valid coordination.
- Remove unread interface slots from the declarations, by this criterion: a (feature, category) interface slot is unread if no `require`, `agree` or table argument reads it, directly or through a forward to a slot that is read. At the review 89 of 714 slots were unread: `Polarity` sits in 16 category interfaces and is read from none (the one `require head.Polarity` at `declarations.rs:854` reads a lexical verb's feature, not an interface slot). The same pass removes the 2 policies and 2 tables nothing uses. The inventory is produced by a script run at the start, listed in the landing, and removed slots may not change admission.
- Convert all of `declarations.rs` to the new forms, class by class. Remove every declaration the conversion makes unused. Report the lines saved per declaration kind.

## Gate

Every `readings_sha256` identical on every face after each class of conversion and at the end, checked with the census check mode as an identical-digest landing. The landing updates `crates/deckmaste_construction_v3_core/README.md` for the completed compiler capabilities. Note: census delta (none); the judgment-set line does not apply yet.


## Completed prefix

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

Remaining, in order: Boolean requirements and policy composition, with
compiler tests and README updates; wildcard table conversion; then unused
PurposeConcord, PurposeHostProjection and coordinate_number and the broader
dead-declaration pass, including unread slots one feature at a time. Boolean
absence semantics await a ruling: unknown with three-valued connectives,
rejection on any absent input, or treating an absent comparison as false.
No compiler work proceeds across that authoring-model decision.

Census delta (`nrtousno`, prefix): faces read 30,794 → 30,794; total Readings 482,041 → 482,041; lost faces: none.
Judgment set (`nrtousno`): does not apply yet.
