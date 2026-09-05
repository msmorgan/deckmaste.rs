use std::cmp::Ordering;
use std::path::Path;

use deckmaste_construction_core::macro_def::Onset;
use deckmaste_english_v2::ast::CatalogProvider;
use deckmaste_english_v2::ast::CommonNoun;
use deckmaste_english_v2::ast::ComparisonDirection;
use deckmaste_english_v2::ast::CountReference;
use deckmaste_english_v2::ast::MannerReference;
use deckmaste_english_v2::ast::Noun;
use deckmaste_english_v2::ast::ScalarNumber;
use deckmaste_english_v2::ast::ScalarReference;
use deckmaste_english_v2::ast::SingularDemonstrative;
use deckmaste_english_v2::ast::ThatMany;
use deckmaste_english_v2::ast::ThatMuch;
use deckmaste_english_v2::ast::ThisWay;
use deckmaste_english_v2::ast::Variable;
use deckmaste_english_v2::context::ParseContext;
use deckmaste_english_v2::environment::CatalogProviderRow;
use deckmaste_english_v2::environment::CatalogProviderRows;
use deckmaste_english_v2::environment::CoreVerbIdentity;
use deckmaste_english_v2::environment::ParserEnvironment;
use deckmaste_english_v2::environment::VerbInventoryRef;
use deckmaste_english_v2::parser::LexicalProvenanceKind;
use deckmaste_english_v2::parser::ParseAnalysisOutcome;
use deckmaste_english_v2::parser::ParseError;
use deckmaste_english_v2::parser::Parser;
use deckmaste_english_v2::parser::SelectionDecisive;
use deckmaste_english_v2::parser::SelectionResolution;
use deckmaste_english_v2::parser::SpecificityTier;
use deckmaste_english_v2::render::Render;
use deckmaste_english_v2::visit::Visitor;

