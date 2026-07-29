use super::BareNominalAdjunct;
use super::InitialSound;
use super::Verb;
use super::VerbSlot;
use super::Vocab;
use super::Vocabulary;
use super::modifier::character_initial_sound;
use crate::catalog::CatalogAtom;
use crate::syntax::NumberLiteral;
use crate::syntax::OpaqueLexeme;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Noun {
    Word(Vocab),
    Catalog(CatalogAtom),
    Die(NumberLiteral),
    Gerund(Verb),
    Agentive(Verb),
    Opaque(OpaqueLexeme),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NounInstance {
    Singular(Noun),
    Plural(Noun),
    Mass(Noun),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NounDeclension {
    Regular,
    Irregular {
        singular: &'static str,
        plural: &'static str,
    },
    Invariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Countability {
    Count,
    Mass,
    CountOrMass,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NounDefinition {
    pub noun: Noun,
    pub declension: NounDeclension,
    pub countability: Countability,
}

impl Noun {
    pub(crate) fn bare_nominal_adjunct(&self) -> Option<BareNominalAdjunct> {
        match self {
            Self::Word(vocab) => vocab.bare_nominal_adjunct(),
            Self::Catalog(_)
            | Self::Die(_)
            | Self::Gerund(_)
            | Self::Agentive(_)
            | Self::Opaque(_) => None,
        }
    }
}

impl Vocabulary {
    #[must_use]
    pub fn noun_definition(self, vocab: Vocab) -> Option<NounDefinition> {
        let definition = vocab.definition();
        definition
            .noun
            .map(|(declension, countability)| NounDefinition {
                noun: Noun::Word(vocab),
                declension,
                countability,
            })
    }

    pub(crate) fn exceptional_catalog_noun(canonical: &str) -> Option<Vocab> {
        Vocab::ALL.iter().copied().find(|vocab| {
            let definition = vocab.definition();
            definition.catalog_noun && definition.spelling.eq_ignore_ascii_case(canonical)
        })
    }

    #[must_use]
    pub fn render_noun(self, noun: &NounInstance) -> Option<String> {
        let (noun, form) = match noun {
            NounInstance::Singular(noun) => (noun, NounSurface::Singular),
            NounInstance::Plural(noun) => (noun, NounSurface::Plural),
            NounInstance::Mass(noun) => (noun, NounSurface::Mass),
        };

        match noun {
            Noun::Word(vocab) => Self::render_vocab_noun(*vocab, form),
            Noun::Catalog(atom) => match form {
                NounSurface::Singular | NounSurface::Mass => Some(atom.render_noun(false)),
                NounSurface::Plural => Some(atom.render_noun(true)),
            },
            Noun::Die(number) => match form {
                NounSurface::Singular => Some(format!("d{}", number.numeral.format(number.value))),
                NounSurface::Plural | NounSurface::Mass => None,
            },
            Noun::Gerund(verb) => {
                let present_participle =
                    self.render_verb_identity(verb, VerbSlot::PresentParticiple)?;
                Some(match form {
                    NounSurface::Plural => regular_plural(&present_participle),
                    NounSurface::Singular | NounSurface::Mass => present_participle,
                })
            }
            Noun::Agentive(verb) => {
                let singular = self.render_agent_noun(verb)?;
                match form {
                    NounSurface::Singular => Some(singular),
                    NounSurface::Plural => Some(regular_plural(&singular)),
                    NounSurface::Mass => None,
                }
            }
            Noun::Opaque(opaque) => Some(opaque.spelling().to_owned()),
        }
    }

    pub(super) fn render_vocab_noun(vocab: Vocab, form: NounSurface) -> Option<String> {
        let definition = vocab.definition();
        let (declension, countability) = definition.noun?;
        if !countability.accepts(form) {
            return None;
        }
        Some(render_noun_form(definition.spelling, declension, form))
    }

    fn render_agent_noun(self, verb: &Verb) -> Option<String> {
        // Agent nouns are a lexical-vocabulary derivation, not a catalog
        // keyword-action derivation. Final `e` takes `-r` (`vote` → `voter`);
        // otherwise the present-participle stem supplies any declared
        // consonant doubling (`bid` → `bidding` → `bidder`).
        let Verb::Word(vocab) = verb else {
            return None;
        };
        let lemma = vocab.spelling();
        if lemma.ends_with('e') {
            return Some(format!("{lemma}r"));
        }

        let present_participle = self.render_verb(*vocab, VerbSlot::PresentParticiple)?;
        let stem = present_participle.strip_suffix("ing")?;
        Some(format!("{stem}er"))
    }
}

impl Countability {
    pub(super) const fn accepts(self, form: NounSurface) -> bool {
        match self {
            Self::Count => matches!(form, NounSurface::Singular | NounSurface::Plural),
            Self::Mass => matches!(form, NounSurface::Mass),
            Self::CountOrMass => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum NounSurface {
    Singular,
    Plural,
    Mass,
}

pub(super) fn render_noun_form(
    lemma: &str,
    declension: NounDeclension,
    form: NounSurface,
) -> String {
    match (declension, form) {
        (NounDeclension::Irregular { singular, .. }, NounSurface::Singular)
        | (NounDeclension::Irregular { singular, .. }, NounSurface::Mass) => singular.to_owned(),
        (NounDeclension::Irregular { plural, .. }, NounSurface::Plural) => plural.to_owned(),
        (NounDeclension::Invariant, _) | (NounDeclension::Regular, NounSurface::Mass) => {
            lemma.to_owned()
        }
        (NounDeclension::Regular, NounSurface::Singular) => lemma.to_owned(),
        (NounDeclension::Regular, NounSurface::Plural) => regular_plural(lemma),
    }
}

pub(crate) fn regular_plural(word: &str) -> String {
    if let Some(stem) = consonant_y_stem(word) {
        format!("{stem}ies")
    } else if has_sibilant_ending(word) {
        format!("{word}es")
    } else {
        format!("{word}s")
    }
}

pub(super) fn consonant_y_stem(word: &str) -> Option<&str> {
    let stem = word.strip_suffix('y')?;
    stem.chars()
        .next_back()
        .is_some_and(|character| character_initial_sound(character) == InitialSound::Consonant)
        .then_some(stem)
}

pub(super) fn has_sibilant_ending(word: &str) -> bool {
    ["s", "x", "z", "ch", "sh"]
        .iter()
        .any(|ending| word.ends_with(ending))
}
