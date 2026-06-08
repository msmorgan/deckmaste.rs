//! The `cite` subcommand: check / bless / diff / list.

use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::Context;

use crate::citations::{Site, members, parse_refs, scan_text};
use crate::cr::Rules;
use crate::lockfile::Lockfile;

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
            let member_rules = match members(&refs, rules) {
                Ok(m) => m,
                Err(_) => {
                    outcome.checked += 1;
                    outcome.stale.push(Stale {
                        rule: site.raw.clone(),
                        reason: Reason::Gone,
                        site: site.clone(),
                    });
                    continue;
                }
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

fn cmd_list(_flags: &[String]) -> anyhow::Result<()> {
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

pub fn run(args: impl Iterator<Item = OsString>) -> anyhow::Result<()> {
    let args: Vec<String> = args.map(|a| a.to_string_lossy().into_owned()).collect();
    // `args` are the tokens after `cite`; args[0] is the subcommand (default:
    // check).
    let sub = args.first().map(String::as_str).unwrap_or("check");
    let flags: &[String] = args.get(1..).unwrap_or(&[]);
    match sub {
        "check" => cmd_check(flags),
        "list" => cmd_list(flags),
        "bless" => cmd_bless(flags),
        "diff" => cmd_diff(flags),
        other => {
            anyhow::bail!("cite: unknown subcommand {other:?}; expected check|list|bless|diff")
        }
    }
}

fn load_rules() -> anyhow::Result<Rules> {
    let text = std::fs::read_to_string(cr_txt_path())?;
    Rules::parse(&text)
}

fn load_sources() -> anyhow::Result<Vec<(PathBuf, String)>> {
    let root = repo_root();
    let mut out = Vec::new();
    for path in crate::citations::tracked_files(&root)? {
        if let Ok(content) = std::fs::read_to_string(&path) {
            out.push((path, content));
        }
    }
    Ok(out)
}

fn cmd_check(flags: &[String]) -> anyhow::Result<()> {
    if flags.iter().any(|f| f == "--list-noncompliant") {
        return cmd_list_noncompliant();
    }
    // ↓↓↓ the existing cmd_check body stays exactly as-is below ↓↓↓
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
        for site in crate::legacy::scan_noncompliant(path, content) {
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

fn cmd_diff(flags: &[String]) -> anyhow::Result<()> {
    let rule = flags.first().context("usage: cite diff <rule>")?;
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

/// Follow AcademyRuins `link/cr` to its final version-specific Wizards URL.
fn resolve_wizards_url() -> anyhow::Result<String> {
    let resp = ureq::get(ACADEMYRUINS_LINK).call()?;
    Ok(resp.get_url().to_string())
}

fn cmd_bless(_flags: &[String]) -> anyhow::Result<()> {
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

pub fn repo_root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..") }

pub fn cr_txt_path() -> PathBuf {
    // Override for workspaces/CI where the gitignored `data/` lives elsewhere.
    if let Ok(p) = std::env::var("DECKMASTE_CR_TXT") {
        return PathBuf::from(p);
    }
    repo_root().join("data/rules/cr.txt")
}

pub fn lockfile_path() -> PathBuf { repo_root().join("cr-citations.lock") }
