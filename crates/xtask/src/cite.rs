//! The `cite` command: check / bless / diff / list / show.

use std::path::PathBuf;

use clap::{Args, Subcommand};

pub mod citations;
pub mod cr;
pub mod legacy;
pub mod lockfile;

use crate::cite::citations::{Site, members, parse_refs, scan_text, strip_citation_wrapper};
use crate::cite::cr::Rules;
use crate::cite::lockfile::Lockfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reason {
    Gone,     // rule number not in current cr.txt
    Changed,  // checksum differs from lockfile baseline
    Unlocked, // cited rule missing from the lockfile (run `bless`)
}

#[derive(Debug, Clone)]
pub struct Stale {
    pub rule: String,
    pub reason: Reason,
    pub site: Site,
}

#[derive(Debug, Default)]
pub struct CheckOutcome {
    pub stale: Vec<Stale>,
    pub checked: usize,
}

/// Validate already-loaded sources against the rules + lockfile. Pure: no IO.
pub fn check_sources(
    rules: &Rules,
    lock: &Lockfile,
    sources: &[(PathBuf, String)],
) -> anyhow::Result<CheckOutcome> {
    let mut outcome = CheckOutcome::default();
    for (path, content) in sources {
        for site in scan_text(path, content) {
            let refs = match parse_refs(&site.raw) {
                Ok(r) => r,
                Err(_) => continue, // malformed [CR#…] is a legacy/format concern, not staleness
            };
            // A range whose endpoint no longer resolves can't be expanded;
            // report the whole citation as GONE rather than aborting the check.
            let member_rules = if let Ok(m) = members(&refs, rules) {
                m
            } else {
                outcome.checked += 1;
                outcome.stale.push(Stale {
                    rule: site.raw.clone(),
                    reason: Reason::Gone,
                    site: site.clone(),
                });
                continue;
            };
            for rule in member_rules {
                outcome.checked += 1;
                let reason = match (rules.checksum(&rule), lock.checksums.get(&rule)) {
                    (None, _) => Some(Reason::Gone),
                    (Some(_), None) => Some(Reason::Unlocked),
                    (Some(cur), Some(base)) if &cur != base => Some(Reason::Changed),
                    _ => None,
                };
                if let Some(reason) = reason {
                    outcome.stale.push(Stale {
                        rule,
                        reason,
                        site: site.clone(),
                    });
                }
            }
        }
    }
    Ok(outcome)
}

/// Resolve a citation argument to its member rules, each paired with the rule
/// text (`None` when the rule isn't in `cr.txt`). Bails on a malformed citation
/// or an unresolvable range endpoint.
pub fn show_rules(rules: &Rules, arg: &str) -> anyhow::Result<Vec<(String, Option<String>)>> {
    let refs = parse_refs(strip_citation_wrapper(arg))?;
    Ok(members(&refs, rules)?
        .into_iter()
        .map(|n| {
            let text = rules.get(&n).map(|r| r.text.clone());
            (n, text)
        })
        .collect())
}

/// Greedily word-wrap `text` to lines of at most `width` columns, breaking only
/// on whitespace. A word longer than `width` gets its own (over-long) line.
#[must_use]
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if cur.is_empty() {
            cur.push_str(word);
        } else if cur.len() + 1 + word.len() <= width {
            cur.push(' ');
            cur.push_str(word);
        } else {
            lines.push(std::mem::take(&mut cur));
            cur.push_str(word);
        }
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// Every citation site paired with its expanded member rule numbers.
pub fn list_sites(
    rules: &Rules,
    sources: &[(PathBuf, String)],
) -> anyhow::Result<Vec<(Site, Vec<String>)>> {
    let mut out = Vec::new();
    for (path, content) in sources {
        for site in scan_text(path, content) {
            if let Ok(refs) = parse_refs(&site.raw) {
                // Skip citations whose ranges can't expand (missing endpoint);
                // `check` reports those as GONE.
                if let Ok(members) = members(&refs, rules) {
                    out.push((site, members));
                }
            }
        }
    }
    Ok(out)
}

fn cmd_list() -> anyhow::Result<()> {
    let rules = load_rules()?;
    let sources = load_sources()?;
    for (site, members) in list_sites(&rules, &sources)? {
        println!(
            "{}:{}  [CR#{}]  -> {}",
            site.file.display(),
            site.line,
            site.raw,
            members.join(", "),
        );
    }
    Ok(())
}

