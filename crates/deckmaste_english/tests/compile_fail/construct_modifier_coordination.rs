use deckmaste_english::features::Conjunction;
use deckmaste_english::syntax::ModifierCoordination;
use deckmaste_english::syntax::NominalModifier;

fn bypass_checked_member(
    conjunction: Option<Conjunction>,
    modifier: NominalModifier,
) -> ModifierCoordination {
    ModifierCoordination {
        conjunction,
        modifier,
    }
}

fn main() {}
