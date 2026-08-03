//! `cargo xtask macro templates` — the D10 coexistence contract: every
//! framed macro definition's checked-in `template:` field (the legacy
//! rules-text mini-language `deckmaste_legacy_render::template` renders from)
//! must equal [`project`]ion of its first frame's authored text, so `template:`
//! and `frames:` can't quietly drift into two different sources of truth
//! for the same rendering.
//!
//! `--check` reports every divergence and exits 1. `--write` rewrites the
//! `template:` field in place, byte-surgically (find the field's quoted RON
//! string literal, replace only that span) so every comment and the rest of
//! the file's formatting survives untouched — a full `ron`-round-trip
//! rewrite would lose the file's comments, which this corpus leans on
//! heavily for CR citations and rationale.
//!
//! Neither mode ever touches `frames:`. A guard's stored spelling
//! (`when: [(0, "Exactly(1)")]`) is the readable sugar on purpose — guard
//! *satisfaction* is defined on the fully-expanded canonical form
//! (`deckmaste_spelling::guard`), but the catalog keeps the sugar, and
//! `project` must not silently "fix" that by expanding it: `project` only
//! ever looks at a frame's `text`, never its `when`.
//!
//! A macro whose checked-in `template:` uses a legacy mini-language feature
//! `project` cannot express (a `${i:modifier}` codec, a
//! `${prefix#name#suffix}` conditional, a `${slot*{literal}}` repeat, or a
//! named `${name}` slot) is reported as `excepted`, not failed — see
//! [`has_mini_language_exception`].
//!
//! A macro whose `frames[0]` is **guarded** is `refused`, and fails the run
//! in either mode: a guard pre-binds params, so a guarded frame's text omits
//! them and its projection is a *partial* rendering that can never be the
//! right `template:` target. See [`GuardedProjection`].

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use deckmaste_plugin::plugin::Plugin;
use macro_ron::MacroDef;

#[derive(Debug, Args)]
pub(super) struct TemplatesArgs {
    /// Verify every framed def's checked-in `template:` equals
    /// projection(first frame); exits 1 listing every divergence.
    #[arg(long, conflicts_with = "write")]
    check: bool,

    /// Rewrite every divergent `template:` field in place. Never touches
    /// `frames:` — see the module doc. Refuses (reporting the file, not
    /// guessing) when a framed def has no existing `template: "..."` field
    /// to rewrite — add one by hand first.
    #[arg(long)]
    write: bool,

    /// Defaults to this workspace's `plugins/builtin`.
    plugin_dir: Option<PathBuf>,
}

pub(super) fn run(args: TemplatesArgs) -> anyhow::Result<()> {
    anyhow::ensure!(
        args.check ^ args.write,
        "pass exactly one of `--check` or `--write`"
    );
    let plugin_dir = args.plugin_dir.unwrap_or_else(super::default_plugin_dir);
    let mode = if args.write { Mode::Write } else { Mode::Check };

    let report = execute(&plugin_dir, mode)?;

    println!(
        "{}: {} framed def(s) checked, {} excepted, {} rewritten, {} divergent, {} refused",
        plugin_dir.display(),
        report.checked,
        report.excepted,
        report.rewritten,
        report.divergences.len(),
        report.guarded_projections.len(),
    );
    for divergence in &report.divergences {
        eprintln!(
            "{}: `{}` template {:?} does not match projection(first frame) {:?}",
            divergence.path.display(),
            divergence.name,
            divergence.actual,
            divergence.expected,
        );
    }
    for refused in &report.guarded_projections {
        eprintln!(
            "{}: `{}` frames[0] is guarded ({:?}), so it is a partial rendering — it omits every \
             pre-bound param, and projecting it would compare (or, under --write, overwrite) \
             `template:` against a wording that drops them. Reorder `frames:` so an unguarded \
             frame is first, or give this def a `template:` the projection cannot express.",
            refused.path.display(),
            refused.name,
            refused.text,
        );
    }
    for (path, error) in &report.write_failures {
        eprintln!("{}: {error:#}", path.display());
    }
    anyhow::ensure!(
        report.divergences.is_empty() && report.guarded_projections.is_empty(),
        "{} `template:` divergence(s), {} guarded first frame(s) refused",
        report.divergences.len(),
        report.guarded_projections.len(),
    );
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Check,
    Write,
}

#[derive(Debug)]
struct Divergence {
    path: PathBuf,
    name: String,
    actual: Option<String>,
    expected: String,
}

/// A framed def whose `frames[0]` is **guarded**, so [`project`]ing it can
/// never be the right `template:` target.
///
/// A guard pre-binds params, which is precisely a promise that the frame's
/// text does *not* spell them: `Draws.ron`'s `frames[0]` is the `You`-guarded
/// imperative `"draw <Param(1)> cards"`, projecting to `"draw ${1}"`, while
/// its checked-in template is `"${0} draws ${1:card|cards}"` — the guarded
/// frame is a *partial* rendering by construction and omits the subject.
/// Comparing against it would report a spurious divergence; `--write`ing it
/// would silently delete a param from a checked-in file. Neither is a thing
/// to guess at, so this is reported and the run fails.
#[derive(Debug)]
struct GuardedProjection {
    path: PathBuf,
    name: String,
    text: String,
}

