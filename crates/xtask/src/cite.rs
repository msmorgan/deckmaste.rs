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

#[cfg(test)]
fn noncompliant_matches(line: &str) -> Vec<String> {
    noncompliant_match_ranges(line)
        .into_iter()
        .map(|matched| matched.text)
        .collect()
}

fn noncompliant_match_ranges(line: &str) -> Vec<NoncompliantMatch> {
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
                .or_insert_with(|| NoncompliantMatch {
                    start: blanked[..found.start()].chars().count(),
                    end: blanked[..found.end()].chars().count(),
                    text: found.as_str().to_string(),
                });
        }
    }
    by_offset.into_values().collect()
}

#[derive(Debug)]
struct NoncompliantMatch {
    start: usize,
    end: usize,
    text: String,
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
        return Ok(noncompliant_hits_on_lines(file, &lines, &[]));
    }

    let syntax = syn::parse_file(source)
        .with_context(|| format!("parsing Rust source {file} for citation exemptions"))?;
    let mut sites = RustExemptionSites::for_file(file, &syntax)?;
    sites.visit_file(&syntax);
    let mut valid_marker_lines = BTreeSet::new();
    let mut exempt_spans = Vec::new();
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
            valid_marker_lines.extend([begin, end]);
            exempt_spans.push(fixture.literal_span);
        }
    }

    for line_number in marker_lines {
        if valid_marker_lines.contains(&line_number) {
            continue;
        }
        let line = lines[line_number - 1];
        let marker_column = line
            .trim_end()
            .strip_suffix(LINE_EXEMPT)
            .map(|prefix| prefix.chars().count());
        let matching_arguments = marker_column.map_or_else(Vec::new, |column| {
            sites
                .rule_arguments
                .iter()
                .filter(|argument| argument.precedes_marker_on_line(line, line_number, column))
                .collect::<Vec<_>>()
        });
        anyhow::ensure!(
            matching_arguments.len() == 1,
            "cite: noncompliant marker is not at a structural Rust position in {file}:{line_number}"
        );
        exempt_spans.push(matching_arguments[0].literal_span);
    }

    Ok(noncompliant_hits_on_lines(file, &lines, &exempt_spans))
}

fn noncompliant_hits_on_lines(
    file: &str,
    lines: &[&str],
    exempt_spans: &[SourceSpan],
) -> Vec<NoncompliantHit> {
    let mut hits = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;
        for matched in noncompliant_match_ranges(line) {
            if exempt_spans
                .iter()
                .any(|span| span.contains(line_number, matched.start, matched.end))
            {
                continue;
            }
            hits.push(NoncompliantHit {
                file: file.to_owned(),
                line: line_number,
                matched: matched.text,
                context: line.trim().to_string(),
            });
        }
    }
    hits
}

#[derive(Debug)]
struct RustExemptionSites {
    allow_rule_arguments: bool,
    rule_arguments: Vec<RuleArgumentSite>,
    fixtures: Vec<FixtureItemSite>,
}

#[derive(Debug)]
struct RuleArgumentSite {
    line: usize,
    end_column: usize,
    literal_span: SourceSpan,
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
                .chars()
                .skip(self.end_column)
                .take(marker_column - self.end_column)
                .collect::<String>()
                .trim()
                == ","
    }
}

#[derive(Debug)]
struct FixtureItemSite {
    start_line: usize,
    end_line: usize,
    literal_span: SourceSpan,
}

#[derive(Debug, Clone, Copy)]
struct SourceSpan {
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
}

impl SourceSpan {
    fn from_span(span: proc_macro2::Span) -> Self {
        let start = span.start();
        let end = span.end();
        Self {
            start_line: start.line,
            start_column: start.column,
            end_line: end.line,
            end_column: end.column,
        }
    }

    fn contains(self, line: usize, start: usize, end: usize) -> bool {
        if line < self.start_line || line > self.end_line {
            return false;
        }
        let allowed_start = if line == self.start_line { self.start_column } else { 0 };
        let allowed_end = if line == self.end_line { self.end_column } else { usize::MAX };
        allowed_start <= start && end <= allowed_end
    }
}

impl<'ast> syn::visit::Visit<'ast> for RustExemptionSites {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if self.allow_rule_arguments {
            self.collect_rule_argument(call);
        }
        syn::visit::visit_expr_call(self, call);
    }
}

