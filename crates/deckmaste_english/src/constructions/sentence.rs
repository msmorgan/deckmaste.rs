//! Generated declaration data and projections for the sentence wrapper.

use deckmaste_construction_compiler::runtime::AtomData;
use deckmaste_construction_compiler::runtime::ConstructionData;
use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::ErasedBuildError;
use deckmaste_construction_compiler::runtime::ErasedValue;
use deckmaste_construction_compiler::runtime::FieldData;
use deckmaste_construction_compiler::runtime::FieldKindData;
use deckmaste_construction_compiler::runtime::FormData;
use deckmaste_construction_compiler::runtime::GroupData;
use deckmaste_construction_compiler::runtime::LinearizationError;
use deckmaste_construction_compiler::runtime::LinearizationVisitor;
use deckmaste_construction_compiler::runtime::take_erased;

use crate::syntax::Sentence;
use crate::syntax::SentenceBody;

const PERIOD_FORM: u16 = 0;
const TERMINAL_FORM: u16 = 1;

fn erased_build_sentence(values: Vec<ErasedValue>) -> Result<ErasedValue, ErasedBuildError> {
    let mut values = values.into_iter();
    let body: SentenceBody = take_erased(&mut values, "sentence", "body", "SentenceBody")?;
    if values.next().is_some() {
        return Err(ErasedBuildError::ExtraFields { owner: "sentence" });
    }
    build_sentence(body)
        .map(|value| Box::new(value) as ErasedValue)
        .map_err(ErasedBuildError::Declaration)
}

/// The one runtime declaration consumed by registration, lowering, and
/// inspection. `Clause` is the chart category of the hole; its erased adapter
/// deliberately supplies the existing `SentenceBody::Independent` payload.
pub(crate) static SENTENCE_DECLARATION: GroupData = GroupData {
    name: "sentence",
    elements: &[],
    element_data: &[],
    constructions: &[ConstructionData {
        id: "sentence",
        category: "Sentence",
        internal: false,
        own_type: Some("Sentence"),
        bind_path: None,
        projection_variant: None,
        fields: &[FieldData {
            name: "body",
            kind: FieldKindData::Subtree {
                category: "Clause",
                boxed: false,
            },
        }],
        witnesses: &[],
        deserialize: false,
        selection_unique: true,
        dominates: &[],
        dominated_by: &[],
        forms: &[
            FormData {
                name: "period",
                ordinal: PERIOD_FORM,
                guarded: false,
                atoms: &[AtomData::Hole("body"), AtomData::Literal(".")],
            },
            FormData {
                name: "terminal",
                ordinal: TERMINAL_FORM,
                guarded: false,
                atoms: &[AtomData::Hole("body")],
            },
        ],
        requirements: &[],
        recognition_requirements: &[],
        feature_combinators: &[],
        erased_builder: Some(erased_build_sentence),
        erased_projector: None,
    }],
};

pub(crate) static GROUPS: &[&GroupData] = &[&SENTENCE_DECLARATION];

/// Checked construction door. Terminal punctuation is intentionally absent:
/// it is selected from sentence structure by linearization.
#[allow(
    clippy::unnecessary_wraps,
    reason = "construction builders share the declaration-generated fallible interface"
)]
pub(crate) fn build_sentence(body: SentenceBody) -> Result<Sentence, DeclarationViolation> {
    Ok(Sentence { body })
}

/// Full destructuring projection paired with [`build_sentence`].
pub(crate) const fn parts_sentence(value: &Sentence) -> &SentenceBody {
    &value.body
}

/// Linearizes through an explicitly selected declared form.
///
/// Context-sensitive renderers use this entry point because an enclosing
/// modal header may suppress punctuation even though the AST itself is an
/// ordinary sentence.
pub(crate) fn linearize_sentence_form_with<V>(
    value: &Sentence,
    ordinal: u16,
    visitor: &mut V,
) -> Result<(), LinearizationError<V::Error>>
where
    V: LinearizationVisitor,
{
    let construction = &SENTENCE_DECLARATION.constructions[0];
    let Some(form) = construction
        .forms
        .iter()
        .find(|form| form.ordinal == ordinal)
    else {
        return Err(LinearizationError::NoMatchingForm {
            construction: construction.id,
        });
    };
    visitor
        .begin_form(construction.id, form.name, form.ordinal)
        .map_err(LinearizationError::Visitor)?;
    for atom in form.atoms {
        match atom {
            AtomData::Hole("body") => visitor
                .subtree("Clause", parts_sentence(value))
                .map_err(LinearizationError::Visitor)?,
            AtomData::Literal(literal) => visitor
                .literal(literal)
                .map_err(LinearizationError::Visitor)?,
            AtomData::Hole(_) | AtomData::Lexeme(_) => {
                unreachable!("sentence declaration contains only its Clause hole")
            }
        }
    }
    visitor
        .end_form(construction.id)
        .map_err(LinearizationError::Visitor)
}

/// Total context-free linearization. A terminal quoted ability or recovered
/// surface owns its terminator; all other sentence structures take a period.
pub(crate) fn linearize_sentence_with<V>(
    value: &Sentence,
    visitor: &mut V,
) -> Result<(), LinearizationError<V::Error>>
where
    V: LinearizationVisitor,
{
    let ordinal = if crate::renderer::sentence_has_structural_terminator(value) {
        TERMINAL_FORM
    } else {
        PERIOD_FORM
    };
    linearize_sentence_form_with(value, ordinal, visitor)
}

pub(crate) const fn period_form_ordinal() -> u16 {
    PERIOD_FORM
}

pub(crate) const fn terminal_form_ordinal() -> u16 {
    TERMINAL_FORM
}
