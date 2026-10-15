//! The core English lexicon and the Catalogs of English v4, as one Lexical
//! Environment for `deckmaste_syntax`.
//!
//! Core vocabulary (`lexicon/english/`) is declared with Core Provenance and
//! is the only vocabulary a grammar may name. Catalog items (`catalogs/`) and
//! card names carry Catalog Provenance and reach a grammar only through their
//! Category and Subcategorization Frames.

mod build;
mod names;
pub mod schema;

use std::path::PathBuf;

use deckmaste_lexical::Binding;
use deckmaste_lexical::Category;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalError;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceStructure;
use deckmaste_syntax::FrameCatalog;
use serde::de::DeserializeOwned;

use crate::build::Builder;
use crate::build::bound;
use crate::build::card_scoped;
use crate::build::exact;
use crate::build::source;
pub use crate::names::shortened_names;
use crate::schema::AbilityWord;
use crate::schema::AdjectiveDecl;
use crate::schema::AdverbEntry;
use crate::schema::AnchorWord;
use crate::schema::ClosedClass;
use crate::schema::CounterKind;
use crate::schema::Designation;
use crate::schema::IntroducedName;
use crate::schema::KeywordAbility;
use crate::schema::KeywordAction;
use crate::schema::NounDecl;
use crate::schema::PrepositionEntry;
use crate::schema::StemClass;
use crate::schema::SubtypeFamily;
use crate::schema::Subtypes;
use crate::schema::Token;
use crate::schema::TurnPart;
use crate::schema::Types;
use crate::schema::VerbEntry;

/// Why the lexicon or a catalog cannot be loaded.
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error("{path}: {message}")]
    Format { path: String, message: String },
    #[error("{0}")]
    Invalid(String),
    #[error(transparent)]
    Lexical(#[from] LexicalError),
    #[error("{path}: {message}")]
    Io { path: String, message: String },
}

/// Every data file of this crate, by path, embedded at build time.
pub const FILES: [(&str, &str); 17] = [
    (
        CLOSED_CLASS,
        include_str!("../lexicon/english/closed_class.ron"),
    ),
    (
        PREPOSITIONS,
        include_str!("../lexicon/english/prepositions.ron"),
    ),
    (VERBS, include_str!("../lexicon/english/verbs.ron")),
    (NOUNS, include_str!("../lexicon/english/nouns.ron")),
    (
        ADJECTIVES,
        include_str!("../lexicon/english/adjectives.ron"),
    ),
    (ADVERBS, include_str!("../lexicon/english/adverbs.ron")),
    (TYPES, include_str!("../catalogs/types.ron")),
    (SUBTYPES, include_str!("../catalogs/subtypes.ron")),
    (
        KEYWORD_ACTIONS,
        include_str!("../catalogs/keyword_actions.ron"),
    ),
    (
        KEYWORD_ABILITIES,
        include_str!("../catalogs/keyword_abilities.ron"),
    ),
    (ABILITY_WORDS, include_str!("../catalogs/ability_words.ron")),
    (COUNTERS, include_str!("../catalogs/counters.ron")),
    (DESIGNATIONS, include_str!("../catalogs/designations.ron")),
    (TURN_PARTS, include_str!("../catalogs/turn_parts.ron")),
    (TOKENS, include_str!("../catalogs/tokens.ron")),
    (
        INTRODUCED_NAMES,
        include_str!("../catalogs/introduced_names.ron"),
    ),
    (ANCHOR_WORDS, include_str!("../catalogs/anchor_words.ron")),
];

