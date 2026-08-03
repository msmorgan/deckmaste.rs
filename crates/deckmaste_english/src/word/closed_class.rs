use super::Number;
use super::Person;
use super::Vocabulary;
/// Compatibility name for the inherent-realization gender feature.
pub use crate::features::Gender;
/// Compatibility name for the inherent-realization pronoun-case feature.
pub use crate::features::PronounCase;
/// Compatibility name for the inherent-realization pronoun-class feature.
pub use crate::features::PronounClass as Pronoun;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct PronounInstance {
    pub pronoun: Pronoun,
    pub case: PronounCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum Auxiliary {
    Can,
    Could,
    Do,
    May,
    Might,
    Must,
    Shall,
    Should,
    Will,
    Would,
    Be,
    Have,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub enum AuxiliaryInflection {
    Base,
    Present {
        person: Person,
        number: Number,
    },
    Past {
        person: Person,
        number: Number,
    },
    /// The past-subjunctive `were`, carried with no person/number: licensed
    /// only under `as though` (see the `Features::Subordinator`/`subjunctive`
    /// licensing gate in `grammar`). Never agrees like `Past`.
    PastSubjunctive,
    PresentParticiple,
    PastParticiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
pub struct AuxiliaryInstance {
    pub auxiliary: Auxiliary,
    pub inflection: AuxiliaryInflection,
    pub contracted_negation: bool,
}

impl Vocabulary {
    #[must_use]
    pub const fn render_auxiliary(self, auxiliary: AuxiliaryInstance) -> Option<&'static str> {
        render_auxiliary(auxiliary)
    }

    #[must_use]
    pub const fn render_pronoun(self, pronoun: PronounInstance) -> Option<&'static str> {
        match (pronoun.pronoun, pronoun.case) {
            (Pronoun::You, PronounCase::Subject | PronounCase::Object) => Some("you"),
            (Pronoun::It(Gender::Masculine), PronounCase::Subject) => Some("he"),
            (Pronoun::It(Gender::Masculine), PronounCase::Object) => Some("him"),
            (Pronoun::It(Gender::Feminine), PronounCase::Subject) => Some("she"),
            (Pronoun::It(Gender::Feminine), PronounCase::Object) => Some("her"),
            (Pronoun::It(Gender::Neuter), PronounCase::Subject | PronounCase::Object) => Some("it"),
            (Pronoun::They, PronounCase::Subject) => Some("they"),
            (Pronoun::They, PronounCase::Object) => Some("them"),
            (Pronoun::EachOther, PronounCase::Object) => Some("each other"),
            (Pronoun::Itself, PronounCase::Object) => Some("itself"),
            (Pronoun::Himself, PronounCase::Object) => Some("himself"),
            (Pronoun::YoursAbsolute, PronounCase::Object) => Some("yours"),
            (
                Pronoun::EachOther | Pronoun::Itself | Pronoun::Himself | Pronoun::YoursAbsolute,
                PronounCase::Subject,
            ) => None,
        }
    }

    #[must_use]
    pub fn render_possessive_pronoun(self, pronoun: Pronoun) -> Option<&'static str> {
        pronoun.possessive_spelling()
    }
}

pub(super) fn auxiliary_instances() -> impl Iterator<Item = AuxiliaryInstance> {
    const AUXILIARIES: [Auxiliary; 12] = [
        Auxiliary::Can,
        Auxiliary::Could,
        Auxiliary::Do,
        Auxiliary::May,
        Auxiliary::Might,
        Auxiliary::Must,
        Auxiliary::Shall,
        Auxiliary::Should,
        Auxiliary::Will,
        Auxiliary::Would,
        Auxiliary::Be,
        Auxiliary::Have,
    ];
    const INFLECTIONS: [AuxiliaryInflection; 12] = [
        AuxiliaryInflection::Base,
        AuxiliaryInflection::Present {
            person: Person::Second,
            number: Number::Singular,
        },
        AuxiliaryInflection::Present {
            person: Person::Second,
            number: Number::Plural,
        },
        AuxiliaryInflection::Present {
            person: Person::Third,
            number: Number::Singular,
        },
        AuxiliaryInflection::Present {
            person: Person::Third,
            number: Number::Plural,
        },
        AuxiliaryInflection::Past {
            person: Person::Second,
            number: Number::Singular,
        },
        AuxiliaryInflection::Past {
            person: Person::Second,
            number: Number::Plural,
        },
        AuxiliaryInflection::Past {
            person: Person::Third,
            number: Number::Singular,
        },
        AuxiliaryInflection::Past {
            person: Person::Third,
            number: Number::Plural,
        },
        AuxiliaryInflection::PastSubjunctive,
        AuxiliaryInflection::PresentParticiple,
        AuxiliaryInflection::PastParticiple,
    ];

    AUXILIARIES.into_iter().flat_map(|auxiliary| {
        INFLECTIONS.into_iter().flat_map(move |inflection| {
            [false, true].map(move |contracted_negation| AuxiliaryInstance {
                auxiliary,
                inflection,
                contracted_negation,
            })
        })
    })
}

