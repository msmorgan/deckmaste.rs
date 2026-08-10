use super::EnglishLexicalSlot;
use super::Expected;
use super::Nonterminal;
use super::ParseCost;
use super::Punctuation;
use super::RuleBuilder;
use super::RuleTag;

#[allow(
    clippy::too_many_lines,
    reason = "grammar table builder is intentionally verbose"
)]
pub(in crate::grammar) fn add_rules(builder: &mut RuleBuilder) {
    use EnglishLexicalSlot as L;
    use Expected::Lexical as l;
    use Expected::Nonterminal as n;
    use Nonterminal as N;

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
    // The relative-clause counterpart of the finite verbal quantifier float
    // (`target creature cards that each have a different mana value`). No
    // dot-1 gate is possible: `RelativeMarker` carries `Features::None`. The
    // reduce-time gate in `reduce_simple_clause` requires a complete finite
    // plural verb phrase.
    builder.add(
        RuleTag::RelativeSubjectDistributiveEach,
        N::RelativeClause,
        [l(L::RelativeMarker), l(L::EachDeterminer), n(N::VerbPhrase)],
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

    // The productions below are appended after every pre-existing rule so their
    // rule indices are the highest in the grammar. The forest's equal-cost
    // tiebreak prefers the lowest rule index, so a new production that competes
    // with an existing analysis for the same span (a copular prepositional
    // adjunct against a noun-internal complement) loses the tie, leaving every
    // already-clean parse untouched.

    // The exception rider: a leading `except` marker heading a coordinated list
    // of finite clauses. An asyndetic comma run lives on `ExceptionRiderList`
    // until it closes; this keeps punctuation-derived list state out of chart
    // features while preserving the dispreferred direct attachment fallback.
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
        RuleTag::ExceptionRiderConjoined,
        N::ExceptionRider,
        [n(N::ExceptionRiderList), l(L::Conjunction), n(N::Clause)],
    );
    builder.add(
        RuleTag::ExceptionRiderComma,
        N::ExceptionRiderList,
        [
            n(N::ExceptionRider),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::Clause),
        ],
    );
    builder.add(
        RuleTag::ExceptionRiderComma,
        N::ExceptionRiderList,
        [
            n(N::ExceptionRiderList),
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
        RuleTag::ExceptionRiderOxford,
        N::ExceptionRider,
        [
            n(N::ExceptionRiderList),
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
    builder.add_with_cost(
        RuleTag::ClauseExcepted,
        N::Clause,
        [
            n(N::Clause),
            l(L::Punctuation(Punctuation::Comma)),
            n(N::ExceptionRiderList),
        ],
        ParseCost {
            reading_dispreference: 1,
            ..ParseCost::default()
        },
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
