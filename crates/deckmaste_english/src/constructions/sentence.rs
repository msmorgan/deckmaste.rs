//! Compiler-derived sentence wrapper declaration.

use deckmaste_construction_compiler::runtime::DeclarationViolation;
use deckmaste_construction_compiler::runtime::GroupData;

use crate::syntax::Clause;
use crate::syntax::Sentence;
use crate::syntax::SentenceBody;

fn sentence_from_clause(clause: Clause) -> Result<Sentence, DeclarationViolation> {
    let Clause::Independent(clause) = clause else {
        return Err(DeclarationViolation {
            construction: "sentence",
            requirement: "clause is standalone and non-subjunctive",
        });
    };
    Ok(Sentence {
        body: SentenceBody::Independent(clause),
    })
}

fn sentence_clause(value: &Sentence) -> Clause {
    let SentenceBody::Independent(clause) = &value.body else {
        unreachable!("only independent-clause sentences inhabit the generated construction")
    };
    Clause::Independent(clause.clone())
}

fn sentence_accepts_period_form(value: &Sentence) -> bool {
    !crate::renderer::sentence_has_structural_terminator(value)
}

deckmaste_constructions_macro::constructions! {
    group sentence;

    construction sentence: Sentence {
        bind Sentence via sentence_from_clause, sentence_clause {
            clause: hole Clause,
        }
        derive clause = complete_sentence(clause);
        form period @ 0 when check(sentence_accepts_period_form) = clause ".";
        form terminal @ 1 otherwise = clause;
        selection unique;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&SENTENCE_DECLARATION];

pub(crate) const fn period_form_ordinal() -> u16 {
    0
}

pub(crate) const fn terminal_form_ordinal() -> u16 {
    1
}
