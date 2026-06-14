//! Replacement effects as sentences.

use deckmaste_core::Replacement;

use super::Ctx;
use super::ability;
use super::effect;

pub(super) fn replacement(r: &Replacement, ctx: &Ctx) -> String {
    match r {
        Replacement::Expanded(e) => replacement(&e.value, ctx),
        Replacement::Also { would, also } => {
            // "As <subject> enters, <also>." — the also-effect refers to the host as "it".
            let (_lead, when) = ability::event_clause(would, ctx);
            let it = Ctx {
                subject: "it",
                targets: ctx.targets,
            };
            let act = ability::lower_first(&effect::effect(also, &it));
            format!("As {when}, {act}")
        }
        Replacement::Instead { would, instead } => {
            let (_lead, when) = ability::event_clause(would, ctx);
            let it = Ctx {
                subject: "it",
                targets: ctx.targets,
            };
            format!(
                "If {when} would happen, {} instead.",
                trim_suffix_period(&effect::effect(instead, &it))
            )
        }
        Replacement::Skip { what } => format!("[unrendered: Skip({what:?})]."),
    }
}

fn trim_suffix_period(s: &str) -> String { s.strip_suffix('.').unwrap_or(s).to_string() }
