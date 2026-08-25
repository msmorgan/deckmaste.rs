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
    let unknown_declaration_noun_arm = (!declaration_noun_arms.is_empty())
        .then(|| quote! { Lexical::DeclarationNoun(_, _) => Vec::new(), });
    let declaration_verb_arms = declaration_verb_arms(plan);
    let unknown_declaration_verb_arm = (!declaration_verb_arms.is_empty())
        .then(|| quote! { Lexical::DeclarationVerb(_, _) => Vec::new(), });
    let noun_lexeme_arm = noun_lexeme_arm(plan);
    let punctuation_literals = plan.runtime_punctuation_literals();
    let punctuation_arm = (!punctuation_literals.is_empty()).then(|| {
        quote! {
            Lexical::Literal(literal @ (#(#punctuation_literals)|*)) => input
                .punctuation_end(literal)
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
                #(#declaration_verb_arms,)*
                #unknown_declaration_verb_arm
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

fn verb_lexeme_arm(plan: &SemanticPlan) -> Option<TokenStream> {
    plan.runtime_verb_lexeme().map(|lexeme| {
        let ty = lexeme.name_ident();
        let candidates = lexeme.surfaces().iter().map(|row| {
            let member = emitted_ident(row.member(), Span::call_site());
            let agreement = match row.feature() {
                macro_ron::v2::SurfaceFeature::Bare => quote! { Agreement::Bare },
                macro_ron::v2::SurfaceFeature::ThirdPersonSingular => {
                    quote! { Agreement::ThirdPersonSingular }
                }
                macro_ron::v2::SurfaceFeature::Singular
                | macro_ron::v2::SurfaceFeature::Plural
                | macro_ron::v2::SurfaceFeature::Fixed => {
                    unreachable!("validated verb lexeme has the Agreement feature axis")
                }
            };
            let surface = syn::LitStr::new(row.surface(), Span::call_site());
            let onset = super::onset(row.onset());
            quote! { (#ty::#member, #agreement, #onset, #surface) }
        });
        quote! {
            Lexical::Verb(wanted, constraint) => [#(#candidates),*]
                .into_iter()
                .filter(|(lexeme, _, _, _)| *lexeme == wanted)
                .filter(|(_, agreement, _, _)| {
                    matches!(constraint, FeatureConstraint::Any)
                        || matches!(constraint, FeatureConstraint::Exact(expected) if expected == *agreement)
                })
                .filter_map(|(lexeme, agreement, onset, surface)| {
                    input.word_end(surface, terminal.right_boundary).map(|end| LexicalMatch {
                        end,
                        value: Leaf::Verb { lexeme, agreement, onset },
                        owner: None,
                    })
                })
                .collect(),
        }
    })
}

fn noun_lexeme_arm(plan: &SemanticPlan) -> Option<TokenStream> {
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
            macro_ron::v2::SurfaceFeature::Singular => quote! { Number::Singular },
            macro_ron::v2::SurfaceFeature::Plural => quote! { Number::Plural },
            macro_ron::v2::SurfaceFeature::Bare
            | macro_ron::v2::SurfaceFeature::ThirdPersonSingular
            | macro_ron::v2::SurfaceFeature::Fixed => {
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
                quote! { ::macro_ron::v2::DeclarationKind::Type }
            }
            crate::semantic::DeclarationKindFamily::Subtype => {
                quote! { ::macro_ron::v2::DeclarationKind::Subtype(_) }
            }
            crate::semantic::DeclarationKindFamily::SubtypeFamily(family) => {
                let family = crate::emit::subtype_category(*family);
                quote! { ::macro_ron::v2::DeclarationKind::Subtype(#family) }
            }
        });
        let position = crate::emit::grammar_position(codec.position());
        let number_feature = match codec.feature_axis() {
            crate::feature::Feature::Number => quote! { wanted },
            crate::feature::Feature::Agreement
            | crate::feature::Feature::Cardinality
            | crate::feature::Feature::Onset
            | crate::feature::Feature::PossessiveEnding => {
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
                            ::macro_ron::v2::SurfaceFeature::Singular => Number::Singular,
                            ::macro_ron::v2::SurfaceFeature::Plural => Number::Plural,
                            _ => continue,
                        };
                        let Some(declaration) = #declaration::from_reading(id) else {
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

fn declaration_verb_arms(plan: &SemanticPlan) -> Vec<TokenStream> {
    plan.runtime_declaration_verbs()
        .map(|(terminal_index, codec)| {
            let verb = codec.codec_ident();
            let declaration = codec.declaration_value_ident();
            let kinds = codec
                .kinds()
                .iter()
                .map(|kind| crate::emit::declaration_kind(*kind))
                .collect::<Vec<_>>();
            let frame_atoms = codec
                .frame_key()
                .atoms()
                .iter()
                .map(|atom| match atom {
                    crate::semantic::VerbFrameAtom::Literal(literal) => {
                        let literal = syn::LitStr::new(literal, Span::call_site());
                        quote! { VerbFrameAtom::Literal(#literal) }
                    }
                    crate::semantic::VerbFrameAtom::Amount => quote! { VerbFrameAtom::Amount },
                    crate::semantic::VerbFrameAtom::ObjectNounPhrase => {
                        quote! { VerbFrameAtom::ObjectNounPhrase }
                    }
                })
                .collect::<Vec<_>>();
            let closed_scan = codec.closed_lexeme().map(|closed| {
                let closed_plan = plan
                    .runtime_verb_lexeme()
                    .expect("validated declaration_verb closed branch is the verb lexeme");
                assert_eq!(closed, closed_plan.name());
                let closed_candidates = closed_plan.surfaces().iter().map(|row| {
                    let member = emitted_ident(row.member(), Span::call_site());
                    let agreement = match row.feature() {
                        macro_ron::v2::SurfaceFeature::Bare => quote! { Agreement::Bare },
                        macro_ron::v2::SurfaceFeature::ThirdPersonSingular => {
                            quote! { Agreement::ThirdPersonSingular }
                        }
                        macro_ron::v2::SurfaceFeature::Singular
                        | macro_ron::v2::SurfaceFeature::Plural
                        | macro_ron::v2::SurfaceFeature::Fixed => {
                            unreachable!("validated verb lexeme has the Agreement feature axis")
                        }
                    };
                    let surface = syn::LitStr::new(row.surface(), Span::call_site());
                    quote! { (#closed::#member, #agreement, #surface) }
                });
                quote! {
                    for (lexeme, agreement, surface) in [#(#closed_candidates),*] {
                        if (matches!(wanted, FeatureConstraint::Any)
                            || matches!(wanted, FeatureConstraint::Exact(expected) if expected == agreement))
                            && let Some(end) = input.word_end(surface, terminal.right_boundary)
                        {
                            matches.push(LexicalMatch {
                                end,
                                value: Leaf::#verb {
                                    verb: #verb::Lexeme(lexeme),
                                    agreement,
                                },
                                owner: None,
                            });
                        }
                    }
                }
            });
            let open_value = if codec.closed_lexeme().is_some() {
                quote! { #verb::Declaration(declaration) }
            } else {
                quote! { declaration }
            };
            quote! {
                Lexical::DeclarationVerb(#terminal_index, wanted) => {
                    let mut matches = Vec::new();
                    #closed_scan
                    let frame = VerbFrameKey::new(&[#(#frame_atoms),*]);
                    for agreement in [Agreement::Bare, Agreement::ThirdPersonSingular] {
                        if !matches!(wanted, FeatureConstraint::Any)
                            && !matches!(wanted, FeatureConstraint::Exact(expected) if expected == agreement)
                        {
                            continue;
                        }
                        for (end, id) in input.declaration_verb_readings(
                            input.position.byte_offset,
                            &[#(#kinds),*],
                            &frame,
                            agreement,
                        ) {
                            let Some(declaration) = #declaration::from_reading(id) else {
                                continue;
                            };
                            matches.push(LexicalMatch {
                                end,
                                value: Leaf::#verb {
                                    verb: #open_value,
                                    agreement,
                                },
                                owner: None,
                            });
                        }
                    }
                    matches
                }
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
            "(VerbLexeme :: InventedLemma , Agreement :: Bare , Onset :: Consonant , \"deal\")",
            "(VerbLexeme :: InventedLemma , Agreement :: ThirdPersonSingular , Onset :: Consonant , \"deals\")",
            "(VerbLexeme :: Be , Agreement :: Bare , Onset :: Vowel , \"are\")",
            "(VerbLexeme :: Be , Agreement :: ThirdPersonSingular , Onset :: Vowel , \"is\")",
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
            source.contains("Leaf :: Verb { lexeme , agreement , onset }"),
            "closed verb scanner discarded the normalized-row onset: {source}"
        );
        assert!(
            source.contains("matches ! (constraint , FeatureConstraint :: Any)")
                && source.contains(
                    "matches ! (constraint , FeatureConstraint :: Exact (expected) if expected == * agreement)"
                ),
            "closed verb candidates are not filtered by requested agreement: {source}"
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