pub(crate) const CLOSED_CLASS: &str = "lexicon/english/closed_class.ron";
const PREPOSITIONS: &str = "lexicon/english/prepositions.ron";
const VERBS: &str = "lexicon/english/verbs.ron";
const NOUNS: &str = "lexicon/english/nouns.ron";
const ADJECTIVES: &str = "lexicon/english/adjectives.ron";
const ADVERBS: &str = "lexicon/english/adverbs.ron";
const TYPES: &str = "catalogs/types.ron";
const SUBTYPES: &str = "catalogs/subtypes.ron";
const KEYWORD_ACTIONS: &str = "catalogs/keyword_actions.ron";
const KEYWORD_ABILITIES: &str = "catalogs/keyword_abilities.ron";
const ABILITY_WORDS: &str = "catalogs/ability_words.ron";
const COUNTERS: &str = "catalogs/counters.ron";
const DESIGNATIONS: &str = "catalogs/designations.ron";
const TURN_PARTS: &str = "catalogs/turn_parts.ron";
const TOKENS: &str = "catalogs/tokens.ron";
const INTRODUCED_NAMES: &str = "catalogs/introduced_names.ron";
const ANCHOR_WORDS: &str = "catalogs/anchor_words.ron";
/// The card-name catalog, relative to the repository root.
pub const CARD_NAMES: &str = "data/gen/catalogs/card-names.txt";

fn parse<T: DeserializeOwned>(path: &str) -> Result<T, SourceError> {
    let text = FILES
        .iter()
        .find(|(p, _)| *p == path)
        .map(|(_, text)| *text)
        .ok_or_else(|| SourceError::Invalid(format!("no data file {path}")))?;
    // Optional fields are written bare: `plural: "Elves"`, not `Some("Elves")`.
    ron::Options::default()
        .with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME)
        .from_str(text)
        .map_err(|error| SourceError::Format {
            path: path.to_owned(),
            message: error.to_string(),
        })
}

/// Every authored lexicon and catalog file, parsed.
#[derive(Debug, Clone)]
pub struct Sources {
    pub closed_class: ClosedClass,
    pub prepositions: Vec<PrepositionEntry>,
    pub verbs: Vec<VerbEntry>,
    pub nouns: Vec<NounDecl>,
    pub adjectives: Vec<AdjectiveDecl>,
    pub adverbs: Vec<AdverbEntry>,
    pub types: Types,
    pub subtypes: Subtypes,
    pub keyword_actions: Vec<KeywordAction>,
    pub keyword_abilities: Vec<KeywordAbility>,
    pub ability_words: Vec<AbilityWord>,
    pub counters: Vec<CounterKind>,
    pub designations: Vec<Designation>,
    pub turn_parts: Vec<TurnPart>,
    pub tokens: Vec<Token>,
    pub introduced_names: Vec<IntroducedName>,
    pub anchor_words: Vec<AnchorWord>,
}

/// A Lexicon and the Subcategorization Frames of its Lexemes.
#[derive(Debug)]
pub struct LexicalEnvironment {
    pub lexicon: Lexicon,
    pub frames: FrameCatalog,
}

impl LexicalEnvironment {
    /// Loads every data file and the card names at [`card_names_path`].
    ///
    /// # Errors
    /// An unreadable card-name file or any [`Sources::environment`] error.
    pub fn load() -> Result<Self, SourceError> {
        let path = card_names_path();
        let names = std::fs::read_to_string(&path).map_err(|error| SourceError::Io {
            path: path.display().to_string(),
            message: error.to_string(),
        })?;
        Sources::load()?.environment(&names)
    }
}

/// The card-name catalog in this repository.
#[must_use]
pub fn card_names_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(CARD_NAMES)
}

impl Sources {
    /// # Errors
    /// Malformed RON in any data file.
    pub fn load() -> Result<Self, SourceError> {
        Ok(Self {
            closed_class: parse(CLOSED_CLASS)?,
            prepositions: parse(PREPOSITIONS)?,
            verbs: parse(VERBS)?,
            nouns: parse(NOUNS)?,
            adjectives: parse(ADJECTIVES)?,
            adverbs: parse(ADVERBS)?,
            types: parse(TYPES)?,
            subtypes: parse(SUBTYPES)?,
            keyword_actions: parse(KEYWORD_ACTIONS)?,
            keyword_abilities: parse(KEYWORD_ABILITIES)?,
            ability_words: parse(ABILITY_WORDS)?,
            counters: parse(COUNTERS)?,
            designations: parse(DESIGNATIONS)?,
            turn_parts: parse(TURN_PARTS)?,
            tokens: parse(TOKENS)?,
            introduced_names: parse(INTRODUCED_NAMES)?,
            anchor_words: parse(ANCHOR_WORDS)?,
        })
    }

