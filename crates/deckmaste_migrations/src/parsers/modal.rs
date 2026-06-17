//! The modal "Choose one —" parser ([CR#700.2]). Unlike the per-line registry
//! parsers, a modal ability spans MULTIPLE consecutive `Unparsed` lines — a
//! header ("Choose one —") plus one bulleted `• <effect>` line per mode — so
//! this is a face-level pre-pass (like `resolve::fold_spell_ascend`), not an
//! `AbilityParser`. It scans a face's abilities for a modal header, gathers the
//! bullet lines that follow, parses each bullet's effect through the shared
//! [`effect::parse_clause`] grammar, and (only if EVERY bullet parses)
//! collapses the whole run into one `Spell(effect: Modal(...))` ability.
//!
//! Each mode's targets ride an `Effect::Targeted` wrapper inside its `effect`
//! ([CR#700.2c]); per-mode additional costs ride `Mode.cost` ([CR#700.2h], not
//! yet emitted — see the deferral note on [`fold_modal`]). The engine's `Modal`
//! resolution maps the emitted `ChooseSpec { count, up_to, repeats }` to the
//! `ChooseModes` decision's `min`/`max`/`repeats` (`max = repeats ? count :
//! min(count, options)`, `min = up_to ? 0 : max`).

use crate::parsers::effect;
use crate::parsers::effect::ParsedEffect;
use crate::resolve::CardKind;
use crate::resolve::ResolveCtx;
use crate::todo_card::TodoAbility;
use crate::todo_card::TodoCardFace;

/// The bullet glyph an Oracle modal mode line opens with (U+2022), followed by
/// a space in the source text.
const BULLET: char = '\u{2022}';

/// How a header's "Choose …" instruction maps onto the engine's
/// [`ChooseSpec`](deckmaste_core::ChooseSpec) fields — `count` is the bare
/// numeral written into the spec, `up_to`/`repeats` the two flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ChooseShape {
    count: u32,
    up_to: bool,
    repeats: bool,
}

/// Classifies a modal header line into a [`ChooseShape`], or `None` if the line
/// isn't a recognized modal header. The em-dash–terminated forms
/// ("Choose one —", "Choose two —", "Choose one or both —", "Choose one or more
/// —") are the bulk; the sentence form ("Choose three. You may choose the same
/// mode more than once.") carries `repeats`.
///
/// Fidelity gaps deferred to the announce-time engine seam: "one or both" /
/// "one or more" emit `up_to: true` (engine `min = 0`), so the rules-faithful
/// "at least one" floor is not yet enforced — `count` still caps the maximum.
fn classify_header(line: &str, mode_count: u32) -> Option<ChooseShape> {
    // Em-dash forms: "Choose <n> —", with optional "or both"/"or more".
    if let Some(rest) = line.strip_prefix("Choose ")
        && let Some(word) = rest.strip_suffix(" \u{2014}")
    {
        return match word {
            "one" => Some(ChooseShape {
                count: 1,
                up_to: false,
                repeats: false,
            }),
            "two" => Some(ChooseShape {
                count: 2,
                up_to: false,
                repeats: false,
            }),
            "three" => Some(ChooseShape {
                count: 3,
                up_to: false,
                repeats: false,
            }),
            // "one or both" — up to two ([CR#700.2]); "one or more" (escalate)
            // — up to all modes. Both lift the floor to 0 (announce-time seam).
            "one or both" => Some(ChooseShape {
                count: 2,
                up_to: true,
                repeats: false,
            }),
            "one or more" => Some(ChooseShape {
                count: mode_count,
                up_to: true,
                repeats: false,
            }),
            _ => None,
        };
    }
    // Sentence form with the repeat clause ([CR#700.2d]): "Choose three. You may
    // choose the same mode more than once."
    let repeat_clause = ". You may choose the same mode more than once.";
    if let Some(head) = line.strip_suffix(repeat_clause)
        && let Some(n) = head.strip_prefix("Choose ").and_then(word_to_count)
    {
        return Some(ChooseShape {
            count: n,
            up_to: false,
            repeats: true,
        });
    }
    None
}

