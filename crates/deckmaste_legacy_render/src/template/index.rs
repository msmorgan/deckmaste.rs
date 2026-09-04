//! A kind-scoped reverse index: `template → macro`. Built from a [`MacroSet`]'s
//! registered macros, it matches an oracle-text fragment back to the macro
//! whose template would render it. Nullary patterns (pure-literal /
//! self-only templates) resolve via [`TemplateIndex::match_kind`];
//! slot-bearing patterns fill their `${…}` holes through the typed slot
//! codec ([`TemplateIndex::match_with`]), including the sign (`${0:+}`)
//! and plural (`${n:card|cards}`) modifiers — the parse-side twins of the
//! render codecs.

use std::collections::HashMap;

use macro_ron::Ident;
use macro_ron::MacroSet;

use super::pattern::ParsePattern;
use super::pattern::Segment;
use super::pattern::Slot;
use super::pattern::SlotKey;
use super::pattern::compile;

/// A successful reverse match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    /// The macro whose template matched.
    pub macro_name: Ident,
    /// How many bytes of `input` the match consumed (from the start).
    pub consumed: usize,
}

/// A successful slot-bearing match: the full macro invocation RON
/// (`Protection(ColorIs(Black))`) and how many bytes of `input` it consumed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotMatch {
    pub invocation: String,
    pub consumed: usize,
}

/// Kind-scoped reverse index over templated macros.
#[derive(Debug, Default)]
pub struct TemplateIndex {
    by_kind: HashMap<Ident, Vec<ParsePattern>>,
}

impl TemplateIndex {
    /// Build the index from every templated macro in `macros`, grouped by the
    /// kind it is registered under, each kind's patterns ordered most-specific
    /// first (greatest total literal length) so specific templates win.
    #[must_use]
    pub fn build(macros: &MacroSet) -> Self {
        let mut by_kind: HashMap<Ident, Vec<ParsePattern>> = HashMap::new();
        for (kind, def) in macros.iter() {
            let Some(template) = def.template() else { continue };
            by_kind.entry(*kind).or_default().push(compile(
                def.name,
                template,
                &def.params,
                def.plural(),
            ));
        }
        for patterns in by_kind.values_mut() {
            patterns.sort_by_key(|p| std::cmp::Reverse(p.literal_len()));
        }
        Self { by_kind }
    }

    /// Match `input` against the bare-emittable (param-less, slot-less)
    /// patterns of `kind`, most-specific first. Returns the matched macro
    /// and how much of `input` it consumed. Defaulted-param macros (e.g.
    /// `Hexproof`) are skipped here — they need the `Name(...)` form, not a
    /// bare nullary invocation.
    ///
    /// Judges ambiguity on FULL matches only (`consumed == input.len()`): a
    /// second full-consuming match from a distinct macro is a generation
    /// error (`anyhow::bail!`) — nothing in the input picks between them —
    /// rather than the old first-match-wins tie-break.
    ///
    /// # Errors
    /// If two or more distinct macros of `kind` fully match `input`.
    pub fn match_kind(&self, kind: &str, input: &str) -> anyhow::Result<Option<Match>> {
        let mut hit: Option<Match> = None;
        for pattern in self.by_kind.get(kind).into_iter().flatten() {
            if pattern.emits_bare()
                && let Some(consumed) = match_nullary(pattern, input)
                && consumed == input.len()
            {
                let m = Match {
                    macro_name: pattern.macro_name,
                    consumed,
                };
                if let Some(prev) = &hit {
                    anyhow::bail!(
                        "ambiguous template match on {input:?}: `{}` and `{}`",
                        prev.macro_name,
                        m.macro_name
                    );
                }
                hit = Some(m);
            }
        }
        Ok(hit)
    }

