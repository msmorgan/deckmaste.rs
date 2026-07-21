use std::fmt;

use crate::Ability;
use crate::AbilityKind;
use crate::ActivatedAbility;
use crate::Clause;
use crate::ConditionalClause;
use crate::CoordinatedClause;
use crate::CoordinatedPredicate;
use crate::Cost;
use crate::Diagnostic;
use crate::EmbeddedRules;
use crate::KeywordAbility;
use crate::KeywordAbilityList;
use crate::LoyaltyAbility;
use crate::ModalAbility;
use crate::ModalFrame;
use crate::Mode;
use crate::OracleText;
use crate::Paragraph;
use crate::Phrase;
use crate::Predicate;
use crate::ReminderText;
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

/// A human-readable view of only an Oracle text's parsed abilities.
pub struct AbilitiesSourceDebug<'ast, 'source> {
    ast: &'ast OracleText,
    source: &'source str,
}

/// A human-readable view of only an Oracle text's diagnostics.
pub struct DiagnosticsSourceDebug<'ast, 'source> {
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

    #[must_use]
    pub const fn abilities_source_debug<'ast, 'source>(
        &'ast self,
        source: &'source str,
    ) -> AbilitiesSourceDebug<'ast, 'source> {
        AbilitiesSourceDebug { ast: self, source }
    }

    #[must_use]
    pub const fn diagnostics_source_debug<'ast, 'source>(
        &'ast self,
        source: &'source str,
    ) -> DiagnosticsSourceDebug<'ast, 'source> {
        DiagnosticsSourceDebug { ast: self, source }
    }
}

impl fmt::Debug for SourceDebug<'_, '_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Resolved::new(self.source, self.ast).fmt(formatter)
    }
}

impl fmt::Debug for AbilitiesSourceDebug<'_, '_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Resolved::new(self.source, self.ast.abilities.as_slice()).fmt(formatter)
    }
}

impl fmt::Debug for DiagnosticsSourceDebug<'_, '_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        Resolved::new(self.source, self.ast.diagnostics.as_slice()).fmt(formatter)
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
        let mut ability = formatter.debug_struct("Ability");
        ability
            .field("text", &Resolved::new(source, &self.span))
            .field("ability_word", &Resolved::new(source, &self.ability_word));
        if !self.reminder_text.is_empty() {
            ability.field(
                "reminder_text",
                &Resolved::new(source, self.reminder_text.as_slice()),
            );
        }
        ability
            .field("kind", &Resolved::new(source, &self.kind))
            .finish()
    }
}

impl ResolvedDebug for ReminderText {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.span.fmt_resolved(source, formatter)
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
        let mut keyword = formatter.debug_struct("KeywordAbility");
        keyword
            .field("name", &self.name)
            .field("argument", &Resolved::new(source, &self.argument));
        keyword.finish()
    }
}

impl ResolvedDebug for EmbeddedRules {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("EmbeddedRules")
            .field("text", &Resolved::new(source, &self.span))
            .field("ability", &Resolved::new(source, self.ability.as_ref()))
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
        let mut clause = formatter.debug_struct("SimpleClause");
        if let Some(unparsed) = &self.unparsed {
            clause.field("unparsed", &Resolved::new(source, unparsed));
        }
        clause
            .field("subject", &Resolved::new(source, &self.subject))
            .field("predicate", &Resolved::new(source, &self.predicate));
        if !self.coordinated_predicates.is_empty() {
            clause.field(
                "coordinated_predicates",
                &Resolved::new(source, self.coordinated_predicates.as_slice()),
            );
        }
        if !self.coordinated_clauses.is_empty() {
            clause.field(
                "coordinated_clauses",
                &Resolved::new(source, self.coordinated_clauses.as_slice()),
            );
        }
        clause.finish()
    }
}

impl ResolvedDebug for CoordinatedClause {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CoordinatedClause")
            .field("conjunction", &self.conjunction)
            .field("clause", &Resolved::new(source, self.clause.as_ref()))
            .finish()
    }
}

impl ResolvedDebug for CoordinatedPredicate {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CoordinatedPredicate")
            .field("conjunction", &self.conjunction)
            .field("predicate", &Resolved::new(source, &self.predicate))
            .finish()
    }
}

impl ResolvedDebug for Predicate {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut predicate = formatter.debug_struct("Predicate");
        predicate
            .field("auxiliary", &Resolved::new(source, &self.auxiliary))
            .field("verb", &Resolved::new(source, &self.verb))
            .field("verb_kind", &self.verb_kind)
            .field("complement", &Resolved::new(source, &self.complement));
        predicate.field("negated", &self.negated).finish()
    }
}

