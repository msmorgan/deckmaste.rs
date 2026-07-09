//! Replacement effects as sentences.

use deckmaste_core::Action;
use deckmaste_core::Count;
use deckmaste_core::CounterRef;
use deckmaste_core::EventFilter;
use deckmaste_core::OneShotEffect;
use deckmaste_core::PlayerAction;
use deckmaste_core::Predicate;
use deckmaste_core::Reference;
use deckmaste_core::Replacement;
use deckmaste_core::Zone;

use super::Ctx;
use super::ability;
use super::effect;

pub(super) fn replacement(r: &Replacement, ctx: &Ctx) -> String {
    match r {
        Replacement::Expanded(e) => replacement(&e.value, ctx),
        Replacement::Also { would, also } => {
            // The enters-tapped rider ([CR#614.1c]) prints its dedicated
            // oracle form: "~ enters tapped."
            if is_this_enters(would) && is_tap_this(also) {
                return format!("{} enters tapped.", ctx.subject);
            }
            // "~ enters with a +1/+1 counter on it." ([CR#122.6a] — the
            // `EntersWithCounters` macro's expansion) — printed in its
            // idiomatic oracle form rather than the generic "As ~ enters, put
            // ... on it." fallback below.
            if is_this_enters(would)
                && let Some((kind, count)) = put_counters_on_this(also)
            {
                return format!(
                    "{} enters with {} on it.",
                    ctx.subject,
                    effect::counter_phrase(kind, count)
                );
            }
            // "As <subject> enters, <also>." — the also-effect refers to the host as "it".
            let (_lead, when) = ability::event_clause(would, ctx);
            let it = Ctx {
                subject: "it",
                targets: ctx.targets,
                that: None,
            };
            let act = ability::lower_first(&effect::effect(also, &it));
            format!("As {when}, {act}")
        }
        Replacement::Instead { would, instead } => {
            let (_lead, when) = ability::event_clause(would, ctx);
            let it = Ctx {
                subject: "it",
                targets: ctx.targets,
                that: None,
            };
            format!(
                "If {when} would happen, {} instead.",
                trim_suffix_period(&effect::effect(instead, &it))
            )
        }
        Replacement::Skip { what } => format!("[unrendered: Skip({what:?})]."),
    }
}

fn trim_suffix_period(s: &str) -> String {
    s.strip_suffix('.').unwrap_or(s).to_string()
}

/// The `AsEnters` event: THIS entering the battlefield.
fn is_this_enters(e: &EventFilter) -> bool {
    match e {
        EventFilter::Expanded(exp) => is_this_enters(&exp.value),
        EventFilter::ZoneChange {
            what,
            to: Some(Zone::Battlefield),
            from: None,
            ..
        } => matches!(
            super::fragment::strip_expanded(what),
            Predicate::Ref(Reference::This)
        ),
        _ => false,
    }
}

/// A lone "tap this" rider body.
fn is_tap_this(e: &OneShotEffect) -> bool {
    match e {
        OneShotEffect::Expanded(exp) => is_tap_this(&exp.value),
        OneShotEffect::Act(Action::By(_, PlayerAction::Tap(Reference::This))) => true,
        _ => false,
    }
}

/// A lone "put N of `<kind>` on this" rider body — the enters-with-counters
/// self-replacement's `also` clause ([CR#122.6a]).
fn put_counters_on_this(e: &OneShotEffect) -> Option<(&CounterRef, &Count)> {
    match e {
        OneShotEffect::Expanded(exp) => put_counters_on_this(&exp.value),
        OneShotEffect::Act(Action::By(
            _,
            PlayerAction::PutCounters(Reference::This, kind, count),
        )) => Some((kind, count)),
        _ => None,
    }
}
