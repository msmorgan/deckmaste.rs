//! Shared noun-phrase / count fragment renderers.

use deckmaste_semantics::AggregateOp;
use deckmaste_semantics::Anchor;
use deckmaste_semantics::Characteristic;
use deckmaste_semantics::CharacteristicPredicate;
use deckmaste_semantics::Color;
use deckmaste_semantics::Count;
use deckmaste_semantics::Countable;
use deckmaste_semantics::ObjectKind;
use deckmaste_semantics::PlayerAttr;
use deckmaste_semantics::Predicate;
use deckmaste_semantics::Projection;
use deckmaste_semantics::Quantity;
use deckmaste_semantics::Reference;
use deckmaste_semantics::RelationPredicate;
use deckmaste_semantics::RoundMode;
use deckmaste_semantics::Selection;
use deckmaste_semantics::Stat;
use deckmaste_semantics::StatePredicate;
use deckmaste_semantics::Status;
use deckmaste_semantics::SymbolPred;
use deckmaste_semantics::TargetSpec;
use deckmaste_semantics::Zone;

use super::Ctx;

type SemValue = Count;

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
        SemValue::X => "X".to_string(),
        Count::Damage(r) => format!("damage marked on {}", reference(r, &it_ctx())),
        // [CR#122.1]: the counter-count read — "the number of experience
        // counters you have" (a player-borne kind), "the number of lore
        // counters on it" (object-borne).
        Count::CounterCount(r, kind) => {
            let noun = counter_noun(kind.as_str());
            match r.as_ref() {
                Reference::You => format!("the number of {noun} counters you have"),
                other => format!(
                    "the number of {noun} counters on {}",
                    reference(other, &it_ctx())
                ),
            }
        }
        // [CR#119.1,402.2]: a player's numeric attribute — "your life total",
        // "the number of cards in that player's hand".
        Count::PlayerStatOf(r, attr) => {
            let who = reference(
                r,
                &Ctx {
                    subject: "that player",
                    targets: &[],
                    that: None,
                    named: None,
                },
            );
            match attr {
                PlayerAttr::Life => format!("{who}'s life total"),
                PlayerAttr::HandSize => format!("the number of cards in {who}'s hand"),
                PlayerAttr::HandSizeLimit => format!("{who}'s maximum hand size"),
                PlayerAttr::LandPlaysPerTurn => format!("the number of lands {who} can play"),
            }
        }
        // [CR#102.1]: opponent count — "the number of opponents you have".
        Count::Opponents(r) => match r {
            Reference::You => "the number of opponents you have".to_string(),
            other => format!(
                "the number of opponents {} has",
                reference(
                    other,
                    &Ctx {
                        subject: "that player",
                        targets: &[],
                        that: None,
                        named: None,
                    }
                )
            ),
        },
        // [CR#106.4]: the floated-mana-pool reader (a strategy sensing
        // source) — "the amount of unspent mana you have".
        Count::ManaAvailable(r) => match r {
            Reference::You => "the amount of unspent mana you have".to_string(),
            other => format!(
                "the amount of unspent mana {} has",
                reference(
                    other,
                    &Ctx {
                        subject: "that player",
                        targets: &[],
                        that: None,
                        named: None,
                    }
                )
            ),
        },
        Count::ManaAvailableKind(r, kind) => {
            let symbol = super::card::color_letter(*kind);
            match r {
                Reference::You => format!("the amount of unspent {{{symbol}}} mana you have"),
                other => format!(
                    "the amount of unspent {{{symbol}}} mana {} has",
                    reference(
                        other,
                        &Ctx {
                            subject: "that player",
                            targets: &[],
                            that: None,
                            named: None,
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
        Count::Half(mode, inner) => format!("half {}, {}", count(inner), rounding_word(*mode)),
        // [CR#107.1a]: `Half`'s general twin — "X divided by Y, rounded ...".
        Count::Divide(mode, a, b) => format!(
            "{} divided by {}, {}",
            count(a),
            count(b),
            rounding_word(*mode)
        ),
        // [CR#107.1]: remainder — parity checks read `Compare(Mod(x, 2), Eq, 0)`.
        Count::Mod(a, b) => format!("the remainder of {} divided by {}", count(a), count(b)),
        // [CR#107.1]: exponentiation — doubling effects build `Pow(2, X)`; a
        // real card's own "double ~'s power X times" reads through the
        // dedicated `doubling_power_clause` recognizer instead, so this is
        // the structural fallback for any other `Pow` shape.
        Count::Pow(base, exp) => format!("{} raised to the power of {}", count(base), count(exp)),
        // [CR#115.9a]: how many times a referenced object was chosen as a
        // target when it was put on the stack (Strive).
        Count::TargetsOf(r) => format!(
            "the number of times {} was chosen as a target",
            reference(r, &it_ctx())
        ),
        // [CR#107.3] distinct-union count (Domain / Coven / Tarmogoyf). The
        // subtype axis over a typed group names the type's own subtype
        // family ("land types"); the group reads as its plural subject
        // phrase ("lands you control").
        // `ManaSymbols`-sourced distinct counts have no forced card yet — the
        // devotion-style phrasing lands with `Count::Aggregate` (a later
        // task); render structurally for now (YAGNI).
        Count::CountDistinct(_, Countable::ManaSymbols(..) | Countable::ManaSpentMatching(..)) => {
            format!("[unrendered: {c:?}]")
        }
        Count::CountDistinct(axis, Countable::Objects(filter)) => {
            let axis_word = match (axis, find_card_type(filter)) {
                (Characteristic::Subtypes, Some(t)) => {
                    format!("{} types", t.as_str().to_lowercase())
                }
                _ => characteristic_word(*axis).to_string(),
            };
            let group = super::ability::lower_first(&filter_subject(filter));
            format!("the number of {axis_word} among {group}")
        }
        // [CR#105.2]: the distinct-union axis read off a SINGLE object —
        // Embiggen's "number of card types it has" = `CountDistinct(Types,
        // Singleton(This))`. A real card's own triple-axis phrasing ("for
        // each supertype, card type, and subtype it has") reads through the
        // dedicated `axis_sum_pump_clause` recognizer instead; this is the
        // structural single-axis fallback.
        Count::CountDistinct(axis, Countable::Singleton(r)) => format!(
            "the number of {} {} has",
            characteristic_word(*axis),
            reference(r, &it_ctx())
        ),
        // [CR#107.3] "for each" — a plain filtered-object count read as a
        // bare noun (no "for each" text here; the caller — a pump's own
        // dedicated clause, `ability::for_each_pump_clause` — supplies
        // that). Structural fallback for any OTHER position a bare
        // `CountOf(Objects)` count appears (outside the pump family, which
        // has its own recognizer for the reason `Count::Aggregate`'s doc
        // comment above gives — the delta family reads only
        // `Count::Literal` today).
        Count::CountOf(Countable::Objects(filter)) => filter_noun(filter),
        // [CR#107.1] the fold over a projection — "the total/greatest/
        // least/average [by] among [of]". A devotion-shaped fold (`SumOf`
        // over permanents you control's matching mana symbols, [CR#700.5])
        // is recognized first and reads as "your devotion to <color>"; a
        // cross-player fold ([CR#119.1]) over a player's numeric attribute is
        // recognized next and reads "the highest life total among all
        // players" (Arbiter of Knollridge); every other shape stays
        // structural. A `ManaSymbols`/`Singleton`/`ManaSpentMatching`-sourced
        // `of` is not projectable — falls through to the generic
        // `[unrendered: …]`, as does a `Players`-sourced fold the recognizer
        // doesn't cover (a non-`Life` attribute, a non-"all players" group).
        Count::Aggregate(op, proj) => {
            match devotion_phrase(*op, proj).or_else(|| player_aggregate_phrase(*op, proj)) {
                Some(phrase) => phrase,
                None => match &proj.of {
                    Countable::Objects(filter) => {
                        let fold_word = match op {
                            AggregateOp::SumOf => "total",
                            AggregateOp::MinOf => "least",
                            AggregateOp::MaxOf => "greatest",
                            AggregateOp::AverageOf(_) => "average",
                        };
                        let group = super::ability::lower_first(&filter_subject(filter));
                        format!("the {fold_word} {} among {group}", count(&proj.by))
                    }
                    Countable::Players(..)
                    | Countable::ManaSymbols(..)
                    | Countable::Singleton(..)
                    | Countable::ManaSpentMatching(..) => format!("[unrendered: {c:?}]"),
                },
            }
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
pub(super) fn counter_noun(ident: &str) -> String {
    ident.trim_end_matches("Counter").to_lowercase()
}

/// The plural noun for a [`Characteristic`] axis, used by the distinct-count
/// phrase ("the number of subtypes among …").
/// A [`RoundMode`] as its "rounded ..." adverbial phrase — shared by `Half`
/// and `Divide`'s render arms.
fn rounding_word(mode: RoundMode) -> &'static str {
    match mode {
        RoundMode::RoundUp => "rounded up",
        RoundMode::RoundDown => "rounded down",
    }
}

/// The `Ctx` a bare pronominal read uses when no real subject/targets/that
/// context is in scope — "it" with nothing else bound. Shared by the handful
/// of `count()` arms (`Damage`, `CounterCount`, `TargetsOf`, `CountDistinct`
/// over a `Singleton`) that read a plain `Reference` this way.
fn it_ctx() -> Ctx<'static> {
    Ctx {
        subject: "it",
        targets: &[],
        that: None,
        named: None,
    }
}

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
        // The nth announced slot read as its whole group ([CR#115.3,601.2c]) —
        // prints the slot's own target phrase, exactly as the singular
        // `Reference::Target` does ("Arc Lightning deals 3 damage divided as
        // you choose among **one, two, or three targets**").
        Selection::Targets(n) => target_phrase(*n, ctx),
        // The plural anaphors ([CR#608.2d]): `They` reads the bound group's
        // noun phrase when an enclosing binder supplies one, else the bare
        // pronoun; `Them(sort)` names its sort ("those cards"). Neither ever
        // reads an announced target — that is `Targets(n)` above.
        Selection::They => ctx.that.unwrap_or("them").to_string(),
        Selection::Them(sort) => format!("those {}s", sort.noun()),
        Selection::PilesOf { of, .. } => {
            format!("the piles {} separated", reference(of, ctx))
        }
        // The top `count` cards of a graveyard ([CR#404.2] — Soldevi
        // Digger's "the top card of your graveyard"); `of: You` reads
        // "your", any other reference its possessive noun phrase.
        Selection::TopOfGraveyard { count: n, of } => {
            let owner = match of {
                Reference::You => "your".to_string(),
                other => format!("{}'s", reference(other, ctx)),
            };
            match n {
                Count::Literal(1) => format!("the top card of {owner} graveyard"),
                _ => format!("the top {} cards of {owner} graveyard", count(n)),
            }
        }
        // [CR#107.1] the extremal element: "the creature with the greatest
        // power". The projection's axis is named when it is a simple stat
        // read. A non-extremal `op` is malformed semantic input (fizzles at
        // resolution) and falls through to the generic `[unrendered: …]`; a
        // `ManaSymbols`-sourced `of` has no forced card yet.
        Selection::Pick { op, proj } => {
            let extreme = match op {
                AggregateOp::MaxOf => "greatest",
                AggregateOp::MinOf => "least",
                AggregateOp::SumOf | AggregateOp::AverageOf(_) => "",
            };
            match &proj.of {
                Countable::Objects(filter) if !extreme.is_empty() => {
                    let axis = match proj.by.as_ref() {
                        Count::StatOf(_, Stat::Power) => " power",
                        Count::StatOf(_, Stat::Toughness) => " toughness",
                        _ => "",
                    };
                    format!("the {} with the {extreme}{axis}", filter_noun(filter))
                }
                _ => format!("[unrendered: {sel:?}]"),
            }
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
        // The nth announced target ([CR#115.3,601.2c]). English announces a
        // slot once and pronominalizes afterwards, so the FIRST read of slot
        // `n` in an ability prints the announce phrase ("target creature you
        // control") and every later read prints "that creature". The model
        // itself draws no such distinction — every read is the same indexed
        // slot — so the announce/re-mention register lives here.
        Reference::Target(n) => {
            if ctx.announce(*n) {
                target_phrase(*n, ctx)
            } else {
                target_rementioned(*n, ctx)
            }
        }
        // `It`: an `Each`/`Distribute` element reads the binder's noun phrase
        // from the shared `ctx.that` slot ([CR#601.2b,608]); otherwise it is
        // the wildcard anaphor — the plain English pronoun. It never reads an
        // announced target: a target prints through `Target(n)` above.
        Reference::It => ctx.that.map_or_else(|| "it".to_string(), str::to_string),
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
        // The first-non-null payer read of a Rhystic-toll ([CR#118.12a]): the
        // targeted permanent's controller ([CR#109.4]), or the targeted player
        // itself. Both branches resolve to a PLAYER — the toll is always paid
        // by a player — so the disjunction collapses to the single anaphor
        // "that player" (coincidentally the same phrase `EventActor` prints,
        // for the unrelated reason that a responsible player is also "that
        // player"). The engine, not the renderer, picks the live branch
        // (`eval_reference`); here it is one player either way.
        #[expect(
            clippy::match_same_arms,
            reason = "distinct references (an event's responsible player vs. a coalesced \
                      toll-payer) that happen to share the 'that player' anaphor; kept apart \
                      for their separate semantics and documentation"
        )]
        Reference::Coalesce(_) => "that player".to_string(),
        other => format!("[unrendered: {other:?}]"),
    }
}

/// The i-th announced slot's ANNOUNCE phrase — "target creature", "any
/// target" ([CR#115.3,601.2c]).
fn target_phrase(i: usize, ctx: &Ctx) -> String {
    match ctx.targets.get(i) {
        Some(spec) => target_spec(spec),
        None => "[unrendered: missing target]".to_string(),
    }
}

/// The i-th announced slot as a RE-MENTION — "that player", "that creature".
///
/// English announces a target once and pronominalizes every later mention of
/// it ("Separate all creatures **target player** controls into two piles.
/// Destroy all creatures in the pile of **that player**'s choice."). The RON
/// carries no such distinction — both reads are the same `Target(i)`, because a
/// slot is one indexed entry however often it is named — so the register is the
/// renderer's to supply, not the model's.
///
/// The noun comes from the slot's own filter. A slot with no single head noun
/// (`AnyTarget`'s player/permanent disjunction, [CR#115.4]) has no "that
/// <noun>" to print, so it falls back to the bare pronoun.
fn target_rementioned(i: usize, ctx: &Ctx) -> String {
    let noun = ctx
        .targets
        .get(i)
        .and_then(target_spec_filter)
        .and_then(slot_noun);
    noun.map_or_else(|| "it".to_string(), |n| format!("that {n}"))
}

/// The number-aware nominative PRONOUN for a re-mention of `raw` (a `Reference`
/// or `Selection` arg) — the render of a `${n:pro}` template slot. "it" for a
/// single object, "they" for a plural group.
///
/// This is the *pronominal* re-mention, distinct from [`target_rementioned`]'s
/// *demonstrative* one ("that creature"): English uses the bare pronoun for an
/// immediate next-sentence back-reference ("Destroy target creature. **It**
/// can't be regenerated."; "Destroy all creatures. **They** can't be
/// regenerated.") and reserves "that <noun>" for a distal or disambiguating
/// mention. Both are the same indexed slot in the model — the register is the
/// renderer's to supply ([CR#608.2d]).
///
/// Number is a property of the arg's KIND, not the slot: a `Reference` names a
/// single object ([CR#115.4] — a target is one object; a plural target is read
/// as `Selection::Targets`) → "it"; a plural `Selection` group → "they". The
/// lowercase pronoun is returned; sentence-initial capitalization is the
/// template filler's job ([`super::template::fill_with`]), not this layer's.
/// A shape this layer does not classify (a player reference, whose pronoun is
/// "they" not "it"; a count-bearing or singleton `Selection`) declines
/// (`None`) — the caller falls back to structural rendering, never a wrong
/// pronoun.
pub(super) fn reference_pronoun(raw: &str, ctx: &Ctx) -> Option<String> {
    let opts = deckmaste_semantics::ron::options();
    if let Ok(r) = opts.from_str::<Reference>(raw) {
        return reference_object_pronoun(&r, ctx);
    }
    if let Ok(sel) = opts.from_str::<Selection>(raw) {
        return selection_is_plural(&sel).map(|plural| pronoun_word(plural).to_string());
    }
    None
}

/// "they" for a plural referent, "it" for a singular one — the nominative
/// pronoun by number.
fn pronoun_word(plural: bool) -> &'static str {
    if plural { "they" } else { "it" }
}