#[derive(Debug, Default)]
struct Report {
    /// Framed defs seen (non-empty `frames:`) — the population `--check`
    /// and `--write` operate over.
    checked: usize,
    /// Framed defs skipped because their checked-in `template:` uses a
    /// mini-language feature `project` cannot express.
    excepted: usize,
    /// Divergent `template:` fields actually rewritten (`Mode::Write` only).
    rewritten: usize,
    /// Files still divergent after this run: every one, under `Mode::Check`;
    /// under `Mode::Write`, only the ones a rewrite attempt failed on.
    divergences: Vec<Divergence>,
    /// Framed defs whose first frame is guarded, so there is no honest
    /// projection to compare or write — see [`GuardedProjection`]. Empty in
    /// the current corpus; a non-empty list fails the run in either mode.
    guarded_projections: Vec<GuardedProjection>,
    /// Why a `Mode::Write` rewrite attempt failed, alongside its file.
    write_failures: Vec<(PathBuf, anyhow::Error)>,
}

/// Walks every `.ron` file under `<plugin_dir>/macros`, classifying each
/// framed def (non-empty `frames:`) as matching, excepted, or divergent —
/// and, under [`Mode::Write`], rewriting divergent ones in place.
///
/// Parses each file's raw text through `plugin`'s own fully-loaded
/// [`MacroSet`](macro_ron::MacroSet) (`plugin.macros.read_str`), not a bare
/// `ron` read: roughly an eighth of this corpus's macro files (`Ascend.ron`,
/// every other `KeywordAbility(...)`/subtype-meta invocation) are
/// meta-macro *invocations*, not literal `MacroDef` structs — reading one
/// bare (no macros registered) fails outright with "expected struct `Macro`
/// but found `KeywordAbility`". `Plugin::load` resolves exactly these
/// through the same `read_str` call, registering meta-macros as it goes
/// until every file parses; re-running that same call against the
/// finished, fully-populated set (deterministic given a fixed registry)
/// reproduces the identical resolved `MacroDef` for every file, invocation
/// or not.
fn execute(plugin_dir: &Path, mode: Mode) -> anyhow::Result<Report> {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)
        .with_context(|| format!("loading plugin {}", plugin_dir.display()))?;
    let macros_dir = plugin_dir.join(deckmaste_core::plugin::MACROS_DIR);
    let mut report = Report::default();

    for path in ron_files_recursive(&macros_dir)? {
        let source =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let def: MacroDef = plugin
            .macros
            .read_str(&source)
            .with_context(|| format!("parsing {} as a macro definition", path.display()))?;
        if def.frames().is_empty() {
            continue;
        }
        report.checked += 1;

        let actual = def.template().map(str::to_owned);
        if actual.as_deref().is_some_and(has_mini_language_exception) {
            report.excepted += 1;
            continue;
        }
        // Only past the exception gate is the projection actually *consumed*
        // — compared under `--check`, written to disk under `--write`. So the
        // guard refusal below sits here, not above it: an excepted def's
        // projection is never read, and refusing one would report a hazard
        // that structurally cannot fire. (This is exactly `Draws.ron`'s
        // situation today, and the whole reason the defect was latent: its
        // `${1:card|cards}` codec excepts it before its guarded `frames[0]`
        // can do any damage.)
        let first = &def.frames()[0];
        if !first.is_unguarded() {
            report.guarded_projections.push(GuardedProjection {
                path: path.clone(),
                name: def.name.as_str().to_string(),
                text: first.text.clone(),
            });
            continue;
        }
        let expected = project(&first.text);
        if actual.as_deref() == Some(expected.as_str()) {
            continue;
        }

        let name = def.name.as_str().to_string();
        if mode == Mode::Write {
            match rewrite_template_field(&source, &expected)
                .and_then(|rewritten| verify_rewrite(&plugin, &def, &expected, rewritten))
            {
                Ok(rewritten) => {
                    fs::write(&path, rewritten)
                        .with_context(|| format!("writing {}", path.display()))?;
                    report.rewritten += 1;
                    continue;
                }
                Err(error) => report.write_failures.push((path.clone(), error)),
            }
        }
        report.divergences.push(Divergence {
            path,
            name,
            actual,
            expected,
        });
    }
    Ok(report)
}

