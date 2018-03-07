//! Contains the main section processing functions
use std::collections::HashMap;

/// Processes the main section
pub fn mainsec(
    buf: &mut Vec<u8>,
    term: &str,
    mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    let offset_len = buf.len();
    match *context {
        0 => no_context(buf, term, mode, context, nametable),
        1 => frame_body(buf, term, mode, context, nametable),
        _ => println!("Unknown context for term: `{:?}", term),
    }
    if buf.len() == offset_len {
        println!("Unknown mnemonic for term: `{:?}`", term);
    }
}

/// Processes contextless instructions
pub fn no_context(
    buf: &mut Vec<u8>,
    term: &str,
    mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    // Process section changes
    context_rule!(term, buf, nametable, "section", 0, |term: &str, _| {
        match term.split_whitespace().last() {
            Some("TYPEDEF") => *mode = 0,
            Some("DATA") => *mode = 1,
            Some("MAIN") => *mode = 2,
            _ => println!("Unknown mode entry for term: `{:?}`", term),
        };
    });
    // Process the beginning of a frame body
    context_rule!(term, buf, nametable, "begin_frame_body", 1, |_, _| {
        *context = 1;
    });
}

/// Processes Frame Body Instructions
pub fn frame_body(
    buf: &mut Vec<u8>,
    term: &str,
    _mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    // Process the end of a frame body
    context_rule!(term, buf, nametable, "end_frame_body", 0x0, |_, _| {
        *context = 0;
    });
    // Process all standard in method operations
    context_rule!(term, buf, nametable, "init_bytes_data", 0x1);
    context_rule!(term, buf, nametable, "init_buffer_data", 0x2);
    context_rule!(term, buf, nametable, "copy_bytes_local", 0x3);
    context_rule!(term, buf, nametable, "copy_buffer_local", 0x4);
    context_rule!(term, buf, nametable, "copy_product_local", 0x5);
    context_rule!(term, buf, nametable, "copy_sum_local", 0x6);
    context_rule!(term, buf, nametable, "set_product_local", 0x7);
    context_rule!(term, buf, nametable, "set_sum_local", 0x8);
    context_rule!(term, buf, nametable, "set_frame_local", 0x9);
    context_rule!(term, buf, nametable, "call_frame", 0xA);
    context_rule!(term, buf, nametable, "add", 0xB);
    context_rule!(term, buf, nametable, "sadd", 0xC);
    context_rule!(term, buf, nametable, "sub", 0xD);
    context_rule!(term, buf, nametable, "ssub", 0xE);
    context_rule!(term, buf, nametable, "mul", 0xF);
    context_rule!(term, buf, nametable, "smul", 0x10);
    context_rule!(term, buf, nametable, "rem", 0x13);
    context_rule!(term, buf, nametable, "srem", 0x14);
    context_rule!(term, buf, nametable, "fadd", 0x15);
    context_rule!(term, buf, nametable, "fsub", 0x16);
    context_rule!(term, buf, nametable, "fmul", 0x17);
    context_rule!(term, buf, nametable, "fdiv", 0x18);
    context_rule!(term, buf, nametable, "and", 0x19);
    context_rule!(term, buf, nametable, "or", 0xB);
    context_rule!(term, buf, nametable, "xor", 0x1B);
    context_rule!(term, buf, nametable, "shl", 0x1D);
    context_rule!(term, buf, nametable, "shr", 0x1E);
    context_rule!(term, buf, nametable, "ashr", 0x1F);
    context_rule!(term, buf, nametable, "rll", 0x20);
    context_rule!(term, buf, nametable, "rlr", 0x21);
    context_rule!(term, buf, nametable, "bpush", 0x22);
    context_rule!(term, buf, nametable, "bpop", 0x23);
    context_rule!(term, buf, nametable, "bget", 0x24);
    context_rule!(term, buf, nametable, "bput", 0x25);
    context_rule!(term, buf, nametable, "blen", 0x26);
    context_rule!(term, buf, nametable, "eq", 0x27);
    context_rule!(term, buf, nametable, "leq", 0x28);
    context_rule!(term, buf, nametable, "sleq", 0x29);
    context_rule!(term, buf, nametable, "fleq", 0x2A);
    context_rule!(term, buf, nametable, "geq", 0x2B);
    context_rule!(term, buf, nametable, "sgeq", 0x2C);
    context_rule!(term, buf, nametable, "fgeq", 0x2D);
    context_rule!(term, buf, nametable, "jump", 0x2E);
    context_rule!(term, buf, nametable, "jumpif", 0x2F);
    context_rule!(term, buf, nametable, "stdout_string", 0xFFFC);
    context_rule!(term, buf, nametable, "stdout_dec", 0xFFFE);
}
