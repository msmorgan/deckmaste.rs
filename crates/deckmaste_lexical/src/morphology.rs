use crate::Category;
use crate::FeatureBundle;
use crate::Finiteness;
use crate::Lexeme;
use crate::Number;
use crate::Person;
use crate::Tense;
use crate::WordForm;

pub(crate) fn applicable(category: Category, form: WordForm, features: &FeatureBundle) -> bool {
    let nominal =
        features.person.is_none() && features.tense.is_none() && features.finiteness.is_none();
    match form {
        WordForm::Invariant => match category {
            Category::Verb => false,
            Category::Pronoun => features.tense.is_none() && features.finiteness.is_none(),
            Category::Noun | Category::Determinative => nominal,
            _ => features == &FeatureBundle::default(),
        },
        WordForm::Singular | WordForm::Plural => {
            category == Category::Noun
                && nominal
                && features.number
                    == Some(if form == WordForm::Singular {
                        Number::Singular
                    } else {
                        Number::Plural
                    })
        }
        WordForm::Plain | WordForm::GerundParticiple | WordForm::PastParticiple => {
            category == Category::Verb
                && features.finiteness == Some(Finiteness::Nonfinite)
                && features.tense.is_none()
                && features.person.is_none()
                && features.number.is_none()
                && features.case.is_none()
        }
        WordForm::Present | WordForm::Preterite => {
            category == Category::Verb
                && features.finiteness == Some(Finiteness::Finite)
                && features.person.is_some()
                && features.number.is_some()
                && features.case.is_none()
                && features.tense
                    == Some(if form == WordForm::Present { Tense::Present } else { Tense::Past })
        }
    }
}

pub(crate) fn default_surface(lexeme: &Lexeme, form: WordForm, features: &FeatureBundle) -> String {
    let lemma = &lexeme.lemma;
    match form {
        WordForm::Plural => suffix_s(lemma),
        WordForm::Present
            if features.person == Some(Person::Third)
                && features.number == Some(Number::Singular) =>
        {
            suffix_s(lemma)
        }
        WordForm::Preterite | WordForm::PastParticiple => past(lemma),
        WordForm::GerundParticiple => gerund(lemma),
        _ => lemma.clone(),
    }
}

fn consonant_y(stem: &str) -> Option<&str> {
    let base = stem.strip_suffix('y')?;
    let last = base.chars().next_back()?;
    (last.is_ascii_alphabetic() && !matches!(last, 'a' | 'e' | 'i' | 'o' | 'u')).then_some(base)
}

fn suffix_s(stem: &str) -> String {
    if let Some(base) = consonant_y(stem) {
        return format!("{base}ies");
    }
    if ["s", "x", "z", "ch", "sh"]
        .iter()
        .any(|ending| stem.ends_with(ending))
    {
        format!("{stem}es")
    } else {
        format!("{stem}s")
    }
}

/// The regular past-participle spelling, also used by explicitly declared
/// participial-adjective derivations. Irregular spellings replace this rule.
#[must_use]
pub fn default_participle(stem: &str) -> String {
    if let Some(base) = consonant_y(stem) {
        return format!("{base}ied");
    }
    if stem.ends_with('e') { format!("{stem}d") } else { format!("{stem}ed") }
}

fn past(stem: &str) -> String {
    default_participle(stem)
}

fn gerund(stem: &str) -> String {
    if let Some(base) = stem.strip_suffix("ie") {
        return format!("{base}ying");
    }
    if let Some(base) = stem.strip_suffix('e')
        && !["ee", "oe", "ye"]
            .iter()
            .any(|ending| stem.ends_with(ending))
    {
        return format!("{base}ing");
    }
    format!("{stem}ing")
}