    /// Match `input` against the SLOT-bearing patterns of `kind`, filling each
    /// `${i}` via `slot_reader(declared_type, remaining_input) -> (arg_ron,
    /// consumed)` — codec-driven matching, so each slot is bounded by what its
    /// reader accepts, not by greedy literal capture. Returns the macro
    /// invocation. Nullary / defaulted-param patterns are not matched here (see
    /// [`Self::match_kind`]); a slot whose `slot_reader` declines fails the
    /// whole pattern.
    ///
    /// Judges ambiguity on FULL matches only (`consumed == input.len()`), same
    /// as [`Self::match_kind`]: a second full-consuming match from a distinct
    /// macro is a generation error, not a first-match-wins tie-break.
    ///
    /// # Errors
    /// If two or more distinct macros of `kind` fully match `input`.
    pub fn match_with<F>(
        &self,
        kind: &str,
        input: &str,
        mut slot_reader: F,
    ) -> anyhow::Result<Option<SlotMatch>>
    where
        F: FnMut(&str, &str) -> Option<(String, usize)>,
    {
        let mut hit: Option<SlotMatch> = None;
        for pattern in self.by_kind.get(kind).into_iter().flatten() {
            if pattern.is_nullary() {
                continue;
            }
            if let Some(m) = fill_pattern(pattern, input, &mut slot_reader)
                && m.consumed == input.len()
            {
                if let Some(prev) = &hit {
                    anyhow::bail!(
                        "ambiguous template match on {input:?}: `{}` and `{}`",
                        prev.invocation,
                        m.invocation
                    );
                }
                hit = Some(m);
            }
        }
        Ok(hit)
    }
}

/// Format one matched slot argument: positional contributes the bare arg,
/// named the `name: arg` pair.
fn fmt_arg(key: &SlotKey, arg: &str) -> String {
    match key {
        SlotKey::Index(_) => arg.to_owned(),
        SlotKey::Name(name) => format!("{name}: {arg}"),
    }
}

/// Read one typed slot at `cursor`, applying its `:modifier` codec — the
/// parse-side twin of `render::template::render_slot` ([typed-holes delta
/// 6]):
///
/// - none — the plain typed read.
/// - `+` — sign-aware: a literal `+` prefix is stripped before the read (`+1`
///   reads as `1`); a `-` declines (counts are unsigned — the debuff spelling
///   is a different template).
/// - `sing|plur` — plural-aware: after the count reads, the agreeing noun must
///   follow (`a card`/`1 card` singular, `two cards`/`X cards` plural),
///   mirroring the render direction exactly.
///
/// Returns the raw arg RON and the cursor past the slot (and its agreed
/// noun), or `None` (the slot declines).
fn read_slot<F>(
    slot: &Slot,
    input: &str,
    cursor: usize,
    slot_reader: &mut F,
) -> Option<(String, usize)>
where
    F: FnMut(&str, &str) -> Option<(String, usize)>,
{
    let rest = input.get(cursor..)?;
    match slot.modifier.as_deref() {
        None => {
            let (arg, used) = slot_reader(slot.ty.as_str(), rest)?;
            Some((arg, cursor + used))
        }
        Some("+") => {
            let unsigned = rest.strip_prefix('+')?;
            let (arg, used) = slot_reader(slot.ty.as_str(), unsigned)?;
            Some((arg, cursor + 1 + used))
        }
        Some(m) => {
            let (sing, plur) = m.split_once('|')?;
            let (arg, used) = slot_reader(slot.ty.as_str(), rest)?;
            let after_arg = cursor + used;
            // Number selection mirrors the render side: a literal 1 is singular,
            // any other literal — and every dynamic count — is plural.
            let noun = match arg.trim().parse::<i64>() {
                Ok(1) => sing,
                _ => plur,
            };
            let expected = format!(" {noun}");
            let end = after_arg + expected.len();
            if !input.get(after_arg..end)?.eq_ignore_ascii_case(&expected) {
                return None;
            }
            // Trailing word boundary: singular "card" must not eat the
            // start of "cards".
            if input[end..]
                .chars()
                .next()
                .is_some_and(char::is_alphanumeric)
            {
                return None;
            }
            Some((arg, end))
        }
    }
}

/// Consume an anaphoric pronoun ("it"/"they", case-folded, word-bounded) at
/// `cursor` — the parse twin of a `${n:pro}` slot's number-aware render.
/// Returns the cursor past the pronoun, or `None` if none is present. Binds
/// nothing: the slot it re-mentions is already captured, so parse need only
/// accept whichever pronoun the render side emitted (the render guarantees the
/// right number).
fn match_anaphor(input: &str, cursor: usize) -> Option<usize> {
    // Longer pronoun first, though the two share no prefix. The trailing
    // word-boundary check stops "it" from eating the start of "items".
    for pronoun in ["they", "it"] {
        let end = cursor + pronoun.len();
        if input
            .get(cursor..end)
            .is_some_and(|s| s.eq_ignore_ascii_case(pronoun))
            && !input[end..]
                .chars()
                .next()
                .is_some_and(char::is_alphanumeric)
        {
            return Some(end);
        }
    }
    None
}

