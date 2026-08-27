use std::cmp::Ordering;
use std::path::Path;

use deckmaste_english_v2::ast::BareLocativeNoun;
use deckmaste_english_v2::ast::CardName;
use deckmaste_english_v2::ast::CardinalNumber;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::Color;
use deckmaste_english_v2::ast::CommonNoun;
use deckmaste_english_v2::ast::ControllerNoun;
use deckmaste_english_v2::ast::ObjectPronoun;
use deckmaste_english_v2::ast::PossessiveAbsolutePronoun;
use deckmaste_english_v2::ast::PossessiveDeterminerPronoun;
use deckmaste_english_v2::ast::ReflexivePronoun;
use deckmaste_english_v2::ast::ScalarCharacteristic;
use deckmaste_english_v2::ast::ScalarNumber;
use deckmaste_english_v2::ast::SelfReferenceSpelling;
use deckmaste_english_v2::ast::Status;
use deckmaste_english_v2::ast::SubjectPronoun;
use deckmaste_english_v2::ast::Supertype;
use deckmaste_english_v2::ast::Variable;
use deckmaste_english_v2::ast::VerbLexeme;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::parser::LexicalProvenanceKind;
use deckmaste_english_v2::parser::ParseAnalysisOutcome;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionDecisive;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::parser::SpecificityTier;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;
use macro_ron::v2::Onset;

fn parser() -> Parser {
    let declarations = macro_ron::v2::read_builtin_v2(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin_v2"),
    )
    .expect("integrated builtin-v2 declarations load");
    let environment = ParserEnvironment::try_from_parts(
        declarations,
        [CatalogProviderRows::new(
            CatalogProvider::CardNames,
            [CatalogProviderRow::new(
                "magnifying-glass",
                "Magnifying Glass",
                Onset::Consonant,
            )],
        )],
    )
    .expect("nominal test environment freezes");
    Parser::new(environment).expect("required grammar declarations are present")
}

fn context(name: &str) -> ParseContext<'_> {
    ParseContext::new(name, false, Onset::Consonant)
        .expect("test card name is a valid parse context")
}

#[derive(Default)]
struct NominalVisitor {
    events: Vec<String>,
}

macro_rules! record_product {
    ($visit:ident, $type:ident, $walk:ident) => {
        fn $visit(&mut self, value: &deckmaste_english_v2::ast::$type) {
            self.events.push(stringify!($type).to_owned());
            deckmaste_english_v2::visit::$walk(self, value);
        }
    };
}

impl Visitor for NominalVisitor {
    fn visit_ability(&mut self, value: &deckmaste_english_v2::ast::Ability) {
        self.events.push("Ability".to_owned());
        deckmaste_english_v2::visit::walk_ability(self, value);
    }

    fn visit_sentence(&mut self, value: &deckmaste_english_v2::ast::Sentence) {
        self.events.push("Sentence".to_owned());
        deckmaste_english_v2::visit::walk_sentence(self, value);
    }

    fn visit_verb_phrase(&mut self, value: &deckmaste_english_v2::ast::VerbPhrase) {
        self.events.push("VerbPhrase".to_owned());
        deckmaste_english_v2::visit::walk_verb_phrase(self, value);
    }

    fn visit_subject(&mut self, value: &deckmaste_english_v2::ast::Subject) {
        self.events.push("Subject".to_owned());
        deckmaste_english_v2::visit::walk_subject(self, value);
    }

    fn visit_object(&mut self, value: &deckmaste_english_v2::ast::Object) {
        self.events.push("Object".to_owned());
        deckmaste_english_v2::visit::walk_object(self, value);
    }

    fn visit_noun_phrase(&mut self, value: &deckmaste_english_v2::ast::NounPhrase) {
        self.events.push("NounPhrase".to_owned());
        deckmaste_english_v2::visit::walk_noun_phrase(self, value);
    }

    fn visit_singular_selector(&mut self, value: &deckmaste_english_v2::ast::SingularSelector) {
        self.events.push("SingularSelector".to_owned());
        deckmaste_english_v2::visit::walk_singular_selector(self, value);
    }

    fn visit_plural_selector(&mut self, value: &deckmaste_english_v2::ast::PluralSelector) {
        self.events.push("PluralSelector".to_owned());
        deckmaste_english_v2::visit::walk_plural_selector(self, value);
    }

    fn visit_singular_nominal(&mut self, value: &deckmaste_english_v2::ast::SingularNominal) {
        self.events.push("SingularNominal".to_owned());
        deckmaste_english_v2::visit::walk_singular_nominal(self, value);
    }

    fn visit_plural_nominal(&mut self, value: &deckmaste_english_v2::ast::PluralNominal) {
        self.events.push("PluralNominal".to_owned());
        deckmaste_english_v2::visit::walk_plural_nominal(self, value);
    }

    fn visit_nominal_modifier(&mut self, value: &deckmaste_english_v2::ast::NominalModifier) {
        self.events.push("NominalModifier".to_owned());
        deckmaste_english_v2::visit::walk_nominal_modifier(self, value);
    }

    fn visit_singular_head(&mut self, value: &deckmaste_english_v2::ast::SingularHead) {
        self.events.push("SingularHead".to_owned());
        deckmaste_english_v2::visit::walk_singular_head(self, value);
    }

    fn visit_plural_head(&mut self, value: &deckmaste_english_v2::ast::PluralHead) {
        self.events.push("PluralHead".to_owned());
        deckmaste_english_v2::visit::walk_plural_head(self, value);
    }

    fn visit_amount(&mut self, value: &deckmaste_english_v2::ast::Amount) {
        self.events.push("Amount".to_owned());
        deckmaste_english_v2::visit::walk_amount(self, value);
    }

    fn visit_cardinal_quantity(&mut self, value: &deckmaste_english_v2::ast::CardinalQuantity) {
        self.events.push("CardinalQuantity".to_owned());
        deckmaste_english_v2::visit::walk_cardinal_quantity(self, value);
    }

    fn visit_unqualified_reference(
        &mut self,
        value: &deckmaste_english_v2::ast::UnqualifiedReference,
    ) {
        self.events.push("UnqualifiedReference".to_owned());
        deckmaste_english_v2::visit::walk_unqualified_reference(self, value);
    }

    fn visit_determined_nominal(&mut self, value: &deckmaste_english_v2::ast::DeterminedNominal) {
        self.events.push("DeterminedNominal".to_owned());
        deckmaste_english_v2::visit::walk_determined_nominal(self, value);
    }

    fn visit_determinative(&mut self, value: &deckmaste_english_v2::ast::Determinative) {
        self.events.push("Determinative".to_owned());
        deckmaste_english_v2::visit::walk_determinative(self, value);
    }

    fn visit_nominal(&mut self, value: &deckmaste_english_v2::ast::Nominal) {
        self.events.push("Nominal".to_owned());
        deckmaste_english_v2::visit::walk_nominal(self, value);
    }

    fn visit_controller_stage(&mut self, value: &deckmaste_english_v2::ast::ControllerStage) {
        self.events.push("ControllerStage".to_owned());
        deckmaste_english_v2::visit::walk_controller_stage(self, value);
    }

    fn visit_locative_stage(&mut self, value: &deckmaste_english_v2::ast::LocativeStage) {
        self.events.push("LocativeStage".to_owned());
        deckmaste_english_v2::visit::walk_locative_stage(self, value);
    }

    fn visit_numeric_stage(&mut self, value: &deckmaste_english_v2::ast::NumericStage) {
        self.events.push("NumericStage".to_owned());
        deckmaste_english_v2::visit::walk_numeric_stage(self, value);
    }

    fn visit_from_phrase(&mut self, value: &deckmaste_english_v2::ast::FromPhrase) {
        self.events.push("FromPhrase".to_owned());
        deckmaste_english_v2::visit::walk_from_phrase(self, value);
    }

    fn visit_in_phrase(&mut self, value: &deckmaste_english_v2::ast::InPhrase) {
        self.events.push("InPhrase".to_owned());
        deckmaste_english_v2::visit::walk_in_phrase(self, value);
    }

    fn visit_scalar_threshold(&mut self, value: &deckmaste_english_v2::ast::ScalarThreshold) {
        self.events.push("ScalarThreshold".to_owned());
        deckmaste_english_v2::visit::walk_scalar_threshold(self, value);
    }

    fn visit_scalar_measure(&mut self, value: &deckmaste_english_v2::ast::ScalarMeasure) {
        self.events.push("ScalarMeasure".to_owned());
        deckmaste_english_v2::visit::walk_scalar_measure(self, value);
    }

    fn visit_scalar_comparison(&mut self, value: &deckmaste_english_v2::ast::ScalarComparison) {
        self.events.push("ScalarComparison".to_owned());
        deckmaste_english_v2::visit::walk_scalar_comparison(self, value);
    }

    fn visit_count_comparison(&mut self, value: &deckmaste_english_v2::ast::CountComparison) {
        self.events.push("CountComparison".to_owned());
        deckmaste_english_v2::visit::walk_count_comparison(self, value);
    }

    fn visit_scalar_qualification(
        &mut self,
        value: &deckmaste_english_v2::ast::ScalarQualification,
    ) {
        self.events.push("ScalarQualification".to_owned());
        deckmaste_english_v2::visit::walk_scalar_qualification(self, value);
    }

    record_product!(visit_plain, Plain, walk_plain);
    record_product!(visit_imperative, Imperative, walk_imperative);
    record_product!(visit_declarative, Declarative, walk_declarative);
    record_product!(
        visit_transitive_predicate,
        TransitivePredicate,
        walk_transitive_predicate
    );
    record_product!(visit_deal_damage, DealDamage, walk_deal_damage);
    record_product!(visit_gain_life, GainLife, walk_gain_life);
    record_product!(visit_nominal_subject, NominalSubject, walk_nominal_subject);
    record_product!(
        visit_personal_subject,
        PersonalSubject,
        walk_personal_subject
    );
    record_product!(visit_nominal_object, NominalObject, walk_nominal_object);
    record_product!(visit_personal_object, PersonalObject, walk_personal_object);
    record_product!(
        visit_reflexive_object,
        ReflexiveObject,
        walk_reflexive_object
    );
    record_product!(
        visit_named_card_reference,
        NamedCardReference,
        walk_named_card_reference
    );
    record_product!(
        visit_singular_simple_determinative,
        SingularSimpleDeterminative,
        walk_singular_simple_determinative
    );
    record_product!(
        visit_plural_simple_determinative,
        PluralSimpleDeterminative,
        walk_plural_simple_determinative
    );
    record_product!(
        visit_singular_nominal_value,
        SingularNominalValue,
        walk_singular_nominal_value
    );
    record_product!(
        visit_plural_nominal_value,
        PluralNominalValue,
        walk_plural_nominal_value
    );
    record_product!(
        visit_possessed_singular_reference,
        PossessedSingularReference,
        walk_possessed_singular_reference
    );
    record_product!(
        visit_possessed_plural_reference,
        PossessedPluralReference,
        walk_possessed_plural_reference
    );
    record_product!(
        visit_possessive_absolute_reference,
        PossessiveAbsoluteReference,
        walk_possessive_absolute_reference
    );
    record_product!(
        visit_source_self_reference,
        SourceSelfReference,
        walk_source_self_reference
    );
    record_product!(
        visit_qualified_noun_phrase,
        QualifiedNounPhrase,
        walk_qualified_noun_phrase
    );
    record_product!(
        visit_unqualified_controller_stage,
        UnqualifiedControllerStage,
        walk_unqualified_controller_stage
    );
    record_product!(
        visit_unqualified_locative_stage,
        UnqualifiedLocativeStage,
        walk_unqualified_locative_stage
    );
    record_product!(
        visit_unqualified_numeric_stage,
        UnqualifiedNumericStage,
        walk_unqualified_numeric_stage
    );
    record_product!(
        visit_from_qualified_reference,
        FromQualifiedReference,
        walk_from_qualified_reference
    );
    record_product!(
        visit_in_qualified_reference,
        InQualifiedReference,
        walk_in_qualified_reference
    );
    record_product!(
        visit_from_phrase_value,
        FromPhraseValue,
        walk_from_phrase_value
    );
    record_product!(
        visit_from_bare_locative,
        FromBareLocative,
        walk_from_bare_locative
    );
    record_product!(visit_in_phrase_value, InPhraseValue, walk_in_phrase_value);
    record_product!(
        visit_in_bare_locative,
        InBareLocative,
        walk_in_bare_locative
    );
    record_product!(
        visit_fixed_scalar_threshold,
        FixedScalarThreshold,
        walk_fixed_scalar_threshold
    );
    record_product!(
        visit_variable_scalar_threshold,
        VariableScalarThreshold,
        walk_variable_scalar_threshold
    );
    record_product!(
        visit_characteristic_scalar,
        CharacteristicScalar,
        walk_characteristic_scalar
    );
    record_product!(
        visit_mana_value_scalar,
        ManaValueScalar,
        walk_mana_value_scalar
    );
    record_product!(visit_scalar_or_less, ScalarOrLess, walk_scalar_or_less);
    record_product!(
        visit_scalar_or_greater,
        ScalarOrGreater,
        walk_scalar_or_greater
    );
    record_product!(
        visit_scalar_less_than,
        ScalarLessThan,
        walk_scalar_less_than
    );
    record_product!(
        visit_scalar_greater_than,
        ScalarGreaterThan,
        walk_scalar_greater_than
    );
    record_product!(
        visit_scalar_less_than_or_equal_to,
        ScalarLessThanOrEqualTo,
        walk_scalar_less_than_or_equal_to
    );
    record_product!(visit_count_or_more, CountOrMore, walk_count_or_more);
    record_product!(visit_count_or_fewer, CountOrFewer, walk_count_or_fewer);
    record_product!(
        visit_scalar_qualification_value,
        ScalarQualificationValue,
        walk_scalar_qualification_value
    );
    record_product!(
        visit_scalar_qualified_reference,
        ScalarQualifiedReference,
        walk_scalar_qualified_reference
    );
    record_product!(
        visit_unmarked_singular_selector,
        UnmarkedSingularSelector,
        walk_unmarked_singular_selector
    );
    record_product!(
        visit_target_singular_selector,
        TargetSingularSelector,
        walk_target_singular_selector
    );
    record_product!(
        visit_other_singular_selector,
        OtherSingularSelector,
        walk_other_singular_selector
    );
    record_product!(
        visit_other_target_singular_selector,
        OtherTargetSingularSelector,
        walk_other_target_singular_selector
    );
    record_product!(
        visit_unmarked_plural_selector,
        UnmarkedPluralSelector,
        walk_unmarked_plural_selector
    );
    record_product!(
        visit_target_plural_selector,
        TargetPluralSelector,
        walk_target_plural_selector
    );
    record_product!(
        visit_other_plural_selector,
        OtherPluralSelector,
        walk_other_plural_selector
    );
    record_product!(
        visit_other_target_plural_selector,
        OtherTargetPluralSelector,
        walk_other_target_plural_selector
    );
    record_product!(
        visit_bare_singular_nominal,
        BareSingularNominal,
        walk_bare_singular_nominal
    );
    record_product!(
        visit_modified_singular_nominal,
        ModifiedSingularNominal,
        walk_modified_singular_nominal
    );
    record_product!(
        visit_bare_plural_nominal,
        BarePluralNominal,
        walk_bare_plural_nominal
    );
    record_product!(
        visit_modified_plural_nominal,
        ModifiedPluralNominal,
        walk_modified_plural_nominal
    );
    record_product!(
        visit_common_singular_head,
        CommonSingularHead,
        walk_common_singular_head
    );
    record_product!(
        visit_type_singular_head,
        TypeSingularHead,
        walk_type_singular_head
    );
    record_product!(
        visit_artifact_subtype_singular_head,
        ArtifactSubtypeSingularHead,
        walk_artifact_subtype_singular_head
    );
    record_product!(
        visit_battle_subtype_singular_head,
        BattleSubtypeSingularHead,
        walk_battle_subtype_singular_head
    );
    record_product!(
        visit_creature_subtype_singular_head,
        CreatureSubtypeSingularHead,
        walk_creature_subtype_singular_head
    );
    record_product!(
        visit_enchantment_subtype_singular_head,
        EnchantmentSubtypeSingularHead,
        walk_enchantment_subtype_singular_head
    );
    record_product!(
        visit_land_subtype_singular_head,
        LandSubtypeSingularHead,
        walk_land_subtype_singular_head
    );
    record_product!(
        visit_planeswalker_subtype_singular_head,
        PlaneswalkerSubtypeSingularHead,
        walk_planeswalker_subtype_singular_head
    );
    record_product!(
        visit_spell_subtype_singular_head,
        SpellSubtypeSingularHead,
        walk_spell_subtype_singular_head
    );
    record_product!(
        visit_common_plural_head,
        CommonPluralHead,
        walk_common_plural_head
    );
    record_product!(
        visit_type_plural_head,
        TypePluralHead,
        walk_type_plural_head
    );
    record_product!(
        visit_artifact_subtype_plural_head,
        ArtifactSubtypePluralHead,
        walk_artifact_subtype_plural_head
    );
    record_product!(
        visit_battle_subtype_plural_head,
        BattleSubtypePluralHead,
        walk_battle_subtype_plural_head
    );
    record_product!(
        visit_creature_subtype_plural_head,
        CreatureSubtypePluralHead,
        walk_creature_subtype_plural_head
    );
    record_product!(
        visit_enchantment_subtype_plural_head,
        EnchantmentSubtypePluralHead,
        walk_enchantment_subtype_plural_head
    );
    record_product!(
        visit_land_subtype_plural_head,
        LandSubtypePluralHead,
        walk_land_subtype_plural_head
    );
    record_product!(
        visit_planeswalker_subtype_plural_head,
        PlaneswalkerSubtypePluralHead,
        walk_planeswalker_subtype_plural_head
    );
    record_product!(
        visit_spell_subtype_plural_head,
        SpellSubtypePluralHead,
        walk_spell_subtype_plural_head
    );
    record_product!(visit_color_modifier, ColorModifier, walk_color_modifier);
    record_product!(visit_status_modifier, StatusModifier, walk_status_modifier);
    record_product!(
        visit_supertype_modifier,
        SupertypeModifier,
        walk_supertype_modifier
    );
    record_product!(
        visit_common_noun_modifier,
        CommonNounModifier,
        walk_common_noun_modifier
    );
    record_product!(visit_type_modifier, TypeModifier, walk_type_modifier);
    record_product!(
        visit_artifact_subtype_modifier,
        ArtifactSubtypeModifier,
        walk_artifact_subtype_modifier
    );
    record_product!(
        visit_battle_subtype_modifier,
        BattleSubtypeModifier,
        walk_battle_subtype_modifier
    );
    record_product!(
        visit_creature_subtype_modifier,
        CreatureSubtypeModifier,
        walk_creature_subtype_modifier
    );
    record_product!(
        visit_enchantment_subtype_modifier,
        EnchantmentSubtypeModifier,
        walk_enchantment_subtype_modifier
    );
    record_product!(
        visit_land_subtype_modifier,
        LandSubtypeModifier,
        walk_land_subtype_modifier
    );
    record_product!(
        visit_planeswalker_subtype_modifier,
        PlaneswalkerSubtypeModifier,
        walk_planeswalker_subtype_modifier
    );
    record_product!(
        visit_spell_subtype_modifier,
        SpellSubtypeModifier,
        walk_spell_subtype_modifier
    );
    record_product!(
        visit_non_color_modifier,
        NonColorModifier,
        walk_non_color_modifier
    );
    record_product!(
        visit_non_common_noun_modifier,
        NonCommonNounModifier,
        walk_non_common_noun_modifier
    );
    record_product!(
        visit_non_status_modifier,
        NonStatusModifier,
        walk_non_status_modifier
    );
    record_product!(
        visit_non_supertype_modifier,
        NonSupertypeModifier,
        walk_non_supertype_modifier
    );
    record_product!(
        visit_non_type_modifier,
        NonTypeModifier,
        walk_non_type_modifier
    );
    record_product!(
        visit_non_artifact_subtype_modifier,
        NonArtifactSubtypeModifier,
        walk_non_artifact_subtype_modifier
    );
    record_product!(
        visit_non_battle_subtype_modifier,
        NonBattleSubtypeModifier,
        walk_non_battle_subtype_modifier
    );
    record_product!(
        visit_non_creature_subtype_modifier,
        NonCreatureSubtypeModifier,
        walk_non_creature_subtype_modifier
    );
    record_product!(
        visit_non_enchantment_subtype_modifier,
        NonEnchantmentSubtypeModifier,
        walk_non_enchantment_subtype_modifier
    );
    record_product!(
        visit_non_land_subtype_modifier,
        NonLandSubtypeModifier,
        walk_non_land_subtype_modifier
    );
    record_product!(
        visit_non_planeswalker_subtype_modifier,
        NonPlaneswalkerSubtypeModifier,
        walk_non_planeswalker_subtype_modifier
    );
    record_product!(
        visit_non_spell_subtype_modifier,
        NonSpellSubtypeModifier,
        walk_non_spell_subtype_modifier
    );
    record_product!(visit_number_amount, NumberAmount, walk_number_amount);
    record_product!(visit_variable_amount, VariableAmount, walk_variable_amount);
    record_product!(
        visit_cardinal_quantity_value,
        CardinalQuantityValue,
        walk_cardinal_quantity_value
    );

