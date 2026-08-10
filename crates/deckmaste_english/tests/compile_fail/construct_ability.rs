use deckmaste_english::syntax::Ability;
use deckmaste_english::syntax::AbilityKind;
use deckmaste_english::syntax::Paragraph;

fn main() {
    let _ = Ability {
        ability_word: None,
        flavor_header: None,
        kind: AbilityKind::Paragraph(Paragraph::default()),
    };
}
