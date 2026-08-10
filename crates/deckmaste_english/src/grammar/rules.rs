use super::EnglishLexicalSlot;
use super::Expected;
use super::HashMap;
use super::Nonterminal;
use super::ParseCost;
use super::Punctuation;
use super::Rule;
use super::RuleId;
use super::RuleTag;
use super::clause;
use super::construction::construction_id;
#[cfg(test)]
use crate::construction::ConstructionId;
use crate::construction::ProductionId;

#[derive(Debug, Clone, Copy)]
pub(super) enum RuleImpl {
    Handwritten(RuleTag),
    Generated(GeneratedRuleRef),
    GeneratedAux(GeneratedAuxRuleRef),
}

/// One generated production's home: the group's declaration data plus the
/// construction/form indices inside it. Indices, not references, so the type
/// stays `Copy` without self-referential borrows.
#[derive(Debug, Clone, Copy)]
pub(super) struct GeneratedRuleRef {
    pub(super) group: &'static deckmaste_construction_compiler::runtime::GroupData,
    pub(super) construction: usize,
    pub(super) form: usize,
    /// Bit `i` records that sequence-valued surface atom `i` used its
    /// one-or-more helper. A clear bit means the field is the empty sequence.
    pub(super) sequence_atoms: u64,
    pub(super) context: GeneratedRuleContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GeneratedRuleContext {
    Value,
    ObjectGap,
    ReducedRecipientPassive,
    SharedPreposition,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum GeneratedAuxRuleRef {
    Transparent,
    ElementStruct {
        group: &'static deckmaste_construction_compiler::runtime::GroupData,
        element: usize,
        present_fields: u64,
    },
    ElementVariant {
        group: &'static deckmaste_construction_compiler::runtime::GroupData,
        element: usize,
        variant: usize,
    },
    SequenceSeed {
        group: &'static deckmaste_construction_compiler::runtime::GroupData,
        element: usize,
    },
    SequenceExtend {
        group: &'static deckmaste_construction_compiler::runtime::GroupData,
        element: usize,
    },
}

#[derive(Default)]
pub(super) struct RuleBuilder {
    registrations: Vec<RuleRegistration>,
    next_ordinals: HashMap<RuleTag, u16>,
}

struct RuleRegistration {
    rule_impl: RuleImpl,
    rule: Rule<Nonterminal, EnglishLexicalSlot>,
}

pub(super) struct RuleBook {
    pub(super) rules: Vec<Rule<Nonterminal, EnglishLexicalSlot>>,
    pub(super) impls: Vec<RuleImpl>,
    pub(super) rules_by_lhs: HashMap<Nonterminal, Vec<RuleId>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RegistrationOrder {
    Normal,
    #[cfg(test)]
    Reversed,
    #[cfg(test)]
    FixedShuffle,
}

impl RuleBuilder {
    pub(super) fn replace_handwritten_families_for_generated_test(
        &mut self,
        groups: &[&'static deckmaste_construction_compiler::runtime::GroupData],
    ) {
        let generated_ids = groups
            .iter()
            .flat_map(|group| {
                group
                    .constructions
                    .iter()
                    .map(|construction| construction.id)
            })
            .collect::<std::collections::BTreeSet<_>>();
        self.registrations.retain(|registration| {
            let RuleImpl::Handwritten(tag) = registration.rule_impl else {
                return true;
            };
            let id: &'static str = tag.into();
            !generated_ids.contains(id)
        });
    }

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
        let ordinal = self.next_ordinals.entry(tag).or_default();
        let production = ProductionId {
            construction: construction_id(tag),
            ordinal: *ordinal,
        };
        *ordinal = ordinal
            .checked_add(1)
            .expect("a construction family cannot exceed u16 productions");
        self.registrations.push(RuleRegistration {
            rule_impl: RuleImpl::Handwritten(tag),
            rule: Rule {
                production,
                lhs,
                rhs: rhs.into_iter().collect(),
                local_cost,
            },
        });
    }

    /// Generated productions carry EXPLICIT ordinals from their declaration
    /// (`form … @ N`); nothing here may consult `next_ordinals`, whose
    /// incremental numbering generated families must never depend on.
    pub(super) fn add_generated(
        &mut self,
        rule_impl: RuleImpl,
        production: ProductionId,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
    ) {
        self.add_generated_with_cost(rule_impl, production, lhs, rhs, ParseCost::default());
    }

    pub(super) fn add_generated_with_cost(
        &mut self,
        rule_impl: RuleImpl,
        production: ProductionId,
        lhs: Nonterminal,
        rhs: impl IntoIterator<Item = Expected<Nonterminal, EnglishLexicalSlot>>,
        local_cost: ParseCost,
    ) {
        self.registrations.push(RuleRegistration {
            rule_impl,
            rule: Rule {
                production,
                lhs,
                rhs: rhs.into_iter().collect(),
                local_cost,
            },
        });
    }

    pub(super) fn finish(mut self, order: RegistrationOrder) -> RuleBook {
        reorder_registrations(&mut self.registrations, order);
        let mut rules = Vec::with_capacity(self.registrations.len());
        let mut impls = Vec::with_capacity(self.registrations.len());
        let mut rules_by_lhs = HashMap::<Nonterminal, Vec<RuleId>>::new();
        for registration in self.registrations {
            let id = RuleId::new(rules.len());
            rules_by_lhs
                .entry(registration.rule.lhs)
                .or_default()
                .push(id);
            rules.push(registration.rule);
            impls.push(registration.rule_impl);
        }
        RuleBook {
            rules,
            impls,
            rules_by_lhs,
        }
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

    /// A narrow subject-shared copular continuation for additive type
    /// predications: `... and is <NP> in <PP>`. The dedicated shape avoids
    /// admitting arbitrary copular remainders such as `every creature type`.
    pub(super) fn add_shared_copular_coordination_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        self.add(
            RuleTag::ClauseCoordinationCopularNounPrepositional,
            N::Clause,
            [
                n(N::Clause),
                l(L::Conjunction),
                l(L::Copula),
                n(N::NounPhrase),
                n(N::PrepositionalPhrase),
            ],
        );
        self.add(
            RuleTag::ClauseCoordinationCopularNounPrepositionalComma,
            N::Clause,
            [
                n(N::Clause),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Conjunction),
                l(L::Copula),
                n(N::NounPhrase),
                n(N::PrepositionalPhrase),
            ],
        );
        self.add(
            RuleTag::ClauseCoordinationCopularNounPrepositionalAsyndetic,
            N::Clause,
            [
                n(N::Clause),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Copula),
                n(N::NounPhrase),
                n(N::PrepositionalPhrase),
            ],
        );
    }

    /// General coordination inside the nominal. Stable production identity
    /// makes these two shapes independent of their registration position:
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

        self.add_selectional_coordination_rules();
    }