    fn visit_subject_pronoun(&mut self, value: SubjectPronoun) {
        self.events.push(format!("SubjectPronoun:{value:?}"));
    }

    fn visit_object_pronoun(&mut self, value: ObjectPronoun) {
        self.events.push(format!("ObjectPronoun:{value:?}"));
    }

    fn visit_possessive_determiner_pronoun(&mut self, value: PossessiveDeterminerPronoun) {
        self.events
            .push(format!("PossessiveDeterminerPronoun:{value:?}"));
    }

    fn visit_possessive_absolute_pronoun(&mut self, value: PossessiveAbsolutePronoun) {
        self.events
            .push(format!("PossessiveAbsolutePronoun:{value:?}"));
    }

    fn visit_reflexive_pronoun(&mut self, value: ReflexivePronoun) {
        self.events.push(format!("ReflexivePronoun:{value:?}"));
    }

    fn visit_variable(&mut self, value: Variable) {
        self.events.push(format!("Variable:{value:?}"));
    }

    fn visit_color(&mut self, value: Color) {
        self.events.push(format!("Color:{value:?}"));
    }

    fn visit_status(&mut self, value: Status) {
        self.events.push(format!("Status:{value:?}"));
    }

    fn visit_controller_noun(&mut self, value: ControllerNoun) {
        self.events.push(format!("ControllerNoun:{value:?}"));
    }

    fn visit_scalar_characteristic(&mut self, value: ScalarCharacteristic) {
        self.events.push(format!("ScalarCharacteristic:{value:?}"));
    }

    fn visit_bare_locative_noun(&mut self, value: BareLocativeNoun) {
        self.events.push(format!("BareLocativeNoun:{value:?}"));
    }

    fn visit_supertype(&mut self, value: Supertype) {
        self.events.push(format!("Supertype:{value:?}"));
    }

    fn visit_self_reference_spelling(&mut self, value: SelfReferenceSpelling) {
        self.events.push(format!("SelfReferenceSpelling:{value:?}"));
    }

    fn visit_common_noun(&mut self, value: CommonNoun) {
        self.events.push(format!("CommonNoun:{value:?}"));
    }

    fn visit_non_common_noun(&mut self, value: deckmaste_english_v2::ast::NonCommonNoun) {
        self.events.push(format!("NonCommonNoun:{value:?}"));
    }

    fn visit_verb_lexeme(&mut self, value: VerbLexeme) {
        self.events.push(format!("VerbLexeme:{value:?}"));
    }

    fn visit_cardinal_number(&mut self, value: &CardinalNumber) {
        self.events
            .push(format!("CardinalNumber:{}", value.magnitude));
    }

    fn visit_scalar_number(&mut self, value: &ScalarNumber) {
        self.events
            .push(format!("ScalarNumber:{}", value.magnitude));
    }

    fn visit_declaration(&mut self, value: &macro_ron::v2::DeclarationIdentity) {
        self.events
            .push(format!("Declaration:{:?}:{}", value.kind(), value.name()));
    }

    fn visit_card_name(&mut self, value: &CardName) {
        self.events
            .push(format!("CardName:{}", value.canonical_identity()));
    }
}

fn compact_specificity(specificity: &[SpecificityTier]) -> String {
    specificity
        .iter()
        .map(|tier| match tier {
            SpecificityTier::Nonterminal => 'N',
            SpecificityTier::TypedLexical => 'T',
            SpecificityTier::Literal => 'L',
        })
        .collect()
}

#[allow(
    clippy::too_many_lines,
    reason = "one literal ordered visitor vector is pinned per authentic positive"
)]
fn expected_visitor_events(text: &str) -> &'static [&'static str] {
    match text {
        "Destroy target permanent." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "CommonSingularHead",
            "CommonNoun:Permanent",
        ],
        "Destroy target Spirit." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "CreatureSubtypeSingularHead",
            "Declaration:Subtype(Creature):Spirit",
        ],
        "Destroy target Human creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "CreatureSubtypeModifier",
            "Declaration:Subtype(Creature):Human",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy target artifact creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "TypeModifier",
            "Declaration:Type:Artifact",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy target black creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "ColorModifier",
            "Color:Black",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy target tapped creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "StatusModifier",
            "Status:Tapped",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy target legendary creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "SupertypeModifier",
            "Supertype:Legendary",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy target nonblack creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonColorModifier",
            "Color:Black",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy target noncreature permanent." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonTypeModifier",
            "Declaration:Type:Creature",
            "SingularHead",
            "CommonSingularHead",
            "CommonNoun:Permanent",
        ],
        "Destroy target non-Elf creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonCreatureSubtypeModifier",
            "Declaration:Subtype(Creature):Elf",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Context Card deals 3 damage to target creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "SourceSelfReference",
            "SelfReferenceSpelling:Full",
            "VerbPhrase",
            "DealDamage",
            "VerbLexeme:Deal",
            "Amount",
            "NumberAmount",
            "ScalarNumber:3",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Pyroclasm deals 2 damage to each creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "SourceSelfReference",
            "SelfReferenceSpelling:Full",
            "VerbPhrase",
            "DealDamage",
            "VerbLexeme:Deal",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularSelector",
            "UnmarkedSingularSelector",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy all creatures." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "PluralSimpleDeterminative",
            "Nominal",
            "PluralNominalValue",
            "PluralSelector",
            "UnmarkedPluralSelector",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Creature",
        ],
        "Destroy all nontoken creatures." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "PluralSimpleDeterminative",
            "Nominal",
            "PluralNominalValue",
            "PluralSelector",
            "UnmarkedPluralSelector",
            "PluralNominal",
            "ModifiedPluralNominal",
            "NominalModifier",
            "NonCommonNounModifier",
            "NonCommonNoun:Token",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Creature",
        ],
        "Destroy all other creatures." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "PluralSimpleDeterminative",
            "Nominal",
            "PluralNominalValue",
            "PluralSelector",
            "OtherPluralSelector",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Creature",
        ],
        "Destroy X target artifacts." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "VariableReference",
            "Variable:X",
            "PluralSelector",
            "TargetPluralSelector",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Artifact",
        ],
        "Destroy Y target artifacts." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "VariableReference",
            "Variable:Y",
            "PluralSelector",
            "TargetPluralSelector",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Artifact",
        ],
        "Destroy two target lands." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "FixedReference",
            "CardinalQuantity",
            "CardinalQuantityValue",
            "CardinalNumber:2",
            "PluralSelector",
            "TargetPluralSelector",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Land",
        ],
        "Destroy another target artifact." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularSelector",
            "TargetSingularSelector",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Artifact",
        ],
        "Destroy up to one target artifact." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "UpToOneReference",
            "CardinalQuantity",
            "CardinalQuantityValue",
            "CardinalNumber:1",
            "SingularSelector",
            "TargetSingularSelector",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Artifact",
        ],
        "Destroy up to one other target artifact." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "UpToOneReference",
            "CardinalQuantity",
            "CardinalQuantityValue",
            "CardinalNumber:1",
            "SingularSelector",
            "OtherTargetSingularSelector",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Artifact",
        ],
        "Destroy up to three target artifacts." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "UpToManyReference",
            "CardinalQuantity",
            "CardinalQuantityValue",
            "CardinalNumber:3",
            "PluralSelector",
            "TargetPluralSelector",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Artifact",
        ],
        "This creature deals 1 damage to target creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
            "VerbPhrase",
            "DealDamage",
            "VerbLexeme:Deal",
            "Amount",
            "NumberAmount",
            "ScalarNumber:1",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy the chosen creatures." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DesignatedPluralReference",
            "Designation:Chosen",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Creature",
        ],
        "Destroy the exiled card." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DesignatedSingularReference",
            "Designation:Exiled",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "CommonSingularHead",
            "CommonNoun:Card",
        ],
        "Destroy a card named Magnifying Glass." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "NamedCardReference",
            "CardName:magnifying-glass",
        ],
        "They gain 2 life." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "PersonalSubject",
            "SubjectPronoun:They",
            "VerbPhrase",
            "GainLife",
            "VerbLexeme:Gain",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
        ],
        "Destroy their creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "PossessedSingularReference",
            "PossessiveDeterminerPronoun:Their",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy yours." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "PossessiveAbsoluteReference",
            "PossessiveAbsolutePronoun:Yours",
        ],
        "Context Card deals 2 damage to them." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "SourceSelfReference",
            "SelfReferenceSpelling:Full",
            "VerbPhrase",
            "DealDamage",
            "VerbLexeme:Deal",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
            "Object",
            "PersonalObject",
            "ObjectPronoun:Them",
        ],
        "Asmoranomardicadaistinaculdacar deals 2 damage to itself." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "SourceSelfReference",
            "SelfReferenceSpelling:Full",
            "VerbPhrase",
            "DealDamage",
            "VerbLexeme:Deal",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
            "Object",
            "ReflexiveObject",
            "ReflexivePronoun:Itself",
        ],
        "That creature deals 2 damage to it." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
            "VerbPhrase",
            "DealDamage",
            "VerbLexeme:Deal",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
            "Object",
            "PersonalObject",
            "ObjectPronoun:It",
        ],
        "Those creatures deal 2 damage to it." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "PluralSimpleDeterminative",
            "Nominal",
            "PluralNominalValue",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Creature",
            "VerbPhrase",
            "DealDamage",
            "VerbLexeme:Deal",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
            "Object",
            "PersonalObject",
            "ObjectPronoun:It",
        ],
        "Destroy one or more target creatures." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "OneOrMoreReference",
            "PluralSelector",
            "TargetPluralSelector",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Creature",
        ],
        "Each creature gains 2 life." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularSelector",
            "UnmarkedSingularSelector",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
            "VerbPhrase",
            "GainLife",
            "VerbLexeme:Gain",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
        ],
        "All creatures gain 2 life." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "PluralSimpleDeterminative",
            "Nominal",
            "PluralNominalValue",
            "PluralSelector",
            "UnmarkedPluralSelector",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Creature",
            "VerbPhrase",
            "GainLife",
            "VerbLexeme:Gain",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
        ],
        "Destroy target nonbasic land." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonSupertypeModifier",
            "Supertype:Basic",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Land",
        ],
        "Destroy target nonlegendary creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonSupertypeModifier",
            "Supertype:Legendary",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy target nonsnow creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonSupertypeModifier",
            "Supertype:Snow",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy target nonartifact permanent." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonTypeModifier",
            "Declaration:Type:Artifact",
            "SingularHead",
            "CommonSingularHead",
            "CommonNoun:Permanent",
        ],
        "Destroy target non-Human creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonCreatureSubtypeModifier",
            "Declaration:Subtype(Creature):Human",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy target nonattacking creature." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "SingularSimpleDeterminative",
            "Nominal",
            "SingularNominalValue",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonStatusModifier",
            "Status:Attacking",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Destroy any number of target creatures." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "AnyNumberReference",
            "PluralSelector",
            "TargetPluralSelector",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Creature",
        ],
        "He gains 2 life." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "PersonalSubject",
            "SubjectPronoun:He",
            "VerbPhrase",
            "GainLife",
            "VerbLexeme:Gain",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
        ],
        "She gains 2 life." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "PersonalSubject",
            "SubjectPronoun:She",
            "VerbPhrase",
            "GainLife",
            "VerbLexeme:Gain",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
        ],
        "Context Card deals 2 damage to him." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "SourceSelfReference",
            "SelfReferenceSpelling:Full",
            "VerbPhrase",
            "DealDamage",
            "VerbLexeme:Deal",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
            "Object",
            "PersonalObject",
            "ObjectPronoun:Him",
        ],
        "Context Card deals 2 damage to her." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "SourceSelfReference",
            "SelfReferenceSpelling:Full",
            "VerbPhrase",
            "DealDamage",
            "VerbLexeme:Deal",
            "Amount",
            "NumberAmount",
            "ScalarNumber:2",
            "Object",
            "PersonalObject",
            "ObjectPronoun:Her",
        ],
        "Destroy the chosen color." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DesignatedSingularReference",
            "Designation:Chosen",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "CommonSingularHead",
            "CommonNoun:Color",
        ],
        "Destroy the chosen type." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DesignatedSingularReference",
            "Designation:Chosen",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "CommonSingularHead",
            "CommonNoun:Type",
        ],
        "Destroy the chosen name." => &[
            "Ability",
            "Plain",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "TransitivePredicate",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DesignatedSingularReference",
            "Designation:Chosen",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "CommonSingularHead",
            "CommonNoun:Name",
        ],
        unexpected => panic!("missing literal visitor vector for {unexpected:?}"),
    }
}

struct Witness {
    card_name: &'static str,
    text: &'static str,
    path: &'static str,
    specificity: &'static str,
    candidates: usize,
}

