use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::parser::TextSpan;
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
            "/synthetic/actions/Exile.ron",
            r#"KeywordAction(name:"Exile",spelling:"exile",grammar:Verb(bare:"exile",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Regenerate.ron",
            r#"KeywordAction(name:"Regenerate",spelling:"regenerate",grammar:Verb(bare:"regenerate",participle:"regenerated",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Cast.ron",
            r#"KeywordAction(name:"Cast",spelling:"cast",grammar:Verb(bare:"cast",participle:"cast",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Search.ron",
            r#"KeywordAction(name:"Search",spelling:"search",grammar:Verb(bare:"search",third_person:"searches",valence:Custom(shapes:[[ObjectNounPhrase],[Literal("for"),ObjectNounPhrase],[ObjectNounPhrase,Literal("for"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Reveal.ron",
            r#"KeywordAction(name:"Reveal",spelling:"reveal",grammar:Verb(bare:"reveal",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Shuffle.ron",
            r#"KeywordAction(name:"Shuffle",spelling:"shuffle",grammar:Verb(bare:"shuffle",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Attach.ron",
            r#"KeywordAction(name:"Attach",spelling:"attach",grammar:Verb(bare:"attach",third_person:"attaches",valence:Custom(shapes:[[ObjectNounPhrase,Literal("to"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Exchange.ron",
            r#"KeywordAction(name:"Exchange",spelling:"exchange",grammar:Verb(bare:"exchange",valence:Custom(shapes:[[ObjectNounPhrase],[ObjectNounPhrase,Literal("with"),ObjectNounPhrase],[ObjectNounPhrase,Literal("for"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Vote.ron",
            r#"KeywordAction(name:"Vote",spelling:"vote",grammar:Verb(bare:"vote",valence:Custom(shapes:[[],[ObjectNounPhrase],[Literal("for"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/types/Creature.ron",
            r#"Type(name:"Creature",spelling:"creature",grammar:Noun(singular:"creature"))"#,
        ),
        (
            "/synthetic/types/Artifact.ron",
            r#"Type(name:"Artifact",spelling:"artifact",grammar:Noun(singular:"artifact"))"#,
        ),
        (
            "/synthetic/types/Land.ron",
            r#"Type(name:"Land",spelling:"land",grammar:Noun(singular:"land"))"#,
        ),
        (
            "/synthetic/types/Instant.ron",
            r#"Type(name:"Instant",spelling:"instant",grammar:Noun(singular:"instant"))"#,
        ),
        (
            "/synthetic/types/Sorcery.ron",
            r#"Type(name:"Sorcery",spelling:"sorcery",grammar:Noun(singular:"sorcery"))"#,
        ),
        (
            "/synthetic/types/Planeswalker.ron",
            r#"Type(name:"Planeswalker",spelling:"planeswalker",grammar:Noun(singular:"planeswalker"))"#,
        ),
        (
            "/synthetic/subtypes/Equipment.ron",
            r#"Subtype(category:Artifact,name:"Equipment",spelling:"Equipment",grammar:Noun(singular:"Equipment",plural:"Equipment"))"#,
        ),
        (
            "/synthetic/subtypes/Sliver.ron",
            r#"Subtype(category:Creature,name:"Sliver",spelling:"Sliver",grammar:Noun(singular:"Sliver"))"#,
        ),
        (
            "/synthetic/subtypes/Goblin.ron",
            r#"Subtype(category:Creature,name:"Goblin",spelling:"Goblin",grammar:Noun(singular:"Goblin"))"#,
        ),
        (
            "/synthetic/subtypes/Forest.ron",
            r#"Subtype(category:Land,name:"Forest",spelling:"Forest",grammar:Noun(singular:"Forest"))"#,
        ),
        (
            "/synthetic/subtypes/Mountain.ron",
            r#"Subtype(category:Land,name:"Mountain",spelling:"Mountain",grammar:Noun(singular:"Mountain"))"#,
        ),
        (
            "/synthetic/actions/Activate.ron",
            r#"KeywordAction(name:"Activate",spelling:"activate",grammar:Verb(bare:"activate",valence:Transitive))"#,
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
    *predicate.clone()
}

fn declarative_atomic(parser: &Parser, context: &ParseContext<'_>, text: &str) -> VerbPhrase {
    let Sentence::Declarative(declarative) = parser
        .parse_sentence(text, context)
        .unwrap_or_else(|error| panic!("declarative product must parse {text:?}: {error:?}"))
    else {
        panic!("finite predicate has a declarative envelope: {text:?}")
    };
    let Clause::Finite(finite) = declarative.clause.as_ref() else {
        panic!("one finite predicate has a plain finite clause: {text:?}")
    };
    let FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
        panic!("one finite predicate has a plain finite clause: {text:?}")
    };
    let Predicate::Atomic(predicate) = clause.predicate() else {
        panic!("one finite predicate has an atomic envelope: {text:?}")
    };
    *predicate.clone()
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

