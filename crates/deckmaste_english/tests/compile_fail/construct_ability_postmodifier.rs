use deckmaste_english::syntax::AbilityPostmodifier;
use deckmaste_english::syntax::QuotedAbility;

fn bypass(ability: Box<QuotedAbility>) -> AbilityPostmodifier {
    AbilityPostmodifier { ability }
}

fn main() {}
