use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::OnceLock;

use super::Adjective;
use super::AuxiliaryInstance;
use super::ColorWord;
use super::Noun;
use super::NounInstance;
use super::Pronoun;
use super::PronounCase;
use super::PronounInstance;
use super::RegularVocab;
use super::Tense;
use super::Verb;
use super::VerbInstance;
use super::VerbSlot;
use super::Vocab;
use super::VocabDefinition;
use super::Vocabulary;
use super::closed_class::auxiliary_instances;
use super::noun::NounSurface;
use super::regular_vocabulary;
use super::verb::VERB_SLOTS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum NounUsage {
    Count,
    Mass,
    Either,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum LexicalSlot {
    Noun(NounUsage),
    Verb(VerbSlot),
    Adjective,
    Adverb,
    SentenceAdverbial,
    Pronoun(PronounCase),
    Auxiliary,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum WordMatch {
    Noun(NounInstance),
    Verb(VerbInstance),
    Adjective(Adjective),
    Adverb(Vocab),
    SentenceAdverbial(Vocab),
    Pronoun(PronounInstance),
    Auxiliary(AuxiliaryInstance),
}

impl Vocabulary {
    #[must_use]
    pub fn matches(self, surface: &str, slot: LexicalSlot) -> Vec<WordMatch> {
        let key = if surface.bytes().any(|byte| byte.is_ascii_uppercase()) {
            Cow::Owned(surface.to_ascii_lowercase())
        } else {
            Cow::Borrowed(surface)
        };
        reverse_index()
            .get(key.as_ref())
            .into_iter()
            .flatten()
            .filter_map(|candidate| candidate.for_slot(slot))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub(super) enum IndexedWord {
    Noun { vocab: Vocab, form: NounSurface },
    Agentive { vocab: Vocab, form: NounSurface },
    Verb { vocab: Vocab, slot: VerbSlot },
    Adjective(Vocab),
    Participle { vocab: Vocab, tense: Tense },
    Gerund(Vocab),
    Adverb(Vocab),
    SentenceAdverbial(Vocab),
    Color(ColorWord),
    Pronoun(PronounInstance),
    Auxiliary(AuxiliaryInstance),
}

impl IndexedWord {
    pub(super) fn for_slot(self, slot: LexicalSlot) -> Option<WordMatch> {
        match (self, slot) {
            (Self::Noun { vocab, form }, LexicalSlot::Noun(usage)) if usage.accepts(form) => {
                let noun = Noun::Word(vocab);
                Some(WordMatch::Noun(match form {
                    NounSurface::Singular => NounInstance::Singular(noun),
                    NounSurface::Plural => NounInstance::Plural(noun),
                    NounSurface::Mass => NounInstance::Mass(noun),
                }))
            }
            (Self::Agentive { vocab, form }, LexicalSlot::Noun(usage)) if usage.accepts(form) => {
                let noun = Noun::Agentive(Verb::Word(vocab));
                Some(WordMatch::Noun(match form {
                    NounSurface::Singular => NounInstance::Singular(noun),
                    NounSurface::Plural => NounInstance::Plural(noun),
                    NounSurface::Mass => return None,
                }))
            }
            (Self::Gerund(vocab), LexicalSlot::Noun(NounUsage::Mass | NounUsage::Either)) => Some(
                WordMatch::Noun(NounInstance::Mass(Noun::Gerund(Verb::Word(vocab)))),
            ),
            (
                Self::Verb {
                    vocab,
                    slot: candidate_slot,
                },
                LexicalSlot::Verb(requested_slot),
            ) if candidate_slot == requested_slot => Some(WordMatch::Verb(VerbInstance {
                verb: Verb::Word(vocab),
                slot: candidate_slot,
            })),
            (Self::Adjective(vocab), LexicalSlot::Adjective) => {
                Some(WordMatch::Adjective(Adjective::Word(vocab)))
            }
            (Self::Participle { vocab, tense }, LexicalSlot::Adjective) => Some(
                WordMatch::Adjective(Adjective::Participle(tense, Verb::Word(vocab))),
            ),
            (Self::Adverb(vocab), LexicalSlot::Adverb) => Some(WordMatch::Adverb(vocab)),
            (Self::SentenceAdverbial(vocab), LexicalSlot::SentenceAdverbial) => {
                Some(WordMatch::SentenceAdverbial(vocab))
            }
            (Self::Color(color), LexicalSlot::Adjective) => {
                Some(WordMatch::Adjective(Adjective::Color(color)))
            }
            (Self::Pronoun(pronoun), LexicalSlot::Pronoun(case)) if pronoun.case == case => {
                Some(WordMatch::Pronoun(pronoun))
            }
            (Self::Auxiliary(auxiliary), LexicalSlot::Auxiliary) => {
                Some(WordMatch::Auxiliary(auxiliary))
            }
            _ => None,
        }
    }
}

impl NounUsage {
    const fn accepts(self, form: NounSurface) -> bool {
        match self {
            Self::Count => matches!(form, NounSurface::Singular | NounSurface::Plural),
            Self::Mass => matches!(form, NounSurface::Mass),
            Self::Either => true,
        }
    }
}

pub(super) fn reverse_index() -> &'static HashMap<String, Vec<IndexedWord>> {
    static INDEX: OnceLock<HashMap<String, Vec<IndexedWord>>> = OnceLock::new();
    INDEX.get_or_init(build_reverse_index)
}

fn build_reverse_index() -> HashMap<String, Vec<IndexedWord>> {
    let vocabulary = Vocabulary::new();
    let mut index = HashMap::new();

    for &vocab in Vocab::ALL {
        let definition = vocab.definition();
        index_vocab(&mut index, vocabulary, vocab, definition);
    }

    for definition in regular_vocabulary() {
        if Vocab::ALL
            .iter()
            .copied()
            .any(|vocab| vocab.spelling() == definition.spelling)
        {
            continue;
        }
        let vocab = Vocab::Regular(RegularVocab(definition.spelling));
        index_vocab(&mut index, vocabulary, vocab, *definition);
    }

    for color in [
        ColorWord::White,
        ColorWord::Blue,
        ColorWord::Black,
        ColorWord::Red,
        ColorWord::Green,
    ] {
        insert_index(&mut index, color.spelling(), IndexedWord::Color(color));
    }

    for auxiliary in auxiliary_instances() {
        if let Some(surface) = vocabulary.render_auxiliary(auxiliary) {
            insert_index(&mut index, surface, IndexedWord::Auxiliary(auxiliary));
        }
    }

    for pronoun in Pronoun::ALL {
        for case in [PronounCase::Subject, PronounCase::Object] {
            let pronoun = PronounInstance { pronoun, case };
            if let Some(surface) = vocabulary.render_pronoun(pronoun) {
                insert_index(&mut index, surface, IndexedWord::Pronoun(pronoun));
            }
        }
    }

    index
}

fn index_vocab(
    index: &mut HashMap<String, Vec<IndexedWord>>,
    vocabulary: Vocabulary,
    vocab: Vocab,
    definition: VocabDefinition,
) {
    if let Some((_, countability)) = definition.noun
        && !definition.catalog_noun
    {
        for form in [
            NounSurface::Singular,
            NounSurface::Plural,
            NounSurface::Mass,
        ] {
            if countability.accepts(form) {
                let surface = Vocabulary::render_vocab_noun(vocab, form)
                    .expect("declared noun form must render");
                insert_index(index, &surface, IndexedWord::Noun { vocab, form });
            }
        }
    }

    if definition.verb.is_some() {
        for form in [NounSurface::Singular, NounSurface::Plural] {
            let noun = match form {
                NounSurface::Singular => NounInstance::Singular(Noun::Agentive(Verb::Word(vocab))),
                NounSurface::Plural => NounInstance::Plural(Noun::Agentive(Verb::Word(vocab))),
                NounSurface::Mass => unreachable!("agent nouns are count-only"),
            };
            let surface = vocabulary
                .render_noun(&noun)
                .expect("derived agent noun form must render");
            insert_index(index, &surface, IndexedWord::Agentive { vocab, form });
        }

        for slot in VERB_SLOTS {
            let surface = vocabulary
                .render_verb(vocab, slot)
                .expect("declared verb form must render");
            insert_index(index, &surface, IndexedWord::Verb { vocab, slot });
        }

        let present_participle = vocabulary
            .render_verb(vocab, VerbSlot::PresentParticiple)
            .expect("declared verb present participle must render");
        insert_index(
            index,
            &present_participle,
            IndexedWord::Participle {
                vocab,
                tense: Tense::Present,
            },
        );
        insert_index(index, &present_participle, IndexedWord::Gerund(vocab));

        let past_participle = vocabulary
            .render_verb(vocab, VerbSlot::PastParticiple)
            .expect("declared verb past participle must render");
        insert_index(
            index,
            &past_participle,
            IndexedWord::Participle {
                vocab,
                tense: Tense::Past,
            },
        );
    }

    if definition.adjective
        && !index.get(definition.spelling).is_some_and(|candidates| {
            candidates
                .iter()
                .any(|candidate| matches!(candidate, IndexedWord::Participle { .. }))
        })
    {
        insert_index(index, definition.spelling, IndexedWord::Adjective(vocab));
    }
    if definition.adverb {
        insert_index(index, definition.spelling, IndexedWord::Adverb(vocab));
    }
    if definition.sentence_adverbial {
        insert_index(
            index,
            definition.spelling,
            IndexedWord::SentenceAdverbial(vocab),
        );
    }
}

fn insert_index(
    index: &mut HashMap<String, Vec<IndexedWord>>,
    surface: &str,
    candidate: IndexedWord,
) {
    let candidates = index.entry(surface.to_ascii_lowercase()).or_default();
    if !candidates.contains(&candidate) {
        candidates.push(candidate);
    }
}
