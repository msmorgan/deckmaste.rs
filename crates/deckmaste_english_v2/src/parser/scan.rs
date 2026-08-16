use deckmaste_catalogs::CatalogKind;

use super::engine::ChartFailure;
use super::engine::Forest;
use super::engine::LexicalMatch;
use super::engine::parse;
use super::lexical::Lexical;
use super::lexical::NounNumber;
use super::materialize::completion_has_checked_build;
use crate::ast::Article;
use crate::ast::CatalogIdentity;
use crate::ast::Demonstrative;
use crate::ast::Noun;
use crate::ast::NounLexeme;
use crate::ast::Pronoun;
use crate::ast::SelfReferenceSpelling;
use crate::ast::Sign;
use crate::ast::SignedNumber;
use crate::ast::TriggerWord;
use crate::ast::Variable;
use crate::ast::VerbLexeme;
use crate::catalogs::ParserCatalogs;
use crate::constructions::Category;
use crate::constructions::RULES;
use crate::constructions::RuleId;
use crate::context::ParseContext;
use crate::features::Agreement;
use crate::features::inflect;

pub(crate) struct SliceGrammar<'a> {
    pub(crate) catalogs: &'a ParserCatalogs,
    pub(crate) context: &'a ParseContext<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Leaf {
    Literal(&'static str),
    EndOfInput,
    TriggerWord(TriggerWord),
    Article(Article),
    Demonstrative(Demonstrative),
    Pronoun(Pronoun),
    Variable(Variable),
    Noun {
        noun: Noun,
        number: NounNumber,
    },
    Verb {
        lexeme: VerbLexeme,
        agreement: Agreement,
    },
    SignedNumber(SignedNumber),
    SelfReference(SelfReferenceSpelling),
}

pub(crate) fn parse_forest(
    grammar: &SliceGrammar<'_>,
    text: &str,
) -> Result<Forest<RuleId, Leaf>, ChartFailure<Category, Lexical>> {
    parse(
        RULES,
        Category::Ability,
        text.len(),
        |lexical, offset| grammar.scan(lexical, text, offset),
        |rule, family, forest| completion_has_checked_build(rule, family, forest, grammar.context),
    )
}

