# English clauses are structural

## Decision

`deckmaste_english` parses Oracle text into strict grammatical clause and
predicate shapes before any translation into Magic semantics or runnable card
RON. Clause dependency, clause form, predicate valency, voice, modality, and
attachment scope remain explicit rather than being inferred later from a
generic subject plus a list of verb dependents.

The English parser accepts syntactically valid combinations even when they are
meaningless as Magic rules or unsupported by semantic lowering. For example,
`As long as you may draw a card` is a valid subordinate clause. Rejecting or
interpreting that combination belongs to a later Magic-aware layer.

## Clause hierarchy

The public syntax distinguishes independent and dependent clauses:

~~~rust
pub enum Clause {
    Independent(IndependentClause),
    Dependent(DependentClause),
}
~~~

Independent clauses expose their construction directly:

~~~rust
pub enum IndependentClause {
    Transitive(Subject, TransitivePredicate),
    Intransitive(Subject, IntransitivePredicate),
    Copular(Subject, CopularPredicate),
    Imperative(Predicate),
    Deontic(Subject, Modal, Predicate),
    Existential(ExistentialForm, NounPhrase),
    Proform(Subject, ProPredicate),
    Complex(ComplexClause),
    Coordinated(CoordinatedIndependentClause),
}
~~~

The exact payload structs may gain corpus-motivated fields, but these
distinctions do not collapse back into `subject: Option<_>` or an unrestricted
dependent list. An imperative has an implicit second-person subject. A deontic
clause owns its overt subject and modal while its predicate is in the selected
bare form. Thus `You may draw a card` has the conceptual shape:

~~~rust
IndependentClause::Deontic(
    Subject::You,
    Modal::May,
    Predicate::Transitive(TransitivePredicate {
        verb: Verb::Draw,
        object: NounPhrase::Indefinite(
            IndefiniteArticle::A,
            NounInstance::Singular(Noun::Word(Vocab::Card)),
        ),
    }),
)
~~~

An existential is not a subjectless ordinary verb phrase. It owns the
existential surface form and its pivot noun phrase. `There is`, `there's`, and
`there are` remain distinguishable because number alone cannot regenerate the
source contraction.

## Predicates

Predicate variants describe the complement structure required by their slot:

~~~rust
pub enum Predicate {
    Transitive(TransitivePredicate),
    Intransitive(IntransitivePredicate),
    Copular(CopularPredicate),
    Passive(PassivePredicate),
    Proform(ProPredicate),
}
~~~

The same payload structs are reused by direct independent-clause variants and
by wrappers such as `Imperative` and `Deontic`; their internal representation
is not duplicated. A transitive predicate requires a direct object. An
intransitive predicate cannot acquire one through generic dependent recovery.
A copular complement is distinct from a direct object. Passive voice promotes
the patient to clause subject and does not masquerade as either an intransitive
verb or a copular participial adjective.

A pro-predicate such as `do` in `if you do` remains explicit. It refers to an
antecedent predicate without claiming that the elided predicate was transitive
or intransitive.

Additional complements and adjuncts are typed by their grammatical role. For
example, `this combat` in `blocks this combat` is a temporal adjunct rather
than the direct object of `blocks`.

## Dependency and attachment

Subordination is a dependent-clause construction, not a sibling of
transitive, imperative, deontic, or existential independent clauses:

~~~rust
pub enum DependentClause {
    Subordinate(Subordinator, SubordinateBody),
    Relative(RelativeClause),
    Infinitive(InfinitiveClause),
    // Further forms require supported-corpus evidence.
}

pub enum SubordinateBody {
    Finite(Box<IndependentClause>),
    Elliptical(EllipticalClause),
}
~~~

A finite subordinate body may contain any independent clause form. This makes
both `as long as there's a Lesson card` and the syntactically valid `as long as
you may draw a card` ordinary compositions. Elliptical bodies cover forms such
as `if able` without inventing a copied-text subordinator payload.

A dependent clause is never a sentence root by itself. Sentence grammar
requires an independent clause, possibly a complex one that owns dependent
attachments.

A complex independent clause owns a matrix clause and one or more positioned
dependent attachments. Dependents attach at the narrowest owning syntax node:
clause-level conditions belong to the complex clause, predicate conditions
belong to the predicate, and relative clauses belong to the noun phrase they
modify. Position is retained for exact rendering, but a detached `scope` flag
does not substitute for tree structure.

Relative clauses record their relativizer, including a zero relativizer, and
their gap. In `target permanent that opponent controls`, `that` is the
demonstrative determiner of `opponent`; the relative clause has a zero
relativizer and an object gap.

## Magic-aware boundaries

Intervening-if is not a special English sentence or subordinate-clause
production. The English grammar produces an ordinary
`DependentClause::Subordinate(Subordinator::If, ...)` with its attachment
preserved. Triggered-ability lowering recognizes an if-clause in the
intervening position and exposes it as the trigger's intervening condition.
This keeps the important Magic distinction first-class without encoding Magic
rules semantics in general English syntax.

The same boundary applies to later RON translation. English parsing determines
subjects, objects, modality, voice, gaps, and attachment. Magic lowering maps
those already-structured roles into game concepts and may report unsupported
semantics; it does not parse the text again.

## Forest identity and recovery

Completed forest nodes pack only grammatically and semantically
interchangeable alternatives. Strict clause and predicate meanings become the
forest identity, replacing shallow `Sentence` and `Clause` meanings whose
correctness currently depends on a recursively computed `shape: u64` hash.
The shape hash is removed only after the typed meanings distinguish every
alternative observable by an enclosing production. Replacing it with a
constant is unsound.

Unknown recovery does not select a part of speech or clause shape merely
because an opaque leaf can make a derivation complete. General opaque recovery
is limited to unknown nouns and whole unsupported sentences. Costs and other
deliberately restrictive slots may define their own explicit recovery leaves.
`Clause` and `Predicate` have no generic unknown variant, so grammar errors
remain visible instead of being hidden inside arbitrary verb dependents.

After strict meanings replace shape hashing, the parser is profiled again.
Recursive semantic hashing is expected to disappear as a major cost, but
allocation work is optimized only from new measurements.

## Corpus fixtures

Tests use exact supported Oracle text rather than synthetic approximations.
At minimum the strict clause work covers:

- Aang, A Lot to Learn: `as long as there's a Lesson card in your graveyard`
  is a subordinate existential with an indefinite singular pivot and a
  graveyard constraint.
- Keeper of Keys: `At the beginning of your upkeep, if you're the monarch,
  creatures you control can't be blocked this turn.` exercises a trigger,
  subordinate copular condition, deontic negation, passive voice, and a
  temporal adjunct.
- Karmic Justice: `Whenever a spell or ability an opponent controls destroys a
  noncreature permanent you control, you may destroy target permanent that
  opponent controls.` exercises nested relative clauses, object gaps,
  transitivity, and a deontic transitive consequence.

Fixtures render without access to source text or spans. The supported corpus
continues to mean cards whose derived data marks them Vintage Legal or
Restricted. Unsupported cards do not motivate grammar or acceptance cases.

## Consequences

The grammar gains more nonterminals and reducers, but each production has
stronger local constraints and produces fewer meaningless chart alternatives.
Renderer and future RON lowering become exhaustive structural matches rather
than heuristic searches through generic dependents. Unsupported syntax fails
at a named boundary, and corpus-driven additions extend an explicit enum or
payload instead of weakening the grammar.

## Tracked references

- [Authored card surface](authored-card-surface.md)
- [Macro templates are bidirectional](macro-templates-are-bidirectional.md)
