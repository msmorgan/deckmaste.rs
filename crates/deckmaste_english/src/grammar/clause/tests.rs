use super::lowering::*;
use super::reduction::*;
use super::*;
use crate::catalog::CatalogKind;
use crate::catalog::Catalogs;
use crate::identity::SelfReference;
use crate::syntax::Ability;
use crate::syntax::AbilityKind;
use crate::syntax::AdjectiveComplement;
use crate::syntax::Demonstrative;
use crate::syntax::DependentAttachment;
use crate::syntax::FrequencyBound;
use crate::syntax::FrequencyCount;
use crate::syntax::InfinitiveMarker;
use crate::syntax::KeywordArgument;
use crate::syntax::KeywordCost;
use crate::syntax::NominalComplement;
use crate::syntax::NominalModifier;
use crate::syntax::NounPhrase;
use crate::syntax::OracleText;
use crate::syntax::Paragraph;
use crate::syntax::Phrase;
use crate::syntax::PredicateConjunction;
use crate::syntax::PredicateObject;
use crate::syntax::Preposition;
use crate::syntax::RelativeBody;
use crate::syntax::RelativeMarker;
use crate::syntax::Sentence;
use crate::syntax::SentenceBody;
use crate::syntax::Subject;
use crate::syntax::Subordinator;
use crate::word::Adjective;
use crate::word::Auxiliary;
use crate::word::AuxiliaryInflection;
use crate::word::ColorWord;
use crate::word::Noun;
use crate::word::NounInstanceKind;
use crate::word::Number;
use crate::word::Person;
use crate::word::Tense;
use crate::word::Verb;
use crate::word::VerbSlot;
use crate::word::Vocab;

fn predicate_coordination(sentence: &Sentence) -> &Coordination<PredicateExpression> {
    let SentenceBody::Independent(IndependentClause::Predicated(
        _,
        PredicateExpression::Coordinated(coordination),
    )) = &sentence.body
    else {
        panic!("expected shared-subject predicate coordination: {sentence:#?}");
    };
    coordination
}

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
fn sentence_forms_share_one_ast_and_derive_terminal_punctuation() {
    // Mutation caught: store or guess a punctuation bit instead of routing
    // both admitted surfaces through the generated sentence declaration.
    let punctuated = parse("Draw a card.");
    let bare = parse("Draw a card");
    assert_eq!(punctuated.sentence(), bare.sentence());
    assert_eq!(
        render_sentence(punctuated.sentence().unwrap()),
        "Draw a card."
    );
    assert_eq!(render_sentence(bare.sentence().unwrap()), "Draw a card.");
}

#[test]
fn quote_terminal_sentence_neither_requires_nor_admits_an_outer_period() {
    // Mutation caught: choose the period-consuming sentence form without
    // consulting the terminal quoted-ability structure, producing or
    // accepting doubled punctuation after the closing quote.
    let source = "Enchanted creature has \"{T}: Draw a card.\"";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);

    let doubled = "Enchanted creature has \"{T}: Draw a card.\".";
    assert!(
        parse_nonterminal(doubled, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
        "a quote-terminated sentence must not admit a second outer period",
    );
    assert!(
        parse_nonterminal(
            "if you control a creature",
            &fixture_catalogs(),
            Nonterminal::Sentence,
        )
        .is_err(),
        "a dependent-only clause must not become a Sentence root",
    );
}

#[test]
fn singular_demonstratives_determine_mass_nouns() {
    // Positive: `that`/`this` now determine a mass noun (`that damage`), the
    // demonstrative object of a prevention/replacement imperative.
    let parsed = parse("Prevent that damage.");
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected an imperative transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
        panic!("expected a nominal object, got {:?}", predicate.object);
    };
    assert_eq!(
        nominal.determiner(),
        Some(&crate::determiner::demonstrative(Demonstrative::That))
    );
    assert!(matches!(
        nominal.head().kind(),
        NounInstanceKind::Mass(Noun::Word(Vocab::Damage))
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
fn flip_is_a_count_noun_alongside_its_irregular_verb() {
    // Positive: `the flip` is a nominal condition object inside the
    // `if`-subordinate of a complex clause.
    let parsed = parse("If you win the flip, draw a card.");
    let SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
        attachments, ..
    })) = &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a complex clause with a fronted condition");
    };
    let [attachment] = attachments.as_slice() else {
        panic!("expected exactly one fronted attachment");
    };
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        Subordinator::If,
        SubordinateBody::Finite(condition),
    )) = &attachment.payload
    else {
        panic!(
            "expected an `if` subordinate frame, got {:?}",
            attachment.payload
        );
    };
    let IndependentClause::Transitive(_, predicate) = condition.as_ref() else {
        panic!("expected a transitive condition clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
        panic!("expected a nominal object, got {:?}", predicate.object);
    };
    assert_eq!(nominal.determiner(), Some(&crate::determiner::the()));
    assert!(matches!(
        nominal.head().kind(),
        NounInstanceKind::Singular(Noun::Word(Vocab::Flip))
    ));

    // Morphology: singular `flip`, plural `flips`, and the irregular verb
    // `flipped`/`flipping` coexist.
    assert!(parse_nonterminal("the flips", &fixture_catalogs(), Nonterminal::NounPhrase).is_ok());
    assert!(
        parse_nonterminal(
            "This creature phases out.",
            &fixture_catalogs(),
            Nonterminal::Sentence
        )
        .is_ok()
    );

    // Required negative: `Flip a coin.` remains an imperative transitive
    // with direct object `a coin`; no noun-headed alternative may win.
    let flip_coin = parse("Flip a coin.");
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &flip_coin.sentence().expect("sentence root").body
    else {
        panic!("expected an imperative transitive clause");
    };
    assert!(matches!(predicate.head.verb.verb, Verb::Word(Vocab::Flip)));
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
        panic!("expected a nominal object, got {:?}", predicate.object);
    };
    assert_eq!(nominal.determiner(), Some(&crate::determiner::indefinite()));
    assert!(matches!(
        nominal.head().kind(),
        NounInstanceKind::Singular(Noun::Word(Vocab::Coin))
    ));
}

#[test]
fn ensue_is_a_regular_intransitive_verb() {
    let parsed = parse("Chaos ensues.");
    let SentenceBody::Independent(IndependentClause::Intransitive(_, predicate)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected an intransitive clause");
    };
    assert!(matches!(predicate.head.verb.verb, Verb::Word(Vocab::Ensue)));

    // Reject a direct object after `ensue`.
    assert!(
        parse_nonterminal(
            "Chaos ensues damage.",
            &fixture_catalogs(),
            Nonterminal::Sentence
        )
        .is_err(),
        "ensue must not take a direct object"
    );
}

#[test]
fn coin_result_is_a_closed_two_word_predicate_tail() {
    use crate::syntax::CoinSide;
    use crate::syntax::PredicateElement;

    // `If the coin comes up heads, ...` has head verb `Come` and
    // `PredicateElement::CoinResult(Heads)`.
    let parsed = parse("If the coin comes up heads, draw a card.");
    let SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
        attachments, ..
    })) = &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a complex clause with a fronted condition");
    };
    let [attachment] = attachments.as_slice() else {
        panic!("expected exactly one fronted attachment");
    };
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        Subordinator::If,
        SubordinateBody::Finite(condition),
    )) = &attachment.payload
    else {
        panic!(
            "expected an `if` subordinate frame, got {:?}",
            attachment.payload
        );
    };
    let IndependentClause::Intransitive(_, predicate) = condition.as_ref() else {
        panic!("expected an intransitive condition clause");
    };
    assert!(matches!(predicate.head.verb.verb, Verb::Word(Vocab::Come)));
    assert_eq!(
        predicate.elements.as_slice(),
        [PredicateElement::CoinResult(CoinSide::Heads)]
    );

    // `coins came up heads` proves the irregular past; `comes up tails`
    // proves the other side.
    let past = parse("Coins came up heads.");
    let SentenceBody::Independent(IndependentClause::Intransitive(_, past_predicate)) =
        &past.sentence().expect("sentence root").body
    else {
        panic!("expected an intransitive clause");
    };
    assert!(matches!(
        past_predicate.head.verb.slot,
        VerbSlot::Past { .. }
    ));
    assert_eq!(
        past_predicate.elements.as_slice(),
        [PredicateElement::CoinResult(CoinSide::Heads)]
    );

    let tails = parse("The coin comes up tails.");
    let SentenceBody::Independent(IndependentClause::Intransitive(_, tails_predicate)) =
        &tails.sentence().expect("sentence root").body
    else {
        panic!("expected an intransitive clause");
    };
    assert_eq!(
        tails_predicate.elements.as_slice(),
        [PredicateElement::CoinResult(CoinSide::Tails)]
    );

    // Reject `comes heads`, `comes up edge`, `phases up heads`, and every
    // general noun/adjective lookup for `heads`/designated `tails`.
    for rejected in [
        "The coin comes heads.",
        "The coin comes up edge.",
        "This creature phases up heads.",
    ] {
        assert!(
            parse_nonterminal(rejected, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "{rejected:?} must not parse as a complete sentence"
        );
    }

    // Existing `This creature phases out.` remains the same particle
    // tree.
    let phases_out = parse("This creature phases out.");
    let SentenceBody::Independent(IndependentClause::Intransitive(_, phase_predicate)) =
        &phases_out.sentence().expect("sentence root").body
    else {
        panic!("expected an intransitive clause");
    };
    assert_eq!(
        phase_predicate.elements.as_slice(),
        [PredicateElement::Particle(VerbParticle::Out)]
    );

    // Required negative: `Flip a coin.` remains an imperative transitive
    // with object `a coin`; no CoinResult reading intrudes.
    let flip_coin = parse("Flip a coin.");
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
        flip_predicate,
    ))) = &flip_coin.sentence().expect("sentence root").body
    else {
        panic!("expected an imperative transitive clause");
    };
    assert!(flip_predicate.elements.is_empty());

    // Render/reparse both hand-constructed `CoinResult` values and
    // compare the structured subtrees.
    for side in [CoinSide::Heads, CoinSide::Tails] {
        let source = match side {
            CoinSide::Heads => "The coin comes up heads.",
            CoinSide::Tails => "The coin comes up tails.",
        };
        let parsed = parse(source);
        let sentence = parsed.sentence().expect("sentence root");
        let rendered = render_sentence(sentence);
        assert_eq!(rendered, source);
        let reparsed = parse(&rendered);
        assert_eq!(
            reparsed.sentence().expect("sentence root").body,
            sentence.body
        );
    }
}

#[test]
fn while_fronts_a_gerund_clause_before_the_matrix() {
    // `While voting, you may vote an additional time.` lowers to a
    // before-matrix `Subordinator::While` attachment with
    // `SubordinateBody::Gerund`.
    let parsed = parse("While voting, you may vote an additional time.");
    let SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
        attachments, ..
    })) = &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a complex clause with a fronted gerund");
    };
    let [attachment] = attachments.as_slice() else {
        panic!("expected exactly one fronted attachment");
    };
    assert_eq!(attachment.position, AttachmentPosition::BeforeMatrix);
    assert!(attachment.comma.is_present());
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        Subordinator::While,
        SubordinateBody::Gerund(_),
    )) = &attachment.payload
    else {
        panic!(
            "expected a `while` gerund frame, got {:?}",
            attachment.payload
        );
    };

    // The Brago `get an additional vote` variant parses.
    parse("While voting, you get an additional vote.");

    // Reject a missing comma on the gerund frame.
    assert!(
        parse_nonterminal(
            "While voting you may vote an additional time.",
            &fixture_catalogs(),
            Nonterminal::Sentence
        )
        .is_err(),
        "a missing comma must not parse"
    );

    // A finite (non-gerund) body after `while` still parses, but through
    // the pre-existing generic `ClauseSubordinateBefore` production, not
    // the new gerund-only one — confirming the new production didn't
    // widen `while`'s existing finite-clause reading.
    let finite = parse("While you vote, you may vote an additional time.");
    let SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
        attachments: finite_attachments,
        ..
    })) = &finite.sentence().expect("sentence root").body
    else {
        panic!("expected a complex clause");
    };
    let [finite_attachment] = finite_attachments.as_slice() else {
        panic!("expected exactly one attachment");
    };
    assert!(matches!(
        &finite_attachment.payload,
        ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
            Subordinator::While,
            SubordinateBody::Finite(_)
        ))
    ));

    // Existing postposed `This creature attacks while saddled.` keeps
    // its elliptical tree (already covered by FIXTURES, reconfirmed
    // here for this stage).
    assert_eq!(
        render_sentence(
            parse("This creature attacks while saddled.")
                .sentence()
                .unwrap()
        ),
        "This creature attacks while saddled."
    );
}

#[test]
fn receive_is_a_regular_verb_with_a_required_object() {
    let parsed = parse("An opponent received no votes.");
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a transitive clause");
    };
    assert!(matches!(
        predicate.head.verb.verb,
        Verb::Word(Vocab::Receive)
    ));

    // Reject an objectless `received.` under the required-object frame.
    assert!(
        parse_nonterminal(
            "An opponent received.",
            &fixture_catalogs(),
            Nonterminal::Sentence
        )
        .is_err(),
        "receive requires a direct object"
    );
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
    assert!(attachment.comma.is_present());
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        Subordinator::TheNextTime,
        SubordinateBody::Finite(condition),
    )) = &attachment.payload
    else {
        panic!(
            "expected a `the next time` subordinate frame, got {:?}",
            attachment.payload
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
    let [NominalComplement::Prepositional(of)] = number.complements() else {
        panic!("expected an `of` complement: {number:#?}");
    };
    let crate::syntax::Phrase::NounPhrase(object) = of.head().object.as_ref() else {
        panic!("`of` object should be a noun phrase");
    };
    let NounPhrase::Nominal(times) = object.as_ref() else {
        panic!("`of` object should be a `times` nominal");
    };
    assert!(matches!(
        times.complements(),
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
    assert!(attachment.comma.is_present());
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        Subordinator::Where,
        SubordinateBody::Finite(body),
    )) = &attachment.payload
    else {
        panic!(
            "expected a `where` subordinate definition, got {:?}",
            attachment.payload
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
    let source = "Choose up to X target creatures, where X is the number of creatures you control.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
        attachments, ..
    })) = &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a complex clause with a trailing where-definition");
    };
    assert!(attachments.iter().any(|attachment| matches!(
        &attachment.payload,
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
        attachments, ..
    })) = &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a complex clause with a trailing as-though attachment");
    };
    let [attachment] = attachments.as_slice() else {
        panic!("expected exactly one trailing attachment");
    };
    assert_eq!(attachment.position, AttachmentPosition::AfterMatrix);
    assert!(!attachment.comma.is_present());
    assert!(matches!(
        &attachment.payload,
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
    // generated sentence feature gate requires
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
    let source = "Until end of turn, you may spend white mana as though it were mana of any color.";
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
    // generated `verb_phrase_infinitive` (the subordinate clause linearly
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
    )) = &attachment.payload
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
    let [NominalComplement::Prepositional(of_any_color)] = mana.complements() else {
        panic!("expected `mana` to take one prepositional complement: {mana:#?}");
    };
    let crate::syntax::Phrase::NounPhrase(any_color_np) = of_any_color.head().object.as_ref()
    else {
        panic!("expected a noun-phrase object of `of`: {of_any_color:#?}");
    };
    let NounPhrase::Nominal(any_color) = any_color_np.as_ref() else {
        panic!("expected a nominal object of `of`: {any_color_np:#?}");
    };
    assert!(
        matches!(
            any_color.complements(),
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
    )) = &attachment.payload
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
    assert!(!predicate.copula.contracted_with_subject.is_contracted());
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
            )) = &attachment.payload
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
    let source = "This token saddles Mounts and crews Vehicles as though its power were 2 greater.";
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
    )) = &attachment.payload
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
        adjective.degree().copied(),
        Some(NumberLiteral {
            value: 2,
            numeral: Numeral::Arabic(false),
        })
    );
    assert_eq!(
        adjective.head(),
        &crate::word::Adjective::Word(Vocab::Greater)
    );
    assert!(adjective.complements().is_empty());
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
    let [crate::syntax::NominalModifier::Adjective { phrase, .. }] = nominal.modifiers() else {
        panic!("expected a single adjective modifier: {nominal:#?}");
    };
    assert!(
        phrase.degree().is_none(),
        "the `other` modifier must not carry a degree measure: {phrase:#?}"
    );
}

