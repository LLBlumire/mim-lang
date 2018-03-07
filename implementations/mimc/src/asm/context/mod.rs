//! Contains the compilation instruction modes and contexts.

/// The `context_rule` macro registers a context instruction. Writing bytes to a buffer and
/// returning from it's expansion site if a match occurs and is written.
macro_rules! context_rule {
    ($term:ident, $out:ident, $nametable:expr, $mne:expr, $cv:expr) => {
        context_rule!($term, $out, $nametable, $mne, $cv, (|_, _|{}))
    };
    ($term:ident, $out:ident, $nametable:expr, $mne:expr, $cv:expr, $callback:expr) => {
        context_rule!($term, $out, $nametable, $mne, $cv, $callback, true);
    };
    ($term:ident, $out:ident, $nametable:expr, $mne:expr, $cv:expr, $cb:expr, $emmit:expr) => {
        if {
            use byteorder::WriteBytesExt;
            use byteorder::BE;
            use std;

            let emmit: bool = $emmit;
            let mut expr = false;
            let mut terms = $term.split_whitespace().take(4);
            let mut iterm = terms.next();
            let term: &str = &$term[..];

            // Remove and register nametable labels
            loop {
                if let Some(term) = iterm {
                    if term.chars().last() == Some(':') {
                        $nametable.insert(term[0..term.len()-1].to_string(), $out.len() as u64);
                        iterm = terms.next();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // If context rule matches
            if Some($mne) == iterm {
                expr = true;
                let buf: &mut Vec<u8> = $out;
                let nametable: &mut HashMap<String, u64> = $nametable;
                let starting_align = buf.len();
                let mut postbuf: Vec<u64> = Vec::new();

                // Write the instruction
                buf.write_u16::<BE>($cv).unwrap();

                // For every parameter
                while let Some(param) = terms.next() {
                    // Perform constant substitutions, or parse to integer
                    if let Ok(param_val) = match param {
                        "TYPEDEF" => Ok(0),
                        "DATA" => Ok(1),
                        "MAIN" => Ok(2),
                        n if &n[..1] == ":" => Ok(nametable.get(&n[1..]).cloned().unwrap_or_else(||{
                            if emmit {
                                println!("Unknown label `{:?}` for term `{:?}`", n, $term);
                            }
                            0
                        })),
                        n => n.parse::<u64>()
                    } {
                        // Check for overflow, queueing to write a full 64 bit integer if needed
                        if param_val >= std::u16::MAX as u64 {
                            buf.write_u16::<BE>(std::u16::MAX).unwrap();
                            postbuf.push(param_val);
                        } else {
                            buf.write_u16::<BE>(param_val as u16).unwrap();
                        }
                    } else {
                        if emmit {
                            println!("Unknown data `{:?}` for term `{:?}`:", param, $term);
                        }
                        buf.write_u16::<BE>(0).unwrap();
                    }
                }
                // Align to instruction width, if not all params used
                let added = (buf.len() - starting_align) as i64;
                for _ in 0..(8 + ((-added) % 8)) {
                    buf.push(0);
                }
                // Write the extended 64 bit integers that are queued
                for extended_param in postbuf {
                    buf.write_u64::<BE>(extended_param).unwrap();
                }
                let added = (buf.len() - starting_align) as i64;
                // Signal the callback, allowing the user to perform post processing before return
                ($cb)(term, (buf, added));
            }
            expr
        } {
            // If a match occured, return from the callsite context
            return;
        }
    };
}

mod typedef;
mod data;
mod mainsec;

pub use self::typedef::typedef;
pub use self::data::data;
pub use self::mainsec::mainsec as main;
