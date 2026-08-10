use deckmaste_english::features::GapState;
use deckmaste_english::syntax::RelativeBody;
use deckmaste_english::syntax::RelativeClause;
use deckmaste_english::syntax::RelativeMarker;

fn bypass(marker: RelativeMarker, gap: GapState, body: RelativeBody) -> RelativeClause {
    RelativeClause { marker, gap, body }
}

fn main() {}
