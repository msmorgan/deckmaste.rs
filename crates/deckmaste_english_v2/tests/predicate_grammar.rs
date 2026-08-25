use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use macro_ron::v2::DeclarationIdentity;
use macro_ron::v2::Onset;
use macro_ron::v2::read_str;

fn environment() -> ParserEnvironment {
    let declarations = [
        (
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Sacrifice.ron",
            r#"KeywordAction(name:"Sacrifice",spelling:"sacrifice",grammar:Verb(bare:"sacrifice",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"connive",grammar:Verb(bare:"connive",valence:Intransitive))"#,
        ),
        (
            "/synthetic/actions/Scry.ron",
            r#"KeywordAction(name:"Scry",spelling:"scry",grammar:Verb(bare:"scry",third_person:"scries",valence:Custom(shapes:[[],[Amount]])))"#,
        ),
        (
            "/synthetic/actions/Surveil.ron",
            r#"KeywordAction(name:"Surveil",spelling:"surveil",grammar:Verb(bare:"surveil",valence:Numerative))"#,
        ),
        (
            "/synthetic/actions/Discard.ron",
            r#"KeywordAction(name:"Discard",spelling:"discard",grammar:Verb(bare:"discard",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Create.ron",
            r#"KeywordAction(name:"Create",spelling:"create",grammar:Verb(bare:"create",valence:Transitive))"#,
        ),
        (
            "/synthetic/types/Creature.ron",
            r#"Type(name:"Creature",spelling:"creature",grammar:Noun(singular:"creature"))"#,
        ),
    ]
    .into_iter()
    .map(|(path, source)| read_str(path, source).expect("synthetic keyword action is valid"));
    ParserEnvironment::try_from_parts(
        declarations,
        [CatalogProviderRows::new(
            CatalogProvider::CardNames,
            [
                CatalogProviderRow::new("context-card", "Context Card", Onset::Consonant),
                CatalogProviderRow::new("plus-two-mace", "+2 Mace", Onset::Consonant),
                CatalogProviderRow::new(
                    "zoraline-cosmos-caller",
                    "Zoraline, Cosmos Caller",
                    Onset::Consonant,
                ),
            ],
        )],
    )
    .expect("synthetic predicate environment freezes")
}

fn parser() -> Parser {
    Parser::new(environment()).expect("shared active frames have no fixed-name requirements")
}

fn context() -> ParseContext<'static> {
    ParseContext::new("Context Card", false, Onset::Consonant).expect("context is valid")
}

fn imperative_atomic(parser: &Parser, context: &ParseContext<'_>, text: &str) -> VerbPhrase {
    let Sentence::Imperative(imperative) = parser
        .parse_sentence(text, context)
        .unwrap_or_else(|error| panic!("imperative product must parse {text:?}: {error:?}"))
    else {
        panic!("bare predicate has an imperative envelope: {text:?}")
    };
    let Predicate::Atomic(predicate) = imperative.predicate() else {
        panic!("one predicate has an atomic envelope: {text:?}")
    };
    predicate.clone()
}