fn parser() -> Parser {
    let declarations = deckmaste_construction_core::macro_def::read_builtin_v2(
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

fn compact_specificity(specificity: &[SpecificityTier]) -> String {
    specificity
        .iter()
        .map(|tier| match tier {
            SpecificityTier::Nonterminal => 'N',
            SpecificityTier::TypedLexical => 'T',
            SpecificityTier::Identity => 'I',
            SpecificityTier::Literal => 'L',
        })
        .collect()
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
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalBareSingularNominal/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target Spirit.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalBareSingularNominal/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Human Frailty",
            text: "Destroy target Human creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNounModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target artifact creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNounModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Dark Betrayal",
            text: "Destroy target black creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierColorModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Take Vengeance",
            text: "Destroy target tapped creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierStatusModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Hero's Demise",
            text: "Destroy target legendary creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierSupertypeModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Doom Blade",
            text: "Destroy target nonblack creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNonColorModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Bramblecrush",
            text: "Destroy target noncreature permanent.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNonNounModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Eyeblight's Ending",
            text: "Destroy target non-Elf creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNonProperNounModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 3 damage to target creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceSelfReference/VerbPhraseDeclaredToObjectPredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalBareSingularNominal/HeadNounSingularHead",
            specificity: "NNNNNNNNITNTNNNNNNNTNTNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Pyroclasm",
            text: "Pyroclasm deals 2 damage to each creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceSelfReference/VerbPhraseDeclaredToObjectPredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalBareSingularNominal/HeadNounSingularHead",
            specificity: "NNNNNNNNITNTNNNNNNNTNTNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Day of Judgment",
            text: "Destroy all creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativePluralSimpleDeterminative/NominalBarePluralNominal/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Hour of Reckoning",
            text: "Destroy all nontoken creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativePluralSimpleDeterminative/NominalModifiedPluralNominal/NominalModifierNonNounModifier/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy all other creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativePluralSimpleDeterminative/NominalModifiedPluralNominal/NominalModifierAttributiveAdjectiveModifier/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "By Force",
            text: "Destroy X target artifacts.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeVariableQuantifyingDeterminer/NominalModifiedPluralNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy Y target artifacts.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeVariableQuantifyingDeterminer/NominalModifiedPluralNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Rain of Salt",
            text: "Destroy two target lands.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeCardinalQuantifyingDeterminer/CardinalQuantityCardinal/NominalModifiedPluralNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Aetherjacket",
            text: "Destroy another target artifact.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalModifiedSingularNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Gearbane Orangutan",
            text: "Destroy up to one target artifact.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeUpToQuantifyingDeterminer/CardinalQuantityCardinal/NominalModifiedSingularNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNLLNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy up to one other target artifact.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeUpToQuantifyingDeterminer/CardinalQuantityCardinal/NominalModifiedSingularNominal/NominalModifierAttributiveAdjectiveModifier/NominalModifierTargetingMarkerNominalModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNLLNTNNNTTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy up to three target artifacts.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeUpToQuantifyingDeterminer/CardinalQuantityCardinal/NominalModifiedPluralNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNLLNTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "This creature deals 1 damage to target creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalBareSingularNominal/HeadNounSingularHead/VerbPhraseDeclaredToObjectPredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalBareSingularNominal/HeadNounSingularHead",
            specificity: "NNNNNNNNNNTNTTNTNNNNNNNTNTNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Sadistic Shell Game",
            text: "Destroy the chosen creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDesignatedPluralReference/NominalBarePluralNominal/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the exiled card.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDesignatedSingularReference/NominalBareSingularNominal/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy a card named Magnifying Glass.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceNamedCardReference/HeadNounSingularHead",
            specificity: "NNNNTNNNNLNLIT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "They gain 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectPronoun/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun",
            specificity: "NNNNNTNTNNNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy their creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferencePossessedReference/NominalBareSingularNominal/HeadNounSingularHead",
            specificity: "NNNNTNNNNTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy yours.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferencePossessiveAbsoluteReference",
            specificity: "NNNNTNNNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 2 damage to them.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceSelfReference/VerbPhraseDeclaredToObjectPredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun/ObjectObjectPronoun",
            specificity: "NNNNNNNNITNTNNNNNNNTNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Asmoranomardicadaistinaculdacar",
            text: "Asmoranomardicadaistinaculdacar deals 2 damage to itself.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceSelfReference/VerbPhraseDeclaredToObjectPredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun/ObjectReflexiveObject",
            specificity: "NNNNNNNNITNTNNNNNNNTNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "That creature deals 2 damage to it.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalBareSingularNominal/HeadNounSingularHead/VerbPhraseDeclaredToObjectPredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun/ObjectObjectPronoun",
            specificity: "NNNNNNNNNNTNTTNTNNNNNNNTNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Those creatures deal 2 damage to it.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativePluralSimpleDeterminative/NominalBarePluralNominal/HeadNounPluralHead/VerbPhraseDeclaredToObjectPredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun/ObjectObjectPronoun",
            specificity: "NNNNNNNNNNTNTTNTNNNNNNNTNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy one or more target creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeCountComparisonQuantifyingDeterminer/CardinalQuantityCardinal/CountComparisonCountOrMore/NominalModifiedPluralNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNNNTLTNNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Each creature gains 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalBareSingularNominal/HeadNounSingularHead/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun",
            specificity: "NNNNNNNNNNTNTNTNNNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "All creatures gain 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativePluralSimpleDeterminative/NominalBarePluralNominal/HeadNounPluralHead/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun",
            specificity: "NNNNNNNNNNTNTNTNNNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Dust Bowl",
            text: "Destroy target nonbasic land.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNonSupertypeModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonlegendary creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNonSupertypeModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonsnow creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNonSupertypeModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonartifact permanent.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNonNounModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target non-Human creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNonProperNounModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy target nonattacking creature.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNonStatusModifier/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNTNNLTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy any number of target creatures.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeAnyNumberQuantifyingDeterminer/HeadNounSingularHead/NominalModifiedPluralNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounPluralHead",
            specificity: "NNNNTNNNNNNTNTTNNTT",
            candidates: 2,
        },
        Witness {
            card_name: "Context Card",
            text: "He gains 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectPronoun/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun",
            specificity: "NNNNNTNTNNNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "She gains 2 life.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectPronoun/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun",
            specificity: "NNNNNTNTNNNNNNNTNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 2 damage to him.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceSelfReference/VerbPhraseDeclaredToObjectPredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun/ObjectObjectPronoun",
            specificity: "NNNNNNNNITNTNNNNNNNTNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Context Card deals 2 damage to her.",
            path: "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceSelfReference/VerbPhraseDeclaredToObjectPredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun/ObjectObjectPronoun",
            specificity: "NNNNNNNNITNTNNNNNNNTNTT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the chosen color.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDesignatedSingularReference/NominalBareSingularNominal/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the chosen type.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDesignatedSingularReference/NominalBareSingularNominal/HeadNounSingularHead",
            specificity: "NNNNTNNNNNNLTNNT",
            candidates: 1,
        },
        Witness {
            card_name: "Context Card",
            text: "Destroy the chosen name.",
            path: "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDesignatedSingularReference/NominalBareSingularNominal/HeadNounSingularHead",
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
            "candidate census changed for {:?}: {decision:#?}",
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

        if witness.candidates == 1 {
            assert!(
                decision.comparisons().is_empty(),
                "a unique positive gained a materialized shadow for {:?}",
                witness.text,
            );
        } else {
            assert_eq!(decision.survivors(), [selected_ordinal]);
            assert_eq!(decision.comparisons().len(), 1);
        }

        let rendered = parsed.render(&context, parser.environment());
        assert_eq!(rendered, witness.text, "positive rendering changed");

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
fn invalid_nominal_order_concord_class_join_and_case_are_rejected() {
    let parser = parser();
    let context = context("Context Card");
    for text in [
        "Destroy target another creature.",
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
        "Destroy target creature with power 2 or less.",
        "Destroy target creature with power 2 or greater.",
        "Destroy target creature with mana value X or greater.",
        "Destroy target creature with base power 1.",
        "Destroy target creature with greater power.",
        "Destroy target creature with base power and toughness 1/1.",
        "Destroy target creature with power and toughness each equal to its mana value.",
        "Create a 1/1 white Bird creature token with flying.",
        "Create a 1/1 white Bird creature token with trample and haste.",
        "Create a 1/1 white Spirit creature token with \"When this token dies, draw a card.\".",
        "Destroy target creature with power 2 or less you control.",
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
            "Destroy target card in a coin.",
            "the complement head does not license a locative in phrase",
        ),
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
            "Destroy target creature with power and toughness each.",
            "a power/toughness equality complement requires its value",
        ),
        (
            "Destroy target creature with power and toughness 1/1 equal to its mana value.",
            "fixed and equality power/toughness values are mutually exclusive",
        ),
        (
            "Destroy target creature from your graveyard you controls.",
            "embedded controller relatives retain subject-verb concord_class",
        ),
        (
            "Creatures you controls gain 2 life.",
            "you requires bare control",
        ),
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{reason}: {text:?}: {:#?}",
            parser.analyze(text, &context),
        );
    }

    // The ungrammatical relative-clause reading is absent: singular
    // `an opponent` cannot license bare `control`. The duration rival is also
    // absent because mass `control` cannot head the count phrase licensed by
    // `an`.
    let text = "Destroy a creature an opponent control.";
    assert!(parser.parse(text, &context).is_err());
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
            "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferencePrepositionalQualifiedReference/PostmodifiedReferenceRelativeQualifiedReference/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/NominalBarePluralNominal/HeadNounPluralHead/PositiveObjectGapRelativeClausePositiveObjectGapRelative/SubjectSubjectPronoun/PrepositionalPhrasePrepositionalPhrase/ScalarMeasureValueScalarMeasureValue/ScalarMeasureNominalScalarMeasure/NominalBareSingularNominal/HeadNounSingularHead/ScalarComparisonScalarOrLess/ScalarThresholdFixedScalarThreshold/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountVariable/NominalMassNominal/MassNounMassNoun",
            "NNNNNNNNNNNNNNNTTTLNNTNLLTTNLT",
        ),
        (
            "Defeat",
            "Destroy target creature with power 2 or less.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferencePrepositionalQualifiedReference/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalBareSingularNominal/HeadNounSingularHead/PrepositionalPhrasePrepositionalPhrase/ScalarMeasureValueScalarMeasureValue/ScalarMeasureNominalScalarMeasure/NominalBareSingularNominal/HeadNounSingularHead/ScalarComparisonScalarOrLess/ScalarThresholdFixedScalarThreshold",
            "NNNNTNNNNNNNNNTNNTLNNTNLLT",
        ),
        (
            "Context Card",
            "Creatures you control gain 2 life.",
            "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceRelativeQualifiedReference/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/NominalBarePluralNominal/HeadNounPluralHead/PositiveObjectGapRelativeClausePositiveObjectGapRelative/SubjectSubjectPronoun/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun",
            "NNNNNNNNNNNNNNTTTTNLT",
        ),
        (
            "Context Card",
            "Destroy a creature an opponent controls.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceRelativeQualifiedReference/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalBareSingularNominal/HeadNounSingularHead/PositiveObjectGapRelativeClausePositiveObjectGapRelative/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeSingularSimpleDeterminative/NominalBareSingularNominal/HeadNounSingularHead",
            "NNNNTNNNNNNNNTNNTNTLT",
        ),
        (
            "Context Card",
            "Destroy target creature you own.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceRelativeQualifiedReference/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalBareSingularNominal/HeadNounSingularHead/PositiveObjectGapRelativeClausePositiveObjectGapRelative/SubjectSubjectPronoun",
            "NNNNTNNNNNNNNLNNTTT",
        ),
        (
            "Raise Dead",
            "Destroy target creature card from your graveyard.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferencePrepositionalQualifiedReference/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalModifiedSingularNominal/NominalModifierNounModifier/HeadNounSingularHead/PrepositionalPhrasePrepositionalPhrase/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferencePossessedReference/NominalBareSingularNominal/HeadNounSingularHead",
            "NNNNTNNNNNNNNTNNTTTNNNNTNNT",
        ),
        (
            "Context Card",
            "Destroy target creature with mana value X or greater.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferencePrepositionalQualifiedReference/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeTargetingMarkerDeterminative/NominalBareSingularNominal/HeadNounSingularHead/PrepositionalPhrasePrepositionalPhrase/ScalarMeasureValueScalarMeasureValue/ScalarMeasureNominalScalarMeasure/NominalModifiedSingularNominal/NominalModifierNounModifier/HeadNounSingularHead/ScalarComparisonScalarOrGreater/ScalarThresholdVariableScalarThreshold",
            "NNNNTNNNNNNNNNTNNTLNNLLNLLT",
        ),
        (
            "Context Card",
            "Destroy two or more creatures.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeCountComparisonQuantifyingDeterminer/CardinalQuantityCardinal/CountComparisonCountOrMore/NominalBarePluralNominal/HeadNounPluralHead",
            "NNNNTNNNNNNNNTLTNT",
        ),
        (
            "Context Card",
            "Destroy two or fewer creatures.",
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeCountComparisonQuantifyingDeterminer/CardinalQuantityCardinal/CountComparisonCountOrFewer/NominalBarePluralNominal/HeadNounPluralHead",
            "NNNNTNNNNNNNNTLTNT",
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
        assert_eq!(decision.selected(), Some(0), "{text:?}");
        let selected_ordinal = decision.selected().expect("selected parse has an ordinal");
        assert_eq!(decision.survivors(), [selected_ordinal]);
        assert!(decision.comparisons().is_empty());
        assert!(decision.exception_uses().is_empty());
        assert_eq!(decision.candidates()[0].ordinal(), 0);
        let actual_path = decision.candidates()[0].construction_path().join("/");
        if path.contains("RelativeQualifiedReference") {
            assert!(
                actual_path.contains("PostmodifiedReferenceRelativeQualifiedReference")
                    && actual_path
                        .contains("PositiveObjectGapRelativeClausePositiveObjectGapRelative"),
                "{text:?}: {actual_path}",
            );
        } else {
            assert_eq!(actual_path, path);
            if !path.contains("NominalScalarMeasure") {
                assert_eq!(
                    compact_specificity(decision.candidates()[0].specificity()),
                    specificity,
                );
            }
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
        (13, 21, LexicalProvenanceKind::Lexeme, "core-verb:Control"),
        (
            21,
            26,
            LexicalProvenanceKind::Vocab,
            "vocab:Preposition/With",
        ),
        (
            26,
            32,
            LexicalProvenanceKind::Lexeme,
            "lexeme:CommonNoun/Power/singular",
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
            LexicalProvenanceKind::Vocab,
            "vocab:ComparisonDirection/Less",
        ),
        (42, 47, LexicalProvenanceKind::Lexeme, "core-verb:Gain"),
        (47, 49, LexicalProvenanceKind::Vocab, "vocab:Variable/X"),
        (
            49,
            54,
            LexicalProvenanceKind::Lexeme,
            "lexeme:CommonNoun/Life/singular",
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
#[expect(
    clippy::too_many_lines,
    reason = "the contract keeps AST and ownership evidence together for one fixture"
)]
fn former_count_fixture_has_exact_compositional_ast_and_ownership() {
    use deckmaste_english_v2::ast::Ability;
    use deckmaste_english_v2::ast::AbilityBody;
    use deckmaste_english_v2::ast::Amount;
    use deckmaste_english_v2::ast::NounPhrase;
    use deckmaste_english_v2::ast::Plain;
    use deckmaste_english_v2::ast::ScalarComparison;
    use deckmaste_english_v2::ast::ScalarMeasure;
    use deckmaste_english_v2::ast::ScalarMeasureAssignedValue;
    use deckmaste_english_v2::ast::ScalarMeasureValue;
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
    let AbilityBody::Sentences(paragraph) = &body else {
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
    let deckmaste_english_v2::ast::PostmodifiedReference::PrepositionalQualifiedReference(
        reference,
    ) = noun_phrase.reference.as_ref()
    else {
        panic!("the final PP stage owns the scalar qualification: {subject:?}");
    };
    let deckmaste_english_v2::ast::PostmodifiedReference::RelativeQualifiedReference(relative) =
        reference.reference.as_ref()
    else {
        panic!("the object-gap relative precedes scalar qualification: {subject:?}");
    };
    let deckmaste_english_v2::ast::PostmodifiedReference::UnqualifiedPostmodifiedReference(
        unqualified,
    ) = relative.reference.as_ref()
    else {
        panic!("the relative directly modifies its determined nominal: {subject:?}");
    };
    let deckmaste_english_v2::ast::UnqualifiedReference::DeterminedNominal(determined) =
        unqualified.reference.as_ref()
    else {
        panic!("the zero-headed plural reference is a determined nominal: {subject:?}");
    };
    assert!(matches!(
        determined.det(),
        deckmaste_english_v2::ast::Determiner::Zero
    ));
    assert!(matches!(
        determined.nominal,
        deckmaste_english_v2::ast::Nominal::BarePluralNominal(_)
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
    let deckmaste_english_v2::ast::PrepositionalPhrase::PrepositionalPhrase(modifier) =
        reference.modifier()
    else {
        panic!("the final modifier is a prepositional phrase")
    };
    assert_eq!(
        modifier.preposition,
        deckmaste_english_v2::ast::Preposition::With
    );
    let deckmaste_english_v2::ast::PrepositionalComplement::ScalarMeasure(qualification) =
        modifier.complement.as_ref()
    else {
        panic!("the PP complement is a scalar qualification")
    };
    let ScalarMeasureValue::ScalarMeasureValue(scalar) = qualification;
    let ScalarMeasure::NominalScalarMeasure(measure) = &scalar.measure;
    assert!(matches!(
        measure.nominal(),
        deckmaste_english_v2::ast::Nominal::BareSingularNominal(nominal)
            if matches!(nominal.head(), deckmaste_english_v2::ast::Head::NounSingularHead(head)
                if head.noun() == &deckmaste_english_v2::ast::Noun::Lexeme(CommonNoun::Power))
    ));
    let ScalarMeasureAssignedValue::Comparison(ScalarComparison::ScalarOrLess(comparison)) =
        &scalar.value
    else {
        panic!("the parsed comparison keeps its expected Category member")
    };
    assert!(matches!(
        &comparison.threshold,
        ScalarThreshold::FixedScalarThreshold(deckmaste_english_v2::ast::FixedScalarThreshold {
            value: ScalarNumber { magnitude: 2 },
        })
    ));
    assert_eq!(comparison.direction(), ComparisonDirection::Less);
    let deckmaste_english_v2::ast::Predicate::Atomic(predicate) = predicate else {
        panic!("gain-life comparison keeps an atomic predicate")
    };
    let VerbPhrase::BaseVerbPhrase(deckmaste_english_v2::ast::BaseVerbPhrase { frame }) =
        predicate.as_ref()
    else {
        panic!("gain-life comparison uses the generic base frame: {predicate:?}");
    };
    let deckmaste_english_v2::ast::LexicalVerbPhrase::TransitiveLexicalVerbPhrase(frame) =
        frame.as_ref()
    else {
        panic!("gain-life comparison uses the generic transitive frame");
    };
    let deckmaste_english_v2::ast::TransitiveLexicalVerbPhrase::TransitivePredicate(
        deckmaste_english_v2::ast::TransitivePredicate { head, object },
    ) = frame.as_ref();
    assert_eq!(
        head.reference(),
        &VerbInventoryRef::Core(CoreVerbIdentity::Gain)
    );
    let deckmaste_english_v2::ast::Object::ObjectNominal(object) = object else {
        panic!("gain-life comparison has a nominal object")
    };
    let deckmaste_english_v2::ast::NounPhrase::QualifiedNounPhrase(qualified) = object.value()
    else {
        panic!("gain-life object uses the ordinary noun-phrase pipeline")
    };
    let deckmaste_english_v2::ast::PostmodifiedReference::UnqualifiedPostmodifiedReference(
        reference,
    ) = qualified.reference.as_ref()
    else {
        panic!("gain-life object has no postmodifier")
    };
    let deckmaste_english_v2::ast::UnqualifiedReference::DeterminedNominal(determined) =
        reference.reference.as_ref()
    else {
        panic!("gain-life object is a determined mass nominal")
    };
    assert!(matches!(
        determined.det(),
        deckmaste_english_v2::ast::Determiner::Headed(
            deckmaste_english_v2::ast::Determinative::MassQuantityDeterminer(
                deckmaste_english_v2::ast::MassQuantityDeterminer {
                    amount: Amount::Variable(deckmaste_english_v2::ast::VariableAmount {
                        variable: Variable::X,
                    }),
                }
            )
        )
    ));
    assert!(matches!(
        &determined.nominal,
        deckmaste_english_v2::ast::Nominal::MassNominal(deckmaste_english_v2::ast::MassNominal {
            noun: deckmaste_english_v2::ast::MassNoun::MassNoun(
                deckmaste_english_v2::ast::MassNounValue { .. }
            ),
        })
    ));

    assert_former_count_fixture_ownership(
        analysis.ownership().expect("former fixture owns all bytes"),
        text,
    );
}

#[expect(
    clippy::too_many_lines,
    reason = "the witness table carries its expected ordinal per row rather than keying off the text"
)]
#[test]
fn every_selector_family_enters_the_repeatable_order_free_postmodifier_position() {
    let parser = parser();
    let context = context("Context Card");

    for (text, candidate_count, resolution, expected_selected) in [
        (
            "A creature card you control in exile with mana value 2 or less gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "Target creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "Creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "Each creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "All creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "Two creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "X target creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "Another creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "The creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "Up to one target creature you control gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "Up to two target creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "Any number of target creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "One or more target creatures you control gain 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "A creature with mana value 2 or less from your graveyard gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "A card in exile you own gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "A creature you control you own gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "A card in exile in your graveyard gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "A creature with power 2 or less with toughness 2 or less gains 2 life.",
            1,
            SelectionResolution::Unique,
            0,
        ),
        (
            "Creatures with flying you control get +1/+0.",
            1,
            SelectionResolution::Unique,
            0,
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        let selected = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        let decision = analysis.decision().expect("selected parse has a decision");
        assert_eq!(decision.candidates().len(), candidate_count, "{text:?}");
        assert_eq!(decision.resolution(), resolution, "{text:?}: {decision:?}");
        assert_eq!(decision.selected(), Some(expected_selected), "{text:?}");
        let selected_ordinal = decision.selected().expect("selected parse has an ordinal");
        assert_eq!(decision.survivors(), [selected_ordinal]);
        if resolution == SelectionResolution::Unique {
            assert!(decision.comparisons().is_empty(), "{text:?}: {decision:?}");
        } else {
            assert_eq!(decision.comparisons().len(), 1, "{text:?}: {decision:?}");
            assert_eq!(
                decision.comparisons()[0].ordering(),
                if selected_ordinal == 0 { Ordering::Greater } else { Ordering::Less }
            );
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
        "Each creature you control gain 2 life.",
        "All creatures you control gains 2 life.",
    ] {
        assert!(
            parser.analyze(text, &context).outcome() == ParseAnalysisOutcome::ParseFailure,
            "{text:?} must remain an ordinary wrong-concord_class failure",
        );
    }
}

#[test]
fn deictic_manner_count_and_scalar_forms_are_distinct_generated_categories() {
    let parser = parser();
    let context = context("Context Card");

    let manner_analysis = parser.analyze_manner_reference("This way", &context);
    let manner = manner_analysis
        .selected()
        .unwrap_or_else(|| panic!("the manner deictic selects: {manner_analysis:?}"));
    let MannerReference::ThisWay(this_way) = manner else {
        panic!("the manner deictic uses its dedicated construction: {manner:?}")
    };
    assert_eq!(this_way.demonstrative(), SingularDemonstrative::This);
    assert_eq!(this_way.noun(), &Noun::Lexeme(CommonNoun::Way));
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
    assert_eq!(
        count,
        &CountReference::ThatMany(
            ThatMany::new(SingularDemonstrative::That)
                .expect("the closed member satisfies the demonstrative requirement")
        )
    );
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
    assert_eq!(
        scalar,
        &ScalarReference::ThatMuch(
            ThatMuch::new(SingularDemonstrative::That)
                .expect("the closed member satisfies the demonstrative requirement")
        )
    );
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
    assert!(parser.parse_manner_reference("That way", &context).is_err());
    assert!(
        parser
            .parse_manner_reference("This card", &context)
            .is_err()
    );
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
            "{text:?} must remain outside the wrong concord_class or quantity category",
        );
    }
}

#[test]
fn non_modifiers_follow_declared_common_and_proper_spelling() {
    let parser = parser();
    let context = context("Context Card");

    for text in [
        "Destroy all nontoken creatures.",
        "Destroy all nonplayer creatures.",
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
        "Destroy target nonElf creature.",
        "Destroy target nonHuman creature.",
        "Destroy target non-creature permanent.",
        "Destroy target non-artifact permanent.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must follow the noun's declared common/proper spelling",
        );
    }
}

#[test]
fn quantity_determinatives_exclude_zero_and_derive_number() {
    let parser = parser();
    let context = context("Context Card");

    for text in [
        "Destroy one target creature.",
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
        "Destroy up to zero target creatures.",
        "X target creatures gains 2 life.",
        "Y target creatures gains 2 life.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must remain outside the quantity-determiner concord_class join",
        );
    }

    let agreement_violation = "Destroy up to one target creatures.";
    let analysis = parser.analyze(agreement_violation, &context);
    assert!(
        analysis.selected().is_none(),
        "a plural head under `up to one` no longer reaches selection through a duration adjunct: {analysis:#?}",
    );
    assert!(parser.parse(agreement_violation, &context).is_err());
}

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "the target determiner contract compares the full singular and zero-headed plural family"
)]
fn target_determiner_is_singular_and_target_modifier_plurals_are_zero_headed() {
    let parser = parser();
    let context = context("Context Card");

    for (text, singular_head) in [
        ("Destroy target Equipment.", "HeadNounSingularHead"),
        ("Destroy target Plains.", "HeadNounSingularHead"),
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
        let selected_candidate = decision
            .candidates()
            .iter()
            .find(|candidate| Some(candidate.ordinal()) == decision.selected())
            .expect("the selected ordinal names a candidate");
        assert_eq!(
            selected_candidate.construction_path(),
            [
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveLexicalVerbPhraseTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeTargetingMarkerDeterminative",
                "NominalBareSingularNominal",
                singular_head,
            ],
            "bare target must have exactly one singular determiner-phrase AST for {text:?}",
        );
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    let target_modifier_cases = [
        (
            "Destroy target artifacts.",
            1,
            SelectionResolution::Unique,
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/NominalModifiedPluralNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounPluralHead",
            1,
            &[
                (
                    0,
                    7,
                    LexicalProvenanceKind::Lexeme,
                    "lexeme:keyword_action/Destroy/bare",
                ),
                (
                    7,
                    14,
                    LexicalProvenanceKind::Vocab,
                    "vocab:TargetingMarker/Target",
                ),
                (
                    14,
                    24,
                    LexicalProvenanceKind::Lexeme,
                    "lexeme:type/Artifact/plural",
                ),
                (
                    24,
                    25,
                    LexicalProvenanceKind::FormLiteral,
                    "structural:Sentences/sentences/terminator/0",
                ),
            ][..],
        ),
        (
            "Destroy target creatures or planeswalkers.",
            1,
            SelectionResolution::Unique,
            "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/NominalModifiedPluralCoordinationNominalValue/NominalModifierTargetingMarkerNominalModifier/NominalCoordinationOrNominalCoordination/CoordinationMemberBareCoordinationMember/HeadNounPluralHead/CoordinationMemberBareCoordinationMember/HeadNounPluralHead",
            1,
            &[
                (
                    0,
                    7,
                    LexicalProvenanceKind::Lexeme,
                    "lexeme:keyword_action/Destroy/bare",
                ),
                (
                    7,
                    14,
                    LexicalProvenanceKind::Vocab,
                    "vocab:TargetingMarker/Target",
                ),
                (
                    14,
                    24,
                    LexicalProvenanceKind::Lexeme,
                    "lexeme:type/Creature/plural",
                ),
                (
                    24,
                    28,
                    LexicalProvenanceKind::FormLiteral,
                    "structural:OrNominalCoordination/members/separator/pair/0",
                ),
                (
                    28,
                    41,
                    LexicalProvenanceKind::Lexeme,
                    "lexeme:type/Planeswalker/plural",
                ),
                (
                    41,
                    42,
                    LexicalProvenanceKind::FormLiteral,
                    "structural:Sentences/sentences/terminator/0",
                ),
            ][..],
        ),
        (
            "Target creatures gain 2 life.",
            1,
            SelectionResolution::Unique,
            "AbilityPlain/AbilityBodySentences/SentenceDeclarative/FiniteClausePlainFiniteClause/SubjectSubjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/NominalModifiedPluralNominal/NominalModifierTargetingMarkerNominalModifier/HeadNounPluralHead/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/DeterminativeMassQuantityDeterminer/AmountNumber/NominalMassNominal/MassNounMassNoun",
            1,
            &[
                (
                    0,
                    6,
                    LexicalProvenanceKind::Vocab,
                    "vocab:TargetingMarker/Target",
                ),
                (
                    6,
                    16,
                    LexicalProvenanceKind::Lexeme,
                    "lexeme:type/Creature/plural",
                ),
                (16, 21, LexicalProvenanceKind::Lexeme, "core-verb:Gain"),
                (21, 23, LexicalProvenanceKind::Codec, "codec:ScalarNumber"),
                (
                    23,
                    28,
                    LexicalProvenanceKind::Lexeme,
                    "lexeme:CommonNoun/Life/singular",
                ),
                (
                    28,
                    29,
                    LexicalProvenanceKind::FormLiteral,
                    "structural:Sentences/sentences/terminator/0",
                ),
            ][..],
        ),
    ];
    for (text, candidates, resolution, path, zero_headed_nominals, ownership) in
        target_modifier_cases
    {
        assert_target_modifier_phrase(
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
    assert_target_modifier_phrase(
        &parser,
        &context,
        text,
        1,
        SelectionResolution::Unique,
        "AbilityPlain/AbilityBodySentences/SentenceImperative/VerbPhraseBaseVerbPhrase/TransitiveLexicalVerbPhraseTransitivePredicate/ObjectObjectNominal/NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal/NominalNegativeModifiedPluralNominal/NominalModifierTargetingMarkerNominalModifier/NegativeNominalModifierNegativeModifierMember/NominalModifierNonNounModifier/NegativeNominalModifierNegativeModifierMember/NominalModifierNonColorModifier/HeadNounPluralHead",
        1,
        &[
            (
                0,
                7,
                LexicalProvenanceKind::Lexeme,
                "lexeme:keyword_action/Destroy/bare",
            ),
            (
                7,
                14,
                LexicalProvenanceKind::Vocab,
                "vocab:TargetingMarker/Target",
            ),
            (
                14,
                18,
                LexicalProvenanceKind::FormLiteral,
                "form:non_noun_modifier/non_noun_modifier/0/affix",
            ),
            (
                18,
                26,
                LexicalProvenanceKind::Lexeme,
                "lexeme:type/Artifact/singular",
            ),
            (
                26,
                28,
                LexicalProvenanceKind::FormLiteral,
                "structural:NegativeModifiedPluralNominal/modifiers/separator/uniform/0",
            ),
            (
                28,
                31,
                LexicalProvenanceKind::FormLiteral,
                "form:non_color_modifier/non_color_modifier/0/affix",
            ),
            (
                31,
                36,
                LexicalProvenanceKind::Vocab,
                "vocab:ColorWord/Black",
            ),
            (
                36,
                46,
                LexicalProvenanceKind::Lexeme,
                "lexeme:type/Creature/plural",
            ),
            (
                46,
                47,
                LexicalProvenanceKind::FormLiteral,
                "structural:Sentences/sentences/terminator/0",
            ),
        ],
    );

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

#[test]
fn target_determiner_and_modifier_distributions_select_uniquely() {
    let parser = parser();
    let context = context("Context Card");

    for text in [
        "Destroy target creature.",
        "Destroy two target creatures.",
        "Destroy another target creature.",
    ] {
        let analysis = parser.analyze(text, &context);
        let selected = analysis
            .selected()
            .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
        let decision = analysis.decision().expect("selected parse has a decision");
        assert_eq!(decision.candidates().len(), 1, "{text:?}: {decision:?}");
        assert_eq!(
            decision.resolution(),
            SelectionResolution::Unique,
            "{text:?}"
        );
        assert_eq!(selected.render(&context, parser.environment()), text);
    }
}

#[test]
fn targeting_marker_projections_retain_count_noun_homograph() {
    let parser = parser();
    let context = context("Context Card");

    for text in [
        "Destroy target creature.",
        "Destroy target artifacts.",
        "Destroy target tapped creature.",
        "Destroy two target creatures.",
        "Destroy up to two target creatures.",
        "Destroy another target artifact.",
        "Choose new targets.",
    ] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("targeting acceptance {text:?} must select: {error:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    let text = "Choose new targets.";
    let owner = "lexeme:CommonNoun/Target/plural";
    let analysis = parser.analyze(text, &context);
    assert!(
        analysis
            .ownership()
            .expect("accepted homograph owns every byte")
            .parsed_claims()
            .iter()
            .any(|claim| claim.stable_owner_id() == owner),
        "{text:?} must use {owner}",
    );

    for text in [
        "Destroy a more target creature.",
        "This creature is target.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "the targeting marker must reject adjective-like grading and predicative use: {text:?}",
        );
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "one helper keeps every target-modifier phrase's path, AST, and ownership pinned"
)]
fn assert_target_modifier_phrase(
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
        .unwrap_or_else(|| panic!("target-modifier phrase must select: {text:?}: {analysis:?}"));
    let decision = analysis
        .decision()
        .expect("selected compound retains a decision");
    assert_eq!(decision.candidates().len(), candidates, "{text:?}");
    assert_eq!(decision.resolution(), resolution, "{text:?}");
    assert!(decision.exception_uses().is_empty(), "{text:?}");
    let selected_ordinal = decision
        .selected()
        .expect("selected compound names its candidate");
    let selected_candidate = decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == selected_ordinal)
        .expect("selected ordinal names a retained candidate");
    assert_eq!(
        selected_candidate.construction_path().join("/"),
        path,
        "target-modifier construction ownership changed for {text:?}",
    );
    assert_eq!(selected.render(context, parser.environment()), text);

    let mut ast = TargetAsNounAstVisitor::default();
    ast.visit_ability(selected);
    let expected_determined_nominals = zero_headed_nominals + usize::from(text.contains(" gain "));
    assert_eq!(
        ast.determined_nominals, expected_determined_nominals,
        "{text:?}"
    );
    assert_eq!(ast.zero_headed_nominals, zero_headed_nominals, "{text:?}");
    assert_eq!(ast.target_modifiers, 1, "{text:?}");

    let ownership = analysis
        .ownership()
        .expect("selected compound owns its bytes");
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
            deckmaste_english_v2::ast::NominalModifier::TargetingMarkerNominalModifier(
                deckmaste_english_v2::ast::TargetingMarkerNominalModifier {
                    marker: deckmaste_english_v2::ast::TargetingMarker::Target,
                }
            )
        ) {
            self.target_modifiers += 1;
        }
        deckmaste_english_v2::visit::walk_nominal_modifier(self, value);
    }
}

#[test]
fn other_target_plurals_are_compositional_determiner_phrases() {
    let parser = parser();
    let context = context("Context Card");

    let path_prefix = [
        "AbilityPlain",
        "AbilityBodySentences",
        "SentenceImperative",
        "VerbPhraseBaseVerbPhrase",
        "TransitiveLexicalVerbPhraseTransitivePredicate",
        "ObjectObjectNominal",
        "NounPhraseQualifiedNounPhrase",
        "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
    ];
    for (text, candidates, resolution, path_suffix) in [
        (
            "Destroy other target creatures.",
            1,
            SelectionResolution::Unique,
            &[
                "UnqualifiedReferenceDeterminedNominal",
                "NominalModifiedPluralNominal",
                "NominalModifierAttributiveAdjectiveModifier",
                "NominalModifierTargetingMarkerNominalModifier",
                "HeadNounPluralHead",
            ][..],
        ),
        (
            "Destroy other target Equipment.",
            1,
            SelectionResolution::Unique,
            &[
                "UnqualifiedReferenceDeterminedNominal",
                "NominalModifiedPluralNominal",
                "NominalModifierAttributiveAdjectiveModifier",
                "NominalModifierTargetingMarkerNominalModifier",
                "HeadNounPluralHead",
            ][..],
        ),
        (
            "Destroy all other target creatures.",
            1,
            SelectionResolution::Unique,
            &[
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativePluralSimpleDeterminative",
                "NominalModifiedPluralNominal",
                "NominalModifierAttributiveAdjectiveModifier",
                "NominalModifierTargetingMarkerNominalModifier",
                "HeadNounPluralHead",
            ][..],
        ),
        (
            "Destroy two other target creatures.",
            1,
            SelectionResolution::Unique,
            &[
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeCardinalQuantifyingDeterminer",
                "CardinalQuantityCardinal",
                "NominalModifiedPluralNominal",
                "NominalModifierAttributiveAdjectiveModifier",
                "NominalModifierTargetingMarkerNominalModifier",
                "HeadNounPluralHead",
            ][..],
        ),
        (
            "Destroy X other target creatures.",
            1,
            SelectionResolution::Unique,
            &[
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeVariableQuantifyingDeterminer",
                "NominalModifiedPluralNominal",
                "NominalModifierAttributiveAdjectiveModifier",
                "NominalModifierTargetingMarkerNominalModifier",
                "HeadNounPluralHead",
            ][..],
        ),
        (
            "Destroy up to three other target creatures.",
            1,
            SelectionResolution::Unique,
            &[
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeUpToQuantifyingDeterminer",
                "CardinalQuantityCardinal",
                "NominalModifiedPluralNominal",
                "NominalModifierAttributiveAdjectiveModifier",
                "NominalModifierTargetingMarkerNominalModifier",
                "HeadNounPluralHead",
            ][..],
        ),
        (
            "Destroy any number of other target Equipment.",
            2,
            SelectionResolution::Specificity,
            &[
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeAnyNumberQuantifyingDeterminer",
                "HeadNounSingularHead",
                "NominalModifiedPluralNominal",
                "NominalModifierAttributiveAdjectiveModifier",
                "NominalModifierTargetingMarkerNominalModifier",
                "HeadNounPluralHead",
            ][..],
        ),
    ] {
        assert_quantified_other_target_case(
            &parser,
            &context,
            text,
            candidates,
            resolution,
            &path_prefix,
            path_suffix,
        );
    }
}
fn assert_quantified_other_target_case(
    parser: &Parser,
    context: &ParseContext<'_>,
    text: &str,
    expected_candidates: usize,
    expected_resolution: SelectionResolution,
    path_prefix: &[&str],
    path_suffix: &[&str],
) {
    let analysis = parser.analyze(text, context);
    let selected = analysis
        .selected()
        .unwrap_or_else(|| panic!("quantified other-target plural must select: {analysis:?}"));
    let decision = analysis.decision().expect("selected parse has a decision");
    assert_eq!(decision.candidates().len(), expected_candidates, "{text:?}");
    assert_eq!(decision.resolution(), expected_resolution, "{text:?}");
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
            "{text:?} must respect pronoun case and nominal concord_class",
        );
    }
}

