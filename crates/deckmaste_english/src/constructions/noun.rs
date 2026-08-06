//! Compiler-derived noun identity declarations.

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::word::Noun;
use crate::word::NounInstance;

pub(crate) fn is_opaque(value: &NounInstance) -> bool {
    matches!(
        value,
        NounInstance::Singular(Noun::Opaque(_))
            | NounInstance::Plural(Noun::Opaque(_))
            | NounInstance::Mass(Noun::Opaque(_))
    )
}

fn known_noun(identity: NounInstance) -> Result<NounInstance, DeclarationViolation> {
    if is_opaque(&identity) {
        return Err(DeclarationViolation {
            construction: "noun",
            requirement: "identity is known",
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
    use crate::syntax::OpaqueLexeme;
    use crate::word::Vocab;

    #[test]
    fn generated_builders_keep_known_and_opaque_domains_disjoint() {
        let known = NounInstance::Singular(Noun::Word(Vocab::Card));
        let opaque = NounInstance::Mass(Noun::Opaque(OpaqueLexeme::new("blorple")));

        assert_eq!(build_noun(known.clone()).unwrap(), known);
        assert_eq!(build_noun_opaque(opaque.clone()).unwrap(), opaque);
        assert!(build_noun(opaque).is_err());
        assert!(build_noun_opaque(known).is_err());
    }
}
