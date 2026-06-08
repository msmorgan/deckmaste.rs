use super::keyword_todos::create_keyword_todos;

/// The rule that introduces ability words and lists them all; ability words
/// have no individual entries in the CR.
const ABILITY_WORD_RULE: &str = "207.2c";

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

pub(super) fn generate(plugin: &super::PluginLayout) -> anyhow::Result<()> {
    let keywords_bytes = crate::data::academyruins::keywords_bytes()?;
    let keywords = crate::data::academyruins::Keywords::parse(&keywords_bytes)?;
    let rules_bytes = crate::data::academyruins::comprehensive_rules_bytes()?;
    let rules = crate::data::academyruins::RulesMap::parse(&rules_bytes)?;
    create_keyword_todos(
        &plugin.ability_words_dir()?,
        &keywords.ability_words,
        &rules,
        |_| Some(ABILITY_WORD_RULE),
        // [CR#207.2c]'s third sentence is the ever-growing list of every
        // ability word; the first two say everything worth repeating in
        // each stub.
        |rule| truncate_sentences(&rule.format(), 2),
    )
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