fn exact_claim_trace(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
) -> Vec<(String, String)> {
    let analysis = parser.analyze(text, context);
    assert!(analysis.selected().is_some(), "{text:?}: {analysis:?}");
    let claims = analysis
        .ownership()
        .expect("selected document has ownership")
        .parsed_claims();
    let mut cursor = 0;
    let trace = claims
        .iter()
        .map(|claim| {
            assert_eq!(claim.span().start, cursor, "{text:?}: {claim:?}");
            assert!(claim.span().end > claim.span().start, "{text:?}: {claim:?}");
            cursor = claim.span().end;
            (
                text[claim.span().start..claim.span().end].to_owned(),
                claim.stable_owner_id().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        cursor,
        text.len(),
        "{text:?}: claims are total and disjoint"
    );
    trace
}

fn assert_selected(parser: &Parser, context: &ParseContext<'_>, text: &str) -> Ability {
    assert_selected_with_specificity(parser, context, text, false)
}

fn assert_selected_with_specificity(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    permits_specificity: bool,
) -> Ability {
    let analysis = parser.analyze(text, context);
    let selected = analysis.selected().unwrap_or_else(|| {
        panic!("complete predicate document must select {text:?}: {analysis:?}")
    });
    let decision = analysis
        .decision()
        .expect("selected document has a decision");
    assert!(
        decision.resolution() == SelectionResolution::Unique
            || (permits_specificity && decision.resolution() == SelectionResolution::Specificity),
        "{text:?}: {decision:?}",
    );
    assert_eq!(decision.survivors().len(), 1, "{text:?}");
    assert!(decision.exception_uses().is_empty(), "{text:?}");
    assert_eq!(selected.render(context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("selected document has ownership");
    assert!(ownership.summary().covered(), "{text:?}: {ownership:?}");
    assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
    assert_eq!(ownership.rendered_text(), text);
    assert_eq!(ownership.summary().gap_spans(), 0, "{text:?}");
    assert_eq!(ownership.summary().overlap_spans(), 0, "{text:?}");
    assert_eq!(ownership.summary().synthetic_claims(), 0, "{text:?}");
    assert_eq!(
        ownership.summary().provenance_plan_mismatches(),
        0,
        "{text:?}"
    );
    selected.clone()
}

#[test]
fn shared_active_frames_reuse_core_and_declared_heads_across_sentence_shapes() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy target player.",
        "Sacrifice target player.",
        "You destroy target player.",
        "It sacrifices target player.",
        "Control target player.",
        "You control target player.",
        "Connive.",
        "It connives.",
        "Enter.",
        "It enters.",
        "You scry.",
        "It scries.",
        "Scry 2.",
        "Surveil X.",
        "Draw 2.",
        "Sacrifice target player and control target player.",
        "You sacrifice target player and control target player.",
    ] {
        assert_selected(&parser, &context, text);
    }

    let sentence = parser
        .parse_sentence("Sacrifice target player.", &context)
        .expect("declared transitive imperative parses");
    let Sentence::Imperative(imperative) = sentence else {
        panic!("bare predicate has an imperative envelope")
    };
    let Predicate::Atomic(VerbPhrase::TransitivePredicate(predicate)) = imperative.predicate()
    else {
        panic!("declared object verb uses the shared transitive product")
    };
    let TransitiveVerb::Declaration(head) = &predicate.head else {
        panic!("declared transitive identity remains open")
    };
    assert_eq!(head.id().name(), "Sacrifice");

    let sentence = parser
        .parse_sentence("Control target player.", &context)
        .expect("core transitive imperative parses");
    let Sentence::Imperative(imperative) = sentence else {
        panic!("bare predicate has an imperative envelope")
    };
    let Predicate::Atomic(VerbPhrase::TransitivePredicate(predicate)) = imperative.predicate()
    else {
        panic!("core object verb uses the shared transitive product")
    };
    assert!(matches!(
        &predicate.head,
        TransitiveVerb::Lexeme(CoreTransitiveVerb::Control)
    ));
}

#[derive(Default)]
struct LexicalVisitor(Vec<String>);

impl Visitor for LexicalVisitor {
    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        self.0.push(format!("declared:{}", declaration.name()));
    }

    fn visit_core_transitive_verb(&mut self, verb: CoreTransitiveVerb) {
        self.0.push(format!("core:{verb:?}"));
    }

    fn visit_common_noun(&mut self, noun: CommonNoun) {
        self.0.push(format!("noun:{noun:?}"));
    }
}

#[test]
fn shared_transitive_frame_preserves_visit_order_and_literal_claims() {
    let parser = parser();
    let context = context();
    let text = "You sacrifice target player and control target player.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = LexicalVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.0,
        [
            "declared:Sacrifice",
            "noun:Player",
            "core:Control",
            "noun:Player",
        ]
    );

    let ownership = parser
        .analyze(text, &context)
        .ownership()
        .expect("selected coordination has ownership")
        .parsed_claims()
        .iter()
        .map(|claim| {
            (
                claim.span().start,
                claim.span().end,
                claim.stable_owner_id().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        ownership,
        [
            (0, 3, "vocab:SubjectPronoun/You".to_owned()),
            (3, 13, "lexeme:keyword_action/Sacrifice/bare".to_owned(),),
            (
                13,
                20,
                "form:target_determiner_phrase/target_determiner_phrase/0".to_owned(),
            ),
            (20, 27, "lexeme:CommonNoun/Player/singular".to_owned()),
            (
                27,
                32,
                "structural:AndPredicateCoordination/members/separator/pair/0".to_owned(),
            ),
            (32, 39, "lexeme:CoreTransitiveVerb/Control/bare".to_owned()),
            (
                39,
                46,
                "form:target_determiner_phrase/target_determiner_phrase/0".to_owned(),
            ),
            (46, 53, "lexeme:CommonNoun/Player/singular".to_owned()),
            (
                53,
                54,
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ]
    );
}

#[test]
fn shared_active_frames_reject_complement_and_agreement_reciprocals() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy.",
        "Connive target player.",
        "Control.",
        "Enter target player.",
        "Scry target player.",
        "Scry 2 target player.",
        "Surveil.",
        "Surveil target player.",
        "Sacrifice 2.",
        "It sacrifice target player.",
        "You sacrifices target player.",
        "It connive.",
        "You connives.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong frame or agreement must reject {text:?}",
        );
    }
}

