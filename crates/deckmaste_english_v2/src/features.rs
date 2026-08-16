use crate::ast::Pronoun;
use crate::ast::VerbLexeme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Agreement {
    Bare,
    ThirdPersonSingular,
}

pub(crate) fn inflect(lexeme: VerbLexeme, agreement: Agreement) -> &'static str {
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

pub(crate) fn agreement_for_pronoun(pronoun: Pronoun) -> Agreement {
    match pronoun {
        Pronoun::It => Agreement::ThirdPersonSingular,
        Pronoun::You => Agreement::Bare,
    }
}