/// A number word a modal header counts in (`one`..`five`), as used by the
/// "Choose <word>." sentence form.
fn word_to_count(word: &str) -> Option<u32> {
    Some(match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        _ => return None,
    })
}

/// A bullet mode line stripped to its effect body, or `None` if `line` isn't a
/// bullet. The Oracle bullet is `• ` (U+2022 + space).
fn strip_bullet(line: &str) -> Option<&str> {
    line.strip_prefix(BULLET)
        .map(str::trim_start)
        .filter(|body| !body.is_empty())
}

/// Renders one parsed mode effect into a `Mode(effect: …)` RON fragment,
/// lifting declared targets onto an `Effect::Targeted` wrapper ([CR#700.2c]) —
/// the same framing the `Spell` parser applies, scoped per-mode.
fn render_mode(parsed: &ParsedEffect) -> String {
    if parsed.targets.is_empty() {
        format!("Mode(effect: {})", parsed.effect)
    } else {
        format!(
            "Mode(effect: Targeted(targets: [{}], effect: {}))",
            parsed.targets.join(", "),
            parsed.effect,
        )
    }
}

/// Renders a classified header + its parsed modes into the bare
/// `Spell(effect: Modal(...))` RON. `count` writes bare ([`Count::Literal`]);
/// the two flags are omitted when false (matching `ChooseSpec`'s
/// `skip_serializing_if`).
fn render_modal(shape: ChooseShape, modes: &[ParsedEffect]) -> String {
    let mut choose = format!("count: {}", shape.count);
    if shape.up_to {
        choose.push_str(", up_to: true");
    }
    if shape.repeats {
        choose.push_str(", repeats: true");
    }
    let modes = modes.iter().map(render_mode).collect::<Vec<_>>().join(", ");
    format!("Spell(effect: Modal(choose: ChooseSpec({choose}), modes: [{modes}]))")
}

/// The face-level modal pre-pass. Finds the first modal header among `face`'s
/// abilities, gathers the bullet lines immediately following it, and — only if
/// EVERY bullet's effect body parses through [`effect::parse_clause`] —
/// replaces the header slot with one `Parsed` `Spell(effect: Modal(...))`
/// ability and removes the consumed bullet slots. Returns whether it folded
/// anything.
///
/// Conservative by design: a header with a non-parsing bullet, fewer than two
/// modes ([CR#700.2]), or on a non-spell card is left untouched (every slot
/// stays `Unparsed`) so no partial/garbage modal lands.
///
/// Deferred (reported on the ticket): per-mode `{P}`-cost ([CR#700.2i]) and
/// Spree `+ {cost} —` ([CR#702.172a]) mode prefixes, and permanent-borne modals
/// embedded in an activated/triggered header (e.g. Marath's
/// "…: Choose one —"). Those bullets don't strip cleanly through `effect::
/// parse_clause`, so such headers decline and stay `Unparsed`.
pub(crate) fn fold_modal(face: &mut TodoCardFace, ctx: &ResolveCtx) -> bool {
    // v1 frames modals as `Spell` abilities only. Permanent-borne modals
    // (activated/triggered headers carrying "… : Choose one —", e.g. Marath)
    // need the cost/trigger frame around the modal and are deferred.
    if ctx.kind != CardKind::Spell {
        return false;
    }

    // Locate a header line whose following slots are bullets.
    let Some(header_idx) = find_modal_header(&face.abilities) else {
        return false;
    };

    // Gather the contiguous bullet run after the header.
    let mut bodies = Vec::new();
    let mut idx = header_idx + 1;
    while let Some(TodoAbility::Unparsed(line)) = face.abilities.get(idx) {
        let Some(body) = strip_bullet(line) else { break };
        bodies.push(body.to_owned());
        idx += 1;
    }

    // A modal has two or more modes ([CR#700.2]).
    if bodies.len() < 2 {
        return false;
    }

    let TodoAbility::Unparsed(header) = &face.abilities[header_idx] else {
        return false;
    };
    let mode_count = u32::try_from(bodies.len()).unwrap_or(u32::MAX);
    let Some(shape) = classify_header(header, mode_count) else {
        return false;
    };

    // EVERY bullet must parse, or we decline wholesale (no partial modal).
    let mut modes = Vec::with_capacity(bodies.len());
    for body in &bodies {
        let Some(parsed) = effect::parse_clause(body, ctx) else {
            return false;
        };
        modes.push(parsed);
    }

    let ron = render_modal(shape, &modes);
    // Replace the header with the folded Modal, drop the bullet slots.
    face.abilities[header_idx] = TodoAbility::Parsed(ron);
    face.abilities.drain(header_idx + 1..idx);
    true
}

