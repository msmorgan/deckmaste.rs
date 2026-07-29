use super::BoundedQuantityKind;
use super::EnglishLexicalSlot;
use super::Expected;
use super::HashMap;
use super::Nonterminal;
use super::NounUsage;
use super::Numeral;
use super::ParseCost;
use super::PronounCase;
use super::Punctuation;
use super::Rule;
use super::RuleId;
use super::RuleTag;
use super::clause;

#[derive(Default)]
pub(super) struct RuleBuilder {
    pub(super) rules: Vec<Rule<Nonterminal, EnglishLexicalSlot>>,
    pub(super) tags: Vec<RuleTag>,
    pub(super) rules_by_lhs: HashMap<Nonterminal, Vec<RuleId>>,
}

impl RuleBuilder {
    pub(super) fn add(
        &mut self,
        tag: RuleTag,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
    ) {
        self.add_with_cost(tag, lhs, rhs, ParseCost::default());
    }

    pub(super) fn add_with_cost(
        &mut self,
        tag: RuleTag,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
        local_cost: ParseCost,
    ) {
        let id = RuleId::new(self.rules.len());
        self.rules.push(Rule {
            lhs,
            rhs: rhs.into_iter().collect(),
            local_cost,
        });
        self.tags.push(tag);
        self.rules_by_lhs.entry(lhs).or_default().push(id);
    }