    /// The core lexicon and every catalog except card names.
    ///
    /// # Errors
    /// An invalid declaration: a duplicate identity, a frame naming an
    /// undeclared preposition, a verb without frames, a word declared both in
    /// the core lexicon and a catalog, or anything [`Lexicon::new`] rejects.
    pub fn vocabulary(&self) -> Result<(Vec<Lexeme>, FrameCatalog), SourceError> {
        let mut builder = Builder::default();
        self.core(&mut builder)?;
        self.catalogs(&mut builder)?;
        let (lexemes, frames) = builder.finish()?;
        check_core_and_catalogs_disjoint(&lexemes)?;
        Ok((lexemes, frames))
    }

    /// The whole Lexical Environment, with card names read from `card_names`
    /// (one per line).
    ///
    /// # Errors
    /// Any [`Sources::vocabulary`] error.
    pub fn environment(&self, card_names: &str) -> Result<LexicalEnvironment, SourceError> {
        let (mut lexemes, frames) = self.vocabulary()?;
        let vocabulary = Lexicon::new(lexemes.clone())?;
        let card_names = names::card_names(card_names, &vocabulary);
        self.check_card_scopes(&lexemes, &card_names)?;
        lexemes.extend(card_names);
        Ok(LexicalEnvironment {
            lexicon: Lexicon::new(lexemes)?,
            frames,
        })
    }

    fn core(&self, builder: &mut Builder) -> Result<(), SourceError> {
        let core = |path: &str, owner: &str| source(SourceKind::Core, path, owner);
        let closed = &self.closed_class;
        for entry in &closed.determinatives {
            let id = format!("det:{}", entry.lemma);
            let mut lexeme = Lexeme::invariant(
                &id,
                &entry.lemma,
                Category::Determinative,
                core(CLOSED_CLASS, &id),
            );
            lexeme.forms[0].surfaces.clone_from(&entry.spellings);
            lexeme.article_onsets.clone_from(&entry.article_onsets);
            builder.push(lexeme);
            builder.frames(id.as_str().into(), Category::Determinative, &entry.frames);
        }
        for entry in &closed.pronouns {
            let id = format!("pronoun:{}", entry.lemma);
            let mut lexeme = Lexeme::invariant(
                &id,
                &entry.lemma,
                Category::Pronoun,
                core(CLOSED_CLASS, &id),
            );
            lexeme.forms = entry
                .forms
                .iter()
                .map(|form| deckmaste_lexical::FormDeclaration {
                    form: form.form,
                    features: deckmaste_lexical::FeatureBundle {
                        person: Some(form.person),
                        number: Some(form.number),
                        case: form.case,
                        ..Default::default()
                    },
                    surfaces: Some(form.surfaces.to_vec()),
                })
                .collect();
            builder.push(lexeme);
        }
        for (prefix, category, words) in [
            ("coord", Category::Coordinator, &closed.coordinators),
            ("sub", Category::Subordinator, &closed.subordinators),
        ] {
            for word in words {
                let id = format!("{prefix}:{word}");
                builder.push(Lexeme::invariant(
                    &id,
                    word,
                    category,
                    core(CLOSED_CLASS, &id),
                ));
            }
        }
        for entry in &closed.auxiliaries {
            let id = format!("verb:{}", entry.lemma);
            builder.verb(&id, entry, core(CLOSED_CLASS, &id))?;
        }
        for (prefix, category, entries) in [
            ("affix", Category::Affix, &closed.affixes),
            ("clitic", Category::Clitic, &closed.clitics),
        ] {
            for entry in entries {
                let id = format!("{prefix}:{}", entry.name);
                let mut lexeme = bound(
                    &id,
                    &entry.spellings[0],
                    category,
                    &entry.spellings,
                    entry.binding,
                    core(CLOSED_CLASS, &id),
                );
                // A prefix opens a sentence as readily as its host: *Nonland*.
                if category == Category::Affix {
                    lexeme.capitalization = deckmaste_lexical::Capitalization::Initial;
                }
                builder.push(lexeme);
            }
        }
        for symbol in &closed.symbols {
            let id = format!("symbol:{symbol}");
            builder.push(exact(
                &id,
                symbol,
                Category::Symbol,
                source(SourceKind::Symbol, CLOSED_CLASS, &id),
            ));
        }
        for entry in &self.prepositions {
            let id = format!("prep:{}", entry.lemma);
            builder.preposition(&entry.lemma, &entry.frames, core(PREPOSITIONS, &id));
        }
        for entry in &self.verbs {
            let id = format!("verb:{}", entry.lemma);
            builder.verb(&id, entry, core(VERBS, &id))?;
        }
        for decl in &self.nouns {
            let entry = decl.entry();
            let id = format!("noun:{}", entry.lemma);
            builder.noun(&id, &entry, core(NOUNS, &id));
        }
        for decl in &self.adjectives {
            let entry = decl.entry();
            builder.adjective("adj", &entry, &core(ADJECTIVES, &entry.lemma));
        }
        for entry in &self.adverbs {
            let id = format!("adv:{}", entry.lemma);
            let mut lexeme =
                Lexeme::invariant(&id, &entry.lemma, Category::Adverb, core(ADVERBS, &id));
            lexeme.properties.classes = entry
                .positions
                .iter()
                .map(|position| format!("{position:?}Adverb"))
                .collect();
            builder.push(lexeme);
        }
        Ok(())
    }