#[derive(Default)]
struct Task10Visitor(Vec<&'static str>);

impl Visitor for Task10Visitor {
    fn visit_transitive_requirement_predicate_value(
        &mut self,
        value: &TransitiveRequirementPredicateValue,
    ) {
        self.0.push("transitive-requirement");
        deckmaste_english_v2::visit::walk_transitive_requirement_predicate_value(self, value);
    }

    fn visit_blocked_except_by_status_value(&mut self, value: &BlockedExceptByStatusValue) {
        self.0.push("blocked-except-by");
        deckmaste_english_v2::visit::walk_blocked_except_by_status_value(self, value);
    }

    fn visit_flexible_mana(&mut self, value: &FlexibleMana) {
        self.0.push("flexible-mana");
        deckmaste_english_v2::visit::walk_flexible_mana(self, value);
    }

    fn visit_all_colors_value(&mut self, value: &AllColorsValue) {
        self.0.push("all-colors");
        deckmaste_english_v2::visit::walk_all_colors_value(self, value);
    }

    fn visit_from_anywhere(&mut self, value: &FromAnywhere) {
        self.0.push("from-anywhere");
        deckmaste_english_v2::visit::walk_from_anywhere(self, value);
    }

    fn visit_enter_with_counters(&mut self, value: &EnterWithCounters) {
        self.0.push("enter-with-counters");
        deckmaste_english_v2::visit::walk_enter_with_counters(self, value);
    }

    fn visit_preposed_as(&mut self, value: &PreposedAs) {
        self.0.push("as-clause");
        deckmaste_english_v2::visit::walk_preposed_as(self, value);
    }

    fn visit_prevent_damage(&mut self, value: &PreventDamage) {
        self.0.push("prevent-damage");
        deckmaste_english_v2::visit::walk_prevent_damage(self, value);
    }

    fn visit_predicate_coordination(&mut self, value: &PredicateCoordination) {
        self.0.push("predicate-coordination");
        deckmaste_english_v2::visit::walk_predicate_coordination(self, value);
    }

    fn visit_clause_coordination(&mut self, value: &ClauseCoordination) {
        self.0.push("clause-coordination");
        deckmaste_english_v2::visit::walk_clause_coordination(self, value);
    }
}

fn task10_visits(parser: &Parser, context: &ParseContext<'_>, text: &str) -> Vec<&'static str> {
    task10_visits_with_specificity(parser, context, text, false)
}

fn task10_visits_with_specificity(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    permits_specificity: bool,
) -> Vec<&'static str> {
    let ability = assert_selected_with_specificity(parser, context, text, permits_specificity);
    let mut visitor = Task10Visitor::default();
    visitor.visit_ability(&ability);
    visitor.0
}

#[test]
fn task10_builds_keep_new_products_in_the_existing_typed_algebra() {
    let parser = parser();
    let context = context();

    let flexible_mana = VerbPhrase::FlexibleMana(FlexibleMana {
        amount: CardinalQuantity::Cardinal(CardinalQuantityValue {
            number: CardinalNumber { magnitude: 1 },
        }),
        kind: FlexibleManaKind::Color,
    });
    let expected = Sentence::Imperative(
        Imperative::new(Box::new(Predicate::Atomic(Box::new(flexible_mana.clone()))))
            .expect("flexible mana is a valid bare imperative predicate"),
    );
    let text = "Add one mana of any color.";
    assert_eq!(parser.parse_sentence(text, &context), Ok(expected.clone()));
    assert_eq!(expected.render(&context, parser.environment()), text);
    assert_eq!(imperative_atomic(&parser, &context, text), flexible_mana,);
    assert_eq!(
        exact_claim_trace(&parser, &context, text),
        [
            ("Add".to_owned(), "lexeme:VerbLexeme/Add/bare".to_owned()),
            (" one".to_owned(), "codec:CardinalNumber".to_owned()),
            (
                " mana".to_owned(),
                "form:flexible_mana/flexible_mana/2".to_owned(),
            ),
            (
                " of".to_owned(),
                "form:flexible_mana/flexible_mana/3".to_owned(),
            ),
            (
                " any".to_owned(),
                "form:flexible_mana/flexible_mana/4".to_owned(),
            ),
            (
                " color".to_owned(),
                "vocab:FlexibleManaKind/Color".to_owned(),
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
    );

    let Sentence::Declarative(requirement) = parser
        .parse_sentence(
            "Target creature attacks target opponent this turn if able.",
            &context,
        )
        .expect("transitive combat requirement parses")
    else {
        panic!("transitive requirement has a declarative envelope")
    };
    let Clause::Finite(requirement) = requirement.clause.as_ref() else {
        panic!("transitive requirement has an ordinary finite clause")
    };
    let FiniteClause::PlainFiniteClause(requirement) = requirement.as_ref() else {
        panic!("transitive requirement has an ordinary finite clause")
    };
    assert!(matches!(
        requirement.predicate(),
        Predicate::TransitiveRequirement(_)
    ));

    let Sentence::Declarative(all_colors) = parser
        .parse_sentence("This permanent is all colors.", &context)
        .expect("all-colors complement parses")
    else {
        panic!("all-colors complement has a declarative envelope")
    };
    let Clause::Copular(all_colors) = all_colors.clause.as_ref() else {
        panic!("all-colors complement uses the ordinary copular clause")
    };
    let CopularClause::CopularClause(CopularClauseValue { complement, .. }) = all_colors.as_ref();
    assert!(matches!(
        complement.as_ref(),
        PredicativeComplement::AllColors(_)
    ));

    let as_ability = assert_selected(
        &parser,
        &context,
        "As this artifact enters, choose a color.",
    );
    let Ability::Plain(Plain { body }) = as_ability else {
        panic!("as-entry witness has an ordinary ability envelope")
    };
    let AbilityBody::Sentences(sentences) = body.as_ref() else {
        panic!("as-entry witness has an ordinary sentence body")
    };
    let [Sentence::Attached(Attached { attachment })] = sentences.sentences() else {
        panic!("as-entry witness has one typed attached sentence")
    };
    assert!(matches!(
        attachment.as_ref(),
        ClauseAttachment::PreposedAs(_)
    ));

    let coordinated = assert_selected(
        &parser,
        &context,
        "This permanent is all colors and this creature becomes tapped.",
    );
    let Ability::Plain(Plain { body }) = coordinated else {
        panic!("clause coordination has an ordinary ability envelope")
    };
    let AbilityBody::Sentences(sentences) = body.as_ref() else {
        panic!("clause coordination has an ordinary sentence body")
    };
    let [Sentence::Declarative(declarative)] = sentences.sentences() else {
        panic!("clause coordination has one declarative sentence")
    };
    let Clause::Coordination(coordination) = declarative.clause.as_ref() else {
        panic!("new clause families share the n-ary coordination product")
    };
    let ClauseCoordination::AndClauseCoordination(coordination) = coordination.as_ref() else {
        panic!("the witness retains its and coordinator")
    };
    assert!(matches!(
        coordination.members(),
        [CoordinatedClause::Copular(_), CoordinatedClause::Finite(_)]
    ));
}

#[test]
fn task10_combat_frames_keep_active_valence_passive_agents_and_if_able_distinct() {
    let parser = parser();
    let context = context();

    for (text, expected_path_member) in [
        (
            "Whenever this creature attacks, draw a card.",
            "VerbPhraseIntransitivePredicate",
        ),
        (
            "Whenever this creature attacks a player, draw a card.",
            "VerbPhraseTransitivePredicate",
        ),
        (
            "Whenever this creature blocks a creature, draw a card.",
            "VerbPhraseTransitivePredicate",
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        assert_selected_with_specificity(&parser, &context, text, true);
        let decision = analysis
            .decision()
            .expect("combat witness has a selection decision");
        let ordinal = decision
            .selected()
            .expect("combat witness has a selected production derivation");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("selected ordinal names one production derivation");
        assert!(
            selected
                .construction_path()
                .iter()
                .any(|name| name == expected_path_member),
            "{text:?}: {selected:?}",
        );
    }

    assert_eq!(
        task10_visits_with_specificity(
            &parser,
            &context,
            "Whenever this creature blocks or becomes blocked by a creature, draw a card.",
            true,
        ),
        ["predicate-coordination"],
    );
    assert_eq!(
        task10_visits(
            &parser,
            &context,
            "Target creature attacks target opponent this turn if able.",
        ),
        ["transitive-requirement"],
    );
    assert_eq!(
        task10_visits(
            &parser,
            &context,
            "This creature can't be blocked except by two or more creatures.",
        ),
        ["blocked-except-by"],
    );

    for crossed in [
        "Whenever this creature block a creature, draw a card.",
        "Target creature attack target opponent this turn if able.",
        "This creature can't be blocked except two or more creatures.",
    ] {
        assert!(
            parser.parse(crossed, &context).is_err(),
            "combat agreement and passive-agent syntax reject {crossed:?}",
        );
    }
}

#[test]
fn task10_damage_life_mana_and_continuous_state_use_typed_ordinary_products() {
    let parser = parser();
    let context = context();

    for text in [
        "This creature deals 3 damage to any target.",
        "You gain 3 life.",
        "Target opponent loses 2 life.",
        "Pay 4 life.",
        "Add {R}{R}{R}.",
        "Other creatures you control get +1/+1.",
        "This creature becomes tapped.",
    ] {
        assert_selected(&parser, &context, text);
    }
    assert_eq!(
        task10_visits(&parser, &context, "Add one mana of any color."),
        ["flexible-mana"],
    );
    assert_eq!(
        task10_visits(&parser, &context, "This permanent is all colors."),
        ["all-colors"],
    );
    assert_eq!(
        task10_visits(
            &parser,
            &context,
            "This permanent is all colors and this creature becomes tapped.",
        ),
        ["clause-coordination", "all-colors"],
    );

    for crossed in [
        "Add one mana of any colors.",
        "This permanent is all color.",
        "This creature become tapped.",
    ] {
        assert!(
            parser.parse(crossed, &context).is_err(),
            "resource and continuous-state morphology reject {crossed:?}",
        );
    }
}

#[test]
fn task10_replacement_entry_and_skip_surfaces_reuse_clause_and_predicate_algebra() {
    let parser = parser();
    let context = context();

    assert_eq!(
        task10_visits(
            &parser,
            &context,
            "If a card would be put into your graveyard from anywhere, exile it instead.",
        ),
        ["from-anywhere"],
    );
    assert_eq!(
        task10_visits(
            &parser,
            &context,
            "This creature enters with two +1/+1 counters on it.",
        ),
        ["enter-with-counters"],
    );
    assert_eq!(
        task10_visits(
            &parser,
            &context,
            "As this artifact enters, choose a color.",
        ),
        ["as-clause"],
    );
    assert_selected(&parser, &context, "Players skip their untap steps.");
    assert_selected_with_specificity(
        &parser,
        &context,
        "If a player would draw a card, that player skips that draw instead.",
        true,
    );

    for crossed in [
        "If a card would be put into your graveyard anywhere, exile it instead.",
        "This creature enters two +1/+1 counters on it.",
        "As this artifact enters choose a color.",
        "Players skips their untap steps.",
    ] {
        assert!(
            parser.parse(crossed, &context).is_err(),
            "replacement and skip boundaries reject {crossed:?}",
        );
    }
}

#[test]
fn task10_prevention_restriction_requirement_permission_exception_and_if_able_remain_linguistic() {
    let parser = parser();
    let context = context();

    assert_eq!(
        task10_visits(
            &parser,
            &context,
            "Prevent the next 3 damage that would be dealt to target creature this turn.",
        ),
        ["prevent-damage"],
    );
    for text in [
        "If a source would deal damage to this creature, prevent that damage.",
        "Damage can't be prevented.",
        "This creature can't attack or block.",
        "This creature attacks each combat if able.",
        "You may cast spells as though they had flash.",
        "This creature can't attack unless you control a Forest.",
        "This creature can't be blocked except by two or more creatures.",
    ] {
        assert_selected(&parser, &context, text);
    }

    for crossed in [
        "Prevent next 3 damage that would be dealt to target creature this turn.",
        "Damage can't be prevent.",
        "This creature attacks each combat able.",
        "This creature can't attack unless control a Forest.",
    ] {
        assert!(
            parser.parse(crossed, &context).is_err(),
            "prevention and deontic surface boundaries reject {crossed:?}",
        );
    }
}

#[derive(Default)]
struct Task9ObjectVisitor(Vec<&'static str>);

impl Visitor for Task9ObjectVisitor {
    fn visit_declared_to_object_predicate(&mut self, value: &DeclaredToObjectPredicate) {
        self.0.push("declared-to-object");
        deckmaste_english_v2::visit::walk_declared_to_object_predicate(self, value);
    }

    fn visit_declared_for_object_predicate(&mut self, value: &DeclaredForObjectPredicate) {
        self.0.push("declared-for-object");
        deckmaste_english_v2::visit::walk_declared_for_object_predicate(self, value);
    }

    fn visit_maximum_hand_size_reference(&mut self, value: &MaximumHandSizeReference) {
        self.0.push("maximum-hand-size");
        deckmaste_english_v2::visit::walk_maximum_hand_size_reference(self, value);
    }

    fn visit_predicative_scalar_value(&mut self, value: &PredicativeScalarValue) {
        self.0.push("predicative-scalar");
        deckmaste_english_v2::visit::walk_predicative_scalar_value(self, value);
    }

    fn visit_power_toughness_modifier(&mut self, value: &PowerToughnessModifier) {
        self.0.push("power-toughness-modifier");
        deckmaste_english_v2::visit::walk_power_toughness_modifier(self, value);
    }

    fn visit_token_copy_reference(&mut self, value: &TokenCopyReference) {
        self.0.push("token-copy");
        deckmaste_english_v2::visit::walk_token_copy_reference(self, value);
    }

    fn visit_described_token_reference(&mut self, value: &DescribedTokenReference) {
        self.0.push("described-token");
        deckmaste_english_v2::visit::walk_described_token_reference(self, value);
    }

    fn visit_get_power_toughness(&mut self, value: &GetPowerToughness) {
        self.0.push("get-power-toughness");
        deckmaste_english_v2::visit::walk_get_power_toughness(self, value);
    }

    fn visit_have_base_power_toughness(&mut self, value: &HaveBasePowerToughness) {
        self.0.push("have-base-power-toughness");
        deckmaste_english_v2::visit::walk_have_base_power_toughness(self, value);
    }

    fn visit_quoted_ability_predicate(&mut self, value: &QuotedAbilityPredicate) {
        self.0.push("quoted-ability");
        deckmaste_english_v2::visit::walk_quoted_ability_predicate(self, value);
    }

    fn visit_transitive_predicate(&mut self, value: &TransitivePredicate) {
        match value.head {
            TransitiveVerb::Lexeme(CoreTransitiveVerb::Copy) => self.0.push("copy"),
            TransitiveVerb::Lexeme(CoreTransitiveVerb::Flip) => self.0.push("flip"),
            TransitiveVerb::Lexeme(CoreTransitiveVerb::Lose) => self.0.push("lose-abilities"),
            TransitiveVerb::Lexeme(CoreTransitiveVerb::Unattach) => self.0.push("unattach"),
            _ => {}
        }
        if matches!(
            &value.head,
            TransitiveVerb::Declaration(head) if head.id().name() == "Exchange"
        ) {
            self.0.push("exchange");
        }
        deckmaste_english_v2::visit::walk_transitive_predicate(self, value);
    }
}

#[test]
fn task9_declared_to_object_frame_parses_attach_without_a_card_specific_rule() {
    let parser = parser();
    let context = context();
    let text = "Attach target Equipment to target creature.";
    let ability = assert_selected(&parser, &context, text);
    assert!(matches!(
        imperative_atomic(&parser, &context, text),
        VerbPhrase::DeclaredToObjectPredicate(DeclaredToObjectPredicate { .. })
    ));

    let mut visitor = Task9ObjectVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["declared-to-object"]);

    assert_eq!(
        exact_claim_trace(&parser, &context, text),
        [
            (
                "Attach".to_owned(),
                "lexeme:keyword_action/Attach/bare".to_owned(),
            ),
            (
                " target".to_owned(),
                "form:target_determiner_phrase/target_determiner_phrase/0".to_owned(),
            ),
            (
                " Equipment".to_owned(),
                "lexeme:artifact_subtype/Equipment/singular".to_owned(),
            ),
            (
                " to".to_owned(),
                "form:declared_to_object_predicate/declared_to_object_predicate/2".to_owned(),
            ),
            (
                " target".to_owned(),
                "form:target_determiner_phrase/target_determiner_phrase/0".to_owned(),
            ),
            (
                " creature".to_owned(),
                "lexeme:type/Creature/singular".to_owned(),
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
    );
}

#[test]
fn task9_quote_boundary_recurses_only_through_an_ordinary_ability() {
    let parser = parser();
    let context = context();
    let text = r#"All Slivers have "When this permanent enters, draw a card.""#;
    let ability = assert_selected_with_specificity(&parser, &context, text, true);
    let Ability::Plain(Plain { body }) = &ability else {
        panic!("quoted complement witness has an ordinary ability envelope")
    };
    let AbilityBody::QuoteTerminatedStatement(statement) = body.as_ref() else {
        panic!("quoted complement witness has the derived quote terminator envelope")
    };
    let VerbPhrase::QuotedAbilityPredicate(predicate) = statement.predicate() else {
        panic!("quote terminator envelope is restricted to the shared quoted predicate")
    };
    let QuotedAbility::QuotedAbility(quoted) = predicate.ability.as_ref();
    let Ability::Triggered(_) = quoted.ability.as_ref() else {
        panic!("quoted complement stores the ordinary triggered ability AST")
    };

    let mut visitor = Task9ObjectVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["quoted-ability"]);
    let claims = exact_claim_trace(&parser, &context, text);
    assert!(
        claims.iter().any(|(surface, owner)| {
            surface == " \"" && owner == "form:quoted_ability/quoted_ability/0"
        }),
        "{claims:#?}",
    );
    assert!(claims.iter().any(|(surface, owner)| {
        surface == "\"" && owner == "form:quoted_ability/quoted_ability/1/affix"
    }));
}

#[test]
fn task9_quoted_plan10_interiors_remain_exact_ordinary_failures() {
    let parser = parser();
    let context = context();
    let documents = [
        r#"Artifacts you control have "Ward—Pay 2 life.""#,
        r#"All creatures have "{2}: Put a +1/+1 counter on this creature. You gain 1 life. This creature shares its feelings. Activate only as a sorcery. (Creatures can continue to share their feelings.)""#,
    ];
    let observed = documents
        .iter()
        .map(|text| {
            let analysis = parser.analyze(text, &context);
            assert_ne!(
                analysis.outcome(),
                deckmaste_english_v2::parser::ParseAnalysisOutcome::Selected,
                "Plan 10 quote interior must not select: {text:?}",
            );
            if let Some(decision) = analysis.decision() {
                assert!(decision.candidates().iter().all(|candidate| {
                    !candidate
                        .construction_path()
                        .iter()
                        .any(|identity| identity == "VerbPhraseQuotedAbilityPredicate")
                }));
            }
            let error = analysis
                .into_parse_result()
                .expect_err("Plan 10 quote interior remains an ordinary failure");
            let deckmaste_english_v2::parser::ParseError::Failure { span, .. } = error else {
                panic!("Plan 10 quote interior has exact ordinary failure class: {error:?}")
            };
            (span.start, span.end, text[span.start..span.end].to_owned())
        })
        .collect::<Vec<_>>();
    assert_eq!(
        observed,
        [
            (28, 38, "Ward—Pay".to_owned()),
            (94, 100, "shares".to_owned()),
        ]
    );
}

#[test]
fn task9_closed_information_heads_parse_copy_and_flip_without_action_declarations() {
    let parser = parser();
    let context = context();
    for (text, expected_head, expected_visit) in [
        (
            "Copy target instant or sorcery spell.",
            CoreTransitiveVerb::Copy,
            "copy",
        ),
        ("Flip a coin.", CoreTransitiveVerb::Flip, "flip"),
        (
            "Unattach that Equipment.",
            CoreTransitiveVerb::Unattach,
            "unattach",
        ),
    ] {
        let ability = assert_selected(&parser, &context, text);
        let VerbPhrase::TransitivePredicate(predicate) = imperative_atomic(&parser, &context, text)
        else {
            panic!("{text:?} stores the shared transitive frame")
        };
        assert_eq!(
            predicate.head,
            TransitiveVerb::Lexeme(expected_head),
            "{text:?}",
        );
        let mut visitor = Task9ObjectVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, [expected_visit], "{text:?}");
    }
}

#[test]
fn task9_exchange_uses_declared_object_valence_and_a_typed_control_reference() {
    let parser = parser();
    let context = context();
    let text = "Exchange control of two target creatures.";
    let ability = assert_selected(&parser, &context, text);
    let VerbPhrase::TransitivePredicate(predicate) = imperative_atomic(&parser, &context, text)
    else {
        panic!("exchange uses the shared declared transitive frame")
    };
    let TransitiveVerb::Declaration(head) = &predicate.head else {
        panic!("exchange retains its authored keyword-action identity")
    };
    assert_eq!(head.id().name(), "Exchange");
    assert!(matches!(predicate.object, Object::ObjectNominal(_)));
    let mut visitor = Task9ObjectVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["exchange"]);
}

#[test]
fn task9_vote_uses_declared_for_object_valence_and_productive_common_noun_choices() {
    let parser = parser();
    let context = context();
    let text = "Starting with you, each player votes for death or taxes.";
    let ability = assert_selected(&parser, &context, text);
    let Ability::Plain(Plain { body }) = &ability else {
        panic!("vote witness has an ordinary ability envelope")
    };
    let AbilityBody::Sentences(sentences) = body.as_ref() else {
        panic!("vote witness has an ordinary sentence body")
    };
    let Sentence::Attached(Attached { attachment }) = &sentences.sentences()[0] else {
        panic!("vote order stays a typed preposed clause attachment")
    };
    let ClauseAttachment::StartingWithYou(_) = attachment.as_ref() else {
        panic!("vote order stays a typed preposed clause attachment")
    };
    let mut visitor = Task9ObjectVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["declared-for-object"]);

    let productive = "Starting with you, each player votes for card or token.";
    let ability = assert_selected(&parser, &context, productive);
    let mut visitor = Task9ObjectVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["declared-for-object"]);
}

#[test]
fn task9_maximum_hand_size_is_a_typed_copular_scalar_statement() {
    let parser = parser();
    let context = context();
    let text = "Your maximum hand size is seven.";
    let ability = assert_selected(&parser, &context, text);
    let Ability::Plain(Plain { body }) = &ability else {
        panic!("maximum hand size witness has an ordinary ability envelope")
    };
    let AbilityBody::Sentences(sentences) = body.as_ref() else {
        panic!("maximum hand size witness has an ordinary sentence body")
    };
    let Sentence::Declarative(declarative) = &sentences.sentences()[0] else {
        panic!("maximum hand size witness is declarative")
    };
    let Clause::Copular(copular) = declarative.clause.as_ref() else {
        panic!("maximum hand size witness is a copular clause")
    };
    let CopularClause::CopularClause(value) = copular.as_ref();
    assert!(matches!(
        value.complement.as_ref(),
        PredicativeComplement::Scalar(_)
    ));
    let mut visitor = Task9ObjectVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["maximum-hand-size", "predicative-scalar"]);
}

#[test]
fn task9_token_descriptions_share_typed_power_toughness_and_copy_constituents() {
    let parser = parser();
    let context = context();
    for (text, expected_visit) in [
        ("Create a 1/1 red Goblin creature token.", "described-token"),
        (
            "Create a token that's a copy of target creature.",
            "token-copy",
        ),
    ] {
        let ability = assert_selected(&parser, &context, text);
        let VerbPhrase::TransitivePredicate(predicate) = imperative_atomic(&parser, &context, text)
        else {
            panic!("{text:?} uses Create's authored transitive frame")
        };
        let TransitiveVerb::Declaration(head) = &predicate.head else {
            panic!("{text:?} retains Create's declaration identity")
        };
        assert_eq!(head.id().name(), "Create", "{text:?}");
        let mut visitor = Task9ObjectVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, [expected_visit], "{text:?}");
    }
}

#[test]
fn task9_power_toughness_predicates_keep_modifier_and_base_value_frames_distinct() {
    let parser = parser();
    let context = context();
    for (text, expected_visit) in [
        (
            "Target creature gets +3/+1 until end of turn.",
            "get-power-toughness",
        ),
        (
            "This creature has base power and toughness 4/4.",
            "have-base-power-toughness",
        ),
    ] {
        let ability = assert_selected(&parser, &context, text);
        let mut visitor = Task9ObjectVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, [expected_visit], "{text:?}");
    }
    assert!(matches!(
        declarative_atomic(
            &parser,
            &context,
            "Target creature gets +3/+1 until end of turn."
        ),
        VerbPhrase::GetPowerToughness(_)
    ));
    assert!(matches!(
        declarative_atomic(
            &parser,
            &context,
            "This creature has base power and toughness 4/4."
        ),
        VerbPhrase::HaveBasePowerToughness(_)
    ));
}