/// Projects an authored frame's text into the legacy `template:`
/// mini-language spelling: `<Param(i)>` becomes `${i}`; `~` and every other
/// byte copies through unchanged.
///
/// Purely lexical — it never parses the frame text as English and never
/// looks at a frame's `when`/`position`, so it cannot see (and cannot
/// disturb) a guard's stored spelling.
#[must_use]
fn project(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while !rest.is_empty() {
        if let Some(tail) = rest.strip_prefix("<Param(")
            && let Some(end) = tail.find(")>")
        {
            out.push_str("${");
            out.push_str(tail[..end].trim());
            out.push('}');
            rest = &tail[end + 2..];
            continue;
        }
        let step = rest
            .char_indices()
            .nth(1)
            .map_or(rest.len(), |(offset, _)| offset);
        out.push_str(&rest[..step]);
        rest = &rest[step..];
    }
    out
}

/// Whether `template` uses a legacy mini-language feature [`project`]
/// cannot express: a codec (`${0:card|cards}`), a conditional
/// (`${ from #from#}`), a repeat (`${0*{E}}`), or a named slot (`${effect}`,
/// as opposed to a positional `${0}`).
///
/// One rule covers all four: a hole whose content is anything other than a
/// bare run of ASCII digits is an exception. A colon/hash/star-bearing
/// content is never all-digits, and neither is a bare name — so this is the
/// union of the four patterns without needing to name them separately.
#[must_use]
fn has_mini_language_exception(template: &str) -> bool {
    let mut rest = template;
    while let Some((content, consumed)) = next_hole(rest) {
        let plain_positional = !content.is_empty() && content.bytes().all(|b| b.is_ascii_digit());
        if !plain_positional {
            return true;
        }
        rest = &rest[consumed..];
    }
    false
}

/// The content of the first `${...}` hole in `text`, and the byte offset
/// just past its closing `}`. Brace-nesting aware: a repeat's literal
/// payload (`${0*{E}}`) contains an inner `{`/`}` pair, so a naive
/// first-`}`-wins scan would cut the hole short at `${0*{E}` — this instead
/// tracks depth from the `${`'s own opening brace and stops when it returns
/// to zero. `None` if `text` has no `${` at all, or an unterminated one.
fn next_hole(text: &str) -> Option<(&str, usize)> {
    let start = text.find("${")?;
    let content_start = start + 2;
    let mut depth = 1usize;
    for (offset, ch) in text[content_start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some((
                        &text[content_start..content_start + offset],
                        content_start + offset + 1,
                    ));
                }
            }
            _ => {}
        }
    }
    None
}

/// Scans every `template:` field under `<plugin_dir>/macros` — not just
/// framed defs (`execute`'s `checked`/`excepted` counters are scoped to
/// those; this is the *other* population, see that struct's own doc) — and
/// returns the sorted names of every macro whose template carries a
/// D10-relevant legacy mini-language feature [`project`] cannot express.
/// `cargo xtask macro census`'s "excepted-template count (corpus-wide)"
/// figure is this list's length; kept as one implementation so census and
/// this module's own cross-check test can't quietly drift apart on what
/// counts.
///
/// # Errors
/// If `plugin_dir` fails to load, or any `.ron` file under its `macros/`
/// fails to parse.
pub(super) fn corpus_wide_excepted_names(plugin_dir: &Path) -> anyhow::Result<Vec<String>> {
    let plugin = Plugin::load_with_sibling_prelude(plugin_dir)
        .with_context(|| format!("loading plugin {}", plugin_dir.display()))?;
    let macros_dir = plugin_dir.join(deckmaste_core::plugin::MACROS_DIR);
    let mut hits = Vec::new();
    for path in ron_files_recursive(&macros_dir)? {
        let source =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let def: MacroDef = plugin
            .macros
            .read_str(&source)
            .with_context(|| format!("parsing {} as a macro definition", path.display()))?;
        if def.template().is_some_and(has_mini_language_exception) {
            hits.push(def.name.as_str().to_string());
        }
    }
    hits.sort();
    Ok(hits)
}

/// Rewrites `source`'s top-level `template: "..."` string field to hold
/// `new_value`, keeping every other byte — comments, other fields,
/// formatting — untouched.
///
/// Located by its exact on-disk spelling, `template: "`: a top-level string
/// field always has a space then an opening quote right after the colon,
/// while the field-*forwarding* spelling a meta-macro's produced-def
/// literal nests (`template: Param(template)`, seen in e.g.
/// `plugins/builtin/macros/macro/subtype/CreatureType.ron`) has no quote
/// there and so is never matched by this marker.
///
/// # Errors
/// If `source` has no `template: "..."` field to rewrite — a framed def
/// that has never had its own `template:` field needs one added by hand;
/// this function refuses to guess where to insert it.
fn rewrite_template_field(source: &str, new_value: &str) -> anyhow::Result<String> {
    const MARKER: &str = "template: \"";
    let marker_at = source
        .find(MARKER)
        .ok_or_else(|| anyhow::anyhow!("no `template: \"...\"` field found to rewrite"))?;
    let quote_open = marker_at + MARKER.len() - 1;
    let value_start = quote_open + 1;

    let bytes = source.as_bytes();
    let mut index = value_start;
    let mut escaped = false;
    let quote_close = loop {
        let byte = *bytes
            .get(index)
            .ok_or_else(|| anyhow::anyhow!("unterminated `template:` string literal"))?;
        if escaped {
            escaped = false;
        } else if byte == b'\\' {
            escaped = true;
        } else if byte == b'"' {
            break index;
        }
        index += 1;
    };

    let quoted = deckmaste_core::ron::options().to_string(&new_value)?;
    let mut rewritten = String::with_capacity(source.len());
    rewritten.push_str(&source[..quote_open]);
    rewritten.push_str(&quoted);
    rewritten.push_str(&source[quote_close + 1..]);
    Ok(rewritten)
}

