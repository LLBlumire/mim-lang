use std::collections::HashMap;

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

fn no_context(
    buf: &mut Vec<u8>,
    term: &str,
    mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    context_rule!(term, buf, nametable, "section", 0, |term: &str, _| {
        match term.split_whitespace().last() {
            Some("TYPEDEF") => *mode = 0,
            Some("DATA") => *mode = 1,
            Some("MAIN") => *mode = 2,
            _ => println!("Unknown mode entry for term: `{:?}`", term),
        };
    });
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


fn frame(
    buf: &mut Vec<u8>,
    term: &str,
    _mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    context_rule!(term, buf, nametable, "end_frame", 0, |_, _| {
        *context = 0;
    });
    context_rule!(term, buf, nametable, "return", 1);
    context_rule!(term, buf, nametable, "local", 2);
}

fn sum(
    buf: &mut Vec<u8>,
    term: &str,
    _mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    context_rule!(term, buf, nametable, "end_sum", 0, |_, _| {
        *context = 0;
    });
    context_rule!(term, buf, nametable, "variant", 1);
}

fn product(
    buf: &mut Vec<u8>,
    term: &str,
    _mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {
    context_rule!(term, buf, nametable, "end_product", 0, |_, _| {
        *context = 0;
    });
    context_rule!(term, buf, nametable, "compose", 1);
}