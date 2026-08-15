use crate::ast::Ability;
use crate::ast::Amount;
use crate::ast::Article;
use crate::ast::CatalogIdentity;
use crate::ast::Clause;
use crate::ast::Common;
use crate::ast::Comparative;
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
use crate::ast::SelfName;
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

pub trait Visitor {
    fn visit_ability(&mut self, ability: &Ability) {
        walk_ability(self, ability);
    }

    fn visit_sentence(&mut self, sentence: &Sentence) {
        walk_sentence(self, sentence);
    }

    fn visit_clause(&mut self, clause: &Clause) {
        walk_clause(self, clause);
    }

    fn visit_noun_phrase(&mut self, noun_phrase: &NounPhrase) {
        walk_noun_phrase(self, noun_phrase);
    }

    fn visit_verb_phrase(&mut self, verb_phrase: &VerbPhrase) {
        walk_verb_phrase(self, verb_phrase);
    }

    fn visit_amount(&mut self, amount: &Amount) {
        walk_amount(self, amount);
    }

    fn visit_noun(&mut self, noun: &Noun) {
        walk_noun(self, noun);
    }

    fn visit_spell(&mut self, spell: &Spell) {
        walk_spell(self, spell);
    }

    fn visit_triggered(&mut self, triggered: &Triggered) {
        walk_triggered(self, triggered);
    }

    fn visit_imperative(&mut self, imperative: &Imperative) {
        walk_imperative(self, imperative);
    }

    fn visit_declarative(&mut self, declarative: &Declarative) {
        walk_declarative(self, declarative);
    }

    fn visit_with_where(&mut self, with_where: &WithWhere) {
        walk_with_where(self, with_where);
    }

    fn visit_event_clause(&mut self, event_clause: &EventClause) {
        walk_event_clause(self, event_clause);
    }

    fn visit_where_clause(&mut self, where_clause: &WhereClause) {
        walk_where_clause(self, where_clause);
    }

    fn visit_pronoun_np(&mut self, pronoun_np: &PronounNp) {
        walk_pronoun_np(self, pronoun_np);
    }

    fn visit_common(&mut self, common: &Common) {
        walk_common(self, common);
    }

    fn visit_demonstrative_np(&mut self, demonstrative_np: &DemonstrativeNp) {
        walk_demonstrative_np(self, demonstrative_np);
    }

    fn visit_target_np(&mut self, target_np: &TargetNp) {
        walk_target_np(self, target_np);
    }

    fn visit_self_reference_np(&mut self, self_reference_np: &SelfReferenceNp) {
        walk_self_reference_np(self, self_reference_np);
    }

    fn visit_count_np(&mut self, count_np: &CountNp) {
        walk_count_np(self, count_np);
    }

    fn visit_destroy(&mut self, destroy: &Destroy) {
        walk_destroy(self, destroy);
    }

    fn visit_connive(&mut self, connive: &Connive) {
        walk_connive(self, connive);
    }

    fn visit_deal_damage(&mut self, deal_damage: &DealDamage) {
        walk_deal_damage(self, deal_damage);
    }

    fn visit_gain_life(&mut self, gain_life: &GainLife) {
        walk_gain_life(self, gain_life);
    }

    fn visit_number_amount(&mut self, number_amount: &NumberAmount) {
        walk_number_amount(self, number_amount);
    }

    fn visit_variable_amount(&mut self, variable_amount: &VariableAmount) {
        walk_variable_amount(self, variable_amount);
    }

    fn visit_trigger_word(&mut self, _word: TriggerWord) {}

    fn visit_article(&mut self, _article: Article) {}

    fn visit_demonstrative(&mut self, _demonstrative: Demonstrative) {}

    fn visit_pronoun(&mut self, _pronoun: Pronoun) {}

    fn visit_variable(&mut self, _variable: Variable) {}

    fn visit_sign(&mut self, _sign: Sign) {}

    fn visit_self_reference_spelling(&mut self, _spelling: SelfReferenceSpelling) {}

    fn visit_noun_lexeme(&mut self, _noun: NounLexeme) {}

    fn visit_verb_lexeme(&mut self, _verb: VerbLexeme) {}

    fn visit_comparative(&mut self, _comparative: Comparative) {}

    fn visit_signed_number(&mut self, _number: &SignedNumber) {}

