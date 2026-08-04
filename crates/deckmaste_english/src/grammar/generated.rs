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
            let lhs = category_nonterminal(cats, construction, construction.category)?;
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
