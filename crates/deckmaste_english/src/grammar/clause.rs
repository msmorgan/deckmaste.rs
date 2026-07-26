#[allow(
    clippy::wildcard_imports,
    reason = "module uses generated imports and shared grammar aliases"
)]
use super::*;
use crate::syntax::AbilityObject;
use crate::syntax::AttachmentPosition;
use crate::syntax::ClauseAttachment;
use crate::syntax::ClauseAttachmentKind;
use crate::syntax::ClauseCoordination;
use crate::syntax::ComplexClause;
use crate::syntax::CoordinatedClauseMember;
use crate::syntax::CoordinatedIndependentClause;
use crate::syntax::CoordinatedPredicateObject;
use crate::syntax::DependentAttachment;
use crate::syntax::DependentClause;
use crate::syntax::EllipticalClause;
use crate::syntax::ExceptionConjunct;
use crate::syntax::ExceptionRider;
use crate::syntax::IndependentClause;
use crate::syntax::InfinitiveMarker;
use crate::syntax::Modal;
use crate::syntax::ObjectGapPredicate;
use crate::syntax::PassivePredicate;
use crate::syntax::Predicate;
use crate::syntax::PredicateAdjunct;
use crate::syntax::PredicateComplement;
use crate::syntax::PredicateElement;
use crate::syntax::PredicateHead;
use crate::syntax::PredicateObject;
use crate::syntax::PredicateObjectCoordination;
use crate::syntax::PreverbModifier;
use crate::syntax::ProPredicate;
use crate::syntax::RelativeBody;
use crate::syntax::RelativeMarker;
use crate::syntax::RestrictionCoordination;
use crate::syntax::RestrictionRun;
use crate::syntax::SentenceBody;
use crate::syntax::SubordinateBody;

