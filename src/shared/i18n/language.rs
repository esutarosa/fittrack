#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AppLanguage {
    #[default]
    English,
    Ukrainian,
}

impl AppLanguage {
    pub const ALL: [Self; 2] = [Self::English, Self::Ukrainian];

    pub fn short_label(self) -> &'static str {
        match self {
            Self::English => "EN",
            Self::Ukrainian => "UA",
        }
    }
}