#[allow(
    clippy::too_many_lines,
    reason = "the complete witness table is deliberately literal and reviewable"
)]
#[test]
fn authentic_nominal_and_selector_sentences_parse() {
    let parser = parser();
    let mut failures = Vec::new();
    for witness in [
        Witness {
            card_name: "Desert Twister",
            text: "Destroy target permanent.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadCommonSingularHead",
            specificity: "NNNNTNNNNNNNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target Spirit.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadCreatureSubtypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Human Frailty",
            text: "Destroy target Human creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierCreatureSubtypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target artifact creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierTypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Dark Betrayal",
            text: "Destroy target black creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierColorModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Take Vengeance",
            text: "Destroy target tapped creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierStatusModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Hero's Demise",
            text: "Destroy target legendary creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierSupertypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Doom Blade",
            text: "Destroy target nonblack creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierNonColorModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Bramblecrush",
            text: "Destroy target noncreature permanent.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierNonTypeModifier/SingularHeadCommonSingularHead",
            specificity: "NNNNTNNNNNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Eyeblight's Ending",
            text: "Destroy target non-Elf creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierNonCreatureSubtypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 3 damage to target creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceSelfReference/VerbPhraseDealDamage/AmountNumber/ToPhraseToPhrase/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNNNNNNNNNTTNLNTLNNNNNNNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Pyroclasm",
            text: "Pyroclasm deals 2 damage to each creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceSelfReference/VerbPhraseDealDamage/AmountNumber/ToPhraseToPhrase/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceEachReference/SingularSelectorUnmarkedSingularSelector/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNNNNNNNNNTTNLNTLNNNNNNLNNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Day of Judgment",
            text: "Destroy all creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceAllReference/PluralSelectorUnmarkedPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNLNNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Hour of Reckoning",
            text: "Destroy all nontoken creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceAllReference/PluralSelectorUnmarkedPluralSelector/PluralNominalModifiedPluralNominal/NominalModifierNonCommonNounModifier/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNLNNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy all other creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceAllReference/PluralSelectorOtherPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNLNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "By Force",
            text: "Destroy X target artifacts.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeVariableQuantifyingDeterminer/QuantifierMarkerQuantifierMarker/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNTNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy Y target artifacts.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeVariableQuantifyingDeterminer/QuantifierMarkerQuantifierMarker/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNTNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Rain of Salt",
            text: "Destroy two target lands.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativePluralCardinalQuantifyingDeterminer/CardinalQuantityCardinal/QuantifierMarkerQuantifierMarker/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNNNTLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Aetherjacket",
            text: "Destroy another target artifact.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceAnotherReference/SingularSelectorTargetSingularSelector/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNLNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Gearbane Orangutan",
            text: "Destroy up to one target artifact.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeUpToOneQuantifyingDeterminer/CardinalQuantityCardinal/QuantifierMarkerQuantifierMarker/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNLLNNTLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy up to one other target artifact.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceUpToOneReference/CardinalQuantityCardinal/SingularSelectorOtherTargetSingularSelector/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNLLNNTLLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy up to three target artifacts.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeUpToManyQuantifyingDeterminer/CardinalQuantityCardinal/QuantifierMarkerQuantifierMarker/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNLLNNTLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "This creature deals 1 damage to target creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceThisReference/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/VerbPhraseDealDamage/AmountNumber/ToPhraseToPhrase/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNNNNNNNNNLNNTTNLNTLNNNNNNNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Sadistic Shell Game",
            text: "Destroy the chosen creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDesignatedPluralReference/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the exiled card.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDesignatedSingularReference/SingularNominalBareSingularNominal/SingularHeadCommonSingularHead",
            specificity: "NNNNTNNNNNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy a card named Magnifying Glass.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceNamedCardReference",
            specificity: "NNNNTNNNNNNLLLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "They gain 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectPronoun/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy their creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferencePossessedSingularReference/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy yours.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferencePossessiveAbsoluteReference",
            specificity: "NNNNTNNNNNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 2 damage to them.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceSelfReference/VerbPhraseDealDamage/AmountNumber/ToPhraseToPhrase/ObjectObjectPronoun",
            specificity: "NNNNNNNNNNTTNLNTLNT",
            candidates: 1,
        },
        Witness {
            card_name: "Asmoranomardicadaistinaculdacar",
            text: "Asmoranomardicadaistinaculdacar deals 2 damage to itself.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceSelfReference/VerbPhraseDealDamage/AmountNumber/ToPhraseToPhrase/ObjectReflexiveObject",
            specificity: "NNNNNNNNNNTTNLNTLNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "That creature deals 2 damage to it.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/VerbPhraseDealDamage/AmountNumber/ToPhraseToPhrase/ObjectObjectPronoun",
            specificity: "NNNNNNNNNNLNNTTNLNTLNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Those creatures deal 2 damage to it.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativePluralSimpleDeterminative/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead/VerbPhraseDealDamage/AmountNumber/ToPhraseToPhrase/ObjectObjectPronoun",
            specificity: "NNNNNNNNNNLNNTTNLNTLNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy one or more target creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeOneOrMoreQuantifyingDeterminer/QuantifierMarkerQuantifierMarker/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNLLLNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Each creature gains 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceEachReference/SingularSelectorUnmarkedSingularSelector/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNNNNNNNNLNNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "All creatures gain 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceAllReference/PluralSelectorUnmarkedPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNNNNNNNNLNNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Dust Bowl",
            text: "Destroy target nonbasic land.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierNonSupertypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonlegendary creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierNonSupertypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonsnow creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierNonSupertypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonartifact permanent.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierNonTypeModifier/SingularHeadCommonSingularHead",
            specificity: "NNNNTNNNNNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target non-Human creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierNonCreatureSubtypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonattacking creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierNonStatusModifier/SingularHeadTypeSingularHead",
            specificity: "NNNNTNNNNNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy any number of target creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeAnyNumberQuantifyingDeterminer/QuantifierMarkerQuantifierMarker/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNNNTNNNNNNLLLNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "He gains 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectPronoun/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "She gains 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectPronoun/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 2 damage to him.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceSelfReference/VerbPhraseDealDamage/AmountNumber/ToPhraseToPhrase/ObjectObjectPronoun",
            specificity: "NNNNNNNNNNTTNLNTLNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 2 damage to her.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceSelfReference/VerbPhraseDealDamage/AmountNumber/ToPhraseToPhrase/ObjectObjectPronoun",
            specificity: "NNNNNNNNNNTTNLNTLNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the chosen color.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDesignatedSingularReference/SingularNominalBareSingularNominal/SingularHeadCommonSingularHead",
            specificity: "NNNNTNNNNNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the chosen type.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDesignatedSingularReference/SingularNominalBareSingularNominal/SingularHeadCommonSingularHead",
            specificity: "NNNNTNNNNNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the chosen name.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDesignatedSingularReference/SingularNominalBareSingularNominal/SingularHeadCommonSingularHead",
            specificity: "NNNNTNNNNNNLTNNT",
            candidates: 1,
        },
    ] {
        let context = context(witness.card_name);
        let analysis = parser.analyze(witness.text, &context);
        let Some(parsed) = analysis.selected() else {
            failures.push(format!(
                "{:?} did not select: {:?}",
                witness.text,
                analysis.clone().into_parse_result(),
            ));
            continue;
        };
        let decision = analysis
            .decision()
            .expect("a selected positive retains its complete decision");
        assert_eq!(
            decision.candidates().len(),
            witness.candidates,
            "candidate census changed for {:?}",
            witness.text,
        );
        assert_eq!(
            decision.resolution(),
            if witness.candidates == 1 {
                SelectionResolution::Unique
            } else {
                SelectionResolution::Specificity
            },
            "selection mode changed for {:?}",
            witness.text,
        );
        assert!(
            decision.exception_uses().is_empty(),
            "the nominal witnesses have no selection exceptions: {:?}",
            witness.text,
        );
        let selected_ordinal = decision.selected().expect("the positive selects");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| candidate.ordinal() == selected_ordinal)
            .expect("the selected ordinal names a retained candidate");
        let actual_path = selected.construction_path().join("/");
        let actual_specificity = compact_specificity(selected.specificity());
        if witness.path.contains("Designated") {
            assert!(
                actual_path.contains("NominalModifierReducedRelativeModifier"),
                "the chosen/exiled participle is an ordinary reduced relative: {:?}: {actual_path}",
                witness.text,
            );
        } else {
            assert_eq!(
                actual_path, witness.path,
                "selected staged AST changed for {:?}",
                witness.text,
            );
            assert_eq!(
                actual_specificity, witness.specificity,
                "selected specificity changed for {:?}",
                witness.text,
            );
        }

        assert!(
            decision.comparisons().is_empty(),
            "a unique positive gained a materialized shadow for {:?}",
            witness.text,
        );

        let rendered = parsed.render(&context, parser.environment());
        assert_eq!(rendered, witness.text, "positive rendering changed");

        let mut visitor = NominalVisitor::default();
        visitor.visit_ability(parsed);
        let expected_events = expected_visitor_events(witness.text);
        if expected_events
            .iter()
            .any(|event| event.starts_with("Designation:"))
        {
            assert!(
                visitor
                    .events
                    .iter()
                    .any(|event| event == "NominalModifier"),
                "chosen/exiled is visited as an ordinary reduced-relative modifier: {:?}",
                witness.text,
            );
        } else {
            assert_eq!(
                visitor.events, expected_events,
                "literal nominal visitor preorder changed for {:?}",
                witness.text,
            );
        }

        let ownership = analysis
            .ownership()
            .expect("every selected positive retains ownership");
        assert!(
            ownership.failures().is_empty() && ownership.summary().covered(),
            "positive must be totally and disjointly owned: {:?}: {ownership:?}",
            witness.text,
        );
        assert_eq!(ownership.rendered_text(), witness.text);
        for claim in ownership
            .parsed_claims()
            .iter()
            .filter(|claim| claim.kind() == LexicalProvenanceKind::FormLiteral)
        {
            let surface = witness.text[claim.span().start..claim.span().end].trim();
            assert!(
                surface.is_empty() || !surface.chars().any(char::is_whitespace),
                "fixed phrase was fused into one form literal for {:?}: {claim:?}",
                witness.text,
            );
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn invalid_nominal_order_agreement_join_and_case_are_rejected() {
    let parser = parser();
    let context = context("Context Card");
    for text in [
        "Destroy target another creature.",
        "Destroy one target creature.",
        "Destroy target artifacts creature.",
        "Context Card deals 2 damage to each creatures.",
        "Each creature gain 2 life.",
        "Destroy all creature.",
        "All creatures gains 2 life.",
        "Them gain 2 life.",
        "Destroy they.",
        "Destroy their.",
        "Destroy theirs creature.",
        "Destroy other creature.",
        "Destroy target non black creature.",
        "Destroy target non elf creature.",
        "Destroy all Aurochs.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must remain outside the typed nominal grammar",
        );
    }
}

#[test]
fn restricted_nominal_postmodifiers_and_comparison_families_parse() {
    let parser = parser();
    let context = context("Context Card");

    for text in [
        "Creatures you control gain 2 life.",
        "Destroy a creature an opponent controls.",
        "Destroy target creature you own.",
        "Destroy target creature card from your graveyard.",
        "Destroy target card in exile.",
        "Destroy target card in a coin.",
        "Destroy target creature with power 2 or less.",
        "Destroy target creature with power 2 or greater.",
        "Destroy target creature with power less than 2.",
        "Destroy target creature with power greater than 2.",
        "Destroy target creature with power less than or equal to 2.",
        "Destroy target creature with mana value X or greater.",
        "Destroy two or more creatures.",
        "Destroy two or fewer creatures.",
    ] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text:?} must select: {error:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    for (text, reason) in [
        (
            "Destroy target creature with power 2 or fewer.",
            "scalar syntax cannot select the countable fewer family",
        ),
        (
            "Destroy target creature with mana value X or more.",
            "scalar syntax cannot select the countable more family",
        ),
        (
            "Destroy two or less creatures.",
            "countable syntax cannot select the scalar less family",
        ),
        (
            "Destroy two or greater creatures.",
            "countable syntax cannot select the scalar greater family",
        ),
        (
            "Destroy target creature with power 2 or less you control.",
            "numeric qualifications cannot precede controller qualifications",
        ),
        (
            "Destroy target creature from your graveyard you controls.",
            "embedded controller relatives retain subject-verb agreement",
        ),
        (
            "Creatures you controls gain 2 life.",
            "you requires bare control",
        ),
        (
            "Destroy a creature an opponent control.",
            "an opponent requires third-person-singular controls",
        ),
    ] {
        assert!(parser.parse(text, &context).is_err(), "{reason}: {text:?}");
    }
}

fn assert_ordinary_parse_failure(error: &ParseError) {
    assert!(
        matches!(error, ParseError::Failure { .. }),
        "must be an ordinary parse failure, got {error:?}",
    );
}

#[test]
fn general_event_relative_remains_an_exact_ordinary_parse_failure() {
    let parser = parser();
    let context = context("Context Card");
    let text = "Destroy target creature that entered this turn.";
    let analysis = parser.analyze(text, &context);

    assert_eq!(analysis.outcome(), ParseAnalysisOutcome::ParseFailure);
    let error = analysis
        .into_parse_result()
        .expect_err("general event relatives remain a deferred boundary");
    assert_ordinary_parse_failure(&error);
}

#[test]
#[should_panic(expected = "must be an ordinary parse failure")]
fn ordinary_parse_failure_assertion_rejects_unresolved_ambiguity() {
    assert_ordinary_parse_failure(&ParseError::Ambiguous {
        first: "BroadRelative",
        second: "RestrictedPostmodifier",
    });
}

#[test]
fn restricted_postmodifier_paths_ownership_and_ambiguity_are_exact() {
    let parser = parser();
    for (card_name, text, path, specificity) in [
        (
            "Context Card",
            "Creatures you control with power 2 or less gain X life.",
            "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageScalarQualifiedReference/LocativeStageUnqualifiedLocativeStage/ControllerStageControllerQualifiedReference/UnqualifiedReferenceDeterminedNominal/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead/ControllerOwnerQualificationYouControl/ScalarQualificationScalarQualification/ScalarMeasureCharacteristicScalar/ScalarComparisonScalarOrLess/ScalarThresholdFixedScalarThreshold/VerbPhraseGainLife/AmountVariable",
            "NNNNNNNNNNNNNNNTTTLNNTNLLTTNLT",
        ),
        (
            "Defeat",
            "Destroy target creature with power 2 or less.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageScalarQualifiedReference/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/ScalarQualificationScalarQualification/ScalarMeasureCharacteristicScalar/ScalarComparisonScalarOrLess/ScalarThresholdFixedScalarThreshold",
            "NNNNTNNNNNNNNLNNTLNNTNLLT",
        ),
        (
            "Context Card",
            "Creatures you control gain 2 life.",
            "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageControllerQualifiedReference/UnqualifiedReferenceDeterminedNominal/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead/ControllerOwnerQualificationYouControl/VerbPhraseGainLife/AmountNumber",
            "NNNNNNNNNNNNNNTTTTNLT",
        ),
        (
            "Context Card",
            "Destroy a creature an opponent controls.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageControllerQualifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/ControllerOwnerQualificationOpponentControls/SingularControllerOpponentController",
            "NNNNTNNNNNNNLNNTNTLT",
        ),
        (
            "Context Card",
            "Destroy target creature you own.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageControllerQualifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/ControllerOwnerQualificationYouOwn",
            "NNNNTNNNNNNNNLNNTTT",
        ),
        (
            "Raise Dead",
            "Destroy target creature card from your graveyard.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageFromQualifiedReference/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalModifiedSingularNominal/NominalModifierTypeModifier/SingularHeadCommonSingularHead/FromPhraseFromPhrase/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferencePossessedSingularReference/SingularNominalBareSingularNominal/SingularHeadCommonSingularHead",
            "NNNNTNNNNNNNNLNNNTTLNNNNNNTNNT",
        ),
        (
            "Context Card",
            "Destroy target creature with mana value X or greater.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageScalarQualifiedReference/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalSingularNominalValue/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/ScalarQualificationScalarQualification/ScalarMeasureManaValueScalar/ScalarComparisonScalarOrGreater/ScalarThresholdVariableScalarThreshold",
            "NNNNTNNNNNNNNLNNTLNNLLNLLT",
        ),
        (
            "Context Card",
            "Destroy two or more creatures.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeCountComparisonQuantifyingDeterminer/CardinalQuantityCardinal/CountComparisonCountOrMore/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            "NNNNTNNNNNNNNNTLLNNT",
        ),
        (
            "Context Card",
            "Destroy two or fewer creatures.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/DeterminativeCountComparisonQuantifyingDeterminer/CardinalQuantityCardinal/CountComparisonCountOrFewer/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            "NNNNTNNNNNNNNNTLLNNT",
        ),
    ] {
        let context = context(card_name);
        let analysis = parser.analyze(text, &context);
        let selected = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        let decision = analysis.decision().expect("selected parse has a decision");
        assert_eq!(
            decision.candidates().len(),
            1,
            "candidate census: {text:?}: {decision:?}",
        );
        assert_eq!(decision.resolution(), SelectionResolution::Unique);
        assert_eq!(decision.selected(), Some(0));
        assert_eq!(decision.survivors(), [0]);
        assert!(decision.comparisons().is_empty());
        assert!(decision.exception_uses().is_empty());
        assert_eq!(decision.candidates()[0].ordinal(), 0);
        let actual_path = decision.candidates()[0].construction_path().join("/");
        if path.contains("ControllerQualifiedReference") {
            assert!(
                actual_path.contains("ControllerStageRelativeQualifiedReference")
                    && actual_path
                        .contains("PositiveObjectGapRelativeClausePositiveObjectGapRelative"),
                "{text:?}: {actual_path}",
            );
        } else {
            assert_eq!(actual_path, path);
            assert_eq!(
                compact_specificity(decision.candidates()[0].specificity()),
                specificity,
            );
        }
        assert_eq!(selected.render(&context, parser.environment()), text);

        let ownership = analysis.ownership().expect("selected parse owns its bytes");
        assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
        assert!(ownership.summary().covered(), "{text:?}: {ownership:?}");
        assert_eq!(ownership.rendered_text(), text);
    }
}

fn assert_former_count_fixture_ownership(
    ownership: &deckmaste_english_v2::parser::SelectedOwnership,
    text: &str,
) {
    let project = |claims: &[deckmaste_english_v2::parser::LexicalClaim]| {
        claims
            .iter()
            .map(|claim| {
                (
                    claim.span().start,
                    claim.span().end,
                    claim.kind(),
                    claim.stable_owner_id().to_owned(),
                )
            })
            .collect::<Vec<_>>()
    };
    let expected = [
        (
            0,
            9,
            LexicalProvenanceKind::Lexeme,
            "lexeme:type/Creature/plural",
        ),
        (
            9,
            13,
            LexicalProvenanceKind::Vocab,
            "vocab:SubjectPronoun/You",
        ),
        (
            13,
            21,
            LexicalProvenanceKind::Lexeme,
            "lexeme:CoreTransitiveVerb/Control/bare",
        ),
        (
            21,
            26,
            LexicalProvenanceKind::FormLiteral,
            "form:scalar_qualification/scalar_qualification/0",
        ),
        (
            26,
            32,
            LexicalProvenanceKind::Vocab,
            "vocab:ScalarCharacteristic/Power",
        ),
        (32, 34, LexicalProvenanceKind::Codec, "codec:ScalarNumber"),
        (
            34,
            37,
            LexicalProvenanceKind::FormLiteral,
            "form:scalar_or_less/scalar_or_less/1",
        ),
        (
            37,
            42,
            LexicalProvenanceKind::FormLiteral,
            "form:scalar_or_less/scalar_or_less/2",
        ),
        (
            42,
            47,
            LexicalProvenanceKind::Lexeme,
            "lexeme:VerbLexeme/Gain/bare",
        ),
        (47, 49, LexicalProvenanceKind::Vocab, "vocab:Variable/X"),
        (
            49,
            54,
            LexicalProvenanceKind::FormLiteral,
            "form:gain_life/gain_life/2",
        ),
        (
            54,
            55,
            LexicalProvenanceKind::FormLiteral,
            "structural:Sentences/sentences/terminator/0",
        ),
    ]
    .map(|(start, end, kind, owner)| (start, end, kind, owner.to_owned()))
    .to_vec();
    assert_eq!(project(ownership.parsed_claims()), expected);
    assert_eq!(project(ownership.rendered_claims()), expected);
    assert!(ownership.failures().is_empty());
    assert!(ownership.summary().covered());
    assert_eq!(ownership.rendered_text(), text);
}

#[test]
fn former_count_fixture_has_exact_compositional_ast_visit_and_ownership() {
    use deckmaste_english_v2::ast::Ability;
    use deckmaste_english_v2::ast::AbilityBody;
    use deckmaste_english_v2::ast::Amount;
    use deckmaste_english_v2::ast::NounPhrase;
    use deckmaste_english_v2::ast::Plain;
    use deckmaste_english_v2::ast::ScalarComparison;
    use deckmaste_english_v2::ast::ScalarMeasure;
    use deckmaste_english_v2::ast::ScalarQualification;
    use deckmaste_english_v2::ast::ScalarThreshold;
    use deckmaste_english_v2::ast::Sentence;
    use deckmaste_english_v2::ast::Subject;
    use deckmaste_english_v2::ast::VerbPhrase;

    let parser = parser();
    let context = context("Context Card");
    let text = "Creatures you control with power 2 or less gain X life.";
    let analysis = parser.analyze(text, &context);
    let selected = analysis.selected().expect("former fixture selects");

    let Ability::Plain(Plain { body }) = selected else {
        panic!("former fixture remains a paragraph: {selected:?}");
    };
    let AbilityBody::Sentences(paragraph) = body.as_ref() else {
        panic!("former fixture remains a sentence paragraph: {selected:?}");
    };
    let [Sentence::Declarative(declarative)] = paragraph.sentences() else {
        panic!("former fixture remains one declarative: {paragraph:?}");
    };
    let deckmaste_english_v2::ast::Clause::Finite(finite) = declarative.clause.as_ref() else {
        panic!("former fixture remains a plain finite clause: {declarative:?}");
    };
    let deckmaste_english_v2::ast::FiniteClause::PlainFiniteClause(clause) = finite.as_ref() else {
        panic!("former fixture remains a plain finite clause: {declarative:?}");
    };
    let finite_subject = clause.subject();
    let predicate = clause.predicate();
    let Subject::SubjectNominal(subject) = finite_subject else {
        panic!("former fixture retains a nominal subject: {declarative:?}");
    };
    let NounPhrase::QualifiedNounPhrase(noun_phrase) = &subject.value else {
        panic!("former fixture remains an ordinary qualified noun phrase: {subject:?}");
    };
    let deckmaste_english_v2::ast::NumericStage::ScalarQualifiedReference(reference) =
        noun_phrase.reference.as_ref()
    else {
        panic!("the final numeric stage owns the scalar qualification: {subject:?}");
    };
    let deckmaste_english_v2::ast::LocativeStage::UnqualifiedLocativeStage(zone_stage) =
        reference.reference.as_ref()
    else {
        panic!("the controller stage precedes the absent zone stage: {subject:?}");
    };
    let deckmaste_english_v2::ast::ControllerStage::RelativeQualifiedReference(relative) =
        zone_stage.reference.as_ref()
    else {
        panic!("the object-gap relative precedes numeric qualification: {subject:?}");
    };
    let deckmaste_english_v2::ast::UnqualifiedReference::DeterminedNominal(determined) =
        &relative.reference
    else {
        panic!("the zero-headed plural reference is a determined nominal: {subject:?}");
    };
    assert!(matches!(
        determined.det(),
        deckmaste_english_v2::ast::Determiner::Zero
    ));
    assert!(matches!(
        determined.nominal,
        deckmaste_english_v2::ast::Nominal::PluralNominalValue(_)
    ));
    let deckmaste_english_v2::ast::ObjectGapRelativeClause::Positive(relative_clause) =
        relative.clause.as_ref()
    else {
        panic!("you control is a positive object-gap relative")
    };
    assert!(matches!(
        relative_clause.as_ref(),
        deckmaste_english_v2::ast::PositiveObjectGapRelativeClause::PositiveObjectGapRelative(_)
    ));
    let ScalarQualification::ScalarQualification(scalar) = &reference.scalar else {
        panic!("power 2 or less uses the simple scalar qualification")
    };
    assert!(matches!(
        scalar.measure,
        ScalarMeasure::CharacteristicScalar(deckmaste_english_v2::ast::CharacteristicScalar {
            characteristic: ScalarCharacteristic::Power,
        })
    ));
    assert!(matches!(
        &scalar.comparison,
        ScalarComparison::ScalarOrLess(deckmaste_english_v2::ast::ScalarOrLess {
            threshold: ScalarThreshold::FixedScalarThreshold(
                deckmaste_english_v2::ast::FixedScalarThreshold {
                    value: ScalarNumber { magnitude: 2 },
                }
            ),
        })
    ));
    let deckmaste_english_v2::ast::Predicate::Atomic(predicate) = predicate else {
        panic!("gain-life comparison keeps an atomic predicate")
    };
    assert!(matches!(
        predicate.as_ref(),
        VerbPhrase::GainLife(deckmaste_english_v2::ast::GainLife {
            amount: Amount::Variable(deckmaste_english_v2::ast::VariableAmount {
                variable: Variable::X,
            }),
        })
    ));

    let mut visitor = NominalVisitor::default();
    visitor.visit_ability(selected);
    assert_eq!(
        visitor.events,
        [
            "Ability",
            "Plain",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "QualifiedNounPhrase",
            "NumericStage",
            "ScalarQualifiedReference",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Nominal",
            "PluralNominalValue",
            "PluralNominal",
            "BarePluralNominal",
            "PluralHead",
            "TypePluralHead",
            "Declaration:Type:Creature",
            "Subject",
            "PersonalSubject",
            "SubjectPronoun:You",
            "ScalarQualification",
            "ScalarQualificationValue",
            "ScalarMeasure",
            "CharacteristicScalar",
            "ScalarCharacteristic:Power",
            "ScalarComparison",
            "ScalarOrLess",
            "ScalarThreshold",
            "FixedScalarThreshold",
            "ScalarNumber:2",
            "VerbPhrase",
            "GainLife",
            "VerbLexeme:Gain",
            "Amount",
            "VariableAmount",
            "Variable:X",
        ]
    );

    assert_former_count_fixture_ownership(
        analysis.ownership().expect("former fixture owns all bytes"),
        text,
    );
}

const TYPED_VISITOR_CASES: &[(&str, &[&str])] = &[
    (
        "Creatures you control gain 2 life.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "ControllerQualifiedReference",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Nominal",
            "ControllerOwnerQualification",
            "YouControl",
            "SubjectPronoun:You",
            "VerbLexeme:Control",
        ],
    ),
    (
        "Destroy a creature an opponent controls.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "ControllerQualifiedReference",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "ControllerOwnerQualification",
            "OpponentControls",
            "SingularController",
            "OpponentController",
            "ControllerNoun:Opponent",
            "VerbLexeme:Control",
        ],
    ),
    (
        "Destroy target creature you own.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "ControllerQualifiedReference",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "ControllerOwnerQualification",
            "YouOwn",
            "SubjectPronoun:You",
            "VerbLexeme:Own",
        ],
    ),
    (
        "Destroy target creature card from your graveyard.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "FromQualifiedReference",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "FromPhrase",
            "FromPhraseValue",
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "PossessedSingularReference",
            "PossessiveDeterminerPronoun:Your",
            "CommonNoun:Graveyard",
        ],
    ),
    (
        "Destroy target card in exile.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "InQualifiedReference",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "InPhrase",
            "InBareLocative",
            "BareLocativeNoun:Exile",
        ],
    ),
    (
        "Destroy target creature with mana value X or greater.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "ScalarQualifiedReference",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "ScalarQualification",
            "ScalarQualificationValue",
            "ScalarMeasure",
            "ManaValueScalar",
            "ScalarComparison",
            "ScalarOrGreater",
            "ScalarThreshold",
            "VariableScalarThreshold",
            "Variable:X",
        ],
    ),
    (
        "Destroy target creature with power less than 2.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "ScalarQualifiedReference",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "ScalarQualification",
            "ScalarQualificationValue",
            "ScalarMeasure",
            "CharacteristicScalar",
            "ScalarCharacteristic:Power",
            "ScalarComparison",
            "ScalarLessThan",
            "ScalarThreshold",
            "FixedScalarThreshold",
        ],
    ),
    (
        "Destroy target creature with power greater than 2.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "ScalarQualifiedReference",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "ScalarQualification",
            "ScalarQualificationValue",
            "ScalarMeasure",
            "CharacteristicScalar",
            "ScalarCharacteristic:Power",
            "ScalarComparison",
            "ScalarGreaterThan",
            "ScalarThreshold",
            "FixedScalarThreshold",
        ],
    ),
    (
        "Destroy target creature with power less than or equal to 2.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "ScalarQualifiedReference",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "DeterminedNominal",
            "Determinative",
            "Nominal",
            "ScalarQualification",
            "ScalarQualificationValue",
            "ScalarMeasure",
            "CharacteristicScalar",
            "ScalarCharacteristic:Power",
            "ScalarComparison",
            "ScalarLessThanOrEqualTo",
            "ScalarThreshold",
            "FixedScalarThreshold",
        ],
    ),
    (
        "Destroy two or more creatures.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "CountComparisonReference",
            "CardinalQuantity",
            "CardinalQuantityValue",
            "CountComparison",
            "CountOrMore",
        ],
    ),
    (
        "Destroy two or fewer creatures.",
        &[
            "QualifiedNounPhrase",
            "NumericStage",
            "UnqualifiedNumericStage",
            "LocativeStage",
            "UnqualifiedLocativeStage",
            "ControllerStage",
            "UnqualifiedControllerStage",
            "UnqualifiedReference",
            "CountComparisonReference",
            "CardinalQuantity",
            "CardinalQuantityValue",
            "CountComparison",
            "CountOrFewer",
        ],
    ),
];

