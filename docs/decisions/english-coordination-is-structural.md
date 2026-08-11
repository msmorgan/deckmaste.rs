# English coordination is structural

## Decision

English coordination is a recursive, one-span construction over a typed
grammatical role. A coordination layer is homogeneous in that role, but its
member type may be a closed sum when different syntax categories can fill the
same role. Shared surface material belongs to one explicit host outside the
member sequence; it is not copied into the members or recovered from a flat
serialization layout.

This makes the remaining phrase and clause coordination families ordinary
`ConstructionBackend::Chart` work. They do not require tuple-valued yields or
a PMCFG backend. Each member and each locally attached dependent occupies one
contiguous source span after the shared determiner, head, subject, copula, or
preposition is represented by its actual owner.

## Owners and member roles

The same structural rule applies at each grammatical boundary:

| surface family | shared owner | coordinated member role |
|---|---|---|
| common-head nominal, such as `artifact or creature card` | nominal determiner, noncoordinated modifiers, head, and group complements | one attributive modifier |
| shared-determiner head list, such as `a basic land card or Gate card` | determiner and group complements | one complete nominal phrase |
| mixed `with` list, such as `with flying and "..."` | the `with` prepositional phrase | one `WithAttributeMember` |
| shared-subject predicates | finite subject and agreement | one predicate expression |
| complete-clause coordination | the enclosing sentence or subordinate body | one independent clause |
| repeated prepositional phrases | the enclosing complement or adjunct slot | one complete prepositional phrase |