#[test]
fn aggregate_noun_inventory_accepts_every_contributing_subtype_family() {
    use deckmaste_construction_core::macro_def::DeclarationIdentity;
    use deckmaste_construction_core::macro_def::DeclarationKind;
    use deckmaste_construction_core::macro_def::SubtypeCategory;

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

    for identity in [
        artifact,
        battle,
        creature,
        enchantment,
        land,
        planeswalker,
        spell,
    ] {
        assert!(
            deckmaste_english_v2::ast::DeclarationNoun::new(environment, identity).is_some(),
            "every declared subtype family contributes to the one noun inventory",
        );
    }
    assert!(
        deckmaste_english_v2::ast::DeclarationNoun::new(
            environment,
            DeclarationIdentity::new(DeclarationKind::KeywordAbility, "Flying"),
        )
        .is_none(),
        "noncontributing declaration kinds remain outside the noun inventory",
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
    let selected_ordinal = decision.selected().expect("full NP selects");
    let selected_candidate = decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == selected_ordinal)
        .expect("selected ordinal names the full-NP candidate");
    assert_eq!(
        selected_candidate.construction_path(),
        [
            "AbilityPlain",
            "AbilityBodySentences",
            "SentenceImperative",
            "VerbPhraseBaseVerbPhrase",
            "TransitiveLexicalVerbPhraseTransitivePredicate",
            "ObjectObjectNominal",
            "NounPhraseQualifiedNounPhrase",
            "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
            "UnqualifiedReferenceCoordinatedNounPhrase",
            "FullNounPhraseCoordinationFullOrNounPhraseCoordination",
            "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
            "UnqualifiedReferenceDeterminedNominal",
            "DeterminativeTargetingMarkerDeterminative",
            "NominalBareSingularNominal",
            "HeadNounSingularHead",
            "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
            "UnqualifiedReferenceDeterminedNominal",
            "DeterminativeTargetingMarkerDeterminative",
            "NominalBareSingularNominal",
            "HeadNounSingularHead",
        ],
        "a repeated target belongs to two full noun phrases, never one shared selector",
    );
    assert!(decision.comparisons().is_empty(), "{decision:?}");
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
            "NominalCoordinationAndNominalCoordination",
            "Destroy target artifact and creature.",
        ),
        (
            "shared singular and",
            3,
            "NominalCoordinationAndNominalCoordination",
            "Destroy target artifact, creature, and planeswalker.",
        ),
        (
            "shared singular and",
            4,
            "NominalCoordinationAndNominalCoordination",
            "Destroy target artifact, creature, enchantment, and land.",
        ),
        (
            "shared singular or",
            2,
            "NominalCoordinationOrNominalCoordination",
            "Destroy target artifact or creature.",
        ),
        (
            "shared singular or",
            3,
            "NominalCoordinationOrNominalCoordination",
            "Destroy target artifact, creature, or planeswalker.",
        ),
        (
            "shared singular or",
            4,
            "NominalCoordinationOrNominalCoordination",
            "Destroy target artifact, creature, enchantment, or land.",
        ),
        (
            "shared singular and/or",
            2,
            "NominalCoordinationAndOrNominalCoordination",
            "Destroy target artifact and/or creature.",
        ),
        (
            "shared singular and/or",
            3,
            "NominalCoordinationAndOrNominalCoordination",
            "Destroy target artifact, creature, and/or planeswalker.",
        ),
        (
            "shared singular and/or",
            4,
            "NominalCoordinationAndOrNominalCoordination",
            "Destroy target artifact, creature, enchantment, and/or land.",
        ),
        (
            "shared plural and",
            2,
            "NominalCoordinationAndNominalCoordination",
            "Destroy all artifacts and creatures.",
        ),
        (
            "shared plural and",
            3,
            "NominalCoordinationAndNominalCoordination",
            "Destroy all artifacts, creatures, and planeswalkers.",
        ),
        (
            "shared plural and",
            4,
            "NominalCoordinationAndNominalCoordination",
            "Destroy all artifacts, creatures, enchantments, and lands.",
        ),
        (
            "shared plural or",
            2,
            "NominalCoordinationOrNominalCoordination",
            "Destroy all artifacts or creatures.",
        ),
        (
            "shared plural or",
            3,
            "NominalCoordinationOrNominalCoordination",
            "Destroy all artifacts, creatures, or planeswalkers.",
        ),
        (
            "shared plural or",
            4,
            "NominalCoordinationOrNominalCoordination",
            "Destroy all artifacts, creatures, enchantments, or lands.",
        ),
        (
            "shared plural and/or",
            2,
            "NominalCoordinationAndOrNominalCoordination",
            "Destroy all artifacts and/or creatures.",
        ),
        (
            "shared plural and/or",
            3,
            "NominalCoordinationAndOrNominalCoordination",
            "Destroy all artifacts, creatures, and/or planeswalkers.",
        ),
        (
            "shared plural and/or",
            4,
            "NominalCoordinationAndOrNominalCoordination",
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

fn assert_coordination_evidence(
    parser: &Parser,
    card_name: &str,
    text: &str,
    expected_path: &[&str],
    expected_specificity: &[SpecificityTier],
    expected_candidates: usize,
    expected_resolution: SelectionResolution,
) {
    let context = context(card_name);
    let analysis = parser.analyze(text, &context);
    let parsed = analysis
        .selected()
        .unwrap_or_else(|| panic!("{text:?} must select: {analysis:?}"));
    let decision = analysis
        .decision()
        .expect("a selected parse has a decision");
    assert_eq!(
        decision.candidates().len(),
        expected_candidates,
        "{text:?}: {decision:?}",
    );
    assert_eq!(decision.resolution(), expected_resolution);
    let selected_ordinal = decision.selected().expect("coordination selects");
    assert_eq!(decision.survivors(), [selected_ordinal]);
    assert!(decision.exception_uses().is_empty());
    let selected = decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == selected_ordinal)
        .expect("selected ordinal names a retained candidate");
    assert_eq!(selected.construction_path(), expected_path);
    assert_eq!(selected.specificity(), expected_specificity);

    assert_eq!(parsed.render(&context, parser.environment()), text);
    let ownership = analysis.ownership().expect("selected parse owns its bytes");
    assert!(ownership.failures().is_empty());
    assert!(ownership.summary().covered());
    assert_eq!(ownership.rendered_text(), text);
}