/// Try to match an optional fragment (`prefix` literal, typed `slot`, `suffix`
/// literal) at `cursor`. Returns the slot's raw arg and the new cursor on a
/// full match, or `None` (the fragment is absent — caller leaves the cursor).
fn try_conditional<F>(
    prefix: &str,
    slot: &Slot,
    suffix: &str,
    input: &str,
    cursor: usize,
    slot_reader: &mut F,
) -> Option<(String, usize)>
where
    F: FnMut(&str, &str) -> Option<(String, usize)>,
{
    // prefix/suffix are ASCII template literals; byte-length arithmetic below is
    // panic-safe (`input.get(..)?`) and a non-ASCII literal would simply decline.
    let after_prefix = cursor + prefix.len();
    if !input
        .get(cursor..after_prefix)?
        .eq_ignore_ascii_case(prefix)
    {
        return None;
    }
    let (arg, after_slot) = read_slot(slot, input, after_prefix, slot_reader)?;
    let after_suffix = after_slot + suffix.len();
    if !input
        .get(after_slot..after_suffix)?
        .eq_ignore_ascii_case(suffix)
    {
        return None;
    }
    Some((arg, after_suffix))
}

/// Walk a slot-bearing pattern against `input`: literals match (case-folded),
/// each slot is read by `slot_reader`. Returns the invocation + bytes consumed,
/// or `None` if any literal mismatches or a slot reader declines.
fn fill_pattern<F>(pattern: &ParsePattern, input: &str, slot_reader: &mut F) -> Option<SlotMatch>
where
    F: FnMut(&str, &str) -> Option<(String, usize)>,
{
    let mut cursor = 0usize;
    let mut args: Vec<String> = Vec::new();
    for seg in &pattern.segments {
        match seg {
            Segment::Literal(t) => {
                if !input.get(cursor..cursor + t.len())?.eq_ignore_ascii_case(t) {
                    return None;
                }
                cursor += t.len();
            }
            Segment::SelfRef => {
                if input.get(cursor..cursor + 1)? != "~" {
                    return None;
                }
                cursor += 1;
            }
            Segment::Slot(slot) if slot.modifier.as_deref() == Some("pro") => {
                // An anaphoric `${n:pro}` slot: a PRONOMINAL re-mention that
                // consumes the pronoun ("it"/"they") but binds NOTHING — it
                // re-points at slot n, already captured by that slot's own
                // `${n}`. So the pronoun is matched and skipped, no arg pushed;
                // the invocation's args come only from the primary slots. This
                // is what lets a multi-sentence template ("destroy ${0}.
                // ${0:pro} can't be regenerated") parse back to a single-arg
                // invocation, the reverse of the render side's number-aware
                // pronoun.
                cursor = match_anaphor(input, cursor)?;
            }
            Segment::Slot(slot) => {
                let (arg, new_cursor) = read_slot(slot, input, cursor, slot_reader)?;
                args.push(fmt_arg(&slot.key, &arg));
                cursor = new_cursor;
            }
            Segment::Conditional {
                prefix,
                slot,
                suffix,
            } => {
                // An absent fragment never fails the pattern — the caller checks
                // full-line consumption, so a conditional template can't over-claim.
                if let Some((arg, new_cursor)) =
                    try_conditional(prefix, slot, suffix, input, cursor, slot_reader)
                {
                    args.push(fmt_arg(&slot.key, &arg));
                    cursor = new_cursor;
                }
                // Absent: no arg, cursor unchanged.
            }
            Segment::Repeat { slot, literal } => {
                // Match a run of `literal` (case-folded) at the cursor and fold
                // the run length back into the slot as a plain number — the
                // parse twin of `render::template`'s repeat branch. A zero-length
                // run declines (the literal isn't present at all).
                let (count, new_cursor) = match_repeat(input, cursor, literal)?;
                args.push(fmt_arg(&slot.key, &count.to_string()));
                cursor = new_cursor;
            }
        }
    }
    Some(SlotMatch {
        invocation: format!("{}({})", pattern.macro_name, args.join(", ")),
        consumed: cursor,
    })
}

/// Consume a maximal run of `literal` (case-folded) starting at `cursor`,
/// returning the run length (number of repetitions) and the cursor past it.
/// `None` when `literal` is empty or does not occur even once at `cursor`.
fn match_repeat(input: &str, cursor: usize, literal: &str) -> Option<(usize, usize)> {
    if literal.is_empty() {
        return None;
    }
    let mut count = 0usize;
    let mut cursor = cursor;
    while input
        .get(cursor..cursor + literal.len())
        .is_some_and(|s| s.eq_ignore_ascii_case(literal))
    {
        cursor += literal.len();
        count += 1;
    }
    (count > 0).then_some((count, cursor))
}