    fn add_selectional_coordination_rules(&mut self) {
        use EnglishLexicalSlot as L;
        use Expected::Lexical as l;
        use Expected::Nonterminal as n;
        use Nonterminal as N;

        // Sibling coordination: each member repeats its own preposition (`from
        // blue and from black`). The rules above instead share one preposition
        // across coordinated objects (`from artifacts, creatures, and
        // enchantments`); registering sibling coordination after them keeps the
        // shared-preposition reading preferred where both would fit.
        // The run's base is a *pair*, not a single phrase, so
        // `PrepositionalPhraseList` always holds at least two members and the
        // Oxford close below therefore needs at least three. That makes strict
        // serial-comma style structural: `A and B` takes no comma (the binary
        // rule), `A, B, and C` takes one (this run plus the close), and
        // `A, and B` matches nothing — a comma before the conjunction of a
        // two-member coordination marks a clause boundary, not a list.
        self.add(
            RuleTag::PrepositionalPhraseListPair,
            N::PrepositionalPhraseList,
            [
                n(N::PrepositionalPhrase),
                l(L::Punctuation(Punctuation::Comma)),
                n(N::PrepositionalPhrase),
            ],
        );
        self.add(
            RuleTag::PrepositionalPhraseListComma,
            N::PrepositionalPhraseList,
            [
                n(N::PrepositionalPhraseList),
                l(L::Punctuation(Punctuation::Comma)),
                n(N::PrepositionalPhrase),
            ],
        );
        self.add(
            RuleTag::PrepositionalPhraseSiblingCoordinated,
            N::PrepositionalPhrase,
            [
                n(N::PrepositionalPhrase),
                l(L::Conjunction),
                n(N::PrepositionalPhrase),
            ],
        );
        self.add(
            RuleTag::PrepositionalPhraseSiblingCoordinated,
            N::PrepositionalPhrase,
            [
                n(N::PrepositionalPhraseList),
                l(L::Punctuation(Punctuation::Comma)),
                l(L::Conjunction),
                n(N::PrepositionalPhrase),
            ],
        );
    }

