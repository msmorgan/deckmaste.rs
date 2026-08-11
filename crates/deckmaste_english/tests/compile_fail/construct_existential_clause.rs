use deckmaste_english::syntax::ExistentialClause;
use deckmaste_english::syntax::ExistentialForm;
use deckmaste_english::syntax::NounPhrase;

fn bypass(form: ExistentialForm, pivot: NounPhrase) -> ExistentialClause {
    ExistentialClause { form, pivot }
}

fn main() {}
