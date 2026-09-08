//! Inspect the generated slice with an explicit materialization limit.

use clap::Parser;
use clap::ValueEnum;
use deckmaste_english_v3::parse;
use deckmaste_english_v3::slice;
use deckmaste_english_v3::slice::grammar;

#[derive(Clone, Copy, ValueEnum)]
enum Category {
    Nominal,
    NounPhrase,
    Clause,
    Sentence,
    Document,
}

impl From<Category> for grammar::Category {
    fn from(category: Category) -> Self {
        match category {
            Category::Nominal => Self::Nominal,
            Category::NounPhrase => Self::NounPhrase,
            Category::Clause => Self::Clause,
            Category::Sentence => Self::Sentence,
            Category::Document => Self::Document,
        }
    }
}

#[derive(Parser)]
struct Args {
    text: String,
    #[arg(long, value_enum, default_value = "document")]
    category: Category,
    /// Maximum distinct Readings requested; zero inspects recognition only.
    #[arg(long, default_value_t = 4)]
    readings: usize,
    /// Print shared nodes, families and lexical leaves (never expanded paths).
    #[arg(long)]
    packed: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let lexicon = slice::lexicon()?;
    let grammar = grammar::Grammar::default();
    let started = std::time::Instant::now();
    let input = lexicon.analyze(&args.text);
    let lexical_time = started.elapsed();
    println!("lexical alternatives: {:#?}", input.matches);
    println!("vocabulary gaps: {:?}", input.unknown_words());
    let started = std::time::Instant::now();
    let forest = parse(&grammar, &lexicon, &input, &args.category.into())?;
    println!(
        "lexical analysis: {lexical_time:?}; recognition: {:?}; {:?}",
        started.elapsed(),
        forest.metrics()
    );
    println!("admitted root summaries: {}", forest.roots().len());
    if args.packed {
        forest.write_packed(&mut std::io::stdout().lock())?;
    }
    let mut readings = forest.readings(&grammar);
    let mut exhausted = false;
    for index in 0..args.readings {
        let Some(value) = readings.next() else {
            exhausted = true;
            break;
        };
        let grammar::Value::Reading(value) = value? else {
            return Err("internal error: non-Reading at a public root".into());
        };
        println!("Reading {}: {value:#?}", index + 1);
        println!("realization: {:?}", value.realize(&lexicon)?);
    }
    println!("materialization: {:?}", readings.metrics());
    println!(
        "enumeration: {}",
        if exhausted {
            "exhausted"
        } else {
            "limit reached; remaining Readings not counted"
        }
    );
    Ok(())
}
