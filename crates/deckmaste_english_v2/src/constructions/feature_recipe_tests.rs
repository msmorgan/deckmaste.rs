//! Unit tests for the compiler-owned feature recipes declared in
//! `constructions!`.

#[cfg(test)]
mod finite_copula_inflectional_forms {
    use deckmaste_construction_core::macro_def::InflectionalForm;

    use crate::constructions::ConcordClass;
    use crate::constructions::FiniteCopula;
    use crate::constructions::concord_class_for_finite_copula;

    #[test]
    fn every_finite_copula_concord_class_matches_its_inflectional_form() {
        for (copula, form, expected) in [
            (
                FiniteCopula::Is,
                InflectionalForm::ThirdPersonSingularPresent,
                ConcordClass::ThirdPersonSingular,
            ),
            (
                FiniteCopula::Isnt,
                InflectionalForm::ThirdPersonSingularPresent,
                ConcordClass::ThirdPersonSingular,
            ),
            (
                FiniteCopula::Are,
                InflectionalForm::Plain,
                ConcordClass::Other,
            ),
            (
                FiniteCopula::Arent,
                InflectionalForm::Plain,
                ConcordClass::Other,
            ),
            (
                FiniteCopula::Was,
                InflectionalForm::Preterite,
                ConcordClass::ThirdPersonSingular,
            ),
            (
                FiniteCopula::Were,
                InflectionalForm::Preterite,
                ConcordClass::Other,
            ),
        ] {
            let derived = concord_class_for_finite_copula(copula);
            assert_eq!(
                derived, expected,
                "{copula:?} derives the wrong Concord Class"
            );
            let applicability = form.concord_class_applicability();
            let permitted = match derived {
                ConcordClass::Other => applicability.other,
                ConcordClass::ThirdPersonSingular => applicability.third_person_singular,
            };
            assert!(
                permitted,
                "{copula:?} pairs {derived:?} with an Inflectional Form that cannot realize it",
            );
        }
    }
}

#[cfg(test)]
mod coordination_feature_recipes {
    use crate::constructions::*;

    fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
        NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
                UnqualifiedPostmodifiedReference {
                    reference: Box::new(reference),
                },
            )),
        })
    }

    fn member(head: Head) -> CoordinationMember {
        CoordinationMember::BareCoordinationMember(BareCoordinationMember { head })
    }

    fn singular_nominal(noun: CommonNoun) -> Nominal {
        Nominal::BareSingularNominal(BareSingularNominal {
            head: Head::NounSingularHead(NounSingularHead {
                noun: Noun::Lexeme(noun),
            }),
        })
    }

    fn determinative(number: Number) -> Determinative {
        match number {
            Number::Singular => {
                Determinative::TargetingMarkerDeterminative(TargetingMarkerDeterminative {
                    marker: TargetingMarker::Target,
                })
            }
            Number::Plural => Determinative::PluralSimpleDeterminative(PluralSimpleDeterminative {
                head: DeterminativeHead::Closed(DeterminativeHeadLemma::All),
            }),
        }
    }

    fn determined(det: Determinative, nominal: Nominal) -> UnqualifiedReference {
        UnqualifiedReference::DeterminedNominal(
            DeterminedNominal::new(Determiner::Headed(det), nominal)
                .expect("determiner and nominal number agree"),
        )
    }

    #[test]
    fn coordination_scopes_derive_exact_number_and_concord_class() {
        let singular_shared = noun_phrase(determined(
            determinative(Number::Singular),
            Nominal::SingularCoordinationNominalValue(SingularCoordinationNominalValue {
                coordination: NominalCoordination::OrNominalCoordination(
                    OrNominalCoordination::new(vec![
                        member(Head::NounSingularHead(NounSingularHead {
                            noun: Noun::Lexeme(CommonNoun::Player),
                        })),
                        member(Head::NounSingularHead(NounSingularHead {
                            noun: Noun::Lexeme(CommonNoun::Opponent),
                        })),
                    ])
                    .expect("binary singular coordination satisfies minimum arity"),
                ),
            }),
        ));
        let plural_shared = noun_phrase(determined(
            determinative(Number::Plural),
            Nominal::PluralCoordinationNominalValue(PluralCoordinationNominalValue {
                coordination: NominalCoordination::OrNominalCoordination(
                    OrNominalCoordination::new(vec![
                        member(Head::NounPluralHead(NounPluralHead {
                            noun: Noun::Lexeme(CommonNoun::Player),
                        })),
                        member(Head::NounPluralHead(NounPluralHead {
                            noun: Noun::Lexeme(CommonNoun::Opponent),
                        })),
                    ])
                    .expect("binary plural coordination satisfies minimum arity"),
                ),
            }),
        ));
        let full_np = noun_phrase(UnqualifiedReference::CoordinatedNounPhrase(
            CoordinatedNounPhrase {
                coordination: FullNounPhraseCoordination::FullAndNounPhraseCoordination(
                    FullAndNounPhraseCoordination::new(Box::new(vec![
                        PostmodifiedReference::UnqualifiedPostmodifiedReference(
                            UnqualifiedPostmodifiedReference {
                                reference: Box::new(determined(
                                    determinative(Number::Singular),
                                    singular_nominal(CommonNoun::Player),
                                )),
                            },
                        ),
                        PostmodifiedReference::UnqualifiedPostmodifiedReference(
                            UnqualifiedPostmodifiedReference {
                                reference: Box::new(determined(
                                    determinative(Number::Singular),
                                    singular_nominal(CommonNoun::Opponent),
                                )),
                            },
                        ),
                    ]))
                    .expect("binary full-NP coordination satisfies minimum arity"),
                ),
            },
        ));

        assert_eq!(number_for_noun_phrase(&singular_shared), Number::Singular);
        assert_eq!(
            concord_class_for_noun_phrase(&singular_shared),
            ConcordClass::ThirdPersonSingular
        );
        assert_eq!(number_for_noun_phrase(&plural_shared), Number::Plural);
        assert_eq!(
            concord_class_for_noun_phrase(&plural_shared),
            ConcordClass::Other
        );
        assert_eq!(number_for_noun_phrase(&full_np), Number::Plural);
        assert_eq!(concord_class_for_noun_phrase(&full_np), ConcordClass::Other);
    }
}

