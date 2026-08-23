use std::cmp::Ordering;
use std::path::Path;

use deckmaste_english_v2::ast::CardName;
use deckmaste_english_v2::ast::CardinalNumber;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::ChosenQuality;
use deckmaste_english_v2::ast::Color;
use deckmaste_english_v2::ast::CommonNoun;
use deckmaste_english_v2::ast::Designation;
use deckmaste_english_v2::ast::ObjectPronoun;
use deckmaste_english_v2::ast::PossessiveAbsolutePronoun;
use deckmaste_english_v2::ast::PossessiveDeterminerPronoun;
use deckmaste_english_v2::ast::ReflexivePronoun;
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
    ParseContext::new(name).expect("test card name is a valid parse context")
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

    record_product!(visit_paragraph, Paragraph, walk_paragraph);
    record_product!(visit_imperative, Imperative, walk_imperative);
    record_product!(visit_declarative, Declarative, walk_declarative);
    record_product!(visit_destroy, Destroy, walk_destroy);
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
        visit_indefinite_reference,
        IndefiniteReference,
        walk_indefinite_reference
    );
    record_product!(
        visit_named_card_reference,
        NamedCardReference,
        walk_named_card_reference
    );
    record_product!(
        visit_ordinary_singular_reference,
        OrdinarySingularReference,
        walk_ordinary_singular_reference
    );
    record_product!(
        visit_ordinary_plural_reference,
        OrdinaryPluralReference,
        walk_ordinary_plural_reference
    );
    record_product!(
        visit_definite_singular_reference,
        DefiniteSingularReference,
        walk_definite_singular_reference
    );
    record_product!(
        visit_definite_plural_reference,
        DefinitePluralReference,
        walk_definite_plural_reference
    );
    record_product!(
        visit_any_target_reference,
        AnyTargetReference,
        walk_any_target_reference
    );
    record_product!(
        visit_another_reference,
        AnotherReference,
        walk_another_reference
    );
    record_product!(visit_each_reference, EachReference, walk_each_reference);
    record_product!(visit_all_reference, AllReference, walk_all_reference);
    record_product!(visit_fixed_reference, FixedReference, walk_fixed_reference);
    record_product!(
        visit_variable_reference,
        VariableReference,
        walk_variable_reference
    );
    record_product!(
        visit_up_to_one_reference,
        UpToOneReference,
        walk_up_to_one_reference
    );
    record_product!(
        visit_up_to_many_reference,
        UpToManyReference,
        walk_up_to_many_reference
    );
    record_product!(
        visit_any_number_reference,
        AnyNumberReference,
        walk_any_number_reference
    );
    record_product!(
        visit_one_or_more_reference,
        OneOrMoreReference,
        walk_one_or_more_reference
    );
    record_product!(visit_this_reference, ThisReference, walk_this_reference);
    record_product!(visit_that_reference, ThatReference, walk_that_reference);
    record_product!(visit_those_reference, ThoseReference, walk_those_reference);
    record_product!(
        visit_designated_singular_reference,
        DesignatedSingularReference,
        walk_designated_singular_reference
    );
    record_product!(
        visit_designated_plural_reference,
        DesignatedPluralReference,
        walk_designated_plural_reference
    );
    record_product!(
        visit_chosen_quality_reference,
        ChosenQualityReference,
        walk_chosen_quality_reference
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
    record_product!(visit_count_np, CountNp, walk_count_np);
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

    fn visit_designation(&mut self, value: Designation) {
        self.events.push(format!("Designation:{value:?}"));
    }

    fn visit_chosen_quality(&mut self, value: ChosenQuality) {
        self.events.push(format!("ChosenQuality:{value:?}"));
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
    reason = "Task 8 pins one literal ordered visitor vector per authentic positive"
)]
fn expected_visitor_events(text: &str) -> &'static [&'static str] {
    match text {
        "Destroy target permanent." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "CommonSingularHead",
            "CommonNoun:Permanent",
        ],
        "Destroy target Spirit." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
            "SingularNominal",
            "BareSingularNominal",
            "SingularHead",
            "CreatureSubtypeSingularHead",
            "Declaration:Subtype(Creature):Spirit",
        ],
        "Destroy target Human creature." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
            "SingularNominal",
            "ModifiedSingularNominal",
            "NominalModifier",
            "NonCreatureSubtypeModifier",
            "Declaration:Subtype(Creature):Elf",
            "SingularHead",
            "TypeSingularHead",
            "Declaration:Type:Creature",
        ],
        "Lightning Bolt deals 3 damage to any target." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
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
            "AnyTargetReference",
        ],
        "Pyroclasm deals 2 damage to each creature." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
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
            "EachReference",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "AllReference",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "AllReference",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "AllReference",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "AnotherReference",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
        "This creature deals 1 damage to any target." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "ThisReference",
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
            "AnyTargetReference",
        ],
        "Destroy the chosen creatures." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "NamedCardReference",
            "CardName:magnifying-glass",
        ],
        "They gain 2 life." => &[
            "Ability",
            "Paragraph",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "PossessiveAbsoluteReference",
            "PossessiveAbsolutePronoun:Yours",
        ],
        "Context Card deals 2 damage to them." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "ThatReference",
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
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "ThoseReference",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "EachReference",
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
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
            "AllReference",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "OrdinarySingularReference",
            "SingularSelector",
            "TargetSingularSelector",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
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
            "Paragraph",
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
            "Paragraph",
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
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Declarative",
            "Subject",
            "NominalSubject",
            "NounPhrase",
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
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "ChosenQualityReference",
            "ChosenQuality:Color",
        ],
        "Destroy the chosen type." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "ChosenQualityReference",
            "ChosenQuality:Type",
        ],
        "Destroy the chosen name." => &[
            "Ability",
            "Paragraph",
            "Sentence",
            "Imperative",
            "VerbPhrase",
            "Destroy",
            "Declaration:KeywordAction:Destroy",
            "Object",
            "NominalObject",
            "NounPhrase",
            "ChosenQualityReference",
            "ChosenQuality:Name",
        ],
        unexpected => panic!("missing literal Task 8 visitor vector for {unexpected:?}"),
    }
}

