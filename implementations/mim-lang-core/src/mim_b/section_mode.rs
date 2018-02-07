//! Contains the `SectionMode` enum.

/// The different modes the virtual machine can be in.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SectionMode {
    /// Configuring VM settings.
    Config = 0,
    /// Defining VM types.
    Typedef,
    /// Defining constant data.
    Data,
    /// Defining program code.
    Main,
    /// Unknown section Mode
    Unknown,
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
