//! Builds Lexemes and Subcategorization Frames from authored entries.

use std::collections::BTreeSet;

use deckmaste_lexical::Binding;
use deckmaste_lexical::Capitalization;
use deckmaste_lexical::Category;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Finiteness;
use deckmaste_lexical::FormDeclaration;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexemeId;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceStructure;
use deckmaste_lexical::WordForm;
use deckmaste_syntax::FrameCatalog;
use deckmaste_syntax::frames::ClauseKind;
use deckmaste_syntax::frames::Frame;
use deckmaste_syntax::frames::FrameFamily;
use deckmaste_syntax::frames::FrameSlot;
use deckmaste_syntax::frames::PrepositionFrame;
use deckmaste_syntax::frames::SlotFill;
use deckmaste_syntax::frames::SlotRelation;
use deckmaste_syntax::frames::SlotRelation::Complement;
use deckmaste_syntax::frames::SlotRelation::IndirectObject;
use deckmaste_syntax::frames::SlotRelation::Object;
use deckmaste_syntax::frames::SlotRelation::PredicativeComplement;
use dpsi::Ident;

use crate::SourceError;
use crate::schema::AdjectiveEntry;
use crate::schema::AdjectiveGrade;
use crate::schema::Catenation;
use crate::schema::DeterminativeEntry;
use crate::schema::FillSpec;
use crate::schema::FrameSpec;
use crate::schema::IdiomPart;
use crate::schema::NounEntry;
use crate::schema::PrepositionFrameSpec;
use crate::schema::PrepositionMarking;
use crate::schema::VerbEntry;

/// Lexemes and frames gathered from every data file, with frames still naming
/// prepositions by lemma until every preposition is known.
#[derive(Default)]
pub(crate) struct Builder {
    parameters:
        std::collections::BTreeMap<LexemeId, Vec<Vec<deckmaste_syntax::frames::ParameterPart>>>,
    pub(crate) lexemes: Vec<Lexeme>,
    frames: Vec<(LexemeId, Category, Vec<FrameSpec>, String)>,
    prepositions: Vec<(LexemeId, Vec<PrepositionFrameSpec>, String)>,
}

pub(crate) fn source(kind: SourceKind, path: &str, owner: &str) -> Source {
    Source {
        kind,
        path: path.to_owned(),
        owner: owner.to_owned(),
    }
}

/// Words separated by spaces are one multiword Lexeme.
pub(crate) fn structured(mut lexeme: Lexeme) -> Lexeme {
    if lexeme.surface_structure == SurfaceStructure::Word && lexeme.lemma.contains(' ') {
        lexeme.surface_structure = SurfaceStructure::Multiword;
    }
    lexeme
}

fn replace(
    lexeme: &mut Lexeme,
    form: WordForm,
    person: Option<Person>,
    number: Option<Number>,
    surfaces: &[String],
) -> bool {
    let mut found = false;
    for slot in &mut lexeme.forms {
        if slot.form == form
            && person.is_none_or(|p| slot.features.person == Some(p))
            && number.is_none_or(|n| slot.features.number == Some(n))
        {
            slot.surfaces = Some(surfaces.to_vec());
            found = true;
        }
    }
    found
}

impl Builder {
    pub(crate) fn push(&mut self, lexeme: Lexeme) {
        self.lexemes.push(structured(lexeme));
    }

    pub(crate) fn frames(&mut self, id: LexemeId, category: Category, frames: &[FrameSpec]) {
        if !frames.is_empty() {
            let owner = id.to_string();
            self.frames.push((id, category, frames.to_vec(), owner));
        }
    }

    pub(crate) fn determinative(&mut self, id: &str, entry: &DeterminativeEntry, source: Source) {
        let mut lexeme = Lexeme::invariant(id, &entry.lemma, Category::Determinative, source);
        lexeme.properties.features.insert(
            "DeterminativeDistribution".into(),
            Ident::new(&format!("{:?}", entry.distribution)),
        );
        lexeme.forms[0].surfaces.clone_from(&entry.spellings);
        lexeme.forms[0].features.number = entry.number;
        if let Some(countability) = entry.countability {
            lexeme.properties.features.insert(
                "Countability".into(),
                Ident::new(&format!("{countability:?}")),
            );
        }
        if let Some(kind) = entry.noun_kind {
            lexeme
                .properties
                .features
                .insert("NounKind".into(), Ident::new(&format!("{kind:?}")));
        }
        self.push(lexeme);
        self.frames(id.into(), Category::Determinative, &entry.frames);
    }