#[test]
fn numeral_before_an_or_comparative_stays_attributive() {
    // NB2: the shapes an `OrComparative`-only guard would still swallow.
    // These are already resolved at baseline (attributive quantity +
    // adjective modifier, an existing pathway unrelated to
    // `AdjectiveComparisonState`); this round must not divert them
    // through the new degree-measure production.
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
        let sentence = parsed.sentence().expect(source);
        let nominal = match &sentence.body {
            SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(
                predicate,
            ))) => predicate.elements.iter().find_map(|element| match element {
                PredicateElement::Adjunct(PredicateAdjunct::Temporal(NounPhrase::Nominal(
                    nominal,
                ))) => Some(nominal),
                _ => None,
            }),
            SentenceBody::Independent(IndependentClause::Deontic(
                _,
                _,
                Some(Predicate::Transitive(predicate)),
            )) => match &predicate.object {
                PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) => Some(nominal),
                _ => None,
            },
            SentenceBody::Independent(IndependentClause::Complex(complex)) => {
                let IndependentClause::Intransitive(_, predicate) = complex.matrix.as_ref() else {
                    panic!("expected an intransitive matrix: {sentence:#?}");
                };
                predicate.elements.iter().find_map(|element| match element {
                    PredicateElement::Adjunct(PredicateAdjunct::Prepositional(
                        PrepositionalPhrase::Simple(crate::syntax::SimplePrepositionalPhrase {
                            object,
                            ..
                        }),
                    )) => match object.as_ref() {
                        Phrase::NounPhrase(noun) => match noun.as_ref() {
                            NounPhrase::Nominal(nominal) => Some(nominal),
                            _ => None,
                        },
                        _ => None,
                    },
                    _ => None,
                })
            }
            _ => None,
        }
        .unwrap_or_else(|| panic!("expected the quantified comparative nominal: {sentence:#?}"));
        let comparative = nominal
            .modifiers()
            .iter()
            .find_map(|modifier| match modifier {
                NominalModifier::Adjective { phrase, .. }
                    if matches!(phrase.head(), Adjective::Word(Vocab::More | Vocab::Fewer)) =>
                {
                    Some(phrase)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("expected an attributive comparative: {nominal:#?}"));
        assert!(
            comparative.degree().is_none(),
            "{source} must not carry a degree measure: {comparative:#?}"
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
            if matches!(nominal.head().kind(), NounInstanceKind::Plural(Noun::Word(Vocab::Spell)))
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
                    nominal.determiner(),
                    Some(determiner)
                        if matches!(
                            determiner.kind(),
                            crate::syntax::DeterminerKind::Quantity(quantity)
                                if matches!(quantity.kind(), crate::syntax::QuantityKind::Exact(_))
                        )
                ) && matches!(
                    nominal.head().kind(),
                    NounInstanceKind::Mass(Noun::Word(word)) if *word == expected
                )
        ));
    }
}

