#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SectionMode {
    Config = 0,
    Typedef,
    Data,
    Main,
    Unknown
}
impl<'a> From<&'a str> for SectionMode {
    fn from(from: &'a str) -> SectionMode {
        match from {
            "config" => SectionMode::Config,
            "typedef" => SectionMode::Typedef,
            "data" => SectionMode::Data,
            "main" => SectionMode::Main,
            _ => SectionMode::Unknown,
        }
    }
}