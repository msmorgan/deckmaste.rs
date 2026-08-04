//! Chart assembly for generated construction groups: category allocation,
//! atom-to-production mapping, and rule registration. Production assemblies
//! activate nothing here; test assemblies opt in through
//! [`GeneratedActivation::Groups`].

use std::collections::BTreeMap;

use deckmaste_construction_compiler::runtime::AtomData;
use deckmaste_construction_compiler::runtime::ConstructionData;
use deckmaste_construction_compiler::runtime::FieldKindData;
use deckmaste_construction_compiler::runtime::GroupData;

use super::EnglishLexicalSlot;
use super::Expected;
use super::Nonterminal;
use super::rules::GeneratedRuleRef;
use super::rules::RuleBuilder;
use super::rules::RuleImpl;
use crate::construction::ConstructionId;
use crate::construction::ProductionId;
use crate::surface::Punctuation;

#[derive(Debug, Clone, Copy)]
pub(super) enum GeneratedActivation {
    /// Every production assembly. No generated group exists in the grammar.
    Inactive,
    /// Test assemblies only: register exactly these groups, in slice order.
    #[cfg(test)]
    #[allow(
        dead_code,
        reason = "constructed by full-parse generated-activation tests once a pilot group lands; Tasks 4-5 pin assembly directly via internal_categories/register_generated/merged_registry"
    )]
    Groups(&'static [&'static GroupData]),
}