#[test]
fn typed_scalar_measure_counter_and_object_complements_select_exact_products() {
    let parser = parser();
    let context = context();
    for text in [
        "Deal X damage to any target.",
        "It deals damage equal to its power to any target.",
        "Gain that much life.",
        "Gain life equal to its power.",
        "You lose 2 life.",
        "You lose life equal to its toughness.",
        "Pay X life.",
        "Pay {2}.",
        "Pay {2}{B}.",
        "Add {B}{B}{B}.",
        "Draw a card.",
        "Draw two cards.",
        "Draw X cards.",
        "Draw that many cards.",
        "Draw cards equal to its toughness.",
        "Roll a six-sided die.",
        "Roll two six-sided dice.",
        "Roll a d20.",
        "Put a +1/+1 counter on target creature.",
        "Put a -1/-1 counter on target creature.",
        "Put a time counter on target creature.",
        "Put two stun counters on it.",
        "Put that many charge counters on target creature.",
        "Remove X time counters from this card.",
        "Create a token.",
        "Discard a card.",
        "Each player sacrifices a creature of their choice.",
        "Target player discards two cards at random.",
        "Destroy a card named +2 Mace.",
        "Destroy a card named Zoraline, Cosmos Caller.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    let sentence = parser
        .parse_sentence("Put two stun counters on it.", &context)
        .expect("counter-bearing predicate parses");
    let Sentence::Imperative(imperative) = sentence else {
        panic!("bare counter predicate has an imperative envelope")
    };
    let Predicate::Atomic(VerbPhrase::PutCounters(predicate)) = imperative.predicate() else {
        panic!("put-counter syntax selects its exact typed product")
    };
    let CounterQuantity::FixedCounterQuantity(counters) = &predicate.counters else {
        panic!("two counters retain a fixed count")
    };
    let CardinalQuantity::Cardinal(CardinalQuantityValue { number }) = counters.count();
    assert_eq!(number.magnitude, 2);
    assert!(matches!(
        &counters.kind,
        CounterKind::NamedCounter(NamedCounter {
            name: CounterName::Stun,
        })
    ));
    assert!(matches!(
        &predicate.recipient,
        Object::ObjectPronoun(PersonalObject {
            word: ObjectPronoun::It,
        })
    ));

    let sentence = parser
        .parse_sentence(
            "It deals damage equal to its power to any target.",
            &context,
        )
        .expect("scalar-equality damage parses");
    let Sentence::Declarative(declarative) = sentence else {
        panic!("finite scalar predicate has a declarative envelope")
    };
    let Clause::Finite(FiniteClause::PlainFiniteClause(clause)) = &declarative.clause else {
        panic!("finite scalar predicate remains a plain finite clause")
    };
    let Predicate::Atomic(VerbPhrase::DealDamageEqualTo(predicate)) = clause.predicate() else {
        panic!("damage equality selects its exact typed product")
    };
    let ScalarEquality::ScalarEquality(equality) = &predicate.equality;
    assert!(matches!(
        equality.value,
        ScalarValue::PossessedScalarValue(PossessedScalarValue {
            possessor: PossessiveDeterminerPronoun::Its,
            measure: ScalarMeasure::CharacteristicScalar(CharacteristicScalar {
                characteristic: ScalarCharacteristic::Power,
            }),
        })
    ));
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the exact generated complement shape matrix is deliberately exhaustive"
)]
fn typed_complement_products_expose_checked_generated_ast_shapes() {
    let parser = parser();
    let context = context();
    macro_rules! assert_frame {
        ($text:literal, $pattern:pat) => {
            assert!(
                matches!(imperative_atomic(&parser, &context, $text), $pattern),
                "{:?} selects its exact predicate product",
                $text,
            );
        };
    }

    assert_frame!("Deal X damage to any target.", VerbPhrase::DealDamage(_));
    assert_frame!(
        "Deal damage equal to its power to any target.",
        VerbPhrase::DealDamageEqualTo(_)
    );
    assert_frame!("Gain that much life.", VerbPhrase::GainLife(_));
    assert_frame!(
        "Gain life equal to its power.",
        VerbPhrase::GainLifeEqualTo(_)
    );
    assert_frame!("Lose 2 life.", VerbPhrase::LoseLife(_));
    assert_frame!(
        "Lose life equal to its toughness.",
        VerbPhrase::LoseLifeEqualTo(_)
    );
    assert_frame!("Pay X life.", VerbPhrase::PayLife(_));
    assert_frame!("Pay {2}{B}.", VerbPhrase::PayMana(_));
    assert_frame!("Add {B}{B}{B}.", VerbPhrase::AddMana(_));
    assert_frame!("Draw two cards.", VerbPhrase::DrawCards(_));
    assert_frame!(
        "Draw cards equal to its toughness.",
        VerbPhrase::DrawCardsEqualTo(_)
    );
    assert_frame!("Roll a d20.", VerbPhrase::RollDice(_));
    assert_frame!("Put two stun counters on it.", VerbPhrase::PutCounters(_));
    assert_frame!(
        "Remove X time counters from this card.",
        VerbPhrase::RemoveCounters(_)
    );

    for (text, predicate) in [
        (
            "Draw a card.",
            matches!(
                imperative_atomic(&parser, &context, "Draw a card."),
                VerbPhrase::DrawCards(DrawCards {
                    cards: CardQuantity::SingularCardQuantity(_),
                })
            ),
        ),
        (
            "Draw two cards.",
            matches!(
                imperative_atomic(&parser, &context, "Draw two cards."),
                VerbPhrase::DrawCards(DrawCards {
                    cards: CardQuantity::FixedCardQuantity(_),
                })
            ),
        ),
        (
            "Draw X cards.",
            matches!(
                imperative_atomic(&parser, &context, "Draw X cards."),
                VerbPhrase::DrawCards(DrawCards {
                    cards: CardQuantity::VariableCardQuantity(_),
                })
            ),
        ),
        (
            "Draw that many cards.",
            matches!(
                imperative_atomic(&parser, &context, "Draw that many cards."),
                VerbPhrase::DrawCards(DrawCards {
                    cards: CardQuantity::AnaphoricCardQuantity(_),
                })
            ),
        ),
    ] {
        assert!(predicate, "{text:?} retains its typed card quantity");
    }

    for (text, expected) in [
        ("Roll a six-sided die.", "singular"),
        ("Roll two six-sided dice.", "fixed"),
        ("Roll a d20.", "d20"),
    ] {
        let VerbPhrase::RollDice(RollDice { dice }) = imperative_atomic(&parser, &context, text)
        else {
            panic!("{text:?} selects RollDice")
        };
        assert!(
            matches!(
                (expected, dice),
                ("singular", DieObject::SingularDieObject(_))
                    | ("fixed", DieObject::FixedDiceObject(_))
                    | ("d20", DieObject::D20Object(_))
            ),
            "{text:?} retains its typed die object",
        );
    }

    for (text, expected) in [
        ("Put a time counter on target creature.", "singular"),
        ("Put two stun counters on it.", "fixed"),
        ("Put X time counters on target creature.", "variable"),
        (
            "Put that many charge counters on target creature.",
            "anaphoric",
        ),
    ] {
        let VerbPhrase::PutCounters(PutCounters { counters, .. }) =
            imperative_atomic(&parser, &context, text)
        else {
            panic!("{text:?} selects PutCounters")
        };
        assert!(
            matches!(
                (expected, counters),
                ("singular", CounterQuantity::SingularCounterQuantity(_))
                    | ("fixed", CounterQuantity::FixedCounterQuantity(_))
                    | ("variable", CounterQuantity::VariableCounterQuantity(_))
                    | ("anaphoric", CounterQuantity::AnaphoricCounterQuantity(_))
            ),
            "{text:?} retains its typed counter quantity",
        );
    }

    let VerbPhrase::PutCounters(PutCounters { counters, .. }) =
        imperative_atomic(&parser, &context, "Put a +1/+1 counter on target creature.")
    else {
        unreachable!()
    };
    let CounterQuantity::SingularCounterQuantity(counters) = counters else {
        unreachable!()
    };
    let CounterKind::PositivePowerToughnessCounter(counter) = counters.kind else {
        panic!("positive power/toughness kind stays typed")
    };
    assert_eq!(counter.magnitudes().len(), 2);

    let VerbPhrase::PutCounters(PutCounters { counters, .. }) =
        imperative_atomic(&parser, &context, "Put a -1/-1 counter on target creature.")
    else {
        unreachable!()
    };
    let CounterQuantity::SingularCounterQuantity(counters) = counters else {
        unreachable!()
    };
    let CounterKind::NegativePowerToughnessCounter(counter) = counters.kind else {
        panic!("negative power/toughness kind stays typed")
    };
    assert_eq!(counter.magnitudes().len(), 2);

    let VerbPhrase::PayMana(PayMana { mana }) = imperative_atomic(&parser, &context, "Pay {2}{B}.")
    else {
        unreachable!()
    };
    let ManaAmount::ManaAmount(mana) = *mana;
    assert!(matches!(mana.run(), ActivationCostComponent::SymbolRun(_)));

    let VerbPhrase::TransitivePredicate(TransitivePredicate { object, .. }) =
        imperative_atomic(&parser, &context, "Sacrifice a creature of their choice.")
    else {
        panic!("choice phrase remains an ordinary transitive object")
    };
    assert!(matches!(object, Object::ChoiceObject(ChoiceObject { .. })));

    let VerbPhrase::TransitivePredicate(TransitivePredicate { object, .. }) =
        imperative_atomic(&parser, &context, "Discard two cards at random.")
    else {
        panic!("random phrase remains an ordinary transitive object")
    };
    assert!(matches!(object, Object::RandomObject(RandomObject { .. })));
}