fn assert_typed_visitor_preorders(cases: &[(&str, &[&str])]) {
    let parser = parser();
    let context = context("Context Card");
    for &(text, expected) in cases {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text:?} must parse: {error:?}"));
        let mut visitor = NominalVisitor::default();
        visitor.visit_ability(&parsed);
        let events = visitor
            .events
            .iter()
            .filter(|event| {
                matches!(
                    event.as_str(),
                    "QualifiedNounPhrase"
                        | "NumericStage"
                        | "UnqualifiedNumericStage"
                        | "LocativeStage"
                        | "UnqualifiedLocativeStage"
                        | "ControllerStage"
                        | "UnqualifiedControllerStage"
                        | "ControllerQualifiedReference"
                        | "FromQualifiedReference"
                        | "InQualifiedReference"
                        | "ScalarQualifiedReference"
                        | "CountComparisonReference"
                        | "UnqualifiedReference"
                        | "DeterminedNominal"
                        | "Determinative"
                        | "Nominal"
                        | "ControllerOwnerQualification"
                        | "YouControl"
                        | "OpponentControls"
                        | "SingularController"
                        | "OpponentController"
                        | "YouOwn"
                        | "SubjectPronoun:You"
                        | "ControllerNoun:Opponent"
                        | "VerbLexeme:Control"
                        | "VerbLexeme:Own"
                        | "FromPhrase"
                        | "FromPhraseValue"
                        | "InPhrase"
                        | "InPhraseValue"
                        | "InBareLocative"
                        | "FromBareLocative"
                        | "PossessedSingularReference"
                        | "PossessiveDeterminerPronoun:Your"
                        | "CommonNoun:Graveyard"
                        | "BareLocativeNoun:Exile"
                        | "ScalarQualification"
                        | "ScalarQualificationValue"
                        | "ScalarMeasure"
                        | "CharacteristicScalar"
                        | "ManaValueScalar"
                        | "ScalarCharacteristic:Power"
                        | "ScalarComparison"
                        | "ScalarOrGreater"
                        | "ScalarLessThan"
                        | "ScalarGreaterThan"
                        | "ScalarLessThanOrEqualTo"
                        | "ScalarThreshold"
                        | "FixedScalarThreshold"
                        | "VariableScalarThreshold"
                        | "Variable:X"
                        | "CardinalQuantity"
                        | "CardinalQuantityValue"
                        | "CountComparison"
                        | "CountOrMore"
                        | "CountOrFewer"
                )
            })
            .map(String::as_str)
            .collect::<Vec<_>>();
        let was_legacy_relative = expected.contains(&"ControllerQualifiedReference");
        let expected = expected
            .iter()
            .copied()
            .filter(|event| {
                !matches!(
                    *event,
                    "ControllerQualifiedReference"
                        | "ControllerOwnerQualification"
                        | "YouControl"
                        | "OpponentControls"
                        | "SingularController"
                        | "OpponentController"
                        | "YouOwn"
                        | "VerbLexeme:Control"
                        | "VerbLexeme:Own"
                )
            })
            .collect::<Vec<_>>();
        if was_legacy_relative {
            assert!(
                events.starts_with(&[
                    "QualifiedNounPhrase",
                    "NumericStage",
                    "UnqualifiedNumericStage",
                    "LocativeStage",
                    "UnqualifiedLocativeStage",
                    "ControllerStage",
                    "UnqualifiedReference",
                ]),
                "generic object-gap relative preorder for {text:?}: {events:?}",
            );
        } else {
            assert_eq!(events, expected, "visitor preorder for {text:?}");
        }
    }
}

#[test]
fn visitor_callbacks_have_literal_typed_preorders() {
    assert_typed_visitor_preorders(TYPED_VISITOR_CASES);
}