    /// Attachment points that *consume* the landed coordination nonterminals at
    /// positions other than the nominal head. Kept after
    /// [`Self::add_coordination_rules`] for authoring locality, not precedence.
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
}

fn reorder_registrations(registrations: &mut [RuleRegistration], order: RegistrationOrder) {
    if order == RegistrationOrder::Normal {
        return;
    }
    #[cfg(not(test))]
    let _ = registrations;
    #[cfg(test)]
    {
        let mut families = Vec::<ConstructionId>::new();
        for registration in registrations.iter() {
            let id = registration.rule.production.construction;
            if !families.contains(&id) {
                families.push(id);
            }
        }
        match order {
            RegistrationOrder::Normal => unreachable!(),
            RegistrationOrder::Reversed => families.reverse(),
            RegistrationOrder::FixedShuffle => {
                let original = families.clone();
                families.sort_unstable_by_key(|id| (fnv1a(id.as_str()), *id));
                if families == original && families.len() > 1 {
                    families.rotate_left(1);
                }
            }
        }
        let ranks = families
            .into_iter()
            .enumerate()
            .map(|(rank, id)| (id, rank))
            .collect::<HashMap<_, _>>();
        registrations.sort_by_key(|registration| ranks[&registration.rule.production.construction]);
    }
}

#[cfg(test)]
fn fnv1a(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

#[cfg(test)]
impl RuleBook {
    fn stable_signature(&self) -> Vec<ProductionId> {
        let mut signature = self.numeric_signature();
        signature.sort_unstable();
        signature
    }

    fn numeric_signature(&self) -> Vec<ProductionId> {
        self.rules.iter().map(|rule| rule.production).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::RuleBuilder;
    use crate::chart::Expected;
    use crate::grammar::EnglishLexicalSlot;
    use crate::grammar::Nonterminal;
    use crate::grammar::RuleTag;
    use crate::grammar::rules::RegistrationOrder;

    fn interleaved_builder() -> RuleBuilder {
        let mut builder = RuleBuilder::default();
        builder.add(
            RuleTag::PrepositionalObject,
            Nonterminal::Determiner,
            [Expected::Lexical(EnglishLexicalSlot::Determiner)],
        );
        builder.add(
            RuleTag::PrepositionalObject,
            Nonterminal::Determiner,
            [Expected::Lexical(EnglishLexicalSlot::Determiner)],
        );
        builder.add(
            RuleTag::PrepositionalPhrase,
            Nonterminal::Determiner,
            [Expected::Lexical(EnglishLexicalSlot::DeterminerTarget)],
        );
        builder
    }

    #[test]
    fn family_permutations_preserve_stable_production_inventory() {
        let normal = interleaved_builder().finish(RegistrationOrder::Normal);
        let reversed = interleaved_builder().finish(RegistrationOrder::Reversed);
        let shuffled = interleaved_builder().finish(RegistrationOrder::FixedShuffle);

        assert_eq!(normal.stable_signature(), reversed.stable_signature());
        assert_eq!(normal.stable_signature(), shuffled.stable_signature());
        assert_ne!(normal.numeric_signature(), reversed.numeric_signature());
        assert_ne!(normal.numeric_signature(), shuffled.numeric_signature());
    }
}
