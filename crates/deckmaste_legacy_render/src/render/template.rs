//! Filling a macro's `template:` metadata into rules text.
//! `~` -> the subject; `${i}` / `${name}` -> the rendered positional / named
//! arg. Single-brace `{…}` is a literal game symbol (mana, `{T}`, …).

use deckmaste_semantics::Expansion;
use deckmaste_semantics::ExpansionArgs;

/// The one template-first hook: render a macro invocation through its own
/// rules-text `template`, if it carries a fillable one. `None` when there is no
/// template (or an arg can't be rendered) — the caller then falls back to
/// structural rendering of `e.value`. Every `X::Expanded` arm routes through
/// this so the "prefer the macro's text, else reconstruct it" rule is written
/// once, not re-derived per kind.
pub(super) fn expanded<T>(e: &Expansion<T>, subject: &str) -> Option<String> {
    fill(e.template.as_deref()?, subject, &e.args)
}

/// Fill a template. Returns `None` if any `${…}` can't be resolved/rendered
/// cleanly (caller then falls back to structural rendering — never emit a
/// half-filled template with a literal `${0}` left in it). A single-brace
/// `{…}` is a literal game symbol (mana, `{T}`, …) and passes through
/// untouched, which is why `${…}` — not `{…}` — is the placeholder sigil.
pub(super) fn fill(template: &str, subject: &str, args: &ExpansionArgs) -> Option<String> {
    fill_with(template, subject, args, |raw, modifier| {
        render_slot(raw, modifier)
    })
}

/// Like [`fill`], but the caller supplies `resolve`: how a looked-up raw arg
/// (with its optional `:modifier`) becomes text. [`fill`] passes the
/// context-free [`render_slot`]; the effect renderer passes a `ctx`-aware
/// resolver so a `Reference` arg (a fight's `Target(n)`) renders via
/// `fragment::reference` — which this layer can't call directly without a
/// module cycle, so the resolver is injected from above.
pub(super) fn fill_with(
    template: &str,
    subject: &str,
    args: &ExpansionArgs,
    resolve: impl Fn(&str, Option<&str>) -> Option<String>,
) -> Option<String> {
    let mut out = String::new();
    let mut chars = template.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        match c {
            '~' => out.push_str(subject),
            '$' if matches!(chars.peek(), Some((_, '{'))) => {
                chars.next(); // consume the '{'
                // Read the placeholder body up to the matching unescaped `}`.
                // A backslash escapes the next character (so `\{`/`\}` are a
                // literal brace INSIDE the body, not the closing delimiter) —
                // the escaping convention that lets a repeat construct carry a
                // braced literal like `${0*\{E\}}`. The backslash is dropped;
                // the escaped char is kept verbatim.
                let mut content = String::new();
                while let Some((_, d)) = chars.next() {
                    if d == '\\' {
                        if let Some((_, e)) = chars.next() {
                            content.push(e);
                        }
                        continue;
                    }
                    if d == '}' {
                        break;
                    }
                    content.push(d);
                }
                if content.matches('#').count() == 2 {
                    // Conditional fragment: render prefix+value+suffix iff the value
                    // param is present; absent renders to nothing. The value part
                    // may itself carry a `:modifier` codec.
                    let mut parts = content.splitn(3, '#');
                    let prefix = parts.next().unwrap_or("");
                    let value = parts.next().unwrap_or("").trim();
                    let suffix = parts.next().unwrap_or("");
                    let (key, modifier) = split_modifier(value);
                    if let Some(raw) = lookup_arg(args, key) {
                        out.push_str(prefix);
                        out.push_str(&resolve(raw, modifier)?);
                        out.push_str(suffix);
                    }
                } else if let Some((key, literal)) = content.split_once('*') {
                    // Count-driven literal repetition (`${slot*\{E\}}`): emit the
                    // (already-unescaped) `literal` `slot`-many times — the render
                    // twin of `pattern::Segment::Repeat`. `n == 0` emits nothing;
                    // a missing or non-numeric count declines to the structural
                    // fallback. Past five glyphs the energy convention ([CR#107.14]
                    // — the printed `{E}` count) spells the number and prints a
                    // single glyph ("six {E}", "fifty {E}"), so a large count reads
                    // rather than sprawling; `fidelity::normalize` folds either form
                    // to a run, so the diff is clean either way.
                    let raw = lookup_arg(args, key.trim())?;
                    let n: usize = raw.trim().parse().ok()?;
                    if n > 5 {
                        out.push_str(&deckmaste_plugin::energy::spell_number(n));
                        out.push(' ');
                        out.push_str(literal);
                    } else {
                        for _ in 0..n {
                            out.push_str(literal);
                        }
                    }
                } else {
                    let (key, modifier) = split_modifier(content.trim());
                    let val = resolve(lookup_arg(args, key)?, modifier)?;
                    // Sentence-initial capitalization: a slot that opens a new
                    // sentence (its render directly follows ". ") is capitalized,
                    // the way a baked-capital literal ("It can't be regenerated")
                    // used to carry it. This is what lets a multi-sentence
                    // template pronominalize an earlier mention through a slot
                    // (`${0:pro}` → "It"/"They") instead of a hardcoded pronoun,
                    // while a mid-sentence slot stays lowercase.
                    if at_sentence_start(&out) {
                        out.push_str(&capitalize_first(&val));
                    } else {
                        out.push_str(&val);
                    }
                }
            }
            other => out.push(other),
        }
    }
    Some(out)
}

