use asm::context;
use std::collections::HashMap;

use byteorder::WriteBytesExt;
use byteorder::BE;

pub fn compile(source: &str) -> Vec<u8> {
    let mut mode = 0;
    let mut context = 0;
    let mut buf: Vec<u8> = Vec::new();
    let mut nametable: HashMap<String, u64> = HashMap::new();

    for term in source.split(';') {
        if term == "" {
            continue;
        }
        let starting_mode = mode;
        match mode {
            0 => context::typedef(&mut buf, term, &mut mode, &mut context, &mut nametable),
            1 => context::data(&mut buf, term, &mut mode, &mut context, &mut nametable),
            2 => context::main(&mut buf, term, &mut mode, &mut context, &mut nametable),
            _ => println!("Unknown mode for term: `{:?}`", term),
        }
        if mode != starting_mode {
            let added = buf.len() as i64;
            if added % 8 != 0 {
                for _ in 0..(8 + ((-added) % 8)) {
                    buf.push(0);
                }
            }
            context = 0;
            if mode == 1 {
                buf.write_u64::<BE>(0).unwrap();
            }
        }
    }

    buf
}