/// The safety net on `rewrite_template_field`'s marker search: that search
/// is a plain substring scan with no RON-structural anchor, so it has no way
/// to *know* it found the real `template: "..."` field rather than, say, one
/// spelled out inside a `//` comment sitting earlier in the file. Rather than
/// trying to make the search itself comment-aware (still no guarantee — a
/// commented-out def, a doc example, anything with the right shape would
/// still fool it), this re-parses the byte-patched result through `plugin`'s
/// own macro-aware reader and confirms the patch did what it was supposed to
/// and nothing else: `rewritten` must still parse; the `MacroDef` it parses
/// to must report `template() == Some(expected)`; and every other field
/// (`name`, `kinds`, `params`, `plural`, `frames`, `body`) must be identical
/// to `original`, the def the caller already parsed from the *pre*-patch
/// source. A wrong-span patch either breaks the parse outright or leaves the
/// real `template:` field (elsewhere in the file, untouched) reading back as
/// something other than `expected` — either way this turns what would
/// otherwise be a silent corruption of a checked-in file into a loud
/// `Err`, before a single byte reaches disk.
///
/// Returns `rewritten` unchanged on success, so this composes with
/// `rewrite_template_field` via `.and_then`.
///
/// # Errors
/// If `rewritten` fails to parse, its `template:` doesn't read back as
/// `expected`, or any other field differs from `original`.
fn verify_rewrite(
    plugin: &Plugin,
    original: &MacroDef,
    expected: &str,
    rewritten: String,
) -> anyhow::Result<String> {
    let reparsed: MacroDef = plugin
        .macros
        .read_str(&rewritten)
        .context("post-write verification: the rewritten source no longer parses")?;
    anyhow::ensure!(
        reparsed.template() == Some(expected),
        "post-write verification: rewritten `template:` reads back as {:?}, expected {:?} — \
         the marker search likely patched the wrong `template: \"...\"` span",
        reparsed.template(),
        expected,
    );
    anyhow::ensure!(
        defs_agree_except_template(original, &reparsed),
        "post-write verification: rewriting `template:` changed more than `template:` — \
         the marker search likely patched the wrong `template: \"...\"` span"
    );
    Ok(rewritten)
}

/// Whether `a` and `b` agree on every field but `template` — deliberately
/// excluded, since [`verify_rewrite`] is comparing a def from *before* the
/// rewrite against one from *after* it, where `template` is expected (and
/// meant) to differ.
fn defs_agree_except_template(a: &MacroDef, b: &MacroDef) -> bool {
    a.name == b.name
        && a.kinds == b.kinds
        && a.params == b.params
        && a.plural() == b.plural()
        && a.frames() == b.frames()
        && a.body() == b.body()
}