#[derive(Default)]
struct Task10aPowerToughnessVisitor(Vec<&'static str>);

impl Visitor for Task10aPowerToughnessVisitor {
    fn visit_clause_coordination(&mut self, value: &ClauseCoordination) {
        self.0.push("clause-coordination");
        deckmaste_english_v2::visit::walk_clause_coordination(self, value);
    }

    fn visit_positive_power_toughness_magnitude(
        &mut self,
        value: &PositivePowerToughnessMagnitude,
    ) {
        self.0.push("positive-magnitude");
        deckmaste_english_v2::visit::walk_positive_power_toughness_magnitude(self, value);
    }

    fn visit_negative_power_toughness_magnitude(
        &mut self,
        value: &NegativePowerToughnessMagnitude,
    ) {
        self.0.push("negative-magnitude");
        deckmaste_english_v2::visit::walk_negative_power_toughness_magnitude(self, value);
    }
}

#[test]
fn task10a_negative_adjustments_build_render_visit_and_claim_the_typed_sign_product() {
    let parser = parser();
    let context = context();
    let text = "Target creature gets -1/-1 until end of turn.";
    let ability = assert_selected(&parser, &context, text);
    let VerbPhrase::GetPowerToughness(GetPowerToughness {
        adjustment: PowerToughnessAdjustment::PowerToughnessAdjustment(adjustment),
        duration: Some(PredicateDuration::UntilEndOfTurn),
    }) = declarative_atomic(&parser, &context, text)
    else {
        panic!("negative adjustment stays in the ordinary typed gets frame")
    };
    assert_eq!(
        adjustment.magnitudes(),
        [
            PowerToughnessAdjustmentMagnitude::NegativePowerToughnessMagnitude(
                NegativePowerToughnessMagnitude {
                    amount: Amount::Number(NumberAmount {
                        number: ScalarNumber { magnitude: 1 },
                    }),
                },
            ),
            PowerToughnessAdjustmentMagnitude::NegativePowerToughnessMagnitude(
                NegativePowerToughnessMagnitude {
                    amount: Amount::Number(NumberAmount {
                        number: ScalarNumber { magnitude: 1 },
                    }),
                },
            ),
        ],
    );
    assert_eq!(ability.render(&context, parser.environment()), text);

    let mut visitor = Task10aPowerToughnessVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["negative-magnitude", "negative-magnitude"]);
    assert_eq!(
        exact_claim_trace(&parser, &context, text),
        [
            (
                "Target".to_owned(),
                "form:target_determiner_phrase/target_determiner_phrase/0".to_owned(),
            ),
            (
                " creature".to_owned(),
                "lexeme:type/Creature/singular".to_owned(),
            ),
            (
                " gets".to_owned(),
                "lexeme:VerbLexeme/Get/third_person_singular".to_owned(),
            ),
            (
                " -".to_owned(),
                "form:negative_power_toughness_magnitude/negative_power_toughness_magnitude/0/affix"
                    .to_owned(),
            ),
            ("1".to_owned(), "codec:ScalarNumber".to_owned()),
            (
                "/".to_owned(),
                "structural:PowerToughnessAdjustmentValue/magnitudes/separator/uniform/0"
                    .to_owned(),
            ),
            (
                "-".to_owned(),
                "form:negative_power_toughness_magnitude/negative_power_toughness_magnitude/0/affix"
                    .to_owned(),
            ),
            ("1".to_owned(), "codec:ScalarNumber".to_owned()),
            (
                " until end of turn".to_owned(),
                "vocab:PredicateDuration/UntilEndOfTurn".to_owned(),
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
    );
}

#[test]
fn task10a_fixed_variable_asymmetric_and_crossed_adjustments_share_existing_amounts() {
    let parser = parser();
    let context = context();
    for (text, expected_visits) in [
        (
            "All creatures get -1/-0 until end of turn.",
            &["negative-magnitude", "negative-magnitude"][..],
        ),
        (
            "Target creature gets -X/-X until end of turn.",
            &["negative-magnitude", "negative-magnitude"][..],
        ),
        (
            "Target creature gets +X/-X until end of turn.",
            &["positive-magnitude", "negative-magnitude"][..],
        ),
        (
            "Target creature gets -X/+X until end of turn.",
            &["negative-magnitude", "positive-magnitude"][..],
        ),
    ] {
        let ability = assert_selected(&parser, &context, text);
        let mut visitor = Task10aPowerToughnessVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, expected_visits, "{text:?}");
    }

    let text = "This creature gets -1/-1 and target creature gets +1/+1.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = Task10aPowerToughnessVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.0,
        [
            "clause-coordination",
            "negative-magnitude",
            "negative-magnitude",
            "positive-magnitude",
            "positive-magnitude",
        ],
    );
}

#[test]
fn task10a_adjustment_sign_slash_pairing_and_agreement_boundaries_are_reciprocal() {
    let parser = parser();
    let context = context();

    for retained in [
        "Target creature gets +1/+1 until end of turn.",
        "This creature has base power and toughness 4/4.",
    ] {
        assert_selected(&parser, &context, retained);
    }
    for malformed in [
        "Target creature gets 1/-1 until end of turn.",
        "Target creature gets -1/1 until end of turn.",
        "Target creature gets --1/-1 until end of turn.",
        "Target creature gets -1/--1 until end of turn.",
        "Target creature gets +-1/-1 until end of turn.",
        "Target creature gets -1/-+1 until end of turn.",
        "Target creature gets -1 until end of turn.",
        "Target creature gets -1/ until end of turn.",
        "Target creature gets /-1 until end of turn.",
        "Target creature gets -1//-1 until end of turn.",
        "Target creature gets -1 /-1 until end of turn.",
        "Target creature gets -1/ -1 until end of turn.",
        "Target creature gets 3/3 until end of turn.",
        "Target creature get -1/-1 until end of turn.",
        "Creatures gets -1/-1 until end of turn.",
    ] {
        assert!(
            parser.parse(malformed, &context).is_err(),
            "malformed adjustment boundary must reject {malformed:?}",
        );
    }
}

