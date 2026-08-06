//! Generated noun-phrase and shared-determiner nominal coordination.

use deckmaste_construction_compiler::runtime::GroupData;
use serde::ser::SerializeStruct;

use crate::features::Comma;
use crate::features::Conjunction;
use crate::syntax::AdjectivePhrase;
use crate::syntax::CoordinatedAdjectivePhrase;
use crate::syntax::Determiner;
use crate::syntax::DevotionColors;
use crate::syntax::IndependentClause;
use crate::syntax::InfinitiveClause;
use crate::syntax::KeywordArgument;
use crate::syntax::NominalComplement;
use crate::syntax::NominalPhrase;
use crate::syntax::NominalPhraseCoordination;
use crate::syntax::NounPhrase;
use crate::syntax::NounPhraseCoordination;
use crate::syntax::PowerToughness;
use crate::syntax::PrepositionalPhrase;
use crate::syntax::Quantity;
use crate::syntax::RelativeClause;
use crate::syntax::TransitivePredicate;

deckmaste_constructions_macro::constructions! {
    group noun_coordination;

    element noun_phrase_member bind NounPhraseCoordination {
        comma: surface lex Comma,
        conjunction: opt lex Conjunction,
        phrase: hole NounPhrase,
    }

    element nominal_phrase_member bind NominalPhraseCoordination {
        comma: surface lex Comma,
        conjunction: opt lex Conjunction,
        phrase: hole NominalPhrase,
    }

    element nominal_complement bind NominalComplement {
        variant Adjective: hole AdjectivePhrase,
        variant CoordinatedAdjective: hole CoordinatedAdjectivePhrase,
        variant Prepositional: hole PrepositionalPhrase,
        variant Infinitive: hole InfinitiveClause,
        variant Relative: hole RelativeClause,
        variant ReducedRecipientPassive: hole TransitivePredicate,
        variant Quantity: hole Quantity,
        variant Devotion: hole DevotionColors,
        variant PowerToughness: hole PowerToughness,
        variant EventClause: hole box IndependentClause,
        variant KeywordArgument: hole KeywordArgument,
    }

    construction noun_phrase_coordination: NounPhrase {
        own CoordinatedNounPhrase {
            first: hole box NounPhrase,
            rest: seq noun_phrase_member,
        }
        require rest.len() >= 1;
        require rest.nonfinal.conjunction.is_none();
        require rest.last.conjunction.is_some();
        require rest.last.conjunction in [And, Or, Plus, AndOr];
        derive first = noun_phrase_coordination(first, rest);
        form flat @ 0 = first rest;
    }

    construction shared_determiner_nominal: NounPhrase {
        own CoordinatedNominalPhrase {
            determiner: hole Determiner,
            first: hole box NominalPhrase,
            rest: seq nominal_phrase_member,
            complements: seq nominal_complement,
        }
        require rest.len() >= 1;
        require rest.nonfinal.conjunction.is_none();
        require rest.last.conjunction.is_some();
        require rest.last.conjunction in [And, Or, AndOr];
        derive first = noun_phrase_coordination(determiner, first, rest);
        form shared @ 0 = determiner first rest complements;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&NOUN_COORDINATION_DECLARATION];

#[cfg(test)]
pub(crate) fn build_noun_phrase_coordination(
    first: Box<NounPhrase>,
    rest: Vec<NounPhraseCoordination>,
) -> Result<CoordinatedNounPhrase, deckmaste_construction_compiler::runtime::DeclarationViolation> {
    CoordinatedNounPhrase::try_new(first, rest)
}

#[cfg(test)]
pub(crate) fn parts_noun_phrase_coordination(
    value: &CoordinatedNounPhrase,
) -> (&NounPhrase, &Vec<NounPhraseCoordination>) {
    (value.first().as_ref(), value.rest())
}

#[cfg(test)]
pub(crate) fn build_shared_determiner_nominal(
    determiner: Determiner,
    first: Box<NominalPhrase>,
    rest: Vec<NominalPhraseCoordination>,
    complements: Vec<NominalComplement>,
) -> Result<CoordinatedNominalPhrase, deckmaste_construction_compiler::runtime::DeclarationViolation>
{
    CoordinatedNominalPhrase::try_new(determiner, first, rest, complements)
}

#[cfg(test)]
pub(crate) fn parts_shared_determiner_nominal(
    value: &CoordinatedNominalPhrase,
) -> (
    &Determiner,
    &NominalPhrase,
    &Vec<NominalPhraseCoordination>,
    &Vec<NominalComplement>,
) {
    (
        value.determiner(),
        value.first().as_ref(),
        value.rest(),
        value.complements(),
    )
}

impl Clone for CoordinatedNounPhrase {
    fn clone(&self) -> Self {
        Self::try_new(self.first().clone(), self.rest().clone())
            .expect("an existing coordinated noun phrase satisfies its declaration")
    }
}

impl serde::Serialize for CoordinatedNounPhrase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("CoordinatedNounPhrase", 2)?;
        state.serialize_field("first", self.first())?;
        state.serialize_field("rest", self.rest())?;
        state.end()
    }
}

impl Clone for CoordinatedNominalPhrase {
    fn clone(&self) -> Self {
        Self::try_new(
            self.determiner().clone(),
            self.first().clone(),
            self.rest().clone(),
            self.complements().clone(),
        )
        .expect("an existing coordinated nominal phrase satisfies its declaration")
    }
}

impl serde::Serialize for CoordinatedNominalPhrase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("CoordinatedNominalPhrase", 4)?;
        state.serialize_field("determiner", self.determiner())?;
        state.serialize_field("first", self.first())?;
        state.serialize_field("rest", self.rest())?;
        state.serialize_field("complements", self.complements())?;
        state.end()
    }
}
