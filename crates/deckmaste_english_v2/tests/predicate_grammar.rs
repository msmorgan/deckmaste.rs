use deckmaste_construction_core::macro_def::DeclarationIdentity;
use deckmaste_construction_core::macro_def::Onset;
use deckmaste_construction_core::macro_def::read_str;
use deckmaste_english_v2::ast::*;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::CoreVerbIdentity;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::environment::VerbInventoryRef;
use deckmaste_english_v2::parser::ParseAnalysisOutcome;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;

#[expect(
    clippy::too_many_lines,
    reason = "the shared predicate fixture lists the declaration environment explicitly"
)]
fn environment() -> ParserEnvironment {
    let declarations = [
        (
            "/synthetic/actions/Destroy.ron",
            r#"KeywordAction(name:"Destroy",spelling:"destroy",grammar:Verb(bare:"destroy",frame_set:Transitive))"#,
        ),
        (
            "/synthetic/actions/Sacrifice.ron",
            r#"KeywordAction(name:"Sacrifice",spelling:"sacrifice",grammar:Verb(bare:"sacrifice",participle:"sacrificed",frame_set:Transitive))"#,
        ),
        (
            "/synthetic/actions/Connive.ron",
            r#"KeywordAction(name:"Connive",spelling:"connive",grammar:Verb(bare:"connive",frame_set:Intransitive))"#,
        ),
        (
            "/synthetic/actions/Scry.ron",
            r#"KeywordAction(name:"Scry",spelling:"scry",grammar:Verb(bare:"scry",third_person:"scries",frame_set:Custom(frames:[[],[Amount]])))"#,
        ),
        (
            "/synthetic/actions/Surveil.ron",
            r#"KeywordAction(name:"Surveil",spelling:"surveil",grammar:Verb(bare:"surveil",frame_set:MeasureComplement))"#,
        ),
        (
            "/synthetic/actions/Discard.ron",
            r#"KeywordAction(name:"Discard",spelling:"discard",grammar:Verb(bare:"discard",frame_set:Transitive))"#,
        ),
        (
            "/synthetic/actions/Create.ron",
            r#"KeywordAction(name:"Create",spelling:"create",grammar:Verb(bare:"create",frame_set:Transitive))"#,
        ),
        (
            "/synthetic/actions/Exile.ron",
            r#"KeywordAction(name:"Exile",spelling:"exile",grammar:Verb(bare:"exile",participle:"exiled",frame_set:Custom(frames:[[ObjectNounPhrase],[ObjectNounPhrase,PredicativeComplement]])))"#,
        ),
        (
            "/synthetic/actions/Regenerate.ron",
            r#"KeywordAction(name:"Regenerate",spelling:"regenerate",grammar:Verb(bare:"regenerate",participle:"regenerated",frame_set:Transitive))"#,
        ),
        (
            "/synthetic/actions/Declare.ron",
            r#"KeywordAction(name:"Declare",spelling:"proclaim",grammar:Verb(bare:"proclaim",frame_set:Transitive))"#,
        ),
        (
            "/synthetic/actions/Cast.ron",
            r#"KeywordAction(name:"Cast",spelling:"cast",grammar:Verb(bare:"cast",participle:"cast",frame_set:Transitive))"#,
        ),
        (
            "/synthetic/actions/Search.ron",
            r#"KeywordAction(name:"Search",spelling:"search",grammar:Verb(bare:"search",third_person:"searches",frame_set:Custom(frames:[[ObjectNounPhrase],[Lex("Preposition","For"),ObjectNounPhrase],[ObjectNounPhrase,Lex("Preposition","For"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Reveal.ron",
            r#"KeywordAction(name:"Reveal",spelling:"reveal",grammar:Verb(bare:"reveal",frame_set:Transitive))"#,
        ),
        (
            "/synthetic/actions/Shuffle.ron",
            r#"KeywordAction(name:"Shuffle",spelling:"shuffle",grammar:Verb(bare:"shuffle",frame_set:Custom(frames:[[],[ObjectNounPhrase],[ObjectNounPhrase,Lex("Preposition","Into"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Attach.ron",
            r#"KeywordAction(name:"Attach",spelling:"attach",grammar:Verb(bare:"attach",third_person:"attaches",frame_set:Custom(frames:[[ObjectNounPhrase,Lex("Preposition","To"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Amass.ron",
            r#"KeywordAction(name:"Amass",spelling:"amass",grammar:Verb(bare:"amass",third_person:"amasses",frame_set:Custom(frames:[[ObjectNounPhrase,Amount]])))"#,
        ),
        (
            "/synthetic/actions/Clash.ron",
            r#"KeywordAction(name:"Clash",spelling:"clash",grammar:Verb(bare:"clash",third_person:"clashes",frame_set:Custom(frames:[[],[Lex("Preposition","With"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Exchange.ron",
            r#"KeywordAction(name:"Exchange",spelling:"exchange",grammar:Verb(bare:"exchange",frame_set:Custom(frames:[[ObjectNounPhrase],[ObjectNounPhrase,Lex("Preposition","With"),ObjectNounPhrase],[ObjectNounPhrase,Lex("Preposition","For"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Vote.ron",
            r#"KeywordAction(name:"Vote",spelling:"vote",grammar:Verb(bare:"vote",frame_set:Custom(frames:[[],[ObjectNounPhrase],[Lex("Preposition","For"),ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/keyword_abilities/Flash.ron",
            r#"KeywordAbility(name:"Flash",spelling:"flash",grammar:FixedKeyword(surface:"flash"))"#,
        ),
        (
            "/synthetic/keyword_abilities/Hexproof.ron",
            r#"KeywordAbility(name:"Hexproof",spelling:"hexproof",grammar:FixedKeyword(surface:"hexproof"))"#,
        ),
        (
            "/synthetic/keyword_abilities/Cascade.ron",
            r#"KeywordAbility(name:"Cascade",spelling:"cascade",grammar:FixedKeyword(surface:"cascade"))"#,
        ),
        (
            "/synthetic/keyword_abilities/Equip.ron",
            r#"KeywordAbility(name:"Equip",params:[Cost],spelling:"equip <Param(0)>",grammar:FixedKeyword(surface:"equip",participial_adjective:(surface:"equipped")))"#,
        ),
        (
            "/synthetic/keyword_abilities/Enchant.ron",
            r#"KeywordAbility(name:"Enchant",params:[Subject],spelling:"enchant",grammar:FixedKeyword(surface:"enchant",participial_adjective:(surface:"enchanted")))"#,
        ),
        (
            "/synthetic/keyword_abilities/Deathtouch.ron",
            r#"KeywordAbility(name:"Deathtouch",spelling:"deathtouch",grammar:FixedKeyword(surface:"deathtouch"))"#,
        ),
        (
            "/synthetic/keyword_abilities/Flying.ron",
            r#"KeywordAbility(name:"Flying",spelling:"flying",grammar:FixedKeyword(surface:"flying"))"#,
        ),
        (
            "/synthetic/keyword_abilities/Haste.ron",
            r#"KeywordAbility(name:"Haste",spelling:"haste",grammar:FixedKeyword(surface:"haste"))"#,
        ),
        (
            "/synthetic/keyword_abilities/Reach.ron",
            r#"KeywordAbility(name:"Reach",spelling:"reach",grammar:FixedKeyword(surface:"reach"))"#,
        ),
        (
            "/synthetic/keyword_abilities/Trample.ron",
            r#"KeywordAbility(name:"Trample",spelling:"trample",grammar:FixedKeyword(surface:"trample"))"#,
        ),
        (
            "/synthetic/keyword_abilities/Vigilance.ron",
            r#"KeywordAbility(name:"Vigilance",spelling:"vigilance",grammar:FixedKeyword(surface:"vigilance"))"#,
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
            "/synthetic/turn_parts/Upkeep.ron",
            r#"TurnPart(name:"Upkeep",spelling:"upkeep",grammar:Noun(singular:"upkeep"))"#,
        ),
        (
            "/synthetic/turn_parts/Combat.ron",
            r#"TurnPart(name:"Combat",spelling:"combat",grammar:Noun(singular:"combat"))"#,
        ),
        (
            "/synthetic/turn_parts/Cleanup.ron",
            r#"TurnPart(name:"Cleanup",spelling:"cleanup",grammar:Noun(singular:"cleanup"))"#,
        ),
        (
            "/synthetic/turn_parts/EndStep.ron",
            r#"TurnPart(name:"EndStep",spelling:"end step",grammar:Noun(singular:"end step"))"#,
        ),
        (
            "/synthetic/actions/Activate.ron",
            r#"KeywordAction(name:"Activate",spelling:"activate",grammar:Verb(bare:"activate",participle:"activated",frame_set:Custom(frames:[[],[ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/actions/Untap.ron",
            r#"KeywordAction(name:"Untap",spelling:"untap",grammar:Verb(bare:"untap",frame_set:Custom(frames:[[],[ObjectNounPhrase]])))"#,
        ),
        (
            "/synthetic/counters/ChargeCounter.ron",
            r#"CounterKind(name:"ChargeCounter",spelling:"charge",grammar:FixedTerm(surface:"charge"))"#,
        ),
        (
            "/synthetic/counters/LoreCounter.ron",
            r#"CounterKind(name:"LoreCounter",spelling:"lore",grammar:FixedTerm(surface:"lore"))"#,
        ),
        (
            "/synthetic/counters/LoyaltyCounter.ron",
            r#"CounterKind(name:"LoyaltyCounter",spelling:"loyalty",grammar:FixedTerm(surface:"loyalty"))"#,
        ),
        (
            "/synthetic/counters/OilCounter.ron",
            r#"CounterKind(name:"OilCounter",spelling:"oil",grammar:FixedTerm(surface:"oil"))"#,
        ),
        (
            "/synthetic/counters/SporeCounter.ron",
            r#"CounterKind(name:"SporeCounter",spelling:"spore",grammar:FixedTerm(surface:"spore"))"#,
        ),
        (
            "/synthetic/counters/StunCounter.ron",
            r#"CounterKind(name:"StunCounter",spelling:"stun",grammar:FixedTerm(surface:"stun"))"#,
        ),
        (
            "/synthetic/counters/TimeCounter.ron",
            r#"CounterKind(name:"TimeCounter",spelling:"time",grammar:FixedTerm(surface:"time"))"#,
        ),
        (
            "/synthetic/counters/VerseCounter.ron",
            r#"CounterKind(name:"VerseCounter",spelling:"verse",grammar:FixedTerm(surface:"verse"))"#,
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

fn transitive_lexical_verb_phrase(predicate: &VerbPhrase) -> &TransitivePredicate {
    let VerbPhrase::BaseVerbPhrase(base) = predicate else {
        panic!("transitive predicate has the shared base-frame envelope")
    };
    let LexicalVerbPhrase::TransitiveLexicalVerbPhrase(frame) = base.frame.as_ref() else {
        panic!("transitive predicate has the transitive frame_set frame")
    };
    let TransitiveLexicalVerbPhrase::TransitivePredicate(predicate) = frame.as_ref();
    predicate
}

fn imperative_transitive(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
) -> TransitivePredicate {
    transitive_lexical_verb_phrase(&imperative_atomic(parser, context, text)).clone()
}

fn declarative_predicate(parser: &Parser, context: &ParseContext<'_>, text: &str) -> Predicate {
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
    clause.predicate().clone()
}

fn declarative_atomic(parser: &Parser, context: &ParseContext<'_>, text: &str) -> VerbPhrase {
    let Predicate::Atomic(predicate) = declarative_predicate(parser, context, text) else {
        panic!("one finite predicate has an atomic envelope: {text:?}")
    };
    *predicate
}

fn declarative_get_power_toughness_with_duration(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
) -> (GetPowerToughnessVerb, PowerToughnessAdjustment) {
    let Predicate::Adjunct(adjunct_predicate) = declarative_predicate(parser, context, text) else {
        panic!("duration has the shared predicate-adjunct envelope: {text:?}")
    };
    let PredicateAdjunctPredicate::PredicateAdjunctPredicate(value) = adjunct_predicate.as_ref()
    else {
        panic!("duration has the general predicate-adjunct construction: {text:?}")
    };
    assert_duration_adjunct(value.adjunct.as_ref(), text);
    let LexicalVerbPhrase::GetPowerToughnessLexicalVerbPhrase(predicate) = value.predicate.as_ref()
    else {
        panic!("power/toughness adjustment remains a lexical verb phrase: {text:?}")
    };
    let GetPowerToughnessLexicalVerbPhrase::GetPowerToughness(predicate) = predicate;
    (predicate.head.clone(), predicate.adjustment.clone())
}

fn assert_duration_adjunct(adjunct: &PredicateAdjunct, text: &str) {
    let PredicateAdjunct::Duration(duration) = adjunct else {
        panic!("the shared adjunct is a duration: {text:?}")
    };
    assert!(matches!(
        duration.duration.as_ref(),
        DurationPhrase::Until(_)
    ));
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
    _permits_specificity: bool,
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
            || decision.resolution() == SelectionResolution::Specificity,
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

    fn visit_predicative_nominal_value(&mut self, value: &PredicativeNominalValue) {
        self.0.push("nominal");
        deckmaste_english_v2::visit::walk_predicative_nominal_value(self, value);
    }

    fn visit_bare_locative_proform(&mut self, value: &BareLocativeProform) {
        self.0.push("from-anywhere");
        deckmaste_english_v2::visit::walk_bare_locative_proform(self, value);
    }

    fn visit_verb_inventory(&mut self, verb: &VerbInventoryRef) {
        if matches!(verb, VerbInventoryRef::Core(CoreVerbIdentity::Enter)) {
            self.0.push("enter-head");
        }
    }

    fn visit_as_clause_tail(&mut self, value: &AsClauseTail) {
        self.0.push("as-clause");
        deckmaste_english_v2::visit::walk_as_clause_tail(self, value);
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
    assert_selected_with_specificity(&parser, &context, text, true);

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

    assert_selected_with_specificity(&parser, &context, "This permanent is all colors.", true);

    let as_ability = assert_selected(
        &parser,
        &context,
        "As this artifact enters, choose a color.",
    );
    let Ability::Plain(Plain { body }) = as_ability else {
        panic!("as-entry witness has an ordinary ability envelope")
    };
    let AbilityBody::Sentences(sentences) = body else {
        panic!("as-entry witness has an ordinary sentence body")
    };
    let [Sentence::Attached(Attached { attachment })] = sentences.sentences() else {
        panic!("as-entry witness has one typed attached sentence")
    };
    assert!(matches!(
        attachment.as_ref(),
        ClauseAttachment::PreposedPredicateClauseTail(value)
            if matches!(value.tail.as_ref(), PreposedClauseTail::As(_))
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
    let AbilityBody::Sentences(sentences) = body else {
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
    assert_eq!(coordination.members().len(), 2);
}

#[test]
fn combat_frames_keep_active_frame_set_passive_agents_and_if_able_distinct() {
    let parser = parser();
    let context = context();

    for (text, expected_path_member) in [
        (
            "Whenever this creature attacks, draw a card.",
            "IntransitiveLexicalVerbPhraseIntransitivePredicate",
        ),
        (
            "Whenever this creature attacks a player, draw a card.",
            "TransitiveLexicalVerbPhraseTransitivePredicate",
        ),
        (
            "Whenever this creature blocks a creature, draw a card.",
            "TransitiveLexicalVerbPhraseTransitivePredicate",
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
    assert_selected(
        &parser,
        &context,
        "This creature can't be blocked except by two or more creatures.",
    );

    for crossed in [
        "Whenever this creature block a creature, draw a card.",
        "Target creature attack target opponent this turn if able.",
        "This creature can't be blocked except two or more creatures.",
    ] {
        assert!(
            parser.parse(crossed, &context).is_err(),
            "combat Concord Class and passive-agent syntax reject {crossed:?}",
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
    assert_selected_with_specificity(&parser, &context, "Add one mana of any color.", true);
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

    assert_selected_with_specificity(&parser, &context, "Add one mana of any colors.", true);
    for crossed in [
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
        ["enter-head"],
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
        assert_selected_with_specificity(&parser, &context, text, true);
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

    fn visit_quoted_ability_value(&mut self, value: &QuotedAbilityValue) {
        self.0.push("quoted-ability");
        deckmaste_english_v2::visit::walk_quoted_ability_value(self, value);
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
    let ability = assert_selected_with_specificity(&parser, &context, text, true);
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
                "vocab:TargetingMarker/Target".to_owned(),
            ),
            (
                " Equipment".to_owned(),
                "lexeme:artifact_subtype/Equipment/singular".to_owned(),
            ),
            (" to".to_owned(), "vocab:Preposition/To".to_owned(),),
            (
                " target".to_owned(),
                "vocab:TargetingMarker/Target".to_owned(),
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
fn quote_boundary_discharges_the_enclosing_sentence_terminator() {
    let parser = parser();
    let context = context();
    let text = r#"All Slivers have "When this permanent enters, draw a card.""#;
    let ability = assert_selected_with_specificity(&parser, &context, text, true);
    let Ability::Plain(Plain { body }) = &ability else {
        panic!("quoted complement witness has an ordinary ability envelope")
    };
    let AbilityBody::QuoteTerminatedStatement(statement) = body else {
        panic!("quoted complement witness has the derived quote terminator envelope")
    };
    let Sentence::Declarative(statement) = &statement.sentence else {
        panic!("quoted complement witness is a declarative sentence")
    };
    let Clause::Finite(statement) = statement.clause.as_ref() else {
        panic!("quoted complement witness is an ordinary finite clause")
    };
    let FiniteClause::PlainFiniteClause(statement) = statement.as_ref() else {
        panic!("quoted complement witness is an ordinary finite clause")
    };
    let Predicate::Atomic(predicate) = statement.predicate() else {
        panic!("quoted complement witness ends in an atomic predicate")
    };
    let VerbPhrase::BaseVerbPhrase(base) = predicate.as_ref() else {
        panic!("quoted complement witness ends in a base verb phrase")
    };
    let LexicalVerbPhrase::GrantedAbilityLexicalVerbPhrase(predicate) = base.frame.as_ref() else {
        panic!("quoted complement witness ends in the granted-ability verb frame")
    };
    let GrantedAbilityLexicalVerbPhrase::GrantedAbilityLexicalVerbPhrase(predicate) =
        predicate.as_ref();
    let GrantedAbility::Quoted(granted) = predicate.ability.as_ref() else {
        panic!("quoted complement witness stores a quoted granted ability")
    };
    assert!(matches!(
        predicate.head.reference(),
        VerbInventoryRef::Core(CoreVerbIdentity::Have)
    ));
    let QuotedAbility::QuotedAbility(quoted) = granted.as_ref();
    let QuotedBlock::QuotedBlock(quoted_block) = &quoted.block;
    let [DocumentBlock::Ability(quoted_ability)] = quoted_block.block.blocks.as_slice() else {
        panic!("quoted complement stores a one-block ability document")
    };
    let Ability::Triggered(_) = quoted_ability.as_ref() else {
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

    let nominal_text = r#"Create a 1/1 red Goblin creature token with "This token can't block.""#;
    let nominal = assert_selected_with_specificity(&parser, &context, nominal_text, true);
    let Ability::Plain(Plain {
        body: AbilityBody::QuoteTerminatedStatement(statement),
    }) = &nominal
    else {
        panic!("sentence-final quoted nominal uses the shared terminator envelope")
    };
    assert!(matches!(statement.sentence, Sentence::Imperative(_)));

    let claims = exact_claim_trace(&parser, &context, nominal_text);
    assert!(claims.iter().any(|(surface, owner)| {
        surface == "." && owner == "structural:Sentences/sentences/terminator/0"
    }));
    assert_eq!(
        claims.last(),
        Some(&(
            "\"".to_owned(),
            "form:quoted_ability/quoted_ability/1/affix".to_owned(),
        )),
        "the quote is the final outer-sentence byte; no second period is claimed",
    );

    let sequence_text =
        r#"Create a 1/1 red Goblin creature token. It has "This token can't block.""#;
    let sequence = assert_selected_with_specificity(&parser, &context, sequence_text, true);
    let Ability::Plain(Plain {
        body: AbilityBody::QuoteTerminatedSentences(sequence),
    }) = &sequence
    else {
        panic!("a final quoted sentence extends an ordinary sentence sequence")
    };
    assert_eq!(sequence.preceding().len(), 1);
    assert!(matches!(sequence.sentence, Sentence::Declarative(_)));

    assert_selected_with_specificity(
        &parser,
        &context,
        r#"Until end of turn, any number of target creatures you control each get +1/+0 and gain "When this creature dies, draw a card.""#,
        true,
    );

    assert!(
        parser.parse("Destroy target creature", &context).is_err(),
        "an ordinary sentence still requires its own final period",
    );
    assert!(
        parser
            .parse("Destroy target creature. Draw a card", &context)
            .is_err(),
        "an ordinary final sentence in a sequence still requires its period",
    );
    assert!(
        parser
            .parse(
                r#"Target creature gains "When this creature dies, draw a card." instead"#,
                &context,
            )
            .is_err(),
        "a trailing literal keeps an earlier quoted block from discharging the sentence terminator",
    );
}

#[test]
fn ability_expressions_use_the_general_coordination_algebra() {
    let parser = parser();
    let context = context();
    let witnesses = [
        (
            r#"Create a 1/1 red Goblin creature token with trample, haste, and "This creature can't block.""#,
            "AbilityExpressionAndAbilityCoordination",
        ),
        (
            "Destroy target creature with deathtouch, hexproof, reach, or trample.",
            "AbilityExpressionOrAbilityCoordination",
        ),
        (
            "Destroy target creature with deathtouch and/or trample.",
            "AbilityExpressionAndOrAbilityCoordination",
        ),
        (
            r#"Enchanted creature has "This creature can't block." and "This creature can't attack.""#,
            "AbilityExpressionAndAbilityCoordination",
        ),
    ];

    for (text, expected_path_member) in witnesses {
        let analysis = parser.analyze(text, &context);
        assert_selected_with_specificity(&parser, &context, text, true);
        let decision = analysis
            .decision()
            .expect("ability coordination witness has a selection decision");
        let ordinal = decision
            .selected()
            .expect("ability coordination witness has a selected derivation");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("selected ordinal names an ability coordination derivation");
        assert!(
            selected
                .construction_path()
                .iter()
                .any(|name| name == expected_path_member),
            "{text:?}: {:#?}",
            selected.construction_path(),
        );
        if text.starts_with("Enchanted creature has") {
            assert!(
                selected.construction_path().iter().any(|name| {
                    name == "GrantedAbilityLexicalVerbPhraseGrantedAbilityLexicalVerbPhrase"
                }),
                "{text:?}: {:#?}",
                selected.construction_path(),
            );
        }
    }

    assert_selected_with_specificity(&parser, &context, "If it has haste, draw a card.", true);

    let uncoordinated = "Destroy target creature with deathtouch, trample.";
    assert!(
        parser.analyze(uncoordinated, &context).decision().is_none(),
        "a comma alone does not coordinate granted abilities",
    );
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
                    !candidate.construction_path().iter().any(|identity| {
                        identity == "GrantedAbilityLexicalVerbPhraseGrantedAbilityLexicalVerbPhrase"
                    })
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
fn exchange_uses_declared_object_frame_set_and_a_typed_control_reference() {
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
fn declared_custom_frame_sets_select_one_lexical_verb_phrase_per_verb_frame() {
    let parser = parser();
    let context = context();

    for text in [
        "Amass Slivers 2.",
        "Exchange target creature for target artifact.",
        "Shuffle target card into its owner's library.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
}

#[test]
fn maximum_hand_size_is_a_typed_copular_scalar_statement() {
    let parser = parser();
    let context = context();
    let text = "Your maximum hand size is seven.";
    let ability = assert_selected_with_specificity(&parser, &context, text, true);
    let mut visitor = ObjectFrameVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["predicative-scalar"]);
}

#[test]
fn copular_scalar_equalities_use_the_predicative_complement_sum() {
    let parser = parser();
    let context = context();

    for text in [
        "Context Card's power is equal to the number of cards in your hand.",
        "Context Card's power and toughness are each equal to the number of cards in your hand.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
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
    let text = "Target creature gets +3/+1 until end of turn.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = ObjectFrameVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["get-power-toughness"], "{text:?}");
    let (head, _) = declarative_get_power_toughness_with_duration(
        &parser,
        &context,
        "Target creature gets +3/+1 until end of turn.",
    );
    assert!(matches!(
        head.reference(),
        VerbInventoryRef::Core(CoreVerbIdentity::Get)
    ));
    let text = "This creature has base power and toughness 4/4.";
    assert_selected(&parser, &context, text);
    let analysis = parser.analyze(text, &context);
    let decision = analysis.decision().expect("base value has a decision");
    let selected = decision
        .candidates()
        .iter()
        .find(|candidate| Some(candidate.ordinal()) == decision.selected())
        .expect("selected ordinal names a candidate");
    assert!(
        selected
            .construction_path()
            .iter()
            .any(|name| name.contains("DeclaredObjectPredicative")),
        "{:#?}",
        selected.construction_path(),
    );
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
fn a_causative_complement_hosts_the_shared_clause_level_duration() {
    let parser = parser();
    let context = context();
    let text = "You may have target creature get -1/-1 until end of turn.";
    let ability = assert_selected(&parser, &context, text);
    assert_eq!(ability.render(&context, parser.environment()), text);

    let Predicate::Auxiliary(auxiliary) = declarative_predicate(&parser, &context, text) else {
        panic!("the modal takes a bare complement: {text:?}")
    };
    let AuxiliaryPredicate::AuxiliaryPredicate(auxiliary) = auxiliary.as_ref();
    let BarePredicate::Atomic(complement) = auxiliary.predicate() else {
        panic!("the causative is the modal's atomic complement: {text:?}")
    };
    let VerbPhrase::HaveObjectControl(causative) = complement.as_ref() else {
        panic!("the modal complement is the object-control frame: {text:?}")
    };
    let BarePredicate::Adjunct(embedded) = causative.predicate() else {
        panic!("the embedded clause carries the shared adjunct envelope: {text:?}")
    };
    let PredicateAdjunctPredicate::PredicateAdjunctPredicate(embedded) = embedded.as_ref() else {
        panic!("the embedded duration uses the general adjunct construction: {text:?}")
    };
    assert_duration_adjunct(embedded.adjunct.as_ref(), text);
    assert!(matches!(
        embedded.predicate.as_ref(),
        LexicalVerbPhrase::GetPowerToughnessLexicalVerbPhrase(_)
    ));
}

#[test]
fn negative_adjustments_build_render_visit_and_claim_the_typed_sign_product() {
    let parser = parser();
    let context = context();
    let text = "Target creature gets -1/-1 until end of turn.";
    let ability = assert_selected(&parser, &context, text);
    let (head, adjustment) = declarative_get_power_toughness_with_duration(&parser, &context, text);
    let PowerToughnessAdjustment::PowerToughnessAdjustment(adjustment) = adjustment;
    assert!(matches!(
        head.reference(),
        VerbInventoryRef::Core(CoreVerbIdentity::Get)
    ));
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
                "vocab:TargetingMarker/Target".to_owned(),
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
                "lexeme:CommonNoun/End/singular".to_owned(),
            ),
            (
                " of".to_owned(),
                "vocab:Preposition/Of".to_owned(),
            ),
            (
                " turn".to_owned(),
                "lexeme:CommonNoun/Turn/singular".to_owned(),
            ),
            (
                ".".to_owned(),
                "structural:Sentences/sentences/terminator/0".to_owned(),
            ),
        ],
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
fn adjustment_sign_slash_pairing_and_concord_class_boundaries_are_reciprocal() {
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
fn ordinary_ability_nouns_and_keyword_grants_use_distinct_verb_frames() {
    let parser = parser();
    let context = context();
    let text = "Target creature loses all abilities.";
    let ability = assert_selected(&parser, &context, text);
    let predicate =
        transitive_lexical_verb_phrase(&declarative_atomic(&parser, &context, text)).clone();
    assert!(matches!(
        predicate.head.reference(),
        VerbInventoryRef::Core(CoreVerbIdentity::Lose)
    ));
    let mut visitor = ObjectFrameVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["lose-abilities"]);

    for keyword_grant in [
        "Target creature gains flying until end of turn.",
        "Creatures you control have vigilance.",
    ] {
        let analysis = parser.analyze(keyword_grant, &context);
        assert_selected_with_specificity(&parser, &context, keyword_grant, true);
        let decision = analysis
            .decision()
            .expect("keyword grant has a selection decision");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("keyword grant has one selected candidate");
        assert!(
            selected.construction_path().iter().any(|name| {
                name == "GrantedAbilityLexicalVerbPhraseGrantedAbilityLexicalVerbPhrase"
            }),
            "{keyword_grant:?}: {:#?}",
            selected.construction_path(),
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
    let predicate = transitive_lexical_verb_phrase(predicate.as_ref());
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
    let predicate = transitive_lexical_verb_phrase(predicate.as_ref());
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
fn shared_transitive_lexical_verb_phrase_preserves_visit_order_and_literal_claims() {
    let parser = parser();
    let context = context();
    let text = "You sacrifice target player and control target player.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = LexicalVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(
        visitor.0,
        [
            "core:Declaration(DeclarationIdentity { kind: KeywordAction, name: \"Sacrifice\" })",
            "declared:Sacrifice",
            "noun:Player",
            "core:Core(Control)",
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
            (13, 20, "vocab:TargetingMarker/Target".to_owned(),),
            (20, 27, "lexeme:CommonNoun/Player/singular".to_owned()),
            (
                27,
                32,
                "structural:AndPredicateCoordination/members/separator/pair/0".to_owned(),
            ),
            (32, 39, "core-verb:Control".to_owned()),
            (39, 46, "vocab:TargetingMarker/Target".to_owned(),),
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
fn shared_active_frames_reject_complement_and_concord_class_reciprocals() {
    let parser = parser();
    let context = context();
    for text in [
        "Destroy.",
        "Control.",
        "Surveil.",
        "Surveil target player.",
        "Sacrifice 2.",
        "It sacrifice target player.",
        "You sacrifices target player.",
        "It connive.",
        "You connives.",
        "Connive target player.",
        "Scry target player.",
        "Scry 2 target player.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong frame or Concord Class must reject {text:?}",
        );
    }
    assert_selected_with_specificity(&parser, &context, "Scry 2.", true);
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
fn preterite_copulas_keep_every_supported_construction_path() {
    let parser = parser();
    let context = context();
    for (text, required_path) in [
        (
            "It was dealt damage.",
            "FinitePassivePredicateFinitePassivePredicate",
        ),
        (
            "They were dealt damage.",
            "FinitePassivePredicateFinitePassivePredicate",
        ),
        (
            "It was legendary.",
            "FiniteCopularPredicateFiniteCopularPredicate",
        ),
        (
            "They were legendary.",
            "FiniteCopularPredicateFiniteCopularPredicate",
        ),
        ("There was a card.", "FiniteClauseExistentialFiniteClause"),
        ("There were cards.", "FiniteClauseExistentialFiniteClause"),
        (
            "Destroy a creature that was legendary.",
            "CopularSubjectGapRelativeClauseCopularSubjectGapRelativeClause",
        ),
        (
            "Destroy all permanents that were legendary.",
            "CopularSubjectGapRelativeClauseCopularSubjectGapRelativeClause",
        ),
        (
            "Tapped creatures you control can block as though they were untapped.",
            "IrrealisCopularClauseIrrealisCopularClause",
        ),
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected preterite copula path has a decision");
        let ordinal = decision
            .selected()
            .expect("selected preterite copula path has an ordinal");
        let path = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("selected ordinal names a retained preterite copula candidate")
            .construction_path();
        assert!(
            path.iter().any(|actual| actual == required_path),
            "{text:?} must retain {required_path}: {path:?}",
        );
    }
}

#[test]
fn finite_clause_families_compose_in_triggers_and_conditions() {
    let parser = parser();
    let context = context();

    for text in [
        "Whenever this creature is dealt damage, it deals that much damage to you.",
        "Whenever this creature is dealt damage, you gain 1 life.",
        "Whenever this creature is dealt damage, it deals that much damage to each player.",
        "Whenever a creature is put into your graveyard from the battlefield, you gain 1 life.",
        "Whenever a permanent is turned face up, this creature deals 1 damage to target creature.",
        "Whenever this creature is dealt combat damage, you gain that much life.",
        "Whenever this creature is dealt damage, each opponent gains that much life.",
        "Whenever a creature you control is put into your graveyard from the battlefield, you gain 1 life.",
        "At the beginning of your upkeep, if all creatures are white, you gain 1 life.",
        "At the beginning of each player's end step, if that player didn't cast a spell this turn, this enchantment deals 4 damage to that player.",
    ] {
        let ability = assert_selected_with_specificity(&parser, &context, text, true);
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected finite family has a decision");
        let selected_ordinal = decision
            .selected()
            .expect("selected finite family has an ordinal");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == selected_ordinal)
            .expect("selected ordinal names a retained candidate");
        assert!(
            selected
                .construction_path()
                .iter()
                .any(|name| name == "FiniteClausePlainFiniteClause"),
            "finite family must use the general finite-clause algebra: {text:?}: {selected:?}",
        );
        assert_eq!(ability.render(&context, parser.environment()), text);
    }
}

#[test]
fn finite_clause_generalization_preserves_complements_and_rejects_crossed_forms() {
    let parser = parser();
    let context = context();

    for text in [
        "It is legendary.",
        "They are white.",
        "It is a creature.",
        "It was tapped.",
        "They were 2/2.",
        "Be white.",
        "Become tapped.",
        "It becomes blocked by target creature.",
        "It didn't cast a spell.",
        "It would attack.",
        "It would be white.",
        "It can't be dealt damage.",
        "It is dealt combat damage.",
        "It is put into your graveyard from the battlefield.",
        "It is turned face up.",
        "A spell was cast.",
        "It deals damage.",
        "It gains life.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    for text in [
        "It are white.",
        "They is white.",
        "It become tapped.",
        "They becomes tapped.",
        "It didn't casts a spell.",
        "It would attacks.",
        "It can't is dealt damage.",
        "It is deal damage.",
        "It is dealt face up.",
        "It is put damage.",
        "It is turned damage.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "crossed concord_class, form, or complement must reject {text:?}",
        );
    }
    assert_selected_with_specificity(&parser, &context, "It is dealt.", true);
    assert_selected_with_specificity(&parser, &context, "A spell was cast damage.", true);
    assert_selected_with_specificity(&parser, &context, "It deals life.", true);
    assert_selected_with_specificity(&parser, &context, "It gains damage.", true);
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

    for (text, required_path) in [
        ("Put two stun counters on it.", "VerbPhrasePutOn"),
        (
            "It deals damage equal to its power to target creature.",
            "MassNounMassNoun",
        ),
        ("Draw two cards.", "NounPluralHead"),
    ] {
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected generic frame has a decision");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("selected ordinal names a candidate");
        assert!(
            selected
                .construction_path()
                .iter()
                .any(|name| name.contains(required_path)),
            "{text:?}: {:?}",
            selected.construction_path(),
        );
        assert!(
            selected.construction_path().iter().all(|name| {
                !name.contains("PutCounters")
                    && !name.contains("DealDamage")
                    && !name.contains("DrawCards")
                    && !name.contains("LifeAmount")
            }),
            "the selected path must use shared linguistic frames: {text:?}: {:?}",
            selected.construction_path(),
        );
    }
}

#[test]
fn scalar_values_compose_genitives_counts_and_post_recipient_equalities() {
    let parser = parser();
    let context = context();

    for text in [
        "Draw cards equal to the blue creature's toughness.",
        "Draw cards equal to the blue creatures' toughness.",
        "Draw cards equal to the sacrificed creature's toughness.",
        "Deal damage equal to the number of Slivers you control to target artifact.",
        "Deal damage to target artifact equal to the number of Slivers you control.",
        "Gain life equal to twice the number of Slivers you control.",
        "Draw cards equal to the greatest power among creatures you control.",
        "Draw cards equal to the number of artifacts they control.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

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

    for text in [
        "Draw cards equal to the blue creature's toughness.",
        "Deal damage to target artifact equal to the number of Slivers you control.",
        "Gain life equal to twice the number of Slivers you control.",
        "Draw cards equal to the greatest power among creatures you control.",
    ] {
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected scalar composition has a decision");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("selected ordinal names a candidate");
        assert!(
            selected.construction_path().iter().all(|name| {
                !name.contains("DrawCardsEqualTo")
                    && !name.contains("DealDamageToEqualTo")
                    && !name.contains("LifeEquality")
            }),
            "scalar composition must flow through shared frames: {text:?}: {:?}",
            selected.construction_path(),
        );
    }

    for malformed in [
        "Draw cards equal to the blue creature toughness.",
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
fn object_gap_relatives_follow_subject_concord_class() {
    let parser = parser();
    let context = context();

    for text in [
        "Creatures target player controls get -2/-2 until end of turn.",
        "It deals X damage divided evenly, rounded down, among all creatures target opponent controls.",
        "Destroy target creature you don't control.",
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
            "singular determiner controller must reject bare concord_class in {malformed:?}",
        );
    }
}

#[test]
fn floated_subject_quantifiers_relay_plural_concord_class() {
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
        "floated each requires a plural-concord_class subject",
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
fn predicate_adjuncts_pin_postposed_prepositions_outside_their_objects() {
    let parser = parser();
    let context = context();

    for (text, expected_preposition) in [
        (
            "Untap all permanents during each other player's untap step.",
            Preposition::During,
        ),
        (
            "Return it to its owner's hand at the beginning of the next end step.",
            Preposition::At,
        ),
        (
            "Draw a card for each creature you control.",
            Preposition::For,
        ),
    ] {
        let analysis = parser.analyze_sentence(text, &context);
        let Some(Sentence::Imperative(sentence)) = analysis.selected() else {
            panic!("the predicate-adjunct witness is imperative: {text:?}")
        };
        let Predicate::Adjunct(predicate) = sentence.predicate() else {
            panic!("the PP must attach to the predicate: {text:?}")
        };
        let PredicateAdjunctPredicate::PrepositionalPredicateAdjunctPredicate(value) =
            predicate.as_ref()
        else {
            panic!("the PP must use the general prepositional attachment: {text:?}")
        };
        let PredicateAdjunct::Prepositional(adjunct) = value.adjunct.as_ref() else {
            panic!("the shared adjunct value must be prepositional: {text:?}")
        };
        let PrepositionalPhrase::PrepositionalPhrase(phrase) = adjunct.adjunct() else {
            panic!("the witness uses an ordinary prepositional phrase: {text:?}")
        };
        assert_eq!(phrase.preposition, expected_preposition, "{text:?}");
    }

    let text = "This ability costs {1} less to activate for each legendary creature you control.";
    let analysis = parser.analyze_sentence(text, &context);
    let Some(Sentence::Declarative(sentence)) = analysis.selected() else {
        panic!("the cost-comparison witness is declarative")
    };
    let Clause::Finite(clause) = sentence.clause.as_ref() else {
        panic!("the cost-comparison witness has an ordinary finite clause")
    };
    let FiniteClause::PlainFiniteClause(clause) = clause.as_ref() else {
        panic!("the cost-comparison witness has an ordinary finite clause")
    };
    let Predicate::Adjunct(predicate) = clause.predicate() else {
        panic!("for each must attach outside the cost-comparison predicate")
    };
    let PredicateAdjunctPredicate::PrepositionalPredicateAdjunctPredicate(value) =
        predicate.as_ref()
    else {
        panic!("for each must use the general prepositional attachment")
    };
    assert!(matches!(
        value.predicate.as_ref(),
        PrepositionalPredicateAdjunctHost::CostComparison(_)
    ));
}

#[test]
fn object_gap_adjunct_attachment_selects_the_low_relative_reading() {
    let parser = parser();
    let context = context();

    let selected_path = |text: &str| {
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .unwrap_or_else(|| panic!("{text:?} must retain a selection decision: {analysis:?}"));
        let ordinal = decision
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {decision:?}"));
        decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("the selected ordinal names a retained candidate")
            .construction_path()
            .to_vec()
    };

    for text in [
        "Destroy each creature you sacrifice in your graveyard.",
        "Exile each card you exile in your graveyard.",
        "Destroy each creature you sacrifice during your upkeep.",
        "Untap all permanents you control during each other player's untap step.",
    ] {
        let path = selected_path(text);
        assert!(
            path.iter().any(|name| {
                name == "PositiveObjectGapRelativeClausePositiveObjectGapRelativeWithPrepositionalAdjunct"
            }),
            "the object-gap relative selects the low adjunct attachment for {text:?}: {path:?}",
        );
    }

    for text in [
        "Spells your opponents cast during your turn cost {1} more to cast.",
        "The next spell you cast this turn has cascade.",
        "Spells you cast this turn cost {1} less to cast.",
    ] {
        let path = selected_path(text);
        assert!(
            path.iter().any(|name| name
                == "PositiveObjectGapRelativeClausePositiveObjectGapRelativeWithAdjunct"
                || name
                    == "PositiveObjectGapRelativeClausePositiveObjectGapRelativeWithPrepositionalAdjunct"),
            "the object-gap relative consumes the low adjunct attachment for {text:?}: {path:?}",
        );
    }
}

#[test]
fn transitive_subject_gap_relatives_take_objects_before_later_postmodifiers() {
    let parser = parser();
    let context = context();

    for text in [
        "Destroy a creature that has a card from your graveyard.",
        "Return target creature that controls a creature from your graveyard to your hand.",
    ] {
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .unwrap_or_else(|| panic!("{text:?} must retain a decision: {analysis:?}"));
        let ordinal = decision
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {decision:?}"));
        let path = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("the selected ordinal names a candidate")
            .construction_path();
        assert!(
            path.iter().any(|name| name
                == "FiniteSubjectGapRelativeClauseFiniteTransitiveSubjectGapRelativeClause"),
            "the subject-gap relative must consume its declared object for {text:?}: {path:?}",
        );
    }
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
    assert!(
        parser
            .parse("Draw a card as long you control a Forest.", &context)
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

    assert_selected_with_specificity(
        &parser,
        &context,
        "Exile target Equipment attached to that creature.",
        true,
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
    let Predicate::Adjunct(adjunct_predicate) = sentence.predicate() else {
        panic!("the subject-relative NP remains inside an ordinary adjunct predicate")
    };
    let PredicateAdjunctPredicate::PredicateAdjunctPredicate(value) = adjunct_predicate.as_ref()
    else {
        panic!("the duration adjunct uses the general base-frame attachment")
    };
    assert!(matches!(
        value.predicate.as_ref(),
        LexicalVerbPhrase::TransitiveLexicalVerbPhrase(_)
    ));
    assert!(matches!(
        value.adjunct.as_ref(),
        PredicateAdjunct::Duration(_)
    ));
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
fn subject_gap_relatives_relay_nominal_concord_class_into_finite_predicates() {
    let parser = parser();
    let context = context();

    for text in [
        "Destroy a creature that attacks.",
        "Destroy a creature that would attack.",
        "Destroy all permanents that are legendary.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
    for text in [
        "Destroy a creature that attack.",
        "Destroy all permanents that is legendary.",
    ] {
        assert!(parser.parse(text, &context).is_err(), "reject {text:?}");
    }
}

#[test]
fn copular_complements_accept_color_nominal_and_duration_witnesses() {
    let parser = parser();
    let context = context();

    for text in [
        "Destroy target creature that is red.",
        "Destroy target creature that is a Goblin.",
        "Target land becomes a 3/3 creature.",
        "Target land becomes a 3/3 creature until end of turn.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
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
        assert_selected_with_specificity(&parser, &context, text, true);
    }
    let partitive_head = |text| {
        let predicate = imperative_transitive(&parser, &context, text);
        let Object::ObjectNominal(nominal) = predicate.object else {
            panic!("partitive probe keeps its nominal object: {text:?}")
        };
        let NounPhrase::QualifiedNounPhrase(qualified) = nominal.value() else {
            panic!("partitive probe enters the ordinary qualification stages: {text:?}")
        };
        let PostmodifiedReference::UnqualifiedPostmodifiedReference(reference) =
            qualified.reference.as_ref()
        else {
            panic!("partitive probe has no postmodifier: {text:?}")
        };
        let UnqualifiedReference::DeterminativePartitive(partitive) = reference.reference.as_ref()
        else {
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
fn participial_relatives_select_low_adjunct_attachment() {
    let parser = parser();
    let context = context();

    for (text, inner) in [
        (
            "Destroy a card you've exiled this turn.",
            "ContractedPerfectAdjunctObjectGapRelativeClauseContractedPerfectObjectGapRelativeWithAdjunct",
        ),
        (
            "Destroy each creature turned face up this turn.",
            "PostmodifiedReferenceReducedPassiveAdjunctQualifiedReference",
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .unwrap_or_else(|| panic!("{text:?} must retain a decision: {analysis:?}"));
        let ordinal = decision
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {decision:?}"));
        let path = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == ordinal)
            .expect("the selected ordinal names a candidate")
            .construction_path();
        assert!(
            path.iter().any(|name| name == inner),
            "the low participial-relative attachment must win for {text:?}: {path:?}",
        );
    }
}

#[test]
fn participial_relative_adjunct_minimal_pairs_select() {
    let parser = parser();
    let context = context();

    for text in [
        "Draw a card for each card you've exiled this turn.",
        "Draw a card for each card you've revealed this turn.",
        "Draw a card for each card you've discarded this turn.",
    ] {
        assert_selected(&parser, &context, text);
    }
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
fn negative_nominal_quantifiers_preserve_singular_and_plural_concord_class() {
    let parser = parser();
    let context = context();

    for text in ["There is no card.", "There are no cards."] {
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
fn all_plus_nominal_is_a_transitive_object_not_an_object_predicative_split() {
    let parser = parser();
    let context = context();
    let text = "Exile all permanents.";

    assert_selected(&parser, &context, text);
    let analysis = parser.analyze(text, &context);
    let decision = analysis.decision().expect("the exact witness is selected");
    let selected = decision
        .selected()
        .expect("the exact witness has an ordinal");
    let path = decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == selected)
        .expect("the selected ordinal names a candidate")
        .construction_path();
    assert!(
        path.iter()
            .any(|name| name == "TransitiveLexicalVerbPhraseTransitivePredicate"),
        "{path:?}",
    );
    assert!(
        path.iter()
            .all(|name| name != "VerbPhraseDeclaredObjectPredicativeVerbPhrase"),
        "{path:?}",
    );
}

#[test]
fn all_predeterminer_keeps_its_following_determined_nominal() {
    let parser = parser();
    let context = context();
    let text = "Exile all the cards from your hand.";

    assert_selected(&parser, &context, text);
    let analysis = parser.analyze(text, &context);
    let decision = analysis.decision().expect("the exact witness is selected");
    let selected = decision
        .selected()
        .expect("the exact witness has an ordinal");
    let path = decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == selected)
        .expect("the selected ordinal names a candidate")
        .construction_path();
    assert!(
        path.iter()
            .any(|name| name == "UnqualifiedReferenceAllPredeterminedNominal"),
        "{path:?}",
    );
    assert!(
        path.iter()
            .all(|name| name != "VerbPhraseDeclaredObjectPredicativeVerbPhrase"),
        "{path:?}",
    );
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
fn target_remains_productive_as_a_determiner_and_a_nominal_modifier() {
    let parser = parser();
    let context = context();

    assert_selected_with_specificity(
        &parser,
        &context,
        "This creature deals 1 damage to any target.",
        true,
    );
    assert_selected_with_specificity(
        &parser,
        &context,
        "This creature deals 1 damage to target creature.",
        true,
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
        "This creature is equip.",
        "Creatures isn't blocked.",
        "This creature aren't blocked.",
        "Whenever they attack and isn't blocked, draw a card.",
        "As long as this creature equipped, it untaps.",
        "They doesn't untap.",
        "That creature don't untap.",
        "That creature doesn't untaps.",
    ] {
        assert!(
            parser.parse(malformed, &context).is_err(),
            "malformed finite passive or negative auxiliary must reject {malformed:?}",
        );
    }
    assert_selected_with_specificity(&parser, &context, "This spell was cast a graveyard.", true);
    assert_selected_with_specificity(
        &parser,
        &context,
        "That creature doesn't untap its controller's next untap step.",
        true,
    );
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
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    for malformed in [
        "Destroy target activate ability.",
        "Discard up to two card.",
        "Destroy each X target creatures.",
        "Destroy each of X target creature.",
        "Discard up to one cards.",
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
    assert!(
        parser
            .parse("Draw a card after your library.", &context)
            .is_err(),
        "after cannot take an ordinary library object complement",
    );
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
    assert!(
        parser
            .parse("Context Card color is red.", &context)
            .is_err()
    );
}

#[test]
fn generic_complements_preserve_surface_contrasts_without_topic_products() {
    let parser = parser();
    let context = context();
    for text in [
        "Deal X damage to target creature.",
        "Deal damage equal to its power to target creature.",
        "Gain that much life.",
        "Gain life equal to its power.",
        "Lose 2 life.",
        "Lose life equal to its toughness.",
        "Pay X life.",
        "Pay {2}{B}.",
        "Add {B}{B}{B}.",
        "Add {B}.",
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
        "Put X time counters on target creature.",
        "Put that many charge counters on target creature.",
        "Remove X time counters from this card.",
        "Sacrifice a creature of their choice.",
        "Discard two cards at random.",
        "Each player sacrifices a creature of their choice.",
        "Target player discards two cards at random.",
        "Pay {2}, draw cards equal to its toughness, and put two stun counters on target creature.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .expect("selected generic complement has a decision");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("selected ordinal names a candidate");
        assert!(
            selected.construction_path().iter().all(|name| {
                ![
                    "DealAmountDamage",
                    "DealDamageEqualTo",
                    "LifeAmount",
                    "LifeEquality",
                    "DrawCards",
                    "RollDice",
                    "PutCounters",
                    "RemoveCounters",
                    "CardQuantity",
                    "DieObject",
                    "CounterQuantity",
                ]
                .iter()
                .any(|obsolete| name.contains(obsolete))
            }),
            "shared frames must not retain a topical complement product: {text:?}: {:?}",
            selected.construction_path(),
        );
    }
}

#[test]
fn generic_complements_keep_lexical_and_structural_claims_distinct() {
    let parser = parser();
    let context = context();
    let text =
        "Pay {2}, draw cards equal to its toughness, and put two stun counters on target creature.";
    let claims = exact_claim_trace(&parser, &context, text);
    assert!(claims.iter().any(|(surface, owner)| {
        surface == " {" && owner == "form:symbol_run/symbol_run/0/prefix"
    }));
    assert!(claims.iter().any(|(surface, owner)| {
        surface == ", and "
            && owner == "structural:AndPredicateCoordination/members/separator/last/0"
    }));
    assert_eq!(
        claims
            .last()
            .map(|(surface, owner)| (surface.as_str(), owner.as_str())),
        Some((".", "structural:Sentences/sentences/terminator/0")),
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
    assert_selected_with_specificity(
        &parser,
        &context,
        "Put two stun counters on target creatures.",
        true,
    );
    assert_selected_with_specificity(
        &parser,
        &context,
        "Put two stun counters on the target of Context Card.",
        true,
    );
    let malformed = "Put two stun counters on two target creature.";
    assert!(
        parser.parse(malformed, &context).is_err(),
        "target determiners and modifiers preserve ordinary number concord_class: {malformed:?}",
    );

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
        "singular shortened self-reference requires singular verb concord_class",
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
fn typed_complements_reject_reciprocal_concord_class_amount_number_and_determiners() {
    let parser = parser();
    let context = context();
    for text in [
        "Deal 2 damages to any target.",
        "It deal 2 damage to any target.",
        "Gain 2 lives.",
        "You gains 2 life.",
        "Pay 2 lives.",
        "Pay 2.",
        "Pay {2} mana.",
        "Add B.",
        "Draw two card.",
        "Draw card.",
        "Draw cards equals to its power.",
        "Draw cards equal its power.",
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
        "Draw 2 cards.",
        "Destroy two target creature.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "malformed reciprocal must reject {text:?}",
        );
    }
    assert_selected_with_specificity(&parser, &context, "Deal two damage to any target.", true);
    assert_selected_with_specificity(&parser, &context, "Gain two life.", true);
    assert_selected_with_specificity(&parser, &context, "Lose two life.", true);
    assert_selected_with_specificity(&parser, &context, "Draw two cards.", true);
    assert_selected_with_specificity(&parser, &context, "Roll one die.", true);
    assert_selected_with_specificity(&parser, &context, "Destroy target creatures.", true);
    assert_selected_with_specificity(
        &parser,
        &context,
        "Destroy the target of Context Card.",
        true,
    );
}

#[test]
fn movement_frames_select_exact_source_destination_state_and_control_roles() {
    let parser = parser();
    let context = context();
    for (text, permits_specificity) in [
        ("Put that card into your hand.", true),
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
fn declared_frame_complement_pairs_coordinate_for_every_coordinator() {
    let parser = parser();
    let context = context();

    for (text, coordination) in [
        (
            "Deal 1 damage to target creature and 1 damage to you.",
            "VerbPhraseAndFrameComplementPairCoordination",
        ),
        (
            "Deal 1 damage to target creature or 1 damage to you.",
            "VerbPhraseOrFrameComplementPairCoordination",
        ),
        (
            "Deal 1 damage to target creature and/or 1 damage to you.",
            "VerbPhraseAndOrFrameComplementPairCoordination",
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let selected = analysis
            .selected()
            .unwrap_or_else(|| panic!("declared pair must select {text:?}: {analysis:#?}"));
        let decision = analysis.decision().expect("selected pair has a decision");
        assert_eq!(decision.survivors().len(), 1, "{text:?}: {decision:#?}");
        let candidate = &decision.candidates()[decision
            .selected()
            .expect("selected decision identifies its candidate")];
        assert!(
            candidate
                .construction_path()
                .iter()
                .any(|construction| construction == coordination),
            "the declared pair uses its positional coordinator arm: {text:?}: {candidate:#?}",
        );
        assert_eq!(
            candidate
                .construction_path()
                .iter()
                .filter(|construction| *construction == "FrameComplementPairFrameComplementPair")
                .count(),
            2,
            "each Conjunct is one object-plus-marked-complement tuple: {text:?}: {candidate:#?}",
        );
        assert_eq!(selected.render(&context, parser.environment()), text);
    }

    let mismatched = parser.analyze(
        "Deal 1 damage to target creature and 1 damage on you.",
        &context,
    );
    let decision = mismatched
        .decision()
        .expect("the sentence retains its independently licensed analysis");
    assert!(
        decision.candidates().iter().all(|candidate| candidate
            .construction_path()
            .iter()
            .all(|construction| !construction.contains("FrameComplementPairCoordination"))),
        "Conjuncts with different markers do not form a declared frame-complement pair Coordination: {mismatched:#?}",
    );
}

#[test]
fn movement_control_role_rejects_an_undeclared_marker() {
    let parser = parser();
    let context = context();

    for text in [
        "Put target creature card onto the battlefield onto the battlefield.",
        "Put target creature card onto the battlefield into your graveyard.",
        "Return target creature card to your hand into your graveyard.",
    ] {
        assert!(parser.parse(text, &context).is_err(), "reject {text:?}");
    }
}

#[test]
fn declared_optional_source_preempts_the_same_noun_postmodifier_derivation() {
    let parser = parser();
    let context = context();

    for text in [
        "Return target creature card from your graveyard to your hand.",
        "Return target creature that controls a creature from your graveyard to your hand.",
        "Return target creature card to your hand.",
    ] {
        let analysis = parser.analyze(text, &context);
        assert!(analysis.selected().is_some(), "{text:?}: {analysis:#?}");
        let decision = analysis.decision().expect("selected parse has a decision");
        assert_eq!(decision.candidates().len(), 1, "{text:?}: {decision:#?}");
        assert_eq!(
            decision.resolution(),
            SelectionResolution::Unique,
            "{text:?}"
        );
        let selected = &decision.candidates()[0];
        assert!(
            selected
                .construction_path()
                .iter()
                .any(|construction| construction == "VerbPhraseReturnTo"),
            "the selected source belongs to the Verb Frame: {text:?}: {selected:#?}",
        );
        if text.contains(" from ") {
            assert!(
                selected
                    .construction_path()
                    .iter()
                    .all(|construction| !construction.ends_with("PrepositionalQualifiedReference")),
                "the declared source must not remain a nominal Postmodifier: {text:?}: {selected:#?}",
            );
        }
    }

    let noun_modified = parser.analyze(
        "Destroy target creature card from your graveyard.",
        &context,
    );
    assert!(noun_modified.selected().is_some(), "{noun_modified:#?}");
    let decision = noun_modified
        .decision()
        .expect("selected noun-postmodifier parse has a decision");
    assert_eq!(decision.candidates().len(), 1, "{decision:#?}");
    assert!(
        decision.candidates()[0]
            .construction_path()
            .iter()
            .any(|construction| construction.ends_with("PrepositionalQualifiedReference"))
    );
}

#[test]
fn role_preemption_reaches_only_the_right_periphery_of_the_governed_material() {
    let parser = parser();
    let context = context();

    let selected_path = |text: &str| {
        let analysis = parser.analyze(text, &context);
        assert!(analysis.selected().is_some(), "{text:?}: {analysis:#?}");
        let decision = analysis.decision().expect("selected parse has a decision");
        decision.candidates()[decision
            .selected()
            .expect("selected decision identifies its candidate")]
        .construction_path()
        .to_vec()
    };
    let postmodifiers = |path: &[String]| {
        path.iter()
            .enumerate()
            .filter(|(_, construction)| construction.ends_with("PrepositionalQualifiedReference"))
            .map(|(index, _)| index)
            .collect::<Vec<_>>()
    };

    // The last Conjunct is right-peripheral to the object, so its `from` fills
    // Return's declared source role instead of postmodifying that Conjunct.
    let preempted = selected_path(
        "Return target creature card and target land card from your graveyard to your hand.",
    );
    assert!(
        postmodifiers(&preempted).is_empty(),
        "a right-peripheral declared source is not a Postmodifier: {preempted:#?}",
    );

    // A non-final Conjunct is not right-peripheral, so its `from` keeps its
    // Postmodifier derivation and both targets stay inside the object.
    let spared = selected_path(
        "Return target creature card from a graveyard and target creature on the battlefield to their owners' hands.",
    );
    let coordination = spared
        .iter()
        .position(|construction| construction.ends_with("AndNounPhraseCoordination"))
        .expect("the object coordinates its two targets");
    let spared_postmodifiers = postmodifiers(&spared);
    assert_eq!(
        spared_postmodifiers.len(),
        2,
        "each Conjunct keeps its own Postmodifier: {spared:#?}",
    );
    assert!(
        spared_postmodifiers
            .iter()
            .all(|index| *index > coordination),
        "both Postmodifiers sit inside the object coordination: {spared:#?}",
    );
}

#[test]
fn first_eligible_frame_role_preserves_later_same_preposition_postmodifier() {
    let parser = parser();
    let context = context();

    for (text, objects_before_modifier) in [
        (
            "Put a spore counter on each creature on the battlefield.",
            2,
        ),
        (
            "Put a +1/+1 counter on each creature that has a +1/+1 counter on it.",
            3,
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        assert!(analysis.selected().is_some(), "{text:?}: {analysis:#?}");
        let decision = analysis.decision().expect("selected parse has a decision");
        let selected = &decision.candidates()[decision
            .selected()
            .expect("selected decision identifies its candidate")];
        let put_on = selected
            .construction_path()
            .iter()
            .position(|construction| construction == "VerbPhrasePutOn")
            .expect("the selected analysis uses the declared frame");
        let governed = &selected.construction_path()[put_on + 1..];
        let modifier = governed
            .iter()
            .position(|construction| construction.ends_with("PrepositionalQualifiedReference"))
            .expect("the later phrase remains a Postmodifier");
        assert_eq!(
            governed[..modifier]
                .iter()
                .filter(|construction| *construction == "ObjectObjectNominal")
                .count(),
            objects_before_modifier,
            "the first role marker must fill the frame before the later Postmodifier: {text:?}: {selected:#?}",
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
fn movement_location_and_control_frames_reject_reciprocal_heads_prepositions_and_tails() {
    let parser = parser();
    let context = context();
    for text in [
        "Return that card into your hand.",
        "Control that card to your hand.",
        "Put that card from your hand.",
        "Put that card to tapped.",
        "Remove a time counter to this card.",
        "This creature leaves to the battlefield.",
        "This creature enters from your graveyard.",
        "Look into the top two cards of your library.",
        "Reveal at the top card of your library.",
        "You have one or fewer card in hand.",
        "You have three or fewer cards on hand.",
        "Put that card under your control onto the battlefield.",
        "Search your library a creature card.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "wrong head, preposition, or tail must reject {text:?}",
        );
    }
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

    fn visit_ordered_predicate_value(&mut self, value: &OrderedPredicateValue) {
        self.0.push("ordered");
        deckmaste_english_v2::visit::walk_ordered_predicate_value(self, value);
    }

    fn visit_purpose_predicate_adjunct(&mut self, value: &PurposePredicateAdjunct) {
        self.0.push("purpose");
        deckmaste_english_v2::visit::walk_purpose_predicate_adjunct(self, value);
    }

    fn visit_duration_predicate_adjunct(&mut self, value: &DurationPredicateAdjunct) {
        self.0.push("duration");
        deckmaste_english_v2::visit::walk_duration_predicate_adjunct(self, value);
    }

    fn visit_instead_predicate_value(&mut self, value: &InsteadPredicateValue) {
        self.0.push("instead");
        deckmaste_english_v2::visit::walk_instead_predicate_value(self, value);
    }

    fn visit_manner_predicate_adjunct(&mut self, value: &MannerPredicateAdjunct) {
        self.0.push("manner");
        deckmaste_english_v2::visit::walk_manner_predicate_adjunct(self, value);
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
            &["as-though"][..],
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
fn as_though_uses_the_general_finite_clause_family() {
    let parser = parser();
    let context = context();
    for text in [
        "It can block as though it didn't have hexproof.",
        "You can cast spells as though they had flash.",
        "Tapped creatures you control can block as though they were untapped.",
    ] {
        let ability = assert_selected(&parser, &context, text);
        let mut visitor = AdjunctVisitor::default();
        visitor.visit_ability(&ability);
        assert_eq!(visitor.0, ["as-though"], "{text:?}");
        let analysis = parser.analyze(text, &context);
        let decision = analysis.decision().expect("counterfactual has a decision");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("selected ordinal names a candidate");
        assert!(
            selected.construction_path().iter().all(|name| {
                !name.contains("CounterfactualFiniteClause")
                    && !name.contains("CounterfactualStatusClause")
                    && !name.contains("CounterfactualNegativeAbilityClause")
                    && !name.contains("CounterfactualPastAbilityClause")
            }),
            "{text:?}: {:?}",
            selected.construction_path(),
        );
    }
    for crossed in [
        "It can block as though it didn't has hexproof.",
        "It can block as though it did have hexproof.",
        "You can cast spells as though they has flash.",
        "Tapped creatures you control can block as though they was untapped.",
    ] {
        assert!(parser.parse(crossed, &context).is_err(), "{crossed:?}");
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
fn this_way_keeps_a_specific_manner_reading_beside_downstream_semantic_rivals() {
    let parser = parser();
    let context = context();
    let text = "You didn't create a token this way.";
    let ability = assert_selected(&parser, &context, text);
    let mut visitor = AdjunctVisitor::default();
    visitor.visit_ability(&ability);
    assert_eq!(visitor.0, ["manner"]);

    let text = "Attack this way.";
    let analysis = parser.analyze(text, &context);
    assert_eq!(analysis.outcome(), ParseAnalysisOutcome::Selected);
    let decision = analysis.decision().expect("syntactic rivals are retained");
    assert_eq!(decision.candidates().len(), 2);
    assert_eq!(decision.resolution(), SelectionResolution::Specificity);
    assert!(decision.exception_uses().is_empty());
    assert!(decision.candidates().iter().any(|candidate| {
        candidate
            .construction_path()
            .iter()
            .any(|item| item == "PredicateAdjunctMannerPredicateAdjunct")
    }));
    assert!(!decision.candidates().iter().any(|candidate| {
        candidate
            .construction_path()
            .iter()
            .any(|item| item == "PredicateAdjunctDurationPredicateAdjunct")
    }));
    let selected = decision
        .candidates()
        .iter()
        .find(|candidate| Some(candidate.ordinal()) == decision.selected())
        .expect("specificity retains one syntactic reading");
    assert!(
        selected
            .construction_path()
            .iter()
            .any(|item| item == "TransitiveLexicalVerbPhraseTransitivePredicate")
    );
}

#[derive(Default)]
struct AdjunctSurfaceVisitor(Vec<String>);

impl Visitor for AdjunctSurfaceVisitor {
    fn visit_verb_inventory(&mut self, value: &VerbInventoryRef) {
        match value {
            VerbInventoryRef::Core(CoreVerbIdentity::May) => {
                self.0.push("auxiliary:May".to_owned());
            }
            VerbInventoryRef::Core(CoreVerbIdentity::Cant) => {
                self.0.push("auxiliary:Cant".to_owned());
            }
            _ => {}
        }
    }

    fn visit_duration_predicate_adjunct(&mut self, value: &DurationPredicateAdjunct) {
        self.0.push("duration".to_owned());
        deckmaste_english_v2::visit::walk_duration_predicate_adjunct(self, value);
    }

    fn visit_at_random_manner(&mut self, value: &AtRandomManner) {
        self.0.push("at-random-manner".to_owned());
        deckmaste_english_v2::visit::walk_at_random_manner(self, value);
    }

    fn visit_arbitrary_determiner(&mut self, value: ArbitraryDeterminer) {
        self.0.push(format!("order:{value:?}"));
    }

    fn visit_unless_clause_tail(&mut self, value: &UnlessClauseTail) {
        self.0.push("exception:unless".to_owned());
        deckmaste_english_v2::visit::walk_unless_clause_tail(self, value);
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
    let purpose = parser.analyze("To draw a card, discard a card.", &context);
    let decision = purpose
        .decision()
        .expect("the preposed purpose adjunct has a selection decision");
    let selected = &decision.candidates()[decision
        .selected()
        .expect("the preposed purpose adjunct selects")];
    assert!(
        selected
            .construction_path()
            .iter()
            .any(|item| item == "ClauseAttachmentPreposedPredicateClauseTail")
    );
    assert!(
        selected
            .construction_path()
            .iter()
            .any(|item| item == "PredicateAdjunctClauseTailPredicateAdjunctClauseTail")
    );

    for text in [
        "You may this turn cast it.",
        "Instead draw a card.",
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

    let object_internal_move = "You didn't create this way a token.";
    let analysis = parser.analyze(object_internal_move, &context);
    assert_eq!(analysis.outcome(), ParseAnalysisOutcome::ParseFailure);
    assert!(
        analysis.selected().is_none(),
        "an object-internal manner surface has no duration rescue: {analysis:?}"
    );
    assert!(parser.parse(object_internal_move, &context).is_err());
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
            (
                " a".to_owned(),
                "determinative:DeterminativeHead/IndefiniteArticle".to_owned(),
            ),
            (
                " card".to_owned(),
                "lexeme:CommonNoun/Card/singular".to_owned(),
            ),
            (
                " instead".to_owned(),
                "vocab:ReplacementMarker/Instead".to_owned(),
            ),
            (": ".to_owned(), "form:activated/activated/1".to_owned(),),
            ("Draw".to_owned(), "core-verb:Draw".to_owned(),),
            (
                " a".to_owned(),
                "determinative:DeterminativeHead/IndefiniteArticle".to_owned(),
            ),
            (
                " card".to_owned(),
                "lexeme:CommonNoun/Card/singular".to_owned(),
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
            "cost position enforces bare concord_class and document-internal case: {crossed:?}",
        );
    }
}

#[test]
fn object_internal_discarded_this_way_remains_a_downstream_semantic_decision() {
    let parser = parser();
    let context = context();
    let text = "Target player discards three cards. Put up to one artifact card discarded this way onto the battlefield tapped under your control.";
    let analysis = parser.analyze(text, &context);
    assert_eq!(analysis.outcome(), ParseAnalysisOutcome::Selected);
    let decision = analysis
        .decision()
        .expect("the syntactic reading is explicit");
    assert_eq!(decision.resolution(), SelectionResolution::Unique);
    assert!(decision.exception_uses().is_empty());
    let selected = decision
        .candidates()
        .iter()
        .find(|candidate| Some(candidate.ordinal()) == decision.selected())
        .expect("specificity retains one syntactic reading");
    assert!(
        selected
            .construction_path()
            .iter()
            .any(|item| item == "PostmodifiedReferenceReducedPassiveQualifiedReference")
    );
    assert!(
        selected
            .construction_path()
            .iter()
            .any(|item| item == "DeclaredObjectPassivePredicateDeclaredObjectPassivePredicate")
    );
    assert!(
        !selected
            .construction_path()
            .iter()
            .any(|item| item == "PredicateAdjunctMannerPredicateAdjunct")
    );
    let parsed = analysis.selected().expect("selected syntax has an AST");
    assert_eq!(parsed.render(&context, parser.environment()), text);
    assert!(
        analysis
            .ownership()
            .expect("selected syntax owns its bytes")
            .failures()
            .is_empty()
    );
}

#[test]
fn passive_distribution_and_counter_frames_use_shared_products() {
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
        let analysis = parser.analyze(text, &context);
        let decision = analysis.decision().expect("shared frame has a decision");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("selected ordinal names a candidate");
        assert!(
            selected.construction_path().iter().all(|name| {
                !name.contains("DealDistributedDamage")
                    && !name.contains("PutCounters")
                    && !name.contains("RemoveCounters")
                    && !name.contains("CounterQuantity")
            }),
            "{text:?}: {:?}",
            selected.construction_path(),
        );
    }
}

#[test]
fn distribution_preserves_target_noun_boundaries() {
    let parser = parser();
    let context = context();

    assert!(
        parser
            .parse(
                "It deals 2 damage divided as you choose among one or two targets.",
                &context,
            )
            .is_err(),
        "distribution keeps bare target nouns outside the productive target modifier",
    );
    assert_selected_with_specificity(
        &parser,
        &context,
        "It deals X damage divided evenly, rounded down, among any number of targets.",
        true,
    );
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
    for text in ["You gain twice X life.", "You gain X plus 3 life."] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
    let accepted = ["Target creature can't be legendary this turn."]
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
        "Cast only this spell if you control a snow land.",
        "Destroy this spell only during your turn.",
        "Activate this ability only if you control a snow land.",
        "Draw a card only if you control a snow land.",
        "Draw only a card.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }
}

#[test]
fn qualified_frequency_adjuncts_compose_with_temporal_and_focus_syntax() {
    let parser = parser();
    let context = context();

    for text in [
        "Draw a card once each turn.",
        "Draw a card twice each turn.",
        "Draw a card once during each of your turns.",
        "Activate only once each turn.",
    ] {
        let analysis = parser.analyze(text, &context);
        let decision = analysis
            .decision()
            .unwrap_or_else(|| panic!("{text:?} must have a selection decision: {analysis:#?}"));
        let selected = decision
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {decision:#?}"));
        let path = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == selected)
            .expect("the selected ordinal names a candidate")
            .construction_path();
        assert!(
            path.iter()
                .any(|item| item == "PredicateAdjunctQualifiedFrequencyPredicateAdjunct"),
            "{text:?} must use the qualified frequency member: {path:#?}",
        );
    }
}

#[test]
fn cost_frame_reciprocals_reject_crossed_boundaries() {
    let parser = parser();
    let context = context();

    let unfocused = parser.analyze("Cast this spell your turn.", &context);
    let focused = parser.analyze("Cast this spell only your turn.", &context);
    assert!(
        unfocused.selected().is_some(),
        "the pre-existing bare-duration defect changed: {unfocused:#?}",
    );
    assert_eq!(
        focused.selected().is_some(),
        unfocused.selected().is_some(),
        "focus must preserve the host's duration decision: focused={focused:#?}, unfocused={unfocused:#?}",
    );

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
        "Cast this spell only only during your turn.",
    ] {
        let analysis = parser.analyze(text, &context);
        assert!(
            analysis.selected().is_none(),
            "crossed cost frame must reject {text:?}: {analysis:#?}",
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
        "Spells cost {} less to cast.",
        "Spells cost {1 less to cast.",
        "Spells cost one less to cast.",
        "Spells cost {1} more less to cast.",
        "Spells cost {1} less to cast for for each creature you control.",
        "Cast this spell if you control a snow land only.",
        "Cast this spell only only if you control a snow land.",
        "Activate only as a sorcery.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "cost scope must reject {text:?}",
        );
    }
    assert_selected_with_specificity(
        &parser,
        &context,
        "You may cast spells without paying mana costs.",
        true,
    );
}

#[test]
fn cost_surfaces_keep_shared_frames_and_lexical_ownership() {
    let parser = parser();
    let context = context();
    for text in [
        "As an additional cost to cast this spell, discard a card.",
        "You may sacrifice a Mountain rather than pay this spell's mana cost.",
        "You may cast spells from your hand without paying their mana costs.",
        "Spells cost {1} less to cast.",
        "This ability costs {1} less to activate for each legendary creature you control.",
        "Cast this spell only during your turn and only if you control a snow land.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
        let analysis = parser.analyze(text, &context);
        let decision = analysis.decision().expect("cost surface has a decision");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("selected ordinal names a candidate");
        assert!(
            selected.construction_path().iter().all(|name| {
                !name.contains("RatherThanManaCostPredicate")
                    && !name.contains("WithoutPayingManaCostPredicate")
            }),
            "{text:?}: {:?}",
            selected.construction_path(),
        );
    }
    let claims = exact_claim_trace(&parser, &context, "Spells cost {1} less to cast.");
    assert!(
        claims
            .iter()
            .any(|(surface, owner)| { surface == " cost" && owner == "core-verb:Cost" })
    );
    assert_eq!(
        claims
            .last()
            .map(|(surface, owner)| (surface.as_str(), owner.as_str())),
        Some((".", "structural:Sentences/sentences/terminator/0")),
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
        matches!(body, AbilityBody::Sentences(_)),
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
    let AbilityBody::Sentences(sentences) = body else {
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
    let AbilityBody::Sentences(sentences) = body else {
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
    let AbilityBody::ThenSentences(sentence_sequence) = body else {
        panic!("sentence-initial Then has a distinct structural AST")
    };
    assert_eq!(sentence_sequence.members().len(), 2);
    assert!(ThenSentenceSequence::new(Box::new(sentence_sequence.members().to_vec())).is_some());
    assert!(
        ThenSentenceSequence::new(Box::new(vec![sentence_sequence.members()[0].clone()])).is_none()
    );

    let mut visitor = ThenVisitor::default();
    visitor.visit_then_sentence_sequence(&sentence_sequence);
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

#[test]
fn restored_general_construction_probes_select_uniquely() {
    let parser = parser();
    let context = context();

    for text in [
        "Destroy all white, blue, black, and red creatures.",
        "You gain less than 3 life.",
        "Add {2/W}.",
        "It is able to attack.",
        "Put all creatures on the bottom of their owners' libraries.",
    ] {
        assert_selected(&parser, &context, text);
    }
}

#[test]
fn preposition_attachment_and_complement_head_licenses_are_conjunctive() {
    let parser = parser();
    let context = context();

    for text in [
        "Draw a card at the beginning of your end step.",
        "Creatures you control get +1/+1 on your turn.",
        "Destroy creatures you control of the chosen type.",
        "Sacrifice a nontoken creature of their choice.",
        "Remove X counters from among them.",
        "Put two counters on this artifact.",
        "Destroy cards in your graveyard.",
        "Destroy target creature on the battlefield.",
        "Destroy each creature on the battlefield.",
        "Reveal the top card of your library.",
        "Destroy a copy of target creature.",
        "You gain life equal to that creature's power.",
        "You gain life equal to that card's mana value.",
        "Sacrifice a creature during your upkeep.",
        "Draw a card for each creature you control.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    let m3 = parser.analyze("Destroy each creature on the battlefield.", &context);
    let decision = m3.decision().expect("M3 witness has a selection decision");
    let selected = decision.selected().expect("M3 witness selects one reading");
    assert!(
        decision.candidates()[selected]
            .construction_path()
            .iter()
            .any(|construction| construction
                == "PostmodifiedReferencePrepositionalQualifiedReference"),
        "the licensed PP must attach inside the object noun phrase"
    );
    assert!(
        !decision.candidates()[selected]
            .construction_path()
            .iter()
            .any(|construction| {
                construction == "PredicateAdjunctPrepositionalPredicateAdjunct"
            }),
        "the predicate-adjunct rival must lose"
    );

    for text in [
        "Into your graveyard, draw a card.",
        "Of your library, draw a card.",
        "To target player, draw a card.",
        "Under your control, draw a card.",
        "Among them, draw a card.",
        "Draw a card of target player.",
        "Draw a card of a Goblin.",
        "Destroy target creature on an artifact.",
        "Destroy target creature on target player.",
        "Draw a card of your library.",
        "Sacrifice a creature of your hand.",
        "You gain 2 life of your library.",
        "Destroy target creature on your hand.",
        "There is a creature into your graveyard.",
        "Draw a card for from your graveyard.",
    ] {
        assert_eq!(
            parser.analyze(text, &context).outcome(),
            ParseAnalysisOutcome::ParseFailure,
            "preposition data must reject {text:?}",
        );
    }
}

/// The predicative color complement takes the whole color-property
/// vocabulary, including `colorless` ([CR#105.2c]) beside the five colors and
/// the two cardinality adjectives.
#[test]
fn predicative_color_complements_accept_the_colorless_property() {
    let parser = parser();
    let context = context();

    for text in [
        "This permanent is colorless.",
        "Target creature becomes colorless.",
        "This permanent is monocolored.",
    ] {
        assert_selected_with_specificity(&parser, &context, text, true);
    }

    assert!(
        parser
            .parse("This permanent is colored.", &context)
            .is_err(),
        "the color-property vocabulary has no *colored* member",
    );
}
