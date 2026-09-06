use crate::macro_def::{FrameItem, FrameRelation, VerbFrame};

/// An interned frame schema; its index has no Construction identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SchemaId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedFrameItem<C, L> {
    pub complement: Option<(FrameRelation, C)>,
    pub marker: Option<L>,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FrameSchemaError {
    UnknownCategory(String),
    UnknownLexicalReference(String, String),
    UnknownRole(String),
    NestedOptional,
    MissingFrames,
    MissingForms,
    InvalidVerbSurface(String),
    InvalidVerbFeature,
    DuplicateVerbFeature,
    DuplicateLexeme,
    UnownedLiteral(String),
}

/// Environment-owned schemas resolved against the generated grammar inventory.
#[derive(Debug, Clone)]
pub struct FrameRegistry<C, L> {
    schemas: Vec<Vec<ResolvedFrameItem<C, L>>>,
}

impl<C: Clone + Eq, L: Clone + Eq> FrameRegistry<C, L> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            schemas: Vec::new(),
        }
    }

    /// Resolves and interns a schema without changing the registry on failure.
    ///
    /// # Errors
    /// Returns the unresolved category, role or lexical reference, or malformed optional item.
    pub fn intern(
        &mut self,
        frame: &VerbFrame,
        category: impl Fn(&str) -> Option<C>,
        lexical: impl Fn(&str, &str) -> Option<L>,
        legacy_role: impl Fn(&str) -> Option<(FrameRelation, C)>,
    ) -> Result<SchemaId, FrameSchemaError> {
        let resolved = frame
            .iter()
            .map(|item| resolve(item, &category, &lexical, &legacy_role))
            .collect::<Result<Vec<_>, _>>()?;
        if let Some(index) = self
            .schemas
            .iter()
            .position(|existing| *existing == resolved)
        {
            return Ok(SchemaId(index));
        }
        let id = SchemaId(self.schemas.len());
        self.schemas.push(resolved);
        Ok(id)
    }

    #[must_use]
    pub fn schemas(&self) -> &[Vec<ResolvedFrameItem<C, L>>] {
        &self.schemas
    }
}

impl<C: Clone + Eq, L: Clone + Eq> Default for FrameRegistry<C, L> {
    fn default() -> Self {
        Self::new()
    }
}

fn resolve<C, L>(
    item: &FrameItem,
    category: &impl Fn(&str) -> Option<C>,
    lexical: &impl Fn(&str, &str) -> Option<L>,
    legacy_role: &impl Fn(&str) -> Option<(FrameRelation, C)>,
) -> Result<ResolvedFrameItem<C, L>, FrameSchemaError> {
    let argument = |name: &str| {
        category(name).ok_or_else(|| FrameSchemaError::UnknownCategory(name.to_owned()))
    };
    let marker = |vocabulary: &str, member: &str| {
        lexical(vocabulary, member).ok_or_else(|| {
            FrameSchemaError::UnknownLexicalReference(vocabulary.to_owned(), member.to_owned())
        })
    };
    let role = |name: &str| {
        legacy_role(name).ok_or_else(|| FrameSchemaError::UnknownRole(name.to_owned()))
    };
    let (complement, marker, optional) = match item {
        FrameItem::Argument(value) => (
            Some((value.relation, argument(&value.category)?)),
            None,
            false,
        ),
        FrameItem::Fixed(value) => (None, Some(marker(&value.vocabulary, &value.member)?), false),
        FrameItem::Marked(value, complement) => (
            Some((complement.relation, argument(&complement.category)?)),
            Some(marker(&value.vocabulary, &value.member)?),
            false,
        ),
        FrameItem::Optional(inner) => {
            let mut value = resolve(inner, category, lexical, legacy_role)?;
            if value.optional {
                return Err(FrameSchemaError::NestedOptional);
            }
            value.optional = true;
            return Ok(value);
        }
        FrameItem::Lex(v, m) => (None, Some(marker(v, m)?), false),
        FrameItem::OptionalLex(v, m) => (None, Some(marker(v, m)?), true),
        FrameItem::MarkedRole(v, m, r) => (Some(role(r)?), Some(marker(v, m)?), false),
        FrameItem::OptionalMarkedRole(v, m, r) => (Some(role(r)?), Some(marker(v, m)?), true),
        FrameItem::Role(r) => (Some(role(r)?), None, false),
        FrameItem::OptionalRole(r) => (Some(role(r)?), None, true),
        FrameItem::Amount => (Some(role("Amount")?), None, false),
        FrameItem::ObjectNounPhrase => (Some(role("ObjectNounPhrase")?), None, false),
        FrameItem::PredicativeComplement => (Some(role("PredicativeComplement")?), None, false),
        FrameItem::Literal(value) => return Err(FrameSchemaError::UnownedLiteral(value.clone())),
    };
    Ok(ResolvedFrameItem {
        complement,
        marker,
        optional,
    })
}

/// Checks the realized forms supplied to a prepared verb inventory.
///
/// # Errors
/// Rejects empty inventories, malformed surfaces, nonverbal features and repeated feature rows.
pub fn validate_verb_forms(
    forms: &[(crate::macro_def::SurfaceFeature, String)],
) -> Result<(), FrameSchemaError> {
    if forms.is_empty() {
        return Err(FrameSchemaError::MissingForms);
    }
    let mut seen = std::collections::HashSet::new();
    for (feature, surface) in forms {
        if feature.inflectional_form().is_none() {
            return Err(FrameSchemaError::InvalidVerbFeature);
        }
        if !seen.insert(*feature) {
            return Err(FrameSchemaError::DuplicateVerbFeature);
        }
        if !crate::macro_def::valid_surface(surface) {
            return Err(FrameSchemaError::InvalidVerbSurface(surface.clone()));
        }
    }
    Ok(())
}