    pub(crate) fn verb(
        &mut self,
        id: &str,
        entry: &VerbEntry,
        source: Source,
    ) -> Result<(), SourceError> {
        let invalid = |reason: &str| SourceError::Invalid(format!("verb `{id}`: {reason}"));
        let auxiliary = source.kind == SourceKind::Core && source.path == crate::CLOSED_CLASS;
        if entry.frames.is_empty() && !auxiliary {
            return Err(invalid(
                "a verb declares at least one Subcategorization Frame",
            ));
        }
        if entry.binding.is_some() && !auxiliary {
            return Err(invalid("only a contracted auxiliary is bound"));
        }
        let mut lexeme = Lexeme::verb(id, &entry.lemma, source);
        // Oracle text has no first person (owner ruling): a paradigm has
        // second- and third-person cells only.
        lexeme
            .forms
            .retain(|slot| slot.features.person != Some(Person::First));
        if entry.frames.contains(&FrameSpec::Copular) {
            lexeme.properties.classes.insert("CopularVerb".into());
        }
        if let Some(binding) = entry.binding {
            lexeme.binding = binding;
            lexeme.capitalization = deckmaste_lexical::Capitalization::Exact;
        }
        if entry.finite_only {
            lexeme.forms.retain(|slot| {
                slot.form == WordForm::Present
                    || (slot.form == WordForm::Preterite
                        && (entry.preterite.is_some() || entry.past.is_some()))
            });
        }
        if entry.finite_only && (entry.past_participle.is_some() || entry.gerund.is_some()) {
            return Err(invalid("a finite-only verb has no participles"));
        }
        let past: &[WordForm] = if entry.finite_only {
            &[WordForm::Preterite]
        } else {
            &[WordForm::Preterite, WordForm::PastParticiple]
        };
        let third = (Some(Person::Third), Some(Number::Singular));
        let fields = [
            (&[WordForm::Present][..], &entry.third_singular, third),
            (past, &entry.past, (None, None)),
            (&[WordForm::Preterite][..], &entry.preterite, (None, None)),
            (
                &[WordForm::PastParticiple][..],
                &entry.past_participle,
                (None, None),
            ),
            (
                &[WordForm::GerundParticiple][..],
                &entry.gerund,
                (None, None),
            ),
        ];
        for (forms, spellings, (person, number)) in fields {
            let Some(spellings) = spellings else { continue };
            for form in forms {
                if !replace(&mut lexeme, *form, person, number, &spellings.to_vec()) {
                    return Err(invalid(&format!("no {form:?} slot to spell")));
                }
            }
        }
        if let Some(irrealis) = &entry.irrealis {
            lexeme.forms.push(FormDeclaration {
                form: WordForm::Irrealis,
                features: FeatureBundle {
                    number: Some(Number::Singular),
                    person: Some(Person::Third),
                    finiteness: Some(Finiteness::Finite),
                    ..FeatureBundle::default()
                },
                surfaces: Some(irrealis.to_vec()),
            });
        }
        for slot in &entry.forms {
            if !replace(
                &mut lexeme,
                slot.form,
                slot.person,
                slot.number,
                &slot.surfaces.to_vec(),
            ) {
                return Err(invalid(&format!("no {:?} slot to spell", slot.form)));
            }
        }
        lexeme.merge_number_neutral_forms();
        if auxiliary {
            lexeme
                .properties
                .features
                .insert("VerbKind".into(), "Auxiliary".into());
        }
        if let Some(base) = &entry.form_of {
            if !auxiliary {
                return Err(invalid("only an auxiliary is a form of another verb"));
            }
            // Spell every form from this entry's own lemma before naming it.
            let spelled = lexeme.clone();
            for slot in &mut lexeme.forms {
                if slot.surfaces.is_none() {
                    slot.surfaces = Some(vec![deckmaste_lexical::default_surface(
                        &spelled,
                        slot.form,
                        &slot.features,
                    )]);
                }
            }
            lexeme.lemma.clone_from(base);
        }
        if entry.negative && !auxiliary {
            return Err(invalid("only an auxiliary has a negative form"));
        }
        if auxiliary {
            lexeme.properties.features.insert(
                "Polarity".into(),
                if entry.negative { "Negative" } else { "Affirmative" }.into(),
            );
        }
        let id = lexeme.id;
        self.push(lexeme);
        self.frames(id, Category::Verb, &entry.frames);
        Ok(())
    }