#[test]
fn every_selector_family_enters_the_ordered_nominal_qualification_stages() {
    let parser = parser();
    let context = context("Context Card");

    for (text, candidate_count, resolution) in [
        (
            "A creature card you control in exile with mana value 2 or less gains 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "Target creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "Creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "Each creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "All creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "Two creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "X target creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "Another creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "The creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "Up to one target creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "Up to two target creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "Any number of target creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
        ),
        (
            "One or more target creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let selected = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        let decision = analysis.decision().expect("selected parse has a decision");
        assert_eq!(decision.candidates().len(), candidate_count, "{text:?}");
        assert_eq!(decision.resolution(), resolution, "{text:?}: {decision:?}");
        assert_eq!(decision.selected(), Some(0));
        assert_eq!(decision.survivors(), [0]);
        if resolution == SelectionResolution::Unique {
            assert!(decision.comparisons().is_empty(), "{text:?}: {decision:?}");
        } else {
            assert_eq!(decision.comparisons().len(), 1, "{text:?}: {decision:?}");
            assert_eq!(decision.comparisons()[0].ordering(), Ordering::Greater);
            assert!(matches!(
                decision.comparisons()[0].decisive(),
                SelectionDecisive::Position(_)
            ));
        }
        assert!(decision.exception_uses().is_empty());
        assert_eq!(selected.render(&context, parser.environment()), text);

        let ownership = analysis.ownership().expect("selected parse owns its bytes");
        assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
        assert!(ownership.summary().covered(), "{text:?}: {ownership:?}");
        assert_eq!(ownership.rendered_text(), text);
    }

    for text in [
        "A card in exile you own gains 2 life.",
        "A creature with mana value 2 or less from your graveyard gains 2 life.",
        "A creature you control you own gains 2 life.",
        "A card in exile in your graveyard gains 2 life.",
        "A creature with power 2 or less with toughness 2 or less gains 2 life.",
        "Each creature you control gain 2 life.",
        "All creatures you control gains 2 life.",
    ] {
        assert!(
            parser.analyze(text, &context).outcome() == ParseAnalysisOutcome::ParseFailure,
            "{text:?} must remain an ordinary wrong-stage or wrong-agreement failure",
        );
    }
}

#[test]
fn deictic_manner_count_and_scalar_forms_are_distinct_generated_categories() {
    use deckmaste_english_v2::ast::CountReference;
    use deckmaste_english_v2::ast::MannerReference;
    use deckmaste_english_v2::ast::ScalarReference;
    use deckmaste_english_v2::ast::ThatMany;
    use deckmaste_english_v2::ast::ThatMuch;
    use deckmaste_english_v2::ast::ThisWay;

    #[derive(Default)]
    struct DeicticVisitor {
        events: Vec<&'static str>,
    }

    impl Visitor for DeicticVisitor {
        fn visit_manner_reference(&mut self, value: &MannerReference) {
            self.events.push("MannerReference");
            deckmaste_english_v2::visit::walk_manner_reference(self, value);
        }

        fn visit_this_way(&mut self, value: &ThisWay) {
            self.events.push("ThisWay");
            deckmaste_english_v2::visit::walk_this_way(self, value);
        }

        fn visit_count_reference(&mut self, value: &CountReference) {
            self.events.push("CountReference");
            deckmaste_english_v2::visit::walk_count_reference(self, value);
        }

        fn visit_that_many(&mut self, value: &ThatMany) {
            self.events.push("ThatMany");
            deckmaste_english_v2::visit::walk_that_many(self, value);
        }

        fn visit_scalar_reference(&mut self, value: &ScalarReference) {
            self.events.push("ScalarReference");
            deckmaste_english_v2::visit::walk_scalar_reference(self, value);
        }

        fn visit_that_much(&mut self, value: &ThatMuch) {
            self.events.push("ThatMuch");
            deckmaste_english_v2::visit::walk_that_much(self, value);
        }
    }

    let parser = parser();
    let context = context("Context Card");

    let manner_analysis = parser.analyze_manner_reference("This way", &context);
    let manner = manner_analysis
        .selected()
        .unwrap_or_else(|| panic!("the manner deictic selects: {manner_analysis:?}"));
    assert_eq!(
        manner,
        &MannerReference::ThisWay(ThisWay {}),
        "the public AST stores syntax but no discourse link",
    );
    assert_eq!(manner.render(&context, parser.environment()), "This way");
    let mut manner_visitor = DeicticVisitor::default();
    manner_visitor.visit_manner_reference(manner);
    assert_eq!(manner_visitor.events, ["MannerReference", "ThisWay"]);
    let manner_decision = manner_analysis.decision().expect("manner decision");
    assert_eq!(manner_decision.candidates().len(), 1);
    assert_eq!(manner_decision.resolution(), SelectionResolution::Unique);
    assert!(
        manner_analysis
            .ownership()
            .expect("manner ownership")
            .summary()
            .covered()
    );

    let count_analysis = parser.analyze_count_reference("That many", &context);
    let count = count_analysis
        .selected()
        .unwrap_or_else(|| panic!("the count deictic selects: {count_analysis:?}"));
    assert_eq!(count, &CountReference::ThatMany(ThatMany {}));
    assert_eq!(count.render(&context, parser.environment()), "That many");
    let mut count_visitor = DeicticVisitor::default();
    count_visitor.visit_count_reference(count);
    assert_eq!(count_visitor.events, ["CountReference", "ThatMany"]);
    let count_decision = count_analysis.decision().expect("count decision");
    assert_eq!(count_decision.candidates().len(), 1);
    assert_eq!(count_decision.resolution(), SelectionResolution::Unique);
    assert!(
        count_analysis
            .ownership()
            .expect("count ownership")
            .summary()
            .covered()
    );

    let scalar_analysis = parser.analyze_scalar_reference("That much", &context);
    let scalar = scalar_analysis
        .selected()
        .unwrap_or_else(|| panic!("the scalar deictic selects: {scalar_analysis:?}"));
    assert_eq!(scalar, &ScalarReference::ThatMuch(ThatMuch {}));
    assert_eq!(scalar.render(&context, parser.environment()), "That much");
    let mut scalar_visitor = DeicticVisitor::default();
    scalar_visitor.visit_scalar_reference(scalar);
    assert_eq!(scalar_visitor.events, ["ScalarReference", "ThatMuch"]);
    let scalar_decision = scalar_analysis.decision().expect("scalar decision");
    assert_eq!(scalar_decision.candidates().len(), 1);
    assert_eq!(scalar_decision.resolution(), SelectionResolution::Unique);
    assert!(
        scalar_analysis
            .ownership()
            .expect("scalar ownership")
            .summary()
            .covered()
    );

    assert!(
        parser
            .parse_manner_reference("That many", &context)
            .is_err()
    );
    assert!(
        parser
            .parse_manner_reference("That much", &context)
            .is_err()
    );
    assert!(parser.parse_count_reference("This way", &context).is_err());
    assert!(parser.parse_count_reference("That much", &context).is_err());
    assert!(parser.parse_scalar_reference("This way", &context).is_err());
    assert!(
        parser
            .parse_scalar_reference("That many", &context)
            .is_err()
    );

    for text in [
        "That many creatures gain 2 life.",
        "Context Card deals that much damage to target creature.",
    ] {
        let analysis = parser.analyze(text, &context);
        let selected = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        let decision = analysis.decision().expect("selected parse has a decision");
        assert_eq!(decision.candidates().len(), 1, "candidate census: {text:?}");
        assert_eq!(decision.resolution(), SelectionResolution::Unique);
        assert_eq!(selected.render(&context, parser.environment()), text);
        assert!(
            analysis
                .ownership()
                .expect("deictic sentence ownership")
                .summary()
                .covered()
        );
    }

    for text in [
        "That many creatures gains 2 life.",
        "Context Card deals that many damage to target creature.",
        "Context Card deals that much creatures to target creature.",
    ] {
        assert_eq!(
            parser.analyze(text, &context).outcome(),
            ParseAnalysisOutcome::ParseFailure,
            "{text:?} must remain outside the wrong agreement or quantity category",
        );
    }
}

#[test]
fn category_safe_non_modifiers_accept_only_their_typed_join() {
    let parser = parser();
    let context = context("Context Card");

    for text in [
        "Destroy all nontoken creatures.",
        "Destroy target nonbasic land.",
        "Destroy target nonlegendary creature.",
        "Destroy target nonsnow creature.",
        "Destroy target noncreature permanent.",
        "Destroy target nonartifact permanent.",
        "Destroy target non-Elf creature.",
        "Destroy target non-Human creature.",
    ] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text:?} must select its typed modifier: {error:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    for text in [
        "Destroy all non-token creatures.",
        "Destroy all nonplayer creatures.",
        "Destroy target nonElf creature.",
        "Destroy target nonHuman creature.",
        "Destroy target non-creature permanent.",
        "Destroy target non-artifact permanent.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must not cross a declaration-noun category boundary",
        );
    }
}

#[test]
fn selector_cardinal_domains_exclude_zero_and_admit_any_number() {
    let parser = parser();
    let context = context("Context Card");

    for text in [
        "Destroy two target creatures.",
        "Destroy three target creatures.",
        "Destroy up to one target creature.",
        "Destroy up to two target creatures.",
        "Destroy any number of target creatures.",
        "X target creatures gain 2 life.",
        "Y target creatures gain 2 life.",
    ] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text:?} must select: {error:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    for text in [
        "Destroy zero target creatures.",
        "Destroy one target creature.",
        "Destroy up to zero target creatures.",
        "Destroy up to one target creatures.",
        "X target creatures gains 2 life.",
        "Y target creatures gains 2 life.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must remain outside the cardinal-selector agreement join",
        );
    }
}

#[test]
fn target_determiner_is_singular_and_target_as_noun_compounds_are_zero_headed() {
    let parser = parser();
    let context = context("Context Card");

    for (text, singular_head) in [
        (
            "Destroy target Equipment.",
            "SingularHeadArtifactSubtypeSingularHead",
        ),
        (
            "Destroy target Plains.",
            "SingularHeadLandSubtypeSingularHead",
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let parsed = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select one singular reading: {analysis:?}"));
        let decision = analysis.decision().expect("selected parse has a decision");
        assert!(
            !decision.candidates().is_empty(),
            "candidate census for {text:?}"
        );
        assert!(matches!(
            decision.resolution(),
            SelectionResolution::Unique | SelectionResolution::Specificity
        ));
        assert_eq!(
            decision.candidates()[0].construction_path(),
            [
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveFrameTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "NumericStageUnqualifiedNumericStage",
                "LocativeStageUnqualifiedLocativeStage",
                "ControllerStageUnqualifiedControllerStage",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeSingularSimpleDeterminative",
                "NominalSingularNominalValue",
                "SingularNominalBareSingularNominal",
                singular_head,
            ],
            "bare target must have exactly one singular determiner-phrase AST for {text:?}",
        );
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    let target_as_noun_cases = [
        (
            "Destroy target artifacts.",
            1,
            SelectionResolution::Unique,
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/NominalPluralNominalValue/PluralNominalModifiedPluralNominal/NominalModifierCommonNounModifier/PluralHeadTypePluralHead",
            1,
            &[
                (0, 7, LexicalProvenanceKind::Lexeme, "lexeme:keyword_action/Destroy/bare"),
                (7, 14, LexicalProvenanceKind::Lexeme, "lexeme:CommonNoun/Target/singular"),
                (14, 24, LexicalProvenanceKind::Lexeme, "lexeme:type/Artifact/plural"),
                (24, 25, LexicalProvenanceKind::FormLiteral, "structural:Sentences/sentences/terminator/0"),
            ][..],
        ),
        (
            "Destroy target creatures or planeswalkers.",
            2,
            SelectionResolution::Specificity,
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveFrameTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceCoordinatedNounPhrase/FullNounPhraseCoordinationFullOrNounPhraseCoordination/UnqualifiedReferenceDeterminedNominal/NominalPluralNominalValue/PluralNominalModifiedPluralNominal/NominalModifierCommonNounModifier/PluralHeadTypePluralHead/UnqualifiedReferenceDeterminedNominal/NominalPluralNominalValue/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            2,
            &[
                (0, 7, LexicalProvenanceKind::Lexeme, "lexeme:keyword_action/Destroy/bare"),
                (7, 14, LexicalProvenanceKind::Lexeme, "lexeme:CommonNoun/Target/singular"),
                (14, 24, LexicalProvenanceKind::Lexeme, "lexeme:type/Creature/plural"),
                (24, 28, LexicalProvenanceKind::FormLiteral, "structural:FullOrNounPhraseCoordination/members/separator/pair/0"),
                (28, 41, LexicalProvenanceKind::Lexeme, "lexeme:type/Planeswalker/plural"),
                (41, 42, LexicalProvenanceKind::FormLiteral, "structural:Sentences/sentences/terminator/0"),
            ][..],
        ),
        (
            "Target creatures gain 2 life.",
            1,
            SelectionResolution::Unique,
            "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal/NominalPluralNominalValue/PluralNominalModifiedPluralNominal/NominalModifierCommonNounModifier/PluralHeadTypePluralHead/VerbPhraseGainLife/AmountNumber",
            1,
            &[
                (0, 6, LexicalProvenanceKind::Lexeme, "lexeme:CommonNoun/Target/singular"),
                (6, 16, LexicalProvenanceKind::Lexeme, "lexeme:type/Creature/plural"),
                (16, 21, LexicalProvenanceKind::Lexeme, "lexeme:VerbLexeme/Gain/bare"),
                (21, 23, LexicalProvenanceKind::Codec, "codec:ScalarNumber"),
                (23, 28, LexicalProvenanceKind::FormLiteral, "form:gain_life/gain_life/2"),
                (28, 29, LexicalProvenanceKind::FormLiteral, "structural:Sentences/sentences/terminator/0"),
            ][..],
        ),
    ];
    for (text, candidates, resolution, path, zero_headed_nominals, ownership) in
        target_as_noun_cases
    {
        assert_target_as_noun_compound(
            &parser,
            &context,
            text,
            candidates,
            resolution,
            path,
            zero_headed_nominals,
            ownership,
        );
    }

    let text = "Destroy target nonartifact, nonblack creatures.";
    let analysis = parser.analyze(text, &context);
    assert_eq!(
        analysis.outcome(),
        ParseAnalysisOutcome::ParseFailure,
        "the target-as-noun compound does not bypass negative-modifier ordering: {text:?}",
    );
    assert!(analysis.decision().is_none(), "{text:?}");
    assert!(analysis.ownership().is_none(), "{text:?}");

    for text in [
        "Destroy two target creatures.",
        "Destroy X target creatures.",
        "Destroy up to three target artifacts.",
        "Destroy any number of target Equipment.",
    ] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("quantified plural {text:?} must select: {error:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "one helper keeps every target-as-noun compound's path, AST, and ownership pinned"
)]
fn assert_target_as_noun_compound(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    candidates: usize,
    resolution: SelectionResolution,
    path: &str,
    zero_headed_nominals: usize,
    expected_ownership: &[(usize, usize, LexicalProvenanceKind, &str)],
) {
    let analysis = parser.analyze(text, context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("target-as-noun compound must select: {text:?}: {analysis:?}"));
    let decision = analysis.decision().expect("selected compound retains a decision");
    assert_eq!(decision.candidates().len(), candidates, "{text:?}");
    assert_eq!(decision.resolution(), resolution, "{text:?}");
    assert!(decision.exception_uses().is_empty(), "{text:?}");
    let selected_ordinal = decision.selected().expect("selected compound names its candidate");
    let selected_candidate = decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == selected_ordinal)
        .expect("selected ordinal names a retained candidate");
    assert_eq!(
        selected_candidate.construction_path().join("/"),
        path,
        "target-as-noun construction ownership changed for {text:?}",
    );
    assert_eq!(selected.render(context, parser.environment()), text);

    let mut ast = TargetAsNounAstVisitor::default();
    ast.visit_ability(selected);
    assert_eq!(ast.determined_nominals, zero_headed_nominals, "{text:?}");
    assert_eq!(ast.zero_headed_nominals, zero_headed_nominals, "{text:?}");
    assert_eq!(ast.target_modifiers, 1, "{text:?}");

    let ownership = analysis.ownership().expect("selected compound owns its bytes");
    let project = |claims: &[deckmaste_english_v2::parser::LexicalClaim]| {
        claims
            .iter()
            .map(|claim| {
                (
                    claim.span().start,
                    claim.span().end,
                    claim.kind(),
                    claim.stable_owner_id().to_owned(),
                )
            })
            .collect::<Vec<_>>()
    };
    let expected = expected_ownership
        .iter()
        .map(|(start, end, kind, owner)| (*start, *end, *kind, (*owner).to_owned()))
        .collect::<Vec<_>>();
    assert_eq!(project(ownership.parsed_claims()), expected, "{text:?}");
    assert_eq!(project(ownership.rendered_claims()), expected, "{text:?}");
    assert_eq!(ownership.rendered_text(), text);
    assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
    assert!(ownership.summary().covered(), "{text:?}: {ownership:?}");
}

#[derive(Default)]
struct TargetAsNounAstVisitor {
    determined_nominals: usize,
    zero_headed_nominals: usize,
    target_modifiers: usize,
}

impl Visitor for TargetAsNounAstVisitor {
    fn visit_determined_nominal(&mut self, value: &deckmaste_english_v2::ast::DeterminedNominal) {
        self.determined_nominals += 1;
        if matches!(value.det(), deckmaste_english_v2::ast::Determiner::Zero) {
            self.zero_headed_nominals += 1;
        }
        deckmaste_english_v2::visit::walk_determined_nominal(self, value);
    }

    fn visit_nominal_modifier(&mut self, value: &deckmaste_english_v2::ast::NominalModifier) {
        if matches!(
            value,
            deckmaste_english_v2::ast::NominalModifier::CommonNounModifier(
                deckmaste_english_v2::ast::CommonNounModifier {
                    noun: CommonNoun::Target,
                }
            )
        ) {
            self.target_modifiers += 1;
        }
        deckmaste_english_v2::visit::walk_nominal_modifier(self, value);
    }
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "one table keeps the quantified other-target AST and visitor cases comparable"
)]
fn other_target_plurals_require_explicit_quantification_with_exact_semantics() {
    let parser = parser();
    let context = context("Context Card");
    assert_bare_other_target_plurals_reject(&parser, &context);

    let path_prefix = [
        "AbilityPlain",
        "AbilityBodySentences",
        "SentenceImperative",
        "VerbPhraseBaseVerbPhrase",
        "TransitiveFrameTransitivePredicate",
        "ObjectObjectNominal",
        "NounPhraseQualifiedNounPhrase",
        "NumericStageUnqualifiedNumericStage",
        "LocativeStageUnqualifiedLocativeStage",
        "ControllerStageUnqualifiedControllerStage",
    ];
    let visitor_prefix = [
        "Ability",
        "Plain",
        "Sentence",
        "Imperative",
        "VerbPhrase",
        "TransitivePredicate",
        "Declaration:KeywordAction:Destroy",
        "Object",
        "NominalObject",
        "NounPhrase",
        "QualifiedNounPhrase",
        "NumericStage",
        "UnqualifiedNumericStage",
        "LocativeStage",
        "UnqualifiedLocativeStage",
        "ControllerStage",
        "UnqualifiedControllerStage",
        "UnqualifiedReference",
    ];
    for (text, path_suffix, visitor_suffix) in [
        (
            "Destroy all other target creatures.",
            &[
                "UnqualifiedReferenceAllReference",
                "PluralSelectorOtherTargetPluralSelector",
                "PluralNominalBarePluralNominal",
                "PluralHeadTypePluralHead",
            ][..],
            &[
                "AllReference",
                "PluralSelector",
                "OtherTargetPluralSelector",
                "PluralNominal",
                "BarePluralNominal",
                "PluralHead",
                "TypePluralHead",
                "Declaration:Type:Creature",
            ][..],
        ),
        (
            "Destroy two other target creatures.",
            &[
                "UnqualifiedReferenceFixedReference",
                "CardinalQuantityCardinal",
                "PluralSelectorOtherTargetPluralSelector",
                "PluralNominalBarePluralNominal",
                "PluralHeadTypePluralHead",
            ][..],
            &[
                "FixedReference",
                "CardinalQuantity",
                "CardinalQuantityValue",
                "CardinalNumber:2",
                "PluralSelector",
                "OtherTargetPluralSelector",
                "PluralNominal",
                "BarePluralNominal",
                "PluralHead",
                "TypePluralHead",
                "Declaration:Type:Creature",
            ][..],
        ),
        (
            "Destroy X other target creatures.",
            &[
                "UnqualifiedReferenceVariableReference",
                "PluralSelectorOtherTargetPluralSelector",
                "PluralNominalBarePluralNominal",
                "PluralHeadTypePluralHead",
            ][..],
            &[
                "VariableReference",
                "Variable:X",
                "PluralSelector",
                "OtherTargetPluralSelector",
                "PluralNominal",
                "BarePluralNominal",
                "PluralHead",
                "TypePluralHead",
                "Declaration:Type:Creature",
            ][..],
        ),
        (
            "Destroy up to three other target creatures.",
            &[
                "UnqualifiedReferenceUpToManyReference",
                "CardinalQuantityCardinal",
                "PluralSelectorOtherTargetPluralSelector",
                "PluralNominalBarePluralNominal",
                "PluralHeadTypePluralHead",
            ][..],
            &[
                "UpToManyReference",
                "CardinalQuantity",
                "CardinalQuantityValue",
                "CardinalNumber:3",
                "PluralSelector",
                "OtherTargetPluralSelector",
                "PluralNominal",
                "BarePluralNominal",
                "PluralHead",
                "TypePluralHead",
                "Declaration:Type:Creature",
            ][..],
        ),
        (
            "Destroy any number of other target Equipment.",
            &[
                "UnqualifiedReferenceAnyNumberReference",
                "PluralSelectorOtherTargetPluralSelector",
                "PluralNominalBarePluralNominal",
                "PluralHeadArtifactSubtypePluralHead",
            ][..],
            &[
                "AnyNumberReference",
                "PluralSelector",
                "OtherTargetPluralSelector",
                "PluralNominal",
                "BarePluralNominal",
                "PluralHead",
                "ArtifactSubtypePluralHead",
                "Declaration:Subtype(Artifact):Equipment",
            ][..],
        ),
    ] {
        assert_quantified_other_target_case(
            &parser,
            &context,
            text,
            &path_prefix,
            path_suffix,
            &visitor_prefix,
            visitor_suffix,
        );
    }
}

