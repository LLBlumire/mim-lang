#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ops {
    NoOp = 0,
    Section = 1,
    Bytes,
    Buffer,
    BeginFrame,
    NoDomain,
    Local,
    Range,
    Body,
    EndFrame,
    DataUtf32,
    Data,
    BeginFrameBody,
    LocalBufferInit,
    StdoutUtf32,
    EndFrameBody,
    Unknown
}
impl Ops {
    pub fn valency(&self) -> u64 {
        match *self {
            Ops::NoOp => 0,
            Ops::Section => 1,
            Ops::Bytes => 1,
            Ops::Buffer => 1,
            Ops::BeginFrame => 0,
            Ops::NoDomain => 0,
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
            _ => Ops::Unknown
        }
    }
}