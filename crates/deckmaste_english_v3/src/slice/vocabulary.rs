use deckmaste_lexical::Case;
use deckmaste_lexical::Category;
use deckmaste_lexical::Countability;
use deckmaste_lexical::FeatureBundle;
use deckmaste_lexical::Frame;
use deckmaste_lexical::FrameItem;
use deckmaste_lexical::FrameSlot;
use deckmaste_lexical::Lexeme;
use deckmaste_lexical::LexicalError;
use deckmaste_lexical::Lexicon;
use deckmaste_lexical::Number;
use deckmaste_lexical::Person;
use deckmaste_lexical::Relation;
use deckmaste_lexical::Source;
use deckmaste_lexical::SourceKind;
use deckmaste_lexical::SurfaceStructure;
use deckmaste_lexical::WordForm;

fn source(id: &str) -> Source {
    Source {
        kind: SourceKind::Core,
        path: "crates/deckmaste_english_v3/src/slice/vocabulary.rs".into(),
        owner: id.into(),
    }
}

fn frame(kind: &str, category: &str) -> Frame {
    Frame {
        kind: kind.into(),
        items: vec![FrameItem::Argument(FrameSlot {
            relation: Relation::Complement,
            category: category.into(),
        })],
    }
}

/// The slice's declared vocabulary, using the lexical engine's shared
/// morphology defaults and explicit irregular replacements.
///
/// # Errors
/// Reports an invalid lexical declaration or morphology override.
pub fn lexicon() -> Result<Lexicon, LexicalError> {
    let mut words = Vec::new();
    for (id, lemma) in [
        ("noun:cast", "cast"),
        ("noun:one", "one"),
        ("noun:spell", "spell"),
        ("noun:counter", "counter"),
        ("noun:strike", "strike"),
        ("noun:first-strike", "first strike"),
    ] {
        let mut noun = Lexeme::noun(id, lemma, vec![Countability::Count], source(id));
        if lemma.contains(' ') {
            noun.surface_structure = SurfaceStructure::Multiword;
        }
        words.push(noun);
    }
    words.push(Lexeme::invariant(
        "adjective:first",
        "first",
        Category::Adjective,
        source("adjective:first"),
    ));
    for (id, lemma, number) in [
        ("determinative:one", "one", Number::Singular),
        ("determinative:two", "two", Number::Plural),
    ] {
        let mut word = Lexeme::invariant(id, lemma, Category::Determinative, source(id));
        word.forms[0].features.number = Some(number);
        words.push(word);
    }
    for (id, lemma, person, numbers) in [
        (
            "pronoun:you",
            "you",
            Person::Second,
            vec![Number::Singular, Number::Plural],
        ),
        ("pronoun:it", "it", Person::Third, vec![Number::Singular]),
        ("pronoun:they", "they", Person::Third, vec![Number::Plural]),
    ] {
        let mut word = Lexeme::invariant(id, lemma, Category::Pronoun, source(id));
        let form = word.forms[0].clone();
        word.forms = numbers
            .into_iter()
            .map(|number| {
                let mut form = form.clone();
                form.features = FeatureBundle {
                    number: Some(number),
                    person: Some(person),
                    case: Some(Case::Nominative),
                    ..FeatureBundle::default()
                };
                form
            })
            .collect();
        words.push(word);
    }
    for (id, lemma, past) in [
        ("verb:cast", "cast", vec!["cast"]),
        ("verb:draw", "draw", vec!["drew"]),
        ("verb:learn", "learn", vec!["learned", "learnt"]),
    ] {
        let mut word = Lexeme::verb(id, lemma, source(id));
        word.properties
            .frames
            .push(frame("transitive", "NounPhrase"));
        for form in &mut word.forms {
            if form.form == WordForm::Preterite {
                form.surfaces = Some(past.iter().map(|s| (*s).into()).collect());
            }
            if form.form == WordForm::PastParticiple {
                form.surfaces = Some(match id {
                    "verb:draw" => vec!["drawn".into()],
                    _ => past.iter().map(|s| (*s).into()).collect(),
                });
            }
        }
        words.push(word);
    }
    let mut walk = Lexeme::verb("verb:walk", "walk", source("verb:walk"));
    walk.properties.frames.push(Frame {
        kind: "intransitive".into(),
        items: vec![],
    });
    words.push(walk);
    let mut with = Lexeme::invariant(
        "preposition:with",
        "with",
        Category::Preposition,
        source("preposition:with"),
    );
    with.properties
        .frames
        .push(frame("prepositional", "BasicNounPhrase"));
    words.push(with);
    Lexicon::new(words)
}
