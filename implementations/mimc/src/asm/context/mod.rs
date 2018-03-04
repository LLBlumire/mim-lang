macro_rules! context_rule {
    ($term:ident, $out:ident, $nametable:expr, $mne:expr, $cv:expr) => {
        context_rule!($term, $out, $nametable, $mne, $cv, (|_, _|{}))
    };
    ($term:ident, $out:ident, $nametable:expr, $mne:expr, $cv:expr, $callback:expr) => {
        context_rule!($term, $out, $nametable, $mne, $cv, $callback, true);
    };
    ($term:ident, $out:ident, $nametable:expr, $mne:expr, $cv:expr, $callback:expr, $emmit:expr) => {
        if {
            let emmit: bool = $emmit;
            let mut expr = false;
            use byteorder;
            use std;
            let mut terms = $term.split_whitespace();
            let mut iterm = terms.next();
            let term: &str = &$term[..];

            loop {
                if let Some(internal_term) = iterm {
                    if internal_term.chars().last() == Some(':') {
                        $nametable.insert(internal_term[0..internal_term.len()-1].to_string(), $out.len() as u64);
                        iterm = terms.next();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if Some($mne) == iterm {
                expr = true;
                let buf: &mut Vec<u8> = $out;
                let nametable: &mut HashMap<String, u64> = $nametable;
                let starting_align = buf.len();
                let mut postbuf: Vec<u64> = Vec::new();
                byteorder::WriteBytesExt::write_u16::<byteorder::BE>(buf, $cv).unwrap();
                while let Some(param) = terms.next() {
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
                        if param_val >= std::u16::MAX as u64 {
                            byteorder::WriteBytesExt::write_u16::<byteorder::BE>(buf, std::u16::MAX).unwrap();
                            postbuf.push(param_val);
                        } else {
                            byteorder::WriteBytesExt::write_u16::<byteorder::BE>(buf, param_val as u16).unwrap();
                        }
                    } else {
                        if emmit {
                            println!("Unknown data `{:?}` for term `{:?}`:", param, $term);
                        }
                        byteorder::WriteBytesExt::write_u16::<byteorder::BE>(buf, 0).unwrap();
                    }
                }
                let added = (buf.len() - starting_align) as i64;
                for _ in 0..(8 + ((-added) % 8)) {
                    buf.push(0);
                }
                for extended_param in postbuf {
                    byteorder::WriteBytesExt::write_u64::<byteorder::BE>(buf, extended_param).unwrap();
                }
                let added = (buf.len() - starting_align) as i64;
                ($callback)(term, (buf, added));
            }
            expr
        } {
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