#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseContext<'a> {
    card_name: &'a str,
    abbreviated_card_name: &'a str,
}

impl<'a> ParseContext<'a> {
    #[must_use]
    pub fn new(card_name: &'a str) -> Option<Self> {
        let abbreviated_card_name = card_name
            .split_once(',')
            .map_or(card_name, |(abbreviated, _)| abbreviated);
        (!card_name.is_empty() && !abbreviated_card_name.is_empty()).then_some(Self {
            card_name,
            abbreviated_card_name,
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
}