    fn visit_catalog_identity(&mut self, _identity: &CatalogIdentity) {}

    fn visit_catalog_spelling(&mut self, _spelling: &str) {}

    fn visit_self_name(&mut self, _name: &SelfName) {}
}

pub fn walk_ability<V: Visitor + ?Sized>(visitor: &mut V, ability: &Ability) {
    match ability {
        Ability::Spell(spell) => visitor.visit_spell(spell),
        Ability::Triggered(triggered) => visitor.visit_triggered(triggered),
    }
}

pub fn walk_sentence<V: Visitor + ?Sized>(visitor: &mut V, sentence: &Sentence) {
    match sentence {
        Sentence::Imperative(imperative) => visitor.visit_imperative(imperative),
        Sentence::Declarative(declarative) => visitor.visit_declarative(declarative),
        Sentence::WithWhere(with_where) => visitor.visit_with_where(with_where),
    }
}

pub fn walk_clause<V: Visitor + ?Sized>(visitor: &mut V, clause: &Clause) {
    match clause {
        Clause::Event(event_clause) => visitor.visit_event_clause(event_clause),
        Clause::Where(where_clause) => visitor.visit_where_clause(where_clause),
    }
}

pub fn walk_noun_phrase<V: Visitor + ?Sized>(visitor: &mut V, noun_phrase: &NounPhrase) {
    match noun_phrase {
        NounPhrase::Pronoun(pronoun_np) => visitor.visit_pronoun_np(pronoun_np),
        NounPhrase::Common(common) => visitor.visit_common(common),
        NounPhrase::Demonstrative(demonstrative_np) => {
            visitor.visit_demonstrative_np(demonstrative_np);
        }
        NounPhrase::Target(target_np) => visitor.visit_target_np(target_np),
        NounPhrase::SelfReference(self_reference_np) => {
            visitor.visit_self_reference_np(self_reference_np);
        }
        NounPhrase::Count(count_np) => visitor.visit_count_np(count_np),
    }
}

pub fn walk_verb_phrase<V: Visitor + ?Sized>(visitor: &mut V, verb_phrase: &VerbPhrase) {
    match verb_phrase {
        VerbPhrase::Destroy(destroy) => visitor.visit_destroy(destroy),
        VerbPhrase::Connive(connive) => visitor.visit_connive(connive),
        VerbPhrase::DealDamage(deal_damage) => visitor.visit_deal_damage(deal_damage),
        VerbPhrase::GainLife(gain_life) => visitor.visit_gain_life(gain_life),
    }
}

pub fn walk_amount<V: Visitor + ?Sized>(visitor: &mut V, amount: &Amount) {
    match amount {
        Amount::Number(number_amount) => visitor.visit_number_amount(number_amount),
        Amount::Variable(variable_amount) => visitor.visit_variable_amount(variable_amount),
    }
}

pub fn walk_noun<V: Visitor + ?Sized>(visitor: &mut V, noun: &Noun) {
    match noun {
        Noun::Lexeme(noun_lexeme) => walk_noun_lexeme(visitor, *noun_lexeme),
        Noun::Catalog(catalog_identity) => walk_catalog_identity(visitor, catalog_identity),
    }
}

pub fn walk_spell<V: Visitor + ?Sized>(visitor: &mut V, spell: &Spell) {
    let Spell { effect } = spell;
    visitor.visit_sentence(effect);
}

pub fn walk_triggered<V: Visitor + ?Sized>(visitor: &mut V, triggered: &Triggered) {
    walk_trigger_word(visitor, triggered.trigger());
    visitor.visit_clause(triggered.event());
    visitor.visit_sentence(triggered.effect());
}

pub fn walk_imperative<V: Visitor + ?Sized>(visitor: &mut V, imperative: &Imperative) {
    let Imperative { predicate } = imperative;
    visitor.visit_verb_phrase(predicate);
}

pub fn walk_declarative<V: Visitor + ?Sized>(visitor: &mut V, declarative: &Declarative) {
    let Declarative { subject, predicate } = declarative;
    visitor.visit_noun_phrase(subject);
    visitor.visit_verb_phrase(predicate);
}

pub fn walk_with_where<V: Visitor + ?Sized>(visitor: &mut V, with_where: &WithWhere) {
    let WithWhere { body, clause } = with_where;
    visitor.visit_sentence(body);
    visitor.visit_clause(clause);
}