#[test]
fn bonfire_recipient_is_full_coordination_with_a_shared_target_member() {
    let source = "This card deals X damage to target player or planeswalker and each creature that player or that planeswalker's controller controls.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(damage)) = &predicate.object else {
        panic!("expected a nominal damage object: {:#?}", predicate.object);
    };
    assert!(matches!(
        damage.head().kind(),
        NounInstanceKind::Mass(Noun::Word(Vocab::Damage))
    ));
    let [NominalComplement::Prepositional(recipient)] = damage.complements() else {
        panic!("damage must own exactly one recipient PP: {damage:#?}");
    };
    assert_eq!(recipient.head().preposition, Preposition::To);
    let Phrase::NounPhrase(recipient) = recipient.head().object.as_ref() else {
        panic!("expected a noun-phrase recipient: {recipient:#?}");
    };
    let NounPhrase::Coordinated(recipient) = recipient.as_ref() else {
        panic!("expected full recipient coordination: {recipient:#?}");
    };
    let NounPhrase::CoordinatedNominal(targets) = recipient.first().as_ref() else {
        panic!("expected one shared-target first member: {recipient:#?}");
    };
    assert_eq!(*targets.determiner(), crate::determiner::target(None));
    assert!(targets.first().determiner().is_none());
    assert!(matches!(
        targets.rest().as_slice(),
        [crate::syntax::NominalPhraseCoordination {
            conjunction: Some(crate::syntax::NounPhraseConjunction::Or),
            phrase,
            ..
        }] if phrase.determiner().is_none()
    ));
    let [creatures] = recipient.rest().as_slice() else {
        panic!("expected one outer coordination member: {recipient:#?}");
    };
    assert_eq!(
        creatures.conjunction,
        Some(crate::syntax::NounPhraseConjunction::And)
    );
    let NounPhrase::Nominal(creatures) = &creatures.phrase else {
        panic!("expected an independently determined creature member");
    };
    assert_eq!(creatures.determiner(), Some(&crate::determiner::each()));
    assert!(matches!(
        creatures.complements(),
        [NominalComplement::Relative(RelativeClause {
            marker: RelativeMarker::Zero,
            gap: RelativeGap::Object,
            ..
        })]
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn adversarial_shared_subject_keeps_damage_and_recipient_coordinations_nested() {
    let source = "That creature gets +1/+1 and deals 3 damage to target player or planeswalker and 1 damage to each creature that player or that planeswalker controls.";
    let parsed = parse(source);
    let predicates = predicate_coordination(parsed.sentence().expect("sentence root"));
    let [_, PredicateExpression::Simple(Predicate::Transitive(deals))] = predicates.conjuncts()
    else {
        panic!("expected coordinated `gets` and `deals` predicates: {predicates:#?}");
    };
    let PredicateObject::NounPhrase(NounPhrase::Coordinated(damage)) = &deals.object else {
        panic!("expected two independently quantified damage nominals: {deals:#?}");
    };
    let NounPhrase::Nominal(first_damage) = damage.first().as_ref() else {
        panic!("expected the first damage nominal: {damage:#?}");
    };
    let [NominalComplement::Prepositional(first_recipient)] = first_damage.complements() else {
        panic!("the first damage needs one recipient PP: {first_damage:#?}");
    };
    let Phrase::NounPhrase(first_recipient) = first_recipient.head().object.as_ref() else {
        panic!("expected a noun-phrase first recipient: {first_recipient:#?}");
    };
    assert!(matches!(
        first_recipient.as_ref(),
        NounPhrase::CoordinatedNominal(targets)
            if *targets.determiner() == crate::determiner::target(None) && targets.rest().len() == 1
    ));

    let [second_damage] = damage.rest().as_slice() else {
        panic!("expected exactly one later damage nominal: {damage:#?}");
    };
    assert_eq!(
        second_damage.conjunction,
        Some(crate::syntax::NounPhraseConjunction::And)
    );
    let NounPhrase::Nominal(second_damage) = &second_damage.phrase else {
        panic!("expected the second damage nominal: {second_damage:#?}");
    };
    let [NominalComplement::Prepositional(second_recipient)] = second_damage.complements() else {
        panic!("the second damage needs one recipient PP: {second_damage:#?}");
    };
    let Phrase::NounPhrase(second_recipient) = second_recipient.head().object.as_ref() else {
        panic!("expected a noun-phrase second recipient: {second_recipient:#?}");
    };
    assert!(matches!(
        second_recipient.as_ref(),
        NounPhrase::Nominal(creatures)
            if creatures.determiner() == Some(&crate::determiner::each())
                && matches!(
                    creatures.complements(),
                    [NominalComplement::Relative(_)]
                )
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn rules_object_gap_skips_a_mass_comparison_head() {
    let source = "the greatest toughness among creatures you control";
    let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
        .expect("comparison-set nominal should parse");
    let Some(NounPhrase::Nominal(toughness)) = parsed.noun_phrase() else {
        panic!("expected a toughness nominal");
    };
    let [NominalComplement::Prepositional(set)] = toughness.complements() else {
        panic!("expected one comparison-set PP: {toughness:#?}");
    };
    assert_eq!(set.head().preposition, Preposition::Among);
    let Phrase::NounPhrase(object) = set.head().object.as_ref() else {
        panic!("expected a nominal comparison-set object: {set:#?}");
    };
    let NounPhrase::Nominal(object) = object.as_ref() else {
        panic!("expected a nominal comparison-set object: {object:#?}");
    };
    assert!(
        matches!(
            object.complements(),
            [NominalComplement::Relative(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: RelativeGap::Object,
                ..
            })]
        ),
        "the comparison-set member must host the relative: {object:#?}"
    );
}

#[test]
fn subject_gap_attachment_remains_governed_by_agreement() {
    let source = "the toughness among creatures that is equal to 2";
    let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
        .expect("singular relative should attach to singular toughness");
    let Some(NounPhrase::Nominal(toughness)) = parsed.noun_phrase() else {
        panic!("expected a toughness nominal");
    };
    assert!(
        matches!(
            toughness.complements(),
            [
                NominalComplement::Prepositional(PrepositionalPhrase::Simple(
                    crate::syntax::SimplePrepositionalPhrase {
                        preposition: Preposition::Among,
                        ..
                    }
                )),
                NominalComplement::Relative(RelativeClause {
                    marker: RelativeMarker::That,
                    gap: RelativeGap::Subject,
                    ..
                }),
                ..
            ]
        ),
        "agreement must keep the relative on the outer singular head: {toughness:#?}"
    );
}

#[test]
fn rules_object_gap_reaches_the_deepest_nested_pp_host() {
    let source = "damage to each player equal to the number of lands they control";
    let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
        .expect("nested damage nominal should parse");
    let Some(NounPhrase::Nominal(damage)) = parsed.noun_phrase() else {
        panic!("expected a damage nominal");
    };
    assert!(
        !damage
            .complements()
            .iter()
            .any(|complement| matches!(complement, NominalComplement::Relative(_))),
        "the rules-object gap cannot remain on mass damage: {damage:#?}"
    );
    let Some(lands) = damage.complements().iter().find_map(|complement| {
        let NominalComplement::Prepositional(phrase) = complement else {
            return None;
        };
        let Phrase::NounPhrase(object) = phrase.head().object.as_ref() else {
            return None;
        };
        let NounPhrase::Nominal(object) = object.as_ref() else {
            return None;
        };
        matches!(object.head().kind(), NounInstanceKind::Plural(_)).then_some(object)
    }) else {
        panic!("expected a plural lands host below the damage PPs: {damage:#?}");
    };
    assert!(matches!(
        lands.complements(),
        [NominalComplement::Relative(RelativeClause {
            marker: RelativeMarker::Zero,
            gap: RelativeGap::Object,
            ..
        })]
    ));
}

#[test]
fn rules_object_gap_ranks_valid_hosts_across_nested_pps() {
    let source = "spells from among cards in exile your opponents own";
    let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
        .expect("nested ownership nominal should parse");
    let Some(NounPhrase::Nominal(spells)) = parsed.noun_phrase() else {
        panic!("expected a spells nominal");
    };
    let [NominalComplement::Prepositional(from)] = spells.complements() else {
        panic!("ownership relative must not remain on spells: {spells:#?}");
    };
    let Phrase::PrepositionalPhrase(among) = from.head().object.as_ref() else {
        panic!("expected nested `from among` PPs: {from:#?}");
    };
    let Phrase::NounPhrase(cards) = among.head().object.as_ref() else {
        panic!("expected cards inside `among`: {among:#?}");
    };
    let NounPhrase::Nominal(cards) = cards.as_ref() else {
        panic!("expected a cards nominal: {cards:#?}");
    };
    assert!(matches!(
        cards.complements(),
        [
            NominalComplement::Prepositional(PrepositionalPhrase::Simple(
                crate::syntax::SimplePrepositionalPhrase {
                    preposition: Preposition::In,
                    ..
                }
            )),
            NominalComplement::Relative(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: RelativeGap::Object,
                ..
            })
        ]
    ));
}

#[test]
fn invalid_mass_pp_object_falls_back_to_the_outer_count_host() {
    let source = "target creature with flying an opponent controls";
    let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
        .expect("selectionally constrained outer relative should parse");
    let Some(NounPhrase::Nominal(creature)) = parsed.noun_phrase() else {
        panic!("expected a creature nominal");
    };
    assert!(matches!(
        creature.complements(),
        [
            NominalComplement::Prepositional(PrepositionalPhrase::Simple(
                crate::syntax::SimplePrepositionalPhrase {
                    preposition: Preposition::With,
                    ..
                }
            )),
            NominalComplement::Relative(RelativeClause {
                marker: RelativeMarker::Zero,
                gap: RelativeGap::Object,
                ..
            })
        ]
    ));
}

#[test]
fn rules_object_gap_keeps_coordinated_members_inside_the_preposition() {
    let source = "damage to you and creatures you control";
    let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
        .expect("coordinated recipient nominal should parse");
    let Some(NounPhrase::Nominal(damage)) = parsed.noun_phrase() else {
        panic!("expected a damage nominal");
    };
    let [NominalComplement::Prepositional(recipient)] = damage.complements() else {
        panic!("damage must retain one coordinated recipient PP: {damage:#?}");
    };
    assert_eq!(recipient.head().preposition, Preposition::To);
    let Phrase::NounPhrase(recipient) = recipient.head().object.as_ref() else {
        panic!("expected a noun-phrase recipient: {recipient:#?}");
    };
    let NounPhrase::Coordinated(recipient) = recipient.as_ref() else {
        panic!("expected recipient coordination inside `to`: {recipient:#?}");
    };
    assert!(matches!(
        recipient.first().as_ref(),
        NounPhrase::Pronoun {
            pronoun: crate::word::Pronoun::You,
            ..
        }
    ));
    let [creatures] = recipient.rest().as_slice() else {
        panic!("expected one coordinated creature member: {recipient:#?}");
    };
    let NounPhrase::Nominal(creatures) = &creatures.phrase else {
        panic!("expected a nominal creature member: {creatures:#?}");
    };
    assert!(matches!(
        creatures.complements(),
        [NominalComplement::Relative(RelativeClause {
            marker: RelativeMarker::Zero,
            gap: RelativeGap::Object,
            ..
        })]
    ));
}

#[test]
fn rules_object_gap_prefers_the_nearest_preposition_without_stealing_its_subject() {
    let source = "control of all artifacts and creatures target opponent controls";
    let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
        .expect("coordinated control nominal should parse");
    let Some(NounPhrase::Nominal(control)) = parsed.noun_phrase() else {
        panic!("expected a control nominal");
    };
    let [NominalComplement::Prepositional(objects)] = control.complements() else {
        panic!("control must retain one coordinated `of` PP: {control:#?}");
    };
    assert_eq!(objects.head().preposition, Preposition::Of);
    let Phrase::NounPhrase(objects) = objects.head().object.as_ref() else {
        panic!("expected noun-phrase objects: {objects:#?}");
    };
    let NounPhrase::CoordinatedNominal(objects) = objects.as_ref() else {
        panic!("expected shared-determiner coordination inside `of`: {objects:#?}");
    };
    let [creatures] = objects.rest().as_slice() else {
        panic!("expected one creature member: {objects:#?}");
    };
    assert!(creatures.phrase.complements().is_empty());
    let [NominalComplement::Relative(relative)] = objects.complements().as_slice() else {
        panic!("the coordinated objects must host the relative: {objects:#?}");
    };
    let RelativeBody::ObjectGap {
        subject: Subject(NounPhrase::Nominal(subject)),
        ..
    } = &relative.body
    else {
        panic!("expected an object-gap relative with a nominal subject: {relative:#?}");
    };
    assert_eq!(subject.determiner(), Some(&crate::determiner::target(None)));
    assert!(matches!(
        subject.head().kind(),
        NounInstanceKind::Singular(Noun::Word(Vocab::Opponent))
    ));
}

#[test]
fn coordinated_member_consumes_following_modifiers_before_the_pp_closes() {
    let source = "control of target nonland permanent you control and target permanent an opponent controls that shares a card type with it";
    let parsed = parse_nonterminal(source, &fixture_catalogs(), Nonterminal::NounPhrase)
        .expect("multiply modified coordinated control nominal should parse");
    let Some(NounPhrase::Nominal(control)) = parsed.noun_phrase() else {
        panic!("expected a control nominal");
    };
    let [NominalComplement::Prepositional(objects)] = control.complements() else {
        panic!("control must retain one complete `of` PP: {control:#?}");
    };
    let Phrase::NounPhrase(objects) = objects.head().object.as_ref() else {
        panic!("expected noun-phrase objects: {objects:#?}");
    };
    let NounPhrase::Coordinated(objects) = objects.as_ref() else {
        panic!("expected coordination inside `of`: {objects:#?}");
    };
    let [second] = objects.rest().as_slice() else {
        panic!("expected one second permanent: {objects:#?}");
    };
    let NounPhrase::Nominal(second) = &second.phrase else {
        panic!("expected a nominal second permanent: {second:#?}");
    };
    assert!(matches!(
        second.complements(),
        [
            NominalComplement::Relative(RelativeClause {
                gap: RelativeGap::Object,
                ..
            }),
            NominalComplement::Relative(RelativeClause {
                gap: RelativeGap::Subject,
                ..
            }),
            NominalComplement::Prepositional(PrepositionalPhrase::Simple(
                crate::syntax::SimplePrepositionalPhrase {
                    preposition: Preposition::With,
                    ..
                }
            ))
        ]
    ));
}

#[test]
fn coordinated_member_refuses_unrelated_following_pps() {
    let source = "Gain control of target artifact or creature you control until end of turn.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a transitive imperative");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(control)) = &predicate.object else {
        panic!("expected a nominal control object: {predicate:#?}");
    };
    let [
        NominalComplement::Prepositional(objects),
        NominalComplement::Prepositional(PrepositionalPhrase::Simple(
            crate::syntax::SimplePrepositionalPhrase {
                preposition: Preposition::Until,
                ..
            },
        )),
        NominalComplement::Prepositional(PrepositionalPhrase::Simple(
            crate::syntax::SimplePrepositionalPhrase {
                preposition: Preposition::Of,
                ..
            },
        )),
    ] = control.complements()
    else {
        panic!("the following PPs must remain outside the final member: {control:#?}");
    };
    let Phrase::NounPhrase(objects) = objects.head().object.as_ref() else {
        panic!("expected noun-phrase objects: {objects:#?}");
    };
    let NounPhrase::CoordinatedNominal(objects) = objects.as_ref() else {
        panic!("expected shared-determiner coordination inside `of`: {objects:#?}");
    };
    let [last] = objects.rest().as_slice() else {
        panic!("expected one final creature member: {objects:#?}");
    };
    assert!(last.phrase.complements().is_empty());
    assert!(matches!(
        objects.complements().as_slice(),
        [NominalComplement::Relative(RelativeClause {
            gap: RelativeGap::Object,
            ..
        })]
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn binary_comma_and_does_not_masquerade_as_an_oxford_nominal_list() {
    let source = "Search your library for seven cards, exile them in a face-down pile, and shuffle that pile.";
    let parsed = parse(source);
    let coordination = predicate_coordination(parsed.sentence().expect("sentence root"));
    assert_eq!(
        coordination.conjuncts().len(),
        3,
        "the final `shuffle` predicate must remain a clause member: {coordination:#?}"
    );
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn verb_homograph_does_not_close_a_shared_determiner_group() {
    let source = "Create a token and copy target spell.";
    let parsed = parse(source);
    let coordination = predicate_coordination(parsed.sentence().expect("sentence root"));
    let [_, PredicateExpression::Simple(Predicate::Transitive(copy))] = coordination.conjuncts()
    else {
        panic!("`copy` must remain the second predicate: {coordination:#?}");
    };
    assert!(matches!(copy.head.verb.verb, Verb::Word(Vocab::Copy)));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn common_head_object_survives_following_finite_clause_coordination() {
    let source = concat!(
        "you may cast target instant or sorcery card from a graveyard, and ",
        "mana of any type can be spent to cast that spell"
    );
    let catalogs = fixture_catalogs().with_catalog(
        CatalogKind::CardType,
        ["Creature", "Instant", "Land", "Planeswalker", "Sorcery"],
    );
    let parsed = parse_nonterminal(source, &catalogs, Nonterminal::Clause)
        .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"));
    let Clause::Independent(IndependentClause::Coordinated(coordination)) =
        parsed.clause().expect("clause root")
    else {
        panic!("expected two complete finite clauses");
    };
    let IndependentClause::Deontic(_, _, Some(first_predicate)) = coordination.first.as_ref()
    else {
        panic!("expected a modal first clause: {coordination:#?}");
    };
    let Predicate::Transitive(cast) = first_predicate else {
        panic!("expected a transitive cast predicate: {first_predicate:#?}");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(card)) = &cast.object else {
        panic!("expected one common-head card object: {cast:#?}");
    };
    assert!(matches!(
        card.modifiers(),
        [NominalModifier::Coordinated(_)]
    ));
    assert!(matches!(
        card.head().kind(),
        NounInstanceKind::Singular(Noun::Word(Vocab::Card))
    ));
    assert!(matches!(
        coordination.rest.as_slice(),
        [ClauseCoordination {
            conjunction: Some(PredicateConjunction::And),
            comma: crate::features::Comma::Present,
            member: CoordinatedClauseMember::Independent(_),
        }]
    ));
}

#[test]
fn later_relative_consumes_its_temporal_adjunct_before_the_pp_closes() {
    let source = "Put a counter on each creature or permanent you control that entered the battlefield this turn.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a transitive imperative");
    };
    assert!(
        predicate.elements.is_empty(),
        "the temporal adjunct belongs to the later relative: {predicate:#?}"
    );
    let PredicateObject::NounPhrase(NounPhrase::Nominal(counter)) = &predicate.object else {
        panic!("expected a counter nominal: {predicate:#?}");
    };
    let [NominalComplement::Prepositional(objects)] = counter.complements() else {
        panic!("counter must retain one complete `on` PP: {counter:#?}");
    };
    let Phrase::NounPhrase(objects) = objects.head().object.as_ref() else {
        panic!("expected noun-phrase counter recipients: {objects:#?}");
    };
    let NounPhrase::CoordinatedNominal(objects) = objects.as_ref() else {
        panic!("expected shared-determiner counter recipients: {objects:#?}");
    };
    let [last] = objects.rest().as_slice() else {
        panic!("expected one final permanent member: {objects:#?}");
    };
    assert!(last.phrase.complements().is_empty());
    let [
        NominalComplement::Relative(RelativeClause {
            gap: RelativeGap::Object,
            ..
        }),
        NominalComplement::Relative(RelativeClause {
            gap: RelativeGap::Subject,
            body: RelativeBody::SubjectGap(Predicate::Transitive(entered)),
            ..
        }),
    ] = objects.complements().as_slice()
    else {
        panic!("the coordinated recipients must retain both relatives: {objects:#?}");
    };
    assert!(matches!(
        entered.elements.as_slice(),
        [PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))]
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn repeated_damage_themes_coordinate_as_complete_noun_phrases() {
    let source = "This card deals 2 damage to each attacking creature and 1 damage to you and each creature you control.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Coordinated(objects)) = &predicate.object else {
        panic!(
            "expected coordinated damage objects: {:#?}",
            predicate.object
        );
    };
    assert!(matches!(
        objects.first().as_ref(),
        NounPhrase::Nominal(first)
            if matches!(first.head().kind(), NounInstanceKind::Mass(Noun::Word(Vocab::Damage)))
                && matches!(first.complements(), [NominalComplement::Prepositional(_)])
    ));
    assert!(matches!(
        objects.rest().as_slice(),
        [crate::syntax::NounPhraseCoordination {
            conjunction: Some(crate::syntax::NounPhraseConjunction::And),
            phrase: NounPhrase::Nominal(second),
            ..
        }] if matches!(second.head().kind(), NounInstanceKind::Mass(Noun::Word(Vocab::Damage)))
            && matches!(second.complements(), [NominalComplement::Prepositional(_)])
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn passive_to_shared_target_stays_inside_the_relative_predicate() {
    let source =
        "Prevent all damage that would be dealt to target player or planeswalker this turn.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a transitive imperative");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(damage)) = &predicate.object else {
        panic!("expected a nominal damage object");
    };
    let [
        NominalComplement::Relative(RelativeClause {
            body:
                RelativeBody::SubjectGap(Predicate::Deontic(DeonticPredicate {
                    inner: Some(inner),
                    ..
                })),
            ..
        }),
    ] = damage.complements()
    else {
        panic!("damage must own one complete relative: {damage:#?}");
    };
    let Predicate::Passive(passive) = inner.as_ref() else {
        panic!("expected a passive predicate inside the relative: {inner:#?}");
    };
    let [
        PredicateElement::Adjunct(PredicateAdjunct::Prepositional(to)),
        PredicateElement::Adjunct(PredicateAdjunct::Temporal(_)),
    ] = passive.elements.as_slice()
    else {
        panic!("the passive must retain its recipient and temporal adjunct: {passive:#?}");
    };
    assert_eq!(to.head().preposition, Preposition::To);
    assert!(matches!(
        to.head().object.as_ref(),
        Phrase::NounPhrase(targets)
            if matches!(targets.as_ref(), NounPhrase::CoordinatedNominal(_))
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
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
            ] if preposition.head().preposition == crate::syntax::Preposition::As
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
    let IndependentClause::Imperative(Predicate::Intransitive(predicate)) = complex.matrix.as_ref()
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
            &complex.attachments[0].payload,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(Subordinator::Before, _))
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
    let IndependentClause::Imperative(Predicate::Intransitive(predicate)) = complex.matrix.as_ref()
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
    assert!(!attachment.comma.is_present());
    let ClauseAttachmentKind::Restriction(run) = &attachment.payload else {
        panic!("expected a Restriction attachment: {attachment:#?}");
    };
    assert!(matches!(
        run.first.as_slice(),
        [PredicateAdjunct::Prepositional(_)]
    ));
    assert_eq!(run.rest.len(), 1);
    assert_eq!(run.rest[0].conjunction, Some(PredicateConjunction::And));
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
    let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].payload else {
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
    let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].payload else {
        panic!(
            "expected a Restriction attachment: {:#?}",
            complex.attachments[0]
        );
    };
    assert_eq!(run.rest.len(), 2);
    assert_eq!(run.rest[0].conjunction, None);
    assert_eq!(run.rest[1].conjunction, Some(PredicateConjunction::And));
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
                complex.attachments.last().unwrap().payload,
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
            predicate
                .elements
                .iter()
                .all(|element| matches!(element, PredicateElement::Adjunct(_))),
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
    let IndependentClause::Imperative(Predicate::Intransitive(predicate)) = complex.matrix.as_ref()
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
    assert!(!complex.attachments[0].comma.is_present());
    assert!(matches!(
        &complex.attachments[0].payload,
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
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &extra.sentence().expect("extra turn").body
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
    // Causal pair on the `after` preposition: a trailing complement of the
    // turn nominal, and a fronted clause adjunct on an existential.
    let trailing = parse("Take an extra turn after this one.");
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &trailing.sentence().expect("trailing").body
    else {
        panic!(
            "expected an imperative transitive: {:#?}",
            trailing.sentence()
        );
    };
    assert!(
        matches!(
            &predicate.object,
            PredicateObject::NounPhrase(NounPhrase::Nominal(nominal))
                if nominal.complements().iter().any(|complement| matches!(
                    complement,
                    NominalComplement::Prepositional(PrepositionalPhrase::Simple(crate::syntax::SimplePrepositionalPhrase {
                        preposition: Preposition::After,
                        ..
                    }))
                ))
        ),
        "trailing `after this one` should complement the turn nominal: {predicate:#?}"
    );

    let fronted = parse("After this phase, there is an additional combat phase.");
    let SentenceBody::Independent(IndependentClause::Complex(complex)) =
        &fronted.sentence().expect("fronted").body
    else {
        panic!("expected a complex clause: {:#?}", fronted.sentence());
    };
    assert!(
        complex.attachments.iter().any(|attachment| matches!(
            attachment,
            ClauseAttachment {
                position: AttachmentPosition::BeforeMatrix,
                payload: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(
                    PrepositionalPhrase::Simple(crate::syntax::SimplePrepositionalPhrase {
                        preposition: Preposition::After,
                        ..
                    })
                )),
                ..
            }
        )),
        "fronted `After this phase` should carry the After preposition: {complex:#?}"
    );
}

#[test]
fn draw_noun_sense_does_not_capture_the_imperative_draw_verb() {
    // Negative armor for the `draw` noun addition: `Draw a card.` stays an
    // imperative headed by the `draw` verb, never a bare `draw` nominal.
    let parsed = parse("Draw a card.");
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("draw a card").body
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
            SentenceBody::Independent(IndependentClause::Imperative(Predicate::Intransitive(
                predicate,
            ))) => &predicate.elements,
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
        let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(choose))) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected an imperative choice: {:#?}", parsed.sentence());
        };
        let PredicateObject::NounPhrase(NounPhrase::Nominal(opponent)) = &choose.object else {
            panic!("expected an opponent object: {:#?}", choose.object);
        };
        let [NominalComplement::Relative(relative)] = opponent.complements() else {
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
            .modifiers()
            .iter()
            .find_map(|modifier| match modifier {
                NominalModifier::Adjective {
                    phrase: adjective, ..
                } if matches!(
                    adjective.head(),
                    Adjective::Word(word) if word.spelling() == expected_adjective
                ) =>
                {
                    Some(adjective)
                }
                _ => None,
            })
            .unwrap_or_else(|| panic!("missing {expected_adjective:?} modifier: {cards:#?}"));
        let [AdjectiveComplement::PostnominalComparison(comparison)] = adjective.complements()
        else {
            panic!("expected a comparison complement: {adjective:#?}");
        };
        assert!(matches!(
            comparison.standard(),
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
                    if preposition.head().preposition == crate::syntax::Preposition::During
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
        ))] if matches!(turn.head().kind(), NounInstanceKind::Singular(Noun::Word(Vocab::Turn)))
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
            ] if matches!(time.head().kind(), NounInstanceKind::Singular(Noun::Word(Vocab::Time)))
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
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a transitive imperative: {:#?}", parsed.sentence());
    };
    assert!(matches!(
        predicate.object,
        PredicateObject::NounPhrase(NounPhrase::Nominal(ref number))
            if matches!(number.head().kind(), NounInstanceKind::Singular(Noun::Word(Vocab::Number)))
    ));
    assert!(matches!(
        predicate.pre_object_elements.as_slice(),
        [PredicateElement::Complement(PredicateComplement::IndirectObject(
            NounPhrase::Nominal(player),
        ))] if matches!(player.head().kind(), NounInstanceKind::Singular(Noun::Word(Vocab::Player)))
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
                    payload: ClauseAttachmentKind::Dependent(
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
            comma: crate::features::Comma::Present,
            payload: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(preposition)),
        },
    ] = complex.attachments.as_slice()
    else {
        panic!("expected one fronted prepositional adjunct: {complex:#?}");
    };
    assert_eq!(
        preposition.head().preposition,
        crate::syntax::Preposition::As
    );
    let crate::syntax::Phrase::NounPhrase(noun_phrase) = preposition.head().object.as_ref() else {
        panic!("expected the adjunct to modify a cost noun phrase: {preposition:#?}");
    };
    let NounPhrase::Nominal(nominal) = noun_phrase.as_ref() else {
        panic!("expected a nominal cost phrase: {noun_phrase:#?}");
    };
    assert!(matches!(
        nominal.head().kind(),
        NounInstanceKind::Singular(Noun::Word(Vocab::Cost))
    ));
    assert!(matches!(
        nominal.complements(),
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
            payload:
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
        ))] if matches!(way.head().kind(), NounInstanceKind::Singular(Noun::Word(Vocab::Way)))
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
            comma: crate::features::Comma::Absent,
            payload: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Then)),
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
            comma: crate::features::Comma::Present,
            payload: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Otherwise)),
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
            comma: crate::features::Comma::Present,
            payload: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Otherwise)),
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
            comma: crate::features::Comma::Present,
            payload: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Otherwise)),
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
    assert!(
        comma.is_present(),
        "expected a comma on the otherwise attachment"
    );
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
    // (The witness must genuinely fail to reduce as a clause; it no
    // longer relies on the shared-deontic modal-coordination gap fixed
    // in Stage A, which now resolves that shape.)
    let source = "Otherwise, it has base power and toughness 1/1 and can't block except.";
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
            comma: crate::features::Comma::Absent,
            payload: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Adverb(Vocab::Then)),
        }]
    ));
}