impl ResolvedDebug for Phrase {
    fn fmt_resolved(&self, source: &str, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownPhrase(text) => {
                formatter.debug_tuple("UnknownPhrase").field(text).finish()
            }
            Self::Lexeme {
                text,
                lemma,
                part_of_speech,
            } => formatter
                .debug_struct("Lexeme")
                .field("text", text)
                .field("lemma", lemma)
                .field("part_of_speech", part_of_speech)
                .finish(),
            Self::ThisCard { text, form } => formatter
                .debug_struct("ThisCard")
                .field("text", text)
                .field("form", form)
                .finish(),
            Self::OracleSymbol { text, symbol } => formatter
                .debug_struct("OracleSymbol")
                .field("text", text)
                .field("symbol", symbol)
                .finish(),
            Self::NounPhrase(phrase) => formatter
                .debug_struct("NounPhrase")
                .field("text", &phrase.text)
                .field("determiner", &phrase.determiner)
                .field("head", &Resolved::new(source, phrase.head.as_ref()))
                .finish(),
            Self::CatalogTerm {
                text,
                canonical,
                kind,
            } => formatter
                .debug_struct("CatalogTerm")
                .field("text", text)
                .field("canonical", canonical)
                .field("kind", kind)
                .finish(),
            Self::EmbeddedRulesPhrase {
                text,
                embedded_rules,
            } => formatter
                .debug_struct("EmbeddedRulesPhrase")
                .field("text", text)
                .field(
                    "embedded_rules",
                    &Resolved::new(source, embedded_rules.as_slice()),
                )
                .finish(),
        }
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
    use crate::Catalogs;
    use crate::parse;
    use crate::parse_with_catalogs;

    #[test]
    fn source_debug_resolves_syntactic_spans_to_their_text() {
        let source = "Lightning Bolt deals 3 damage to any target.";

        let output = format!("{:#?}", parse(source).source_debug(source));

        assert!(output.contains("text: \"Lightning Bolt deals 3 damage to any target.\""));
        assert!(output.contains("subject: Some("));
        assert!(output.contains("UnknownPhrase("));
        assert!(output.contains("\"Lightning Bolt\""));
        assert!(output.contains("verb: Lexeme {"));
        assert!(output.contains("lemma: \"deal\""));
        assert!(output.contains("part_of_speech: Verb"));
        assert!(output.contains("\"deals\""));
        assert!(output.contains("complement: Some("));
        assert!(output.contains("\"3 damage to any target\""));
        assert!(!output.contains("Span"));
        assert!(!output.contains("start:"));
        assert!(!output.contains("end:"));
        assert!(!output.contains("reminder_text"));
    }

    #[test]
    fn source_debug_uses_canonical_keyword_names_without_repeating_item_text() {
        let source = "Flying, deathtouch";
        let catalogs = Catalogs::new(
            ["Flying", "Deathtouch"],
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
        );

        let output = format!(
            "{:#?}",
            parse_with_catalogs(source, &catalogs).source_debug(source)
        );

        assert!(output.contains("name: \"Flying\""));
        assert!(output.contains("name: \"Deathtouch\""));
        assert!(!output.contains("name: \"deathtouch\""));
        assert!(!output.contains("printed_name:"));
    }

    #[test]
    fn source_debug_shows_reminder_text_as_an_opaque_comment() {
        let source = "Draw a card. (Ignore the English in here.)";

        let output = format!("{:#?}", parse(source).source_debug(source));

        assert!(output.contains("reminder_text: ["));
        assert!(output.contains("\"(Ignore the English in here.)\""));
        assert!(!output.contains("verb: \"Ignore\""));
    }

    #[test]
    fn source_debug_shows_coordinated_predicates_without_spans() {
        let source = "Other Goblin creatures you control get +1/+1 and have haste.";

        let output = format!("{:#?}", parse(source).source_debug(source));

        assert!(output.contains("coordinated_predicates: ["));
        assert!(output.contains("conjunction: And"));
        assert!(output.contains("UnknownPhrase("));
        assert!(output.contains("\"+1/+1\""));
        assert!(output.contains("\"get\""));
        assert!(output.contains("\"have\""));
        assert!(!output.contains("parts:"));
        assert!(!output.contains("Punctuation"));
        assert!(!output.contains("conjunction_span"));
        assert!(!output.contains("Span"));
    }

    #[test]
    fn source_debug_shows_catalog_kind_and_canonical_spelling() {
        let source = "Creatures you control have haste.";
        let catalogs = Catalogs::new(
            ["Haste"],
            std::iter::empty::<&str>(),
            std::iter::empty::<&str>(),
        );

        let output = format!(
            "{:#?}",
            parse_with_catalogs(source, &catalogs).source_debug(source)
        );

        assert!(output.contains("CatalogTerm"));
        assert!(output.contains("text: \"haste\""));
        assert!(output.contains("canonical: \"Haste\""));
        assert!(output.contains("kind: KeywordAbility"));
    }
}