#[derive(Default)]
struct ComplementVisitor(Vec<String>);

macro_rules! trace_product {
    ($method:ident, $type:ty, $walk:ident, $label:literal) => {
        fn $method(&mut self, value: &$type) {
            self.0.push(concat!("product:", $label).to_owned());
            deckmaste_english_v2::visit::$walk(self, value);
        }
    };
}

impl Visitor for ComplementVisitor {
    trace_product!(
        visit_deal_damage,
        DealDamage,
        walk_deal_damage,
        "DealDamage"
    );
    trace_product!(
        visit_deal_damage_equal_to,
        DealDamageEqualTo,
        walk_deal_damage_equal_to,
        "DealDamageEqualTo"
    );
    trace_product!(visit_gain_life, GainLife, walk_gain_life, "GainLife");
    trace_product!(
        visit_gain_life_equal_to,
        GainLifeEqualTo,
        walk_gain_life_equal_to,
        "GainLifeEqualTo"
    );
    trace_product!(visit_lose_life, LoseLife, walk_lose_life, "LoseLife");
    trace_product!(
        visit_lose_life_equal_to,
        LoseLifeEqualTo,
        walk_lose_life_equal_to,
        "LoseLifeEqualTo"
    );
    trace_product!(visit_pay_life, PayLife, walk_pay_life, "PayLife");
    trace_product!(visit_pay_mana, PayMana, walk_pay_mana, "PayMana");
    trace_product!(visit_add_mana, AddMana, walk_add_mana, "AddMana");
    trace_product!(visit_draw_cards, DrawCards, walk_draw_cards, "DrawCards");
    trace_product!(
        visit_draw_cards_equal_to,
        DrawCardsEqualTo,
        walk_draw_cards_equal_to,
        "DrawCardsEqualTo"
    );
    trace_product!(visit_roll_dice, RollDice, walk_roll_dice, "RollDice");
    trace_product!(
        visit_put_counters,
        PutCounters,
        walk_put_counters,
        "PutCounters"
    );
    trace_product!(
        visit_remove_counters,
        RemoveCounters,
        walk_remove_counters,
        "RemoveCounters"
    );
    trace_product!(
        visit_singular_card_quantity,
        SingularCardQuantity,
        walk_singular_card_quantity,
        "SingularCardQuantity"
    );
    trace_product!(
        visit_fixed_card_quantity,
        FixedCardQuantity,
        walk_fixed_card_quantity,
        "FixedCardQuantity"
    );
    trace_product!(
        visit_variable_card_quantity,
        VariableCardQuantity,
        walk_variable_card_quantity,
        "VariableCardQuantity"
    );
    trace_product!(
        visit_anaphoric_card_quantity,
        AnaphoricCardQuantity,
        walk_anaphoric_card_quantity,
        "AnaphoricCardQuantity"
    );
    trace_product!(
        visit_singular_die_object,
        SingularDieObject,
        walk_singular_die_object,
        "SingularDieObject"
    );
    trace_product!(
        visit_fixed_dice_object,
        FixedDiceObject,
        walk_fixed_dice_object,
        "FixedDiceObject"
    );
    trace_product!(visit_d20object, D20Object, walk_d20object, "D20Object");
    trace_product!(
        visit_positive_power_toughness_counter,
        PositivePowerToughnessCounter,
        walk_positive_power_toughness_counter,
        "PositivePowerToughnessCounter"
    );
    trace_product!(
        visit_negative_power_toughness_counter,
        NegativePowerToughnessCounter,
        walk_negative_power_toughness_counter,
        "NegativePowerToughnessCounter"
    );
    trace_product!(
        visit_singular_counter_quantity,
        SingularCounterQuantity,
        walk_singular_counter_quantity,
        "SingularCounterQuantity"
    );
    trace_product!(
        visit_fixed_counter_quantity,
        FixedCounterQuantity,
        walk_fixed_counter_quantity,
        "FixedCounterQuantity"
    );
    trace_product!(
        visit_variable_counter_quantity,
        VariableCounterQuantity,
        walk_variable_counter_quantity,
        "VariableCounterQuantity"
    );
    trace_product!(
        visit_anaphoric_counter_quantity,
        AnaphoricCounterQuantity,
        walk_anaphoric_counter_quantity,
        "AnaphoricCounterQuantity"
    );
    trace_product!(
        visit_choice_object,
        ChoiceObject,
        walk_choice_object,
        "ChoiceObject"
    );
    trace_product!(
        visit_random_object,
        RandomObject,
        walk_random_object,
        "RandomObject"
    );