impl RustExemptionSites {
    fn for_file(file: &str, syntax: &syn::File) -> anyhow::Result<Self> {
        let allow_rule_arguments = file == "crates/deckmaste_catalogs/src/cr.rs";
        if allow_rule_arguments {
            ensure_parser_helpers_are_unshadowed(file, syntax)?;
        }
        let allow_fixture = matches!(
            file,
            "crates/deckmaste_catalogs/src/lib.rs" | "crates/deckmaste_catalogs/src/legacy.rs"
        );
        let fixtures =
            if allow_fixture { vec![exact_cr_fixture(file, syntax)?] } else { Vec::new() };
        Ok(Self {
            allow_rule_arguments,
            rule_arguments: Vec::new(),
            fixtures,
        })
    }

    fn collect_rule_argument(&mut self, call: &syn::ExprCall) {
        let syn::Expr::Path(function) = call.func.as_ref() else {
            return;
        };
        if function.qself.is_some() || function.path.segments.len() != 1 {
            return;
        }
        let helper = &function.path.segments[0].ident;
        if !matches!(
            helper.to_string().as_str(),
            "parse_list_rule" | "parse_subtype_rule" | "numbered_rule"
        ) {
            return;
        }
        let Some(syn::Expr::Lit(expression)) = call.args.iter().nth(1) else {
            return;
        };
        let syn::Lit::Str(literal) = &expression.lit else {
            return;
        };
        if !is_rule_parser_key(&literal.value()) {
            return;
        }
        let span = literal.span();
        let start = span.start();
        let end = span.end();
        if start.line == end.line {
            self.rule_arguments.push(RuleArgumentSite {
                line: end.line,
                end_column: end.column,
                literal_span: SourceSpan::from_span(span),
            });
        }
    }
}

fn ensure_parser_helpers_are_unshadowed(file: &str, syntax: &syn::File) -> anyhow::Result<()> {
    let mut shadows = ParserHelperShadows::default();
    shadows.visit_file(syntax);
    anyhow::ensure!(
        shadows.names.is_empty(),
        "cite: parser helper is shadowed in {file}: {}",
        shadows.names.into_iter().collect::<Vec<_>>().join(", ")
    );
    Ok(())
}

#[derive(Default)]
struct ParserHelperShadows {
    function_depth: usize,
    module_depth: usize,
    names: BTreeSet<String>,
}

impl<'ast> syn::visit::Visit<'ast> for ParserHelperShadows {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        if (self.function_depth > 0 || self.module_depth > 0)
            && is_parser_helper_name(&item.sig.ident)
        {
            self.names.insert(item.sig.ident.to_string());
        }
        self.function_depth += 1;
        syn::visit::visit_item_fn(self, item);
        self.function_depth -= 1;
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        self.module_depth += 1;
        syn::visit::visit_item_mod(self, item);
        self.module_depth -= 1;
    }

    fn visit_pat_ident(&mut self, pattern: &'ast syn::PatIdent) {
        if is_parser_helper_name(&pattern.ident) {
            self.names.insert(pattern.ident.to_string());
        }
        syn::visit::visit_pat_ident(self, pattern);
    }

    fn visit_item_const(&mut self, item: &'ast syn::ItemConst) {
        if is_parser_helper_name(&item.ident) {
            self.names.insert(item.ident.to_string());
        }
        syn::visit::visit_item_const(self, item);
    }

    fn visit_item_static(&mut self, item: &'ast syn::ItemStatic) {
        if is_parser_helper_name(&item.ident) {
            self.names.insert(item.ident.to_string());
        }
        syn::visit::visit_item_static(self, item);
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        collect_imported_parser_helpers(&item.tree, &mut self.names);
        if (self.function_depth > 0 || self.module_depth > 0) && use_tree_contains_glob(&item.tree)
        {
            self.names.insert("non-root glob import".to_string());
        }
        syn::visit::visit_item_use(self, item);
    }
}

fn is_parser_helper_name(ident: &syn::Ident) -> bool {
    matches!(
        ident.to_string().as_str(),
        "parse_list_rule" | "parse_subtype_rule" | "numbered_rule"
    )
}

