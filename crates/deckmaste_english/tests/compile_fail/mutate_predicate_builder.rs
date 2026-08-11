use deckmaste_english::predicate::PredicateBuilder;

fn bypass(builder: &mut PredicateBuilder) {
    builder.phrase = replacement();
}

fn replacement<T>() -> T {
    loop {}
}

fn main() {}