    fn visit_verb_lexeme(&mut self, verb: VerbLexeme) {
        self.0.push(format!("verb:{verb:?}"));
    }

    fn visit_scalar_number(&mut self, number: &ScalarNumber) {
        self.0.push(format!("scalar:{}", number.magnitude));
    }

    fn visit_cardinal_number(&mut self, number: &CardinalNumber) {
        self.0.push(format!("cardinal:{}", number.magnitude));
    }

    fn visit_possessive_determiner_pronoun(&mut self, pronoun: PossessiveDeterminerPronoun) {
        self.0.push(format!("possessive:{pronoun:?}"));
    }

    fn visit_scalar_characteristic(&mut self, characteristic: ScalarCharacteristic) {
        self.0.push(format!("characteristic:{characteristic:?}"));
    }

    fn visit_counter_name(&mut self, counter: CounterName) {
        self.0.push(format!("counter:{counter:?}"));
    }

    fn visit_die_shape(&mut self, shape: DieShape) {
        self.0.push(format!("die:{shape:?}"));
    }

    fn visit_variable(&mut self, variable: Variable) {
        self.0.push(format!("variable:{variable:?}"));
    }

    fn visit_fixed_cost_symbol(&mut self, symbol: FixedCostSymbol) {
        self.0.push(format!("symbol:{symbol:?}"));
    }

    fn visit_common_noun(&mut self, noun: CommonNoun) {
        self.0.push(format!("noun:{noun:?}"));
    }

    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        self.0.push(format!("declared:{}", declaration.name()));
    }
}

#[test]
fn typed_complements_visit_payloads_in_surface_order_with_exact_claims() {
    let parser = parser();
    let context = context();
    let text =
        "Pay {2}, draw cards equal to its toughness, and put two stun counters on target creature.";
    let ability = assert_selected_with_specificity(&parser, &context, text, true);
    let mut visitor = ComplementVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.0,
        [
            "product:PayMana",
            "verb:Pay",
            "scalar:2",
            "product:DrawCardsEqualTo",
            "verb:Draw",
            "possessive:Its",
            "characteristic:Toughness",
            "product:PutCounters",
            "verb:Put",
            "product:FixedCounterQuantity",
            "cardinal:2",
            "counter:Stun",
            "declared:Creature",
        ]
    );

    let analysis = parser.analyze(text, &context);
    let claims = analysis
        .ownership()
        .expect("selected typed coordination has ownership")
        .parsed_claims();
    assert_eq!(claims.first().expect("first claim").span().start, 0);
    assert_eq!(claims.last().expect("last claim").span().end, text.len(),);
    assert!(claims.iter().any(|claim| {
        claim.stable_owner_id() == "form:symbol_run/symbol_run/0/prefix"
            && &text[claim.span().start..claim.span().end] == " {"
    }));
    assert!(claims.iter().any(|claim| {
        claim.stable_owner_id() == "structural:AndPredicateCoordination/members/separator/last/0"
            && &text[claim.span().start..claim.span().end] == ", and "
    }));
    assert_eq!(
        claims.last().expect("terminator claim").stable_owner_id(),
        "structural:Sentences/sentences/terminator/0",
    );
}