/// The `cite` subcommands; `check` is the default when none is given.
#[derive(Debug, Args)]
pub struct CiteArgs {
    #[command(subcommand)]
    command: Option<CiteCmd>,
}

#[derive(Debug, Subcommand)]
enum CiteCmd {
    /// Validate cited rules against cr.txt and the lockfile (the default).
    Check(CheckArgs),
    /// List every citation site with its expanded member rule numbers.
    List,
    /// Print the rule(s) a citation resolves to.
    Show(ShowArgs),
    /// Rebuild the lockfile from the rules every citation currently resolves
    /// to.
    Bless,
    /// Show an old-vs-current text diff for one rule.
    Diff(DiffArgs),
}

#[derive(Debug, Default, Args)]
struct CheckArgs {
    /// List citation-looking strings that don't use the [CR#…] format,
    /// instead of checking staleness.
    #[arg(long)]
    list_noncompliant: bool,
}

#[derive(Debug, Args)]
struct ShowArgs {
    /// The citation to resolve: `704.5g`, `CR#704.5g`, or `[CR#704.5g]`;
    /// comma-lists and `..` ranges are accepted too.
    citation: String,
    /// Print one rule per line, without the bracketed header or word-wrapping.
    #[arg(long)]
    plain: bool,
}

#[derive(Debug, Args)]
struct DiffArgs {
    /// The rule number to diff, e.g. `702.158b`.
    rule: String,
}

/// Run a parsed `cite` invocation; no subcommand defaults to `check`.
///
/// # Errors
/// Propagates any failure from the selected subcommand.
pub fn dispatch(args: CiteArgs) -> anyhow::Result<()> {
    match args
        .command
        .unwrap_or_else(|| CiteCmd::Check(CheckArgs::default()))
    {
        CiteCmd::Check(check) => cmd_check(check.list_noncompliant),
        CiteCmd::List => cmd_list(),
        CiteCmd::Show(show) => cmd_show(show),
        CiteCmd::Bless => cmd_bless(),
        CiteCmd::Diff(diff) => cmd_diff(&diff.rule),
    }
}

/// `cite show <citation> [--plain]`: print the rule(s) a citation resolves to.
/// Resolved rules go to stdout; an unresolved single rule prints a marker to
/// stderr and forces a non-zero exit.
fn cmd_show(args: ShowArgs) -> anyhow::Result<()> {
    let rules = load_rules()?;
    let hits = show_rules(&rules, &args.citation)?;

    // The indented block wraps to the terminal width (or 80), less the 4-space
    // indent.
    let width = std::env::var("COLUMNS")
        .ok()
        .and_then(|c| c.parse::<usize>().ok())
        .unwrap_or(80);

    let mut missing = 0;
    for (number, text) in &hits {
        match text {
            Some(t) if args.plain => println!("{number}  {t}"),
            Some(t) => {
                println!("[CR#{number}]");
                for line in wrap_text(t, width.saturating_sub(4)) {
                    println!("    {line}");
                }
            }
            None => {
                missing += 1;
                eprintln!("[CR#{number}]  (not found in cr.txt)");
            }
        }
    }
    if missing > 0 {
        anyhow::bail!("{missing} cited rule(s) not found in cr.txt");
    }
    Ok(())
}

fn load_rules() -> anyhow::Result<Rules> {
    let text = std::fs::read_to_string(cr_txt_path())?;
    Rules::parse(&text)
}

fn load_sources() -> anyhow::Result<Vec<(PathBuf, String)>> {
    let root = repo_root();
    let mut out = Vec::new();
    for path in crate::cite::citations::tracked_files(&root)? {
        if let Ok(content) = std::fs::read_to_string(&path) {
            out.push((path, content));
        }
    }
    Ok(out)
}

fn cmd_check(list_noncompliant: bool) -> anyhow::Result<()> {
    if list_noncompliant {
        return cmd_list_noncompliant();
    }
    let rules = load_rules()?;
    let lock = Lockfile::load(&lockfile_path())?;
    let sources = load_sources()?;
    let outcome = check_sources(&rules, &lock, &sources)?;

    for s in &outcome.stale {
        let tag = match s.reason {
            Reason::Gone => "GONE",
            Reason::Changed => "CHANGED",
            Reason::Unlocked => "UNLOCKED",
        };
        eprintln!(
            "{tag}  {}  {}:{}  {}",
            s.rule,
            s.site.file.display(),
            s.site.line,
            s.site.context
        );
    }
    eprintln!(
        "checked {} citations against cr.txt (eff. {}); {} stale",
        outcome.checked,
        rules.effective,
        outcome.stale.len(),
    );
    if outcome.stale.is_empty() {
        Ok(())
    } else {
        anyhow::bail!("{} stale citation(s)", outcome.stale.len())
    }
}

