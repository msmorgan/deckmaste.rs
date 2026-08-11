use deckmaste_english::syntax::AttachmentScope;
use deckmaste_english::syntax::IndependentClause;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<AttachmentScope<IndependentClause>>();
}
