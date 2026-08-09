use deckmaste_english::syntax::ComparisonComplement;
use deckmaste_english::syntax::ComparisonMarker;

fn bypass(mut comparison: ComparisonComplement) {
    comparison.marker = ComparisonMarker::ThanOrEqualTo;
}

fn main() {}
