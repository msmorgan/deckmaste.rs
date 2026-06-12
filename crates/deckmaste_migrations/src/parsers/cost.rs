//! The shared cost-clause grammar: a ", "-separated cost clause -> a list of
//! `CostComponent` RON strings. Two positions speak it: the activated-ability
//! frame's pre-colon clause ("{1}{B}, {T}, Sacrifice ~: …" [CR#602.1a]) and
//! the keyword parser's cost arguments ("Ward {2}", "Ward—Pay 3 life."
//! [CR#702.21a]). Components: mana runs, the tap/untap symbols, sacrifice-self,
//! pay-life, discard.

use deckmaste_core::ManaCost;
use deckmaste_core::ManaSymbol;

use crate::parsers::effect;
use crate::ron_output::ron_options;

/// Whether `{X}` may appear in a mana component. A keyword cost argument
/// records the printed cost faithfully — what X equals is the payer's or the
/// card's business at pay time ([CR#107.3a,702.21b]), behind the macro seam.
/// A variable ACTIVATION cost still declines: announcing X ([CR#601.2b]) has
/// no representation in the `Activated` frame yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableMana {
    Allow,
    Decline,
}

/// Parses a cost clause: every ", "-separated component must be recognized,
/// or the whole cost declines.
pub(crate) fn parse_cost(
    clause: &str,
    variable: VariableMana,
) -> anyhow::Result<Option<Vec<String>>> {
    let mut components = Vec::new();
    for part in clause.split(", ") {
        let Some(component) = cost_component(part, variable)? else {
            return Ok(None);
        };
        components.push(component);
    }
    Ok(Some(components))
}

/// One cost component -> its `CostComponent` RON, or `None`. The tap/untap
/// symbols [CR#107.5,107.6] and sacrifice-self (the `SacrificeThis` macro)
/// are exact matches; the rest are shape productions.
fn cost_component(text: &str, variable: VariableMana) -> anyhow::Result<Option<String>> {
    Ok(match text {
        "{T}" => Some("Tap".to_owned()),
        "{Q}" => Some("Untap".to_owned()),
        "Sacrifice ~" => Some("SacrificeThis".to_owned()),
        _ => match pay_life(text).or_else(|| discard(text)) {
            Some(component) => Some(component),
            None => mana_component(text, variable)?,
        },
    })
}

/// `Pay N life` -> `Do(LoseLife(N))`: paying life is losing that much life
/// [CR#119.4].
fn pay_life(text: &str) -> Option<String> {
    let n = effect::number_word(text.strip_prefix("Pay ")?.strip_suffix(" life")?)?;
    Some(format!("Do(LoseLife({n}))"))
}

/// `Discard a card` / `Discard N cards` -> `Do(Discard(N))` (cards of the
/// payer's choice). Riders ("at random", "your hand") decline.
fn discard(text: &str) -> Option<String> {
    let rest = text.strip_prefix("Discard ")?;
    let count = rest
        .strip_suffix(" cards")
        .or_else(|| rest.strip_suffix(" card"))?;
    let n = effect::number_word(count)?;
    Some(format!("Do(Discard({n}))"))
}

/// A run of mana symbols -> `Mana([...])`. Declines on non-mana text, the
/// empty cost, and — under [`VariableMana::Decline`] — `{X}`.
fn mana_component(text: &str, variable: VariableMana) -> anyhow::Result<Option<String>> {
    let Ok(mana) = text.parse::<ManaCost>() else {
        return Ok(None);
    };
    if mana.is_empty() {
        return Ok(None);
    }
    if variable == VariableMana::Decline && mana.iter().any(|s| matches!(s, ManaSymbol::Variable)) {
        return Ok(None);
    }
    Ok(Some(format!("Mana({})", ron_options().to_string(&mana)?)))
}
