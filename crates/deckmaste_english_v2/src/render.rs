use deckmaste_catalogs::CatalogKind;

use crate::ast::Ability;
use crate::ast::Amount;
use crate::ast::Article;
use crate::ast::CatalogIdentity;
use crate::ast::Clause;
use crate::ast::Common;
use crate::ast::Connive;
use crate::ast::CountNp;
use crate::ast::DealDamage;
use crate::ast::Declarative;
use crate::ast::Demonstrative;
use crate::ast::DemonstrativeNp;
use crate::ast::Destroy;
use crate::ast::EventClause;
use crate::ast::GainLife;
use crate::ast::Imperative;
use crate::ast::Noun;
use crate::ast::NounLexeme;
use crate::ast::NounPhrase;
use crate::ast::NumberAmount;
use crate::ast::Pronoun;
use crate::ast::PronounNp;
use crate::ast::SelfReferenceNp;
use crate::ast::SelfReferenceSpelling;
use crate::ast::Sentence;
use crate::ast::Sign;
use crate::ast::SignedNumber;
use crate::ast::Spell;
use crate::ast::TargetNp;
use crate::ast::TriggerWord;
use crate::ast::Triggered;
use crate::ast::Variable;
use crate::ast::VariableAmount;
use crate::ast::VerbLexeme;
use crate::ast::VerbPhrase;
use crate::ast::WhereClause;
use crate::ast::WithWhere;
use crate::context::ParseContext;

pub trait Render {
    fn render(&self, context: &ParseContext<'_>) -> String;
}

#[derive(Clone, Copy)]
enum Agreement {
    Bare,
    ThirdPersonSingular,
}

#[derive(Clone, Copy)]
enum Number {
    Singular,
    Plural,
}

#[derive(Clone, Copy)]
enum CatalogCasing {
    Lowercase,
    Preserve,
}

trait CatalogKindCasing {
    fn casing(self) -> CatalogCasing;
}

struct Writer {
    output: String,
    capitalize_next: bool,
}

impl Writer {
    fn new() -> Self {
        Self {
            output: String::new(),
            capitalize_next: true,
        }
    }

    fn word(&mut self, word: &str) {
        if !self.output.is_empty() {
            self.output.push(' ');
        }
        if self.capitalize_next {
            let mut characters = word.chars();
            if let Some(first) = characters.next() {
                self.output.extend(first.to_uppercase());
                self.output.push_str(characters.as_str());
            }
            self.capitalize_next = false;
        } else {
            self.output.push_str(word);
        }
    }

    fn punctuation(&mut self, mark: char) {
        self.output.push(mark);
        self.capitalize_next = mark == '.';
    }

