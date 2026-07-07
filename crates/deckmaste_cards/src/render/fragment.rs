//! Shared noun-phrase / count fragment renderers.

use deckmaste_core::Anchor;
use deckmaste_core::Characteristic;
use deckmaste_core::CharacteristicPredicate;
use deckmaste_core::Color;
use deckmaste_core::Count;
use deckmaste_core::Extremum;
use deckmaste_core::ObjectKind;
use deckmaste_core::Predicate;
use deckmaste_core::Quantity;
use deckmaste_core::Reference;
use deckmaste_core::RelationPredicate;
use deckmaste_core::RoundMode;
use deckmaste_core::Selection;
use deckmaste_core::Stat;
use deckmaste_core::StatePredicate;
use deckmaste_core::TargetSpec;
use deckmaste_core::Zone;

use super::Ctx;

/// Small literal counts of OBJECTS spell out as words in oracle text
/// ("draw three cards", "put two cards…"); amounts of damage/life keep
/// digits ("deals 3 damage", "gain 2 life").
pub(super) fn number_word(n: u32) -> Option<&'static str> {
    Some(match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        _ => return None,
    })
}

/// A small count as text: `Literal(n)` -> "n"; callers special-case "a/an".
pub(super) fn count(c: &Count) -> String {
    match c {
        Count::Literal(n) => n.to_string(),
        Count::X => "X".to_string(),
        Count::Damage(r) => format!(
            "damage marked on {}",
            reference(
                r,
                &Ctx {
                    subject: "it",
                    targets: &[],
                    that: None,
                }
            )
        ),
        // [CR#122.1]: the counter-count read — "the number of experience
        // counters you have" (a player-borne kind), "the number of lore
        // counters on it" (object-borne).
        Count::CounterCount(r, kind) => {
            let noun = counter_noun(kind.as_str());
            match r.as_ref() {
                Reference::You => format!("the number of {noun} counters you have"),
                other => format!(
                    "the number of {noun} counters on {}",
                    reference(
                        other,
                        &Ctx {
                            subject: "it",
                            targets: &[],
                            that: None,
                        }
                    )
                ),
            }
        }
        // [CR#107.1] value arithmetic.
        Count::Plus(a, b) => format!("{} plus {}", count(a), count(b)),
        Count::Minus(a, b) => format!("{} minus {}", count(a), count(b)),
        Count::Times(a, b) => format!("{} times {}", count(a), count(b)),
        Count::Max(a, b) => format!("the greater of {} and {}", count(a), count(b)),
        Count::Min(a, b) => format!("the lesser of {} and {}", count(a), count(b)),
        Count::Half(mode, inner) => {
            let rounding = match mode {
                RoundMode::RoundUp => "rounded up",
                RoundMode::RoundDown => "rounded down",
            };
            format!("half {}, {rounding}", count(inner))
        }
        // [CR#107.3] distinct-union count (Domain / Coven / Tarmogoyf). The
        // subtype axis over a typed group names the type's own subtype
        // family ("land types"); the group reads as its plural subject
        // phrase ("lands you control").
        Count::CountDistinct(axis, filter) => {
            let axis_word = match (axis, find_card_type(filter)) {
                (Characteristic::Subtypes, Some(t)) => {
                    format!("{} types", super::card::type_str(t).to_lowercase())
                }
                _ => characteristic_word(*axis).to_string(),
            };
            let group = super::ability::lower_first(&filter_subject(filter));
            format!("the number of {axis_word} among {group}")
        }
        // The value anaphor's two spellings ([CR#107.3,608.2i]).
        Count::ThatMany => "that many".to_string(),
        Count::ThatMuch => "that much".to_string(),
        // A remembered count macro (e.g. `Domain`): prefer its own template,
        // else render the expansion structurally.
        Count::Expanded(e) => super::template::expanded(e, "it").unwrap_or_else(|| count(&e.value)),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// A counter kind's English noun: the ident minus its `Counter` suffix,
/// lowercased — `Experience` → "experience", `LoreCounter` → "lore",
/// `AgeCounter` → "age".
fn counter_noun(ident: &str) -> String {
    ident.trim_end_matches("Counter").to_lowercase()
}

/// The plural noun for a [`Characteristic`] axis, used by the distinct-count
/// phrase ("the number of subtypes among …").
fn characteristic_word(axis: Characteristic) -> &'static str {
    match axis {
        Characteristic::Colors => "colors",
        Characteristic::Types => "types",
        Characteristic::Subtypes => "subtypes",
        // [CR#205.3i] — Domain's axis.
        Characteristic::BasicLandTypes => "basic land types",
        Characteristic::Supertypes => "supertypes",
        Characteristic::Power => "powers",
        Characteristic::Toughness => "toughnesses",
        Characteristic::Defense => "defenses",
        Characteristic::ManaCost => "mana costs",
        Characteristic::Name => "names",
    }
}

/// A `Selection` GROUP as the noun phrase a combinator divides/iterates over
/// (`Distribute.group`, `Each.over`). Verb patients are single objects now
/// and render via [`reference`].
pub(super) fn selection(sel: &Selection, ctx: &Ctx) -> String {
    match sel {
        // Look through a macro-provenance wrapper (a Selection-position
        // macro like `OtherCreaturesYouControl` expands the WHOLE selection,
        // not just an inner filter) — recompute from the expanded value,
        // same as every other `Expanded` arm in this renderer.
        Selection::Expanded(e) => selection(&e.value, ctx),
        // [CR#608.2d] the whole matching set — "each creature".
        Selection::SelectAll(f) => format!("each {}", filter_noun(f)),
        // Combined groups — "each X and each Y".
        Selection::Union(members) => members
            .iter()
            .map(|m| selection(m, ctx))
            .collect::<Vec<_>>()
            .join(" and "),
        // The plural anaphors ([CR#608.2d]): `They` reads the bound group's
        // noun phrase when an enclosing binder supplies one, else the bare
        // pronoun; `Them(sort)` names its sort ("those cards").
        Selection::They => ctx.that.unwrap_or("them").to_string(),
        Selection::Them(sort) => format!("those {}s", sort.noun()),
        Selection::PilesOf { of, .. } => {
            format!("the piles {} separated", reference(of, ctx))
        }
        // [CR#107.1] the extremal element: "the creature with the greatest
        // power". The projection's axis is named when it is a simple stat read.
        Selection::Pick { op, of, by } => {
            let extreme = match op {
                Extremum::Greatest => "greatest",
                Extremum::Least => "least",
            };
            let axis = match by.as_ref() {
                Count::StatOf(_, Stat::Power) => " power",
                Count::StatOf(_, Stat::Toughness) => " toughness",
                _ => "",
            };
            format!("the {} with the {extreme}{axis}", filter_noun(of))
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

/// A `Reference` as a noun phrase.
pub(super) fn reference(r: &Reference, ctx: &Ctx) -> String {
    match r {
        Reference::This => ctx.subject.to_string(),
        Reference::You => "you".to_string(),
        // The sorted singular anaphor: an enclosing binder's noun phrase
        // when one is bound (the With collapse — "Sacrifice a creature"),
        // else the English pronoun phrase — "that card", "that creature"
        // ([CR#608.2d]; the engine, not the renderer, resolves it).
        Reference::That(sort) => ctx
            .that
            .map_or_else(|| format!("that {}", sort.noun()), str::to_string),
        // The nth announced target ([CR#115.3,601.2c]) prints its slot's
        // target phrase ("target creature you control").
        Reference::Target(n) => target_phrase(*n, ctx),
        // `It`: an `Each`/`Distribute` element reads the binder's noun
        // phrase from the shared `ctx.that` slot ([CR#601.2b,608]); at a
        // single-slot announce root it reads the announced target's phrase
        // ("any target", the R1 nearest antecedent); otherwise it is the
        // wildcard anaphor — the plain English pronoun.
        Reference::It => match (ctx.that, ctx.targets.len()) {
            (Some(that), _) => that.to_string(),
            (None, 1) => target_phrase(0, ctx),
            _ => "it".to_string(),
        },
        // The triggering event's object/patient ([CR#603.2e,608.2k]): an
        // enclosing binder's descriptive phrase when one is threaded
        // (`AdditionalCost`'s "the sacrificed creature", mirroring `That`'s
        // `ctx.that` read), else the generic anaphor "it".
        Reference::EventObject | Reference::EventPatient => {
            ctx.that.map_or_else(|| "it".to_string(), str::to_string)
        }
        // The responsible player ("that player").
        Reference::EventActor => "that player".to_string(),
        // The combat defender ([CR#506.2]) — always a player.
        Reference::DefendingPlayer => "the defending player".to_string(),
        // A player derived from an object: the possessive pronoun stands for
        // the wrapped object, which the surrounding clause already names —
        // Mana Leak's "counter target spell unless its controller pays {3}"
        // ([CR#118.12a]). An object's controller ([CR#109.4]) / owner
        // ([CR#108.3]).
        Reference::ControllerOf(_) => "its controller".to_string(),
        Reference::OwnerOf(_) => "its owner".to_string(),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// The i-th announced slot's phrase (an anaphor's slot-bound read).
fn target_phrase(i: usize, ctx: &Ctx) -> String {
    match ctx.targets.get(i) {
        Some(spec) => target_spec(spec),
        None => "[unrendered: missing target]".to_string(),
    }
}

/// A `TargetSpec` as the phrase naming what it points at.
pub(super) fn target_spec(spec: &TargetSpec) -> String {
    match spec {
        // Macro-provenance: prefer the invocation's own template (e.g. AnyTarget
        // -> "any target"); fall back to the expansion. Target templates name
        // the target, never the host, so the subject is irrelevant here.
        TargetSpec::Expanded(exp) => {
            super::template::expanded(exp, "").unwrap_or_else(|| target_spec(&exp.value))
        }
        TargetSpec::Target(q, filter) if q.is_one() => {
            format!("target {}", filter_noun(filter))
        }
        // The co-target set-distinctness constraint ([CR#115.7e]) prints as
        // the "another" restrictor: "another target creature".
        TargetSpec::Distinct(_, inner) => format!("another {}", target_spec(inner)),
        other @ TargetSpec::Target(..) => format!("[unrendered: {other:?}]"),
    }
}

/// The announce phrase of a PLURAL target slot, read where a divided
/// distribution names its announced set ([CR#601.2d]): `Target(Between(1,
/// 3), AnyTarget)` → "one, two, or three targets". `None` for shapes without
/// an oracle enumeration (the caller falls back to the plural pronoun).
pub(super) fn announced_group_phrase(spec: &TargetSpec) -> Option<String> {
    let (q, filter) = match spec {
        TargetSpec::Expanded(exp) => return announced_group_phrase(&exp.value),
        TargetSpec::Distinct(_, inner) => {
            return announced_group_phrase(inner);
        }
        TargetSpec::Target(q, filter) => (q, filter),
    };
    let (Some(Count::Literal(lo)), Some(Count::Literal(hi))) = q.bounds() else {
        return None;
    };
    if *lo < 1 || hi < lo || *hi - *lo > 3 {
        return None;
    }
    let words: Vec<&str> = (*lo..=*hi)
        .map(|n| number_word(n).unwrap_or("some"))
        .collect();
    let counts = match words.as_slice() {
        [one] => (*one).to_string(),
        [a, b] => format!("{a} or {b}"),
        many => {
            let (last, rest) = many.split_last()?;
            format!("{}, or {last}", rest.join(", "))
        }
    };
    // The any-target slot reads bare "targets"; a filtered slot names its
    // noun ("target creatures").
    let noun = filter_noun(filter);
    let noun_phrase = if noun == "any target" || noun.starts_with("[unrendered") {
        "targets".to_string()
    } else {
        format!("target {noun}s")
    };
    Some(format!("{counts} {noun_phrase}"))
}

/// A simple noun for a filter, used in target phrases and "each <noun>"
/// selection phrases.  Prefers a filter macro's own noun template ("creature",
/// "player", ...); falls back to structural derivation ([`find_card_type`] /
/// [`strip_expanded`]) for hand-built (un-wrapped) filters.
pub(super) fn filter_noun(filter: &Predicate) -> String {
    if let Predicate::Expanded(exp) = filter
        && let Some(noun) = super::template::expanded(exp, "")
    {
        return noun;
    }
    if let Some(t) = find_card_type(filter) {
        let base = super::card::type_str(t).to_lowercase();
        // A controller restrictor rides the noun: "creature you control",
        // "creature you don't control", "creature an opponent controls" —
        // the restrictor is printed text, never dropped.
        return match controller_suffix(filter) {
            Some(suffix) => format!("{base} {suffix}"),
            None => base,
        };
    }
    match strip_expanded(filter) {
        Predicate::Characteristic(CharacteristicPredicate::ColorIs(c)) => {
            super::effect::color_word(*c).to_string()
        }
        Predicate::Kind(ObjectKind::Player) => "player".to_string(),
        // An ability on the stack ([CR#602.2a,603.3]): "counter target ability".
        Predicate::Kind(ObjectKind::Ability) => "ability".to_string(),
        // Team-relative player nouns ([CR#102.3]): "target opponent" /
        // "target teammate" (relative to the carrier's controller).
        Predicate::Relation(RelationPredicate::OpponentOf(inner))
            if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
        {
            "opponent".to_string()
        }
        Predicate::Relation(RelationPredicate::TeammateOf(inner))
            if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
        {
            "teammate".to_string()
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

/// The controller-restrictor phrase among a filter's `AllOf` parts:
/// `ControlledBy(You)` → "you control"; `Not(ControlledBy(You))` → "you
/// don't control"; `ControlledBy(OpponentOf(You))` → "an opponent controls".
/// `None` when the filter carries no controller part.
fn controller_suffix(filter: &Predicate) -> Option<&'static str> {
    for part in flatten_all_of(filter) {
        match strip_expanded(part) {
            Predicate::Relation(RelationPredicate::ControlledBy(inner)) => {
                return match strip_expanded(inner) {
                    Predicate::Ref(Reference::You) => Some("you control"),
                    Predicate::Relation(RelationPredicate::OpponentOf(who))
                        if matches!(strip_expanded(who), Predicate::Ref(Reference::You)) =>
                    {
                        Some("an opponent controls")
                    }
                    _ => None,
                };
            }
            Predicate::Not(negated) => {
                if let Predicate::Relation(RelationPredicate::ControlledBy(inner)) =
                    strip_expanded(negated)
                    && matches!(strip_expanded(inner), Predicate::Ref(Reference::You))
                {
                    return Some("you don't control");
                }
            }
            _ => {}
        }
    }
    None
}

// ── Static-ability subject phrases ──────────────────────────────────────────

/// A zone as the noun used in "in your <zone>" / "from your <zone>" phrases.
pub(super) fn zone_word(z: Zone) -> &'static str {
    match z {
        Zone::Battlefield => "battlefield",
        Zone::Command => "command zone",
        Zone::Exile => "exile",
        Zone::Graveyard => "graveyard",
        Zone::Hand => "hand",
        Zone::Library => "library",
        Zone::Stack => "stack",
    }
}

/// See through macro-provenance wrappers on a `Predicate`.
pub(super) fn strip_expanded(f: &Predicate) -> &Predicate {
    match f {
        Predicate::Expanded(e) => strip_expanded(&e.value),
        other => other,
    }
}

/// A bare `Modify`'s subject: a single [`Reference`], singular agreement
/// ("Test Aura gets +1/+1.", "Enchanted creature gets +2/+2.").
pub(super) fn modify_subject(r: &Reference, ctx: &super::Ctx) -> String {
    reference_subject(r, ctx)
}

/// An `Each`'s subject: the [`Selection`] it distributes over, plural
/// agreement ("Creatures you control get +1/+1."). `SelectAll` reads through
/// [`filter_subject`] (the plural-controller-phrase reading); any other
/// selection shape falls back to the generic noun-phrase reader, capitalized
/// for sentence-start use.
pub(super) fn each_subject(sel: &Selection, ctx: &super::Ctx) -> String {
    match sel {
        // Look through a macro-provenance wrapper first, same as `selection`
        // above — a Selection-position macro's expanded value still reads as
        // whichever shape it produced (usually `SelectAll`).
        Selection::Expanded(e) => each_subject(&e.value, ctx),
        Selection::SelectAll(f) => filter_subject(f),
        other => capitalize(&selection(other, ctx)),
    }
}

/// A `Reference` as a static-effect subject phrase (capitalized for
/// sentence-start use).
fn reference_subject(r: &Reference, ctx: &super::Ctx) -> String {
    match r {
        Reference::This => ctx.subject.to_string(),
        // The slot-bound anaphors as a sentence subject ("Target creature
        // gets +3/+3 …"): `It` at a single-slot announce root, `Target(n)`
        // by position.
        Reference::It if ctx.that.is_none() && ctx.targets.len() == 1 => {
            capitalize(&target_phrase(0, ctx))
        }
        Reference::Target(n) => capitalize(&target_phrase(*n, ctx)),
        // Aura host: "Enchanted creature gets +2/+2." (matches deontic_subject).
        Reference::AttachHostOf(inner) if matches!(**inner, Reference::This) => {
            "Enchanted creature".to_string()
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

/// Capitalize the first character of a string.
pub(super) fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_uppercase().chain(c).collect(),
        None => String::new(),
    }
}

/// A `Predicate` as a plural subject noun phrase: "Creatures you control",
/// "Other creatures you control", "Creatures your opponents control".
///
/// The Creature filter macro expands as
/// `Expanded(value=AllOf([Expanded(Permanent),
/// Characteristic(Type(Creature))]))`. `flatten_all_of` and `find_card_type`
/// see through both layers.
pub(super) fn filter_subject(f: &Predicate) -> String {
    let parts = flatten_all_of(f);
    let mut other = false;
    let mut base = "Permanents".to_string();
    let mut typed = false;
    let mut color: Option<Color> = None;
    let mut control: Option<String> = None;
    for p in parts {
        match strip_expanded(p) {
            Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
                base = format!("{}s", super::card::type_str(*t));
                typed = true;
            }
            Predicate::Characteristic(CharacteristicPredicate::ColorIs(c)) => {
                color = Some(*c);
            }
            Predicate::Not(inner) if strip_expanded(inner).is_this() => {
                other = true;
            }
            Predicate::Relation(RelationPredicate::ControlledBy(inner)) => {
                control = Some(controller_phrase(inner));
            }
            // The Creature macro expands to AllOf([Expanded(Permanent),
            // Characteristic(Type(Creature))]); check whether this part holds
            // a card type buried in a nested AllOf.
            stripped => {
                if let Some(t) = find_card_type(stripped) {
                    base = format!("{}s", super::card::type_str(t));
                    typed = true;
                }
            }
        }
    }
    let mut s = String::new();
    if other {
        s.push_str("Other ");
        if let Some(c) = color {
            s.push_str(super::effect::color_word(c));
            s.push(' ');
        }
        s.push_str(&base.to_lowercase());
    } else if let Some(c) = color {
        // A color qualifier rides the subject: "Black creatures get
        // +1/+1." — printed text, never dropped.
        s.push_str(&capitalize(super::effect::color_word(c)));
        s.push(' ');
        s.push_str(&base.to_lowercase());
    } else if typed && control.is_none() {
        // The set-wide unqualified subject prints the "All" quantifier —
        // "All creatures get -1/-1."
        s.push_str("All ");
        s.push_str(&base.to_lowercase());
    } else {
        s.push_str(&base);
    }
    if let Some(c) = control {
        s.push(' ');
        s.push_str(&c);
    }
    s
}

/// Recursively search a stripped filter for a `Characteristic(Type(t))`.
/// Used to find the type name inside a macro-expanded Creature/Land/etc.
/// filter.
pub(super) fn find_card_type(f: &Predicate) -> Option<deckmaste_core::Type> {
    match strip_expanded(f) {
        Predicate::Characteristic(CharacteristicPredicate::Type(t)) => Some(*t),
        Predicate::AllOf(vs) => vs.iter().find_map(find_card_type),
        _ => None,
    }
}

fn controller_phrase(f: &Predicate) -> String {
    match strip_expanded(f) {
        Predicate::Ref(Reference::You) => "you control".to_string(),
        Predicate::Relation(RelationPredicate::OpponentOf(inner))
            if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
        {
            "your opponents control".to_string()
        }
        Predicate::Relation(RelationPredicate::TeammateOf(inner))
            if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
        {
            "your teammates control".to_string()
        }
        other => format!("[unrendered: {other:?}]"),
    }
}

fn flatten_all_of(f: &Predicate) -> Vec<&Predicate> {
    match strip_expanded(f) {
        Predicate::AllOf(v) => v.iter().collect(),
        single => vec![single],
    }
}

// ── PutInLibrary helpers ─────────────────────────────────────────────────────

/// A `Selection` GROUP as the object of "put __": "them" for a bound group.
pub(super) fn quantity(q: &Quantity) -> String {
    // `Quantity` is one `Range(lo, hi)` primitive (seen through a remembered
    // macro by `bounds`). An exactly-N range renders as the object-count
    // word ("two cards", [`number_word`]); richer phrasings ("up to N",
    // "any number of") are a renderer follow-up.
    match q.bounds() {
        (Some(lo), Some(hi)) if lo == hi => match lo {
            Count::Literal(n) => number_word(*n).map_or_else(|| n.to_string(), str::to_string),
            other => count(other),
        },
        other => format!("[unrendered: {other:?}]"),
    }
}

/// A `Predicate` as the object noun for cards: "cards from your hand", or a
/// bare "cards" for the unqualified card kind.
pub(super) fn filter_object(f: &Predicate) -> String {
    // A bare card kind ([CR#108.2]) reads as the plain plural "cards".
    if matches!(strip_expanded(f), Predicate::Kind(ObjectKind::Card)) {
        return "cards".to_string();
    }
    let parts = flatten_all_of(f);
    let mut zone = "";
    let mut yours = false;
    for p in parts {
        match strip_expanded(p) {
            Predicate::State(StatePredicate::InZone(Zone::Hand)) => zone = "hand",
            Predicate::Relation(RelationPredicate::Owner(inner))
                if matches!(strip_expanded(inner), Predicate::Ref(Reference::You)) =>
            {
                yours = true;
            }
            _ => {}
        }
    }
    match (zone, yours) {
        ("hand", true) => "cards from your hand".to_string(),
        ("hand", false) => "cards from a hand".to_string(),
        _ => format!("cards [unrendered: {f:?}]"),
    }
}

/// Library position from an [`Anchor`] ([CR#401.7]): `FromTop(0)` -> "top",
/// `FromBottom(0)` -> "the bottom"; deeper offsets fall back to a generic
/// phrase.
pub(super) fn library_position(anchor: &Anchor) -> String {
    match anchor {
        Anchor::FromTop(Count::Literal(0)) => "top".to_string(),
        Anchor::FromBottom(Count::Literal(0)) => "the bottom".to_string(),
        Anchor::FromTop(c) => format!("{} from the top", count(c)),
        Anchor::FromBottom(c) => format!("{} from the bottom", count(c)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> Ctx<'static> {
        Ctx {
            subject: "Grizzly Bears",
            targets: &[],
            that: None,
        }
    }

    /// The provenance-explicit event roles render as English anaphora: the
    /// object/patient as "it", the actor as "that player", and the combat
    /// defender as "the defending player" ([CR#603.2e,608.2k,506.2]).
    #[test]
    fn event_role_references_render() {
        let c = ctx();
        assert_eq!(reference(&Reference::EventObject, &c), "it");
        assert_eq!(reference(&Reference::EventPatient, &c), "it");
        assert_eq!(reference(&Reference::EventActor, &c), "that player");
        assert_eq!(
            reference(&Reference::DefendingPlayer, &c),
            "the defending player"
        );
    }

    /// `Reference::It` — the `Each`/`Distribute` element — reads the enclosing
    /// binder's phrase from `ctx.that`; at a frameless position it is the
    /// wildcard stack anaphor and prints the plain pronoun ([CR#608]).
    #[test]
    fn it_anaphor_reads_binder_phrase_else_marks() {
        let frameless = ctx(); // that: None
        assert_eq!(reference(&Reference::It, &frameless), "it");
        let scoped = Ctx {
            subject: "Grizzly Bears",
            targets: &[],
            that: Some("each creature"),
        };
        assert_eq!(reference(&Reference::It, &scoped), "each creature");
    }

    /// The new filter nouns: an ability on the stack, and the team-relative
    /// player relations relative to "you".
    #[test]
    fn filter_noun_renders_ability_and_team_relative_players() {
        let you = || Box::new(Predicate::Ref(Reference::You));
        assert_eq!(
            filter_noun(&Predicate::Kind(ObjectKind::Ability)),
            "ability"
        );
        assert_eq!(
            filter_noun(&Predicate::Relation(RelationPredicate::OpponentOf(you()))),
            "opponent"
        );
        assert_eq!(
            filter_noun(&Predicate::Relation(RelationPredicate::TeammateOf(you()))),
            "teammate"
        );
    }

    /// "creatures your teammates control" — the team-relative controller phrase
    /// inside a plural subject ([CR#102.3]).
    #[test]
    fn filter_subject_renders_teammate_controller_phrase() {
        let f = Predicate::AllOf(vec![
            Predicate::Characteristic(CharacteristicPredicate::Type(
                deckmaste_core::Type::Creature,
            )),
            Predicate::Relation(RelationPredicate::ControlledBy(Box::new(
                Predicate::Relation(RelationPredicate::TeammateOf(Box::new(Predicate::Ref(
                    Reference::You,
                )))),
            ))),
        ]);
        assert_eq!(filter_subject(&f), "Creatures your teammates control");
    }
}
