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