#[test]
fn for_as_long_as_is_one_finite_subordinator() {
    let parsed = parse("Gain control of target creature for as long as you control this artifact.");
    assert!(matches!(
        &parsed.sentence().expect("sentence root").body,
        SentenceBody::Independent(IndependentClause::Complex(ComplexClause {
            attachments,
            ..
        })) if matches!(
            attachments.as_slice(),
            [ClauseAttachment {
                position: AttachmentPosition::AfterMatrix,
                payload: ClauseAttachmentKind::Dependent(
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
                payload: ClauseAttachmentKind::Dependent(
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
                payload: ClauseAttachmentKind::Dependent(
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
        subject.modifiers(),
        [NominalModifier::Adjective { phrase: adjective, .. }]
            if matches!(
                adjective.head(),
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
    for (source, contraction) in [
        ("you are the monarch", crate::features::Contraction::Full),
        (
            "you're the monarch",
            crate::features::Contraction::Contracted,
        ),
    ] {
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
        assert_eq!(predicate.copula.contracted_with_subject, contraction);
        assert!(matches!(
            predicate.complement,
            crate::syntax::CopularComplement::NounPhrase(_)
        ));
    }
}

#[test]
fn copular_adverbs_precede_the_complement() {
    for (source, contraction) in [
        ("It is still a land.", crate::features::Contraction::Full),
        (
            "It's still a land.",
            crate::features::Contraction::Contracted,
        ),
    ] {
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
        assert_eq!(predicate.copula.contracted_with_subject, contraction);
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
fn contracted_copular_predications_carry_a_free_standing_negation() {
    // Causal pairs. Negation is normally spelled on the copula (`isn't`);
    // when the subject and auxiliary contract there is no auxiliary
    // token left to carry it, so English spells `not` separately.
    for (source, negated) in [
        ("It's not your turn.", true),
        ("It's your turn.", false),
        ("It's not historic.", true),
        ("It's historic.", false),
        ("It's not a token.", true),
        ("It's a token.", false),
        ("That's not historic.", true),
        ("They're not historic.", true),
        ("You're not the monarch.", true),
    ] {
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        let SentenceBody::Independent(IndependentClause::Copular(_subject, predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a copular clause: {:#?}", parsed.sentence());
        };
        assert_eq!(predicate.negated, negated, "{source}");
        assert!(
            predicate.copula.contracted_with_subject.is_contracted(),
            "{source}"
        );
        assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    }
}

#[test]
fn negated_copular_predications_compose_with_precomplement_adverbs() {
    // Both orders parse and carry the same fields — negation and the
    // adverb are independent flags/lists, not positionally tracked — but
    // the renderer's fixed linear order (`not` before adverbs, §2.3)
    // canonicalizes on render, so only the `not`-first source round-trips
    // to itself byte-for-byte.
    for (source, round_trips) in [
        ("It's not still historic.", true),
        ("It's still not historic.", false),
    ] {
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        let SentenceBody::Independent(IndependentClause::Copular(_subject, predicate)) =
            &parsed.sentence().expect("sentence root").body
        else {
            panic!("expected a copular clause: {:#?}", parsed.sentence());
        };
        assert!(predicate.negated, "{source}");
        assert_eq!(predicate.precomplement_adverbs.len(), 1, "{source}");
        assert_eq!(predicate.precomplement_adverbs[0].spelling(), "still");
        let rendered = render_sentence(parsed.sentence().unwrap());
        if round_trips {
            assert_eq!(rendered, source);
        } else {
            assert_eq!(rendered, "It's not still historic.");
        }
    }
}

#[test]
fn negated_copular_renderer_inverse_orders_not_before_each() {
    // Renderer inverse: `not` after the copula, before `each`/adverbs/complement.
    let source = "They're not each equal to 3.";
    let parsed = parse(source);
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
    let SentenceBody::Independent(IndependentClause::Copular(_subject, predicate)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a copular clause: {:#?}", parsed.sentence());
    };
    assert!(predicate.negated, "{source}");
    assert!(predicate.distributive_each, "{source}");
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn double_negation_on_a_copular_predication_does_not_parse() {
    // Double negation is rejected: `[Not, [Not, remainder]]` must not lower.
    let catalogs = fixture_catalogs();
    assert!(parse_nonterminal("it's not not legendary", &catalogs, Nonterminal::Clause).is_err());
}

#[test]
fn negation_spellings_do_not_both_fire_on_one_predication() {
    let source = "It isn't your turn.";
    let parsed = parse(source);
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
    let SentenceBody::Independent(IndependentClause::Copular(_subject, predicate)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a copular clause: {:#?}", parsed.sentence());
    };
    assert!(
        predicate
            .copula
            .auxiliary
            .contracted_negation
            .is_contracted(),
        "{source}"
    );
    assert!(
        !predicate.copula.contracted_with_subject.is_contracted(),
        "{source}"
    );
    assert!(!predicate.negated, "{source}");
}

#[test]
fn relative_contracted_copular_negation_remains_unresolved() {
    // The relative family (`RelativeContractedCopular{Noun,Adjective,
    // Prepositional}`) inlines its complement instead of factoring
    // through `N::CopularRemainder`, so it does not inherit the new
    // negation slot. This is deliberate (see the plan's residue ticket),
    // not accidental: pin that `that's not historic` still fails inside
    // a relative clause.
    let catalogs = fixture_catalogs();
    assert!(
        parse_nonterminal(
            "each permanent that's not historic",
            &catalogs,
            Nonterminal::NounPhrase,
        )
        .is_err()
    );
}

#[test]
fn contracted_subject_auxiliaries_are_structural() {
    let catalogs = fixture_catalogs();

    let parsed = parse_nonterminal("each spell you've cast", &catalogs, Nonterminal::NounPhrase)
        .expect("perfect relative clause should parse");
    let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
        panic!("expected a nominal with a relative clause");
    };
    let [NominalComplement::Relative(relative)] = nominal.complements() else {
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
    assert!(
        predicate
            .head
            .first_auxiliary_contracted_with_subject
            .is_contracted()
    );
    assert!(matches!(
        predicate.head.auxiliaries.as_slice(),
        [auxiliary] if auxiliary.auxiliary == Auxiliary::Have
    ));
    assert_eq!(predicate.head.verb.slot, VerbSlot::PastParticiple);

    let parsed = parse_nonterminal("it's put into exile", &catalogs, Nonterminal::Clause)
        .expect("contracted passive should parse");
    let Some(Clause::Independent(IndependentClause::Passive(subject, predicate))) = parsed.clause()
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
    assert!(
        predicate
            .head
            .first_auxiliary_contracted_with_subject
            .is_contracted()
    );
    assert!(matches!(
        predicate.head.auxiliaries.as_slice(),
        [auxiliary] if auxiliary.auxiliary == Auxiliary::Be
    ));

    let parsed = parse_nonterminal("that's one or more colors", &catalogs, Nonterminal::Clause)
        .expect("contracted demonstrative copula should parse");
    let Some(Clause::Independent(IndependentClause::Copular(subject, predicate))) = parsed.clause()
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
    assert!(predicate.copula.contracted_with_subject.is_contracted());

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
            nominal.complements(),
            [NominalComplement::Relative(RelativeClause {
                marker: RelativeMarker::That,
                gap: RelativeGap::Subject,
                body: RelativeBody::SubjectGap(Predicate::Copular(predicate)),
            })] if predicate.copula.contracted_with_subject.is_contracted()
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
            comma: crate::features::Comma::Absent,
            payload: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
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
                    if preposition.head().preposition == crate::syntax::Preposition::By
            )
        })
    else {
        panic!("expected a by-gerund adjunct: {cast:#?}");
    };
    let crate::syntax::Phrase::Clause(clause) = by.head().object.as_ref() else {
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
            comma: crate::features::Comma::Absent,
            payload: DependentClause::Subordinate(
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
            PredicateComplement::Infinitive(crate::syntax::InfinitiveClause { negated: true, .. })
        )]
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn production_f01_forms_render_exactly_and_report_generated_owners() {
    for (source, required) in [
        ("Spells cost {1} less to cast.", &["infinitive_to"][..]),
        (
            "You may choose not to untap this creature.",
            &["infinitive_not_to"][..],
        ),
        (
            "You may cast that card by paying life equal to the spell's mana value rather than paying its mana cost.",
            &["gerund_clause_base", "gerund_clause_subordinate_after"][..],
        ),
    ] {
        let parsed = parse(source);
        assert_eq!(
            render_sentence(parsed.sentence().expect("sentence root")),
            source
        );
        for id in required {
            let decision = parsed
                .construction_decisions()
                .iter()
                .find(|decision| decision.selected().as_str() == *id)
                .unwrap_or_else(|| panic!("{source:?} did not select {id}"));
            assert_eq!(
                decision.owner(),
                crate::construction::ConstructionOwner::Generated,
                "{id}"
            );
        }
    }
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
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!(
            "expected an imperative turn clause: {:#?}",
            parsed.sentence()
        );
    };
    assert!(matches!(
        predicate.elements.as_slice(),
        [PredicateElement::Complement(PredicateComplement::Adjective(
            phrase
        ))] if phrase.degree().is_none()
            && matches!(phrase.head(), Adjective::CardOrientation(crate::word::CardOrientation::FaceDown))
            && phrase.complements().is_empty()
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
        object.rest().as_slice(),
        [crate::syntax::NounPhraseCoordination {
            conjunction: Some(crate::syntax::NounPhraseConjunction::Plus),
            ..
        }]
    ));
    assert!(parsed.construction_decisions().iter().any(|decision| {
        decision.selected().as_str() == "noun_phrase_coordination"
            && decision.owner() == crate::construction::ConstructionOwner::Generated
    }));
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
        CopularComplement::NounPhrase(NounPhrase::Quantity(quantity))
            if matches!(
                quantity.kind(),
                crate::syntax::QuantityKind::Exact(n) if n.value == 0
            )
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
    let SentenceBody::Independent(body) = &parsed.sentence().expect("sentence root").body else {
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
        ))] if matches!(turn.head().kind(), NounInstanceKind::Singular(Noun::Word(Vocab::Turn)))
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
    let (Subject(NounPhrase::Nominal(target)), _) = finite(target_parse.sentence().unwrap()) else {
        panic!("expected target nominal subject");
    };
    assert_eq!(target.determiner(), Some(&crate::determiner::target(None)));

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
        subject.complements(),
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
        object.complements(),
        [NominalComplement::Relative(relative)]
            if relative.gap == crate::syntax::RelativeGap::Object
    ));
}

#[test]
fn goblin_chieftain_stat_change_remains_one_magic_atom() {
    let sentence = parse("Other Goblin creatures you control get +1/+1 and have haste.");
    let SentenceBody::Independent(IndependentClause::Predicated(
        _,
        PredicateExpression::Coordinated(coordination),
    )) = &sentence.sentence().unwrap().body
    else {
        panic!("expected coordinated predicates");
    };
    let [PredicateExpression::Simple(Predicate::Transitive(first)), _] = coordination.conjuncts()
    else {
        panic!("expected two predicate conjuncts");
    };
    assert!(matches!(first.object, PredicateObject::PowerToughness(_)));
}

#[test]
fn contiguous_oracle_symbols_are_one_scalar_object() {
    let source = "Add {C}{C}.";
    let parsed = parse(source);
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
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
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a transitive imperative: {:#?}", parsed.sentence());
    };
    assert!(matches!(
        &predicate.object,
        PredicateObject::Coordinated(CoordinatedPredicateObject { first, rest })
            if matches!(first.as_ref(), PredicateObject::OracleSymbol(symbol) if symbol.as_str() == "{R}")
                && matches!(rest.as_slice(), [PredicateObjectCoordination {
                    conjunction: Some(crate::syntax::PredicateConjunction::Or),
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
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
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
                        object: PredicateObject::OracleSymbol(b),
                    },
                    PredicateObjectCoordination {
                        conjunction: Some(crate::syntax::PredicateConjunction::Or),
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
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
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
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
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
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &parsed.sentence().expect("sentence root").body
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
    }
    assert_eq!(
        rest[3].conjunction,
        Some(crate::syntax::PredicateConjunction::And)
    );
    // The serial comma is derived from `conjunction`/`rest.len()`, not stored
    // (see `PredicateObjectCoordination`); the render assertion below is what
    // pins every member's comma in this five-member Oxford list.
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
    let source = "Add {B}, then add an additional {B} for each charge counter removed this way.";
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
            payload: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(_, body)),
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
            if matches!(damage.head().kind(), NounInstanceKind::Mass(Noun::Word(Vocab::Damage)))
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
            if matches!(damage.head().kind(), NounInstanceKind::Mass(Noun::Word(Vocab::Damage)))
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
        object.complements(),
        [NominalComplement::Relative(RelativeClause {
            marker: RelativeMarker::That,
            gap: RelativeGap::Subject,
            body: RelativeBody::SubjectGap(Predicate::Passive(passive)),
        })] if passive.retained_object.is_some()
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn postnominal_participial_phrase_reduces_only_with_a_retained_object() {
    let source = "A creature dealt damage this way can't block this turn.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Deontic(
        Subject(NounPhrase::Nominal(subject)),
        _,
        _,
    )) = &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a modal clause with a nominal subject");
    };
    assert!(matches!(
        subject.complements(),
        [NominalComplement::ReducedRecipientPassive(predicate)]
            if predicate.head.auxiliaries.is_empty()
                && predicate.head.verb.slot == VerbSlot::PastParticiple
                && matches!(
                    predicate.object,
                    PredicateObject::NounPhrase(NounPhrase::Nominal(ref damage))
                        if matches!(
                            damage.head().kind(),
                            NounInstanceKind::Mass(Noun::Word(Vocab::Damage))
                        )
                )
    ));
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn reduced_recipient_passive_can_follow_a_completed_relative() {
    let source = "A creature you control dealt damage this way can't block this turn.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Deontic(
        Subject(NounPhrase::Nominal(subject)),
        _,
        _,
    )) = &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a modal clause with a nominal subject");
    };
    assert!(matches!(
        subject.complements(),
        [
            NominalComplement::Relative(_),
            NominalComplement::ReducedRecipientPassive(_),
        ]
    ));
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn postnominal_participial_phrase_does_not_fire_without_the_frame_and_object() {
    for source in ["a creature enchanted this turn", "a creature dealt"] {
        let surface = crate::surface::lex(source);
        let parsed = parse_nonterminal_with_mode(
            source,
            &fixture_catalogs(),
            Nonterminal::NounPhrase,
            &surface.tokens,
            OpacityMode::Exact,
            &SelfReference::default(),
        );
        if let Ok(parsed) = parsed {
            let Some(NounPhrase::Nominal(nominal)) = parsed.noun_phrase() else {
                panic!("expected a nominal noun phrase for {source:?}");
            };
            assert!(
                !nominal.complements().iter().any(|complement| matches!(
                    complement,
                    NominalComplement::ReducedRecipientPassive(_)
                )),
                "the reduced recipient-passive complement fired for {source:?}"
            );
        }
    }
}

#[test]
fn reduced_recipient_passive_does_not_invent_an_opaque_theme() {
    let source =
        "You gain X life, where X is twice the damage dealt to you so far this turn by artifacts.";
    assert!(
        parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
        "`far` must not become an opaque retained theme"
    );
}

#[test]
fn fronted_conditional_carries_a_reduced_recipient_passive() {
    let source = "If a creature dealt damage this way would die this turn, exile it instead.";
    let parsed = parse(source);
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn reduced_recipient_passive_keeps_agent_and_temporal_tails() {
    let source = "A creature dealt damage by this creature this turn can't block.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Deontic(
        Subject(NounPhrase::Nominal(subject)),
        _,
        _,
    )) = &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a modal clause with a nominal subject");
    };
    let [NominalComplement::ReducedRecipientPassive(predicate)] = subject.complements() else {
        panic!("expected one reduced recipient-passive complement");
    };
    assert!(matches!(
        predicate.elements.as_slice(),
        [
            PredicateElement::Adjunct(PredicateAdjunct::Prepositional(
                PrepositionalPhrase::Simple(crate::syntax::SimplePrepositionalPhrase {
                    preposition: Preposition::By,
                    ..
                })
            )),
            PredicateElement::Adjunct(PredicateAdjunct::Temporal(_)),
        ]
    ));
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
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
    assert!(
        predicate
            .head
            .first_auxiliary_contracted_with_subject
            .is_contracted()
    );
    assert!(matches!(
        &predicate.retained_object,
        Some(PredicateObject::NounPhrase(NounPhrase::Nominal(damage)))
            if matches!(damage.head().kind(), NounInstanceKind::Mass(Noun::Word(Vocab::Damage)))
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
        contracted_negation: crate::features::Contraction::Full,
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
        object.complements(),
        [NominalComplement::Relative(RelativeClause {
            marker: RelativeMarker::That,
            gap: RelativeGap::Subject,
            body: RelativeBody::SubjectGap(Predicate::Deontic(DeonticPredicate {
                modal: Modal {
                    auxiliary: AuxiliaryInstance {
                        auxiliary: Auxiliary::Would,
                        ..
                    },
                },
                inner: Some(predicate),
            })),
        })] if matches!(
            predicate.as_ref(),
            Predicate::Passive(PassivePredicate { elements, .. }) if matches!(
                elements.as_slice(),
                [PredicateElement::Adjunct(PredicateAdjunct::Temporal(_))]
            )
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
                    payload: ClauseAttachmentKind::Adjunct(PredicateAdjunct::Prepositional(_)),
                    ..
                },
                ClauseAttachment {
                    payload: ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
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
    let passive = "Prevent all combat damage that would be dealt to you and creatures you control.";
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
        let crate::syntax::CopularComplement::Adjective(adjective) = &predicate.complement else {
            panic!("expected an adjective complement: {predicate:#?}");
        };
        assert_eq!(adjective.head(), &Adjective::Word(Vocab::Equal));
        assert!(
            matches!(
                adjective.complements(),
                [AdjectiveComplement::Prepositional(preposition)]
                    if preposition.head().preposition == crate::syntax::Preposition::To
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
    let coordination = predicate_coordination(parsed.sentence().unwrap());
    let [
        _,
        PredicateExpression::Simple(Predicate::Transitive(shared)),
    ] = coordination.conjuncts()
    else {
        panic!("expected one shared-predicate conjunct: {coordination:#?}");
    };
    assert!(
        matches!(&shared.object, PredicateObject::QuotedAbility(_)),
        "the shared predicate's object is the quoted ability: {:#?}",
        shared.object
    );
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

/// Two quoted abilities coordinate as one object (`has "A" and "B."`); only
/// the sentence-final conjunct keeps its terminal period inside the quote.
/// Both conjuncts are identical closed quotes structurally — the placement is
/// derived from which one ends the sentence, so the round-trip equality below
/// is what pins it.
#[test]
fn two_quoted_abilities_are_a_coordinated_object() {
    let source =
        "Enchanted creature has \"When this creature dies, draw a card\" and \"{T}: Draw a card.\"";
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
        matches!(first.as_ref(), PredicateObject::QuotedAbility(_)),
        "the non-final conjunct is a quoted ability: {first:#?}"
    );
    assert!(
        matches!(
            rest.as_slice(),
            [PredicateObjectCoordination {
                conjunction: Some(crate::syntax::PredicateConjunction::And),
                object: PredicateObject::QuotedAbility(_),
                ..
            }]
        ),
        "the final conjunct is a quoted ability too: {rest:#?}"
    );
    // The period placement the two conjuncts differ on lives only here now:
    // `card" and "` for the non-final one, `card."` for the sentence-final one.
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

/// The interior's terminal period is a positional distinction **derived at
/// render**, not carried on the node: a quote before a trailing adjunct drops
/// it (`gains "…" until end of turn.`); the same quote in sentence-final
/// position keeps it (`gains "…."`).
///
/// This pins the derivation from both sides. The two quoted abilities are
/// structurally *equal* — nothing about the node records where its period went
/// — yet they round-trip to different surfaces, because the renderer reads the
/// placement off each sentence's tail instead of off the node.
#[test]
fn quoted_ability_interior_period_is_derived_from_sentence_final_position() {
    fn quoted_object(source: &str) -> (crate::syntax::QuotedAbility, String, bool) {
        let parsed = parse(source);
        let sentence = parsed.sentence().unwrap();
        let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) = &sentence.body
        else {
            panic!("expected a transitive clause: {sentence:#?}");
        };
        let PredicateObject::QuotedAbility(quoted) = &predicate.object else {
            panic!("expected a quoted-ability object: {:#?}", predicate.object);
        };
        (
            (**quoted).clone(),
            render_sentence(sentence),
            !predicate.elements.is_empty(),
        )
    }

    let non_final = "This creature gains \"{T}: Draw a card\" until end of turn.";
    let sentence_final = "This creature gains \"{T}: Draw a card.\"";
    let (non_final_quote, non_final_rendered, has_adjunct) = quoted_object(non_final);
    let (final_quote, final_rendered, _) = quoted_object(sentence_final);

    assert!(has_adjunct, "the trailing adjunct survives");
    assert_eq!(
        non_final_quote, final_quote,
        "the same quote in both positions: no stored period distinguishes them"
    );
    assert_eq!(non_final_rendered, non_final);
    assert_eq!(final_rendered, sentence_final);
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

#[test]
fn third_person_host_does_not_adopt_an_infinitive_tail() {
    let source = "That player sacrifices a creature, then draw a card.";
    assert!(
        parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
        "a third-person host must not license a subjectless infinitive tail"
    );
}

#[test]
fn existential_host_does_not_adopt_a_bare_imperative_tail() {
    let source = "There is a creature, then draw a card.";
    assert!(
        parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
        "an agreement-less existential host must not license an imperative tail"
    );
}

// `kwgrant` round, Stage A: a parameterized keyword ability's symbol-cost
// argument fused onto its keyword-noun head in grant position (`ward
// {2}`). Fixture keyword catalog above adds `Ward`, `Equip`,
// `Protection`, `Annihilator`, `Double strike` for this and later stages.

fn keyword_symbol_cost(
    nominal: &crate::syntax::NominalPhrase,
) -> (&str, &[crate::syntax::OracleSymbol]) {
    let NounInstanceKind::Mass(Noun::Catalog(atom)) = nominal.head().kind() else {
        panic!("expected a catalog noun head, got {:?}", nominal.head());
    };
    let [
        NominalComplement::KeywordArgument(KeywordArgument::Costed(KeywordCost::Symbols(symbols))),
    ] = nominal.complements()
    else {
        panic!(
            "expected exactly one symbol-cost keyword argument complement, got {:?}",
            nominal.complements()
        );
    };
    (atom.canonical(), symbols.as_slice())
}

#[test]
fn keyword_grant_symbol_cost_after_keyword_nominal() {
    let source = "Enchanted creature has ward {2}.";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().unwrap().body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
        panic!("expected a nominal object, got {:?}", predicate.object);
    };
    let (canonical, symbols) = keyword_symbol_cost(nominal);
    assert_eq!(canonical, "Ward");
    assert_eq!(symbols.len(), 1);
    assert!(matches!(
        nominal.head().kind(),
        NounInstanceKind::Mass(Noun::Catalog(_))
    ));
}

#[test]
fn keyword_grant_symbol_cost_coordinated_with_bare_keyword() {
    let source = "Equipped creature has flying and ward {4}.";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().unwrap().body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Coordinated(coordinated)) = &predicate.object
    else {
        panic!("expected a coordinated object, got {:?}", predicate.object);
    };
    let NounPhrase::Nominal(flying) = coordinated.first().as_ref() else {
        panic!("expected the first conjunct to be a bare nominal");
    };
    assert!(flying.complements().is_empty(), "flying must stay bare");
    let [second] = coordinated.rest().as_slice() else {
        panic!("expected exactly one coordinated conjunct");
    };
    let NounPhrase::Nominal(ward) = &second.phrase else {
        panic!("expected a nominal conjunct");
    };
    let (canonical, symbols) = keyword_symbol_cost(ward);
    assert_eq!(canonical, "Ward");
    assert_eq!(symbols.len(), 1);
}

#[test]
fn keyword_grant_symbol_cost_after_buff_and_bare_keyword() {
    let source = "Equipped creature gets +2/+1 and has ward {2}.";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn keyword_grant_incidental_equip_symbol_cost() {
    let source = "Equipment you control have equip {1}.";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().unwrap().body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
        panic!("expected a nominal object, got {:?}", predicate.object);
    };
    let (canonical, symbols) = keyword_symbol_cost(nominal);
    assert_eq!(canonical, "Equip");
    assert_eq!(symbols.len(), 1);
}

#[test]
fn keyword_grant_symbol_sequence_cost() {
    let source = "This creature has ward {2}{U}.";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().unwrap().body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
        panic!("expected a nominal object, got {:?}", predicate.object);
    };
    let (canonical, symbols) = keyword_symbol_cost(nominal);
    assert_eq!(canonical, "Ward");
    assert_eq!(symbols.len(), 2);
}

#[test]
fn keyword_grant_annihilator_quantity_control_unchanged() {
    // Negative control (plan §6): `annihilator 2` must remain
    // `NominalComplement::Quantity`, never reclassified as the new
    // `KeywordArgument` complement merely because `annihilator` is a
    // catalog keyword atom. A bare number is not a symbol/predicated
    // shape, so the new rules structurally cannot fire here — this pins
    // that down.
    let source = "This creature has trample and annihilator 2.";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().unwrap().body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Coordinated(coordinated)) = &predicate.object
    else {
        panic!("expected a coordinated object");
    };
    let [second] = coordinated.rest().as_slice() else {
        panic!("expected exactly one coordinated conjunct");
    };
    let NounPhrase::Nominal(annihilator) = &second.phrase else {
        panic!("expected a nominal conjunct");
    };
    assert!(matches!(
        annihilator.complements(),
        [NominalComplement::Quantity(_)]
    ));
}

#[test]
fn keyword_grant_ordinary_noun_never_acquires_a_symbol_complement() {
    // Negative gate (plan §6, orchestrator correction #1): the
    // symbol-argument head slot is gated on catalog keyword-atom
    // membership, never on ordinary noun status, so an ordinary noun
    // must never parse a following symbol cost as a nominal complement.
    for rejected in ["A creature {2}.", "This creature has armor {2}."] {
        assert!(
            parse_nonterminal(rejected, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "{rejected:?} must not parse as a complete sentence"
        );
    }
}

#[test]
fn keyword_grant_ordinary_noun_never_acquires_a_predicated_complement() {
    // Negative gate (plan §6): `armor` is not a keyword catalog atom, so
    // it must not acquire a `from`-predicated complement either, even
    // though Stage B's shared quality carriers are not gated on cost
    // shape.
    assert!(
        parse_nonterminal(
            "this creature has armor from black",
            &fixture_catalogs(),
            Nonterminal::Sentence
        )
        .is_err(),
        "ordinary noun + predicated complement must not parse"
    );
}

// `kwgrant` round, Stage B: explicit `from` qualities fused onto a
// keyword-noun head in grant position (`protection from black`).

fn keyword_predicated_argument(
    nominal: &crate::syntax::NominalPhrase,
) -> (&str, &[crate::syntax::PredicatedQuality]) {
    let NounInstanceKind::Mass(Noun::Catalog(atom)) = nominal.head().kind() else {
        panic!("expected a catalog noun head, got {:?}", nominal.head());
    };
    let [NominalComplement::KeywordArgument(KeywordArgument::Predicated(argument))] =
        nominal.complements()
    else {
        panic!(
            "expected exactly one predicated keyword argument complement, got {:?}",
            nominal.complements()
        );
    };
    (atom.canonical(), argument.qualities.as_slice())
}

#[test]
fn keyword_grant_predicated_single_from_quality() {
    let source = "Enchanted creature has protection from black.";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().unwrap().body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
        panic!("expected a nominal object, got {:?}", predicate.object);
    };
    let (canonical, qualities) = keyword_predicated_argument(nominal);
    assert_eq!(canonical, "Protection");
    let [quality] = qualities else {
        panic!("expected exactly one quality, got {qualities:?}");
    };
    assert_eq!(quality.preposition, Some(Preposition::From));
    assert!(matches!(
        quality.quality,
        Phrase::ColorWord(ColorWord::Black)
    ));
}

#[test]
fn keyword_grant_predicated_coordinated_from_qualities() {
    let source = "Enchanted creature has protection from black and from red.";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().unwrap().body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
        panic!("expected a nominal object, got {:?}", predicate.object);
    };
    let (canonical, qualities) = keyword_predicated_argument(nominal);
    assert_eq!(canonical, "Protection");
    let [first, second] = qualities else {
        panic!("expected exactly two qualities, got {qualities:?}");
    };
    assert_eq!(first.preposition, Some(Preposition::From));
    assert!(matches!(first.quality, Phrase::ColorWord(ColorWord::Black)));
    assert_eq!(second.preposition, Some(Preposition::From));
    assert!(matches!(second.quality, Phrase::ColorWord(ColorWord::Red)));
}

// NOTE: a single, uncoordinated `<keyword> from <NounPhrase>` quality
// (no `and from …` repeat) is genuinely ambiguous against the
// pre-existing generic `Nominal -> Nominal PrepositionalPhrase`
// attachment (`NominalPrepositional`), which can independently complete
// the same span (a bare plural/mass `NounPhrase` needs no determiner).
// Both trees render byte-identically. The declared construction edge
// `nominal_prepositional > nominal_keyword_predicated_argument` makes the
// generic reading win independently of family registration order. A
// coordinated quality (`from X and from Y`) is
// NOT ambiguous this way: the generic single-PP rule cannot consume the
// `and from Y` tail at all, so only the new rule completes the sentence.
// This is documented, not fixed, this round — see the mechanic report's
// residue section; forcing it would require carrying catalog-atom
// identity into `Features::Noun`/`Features::Nominal`, a site-wide change
// to many unrelated rules, out of proportion for this round.

#[test]
fn keyword_grant_predicated_noun_phrase_quality_is_a_known_ambiguity() {
    let source = "This creature has protection from artifacts.";
    let parsed = parse(source);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
    let SentenceBody::Independent(IndependentClause::Transitive(_, predicate)) =
        &parsed.sentence().unwrap().body
    else {
        panic!("expected a transitive clause");
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.object else {
        panic!("expected a nominal object, got {:?}", predicate.object);
    };
    // Pins the currently-observed (ambiguous, generic) tree so a future
    // fix's regression is visible here rather than silently reverting.
    assert!(matches!(
        nominal.complements(),
        [NominalComplement::Prepositional(_)]
    ));
}

#[test]
fn keyword_grant_predicated_or_coordination_stays_unresolved() {
    // Negative gate (confirmed deferral, orchestrator brief): the syntax
    // records no connective, so an `or`-joined predicated argument must
    // never parse through the new list-extension rule (which admits
    // `and` only) and must never be normalized to `and`.
    assert!(
        parse_nonterminal(
            "Enchanted creature has protection from black or from red.",
            &fixture_catalogs(),
            Nonterminal::Sentence,
        )
        .is_err(),
        "an `or`-joined predicated argument must not parse"
    );
}

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

pub(crate) fn fixture_catalogs() -> Catalogs {
    Catalogs::default()
        .with_catalog(
            CatalogKind::KeywordAbility,
            [
                "Flying",
                "Haste",
                "Flash",
                "Defender",
                "Hexproof",
                "Hexproof from",
                "Ward",
                "Equip",
                "Protection",
                "Annihilator",
                "Double strike",
            ],
        )
        .with_catalog(CatalogKind::CreatureType, ["Goblin", "Mount", "Satyr"])
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
    match nominal.head().kind() {
        NounInstanceKind::Singular(Noun::Word(word)) | NounInstanceKind::Mass(Noun::Word(word)) => {
            word.spelling()
        }
        other => panic!("unexpected nominal head: {other:?}"),
    }
}

fn sole_adjective_spelling(nominal: &crate::syntax::NominalPhrase) -> &'static str {
    for modifier in nominal.modifiers() {
        if let NominalModifier::Adjective { phrase, .. } = modifier
            && let Adjective::Word(word) = phrase.head()
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
    let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].payload else {
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
    let crate::syntax::Phrase::NounPhrase(object) = during.head().object.as_ref() else {
        panic!("expected a noun-phrase PP object: {during:#?}");
    };
    let crate::syntax::NounPhrase::Nominal(nominal) = object.as_ref() else {
        panic!("expected a nominal PP object: {during:#?}");
    };
    assert!(
        matches!(
            nominal.modifiers(),
            [NominalModifier::CombatStepName { .. }]
        ),
        "{nominal:#?}"
    );
    assert_eq!(run.rest.len(), 1);
    assert_eq!(run.rest[0].conjunction, Some(PredicateConjunction::And));
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
        predicate_head
            .first_auxiliary_contracted_with_subject
            .is_contracted(),
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
    let SentenceBody::Independent(IndependentClause::Complex(complex)) =
        &parsed.sentence().expect(source).body
    else {
        panic!(
            "expected an until-attached complex clause: {:#?}",
            parsed.sentence()
        );
    };
    assert!(
        complex.attachments.iter().any(|attachment| matches!(
            &attachment.payload,
            ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
                Subordinator::Until,
                SubordinateBody::Finite(clause),
            )) if matches!(
                clause.as_ref(),
                IndependentClause::Passive(_, predicate)
                    if matches!(
                        predicate.head.verb,
                        VerbInstance {
                            verb: Verb::Word(Vocab::Break),
                            slot: VerbSlot::PastParticiple,
                        }
                    )
            )
        )),
        "the until-complement must be a passive verb phrase, not a copular \
         predicate-nominal: {complex:#?}"
    );
}

#[test]
fn youve_is_never_an_opaque_noun() {
    let source = "Cast this spell only during the declare attackers step and only if you've been attacked this step.";
    let parsed = parse(source);
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
    assert_eq!(render_sentence(parsed.sentence().expect(source)), source);
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
    let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].payload else {
        panic!(
            "expected a Restriction attachment: {:#?}",
            complex.attachments[0]
        );
    };
    assert_eq!(run.rest.len(), 2);
    assert_eq!(run.rest[0].conjunction, None);
    assert_eq!(run.rest[1].conjunction, Some(PredicateConjunction::And));
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

fn beginning_of_step_nominal(elements: &[PredicateElement]) -> &NominalPhrase {
    let at = elements
        .iter()
        .find_map(|element| match element {
            PredicateElement::Adjunct(PredicateAdjunct::Prepositional(preposition))
                if preposition.head().preposition == Preposition::At =>
            {
                Some(preposition)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("expected an `at` adjunct: {elements:#?}"));
    let Phrase::NounPhrase(at_object) = at.head().object.as_ref() else {
        panic!("expected an `at` noun-phrase object: {at:#?}");
    };
    let NounPhrase::Nominal(beginning) = at_object.as_ref() else {
        panic!("expected a `beginning` nominal: {at_object:#?}");
    };
    let of = beginning
        .complements()
        .iter()
        .find_map(|complement| match complement {
            NominalComplement::Prepositional(preposition)
                if preposition.head().preposition == Preposition::Of =>
            {
                Some(preposition)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("expected an `of` complement: {beginning:#?}"));
    let Phrase::NounPhrase(of_object) = of.head().object.as_ref() else {
        panic!("expected an `of` noun-phrase object: {of:#?}");
    };
    let NounPhrase::Nominal(step) = of_object.as_ref() else {
        panic!("expected a step nominal: {of_object:#?}");
    };
    step
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
                if pp.head().preposition == Preposition::From =>
            {
                Some(pp)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("expected a `from` PP adjunct: {elements:#?}"));
    let crate::syntax::Phrase::NounPhrase(from_object) = from_pp.head().object.as_ref() else {
        panic!("expected a noun-phrase PP object: {from_pp:#?}");
    };
    let crate::syntax::NounPhrase::Nominal(from_nominal) = from_object.as_ref() else {
        panic!("expected a nominal PP object: {from_pp:#?}");
    };
    let during_pp = from_nominal
        .complements()
        .iter()
        .find_map(|complement| match complement {
            crate::syntax::NominalComplement::Prepositional(pp)
                if pp.head().preposition == Preposition::During =>
            {
                Some(pp)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("expected a `during` PP complement: {from_nominal:#?}"));
    let crate::syntax::Phrase::NounPhrase(object) = during_pp.head().object.as_ref() else {
        panic!("expected a noun-phrase PP object: {during_pp:#?}");
    };
    let crate::syntax::NounPhrase::Nominal(nominal) = object.as_ref() else {
        panic!("expected a nominal PP object: {during_pp:#?}");
    };
    assert!(
        matches!(
            nominal.head().kind(),
            NounInstanceKind::Plural(Noun::Word(word)) if word.spelling() == "step"
        ),
        "expected head `steps`: {nominal:#?}"
    );
    assert!(
        matches!(
            nominal.modifiers(),
            [NominalModifier::Noun {
                noun,
                ..
            }] if matches!(
                noun.kind(),
                NounInstanceKind::Singular(Noun::Opaque(opaque))
                    if opaque.spelling() == "cleanup"
            )
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
    let nominal = beginning_of_step_nominal(elements);
    assert!(
        nominal.modifiers().iter().any(|modifier| matches!(
            modifier,
            NominalModifier::Adjective { phrase, .. }
                if phrase.degree().is_none()
                    && matches!(phrase.head(), Adjective::Word(vocab)
                        if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "next")
        )),
        "expected the `next` adjective on the `beginning of …` object: {nominal:#?}"
    );
    assert!(
        nominal.modifiers().iter().any(|modifier| matches!(
            modifier,
            NominalModifier::Noun {
                noun,
                ..
            } if matches!(
                noun.kind(),
                NounInstanceKind::Singular(Noun::Opaque(opaque))
                    if opaque.spelling() == "cleanup"
            )
        )),
        "expected `cleanup` as a modifier inside the same nominal: {nominal:#?}"
    );
}

#[test]
fn known_noun_step_compounds_are_not_split() {
    // The zero-opacity regression guard: this is the test whose absence
    // let slice B land — `draw`/`end` are fully known nouns, so the
    // slice-B misparse was invisible on the opacity axis and
    // byte-identical on roundtrip.
    for (source, expected_modifier) in [
        (
            "Exile them at the beginning of the next draw step.",
            Vocab::Draw,
        ),
        (
            "Exile them at the beginning of the next end step.",
            Vocab::End,
        ),
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
        let nominal = beginning_of_step_nominal(elements);
        assert!(
            nominal.modifiers().iter().any(|modifier| matches!(
                modifier,
                NominalModifier::Adjective { phrase, .. }
                    if phrase.degree().is_none()
                        && matches!(phrase.head(), Adjective::Word(vocab)
                            if matches!(vocab, Vocab::Regular(_)) && vocab.spelling() == "next")
            )) && nominal.modifiers().iter().any(|modifier| matches!(
                modifier,
                NominalModifier::Noun {
                    noun,
                    ..
                } if matches!(
                    noun.kind(),
                    NounInstanceKind::Singular(Noun::Word(vocab))
                        if *vocab == expected_modifier
                )
            )),
            "expected both compound modifiers attached, not split ({source}): {nominal:#?}"
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
    let ClauseAttachmentKind::Restriction(run) = &complex.attachments[0].payload else {
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
    assert!(
        predicate_head
            .first_auxiliary_contracted_with_subject
            .is_contracted()
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
    let Clause::Independent(IndependentClause::Transitive(_, with_predicate)) = &with_next else {
        panic!("expected transitive");
    };
    let Clause::Independent(IndependentClause::Transitive(_, without_predicate)) = &without_next
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
    // `twice`/`still` never reduce through `verb_phrase_preverb_adverb`.
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
        nominal.modifiers().iter().any(|modifier| matches!(
            modifier,
            NominalModifier::Adjective { phrase, .. }
                if matches!(phrase.head(), Adjective::Word(vocab) if vocab.spelling() == "next")
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

// Round `exrider`: the `except by <PP>` typed predicate exception
// [CR#508.1c,509.1b,702.9b,702.13b,702.36b,702.111b].

fn exception_catalogs() -> Catalogs {
    fixture_catalogs()
        .with_catalog(
            CatalogKind::CreatureType,
            [
                "Goblin",
                "Mount",
                "Wall",
                "Kraken",
                "Leviathan",
                "Octopus",
                "Serpent",
            ],
        )
        .with_catalog(
            CatalogKind::CardType,
            ["Creature", "Land", "Sorcery", "Planeswalker", "Artifact"],
        )
        .with_catalog(
            CatalogKind::KeywordAbility,
            [
                "Flying",
                "Haste",
                "Flash",
                "Defender",
                "Hexproof",
                "Reach",
                "Islandwalk",
                "Menace",
            ],
        )
}

fn parse_exception(source: &str) -> ParsedNonterminal {
    parse_nonterminal(source, &exception_catalogs(), Nonterminal::Sentence)
        .unwrap_or_else(|error| panic!("failed to parse {source:?}: {error:?}"))
}

fn passive_deontic(sentence: &Sentence) -> &PassivePredicate {
    let SentenceBody::Independent(independent) = &sentence.body else {
        panic!("expected an independent clause, got {:?}", sentence.body);
    };
    passive_deontic_in(independent)
}

/// Unwraps a fronted/attached `ComplexClause` (Island Sanctuary's `If you
/// do, until your next turn, ...`) down to its matrix clause before
/// locating the passive deontic predicate.
fn passive_deontic_in(clause: &IndependentClause) -> &PassivePredicate {
    match clause {
        IndependentClause::Deontic(_, _, Some(Predicate::Passive(predicate))) => predicate,
        IndependentClause::Complex(complex) => passive_deontic_in(&complex.matrix),
        other => panic!("expected a deontic passive clause, got {other:?}"),
    }
}

fn sole_exception_pp(predicate: &PassivePredicate) -> &PrepositionalPhrase {
    let [PredicateElement::Adjunct(PredicateAdjunct::Exception(pp))] =
        predicate.elements.as_slice()
    else {
        panic!(
            "expected exactly one Exception adjunct, got {:?}",
            predicate.elements
        );
    };
    pp
}

#[test]
fn except_by_is_typed_predicate_exception() {
    let source = "This creature can't be blocked except by creatures with flying.";
    let parsed = parse_exception(source);
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
    let sentence = parsed.sentence().expect("sentence root");
    let predicate = passive_deontic(sentence);
    let pp = sole_exception_pp(predicate);
    assert_eq!(pp.head().preposition, Preposition::By);
    assert_eq!(render_sentence(sentence), source);
}

#[test]
fn except_by_follows_temporal_adjunct() {
    let source = "This creature can't be blocked this turn except by creatures with haste.";
    let parsed = parse_exception(source);
    assert_eq!(parsed.opacity_mode(), OpacityMode::Exact);
    let sentence = parsed.sentence().expect("sentence root");
    let predicate = passive_deontic(sentence);
    let [
        PredicateElement::Adjunct(PredicateAdjunct::Temporal(_)),
        PredicateElement::Adjunct(PredicateAdjunct::Exception(pp)),
    ] = predicate.elements.as_slice()
    else {
        panic!(
            "expected [Temporal, Exception] in that order, got {:?}",
            predicate.elements
        );
    };
    assert_eq!(pp.head().preposition, Preposition::By);
    assert_eq!(render_sentence(sentence), source);
}

#[test]
fn except_by_reuses_existing_noun_phrase_coordination() {
    for source in [
        "This creature can't be blocked except by Walls and/or creatures with flying.",
        "This creature can't be blocked except by artifact creatures and/or white creatures.",
        "Krakens, Leviathans, Octopuses, and Serpents you control can't be blocked except by \
             Krakens, Leviathans, Octopuses, and Serpents.",
    ] {
        let parsed = parse_exception(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        let sentence = parsed.sentence().expect("sentence root");
        let predicate = passive_deontic(sentence);
        let pp = sole_exception_pp(predicate);
        assert_eq!(pp.head().preposition, Preposition::By, "{source}");
        assert_eq!(render_sentence(sentence), source, "{source}");
    }
}

#[test]
fn except_by_survives_complex_hosts() {
    // Agility Bobblehead and Infiltrator's Magemark are excluded here:
    // the host pre-check found their hosts carry an independent `and
    // can't be blocked` VP-coordination gap and so cannot become whole
    // recoveries this round (residue, not a mover) — see the round
    // report. These four rows are confirmed movers.
    for source in [
        "That creature can't be blocked this combat except by creatures with flying and \
             creatures in a pile with the chosen label.",
        "If you do, until your next turn, you can't be attacked except by creatures with \
             flying and/or islandwalk.",
        "Each creature you control with menace can't be blocked except by three or more \
             creatures.",
        "This creature can't be blocked this turn except by snow creatures.",
    ] {
        let parsed = parse_exception(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        let sentence = parsed.sentence().expect("sentence root");
        let predicate = passive_deontic(sentence);
        assert!(
            predicate.elements.iter().any(|element| matches!(
                element,
                PredicateElement::Adjunct(PredicateAdjunct::Exception(_))
            )),
            "expected an Exception adjunct for {source}: {:?}",
            predicate.elements
        );
        assert_eq!(render_sentence(sentence), source, "{source}");
    }
}

#[test]
fn exception_adjunct_renders_and_reparses_from_ast() {
    // Source-free: start from a bare passive `by` PP (never containing
    // `except`), rewrite the typed adjunct in memory to
    // `PredicateAdjunct::Exception`, render, reparse, and compare —
    // a different head (`Attack`) and object (`haste`) from the
    // `except_by_is_typed_predicate_exception` fixture's `Block`/`flying`.
    let base = "This creature can't be attacked by creatures with haste.";
    let mut sentence = parse_exception(base)
        .sentence()
        .expect("sentence root")
        .clone();
    let SentenceBody::Independent(IndependentClause::Deontic(
        _,
        _,
        Some(Predicate::Passive(predicate)),
    )) = &mut sentence.body
    else {
        panic!("expected a deontic passive clause");
    };
    let [PredicateElement::Adjunct(adjunct @ PredicateAdjunct::Prepositional(_))] =
        predicate.elements.as_mut_slice()
    else {
        panic!("expected a single Prepositional adjunct in the base fixture");
    };
    let PredicateAdjunct::Prepositional(pp) = adjunct.clone() else {
        unreachable!()
    };
    *adjunct = PredicateAdjunct::Exception(pp.clone());
    let rendered = render_sentence(&sentence);
    assert_eq!(
        rendered,
        "This creature can't be attacked except by creatures with haste."
    );
    let reparsed = parse_exception(&rendered);
    let reparsed_sentence = reparsed.sentence().expect("sentence root");
    let reparsed_predicate = passive_deontic(reparsed_sentence);
    let reparsed_pp = sole_exception_pp(reparsed_predicate);
    assert_eq!(*reparsed_pp, pp);
}

#[test]
fn bare_by_blocking_controls_are_unchanged() {
    for source in [
        "This creature can't be blocked by creatures with flying.",
        "This creature can't be blocked except by creatures with flying.",
    ] {
        let parsed = parse_exception(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
    }
    // The bare-`by` control keeps its ordinary Prepositional adjunct —
    // never Exception — even after the new production is registered.
    let source = "This creature can't be blocked by creatures with flying.";
    let parsed = parse_exception(source);
    let sentence = parsed.sentence().expect("sentence root");
    let predicate = passive_deontic(sentence);
    assert!(matches!(
        predicate.elements.as_slice(),
        [PredicateElement::Adjunct(PredicateAdjunct::Prepositional(
            _
        ))]
    ));
    assert_eq!(render_sentence(sentence), source);
}

#[test]
fn other_except_classes_are_unchanged() {
    // Class C (clausal `ExceptionRider`), class D (draw-event `except
    // NP`), class E (`except that Clause`), and class F (non-`By`
    // `except` PPs and fronted `Except for`) must all keep their prior
    // disposition: the new append-last `By`-only production must not
    // consume any of these.
    let source = "Each spell costs {3} more to cast except during its controller's turn.";
    let result = parse_nonterminal(source, &exception_catalogs(), Nonterminal::Sentence);
    assert!(
        result.is_err() || result.unwrap().opacity_mode() != OpacityMode::Exact,
        "{source} must not become a whole recovery via the new production"
    );
}

#[test]
fn except_by_rejects_non_by_and_malformed_tails() {
    for source in [
        "This creature can't be blocked except during its controller's turn.",
        "This creature can't be blocked except creatures with flying.",
        "This creature can't be blocked except by Walls except by creatures with flying.",
    ] {
        let result = parse_nonterminal(source, &exception_catalogs(), Nonterminal::Sentence);
        let whole = matches!(&result, Ok(parsed) if parsed.opacity_mode() == OpacityMode::Exact);
        assert!(
            !whole,
            "{source} must keep failing to become a whole recovery"
        );
    }
}

// ---- Stage A: shared-deontic coordinated clause members ----

#[test]
fn shared_deontic_active_modal_coordinates_with_a_transitive_first_conjunct() {
    let source = "Enchanted creature gets +2/+2 and can't attack.";
    let parsed = parse(source);
    let coordination = predicate_coordination(parsed.sentence().expect("sentence root"));
    let [
        _,
        PredicateExpression::Simple(Predicate::Deontic(DeonticPredicate {
            modal,
            inner: Some(predicate),
        })),
    ] = coordination.conjuncts()
    else {
        panic!("expected one shared-deontic conjunct: {coordination:#?}");
    };
    assert_eq!(modal.auxiliary.auxiliary, Auxiliary::Can);
    assert!(
        matches!(predicate.as_ref(), Predicate::Intransitive(_)),
        "`attack` under the modal stays intransitive: {predicate:#?}"
    );
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn shared_predicates_receive_the_finite_subject_agreement() {
    let plural = parse("Other Merfolk get +1/+1 and have islandwalk.");
    let coordination = predicate_coordination(plural.sentence().expect("plural sentence"));
    let [_, PredicateExpression::Simple(Predicate::Transitive(have))] = coordination.conjuncts()
    else {
        panic!("expected a transitive `have` conjunct: {coordination:#?}");
    };
    assert_eq!(
        have.head.verb.slot,
        VerbSlot::Present {
            person: Person::Third,
            number: Number::Plural,
        }
    );

    let singular = parse("This creature gets +2/+0 and has flying.");
    let coordination = predicate_coordination(singular.sentence().expect("singular sentence"));
    let [_, PredicateExpression::Simple(Predicate::Transitive(has))] = coordination.conjuncts()
    else {
        panic!("expected a transitive `has` conjunct: {coordination:#?}");
    };
    assert_eq!(
        has.head.verb.slot,
        VerbSlot::Present {
            person: Person::Third,
            number: Number::Singular,
        }
    );
}

#[test]
fn shared_predicate_agreement_never_changes_an_ambiguous_tail_spelling() {
    for source in [
        "Target creature gains flying and double strike until end of turn.",
        "Return enchanted creature card to the battlefield under your control and attach this Aura to it.",
    ] {
        let parsed = parse(source);
        assert_eq!(
            render_sentence(parsed.sentence().expect("sentence root")),
            source
        );
    }
}

#[test]
fn shared_deontic_passive_modal_coordinates_with_a_transitive_first_conjunct() {
    let source = "Enchanted creature gets +1/+0 and can't be blocked.";
    let parsed = parse(source);
    let coordination = predicate_coordination(parsed.sentence().expect("sentence root"));
    let [
        _,
        PredicateExpression::Simple(Predicate::Deontic(DeonticPredicate {
            modal,
            inner: Some(predicate),
        })),
    ] = coordination.conjuncts()
    else {
        panic!("expected one shared-deontic conjunct: {coordination:#?}");
    };
    assert_eq!(modal.auxiliary.auxiliary, Auxiliary::Can);
    assert!(
        matches!(predicate.as_ref(), Predicate::Passive(_)),
        "`be blocked` under the modal is passive: {predicate:#?}"
    );
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn shared_deontic_composes_with_a_modal_first_clause() {
    let source = "This creature can't block and can't be blocked.";
    let parsed = parse(source);
    let coordination = predicate_coordination(parsed.sentence().expect("sentence root"));
    let [
        PredicateExpression::Simple(Predicate::Deontic(DeonticPredicate {
            inner: Some(first),
            ..
        })),
        PredicateExpression::Simple(Predicate::Deontic(DeonticPredicate {
            modal,
            inner: Some(predicate),
        })),
    ] = coordination.conjuncts()
    else {
        panic!("expected two deontic predicate conjuncts: {coordination:#?}");
    };
    assert!(matches!(first.as_ref(), Predicate::Intransitive(_)));
    assert_eq!(modal.auxiliary.auxiliary, Auxiliary::Can);
    assert!(
        matches!(predicate.as_ref(), Predicate::Passive(_)),
        "`be blocked` under the modal is passive: {predicate:#?}"
    );
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn shared_deontic_passive_composes_with_an_exception_tail() {
    let source =
        "Enchanted creature gets +1/+1 and can't be blocked except by creatures with flying.";
    let parsed = parse(source);
    let coordination = predicate_coordination(parsed.sentence().expect("sentence root"));
    let [
        _,
        PredicateExpression::Simple(Predicate::Deontic(DeonticPredicate {
            modal,
            inner: Some(predicate),
        })),
    ] = coordination.conjuncts()
    else {
        panic!("expected one shared-deontic conjunct: {coordination:#?}");
    };
    assert_eq!(modal.auxiliary.auxiliary, Auxiliary::Can);
    let Predicate::Passive(passive) = predicate.as_ref() else {
        panic!("expected a passive predicate under the modal: {predicate:#?}");
    };
    assert!(
        matches!(
            passive.elements.as_slice(),
            [PredicateElement::Adjunct(PredicateAdjunct::Exception(_))]
        ),
        "expected a single exception-tail adjunct: {:#?}",
        passive.elements
    );
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn shared_deontic_with_vp_ellipsis_preserves_the_first_deontic_ellipsis() {
    // Gilded-Drake-shaped (the real card's "make an exchange" object is
    // not in the fixture/regular vocabulary, so this witness swaps in a
    // supported verb while keeping the exact motivating shape): the first
    // conjunct ("you don't") is do-support VP-ellipsis — it lowers to
    // `IndependentClause::Proform`, not `Deontic`, because `do` is never
    // classified as a modal auxiliary (`is_modal`); the coordinated
    // member ("can't attack") carries a real modal and predicate. The two
    // ellipsis sites are independent — fixing the coordinated-member case
    // must not disturb the pre-existing first-clause ellipsis.
    let source = "If you don't or can't attack, sacrifice this creature.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Complex(complex)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a complex clause: {:#?}", parsed.sentence());
    };
    assert!(matches!(
        complex.matrix.as_ref(),
        IndependentClause::Imperative(_)
    ));
    let [attachment] = complex.attachments.as_slice() else {
        panic!("expected one attachment: {:#?}", complex.attachments);
    };
    let ClauseAttachmentKind::Dependent(DependentClause::Subordinate(
        Subordinator::If,
        SubordinateBody::Finite(if_body),
    )) = &attachment.payload
    else {
        panic!(
            "expected a finite `if` subordinate: {:#?}",
            attachment.payload
        );
    };
    let IndependentClause::Predicated(_, PredicateExpression::Coordinated(coordination)) =
        if_body.as_ref()
    else {
        panic!("expected coordinated predicates in the `if` body: {if_body:#?}");
    };
    let [
        PredicateExpression::Simple(Predicate::Proform(crate::syntax::ProPredicate { auxiliary })),
        PredicateExpression::Simple(Predicate::Deontic(DeonticPredicate {
            modal,
            inner: Some(predicate),
        })),
    ] = coordination.conjuncts()
    else {
        panic!("expected proform and deontic predicate conjuncts: {coordination:#?}");
    };
    assert_eq!(auxiliary.auxiliary, Auxiliary::Do);
    assert!(auxiliary.contracted_negation.is_contracted());
    assert_eq!(modal.auxiliary.auxiliary, Auxiliary::Can);
    assert!(matches!(predicate.as_ref(), Predicate::Intransitive(_)));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn shared_deontic_none_renders_as_the_bare_modal() {
    // Source-free renderer check: an elided deontic conjunct must never
    // synthesize a pro-verb, only the modal auxiliary itself.
    let second = Predicate::Deontic(DeonticPredicate {
        modal: Modal {
            auxiliary: crate::word::AuxiliaryInstance {
                auxiliary: Auxiliary::Can,
                inflection: AuxiliaryInflection::Base,
                contracted_negation: crate::features::Contraction::Contracted,
            },
        },
        inner: None,
    });
    let parsed_first = parse("You draw a card.");
    let SentenceBody::Independent(IndependentClause::Transitive(subject, first_predicate)) =
        parsed_first.sentence().expect("sentence root").body.clone()
    else {
        panic!("expected a finite first clause");
    };
    let coordinated = IndependentClause::Predicated(
        Some(subject),
        PredicateExpression::Coordinated(Coordination::new(
            PredicateExpression::Simple(Predicate::Transitive(first_predicate)),
            CoordinationJunction {
                conjunction: Some(PredicateConjunction::And),
                comma: crate::features::Comma::Absent,
            },
            PredicateExpression::Simple(second),
        )),
    );
    let sentence = Sentence {
        body: SentenceBody::Independent(coordinated),
    };
    let rendered = render_sentence(&sentence);
    assert_eq!(rendered, "You draw a card and can't.");
}

#[test]
fn changed_predicate_connective_nests_the_completed_left_group() {
    let source = "Enchanted creature can't attack or block and has flying.";
    let parsed = parse(source);
    let outer = predicate_coordination(parsed.sentence().expect("sentence root"));
    assert!(matches!(
        outer.junctions(),
        [CoordinationJunction {
            conjunction: Some(PredicateConjunction::And),
            comma: crate::features::Comma::Absent,
        }]
    ));
    let [
        PredicateExpression::Coordinated(inner),
        PredicateExpression::Simple(_),
    ] = outer.conjuncts()
    else {
        panic!("expected `(attack or block) and has`: {outer:#?}");
    };
    assert_eq!(inner.conjuncts().len(), 2);
    assert!(matches!(
        inner.junctions(),
        [CoordinationJunction {
            conjunction: Some(PredicateConjunction::Or),
            comma: crate::features::Comma::Absent,
        }]
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn mixed_complete_clause_connectives_keep_both_punctuation_groups() {
    let source = concat!(
        "You draw a card, then you discard a card, or you lose 1 life ",
        "and you gain 1 life."
    );
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Coordinated(outer)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!(
            "expected outer complete-clause coordination: {:#?}",
            parsed.sentence()
        );
    };
    assert!(
        matches!(outer.first.as_ref(), IndependentClause::Coordinated(left) if matches!(
            left.rest.as_slice(),
            [ClauseCoordination {
                conjunction: Some(PredicateConjunction::Then),
                comma: crate::features::Comma::Present,
                ..
            }]
        ))
    );
    let [
        ClauseCoordination {
            conjunction: Some(PredicateConjunction::Or),
            comma: crate::features::Comma::Present,
            member: CoordinatedClauseMember::Independent(right),
        },
    ] = outer.rest.as_slice()
    else {
        panic!("expected one comma-delimited outer `or`: {outer:#?}");
    };
    assert!(
        matches!(right.as_ref(), IndependentClause::Coordinated(right) if matches!(
            right.rest.as_slice(),
            [ClauseCoordination {
                conjunction: Some(PredicateConjunction::And),
                comma: crate::features::Comma::Absent,
                ..
            }]
        ))
    );
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn and_or_clause_coordination_preserves_opaque_named_card_search() {
    let source = "Search your library and/or graveyard for a card named TARDIS.";
    let parsed = parse(source);
    assert_eq!(parsed.opacity_mode(), OpacityMode::OpaqueNouns);
    let SentenceBody::Independent(IndependentClause::Coordinated(coordination)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected clause coordination: {:#?}", parsed.sentence());
    };
    assert!(matches!(
        coordination.rest.as_slice(),
        [ClauseCoordination {
            conjunction: Some(PredicateConjunction::AndOr),
            comma: crate::features::Comma::Absent,
            ..
        }]
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

#[test]
fn uniform_clause_and_predicate_runs_remain_flat() {
    let clauses = "You draw a card and you discard a card and you gain 1 life.";
    let parsed = parse(clauses);
    let SentenceBody::Independent(IndependentClause::Coordinated(coordination)) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected one flat clause run: {:#?}", parsed.sentence());
    };
    assert_eq!(coordination.rest.len(), 2);
    assert!(matches!(
        coordination.rest.as_slice(),
        [
            ClauseCoordination {
                conjunction: Some(PredicateConjunction::And),
                comma: crate::features::Comma::Absent,
                ..
            },
            ClauseCoordination {
                conjunction: Some(PredicateConjunction::And),
                comma: crate::features::Comma::Absent,
                ..
            }
        ]
    ));

    let predicates = "This creature gets +1/+1 and gains flying and has vigilance.";
    let parsed = parse(predicates);
    let coordination = predicate_coordination(parsed.sentence().expect("sentence root"));
    assert!(
        coordination
            .conjuncts()
            .iter()
            .all(|expression| matches!(expression, PredicateExpression::Simple(_)))
    );
    assert_eq!(coordination.conjuncts().len(), 3);
    assert_eq!(render_sentence(parsed.sentence().unwrap()), predicates);
}

#[test]
fn coordinated_postpositive_participles_share_the_creature_head() {
    let source = "Put a counter on target creature blocking or blocked by this creature.";
    let parsed = parse(source);
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(put))) =
        &parsed.sentence().expect("sentence root").body
    else {
        panic!("expected a transitive imperative: {:#?}", parsed.sentence());
    };
    let PredicateObject::NounPhrase(NounPhrase::Nominal(counter)) = &put.object else {
        panic!("expected a counter object: {put:#?}");
    };
    let [NominalComplement::Prepositional(on)] = counter.complements() else {
        panic!("expected one recipient PP: {counter:#?}");
    };
    let Phrase::NounPhrase(target) = on.head().object.as_ref() else {
        panic!("expected a target creature: {on:#?}");
    };
    let NounPhrase::Nominal(target) = target.as_ref() else {
        panic!("expected one nominal target: {target:#?}");
    };
    let [NominalComplement::CoordinatedAdjective(participles)] = target.complements() else {
        panic!("expected coordinated postpositive participles: {target:#?}");
    };
    assert!(matches!(
        participles.first.head(),
        Adjective::Participle(Tense::Present, Verb::Word(Vocab::Block))
    ));
    let [second] = participles.rest.as_slice() else {
        panic!("expected one `or blocked` member: {participles:#?}");
    };
    assert_eq!(second.conjunction, Some(PredicateConjunction::Or));
    assert!(matches!(
        second.phrase.head(),
        Adjective::Participle(Tense::Past, Verb::Word(Vocab::Block))
    ));
    assert!(matches!(
        second.phrase.complements(),
        [AdjectiveComplement::Prepositional(by)] if by.head().preposition == Preposition::By
    ));
    assert_eq!(render_sentence(parsed.sentence().unwrap()), source);
}

// ---- Stage C: narrow additive-type shared copular members ----

#[test]
fn additive_type_copular_continuations_share_the_finite_subject() {
    for (source, conjunction, comma) in [
        (
            "Enchanted creature gets +1/+1 and is a Goblin in addition to its other types.",
            Some(PredicateConjunction::And),
            false,
        ),
        (
            "Enchanted creature gets +1/+1, and is a Goblin in addition to its other types.",
            Some(PredicateConjunction::And),
            true,
        ),
        (
            "Enchanted creature gets +1/+1, is a Goblin in addition to its other types.",
            None,
            true,
        ),
    ] {
        let parsed = parse(source);
        assert_eq!(parsed.opacity_mode(), OpacityMode::Exact, "{source}");
        let coordination = predicate_coordination(parsed.sentence().expect("sentence root"));
        let [_, PredicateExpression::Simple(Predicate::Copular(copular))] =
            coordination.conjuncts()
        else {
            panic!("expected a shared copular continuation: {coordination:#?}");
        };
        assert!(matches!(
            copular.complement,
            CopularComplement::NounPhrase(NounPhrase::Nominal(_))
        ));
        let [PredicateAdjunct::Prepositional(preposition)] = copular.adjuncts.as_slice() else {
            panic!("expected one additive prepositional adjunct: {copular:#?}");
        };
        assert_eq!(preposition.head().preposition, Preposition::In);
        let Phrase::NounPhrase(object) = preposition.head().object.as_ref() else {
            panic!("expected a nominal `addition` object: {preposition:#?}");
        };
        let NounPhrase::Nominal(addition) = object.as_ref() else {
            panic!("expected a nominal `addition` object: {object:#?}");
        };
        assert!(matches!(
            addition.complements(),
            [NominalComplement::Prepositional(to)] if to.head().preposition == Preposition::To
        ));
        assert!(matches!(
            coordination.junctions(),
            [CoordinationJunction {
                conjunction: actual_conjunction,
                comma: actual_comma,
            }] if *actual_conjunction == conjunction && actual_comma.is_present() == comma
        ));

        let rendered = render_sentence(parsed.sentence().unwrap());
        assert_eq!(rendered, source);
        let reparsed = parse(&rendered);
        assert_eq!(reparsed.sentence(), parsed.sentence(), "{source}");
    }
}

#[test]
fn additive_type_copular_continuation_rejects_broader_copular_surfaces() {
    for source in [
        "Equipped creature gets +3/+0 and is every creature type.",
        "Equipped creature gets +1/+1 and is all creature types.",
        "Equipped creature gets +2/+2 and is a black Goblin.",
        "Equipped creature gets +2/+2 or is a Goblin in addition to its other types.",
        "Equipped creature gets +2/+2 and are Goblins in addition to their other types.",
        "Draw a card and is a Goblin in addition to its other types.",
    ] {
        assert!(
            parse_nonterminal(source, &fixture_catalogs(), Nonterminal::Sentence).is_err(),
            "the narrow additive-type continuation must reject {source:?}"
        );
    }
}

/// The members of the sole coordinated prepositional adjunct of an imperative
/// transitive predicate.
fn sole_coordinated_prepositional_adjunct(
    sentence: &Sentence,
) -> &crate::syntax::CoordinatedPrepositionalPhrase {
    let SentenceBody::Independent(IndependentClause::Imperative(Predicate::Transitive(predicate))) =
        &sentence.body
    else {
        panic!(
            "expected an imperative transitive clause: {:#?}",
            sentence.body
        );
    };
    // The run attaches to the direct object's nominal, not to the verb:
    // `target card from your graveyard and from your hand`.
    let PredicateObject::NounPhrase(NounPhrase::Nominal(nominal)) = &predicate.kind.object else {
        panic!("expected a nominal direct object: {:#?}", predicate.kind);
    };
    let [NominalComplement::Prepositional(pp)] = nominal.complements() else {
        panic!(
            "expected exactly one prepositional complement: {:#?}",
            nominal.complements()
        );
    };
    let PrepositionalPhrase::Coordinated(coordinated) = pp else {
        panic!("expected a coordinated prepositional phrase: {pp:#?}");
    };
    coordinated
}

#[test]
fn sibling_prepositional_pair_takes_no_serial_comma() {
    let source = "Exile target card from your graveyard and from your hand.";
    let parsed = parse(source);
    let sentence = parsed.sentence().expect("sentence root");
    let coordinated = sole_coordinated_prepositional_adjunct(sentence);
    // Two members: the conjunction rides the single continuation, and no comma
    // is stored anywhere — the renderer derives its absence from the length.
    let [second] = coordinated.rest.as_slice() else {
        panic!("expected one continuation: {:#?}", coordinated.rest);
    };
    assert_eq!(
        second.conjunction,
        Some(crate::syntax::NounPhraseConjunction::And)
    );
    assert_eq!(render_sentence(sentence), source);
}

#[test]
fn sibling_prepositional_oxford_list_takes_the_serial_comma() {
    let source = "Exile target card from your graveyard, from your hand, and from exile.";
    let parsed = parse(source);
    let sentence = parsed.sentence().expect("sentence root");
    let coordinated = sole_coordinated_prepositional_adjunct(sentence);
    // Three members: the interior one is asyndetic, the last carries `and`.
    let [interior, last] = coordinated.rest.as_slice() else {
        panic!("expected two continuations: {:#?}", coordinated.rest);
    };
    assert_eq!(interior.conjunction, None);
    assert_eq!(
        last.conjunction,
        Some(crate::syntax::NounPhraseConjunction::And)
    );
    assert_eq!(render_sentence(sentence), source);
}

#[test]
fn a_comma_before_a_two_member_conjunction_is_not_a_prepositional_coordination() {
    // Giant Oyster's shape: the comma before `and` marks a clause boundary, so
    // strict serial-comma style forbids reading `from A, and from B` as one
    // coordinated phrase. The pair base of `PrepositionalPhraseList` makes that
    // unrepresentable rather than merely unpreferred.
    let source = "Exile target card from your graveyard, and put a card into your hand.";
    let parsed = parse(source);
    let sentence = parsed.sentence().expect("sentence root");
    assert!(
        !format!("{sentence:#?}").contains("CoordinatedPrepositionalPhrase"),
        "a clause-boundary comma must not coordinate prepositional phrases: {sentence:#?}"
    );
    assert_eq!(render_sentence(sentence), source);
}