pub fn walk_event_clause<V: Visitor + ?Sized>(visitor: &mut V, event_clause: &EventClause) {
    let EventClause { subject, predicate } = event_clause;
    visitor.visit_noun_phrase(subject);
    visitor.visit_verb_phrase(predicate);
}

pub fn walk_where_clause<V: Visitor + ?Sized>(visitor: &mut V, where_clause: &WhereClause) {
    let WhereClause { variable, value } = where_clause;
    walk_variable(visitor, *variable);
    walk_verb_lexeme(visitor, VerbLexeme::Be);
    walk_noun_lexeme(visitor, NounLexeme::Number);
    visitor.visit_noun_phrase(value);
}

pub fn walk_pronoun_np<V: Visitor + ?Sized>(visitor: &mut V, pronoun_np: &PronounNp) {
    let PronounNp { word } = pronoun_np;
    walk_pronoun(visitor, *word);
}

pub fn walk_common<V: Visitor + ?Sized>(visitor: &mut V, common: &Common) {
    let Common { article, head } = common;
    walk_article(visitor, *article);
    visitor.visit_noun(head);
}

pub fn walk_demonstrative_np<V: Visitor + ?Sized>(
    visitor: &mut V,
    demonstrative_np: &DemonstrativeNp,
) {
    let DemonstrativeNp { word, head } = demonstrative_np;
    walk_demonstrative(visitor, *word);
    visitor.visit_noun(head);
}

pub fn walk_target_np<V: Visitor + ?Sized>(visitor: &mut V, target_np: &TargetNp) {
    let TargetNp { head } = target_np;
    visitor.visit_noun(head);
}

pub fn walk_self_reference_np<V: Visitor + ?Sized>(
    visitor: &mut V,
    self_reference_np: &SelfReferenceNp,
) {
    walk_self_name(visitor, self_reference_np.name());
    walk_self_reference_spelling(visitor, self_reference_np.spelling());
}

pub fn walk_count_np<V: Visitor + ?Sized>(visitor: &mut V, count_np: &CountNp) {
    let CountNp {
        head,
        controller,
        threshold,
    } = count_np;
    visitor.visit_noun(head);
    walk_pronoun(visitor, *controller);
    walk_verb_lexeme(visitor, VerbLexeme::Control);
    walk_noun_lexeme(visitor, NounLexeme::Power);
    walk_signed_number(visitor, threshold);
    walk_comparative(visitor, Comparative::OrLess);
}

pub fn walk_destroy<V: Visitor + ?Sized>(visitor: &mut V, destroy: &Destroy) {
    let Destroy { object } = destroy;
    walk_verb_lexeme(visitor, VerbLexeme::Destroy);
    visitor.visit_noun_phrase(object);
}

pub fn walk_connive<V: Visitor + ?Sized>(visitor: &mut V, connive: &Connive) {
    let Connive = connive;
    walk_verb_lexeme(visitor, VerbLexeme::Connive);
}

pub fn walk_deal_damage<V: Visitor + ?Sized>(visitor: &mut V, deal_damage: &DealDamage) {
    let DealDamage { amount, to } = deal_damage;
    walk_verb_lexeme(visitor, VerbLexeme::Deal);
    visitor.visit_amount(amount);
    walk_noun_lexeme(visitor, NounLexeme::Damage);
    visitor.visit_noun_phrase(to);
}

pub fn walk_gain_life<V: Visitor + ?Sized>(visitor: &mut V, gain_life: &GainLife) {
    let GainLife { amount } = gain_life;
    walk_verb_lexeme(visitor, VerbLexeme::Gain);
    visitor.visit_amount(amount);
    walk_noun_lexeme(visitor, NounLexeme::Life);
}

pub fn walk_number_amount<V: Visitor + ?Sized>(visitor: &mut V, number_amount: &NumberAmount) {
    let NumberAmount { number } = number_amount;
    walk_signed_number(visitor, number);
}

pub fn walk_variable_amount<V: Visitor + ?Sized>(
    visitor: &mut V,
    variable_amount: &VariableAmount,
) {
    let VariableAmount { variable } = variable_amount;
    walk_variable(visitor, *variable);
}