/// A singular `Reference`'s nominative pronoun for a re-mention: "it" for an
/// OBJECT reference. A player reference (a responsible player, a derived
/// controller/owner) has no object pronoun — its re-mention is "they", handled
/// when a card needs it — so it declines (`None`) rather than mis-render "it".
fn reference_object_pronoun(r: &Reference, ctx: &Ctx) -> Option<String> {
    match r {
        // A target slot: "it" for an object slot; a player slot declines (its
        // pronoun is "they", a follow-up). A slot index past the announce list
        // has no filter to inspect and reads as an object re-mention.
        Reference::Target(n) => match ctx
            .targets
            .get(*n)
            .and_then(target_spec_filter)
            .and_then(slot_noun)
        {
            Some(noun) if noun == "player" => None,
            _ => Some("it".to_string()),
        },
        // The source object, the stack/element anaphor, a triggering event's
        // object/patient — all single objects → "it".
        Reference::This | Reference::It | Reference::EventObject | Reference::EventPatient => {
            Some("it".to_string())
        }
        // Player references (responsible player, defending player, derived
        // controller/owner, coalesced toll-payer, `You`) have no object pronoun
        // yet — decline.
        _ => None,
    }
}

/// Whether a [`Selection`] group reads as grammatically plural ("they") or
/// singular ("it") for a pronominal re-mention. `None` for a shape whose number
/// this layer does not yet classify — the caller declines rather than guess.
/// Only the plural GROUP reads a mass re-mention uses today are classified; the
/// count-bearing and singleton selections graduate with the paths that need
/// them.
fn selection_is_plural(sel: &Selection) -> Option<bool> {
    match sel {
        Selection::They
        | Selection::Them(_)
        | Selection::SelectAll(_)
        | Selection::Union(_)
        | Selection::PilesOf { .. } => Some(true),
        Selection::Expanded(e) => selection_is_plural(&e.value),
        _ => None,
    }
}

