use deckmaste_english::syntax::Separated;
use deckmaste_english::syntax::SeparatedNonEmpty;

fn bypass_checked_topology<T, S>(
    first: T,
    continuation: Separated<T, S>,
) -> SeparatedNonEmpty<T, S> {
    SeparatedNonEmpty {
        first,
        rest: vec![continuation],
    }
}

fn main() {}
