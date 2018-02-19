pub mod typedef;

use byteorder::WriteBytesExt;
use byteorder::BE;

macro_rules! write_instruction_component {
    ($test:ident, $insbyte:ident, $extbyte:ident, $bytes:ident) => {
        if $test < (u16::max_value() as u64) {
            $insbyte.write_u16::<BE>($test as u16).unwrap();
        } else {
            $insbyte.write_u16::<BE>(u16::max_value()).unwrap();
            $extbyte.write_u64::<BE>($test).unwrap();
            $bytes += 1;
        }
    }
}

pub trait Instruction {
    fn ident(&self) -> &str;
    fn opcode(&self) -> u16;
    fn compile(&self, first: u64, second: u64, third: u64) -> (Vec<u8>, u64) {
        let mut insbyte: Vec<u8> = Vec::new();
        let mut extbyte: Vec<u8> = Vec::new();
        let mut bytes_used: u64 = 0;
        
        insbyte.write_u16::<BE>(self.opcode()).unwrap();

        write_instruction_component!(first, insbyte, extbyte, bytes_used);
        write_instruction_component!(second, insbyte, extbyte, bytes_used);
        write_instruction_component!(third, insbyte, extbyte, bytes_used);

        insbyte.append(&mut extbyte);
        (insbyte, bytes_used + 1)
    }
}