    pub(crate) fn noun(&mut self, id: &str, entry: &NounEntry, source: Source) {
        let mut lexeme = Lexeme::noun(id, &entry.lemma, entry.countability.clone(), source);
        lexeme.properties.features.insert(
            "NounDistribution".into(),
            Ident::new(&format!("{:?}", entry.distribution)),
        );
        lexeme
            .properties
            .features
            .insert("NounKind".into(), Ident::new(&format!("{:?}", entry.kind)));
        if let Some(plural) = &entry.plural {
            replace(&mut lexeme, WordForm::Plural, None, None, &plural.to_vec());
        }
        let id = lexeme.id;
        self.push(lexeme);
        self.frames(id, Category::Noun, &entry.frames);
        if !entry.parameters.is_empty() {
            self.parameters.insert(id, entry.parameters.clone());
        }
    }

    pub(crate) fn adjective(&mut self, prefix: &str, entry: &AdjectiveEntry, source: &Source) {
        let own = match entry.grade {
            None => "Plain",
            Some(AdjectiveGrade::Comparative) => "Comparative",
            Some(AdjectiveGrade::Superlative) => "Superlative",
            Some(AdjectiveGrade::Equative) => "Equative",
        };
        let grades = [
            (Some(&entry.lemma), Some(own)),
            (entry.comparative.as_ref(), Some("Comparative")),
            (entry.superlative.as_ref(), Some("Superlative")),
        ];
        for (spelling, grade) in grades {
            let Some(spelling) = spelling else { continue };
            // A grade is an inflection of its adjective: it shares the lemma.
            let mut lexeme = Lexeme::invariant(
                format!("{prefix}:{spelling}"),
                &entry.lemma,
                Category::Adjective,
                source.clone(),
            );
            if spelling != &entry.lemma {
                replace(
                    &mut lexeme,
                    WordForm::Invariant,
                    None,
                    None,
                    std::slice::from_ref(spelling),
                );
            }
            if let Some(position) = entry.position {
                lexeme.properties.features.insert(
                    "AdjectivePosition".into(),
                    Ident::new(&format!("{position:?}")),
                );
            }
            if let Some(determination) = entry.determination {
                lexeme.properties.features.insert(
                    "Determination".into(),
                    Ident::new(&format!("{determination:?}")),
                );
            }
            if entry.fused_head {
                lexeme
                    .properties
                    .classes
                    .insert("FusedHeadAdjective".into());
            }
            if let Some(grade) = grade {
                lexeme
                    .properties
                    .features
                    .insert("Grade".into(), grade.into());
            }
            let id = lexeme.id;
            self.push(lexeme);
            match grade {
                Some(g) if g == own => self.frames(id, Category::Adjective, &entry.frames),
                // A comparative selects its *than* Complement (CGEL, Ch. 13
                // §2.1, p. 1104).
                Some("Comparative") => self.frames(
                    id,
                    Category::Adjective,
                    &[FrameSpec::Complement(FillSpec::PP(vec!["than".into()]))],
                ),
                _ => {}
            }
        }
    }

    pub(crate) fn preposition(
        &mut self,
        lemma: &str,
        frames: &[PrepositionFrameSpec],
        marking: Option<PrepositionMarking>,
        source: Source,
    ) {
        let mut lexeme = Lexeme::invariant(
            format!("prep:{lemma}"),
            lemma,
            Category::Preposition,
            source,
        );
        if let Some(marking) = marking {
            lexeme.properties.features.insert(
                "PrepositionMarking".into(),
                Ident::new(&format!("{marking:?}")),
            );
        }
        let id = lexeme.id;
        self.push(lexeme);
        self.prepositions
            .push((id, frames.to_vec(), id.to_string()));
    }

    /// Validates and indexes everything gathered, resolving preposition lemmas.
    pub(crate) fn finish(self) -> Result<(Vec<Lexeme>, FrameCatalog), SourceError> {
        let mut ids = BTreeSet::new();
        for lexeme in &self.lexemes {
            if !ids.insert(lexeme.id) {
                return Err(SourceError::Invalid(format!(
                    "`{}` is declared twice",
                    lexeme.id
                )));
            }
        }
        let resolver = Resolver { ids: &ids };
        let mut catalog = FrameCatalog {
            parameters: self.parameters,
            ..FrameCatalog::default()
        };
        for (id, category, specs, owner) in &self.frames {
            let mut frames = Vec::new();
            for spec in specs {
                let frame = resolver.frame(spec, *category, owner)?;
                if frames.contains(&frame) {
                    return Err(SourceError::Invalid(format!(
                        "`{owner}` declares {spec:?} twice"
                    )));
                }
                frames.push(frame);
            }
            catalog.frames.insert(*id, frames);
        }
        for (id, specs, owner) in &self.prepositions {
            let mut frames = Vec::new();
            for spec in specs {
                let frame = PrepositionFrame {
                    relation: spec.relation,
                    complement: resolver.fill(&spec.complement, owner)?,
                    hosts: spec.hosts,
                };
                if frames
                    .iter()
                    .any(|f: &PrepositionFrame| f.complement == frame.complement)
                {
                    return Err(SourceError::Invalid(format!(
                        "`{owner}` declares a {:?} complement twice",
                        spec.complement
                    )));
                }
                frames.push(frame);
            }
            catalog.prepositions.insert(*id, frames);
        }
        Ok((self.lexemes, catalog))
    }
}

