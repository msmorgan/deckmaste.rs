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
mod outcome;
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
    /// The current ability's targets, so the slot-bound anaphors
    /// (`It`/`Target(n)`) can print their announce phrases.
    pub targets: &'a [TargetSpec],
    /// The noun phrase the enclosing `OneShotEffect::With` bound, so the body's
    /// `Reference::That` / `Selection::Those` anaphor renders as that phrase
    /// ("Sacrifice a creature"). `None` outside a `With` body.
    pub that: Option<&'a str>,
}

impl<'a> Ctx<'a> {
    /// Re-bind the `that` anaphor over an inner render — the
    /// `OneShotEffect::With` body sees its binder's noun phrase via
    /// `Reference::That` / `Selection::Those`.
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
    let mut idx = 0;
    while idx < view.abilities.len() {
        let ability = &view.abilities[idx];
        // An adjacent can't-attack + can't-block pair, each its OWN
        // `Ability::Static` (one effect per `Static` now), over the same
        // subject prints as the single oracle clause "… can't attack or
        // block." (Pacifism, [CR#509]) — checked before the general
        // per-ability dispatch below so it can consume both abilities.
        if let Ability::Static(s) = ability
            && let Some(Ability::Static(s2)) = view.abilities.get(idx + 1)
            && let Some(merged) = deontic::merged_cant_attack_block(
                s,
                s2,
                &Ctx {
                    subject: view.name,
                    targets: &[],
                    that: None,
                },
            )
        {
            // `Ability::Static` carries no `ability_word` slot anymore (the
            // deleted `StaticAbility` struct did) — a static clause never
            // gets the printed "Word — " prefix.
            body.push(merged);
            idx += 2;
            continue;
        }
        // An adjacent pair of `StaticEffect::OutcomeGate`s over complementary
        // subjects (Platinum Angel/Abyssal Persecutor: one `who: Ref(You)`,
        // one `who: OpponentOf(Ref(You))`) prints as the single oracle
        // sentence "You can't lose the game and your opponents can't win the
        // game." ([CR#104],[CR#704]) — same cross-ability-merge shape as the
        // Pacifism block above.
        if let Ability::Static(s) = ability
            && let Some(Ability::Static(s2)) = view.abilities.get(idx + 1)
            && let Some(merged) = outcome::merge_outcome_gates(
                s,
                s2,
                &Ctx {
                    subject: view.name,
                    targets: &[],
                    that: None,
                },
            )
        {
            body.push(merged);
            idx += 2;
            continue;
        }
        match ability {
            Ability::Keyword(k) => kw_line.push(keyword::keyword_name(k)),
            Ability::Spell(s) => {
                // Targeting lives on an `OneShotEffect::Targeted` wrapper, which the
                // effect walk rebinds `ctx.targets` from ([CR#115.1]).
                let ctx = Ctx {
                    subject: view.name,
                    targets: &[],
                    that: None,
                };
                // A spell's effect can itself be multi-line (a modal's
                // "Choose ..." + bulleted modes, or an "as an additional
                // cost" clause the printed card breaks onto its own line):
                // split on the renderer's embedded `\n`s into separate
                // printed lines, mirroring the `Static` arm just below.
                let mut lines: Vec<String> = effect::effect(&s.effect, &ctx)
                    .split('\n')
                    .map(str::to_string)
                    .collect();
                if let Some(first) = lines.first_mut() {
                    *first = with_ability_word(s.ability_word.as_ref(), std::mem::take(first));
                }
                body.extend(lines);
            }
            Ability::Triggered(t) => body.push(with_ability_word(
                t.ability_word.as_ref(),
                ability::triggered(t, view),
            )),
            Ability::Static(s) => {
                // `Ability::Static` carries no `ability_word` slot anymore
                // (the deleted `StaticAbility` struct did), so there is no
                // "Word — " prefix to apply here.
                let lines = ability::static_ability(
                    s,
                    &Ctx {
                        subject: view.name,
                        targets: &[],
                        that: None,
                    },
                );
                body.extend(lines);
            }
            Ability::Activated(a) => body.push(with_ability_word(
                a.ability_word.as_ref(),
                ability::activated(a, view),
            )),
            _ => {} // Mana-only variants beyond Activated: later tasks
        }
        idx += 1;
    }
    let mut out = Vec::new();
    if !kw_line.is_empty() {
        // The keyword line prints its first keyword capitalized and the
        // rest lowercased ("Flying, deathtouch").
        let line = kw_line
            .iter()
            .enumerate()
            .map(
                |(i, k)| {
                    if i == 0 { fragment_capitalize(k) } else { ability::lower_first(k) }
                },
            )
            .collect::<Vec<_>>()
            .join(", ");
        out.push(line);
    }
    out.extend(body);
    out
}

/// Prefix a rendered ability line with its printed ability word
/// ([CR#207.2c] — italic render metadata, no rules meaning): "Domain — …".
fn with_ability_word(word: Option<&deckmaste_core::Ident>, line: String) -> String {
    match word {
        Some(w) => format!("{} — {line}", w.as_str()),
        None => line,
    }
}

fn fragment_capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map_or_else(String::new, |first| first.to_uppercase().chain(c).collect())
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