/// The `.ron` files under `dir` at any depth, sorted; an absent directory is
/// empty. A private copy of the same small walker every plugin-tree reader
/// in this workspace carries (`macro_ron::frames::ron_files_recursive`,
/// `deckmaste_plugin::plugin::ron_files_recursive`, both crate-private to
/// their own crates) — xtask depends on neither crate's internals, so this
/// stays its own copy rather than a new public API surface just for one
/// caller.
fn ron_files_recursive(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    let mut subdirs = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            subdirs.push(path);
        } else if path.extension().is_some_and(|ext| ext == "ron") {
            files.push(path);
        }
    }
    subdirs.sort();
    for subdir in subdirs {
        files.extend(ron_files_recursive(&subdir)?);
    }
    files.sort();
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // `project` — the brief's own TDD case, verbatim.
    // -----------------------------------------------------------------

    #[test]
    fn projection_replaces_param_holes_and_keeps_self_reference() {
        assert_eq!(
            project("~ deals <Param(0)> damage to each <Param(1)>"),
            "~ deals ${0} damage to each ${1}"
        );
    }

    #[test]
    fn projection_of_a_frame_with_no_holes_is_unchanged() {
        assert_eq!(project("hexproof"), "hexproof");
    }

    #[test]
    fn projection_never_expands_a_guards_spelling() {
        // `project` only ever sees `FrameSpec::text`; a guard's `when` isn't
        // even in scope for it to expand.
        assert_eq!(project("target <Param(1)>"), "target ${1}");
    }

    // -----------------------------------------------------------------
    // `has_mini_language_exception` — the census's four buckets, plus the
    // ordinary positional case that must NOT be flagged.
    // -----------------------------------------------------------------

    #[test]
    fn plain_positional_holes_are_not_an_exception() {
        assert!(!has_mini_language_exception(
            "${0} deals ${1} damage to ${2}"
        ));
        assert!(!has_mini_language_exception("hexproof"));
    }

    #[test]
    fn a_codec_hole_is_an_exception() {
        // `Discard.ron`.
        assert!(has_mini_language_exception("discard ${0:card|cards}"));
    }

    #[test]
    fn a_conditional_hole_is_an_exception() {
        // `Hexproof.ron`.
        assert!(has_mini_language_exception("hexproof${ from #from#}"));
    }

    #[test]
    fn a_repeat_hole_is_an_exception() {
        // `GainEnergy.ron`, brace-nested payload.
        assert!(has_mini_language_exception("you get ${0*{E}}"));
    }

    #[test]
    fn a_named_slot_is_an_exception() {
        // `Chapter.ron`.
        assert!(has_mini_language_exception("${n} — ${effect}"));
    }

    #[test]
    fn next_hole_is_brace_nesting_aware() {
        let (content, consumed) = next_hole("you get ${0*{E}} today").unwrap();
        assert_eq!(content, "0*{E}");
        assert_eq!(&"you get ${0*{E}} today"[consumed..], " today");
    }

    /// Not part of the regular suite — a one-time (re-runnable on demand)
    /// cross-check of the detector against
    /// `docs/superpowers/research/2026-07-30-macro-frames/
    /// macro-schema-census.md` §3's independently-derived count: "21 macros
    /// total carry a D10-relevant feature". Scans every `template:` in the
    /// real corpus (not just framed defs — the census counted by template
    /// content, independently of whether a def carries `frames:`) and
    /// asserts the same 21 names. Ignored by default because the corpus is
    /// living data — frames and macros are both still being added — and a
    /// hard-coded name list tied to it would go stale exactly the way a test
    /// that depends on repo content always does; run with
    /// `cargo test -p xtask --lib -- --ignored
    /// macro_schema_census_count_matches_21` to re-verify by hand.
    #[test]
    #[ignore = "cross-checks the live corpus against the census; run on demand"]
    fn macro_schema_census_count_matches_21() {
        let plugin_dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../plugins/builtin");
        let hits = corpus_wide_excepted_names(&plugin_dir).unwrap();
        let expected = {
            let mut names = vec![
                "Chapter",
                "DestroyNoRegen",
                "Discard",
                "DiscardAtRandom",
                "DiscardCards",
                "Discards",
                "DiscardsAtRandom",
                "Draw",
                "Draws",
                "GainEnergy",
                "Hexproof",
                "LoyaltyMinus",
                "LoyaltyPlus",
                "LoyaltyZero",
                "Mill",
                "Mills",
                "PayEnergy",
                "PreventAll",
                "PreventNext",
                "Unless",
                "Ward",
            ];
            names.sort_unstable();
            names
        };
        assert_eq!(
            hits,
            expected,
            "{} macro(s) flagged, census says 21",
            hits.len()
        );
    }

    // -----------------------------------------------------------------
    // `rewrite_template_field` — the byte-surgical splice.
    // -----------------------------------------------------------------

    #[test]
    fn rewrite_replaces_only_the_template_string_preserving_the_rest() {
        let source = "// a comment mentioning template: too\n(\n    name: \"Draws\",\n    template: \"${0} draws ${1} cards\",\n    kinds: [OneShotEffect],\n)\n";
        let rewritten = rewrite_template_field(source, "${0} draws ${1} card(s)").unwrap();
        assert!(rewritten.contains("template: \"${0} draws ${1} card(s)\","));
        assert!(rewritten.contains("// a comment mentioning template: too"));
        assert!(rewritten.contains("name: \"Draws\","));
    }

    #[test]
    fn rewrite_is_idempotent() {
        let source =
            "(\n    name: \"Draw\",\n    template: \"draw ${0} cards\",\n    kinds: [Count],\n)\n";
        let once = rewrite_template_field(source, "draw ${0} card").unwrap();
        let twice = rewrite_template_field(&once, "draw ${0} card").unwrap();
        assert_eq!(once, twice);
    }

    #[test]
    fn rewrite_refuses_to_guess_at_a_missing_field() {
        let source = "(\n    name: \"Chapter\",\n    kinds: [Ability],\n)\n";
        let error = rewrite_template_field(source, "${n}").unwrap_err();
        assert!(format!("{error:#}").contains("no `template:"));
    }

    #[test]
    fn rewrite_does_not_touch_a_meta_macros_nested_forwarding_field() {
        // `CreatureType.ron`'s shape: no top-level `template: "..."` string
        // at all, only the produced def's `template: Param(template)`
        // forwarding spelling nested in `body:`.
        let source = "(\n    name: \"CreatureType\",\n    kinds: [Macro],\n    params: { \"name\": String, \"template\": Default(String, Param(name)) },\n    body: (\n        name: Param(name),\n        template: Param(template),\n        kinds: [Subtype],\n        body: Subtype(name: Param(template), types: [Creature]),\n    ),\n)\n";
        let error = rewrite_template_field(source, "whatever").unwrap_err();
        assert!(format!("{error:#}").contains("no `template:"));
    }

    // -----------------------------------------------------------------
    // `execute` end to end — real files under a throwaway temp dir, per
    // the round's ruling that these tests build their own fixtures rather
    // than depending on the corpus (which currently frames nothing).
    // -----------------------------------------------------------------

    fn tempdir_with(files: &[(&str, &str)]) -> TempDir {
        let dir = TempDir::new();
        for (rel, body) in files {
            let path = dir.path().join(rel);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, body).unwrap();
        }
        dir
    }

    // Every fixture body is a trivial, name-collision-free RON value (`0`),
    // never the macro's own name (`body: Draws(...)` inside a def named
    // `Draws` reads as a self-invocation and trips `macro_ron`'s cycle
    // check at registration) — these fixtures only need to *register*
    // cleanly, `execute` never expands a body.
    const MATCHING: &str = r#"(
    name: "Draws",
    template: "${0} draws ${1} cards",
    kinds: [OneShotEffect],
    params: [Reference, Count],
    frames: ["<Param(0)> draws <Param(1)> cards"],
    body: 0,
)
"#;

    const DIVERGENT: &str = r#"(
    name: "GainLife",
    template: "${0} gains ${1} life",
    kinds: [OneShotEffect],
    params: [Reference, Count],
    frames: ["<Param(0)> gain <Param(1)> life"],
    body: 0,
)
"#;

    const EXCEPTED: &str = r#"(
    name: "Discard",
    template: "discard ${0:card|cards}",
    kinds: [OneShotEffect, KeywordAction],
    params: [Count],
    frames: ["discard <Param(0)>"],
    body: 0,
)
"#;

    const UNFRAMED: &str = r#"(
    name: "Landwalk",
    template: "landwalk",
    kinds: [KeywordAbility],
    body: 0,
)
"#;

    /// `Draws.ron`'s real shape, minus the codec that excepts it today: the
    /// `You`-guarded imperative is `frames[0]`, so `project`ing it yields
    /// `"draw ${1}"` — the subject `${0}` is gone, because a guard pre-binds
    /// it and the frame therefore never spells it.
    const GUARDED_FIRST_FRAME: &str = r#"(
    name: "Draws",
    template: "${0} draws ${1} cards",
    kinds: [OneShotEffect],
    params: [Reference, Count],
    frames: [
        (text: "draw <Param(1)> cards", when: [(0, "You")], position: Main),
        "<Param(0)> draws <Param(1)> cards",
    ],
    body: 0,
)
"#;

    /// The same def with an excepting `${1:card|cards}` codec — the live
    /// corpus's actual state, and the reason the defect above is latent.
    const GUARDED_FIRST_FRAME_EXCEPTED: &str = r#"(
    name: "Draws",
    template: "${0} draws ${1:card|cards}",
    kinds: [OneShotEffect],
    params: [Reference, Count],
    frames: [
        (text: "draw <Param(1)> cards", when: [(0, "You")], position: Main),
        "<Param(0)> draws <Param(1)> cards",
    ],
    body: 0,
)
"#;

    #[test]
    fn check_passes_when_every_framed_def_matches_its_projection() {
        let dir = tempdir_with(&[
            ("macros/action/Draws.ron", MATCHING),
            ("macros/keyword/Landwalk.ron", UNFRAMED),
        ]);
        let report = execute(dir.path(), Mode::Check).unwrap();
        assert_eq!(report.checked, 1, "the unframed def is out of scope");
        assert_eq!(report.excepted, 0);
        assert!(report.divergences.is_empty());
    }

    #[test]
    fn check_reports_a_divergence_and_names_the_file() {
        let dir = tempdir_with(&[("macros/effect/GainLife.ron", DIVERGENT)]);
        let report = execute(dir.path(), Mode::Check).unwrap();
        assert_eq!(report.checked, 1);
        assert_eq!(report.divergences.len(), 1);
        let divergence = &report.divergences[0];
        assert_eq!(divergence.name, "GainLife");
        assert_eq!(divergence.expected, "${0} gain ${1} life");
        assert_eq!(divergence.actual.as_deref(), Some("${0} gains ${1} life"));
    }

    /// A guarded `frames[0]` is a *partial* rendering: projecting it drops
    /// every pre-bound param. `--check` must refuse it rather than report a
    /// spurious divergence, and — the reason this is loud rather than a
    /// silent skip — `--write` must refuse it rather than overwrite a
    /// checked-in `template:` with a wording that has lost its subject.
    #[test]
    fn a_guarded_first_frame_is_refused_rather_than_projected() {
        for mode in [Mode::Check, Mode::Write] {
            let dir = tempdir_with(&[("macros/action/Draws.ron", GUARDED_FIRST_FRAME)]);
            let report = execute(dir.path(), mode).unwrap();
            assert_eq!(report.checked, 1);
            assert_eq!(report.guarded_projections.len(), 1, "{mode:?}");
            assert_eq!(report.guarded_projections[0].name, "Draws");
            assert_eq!(
                report.guarded_projections[0].text, "draw <Param(1)> cards",
                "the report must name the guarded frame it refused"
            );
            assert!(report.divergences.is_empty(), "{mode:?}");
            assert_eq!(report.rewritten, 0, "{mode:?}");
            // The file on disk is untouched — the whole point of refusing.
            let after =
                std::fs::read_to_string(dir.path().join("macros/action/Draws.ron")).unwrap();
            assert_eq!(after, GUARDED_FIRST_FRAME, "{mode:?}");
        }
    }

    /// The ordering half of the fix, pinned so it cannot be reordered back:
    /// the mini-language exception is decided *before* the guard refusal,
    /// because an excepted def's projection is never consumed at all. This
    /// is the live corpus's `Draws.ron`, which must stay `excepted` — the
    /// figure `cargo xtask macro templates --check` reports today.
    #[test]
    fn an_excepted_template_is_excepted_even_when_its_first_frame_is_guarded() {
        let dir = tempdir_with(&[("macros/action/Draws.ron", GUARDED_FIRST_FRAME_EXCEPTED)]);
        let report = execute(dir.path(), Mode::Check).unwrap();
        assert_eq!(report.checked, 1);
        assert_eq!(report.excepted, 1);
        assert!(report.guarded_projections.is_empty());
        assert!(report.divergences.is_empty());
    }

    #[test]
    fn check_excepts_a_mini_language_template_instead_of_failing_it() {
        let dir = tempdir_with(&[("macros/action/Discard.ron", EXCEPTED)]);
        let report = execute(dir.path(), Mode::Check).unwrap();
        assert_eq!(report.checked, 1);
        assert_eq!(report.excepted, 1);
        assert!(
            report.divergences.is_empty(),
            "an excepted def must not also be reported as a divergence: {:?}",
            report.divergences
        );
    }

    #[test]
    fn write_rewrites_a_divergent_template_in_place_and_becomes_check_clean() {
        let dir = tempdir_with(&[("macros/effect/GainLife.ron", DIVERGENT)]);
        let path = dir.path().join("macros/effect/GainLife.ron");

        let written = execute(dir.path(), Mode::Write).unwrap();
        assert_eq!(written.rewritten, 1);
        assert!(written.divergences.is_empty());
        assert!(written.write_failures.is_empty());

        let on_disk = std::fs::read_to_string(&path).unwrap();
        assert!(on_disk.contains(r#"template: "${0} gain ${1} life","#));
        // Nothing else moved: same `frames:`/`body:` text, byte for byte.
        assert!(on_disk.contains(r#"frames: ["<Param(0)> gain <Param(1)> life"],"#));
        assert!(on_disk.contains("body: 0,"));

        let rechecked = execute(dir.path(), Mode::Check).unwrap();
        assert!(rechecked.divergences.is_empty(), "now idempotent");
    }

    /// The guard sits on `frames[1]`, not `frames[0]`: a guarded *first*
    /// frame is refused outright (see
    /// `a_guarded_first_frame_is_refused_rather_than_projected`), so it can
    /// never reach a write at all, and the property this test exists for —
    /// that the byte-surgical rewrite leaves a guard's stored sugar
    /// untouched, never expanding `Exactly(1)` to
    /// `Range(Some(1), Some(1))` — needs a def a write actually happens on.
    #[test]
    fn write_never_expands_a_guards_stored_spelling() {
        let source = r#"(
    name: "Target",
    template: "target any target",
    kinds: [TargetSpec],
    params: [Predicate],
    frames: [
        "target <Param(0)>",
        (text: "target <Param(0)>", when: [(0, "Exactly(1)")]),
    ],
    body: 0,
)
"#;
        let dir = tempdir_with(&[("macros/target/Target.ron", source)]);
        let path = dir.path().join("macros/target/Target.ron");

        let written = execute(dir.path(), Mode::Write).unwrap();
        assert_eq!(written.rewritten, 1);

        let on_disk = std::fs::read_to_string(&path).unwrap();
        assert!(on_disk.contains(r#"template: "target ${0}","#));
        // The guard's sugar is untouched — still `Exactly(1)`, not expanded
        // to `Range(Some(1), Some(1))`.
        assert!(on_disk.contains(r#"when: [(0, "Exactly(1)")]"#));
    }

    // -----------------------------------------------------------------
    // `verify_rewrite` — the post-write safety net. `rewrite_template_field`
    // locates its marker by a plain substring search with no RON-structural
    // anchor; these pin what catches it when that search finds the wrong
    // `template: "..."` span.
    // -----------------------------------------------------------------

    /// The actual collision, constructed: a `//` comment spelling out the
    /// exact marker (`template: "`) *with* its trailing quote, sitting
    /// before the real field. A naive `rewrite_template_field` alone would
    /// patch the comment (the first match) and leave the real field
    /// untouched — `execute` must refuse the write outright rather than
    /// silently corrupting the file.
    const DECOY_COMMENT_BEFORE_REAL_FIELD: &str = r#"// see also template: "some other spelling" for context
(
    name: "GainLife",
    template: "${0} gains ${1} life",
    kinds: [OneShotEffect],
    params: [Reference, Count],
    frames: ["<Param(0)> gain <Param(1)> life"],
    body: 0,
)
"#;

    #[test]
    fn write_refuses_rather_than_patch_a_decoy_comments_template_marker() {
        let dir = tempdir_with(&[(
            "macros/effect/GainLife.ron",
            DECOY_COMMENT_BEFORE_REAL_FIELD,
        )]);
        let path = dir.path().join("macros/effect/GainLife.ron");
        let before = std::fs::read_to_string(&path).unwrap();

        let report = execute(dir.path(), Mode::Write).unwrap();

        assert_eq!(
            report.rewritten, 0,
            "the comment's marker must not be mistaken for the real field"
        );
        assert_eq!(report.write_failures.len(), 1);
        assert!(
            format!("{:#}", report.write_failures[0].1).contains("post-write verification"),
            "{:?}",
            report.write_failures[0].1
        );
        // Still shows up as unresolved, matching `--check`'s own view.
        assert_eq!(report.divergences.len(), 1);
        assert_eq!(report.divergences[0].name, "GainLife");

        // And, crucially: the file on disk was never touched. A caught
        // `Err` from `verify_rewrite` must short-circuit *before*
        // `execute` calls `fs::write`.
        let after = std::fs::read_to_string(&path).unwrap();
        assert_eq!(before, after, "a refused rewrite must not touch the file");
    }

    #[test]
    fn verify_rewrite_accepts_a_clean_rewrite() {
        let dir = tempdir_with(&[("macros/effect/GainLife.ron", DIVERGENT)]);
        let plugin = Plugin::load_with_sibling_prelude(dir.path()).unwrap();
        let source =
            std::fs::read_to_string(dir.path().join("macros/effect/GainLife.ron")).unwrap();
        let def: MacroDef = plugin.macros.read_str(&source).unwrap();
        let expected = project(&def.frames()[0].text);

        let rewritten = rewrite_template_field(&source, &expected).unwrap();
        let verified = verify_rewrite(&plugin, &def, &expected, rewritten.clone()).unwrap();
        assert_eq!(verified, rewritten);
    }

    #[test]
    fn defs_agree_except_template_ignores_only_that_field() {
        let dir = tempdir_with(&[("macros/effect/GainLife.ron", DIVERGENT)]);
        let plugin = Plugin::load_with_sibling_prelude(dir.path()).unwrap();
        let source =
            std::fs::read_to_string(dir.path().join("macros/effect/GainLife.ron")).unwrap();
        let original: MacroDef = plugin.macros.read_str(&source).unwrap();

        let same_but_template: MacroDef = plugin
            .macros
            .read_str(&source.replace(
                r#"template: "${0} gains ${1} life","#,
                r#"template: "anything else","#,
            ))
            .unwrap();
        assert!(defs_agree_except_template(&original, &same_but_template));

        let different_frames: MacroDef = plugin
            .macros
            .read_str(&source.replace(
                r#"frames: ["<Param(0)> gain <Param(1)> life"],"#,
                r#"frames: ["<Param(0)> gains <Param(1)> life"],"#,
            ))
            .unwrap();
        assert!(!defs_agree_except_template(&original, &different_frames));
    }

    /// Minimal self-cleaning temp dir (avoids adding the `tempfile` crate) —
    /// same pattern as `xtask::coverage`'s own test-only copy.
    struct TempDir(std::path::PathBuf);
    impl TempDir {
        fn new() -> Self {
            use std::sync::atomic::AtomicU32;
            use std::sync::atomic::Ordering;
            static N: AtomicU32 = AtomicU32::new(0);
            let base = std::env::temp_dir().join(format!(
                "xtask-macro-templates-{}-{}",
                std::process::id(),
                N.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&base).unwrap();
            Self(base)
        }
        fn path(&self) -> &std::path::Path {
            &self.0
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
