use deckmaste_english::syntax::KeywordAbilityList;

fn requires_deserialize<T: for<'de> serde::Deserialize<'de>>() {}

fn main() {
    requires_deserialize::<KeywordAbilityList>();
}
