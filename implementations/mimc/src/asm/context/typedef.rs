//! Contains the typedef section processing functions
use std::collections::HashMap;

/// Process the typedef section
pub fn typedef(
    buf: &mut Vec<u8>,
    term: &str,
    mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    let offset_len = buf.len();
    match *context {
        0 => no_context(buf, term, mode, context, nametable),
        1 => frame(buf, term, mode, context, nametable),
        2 => sum(buf, term, mode, context, nametable),
        3 => product(buf, term, mode, context, nametable),
        _ => println!("Unknown context for term: `{:?}", term),
    }
    if buf.len() == offset_len {
        println!("Unknown mnemonic for term: `{:?}`", term);
    }
}

/// Processes no context instructions
fn no_context(
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
    // Process type registration
    context_rule!(term, buf, nametable, "bytes", 1);
    context_rule!(term, buf, nametable, "buffer", 2);
    context_rule!(term, buf, nametable, "begin_frame", 0xFFFC, |_, _| {
        *context = 1;
    });
    context_rule!(term, buf, nametable, "begin_sum", 0xFFFD, |_, _| {
        *context = 2;
    });
    context_rule!(term, buf, nametable, "begin_product", 0xFFFE, |_, _| {
        *context = 3;
    });
}

/// Process frame context instructions
fn frame(
    buf: &mut Vec<u8>,
    term: &str,
    _mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    /// Process the ending of a frame
    context_rule!(term, buf, nametable, "end_frame", 0, |_, _| {
        *context = 0;
    });
    /// Process the return type of a frame
    context_rule!(term, buf, nametable, "return", 1);
    /// Process the local types of a frame
    context_rule!(term, buf, nametable, "local", 2);
}

/// Process the sum context instructions
fn sum(
    buf: &mut Vec<u8>,
    term: &str,
    _mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    /// Process the ending of the sum context
    context_rule!(term, buf, nametable, "end_sum", 0, |_, _| {
        *context = 0;
    });
    /// Process the variants of a sum
    context_rule!(term, buf, nametable, "variant", 1);
}

/// Process the product context instructions
fn product(
    buf: &mut Vec<u8>,
    term: &str,
    _mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    /// Process the ending of the product context
    context_rule!(term, buf, nametable, "end_product", 0, |_, _| {
        *context = 0;
    });
    /// Process the composition of a product
    context_rule!(term, buf, nametable, "compose", 1);
}
