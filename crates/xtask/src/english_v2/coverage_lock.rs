use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Context;
use anyhow::bail;
use tempfile::NamedTempFile;

pub(super) const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub(super) struct CoverageLock {
    schema_version: u32,
    source_fingerprint: String,
    accepted: BTreeSet<String>,
}

impl CoverageLock {
    pub(super) fn read(path: &Path) -> anyhow::Result<Self> {
        let bytes = fs::read(path)
            .with_context(|| format!("reading English-v2 coverage lock {}", path.display()))?;
        let lock = serde_json::from_slice::<Self>(&bytes)
            .with_context(|| format!("parsing English-v2 coverage lock {}", path.display()))?;
        lock.validate(path)?;
        Ok(lock)
    }

    pub(super) fn check(&self, current: &BTreeSet<String>) -> anyhow::Result<()> {
        let lost = self.accepted.difference(current).collect::<Vec<_>>();
        if lost.is_empty() {
            return Ok(());
        }

        bail!(
            "lost {} previously accepted corpus identit{}:\n{}",
            lost.len(),
            if lost.len() == 1 { "y" } else { "ies" },
            lost.into_iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }

    pub(super) fn bless(
        &self,
        current: &BTreeSet<String>,
        source_fingerprint: &str,
    ) -> anyhow::Result<Self> {
        self.check(current)?;
        Self::new(current.clone(), source_fingerprint.to_owned())
    }

    pub(super) fn new(
        accepted: BTreeSet<String>,
        source_fingerprint: String,
    ) -> anyhow::Result<Self> {
        let lock = Self {
            schema_version: SCHEMA_VERSION,
            source_fingerprint,
            accepted,
        };
        lock.validate(Path::new("<new coverage lock>"))?;
        Ok(lock)
    }

    pub(super) fn write(&self, path: &Path) -> anyhow::Result<()> {
        self.validate(path)?;
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        let mut temporary = NamedTempFile::new_in(parent).with_context(|| {
            format!(
                "creating temporary English-v2 coverage lock beside {}",
                path.display()
            )
        })?;
        let mut serialized =
            serde_json::to_vec_pretty(self).context("serializing English-v2 coverage lock")?;
        serialized.push(b'\n');
        temporary
            .write_all(&serialized)
            .context("writing temporary English-v2 coverage lock")?;
        temporary
            .flush()
            .context("flushing temporary English-v2 coverage lock")?;
        temporary
            .persist(path)
            .map_err(|error| error.error)
            .with_context(|| format!("persisting English-v2 coverage lock {}", path.display()))?;
        Ok(())
    }

    pub(super) fn accepted(&self) -> &BTreeSet<String> {
        &self.accepted
    }

    pub(super) fn source_fingerprint(&self) -> &str {
        &self.source_fingerprint
    }

    fn validate(&self, path: &Path) -> anyhow::Result<()> {
        if self.schema_version != SCHEMA_VERSION {
            bail!(
                "invalid English-v2 coverage lock {}: unsupported schema version {} (expected {})",
                path.display(),
                self.schema_version,
                SCHEMA_VERSION,
            );
        }
        validate_identity(&self.source_fingerprint, "source fingerprint", path)?;
        for accepted in &self.accepted {
            validate_identity(accepted, "accepted corpus identity", path)?;
        }
        Ok(())
    }
}

fn validate_identity(value: &str, kind: &str, path: &Path) -> anyhow::Result<()> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Ok(());
    }

    bail!(
        "invalid English-v2 coverage lock {}: {kind} must be lowercase 64-hex",
        path.display(),
    );
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fmt::Write as _;
    use std::fs;
    use std::path::Path;

    use sha2::Digest;
    use sha2::Sha256;

    use super::CoverageLock;
    use super::SCHEMA_VERSION;

    fn id(digit: char) -> String {
        digit.to_string().repeat(64)
    }

    fn set(values: impl IntoIterator<Item = String>) -> BTreeSet<String> {
        values.into_iter().collect()
    }

    fn lock(values: impl IntoIterator<Item = String>, source: String) -> CoverageLock {
        CoverageLock {
            schema_version: SCHEMA_VERSION,
            source_fingerprint: source,
            accepted: set(values),
        }
    }

    #[test]
    fn check_and_bless_only_allow_accepted_identity_growth() {
        let (a, b, c) = (id('a'), id('b'), id('c'));
        let baseline = lock([a.clone(), b.clone()], id('1'));
        assert!(
            baseline
                .check(&set([a.clone(), b.clone(), c.clone()]))
                .is_ok()
        );
        let error = baseline
            .check(&set([b.clone(), c.clone()]))
            .unwrap_err()
            .to_string();
        assert!(error.contains("lost 1 previously accepted corpus identity"));
        assert!(error.contains(&a));

        let ordered_error = lock([a.clone(), c.clone()], id('1'))
            .check(&set([b.clone()]))
            .unwrap_err()
            .to_string();
        assert!(ordered_error.find(&a).unwrap() < ordered_error.find(&c).unwrap());

        let blessed = baseline
            .bless(&set([a.clone(), b.clone(), c.clone()]), &id('2'))
            .unwrap();
        assert_eq!(blessed.accepted, set([a.clone(), b.clone(), c.clone()]));
        assert_eq!(blessed.source_fingerprint, id('2'));
        assert!(baseline.bless(&set([b, c]), &id('2')).is_err());
    }

    #[test]
    fn disk_round_trip_is_canonical_sorted_and_newline_terminated() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let baseline = lock([id('b'), id('a')], id('1'));

        baseline.write(&path).unwrap();
        let first = fs::read(&path).unwrap();
        let read_back = CoverageLock::read(&path).unwrap();
        read_back.write(&path).unwrap();
        let second = fs::read(&path).unwrap();

        assert_eq!(first, second);
        assert_eq!(
            fs::read_dir(directory.path())
                .unwrap()
                .filter_map(Result::ok)
                .count(),
            1,
        );
        assert!(first.ends_with(b"\n"));
        assert!(!first.ends_with(b"\n\n"));
        let serialized = String::from_utf8(first).unwrap();
        assert!(serialized.find(&id('a')).unwrap() < serialized.find(&id('b')).unwrap());
    }

    #[test]
    fn read_rejects_invalid_schema_and_names_its_path() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        fs::write(
            &path,
            format!(
                "{{\n  \"schema_version\": 0,\n  \"source_fingerprint\": \"{}\",\n  \"accepted\": [\"{}\"]\n}}\n",
                id('1'),
                id('a'),
            ),
        )
        .unwrap();

        let error = CoverageLock::read(&path).unwrap_err().to_string();
        assert!(error.contains("unsupported schema version 0"));
        assert!(error.contains(&path.display().to_string()));

        fs::write(
            &path,
            format!(
                "{{\n  \"schema_version\": 1,\n  \"source_fingerprint\": \"{}\",\n  \"accepted\": [\"{}\"]\n}}\n",
                "A".repeat(64),
                id('a'),
            ),
        )
        .unwrap();
        let error = CoverageLock::read(&path).unwrap_err().to_string();
        assert!(error.contains("source fingerprint must be lowercase 64-hex"));
    }

    #[test]
    fn write_rejects_noncanonical_identity_before_persisting() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("coverage.lock");
        let invalid = lock(["A".repeat(64)], id('1'));

        let error = invalid.write(&path).unwrap_err().to_string();

        assert!(error.contains("accepted corpus identity must be lowercase 64-hex"));
        assert!(!path.exists());
    }

    #[test]
    fn production_lock_adds_only_the_reviewed_rend_spirit_identity() {
        const REND_SPIRIT: &str =
            "5a0bd9563d2e05ca394ee7bedc5e55f386f82ee16f4227c410565066c6585660";
        const PRIOR_ACCEPTED_SHA256: &str =
            "042ca946aff58bad02ba7c2daf6df3fce41493939c59caeab4022bb790669409";

        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../english-v2-coverage.lock");
        let lock = CoverageLock::read(&path).expect("production coverage lock is valid");
        let mut prior = lock.accepted().clone();

        assert_eq!(lock.accepted().len(), 48);
        assert!(prior.remove(REND_SPIRIT));
        assert_eq!(prior.len(), 47);

        let mut hasher = Sha256::new();
        for identity in prior {
            hasher.update(identity.as_bytes());
            hasher.update(b"\n");
        }
        let digest = hasher.finalize();
        let mut hexadecimal = String::with_capacity(digest.len() * 2);
        for byte in digest {
            write!(&mut hexadecimal, "{byte:02x}").expect("writing to String cannot fail");
        }
        assert_eq!(hexadecimal, PRIOR_ACCEPTED_SHA256);
    }
}
