use super::Instruction;

pub fn parse(source: &str) -> Option<Box<Instruction>> {
    match source {
        "section" => Some(Box::new(Section)),
        _ => None,
    }
}

pub struct Section;
impl Instruction for Section {
    fn ident(&self) -> &str { "section" }
    fn opcode(&self) -> u16 { 0 }
}