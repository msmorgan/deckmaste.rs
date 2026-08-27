//! Unit tests for the compiler-owned feature recipes declared in `constructions!`.

#[cfg(test)]
mod coordination_feature_recipes {
    use crate::constructions::*;

    fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
        NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: Box::new(NumericStage::UnqualifiedNumericStage(
                UnqualifiedNumericStage {
                    reference: Box::new(LocativeStage::UnqualifiedLocativeStage(
                        UnqualifiedLocativeStage {
                            reference: Box::new(ControllerStage::UnqualifiedControllerStage(
                                UnqualifiedControllerStage { reference },
                            )),
                        },
                    )),
                },
            )),
        })
    }

    fn singular_member(noun: CommonNoun) -> SingularCoordinationMember {
        SingularCoordinationMember::BareSingularCoordinationMember(BareSingularCoordinationMember {
            head: SingularHead::CommonSingularHead(CommonSingularHead { noun }),
        })
    }

    fn plural_member(noun: CommonNoun) -> PluralCoordinationMember {
        PluralCoordinationMember::BarePluralCoordinationMember(BarePluralCoordinationMember {
            head: PluralHead::CommonPluralHead(CommonPluralHead { noun }),
        })
    }

    fn singular_nominal(noun: CommonNoun) -> SingularNominal {
        SingularNominal::BareSingularNominal(BareSingularNominal {
            head: SingularHead::CommonSingularHead(CommonSingularHead { noun }),
        })
    }

    #[test]
    fn coordination_scopes_derive_exact_number_and_agreement() {
        let singular_shared = noun_phrase(UnqualifiedReference::OrdinarySingularReference(
            OrdinarySingularReference {
                phrase: DeterminerPhrase::TargetCoordinationDeterminerPhrase(
                    TargetCoordinationDeterminerPhrase {
                        coordination: SingularNominalCoordination::SingularOrNominalCoordination(
                            SingularOrNominalCoordination::new(vec![
                                singular_member(CommonNoun::Player),
                                singular_member(CommonNoun::Opponent),
                            ])
                            .expect("binary singular coordination satisfies minimum arity"),
                        ),
                    },
                ),
            },
        ));
        let plural_shared = noun_phrase(UnqualifiedReference::OrdinaryPluralReference(
            OrdinaryPluralReference {
                selector: PluralSelector::TargetPluralCoordinationSelector(
                    TargetPluralCoordinationSelector {
                        coordination: PluralNominalCoordination::PluralOrNominalCoordination(
                            PluralOrNominalCoordination::new(vec![
                                plural_member(CommonNoun::Player),
                                plural_member(CommonNoun::Opponent),
                            ])
                            .expect("binary plural coordination satisfies minimum arity"),
                        ),
                    },
                ),
            },
        ));
        let full_np = noun_phrase(UnqualifiedReference::CoordinatedNounPhrase(
            CoordinatedNounPhrase {
                coordination: FullNounPhraseCoordination::FullAndNounPhraseCoordination(
                    FullAndNounPhraseCoordination::new(vec![
                        DeterminerPhrase::TargetDeterminerPhrase(TargetDeterminerPhrase {
                            nominal: singular_nominal(CommonNoun::Player),
                        }),
                        DeterminerPhrase::TargetDeterminerPhrase(TargetDeterminerPhrase {
                            nominal: singular_nominal(CommonNoun::Opponent),
                        }),
                    ])
                    .expect("binary full-NP coordination satisfies minimum arity"),
                ),
            },
        ));

        assert_eq!(number_for_noun_phrase(&singular_shared), Number::Singular);
        assert_eq!(
            agreement_for_noun_phrase(&singular_shared),
            Agreement::ThirdPersonSingular
        );
        assert_eq!(number_for_noun_phrase(&plural_shared), Number::Plural);
        assert_eq!(agreement_for_noun_phrase(&plural_shared), Agreement::Bare);
        assert_eq!(number_for_noun_phrase(&full_np), Number::Plural);
        assert_eq!(agreement_for_noun_phrase(&full_np), Agreement::Bare);
    }
}

#[cfg(test)]
mod reference_onset_recipes {
    use crate::constructions::*;

    fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
        NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: Box::new(NumericStage::UnqualifiedNumericStage(
                UnqualifiedNumericStage {
                    reference: Box::new(LocativeStage::UnqualifiedLocativeStage(
                        UnqualifiedLocativeStage {
                            reference: Box::new(ControllerStage::UnqualifiedControllerStage(
                                UnqualifiedControllerStage { reference },
                            )),
                        },
                    )),
                },
            )),
        })
    }

    fn variable_reference(count: Variable) -> NounPhrase {
        noun_phrase(UnqualifiedReference::VariableReference(VariableReference {
            count,
            selector: PluralSelector::UnmarkedPluralSelector(UnmarkedPluralSelector {
                nominal: PluralNominal::BarePluralNominal(BarePluralNominal {
                    head: PluralHead::CommonPluralHead(CommonPluralHead {
                        noun: CommonNoun::Player,
                    }),
                }),
            }),
        }))
    }

    #[test]
    fn variable_reference_onset_is_derived_from_its_count_lexeme() {
        let context = ParseContext::new("Context Card", false, Onset::Consonant)
            .expect("test context is valid");
        let environment = crate::environment::canonical_test_environment();

        assert_eq!(
            onset_for_noun_phrase(&variable_reference(Variable::X), &context, &environment),
            Onset::Vowel,
        );
        assert_eq!(
            onset_for_noun_phrase(&variable_reference(Variable::Y), &context, &environment),
            Onset::Consonant,
        );
    }

    #[test]
    fn self_reference_onset_is_the_explicit_context_realization_fact() {
        let environment = crate::environment::canonical_test_environment();
        for (name, is_legendary, onset, spelling) in [
            (
                "+2 Mace",
                false,
                Onset::Consonant,
                SelfReferenceSpelling::Full,
            ),
            (
                "Aang, A Lot to Learn",
                true,
                Onset::Vowel,
                SelfReferenceSpelling::Abbreviated,
            ),
        ] {
            let context = ParseContext::new(name, is_legendary, onset)
                .expect("nonempty opaque card-name context is valid");
            let reference = SourceSelfReference::new(spelling, &context)
                .expect("chosen spelling is licensed by the context");
            assert_eq!(
                onset_for_noun_phrase(
                    &noun_phrase(UnqualifiedReference::SelfReference(reference)),
                    &context,
                    &environment,
                ),
                onset,
                "{name}",
            );
        }
    }
}
