//! The seed lexical environment: about fifteen open-class words, the closed
//! class they need, and their Subcategorization Frames.

use deckmaste_lexical::Binding;
use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::FormDeclaration;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceStructure;
use deckmaste_lexical::WordForm;
use deckmaste_syntax::FrameCatalog;

pub const FRAMES: &str = include_str!("frames.ron");

/// Open-class lemmas of the seed, which the grammar data file must never name.
pub const OPEN_CLASS: [&str; 20] = [
    "attack",
    "destroy",
    "exile",
    "control",
    "put",
    "search",
    "enter",
    "leave",
    "untap",
    "get",
    "card",
    "creature",
    "hand",
    "library",
    "battlefield",
    "permanent",
    "player",
    "counter",
    "untap step",
    "flying",
];

fn source(kind: SourceKind) -> Source {
    Source {
        kind,
        path: "tests/seed".to_owned(),
        owner: "seed".to_owned(),
    }
}

/// Replaces the surfaces of every slot of `form` matching `person` and
/// `number` (`None` matches any).
fn set(
    lexeme: &mut Lexeme,
    form: WordForm,
    person: Option<Person>,
    number: Option<Number>,
    surface: &str,
) {
    for declaration in &mut lexeme.forms {
        if declaration.form == form
            && person.is_none_or(|p| declaration.features.person == Some(p))
            && number.is_none_or(|n| declaration.features.number == Some(n))
        {
            declaration.surfaces = Some(vec![surface.to_owned()]);
        }
    }
}

fn verb(lemma: &str, kind: SourceKind) -> Lexeme {
    Lexeme::verb(format!("verb:{lemma}"), lemma, source(kind))
}

fn invariant(prefix: &str, lemma: &str, category: Category, kind: SourceKind) -> Lexeme {
    Lexeme::invariant(format!("{prefix}:{lemma}"), lemma, category, source(kind))
}

fn noun(lemma: &str, kind: SourceKind) -> Lexeme {
    let mut lexeme = Lexeme::noun(
        format!("noun:{lemma}"),
        lemma,
        vec![Countability::Count],
        source(kind),
    );
    if lemma.contains(' ') {
        lexeme.surface_structure = SurfaceStructure::Multiword;
    }
    lexeme
}

fn pronoun(lemma: &str, person: Person, number: Option<Number>) -> Lexeme {
    let mut lexeme = invariant("pronoun", lemma, Category::Pronoun, SourceKind::Core);
    lexeme.forms = vec![FormDeclaration {
        form: WordForm::Invariant,
        features: FeatureBundle {
            person: Some(person),
            number,
            ..FeatureBundle::default()
        },
        surfaces: None,
    }];
    lexeme
}

#[must_use]
pub fn lexicon() -> Lexicon {
    use Number::Singular;
    use Person::{First, Second, Third};
    use SourceKind::{Catalog, Core, Keyword};
    use WordForm::{GerundParticiple, PastParticiple, Present, Preterite};

    let mut put = verb("put", Core);
    set(&mut put, Preterite, None, None, "put");
    set(&mut put, PastParticiple, None, None, "put");
    set(&mut put, GerundParticiple, None, None, "putting");
    let mut control = verb("control", Core);
    set(&mut control, Preterite, None, None, "controlled");
    set(&mut control, PastParticiple, None, None, "controlled");
    set(&mut control, GerundParticiple, None, None, "controlling");
    let mut untap = verb("untap", Catalog);
    set(&mut untap, Preterite, None, None, "untapped");
    set(&mut untap, PastParticiple, None, None, "untapped");
    set(&mut untap, GerundParticiple, None, None, "untapping");
    let mut leave = verb("leave", Core);
    set(&mut leave, Preterite, None, None, "left");
    set(&mut leave, PastParticiple, None, None, "left");
    let mut get = verb("get", Core);
    set(&mut get, Preterite, None, None, "got");
    set(&mut get, PastParticiple, None, None, "got");
    set(&mut get, GerundParticiple, None, None, "getting");
    let mut have = verb("have", Core);
    set(&mut have, Present, Some(Third), Some(Singular), "has");
    set(&mut have, Preterite, None, None, "had");
    set(&mut have, PastParticiple, None, None, "had");
    let mut be = verb("be", Core);
    set(&mut be, Present, None, None, "are");
    set(&mut be, Present, Some(First), Some(Singular), "am");
    set(&mut be, Present, Some(Third), Some(Singular), "is");
    set(&mut be, Preterite, None, None, "were");
    set(&mut be, Preterite, Some(First), Some(Singular), "was");
    set(&mut be, Preterite, Some(Third), Some(Singular), "was");
    set(&mut be, GerundParticiple, None, None, "being");
    set(&mut be, PastParticiple, None, None, "been");
    let mut genitive = invariant("clitic", "genitive", Category::Clitic, Core);
    genitive.forms[0].surfaces = Some(vec!["'s".to_owned()]);
    genitive.binding = Binding::Suffix;

    let mut lexemes = vec![
        put,
        control,
        untap,
        leave,
        get,
        have,
        be,
        genitive,
        verb("attack", Core),
        verb("enter", Core),
        // Keyword actions [CR#701.1] are catalog vocabulary.
        verb("destroy", Catalog),
        verb("exile", Catalog),
        verb("search", Catalog),
        noun("creature", Catalog),
        noun("untap step", Catalog),
        noun("card", Core),
        noun("hand", Core),
        noun("library", Core),
        noun("battlefield", Core),
        noun("permanent", Core),
        noun("player", Core),
        noun("counter", Core),
        invariant("keyword", "flying", Category::Keyword, Keyword),
        invariant("adjective", "other", Category::Adjective, Core),
        pronoun("you", Second, None),
        pronoun("it", Third, Some(Singular)),
    ];
    for word in ["the", "a", "your", "each", "all", "target", "any"] {
        lexemes.push(invariant("det", word, Category::Determinative, Core));
    }
    for word in [
        "in", "into", "on", "onto", "for", "with", "from", "during", "by",
    ] {
        lexemes.push(invariant("prep", word, Category::Preposition, Core));
    }
    for word in ["and", "or"] {
        lexemes.push(invariant("coord", word, Category::Coordinator, Core));
    }
    Lexicon::new(lexemes).expect("seed lexicon")
}

#[must_use]
pub fn frames() -> FrameCatalog {
    FrameCatalog::from_ron_str(FRAMES).expect("seed frames")
}
