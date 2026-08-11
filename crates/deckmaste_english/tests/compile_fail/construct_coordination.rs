use deckmaste_english::syntax::Coordination;
use deckmaste_english::syntax::CoordinationJunction;

fn bypass<T>(conjuncts: Vec<T>, junctions: Vec<CoordinationJunction>) -> Coordination<T> {
    Coordination {
        conjuncts,
        junctions,
    }
}

fn main() {}
