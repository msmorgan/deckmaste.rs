//! The shared cost-clause grammar: a ", "-separated cost clause -> a list of
//! `CostComponent` RON strings. Two positions speak it: the activated-ability
//! frame's pre-colon clause ("{1}{B}, {T}, Sacrifice ~: …" [CR#602.1a]) and
//! the keyword parser's cost arguments ("Ward {2}", "Ward—Pay 3 life."
//! [CR#702.21a]). Components: mana runs, the tap/untap symbols, sacrifice-self,
//! chosen-sacrifice (`Sacrifice a creature`), pay-life, discard.

use deckmaste_authoring::ManaCost;
use deckmaste_authoring::ManaSymbol;
use deckmaste_legacy_render::template::index::TemplateIndex;

use crate::parsers::effect;
use crate::parsers::filter;
use crate::ron_output::ron_options;

/// Whether `{X}` may appear in a mana component. A keyword cost argument
/// records the printed cost faithfully — what X equals is announced by the
/// controller ([CR#107.3a]) or stated by the card's own text (ward:
/// determined at resolution, [CR#702.21b]), behind the macro seam either way.
/// A variable ACTIVATION cost likewise records the `{X}` symbol now: the engine
/// announces X onto the activation's announce slot and concretizes the cost
/// (engine-x-costs), so the `Activated` frame carries the printed `{X}`
/// faithfully ([CR#601.2b]). `Decline` remains for any future caller that must
/// reject `{X}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VariableMana {
    Allow,
    Decline,
}

/// Parses a cost clause: every ", "-separated component must be recognized,
/// or the whole cost declines. `index` (when present) routes a component to a
/// slot-bearing `CostComponent` macro via the reverse [`TemplateIndex`] — e.g.
/// the energy-payment `Pay {E}{E}` → `PayEnergy(2)`; callers with no macro
/// context (keyword cost args, mana-ability production costs) pass `None`.
pub(crate) fn parse_cost(
    clause: &str,
    variable: VariableMana,
    index: Option<&TemplateIndex>,
) -> anyhow::Result<Option<Vec<String>>> {
    let mut components = Vec::new();
    for part in clause.split(", ") {
        let Some(component) = cost_component(part, variable, index)? else {
            return Ok(None);
        };
        components.push(component);
    }
    Ok(Some(components))
}

/// One cost component -> its `CostComponent` RON, or `None`. The tap/untap
/// symbols [CR#107.5,107.6] and sacrifice-self (the `SacrificeThis` macro)
/// are exact matches; the rest are shape productions. A component matching no
/// bespoke shape is finally offered to the slot-bearing `CostComponent` macro
/// index (`Pay {E}{E}` → `PayEnergy(2)`), when a macro context is supplied.
fn cost_component(
    text: &str,
    variable: VariableMana,
    index: Option<&TemplateIndex>,
) -> anyhow::Result<Option<String>> {
    Ok(match text {
        "{T}" => Some("Tap".to_owned()),
        "{Q}" => Some("Untap".to_owned()),
        "Sacrifice ~" => Some("SacrificeThis".to_owned()),
        _ => match pay_life(text)
            .or_else(|| discard(text))
            .or_else(|| sacrifice(text))
        {
            Some(component) => Some(component),
            None => match mana_component(text, variable)? {
                Some(mana) => Some(mana),
                None => cost_macro(text, index)?,
            },
        },
    })
}

/// Route a cost component to a slot-bearing `CostComponent` macro via the
/// reverse [`TemplateIndex`]: `Pay {E}{E}` → `PayEnergy(2)`. The count-driven
/// repeat construct folds the `{E}` run into the count itself, so the slot
/// reader is never consulted (it declines unconditionally). `None` when no
/// macro context is supplied or no macro template claims the whole component.
fn cost_macro(text: &str, index: Option<&TemplateIndex>) -> anyhow::Result<Option<String>> {
    let Some(index) = index else {
        return Ok(None);
    };
    let m = index.match_with("CostComponent", text, |_, _| None)?;
    Ok(m.map(|m| m.invocation))
}

/// `Pay N life` -> `Do(ChangeLife(You, Down(N)))`: paying life is losing that
/// much life [CR#119.4].
fn pay_life(text: &str) -> Option<String> {
    let n = effect::number_word(text.strip_prefix("Pay ")?.strip_suffix(" life")?)?;
    Some(format!("Do(ChangeLife(You, Down({n})))"))
}

/// `Discard a card` / `Discard N cards` -> `DiscardCards(N)` — the
/// chosen-from-hand discard composite as a cost ([CR#701.9,601.2b]), the
/// discard sibling of `Sacrifice ~` -> `SacrificeThis`. Riders ("at
/// random", "your hand") decline.
fn discard(text: &str) -> Option<String> {
    let rest = text.strip_prefix("Discard ")?;
    let count = rest
        .strip_suffix(" cards")
        .or_else(|| rest.strip_suffix(" card"))?;
    let n = effect::number_word(count)?;
    Some(format!("DiscardCards({n})"))
}

/// `Sacrifice <subject>` (non-self) -> the cost-side choose-then-pay `With`
/// step ([CR#601.2b]): `With(binder: ChooseOne(filter: <filter>), body:
/// [Do(Sacrifice(You, That(Permanent)))])` for one, or `With(binder:
/// Choose(quantity: Exactly(N), filter: <filter>), body:
/// [Do(Sacrifice(You, That(Permanent)))])` for N>1. The binder makes the choice
/// (bound as `That`) and `Sacrifice(You, That(Permanent))` pays against
/// it — choosing kept OUT of the verb (a verb takes a single [`Reference`]).
/// The implicit "you control" restriction is the Sacrifice verb's own
/// ([CR#701.21a]), not part of the printed-text filter — matching the
/// self-sacrifice form (`Sacrifice ~` -> `SacrificeThis`). The leading
/// determiner fixes N: `a`/`an`/`another`/`other` mean one and stay in the
/// phrase (so the filter parser reads `another`/`other` as self-exclusion); a
/// spelled count (`one`, `two`, …) is stripped and sets N. An unrecognized
/// leader or a subject the filter grammar can't parse declines. The self form
/// `Sacrifice ~` is matched earlier and never reaches here (it has no space).
fn sacrifice(text: &str) -> Option<String> {
    let rest = text.strip_prefix("Sacrifice ")?;
    let (first, tail) = rest.split_once(' ')?;
    let (count, phrase) = match first.to_ascii_lowercase().as_str() {
        "a" | "an" | "another" | "other" => (1, rest),
        _ => (effect::number_word(first)?, tail),
    };
    let filter = filter::parse_phrase(phrase)?;
    let binder = if count == 1 {
        format!("ChooseOne(filter: {filter})")
    } else {
        format!("Choose(quantity: Exactly({count}), filter: {filter})")
    };
    Some(format!(
        "With(binder: {binder}, body: [Do(Sacrifice(You, That(Permanent)))])"
    ))
}

/// A run of mana symbols -> `Mana([...])`. Declines on non-mana text, the
/// empty cost, and — under [`VariableMana::Decline`] — `{X}`. Shared with the
/// static-ability cost-modifier grammar (`{N} less/more to cast`), whose amount
/// is a bare mana run.
pub(crate) fn mana_component(text: &str, variable: VariableMana) -> anyhow::Result<Option<String>> {
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
