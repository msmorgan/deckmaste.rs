use deckmaste_english::syntax::Cost;

fn bypass_generated_validation(source: &Cost) -> Cost {
    Cost {
        flavor_header: source.flavor_header().cloned(),
        components: source.components().to_vec(),
    }
}

fn main() {}
