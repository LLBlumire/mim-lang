//! Contains the compile method, the core method for compiling MimASM

use asm::context;
use std::collections::HashMap;

use byteorder::WriteBytesExt;
use byteorder::BE;

/// Converts an input language, MimASM, into it's output Bytes.
pub fn compile(source: &str) -> Vec<u8> {
    let mut mode = 0;
    let mut context = 0;
    let mut buf: Vec<u8> = Vec::new();
    let mut nametable: HashMap<String, u64> = HashMap::new();

    // Every term is split at a semicolon.
    for term in source.split(';') {
        if term == "" {
            continue;
        }
        let starting_mode = mode;
        match mode {
            // Mode 0 = Type Definitions
            0 => context::typedef(&mut buf, term, &mut mode, &mut context, &mut nametable),
            // Mode 1 = Data Definitions
            1 => context::data(&mut buf, term, &mut mode, &mut context, &mut nametable),
            // Mode 3 = Frame Definitions
            2 => context::main(&mut buf, term, &mut mode, &mut context, &mut nametable),
            _ => println!("Unknown mode for term: `{:?}`", term),
        }
        // If the mode has changed
        if mode != starting_mode {
            let added = buf.len() as i64;
            // If padding is required, pad to be instruction aligned between sections
            if added % 8 != 0 {
                for _ in 0..(8 + ((-added) % 8)) {
                    buf.push(0);
                }
            }
            // Reset the context
            context = 0;
            // If entering the Data mode, write a blank instruction to fill with the skip length
            if mode == 1 {
                buf.write_u64::<BE>(0).unwrap();
            }
        }
    }

    buf
}
