use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::emitted_ident;
use crate::identifier::key as identifier_key;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::BindingPlan;
use crate::semantic::SemanticPlan;

#[expect(
    clippy::too_many_lines,
    reason = "the one generated scanner keeps every terminal family in one exhaustive dispatch"
)]
pub(crate) fn emit(plan: &SemanticPlan) -> Vec<GeneratedItem> {
    let vocab_arms = plan.runtime_vocabs().map(|vocab| {
        let vocab_name = emitted_ident(vocab.name(), vocab.name_ident().span());
        let variants = vocab.variants().iter().map(|variant| {
            let member = emitted_ident(&identifier_key(variant.name()), variant.name().span());
            let word = variant.word();
            quote! { (#word, #vocab_name::#member) }
        });
        quote! {
            Lexical::#vocab_name => [#(#variants),*]
                .into_iter()
                .filter_map(|(running_text, value)| {
                    let end = if structural_surface {
                        input.structural_surface_end(running_text)
                    } else {
                        input.word_end(running_text, terminal.right_boundary)
                    };
                    end.map(|end| LexicalMatch {
                        end,
                        value: Leaf::#vocab_name(value),
                        owner: None,
                    })
                })
                .collect()
        }
    });
    let bound_arms = bound_arms(plan);
    let signed_decimal_arm = plan.runtime_signed_decimal().map(|codec| {
        let codec_name = codec.codec_ident();
        let sign_type = codec.sign_type();
        let positive = codec.positive_variant();
        let negative = codec.negative_variant();
        quote! {
            Lexical::#codec_name => {
                let offset = input.position.byte_offset;
                let prefix = usize::from(
                    input.position.prefix == PrefixPosition::WordOwnedSpace,
                );
                let Some(remainder) = input.text.get(offset..) else {
                    return Vec::new();
                };
                let Some(number) = (prefix == 0)
                    .then_some(remainder)
                    .or_else(|| remainder.strip_prefix(' '))
                else {
                    return Vec::new();
                };
                let (sign, digits) = number.strip_prefix('-').map_or(
                    (#sign_type::#positive, number),
                    |digits| (#sign_type::#negative, digits),
                );
                let digit_length = digits.bytes().take_while(u8::is_ascii_digit).count();
                let digits = &digits[..digit_length];
                let Some(magnitude) = (!digits.is_empty())
                    .then(|| digits.parse::<u32>().ok())
                    .flatten()
                else {
                    return Vec::new();
                };
                let candidate_length = usize::from(sign == #sign_type::#negative) + digit_length;
                let candidate = &number[..candidate_length];
                (magnitude.to_string() == digits)
                    .then(|| input.word_end(candidate, terminal.right_boundary))
                    .flatten()
                    .map(|end| LexicalMatch {
                        end,
                        value: Leaf::#codec_name(#codec_name { sign, magnitude }),
                        owner: None,
                    })
                    .into_iter()
                    .collect()
            }
        }
    });
    let unsigned_number_arms = plan.runtime_unsigned_numbers().map(|codec| {
        let codec_name = codec.codec_ident();
        let stem = crate::identifier::snake_case(codec.codec_name());
        let formatter = emitted_ident(&format!("format_{stem}"), Span::call_site());
        let parser = emitted_ident(&format!("parse_{stem}"), Span::call_site());
        if codec.kind() == crate::semantic::UnsignedNumberKind::UnsignedDecimal
            && codec.magnitude() == crate::semantic::UnsignedPrimitive::NonZeroU32
        {
            return quote! {
                Lexical::#codec_name => {
                    let offset = input.position.byte_offset;
                    let prefix = usize::from(
                        input.position.prefix == PrefixPosition::WordOwnedSpace,
                    );
                    let Some(remainder) = input.text.get(offset..) else {
                        return Vec::new();
                    };
                    let Some(number) = (prefix == 0)
                        .then_some(remainder)
                        .or_else(|| remainder.strip_prefix(' '))
                    else {
                        return Vec::new();
                    };
                    let candidate_length = number
                        .bytes()
                        .take_while(|byte| byte.is_ascii_digit() || *byte == b',')
                        .count();
                    let Some(candidate) = number.get(..candidate_length) else {
                        return Vec::new();
                    };
                    let Some(magnitude) = #parser(candidate) else {
                        return Vec::new();
                    };
                    input
                        .word_end(&#formatter(magnitude), terminal.right_boundary)
                        .map(|end| LexicalMatch {
                            end,
                            value: Leaf::#codec_name(#codec_name { magnitude }),
                            owner: None,
                        })
                        .into_iter()
                        .collect()
                }
            };
        }
        quote! {
            Lexical::#codec_name => {
                let offset = input.position.byte_offset;
                let prefix = usize::from(
                    input.position.prefix == PrefixPosition::WordOwnedSpace,
                );
                let Some(remainder) = input.text.get(offset..) else {
                    return Vec::new();
                };
                let Some(number) = (prefix == 0)
                    .then_some(remainder)
                    .or_else(|| remainder.strip_prefix(' '))
                else {
                    return Vec::new();
                };
                let initial = matches!(
                    input.position.case,
                    CasePosition::DocumentInitial | CasePosition::SentenceInitial,
                );
                (1..=number.len())
                    .filter(|end| number.is_char_boundary(*end))
                    .filter_map(|candidate_length| {
                        let candidate = number.get(..candidate_length)?;
                        let mut canonical = candidate.to_owned();
                        if initial {
                            canonical.get_mut(..1)?.make_ascii_lowercase();
                        }
                        let magnitude = #parser(&canonical)?;
                        let end = input.word_end(
                            &#formatter(magnitude),
                            terminal.right_boundary,
                        )?;
                        Some(LexicalMatch {
                            end,
                            value: Leaf::#codec_name(#codec_name { magnitude }),
                            owner: None,
                        })
                    })
                    .max_by_key(|candidate| candidate.end)
                    .into_iter()
                    .collect()
            }
        }
    });
    let context_identity_arms = plan.runtime_context_identities().map(|identity| {
        let ty = identity.ident();
        let aggregate = identity.aggregate_ident();
        let candidates = identity.arms().iter().map(|arm| {
            let variant = arm.variant();
            let accessor = arm.accessor();
            quote! { (#ty::#variant, input.context.#accessor()) }
        });
        quote! {
            Lexical::#aggregate => [#(#candidates),*]
                .into_iter()
                .filter(|(value, _)| value.valid_in(input.context))
                .filter_map(|(value, surface)| {
                    input.identity_end(surface, terminal.right_boundary).map(|end| LexicalMatch {
                        end,
                        value: Leaf::#aggregate(value),
                        owner: None,
                    })
                })
                .collect()
        }
    });
    let catalog_identity_arms =
        plan.runtime_catalog_identities()
            .map(|(terminal_index, identity)| {
                let provider = identity.provider();
                quote! {
                    Lexical::CatalogIdentity(#terminal_index) => input
                        .catalog_identity_reading(
                            CatalogProvider::#provider,
                            terminal.right_boundary,
                        )
                        .map(|(end, canonical_identity, onset)| LexicalMatch {
                            end,
                            value: Leaf::CatalogIdentity {
                                provider: CatalogProvider::#provider,
                                canonical_identity,
                                onset,
                                possessive_ending: possessive_ending_at(input.text, end),
                            },
                            owner: None,
                        })
                        .into_iter()
                        .collect()
                }
            });
    let unknown_catalog_identity_arm = plan
        .runtime_catalog_identities()
        .next()
        .is_some()
        .then(|| quote! { Lexical::CatalogIdentity(_) => Vec::new(), });
    let verb_lexeme_arm = verb_lexeme_arm(plan);
    let declaration_noun_arms = declaration_noun_arms(plan);
    let declaration_determinative_arms = declaration_determinative_arms(plan);
    let unknown_declaration_determinative_arm = (!declaration_determinative_arms.is_empty())
        .then(|| quote! { Lexical::DeclarationDeterminative(_) => Vec::new(), });
    let unknown_declaration_noun_arm = (!declaration_noun_arms.is_empty())
        .then(|| quote! { Lexical::DeclarationNoun(_, _) => Vec::new(), });
    let declaration_term_arms = declaration_term_arms(plan);
    let unknown_declaration_term_arm = (!declaration_term_arms.is_empty())
        .then(|| quote! { Lexical::DeclarationTerm(_) => Vec::new(), });
    let declaration_verb_arms = declaration_verb_arms(plan);
    let unknown_declaration_verb_arm = plan
        .runtime_declaration_verbs()
        .any(|(_, codec)| codec.feature_axis() == crate::feature::Feature::ConcordClass)
        .then(|| quote! { Lexical::DeclarationVerb(_, _) => Vec::new(), });
    let unknown_declaration_participle_arm = plan
        .runtime_declaration_verbs()
        .any(|(_, codec)| codec.feature_axis() == crate::feature::Feature::Participle)
        .then(|| quote! { Lexical::DeclarationParticiple(_) => Vec::new(), });
    let noun_lexeme_arm = noun_lexeme_arm(plan);
    let punctuation_literals = plan.runtime_punctuation_literals();
    let punctuation_arm = (!punctuation_literals.is_empty()).then(|| {
        quote! {
            Lexical::Literal(literal @ (#(#punctuation_literals)|*)) => (if structural_surface
                || !matches!(
                    terminal.right_boundary,
                    LexicalBoundary::Adjacent | LexicalBoundary::BothAdjacent,
                )
            {
                input.structural_surface_end(literal)
            } else {
                input.punctuation_end(literal)
            })
                .map(|end| LexicalMatch {
                    end,
                    value: Leaf::Literal(literal),
                    owner: None,
                })
                .into_iter()
                .collect(),
        }
    });
    let bound_arm = (!bound_arms.is_empty()).then(|| {
        quote! {
            #(#bound_arms)|* => scan_bound_terminal(input, terminal),
        }
    });

    let tokens = quote! {
        pub(crate) fn scan_lexical(
            input: &ScanInput<'_>,
            terminal: LexicalTerminal,
        ) -> Vec<LexicalMatch<Leaf, LexicalOwner>> {
            let structural_surface = matches!(
                terminal.owner,
                LexicalOwnerTemplate::Structural { .. }
                    | LexicalOwnerTemplate::TransitionedStatic { .. },
            );
            let mut matches: Vec<LexicalMatch<Leaf, LexicalOwner>> = match terminal.matcher {
                Lexical::EndOfInput => (input.position.byte_offset == input.text.len())
                    .then_some(LexicalMatch {
                        end: input.position.byte_offset,
                        value: Leaf::EndOfInput,
                        owner: None,
                    })
                    .into_iter()
                    .collect(),
                #punctuation_arm
                Lexical::Literal(literal) => (if structural_surface {
                    input.structural_surface_end(literal)
                } else {
                    input.word_end(literal, terminal.right_boundary)
                })
                    .map(|end| LexicalMatch {
                        end,
                        value: Leaf::Literal(literal),
                        owner: None,
                    })
                    .into_iter()
                    .collect(),
                #(#vocab_arms,)*
                #(#context_identity_arms,)*
                #(#catalog_identity_arms,)*
                #unknown_catalog_identity_arm
                #signed_decimal_arm
                #(#unsigned_number_arms,)*
                #verb_lexeme_arm
                #(#declaration_noun_arms,)*
                #unknown_declaration_noun_arm
                #(#declaration_determinative_arms,)*
                #unknown_declaration_determinative_arm
                #(#declaration_term_arms,)*
                #unknown_declaration_term_arm
                #(#declaration_verb_arms,)*
                #unknown_declaration_verb_arm
                #unknown_declaration_participle_arm
                #noun_lexeme_arm
                #bound_arm
                Lexical::Declaration(matcher) => input
                    .declaration_readings(matcher, terminal.right_boundary)
                    .into_iter()
                    .map(|(end, id, feature, onset)| LexicalMatch {
                        end,
                        value: Leaf::Declaration(DeclarationLeaf { id, feature, onset }),
                        owner: None,
                    })
                    .collect(),
            };
            for lexical_match in &mut matches {
                lexical_match.owner = terminal.owner.instantiate(&lexical_match.value);
            }
            matches
        }
    };

    vec![
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: "scan_lexical".to_owned(),
            },
            tokens,
            plan.runtime_scanner_origins(),
        ),
        GeneratedItem::new(
            ItemKey::Named {
                kind: NamedKind::Function,
                name: "possessive_ending_at".to_owned(),
            },
            quote! {
                fn possessive_ending_at(text: &str, end: usize) -> PossessiveEnding {
                    if text
                        .get(..end)
                        .and_then(|surface| surface.as_bytes().last())
                        .is_some_and(|byte| matches!(byte, b's' | b'S'))
                    {
                        PossessiveEnding::EndsInS
                    } else {
                        PossessiveEnding::Other
                    }
                }
            },
            plan.runtime_scanner_origins(),
        ),
    ]
}

fn declaration_determinative_arms(plan: &SemanticPlan) -> Vec<TokenStream> {
    plan.runtime_declaration_determinatives().map(|(terminal_index, codec)| {
        let ty = codec.codec_ident();
        let lemma = codec.lemma_ident();
        let closed = codec.closed().iter().flat_map(|member| member.realizations().iter().map(move |realization| {
            let surface = syn::LitStr::new(realization.surface(), Span::call_site());
            let onset = crate::macro_def::normalize_surface_onset(realization.surface(), None)
                .expect("validated declaration_determinative realization has an onset");
            let onset = crate::emit::onset(onset);
            let member_name = member.lemma();
            let number = match realization.phrase_number() {
                Some(crate::macro_def::DeterminativePhraseNumber::Singular) => quote! { DeterminerNumber::SingularOnly },
                Some(crate::macro_def::DeterminativePhraseNumber::Plural) => quote! { DeterminerNumber::PluralOnly },
                None => match member.number_license() {
                    crate::macro_def::DeterminativeNumberLicense::SingularOnly => quote! { DeterminerNumber::SingularOnly },
                    crate::macro_def::DeterminativeNumberLicense::PluralOnly => quote! { DeterminerNumber::PluralOnly },
                    crate::macro_def::DeterminativeNumberLicense::Both => quote! { DeterminerNumber::Both },
                },
            };
            let following_onset = realization.following_onset().map_or_else(
                || quote! { FeatureConstraint::Any },
                |onset| {
                    let onset = crate::emit::onset(onset);
                    quote! { FeatureConstraint::Exact(#onset) }
                },
            );
            let nominal = match member.nominal_license() {
                crate::macro_def::DeterminativeNominalLicense::AnyNominal => quote! { NominalLicense::AnyNominal },
                crate::macro_def::DeterminativeNominalLicense::CountNominal => quote! { NominalLicense::CountNominal },
                crate::macro_def::DeterminativeNominalLicense::BareSingularNoun => quote! { NominalLicense::BareSingularNoun },
                crate::macro_def::DeterminativeNominalLicense::MassOrPluralCount => quote! { NominalLicense::MassOrPluralCount },
            };
            let fused_head = match member.fused_head_license() {
                crate::macro_def::DeterminativeFusedHeadLicense::NominalOnly => quote! { FusedHeadLicense::NominalOnly },
                crate::macro_def::DeterminativeFusedHeadLicense::PartitiveOnly => quote! { FusedHeadLicense::PartitiveOnly },
                crate::macro_def::DeterminativeFusedHeadLicense::FusedHead => quote! { FusedHeadLicense::FusedHead },
                crate::macro_def::DeterminativeFusedHeadLicense::PluralPredeterminer => quote! { FusedHeadLicense::PluralPredeterminer },
            };
            quote! { if let Some(end) = input.word_end(#surface, terminal.right_boundary) { matches.push(LexicalMatch { end, value: Leaf::#ty { value: #ty::Closed(#lemma::#member_name), onset: #onset, following_onset: #following_onset, number_license: #number, fused_head_license: #fused_head, nominal_license: #nominal }, owner: None }); } }
        }));
        {
            quote! { Lexical::DeclarationDeterminative(#terminal_index) => { let mut matches = Vec::new(); #(#closed)* matches } }
        }
    }).collect()
}

fn verb_lexeme_arm(plan: &SemanticPlan) -> Option<TokenStream> {
    plan.runtime_verb_lexeme().map(|lexeme| {
        let ty = lexeme.name_ident();
        let candidates = lexeme.surfaces().iter().map(|row| {
            let member = emitted_ident(row.member(), Span::call_site());
            let concord_class = match row.feature() {
                crate::macro_def::SurfaceFeature::PLAIN => quote! { ConcordClass::Other },
                crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => {
                    quote! { ConcordClass::ThirdPersonSingular }
                }
                crate::macro_def::SurfaceFeature::Inflectional(_)
                | crate::macro_def::SurfaceFeature::Singular
                | crate::macro_def::SurfaceFeature::Plural
                | crate::macro_def::SurfaceFeature::Fixed
                | crate::macro_def::SurfaceFeature::BlockLabel => {
                    unreachable!("validated verb lexeme has the ConcordClass feature axis")
                }
            };
            let surface = syn::LitStr::new(row.surface(), Span::call_site());
            let onset = super::onset(row.onset());
            quote! { (#ty::#member, #concord_class, #onset, #surface) }
        });
        quote! {
            Lexical::Verb(wanted, constraint) => [#(#candidates),*]
                .into_iter()
                .filter(|(lexeme, _, _, _)| *lexeme == wanted)
                .filter(|(_, concord_class, _, _)| {
                    matches!(constraint, FeatureConstraint::Any)
                        || matches!(constraint, FeatureConstraint::Exact(expected) if expected == *concord_class)
                })
                .filter_map(|(lexeme, concord_class, onset, surface)| {
                    input.word_end(surface, terminal.right_boundary).map(|end| LexicalMatch {
                        end,
                        value: Leaf::Verb { lexeme, concord_class, onset },
                        owner: None,
                    })
                })
                .collect(),
        }
    })
}

fn noun_lexeme_arm(plan: &SemanticPlan) -> Option<TokenStream> {
    if plan.runtime_aggregate_noun().is_some() {
        return None;
    }
    plan.runtime_noun_lexeme().map(|lexeme| {
        let candidates = noun_surface_candidates(lexeme);
        quote! {
            Lexical::Noun(wanted) => [#(#candidates),*]
                .into_iter()
                .filter(|(_, number, _, _)| {
                    matches!(wanted, FeatureConstraint::Any)
                        || matches!(wanted, FeatureConstraint::Exact(expected) if expected == *number)
                })
                .filter_map(|(lexeme, number, onset, surface)| {
                    input.word_end(surface, terminal.right_boundary).map(|end| LexicalMatch {
                        end,
                        value: Leaf::Noun {
                            noun: lexeme,
                            number,
                            onset,
                            possessive_ending: possessive_ending_at(input.text, end),
                        },
                        owner: None,
                    })
                })
                .collect(),
        }
    })
}

fn noun_surface_candidates(
    lexeme: &crate::semantic::LexemePlan,
) -> impl Iterator<Item = TokenStream> + '_ {
    let ty = lexeme.name_ident();
    lexeme.surfaces().iter().map(move |row| {
        let member = emitted_ident(row.member(), Span::call_site());
        let number = match row.feature() {
            crate::macro_def::SurfaceFeature::Singular => {
                quote! { Number::Singular }
            }
            crate::macro_def::SurfaceFeature::Plural => {
                quote! { Number::Plural }
            }
            crate::macro_def::SurfaceFeature::Inflectional(_)
            | crate::macro_def::SurfaceFeature::Fixed
            | crate::macro_def::SurfaceFeature::BlockLabel => {
                unreachable!("validated noun lexeme has the Number feature axis")
            }
        };
        let surface = syn::LitStr::new(row.surface(), Span::call_site());
        let onset = super::onset(row.onset());
        quote! { (#ty::#member, #number, #onset, #surface) }
    })
}

fn declaration_noun_arms(plan: &SemanticPlan) -> Vec<TokenStream> {
    plan.runtime_declaration_nouns().map(|(terminal_index, codec)| {
        let noun = codec.codec_ident();
        let declaration = codec.declaration_value_ident();
        let closed_scan = codec.closed_lexeme().map(|closed| {
                let closed_plan = plan
                    .runtime_noun_lexeme()
                    .expect("validated declaration_noun closed branch is the noun lexeme");
                assert_eq!(closed, closed_plan.name());
                let closed_candidates = noun_surface_candidates(closed_plan);
                quote! {
                    for (lexeme, number, onset, surface) in [#(#closed_candidates),*] {
                        if matches!(wanted, FeatureConstraint::Any)
                            || matches!(wanted, FeatureConstraint::Exact(expected) if expected == number)
                        {
                            if let Some(end) = input.word_end(surface, terminal.right_boundary) {
                                matches.push(LexicalMatch {
                                    end,
                                    value: Leaf::#noun {
                                        noun: #noun::Lexeme(lexeme),
                                        number,
                                        onset,
                                        possessive_ending: possessive_ending_at(input.text, end),
                                    },
                                    owner: None,
                                });
                            }
                        }
                    }
                }
            });
        let allowed = codec.kinds().iter().map(|kind| match kind {
            crate::semantic::DeclarationKindFamily::Type => {
                quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::Type }
            }
            crate::semantic::DeclarationKindFamily::TurnPart => {
                quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::TurnPart }
            }
            crate::semantic::DeclarationKindFamily::Subtype => {
                quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::Subtype(_) }
            }
            crate::semantic::DeclarationKindFamily::SubtypeFamily(family) => {
                let family = crate::emit::subtype_category(*family);
                quote! { ::deckmaste_construction_core::macro_def::DeclarationKind::Subtype(#family) }
            }
        });
        let position = crate::emit::grammar_position(codec.position());
        let number_feature = match codec.feature_axis() {
            crate::feature::Feature::Number => quote! { wanted },
            crate::feature::Feature::ConcordClass
            | crate::feature::Feature::BareLocativeLicense
            | crate::feature::Feature::Cardinality
            | crate::feature::Feature::Compoundability
            | crate::feature::Feature::Countability
            | crate::feature::Feature::HomographLicense
            | crate::feature::Feature::MannerAnaphorClass
            | crate::feature::Feature::ModifierLicense
            | crate::feature::Feature::DeterminerNumber
            | crate::feature::Feature::FusedHeadLicense
            | crate::feature::Feature::PrepositionComplementKind
            | crate::feature::Feature::LocativeTemporalLicense
            | crate::feature::Feature::NominalForm
            | crate::feature::Feature::NominalLicense
            | crate::feature::Feature::BareLocativeComplement
            | crate::feature::Feature::PrepositionAttachment
            | crate::feature::Feature::Onset
            | crate::feature::Feature::Participle
            | crate::feature::Feature::PossessiveEnding
            | crate::feature::Feature::Properness
            | crate::feature::Feature::Relationality => {
                unreachable!("validated declaration_noun has the Number feature axis")
            }
        };
        quote! {
            Lexical::DeclarationNoun(#terminal_index, wanted) => {
                let mut matches = Vec::new();
                #closed_scan
                for (end, id, feature, onset) in input.declaration_noun_readings(
                    #position,
                    #number_feature,
                    terminal.right_boundary,
                ) {
                    if matches!(id.kind(), #(#allowed)|*) {
                        let number = match feature {
                            ::deckmaste_construction_core::macro_def::SurfaceFeature::Singular => Number::Singular,
                            ::deckmaste_construction_core::macro_def::SurfaceFeature::Plural => Number::Plural,
                            _ => continue,
                        };
                        let Some((locative_temporal_license, relationality, number_invariant)) =
                            input.environment.declaration_noun_features(&id)
                        else {
                            continue;
                        };
                        let Some(declaration) = #declaration::from_reading(
                            id,
                            locative_temporal_license,
                            relationality,
                            number_invariant,
                        ) else {
                            continue;
                        };
                        matches.push(LexicalMatch {
                            end,
                            value: Leaf::#noun {
                                noun: #noun::Declaration(declaration),
                                number,
                                onset,
                                possessive_ending: possessive_ending_at(input.text, end),
                            },
                            owner: None,
                        });
                    }
                }
                matches
            }
        }
    }).collect()
}

fn declaration_term_arms(plan: &SemanticPlan) -> Vec<TokenStream> {
    plan.runtime_declaration_terms()
        .map(|(terminal_index, codec)| {
            let position = crate::emit::grammar_position(codec.position());
            let kinds = codec
                .kinds()
                .iter()
                .map(|kind| crate::emit::declaration_kind(*kind));
            let params = codec.params().map_or_else(
                || quote! { None },
                |params| {
                    let params = params
                        .iter()
                        .map(|param| syn::LitStr::new(param, proc_macro2::Span::call_site()));
                    quote! { Some(&[#(#params),*]) }
                },
            );
            let feature = crate::emit::surface_feature(codec.feature());
            quote! {
                Lexical::DeclarationTerm(#terminal_index) => input
                    .declaration_term_readings(
                        #position,
                        &[#(#kinds),*],
                        #params,
                        #feature,
                        terminal.right_boundary,
                    )
                    .into_iter()
                    .map(|(end, id, onset)| LexicalMatch {
                        end,
                        value: Leaf::DeclarationTerm {
                            terminal_index: #terminal_index,
                            id,
                            onset,
                            possessive_ending: possessive_ending_at(input.text, end),
                        },
                        owner: None,
                    })
                    .collect()
            }
        })
        .collect()
}

#[expect(
    clippy::too_many_lines,
    reason = "declaration-verb scanning exhaustively emits both feature-axis branches"
)]
fn declaration_verb_arms(plan: &SemanticPlan) -> Vec<TokenStream> {
    plan.runtime_declaration_verbs()
        .map(|(terminal_index, codec)| {
            let verb = codec.codec_ident();
            let declaration = codec.declaration_value_ident();
            let frame_class = match codec.frame_key().class() {
                crate::semantic::VerbFrameClass::Predicate => {
                    quote! { VerbFrameClass::Predicate }
                }
                crate::semantic::VerbFrameClass::Auxiliary => {
                    quote! { VerbFrameClass::Auxiliary }
                }
                crate::semantic::VerbFrameClass::ProVerb => {
                    quote! { VerbFrameClass::ProVerb }
                }
            };
            let frame_atoms = codec
                .frame_key()
                .atoms()
                .iter()
                .map(|atom| match atom {
                    crate::semantic::VerbFrameAtom::Literal(literal) => {
                        let literal = syn::LitStr::new(literal, Span::call_site());
                        quote! { VerbFrameAtom::Literal(#literal) }
                    }
                    crate::semantic::VerbFrameAtom::Lex(terminal, variant) => {
                        let terminal = syn::LitStr::new(terminal, Span::call_site());
                        let variant = syn::LitStr::new(variant, Span::call_site());
                        quote! { VerbFrameAtom::Lex(#terminal, #variant) }
                    }
                    crate::semantic::VerbFrameAtom::OptionalLex(terminal, variant) => {
                        let terminal = syn::LitStr::new(terminal, Span::call_site());
                        let variant = syn::LitStr::new(variant, Span::call_site());
                        quote! { VerbFrameAtom::OptionalLex(#terminal, #variant) }
                    }
                    crate::semantic::VerbFrameAtom::Amount => quote! { VerbFrameAtom::Amount },
                    crate::semantic::VerbFrameAtom::ObjectNounPhrase => {
                        quote! { VerbFrameAtom::ObjectNounPhrase }
                    }
                    crate::semantic::VerbFrameAtom::PredicativeComplement => {
                        quote! { VerbFrameAtom::PredicativeComplement }
                    }
                    crate::semantic::VerbFrameAtom::Role(role) => {
                        let role = syn::LitStr::new(role, Span::call_site());
                        quote! { VerbFrameAtom::Role(#role) }
                    }
                    crate::semantic::VerbFrameAtom::OptionalRole(role) => {
                        let role = syn::LitStr::new(role, Span::call_site());
                        quote! { VerbFrameAtom::OptionalRole(#role) }
                    }
                })
                .collect::<Vec<_>>();
            let closed_scan = codec.closed_lexeme().map(|closed| {
                let closed_plan = plan
                    .lexeme(&closed.to_string())
                    .expect("validated declaration_verb closed branch is a verb lexeme");
                assert_eq!(closed, closed_plan.name());
                match codec.feature_axis() {
                    crate::feature::Feature::ConcordClass => {
                        let candidates = closed_plan.surfaces().iter().map(|row| {
                            let member = emitted_ident(row.member(), Span::call_site());
                            let concord_class = match row.feature() {
                                crate::macro_def::SurfaceFeature::PLAIN => quote! { ConcordClass::Other },
                                crate::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT => quote! { ConcordClass::ThirdPersonSingular },
                                _ => unreachable!("validated ConcordClass declaration verb has ConcordClass rows"),
                            };
                            let surface = syn::LitStr::new(row.surface(), Span::call_site());
                            let onset = crate::emit::onset(row.onset());
                            quote! { (#closed::#member, #concord_class, #onset, #surface) }
                        });
                        quote! {
                            for (lexeme, concord_class, onset, surface) in [#(#candidates),*] {
                                if (matches!(wanted, FeatureConstraint::Any)
                                    || matches!(wanted, FeatureConstraint::Exact(expected) if expected == concord_class))
                                    && let Some(end) = input.word_end(surface, terminal.right_boundary)
                                {
                                    matches.push(LexicalMatch { end, value: Leaf::#verb {
                                        verb: #verb::Lexeme(lexeme), concord_class, onset,
                                    }, owner: None });
                                }
                            }
                        }
                    }
                    crate::feature::Feature::Participle => {
                        let candidates = closed_plan.surfaces().iter().map(|row| {
                            debug_assert_eq!(row.feature(), crate::macro_def::SurfaceFeature::PAST_PARTICIPLE);
                            let member = emitted_ident(row.member(), Span::call_site());
                            let surface = syn::LitStr::new(row.surface(), Span::call_site());
                            let onset = crate::emit::onset(row.onset());
                            quote! { (#closed::#member, #onset, #surface) }
                        });
                        quote! {
                            for (lexeme, onset, surface) in [#(#candidates),*] {
                                if let Some(end) = input.word_end(surface, terminal.right_boundary) {
                                    matches.push(LexicalMatch { end, value: Leaf::#verb {
                                        verb: #verb::Lexeme(lexeme), onset,
                                    }, owner: None });
                                }
                            }
                        }
                    }
                    _ => unreachable!("validated declaration verb feature axis is closed"),
                }
            });
            let open_value = if codec.closed_lexeme().is_some() {
                quote! { #verb::Declaration(declaration) }
            } else {
                quote! { declaration }
            };
            match codec.feature_axis() {
                crate::feature::Feature::ConcordClass => quote! {
                    Lexical::DeclarationVerb(#terminal_index, wanted) => {
                    let mut matches = Vec::new();
                    #closed_scan
                    let frame = VerbFrameKey::with_class(
                        #frame_class,
                        &[#(#frame_atoms),*],
                    );
                    for concord_class in [ConcordClass::Other, ConcordClass::ThirdPersonSingular] {
                        if !matches!(wanted, FeatureConstraint::Any)
                            && !matches!(wanted, FeatureConstraint::Exact(expected) if expected == concord_class)
                        {
                            continue;
                        }
                        for (end, reading) in input.declaration_verb_readings(
                            input.position.byte_offset,
                            &frame,
                            match concord_class {
                                ConcordClass::Other => ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN,
                                ConcordClass::ThirdPersonSingular => ::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT,
                            },
                        ) {
                            let reference = reading.reference().clone();
                            let feature = match concord_class {
                                ConcordClass::Other => ::deckmaste_construction_core::macro_def::SurfaceFeature::PLAIN,
                                ConcordClass::ThirdPersonSingular => {
                                    ::deckmaste_construction_core::macro_def::SurfaceFeature::THIRD_PERSON_SINGULAR_PRESENT
                                }
                            };
                            let onset = reading.onset();
                            let Some(declaration) = #declaration::new(input.environment, reference) else {
                                continue;
                            };
                            matches.push(LexicalMatch {
                                end,
                                value: Leaf::#verb {
                                    verb: #open_value,
                                    concord_class,
                                    onset,
                                },
                                owner: None,
                            });
                        }
                    }
                    matches
                    }
                },
                crate::feature::Feature::Participle => quote! {
                    Lexical::DeclarationParticiple(#terminal_index) => {
                        let mut matches = Vec::new();
                        #closed_scan
                        let frame = VerbFrameKey::with_class(
                            #frame_class,
                            &[#(#frame_atoms),*],
                        );
                    for (end, reading) in input.declaration_verb_readings(
                        input.position.byte_offset,
                        &frame,
                            ::deckmaste_construction_core::macro_def::SurfaceFeature::PAST_PARTICIPLE,
                        ) {
                            let onset = reading.onset();
                            let Some(declaration) = #declaration::new(input.environment, reading.reference().clone()) else { continue; };
                            matches.push(LexicalMatch { end, value: Leaf::#verb {
                                verb: #open_value,
                                onset,
                            }, owner: None });
                        }
                        matches
                    }
                },
                _ => unreachable!("validated declaration verb feature axis is closed"),
            }
        })
        .collect()
}

fn bound_arms(plan: &SemanticPlan) -> Vec<TokenStream> {
    let mut arms = Vec::new();
    if plan.runtime_declaration_nouns().next().is_none() && plan.runtime_noun_binding().is_some() {
        arms.push(quote! { Lexical::Noun(_) });
    }
    arms.extend(plan.runtime_direct_bindings().map(binding_arm));
    arms.extend(plan.runtime_opaque_bindings().map(binding_arm));
    arms
}

fn binding_arm(binding: &BindingPlan) -> TokenStream {
    let variant = binding.lexical_variant().map_or_else(
        || emitted_ident(binding.name(), binding.origin_span()),
        |path| {
            path.segments
                .last()
                .expect("sealed lexical variant has a segment")
                .ident
                .clone()
        },
    );
    quote! { Lexical::#variant }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens as _;
    use syn::visit::Visit as _;

    fn generated_scanner_source() -> String {
        crate::test_support::generated_morphology_expansion()
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Named {
                        kind: crate::NamedKind::Function,
                        name,
                    } if name == "scan_lexical"
                )
            })
            .expect("generated morphology fixture emits its scanner")
            .tokens
            .to_string()
    }

    #[test]
    fn generated_morphology_scanner_uses_every_sealed_surface_row() {
        let source = generated_scanner_source();
        for row in [
            "(VerbLexeme :: InventedLemma , ConcordClass :: Other , Onset :: Consonant , \"deal\")",
            "(VerbLexeme :: InventedLemma , ConcordClass :: ThirdPersonSingular , Onset :: Consonant , \"deals\")",
            "(VerbLexeme :: Be , ConcordClass :: Other , Onset :: Vowel , \"are\")",
            "(VerbLexeme :: Be , ConcordClass :: ThirdPersonSingular , Onset :: Vowel , \"is\")",
            "(NounLexeme :: TwoWords , Number :: Singular , Onset :: Vowel , \"object\")",
            "(NounLexeme :: TwoWords , Number :: Plural , Onset :: Vowel , \"objects\")",
        ] {
            assert!(
                source.contains(row),
                "missing sealed scanner row `{row}`: {source}"
            );
        }
        assert!(
            !source.contains("\"be\""),
            "an overridden derived alias leaked: {source}"
        );
        assert!(
            !source.contains("\"bes\""),
            "an overridden derived alias leaked: {source}"
        );
        assert!(
            source.contains("Lexical :: Verb (wanted , constraint)"),
            "closed verb matcher does not carry its sealed feature constraint: {source}"
        );
        assert!(
            source.contains("Leaf :: Verb { lexeme , concord_class , onset }"),
            "closed verb scanner discarded the normalized-row onset: {source}"
        );
        assert!(
            source.contains("matches ! (constraint , FeatureConstraint :: Any)")
                && source.contains(
                    "matches ! (constraint , FeatureConstraint :: Exact (expected) if expected == * concord_class)"
                ),
            "closed verb candidates are not filtered by requested concord_class: {source}"
        );
    }

    #[test]
    fn generated_morphology_scanner_never_infers_lemma_from_member_identifier() {
        let source = generated_scanner_source();
        assert!(
            source.contains("\"object\""),
            "declared noun lemma is absent: {source}"
        );
        assert!(
            !source.contains("\"two_words\""),
            "member identifier became a surface: {source}"
        );
        assert!(
            !source.contains("\"two_wordss\""),
            "member identifier became a plural: {source}"
        );
    }

    #[test]
    fn generated_morphology_scanner_does_not_delegate_lexemes_to_binding_seam() {
        let source = generated_scanner_source();
        let function = syn::parse_str::<syn::ItemFn>(&source)
            .expect("generated scanner remains a syntactically valid function");
        let mut visitor = ClosedVerbArmVisitor::default();
        visitor.visit_item_fn(&function);

        assert_eq!(
            visitor.bodies.len(),
            1,
            "generated scanner must have exactly one current-shape closed verb arm: {source}"
        );
        let body = visitor.bodies[0].to_token_stream().to_string();
        assert!(
            !body.contains("scan_bound_terminal"),
            "closed two-field verb arm delegated through binding seam: {body}"
        );
    }

    #[test]
    fn circumfix_scanning_uses_the_generic_literal_dispatch() {
        let expansion = crate::generate(quote::quote! {
            construction item: Item {
                element ItemValue {}
                form item = "item";
            }
            construction bracketed: Root {
                element Bracketed { value: Item, }
                form bracketed = circumfix("[", value, "]");
            }
            construction braced: Root {
                element Braced { values: seq Item separated by "}{", }
                require len(values) >= 1;
                form braced = circumfix("{", values, "}");
            }
            root Root { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("circumfix scanner fixture generates");
        let source = expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Named {
                        kind: crate::NamedKind::Function,
                        name,
                    } if name == "scan_lexical"
                )
            })
            .expect("circumfix fixture emits its scanner")
            .tokens
            .to_string();

        assert!(source.contains("Lexical :: Literal (literal)"), "{source}");
        for forbidden in ["bracket", "brace", "mana", "loyalty"] {
            assert!(
                !source.to_ascii_lowercase().contains(forbidden),
                "scanner contains a circumfix-specific branch `{forbidden}`: {source}",
            );
        }
    }

    #[test]
    fn conditioned_determinative_scanning_carries_the_condition_without_raw_lookahead() {
        let expansion = crate::generate(quote::quote! {
            codec Head {
                generate declaration_determinative {
                    closed = [Article {
                        number_license = SingularOnly;
                        fused_head_license = NominalOnly;
                        nominal_license = CountNominal;
                        realizations = [
                            { surface = "an"; following_onset = Vowel; },
                            { surface = "a"; following_onset = Consonant; },
                        ];
                    }];
                }
            }
            construction wrapper: Wrapper {
                element WrapperValue { head: lex Head, }
                derive number = Values::Singular;
                derive onset = head.onset;
                form wrapper = lex(head);
            }
            root Wrapper { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect("conditioned determinative scanner fixture generates");
        let source = expansion
            .items()
            .iter()
            .find(|item| {
                matches!(
                    &item.key,
                    crate::ItemKey::Named {
                        kind: crate::NamedKind::Function,
                        name,
                    } if name == "scan_lexical"
                )
            })
            .expect("conditioned determinative fixture emits its scanner")
            .tokens
            .to_string();

        assert!(!source.contains("input . following_onset"), "{source}");
        assert!(
            source.contains("following_onset : FeatureConstraint :: Exact (Onset :: Vowel)")
                && source
                    .contains("following_onset : FeatureConstraint :: Exact (Onset :: Consonant)"),
            "the matched realization carries its erased right-context condition: {source}",
        );
        assert!(
            !source.contains("declaration_determinative_readings"),
            "a closed-only determinative must not scan undeclared open readings: {source}",
        );
    }

    #[test]
    fn declaration_determinative_rejects_open_declaration_kinds() {
        let error = crate::generate(quote::quote! {
            codec Head {
                generate declaration_determinative {
                    kinds = [KeywordAbility];
                }
            }
            construction wrapper: Wrapper {
                element WrapperValue { head: lex Head, }
                derive number = Values::Singular;
                derive onset = head.onset;
                form wrapper = lex(head);
            }
            root Wrapper { punctuation = "."; eoi = true; standalone_render = true; }
        })
        .expect_err("declaration determinatives are closed-only");
        assert!(
            error
                .to_string()
                .contains("declaration_determinative recipe accepts only a `closed` field"),
            "{error}",
        );
    }

    #[derive(Default)]
    struct ClosedVerbArmVisitor<'ast> {
        bodies: Vec<&'ast syn::Expr>,
    }

    impl<'ast> syn::visit::Visit<'ast> for ClosedVerbArmVisitor<'ast> {
        fn visit_arm(&mut self, arm: &'ast syn::Arm) {
            if arm.pat.to_token_stream().to_string() == "Lexical :: Verb (wanted , constraint)" {
                self.bodies.push(&arm.body);
            }
            syn::visit::visit_arm(self, arm);
        }
    }
}
