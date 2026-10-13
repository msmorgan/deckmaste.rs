//! Restricted nominal modifier-order equivalence. This does not decide admission.
//!
//! The generated Reading Debug format is already the census's tree identity.
//! Parse it losslessly into labelled records: strings, field order, constructors,
//! forms and every lexical payload stay exact. Unknown records remain exact;
//! malformed syntax or changed shapes of recognized wrappers are errors.
use anyhow::{Context, Result, bail, ensure};
use deckmaste_english_v3::grammar::Reading;
use serde::Serialize;

// Bump on every classification or normalization change and append the new
// source identity to the version lock in the tests. Historical entries stay fixed.
pub(super) const VERSION: usize = 3;

pub(super) fn source_sha256() -> String {
    // Compile the source into the binary: a disk edit after building must not
    // misidentify the matcher that actually produced a census.
    crate::raw_corpus::digest(include_str!("nominal_normal_form.rs").as_bytes())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum Tree {
    Atom(String),
    Call(String, Vec<Self>),
    Record(String, Vec<(String, Self)>),
    List(Vec<Self>),
    Nominal {
        base: Box<Self>,
        pre: Vec<Self>,
        post: Vec<Self>,
    },
}

struct Parser<'a> {
    input: &'a str,
}
impl Parser<'_> {
    fn space(&mut self) {
        self.input = self.input.trim_start();
    }
    fn take(&mut self, text: &str) -> bool {
        self.space();
        if let Some(rest) = self.input.strip_prefix(text) {
            self.input = rest;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, text: &str) -> Result<()> {
        ensure!(self.take(text), "Reading encoding: expected {text}");
        Ok(())
    }
    fn atom(&mut self) -> Result<String> {
        self.space();
        let mut end = 0;
        if self.input.starts_with('"') {
            let mut escaped = false;
            for (i, c) in self.input.char_indices().skip(1) {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    end = i + 1;
                    break;
                }
            }
            ensure!(end != 0, "unterminated Reading string");
        } else {
            end = self
                .input
                .find(|c: char| c.is_whitespace() || "{}()[],:".contains(c))
                .unwrap_or(self.input.len());
            ensure!(
                end != 0,
                "missing Reading atom near {:?}",
                self.input.chars().take(40).collect::<String>()
            );
        }
        let result = self.input[..end].to_owned();
        self.input = &self.input[end..];
        Ok(result)
    }
    fn values(&mut self, close: &str) -> Result<Vec<Tree>> {
        let mut values = Vec::new();
        while !self.take(close) {
            values.push(self.tree()?);
            if !self.take(",") {
                self.expect(close)?;
                break;
            }
        }
        Ok(values)
    }
    fn tree(&mut self) -> Result<Tree> {
        if self.take("[") {
            return Ok(Tree::List(self.values("]")?));
        }
        let name = self.atom()?;
        if self.take("(") {
            return Ok(Tree::Call(name, self.values(")")?));
        }
        if self.take("{") {
            let mut fields = Vec::new();
            while !self.take("}") {
                let field = self.atom()?;
                self.expect(":")?;
                ensure!(
                    !fields.iter().any(|(n, _)| n == &field),
                    "duplicate Reading field {field}"
                );
                fields.push((field, self.tree()?));
                if !self.take(",") {
                    self.expect("}")?;
                    break;
                }
            }
            return Ok(Tree::Record(name, fields));
        }
        Ok(Tree::Atom(name))
    }
}