pub fn walk_trigger_word<V: Visitor + ?Sized>(visitor: &mut V, word: TriggerWord) {
    match word {
        TriggerWord::Whenever => visitor.visit_trigger_word(TriggerWord::Whenever),
    }
}

pub fn walk_article<V: Visitor + ?Sized>(visitor: &mut V, article: Article) {
    match article {
        Article::A => visitor.visit_article(Article::A),
        Article::An => visitor.visit_article(Article::An),
    }
}

pub fn walk_demonstrative<V: Visitor + ?Sized>(visitor: &mut V, demonstrative: Demonstrative) {
    match demonstrative {
        Demonstrative::That => visitor.visit_demonstrative(Demonstrative::That),
        Demonstrative::Those => visitor.visit_demonstrative(Demonstrative::Those),
    }
}

pub fn walk_pronoun<V: Visitor + ?Sized>(visitor: &mut V, pronoun: Pronoun) {
    match pronoun {
        Pronoun::It => visitor.visit_pronoun(Pronoun::It),
        Pronoun::You => visitor.visit_pronoun(Pronoun::You),
    }
}

pub fn walk_variable<V: Visitor + ?Sized>(visitor: &mut V, variable: Variable) {
    match variable {
        Variable::X => visitor.visit_variable(Variable::X),
    }
}

pub fn walk_sign<V: Visitor + ?Sized>(visitor: &mut V, sign: Sign) {
    match sign {
        Sign::Positive => visitor.visit_sign(Sign::Positive),
        Sign::Negative => visitor.visit_sign(Sign::Negative),
    }
}

pub fn walk_self_reference_spelling<V: Visitor + ?Sized>(
    visitor: &mut V,
    spelling: SelfReferenceSpelling,
) {
    match spelling {
        SelfReferenceSpelling::Full => {
            visitor.visit_self_reference_spelling(SelfReferenceSpelling::Full);
        }
        SelfReferenceSpelling::Abbreviated => {
            visitor.visit_self_reference_spelling(SelfReferenceSpelling::Abbreviated);
        }
    }
}

pub fn walk_noun_lexeme<V: Visitor + ?Sized>(visitor: &mut V, noun: NounLexeme) {
    match noun {
        NounLexeme::Player => visitor.visit_noun_lexeme(NounLexeme::Player),
        NounLexeme::Damage => visitor.visit_noun_lexeme(NounLexeme::Damage),
        NounLexeme::Life => visitor.visit_noun_lexeme(NounLexeme::Life),
        NounLexeme::Number => visitor.visit_noun_lexeme(NounLexeme::Number),
        NounLexeme::Power => visitor.visit_noun_lexeme(NounLexeme::Power),
    }
}

pub fn walk_verb_lexeme<V: Visitor + ?Sized>(visitor: &mut V, verb: VerbLexeme) {
    match verb {
        VerbLexeme::Destroy => visitor.visit_verb_lexeme(VerbLexeme::Destroy),
        VerbLexeme::Connive => visitor.visit_verb_lexeme(VerbLexeme::Connive),
        VerbLexeme::Deal => visitor.visit_verb_lexeme(VerbLexeme::Deal),
        VerbLexeme::Gain => visitor.visit_verb_lexeme(VerbLexeme::Gain),
        VerbLexeme::Control => visitor.visit_verb_lexeme(VerbLexeme::Control),
        VerbLexeme::Be => visitor.visit_verb_lexeme(VerbLexeme::Be),
    }
}

pub fn walk_comparative<V: Visitor + ?Sized>(visitor: &mut V, comparative: Comparative) {
    match comparative {
        Comparative::OrLess => visitor.visit_comparative(Comparative::OrLess),
    }
}

pub fn walk_signed_number<V: Visitor + ?Sized>(visitor: &mut V, number: &SignedNumber) {
    walk_sign(visitor, number.sign());
    visitor.visit_signed_number(number);
}

pub fn walk_catalog_identity<V: Visitor + ?Sized>(visitor: &mut V, identity: &CatalogIdentity) {
    visitor.visit_catalog_identity(identity);
    visitor.visit_catalog_spelling(identity.spelling());
}

pub fn walk_self_name<V: Visitor + ?Sized>(visitor: &mut V, name: &SelfName) {
    visitor.visit_self_name(name);
}