fn assert_bare_other_target_plurals_reject(parser: &Parser, context: &ParseContext<'_>) {
    for text in [
        "Destroy other target creatures.",
        "Destroy other target Equipment.",
    ] {
        assert!(
            parser.parse(text, context).is_err(),
            "bare other-target plural must not bypass the explicit-quantifier boundary: {text:?}",
        );
    }
}

fn assert_quantified_other_target_case(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    path_prefix: &[&str],
    path_suffix: &[&str],
    visitor_prefix: &[&str],
    visitor_suffix: &[&str],
) {
    let analysis = parser.analyze(text, context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("quantified other-target plural must select: {analysis:?}"));
    let decision = analysis.decision().expect("selected parse has a decision");
    assert_eq!(decision.candidates().len(), 1, "{text:?}");
    assert_eq!(
        decision.resolution(),
        SelectionResolution::Unique,
        "{text:?}"
    );
    assert!(decision.exception_uses().is_empty(), "{text:?}");
    assert_eq!(
        decision.candidates()[0].construction_path(),
        path_prefix
            .iter()
            .copied()
            .chain(path_suffix.iter().copied())
            .collect::<Vec<_>>(),
        "exact quantified AST path changed for {text:?}",
    );
    assert_eq!(selected.render(context, parser.environment()), text);

    let mut visitor = NominalVisitor::default();
    visitor.visit_ability(selected);
    assert_eq!(
        visitor.events,
        visitor_prefix
            .iter()
            .copied()
            .chain(visitor_suffix.iter().copied())
            .collect::<Vec<_>>(),
        "exact quantified visitor preorder changed for {text:?}",
    );

    let ownership = analysis.ownership().expect("selected parse owns its bytes");
    assert_eq!(ownership.rendered_text(), text);
    assert!(ownership.failures().is_empty(), "{text:?}: {ownership:?}");
    let summary = ownership.summary();
    assert!(summary.covered(), "{text:?}: {ownership:?}");
    assert_eq!(summary.gap_spans(), 0, "{text:?}");
    assert_eq!(summary.overlap_spans(), 0, "{text:?}");
    assert_eq!(summary.synthetic_claims(), 0, "{text:?}");
    assert_eq!(summary.provenance_plan_mismatches(), 0, "{text:?}");
}

#[test]
fn personal_pronoun_case_and_chosen_quality_references_are_typed() {
    let parser = parser();
    let context = context("Context Card");

    for text in [
        "He gains 2 life.",
        "She gains 2 life.",
        "Context Card deals 2 damage to him.",
        "Context Card deals 2 damage to her.",
        "Destroy the chosen color.",
        "Destroy the chosen type.",
        "Destroy the chosen name.",
    ] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text:?} must select: {error:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    for text in [
        "Him gains 2 life.",
        "Her gains 2 life.",
        "Context Card deals 2 damage to he.",
        "Context Card deals 2 damage to she.",
        "Those color gains 2 life.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must respect pronoun case and nominal agreement",
        );
    }
}

#[test]
fn exact_subtype_noun_codecs_accept_only_their_declared_family() {
    use macro_ron::v2::DeclarationIdentity;
    use macro_ron::v2::DeclarationKind;
    use macro_ron::v2::SubtypeCategory;

    let parser = parser();
    let environment = parser.environment();
    let artifact =
        DeclarationIdentity::new(DeclarationKind::Subtype(SubtypeCategory::Artifact), "Clue");
    let battle =
        DeclarationIdentity::new(DeclarationKind::Subtype(SubtypeCategory::Battle), "Siege");
    let creature =
        DeclarationIdentity::new(DeclarationKind::Subtype(SubtypeCategory::Creature), "Elf");
    let enchantment = DeclarationIdentity::new(
        DeclarationKind::Subtype(SubtypeCategory::Enchantment),
        "Aura",
    );
    let land = DeclarationIdentity::new(DeclarationKind::Subtype(SubtypeCategory::Land), "Forest");
    let planeswalker = DeclarationIdentity::new(
        DeclarationKind::Subtype(SubtypeCategory::Planeswalker),
        "Jace",
    );
    let spell =
        DeclarationIdentity::new(DeclarationKind::Subtype(SubtypeCategory::Spell), "Arcane");

    macro_rules! assert_exact_family {
        ($noun:ty, $accepted:expr; $($rejected:expr),+ $(,)?) => {{
            assert!(<$noun>::new(environment, $accepted.clone()).is_some());
            $(
                assert!(
                    <$noun>::new(environment, $rejected.clone()).is_none(),
                    "a generated exact subtype terminal admitted a cross-family identity",
                );
            )+
        }};
    }

    assert_exact_family!(
        deckmaste_english_v2::ast::DeclarationArtifactSubtypeNoun,
        artifact;
        battle,
        creature,
        enchantment,
        land,
        planeswalker,
        spell,
    );
    assert_exact_family!(
        deckmaste_english_v2::ast::DeclarationBattleSubtypeNoun,
        battle;
        artifact,
        creature,
        enchantment,
        land,
        planeswalker,
        spell,
    );
    assert_exact_family!(
        deckmaste_english_v2::ast::DeclarationCreatureSubtypeNoun,
        creature;
        artifact,
        battle,
        enchantment,
        land,
        planeswalker,
        spell,
    );
    assert_exact_family!(
        deckmaste_english_v2::ast::DeclarationEnchantmentSubtypeNoun,
        enchantment;
        artifact,
        battle,
        creature,
        land,
        planeswalker,
        spell,
    );
    assert_exact_family!(
        deckmaste_english_v2::ast::DeclarationLandSubtypeNoun,
        land;
        artifact,
        battle,
        creature,
        enchantment,
        planeswalker,
        spell,
    );
    assert_exact_family!(
        deckmaste_english_v2::ast::DeclarationPlaneswalkerSubtypeNoun,
        planeswalker;
        artifact,
        battle,
        creature,
        enchantment,
        land,
        spell,
    );
    assert_exact_family!(
        deckmaste_english_v2::ast::DeclarationSpellSubtypeNoun,
        spell;
        artifact,
        battle,
        creature,
        enchantment,
        land,
        planeswalker,
    );
}

#[test]
fn authentic_nominal_and_full_np_coordination_surfaces_parse() {
    let parser = parser();

    for (card_name, text) in [
        ("Naturalize", "Destroy target artifact or enchantment."),
        (
            "Bedevil",
            "Destroy target artifact, creature, or planeswalker.",
        ),
        ("Desist", "Destroy all artifacts and enchantments."),
        (
            "Context Card",
            "Destroy any number of target artifacts and/or enchantments.",
        ),
        (
            "Context Card",
            "Destroy target artifact and target enchantment.",
        ),
        (
            "Decimate",
            "Destroy target artifact, target creature, target enchantment, and target land.",
        ),
        (
            "Context Card",
            "Destroy target nonartifact, nonblack creature.",
        ),
    ] {
        let context = context(card_name);
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text:?} must select: {error:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }
}