    fn catalogs(&self, builder: &mut Builder) -> Result<(), SourceError> {
        let catalog = |path: &str, owner: &str| source(SourceKind::Catalog, path, owner);
        for decl in &self.types.card_types {
            let entry = decl.entry();
            let id = format!("type:{}", entry.lemma);
            builder.noun(&id, &entry, catalog(TYPES, &id));
        }
        for lemma in &self.types.supertypes {
            let id = format!("supertype:{lemma}");
            builder.push(Lexeme::invariant(
                &id,
                lemma,
                Category::Adjective,
                catalog(TYPES, &id),
            ));
        }
        let mut subtypes =
            std::collections::BTreeMap::<String, (NounDecl, Vec<SubtypeFamily>)>::new();
        for (family, decls) in &self.subtypes {
            for decl in decls {
                let lemma = decl.entry().lemma;
                let slot = subtypes
                    .entry(lemma)
                    .or_insert_with(|| (decl.clone(), Vec::new()));
                if slot.0 != *decl {
                    return Err(SourceError::Invalid(format!(
                        "subtype `{}` is declared differently in two families",
                        slot.0.entry().lemma
                    )));
                }
                slot.1.push(*family);
            }
        }
        for (lemma, (decl, families)) in &subtypes {
            let id = format!("subtype:{lemma}");
            builder.noun(&id, &decl.entry(), catalog(SUBTYPES, &id));
            let lexeme = builder.lexemes.last_mut().expect("just pushed");
            lexeme.properties.classes = families
                .iter()
                .map(|family| format!("{family:?}Type"))
                .collect();
        }
        self.keywords(builder)?;
        for word in &self.ability_words {
            let id = format!("ability_word:{}", word.lemma);
            let mut lexeme = Lexeme::invariant(
                &id,
                &word.lemma,
                Category::Keyword,
                catalog(ABILITY_WORDS, &id),
            );
            lexeme.surface_structure = SurfaceStructure::Opaque;
            lexeme
                .properties
                .features
                .insert("LabelKind".to_owned(), "AbilityWord".to_owned());
            builder.push(lexeme);
        }
        for counter in &self.counters {
            let id = format!("counter:{}", counter.kind);
            let entry = schema::NounEntry {
                lemma: format!("{} counter", counter.kind),
                countability: vec![deckmaste_lexical::Countability::Count],
                plural: None,
                frames: Vec::new(),
            };
            builder.noun(&id, &entry, catalog(COUNTERS, &id));
            if counter.kind.starts_with(['+', '-'])
                || counter.kind.starts_with(|c: char| c.is_ascii_digit())
            {
                builder
                    .lexemes
                    .last_mut()
                    .expect("just pushed")
                    .surface_structure = SurfaceStructure::MeasuredCompound;
            }
        }
        for designation in &self.designations {
            for decl in &designation.nouns {
                let entry = decl.entry();
                let id = format!("designation:{}", entry.lemma);
                builder.noun(
                    &id,
                    &entry,
                    catalog(DESIGNATIONS, designation.rule.number()),
                );
            }
            for lemma in &designation.adjectives {
                let id = format!("designation:{lemma}");
                builder.push(Lexeme::invariant(
                    &id,
                    lemma,
                    Category::Adjective,
                    catalog(DESIGNATIONS, designation.rule.number()),
                ));
            }
        }
        for part in &self.turn_parts {
            let entry = part.noun.entry();
            let id = format!("turn_part:{}", entry.lemma);
            builder.noun(&id, &entry, catalog(TURN_PARTS, part.rule.number()));
        }
        for token in &self.tokens {
            if !subtypes.contains_key(&token.subtype) {
                return Err(SourceError::Invalid(format!(
                    "token `{}` names `{}`, which is not a declared subtype",
                    token.name, token.subtype
                )));
            }
            for name in &token.names {
                let id = format!("token:{name}");
                builder.push(exact(
                    &id,
                    name,
                    Category::Catalog,
                    catalog(TOKENS, token.rule.number()),
                ));
            }
        }
        for introduced in &self.introduced_names {
            let id = format!("introduced_name:{}", introduced.name);
            let lexeme = exact(
                &id,
                &introduced.name,
                Category::Catalog,
                catalog(INTRODUCED_NAMES, &introduced.name),
            );
            builder.push(card_scoped(lexeme, &introduced.cards));
        }
        for anchor in &self.anchor_words {
            let id = format!("anchor_word:{}", anchor.word);
            let mut lexeme = exact(
                &id,
                &anchor.word,
                Category::Keyword,
                catalog(ANCHOR_WORDS, &anchor.word),
            );
            lexeme.surface_structure = SurfaceStructure::Opaque;
            lexeme
                .properties
                .features
                .insert("LabelKind".to_owned(), "AnchorWord".to_owned());
            builder.push(card_scoped(lexeme, &anchor.cards));
        }
        Ok(())
    }