    fn finish(self) -> String {
        self.output
    }
}

impl Render for Ability {
    fn render(&self, context: &ParseContext<'_>) -> String {
        let mut writer = Writer::new();
        match self {
            Self::Spell(Spell { effect }) => render_sentence_body(&mut writer, effect, context),
            Self::Triggered(Triggered {
                trigger,
                event,
                effect,
            }) => {
                render_trigger_word(&mut writer, *trigger);
                render_clause(&mut writer, event, context);
                writer.punctuation(',');
                render_sentence_body(&mut writer, effect, context);
            }
        }
        writer.punctuation('.');
        writer.finish()
    }
}

impl Render for Sentence {
    fn render(&self, context: &ParseContext<'_>) -> String {
        let mut writer = Writer::new();
        render_sentence_body(&mut writer, self, context);
        writer.punctuation('.');
        writer.finish()
    }
}

fn render_sentence_body(writer: &mut Writer, sentence: &Sentence, context: &ParseContext<'_>) {
    match sentence {
        Sentence::Imperative(Imperative { predicate }) => {
            render_verb_phrase(writer, predicate, Agreement::Bare, context);
        }
        Sentence::Declarative(Declarative { subject, predicate }) => {
            render_noun_phrase(writer, subject, context);
            render_verb_phrase(
                writer,
                predicate,
                agreement_for_noun_phrase(subject),
                context,
            );
        }
        Sentence::WithWhere(WithWhere { body, clause }) => {
            render_sentence_body(writer, body, context);
            writer.punctuation(',');
            render_clause(writer, clause, context);
        }
    }
}

fn render_clause(writer: &mut Writer, clause: &Clause, context: &ParseContext<'_>) {
    match clause {
        Clause::Event(EventClause { subject, predicate }) => {
            render_noun_phrase(writer, subject, context);
            render_verb_phrase(
                writer,
                predicate,
                agreement_for_noun_phrase(subject),
                context,
            );
        }
        Clause::Where(WhereClause { variable, value }) => {
            writer.word("where");
            render_variable(writer, *variable);
            writer.word(inflect(VerbLexeme::Be, Agreement::ThirdPersonSingular));
            writer.word("the");
            writer.word("number");
            writer.word("of");
            render_noun_phrase(writer, value, context);
        }
    }
}

fn render_verb_phrase(
    writer: &mut Writer,
    phrase: &VerbPhrase,
    agreement: Agreement,
    context: &ParseContext<'_>,
) {
    match phrase {
        VerbPhrase::Destroy(Destroy { object }) => {
            writer.word(inflect(VerbLexeme::Destroy, agreement));
            render_noun_phrase(writer, object, context);
        }
        VerbPhrase::Connive(Connive) => writer.word(inflect(VerbLexeme::Connive, agreement)),
        VerbPhrase::DealDamage(DealDamage { amount, to }) => {
            writer.word(inflect(VerbLexeme::Deal, agreement));
            render_amount(writer, amount);
            writer.word("damage");
            writer.word("to");
            render_noun_phrase(writer, to, context);
        }
        VerbPhrase::GainLife(GainLife { amount }) => {
            writer.word(inflect(VerbLexeme::Gain, agreement));
            render_amount(writer, amount);
            writer.word("life");
        }
    }
}

fn render_noun_phrase(writer: &mut Writer, phrase: &NounPhrase, context: &ParseContext<'_>) {
    match phrase {
        NounPhrase::Pronoun(PronounNp { word }) => render_pronoun(writer, *word),
        NounPhrase::Common(Common { article, head }) => {
            render_article(writer, *article);
            render_noun(writer, head, number_for_noun_phrase(phrase));
        }
        NounPhrase::Demonstrative(DemonstrativeNp { word, head }) => {
            render_demonstrative(writer, *word);
            render_noun(writer, head, number_for_noun_phrase(phrase));
        }
        NounPhrase::Target(TargetNp { head }) => {
            writer.word("target");
            render_noun(writer, head, number_for_noun_phrase(phrase));
        }
        NounPhrase::SelfReference(SelfReferenceNp { spelling }) => match spelling {
            SelfReferenceSpelling::Full => writer.word(context.card_name()),
            SelfReferenceSpelling::Abbreviated => writer.word(context.abbreviated_card_name()),
        },
        NounPhrase::Count(CountNp {
            head,
            controller,
            threshold,
        }) => {
            render_noun(writer, head, number_for_noun_phrase(phrase));
            render_pronoun(writer, *controller);
            writer.word(inflect(
                VerbLexeme::Control,
                agreement_for_pronoun(*controller),
            ));
            writer.word("with");
            writer.word("power");
            render_signed_number(writer, threshold);
            writer.word("or");
            writer.word("less");
        }
    }
}

fn render_amount(writer: &mut Writer, amount: &Amount) {
    match amount {
        Amount::Number(NumberAmount { number }) => render_signed_number(writer, number),
        Amount::Variable(VariableAmount { variable }) => render_variable(writer, *variable),
    }
}

fn render_noun(writer: &mut Writer, noun: &Noun, number: Number) {
    let (singular, casing) = match noun {
        Noun::Lexeme(NounLexeme::Player) => ("player", CatalogCasing::Lowercase),
        Noun::Catalog(CatalogIdentity { kind, spelling }) => (spelling.as_str(), kind.casing()),
    };
    let cased = match casing {
        CatalogCasing::Lowercase => singular.to_lowercase(),
        CatalogCasing::Preserve => singular.to_owned(),
    };
    let rendered = match number {
        Number::Singular => cased,
        Number::Plural => pluralize(noun, &cased),
    };
    writer.word(&rendered);
}

fn pluralize(noun: &Noun, singular: &str) -> String {
    match noun {
        Noun::Lexeme(NounLexeme::Player)
        | Noun::Catalog(CatalogIdentity {
            kind: _,
            spelling: _,
        }) => format!("{singular}s"),
    }
}

fn render_trigger_word(writer: &mut Writer, trigger: TriggerWord) {
    match trigger {
        TriggerWord::Whenever => writer.word("whenever"),
    }
}

fn render_article(writer: &mut Writer, article: Article) {
    match article {
        Article::A => writer.word("a"),
        Article::An => writer.word("an"),
    }
}

fn render_demonstrative(writer: &mut Writer, demonstrative: Demonstrative) {
    match demonstrative {
        Demonstrative::That => writer.word("that"),
        Demonstrative::Those => writer.word("those"),
    }
}

fn render_pronoun(writer: &mut Writer, pronoun: Pronoun) {
    match pronoun {
        Pronoun::It => writer.word("it"),
        Pronoun::You => writer.word("you"),
    }
}

fn render_variable(writer: &mut Writer, variable: Variable) {
    match variable {
        Variable::X => writer.word("X"),
    }
}

fn render_signed_number(writer: &mut Writer, number: &SignedNumber) {
    let SignedNumber { sign, magnitude } = number;
    let magnitude = magnitude.to_string();
    match sign {
        Sign::Positive => writer.word(&magnitude),
        Sign::Negative => writer.word(&format!("-{magnitude}")),
    }
}

fn agreement_for_noun_phrase(phrase: &NounPhrase) -> Agreement {
    match phrase {
        NounPhrase::Pronoun(PronounNp { word }) => agreement_for_pronoun(*word),
        NounPhrase::Common(Common {
            article: Article::A | Article::An,
            head: _,
        })
        | NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::That,
            head: _,
        })
        | NounPhrase::Target(TargetNp { head: _ })
        | NounPhrase::SelfReference(SelfReferenceNp { spelling: _ })
        | NounPhrase::Count(CountNp {
            head: _,
            controller: _,
            threshold: _,
        }) => Agreement::ThirdPersonSingular,
        NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::Those,
            head: _,
        }) => Agreement::Bare,
    }
}