/// Whether `out` currently sits at the start of a new sentence: the emitted
/// text ends with a sentence terminator followed by whitespace (". "). Used to
/// capitalize a slot that opens an interior sentence of a multi-sentence
/// template. The leading sentence is capitalized by the caller
/// (`capitalize_first` over the whole filled string), so an empty `out` is not
/// a sentence start here.
fn at_sentence_start(out: &str) -> bool {
    let trimmed = out.trim_end_matches(char::is_whitespace);
    trimmed.len() < out.len() && trimmed.ends_with('.')
}

/// Uppercase the first character of `s` (the rest verbatim). Local twin of the
/// effect renderer's private `capitalize_first`, kept here so the template
/// filler needn't reach across modules for a two-line helper.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Split a slot spec `key:modifier` into its key and optional modifier codec
/// ([typed-holes delta 6]): `${0:+}` → (`0`, `+`), `${n:card|cards}` → (`n`,
/// `card|cards`), a bare `${0}` → (`0`, None). The modifier rides after the
/// FIRST colon so a value key can't contain one.
fn split_modifier(spec: &str) -> (&str, Option<&str>) {
    match spec.split_once(':') {
        Some((key, modifier)) => (key.trim(), Some(modifier.trim())),
        None => (spec.trim(), None),
    }
}

/// Render one slot, honoring its `:modifier` codec ([typed-holes delta 6]):
///
/// - none — the ordinary [`render_arg`] rendering.
/// - `+` — sign-aware: a numeric arg renders with an explicit leading sign
///   (`+1` / `-1`), the P/T-pump shape. A non-numeric arg declines (`None`).
/// - `sing|plur` — plural-aware: a numeric arg renders oracle-style — the word
///   `sing` with the indefinite article when `n == 1` else `plur` with the
///   count as a number word ("a card" / "three cards"). A dynamic (non-literal)
///   arg renders `<arg> <plur>` (the natural plural reading of a variable
///   count).
pub(super) fn render_slot(raw: &str, modifier: Option<&str>) -> Option<String> {
    let Some(modifier) = modifier else {
        return render_arg(raw);
    };
    let t = raw.trim();
    if modifier == "+" {
        let n: i64 = t.parse().ok()?;
        return Some(if n < 0 { n.to_string() } else { format!("+{n}") });
    }
    if let Some((sing, plur)) = modifier.split_once('|') {
        return Some(match t.parse::<i64>() {
            // Object counts print oracle-style words: "a card",
            // "three cards"; large literals keep digits.
            Ok(1) => format!("a {sing}"),
            Ok(n) => {
                let n_word = u32::try_from(n)
                    .ok()
                    .and_then(super::fragment::number_word)
                    .map_or_else(|| n.to_string(), str::to_owned);
                format!("{n_word} {plur}")
            }
            // A dynamic count (`X`, `CountOf(...)`) reads plural — rendered
            // through the shared count fragment, declining if it isn't a count.
            Err(_) => {
                let count: deckmaste_semantics::Count =
                    deckmaste_semantics::ron::options().from_str(t).ok()?;
                format!("{} {plur}", super::fragment::count(&count))
            }
        });
    }
    // An unrecognized modifier: decline to the structural fallback rather than
    // emit a half-understood slot.
    None
}

