//! Generated interacting grammar used before whole-grammar activation.
//!
//! Vocabulary is deliberately bounded; grammar admission reads Categories,
//! correlated features and selected Complement frames only.

mod vocabulary;
use deckmaste_construction_v3::constructions;
pub use vocabulary::lexicon;

constructions! {
    pub mod grammar {
        category Nominal(number);
        category BasicNounPhrase(number);
        category NounPhrase(number);
        category PrepositionPhrase();
        category Adjunct();
        category Subject(number, person);
        category FinitePredicate(number, person, tense);
        category BarePredicate();
        category Clause();
        category Sentence();
        category Document();

        frame Object = "(kind: \"transitive\", items: [Argument((relation: Complement, category: \"NounPhrase\"))])";
        frame NominalComplement = "(kind: \"prepositional\", items: [Argument((relation: Complement, category: \"BasicNounPhrase\"))])";

        construction Noun: Nominal {
            form [head: lexical(Noun)];
            // Normalized equivalent forms exercise derivation/Reading identity.
            form ["", head: lexical(Noun)];
            require head.countability = Count;
            export number = head.number;
        }
        construction PremodifiedNominal: Nominal {
            form [modifier: lexical(Adjective), " ", head: lexical(Noun)];
            require head.countability = Count;
            export number = head.number;
        }
        construction DeterminedNounPhrase: BasicNounPhrase {
            form [determiner: lexical(Determinative), " ", nominal: Nominal];
            agree determiner.number = nominal.number;
            export number = nominal.number;
        }
        construction BarePlural: BasicNounPhrase {
            form [nominal: Nominal];
            require nominal.number = Plural;
            export number = nominal.number;
        }
        construction NounPhrase: NounPhrase {
            form [head: BasicNounPhrase, modifier: optional(Adjunct)];
            export number = head.number;
        }
        construction PrepositionPhrase: PrepositionPhrase {
            form [head: lexical(Preposition), " ", complement: BasicNounPhrase];
            require head.frame = NominalComplement;
        }
        construction Adjunct: Adjunct {
            form [" ", phrase: PrepositionPhrase];
        }
        construction Subject: Subject {
            form [head: lexical(Pronoun)];
            require head.case = Nominative;
            export number = head.number;
            export person = head.person;
        }
        construction FinitePredicate: FinitePredicate {
            form [head: lexical(Verb), " ", complement: NounPhrase, modifier: optional(Adjunct)];
            require head.finiteness = Finite;
            require head.frame = Object;
            export number = head.number;
            export person = head.person;
            export tense = head.tense;
        }
        construction BarePredicate: BarePredicate {
            form [head: lexical(Verb), " ", complement: NounPhrase, modifier: optional(Adjunct)];
            require head.form = Plain;
            require head.finiteness = Nonfinite;
            require head.frame = Object;
        }
        construction FiniteClause: Clause {
            form [subject: Subject, " ", predicate: FinitePredicate];
            agree subject.number = predicate.number;
            agree subject.person = predicate.person;
        }
        construction Imperative: Clause {
            form [predicate: BarePredicate];
        }
        construction Sentence: Sentence {
            form [clause: Clause, "."];
        }
        construction Document: Document {
            form [sentences: repeat(Sentence, "\n")];
        }
        construction HeadedDocument: Document {
            form [header: Nominal, "\n", sentences: repeat(Sentence, "\n")];
        }
    }
}
