---
needs: [english-derived-family-inventory, english-coordination-structural-design, english-derived-nominal-family, english-derived-predicate-family]
---
**Derive the remaining phrase-coordination and consumer rows.**

Migrate C01 from `docs/english-derived-family-inventory.md`:
`modifier_conjunct_adjective`, `modifier_conjunct_noun`,
`modifier_conjunct_negated`, `modifier_list_single`, `modifier_list_comma`,
`coordinated_modifier_conjoined`, `coordinated_modifier_oxford`,
`nominal_coordinated_modifier`, `prepositional_phrase_list_pair`,
`prepositional_phrase_list_comma`,
`prepositional_phrase_sibling_coordinated`,
`verb_phrase_coordinated_adjective`,
`nominal_power_toughness_complement`.

Use the ordinary fan-out-one Chart backend required by
[`english-coordination-is-structural`](../../decisions/english-coordination-is-structural.md).
Shared determiners, nominal heads, and prepositions are explicit owners; each
member and its attachments are contiguous. Use the compiler's direct sums,
products, nonempty/separated sequences, typed subtree aliases, and generated
projection. Add only still-missing generic feature flow, named sequence-member
predicates, presence-valued comma, lenses, and total own-mode linearization.
Do not add tuple-valued yields or a coordination-only compiler callback.
The Chart/backend conclusion remains applicable; where the decision's older
member-type or common-head descriptions conflict with the bracketings below,
this amended implementation ticket supersedes them.

Implement the structural-design output here. Add a dedicated `with`-attribute
member sum containing `KeywordAbility` and `QuotedAbility`, admitted only under
attributive `with` and only for a genuinely mixed list; do not widen
`PredicateObject` or generic P02 objects.

Select common-head and shared-determiner head-list readings from recursive
grammatical constituency, not from MTG-semantic modifier classes. Coordination
members occupy one grammatical role but may have different internal shapes.
When one overt head follows a coordinated modifier constituent, build that
coordination beneath the shared head. When every member contains its own overt
head, coordinate the complete headed nominals beneath the shared determiner.
Preserve outer complements and postmodifiers on the completed owner. In
particular, require these bracketings:

- `an <<<Elf>, <Orc>, or <enchantment>> creature <you control>>`;
- `a <<<basic> <land> card> or <<Gate> card>>`;
- `<a <<<basic>, <Sphere>, or <Locus>> land> card>`;
- `<target <<artifact>, <enchantment>, or <<tapped> creature>>>`; and
- `<a <<Mutant>, <Ninja>, <Turtle>, or <land>> card>`.

If more than one holistic grammatical bracketing survives, retain every tied
alternative instead of adding a semantic taxonomy, spelling gate, registration
preference, or broad parse cost. Repair generated P01 admission/selection for
`that many cards minus one` as the independent Sycorax acceptance prerequisite;
arithmetic does not become a coordination member. None remains an exception.

Land declarations and generated parse/reduction/lowering, total render, and
validated build projections; Nominal/Sentence/Ability plus every spelling/view
consumer; direct-AST and `inspect` grouping/attachment fixtures; exact comma/
conjunction replay and illegal-list negatives; and all 13 handwritten registry
flips. Delete every NR registration and phrase-side reducer/lowerer arm, the CR
reducer/lowerer arm for `verb_phrase_coordinated_adjective`, every renderer
mirror, and every constructor bypass. The already-generated F02
`copular_remainder_coordinated_adjective` and R01
`relative_contracted_copular_coordinated_adjective` rows remain owned by those
families; migrate their typed consumer boundary only if the coordinated
adjective carrier changes. Update the inventory. No consumer or deletion work
is deferred.

Acceptance fixtures include Alien Invasion, Basilica Shepherd, and Blink;
Abzan Monument or Deceptive Landscape; Grassland Crusader; Open the Gates or
District Guide; Monument to Perfection; Banishing Slash or Summon: Yojimbo;
Cowabunga! or Kirri, Talented Sprout; and the synthetic `an Elf, Orc, or
enchantment creature you control` grouping control. Negative fixtures reject
the heterogeneous member sum after non-`with` prepositions, a member that
cannot occupy the required grammatical role, binary Oxford punctuation, and a
group-level relative captured by the final member. They must not reject a
grammatical common-head phrase merely because its modifiers belong to different
MTG semantic classes.

Standard constraints apply.
