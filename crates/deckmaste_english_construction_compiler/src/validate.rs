//! Layer-2 validation. `ValidatedGroup` is deliberately the ONLY door to
//! the emitter: its constructor is private to this module, so unvalidated
//! emission is unrepresentable.

use crate::diag::Diagnostic;
use crate::diag::sort_key;
use crate::model::GroupDeclaration;

pub struct ValidatedGroup<'a> {
    group: &'a GroupDeclaration,
}

impl<'a> ValidatedGroup<'a> {
    pub fn group(&self) -> &'a GroupDeclaration {
        self.group
    }
}

pub fn validate(group: &GroupDeclaration) -> Result<ValidatedGroup<'_>, Vec<Diagnostic>> {
    let mut diags: Vec<Diagnostic> = Vec::new();
    checks(group, &mut diags);
    if diags.is_empty() {
        Ok(ValidatedGroup { group })
    } else {
        diags.sort_by_key(sort_key);
        Err(diags)
    }
}

// Each task in this plan appends one check family here.
fn checks(_group: &GroupDeclaration, _diags: &mut Vec<Diagnostic>) {}

#[cfg(test)]
pub(crate) mod fixtures {
    use crate::model::AstShape;
    use crate::model::ConstructionDeclaration;
    use crate::model::FieldBinding;
    use crate::model::FieldKind;
    use crate::model::FieldPath;
    use crate::model::FormDeclaration;
    use crate::model::GroupDeclaration;
    use crate::model::SelectionPromise;
    use crate::model::Spanned;
    use crate::model::SurfaceAtom;

    /// Smallest valid group: one construction, one scalar field, one form
    /// producing it. Every validator test perturbs a clone of this.
    pub(crate) fn minimal_group() -> GroupDeclaration {
        GroupDeclaration {
            name: Spanned::call_site("noun_coordination".to_owned()),
            elements: vec![],
            constructions: vec![ConstructionDeclaration {
                id: Spanned::call_site("noun_phrase_coordination".to_owned()),
                category: Spanned::call_site("NounPhrase".to_owned()),
                internal: false,
                ast: AstShape::Bind {
                    path: Spanned::call_site("crate::syntax::CoordinatedNounPhrase".to_owned()),
                    fields: vec![FieldBinding {
                        field: Spanned::call_site("conjunction".to_owned()),
                        kind: FieldKind::Scalar {
                            codec: Spanned::call_site("Conjunction".to_owned()),
                        },
                    }],
                },
                constraints: vec![],
                witnesses: vec![],
                forms: vec![FormDeclaration {
                    name: Spanned::call_site("binary".to_owned()),
                    ordinal: Spanned::call_site(0),
                    surface: vec![SurfaceAtom::Lexeme(FieldPath::call_site("conjunction"))],
                    guard: None,
                }],
                dominance: vec![],
                selection: SelectionPromise::Packed,
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fixtures::minimal_group;
    use super::*;

    #[test]
    fn minimal_group_validates() {
        let group = minimal_group();
        let validated = validate(&group).expect("minimal group is valid");
        assert_eq!(validated.group().constructions.len(), 1);
    }
}
