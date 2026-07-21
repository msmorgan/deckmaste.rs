use std::fmt;

use crate::Ability;
use crate::AbilityKind;
use crate::ActivatedAbility;
use crate::Clause;
use crate::ConditionalClause;
use crate::Cost;
use crate::Diagnostic;
use crate::KeywordAbility;
use crate::KeywordAbilityList;
use crate::LoyaltyAbility;
use crate::ModalAbility;
use crate::ModalFrame;
use crate::Mode;
use crate::OracleText;
use crate::Paragraph;
use crate::Predicate;
use crate::Sentence;
use crate::SimpleClause;
use crate::Span;
use crate::Token;
use crate::TriggeredAbility;

/// A human-readable AST view whose spans are resolved against its source text.
///
/// Container spans are omitted. Spans that carry syntactic meaning are shown
/// as the text they cover, such as a complete printed ability, a token's text,
/// or a predicate's verb.
pub struct SourceDebug<'ast, 'source> {
    ast: &'ast OracleText,
    source: &'source str,
}

impl OracleText {
    /// Returns a debug view that resolves this AST's spans to `source` slices.
    ///
    /// `source` should be the same string that was passed to [`crate::parse`].
    #[must_use]
    pub const fn source_debug<'ast, 'source>(
        &'ast self,
        source: &'source str,
    ) -> SourceDebug<'ast, 'source> {
        SourceDebug { ast: self, source }
    }
}

impl fmt::Debug for SourceDebug<'_, '_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Resolved::new(self.source, self.ast).fmt(formatter)
    }
}

struct Resolved<'source, 'value, T: ?Sized> {
    source: &'source str,
    value: &'value T,
}

impl<'source, 'value, T: ?Sized> Resolved<'source, 'value, T> {
    const fn new(source: &'source str, value: &'value T) -> Self {
        Self { source, value }
    }
}

impl<T: ResolvedDebug + ?Sized> fmt::Debug for Resolved<'_, '_, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt_resolved(self.source, formatter)
    }
}

trait ResolvedDebug {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T: ResolvedDebug> ResolvedDebug for [T] {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = formatter.debug_list();
        for value in self {
            list.entry(&Resolved::new(source, value));
        }
        list.finish()
    }
}

impl<T: ResolvedDebug> ResolvedDebug for Option<T> {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Some(value) => formatter
                .debug_tuple("Some")
                .field(&Resolved::new(source, value))
                .finish(),
            None => formatter.write_str("None"),
        }
    }
}

impl ResolvedDebug for Span {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(
            &self
                .text(source)
                .unwrap_or("<span does not belong to this source>"),
            formatter,
        )
    }
}

impl ResolvedDebug for OracleText {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OracleText")
            .field("tokens", &Resolved::new(source, self.tokens.as_slice()))
            .field(
                "abilities",
                &Resolved::new(source, self.abilities.as_slice()),
            )
            .field(
                "diagnostics",
                &Resolved::new(source, self.diagnostics.as_slice()),
            )
            .finish()
    }
}

impl ResolvedDebug for Token {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Token")
            .field("kind", &self.kind)
            .field("text", &Resolved::new(source, &self.span))
            .finish()
    }
}

impl ResolvedDebug for Ability {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Ability")
            .field("text", &Resolved::new(source, &self.span))
            .field("ability_word", &Resolved::new(source, &self.ability_word))
            .field("kind", &Resolved::new(source, &self.kind))
            .finish()
    }
}

impl ResolvedDebug for AbilityKind {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Activated(ability) => formatter
                .debug_tuple("Activated")
                .field(&Resolved::new(source, ability))
                .finish(),
            Self::Triggered(ability) => formatter
                .debug_tuple("Triggered")
                .field(&Resolved::new(source, ability))
                .finish(),
            Self::Loyalty(ability) => formatter
                .debug_tuple("Loyalty")
                .field(&Resolved::new(source, ability))
                .finish(),
            Self::Modal(ability) => formatter
                .debug_tuple("Modal")
                .field(&Resolved::new(source, ability))
                .finish(),
            Self::Keyword(ability) => formatter
                .debug_tuple("Keyword")
                .field(&Resolved::new(source, ability))
                .finish(),
            Self::Paragraph(paragraph) => formatter
                .debug_tuple("Paragraph")
                .field(&Resolved::new(source, paragraph))
                .finish(),
        }
    }
}

impl ResolvedDebug for KeywordAbilityList {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("KeywordAbilityList")
            .field(
                "abilities",
                &Resolved::new(source, self.abilities.as_slice()),
            )
            .finish()
    }
}