impl GeneratedActivation {
    pub(super) fn groups(self) -> Option<&'static [&'static GroupData]> {
        match self {
            Self::Inactive => None,
            #[cfg(test)]
            Self::Groups(groups) => Some(groups),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum GeneratedAssemblyError {
    /// A non-internal category with no engine mapping (the table grows with
    /// the pilot; today every generated category must be `internal`).
    UnknownCategory {
        construction: &'static str,
        category: &'static str,
    },
    /// The M3 literal table admits `","` only.
    UnsupportedLiteral {
        construction: &'static str,
        literal: &'static str,
    },
    /// A lexeme codec without a closed engine slot.
    UnknownLexemeCodec {
        construction: &'static str,
        codec: &'static str,
    },
    /// Multi-segment atom paths have no chart meaning yet.
    UnsupportedAtomPath {
        construction: &'static str,
        path: &'static str,
    },
    /// An atom naming a field this construction does not declare.
    UnknownAtomField {
        construction: &'static str,
        field: &'static str,
    },
    /// Sequence holes are the pilot's seed/extend design work; optional and
    /// scalar holes have no production meaning at all.
    UnsupportedHoleKind {
        construction: &'static str,
        field: &'static str,
    },
    /// `bind` is legal only while the family's owner row is Handwritten;
    /// activation as a generated group is the Generated owner state.
    BindWhileGenerated { construction: &'static str },
}

/// Deterministic internal-category ids: the sorted set of `internal`
/// constructions' category names across the active groups, in order. Shared
/// categories collapse to one id; ids never depend on group or registration
/// order.
pub(super) fn internal_categories(groups: &[&'static GroupData]) -> BTreeMap<&'static str, u16> {
    let mut names = BTreeMap::new();
    for group in groups {
        for construction in group.constructions {
            if construction.internal {
                names.insert(construction.category, 0_u16);
            }
        }
    }
    for (index, (_, id)) in names.iter_mut().enumerate() {
        *id = u16::try_from(index).expect("more than u16::MAX internal categories");
    }
    names
}

/// Non-internal categories name engine nonterminals. Empty until the pilot
/// declarations land; the check that consults it is live now.
fn engine_category(name: &str) -> Option<Nonterminal> {
    let _ = name;
    None
}

/// Resolves a category referenced from a `Hole` atom's `Subtree` field. Such
/// a reference names a category, not a specific construction, so it may
/// legitimately land on any active group's internal category or (once the
/// table grows) an engine nonterminal.
fn category_nonterminal(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
    category: &'static str,
) -> Result<Nonterminal, GeneratedAssemblyError> {
    if let Some(&id) = cats.get(category) {
        return Ok(Nonterminal::Generated(id));
    }
    engine_category(category).ok_or(GeneratedAssemblyError::UnknownCategory {
        construction: construction.id,
        category,
    })
}

/// Resolves a construction's own left-hand-side category. Membership is
/// decided by the construction's OWN `internal` flag, never by whether some
/// other construction happens to declare the same category name as internal
/// — otherwise a non-internal construction sharing a category name with an
/// internal sibling would be silently admitted as `Nonterminal::Generated`
/// instead of surfacing `UnknownCategory`.
fn lhs_category_nonterminal(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
) -> Result<Nonterminal, GeneratedAssemblyError> {
    if construction.internal {
        let id = *cats.get(construction.category).expect(
            "internal_categories collected this construction's category from the same group set",
        );
        return Ok(Nonterminal::Generated(id));
    }
    engine_category(construction.category).ok_or(GeneratedAssemblyError::UnknownCategory {
        construction: construction.id,
        category: construction.category,
    })
}

fn codec_slot(codec: &'static str) -> Option<EnglishLexicalSlot> {
    match codec {
        "Conjunction" => Some(EnglishLexicalSlot::Conjunction),
        "Comma" => Some(EnglishLexicalSlot::Punctuation(Punctuation::Comma)),
        _ => None,
    }
}

fn atom_expected(
    cats: &BTreeMap<&'static str, u16>,
    construction: &'static ConstructionData,
    atom: AtomData,
) -> Result<Expected<Nonterminal, EnglishLexicalSlot>, GeneratedAssemblyError> {
    match atom {
        AtomData::Literal(",") => Ok(Expected::Lexical(EnglishLexicalSlot::Punctuation(
            Punctuation::Comma,
        ))),
        AtomData::Literal(literal) => Err(GeneratedAssemblyError::UnsupportedLiteral {
            construction: construction.id,
            literal,
        }),
        AtomData::Hole(path) | AtomData::Lexeme(path) => {
            if path.contains('.') {
                return Err(GeneratedAssemblyError::UnsupportedAtomPath {
                    construction: construction.id,
                    path,
                });
            }
            let field = construction
                .fields
                .iter()
                .find(|field| field.name == path)
                .ok_or(GeneratedAssemblyError::UnknownAtomField {
                    construction: construction.id,
                    field: path,
                })?;
            match (atom, field.kind) {
                (AtomData::Hole(_), FieldKindData::Subtree { category, .. }) => Ok(
                    Expected::Nonterminal(category_nonterminal(cats, construction, category)?),
                ),
                (AtomData::Lexeme(_), FieldKindData::Scalar { codec }) => codec_slot(codec)
                    .map(Expected::Lexical)
                    .ok_or(GeneratedAssemblyError::UnknownLexemeCodec {
                        construction: construction.id,
                        codec,
                    }),
                _ => Err(GeneratedAssemblyError::UnsupportedHoleKind {
                    construction: construction.id,
                    field: path,
                }),
            }
        }
    }
}

/// Registers every form of every construction of every active group, in
/// declaration order, with EXPLICIT ordinals from the declaration data.
pub(super) fn register_generated(
    builder: &mut RuleBuilder,
    groups: &[&'static GroupData],
    cats: &BTreeMap<&'static str, u16>,
) -> Result<(), GeneratedAssemblyError> {
    for group in groups {
        for (construction_index, construction) in group.constructions.iter().enumerate() {
            if construction.bind_path.is_some() {
                return Err(GeneratedAssemblyError::BindWhileGenerated {
                    construction: construction.id,
                });
            }
            let lhs = lhs_category_nonterminal(cats, construction)?;
            for (form_index, form) in construction.forms.iter().enumerate() {
                let mut rhs = Vec::with_capacity(form.atoms.len());
                for atom in form.atoms {
                    rhs.push(atom_expected(cats, construction, *atom)?);
                }
                builder.add_generated(
                    RuleImpl::Generated(GeneratedRuleRef {
                        group,
                        construction: construction_index,
                        form: form_index,
                    }),
                    ProductionId {
                        construction: ConstructionId::new(construction.id),
                        ordinal: form.ordinal,
                    },
                    lhs,
                    rhs,
                );
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use deckmaste_construction_compiler::runtime::AtomData;
    use deckmaste_construction_compiler::runtime::ConstructionData;
    use deckmaste_construction_compiler::runtime::FieldData;
    use deckmaste_construction_compiler::runtime::FieldKindData;
    use deckmaste_construction_compiler::runtime::FormData;
    use deckmaste_construction_compiler::runtime::GroupData;

    use super::super::EnglishLexicalSlot;
    use super::super::Expected;
    use super::super::rules::RegistrationOrder;
    use super::super::rules::RuleBuilder;
    use super::GeneratedAssemblyError;
    use super::internal_categories;
    use super::register_generated;
    use crate::surface::Punctuation;

    const fn construction(
        id: &'static str,
        category: &'static str,
        internal: bool,
        fields: &'static [FieldData],
        forms: &'static [FormData],
    ) -> ConstructionData {
        ConstructionData {
            id,
            category,
            internal,
            own_type: Some("Synthetic"),
            bind_path: None,
            deserialize: false,
            selection_unique: false,
            dominates: &[],
            fields,
            witnesses: &[],
            forms,
        }
    }

    const WORD_FIELDS: &[FieldData] = &[FieldData {
        name: "word",
        kind: FieldKindData::Scalar {
            codec: "Conjunction",
        },
    }];
    const WORD_FORM: &[FormData] = &[FormData {
        name: "only",
        ordinal: 0,
        guarded: false,
        atoms: &[AtomData::Lexeme("word")],
    }];

    #[test]
    fn unknown_category_is_refused() {
        const CONSTRUCTIONS: &[ConstructionData] = &[construction(
            "np_only",
            "NounPhrase",
            false,
            WORD_FIELDS,
            WORD_FORM,
        )];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnknownCategory {
                construction: "np_only",
                category: "NounPhrase",
            }
        );
    }

    /// Controller finding: a non-internal construction sharing a category
    /// name with an internal sibling in the same group must still surface
    /// `UnknownCategory` — membership is decided by the construction's own
    /// `internal` flag, never by name presence in the internal-category map.
    #[test]
    fn non_internal_construction_sharing_an_internal_category_name_is_refused() {
        const CONSTRUCTIONS: &[ConstructionData] = &[
            construction("internal_member", "Shared", true, WORD_FIELDS, WORD_FORM),
            construction(
                "non_internal_member",
                "Shared",
                false,
                WORD_FIELDS,
                WORD_FORM,
            ),
        ];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        assert!(
            cats.contains_key("Shared"),
            "the internal member must seed the category map"
        );
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnknownCategory {
                construction: "non_internal_member",
                category: "Shared",
            }
        );
    }

    #[test]
    fn unsupported_literal_is_refused() {
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Literal("and")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("lit_and", "Internal", true, &[], FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedLiteral {
                construction: "lit_and",
                literal: "and"
            }
        );
    }

    #[test]
    fn comma_literal_is_admitted() {
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Literal(",")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("lit_comma", "Internal", true, &[], FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        register_generated(&mut builder, &[&GROUP], &cats).expect("comma literal must assemble");
        let rule_book = builder.finish(RegistrationOrder::Normal);
        assert_eq!(
            rule_book.rules[0].rhs,
            vec![Expected::Lexical(EnglishLexicalSlot::Punctuation(
                Punctuation::Comma
            ))]
        );
    }

    #[test]
    fn unknown_lexeme_codec_is_refused() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "who",
            kind: FieldKindData::Scalar { codec: "Person" },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Lexeme("who")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("codec_test", "Internal", true, FIELDS, FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnknownLexemeCodec {
                construction: "codec_test",
                codec: "Person",
            }
        );
    }

    #[test]
    fn sequence_hole_is_a_deferred_error() {
        const FIELDS: &[FieldData] = &[FieldData {
            name: "items",
            kind: FieldKindData::Sequence { element: "Foo" },
        }];
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Hole("items")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("seq_test", "Internal", true, FIELDS, FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedHoleKind {
                construction: "seq_test",
                field: "items",
            }
        );
    }

    #[test]
    fn dotted_atom_path_is_refused() {
        const FORM: &[FormData] = &[FormData {
            name: "only",
            ordinal: 0,
            guarded: false,
            atoms: &[AtomData::Hole("members.last")],
        }];
        const CONSTRUCTIONS: &[ConstructionData] =
            &[construction("dotted", "Internal", true, &[], FORM)];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::UnsupportedAtomPath {
                construction: "dotted",
                path: "members.last",
            }
        );
    }

    #[test]
    fn bind_while_generated_is_refused() {
        const CONSTRUCTIONS: &[ConstructionData] = &[ConstructionData {
            id: "bound",
            category: "Internal",
            internal: true,
            own_type: None,
            bind_path: Some("x::Y"),
            deserialize: false,
            selection_unique: false,
            dominates: &[],
            fields: &[],
            witnesses: &[],
            forms: &[],
        }];
        const GROUP: GroupData = GroupData {
            name: "g",
            elements: &[],
            constructions: CONSTRUCTIONS,
        };
        let cats = internal_categories(&[&GROUP]);
        let mut builder = RuleBuilder::default();
        let error = register_generated(&mut builder, &[&GROUP], &cats).unwrap_err();
        assert_eq!(
            error,
            GeneratedAssemblyError::BindWhileGenerated {
                construction: "bound"
            }
        );
    }
}
