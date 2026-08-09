use deckmaste_english::syntax::ComparisonComplement;
use deckmaste_english::syntax::ComparisonMarker;
use deckmaste_english::syntax::Phrase;

fn bypass(standard: Phrase) -> ComparisonComplement {
    ComparisonComplement {
        marker: ComparisonMarker::Than,
        standard: Box::new(standard),
    }
}

fn main() {}
