use std::io::Write;

use anyhow::Context;
use deckmaste_english_v2::ast::Ability;
use deckmaste_english_v2::ast::AttachmentSitePath;
use deckmaste_english_v2::ast::AttachmentSiteStep;
use deckmaste_english_v2::ast::OracleText;
use deckmaste_english_v2::ast::Sentence;
use deckmaste_english_v2::visit::Visitor;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub(super) struct PackedSite {
    construction_path: Vec<String>,
    role: String,
    paths: Vec<Vec<PackedSiteStep>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum PackedSiteStep {
    Role { name: String },
    Conjunct { name: String, ordinal: usize },
}

pub(super) fn ability(value: &Ability) -> Vec<PackedSite> {
    let mut collector = PackedSiteCollector::default();
    deckmaste_english_v2::visit::walk_ability(&mut collector, value);
    collector.finish()
}

pub(super) fn sentence(value: &Sentence) -> Vec<PackedSite> {
    let mut collector = PackedSiteCollector::default();
    deckmaste_english_v2::visit::walk_sentence(&mut collector, value);
    collector.finish()
}

pub(super) fn oracle_text(value: &OracleText) -> Vec<PackedSite> {
    let mut collector = PackedSiteCollector::default();
    deckmaste_english_v2::visit::walk_oracle_text(&mut collector, value);
    collector.finish()
}

pub(super) fn write_human(
    packed_sites: &[PackedSite],
    output: &mut dyn Write,
) -> anyhow::Result<()> {
    if packed_sites.is_empty() {
        return Ok(());
    }
    writeln!(output, "packed:").context("writing packed-site heading")?;
    for packed_site in packed_sites {
        writeln!(
            output,
            "  mobile construction_path={} role={}",
            serde_json::to_string(&packed_site.construction_path)
                .context("serializing packed-site construction path")?,
            serde_json::to_string(&packed_site.role).context("serializing packed-site role")?,
        )
        .context("writing packed-site mobile")?;
        for path in &packed_site.paths {
            writeln!(
                output,
                "    site_path={}",
                serde_json::to_string(path).context("serializing packed-site path")?
            )
            .context("writing packed-site path")?;
        }
    }
    Ok(())
}

#[derive(Default)]
struct PackedSiteCollector {
    construction_path: Vec<String>,
    packed_sites: Vec<PackedSite>,
}

impl PackedSiteCollector {
    fn finish(self) -> Vec<PackedSite> {
        assert!(
            self.construction_path.is_empty(),
            "visitor must exit every construction it enters"
        );
        self.packed_sites
    }
}

impl Visitor for PackedSiteCollector {
    fn enter_construction(&mut self, construction: &'static str) {
        self.construction_path.push(construction.to_owned());
    }

    fn exit_construction(&mut self) {
        self.construction_path
            .pop()
            .expect("visitor cannot exit a construction it did not enter");
    }

    fn enter_role(
        &mut self,
        role: &'static str,
        _scope_sibling: Option<&'static str>,
        admissible_sites: Option<&deckmaste_english_v2::ast::AdmissibleSites>,
    ) {
        let Some(admissible_sites) = admissible_sites.filter(|sites| !sites.is_empty()) else {
            return;
        };
        self.packed_sites.push(PackedSite {
            construction_path: self.construction_path.clone(),
            role: role.to_owned(),
            paths: admissible_sites.paths().iter().map(site_path).collect(),
        });
    }
}

fn site_path(path: &AttachmentSitePath) -> Vec<PackedSiteStep> {
    path.steps()
        .iter()
        .map(|step| match step {
            AttachmentSiteStep::Role(name) => PackedSiteStep::Role {
                name: (*name).to_owned(),
            },
            AttachmentSiteStep::Conjunct(name, ordinal) => PackedSiteStep::Conjunct {
                name: (*name).to_owned(),
                ordinal: *ordinal,
            },
        })
        .collect()
}

#[cfg(test)]
pub(super) fn fixture() -> PackedSite {
    PackedSite {
        construction_path: vec!["Fixture".to_owned()],
        role: "mobile".to_owned(),
        paths: vec![vec![
            PackedSiteStep::Role {
                name: "host".to_owned(),
            },
            PackedSiteStep::Conjunct {
                name: "members".to_owned(),
                ordinal: 1,
            },
        ]],
    }
}
