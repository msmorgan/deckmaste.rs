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
                        input.word_end(running_text)
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
                    .then(|| input.word_end(candidate))
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
                    input.identity_end(surface).map(|end| LexicalMatch {
                        end,
                        value: Leaf::#aggregate(value),
                        owner: None,
                    })
                })
                .collect()
        }
    });
    let verb_lexeme_arm = verb_lexeme_arm(plan);
    let declaration_noun_arm = declaration_noun_arm(plan);
    let noun_lexeme_arm = plan
        .runtime_declaration_noun()
        .is_none()
        .then(|| noun_lexeme_arm(plan))
        .flatten();
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
                LexicalOwnerTemplate::Structural { .. },
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
                    input.word_end(literal)
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
                #signed_decimal_arm
                #verb_lexeme_arm
                #declaration_noun_arm
                #noun_lexeme_arm
                #bound_arm
                Lexical::Declaration(matcher) => input
                    .declaration_readings(matcher)
                    .into_iter()
                    .map(|(end, id, feature)| LexicalMatch {
                        end,
                        value: Leaf::Declaration(DeclarationLeaf { id, feature }),
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

    vec![GeneratedItem::new(
        ItemKey::Named {
            kind: NamedKind::Function,
            name: "scan_lexical".to_owned(),
        },
        tokens,
        plan.runtime_scanner_origins(),
    )]
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
            quote! { (#ty::#member, #agreement, #surface) }
        });
        quote! {
            Lexical::Verb(wanted, constraint) => [#(#candidates),*]
                .into_iter()
                .filter(|(lexeme, _, _)| *lexeme == wanted)
                .filter(|(_, agreement, _)| {
                    matches!(constraint, FeatureConstraint::Any)
                        || matches!(constraint, FeatureConstraint::Exact(expected) if expected == *agreement)
                })
                .filter_map(|(lexeme, agreement, surface)| {
                    input.word_end(surface).map(|end| LexicalMatch {
                        end,
                        value: Leaf::Verb { lexeme, agreement },
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
                .filter(|(_, number, _)| {
                    matches!(wanted, FeatureConstraint::Any)
                        || matches!(wanted, FeatureConstraint::Exact(expected) if expected == *number)
                })
                .filter_map(|(lexeme, number, surface)| {
                    input.word_end(surface).map(|end| LexicalMatch {
                        end,
                        value: Leaf::Noun { noun: lexeme, number },
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
        quote! { (#ty::#member, #number, #surface) }
    })
}

fn declaration_noun_arm(plan: &SemanticPlan) -> Option<TokenStream> {
    plan.runtime_declaration_noun().map(|codec| {
        let noun = codec.codec_ident();
        let declaration = codec.declaration_value_ident();
        let closed_plan = plan
            .runtime_noun_lexeme()
            .expect("validated declaration_noun closed branch is the noun lexeme");
        let closed_candidates = noun_surface_candidates(closed_plan);
        let allowed = codec.kinds().iter().map(|kind| match kind {
            crate::semantic::DeclarationKindFamily::Type => {
                quote! { ::macro_ron::v2::DeclarationKind::Type }
            }
            crate::semantic::DeclarationKindFamily::Subtype => {
                quote! { ::macro_ron::v2::DeclarationKind::Subtype(_) }
            }
        });
        let position = crate::emit::grammar_position(codec.position());
        let number_feature = match codec.feature_axis() {
            crate::feature::Feature::Number => quote! { wanted },
            crate::feature::Feature::Agreement => {
                unreachable!("validated declaration_noun has the Number feature axis")
            }
        };
        quote! {
            Lexical::Noun(wanted) => {
                let mut matches = Vec::new();
                for (lexeme, number, surface) in [#(#closed_candidates),*] {
                    if matches!(wanted, FeatureConstraint::Any)
                        || matches!(wanted, FeatureConstraint::Exact(expected) if expected == number)
                    {
                        if let Some(end) = input.word_end(surface) {
                            matches.push(LexicalMatch {
                                end,
                                value: Leaf::Noun {
                                    noun: #noun::Lexeme(lexeme),
                                    number,
                                },
                                owner: None,
                            });
                        }
                    }
                }
                for (end, id, feature) in input.declaration_noun_readings(#position, #number_feature) {
                    if matches!(id.kind(), #(#allowed)|*) {
                        let number = match feature {
                            ::macro_ron::v2::SurfaceFeature::Singular => Number::Singular,
                            ::macro_ron::v2::SurfaceFeature::Plural => Number::Plural,
                            _ => continue,
                        };
                        let Some(declaration) = #declaration::from_reading(id, feature) else {
                            continue;
                        };
                        matches.push(LexicalMatch {
                            end,
                            value: Leaf::Noun {
                                noun: #noun::Declaration(declaration),
                                number,
                            },
                            owner: None,
                        });
                    }
                }
                matches
            }
        }
    })
}

fn bound_arms(plan: &SemanticPlan) -> Vec<TokenStream> {
    let mut arms = Vec::new();
    if plan.runtime_declaration_noun().is_none() && plan.runtime_noun_binding().is_some() {
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
            "(VerbLexeme :: InventedLemma , Agreement :: Bare , \"deal\")",
            "(VerbLexeme :: InventedLemma , Agreement :: ThirdPersonSingular , \"deals\")",
            "(VerbLexeme :: Be , Agreement :: Bare , \"are\")",
            "(VerbLexeme :: Be , Agreement :: ThirdPersonSingular , \"is\")",
            "(NounLexeme :: TwoWords , Number :: Singular , \"object\")",
            "(NounLexeme :: TwoWords , Number :: Plural , \"objects\")",
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
