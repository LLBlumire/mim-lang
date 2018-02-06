use ::mim_b::SectionMode;
use ::mim_b::Ops;
use byteorder::WriteBytesExt;
use byteorder::BE;

/// section {sectionmode};
/// 
/// 1. **Opcode**: Section
/// 2. Padding
/// 3. Padding
/// 4. SectionMode
pub fn section(mode: SectionMode) -> [u8; 8] {
    let mut bufinst: Vec<u8> = Vec::with_capacity(2);
    let mut bufmode: Vec<u8> = Vec::with_capacity(2);
    bufinst.write_u16::<BE>(Ops::Section as u16).unwrap();
    bufmode.write_u16::<BE>(mode as u16).unwrap();
    [bufinst[0], bufinst[1], 0, 0, 0, 0, bufmode[0], bufmode[1]]
}
