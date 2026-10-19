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
    lexeme
        .properties
        .features
        .insert("NounDistribution".into(), "Ordinary".into());
    lexeme
        .properties
        .features
        .insert("NounKind".into(), "Ordinary".into());
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

/// Fused heads and comparative and superlative determinatives the grammar
/// names.
const DEPENDENT_HEADS: [&str; 14] = [
    "both",
    "either",
    "neither",
    "everything",
    "many",
    "much",
    "anywhere",
    "this",
    "none",
    "less",
    "fewer",
    "most",
    "least",
    "fewest",
];

/// *More* is two lexemes: the comparative of *many* and of *much*.
fn more() -> Vec<Lexeme> {
    ["count", "mass"]
        .into_iter()
        .map(|sense| {
            let mut det = Lexeme::invariant(
                format!("det:more.{sense}"),
                "more",
                Category::Determinative,
                source(SourceKind::Core),
            );
            det.properties
                .features
                .insert("DeterminativeDistribution".into(), "Dependent".into());
            det
        })
        .collect()
}

#[must_use]
pub fn lexicon() -> Lexicon {
    use Number::Singular;
    use Person::First;
    use Person::Second;
    use Person::Third;
    use SourceKind::Catalog;
    use SourceKind::Core;
    use SourceKind::Keyword;
    use WordForm::GerundParticiple;
    use WordForm::PastParticiple;
    use WordForm::Present;
    use WordForm::Preterite;

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
        // Oracle "you" is always one player.
        pronoun("you", Second, Some(Singular)),
        pronoun("it", Third, Some(Singular)),
    ];
    for word in [
        "the", "a", "your", "each", "all", "target", "any", "that", "another", "every",
    ] {
        let mut det = invariant("det", word, Category::Determinative, Core);
        det.properties
            .features
            .insert("DeterminativeDistribution".into(), "Dependent".into());
        lexemes.push(det);
    }
    for word in [
        "in", "into", "on", "onto", "for", "with", "from", "during", "by", "up", "of", "than",
        "plus", "minus", "as",
    ] {
        lexemes.push(invariant("prep", word, Category::Preposition, Core));
    }
    for word in ["and", "or", "and/or", "nor", "but"] {
        lexemes.push(invariant("coord", word, Category::Coordinator, Core));
    }
    for word in DEPENDENT_HEADS {
        let mut det = invariant("det", word, Category::Determinative, Core);
        det.properties
            .features
            .insert("DeterminativeDistribution".into(), "Dependent".into());
        lexemes.push(det);
    }
    lexemes.extend(more());
    // Only interrogative and fused relative: never a plain determiner.
    for word in ["whichever", "which"] {
        let mut det = invariant("det", word, Category::Determinative, Core);
        det.properties
            .features
            .insert("DeterminativeDistribution".into(), "Independent".into());
        lexemes.push(det);
    }
    lexemes.push(invariant("sub", "to", Category::Subordinator, Core));
    // Relative *that* is a subordinator; *which* and *who* are relative
    // pronouns.
    lexemes.push(invariant("sub", "that", Category::Subordinator, Core));
    lexemes.push(invariant("sub", "whether", Category::Subordinator, Core));
    for lemma in ["which", "who"] {
        let mut relative = pronoun(lemma, Third, None);
        relative
            .properties
            .features
            .insert("PronounKind".into(), "Relative".into());
        lexemes.push(relative);
    }
    for (word, third, preterite) in [
        ("'ve", "'s", None),
        ("haven't", "hasn't", Some("hadn't")),
        ("'re", "'s", None),
        ("aren't", "isn't", Some("weren't")),
    ] {
        let mut auxiliary = verb(word, Core);
        auxiliary
            .forms
            .retain(|f| f.form == Present || (preterite.is_some() && f.form == Preterite));
        set(&mut auxiliary, Present, None, None, word);
        set(&mut auxiliary, Present, Some(Third), Some(Singular), third);
        match preterite {
            Some(preterite) => set(&mut auxiliary, Preterite, None, None, preterite),
            None => auxiliary.binding = Binding::Suffix,
        }
        lexemes.push(auxiliary);
    }
    // Auxiliary verbs are a lexical class of their own.
    for lexeme in &mut lexemes {
        if lexeme.category == Category::Verb
            && ["be", "have", "'ve", "haven't", "'re", "aren't"].contains(&lexeme.lemma.as_str())
        {
            lexeme
                .properties
                .features
                .insert("VerbKind".into(), "Auxiliary".into());
        }
    }
    Lexicon::new(lexemes).expect("seed lexicon")
}

#[must_use]
pub fn frames() -> FrameCatalog {
    let mut frames = FrameCatalog::from_ron_str(FRAMES).expect("seed frames");
    frames.parameters.insert(
        "keyword:flying".into(),
        vec![vec![deckmaste_syntax::frames::ParameterPart::Head]],
    );
    frames
}
