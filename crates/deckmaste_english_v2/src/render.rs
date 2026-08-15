use crate::ast::*;

pub trait Render {
    fn render(&self) -> String;
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
    fn render(&self) -> String {
        let mut writer = Writer::new();
        match self {
            Self::Spell(spell) => render_sentence_body(&mut writer, &spell.effect),
            Self::Triggered(triggered) => {
                render_trigger_word(&mut writer, triggered.trigger());
                render_clause(&mut writer, triggered.event());
                writer.punctuation(',');
                render_sentence_body(&mut writer, triggered.effect());
            }
        }
        writer.punctuation('.');
        writer.finish()
    }
}

impl Render for Sentence {
    fn render(&self) -> String {
        let mut writer = Writer::new();
        render_sentence_body(&mut writer, self);
        writer.punctuation('.');
        writer.finish()
    }
}

fn render_sentence_body(writer: &mut Writer, sentence: &Sentence) {
    match sentence {
        Sentence::Imperative(imperative) => {
            render_verb_phrase(writer, &imperative.predicate, Agreement::Bare)
        }
        Sentence::Declarative(declarative) => {
            render_noun_phrase(writer, &declarative.subject);
            render_verb_phrase(
                writer,
                &declarative.predicate,
                agreement_for_noun_phrase(&declarative.subject),
            );
        }
        Sentence::WithWhere(with_where) => {
            render_sentence_body(writer, &with_where.body);
            writer.punctuation(',');
            render_clause(writer, &with_where.clause);
        }
    }
}

fn render_clause(writer: &mut Writer, clause: &Clause) {
    match clause {
        Clause::Event(event) => {
            render_noun_phrase(writer, &event.subject);
            render_verb_phrase(
                writer,
                &event.predicate,
                agreement_for_noun_phrase(&event.subject),
            );
        }
        Clause::Where(where_clause) => {
            writer.word("where");
            render_variable(writer, where_clause.variable);
            writer.word(inflect(VerbLexeme::Be, Agreement::ThirdPersonSingular));
            writer.word("the");
            writer.word("number");
            writer.word("of");
            render_noun_phrase(writer, &where_clause.value);
        }
    }
}

fn render_verb_phrase(writer: &mut Writer, phrase: &VerbPhrase, agreement: Agreement) {
    match phrase {
        VerbPhrase::Destroy(destroy) => {
            writer.word(inflect(VerbLexeme::Destroy, agreement));
            render_noun_phrase(writer, &destroy.object);
        }
        VerbPhrase::Connive(Connive) => writer.word(inflect(VerbLexeme::Connive, agreement)),
        VerbPhrase::DealDamage(deal_damage) => {
            writer.word(inflect(VerbLexeme::Deal, agreement));
            render_amount(writer, &deal_damage.amount);
            writer.word("damage");
            writer.word("to");
            render_noun_phrase(writer, &deal_damage.to);
        }
        VerbPhrase::GainLife(gain_life) => {
            writer.word(inflect(VerbLexeme::Gain, agreement));
            render_amount(writer, &gain_life.amount);
            writer.word("life");
        }
    }
}

fn render_noun_phrase(writer: &mut Writer, phrase: &NounPhrase) {
    match phrase {
        NounPhrase::Pronoun(pronoun) => render_pronoun(writer, pronoun.word),
        NounPhrase::Common(common) => {
            render_article(writer, common.article);
            render_noun(writer, &common.head, number_for_noun_phrase(phrase));
        }
        NounPhrase::Demonstrative(demonstrative) => {
            render_demonstrative(writer, demonstrative.word);
            render_noun(writer, &demonstrative.head, number_for_noun_phrase(phrase));
        }
        NounPhrase::Target(target) => {
            writer.word("target");
            render_noun(writer, &target.head, number_for_noun_phrase(phrase));
        }
        NounPhrase::SelfReference(self_reference) => match self_reference.spelling() {
            SelfReferenceSpelling::Full => writer.word(self_reference.name().full()),
            SelfReferenceSpelling::Abbreviated => {
                writer.word(
                    self_reference
                        .name()
                        .abbreviated()
                        .expect("checked constructor"),
                );
            }
        },
        NounPhrase::Count(count) => {
            render_noun(writer, &count.head, number_for_noun_phrase(phrase));
            render_pronoun(writer, count.controller);
            writer.word(inflect(
                VerbLexeme::Control,
                agreement_for_pronoun(count.controller),
            ));
            writer.word("with");
            writer.word("power");
            render_signed_number(writer, &count.threshold);
            writer.word("or");
            writer.word("less");
        }
    }
}

fn render_amount(writer: &mut Writer, amount: &Amount) {
    match amount {
        Amount::Number(number) => render_signed_number(writer, &number.number),
        Amount::Variable(variable) => render_variable(writer, variable.variable),
    }
}

fn render_noun(writer: &mut Writer, noun: &Noun, number: Number) {
    let singular = match noun {
        Noun::Lexeme(lexeme) => match lexeme {
            NounLexeme::Player => "player",
            NounLexeme::Damage => "damage",
            NounLexeme::Life => "life",
            NounLexeme::Number => "number",
            NounLexeme::Power => "power",
        },
        Noun::Catalog(identity) => identity.spelling(),
    };
    let lower = singular.to_lowercase();
    let rendered = match number {
        Number::Singular => lower,
        Number::Plural => pluralize(noun, &lower),
    };
    writer.word(&rendered);
}

fn pluralize(noun: &Noun, singular: &str) -> String {
    match noun {
        Noun::Lexeme(NounLexeme::Life) => "lives".to_owned(),
        Noun::Lexeme(NounLexeme::Damage) => "damages".to_owned(),
        Noun::Lexeme(NounLexeme::Player)
        | Noun::Lexeme(NounLexeme::Number)
        | Noun::Lexeme(NounLexeme::Power)
        | Noun::Catalog(_) => format!("{singular}s"),
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
    let magnitude = number.magnitude().to_string();
    match number.sign() {
        Sign::Positive => writer.word(&magnitude),
        Sign::Negative => writer.word(&format!("-{magnitude}")),
    }
}

fn agreement_for_noun_phrase(phrase: &NounPhrase) -> Agreement {
    match phrase {
        NounPhrase::Pronoun(pronoun) => agreement_for_pronoun(pronoun.word),
        NounPhrase::Common(Common {
            article: Article::A | Article::An,
            ..
        })
        | NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::That,
            ..
        })
        | NounPhrase::Target(_)
        | NounPhrase::SelfReference(_)
        | NounPhrase::Count(_) => Agreement::ThirdPersonSingular,
        NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::Those,
            ..
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
            ..
        })
        | NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::That,
            ..
        })
        | NounPhrase::Target(_)
        | NounPhrase::SelfReference(_) => Number::Singular,
        NounPhrase::Demonstrative(DemonstrativeNp {
            word: Demonstrative::Those,
            ..
        })
        | NounPhrase::Count(_) => Number::Plural,
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