#[allow(
    clippy::too_many_lines,
    reason = "grammar table builder is intentionally verbose"
)]
pub(super) fn add_rules(builder: &mut RuleBuilder) {
    use EnglishLexicalSlot as L;
    use Expected::Lexical as l;
    use Expected::Nonterminal as n;
    use Nonterminal as N;

    builder.add(
        RuleTag::FrequencyPhrase,
        N::FrequencyPhrase,
        [l(L::Frequency)],
    );
    builder.add(
        RuleTag::FrequencyPhraseAdverb,
        N::FrequencyPhrase,
        [l(L::Adverb), l(L::Frequency)],
    );

    for slot in crate::word::VERB_SLOTS {
        builder.add(RuleTag::Verb, N::Verb, [l(L::Verb(slot))]);
    }
    builder.add(RuleTag::VerbPhraseBase, N::VerbPhrase, [n(N::Verb)]);
    builder.add(
        RuleTag::VerbPhraseAuxiliary,
        N::VerbPhrase,
        [l(L::Auxiliary), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::VerbPhraseAuxiliaryProform,
        N::VerbPhrase,
        [l(L::Auxiliary)],
    );
    builder.add(
        RuleTag::VerbPhraseDirectObject,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::NounPhrase)],
    );
    builder.add(
        RuleTag::VerbPhraseIndirectObject,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::NounPhrase)],
    );
    builder.add_with_cost(
        RuleTag::VerbPhraseAdjective,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::AdjectivePhrase)],
        ParseCost {
            precedence: 2,
            ..ParseCost::default()
        },
    );
    builder.add_with_cost(
        RuleTag::VerbPhrasePrepositional,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::PrepositionalPhrase)],
        ParseCost {
            precedence: 1,
            ..ParseCost::default()
        },
    );
    builder.add(
        RuleTag::VerbPhraseInfinitive,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::InfinitiveClause)],
    );
    builder.add(
        RuleTag::VerbPhraseAdverb,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::Adverb)],
    );
    builder.add(
        RuleTag::VerbPhrasePreverbAdverb,
        N::VerbPhrase,
        [l(L::PreverbAdverb), n(N::VerbPhrase)],
    );
    for particle in [VerbParticle::In, VerbParticle::Out] {
        builder.add(
            RuleTag::VerbPhraseParticle,
            N::VerbPhrase,
            [n(N::VerbPhrase), l(L::VerbParticle(particle))],
        );
    }
    builder.add(
        RuleTag::VerbPhraseFrequency,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::FrequencyPhrase)],
    );
    builder.add(
        RuleTag::VerbPhraseAbility,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::AbilityItem)],
    );
    // A quoted ability fills the same grant-verb object slot as a keyword
    // ability (`… has "…"`), and coordinates as a two-conjunct object list
    // (`… has "A" and "B"`) exactly as symbol alternatives do. One general
    // production admits the quoted-ability conjunct wherever the grammar
    // already licenses a quoted-ability object; the surrounding predicate,
    // clause, and prepositional coordination is the existing machinery.
    builder.add(
        RuleTag::VerbPhraseQuotedAbility,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::QuotedAbility)],
    );
    builder.add(
        RuleTag::VerbPhraseQuotedAbilityCoordination,
        N::VerbPhrase,
        [
            n(N::VerbPhrase),
            l(L::QuotedAbility),
            l(L::Conjunction),
            l(L::QuotedAbility),
        ],
    );
    // A keyword-ability object coordinated with a quoted ability
    // (`… has flying and "…"`) — the mixed conjunct the noun-phrase coordination
    // that already handles `… has flying and haste` cannot form, since a quoted
    // ability is not a noun phrase.
    builder.add(
        RuleTag::VerbPhraseAbilityQuotedCoordination,
        N::VerbPhrase,
        [
            n(N::VerbPhrase),
            l(L::AbilityItem),
            l(L::Conjunction),
            l(L::QuotedAbility),
        ],
    );
    builder.add(
        RuleTag::VerbPhraseOracleSymbol,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::OracleSymbol)],
    );
    builder.add(
        RuleTag::VerbPhraseSymbolSequence,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::SymbolSequence)],
    );
    // Mana-amount coordination: a symbol-typed mirror of the generic
    // noun-phrase list machinery (`N::NounPhraseList` +
    // `NounPhraseCoordinationOxford`), never riding `NounPhrase` itself so no
    // symbol is ever licensed as a nominal. `ManaAmount` is a lone oracle
    // symbol or a contiguous symbol group; `ManaAmountList` is an open,
    // comma-separated run reached only by the list-extension and Oxford-close
    // rules; `CoordinatedManaAmount` is the closed run consumed only by the
    // verb-phrase attachment.
    builder.add(
        RuleTag::ManaAmountSymbol,
        N::ManaAmount,
        [l(L::OracleSymbol)],
    );
    builder.add(
        RuleTag::ManaAmountSequence,
        N::ManaAmount,
        [l(L::SymbolSequence)],
    );
    builder.add(
        RuleTag::ManaAmountListSingle,
        N::ManaAmountList,
        [n(N::ManaAmount)],
    );
    builder.add(
        RuleTag::ManaAmountListComma,
        N::ManaAmountList,
        [
            n(N::ManaAmountList),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::ManaAmount),
        ],
    );
    builder.add(
        RuleTag::ManaAmountCoordination,
        N::CoordinatedManaAmount,
        [n(N::ManaAmount), l(L::Conjunction), n(N::ManaAmount)],
    );
    builder.add(
        RuleTag::ManaAmountCoordinationOxford,
        N::CoordinatedManaAmount,
        [
            n(N::ManaAmountList),
            l(L::Punctuation(Punctuation::Comma)),
            l(L::Conjunction),
            n(N::ManaAmount),
        ],
    );
    builder.add(
        RuleTag::VerbPhraseManaAmountCoordination,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::CoordinatedManaAmount)],
    );
    builder.add(
        RuleTag::VerbPhrasePowerToughness,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::PowerToughness)],
    );
    builder.add(
        RuleTag::VerbPhraseQuantity,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::Quantity)],
    );

    builder.add(
        RuleTag::VerbPhraseBase,
        N::ObjectGapVerbPhrase,
        [n(N::Verb)],
    );
    builder.add(
        RuleTag::VerbPhraseAuxiliary,
        N::ObjectGapVerbPhrase,
        [l(L::Auxiliary), n(N::ObjectGapVerbPhrase)],
    );
    builder.add(
        RuleTag::VerbPhraseIndirectObject,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::NounPhrase)],
    );
    builder.add_with_cost(
        RuleTag::VerbPhraseAdjective,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::AdjectivePhrase)],
        ParseCost {
            precedence: 2,
            ..ParseCost::default()
        },
    );
    builder.add_with_cost(
        RuleTag::VerbPhrasePrepositional,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::PrepositionalPhrase)],
        ParseCost {
            precedence: 1,
            ..ParseCost::default()
        },
    );
    builder.add(
        RuleTag::VerbPhraseInfinitive,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::InfinitiveClause)],
    );
    builder.add(
        RuleTag::VerbPhraseAdverb,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), l(L::Adverb)],
    );
    for particle in [VerbParticle::In, VerbParticle::Out] {
        builder.add(
            RuleTag::VerbPhraseParticle,
            N::ObjectGapVerbPhrase,
            [n(N::ObjectGapVerbPhrase), l(L::VerbParticle(particle))],
        );
    }
    builder.add(
        RuleTag::VerbPhraseFrequency,
        N::ObjectGapVerbPhrase,
        [n(N::ObjectGapVerbPhrase), n(N::FrequencyPhrase)],
    );

    builder.add(
        RuleTag::InfinitiveTo,
        N::InfinitiveClause,
        [l(L::To), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::InfinitiveNotTo,
        N::InfinitiveClause,
        [l(L::Not), l(L::To), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::GerundClauseBase,
        N::GerundClause,
        [n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::GerundClauseSubordinateAfter,
        N::GerundClause,
        [n(N::GerundClause), l(L::RatherThan), n(N::GerundClause)],
    );
    builder.add(
        RuleTag::SimpleClauseSubject,
        N::SimpleClause,
        [n(N::NounPhrase), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::SimpleClauseContractedSubject,
        N::SimpleClause,
        [l(L::SubjectAuxiliary), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::SimpleClauseSubjectless,
        N::SimpleClause,
        [n(N::VerbPhrase)],
    );
    builder.add(RuleTag::ClauseSimple, N::Clause, [n(N::SimpleClause)]);
    builder.add(
        RuleTag::ClauseExistential,
        N::Clause,
        [l(L::Existential), n(N::NounPhrase)],
    );
    builder.add_with_cost(
        RuleTag::CopularRemainderNoun,
        N::CopularRemainder,
        [n(N::NounPhrase)],
        ParseCost {
            precedence: 1,
            ..ParseCost::default()
        },
    );
    builder.add(
        RuleTag::CopularRemainderAdjective,
        N::CopularRemainder,
        [n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::CopularRemainderPrepositional,
        N::CopularRemainder,
        [n(N::PrepositionalPhrase)],
    );
    builder.add(
        RuleTag::CopularRemainderAdverb,
        N::CopularRemainder,
        [l(L::Adverb), n(N::CopularRemainder)],
    );
    // The distributive floating `each` of a characteristic-defining copular
    // (`Rosie's power and toughness are each equal to <measure>`). Requiring the
    // `each` token keeps this frame off the non-`each` singular/plural forms,
    // which stay on the intransitive `be` + adjective + prepositional-adjunct
    // analysis. The adjective and its `to`-standard are taken apart here and
    // rejoined as an `equal to X` adjective phrase (prepositional complement).
    builder.add(
        RuleTag::CopularRemainderDistributiveEach,
        N::CopularRemainder,
        [
            l(L::EachDeterminer),
            n(N::AdjectivePhrase),
            n(N::PrepositionalPhrase),
        ],
    );
    builder.add(
        RuleTag::ClauseCopular,
        N::Clause,
        [n(N::NounPhrase), l(L::Copula), n(N::CopularRemainder)],
    );
    builder.add(
        RuleTag::ClauseContractedCopular,
        N::Clause,
        [l(L::SubjectAuxiliary), n(N::CopularRemainder)],
    );
    // A variable's value constraint under a modal (`X can't be 0`): four
    // literal-token slots (no `Noun`, `Verb`, `Adjective`, or `NounPhrase`
    // nonterminal anywhere in the production), one per numeral notation so
    // `X can't be 5`, `X can't be fifth`, `X can't be V`, etc. all reach the
    // same shape.
    for notation in [
        Numeral::Cardinal,
        Numeral::Ordinal,
        Numeral::Arabic(false),
        Numeral::Arabic(true),
        Numeral::Roman,
    ] {
        builder.add(
            RuleTag::ClauseVariableValueConstraint,
            N::Clause,
            [
                l(L::QuantityX),
                l(L::Auxiliary),
                l(L::Auxiliary),
                l(L::Number(notation)),
            ],
        );
    }
    builder.add(
        RuleTag::ClauseElliptical,
        N::Clause,
        [n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::ClauseCoordination,
        N::Clause,
        [n(N::Clause), l(L::Conjunction), n(N::SimpleClause)],
    );
    builder.add(
        RuleTag::ClauseCoordinationComma,
        N::Clause,
        [
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            l(L::Conjunction),
            n(N::SimpleClause),
        ],
    );
    builder.add(
        RuleTag::ClauseCoordinationAsyndetic,
        N::Clause,
        [
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::SimpleClause),
        ],
    );
    builder.add(
        RuleTag::ClauseSubordinateBefore,
        N::Clause,
        [
            l(L::Subordinator),
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ClauseAdverbBefore,
        N::Clause,
        [l(L::Adverb), n(N::Clause)],
    );
    builder.add(
        RuleTag::ClauseSentenceAdverbialBefore,
        N::Clause,
        [
            l(L::SentenceAdverbial),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ClausePrepositionalBefore,
        N::Clause,
        [
            n(N::PrepositionalPhrase),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ClauseSubordinateAfterElliptical,
        N::Clause,
        [n(N::Clause), l(L::Subordinator), n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::ClauseSubordinateAfter,
        N::Clause,
        [n(N::Clause), l(L::Subordinator), n(N::Clause)],
    );
    builder.add(
        RuleTag::ClauseSubordinateAfterComma,
        N::Clause,
        [
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            l(L::Subordinator),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ClauseSubordinateAfterInfinitive,
        N::Clause,
        [n(N::Clause), l(L::RatherThan), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeObject,
        N::RelativeClause,
        [n(N::NounPhrase), n(N::ObjectGapVerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeObjectContractedSubject,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::ObjectGapVerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeSubjectContractedAuxiliary,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeSubject,
        N::RelativeClause,
        [l(L::RelativeMarker), n(N::VerbPhrase)],
    );
    builder.add(
        RuleTag::RelativeContractedCopularNoun,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::NounPhrase)],
    );
    builder.add(
        RuleTag::RelativeContractedCopularAdjective,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::AdjectivePhrase)],
    );
    builder.add(
        RuleTag::RelativeContractedCopularPrepositional,
        N::RelativeClause,
        [l(L::SubjectAuxiliary), n(N::PrepositionalPhrase)],
    );

    // Both rules carry the same tag: the terminal period is derivable from the
    // sentence's structure at render time, so the parse need not record whether
    // it was present. The period-consuming rule simply discards the token.
    builder.add(
        RuleTag::Sentence,
        N::Sentence,
        [n(N::Clause), l(L::Punctuation(Punctuation::Period))],
    );
    builder.add(RuleTag::Sentence, N::Sentence, [n(N::Clause)]);

    // The productions below are appended after every pre-existing rule so their
    // rule indices are the highest in the grammar. The forest's equal-cost
    // tiebreak prefers the lowest rule index, so a new production that competes
    // with an existing analysis for the same span (a copular prepositional
    // adjunct against a noun-internal complement) loses the tie, leaving every
    // already-clean parse untouched.

    // `it's 7/7` — a power/toughness copular complement.
    builder.add(
        RuleTag::CopularRemainderPowerToughness,
        N::CopularRemainder,
        [l(L::PowerToughness)],
    );
    // A trailing prepositional adjunct on a copular predication (`it's legendary
    // in addition to its other types`). The intransitive `become`/`be` path
    // already admits this adjunct through the verb phrase; recording it on the
    // copular remainder closes the same gap for the copular clause. It is a
    // last resort (high precedence): whenever the preposition can attach to a
    // verb, adjective, or noun instead — a passive (`it's put into exile`), an
    // adjective standard (`its power is equal to X`), or a noun complement (`an
    // Illusion in addition to …`) — that analysis wins. A genuine copular
    // adjunct on a bare supertype/color adjective has no such competitor and is
    // the only complete parse, so it wins despite the cost.
    builder.add_with_cost(
        RuleTag::CopularRemainderPrepositionalAdjunct,
        N::CopularRemainder,
        [n(N::CopularRemainder), n(N::PrepositionalPhrase)],
        ParseCost {
            precedence: 8,
            ..ParseCost::default()
        },
    );
    // The causative `have <object> <bare-infinitive VP>` construction. The first
    // verb phrase is a causative verb (`have`) that has already taken its object
    // (the causee); the second is a complete bare-infinitive verb phrase. The
    // `causative_complement` frame flag, set only on `have`, keeps every other
    // `VerbPhrase VerbPhrase` adjacency from reducing.
    builder.add(
        RuleTag::VerbPhraseCausative,
        N::VerbPhrase,
        [n(N::VerbPhrase), n(N::VerbPhrase)],
    );
    // The exception rider: a leading `except` marker heading a coordinated list
    // of finite clauses. Coordination lives on the dedicated `ExceptionRider`
    // nonterminal so a copular or possessive-subject conjunct (`it's legendary`,
    // `its name is X`) — which the general clause coordination cannot form as a
    // non-first member — joins an Oxford list here.
    builder.add(
        RuleTag::ExceptionRiderSingle,
        N::ExceptionRider,
        [l(L::Except), n(N::Clause)],
    );
    builder.add(
        RuleTag::ExceptionRiderConjoined,
        N::ExceptionRider,
        [n(N::ExceptionRider), l(L::Conjunction), n(N::Clause)],
    );
    builder.add(
        RuleTag::ExceptionRiderComma,
        N::ExceptionRider,
        [
            n(N::ExceptionRider),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ExceptionRiderOxford,
        N::ExceptionRider,
        [
            n(N::ExceptionRider),
            l(L::Punctuation(Punctuation::Comma)),
            l(L::Conjunction),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ClauseExcepted,
        N::Clause,
        [
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::ExceptionRider),
        ],
    );

    // A trailing run of two or more coordinated `only …` timing restrictions.
    // The member nonterminal admits exactly the shapes attested in the
    // corpus (verified by probe, `restrict-members.txt`): a prepositional
    // restriction (`only as a sorcery`, `only during your turn`), an `if`
    // restriction (`only if <clause>` — the first live construction of
    // `PredicateAdjunct::Dependent`), and the flat `once`/`once each turn`
    // frequency pair, which the existing grammar already lowers as a bare
    // adverb optionally followed by a `Temporal` noun phrase rather than as
    // one combined nonterminal (`restrict-probe2b.txt`) — hence the member
    // carries a small `Vec<PredicateAdjunct>`, not a single adjunct.
    builder.add(
        RuleTag::ClauseRestrictionMember,
        N::RestrictionMember,
        [l(L::Adverb), n(N::PrepositionalPhrase)],
    );
    builder.add(
        RuleTag::ClauseRestrictionMember,
        N::RestrictionMember,
        [l(L::Adverb), l(L::Subordinator), n(N::Clause)],
    );
    builder.add(
        RuleTag::ClauseRestrictionMember,
        N::RestrictionMember,
        [l(L::Adverb), l(L::Adverb)],
    );
    builder.add(
        RuleTag::ClauseRestrictionMember,
        N::RestrictionMember,
        [l(L::Adverb), l(L::Adverb), n(N::NounPhrase)],
    );
    // The run itself: a two-member base joined by a bare `and` (the only
    // licensed two-member spelling — a two-member Oxford comma is not
    // attested and must keep failing, §6 test 11), then asyndetic-comma and
    // Oxford-comma growth, mirroring `ExceptionRider`'s fold shape. Finally,
    // the closed run attaches `AfterMatrix` onto a complete host clause. All
    // five productions share this tag; `reduce_clause`/`lower_clause`
    // disambiguate by arity and child type.
    builder.add(
        RuleTag::ClauseRestrictionRun,
        N::RestrictionRun,
        [
            n(N::RestrictionMember),
            l(L::Conjunction),
            n(N::RestrictionMember),
        ],
    );
    // The comma-joined two-member base — the first interior pair of a
    // 3+-member Oxford run (`only X, only Y, and only Z`), which has no bare
    // `and` between its first two members.
    builder.add(
        RuleTag::ClauseRestrictionRun,
        N::RestrictionRun,
        [
            n(N::RestrictionMember),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::RestrictionMember),
        ],
    );
    builder.add(
        RuleTag::ClauseRestrictionRun,
        N::RestrictionRun,
        [
            n(N::RestrictionRun),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::RestrictionMember),
        ],
    );
    builder.add(
        RuleTag::ClauseRestrictionRun,
        N::RestrictionRun,
        [
            n(N::RestrictionRun),
            l(L::Punctuation(Punctuation::Comma)),
            l(L::Conjunction),
            n(N::RestrictionMember),
        ],
    );
    builder.add(
        RuleTag::ClauseRestrictionRun,
        N::Clause,
        [n(N::Clause), n(N::RestrictionRun)],
    );
}

pub(super) fn reduce_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseAuxiliaryProform
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhrasePreverbAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::VerbPhraseCausative
        | RuleTag::VerbPhraseCoordinatedAdjective
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo => reduce_predicate(tag, children),
        RuleTag::ManaAmountSymbol
        | RuleTag::ManaAmountSequence
        | RuleTag::ManaAmountListSingle
        | RuleTag::ManaAmountListComma => Some(Features::None),
        RuleTag::ManaAmountCoordination | RuleTag::ManaAmountCoordinationOxford => {
            let conjunction_index =
                if tag == RuleTag::ManaAmountCoordinationOxford { 2 } else { 1 };
            let Features::Conjunction(
                crate::syntax::PredicateConjunction::And | crate::syntax::PredicateConjunction::Or,
            ) = children.get(conjunction_index)?.features
            else {
                return None;
            };
            Some(Features::None)
        }
        RuleTag::GerundClauseBase => {
            let Features::VerbPhrase {
                form: PredicateForm::PresentParticiple,
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            Some(Features::GerundClause)
        }
        RuleTag::GerundClauseSubordinateAfter => {
            if !matches!(children.first()?.features, Features::GerundClause)
                || !matches!(children.get(2)?.features, Features::GerundClause)
            {
                return None;
            }
            Some(Features::GerundClause)
        }
        RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseExistential
        | RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderDistributiveEach
        | RuleTag::ClauseCopular
        | RuleTag::ClauseContractedCopular
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::CopularRemainderCoordinatedAdjective
        | RuleTag::RelativeContractedCopularCoordinatedAdjective => {
            reduce_simple_clause(tag, children)
        }
        RuleTag::ClauseVariableValueConstraint => reduce_variable_value_constraint(children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseAdverbBefore
        | RuleTag::ClauseSentenceAdverbialBefore
        | RuleTag::ClausePrepositionalBefore
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterComma
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::ExceptionRiderSingle
        | RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford
        | RuleTag::ClauseExcepted
        | RuleTag::ClauseRestrictionMember
        | RuleTag::ClauseRestrictionRun
        | RuleTag::Sentence => reduce_composed_clause(tag, children),
        _ => None,
    }
}

pub(super) fn accepts_predicate_prefix(
    tag: RuleTag,
    completed_children: usize,
    features: &Features,
) -> bool {
    if completed_children != 1 {
        return true;
    }
    let predicate_rule = matches!(
        tag,
        RuleTag::VerbPhraseDirectObject
            | RuleTag::VerbPhraseIndirectObject
            | RuleTag::VerbPhraseAdjective
            | RuleTag::VerbPhraseCoordinatedAdjective
            | RuleTag::VerbPhrasePrepositional
            | RuleTag::VerbPhraseInfinitive
            | RuleTag::VerbPhraseParticle
            | RuleTag::VerbPhraseAbility
            | RuleTag::VerbPhraseQuotedAbility
            | RuleTag::VerbPhraseQuotedAbilityCoordination
            | RuleTag::VerbPhraseAbilityQuotedCoordination
            | RuleTag::VerbPhraseOracleSymbol
            | RuleTag::VerbPhraseSymbolSequence
            | RuleTag::VerbPhraseManaAmountCoordination
            | RuleTag::VerbPhrasePowerToughness
            | RuleTag::VerbPhraseQuantity
            | RuleTag::VerbPhraseCausative
    );
    if !predicate_rule {
        return true;
    }
    let Features::VerbPhrase {
        object,
        indirect_object,
        phase,
        frame,
        passive,
        ..
    } = features
    else {
        return false;
    };
    match tag {
        // Only predict the bare-infinitive complement after a causative head
        // (`have`) that has already taken its direct-object causee. Without this
        // gate the `VerbPhrase = VerbPhrase VerbPhrase` production predicts a
        // second verb phrase after every verb phrase, perturbing the parse
        // forest of unrelated clauses.
        RuleTag::VerbPhraseCausative => {
            frame.causative_complement()
                && *object == PredicateObjectState::Direct
                && *phase == PredicateAttachmentPhase::Object
        }
        RuleTag::VerbPhraseDirectObject => {
            let nominal_adjunct = frame.licenses_bare_nominal_adjunct(BareNominalAdjunct::Temporal)
                || frame.licenses_bare_nominal_adjunct(BareNominalAdjunct::Manner);
            if *phase == PredicateAttachmentPhase::Object && *object == PredicateObjectState::None {
                frame.direct_object().accepts() || nominal_adjunct
            } else {
                nominal_adjunct
            }
        }
        RuleTag::VerbPhraseIndirectObject => {
            // An indirect object is a literal dependent NP; under a
            // recipient-passive frame the recipient is the promoted subject,
            // never a further explicit indirect object, so the passive never
            // predicts this rule regardless of frame.
            !*passive
                && *phase == PredicateAttachmentPhase::Object
                && *object == PredicateObjectState::None
                && !*indirect_object
                && frame.indirect_object().accepts()
        }
        RuleTag::VerbPhraseAdjective | RuleTag::VerbPhraseCoordinatedAdjective => {
            frame.licenses_complement(PredicateComplementKind::Adjective)
        }
        RuleTag::VerbPhraseInfinitive => {
            frame.licenses_complement(PredicateComplementKind::Infinitive)
        }
        RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination => {
            *object == PredicateObjectState::None
                && frame.licenses_complement(PredicateComplementKind::Ability)
        }
        RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination => {
            *object == PredicateObjectState::None
                && frame.licenses_complement(PredicateComplementKind::Scalar)
        }
        RuleTag::VerbPhrasePowerToughness => {
            *object == PredicateObjectState::None
                && frame.licenses_complement(PredicateComplementKind::Statistic)
        }
        RuleTag::VerbPhraseQuantity => {
            matches!(
                object,
                PredicateObjectState::None | PredicateObjectState::Ability
            ) && frame.licenses_complement(PredicateComplementKind::Scalar)
        }
        RuleTag::VerbPhraseParticle => !frame.particles.is_empty(),
        _ => true,
    }
}

pub(super) fn reduction_cost(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> ParseCost {
    let active_temporal_attachment = tag == RuleTag::VerbPhraseDirectObject
        && matches!(
            children.first().map(|child| child.features),
            Some(Features::VerbPhrase { passive: false, .. })
        )
        && matches!(
            children.get(1).map(|child| child.features),
            Some(Features::NounPhrase {
                adjunct: Some(BareNominalAdjunct::Temporal),
                ..
            })
        );
    let precedence =
        u32::from(active_temporal_attachment || tag == RuleTag::ClauseSubordinateAfter);
    // The finite-first shared-predicate reading (a modal/finite clause hosting
    // a subjectless standalone-imperative continuation, asyndetic or
    // `then`/`and`-joined) is a narrow additive allowance layered on top of the
    // existing coordination gate. Dispreference it so it never outranks an
    // existing winning parse of a currently-supported card and only wins when
    // no subject/`and`/`then`-independent reading exists for the same span.
    let finite_first_shared_predicate = matches!(
        tag,
        RuleTag::ClauseCoordination
            | RuleTag::ClauseCoordinationComma
            | RuleTag::ClauseCoordinationAsyndetic
    ) && matches!(
        children.first().map(|child| child.features),
        Some(Features::Clause { finite: true, .. })
    ) && matches!(
        children.last().map(|child| child.features),
        Some(Features::SimpleClause {
            agreement: None,
            has_subject: false,
            standalone: true,
            ..
        })
    );
    ParseCost {
        precedence,
        reading_dispreference: u32::from(finite_first_shared_predicate),
        ..ParseCost::default()
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "grammar reduction rule set is intentionally long"
)]
fn reduce_predicate(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::Verb => Some(propagate(children.first()?)),
        RuleTag::VerbPhraseBase => {
            let Features::Verb { slot, frame } = children.first()?.features else {
                return None;
            };
            let form = predicate_form(*slot);
            Some(Features::VerbPhrase {
                form,
                passive: false,
                object: PredicateObjectState::None,
                indirect_object: false,
                selected_preposition: false,
                phase: PredicateAttachmentPhase::Object,
                frame: *frame,
                bare: true,
                subjunctive: false,
            })
        }
        RuleTag::VerbPhraseAuxiliaryProform => {
            let Features::Auxiliary(auxiliary) = children.first()?.features else {
                return None;
            };
            let form = auxiliary_form(*auxiliary, PredicateForm::Infinitive)?;
            Some(Features::VerbPhrase {
                form,
                passive: false,
                object: PredicateObjectState::None,
                indirect_object: false,
                selected_preposition: false,
                phase: PredicateAttachmentPhase::Object,
                frame: crate::word::PROFORM_PREDICATE_FRAMES[0],
                bare: false,
                subjunctive: matches!(
                    auxiliary.inflection,
                    crate::word::AuxiliaryInflection::PastSubjunctive
                ),
            })
        }
        RuleTag::VerbPhraseAuxiliary => {
            let Features::Auxiliary(auxiliary) = children.first()?.features else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form,
                passive: child_passive,
                object,
                indirect_object,
                selected_preposition,
                phase,
                frame,
                subjunctive: child_subjunctive,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let form = auxiliary_form(*auxiliary, *child_form)?;
            let passive = fold_auxiliary_passive(
                *auxiliary,
                *child_form,
                *child_passive,
                *object,
                *indirect_object,
                *frame,
            )?;
            let subjunctive = *child_subjunctive
                || matches!(
                    auxiliary.inflection,
                    crate::word::AuxiliaryInflection::PastSubjunctive
                );
            Some(Features::VerbPhrase {
                form,
                passive,
                object: *object,
                indirect_object: *indirect_object,
                selected_preposition: *selected_preposition,
                phase: *phase,
                frame: *frame,
                bare: false,
                subjunctive,
            })
        }
        RuleTag::VerbPhraseDirectObject | RuleTag::VerbPhraseIndirectObject => {
            let Features::NounPhrase {
                pronoun_case,
                adjunct,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if *pronoun_case == Some(PronounCase::Subject) {
                return None;
            }
            let attachment = if tag == RuleTag::VerbPhraseIndirectObject {
                PredicateAttachment::IndirectObject
            } else {
                let Features::VerbPhrase {
                    form,
                    passive,
                    object,
                    phase,
                    frame,
                    ..
                } = children.first()?.features
                else {
                    return None;
                };
                match adjunct.filter(|adjunct| {
                    frame.licenses_bare_nominal_adjunct(*adjunct)
                        && (*form == PredicateForm::PastParticiple
                            || *passive
                            || *phase != PredicateAttachmentPhase::Object
                            || object.has_direct_object()
                            || !frame.direct_object().accepts())
                }) {
                    Some(adjunct) => PredicateAttachment::NominalAdjunct(adjunct),
                    None => PredicateAttachment::DirectObject,
                }
            };
            extend_predicate(children.first()?, attachment)
        }
        RuleTag::VerbPhraseManaAmountCoordination => {
            extend_predicate(children.first()?, PredicateAttachment::ScalarComplement)
        }
        RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination => {
            let Features::Conjunction(
                crate::syntax::PredicateConjunction::And | crate::syntax::PredicateConjunction::Or,
            ) = children.get(2)?.features
            else {
                return None;
            };
            extend_predicate(children.first()?, PredicateAttachment::QuotedObject)
        }
        RuleTag::VerbPhraseCoordinatedAdjective => {
            // Only a coordinated run whose every conjunct is an adjective
            // predicates as an adjective complement. A bare coordinated *noun*
            // pair after a verb (`Enchant creature or Vehicle`) keeps its
            // ordinary coordinated-noun-object parse rather than reducing here
            // and then failing to lower.
            let Features::CoordinatedModifier {
                all_adjectives: true,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            extend_predicate(children.first()?, PredicateAttachment::AdjectiveComplement)
        }
        RuleTag::VerbPhrasePreverbAdverb => {
            // Not an extend_predicate/Adjunct site (§2.4): the preverb
            // modifier attaches before the verb phrase (child 1, not child 0)
            // and changes no predicate-phase or object-state licensing, so the
            // composed features are exactly the inner VerbPhrase's, unchanged.
            Some(children.get(1)?.features.clone())
        }
        RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity => {
            let attachment = match tag {
                RuleTag::VerbPhraseAdjective => PredicateAttachment::AdjectiveComplement,
                RuleTag::VerbPhrasePrepositional => {
                    let Features::PrepositionalPhrase { preposition, .. } =
                        children.get(1)?.features
                    else {
                        return None;
                    };
                    PredicateAttachment::Prepositional(*preposition)
                }
                RuleTag::VerbPhraseInfinitive => PredicateAttachment::InfinitiveComplement,
                RuleTag::VerbPhraseAdverb | RuleTag::VerbPhraseFrequency => {
                    PredicateAttachment::Adjunct
                }
                RuleTag::VerbPhraseParticle => {
                    let Features::VerbParticle(particle) = children.get(1)?.features else {
                        return None;
                    };
                    PredicateAttachment::Particle(*particle)
                }
                RuleTag::VerbPhraseAbility => PredicateAttachment::AbilityComplement,
                RuleTag::VerbPhraseQuotedAbility => PredicateAttachment::QuotedObject,
                RuleTag::VerbPhraseOracleSymbol | RuleTag::VerbPhraseSymbolSequence => {
                    PredicateAttachment::ScalarComplement
                }
                RuleTag::VerbPhrasePowerToughness => PredicateAttachment::StatisticComplement,
                RuleTag::VerbPhraseQuantity => PredicateAttachment::ScalarOrAbilityArgument,
                _ => return None,
            };
            extend_predicate(children.first()?, attachment)
        }
        RuleTag::InfinitiveTo | RuleTag::InfinitiveNotTo => {
            let predicate_index = if tag == RuleTag::InfinitiveNotTo { 2 } else { 1 };
            let Features::VerbPhrase {
                form: PredicateForm::Infinitive,
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                ..
            } = children.get(predicate_index)?.features
            else {
                return None;
            };
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            Some(Features::InfinitiveClause)
        }
        RuleTag::VerbPhraseCausative => {
            let Features::VerbPhrase {
                form: head_form,
                passive: false,
                object: PredicateObjectState::Direct,
                indirect_object,
                selected_preposition,
                phase: PredicateAttachmentPhase::Object,
                frame,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if !frame.causative_complement() || *head_form != PredicateForm::Infinitive {
                return None;
            }
            if !predicate_arguments_complete(
                *frame,
                false,
                PredicateObjectState::Direct,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            // The causative complement is a complete bare-infinitive verb phrase.
            let Features::VerbPhrase {
                form: PredicateForm::Infinitive,
                passive: complement_passive,
                object: complement_object,
                indirect_object: complement_indirect_object,
                selected_preposition: complement_selected_preposition,
                frame: complement_frame,
                bare: complement_bare,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if *complement_bare && complement_frame.is_proform() {
                return None;
            }
            if !predicate_arguments_complete(
                *complement_frame,
                *complement_passive,
                *complement_object,
                *complement_indirect_object,
                *complement_selected_preposition,
            ) {
                return None;
            }
            Some(Features::VerbPhrase {
                form: *head_form,
                passive: false,
                object: PredicateObjectState::Direct,
                indirect_object: *indirect_object,
                selected_preposition: *selected_preposition,
                phase: PredicateAttachmentPhase::Tail,
                frame: *frame,
                bare: false,
                subjunctive: false,
            })
        }
        _ => None,
    }
}

fn extend_predicate(
    predicate: &Child<'_, EnglishGrammar<'_, '_>>,
    attachment: PredicateAttachment,
) -> Option<Reduced> {
    let Features::VerbPhrase {
        form,
        passive,
        object,
        indirect_object,
        selected_preposition,
        phase,
        frame,
        subjunctive,
        ..
    } = predicate.features
    else {
        return None;
    };
    let prepositional_role = match attachment {
        PredicateAttachment::Prepositional(preposition) => {
            Some(frame.prepositional_role(preposition)?)
        }
        _ => None,
    };
    let licensed = match attachment {
        PredicateAttachment::Adjunct | PredicateAttachment::Prepositional(_) => true,
        PredicateAttachment::DirectObject => frame.direct_object().accepts(),
        PredicateAttachment::IndirectObject => frame.indirect_object().accepts(),
        PredicateAttachment::NominalAdjunct(adjunct) => {
            frame.licenses_bare_nominal_adjunct(adjunct)
        }
        PredicateAttachment::AdjectiveComplement => {
            frame.licenses_complement(PredicateComplementKind::Adjective)
        }
        PredicateAttachment::InfinitiveComplement => {
            frame.licenses_complement(PredicateComplementKind::Infinitive)
        }
        PredicateAttachment::Particle(particle) => frame.licenses_particle(particle),
        PredicateAttachment::AbilityComplement | PredicateAttachment::QuotedObject => {
            frame.licenses_complement(PredicateComplementKind::Ability)
        }
        PredicateAttachment::ScalarComplement | PredicateAttachment::ScalarOrAbilityArgument => {
            frame.licenses_complement(PredicateComplementKind::Scalar)
        }
        PredicateAttachment::StatisticComplement => {
            frame.licenses_complement(PredicateComplementKind::Statistic)
        }
    };
    if !licensed {
        return None;
    }
    // A recipient-passive frame retains its theme post-verbally, so a direct
    // object may attach under it in the passive; every other frame keeps the
    // blanket ban (an ordinary passive's promoted subject IS the theme, so a
    // further direct object is never a coherent reading).
    let direct_object_attaches_under_passive =
        matches!(attachment, PredicateAttachment::DirectObject) && frame.is_recipient_passive();
    if *passive
        && !direct_object_attaches_under_passive
        && !matches!(
            attachment,
            PredicateAttachment::Adjunct
                | PredicateAttachment::NominalAdjunct(_)
                | PredicateAttachment::AdjectiveComplement
                | PredicateAttachment::Prepositional(_)
                | PredicateAttachment::InfinitiveComplement
                | PredicateAttachment::Particle(_)
        )
    {
        return None;
    }
    let next_phase = match attachment {
        PredicateAttachment::DirectObject
        | PredicateAttachment::IndirectObject
        | PredicateAttachment::AbilityComplement
        | PredicateAttachment::QuotedObject
        | PredicateAttachment::ScalarComplement
        | PredicateAttachment::StatisticComplement
        | PredicateAttachment::ScalarOrAbilityArgument => {
            if *phase != PredicateAttachmentPhase::Object {
                return None;
            }
            PredicateAttachmentPhase::Object
        }
        PredicateAttachment::Prepositional(_) => PredicateAttachmentPhase::PrepositionalTail,
        PredicateAttachment::Adjunct => {
            if *object == PredicateObjectState::None {
                PredicateAttachmentPhase::Object
            } else {
                PredicateAttachmentPhase::Tail
            }
        }
        PredicateAttachment::NominalAdjunct(_)
        | PredicateAttachment::AdjectiveComplement
        | PredicateAttachment::InfinitiveComplement
        | PredicateAttachment::Particle(_) => PredicateAttachmentPhase::Tail,
    };
    let object = match (attachment, *object) {
        (
            PredicateAttachment::Adjunct
            | PredicateAttachment::IndirectObject
            | PredicateAttachment::NominalAdjunct(_)
            | PredicateAttachment::AdjectiveComplement
            | PredicateAttachment::Prepositional(_)
            | PredicateAttachment::InfinitiveComplement
            | PredicateAttachment::Particle(_),
            object,
        ) => object,
        (
            PredicateAttachment::DirectObject
            | PredicateAttachment::QuotedObject
            | PredicateAttachment::ScalarComplement
            | PredicateAttachment::StatisticComplement
            | PredicateAttachment::ScalarOrAbilityArgument,
            PredicateObjectState::None,
        ) => PredicateObjectState::Direct,
        (PredicateAttachment::AbilityComplement, PredicateObjectState::None) => {
            PredicateObjectState::Ability
        }
        (PredicateAttachment::ScalarOrAbilityArgument, PredicateObjectState::Ability) => {
            PredicateObjectState::AbilityWithArgument
        }
        _ => return None,
    };
    let indirect_object = match attachment {
        PredicateAttachment::IndirectObject if !*indirect_object && !object.has_direct_object() => {
            true
        }
        PredicateAttachment::IndirectObject => return None,
        _ => *indirect_object,
    };
    let selected_preposition = match prepositional_role {
        Some(PrepositionalRole::SelectedComplement) if *selected_preposition => return None,
        Some(PrepositionalRole::SelectedComplement) => true,
        Some(PrepositionalRole::Adjunct) | None => *selected_preposition,
    };
    Some(Features::VerbPhrase {
        form: *form,
        passive: *passive,
        object,
        indirect_object,
        selected_preposition,
        phase: next_phase,
        frame: *frame,
        bare: false,
        subjunctive: *subjunctive,
    })
}

#[derive(Debug, Clone, Copy)]
enum PredicateAttachment {
    Adjunct,
    DirectObject,
    IndirectObject,
    NominalAdjunct(BareNominalAdjunct),
    AdjectiveComplement,
    Prepositional(Preposition),
    InfinitiveComplement,
    Particle(VerbParticle),
    AbilityComplement,
    /// A quoted (or coordinated quoted) ability object. Licensed by the same
    /// frames that admit a keyword-ability complement (the grant verbs), but it
    /// is a complete direct object — unlike a bare keyword ability it never
    /// takes a following scalar argument — so it settles the object slot to
    /// `Direct`.
    QuotedObject,
    ScalarComplement,
    StatisticComplement,
    ScalarOrAbilityArgument,
}

/// Folds an auxiliary attaching from outside a verb phrase into that phrase's
/// passive determination, and applies the retained-object rule. Shared by
/// [`RuleTag::VerbPhraseAuxiliary`] (the auxiliary sits inside the phrase) and
/// [`RuleTag::SimpleClauseContractedSubject`] (the auxiliary is contracted onto
/// the subject and reaches the phrase as a sibling), so the two paths cannot
/// drift: before this was shared, the contracted path never credited
/// passivization and no recipient passive could reduce under it.
///
/// Returns `None` when the combination is ill-formed — a passive may keep a
/// direct object only as a recipient passive's retained theme, and never keeps
/// an explicit indirect object.
fn fold_auxiliary_passive(
    auxiliary: AuxiliaryInstance,
    child_form: PredicateForm,
    child_passive: bool,
    object: PredicateObjectState,
    indirect_object: bool,
    frame: PredicateFrame,
) -> Option<bool> {
    let passive = child_passive
        || (auxiliary.auxiliary == Auxiliary::Be && child_form == PredicateForm::PastParticiple);
    // A direct object survives passivization only under a recipient-passive
    // frame (the retained theme); every other frame keeps the blanket ban. An
    // indirect-object DEPENDENT (a literal NP, as opposed to the frame-level
    // promotion) is never valid under any passive: the recipient-passive's
    // indirect-object requirement is satisfied by the promotion itself, never
    // by a further explicit indirect object.
    if passive && (indirect_object || (object.has_direct_object() && !frame.is_recipient_passive()))
    {
        return None;
    }
    Some(passive)
}

fn predicate_arguments_complete(
    frame: PredicateFrame,
    passive: bool,
    object: PredicateObjectState,
    indirect_object: bool,
    selected_preposition: bool,
) -> bool {
    // A recipient-passive frame exists only to license the passive; it must
    // never be selectable in the active, or `deals 2 damage to X` would gain a
    // spurious double-object reading.
    if frame.is_recipient_passive() && !passive {
        return false;
    }
    let recipient_passive = passive && frame.is_recipient_passive();
    frame
        .direct_object()
        // Ordinary passivization promotes the direct object; recipient
        // passivization does not — the theme must still be present, retained.
        .is_satisfied_by((passive && !recipient_passive) || object.has_direct_object())
        && frame
            .indirect_object()
            // The recipient is the promoted subject, so the indirect-object
            // requirement is satisfied by the promotion itself.
            .is_satisfied_by(indirect_object || recipient_passive)
        && frame
            .selected_preposition()
            .is_satisfied_by(selected_preposition)
}

fn predicate_object_gap_complete(
    frame: PredicateFrame,
    indirect_object: bool,
    selected_preposition: bool,
) -> bool {
    if frame.is_recipient_passive() {
        return false;
    }
    frame.direct_object().accepts()
        && frame.indirect_object().is_satisfied_by(indirect_object)
        && frame
            .selected_preposition()
            .is_satisfied_by(selected_preposition)
}

#[allow(
    clippy::too_many_lines,
    reason = "simple-clause reduction logic is intentionally long"
)]
fn reduce_simple_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::SimpleClauseSubject => {
            let Features::NounPhrase {
                agreement: Some(subject_agreement),
                pronoun_case,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if *pronoun_case == Some(PronounCase::Object) {
                return None;
            }
            let Features::VerbPhrase {
                form: PredicateForm::Finite(predicate_agreement),
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                subjunctive,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if predicate_agreement.is_some_and(|agreement| agreement != *subject_agreement) {
                return None;
            }
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            let host_addressee_subject = *pronoun_case == Some(PronounCase::Subject)
                && subject_agreement.person == Person::Second;
            let host_modal = predicate_agreement.is_none();
            Some(simple_clause_reduction(
                Some(*subject_agreement),
                true,
                true,
                object.has_direct_object(),
                host_addressee_subject,
                host_modal,
                *subjunctive,
            ))
        }
        RuleTag::SimpleClauseContractedSubject => {
            let Features::SubjectAuxiliary {
                agreement: subject_agreement,
                auxiliary,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form,
                passive: child_passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                subjunctive,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let PredicateForm::Finite(Some(predicate_agreement)) =
                auxiliary_form(*auxiliary, *child_form)?
            else {
                return None;
            };
            if predicate_agreement != *subject_agreement {
                return None;
            }
            let passive = fold_auxiliary_passive(
                *auxiliary,
                *child_form,
                *child_passive,
                *object,
                *indirect_object,
                *frame,
            )?;
            if !predicate_arguments_complete(
                *frame,
                passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            let host_addressee_subject = subject_agreement.person == Person::Second;
            // `auxiliary_form` only yields `Finite(Some(_))` here (the
            // `Finite(None)` base-modal path never satisfies this pattern),
            // so a contracted-subject host never carries `host_modal`.
            let host_modal = false;
            Some(simple_clause_reduction(
                Some(*subject_agreement),
                true,
                true,
                object.has_direct_object(),
                host_addressee_subject,
                host_modal,
                *subjunctive,
            ))
        }
        RuleTag::SimpleClauseSubjectless => {
            let Features::VerbPhrase {
                form,
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                subjunctive,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            match form {
                PredicateForm::Imperative => Some(simple_clause_reduction(
                    None,
                    false,
                    true,
                    object.has_direct_object(),
                    false,
                    false,
                    *subjunctive,
                )),
                PredicateForm::Finite(agreement) => Some(simple_clause_reduction(
                    *agreement,
                    false,
                    false,
                    object.has_direct_object(),
                    false,
                    agreement.is_none(),
                    *subjunctive,
                )),
                PredicateForm::Infinitive => Some(simple_clause_reduction(
                    None,
                    false,
                    false,
                    object.has_direct_object(),
                    false,
                    false,
                    *subjunctive,
                )),
                PredicateForm::PresentParticiple | PredicateForm::PastParticiple => None,
            }
        }
        RuleTag::ClauseSimple => {
            let Features::SimpleClause {
                agreement,
                has_subject,
                standalone,
                host_addressee_subject,
                host_modal,
                subjunctive,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Clause {
                agreement: *agreement,
                standalone: *standalone,
                finite: *has_subject,
                host_addressee_subject: *host_addressee_subject,
                host_modal: *host_modal,
                subjunctive: *subjunctive,
            })
        }
        RuleTag::ClauseElliptical => Some(Features::Clause {
            agreement: None,
            standalone: false,
            finite: false,
            host_addressee_subject: false,
            host_modal: false,
            subjunctive: false,
        }),
        RuleTag::ClauseExistential => {
            let Features::Existential {
                number: expected_number,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::NounPhrase {
                agreement: Some(agreement),
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if agreement.number != *expected_number {
                return None;
            }
            Some(Features::Clause {
                agreement: None,
                standalone: true,
                finite: true,
                host_addressee_subject: false,
                host_modal: false,
                subjunctive: false,
            })
        }
        RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderDistributiveEach => Some(Features::None),
        RuleTag::CopularRemainderCoordinatedAdjective => {
            // Only an all-adjective coordinated run predicates as a copular
            // adjective complement; a coordinated run holding a noun reading
            // (supertypes such as `snow` scan as both) is rejected here so the
            // adjective-reading conjuncts are the ones lowering converts.
            let Features::CoordinatedModifier {
                all_adjectives: true,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::None)
        }
        tag @ (RuleTag::ClauseCopular | RuleTag::ClauseContractedCopular) => {
            reduce_copular_clause(tag, children)
        }
        RuleTag::RelativeObject => {
            let Features::NounPhrase {
                agreement: Some(subject_agreement),
                pronoun_case,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if *pronoun_case == Some(PronounCase::Object) {
                return None;
            }
            let Features::VerbPhrase {
                form: PredicateForm::Finite(predicate_agreement),
                object: PredicateObjectState::None,
                indirect_object,
                selected_preposition,
                frame,
                bare,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if (*bare && frame.is_proform())
                || !predicate_object_gap_complete(*frame, *indirect_object, *selected_preposition)
                || predicate_agreement.is_some_and(|agreement| agreement != *subject_agreement)
            {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Object,
                antecedent_agreement: None,
            })
        }
        RuleTag::RelativeObjectContractedSubject => {
            let Features::SubjectAuxiliary {
                agreement: subject_agreement,
                auxiliary,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form,
                object: PredicateObjectState::None,
                indirect_object,
                selected_preposition,
                frame,
                bare,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let PredicateForm::Finite(Some(predicate_agreement)) =
                auxiliary_form(*auxiliary, *child_form)?
            else {
                return None;
            };
            if (*bare && frame.is_proform())
                || !predicate_object_gap_complete(*frame, *indirect_object, *selected_preposition)
                || predicate_agreement != *subject_agreement
            {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Object,
                antecedent_agreement: None,
            })
        }
        RuleTag::RelativeSubjectContractedAuxiliary => {
            let Features::SubjectAuxiliary {
                subject: ContractedSubjectKey::Demonstrative(Demonstrative::That),
                agreement,
                auxiliary,
            } = children.first()?.features
            else {
                return None;
            };
            let Features::VerbPhrase {
                form: child_form, ..
            } = children.get(1)?.features
            else {
                return None;
            };
            let PredicateForm::Finite(Some(predicate_agreement)) =
                auxiliary_form(*auxiliary, *child_form)?
            else {
                return None;
            };
            if predicate_agreement != *agreement {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Subject,
                antecedent_agreement: Some(predicate_agreement),
            })
        }
        RuleTag::RelativeSubject => {
            let Features::VerbPhrase {
                form: PredicateForm::Finite(antecedent_agreement),
                passive,
                object,
                indirect_object,
                selected_preposition,
                frame,
                ..
            } = children.get(1)?.features
            else {
                return None;
            };
            if !predicate_arguments_complete(
                *frame,
                *passive,
                *object,
                *indirect_object,
                *selected_preposition,
            ) {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Subject,
                antecedent_agreement: *antecedent_agreement,
            })
        }
        RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::RelativeContractedCopularCoordinatedAdjective => {
            let Features::SubjectAuxiliary {
                subject: ContractedSubjectKey::Demonstrative(Demonstrative::That),
                agreement,
                auxiliary,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            if auxiliary.auxiliary != Auxiliary::Be {
                return None;
            }
            // The coordinated variant only predicates when every conjunct is an
            // adjective; a noun-reading coordinated run is rejected so the
            // adjective-reading conjuncts are the ones lowering keeps.
            if tag == RuleTag::RelativeContractedCopularCoordinatedAdjective
                && !matches!(
                    children.get(1)?.features,
                    Features::CoordinatedModifier {
                        all_adjectives: true,
                        ..
                    }
                )
            {
                return None;
            }
            Some(Features::RelativeClause {
                gap: RelativeGap::Subject,
                antecedent_agreement: Some(*agreement),
            })
        }
        _ => None,
    }
}

fn reduce_copular_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let contracted = tag == RuleTag::ClauseContractedCopular;
    let (agreement, subjunctive) = if contracted {
        let Features::SubjectAuxiliary {
            agreement,
            auxiliary,
            ..
        } = children.first()?.features
        else {
            return None;
        };
        if auxiliary.auxiliary != Auxiliary::Be {
            return None;
        }
        (*agreement, false)
    } else {
        let Features::NounPhrase {
            agreement: Some(subject_agreement),
            pronoun_case,
            ..
        } = children.first()?.features
        else {
            return None;
        };
        if *pronoun_case == Some(PronounCase::Object) {
            return None;
        }
        let Features::Copula(copula) = children.get(1)?.features else {
            return None;
        };
        match copula {
            CopulaAgreement::Indicative(copula_agreement) => {
                if *subject_agreement != *copula_agreement {
                    return None;
                }
                (*subject_agreement, false)
            }
            // Recognition-level licensing: no agreement constraint, but the
            // clause is marked subjunctive, so every consumer other than the
            // `as though` gate (clause.rs:2056) rejects it.
            CopulaAgreement::PastSubjunctive => (*subject_agreement, true),
        }
    };
    Some(Features::Clause {
        agreement: Some(agreement),
        standalone: true,
        finite: true,
        host_addressee_subject: false,
        host_modal: false,
        subjunctive,
    })
}

fn reduce_variable_value_constraint(
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    let Features::Quantity(_) = children.first()?.features else {
        return None;
    };
    let Features::Auxiliary(modal) = children.get(1)?.features else {
        return None;
    };
    if !is_modal(modal.auxiliary) {
        return None;
    }
    let Features::Auxiliary(copula) = children.get(2)?.features else {
        return None;
    };
    if copula.auxiliary != Auxiliary::Be || copula.inflection != AuxiliaryInflection::Base {
        return None;
    }
    let Features::Number { .. } = children.get(3)?.features else {
        return None;
    };
    Some(Features::Clause {
        agreement: None,
        standalone: true,
        finite: true,
        host_addressee_subject: false,
        host_modal: false,
        subjunctive: false,
    })
}

#[allow(
    clippy::fn_params_excessive_bools,
    reason = "each bool is an independently-computed SimpleClause feature bit; \
              grouping into enums would obscure the 1:1 field mapping"
)]
fn simple_clause_reduction(
    agreement: Option<Agreement>,
    has_subject: bool,
    standalone: bool,
    has_direct_object: bool,
    host_addressee_subject: bool,
    host_modal: bool,
    subjunctive: bool,
) -> Reduced {
    Features::SimpleClause {
        agreement,
        has_subject,
        standalone,
        has_direct_object,
        host_addressee_subject,
        host_modal,
        subjunctive,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "composed-clause reduction logic is intentionally long"
)]
fn reduce_composed_clause(
    tag: RuleTag,
    children: &[Child<'_, EnglishGrammar<'_, '_>>],
) -> Option<Reduced> {
    match tag {
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic => {
            let Features::Clause {
                agreement: first_agreement,
                standalone: true,
                finite,
                host_addressee_subject: first_host_addressee_subject,
                host_modal: first_host_modal,
                subjunctive: first_subjunctive,
            } = children.first()?.features
            else {
                return None;
            };
            let last = children.last()?;
            let Features::SimpleClause {
                agreement: next_agreement,
                has_subject,
                standalone,
                subjunctive: next_subjunctive,
                ..
            } = last.features
            else {
                return None;
            };
            // A subjunctive-flagged clause never surfaces as a coordinated
            // member (only `ClauseSubordinateAfter`/`…Comma` under
            // `as though` may consume one).
            if *first_subjunctive || *next_subjunctive {
                return None;
            }
            let host_adopts_imperative = *first_host_addressee_subject || *first_host_modal;
            // A finite, subject-bearing first clause may still host a bare
            // (subjectless, standalone) imperative continuation asyndetically —
            // "you may search ..., reveal it" — even though the first-clause
            // conditions below would otherwise disqualify it. The continuation
            // side (`next_agreement`/`standalone`) still must hold, and the
            // host must adopt the imperative: a genuine addressee-`you`
            // subject or a base-inflection modal (`may`/`can`/...) shares the
            // continuation's implicit "you", so a third-person, modal-less
            // host (in practice an opaque-noun subject like "When ..."
            // misparsed as a nominal, or its nearest-conjunct-donated
            // agreement) cannot adopt it and stays honestly unparsed.
            let continuation_is_bare_imperative =
                next_agreement.is_none() && !*has_subject && *standalone;
            if tag == RuleTag::ClauseCoordinationAsyndetic
                && !(continuation_is_bare_imperative && host_adopts_imperative)
                && (first_agreement.is_some()
                    || *finite
                    || *has_subject
                    || next_agreement.is_some()
                    || !*standalone)
            {
                return None;
            }
            if !coordination_agrees(
                *first_agreement,
                *next_agreement,
                *has_subject,
                *standalone,
                host_adopts_imperative,
            ) {
                return None;
            }
            Some(Features::Clause {
                agreement: *first_agreement,
                standalone: true,
                finite: *finite,
                host_addressee_subject: *first_host_addressee_subject,
                host_modal: *first_host_modal,
                subjunctive: false,
            })
        }
        RuleTag::ClauseSubordinateBefore => {
            let Features::Subordinator(subordinator) = children.first()?.features else {
                return None;
            };
            conditional_reduction(*subordinator, children.get(1)?, children.get(3)?)
        }
        RuleTag::ClauseAdverbBefore => fronted_attachment_reduction(children.get(1)?),
        RuleTag::ClauseSentenceAdverbialBefore => fronted_attachment_reduction(children.get(2)?),
        RuleTag::ClausePrepositionalBefore => {
            let Features::PrepositionalPhrase { .. } = children.first()?.features else {
                return None;
            };
            fronted_attachment_reduction(children.get(2)?)
        }
        RuleTag::ClauseSubordinateAfterElliptical => {
            let consequence = children.first()?;
            let Features::Clause {
                agreement,
                standalone: true,
                finite,
                host_addressee_subject,
                host_modal,
                subjunctive,
            } = consequence.features
            else {
                return None;
            };
            if *subjunctive {
                return None;
            }
            Some(Features::Clause {
                agreement: *agreement,
                standalone: true,
                finite: *finite,
                host_addressee_subject: *host_addressee_subject,
                host_modal: *host_modal,
                subjunctive: false,
            })
        }
        RuleTag::ClauseSubordinateAfter => {
            let Features::Subordinator(subordinator) = children.get(1)?.features else {
                return None;
            };
            conditional_reduction(*subordinator, children.get(2)?, children.first()?)
        }
        RuleTag::ClauseSubordinateAfterComma => {
            let Features::Subordinator(subordinator) = children.get(2)?.features else {
                return None;
            };
            conditional_reduction(*subordinator, children.get(3)?, children.first()?)
        }
        RuleTag::ClauseSubordinateAfterInfinitive => {
            let Features::Clause {
                agreement,
                standalone: true,
                finite,
                host_addressee_subject,
                host_modal,
                subjunctive,
            } = children.first()?.features
            else {
                return None;
            };
            if *subjunctive {
                return None;
            }
            let Features::VerbPhrase {
                form: PredicateForm::Infinitive,
                ..
            } = children.get(2)?.features
            else {
                return None;
            };
            Some(Features::Clause {
                agreement: *agreement,
                standalone: true,
                finite: *finite,
                host_addressee_subject: *host_addressee_subject,
                host_modal: *host_modal,
                subjunctive: false,
            })
        }
        RuleTag::ExceptionRiderSingle => {
            let Features::Clause {
                standalone: true, ..
            } = children.get(1)?.features
            else {
                return None;
            };
            Some(Features::ExceptionRider)
        }
        RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford => {
            if !matches!(children.first()?.features, Features::ExceptionRider) {
                return None;
            }
            let Features::Clause {
                standalone: true, ..
            } = children.last()?.features
            else {
                return None;
            };
            Some(Features::ExceptionRider)
        }
        RuleTag::ClauseExcepted => {
            let Features::Clause {
                agreement,
                standalone: true,
                finite,
                host_addressee_subject,
                host_modal,
                subjunctive,
            } = children.first()?.features
            else {
                return None;
            };
            if *subjunctive {
                return None;
            }
            if !matches!(children.get(2)?.features, Features::ExceptionRider) {
                return None;
            }
            Some(Features::Clause {
                agreement: *agreement,
                standalone: true,
                finite: *finite,
                host_addressee_subject: *host_addressee_subject,
                host_modal: *host_modal,
                subjunctive: false,
            })
        }
        RuleTag::ClauseRestrictionMember => {
            if children.len() == 3 {
                // `[only, Subordinator, Clause]` — the `if`-clause member.
                if let Features::Clause {
                    standalone: true, ..
                } = children.get(2)?.features
                {
                    return Some(Features::RestrictionMember);
                }
            }
            // `[only, Prepositional]`, `[only, Adverb]`, `[only, Adverb,
            // NounPhrase]` — no cross-child feature agreement needed; the
            // grammatical shape alone licenses these.
            Some(Features::RestrictionMember)
        }
        RuleTag::ClauseRestrictionRun => {
            match children.len() {
                2 => {
                    // The `[Clause, RestrictionRun]` attachment.
                    let Features::Clause {
                        agreement,
                        standalone: true,
                        finite,
                        host_addressee_subject,
                        host_modal,
                        subjunctive,
                    } = children.first()?.features
                    else {
                        return None;
                    };
                    if *subjunctive {
                        return None;
                    }
                    if !matches!(children.get(1)?.features, Features::RestrictionRun) {
                        return None;
                    }
                    Some(Features::Clause {
                        agreement: *agreement,
                        standalone: true,
                        finite: *finite,
                        host_addressee_subject: *host_addressee_subject,
                        host_modal: *host_modal,
                        subjunctive: false,
                    })
                }
                3 => {
                    // `[Member, Conjunction, Member]` (base pair) or
                    // `[Run, Comma, Member]` (asyndetic growth).
                    let first_ok = matches!(
                        children.first()?.features,
                        Features::RestrictionMember | Features::RestrictionRun
                    );
                    let last_ok = matches!(children.last()?.features, Features::RestrictionMember);
                    if first_ok && last_ok { Some(Features::RestrictionRun) } else { None }
                }
                4 => {
                    // `[Run, Comma, Conjunction, Member]` (Oxford growth).
                    if !matches!(children.first()?.features, Features::RestrictionRun) {
                        return None;
                    }
                    if !matches!(children.last()?.features, Features::RestrictionMember) {
                        return None;
                    }
                    Some(Features::RestrictionRun)
                }
                _ => None,
            }
        }
        RuleTag::Sentence => {
            let Features::Clause {
                standalone: true,
                subjunctive: false,
                ..
            } = children.first()?.features
            else {
                return None;
            };
            Some(Features::Sentence)
        }
        _ => None,
    }
}

fn fronted_attachment_reduction(matrix: &Child<'_, EnglishGrammar<'_, '_>>) -> Option<Reduced> {
    let Features::Clause {
        agreement,
        standalone: true,
        finite,
        host_addressee_subject,
        host_modal,
        subjunctive,
    } = matrix.features
    else {
        return None;
    };
    if *subjunctive {
        return None;
    }
    Some(Features::Clause {
        agreement: *agreement,
        standalone: true,
        finite: *finite,
        host_addressee_subject: *host_addressee_subject,
        host_modal: *host_modal,
        subjunctive: false,
    })
}

fn conditional_reduction(
    subordinator: crate::syntax::Subordinator,
    condition: &Child<'_, EnglishGrammar<'_, '_>>,
    consequence: &Child<'_, EnglishGrammar<'_, '_>>,
) -> Option<Reduced> {
    let Features::Clause {
        standalone: true,
        finite: true,
        subjunctive: condition_subjunctive,
        ..
    } = condition.features
    else {
        return None;
    };
    // Licensing gate: a past-subjunctive condition clause (`it were ...`) is
    // only ever well-formed under `as though` — every other subordinator
    // (`until`, `as long as`, `where`, ...) must reject it outright. See
    // `Features::Subordinator`/`AuxiliaryInflection::PastSubjunctive`.
    if *condition_subjunctive && !matches!(subordinator, crate::syntax::Subordinator::AsThough) {
        return None;
    }
    let Features::Clause {
        agreement,
        standalone: true,
        finite,
        host_addressee_subject,
        host_modal,
        subjunctive: consequence_subjunctive,
    } = consequence.features
    else {
        return None;
    };
    // The matrix clause itself is never subjunctive in this construction.
    if *consequence_subjunctive {
        return None;
    }
    Some(Features::Clause {
        agreement: *agreement,
        standalone: true,
        finite: *finite,
        host_addressee_subject: *host_addressee_subject,
        host_modal: *host_modal,
        // The composed clause is not itself subjunctive: the flag is
        // consumed by this gate, never propagated further.
        subjunctive: false,
    })
}

fn coordination_agrees(
    first: Option<Agreement>,
    next: Option<Agreement>,
    next_has_subject: bool,
    next_standalone: bool,
    first_adopts_imperative: bool,
) -> bool {
    next_has_subject
        || matches!((first, next, next_standalone),
            (Some(left), Some(right), false) if left == right
        )
        || matches!((first, next, next_standalone), (None, None, true))
        || matches!((first, next, next_standalone), (Some(_), None, false))
        // A first clause whose host adopts the imperative (a genuine
        // addressee-`you` subject or a base-inflection modal, often "you may
        // search ..." or "Its controller may search ...") followed by a
        // subjectless standalone imperative continuation ("..., then
        // shuffle") — the `then`-tail counterpart to the asyndetic allowance
        // in `reduce_composed_clause`.
        || (first_adopts_imperative && next.is_none() && next_standalone)
}

const fn predicate_form(slot: VerbSlot) -> PredicateForm {
    match slot {
        VerbSlot::Imperative => PredicateForm::Imperative,
        VerbSlot::Infinitive => PredicateForm::Infinitive,
        VerbSlot::Present { person, number } | VerbSlot::Past { person, number } => {
            PredicateForm::Finite(Some(Agreement { person, number }))
        }
        VerbSlot::PresentParticiple => PredicateForm::PresentParticiple,
        VerbSlot::PastParticiple => PredicateForm::PastParticiple,
    }
}

fn auxiliary_form(auxiliary: AuxiliaryInstance, child: PredicateForm) -> Option<PredicateForm> {
    let accepts_child = match auxiliary.auxiliary {
        Auxiliary::Be => matches!(
            child,
            PredicateForm::PresentParticiple | PredicateForm::PastParticiple
        ),
        Auxiliary::Have => child == PredicateForm::PastParticiple,
        Auxiliary::Do
        | Auxiliary::Can
        | Auxiliary::Could
        | Auxiliary::May
        | Auxiliary::Might
        | Auxiliary::Must
        | Auxiliary::Shall
        | Auxiliary::Should
        | Auxiliary::Will
        | Auxiliary::Would => child == PredicateForm::Infinitive,
    };
    if !accepts_child {
        return None;
    }

    match auxiliary.inflection {
        AuxiliaryInflection::Base => match auxiliary.auxiliary {
            Auxiliary::Be | Auxiliary::Have | Auxiliary::Do => Some(PredicateForm::Infinitive),
            Auxiliary::Can
            | Auxiliary::Could
            | Auxiliary::May
            | Auxiliary::Might
            | Auxiliary::Must
            | Auxiliary::Shall
            | Auxiliary::Should
            | Auxiliary::Will
            | Auxiliary::Would => Some(PredicateForm::Finite(None)),
        },
        AuxiliaryInflection::Present { person, number }
        | AuxiliaryInflection::Past { person, number } => {
            Some(PredicateForm::Finite(Some(Agreement { person, number })))
        }
        AuxiliaryInflection::PastSubjunctive => Some(PredicateForm::Finite(None)),
        AuxiliaryInflection::PresentParticiple => Some(PredicateForm::PresentParticiple),
        AuxiliaryInflection::PastParticiple => Some(PredicateForm::PastParticiple),
    }
}

pub(super) fn lower_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Verb
        | RuleTag::VerbPhraseBase
        | RuleTag::VerbPhraseAuxiliary
        | RuleTag::VerbPhraseAuxiliaryProform
        | RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhrasePreverbAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::VerbPhraseCausative
        | RuleTag::VerbPhraseCoordinatedAdjective
        | RuleTag::InfinitiveTo
        | RuleTag::InfinitiveNotTo => lower_predicate(tag, children),
        RuleTag::ManaAmountSymbol
        | RuleTag::ManaAmountSequence
        | RuleTag::ManaAmountListSingle
        | RuleTag::ManaAmountListComma
        | RuleTag::ManaAmountCoordination
        | RuleTag::ManaAmountCoordinationOxford => lower_mana_amount(tag, children),
        RuleTag::GerundClauseBase => {
            let Lowered::VerbPhrase(predicate) = take(children, 0)? else {
                return None;
            };
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            Some(Lowered::GerundClause(crate::syntax::GerundClause {
                predicate: Box::new(predicate),
                attachments: Vec::new(),
            }))
        }
        RuleTag::GerundClauseSubordinateAfter => {
            let Lowered::GerundClause(mut matrix) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(crate::syntax::Subordinator::RatherThan) = take(children, 1)?
            else {
                return None;
            };
            let Lowered::GerundClause(alternative) = take(children, 2)? else {
                return None;
            };
            matrix.attachments.push(DependentAttachment {
                position: AttachmentPosition::AfterMatrix,
                comma: false,
                clause: DependentClause::Subordinate(
                    crate::syntax::Subordinator::RatherThan,
                    SubordinateBody::Gerund(alternative),
                ),
            });
            Some(Lowered::GerundClause(matrix))
        }
        RuleTag::SimpleClauseSubject
        | RuleTag::SimpleClauseContractedSubject
        | RuleTag::SimpleClauseSubjectless
        | RuleTag::ClauseSimple
        | RuleTag::ClauseElliptical
        | RuleTag::ClauseExistential
        | RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderDistributiveEach
        | RuleTag::ClauseCopular
        | RuleTag::ClauseContractedCopular
        | RuleTag::RelativeObject
        | RuleTag::RelativeObjectContractedSubject
        | RuleTag::RelativeSubjectContractedAuxiliary
        | RuleTag::RelativeSubject
        | RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::CopularRemainderCoordinatedAdjective
        | RuleTag::RelativeContractedCopularCoordinatedAdjective => {
            lower_simple_clause(tag, children)
        }
        RuleTag::ClauseVariableValueConstraint => lower_variable_value_constraint(children),
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic
        | RuleTag::ClauseAdverbBefore
        | RuleTag::ClauseSentenceAdverbialBefore
        | RuleTag::ClausePrepositionalBefore
        | RuleTag::ClauseSubordinateBefore
        | RuleTag::ClauseSubordinateAfterElliptical
        | RuleTag::ClauseSubordinateAfter
        | RuleTag::ClauseSubordinateAfterComma
        | RuleTag::ClauseSubordinateAfterInfinitive
        | RuleTag::ExceptionRiderSingle
        | RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford
        | RuleTag::ClauseExcepted
        | RuleTag::ClauseRestrictionMember
        | RuleTag::ClauseRestrictionRun
        | RuleTag::Sentence => lower_composed_clause(tag, children),
        _ => None,
    }
}

fn lower_predicate(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::Verb => take(children, 0),
        RuleTag::VerbPhraseBase => {
            let Lowered::Verb(VerbAnalysis { instance, frame }) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::VerbPhrase(VerbPhrase {
                auxiliaries: Vec::new(),
                first_auxiliary_contracted_with_subject: false,
                preverb_modifiers: Vec::new(),
                verb: instance,
                frame,
                dependents: Vec::new(),
            }))
        }
        RuleTag::VerbPhraseAuxiliaryProform => {
            let Lowered::Auxiliary(auxiliary) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::VerbPhrase(VerbPhrase {
                auxiliaries: vec![auxiliary],
                first_auxiliary_contracted_with_subject: false,
                preverb_modifiers: Vec::new(),
                verb: VerbInstance {
                    verb: crate::word::Verb::Word(Vocab::Do),
                    slot: VerbSlot::Infinitive,
                },
                frame: crate::word::PROFORM_PREDICATE_FRAMES[0],
                dependents: Vec::new(),
            }))
        }
        RuleTag::VerbPhraseAuxiliary => {
            let Lowered::Auxiliary(auxiliary) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.auxiliaries.insert(0, auxiliary);
            Some(Lowered::VerbPhrase(predicate))
        }
        RuleTag::VerbPhrasePreverbAdverb => {
            // Not a §2.4 dependent site: children are swapped relative to the
            // sibling VerbPhraseAdverb (adverb at 0, VerbPhrase at 1), and the
            // modifier lands on preverb_modifiers rather than becoming a
            // VerbDependent — routing through PredicateAttachment::Adjunct
            // would render post-verbally (`cast next …`), a round-trip
            // failure. The literal matcher already pinned this token to
            // `next`, so the specific Vocab value need not be inspected here.
            let Lowered::Adverb(_next) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.preverb_modifiers.push(PreverbModifier::Next);
            Some(Lowered::VerbPhrase(predicate))
        }
        RuleTag::VerbPhraseCausative => {
            let Lowered::VerbPhrase(mut predicate) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(complement) = take(children, 1)? else {
                return None;
            };
            predicate
                .dependents
                .push(VerbDependent::Infinitive(InfinitiveClause {
                    negated: false,
                    marker: InfinitiveMarker::Bare,
                    predicate: Box::new(complement),
                }));
            Some(Lowered::VerbPhrase(predicate))
        }
        RuleTag::VerbPhraseDirectObject
        | RuleTag::VerbPhraseIndirectObject
        | RuleTag::VerbPhraseAdjective
        | RuleTag::VerbPhrasePrepositional
        | RuleTag::VerbPhraseInfinitive
        | RuleTag::VerbPhraseAdverb
        | RuleTag::VerbPhraseParticle
        | RuleTag::VerbPhraseFrequency
        | RuleTag::VerbPhraseAbility
        | RuleTag::VerbPhraseQuotedAbility
        | RuleTag::VerbPhraseQuotedAbilityCoordination
        | RuleTag::VerbPhraseAbilityQuotedCoordination
        | RuleTag::VerbPhraseOracleSymbol
        | RuleTag::VerbPhraseSymbolSequence
        | RuleTag::VerbPhraseManaAmountCoordination
        | RuleTag::VerbPhrasePowerToughness
        | RuleTag::VerbPhraseQuantity
        | RuleTag::VerbPhraseCoordinatedAdjective => lower_predicate_dependent(tag, children),
        RuleTag::InfinitiveTo | RuleTag::InfinitiveNotTo => {
            let negated = tag == RuleTag::InfinitiveNotTo;
            let predicate_index = if negated { 2 } else { 1 };
            let Lowered::VerbPhrase(predicate) = take(children, predicate_index)? else {
                return None;
            };
            Some(Lowered::InfinitiveClause(InfinitiveClause {
                negated,
                marker: InfinitiveMarker::To,
                predicate: Box::new(predicate),
            }))
        }
        _ => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "one arm per predicate-dependent rule tag is intentionally verbose"
)]
fn lower_predicate_dependent(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::VerbPhrase(mut predicate) = take(children, 0)? else {
        return None;
    };
    let dependent = match tag {
        RuleTag::VerbPhraseDirectObject => {
            let Lowered::NounPhrase(noun_phrase) = take(children, 1)? else {
                return None;
            };
            match lowered_nominal_adjunct_kind(&predicate, &noun_phrase) {
                Some(BareNominalAdjunct::Temporal) => VerbDependent::Temporal(noun_phrase),
                Some(BareNominalAdjunct::Manner) => VerbDependent::Manner(noun_phrase),
                None => VerbDependent::DirectObject(noun_phrase),
            }
        }
        RuleTag::VerbPhraseIndirectObject => {
            let Lowered::NounPhrase(noun_phrase) = take(children, 1)? else {
                return None;
            };
            VerbDependent::IndirectObject(noun_phrase)
        }
        RuleTag::VerbPhraseAdjective => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Adverbial(Phrase::AdjectivePhrase(Box::new(adjective)))
        }
        RuleTag::VerbPhraseCoordinatedAdjective => {
            let Lowered::CoordinatedModifier(coordinated) = take(children, 1)? else {
                return None;
            };
            VerbDependent::CoordinatedAdjective(coordinated_modifier_as_adjectives(coordinated)?)
        }
        RuleTag::VerbPhrasePrepositional => {
            let Lowered::PrepositionalPhrase(preposition) = take(children, 1)? else {
                return None;
            };
            match predicate
                .frame
                .prepositional_role(preposition.preposition)?
            {
                PrepositionalRole::SelectedComplement => VerbDependent::PredicateComplement(
                    Phrase::PrepositionalPhrase(Box::new(preposition)),
                ),
                PrepositionalRole::Adjunct => VerbDependent::Prepositional(preposition),
            }
        }
        RuleTag::VerbPhraseInfinitive => {
            let Lowered::InfinitiveClause(infinitive) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Infinitive(infinitive)
        }
        RuleTag::VerbPhraseAdverb => {
            let Lowered::Adverb(adverb) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Adverbial(Phrase::Adverb(adverb))
        }
        RuleTag::VerbPhraseParticle => {
            let Lowered::VerbParticle(particle) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Particle(particle)
        }
        RuleTag::VerbPhraseFrequency => {
            let Lowered::Frequency(frequency) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Frequency(frequency)
        }
        RuleTag::VerbPhraseAbility => {
            let Lowered::Catalog(atom) = take(children, 1)? else {
                return None;
            };
            VerbDependent::PredicateComplement(Phrase::CatalogAtom(atom))
        }
        RuleTag::VerbPhraseQuotedAbility => {
            let Lowered::Phrase(phrase @ Phrase::QuotedAbility(_)) = take(children, 1)? else {
                return None;
            };
            VerbDependent::PredicateComplement(phrase)
        }
        RuleTag::VerbPhraseQuotedAbilityCoordination => {
            let Lowered::Phrase(Phrase::QuotedAbility(first)) = take(children, 1)? else {
                return None;
            };
            let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                return None;
            };
            let Lowered::Phrase(Phrase::QuotedAbility(next)) = take(children, 3)? else {
                return None;
            };
            VerbDependent::CoordinatedObject(CoordinatedPredicateObject {
                first: Box::new(PredicateObject::QuotedAbility(first)),
                rest: vec![PredicateObjectCoordination {
                    conjunction: Some(conjunction),
                    comma: false,
                    object: PredicateObject::QuotedAbility(next),
                }],
            })
        }
        RuleTag::VerbPhraseAbilityQuotedCoordination => {
            let Lowered::Catalog(atom) = take(children, 1)? else {
                return None;
            };
            let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                return None;
            };
            let Lowered::Phrase(Phrase::QuotedAbility(next)) = take(children, 3)? else {
                return None;
            };
            VerbDependent::CoordinatedObject(CoordinatedPredicateObject {
                first: Box::new(PredicateObject::Ability(AbilityObject {
                    ability: atom,
                    argument: None,
                })),
                rest: vec![PredicateObjectCoordination {
                    conjunction: Some(conjunction),
                    comma: false,
                    object: PredicateObject::QuotedAbility(next),
                }],
            })
        }
        RuleTag::VerbPhraseOracleSymbol => {
            let Lowered::OracleSymbol(symbol) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Scalar(Phrase::OracleSymbol(symbol))
        }
        RuleTag::VerbPhraseSymbolSequence => {
            let Lowered::SymbolSequence(symbols) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Scalar(Phrase::SymbolSequence(symbols))
        }
        RuleTag::VerbPhraseManaAmountCoordination => {
            let Lowered::ManaAmount(PredicateObject::Coordinated(coordinated)) = take(children, 1)?
            else {
                return None;
            };
            VerbDependent::CoordinatedObject(coordinated)
        }
        RuleTag::VerbPhrasePowerToughness => {
            let Lowered::PowerToughness(power_toughness) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Statistic(Phrase::PowerToughness(power_toughness))
        }
        RuleTag::VerbPhraseQuantity => {
            let Lowered::Quantity(quantity) = take(children, 1)? else {
                return None;
            };
            VerbDependent::Scalar(Phrase::Quantity(quantity))
        }
        _ => return None,
    };
    predicate.dependents.push(dependent);
    Some(Lowered::VerbPhrase(predicate))
}

/// Lower the six `ManaAmount*` rules. Mirrors the noun-phrase list lowering
/// (`mod.rs`, `NounPhraseListSingle` / `NounPhraseListComma` /
/// `NounPhraseCoordinationOxford`) but stays typed to
/// [`PredicateObject`] so no symbol is ever wrapped as a `NounPhrase`.
fn lower_mana_amount(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    fn member(lowered: Lowered) -> Option<PredicateObject> {
        let Lowered::ManaAmount(object) = lowered else {
            return None;
        };
        Some(object)
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "mirrors the by-value take() idiom used throughout this module"
    )]
    fn mana_conjunction(lowered: Lowered) -> Option<crate::syntax::PredicateConjunction> {
        let Lowered::Conjunction(conjunction) = lowered else {
            return None;
        };
        match conjunction {
            crate::syntax::PredicateConjunction::And | crate::syntax::PredicateConjunction::Or => {
                Some(conjunction)
            }
            // `then`/`and-or` never join predicate objects.
            crate::syntax::PredicateConjunction::Then
            | crate::syntax::PredicateConjunction::AndOr => None,
        }
    }

    match tag {
        RuleTag::ManaAmountSymbol => {
            let Lowered::OracleSymbol(symbol) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::ManaAmount(PredicateObject::OracleSymbol(symbol)))
        }
        RuleTag::ManaAmountSequence => {
            let Lowered::SymbolSequence(symbols) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::ManaAmount(PredicateObject::SymbolSequence(
                symbols,
            )))
        }
        RuleTag::ManaAmountListSingle => take(children, 0),
        RuleTag::ManaAmountListComma => {
            let first = member(take(children, 0)?)?;
            let next = member(take(children, 2)?)?;
            let coordination = PredicateObjectCoordination {
                conjunction: None,
                comma: true,
                object: next,
            };
            Some(Lowered::ManaAmount(PredicateObject::Coordinated(
                push_mana_coordination(first, coordination),
            )))
        }
        RuleTag::ManaAmountCoordination => {
            let first = member(take(children, 0)?)?;
            let conjunction = mana_conjunction(take(children, 1)?)?;
            let next = member(take(children, 2)?)?;
            let coordination = PredicateObjectCoordination {
                conjunction: Some(conjunction),
                comma: false,
                object: next,
            };
            Some(Lowered::ManaAmount(PredicateObject::Coordinated(
                push_mana_coordination(first, coordination),
            )))
        }
        RuleTag::ManaAmountCoordinationOxford => {
            let first = member(take(children, 0)?)?;
            let conjunction = mana_conjunction(take(children, 2)?)?;
            let next = member(take(children, 3)?)?;
            let coordination = PredicateObjectCoordination {
                conjunction: Some(conjunction),
                comma: true,
                object: next,
            };
            Some(Lowered::ManaAmount(PredicateObject::Coordinated(
                push_mana_coordination(first, coordination),
            )))
        }
        _ => None,
    }
}

/// Mirrors `push_noun_phrase_coordination` (`mod.rs`): fold a new coordination
/// member onto an already-coordinated first object, or start a fresh
/// coordinated run.
fn push_mana_coordination(
    first: PredicateObject,
    coordination: PredicateObjectCoordination,
) -> CoordinatedPredicateObject {
    match first {
        PredicateObject::Coordinated(mut coordinated) => {
            coordinated.rest.push(coordination);
            coordinated
        }
        first => CoordinatedPredicateObject {
            first: Box::new(first),
            rest: vec![coordination],
        },
    }
}

#[allow(clippy::too_many_lines, reason = "lowering has many grammar variants")]
fn lower_simple_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::SimpleClauseSubject => {
            let Lowered::NounPhrase(subject) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::SimpleClause(SimpleClause {
                subject: Some(Subject(subject)),
                predicate,
            }))
        }
        RuleTag::SimpleClauseContractedSubject => {
            let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.auxiliaries.insert(0, subject_auxiliary.auxiliary);
            predicate.first_auxiliary_contracted_with_subject = true;
            Some(Lowered::SimpleClause(SimpleClause {
                subject: Some(subject_auxiliary.subject),
                predicate,
            }))
        }
        RuleTag::SimpleClauseSubjectless => {
            let Lowered::VerbPhrase(predicate) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::SimpleClause(SimpleClause {
                subject: None,
                predicate,
            }))
        }
        RuleTag::ClauseSimple => {
            let Lowered::SimpleClause(simple) = take(children, 0)? else {
                return None;
            };
            let independent = finish_simple_clause(simple)?;
            Some(Lowered::Clause(Clause::Independent(independent)))
        }
        RuleTag::ClauseElliptical => {
            let Lowered::AdjectivePhrase(adjective) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::EllipticalClause(EllipticalClause::Adjective(
                adjective,
            )))
        }
        RuleTag::ClauseExistential => {
            let Lowered::Existential(form) = take(children, 0)? else {
                return None;
            };
            let Lowered::NounPhrase(pivot) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Independent(
                IndependentClause::Existential(crate::syntax::ExistentialClause {
                    form,
                    pivot,
                    adjuncts: vec![],
                }),
            )))
        }
        tag @ (RuleTag::CopularRemainderNoun
        | RuleTag::CopularRemainderAdjective
        | RuleTag::CopularRemainderPrepositional
        | RuleTag::CopularRemainderPowerToughness
        | RuleTag::CopularRemainderPrepositionalAdjunct
        | RuleTag::CopularRemainderAdverb
        | RuleTag::CopularRemainderDistributiveEach
        | RuleTag::CopularRemainderCoordinatedAdjective) => lower_copular_remainder(tag, children),
        tag @ (RuleTag::ClauseCopular | RuleTag::ClauseContractedCopular) => {
            lower_copular_clause(tag, children)
        }
        RuleTag::RelativeObject => {
            let Lowered::NounPhrase(subject) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            let Predicate::Intransitive(predicate) = predicate else {
                return None;
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: RelativeGap::Object,
                body: RelativeBody::ObjectGap {
                    subject: Subject(subject),
                    predicate: ObjectGapPredicate {
                        head: predicate.head,
                        elements: predicate.elements,
                    },
                },
            }))
        }
        RuleTag::RelativeObjectContractedSubject => {
            let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.auxiliaries.insert(0, subject_auxiliary.auxiliary);
            predicate.first_auxiliary_contracted_with_subject = true;
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            let Predicate::Intransitive(predicate) = predicate else {
                return None;
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: RelativeGap::Object,
                body: RelativeBody::ObjectGap {
                    subject: subject_auxiliary.subject,
                    predicate: ObjectGapPredicate {
                        head: predicate.head,
                        elements: predicate.elements,
                    },
                },
            }))
        }
        RuleTag::RelativeSubjectContractedAuxiliary => {
            let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(mut predicate) = take(children, 1)? else {
                return None;
            };
            predicate.auxiliaries.insert(0, subject_auxiliary.auxiliary);
            predicate.first_auxiliary_contracted_with_subject = true;
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::That,
                gap: RelativeGap::Subject,
                body: RelativeBody::SubjectGap(predicate),
            }))
        }
        RuleTag::RelativeSubject => {
            let Lowered::RelativeMarker(marker) = take(children, 0)? else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 1)? else {
                return None;
            };
            let FinishedPredicate {
                modal,
                predicate,
                elided,
            } = finish_predicate(predicate)?;
            // Only VP-ellipsis under the modal ("creature that can't") drops the
            // predicate. A modal with a real, complement-less verb ("damage that
            // would be", "creature that would die") keeps it — nulling those on
            // the loose "intransitive with empty elements" test silently ate the
            // verb (e.g. the "be" of "would be dealt").
            let body = match modal {
                Some(modal) => RelativeBody::ModalSubjectGap {
                    modal,
                    predicate: if elided { None } else { Some(predicate) },
                },
                None => RelativeBody::SubjectGap(predicate),
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker,
                gap: RelativeGap::Subject,
                body,
            }))
        }
        RuleTag::RelativeContractedCopularNoun
        | RuleTag::RelativeContractedCopularAdjective
        | RuleTag::RelativeContractedCopularPrepositional
        | RuleTag::RelativeContractedCopularCoordinatedAdjective => {
            let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
                return None;
            };
            let complement = match tag {
                RuleTag::RelativeContractedCopularNoun => {
                    let Lowered::NounPhrase(complement) = take(children, 1)? else {
                        return None;
                    };
                    crate::syntax::CopularComplement::NounPhrase(complement)
                }
                RuleTag::RelativeContractedCopularAdjective => {
                    let Lowered::AdjectivePhrase(complement) = take(children, 1)? else {
                        return None;
                    };
                    crate::syntax::CopularComplement::Adjective(complement)
                }
                RuleTag::RelativeContractedCopularCoordinatedAdjective => {
                    let Lowered::CoordinatedModifier(coordinated) = take(children, 1)? else {
                        return None;
                    };
                    crate::syntax::CopularComplement::CoordinatedAdjective(
                        coordinated_modifier_as_adjectives(coordinated)?,
                    )
                }
                RuleTag::RelativeContractedCopularPrepositional => {
                    let Lowered::PrepositionalPhrase(complement) = take(children, 1)? else {
                        return None;
                    };
                    crate::syntax::CopularComplement::Prepositional(complement)
                }
                _ => return None,
            };
            Some(Lowered::RelativeClause(RelativeClause {
                marker: RelativeMarker::That,
                gap: RelativeGap::Subject,
                body: RelativeBody::SubjectGap(Predicate::Copular(
                    crate::syntax::CopularPredicate {
                        copula: crate::syntax::Copula {
                            auxiliary: subject_auxiliary.auxiliary,
                            contracted_with_subject: true,
                        },
                        distributive_each: false,
                        precomplement_adverbs: vec![],
                        complement,
                        adjuncts: vec![],
                    },
                )),
            }))
        }
        _ => None,
    }
}

