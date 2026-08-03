//! Ad-hoc probe: bisect why one-away `Unparsed` clauses fail to parse.
//!
//! Reads a TSV of `kind \t card \t clause` (newline/tab escaped) and, per
//! clause, reports which fragments the resolver registry can already
//! structure: the whole clause, each sentence alone, and (for triggers) the
//! head with a trivial body vs the body alone as a spell line.
//!
//! Output TSV: `card \t whole \t n_sentences \t sent_ok_bitmap \t head_ok \t
//! body_ok \t first_failing_fragment`

use std::io::BufRead;
use std::io::Write;

use deckmaste_authoring::ManaCost;
use deckmaste_migrations::resolve::resolve_card;
use deckmaste_migrations::todo_card::RawIdent;
use deckmaste_migrations::todo_card::TodoAbility;
use deckmaste_migrations::todo_card::TodoCard;
use deckmaste_migrations::todo_card::TodoCardFace;
use deckmaste_plugin::plugin::Plugin;
use deckmaste_plugin::template::index::TemplateIndex;

fn parses(line: &str, kind: &str, index: &TemplateIndex) -> bool {
    let types = if kind == "Spell" {
        vec![RawIdent("Instant".to_owned())]
    } else {
        vec![RawIdent("Creature".to_owned())]
    };
    let mut card = TodoCard::Normal(TodoCardFace {
        name: "Probe".to_owned(),
        mana_cost: ManaCost::default(),
        color_indicator: vec![],
        supertypes: vec![],
        types,
        subtypes: vec![],
        abilities: vec![TodoAbility::Unparsed(line.to_owned())],
        power: None,
        toughness: None,
        loyalty: None,
        defense: None,
    });
    match resolve_card(&mut card, index) {
        Ok(changed) => changed,
        Err(e) => {
            eprintln!("ERR on {line:?}: {e}");
            false
        }
    }
}

/// Split on ". " sentence boundaries, keeping the period. Naive (quoted
/// abilities etc.) but fine for statistics.
fn sentences(clause: &str) -> Vec<String> {
    let mut out = Vec::new();
    for part in clause.split(". ") {
        let mut s = part.trim().to_owned();
        if !s.ends_with('.') && !s.ends_with('"') {
            s.push('.');
        }
        out.push(s);
    }
    out
}

fn trigger_split(clause: &str) -> Option<(String, String)> {
    // optional ability-word prefix "word — ", then When/Whenever/At head.
    let rest = match clause.split_once(" \u{2014} ") {
        Some((_aw, r))
            if r.starts_with("When") || r.starts_with("At ") || r.starts_with("Whenever") =>
        {
            r
        }
        _ => clause,
    };
    if !(rest.starts_with("When") || rest.starts_with("Whenever") || rest.starts_with("At ")) {
        return None;
    }
    let (head, body) = rest.split_once(", ")?;
    let mut b = body.to_owned();
    if let Some(first) = b.get(..1) {
        b = first.to_uppercase() + &b[1..];
    }
    Some((head.to_owned(), b))
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let tsv = &args[1];
    let plugin_dir = args.get(2).map_or("plugins/wizards", String::as_str);
    let plugin = Plugin::load_with_sibling_prelude(std::path::Path::new(plugin_dir))?;
    let index = TemplateIndex::build(&plugin.macros);
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    for line in std::io::BufReader::new(std::fs::File::open(tsv)?).lines() {
        let line = line?;
        let mut it = line.splitn(3, '\t');
        let (Some(kind), Some(card), Some(esc)) = (it.next(), it.next(), it.next()) else {
            continue;
        };
        let clause = esc
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\\\", "\\");

        let whole = parses(&clause, kind, &index);
        let sents = sentences(&clause);
        let mut bitmap = String::new();
        let mut first_fail = String::new();
        for s in &sents {
            let ok = if sents.len() == 1 { whole } else { parses(s, kind, &index) };
            bitmap.push(if ok { '1' } else { '0' });
            if !ok && first_fail.is_empty() {
                first_fail = s.replace('\n', " / ").replace('\t', " ");
            }
        }
        let (head_ok, body_ok) = match trigger_split(&clause) {
            Some((head, body)) => {
                let h = parses(&format!("{head}, draw a card."), kind, &index);
                let b = parses(&body, "Spell", &index);
                (h.to_string(), b.to_string())
            }
            None => ("-".to_owned(), "-".to_owned()),
        };
        writeln!(
            out,
            "{card}\t{whole}\t{}\t{bitmap}\t{head_ok}\t{body_ok}\t{first_fail}",
            sents.len()
        )?;
    }
    Ok(())
}