/// Match a nullary pattern against the start of `input` (case-folded),
/// requiring a trailing word boundary. Returns the byte length consumed.
fn match_nullary(pattern: &ParsePattern, input: &str) -> Option<usize> {
    let mut target = String::new();
    for seg in &pattern.segments {
        match seg {
            Segment::Literal(t) => target.push_str(t),
            Segment::SelfRef => target.push('~'),
            Segment::Slot(_) | Segment::Conditional { .. } | Segment::Repeat { .. } => {
                return None;
            }
        }
    }
    let n = target.len();
    if !input.get(..n)?.eq_ignore_ascii_case(&target) {
        return None;
    }
    // Trailing word boundary: nullary "flash" must not match the start of
    // "flashbacky".
    if input[n..].chars().next().is_some_and(char::is_alphanumeric) {
        return None;
    }
    Some(n)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use deckmaste_plugin::plugin::Plugin;

    use super::*;

    fn builtin() -> TemplateIndex {
        let plugins = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins");
        let plugin = Plugin::load(plugins.join("builtin")).unwrap();
        TemplateIndex::build(&plugin.macros)
    }

    #[test]
    fn matches_nullary_keyword() {
        let m = builtin()
            .match_kind("KeywordAbility", "flying")
            .unwrap()
            .expect("flying matches");
        assert_eq!(m.macro_name.as_str(), "Flying");
        assert_eq!(m.consumed, "flying".len());
    }

    #[test]
    fn matches_any_target_targetspec() {
        let m = builtin()
            .match_kind("TargetSpec", "any target")
            .unwrap()
            .expect("any target matches");
        assert_eq!(m.macro_name.as_str(), "AnyTarget");
    }

    #[test]
    fn unknown_text_does_not_match() {
        assert!(
            builtin()
                .match_kind("KeywordAbility", "blinking")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn nullary_does_not_prefix_match_a_longer_word() {
        // A slot-bearing "flashback ${0}" is skipped; a nullary "flash" (if any)
        // must not eat the start of "flashback {2}".
        assert!(
            builtin()
                .match_kind("KeywordAbility", "flashback {2}")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn fills_a_typed_slot_via_reader() {
        // Protection is now `params: [Predicate]`; the slot reader is handed the
        // declared type and the remaining input, and returns the arg RON.
        let m = builtin()
            .match_with("KeywordAbility", "protection from black", |ty, rest| {
                assert_eq!(ty, "Predicate");
                Some((format!("ColorIs({})", rest.trim()), rest.len()))
            })
            .unwrap()
            .expect("protection from <x> matches");
        assert_eq!(m.invocation, "Protection(ColorIs(black))");
        assert_eq!(m.consumed, "protection from black".len());
    }

    /// A `${n:pro}` anaphoric slot consumes the pronoun ("It") but binds
    /// nothing: `DestroyNoRegen`'s "destroy ${0}. ${0:pro} can't be
    /// regenerated" parses back to a single-arg invocation, the reverse of
    /// the render side's number-aware pronoun.
    #[test]
    fn anaphoric_pro_slot_binds_nothing_and_round_trips() {
        let m = builtin()
            .match_with(
                "OneShotEffect",
                "destroy target creature. It can't be regenerated",
                |_ty, rest| {
                    // The target reader stops at the sentence break; the
                    // `${0:pro}` slot then matches "It" and captures nothing.
                    let end = rest.find(". ").unwrap_or(rest.len());
                    Some(("Target(0)".to_string(), end))
                },
            )
            .unwrap()
            .expect("destroy-no-regen matches");
        assert_eq!(m.invocation, "DestroyNoRegen(Target(0))");
        assert_eq!(
            m.consumed,
            "destroy target creature. It can't be regenerated".len()
        );
    }

    /// `match_anaphor` accepts either nominative pronoun (case-folded) and
    /// respects a word boundary — "it" must not eat the start of "items".
    #[test]
    fn match_anaphor_accepts_it_and_they_word_bounded() {
        assert_eq!(match_anaphor("it can't", 0), Some(2));
        assert_eq!(match_anaphor("They can't", 0), Some(4));
        assert_eq!(match_anaphor("items", 0), None);
        assert_eq!(match_anaphor("that creature", 0), None);
    }

    #[test]
    fn matches_conditional_named_and_absent() {
        let idx = builtin();
        let present = idx
            .match_with("KeywordAbility", "hexproof from blue", |ty, rest| {
                assert_eq!(ty, "Predicate");
                Some((format!("ColorIs({})", rest.trim()), rest.len()))
            })
            .unwrap()
            .expect("hexproof from <x> matches");
        assert_eq!(present.invocation, "Hexproof(from: ColorIs(blue))");
        assert_eq!(present.consumed, "hexproof from blue".len());

        let absent = idx
            .match_with("KeywordAbility", "hexproof", |_, _| None)
            .unwrap()
            .expect("bare hexproof matches");
        assert_eq!(absent.invocation, "Hexproof()");
        assert_eq!(absent.consumed, "hexproof".len());
    }

    /// A slot reader for a bare leading run of ASCII digits, standing in for
    /// the typed `Count` reader in the ambiguity fixture below.
    fn count_reader(ty: &str, rest: &str) -> Option<(String, usize)> {
        assert_eq!(ty, "Count");
        let n = rest
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(rest.len());
        (n > 0).then(|| (rest[..n].to_owned(), n))
    }

    /// Two `Modification` macros registering the IDENTICAL template ("gets
    /// +${0}/+${1}") are a generation error, not a silent first-match-wins —
    /// nothing in the input disambiguates which macro the card meant.
    #[test]
    fn ambiguous_templates_error() {
        let mut set = deckmaste_plugin::macros::macro_set();
        let boost = |name: &str, body: &str| -> macro_ron::MacroDef {
            deckmaste_semantics::ron::options()
                .from_str(&format!(
                    r#"(
                        name: "{name}",
                        template: "gets +${{0}}/+${{1}}",
                        kinds: [Modification],
                        params: [Count, Count],
                        body: {body},
                    )"#
                ))
                .unwrap()
        };
        set.insert(&boost(
            "BoostA",
            "PowerAndToughness(Up(Param(0)), Up(Param(1)))",
        ))
        .unwrap();
        set.insert(&boost(
            "BoostB",
            "PowerAndToughness(Up(Param(1)), Up(Param(0)))",
        ))
        .unwrap();
        let idx = TemplateIndex::build(&set);
        let err = idx
            .match_with("Modification", "gets +2/+2", count_reader)
            .unwrap_err();
        assert!(err.to_string().contains("ambiguous"), "{err}");
    }

    #[test]
    fn unambiguous_still_matches() {
        let m = builtin().match_kind("KeywordAbility", "flying").unwrap();
        assert!(m.is_some());
    }

    /// The repeat construct's reverse direction: a run of the literal folds
    /// back into the slot as a plain number. `PayEnergy`'s `Pay ${0*\{E\}}`
    /// template matches `Pay {E}{E}` → `PayEnergy(2)`, and a single `Pay
    /// {E}` → `(1)`. The slot reader is never consulted (the count comes
    /// from the run length).
    #[test]
    fn matches_repeat_construct_via_builtin() {
        let idx = builtin();
        let never = |_: &str, _: &str| -> Option<(String, usize)> {
            panic!("repeat slot must not consult the slot reader")
        };
        let two = idx
            .match_with("CostComponent", "Pay {E}{E}", never)
            .unwrap()
            .expect("Pay {E}{E} matches PayEnergy");
        assert_eq!(two.invocation, "PayEnergy(2)");
        assert_eq!(two.consumed, "Pay {E}{E}".len());

        let one = idx
            .match_with("CostComponent", "Pay {E}", never)
            .unwrap()
            .expect("Pay {E} matches PayEnergy");
        assert_eq!(one.invocation, "PayEnergy(1)");
    }

    /// A `Pay ` with no `{E}` run does not match (the repeat requires at least
    /// one occurrence), and a trailing non-`{E}` tail fails full consumption.
    #[test]
    fn repeat_construct_requires_a_run_and_full_consumption() {
        let idx = builtin();
        let never = |_: &str, _: &str| -> Option<(String, usize)> { None };
        assert!(
            idx.match_with("CostComponent", "Pay 3 life", never)
                .unwrap()
                .is_none()
        );
        assert!(
            idx.match_with("CostComponent", "Pay {E} life", never)
                .unwrap()
                .is_none()
        );
    }
}