#[test]
fn coordination_ast_scope_ownership_and_ambiguity_are_exact() {
    let parser = parser();
    for (
        card_name,
        text,
        expected_path,
        expected_specificity,
        expected_candidates,
        expected_resolution,
    ) in [
        (
            "Bedevil",
            "Destroy target artifact, creature, or planeswalker.",
            &[
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveLexicalVerbPhraseTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeTargetingMarkerDeterminative",
                "NominalSingularCoordinationNominalValue",
                "NominalCoordinationOrNominalCoordination",
                "CoordinationMemberBareCoordinationMember",
                "HeadNounSingularHead",
                "CoordinationMemberBareCoordinationMember",
                "HeadNounSingularHead",
                "CoordinationMemberBareCoordinationMember",
                "HeadNounSingularHead",
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
                SpecificityTier::TypedLexical,
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
            1,
            SelectionResolution::Unique,
        ),
        (
            "Decimate",
            "Destroy target artifact, target creature, target enchantment, and target land.",
            &[
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveLexicalVerbPhraseTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                "UnqualifiedReferenceCoordinatedNounPhrase",
                "FullNounPhraseCoordinationFullAndNounPhraseCoordination",
                "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeTargetingMarkerDeterminative",
                "NominalBareSingularNominal",
                "HeadNounSingularHead",
                "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeTargetingMarkerDeterminative",
                "NominalBareSingularNominal",
                "HeadNounSingularHead",
                "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeTargetingMarkerDeterminative",
                "NominalBareSingularNominal",
                "HeadNounSingularHead",
                "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeTargetingMarkerDeterminative",
                "NominalBareSingularNominal",
                "HeadNounSingularHead",
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
                SpecificityTier::Literal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
                SpecificityTier::Nonterminal,
                SpecificityTier::TypedLexical,
            ][..],
            1,
            SelectionResolution::Unique,
        ),
    ] {
        assert_coordination_evidence(
            &parser,
            card_name,
            text,
            expected_path,
            expected_specificity,
            expected_candidates,
            expected_resolution,
        );
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
    for (text, expected_path) in [
        (
            "Destroy target nonartifact, nonblack creature.",
            &[
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveLexicalVerbPhraseTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeTargetingMarkerDeterminative",
                "NominalNegativeModifiedSingularNominal",
                "NegativeNominalModifierNegativeModifierMember",
                "NominalModifierNonNounModifier",
                "NegativeNominalModifierNegativeModifierMember",
                "NominalModifierNonColorModifier",
                "HeadNounSingularHead",
            ][..],
        ),
        (
            "Destroy two target nonartifact, nonblack creatures.",
            &[
                "AbilityPlain",
                "AbilityBodySentences",
                "SentenceImperative",
                "VerbPhraseBaseVerbPhrase",
                "TransitiveLexicalVerbPhraseTransitivePredicate",
                "ObjectObjectNominal",
                "NounPhraseQualifiedNounPhrase",
                "PostmodifiedReferenceUnqualifiedPostmodifiedReference",
                "UnqualifiedReferenceDeterminedNominal",
                "DeterminativeCardinalQuantifyingDeterminer",
                "CardinalQuantityCardinal",
                "NominalNegativeModifiedPluralNominal",
                "NominalModifierTargetingMarkerNominalModifier",
                "NegativeNominalModifierNegativeModifierMember",
                "NominalModifierNonNounModifier",
                "NegativeNominalModifierNegativeModifierMember",
                "NominalModifierNonColorModifier",
                "HeadNounPluralHead",
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
        "Destroy target nonartifact, nonblack, creature.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?} must not enter a coordination or modifier-sequence AST",
        );
    }
}

#[test]
fn coordination_minimum_arity_and_concord_class_are_unconstructible_when_inconsistent() {
    use deckmaste_construction_core::macro_def::DeclarationIdentity;
    use deckmaste_construction_core::macro_def::DeclarationKind;
    use deckmaste_english_v2::ast::*;

    let parser = parser();
    let environment = parser.environment();
    let head = |name| {
        Head::NounSingularHead(
            NounSingularHead::new(Noun::Declaration(
                DeclarationNoun::new(
                    environment,
                    DeclarationIdentity::new(DeclarationKind::Type, name),
                )
                .expect("the builtin Type noun is present"),
            ))
            .expect("Type declarations are count nouns"),
        )
    };
    let nominal = |name| {
        Nominal::BareSingularNominal(
            BareSingularNominal::new(head(name))
                .expect("the Singular nominal accepts a Singular Head"),
        )
    };
    let coordination_member =
        |head| CoordinationMember::BareCoordinationMember(BareCoordinationMember { head });
    let plural_head = |name| {
        Head::NounPluralHead(
            NounPluralHead::new(Noun::Declaration(
                DeclarationNoun::new(
                    environment,
                    DeclarationIdentity::new(DeclarationKind::Type, name),
                )
                .expect("the builtin Type noun is present"),
            ))
            .expect("Type declarations are count nouns"),
        )
    };
    assert!(AndNominalCoordination::new(vec![coordination_member(head("Artifact"))]).is_none());
    assert!(OrNominalCoordination::new(vec![coordination_member(head("Artifact"))]).is_none());
    assert!(AndOrNominalCoordination::new(vec![coordination_member(head("Artifact"))]).is_none());
    assert!(
        AndNominalCoordination::new(vec![coordination_member(plural_head("Artifact"))]).is_none()
    );
    assert!(
        OrNominalCoordination::new(vec![coordination_member(plural_head("Artifact"))]).is_none()
    );
    assert!(
        AndOrNominalCoordination::new(vec![coordination_member(plural_head("Artifact"))]).is_none()
    );
    let mixed_number = || {
        vec![
            coordination_member(head("Artifact")),
            coordination_member(plural_head("Artifact")),
        ]
    };
    assert!(AndNominalCoordination::new(mixed_number()).is_none());
    assert!(OrNominalCoordination::new(mixed_number()).is_none());
    assert!(AndOrNominalCoordination::new(mixed_number()).is_none());
    let determined = DeterminedNominal::new(
        Determiner::Headed(Determinative::TargetingMarkerDeterminative(
            TargetingMarkerDeterminative {
                marker: TargetingMarker::Target,
            },
        )),
        nominal("Artifact"),
    )
    .expect("target determiner agrees with artifact");
    let determined = UnqualifiedReference::DeterminedNominal(determined);
    let determined =
        PostmodifiedReference::UnqualifiedPostmodifiedReference(UnqualifiedPostmodifiedReference {
            reference: Box::new(determined),
        });
    assert!(FullAndNounPhraseCoordination::new(Box::new(vec![determined.clone()])).is_none());
    assert!(FullOrNounPhraseCoordination::new(Box::new(vec![determined.clone()])).is_none());
    assert!(FullAndOrNounPhraseCoordination::new(Box::new(vec![determined])).is_none());

    let context = context("Context Card");
    for (text, noun_phrase_path, concord_class_summary) in [
        (
            "Target creature or planeswalker gains 2 life.",
            "NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal",
            "concord_class: ThirdPersonSingular",
        ),
        (
            "Two target creatures or planeswalkers gain 2 life.",
            "NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal",
            "concord_class: Other",
        ),
        (
            "Target creatures or planeswalkers gain 2 life.",
            "NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceDeterminedNominal",
            "concord_class: Other",
        ),
        (
            "Target creature and target planeswalker gain 2 life.",
            "NounPhraseQualifiedNounPhrase/PostmodifiedReferenceUnqualifiedPostmodifiedReference/UnqualifiedReferenceCoordinatedNounPhrase",
            "concord_class: Other",
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
            .expect("selected concord_class probe owns its lexical leaves")
            .parsed_claims();
        assert!(
            claims
                .iter()
                .any(|claim| claim.semantic_summary().contains(concord_class_summary)),
            "derived ConcordClass evidence changed for {text:?}: {claims:?}",
        );
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }
    for text in [
        "Target creature or planeswalker gain 2 life.",
        "Target creatures or planeswalkers gains 2 life.",
        "Target creature and target planeswalker gains 2 life.",
    ] {
        let analysis = parser.analyze(text, &context);
        assert!(
            analysis.outcome() == ParseAnalysisOutcome::ParseFailure,
            "{text:?} carries inconsistent shared-selector/full-NP concord_class: {analysis:?}",
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
            nominal_path: "NominalModifiedSingularNominal",
            head_owner: "lexeme:CommonNoun/Type/singular",
        },
        PositiveWitness {
            text: "Destroy all tapped artifact types.",
            modifier_path: "NominalModifierStatusModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all legendary artifact types.",
            modifier_path: "NominalModifierSupertypeModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all spell artifact types.",
            modifier_path: "NominalModifierNounModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all artifact creature types.",
            modifier_path: "NominalModifierNounModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Equipment artifact types.",
            modifier_path: "NominalModifierNounModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Siege battle types.",
            modifier_path: "NominalModifierNounModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Human creature types.",
            modifier_path: "NominalModifierNounModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Aura enchantment types.",
            modifier_path: "NominalModifierNounModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Plains land types.",
            modifier_path: "NominalModifierNounModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Jace planeswalker types.",
            modifier_path: "NominalModifierNounModifier",
            nominal_path: "NominalModifiedPluralNominal",
            head_owner: "lexeme:CommonNoun/Type/plural",
        },
        PositiveWitness {
            text: "Destroy all Arcane spell types.",
            modifier_path: "NominalModifierNounModifier",
            nominal_path: "NominalModifiedPluralNominal",
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
        assert_eq!(
            decision.candidates().len(),
            1,
            "{:?}: {decision:#?}",
            witness.text
        );
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
            claim.stable_owner_id() == "determinative:DeterminativeHead/IndefiniteArticle"
        })
        .map(deckmaste_english_v2::parser::LexicalClaim::stable_owner_id)
        .collect::<Vec<_>>();
    assert_eq!(
        article_owners,
        [
            "determinative:DeterminativeHead/IndefiniteArticle",
            "determinative:DeterminativeHead/IndefiniteArticle",
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

#[test]
fn any_one_is_one_closed_determiner_with_no_generic_duration_rival() {
    let parser = parser();
    let context = context("Grammar Witness");
    let text = "Add two mana of any one color.";
    let analysis = parser.analyze(text, &context);

    assert_eq!(analysis.outcome(), ParseAnalysisOutcome::Selected);
    let decision = analysis
        .decision()
        .expect("the selected reading records its decision");
    assert_eq!(decision.candidates().len(), 1);
    assert_eq!(decision.resolution(), SelectionResolution::Unique);
    let selected_ordinal = decision.selected().expect("one reading is selected");
    assert_eq!(decision.survivors(), [selected_ordinal]);
    assert!(decision.exception_uses().is_empty());
    let selected_path = decision.candidates()[selected_ordinal].construction_path();
    assert!(
        selected_path
            .iter()
            .any(|item| item == "DeterminativeSingularSimpleDeterminative")
    );
    assert!(
        !selected_path
            .iter()
            .any(|item| item == "PredicateAdjunctDurationPredicateAdjunct")
    );
    assert!(!decision.candidates().iter().any(|candidate| {
        candidate
            .construction_path()
            .iter()
            .any(|item| item == "PredicateAdjunctDurationPredicateAdjunct")
    }));

    let selected = analysis
        .selected()
        .expect("the intended reading has an AST");
    assert_eq!(selected.render(&context, parser.environment()), text);
    let ownership = analysis
        .ownership()
        .expect("the intended reading owns every source byte");
    assert!(ownership.failures().is_empty());
    assert!(ownership.summary().covered());
    assert!(
        ownership
            .parsed_claims()
            .iter()
            .any(|claim| { claim.stable_owner_id() == "determinative:DeterminativeHead/AnyOne" })
    );
}

#[test]
fn possessive_determiners_license_nominals_without_form_partitioning() {
    let parser = parser();
    let context = context("Grammar Witness");
    for (text, required_path) in [
        (
            "Destroy your creature.",
            [
                "UnqualifiedReferencePossessedReference",
                "NominalBareSingularNominal",
            ],
        ),
        (
            "Destroy your creatures.",
            [
                "UnqualifiedReferencePossessedReference",
                "NominalBarePluralNominal",
            ],
        ),
        (
            "Destroy target player's creatures and artifacts.",
            [
                "UnqualifiedReferenceGenitiveDeterminerCoordinationReference",
                "NominalCoordinationAndNominalCoordination",
            ],
        ),
        (
            "That source's controllers gain 2 life.",
            [
                "UnqualifiedReferenceDemonstrativePossessiveReference",
                "NominalBarePluralNominal",
            ],
        ),
        (
            "Destroy that player's creature and artifact.",
            [
                "UnqualifiedReferenceGenitiveDeterminerCoordinationReference",
                "NominalCoordinationAndNominalCoordination",
            ],
        ),
    ] {
        let analysis = parser.analyze(text, &context);
        assert_eq!(
            analysis.outcome(),
            ParseAnalysisOutcome::Selected,
            "{text:?}: {analysis:#?}",
        );
        let decision = analysis
            .decision()
            .expect("a selected possessive analysis records its decision");
        let selected = decision.selected().expect("one analysis is selected");
        let path = decision.candidates()[selected].construction_path();
        for required in required_path {
            assert!(
                path.iter().any(|item| item == required),
                "{text:?} lacks {required}: {path:?}",
            );
        }
        let parsed = analysis
            .selected()
            .expect("a selected possessive analysis has an AST");
        assert_eq!(parsed.render(&context, parser.environment()), text);
    }

    let ungrammatical = "That sources' controller gains 2 life.";
    assert_eq!(
        parser.analyze(ungrammatical, &context).outcome(),
        ParseAnalysisOutcome::ParseFailure,
        "a singular demonstrative cannot determine a plural possessor",
    );
}

#[test]
fn shared_head_coordination_remains_selected_under_a_possessive_determiner() {
    let parser = parser();
    let context = context("Grammar Witness");
    let text = "Destroy your first instant or sorcery spell.";
    let analysis = parser.analyze(text, &context);
    assert_eq!(analysis.outcome(), ParseAnalysisOutcome::Selected);
    let decision = analysis
        .decision()
        .expect("the selected shared-head analysis records its decision");
    assert_eq!(decision.resolution(), SelectionResolution::Unique);
    let selected = decision.selected().expect("one analysis is selected");
    let path = decision.candidates()[selected].construction_path();
    let relevant = path
        .iter()
        .filter(|item| {
            item.starts_with("UnqualifiedReferencePossessed")
                || item.starts_with("NominalModified")
                || item.starts_with("NominalModifierOrSharedHead")
                || item.starts_with("NominalCoordinationOr")
        })
        .map(String::as_str)
        .collect::<Vec<_>>();
    assert_eq!(
        relevant,
        [
            "UnqualifiedReferencePossessedReference",
            "NominalModifiedSingularNominal",
            "NominalModifierOrSharedHeadModifier",
        ],
    );
    let parsed = analysis
        .selected()
        .expect("the shared-head analysis has an AST");
    assert_eq!(parsed.render(&context, parser.environment()), text);
}

fn selected_path_and_specificity(
    parser: &Parser,
    card_name: &'static str,
    text: &str,
) -> (Vec<String>, Vec<SpecificityTier>) {
    let context = context(card_name);
    let analysis = parser.analyze(text, &context);
    let decision = analysis
        .decision()
        .unwrap_or_else(|| panic!("{card_name} must select {text:?}"));
    assert_eq!(
        decision.survivors().len(),
        1,
        "{card_name}: {text:?}: {decision:#?}",
    );
    assert!(
        decision.exception_uses().is_empty(),
        "{card_name}: {text:?}",
    );
    let selected_ordinal = decision.selected().expect("a survivor is selected");
    let selected = decision
        .candidates()
        .iter()
        .find(|candidate| candidate.ordinal() == selected_ordinal)
        .expect("the selected ordinal names a retained candidate");
    (
        selected.construction_path().to_vec(),
        selected.specificity().to_vec(),
    )
}

/// A spelling supplied by the parse context names one particular object, so it
/// outranks a declared type spelling the same bytes [CR#201.5,201.5c]. The
/// ranking is a claim-kind ordering over every identity, not a rule about any
/// one word: with no identity claim on those bytes the declared type still
/// wins, and the two readings are told apart only by the context.
#[test]
fn an_identity_claim_outranks_a_declared_type_over_the_same_bytes() {
    let parser = parser();
    let text =
        "Nightmare's power and toughness are each equal to the number of Swamps you control.";

    let (own_path, own_specificity) = selected_path_and_specificity(&parser, "Nightmare", text);
    assert!(
        own_path.contains(&"PossessiveOwnerPossessiveSelfReference".to_owned()),
        "the card's own name is its self-reference: {own_path:?}",
    );
    assert!(
        !own_path.contains(&"PossessiveOwnerPossessiveSingularNominal".to_owned()),
        "the declared type does not survive beside the identity: {own_path:?}",
    );
    assert!(
        own_specificity.contains(&SpecificityTier::Identity),
        "the identity tier is what the winner claims: {own_specificity:?}",
    );

    let (other_path, other_specificity) =
        selected_path_and_specificity(&parser, "Context Card", text);
    assert!(
        other_path.contains(&"PossessiveOwnerPossessiveSingularNominal".to_owned()),
        "the same bytes on another card are the declared type: {other_path:?}",
    );
    assert!(
        !other_path.contains(&"PossessiveOwnerPossessiveSelfReference".to_owned()),
        "no identity claim covers those bytes: {other_path:?}",
    );
    assert!(
        !other_specificity.contains(&SpecificityTier::Identity),
        "no position claims the identity tier: {other_specificity:?}",
    );
}

/// The attested corpus witness for the losing side: an ordinary declared type
/// as a possessive owner, with no identity claim competing for its bytes.
#[test]
fn a_declared_type_owns_a_possessive_without_an_identity_claim() {
    let parser = parser();
    let (path, specificity) = selected_path_and_specificity(
        &parser,
        "Lumbering Worldwagon",
        "This Vehicle's power is equal to the number of lands you control.",
    );
    assert!(
        path.contains(&"PossessiveOwnerPossessiveSingularNominal".to_owned()),
        "{path:?}",
    );
    assert!(
        !specificity.contains(&SpecificityTier::Identity),
        "{specificity:?}",
    );
}

/// `Colorless` is the eighth member of the color-property vocabulary
/// ([CR#105.2c]), and every nominal consumer of that vocabulary reaches it:
/// the attributive modifier, the `non` prefix modifier, and the fused nominal
/// head. The class is closed at those eight, so *colored* has no member.
#[test]
fn the_colorless_color_property_reaches_every_nominal_consumer() {
    let parser = parser();
    let context = context("Context Card");

    for (text, construction) in [
        (
            "Create a 1/1 colorless Servo artifact creature token.",
            "NominalModifierColorModifier",
        ),
        (
            "Destroy target noncolorless creature.",
            "NominalModifierNonColorModifier",
        ),
        ("Sacrifice a colorless.", "NominalFusedColorNominal"),
    ] {
        let parsed = parser
            .parse(text, &context)
            .unwrap_or_else(|error| panic!("{text:?} must select: {error:?}"));
        assert_eq!(parsed.render(&context, parser.environment()), text);
        let (path, _) = selected_path_and_specificity(&parser, "Context Card", text);
        assert!(
            path.contains(&construction.to_owned()),
            "{text:?} must select through {construction}: {path:?}",
        );
    }

    for text in [
        "Create a 1/1 colored Servo artifact creature token.",
        "Destroy target noncolored creature.",
        "Sacrifice a colored.",
    ] {
        assert!(
            parser.parse(text, &context).is_err(),
            "{text:?}: the color-property vocabulary is closed at eight members",
        );
    }
}