#[test]
fn common_noun_modifiers_compose_under_a_shared_target_selector() {
    let parser = parser();
    let context = context("Context Card");

    for text in [
        "Destroy target card creature or artifact.",
        "Destroy target controller creature or artifact.",
        "Destroy target opponent creature or artifact.",
        "Destroy target owner creature or artifact.",
        "Destroy target permanent creature or artifact.",
        "Destroy target player creature or artifact.",
        "Destroy target source creature or artifact.",
        "Destroy target spell creature or artifact.",
        "Destroy target token creature or artifact.",
        "Destroy target permanent card or creature card.",
    ] {
        let analysis = parser.analyze(text, &context);
        let parsed = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        let decision = analysis.decision().expect("selected parse has a decision");
        assert!(
            !decision.candidates().is_empty(),
            "candidate census for {text:?}"
        );
        assert!(matches!(
            decision.resolution(),
            SelectionResolution::Unique | SelectionResolution::Specificity
        ));
        assert_eq!(decision.selected(), Some(0));
        assert!(decision.exception_uses().is_empty());
        assert!(
            decision.candidates()[0]
                .construction_path()
                .iter()
                .any(|node| node == "CoordinatedNominalModifierCoordinatedModifierMember"),
            "the ordinary coordinated modifier boundary must own {text:?}",
        );
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    let full_np = "Destroy target artifact or target creature.";
    let analysis = parser.analyze(full_np, &context);
    let parsed = analysis
        .selected()
        .unwrap_or_else(|| panic!("{full_np:?} must select: {analysis:?}"));
    let decision = analysis.decision().expect("selected parse has a decision");
    assert_eq!(decision.candidates().len(), 1);
    assert_eq!(decision.resolution(), SelectionResolution::Unique);
    assert_eq!(decision.selected(), Some(0));
    assert_eq!(
        decision.candidates()[0].construction_path(),
        [
            "AbilityPlain",
            "AbilityBodySentences",
            "SentenceImperative",
            "VerbPhraseBaseVerbPhrase",
            "TransitiveFrameTransitivePredicate",
            "ObjectObjectNominal",
            "NounPhraseQualifiedNounPhrase",
            "NumericStageUnqualifiedNumericStage",
            "LocativeStageUnqualifiedLocativeStage",
            "ControllerStageUnqualifiedControllerStage",
            "UnqualifiedReferenceCoordinatedNounPhrase",
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            "UnqualifiedReferenceDeterminedNominal",
            "DeterminativeSingularSimpleDeterminative",
            "NominalSingularNominalValue",
            "SingularNominalBareSingularNominal",
            "SingularHeadTypeSingularHead",
            "UnqualifiedReferenceDeterminedNominal",
            "DeterminativeSingularSimpleDeterminative",
            "NominalSingularNominalValue",
            "SingularNominalBareSingularNominal",
            "SingularHeadTypeSingularHead",
        ],
        "a repeated target belongs to two full noun phrases, never one shared selector",
    );
    assert!(decision.comparisons().is_empty());
    assert!(decision.exception_uses().is_empty());
    assert_eq!(parsed.render(&context, parser.environment()), full_np);
}

#[allow(
    clippy::too_many_lines,
    reason = "the complete 9-product by 3-arity surface matrix is deliberately literal"
)]
#[test]
fn every_coordination_product_has_exact_binary_three_and_four_member_surfaces() {
    let parser = parser();
    let context = context("Context Card");
    let rows = [
        (
            "shared singular and",
            2,
            "SingularNominalCoordinationSingularAndNominalCoordination",
            "Destroy target artifact and creature.",
        ),
        (
            "shared singular and",
            3,
            "SingularNominalCoordinationSingularAndNominalCoordination",
            "Destroy target artifact, creature, and planeswalker.",
        ),
        (
            "shared singular and",
            4,
            "SingularNominalCoordinationSingularAndNominalCoordination",
            "Destroy target artifact, creature, enchantment, and land.",
        ),
        (
            "shared singular or",
            2,
            "SingularNominalCoordinationSingularOrNominalCoordination",
            "Destroy target artifact or creature.",
        ),
        (
            "shared singular or",
            3,
            "SingularNominalCoordinationSingularOrNominalCoordination",
            "Destroy target artifact, creature, or planeswalker.",
        ),
        (
            "shared singular or",
            4,
            "SingularNominalCoordinationSingularOrNominalCoordination",
            "Destroy target artifact, creature, enchantment, or land.",
        ),
        (
            "shared singular and/or",
            2,
            "SingularNominalCoordinationSingularAndOrNominalCoordination",
            "Destroy target artifact and/or creature.",
        ),
        (
            "shared singular and/or",
            3,
            "SingularNominalCoordinationSingularAndOrNominalCoordination",
            "Destroy target artifact, creature, and/or planeswalker.",
        ),
        (
            "shared singular and/or",
            4,
            "SingularNominalCoordinationSingularAndOrNominalCoordination",
            "Destroy target artifact, creature, enchantment, and/or land.",
        ),
        (
            "shared plural and",
            2,
            "PluralNominalCoordinationPluralAndNominalCoordination",
            "Destroy all artifacts and creatures.",
        ),
        (
            "shared plural and",
            3,
            "PluralNominalCoordinationPluralAndNominalCoordination",
            "Destroy all artifacts, creatures, and planeswalkers.",
        ),
        (
            "shared plural and",
            4,
            "PluralNominalCoordinationPluralAndNominalCoordination",
            "Destroy all artifacts, creatures, enchantments, and lands.",
        ),
        (
            "shared plural or",
            2,
            "PluralNominalCoordinationPluralOrNominalCoordination",
            "Destroy all artifacts or creatures.",
        ),
        (
            "shared plural or",
            3,
            "PluralNominalCoordinationPluralOrNominalCoordination",
            "Destroy all artifacts, creatures, or planeswalkers.",
        ),
        (
            "shared plural or",
            4,
            "PluralNominalCoordinationPluralOrNominalCoordination",
            "Destroy all artifacts, creatures, enchantments, or lands.",
        ),
        (
            "shared plural and/or",
            2,
            "PluralNominalCoordinationPluralAndOrNominalCoordination",
            "Destroy all artifacts and/or creatures.",
        ),
        (
            "shared plural and/or",
            3,
            "PluralNominalCoordinationPluralAndOrNominalCoordination",
            "Destroy all artifacts, creatures, and/or planeswalkers.",
        ),
        (
            "shared plural and/or",
            4,
            "PluralNominalCoordinationPluralAndOrNominalCoordination",
            "Destroy all artifacts, creatures, enchantments, and/or lands.",
        ),
        (
            "full NP and",
            2,
            "FullNounPhraseCoordinationFullAndNounPhraseCoordination",
            "Destroy target artifact and target creature.",
        ),
        (
            "full NP and",
            3,
            "FullNounPhraseCoordinationFullAndNounPhraseCoordination",
            "Destroy target artifact, target creature, and target planeswalker.",
        ),
        (
            "full NP and",
            4,
            "FullNounPhraseCoordinationFullAndNounPhraseCoordination",
            "Destroy target artifact, target creature, target enchantment, and target land.",
        ),
        (
            "full NP or",
            2,
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            "Destroy target artifact or target creature.",
        ),
        (
            "full NP or",
            3,
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            "Destroy target artifact, target creature, or target planeswalker.",
        ),
        (
            "full NP or",
            4,
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            "Destroy target artifact, target creature, target enchantment, or target land.",
        ),
        (
            "full NP and/or",
            2,
            "FullNounPhraseCoordinationFullAndOrNounPhraseCoordination",
            "Destroy target artifact and/or target creature.",
        ),
        (
            "full NP and/or",
            3,
            "FullNounPhraseCoordinationFullAndOrNounPhraseCoordination",
            "Destroy target artifact, target creature, and/or target planeswalker.",
        ),
        (
            "full NP and/or",
            4,
            "FullNounPhraseCoordinationFullAndOrNounPhraseCoordination",
            "Destroy target artifact, target creature, target enchantment, and/or target land.",
        ),
    ];

    for (product, arity, expected_path_node, text) in rows {
        let analysis = parser.analyze(text, &context);
        let parsed = analysis
            .selected()
            .unwrap_or_else(|| panic!("{product} arity {arity} must select: {analysis:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
        let decision = analysis.decision().expect("selected parse has a decision");
        let selected = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("selected ordinal is retained");
        assert!(
            selected
                .construction_path()
                .iter()
                .any(|node| node == expected_path_node),
            "wrong product for {product} arity {arity}: {:?}",
            selected.construction_path(),
        );
    }
}

#[derive(Default)]
struct CoordinationVisitor {
    events: Vec<&'static str>,
}

impl Visitor for CoordinationVisitor {
    fn visit_negative_nominal_modifier(
        &mut self,
        value: &deckmaste_english_v2::ast::NegativeNominalModifier,
    ) {
        self.events.push("NegativeNominalModifier");
        deckmaste_english_v2::visit::walk_negative_nominal_modifier(self, value);
    }

    fn visit_coordinated_nominal_modifier(
        &mut self,
        value: &deckmaste_english_v2::ast::CoordinatedNominalModifier,
    ) {
        self.events.push("CoordinatedNominalModifier");
        deckmaste_english_v2::visit::walk_coordinated_nominal_modifier(self, value);
    }

    fn visit_singular_coordination_member(
        &mut self,
        value: &deckmaste_english_v2::ast::SingularCoordinationMember,
    ) {
        self.events.push("SingularCoordinationMember");
        deckmaste_english_v2::visit::walk_singular_coordination_member(self, value);
    }

    fn visit_plural_coordination_member(
        &mut self,
        value: &deckmaste_english_v2::ast::PluralCoordinationMember,
    ) {
        self.events.push("PluralCoordinationMember");
        deckmaste_english_v2::visit::walk_plural_coordination_member(self, value);
    }

    fn visit_singular_nominal_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::SingularNominalCoordination,
    ) {
        self.events.push("SingularNominalCoordination");
        deckmaste_english_v2::visit::walk_singular_nominal_coordination(self, value);
    }

    fn visit_plural_nominal_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::PluralNominalCoordination,
    ) {
        self.events.push("PluralNominalCoordination");
        deckmaste_english_v2::visit::walk_plural_nominal_coordination(self, value);
    }

    fn visit_determined_nominal(
        &mut self,
        value: &deckmaste_english_v2::ast::DeterminedNominal,
    ) {
        self.events.push("DeterminedNominal");
        deckmaste_english_v2::visit::walk_determined_nominal(self, value);
    }

    fn visit_determinative(&mut self, value: &deckmaste_english_v2::ast::Determinative) {
        self.events.push("Determinative");
        deckmaste_english_v2::visit::walk_determinative(self, value);
    }

    fn visit_nominal(&mut self, value: &deckmaste_english_v2::ast::Nominal) {
        self.events.push("Nominal");
        deckmaste_english_v2::visit::walk_nominal(self, value);
    }

    fn visit_full_noun_phrase_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::FullNounPhraseCoordination,
    ) {
        self.events.push("FullNounPhraseCoordination");
        deckmaste_english_v2::visit::walk_full_noun_phrase_coordination(self, value);
    }

    fn visit_negative_modifier_member(
        &mut self,
        value: &deckmaste_english_v2::ast::NegativeModifierMember,
    ) {
        self.events.push("NegativeModifierMember");
        deckmaste_english_v2::visit::walk_negative_modifier_member(self, value);
    }

    fn visit_coordinated_modifier_member(
        &mut self,
        value: &deckmaste_english_v2::ast::CoordinatedModifierMember,
    ) {
        self.events.push("CoordinatedModifierMember");
        deckmaste_english_v2::visit::walk_coordinated_modifier_member(self, value);
    }

    fn visit_negative_modified_singular_nominal(
        &mut self,
        value: &deckmaste_english_v2::ast::NegativeModifiedSingularNominal,
    ) {
        self.events.push("NegativeModifiedSingularNominal");
        deckmaste_english_v2::visit::walk_negative_modified_singular_nominal(self, value);
    }

    fn visit_negative_modified_plural_nominal(
        &mut self,
        value: &deckmaste_english_v2::ast::NegativeModifiedPluralNominal,
    ) {
        self.events.push("NegativeModifiedPluralNominal");
        deckmaste_english_v2::visit::walk_negative_modified_plural_nominal(self, value);
    }

    fn visit_bare_singular_coordination_member(
        &mut self,
        value: &deckmaste_english_v2::ast::BareSingularCoordinationMember,
    ) {
        self.events.push("BareSingularCoordinationMember");
        deckmaste_english_v2::visit::walk_bare_singular_coordination_member(self, value);
    }

    fn visit_modified_singular_coordination_member(
        &mut self,
        value: &deckmaste_english_v2::ast::ModifiedSingularCoordinationMember,
    ) {
        self.events.push("ModifiedSingularCoordinationMember");
        deckmaste_english_v2::visit::walk_modified_singular_coordination_member(self, value);
    }

    fn visit_negative_modified_singular_coordination_member(
        &mut self,
        value: &deckmaste_english_v2::ast::NegativeModifiedSingularCoordinationMember,
    ) {
        self.events
            .push("NegativeModifiedSingularCoordinationMember");
        deckmaste_english_v2::visit::walk_negative_modified_singular_coordination_member(
            self, value,
        );
    }

    fn visit_bare_plural_coordination_member(
        &mut self,
        value: &deckmaste_english_v2::ast::BarePluralCoordinationMember,
    ) {
        self.events.push("BarePluralCoordinationMember");
        deckmaste_english_v2::visit::walk_bare_plural_coordination_member(self, value);
    }

    fn visit_modified_plural_coordination_member(
        &mut self,
        value: &deckmaste_english_v2::ast::ModifiedPluralCoordinationMember,
    ) {
        self.events.push("ModifiedPluralCoordinationMember");
        deckmaste_english_v2::visit::walk_modified_plural_coordination_member(self, value);
    }

    fn visit_negative_modified_plural_coordination_member(
        &mut self,
        value: &deckmaste_english_v2::ast::NegativeModifiedPluralCoordinationMember,
    ) {
        self.events.push("NegativeModifiedPluralCoordinationMember");
        deckmaste_english_v2::visit::walk_negative_modified_plural_coordination_member(self, value);
    }

    fn visit_singular_and_nominal_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::SingularAndNominalCoordination,
    ) {
        self.events.push("SingularAndNominalCoordination");
        deckmaste_english_v2::visit::walk_singular_and_nominal_coordination(self, value);
    }

    fn visit_singular_or_nominal_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::SingularOrNominalCoordination,
    ) {
        self.events.push("SingularOrNominalCoordination");
        deckmaste_english_v2::visit::walk_singular_or_nominal_coordination(self, value);
    }

    fn visit_singular_and_or_nominal_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::SingularAndOrNominalCoordination,
    ) {
        self.events.push("SingularAndOrNominalCoordination");
        deckmaste_english_v2::visit::walk_singular_and_or_nominal_coordination(self, value);
    }

    fn visit_plural_and_nominal_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::PluralAndNominalCoordination,
    ) {
        self.events.push("PluralAndNominalCoordination");
        deckmaste_english_v2::visit::walk_plural_and_nominal_coordination(self, value);
    }

    fn visit_plural_or_nominal_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::PluralOrNominalCoordination,
    ) {
        self.events.push("PluralOrNominalCoordination");
        deckmaste_english_v2::visit::walk_plural_or_nominal_coordination(self, value);
    }

    fn visit_plural_and_or_nominal_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::PluralAndOrNominalCoordination,
    ) {
        self.events.push("PluralAndOrNominalCoordination");
        deckmaste_english_v2::visit::walk_plural_and_or_nominal_coordination(self, value);
    }

    fn visit_target_singular_coordination_selector(
        &mut self,
        value: &deckmaste_english_v2::ast::TargetSingularCoordinationSelector,
    ) {
        self.events.push("TargetSingularCoordinationSelector");
        deckmaste_english_v2::visit::walk_target_singular_coordination_selector(self, value);
    }

    fn visit_unmarked_plural_coordination_selector(
        &mut self,
        value: &deckmaste_english_v2::ast::UnmarkedPluralCoordinationSelector,
    ) {
        self.events.push("UnmarkedPluralCoordinationSelector");
        deckmaste_english_v2::visit::walk_unmarked_plural_coordination_selector(self, value);
    }

    fn visit_target_plural_coordination_selector(
        &mut self,
        value: &deckmaste_english_v2::ast::TargetPluralCoordinationSelector,
    ) {
        self.events.push("TargetPluralCoordinationSelector");
        deckmaste_english_v2::visit::walk_target_plural_coordination_selector(self, value);
    }

    fn visit_full_and_noun_phrase_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::FullAndNounPhraseCoordination,
    ) {
        self.events.push("FullAndNounPhraseCoordination");
        deckmaste_english_v2::visit::walk_full_and_noun_phrase_coordination(self, value);
    }

    fn visit_full_or_noun_phrase_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::FullOrNounPhraseCoordination,
    ) {
        self.events.push("FullOrNounPhraseCoordination");
        deckmaste_english_v2::visit::walk_full_or_noun_phrase_coordination(self, value);
    }

    fn visit_full_and_or_noun_phrase_coordination(
        &mut self,
        value: &deckmaste_english_v2::ast::FullAndOrNounPhraseCoordination,
    ) {
        self.events.push("FullAndOrNounPhraseCoordination");
        deckmaste_english_v2::visit::walk_full_and_or_noun_phrase_coordination(self, value);
    }

    fn visit_coordinated_noun_phrase(
        &mut self,
        value: &deckmaste_english_v2::ast::CoordinatedNounPhrase,
    ) {
        self.events.push("CoordinatedNounPhrase");
        deckmaste_english_v2::visit::walk_coordinated_noun_phrase(self, value);
    }
}

fn assert_coordination_evidence(
    parser: &Parser,
    card_name: &str,
    text: &str,
    expected_path: &[&str],
    expected_specificity: &[SpecificityTier],
    visitor_events: &[&str],
) {
    let context = context(card_name);
    let analysis = parser.analyze(text, &context);
    let parsed = analysis
        .selected()
        .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
    let decision = analysis
        .decision()
        .expect("a selected parse has a decision");
    assert_eq!(decision.candidates().len(), 1);
    assert_eq!(decision.resolution(), SelectionResolution::Unique);
    assert_eq!(decision.selected(), Some(0));
    assert_eq!(decision.survivors(), [0]);
    assert!(decision.comparisons().is_empty());
    assert!(decision.exception_uses().is_empty());
    let selected = &decision.candidates()[0];
    assert_eq!(selected.ordinal(), 0);
    assert_eq!(selected.construction_path(), expected_path);
    assert_eq!(selected.specificity(), expected_specificity);

    assert_eq!(parsed.render(&context, parser.environment()), text);
    let ownership = analysis.ownership().expect("selected parse owns its bytes");
    assert!(ownership.failures().is_empty());
    assert!(ownership.summary().covered());
    assert_eq!(ownership.rendered_text(), text);

    let mut visitor = CoordinationVisitor::default();
    visitor.visit_ability(parsed);
    assert_eq!(
        visitor.events, visitor_events,
        "visitor preorder changed for {text:?}"
    );
}

#[test]
fn coordination_ast_scope_traversal_ownership_and_ambiguity_are_exact() {
    let parser = parser();
    for (card_name, text, expected_path, expected_specificity, visitor_events) in [
        (
            "Bedevil",
            "Destroy target artifact, creature, or planeswalker.",
            &[
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveFrameTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "NumericStageUnqualifiedNumericStage",
                "LocativeStageUnqualifiedLocativeStage",
                "ControllerStageUnqualifiedControllerStage",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeSingularSimpleDeterminative",
                "NominalSingularCoordinationNominalValue",
                "SingularNominalCoordinationSingularOrNominalCoordination",
                "SingularCoordinationMemberBareSingularCoordinationMember",
                "SingularHeadTypeSingularHead",
                "SingularCoordinationMemberBareSingularCoordinationMember",
                "SingularHeadTypeSingularHead",
                "SingularCoordinationMemberBareSingularCoordinationMember",
                "SingularHeadTypeSingularHead",
            ][..],
            &[
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Literal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Literal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
            ][..],
            &[
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "SingularNominalCoordination",
                "SingularOrNominalCoordination",
                "SingularCoordinationMember",
                "BareSingularCoordinationMember",
                "SingularCoordinationMember",
                "BareSingularCoordinationMember",
                "SingularCoordinationMember",
                "BareSingularCoordinationMember",
            ][..],
        ),
        (
            "Decimate",
            "Destroy target artifact, target creature, target enchantment, and target land.",
            &[
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveFrameTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "NumericStageUnqualifiedNumericStage",
                "LocativeStageUnqualifiedLocativeStage",
                "ControllerStageUnqualifiedControllerStage",
                "UnqualifiedReferenceCoordinatedNounPhrase",
                "FullNounPhraseCoordinationFullAndNounPhraseCoordination",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeSingularSimpleDeterminative",
                "NominalSingularNominalValue",
                "SingularNominalBareSingularNominal",
                "SingularHeadTypeSingularHead",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeSingularSimpleDeterminative",
                "NominalSingularNominalValue",
                "SingularNominalBareSingularNominal",
                "SingularHeadTypeSingularHead",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeSingularSimpleDeterminative",
                "NominalSingularNominalValue",
                "SingularNominalBareSingularNominal",
                "SingularHeadTypeSingularHead",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeSingularSimpleDeterminative",
                "NominalSingularNominalValue",
                "SingularNominalBareSingularNominal",
                "SingularHeadTypeSingularHead",
            ][..],
            &[
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Literal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Literal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Literal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Literal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Literal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
            ][..],
            &[
                "CoordinatedNounPhrase",
                "FullNounPhraseCoordination",
                "FullAndNounPhraseCoordination",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
            ][..],
        ),
    ] {
        assert_coordination_evidence(
            &parser,
            card_name,
            text,
            expected_path,
            expected_specificity,
            visitor_events,
        );
    }
}

