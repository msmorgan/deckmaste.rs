use deckmaste_english::syntax::Ability;
use deckmaste_english::syntax::AbilityKind;
use deckmaste_english::syntax::Paragraph;

fn main() {
    let _ = Ability {
        header: None,
        kind: AbilityKind::Paragraph(Paragraph::default()),
    };
}