fn lower_copular_remainder(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let remainder = match tag {
        RuleTag::CopularRemainderNoun => {
            let Lowered::NounPhrase(complement) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::NounPhrase(complement),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderAdjective => {
            let Lowered::AdjectivePhrase(complement) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::Adjective(complement),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderCoordinatedAdjective => {
            let Lowered::CoordinatedModifier(coordinated) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::CoordinatedAdjective(
                    coordinated_modifier_as_adjectives(coordinated)?,
                ),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderPrepositional => {
            let Lowered::PrepositionalPhrase(complement) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::Prepositional(complement),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderPowerToughness => {
            let Lowered::PowerToughness(power_toughness) = take(children, 0)? else {
                return None;
            };
            CopularRemainder {
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::PowerToughness(power_toughness),
                adjuncts: Vec::new(),
            }
        }
        RuleTag::CopularRemainderPrepositionalAdjunct => {
            let Lowered::CopularRemainder(mut remainder) = take(children, 0)? else {
                return None;
            };
            let Lowered::PrepositionalPhrase(adjunct) = take(children, 1)? else {
                return None;
            };
            remainder
                .adjuncts
                .push(PredicateAdjunct::Prepositional(adjunct));
            remainder
        }
        RuleTag::CopularRemainderAdverb => {
            let Lowered::Adverb(adverb) = take(children, 0)? else {
                return None;
            };
            let Lowered::CopularRemainder(mut remainder) = take(children, 1)? else {
                return None;
            };
            remainder.precomplement_adverbs.insert(0, adverb);
            remainder
        }
        RuleTag::CopularRemainderDistributiveEach => {
            // `each` (index 0) is the floating quantifier — carried as a flag,
            // its lexical child discarded. The adjective (`equal`) takes the
            // trailing prepositional phrase (`to X`) as its own complement so
            // the standard stays bound to the adjective rather than floating as
            // a clause adjunct.
            let Lowered::AdjectivePhrase(mut adjective) = take(children, 1)? else {
                return None;
            };
            let Lowered::PrepositionalPhrase(standard) = take(children, 2)? else {
                return None;
            };
            adjective
                .complements
                .push(crate::syntax::AdjectiveComplement::Prepositional(standard));
            CopularRemainder {
                distributive_each: true,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::Adjective(adjective),
                adjuncts: Vec::new(),
            }
        }
        _ => return None,
    };
    Some(Lowered::CopularRemainder(remainder))
}

fn lower_copular_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let contracted = tag == RuleTag::ClauseContractedCopular;
    let (subject, copula, remainder_index) = if contracted {
        let Lowered::SubjectAuxiliary(subject_auxiliary) = take(children, 0)? else {
            return None;
        };
        if subject_auxiliary.auxiliary.auxiliary != Auxiliary::Be {
            return None;
        }
        (
            subject_auxiliary.subject,
            crate::syntax::Copula {
                auxiliary: subject_auxiliary.auxiliary,
                contracted_with_subject: true,
            },
            1,
        )
    } else {
        let Lowered::NounPhrase(subject) = take(children, 0)? else {
            return None;
        };
        let Lowered::Auxiliary(auxiliary) = take(children, 1)? else {
            return None;
        };
        (
            Subject(subject),
            crate::syntax::Copula {
                auxiliary,
                contracted_with_subject: false,
            },
            2,
        )
    };
    let Lowered::CopularRemainder(remainder) = take(children, remainder_index)? else {
        return None;
    };
    Some(Lowered::Clause(Clause::Independent(
        IndependentClause::Copular(
            subject,
            crate::syntax::CopularPredicate {
                copula,
                distributive_each: remainder.distributive_each,
                precomplement_adverbs: remainder.precomplement_adverbs,
                complement: remainder.complement,
                adjuncts: remainder.adjuncts,
            },
        ),
    )))
}

fn lower_variable_value_constraint(children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::Quantity(subject_quantity @ Quantity::X) = take(children, 0)? else {
        return None;
    };
    let Lowered::Auxiliary(modal) = take(children, 1)? else {
        return None;
    };
    let Lowered::Auxiliary(be) = take(children, 2)? else {
        return None;
    };
    let Lowered::Number(number) = take(children, 3)? else {
        return None;
    };
    Some(Lowered::Clause(Clause::Independent(
        IndependentClause::Deontic(
            Subject(NounPhrase::Quantity(subject_quantity)),
            Modal { auxiliary: modal },
            Some(Predicate::Copular(crate::syntax::CopularPredicate {
                copula: crate::syntax::Copula {
                    auxiliary: be,
                    contracted_with_subject: false,
                },
                distributive_each: false,
                precomplement_adverbs: Vec::new(),
                complement: CopularComplement::NounPhrase(NounPhrase::Quantity(Quantity::Exact(
                    number.literal(),
                ))),
                adjuncts: Vec::new(),
            })),
        ),
    )))
}

#[allow(
    clippy::too_many_lines,
    reason = "one arm per composed-clause rule tag is intentionally verbose"
)]
fn lower_composed_clause(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    match tag {
        RuleTag::ClauseCoordination
        | RuleTag::ClauseCoordinationComma
        | RuleTag::ClauseCoordinationAsyndetic => lower_coordination(tag, children),
        RuleTag::ClauseSubordinateBefore => {
            let Lowered::Subordinator(subordinator) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(condition) = take(children, 1)? else {
                return None;
            };
            let Lowered::Clause(consequence) = take(children, 3)? else {
                return None;
            };
            conditional(
                subordinator,
                AttachmentPosition::BeforeMatrix,
                true,
                condition,
                consequence,
            )
        }
        RuleTag::ClauseAdverbBefore => {
            let Lowered::Adverb(adverb) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(Clause::Independent(matrix)) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Independent(
                with_clause_attachment(
                    matrix,
                    ClauseAttachment {
                        position: AttachmentPosition::BeforeMatrix,
                        comma: false,
                        kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(adverb)),
                    },
                ),
            )))
        }
        RuleTag::ClauseSentenceAdverbialBefore => {
            let Lowered::Adverb(adverb) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(Clause::Independent(matrix)) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Independent(
                with_clause_attachment(
                    matrix,
                    ClauseAttachment {
                        position: AttachmentPosition::BeforeMatrix,
                        comma: true,
                        kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(adverb)),
                    },
                ),
            )))
        }
        RuleTag::ClausePrepositionalBefore => {
            let Lowered::PrepositionalPhrase(preposition) = take(children, 0)? else {
                return None;
            };
            let Lowered::Clause(Clause::Independent(matrix)) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Independent(
                with_clause_attachment(
                    matrix,
                    ClauseAttachment {
                        position: AttachmentPosition::BeforeMatrix,
                        comma: true,
                        kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(
                            preposition,
                        )),
                    },
                ),
            )))
        }
        RuleTag::ClauseSubordinateAfterElliptical => {
            let Lowered::Clause(consequence) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(subordinator) = take(children, 1)? else {
                return None;
            };
            let Lowered::AdjectivePhrase(condition) = take(children, 2)? else {
                return None;
            };
            conditional_body(
                subordinator,
                AttachmentPosition::AfterMatrix,
                false,
                SubordinateBody::Elliptical(EllipticalClause::Adjective(condition)),
                consequence,
            )
        }
        RuleTag::ClauseSubordinateAfter | RuleTag::ClauseSubordinateAfterComma => {
            let offset = usize::from(tag == RuleTag::ClauseSubordinateAfterComma);
            let Lowered::Clause(consequence) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(subordinator) = take(children, 1 + offset)? else {
                return None;
            };
            let Lowered::Clause(condition) = take(children, 2 + offset)? else {
                return None;
            };
            conditional(
                subordinator,
                AttachmentPosition::AfterMatrix,
                tag == RuleTag::ClauseSubordinateAfterComma,
                condition,
                consequence,
            )
        }
        RuleTag::ClauseSubordinateAfterInfinitive => {
            let Lowered::Clause(consequence) = take(children, 0)? else {
                return None;
            };
            let Lowered::Subordinator(crate::syntax::Subordinator::RatherThan) = take(children, 1)?
            else {
                return None;
            };
            let Lowered::VerbPhrase(predicate) = take(children, 2)? else {
                return None;
            };
            let FinishedPredicate {
                modal, predicate, ..
            } = finish_predicate(predicate)?;
            if modal.is_some() {
                return None;
            }
            conditional_body(
                crate::syntax::Subordinator::RatherThan,
                AttachmentPosition::AfterMatrix,
                false,
                SubordinateBody::Infinitive(crate::syntax::InfinitiveClause {
                    negated: false,
                    marker: InfinitiveMarker::Bare,
                    predicate: Box::new(predicate),
                }),
                consequence,
            )
        }
        RuleTag::ExceptionRiderSingle => {
            let Lowered::Clause(Clause::Independent(first)) = take(children, 1)? else {
                return None;
            };
            Some(Lowered::ExceptionRider(ExceptionRider {
                first: Box::new(first),
                rest: Vec::new(),
            }))
        }
        RuleTag::ExceptionRiderConjoined
        | RuleTag::ExceptionRiderComma
        | RuleTag::ExceptionRiderOxford => {
            let Lowered::ExceptionRider(mut rider) = take(children, 0)? else {
                return None;
            };
            let (conjunction, clause_index, comma) = match tag {
                RuleTag::ExceptionRiderConjoined => (Some(1), 2, false),
                RuleTag::ExceptionRiderComma => (None, 2, true),
                RuleTag::ExceptionRiderOxford => (Some(2), 3, true),
                _ => return None,
            };
            let conjunction = match conjunction {
                Some(index) => {
                    let Lowered::Conjunction(conjunction) = take(children, index)? else {
                        return None;
                    };
                    Some(conjunction)
                }
                None => None,
            };
            let Lowered::Clause(Clause::Independent(clause)) = take(children, clause_index)? else {
                return None;
            };
            rider.rest.push(ExceptionConjunct {
                conjunction,
                comma,
                clause,
            });
            Some(Lowered::ExceptionRider(rider))
        }
        RuleTag::ClauseExcepted => {
            let Lowered::Clause(Clause::Independent(matrix)) = take(children, 0)? else {
                return None;
            };
            let Lowered::ExceptionRider(rider) = take(children, 2)? else {
                return None;
            };
            Some(Lowered::Clause(Clause::Independent(
                with_clause_attachment(
                    matrix,
                    ClauseAttachment {
                        position: AttachmentPosition::AfterMatrix,
                        comma: true,
                        kind: ClauseAttachmentKind::Exception(rider),
                    },
                ),
            )))
        }
        RuleTag::ClauseRestrictionMember => {
            let Lowered::Adverb(only) = take(children, 0)? else {
                return None;
            };
            // A bare `only only` (nesting) is unrepresentable: the member
            // payload is never itself the `only` adverb.
            if only != crate::word::Vocab::Only {
                return None;
            }
            let adjuncts = match (children.len(), take(children, 1)?) {
                (2, Lowered::PrepositionalPhrase(preposition)) => {
                    vec![PredicateAdjunct::Prepositional(preposition)]
                }
                (2, Lowered::Adverb(word)) => {
                    if word.spelling() != "once" {
                        return None;
                    }
                    vec![PredicateAdjunct::Adverb(word)]
                }
                (3, Lowered::Subordinator(subordinator)) => {
                    if subordinator != crate::syntax::Subordinator::If {
                        return None;
                    }
                    let Lowered::Clause(Clause::Independent(condition)) = take(children, 2)? else {
                        return None;
                    };
                    vec![PredicateAdjunct::Dependent(Box::new(
                        DependentClause::Subordinate(
                            crate::syntax::Subordinator::If,
                            SubordinateBody::Finite(Box::new(condition)),
                        ),
                    ))]
                }
                (3, Lowered::Adverb(word)) => {
                    if word.spelling() != "once" {
                        return None;
                    }
                    let Lowered::NounPhrase(temporal) = take(children, 2)? else {
                        return None;
                    };
                    vec![
                        PredicateAdjunct::Adverb(word),
                        PredicateAdjunct::Temporal(temporal),
                    ]
                }
                _ => return None,
            };
            Some(Lowered::RestrictionMember(adjuncts))
        }
        RuleTag::ClauseRestrictionRun => match children.len() {
            2 => {
                let Lowered::Clause(Clause::Independent(matrix)) = take(children, 0)? else {
                    return None;
                };
                let Lowered::RestrictionRun(run) = take(children, 1)? else {
                    return None;
                };
                Some(Lowered::Clause(Clause::Independent(
                    with_clause_attachment(
                        matrix,
                        ClauseAttachment {
                            position: AttachmentPosition::AfterMatrix,
                            comma: false,
                            kind: ClauseAttachmentKind::Restriction(run),
                        },
                    ),
                )))
            }
            3 => match (take(children, 0)?, take(children, 1)?) {
                (Lowered::RestrictionMember(first), Lowered::Conjunction(conjunction)) => {
                    // Restrictions are cumulative [CR#601.3,602.5]: only a
                    // bare `and` joins members. `or`/`then`/`and-or` must keep
                    // failing.
                    if conjunction != crate::syntax::PredicateConjunction::And {
                        return None;
                    }
                    let Lowered::RestrictionMember(next) = take(children, 2)? else {
                        return None;
                    };
                    Some(Lowered::RestrictionRun(RestrictionRun {
                        first,
                        rest: vec![RestrictionCoordination {
                            conjunction: Some(conjunction),
                            comma: false,
                            adjuncts: next,
                        }],
                    }))
                }
                (Lowered::RestrictionMember(first), Lowered::Ignored) => {
                    // The comma-joined two-member base.
                    let Lowered::RestrictionMember(next) = take(children, 2)? else {
                        return None;
                    };
                    Some(Lowered::RestrictionRun(RestrictionRun {
                        first,
                        rest: vec![RestrictionCoordination {
                            conjunction: None,
                            comma: true,
                            adjuncts: next,
                        }],
                    }))
                }
                (Lowered::RestrictionRun(mut run), Lowered::Ignored) => {
                    let Lowered::RestrictionMember(next) = take(children, 2)? else {
                        return None;
                    };
                    run.rest.push(RestrictionCoordination {
                        conjunction: None,
                        comma: true,
                        adjuncts: next,
                    });
                    Some(Lowered::RestrictionRun(run))
                }
                _ => None,
            },
            4 => {
                let Lowered::RestrictionRun(mut run) = take(children, 0)? else {
                    return None;
                };
                let Lowered::Conjunction(conjunction) = take(children, 2)? else {
                    return None;
                };
                if conjunction != crate::syntax::PredicateConjunction::And {
                    return None;
                }
                let Lowered::RestrictionMember(next) = take(children, 3)? else {
                    return None;
                };
                run.rest.push(RestrictionCoordination {
                    conjunction: Some(conjunction),
                    comma: true,
                    adjuncts: next,
                });
                Some(Lowered::RestrictionRun(run))
            }
            _ => None,
        },
        RuleTag::Sentence => {
            let Lowered::Clause(Clause::Independent(clause)) = take(children, 0)? else {
                return None;
            };
            Some(Lowered::Sentence(Sentence {
                initial_uppercase: true,
                body: SentenceBody::Independent(clause),
            }))
        }
        _ => None,
    }
}

