use deckmaste_english::syntax::Coordination;
use deckmaste_english::syntax::CoordinationJunction;

fn bypass<T>(first: T, junction: CoordinationJunction, second: T) -> Coordination<T> {
    Coordination::new(first, junction, second)
}

fn main() {}