#[cfg(test)]
mod reference_onset_recipes {
    use crate::constructions::*;

    fn noun_phrase(reference: UnqualifiedReference) -> NounPhrase {
        NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: Box::new(PostmodifiedReference::UnqualifiedPostmodifiedReference(
                UnqualifiedPostmodifiedReference {
                    reference: Box::new(reference),
                },
            )),
        })
    }

    fn variable_reference(count: Variable) -> NounPhrase {
        noun_phrase(UnqualifiedReference::DeterminedNominal(
            DeterminedNominal::new(
                Determiner::Headed(Determinative::VariableQuantifyingDeterminer(
                    VariableQuantifyingDeterminer { count },
                )),
                Nominal::BarePluralNominal(BarePluralNominal {
                    head: Head::NounPluralHead(NounPluralHead {
                        noun: Noun::Lexeme(CommonNoun::Player),
                    }),
                }),
            )
            .expect("variable quantifier licenses a plural nominal"),
        ))
    }

    #[test]
    fn variable_quantifying_determiners_share_the_nominal_reference_onset() {
        let context = ParseContext::new("Context Card", false, Onset::Consonant)
            .expect("test context is valid");
        let environment = crate::environment::canonical_test_environment();

        assert_eq!(
            onset_for_noun_phrase(&variable_reference(Variable::X), &context, &environment),
            Onset::Consonant,
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

#[cfg(test)]
mod mass_nominal_feature_recipes {
    use crate::constructions::*;

    #[test]
    fn zero_determiner_licenses_a_mass_nominal() {
        let nominal = Nominal::MassNominal(MassNominal {
            noun: MassNoun::MassNoun(
                MassNounValue::new(Noun::Lexeme(CommonNoun::Damage))
                    .expect("damage is declared mass"),
            ),
        });
        assert_eq!(nominal_form_for_nominal(&nominal), NominalForm::MassNoun);
        assert_eq!(number_for_nominal(&nominal), Number::Singular);
        assert!(DeterminedNominal::new(Determiner::Zero, nominal).is_some());
    }
}

#[cfg(test)]
mod preposition_license_recipes {
    use crate::constructions::*;

    fn possessed_singular_complement(noun: CommonNoun) -> PrepositionalComplement {
        let noun = Noun::Lexeme(noun);
        let head = Head::NounSingularHead(
            NounSingularHead::new(noun).expect("licensed complement head is a count noun"),
        );
        let nominal = Nominal::BareSingularNominal(BareSingularNominal { head });
        let reference = UnqualifiedReference::PossessedReference(PossessedReference {
            possessor: PossessiveDeterminerPronoun::Your,
            nominal,
            admissible_sites: AdmissibleSites::empty(),
        });
        let postmodified = PostmodifiedReference::UnqualifiedPostmodifiedReference(
            UnqualifiedPostmodifiedReference {
                reference: Box::new(reference),
            },
        );
        let phrase = NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: Box::new(postmodified),
        });
        PrepositionalComplement::Object(Box::new(Object::ObjectNominal(NominalObject {
            value: Box::new(phrase),
        })))
    }

    #[test]
    fn turn_license_relays_from_the_lexeme_through_the_nominal() {
        let noun = Noun::Lexeme(CommonNoun::Turn);
        assert_eq!(
            locative_temporal_license_for_common_noun(CommonNoun::Turn),
            LocativeTemporalLicense::OfAndTemporalLicensed,
        );
        assert_eq!(
            locative_temporal_license_for_noun(&noun),
            LocativeTemporalLicense::OfAndTemporalLicensed,
        );
        let head =
            Head::NounSingularHead(NounSingularHead::new(noun).expect("turn is a count noun"));
        assert_eq!(
            locative_temporal_license_for_head(&head),
            LocativeTemporalLicense::OfAndTemporalLicensed,
        );
        let nominal = Nominal::BareSingularNominal(BareSingularNominal { head });
        assert_eq!(
            locative_temporal_license_for_nominal(&nominal),
            LocativeTemporalLicense::OfAndTemporalLicensed,
        );
        let reference = UnqualifiedReference::PossessedReference(PossessedReference {
            possessor: PossessiveDeterminerPronoun::Your,
            nominal,
            admissible_sites: AdmissibleSites::empty(),
        });
        let postmodified = PostmodifiedReference::UnqualifiedPostmodifiedReference(
            UnqualifiedPostmodifiedReference {
                reference: Box::new(reference),
            },
        );
        let phrase = NounPhrase::QualifiedNounPhrase(QualifiedNounPhrase {
            reference: Box::new(postmodified),
        });
        assert_eq!(
            locative_temporal_license_for_noun_phrase(&phrase),
            LocativeTemporalLicense::OfAndTemporalLicensed,
        );
        let object = Object::ObjectNominal(NominalObject {
            value: Box::new(phrase),
        });
        assert_eq!(
            locative_temporal_license_for_object(&object),
            LocativeTemporalLicense::OfAndTemporalLicensed,
        );
        let complement = PrepositionalComplement::Object(Box::new(object));
        assert_eq!(
            locative_temporal_license_for_prepositional_complement(&complement),
            LocativeTemporalLicense::OfAndTemporalLicensed,
        );
        assert_eq!(
            preposition_complement_kind_for_preposition(Preposition::On),
            PrepositionComplementKind::OnComplement,
        );
        assert!(preposition_complement_is_licensed(
            &complement,
            PrepositionComplementKind::OnComplement,
            locative_temporal_license_for_prepositional_complement(&complement),
        ));
    }

    #[test]
    fn amended_under_and_hand_data_enforce_the_rejection_reasons() {
        assert_eq!(
            preposition_attachment_for_preposition(Preposition::Under),
            PrepositionAttachment::SelectedOnly,
        );
        for preposition in [Preposition::On, Preposition::At, Preposition::During] {
            assert_eq!(
                preposition_attachment_for_preposition(preposition),
                PrepositionAttachment::AdjunctCapable,
                "{preposition:?}",
            );
        }

        let complement = possessed_singular_complement(CommonNoun::Hand);
        let license = locative_temporal_license_for_prepositional_complement(&complement);
        assert_eq!(license, LocativeTemporalLicense::InLicensed);
        assert!(preposition_complement_is_licensed(
            &complement,
            PrepositionComplementKind::InComplement,
            license,
        ));
        assert!(preposition_complement_is_licensed(
            &complement,
            PrepositionComplementKind::OnComplement,
            license,
        ));
        let modifier = PrepositionalPhrase::PrepositionalPhrase(PrepositionalPhraseValue {
            preposition: Preposition::On,
            complement: Box::new(complement),
            admissible_sites: AdmissibleSites::empty(),
        });
        assert!(!nominal_preposition_is_licensed(
            &modifier,
            Relationality::QualifiedRelational,
            LocativeTemporalLicense::ObjectAttachmentLicensed,
            PrepositionAttachment::AdjunctCapable,
            PrepositionComplementKind::OnComplement,
            Relationality::NonRelational,
            license,
        ));
    }
}