/// Resolve a `${key}` reference against the invocation's args: a numeric key
/// indexes positional args; a name keys into named args. Named signatures carry
/// no indices, so a numeric key never resolves against them (and vice versa).
fn lookup_arg<'a>(args: &'a ExpansionArgs, key: &str) -> Option<&'a String> {
    match args {
        ExpansionArgs::Positional(v) => v.get(key.parse::<usize>().ok()?),
        ExpansionArgs::Named(pairs) => pairs
            .iter()
            .find(|(name, _)| name.as_str() == key)
            .map(|(_, raw)| raw),
    }
}

/// Render one raw-RON-source positional arg back to English (the `show`
/// direction): bare integers pass through; a `Predicate` arg renders as its
/// noun (`ColorIs(Black)` → "black", `Type(Creature)` → "creature"); a `Cost`
/// arg renders as its symbols (`[Mana([Generic(2)])]` → "{2}"). Each is parsed
/// with the bare core reader, so the arg's type is recovered without a
/// `MacroSet`. Anything else (a filter with no clean noun, a verb-cost) returns
/// `None`, so the caller falls back to structural rendering.
fn render_arg(raw: &str) -> Option<String> {
    let t = raw.trim();
    if t.parse::<i64>().is_ok() {
        return Some(t.to_string());
    }
    if let Ok(filter) =
        deckmaste_semantics::ron::options().from_str::<deckmaste_semantics::Predicate>(t)
    {
        let noun = super::fragment::filter_noun(&filter);
        if !noun.contains("[unrendered") {
            return Some(noun);
        }
    }
    // A bare `Type(Name)` / `Subtype(Name)` filter arg (`enchant creature`,
    // `Forestwalk`): the two atoms whose filter-position spelling is a bare
    // macro name the macro-less core reader above can't expand. Identity is
    // by-name, so a `named` ref renders the same noun the macro-expanded def
    // would (see `TypeRef::named`).
    if let Some(filter) = bare_type_or_subtype(t) {
        let noun = super::fragment::filter_noun(&filter);
        if !noun.contains("[unrendered") {
            return Some(noun);
        }
    }
    // A Cost arg (`ward ${0}`, `equip ${0}`): a bracketed cost-component list.
    if let Ok(cost) =
        deckmaste_semantics::ron::options().from_str::<Vec<deckmaste_semantics::CostComponent>>(t)
    {
        return render_cost(&cost);
    }
    // A Count arg (ward-{X}'s `where_x`, quantity slots): the shared count
    // fragment, declining when it has no clean phrase.
    if let Ok(count) = deckmaste_semantics::ron::options().from_str::<deckmaste_semantics::Count>(t)
    {
        let phrase = super::fragment::count(&count);
        if !phrase.contains("[unrendered") {
            return Some(phrase);
        }
    }
    None
}

