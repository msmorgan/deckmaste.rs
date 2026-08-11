use deckmaste_english::syntax::RelativeBody;
use deckmaste_english::syntax::RelativeClause;
use deckmaste_english::syntax::RelativeMarker;

fn bypass(marker: RelativeMarker, body: RelativeBody) -> RelativeClause {
    RelativeClause { marker, body }
}

fn main() {}