fn collect_imported_parser_helpers(tree: &syn::UseTree, names: &mut BTreeSet<String>) {
    match tree {
        syn::UseTree::Name(name) if is_parser_helper_name(&name.ident) => {
            names.insert(name.ident.to_string());
        }
        syn::UseTree::Rename(rename) if is_parser_helper_name(&rename.rename) => {
            names.insert(rename.rename.to_string());
        }
        syn::UseTree::Path(path) => collect_imported_parser_helpers(&path.tree, names),
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_imported_parser_helpers(item, names);
            }
        }
        syn::UseTree::Name(_) | syn::UseTree::Rename(_) | syn::UseTree::Glob(_) => {}
    }
}

fn use_tree_contains_glob(tree: &syn::UseTree) -> bool {
    match tree {
        syn::UseTree::Glob(_) => true,
        syn::UseTree::Path(path) => use_tree_contains_glob(&path.tree),
        syn::UseTree::Group(group) => group.items.iter().any(use_tree_contains_glob),
        syn::UseTree::Name(_) | syn::UseTree::Rename(_) => false,
    }
}

fn exact_cr_fixture(file: &str, syntax: &syn::File) -> anyhow::Result<FixtureItemSite> {
    let mut fixtures = CrFixtureCollector::default();
    fixtures.visit_file(syntax);
    anyhow::ensure!(
        fixtures.occurrences.len() == 1,
        "cite: expected exactly one direct tests::CR_FIXTURE in {file}, found {}",
        fixtures.occurrences.len()
    );
    let occurrence = fixtures.occurrences.pop().expect("length checked");
    anyhow::ensure!(
        occurrence.module_path == ["tests"] && occurrence.block_depth == 0,
        "cite: CR_FIXTURE must be a direct item of tests in {file}"
    );
    occurrence
        .site
        .with_context(|| format!("cite: tests::CR_FIXTURE has the wrong shape in {file}"))
}

#[derive(Default)]
struct CrFixtureCollector {
    module_path: Vec<String>,
    block_depth: usize,
    occurrences: Vec<CrFixtureOccurrence>,
}

struct CrFixtureOccurrence {
    module_path: Vec<String>,
    block_depth: usize,
    site: Option<FixtureItemSite>,
}

