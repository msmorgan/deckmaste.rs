//! `cargo xtask map` — on-demand "bearings" dumps of current code shape, so
//! an agent can regenerate what a corpus of `pub enum`s or Idris `data` types
//! currently looks like instead of re-reading the source wholesale.
//!
//! Two subcommands:
//!  - `map enums [DIR]` — every `pub enum` under a Rust source tree (default
//!    `crates/deckmaste_core/src`), with each variant's shape and first doc
//!    line.
//!  - `map idris [FILE]` — every `data` declaration in an Idris source file
//!    (default `idris/src/Core.idr`), ADT or GADT style, with its constructors.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use clap::Args;
use clap::Subcommand;
use quote::ToTokens;
use syn::spanned::Spanned;

#[derive(Debug, Args)]
pub struct MapArgs {
    #[command(subcommand)]
    command: MapCmd,
}

#[derive(Debug, Subcommand)]
enum MapCmd {
    /// Dump every `pub enum` under a Rust source tree.
    Enums(EnumsArgs),
    /// Dump every `data` declaration in an Idris source file.
    Idris(IdrisArgs),
}

#[derive(Debug, Args)]
pub struct EnumsArgs {
    /// Directory to walk recursively for `.rs` files. Defaults to
    /// `crates/deckmaste_core/src`.
    dir: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct IdrisArgs {
    /// Idris source file to scan. Defaults to `idris/src/Core.idr`.
    file: Option<PathBuf>,
}

/// # Errors
/// If the target directory/file is missing, unreadable, or (for `enums`) a
/// `.rs` file fails to parse.
pub fn run(args: &MapArgs) -> anyhow::Result<()> {
    match &args.command {
        MapCmd::Enums(a) => run_enums(a),
        MapCmd::Idris(a) => run_idris(a),
    }
}

// ---------------------------------------------------------------------
// `map enums`
// ---------------------------------------------------------------------

/// # Errors
/// If DIR isn't a directory, a `.rs` file can't be read, or fails to parse.
fn run_enums(args: &EnumsArgs) -> anyhow::Result<()> {
    let dir = args.dir.clone().unwrap_or_else(|| {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/deckmaste_core/src")
    });
    anyhow::ensure!(dir.is_dir(), "not a directory: {}", dir.display());

    let mut rs_files = Vec::new();
    collect_rs_files(&dir, &mut rs_files).with_context(|| format!("walking {}", dir.display()))?;
    rs_files.sort();

    for path in &rs_files {
        let src =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let file = syn::parse_file(&src).with_context(|| format!("parsing {}", path.display()))?;
        let mut enums = Vec::new();
        collect_pub_enums(&file.items, &mut enums);
        for item_enum in enums {
            print_enum(path, item_enum);
        }
    }

    Ok(())
}

/// Recursively walk `dir` collecting `.rs` file paths.
fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("reading dir {}", dir.display()))? {
        let entry = entry.with_context(|| format!("reading an entry of {}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out)?;
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
    Ok(())
}

/// Recursively walk a file's items (descending into inline `mod { .. }`
/// blocks) collecting `pub enum` items, in source order.
fn collect_pub_enums<'a>(items: &'a [syn::Item], out: &mut Vec<&'a syn::ItemEnum>) {
    for item in items {
        match item {
            syn::Item::Enum(item_enum) if matches!(item_enum.vis, syn::Visibility::Public(_)) => {
                out.push(item_enum);
            }
            syn::Item::Mod(item_mod) => {
                if let Some((_, inner_items)) = &item_mod.content {
                    collect_pub_enums(inner_items, out);
                }
            }
            _ => {}
        }
    }
}

fn print_enum(path: &Path, item: &syn::ItemEnum) {
    let line = item.span().start().line;
    println!(
        "## {} — {}:{line} ({} variants)",
        item.ident,
        path.display(),
        item.variants.len()
    );
    if let Some(doc) = first_doc_line(&item.attrs) {
        println!("{doc}");
    }
    for variant in &item.variants {
        let name = &variant.ident;
        let shape = variant_shape(&variant.fields);
        match first_doc_line(&variant.attrs) {
            Some(doc) => println!("- {name}{shape} — {doc}"),
            None => println!("- {name}{shape}"),
        }
    }
    println!();
}

/// `(<field types>)` for a tuple variant, `{ <name>: <type>, … }` for a
/// struct variant, empty for a unit variant.
fn variant_shape(fields: &syn::Fields) -> String {
    match fields {
        syn::Fields::Unit => String::new(),
        syn::Fields::Unnamed(unnamed) => {
            let types: Vec<String> = unnamed
                .unnamed
                .iter()
                .map(|field| field.ty.to_token_stream().to_string())
                .collect();
            format!("({})", types.join(", "))
        }
        syn::Fields::Named(named) => {
            let fields: Vec<String> = named
                .named
                .iter()
                .map(|field| {
                    // Named fields always carry an ident; `unwrap_or` covers
                    // the syntactically-unreachable case rather than panic.
                    let name = field
                        .ident
                        .as_ref()
                        .map_or_else(|| "_".to_string(), std::string::ToString::to_string);
                    let ty = field.ty.to_token_stream().to_string();
                    format!("{name}: {ty}")
                })
                .collect();
            format!("{{ {} }}", fields.join(", "))
        }
    }
}

/// The first non-blank `///` doc-comment line attached to `attrs`, if any.
fn first_doc_line(attrs: &[syn::Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("doc") {
            return None;
        }
        let syn::Meta::NameValue(name_value) = &attr.meta else {
            return None;
        };
        let syn::Expr::Lit(expr_lit) = &name_value.value else {
            return None;
        };
        let syn::Lit::Str(lit_str) = &expr_lit.lit else {
            return None;
        };
        let text = lit_str.value();
        let trimmed = text.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

// ---------------------------------------------------------------------
// `map idris`
// ---------------------------------------------------------------------

/// One `data <Name>` declaration found in an Idris source file.
struct DataDecl {
    name: String,
    /// 1-based source line of the `data` keyword.
    line: usize,
    /// Constructor name + 1-based source line, in declaration order.
    ctors: Vec<(String, usize)>,
}

/// # Errors
/// If FILE can't be read.
fn run_idris(args: &IdrisArgs) -> anyhow::Result<()> {
    let file = args
        .file
        .clone()
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../idris/src/Core.idr"));
    let src = fs::read_to_string(&file).with_context(|| format!("reading {}", file.display()))?;

    for decl in scan_idris_data(&src) {
        println!("## {} — {}", decl.name, decl.line);
        for (ctor_name, ctor_line) in &decl.ctors {
            println!("- {ctor_name} — {ctor_line}");
        }
        println!();
    }

    Ok(())
}

/// Whether a declaration's constructors are listed `data X = A | B | …`
/// (ADT) or `data X : … where` followed by indented `Ctor : …` lines (GADT).
enum Style {
    Adt,
    Gadt,
}

/// Line-scan `src` for `data <Name>` declarations (both ADT and GADT style)
/// and their constructors. Not a real Idris parser — a heuristic scanner
/// tuned to this file's layout (see module doc comment).
fn scan_idris_data(src: &str) -> Vec<DataDecl> {
    let lines: Vec<&str> = src.lines().collect();
    let mut decls = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let Some((indent, name)) = data_decl_start(line) else {
            continue;
        };
        let (style, header_idx) = find_header(&lines, i);
        let ctors = match style {
            Style::Gadt => gadt_ctors(&lines, header_idx, indent),
            Style::Adt => adt_ctors(&lines, header_idx, indent),
        };
        decls.push(DataDecl {
            name,
            line: i + 1,
            ctors,
        });
    }

    decls
}

/// If `line` opens a `data <Name>` declaration, its indentation (leading
/// column count) and the declared name.
fn data_decl_start(line: &str) -> Option<(usize, String)> {
    let indent = line.len() - line.trim_start().len();
    let rest = line.trim_start().strip_prefix("data ")?;
    let name = ident_at(rest)?;
    Some((indent, name))
}

/// Scan forward from `start` (inclusive) for the line that either contains
/// `=` (ADT header) or ends with `where` (GADT header), skipping over the
/// `data <Name>` line itself when its header continues on later lines (e.g.
/// `data BeginningStep\n    = UntapStep\n    | …`).
fn find_header(lines: &[&str], start: usize) -> (Style, usize) {
    let mut j = start;
    loop {
        let content = strip_comment(lines[j]).trim();
        if content.ends_with("where") {
            return (Style::Gadt, j);
        }
        if content.contains('=') {
            return (Style::Adt, j);
        }
        if j + 1 >= lines.len() {
            // Malformed/truncated input; treat the decl line itself as an
            // (empty) ADT header rather than looping forever.
            return (Style::Adt, start);
        }
        j += 1;
    }
}

/// Collect `Ctor : …` lines following a GADT `where` header, stopping at the
/// first non-blank line dedented to (or past) the declaration's own
/// indentation.
fn gadt_ctors(lines: &[&str], header_idx: usize, data_indent: usize) -> Vec<(String, usize)> {
    let mut ctors = Vec::new();
    let mut ctor_indent = None;

    for (offset, line) in lines[header_idx + 1..].iter().enumerate() {
        let j = header_idx + 1 + offset;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let cur_indent = line.len() - line.trim_start().len();
        if cur_indent <= data_indent {
            break;
        }
        if trimmed.starts_with("--") || trimmed.starts_with("|||") {
            continue;
        }
        match ctor_indent {
            None => {
                if let Some(name) = ctor_name_at(trimmed) {
                    ctor_indent = Some(cur_indent);
                    ctors.push((name, j + 1));
                }
            }
            Some(ci) if cur_indent == ci => {
                if let Some(name) = ctor_name_at(trimmed) {
                    ctors.push((name, j + 1));
                }
            }
            Some(_) => {
                // Deeper (wrapped continuation of a ctor's type) or
                // shallower-but-still-nested line; neither starts a ctor.
            }
        }
    }

    ctors
}

/// A GADT constructor line is `<Ident> : <type>` or, for an
/// operator constructor (e.g. `SeqList`'s `(::)`), `(<op>) : <type>`;
/// extract the leading name.
fn ctor_name_at(trimmed: &str) -> Option<String> {
    let stripped = strip_comment(trimmed);
    let name = ctor_token_at(stripped)?;
    stripped[name.len()..]
        .trim_start()
        .starts_with(':')
        .then_some(name)
}

/// The leading constructor token at the start of `s`: either a plain ident
/// or a parenthesized operator like `(::)`.
fn ctor_token_at(s: &str) -> Option<String> {
    if let Some(rest) = s.strip_prefix('(') {
        let end = rest.find(')')?;
        return Some(s[..=end + 1].to_string());
    }
    ident_at(s)
}

/// Collect constructors of an ADT `data X = A | B | …` declaration: the
/// `|`-separated names after `=` on the header line itself, plus any
/// continuation lines that lead with `|` and stay nested under the decl.
fn adt_ctors(lines: &[&str], header_idx: usize, data_indent: usize) -> Vec<(String, usize)> {
    let mut ctors = Vec::new();

    let header = strip_comment(lines[header_idx]);
    if let Some(eq_pos) = header.find('=') {
        for seg in header[eq_pos + 1..].split('|') {
            if let Some(name) = ident_at(seg.trim_start()) {
                ctors.push((name, header_idx + 1));
            }
        }
    }

    for (offset, line) in lines[header_idx + 1..].iter().enumerate() {
        let j = header_idx + 1 + offset;
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let cur_indent = line.len() - trimmed.len();
        if !trimmed.starts_with('|') || cur_indent <= data_indent {
            break;
        }
        let content = strip_comment(trimmed).trim_start_matches('|');
        for seg in content.split('|') {
            if let Some(name) = ident_at(seg.trim_start()) {
                ctors.push((name, j + 1));
            }
        }
    }

    ctors
}

/// The leading Idris identifier (letters/digits/`_`/`'`, starting with a
/// letter) at the start of `s`, if any.
fn ident_at(s: &str) -> Option<String> {
    let mut chars = s.chars();
    let first = chars.next()?;
    if !first.is_alphabetic() {
        return None;
    }
    let rest_len: usize = s
        .chars()
        .skip(1)
        .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '\'')
        .map(char::len_utf8)
        .sum();
    Some(s[..first.len_utf8() + rest_len].to_string())
}

/// Strip a trailing Idris `--` line comment (does not attempt to respect
/// string literals — none of this file's `data` headers/ctors contain one).
fn strip_comment(line: &str) -> &str {
    line.find("--").map_or(line, |pos| &line[..pos])
}
