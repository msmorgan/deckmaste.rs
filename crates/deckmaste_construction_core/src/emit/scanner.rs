use proc_macro2::TokenStream;
use quote::quote;

use crate::identifier::emitted_ident;
use crate::identifier::key as identifier_key;
use crate::plan::GeneratedItem;
use crate::plan::ItemKey;
use crate::plan::NamedKind;
use crate::semantic::BindingPlan;
use crate::semantic::SemanticPlan;

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
                    input.word_end(running_text).map(|end| LexicalMatch {
                        end,
                        value: Leaf::#vocab_name(value),
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
                let prefix = usize::from(input.position.case == CasePosition::Continuation);
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
                    })
                })
                .collect()
        }
    });
    let punctuation_literals = plan.runtime_punctuation_literals();
    let punctuation_arm = (!punctuation_literals.is_empty()).then(|| {
        quote! {
            Lexical::Literal(literal @ (#(#punctuation_literals)|*)) => input
                .punctuation_end(literal)
                .map(|end| LexicalMatch {
                    end,
                    value: Leaf::Literal(literal),
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
        ) -> Vec<LexicalMatch<Leaf>> {
            match terminal.matcher {
                Lexical::EndOfInput => (input.position.byte_offset == input.text.len())
                    .then_some(LexicalMatch {
                        end: input.position.byte_offset,
                        value: Leaf::EndOfInput,
                    })
                    .into_iter()
                    .collect(),
                #punctuation_arm
                Lexical::Literal(literal) => input
                    .word_end(literal)
                    .map(|end| LexicalMatch {
                        end,
                        value: Leaf::Literal(literal),
                    })
                    .into_iter()
                    .collect(),
                #(#vocab_arms,)*
                #(#context_identity_arms,)*
                #signed_decimal_arm
                #bound_arm
                Lexical::Declaration(matcher) => input
                    .declaration_readings(matcher)
                    .into_iter()
                    .map(|(end, id, feature)| LexicalMatch {
                        end,
                        value: Leaf::Declaration(DeclarationLeaf { id, feature }),
                    })
                    .collect(),
            }
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

fn bound_arms(plan: &SemanticPlan) -> Vec<TokenStream> {
    let mut arms = Vec::new();
    if plan.runtime_noun_binding().is_some() || plan.runtime_noun_lexeme().is_some() {
        arms.push(quote! { Lexical::Noun(_) });
    }
    if plan.runtime_verb_lexeme().is_some() {
        arms.push(quote! { Lexical::Verb(_) });
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