/// Recover a bare `Type(Name)` / `Subtype(Name)` filter arg as a predicate,
/// name-only. These two filter atoms serialize to a bare macro name
/// (`Type(Creature)`, `Subtype(Goblin)`), so the macro-less core reader in
/// [`render_arg`] can't parse them; but rendering keys only on the name, so a
/// [`deckmaste_semantics::TypeRef::named`] /
/// [`deckmaste_semantics::SubtypeRef::named`] ref suffices. `None` for anything
/// that isn't exactly one of those two atoms wrapping a bare identifier (a
/// nested filter, a colored/negated form) — the caller then declines to its
/// structural fallback, unchanged.
fn bare_type_or_subtype(t: &str) -> Option<deckmaste_semantics::Predicate> {
    use deckmaste_semantics::CharacteristicPredicate;
    use deckmaste_semantics::Ident;
    use deckmaste_semantics::Predicate;
    use deckmaste_semantics::SubtypeRef;
    use deckmaste_semantics::TypeRef;

    let inner = |head: &str| {
        t.strip_prefix(head)
            .and_then(|s| s.strip_suffix(')'))
            .map(str::trim)
            .filter(|name| {
                !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
            })
    };
    if let Some(name) = inner("Type(") {
        return Some(Predicate::Characteristic(CharacteristicPredicate::Type(
            TypeRef::named(Ident::new(name)),
        )));
    }
    if let Some(name) = inner("Subtype(") {
        return Some(Predicate::Characteristic(CharacteristicPredicate::Subtype(
            SubtypeRef::named(Ident::new(name)),
        )));
    }
    None
}

