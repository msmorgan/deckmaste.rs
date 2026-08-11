use deckmaste_english::syntax::KeywordAbilityList;

fn bypass_generated_validation(line: &mut KeywordAbilityList) {
    line.abilities = line.separated_abilities().clone();
}

fn main() {}
