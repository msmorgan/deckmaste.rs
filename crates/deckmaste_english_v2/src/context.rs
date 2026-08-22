use macro_ron::v2::Onset;

use crate::orthography::surface_onset;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseContext<'a> {
    card_name: &'a str,
    abbreviated_card_name: &'a str,
    card_name_onset: Onset,
    abbreviated_card_name_onset: Onset,
}

impl<'a> ParseContext<'a> {
    #[must_use]
    pub fn new(card_name: &'a str) -> Option<Self> {
        let abbreviated_card_name = card_name
            .split_once(',')
            .map_or(card_name, |(abbreviated, _)| abbreviated);
        let card_name_onset = surface_onset(card_name)?;
        let abbreviated_card_name_onset = surface_onset(abbreviated_card_name)?;
        (!card_name.is_empty() && !abbreviated_card_name.is_empty()).then_some(Self {
            card_name,
            abbreviated_card_name,
            card_name_onset,
            abbreviated_card_name_onset,
        })
    }

    #[must_use]
    pub const fn card_name(self) -> &'a str {
        self.card_name
    }

    #[must_use]
    pub const fn abbreviated_card_name(self) -> &'a str {
        self.abbreviated_card_name
    }

    pub(crate) const fn card_name_onset(self) -> Onset {
        self.card_name_onset
    }

    pub(crate) const fn abbreviated_card_name_onset(self) -> Onset {
        self.abbreviated_card_name_onset
    }
}
