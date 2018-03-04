use std::collections::HashMap;
use byteorder::WriteBytesExt;
use byteorder::BE;
use std::io::Cursor;

pub fn data(
    buf: &mut Vec<u8>,
    term: &str,
    mode: &mut i32,
    context: &mut i32,
    nametable: &mut HashMap<String, u64>,
) {


    let offset_len = buf.len();
    context_rule!(term, buf, nametable, "section", 0, |term: &str, (buf, _): (&mut Vec<u8>, _)| {
        match term.split_whitespace().last() {
            Some("TYPEDEF") => *mode = 0,
            Some("DATA") => *mode = 1,
            Some("MAIN") => *mode = 2,
            _ => println!("Unknown mode entry for term: `{:?}`", term),
        };
        *context += (buf.len() - offset_len) as i32;
        let context = *context as usize;
        let buflen = buf.len();
        let mut cursor = Cursor::new(&mut buf[buflen-context-8..buflen-context]);
        cursor.write_u64::<BE>(context as u64).unwrap();
    });

    // Call inside context rule to properly process all nametabling
    context_rule!(term, buf, nametable, "number", 0, |term: &str, (buf, added): (&mut Vec<u8>, i64)| {
        for _ in 0..added {
            buf.pop();
        }
        let mut params = term.split_whitespace().skip_while(|n| &n[n.len()-1..] == ":").skip(1);
        if let Some(Some(base)) = params.next().map(|n| n.parse::<u32>().ok()) {
            if let Some(bytes) = params.next() {
                if let Some(number) = params.next() {
                    macro_rules! err {
                        () => {{
                            println!("Number badly formatted for term: `{}`", term);
                            0
                        }}
                    }
                    match bytes {
                        "1" | "size8" => buf.write_u8(u8::from_str_radix(number, base).unwrap_or_else(|_| err!())),
                        "2" | "size16" => buf.write_u16::<BE>(u16::from_str_radix(number, base).unwrap_or_else(|_| err!())),
                        "4" | "size32" => buf.write_u32::<BE>(u32::from_str_radix(number, base).unwrap_or_else(|_| err!())),
                        "8" | "size64" => buf.write_u64::<BE>(u64::from_str_radix(number, base).unwrap_or_else(|_| err!())),
                        _ => {
                            println!("Unknown size align for term: `{}`", term);
                            buf.write_u64::<BE>(u64::from_str_radix(number, base).unwrap_or_else(|_| err!()))
                        }
                    }.unwrap();
                } else {
                    println!("Number requires number for term: `{}`", term);
                }
            } else {
                println!("Number requires byte size for term: `{}`", term);
            }
        } else {
            println!("Number requires base for term: `{}`", term);
        }
        *context += (buf.len() - offset_len) as i32;
    }, false);
    context_rule!(term, buf, nametable, "signed", 0, |term: &str, (buf, added): (&mut Vec<u8>, i64)| {
        for _ in 0..added {
            buf.pop();
        }
        let mut params = term.split_whitespace().skip_while(|n| &n[n.len()-1..] == ":").skip(1);
        if let Some(Some(base)) = params.next().map(|n| n.parse::<u32>().ok()) {
            if let Some(bytes) = params.next() {
                if let Some(number) = params.next() {
                    macro_rules! err {
                        () => {{
                            println!("Number badly formatted for term: `{}`", term);
                            0
                        }}
                    }
                    match bytes {
                        "1" | "size8" => buf.write_i8(i8::from_str_radix(number, base).unwrap_or_else(|_| err!())),
                        "2" | "size16" => buf.write_i16::<BE>(i16::from_str_radix(number, base).unwrap_or_else(|_| err!())),
                        "4" | "size32" => buf.write_i32::<BE>(i32::from_str_radix(number, base).unwrap_or_else(|_| err!())),
                        "8" | "size64" => buf.write_i64::<BE>(i64::from_str_radix(number, base).unwrap_or_else(|_| err!())),
                        _ => {
                            println!("Unknown size align for term: `{}`", term);
                            buf.write_i64::<BE>(i64::from_str_radix(number, base).unwrap_or_else(|_| err!()))
                        }
                    }.unwrap();
                } else {
                    println!("Number requires number for term: `{}`", term);
                }
            } else {
                println!("Number requires byte size for term: `{}`", term);
            }
        } else {
            println!("Number requires base for term: `{}`", term);
        }
        *context += (buf.len() - offset_len) as i32;
    }, false);
    context_rule!(term, buf, nametable, "string", 0, |term: &str, (buf, added): (&mut Vec<u8>, i64)| {
        for _ in 0..added {
            buf.pop();
        }
        if let Some(start) = term.find('`') {
            if let Some(end) = term.rfind('\'') {
                for byte in term[start..end].as_bytes() {
                    buf.push(*byte);
                }
            } else {
                println!("String requires end for term: `{}`", term);
            }
        } else {
            println!("String requires start for term: `{}`", term);
        }
        *context += (buf.len() - offset_len) as i32;
    }, false);


    if buf.len() == offset_len {
        println!("Unknown mnemonic for term: `{:?}`", term);
    }
}