use RulePosition::Lexical as L;
use RulePosition::Nonterminal as N;
use deckmaste_construction::constructions;

use crate::context::ParseContext;
use crate::parser::LexicalMatch;
use crate::parser::Rule;
use crate::parser::RulePosition;
use crate::parser::ScanInput;
use crate::parser::ownership::RawRenderedClaim;
use crate::render::Render;
use crate::render::Writer;

constructions! {
    vocab TriggerWord { Whenever = "whenever", }
    vocab Article { A = "a", An = "an", }
    vocab Demonstrative { That = "that", Those = "those", }
    vocab Pronoun { It = "it", You = "you", }
    vocab Variable { X = "X", }

    morphology EnglishVerb {
        feature = Agreement;
        recipe = english_verb;
    }
    morphology EnglishNoun {
        feature = Number;
        recipe = english_noun;
    }
    lexeme NounLexeme using EnglishNoun {
        Player = "player",
    }
    lexeme VerbLexeme using EnglishVerb {
        Deal = "deal",
        Gain = "gain",
        Control = "control",
        Be = "be" {
            Bare = "are",
            ThirdPersonSingular = "is",
        },
    }

    codec Noun {
        generate declaration_noun {
            closed = NounLexeme;
            position = Noun;
            kinds = [Type, Subtype];
            feature = Number;
        }
    }
    identity SelfReferenceSpelling {
        generate context {
            Full => card_name,
            Abbreviated => abbreviated_card_name,
            canonical_on_collision = Full;
        }
    }
    codec SignedNumber {
        generate signed_decimal {
            magnitude = u32;
            sign_type = Sign {
                Positive = none,
                Negative = "-",
            };
        }
    }
    construction spell: Ability {
        element Spell { effect: Sentence, }
        form spell = effect;
    }
    construction triggered: Ability {
        element Triggered {
            trigger: lex TriggerWord,
            event: Clause,
            effect: Sentence,
        }
        require event is Event;
        form triggered = lex(trigger) event "," effect;
    }
    construction imperative: Sentence {
        element Imperative { predicate: VerbPhrase, }
        derive predicate.agreement = Values::Bare;
        form imperative = predicate;
    }
    construction declarative: Sentence {
        element Declarative { subject: NounPhrase, predicate: VerbPhrase, }
        derive predicate.agreement = subject.agreement;
        form declarative = subject predicate;
    }
    construction with_where: Sentence {
        element WithWhere { body: Sentence, clause: Clause, }
        require clause is Where;
        form with_where = body "," clause;
    }
    construction event: Clause {
        element EventClause { subject: NounPhrase, predicate: VerbPhrase, }
        derive predicate.agreement = subject.agreement;
        form event = subject predicate;
    }
    construction where: Clause {
        element WhereClause { variable: lex Variable, value: NounPhrase, }
        derive verb.agreement = Values::ThirdPersonSingular;
        form where = "where" lex(variable) verb(VerbLexeme::Be)
            "the" "number" "of" value;
    }
    construction pronoun: NounPhrase {
        element PronounNp { word: lex Pronoun, }
        derive word.agreement = match word {
            It => Values::ThirdPersonSingular,
            You => Values::Bare,
        };
        derive agreement = word.agreement;
        derive number = Values::Singular;
        form pronoun = lex(word);
    }
    construction common: NounPhrase {
        element Common { article: lex Article, head: lex Noun, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form common = lex(article) noun(head);
    }
    construction demonstrative: NounPhrase {
        element DemonstrativeNp { word: lex Demonstrative, head: lex Noun, }
        derive agreement = match word {
            That => Values::ThirdPersonSingular,
            Those => Values::Bare,
        };
        derive number = match word {
            That => Values::Singular,
            Those => Values::Plural,
        };
        form demonstrative = lex(word) noun(head);
    }
    construction target: NounPhrase {
        element TargetNp { head: lex Noun, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form target = "target" noun(head);
    }
    construction self_reference: NounPhrase {
        element SelfReferenceNp { spelling: identity SelfReferenceSpelling, }
        derive agreement = Values::ThirdPersonSingular;
        derive number = Values::Singular;
        form self_reference = identity(spelling);
    }
    construction count: NounPhrase {
        element CountNp {
            head: lex Noun,
            controller: lex Pronoun,
            threshold: lex SignedNumber,
        }
        require controller is You;
        derive agreement = Values::Bare;
        derive number = Values::Plural;
        derive controller.agreement = match controller {
            It => Values::ThirdPersonSingular,
            You => Values::Bare,
        };
        derive verb.agreement = controller.agreement;
        form count = noun(head) lex(controller) verb(VerbLexeme::Control)
            "with" "power" lex(threshold) "or" "less";
    }
    construction destroy: VerbPhrase {
        element Destroy { object: NounPhrase, }
        derive agreement = verb.agreement;
        form destroy = open_verb(KeywordAction, "Destroy") object;
    }
    construction connive: VerbPhrase {
        element Connive {}
        derive agreement = verb.agreement;
        form connive = open_verb(KeywordAction, "Connive");
    }
    construction deal_damage: VerbPhrase {
        element DealDamage { amount: Amount, to: NounPhrase, }
        derive agreement = verb.agreement;
        form deal_damage = verb(VerbLexeme::Deal) amount "damage" "to" to;
    }
    construction gain_life: VerbPhrase {
        element GainLife { amount: Amount, }
        derive agreement = verb.agreement;
        form gain_life = verb(VerbLexeme::Gain) amount "life";
    }
    construction number: Amount {
        element NumberAmount { number: lex SignedNumber, }
        form number = lex(number);
    }
    construction variable: Amount {
        element VariableAmount { variable: lex Variable, }
        form variable = lex(variable);
    }

    root Ability { punctuation = "."; eoi = true; standalone_render = true; }
    root Sentence { punctuation = "."; eoi = false; standalone_render = true; }
}
