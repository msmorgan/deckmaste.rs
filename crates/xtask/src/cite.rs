//! Repository-owned Comprehensive Rules citation checking.
//!
//! The checker reads the repository's `cite-config.json`, scans the configured
//! source files, and resolves every citation against `data/rules/cr.txt`. The
//! committed lock stores only normalized-text hashes, never Wizards rules text.
//! The command surface is `check [--list-noncompliant]`, `bless`, `list`,
//! `show`, `diff`, `audit`, plus the separate citation-coverage ratchet.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::io::IsTerminal as _;
use std::io::Read as _;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::OnceLock;

use anyhow::Context as _;
use clap::Args;
use clap::Subcommand;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;
use syn::visit::Visit as _;

/// Repository citation commands.
#[derive(Debug, Args)]
pub struct CiteArgs {
    #[command(subcommand)]
    command: Option<CiteCommand>,
}

#[derive(Debug, Subcommand)]
enum CiteCommand {
    /// Validate citation existence and locked rule-text hashes.
    Check(CheckArgs),
    /// Rebuild the citation lock from the current CR text.
    Bless,
    /// List every citation site and its expanded rule members.
    List,
    /// Print the official text resolved by one citation.
    Show(ShowArgs),
    /// Compare a rule in the locked CR snapshot with the current snapshot.
    Diff(DiffArgs),
    /// Print each citing claim beside the full rules text it names.
    Audit(AuditArgs),
    /// Report, check, or bless the independent citation-coverage ratchet.
    Coverage(CoverageArgs),
}

#[derive(Debug, Default, Args)]
struct CheckArgs {
    /// List citation-looking prose that does not use `[CR#…]` syntax.
    #[arg(long)]
    list_noncompliant: bool,
}

#[derive(Debug, Args)]
struct ShowArgs {
    citation: String,
    /// Print one unwrapped `rule  text` line per resolved member.
    #[arg(long)]
    plain: bool,
}

#[derive(Debug, Args)]
struct DiffArgs {
    rule: String,
}

#[derive(Debug, Args)]
struct AuditArgs {
    /// Audit only citation sites on added lines of a unified diff.
    #[arg(long)]
    diff: bool,
    files: Vec<PathBuf>,
}

#[derive(Debug, Args)]
struct CoverageArgs {
    #[arg(long, conflicts_with = "bless")]
    check: bool,
    #[arg(long, conflicts_with = "check")]
    bless: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
enum CitationFormat {
    #[default]
    Bracketed,
    Bare,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct SourceConfig {
    tracked: bool,
    extensions: Vec<String>,
    exclude: Vec<String>,
    globs: Vec<String>,
}

fn default_lockfile() -> PathBuf {
    PathBuf::from("cr-citations.lock")
}

#[derive(Debug, Deserialize)]
#[serde(default)]
struct CiteConfig {
    format: CitationFormat,
    lockfile: PathBuf,
    sources: SourceConfig,
}

impl Default for CiteConfig {
    fn default() -> Self {
        Self {
            format: CitationFormat::Bracketed,
            lockfile: default_lockfile(),
            sources: SourceConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CitationLock {
    checksums: BTreeMap<String, String>,
    cr_date: String,
    wizards_url: String,
}

#[derive(Debug)]
struct Repository {
    root: PathBuf,
    config: CiteConfig,
}

#[derive(Debug)]
struct CrSnapshot {
    effective_date: String,
    order: Vec<String>,
    indexes: BTreeMap<String, usize>,
    rules: BTreeMap<String, String>,
    sections: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CitationSite {
    file: String,
    line: usize,
    context: String,
    raw: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Resolution {
    Ok(Vec<String>),
    Malformed,
    Gone,
    Placeholder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedCitation {
    site: CitationSite,
    resolution: Resolution,
}

#[derive(Debug, PartialEq, Eq)]
struct NoncompliantHit {
    file: String,
    line: usize,
    matched: String,
    context: String,
}

#[derive(Debug)]
struct CheckReport {
    stale: Vec<String>,
    checked: usize,
}

#[must_use]
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Run a repository citation command.
///
/// # Errors
/// Fails for malformed configuration or CR input, unavailable source files,
/// failed network/VCS helpers, or a citation gate that finds violations.
pub fn dispatch(args: &CiteArgs) -> anyhow::Result<()> {
    if let Some(CiteCommand::Coverage(args)) = &args.command {
        let mut raw = Vec::new();
        if args.check {
            raw.push("--check".to_string());
        }
        if args.bless {
            raw.push("--bless".to_string());
        }
        return crate::coverage::run(&raw);
    }

    let repo = Repository::load(&repo_root())?;
    match args.command.as_ref() {
        None
        | Some(CiteCommand::Check(CheckArgs {
            list_noncompliant: false,
        })) => repo.command_check(),
        Some(CiteCommand::Check(_)) => repo.command_noncompliant(),
        Some(CiteCommand::Bless) => repo.command_bless(),
        Some(CiteCommand::List) => repo.command_list(),
        Some(CiteCommand::Show(args)) => repo.command_show(args),
        Some(CiteCommand::Diff(args)) => repo.command_diff(args),
        Some(CiteCommand::Audit(args)) => repo.command_audit(args),
        Some(CiteCommand::Coverage(_)) => unreachable!("coverage returned before repository load"),
    }
}

impl Repository {
    fn load(root: &Path) -> anyhow::Result<Self> {
        let root = root
            .canonicalize()
            .with_context(|| format!("canonicalizing repository root {}", root.display()))?;
        let config_path = root.join("cite-config.json");
        let text = std::fs::read_to_string(&config_path)
            .with_context(|| format!("reading {}", config_path.display()))?;
        let config = serde_json::from_str(&text)
            .with_context(|| format!("parsing {}", config_path.display()))?;
        Ok(Self { root, config })
    }

    fn load_cr(&self) -> anyhow::Result<CrSnapshot> {
        let path = self.root.join("data/rules/cr.txt");
        let text = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "reading {} — run `scripts/fetch_data` to stage the CR snapshot",
                path.display()
            )
        })?;
        CrSnapshot::parse(&text).with_context(|| format!("parsing {}", path.display()))
    }

    fn lock_path(&self) -> PathBuf {
        self.root.join(&self.config.lockfile)
    }

    fn read_lock(&self, command: &str) -> anyhow::Result<CitationLock> {
        let path = self.lock_path();
        let text = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "cite {command}: lockfile missing: {} (run: cite bless)",
                path.display()
            )
        })?;
        anyhow::ensure!(
            text.starts_with('{'),
            "cite {command}: {} is not JSON (legacy TOML lock?) — regenerate with: cite bless",
            path.display()
        );
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }

    fn command_check(&self) -> anyhow::Result<()> {
        let cr = self.load_cr()?;
        let lock = self.read_lock("check")?;
        let citations = self.resolved_citations(&cr)?;
        let report = check_report(&citations, &cr, &lock);
        for stale in &report.stale {
            println!("{stale}");
        }
        println!(
            "checked {} citations against cr.txt (eff. {}); {} stale",
            report.checked,
            cr.effective_date,
            report.stale.len()
        );
        anyhow::ensure!(report.stale.is_empty(), "citation check failed");
        Ok(())
    }

    fn command_noncompliant(&self) -> anyhow::Result<()> {
        anyhow::ensure!(
            self.config.format == CitationFormat::Bracketed,
            "cite: --list-noncompliant applies to the bracketed format only"
        );
        let hits = self.noncompliant_hits()?;
        for hit in &hits {
            println!(
                "{}:{}  \"{}\"  {}",
                hit.file, hit.line, hit.matched, hit.context
            );
        }
        println!("{} non-compliant citation-looking string(s)", hits.len());
        anyhow::ensure!(hits.is_empty(), "non-compliant citations found");
        Ok(())
    }

    fn command_list(&self) -> anyhow::Result<()> {
        let cr = self.load_cr()?;
        for citation in self.resolved_citations(&cr)? {
            if let Resolution::Ok(members) = citation.resolution {
                println!(
                    "{}:{}  {}  -> {}",
                    citation.site.file,
                    citation.site.line,
                    self.display_citation(&citation.site.raw),
                    members.join(", ")
                );
            }
        }
        Ok(())
    }

    fn command_bless(&self) -> anyhow::Result<()> {
        let cr = self.load_cr()?;
        let citations = self.resolved_citations(&cr)?;
        let old = self.read_lock("bless").ok();
        let url = resolve_wizards_url(old.as_ref())?;
        let lock = build_lock(&citations, &cr, &url);

        if let Some(old) = &old {
            let newly_registered: Vec<&String> = lock
                .checksums
                .keys()
                .filter(|rule| !old.checksums.contains_key(*rule))
                .collect();
            if !newly_registered.is_empty() {
                println!("newly registered rule(s) — eyeball each against its citing claim:");
                for rule in newly_registered {
                    let preview = cr
                        .text(rule)
                        .map_or("(?)".to_string(), |text| text.chars().take(110).collect());
                    println!("  {rule}: {preview}");
                }
            }
        }

        write_lock(&self.lock_path(), &lock)?;
        println!(
            "blessed {} rules at cr_date {} -> {}",
            lock.checksums.len(),
            cr.effective_date,
            self.lock_path().display()
        );
        Ok(())
    }

    fn command_show(&self, args: &ShowArgs) -> anyhow::Result<()> {
        let cr = self.load_cr()?;
        let mut inner = args.citation.trim();
        if let Some(stripped) = inner.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            inner = stripped;
        }
        inner = inner.strip_prefix("CR#").unwrap_or(inner).trim();
        let resolution = resolve(inner, &cr);
        let members = match resolution {
            Resolution::Ok(members) => members,
            Resolution::Malformed => {
                anyhow::bail!("cite show: malformed citation: {}", args.citation)
            }
            Resolution::Placeholder => anyhow::bail!(
                "cite show: not a citation, an ellipsis placeholder: {}",
                args.citation
            ),
            Resolution::Gone => {
                anyhow::bail!("cite show: range endpoint not found: {}", args.citation)
            }
        };

        let width = std::env::var("COLUMNS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .filter(|width| *width > 20)
            .unwrap_or(80);
        let mut missing = 0;
        for rule in members {
            let Some(text) = cr.text(&rule) else {
                eprintln!("[CR#{rule}]  (not found in the CR)");
                missing += 1;
                continue;
            };
            if args.plain {
                println!("{rule}  {text}");
            } else {
                println!("[CR#{rule}]");
                for line in wrap(text, width.saturating_sub(4)) {
                    println!("    {line}");
                }
            }
        }
        anyhow::ensure!(missing == 0, "{missing} cited rule(s) not found in the CR");
        Ok(())
    }

    fn command_diff(&self, args: &DiffArgs) -> anyhow::Result<()> {
        let lock = self.read_lock("diff")?;
        anyhow::ensure!(
            !lock.wizards_url.is_empty(),
            "cite diff: no wizards_url in the lockfile (legacy lock? re-bless)"
        );
        let old = curl_body(&lock.wizards_url)
            .with_context(|| format!("cite diff: failed to fetch {}", lock.wizards_url))?;
        let current = self.load_cr()?;
        let old_text = rule_text_from_raw_cr(&old, &args.rule);
        let new_text = current.text(&args.rule);
        println!("[CR#{}]", args.rule);
        println!(
            "- {}",
            old_text.as_deref().unwrap_or("(absent in old version)")
        );
        println!("+ {}", new_text.unwrap_or("(absent in current CR)"));
        Ok(())
    }

    fn command_audit(&self, args: &AuditArgs) -> anyhow::Result<()> {
        let cr = self.load_cr()?;
        let citations = self.resolved_citations(&cr)?;
        let selected = if args.diff {
            let diff = read_audit_diff(&self.root)?;
            let added = added_diff_lines(&diff);
            citations
                .into_iter()
                .filter(|citation| {
                    added.contains(&(citation.site.file.clone(), citation.site.line))
                })
                .collect::<Vec<_>>()
        } else if args.files.is_empty() {
            citations
        } else {
            let files: BTreeSet<String> = args
                .files
                .iter()
                .map(|path| self.relative_argument(path))
                .collect();
            citations
                .into_iter()
                .filter(|citation| files.contains(&citation.site.file))
                .collect()
        };

        if selected.is_empty() {
            println!("audited 0 citation site(s) — nothing selected");
            return Ok(());
        }
        for citation in &selected {
            let shown = self.display_citation(&citation.site.raw);
            match &citation.resolution {
                Resolution::Ok(members) => {
                    println!("{}:{}  {shown}", citation.site.file, citation.site.line);
                    println!("  > {}", citation.site.context);
                    for rule in members {
                        println!("    {rule}: {}", cr.text(rule).unwrap_or("(not found)"));
                    }
                    println!();
                }
                status => {
                    println!(
                        "{}:{}  {shown}  !! {} — fix before auditing\n",
                        citation.site.file,
                        citation.site.line,
                        status_name(status)
                    );
                }
            }
        }
        println!(
            "audited {} citation site(s) — read each rule text against its claim",
            selected.len()
        );
        Ok(())
    }