`Coordination<T>` remains n-ary within one layer. A member may itself be a
coordination, so grouping remains explicit: Sycorax Commander's `A, then B, or
C` is an outer `or` coordination whose first complete-clause member contains a
shared-subject `then` predicate coordination. A flat list must not erase that
grouping. A homogeneous Oxford run may remain one n-ary layer, including its
asyndetic interior junctions.

Shared-subject coordination uses the existing canonical clause shape:

~~~rust
FiniteClause {
    subject: Some(subject),
    predicate: PredicateExpression::Coordinated(
        Coordination<PredicateExpression>,
    ),
}
~~~

Complete members use `IndependentClause::Coordinated`. The distinction is
semantic and invertible from the checked AST; a renderer never has to infer a
missing subject from source text. Shared-copular continuations likewise become
ordinary copular predicate members under the finite subject.

## Typed heterogeneous members

Heterogeneous coordination is local to the grammatical slot that admits it.
It is not a reason to widen `Phrase`, `PredicateObject`, or a prepositional
object to an unconstrained catch-all.

Mixed keyword/quoted-ability `with` lists use a dedicated closed member sum,
conceptually:

~~~rust
enum WithAttributeMember {
    Nominal(NounPhrase),
    QuotedAbility(QuotedAbility),
}
~~~

The coordination is admitted only as the object of `with` and only when both
member classes occur. Keyword-only lists continue through their existing
typed nominal coordination, while a singular quoted ability keeps its existing
ability-postmodifier path. This prevents the new production from admitting
the same mixture after `of`, `from`, or another preposition, and avoids reusing
the much broader verb-object sum.

## Member attachment and grouping

An attachment whose source span ends before a coordination junction belongs
to that member. Predicate-local dependents therefore wrap the simple
predicate before it enters `PredicateExpression::Coordinated`. Tek's five
conditions are five `Predicate::Attached` members; Chaos Mutation's `until`
condition stays on the reveal member before the following `puts` members.

A trailing attachment after the final member can genuinely denote either the
last member or the whole coordination. When syntax supplies no discriminator,
both readings remain packed. The parallel Tek shape does supply one: preceding
members at the same coordination layer each carry the same kind of trailing
condition. The construction that requires every member to own that attachment
dominates the competing reading that moves only the final condition outside
the coordination. This is a local structural guard, not a global attachment
cost.

Mixed connectives preserve recursive grouping. `A, B, then C` is one ordered
n-ary predicate layer; `(A then B) or C` is two layers. Complete-clause and
shared-subject members are never flattened into each other merely because
their rendered bytes are similar.

## Common head versus head list

Common-head eligibility is determined by a typed attributive coordination
class carried by each modifier and by the trailing head's declared modifier
frame. This class is deliberately distinct from `CoordinationDomain`:
`CoordinationDomain` says whether meanings may coordinate, while the new class
says whether members fill the same surface modifier role for one head.

Examples of distinct classes include adjectival or participial properties,
colors, card types, creature subtypes, basic land types, and keyword-ability
names. A common-head construction is eligible only when every member has the
same attributive class and the head admits that class. Thus:

- `Plains, Swamp, or Forest card`, `artifact and/or creature card`, `instant
  or sorcery spell`, and `first strike, vigilance, or lifelink counter` keep a
  common head;
- `Elf or Soldier creature` and `attacking or blocking creature` keep a common
  head because their members are respectively uniform subtypes and uniform
  participial modifiers;
- `an Elf, Orc, or enchantment creature` is a shared-determiner head list:
  subtype members and a card-type modifier do not form one attributive class;
- `a basic land card or Gate card` is a head list because both conjuncts carry
  their own overt `card` head.

The head frame and member classes are lexical or catalog features propagated
through construction roles. They are not spelling checks. If two distinct
structural readings remain equally valid after these guards, the forest keeps
both; registration order and broad parse costs do not choose between them.
Group-level relatives and other complements attach to the resulting common
nominal or head-list owner, never to whichever final member happened to close
the parse.

## Exactness and implementation boundary

Conjunction class, recursive grouping, member attachment, and shared context
are semantic. A comma remains stored where the existing measured exactness
law cannot derive it, notably clause and predicate junctions. Modifier and
prepositional Oxford commas remain derived only where the existing corpus law
is exact. No family may replace either rule with a source substring check.

The construction compiler already supplies the direct sums, products,
nonempty/separated sequences, typed subtree aliases, and generated projections
needed for these owners. C01 may add only the remaining generic feature flow,
sequence-member predicates, and lenses needed to state its invariants. F04
reuses that machinery on the ordinary Chart backend. Neither migration adds a
tuple-yield backend.

The arithmetic in `that many cards minus one` is not coordination. Sycorax
Commander acceptance may repair the generated P01 `noun_phrase_minus`
admission or selection path, but the arithmetic stays a noun-phrase subtree
inside the draw predicate. Dominaria's Judgment remains separate conditioned
keyword-argument/ellipsis work.

## Acceptance evidence

The vertical migrations retain exact, source-free render/reparse and reverse-
registration coverage. At minimum they pin:

- Alien Invasion, Basilica Shepherd, and Blink for mixed `with` members;
- Tek for five locally conditioned predicates;
- Chaos Mutation for a shared-subject asyndetic/`then` chain;
- Sycorax Commander for nested `then`/`or` grouping and the independent P01
  arithmetic subtree;
- Abzan Monument or Deceptive Landscape, Grassland Crusader, and the existing
  artifact/creature, instant/sorcery, keyword-counter, and participial
  common-head controls;
- Open the Gates or District Guide plus the heterogeneous synthetic
  `Elf, Orc, or enchantment creature` control for head lists.

Negative gates reject mixed `with` members under another preposition,
ill-formed/binary Oxford punctuation, incompatible attributive classes in a
common-head owner, invalid agreement after a shared subject, and any builder
that loses a member-local attachment or nested connective. `inspect` exposes
the selected construction, member roles, and attachment owner rather than a
serialized Rust layout.

## Consequences

C01 owns the remaining phrase-coordination carriers and the local nominal/P02
selection changes. F04 then derives clause coordination over the same checked
sequence and junction machinery. The current handwritten migration surface is
13 C01 rows plus six F04 rows. Generated F02 and R01 coordinated-adjective
consumer rows remain owned by those families and are not counted again.

PMCFG remains the project escape hatch for a future construction whose member
is still genuinely discontinuous after shared context is represented
explicitly. None of the coordination families decided here meets that
criterion.

## Tracked references

- [English clauses are structural](english-clauses-are-structural.md)
- [English grammar is derived](english-grammar-is-derived.md)
- [English productions ship their inverse](english-productions-ship-their-inverse.md)
- [Oracle text style guide](../oracle-style-guide.md)
