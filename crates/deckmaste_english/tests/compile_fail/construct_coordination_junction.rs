use deckmaste_english::features::Comma;
use deckmaste_english::features::Conjunction;
use deckmaste_english::syntax::CoordinationJunction;

fn bypass(conjunction: Option<Conjunction>, comma: Comma) -> CoordinationJunction {
    CoordinationJunction { conjunction, comma }
}

fn main() {}