pub(super) const fn render_auxiliary(instance: AuxiliaryInstance) -> Option<&'static str> {
    use Auxiliary as A;
    use AuxiliaryInflection as I;
    use Number as N;
    use Person as P;

    if instance.contracted_negation {
        return match (instance.auxiliary, instance.inflection) {
            (A::Can, I::Base) => Some("can't"),
            (A::Could, I::Base) => Some("couldn't"),
            (A::Will, I::Base) => Some("won't"),
            (
                A::Do,
                I::Base
                | I::Present {
                    person: P::Second, ..
                }
                | I::Present {
                    person: P::Third,
                    number: N::Plural,
                },
            ) => Some("don't"),
            (
                A::Do,
                I::Present {
                    person: P::Third,
                    number: N::Singular,
                },
            ) => Some("doesn't"),
            (A::Do, I::Past { .. }) => Some("didn't"),
            (
                A::Be,
                I::Present {
                    person: P::Third,
                    number: N::Singular,
                },
            ) => Some("isn't"),
            (A::Be, I::Present { .. }) => Some("aren't"),
            (
                A::Be,
                I::Past {
                    person: P::Third,
                    number: N::Singular,
                },
            ) => Some("wasn't"),
            (A::Be, I::Past { .. } | I::PastSubjunctive) => Some("weren't"),
            (
                A::Have,
                I::Present {
                    person: P::Third,
                    number: N::Singular,
                },
            ) => Some("hasn't"),
            (A::Have, I::Base | I::Present { .. }) => Some("haven't"),
            (A::Have, I::Past { .. }) => Some("hadn't"),
            _ => None,
        };
    }

    match (instance.auxiliary, instance.inflection) {
        (A::Can, I::Base) => Some("can"),
        (A::Could, I::Base) => Some("could"),
        (A::May, I::Base) => Some("may"),
        (A::Might, I::Base) => Some("might"),
        (A::Must, I::Base) => Some("must"),
        (A::Shall, I::Base) => Some("shall"),
        (A::Should, I::Base) => Some("should"),
        (A::Will, I::Base) => Some("will"),
        (A::Would, I::Base) => Some("would"),
        (
            A::Do,
            I::Base
            | I::Present {
                person: P::Second, ..
            }
            | I::Present {
                person: P::Third,
                number: N::Plural,
            },
        ) => Some("do"),
        (
            A::Do,
            I::Present {
                person: P::Third,
                number: N::Singular,
            },
        ) => Some("does"),
        (A::Do, I::Past { .. }) => Some("did"),
        (A::Do, I::PresentParticiple) => Some("doing"),
        (A::Do, I::PastParticiple) => Some("done"),
        (A::Be, I::Base) => Some("be"),
        (
            A::Be,
            I::Present {
                person: P::Third,
                number: N::Singular,
            },
        ) => Some("is"),
        (A::Be, I::Present { .. }) => Some("are"),
        (
            A::Be,
            I::Past {
                person: P::Third,
                number: N::Singular,
            },
        ) => Some("was"),
        (A::Be, I::Past { .. } | I::PastSubjunctive) => Some("were"),
        (A::Be, I::PresentParticiple) => Some("being"),
        (A::Be, I::PastParticiple) => Some("been"),
        (
            A::Have,
            I::Base
            | I::Present {
                person: P::Second, ..
            }
            | I::Present {
                person: P::Third,
                number: N::Plural,
            },
        ) => Some("have"),
        (
            A::Have,
            I::Present {
                person: P::Third,
                number: N::Singular,
            },
        ) => Some("has"),
        (A::Have, I::Past { .. } | I::PastParticiple) => Some("had"),
        (A::Have, I::PresentParticiple) => Some("having"),
        _ => None,
    }
}