fn lower_coordination(tag: RuleTag, children: &mut [Lowered]) -> Option<Lowered> {
    let Lowered::Clause(Clause::Independent(first)) = take(children, 0)? else {
        return None;
    };
    let (conjunction_index, clause_index, comma) = match tag {
        RuleTag::ClauseCoordination => (Some(1), 2, false),
        RuleTag::ClauseCoordinationComma => (Some(2), 3, true),
        RuleTag::ClauseCoordinationAsyndetic => (None, 2, true),
        _ => return None,
    };
    let conjunction = match conjunction_index {
        Some(index) => {
            let Lowered::Conjunction(conjunction) = take(children, index)? else {
                return None;
            };
            Some(conjunction)
        }
        None => None,
    };
    let Lowered::SimpleClause(next) = take(children, clause_index)? else {
        return None;
    };
    let member = if next.subject.is_some() {
        CoordinatedClauseMember::Independent(Box::new(finish_simple_clause(next)?))
    } else {
        let FinishedPredicate {
            modal, predicate, ..
        } = finish_predicate(next.predicate)?;
        if modal.is_some() {
            return None;
        }
        CoordinatedClauseMember::SharedPredicate(predicate)
    };
    let coordination = ClauseCoordination {
        conjunction,
        comma,
        member,
    };
    let coordinated = match first {
        IndependentClause::Coordinated(mut coordinated) => {
            coordinated.rest.push(coordination);
            coordinated
        }
        first => CoordinatedIndependentClause {
            first: Box::new(first),
            rest: vec![coordination],
        },
    };
    Some(Lowered::Clause(Clause::Independent(
        IndependentClause::Coordinated(coordinated),
    )))
}

fn conditional(
    subordinator: crate::syntax::Subordinator,
    position: AttachmentPosition,
    comma: bool,
    condition: Clause,
    consequence: Clause,
) -> Option<Lowered> {
    let Clause::Independent(condition) = condition else {
        return None;
    };
    conditional_body(
        subordinator,
        position,
        comma,
        SubordinateBody::Finite(Box::new(condition)),
        consequence,
    )
}

fn conditional_body(
    subordinator: crate::syntax::Subordinator,
    position: AttachmentPosition,
    comma: bool,
    body: SubordinateBody,
    consequence: Clause,
) -> Option<Lowered> {
    let Clause::Independent(matrix) = consequence else {
        return None;
    };
    Some(Lowered::Clause(Clause::Independent(
        with_clause_attachment(
            matrix,
            ClauseAttachment {
                position,
                comma,
                kind: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                    subordinator,
                    body,
                )),
            },
        ),
    )))
}

fn with_clause_attachment(
    matrix: IndependentClause,
    attachment: ClauseAttachment,
) -> IndependentClause {
    match matrix {
        IndependentClause::Complex(mut complex) => {
            match attachment.position {
                AttachmentPosition::BeforeMatrix => complex.attachments.insert(0, attachment),
                AttachmentPosition::AfterMatrix => complex.attachments.push(attachment),
            }
            IndependentClause::Complex(complex)
        }
        matrix => IndependentClause::Complex(ComplexClause {
            matrix: Box::new(matrix),
            attachments: vec![attachment],
        }),
    }
}

struct FinishedPredicate {
    modal: Option<Modal>,
    predicate: Predicate,
    /// True when the verb phrase was an elided proform under a modal, i.e.
    /// VP-ellipsis ("If you can't, …"). The `predicate` field then holds the
    /// synthesized proform as a placeholder; consumers building a modal clause
    /// drop it so the modal renders alone.
    elided: bool,
}

pub(super) fn finish_simple_clause(simple: SimpleClause) -> Option<IndependentClause> {
    let imperative = simple.subject.is_none() && simple.predicate.verb.slot == VerbSlot::Imperative;
    let subject = simple.subject;
    let FinishedPredicate {
        modal,
        predicate,
        elided,
    } = finish_predicate(simple.predicate)?;
    match (subject, modal, imperative) {
        (None, None, true) => Some(IndependentClause::Imperative(predicate)),
        (Some(subject), Some(modal), false) => Some(IndependentClause::Deontic(
            subject,
            modal,
            if elided { None } else { Some(predicate) },
        )),
        (Some(subject), None, false) => Some(independent_with_subject(subject, predicate)),
        _ => None,
    }
}

fn independent_with_subject(subject: Subject, predicate: Predicate) -> IndependentClause {
    match predicate {
        Predicate::Transitive(predicate) => IndependentClause::Transitive(subject, predicate),
        Predicate::Intransitive(predicate) => IndependentClause::Intransitive(subject, predicate),
        Predicate::Copular(predicate) => IndependentClause::Copular(subject, predicate),
        Predicate::Passive(predicate) => IndependentClause::Passive(subject, predicate),
        Predicate::Proform(predicate) => IndependentClause::Proform(subject, predicate),
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "sentence assembly is intentionally verbose"
)]
/// Converts a parsed
/// [`CoordinatedModifier`](crate::syntax::CoordinatedModifier)
/// into a predicative
/// [`CoordinatedAdjectivePhrase`](crate::syntax::CoordinatedAdjectivePhrase),
/// keeping only positive attributive-adjective conjuncts. Returns `None` when
/// any conjunct is a noun, a `non-` negated modifier, a quantity, or a
/// power/toughness — none of those predicate coordinately in a copular
/// position, so rejecting them keeps the attributive-only shapes out of the
/// predicative slot.
fn coordinated_modifier_as_adjectives(
    modifier: crate::syntax::CoordinatedModifier,
) -> Option<crate::syntax::CoordinatedAdjectivePhrase> {
    let first = modifier_as_predicative_adjective(*modifier.first)?;
    let mut rest = Vec::with_capacity(modifier.rest.len());
    for coordination in modifier.rest {
        rest.push(crate::syntax::AdjectivePhraseCoordination {
            conjunction: coordination.conjunction,
            comma: coordination.comma,
            phrase: modifier_as_predicative_adjective(coordination.modifier)?,
        });
    }
    Some(crate::syntax::CoordinatedAdjectivePhrase {
        first: Box::new(first),
        rest,
    })
}

fn modifier_as_predicative_adjective(
    modifier: crate::syntax::NominalModifier,
) -> Option<crate::syntax::AdjectivePhrase> {
    match modifier {
        crate::syntax::NominalModifier::Adjective {
            polarity: crate::syntax::Polarity::Positive,
            phrase,
        } => Some(phrase),
        _ => None,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "one arm per verb dependent is intentionally verbose"
)]
fn finish_predicate(mut phrase: VerbPhrase) -> Option<FinishedPredicate> {
    let proform = phrase.frame.is_proform();
    let modal = phrase
        .auxiliaries
        .first()
        .copied()
        .filter(|auxiliary| is_modal(auxiliary.auxiliary))
        .map(|auxiliary| {
            phrase.auxiliaries.remove(0);
            Modal { auxiliary }
        });
    let passive = phrase.verb.slot == VerbSlot::PastParticiple
        && phrase
            .auxiliaries
            .first()
            .is_some_and(|auxiliary| auxiliary.auxiliary == Auxiliary::Be);
    let mut object = None;
    let mut pre_object_elements = Vec::new();
    let mut elements = Vec::new();
    for dependent in phrase.dependents {
        let target_elements =
            if object.is_none() { &mut pre_object_elements } else { &mut elements };
        match dependent {
            VerbDependent::DirectObject(noun_phrase) => {
                attach_object(&mut object, PredicateObject::NounPhrase(noun_phrase))?;
            }
            VerbDependent::IndirectObject(noun_phrase) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::IndirectObject(noun_phrase),
                ));
            }
            VerbDependent::PredicateComplement(phrase) => match phrase {
                Phrase::CatalogAtom(atom) => {
                    attach_object(
                        &mut object,
                        PredicateObject::Ability(AbilityObject {
                            ability: atom,
                            argument: None,
                        }),
                    )?;
                }
                Phrase::EmbeddedAbility(ability) => {
                    attach_object(&mut object, PredicateObject::EmbeddedAbility(ability))?;
                }
                Phrase::QuotedAbility(ability) => {
                    attach_object(&mut object, PredicateObject::QuotedAbility(ability))?;
                }
                Phrase::AdjectivePhrase(adjective) => {
                    target_elements.push(PredicateElement::Complement(
                        PredicateComplement::Adjective(*adjective),
                    ));
                }
                Phrase::PrepositionalPhrase(preposition) => {
                    target_elements.push(PredicateElement::Complement(
                        PredicateComplement::Prepositional(*preposition),
                    ));
                }
                _ => return None,
            },
            VerbDependent::Scalar(phrase) => match phrase {
                Phrase::Quantity(quantity) => {
                    attach_object(&mut object, PredicateObject::Quantity(quantity))?;
                }
                Phrase::OracleSymbol(symbol) => {
                    attach_object(&mut object, PredicateObject::OracleSymbol(symbol))?;
                }
                Phrase::SymbolSequence(symbols) => {
                    attach_object(&mut object, PredicateObject::SymbolSequence(symbols))?;
                }
                _ => return None,
            },
            VerbDependent::Statistic(Phrase::PowerToughness(value)) => {
                attach_object(&mut object, PredicateObject::PowerToughness(value))?;
            }
            VerbDependent::Prepositional(phrase) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Prepositional(
                    phrase,
                )));
            }
            VerbDependent::Temporal(phrase) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Temporal(
                    phrase,
                )));
            }
            VerbDependent::Manner(phrase) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Manner(phrase)));
            }
            VerbDependent::Infinitive(clause) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::Infinitive(finish_infinitive(clause)?),
                ));
            }
            VerbDependent::Subordinate(clause) => {
                let Clause::Dependent(clause) = *clause else {
                    return None;
                };
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Dependent(
                    Box::new(clause),
                )));
            }
            VerbDependent::Adverbial(Phrase::Adverb(adverb)) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Adverb(adverb)));
            }
            VerbDependent::Adverbial(Phrase::AdjectivePhrase(adjective)) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::Adjective(*adjective),
                ));
            }
            VerbDependent::Statistic(_) | VerbDependent::Adverbial(_) => return None,
            VerbDependent::Particle(particle) => {
                target_elements.push(PredicateElement::Particle(particle));
            }
            VerbDependent::Frequency(frequency) => {
                target_elements.push(PredicateElement::Adjunct(PredicateAdjunct::Frequency(
                    frequency,
                )));
            }
            VerbDependent::CoordinatedObject(coordinated) => {
                attach_object(&mut object, PredicateObject::Coordinated(coordinated))?;
            }
            VerbDependent::CoordinatedAdjective(coordinated) => {
                target_elements.push(PredicateElement::Complement(
                    PredicateComplement::CoordinatedAdjective(coordinated),
                ));
            }
        }
    }
    let head = PredicateHead {
        auxiliaries: phrase.auxiliaries,
        first_auxiliary_contracted_with_subject: phrase.first_auxiliary_contracted_with_subject,
        preverb_modifiers: phrase.preverb_modifiers,
        verb: phrase.verb,
    };
    // A bare proform under a modal (nothing surviving beside the modal) is
    // VP-ellipsis: "If you can't, …". The synthesized `do` pro-verb is a
    // placeholder the surface never spelled, so flag it for the modal-clause
    // builder to drop rather than render.
    let elided = proform
        && modal.is_some()
        && object.is_none()
        && head.auxiliaries.is_empty()
        && head.preverb_modifiers.is_empty()
        && pre_object_elements.is_empty()
        && elements.is_empty();
    let predicate = if passive {
        let retained_object = match object {
            None => None,
            Some(object) => {
                if !phrase.frame.is_recipient_passive() {
                    return None;
                }
                // Surface order is head → retained object → elements. Any
                // element that landed BEFORE the object would be lost by the
                // flattening below, so refuse rather than render a reordered
                // face. No corpus witness exists; if one appears, model the
                // position explicitly.
                if !pre_object_elements.is_empty() {
                    return None;
                }
                Some(object)
            }
        };
        pre_object_elements.append(&mut elements);
        Predicate::Passive(PassivePredicate {
            head,
            retained_object,
            elements: pre_object_elements,
        })
    } else if let Some(object) = object {
        Predicate::Transitive(crate::syntax::TransitivePredicate {
            head,
            pre_object_elements,
            object,
            elements,
        })
    } else if head.auxiliaries.is_empty()
        && modal.is_none()
        && head.preverb_modifiers.is_empty()
        && pre_object_elements.is_empty()
        && elements.is_empty()
        && proform
    {
        Predicate::Proform(ProPredicate {
            auxiliary: AuxiliaryInstance {
                auxiliary: Auxiliary::Do,
                inflection: proform_inflection(head.verb.slot),
                contracted_negation: false,
            },
        })
    } else if head.auxiliaries.len() == 1
        && head.preverb_modifiers.is_empty()
        && pre_object_elements.is_empty()
        && elements.is_empty()
        && proform
    {
        Predicate::Proform(ProPredicate {
            auxiliary: head.auxiliaries[0],
        })
    } else {
        pre_object_elements.append(&mut elements);
        Predicate::Intransitive(crate::syntax::IntransitivePredicate {
            head,
            elements: pre_object_elements,
        })
    };
    Some(FinishedPredicate {
        modal,
        predicate,
        elided,
    })
}

const fn proform_inflection(slot: VerbSlot) -> AuxiliaryInflection {
    match slot {
        VerbSlot::Infinitive | VerbSlot::Imperative => AuxiliaryInflection::Base,
        VerbSlot::Present { person, number } => AuxiliaryInflection::Present { person, number },
        VerbSlot::Past { person, number } => AuxiliaryInflection::Past { person, number },
        VerbSlot::PresentParticiple => AuxiliaryInflection::PresentParticiple,
        VerbSlot::PastParticiple => AuxiliaryInflection::PastParticiple,
    }
}

pub(super) fn finish_infinitive(
    clause: InfinitiveClause,
) -> Option<crate::syntax::InfinitiveClause> {
    let FinishedPredicate {
        modal, predicate, ..
    } = finish_predicate(*clause.predicate)?;
    if modal.is_some() {
        return None;
    }
    Some(crate::syntax::InfinitiveClause {
        negated: clause.negated,
        marker: clause.marker,
        predicate: Box::new(predicate),
    })
}

fn attach_object(slot: &mut Option<PredicateObject>, object: PredicateObject) -> Option<()> {
    if let Some(PredicateObject::Ability(ability)) = slot
        && ability.argument.is_none()
    {
        ability.argument = Some(Box::new(object));
        return Some(());
    }
    if slot.replace(object).is_some() {
        return None;
    }
    Some(())
}

const fn is_modal(auxiliary: Auxiliary) -> bool {
    matches!(
        auxiliary,
        Auxiliary::Can
            | Auxiliary::Could
            | Auxiliary::May
            | Auxiliary::Might
            | Auxiliary::Must
            | Auxiliary::Shall
            | Auxiliary::Should
            | Auxiliary::Will
            | Auxiliary::Would
    )
}

fn lowered_nominal_adjunct_kind(
    predicate: &VerbPhrase,
    phrase: &NounPhrase,
) -> Option<BareNominalAdjunct> {
    let adjunct = nominal_adjunct_kind(phrase)?;
    if !predicate.frame.licenses_bare_nominal_adjunct(adjunct) {
        return None;
    }
    let (form, passive) = lowered_predicate_form(predicate)?;
    let has_direct_object = predicate.dependents.iter().any(|dependent| {
        matches!(
            dependent,
            VerbDependent::DirectObject(_)
                | VerbDependent::PredicateComplement(
                    Phrase::CatalogAtom(_) | Phrase::EmbeddedAbility(_) | Phrase::QuotedAbility(_)
                )
                | VerbDependent::Scalar(_)
                | VerbDependent::Statistic(_)
                | VerbDependent::CoordinatedObject(_)
        )
    });
    let has_tail = predicate.dependents.iter().any(|dependent| {
        !matches!(
            dependent,
            VerbDependent::DirectObject(_)
                | VerbDependent::IndirectObject(_)
                | VerbDependent::PredicateComplement(
                    Phrase::CatalogAtom(_) | Phrase::EmbeddedAbility(_) | Phrase::QuotedAbility(_)
                )
                | VerbDependent::Scalar(_)
                | VerbDependent::Statistic(_)
                | VerbDependent::CoordinatedObject(_)
        )
    });
    (form == PredicateForm::PastParticiple
        || passive
        || has_direct_object
        || has_tail
        || !predicate.frame.direct_object().accepts())
    .then_some(adjunct)
}

fn lowered_predicate_form(predicate: &VerbPhrase) -> Option<(PredicateForm, bool)> {
    let mut form = predicate_form(predicate.verb.slot);
    let mut passive = false;
    for auxiliary in predicate.auxiliaries.iter().rev() {
        passive |= auxiliary.auxiliary == Auxiliary::Be && form == PredicateForm::PastParticiple;
        form = auxiliary_form(*auxiliary, form)?;
    }
    Some((form, passive))
}