/// The index of the first `Unparsed` modal header immediately followed by at
/// least one bullet `Unparsed` line. The "followed by a bullet" guard is what
/// distinguishes a true modal header from a lone "Choose target …" verb line
/// (which is a regular targeting effect, not a modal).
fn find_modal_header(abilities: &[TodoAbility]) -> Option<usize> {
    let mode_count = u32::try_from(abilities.len()).unwrap_or(u32::MAX);
    abilities.iter().enumerate().find_map(|(i, a)| {
        let TodoAbility::Unparsed(line) = a else { return None };
        // Must be a recognizable header AND be followed by a bullet line.
        classify_header(line, mode_count)?;
        match abilities.get(i + 1) {
            Some(TodoAbility::Unparsed(next)) if strip_bullet(next).is_some() => Some(i),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::test_ctx;

    /// Builds a spell face from raw `Unparsed` lines.
    fn spell_face(lines: &[&str]) -> TodoCardFace {
        TodoCardFace {
            name: "X".into(),
            types: vec![crate::todo_card::RawIdent("Instant".into())],
            abilities: lines
                .iter()
                .map(|l| TodoAbility::Unparsed((*l).to_owned()))
                .collect(),
            ..Default::default()
        }
    }

    fn parsed(face: &TodoCardFace, i: usize) -> &str {
        match &face.abilities[i] {
            TodoAbility::Parsed(s) => s,
            TodoAbility::Unparsed(u) => panic!("expected Parsed at {i}, got Unparsed({u:?})"),
        }
    }

    /// "Choose one —" with two untargeted modes folds into one `Spell(Modal)`.
    #[test]
    fn folds_choose_one_untargeted() {
        let mut face = spell_face(&[
            "Choose one \u{2014}",
            "\u{2022} You gain 3 life.",
            "\u{2022} Draw two cards.",
        ]);
        assert!(fold_modal(&mut face, &test_ctx::ctx(CardKind::Spell)));
        assert_eq!(
            face.abilities.len(),
            1,
            "header + bullets collapsed to one slot"
        );
        assert_eq!(
            parsed(&face, 0),
            "Spell(effect: Modal(choose: ChooseSpec(count: 1), modes: [\
             Mode(effect: GainLife(3)), Mode(effect: Draw(2))]))"
        );
    }

    /// A targeted bullet lifts the target onto a per-mode `Targeted` wrapper.
    #[test]
    fn folds_targeted_mode() {
        let mut face = spell_face(&[
            "Choose one \u{2014}",
            "\u{2022} Destroy target artifact.",
            "\u{2022} You gain 3 life.",
        ]);
        assert!(fold_modal(&mut face, &test_ctx::ctx(CardKind::Spell)));
        assert_eq!(
            parsed(&face, 0),
            "Spell(effect: Modal(choose: ChooseSpec(count: 1), modes: [\
             Mode(effect: Targeted(targets: [TargetOne(Type(Artifact))], effect: Destroy(Target(0)))), \
             Mode(effect: GainLife(3))]))"
        );
    }

    /// "Choose two —" emits `count: 2`.
    #[test]
    fn folds_choose_two() {
        let mut face = spell_face(&[
            "Choose two \u{2014}",
            "\u{2022} You gain 3 life.",
            "\u{2022} Draw two cards.",
        ]);
        assert!(fold_modal(&mut face, &test_ctx::ctx(CardKind::Spell)));
        assert!(parsed(&face, 0).contains("ChooseSpec(count: 2)"));
    }

    /// "Choose one or both —" emits `up_to: true`.
    #[test]
    fn folds_choose_one_or_both() {
        let mut face = spell_face(&[
            "Choose one or both \u{2014}",
            "\u{2022} You gain 3 life.",
            "\u{2022} Draw two cards.",
        ]);
        assert!(fold_modal(&mut face, &test_ctx::ctx(CardKind::Spell)));
        assert!(parsed(&face, 0).contains("ChooseSpec(count: 2, up_to: true)"));
    }

    /// "Choose one or more —" (escalate) emits `up_to` over the mode count.
    #[test]
    fn folds_choose_one_or_more() {
        let mut face = spell_face(&[
            "Choose one or more \u{2014}",
            "\u{2022} You gain 3 life.",
            "\u{2022} Draw two cards.",
            "\u{2022} Draw one card.",
        ]);
        assert!(fold_modal(&mut face, &test_ctx::ctx(CardKind::Spell)));
        assert!(parsed(&face, 0).contains("ChooseSpec(count: 3, up_to: true)"));
    }

    /// The repeat sentence form emits `repeats: true`.
    #[test]
    fn folds_repeat_sentence_form() {
        let mut face = spell_face(&[
            "Choose three. You may choose the same mode more than once.",
            "\u{2022} You gain 3 life.",
            "\u{2022} Draw two cards.",
        ]);
        assert!(fold_modal(&mut face, &test_ctx::ctx(CardKind::Spell)));
        assert!(
            parsed(&face, 0).contains("ChooseSpec(count: 3, repeats: true)"),
            "{}",
            parsed(&face, 0)
        );
    }

    /// A non-parsing bullet declines the WHOLE modal — every slot stays
    /// Unparsed.
    #[test]
    fn declines_when_a_bullet_doesnt_parse() {
        let mut face = spell_face(&[
            "Choose one \u{2014}",
            "\u{2022} You gain 3 life.",
            "\u{2022} Exile target creature, then do something inexpressible.",
        ]);
        assert!(!fold_modal(&mut face, &test_ctx::ctx(CardKind::Spell)));
        assert_eq!(face.abilities.len(), 3, "nothing folded");
        assert!(matches!(&face.abilities[0], TodoAbility::Unparsed(_)));
    }

    /// A lone "Choose target …" verb (no bullets) is not a modal.
    #[test]
    fn ignores_non_modal_choose_lines() {
        let mut face = spell_face(&["Choose target artifact. Destroy it."]);
        assert!(!fold_modal(&mut face, &test_ctx::ctx(CardKind::Spell)));
        assert!(matches!(&face.abilities[0], TodoAbility::Unparsed(_)));
    }

    /// Modal framing is spell-only in v1: a permanent's modal is untouched.
    #[test]
    fn declines_on_permanent() {
        let mut face = spell_face(&[
            "Choose one \u{2014}",
            "\u{2022} You gain 3 life.",
            "\u{2022} Draw two cards.",
        ]);
        face.types = vec![crate::todo_card::RawIdent("Creature".into())];
        assert!(!fold_modal(&mut face, &test_ctx::ctx(CardKind::Permanent)));
        assert_eq!(face.abilities.len(), 3);
    }

    /// A single mode is not a modal ([CR#700.2] needs two or more).
    #[test]
    fn declines_single_mode() {
        let mut face = spell_face(&["Choose one \u{2014}", "\u{2022} You gain 3 life."]);
        assert!(!fold_modal(&mut face, &test_ctx::ctx(CardKind::Spell)));
    }
}