fn agreement_for_pronoun(pronoun: Pronoun) -> Agreement {
    match pronoun {
        Pronoun::It => Agreement::ThirdPersonSingular,
        Pronoun::You => Agreement::Bare,
    }
}

fn number_for_noun_phrase(phrase: &NounPhrase) -> Number {
    match phrase {
        NounPhrase::Pronoun(PronounNp {
            word: Pronoun::It | Pronoun::You,
        })
        | NounPhrase::Common(Common {
            article: Article::A | Article::An,
            head: _,
        })
        | NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::That,
            head: _,
        })
        | NounPhrase::Target(TargetNp { head: _ })
        | NounPhrase::SelfReference(SelfReferenceNp { spelling: _ }) => Number::Singular,
        NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::Those,
            head: _,
        })
        | NounPhrase::Count(CountNp {
            head: _,
            controller: _,
            threshold: _,
        }) => Number::Plural,
    }
}

fn inflect(lexeme: VerbLexeme, agreement: Agreement) -> &'static str {
    match (lexeme, agreement) {
        (VerbLexeme::Destroy, Agreement::Bare) => "destroy",
        (VerbLexeme::Destroy, Agreement::ThirdPersonSingular) => "destroys",
        (VerbLexeme::Connive, Agreement::Bare) => "connive",
        (VerbLexeme::Connive, Agreement::ThirdPersonSingular) => "connives",
        (VerbLexeme::Deal, Agreement::Bare) => "deal",
        (VerbLexeme::Deal, Agreement::ThirdPersonSingular) => "deals",
        (VerbLexeme::Gain, Agreement::Bare) => "gain",
        (VerbLexeme::Gain, Agreement::ThirdPersonSingular) => "gains",
        (VerbLexeme::Control, Agreement::Bare) => "control",
        (VerbLexeme::Control, Agreement::ThirdPersonSingular) => "controls",
        (VerbLexeme::Be, Agreement::Bare) => "are",
        (VerbLexeme::Be, Agreement::ThirdPersonSingular) => "is",
    }
}

impl CatalogKindCasing for CatalogKind {
    fn casing(self) -> CatalogCasing {
        match self {
            Self::CardTypes | Self::Supertypes => CatalogCasing::Lowercase,
            Self::AbilityWords
            | Self::ArtifactTypes
            | Self::BattleTypes
            | Self::CardNames
            | Self::CounterKindPhrases
            | Self::CreatureTypes
            | Self::EnchantmentTypes
            | Self::KeywordAbilities
            | Self::KeywordActions
            | Self::LandTypes
            | Self::PlaneswalkerTypes
            | Self::SpellTypes => CatalogCasing::Preserve,
        }
    }
}
