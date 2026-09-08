use deckmaste_data::mtgjson::AtomicCard;
use deckmaste_data::mtgjson::AtomicCards;
use sha2::Digest;
use sha2::Sha256;

pub(crate) const SUPPORT_FILTER: &str = "AtomicCard::vintage_playable: Vintage Legal or Restricted";

pub(crate) struct SupportedFace<'view, 'data> {
    pub group_name: &'view str,
    pub index: usize,
    pub card: &'view AtomicCard<'data>,
}

pub(crate) fn supported_faces<'view, 'data>(
    cards: &'view AtomicCards<'data>,
) -> Vec<SupportedFace<'view, 'data>> {
    let mut groups = cards.data.iter().collect::<Vec<_>>();
    groups.sort_by(|(left, _), (right, _)| left.as_str().cmp(right.as_str()));
    groups
        .into_iter()
        .flat_map(|(group, faces)| {
            faces.iter().enumerate().filter_map(move |(index, card)| {
                card.vintage_playable().then_some(SupportedFace {
                    group_name: group.as_str(),
                    index,
                    card,
                })
            })
        })
        .collect()
}

pub(crate) fn digest(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut result = String::with_capacity(64);
    for byte in Sha256::digest(bytes) {
        write!(result, "{byte:02x}").expect("writing to a String is infallible");
    }
    result
}