#[test]
fn visitor_callbacks_have_literal_full_preorders() {
    let parser = parser();
    let context = context("Context Card");
    for (text, expected) in [
        (
            "Destroy target permanent card or creature card.",
            &[
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "SingularNominalCoordination",
                "SingularOrNominalCoordination",
                "SingularCoordinationMember",
                "ModifiedSingularCoordinationMember",
                "CoordinatedNominalModifier",
                "CoordinatedModifierMember",
                "SingularCoordinationMember",
                "ModifiedSingularCoordinationMember",
                "CoordinatedNominalModifier",
                "CoordinatedModifierMember",
            ][..],
        ),
        (
            "Destroy all artifacts and enchantments.",
            &[
                "UnmarkedPluralCoordinationSelector",
                "PluralNominalCoordination",
                "PluralAndNominalCoordination",
                "PluralCoordinationMember",
                "BarePluralCoordinationMember",
                "PluralCoordinationMember",
                "BarePluralCoordinationMember",
            ][..],
        ),
        (
            "Destroy two target artifacts or creatures.",
            &[
                "TargetPluralCoordinationSelector",
                "PluralNominalCoordination",
                "PluralOrNominalCoordination",
                "PluralCoordinationMember",
                "BarePluralCoordinationMember",
                "PluralCoordinationMember",
                "BarePluralCoordinationMember",
            ][..],
        ),
        (
            "Destroy target artifact and target creature.",
            &[
                "CoordinatedNounPhrase",
                "FullNounPhraseCoordination",
                "FullAndNounPhraseCoordination",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "DeterminedNominal",
                "Determinative",
                "Nominal",
            ][..],
        ),
        (
            "Destroy target nonartifact, nonblack creature.",
            &[
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "NegativeModifiedSingularNominal",
                "NegativeNominalModifier",
                "NegativeModifierMember",
                "NegativeNominalModifier",
                "NegativeModifierMember",
            ][..],
        ),
        (
            "Destroy two target nonartifact, nonblack creatures.",
            &[
                "NegativeModifiedPluralNominal",
                "NegativeNominalModifier",
                "NegativeModifierMember",
                "NegativeNominalModifier",
                "NegativeModifierMember",
            ][..],
        ),
        (
            "Destroy target nonartifact, nonblack creature or artifact.",
            &[
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "SingularNominalCoordination",
                "SingularOrNominalCoordination",
                "SingularCoordinationMember",
                "NegativeModifiedSingularCoordinationMember",
                "NegativeNominalModifier",
                "NegativeModifierMember",
                "NegativeNominalModifier",
                "NegativeModifierMember",
                "SingularCoordinationMember",
                "BareSingularCoordinationMember",
            ][..],
        ),
        (
            "Destroy all nonartifact, nonblack creatures or artifacts.",
            &[
                "UnmarkedPluralCoordinationSelector",
                "PluralNominalCoordination",
                "PluralOrNominalCoordination",
                "PluralCoordinationMember",
                "NegativeModifiedPluralCoordinationMember",
                "NegativeNominalModifier",
                "NegativeModifierMember",
                "NegativeNominalModifier",
                "NegativeModifierMember",
                "PluralCoordinationMember",
                "BarePluralCoordinationMember",
            ][..],
        ),
        (
            "Destroy all token creatures or artifacts.",
            &[
                "UnmarkedPluralCoordinationSelector",
                "PluralNominalCoordination",
                "PluralOrNominalCoordination",
                "PluralCoordinationMember",
                "ModifiedPluralCoordinationMember",
                "CoordinatedNominalModifier",
                "CoordinatedModifierMember",
                "PluralCoordinationMember",
                "BarePluralCoordinationMember",
            ][..],
        ),
    ] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text:?} must select: {error:?}"));
        let mut visitor = CoordinationVisitor::default();
        visitor.visit_ability(&parsed);
        assert_eq!(visitor.events, expected, "preorder changed for {text:?}");
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "scanner and renderer ownership claims are pinned as literal span tables"
)]
#[test]
fn singular_and_plural_negative_modifier_sequences_have_exact_ast_scope() {
    let parser = parser();
    let context = context("Context Card");
    for (text, expected_path, expected_visitor) in [
        (
            "Destroy target nonartifact, nonblack creature.",
            &[
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveFrameTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "NumericStageUnqualifiedNumericStage",
                "LocativeStageUnqualifiedLocativeStage",
                "ControllerStageUnqualifiedControllerStage",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeSingularSimpleDeterminative",
                "NominalSingularNominalValue",
                "SingularNominalNegativeModifiedSingularNominal",
                "NegativeNominalModifierNegativeModifierMember",
                "NominalModifierNonTypeModifier",
                "NegativeNominalModifierNegativeModifierMember",
                "NominalModifierNonColorModifier",
                "SingularHeadTypeSingularHead",
            ][..],
            &[
                "DeterminedNominal",
                "Determinative",
                "Nominal",
                "NegativeModifiedSingularNominal",
                "NegativeNominalModifier",
                "NegativeModifierMember",
                "NegativeNominalModifier",
                "NegativeModifierMember",
            ][..],
        ),
        (
            "Destroy two target nonartifact, nonblack creatures.",
            &[
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveFrameTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "NumericStageUnqualifiedNumericStage",
                "LocativeStageUnqualifiedLocativeStage",
                "ControllerStageUnqualifiedControllerStage",
                "UnqualifiedReferenceFixedReference",
                "CardinalQuantityCardinal",
                "PluralSelectorTargetPluralSelector",
                "PluralNominalNegativeModifiedPluralNominal",
                "NegativeNominalModifierNegativeModifierMember",
                "NominalModifierNonTypeModifier",
                "NegativeNominalModifierNegativeModifierMember",
                "NominalModifierNonColorModifier",
                "PluralHeadTypePluralHead",
            ][..],
            &[
                "NegativeModifiedPluralNominal",
                "NegativeNominalModifier",
                "NegativeModifierMember",
                "NegativeNominalModifier",
                "NegativeModifierMember",
            ][..],
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let parsed = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
        let decision = analysis.decision().expect("selected parse has a decision");
        assert_eq!(decision.candidates().len(), 1);
        assert_eq!(decision.resolution(), SelectionResolution::Unique);
        assert_eq!(decision.selected(), Some(0));
        assert_eq!(decision.candidates()[0].construction_path(), expected_path);

        let mut visitor = CoordinationVisitor::default();
        visitor.visit_ability(parsed);
        assert_eq!(visitor.events, expected_visitor);
    }

    assert!(
        parser
            .parse("Destroy target nonartifact, black creature.", &context,)
            .is_err(),
        "a positive modifier cannot enter a typed negative-modifier sequence",
    );
}

#[test]
fn malformed_coordination_punctuation_and_scoping_are_rejected() {
    let parser = parser();
    let context = context("Context Card");
    for text in [
        "Destroy target artifact, creature or planeswalker.",
        "Destroy target artifact, or enchantment.",
        "Destroy target artifact or or enchantment.",
        "Destroy target artifact, target creature and target land.",
        "Destroy target artifact, and target enchantment.",
        "Destroy target artifact, target creature, or planeswalker.",
        "Destroy target target creature or artifact.",
        "Destroy target nonartifact, nonblack, creature.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must not enter a coordination or modifier-sequence AST",
        );
    }
}

#[test]
fn coordination_minimum_arity_and_agreement_are_unconstructible_when_inconsistent() {
    use deckmaste_english_v2::ast::*;
    use macro_ron::v2::DeclarationIdentity;
    use macro_ron::v2::DeclarationKind;

    let parser = parser();
    let environment = parser.environment();
    let head = |name| {
        SingularHead::TypeSingularHead(TypeSingularHead {
            noun: TypeNoun::Declaration(
                DeclarationTypeNoun::new(
                    environment,
                    DeclarationIdentity::new(DeclarationKind::Type, name),
                )
                .expect("the builtin Type noun is present"),
            ),
        })
    };
    let nominal =
        |name| SingularNominal::BareSingularNominal(BareSingularNominal { head: head(name) });
    let coordination_member = |name| {
        SingularCoordinationMember::BareSingularCoordinationMember(BareSingularCoordinationMember {
            head: head(name),
        })
    };
    let plural_head = |name| {
        PluralHead::TypePluralHead(TypePluralHead {
            noun: TypeNoun::Declaration(
                DeclarationTypeNoun::new(
                    environment,
                    DeclarationIdentity::new(DeclarationKind::Type, name),
                )
                .expect("the builtin Type noun is present"),
            ),
        })
    };
    let plural_coordination_member = |name| {
        PluralCoordinationMember::BarePluralCoordinationMember(BarePluralCoordinationMember {
            head: plural_head(name),
        })
    };

    assert!(SingularAndNominalCoordination::new(vec![coordination_member("Artifact")]).is_none());
    assert!(SingularOrNominalCoordination::new(vec![coordination_member("Artifact")]).is_none());
    assert!(SingularAndOrNominalCoordination::new(vec![coordination_member("Artifact")]).is_none());
    assert!(
        PluralAndNominalCoordination::new(vec![plural_coordination_member("Artifact")]).is_none()
    );
    assert!(
        PluralOrNominalCoordination::new(vec![plural_coordination_member("Artifact")]).is_none()
    );
    assert!(
        PluralAndOrNominalCoordination::new(vec![plural_coordination_member("Artifact")]).is_none()
    );
    let determined = DeterminedNominal::new(
        Determiner::Headed(Determinative::SingularSimpleDeterminative(
            SingularSimpleDeterminative::new(SimpleDeterminative::Target)
                .expect("target is a singular simple determinative"),
        )),
        Nominal::SingularNominalValue(SingularNominalValue {
            nominal: nominal("Artifact"),
        }),
    )
    .expect("target determiner agrees with artifact");
    let determined = UnqualifiedReference::DeterminedNominal(determined);
    assert!(FullAndNounPhraseCoordination::new(Box::new(vec![determined.clone()])).is_none());
    assert!(FullOrNounPhraseCoordination::new(Box::new(vec![determined.clone()])).is_none());
    assert!(FullAndOrNounPhraseCoordination::new(Box::new(vec![determined])).is_none());

    let context = context("Context Card");
    for (text, noun_phrase_path, agreement_summary) in [
        (
            "Target creature or planeswalker gains 2 life.",
            "NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceDeterminedNominal",
            "agreement: ThirdPersonSingular",
        ),
        (
            "Two target creatures or planeswalkers gain 2 life.",
            "NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceFixedReference",
            "agreement: Bare",
        ),
        (
            "Target creature and target planeswalker gain 2 life.",
            "NounPhraseQualifiedNounPhrase/NumericStageUnqualifiedNumericStage/LocativeStageUnqualifiedLocativeStage/ControllerStageUnqualifiedControllerStage/UnqualifiedReferenceCoordinatedNounPhrase",
            "agreement: Bare",
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let parsed = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        let decision = analysis.decision().expect("selected parse has a decision");
        assert_eq!(
            decision.candidates().len(),
            1,
            "candidate census for {text:?}"
        );
        assert_eq!(decision.resolution(), SelectionResolution::Unique);
        assert_eq!(decision.selected(), Some(0));
        assert!(
            decision.candidates()[0]
                .construction_path()
                .join("/")
                .contains(noun_phrase_path),
            "derived Number must retain its exact noun-phrase scope for {text:?}",
        );
        let claims = analysis
            .ownership()
            .expect("selected agreement probe owns its lexical leaves")
            .parsed_claims();
        assert!(
            claims
                .iter()
                .any(|claim| claim.semantic_summary().contains(agreement_summary)),
            "derived Agreement evidence changed for {text:?}: {claims:?}",
        );
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }
    for text in [
        "Target creature or planeswalker gain 2 life.",
        "Target creatures or planeswalkers gain 2 life.",
        "Target creatures or planeswalkers gains 2 life.",
        "Target creature and target planeswalker gains 2 life.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} carries inconsistent shared-selector/full-NP agreement",
        );
    }
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the complete positive and negative compound-modifier matrix is deliberately literal"
)]
fn compound_classifier_nominals_admit_every_positive_modifier_and_reject_negative_ones() {
    struct PositiveWitness {
        text: &'static str,
        modifier_path: &'static str,
        nominal_path: &'static str,
        head_owner: &'static str,
    }

    let parser = parser();
    let context = context("Grammar Witness");
    for witness in [
        PositiveWitness {
            text: "Destroy a blue artifact type.",
            modifier_path: "NominalModifierColorModifier",
            nominal_path: "SingularNominalModifiedSingularNominal",
            head_owner: "lexeme:CommonNoun/Type/singular",
        },
        PositiveWitness {
            text: "Destroy all tapped artifact types.",
            modifier_path: "NominalModifierStatusModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all legendary artifact types.",
            modifier_path: "NominalModifierSupertypeModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all spell artifact types.",
            modifier_path: "NominalModifierCommonNounModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all artifact creature types.",
            modifier_path: "NominalModifierTypeModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Equipment artifact types.",
            modifier_path: "NominalModifierArtifactSubtypeModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Siege battle types.",
            modifier_path: "NominalModifierBattleSubtypeModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Human creature types.",
            modifier_path: "NominalModifierCreatureSubtypeModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Aura enchantment types.",
            modifier_path: "NominalModifierEnchantmentSubtypeModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Plains land types.",
            modifier_path: "NominalModifierLandSubtypeModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Jace planeswalker types.",
            modifier_path: "NominalModifierPlaneswalkerSubtypeModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Arcane spell types.",
            modifier_path: "NominalModifierSpellSubtypeModifier",
            nominal_path: "PluralNominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
    ] {
        let analysis = parser.analyze(witness.text, &context);
        assert_eq!(
            analysis.outcome(),
            ParseAnalysisOutcome::Selected,
            "positive compound classifier must select: {:?}",
            witness.text,
        );
        let decision = analysis
            .decision()
            .expect("selected compound classifier has a decision");
        assert_eq!(decision.candidates().len(), 1, "{:?}", witness.text);
        assert_eq!(decision.resolution(), SelectionResolution::Unique);
        assert_eq!(decision.survivors(), [0]);
        assert_eq!(decision.selected(), Some(0));
        assert!(decision.exception_uses().is_empty());
        let path = decision.candidates()[0].construction_path();
        assert!(path.iter().any(|item| item == witness.modifier_path));
        assert!(
            path.iter().any(|item| item == witness.nominal_path),
            "the ordinary modifier sequence owns {:?}: {path:?}",
            witness.text,
        );

        let selected = analysis
            .selected()
            .expect("positive compound classifier has an AST");
        assert_eq!(
            selected.render(&context, parser.environment()),
            witness.text
        );
        let ownership = analysis
            .ownership()
            .expect("positive compound classifier owns its bytes");
        assert_eq!(ownership.rendered_text(), witness.text);
        assert!(ownership.failures().is_empty());
        let summary = ownership.summary();
        assert!(summary.covered());
        assert_eq!(summary.gap_spans(), 0);
        assert_eq!(summary.overlap_spans(), 0);
        assert_eq!(summary.synthetic_claims(), 0);
        assert_eq!(summary.provenance_plan_mismatches(), 0);
        assert!(
            ownership
                .parsed_claims()
                .iter()
                .any(|claim| claim.stable_owner_id() == witness.head_owner),
            "the ordinary common-noun head owns its lexeme in {:?}: {:?}",
            witness.text,
            ownership.parsed_claims(),
        );
    }

    for text in [
        "Destroy all nonblack artifact types.",
        "Destroy all nontoken artifact types.",
        "Destroy all nontapped artifact types.",
        "Destroy all nonlegendary artifact types.",
        "Destroy all nonartifact creature types.",
        "Destroy all non-Equipment artifact types.",
        "Destroy all non-Siege battle types.",
        "Destroy all non-Human creature types.",
        "Destroy all non-Aura enchantment types.",
        "Destroy all non-Plains land types.",
        "Destroy all non-Jace planeswalker types.",
        "Destroy all non-Arcane spell types.",
    ] {
        let analysis = parser.analyze(text, &context);
        assert_eq!(
            analysis.outcome(),
            ParseAnalysisOutcome::Selected,
            "ordinary negative modifiers compose in classifier sequences: {text:?}",
        );
    }
}

#[test]
fn indefinite_full_noun_phrase_members_derive_each_article_from_their_own_onset() {
    let parser = parser();
    let context = context("Grammar Witness");
    let text = "Whenever a creature and an artifact deal 1 damage to you, you gain 1 life.";
    let analysis = parser.analyze(text, &context);
    assert_eq!(analysis.outcome(), ParseAnalysisOutcome::Selected);
    let decision = analysis
        .decision()
        .expect("full noun-phrase coordination has a decision");
    assert_eq!(decision.candidates().len(), 1);
    assert_eq!(decision.resolution(), SelectionResolution::Unique);
    assert_eq!(decision.survivors(), [0]);
    assert_eq!(decision.selected(), Some(0));
    assert!(decision.exception_uses().is_empty());
    assert_eq!(
        decision.candidates()[0]
            .construction_path()
            .iter()
            .filter(|item| *item == "DeterminativeSingularSimpleDeterminative")
            .count(),
        2,
    );
    let selected = analysis
        .selected()
        .expect("full noun-phrase coordination has an AST");
    assert_eq!(selected.render(&context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("full noun-phrase coordination owns its bytes");
    assert_eq!(ownership.rendered_text(), text);
    assert!(ownership.failures().is_empty());
    let summary = ownership.summary();
    assert!(summary.covered());
    assert_eq!(summary.gap_spans(), 0);
    assert_eq!(summary.overlap_spans(), 0);
    assert_eq!(summary.synthetic_claims(), 0);
    assert_eq!(summary.provenance_plan_mismatches(), 0);
    let article_owners = ownership
        .parsed_claims()
        .iter()
        .filter(|claim| {
            matches!(
                claim.stable_owner_id(),
                "vocab:SimpleDeterminative/A" | "vocab:SimpleDeterminative/An"
            )
        })
        .map(deckmaste_english_v2::parser::LexicalClaim::stable_owner_id)
        .collect::<Vec<_>>();
    assert_eq!(
        article_owners,
        [
            "vocab:SimpleDeterminative/A",
            "vocab:SimpleDeterminative/An",
        ],
    );

    for malformed in [
        "Whenever an creature and an artifact deal 1 damage to you, you gain 1 life.",
        "Whenever a creature and a artifact deal 1 damage to you, you gain 1 life.",
    ] {
        assert_eq!(
            parser.analyze(malformed, &context).outcome(),
            ParseAnalysisOutcome::ParseFailure,
            "each determiner must reject the reciprocal onset: {malformed:?}",
        );
    }
}