    #[allow(
        clippy::too_many_lines,
        reason = "nominal rule construction is intentionally broad"
    )]
    pub(super) fn add_nominal_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        for notation in [
            Numeral::Cardinal,
            Numeral::Ordinal,
            Numeral::Arabic(false),
            Numeral::Arabic(true),
            Numeral::Roman,
        ] {
            self.add(
                RuleTag::QuantityExact,
                N::Quantity,
                [l(L::Number(notation))],
            );
            self.add(
                RuleTag::QuantityUpTo,
                N::Quantity,
                [l(L::Up), l(L::To), l(L::Number(notation))],
            );
        }
        self.add(
            RuleTag::QuantityAtLeast,
            N::Quantity,
            [l(L::QuantityAtLeast)],
        );
        self.add(RuleTag::QuantityOr, N::Quantity, [l(L::QuantityOr)]);
        self.add(RuleTag::QuantityX, N::Quantity, [l(L::QuantityX)]);
        self.add(RuleTag::QuantityBoth, N::Quantity, [l(L::QuantityBoth)]);
        self.add(
            RuleTag::QuantityThatMany,
            N::Quantity,
            [l(L::QuantityThatMany)],
        );
        self.add(
            RuleTag::QuantityThatMuch,
            N::Quantity,
            [l(L::QuantityThatMuch)],
        );
        self.add(
            RuleTag::QuantityMoreThan,
            N::Quantity,
            [l(L::QuantityBound(BoundedQuantityKind::MoreThan))],
        );
        self.add(
            RuleTag::QuantityFewerThan,
            N::Quantity,
            [l(L::QuantityBound(BoundedQuantityKind::FewerThan))],
        );

        self.add(RuleTag::DeterminerClosed, N::Determiner, [l(L::Determiner)]);
        self.add(
            RuleTag::DeterminerTarget,
            N::Determiner,
            [l(L::DeterminerTarget)],
        );
        self.add(
            RuleTag::DeterminerQuantifiedTarget,
            N::Determiner,
            [n(N::Quantity), l(L::DeterminerTarget)],
        );
        self.add(RuleTag::DeterminerQuantity, N::Determiner, [n(N::Quantity)]);
        self.add(
            RuleTag::DeterminerPossessiveThisCard,
            N::Determiner,
            [l(L::PossessiveThisCard)],
        );
        self.add(
            RuleTag::DeterminerPossessiveNoun,
            N::Determiner,
            [n(N::PossessiveNounPhrase)],
        );

        self.add(RuleTag::Adjective, N::Adjective, [l(L::Adjective)]);
        self.add(
            RuleTag::AdjectivePhrase,
            N::AdjectivePhrase,
            [n(N::Adjective)],
        );
        self.add(
            RuleTag::AdjectivePhraseFaceUp,
            N::AdjectivePhrase,
            [l(L::Face), l(L::Up)],
        );
        self.add(
            RuleTag::AdjectivePhraseFaceDown,
            N::AdjectivePhrase,
            [l(L::Face), l(L::Down)],
        );
        self.add(
            RuleTag::ComparisonStandard,
            N::ComparisonStandard,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::ComparisonStandard,
            N::ComparisonStandard,
            [n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::ComparisonStandard,
            N::ComparisonStandard,
            [n(N::Clause)],
        );
        self.add(
            RuleTag::ComparisonThan,
            N::ComparisonComplement,
            [l(L::Than), n(N::ComparisonStandard)],
        );
        self.add(
            RuleTag::ComparisonThanOrEqualTo,
            N::ComparisonComplement,
            [l(L::Than), l(L::OrEqualTo), n(N::ComparisonStandard)],
        );
        self.add(
            RuleTag::AdjectivePhraseComparison,
            N::AdjectivePhrase,
            [n(N::Adjective), n(N::ComparisonComplement)],
        );
        // `2 greater` / `two greater`: a numeral degree premodifier on an
        // `OrComparative` adjective. Predicative only — see
        // `AdjectiveComparisonState::Measured`.
        for notation in [Numeral::Cardinal, Numeral::Arabic(false)] {
            self.add(
                RuleTag::AdjectivePhraseDegreeMeasure,
                N::AdjectivePhrase,
                [l(L::Number(notation)), n(N::Adjective)],
            );
        }
        self.add(RuleTag::Noun, N::Noun, [l(L::Noun(NounUsage::Either))]);
        self.add(
            RuleTag::PossessiveNounBase,
            N::PossessiveNounPhrase,
            [l(L::PossessiveNoun)],
        );
        self.add(
            RuleTag::PossessiveNounDetermined,
            N::PossessiveNounPhrase,
            [n(N::Determiner), n(N::PossessiveNounPhrase)],
        );

        self.add(RuleTag::NominalNoun, N::Nominal, [n(N::Noun)]);
        self.add_with_cost(
            RuleTag::NominalAdjective,
            N::Nominal,
            [n(N::AdjectivePhrase), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NominalNounModifier,
            N::Nominal,
            [n(N::Noun), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        // `declare attackers`/`declare blockers` — every child is a
        // literal-token slot, so this production can only ever match those
        // two exact three-word sequences. Plain `add`, no cost: cost cannot
        // fix a recognition-breadth problem (the `restrict`-round
        // regression), so none is used here on purpose.
        self.add(
            RuleTag::NominalCombatStepName,
            N::Nominal,
            [
                l(L::CombatStepDeclare),
                l(L::CombatStepParticipants),
                l(L::CombatStepHead),
            ],
        );
        self.add_with_cost(
            RuleTag::NominalNegatedModifier,
            N::Nominal,
            [l(L::NegatedModifier), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NominalQuantityModifier,
            N::Nominal,
            [n(N::Quantity), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NominalPowerToughnessModifier,
            N::Nominal,
            [l(L::PowerToughness), n(N::Nominal)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add(
            RuleTag::NominalDeterminer,
            N::Nominal,
            [n(N::Determiner), n(N::Nominal)],
        );
        self.add(
            RuleTag::NominalPrepositional,
            N::Nominal,
            [n(N::Nominal), n(N::PrepositionalPhrase)],
        );
        self.add(
            RuleTag::NominalInfinitive,
            N::Nominal,
            [n(N::Nominal), n(N::InfinitiveClause)],
        );
        self.add_with_cost(
            RuleTag::NominalQuantityComplement,
            N::Nominal,
            [n(N::Nominal), n(N::Quantity)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add(
            RuleTag::NominalRelative,
            N::Nominal,
            [n(N::Nominal), n(N::RelativeClause)],
        );
        self.add(
            RuleTag::NominalPostpositiveAdjective,
            N::Nominal,
            [n(N::Nominal), n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::NominalComparison,
            N::Nominal,
            [n(N::Nominal), n(N::ComparisonComplement)],
        );

        // The `devotion` value nominal with its mandatory concrete-color
        // argument: `devotion to <color>` or `devotion to <color> and <color>`.
        // Gated by the dedicated `DevotionValue` head so the bare-color
        // `DevotionColors` rules stay out of ordinary phrases; the generic
        // `devotion to a/each/that color` shapes take the count-noun +
        // prepositional path instead.
        self.add(
            RuleTag::NominalDevotion,
            N::Nominal,
            [l(L::DevotionValue), l(L::To), n(N::DevotionColors)],
        );
        self.add(
            RuleTag::DevotionColorSingle,
            N::DevotionColors,
            [l(L::ColorWord)],
        );
        self.add(
            RuleTag::DevotionColorPair,
            N::DevotionColors,
            [l(L::ColorWord), l(L::Conjunction), l(L::ColorWord)],
        );

        // `the number of times <clause>`: the plural `times` head takes a bare
        // finite clause as a reduced adjunct-relative complement. Gated by the
        // dedicated `TimesNoun` head so no other noun admits a bare clause.
        self.add(
            RuleTag::NominalTimesClause,
            N::Nominal,
            [l(L::TimesNoun), n(N::Clause)],
        );

        self.add(RuleTag::NounPhraseNominal, N::NounPhrase, [n(N::Nominal)]);
        self.add(
            RuleTag::NounPhraseSubjectPronoun,
            N::NounPhrase,
            [l(L::Pronoun(PronounCase::Subject))],
        );
        self.add(
            RuleTag::NounPhraseObjectPronoun,
            N::NounPhrase,
            [l(L::Pronoun(PronounCase::Object))],
        );
        self.add(
            RuleTag::NounPhraseReciprocal,
            N::NounPhrase,
            [l(L::Reciprocal)],
        );
        self.add(RuleTag::NounPhraseQuantity, N::NounPhrase, [n(N::Quantity)]);
        self.add(RuleTag::NounPhraseThisCard, N::NounPhrase, [l(L::ThisCard)]);
        self.add(
            RuleTag::NounPhraseFullThisCard,
            N::NounPhrase,
            [l(L::FullThisCard)],
        );
        self.add(
            RuleTag::NounPhrasePossessiveThisCard,
            N::NounPhrase,
            [l(L::PossessiveThisCard)],
        );
        self.add(
            RuleTag::NounPhraseDemonstrative,
            N::NounPhrase,
            [l(L::Demonstrative)],
        );
        self.add(
            RuleTag::NounPhrasePartitive,
            N::NounPhrase,
            [n(N::Quantity), l(L::Of), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::NounPhraseEachPartitive,
            N::NounPhrase,
            [l(L::EachDeterminer), l(L::Of), n(N::NounPhrase)],
        );
        // `any number of <plural NounPhrase>` — notional plural concord
        // [`anof` round]. The categorical gate is the dedicated `any` and
        // `number` lexical slots, which scan nothing but those two literal
        // words; that gate is already enforced at dot 0/dot 1, before the
        // recursive `NounPhrase` is even predicted, so no `accepts_prefix`
        // change is needed. The one remaining condition — the final noun
        // phrase must be plural — depends on the fourth child and cannot be
        // hoisted before reduce. Registered with a precedence dispreference:
        // this is a fallback behind the ordinary formal-singular nominal
        // analysis of the same surface, not a competing analysis of a
        // different surface. `precedence` only breaks ties among derivations
        // that both complete, so the formal-singular reading wins whenever it
        // completes, and this production is the only parse whenever a plural
        // predicate makes the singular reading fail to complete. Same idiom
        // as `NounPhraseCoordination` below.
        self.add_with_cost(
            RuleTag::NounPhraseAnyNumberOf,
            N::NounPhrase,
            [
                l(L::AnyDeterminer),
                l(L::NumberNoun),
                l(L::Of),
                n(N::NounPhrase),
            ],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NounPhraseCoordination,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Conjunction), n(N::NounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::NounPhraseAdditiveCoordination,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Plus), n(N::NounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        // Arithmetic value expressions. `plus` rides the additive coordination
        // above and `twice` the copular precomplement adverb, so only the
        // subtraction and halving operators are added here as structured value
        // nodes.
        self.add_with_cost(
            RuleTag::NounPhraseMinus,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Minus), n(N::NounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add(
            RuleTag::NounPhraseHalf,
            N::NounPhrase,
            [l(L::Half), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::NounPhraseHalfRoundedUp,
            N::NounPhrase,
            [
                l(L::Half),
                n(N::NounPhrase),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Rounded),
                l(L::Up),
            ],
        );
        self.add(
            RuleTag::NounPhraseHalfRoundedDown,
            N::NounPhrase,
            [
                l(L::Half),
                n(N::NounPhrase),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Rounded),
                l(L::Down),
            ],
        );

        self.add(
            RuleTag::PrepositionalPhrase,
            N::PrepositionalPhrase,
            [l(L::Preposition), n(N::PrepositionalObject)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [n(N::PrepositionalPhrase)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [n(N::GerundClause)],
        );
        self.add(
            RuleTag::PrepositionalObject,
            N::PrepositionalObject,
            [l(L::Adverb)],
        );
    }

    pub(super) fn add_clause_rules(&mut self) {
        clause::add_rules(self);
    }

    /// The markerless recipient-passive relative (`a creature dealt damage
    /// this way`). Its dedicated nonterminal starts with a scan-time-gated
    /// participle and reuses only the predicate extensions the construction
    /// needs. It never predicts generic `VerbPhrase`, whose registration here
    /// previously perturbed unrelated `do so` derivations.
    pub(super) fn add_reduced_recipient_passive_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add(
            RuleTag::VerbPhraseBase,
            N::ReducedRecipientPassive,
            [l(L::ReducedRecipientPassiveParticiple)],
        );
        self.add(
            RuleTag::ReducedRecipientPassiveTheme,
            N::ReducedRecipientPassiveTheme,
            [n(N::Nominal)],
        );
        self.add(
            RuleTag::VerbPhraseDirectObject,
            N::ReducedRecipientPassive,
            [
                n(N::ReducedRecipientPassive),
                n(N::ReducedRecipientPassiveTheme),
            ],
        );
        self.add(
            RuleTag::ReducedRecipientPassiveNominalAdjunct,
            N::ReducedRecipientPassive,
            [n(N::ReducedRecipientPassive), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::VerbPhrasePrepositional,
            N::ReducedRecipientPassive,
            [n(N::ReducedRecipientPassive), n(N::PrepositionalPhrase)],
        );
        self.add(
            RuleTag::VerbPhraseAdverb,
            N::ReducedRecipientPassive,
            [n(N::ReducedRecipientPassive), l(L::Adverb)],
        );
        self.add(
            RuleTag::VerbPhraseFrequency,
            N::ReducedRecipientPassive,
            [n(N::ReducedRecipientPassive), n(N::FrequencyPhrase)],
        );
        self.add(
            RuleTag::NominalReducedRecipientPassive,
            N::Nominal,
            [n(N::Nominal), n(N::ReducedRecipientPassive)],
        );
    }

    /// A noun-phrase-denotation exception (`all creatures except (for)
    /// Dragons`).
    /// The completed host is checked at dot 1 before either `except` surface is
    /// predicted, preventing open noun fragments from launching this recursive
    /// noun-phrase attachment.
    pub(super) fn add_set_exception_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add(
            RuleTag::NounPhraseSetExceptionBare,
            N::NounPhrase,
            [n(N::NounPhrase), l(L::Except), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::NounPhraseSetExceptionFor,
            N::NounPhrase,
            [
                n(N::NounPhrase),
                l(L::Except),
                l(L::ForWord),
                n(N::NounPhrase),
            ],
        );
        self.add(
            RuleTag::NounPhraseSetExceptionBare,
            N::NounPhrase,
            [
                n(N::NounPhrase),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Except),
                n(N::NounPhrase),
            ],
        );
        self.add(
            RuleTag::NounPhraseSetExceptionFor,
            N::NounPhrase,
            [
                n(N::NounPhrase),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Except),
                l(L::ForWord),
                n(N::NounPhrase),
            ],
        );
    }

    /// General coordination inside the nominal, appended last so every existing
    /// rule keeps its `RuleId` and every existing parse forest keeps its
    /// alternative indices. Two independent shapes:
    ///
    /// * **Modifier coordination** — a coordinated run of attributive modifiers
    ///   filling one modifier slot (`white and blue`, `artifact, creature, and
    ///   land`). The list is gathered on the dedicated `ModifierList`/
    ///   `CoordinatedModifier` nonterminals so a bare comma run never becomes a
    ///   standalone modifier, and only the closed form prepends to a nominal.
    /// * **Head-list coordination** — comma/Oxford extensions of the existing
    ///   binary noun-phrase coordination (`target artifact, enchantment, or
    ///   land`), so a shared-determiner list of heads joins one construction.
    pub(super) fn add_coordination_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        // A coordinable modifier atom: an adjective phrase, a bare noun, or a
        // `non-` negated modifier (polarity composes per conjunct).
        self.add(
            RuleTag::ModifierConjunctAdjective,
            N::ModifierConjunct,
            [n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::ModifierConjunctNoun,
            N::ModifierConjunct,
            [n(N::Noun)],
        );
        self.add(
            RuleTag::ModifierConjunctNegated,
            N::ModifierConjunct,
            [l(L::NegatedModifier)],
        );

        // The open comma-separated run, gathered left to right.
        self.add(
            RuleTag::ModifierListSingle,
            N::ModifierList,
            [n(N::ModifierConjunct)],
        );
        self.add(
            RuleTag::ModifierListComma,
            N::ModifierList,
            [
                n(N::ModifierList),
                l(L::Punctuation(Punctuation::Comma)),
                n(N::ModifierConjunct),
            ],
        );

        // Closing the run with a conjunction. The bare-conjunction close covers
        // both the simple two-way pair (`white and blue`) and the non-Oxford
        // list (`artifact, creature and land`); the Oxford close adds the comma
        // before the final conjunction (`artifact, creature, and land`).
        self.add_with_cost(
            RuleTag::CoordinatedModifierConjoined,
            N::CoordinatedModifier,
            [
                n(N::ModifierList),
                l(L::Conjunction),
                n(N::ModifierConjunct),
            ],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
        self.add_with_cost(
            RuleTag::CoordinatedModifierOxford,
            N::CoordinatedModifier,
            [
                n(N::ModifierList),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Conjunction),
                n(N::ModifierConjunct),
            ],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );

        // The coordinated run fills one modifier slot on the nominal, binding
        // tighter than the rest of the modifier stack.
        self.add(
            RuleTag::NominalCoordinatedModifier,
            N::Nominal,
            [n(N::CoordinatedModifier), n(N::Nominal)],
        );

        // Head-list coordination: comma/Oxford extension of the existing binary
        // noun-phrase coordination. The open run is gathered on the dedicated
        // `NounPhraseList` nonterminal so a bare comma run never coordinates on
        // its own; only the Oxford close (`, and`/`, or` + a final member)
        // produces a coordinated noun phrase. Two-way `A and B` and un-comma'd
        // `A and B or C` chains already ride the existing binary rule.
        self.add(
            RuleTag::NounPhraseListSingle,
            N::NounPhraseList,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::NounPhraseListComma,
            N::NounPhraseList,
            [
                n(N::NounPhraseList),
                l(L::Punctuation(Punctuation::Comma)),
                n(N::NounPhrase),
            ],
        );
        self.add_with_cost(
            RuleTag::NounPhraseCoordinationOxford,
            N::NounPhrase,
            [
                n(N::NounPhraseList),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Conjunction),
                n(N::NounPhrase),
            ],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
    }

    /// Attachment points that *consume* the landed coordination nonterminals at
    /// positions other than the nominal head. Appended after
    /// [`Self::add_coordination_rules`] so every rule here takes a higher
    /// `RuleId` than the coordination machinery it reads.
    ///
    /// * **Predicative-adjective coordination** (Family C) — a closed
    ///   [`Nonterminal::CoordinatedModifier`] filling a copular or
    ///   intransitive-`be` adjective complement (`it's legendary and snow`,
    ///   `that's red or green`, `that are green and/or white`). The three rules
    ///   below cover the matrix copular remainder, the contracted relative
    ///   copular, and the non-contracted intransitive-`be` verb phrase;
    ///   lowering converts the modifier list into a
    ///   [`CoordinatedAdjectivePhrase`](crate::syntax::CoordinatedAdjectivePhrase),
    ///   rejecting any non-adjective conjunct so the attributive-only shapes
    ///   stay out of predicative position.
    pub(super) fn add_coordination_consumer_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        // Family C: predicative-adjective coordination.
        self.add(
            RuleTag::VerbPhraseCoordinatedAdjective,
            N::VerbPhrase,
            [n(N::VerbPhrase), n(N::CoordinatedModifier)],
        );
        self.add(
            RuleTag::CopularRemainderCoordinatedAdjective,
            N::CopularRemainder,
            [n(N::CoordinatedModifier)],
        );
        self.add(
            RuleTag::RelativeContractedCopularCoordinatedAdjective,
            N::RelativeClause,
            [l(L::SubjectAuxiliary), n(N::CoordinatedModifier)],
        );

        // Family A: a power/toughness value complement on a characteristic
        // nominal (`base power and toughness X/X`). Mirrors the quantity
        // complement (`base power 2`) for the `N/N` token; the shared `base`
        // modifier and coordinated `power and toughness` heads ride the existing
        // nominal-modifier and noun-phrase coordination, with the value recorded
        // on the final characteristic.
        self.add_with_cost(
            RuleTag::NominalPowerToughnessComplement,
            N::Nominal,
            [n(N::Nominal), l(L::PowerToughness)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
    }

    /// A premodified possessor: `[AdjP] [PossessiveNounPhrase]`, e.g. `the
    /// sacrificed creature's`. Registered append-last (see call site) so
    /// existing `RuleId`s are untouched; same cost as `NominalAdjective`.
    pub(super) fn add_possessive_modifier_rules(&mut self) {
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add_with_cost(
            RuleTag::PossessiveNounAdjective,
            N::PossessiveNounPhrase,
            [n(N::AdjectivePhrase), n(N::PossessiveNounPhrase)],
            ParseCost {
                precedence: 1,
                ..ParseCost::default()
            },
        );
    }

    /// Registers the closed `come up heads`/`come up tails` coin-result
    /// predicate tail [CR#705.1,705.2] after every other rule in the
    /// grammar, mirroring `VerbPhraseParticle`'s structural placement. Its
    /// dot-1 gate (`accepts_predicate_prefix`) requires the narrow pending
    /// `Come` frame before either alternative is even predicted, so no other
    /// verb phrase can reach this production.
    pub(super) fn add_coin_result_rules(&mut self) {
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        for side in [
            crate::syntax::CoinSide::Heads,
            crate::syntax::CoinSide::Tails,
        ] {
            self.add(
                RuleTag::VerbPhraseCoinResult,
                N::VerbPhrase,
                [n(N::VerbPhrase), l(EnglishLexicalSlot::CoinResult(side))],
            );
        }
    }

    /// Registers the fronted `While <gerund clause>, <clause>.` production
    /// [CR#701.38d] after every other rule, including the coin-result
    /// predicate. The dot-1 gate in `accepts_predicate_prefix` requires
    /// `Features::Subordinator(While)` before `GerundClause` is even
    /// predicted, so this never cascades into a general fronted-gerund
    /// shape for other subordinators.
    pub(super) fn add_while_gerund_rules(&mut self) {
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add(
            RuleTag::ClauseSubordinateGerundBefore,
            N::Clause,
            [
                l(EnglishLexicalSlot::Subordinator),
                n(N::GerundClause),
                l(EnglishLexicalSlot::Punctuation(Punctuation::Comma)),
                n(N::Clause),
            ],
        );
    }

    /// `kwgrant` round, Stage A: a parameterized keyword ability's symbol-cost
    /// argument fused onto its keyword-noun head in one production —
    /// `ward {2}`, `equip {1}`. The lexical head is one of the dedicated
    /// keyword-noun slots (never the ordinary noun slot), so an ordinary noun
    /// can never enter this rule; only a catalog-surface property (keyword
    /// atom membership) gates it.
    pub(super) fn add_keyword_grant_rules(&mut self) {
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add(
            RuleTag::NominalKeywordSymbolArgument,
            N::Nominal,
            [
                l(EnglishLexicalSlot::SymbolArgumentKeywordNoun),
                l(EnglishLexicalSlot::OracleSymbol),
            ],
        );
        self.add(
            RuleTag::NominalKeywordSymbolArgument,
            N::Nominal,
            [
                l(EnglishLexicalSlot::SymbolArgumentKeywordNoun),
                l(EnglishLexicalSlot::SymbolSequence),
            ],
        );

        // Stage B: explicit `from` qualities (`protection from black`).
        self.add(
            RuleTag::PredicatedQualityFrom,
            N::PredicatedQualityFrom,
            [
                l(EnglishLexicalSlot::FromWord),
                l(EnglishLexicalSlot::ColorWord),
            ],
        );
        self.add(
            RuleTag::PredicatedQualityFrom,
            N::PredicatedQualityFrom,
            [l(EnglishLexicalSlot::FromWord), n(N::NounPhrase)],
        );
        self.add(
            RuleTag::PredicatedArgumentFromSingle,
            N::PredicatedArgumentFrom,
            [n(N::PredicatedQualityFrom)],
        );
        self.add(
            RuleTag::PredicatedArgumentFromExtend,
            N::PredicatedArgumentFrom,
            [
                n(N::PredicatedArgumentFrom),
                l(EnglishLexicalSlot::Conjunction),
                n(N::PredicatedQualityFrom),
            ],
        );
        self.add(
            RuleTag::NominalKeywordPredicatedArgument,
            N::Nominal,
            [
                l(EnglishLexicalSlot::ExplicitPredicatedKeywordNoun),
                n(N::PredicatedArgumentFrom),
            ],
        );

        // Stage C: the atom itself carries `from` (`Hexproof from black`).
        self.add(
            RuleTag::PredicatedQualityBare,
            N::PredicatedQualityBare,
            [l(EnglishLexicalSlot::ColorWord)],
        );
        self.add(
            RuleTag::PredicatedQualityBare,
            N::PredicatedQualityBare,
            [n(N::AdjectivePhrase)],
        );
        self.add(
            RuleTag::PredicatedQualityBare,
            N::PredicatedQualityBare,
            [n(N::NounPhrase)],
        );
        self.add(
            RuleTag::PredicatedArgumentBareSingle,
            N::PredicatedArgumentBare,
            [n(N::PredicatedQualityBare)],
        );
        self.add(
            RuleTag::PredicatedArgumentBareExtend,
            N::PredicatedArgumentBare,
            [
                n(N::PredicatedArgumentBare),
                l(EnglishLexicalSlot::Conjunction),
                n(N::PredicatedQualityFrom),
            ],
        );
        self.add(
            RuleTag::NominalKeywordAtomCarriedPredicatedArgument,
            N::Nominal,
            [
                l(EnglishLexicalSlot::AtomCarriedPredicatedKeywordNoun),
                n(N::PredicatedArgumentBare),
            ],
        );
    }
}
