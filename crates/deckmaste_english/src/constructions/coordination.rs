//! The real noun/nominal coordination declarations in bind mode: inactive
//! compiler scaffolding related by tests to the handwritten grammar.

use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Comma;
use crate::features::Conjunction;
use crate::syntax::AdjectivePhrase;
use crate::syntax::CoordinatedAdjectivePhrase;
use crate::syntax::CoordinatedNominalPhrase;
use crate::syntax::CoordinatedNounPhrase;
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
        comma: lex Comma,
        conjunction: opt lex Conjunction,
        phrase: hole NounPhrase,
    }

    element nominal_phrase_member bind NominalPhraseCoordination {
        comma: lex Comma,
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
        bind CoordinatedNounPhrase {
            first: hole box NounPhrase,
            rest: seq noun_phrase_member,
        }
        require rest.len() >= 1;
        require rest.nonfinal.comma in [Present];
        require rest.nonfinal.conjunction.is_none();
        require any(rest.len() >= 2, rest.last.comma in [Absent]);
        require rest.last.conjunction.is_some();
        derive first = noun_phrase_coordination(first, rest);
        witness oxford = stored rest.last.comma;
        form flat @ 0 = first rest;
    }

    construction shared_determiner_nominal: NounPhrase {
        bind CoordinatedNominalPhrase {
            determiner: hole Determiner,
            first: hole box NominalPhrase,
            rest: seq nominal_phrase_member,
            complements: seq nominal_complement,
        }
        require rest.len() >= 1;
        require rest.nonfinal.comma in [Present];
        require rest.nonfinal.conjunction.is_none();
        require any(rest.len() >= 2, rest.last.comma in [Absent]);
        require rest.last.conjunction.is_some();
        // The typed mapping is live, but generated chart sequence production
        // lands with the lowering backend. Keep the inactive family honest
        // about its present chart capability until that slice removes this.
        require complements.len() == 0;
        derive first = noun_phrase_coordination(determiner, first, rest);
        witness oxford = stored rest.last.comma;
        // The final atom is always empty but is required for EC021 and makes
        // the whole target shape explicit.
        form shared @ 0 = determiner first rest complements;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&NOUN_COORDINATION_DECLARATION];
