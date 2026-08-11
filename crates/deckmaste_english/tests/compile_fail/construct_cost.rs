use deckmaste_english::syntax::Cost;
use deckmaste_english::syntax::CostComponent;
use deckmaste_english::syntax::NonEmpty;

fn bypass_generated_validation(source: &Cost) -> Cost {
    Cost {
        flavor_header: source.flavor_header().cloned(),
        components: NonEmpty::from_first(CostComponent::Symbols(Vec::new())),
    }
}

fn main() {}