    /// Every card a card scope names is a card, and no Introduced Name is
    /// one.
    fn check_card_scopes(
        &self,
        lexemes: &[Lexeme],
        card_names: &[Lexeme],
    ) -> Result<(), SourceError> {
        let cards: std::collections::BTreeSet<_> = card_names.iter().map(|card| card.id).collect();
        for lexeme in lexemes {
            if let Some(card) = lexeme.card_scope.iter().find(|card| !cards.contains(*card)) {
                return Err(SourceError::Invalid(format!(
                    "`{}` is scoped to `{card}`, which is not a card name",
                    lexeme.id
                )));
            }
        }
        for introduced in &self.introduced_names {
            if cards.contains(&names::card_name_id(&introduced.name)) {
                return Err(SourceError::Invalid(format!(
                    "introduced name `{}` is a card name, already in scope everywhere",
                    introduced.name
                )));
            }
        }
        Ok(())
    }

    /// Keyword actions and keyword abilities, with their variants, verbs,
    /// Bound Keyword Surfaces and stems.
    fn keywords(&self, builder: &mut Builder) -> Result<(), SourceError> {
        let catalog = |path: &str, owner: &str| source(SourceKind::Catalog, path, owner);
        for action in &self.keyword_actions {
            let owner = action.rule.number();
            for verb in &action.verbs {
                let id = format!("action:{}", verb.lemma);
                builder.verb(&id, verb, catalog(KEYWORD_ACTIONS, owner))?;
            }
            for decl in &action.nouns {
                let entry = decl.entry();
                let id = format!("action:{}", entry.lemma);
                builder.noun(&id, &entry, catalog(KEYWORD_ACTIONS, owner));
            }
        }
        let mut stems = std::collections::BTreeSet::new();
        for ability in &self.keyword_abilities {
            let owner = ability.rule.number();
            let lemmas = std::iter::once(&ability.lemma)
                .chain(ability.variants.iter().map(|variant| &variant.lemma));
            for lemma in lemmas {
                let id = format!("keyword:{lemma}");
                let mut lexeme = Lexeme::invariant(
                    &id,
                    lemma,
                    Category::Keyword,
                    catalog(KEYWORD_ABILITIES, owner),
                );
                lexeme.surface_structure = SurfaceStructure::Opaque;
                builder.push(lexeme);
            }
            for verb in &ability.verbs {
                let id = format!("keyword_verb:{}", verb.lemma);
                builder.verb(&id, verb, catalog(KEYWORD_ABILITIES, owner))?;
            }
            if let Some(surface) = &ability.bound {
                builder.push(bound(
                    &format!("keyword:{}:bound", ability.lemma),
                    &surface.surface,
                    Category::Keyword,
                    std::slice::from_ref(&surface.surface),
                    Binding::Suffix,
                    catalog(KEYWORD_ABILITIES, owner),
                ));
                for class in &surface.stems {
                    stems.extend(self.stems(*class));
                }
            }
        }
        for stem in stems {
            let id = format!("stem:{stem}");
            builder.push(bound(
                &id,
                &stem,
                Category::Noun,
                std::slice::from_ref(&stem),
                Binding::Prefix,
                catalog(KEYWORD_ABILITIES, &id),
            ));
        }
        Ok(())
    }