/// The head noun of an announced slot's filter, for a re-mention's "that
/// <noun>": a card type ("creature", "artifact"), or the player kind.
fn slot_noun(filter: &Predicate) -> Option<String> {
    if flatten_all_of(filter)
        .into_iter()
        .any(|p| matches!(strip_expanded(p), Predicate::Kind(ObjectKind::Player)))
    {
        return Some("player".to_string());
    }
    find_card_type(filter).map(|t| t.as_str().to_ascii_lowercase())
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
    // A graveyard-scoped card ([`graveyard_card_filter`]'s shape, migrations
    // effect.rs) rides the noun as a trailing "card" — a graveyard object is
    // a card, not a permanent, so "target creature card"/"target card" reads
    // distinctly from the battlefield-scoped "target creature". The zone
    // itself ("from your graveyard") is NOT this function's job — that
    // clause is the caller's static boilerplate (the graveyard-recursion
    // family, [CR#400.7]).
    let is_graveyard_card = is_graveyard_scoped(filter);
    // The base noun: a card TYPE atom ("creature") or, failing that, a bare
    // macro-provenance noun among the `And` parts ("permanent" — the
    // `Permanent` filter macro has no `Type(_)` atom of its own to key off
    // of, [CR#110.1]; "target nonland permanent", Avarice Totem), or (for a
    // graveyard-scoped filter with no type qualifier) the bare "card" noun.
    let base_noun = find_card_type(filter)
        .map(|t| t.as_str().to_lowercase())
        .or_else(|| find_bare_subtype_noun(filter))
        .or_else(|| find_macro_noun(filter))
        .or_else(|| is_graveyard_card.then(|| "card".to_string()));
    if let Some(base) = base_noun {
        // A negated-subtype/-card-type exclusion rides the noun as a prefix
        // ([CR#205.2,205.3] — subtype "non-Brushwagg creature"; a card-type
        // exclusion elides the hyphen, "nonland permanent"), ahead of any controller
        // suffix.
        let base = match subtype_exclusion_prefix(filter).or_else(|| type_exclusion_prefix(filter))
        {
            Some(prefix) => format!("{prefix} {base}"),
            None => base,
        };
        // The trailing "card" noun (skipped when the base noun already IS
        // "card" — the untyped graveyard case above).
        let base = if is_graveyard_card && base != "card" { format!("{base} card") } else { base };
        // A keyword-quality restrictor rides the noun: "creature with
        // flying" / "creature without flying".
        let base = match keyword_quality_suffix(filter) {
            Some(suffix) => format!("{base} {suffix}"),
            None => base,
        };
        // A combat/tap-state adjective rides directly ahead of the noun:
        // "attacking Goblin" (Goblin Piledriver, [CR#508.1a]), "untapped
        // creature" (Knotvine Paladin, [CR#110.5]).
        let base = match adjective_adjunct(filter) {
            Some(adj) => format!("{adj} {base}"),
            None => base,
        };
        // The self-exclusion "other" prefix rides outermost — ahead of any
        // adjective adjunct too ([CR#205.3g]-style: "for each other
        // attacking Goblin", never "attacking other Goblin").
        let base = match self_exclusion_prefix(filter) {
            Some(prefix) => format!("{prefix} {base}"),
            None => base,
        };
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
        // A bare supertype standing alone as a card descriptor ([CR#205.4a]):
        // "snow" / "legendary" — the type-gated look-at-top peek's `${0}`
        // classifies "a snow card" to `Supertype(Snow)`, which renders back to
        // the lowercase supertype word (no accompanying type noun).
        Predicate::Characteristic(CharacteristicPredicate::Supertype(s)) => {
            super::card::supertype_str(*s).to_lowercase()
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

/// The single keyword-quality phrase among a filter's `And` parts.
fn keyword_quality_suffix(filter: &Predicate) -> Option<String> {
    for part in flatten_all_of(filter) {
        match strip_expanded(part) {
            Predicate::Characteristic(CharacteristicPredicate::Has(keyword)) => {
                return Some(format!("with {}", keyword.as_str().to_lowercase()));
            }
            Predicate::Not(inner) => {
                if let Predicate::Characteristic(CharacteristicPredicate::Has(keyword)) =
                    strip_expanded(inner)
                {
                    return Some(format!("without {}", keyword.as_str().to_lowercase()));
                }
            }
            _ => {}
        }
    }
    None
}

/// A combat/tap-state adjective among a filter's `And` parts, read as an
/// adjunct riding directly ahead of the base noun ([CR#508.1a] "attacking",
/// [CR#110.5] "untapped") — Goblin Piledriver's `Attacking` half of "for each
/// other attacking Goblin", Knotvine Paladin's `Status(Untapped)` half of
/// "for each untapped creature you control". `None` when the filter carries
/// neither atom.
fn adjective_adjunct(filter: &Predicate) -> Option<&'static str> {
    for part in flatten_all_of(filter) {
        match strip_expanded(part) {
            Predicate::State(StatePredicate::Attacking) => return Some("attacking"),
            Predicate::State(StatePredicate::Status(Status::Untapped)) => return Some("untapped"),
            _ => {}
        }
    }
    None
}

/// The self-exclusion "other" prefix among a filter's `And` parts
/// ([CR#205.3g]-style anaphora, "each OTHER creature") — `Not(Ref(This))` ->
/// "other", read ahead of the base noun (and any adjective adjunct):
/// Goblin Piledriver's "for each other attacking Goblin". Bare-noun register
/// only — mid-sentence subject position reads "another" instead
/// ([`subject_phrase`]'s `SingularArticle` register), a different function's
/// job. `None` when the filter carries no self-exclusion.
fn self_exclusion_prefix(filter: &Predicate) -> Option<&'static str> {
    for part in flatten_all_of(filter) {
        if let Predicate::Not(inner) = strip_expanded(part)
            && strip_expanded(inner).is_this()
        {
            return Some("other");
        }
    }
    None
}

/// The controller-restrictor phrase among a filter's `And` parts:
/// `ControlledBy(You)` → "you control"; `Not(ControlledBy(You))` → "you
/// don't control"; `ControlledBy(OpponentOf(You))` → "an opponent controls";
/// `ControlledBy(TeammateOf(You))` → "a teammate controls". `None` when the
/// filter carries no controller part.
fn controller_suffix(filter: &Predicate) -> Option<&'static str> {
    for part in flatten_all_of(filter) {
        match strip_expanded(part) {
            Predicate::Relation(RelationPredicate::ControlledBy(inner)) => {
                return singular_controller_suffix(inner);
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

/// The singular-register text for a `ControlledBy`'s player-predicate
/// argument: `Ref(You)` → "you control", `OpponentOf(Ref(You))` → "an
/// opponent controls", `TeammateOf(Ref(You))` → "a teammate controls" —
/// mirroring `turn_owner`'s (condition.rs) article+noun convention for
/// opponent/teammate ("an opponent's turn"/"a teammate's turn"), not
/// [`controller_phrase`]'s plural "your opponents/teammates control". `None`
/// for any other shape — both [`controller_suffix`] and [`subject_phrase`]'s
/// `SingularArticle` register fall back to printing no controller suffix at
/// all in that case, rather than duplicating this match a third time.
fn singular_controller_suffix(inner: &Predicate) -> Option<&'static str> {
    match strip_expanded(inner) {
        Predicate::Ref(Reference::You) => Some("you control"),
        Predicate::Relation(RelationPredicate::OpponentOf(who))
            if matches!(strip_expanded(who), Predicate::Ref(Reference::You)) =>
        {
            Some("an opponent controls")
        }
        Predicate::Relation(RelationPredicate::TeammateOf(who))
            if matches!(strip_expanded(who), Predicate::Ref(Reference::You)) =>
        {
            Some("a teammate controls")
        }
        _ => None,
    }
}

/// A negated-subtype exclusion among a filter's `And` parts ([CR#205.3]):
/// `Not(Subtype("Brushwagg"))` -> "non-Brushwagg", prefixed onto the base
/// noun ("non-Brushwagg creature", Embiggen). `None` when the filter carries
/// no such exclusion.
fn subtype_exclusion_prefix(filter: &Predicate) -> Option<String> {
    for part in flatten_all_of(filter) {
        if let Predicate::Not(negated) = strip_expanded(part)
            && let Predicate::Characteristic(CharacteristicPredicate::Subtype(name)) =
                strip_expanded(negated)
        {
            return Some(format!("non-{}", name.name()));
        }
    }
    None
}

/// A negated CARD-TYPE exclusion among a filter's `And` parts ([CR#205.2]):
/// `Not(Type(Land))` -> "nonland", prefixed onto the base noun ("nonland
/// permanent", Avarice Totem). Unlike [`subtype_exclusion_prefix`]'s
/// hyphenated "non-Brushwagg", a card-type exclusion elides the hyphen —
/// oracle templating's own convention ("nonland", "noncreature",
/// "nonbasic"). `None` when the filter carries no such exclusion.
fn type_exclusion_prefix(filter: &Predicate) -> Option<String> {
    for part in flatten_all_of(filter) {
        if let Predicate::Not(negated) = strip_expanded(part)
            && let Predicate::Characteristic(CharacteristicPredicate::Type(t)) =
                strip_expanded(negated)
        {
            return Some(format!("non{}", t.name().as_str().to_lowercase()));
        }
    }
    None
}

/// A bare macro-provenance noun among a filter's `And` parts — the
/// non-card-type twin of [`find_card_type`]: `Permanent`'s
/// `InZone(Battlefield)` expansion carries no `Type(_)` atom of its own, so the
/// noun comes off its template ("permanent") instead. `None` for a filter with
/// no such macro part (or one whose macro carries no nullary template).
pub(super) fn find_macro_noun(f: &Predicate) -> Option<String> {
    match f {
        Predicate::Expanded(exp) => super::template::expanded(exp, ""),
        Predicate::And(parts) => parts.iter().find_map(find_macro_noun),
        _ => None,
    }
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
        // An announced slot as a sentence subject ("Target creature gets
        // +3/+3 …") — named by position ([CR#115.3,601.2c]).
        Reference::Target(n) => capitalize(&target_phrase(*n, ctx)),
        // Aura host: "Enchanted creature gets +2/+2." (matches deontic_subject).
        Reference::AttachHostOf(inner) if matches!(**inner, Reference::This) => {
            "Enchanted creature".to_string()
        }
        // The sorted anaphor as a sentence subject: "That creature can't block
        // this turn." ([CR#608.2d]; capitalized for sentence-start, mirroring
        // the noun-phrase `reference` arm but at subject position).
        Reference::That(sort) => ctx
            .that
            .map_or_else(|| format!("That {}", sort.noun()), capitalize),
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

/// Grammatical register [`subject_phrase`]'s two callers need from the same
/// filter-qualifier walk: [`filter_subject`]'s sentence-start plural subject
/// ("Other creatures you control") vs. `ability::subject_of`'s mid-sentence
/// singular subject ("another creature you control").
#[derive(Clone, Copy)]
pub(super) enum SubjectNumber {
    /// Capitalized plural lead — [`filter_subject`]'s existing register.
    PluralCapitalized,
    /// Lowercase singular with an article/"another" determiner —
    /// `ability::subject_of`'s register.
    SingularArticle,
}

/// The filter-qualifier walk shared by [`filter_subject`] (plural) and
/// `ability::subject_of` (singular): self-exclusion (`Not(Ref(This))` →
/// "another"/"Other"), a color qualifier, the base type noun, and the
/// controller suffix (`ControlledBy(..)`). The controller suffix is
/// register-sensitive — resolved per-register at composition time, not
/// pre-rendered in this loop: [`controller_phrase`] for
/// `PluralCapitalized` ("you control"/"your opponents control"/"your
/// teammates control") vs. [`singular_controller_suffix`] for
/// `SingularArticle` ("you control"/"an opponent controls"/"a teammate
/// controls"). Self-exclusion prints once even if `Not(Ref(This))` repeats
/// among the filter's `And` parts — `other` is a flag, not a string append.
///
/// The Creature filter macro expands as
/// `Expanded(value=And([Expanded(Permanent),
/// Characteristic(Type(Creature))]))`. `flatten_all_of` and `find_card_type`
/// see through both layers.
///
/// `None` only when the filter carries NONE of the four qualifiers above — a
/// shape this walk has nothing to say about (a bare player reference,
/// `Predicate::Any`, …). [`filter_subject`] maps that to its own
/// "Permanent(s)" default (unreachable by any filter a real `SelectAll` uses
/// today, so this never changes its output); `subject_of` maps it to its own
/// `[unrendered: ..]` marker.
pub(super) fn subject_phrase(f: &Predicate, number: SubjectNumber) -> Option<String> {
    let parts = flatten_all_of(f);
    let mut other = false;
    let mut base: Option<String> = None;
    let mut subtype_base = false;
    let mut color: Option<Color> = None;
    let mut control: Option<&Predicate> = None;
    for p in parts {
        match strip_expanded(p) {
            Predicate::Characteristic(CharacteristicPredicate::Type(t)) => {
                base = Some(t.name().as_str().to_string());
            }
            Predicate::Characteristic(CharacteristicPredicate::ColorIs(c)) => {
                color = Some(*c);
            }
            Predicate::Not(inner) if strip_expanded(inner).is_this() => {
                other = true;
            }
            Predicate::Relation(RelationPredicate::ControlledBy(inner)) => {
                control = Some(inner.as_ref());
            }
            // No arm for Not(ControlledBy(..)) ("you don't control") —
            // pre-existing gap in the walk (filter_subject lacked it too
            // before this fn existed), out of scope here.
            // The Creature macro expands to And([Expanded(Permanent),
            // Characteristic(Type(Creature))]); check whether this part holds
            // a card type buried in a nested And.
            stripped => {
                if let Some(t) = find_card_type(stripped) {
                    base = Some(t.as_str().to_string());
                }
            }
        }
    }
    // A subtype is the printed head noun when no card-type atom is present:
    // `And([Permanent, Subtype(Ally), Not(This), ControlledBy(You)])` is
    // "another Ally you control", not the lossy "another permanent you
    // control". Card type keeps precedence when both are present, matching
    // `filter_noun`'s established ordering.
    if base.is_none() {
        base = find_bare_subtype_noun(f);
        subtype_base = base.is_some();
    }
    if base.is_none() && !other && color.is_none() && control.is_none() {
        return None;
    }
    let typed = base.is_some();
    let noun = base.unwrap_or_else(|| "Permanent".to_string());
    Some(match number {
        SubjectNumber::PluralCapitalized => {
            let plural = format!("{noun}s");
            let mut s = String::new();
            if other {
                s.push_str("Other ");
                if let Some(c) = color {
                    s.push_str(super::effect::color_word(c));
                    s.push(' ');
                }
                s.push_str(&plural.to_lowercase());
            } else if let Some(c) = color {
                // A color qualifier rides the subject: "Black creatures get
                // +1/+1." — printed text, never dropped.
                s.push_str(&capitalize(super::effect::color_word(c)));
                s.push(' ');
                s.push_str(&plural.to_lowercase());
            } else if typed && control.is_none() {
                // The set-wide unqualified subject prints the "All"
                // quantifier — "All creatures get -1/-1."
                s.push_str("All ");
                s.push_str(&plural.to_lowercase());
            } else {
                s.push_str(&plural);
            }
            if let Some(c) = control {
                s.push(' ');
                s.push_str(&controller_phrase(c));
            }
            s
        }
        SubjectNumber::SingularArticle => {
            let lower = if subtype_base { noun } else { noun.to_lowercase() };
            let described = match color {
                Some(c) => format!("{} {lower}", super::effect::color_word(c)),
                None => lower,
            };
            let head = if other {
                format!("another {described}")
            } else {
                super::effect::a_an(&described)
            };
            match control.and_then(singular_controller_suffix) {
                Some(suffix) => format!("{head} {suffix}"),
                None => head,
            }
        }
    })
}

/// A `Predicate` as a plural subject noun phrase: "Creatures you control",
/// "Other creatures you control", "Creatures your opponents control".
pub(super) fn filter_subject(f: &Predicate) -> String {
    subject_phrase(f, SubjectNumber::PluralCapitalized).unwrap_or_else(|| "Permanents".to_string())
}

/// Recursively search a stripped filter for a `Characteristic(Type(t))`.
/// Used to find the type name inside a macro-expanded Creature/Land/etc.
/// filter.
pub(super) fn find_card_type(f: &Predicate) -> Option<deckmaste_semantics::Ident> {
    match strip_expanded(f) {
        Predicate::Characteristic(CharacteristicPredicate::Type(t)) => Some(t.name()),
        Predicate::And(vs) => vs.iter().find_map(find_card_type),
        _ => None,
    }
}

/// A bare (non-negated) subtype atom among a filter's `And` parts
/// ([CR#205.3]), read as the base noun ONLY when [`find_card_type`] found no
/// card-TYPE atom to key off of — a basic land type doubles as its own noun
/// with no accompanying `Type(Land)` atom in a "for each" selection filter
/// ("for each Forest you control", Primal Bellow's `And([Permanent,
/// Subtype("Forest"), ControlledBy(You)])` — the `Permanent` macro carries no
/// `Type(_)` atom of its own, [CR#110.1]). The "other attacking Goblin" case
/// is the SAME shape, not a Type-having one: Goblin Piledriver's own filter
/// is `And([Permanent, Subtype("Goblin"), Not(Ref(This)), Attacking])` —
/// `Permanent` carries no `Type(_)` atom either, so `find_card_type` returns
/// `None` and THIS function is what supplies "Goblin". The latent edge runs
/// the other way: if a filter ever carried BOTH `Type(Creature)` and a bare
/// `Subtype`, `find_card_type` would win (it's tried first in
/// [`filter_noun`]'s `or_else` chain) and the noun would degrade (e.g.
/// "creature" instead of "Goblin") — safe today only because pump/for-each
/// selections are authored `Permanent`+`Subtype`, never `Creature`+
/// `Subtype`. `None` when the filter carries no bare `Subtype(_)` atom (or
/// only a negated one — [`subtype_exclusion_prefix`]'s job, not this
/// noun's).
fn find_bare_subtype_noun(f: &Predicate) -> Option<String> {
    match strip_expanded(f) {
        Predicate::Characteristic(CharacteristicPredicate::Subtype(name)) => {
            Some(name.name().as_str().to_string())
        }
        Predicate::And(vs) => vs.iter().find_map(find_bare_subtype_noun),
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

pub(super) fn flatten_all_of(f: &Predicate) -> Vec<&Predicate> {
    match strip_expanded(f) {
        Predicate::And(v) => v.iter().collect(),
        single => vec![single],
    }
}

/// Whether `filter` carries an `InZone(Graveyard)` atom — shared by
/// `filter_noun`'s graveyard-card "card" noun and the exile-from-a-graveyard
/// render arm (effect.rs's `Action::Move(_, Exile, _, None)`), which needs to
/// know whether to append "from a graveyard" without re-deriving this zone
/// check.
/// Owner-agnostic by design: both the your-graveyard recursion family's filter
/// and the any-graveyard exile family's filter carry this same atom.
pub(super) fn is_graveyard_scoped(filter: &Predicate) -> bool {
    flatten_all_of(filter).into_iter().any(|p| {
        matches!(
            strip_expanded(p),
            Predicate::State(StatePredicate::InZone(Zone::Graveyard))
        )
    })
}

/// The `Predicate` a `TargetSpec` slot filters on, peeling macro-provenance
/// (`Expanded`) and the co-target wrapper (`Distinct`) to reach the leaf
/// `Target(Quantity, Predicate)`.
fn target_spec_filter(spec: &TargetSpec) -> Option<&Predicate> {
    match spec {
        TargetSpec::Target(_, filter) => Some(filter),
        TargetSpec::Distinct(_, inner) => target_spec_filter(inner),
        TargetSpec::Expanded(exp) => target_spec_filter(&exp.value),
    }
}

/// The filter behind the reference `r` when it names an announced target slot
/// ([CR#115.3,601.2c]) — `None` when `r` isn't a `Target(n)`, or names a slot
/// this ability didn't announce. Used by the exile-from-a-graveyard render arm
/// to decide whether the target's filter is graveyard-scoped. A positional
/// read, so no cardinality test and no anaphor-precedence rule to mirror.
pub(super) fn target_slot_filter<'a>(r: &Reference, ctx: &'a Ctx) -> Option<&'a Predicate> {
    let &Reference::Target(n) = r else {
        return None;
    };
    target_spec_filter(ctx.targets.get(n)?)
}

/// The English possessive for a hand/library zone-move destination
/// ([CR#400.3]/[CR#402.1]): "your" when the moved object is provably the
/// controller's, "its owner's" for a targeted permanent that could be an
/// opponent's (the Unsummon/Excommunicate bounce family).
///
/// A bare `Move` destination carries no owner qualifier, so the possessive is
/// inferred from the reference SHAPE. Only a `Target(n)` slot on a
/// NON-graveyard filter is possibly-foreign: a self-bounce (`This`), a
/// search/reveal anaphor (`It`/`That`), and a graveyard-scoped target (cards in
/// a graveyard are owned by that graveyard's player) all name a provably-owned
/// object. A target slot this ability didn't announce is treated as
/// possibly-foreign — the safe default for a targeted permanent.
///
/// The chosen-subject "a creature you control" bounce prints "its owner's" via
/// its own `With`-composition arm (you control it but may not own it), not this
/// reference-shape helper.
pub(super) fn move_possessive(r: &Reference, ctx: &Ctx) -> &'static str {
    let targeted_foreign = matches!(r, Reference::Target(_))
        && !target_slot_filter(r, ctx).is_some_and(is_graveyard_scoped);
    if targeted_foreign { "its owner's" } else { "your" }
}

// ── Devotion recognizer ([CR#700.5]) ────────────────────────────────────────

/// Whether `filter` is (up to macro provenance) exactly "permanents you
/// control" — `And([Permanent, ControlledBy(Ref(You))])`, i.e.
/// `InZone(Battlefield)` + `ControlledBy(Ref(You))` and nothing else. A
/// narrower subset (e.g. "creatures you control") is a different fold, not
/// devotion.
fn is_permanents_you_control(filter: &Predicate) -> bool {
    let parts = flatten_all_of(filter);
    if parts.len() != 2 {
        return false;
    }
    let has_battlefield = parts.iter().any(|p| {
        matches!(
            strip_expanded(p),
            Predicate::State(StatePredicate::InZone(Zone::Battlefield))
        )
    });
    let has_you_control = parts.iter().any(|p| {
        matches!(
            strip_expanded(p),
            Predicate::Relation(RelationPredicate::ControlledBy(inner))
                if matches!(strip_expanded(inner), Predicate::Ref(Reference::You))
        )
    });
    has_battlefield && has_you_control
}

/// A devotion `SymbolPred` as its English color word(s): a single
/// `CountsAs(c)` → its color word; `Or([CountsAs(c1), CountsAs(c2), ...])` →
/// an English list ("white and black", "white, blue, and black"). `None` for
/// any other shape (e.g. `AnyColor`, `And`, `Not`) — devotion's oracle
/// wording only ever names a color or a color disjunction.
fn devotion_color_words(pred: &SymbolPred) -> Option<String> {
    let colors: Vec<Color> = match pred {
        SymbolPred::CountsAs(c) => vec![*c],
        SymbolPred::Or(ps) => ps
            .iter()
            .map(|p| match p {
                SymbolPred::CountsAs(c) => Some(*c),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?,
        _ => return None,
    };
    let words: Vec<&str> = colors.into_iter().map(super::effect::color_word).collect();
    match words.as_slice() {
        [] => None,
        [one] => Some((*one).to_string()),
        [a, b] => Some(format!("{a} and {b}")),
        many => {
            let (last, rest) = many.split_last()?;
            Some(format!("{}, and {last}", rest.join(", ")))
        }
    }
}

/// The devotion recognizer ([CR#700.5]): `SumOf` over "permanents you
/// control" (via [`is_permanents_you_control`]), folding `CountOf(
/// ManaSymbols(It, <colors>))` per element, reads as "your devotion to
/// <color words>". `None` for every other `Aggregate` shape — the caller
/// falls back to the structural fold phrase.
fn devotion_phrase(op: AggregateOp, proj: &Projection) -> Option<String> {
    if !matches!(op, AggregateOp::SumOf) {
        return None;
    }
    let Countable::Objects(filter) = &proj.of else {
        return None;
    };
    if !is_permanents_you_control(filter) {
        return None;
    }
    let Count::CountOf(Countable::ManaSymbols(reference, pred)) = proj.by.as_ref() else {
        return None;
    };
    if !matches!(reference.as_ref(), Reference::It) {
        return None;
    }
    devotion_color_words(pred).map(|colors| format!("your devotion to {colors}"))
}

/// The cross-player fold recognizer ([CR#119.1]): an `Aggregate` over a
/// `Players` projection whose body reads `PlayerStatOf(It, attr)` off each
/// matching player reads "the highest/lowest/total/average <attr> among
/// <group>" — "the highest life total among all players" (Arbiter of
/// Knollridge). `None` for any other player-projection shape (a non-`It`
/// reference, an attribute with no noun phrase here, or a group this reader
/// doesn't recognize) — the caller falls back to the generic
/// `[unrendered: …]` (no other shape has a real card yet).
fn player_aggregate_phrase(op: AggregateOp, proj: &Projection) -> Option<String> {
    let Countable::Players(filter) = &proj.of else {
        return None;
    };
    let Count::PlayerStatOf(Reference::It, attr) = proj.by.as_ref() else {
        return None;
    };
    let attr_word = match attr {
        PlayerAttr::Life => "life total",
        PlayerAttr::HandSize => "hand size",
        PlayerAttr::HandSizeLimit | PlayerAttr::LandPlaysPerTurn => return None,
    };
    let fold_word = match op {
        AggregateOp::SumOf => "total",
        AggregateOp::MinOf => "lowest",
        AggregateOp::MaxOf => "highest",
        AggregateOp::AverageOf(_) => "average",
    };
    let group = player_group_phrase(filter)?;
    Some(format!("the {fold_word} {attr_word} among {group}"))
}

/// The player-group noun phrase a `Players` projection folds over — "all
/// players" for the unrestricted top predicate ([CR#119.1] Arbiter of
/// Knollridge/Balance's own fold). `None` for any other filter shape — no
/// other real card needs one yet.
fn player_group_phrase(filter: &Predicate) -> Option<String> {
    match strip_expanded(filter) {
        Predicate::Kind(ObjectKind::Player) => Some("all players".to_string()),
        _ => None,
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
    use std::sync::Arc;

    use super::*;

    fn ctx() -> Ctx<'static> {
        Ctx {
            subject: "Grizzly Bears",
            targets: &[],
            that: None,
            named: None,
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
            named: None,
        };
        assert_eq!(reference(&Reference::It, &scoped), "each creature");
    }

    /// `reference_pronoun` is the PRONOMINAL re-mention ("it"/"they"), distinct
    /// from `target_rementioned`'s demonstrative "that creature". Number comes
    /// from the arg's kind: an object `Reference` → "it"; a plural `Selection`
    /// group → "they"; a player reference (pronoun "they", a follow-up) and an
    /// unclassified selection decline. The pronoun is lowercase — sentence-
    /// initial casing is the filler's job.
    #[test]
    fn reference_pronoun_is_number_aware() {
        use deckmaste_semantics::Quantity;
        use deckmaste_semantics::TargetSpec;

        let creature = [TargetSpec::Target(Quantity::one(), Predicate::creature())];
        let c = Ctx {
            subject: "Bolt",
            targets: &creature,
            that: None,
            named: None,
        };
        // An object target slot, the source object, and an event object all
        // re-mention as the singular "it".
        assert_eq!(reference_pronoun("Target(0)", &c).as_deref(), Some("it"));
        assert_eq!(reference_pronoun("This", &c).as_deref(), Some("it"));
        assert_eq!(reference_pronoun("EventObject", &c).as_deref(), Some("it"));
        // A plural `Selection` group re-mentions as "they".
        assert_eq!(reference_pronoun("They", &c).as_deref(), Some("they"));
        // A player target slot has no object pronoun yet — declines.
        let player = [TargetSpec::Target(
            Quantity::one(),
            Predicate::Kind(ObjectKind::Player),
        )];
        let pc = Ctx {
            subject: "x",
            targets: &player,
            that: None,
            named: None,
        };
        assert_eq!(reference_pronoun("Target(0)", &pc), None);
        // A bare player reference declines too.
        assert_eq!(reference_pronoun("You", &c), None);
    }

    /// The new filter nouns: an ability on the stack, and the team-relative
    /// player relations relative to "you".
    #[test]
    fn filter_noun_renders_ability_and_team_relative_players() {
        let you = || Arc::new(Predicate::Ref(Reference::You));
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
        let flying = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::Characteristic(CharacteristicPredicate::Has("Flying".into())),
            ]
            .into(),
        );
        assert_eq!(filter_noun(&flying), "creature with flying");
        let nonflying = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::Not(Arc::new(Predicate::Characteristic(
                    CharacteristicPredicate::Has("Flying".into()),
                ))),
            ]
            .into(),
        );
        assert_eq!(filter_noun(&nonflying), "creature without flying");
    }

    /// The combat/tap-state adjuncts `filter_noun` needs for two real cards'
    /// "for each" selections: Goblin Piledriver's `And([Permanent,
    /// Subtype("Goblin"), Not(Ref(This)), Attacking])` -> "other attacking
    /// Goblin" (bare-subtype base noun, [`find_bare_subtype_noun`] — no
    /// `Type(_)` atom, so the self-exclusion "other" prefix and the
    /// "attacking" adjective both ride ahead of the subtype noun); Knotvine
    /// Paladin's `And([Creature, Status(Untapped), ControlledBy(Ref(You))])`
    /// -> "untapped creature you control" (typed base noun, "untapped"
    /// adjective ahead of it, controller suffix trailing as usual).
    #[test]
    fn filter_noun_renders_attacking_and_untapped_adjuncts() {
        let goblin_piledriver = Predicate::And(
            vec![
                Predicate::Characteristic(CharacteristicPredicate::Subtype(
                    deckmaste_semantics::SubtypeRef::named("Goblin".into()),
                )),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                Predicate::State(StatePredicate::Attacking),
            ]
            .into(),
        );
        assert_eq!(filter_noun(&goblin_piledriver), "other attacking Goblin");

        let knotvine_paladin = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::State(StatePredicate::Status(Status::Untapped)),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                    Reference::You,
                )))),
            ]
            .into(),
        );
        assert_eq!(
            filter_noun(&knotvine_paladin),
            "untapped creature you control"
        );
    }

    /// [`subject_phrase`]'s two inflections over the same qualified filter
    /// (`And([Creature, Not(Ref(This)), ControlledBy(Ref(You))])`, the
    /// `OtherCreatureYouControl` atom): singular "another creature you
    /// control" vs. plural "Other creatures you control" —
    /// [`filter_subject`]'s pre-existing output, byte-identical through the
    /// shared walk. A bare (unqualified) `Creature` filter, with or without
    /// the `And` wrapper, reads singular as "a creature". Self-exclusion
    /// prints once even when `Not(Ref(This))` repeats among the `And` parts
    /// (the double-print guard).
    ///
    /// Also covers the opponent/teammate controller-restrictor regression:
    /// the `SingularArticle` register must resolve `ControlledBy(..)`
    /// through its OWN article+noun text ("an opponent controls"/"a
    /// teammate controls"), not bleed in [`controller_phrase`]'s plural
    /// "your opponents/teammates control" — asserted side-by-side with the
    /// unaffected `PluralCapitalized` reading of the identical predicate, the
    /// strongest pin for "wrong register selected".
    #[test]
    fn subject_phrase_renders_singular_and_plural_inflections() {
        let you = || Arc::new(Predicate::Ref(Reference::You));
        let filtered = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                Predicate::Relation(RelationPredicate::ControlledBy(you())),
            ]
            .into(),
        );
        assert_eq!(
            subject_phrase(&filtered, SubjectNumber::SingularArticle).as_deref(),
            Some("another creature you control")
        );
        assert_eq!(
            subject_phrase(&filtered, SubjectNumber::PluralCapitalized).as_deref(),
            Some("Other creatures you control")
        );
        assert_eq!(filter_subject(&filtered), "Other creatures you control");

        let doubled = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                Predicate::Relation(RelationPredicate::ControlledBy(you())),
            ]
            .into(),
        );
        assert_eq!(
            subject_phrase(&doubled, SubjectNumber::SingularArticle).as_deref(),
            Some("another creature you control")
        );

        let bare_and = Predicate::And(vec![Predicate::creature()].into());
        assert_eq!(
            subject_phrase(&bare_and, SubjectNumber::SingularArticle).as_deref(),
            Some("a creature")
        );

        assert_eq!(
            subject_phrase(&Predicate::creature(), SubjectNumber::SingularArticle).as_deref(),
            Some("a creature")
        );

        let opponent_controls = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                    Predicate::Relation(RelationPredicate::OpponentOf(you())),
                ))),
            ]
            .into(),
        );
        assert_eq!(
            subject_phrase(&opponent_controls, SubjectNumber::SingularArticle).as_deref(),
            Some("another creature an opponent controls")
        );
        assert_eq!(
            subject_phrase(&opponent_controls, SubjectNumber::PluralCapitalized).as_deref(),
            Some("Other creatures your opponents control")
        );

        let teammate_controls = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                    Predicate::Relation(RelationPredicate::TeammateOf(you())),
                ))),
            ]
            .into(),
        );
        assert_eq!(
            subject_phrase(&teammate_controls, SubjectNumber::SingularArticle).as_deref(),
            Some("another creature a teammate controls")
        );
        assert_eq!(
            subject_phrase(&teammate_controls, SubjectNumber::PluralCapitalized).as_deref(),
            Some("Other creatures your teammates control")
        );

        // No `Not(Ref(This))` — the committed `CreatureOpponentControls`
        // macro shape, so the determiner comes from `a_an(...)` rather than
        // "another".
        let opponent_controls_no_self_exclusion = Predicate::And(
            vec![
                Predicate::creature(),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                    Predicate::Relation(RelationPredicate::OpponentOf(you())),
                ))),
            ]
            .into(),
        );
        assert_eq!(
            subject_phrase(
                &opponent_controls_no_self_exclusion,
                SubjectNumber::SingularArticle
            )
            .as_deref(),
            Some("a creature an opponent controls")
        );
    }

    /// "creatures your teammates control" — the team-relative controller phrase
    /// inside a plural subject ([CR#102.3]).
    #[test]
    fn filter_subject_renders_teammate_controller_phrase() {
        let f = Predicate::And(
            vec![
                Predicate::r#type(deckmaste_semantics::Type::Creature),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(
                    Predicate::Relation(RelationPredicate::TeammateOf(Arc::new(Predicate::Ref(
                        Reference::You,
                    )))),
                ))),
            ]
            .into(),
        );
        assert_eq!(filter_subject(&f), "Creatures your teammates control");
    }

    /// "your devotion to green" — the devotion recognizer ([CR#700.5]):
    /// `SumOf` over permanents you control's `CountOf(ManaSymbols(It,
    /// CountsAs(Green)))`.
    #[test]
    fn count_renders_single_color_devotion() {
        let permanents_you_control = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                    Reference::You,
                )))),
            ]
            .into(),
        );
        let devotion_green = Count::Aggregate(
            AggregateOp::SumOf,
            Projection {
                of: Countable::Objects(Arc::new(permanents_you_control)),
                by: Arc::new(Count::CountOf(Countable::ManaSymbols(
                    Arc::new(Reference::It),
                    SymbolPred::CountsAs(Color::Green),
                ))),
            },
        );
        assert_eq!(count(&devotion_green), "your devotion to green");
    }

    /// A color disjunction (`Or([CountsAs(White), CountsAs(Black)])`) reads
    /// as an English color list — "your devotion to white and black".
    #[test]
    fn count_renders_two_color_devotion() {
        let permanents_you_control = Predicate::And(
            vec![
                Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                    Reference::You,
                )))),
            ]
            .into(),
        );
        let devotion_wb = Count::Aggregate(
            AggregateOp::SumOf,
            Projection {
                of: Countable::Objects(Arc::new(permanents_you_control)),
                by: Arc::new(Count::CountOf(Countable::ManaSymbols(
                    Arc::new(Reference::It),
                    SymbolPred::Or(
                        vec![
                            SymbolPred::CountsAs(Color::White),
                            SymbolPred::CountsAs(Color::Black),
                        ]
                        .into(),
                    ),
                ))),
            },
        );
        assert_eq!(count(&devotion_wb), "your devotion to white and black");
    }

    /// A non-devotion `Aggregate` (a typed subset, not "permanents you
    /// control") keeps the structural fold phrase.
    #[test]
    fn count_renders_non_devotion_aggregate_structurally() {
        let total_power = Count::Aggregate(
            AggregateOp::SumOf,
            Projection {
                of: Countable::Objects(Arc::new(Predicate::And(
                    vec![
                        Predicate::State(StatePredicate::InZone(Zone::Battlefield)),
                        Predicate::r#type(deckmaste_semantics::Type::Creature),
                    ]
                    .into(),
                ))),
                by: Arc::new(Count::StatOf(Reference::It, Stat::Power)),
            },
        );
        // `Count::StatOf` itself has no dedicated `count()` phrase yet (a
        // pre-existing gap, orthogonal to devotion) — the point here is that
        // the devotion recognizer does NOT fire for a typed (non-"permanents
        // you control") `of`, so the fold stays on the structural path.
        assert_eq!(
            count(&total_power),
            "the total [unrendered: StatOf(It, Power)] among all creatures"
        );
    }

    /// The cross-player fold recognizer ([CR#119.1]): `Aggregate(MaxOf, (of:
    /// Players(Player), by: PlayerStatOf(It, Life)))` reads "the highest life
    /// total among all players" — Arbiter of Knollridge's own shape.
    #[test]
    fn count_renders_highest_life_total_among_all_players() {
        use deckmaste_semantics::PlayerAttr;

        let highest_life = Count::Aggregate(
            AggregateOp::MaxOf,
            Projection {
                of: Countable::Players(Arc::new(Predicate::Kind(ObjectKind::Player))),
                by: Arc::new(Count::PlayerStatOf(Reference::It, PlayerAttr::Life)),
            },
        );
        assert_eq!(
            count(&highest_life),
            "the highest life total among all players"
        );
    }

    /// A `Players`-sourced `Aggregate` whose body reads something other than
    /// `PlayerStatOf(It, ..)` (or over a filter the recognizer doesn't name)
    /// is not "devotion" or the player-fold shape — falls back to the
    /// generic `[unrendered: …]`, same as any other unrecognized `Aggregate`.
    #[test]
    fn count_renders_non_player_stat_players_aggregate_structurally() {
        let odd_fold = Count::Aggregate(
            AggregateOp::SumOf,
            Projection {
                of: Countable::Players(Arc::new(Predicate::Kind(ObjectKind::Player))),
                by: Arc::new(Count::Literal(3)),
            },
        );
        assert_eq!(count(&odd_fold), format!("[unrendered: {odd_fold:?}]"));
    }
}