#[test]
fn task9_ordinary_ability_nouns_parse_while_keyword_interiors_remain_plan10() {
    let parser = parser();
    let context = context();
    let text = "Target creature loses all abilities.";
    let ability = assert_selected(&parser, &context, text);
    let VerbPhrase::TransitivePredicate(predicate) = declarative_atomic(&parser, &context, text)
    else {
        panic!("ability removal uses the shared transitive frame")
    };
    assert!(matches!(
        predicate.head,
        TransitiveVerb::Lexeme(CoreTransitiveVerb::Lose)
    ));
    let mut visitor = Task9ObjectVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["lose-abilities"]);

    for plan10_keyword_interior in [
        "Target creature gains flying until end of turn.",
        "Creatures you control have vigilance.",
    ] {
        assert!(
            parser.parse(plan10_keyword_interior, &context).is_err(),
            "bare keyword ability remains a Plan 10 boundary: {plan10_keyword_interior:?}",
        );
    }
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
    let Predicate::Atomic(predicate) = imperative.predicate() else {
        panic!("declared object verb has an atomic predicate envelope")
    };
    let VerbPhrase::TransitivePredicate(predicate) = predicate.as_ref() else {
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
    let Predicate::Atomic(predicate) = imperative.predicate() else {
        panic!("core object verb has an atomic predicate envelope")
    };
    let VerbPhrase::TransitivePredicate(predicate) = predicate.as_ref() else {
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
fn copular_change_auxiliary_and_passive_minimal_pairs_select() {
    let parser = parser();
    let context = context();
    for text in [
        "It is legendary.",
        "They are white.",
        "It was tapped.",
        "They were 2/2.",
        "It is a creature.",
        "It is able to attack.",
        "Be white.",
        "Become tapped.",
        "It becomes blocked by target creature.",
        "They become tapped.",
        "It didn't cast a spell.",
        "It would attack.",
        "It would be white.",
        "It can't be dealt damage.",
        "It is dealt damage.",
        "It is dealt combat damage.",
        "It is put into your graveyard from the battlefield.",
        "It is turned face up.",
        "A spell was cast.",
        "It deals damage.",
        "It gains life.",
    ] {
        assert_selected(&parser, &context, text);
    }
}

#[test]
#[allow(
    clippy::items_after_statements,
    clippy::match_same_arms,
    clippy::too_many_lines,
    reason = "one exhaustive integration table keeps every Task 7 clause family and AST oracle together"
)]
fn task7_finite_clause_families_compose_in_triggers_and_conditions() {
    let parser = parser();
    let context = context();
    #[derive(Debug, Clone, Copy)]
    enum IntegratedClause {
        Damage(DamageKind),
        Movement,
        Orientation,
        CopularCondition,
    }

    fn assert_selected_family(
        parser: &Parser,
        context: &ParseContext<'_>,
        text: &str,
        expected: IntegratedClause,
        expected_claims: &[(&str, &str)],
    ) {
        let ability = assert_selected(parser, context, text);
        let Ability::Triggered(triggered) = &ability else {
            panic!("integrated clause witness is triggered: {ability:#?}")
        };
        match expected {
            IntegratedClause::Damage(expected_kind) => {
                let TriggerPrefix::Finite(Finite { marker, clause }) = &triggered.trigger else {
                    panic!("damage witness has a finite trigger: {ability:#?}")
                };
                assert_eq!(*marker, TriggerMarker::Whenever);
                assert!(triggered.intervening_if.as_ref().is_none());
                let Clause::Passive(passive) = clause.as_ref() else {
                    panic!("damage witness has a passive clause: {ability:#?}")
                };
                let PassiveFiniteClause::PassiveFiniteClause(value) = passive;
                let PassivePredicate::Damage(predicate) = &value.predicate else {
                    panic!("damage witness has a passive damage predicate: {ability:#?}")
                };
                let PassiveDamagePredicate::PassiveDamagePredicate(PassiveDamagePredicateValue {
                    head,
                    kind,
                }) = predicate;
                assert!(matches!(
                    head,
                    DamageParticipleHead::Lexeme(DamageParticipleLexeme::Deal)
                ));
                assert_eq!(*kind, expected_kind);
            }
            IntegratedClause::Movement | IntegratedClause::Orientation => {
                let TriggerPrefix::Finite(Finite { marker, clause }) = &triggered.trigger else {
                    panic!("passive witness has a finite trigger: {ability:#?}")
                };
                assert_eq!(*marker, TriggerMarker::Whenever);
                assert!(triggered.intervening_if.as_ref().is_none());
                let Clause::Passive(passive) = clause.as_ref() else {
                    panic!("passive witness has a passive clause: {ability:#?}")
                };
                let PassiveFiniteClause::PassiveFiniteClause(value) = passive;
                assert!(
                    matches!(
                        (expected, &value.predicate),
                        (IntegratedClause::Movement, PassivePredicate::Movement(_))
                            | (
                                IntegratedClause::Orientation,
                                PassivePredicate::Orientation(_)
                            )
                    ),
                    "wrong integrated passive family for {text:?}: {ability:#?}",
                );
            }
            IntegratedClause::CopularCondition => {
                assert!(matches!(triggered.trigger, TriggerPrefix::Temporal(_)));
                let Some(ConditionClause::FiniteCondition(condition)) =
                    triggered.intervening_if.as_ref().as_ref()
                else {
                    panic!("temporal witness has a finite condition: {ability:#?}")
                };
                let FiniteCondition::FiniteCondition(value) = condition.as_ref();
                let Clause::Copular(copular) = value.clause.as_ref() else {
                    panic!("temporal witness has a copular condition: {ability:#?}")
                };
                let CopularClause::CopularClause(value) = copular.as_ref();
                assert!(matches!(
                    value.complement.as_ref(),
                    PredicativeComplement::Color(_)
                ));
            }
        }
        assert_eq!(
            exact_claim_trace(parser, context, text),
            expected_claims
                .iter()
                .map(|(surface, owner)| ((*surface).to_owned(), (*owner).to_owned()))
                .collect::<Vec<_>>(),
            "exact integrated-clause claims for {text:?}",
        );
    }

    macro_rules! selected {
        ($text:literal, $family:expr, [$(($surface:literal, $owner:literal)),+ $(,)?]) => {
            assert_selected_family(
                &parser,
                &context,
                $text,
                $family,
                &[$(($surface, $owner)),+],
            );
        };
    }

    selected!(
        "Whenever this creature is dealt damage, it deals that much damage to you.",
        IntegratedClause::Damage(DamageKind::Ordinary),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" this", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "lexeme:DamageParticipleLexeme/Deal/participle"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" it", "vocab:SubjectPronoun/It"),
            (" deals", "lexeme:VerbLexeme/Deal/third_person_singular"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" damage", "form:deal_damage/deal_damage/2"),
            (" to", "form:damage_recipient/damage_recipient/0"),
            (" you", "vocab:ObjectPronoun/You"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt damage, you gain 1 life.",
        IntegratedClause::Damage(DamageKind::Ordinary),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" this", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "lexeme:DamageParticipleLexeme/Deal/participle"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" you", "vocab:SubjectPronoun/You"),
            (" gain", "lexeme:VerbLexeme/Gain/bare"),
            (" 1", "codec:ScalarNumber"),
            (" life", "form:gain_life/gain_life/2"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt damage, it deals that much damage to each player.",
        IntegratedClause::Damage(DamageKind::Ordinary),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" this", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "lexeme:DamageParticipleLexeme/Deal/participle"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" it", "vocab:SubjectPronoun/It"),
            (" deals", "lexeme:VerbLexeme/Deal/third_person_singular"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" damage", "form:deal_damage/deal_damage/2"),
            (" to", "form:damage_recipient/damage_recipient/0"),
            (" each", "form:each_reference/each_reference/0"),
            (" player", "lexeme:CommonNoun/Player/singular"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );

    let later = [
        (
            "Whenever this creature is dealt damage, it deals that much damage to target opponent or planeswalker.",
            (88, 100, "planeswalker"),
        ),
        (
            "At the beginning of each player's end step, if that player didn't cast a spell this turn, this enchantment deals 2 damage to that player.",
            (95, 106, "enchantment"),
        ),
    ];
    for (text, (expected_start, expected_end, expected_surface)) in later {
        let error = parser
            .analyze(text, &context)
            .into_parse_result()
            .expect_err("reviewed later witness remains an ordinary failure");
        let deckmaste_english_v2::parser::ParseError::Failure { span, .. } = error else {
            panic!("reviewed later witness has exact ordinary failure class: {error:?}")
        };
        assert_eq!(
            (span.start, span.end),
            (expected_start, expected_end),
            "{text:?}"
        );
        assert_eq!(&text[span.start..span.end], expected_surface, "{text:?}");
    }

    const MOVEMENT_TEXT: &str =
        "Whenever a creature is put into your graveyard from the battlefield, you gain 1 life.";
    const MOVEMENT_CLAIMS: &[(&str, &str)] = &[
        ("Whenever", "vocab:TriggerMarker/Whenever"),
        (" a", "form:indefinite_reference/a/0"),
        (" creature", "lexeme:type/Creature/singular"),
        (" is", "vocab:FiniteCopula/Is"),
        (" put", "lexeme:MovementParticipleLexeme/Put/participle"),
        (" into", "form:into_destination/into_destination/0"),
        (" your", "vocab:PossessiveDeterminerPronoun/Your"),
        (" graveyard", "vocab:Zone/Graveyard"),
        (" from", "form:from_source/from_source/0"),
        (" the", "form:definite_zone/definite_zone/0"),
        (" battlefield", "vocab:Zone/Battlefield"),
        (",", "form:triggered/triggered/1"),
        (" you", "vocab:SubjectPronoun/You"),
        (" gain", "lexeme:VerbLexeme/Gain/bare"),
        (" 1", "codec:ScalarNumber"),
        (" life", "form:gain_life/gain_life/2"),
        (".", "structural:Sentences/sentences/terminator/0"),
    ];
    assert_selected_family(
        &parser,
        &context,
        MOVEMENT_TEXT,
        IntegratedClause::Movement,
        MOVEMENT_CLAIMS,
    );
    assert_selected_family(
        &parser,
        &context,
        MOVEMENT_TEXT,
        IntegratedClause::Movement,
        MOVEMENT_CLAIMS,
    );
    selected!(
        "Whenever a permanent is turned face up, this creature deals 1 damage to any target.",
        IntegratedClause::Orientation,
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" a", "form:indefinite_reference/a/0"),
            (" permanent", "lexeme:CommonNoun/Permanent/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (
                " turned",
                "lexeme:OrientationParticipleLexeme/Turn/participle"
            ),
            (" face up", "vocab:FaceOrientation/FaceUp"),
            (",", "form:triggered/triggered/1"),
            (" this", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" deals", "lexeme:VerbLexeme/Deal/third_person_singular"),
            (" 1", "codec:ScalarNumber"),
            (" damage", "form:deal_damage/deal_damage/2"),
            (" to", "form:damage_recipient/damage_recipient/0"),
            (" any", "form:any_target_reference/any_target_reference/0"),
            (
                " target",
                "form:any_target_reference/any_target_reference/1"
            ),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt combat damage, you gain that much life.",
        IntegratedClause::Damage(DamageKind::Combat),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" this", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "lexeme:DamageParticipleLexeme/Deal/participle"),
            (" combat damage", "vocab:DamageKind/Combat"),
            (",", "form:triggered/triggered/1"),
            (" you", "vocab:SubjectPronoun/You"),
            (" gain", "lexeme:VerbLexeme/Gain/bare"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" life", "form:gain_life/gain_life/2"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt damage, each opponent gains that much life.",
        IntegratedClause::Damage(DamageKind::Ordinary),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" this", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "lexeme:DamageParticipleLexeme/Deal/participle"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" each", "form:each_reference/each_reference/0"),
            (" opponent", "lexeme:CommonNoun/Opponent/singular"),
            (" gains", "lexeme:VerbLexeme/Gain/third_person_singular"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" life", "form:gain_life/gain_life/2"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt damage, it deals that much damage to any target.",
        IntegratedClause::Damage(DamageKind::Ordinary),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" this", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "lexeme:DamageParticipleLexeme/Deal/participle"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" it", "vocab:SubjectPronoun/It"),
            (" deals", "lexeme:VerbLexeme/Deal/third_person_singular"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" damage", "form:deal_damage/deal_damage/2"),
            (" to", "form:damage_recipient/damage_recipient/0"),
            (" any", "form:any_target_reference/any_target_reference/0"),
            (
                " target",
                "form:any_target_reference/any_target_reference/1"
            ),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever a creature you control is put into your graveyard from the battlefield, you gain 1 life.",
        IntegratedClause::Movement,
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" a", "form:indefinite_reference/a/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" you", "vocab:SubjectPronoun/You"),
            (" control", "lexeme:VerbLexeme/Control/bare"),
            (" is", "vocab:FiniteCopula/Is"),
            (" put", "lexeme:MovementParticipleLexeme/Put/participle"),
            (" into", "form:into_destination/into_destination/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" graveyard", "vocab:Zone/Graveyard"),
            (" from", "form:from_source/from_source/0"),
            (" the", "form:definite_zone/definite_zone/0"),
            (" battlefield", "vocab:Zone/Battlefield"),
            (",", "form:triggered/triggered/1"),
            (" you", "vocab:SubjectPronoun/You"),
            (" gain", "lexeme:VerbLexeme/Gain/bare"),
            (" 1", "codec:ScalarNumber"),
            (" life", "form:gain_life/gain_life/2"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "At the beginning of your upkeep, if all creatures are white, you gain 1 life.",
        IntegratedClause::CopularCondition,
        [
            ("At", "form:temporal/temporal/0"),
            (" the beginning of", "vocab:AtBoundary/Beginning"),
            (" your", "vocab:TurnSpecifier/Your"),
            (" upkeep", "vocab:TurnPart/Upkeep"),
            (",", "form:triggered/triggered/1"),
            (" if", "form:finite_condition/finite_condition/0"),
            (" all", "form:all_reference/all_reference/0"),
            (" creatures", "lexeme:type/Creature/plural"),
            (" are", "vocab:FiniteCopula/Are"),
            (" white", "vocab:Color/White"),
            (",", "form:finite_condition/finite_condition/2"),
            (" you", "vocab:SubjectPronoun/You"),
            (" gain", "lexeme:VerbLexeme/Gain/bare"),
            (" 1", "codec:ScalarNumber"),
            (" life", "form:gain_life/gain_life/2"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
}

#[test]
fn copular_change_auxiliary_and_passive_reciprocals_reject() {
    let parser = parser();
    let context = context();
    for text in [
        "It are white.",
        "They is white.",
        "It become tapped.",
        "They becomes tapped.",
        "It didn't casts a spell.",
        "It would attacks.",
        "It can't is dealt damage.",
        "It is deal damage.",
        "It is dealt.",
        "It is dealt face up.",
        "It is put damage.",
        "It is turned damage.",
        "A spell was cast damage.",
        "It deals life.",
        "It gains damage.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "crossed agreement, form, or complement must reject {text:?}",
        );
    }
}

#[test]
fn task7_ast_keeps_each_linguistic_product_typed() {
    let parser = parser();
    let context = context();
    for (text, expected) in [
        ("It is legendary.", "adjective"),
        ("They are white.", "color"),
        ("It is a creature.", "type"),
        ("It was tapped.", "status"),
        ("It is able to attack.", "ability"),
        ("They were 2/2.", "power-toughness"),
    ] {
        let Sentence::Declarative(declarative) = parser
            .parse_sentence(text, &context)
            .unwrap_or_else(|error| panic!("copular AST parses {text:?}: {error:?}"))
        else {
            panic!("{text:?} stores a typed copular clause")
        };
        let Clause::Copular(copular) = declarative.clause.as_ref() else {
            panic!("{text:?} stores a typed copular clause")
        };
        let CopularClause::CopularClause(CopularClauseValue { complement, .. }) = copular.as_ref();
        let observed = match complement.as_ref() {
            PredicativeComplement::Adjective(_) => "adjective",
            PredicativeComplement::Color(_) => "color",
            PredicativeComplement::Type(_) => "type",
            PredicativeComplement::Status(_) => "status",
            PredicativeComplement::Ability(_) => "ability",
            PredicativeComplement::PowerToughness(_) => "power-toughness",
            PredicativeComplement::Scalar(_) => "scalar",
            PredicativeComplement::AllColors(_) => "all-colors",
        };
        assert_eq!(observed, expected);
    }

    let Sentence::Imperative(imperative) = parser
        .parse_sentence("Be white.", &context)
        .expect("bare copular predicate parses")
    else {
        panic!("bare copular predicate has an imperative envelope")
    };
    let Predicate::BareCopular(predicate) = imperative.predicate() else {
        panic!("bare copula has its distinct predicate sum branch")
    };
    let BareCopularPredicate::BareCopularPredicate(BareCopularPredicateValue {
        copula,
        complement,
    }) = predicate.as_ref();
    assert_eq!(*copula, BareCopula::Be);
    assert!(matches!(
        complement.as_ref(),
        PredicativeComplement::Color(_)
    ));

    let Sentence::Declarative(declarative) = parser
        .parse_sentence("It becomes blocked by target creature.", &context)
        .expect("change-state predicate parses")
    else {
        panic!("change-state predicate keeps its ordinary finite envelope")
    };
    let Clause::Finite(finite) = declarative.clause.as_ref() else {
        panic!("change-state predicate keeps its ordinary finite envelope")
    };
    let FiniteClause::PlainFiniteClause(change) = finite.as_ref() else {
        panic!("change-state predicate keeps its ordinary finite envelope")
    };
    let Predicate::ChangeState(predicate) = change.predicate() else {
        panic!("become has its distinct predicate sum branch")
    };
    let ChangeStatePredicate::ChangeStatePredicate(ChangeStatePredicateValue { complement }) =
        predicate.as_ref();
    assert!(matches!(
        complement.as_ref(),
        PredicativeComplement::Status(PredicativeStatus::BlockedBy(_))
    ));

    for (text, expected) in [
        ("It is dealt damage.", "damage"),
        (
            "It is put into your graveyard from the battlefield.",
            "movement",
        ),
        ("It is turned face up.", "orientation"),
        ("A spell was cast.", "declared"),
    ] {
        let Sentence::Declarative(declarative) = parser
            .parse_sentence(text, &context)
            .unwrap_or_else(|error| panic!("passive AST parses {text:?}: {error:?}"))
        else {
            panic!("{text:?} stores a typed finite passive clause")
        };
        let Clause::Passive(passive) = declarative.clause.as_ref() else {
            panic!("{text:?} stores a typed finite passive clause")
        };
        let PassiveFiniteClause::PassiveFiniteClause(PassiveFiniteClauseValue {
            predicate, ..
        }) = passive;
        let observed = match predicate {
            PassivePredicate::Damage(_) => "damage",
            PassivePredicate::Movement(_) => "movement",
            PassivePredicate::Orientation(_) => "orientation",
            PassivePredicate::DeclaredTransitive(_) => "declared",
        };
        assert_eq!(observed, expected);
    }

    assert!(matches!(
        declarative_atomic(&parser, &context, "It deals damage."),
        VerbPhrase::DealUnspecifiedDamage(DealUnspecifiedDamage { .. })
    ));
    assert!(matches!(
        declarative_atomic(&parser, &context, "It gains life."),
        VerbPhrase::GainUnspecifiedLife(GainUnspecifiedLife {})
    ));
}

#[derive(Default)]
struct Task7Visitor(Vec<String>);

macro_rules! task7_product {
    ($method:ident, $type:ty, $walk:ident, $label:literal) => {
        fn $method(&mut self, value: &$type) {
            self.0.push(concat!("product:", $label).to_owned());
            deckmaste_english_v2::visit::$walk(self, value);
        }
    };
}

impl Visitor for Task7Visitor {
    task7_product!(
        visit_copular_clause_value,
        CopularClauseValue,
        walk_copular_clause_value,
        "CopularClauseValue"
    );
    task7_product!(
        visit_predicative_adjective_value,
        PredicativeAdjectiveValue,
        walk_predicative_adjective_value,
        "PredicativeAdjectiveValue"
    );
    task7_product!(
        visit_predicative_color_value,
        PredicativeColorValue,
        walk_predicative_color_value,
        "PredicativeColorValue"
    );
    task7_product!(
        visit_predicative_type_value,
        PredicativeTypeValue,
        walk_predicative_type_value,
        "PredicativeTypeValue"
    );
    task7_product!(
        visit_predicative_status_value,
        PredicativeStatusValue,
        walk_predicative_status_value,
        "PredicativeStatusValue"
    );
    task7_product!(
        visit_blocked_by_status_value,
        BlockedByStatusValue,
        walk_blocked_by_status_value,
        "BlockedByStatusValue"
    );
    task7_product!(
        visit_predicative_ability_value,
        PredicativeAbilityValue,
        walk_predicative_ability_value,
        "PredicativeAbilityValue"
    );
    task7_product!(
        visit_predicative_power_toughness_value,
        PredicativePowerToughnessValue,
        walk_predicative_power_toughness_value,
        "PredicativePowerToughnessValue"
    );
    task7_product!(
        visit_bare_copular_predicate_value,
        BareCopularPredicateValue,
        walk_bare_copular_predicate_value,
        "BareCopularPredicateValue"
    );
    task7_product!(
        visit_change_state_predicate_value,
        ChangeStatePredicateValue,
        walk_change_state_predicate_value,
        "ChangeStatePredicateValue"
    );
    task7_product!(
        visit_auxiliary_finite_clause,
        AuxiliaryFiniteClause,
        walk_auxiliary_finite_clause,
        "AuxiliaryFiniteClause"
    );
    task7_product!(
        visit_bare_passive_predicate_value,
        BarePassivePredicateValue,
        walk_bare_passive_predicate_value,
        "BarePassivePredicateValue"
    );
    task7_product!(
        visit_passive_finite_clause_value,
        PassiveFiniteClauseValue,
        walk_passive_finite_clause_value,
        "PassiveFiniteClauseValue"
    );
    task7_product!(
        visit_passive_damage_predicate_value,
        PassiveDamagePredicateValue,
        walk_passive_damage_predicate_value,
        "PassiveDamagePredicateValue"
    );
    task7_product!(
        visit_passive_movement_predicate_value,
        PassiveMovementPredicateValue,
        walk_passive_movement_predicate_value,
        "PassiveMovementPredicateValue"
    );
    task7_product!(
        visit_passive_orientation_predicate_value,
        PassiveOrientationPredicateValue,
        walk_passive_orientation_predicate_value,
        "PassiveOrientationPredicateValue"
    );
    task7_product!(
        visit_declared_transitive_passive_predicate_value,
        DeclaredTransitivePassivePredicateValue,
        walk_declared_transitive_passive_predicate_value,
        "DeclaredTransitivePassivePredicateValue"
    );
    task7_product!(
        visit_deal_unspecified_damage,
        DealUnspecifiedDamage,
        walk_deal_unspecified_damage,
        "DealUnspecifiedDamage"
    );
    task7_product!(
        visit_gain_unspecified_life,
        GainUnspecifiedLife,
        walk_gain_unspecified_life,
        "GainUnspecifiedLife"
    );
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the literal per-frame visitor and ownership matrix is intentionally exhaustive"
)]
fn every_task7_frame_has_exact_visits_and_complete_ordered_claims() {
    const TERMINATOR: &str = "structural:Sentences/sentences/terminator/0";
    let parser = parser();
    let context = context();

    macro_rules! assert_frame {
        ($text:literal, [$($visit:literal),+ $(,)?], [$(($surface:literal, $owner:expr)),+ $(,)?]) => {{
            let ability = assert_selected(&parser, &context, $text);
            let mut visitor = Task7Visitor::default();
            visitor.visit_ability(&ability);
            assert_eq!(visitor.0, [$($visit),+], "exact visitor trace for {:?}", $text);
            assert_eq!(
                exact_claim_trace(&parser, &context, $text),
                [$(($surface.to_owned(), $owner.to_owned())),+],
                "complete ordered ownership for {:?}", $text,
            );
        }};
    }

    assert_frame!(
        "It is legendary.",
        [
            "product:CopularClauseValue",
            "product:PredicativeAdjectiveValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" is", "vocab:FiniteCopula/Is"),
            (" legendary", "vocab:PredicativeAdjective/Legendary"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "They are white.",
        [
            "product:CopularClauseValue",
            "product:PredicativeColorValue"
        ],
        [
            ("They", "vocab:SubjectPronoun/They"),
            (" are", "vocab:FiniteCopula/Are"),
            (" white", "vocab:Color/White"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It was tapped.",
        [
            "product:CopularClauseValue",
            "product:PredicativeStatusValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" was", "vocab:FiniteCopula/Was"),
            (" tapped", "vocab:Status/Tapped"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "They were 2/2.",
        [
            "product:CopularClauseValue",
            "product:PredicativePowerToughnessValue"
        ],
        [
            ("They", "vocab:SubjectPronoun/They"),
            (" were", "vocab:FiniteCopula/Were"),
            (" 2", "codec:ScalarNumber"),
            (
                "/",
                "structural:PredicativePowerToughnessValue/magnitudes/separator/uniform/0"
            ),
            ("2", "codec:ScalarNumber"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It is a creature.",
        ["product:CopularClauseValue", "product:PredicativeTypeValue"],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" is", "vocab:FiniteCopula/Is"),
            (" a", "form:predicative_type/a/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It is able to attack.",
        [
            "product:CopularClauseValue",
            "product:PredicativeAbilityValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" is", "vocab:FiniteCopula/Is"),
            (" able", "form:predicative_ability/predicative_ability/0"),
            (" to", "form:predicative_ability/predicative_ability/1"),
            (" attack", "lexeme:CoreIntransitiveVerb/Attack/bare"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "Be white.",
        [
            "product:BareCopularPredicateValue",
            "product:PredicativeColorValue"
        ],
        [
            ("Be", "vocab:BareCopula/Be"),
            (" white", "vocab:Color/White"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "Become tapped.",
        [
            "product:ChangeStatePredicateValue",
            "product:PredicativeStatusValue"
        ],
        [
            ("Become", "lexeme:VerbLexeme/Become/bare"),
            (" tapped", "vocab:Status/Tapped"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It becomes blocked by target creature.",
        [
            "product:ChangeStatePredicateValue",
            "product:BlockedByStatusValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" becomes", "lexeme:VerbLexeme/Become/third_person_singular"),
            (" blocked", "form:blocked_by_status/blocked_by_status/0"),
            (" by", "form:blocked_by_status/blocked_by_status/1"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It didn't cast a spell.",
        ["product:AuxiliaryFiniteClause"],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" didn't", "vocab:Auxiliary/Didnt"),
            (" cast", "lexeme:keyword_action/Cast/bare"),
            (" a", "form:indefinite_reference/a/0"),
            (" spell", "lexeme:CommonNoun/Spell/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It would attack.",
        ["product:AuxiliaryFiniteClause"],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" would", "vocab:Auxiliary/Would"),
            (" attack", "lexeme:CoreIntransitiveVerb/Attack/bare"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It would be white.",
        [
            "product:AuxiliaryFiniteClause",
            "product:BareCopularPredicateValue",
            "product:PredicativeColorValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" would", "vocab:Auxiliary/Would"),
            (" be", "vocab:BareCopula/Be"),
            (" white", "vocab:Color/White"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It can't be dealt damage.",
        [
            "product:AuxiliaryFiniteClause",
            "product:BarePassivePredicateValue",
            "product:PassiveDamagePredicateValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" can't", "vocab:Auxiliary/Cant"),
            (" be", "vocab:BareCopula/Be"),
            (" dealt", "lexeme:DamageParticipleLexeme/Deal/participle"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It is dealt combat damage.",
        [
            "product:PassiveFiniteClauseValue",
            "product:PassiveDamagePredicateValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "lexeme:DamageParticipleLexeme/Deal/participle"),
            (" combat damage", "vocab:DamageKind/Combat"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It is put into your graveyard from the battlefield.",
        [
            "product:PassiveFiniteClauseValue",
            "product:PassiveMovementPredicateValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" is", "vocab:FiniteCopula/Is"),
            (" put", "lexeme:MovementParticipleLexeme/Put/participle"),
            (" into", "form:into_destination/into_destination/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" graveyard", "vocab:Zone/Graveyard"),
            (" from", "form:from_source/from_source/0"),
            (" the", "form:definite_zone/definite_zone/0"),
            (" battlefield", "vocab:Zone/Battlefield"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It is turned face up.",
        [
            "product:PassiveFiniteClauseValue",
            "product:PassiveOrientationPredicateValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" is", "vocab:FiniteCopula/Is"),
            (
                " turned",
                "lexeme:OrientationParticipleLexeme/Turn/participle"
            ),
            (" face up", "vocab:FaceOrientation/FaceUp"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "A spell was cast.",
        [
            "product:PassiveFiniteClauseValue",
            "product:DeclaredTransitivePassivePredicateValue"
        ],
        [
            ("A", "form:indefinite_reference/a/0"),
            (" spell", "lexeme:CommonNoun/Spell/singular"),
            (" was", "vocab:FiniteCopula/Was"),
            (" cast", "lexeme:keyword_action/Cast/participle"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It deals damage.",
        ["product:DealUnspecifiedDamage"],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" deals", "lexeme:VerbLexeme/Deal/third_person_singular"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It gains life.",
        ["product:GainUnspecifiedLife"],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" gains", "lexeme:VerbLexeme/Gain/third_person_singular"),
            (
                " life",
                "form:gain_unspecified_life/gain_unspecified_life/1"
            ),
            (".", TERMINATOR)
        ]
    );
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
    let Predicate::Atomic(predicate) = imperative.predicate() else {
        panic!("put-counter syntax retains its atomic predicate envelope")
    };
    let VerbPhrase::PutCounters(predicate) = predicate.as_ref() else {
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
        CounterRecipient::CounterRecipient(CounterRecipientValue {
            object: Object::ObjectPronoun(PersonalObject {
                word: ObjectPronoun::It,
            }),
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
    let Clause::Finite(finite) = declarative.clause.as_ref() else {
        panic!("finite scalar predicate remains a plain finite clause")
    };
    let FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
        panic!("finite scalar predicate remains a plain finite clause")
    };
    let Predicate::Atomic(predicate) = clause.predicate() else {
        panic!("damage equality retains its atomic predicate envelope")
    };
    let VerbPhrase::DealDamageEqualTo(predicate) = predicate.as_ref() else {
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
    let ManaAmount::ManaAmount(mana) = mana;
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

#[derive(Default)]
struct MovementVisitor(Vec<String>);

impl Visitor for MovementVisitor {
    trace_product!(
        visit_compared_card_quantity,
        ComparedCardQuantity,
        walk_compared_card_quantity,
        "ComparedCardQuantity"
    );
    trace_product!(
        visit_damage_recipient_value,
        DamageRecipientValue,
        walk_damage_recipient_value,
        "DamageRecipientValue"
    );
    trace_product!(
        visit_counter_recipient_value,
        CounterRecipientValue,
        walk_counter_recipient_value,
        "CounterRecipientValue"
    );
    trace_product!(
        visit_counter_source_value,
        CounterSourceValue,
        walk_counter_source_value,
        "CounterSourceValue"
    );
    trace_product!(
        visit_singular_owner_possessor,
        SingularOwnerPossessor,
        walk_singular_owner_possessor,
        "SingularOwnerPossessor"
    );
    trace_product!(
        visit_plural_owner_possessor,
        PluralOwnerPossessor,
        walk_plural_owner_possessor,
        "PluralOwnerPossessor"
    );
    trace_product!(
        visit_owner_possessed_zone,
        OwnerPossessedZone,
        walk_owner_possessed_zone,
        "OwnerPossessedZone"
    );
    trace_product!(
        visit_definite_zone,
        DefiniteZone,
        walk_definite_zone,
        "DefiniteZone"
    );
    trace_product!(
        visit_possessed_library,
        PossessedLibrary,
        walk_possessed_library,
        "PossessedLibrary"
    );
    trace_product!(
        visit_owner_possessed_library,
        OwnerPossessedLibrary,
        walk_owner_possessed_library,
        "OwnerPossessedLibrary"
    );
    trace_product!(
        visit_singular_library_card_quantity,
        SingularLibraryCardQuantity,
        walk_singular_library_card_quantity,
        "SingularLibraryCardQuantity"
    );
    trace_product!(
        visit_fixed_library_card_quantity,
        FixedLibraryCardQuantity,
        walk_fixed_library_card_quantity,
        "FixedLibraryCardQuantity"
    );
    trace_product!(
        visit_library_slice,
        LibrarySlice,
        walk_library_slice,
        "LibrarySlice"
    );
    trace_product!(
        visit_from_source_value,
        FromSourceValue,
        walk_from_source_value,
        "FromSourceValue"
    );
    trace_product!(
        visit_into_destination_value,
        IntoDestinationValue,
        walk_into_destination_value,
        "IntoDestinationValue"
    );
    trace_product!(
        visit_onto_battlefield_destination,
        OntoBattlefieldDestination,
        walk_onto_battlefield_destination,
        "OntoBattlefieldDestination"
    );
    trace_product!(
        visit_on_library_destination,
        OnLibraryDestination,
        walk_on_library_destination,
        "OnLibraryDestination"
    );
    trace_product!(
        visit_to_destination_value,
        ToDestinationValue,
        walk_to_destination_value,
        "ToDestinationValue"
    );
    trace_product!(
        visit_tapped_post_state,
        TappedPostState,
        walk_tapped_post_state,
        "TappedPostState"
    );
    trace_product!(
        visit_direct_control_postmodifier,
        DirectControlPostmodifier,
        walk_direct_control_postmodifier,
        "DirectControlPostmodifier"
    );
    trace_product!(
        visit_owner_control_postmodifier,
        OwnerControlPostmodifier,
        walk_owner_control_postmodifier,
        "OwnerControlPostmodifier"
    );
    trace_product!(
        visit_zone_location_value,
        ZoneLocationValue,
        walk_zone_location_value,
        "ZoneLocationValue"
    );
    trace_product!(
        visit_at_location_value,
        AtLocationValue,
        walk_at_location_value,
        "AtLocationValue"
    );
    trace_product!(visit_put_into, PutInto, walk_put_into, "PutInto");
    trace_product!(visit_put_onto, PutOnto, walk_put_onto, "PutOnto");
    trace_product!(visit_put_on, PutOn, walk_put_on, "PutOn");
    trace_product!(visit_put_to, PutTo, walk_put_to, "PutTo");
    trace_product!(visit_return_to, ReturnTo, walk_return_to, "ReturnTo");
    trace_product!(
        visit_enter_post_state,
        EnterPostState,
        walk_enter_post_state,
        "EnterPostState"
    );
    trace_product!(
        visit_enter_location,
        EnterLocation,
        walk_enter_location,
        "EnterLocation"
    );
    trace_product!(
        visit_enter_control,
        EnterControl,
        walk_enter_control,
        "EnterControl"
    );
    trace_product!(
        visit_leave_location,
        LeaveLocation,
        walk_leave_location,
        "LeaveLocation"
    );
    trace_product!(visit_look_at, LookAt, walk_look_at, "LookAt");
    trace_product!(visit_search_for, SearchFor, walk_search_for, "SearchFor");
    trace_product!(
        visit_have_cards_in_hand,
        HaveCardsInHand,
        walk_have_cards_in_hand,
        "HaveCardsInHand"
    );
    trace_product!(visit_have_life, HaveLife, walk_have_life, "HaveLife");
    trace_product!(
        visit_have_no_maximum_hand_size,
        HaveNoMaximumHandSize,
        walk_have_no_maximum_hand_size,
        "HaveNoMaximumHandSize"
    );
    trace_product!(
        visit_have_object_control,
        HaveObjectControl,
        walk_have_object_control,
        "HaveObjectControl"
    );
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
            (" to", "form:damage_recipient/damage_recipient/0"),
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
            (" to", "form:damage_recipient/damage_recipient/0"),
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
            (" on", "form:counter_recipient/counter_recipient/0"),
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
            (" on", "form:counter_recipient/counter_recipient/0"),
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
            (" on", "form:counter_recipient/counter_recipient/0"),
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
            (" on", "form:counter_recipient/counter_recipient/0"),
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
            (" on", "form:counter_recipient/counter_recipient/0"),
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
            (" on", "form:counter_recipient/counter_recipient/0"),
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
            (" from", "form:counter_source/counter_source/0"),
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

#[test]
fn movement_frames_select_exact_source_destination_state_and_control_roles() {
    let parser = parser();
    let context = context();
    for (text, permits_specificity) in [
        ("Put that card into your hand.", false),
        ("Put that card to your hand.", false),
        (
            "Put target creature card from your graveyard onto the battlefield tapped under your control.",
            true,
        ),
        ("Put target creature on top of its owner's library.", false),
        ("Return target creature to its owner's hand.", false),
        (
            "Return target creature card from your graveyard to the battlefield tapped under its owner's control.",
            true,
        ),
        ("This creature enters tapped.", false),
        (
            "This creature enters the battlefield under your control.",
            false,
        ),
        ("This creature enters under your control.", false),
        ("This creature leaves the battlefield.", false),
        ("One or more cards leave your graveyard.", false),
    ] {
        assert_selected_with_specificity(&parser, &context, text, permits_specificity);
    }

    // The parser owns grammatical form, not zone or controller legality.
    for text in [
        "Return target spell to the battlefield under your control.",
        "Put target player onto the battlefield tapped under their control.",
    ] {
        assert_selected(&parser, &context, text);
    }
}

#[test]
fn location_state_and_object_control_frames_select_exact_products() {
    let parser = parser();
    let context = context();
    for text in [
        "Search your library for a creature card.",
        "Look at the top two cards of your library.",
        "Reveal the top card of your library.",
        "Each player reveals their hand.",
        "You have three or fewer cards in hand.",
        "You have 10 or less life.",
        "You have no maximum hand size.",
        "Have her deal 2 damage to you.",
    ] {
        assert_selected(&parser, &context, text);
    }
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the literal per-family typed AST matrix is intentionally complete"
)]
fn movement_location_and_control_builds_retain_every_typed_role() {
    let parser = parser();
    let context = context();

    assert!(matches!(
        imperative_atomic(&parser, &context, "Put that card into your hand."),
        VerbPhrase::PutInto(PutInto {
            source: None,
            destination: IntoDestination::IntoDestination(_),
            ..
        })
    ));
    assert!(matches!(
        imperative_atomic(
            &parser,
            &context,
            "Put target creature card from your graveyard onto the battlefield tapped under your control."
        ),
        VerbPhrase::PutOnto(PutOnto {
            source: Some(FromSource::FromSource(_)),
            destination: OntoDestination::OntoBattlefieldDestination(_),
            post_state: Some(PostState::TappedPostState(_)),
            control: Some(ControlPostmodifier::DirectControlPostmodifier(_)),
            ..
        })
    ));
    assert!(matches!(
        imperative_atomic(
            &parser,
            &context,
            "Put target creature on top of its owner's library."
        ),
        VerbPhrase::PutOn(PutOn {
            destination: OnDestination::OnLibraryDestination(OnLibraryDestination {
                library: LibraryReference::OwnerPossessedLibrary(OwnerPossessedLibrary {
                    owner: OwnerPossessor::SingularOwnerPossessor(_),
                }),
                ..
            }),
            ..
        })
    ));
    assert!(matches!(
        imperative_atomic(&parser, &context, "Put that card to your hand."),
        VerbPhrase::PutTo(PutTo {
            object: Object::ObjectNominal(_),
            destination: ToDestination::ToDestination(ToDestinationValue {
                zone: ZoneReference::PossessedZone(PossessedZone {
                    possessor: PossessiveDeterminerPronoun::Your,
                    zone: Zone::Hand,
                }),
            }),
        })
    ));
    assert!(matches!(
        imperative_atomic(
            &parser,
            &context,
            "Return target creature card from your graveyard to the battlefield tapped under its owner's control."
        ),
        VerbPhrase::ReturnTo(ReturnTo {
            source: Some(FromSource::FromSource(_)),
            destination: ToDestination::ToDestination(_),
            post_state: Some(PostState::TappedPostState(_)),
            control: Some(ControlPostmodifier::OwnerControlPostmodifier(_)),
            ..
        })
    ));
    let VerbPhrase::LookAt(LookAt {
        location:
            AtLocation::AtLocation(AtLocationValue {
                object: Object::ObjectNominal(nominal),
            }),
    }) = imperative_atomic(
        &parser,
        &context,
        "Look at the top two cards of your library.",
    )
    else {
        panic!("look-at witness keeps its typed nominal object")
    };
    let NounPhrase::LibrarySlice(slice) = nominal.value.as_ref() else {
        panic!("look-at witness keeps its typed library slice")
    };
    assert!(matches!(
        (&slice.cards, &slice.library),
        (
            LibraryCardQuantity::FixedLibraryCardQuantity(_),
            LibraryReference::PossessedLibrary(_)
        )
    ));
    assert!(matches!(
        imperative_atomic(
            &parser,
            &context,
            "Search your library for a creature card."
        ),
        VerbPhrase::SearchFor(SearchFor {
            location: LibraryReference::PossessedLibrary(_),
            sought: Object::ObjectNominal(_),
            ..
        })
    ));
    let VerbPhrase::HaveObjectControl(control) =
        imperative_atomic(&parser, &context, "Have her deal 2 damage to you.")
    else {
        panic!("object control keeps its exact predicate product")
    };
    assert!(matches!(control.object, Object::ObjectPronoun(_)));
    assert!(matches!(
        control.predicate(),
        VerbPhrase::DealDamage(DealDamage {
            recipient: DamageRecipient::DamageRecipient(_),
            ..
        })
    ));

    assert!(matches!(
        declarative_atomic(&parser, &context, "This creature enters tapped."),
        VerbPhrase::EnterPostState(EnterPostState {
            post_state: PostState::TappedPostState(_),
        })
    ));
    assert!(matches!(
        declarative_atomic(
            &parser,
            &context,
            "This creature enters the battlefield under your control."
        ),
        VerbPhrase::EnterLocation(EnterLocation {
            location: ZoneLocation::ZoneLocation(_),
            control: Some(ControlPostmodifier::DirectControlPostmodifier(_)),
            ..
        })
    ));
    assert!(matches!(
        declarative_atomic(
            &parser,
            &context,
            "This creature enters under your control."
        ),
        VerbPhrase::EnterControl(EnterControl {
            control: ControlPostmodifier::DirectControlPostmodifier(_),
        })
    ));
    assert!(matches!(
        declarative_atomic(&parser, &context, "This creature leaves the battlefield."),
        VerbPhrase::LeaveLocation(LeaveLocation {
            location: ZoneLocation::ZoneLocation(_),
        })
    ));
    assert!(matches!(
        declarative_atomic(&parser, &context, "You have three or fewer cards in hand."),
        VerbPhrase::HaveCardsInHand(HaveCardsInHand {
            cards: CardQuantity::ComparedCardQuantity(_),
        })
    ));
    assert!(matches!(
        declarative_atomic(&parser, &context, "You have 10 or less life."),
        VerbPhrase::HaveLife(HaveLife {
            comparison: ScalarComparison::ScalarOrLess(_),
        })
    ));
    assert!(matches!(
        declarative_atomic(&parser, &context, "You have no maximum hand size."),
        VerbPhrase::HaveNoMaximumHandSize(_)
    ));
}

#[test]
fn movement_location_and_control_frames_reject_reciprocal_heads_prepositions_and_tails() {
    let parser = parser();
    let context = context();
    for text in [
        "Return that card into your hand.",
        "Control that card to your hand.",
        "Put that card from your hand.",
        "Put that card to tapped.",
        "Put that card on your hand.",
        "Put that card onto your hand.",
        "Put that card into the battlefield.",
        "Search your library from a creature card.",
        "Search a creature card for your library.",
        "Reveal your library for a creature card.",
        "Remove a time counter to this card.",
        "This creature leaves to the battlefield.",
        "This creature enters from your graveyard.",
        "Look into the top two cards of your library.",
        "Reveal at the top card of your library.",
        "You have three or fewer cards on hand.",
        "Put that card under your control onto the battlefield.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong head, preposition, or tail must reject {text:?}",
        );
    }
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the literal per-family visitor and claim matrix is intentionally complete"
)]
fn every_movement_location_and_control_family_has_exact_visits_and_claims() {
    const TERMINATOR: &str = "structural:Sentences/sentences/terminator/0";

    let parser = parser();
    let context = context();

    macro_rules! assert_family {
        ($text:literal, $specificity:literal, [$($visit:literal),+ $(,)?], [$(($surface:literal, $owner:expr)),+ $(,)?]) => {{
        let ability = assert_selected_with_specificity(&parser, &context, $text, $specificity);
        let mut visitor = MovementVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, [$($visit),+], "exact visitor trace for {:?}", $text);
        assert_eq!(
            exact_claim_trace(&parser, &context, $text),
            [$(($surface.to_owned(), $owner.to_owned())),+],
            "exact complete ordered ownership for {:?}",
            $text,
        );
        }};
    }

    assert_family!(
        "Put that card into your hand.",
        false,
        ["product:PutInto", "product:IntoDestinationValue"],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (" that", "form:that_reference/that_reference/0"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (" into", "form:into_destination/into_destination/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" hand", "vocab:Zone/Hand"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Put that card to your hand.",
        false,
        ["product:PutTo", "product:ToDestinationValue"],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (" that", "form:that_reference/that_reference/0"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (" to", "form:to_destination/to_destination/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" hand", "vocab:Zone/Hand"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Put target creature card from your graveyard onto the battlefield tapped under your control.",
        true,
        [
            "product:PutOnto",
            "product:FromSourceValue",
            "product:OntoBattlefieldDestination",
            "product:DefiniteZone",
            "product:TappedPostState",
            "product:DirectControlPostmodifier"
        ],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (" from", "form:from_source/from_source/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" graveyard", "vocab:Zone/Graveyard"),
            (
                " onto",
                "form:onto_battlefield_destination/onto_battlefield_destination/0"
            ),
            (" the", "form:definite_zone/definite_zone/0"),
            (" battlefield", "vocab:Zone/Battlefield"),
            (" tapped", "form:tapped_post_state/tapped_post_state/0"),
            (
                " under",
                "form:direct_control_postmodifier/direct_control_postmodifier/0"
            ),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (
                " control",
                "form:direct_control_postmodifier/direct_control_postmodifier/2"
            ),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Put target creature on top of its owner's library.",
        false,
        [
            "product:PutOn",
            "product:OnLibraryDestination",
            "product:OwnerPossessedLibrary",
            "product:SingularOwnerPossessor"
        ],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" on", "form:on_library_destination/top/0"),
            (" top", "vocab:LibraryPosition/Top"),
            (" of", "form:on_library_destination/top/2"),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (
                " owner's",
                "form:singular_owner_possessor/singular_owner_possessor/1"
            ),
            (
                " library",
                "form:owner_possessed_library/owner_possessed_library/1"
            ),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Put target creature on the bottom of their owners' library.",
        false,
        [
            "product:PutOn",
            "product:OnLibraryDestination",
            "product:OwnerPossessedLibrary",
            "product:PluralOwnerPossessor"
        ],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" on", "form:on_library_destination/bottom/0"),
            (" the", "form:on_library_destination/bottom/1"),
            (" bottom", "vocab:LibraryPosition/Bottom"),
            (" of", "form:on_library_destination/bottom/3"),
            (" their", "vocab:PossessiveDeterminerPronoun/Their"),
            (
                " owners'",
                "form:plural_owner_possessor/plural_owner_possessor/1"
            ),
            (
                " library",
                "form:owner_possessed_library/owner_possessed_library/1"
            ),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Return target creature card from your graveyard to the battlefield tapped under its owner's control.",
        true,
        [
            "product:ReturnTo",
            "product:FromSourceValue",
            "product:ToDestinationValue",
            "product:DefiniteZone",
            "product:TappedPostState",
            "product:OwnerControlPostmodifier",
            "product:SingularOwnerPossessor"
        ],
        [
            ("Return", "lexeme:VerbLexeme/Return/bare"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (" from", "form:from_source/from_source/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" graveyard", "vocab:Zone/Graveyard"),
            (" to", "form:to_destination/to_destination/0"),
            (" the", "form:definite_zone/definite_zone/0"),
            (" battlefield", "vocab:Zone/Battlefield"),
            (" tapped", "form:tapped_post_state/tapped_post_state/0"),
            (
                " under",
                "form:owner_control_postmodifier/owner_control_postmodifier/0"
            ),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (
                " owner's",
                "form:singular_owner_possessor/singular_owner_possessor/1"
            ),
            (
                " control",
                "form:owner_control_postmodifier/owner_control_postmodifier/2"
            ),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "This creature enters tapped.",
        false,
        ["product:EnterPostState", "product:TappedPostState"],
        [
            ("This", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" enters", "lexeme:VerbLexeme/Enter/third_person_singular"),
            (" tapped", "form:tapped_post_state/tapped_post_state/0"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "This creature enters the battlefield under your control.",
        false,
        [
            "product:EnterLocation",
            "product:ZoneLocationValue",
            "product:DefiniteZone",
            "product:DirectControlPostmodifier"
        ],
        [
            ("This", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" enters", "lexeme:VerbLexeme/Enter/third_person_singular"),
            (" the", "form:definite_zone/definite_zone/0"),
            (" battlefield", "vocab:Zone/Battlefield"),
            (
                " under",
                "form:direct_control_postmodifier/direct_control_postmodifier/0"
            ),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (
                " control",
                "form:direct_control_postmodifier/direct_control_postmodifier/2"
            ),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "This creature enters under your control.",
        false,
        ["product:EnterControl", "product:DirectControlPostmodifier"],
        [
            ("This", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" enters", "lexeme:VerbLexeme/Enter/third_person_singular"),
            (
                " under",
                "form:direct_control_postmodifier/direct_control_postmodifier/0"
            ),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (
                " control",
                "form:direct_control_postmodifier/direct_control_postmodifier/2"
            ),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "This creature leaves the battlefield.",
        false,
        [
            "product:LeaveLocation",
            "product:ZoneLocationValue",
            "product:DefiniteZone"
        ],
        [
            ("This", "form:this_reference/this_reference/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" leaves", "lexeme:VerbLexeme/Leave/third_person_singular"),
            (" the", "form:definite_zone/definite_zone/0"),
            (" battlefield", "vocab:Zone/Battlefield"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Look at the top two cards of your library.",
        false,
        [
            "product:LookAt",
            "product:AtLocationValue",
            "product:LibrarySlice",
            "product:FixedLibraryCardQuantity",
            "product:PossessedLibrary"
        ],
        [
            ("Look", "lexeme:VerbLexeme/Look/bare"),
            (" at", "form:at_location/at_location/0"),
            (" the", "form:library_slice/library_slice/0"),
            (" top", "vocab:LibraryPosition/Top"),
            (" two", "codec:CardinalNumber"),
            (
                " cards",
                "form:fixed_library_card_quantity/fixed_library_card_quantity/1"
            ),
            (" of", "form:library_slice/library_slice/3"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" library", "form:possessed_library/possessed_library/1"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Reveal the top card of your library.",
        false,
        [
            "product:LibrarySlice",
            "product:SingularLibraryCardQuantity",
            "product:PossessedLibrary"
        ],
        [
            ("Reveal", "lexeme:keyword_action/Reveal/bare"),
            (" the", "form:library_slice/library_slice/0"),
            (" top", "vocab:LibraryPosition/Top"),
            (
                " card",
                "form:singular_library_card_quantity/singular_library_card_quantity/0"
            ),
            (" of", "form:library_slice/library_slice/3"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" library", "form:possessed_library/possessed_library/1"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Search your library for a creature card.",
        false,
        ["product:SearchFor", "product:PossessedLibrary"],
        [
            ("Search", "lexeme:keyword_action/Search/bare"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" library", "form:possessed_library/possessed_library/1"),
            (" for", "form:search_for/search_for/2"),
            (" a", "form:indefinite_reference/a/0"),
            (" creature", "lexeme:type/Creature/singular"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "You have three or fewer cards in hand.",
        false,
        ["product:HaveCardsInHand", "product:ComparedCardQuantity"],
        [
            ("You", "vocab:SubjectPronoun/You"),
            (" have", "lexeme:VerbLexeme/Have/bare"),
            (" three", "codec:CardinalNumber"),
            (" or", "form:count_or_fewer/count_or_fewer/0"),
            (" fewer", "form:count_or_fewer/count_or_fewer/1"),
            (
                " cards",
                "form:compared_card_quantity/compared_card_quantity/2"
            ),
            (" in", "form:have_cards_in_hand/have_cards_in_hand/2"),
            (" hand", "form:have_cards_in_hand/have_cards_in_hand/3"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "You have 10 or less life.",
        false,
        ["product:HaveLife"],
        [
            ("You", "vocab:SubjectPronoun/You"),
            (" have", "lexeme:VerbLexeme/Have/bare"),
            (" 10", "codec:ScalarNumber"),
            (" or", "form:scalar_or_less/scalar_or_less/1"),
            (" less", "form:scalar_or_less/scalar_or_less/2"),
            (" life", "form:have_life/have_life/2"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "You have no maximum hand size.",
        false,
        ["product:HaveNoMaximumHandSize"],
        [
            ("You", "vocab:SubjectPronoun/You"),
            (" have", "lexeme:VerbLexeme/Have/bare"),
            (
                " no",
                "form:have_no_maximum_hand_size/have_no_maximum_hand_size/1"
            ),
            (
                " maximum",
                "form:have_no_maximum_hand_size/have_no_maximum_hand_size/2"
            ),
            (
                " hand",
                "form:have_no_maximum_hand_size/have_no_maximum_hand_size/3"
            ),
            (
                " size",
                "form:have_no_maximum_hand_size/have_no_maximum_hand_size/4"
            ),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Have her deal 2 damage to you.",
        false,
        ["product:HaveObjectControl", "product:DamageRecipientValue"],
        [
            ("Have", "lexeme:VerbLexeme/Have/bare"),
            (" her", "vocab:ObjectPronoun/Her"),
            (" deal", "lexeme:VerbLexeme/Deal/bare"),
            (" 2", "codec:ScalarNumber"),
            (" damage", "form:deal_damage/deal_damage/2"),
            (" to", "form:damage_recipient/damage_recipient/0"),
            (" you", "vocab:ObjectPronoun/You"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Put two stun counters on it.",
        false,
        ["product:CounterRecipientValue"],
        [
            ("Put", "lexeme:VerbLexeme/Put/bare"),
            (" two", "codec:CardinalNumber"),
            (" stun", "vocab:CounterName/Stun"),
            (
                " counters",
                "form:fixed_counter_quantity/fixed_counter_quantity/2"
            ),
            (" on", "form:counter_recipient/counter_recipient/0"),
            (" it", "vocab:ObjectPronoun/It"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Remove X time counters from this card.",
        false,
        ["product:CounterSourceValue"],
        [
            ("Remove", "lexeme:VerbLexeme/Remove/bare"),
            (" X", "vocab:Variable/X"),
            (" time", "vocab:CounterName/Time"),
            (
                " counters",
                "form:variable_counter_quantity/variable_counter_quantity/2"
            ),
            (" from", "form:counter_source/counter_source/0"),
            (" this", "form:this_reference/this_reference/0"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (".", TERMINATOR)
        ]
    );
}

#[derive(Default)]
struct Task8Visitor(Vec<&'static str>);

impl Visitor for Task8Visitor {
    fn visit_object_infinitive_predicate_value(&mut self, value: &ObjectInfinitivePredicateValue) {
        self.0.push("object-infinitive");
        deckmaste_english_v2::visit::walk_object_infinitive_predicate_value(self, value);
    }

    fn visit_requirement_predicate_value(&mut self, value: &RequirementPredicateValue) {
        self.0.push("requirement");
        deckmaste_english_v2::visit::walk_requirement_predicate_value(self, value);
    }

    fn visit_as_though_predicate(&mut self, value: &AsThoughPredicate) {
        self.0.push("as-though");
        deckmaste_english_v2::visit::walk_as_though_predicate(self, value);
    }

    fn visit_counterfactual_finite_clause(&mut self, value: &CounterfactualFiniteClause) {
        self.0.push("counterfactual-finite");
        deckmaste_english_v2::visit::walk_counterfactual_finite_clause(self, value);
    }

    fn visit_counterfactual_status_clause_value(
        &mut self,
        value: &CounterfactualStatusClauseValue,
    ) {
        self.0.push("counterfactual-status");
        deckmaste_english_v2::visit::walk_counterfactual_status_clause_value(self, value);
    }

    fn visit_counterfactual_negative_ability_clause_value(
        &mut self,
        value: &CounterfactualNegativeAbilityClauseValue,
    ) {
        self.0.push("counterfactual-negative-ability");
        deckmaste_english_v2::visit::walk_counterfactual_negative_ability_clause_value(self, value);
    }

    fn visit_counterfactual_past_ability_clause_value(
        &mut self,
        value: &CounterfactualPastAbilityClauseValue,
    ) {
        self.0.push("counterfactual-past-ability");
        deckmaste_english_v2::visit::walk_counterfactual_past_ability_clause_value(self, value);
    }

    fn visit_ordered_predicate_value(&mut self, value: &OrderedPredicateValue) {
        self.0.push("ordered");
        deckmaste_english_v2::visit::walk_ordered_predicate_value(self, value);
    }

    fn visit_purpose_predicate_value(&mut self, value: &PurposePredicateValue) {
        self.0.push("purpose");
        deckmaste_english_v2::visit::walk_purpose_predicate_value(self, value);
    }

    fn visit_duration_predicate_value(&mut self, value: &DurationPredicateValue) {
        self.0.push("duration");
        deckmaste_english_v2::visit::walk_duration_predicate_value(self, value);
    }

    fn visit_instead_predicate_value(&mut self, value: &InsteadPredicateValue) {
        self.0.push("instead");
        deckmaste_english_v2::visit::walk_instead_predicate_value(self, value);
    }

    fn visit_manner_predicate_value(&mut self, value: &MannerPredicateValue) {
        self.0.push("manner");
        deckmaste_english_v2::visit::walk_manner_predicate_value(self, value);
    }
}

#[test]
fn task8_infinitive_requirement_counterfactual_and_order_products_are_typed() {
    let parser = parser();
    let context = context();

    for (text, expected, permits_specificity) in [
        (
            "Whenever a spell or ability an opponent controls causes you to discard a card, you gain 2 life and you may draw a card.",
            &["object-infinitive"][..],
            true,
        ),
        (
            "This creature attacks each combat if able.",
            &["requirement"][..],
            false,
        ),
        (
            "Tapped creatures you control can block as though they were untapped.",
            &[
                "as-though",
                "counterfactual-finite",
                "counterfactual-status",
            ][..],
            false,
        ),
        (
            "Put them on top of your library in any order.",
            &["ordered"][..],
            false,
        ),
    ] {
        let ability =
            assert_selected_with_specificity(&parser, &context, text, permits_specificity);
        let mut visitor = Task8Visitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, expected, "exact typed products for {text:?}");
    }
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the three counterfactual finite-clause frames share one exhaustive typed audit"
)]
fn task8_as_though_owns_the_attested_counterfactual_finite_family() {
    let parser = parser();
    let context = context();

    for (text, expected) in [
        (
            "It can block as though it didn't have hexproof.",
            &[
                "as-though",
                "counterfactual-finite",
                "counterfactual-negative-ability",
            ][..],
        ),
        (
            "You can cast spells as though they had flash.",
            &[
                "as-though",
                "counterfactual-finite",
                "counterfactual-past-ability",
            ][..],
        ),
        (
            "Tapped creatures you control can block as though they were untapped.",
            &[
                "as-though",
                "counterfactual-finite",
                "counterfactual-status",
            ][..],
        ),
    ] {
        let ability = assert_selected(&parser, &context, text);
        let mut visitor = Task8Visitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(
            visitor.0, expected,
            "typed counterfactual finite family for {text:?}"
        );
    }

    assert_eq!(
        exact_claim_trace(
            &parser,
            &context,
            "It can block as though it didn't have hexproof."
        ),
        [
            ("It".to_owned(), "vocab:SubjectPronoun/It".to_owned()),
            (" can".to_owned(), "vocab:Auxiliary/Can".to_owned()),
            (
                " block".to_owned(),
                "lexeme:CoreIntransitiveVerb/Block/bare".to_owned(),
            ),
            (
                " as".to_owned(),
                "form:intransitive_as_though_predicate/intransitive_as_though_predicate/1"
                    .to_owned(),
            ),
            (
                " though".to_owned(),
                "form:intransitive_as_though_predicate/intransitive_as_though_predicate/2"
                    .to_owned(),
            ),
            (" it".to_owned(), "vocab:SubjectPronoun/It".to_owned()),
            (
                " didn't".to_owned(),
                "vocab:CounterfactualNegativeAuxiliary/Didnt".to_owned(),
            ),
            (" have".to_owned(), "lexeme:VerbLexeme/Have/bare".to_owned(),),
            (
                " hexproof".to_owned(),
                "vocab:CounterfactualAbility/Hexproof".to_owned(),
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
    );
    assert_eq!(
        exact_claim_trace(
            &parser,
            &context,
            "You can cast spells as though they had flash."
        ),
        [
            ("You".to_owned(), "vocab:SubjectPronoun/You".to_owned()),
            (" can".to_owned(), "vocab:Auxiliary/Can".to_owned()),
            (
                " cast".to_owned(),
                "lexeme:keyword_action/Cast/bare".to_owned(),
            ),
            (
                " spells".to_owned(),
                "lexeme:CommonNoun/Spell/plural".to_owned(),
            ),
            (
                " as".to_owned(),
                "form:transitive_as_though_predicate/transitive_as_though_predicate/2".to_owned(),
            ),
            (
                " though".to_owned(),
                "form:transitive_as_though_predicate/transitive_as_though_predicate/3".to_owned(),
            ),
            (" they".to_owned(), "vocab:SubjectPronoun/They".to_owned(),),
            (
                " had".to_owned(),
                "vocab:CounterfactualPastPossession/Had".to_owned(),
            ),
            (
                " flash".to_owned(),
                "vocab:CounterfactualAbility/Flash".to_owned(),
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
    );
    assert_eq!(
        exact_claim_trace(
            &parser,
            &context,
            "Tapped creatures you control can block as though they were untapped."
        ),
        [
            ("Tapped".to_owned(), "vocab:Status/Tapped".to_owned()),
            (
                " creatures".to_owned(),
                "lexeme:type/Creature/plural".to_owned(),
            ),
            (" you".to_owned(), "vocab:SubjectPronoun/You".to_owned()),
            (
                " control".to_owned(),
                "lexeme:VerbLexeme/Control/bare".to_owned(),
            ),
            (" can".to_owned(), "vocab:Auxiliary/Can".to_owned()),
            (
                " block".to_owned(),
                "lexeme:CoreIntransitiveVerb/Block/bare".to_owned(),
            ),
            (
                " as".to_owned(),
                "form:intransitive_as_though_predicate/intransitive_as_though_predicate/1"
                    .to_owned(),
            ),
            (
                " though".to_owned(),
                "form:intransitive_as_though_predicate/intransitive_as_though_predicate/2"
                    .to_owned(),
            ),
            (" they".to_owned(), "vocab:SubjectPronoun/They".to_owned(),),
            (" were".to_owned(), "vocab:FiniteCopula/Were".to_owned(),),
            (" untapped".to_owned(), "vocab:Status/Untapped".to_owned(),),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
    );

    for crossed in [
        "It can block as though it didn't has hexproof.",
        "It can block as though it did have hexproof.",
        "You can cast spells as though they has flash.",
        "Tapped creatures you control can block as though they was untapped.",
    ] {
        assert!(
            parser.parse(crossed, &context).is_err(),
            "counterfactual finite morphology and agreement reject {crossed:?}",
        );
    }
}

#[test]
fn task8_purpose_duration_and_instead_products_are_typed() {
    let parser = parser();
    let context = context();

    for (text, expected, permits_specificity) in [
        ("Discard a card to draw a card.", "purpose", true),
        ("You may cast it this turn.", "duration", false),
        ("Draw a card instead.", "instead", false),
    ] {
        let ability =
            assert_selected_with_specificity(&parser, &context, text, permits_specificity);
        let mut visitor = Task8Visitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, [expected], "exact typed product for {text:?}");
    }
}

#[test]
fn task8_this_way_keeps_predicate_manner_scope() {
    let parser = parser();
    let context = context();
    let text = "You didn't create a token this way.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = Task8Visitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["manner"]);
}

#[derive(Default)]
struct Task8SurfaceVisitor(Vec<String>);

impl Visitor for Task8SurfaceVisitor {
    fn visit_auxiliary(&mut self, value: Auxiliary) {
        self.0.push(format!("auxiliary:{value:?}"));
    }

    fn visit_duration_predicate_value(&mut self, value: &DurationPredicateValue) {
        self.0.push("duration".to_owned());
        deckmaste_english_v2::visit::walk_duration_predicate_value(self, value);
    }

    fn visit_random_object(&mut self, value: &RandomObject) {
        self.0.push("at-random-object".to_owned());
        deckmaste_english_v2::visit::walk_random_object(self, value);
    }

    fn visit_object_order(&mut self, value: ObjectOrder) {
        self.0.push(format!("order:{value:?}"));
    }

    fn visit_postposed_unless_predicate(&mut self, value: &PostposedUnlessPredicate) {
        self.0.push("exception:unless".to_owned());
        deckmaste_english_v2::visit::walk_postposed_unless_predicate(self, value);
    }
}

#[test]
fn task8_permission_restriction_exception_random_and_order_surfaces_are_scoped() {
    let parser = parser();
    let context = context();
    for (text, expected) in [
        (
            "You may cast it this turn.",
            &["auxiliary:May", "duration"][..],
        ),
        (
            "Target player can't cast spells this turn.",
            &["auxiliary:Cant", "duration"][..],
        ),
        (
            "Sacrifice this creature unless you discard a card.",
            &["exception:unless"][..],
        ),
        (
            "Target player discards two cards at random.",
            &["at-random-object"][..],
        ),
        (
            "Put them on top of your library in a random order.",
            &["order:Random"][..],
        ),
    ] {
        let ability = assert_selected(&parser, &context, text);
        let mut visitor = Task8SurfaceVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, expected, "exact scoped surface for {text:?}");
    }
}

#[test]
fn task8_attachment_movement_does_not_silently_change_scope() {
    let parser = parser();
    let context = context();
    for text in [
        "To draw a card, discard a card.",
        "You may this turn cast it.",
        "Instead draw a card.",
        "You didn't create this way a token.",
        "If able, this creature attacks each combat.",
        "As though they were untapped, tapped creatures you control can block.",
        "In any order, put them on top of your library.",
        "Target player discards at random two cards.",
    ] {
        let analysis = parser.analyze(text, &context);
        assert!(
            analysis.selected().is_none(),
            "moved attachment must not silently reattach {text:?}: {analysis:?}"
        );
    }
}

#[test]
fn task8_cost_position_reuses_the_typed_predicate_algebra() {
    let parser = parser();
    let context = context();
    let text = "{1}, Discard a card instead: Draw a card.";
    let ability = assert_selected_with_specificity(&parser, &context, text, true);
    let Ability::Activated(activated) = &ability else {
        panic!("the colon surface has the activated envelope")
    };
    let [
        ActivationCostComponent::SymbolRun(_),
        ActivationCostComponent::Clause(cost),
    ] = activated.costs()
    else {
        panic!("the activated cost contains one symbol run and one typed clause")
    };
    assert!(matches!(cost.predicate(), Predicate::Instead(_)));
    assert_eq!(
        CostClause::new(Box::new(cost.predicate().clone())).as_ref(),
        Some(cost.as_ref()),
    );
    assert_eq!(
        Activated::new(Box::new(activated.costs().to_vec()), activated.body.clone()).as_ref(),
        Some(activated),
    );

    let mut visitor = Task8Visitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["instead"]);
    assert_eq!(
        exact_claim_trace(&parser, &context, text),
        [
            (
                "{".to_owned(),
                "form:symbol_run/symbol_run/0/prefix".to_owned(),
            ),
            ("1".to_owned(), "codec:ScalarNumber".to_owned()),
            (
                "}".to_owned(),
                "form:symbol_run/symbol_run/0/suffix".to_owned(),
            ),
            (
                ", ".to_owned(),
                "structural:Activated/costs/separator/pair/0".to_owned(),
            ),
            (
                "Discard".to_owned(),
                "lexeme:keyword_action/Discard/bare".to_owned(),
            ),
            (" a".to_owned(), "form:indefinite_reference/a/0".to_owned(),),
            (
                " card".to_owned(),
                "lexeme:CommonNoun/Card/singular".to_owned(),
            ),
            (
                " instead".to_owned(),
                "form:instead_predicate/instead_predicate/2".to_owned(),
            ),
            (": ".to_owned(), "form:activated/activated/1".to_owned(),),
            ("Draw".to_owned(), "lexeme:VerbLexeme/Draw/bare".to_owned(),),
            (
                " a".to_owned(),
                "form:singular_card_quantity/singular_card_quantity/0".to_owned(),
            ),
            (
                " card".to_owned(),
                "form:singular_card_quantity/singular_card_quantity/1".to_owned(),
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
    );

    for crossed in [
        "{1}, Discards a card instead: Draw a card.",
        "{1}, discard a card instead: Draw a card.",
    ] {
        assert!(
            parser.parse(crossed, &context).is_err(),
            "cost position enforces bare agreement and document-internal case: {crossed:?}",
        );
    }
}

#[test]
fn task8_object_internal_discarded_this_way_does_not_become_outer_manner() {
    let parser = parser();
    let context = context();
    let text = "Target player discards two cards. Put up to one land card discarded this way onto the battlefield tapped under your control.";
    let analysis = parser.analyze(text, &context);
    assert!(analysis.selected().is_none(), "{analysis:?}");
    assert!(analysis.decision().is_none(), "{analysis:?}");
    let error = analysis
        .into_parse_result()
        .expect_err("object-internal participial syntax remains a later nominal boundary");
    assert!(
        matches!(
            error,
            ParseError::Failure {
                span: TextSpan { start: 58, end: 67 },
                ..
            }
        ),
        "the exact unsupported constituent starts at `discarded`, not an outer manner attachment: {error:?}",
    );
    assert_eq!(&text[58..67], "discarded");
}

#[derive(Default)]
struct Task10bVisitor(Vec<&'static str>);

impl Visitor for Task10bVisitor {
    fn visit_state_duration_predicate_value(&mut self, value: &StateDurationPredicateValue) {
        self.0.push("state-duration");
        deckmaste_english_v2::visit::walk_state_duration_predicate_value(self, value);
    }

    fn visit_declared_transitive_passive_predicate_value(
        &mut self,
        value: &DeclaredTransitivePassivePredicateValue,
    ) {
        self.0.push("declared-passive");
        deckmaste_english_v2::visit::walk_declared_transitive_passive_predicate_value(self, value);
    }

    fn visit_deal_distributed_damage(&mut self, value: &DealDistributedDamage) {
        self.0.push("distributed-damage");
        deckmaste_english_v2::visit::walk_deal_distributed_damage(self, value);
    }

    fn visit_bare_target_distribution_recipient(
        &mut self,
        value: &BareTargetDistributionRecipient,
    ) {
        self.0.push("bare-target-distribution");
        deckmaste_english_v2::visit::walk_bare_target_distribution_recipient(self, value);
    }

    fn visit_put_counters(&mut self, value: &PutCounters) {
        self.0.push("put-counters");
        deckmaste_english_v2::visit::walk_put_counters(self, value);
    }

    fn visit_remove_counters(&mut self, value: &RemoveCounters) {
        self.0.push("remove-counters");
        deckmaste_english_v2::visit::walk_remove_counters(self, value);
    }

    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        if declaration.kind() == macro_ron::v2::DeclarationKind::KeywordAction
            && declaration.name() == "Regenerate"
        {
            self.0.push("regenerate-declaration");
        }
    }
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the positive authority checks all Task 10B typed frames, ASTs, visits, and claims together"
)]
fn task10b_passive_distribution_and_counter_frames_select_typed_products() {
    let parser = parser();
    let context = context();

    for text in [
        "It can't be regenerated.",
        "It can't be regenerated this turn.",
        "It deals 2 damage divided as you choose among one or two targets.",
        "It deals 3 damage divided as you choose among one, two, or three target attacking creatures.",
        "It deals X damage divided as you choose among any number of target creatures.",
        "It deals X damage divided as you choose among up to two target creatures and/or planeswalkers.",
        "It deals twice X damage divided as you choose among them instead.",
        "It deals X plus 1 damage divided as you choose among any number of targets.",
        "It deals X damage divided evenly, rounded down, among any number of targets.",
        "Put an oil counter on this creature.",
        "Put two loyalty counters on a planeswalker you control.",
        "Put a spore counter on target creature.",
        "Put a verse counter on this creature.",
        "Remove a counter from a nonland permanent you control.",
        "Remove two counters from target permanent.",
        "Remove X counters from among permanents you control.",
        "Put an oil counter on this creature and remove a counter from target permanent.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    let Sentence::Declarative(declarative) = parser
        .parse_sentence("It can't be regenerated this turn.", &context)
        .expect("declared participle duration parses")
    else {
        unreachable!()
    };
    let Clause::Finite(finite) = declarative.clause.as_ref() else {
        panic!("negative auxiliary retains its finite clause")
    };
    let FiniteClause::AuxiliaryFiniteClause(auxiliary) = finite.as_ref() else {
        panic!("negative auxiliary retains its finite clause")
    };
    let Predicate::StateDuration(duration) = auxiliary.predicate() else {
        panic!("passive duration retains its typed attachment")
    };
    let StateDurationPredicate::StateDurationPredicate(StateDurationPredicateValue {
        predicate,
        duration,
    }) = duration;
    assert_eq!(duration, &PredicateDuration::ThisTurn);
    let StateDurationBase::BarePassive(BarePassivePredicate::BarePassivePredicate(
        BarePassivePredicateValue {
            predicate: PassivePredicate::DeclaredTransitive(declared),
            ..
        },
    )) = predicate
    else {
        panic!("regeneration uses the open transitive-participle frame")
    };
    let DeclaredTransitivePassivePredicate::DeclaredTransitivePassivePredicate(
        DeclaredTransitivePassivePredicateValue { head },
    ) = declared;
    let DeclaredTransitiveParticipleHead::Declaration(_) = head else {
        panic!("regenerate is not duplicated as a core participle")
    };

    let VerbPhrase::DealDistributedDamage(distributed) = declarative_atomic(
        &parser,
        &context,
        "It deals 2 damage divided as you choose among one or two targets.",
    ) else {
        panic!("damage division keeps its exact predicate product")
    };
    assert!(matches!(
        distributed.distribution,
        DamageDistribution::AsYouChoose(ChosenDamageDistribution {
            recipient: DistributionRecipient::BareTargets(_),
        })
    ));

    let VerbPhrase::PutCounters(PutCounters { counters, .. }) =
        imperative_atomic(&parser, &context, "Put an oil counter on this creature.")
    else {
        unreachable!()
    };
    assert!(matches!(
        counters,
        CounterQuantity::SingularCounterQuantity(SingularCounterQuantity {
            kind: CounterKind::NamedCounter(NamedCounter {
                name: CounterName::Oil,
            }),
        })
    ));

    let VerbPhrase::RemoveCounters(RemoveCounters { counters, .. }) = imperative_atomic(
        &parser,
        &context,
        "Remove a counter from a nonland permanent you control.",
    ) else {
        unreachable!()
    };
    assert!(matches!(
        counters,
        CounterQuantity::UnnamedSingularCounterQuantity(_)
    ));

    let mut visitor = Task10bVisitor::default();
    let ability = assert_selected_with_specificity(
        &parser,
        &context,
        "It can't be regenerated this turn. It deals 2 damage divided as you choose among one or two targets. Put an oil counter on this creature. Remove a counter from target permanent.",
        true,
    );
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.0,
        [
            "state-duration",
            "declared-passive",
            "regenerate-declaration",
            "distributed-damage",
            "bare-target-distribution",
            "put-counters",
            "remove-counters",
        ]
    );

    assert_eq!(
        exact_claim_trace(
            &parser,
            &context,
            "It deals 2 damage divided as you choose among one or two targets.",
        ),
        [
            ("It".to_owned(), "vocab:SubjectPronoun/It".to_owned()),
            (
                " deals".to_owned(),
                "lexeme:VerbLexeme/Deal/third_person_singular".to_owned()
            ),
            (" 2".to_owned(), "codec:ScalarNumber".to_owned()),
            (
                " damage".to_owned(),
                "form:deal_distributed_damage/deal_distributed_damage/2".to_owned()
            ),
            (
                " divided".to_owned(),
                "form:chosen_damage_distribution/chosen_damage_distribution/0".to_owned()
            ),
            (
                " as".to_owned(),
                "form:chosen_damage_distribution/chosen_damage_distribution/1".to_owned()
            ),
            (
                " you".to_owned(),
                "form:chosen_damage_distribution/chosen_damage_distribution/2".to_owned()
            ),
            (
                " choose".to_owned(),
                "form:chosen_damage_distribution/chosen_damage_distribution/3".to_owned()
            ),
            (
                " among".to_owned(),
                "form:chosen_damage_distribution/chosen_damage_distribution/4".to_owned()
            ),
            (
                " one or two".to_owned(),
                "vocab:BareTargetDistributionBounds/OneOrTwo".to_owned()
            ),
            (
                " targets".to_owned(),
                "form:bare_target_distribution_recipient/bare_target_distribution_recipient/1"
                    .to_owned()
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned()
            ),
        ]
    );
}

#[test]
fn task10b_frame_reciprocals_reject_crossed_morphology_and_boundaries() {
    let parser = parser();
    let context = context();
    for text in [
        "It can't regenerated.",
        "It can't be regenerate.",
        "It can't be regenerateed.",
        "It can't be regenerated to this turn.",
        "Target creature can't be blocks this turn.",
        "It deals damage divided as you choose among one or two targets.",
        "It deals 2 damages divided as you choose among one or two targets.",
        "It deals 2 damage as you choose among one or two targets.",
        "It deals 2 damage divided as you choose to one or two targets.",
        "It deals 2 damage divided as you choose among target.",
        "It deals 2 damage divided as you choose among one or two target.",
        "It deals 2 damage divided evenly among all creatures you control.",
        "It deals X damage divided evenly, rounded up, among any number of targets.",
        "Put a oil counter on this creature.",
        "Put an time counter on this creature.",
        "Put two loyalty counter on a planeswalker you control.",
        "Remove a counters from target permanent.",
        "Put an oil counter from this creature.",
        "Remove a counter on target permanent.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "crossed Task 10B frame must reject {text:?}",
        );
    }
}

#[test]
fn task10b_scope_rejects_copular_duration_and_global_composite_amounts() {
    let parser = parser();
    let context = context();
    let accepted = [
        "Target creature can't be legendary this turn.",
        "You gain twice X life.",
        "You gain X plus 3 life.",
    ]
    .into_iter()
    .filter(|text| parser.parse(text, &context).is_ok())
    .collect::<Vec<_>>();
    assert!(
        accepted.is_empty(),
        "Task 10B-only syntax leaked through global products: {accepted:?}",
    );
}

#[test]
fn task10c_cost_frames_select_their_complete_typed_paths() {
    let parser = parser();
    let context = context();

    for text in [
        "As an additional cost to cast this spell, discard a card.",
        "You may sacrifice a Mountain rather than pay this spell's mana cost.",
        "You may cast spells from your hand without paying their mana costs.",
        "Spells cost {1} less to cast.",
        "Spells cost {1} more to cast.",
        "This ability costs {1} less to activate for each legendary creature you control.",
        "Cast this spell only if you control a snow land.",
        "Cast this spell only during your turn.",
        "Cast this spell only during your turn and only if you control a snow land.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
}

#[test]
fn task10c_cost_frame_reciprocals_reject_crossed_boundaries() {
    let parser = parser();
    let context = context();

    for text in [
        "As an additional cost cast this spell, discard a card.",
        "As an additional cost to cast, discard a card.",
        "You may sacrifice a Mountain for rather than pay this spell's mana cost.",
        "You may sacrifice a Mountain rather than this spell's mana cost.",
        "You may cast spells from your hand without pay their mana costs.",
        "Spells cost less {1} to cast.",
        "Spells cost {1} less for cast.",
        "Spells costs {1} less to cast.",
        "This ability cost {1} less to activate.",
        "Cast this spell if only you control fewer creatures than each opponent.",
        "Cast this spell only your turn.",
        "Cast this spell only only during your turn.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "crossed Task 10C frame must reject {text:?}",
        );
    }
}

#[test]
fn task10c_cost_scope_rejects_missing_complements_modifiers_and_shortcuts() {
    let parser = parser();
    let context = context();

    for text in [
        "As an additional cost to cast this spell.",
        "As an additional cost to cast, discard a card.",
        "As an additional cost for cast this spell, discard a card.",
        "You may sacrifice a land rather pay this spell's mana cost.",
        "You may sacrifice a land rather than pay mana cost.",
        "You may cast spells without paying mana costs.",
        "Spells cost {} less to cast.",
        "Spells cost {1 less to cast.",
        "Spells cost one less to cast.",
        "Spells cost {1} more less to cast.",
        "Spells cost {1} less to cast for for each creature you control.",
        "Cast only this spell if you control a snow land.",
        "Cast this spell if you control a snow land only.",
        "Cast this spell only only if you control a snow land.",
        "Draw a card only if you control a snow land.",
        "Activate this ability only if you control a snow land.",
        "Draw only a card.",
        "Activate only as a sorcery.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "Task 10C scope must reject {text:?}",
        );
    }
}

#[derive(Default)]
struct Task10cVisitor(Vec<&'static str>);

impl Visitor for Task10cVisitor {
    fn visit_additional_cost(&mut self, value: &AdditionalCost) {
        self.0.push("additional-cost");
        deckmaste_english_v2::visit::walk_additional_cost(self, value);
    }

    fn visit_rather_than_mana_cost_predicate_value(
        &mut self,
        value: &RatherThanManaCostPredicateValue,
    ) {
        self.0.push("rather-than");
        deckmaste_english_v2::visit::walk_rather_than_mana_cost_predicate_value(self, value);
    }

    fn visit_without_paying_mana_cost_predicate_value(
        &mut self,
        value: &WithoutPayingManaCostPredicateValue,
    ) {
        self.0.push("without-paying");
        deckmaste_english_v2::visit::walk_without_paying_mana_cost_predicate_value(self, value);
    }

    fn visit_cost_comparison_predicate_value(&mut self, value: &CostComparisonPredicateValue) {
        self.0.push("cost-comparison");
        deckmaste_english_v2::visit::walk_cost_comparison_predicate_value(self, value);
    }

    fn visit_for_each_cost_basis_value(&mut self, value: &ForEachCostBasisValue) {
        self.0.push("for-each-basis");
        deckmaste_english_v2::visit::walk_for_each_cost_basis_value(self, value);
    }

    fn visit_action_restriction_predicate_value(
        &mut self,
        value: &ActionRestrictionPredicateValue,
    ) {
        self.0.push("action-restriction");
        deckmaste_english_v2::visit::walk_action_restriction_predicate_value(self, value);
    }

    fn visit_only_if_restriction(&mut self, value: &OnlyIfRestriction) {
        self.0.push("only-if");
        deckmaste_english_v2::visit::walk_only_if_restriction(self, value);
    }

    fn visit_only_during_restriction(&mut self, value: &OnlyDuringRestriction) {
        self.0.push("only-during");
        deckmaste_english_v2::visit::walk_only_during_restriction(self, value);
    }

    fn visit_verb_lexeme(&mut self, value: VerbLexeme) {
        if value == VerbLexeme::Cost {
            self.0.push("ordinary-cost-head");
        }
    }

    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        if declaration.kind() == macro_ron::v2::DeclarationKind::KeywordAction {
            match declaration.name() {
                "Cast" => self.0.push("cast-declaration"),
                "Activate" => self.0.push("activate-declaration"),
                _ => {}
            }
        }
    }
}

#[test]
fn task10c_cost_products_keep_ast_render_visit_and_lexical_ownership() {
    let parser = parser();
    let context = context();

    for (text, expected) in [
        (
            "As an additional cost to cast this spell, discard a card.",
            &["additional-cost", "cast-declaration"][..],
        ),
        (
            "You may sacrifice a Mountain rather than pay this spell's mana cost.",
            &["rather-than"][..],
        ),
        (
            "You may cast spells from your hand without paying their mana costs.",
            &["without-paying", "cast-declaration"][..],
        ),
        (
            "Spells cost {1} less to cast.",
            &["cost-comparison", "ordinary-cost-head", "cast-declaration"][..],
        ),
        (
            "This ability costs {1} less to activate for each legendary creature you control.",
            &[
                "cost-comparison",
                "ordinary-cost-head",
                "activate-declaration",
                "for-each-basis",
            ][..],
        ),
        (
            "Cast this spell only during your turn and only if you control a snow land.",
            &[
                "action-restriction",
                "cast-declaration",
                "only-during",
                "only-if",
            ][..],
        ),
    ] {
        let ability = assert_selected_with_specificity(&parser, &context, text, true);
        assert_eq!(ability.render(&context, parser.environment()), text);
        let mut visitor = Task10cVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, expected, "typed visit path for {text:?}");
    }

    let Sentence::Attached(Attached { attachment }) = parser
        .parse_sentence(
            "As an additional cost to cast this spell, discard a card.",
            &context,
        )
        .expect("additional-cost attachment parses")
    else {
        panic!("additional cost has the attached-sentence envelope")
    };
    let ClauseAttachment::AdditionalCost(additional) = attachment.as_ref() else {
        panic!("additional cost retains its dedicated typed attachment")
    };
    assert_eq!(
        AdditionalCost {
            action: additional.action.clone(),
            body: additional.body.clone(),
        },
        *additional.clone(),
    );

    let cost = assert_selected(&parser, &context, "Spells cost {1} less to cast.");
    assert!(matches!(
        cost,
        Ability::Plain(Plain { body })
            if matches!(
                body.as_ref(),
                AbilityBody::Sentences(sentences)
                    if matches!(
                        &sentences.sentences()[0],
                        Sentence::Declarative(Declarative { clause })
                            if matches!(
                                clause.as_ref(),
                                Clause::Finite(finite)
                                    if matches!(
                                        finite.as_ref(),
                                        FiniteClause::PlainFiniteClause(value)
                                            if matches!(
                                                value.predicate(),
                                                Predicate::CostComparison(_)
                                            )
                                    )
                            )
                    )
            )
    ));

    assert_eq!(
        exact_claim_trace(&parser, &context, "Spells cost {1} less to cast."),
        [
            (
                "Spells".to_owned(),
                "lexeme:CommonNoun/Spell/plural".to_owned()
            ),
            (" cost".to_owned(), "lexeme:VerbLexeme/Cost/bare".to_owned()),
            (
                " {".to_owned(),
                "form:symbol_run/symbol_run/0/prefix".to_owned()
            ),
            ("1".to_owned(), "codec:ScalarNumber".to_owned()),
            (
                "}".to_owned(),
                "form:symbol_run/symbol_run/0/suffix".to_owned()
            ),
            (
                " less".to_owned(),
                "vocab:CostComparisonDirection/Less".to_owned()
            ),
            (
                " to".to_owned(),
                "form:controlled_cost_action/controlled_cost_action/0".to_owned()
            ),
            (
                " cast".to_owned(),
                "lexeme:keyword_action/Cast/bare".to_owned()
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
    );
}