/// Render a cost-component list to its symbol/word text (`[Mana([Generic(2)])]`
/// → "{2}"). Declines on any component without a simple rendering (e.g. a
/// `Do(...)` verb cost), so the keyword falls back to its bare name.
pub(super) fn render_cost(cost: &[deckmaste_semantics::CostComponent]) -> Option<String> {
    use deckmaste_semantics::Cmp;
    use deckmaste_semantics::CostComponent;
    use deckmaste_semantics::Stat;
    let mut out = String::new();
    for component in cost {
        match component {
            CostComponent::Mana(mc) => out.push_str(&super::card::mana_cost(Some(mc))),
            CostComponent::Tap => out.push_str("{T}"),
            CostComponent::Untap => out.push_str("{Q}"),
            // "Pay mana equal to its mana cost" ([CR#202.1]) — the
            // granted-flashback phrase (Snapcaster). Only the carrier-relative
            // `This` ("its") has a frameless rendering here; any other
            // reference needs a `Ctx`, so decline to the structural fallback.
            CostComponent::ManaCostOf(deckmaste_semantics::Reference::This) => {
                out.push_str("its mana cost");
            }
            // An aggregate-stat cost ([CR#702.122a]): "tap any number of
            // [filter] with total [stat] [bound]" — the Crew reminder shape.
            // Only a bare-literal count renders here; a dynamic count declines
            // to the structural fallback.
            CostComponent::TapTotal {
                stat,
                cmp,
                count,
                filter,
            } => {
                let n = count.literal_value()?;
                let subject = super::fragment::filter_subject(filter).to_lowercase();
                let stat_word = match stat {
                    Stat::Power => "power",
                    Stat::Toughness => "toughness",
                    Stat::ManaValue => "mana value",
                    Stat::Loyalty => "loyalty",
                    Stat::Defense => "defense",
                };
                let bound = match cmp {
                    Cmp::AtLeast => format!("{n} or greater"),
                    Cmp::Greater => format!("greater than {n}"),
                    Cmp::AtMost => format!("{n} or less"),
                    Cmp::Less => format!("less than {n}"),
                    Cmp::Eq => format!("exactly {n}"),
                };
                out.push_str("tap any number of ");
                out.push_str(&subject);
                out.push_str(" with total ");
                out.push_str(stat_word);
                out.push(' ');
                out.push_str(&bound);
            }
            _ => return None,
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use deckmaste_semantics::Expansion;
    use deckmaste_semantics::ExpansionArgs;

    use super::expanded;
    use super::fill;
    use super::render_cost;

    /// `ManaCostOf(This)` renders to the granted-flashback phrase "its mana
    /// cost" ([CR#202.1]); a non-`This` reference declines to the structural
    /// fallback (no `Ctx` to render it frameless).
    #[test]
    fn render_cost_renders_mana_cost_of_this() {
        use deckmaste_semantics::CostComponent;
        use deckmaste_semantics::Reference;
        assert_eq!(
            render_cost(&[CostComponent::ManaCostOf(Reference::This)]).as_deref(),
            Some("its mana cost"),
        );
        assert_eq!(
            render_cost(&[CostComponent::ManaCostOf(Reference::Opponent)]),
            None,
        );
    }

    /// The aggregate-stat (Crew) cost renders to its reminder-text shape
    /// ([CR#702.122a]): "tap any number of [filter] with total [stat] [bound]".
    /// A dynamic (non-literal) count declines to the structural fallback.
    #[test]
    fn render_cost_renders_tap_total_crew() {
        use deckmaste_semantics::Cmp;
        use deckmaste_semantics::CostComponent;
        use deckmaste_semantics::Count as SemValue;
        use deckmaste_semantics::Predicate;
        use deckmaste_semantics::Reference;
        use deckmaste_semantics::RelationPredicate;
        use deckmaste_semantics::Stat;
        use deckmaste_semantics::Type;

        // "tap any number of other creatures you control with total power 3 or
        // greater" — the Crew 3 cost ([CR#702.122a]).
        let crew = CostComponent::TapTotal {
            stat: Stat::Power,
            cmp: Cmp::AtLeast,
            count: SemValue::Literal(3),
            filter: Arc::new(Predicate::And(
                vec![
                    Predicate::r#type(Type::Creature),
                    Predicate::Not(Arc::new(Predicate::Ref(Reference::This))),
                    Predicate::Relation(RelationPredicate::ControlledBy(Arc::new(Predicate::Ref(
                        Reference::You,
                    )))),
                ]
                .into(),
            )),
        };
        assert_eq!(
            render_cost(&[crew]).as_deref(),
            Some("tap any number of other creatures you control with total power 3 or greater"),
        );

        // A dynamic count (no literal) declines to the structural fallback.
        let dynamic = CostComponent::TapTotal {
            stat: Stat::Power,
            cmp: Cmp::AtLeast,
            count: SemValue::X,
            filter: Arc::new(Predicate::creature()),
        };
        assert_eq!(render_cost(&[dynamic]), None);
    }

    /// A placeholder expanded value: `expanded` never looks at it (it renders
    /// from `template`/`args`), so any type works.
    fn exp(template: Option<&str>, args: ExpansionArgs) -> Expansion<u8> {
        Expansion {
            name: "M".into(),
            args,
            template: template.map(str::to_owned),
            value: Box::new(0),
        }
    }

    #[test]
    fn expanded_prefers_a_nullary_template() {
        // The AnyTarget shape: nullary, no `~`, no `{i}` — the template *is* the
        // text, used in preference to reconstructing it from `value`.
        let e = exp(Some("any target"), ExpansionArgs::none());
        assert_eq!(expanded(&e, "ignored").as_deref(), Some("any target"));
    }

    #[test]
    fn expanded_fills_subject_and_args() {
        let e = exp(
            Some("~ gets +${0}/+${1} until end of turn"),
            ExpansionArgs::Positional(vec!["1".into(), "1".into()]),
        );
        assert_eq!(
            expanded(&e, "Goblin").as_deref(),
            Some("Goblin gets +1/+1 until end of turn")
        );
    }

    #[test]
    fn expanded_without_template_is_none() {
        // No template -> caller falls back to structural rendering of `value`.
        let e = exp(None, ExpansionArgs::none());
        assert_eq!(expanded(&e, "x"), None);
    }

    #[test]
    fn expanded_with_unrenderable_arg_is_none() {
        // A grammar-node arg the v1 filler can't render -> fall back, never a
        // half-filled template.
        let e = exp(
            Some("${0}"),
            ExpansionArgs::Positional(vec!["SomeFilter".into()]),
        );
        assert_eq!(expanded(&e, "x"), None);
    }

    #[test]
    fn fills_pump_template() {
        let args = ExpansionArgs::Positional(vec!["1".into(), "1".into()]);
        let s = fill("~ gets +${0}/+${1} until end of turn", "Goblin", &args);
        assert_eq!(s.as_deref(), Some("Goblin gets +1/+1 until end of turn"));
    }

    #[test]
    fn returns_none_for_unknown_arg() {
        let args = ExpansionArgs::Positional(vec!["SomeFilter".into()]);
        let s = fill("do ${0} things", "it", &args);
        assert_eq!(s, None);
    }

    #[test]
    fn tilde_expands_to_subject() {
        let args = ExpansionArgs::Positional(vec![]);
        let s = fill("~ is here", "Goblin Guide", &args);
        assert_eq!(s.as_deref(), Some("Goblin Guide is here"));
    }

    #[test]
    fn missing_positional_returns_none() {
        let args = ExpansionArgs::Positional(vec!["1".into()]);
        // ${1} doesn't exist
        let s = fill("~ gets +${0}/+${1}", "it", &args);
        assert_eq!(s, None);
    }

    #[test]
    fn fills_dollar_brace_positional() {
        // New sigil: `${i}` is the positional placeholder.
        let args = ExpansionArgs::Positional(vec!["1".into(), "1".into()]);
        let s = fill("~ gets +${0}/+${1} until end of turn", "Goblin", &args);
        assert_eq!(s.as_deref(), Some("Goblin gets +1/+1 until end of turn"));
    }

    #[test]
    fn fills_named_arg() {
        // `${name}` resolves against named args.
        let args =
            ExpansionArgs::Named(vec![("pow".into(), "2".into()), ("tou".into(), "3".into())]);
        let s = fill("~ gets +${pow}/+${tou} until end of turn", "Ogre", &args);
        assert_eq!(s.as_deref(), Some("Ogre gets +2/+3 until end of turn"));
    }

    #[test]
    fn single_brace_passes_through_literally() {
        // Single-brace `{…}` is a literal game symbol (mana), not a placeholder.
        let args = ExpansionArgs::Positional(vec![]);
        let s = fill("add {C}{C}", "x", &args);
        assert_eq!(s.as_deref(), Some("add {C}{C}"));
    }

    #[test]
    fn dollar_index_against_named_args_is_none() {
        // Named signatures have no indices: `${0}` can't resolve against named args.
        let args = ExpansionArgs::Named(vec![("x".into(), "1".into())]);
        let s = fill("~ gets +${0}", "it", &args);
        assert_eq!(s, None);
    }

    /// [typed-holes delta 6] Sign-aware `${i:+}` renders an explicit leading
    /// sign; plural-aware `${i:sing|plur}` agrees the word with the count.
    #[test]
    fn fills_sign_and_plural_codecs() {
        // `+1/-1` sign form (the P/T-pump shape).
        let pos = fill(
            "gets ${0:+}/${1:+}",
            "ignored",
            &ExpansionArgs::Positional(vec!["2".into(), "3".into()]),
        );
        assert_eq!(pos.as_deref(), Some("gets +2/+3"));
        let neg = fill(
            "gets ${0:+}/${1:+}",
            "ignored",
            &ExpansionArgs::Positional(vec!["-1".into(), "-1".into()]),
        );
        assert_eq!(neg.as_deref(), Some("gets -1/-1"));

        // Plural word agrees with the count: "a card" vs "three cards".
        let one = fill(
            "mills ${n:card|cards}",
            "you",
            &ExpansionArgs::Named(vec![("n".into(), "1".into())]),
        );
        assert_eq!(one.as_deref(), Some("mills a card"));
        let many = fill(
            "mills ${n:card|cards}",
            "you",
            &ExpansionArgs::Named(vec![("n".into(), "3".into())]),
        );
        assert_eq!(many.as_deref(), Some("mills three cards"));
    }

    /// A codec that can't render cleanly declines (`None`) so the caller falls
    /// back to structural rendering, never a half-understood slot: a sign codec
    /// over a non-numeric arg, and an unrecognized modifier.
    #[test]
    fn codec_declines_rather_than_emit_garbage() {
        // `${0:+}` over a non-numeric arg declines.
        let bad_sign = fill(
            "gets ${0:+}",
            "ignored",
            &ExpansionArgs::Positional(vec!["SomeFilter".into()]),
        );
        assert_eq!(bad_sign, None);
        // An unrecognized modifier declines.
        let bad_mod = fill(
            "x ${0:???}",
            "ignored",
            &ExpansionArgs::Positional(vec!["1".into()]),
        );
        assert_eq!(bad_mod, None);
    }

    /// A dynamic (non-literal) count reads plural.
    #[test]
    fn plural_codec_reads_dynamic_count_as_plural() {
        let dynamic = fill(
            "mills ${n:card|cards}",
            "you",
            &ExpansionArgs::Named(vec![("n".into(), "X".into())]),
        );
        assert_eq!(dynamic.as_deref(), Some("mills X cards"));
    }

    /// The count-driven repeat construct `${slot*\{E\}}` emits the escaped
    /// literal `slot`-many times: `3` → `{E}{E}{E}`, `1` → `{E}`, `0` → empty.
    /// (The Rust source doubles each backslash; the template value carries the
    /// single-backslash `\{E\}` that the RON source's `\\{E\\}` decodes to.)
    #[test]
    fn fills_repeat_construct() {
        let three = fill(
            "Pay ${0*\\{E\\}}",
            "ignored",
            &ExpansionArgs::Positional(vec!["3".into()]),
        );
        assert_eq!(three.as_deref(), Some("Pay {E}{E}{E}"));

        let one = fill(
            "Pay ${0*\\{E\\}}",
            "ignored",
            &ExpansionArgs::Positional(vec!["1".into()]),
        );
        assert_eq!(one.as_deref(), Some("Pay {E}"));

        let zero = fill(
            "Pay ${0*\\{E\\}}",
            "ignored",
            &ExpansionArgs::Positional(vec!["0".into()]),
        );
        assert_eq!(zero.as_deref(), Some("Pay "));
    }

    /// Past five, the repeat construct spells the count and prints a single
    /// literal ("six {E}", "fifty {E}") — the printed-energy convention —
    /// rather than a sprawling glyph run; five and below stay as a run.
    #[test]
    fn repeat_construct_spells_counts_past_five() {
        let five = fill(
            "Pay ${0*\\{E\\}}",
            "ignored",
            &ExpansionArgs::Positional(vec!["5".into()]),
        );
        assert_eq!(five.as_deref(), Some("Pay {E}{E}{E}{E}{E}"));

        let six = fill(
            "Pay ${0*\\{E\\}}",
            "ignored",
            &ExpansionArgs::Positional(vec!["6".into()]),
        );
        assert_eq!(six.as_deref(), Some("Pay six {E}"));

        let fifty = fill(
            "Pay ${0*\\{E\\}}",
            "ignored",
            &ExpansionArgs::Positional(vec!["50".into()]),
        );
        assert_eq!(fifty.as_deref(), Some("Pay fifty {E}"));
    }

    /// A non-numeric repeat count declines to the structural fallback rather
    /// than emit a half-filled template.
    #[test]
    fn repeat_construct_declines_non_numeric_count() {
        let bad = fill(
            "${0*\\{E\\}}",
            "ignored",
            &ExpansionArgs::Positional(vec!["X".into()]),
        );
        assert_eq!(bad, None);
    }

    /// End-to-end round trip through the real macro pipeline: `PayEnergy(2)`
    /// renders (via its own `template`) to `Pay {E}{E}`, and that string parses
    /// back to `PayEnergy(2)` through the reverse `TemplateIndex` — render and
    /// parse are the same rule read both ways.
    #[test]
    fn pay_energy_round_trips_render_and_parse() {
        use std::path::Path;

        use deckmaste_plugin::plugin::Plugin;

        use crate::template::index::TemplateIndex;

        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        let cost: deckmaste_semantics::CostComponent =
            plugin.macros.read_str("PayEnergy(2)").expect("expands");
        let deckmaste_semantics::CostComponent::Expanded(e) = &cost else {
            panic!("PayEnergy expands to a CostComponent::Expanded, got {cost:?}");
        };
        let rendered = expanded(e, "ignored").expect("renders via template");
        assert_eq!(rendered, "Pay {E}{E}");

        let idx = TemplateIndex::build(&plugin.macros);
        let never = |_: &str, _: &str| -> Option<(String, usize)> { None };
        let back = idx
            .match_with("CostComponent", &rendered, never)
            .unwrap()
            .expect("re-parses");
        assert_eq!(back.invocation, "PayEnergy(2)");
    }

    /// The gain-side sibling `GainEnergy(3)` renders (via its `OneShotEffect`
    /// template) to `you get {E}{E}{E}` and parses back — the same repeat
    /// construct, wired through the `OneShotEffect` index.
    #[test]
    fn gain_energy_round_trips_render_and_parse() {
        use std::path::Path;

        use deckmaste_plugin::plugin::Plugin;

        use crate::template::index::TemplateIndex;

        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        let effect: deckmaste_semantics::OneShotEffect =
            plugin.macros.read_str("GainEnergy(3)").expect("expands");
        let deckmaste_semantics::OneShotEffect::Expanded(e) = &effect else {
            panic!("GainEnergy expands to a OneShotEffect::Expanded, got {effect:?}");
        };
        let rendered = expanded(e, "ignored").expect("renders via template");
        assert_eq!(rendered, "you get {E}{E}{E}");

        let idx = TemplateIndex::build(&plugin.macros);
        let never = |_: &str, _: &str| -> Option<(String, usize)> { None };
        let back = idx
            .match_with("OneShotEffect", &rendered, never)
            .unwrap()
            .expect("re-parses");
        assert_eq!(back.invocation, "GainEnergy(3)");
    }

    #[test]
    fn fills_conditional_present_and_absent() {
        let present = fill(
            "hexproof${ from #from#}",
            "ignored",
            &ExpansionArgs::Named(vec![("from".into(), "ColorIs(Blue)".into())]),
        );
        assert_eq!(present.as_deref(), Some("hexproof from blue"));

        let absent = fill("hexproof${ from #from#}", "ignored", &ExpansionArgs::none());
        assert_eq!(absent.as_deref(), Some("hexproof"));
    }

    /// A slot that opens an interior sentence (its render directly follows ".
    /// ") is capitalized — the general form of the baked-capital pronoun a
    /// multi-sentence template used to hardcode. A mid-sentence slot stays as
    /// rendered.
    #[test]
    fn slot_opening_an_interior_sentence_is_capitalized() {
        let opener = fill(
            "done. ${0} attacks",
            "x",
            &ExpansionArgs::Positional(vec!["Type(Creature)".into()]),
        );
        assert_eq!(opener.as_deref(), Some("done. Creature attacks"));

        let mid = fill(
            "tap ${0}",
            "x",
            &ExpansionArgs::Positional(vec!["Type(Creature)".into()]),
        );
        assert_eq!(mid.as_deref(), Some("tap creature"));
    }
}
