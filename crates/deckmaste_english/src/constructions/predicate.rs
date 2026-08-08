//! Compiler-derived English predicate declarations.

#![allow(
    dead_code,
    clippy::unnecessary_wraps,
    reason = "the inactive declaration is exercised through generated adapters in tests"
)]

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::grammar::Features;
use crate::grammar::VerbAnalysis;
use crate::grammar::VerbPhrase;
use crate::grammar::reduction::reduce_verb_phrase_base;
use crate::word::Vocabulary;

type Verb = VerbAnalysis;

fn make_verb_phrase_base(head: VerbAnalysis) -> Result<VerbPhrase, DeclarationViolation> {
    if Vocabulary::new()
        .render_verb_instance(head.instance())
        .is_none()
    {
        return Err(DeclarationViolation {
            construction: "verb_phrase_base",
            requirement: "the lexical head has a renderable verb form",
        });
    }
    Ok(VerbPhrase::from_base(head))
}

fn verb_phrase_base_parts(value: &VerbPhrase) -> VerbAnalysis {
    value.base_head()
}

fn is_verb_phrase_base(value: &VerbPhrase) -> bool {
    value.is_declaration_base()
}

deckmaste_constructions_macro::constructions! {
    group predicate;

    construction verb_phrase_base: VerbPhrase {
        bind VerbPhrase via make_verb_phrase_base, verb_phrase_base_parts {
            head: hole Verb,
        }
        derive features: Features = reduce_verb_phrase_base(head);
        form only @ 0 inverse check(is_verb_phrase_base) = head;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&PREDICATE_DECLARATION];

#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;
    use crate::catalog::Catalogs;
    use crate::features::VerbSlot;
    use crate::grammar::GeneratedActivation;
    use crate::grammar::Nonterminal;
    use crate::word::PredicateFrame;
    use crate::word::Verb as LexicalVerb;
    use crate::word::VerbInstance;
    use crate::word::Vocab;

    #[derive(Default)]
    struct PredicateRecorder {
        source: Option<String>,
    }

    impl deckmaste_construction_compiler::runtime::LinearizationVisitor for PredicateRecorder {
        type Error = std::convert::Infallible;

        fn literal(&mut self, _literal: &'static str) -> Result<(), Self::Error> {
            Ok(())
        }

        fn subtree<T: Any>(
            &mut self,
            category: &'static str,
            value: &T,
        ) -> Result<(), Self::Error> {
            assert_eq!(category, "Verb");
            let head = (value as &dyn Any)
                .downcast_ref::<VerbAnalysis>()
                .expect("the Verb subtree retains its typed lexical analysis");
            self.source = Vocabulary::new().render_verb_instance(head.instance());
            Ok(())
        }

        fn scalar<T: Any>(&mut self, _codec: &'static str, _value: &T) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    fn attack_head() -> VerbAnalysis {
        VerbAnalysis::new(
            VerbInstance {
                verb: LexicalVerb::Word(Vocab::Attack),
                slot: VerbSlot::Imperative,
            },
            PredicateFrame::OPEN,
        )
    }

    #[test]
    fn verb_phrase_base_builds_destructures_linearizes_and_reparses() {
        // Mutations caught: bypass lexical-form validation in the checked
        // builder, bypass the generated whole-value inverse check so a
        // non-base predicate linearizes, accept a feature argument whose
        // category is not `Features::Verb`, or let the handwritten mirror
        // mask a broken inactive generated lowering path.
        assert_eq!(
            PREDICATE_DECLARATION
                .constructions
                .iter()
                .map(|construction| construction.id)
                .collect::<Vec<_>>(),
            ["verb_phrase_base"]
        );
        assert!(
            !crate::constructions::GROUPS
                .iter()
                .any(|group| group.name == "predicate"),
            "the Task 1 declaration compiles without production activation"
        );
        let head = attack_head();
        let predicate = build_verb_phrase_base(head.clone()).expect("a lexical head builds");
        assert_eq!(parts_verb_phrase_base(&predicate), head);

        let wrong_form = VerbAnalysis::new(
            VerbInstance {
                verb: LexicalVerb::Word(Vocab::Card),
                slot: VerbSlot::Imperative,
            },
            PredicateFrame::OPEN,
        );
        assert!(
            Vocabulary::new()
                .render_verb_instance(wrong_form.instance())
                .is_none(),
            "the negative fixture must lack the declared surface form"
        );
        assert_eq!(
            build_verb_phrase_base(wrong_form),
            Err(DeclarationViolation {
                construction: "verb_phrase_base",
                requirement: "the lexical head has a renderable verb form",
            })
        );

        let verb_features = Features::Verb {
            slot: VerbSlot::Imperative,
            frame: PredicateFrame::OPEN,
            head_is_copular: false,
            object_gap_requires_rules_object: false,
        };
        assert_eq!(
            reduce_predicate_features(0, &[Some(&verb_features)]),
            Some(Features::VerbPhrase {
                form: crate::grammar::PredicateForm::Imperative,
                passive: false,
                object: crate::grammar::PredicateObjectState::None,
                indirect_object: false,
                selected_preposition: false,
                phase: crate::grammar::PredicateAttachmentPhase::Object,
                frame: PredicateFrame::OPEN,
                bare: true,
                head_is_copular: false,
                object_gap_requires_rules_object: false,
                subjunctive: false,
            })
        );
        assert!(reduce_predicate_features(0, &[Some(&Features::None)]).is_none());

        let mut recorder = PredicateRecorder::default();
        linearize_predicate_verb_phrase_with(&predicate, &mut recorder)
            .expect("the named VerbPhrase inverse selects the base declaration");
        let source = recorder
            .source
            .expect("the inverse visits the lexical head");
        assert_eq!(source, "attack");

        let mut wrong_shape = predicate.clone();
        wrong_shape.test_add_preverb_modifier(crate::syntax::PreverbModifier::Not);
        let mut rejected = PredicateRecorder::default();
        assert!(matches!(
            linearize_predicate_verb_phrase_with(&wrong_shape, &mut rejected),
            Err(
                deckmaste_construction_compiler::runtime::LinearizationError::NoMatchingConstruction {
                    group: "predicate"
                }
            )
        ));
        assert!(rejected.source.is_none());

        let parsed = crate::grammar::parse_nonterminal_with_activation(
            &source,
            &Catalogs::default(),
            Nonterminal::VerbPhrase,
            GeneratedActivation::Groups(GROUPS),
        )
        .expect("the generated base declaration reparses its own inverse");
        assert!(parsed.construction_decisions().iter().any(|decision| {
            decision.selected().as_str() == "verb_phrase_base"
                && decision.owner() == crate::construction::ConstructionOwner::Generated
        }));
    }
}