impl SliceGrammar<'_> {
    fn scan(&self, lexical: Lexical, text: &str, offset: usize) -> Vec<LexicalMatch<Leaf>> {
        match lexical {
            Lexical::EndOfInput => (offset == text.len())
                .then_some(LexicalMatch {
                    end: offset,
                    value: Leaf::EndOfInput,
                })
                .into_iter()
                .collect(),
            Lexical::Literal(literal @ ("." | ",")) => text[offset..]
                .starts_with(literal)
                .then_some(LexicalMatch {
                    end: offset + literal.len(),
                    value: Leaf::Literal(literal),
                })
                .into_iter()
                .collect(),
            Lexical::Literal(literal) => Self::word(text, offset, literal)
                .map(|end| LexicalMatch {
                    end,
                    value: Leaf::Literal(literal),
                })
                .into_iter()
                .collect(),
            Lexical::TriggerWord => Self::closed_word(
                text,
                offset,
                "whenever",
                Leaf::TriggerWord(TriggerWord::Whenever),
            ),
            Lexical::Article => [
                ("a", Leaf::Article(Article::A)),
                ("an", Leaf::Article(Article::An)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                Self::word(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Demonstrative => [
                ("that", Leaf::Demonstrative(Demonstrative::That)),
                ("those", Leaf::Demonstrative(Demonstrative::Those)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                Self::word(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Pronoun => [
                ("it", Leaf::Pronoun(Pronoun::It)),
                ("you", Leaf::Pronoun(Pronoun::You)),
            ]
            .into_iter()
            .filter_map(|(word, value)| {
                Self::word(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
            Lexical::Variable => Self::closed_word(text, offset, "X", Leaf::Variable(Variable::X)),
            Lexical::Noun(number) => self.scan_noun(text, offset, number),
            Lexical::Verb(lexeme) => Self::scan_verb(text, offset, lexeme),
            Lexical::SignedNumber => Self::scan_signed_number(text, offset),
            Lexical::SelfReference => std::iter::once((
                self.context.card_name(),
                Leaf::SelfReference(SelfReferenceSpelling::Full),
            ))
            .chain(
                (self.context.abbreviated_card_name() != self.context.card_name()).then_some((
                    self.context.abbreviated_card_name(),
                    Leaf::SelfReference(SelfReferenceSpelling::Abbreviated),
                )),
            )
            .filter_map(|(word, value)| {
                Self::identity(text, offset, word).map(|end| LexicalMatch { end, value })
            })
            .collect(),
        }
    }

    fn closed_word(text: &str, offset: usize, word: &str, value: Leaf) -> Vec<LexicalMatch<Leaf>> {
        Self::word(text, offset, word)
            .map(|end| LexicalMatch { end, value })
            .into_iter()
            .collect()
    }

    fn word(text: &str, offset: usize, word: &str) -> Option<usize> {
        let prefix = usize::from(offset != 0);
        let remainder = text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let word = if offset == 0 { capitalize(word) } else { word.to_owned() };
        let end = offset + prefix + word.len();
        (remainder.starts_with(&word) && has_lexical_boundary(text, end)).then_some(end)
    }

    fn identity(text: &str, offset: usize, identity: &str) -> Option<usize> {
        let prefix = usize::from(offset != 0);
        let remainder = text.get(offset..)?;
        let remainder = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))?;
        let end = offset + prefix + identity.len();
        (!identity.is_empty()
            && end > offset
            && remainder.starts_with(identity)
            && has_lexical_boundary(text, end))
        .then_some(end)
    }

    fn scan_noun(&self, text: &str, offset: usize, wanted: NounNumber) -> Vec<LexicalMatch<Leaf>> {
        let mut matches = Vec::new();
        for (noun, singular) in std::iter::once((
            Noun::Lexeme(NounLexeme::Player),
            "player".to_owned(),
        ))
        .chain(
            self.catalogs
                .set()
                .get(CatalogKind::CardTypes)
                .iter()
                .filter_map(|spelling| {
                    CatalogIdentity::new(self.catalogs, CatalogKind::CardTypes, spelling.clone())
                        .map(|identity| {
                            (
                                Noun::Catalog(identity),
                                rendered_catalog(CatalogKind::CardTypes, spelling),
                            )
                        })
                }),
        ) {
            for (number, word) in noun_forms(&singular, wanted) {
                if let Some(end) = Self::word(text, offset, &word) {
                    matches.push(LexicalMatch {
                        end,
                        value: Leaf::Noun {
                            noun: noun.clone(),
                            number,
                        },
                    });
                }
            }
        }
        matches
    }

    fn scan_verb(text: &str, offset: usize, lexeme: VerbLexeme) -> Vec<LexicalMatch<Leaf>> {
        [Agreement::Bare, Agreement::ThirdPersonSingular]
            .into_iter()
            .filter_map(|agreement| {
                Self::word(text, offset, inflect(lexeme, agreement)).map(|end| LexicalMatch {
                    end,
                    value: Leaf::Verb { lexeme, agreement },
                })
            })
            .collect()
    }

    fn scan_signed_number(text: &str, offset: usize) -> Vec<LexicalMatch<Leaf>> {
        let prefix = usize::from(offset != 0);
        let Some(remainder) = text.get(offset..) else {
            return Vec::new();
        };
        let Some(number) = (prefix == 0)
            .then_some(remainder)
            .or_else(|| remainder.strip_prefix(' '))
        else {
            return Vec::new();
        };
        let (sign, digits) = number
            .strip_prefix('-')
            .map_or((Sign::Positive, number), |digits| (Sign::Negative, digits));
        let digit_length = digits.bytes().take_while(u8::is_ascii_digit).count();
        let digits = &digits[..digit_length];
        let Some(magnitude) = (!digits.is_empty())
            .then(|| digits.parse::<u32>().ok())
            .flatten()
        else {
            return Vec::new();
        };
        let end = offset + prefix + usize::from(sign == Sign::Negative) + digit_length;
        (magnitude.to_string() == digits && has_lexical_boundary(text, end))
            .then_some(LexicalMatch {
                end,
                value: Leaf::SignedNumber(SignedNumber { sign, magnitude }),
            })
            .into_iter()
            .collect()
    }
}

fn has_lexical_boundary(text: &str, end: usize) -> bool {
    matches!(text.as_bytes().get(end), None | Some(b' ' | b',' | b'.'))
}

fn capitalize(word: &str) -> String {
    let Some(first) = word.chars().next() else {
        return String::new();
    };
    first.to_uppercase().chain(word.chars().skip(1)).collect()
}

fn rendered_catalog(kind: CatalogKind, spelling: &str) -> String {
    match kind {
        CatalogKind::CardTypes | CatalogKind::Supertypes => spelling.to_lowercase(),
        CatalogKind::AbilityWords
        | CatalogKind::ArtifactTypes
        | CatalogKind::BattleTypes
        | CatalogKind::CardNames
        | CatalogKind::CounterKindPhrases
        | CatalogKind::CreatureTypes
        | CatalogKind::EnchantmentTypes
        | CatalogKind::KeywordAbilities
        | CatalogKind::KeywordActions
        | CatalogKind::LandTypes
        | CatalogKind::PlaneswalkerTypes
        | CatalogKind::SpellTypes => spelling.to_owned(),
    }
}

fn noun_forms(singular: &str, wanted: NounNumber) -> Vec<(NounNumber, String)> {
    match wanted {
        NounNumber::Singular => vec![(NounNumber::Singular, singular.to_owned())],
        NounNumber::Plural => vec![(NounNumber::Plural, format!("{singular}s"))],
        NounNumber::Either => vec![
            (NounNumber::Singular, singular.to_owned()),
            (NounNumber::Plural, format!("{singular}s")),
        ],
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Category;
    use super::ChartFailure;
    use super::Forest;
    use super::Leaf;
    use super::Lexical;
    use super::RuleId;
    use super::SliceGrammar;
    use super::parse_forest;
    use crate::catalogs::ParserCatalogs;
    use crate::context::ParseContext;

    fn slice_candidates(
        text: &str,
        card_name: &str,
    ) -> Result<Forest<RuleId, Leaf>, ChartFailure<Category, Lexical>> {
        let catalogs = ParserCatalogs::load(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/gen/catalogs"),
        )
        .expect("canonical generated catalogs load");
        let context = context(card_name);
        let grammar = SliceGrammar {
            catalogs: &catalogs,
            context: &context,
        };

        parse_forest(&grammar, text)
    }

    fn context(card_name: &str) -> ParseContext<'_> {
        ParseContext::new(card_name).expect("test card names are valid parse contexts")
    }
    #[test]
    fn scanner_accepts_multi_token_context_identity_and_catalog_nouns() {
        assert!(
            slice_candidates(
                "Zacama deals 3 damage to target creature.",
                "Zacama, Primal Calamity"
            )
            .is_ok()
        );
    }

    #[test]
    fn scanner_rejects_case_and_spacing_that_render_would_not_emit() {
        for text in [
            "destroy target creature.",
            "Destroy Target creature.",
            "Destroy  target creature.",
            "Destroytarget creature.",
            "Whenever a player connives, you Gain X life.",
        ] {
            assert!(
                slice_candidates(text, "Context Card").is_err(),
                "accepted {text:?}"
            );
        }
    }

    #[test]
    fn scanner_accepts_lowercase_you_after_the_trigger_comma() {
        assert!(
            slice_candidates(
                "Whenever a player connives, you gain X life.",
                "Context Card"
            )
            .is_ok()
        );
    }
    #[test]
    fn chart_completion_rejects_invalid_agreement_and_count_facts() {
        for text in [
            "You gains X life.",
            "Creatures you control with power 2 or less gains X life.",
        ] {
            assert!(
                slice_candidates(text, "Context Card").is_err(),
                "completed invalid chart family for {text:?}"
            );
        }
    }

    #[test]
    fn count_np_requires_you_and_bare_control() {
        for text in [
            "You gain X life, where X is the number of creatures it control with power 2 or less.",
            "You gain X life, where X is the number of creatures you controls with power 2 or less.",
        ] {
            assert!(
                slice_candidates(text, "Context Card").is_err(),
                "completed invalid count noun phrase for {text:?}"
            );
        }

        assert!(
            slice_candidates(
                "You gain X life, where X is the number of creatures you control with power 2 or less.",
                "Context Card"
            )
            .is_ok()
        );
    }

    #[test]
    fn chart_completion_rejects_a_construction_category_mismatch() {
        let text = "Whenever where X is the number of creatures you control with power 2 or less, you gain X life.";
        let Err(failure) = slice_candidates(text, "Context Card") else {
            panic!("a where clause cannot satisfy an event-clause construction role");
        };

        assert!(!failure.live.is_empty());
    }
}
