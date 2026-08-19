use crate::constructions::LexicalOwner;
use crate::context::ParseContext;
use crate::parser::TextSpan;
use crate::parser::ownership::RawRenderedClaim;

pub trait Render {
    fn render(
        &self,
        context: &ParseContext<'_>,
        environment: &crate::environment::ParserEnvironment,
    ) -> String;
}

pub(crate) enum ClaimSink<'a> {
    Noop,
    Collect(&'a mut Vec<RawRenderedClaim>),
}

pub(crate) struct Writer<'a> {
    output: String,
    capitalize_next: bool,
    claims: ClaimSink<'a>,
}

impl Writer<'_> {
    pub(crate) fn new() -> Self {
        Self {
            output: String::new(),
            capitalize_next: true,
            claims: ClaimSink::Noop,
        }
    }

    pub(crate) fn collecting(claims: &mut Vec<RawRenderedClaim>) -> Writer<'_> {
        Writer {
            output: String::new(),
            capitalize_next: true,
            claims: ClaimSink::Collect(claims),
        }
    }

    pub(crate) fn claim(
        &mut self,
        owner: impl FnOnce() -> LexicalOwner,
        render: impl FnOnce(&mut Self),
    ) {
        let start = self.output.len();
        render(self);
        let end = self.output.len();
        if let ClaimSink::Collect(claims) = &mut self.claims {
            claims.push(RawRenderedClaim {
                span: TextSpan { start, end },
                owner: owner(),
            });
        }
    }

    pub(crate) fn word(&mut self, word: &str) {
        if !self.output.is_empty() {
            self.output.push(' ');
        }
        if self.capitalize_next {
            let mut characters = word.chars();
            if let Some(first) = characters.next() {
                self.output.extend(first.to_uppercase());
                self.output.push_str(characters.as_str());
            }
            self.capitalize_next = false;
        } else {
            self.output.push_str(word);
        }
    }

    pub(crate) fn identity(&mut self, identity: &str) {
        if !self.output.is_empty() {
            self.output.push(' ');
        }
        self.output.push_str(identity);
        self.capitalize_next = false;
    }

    pub(crate) fn punctuation(&mut self, mark: char) {
        self.output.push(mark);
        self.capitalize_next = mark == '.';
    }

    pub(crate) fn finish(self) -> String {
        self.output
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use crate::constructions::LexicalOwner;
    use crate::constructions::LexicalProvenanceKind;
    use crate::context::ParseContext;
    use crate::environment::canonical_test_environment;
    use crate::parser::Parser;

    #[test]
    fn render_provenance_claims_spaces_punctuation_and_structured_terminals() {
        let environment = canonical_test_environment();
        let parser = Parser::new(environment.clone()).unwrap();
        let context = ParseContext::new("Context Card").unwrap();
        let ability = parser.parse("Destroy target creature.", &context).unwrap();
        let (rendered, claims) =
            crate::constructions::render_ability_with_claims(&ability, &context, &environment);
        assert_eq!(rendered, "Destroy target creature.");
        assert_eq!(
            claims
                .iter()
                .map(|claim| (claim.span.start, claim.span.end, claim.owner.stable_id()))
                .collect::<Vec<_>>(),
            [
                (0, 7, "lexeme:keyword_action/Destroy/bare"),
                (7, 14, "form:target/target/0"),
                (14, 23, "lexeme:type/Creature/singular"),
                (23, 24, "root:Ability/punctuation"),
            ]
        );
    }

    #[test]
    fn render_provenance_noop_sink_never_constructs_an_owner_label() {
        let labels = Cell::new(0);
        let mut writer = super::Writer::new();
        writer.claim(
            || {
                labels.set(labels.get() + 1);
                LexicalOwner::static_owner(LexicalProvenanceKind::FormLiteral, "form:test/test/0")
            },
            |writer| writer.word("test"),
        );
        assert_eq!(writer.finish(), "Test");
        assert_eq!(labels.get(), 0);
    }
}
