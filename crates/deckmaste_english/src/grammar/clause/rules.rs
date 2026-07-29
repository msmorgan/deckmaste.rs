use super::EnglishLexicalSlot;
use super::Expected;
use super::Nonterminal;
use super::Numeral;
use super::ParseCost;
use super::Punctuation;
use super::RuleBuilder;
use super::RuleTag;
use super::VerbParticle;

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
    // The finite verbal quantifier float (`Two target creatures each get
    // +2/+2 ...`). The dot-1 host gate lives in `accepts_predicate_prefix`
    // (plural/second-person subject, non-object case); the reduce-time gate
    // in `reduce_simple_clause` rechecks it and additionally requires exact
    // subject/predicate agreement and complete predicate arguments. Appended
    // after the ordinary subject production so existing `RuleId`s are
    // preserved.
    builder.add(
        RuleTag::SimpleClauseSubjectDistributiveEach,
        N::SimpleClause,
        [n(N::NounPhrase), l(L::EachDeterminer), n(N::VerbPhrase)],
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
    // The free-standing negation of a contracted copular predication (`it's
    // *not* your turn`). Negation is normally spelled on the copula itself
    // (`isn't`), but when the subject and auxiliary contract there is no
    // auxiliary token left to carry it, so English spells `not` separately.
    // Mirrors `CopularRemainderAdverb`'s shape.
    builder.add(
        RuleTag::CopularRemainderNegated,
        N::CopularRemainder,
        [l(L::Not), n(N::CopularRemainder)],
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

    // Round `exrider`: the closed `except by <PP>` exception tail on a
    // passive restriction (`can't be blocked ... except by creatures with
    // flying`) [CR#508.1c,509.1b,702.9b,702.13b,702.36b,702.111b]. Appended
    // last so every existing alternative's (cost, rule_order) key is
    // preserved. Default cost: the dot-1/`By` gates in
    // `accepts_predicate_prefix`/`reduce_predicate` do the licensing, not
    // parse-cost dispreference.
    builder.add(
        RuleTag::VerbPhraseExceptBy,
        N::VerbPhrase,
        [n(N::VerbPhrase), l(L::Except), n(N::PrepositionalPhrase)],
    );
}
