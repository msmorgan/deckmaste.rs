use RulePosition::Lexical as L;
use RulePosition::Nonterminal as N;
use deckmaste_construction::constructions;

use crate::ast::CatalogIdentity;
use crate::ast::Noun;
use crate::ast::SelfReferenceSpelling;
use crate::ast::Sign;
use crate::ast::SignedNumber;
use crate::context::ParseContext;
use crate::features::agreement_for_pronoun;
use crate::features::inflect;
use crate::parser::BuildValue;
use crate::parser::LexicalMatch;
use crate::parser::Rule;
use crate::parser::RulePosition;
use crate::parser::ScanInput;
use crate::parser::scan_bound_terminal;
use crate::render::Render;
use crate::render::Writer;
use crate::render::render_noun;
use crate::render::render_signed_number;

constructions! {
    vocab TriggerWord { Whenever = "whenever", }
    vocab Article { A = "a", An = "an", }
    vocab Demonstrative { That = "that", Those = "those", }
    vocab Pronoun { It = "it", You = "you", }
    vocab Variable { X = "X", }

    lexeme NounLexeme { Player, }
    lexeme VerbLexeme { Deal, Gain, Control, Be, }

    codec Noun {
        atom = noun;
        value_type = crate::ast::Noun;
        lexical = Lexical::Noun;
        render = render_noun;
        build { pattern = BuildValue::Noun(noun); construct = noun; }
        traversal {
            callback = borrowed;
            argument = noun;
            variant Lexeme;
            variant Catalog;
            match noun {
                Noun::Lexeme(noun_lexeme: NounLexeme) => walk_noun_lexeme(copy(noun_lexeme)),
                Noun::Catalog(catalog_identity: CatalogIdentity) => walk_catalog_identity(borrowed(catalog_identity)),
            }
        }
    }
    codec Sign {
        value_type = crate::ast::Sign;
        traversal {
            callback = copy;
            argument = sign;
            variant Positive;
            variant Negative;
        }
    }
    identity SelfReferenceSpelling {
        value_type = crate::ast::SelfReferenceSpelling;
        lexical = Lexical::SelfReference;
        render context_identity {
            Full => card_name,
            Abbreviated => abbreviated_card_name,
        }
        build { pattern = BuildValue::SelfReference(spelling); construct = spelling; }
        traversal {
            callback = copy;
            argument = spelling;
            variant Full;
            variant Abbreviated;
        }
    }
    codec SignedNumber {
        atom = lex;
        value_type = crate::ast::SignedNumber;
        lexical = Lexical::SignedNumber;
        render = render_signed_number;
        build { pattern = BuildValue::SignedNumber(number); construct = number; }
        traversal {
            callback = borrowed;
            argument = number;
            field sign: Sign;
            field magnitude: u32;
            call walk_sign(copy(sign));
            call visitor::visit_signed_number(borrowed(number));
        }
    }
    identity CatalogIdentity {
        value_type = crate::ast::CatalogIdentity;
        traversal {
            callback = borrowed;
            argument = identity;
            leaf visit_catalog_spelling: str = borrowed;
            field kind: CatalogKind;
            field spelling: str;
            call visitor::visit_catalog_identity(borrowed(identity));
            call visitor::visit_catalog_spelling(borrowed(spelling));
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
        checked {
            visibility trigger = pub(crate);
            visibility event = pub(crate);
            visibility effect = pub(crate);
            constructor = Triggered::new(trigger, event, vec![effect]);
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
        checked {
            visibility spelling = private;
            access spelling = spelling;
            constructor = SelfReferenceNp::new(spelling, context);
        }
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

impl SelfReferenceNp {
    #[must_use]
    pub fn new(spelling: SelfReferenceSpelling, context: &ParseContext<'_>) -> Option<Self> {
        (spelling != SelfReferenceSpelling::Abbreviated
            || context.abbreviated_card_name() != context.card_name())
        .then_some(Self { spelling })
    }

    #[must_use]
    pub const fn spelling(&self) -> SelfReferenceSpelling {
        let Self { spelling } = self;
        *spelling
    }
}

impl Triggered {
    #[must_use]
    pub fn new(trigger: TriggerWord, event: Clause, effects: Vec<Sentence>) -> Option<Self> {
        // Multiple effects await grammar buildout.
        let [effect] = effects.try_into().ok()?;
        Some(Self {
            trigger,
            event,
            effect,
        })
    }

    #[must_use]
    pub const fn trigger(&self) -> TriggerWord {
        self.trigger
    }

    #[must_use]
    pub const fn event(&self) -> &Clause {
        &self.event
    }

    #[must_use]
    pub const fn effect(&self) -> &Sentence {
        &self.effect
    }
}