    fn display_citation(&self, raw: &str) -> String {
        match self.config.format {
            CitationFormat::Bracketed => format!("[CR#{raw}]"),
            CitationFormat::Bare => raw.to_string(),
        }
    }

    fn relative_argument(&self, path: &Path) -> String {
        let absolute = if path.is_absolute() { path.to_path_buf() } else { self.root.join(path) };
        let normalized = absolute.canonicalize().unwrap_or(absolute);
        normalized
            .strip_prefix(&self.root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
    }

    fn resolved_citations(&self, cr: &CrSnapshot) -> anyhow::Result<Vec<ResolvedCitation>> {
        Ok(self
            .citation_sites()?
            .into_iter()
            .map(|site| ResolvedCitation {
                resolution: resolve(&site.raw, cr),
                site,
            })
            .collect())
    }

    fn citation_sites(&self) -> anyhow::Result<Vec<CitationSite>> {
        let mut sites = Vec::new();
        let bare = Regex::new(r"\b[1-9][0-9]{2}\.[0-9]+[a-z]?\b")
            .expect("the built-in bare citation regex is valid");
        for (relative, absolute) in self.source_files()? {
            let text = std::fs::read_to_string(&absolute)
                .with_context(|| format!("reading {}", absolute.display()))?;
            for (index, line) in text.lines().enumerate() {
                let refs = match self.config.format {
                    CitationFormat::Bracketed => bracketed_refs(line),
                    CitationFormat::Bare => bare
                        .find_iter(line)
                        .map(|found| found.as_str().to_string())
                        .collect(),
                };
                for raw in refs {
                    sites.push(CitationSite {
                        file: relative.clone(),
                        line: index + 1,
                        context: line.trim().to_string(),
                        raw,
                    });
                }
            }
        }
        Ok(sites)
    }

    fn noncompliant_hits(&self) -> anyhow::Result<Vec<NoncompliantHit>> {
        let mut hits = Vec::new();
        for (relative, absolute) in self.source_files()? {
            let text = std::fs::read_to_string(&absolute)
                .with_context(|| format!("reading {}", absolute.display()))?;
            hits.extend(noncompliant_source_hits(&relative, &text)?);
        }
        Ok(hits)
    }

    fn source_files(&self) -> anyhow::Result<Vec<(String, PathBuf)>> {
        let relative = if self.config.sources.tracked {
            tracked_files(&self.root)?
        } else {
            matching_glob_files(&self.root, &self.config.sources.globs)?
        };
        let mut files = Vec::new();
        for relative in relative {
            let relative = relative.replace('\\', "/");
            if self
                .config
                .sources
                .exclude
                .iter()
                .any(|prefix| relative.starts_with(prefix))
            {
                continue;
            }
            if !self.config.sources.extensions.is_empty()
                && !Path::new(&relative)
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .is_some_and(|extension| {
                        self.config
                            .sources
                            .extensions
                            .iter()
                            .any(|wanted| wanted == extension)
                    })
            {
                continue;
            }
            let absolute = self.root.join(&relative);
            if absolute.is_file() {
                files.push((relative, absolute));
            }
        }
        anyhow::ensure!(
            !files.is_empty(),
            "cite: no source files matched the config"
        );
        Ok(files)
    }
}

impl CrSnapshot {
    fn parse(text: &str) -> anyhow::Result<Self> {
        let effective_date = parse_effective_date(text)?;
        let mut order = Vec::new();
        let mut rules = BTreeMap::new();
        let mut sections = BTreeMap::new();
        let mut continuation_target = None;
        for line in text.lines() {
            if let Some((rule, body, kind)) = parse_rule_line(line) {
                continuation_target = Some((kind, rule.clone()));
                match kind {
                    RuleLineKind::Leaf => {
                        if !rules.contains_key(&rule) {
                            order.push(rule.clone());
                        }
                        rules.insert(rule, body);
                    }
                    RuleLineKind::Section => {
                        sections.insert(rule, body);
                    }
                }
                continue;
            }

            let line = line.trim_end_matches('\r');
            let continuation = normalize_rule_text(line);
            let is_indented = line.as_bytes().first().is_some_and(u8::is_ascii_whitespace);
            if is_indented
                && !continuation.is_empty()
                && !continuation.starts_with("Example:")
                && let Some((kind, rule)) = &continuation_target
            {
                let target = match kind {
                    RuleLineKind::Leaf => rules.get_mut(rule),
                    RuleLineKind::Section => sections.get_mut(rule),
                };
                if let Some(target) = target {
                    target.push(' ');
                    target.push_str(&continuation);
                    continue;
                }
            }
            continuation_target = None;
        }
        anyhow::ensure!(!rules.is_empty(), "cite: no numbered rules found in cr.txt");
        let indexes = order
            .iter()
            .enumerate()
            .map(|(index, rule)| (rule.clone(), index))
            .collect();
        Ok(Self {
            effective_date,
            order,
            indexes,
            rules,
            sections,
        })
    }

