//! Contains the `Ops` enum.

/// Operations available to compute
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ops {
    /// No operation performed
    NoOp = 0,
    /// Change `SectionMode`
    Section,
    /// Define a type that is a sequence of bytes
    Bytes,
    /// Define a type that is a buffer of another type
    Buffer,
    /// Define a frame type (mapping)
    BeginFrame,
    /// The frame has no domain parameters.
    NoDomain,
    /// Define a domain parameter of a frame (mapping).
    Domain,
    /// Define a locally acquired variable. Kept behind a strong pointer.
    Local,
    /// Define the return range of a frame.
    Range,
    /// Define the declared programmatical body of the frame.
    Body,
    /// End a frame (mapping).
    EndFrame,
    /// Place raw utf32 string data into memory.
    DataUtf32,
    /// Place a raw 64 bit value into memory.
    Data,
    /// Begin defining the body of a frame.
    BeginFrameBody,
    /// Initialise a local buffer.
    LocalBufferInit,
    /// Output a local utf32 sequence.
    StdoutUtf32,
    /// End the body of a frame.
    EndFrameBody,
    /// An unknown instruction.
    Unknown,
}

#[allow(match_same_arms)]
impl Ops {
    /// Returns the number of parameters an instruction expects.
    pub fn valency(&self) -> u64 {
        match *self {
            Ops::NoOp => 0,
            Ops::Section => 1,
            Ops::Bytes => 1,
            Ops::Buffer => 1,
            Ops::BeginFrame => 0,
            Ops::NoDomain => 0,
            Ops::Domain => 1,
            Ops::Local => 2,
            Ops::Range => 1,
            Ops::Body => 1,
            Ops::EndFrame => 0,
            Ops::DataUtf32 => 1,
            Ops::Data => 1,
            Ops::BeginFrameBody => 1,
            Ops::LocalBufferInit => 3,
            Ops::StdoutUtf32 => 2,
            Ops::EndFrameBody => 0,
            Ops::Unknown => u64::max_value(),
        }
    }
}

impl<'a> From<&'a str> for Ops {
    fn from(from: &'a str) -> Ops {
        match from {
            "no_op" => Ops::NoOp,
            "section" => Ops::Section,
            "bytes" => Ops::Bytes,
            "buffer" => Ops::Buffer,
            "begin_frame" => Ops::BeginFrame,
            "no_domain" => Ops::NoDomain,
            "local" => Ops::Local,
            "range" => Ops::Range,
            "body" => Ops::Body,
            "end_frame" => Ops::EndFrame,
            "data_utf32" => Ops::DataUtf32,
            "data" => Ops::Data,
            "begin_frame_body" => Ops::BeginFrameBody,
            "local_buffer_init" => Ops::LocalBufferInit,
            "stdout_utf32" => Ops::StdoutUtf32,
            "end_frame_body" => Ops::EndFrameBody,
            _ => Ops::Unknown,
        }
    }
}