#[test]
#[expect(
    clippy::too_many_lines,
    clippy::items_after_statements,
    reason = "one literal matrix authenticates every complement visitor and claim trace"
)]
fn every_new_complement_family_is_reached_by_the_production_visitor() {
    let parser = parser();
    let context = context();
    macro_rules! assert_family {
        ($text:literal, [$($visit:literal),+ $(,)?], [$($claim:expr),+ $(,)?]) => {{
            let text = $text;
            let ability = assert_selected_with_specificity(&parser, &context, text, true);
            let mut visitor = ComplementVisitor::default();
            visitor.visit_ability(&ability);
            assert_eq!(visitor.0, [$($visit),+], "{text:?}: exact visitor trace");
            assert_eq!(
                exact_claim_trace(&parser, &context, text),
                [$((String::from($claim.0), String::from($claim.1))),+],
                "{text:?}: exact total, disjoint claim trace",
            );
        }};
    }

    const TERMINATOR: &str = "structural:Sentences/sentences/terminator/0";
    assert_family!(
        "Deal X damage to any target.",
        ["product:DealDamage", "verb:Deal", "variable:X"],
        [
            ("Deal", "lexeme:VerbLexeme/Deal/bare"),
            (" X", "vocab:Variable/X"),
            (" damage", "form:deal_damage/deal_damage/2"),
            (" to", "form:deal_damage/deal_damage/3"),
            (" any", "form:any_target_reference/any_target_reference/0"),
            (
                " target",
                "form:any_target_reference/any_target_reference/1"
            ),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Deal damage equal to its power to any target.",
        [
            "product:DealDamageEqualTo",
            "verb:Deal",
            "possessive:Its",
            "characteristic:Power"
        ],
        [
            ("Deal", "lexeme:VerbLexeme/Deal/bare"),
            (
                " damage",
                "form:deal_damage_equal_to/deal_damage_equal_to/1"
            ),
            (" equal", "form:scalar_equality/scalar_equality/0"),
            (" to", "form:scalar_equality/scalar_equality/1"),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (" power", "vocab:ScalarCharacteristic/Power"),
            (" to", "form:deal_damage_equal_to/deal_damage_equal_to/3"),
            (" any", "form:any_target_reference/any_target_reference/0"),
            (
                " target",
                "form:any_target_reference/any_target_reference/1"
            ),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Gain that much life.",
        ["product:GainLife", "verb:Gain"],
        [
            ("Gain", "lexeme:VerbLexeme/Gain/bare"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" life", "form:gain_life/gain_life/2"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Gain life equal to its power.",
        [
            "product:GainLifeEqualTo",
            "verb:Gain",
            "possessive:Its",
            "characteristic:Power"
        ],
        [
            ("Gain", "lexeme:VerbLexeme/Gain/bare"),
            (" life", "form:gain_life_equal_to/gain_life_equal_to/1"),
            (" equal", "form:scalar_equality/scalar_equality/0"),
            (" to", "form:scalar_equality/scalar_equality/1"),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (" power", "vocab:ScalarCharacteristic/Power"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Lose 2 life.",
        ["product:LoseLife", "verb:Lose", "scalar:2"],
        [
            ("Lose", "lexeme:VerbLexeme/Lose/bare"),
            (" 2", "codec:ScalarNumber"),
            (" life", "form:lose_life/lose_life/2"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Lose life equal to its toughness.",
        [
            "product:LoseLifeEqualTo",
            "verb:Lose",
            "possessive:Its",
            "characteristic:Toughness"
        ],
        [
            ("Lose", "lexeme:VerbLexeme/Lose/bare"),
            (" life", "form:lose_life_equal_to/lose_life_equal_to/1"),
            (" equal", "form:scalar_equality/scalar_equality/0"),
            (" to", "form:scalar_equality/scalar_equality/1"),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (" toughness", "vocab:ScalarCharacteristic/Toughness"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Pay X life.",
        ["product:PayLife", "verb:Pay", "variable:X"],
        [
            ("Pay", "lexeme:VerbLexeme/Pay/bare"),
            (" X", "vocab:Variable/X"),
            (" life", "form:pay_life/pay_life/2"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Pay {2}{B}.",
        ["product:PayMana", "verb:Pay", "scalar:2", "symbol:Black"],
        [
            ("Pay", "lexeme:VerbLexeme/Pay/bare"),
            (" {", "form:symbol_run/symbol_run/0/prefix"),
            ("2", "codec:ScalarNumber"),
            ("}{", "structural:SymbolRun/symbols/separator/uniform/0"),
            ("B", "vocab:FixedCostSymbol/Black"),
            ("}", "form:symbol_run/symbol_run/0/suffix"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Add {B}.",
        ["product:AddMana", "verb:Add", "symbol:Black"],
        [
            ("Add", "lexeme:VerbLexeme/Add/bare"),
            (" {", "form:symbol_run/symbol_run/0/prefix"),
            ("B", "vocab:FixedCostSymbol/Black"),
            ("}", "form:symbol_run/symbol_run/0/suffix"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Draw a card.",
        [
            "product:DrawCards",
            "verb:Draw",
            "product:SingularCardQuantity"
        ],
        [
            ("Draw", "lexeme:VerbLexeme/Draw/bare"),
            (" a", "form:singular_card_quantity/singular_card_quantity/0"),
            (
                " card",
                "form:singular_card_quantity/singular_card_quantity/1"
            ),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Draw two cards.",
        [
            "product:DrawCards",
            "verb:Draw",
            "product:FixedCardQuantity",
            "cardinal:2"
        ],
        [
            ("Draw", "lexeme:VerbLexeme/Draw/bare"),
            (" two", "codec:CardinalNumber"),
            (" cards", "form:fixed_card_quantity/fixed_card_quantity/1"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Draw X cards.",
        [
            "product:DrawCards",
            "verb:Draw",
            "product:VariableCardQuantity",
            "variable:X"
        ],
        [
            ("Draw", "lexeme:VerbLexeme/Draw/bare"),
            (" X", "vocab:Variable/X"),
            (
                " cards",
                "form:variable_card_quantity/variable_card_quantity/1"
            ),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Draw that many cards.",
        [
            "product:DrawCards",
            "verb:Draw",
            "product:AnaphoricCardQuantity"
        ],
        [
            ("Draw", "lexeme:VerbLexeme/Draw/bare"),
            (" that", "form:that_many/that_many/0"),
            (" many", "form:that_many/that_many/1"),
            (
                " cards",
                "form:anaphoric_card_quantity/anaphoric_card_quantity/1"
            ),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Draw cards equal to its toughness.",
        [
            "product:DrawCardsEqualTo",
            "verb:Draw",
            "possessive:Its",
            "characteristic:Toughness"
        ],
        [
            ("Draw", "lexeme:VerbLexeme/Draw/bare"),
            (" cards", "form:draw_cards_equal_to/draw_cards_equal_to/1"),
            (" equal", "form:scalar_equality/scalar_equality/0"),
            (" to", "form:scalar_equality/scalar_equality/1"),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (" toughness", "vocab:ScalarCharacteristic/Toughness"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Roll a six-sided die.",
        [
            "product:RollDice",
            "verb:Roll",
            "product:SingularDieObject",
            "die:SixSided"
        ],
        [
            ("Roll", "lexeme:VerbLexeme/Roll/bare"),
            (" a", "form:singular_die_object/singular_die_object/0"),
            (" six-sided", "vocab:DieShape/SixSided"),
            (" die", "form:singular_die_object/singular_die_object/2"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Roll two six-sided dice.",
        [
            "product:RollDice",
            "verb:Roll",
            "product:FixedDiceObject",
            "cardinal:2",
            "die:SixSided"
        ],
        [
            ("Roll", "lexeme:VerbLexeme/Roll/bare"),
            (" two", "codec:CardinalNumber"),
            (" six-sided", "vocab:DieShape/SixSided"),
            (" dice", "form:fixed_dice_object/fixed_dice_object/2"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Roll a d20.",
        ["product:RollDice", "verb:Roll", "product:D20Object"],
        [
            ("Roll", "lexeme:VerbLexeme/Roll/bare"),
            (" a", "form:d20_object/d20_object/0"),
            (" d20", "form:d20_object/d20_object/1"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Put a +1/+1 counter on target creature.",
        [
            "product:PutCounters",
            "verb:Put",
            "product:SingularCounterQuantity",
            "product:PositivePowerToughnessCounter",
            "scalar:1",
            "scalar:1",
            "declared:Creature",
        ],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (
                " a",
                "form:singular_counter_quantity/singular_counter_quantity/0"
            ),
            (
                " +",
                "form:positive_counter_magnitude/positive_counter_magnitude/0/affix"
            ),
            ("1", "codec:ScalarNumber"),
            (
                "/",
                "structural:PositivePowerToughnessCounter/magnitudes/separator/uniform/0"
            ),
            (
                "+",
                "form:positive_counter_magnitude/positive_counter_magnitude/0/affix"
            ),
            ("1", "codec:ScalarNumber"),
            (
                " counter",
                "form:singular_counter_quantity/singular_counter_quantity/2"
            ),
            (" on", "form:put_counters/put_counters/2"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Put a -1/-1 counter on target creature.",
        [
            "product:PutCounters",
            "verb:Put",
            "product:SingularCounterQuantity",
            "product:NegativePowerToughnessCounter",
            "scalar:1",
            "scalar:1",
            "declared:Creature",
        ],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (
                " a",
                "form:singular_counter_quantity/singular_counter_quantity/0"
            ),
            (
                " -",
                "form:negative_counter_magnitude/negative_counter_magnitude/0/affix"
            ),
            ("1", "codec:ScalarNumber"),
            (
                "/",
                "structural:NegativePowerToughnessCounter/magnitudes/separator/uniform/0"
            ),
            (
                "-",
                "form:negative_counter_magnitude/negative_counter_magnitude/0/affix"
            ),
            ("1", "codec:ScalarNumber"),
            (
                " counter",
                "form:singular_counter_quantity/singular_counter_quantity/2"
            ),
            (" on", "form:put_counters/put_counters/2"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Put a time counter on target creature.",
        [
            "product:PutCounters",
            "verb:Put",
            "product:SingularCounterQuantity",
            "counter:Time",
            "declared:Creature"
        ],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (
                " a",
                "form:singular_counter_quantity/singular_counter_quantity/0"
            ),
            (" time", "vocab:CounterName/Time"),
            (
                " counter",
                "form:singular_counter_quantity/singular_counter_quantity/2"
            ),
            (" on", "form:put_counters/put_counters/2"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Put two stun counters on it.",
        [
            "product:PutCounters",
            "verb:Put",
            "product:FixedCounterQuantity",
            "cardinal:2",
            "counter:Stun"
        ],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (" two", "codec:CardinalNumber"),
            (" stun", "vocab:CounterName/Stun"),
            (
                " counters",
                "form:fixed_counter_quantity/fixed_counter_quantity/2"
            ),
            (" on", "form:put_counters/put_counters/2"),
            (" it", "vocab:ObjectPronoun/It"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Put X time counters on target creature.",
        [
            "product:PutCounters",
            "verb:Put",
            "product:VariableCounterQuantity",
            "variable:X",
            "counter:Time",
            "declared:Creature"
        ],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (" X", "vocab:Variable/X"),
            (" time", "vocab:CounterName/Time"),
            (
                " counters",
                "form:variable_counter_quantity/variable_counter_quantity/2"
            ),
            (" on", "form:put_counters/put_counters/2"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Put that many charge counters on target creature.",
        [
            "product:PutCounters",
            "verb:Put",
            "product:AnaphoricCounterQuantity",
            "counter:Charge",
            "declared:Creature"
        ],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (" that", "form:that_many/that_many/0"),
            (" many", "form:that_many/that_many/1"),
            (" charge", "vocab:CounterName/Charge"),
            (
                " counters",
                "form:anaphoric_counter_quantity/anaphoric_counter_quantity/2"
            ),
            (" on", "form:put_counters/put_counters/2"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Remove X time counters from this card.",
        [
            "product:RemoveCounters",
            "verb:Remove",
            "product:VariableCounterQuantity",
            "variable:X",
            "counter:Time",
            "noun:Card"
        ],
        [
            ("Remove", "lexeme:VerbLexeme/Remove/bare"),
            (" X", "vocab:Variable/X"),
            (" time", "vocab:CounterName/Time"),
            (
                " counters",
                "form:variable_counter_quantity/variable_counter_quantity/2"
            ),
            (" from", "form:remove_counters/remove_counters/2"),
            (" this", "form:this_reference/this_reference/0"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Each player sacrifices a creature of their choice.",
        [
            "noun:Player",
            "declared:Sacrifice",
            "product:ChoiceObject",
            "declared:Creature"
        ],
        [
            ("Each", "form:each_reference/each_reference/0"),
            (" player", "lexeme:CommonNoun/Player/singular"),
            (
                " sacrifices",
                "lexeme:keyword_action/Sacrifice/third_person_singular"
            ),
            (" a", "form:indefinite_reference/a/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" of", "form:choice_object/choice_object/1"),
            (" their", "form:choice_object/choice_object/2"),
            (" choice", "form:choice_object/choice_object/3"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Target player discards two cards at random.",
        [
            "noun:Player",
            "declared:Discard",
            "product:RandomObject",
            "cardinal:2",
            "noun:Card"
        ],
        [
            (
                "Target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" player", "lexeme:CommonNoun/Player/singular"),
            (
                " discards",
                "lexeme:keyword_action/Discard/third_person_singular"
            ),
            (" two", "codec:CardinalNumber"),
            (" cards", "lexeme:CommonNoun/Card/plural"),
            (" at", "form:random_object/random_object/1"),
            (" random", "form:random_object/random_object/2"),
            (".", TERMINATOR),
        ]
    );
}

#[test]
fn target_and_card_name_boundaries_remain_grammatical_and_metadata_governed() {
    let parser = parser();
    let context = context();
    assert_selected(
        &parser,
        &context,
        "Put two stun counters on two target creatures.",
    );
    for malformed in [
        "Put two stun counters on target creatures.",
        "Put two stun counters on two target creature.",
        "Put two stun counters on the target of Context Card.",
    ] {
        assert!(
            parser.parse(malformed, &context).is_err(),
            "target remains a determiner with ordinary number agreement: {malformed:?}",
        );
    }

    let plus_two = ParseContext::new("+2 Mace", false, Onset::Consonant)
        .expect("opaque punctuation-initial card name is valid");
    assert_selected(&parser, &plus_two, "+2 Mace deals 2 damage to any target.");
    assert!(
        parser
            .parse("Mace deals 2 damage to any target.", &plus_two)
            .is_err(),
        "punctuation does not infer a shortened opaque card name",
    );

    let ordinary = ParseContext::new("Ordinary, Context", false, Onset::Vowel)
        .expect("ordinary punctuated card name is valid");
    assert_selected(
        &parser,
        &ordinary,
        "Ordinary, Context deals 2 damage to any target.",
    );
    assert!(
        parser
            .parse("Ordinary deals 2 damage to any target.", &ordinary)
            .is_err(),
        "a comma does not authorize ordinary-name abbreviation",
    );

    let legendary = ParseContext::new("Zoraline, Cosmos Caller", true, Onset::Consonant)
        .expect("legendary punctuated card name is valid");
    for text in [
        "Zoraline, Cosmos Caller deals 2 damage to any target.",
        "Zoraline deals 2 damage to any target.",
    ] {
        assert_selected(&parser, &legendary, text);
    }
}

#[test]
fn sentence_root_punctuation_and_document_terminators_keep_distinct_exact_claims() {
    let parser = parser();
    let context = context();
    let text = "Put two stun counters on it.";
    let sentence = parser.analyze_sentence(text, &context);
    assert!(sentence.selected().is_some(), "{sentence:?}");
    let root_claim = sentence
        .ownership()
        .expect("selected Sentence root has ownership")
        .parsed_claims()
        .last()
        .expect("Sentence root has punctuation claim");
    assert_eq!((root_claim.span().start, root_claim.span().end), (27, 28));
    assert_eq!(root_claim.stable_owner_id(), "root:Sentence/punctuation");

    let document = parser.analyze(text, &context);
    let terminator_claim = document
        .ownership()
        .expect("selected Ability document has ownership")
        .parsed_claims()
        .last()
        .expect("Ability document has terminator claim");
    assert_eq!(
        (terminator_claim.span().start, terminator_claim.span().end),
        (27, 28),
    );
    assert_eq!(
        terminator_claim.stable_owner_id(),
        "structural:Sentences/sentences/terminator/0",
    );
    for malformed in [
        "Put two stun counters on it .",
        "Put two stun counters on it  .",
    ] {
        assert!(
            parser.parse_sentence(malformed, &context).is_err(),
            "Sentence punctuation remains left-adjacent: {malformed:?}",
        );
    }
}

#[test]
fn typed_complements_reject_reciprocal_agreement_amount_number_and_determiners() {
    let parser = parser();
    let context = context();
    for text in [
        "Deal two damage to any target.",
        "Deal 2 damages to any target.",
        "It deal 2 damage to any target.",
        "Gain two life.",
        "Gain 2 lives.",
        "You gains 2 life.",
        "Lose two life.",
        "Pay 2 lives.",
        "Pay 2.",
        "Pay {2} mana.",
        "Add B.",
        "Draw 2 cards.",
        "Draw two card.",
        "Draw card.",
        "Draw cards equals to its power.",
        "Draw cards equal its power.",
        "Roll one die.",
        "Roll a dice.",
        "Roll two die.",
        "Roll 2 dice.",
        "Put 2 +1/+1 counters on target creature.",
        "Put a +1/+1 counters on target creature.",
        "Put two +1/+1 counter on target creature.",
        "Put a +1/1 counter on target creature.",
        "Put a 1/+1 counter on target creature.",
        "Put a +1 /+1 counter on target creature.",
        "Put a +1/ +1 counter on target creature.",
        "Put a +1//+1 counter on target creature.",
        "Put a +1/+1 counter from target creature.",
        "Remove a time counter on this card.",
        "Create token.",
        "Each player sacrifices a creature of your choice.",
        "Target player discards at random two cards.",
        "Destroy target creatures.",
        "Destroy two target creature.",
        "Destroy the target of Context Card.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "malformed reciprocal must reject {text:?}",
        );
    }
}
