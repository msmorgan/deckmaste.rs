//! Compiler-derived noun identity declarations.

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::NounInstanceRepr;
use crate::word::Vocabulary;

pub(crate) fn is_opaque(value: &NounInstance) -> bool {
    matches!(
        value.repr(),
        NounInstanceRepr::Singular(Noun::Opaque(_))
            | NounInstanceRepr::Plural(Noun::Opaque(_))
            | NounInstanceRepr::Mass(Noun::Opaque(_))
    )
}

fn known_noun(identity: NounInstance) -> Result<NounInstance, DeclarationViolation> {
    if is_opaque(&identity) || Vocabulary::new().render_noun(&identity).is_none() {
        return Err(DeclarationViolation {
            construction: "noun",
            requirement: "identity is a renderable known noun",
        });
    }
    Ok(identity)
}

fn opaque_noun(identity: NounInstance) -> Result<NounInstance, DeclarationViolation> {
    if !is_opaque(&identity) {
        return Err(DeclarationViolation {
            construction: "noun_opaque",
            requirement: "identity is opaque",
        });
    }
    Ok(identity)
}

#[allow(
    dead_code,
    reason = "the declaration-generated parts and linearization APIs own this destructurer"
)]
fn noun_identity(value: &NounInstance) -> NounInstance {
    value.clone()
}

deckmaste_constructions_macro::constructions! {
    group noun;

    construction noun: Noun {
        bind NounInstance via known_noun, noun_identity {
            identity: identity NounInstance via KnownNoun,
        }
        form only @ 0 = identity(identity);
        selection unique;
    }

    construction noun_opaque: Noun {
        bind NounInstance via opaque_noun, noun_identity {
            identity: identity NounInstance via OpaqueNoun,
        }
        form only @ 0 = identity(identity);
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&NOUN_DECLARATION];

pub(crate) fn linearize_with<V>(
    value: &NounInstance,
    visitor: &mut V,
) -> Result<(), deckmaste_construction_compiler::runtime::LinearizationError<V::Error>>
where
    V: deckmaste_construction_compiler::runtime::LinearizationVisitor,
{
    if is_opaque(value) {
        linearize_noun_opaque_with(value, visitor)
    } else {
        linearize_noun_with(value, visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fragment::Fragment;
    use crate::fragment::render_fragment;
    use crate::numeral::Numeral;
    use crate::syntax::NominalPhrase;
    use crate::syntax::NounPhrase;
    use crate::syntax::NumberLiteral;
    use crate::syntax::OpaqueLexeme;
    use crate::word::Verb;
    use crate::word::Vocab;

    fn render_bare_noun(noun: NounInstance) -> Result<String, crate::renderer::RenderError> {
        render_fragment(
            &Fragment::Nominal(NounPhrase::from_nominal_declaration(
                NominalPhrase::test_from_projection_parts(None, Vec::new(), noun, Vec::new()),
            )),
            "Test Card",
            false,
        )
    }

    #[test]
    fn generated_builders_keep_known_and_opaque_domains_disjoint() {
        let known = NounInstance::Singular(Noun::Word(Vocab::Card));
        let opaque = NounInstance::Mass(Noun::Opaque(OpaqueLexeme::new("blorple")));

        assert_eq!(build_noun(known.clone()).unwrap(), known);
        assert_eq!(build_noun_opaque(opaque.clone()).unwrap(), opaque);
        assert!(build_noun(opaque).is_err());
        assert!(build_noun_opaque(known).is_err());
    }

    #[test]
    fn generated_known_builder_rejects_every_unrenderable_identity_class() {
        // Mutations caught: admit a known value solely because it is not
        // opaque, or validate only one of form, noun kind, and countability.
        let one = NumberLiteral {
            value: 1,
            numeral: Numeral::Arabic(false),
        };
        for noun in [
            NounInstance::Plural(Noun::Die(one)),
            NounInstance::Mass(Noun::Die(one)),
            NounInstance::Mass(Noun::Agentive(Verb::Word(Vocab::Bid))),
            NounInstance::Singular(Noun::Word(Vocab::Destroy)),
            NounInstance::Mass(Noun::Word(Vocab::Card)),
            NounInstance::Singular(Noun::Word(Vocab::Damage)),
        ] {
            assert!(
                render_bare_noun(noun.clone()).is_err(),
                "fixture must exercise a genuinely missing lexical form: {noun:?}"
            );
            assert!(
                build_noun(noun.clone()).is_err(),
                "generated builder admitted unrenderable identity {noun:?}"
            );
        }
    }

    #[test]
    fn every_generated_known_builder_value_in_the_lexical_grid_linearizes() {
        // Mutations caught: let builder admission and the declaration-driven
        // renderer disagree for a vocabulary form or productive noun class.
        let mut candidates = Vec::new();
        for vocab in Vocab::ALL {
            for noun in [
                Noun::Word(*vocab),
                Noun::Gerund(Verb::Word(*vocab)),
                Noun::Agentive(Verb::Word(*vocab)),
            ] {
                candidates.extend([
                    NounInstance::Singular(noun.clone()),
                    NounInstance::Plural(noun.clone()),
                    NounInstance::Mass(noun),
                ]);
            }
        }
        let die = Noun::Die(NumberLiteral {
            value: 20,
            numeral: Numeral::Arabic(false),
        });
        candidates.extend([
            NounInstance::Singular(die.clone()),
            NounInstance::Plural(die.clone()),
            NounInstance::Mass(die),
        ]);

        for candidate in candidates {
            if let Ok(value) = build_noun(candidate.clone()) {
                assert!(
                    render_bare_noun(value).is_ok(),
                    "accepted identity did not linearize: {candidate:?}"
                );
            }
        }
    }
}