    /// The lowercase noun stems of a [`StemClass`].
    #[must_use]
    pub fn stems(&self, class: StemClass) -> Vec<String> {
        let families: &[SubtypeFamily] = match class {
            StemClass::LandType => &[SubtypeFamily::Land],
            StemClass::CreatureType => &[SubtypeFamily::Creature],
            StemClass::Subtype => &[
                SubtypeFamily::Artifact,
                SubtypeFamily::Battle,
                SubtypeFamily::Creature,
                SubtypeFamily::Dungeon,
                SubtypeFamily::Enchantment,
                SubtypeFamily::Land,
                SubtypeFamily::Planeswalker,
                SubtypeFamily::Spell,
            ],
            StemClass::CardType => {
                return self
                    .types
                    .card_types
                    .iter()
                    .map(|decl| decl.entry().lemma.to_lowercase())
                    .collect();
            }
            StemClass::Supertype => {
                return self
                    .types
                    .supertypes
                    .iter()
                    .map(|s| s.to_lowercase())
                    .collect();
            }
        };
        families
            .iter()
            .flat_map(|family| self.subtypes.get(family).into_iter().flatten())
            .map(|decl| decl.entry().lemma.to_lowercase())
            .collect()
    }
}

/// No word is both core vocabulary and a catalog item of the same Category.
fn check_core_and_catalogs_disjoint(lexemes: &[Lexeme]) -> Result<(), SourceError> {
    let mut core = std::collections::BTreeMap::new();
    for lexeme in lexemes.iter().filter(|l| l.source.kind == SourceKind::Core) {
        core.insert((lexeme.category, lexeme.lemma.to_lowercase()), lexeme.id);
    }
    for lexeme in lexemes
        .iter()
        .filter(|l| l.source.kind == SourceKind::Catalog)
    {
        if let Some(id) = core.get(&(lexeme.category, lexeme.lemma.to_lowercase())) {
            return Err(SourceError::Invalid(format!(
                "`{}` is a catalog item and must not also be core vocabulary `{id}`",
                lexeme.id
            )));
        }
    }
    Ok(())
}
