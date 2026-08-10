use super::EnglishLexicalSlot;
use super::Expected;
use super::Nonterminal;
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
}