impl<'ast> syn::visit::Visit<'ast> for CrFixtureCollector {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        self.module_path.push(item.ident.to_string());
        syn::visit::visit_item_mod(self, item);
        self.module_path.pop();
    }

    fn visit_block(&mut self, block: &'ast syn::Block) {
        self.block_depth += 1;
        syn::visit::visit_block(self, block);
        self.block_depth -= 1;
    }

    fn visit_item_const(&mut self, item: &'ast syn::ItemConst) {
        if item.ident == "CR_FIXTURE" {
            self.occurrences.push(CrFixtureOccurrence {
                module_path: self.module_path.clone(),
                block_depth: self.block_depth,
                site: cr_fixture_literal(item).map(|literal| FixtureItemSite {
                    start_line: item.const_token.span.start().line,
                    end_line: item.semi_token.span.end().line,
                    literal_span: SourceSpan::from_span(literal.span()),
                }),
            });
        }
        syn::visit::visit_item_const(self, item);
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

fn cr_fixture_literal(item: &syn::ItemConst) -> Option<&syn::LitStr> {
    let syn::Type::Reference(reference) = item.ty.as_ref() else {
        return None;
    };
    let syn::Type::Path(path) = reference.elem.as_ref() else {
        return None;
    };
    let syn::Expr::Lit(expression) = item.expr.as_ref() else {
        return None;
    };
    (item.ident == "CR_FIXTURE"
        && reference.mutability.is_none()
        && path.qself.is_none()
        && path.path.is_ident("str")
        && matches!(expression.lit, syn::Lit::Str(_)))
    .then(|| {
        let syn::Lit::Str(literal) = &expression.lit else {
            unreachable!("shape checked")
        };
        literal
    })
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
            // A unified diff separates the path from an optional timestamp with a
            // TAB, so only a TAB ends the path: a plain space is part of it
            // (`plugins_v2/canon/cards/Luminarch Aspirant.ron`).
            let path = path.split('\t').next().unwrap_or(path);
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
        let line_source = "\
// CR 100.1
fn example() {
    parse_list_rule(
        &lines,
        \"100.1\", // cite: noncompliant-line -- machine-readable parser key
        \"The example values are \",
        \"examples\",
    );
    parse_subtype_rule(
        &lines,
        \"200.1\", // cite: noncompliant-line -- machine-readable parser key
        \"example\",
        \"examples\",
    );
    numbered_rule(
        &lines,
        \"300.1\", // cite: noncompliant-line -- machine-readable parser key
        \"examples\",
    );
}
// rule 400.1
";
        let scope_source = "\
mod tests {
// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims
const CR_FIXTURE: &str = \"rule 200.1\";
// cite: noncompliant end
}
";
        let hits =
            noncompliant_source_hits("crates/deckmaste_catalogs/src/cr.rs", line_source).unwrap();
        assert_eq!(
            hits.iter()
                .map(|hit| hit.matched.as_str())
                .collect::<Vec<_>>(),
            ["CR 100.1", "100.1", "rule 400.1", "400.1"]
        );
        assert!(
            noncompliant_source_hits("crates/deckmaste_catalogs/src/lib.rs", scope_source,)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn noncompliant_scan_rejects_rule_key_in_unrelated_free_call() {
        let source = "fn f() { unrelated(&lines, \"100.1\", // cite: noncompliant-line -- machine-readable parser key\n); }";
        assert!(noncompliant_source_hits("crates/deckmaste_catalogs/src/cr.rs", source).is_err());
    }

    #[test]
    fn noncompliant_scan_rejects_rule_key_in_method_call() {
        let source = "fn f() { parser.numbered_rule(&lines, \"100.1\", // cite: noncompliant-line -- machine-readable parser key\n); }";
        assert!(noncompliant_source_hits("crates/deckmaste_catalogs/src/cr.rs", source).is_err());
    }

    #[test]
    fn noncompliant_scan_rejects_rule_key_in_wrong_helper_argument() {
        let source = "fn f() { parse_list_rule(\"100.1\", // cite: noncompliant-line -- machine-readable parser key\n&lines); }";
        assert!(noncompliant_source_hits("crates/deckmaste_catalogs/src/cr.rs", source).is_err());
    }

    #[test]
    fn noncompliant_scan_rejects_cr_helper_in_wrong_file() {
        let source = "fn f() { numbered_rule(&lines, \"100.1\", // cite: noncompliant-line -- machine-readable parser key\n\"examples\"); }";
        assert!(noncompliant_source_hits("crates/deckmaste_catalogs/src/lib.rs", source).is_err());
    }

    #[test]
    fn noncompliant_scan_reports_loose_claims_beside_exempt_literals() {
        let parser_source = "fn f() { let _ = \"[CR#100.1|é]\"; /* rule 900.1 */ parse_list_rule(&lines, \"100.1\", // cite: noncompliant-line -- machine-readable parser key\n\"lead\", \"catalog\"); }";
        let parser_hits =
            noncompliant_source_hits("crates/deckmaste_catalogs/src/cr.rs", parser_source).unwrap();
        assert_eq!(
            parser_hits
                .iter()
                .map(|hit| hit.matched.as_str())
                .collect::<Vec<_>>(),
            ["rule 900.1", "900.1"]
        );

        let fixture_source = "mod tests {\n\
// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
const CR_FIXTURE: &str = \"rule 200.1\"; // rule 900.1\n\
// cite: noncompliant end\n\
}";
        let fixture_hits =
            noncompliant_source_hits("crates/deckmaste_catalogs/src/lib.rs", fixture_source)
                .unwrap();
        assert_eq!(
            fixture_hits
                .iter()
                .map(|hit| hit.matched.as_str())
                .collect::<Vec<_>>(),
            ["rule 900.1", "900.1"]
        );
    }

    #[test]
    fn noncompliant_scan_rejects_a_shadowed_parser_helper() {
        let source = "fn f() {\n\
fn numbered_rule(_: (), _: &str, _: &str) {}\n\
numbered_rule((), \"100.1\", // cite: noncompliant-line -- machine-readable parser key\n\
\"catalog\");\n\
}";

        assert!(noncompliant_source_hits("crates/deckmaste_catalogs/src/cr.rs", source).is_err());

        let imported_source = "mod other { fn numbered_rule(_: (), _: &str, _: &str) {} }\n\
fn f() {\n\
use other::numbered_rule;\n\
numbered_rule((), \"100.1\", // cite: noncompliant-line -- machine-readable parser key\n\
\"catalog\");\n\
}";

        assert!(
            noncompliant_source_hits("crates/deckmaste_catalogs/src/cr.rs", imported_source)
                .is_err()
        );
    }

    #[test]
    fn noncompliant_scan_rejects_parser_helper_shadows_in_nested_modules() {
        let root_helpers = "fn parse_list_rule() {}\n\
fn parse_subtype_rule() {}\n\
fn numbered_rule() {}\n";
        let nested_definition = format!(
            "{root_helpers}\
mod nested {{\n\
fn numbered_rule(_: (), _: &str, _: &str) {{}}\n\
fn f() {{\n\
numbered_rule((), \"100.1\", // cite: noncompliant-line -- machine-readable parser key\n\
\"catalog\");\n\
}}\n\
}}"
        );
        let nested_glob_import = format!(
            "{root_helpers}\
mod nested {{\n\
use super::*;\n\
fn f() {{\n\
numbered_rule((), \"100.1\", // cite: noncompliant-line -- machine-readable parser key\n\
\"catalog\");\n\
}}\n\
}}"
        );

        for source in [nested_definition, nested_glob_import] {
            assert!(
                noncompliant_source_hits("crates/deckmaste_catalogs/src/cr.rs", &source).is_err()
            );
        }
    }

    #[test]
    fn noncompliant_scan_rejects_license_fixture_scope() {
        let source = "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
const LICENSE_FIXTURE: &str = \"rule 100.1\";\n\
// cite: noncompliant end\n";
        assert!(noncompliant_source_hits("crates/deckmaste_catalogs/src/lib.rs", source).is_err());
    }

    #[test]
    fn noncompliant_scan_rejects_other_named_fixture_scope() {
        let source = "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
const PARSER_FIXTURE: &str = \"rule 100.1\";\n\
// cite: noncompliant end\n";
        assert!(
            noncompliant_source_hits("crates/deckmaste_catalogs/src/legacy.rs", source).is_err()
        );
    }

    #[test]
    fn noncompliant_scan_rejects_cr_fixture_scope_in_wrong_file() {
        let source = "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
const CR_FIXTURE: &str = \"rule 100.1\";\n\
// cite: noncompliant end\n";
        assert!(noncompliant_source_hits("crates/deckmaste_catalogs/src/io.rs", source).is_err());
    }

    #[test]
    fn noncompliant_scan_rejects_fixture_outside_the_direct_tests_module() {
        let source = "// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
const CR_FIXTURE: &str = \"rule 100.1\";\n\
// cite: noncompliant end\n";

        assert!(noncompliant_source_hits("crates/deckmaste_catalogs/src/lib.rs", source).is_err());
    }

    #[test]
    fn noncompliant_scan_rejects_second_or_nested_cr_fixtures() {
        for extra in [
            "const CR_FIXTURE: &str = \"rule 300.1\";",
            "mod nested { const CR_FIXTURE: &str = \"rule 300.1\"; }",
        ] {
            let source = format!(
                "mod tests {{\n\
// cite: noncompliant begin -- verbatim CR parser fixture, not prose claims\n\
const CR_FIXTURE: &str = \"rule 200.1\";\n\
// cite: noncompliant end\n\
{extra}\n\
}}"
            );

            assert!(
                noncompliant_source_hits("crates/deckmaste_catalogs/src/lib.rs", &source).is_err(),
                "{extra}"
            );
        }
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

    /// A card file's name carries spaces, and the citation audit must still
    /// reach the lines it added.
    #[test]
    fn added_diff_lines_keeps_a_path_containing_spaces() {
        let diff = concat!(
            "diff --git a/cards/Luminarch Aspirant.ron b/cards/Luminarch Aspirant.ron\n",
            "--- /dev/null\n",
            "+++ b/cards/Luminarch Aspirant.ron\n",
            "@@ -0,0 +1,1 @@\n",
            "+// [CR#506.1]\n",
        );
        assert_eq!(
            added_diff_lines(diff),
            BTreeSet::from([("cards/Luminarch Aspirant.ron".into(), 1)])
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