impl Tree {
    fn field(&self, field: &str) -> Option<&Self> {
        let Self::Record(_, fields) = self else {
            return None;
        };
        fields.iter().find(|(n, _)| n == field).map(|(_, v)| v)
    }
    fn word_id(&self) -> Option<&str> {
        let Self::Record(name, _) = self else {
            return None;
        };
        if name != "Word" {
            return None;
        }
        let Self::Call(word, values) = self.field("value")? else {
            return None;
        };
        if word != "Word" || values.len() != 1 {
            return None;
        }
        let Self::Call(ident, values) = values[0].field("lexeme")? else {
            return None;
        };
        if ident != "Ident" || values.len() != 1 {
            return None;
        }
        let Self::Atom(id) = &values[0] else {
            return None;
        };
        id.strip_prefix('"')?.strip_suffix('"')
    }
    fn commuting_pre(&self, noun: bool) -> bool {
        let Self::Record(name, fields) = self else {
            return false;
        };
        if !fields.iter().map(|(n, _)| n.as_str()).eq(["form", "head"])
            || self.field("form") != Some(&Self::Atom("0".into()))
        {
            return false;
        }
        if name != if noun { "NounPremodifier" } else { "Adjective" } {
            return false;
        }
        self.field("head")
            .and_then(Self::word_id)
            .is_some_and(|id| {
                // The reviewed list lives here. Vocabulary groups are lexical
                // inventories, not surface-string matches. Do not include /non,
                // subtype inventories, or other participial adjectives implicitly.
                if noun {
                    return id
                        .strip_prefix("lexeme:type/")
                        .is_some_and(|name| !name.is_empty() && !name.contains('/'));
                }
                matches!(
                    id,
                    "vocab:ColorWord/White"
                        | "vocab:ColorWord/Blue"
                        | "vocab:ColorWord/Black"
                        | "vocab:ColorWord/Red"
                        | "vocab:ColorWord/Green"
                        | "vocab:ColorWord/Colorless"
                        | "vocab:ColorWord/Multicolored"
                        | "vocab:ColorWord/Monocolored"
                        | "vocab:PredicativeAdjective/Legendary/non"
                        | "lexeme:keyword_action/tap/adjective"
                )
            })
    }
    fn side(&self) -> Result<Option<bool>> {
        let Self::Record(name, fields) = self else {
            return Ok(None);
        };
        let (pre, dependent) = match name.as_str() {
            "PremodifiedNominal" | "NounPremodifiedNominal" => (true, "modifier"),
            "ObjectRelativeNominal" | "SubjectRelativeNominal" => (false, "relative"),
            "PostmodifiedNominal" | "ParticipialPostmodifiedNominal" | "NamedNominal" => {
                (false, "modifier")
            }
            _ => return Ok(None),
        };
        let expected = if pre { ["form", dependent, "head"] } else { ["form", "head", dependent] };
        ensure!(
            fields.iter().map(|(n, _)| n.as_str()).eq(expected),
            "normal-form adapter needs review: {name} fields changed"
        );
        ensure!(
            self.field("form") == Some(&Self::Atom("0".into())),
            "normal-form adapter needs review: {name} form changed"
        );
        Ok((!pre
            || self
                .field(dependent)
                .is_some_and(|m| m.commuting_pre(name == "NounPremodifiedNominal")))
        .then_some(pre))
    }
    // Invert only the reviewed layer representation before comparing analyses.
    // This pass precedes chain flattening so a lift cannot interrupt a chain.
    fn adapt_layers(self) -> Result<Self> {
        Ok(match self {
            Self::Record(mut name, mut fields) => {
                let expected: Option<&[&str]> = match name.as_str() {
                    "NominalCoreProjection" | "NominalProjection" => Some(&["form", "head"]),
                    "BarrierPremodifiedNominal" | "BarrierNounPremodifiedNominal" => {
                        Some(&["form", "modifier", "head"])
                    }
                    _ => None,
                };
                if let Some(expected) = expected {
                    ensure!(
                        fields
                            .iter()
                            .map(|(n, _)| n.as_str())
                            .eq(expected.iter().copied())
                            && fields[0].1 == Self::Atom("0".into()),
                        "normal-form adapter needs review: {name} shape changed"
                    );
                    if name.ends_with("Projection") {
                        return fields.pop().unwrap().1.adapt_layers();
                    }
                    name.drain(.."Barrier".len());
                }
                let category_shape: Option<&[&str]> = match name.as_str() {
                    "Coordination" => Some(&["form", "category", "left", "coordinator", "right"]),
                    "SerialCoordination" => Some(&["form", "category", "left", "rest"]),
                    "DiscontinuousComparativeNominal" => Some(&[
                        "form",
                        "category",
                        "governor",
                        "head",
                        "marker",
                        "complement",
                    ]),
                    "PostpositiveNominal" => Some(&["form", "category", "head", "modifier"]),
                    _ => None,
                };
                if let Some(expected) = category_shape
                    && fields
                        .iter()
                        .any(|(n, v)| n == "category" && *v == Self::Atom("NominalCore".into()))
                {
                    ensure!(
                        fields
                            .iter()
                            .map(|(n, _)| n.as_str())
                            .eq(expected.iter().copied())
                            && fields[0].1 == Self::Atom("0".into()),
                        "normal-form adapter needs review: {name} shape changed"
                    );
                    fields[1].1 = Self::Atom("Nominal".into());
                }
                Self::Record(
                    name,
                    fields
                        .into_iter()
                        .map(|(n, v)| Ok((n, v.adapt_layers()?)))
                        .collect::<Result<_>>()?,
                )
            }
            Self::Call(name, values) => Self::Call(
                name,
                values
                    .into_iter()
                    .map(Self::adapt_layers)
                    .collect::<Result<_>>()?,
            ),
            Self::List(values) => Self::List(
                values
                    .into_iter()
                    .map(Self::adapt_layers)
                    .collect::<Result<_>>()?,
            ),
            Self::Atom(_) => self,
            Self::Nominal { .. } => bail!("normal form cannot be adapted twice"),
        })
    }
    fn normalize(self) -> Result<Self> {
        if self.side()?.is_some() {
            let mut head = self;
            let mut pre = Vec::new();
            let mut post = Vec::new();
            while let Some(is_pre) = head.side()? {
                let Self::Record(name, fields) = head else { unreachable!() };
                let mut rest = Vec::new();
                let mut next = None;
                for (n, value) in fields {
                    if n == "head" {
                        next = Some(value);
                    } else {
                        rest.push((n, value.normalize()?));
                    }
                }
                let record = Self::Record(name, rest);
                if is_pre {
                    pre.push(record);
                } else {
                    post.push(record);
                }
                head = next.context("nominal head missing")?;
            }
            return Ok(Self::Nominal {
                base: Box::new(head.normalize()?),
                pre,
                post,
            });
        }
        Ok(match self {
            Self::Atom(_) => self,
            Self::Call(name, values) => Self::Call(
                name,
                values
                    .into_iter()
                    .map(Self::normalize)
                    .collect::<Result<_>>()?,
            ),
            Self::List(values) => Self::List(
                values
                    .into_iter()
                    .map(Self::normalize)
                    .collect::<Result<_>>()?,
            ),
            Self::Record(name, fields) => Self::Record(
                name,
                fields
                    .into_iter()
                    .map(|(n, v)| Ok((n, v.normalize()?)))
                    .collect::<Result<_>>()?,
            ),
            Self::Nominal { .. } => bail!("normal form cannot be normalized twice"),
        })
    }
}

fn encoded(debug: &str) -> Result<Vec<u8>> {
    let mut parser = Parser { input: debug };
    let tree = parser.tree()?;
    ensure!(parser.input.trim().is_empty(), "trailing Reading encoding");
    Ok(serde_json::to_vec(&tree.adapt_layers()?.normalize()?)?)
}

pub(super) fn fingerprint(reading: &Reading) -> Result<String> {
    Ok(crate::raw_corpus::digest(&encoded(&format!(
        "{reading:?}"
    ))?))
}

#[cfg(test)]
#[path = "nominal_normal_form_tests.rs"]
mod tests;
