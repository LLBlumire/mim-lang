mod mode;
mod instruction;

use std::collections::HashMap;

pub use self::mode::*;
pub use self::instruction::*;

pub fn compile(source: &str) -> Vec<u8> {
    let mut output_buf: Vec<u8> = Vec::new();
    let mut curptr: u64 = 0;
    let mut mode: Mode = Mode::TypeDef;
    let mut labels: HashMap<String, u64> = HashMap::new();

    for (statement_number, statement) in source.split(';').enumerate() {
        let mut term_offset: u64 = 0;
        let mut instruction: Option<Box<Instruction>> = None; 
        let mut first_param = None;
        let mut second_param = None;
        let mut third_param = None;
        for (term_number, term) in statement.split_whitespace().enumerate() {
            println!("PROC: {:09}.{:09} {}", statement_number, term_number, term);
            let term_number: u64 = term_number as u64 - term_offset;
            if term_number == 0 {
                if &term[term.len()-1..] == ":" {
                    labels.insert((&term[..term.len()-1]).to_owned(), curptr);
                    term_offset += 1;
                }
                instruction = Some(typedef::parse(term).unwrap());
            } else if term_number <= 3 {
                if let Some(ref instruction) = instruction {
                    if instruction.ident() == "section" && term_number == 1 {
                        mode = match term {
                            "main" => Mode::Main,
                            "data" => Mode::Data,
                            "typedef" | _ => Mode::TypeDef,
                        }
                    } else {
                        *[&mut first_param, &mut second_param, &mut third_param][(term_number - 1) as usize] = Some(term);
                    }
                }
            }
        }
    }

    output_buf
}

fn parse_term(term: &str, labels: &HashMap<String, u64>) -> u64 {
    term.parse().unwrap_or_else(|_| {
        *labels.get(term).expect("Undefined Label")
    })
}