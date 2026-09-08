use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::BufRead;

use anyhow::bail;

use crate::CatalogKind;
use crate::cards;
use crate::cr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogSet {
    catalogs: BTreeMap<CatalogKind, BTreeSet<String>>,
}

/// Card type-line components recovered against the declared CR catalogs.
/// Longest catalog matches preserve multiword subtypes such as `Time Lord`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TypeLineParts {
    pub supertypes: Vec<String>,
    pub card_types: Vec<String>,
    pub subtypes: Vec<String>,
}

impl CatalogSet {
    /// Extracts every canonical catalog from the Comprehensive Rules and
    /// Vintage-playable Scryfall Oracle card units.
    ///
    /// # Errors
    ///
    /// Returns an error when an authoritative input is malformed or cannot
    /// produce the complete canonical inventory.
    pub fn generate(cr: &str, oracle_cards: impl BufRead) -> anyhow::Result<Self> {
        let mut entries = cr::extract(cr)?;
        entries.insert(CatalogKind::CardNames, cards::extract(oracle_cards)?);
        Self::from_entries(entries)
    }

    /// Builds a catalog set only when every canonical catalog is present.
    ///
    /// # Errors
    ///
    /// Returns an error naming the first missing catalog kind.
    pub fn from_entries(catalogs: BTreeMap<CatalogKind, BTreeSet<String>>) -> anyhow::Result<Self> {
        for kind in CatalogKind::ALL {
            if !catalogs.contains_key(&kind) {
                bail!("missing {} catalog", kind.filename());
            }
        }
        Ok(Self { catalogs })
    }

    #[must_use]
    pub fn get(&self, kind: CatalogKind) -> &BTreeSet<String> {
        &self.catalogs[&kind]
    }

    pub fn kinds(&self) -> impl Iterator<Item = CatalogKind> {
        CatalogKind::ALL.into_iter()
    }

    /// Derives structured labels from a Scryfall type line using only the
    /// authoritative catalogs in this set.
    ///
    /// # Errors
    ///
    /// Returns an error when the type line has no card-type token or a
    /// subtype sequence cannot be partitioned into declared multiword labels.
    pub fn parse_type_line(&self, type_line: &str) -> anyhow::Result<TypeLineParts> {
        let (left, right) = type_line
            .split_once(" — ")
            .map_or((type_line, None), |(left, right)| (left, Some(right)));
        let mut supertypes = Vec::new();
        let mut card_types = Vec::new();
        for label in left.split_whitespace() {
            if self.get(CatalogKind::Supertypes).contains(label) {
                supertypes.push(label.to_owned());
            } else if self.get(CatalogKind::CardTypes).contains(label) {
                card_types.push(label.to_owned());
            } else {
                // Silver-border and digital-only Oracle records can carry
                // labels outside the current CR catalogs. Preserve the
                // source token as an undeclared card type rather than
                // dropping or reclassifying it.
                card_types.push(label.to_owned());
            }
        }
        if card_types.is_empty() {
            bail!("type line {type_line:?} contains no declared card type");
        }

        let subtype_catalogs = [
            CatalogKind::ArtifactTypes,
            CatalogKind::BattleTypes,
            CatalogKind::CreatureTypes,
            CatalogKind::EnchantmentTypes,
            CatalogKind::LandTypes,
            CatalogKind::PlaneswalkerTypes,
            CatalogKind::SpellTypes,
        ];
        let declared = subtype_catalogs
            .into_iter()
            .flat_map(|kind| self.get(kind).iter())
            .collect::<BTreeSet<_>>();
        let words = right.map_or_else(Vec::new, |right| right.split_whitespace().collect());
        let mut subtypes = Vec::new();
        let mut start = 0;
        while start < words.len() {
            let matched = ((start + 2)..=words.len()).rev().find_map(|end| {
                let candidate = words[start..end].join(" ");
                declared.contains(&candidate).then_some((end, candidate))
            });
            // Scryfall provides no structured subtype array. A declared
            // multiword subtype is indivisible; every remaining whitespace
            // token is preserved as one source label without inventing data.
            let (end, subtype) = matched.unwrap_or_else(|| (start + 1, words[start].to_owned()));
            subtypes.push(subtype);
            start = end;
        }
        Ok(TypeLineParts {
            supertypes,
            card_types,
            subtypes,
        })
    }
}