fn nominal_adjunct_kind(phrase: &NounPhrase) -> Option<BareNominalAdjunct> {
    let NounPhrase::Nominal(nominal) = phrase else {
        return None;
    };
    match &nominal.head {
        NounInstance::Singular(noun) | NounInstance::Plural(noun) | NounInstance::Mass(noun) => {
            noun.bare_nominal_adjunct()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CatalogKind;
    use crate::catalog::Catalogs;
    use crate::identity::SelfReference;
    use crate::syntax::Ability;
    use crate::syntax::AbilityKind;
    use crate::syntax::AdjectiveComplement;
    use crate::syntax::AdjectivePhrase;
    use crate::syntax::Demonstrative;
    use crate::syntax::Determiner;
    use crate::syntax::FrequencyBound;
    use crate::syntax::FrequencyCount;
    use crate::syntax::NominalComplement;
    use crate::syntax::NominalModifier;
    use crate::syntax::NounPhrase;
    use crate::syntax::OracleText;
    use crate::syntax::Paragraph;
    use crate::syntax::PredicateConjunction;
    use crate::syntax::PredicateObject;
    use crate::syntax::RelativeBody;
    use crate::syntax::RelativeMarker;
    use crate::syntax::Sentence;
    use crate::syntax::SentenceBody;
    use crate::syntax::Subject;
    use crate::syntax::Subordinator;
    use crate::word::Adjective;
    use crate::word::Auxiliary;
    use crate::word::AuxiliaryInflection;
    use crate::word::Noun;
    use crate::word::NounInstance;
    use crate::word::Number;
    use crate::word::Person;
    use crate::word::Tense;
    use crate::word::Verb;
    use crate::word::VerbSlot;
    use crate::word::Vocab;

    const FIXTURES: [&str; 29] = [
        "Draw a card.",
        "Prevent that damage.",
        "If damage would be dealt to this creature, prevent that damage.",
        "The next time this creature would deal damage this turn, prevent that damage.",
        "Spells cost {1} less to cast.",
        "This creature costs {1} less to cast.",
        "Creatures you control attack each combat if able.",
        "Prevented damage is dealt to that creature's controller instead.",
        "Target creature gets +1/+1 until end of turn.",
        "Other Goblin creatures you control get +1/+1 and have haste.",
        "Creatures you control gain flying, then draw a card.",
        "If you control a Plains, creatures you control get +1/+1.",
        "Creatures you control get +1/+1 as long as you control a Goblin.",
        "You may exert this creature as it attacks.",
        "As this creature enters, choose a creature type.",
        "This creature attacks while saddled.",
        "Target creature you control fights target creature you don't control.",
        "You gain 2 life.",
        "This creature deals 3 damage to any target.",
        "Activate only as a sorcery.",
        "Activate only once each turn.",
        "This ability triggers only once each turn.",
        "This creature can't attack during extra turns.",
        "You may play that card this turn.",
        "You gain 1 life for each spell you've cast.",
        "It's put into exile.",
        "Counter target spell that's one or more colors.",
        "You may discard a Plains card rather than pay this spell's mana cost.",
        "Gain control of target creature for as long as you control this artifact.",
    ];

    #[test]
    fn clause_fixtures_parse_structurally_and_render_without_source() {
        for source in FIXTURES {
            let parsed = parse(source);
            assert_eq!(
                render_sentence(parsed.sentence().expect("sentence root")),
                source
            );
        }
    }

    #[test]
    fn singular_demonstratives_determine_mass_nouns() {
        // Positive: `that`/`this` now determine a mass noun (`that damage`), the
        // demonstrative object of a prevention/replacement imperative.
        let parsed = parse("Prevent that damage.");
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected an imperative transitive clause");
        };
        let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
            panic!("expected a nominal object, got {:?}", predicate.object);
        };
        assert_eq!(
            nominal.determiner,
            Some(Determiner::Demonstrative(Demonstrative::That))
        );
        assert!(matches!(
            nominal.head,
            NounInstance::Mass(Noun::Word(Vocab::Damage))
        ));

        // Negative (mirror direction): the plural demonstratives are still
        // barred from a mass noun, and the singular ones from a plural count
        // noun, so widening `this`/`that` to mass did not erase cardinality
        // agreement. An exact sentence parse must fail for both.
        for rejected in ["Prevent those damage.", "Prevent that cards."] {
            assert!(
                parse_nonterminal(rejected, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
                "{rejected:?} must not parse as a complete sentence"
            );
        }
    }

    #[test]
    fn the_next_time_frame_fronts_a_subordinate_clause() {
        let parsed =
            parse("The next time this creature would deal damage this turn, prevent that damage.");
        let SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
            matrix,
            attachments,
        })) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause with a fronted frame");
        };
        assert!(matches!(
            matrix.as_ref(),
            IndependentClause::Imperative(Predicate::Transitive(_))
        ));
        let [attachment] = attachments.as_slice() else {
            panic!("expected exactly one fronted attachment");
        };
        assert_eq!(attachment.position, AttachmentPosition::BeforeMatrix);
        assert!(attachment.comma);
        let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
            Subordinator::TheNextTime,
            SubordinateBody::Finite(condition),
        )) = &attachment.kind
        else {
            panic!(
                "expected a `the next time` subordinate frame, got {:?}",
                attachment.kind
            );
        };
        assert!(
            matches!(condition.as_ref(), IndependentClause::Deontic(..)),
            "the frame's event clause is a `would` modal clause"
        );
    }

    #[test]
    fn number_of_times_takes_a_finite_event_clause() {
        use crate::syntax::NominalComplement;
        use crate::syntax::NounPhrase;
        // `the number of times <clause>`: the clause-taking variant of `the
        // number of <nominal>`, reusing the finite-clause machinery. Covers an
        // active perfect, a passive, and a transitive-with-adjunct body.
        for source in [
            "Draw cards equal to the number of times this spell was kicked.",
            "Draw cards equal to the number of times you chose a mode for that spell.",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
        }

        // The clause is carried structurally as the `times` head's complement.
        let value = parse_nonterminal(
            "the number of times you drew a card",
            &fixture_catalogs(),
            Nonterminal::NounPhrase,
        )
        .expect("number-of-times value must parse");
        let Some(NounPhrase::Nominal(number)) = value.noun_phrase() else {
            panic!("expected a nominal");
        };
        let [NominalComplement::Prepositional(of)] = number.complements.as_slice() else {
            panic!("expected an `of` complement: {number:#?}");
        };
        let crate::syntax::Phrase::NounPhrase(object) = of.object.as_ref() else {
            panic!("`of` object should be a noun phrase");
        };
        let NounPhrase::Nominal(times) = object.as_ref() else {
            panic!("`of` object should be a `times` nominal");
        };
        assert!(matches!(
            times.complements.as_slice(),
            [NominalComplement::EventClause(_)]
        ));
    }

    #[test]
    fn arithmetic_value_expressions_round_trip_in_where_definitions() {
        // The subtraction and halving value expressions in a `where X is
        // <value>` definition, in both operand orders and with the `rounded
        // up`/`rounded down` rider on `half`. Additive `plus` and `twice`
        // already parse (coordination / copular adverb) and are not reshaped.
        for source in [
            "Target creature gets +X/+0 until end of turn, where X is 3 minus the number of lands you control.",
            "Target creature gets +X/+0 until end of turn, where X is the number of lands you control minus 4.",
            "Target creature gets +X/+0 until end of turn, where X is half the number of lands you control.",
            "Target creature gets +X/+0 until end of turn, where X is half the number of lands you control, rounded up.",
            "Target creature gets +X/+0 until end of turn, where X is half the number of lands you control, rounded down.",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn half_value_carries_its_rounding_rider_structurally() {
        use crate::syntax::ArithmeticValue;
        use crate::syntax::NounPhrase;
        use crate::syntax::Rounding;
        let source = "Target creature gets +X/+0 until end of turn, where X is half your life total, rounded up.";
        let parsed = parse_self(source);
        // The rounded half value round-trips inside its where-definition host.
        let rendered = render_sentence_as(parsed.sentence().unwrap(), "Nissa Revane", true);
        assert_eq!(rendered, source);
        // Direct structural check via a standalone value parse.
        let value = parse_nonterminal(
            "half the number of Forests you control, rounded down",
            &fixture_catalogs(),
            Nonterminal::NounPhrase,
        )
        .expect("half value must parse");
        assert!(matches!(
            value.noun_phrase(),
            Some(NounPhrase::Arithmetic(ArithmeticValue::Half {
                rounding: Some(Rounding::Down),
                ..
            }))
        ));
    }

    #[test]
    fn devotion_value_nominal_unblocks_its_hosting_clauses() {
        // The `devotion to <color>` value nominal [CR#700.5] appears across the
        // recovery contexts it was blocking: the Theros god `as long as`
        // condition (single color and two-color pair), the `equal to <value>`
        // comparison, and a `where X is <value>` definition.
        for source in [
            "As long as your devotion to green is less than five, Nissa isn't a creature.",
            "As long as your devotion to white and black is less than seven, Nissa isn't a creature.",
            "Each opponent loses life equal to your devotion to black.",
            "Nissa gets +0/+X until end of turn, where X is your devotion to green.",
        ] {
            let parsed = parse_self(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence_as(parsed.sentence().unwrap(), "Nissa Revane", true),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn trailing_where_clause_binds_a_variable_definition() {
        // A trailing `, where X is <value>` clause attaches to the independent
        // clause it follows as a `where` subordinate whose finite body is a
        // copular equation binding the count variable `X` to a value-denoting
        // nominal. Unlike the adverbial subordinators it does not gate the
        // matrix; the matrix carries the `X` the clause defines.
        let source = "Target creature gets +X/+0 until end of turn, where X is the number of creatures you control.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
            matrix,
            attachments,
        })) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause with a trailing where-definition");
        };
        assert!(matches!(matrix.as_ref(), IndependentClause::Transitive(..)));
        let [attachment] = attachments.as_slice() else {
            panic!("expected exactly one trailing attachment");
        };
        assert_eq!(attachment.position, AttachmentPosition::AfterMatrix);
        assert!(attachment.comma);
        let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
            Subordinator::Where,
            SubordinateBody::Finite(body),
        )) = &attachment.kind
        else {
            panic!(
                "expected a `where` subordinate definition, got {:?}",
                attachment.kind
            );
        };
        let IndependentClause::Copular(Subject(subject), _) = body.as_ref() else {
            panic!("the where-body is a copular equation, got {body:?}");
        };
        // The bound variable reuses the count-context `X` quantity rather than a
        // fresh variable kind.
        assert!(matches!(subject, NounPhrase::Quantity(Quantity::X)));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn where_clause_attaches_to_choose_up_to_x() {
        // The `choose up to X …, where X is …` host (Bumi, Riku, Discordant
        // Dirge), a previously unattached residue, takes the same trailing
        // `where` definition as an ordinary effect clause.
        let source =
            "Choose up to X target creatures, where X is the number of creatures you control.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
            attachments,
            ..
        })) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause with a trailing where-definition");
        };
        assert!(attachments.iter().any(|attachment| matches!(
            &attachment.kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(Subordinator::Where, _))
        )));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn as_though_binds_a_trailing_past_indicative_clause() {
        // Mechanism A: `as though` is a thin lexeme/spelling addition onto the
        // existing subordinator-agnostic trailing-clause attachment; no new
        // production. Past-indicative bodies (`had`, `didn't have`) need no
        // subjunctive licensing.
        for source in [
            "You may cast this spell as though it had flash.",
            "You may cast spells as though they had flash.",
            "This creature can attack this turn as though it didn't have defender.",
        ] {
            let parsed = parse_self(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn as_though_clause_attachment_shape() {
        let source = "You may cast this spell as though it had flash.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
            attachments,
            ..
        })) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause with a trailing as-though attachment");
        };
        let [attachment] = attachments.as_slice() else {
            panic!("expected exactly one trailing attachment");
        };
        assert_eq!(attachment.position, AttachmentPosition::AfterMatrix);
        assert!(!attachment.comma);
        assert!(matches!(
            &attachment.kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                Subordinator::AsThough,
                SubordinateBody::Finite(_),
            ))
        ));
    }

    #[test]
    fn as_though_does_not_double_or_leak_into_bare_as() {
        // No doubling / no `though`-less over-fire on a malformed doubled
        // subordinator.
        assert!(
            parse_nonterminal(
                "This creature can attack as though though it didn't have defender.",
                &fixture_catalogs(),
                Nonterminal::Sentence,
            )
            .is_err()
        );
    }

    #[test]
    fn as_though_licenses_a_past_subjunctive_body() {
        // Mechanism B: `were`/`weren't` under `as though` license the
        // otherwise-ungrammatical past-subjunctive body (`it were`, `it
        // weren't`), gated to this subordinator only.
        // NOTE: the mana purpose-tail sub-family (`it were mana of any
        // color`) is NOT covered here — it needs a bare-NP complement after
        // a `PastSubjunctive` `Be`, which routes through the *copular*
        // pathway (`Features::Copula`/`copula_agreement`). That pathway is
        // now licensed too (`CopulaAgreement::PastSubjunctive`, `round
        // copsubj`): the copula carries no agreement constraint but marks the
        // clause `subjunctive`, so every consumer other than the `as though`
        // gate (`clause.rs:2056`) still rejects it. See the `PA*`/`NA*` tests
        // below for that pathway's coverage.
        let source =
            "You may have this creature assign its combat damage as though it weren't blocked.";
        let parsed = parse_self(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        assert_eq!(
            render_sentence(parsed.sentence().unwrap()),
            source,
            "{source}"
        );
    }

    #[test]
    fn as_though_were_reading_is_dispreferenced_to_indicative_plural() {
        // The extra `PastSubjunctive` lexical reading for `were` must not
        // clobber the ordinary indicative plural reading where one is
        // available (`they were untapped`); only a singular subject with no
        // indicative reading (`it were`) forces the subjunctive reading.
        let source = "This creature can attack as though they were untapped.";
        let parsed = parse_self(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn bare_subjunctive_clause_does_not_parse() {
        // N1: a bare subjunctive main clause is not a sentence — the
        // recognition-level gate at `RuleTag::Sentence` requires
        // `subjunctive: false`.
        assert!(
            parse_nonterminal(
                "This creature were blocked.",
                &fixture_catalogs(),
                Nonterminal::Sentence,
            )
            .is_err()
        );
    }

    #[test]
    fn subjunctive_under_a_different_subordinator_does_not_parse() {
        // N2: the licensing gate in `conditional_reduction` checks the
        // specific subordinator lexeme, not just "some subordinator".
        assert!(
            parse_nonterminal(
                "As long as it were blocked, this creature gets +1/+1.",
                &fixture_catalogs(),
                Nonterminal::Sentence,
            )
            .is_err()
        );
    }

    #[test]
    fn subjunctive_under_until_does_not_parse() {
        // N4: same gate, `until` rather than `as long as`.
        assert!(
            parse_nonterminal(
                "Target creature gets +1/+1 until it were blocked.",
                &fixture_catalogs(),
                Nonterminal::Sentence,
            )
            .is_err()
        );
    }

    // --- round `copsubj`: subjunctive copular (`Features::Copula`) ---

    #[test]
    fn as_though_licenses_a_subjunctive_copula_with_no_tail() {
        // PA1 (Chromatic Orrery): the minimal no-tail mana-copular row.
        let source = "You may spend mana as though it were mana of any color.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        assert_eq!(
            render_sentence(parsed.sentence().unwrap()),
            source,
            "{source}"
        );
    }

    #[test]
    fn as_though_licenses_a_subjunctive_copula_with_a_plural_matrix_subject() {
        // PA2 (Mycosynth Lattice).
        let source = "Players may spend mana as though it were mana of any color.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        assert_eq!(
            render_sentence(parsed.sentence().unwrap()),
            source,
            "{source}"
        );
    }

    #[test]
    fn as_though_licenses_a_subjunctive_copula_under_a_fronted_frame() {
        // PA3 (False Dawn): a fronted `until end of turn` frame over a
        // subjunctive-bodied attachment.
        let source =
            "Until end of turn, you may spend white mana as though it were mana of any color.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        assert_eq!(
            render_sentence(parsed.sentence().unwrap()),
            source,
            "{source}"
        );
    }

    #[test]
    fn as_though_mana_copula_purpose_infinitive_attaches_inside_the_complement_nominal() {
        // PA4: pins the §2.5 attachment ruling empirically (confirmed by
        // probing `Agatha's Soul Cauldron`, whose purpose tail has the same
        // shape). The purpose infinitive cannot attach at
        // `RuleTag::VerbPhraseInfinitive` (the subordinate clause linearly
        // separates `spend mana` from the tail); the reachable analysis is
        // `RuleTag::NominalInfinitive`, and it lands as a complement of the
        // *inner* `any color` nominal (not the outer `mana of any color`
        // nominal, nor a matrix-verb complement).
        use crate::syntax::NominalComplement;
        use crate::syntax::NounPhrase;
        let source = "You may spend mana as though it were mana of any color to cast that spell.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        assert_eq!(
            render_sentence(parsed.sentence().unwrap()),
            source,
            "{source}"
        );

        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause: {:#?}", parsed.sentence());
        };
        let [attachment] = complex.attachments.as_slice() else {
            panic!("expected exactly one trailing attachment: {complex:#?}");
        };
        let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
            crate::syntax::Subordinator::AsThough,
            SubordinateBody::Finite(body),
        )) = &attachment.kind
        else {
            panic!("expected an as-though finite attachment: {attachment:#?}");
        };
        let IndependentClause::Copular(_, predicate) = body.as_ref() else {
            panic!("expected a copular as-though body: {body:#?}");
        };
        let crate::syntax::CopularComplement::NounPhrase(complement) = &predicate.complement else {
            panic!("expected a noun-phrase complement: {predicate:#?}");
        };
        let NounPhrase::Nominal(mana) = complement else {
            panic!("expected a nominal complement: {complement:#?}");
        };
        let [NominalComplement::Prepositional(of_any_color)] = mana.complements.as_slice() else {
            panic!("expected `mana` to take one prepositional complement: {mana:#?}");
        };
        let crate::syntax::Phrase::NounPhrase(any_color_np) = of_any_color.object.as_ref() else {
            panic!("expected a noun-phrase object of `of`: {of_any_color:#?}");
        };
        let NounPhrase::Nominal(any_color) = any_color_np.as_ref() else {
            panic!("expected a nominal object of `of`: {any_color_np:#?}");
        };
        assert!(
            matches!(
                any_color.complements.as_slice(),
                [NominalComplement::Infinitive(
                    crate::syntax::InfinitiveClause {
                        marker: InfinitiveMarker::To,
                        ..
                    }
                )]
            ),
            "expected the purpose infinitive on the inner `any color` nominal: {any_color:#?}"
        );
    }

    #[test]
    fn as_though_mana_copula_structural_shape() {
        // PA5: structural pin of the whole subjunctive-copula pathway.
        use crate::syntax::CopularComplement;
        use crate::word::Auxiliary;
        use crate::word::AuxiliaryInflection;
        let source = "You may spend mana as though it were mana of any color.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause: {:#?}", parsed.sentence());
        };
        let [attachment] = complex.attachments.as_slice() else {
            panic!("expected exactly one trailing attachment: {complex:#?}");
        };
        let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
            crate::syntax::Subordinator::AsThough,
            SubordinateBody::Finite(body),
        )) = &attachment.kind
        else {
            panic!("expected an as-though finite attachment: {attachment:#?}");
        };
        let IndependentClause::Copular(_, predicate) = body.as_ref() else {
            panic!("expected a copular as-though body: {body:#?}");
        };
        assert_eq!(predicate.copula.auxiliary.auxiliary, Auxiliary::Be);
        assert_eq!(
            predicate.copula.auxiliary.inflection,
            AuxiliaryInflection::PastSubjunctive
        );
        assert!(!predicate.copula.contracted_with_subject);
        assert!(matches!(
            predicate.complement,
            CopularComplement::NounPhrase(_)
        ));
    }

    #[test]
    fn as_though_indicative_copula_reading_survives_the_subjunctive_copula_filter() {
        // NA1 (anti-shadow): `as though they were untapped` and `as though
        // those cards were in your graveyard` must keep the ordinary
        // indicative reading of `were`, proving the `reading_dispreference`
        // survives the copula filter (E2) rather than the subjunctive
        // reading shadowing it.
        use crate::word::Auxiliary;
        use crate::word::AuxiliaryInflection;
        use crate::word::Number;
        use crate::word::Person;
        for source in [
            "This creature can attack as though they were untapped.",
            "You may cast spells from your hand as though those cards were in your graveyard.",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let SentenceBody::Independent(clause) = &parsed.sentence().expect("root").body else {
                panic!("expected an independent clause: {source}");
            };
            let attachments: &[ClauseAttachment] = match clause {
                IndependentClause::Complex(complex) => complex.attachments.as_slice(),
                other => panic!("expected a complex clause: {other:#?}"),
            };
            let mut found_copula = false;
            for attachment in attachments {
                let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                    crate::syntax::Subordinator::AsThough,
                    SubordinateBody::Finite(body),
                )) = &attachment.kind
                else {
                    continue;
                };
                // `were untapped` / `were in your graveyard` reduce as a
                // passive or prepositional-copular body depending on the
                // complement shape, not necessarily `IndependentClause::Copular`
                // — either way the `Be` auxiliary must keep its ordinary
                // indicative inflection, never `PastSubjunctive`.
                let auxiliary = match body.as_ref() {
                    IndependentClause::Copular(_, predicate) => predicate.copula.auxiliary,
                    IndependentClause::Passive(_, predicate) => *predicate
                        .head
                        .auxiliaries
                        .first()
                        .expect("passive `be` auxiliary"),
                    other => panic!("expected a copular or passive as-though body: {other:#?}"),
                };
                {
                    found_copula = true;
                    assert_eq!(auxiliary.auxiliary, Auxiliary::Be);
                    assert_eq!(
                        auxiliary.inflection,
                        AuxiliaryInflection::Past {
                            person: Person::Third,
                            number: Number::Plural,
                        },
                        "must not be PastSubjunctive: {auxiliary:#?}"
                    );
                }
            }
            assert!(found_copula, "expected an as-though copular body: {source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn bare_subjunctive_copular_clause_does_not_parse() {
        // NA2: extends N1 (bare_subjunctive_clause_does_not_parse) to the
        // copular pathway.
        assert!(
            parse_nonterminal(
                "This creature were red.",
                &fixture_catalogs(),
                Nonterminal::Sentence,
            )
            .is_err()
        );
    }

    #[test]
    fn subjunctive_copula_under_a_different_subordinator_does_not_parse() {
        // NA3: extends N2/N4 to the copular pathway.
        for source in [
            "As long as it were mana of any color, you gain 1 life.",
            "Target creature gets +1/+1 until it were red.",
        ] {
            assert!(
                parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
                "{source}"
            );
        }
    }

    #[test]
    fn indicative_copula_agreement_strictness_is_unchanged() {
        // NA4: the subjunctive branch must not have widened the indicative
        // equality test in `reduce_copular_clause`.
        for source in ["Those creatures is red.", "That creature are red."] {
            assert!(
                parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
                "{source}"
            );
        }
    }

    // --- round `degcmp`: degree-measured comparative (`its power were 2 greater`)
    // ---

    #[test]
    fn as_though_licenses_a_degree_measured_comparative() {
        // PB1 (Hotshot Mechanic).
        let source = "This creature crews Vehicles as though its power were 2 greater.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        assert_eq!(
            render_sentence(parsed.sentence().unwrap()),
            source,
            "{source}"
        );
    }

    #[test]
    fn as_though_degree_measure_under_a_coordinated_matrix() {
        // PB2 (Cloudspire Captain).
        let source =
            "This creature saddles Mounts and crews Vehicles as though its power were 2 greater.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        assert_eq!(
            render_sentence(parsed.sentence().unwrap()),
            source,
            "{source}"
        );
    }

    #[test]
    fn as_though_degree_measure_in_an_embedded_rules_face() {
        // PB3 (the 13-row embedded-rules shape, e.g. Back on Track).
        let source =
            "This token saddles Mounts and crews Vehicles as though its power were 2 greater.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        assert_eq!(
            render_sentence(parsed.sentence().unwrap()),
            source,
            "{source}"
        );
    }

    #[test]
    fn degree_measured_comparative_structural_shape() {
        // PB4: tree pin.
        use crate::Numeral;
        use crate::syntax::CopularComplement;
        use crate::syntax::NumberLiteral;
        use crate::word::Auxiliary;
        use crate::word::AuxiliaryInflection;
        use crate::word::Vocab;
        let source = "This creature crews Vehicles as though its power were 2 greater.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause: {:#?}", parsed.sentence());
        };
        let [attachment] = complex.attachments.as_slice() else {
            panic!("expected exactly one trailing attachment: {complex:#?}");
        };
        let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
            crate::syntax::Subordinator::AsThough,
            SubordinateBody::Finite(body),
        )) = &attachment.kind
        else {
            panic!("expected an as-though finite attachment: {attachment:#?}");
        };
        let IndependentClause::Copular(_, predicate) = body.as_ref() else {
            panic!("expected a copular as-though body: {body:#?}");
        };
        assert_eq!(predicate.copula.auxiliary.auxiliary, Auxiliary::Be);
        assert_eq!(
            predicate.copula.auxiliary.inflection,
            AuxiliaryInflection::PastSubjunctive
        );
        let CopularComplement::Adjective(adjective) = &predicate.complement else {
            panic!("expected an adjective complement: {predicate:#?}");
        };
        assert_eq!(
            adjective.degree,
            Some(NumberLiteral {
                value: 2,
                numeral: Numeral::Arabic(false),
            })
        );
        assert_eq!(adjective.head, crate::word::Adjective::Word(Vocab::Greater));
        assert!(adjective.complements.is_empty());
    }

    #[test]
    fn numeral_before_a_than_only_adjective_is_not_a_degree_phrase() {
        // NB1: the round-`copsubj` regression, tree-verified (Eleshnorn, the
        // Gargantuan Sacrifice three other creatures host).
        let source = "Sacrifice three other creatures.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        let rendered = render_sentence(parsed.sentence().unwrap());
        assert_eq!(rendered, source, "{source}");
        assert!(rendered.contains("three"), "{rendered}");
        assert!(!rendered.contains(" 3 "), "{rendered}");

        let SentenceBody::Independent(IndependentClause::Imperative(predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected an imperative body: {:#?}", parsed.sentence());
        };
        let object = match predicate {
            crate::syntax::Predicate::Transitive(predicate) => &predicate.object,
            other => panic!("expected a transitive predicate: {other:#?}"),
        };
        let crate::syntax::PredicateObject::NounPhrase(crate::syntax::NounPhrase::Nominal(nominal)) =
            object
        else {
            panic!("expected a nominal object: {object:#?}");
        };
        let [crate::syntax::NominalModifier::Adjective { phrase, .. }] =
            nominal.modifiers.as_slice()
        else {
            panic!("expected a single adjective modifier: {nominal:#?}");
        };
        assert!(
            phrase.degree.is_none(),
            "the `other` modifier must not carry a degree measure: {phrase:#?}"
        );
    }

    #[test]
    fn numeral_before_an_or_comparative_stays_attributive() {
        // NB2: the shapes an `OrComparative`-only guard would still swallow.
        // These are already resolved at baseline (attributive quantity +
        // adjective modifier, an existing pathway unrelated to
        // `AdjectiveComparisonState`); this round must not divert them
        // through the new degree-measure production. `degree_leaks` walks
        // the debug tree text as a coarse but exhaustive fidelity check: a
        // `degree: Some` anywhere would mean the new rule mis-fired.
        for source in [
            "Repeat this process two more times.",
            "You can cast only one more spell this turn.",
            "If life was paid, this planeswalker enters with two fewer loyalty counters.",
        ] {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence)
                .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
            let tree = format!("{:#?}", parsed.sentence());
            assert!(
                !tree.contains("degree: Some"),
                "{source} must not carry a degree measure: {tree}"
            );
        }
    }

    #[test]
    fn elenda_at_least_n_greater_stays_unresolved() {
        // NB3: corpus-level pin — out of scope (§8), must stay unresolved.
        let source = "Elenda gets an additional +5/+5 as long as your life total is at least 10 greater than your starting life total.";
        let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence);
        assert!(parsed.is_err(), "{source} must remain unresolved");
    }

    #[test]
    fn where_body_prefers_possessive_self_reference_over_a_vocab_noun() {
        // Regression for `Fang, Roku's Companion`: the where-body `X is Fang's
        // power` must read `Fang's` as the face's own possessive self-reference,
        // not as the regular-vocabulary noun `fang` (which renders lowercase and
        // corrupts the printed name). The possessive scan site pays the same
        // nickname-collision dispreference as the other lowercasing sites, so the
        // case-preserving self-reference wins and the face round-trips with a
        // capital `Fang's`.
        let self_reference = SelfReference::new("Fang, Roku's Companion", true);
        let source = "Target creature gets +X/+0 until end of turn, where X is Fang's power.";
        let parsed = parse_nonterminal_with_self_reference(
            source,
            &fixture_catalogs(),
            Nonterminal::Sentence,
            &self_reference,
        )
        .expect("Fang's where-body must parse");
        assert_eq!(
            render_sentence_as(parsed.sentence().unwrap(), "Fang, Roku's Companion", true),
            source,
            "the possessive must re-emit as the capitalized self-reference, not lowercase `fang's`"
        );
    }

    #[test]
    fn finite_verbs_agree_with_their_subjects() {
        let plural_parse = parse("Spells cost {1} less to cast.");
        let (plural_subject, plural) = finite(plural_parse.sentence().unwrap());
        assert!(matches!(
            plural_subject,
            Subject(NounPhrase::Nominal(nominal))
                if matches!(nominal.head, NounInstance::Plural(Noun::Word(Vocab::Spell)))
        ));
        assert_eq!(
            plural.verb.slot,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Plural,
            }
        );
        assert!(matches!(plural.verb.verb, Verb::Word(Vocab::Cost)));

        let singular_parse = parse("This creature costs {1} less to cast.");
        let (_, singular) = finite(singular_parse.sentence().unwrap());
        assert_eq!(
            singular.verb.slot,
            VerbSlot::Present {
                person: Person::Third,
                number: Number::Singular,
            }
        );
    }

    #[test]
    fn exact_quantities_can_measure_mass_nouns() {
        for (source, expected) in [
            ("You gain 2 life.", Vocab::Life),
            ("This creature deals 3 damage to any target.", Vocab::Damage),
        ] {
            let parsed = parse(source);
            let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
                &parsed.sentence().expect("sentence root").body
            else {
                panic!("expected a transitive clause for {source:?}");
            };
            assert!(matches!(
                &predicate.object,
                PredicateObject::NounPhrase(NounPhrase::Nominal(nominal))
                    if matches!(
                        nominal.determiner,
                        Some(Determiner::Quantity(crate::syntax::Quantity::Exact(_)))
                    ) && matches!(
                        nominal.head,
                        NounInstance::Mass(Noun::Word(ref word)) if *word == expected
                    )
            ));
        }
    }

    #[test]
    fn as_fills_the_preposition_slot_after_an_adverb() {
        let source = "Activate only as a sorcery.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Intransitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected an imperative intransitive clause");
        };
        assert!(
            matches!(
                predicate.elements.as_slice(),
                [
                    PredicateElement::Adjunct(PredicateAdjunct::Adverb(Vocab::Only)),
                    PredicateElement::Adjunct(PredicateAdjunct::Prepositional(preposition)),
                ] if preposition.preposition == crate::syntax::Preposition::As
            ),
            "{predicate:#?}"
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn temporal_noun_phrases_can_follow_predicate_tail_adverbs() {
        for source in [
            "Activate only once each turn.",
            "This ability triggers only once each turn.",
            "Do this only once each turn.",
        ] {
            let parsed = parse(source);
            assert_eq!(render_sentence(parsed.sentence().expect(source)), source);
        }
    }

    #[test]
    fn activation_restriction_clauses_parse_and_render_structurally() {
        for source in [
            "Activate only during your turn, before attackers are declared.",
            "Activate only as a sorcery.",
        ] {
            let parsed = parse(source);
            assert_eq!(render_sentence(parsed.sentence().expect(source)), source);
        }
        // §4.2: the base parse of the first sentence is `elements:
        // [Adverb(Only), Prepositional(During)]` plus an `AfterMatrix` comma
        // `Dependent(Subordinate(Before, …))` attachment — never a
        // `Restriction` run (the `and` is absorbed by the subordinate
        // clause's own adjective coordination, §4.1's hazard).
        let parsed = parse("Activate only during your turn, before attackers are declared.");
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a complex clause");
        };
        let IndependentClause::Imperative(Predicate::Intransitive(predicate)) =
            complex.matrix.as_ref()
        else {
            panic!("expected an imperative intransitive matrix");
        };
        assert!(
            matches!(
                predicate.elements.as_slice(),
                [
                    PredicateElement::Adjunct(PredicateAdjunct::Adverb(Vocab::Only)),
                    PredicateElement::Adjunct(PredicateAdjunct::Prepositional(_)),
                ]
            ),
            "{:#?}",
            predicate.elements
        );
        assert_eq!(complex.attachments.len(), 1);
        assert!(
            matches!(
                &complex.attachments[0].kind,
                ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                    Subordinator::Before,
                    _
                ))
            ),
            "{:#?}",
            complex.attachments[0]
        );
    }

    #[test]
    fn coordinated_only_restrictions_form_one_restriction_attachment() {
        let source = "Activate only as a sorcery and only once each turn.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a complex clause");
        };
        let IndependentClause::Imperative(Predicate::Intransitive(predicate)) =
            complex.matrix.as_ref()
        else {
            panic!("expected an imperative intransitive matrix");
        };
        assert!(
            predicate.elements.is_empty(),
            "matrix elements must be empty: {:#?}",
            predicate.elements
        );
        assert_eq!(complex.attachments.len(), 1);
        let attachment = &complex.attachments[0];
        assert!(!attachment.comma);
        let ClauseAttachmentKind::Restriction(run) = &attachment.kind else {
            panic!("expected a Restriction attachment: {attachment:#?}");
        };
        assert!(matches!(
            run.first.as_slice(),
            [PredicateAdjunct::Prepositional(_)]
        ));
        assert_eq!(run.rest.len(), 1);
        assert_eq!(run.rest[0].conjunction, Some(PredicateConjunction::And));
        assert!(!run.rest[0].comma);
        assert!(matches!(
            run.rest[0].adjuncts.as_slice(),
            [PredicateAdjunct::Adverb(_), PredicateAdjunct::Temporal(_),]
        ));
    }

    #[test]
    fn restriction_run_admits_an_if_clause_member() {
        // The flagship shape (`Prepositional` first member coordinated with
        // an `only if` member — what Revision 1's design could not
        // represent), substituted per §6 test 2's Stage-B-aborted
        // contingency: Stage B (the `declare attackers/blockers step`
        // nominal) did not land this round (regression in
        // `comparison_inside_preposition_stays_with_its_object` — see the
        // mechanic report), so the 14-dup flagship
        // (`Cast this spell only during the declare attackers step and only
        // if you've been attacked this step.`) stays in residue and this
        // test uses an equivalent Stage-A-only witness instead.
        let source =
            "Cast this spell only during your turn and only if you've been attacked this step.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a complex clause");
        };
        assert_eq!(complex.attachments.len(), 1);
        let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].kind else {
            panic!(
                "expected a Restriction attachment: {:#?}",
                complex.attachments[0]
            );
        };
        assert!(matches!(
            run.first.as_slice(),
            [PredicateAdjunct::Prepositional(_)]
        ));
        assert_eq!(run.rest.len(), 1);
        assert_eq!(run.rest[0].conjunction, Some(PredicateConjunction::And));
        assert!(!run.rest[0].comma);
        assert!(matches!(
            run.rest[0].adjuncts.as_slice(),
            [PredicateAdjunct::Dependent(dependent)]
                if matches!(dependent.as_ref(), DependentClause::Subordinate(Subordinator::If, _))
        ));
    }

    #[test]
    fn oxford_restriction_run_carries_member_boundaries() {
        // Grizzled Wolverine's three-way Oxford run, substituted per §6 test
        // 3's Stage-B-aborted contingency (see note on the previous test):
        // the row stays in residue this round.
        let source =
            "Activate only during your turn, only if you control a Swamp, and only once each turn.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a complex clause");
        };
        let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].kind else {
            panic!(
                "expected a Restriction attachment: {:#?}",
                complex.attachments[0]
            );
        };
        assert_eq!(run.rest.len(), 2);
        assert_eq!(run.rest[0].conjunction, None);
        assert!(run.rest[0].comma);
        assert_eq!(run.rest[1].conjunction, Some(PredicateConjunction::And));
        assert!(run.rest[1].comma);
    }

    #[test]
    fn restriction_runs_host_on_every_matrix() {
        for source in [
            "Activate only as a sorcery and only once each turn.",
            "Activate only during your turn and only if an opponent lost life this turn.",
            "Activate only once and only if you control a snow Mountain.",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
            let SentenceBody::Independent(IndependentClause::Complex(complex)) =
                &parsed.sentence().unwrap().body
            else {
                panic!("expected a complex clause for {source}");
            };
            assert!(
                matches!(
                    complex.attachments.last().unwrap().kind,
                    ClauseAttachmentKind::Restriction(_)
                ),
                "{source}"
            );
        }
    }

    #[test]
    fn single_only_restriction_keeps_its_flat_elements() {
        for source in [
            "Activate only as a sorcery.",
            "Activate only once each turn.",
        ] {
            let parsed = parse(source);
            let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Intransitive(
                predicate,
            ))) = &parsed.sentence().unwrap().body
            else {
                panic!("expected an imperative intransitive for {source}");
            };
            assert!(
                predicate.elements.iter().all(|element| !matches!(
                    element,
                    PredicateElement::Adjunct(PredicateAdjunct::Prepositional(_))
                        | PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))
                ) || matches!(
                    element,
                    PredicateElement::Adjunct(_)
                )),
                "{source}: {:#?}",
                predicate.elements
            );
            assert!(
                matches!(
                    predicate.elements.first(),
                    Some(PredicateElement::Adjunct(PredicateAdjunct::Adverb(
                        Vocab::Only
                    )))
                ),
                "{source}: {:#?}",
                predicate.elements
            );
        }
    }

    #[test]
    fn single_only_if_restriction_keeps_its_dependent_attachment() {
        let source = "Activate only if you control a Swamp.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a complex clause");
        };
        let IndependentClause::Imperative(Predicate::Intransitive(predicate)) =
            complex.matrix.as_ref()
        else {
            panic!("expected an imperative intransitive matrix");
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Adverb(
                Vocab::Only
            ))]
        ));
        assert_eq!(complex.attachments.len(), 1);
        assert!(!complex.attachments[0].comma);
        assert!(matches!(
            &complex.attachments[0].kind,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(Subordinator::If, _))
        ));
    }

    #[test]
    fn juxtaposed_only_restrictions_stay_uncoordinated() {
        let source = "Activate only as a sorcery only once each turn.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Intransitive(
            predicate,
        ))) = &parsed.sentence().unwrap().body
        else {
            panic!("expected an imperative intransitive, no attachment");
        };
        // `only as a sorcery only once each turn` stays five flat elements
        // (`Only`, `Prepositional(As …)`, `Only`, `Adverb(once)`,
        // `Temporal(each turn)`) — no coordinator, so no `Restriction` run.
        assert_eq!(predicate.elements.len(), 5, "{:#?}", predicate.elements);
    }

    #[test]
    fn restriction_run_rejects_unlicensed_members_and_conjunctions() {
        for source in [
            "Activate only as a sorcery and once each turn.",
            "Activate and only once each turn.",
            "Draw a card and only once each turn.",
            "Activate only as a sorcery or only once each turn.",
            "Activate only as a sorcery, and only once each turn.",
        ] {
            assert!(
                parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
                "{source} must not parse"
            );
        }
    }

    #[test]
    fn clause_level_trailing_restriction_conjunct_is_not_licensed() {
        assert!(
            parse_nonterminal(
                "Only your opponents may activate this ability and only as a sorcery.",
                &fixture_catalogs(),
                Nonterminal::Sentence,
            )
            .is_err()
        );
        let source = "Only your opponents may activate this ability.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn restriction_runs_are_unambiguous() {
        for source in [
            "Activate only as a sorcery and only once each turn.",
            "Activate only during your turn, only if you control a Swamp, and only once each turn.",
        ] {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence)
                .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            assert!(
                parsed.root_tied_alternatives().len() <= 1,
                "{source} has a real forest tie: {:?}",
                parsed.root_tied_alternatives()
            );
        }
    }

    #[test]
    fn turn_structure_family_parses_and_renders_structurally() {
        // Every admitted turn-structure sub-shape, source-free round-trip.
        for source in [
            // Sub-shape 3: `skip <step>` imperatives.
            "Skip your draw step.",
            "Skip your upkeep.",
            "Skip your next combat phase.",
            // Sub-shape 2: extra/additional + turn-structure nominal, with the
            // `after this one` / `after this phase` tail and the fronted mirror.
            "Take an extra turn after this one.",
            "Target player takes an extra turn after this one.",
            "There is an additional combat phase after this phase.",
            "After this phase, there is an additional combat phase.",
            "You may play an additional land on each of your turns.",
            "You take the initiative.",
            // Sub-shape 4: `during` PPs over steps, whose-step causal pair.
            "Activate only during your upkeep.",
            "Activate only during an opponent's upkeep.",
            // Plain-draw sanity mirror: the `draw` noun sense must not disturb
            // the imperative `draw` verb.
            "Draw a card.",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(render_sentence(parsed.sentence().expect(source)), source);
        }
    }

    #[test]
    fn extra_and_additional_keep_their_distinct_printed_adjectives() {
        // Causal pair: the modifier word is carried structurally, so `extra`
        // (an imperative `take an extra turn`) and `additional` (an existential
        // `there is an additional combat phase`) never collapse to one spelling.
        let extra = parse("Take an extra turn after this one.");
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &extra.sentence().expect("extra turn").body
        else {
            panic!("expected an imperative transitive: {:?}", extra.sentence());
        };
        let PredicateObject::NounPhrase(NounPhrase::Nominal(turn)) = &predicate.object else {
            panic!("expected an `extra turn` object: {predicate:#?}");
        };
        assert_eq!(turn_head_spelling(turn), "turn");
        assert_eq!(sole_adjective_spelling(turn), "extra");

        let additional = parse("There is an additional combat phase.");
        let SentenceBody::Independent(IndependentClause::Existential(existential)) =
            &additional.sentence().expect("additional phase").body
        else {
            panic!("expected an existential: {:?}", additional.sentence());
        };
        let NounPhrase::Nominal(phase) = &existential.pivot else {
            panic!("expected an `additional combat phase` pivot: {existential:#?}");
        };
        assert_eq!(turn_head_spelling(phase), "phase");
        assert_eq!(sole_adjective_spelling(phase), "additional");
    }

    #[test]
    fn after_preposition_attaches_both_trailing_and_fronted() {
        // Causal pair on the `after` preposition: a trailing temporal adjunct on
        // an imperative, and a fronted one on an existential.
        let trailing = parse("Take an extra turn after this one.");
        let debug = format!("{:?}", trailing.sentence().expect("trailing").body);
        assert!(
            debug.contains("Prepositional") && debug.contains("After"),
            "trailing `after this one` should be a prepositional adjunct: {debug}"
        );

        let fronted = parse("After this phase, there is an additional combat phase.");
        let debug = format!("{:?}", fronted.sentence().expect("fronted").body);
        assert!(
            debug.contains("After"),
            "fronted `After this phase` should carry the After preposition: {debug}"
        );
    }

    #[test]
    fn draw_noun_sense_does_not_capture_the_imperative_draw_verb() {
        // Negative armor for the `draw` noun addition: `Draw a card.` stays an
        // imperative headed by the `draw` verb, never a bare `draw` nominal.
        let parsed = parse("Draw a card.");
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("draw a card").body
        else {
            panic!("expected an imperative transitive: {:?}", parsed.sentence());
        };
        assert!(matches!(
            predicate.head.verb,
            VerbInstance {
                verb: Verb::Word(Vocab::Draw),
                slot: VerbSlot::Imperative,
            }
        ));
    }

    #[test]
    fn bounded_frequency_phrases_are_predicate_adjuncts() {
        for (source, expected_bound, expected_count) in [
            (
                "You may choose the same mode more than once.",
                FrequencyBound::MoreThan,
                FrequencyCount::Once,
            ),
            (
                "Activate no more than twice each turn.",
                FrequencyBound::NoMoreThan,
                FrequencyCount::Twice,
            ),
            (
                "Activate no more than three times each turn.",
                FrequencyBound::NoMoreThan,
                FrequencyCount::Times(crate::syntax::NumberLiteral {
                    value: 3,
                    numeral: crate::Numeral::Cardinal,
                }),
            ),
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let elements = match &parsed.sentence().unwrap().body {
                SentenceBody::Independent(IndependentClause::Deontic(
                    _,
                    _,
                    Some(Predicate::Transitive(predicate)),
                )) => &predicate.elements,
                SentenceBody::Independent(IndependentClause::Imperative(
                    Predicate::Intransitive(predicate),
                )) => &predicate.elements,
                clause => panic!("expected a bounded-frequency predicate: {clause:#?}"),
            };
            assert!(
                elements.iter().any(|element| matches!(
                    element,
                    PredicateElement::Adjunct(PredicateAdjunct::Frequency(frequency))
                        if frequency.bound == expected_bound && frequency.count == expected_count
                )),
                "{source}: {elements:#?}"
            );
            assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        }
    }

    #[test]
    fn subject_relative_differential_comparisons_are_structural() {
        for (source, expected_adjective) in [
            (
                "Choose target opponent who has at least two more cards in hand than you do.",
                "more",
            ),
            (
                "Choose target opponent who has at least two fewer creature cards in their graveyard than you do.",
                "fewer",
            ),
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
                choose,
            ))) = &parsed.sentence().expect("sentence root").body
            else {
                panic!("expected an imperative choice: {:#?}", parsed.sentence());
            };
            let PredicateObject::NounPhrase(NounPhrase::Nominal(opponent)) = &choose.object else {
                panic!("expected an opponent object: {:#?}", choose.object);
            };
            let [NominalComplement::Relative(relative)] = opponent.complements.as_slice() else {
                panic!("expected one relative clause: {opponent:#?}");
            };
            assert_eq!(relative.marker, RelativeMarker::Who);
            let RelativeBody::SubjectGap(Predicate::Transitive(have)) = &relative.body else {
                panic!("expected a subject-gap transitive relative: {relative:#?}");
            };
            let PredicateObject::NounPhrase(NounPhrase::Nominal(cards)) = &have.object else {
                panic!("expected a compared card count: {:#?}", have.object);
            };
            let adjective = cards
                .modifiers
                .iter()
                .find_map(|modifier| match modifier {
                    NominalModifier::Adjective {
                        phrase: adjective, ..
                    } if matches!(
                        adjective.head,
                        Adjective::Word(word) if word.spelling() == expected_adjective
                    ) =>
                    {
                        Some(adjective)
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("missing {expected_adjective:?} modifier: {cards:#?}"));
            let [AdjectiveComplement::PostnominalComparison(comparison)] =
                adjective.complements.as_slice()
            else {
                panic!("expected a comparison complement: {adjective:#?}");
            };
            assert!(matches!(
                comparison.standard.as_ref(),
                crate::syntax::Phrase::Clause(clause)
                    if matches!(
                        clause.as_ref(),
                        Clause::Independent(IndependentClause::Proform(_, _))
                    )
            ));
            assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        }
    }

    #[test]
    fn plural_temporal_heads_do_not_become_late_objects() {
        let source = "This creature can't attack during extra turns.";
        let parsed = parse(source);
        assert!(
            matches!(
                &parsed.sentence().expect("sentence root").body,
                SentenceBody::Independent(IndependentClause::Deontic(
                    _,
                    _,
                    Some(Predicate::Intransitive(predicate)),
                )) if matches!(
                    predicate.elements.as_slice(),
                    [PredicateElement::Adjunct(PredicateAdjunct::Prepositional(preposition))]
                        if preposition.preposition == crate::syntax::Preposition::During
                )
            ),
            "{:#?}",
            parsed.sentence()
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn temporal_noun_phrases_can_follow_direct_objects() {
        let source = "You may play that card this turn.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Some(Predicate::Transitive(predicate)),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a deontic transitive clause");
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Temporal(
                NounPhrase::Nominal(turn),
            ))] if matches!(turn.head, NounInstance::Singular(Noun::Word(Vocab::Turn)))
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn selected_preposition_precedes_a_temporal_adjunct() {
        let source = "You may look at the top card of your library any time.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Some(Predicate::Intransitive(predicate)),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a deontic intransitive clause: {:#?}",
                parsed.sentence()
            );
        };
        assert!(
            matches!(
                predicate.elements.as_slice(),
                [
                    PredicateElement::Complement(PredicateComplement::Prepositional(_)),
                    PredicateElement::Adjunct(PredicateAdjunct::Temporal(NounPhrase::Nominal(time))),
                ] if matches!(time.head, NounInstance::Singular(Noun::Word(Vocab::Time)))
            ),
            "{predicate:#?}"
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn intransitive_frame_rejects_a_direct_object() {
        let result = parse_nonterminal("Look a card.", &fixture_catalogs(), Nonterminal::Sentence);

        assert!(result.is_err());
    }

    #[test]
    fn ditransitive_frame_builds_an_indirect_object_complement() {
        let source = "Ask a player a number.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        assert!(matches!(
            predicate.object,
            PredicateObject::NounPhrase(NounPhrase::Nominal(ref number))
                if matches!(number.head, NounInstance::Singular(Noun::Word(Vocab::Number)))
        ));
        assert!(matches!(
            predicate.pre_object_elements.as_slice(),
            [PredicateElement::Complement(PredicateComplement::IndirectObject(
                NounPhrase::Nominal(player),
            ))] if matches!(player.head, NounInstance::Singular(Noun::Word(Vocab::Player)))
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn as_clauses_keep_their_surface_attachment_position() {
        for (source, position) in [
            (
                "You may exert this creature as it attacks.",
                AttachmentPosition::AfterMatrix,
            ),
            (
                "As this creature enters, choose a creature type.",
                AttachmentPosition::BeforeMatrix,
            ),
        ] {
            let parsed = parse(source);
            assert!(matches!(
                &parsed.sentence().expect("sentence root").body,
                SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                    attachments,
                    ..
                })) if matches!(
                    attachments.as_slice(),
                    [ClauseAttachment {
                        position: actual,
                        kind: ClauseAttachmentKind::Dependent(
                            DependentClause::Subordinate(
                                crate::syntax::Subordinator::As,
                                SubordinateBody::Finite(_),
                            ),
                        ),
                        ..
                    }] if *actual == position
                )
            ));
        }
    }

    #[test]
    fn fronted_cost_phrase_is_an_adjunct_with_an_infinitive_complement() {
        let source = "As an additional cost to cast this spell, sacrifice a creature.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause: {:#?}", parsed.sentence());
        };
        let [
            ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                comma: true,
                kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(preposition)),
            },
        ] = complex.attachments.as_slice()
        else {
            panic!("expected one fronted prepositional adjunct: {complex:#?}");
        };
        assert_eq!(preposition.preposition, crate::syntax::Preposition::As);
        let crate::syntax::Phrase::NounPhrase(noun_phrase) = preposition.object.as_ref() else {
            panic!("expected the adjunct to modify a cost noun phrase: {preposition:#?}");
        };
        let NounPhrase::Nominal(nominal) = noun_phrase.as_ref() else {
            panic!("expected a nominal cost phrase: {noun_phrase:#?}");
        };
        assert!(matches!(
            nominal.head,
            NounInstance::Singular(Noun::Word(Vocab::Cost))
        ));
        assert!(matches!(
            nominal.complements.as_slice(),
            [NominalComplement::Infinitive(
                crate::syntax::InfinitiveClause {
                    marker: InfinitiveMarker::To,
                    ..
                }
            )]
        ));
        assert!(matches!(
            complex.matrix.as_ref(),
            IndependentClause::Imperative(_)
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn this_way_is_a_manner_adjunct_inside_a_condition() {
        let source = "If you search your library this way, shuffle.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause: {:#?}", parsed.sentence());
        };
        let [
            ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                kind:
                    ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                        Subordinator::If,
                        SubordinateBody::Finite(condition),
                    )),
                ..
            },
        ] = complex.attachments.as_slice()
        else {
            panic!("expected a fronted if-clause: {complex:#?}");
        };
        let IndependentClause::Transitive(_, condition) = condition.as_ref() else {
            panic!("expected a transitive search condition: {condition:#?}");
        };
        assert!(matches!(
            condition.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Manner(
                NounPhrase::Nominal(way),
            ))] if matches!(way.head, NounInstance::Singular(Noun::Word(Vocab::Way)))
        ));
        assert!(matches!(
            complex.matrix.as_ref(),
            IndependentClause::Imperative(_)
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn then_can_modify_a_following_independent_clause() {
        let source = "Then that player shuffles.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a clause with a fronted adjunct: {:#?}",
                parsed.sentence()
            );
        };
        assert!(matches!(
            complex.attachments.as_slice(),
            [ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                comma: false,
                kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Then)),
            }]
        ));
        assert!(matches!(
            complex.matrix.as_ref(),
            IndependentClause::Intransitive(_, _)
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn otherwise_fronts_a_following_imperative_clause() {
        let source = "Otherwise, put it into your hand.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a clause with a fronted adjunct: {:#?}",
                parsed.sentence()
            );
        };
        assert!(matches!(
            complex.attachments.as_slice(),
            [ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                comma: true,
                kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Otherwise)),
            }]
        ));
        assert!(matches!(
            complex.matrix.as_ref(),
            IndependentClause::Imperative(_)
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn otherwise_fronts_a_finite_clause() {
        let source = "Otherwise, you draw a card.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a clause with a fronted adjunct: {:#?}",
                parsed.sentence()
            );
        };
        assert!(matches!(
            complex.attachments.as_slice(),
            [ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                comma: true,
                kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Otherwise)),
            }]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn otherwise_fronts_a_modal_clause() {
        let source = "Otherwise, you may put it into your graveyard.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a clause with a fronted adjunct: {:#?}",
                parsed.sentence()
            );
        };
        assert!(matches!(
            complex.attachments.as_slice(),
            [ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                comma: true,
                kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Otherwise)),
            }]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn otherwise_fronting_renders_its_comma() {
        let source = "Otherwise, put it into your hand.";
        let parsed = parse(source);
        let rendered = render_sentence(parsed.sentence().unwrap());
        assert!(
            rendered.contains("Otherwise, "),
            "expected rendered sentence to contain the fronting comma: {rendered:?}"
        );
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a clause with a fronted adjunct");
        };
        let [ClauseAttachment { comma, .. }] = complex.attachments.as_slice() else {
            panic!(
                "expected exactly one attachment: {:#?}",
                complex.attachments
            );
        };
        assert!(*comma, "expected comma: true on the otherwise attachment");
    }

    #[test]
    fn ordinary_adverbs_do_not_front_a_clause_with_a_comma() {
        // Load-bearing gate: proves the new production is keyed on the
        // `sentence_adverbial` metadata flag, not on the `Adverb` slot at
        // large — `Immediately` is an ordinary adverb and must not front a
        // clause across a comma.
        let source = "Immediately, draw a card.";
        let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence);
        if let Ok(parsed) = parsed {
            assert_ne!(
                parsed.opacity_mode(),
                OpacityMode::Exact,
                "{source} must not parse cleanly via a BeforeMatrix adverb attachment"
            );
        }
    }

    #[test]
    fn otherwise_does_not_fill_ordinary_adverb_slots() {
        // Proves the deliberate omission of `.adverb()` on `Otherwise` (§2):
        // it must not fill the ordinary `Adverb` slot, so neither a trailing
        // nor a comma-less fronted placement parses cleanly.
        for source in ["Draw a card otherwise.", "Otherwise draw a card."] {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence);
            if let Ok(parsed) = parsed {
                assert_ne!(
                    parsed.opacity_mode(),
                    OpacityMode::Exact,
                    "{source} must not parse cleanly"
                );
            }
        }
    }

    #[test]
    fn otherwise_with_an_unparsable_body_leaves_the_whole_sentence_unresolved() {
        // Anti-span-split gate: the matrix pattern requires
        // `Clause::Independent`; a body that fails to reduce yields `None`
        // for the whole `[adverb, comma, clause]` span, not a partial parse.
        let source = "Otherwise, it has base power and toughness 1/1 and can't block Detectives.";
        let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence);
        assert!(parsed.is_err(), "{source} must remain unresolved");
    }

    #[test]
    fn then_fronting_is_unchanged() {
        let source = "Then that player shuffles.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a clause with a fronted adjunct");
        };
        assert!(matches!(
            complex.attachments.as_slice(),
            [ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                comma: false,
                kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Then)),
            }]
        ));
    }

    #[test]
    fn for_as_long_as_is_one_finite_subordinator() {
        let parsed =
            parse("Gain control of target creature for as long as you control this artifact.");
        assert!(matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                attachments,
                ..
            })) if matches!(
                attachments.as_slice(),
                [ClauseAttachment {
                    position: AttachmentPosition::AfterMatrix,
                    kind: ClauseAttachmentKind::Dependent(
                        DependentClause::Subordinate(
                            crate::syntax::Subordinator::ForAsLongAs,
                            SubordinateBody::Finite(_),
                        ),
                    ),
                    ..
                }]
            )
        ));
    }

    #[test]
    fn while_can_introduce_an_elliptical_postposed_clause() {
        let parsed = parse("This creature attacks while saddled.");
        assert!(matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                attachments,
                ..
            })) if matches!(
                attachments.as_slice(),
                [ClauseAttachment {
                    position: AttachmentPosition::AfterMatrix,
                    kind: ClauseAttachmentKind::Dependent(
                        DependentClause::Subordinate(
                            crate::syntax::Subordinator::While,
                            SubordinateBody::Elliptical(EllipticalClause::Adjective(_)),
                        ),
                    ),
                    ..
                }]
            )
        ));
    }

    #[test]
    fn unless_introduces_a_finite_postposed_clause() {
        let parsed = parse("This land enters tapped unless you control a basic land.");
        assert!(matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                attachments,
                ..
            })) if matches!(
                attachments.as_slice(),
                [ClauseAttachment {
                    position: AttachmentPosition::AfterMatrix,
                    kind: ClauseAttachmentKind::Dependent(
                        DependentClause::Subordinate(
                            crate::syntax::Subordinator::Unless,
                            SubordinateBody::Finite(_),
                        ),
                    ),
                    ..
                }]
            )
        ));
    }

    #[test]
    fn subjectless_imperatives_are_not_finite_subordinate_bodies() {
        let parsed = parse("Target creature gets +1/+1 until end of turn.");
        assert!(!parsed.chart.forest.nodes().any(|node| {
            node.key.symbol == crate::forest::ForestSymbol::Nonterminal(Nonterminal::Clause)
                && (node.key.start, node.key.end) == (5, 8)
                && matches!(
                    node.key.constituent_features(),
                    Some(Features::Clause { finite: true, .. })
                )
        }));
    }

    #[test]
    fn participle_position_distinguishes_modifier_from_passive_predicate() {
        let parsed = parse("Prevented damage is dealt to that creature's controller instead.");
        let SentenceBody::Independent(IndependentClause::Passive(
            Subject(NounPhrase::Nominal(subject)),
            predicate,
        )) = &parsed.sentence().unwrap().body
        else {
            panic!("expected nominal subject");
        };
        assert!(matches!(
            subject.modifiers.as_slice(),
            [NominalModifier::Adjective { phrase: adjective, .. }]
                if matches!(
                    adjective.head,
                    Adjective::Participle(Tense::Past, Verb::Word(Vocab::Prevent))
                )
        ));
        assert!(matches!(
            predicate.head.auxiliaries.as_slice(),
            [auxiliary]
                if auxiliary.auxiliary == Auxiliary::Be
                    && auxiliary.inflection == (AuxiliaryInflection::Present {
                        person: Person::Third,
                        number: Number::Singular,
                    })
        ));
        assert!(matches!(predicate.head.verb.verb, Verb::Word(Vocab::Deal)));
        assert_eq!(predicate.head.verb.slot, VerbSlot::PastParticiple);
    }

    #[test]
    fn subject_copula_contractions_are_structural() {
        let catalogs = fixture_catalogs();
        for (source, contracted) in [("you are the monarch", false), ("you're the monarch", true)] {
            let parsed = parse_nonterminal(source, &catalogs, Nonterminal::Clause)
                .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            let Some(Clause::Independent(IndependentClause::Copular(subject, predicate))) =
                parsed.clause()
            else {
                panic!("expected a copular clause for {source:?}");
            };
            assert!(matches!(
                subject,
                Subject(NounPhrase::Pronoun {
                    pronoun: Pronoun::You,
                    case: PronounCase::Subject,
                })
            ));
            assert_eq!(predicate.copula.auxiliary.auxiliary, Auxiliary::Be);
            assert_eq!(
                predicate.copula.auxiliary.inflection,
                AuxiliaryInflection::Present {
                    person: Person::Second,
                    number: Number::Singular,
                }
            );
            assert_eq!(predicate.copula.contracted_with_subject, contracted);
            assert!(matches!(
                predicate.complement,
                crate::syntax::CopularComplement::NounPhrase(_)
            ));
        }
    }

    #[test]
    fn copular_adverbs_precede_the_complement() {
        for (source, contracted) in [("It is still a land.", false), ("It's still a land.", true)] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
            let SentenceBody::Independent(IndependentClause::Copular(subject, predicate)) =
                &parsed.sentence().expect("sentence root").body
            else {
                panic!("expected a copular clause: {:#?}", parsed.sentence());
            };
            assert!(matches!(
                subject,
                Subject(NounPhrase::Pronoun {
                    pronoun: Pronoun::It(crate::word::Gender::Neuter),
                    case: PronounCase::Subject,
                })
            ));
            assert_eq!(predicate.copula.contracted_with_subject, contracted);
            assert_eq!(predicate.precomplement_adverbs.len(), 1);
            assert_eq!(predicate.precomplement_adverbs[0].spelling(), "still");
            assert!(matches!(
                predicate.complement,
                crate::syntax::CopularComplement::NounPhrase(_)
            ));
            assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        }
    }

    #[test]
    fn contracted_subject_auxiliaries_are_structural() {
        let catalogs = fixture_catalogs();

        let parsed =
            parse_nonterminal("each spell you've cast", &catalogs, Nonterminal::NounPhrase)
                .expect("perfect relative clause should parse");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal with a relative clause");
        };
        let [NominalComplement::Relative(relative)] = nominal.complements.as_slice() else {
            panic!("expected a relative-clause complement: {nominal:#?}");
        };
        let RelativeBody::ObjectGap { subject, predicate } = &relative.body else {
            panic!("expected an object-gap relative clause");
        };
        assert!(matches!(
            subject,
            Subject(NounPhrase::Pronoun {
                pronoun: Pronoun::You,
                case: PronounCase::Subject,
            })
        ));
        assert!(predicate.head.first_auxiliary_contracted_with_subject);
        assert!(matches!(
            predicate.head.auxiliaries.as_slice(),
            [auxiliary] if auxiliary.auxiliary == Auxiliary::Have
        ));
        assert_eq!(predicate.head.verb.slot, VerbSlot::PastParticiple);

        let parsed = parse_nonterminal("it's put into exile", &catalogs, Nonterminal::Clause)
            .expect("contracted passive should parse");
        let Some(Clause::Independent(IndependentClause::Passive(subject, predicate))) =
            parsed.clause()
        else {
            panic!(
                "expected a contracted passive clause: {:#?}",
                parsed.clause()
            );
        };
        assert!(matches!(
            subject,
            Subject(NounPhrase::Pronoun {
                pronoun: Pronoun::It(crate::word::Gender::Neuter),
                case: PronounCase::Subject,
            })
        ));
        assert!(predicate.head.first_auxiliary_contracted_with_subject);
        assert!(matches!(
            predicate.head.auxiliaries.as_slice(),
            [auxiliary] if auxiliary.auxiliary == Auxiliary::Be
        ));

        let parsed = parse_nonterminal("that's one or more colors", &catalogs, Nonterminal::Clause)
            .expect("contracted demonstrative copula should parse");
        let Some(Clause::Independent(IndependentClause::Copular(subject, predicate))) =
            parsed.clause()
        else {
            panic!(
                "expected a contracted copular clause: {:#?}",
                parsed.clause()
            );
        };
        assert!(matches!(
            subject,
            Subject(NounPhrase::Demonstrative(Demonstrative::That))
        ));
        assert!(predicate.copula.contracted_with_subject);

        let parsed = parse_nonterminal(
            "a spell that's one or more colors",
            &catalogs,
            Nonterminal::NounPhrase,
        )
        .expect("contracted relative copula should parse");
        let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
            panic!("expected a nominal with a contracted relative clause");
        };
        assert!(
            matches!(
                nominal.complements.as_slice(),
                [NominalComplement::Relative(RelativeClause {
                    marker: RelativeMarker::That,
                    gap: RelativeGap::Subject,
                    body: RelativeBody::SubjectGap(Predicate::Copular(predicate)),
                })] if predicate.copula.contracted_with_subject
            ),
            "{nominal:#?}"
        );
    }

    #[test]
    fn rather_than_introduces_a_bare_infinitive_clause() {
        let parsed = parse("You may discard a Plains card rather than pay this spell's mana cost.");
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause");
        };
        assert!(matches!(
            complex.attachments.as_slice(),
            [ClauseAttachment {
                position: AttachmentPosition::AfterMatrix,
                comma: false,
                kind: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                    Subordinator::RatherThan,
                    SubordinateBody::Infinitive(crate::syntax::InfinitiveClause {
                        marker: InfinitiveMarker::Bare,
                        ..
                    }),
                ),),
            }]
        ));
    }

    #[test]
    fn rather_than_can_contrast_gerund_clauses() {
        let source = "You may cast that card by paying life equal to the spell's mana value rather than paying its mana cost.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Some(Predicate::Transitive(cast)),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a deontic cast clause: {:#?}", parsed.sentence());
        };
        let Some(PredicateElement::Adjunct(PredicateAdjunct::Prepositional(by))) =
            cast.elements.iter().find(|element| {
                matches!(
                    element,
                    PredicateElement::Adjunct(PredicateAdjunct::Prepositional(preposition))
                        if preposition.preposition == crate::syntax::Preposition::By
                )
            })
        else {
            panic!("expected a by-gerund adjunct: {cast:#?}");
        };
        let crate::syntax::Phrase::Clause(clause) = by.object.as_ref() else {
            panic!("by should take a clause: {by:#?}");
        };
        let Clause::Dependent(DependentClause::Gerund(gerund)) = clause.as_ref() else {
            panic!("by should take a gerund clause: {clause:#?}");
        };
        assert!(matches!(
            gerund.predicate.as_ref(),
            Predicate::Transitive(_)
        ));
        assert!(matches!(
            gerund.attachments.as_slice(),
            [DependentAttachment {
                position: AttachmentPosition::AfterMatrix,
                comma: false,
                clause: DependentClause::Subordinate(
                    Subordinator::RatherThan,
                    SubordinateBody::Gerund(alternative),
                ),
            }] if matches!(alternative.predicate.as_ref(), Predicate::Transitive(_))
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn not_to_negates_an_infinitive_clause() {
        let source = "You may choose not to untap this creature.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Some(Predicate::Intransitive(choose)),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a deontic choose clause: {:#?}", parsed.sentence());
        };
        assert!(matches!(
            choose.elements.as_slice(),
            [PredicateElement::Complement(
                PredicateComplement::Infinitive(crate::syntax::InfinitiveClause {
                    negated: true,
                    ..
                })
            )]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn directional_particle_completes_an_intransitive_predicate() {
        let source = "This creature phases out.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Intransitive(_, predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected an intransitive phase clause: {:#?}",
                parsed.sentence()
            );
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Particle(VerbParticle::Out)]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn directional_particle_requires_a_licensed_verb_pair() {
        let parsed = parse_nonterminal(
            "This creature transforms out.",
            &fixture_catalogs(),
            Nonterminal::Sentence,
        )
        .expect("opaque-noun fallback should remain available");

        assert_eq!(parsed.opacity_mode(), OpacityMode::OpaqueNouns);
        assert!(!matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Intransitive(_, predicate))
                if matches!(predicate.elements.as_slice(), [PredicateElement::Particle(_)])
        ));
    }

    #[test]
    fn face_down_is_a_secondary_adjective_predicate() {
        let source = "Turn this creature face down.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected an imperative turn clause: {:#?}",
                parsed.sentence()
            );
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Complement(PredicateComplement::Adjective(
                AdjectivePhrase {
                    degree: None,
                    head: Adjective::CardOrientation(crate::word::CardOrientation::FaceDown),
                    complements,
                }
            ))] if complements.is_empty()
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn plus_coordinates_additive_noun_phrases() {
        let source = "You gain that much life plus 1.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a transitive gain clause: {:#?}",
                parsed.sentence()
            );
        };
        let PredicateObject::NounPhrase(NounPhrase::Coordinated(object)) = &predicate.object else {
            panic!("expected an additive noun phrase: {:#?}", predicate.object);
        };
        assert!(matches!(
            object.rest.as_slice(),
            [crate::syntax::NounPhraseCoordination {
                conjunction: Some(crate::syntax::NounPhraseConjunction::Plus),
                ..
            }]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn variable_quantity_has_singular_standalone_agreement() {
        let source = "X is 5 or more.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Copular(
            Subject(NounPhrase::Quantity(Quantity::X)),
            _,
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a copular variable clause: {:#?}",
                parsed.sentence()
            );
        };
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn variable_value_constraint_parses_as_a_modal_copular_clause() {
        let source = "X can't be 0.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Deontic(
            Subject(NounPhrase::Quantity(Quantity::X)),
            _,
            Some(Predicate::Copular(predicate)),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!(
                "expected a deontic modal copular clause: {:#?}",
                parsed.sentence()
            );
        };
        assert_eq!(predicate.copula.auxiliary.auxiliary, Auxiliary::Be);
        assert_eq!(
            predicate.copula.auxiliary.inflection,
            AuxiliaryInflection::Base
        );
        assert!(predicate.adjuncts.is_empty());
        assert!(!predicate.distributive_each);
        assert!(matches!(
            predicate.complement,
            CopularComplement::NounPhrase(NounPhrase::Quantity(Quantity::Exact(n))) if n.value == 0
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn variable_value_constraint_composes_under_a_fronted_conditional() {
        let source = "If you cast this spell this way, X can't be 0.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Complex(ComplexClause { matrix, .. })) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause with a fronted condition");
        };
        assert!(matches!(
            matrix.as_ref(),
            IndependentClause::Deontic(
                Subject(NounPhrase::Quantity(Quantity::X)),
                _,
                Some(Predicate::Copular(_)),
            )
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn modal_participle_complement_stays_passive_under_a_variable_subject() {
        let source = "X can't be blocked.";
        let parsed = parse(source);
        let SentenceBody::Independent(body) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected an independent clause");
        };
        assert!(matches!(
            body,
            IndependentClause::Deontic(_, _, Some(Predicate::Passive(_)))
        ));
        assert!(!matches!(
            body,
            IndependentClause::Deontic(_, _, Some(Predicate::Copular(_)))
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn modal_copular_frame_rejects_a_finite_copula() {
        let source = "X can't is 0.";
        assert!(
            parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "{source:?} must not parse as a modal bare-copula clause"
        );
    }

    /// Clause coordination's right conjunct is `n(N::SimpleClause)`
    /// (`RuleTag::ClauseCoordination`), and every copular/modal-copular clause
    /// is an `N::Clause` production with no `SimpleClause` path, so
    /// `... and X can't be 0` cannot be a right conjunct for the same
    /// pre-existing reason `... and X is 5 or more` cannot. This is a
    /// deliberate scope boundary, not a bug — a later round adding
    /// `Clause`-right clause coordination should see this test fail and
    /// retire it.
    #[test]
    fn variable_value_constraint_does_not_coordinate_as_a_right_conjunct() {
        let source = "This ability can't be copied and X can't be 0.";
        assert!(
            parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "{source:?} is expected to stay unresolved (Clause-right clause coordination is a \
             separate round's work); if this now parses, retire this test"
        );
    }

    #[test]
    fn count_sense_of_power_accepts_an_adjective_and_plural_inflection() {
        let source = "You control three creatures with different powers.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);

        let source = "You gain the difference.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn passive_blocking_treats_this_turn_as_a_temporal_adjunct() {
        let source = "Creatures you control can't be blocked this turn.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Deontic(
            _,
            _,
            Some(Predicate::Passive(predicate)),
        )) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a deontic passive clause");
        };
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Temporal(
                NounPhrase::Nominal(turn),
            ))] if matches!(turn.head, NounInstance::Singular(Noun::Word(Vocab::Turn)))
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn direct_objects_cannot_follow_predicate_tail_elements() {
        let result = parse_nonterminal(
            "You draw during your turn a card.",
            &fixture_catalogs(),
            Nonterminal::Sentence,
        );

        assert!(result.is_err());
    }

    #[test]
    fn predicate_prefix_pruning_keeps_possible_nominal_adjuncts() {
        let predicate = |object, phase| Features::VerbPhrase {
            form: PredicateForm::Imperative,
            passive: false,
            object,
            indirect_object: false,
            selected_preposition: false,
            phase,
            frame: PredicateFrame::OPEN,
            bare: object == PredicateObjectState::None,
            subjunctive: false,
        };
        let open = predicate(PredicateObjectState::None, PredicateAttachmentPhase::Object);
        let occupied = predicate(
            PredicateObjectState::Direct,
            PredicateAttachmentPhase::Object,
        );
        let ability = predicate(
            PredicateObjectState::Ability,
            PredicateAttachmentPhase::Object,
        );
        let tail = predicate(PredicateObjectState::None, PredicateAttachmentPhase::Tail);
        let prepositional_tail = predicate(
            PredicateObjectState::None,
            PredicateAttachmentPhase::PrepositionalTail,
        );

        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseDirectObject,
            1,
            &open,
        ));
        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseDirectObject,
            1,
            &occupied,
        ));
        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseQuantity,
            1,
            &ability,
        ));
        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseDirectObject,
            1,
            &tail,
        ));
        assert!(accepts_predicate_prefix(
            RuleTag::VerbPhraseDirectObject,
            1,
            &prepositional_tail,
        ));
    }

    #[test]
    fn karmic_justice_trigger_event_is_transitive() {
        let catalogs = fixture_catalogs();
        for noun_phrase in [
            "a spell or ability an opponent controls",
            "a noncreature permanent you control",
        ] {
            parse_nonterminal(noun_phrase, &catalogs, Nonterminal::NounPhrase)
                .unwrap_or_else(|error| panic!("failed to parse {noun_phrase:?}: {error:?}"));
        }
        let source =
            "a spell or ability an opponent controls destroys a noncreature permanent you control";
        let parsed = parse_nonterminal(source, &catalogs, Nonterminal::SimpleClause)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
        assert!(matches!(
            finish_simple_clause(parsed.simple_clause().unwrap().clone()),
            Some(IndependentClause::Transitive(_, _))
        ));
    }

    #[test]
    fn target_and_relative_clauses_keep_their_nominal_roles() {
        let target_parse = parse("Target creature gets +1/+1 until end of turn.");
        let (Subject(NounPhrase::Nominal(target)), _) = finite(target_parse.sentence().unwrap())
        else {
            panic!("expected target nominal subject");
        };
        assert_eq!(target.determiner, Some(Determiner::Target(None)));

        let fight_parse =
            parse("Target creature you control fights target creature you don't control.");
        let SentenceBody::Independent(IndependentClause::Transitive(
            Subject(NounPhrase::Nominal(subject)),
            fight,
        )) = &fight_parse.sentence().unwrap().body
        else {
            panic!("expected controlled target subject");
        };
        assert!(matches!(
            subject.complements.as_slice(),
            [NominalComplement::Relative(relative)]
                if relative.gap == crate::syntax::RelativeGap::Object
        ));
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &fight.object else {
            panic!(
                "expected one direct-object nominal, got {:#?}",
                fight.object
            );
        };
        assert!(matches!(
            object.complements.as_slice(),
            [NominalComplement::Relative(relative)]
                if relative.gap == crate::syntax::RelativeGap::Object
        ));
    }

    #[test]
    fn goblin_chieftain_stat_change_remains_one_magic_atom() {
        let sentence = parse("Other Goblin creatures you control get +1/+1 and have haste.");
        let SentenceBody::Independent(IndependentClause::Coordinated(coordination)) =
            &sentence.sentence().unwrap().body
        else {
            panic!("expected coordinated predicates");
        };
        let IndependentClause::Transitive(_, first) = coordination.first.as_ref() else {
            panic!("expected simple first predicate");
        };
        assert!(matches!(first.object, PredicateObject::PowerToughness(_)));
    }

    #[test]
    fn contiguous_oracle_symbols_are_one_scalar_object() {
        let source = "Add {C}{C}.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        assert!(matches!(
            &predicate.object,
            PredicateObject::SymbolSequence(symbols)
                if symbols.iter().map(crate::syntax::OracleSymbol::as_str).collect::<String>()
                    == "{C}{C}"
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn oracle_symbol_alternatives_are_a_coordinated_object() {
        let source = "Add {R} or {G}.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        assert!(matches!(
            &predicate.object,
            PredicateObject::Coordinated(CoordinatedPredicateObject { first, rest })
                if matches!(first.as_ref(), PredicateObject::OracleSymbol(symbol) if symbol.as_str() == "{R}")
                    && matches!(rest.as_slice(), [PredicateObjectCoordination {
                        conjunction: Some(crate::syntax::PredicateConjunction::Or),
                        comma: false,
                        object: PredicateObject::OracleSymbol(symbol),
                    }] if symbol.as_str() == "{G}")
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn mana_amount_oxford_list_is_three_scalar_members() {
        let source = "Add {W}, {B}, or {G}.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        assert!(matches!(
            &predicate.object,
            PredicateObject::Coordinated(CoordinatedPredicateObject { first, rest })
                if matches!(first.as_ref(), PredicateObject::OracleSymbol(symbol) if symbol.as_str() == "{W}")
                    && matches!(rest.as_slice(), [
                        PredicateObjectCoordination {
                            conjunction: None,
                            comma: true,
                            object: PredicateObject::OracleSymbol(b),
                        },
                        PredicateObjectCoordination {
                            conjunction: Some(crate::syntax::PredicateConjunction::Or),
                            comma: true,
                            object: PredicateObject::OracleSymbol(g),
                        },
                    ] if b.as_str() == "{B}" && g.as_str() == "{G}")
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    #[allow(
        clippy::items_after_statements,
        reason = "the helper is local to this one assertion"
    )]
    fn filter_land_mana_list_members_are_symbol_groups() {
        let source = "Add {U}{U}, {U}{R}, or {R}{R}.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        let PredicateObject::Coordinated(CoordinatedPredicateObject { first, rest }) =
            &predicate.object
        else {
            panic!("expected a coordinated object: {:#?}", predicate.object);
        };
        fn joined(object: &PredicateObject) -> String {
            let PredicateObject::SymbolSequence(symbols) = object else {
                panic!("expected a symbol sequence: {object:#?}");
            };
            symbols
                .iter()
                .map(crate::syntax::OracleSymbol::as_str)
                .collect::<String>()
        }
        assert_eq!(joined(first), "{U}{U}");
        let [
            PredicateObjectCoordination { object: second, .. },
            PredicateObjectCoordination { object: third, .. },
        ] = rest.as_slice()
        else {
            panic!("expected exactly three members: {rest:#?}");
        };
        assert_eq!(joined(second), "{U}{R}");
        assert_eq!(joined(third), "{R}{R}");
        let PredicateObject::SymbolSequence(symbols) = first.as_ref() else {
            panic!("expected a symbol sequence: {first:#?}");
        };
        assert_eq!(symbols.len(), 2);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn mixed_symbol_group_alternative_is_one_member() {
        let source = "Add {U} or {C}{U}.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        let PredicateObject::Coordinated(CoordinatedPredicateObject { first, rest }) =
            &predicate.object
        else {
            panic!("expected a coordinated object: {:#?}", predicate.object);
        };
        assert!(
            matches!(first.as_ref(), PredicateObject::OracleSymbol(symbol) if symbol.as_str() == "{U}")
        );
        assert_eq!(rest.len(), 1, "expected exactly one member: {rest:#?}");
        let PredicateObject::SymbolSequence(symbols) = &rest[0].object else {
            panic!("expected a symbol sequence: {:#?}", rest[0].object);
        };
        assert_eq!(
            symbols
                .iter()
                .map(crate::syntax::OracleSymbol::as_str)
                .collect::<String>(),
            "{C}{U}"
        );
        assert_eq!(symbols.len(), 2);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn mana_amount_and_list_is_five_members() {
        let source = "Add {W}{W}, {U}{U}, {B}{B}, {R}{R}, and {G}{G}.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
            predicate,
        ))) = &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative: {:#?}", parsed.sentence());
        };
        let PredicateObject::Coordinated(CoordinatedPredicateObject { first: _, rest }) =
            &predicate.object
        else {
            panic!("expected a coordinated object: {:#?}", predicate.object);
        };
        assert_eq!(rest.len(), 4, "expected five members total: {rest:#?}");
        for interior in &rest[..3] {
            assert_eq!(interior.conjunction, None);
            assert!(interior.comma);
        }
        assert_eq!(
            rest[3].conjunction,
            Some(crate::syntax::PredicateConjunction::And)
        );
        assert!(rest[3].comma);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn mana_amount_list_rejects_noun_phrase_alternative() {
        let source = "Add {W} or one mana of the chosen color.";
        let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence);
        assert!(parsed.is_err(), "{source} must remain unresolved");
    }

    #[test]
    fn mana_amount_list_rejects_then_conjunction() {
        let source =
            "Add {B}, then add an additional {B} for each charge counter removed this way.";
        let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence);
        assert!(parsed.is_err(), "{source} must remain unresolved");
    }

    #[test]
    fn mana_amount_list_rejects_and_or() {
        let source = "Add {W} and/or {U}.";
        let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence);
        assert!(parsed.is_err(), "{source} must remain unresolved");
    }

    #[test]
    fn passive_temporal_adjunct_is_not_a_direct_object() {
        let source = "No spells were cast last turn.";
        let parsed = parse(source);
        assert!(
            matches!(
                &parsed.sentence().expect("sentence root").body,
                SentenceBody::Independent(IndependentClause::Passive(_, PassivePredicate {
                    elements,
                    ..
                })) if matches!(
                    elements.as_slice(),
                    [PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))]
                )
            ),
            "{:#?}",
            parsed.sentence()
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn recipient_passive_retains_the_theme_object() {
        let source = "Activate only if an opponent was dealt damage this turn.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a complex clause with a subordinate `if`");
        };
        let [
            ClauseAttachment {
                kind: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(_, body)),
                ..
            },
        ] = complex.attachments.as_slice()
        else {
            panic!("expected a single subordinate `if` attachment");
        };
        let SubordinateBody::Finite(clause) = body else {
            panic!("expected a finite subordinate clause");
        };
        let IndependentClause::Passive(_, predicate) = clause.as_ref() else {
            panic!("expected a passive clause under the subordinate `if`");
        };
        assert!(matches!(
            &predicate.retained_object,
            Some(PredicateObject::NounPhrase(NounPhrase::Nominal(damage)))
                if matches!(damage.head, NounInstance::Mass(Noun::Word(Vocab::Damage)))
        ));
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn recipient_passive_temporal_adjunct_is_not_a_retained_object() {
        // The §4.1 hazard: `this turn` must never be read as the retained
        // theme even though the recipient-passive frame requires a direct
        // object. It must land as a `Temporal` adjunct alongside the real
        // theme, never displace it.
        let source = "Skarrgan Firebird was dealt damage this turn.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Passive(_, predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a passive clause");
        };
        assert!(matches!(
            &predicate.retained_object,
            Some(PredicateObject::NounPhrase(NounPhrase::Nominal(damage)))
                if matches!(damage.head, NounInstance::Mass(Noun::Word(Vocab::Damage)))
        ));
        assert!(matches!(
            predicate.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))]
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn recipient_passive_frame_is_rejected_in_the_active_voice() {
        // Synthetic: no corpus witness. Pins §2.2's passive-only bail — the
        // recipient-passive frame must never license a double-object active.
        let result = parse_nonterminal(
            "You deal them 2 damage.",
            &fixture_catalogs(),
            Nonterminal::Sentence,
        );
        assert!(result.is_err(), "double-object active must stay unresolved");
    }

    #[test]
    fn ordinary_passive_has_no_retained_object() {
        let source = "Prevented damage is dealt to that creature's controller instead.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Passive(_, predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a passive clause");
        };
        assert!(predicate.retained_object.is_none());
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn subject_gap_relative_carries_a_recipient_passive() {
        let source = "Destroy target creature that was dealt damage this turn.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(matrix))) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative");
        };
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &matrix.object else {
            panic!("expected a nominal object");
        };
        assert!(matches!(
            object.complements.as_slice(),
            [NominalComplement::Relative(RelativeClause {
                marker: RelativeMarker::That,
                gap: RelativeGap::Subject,
                body: RelativeBody::SubjectGap(Predicate::Passive(passive)),
            })] if passive.retained_object.is_some()
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn present_tense_recipient_passive_parses() {
        let source = "If a player is dealt damage this way, scry 1.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn contracted_subject_recipient_passive_parses() {
        // Lich's shape: the `'re` auxiliary is contracted onto the subject
        // and reaches the verb phrase as a sibling, not a child — the
        // `fold_auxiliary_passive` helper must fold it in before
        // `predicate_arguments_complete` runs.
        let source = "You're dealt damage.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Passive(_, predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a passive clause");
        };
        assert!(predicate.head.first_auxiliary_contracted_with_subject);
        assert!(matches!(
            &predicate.retained_object,
            Some(PredicateObject::NounPhrase(NounPhrase::Nominal(damage)))
                if matches!(damage.head, NounInstance::Mass(Noun::Word(Vocab::Damage)))
        ));
    }

    #[test]
    fn contracted_subject_recipient_passive_round_trips() {
        // The contraction flag is the round's likeliest round-trip failure:
        // it must render back as `you're dealt damage`, never
        // `you are dealt damage`.
        let source = "You're dealt damage.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn contracted_subject_passive_without_an_object_is_unchanged() {
        // Flowering Lumberknot's shape: a contracted passive under an OPEN
        // frame with no object present. `is_satisfied_by(false)` holds
        // regardless of the folded `passive` flag, so this must keep parsing
        // exactly as it did before the fold.
        let source =
            "This creature can't attack or block unless it's paired with a creature with soulbond.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn contracted_subject_passive_rejects_a_non_recipient_direct_object() {
        // The tightening the fold introduces (§2.3): a contracted passive
        // that keeps a direct object under a non-recipient frame must be
        // rejected, not silently admitted with `passive: false`. Exercised
        // directly on the helper: a full-sentence synthetic like `it's
        // destroyed target creature` is contaminated by `'s` also
        // contracting `has` — `it has destroyed target creature` survives as
        // an unrelated, legitimate active-voice parse of the same string,
        // which would make the assertion pass for the wrong reason.
        let auxiliary = AuxiliaryInstance {
            auxiliary: Auxiliary::Be,
            inflection: AuxiliaryInflection::Present {
                person: Person::Third,
                number: Number::Singular,
            },
            contracted_negation: false,
        };
        let frame = Verb::Word(Vocab::Destroy).predicate_frames()[0];
        assert!(!frame.is_recipient_passive());
        let result = fold_auxiliary_passive(
            auxiliary,
            PredicateForm::PastParticiple,
            false,
            PredicateObjectState::Direct,
            false,
            frame,
        );
        assert_eq!(
            result, None,
            "a contracted passive retaining a non-recipient direct object must be rejected"
        );
    }

    #[test]
    fn contracted_subject_passive_rejects_an_explicit_indirect_object() {
        // The helper's `indirect_object` branch: no explicit indirect object
        // may survive under any passive, contracted or not.
        let result = parse_nonterminal(
            "You're dealt them damage.",
            &fixture_catalogs(),
            Nonterminal::Sentence,
        );
        assert!(
            result.is_err(),
            "a contracted passive keeping an explicit indirect object must not parse"
        );
    }

    #[test]
    fn ordinary_auxiliary_passive_is_unchanged() {
        // Pins Edit A: the ordinary `VerbPhraseAuxiliary` path (Fatal Blow's
        // recipient-passive `that was dealt damage this turn`, an ordinary
        // passive with the auxiliary inside the verb phrase) must be
        // byte-identical after the extraction into `fold_auxiliary_passive`.
        let source = "Destroy target creature that was dealt damage this turn.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let source = "Prevented damage is dealt to that creature's controller instead.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn modal_subject_relative_keeps_its_passive_temporal_adjunct() {
        let source = "Prevent all combat damage that would be dealt this turn.";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(matrix))) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a transitive imperative");
        };
        let PredicateObject::NounPhrase(NounPhrase::Nominal(object)) = &matrix.object else {
            panic!("expected a nominal object");
        };
        assert!(matrix.elements.is_empty(), "{matrix:#?}");
        assert!(matches!(
            object.complements.as_slice(),
            [NominalComplement::Relative(RelativeClause {
                marker: RelativeMarker::That,
                gap: RelativeGap::Subject,
                body: RelativeBody::ModalSubjectGap {
                    modal: Modal {
                        auxiliary: AuxiliaryInstance {
                            auxiliary: Auxiliary::Would,
                            ..
                        },
                    },
                    predicate: Some(Predicate::Passive(PassivePredicate { elements, .. })),
                },
            })] if matches!(
                elements.as_slice(),
                [PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))]
            )
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn multiple_fronted_clause_attachments_keep_surface_order() {
        let source = "At the beginning of each upkeep, if no spells were cast last turn, transform this creature.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert!(matches!(
            &parsed.sentence().expect("sentence root").body,
            SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
                attachments,
                ..
            })) if matches!(
                attachments.as_slice(),
                [
                    ClauseAttachment {
                        kind: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(_)),
                        ..
                    },
                    ClauseAttachment {
                        kind: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                            Subordinator::If,
                            SubordinateBody::Finite(_),
                        )),
                        ..
                    },
                ]
            )
        ));
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn if_you_dont_it_enters_tapped_parses() {
        let source = "If you don't, it enters tapped.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn enchanted_creature_cant_attack_or_block_parses() {
        let source = "Enchanted creature can't attack or block.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn this_creature_can_block_only_creatures_with_flying_parses() {
        let source = "This creature can block only creatures with flying.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn deontic_modal_ellipsis_drops_the_predicate_but_keeps_a_real_verb() {
        // Motivating shape: VP-ellipsis under the modal renders the bare modal
        // with no synthesized `do` ("If you can't, …").
        let elided = "If you can't, draw a card.";
        assert_eq!(render_sentence(parse(elided).sentence().unwrap()), elided);
        // Mirror shape: the same modal production with a real verb keeps it.
        let full = "Creatures you control can't attack.";
        assert_eq!(render_sentence(parse(full).sentence().unwrap()), full);
    }

    #[test]
    fn modal_subject_relative_keeps_the_passive_be_across_a_coordinated_agent() {
        // Motivating shape: the passive `be` must survive when the modal
        // relative carries a prepositional phrase and coordination — the loose
        // "empty intransitive" nulling used to eat it.
        let passive =
            "Prevent all combat damage that would be dealt to you and creatures you control.";
        assert_eq!(render_sentence(parse(passive).sentence().unwrap()), passive);
        // Mirror shape: a genuinely elided modal relative still drops its
        // predicate and renders the bare modal.
        let elided = "Exile each creature that can't.";
        assert_eq!(render_sentence(parse(elided).sentence().unwrap()), elided);
    }

    #[test]
    fn distributive_each_copular_carries_each_and_binds_the_standard() {
        // The characteristic-defining copular: a coordinated `power and
        // toughness` subject, the distributive `each` floating between the
        // copula and the `equal to <measure>` complement. The `each` is carried
        // as a flag on the copular predicate (never re-derived from the subject
        // shape) and the `to`-standard binds to `equal` as a prepositional
        // adjective complement rather than escaping as a clause adjunct.
        for measure in [
            "the number of lands you control",
            "the number of creatures you control",
        ] {
            let source = format!("Nissa's power and toughness are each equal to {measure}.");
            let parsed = parse_self(&source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            let SentenceBody::Independent(IndependentClause::Copular(_, predicate)) =
                &parsed.sentence().expect("sentence root").body
            else {
                panic!(
                    "expected a copular clause for {source:?}: {:#?}",
                    parsed.sentence()
                );
            };
            assert!(
                predicate.distributive_each,
                "each must be carried: {predicate:#?}"
            );
            let crate::syntax::CopularComplement::Adjective(adjective) = &predicate.complement
            else {
                panic!("expected an adjective complement: {predicate:#?}");
            };
            assert_eq!(adjective.head, Adjective::Word(Vocab::Equal));
            assert!(
                matches!(
                    adjective.complements.as_slice(),
                    [AdjectiveComplement::Prepositional(preposition)]
                        if preposition.preposition == crate::syntax::Preposition::To
                ),
                "the `to`-standard must bind to `equal`: {adjective:#?}"
            );
            assert!(
                predicate.adjuncts.is_empty(),
                "the standard must not float as a clause adjunct: {predicate:#?}"
            );
            assert_eq!(
                render_sentence_as(parsed.sentence().unwrap(), "Nissa Revane", true),
                format!("Nissa's power and toughness are each equal to {measure}."),
                "{source}",
            );
        }
    }

    #[test]
    fn rules_bundle_negation_hyphenates_and_round_trips_the_full_sentence() {
        // Shoot the Sheriff: `outlaw` is a lowercase rules-bundle word, but its
        // `non-` negation still hyphenates (`non-outlaw`) — derived from the
        // bundle category, not a spelling guess in the renderer.
        let source = "Destroy target non-outlaw creature.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn rules_bundle_words_round_trip_the_full_sentence() {
        // Each bundle shorthand parses structurally in the slot it occupies and
        // renders back byte-exactly: `modified` attributively (Kodama of the
        // West Tree) and predicatively (Obstinate Gargoyle), `outlaw` as a head
        // noun (Vihaan, Goldwaker), and `historic` attributively.
        for source in [
            "Destroy target modified creature.",
            "This creature has flying as long as it's modified.",
            "Outlaws you control have haste.",
            "Destroy target historic permanent.",
        ] {
            let parsed = parse(source);
            assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        }
    }

    #[test]
    fn non_distributive_copular_forms_are_unchanged_and_lack_each() {
        // Mirror direction: the singular and the plural-without-`each` forms
        // stay on the intransitive `be` + adjective + prepositional-adjunct
        // analysis the earlier grammar already produced — the `each` frame does
        // not steal them — and each round-trips. The distinction between these
        // and the `each` form is exactly the carried flag, never a spelling
        // guess in the renderer.
        for source in [
            "Nissa's power is equal to the number of lands you control.",
            "Nissa's power and toughness are equal to the number of lands you control.",
        ] {
            let parsed = parse_self(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert!(
                matches!(
                    &parsed.sentence().expect("sentence root").body,
                    SentenceBody::Independent(IndependentClause::Intransitive(_, _))
                ),
                "non-`each` copular must stay intransitive: {:#?}",
                parsed.sentence(),
            );
            let rendered = render_sentence_as(parsed.sentence().unwrap(), "Nissa Revane", true);
            assert!(
                !rendered.contains(" each "),
                "no `each` may be synthesized for the non-distributive form: {rendered}"
            );
            assert_eq!(rendered, source, "{source}");
        }
    }

    // ---- Quoted abilities in coordinated hosts ----

    /// A quoted ability fills a grant verb's object inside a coordinated
    /// predicate (`… gets +N/+N and has "…"`), reached through the existing
    /// clause coordination that already handles `get +1/+1 and have haste`.
    #[test]
    fn quoted_ability_is_a_coordinated_grant_predicate_object() {
        let source = "Enchanted creature gets +2/+2 and has \"{T}: Draw a card.\"";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Coordinated(coordination)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected coordinated predicates: {:#?}", parsed.sentence());
        };
        let [
            ClauseCoordination {
                member: CoordinatedClauseMember::SharedPredicate(Predicate::Transitive(shared)),
                ..
            },
        ] = coordination.rest.as_slice()
        else {
            panic!("expected one shared-predicate conjunct: {coordination:#?}");
        };
        assert!(
            matches!(&shared.object, PredicateObject::QuotedAbility(quoted) if quoted.terminal_period),
            "the shared predicate's object is the closed, sentence-final quote: {:#?}",
            shared.object
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    /// Two quoted abilities coordinate as one object (`has "A" and "B."`); only
    /// the sentence-final conjunct keeps its terminal period inside the quote.
    #[test]
    fn two_quoted_abilities_are_a_coordinated_object() {
        let source = "Enchanted creature has \"When this creature dies, draw a card\" and \"{T}: Draw a card.\"";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a transitive clause: {:#?}", parsed.sentence());
        };
        let PredicateObject::Coordinated(CoordinatedPredicateObject { first, rest }) =
            &predicate.object
        else {
            panic!("expected a coordinated object: {:#?}", predicate.object);
        };
        assert!(
            matches!(first.as_ref(), PredicateObject::QuotedAbility(quoted) if !quoted.terminal_period),
            "the non-final conjunct drops its interior period: {first:#?}"
        );
        assert!(
            matches!(rest.as_slice(), [PredicateObjectCoordination {
                conjunction: Some(crate::syntax::PredicateConjunction::And),
                object: PredicateObject::QuotedAbility(quoted),
                ..
            }] if quoted.terminal_period),
            "the final conjunct keeps its interior period: {rest:#?}"
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    /// A keyword ability and a quoted ability coordinate as one object
    /// (`has flying and "…"`) — the mixed conjunct the noun-phrase coordination
    /// cannot form.
    #[test]
    fn keyword_and_quoted_ability_are_a_coordinated_object() {
        let source = "Enchanted creature has flying and \"{T}: Draw a card.\"";
        let parsed = parse(source);
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a transitive clause: {:#?}", parsed.sentence());
        };
        let PredicateObject::Coordinated(CoordinatedPredicateObject { first, rest }) =
            &predicate.object
        else {
            panic!("expected a coordinated object: {:#?}", predicate.object);
        };
        assert!(
            matches!(first.as_ref(), PredicateObject::Ability(ability) if ability.argument.is_none()),
            "the first conjunct is the keyword-ability object: {first:#?}"
        );
        assert!(
            matches!(
                rest.as_slice(),
                [PredicateObjectCoordination {
                    object: PredicateObject::QuotedAbility(_),
                    ..
                }]
            ),
            "the second conjunct is the quoted ability: {rest:#?}"
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    /// The interior's terminal period is a positional distinction carried on
    /// the node, not derived from the interior alone: a quote before a
    /// trailing adjunct drops it (`gains "…" until end of turn.`); the same
    /// quote in sentence-final position keeps it (`gains "…."`). Both
    /// round-trip.
    #[test]
    fn quoted_ability_interior_period_tracks_sentence_final_position() {
        let non_final = "This creature gains \"{T}: Draw a card\" until end of turn.";
        let parsed = parse(non_final);
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a transitive clause: {:#?}", parsed.sentence());
        };
        assert!(
            matches!(&predicate.object, PredicateObject::QuotedAbility(quoted) if !quoted.terminal_period),
            "a quote before a trailing adjunct drops its interior period: {:#?}",
            predicate.object
        );
        assert!(
            !predicate.elements.is_empty(),
            "the trailing adjunct survives"
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), non_final);

        let sentence_final = "This creature gains \"{T}: Draw a card.\"";
        let parsed = parse(sentence_final);
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a transitive clause: {:#?}", parsed.sentence());
        };
        assert!(
            matches!(&predicate.object, PredicateObject::QuotedAbility(quoted) if quoted.terminal_period),
            "a sentence-final quote keeps its interior period: {:#?}",
            predicate.object
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), sentence_final);
    }

    /// Residue: a three-member Oxford-comma object list ending in a quoted
    /// ability (`has flying, haste, and "…"`) is the general comma-coordination
    /// gap, not covered this round — it stays recovering rather than parsing
    /// wrong.
    #[test]
    fn three_way_oxford_comma_quoted_object_list_recovers() {
        let source = "Enchanted creature has flying, haste, and \"{T}: Draw a card.\"";
        assert!(
            parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "the three-way Oxford-comma quoted object list must not parse"
        );
    }

    #[test]
    fn modal_finite_first_clause_hosts_a_bare_imperative_chain() {
        let source = "You may search your library for a Plains card, reveal it, put it into your hand, then shuffle.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    #[test]
    fn finite_subject_bearing_first_clause_hosts_a_then_joined_bare_imperative() {
        let source = "You draw a card, then discard a card.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    /// A finite clause and a following subject-bearing clause remain two
    /// independent members, not a shared-predicate reduction — the allowance
    /// is narrow to subjectless standalone-imperative continuations only.
    #[test]
    fn finite_first_clause_then_subject_bearing_clause_stays_independent() {
        let source = "You draw a card, then that player shuffles.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    /// Two sentences never merge across a period, even when the first is
    /// finite and the second would otherwise look like a bare imperative.
    #[test]
    fn two_sentences_do_not_merge_across_a_period() {
        let source = "You gain 2 life. Draw a card.";
        assert!(
            parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "a period-separated pair must not parse as a single Sentence nonterminal"
        );
    }

    /// A modal on the *continuation* ("you may draw a card") is a distinct,
    /// rarer shape left out of scope by this round's narrow allowance; the
    /// `lower_coordination` modal-continuation guard stays untouched and the
    /// span stays unparsed rather than parsing wrong.
    #[test]
    fn modal_on_a_continuation_stays_unparsed() {
        let source = "You may search your library for a Plains card, you may draw a card.";
        assert!(
            parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "a modal on the continuation must not parse via the new allowance"
        );
    }

    /// A third-person host cannot adopt a bare imperative tail. Without this
    /// restriction the allowance let unparsed trigger sentences misparse with
    /// the trigger word licensed as an opaque noun subject ("When [you lose]"
    /// as a nominal, "control" as its verb) and the effect clause absorbed as
    /// a shared-predicate continuation — render-identical but structurally
    /// wrong. The sentence must stay unparsed and recover honestly.
    #[test]
    fn third_person_host_does_not_adopt_a_bare_imperative_tail() {
        let source = "When there are no lands on the battlefield, sacrifice this enchantment.";
        assert!(
            parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "a trigger sentence must not misparse as third-person coordination"
        );
    }

    /// A coordinated junk subject donating Second-person agreement by
    /// nearest-conjunct (`When [rel] or you`) is not a genuine addressee
    /// subject (`pronoun_case: None`), and `lose` is not a modal, so
    /// `host_adopts_imperative` stays false and the asyndetic tail is
    /// rejected at both gates — the sentence stays honestly unparsed rather
    /// than misparsing with `When` licensed as an opaque noun subject.
    #[test]
    fn coordinated_donated_agreement_does_not_adopt_a_bare_imperative_tail() {
        let source = "When this creature becomes untapped or you lose control of this creature, exile that creature.";
        assert!(
            parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "nearest-conjunct-donated agreement must not license the asyndetic tail"
        );
    }

    // NOTE (out of scope for this round): the plan's negative fixtures
    // `"That player sacrifices a creature, then draw a card."` and
    // `"There is a creature, then draw a card."` were expected to be
    // rejected by the new `host_adopts_imperative` gate. In fact both still
    // parse — not via the new gate, but via two pre-existing,
    // `coordination_agrees` arms this round did not touch:
    // `(Some(_), None, false)` (fires because the bare-stem "draw a card"
    // continuation here lexes as `PredicateForm::Infinitive`, i.e.
    // `standalone: false`, not `Imperative`) and `(None, None, true)`
    // (fires unconditionally whenever the host's own `agreement` is `None`,
    // as it is for an existential clause, regardless of host shape). Both
    // arms predate this round and are unrelated to `host_addressee_subject`/
    // `host_modal`, so fixing them is out of scope here; they are a
    // pre-existing third-person-host over-fire left for a future round.
    // The re-key correctly rejects both shapes via `host_adopts_imperative`
    // — see `coordinated_donated_agreement_does_not_adopt_a_bare_imperative_tail`
    // for the shape the new gate is actually responsible for.

    /// A third-person modal host adopts a bare-imperative chain sharing the
    /// modal, not the addressee (the Cleansing Wildfire shape).
    #[test]
    fn third_person_modal_host_adopts_a_bare_imperative_chain() {
        let source = "Its controller may draw a card, then discard a card.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    fn parse(source: &str) -> ParsedNonterminal {
        parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
    }

    /// Parses a sentence as the legendary face `Nissa Revane` (nickname
    /// `Nissa`), so a `Nissa`/`Nissa's` self-reference is recognized.
    fn parse_self(source: &str) -> ParsedNonterminal {
        parse_nonterminal_with_self_reference(
            source,
            &fixture_catalogs(),
            Nonterminal::Sentence,
            &SelfReference::new("Nissa Revane", true),
        )
        .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
    }

    fn fixture_catalogs() -> Catalogs {
        Catalogs::default()
            .with_catalog(
                CatalogKind::KeywordAbility,
                ["Flying", "Haste", "Flash", "Defender", "Hexproof"],
            )
            .with_catalog(CatalogKind::CreatureType, ["Goblin", "Mount"])
            .with_catalog(CatalogKind::LandType, ["Plains", "Swamp", "Mountain"])
            .with_catalog(
                CatalogKind::CardType,
                ["Creature", "Land", "Sorcery", "Planeswalker"],
            )
            .with_catalog(CatalogKind::ArtifactType, ["Vehicle"])
    }

    fn finite(sentence: &Sentence) -> (&Subject, &crate::syntax::PredicateHead) {
        let SentenceBody::Independent(clause) = &sentence.body else {
            panic!("expected an independent clause, got {:?}", sentence.body);
        };
        match clause {
            IndependentClause::Transitive(subject, predicate) => (subject, &predicate.head),
            IndependentClause::Intransitive(subject, predicate) => (subject, &predicate.head),
            IndependentClause::Passive(subject, predicate) => (subject, &predicate.head),
            other => panic!("expected a finite lexical predicate, got {other:?}"),
        }
    }

    fn turn_head_spelling(nominal: &crate::syntax::NominalPhrase) -> &'static str {
        match &nominal.head {
            NounInstance::Singular(Noun::Word(word)) | NounInstance::Mass(Noun::Word(word)) => {
                word.spelling()
            }
            other => panic!("unexpected nominal head: {other:?}"),
        }
    }

    fn sole_adjective_spelling(nominal: &crate::syntax::NominalPhrase) -> &'static str {
        for modifier in &nominal.modifiers {
            if let NominalModifier::Adjective {
                phrase:
                    crate::syntax::AdjectivePhrase {
                        head: Adjective::Word(word),
                        ..
                    },
                ..
            } = modifier
            {
                return word.spelling();
            }
        }
        panic!("no word-adjective modifier in {nominal:#?}");
    }

    fn render_sentence(sentence: &Sentence) -> String {
        render_sentence_as(sentence, "Test Card", false)
    }

    fn render_sentence_as(sentence: &Sentence, name: &str, is_legendary: bool) -> String {
        OracleText {
            abilities: vec![Ability {
                ability_word: None,
                flavor_header: None,
                kind: AbilityKind::Paragraph(Paragraph {
                    flavor_header: None,
                    sentences: vec![sentence.clone()],
                }),
            }],
        }
        .render(name, is_legendary)
        .expect("parsed sentence must render")
    }

    #[test]
    fn combat_step_restrictions_parse_and_render_structurally() {
        for source in [
            "Cast this spell only during the declare blockers step.",
            "Activate only during the declare blockers step.",
            "Cast this spell only during your declare attackers step.",
            "Cast this spell only during the declare blockers step on an opponent's turn.",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn restriction_run_admits_an_if_clause_member_true_witness() {
        // The 14-duplicate flagship, with the real `declare attackers step`
        // nominal (Stage B, landed this round).
        let source = "Cast this spell only during the declare attackers step and only if you've been attacked this step.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a complex clause");
        };
        assert_eq!(complex.attachments.len(), 1);
        let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].kind else {
            panic!(
                "expected a Restriction attachment: {:#?}",
                complex.attachments[0]
            );
        };
        let [PredicateAdjunct::Prepositional(during)] = run.first.as_slice() else {
            panic!(
                "expected a single Prepositional first member: {:#?}",
                run.first
            );
        };
        let crate::syntax::Phrase::NounPhrase(object) = during.object.as_ref() else {
            panic!("expected a noun-phrase PP object: {during:#?}");
        };
        let crate::syntax::NounPhrase::Nominal(nominal) = object.as_ref() else {
            panic!("expected a nominal PP object: {during:#?}");
        };
        assert!(
            matches!(
                nominal.modifiers.as_slice(),
                [NominalModifier::CombatStepName { .. }]
            ),
            "{nominal:#?}"
        );
        assert_eq!(run.rest.len(), 1);
        assert_eq!(run.rest[0].conjunction, Some(PredicateConjunction::And));
        assert!(!run.rest[0].comma);
        let [PredicateAdjunct::Dependent(dependent)] = run.rest[0].adjuncts.as_slice() else {
            panic!(
                "expected a single Dependent member: {:#?}",
                run.rest[0].adjuncts
            );
        };
        let DependentClause::Subordinate(Subordinator::If, SubordinateBody::Finite(if_body)) =
            dependent.as_ref()
        else {
            panic!("expected a finite `if` subordinate clause: {dependent:#?}");
        };
        // The tree-shape assertion that would have caught the Slice-B
        // finding: `you've` must parse as a genuine contracted
        // subject+auxiliary pronoun (never an `Opaque("you've")` noun
        // subject), heading a passive `been attacked` with `this step` as a
        // bare temporal adjunct (never the direct object).
        // "been attacked" surfaces as an Intransitive clause carrying the
        // Have+Be auxiliary chain and a PastParticiple verb slot (there is no
        // dedicated agentless-passive variant distinct from this shape).
        let (subject, predicate_head, predicate_elements): (
            &Subject,
            &crate::syntax::PredicateHead,
            &[PredicateElement],
        ) = match if_body.as_ref() {
            IndependentClause::Intransitive(subject, predicate) => {
                (subject, &predicate.head, predicate.elements.as_slice())
            }
            IndependentClause::Passive(subject, predicate) => {
                (subject, &predicate.head, predicate.elements.as_slice())
            }
            other => panic!("expected a `you've been attacked` clause: {other:#?}"),
        };
        assert_eq!(
            subject.0,
            crate::syntax::NounPhrase::Pronoun {
                pronoun: crate::word::Pronoun::You,
                case: crate::word::PronounCase::Subject,
            },
            "{subject:#?}"
        );
        assert!(
            predicate_head.first_auxiliary_contracted_with_subject,
            "{predicate_head:#?}"
        );
        let auxiliaries: Vec<crate::word::Auxiliary> = predicate_head
            .auxiliaries
            .iter()
            .map(|instance| instance.auxiliary)
            .collect();
        assert_eq!(
            auxiliaries,
            vec![crate::word::Auxiliary::Have, crate::word::Auxiliary::Be],
            "{predicate_head:#?}"
        );
        assert_eq!(predicate_head.verb.slot, VerbSlot::PastParticiple);
        assert!(
            matches!(predicate_head.verb.verb, crate::word::Verb::Word(_)),
            "{predicate_head:#?}"
        );
        let temporal_adjuncts: Vec<&NounPhrase> = predicate_elements
            .iter()
            .filter_map(|element| match element {
                PredicateElement::Adjunct(PredicateAdjunct::Temporal(np)) => Some(np),
                _ => None,
            })
            .collect();
        assert_eq!(
            temporal_adjuncts.len(),
            1,
            "expected `this step` as a Temporal adjunct, not an object: {predicate_elements:#?}"
        );
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
    }

    #[test]
    fn step_is_a_bare_temporal_adjunct() {
        for source in [
            "You have been attacked this step.",
            "This creature has been attacked this step.",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn contracted_youve_round_trips_contracted() {
        let source = "You've been attacked this step.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(
            render_sentence(parsed.sentence().unwrap()),
            source,
            "must render as `You've`, never `You have`"
        );
    }

    #[test]
    fn the_tie_is_broken_is_not_a_predicate_nominal() {
        // Timesifter: before Edit D, `broken` was unknown to the lexicon, so
        // `the tie is broken` parsed as a copular predicate-nominal with an
        // opaque head instead of a passive verb phrase. `break`/`broken` is
        // now a declared irregular verb, closing that reading.
        let source = "The tied players repeat this process until the tie is broken.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let debug = format!("{:#?}", parsed.sentence().unwrap());
        assert!(
            !debug.contains("OpaqueLexeme"),
            "broken must never lower as an OpaqueLexeme: {debug}"
        );
        assert!(
            debug.contains("Passive"),
            "the until-complement must be a passive verb phrase, not a copular predicate-nominal: {debug}"
        );
    }

    #[test]
    fn youve_is_never_an_opaque_noun() {
        let source = "Cast this spell only during the declare attackers step and only if you've been attacked this step.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        let debug = format!("{:#?}", parsed.sentence().unwrap());
        assert!(
            !debug.contains("OpaqueLexeme(\n            \"you've\""),
            "you've must never lower as an OpaqueLexeme: {debug}"
        );
    }

    #[test]
    fn declarestep_slice_b_unchanged_behavior_negatives() {
        for source in [
            "You have been attacked this turn.",
            "You have attacked this step.",
            "Skip your draw step.",
            "Activate only during the declare blockers step.",
            "Cast this spell only during the declare blockers step.",
            "Activate only during your upkeep.",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
        }
    }

    #[test]
    fn oxford_restriction_run_carries_member_boundaries_true_witness() {
        // Grizzled Wolverine, with the real `declare blockers step` nominal.
        let source = "Activate only during the declare blockers step, only if at least one creature is blocking this creature, and only once each turn.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a complex clause");
        };
        let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].kind else {
            panic!(
                "expected a Restriction attachment: {:#?}",
                complex.attachments[0]
            );
        };
        assert_eq!(run.rest.len(), 2);
        assert_eq!(run.rest[0].conjunction, None);
        assert!(run.rest[0].comma);
        assert_eq!(run.rest[1].conjunction, Some(PredicateConjunction::And));
        assert!(run.rest[1].comma);
    }

    #[test]
    fn combat_step_name_does_not_capture_target_nominals() {
        // The `restrict`-round regression guard, stated as a shape claim: the
        // reverted design recognized any `Verb(Imperative) Noun(Either)
        // Nominal` sequence, which matched `target`/`creature`/`card with
        // …` in this exact sentence and hijacked the whole 13-token phrase
        // (`declarestep-forest-dump.txt`: `child0 = Verb(Word(Target),
        // Imperative)`, `child1 = Noun(Singular(Catalog(CreatureType)))`,
        // `child2` swallowing the entire comparison remainder). This
        // round's three literal-token slots (`CombatStepDeclare` /
        // `CombatStepParticipants` / `CombatStepHead`) are structurally
        // incapable of matching any token but `declare`/`attackers`or
        // `blockers`/`step`, so the comparison stays inside the
        // prepositional object where it belongs.
        let source = "target creature card with mana value less than or equal to Nissa's power";
        let parsed = parse_nonterminal_with_self_reference(
            source,
            &fixture_catalogs(),
            Nonterminal::NounPhrase,
            &SelfReference::new("Nissa Revane", true),
        )
        .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
    }

    #[test]
    fn declare_step_nominal_is_unambiguous() {
        let source = "the declare attackers step";
        let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
        assert!(
            parsed.root_tied_alternatives().len() <= 1,
            "{source} has a real forest tie: {:?}",
            parsed.root_tied_alternatives()
        );
    }

    /// Every witness below asserts point (c) of the plan-C verification
    /// protocol on the *complete* AST: the clause's predicate elements
    /// contain no `PredicateAdjunct::Temporal` that was not written as a
    /// temporal adjunct in the source. This is the assertion whose absence
    /// let slice B land a corpus-wide `<modifier> step`-compound split
    /// (`declarestep-plan-C.md` §5).
    fn no_stray_temporal_adjunct(elements: &[PredicateElement]) -> bool {
        !elements.iter().any(|element| {
            matches!(
                element,
                PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))
            )
        })
    }

    /// Extracts the predicate elements from whichever finite/imperative
    /// clause shape the sentence actually parsed as (transitive,
    /// intransitive, or passive; imperative wraps any of those in a bare
    /// `Predicate` with no subject). Point (c) of the plan-C verification
    /// protocol reads these elements regardless of clause shape.
    fn clause_elements(clause: &IndependentClause) -> &[PredicateElement] {
        fn predicate_elements(predicate: &Predicate) -> &[PredicateElement] {
            match predicate {
                Predicate::Transitive(predicate) => predicate.elements.as_slice(),
                Predicate::Intransitive(predicate) => predicate.elements.as_slice(),
                Predicate::Passive(predicate) => predicate.elements.as_slice(),
                other => panic!("expected a transitive/intransitive/passive predicate: {other:#?}"),
            }
        }
        match clause {
            IndependentClause::Imperative(predicate) => predicate_elements(predicate),
            IndependentClause::Transitive(_, predicate) => predicate.elements.as_slice(),
            IndependentClause::Intransitive(_, predicate) => predicate.elements.as_slice(),
            IndependentClause::Passive(_, predicate) => predicate.elements.as_slice(),
            other => panic!("expected a clause with predicate elements: {other:#?}"),
        }
    }

    fn intransitive_or_passive(
        clause: &IndependentClause,
    ) -> (&Subject, &crate::syntax::PredicateHead, &[PredicateElement]) {
        match clause {
            IndependentClause::Intransitive(subject, predicate) => {
                (subject, &predicate.head, predicate.elements.as_slice())
            }
            IndependentClause::Passive(subject, predicate) => {
                (subject, &predicate.head, predicate.elements.as_slice())
            }
            other => panic!("expected an intransitive or passive clause: {other:#?}"),
        }
    }

    #[test]
    fn cleanup_step_compound_is_not_split() {
        // Ancient Adamantoise: `cleanup` is a genuine, pre-existing opaque
        // lexeme (no vocabulary entry) — it must stay in MODIFIER position on
        // head `steps`, never split out as a bare temporal adjunct.
        let source = "Damage isn't removed from this creature during cleanup steps.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(clause) = &parsed.sentence().unwrap().body else {
            panic!("expected an independent clause");
        };
        let (_, _, elements) = intransitive_or_passive(clause);
        assert!(
            no_stray_temporal_adjunct(elements),
            "no PredicateAdjunct::Temporal may appear here: {elements:#?}"
        );
        // "removed from this creature during cleanup steps": the `during`
        // PP attaches as a complement of `this creature`, inside the `from`
        // PP adjunct — not directly at clause scope.
        let from_pp = elements
            .iter()
            .find_map(|element| match element {
                PredicateElement::Adjunct(PredicateAdjunct::Prepositional(pp))
                    if pp.preposition == Preposition::From =>
                {
                    Some(pp)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("expected a `from` PP adjunct: {elements:#?}"));
        let crate::syntax::Phrase::NounPhrase(from_object) = from_pp.object.as_ref() else {
            panic!("expected a noun-phrase PP object: {from_pp:#?}");
        };
        let crate::syntax::NounPhrase::Nominal(from_nominal) = from_object.as_ref() else {
            panic!("expected a nominal PP object: {from_pp:#?}");
        };
        let during_pp = from_nominal
            .complements
            .iter()
            .find_map(|complement| match complement {
                crate::syntax::NominalComplement::Prepositional(pp)
                    if pp.preposition == Preposition::During =>
                {
                    Some(pp)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("expected a `during` PP complement: {from_nominal:#?}"));
        let crate::syntax::Phrase::NounPhrase(object) = during_pp.object.as_ref() else {
            panic!("expected a noun-phrase PP object: {during_pp:#?}");
        };
        let crate::syntax::NounPhrase::Nominal(nominal) = object.as_ref() else {
            panic!("expected a nominal PP object: {during_pp:#?}");
        };
        assert!(
            matches!(
                &nominal.head,
                NounInstance::Plural(Noun::Word(word)) if word.spelling() == "step"
            ),
            "expected head `steps`: {nominal:#?}"
        );
        assert!(
            matches!(
                nominal.modifiers.as_slice(),
                [NominalModifier::Noun {
                    noun: NounInstance::Singular(Noun::Opaque(opaque)),
                    ..
                }] if opaque.spelling() == "cleanup"
            ),
            "`cleanup` must sit as an opaque MODIFIER of head `step`, never a split-out head: {nominal:#?}"
        );
        // The census-legitimate debt: exactly one opaque word, in modifier
        // position (the noun-opacity walker only counts nominal HEAD
        // opacity, so this correctly contributes to the census while never
        // surfacing as a stray adjunct).
        assert_eq!(parsed.opacity_mode(), OpacityMode::OpaqueNouns);
    }

    #[test]
    fn next_cleanup_step_is_one_nominal() {
        let source = "Exile them at the beginning of the next cleanup step.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(clause) = &parsed.sentence().unwrap().body else {
            panic!("expected an independent clause");
        };
        let elements = clause_elements(clause);
        assert!(
            no_stray_temporal_adjunct(elements),
            "no PredicateAdjunct::Temporal may appear here: {elements:#?}"
        );
        let debug = format!("{elements:#?}");
        assert!(
            debug.contains("\"next\""),
            "expected the `next` adjective on the `beginning of …` object: {debug}"
        );
        assert!(
            debug.contains("\"cleanup\""),
            "expected `cleanup` as a modifier inside the same nominal: {debug}"
        );
    }

    #[test]
    fn known_noun_step_compounds_are_not_split() {
        // The zero-opacity regression guard: this is the test whose absence
        // let slice B land — `draw`/`end` are fully known nouns, so the
        // slice-B misparse was invisible on the opacity axis and
        // byte-identical on roundtrip.
        for source in [
            "Exile them at the beginning of the next draw step.",
            "Exile them at the beginning of the next end step.",
        ] {
            let parsed = parse(source);
            assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
            assert_eq!(
                render_sentence(parsed.sentence().unwrap()),
                source,
                "{source}"
            );
            let SentenceBody::Independent(clause) = &parsed.sentence().unwrap().body else {
                panic!("expected an independent clause: {source}");
            };
            let elements = clause_elements(clause);
            assert!(
                no_stray_temporal_adjunct(elements),
                "no PredicateAdjunct::Temporal may appear here ({source}): {elements:#?}"
            );
            let debug = format!("{elements:#?}");
            assert!(
                debug.contains("\"next\""),
                "expected the compound modifier attached, not split ({source}): {debug}"
            );
        }
    }

    #[test]
    fn this_step_is_still_a_temporal_adjunct() {
        // Slice B's win, preserved: `this step` is determined, so it keeps
        // its bare-temporal-adjunct licensing under the slice-C gate.
        let source = "Cast this spell only during the declare attackers step and only if you've been attacked this step.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(IndependentClause::Complex(complex)) =
            &parsed.sentence().unwrap().body
        else {
            panic!("expected a complex clause");
        };
        let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].kind else {
            panic!(
                "expected a Restriction attachment: {:#?}",
                complex.attachments[0]
            );
        };
        let [PredicateAdjunct::Dependent(dependent)] = run.rest[0].adjuncts.as_slice() else {
            panic!(
                "expected a single Dependent member: {:#?}",
                run.rest[0].adjuncts
            );
        };
        let DependentClause::Subordinate(Subordinator::If, SubordinateBody::Finite(if_body)) =
            dependent.as_ref()
        else {
            panic!("expected a finite `if` subordinate clause: {dependent:#?}");
        };
        let (subject, predicate_head, predicate_elements) = intransitive_or_passive(if_body);
        assert_eq!(
            subject.0,
            crate::syntax::NounPhrase::Pronoun {
                pronoun: crate::word::Pronoun::You,
                case: crate::word::PronounCase::Subject,
            },
            "{subject:#?}"
        );
        assert!(predicate_head.first_auxiliary_contracted_with_subject);
        let temporal_adjuncts: Vec<&NounPhrase> = predicate_elements
            .iter()
            .filter_map(|element| match element {
                PredicateElement::Adjunct(PredicateAdjunct::Temporal(np)) => Some(np),
                _ => None,
            })
            .collect();
        assert_eq!(
            temporal_adjuncts.len(),
            1,
            "expected `this step` as a Temporal adjunct: {predicate_elements:#?}"
        );
    }

    #[test]
    fn bare_nominals_are_not_temporal_adjuncts() {
        // Negative: a completely bare `step` nominal never appears as a
        // Temporal adjunct in any of the witnesses above.
        for source in [
            "Damage isn't removed from this creature during cleanup steps.",
            "Exile them at the beginning of the next cleanup step.",
            "Exile them at the beginning of the next draw step.",
            "Exile them at the beginning of the next end step.",
        ] {
            let parsed = parse(source);
            let SentenceBody::Independent(clause) = &parsed.sentence().unwrap().body else {
                panic!("expected an independent clause: {source}");
            };
            let elements = clause_elements(clause);
            assert!(
                no_stray_temporal_adjunct(elements),
                "a bare `step` nominal must never surface as a Temporal adjunct ({source}): {elements:#?}"
            );
        }
    }

    /// `No spells were cast last turn.` — `last turn` is undetermined but
    /// modified (the adjective `last`), which is precisely and only what the
    /// `determined || modified` gate's `|| modified` disjunct restores
    /// (`declarestepC-stateD-suite.txt`: without it, this is the single
    /// casualty of the `determined`-only gate). Kept alongside the existing
    /// `passive_temporal_adjunct_is_not_a_direct_object`, which covers the
    /// same shape and is this slice's named first gate.
    #[test]
    fn modified_undetermined_temporal_adjunct_survives() {
        let source = "No spells were cast last turn.";
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
        let SentenceBody::Independent(clause) = &parsed.sentence().unwrap().body else {
            panic!("expected an independent clause");
        };
        let (_, _, elements) = intransitive_or_passive(clause);
        let temporal_adjuncts: Vec<&NounPhrase> = elements
            .iter()
            .filter_map(|element| match element {
                PredicateElement::Adjunct(PredicateAdjunct::Temporal(np)) => Some(np),
                _ => None,
            })
            .collect();
        assert_eq!(
            temporal_adjuncts.len(),
            1,
            "expected `last turn` as a Temporal adjunct: {elements:#?}"
        );
    }

    fn parse_clause(source: &str) -> Clause {
        parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Clause)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
            .clause()
            .unwrap_or_else(|| panic!("expected a Clause root for {source:?}"))
            .clone()
    }

    fn transitive_predicate_head(clause: &Clause) -> &PredicateHead {
        let Clause::Independent(IndependentClause::Transitive(_, predicate)) = clause else {
            panic!("expected an independent transitive clause: {clause:#?}");
        };
        &predicate.head
    }

    /// The trigger-clause shape (Chandra, the Firebrand's "you next cast a
    /// creature spell this turn"): the preverbal modifier lands on
    /// `preverb_modifiers`, and the object/adjunct structure is otherwise
    /// unchanged from the minus-`next` twin (the Glimpse of Nature shape).
    #[test]
    fn preverbal_next_attaches_to_the_predicate_head() {
        let with_next = parse_clause("you next cast a creature spell this turn");
        let without_next = parse_clause("you cast a creature spell this turn");
        let with_head = transitive_predicate_head(&with_next);
        let without_head = transitive_predicate_head(&without_next);
        assert_eq!(with_head.preverb_modifiers, [PreverbModifier::Next]);
        assert_eq!(without_head.preverb_modifiers, []);
        // Everything besides the preverb modifier (verb, object, adjunct) is
        // unchanged from the minus-`next` twin.
        assert_eq!(with_head.verb, without_head.verb);
        let Clause::Independent(IndependentClause::Transitive(_, with_predicate)) = &with_next
        else {
            panic!("expected transitive");
        };
        let Clause::Independent(IndependentClause::Transitive(_, without_predicate)) =
            &without_next
        else {
            panic!("expected transitive");
        };
        assert_eq!(with_predicate.object, without_predicate.object);
        assert_eq!(with_predicate.elements, without_predicate.elements);
    }

    /// Renders back to the exact source string, with `next` before the verb —
    /// not `PredicateAttachment::Adjunct`'s post-verbal position (§2.4's
    /// round-trip failure mode: `cast next` rather than `next cast`).
    #[test]
    fn preverbal_next_round_trips_before_the_verb() {
        let source = "You next cast a creature spell this turn.";
        let parsed = parse(source);
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }

    /// The pinned literal matcher admits only `next`: no other adverb in the
    /// vocabulary (`only`/`just`/`once`/`twice`/`still`) reduces preverbally,
    /// and a synthetic `you only cast a spell` has no complete parse.
    #[test]
    fn preverb_adverb_slot_admits_only_next() {
        // Every existing clause fixture's preverb_modifiers stays empty: the
        // pinned literal matcher (§2.1) only ever recognizes `next`, so no
        // other adverb reduces preverbally regardless of shape.
        for source in FIXTURES {
            let parsed = parse(source);
            let SentenceBody::Independent(clause) = &parsed.sentence().unwrap().body else {
                continue;
            };
            let head = match clause {
                IndependentClause::Transitive(_, predicate) => Some(&predicate.head),
                IndependentClause::Intransitive(_, predicate) => Some(&predicate.head),
                IndependentClause::Imperative(predicate) => match predicate {
                    Predicate::Transitive(p) => Some(&p.head),
                    Predicate::Intransitive(p) => Some(&p.head),
                    _ => None,
                },
                _ => None,
            };
            if let Some(head) = head {
                assert!(
                    head.preverb_modifiers.is_empty(),
                    "unexpected preverb modifier in fixture {source:?}: {head:#?}"
                );
            }
        }
        // A synthetic `you only cast a spell` — substituting `only` for the
        // pinned literal — has no complete parse: `only`/`just`/`once`/
        // `twice`/`still` never reduce through `RuleTag::VerbPhrasePreverbAdverb`.
        for adverb in ["only", "just", "once", "twice", "still"] {
            let source = format!("you {adverb} cast a spell");
            assert!(
                parse_nonterminal(&source, &fixture_catalogs(), Nonterminal::Clause).is_err(),
                "expected {source:?} to have no complete parse"
            );
        }
    }

    /// Attributive `next` over a spell nominal (Barl's Cage's "its
    /// controller's next untap step") keeps `next` as an adjective modifier —
    /// no `preverb_modifiers`, and no `VerbPhrase` node spanning `next untap`.
    #[test]
    fn attributive_next_is_not_a_preverb_modifier() {
        let source = "its controller's next untap step";
        let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
            .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
        let NounPhrase::Nominal(nominal) = parsed
            .noun_phrase()
            .unwrap_or_else(|| panic!("expected a NounPhrase root for {source:?}"))
        else {
            panic!("expected a nominal NounPhrase for {source:?}");
        };
        assert!(
            nominal.modifiers.iter().any(|modifier| matches!(
                modifier,
                NominalModifier::Adjective {
                    phrase: AdjectivePhrase {
                        head: Adjective::Word(vocab),
                        ..
                    },
                    ..
                } if vocab.spelling() == "next"
            )),
            "expected `next` as an adjective modifier: {nominal:#?}"
        );
    }

    /// `next` before a verb-homograph noun (Fatigue's "their next draw
    /// step", Exhaustion's "their next untap step") stays a nominal — no
    /// `VerbPhrase` reduction ever considers `draw`/`untap` a verb here.
    #[test]
    fn next_before_a_verb_homograph_noun_does_not_form_a_verb_phrase() {
        for source in ["their next draw step", "their next untap step"] {
            let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
                .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
            assert!(
                parsed.noun_phrase().is_some(),
                "expected a NounPhrase root for {source:?}"
            );
        }
    }
}
