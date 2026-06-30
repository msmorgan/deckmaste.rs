//! Approximate rules-text rendering of a card's encoding / derived
//! characteristics. Total: unhandled grammar yields a `[unrendered: …]` marker,
//! never a panic.

mod ability;
mod card;
mod condition;
mod deontic;
mod effect;
mod fragment;
mod keyword;
mod replacement;
mod template;

use deckmaste_core::Ability;
use deckmaste_core::CardFace;
use deckmaste_core::ManaCost;
use deckmaste_core::StatValue;
use deckmaste_core::Subtype;
use deckmaste_core::Supertype;
use deckmaste_core::TargetSpec;
use deckmaste_core::Type;

/// The rendered, layout-ready pieces of one card/object. All fields are plain
/// text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedCard {
    pub name: String,
    pub mana_cost: String,
    pub type_line: String,
    pub rules: Vec<String>,
    pub pt: Option<String>,
}

/// Borrowed, `core`-typed view — the single input the renderer understands. A
/// printed `CardFace` and a derived live object both reduce to this.
#[derive(Debug, Clone, Copy)]
pub struct CardView<'a> {
    pub name: &'a str,
    pub mana_cost: Option<&'a ManaCost>,
    pub supertypes: &'a [Supertype],
    pub types: &'a [Type],
    pub subtypes: &'a [Subtype],
    pub power: Option<&'a StatValue>,
    pub toughness: Option<&'a StatValue>,
    pub abilities: &'a [Ability],
}

impl<'a> From<&'a CardFace> for CardView<'a> {
    fn from(f: &'a CardFace) -> Self {
        CardView {
            name: &f.name,
            mana_cost: Some(&f.mana_cost),
            supertypes: &f.supertypes,
            types: &f.types,
            subtypes: &f.subtypes,
            power: f.power.as_ref(),
            toughness: f.toughness.as_ref(),
            abilities: &f.abilities,
        }
    }
}

/// Rendering context threaded through the recursive walk.
pub(crate) struct Ctx<'a> {
    /// Display name of the subject object (used for self-referential
    /// events/effects).
    pub subject: &'a str,
    /// The current ability's targets, so `Reference::Target(i)` can resolve.
    pub targets: &'a [TargetSpec],
    /// The noun phrase the enclosing `Effect::With` bound, so the body's
    /// `Reference::That` / `Selection::Those` anaphor renders as that phrase
    /// ("Sacrifice a creature"). `None` outside a `With` body.
    pub that: Option<&'a str>,
}

impl<'a> Ctx<'a> {
    /// Re-bind the `that` anaphor over an inner render — the `Effect::With`
    /// body sees its binder's noun phrase via `Reference::That` /
    /// `Selection::Those`.
    pub(super) fn with_that(&self, phrase: &'a str) -> Ctx<'a> {
        Ctx {
            subject: self.subject,
            targets: self.targets,
            that: Some(phrase),
        }
    }
}

/// Convenience entry for a printed face.
#[must_use]
pub fn render_card_face(face: &CardFace) -> RenderedCard {
    render(&CardView::from(face))
}

fn rules(view: &CardView) -> Vec<String> {
    let mut kw_line: Vec<String> = Vec::new();
    let mut body: Vec<String> = Vec::new();
    for ability in view.abilities {
        match ability {
            Ability::Keyword(k) => kw_line.push(keyword::keyword_name(k)),
            Ability::Spell(s) => {
                // Targeting lives on an `Effect::Targeted` wrapper, which the
                // effect walk rebinds `ctx.targets` from ([CR#115.1]).
                let ctx = Ctx {
                    subject: view.name,
                    targets: &[],
                    that: None,
                };
                body.push(effect::effect(&s.effect, &ctx));
            }
            Ability::Triggered(t) => body.push(ability::triggered(t, view)),
            Ability::Static(s) => body.extend(ability::static_ability(
                s,
                &Ctx {
                    subject: view.name,
                    targets: &[],
                    that: None,
                },
            )),
            _ => {} // Activated: later tasks
        }
    }
    let mut out = Vec::new();
    if !kw_line.is_empty() {
        out.push(kw_line.join(", "));
    }
    out.extend(body);
    out
}

/// General entry: render any `CardView`.
#[must_use]
pub fn render(view: &CardView) -> RenderedCard {
    RenderedCard {
        name: view.name.to_string(),
        mana_cost: card::mana_cost(view.mana_cost),
        type_line: card::type_line(view),
        rules: rules(view),
        pt: card::pt(view),
    }
}
