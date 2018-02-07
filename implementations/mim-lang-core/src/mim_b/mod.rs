//! Provides resources specific to bytecode compilation. 

mod section_mode;
mod ops;

pub mod instruction;

pub use self::section_mode::SectionMode;
pub use self::ops::Ops;
