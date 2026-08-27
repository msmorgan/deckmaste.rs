use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::CoreVerbIdentity;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::environment::VerbInventoryRef;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::parser::TextSpan;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use macro_ron::v2::DeclarationIdentity;
use macro_ron::v2::Onset;
use macro_ron::v2::read_str;

#[expect(
    clippy::too_many_lines,
    reason = "the shared predicate fixture lists the declaration environment explicitly"
)]
fn environment() -> ParserEnvironment {
    let declarations = [
        (
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Sacrifice.ron",
            r#"KeywordAction(name:"Sacrifice",spelling:"sacrifice",grammar:Verb(bare:"sacrifice",participle:"sacrificed",valence:Transitive))"#,
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
            r#"KeywordAction(name:"Exile",spelling:"exile",grammar:Verb(bare:"exile",participle:"exiled",valence:Custom(shapes:[[ObjectNounPhrase],[ObjectNounPhrase,PredicativeComplement]])))"#,
        ),
        (
            "/synthetic/actions/Regenerate.ron",
            r#"KeywordAction(name:"Regenerate",spelling:"regenerate",grammar:Verb(bare:"regenerate",participle:"regenerated",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Declare.ron",
            r#"KeywordAction(name:"Declare",spelling:"declare",grammar:Verb(bare:"declare",participle:"declared",valence:Transitive))"#,
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
            r#"KeywordAction(name:"Shuffle",spelling:"shuffle",grammar:Verb(bare:"shuffle",valence:Custom(shapes:[[],[ObjectNounPhrase],[ObjectNounPhrase,Literal("into"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Attach.ron",
            r#"KeywordAction(name:"Attach",spelling:"attach",grammar:Verb(bare:"attach",third_person:"attaches",valence:Custom(shapes:[[ObjectNounPhrase,Literal("to"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Amass.ron",
            r#"KeywordAction(name:"Amass",spelling:"amass",grammar:Verb(bare:"amass",third_person:"amasses",valence:Custom(shapes:[[ObjectNounPhrase,Amount]])))"#,
        ),
        (
            "/synthetic/actions/Clash.ron",
            r#"KeywordAction(name:"Clash",spelling:"clash",grammar:Verb(bare:"clash",third_person:"clashes",valence:Custom(shapes:[[],[Literal("with"),ObjectNounPhrase]])))"#,
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
            "/synthetic/types/Enchantment.ron",
            r#"Type(name:"Enchantment",spelling:"enchantment",grammar:Noun(singular:"enchantment"))"#,
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
            r#"KeywordAction(name:"Activate",spelling:"activate",grammar:Verb(bare:"activate",participle:"activated",valence:Transitive))"#,
        ),
        (
            "/synthetic/actions/Untap.ron",
            r#"KeywordAction(name:"Untap",spelling:"untap",grammar:Verb(bare:"untap",valence:Custom(shapes:[[],[ObjectNounPhrase]])))"#,
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

fn transitive_frame(predicate: &VerbPhrase) -> &TransitivePredicate {
    let VerbPhrase::BaseVerbPhrase(base) = predicate else {
        panic!("transitive predicate has the shared base-frame envelope")
    };
    let BaseVerbFrame::TransitiveFrame(frame) = &base.frame else {
        panic!("transitive predicate has the transitive valence frame")
    };
    let TransitiveFrame::TransitivePredicate(predicate) = frame;
    predicate
}

fn imperative_transitive(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
) -> TransitivePredicate {
    transitive_frame(&imperative_atomic(parser, context, text)).clone()
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
struct PredicateVisitor(Vec<&'static str>);

impl Visitor for PredicateVisitor {
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

    fn visit_predicative_nominal_value(&mut self, value: &PredicativeNominalValue) {
        self.0.push("nominal");
        deckmaste_english_v2::visit::walk_predicative_nominal_value(self, value);
    }

    fn visit_from_anywhere(&mut self, value: &FromAnywhere) {
        self.0.push("from-anywhere");
        deckmaste_english_v2::visit::walk_from_anywhere(self, value);
    }

    fn visit_enter_with_counters(&mut self, value: &EnterWithCounters) {
        self.0.push("enter-with-counters");
        deckmaste_english_v2::visit::walk_enter_with_counters(self, value);
    }

    fn visit_verb_inventory(&mut self, verb: &VerbInventoryRef) {
        if matches!(verb, VerbInventoryRef::Core(CoreVerbIdentity::Enter)) {
            self.0.push("enter-head");
        }
    }

    fn visit_preposed_as(&mut self, value: &PreposedAs) {
        self.0.push("as-clause");
        deckmaste_english_v2::visit::walk_preposed_as(self, value);
    }

    fn visit_modal_passive_subject_gap_relative_clause(
        &mut self,
        value: &ModalPassiveSubjectGapRelativeClause,
    ) {
        self.0.push("modal-passive-relative");
        deckmaste_english_v2::visit::walk_modal_passive_subject_gap_relative_clause(self, value);
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

fn parse_and_visit(parser: &Parser, context: &ParseContext<'_>, text: &str) -> Vec<&'static str> {
    parse_and_visit_with_specificity(parser, context, text, false)
}

fn parse_and_visit_with_specificity(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    permits_specificity: bool,
) -> Vec<&'static str> {
    let ability = assert_selected_with_specificity(parser, context, text, permits_specificity);
    let mut visitor = PredicateVisitor::default();
    visitor.visit_ability(&ability);
    visitor.0
}

#[test]
fn builds_keep_new_products_in_the_existing_typed_algebra() {
    let parser = parser();
    let context = context();

    let text = "Add one mana of any color.";
    let sentence = parser
        .parse_sentence(text, &context)
        .expect("flexible mana frame parses");
    let flexible_mana = imperative_atomic(&parser, &context, text);
    assert!(matches!(
        flexible_mana,
        VerbPhrase::FlexibleMana(FlexibleMana {
            amount: CardinalQuantity::Cardinal(CardinalQuantityValue {
                number: CardinalNumber { magnitude: 1 },
            }),
            kind: FlexibleManaKind::Color,
            ..
        })
    ));
    assert_eq!(sentence.render(&context, parser.environment()), text);
    assert_eq!(
        exact_claim_trace(&parser, &context, text),
        [
            ("Add".to_owned(), "core-verb:Add".to_owned()),
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
        PredicativeComplement::Nominal(_)
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

    let coordinated = assert_selected_with_specificity(
        &parser,
        &context,
        "This permanent is all colors and this creature becomes tapped.",
        true,
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
fn combat_frames_keep_active_valence_passive_agents_and_if_able_distinct() {
    let parser = parser();
    let context = context();

    for (text, expected_path_member) in [
        (
            "Whenever this creature attacks, draw a card.",
            "IntransitiveFrameIntransitivePredicate",
        ),
        (
            "Whenever this creature attacks a player, draw a card.",
            "TransitiveFrameTransitivePredicate",
        ),
        (
            "Whenever this creature blocks a creature, draw a card.",
            "TransitiveFrameTransitivePredicate",
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
        parse_and_visit_with_specificity(
            &parser,
            &context,
            "Whenever this creature blocks or becomes blocked by a creature, draw a card.",
            true,
        ),
        ["predicate-coordination"],
    );
    assert_eq!(
        parse_and_visit(
            &parser,
            &context,
            "Target creature attacks target opponent this turn if able.",
        ),
        ["transitive-requirement"],
    );
    assert_eq!(
        parse_and_visit(
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
fn damage_life_mana_and_continuous_state_use_typed_ordinary_products() {
    let parser = parser();
    let context = context();

    for text in [
        "This creature deals 3 damage to target creature.",
        "You gain 3 life.",
        "Target opponent loses 2 life.",
        "Pay 4 life.",
        "Add {R}{R}{R}.",
        "Add {R} or {G}.",
        "Other creatures you control get +1/+1.",
        "This creature becomes tapped.",
    ] {
        assert_selected(&parser, &context, text);
    }
    assert_eq!(
        parse_and_visit(&parser, &context, "Add one mana of any color."),
        ["flexible-mana"],
    );
    assert_eq!(
        parse_and_visit_with_specificity(&parser, &context, "This permanent is all colors.", true,),
        ["nominal"],
    );
    assert_eq!(
        parse_and_visit_with_specificity(
            &parser,
            &context,
            "This permanent is all colors and this creature becomes tapped.",
            true,
        ),
        ["clause-coordination", "nominal"],
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
fn replacement_entry_and_skip_surfaces_reuse_clause_and_predicate_algebra() {
    let parser = parser();
    let context = context();

    assert_eq!(
        parse_and_visit(
            &parser,
            &context,
            "If a card would be put into your graveyard from anywhere, exile it instead.",
        ),
        ["from-anywhere"],
    );
    assert_eq!(
        parse_and_visit(
            &parser,
            &context,
            "This creature enters with two +1/+1 counters on it.",
        ),
        ["enter-with-counters", "enter-head"],
    );
    assert_eq!(
        parse_and_visit(
            &parser,
            &context,
            "As this artifact enters, choose a color.",
        ),
        ["as-clause", "enter-head"],
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
fn prevention_restriction_requirement_permission_exception_and_if_able_remain_linguistic() {
    let parser = parser();
    let context = context();

    assert_eq!(
        parse_and_visit(
            &parser,
            &context,
            "Prevent the next 3 damage that would be dealt to target creature this turn.",
        ),
        ["modal-passive-relative"],
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
        assert_selected_with_specificity(
            &parser,
            &context,
            text,
            text == "This creature attacks each combat if able.",
        );
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
struct ObjectFrameVisitor(Vec<&'static str>);

impl Visitor for ObjectFrameVisitor {
    fn visit_declared_to_object_predicate(&mut self, value: &DeclaredToObjectPredicate) {
        self.0.push("declared-to-object");
        deckmaste_english_v2::visit::walk_declared_to_object_predicate(self, value);
    }

    fn visit_declared_for_object_predicate(&mut self, value: &DeclaredForObjectPredicate) {
        self.0.push("declared-for-object");
        deckmaste_english_v2::visit::walk_declared_for_object_predicate(self, value);
    }

    fn visit_predicative_scalar_value(&mut self, value: &PredicativeScalarValue) {
        self.0.push("predicative-scalar");
        deckmaste_english_v2::visit::walk_predicative_scalar_value(self, value);
    }

    fn visit_power_toughness_modifier(&mut self, value: &PowerToughnessModifier) {
        self.0.push("power-toughness-modifier");
        deckmaste_english_v2::visit::walk_power_toughness_modifier(self, value);
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
        match value.head.reference() {
            VerbInventoryRef::Core(CoreVerbIdentity::Copy) => self.0.push("copy"),
            VerbInventoryRef::Core(CoreVerbIdentity::Flip) => self.0.push("flip"),
            VerbInventoryRef::Core(CoreVerbIdentity::Lose) => self.0.push("lose-abilities"),
            VerbInventoryRef::Core(CoreVerbIdentity::Unattach) => self.0.push("unattach"),
            _ => {}
        }
        if matches!(
            value.head.reference(),
            VerbInventoryRef::Declaration(id) if id.name() == "Exchange"
        ) {
            self.0.push("exchange");
        }
        deckmaste_english_v2::visit::walk_transitive_predicate(self, value);
    }
}

#[test]
fn declared_to_object_frame_parses_attach_without_a_card_specific_rule() {
    let parser = parser();
    let context = context();
    let text = "Attach target Equipment to target creature.";
    let ability = assert_selected(&parser, &context, text);
    assert!(matches!(
        imperative_atomic(&parser, &context, text),
        VerbPhrase::DeclaredToObjectPredicate(DeclaredToObjectPredicate { .. })
    ));

    let mut visitor = ObjectFrameVisitor::default();
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
fn quote_boundary_recurses_only_through_an_ordinary_ability() {
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

    let mut visitor = ObjectFrameVisitor::default();
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
fn quoted_deferred_interiors_remain_exact_ordinary_failures() {
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
                "deferred quote interior must not select: {text:?}",
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
                .expect_err("deferred quote interior remains an ordinary failure");
            let deckmaste_english_v2::parser::ParseError::Failure { span, .. } = error else {
                panic!("deferred quote interior has exact ordinary failure class: {error:?}")
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
fn inventory_information_heads_parse_copy_and_flip_without_action_declarations() {
    let parser = parser();
    let context = context();
    for (text, expected_head, expected_visit) in [
        (
            "Copy target instant or sorcery spell.",
            CoreVerbIdentity::Copy,
            "copy",
        ),
        ("Flip a coin.", CoreVerbIdentity::Flip, "flip"),
        (
            "Unattach that Equipment.",
            CoreVerbIdentity::Unattach,
            "unattach",
        ),
    ] {
        let ability = assert_selected_with_specificity(
            &parser,
            &context,
            text,
            text == "Copy target instant or sorcery spell.",
        );
        let predicate = imperative_transitive(&parser, &context, text);
        assert_eq!(
            predicate.head.reference(),
            &VerbInventoryRef::Core(expected_head),
            "{text:?}",
        );
        let mut visitor = ObjectFrameVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, [expected_visit], "{text:?}");
    }
}

#[test]
fn exchange_uses_declared_object_valence_and_a_typed_control_reference() {
    let parser = parser();
    let context = context();
    let text = "Exchange control of two target creatures.";
    let ability = assert_selected(&parser, &context, text);
    let predicate = imperative_transitive(&parser, &context, text);
    assert!(matches!(
        predicate.head.reference(),
        VerbInventoryRef::Declaration(id) if id.name() == "Exchange"
    ));
    assert!(matches!(predicate.object, Object::ObjectNominal(_)));
    let mut visitor = ObjectFrameVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["exchange"]);
}

#[test]
fn declared_custom_valences_select_one_frame_per_linguistic_shape() {
    let parser = parser();
    let context = context();

    for text in [
        "Amass Slivers 2.",
        "Clash with target opponent.",
        "Exchange target creature with target artifact.",
        "Exchange target creature for target artifact.",
        "Shuffle target card into its owner's library.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
}

#[test]
fn vote_uses_declared_for_object_valence_and_productive_common_noun_choices() {
    let parser = parser();
    let context = context();
    let text = "Starting with her, each player votes for death or taxes.";
    let ability = assert_selected_with_specificity(&parser, &context, text, true);
    let Ability::Plain(Plain { body }) = &ability else {
        panic!("vote witness has an ordinary ability envelope")
    };
    let AbilityBody::Sentences(sentences) = body.as_ref() else {
        panic!("vote witness has an ordinary sentence body")
    };
    let Sentence::Attached(Attached { attachment }) = &sentences.sentences()[0] else {
        panic!("vote order stays a typed preposed clause attachment")
    };
    let ClauseAttachment::StartingWith(starting) = attachment.as_ref() else {
        panic!("vote order stays a typed preposed clause attachment")
    };
    assert!(matches!(
        starting.starter,
        Object::ObjectPronoun(PersonalObject {
            word: ObjectPronoun::Her,
        })
    ));
    let mut visitor = ObjectFrameVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["declared-for-object"]);

    for productive in [
        "Starting with you, each player votes for card or token.",
        "Starting with target player, each player votes for card or token.",
    ] {
        let ability = assert_selected_with_specificity(&parser, &context, productive, true);
        let mut visitor = ObjectFrameVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, ["declared-for-object"]);
    }
}

#[test]
fn maximum_hand_size_is_a_typed_copular_scalar_statement() {
    let parser = parser();
    let context = context();
    let text = "Your maximum hand size is seven.";
    let ability = assert_selected_with_specificity(&parser, &context, text, true);
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
    let mut visitor = ObjectFrameVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["predicative-scalar"]);
}

#[test]
fn token_descriptions_share_typed_power_toughness_and_copy_constituents() {
    let parser = parser();
    let context = context();
    for (text, expected_visit) in [
        (
            "Create a 1/1 red Goblin creature token.",
            Some("power-toughness-modifier"),
        ),
        ("Create a token that's a copy of target creature.", None),
    ] {
        let ability = assert_selected(&parser, &context, text);
        let predicate = imperative_transitive(&parser, &context, text);
        assert!(
            matches!(
                predicate.head.reference(),
                VerbInventoryRef::Declaration(id) if id.name() == "Create"
            ),
            "{text:?}"
        );
        let mut visitor = ObjectFrameVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(
            visitor.0,
            expected_visit.into_iter().collect::<Vec<_>>(),
            "{text:?}"
        );
    }
}

#[test]
fn power_toughness_predicates_keep_modifier_and_base_value_frames_distinct() {
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
        let mut visitor = ObjectFrameVisitor::default();
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
struct PowerToughnessVisitor(Vec<&'static str>);

impl Visitor for PowerToughnessVisitor {
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
fn negative_adjustments_build_render_visit_and_claim_the_typed_sign_product() {
    let parser = parser();
    let context = context();
    let text = "Target creature gets -1/-1 until end of turn.";
    let ability = assert_selected(&parser, &context, text);
    let VerbPhrase::GetPowerToughness(GetPowerToughness {
        adjustment: PowerToughnessAdjustment::PowerToughnessAdjustment(adjustment),
        duration: Some(duration),
    }) = declarative_atomic(&parser, &context, text)
    else {
        panic!("negative adjustment stays in the ordinary typed gets frame")
    };
    assert!(matches!(duration, DurationPhrase::Until(_)));
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

    let mut visitor = PowerToughnessVisitor::default();
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
                "core-verb:Get".to_owned(),
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
                " until".to_owned(),
                "form:until_duration_phrase/until_duration_phrase/0".to_owned(),
            ),
            (
                " end".to_owned(),
                "vocab:TemporalBoundary/End".to_owned(),
            ),
            (
                " of".to_owned(),
                "form:bare_boundary_temporal_endpoint/bare_boundary_temporal_endpoint/1"
                    .to_owned(),
            ),
            (
                " turn".to_owned(),
                "vocab:TemporalUnit/Turn".to_owned(),
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
    );
}

#[test]
fn preposed_duration_attaches_to_finite_and_imperative_bodies() {
    let parser = parser();
    let context = context();

    for text in [
        "Until end of turn, target creature gets +1/+1.",
        "Until end of turn, gain 1 life.",
        "Until your next turn, target creature gets +1/+1.",
        "Until the end of your next turn, gain 1 life.",
    ] {
        assert_selected(&parser, &context, text);
    }

    assert!(
        parser
            .parse("Until the end your next turn, gain 1 life.", &context,)
            .is_err(),
        "boundary complements require the preposition of",
    );
}

#[test]
fn fixed_variable_asymmetric_and_crossed_adjustments_share_existing_amounts() {
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
        let mut visitor = PowerToughnessVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, expected_visits, "{text:?}");
    }

    let text = "This creature gets -1/-1 and target creature gets +1/+1.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = PowerToughnessVisitor::default();
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
fn adjustment_sign_slash_pairing_and_agreement_boundaries_are_reciprocal() {
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
fn ordinary_ability_nouns_parse_while_keyword_interiors_remain_deferred() {
    let parser = parser();
    let context = context();
    let text = "Target creature loses all abilities.";
    let ability = assert_selected(&parser, &context, text);
    let predicate = transitive_frame(&declarative_atomic(&parser, &context, text)).clone();
    assert!(matches!(
        predicate.head.reference(),
        VerbInventoryRef::Core(CoreVerbIdentity::Lose)
    ));
    let mut visitor = ObjectFrameVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["lose-abilities"]);

    for deferred_keyword_interior in [
        "Target creature gains flying until end of turn.",
        "Creatures you control have vigilance.",
    ] {
        assert!(
            parser.parse(deferred_keyword_interior, &context).is_err(),
            "bare keyword ability remains a deferred boundary: {deferred_keyword_interior:?}",
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
    let predicate = transitive_frame(predicate.as_ref());
    assert!(matches!(
        predicate.head.reference(),
        VerbInventoryRef::Declaration(id) if id.name() == "Sacrifice"
    ));

    let sentence = parser
        .parse_sentence("Control target player.", &context)
        .expect("core transitive imperative parses");
    let Sentence::Imperative(imperative) = sentence else {
        panic!("bare predicate has an imperative envelope")
    };
    let Predicate::Atomic(predicate) = imperative.predicate() else {
        panic!("core object verb has an atomic predicate envelope")
    };
    let predicate = transitive_frame(predicate.as_ref());
    assert!(matches!(
        predicate.head.reference(),
        VerbInventoryRef::Core(CoreVerbIdentity::Control)
    ));
}

#[derive(Default)]
struct LexicalVisitor(Vec<String>);

impl Visitor for LexicalVisitor {
    fn visit_declaration(&mut self, declaration: &DeclarationIdentity) {
        self.0.push(format!("declared:{}", declaration.name()));
    }

    fn visit_verb_inventory(&mut self, verb: &VerbInventoryRef) {
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
            (32, 39, "core-verb:Control".to_owned()),
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
    assert_selected_with_specificity(&parser, &context, "Enter target player.", true);
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
        assert_selected_with_specificity(
            &parser,
            &context,
            text,
            matches!(
                text,
                "It is legendary."
                    | "They are white."
                    | "It was tapped."
                    | "They were 2/2."
                    | "It is a creature."
                    | "It is able to attack."
                    | "It is dealt damage."
                    | "It is dealt combat damage."
                    | "It is put into your graveyard from the battlefield."
                    | "It is turned face up."
                    | "A spell was cast."
            ),
        );
    }
}

#[test]
#[allow(
    clippy::items_after_statements,
    clippy::match_same_arms,
    clippy::too_many_lines,
    reason = "one exhaustive integration table keeps every finite clause family and AST oracle together"
)]
fn finite_clause_families_compose_in_triggers_and_conditions() {
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
        let ability = assert_selected_with_specificity(
            parser,
            context,
            text,
            match expected {
                // Each integrated finite-clause family also has a compositional
                // finite-predicate derivation; the dedicated clause envelope wins.
                IntegratedClause::Damage(_)
                | IntegratedClause::Movement
                | IntegratedClause::Orientation
                | IntegratedClause::CopularCondition => true,
            },
        );
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
                    head.reference(),
                    VerbInventoryRef::Core(CoreVerbIdentity::Deal)
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
        ($text:literal, $family:expr, [$(($surface:literal, $owner:literal $(,)?)),+ $(,)?]) => {
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
            (
                " this",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "core-verb:Deal"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" it", "vocab:SubjectPronoun/It"),
            (" deals", "core-verb:Deal"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" damage", "form:deal_amount_damage/deal_amount_damage/2"),
            (" to", "form:to_phrase/to_phrase/0"),
            (" you", "vocab:ObjectPronoun/You"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt damage, you gain 1 life.",
        IntegratedClause::Damage(DamageKind::Ordinary),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (
                " this",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "core-verb:Deal"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" you", "vocab:SubjectPronoun/You"),
            (" gain", "core-verb:Gain"),
            (" 1", "codec:ScalarNumber"),
            (" life", "form:life_amount/life_amount/2"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt damage, it deals that much damage to each player.",
        IntegratedClause::Damage(DamageKind::Ordinary),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (
                " this",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "core-verb:Deal"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" it", "vocab:SubjectPronoun/It"),
            (" deals", "core-verb:Deal"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" damage", "form:deal_amount_damage/deal_amount_damage/2"),
            (" to", "form:to_phrase/to_phrase/0"),
            (" each", "determinative:DeterminativeHead/Each"),
            (" player", "lexeme:CommonNoun/Player/singular"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );

    assert_selected_with_specificity(
        &parser,
        &context,
        "Whenever this creature is dealt damage, it deals that much damage to target opponent or planeswalker.",
        true,
    );
    assert_selected_with_specificity(
        &parser,
        &context,
        "At the beginning of each player's end step, if that player didn't cast a spell this turn, this enchantment deals 4 damage to that player.",
        true,
    );

    const MOVEMENT_TEXT: &str =
        "Whenever a creature is put into your graveyard from the battlefield, you gain 1 life.";
    const MOVEMENT_CLAIMS: &[(&str, &str)] = &[
        ("Whenever", "vocab:TriggerMarker/Whenever"),
        (" a", "determinative:DeterminativeHead/IndefiniteArticle"),
        (" creature", "lexeme:type/Creature/singular"),
        (" is", "vocab:FiniteCopula/Is"),
        (" put", "core-verb:Put"),
        (" into", "form:into_phrase/into_phrase/0"),
        (" your", "vocab:PossessiveDeterminerPronoun/Your"),
        (" graveyard", "lexeme:CommonNoun/Graveyard/singular"),
        (" from", "form:from_phrase/from_phrase/0"),
        (" the", "determinative:DeterminativeHead/DefiniteArticle"),
        (" battlefield", "lexeme:CommonNoun/Battlefield/singular"),
        (",", "form:triggered/triggered/1"),
        (" you", "vocab:SubjectPronoun/You"),
        (" gain", "core-verb:Gain"),
        (" 1", "codec:ScalarNumber"),
        (" life", "form:life_amount/life_amount/2"),
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
        "Whenever a permanent is turned face up, this creature deals 1 damage to target creature.",
        IntegratedClause::Orientation,
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" a", "determinative:DeterminativeHead/IndefiniteArticle"),
            (" permanent", "lexeme:CommonNoun/Permanent/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" turned", "core-verb:Turn"),
            (" face up", "vocab:FaceOrientation/FaceUp"),
            (",", "form:triggered/triggered/1"),
            (
                " this",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" deals", "core-verb:Deal"),
            (" 1", "codec:ScalarNumber"),
            (" damage", "form:deal_amount_damage/deal_amount_damage/2"),
            (" to", "form:to_phrase/to_phrase/0"),
            (" target", "determinative:DeterminativeHead/Target"),
            (" creature", "lexeme:type/Creature/singular"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt combat damage, you gain that much life.",
        IntegratedClause::Damage(DamageKind::Combat),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (
                " this",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "core-verb:Deal"),
            (" combat damage", "vocab:DamageKind/Combat"),
            (",", "form:triggered/triggered/1"),
            (" you", "vocab:SubjectPronoun/You"),
            (" gain", "core-verb:Gain"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" life", "form:life_amount/life_amount/2"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt damage, each opponent gains that much life.",
        IntegratedClause::Damage(DamageKind::Ordinary),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (
                " this",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "core-verb:Deal"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" each", "determinative:DeterminativeHead/Each"),
            (" opponent", "lexeme:CommonNoun/Opponent/singular"),
            (" gains", "core-verb:Gain"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" life", "form:life_amount/life_amount/2"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever this creature is dealt damage, it deals that much damage to target creature.",
        IntegratedClause::Damage(DamageKind::Ordinary),
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (
                " this",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" is", "vocab:FiniteCopula/Is"),
            (" dealt", "core-verb:Deal"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (",", "form:triggered/triggered/1"),
            (" it", "vocab:SubjectPronoun/It"),
            (" deals", "core-verb:Deal"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" damage", "form:deal_amount_damage/deal_amount_damage/2"),
            (" to", "form:to_phrase/to_phrase/0"),
            (" target", "determinative:DeterminativeHead/Target"),
            (" creature", "lexeme:type/Creature/singular"),
            (".", "structural:Sentences/sentences/terminator/0")
        ]
    );
    selected!(
        "Whenever a creature you control is put into your graveyard from the battlefield, you gain 1 life.",
        IntegratedClause::Movement,
        [
            ("Whenever", "vocab:TriggerMarker/Whenever"),
            (" a", "determinative:DeterminativeHead/IndefiniteArticle"),
            (" creature", "lexeme:type/Creature/singular"),
            (" you", "vocab:SubjectPronoun/You"),
            (" control", "core-verb:Control"),
            (" is", "vocab:FiniteCopula/Is"),
            (" put", "core-verb:Put"),
            (" into", "form:into_phrase/into_phrase/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" graveyard", "lexeme:CommonNoun/Graveyard/singular"),
            (" from", "form:from_phrase/from_phrase/0"),
            (" the", "determinative:DeterminativeHead/DefiniteArticle"),
            (" battlefield", "lexeme:CommonNoun/Battlefield/singular"),
            (",", "form:triggered/triggered/1"),
            (" you", "vocab:SubjectPronoun/You"),
            (" gain", "core-verb:Gain"),
            (" 1", "codec:ScalarNumber"),
            (" life", "form:life_amount/life_amount/2"),
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
            (" all", "determinative:DeterminativeHead/All"),
            (" creatures", "lexeme:type/Creature/plural"),
            (" are", "vocab:FiniteCopula/Are"),
            (" white", "vocab:Color/White"),
            (",", "form:finite_condition/finite_condition/2"),
            (" you", "vocab:SubjectPronoun/You"),
            (" gain", "core-verb:Gain"),
            (" 1", "codec:ScalarNumber"),
            (" life", "form:life_amount/life_amount/2"),
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
fn ast_keeps_each_linguistic_product_typed() {
    let parser = parser();
    let context = context();
    for (text, expected) in [
        ("It is legendary.", "adjective"),
        ("They are white.", "color"),
        ("It is a creature.", "nominal"),
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
            PredicativeComplement::Nominal(_) => "nominal",
            PredicativeComplement::Status(_) => "status",
            PredicativeComplement::Ability(_) => "ability",
            PredicativeComplement::Orientation(_) => "orientation",
            PredicativeComplement::PowerToughness(_) => "power-toughness",
            PredicativeComplement::Scalar(_) => "scalar",
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
        ("A spell was cast from a graveyard.", "declared-from"),
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
            PassivePredicate::DeclaredTransitiveFrom(_) => "declared-from",
            PassivePredicate::DeclaredToObject(_) => "declared-to",
        };
        assert_eq!(observed, expected);
    }

    assert!(matches!(
        declarative_atomic(&parser, &context, "It deals damage."),
        VerbPhrase::DealUnspecifiedDamage(DealDamageKind { .. })
    ));
    assert!(matches!(
        declarative_atomic(&parser, &context, "It gains life."),
        VerbPhrase::GainUnspecifiedLife(GainUnspecifiedLife { .. })
    ));
}

#[derive(Default)]
struct TypedProductVisitor(Vec<String>);

macro_rules! typed_product {
    ($method:ident, $type:ty, $walk:ident, $label:literal) => {
        fn $method(&mut self, value: &$type) {
            self.0.push(concat!("product:", $label).to_owned());
            deckmaste_english_v2::visit::$walk(self, value);
        }
    };
}

impl Visitor for TypedProductVisitor {
    typed_product!(
        visit_copular_clause_value,
        CopularClauseValue,
        walk_copular_clause_value,
        "CopularClauseValue"
    );
    typed_product!(
        visit_predicative_adjective_value,
        PredicativeAdjectiveValue,
        walk_predicative_adjective_value,
        "PredicativeAdjectiveValue"
    );
    typed_product!(
        visit_predicative_color_value,
        PredicativeColorValue,
        walk_predicative_color_value,
        "PredicativeColorValue"
    );
    typed_product!(
        visit_predicative_nominal_value,
        PredicativeNominalValue,
        walk_predicative_nominal_value,
        "PredicativeNominalValue"
    );
    typed_product!(
        visit_predicative_status_value,
        PredicativeStatusValue,
        walk_predicative_status_value,
        "PredicativeStatusValue"
    );
    typed_product!(
        visit_blocked_by_status_value,
        BlockedByStatusValue,
        walk_blocked_by_status_value,
        "BlockedByStatusValue"
    );
    typed_product!(
        visit_predicative_ability_value,
        PredicativeAbilityValue,
        walk_predicative_ability_value,
        "PredicativeAbilityValue"
    );
    typed_product!(
        visit_predicative_power_toughness_value,
        PredicativePowerToughnessValue,
        walk_predicative_power_toughness_value,
        "PredicativePowerToughnessValue"
    );
    typed_product!(
        visit_bare_copular_predicate_value,
        BareCopularPredicateValue,
        walk_bare_copular_predicate_value,
        "BareCopularPredicateValue"
    );
    typed_product!(
        visit_change_state_predicate_value,
        ChangeStatePredicateValue,
        walk_change_state_predicate_value,
        "ChangeStatePredicateValue"
    );
    typed_product!(
        visit_auxiliary_predicate_value,
        AuxiliaryPredicateValue,
        walk_auxiliary_predicate_value,
        "AuxiliaryPredicateValue"
    );
    typed_product!(
        visit_bare_passive_predicate_value,
        BarePassivePredicateValue,
        walk_bare_passive_predicate_value,
        "BarePassivePredicateValue"
    );
    typed_product!(
        visit_passive_finite_clause_value,
        PassiveFiniteClauseValue,
        walk_passive_finite_clause_value,
        "PassiveFiniteClauseValue"
    );
    typed_product!(
        visit_passive_damage_predicate_value,
        PassiveDamagePredicateValue,
        walk_passive_damage_predicate_value,
        "PassiveDamagePredicateValue"
    );
    typed_product!(
        visit_passive_movement_predicate_value,
        PassiveMovementPredicateValue,
        walk_passive_movement_predicate_value,
        "PassiveMovementPredicateValue"
    );
    typed_product!(
        visit_passive_orientation_predicate_value,
        PassiveOrientationPredicateValue,
        walk_passive_orientation_predicate_value,
        "PassiveOrientationPredicateValue"
    );
    typed_product!(
        visit_declared_transitive_passive_predicate_value,
        DeclaredTransitivePassivePredicateValue,
        walk_declared_transitive_passive_predicate_value,
        "DeclaredTransitivePassivePredicateValue"
    );
    typed_product!(
        visit_deal_damage_kind,
        DealDamageKind,
        walk_deal_damage_kind,
        "DealDamageKind"
    );
    typed_product!(
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
            let ability = assert_selected_with_specificity(
                &parser,
                &context,
                $text,
                matches!(
                    $text,
                    "It is legendary."
                        | "They are white."
                        | "It was tapped."
                        | "They were 2/2."
                        | "It is a creature."
                        | "It is able to attack."
                        | "It is dealt damage."
                        | "It is dealt combat damage."
                        | "It is put into your graveyard from the battlefield."
                        | "It is turned face up."
                        | "A spell was cast."
                ),
            );
            let mut visitor = TypedProductVisitor::default();
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
        [
            "product:CopularClauseValue",
            "product:PredicativeNominalValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" is", "vocab:FiniteCopula/Is"),
            (" a", "form:indefinite_reference/a/0"),
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
            (" attack", "core-verb:Attack"),
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
        ["product:AuxiliaryPredicateValue"],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" didn't", "lexeme:VerbLexeme/Didnt/third_person_singular"),
            (" cast", "lexeme:keyword_action/Cast/bare"),
            (" a", "form:indefinite_reference/a/0"),
            (" spell", "lexeme:CommonNoun/Spell/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It would attack.",
        ["product:AuxiliaryPredicateValue"],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" would", "lexeme:VerbLexeme/Would/third_person_singular"),
            (" attack", "core-verb:Attack"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It would be white.",
        [
            "product:AuxiliaryPredicateValue",
            "product:BareCopularPredicateValue",
            "product:PredicativeColorValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" would", "lexeme:VerbLexeme/Would/third_person_singular"),
            (" be", "vocab:BareCopula/Be"),
            (" white", "vocab:Color/White"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It can't be dealt damage.",
        [
            "product:AuxiliaryPredicateValue",
            "product:BarePassivePredicateValue",
            "product:PassiveDamagePredicateValue"
        ],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" can't", "lexeme:VerbLexeme/Cant/third_person_singular"),
            (" be", "vocab:BareCopula/Be"),
            (" dealt", "core-verb:Deal"),
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
            (" dealt", "core-verb:Deal"),
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
            (" put", "core-verb:Put"),
            (" into", "form:into_phrase/into_phrase/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" graveyard", "lexeme:CommonNoun/Graveyard/singular"),
            (" from", "form:from_phrase/from_phrase/0"),
            (
                " the",
                "form:definite_singular_reference/definite_singular_reference/0"
            ),
            (" battlefield", "lexeme:CommonNoun/Battlefield/singular"),
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
            (" turned", "core-verb:Turn"),
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
        ["product:DealDamageKind"],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" deals", "core-verb:Deal"),
            (" damage", "vocab:DamageKind/Ordinary"),
            (".", TERMINATOR)
        ]
    );
    assert_frame!(
        "It gains life.",
        ["product:GainUnspecifiedLife"],
        [
            ("It", "vocab:SubjectPronoun/It"),
            (" gains", "core-verb:Gain"),
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
        "Deal X damage to target creature.",
        "It deals damage equal to its power to target creature.",
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
        OnPhrase::OnPhrase(OnPhraseValue {
            complement,
        }) if matches!(
            complement.as_ref(),
            Object::ObjectPronoun(PersonalObject {
                word: ObjectPronoun::It,
            })
        )
    ));

    let sentence = parser
        .parse_sentence(
            "It deals damage equal to its power to target creature.",
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
fn scalar_values_compose_genitives_counts_and_post_recipient_equalities() {
    let parser = parser();
    let context = context();

    for text in [
        "Draw cards equal to the blue creature's toughness.",
        "Draw cards equal to the sacrificed creature's toughness.",
        "Deal damage equal to the number of Slivers you control to target artifact.",
        "Deal damage to target artifact equal to the number of Slivers you control.",
        "Gain life equal to twice the number of Slivers you control.",
        "Gain life equal to one plus the number of Slivers you control.",
        "Draw cards equal to the greatest power among creatures you control.",
        "Draw cards equal to the number of artifacts they control.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    let draw = imperative_atomic(
        &parser,
        &context,
        "Draw cards equal to the blue creature's toughness.",
    );
    let VerbPhrase::DrawCardsEqualTo(DrawCardsEqualTo { equality, .. }) = draw else {
        panic!("genitive scalar selects the existing draw-equality frame")
    };
    let ScalarEquality::ScalarEquality(ScalarEqualityValue { value }) = equality;
    assert!(matches!(value, ScalarValue::GenitiveScalarValue(_)));

    for text in ["Destroy the chosen creature.", "Destroy the exiled card."] {
        assert_selected_with_specificity(&parser, &context, text, true);
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected definite reduced-relative witness has a decision");
        let path = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("selected ordinal names a candidate")
            .construction_path();
        assert!(
            path.iter().all(|name| !name.contains("Designated")),
            "{text:?}: {path:?}",
        );
        assert!(
            decision.candidates().iter().all(|candidate| candidate
                .construction_path()
                .iter()
                .all(|name| !name.contains("Designated"))),
            "{text:?}: {decision:?}",
        );
    }

    let damage = imperative_atomic(
        &parser,
        &context,
        "Deal damage to target artifact equal to the number of Slivers you control.",
    );
    let VerbPhrase::DealDamageToEqualTo(DealDamageToEqualTo {
        recipient,
        equality,
        ..
    }) = damage
    else {
        panic!("post-recipient equality selects its ordered damage frame")
    };
    assert!(matches!(recipient, ToPhrase::ToPhrase(_)));
    let ScalarEquality::ScalarEquality(ScalarEqualityValue { value }) = equality;
    assert!(matches!(value, ScalarValue::NumberOfScalarValue(_)));

    for (text, expected) in [
        (
            "Gain life equal to twice the number of Slivers you control.",
            "twice",
        ),
        (
            "Gain life equal to one plus the number of Slivers you control.",
            "offset",
        ),
        (
            "Draw cards equal to the greatest power among creatures you control.",
            "greatest",
        ),
    ] {
        let predicate = imperative_atomic(&parser, &context, text);
        let equality = match predicate {
            VerbPhrase::LifeEquality(LifeEquality { equality, .. })
            | VerbPhrase::DrawCardsEqualTo(DrawCardsEqualTo { equality, .. }) => equality,
            other => panic!("{expected} scalar selected wrong frame: {other:?}"),
        };
        let ScalarEquality::ScalarEquality(ScalarEqualityValue { value }) = equality;
        assert!(
            matches!(
                (expected, value),
                ("twice", ScalarValue::TwiceScalarValue(_))
                    | ("offset", ScalarValue::OffsetScalarValue(_))
                    | ("greatest", ScalarValue::GreatestScalarValue(_))
            ),
            "{expected} scalar retains its compositional AST",
        );
    }

    for malformed in [
        "Draw cards equal to the blue creature toughness.",
        "Draw cards equal to the blue creatures' toughness.",
        "Deal damage to target artifact equal the number of Slivers you control.",
        "Deal damage to target artifact the number of Slivers you control.",
        "Gain life equal to twice number of Slivers you control.",
        "Gain life equal to one the number of Slivers you control.",
        "Draw cards equal to greatest power among creatures you control.",
        "Draw cards equal to the number of artifacts they controls.",
    ] {
        assert!(
            parser.parse(malformed, &context).is_err(),
            "malformed scalar composition must reject {malformed:?}",
        );
    }
}

#[test]
fn object_gap_relatives_follow_subject_agreement() {
    let parser = parser();
    let context = context();

    for text in [
        "Creatures target player controls get -2/-2 until end of turn.",
        "It deals X damage divided evenly, rounded down, among all creatures target opponent controls.",
        "Destroy target creature you don't control.",
        "Destroy target creature that player doesn't control.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    for malformed in [
        "Creatures target player control get -2/-2 until end of turn.",
        "It deals X damage divided evenly, rounded down, among all creatures target opponent control.",
        "Destroy target creature you doesn't control.",
    ] {
        assert!(
            parser.parse(malformed, &context).is_err(),
            "singular determiner controller must reject bare agreement in {malformed:?}",
        );
    }
}

#[test]
fn floated_subject_quantifiers_relay_plural_agreement() {
    let parser = parser();
    let context = context();

    for text in [
        "They each get +2/+2 until end of turn.",
        "Creatures each get +1/+1 this turn.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    assert!(
        parser
            .parse("It each gets +2/+2 until end of turn.", &context)
            .is_err(),
        "floated each requires a plural-agreement subject",
    );
}

#[test]
fn compositional_for_phrases_attach_to_atomic_predicates() {
    let parser = parser();
    let context = context();

    for text in [
        "You gain 1 life for each creature.",
        "Add {G} for each land you control.",
        "Add {G} for each Goblin on the battlefield.",
        "This creature gets +1/+1 for each creature you control.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    assert!(
        parser
            .parse("You gain 1 life for each controls.", &context)
            .is_err(),
        "for requires an independently parsed complement",
    );
}

#[test]
fn subject_sharing_modal_predicates_keep_their_bare_complement() {
    let parser = parser();
    let context = context();

    assert_selected(
        &parser,
        &context,
        "This creature gets +1/+0 and can't be blocked.",
    );
    assert_selected(&parser, &context, "Creatures attack and can't be blocked.");
    assert_selected_with_specificity(
        &parser,
        &context,
        "You may draw a card and discard a card.",
        true,
    );
    assert_selected_with_specificity(
        &parser,
        &context,
        "You may draw a card, then discard a card.",
        true,
    );
    assert_selected(&parser, &context, "This creature can't be blocked.");
    assert!(
        parser
            .parse("This creature gets +1/+0 and can't blocked.", &context)
            .is_err(),
    );
    assert!(
        parser.parse("You may can draw a card.", &context).is_err(),
        "a central auxiliary cannot recursively select another central auxiliary",
    );
    for text in [
        "You may can draw a card and discard a card.",
        "You may can draw a card, then discard a card.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "a coordinated bare complement cannot hide a central auxiliary: {text:?}",
        );
    }
}

#[test]
fn postposed_as_long_as_attaches_to_clauses_and_bare_predicates() {
    let parser = parser();
    let context = context();

    assert_selected(
        &parser,
        &context,
        "This creature gets +1/+1 as long as you control a Forest.",
    );
    assert_selected_with_specificity(
        &parser,
        &context,
        "Draw a card as long as you control a Forest.",
        true,
    );
    assert!(
        parser
            .parse("Draw a card as long you control a Forest.", &context)
            .is_err(),
    );
}

#[test]
fn temporal_casting_restrictions_take_finite_clause_complements() {
    let parser = parser();
    let context = context();

    for text in [
        "Cast this spell only before attackers are declared.",
        "Cast this spell only after blockers are declared.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
    assert!(
        parser
            .parse("Cast this spell only before attackers declared.", &context)
            .is_err(),
    );
}

#[test]
fn postposed_while_forms_a_clause_inside_and_outside_triggers() {
    let parser = parser();
    let context = context();

    assert_selected(
        &parser,
        &context,
        "This creature attacks while you control a Forest.",
    );
    assert_selected_with_specificity(
        &parser,
        &context,
        "Whenever this creature attacks while you control a Forest, draw a card.",
        true,
    );
    assert!(
        parser
            .parse("This creature attacks while control a Forest.", &context)
            .is_err(),
    );
}

#[test]
fn preposed_for_phrases_attach_to_clauses_and_bare_predicates() {
    let parser = parser();
    let context = context();

    assert_selected_with_specificity(
        &parser,
        &context,
        "For each creature, you draw a card.",
        true,
    );
    assert_selected_with_specificity(&parser, &context, "For each creature, draw a card.", true);
    assert!(
        parser
            .parse("For each creature draw a card.", &context)
            .is_err(),
    );
}

#[test]
fn reduced_passive_relatives_postmodify_nominal_references() {
    let parser = parser();
    let context = context();

    assert_selected(
        &parser,
        &context,
        "Exile target Equipment attached to that creature.",
    );
    assert_selected_with_specificity(
        &parser,
        &context,
        "Put an Equipment card from your hand onto the battlefield attached to this creature.",
        true,
    );
    assert!(
        parser
            .parse("Exile target Equipment attached that creature.", &context)
            .is_err(),
    );
}

#[test]
fn comparative_quantifiers_select_plural_nominals_and_standards() {
    let parser = parser();
    let context = context();

    for text in [
        "You control fewer creatures than each opponent.",
        "You control more creatures than target player.",
    ] {
        assert_selected(&parser, &context, text);
    }
    assert!(
        parser
            .parse("You control fewer creature than each opponent.", &context)
            .is_err(),
    );
}

#[test]
fn coordinated_nominal_modifiers_share_one_head() {
    let parser = parser();
    let context = context();

    for text in [
        "Destroy target attacking or blocking creature.",
        "Destroy target red and legendary creature.",
        "Destroy all artifact, enchantment, and planeswalker tokens.",
    ] {
        assert_selected(&parser, &context, text);
    }
    assert!(
        parser
            .parse("Destroy target attacking or creature.", &context)
            .is_err(),
    );
}

#[test]
fn modal_object_gap_relatives_require_bare_transitive_heads() {
    let parser = parser();
    let context = context();

    assert_selected(
        &parser,
        &context,
        "Exile a card target player would discard.",
    );
    assert!(
        parser
            .parse("Exile a card target player would discards.", &context)
            .is_err(),
    );
}

#[test]
fn coordinated_scalar_degrees_precede_their_measure() {
    let parser = parser();
    let context = context();

    assert_selected(
        &parser,
        &context,
        "Destroy target creature with equal or lesser power.",
    );
    assert!(
        parser
            .parse("Destroy target creature with equal lesser power.", &context)
            .is_err(),
    );
}

#[test]
fn resultative_complements_compose_inside_instead_scope() {
    let parser = parser();
    let context = context();

    assert_selected(&parser, &context, "Exile that card face up instead.");
    assert!(
        parser
            .parse("Exile that card face instead.", &context)
            .is_err(),
    );
    assert_selected(&parser, &context, "Untap this creature.");
    let Err(deckmaste_english_v2::parser::ParseError::Failure { span, .. }) =
        parser.parse("Untap this creature face up.", &context)
    else {
        panic!("a well-formed transitive frame cannot consume an undeclared resultative tail")
    };
    assert_eq!(
        &"Untap this creature face up."[span.start..span.end],
        "face",
        "rejection begins exactly where the unselected predicative complement starts",
    );
}

#[test]
fn modal_subject_gap_relatives_modify_ordinary_mass_noun_phrases() {
    let parser = parser();
    let context = context();

    let text = "Prevent all damage that would be dealt to you this turn.";
    let Sentence::Imperative(sentence) = parser.parse_sentence(text, &context).unwrap() else {
        panic!("the witness is imperative")
    };
    let Predicate::Duration(DurationPredicate::DurationPredicate(DurationPredicateValue {
        predicate,
        ..
    })) = sentence.predicate()
    else {
        panic!("the subject-relative NP remains inside an ordinary duration predicate")
    };
    assert!(matches!(predicate, BaseVerbFrame::TransitiveFrame(_)));
    assert!(
        parser
            .parse(
                "Prevent all damage that would dealt to you this turn.",
                &context,
            )
            .is_err(),
    );
}

#[test]
fn subject_gap_relatives_relay_nominal_agreement_into_finite_predicates() {
    let parser = parser();
    let context = context();

    for text in [
        "Destroy a creature that attacks.",
        "Destroy a creature that would attack.",
        "Destroy all permanents that are legendary.",
    ] {
        assert_selected(&parser, &context, text);
    }
    for text in [
        "Destroy a creature that attack.",
        "Destroy all permanents that is legendary.",
    ] {
        assert!(parser.parse(text, &context).is_err(), "reject {text:?}");
    }
}

#[test]
fn determinative_partitives_take_ordinary_reference_phrase_complements() {
    let parser = parser();
    let context = context();

    for text in [
        "Destroy each of up to two target creatures.",
        "Destroy any of them.",
        "Destroy any of up to two target creatures.",
        "Put one of those cards into your hand.",
        "Put two of them into your hand.",
        "Put two of those into your hand.",
        "Put the rest on the bottom of your library.",
    ] {
        assert_selected(&parser, &context, text);
    }
    let partitive_head = |text| {
        let predicate = imperative_transitive(&parser, &context, text);
        let Object::ObjectNominal(nominal) = predicate.object else {
            panic!("partitive probe keeps its nominal object: {text:?}")
        };
        let NounPhrase::DeterminativePartitive(partitive) = nominal.value.as_ref() else {
            panic!("partitive probe keeps its fused-head construction: {text:?}")
        };
        partitive.head.clone()
    };
    assert!(matches!(
        partitive_head("Destroy each of up to two target creatures."),
        Determinative::SingularSimpleDeterminative(_)
    ));
    for text in [
        "Destroy any of them.",
        "Destroy any of up to two target creatures.",
    ] {
        assert!(matches!(
            partitive_head(text),
            Determinative::PluralSimpleDeterminative(_)
        ));
    }
    assert!(
        parser
            .parse("Put one them into your hand.", &context)
            .is_err(),
    );
    for invalid in [
        "Put a of them into your hand.",
        "Put the of them into your hand.",
        "Put target of them into your hand.",
    ] {
        assert!(
            parser.parse(invalid, &context).is_err(),
            "reject {invalid:?}"
        );
    }
}

#[test]
fn postposed_for_as_long_as_modifies_complete_clauses() {
    let parser = parser();
    let context = context();

    assert_selected_with_specificity(
        &parser,
        &context,
        "This creature is legendary for as long as you control a Forest.",
        true,
    );
    assert!(
        parser
            .parse(
                "This creature is legendary for as long you control a Forest.",
                &context,
            )
            .is_err(),
    );
}

#[test]
fn attributive_adjectives_modify_nominal_heads() {
    let parser = parser();
    let context = context();

    for text in [
        "Draw two additional cards.",
        "Destroy target face-down creature.",
    ] {
        assert_selected(&parser, &context, text);
    }
    assert!(
        parser
            .parse("Draw two additionals cards.", &context)
            .is_err(),
    );
}

#[test]
fn contracted_perfect_object_gap_relatives_use_participles() {
    let parser = parser();
    let context = context();

    assert_selected(&parser, &context, "Exile a card you've drawn.");
    assert!(parser.parse("Exile a card you've draw.", &context).is_err(),);
}

#[test]
fn contracted_perfect_transitive_clauses_keep_their_object() {
    let parser = parser();
    let context = context();

    assert_selected(&parser, &context, "You've drawn a card.");
    assert_selected_with_specificity(
        &parser,
        &context,
        "As long as you've drawn a card, draw a card.",
        true,
    );
    assert!(parser.parse("You've draw a card.", &context).is_err(),);
}

#[test]
fn contracted_perfect_object_on_clauses_keep_their_selected_preposition() {
    let parser = parser();
    let context = context();

    assert_selected(&parser, &context, "You've put a counter on this creature.");
    assert_selected_with_specificity(
        &parser,
        &context,
        "As long as you've put one or more +1/+1 counters on this creature, draw a card.",
        true,
    );
    assert!(
        parser
            .parse("You've put a counter this creature.", &context)
            .is_err(),
    );
}

#[test]
fn contracted_perfect_passive_clauses_accept_participles_and_temporal_adjuncts() {
    let parser = parser();
    let context = context();

    assert_selected_with_specificity(
        &parser,
        &context,
        "If you've been attacked this step, draw a card.",
        true,
    );
    assert!(
        parser
            .parse("If you've been attack this step, draw a card.", &context)
            .is_err(),
    );
}

#[test]
fn existential_clauses_agree_with_their_postverbal_pivot() {
    let parser = parser();
    let context = context();

    assert_selected(&parser, &context, "There is an additional card.");
    assert_selected_with_specificity(&parser, &context, "If there is a card, draw a card.", true);
    assert!(
        parser
            .parse("There are an additional card.", &context)
            .is_err(),
    );
}

#[test]
fn intervening_existentials_use_the_same_general_finite_clause() {
    let parser = parser();
    let context = context();
    let text = "Whenever this creature attacks, if there is a card among cards, draw a card.";

    assert_selected_with_specificity(&parser, &context, text, true);
    let analysis = parser.analyze(text, &context);
    let decision = analysis
        .decision()
        .expect("selected witness has a decision");
    let ordinal = decision
        .selected()
        .expect("selected witness has an ordinal");
    let path = decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == ordinal)
        .expect("selected ordinal names a candidate")
        .construction_path();
    assert!(
        path.iter()
            .any(|name| name == "FiniteClauseExistentialFiniteClause"),
        "{path:?}",
    );
    assert!(
        path.iter()
            .all(|name| name != "ExistentialClauseSingularExistentialClause"),
        "{path:?}",
    );
}

#[test]
fn negative_nominal_quantifiers_preserve_singular_and_plural_agreement() {
    let parser = parser();
    let context = context();

    for text in [
        "There is no card.",
        "There are no cards.",
        "There are no more counters.",
    ] {
        assert_selected(&parser, &context, text);
    }
    for text in ["There are no card.", "There is no cards."] {
        assert!(parser.parse(text, &context).is_err(), "{text:?}");
    }
}

#[test]
fn do_proforms_reuse_ordinary_conditional_and_trigger_structure() {
    let parser = parser();
    let context = context();

    for text in ["If you do, draw a card.", "When you do, draw a card."] {
        assert_selected_with_specificity(&parser, &context, text, true);
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected witness has a decision");
        let ordinal = decision
            .selected()
            .expect("selected witness has an ordinal");
        let path = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("selected ordinal names a candidate")
            .construction_path();
        assert!(
            path.iter().any(|name| name == "VerbPhraseProVerbPredicate"),
            "{text:?}: {path:?}",
        );
        assert!(
            path.iter().all(|name| !name.contains("Reflexive")),
            "{text:?}: {path:?}",
        );
    }
}

#[test]
fn infinitival_negation_is_separate_from_the_invariant_to_marker() {
    let parser = parser();
    let context = context();
    let text = "Choose not to draw a card.";

    assert_selected_with_specificity(&parser, &context, text, true);
    let trace = exact_claim_trace(&parser, &context, text);
    assert!(
        trace
            .iter()
            .all(|(_, owner)| !owner.contains("InfinitiveMarker")),
        "{trace:?}",
    );
    assert!(
        trace.iter().any(|(surface, _)| surface == " not"),
        "{trace:?}"
    );
    assert!(
        trace.iter().any(|(surface, _)| surface == " to"),
        "{trace:?}"
    );
}

#[test]
fn choice_pps_and_random_manner_are_not_stored_inside_objects() {
    let parser = parser();
    let context = context();

    for text in [
        "Sacrifice a creature of their choice.",
        "Discard two cards at random.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected witness has a decision");
        let ordinal = decision
            .selected()
            .expect("selected witness has an ordinal");
        let path = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("selected ordinal names a candidate")
            .construction_path();
        assert!(
            path.iter()
                .all(|name| name != "ObjectChoiceObject" && name != "ObjectRandomObject"),
            "{text:?}: {path:?}",
        );
    }
}

#[test]
fn predicative_nominals_reuse_the_ordinary_noun_phrase_algebra() {
    let parser = parser();
    let context = context();

    for text in ["It is a creature.", "It is all colors."] {
        assert_selected_with_specificity(&parser, &context, text, true);
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected witness has a decision");
        let ordinal = decision
            .selected()
            .expect("selected witness has an ordinal");
        let path = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("selected ordinal names a candidate")
            .construction_path();
        assert!(
            path.iter()
                .any(|name| name == "PredicativeNominalComplementPredicativeNominal"),
            "{text:?}: {path:?}",
        );
        assert!(
            path.iter().all(|name| {
                name != "PredicativeTypeComplementPredicativeType"
                    && name != "PredicativeAllColorsComplementAllColors"
            }),
            "{text:?}: {path:?}",
        );
    }
}

#[test]
fn where_attachments_take_an_ordinary_finite_clause() {
    let parser = parser();
    let context = context();
    let text = "Draw X cards, where X is the number of creatures.";

    assert_selected_with_specificity(&parser, &context, text, true);
    let analysis = parser.analyze(text, &context);
    let decision = analysis
        .decision()
        .expect("selected witness has a decision");
    let ordinal = decision
        .selected()
        .expect("selected witness has an ordinal");
    let path = decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == ordinal)
        .expect("selected ordinal names a candidate")
        .construction_path();
    assert!(
        path.iter()
            .any(|name| name == "FiniteClausePlainFiniteClause"),
        "{path:?}",
    );
    assert!(
        path.iter()
            .any(|name| name == "PredicativeNominalComplementPredicativeNominal"),
        "{path:?}",
    );
}

#[test]
fn bare_target_nouns_remain_deferred_outside_the_target_determiner() {
    let parser = parser();
    let context = context();

    assert!(
        parser
            .parse("This creature deals 1 damage to any target.", &context)
            .is_err(),
    );
    assert_selected(
        &parser,
        &context,
        "This creature deals 1 damage to target creature.",
    );
    let coordinated = "Destroy target permanent card or creature card.";
    assert_selected_with_specificity(&parser, &context, coordinated, true);
    let analysis = parser.analyze(coordinated, &context);
    let decision = analysis
        .decision()
        .expect("selected coordinated target-determiner witness has a decision");
    assert!(
        decision.candidates().iter().all(|candidate| candidate
            .construction_path()
            .iter()
            .all(|name| !name.contains("NonTargetCommon"))),
        "{decision:?}",
    );
}

#[test]
fn have_takes_ordinary_nominal_and_locative_complements() {
    let parser = parser();
    let context = context();

    for text in [
        "You have no maximum hand size.",
        "You have seven cards in hand.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected witness has a decision");
        let ordinal = decision
            .selected()
            .expect("selected witness has an ordinal");
        let path = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("selected ordinal names a candidate")
            .construction_path();
        assert!(
            path.iter().all(|name| {
                name != "VerbPhraseHaveNoMaximumHandSize" && name != "VerbPhraseHaveCardsInHand"
            }),
            "{text:?}: {path:?}",
        );
    }
}

#[test]
fn nominal_modifier_stacks_attach_to_an_independent_head() {
    let parser = parser();
    let context = context();

    assert_selected(&parser, &context, "There is an additional combat phase.");
    assert_selected(&parser, &context, "There is a maximum hand size.");
}

#[test]
fn preposed_temporal_prepositions_take_ordinary_nominal_complements() {
    let parser = parser();
    let context = context();

    assert_selected(
        &parser,
        &context,
        "After this phase, there is an additional combat phase.",
    );
}

#[test]
fn finite_passives_states_and_negative_auxiliaries_compose() {
    let parser = parser();
    let context = context();

    for text in [
        "This spell was cast from a graveyard.",
        "This card was put into a graveyard from the battlefield.",
        "This card was put into a library from anywhere.",
        "Whenever one or more cards are put into a graveyard from anywhere, draw a card.",
        "Whenever one or more cards are put into a graveyard from anywhere, put a +1/+1 counter on this creature.",
        "Put target card from a graveyard on the bottom of its owner's library.",
        "{3}: Put target card from a graveyard on the bottom of its owner's library.",
        "If this spell was cast from a graveyard, draw a card.",
        "This creature is equipped.",
        "This creature isn't blocked.",
        "Creatures aren't blocked.",
        "Whenever this creature attacks and isn't blocked, draw a card.",
        "As long as this creature is equipped, it untaps.",
        "That creature doesn't untap.",
        "They don't untap.",
        "Creatures don't untap.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    let two_plain_blocks = "This card was put into a graveyard from the battlefield.\nPut target card from a graveyard on the bottom of its owner's library.";
    let document = parser.analyze_oracle_text(two_plain_blocks, &context);
    let selected = document
        .selected()
        .expect("two plain abilities separated by LF must select as one document");
    assert_eq!(selected.blocks.len(), 2);
    assert_eq!(
        selected.render(&context, parser.environment()),
        two_plain_blocks
    );
    assert!(
        document
            .ownership()
            .expect("selected document has ownership")
            .summary()
            .covered()
    );

    let mixed_blocks = "Whenever one or more cards are put into a graveyard from anywhere, put a +1/+1 counter on this creature.\n{3}: Put target card from a graveyard on the bottom of its owner's library.";
    let document = parser.analyze_oracle_text(mixed_blocks, &context);
    let selected = document
        .selected()
        .expect("triggered and activated abilities separated by LF must select");
    assert_eq!(selected.blocks.len(), 2);
    assert_eq!(
        selected.render(&context, parser.environment()),
        mixed_blocks
    );
    assert!(
        document
            .ownership()
            .expect("selected mixed document has ownership")
            .summary()
            .covered()
    );

    for malformed in [
        "This spell was cast a graveyard.",
        "This creature is equip.",
        "Creatures isn't blocked.",
        "This creature aren't blocked.",
        "Whenever they attack and isn't blocked, draw a card.",
        "As long as this creature equipped, it untaps.",
        "They doesn't untap.",
        "That creature don't untap.",
        "That creature doesn't untaps.",
        "That creature doesn't untap its controller's next untap step.",
    ] {
        assert!(
            parser.parse(malformed, &context).is_err(),
            "malformed finite passive or negative auxiliary must reject {malformed:?}",
        );
    }
}

#[test]
fn distributed_quantifiers_and_declared_participle_modifiers_compose() {
    let parser = parser();
    let context = context();

    for text in [
        "Destroy each of X target creatures.",
        "Destroy target activated ability.",
        "Discard up to one card.",
        "Discard up to two cards.",
        "You may choose not to untap this creature during your untap step.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    for malformed in [
        "Destroy each X target creatures.",
        "Destroy each of X target creature.",
        "Destroy target activate ability.",
        "Discard up to one cards.",
        "Discard up to two card.",
        "You may choose not untap this creature during your untap step.",
        "You may choose not to untap this creature your untap step.",
    ] {
        let result = parser.parse(malformed, &context);
        assert!(
            result.is_err(),
            "malformed distributed nominal must reject {malformed:?}: {result:#?}",
        );
    }
}

#[test]
fn both_quantifier_selects_a_plural_nominal() {
    let parser = parser();
    let context = context();

    assert_selected(&parser, &context, "Destroy both creatures.");
    assert!(parser.parse("Destroy both creature.", &context).is_err());
}

#[test]
fn genitive_determiner_phrases_possess_ordinary_nominals() {
    let parser = parser();
    let context = context();

    for text in [
        "Destroy target opponent's creature.",
        "Destroy target opponent's creatures.",
    ] {
        assert_selected(&parser, &context, text);
    }
    assert!(
        parser
            .parse("Destroy target opponents creature.", &context)
            .is_err()
    );
}

#[test]
fn during_phrases_attach_to_predicates_with_ordinary_nominal_complements() {
    let parser = parser();
    let context = context();

    assert_selected_with_specificity(&parser, &context, "During your turn, draw a card.", true);
    assert_selected(&parser, &context, "You gain 1 life during your turn.");
    assert!(parser.parse("During your, draw a card.", &context).is_err());
}

#[test]
fn contracted_copular_subjects_take_plain_or_negated_complements() {
    let parser = parser();
    let context = context();

    for text in [
        "If it's not your turn, draw a card.",
        "If it's your turn, draw a card.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
    assert!(
        parser
            .parse("If it's your not turn, draw a card.", &context)
            .is_err()
    );
}

#[test]
fn opaque_self_names_form_ordinary_genitive_noun_phrases() {
    let parser = parser();
    let context = context();

    assert_selected_with_specificity(&parser, &context, "Context Card's color is red.", true);
    assert_selected_with_specificity(
        &parser,
        &context,
        "Context Card's power and toughness are 1/1.",
        true,
    );
    assert!(
        parser
            .parse("Context Card color is red.", &context)
            .is_err()
    );
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

    assert_frame!(
        "Deal X damage to target creature.",
        VerbPhrase::DealAmountDamage(_)
    );
    assert_frame!(
        "Deal damage equal to its power to target creature.",
        VerbPhrase::DealDamageEqualTo(_)
    );
    assert_frame!("Gain that much life.", VerbPhrase::LifeAmount(_));
    assert_frame!("Gain life equal to its power.", VerbPhrase::LifeEquality(_));
    assert_frame!("Lose 2 life.", VerbPhrase::LifeAmount(_));
    assert_frame!(
        "Lose life equal to its toughness.",
        VerbPhrase::LifeEquality(_)
    );
    assert_frame!("Pay X life.", VerbPhrase::LifeAmount(_));
    assert_frame!("Pay {2}{B}.", VerbPhrase::ManaPhrase(_));
    assert_frame!("Add {B}{B}{B}.", VerbPhrase::ManaPhrase(_));
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
                    ..
                })
            ),
        ),
        (
            "Draw two cards.",
            matches!(
                imperative_atomic(&parser, &context, "Draw two cards."),
                VerbPhrase::DrawCards(DrawCards {
                    cards: CardQuantity::FixedCardQuantity(_),
                    ..
                })
            ),
        ),
        (
            "Draw X cards.",
            matches!(
                imperative_atomic(&parser, &context, "Draw X cards."),
                VerbPhrase::DrawCards(DrawCards {
                    cards: CardQuantity::VariableCardQuantity(_),
                    ..
                })
            ),
        ),
        (
            "Draw that many cards.",
            matches!(
                imperative_atomic(&parser, &context, "Draw that many cards."),
                VerbPhrase::DrawCards(DrawCards {
                    cards: CardQuantity::AnaphoricCardQuantity(_),
                    ..
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
        let VerbPhrase::RollDice(RollDice { dice, .. }) =
            imperative_atomic(&parser, &context, text)
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

    let VerbPhrase::ManaPhrase(ManaVerbPhrase { mana, .. }) =
        imperative_atomic(&parser, &context, "Pay {2}{B}.")
    else {
        unreachable!()
    };
    let ManaPhrase::Amount(mana) = *mana else {
        panic!("plain mana payment retains an uncoordinated amount")
    };
    let ManaAmount::ManaAmount(mana) = *mana;
    assert!(matches!(mana.run(), ActivationCostComponent::SymbolRun(_)));

    let TransitivePredicate { object, .. } =
        imperative_transitive(&parser, &context, "Sacrifice a creature of their choice.");
    assert!(matches!(object, Object::ObjectNominal(_)));

    assert_selected_with_specificity(&parser, &context, "Discard two cards at random.", true);
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
        visit_deal_amount_damage,
        DealAmountDamage,
        walk_deal_amount_damage,
        "DealAmountDamage"
    );
    trace_product!(
        visit_deal_damage_equal_to,
        DealDamageEqualTo,
        walk_deal_damage_equal_to,
        "DealDamageEqualTo"
    );
    trace_product!(
        visit_life_amount,
        LifeAmount,
        walk_life_amount,
        "LifeAmount"
    );
    trace_product!(
        visit_life_equality,
        LifeEquality,
        walk_life_equality,
        "LifeEquality"
    );
    trace_product!(
        visit_mana_verb_phrase,
        ManaVerbPhrase,
        walk_mana_verb_phrase,
        "ManaVerbPhrase"
    );
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
    fn visit_verb_inventory(&mut self, verb: &VerbInventoryRef) {
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
        visit_transitive_predicate,
        TransitivePredicate,
        walk_transitive_predicate,
        "TransitivePredicate"
    );
    trace_product!(
        visit_in_bare_locative,
        InBareLocative,
        walk_in_bare_locative,
        "InBareLocative"
    );
    trace_product!(
        visit_compared_card_quantity,
        ComparedCardQuantity,
        walk_compared_card_quantity,
        "ComparedCardQuantity"
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
        visit_owner_possessed_reference,
        OwnerPossessedReference,
        walk_owner_possessed_reference,
        "OwnerPossessedReference"
    );
    trace_product!(
        visit_singular_partitive_selection,
        SingularPartitiveSelection,
        walk_singular_partitive_selection,
        "SingularPartitiveSelection"
    );
    trace_product!(
        visit_fixed_partitive_selection,
        FixedPartitiveSelection,
        walk_fixed_partitive_selection,
        "FixedPartitiveSelection"
    );
    trace_product!(
        visit_positional_partitive,
        PositionalPartitive,
        walk_positional_partitive,
        "PositionalPartitive"
    );
    trace_product!(
        visit_from_phrase_value,
        FromPhraseValue,
        walk_from_phrase_value,
        "FromPhraseValue"
    );
    trace_product!(
        visit_into_phrase_value,
        IntoPhraseValue,
        walk_into_phrase_value,
        "IntoPhraseValue"
    );
    trace_product!(
        visit_onto_phrase_value,
        OntoPhraseValue,
        walk_onto_phrase_value,
        "OntoPhraseValue"
    );
    trace_product!(
        visit_on_phrase_value,
        OnPhraseValue,
        walk_on_phrase_value,
        "OnPhraseValue"
    );
    trace_product!(
        visit_on_edge_phrase,
        OnEdgePhrase,
        walk_on_edge_phrase,
        "OnEdgePhrase"
    );
    trace_product!(
        visit_edge_of_phrase_value,
        EdgeOfPhraseValue,
        walk_edge_of_phrase_value,
        "EdgeOfPhraseValue"
    );
    trace_product!(
        visit_to_phrase_value,
        ToPhraseValue,
        walk_to_phrase_value,
        "ToPhraseValue"
    );
    trace_product!(
        visit_predicative_status_value,
        PredicativeStatusValue,
        walk_predicative_status_value,
        "PredicativeStatusValue"
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
    trace_product!(visit_put_into, PutInto, walk_put_into, "PutInto");
    trace_product!(visit_put_onto, PutOnto, walk_put_onto, "PutOnto");
    trace_product!(visit_put_on, PutOn, walk_put_on, "PutOn");
    trace_product!(visit_put_to, PutTo, walk_put_to, "PutTo");
    trace_product!(visit_return_to, ReturnTo, walk_return_to, "ReturnTo");
    trace_product!(
        visit_enter_resultative,
        EnterResultative,
        walk_enter_resultative,
        "EnterResultative"
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
    trace_product!(visit_look_at, LookAt, walk_look_at, "LookAt");
    trace_product!(
        visit_declared_object_for_object_frame,
        DeclaredObjectForObjectFrame,
        walk_declared_object_for_object_frame,
        "DeclaredObjectForObjectFrame"
    );
    trace_product!(visit_have_life, HaveLife, walk_have_life, "HaveLife");
    trace_product!(
        visit_have_object_control,
        HaveObjectControl,
        walk_have_object_control,
        "HaveObjectControl"
    );

    fn visit_verb_inventory(&mut self, verb: &VerbInventoryRef) {
        match verb {
            VerbInventoryRef::Core(CoreVerbIdentity::Enter) => {
                self.0.push("verb:Core(Enter)".to_owned());
            }
            VerbInventoryRef::Core(CoreVerbIdentity::Leave) => {
                self.0.push("verb:Core(Leave)".to_owned());
            }
            VerbInventoryRef::Core(CoreVerbIdentity::Look) => {
                self.0.push("verb:Core(Look)".to_owned());
            }
            _ => {}
        }
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
            "product:ManaVerbPhrase",
            "verb:Core(Pay)",
            "scalar:2",
            "product:DrawCardsEqualTo",
            "verb:Core(Draw)",
            "possessive:Its",
            "characteristic:Toughness",
            "product:PutCounters",
            "verb:Core(Put)",
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
        "Deal X damage to target creature.",
        [
            "product:DealAmountDamage",
            "verb:Core(Deal)",
            "variable:X",
            "declared:Creature"
        ],
        [
            ("Deal", "core-verb:Deal"),
            (" X", "vocab:Variable/X"),
            (" damage", "form:deal_amount_damage/deal_amount_damage/2"),
            (" to", "form:to_phrase/to_phrase/0"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Deal damage equal to its power to target creature.",
        [
            "product:DealDamageEqualTo",
            "verb:Core(Deal)",
            "possessive:Its",
            "characteristic:Power",
            "declared:Creature"
        ],
        [
            ("Deal", "core-verb:Deal"),
            (
                " damage",
                "form:deal_damage_equal_to/deal_damage_equal_to/1"
            ),
            (" equal", "form:scalar_equality/scalar_equality/0"),
            (" to", "form:scalar_equality/scalar_equality/1"),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (" power", "vocab:ScalarCharacteristic/Power"),
            (" to", "form:to_phrase/to_phrase/0"),
            (
                " target",
                "form:target_determiner_phrase/target_determiner_phrase/0"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Gain that much life.",
        ["product:LifeAmount", "verb:Core(Gain)"],
        [
            ("Gain", "core-verb:Gain"),
            (" that", "form:that_much/that_much/0"),
            (" much", "form:that_much/that_much/1"),
            (" life", "form:life_amount/life_amount/2"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Gain life equal to its power.",
        [
            "product:LifeEquality",
            "verb:Core(Gain)",
            "possessive:Its",
            "characteristic:Power"
        ],
        [
            ("Gain", "core-verb:Gain"),
            (" life", "form:life_equality/life_equality/1"),
            (" equal", "form:scalar_equality/scalar_equality/0"),
            (" to", "form:scalar_equality/scalar_equality/1"),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (" power", "vocab:ScalarCharacteristic/Power"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Lose 2 life.",
        ["product:LifeAmount", "verb:Core(Lose)", "scalar:2"],
        [
            ("Lose", "core-verb:Lose"),
            (" 2", "codec:ScalarNumber"),
            (" life", "form:life_amount/life_amount/2"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Lose life equal to its toughness.",
        [
            "product:LifeEquality",
            "verb:Core(Lose)",
            "possessive:Its",
            "characteristic:Toughness"
        ],
        [
            ("Lose", "core-verb:Lose"),
            (" life", "form:life_equality/life_equality/1"),
            (" equal", "form:scalar_equality/scalar_equality/0"),
            (" to", "form:scalar_equality/scalar_equality/1"),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (" toughness", "vocab:ScalarCharacteristic/Toughness"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Pay X life.",
        ["product:LifeAmount", "verb:Core(Pay)", "variable:X"],
        [
            ("Pay", "core-verb:Pay"),
            (" X", "vocab:Variable/X"),
            (" life", "form:life_amount/life_amount/2"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Pay {2}{B}.",
        [
            "product:ManaVerbPhrase",
            "verb:Core(Pay)",
            "scalar:2",
            "symbol:Black"
        ],
        [
            ("Pay", "core-verb:Pay"),
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
        ["product:ManaVerbPhrase", "verb:Core(Add)", "symbol:Black"],
        [
            ("Add", "core-verb:Add"),
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
            "verb:Core(Draw)",
            "product:SingularCardQuantity"
        ],
        [
            ("Draw", "core-verb:Draw"),
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
            "verb:Core(Draw)",
            "product:FixedCardQuantity",
            "cardinal:2"
        ],
        [
            ("Draw", "core-verb:Draw"),
            (" two", "codec:CardinalNumber"),
            (" cards", "form:fixed_card_quantity/fixed_card_quantity/1"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Draw X cards.",
        [
            "product:DrawCards",
            "verb:Core(Draw)",
            "product:VariableCardQuantity",
            "variable:X"
        ],
        [
            ("Draw", "core-verb:Draw"),
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
            "verb:Core(Draw)",
            "product:AnaphoricCardQuantity"
        ],
        [
            ("Draw", "core-verb:Draw"),
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
            "verb:Core(Draw)",
            "possessive:Its",
            "characteristic:Toughness"
        ],
        [
            ("Draw", "core-verb:Draw"),
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
            "verb:Core(Roll)",
            "product:SingularDieObject",
            "die:SixSided"
        ],
        [
            ("Roll", "core-verb:Roll"),
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
            "verb:Core(Roll)",
            "product:FixedDiceObject",
            "cardinal:2",
            "die:SixSided"
        ],
        [
            ("Roll", "core-verb:Roll"),
            (" two", "codec:CardinalNumber"),
            (" six-sided", "vocab:DieShape/SixSided"),
            (" dice", "form:fixed_dice_object/fixed_dice_object/2"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Roll a d20.",
        ["product:RollDice", "verb:Core(Roll)", "product:D20Object"],
        [
            ("Roll", "core-verb:Roll"),
            (" a", "form:d20_object/d20_object/0"),
            (" d20", "form:d20_object/d20_object/1"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Put a +1/+1 counter on target creature.",
        [
            "product:PutCounters",
            "verb:Core(Put)",
            "product:SingularCounterQuantity",
            "product:PositivePowerToughnessCounter",
            "scalar:1",
            "scalar:1",
            "declared:Creature",
        ],
        [
            ("Put", "core-verb:Put"),
            (" a", "form:singular_counter_quantity/a/0"),
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
            (" counter", "form:singular_counter_quantity/a/2"),
            (" on", "form:on_phrase/on_phrase/0"),
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
            "verb:Core(Put)",
            "product:SingularCounterQuantity",
            "product:NegativePowerToughnessCounter",
            "scalar:1",
            "scalar:1",
            "declared:Creature",
        ],
        [
            ("Put", "core-verb:Put"),
            (" a", "form:singular_counter_quantity/a/0"),
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
            (" counter", "form:singular_counter_quantity/a/2"),
            (" on", "form:on_phrase/on_phrase/0"),
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
            "verb:Core(Put)",
            "product:SingularCounterQuantity",
            "counter:Time",
            "declared:Creature"
        ],
        [
            ("Put", "core-verb:Put"),
            (" a", "form:singular_counter_quantity/a/0"),
            (" time", "vocab:CounterName/Time"),
            (" counter", "form:singular_counter_quantity/a/2"),
            (" on", "form:on_phrase/on_phrase/0"),
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
            "verb:Core(Put)",
            "product:FixedCounterQuantity",
            "cardinal:2",
            "counter:Stun"
        ],
        [
            ("Put", "core-verb:Put"),
            (" two", "codec:CardinalNumber"),
            (" stun", "vocab:CounterName/Stun"),
            (
                " counters",
                "form:fixed_counter_quantity/fixed_counter_quantity/2"
            ),
            (" on", "form:on_phrase/on_phrase/0"),
            (" it", "vocab:ObjectPronoun/It"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Put X time counters on target creature.",
        [
            "product:PutCounters",
            "verb:Core(Put)",
            "product:VariableCounterQuantity",
            "variable:X",
            "counter:Time",
            "declared:Creature"
        ],
        [
            ("Put", "core-verb:Put"),
            (" X", "vocab:Variable/X"),
            (" time", "vocab:CounterName/Time"),
            (
                " counters",
                "form:variable_counter_quantity/variable_counter_quantity/2"
            ),
            (" on", "form:on_phrase/on_phrase/0"),
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
            "verb:Core(Put)",
            "product:AnaphoricCounterQuantity",
            "counter:Charge",
            "declared:Creature"
        ],
        [
            ("Put", "core-verb:Put"),
            (" that", "form:that_many/that_many/0"),
            (" many", "form:that_many/that_many/1"),
            (" charge", "vocab:CounterName/Charge"),
            (
                " counters",
                "form:anaphoric_counter_quantity/anaphoric_counter_quantity/2"
            ),
            (" on", "form:on_phrase/on_phrase/0"),
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
            "verb:Core(Remove)",
            "product:VariableCounterQuantity",
            "variable:X",
            "counter:Time",
            "noun:Card"
        ],
        [
            ("Remove", "core-verb:Remove"),
            (" X", "vocab:Variable/X"),
            (" time", "vocab:CounterName/Time"),
            (
                " counters",
                "form:variable_counter_quantity/variable_counter_quantity/2"
            ),
            (" from", "form:from_phrase/from_phrase/0"),
            (
                " this",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Each player sacrifices a creature of their choice.",
        [
            "noun:Player",
            "declared:Sacrifice",
            "declared:Creature",
            "possessive:Their",
            "noun:Choice"
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
            (" of", "form:of_phrase/of_phrase/0"),
            (" their", "vocab:PossessiveDeterminerPronoun/Their"),
            (" choice", "lexeme:CommonNoun/Choice/singular"),
            (".", TERMINATOR),
        ]
    );
    assert_family!(
        "Target player discards two cards at random.",
        ["noun:Player", "declared:Discard", "cardinal:2", "noun:Card"],
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
            (" at", "form:at_random_manner/at_random_manner/0"),
            (" random", "form:at_random_manner/at_random_manner/1"),
            (".", TERMINATOR),
        ]
    );
}

#[test]
fn target_and_card_name_boundaries_remain_grammatical_and_metadata_governed() {
    let parser = parser();
    let context = context();
    assert_selected_with_specificity(
        &parser,
        &context,
        "Put two stun counters on two target creatures.",
        true,
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
    assert_selected(
        &parser,
        &plus_two,
        "+2 Mace deals 2 damage to target creature.",
    );
    assert!(
        parser
            .parse("Mace deals 2 damage to target creature.", &plus_two)
            .is_err(),
        "punctuation does not infer a shortened opaque card name",
    );

    let ordinary = ParseContext::new("Ordinary, Context", false, Onset::Vowel)
        .expect("ordinary punctuated card name is valid");
    assert_selected(
        &parser,
        &ordinary,
        "Ordinary, Context deals 2 damage to target creature.",
    );
    assert!(
        parser
            .parse("Ordinary deals 2 damage to target creature.", &ordinary)
            .is_err(),
        "a comma does not authorize ordinary-name abbreviation",
    );

    let legendary = ParseContext::new("Zoraline, Cosmos Caller", true, Onset::Consonant)
        .expect("legendary punctuated card name is valid");
    for text in [
        "Zoraline, Cosmos Caller deals 2 damage to target creature.",
        "Zoraline deals 2 damage to target creature.",
    ] {
        assert_selected(&parser, &legendary, text);
    }
    assert_selected_with_specificity(
        &parser,
        &legendary,
        "When Zoraline enters, draw a card.",
        true,
    );
    assert_selected_with_specificity(
        &parser,
        &context,
        "Target creature you control deals damage equal to its power to another target creature, artifact, or planeswalker.",
        true,
    );
    assert!(
        parser
            .parse("When Zoraline enter, draw a card.", &legendary)
            .is_err(),
        "singular shortened self-reference requires singular verb agreement",
    );
    assert!(
        parser
            .parse(
                "Target creature you control deals damage equal to its power to another target creature, artifacts, or planeswalker.",
                &context,
            )
            .is_err(),
        "a determiner-scoped singular coordination rejects a plural conjunct",
    );
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
        ("Put that card into your hand.", true),
        ("Put that card to your hand.", true),
        (
            "Put target creature card from your graveyard onto the battlefield tapped under your control.",
            true,
        ),
        ("Put target creature on top of its owner's library.", true),
        ("Return target creature to its owner's hand.", true),
        (
            "Return target creature card from your graveyard to the battlefield tapped under its owner's control.",
            true,
        ),
        ("This creature enters tapped.", false),
        (
            "This creature enters the battlefield under your control.",
            true,
        ),
        ("This creature enters under your control.", false),
        ("This creature leaves the battlefield.", true),
        ("One or more cards leave your graveyard.", true),
    ] {
        assert_selected_with_specificity(&parser, &context, text, permits_specificity);
    }

    // The parser owns grammatical form, not zone or controller legality.
    for text in [
        "Return target spell to the battlefield under your control.",
        "Put target player onto the battlefield tapped under their control.",
        "Put target player into a coin.",
        "Put that card onto your hand.",
        "Put target player onto a coin tapped under their control.",
        "Put that card on your hand.",
        "Put target creature on top of a coin.",
        "Return target spell to a coin under your control.",
        "This creature enters a coin.",
        "This creature leaves a coin.",
        "This creature enters legendary.",
        "Search a creature card for your library.",
    ] {
        assert_selected_with_specificity(
            &parser,
            &context,
            text,
            text.starts_with("Put ")
                || text.starts_with("Return ")
                || text.starts_with("This creature leaves ")
                || matches!(
                    text,
                    "This creature enters a coin." | "Search a creature card for your library."
                ),
        );
    }
}

#[test]
fn location_state_and_object_control_frames_select_exact_products() {
    let parser = parser();
    let context = context();
    for (text, permits_specificity) in [
        ("Search your library for a creature card.", true),
        ("Look at the top two cards of your library.", false),
        ("Look at the top two cards of a coin.", false),
        ("Reveal the top card of your library.", false),
        ("Reveal the top card of a coin.", false),
        ("Each player reveals their hand.", false),
        ("You have two or fewer cards in hand.", true),
        ("You have three or fewer cards in hand.", true),
        ("You have 10 or less life.", false),
        ("You have no maximum hand size.", true),
        ("Have her deal 2 damage to you.", false),
    ] {
        assert_selected_with_specificity(&parser, &context, text, permits_specificity);
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
            head,
            source: None,
            destination: IntoPhrase::IntoPhrase(_),
            ..
        }) if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Put))
    ));
    assert!(matches!(
        imperative_atomic(
            &parser,
            &context,
            "Put target creature card from your graveyard onto the battlefield tapped under your control."
        ),
        VerbPhrase::PutOnto(PutOnto {
            head,
            source: Some(FromPhrase::FromPhrase(_)),
            destination: OntoPhrase::OntoPhrase(_),
            result,
            control: Some(ControlPostmodifier::DirectControlPostmodifier(_)),
            ..
        }) if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Put))
            && matches!(result.as_ref(), Some(PredicativeComplement::Status(_)))
    ));
    assert!(matches!(
        imperative_atomic(
            &parser,
            &context,
            "Put target creature on top of its owner's library."
        ),
        VerbPhrase::PutOn(PutOn {
            head,
            destination: OnPhrase::OnEdgePhrase(OnEdgePhrase {
                complement,
            }),
            ..
        }) if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Put))
            && matches!(
                complement.as_ref(),
                EdgeOfPhrase::EdgeOfPhrase(EdgeOfPhraseValue { whole, .. })
                    if matches!(whole.as_ref(), Object::ObjectNominal(_))
            )
    ));
    assert!(matches!(
        imperative_atomic(&parser, &context, "Put that card to your hand."),
        VerbPhrase::PutTo(PutTo {
            head,
            object: Object::ObjectNominal(_),
            destination: ToPhrase::ToPhrase(ToPhraseValue {
                complement: Object::ObjectNominal(_),
            }),
        }) if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Put))
    ));
    assert!(matches!(
        imperative_atomic(
            &parser,
            &context,
            "Return target creature card from your graveyard to the battlefield tapped under its owner's control."
        ),
        VerbPhrase::ReturnTo(ReturnTo {
            head,
            source: Some(FromPhrase::FromPhrase(_)),
            destination: ToPhrase::ToPhrase(_),
            result,
            control: Some(ControlPostmodifier::OwnerControlPostmodifier(_)),
            ..
        }) if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Return))
            && matches!(result.as_ref(), Some(PredicativeComplement::Status(_)))
    ));
    let VerbPhrase::LookAt(LookAt {
        head,
        object: Object::ObjectNominal(nominal),
    }) = imperative_atomic(
        &parser,
        &context,
        "Look at the top two cards of your library.",
    )
    else {
        panic!("look-at witness keeps its typed nominal object")
    };
    assert!(matches!(
        head.reference(),
        VerbInventoryRef::Core(CoreVerbIdentity::Look)
    ));
    let NounPhrase::PositionalPartitive(partitive) = nominal.value.as_ref() else {
        panic!("look-at witness keeps its typed positional partitive")
    };
    assert!(matches!(
        (&partitive.selection, partitive.whole.as_ref()),
        (
            PartitiveSelection::FixedPartitiveSelection(_),
            Object::ObjectNominal(_)
        )
    ));
    let VerbPhrase::BaseVerbPhrase(search) = imperative_atomic(
        &parser,
        &context,
        "Search your library for a creature card.",
    ) else {
        panic!("search uses the shared base-frame envelope")
    };
    let BaseVerbFrame::ObjectForObjectFrame(search) = search.frame else {
        panic!("search uses the object-for-object frame")
    };
    let ObjectForObjectFrame::DeclaredObjectForObjectFrame(search) = search;
    assert!(matches!(search.object, Object::ObjectNominal(_)));
    assert!(matches!(search.complement, Object::ObjectNominal(_)));
    let VerbPhrase::HaveObjectControl(control) =
        imperative_atomic(&parser, &context, "Have her deal 2 damage to you.")
    else {
        panic!("object control keeps its exact predicate product")
    };
    assert!(matches!(control.object, Object::ObjectPronoun(_)));
    assert!(matches!(
        control.predicate(),
        VerbPhrase::DealAmountDamage(DealAmountDamage {
            recipient: ToPhrase::ToPhrase(_),
            ..
        })
    ));

    assert!(matches!(
        declarative_atomic(&parser, &context, "This creature enters tapped."),
        VerbPhrase::EnterResultative(EnterResultative { head, result })
            if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Enter))
                && matches!(result.as_ref(), PredicativeComplement::Status(_))
    ));
    assert!(matches!(
        declarative_atomic(
            &parser,
            &context,
            "This creature enters with two +1/+1 counters on it."
        ),
        VerbPhrase::EnterWithCounters(EnterWithCounters { head, .. })
            if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Enter))
    ));
    assert!(matches!(
        declarative_atomic(
            &parser,
            &context,
            "This creature enters the battlefield under your control."
        ),
        VerbPhrase::EnterLocation(EnterLocation {
            head,
            location: Object::ObjectNominal(_),
            control: Some(ControlPostmodifier::DirectControlPostmodifier(_)),
            ..
        }) if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Enter))
    ));
    assert!(matches!(
        declarative_atomic(
            &parser,
            &context,
            "This creature enters under your control."
        ),
        VerbPhrase::EnterControl(EnterControl {
            head,
            control: ControlPostmodifier::DirectControlPostmodifier(_),
        }) if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Enter))
    ));
    assert!(matches!(
        declarative_atomic(&parser, &context, "This creature leaves the battlefield."),
        VerbPhrase::BaseVerbPhrase(BaseVerbPhrase {
            frame: BaseVerbFrame::TransitiveFrame(TransitiveFrame::TransitivePredicate(
                TransitivePredicate {
                    head,
                    object: Object::ObjectNominal(_),
                }
            )),
        }) if matches!(head.reference(), VerbInventoryRef::Core(CoreVerbIdentity::Leave))
    ));
    assert_selected_with_specificity(
        &parser,
        &context,
        "You have three or fewer cards in hand.",
        true,
    );
    assert!(matches!(
        declarative_atomic(&parser, &context, "You have 10 or less life."),
        VerbPhrase::HaveLife(HaveLife {
            comparison: ScalarComparison::ScalarOrLess(_),
        })
    ));
    assert_selected_with_specificity(&parser, &context, "You have no maximum hand size.", true);
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
        "Search your library a creature card.",
        "Remove a time counter to this card.",
        "This creature leaves to the battlefield.",
        "This creature enters from your graveyard.",
        "Look into the top two cards of your library.",
        "Reveal at the top card of your library.",
        "You have one or fewer card in hand.",
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
        ($text:literal, $specificity:literal, [$($visit:literal),+ $(,)?], [$(($surface:literal, $owner:expr $(,)?)),+ $(,)?]) => {{
            let ability = assert_selected_with_specificity(&parser, &context, $text, $specificity);
            let mut visitor = MovementVisitor::default();
            visitor.visit_ability(&ability);
            assert_eq!(visitor.0, [$($visit),+], "{}", $text);
            assert_eq!(
                exact_claim_trace(&parser, &context, $text),
                [$(($surface.to_owned(), $owner.to_owned())),+],
                "{}",
                $text,
            );
        }};
    }

    assert_family!(
        "Put that card into your hand.",
        true,
        [
            "product:PutInto",
            "product:IntoPhraseValue"
        ],
        [
            ("Put", "core-verb:Put"),
            (" that", "determinative:DeterminativeHead/DistalDemonstrative"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (" into", "form:into_phrase/into_phrase/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" hand", "lexeme:CommonNoun/Hand/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Put that card to your hand.",
        true,
        ["product:PutTo", "product:ToPhraseValue"],
        [
            ("Put", "core-verb:Put"),
            (" that", "determinative:DeterminativeHead/DistalDemonstrative"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (" to", "form:to_phrase/to_phrase/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" hand", "lexeme:CommonNoun/Hand/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Put target creature card from your graveyard onto the battlefield tapped under your control.",
        true,
        [
            "product:PutOnto",
            "product:FromPhraseValue",
            "product:OntoPhraseValue",
            "product:PredicativeStatusValue",
            "product:DirectControlPostmodifier"
        ],
        [
            ("Put", "core-verb:Put"),
            (
                " target",
                "determinative:DeterminativeHead/Target"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (" from", "form:from_phrase/from_phrase/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" graveyard", "lexeme:CommonNoun/Graveyard/singular"),
            (" onto", "form:onto_phrase/onto_phrase/0"),
            (
                " the",
                "determinative:DeterminativeHead/DefiniteArticle"
            ),
            (" battlefield", "lexeme:CommonNoun/Battlefield/singular"),
            (" tapped", "vocab:Status/Tapped"),
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
        true,
        [
            "product:PutOn",
            "product:OnEdgePhrase",
            "product:EdgeOfPhraseValue"
        ],
        [
            ("Put", "core-verb:Put"),
            (
                " target",
                "determinative:DeterminativeHead/Target"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" on", "form:on_edge_phrase/on_edge_phrase/0"),
            (" top", "vocab:EdgePosition/Top"),
            (" of", "form:edge_of_phrase/top/1"),
            (" its", "vocab:PossessiveDeterminerPronoun/Its"),
            (" owner", "lexeme:CommonNoun/Owner/singular"),
            (
                "'s",
                "form:genitive_determiner_singular_reference/genitive_determiner_singular_reference/0/affix"
            ),
            (" library", "lexeme:CommonNoun/Library/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Put target creature on the bottom of their owners' library.",
        true,
        [
            "product:PutOn",
            "product:OnEdgePhrase",
            "product:EdgeOfPhraseValue",
            "product:OwnerPossessedReference",
            "product:PluralOwnerPossessor"
        ],
        [
            ("Put", "core-verb:Put"),
            (
                " target",
                "determinative:DeterminativeHead/Target"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" on", "form:on_edge_phrase/on_edge_phrase/0"),
            (" the", "form:edge_of_phrase/bottom/0"),
            (" bottom", "vocab:EdgePosition/Bottom"),
            (" of", "form:edge_of_phrase/bottom/2"),
            (" their", "vocab:PossessiveDeterminerPronoun/Their"),
            (
                " owners'",
                "form:plural_owner_possessor/plural_owner_possessor/1"
            ),
            (" library", "lexeme:CommonNoun/Library/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Return target creature card from your graveyard to the battlefield tapped under its owner's control.",
        true,
        [
            "product:ReturnTo",
            "product:FromPhraseValue",
            "product:ToPhraseValue",
            "product:PredicativeStatusValue",
            "product:OwnerControlPostmodifier",
            "product:SingularOwnerPossessor"
        ],
        [
            ("Return", "core-verb:Return"),
            (
                " target",
                "determinative:DeterminativeHead/Target"
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (" from", "form:from_phrase/from_phrase/0"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" graveyard", "lexeme:CommonNoun/Graveyard/singular"),
            (" to", "form:to_phrase/to_phrase/0"),
            (
                " the",
                "determinative:DeterminativeHead/DefiniteArticle"
            ),
            (" battlefield", "lexeme:CommonNoun/Battlefield/singular"),
            (" tapped", "vocab:Status/Tapped"),
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
        [
            "product:EnterResultative",
            "verb:Core(Enter)",
            "product:PredicativeStatusValue"
        ],
        [
            (
                "This",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" enters", "core-verb:Enter"),
            (" tapped", "vocab:Status/Tapped"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "This creature enters the battlefield under your control.",
        false,
        [
            "product:EnterLocation",
            "verb:Core(Enter)",
            "product:DirectControlPostmodifier"
        ],
        [
            (
                "This",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" enters", "core-verb:Enter"),
            (
                " the",
                "determinative:DeterminativeHead/DefiniteArticle"
            ),
            (" battlefield", "lexeme:CommonNoun/Battlefield/singular"),
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
        [
            "product:EnterControl",
            "verb:Core(Enter)",
            "product:DirectControlPostmodifier"
        ],
        [
            (
                "This",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" enters", "core-verb:Enter"),
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
        true,
        ["product:TransitivePredicate", "verb:Core(Leave)"],
        [
            (
                "This",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" creature", "lexeme:type/Creature/singular"),
            (" leaves", "core-verb:Leave"),
            (
                " the",
                "determinative:DeterminativeHead/DefiniteArticle"
            ),
            (" battlefield", "lexeme:CommonNoun/Battlefield/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Look at the top two cards of your library.",
        false,
        [
            "product:LookAt",
            "verb:Core(Look)",
            "product:PositionalPartitive",
            "product:FixedPartitiveSelection"
        ],
        [
            ("Look", "core-verb:Look"),
            (" at", "form:look_at/look_at/1"),
            (" the", "form:positional_partitive/positional_partitive/0"),
            (" top", "vocab:EdgePosition/Top"),
            (" two", "codec:CardinalNumber"),
            (" cards", "lexeme:CommonNoun/Card/plural"),
            (" of", "form:positional_partitive/positional_partitive/3"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" library", "lexeme:CommonNoun/Library/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Reveal the top card of your library.",
        false,
        [
            "product:TransitivePredicate",
            "product:PositionalPartitive",
            "product:SingularPartitiveSelection"
        ],
        [
            ("Reveal", "lexeme:keyword_action/Reveal/bare"),
            (" the", "form:positional_partitive/positional_partitive/0"),
            (" top", "vocab:EdgePosition/Top"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (" of", "form:positional_partitive/positional_partitive/3"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" library", "lexeme:CommonNoun/Library/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Search your library for a creature card.",
        true,
        ["product:DeclaredObjectForObjectFrame"],
        [
            ("Search", "lexeme:keyword_action/Search/bare"),
            (" your", "vocab:PossessiveDeterminerPronoun/Your"),
            (" library", "lexeme:CommonNoun/Library/singular"),
            (
                " for",
                "form:declared_object_for_object_frame/declared_object_for_object_frame/2"
            ),
            (" a", "determinative:DeterminativeHead/IndefiniteArticle"),
            (" creature", "lexeme:type/Creature/singular"),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "You have three or fewer cards in hand.",
        false,
        ["product:TransitivePredicate", "product:InBareLocative"],
        [
            ("You", "vocab:SubjectPronoun/You"),
            (" have", "core-verb:Have"),
            (" three", "codec:CardinalNumber"),
            (" or", "form:count_or_fewer/count_or_fewer/0"),
            (" fewer", "form:count_or_fewer/count_or_fewer/1"),
            (" cards", "lexeme:CommonNoun/Card/plural"),
            (" in", "form:in_bare_locative/in_bare_locative/0"),
            (" hand", "vocab:BareLocativeNoun/Hand"),
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
        ["product:TransitivePredicate"],
        [
            ("You", "vocab:SubjectPronoun/You"),
            (" have", "core-verb:Have"),
            (" no", "determinative:DeterminativeHead/No"),
            (" maximum", "vocab:AttributiveAdjective/Maximum"),
            (" hand", "lexeme:CommonNoun/Hand/singular"),
            (" size", "lexeme:CommonNoun/Size/singular"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Have her deal 2 damage to you.",
        false,
        ["product:HaveObjectControl", "product:ToPhraseValue"],
        [
            ("Have", "lexeme:VerbLexeme/Have/bare"),
            (" her", "vocab:ObjectPronoun/Her"),
            (" deal", "core-verb:Deal"),
            (" 2", "codec:ScalarNumber"),
            (" damage", "form:deal_amount_damage/deal_amount_damage/2"),
            (" to", "form:to_phrase/to_phrase/0"),
            (" you", "vocab:ObjectPronoun/You"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Put two stun counters on it.",
        true,
        ["product:OnPhraseValue"],
        [
            ("Put", "core-verb:Put"),
            (" two", "codec:CardinalNumber"),
            (" stun", "vocab:CounterName/Stun"),
            (
                " counters",
                "form:fixed_counter_quantity/fixed_counter_quantity/2"
            ),
            (" on", "form:on_phrase/on_phrase/0"),
            (" it", "vocab:ObjectPronoun/It"),
            (".", TERMINATOR)
        ]
    );
    assert_family!(
        "Remove X time counters from this card.",
        false,
        ["product:FromPhraseValue"],
        [
            ("Remove", "core-verb:Remove"),
            (" X", "vocab:Variable/X"),
            (" time", "vocab:CounterName/Time"),
            (
                " counters",
                "form:variable_counter_quantity/variable_counter_quantity/2"
            ),
            (" from", "form:from_phrase/from_phrase/0"),
            (
                " this",
                "determinative:DeterminativeHead/ProximalDemonstrative",
            ),
            (" card", "lexeme:CommonNoun/Card/singular"),
            (".", TERMINATOR)
        ]
    );
}

#[derive(Default)]
struct AdjunctVisitor(Vec<&'static str>);

impl Visitor for AdjunctVisitor {
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
fn infinitive_requirement_counterfactual_and_order_products_are_typed() {
    let parser = parser();
    let context = context();

    for (text, expected, permits_specificity) in [
        (
            "Whenever a spell or ability an opponent controls causes you to discard a card, you gain 3 life and you may draw two cards.",
            &["object-infinitive"][..],
            true,
        ),
        (
            "This creature attacks each combat if able.",
            &["requirement"][..],
            true,
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
        let mut visitor = AdjunctVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, expected, "exact typed products for {text:?}");
    }
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the three counterfactual finite-clause frames share one exhaustive typed audit"
)]
fn as_though_owns_the_attested_counterfactual_finite_family() {
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
        let mut visitor = AdjunctVisitor::default();
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
            (
                " can".to_owned(),
                "lexeme:VerbLexeme/Can/third_person_singular".to_owned(),
            ),
            (" block".to_owned(), "core-verb:Block".to_owned(),),
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
            (" can".to_owned(), "lexeme:VerbLexeme/Can/bare".to_owned(),),
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
            (" control".to_owned(), "core-verb:Control".to_owned(),),
            (" can".to_owned(), "lexeme:VerbLexeme/Can/bare".to_owned(),),
            (" block".to_owned(), "core-verb:Block".to_owned(),),
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
fn purpose_duration_and_instead_products_are_typed() {
    let parser = parser();
    let context = context();

    for (text, expected, permits_specificity) in [
        ("Discard a card to draw a card.", "purpose", true),
        ("Attack to draw a card.", "purpose", true),
        ("You may cast it this turn.", "duration", true),
        ("Draw a card instead.", "instead", true),
        ("Attack instead.", "instead", true),
    ] {
        let ability =
            assert_selected_with_specificity(&parser, &context, text, permits_specificity);
        let mut visitor = AdjunctVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, [expected], "exact typed product for {text:?}");
    }
}

#[test]
fn this_way_keeps_predicate_manner_scope() {
    let parser = parser();
    let context = context();
    let text = "You didn't create a token this way.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = AdjunctVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["manner"]);

    let text = "Attack this way.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = AdjunctVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["manner"]);
}

#[derive(Default)]
struct AdjunctSurfaceVisitor(Vec<String>);

impl Visitor for AdjunctSurfaceVisitor {
    fn visit_may_auxiliary(&mut self, _value: &MayAuxiliary) {
        self.0.push("auxiliary:May".to_owned());
    }

    fn visit_cant_auxiliary(&mut self, _value: &CantAuxiliary) {
        self.0.push("auxiliary:Cant".to_owned());
    }

    fn visit_duration_predicate_value(&mut self, value: &DurationPredicateValue) {
        self.0.push("duration".to_owned());
        deckmaste_english_v2::visit::walk_duration_predicate_value(self, value);
    }

    fn visit_at_random_manner(&mut self, value: &AtRandomManner) {
        self.0.push("at-random-manner".to_owned());
        deckmaste_english_v2::visit::walk_at_random_manner(self, value);
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
fn permission_restriction_exception_random_and_order_surfaces_are_scoped() {
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
            &["at-random-manner"][..],
        ),
        (
            "Put them on top of your library in a random order.",
            &["order:Random"][..],
        ),
    ] {
        let ability = assert_selected(&parser, &context, text);
        let mut visitor = AdjunctSurfaceVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, expected, "exact scoped surface for {text:?}");
    }
}

#[test]
fn attachment_movement_does_not_silently_change_scope() {
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
fn cost_position_reuses_the_typed_predicate_algebra() {
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

    let mut visitor = AdjunctVisitor::default();
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
                "form:instead_predicate/instead_predicate/1".to_owned(),
            ),
            (": ".to_owned(), "form:activated/activated/1".to_owned(),),
            ("Draw".to_owned(), "core-verb:Draw".to_owned(),),
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
fn object_internal_discarded_this_way_does_not_become_outer_manner() {
    let parser = parser();
    let context = context();
    let text = "Target player discards three cards. Put up to one artifact card discarded this way onto the battlefield tapped under your control.";
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
                span: TextSpan { start: 74, end: 78 },
                ..
            }
        ),
        "the declared participle stays object-internal and failure advances to its unsupported relative subject: {error:?}",
    );
    assert_eq!(&text[74..78], "this");
}

#[derive(Default)]
struct DistributionVisitor(Vec<&'static str>);

impl Visitor for DistributionVisitor {
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

    fn visit_put_counters(&mut self, value: &PutCounters) {
        self.0.push("put-counters");
        deckmaste_english_v2::visit::walk_put_counters(self, value);
    }

    fn visit_remove_counters(&mut self, value: &RemoveCounters) {
        self.0.push("remove-counters");
        deckmaste_english_v2::visit::walk_remove_counters(self, value);
    }

    fn visit_verb_inventory(&mut self, verb: &VerbInventoryRef) {
        match verb {
            VerbInventoryRef::Core(CoreVerbIdentity::Put) => self.0.push("verb:Core(Put)"),
            VerbInventoryRef::Core(CoreVerbIdentity::Remove) => self.0.push("verb:Core(Remove)"),
            _ => {}
        }
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
    reason = "the positive authority checks all distribution frames, ASTs, visits, and claims together"
)]
fn passive_distribution_and_counter_frames_select_typed_products() {
    let parser = parser();
    let context = context();

    for text in [
        "It can't be regenerated.",
        "It can't be regenerated this turn.",
        "It deals 2 damage divided as you choose among two target creatures.",
        "It deals 3 damage divided as you choose among three target attacking creatures.",
        "It deals X damage divided as you choose among any number of target creatures.",
        "It deals X damage divided as you choose among up to two target creatures and/or planeswalkers.",
        "It deals twice X damage divided as you choose among them instead.",
        "It deals X plus 1 damage divided as you choose among any number of target creatures.",
        "It deals X damage divided evenly, rounded down, among any number of target creatures.",
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
    let FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
        panic!("negative auxiliary retains its finite clause")
    };
    let Predicate::Auxiliary(auxiliary) = clause.predicate() else {
        panic!("negative auxiliary retains its typed predicate")
    };
    let AuxiliaryPredicate::AuxiliaryPredicate(auxiliary) = auxiliary.as_ref();
    let BarePredicate::StateDuration(duration) = auxiliary.predicate() else {
        panic!("passive duration retains its typed attachment")
    };
    let StateDurationPredicate::StateDurationPredicate(StateDurationPredicateValue {
        predicate,
        duration,
    }) = duration;
    let DurationPhrase::Fixed(duration) = duration else {
        panic!("this turn stays a fixed duration")
    };
    let FixedDurationPhrase::FixedDurationPhrase(duration) = duration;
    assert_eq!(duration.unit, TemporalUnit::Turn);
    let StateDurationBase::BarePassive(BarePassivePredicate::BarePassivePredicate(
        BarePassivePredicateValue { predicate, .. },
    )) = predicate;
    let PassivePredicate::DeclaredTransitive(declared) = predicate.as_ref() else {
        panic!("regeneration uses the declared transitive passive frame")
    };
    let DeclaredTransitivePassivePredicate::DeclaredTransitivePassivePredicate(
        DeclaredTransitivePassivePredicateValue { head },
    ) = declared;
    assert!(matches!(
        head.reference(),
        VerbInventoryRef::Declaration(id) if id.name() == "Regenerate"
    ));

    let VerbPhrase::DealDistributedDamage(distributed) = declarative_atomic(
        &parser,
        &context,
        "It deals 2 damage divided as you choose among two target creatures.",
    ) else {
        panic!("damage division keeps its exact predicate product")
    };
    assert!(matches!(
        distributed.distribution,
        DamageDistribution::AsYouChoose(ChosenDamageDistribution {
            recipient: DistributionRecipient::Object(_),
        })
    ));

    let VerbPhrase::PutCounters(PutCounters { head, counters, .. }) =
        imperative_atomic(&parser, &context, "Put an oil counter on this creature.")
    else {
        unreachable!()
    };
    assert!(matches!(
        head.reference(),
        VerbInventoryRef::Core(CoreVerbIdentity::Put)
    ));
    assert!(matches!(
        counters,
        CounterQuantity::SingularCounterQuantity(SingularCounterQuantity {
            kind: CounterKind::NamedCounter(NamedCounter {
                name: CounterName::Oil,
            }),
        })
    ));

    let VerbPhrase::RemoveCounters(RemoveCounters { head, counters, .. }) = imperative_atomic(
        &parser,
        &context,
        "Remove a counter from a nonland permanent you control.",
    ) else {
        unreachable!()
    };
    assert!(matches!(
        head.reference(),
        VerbInventoryRef::Core(CoreVerbIdentity::Remove)
    ));
    assert!(matches!(
        counters,
        CounterQuantity::UnnamedSingularCounterQuantity(_)
    ));

    let mut visitor = DistributionVisitor::default();
    let ability = assert_selected_with_specificity(
        &parser,
        &context,
        "It can't be regenerated this turn. It deals 2 damage divided as you choose among two target creatures. Put an oil counter on this creature. Remove a counter from target permanent.",
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
            "put-counters",
            "verb:Core(Put)",
            "remove-counters",
            "verb:Core(Remove)",
        ]
    );

    assert_eq!(
        exact_claim_trace(
            &parser,
            &context,
            "It deals 2 damage divided as you choose among two target creatures.",
        ),
        [
            ("It".to_owned(), "vocab:SubjectPronoun/It".to_owned()),
            (" deals".to_owned(), "core-verb:Deal".to_owned()),
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
            (" two".to_owned(), "codec:CardinalNumber".to_owned()),
            (
                " target".to_owned(),
                "vocab:AttributiveAdjective/Target".to_owned()
            ),
            (
                " creatures".to_owned(),
                "lexeme:type/Creature/plural".to_owned()
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned()
            ),
        ]
    );
}

#[test]
fn distribution_does_not_bypass_productive_target_noun_deferral() {
    let parser = parser();
    let context = context();

    for text in [
        "It deals 2 damage divided as you choose among one or two targets.",
        "It deals X damage divided evenly, rounded down, among any number of targets.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "damage distribution must not mint a context-specific target noun: {text:?}",
        );
    }
}

#[test]
fn frame_reciprocals_reject_crossed_morphology_and_boundaries() {
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
            "crossed distribution frame must reject {text:?}",
        );
    }
}

#[test]
fn scope_rejects_copular_duration_and_global_composite_amounts() {
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
        "distribution-only syntax leaked through global products: {accepted:?}",
    );
}

#[test]
fn cost_frames_select_their_complete_typed_paths() {
    let parser = parser();
    let context = context();

    for text in [
        "As an additional cost to cast this spell, discard a card.",
        "As an additional cost to attack, discard a card.",
        "You may sacrifice a Mountain rather than pay this spell's mana cost.",
        "You may cast spells from your hand without paying their mana costs.",
        "Spells cost {1} less to cast.",
        "Spells cost {1} more to cast.",
        "This ability costs {1} less to activate for each legendary creature you control.",
        "Cast this spell only if you control a snow land.",
        "Cast this spell only during your turn.",
        "Cast this spell only during your turn and only if you control a snow land.",
        "Destroy this spell only during your turn.",
        "Activate this ability only if you control a snow land.",
        "Draw a card only if you control a snow land.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
}

#[test]
fn cost_frame_reciprocals_reject_crossed_boundaries() {
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
            "crossed cost frame must reject {text:?}",
        );
    }
}

#[test]
fn cost_scope_rejects_missing_complements_modifiers_and_shortcuts() {
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
        "Draw only a card.",
        "Activate only as a sorcery.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "cost scope must reject {text:?}",
        );
    }
}

#[derive(Default)]
struct CostFrameVisitor(Vec<&'static str>);

impl Visitor for CostFrameVisitor {
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
fn cost_products_keep_ast_render_visit_and_lexical_ownership() {
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
        let mut visitor = CostFrameVisitor::default();
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

#[test]
fn then_sequences_select_pair_serial_and_intersentence_forms() {
    let parser = parser();
    let context = context();

    for text in [
        "You gain 1 life, then you connive.",
        "You gain 1 life, you connive, then a player gains 2 life.",
        "Gain 1 life, then connive.",
        "Gain 1 life, connive, then draw a card.",
        "Draw three cards. Then discard two cards.",
        "When this creature enters, you may search your library for a Sliver card, reveal that card, put it into your hand, then shuffle.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    let ordinary =
        assert_selected_with_specificity(&parser, &context, "Draw a card. Discard a card.", true);
    let Ability::Plain(Plain { body }) = ordinary else {
        panic!("ordinary sentence juxtaposition keeps its plain ability envelope")
    };
    assert!(
        matches!(body.as_ref(), AbilityBody::Sentences(_)),
        "a non-then sentence sequence is not an ordered then structure",
    );
}

#[test]
fn then_sequences_have_exact_ast_build_visit_and_structural_ownership() {
    #[derive(Default)]
    struct ThenVisitor {
        sequences: usize,
        sentences: usize,
    }
    impl Visitor for ThenVisitor {
        fn visit_then_sentence_sequence(&mut self, value: &ThenSentenceSequence) {
            self.sequences += 1;
            deckmaste_english_v2::visit::walk_then_sentence_sequence(self, value);
        }

        fn visit_sentence(&mut self, value: &Sentence) {
            self.sentences += 1;
            deckmaste_english_v2::visit::walk_sentence(self, value);
        }
    }

    let parser = parser();
    let context = context();

    let auxiliary_text = "When this creature enters, you may search your library for a Sliver card, reveal that card, put it into your hand, then shuffle.";
    let auxiliary = assert_selected_with_specificity(&parser, &context, auxiliary_text, true);
    assert_eq!(
        auxiliary.render(&context, parser.environment()),
        auxiliary_text,
    );
    let analysis = parser.analyze(auxiliary_text, &context);
    let path = analysis.decision().unwrap().candidates()
        [analysis.decision().unwrap().selected().unwrap()]
    .construction_path();
    assert!(
        path.iter()
            .any(|item| item == "AuxiliaryPredicateAuxiliaryPredicate")
    );
    assert!(
        path.iter()
            .any(|item| item == "ThenPredicateSequenceThenPredicateSequence")
    );
    assert_eq!(
        exact_claim_trace(&parser, &context, auxiliary_text)
            .into_iter()
            .filter(|(_, owner)| owner.starts_with("structural:ThenPredicateSequence"))
            .collect::<Vec<_>>(),
        [
            (
                ", ".to_owned(),
                "structural:ThenPredicateSequenceValue/members/separator/first/0".to_owned(),
            ),
            (
                ", ".to_owned(),
                "structural:ThenPredicateSequenceValue/members/separator/middle/0".to_owned(),
            ),
            (
                ", then ".to_owned(),
                "structural:ThenPredicateSequenceValue/members/separator/last/0".to_owned(),
            ),
        ],
    );

    let finite_text = "You gain 1 life, you connive, then a player gains 2 life.";
    let finite = assert_selected_with_specificity(&parser, &context, finite_text, true);
    let Ability::Plain(Plain { body }) = finite else {
        panic!("finite then sequence has the plain ability envelope")
    };
    let AbilityBody::Sentences(sentences) = body.as_ref() else {
        panic!("intra-sentence then sequence remains one sentence")
    };
    let [Sentence::Attached(attached)] = sentences.sentences() else {
        panic!("finite then sequence is one attached sentence")
    };
    let ClauseAttachment::ThenSequence(finite_sequence) = attached.attachment.as_ref() else {
        panic!("finite subjects use the typed finite-clause sequence")
    };
    assert_eq!(finite_sequence.members().len(), 3);

    let predicate_text = "Gain 1 life, connive, then draw a card.";
    let predicate = assert_selected_with_specificity(&parser, &context, predicate_text, true);
    let Ability::Plain(Plain { body }) = predicate else {
        panic!("shared-subject then sequence has the plain ability envelope")
    };
    let AbilityBody::Sentences(sentences) = body.as_ref() else {
        panic!("shared-subject then sequence remains one sentence")
    };
    let [Sentence::Imperative(imperative)] = sentences.sentences() else {
        panic!("shared-subject then sequence is one imperative sentence")
    };
    let Predicate::ThenSequence(predicate_sequence) = imperative.predicate() else {
        panic!("imperative members use the shared-subject predicate sequence")
    };
    let ThenPredicateSequence::ThenPredicateSequence(predicate_sequence) =
        predicate_sequence.as_ref();
    assert_eq!(predicate_sequence.members().len(), 3);

    let sentence_text = "Draw three cards. Then discard two cards.";
    let sentence = assert_selected_with_specificity(&parser, &context, sentence_text, true);
    let Ability::Plain(Plain { body }) = sentence else {
        panic!("intersentence then sequence has the plain ability envelope")
    };
    let AbilityBody::ThenSentences(sentence_sequence) = body.as_ref() else {
        panic!("sentence-initial Then has a distinct structural AST")
    };
    assert_eq!(sentence_sequence.members().len(), 2);
    assert!(ThenSentenceSequence::new(Box::new(sentence_sequence.members().to_vec())).is_some());
    assert!(
        ThenSentenceSequence::new(Box::new(vec![sentence_sequence.members()[0].clone()])).is_none()
    );

    let mut visitor = ThenVisitor::default();
    visitor.visit_then_sentence_sequence(sentence_sequence);
    assert_eq!((visitor.sequences, visitor.sentences), (1, 2));

    let finite_structural = exact_claim_trace(&parser, &context, finite_text)
        .into_iter()
        .filter(|(_, owner)| owner.starts_with("structural:"))
        .collect::<Vec<_>>();
    assert!(
        finite_structural.contains(&(
            ", ".to_owned(),
            "structural:ThenSequence/members/separator/first/0".to_owned(),
        )),
        "{finite_structural:?}",
    );
    assert!(
        finite_structural.contains(&(
            ", then ".to_owned(),
            "structural:ThenSequence/members/separator/last/0".to_owned(),
        )),
        "{finite_structural:?}",
    );

    let sentence_structural = exact_claim_trace(&parser, &context, sentence_text)
        .into_iter()
        .filter(|(_, owner)| owner.starts_with("structural:"))
        .collect::<Vec<_>>();
    assert_eq!(
        sentence_structural,
        [
            (
                ".".to_owned(),
                "structural:ThenSentenceSequence/members/terminator/0".to_owned(),
            ),
            (
                " Then ".to_owned(),
                "structural:ThenSentenceSequence/members/separator/uniform/0".to_owned(),
            ),
            (
                ".".to_owned(),
                "structural:ThenSentenceSequence/members/terminator/0".to_owned(),
            ),
        ],
    );
}

#[test]
fn then_sequences_reject_malformed_punctuation_case_spacing_and_singletons() {
    let parser = parser();
    let context = context();

    for text in [
        "You gain 1 life then you connive.",
        "You gain 1 life, you connive then a player gains 2 life.",
        "You gain 1 life, then you connive, a player gains 2 life.",
        "You gain 1 life, Then you connive.",
        "Draw a card. then discard a card.",
        "Draw a card.  Then discard a card.",
        "Draw a card Then discard a card.",
        "Draw a card. Then discard a card",
        "Then draw a card.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "malformed then boundary must reject {text:?}",
        );
    }
}
