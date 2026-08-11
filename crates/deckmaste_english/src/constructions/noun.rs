//! Compiler-derived noun identity declarations.

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::word::Noun;
use crate::word::NounInstance;
use crate::word::NounInstanceKind;
use crate::word::Vocabulary;

pub(crate) fn is_opaque(value: &NounInstance) -> bool {
    matches!(
        value.kind(),
        NounInstanceKind::Singular(Noun::Opaque(_))
            | NounInstanceKind::Plural(Noun::Opaque(_))
            | NounInstanceKind::Mass(Noun::Opaque(_))
    )
}

fn is_known(value: &NounInstance) -> bool {
    !is_opaque(value) && Vocabulary::new().render_noun(value).is_some()
}

fn known_noun(identity: NounInstance) -> Result<NounInstance, DeclarationViolation> {
    if !is_known(&identity) {
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
        form only @ 0 inverse check(is_known) = identity(identity);
        selection unique;
    }

    construction noun_opaque: Noun {
        bind NounInstance via opaque_noun, noun_identity {
            identity: identity NounInstance via OpaqueNoun,
        }
        form only @ 0 inverse check(is_opaque) = identity(identity);
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
    linearize_noun_group_with(value, visitor)
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
        let known = NounInstance::unchecked_singular(Noun::Word(Vocab::Card));
        let opaque = NounInstance::unchecked_mass(Noun::Opaque(OpaqueLexeme::new("blorple")));

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
            NounInstance::unchecked_plural(Noun::Die(one)),
            NounInstance::unchecked_mass(Noun::Die(one)),
            NounInstance::unchecked_mass(Noun::Agentive(Verb::Word(Vocab::Bid))),
            NounInstance::unchecked_singular(Noun::Word(Vocab::Destroy)),
            NounInstance::unchecked_mass(Noun::Word(Vocab::Card)),
            NounInstance::unchecked_singular(Noun::Word(Vocab::Damage)),
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
                    NounInstance::unchecked_singular(noun.clone()),
                    NounInstance::unchecked_plural(noun.clone()),
                    NounInstance::unchecked_mass(noun),
                ]);
            }
        }
        let die = Noun::Die(NumberLiteral {
            value: 20,
            numeral: Numeral::Arabic(false),
        });
        candidates.extend([
            NounInstance::unchecked_singular(die.clone()),
            NounInstance::unchecked_plural(die.clone()),
            NounInstance::unchecked_mass(die),
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