/// Resolves authored frames against the declared Lexemes.
struct Resolver<'a> {
    ids: &'a BTreeSet<LexemeId>,
}

fn slot(relation: SlotRelation, fill: SlotFill) -> FrameSlot {
    FrameSlot { relation, fill }
}

fn catenation(catenation: Catenation) -> SlotFill {
    match catenation {
        Catenation::ToInf => SlotFill::ToInf,
        Catenation::BareInf => SlotFill::BareInf,
        Catenation::Gerund => SlotFill::Gerund,
    }
}

impl Resolver<'_> {
    fn preposition(&self, lemma: &str, owner: &str) -> Result<LexemeId, SourceError> {
        let id = LexemeId::new(&format!("prep:{lemma}"));
        if self.ids.contains(&id) {
            Ok(id)
        } else {
            Err(SourceError::Invalid(format!(
                "`{owner}` names `{lemma}`, which is not a declared preposition"
            )))
        }
    }

    fn pp(&self, lemmas: &[String], owner: &str) -> Result<SlotFill, SourceError> {
        Ok(SlotFill::PP(
            lemmas
                .iter()
                .map(|lemma| self.preposition(lemma, owner))
                .collect::<Result<_, _>>()?,
        ))
    }

    fn fill(&self, spec: &FillSpec, owner: &str) -> Result<SlotFill, SourceError> {
        Ok(match spec {
            FillSpec::NP => SlotFill::NP,
            FillSpec::PP(lemmas) => self.pp(lemmas, owner)?,
            FillSpec::Pred => SlotFill::Pred,
            FillSpec::AdjP => SlotFill::AdjP,
            FillSpec::AdvP => SlotFill::AdvP,
            FillSpec::LocativeNP => SlotFill::LocativeNP,
            FillSpec::MeasureNP => SlotFill::MeasureNP,
            FillSpec::FiniteClause => SlotFill::Clause(ClauseKind::Finite),
            FillSpec::VerblessClause => SlotFill::Clause(ClauseKind::Verbless),
            FillSpec::ToInf => SlotFill::ToInf,
            FillSpec::HollowToInf => SlotFill::HollowToInf,
            FillSpec::BareInf => SlotFill::BareInf,
            FillSpec::Gerund => SlotFill::Gerund,
            FillSpec::KeywordPhrase => SlotFill::KeywordPhrase,
            FillSpec::QuotedAbility => SlotFill::QuotedAbility,
            FillSpec::Nominal => SlotFill::Nominal,
            FillSpec::Amount => SlotFill::Amount,
            FillSpec::Cost => SlotFill::Cost,
            FillSpec::Intransitive => SlotFill::Intransitive,
        })
    }

    /// The engine Frame an authored frame of a `category` head declares.
    fn frame(
        &self,
        spec: &FrameSpec,
        category: Category,
        owner: &str,
    ) -> Result<Frame, SourceError> {
        let (family, slots) = match spec {
            FrameSpec::Intransitive => (FrameFamily::Intransitive, vec![]),
            FrameSpec::Monotransitive => (
                FrameFamily::Monotransitive,
                vec![slot(Object, SlotFill::NP)],
            ),
            FrameSpec::Transitive(fill) => (
                FrameFamily::Monotransitive,
                vec![slot(Object, self.fill(fill, owner)?)],
            ),
            FrameSpec::Ditransitive => (
                FrameFamily::Ditransitive,
                vec![
                    slot(IndirectObject, SlotFill::NP),
                    slot(Object, SlotFill::NP),
                ],
            ),
            FrameSpec::TransitiveWithPP(lemmas) => (
                FrameFamily::TransitiveWithPP,
                vec![
                    slot(Object, SlotFill::NP),
                    slot(Complement, self.pp(lemmas, owner)?),
                ],
            ),
            FrameSpec::Prepositional(lemmas) => (
                FrameFamily::Prepositional,
                vec![slot(Complement, self.pp(lemmas, owner)?)],
            ),
            FrameSpec::Copular => (
                FrameFamily::Copular,
                vec![slot(PredicativeComplement, SlotFill::Pred)],
            ),
            FrameSpec::Catenative(kind) => (
                FrameFamily::Catenative,
                vec![slot(Complement, catenation(*kind))],
            ),
            FrameSpec::ComplexTransitive => (
                FrameFamily::ComplexTransitive,
                vec![
                    slot(Object, SlotFill::NP),
                    slot(PredicativeComplement, SlotFill::Pred),
                ],
            ),
            FrameSpec::ComplexCatenative(kind) => (
                FrameFamily::ComplexCatenative,
                vec![
                    slot(Object, SlotFill::NP),
                    slot(Complement, catenation(*kind)),
                ],
            ),
            FrameSpec::ContentClause => (
                FrameFamily::ContentClause,
                vec![slot(
                    Complement,
                    SlotFill::Clause(ClauseKind::Interrogative),
                )],
            ),
            FrameSpec::Amount => (
                FrameFamily::Amount,
                vec![slot(Complement, SlotFill::Amount)],
            ),
            FrameSpec::TransitiveWithComplement(fill) => (
                FrameFamily::TransitiveWithComplement,
                vec![
                    slot(Object, SlotFill::NP),
                    slot(Complement, self.fill(fill, owner)?),
                ],
            ),
            FrameSpec::TransitiveAmount => (
                FrameFamily::TransitiveAmount,
                vec![
                    slot(Object, SlotFill::NP),
                    slot(Complement, SlotFill::Amount),
                ],
            ),
            FrameSpec::Cost => (FrameFamily::Cost, vec![slot(Complement, SlotFill::Cost)]),
            FrameSpec::Idiom(parts) => (
                FrameFamily::Idiom,
                parts
                    .iter()
                    .map(|part| {
                        Ok(match part {
                            IdiomPart::Prep(lemma) => {
                                slot(Complement, self.pp(std::slice::from_ref(lemma), owner)?)
                            }
                            IdiomPart::Pred => slot(PredicativeComplement, SlotFill::Pred),
                        })
                    })
                    .collect::<Result<_, SourceError>>()?,
            ),
            FrameSpec::Complement(fill) => {
                let family = match category {
                    Category::Noun => FrameFamily::NounComplement,
                    Category::Adjective => FrameFamily::AdjectiveComplement,
                    Category::Determinative => FrameFamily::DeterminativeComplement,
                    _ => {
                        return Err(SourceError::Invalid(format!(
                            "`{owner}`: only a noun, adjective or determinative declares a bare Complement frame"
                        )));
                    }
                };
                (family, vec![slot(Complement, self.fill(fill, owner)?)])
            }
        };
        let verb_family = !matches!(spec, FrameSpec::Complement(_));
        if verb_family != (category == Category::Verb) {
            return Err(SourceError::Invalid(format!(
                "`{owner}`: {spec:?} is not a frame of a {category:?}"
            )));
        }
        let frame = Frame { family, slots };
        if !frame.is_well_formed() {
            return Err(SourceError::Invalid(format!(
                "`{owner}`: {spec:?} does not have the shape of its family"
            )));
        }
        Ok(frame)
    }
}

/// A Lexeme of a closed-class or catalog category, spelled exactly.
pub(crate) fn exact(id: &str, lemma: &str, category: Category, source: Source) -> Lexeme {
    let mut lexeme = Lexeme::invariant(id, lemma, category, source);
    lexeme.capitalization = Capitalization::Exact;
    lexeme
}

/// A Lexeme whose spellings are in scope only on `cards`.
pub(crate) fn card_scoped(mut lexeme: Lexeme, cards: &[String]) -> Lexeme {
    lexeme.card_scoped_surfaces = [lexeme.lemma.clone()].into();
    lexeme.card_scope = cards
        .iter()
        .map(|card| crate::names::card_name_id(card))
        .collect();
    lexeme
}

/// A bound Lexeme with its Spelling Variants.
pub(crate) fn bound(
    id: &str,
    lemma: &str,
    category: Category,
    spellings: &[String],
    binding: Binding,
    source: Source,
) -> Lexeme {
    let mut lexeme = exact(id, lemma, category, source);
    lexeme.forms[0].surfaces = Some(spellings.to_vec());
    lexeme.binding = binding;
    lexeme
}