impl ResolvedDebug for KeywordAbility {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("KeywordAbility")
            .field("text", &Resolved::new(source, &self.span))
            .field("name", &Resolved::new(source, &self.name))
            .field("argument", &Resolved::new(source, &self.argument))
            .finish()
    }
}

impl ResolvedDebug for ActivatedAbility {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ActivatedAbility")
            .field("cost", &Resolved::new(source, &self.cost))
            .field("effect", &Resolved::new(source, &self.effect))
            .finish()
    }
}

impl ResolvedDebug for Cost {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Cost")
            .field(
                "components",
                &Resolved::new(source, self.components.as_slice()),
            )
            .finish()
    }
}

impl ResolvedDebug for TriggeredAbility {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TriggeredAbility")
            .field("introducer", &self.introducer)
            .field("event", &Resolved::new(source, &self.event))
            .field("effect", &Resolved::new(source, &self.effect))
            .finish()
    }
}

impl ResolvedDebug for LoyaltyAbility {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LoyaltyAbility")
            .field("cost", &Resolved::new(source, &self.cost))
            .field("effect", &Resolved::new(source, &self.effect))
            .finish()
    }
}

impl ResolvedDebug for ModalAbility {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ModalAbility")
            .field("frame", &Resolved::new(source, &self.frame))
            .field("header", &Resolved::new(source, &self.header))
            .field("modes", &Resolved::new(source, self.modes.as_slice()))
            .finish()
    }
}

impl ResolvedDebug for ModalFrame {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unframed => formatter.write_str("Unframed"),
            Self::Activated(cost) => formatter
                .debug_tuple("Activated")
                .field(&Resolved::new(source, cost))
                .finish(),
            Self::Triggered {
                introducer, event, ..
            } => formatter
                .debug_struct("Triggered")
                .field("introducer", introducer)
                .field("event", &Resolved::new(source, event))
                .finish(),
            Self::Loyalty(cost) => formatter
                .debug_tuple("Loyalty")
                .field(&Resolved::new(source, cost))
                .finish(),
        }
    }
}

impl ResolvedDebug for Mode {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Mode")
            .field("bullet", &Resolved::new(source, &self.bullet))
            .field("body", &Resolved::new(source, &self.body))
            .finish()
    }
}

impl ResolvedDebug for Paragraph {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Paragraph")
            .field(
                "sentences",
                &Resolved::new(source, self.sentences.as_slice()),
            )
            .finish()
    }
}

impl ResolvedDebug for Sentence {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Sentence")
            .field("terminal", &Resolved::new(source, &self.terminal))
            .field("clause", &Resolved::new(source, &self.clause))
            .finish()
    }
}

impl ResolvedDebug for Clause {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple(clause) => formatter
                .debug_tuple("Simple")
                .field(&Resolved::new(source, clause))
                .finish(),
            Self::Conditional(clause) => formatter
                .debug_tuple("Conditional")
                .field(&Resolved::new(source, clause))
                .finish(),
        }
    }
}

impl ResolvedDebug for ConditionalClause {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ConditionalClause")
            .field("subordinator", &self.subordinator)
            .field("position", &self.position)
            .field("condition", &Resolved::new(source, &self.condition))
            .field("consequence", &Resolved::new(source, &self.consequence))
            .finish()
    }
}

impl ResolvedDebug for SimpleClause {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SimpleClause")
            .field("subject", &Resolved::new(source, &self.subject))
            .field("predicate", &Resolved::new(source, &self.predicate))
            .finish()
    }
}

impl ResolvedDebug for Predicate {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Predicate")
            .field("auxiliary", &Resolved::new(source, &self.auxiliary))
            .field("verb", &Resolved::new(source, &self.verb))
            .field("verb_kind", &self.verb_kind)
            .field("complement", &Resolved::new(source, &self.complement))
            .field("negated", &self.negated)
            .finish()
    }
}

impl ResolvedDebug for Diagnostic {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Diagnostic")
            .field("kind", &self.kind)
            .field("text", &Resolved::new(source, &self.span))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn source_debug_resolves_syntactic_spans_to_their_text() {
        let source = "Lightning Bolt deals 3 damage to any target.";

        let output = format!("{:#?}", parse(source).source_debug(source));

        assert!(output.contains("text: \"Lightning\""));
        assert!(output.contains("text: \"Lightning Bolt deals 3 damage to any target.\""));
        assert!(output.contains("subject: Some("));
        assert!(output.contains("\"Lightning Bolt\""));
        assert!(output.contains("verb: \"deals\""));
        assert!(output.contains("complement: Some("));
        assert!(output.contains("\"3 damage to any target\""));
        assert!(!output.contains("Span"));
        assert!(!output.contains("start:"));
        assert!(!output.contains("end:"));
    }
}
