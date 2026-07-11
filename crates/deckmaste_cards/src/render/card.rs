//! The card-frame fields: type line, mana cost, power/toughness.

use std::fmt::Write as _;

use deckmaste_core::Color;
use deckmaste_core::ColorOrColorless;
use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;
use deckmaste_core::SimpleManaSymbol;
use deckmaste_core::StatValue;

use super::CardView;

/// "Legendary Creature — Phyrexian Praetor" / "Instant" / "Enchantment — Aura".
pub(super) fn type_line(view: &CardView) -> String {
    let mut head: Vec<&str> = Vec::new();
    for s in view.supertypes {
        head.push(supertype_str(*s));
    }
    for t in view.types {
        head.push(type_str(t));
    }
    let mut line = head.join(" ");
    if !view.subtypes.is_empty() {
        let subs: Vec<&str> = view.subtypes.iter().map(|s| s.name.as_str()).collect();
        line.push_str(" — ");
        line.push_str(&subs.join(" "));
    }
    line
}

/// "{1}{W}{W}" — empty when there is no cost.
pub(super) fn mana_cost(cost: Option<&ManaCost>) -> String {
    let Some(cost) = cost else { return String::new() };
    let mut s = String::new();
    for sym in cost.iter() {
        push_symbol(&mut s, sym);
    }
    s
}

fn push_symbol(s: &mut String, sym: &ManaSymbol) {
    // The [CR#107.4] printed forms, family by family — the closed symbol
    // set renders totally.
    match sym {
        ManaSymbol::Simple(SimpleManaSymbol::Generic(n)) => {
            write!(s, "{{{n}}}").unwrap();
        }
        ManaSymbol::Simple(SimpleManaSymbol::Specific(c)) => {
            write!(s, "{{{}}}", color_letter(*c)).unwrap();
        }
        ManaSymbol::Variable => s.push_str("{X}"),
        ManaSymbol::Snow => s.push_str("{S}"),
        // A hybrid renders its left component ({2/W}, {C/W}, {W/U}) then its
        // right color.
        ManaSymbol::Hybrid(left, right) => {
            let left_code = match left {
                SimpleManaSymbol::Generic(n) => n.to_string(),
                SimpleManaSymbol::Specific(c) => color_letter(*c).to_owned(),
            };
            write!(s, "{{{left_code}/{}}}", right.code()).unwrap();
        }
        // A Phyrexian renders `{W/P}`, or `{G/U/P}` for a hybrid Phyrexian.
        ManaSymbol::Phyrexian(c, None) => write!(s, "{{{}/P}}", c.code()).unwrap(),
        ManaSymbol::Phyrexian(c, Some(o)) => {
            write!(s, "{{{}/{}/P}}", c.code(), o.code()).unwrap();
        }
    }
}

pub(super) fn color_letter(c: ColorOrColorless) -> &'static str {
    match c {
        ColorOrColorless::Colorless => "C",
        ColorOrColorless::Color(Color::White) => "W",
        ColorOrColorless::Color(Color::Blue) => "U",
        ColorOrColorless::Color(Color::Black) => "B",
        ColorOrColorless::Color(Color::Red) => "R",
        ColorOrColorless::Color(Color::Green) => "G",
    }
}

/// "2/2", "0/8", "*/*"; None when neither stat is set.
pub(super) fn pt(view: &CardView) -> Option<String> {
    match (view.power, view.toughness) {
        (None, None) => None,
        (p, t) => Some(format!("{}/{}", stat(p), stat(t))),
    }
}

fn stat(v: Option<&StatValue>) -> String {
    match v {
        Some(StatValue::Number(n)) => n.to_string(),
        Some(_) | None => "*".to_string(),
    }
}

/// The printed type line word for an expanded [`TypeDef`] — its open name.
/// Total and open-safe: any plugin-declared type renders as its own name.
pub(super) fn type_str(t: &deckmaste_core::TypeDef) -> &'static str {
    t.name.as_str()
}

pub(super) fn supertype_str(s: deckmaste_core::Supertype) -> &'static str {
    use deckmaste_core::Supertype::Basic;
    use deckmaste_core::Supertype::Legendary;
    use deckmaste_core::Supertype::Ongoing;
    use deckmaste_core::Supertype::Snow;
    use deckmaste_core::Supertype::World;
    match s {
        Basic => "Basic",
        Legendary => "Legendary",
        Ongoing => "Ongoing",
        Snow => "Snow",
        World => "World",
    }
}