fn cmd_list_noncompliant() -> anyhow::Result<()> {
    let sources = load_sources()?;
    let mut count = 0;
    for (path, content) in &sources {
        for site in crate::cite::legacy::scan_noncompliant(path, content) {
            count += 1;
            eprintln!(
                "{}:{}  {:?}  {}",
                site.file.display(),
                site.line,
                site.raw,
                site.context
            );
        }
    }
    eprintln!("{count} non-compliant citation-looking string(s)");
    if count == 0 { Ok(()) } else { anyhow::bail!("{count} non-compliant") }
}

/// Render an old-vs-new text comparison for one rule.
#[must_use]
pub fn format_diff(rule: &str, old: Option<&str>, new: Option<&str>) -> String {
    let mut s = format!("[CR#{rule}]\n");
    match old {
        Some(t) => s.push_str(&format!("- {t}\n")),
        None => s.push_str("- (absent in old version)\n"),
    }
    match new {
        Some(t) => s.push_str(&format!("+ {t}\n")),
        None => s.push_str("+ (absent in current cr.txt)\n"),
    }
    s
}

fn cmd_diff(rule: &str) -> anyhow::Result<()> {
    let current = load_rules()?;
    let lock = Lockfile::load(&lockfile_path())?;

    let old_text = ureq::get(&lock.wizards_url).call()?.into_string()?;
    let old = Rules::parse(&old_text)?;

    print!(
        "{}",
        format_diff(
            rule,
            old.get(rule).map(|r| r.text.as_str()),
            current.get(rule).map(|r| r.text.as_str()),
        ),
    );
    Ok(())
}

const ACADEMYRUINS_LINK: &str = "https://academyruins.com/link/cr";

/// Build a fresh lockfile: checksum every cited rule that currently resolves.
/// Skips refs that don't resolve (those surface via `check` as GONE).
pub fn build_lockfile(
    rules: &Rules,
    sources: &[(PathBuf, String)],
    wizards_url: &str,
) -> anyhow::Result<Lockfile> {
    let mut lock = Lockfile {
        cr_date: rules.effective.clone(),
        wizards_url: wizards_url.to_string(),
        ..Default::default()
    };
    for (site, members) in list_sites(rules, sources)? {
        let _ = &site;
        for rule in members {
            if let Some(sum) = rules.checksum(&rule) {
                lock.checksums.insert(rule, sum);
            }
        }
    }
    Ok(lock)
}

/// Follow `AcademyRuins` `link/cr` to its final version-specific Wizards URL.
fn resolve_wizards_url() -> anyhow::Result<String> {
    let resp = ureq::get(ACADEMYRUINS_LINK).call()?;
    Ok(resp.get_url().to_string())
}

fn cmd_bless() -> anyhow::Result<()> {
    let rules = load_rules()?;
    let sources = load_sources()?;
    let path = lockfile_path();

    // Prefer a freshly-resolved URL; fall back to the existing lock's URL.
    let wizards_url =
        resolve_wizards_url().or_else(|_| Lockfile::load(&path).map(|l| l.wizards_url))?;

    let lock = build_lockfile(&rules, &sources, &wizards_url)?;
    lock.save(&path)?;
    eprintln!(
        "blessed {} rules at cr_date {} -> {}",
        lock.checksums.len(),
        lock.cr_date,
        path.display(),
    );
    Ok(())
}

#[must_use]
pub fn repo_root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..") }

#[must_use]
pub fn cr_txt_path() -> PathBuf {
    // Override for workspaces/CI where the gitignored `data/` lives elsewhere.
    if let Ok(p) = std::env::var("DECKMASTE_CR_TXT") {
        return PathBuf::from(p);
    }
    repo_root().join("data/rules/cr.txt")
}

#[must_use]
pub fn lockfile_path() -> PathBuf { repo_root().join("cr-citations.lock") }
