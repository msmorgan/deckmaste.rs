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

use std::cell::Cell;

use deckmaste_authoring::Ability;
use deckmaste_authoring::CardFace;
use deckmaste_authoring::ManaCost;
use deckmaste_authoring::StatValue;
use deckmaste_authoring::Subtype;
use deckmaste_authoring::Supertype;
use deckmaste_authoring::TargetSpec;
use deckmaste_authoring::TypeDef;

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

/// Borrowed view over AUTHORED terms — the single input the renderer
/// understands. A printed `CardFace` reduces to it directly; a derived live
/// object reduces to it by raising each characteristic back through the
/// provenance index (`deckmaste_tui::ui::detail`).
#[derive(Debug, Clone, Copy)]
pub struct CardView<'a> {
    pub name: &'a str,
    pub mana_cost: Option<&'a ManaCost>,
    pub supertypes: &'a [Supertype],
    pub types: &'a [TypeDef],
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
    /// The current ability's announce list ([CR#115.1]), so a positional read
    /// (`Target(n)` / `Targets(n)`) can print its slot's phrase.
    pub targets: &'a [TargetSpec],
    /// The noun phrase the enclosing `OneShotEffect::With` bound, so the body's
    /// `Reference::That` / `Selection::Those` anaphor renders as that phrase
    /// ("Sacrifice a creature"). `None` outside a `With` body.
    pub that: Option<&'a str>,
    /// Which announced slots this announce list has already NAMED — a bitmask
    /// keyed by slot index.
    ///
    /// English announces a target once and pronominalizes every later mention
    /// of it ("… **target player** controls … the pile of **that player**'s
    /// choice"). The model draws no such distinction — both reads are the same
    /// `Reference::Target(n)`, because a slot is one indexed entry however
    /// often it is named — so the announce/re-mention register is the
    /// renderer's to track. Set as each slot is first printed; scoped to one
    /// announce list, since a nested `Targeted` rebinds `targets` and starts
    /// its own slot 0. `None` outside any announce list (a `Ctx` with no
    /// `targets` has no slot to announce), where every read announces.
    pub named: Option<&'a Cell<u32>>,
}

impl<'a> Ctx<'a> {
    /// Re-bind the `that` anaphor over an inner render — the
    /// `OneShotEffect::With` body sees its binder's noun phrase via
    /// `Reference::That` / `Selection::Those`. Keeps the announce state: the
    /// body is still inside the same announce list.
    pub(super) fn with_that(&self, phrase: &'a str) -> Ctx<'a> {
        Ctx {
            subject: self.subject,
            targets: self.targets,
            that: Some(phrase),
            named: self.named,
        }
    }

    /// Bind an announce list over an inner render — the
    /// `OneShotEffect::Targeted` body reads these slots by index
    /// ([CR#115.3,601.2c]). `named` starts fresh: these are new slots, none of
    /// them printed yet.
    pub(super) fn with_targets(&self, targets: &'a [TargetSpec], named: &'a Cell<u32>) -> Ctx<'a> {
        Ctx {
            subject: self.subject,
            targets,
            that: self.that,
            named: Some(named),
        }
    }

    /// Claim slot `n`'s ANNOUNCE: `true` the first time the slot is named (the
    /// caller prints the announce phrase, "target creature"), `false` on every
    /// later read (the caller pronominalizes, "that creature").
    ///
    /// A slot index past the bitmask's width always announces — re-announcing
    /// is a wordier render, never a wrong one.
    pub(super) fn announce(&self, n: usize) -> bool {
        let Some(named) = self.named else {
            return true;
        };
        if n >= u32::BITS as usize {
            return true;
        }
        let seen = named.get();
        named.set(seen | (1 << n));
        seen & (1 << n) == 0
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
        // A saga chapter ([CR#714.2]) prints its Roman-numeral marker, not the
        // generic trigger phrasing — intercept the remembered `Chapter`
        // invocation before the structural peel below.
        if let Some(line) = chapter_line(&view.abilities[idx], view) {
            body.push(line);
            idx += 1;
            continue;
        }
        let ability = peel_expanded(&view.abilities[idx]);
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
                    named: None,
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
                    named: None,
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
                    named: None,
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
                        named: None,
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

/// Peel a macro-remembered `Ability::Expanded` (`EntersWithCounters(...)`, any
/// other `Ability`-kind macro invocation) down to the concrete value it
/// expanded to, so the per-ability dispatch below sees the same shape a
/// hand-written RON ability would — mirrors every other `Expanded` arm in this
/// renderer (`Replacement::Expanded`, `Condition::Expanded`,
/// `Selection::Expanded`, …), which peel structurally rather than trying to
/// re-derive the rendering from the macro's own `template:` metadata.
///
/// This peels ONLY `Expanded`, so it reaches a macro whose expansion is a
/// directly-dispatched arm (`EntersWithCounters` → `Ability::Static`). A saga
/// `Chapter` (`Expanded(Triggered(...))`, [CR#714.2]) is intercepted by
/// [`chapter_line`] BEFORE this peel, so it prints its Roman-numeral marker
/// rather than the generic trigger phrasing its bare `Triggered` would yield.
fn peel_expanded(ability: &Ability) -> &Ability {
    match ability {
        Ability::Expanded(exp) => peel_expanded(&exp.value),
        other => other,
    }
}

/// An integer as a Roman numeral ([CR#714.2a] — a chapter symbol's numeral:
/// I, II, III, IV, …). `0` yields the empty string; chapters are always ≥ 1.
fn roman(mut n: u32) -> String {
    const TABLE: &[(u32, &str)] = &[
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut out = String::new();
    for &(value, symbol) in TABLE {
        while n >= value {
            out.push_str(symbol);
            n -= value;
        }
    }
    out
}

/// Render a saga chapter ability ([CR#714.2b]) as its printed line —
/// "`<markers>` — `<effect>`" ("I — …", "II, III — …", [CR#714.2a..714.2c]) —
/// from the remembered `Chapter` invocation: the markers are its `Crossed`
/// gate's thresholds as Roman numerals, and the effect renders structurally
/// like a spell's. Returns `None` for any non-`Chapter` ability (and for the
/// unexpected non-literal threshold) so the general dispatch handles it.
fn chapter_line(ability: &Ability, view: &CardView) -> Option<String> {
    let Ability::Expanded(exp) = ability else {
        return None;
    };
    if exp.name.as_str() != "Chapter" {
        return None;
    }
    let Ability::Triggered(t) = exp.value.as_ref() else {
        return None;
    };
    let Some(deckmaste_authoring::Condition::Crossed { thresholds, .. }) = &t.condition else {
        return None;
    };
    let markers = thresholds
        .iter()
        .map(|c| match c {
            deckmaste_authoring::Count::Literal(n) => Some(roman(*n)),
            _ => None,
        })
        .collect::<Option<Vec<_>>>()?
        .join(", ");
    let ctx = Ctx {
        subject: view.name,
        targets: &[],
        that: None,
        named: None,
    };
    Some(format!("{markers} — {}", effect::effect(&t.effect, &ctx)))
}

/// Prefix a rendered ability line with its printed ability word
/// ([CR#207.2c] — italic render metadata, no rules meaning): "Domain — …".
fn with_ability_word(word: Option<&deckmaste_authoring::Ident>, line: String) -> String {
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