    fn text(&self, rule: &str) -> Option<&str> {
        self.rules
            .get(rule)
            .or_else(|| self.sections.get(rule))
            .map(String::as_str)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuleLineKind {
    Leaf,
    Section,
}

fn parse_rule_line(line: &str) -> Option<(String, String, RuleLineKind)> {
    let line = line.trim_end_matches('\r');
    let (token, body) = line.split_once(' ')?;
    let body = normalize_rule_text(body);
    if body.is_empty() {
        return None;
    }
    let stripped = token.strip_suffix('.').unwrap_or(token);
    if is_leaf_rule(stripped) {
        return Some((stripped.to_string(), body, RuleLineKind::Leaf));
    }
    if token.ends_with('.')
        && !stripped.is_empty()
        && stripped.len() <= 3
        && stripped.chars().all(|character| character.is_ascii_digit())
    {
        return Some((stripped.to_string(), body, RuleLineKind::Section));
    }
    None
}

fn parse_effective_date(text: &str) -> anyhow::Result<String> {
    const MONTHS: &[&str] = &[
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let line = text
        .lines()
        .find(|line| line.contains("effective as of"))
        .context("cite: cannot parse the effective date from cr.txt")?;
    let rest = line
        .split_once("effective as of ")
        .map(|(_, rest)| rest)
        .context("cite: cannot parse the effective date from cr.txt")?;
    let mut parts = rest.trim_end_matches(['.', '\r']).split_whitespace();
    let month = parts.next().context("cite: effective date has no month")?;
    let day = parts
        .next()
        .context("cite: effective date has no day")?
        .trim_end_matches(',')
        .parse::<u8>()?;
    let year = parts
        .next()
        .context("cite: effective date has no year")?
        .parse::<u16>()?;
    let month_number = MONTHS
        .iter()
        .position(|candidate| *candidate == month)
        .map(|index| index + 1)
        .with_context(|| format!("cite: unknown month in effective date: {month}"))?;
    Ok(format!("{year:04}-{month_number:02}-{day:02}"))
}

fn normalize_rule_text(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_leaf_rule(rule: &str) -> bool {
    let Some((section, rest)) = rule.split_once('.') else {
        return false;
    };
    if section.is_empty()
        || section.len() > 3
        || !section.chars().all(|character| character.is_ascii_digit())
    {
        return false;
    }
    let digit_count = rest.chars().take_while(char::is_ascii_digit).count();
    digit_count > 0
        && rest[digit_count..]
            .chars()
            .all(|character| character.is_ascii_lowercase())
}

fn is_valid_rule(rule: &str) -> bool {
    (!rule.is_empty()
        && rule.len() <= 3
        && rule.chars().all(|character| character.is_ascii_digit()))
        || is_leaf_rule(rule)
}

fn bracketed_refs(line: &str) -> Vec<String> {
    let mut refs = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find("[CR#") {
        rest = &rest[start + 4..];
        let Some(end) = rest.find(']') else {
            break;
        };
        refs.push(rest[..end].to_string());
        rest = &rest[end + 1..];
    }
    refs
}

fn resolve(raw: &str, cr: &CrSnapshot) -> Resolution {
    if matches!(raw.trim(), "..." | "…") {
        return Resolution::Placeholder;
    }
    let parts: Vec<&str> = raw.split(',').collect();
    if parts.is_empty() {
        return Resolution::Malformed;
    }
    let mut members = Vec::new();
    for part in parts {
        if part.contains("..") {
            let endpoints: Vec<&str> = part.split("..").collect();
            if endpoints.len() != 2 || !is_valid_rule(endpoints[0]) || !is_valid_rule(endpoints[1])
            {
                return Resolution::Malformed;
            }
            let Some(start) = cr.indexes.get(endpoints[0]).copied() else {
                return Resolution::Gone;
            };
            let Some(end) = cr.indexes.get(endpoints[1]).copied() else {
                return Resolution::Gone;
            };
            if start > end {
                return Resolution::Gone;
            }
            members.extend(cr.order[start..=end].iter().cloned());
        } else if is_valid_rule(part) {
            members.push(part.to_string());
        } else {
            return Resolution::Malformed;
        }
    }
    Resolution::Ok(members)
}

fn check_report(
    citations: &[ResolvedCitation],
    cr: &CrSnapshot,
    lock: &CitationLock,
) -> CheckReport {
    let mut stale = Vec::new();
    let mut checked = 0;
    for citation in citations {
        match &citation.resolution {
            Resolution::Placeholder => {}
            Resolution::Malformed => {
                checked += 1;
                stale.push(stale_line("MALFORMED", &citation.site.raw, &citation.site));
            }
            Resolution::Gone => {
                checked += 1;
                stale.push(stale_line("GONE", &citation.site.raw, &citation.site));
            }
            Resolution::Ok(members) => {
                checked += members.len();
                for rule in members {
                    let Some(text) = cr.text(rule) else {
                        stale.push(stale_line("GONE", rule, &citation.site));
                        continue;
                    };
                    let Some(baseline) = lock.checksums.get(rule) else {
                        stale.push(stale_line("UNLOCKED", rule, &citation.site));
                        continue;
                    };
                    if hash_rule_text(text) != *baseline {
                        stale.push(stale_line("CHANGED", rule, &citation.site));
                    }
                }
            }
        }
    }
    CheckReport { stale, checked }
}

fn stale_line(tag: &str, rule: &str, site: &CitationSite) -> String {
    format!(
        "{tag}  {rule}  {}:{}  {}",
        site.file, site.line, site.context
    )
}

fn hash_rule_text(text: &str) -> String {
    let digest = sha2::Sha256::digest(normalize_rule_text(text).as_bytes());
    let mut short = String::with_capacity(16);
    for byte in &digest[..8] {
        write!(&mut short, "{byte:02x}").expect("writing to String cannot fail");
    }
    short
}

fn build_lock(citations: &[ResolvedCitation], cr: &CrSnapshot, url: &str) -> CitationLock {
    let mut members = BTreeSet::new();
    for citation in citations {
        if let Resolution::Ok(rules) = &citation.resolution {
            members.extend(rules.iter().cloned());
        }
    }
    let checksums = members
        .into_iter()
        .filter_map(|rule| cr.text(&rule).map(|text| (rule, hash_rule_text(text))))
        .collect();
    CitationLock {
        checksums,
        cr_date: cr.effective_date.clone(),
        wizards_url: url.to_string(),
    }
}

fn write_lock(path: &Path, lock: &CitationLock) -> anyhow::Result<()> {
    let mut rendered = serde_json::to_string_pretty(lock)?;
    rendered.push('\n');
    std::fs::write(path, rendered).with_context(|| format!("writing {}", path.display()))
}

fn resolve_wizards_url(old: Option<&CitationLock>) -> anyhow::Result<String> {
    if let Ok(url) = std::env::var("MTG_RULES_CITE_WIZARDS_URL")
        && !url.is_empty()
    {
        return Ok(url);
    }
    if let Some(url) = curl_effective_url() {
        return Ok(url);
    }
    if let Some(old) = old
        && !old.wizards_url.is_empty()
    {
        return Ok(old.wizards_url.clone());
    }
    anyhow::bail!(
        "cite bless: cannot resolve the wizards URL (offline?) and no existing lock to reuse"
    )
}

fn curl_effective_url() -> Option<String> {
    let output = Command::new("curl")
        .args([
            "-fsSIL",
            "-o",
            "/dev/null",
            "-w",
            "%{url_effective}",
            "--max-time",
            "15",
            "https://academyruins.com/link/cr",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let url = String::from_utf8(output.stdout).ok()?;
    (!url.trim().is_empty()).then(|| url.trim().to_string())
}

fn curl_body(url: &str) -> anyhow::Result<String> {
    let output = Command::new("curl")
        .args(["-fsL", "--max-time", "60", url])
        .output()
        .context("spawning curl")?;
    anyhow::ensure!(
        output.status.success(),
        "curl exited with {}",
        output.status
    );
    String::from_utf8(output.stdout).context("downloaded CR is not UTF-8")
}

fn rule_text_from_raw_cr(text: &str, wanted: &str) -> Option<String> {
    CrSnapshot::parse(text)
        .ok()?
        .text(wanted)
        .map(ToString::to_string)
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        let needed = current.len() + usize::from(!current.is_empty()) + word.len();
        if needed > width && !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn status_name(status: &Resolution) -> &'static str {
    match status {
        Resolution::Ok(_) => "ok",
        Resolution::Malformed => "skip",
        Resolution::Gone => "gone",
        Resolution::Placeholder => "placeholder",
    }
}

fn noncompliant_matches(line: &str) -> Vec<String> {
    let blanked = blank_bracketed_citations(line);
    let mut by_offset = BTreeMap::new();
    for (pattern_index, pattern) in noncompliant_patterns().iter().enumerate() {
        for found in pattern.find_iter(&blanked) {
            if pattern_index == 1
                && blanked[..found.start()]
                    .chars()
                    .next_back()
                    .is_some_and(|character| {
                        character.is_alphanumeric() || matches!(character, '_' | '-')
                    })
            {
                continue;
            }
            by_offset
                .entry(found.start())
                .or_insert_with(|| found.as_str().to_string());
        }
    }
    by_offset.into_values().collect()
}

fn noncompliant_source_hits(file: &str, source: &str) -> anyhow::Result<Vec<NoncompliantHit>> {
    const SCOPE_BEGIN: &str =
        "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims";
    const SCOPE_END: &str = "// cite: noncompliant end";
    const LINE_EXEMPT: &str = "// cite: noncompliant-line -- machine-readable parser key";

    let lines: Vec<&str> = source.lines().collect();
    let marker_lines: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| line.contains("cite: noncompliant").then_some(index + 1))
        .collect();
    if Path::new(file).extension() != Some(std::ffi::OsStr::new("rs")) {
        anyhow::ensure!(
            marker_lines.is_empty(),
            "cite: noncompliant exemptions require Rust source: {file}:{}",
            marker_lines.first().copied().unwrap_or_default()
        );
        return Ok(noncompliant_hits_on_lines(file, &lines, &BTreeSet::new()));
    }

    let syntax = syn::parse_file(source)
        .with_context(|| format!("parsing Rust source {file} for citation exemptions"))?;
    let mut sites = RustExemptionSites::default();
    sites.visit_file(&syntax);
    let mut exempt_lines = BTreeSet::new();
    for fixture in sites.fixtures {
        let Some(begin) = fixture.start_line.checked_sub(1) else {
            continue;
        };
        let end = fixture.end_line + 1;
        if lines
            .get(begin - 1)
            .is_some_and(|text| text.trim() == SCOPE_BEGIN)
            && lines
                .get(end - 1)
                .is_some_and(|text| text.trim() == SCOPE_END)
        {
            exempt_lines.extend(begin..=end);
        }
    }

    for line_number in marker_lines {
        if exempt_lines.contains(&line_number) {
            continue;
        }
        let line = lines[line_number - 1];
        let marker_column = line.trim_end().strip_suffix(LINE_EXEMPT).map(str::len);
        let valid_line_exemption = marker_column.is_some_and(|column| {
            sites
                .rule_arguments
                .iter()
                .any(|argument| argument.precedes_marker_on_line(line, line_number, column))
        });
        anyhow::ensure!(
            valid_line_exemption,
            "cite: noncompliant marker is not at a structural Rust position in {file}:{line_number}"
        );
        exempt_lines.insert(line_number);
    }

    Ok(noncompliant_hits_on_lines(file, &lines, &exempt_lines))
}

fn noncompliant_hits_on_lines(
    file: &str,
    lines: &[&str],
    exempt_lines: &BTreeSet<usize>,
) -> Vec<NoncompliantHit> {
    let mut hits = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;
        if exempt_lines.contains(&line_number) {
            continue;
        }
        for matched in noncompliant_matches(line) {
            hits.push(NoncompliantHit {
                file: file.to_owned(),
                line: line_number,
                matched,
                context: line.trim().to_string(),
            });
        }
    }
    hits
}

#[derive(Debug, Default)]
struct RustExemptionSites {
    rule_arguments: Vec<RuleArgumentSite>,
    fixtures: Vec<FixtureItemSite>,
}

#[derive(Debug)]
struct RuleArgumentSite {
    line: usize,
    end_column: usize,
}

impl RuleArgumentSite {
    fn precedes_marker_on_line(
        &self,
        source_line: &str,
        line: usize,
        marker_column: usize,
    ) -> bool {
        self.line == line
            && self.end_column <= marker_column
            && source_line
                .get(self.end_column..marker_column)
                .is_some_and(|between| between.trim() == ",")
    }
}

#[derive(Debug)]
struct FixtureItemSite {
    start_line: usize,
    end_line: usize,
}

impl<'ast> syn::visit::Visit<'ast> for RustExemptionSites {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        self.collect_rule_arguments(&call.args);
        syn::visit::visit_expr_call(self, call);
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        self.collect_rule_arguments(&call.args);
        syn::visit::visit_expr_method_call(self, call);
    }

    fn visit_item_const(&mut self, item: &'ast syn::ItemConst) {
        if is_string_fixture_const(item) {
            self.fixtures.push(FixtureItemSite {
                start_line: item.const_token.span.start().line,
                end_line: item.semi_token.span.end().line,
            });
        }
        syn::visit::visit_item_const(self, item);
    }
}

impl RustExemptionSites {
    fn collect_rule_arguments(
        &mut self,
        arguments: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
    ) {
        for argument in arguments {
            let syn::Expr::Lit(expression) = argument else {
                continue;
            };
            let syn::Lit::Str(literal) = &expression.lit else {
                continue;
            };
            if is_rule_parser_key(&literal.value()) {
                let span = literal.span();
                let start = span.start();
                let end = span.end();
                if start.line == end.line {
                    self.rule_arguments.push(RuleArgumentSite {
                        line: end.line,
                        end_column: end.column,
                    });
                }
            }
        }
    }
}

fn is_rule_parser_key(value: &str) -> bool {
    static RULE_KEY: OnceLock<Regex> = OnceLock::new();
    RULE_KEY
        .get_or_init(|| {
            Regex::new(r"^[1-9][0-9]{2}\.[0-9]+[a-z]?$")
                .expect("the parser rule-key regex is valid")
        })
        .is_match(value)
}

fn is_string_fixture_const(item: &syn::ItemConst) -> bool {
    let name = item.ident.to_string();
    let uppercase_fixture = name.ends_with("_FIXTURE")
        && name
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_');
    let syn::Type::Reference(reference) = item.ty.as_ref() else {
        return false;
    };
    let syn::Type::Path(path) = reference.elem.as_ref() else {
        return false;
    };
    let syn::Expr::Lit(expression) = item.expr.as_ref() else {
        return false;
    };
    uppercase_fixture
        && reference.mutability.is_none()
        && path.qself.is_none()
        && path.path.is_ident("str")
        && matches!(expression.lit, syn::Lit::Str(_))
}

fn noncompliant_patterns() -> &'static [Regex; 3] {
    static PATTERNS: OnceLock<[Regex; 3]> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        [
            Regex::new(r"CR ?[0-9]{1,3}(?:\.[0-9]+[a-z]*)?")
                .expect("the built-in CR prose regex is valid"),
            Regex::new(r"rule [0-9]{1,3}(?:\.[0-9]+[a-z]*)?")
                .expect("the built-in rule prose regex is valid"),
            Regex::new(r"\b[0-9]{3}\.[0-9]+[a-z]*\b")
                .expect("the built-in bare rule regex is valid"),
        ]
    })
}

fn blank_bracketed_citations(line: &str) -> String {
    let mut blanked = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(start) = rest.find("[CR#") {
        blanked.push_str(&rest[..start]);
        let citation = &rest[start..];
        let Some(end) = citation.find(']') else {
            blanked.push_str(citation);
            return blanked;
        };
        blanked.extend(std::iter::repeat_n(' ', citation[..=end].chars().count()));
        rest = &citation[end + 1..];
    }
    blanked.push_str(rest);
    blanked
}

fn tracked_files(root: &Path) -> anyhow::Result<Vec<String>> {
    let jj = Command::new("jj")
        .arg("-R")
        .arg(root)
        .args(["file", "list"])
        .output();
    if let Ok(output) = jj
        && output.status.success()
    {
        let files = output_lines(&output.stdout);
        if !files.is_empty() {
            return Ok(files);
        }
    }

    let git = Command::new("git")
        .arg("-C")
        .arg(root)
        .arg("ls-files")
        .output();
    if let Ok(output) = git
        && output.status.success()
    {
        let files = output_lines(&output.stdout);
        if !files.is_empty() {
            return Ok(files);
        }
    }
    anyhow::bail!(
        "cite: no tracked files (neither jj nor git answered in {})",
        root.display()
    )
}

fn output_lines(stdout: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn matching_glob_files(root: &Path, globs: &[String]) -> anyhow::Result<Vec<String>> {
    let mut matched = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        for entry in std::fs::read_dir(&directory)
            .with_context(|| format!("reading {}", directory.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let relative = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            if globs.iter().any(|glob| glob_matches(glob, &relative)) {
                matched.push(relative);
            }
        }
    }
    matched.sort();
    matched.dedup();
    Ok(matched)
}

fn glob_matches(pattern: &str, value: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let value: Vec<char> = value.chars().collect();
    let mut table = vec![vec![false; value.len() + 1]; pattern.len() + 1];
    table[0][0] = true;
    for index in 0..pattern.len() {
        if pattern[index] == '*' {
            table[index + 1][0] = table[index][0];
        }
        for offset in 0..value.len() {
            table[index + 1][offset + 1] = match pattern[index] {
                '*' => table[index][offset + 1] || table[index + 1][offset],
                '?' => table[index][offset],
                literal => table[index][offset] && literal == value[offset],
            };
        }
    }
    table[pattern.len()][value.len()]
}

fn read_audit_diff(root: &Path) -> anyhow::Result<String> {
    if !std::io::stdin().is_terminal() {
        let mut diff = String::new();
        std::io::stdin().read_to_string(&mut diff)?;
        return Ok(diff);
    }
    for mut command in [
        {
            let mut command = Command::new("jj");
            command.arg("-R").arg(root).args(["diff", "--git"]);
            command
        },
        {
            let mut command = Command::new("git");
            command.arg("-C").arg(root).arg("diff");
            command
        },
    ] {
        if let Ok(output) = command.output()
            && output.status.success()
        {
            return String::from_utf8(output.stdout).context("VCS diff is not UTF-8");
        }
    }
    anyhow::bail!(
        "cite audit: no diff on stdin and neither jj nor git produced one in {}",
        root.display()
    )
}

fn added_diff_lines(diff: &str) -> BTreeSet<(String, usize)> {
    let mut added = BTreeSet::new();
    let mut file = None;
    let mut new_line = 0;
    for line in diff.lines() {
        if let Some(path) = line.strip_prefix("+++ ") {
            let path = path.split_whitespace().next().unwrap_or(path);
            file = Some(path.strip_prefix("b/").unwrap_or(path).to_string());
            continue;
        }
        if line.starts_with("@@ ") {
            new_line = hunk_new_start(line).unwrap_or(0).saturating_sub(1);
            continue;
        }
        if line.starts_with('+') {
            new_line += 1;
            if let Some(file) = &file
                && file != "/dev/null"
            {
                added.insert((file.clone(), new_line));
            }
        } else if line.starts_with(' ') {
            new_line += 1;
        }
    }
    added
}

fn hunk_new_start(header: &str) -> Option<usize> {
    let plus = header.find('+')? + 1;
    let digits: String = header[plus..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    digits.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CR: &str = "Magic: The Gathering Comprehensive Rules\r\n\
These rules are effective as of January 2, 2026.\r\n\
\r\n\
Contents\r\n\
100. General\r\n\
200. General\r\n\
\r\n\
1. Game Concepts\r\n\
100. General\r\n\
100.1. Alpha   rule.\r\n\
\x20\x20\x20\x20\x20Continued definition.\r\n\
\x20\x20\x20\x20\x20Example: This illustrative text is excluded.\r\n\
100.1a First child.\r\n\
100.1c Third child after a skipped letter.\r\n\
200. General\r\n\
200.1. Another rule.\r\n";

    #[test]
    fn cr_txt_is_the_only_rules_source_and_normalizes_hash_text() {
        let cr = CrSnapshot::parse(CR).unwrap();
        assert_eq!(cr.effective_date, "2026-01-02");
        assert_eq!(cr.text("100.1"), Some("Alpha rule. Continued definition."));
        assert_eq!(cr.text("100"), Some("General"));
        assert_eq!(cr.order, ["100.1", "100.1a", "100.1c", "200.1"]);
    }

    #[test]
    fn indented_rule_continuations_are_hashed_but_examples_are_not() {
        let cr = CrSnapshot::parse(CR).unwrap();
        let text = cr.text("100.1").unwrap();
        assert_eq!(text, "Alpha rule. Continued definition.");
        assert_eq!(
            hash_rule_text(text),
            hash_rule_text("Alpha rule. Continued definition.")
        );
        assert!(!text.contains("illustrative"));
        assert_eq!(rule_text_from_raw_cr(CR, "100.1").as_deref(), Some(text));
    }

    #[test]
    fn ranges_expand_in_document_order_across_skipped_letters() {
        let cr = CrSnapshot::parse(CR).unwrap();
        assert_eq!(
            resolve("100.1a..100.1c", &cr),
            Resolution::Ok(vec!["100.1a".into(), "100.1c".into()])
        );
        assert_eq!(resolve("100.1c..100.1a", &cr), Resolution::Gone);
        assert_eq!(resolve("100..200", &cr), Resolution::Gone);
    }

    #[test]
    fn stale_hashes_and_malformed_tokens_are_distinct_failures() {
        let cr = CrSnapshot::parse(CR).unwrap();
        let sites = [
            CitationSite {
                file: "src/lib.rs".into(),
                line: 1,
                context: "// [CR#100.1]".into(),
                raw: "100.1".into(),
            },
            CitationSite {
                file: "src/lib.rs".into(),
                line: 2,
                context: "// [CR#rule]".into(),
                raw: "rule".into(),
            },
        ];
        let citations: Vec<_> = sites
            .into_iter()
            .map(|site| ResolvedCitation {
                resolution: resolve(&site.raw, &cr),
                site,
            })
            .collect();
        let lock = CitationLock {
            checksums: BTreeMap::from([("100.1".into(), "deadbeefdeadbeef".into())]),
            cr_date: "2025-01-01".into(),
            wizards_url: "https://example.invalid/cr.txt".into(),
        };
        let report = check_report(&citations, &cr, &lock);
        assert_eq!(report.checked, 2);
        assert!(report.stale[0].starts_with("CHANGED  100.1  "));
        assert!(report.stale[1].starts_with("MALFORMED  rule  "));
    }

    #[test]
    fn noncompliant_scan_blanks_brackets_and_rejects_loose_forms() {
        assert!(noncompliant_matches("// [CR#100.1,200.1]").is_empty());
        assert!(noncompliant_matches("document --per-rule 200 here").is_empty());
        assert_eq!(
            noncompliant_matches("CR 100.1 and rule 200.1"),
            ["CR 100.1", "100.1", "rule 200.1", "200.1"]
        );
        // A malformed bracket is owned by `check`, not by this wide-net scan.
        assert!(noncompliant_matches("[CR#rule 100.1]").is_empty());
    }

    #[test]
    fn noncompliant_scan_accepts_exact_parser_key_and_fixture_markers() {
        let source = "\
// CR 100.1
fn parse_rule(_: &str, _: ()) {}
fn example() {
    parse_rule(
        \"100.1\", // cite: noncompliant-line -- machine-readable parser key
        (),
    );
}
// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims
const CR_FIXTURE: &str = \"rule 200.1\";
// cite: noncompliant end
// rule 300.1
";
        let hits = noncompliant_source_hits("src/lib.rs", source).unwrap();
        assert_eq!(
            hits.iter()
                .map(|hit| hit.matched.as_str())
                .collect::<Vec<_>>(),
            ["CR 100.1", "100.1", "rule 300.1", "300.1"]
        );
    }

    #[test]
    fn noncompliant_scan_rejects_nested_fixture_scope() {
        let error = noncompliant_source_hits(
            "src/lib.rs",
            "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
const CR_FIXTURE: &str = \"rule 100.1\";\n\
// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n",
        )
        .unwrap_err();
        assert!(error.to_string().contains("structural Rust position"));
    }

    #[test]
    fn noncompliant_scan_rejects_unmatched_fixture_scope_end() {
        let unmatched_end =
            noncompliant_source_hits("src/lib.rs", "// cite: noncompliant end\n// rule 100.1\n")
                .unwrap_err();
        assert!(
            unmatched_end
                .to_string()
                .contains("structural Rust position"),
            "{unmatched_end:#}"
        );
    }

    #[test]
    fn noncompliant_scan_rejects_unclosed_fixture_scope() {
        let unclosed = noncompliant_source_hits(
            "src/lib.rs",
            "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
const CR_FIXTURE: &str = \"rule 100.1\";\n",
        )
        .unwrap_err();
        assert!(
            unclosed.to_string().contains("structural Rust position"),
            "{unclosed:#}"
        );
    }

    #[test]
    fn noncompliant_scan_rejects_malformed_markers() {
        for malformed in [
            "// cite: noncompliant beginning -- verbatim CR parser fixture, not prose claims",
            "// cite: noncompliant endless",
            "fn f() { parse_rule(\"100.1\"); } // cite: noncompliant-line-altered -- machine-readable parser key",
        ] {
            let error = noncompliant_source_hits("src/lib.rs", malformed).unwrap_err();
            assert!(
                error.to_string().contains("structural Rust position"),
                "{malformed:?}: {error:#}"
            );
        }
    }

    #[test]
    fn noncompliant_scan_rejects_line_marker_on_prose_or_non_key_code() {
        for misuse in [
            "// CR 100.1 // cite: noncompliant-line -- machine-readable parser key",
            "fn f() { let key = rule_number; } // cite: noncompliant-line -- machine-readable parser key",
            "/* \"100.1\", */ // cite: noncompliant-line -- machine-readable parser key",
        ] {
            let error = noncompliant_source_hits("src/lib.rs", misuse).unwrap_err();
            assert!(
                error.to_string().contains("structural Rust position"),
                "{misuse:?}: {error:#}"
            );
        }
    }

    #[test]
    fn noncompliant_scan_rejects_scope_around_prose_or_non_fixture_code() {
        for misuse in [
            "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
// rule 100.1\n\
// cite: noncompliant end\n",
            "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
fn prose() { /* rule 100.1 */ }\n\
// cite: noncompliant end\n",
            "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
const OTHER_FIXTURE: usize = 100;\n\
// cite: noncompliant end\n",
        ] {
            let error = noncompliant_source_hits("src/lib.rs", misuse).unwrap_err();
            assert!(
                error.to_string().contains("structural Rust position"),
                "{misuse:?}: {error:#}"
            );
        }
    }

    #[test]
    fn noncompliant_scan_rejects_markers_outside_rust() {
        let prose = "\
// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims
const CR_FIXTURE: &str = \"rule 100.1\";
// cite: noncompliant end
";
        for file in ["notes.md", "data.ron", "proof.idr"] {
            let error = noncompliant_source_hits(file, prose).unwrap_err();
            assert!(
                error.to_string().contains("Rust source"),
                "{file}: {error:#}"
            );
        }
    }

    #[test]
    fn noncompliant_scan_rejects_line_markers_spoofed_inside_rust_strings() {
        let ordinary = r#"const SPOOF: &str = "// CR 100.1 // cite: noncompliant-line -- machine-readable parser key";"#;
        let raw = r##"const SPOOF: &str = r#"
parse_rule("100.1", // cite: noncompliant-line -- machine-readable parser key
"#;
"##;
        for source in [ordinary, raw] {
            let error = noncompliant_source_hits("src/lib.rs", source).unwrap_err();
            assert!(
                error.to_string().contains("structural Rust position"),
                "{error:#}"
            );
        }
    }

    #[test]
    fn noncompliant_scan_rejects_scope_markers_spoofed_inside_rust_strings() {
        let ordinary = r#"const SPOOF: &str = "
// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims
const CR_FIXTURE: &str = \"rule 100.1\";
// cite: noncompliant end
";"#;
        let raw = r##"const SPOOF: &str = r#"
// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims
const CR_FIXTURE: &str = "rule 100.1";
// cite: noncompliant end
"#;
"##;
        for source in [ordinary, raw] {
            let error = noncompliant_source_hits("src/lib.rs", source).unwrap_err();
            assert!(
                error.to_string().contains("structural Rust position"),
                "{error:#}"
            );
        }
    }

    #[test]
    fn bless_rebuilds_lock_from_current_citation_members() {
        let cr = CrSnapshot::parse(CR).unwrap();
        let citations = [ResolvedCitation {
            site: CitationSite {
                file: "src/lib.rs".into(),
                line: 1,
                context: "// [CR#100.1a..100.1c,200.1]".into(),
                raw: "100.1a..100.1c,200.1".into(),
            },
            resolution: Resolution::Ok(vec!["100.1a".into(), "100.1c".into(), "200.1".into()]),
        }];
        let lock = build_lock(&citations, &cr, "https://example.invalid/cr.txt");
        assert_eq!(
            lock.checksums.keys().cloned().collect::<Vec<_>>(),
            ["100.1a", "100.1c", "200.1"]
        );
        assert_eq!(lock.cr_date, "2026-01-02");
        assert_eq!(lock.checksums["100.1a"], hash_rule_text("First child."));
    }

    #[test]
    fn obsolete_file_exemption_key_cannot_bypass_noncompliant_scan() {
        let fixture = Fixture::new();
        fixture.write("data/rules/cr.txt", CR);
        fixture.write(
            "cite-config.json",
            r#"{
                "format": "bracketed",
                "lockfile": "cr-citations.lock",
                "sources": { "globs": ["src/*.rs"] },
                "noncompliant_exempt": ["src/exempt.rs"]
            }"#,
        );
        fixture.write("src/main.rs", "// [CR#100.1a..100.1c]\n// CR 200.1\n");
        fixture.write("src/exempt.rs", "// rule 200.1\n");
        let repo = Repository::load(fixture.path()).unwrap();
        let cr = repo.load_cr().unwrap();
        let citations = repo.resolved_citations(&cr).unwrap();
        let first = build_lock(&citations, &cr, "https://example.invalid/old.txt");
        write_lock(&repo.lock_path(), &first).unwrap();
        assert_eq!(repo.noncompliant_hits().unwrap().len(), 4);

        fixture.write("src/main.rs", "// [CR#200.1]\n");
        let citations = repo.resolved_citations(&cr).unwrap();
        let updated = build_lock(&citations, &cr, &first.wizards_url);
        write_lock(&repo.lock_path(), &updated).unwrap();
        let reread = repo.read_lock("test").unwrap();
        assert_eq!(
            reread.checksums.keys().cloned().collect::<Vec<_>>(),
            ["200.1"]
        );
        assert_eq!(reread.wizards_url, first.wizards_url);
    }

    #[test]
    fn audit_file_selection_accepts_relative_and_absolute_paths() {
        let fixture = Fixture::new();
        fixture.write("data/rules/cr.txt", CR);
        fixture.write(
            "cite-config.json",
            r#"{
                "format": "bracketed",
                "sources": { "globs": ["src/*.rs"] }
            }"#,
        );
        fixture.write("src/main.rs", "// [CR#100.1]\n");
        fixture.write("nested/.keep", "");
        let repo = Repository::load(&fixture.path().join("nested/..")).unwrap();
        let relative = repo.relative_argument(Path::new("src/main.rs"));
        let absolute = repo.relative_argument(&fixture.path().join("src/main.rs"));
        assert_eq!(relative, "src/main.rs");
        assert_eq!(absolute, relative);
        let citations = repo.resolved_citations(&repo.load_cr().unwrap()).unwrap();
        assert_eq!(
            citations
                .iter()
                .filter(|citation| citation.site.file == absolute)
                .count(),
            1
        );
    }

    #[test]
    fn added_diff_lines_tracks_new_file_line_numbers() {
        let diff = concat!(
            "diff --git a/src/a.rs b/src/a.rs\n",
            "--- a/src/a.rs\n",
            "+++ b/src/a.rs\n",
            "@@ -2,2 +2,3 @@\n",
            " context\n",
            "+added\n",
            "-deleted\n",
            " context two\n",
        );
        assert_eq!(
            added_diff_lines(diff),
            BTreeSet::from([("src/a.rs".into(), 3)])
        );
    }

    struct Fixture(PathBuf);

    impl Fixture {
        fn new() -> Self {
            use std::sync::atomic::AtomicU32;
            use std::sync::atomic::Ordering;
            static NEXT: AtomicU32 = AtomicU32::new(0);
            let path = std::env::temp_dir().join(format!(
                "xtask-cite-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&path).unwrap();
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }

        fn write(&self, relative: &str, text: &str) {
            let path = self.0.join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
    }

    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}