struct Shadow {
    losing_path: &'static str,
    losing_specificity: &'static str,
    decisive_position: usize,
}

fn expected_shadow(text: &str) -> Option<Shadow> {
    match text {
        "Destroy X target artifacts." | "Destroy Y target artifacts." => Some(Shadow {
            losing_path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseVariableReference/PluralSelectorUnmarkedPluralSelector/PluralNominalModifiedPluralNominal/NominalModifierCommonNounModifier/PluralHeadTypePluralHead",
            losing_specificity: "NNTNNTNNNNTT",
            decisive_position: 7,
        }),
        "Destroy two target lands." => Some(Shadow {
            losing_path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseFixedReference/CardinalQuantityCardinal/PluralSelectorUnmarkedPluralSelector/PluralNominalModifiedPluralNominal/NominalModifierCommonNounModifier/PluralHeadTypePluralHead",
            losing_specificity: "NNTNNNNTNNNTT",
            decisive_position: 8,
        }),
        "Destroy another target artifact." => Some(Shadow {
            losing_path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseAnotherReference/SingularSelectorUnmarkedSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierCommonNounModifier/SingularHeadTypeSingularHead",
            losing_specificity: "NNTNNLNNNNTT",
            decisive_position: 7,
        }),
        "Destroy one or more target creatures." => Some(Shadow {
            losing_path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOneOrMoreReference/PluralSelectorUnmarkedPluralSelector/PluralNominalModifiedPluralNominal/NominalModifierCommonNounModifier/PluralHeadTypePluralHead",
            losing_specificity: "NNTNNLLLNNNNTT",
            decisive_position: 9,
        }),
        "Destroy any number of target creatures." => Some(Shadow {
            losing_path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseAnyNumberReference/PluralSelectorUnmarkedPluralSelector/PluralNominalModifiedPluralNominal/NominalModifierCommonNounModifier/PluralHeadTypePluralHead",
            losing_specificity: "NNTNNLLLNNNNTT",
            decisive_position: 9,
        }),
        _ => None,
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
    reason = "the complete Task 8 witness table is deliberately literal and reviewable"
)]
#[test]
fn authentic_nominal_and_selector_sentences_parse() {
    let parser = parser();
    let mut failures = Vec::new();
    for witness in [
        Witness {
            card_name: "Desert Twister",
            text: "Destroy target permanent.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalBareSingularNominal/SingularHeadCommonSingularHead",
            specificity: "NNTNNNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Urgent Exorcism",
            text: "Destroy target Spirit.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalBareSingularNominal/SingularHeadCreatureSubtypeSingularHead",
            specificity: "NNTNNNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Human Frailty",
            text: "Destroy target Human creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierCreatureSubtypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target artifact creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierTypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Dark Betrayal",
            text: "Destroy target black creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierColorModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Take Vengeance",
            text: "Destroy target tapped creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierStatusModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Hero's Demise",
            text: "Destroy target legendary creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierSupertypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Doom Blade",
            text: "Destroy target nonblack creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierNonColorModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Bramblecrush",
            text: "Destroy target noncreature permanent.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierNonTypeModifier/SingularHeadCommonSingularHead",
            specificity: "NNTNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Eyeblight's Ending",
            text: "Destroy target non-Elf creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierNonCreatureSubtypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Lightning Bolt",
            text: "Lightning Bolt deals 3 damage to any target.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseSelfReference/VerbPhraseDealDamage/AmountNumber/ObjectObjectNominal/NounPhraseAnyTargetReference",
            specificity: "NNNNTTNLLNTNLL",
            candidates: 1,
        },
        Witness {
            card_name: "Pyroclasm",
            text: "Pyroclasm deals 2 damage to each creature.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseSelfReference/VerbPhraseDealDamage/AmountNumber/ObjectObjectNominal/NounPhraseEachReference/SingularSelectorUnmarkedSingularSelector/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNNNTTNLLNTNLNNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Day of Judgment",
            text: "Destroy all creatures.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseAllReference/PluralSelectorUnmarkedPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNTNNLNNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Hour of Reckoning",
            text: "Destroy all nontoken creatures.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseAllReference/PluralSelectorUnmarkedPluralSelector/PluralNominalModifiedPluralNominal/NominalModifierNonCommonNounModifier/PluralHeadTypePluralHead",
            specificity: "NNTNNLNNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy all other creatures.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseAllReference/PluralSelectorOtherPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNTNNLNLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "By Force",
            text: "Destroy X target artifacts.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseVariableReference/PluralSelectorTargetPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNTNNTNLNNT",
            candidates: 2,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy Y target artifacts.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseVariableReference/PluralSelectorTargetPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNTNNTNLNNT",
            candidates: 2,
        },
        Witness {
            card_name: "Rain of Salt",
            text: "Destroy two target lands.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseFixedReference/CardinalQuantityCardinal/PluralSelectorTargetPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNTNNNNTLNNT",
            candidates: 2,
        },
        Witness {
            card_name: "Aetherjacket",
            text: "Destroy another target artifact.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseAnotherReference/SingularSelectorTargetSingularSelector/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNTNNLNLNNT",
            candidates: 2,
        },
        Witness {
            card_name: "Gearbane Orangutan",
            text: "Destroy up to one target artifact.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseUpToOneReference/CardinalQuantityCardinal/SingularSelectorTargetSingularSelector/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNTNNLLNNTLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy up to one other target artifact.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseUpToOneReference/CardinalQuantityCardinal/SingularSelectorOtherTargetSingularSelector/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNTNNLLNNTLLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy up to three target artifacts.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseUpToManyReference/CardinalQuantityCardinal/PluralSelectorTargetPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNTNNLLNNTLNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Anaba Shaman",
            text: "This creature deals 1 damage to any target.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseThisReference/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/VerbPhraseDealDamage/AmountNumber/ObjectObjectNominal/NounPhraseAnyTargetReference",
            specificity: "NNNNLNNTTNLLNTNLL",
            candidates: 1,
        },
        Witness {
            card_name: "Sadistic Shell Game",
            text: "Destroy the chosen creatures.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseDesignatedPluralReference/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNTNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Voyager Staff",
            text: "Destroy the exiled card.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseDesignatedSingularReference/SingularNominalBareSingularNominal/SingularHeadCommonSingularHead",
            specificity: "NNTNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Agency Outfitter",
            text: "Destroy a card named Magnifying Glass.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseNamedCardReference",
            specificity: "NNTNNLLLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "They gain 2 life.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectPronoun/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Adrenaline Jockey",
            text: "Destroy their creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhrasePossessedSingularReference/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead",
            specificity: "NNTNNTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy yours.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhrasePossessiveAbsoluteReference",
            specificity: "NNTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 2 damage to them.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseSelfReference/VerbPhraseDealDamage/AmountNumber/ObjectObjectPronoun",
            specificity: "NNNNTTNLLNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Asmoranomardicadaistinaculdacar",
            text: "Asmoranomardicadaistinaculdacar deals 2 damage to itself.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseSelfReference/VerbPhraseDealDamage/AmountNumber/ObjectReflexiveObject",
            specificity: "NNNNTTNLLNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "That creature deals 2 damage to it.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseThatReference/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/VerbPhraseDealDamage/AmountNumber/ObjectObjectPronoun",
            specificity: "NNNNLNNTTNLLNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Those creatures deal 2 damage to it.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseThoseReference/PluralNominalBarePluralNominal/PluralHeadTypePluralHead/VerbPhraseDealDamage/AmountNumber/ObjectObjectPronoun",
            specificity: "NNNNLNNTTNLLNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Dwarven Song",
            text: "Destroy one or more target creatures.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOneOrMoreReference/PluralSelectorTargetPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNTNNLLLNLNNT",
            candidates: 2,
        },
        Witness {
            card_name: "Context Card",
            text: "Each creature gains 2 life.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseEachReference/SingularSelectorUnmarkedSingularSelector/SingularNominalBareSingularNominal/SingularHeadTypeSingularHead/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNNLNNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "All creatures gain 2 life.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseAllReference/PluralSelectorUnmarkedPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNNLNNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Dust Bowl",
            text: "Destroy target nonbasic land.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierNonSupertypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonlegendary creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierNonSupertypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonsnow creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierNonSupertypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonartifact permanent.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierNonTypeModifier/SingularHeadCommonSingularHead",
            specificity: "NNTNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target non-Human creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierNonCreatureSubtypeModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonattacking creature.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseOrdinarySingularReference/SingularSelectorTargetSingularSelector/SingularNominalModifiedSingularNominal/NominalModifierNonStatusModifier/SingularHeadTypeSingularHead",
            specificity: "NNTNNNLNNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy any number of target creatures.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseAnyNumberReference/PluralSelectorTargetPluralSelector/PluralNominalBarePluralNominal/PluralHeadTypePluralHead",
            specificity: "NNTNNLLLNLNNT",
            candidates: 2,
        },
        Witness {
            card_name: "Context Card",
            text: "He gains 2 life.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectPronoun/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "She gains 2 life.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectPronoun/VerbPhraseGainLife/AmountNumber",
            specificity: "NNNTTNLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 2 damage to him.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseSelfReference/VerbPhraseDealDamage/AmountNumber/ObjectObjectPronoun",
            specificity: "NNNNTTNLLNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 2 damage to her.",
            path: "AbilityParagraph/SentenceDeclarative/SubjectSubjectNominal/NounPhraseSelfReference/VerbPhraseDealDamage/AmountNumber/ObjectObjectPronoun",
            specificity: "NNNNTTNLLNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the chosen color.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseChosenQualityReference",
            specificity: "NNTNNLLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the chosen type.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseChosenQualityReference",
            specificity: "NNTNNLLT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the chosen name.",
            path: "AbilityParagraph/SentenceImperative/VerbPhraseDestroy/ObjectObjectNominal/NounPhraseChosenQualityReference",
            specificity: "NNTNNLLT",
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
            "Task 8 has no selection exceptions: {:?}",
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

        if let Some(shadow) = expected_shadow(witness.text) {
            assert_eq!(
                decision
                    .candidates()
                    .iter()
                    .map(deckmaste_english_v2::parser::SelectionCandidate::ordinal)
                    .collect::<Vec<_>>(),
                [0, 1],
                "ordered materialized-candidate ordinals changed for {:?}",
                witness.text,
            );
            assert_eq!(
                decision.selected(),
                Some(0),
                "the first materialized candidate must remain selected for {:?}",
                witness.text,
            );
            let losing = &decision.candidates()[1];
            assert_eq!(
                losing.construction_path().join("/"),
                shadow.losing_path,
                "the losing materialized path changed for {:?}",
                witness.text,
            );
            assert_eq!(
                compact_specificity(losing.specificity()),
                shadow.losing_specificity,
                "the losing materialized specificity changed for {:?}",
                witness.text,
            );
            let [comparison] = decision.comparisons() else {
                panic!(
                    "one exact materialized-candidate comparison is required for {:?}",
                    witness.text,
                );
            };
            assert_eq!(comparison.left_ordinal(), 0);
            assert_eq!(comparison.right_ordinal(), 1);
            assert_eq!(comparison.ordering(), Ordering::Greater);
            assert_eq!(
                comparison.decisive(),
                SelectionDecisive::Position(shadow.decisive_position),
                "the decisive materialized specificity position changed for {:?}",
                witness.text,
            );
            assert_eq!(comparison.exception_id(), None);
        } else {
            assert!(
                decision.comparisons().is_empty(),
                "a unique positive gained a materialized shadow for {:?}",
                witness.text,
            );
        }

        let rendered = parsed.render(&context, parser.environment());
        assert_eq!(rendered, witness.text, "positive rendering changed");

        let mut visitor = NominalVisitor::default();
        visitor.visit_ability(parsed);
        assert_eq!(
            visitor.events,
            expected_visitor_events(witness.text),
            "literal nominal visitor preorder changed for {:?}",
            witness.text,
        );

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
