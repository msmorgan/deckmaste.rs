//! The real noun/nominal coordination declarations in bind mode: inactive
//! compiler scaffolding related by tests to the handwritten grammar.

use deckmaste_construction_compiler::runtime::GroupData;

use crate::features::Comma;
use crate::features::Conjunction;
use crate::syntax::CoordinatedNominalPhrase;
use crate::syntax::CoordinatedNounPhrase;
use crate::syntax::Determiner;
use crate::syntax::NominalComplement;
use crate::syntax::NominalPhrase;
use crate::syntax::NominalPhraseCoordination;
use crate::syntax::NounPhrase;
use crate::syntax::NounPhraseCoordination;

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

    // Opaque because NominalComplement is an enum. EC007 permits this only
    // while every referencing sequence is proven empty.
    element nominal_complement bind NominalComplement {}

    construction noun_phrase_coordination: NounPhrase {
        bind CoordinatedNounPhrase {
            first: hole box NounPhrase,
            rest: seq noun_phrase_member,
        }
        require rest.len() >= 1;
        require any(rest.len() >= 2, rest.last.comma in [Absent]);
        require rest.last.conjunction.is_some();
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
        require any(rest.len() >= 2, rest.last.comma in [Absent]);
        require rest.last.conjunction.is_some();
        require complements.len() == 0;
        witness oxford = stored rest.last.comma;
        // The final atom is always empty but is required for EC021 and makes
        // the whole target shape explicit.
        form shared @ 0 = determiner first rest complements;
    }
}

pub(crate) static GROUPS: &[&GroupData] = &[&NOUN_COORDINATION_DECLARATION];
