# English clauses are structural

## Decision

`deckmaste_english` parses Oracle text into strict grammatical clause and
predicate shapes before any translation into Magic semantics or runnable card
RON. Clause dependency, clause form, predicate valency, voice, modality, and
attachment scope remain explicit rather than being inferred later from Rust
storage layout or an untyped list of verb dependents.

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

Independent clauses expose four semantic owners. Ordinary finite clauses use
one canonical subject/predicate-expression representation:

~~~rust
pub enum IndependentClause {
    Finite(FiniteClause),
    Existential(ExistentialClause),
    Complex(ComplexClause),
    Coordinated(CoordinatedIndependentClause),
}

pub struct FiniteClause {
    subject: Option<Subject>,
    predicate: PredicateExpression,
}

pub enum PredicateExpression {
    Simple(Predicate),
    Coordinated(Coordination<PredicateExpression>),
}
~~~

`FiniteClause::subject` is `None` for a subjectless imperative and `Some` for
an overt grammatical subject. Valency and voice live in the simple predicate;
shared-subject coordination lives in the recursive predicate expression. A
modal is likewise a predicate-layer scope rather than a parallel finite-clause
variant. Thus `You may draw a card` is one finite clause whose subject is
`you` and whose simple predicate is a deontic predicate governing a transitive
`draw` predicate.

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
    Deontic(DeonticPredicate),
    Attached(AttachedPredicate),
}
~~~

The finite owner does not duplicate these complement shapes. A transitive
predicate requires a direct object. An intransitive predicate cannot acquire
one through generic dependent recovery. A copular complement is distinct from
a direct object. Passive voice promotes the patient to clause subject and does
not masquerade as either an intransitive verb or a copular participial
adjective. A deontic predicate holds its modal plus an optional recursively
compositional predicate expression; absence represents verbal ellipsis. An
attached predicate owns one locally scoped attachment edge whose scope ends
before the next coordinated predicate member.

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
    Gerund(GerundClause),
}

pub enum SubordinateBody {
    Finite(Box<IndependentClause>),
    Infinitive(InfinitiveClause),
    Gerund(GerundClause),
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

A complex independent clause owns exactly one recursive attachment edge:

~~~rust
pub struct AttachmentScope<H> {
    host: Box<H>,
    attachment: Box<ClauseAttachment>,
}

pub struct ComplexClause {
    scope: AttachmentScope<IndependentClause>,
}

pub struct AttachedPredicate {
    scope: AttachmentScope<Predicate>,
}
~~~

Wrapping another `ComplexClause` records which edge is outermost without a
flat attachment vector or construction-history rule. The same edge topology
at a predicate member keeps a trailing condition local when a following
predicate shares the finite subject. Position and measured punctuation remain
on the typed attachment for exact rendering. Relative clauses instead belong
to the noun phrase they modify.

A relative clause stores its relativizer, including a zero relativizer, and a
typed body. Its gap is derived from whether that body is `SubjectGap` or
`ObjectGap`; there is no duplicate gap field. In `target permanent that
opponent controls`, `that` is the demonstrative determiner of `opponent`, so
the relative clause has a zero relativizer and its object-gap body determines
the gap.

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
- [English coordination is structural](english-coordination-is-structural.md)
- [Macro templates are bidirectional](macro-templates-are-bidirectional.md)
