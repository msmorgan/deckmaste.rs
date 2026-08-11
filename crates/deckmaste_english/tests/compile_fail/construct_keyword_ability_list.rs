use deckmaste_english::syntax::KeywordAbilityList;
use deckmaste_english::syntax::SeparatedNonEmpty;

fn bypass_generated_validation(source: &KeywordAbilityList) -> KeywordAbilityList {
    KeywordAbilityList {
        abilities: SeparatedNonEmpty::from_first(source.separated_abilities().first().clone()),
        trailing: None,
    }
}

fn main() {}
