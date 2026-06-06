use super::keyword_todos::create_keyword_todos;

/// The rule that introduces ability words and lists them all; ability words
/// have no individual entries in the CR.
const ABILITY_WORD_RULE: &str = "207.2c";

pub(super) struct AbilityWordTodos;

/// Keeps at most the first `count` sentences of `text`.
fn truncate_sentences(text: &str, count: usize) -> String {
    let mut matched = 0;
    for (index, _) in text.match_indices(". ") {
        matched += 1;
        if matched == count {
            return text[..=index].to_owned();
        }
    }
    text.to_owned()
}

impl super::Migration for AbilityWordTodos {
    fn apply(&self, plugin: &super::PluginLayout) -> anyhow::Result<()> {
        let keywords = crate::data::keywords()?;
        let rules = crate::data::comprehensive_rules()?;
        create_keyword_todos(
            &plugin.ability_words_dir()?,
            &keywords.ability_words,
            &rules,
            |_| Some(ABILITY_WORD_RULE),
            // 207.2c's third sentence is the ever-growing list of every
            // ability word; the first two say everything worth repeating in
            // each stub.
            |rule| truncate_sentences(&rule.format(), 2),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::truncate_sentences;

    #[test]
    fn sentence_truncation() {
        assert_eq!(
            truncate_sentences("207.2c One. Two, two. Three three three.", 2),
            "207.2c One. Two, two."
        );
        // Fewer sentences than requested: unchanged.
        assert_eq!(truncate_sentences("Just one.", 2), "Just one.");
        assert_eq!(truncate_sentences("One. Two.", 2), "One. Two.");
    }
}
