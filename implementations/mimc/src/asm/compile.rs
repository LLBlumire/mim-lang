use ::asm::code::code_buf_len;
use ::asm::code::Code;
use ::asm::code::slice_to_code;
use ::asm::code::unify_labels;
use core::mim_b::instruction;
use core::mim_b::Ops;
use core::mim_b::SectionMode;
use std::collections::HashMap;

struct CompilerState {
    operator_valency: Option<u64>,
    operator: Option<Ops>,
    section: SectionMode,
    output_buffer: Vec<Code>,
    labels: HashMap<String, u64>,
}
impl CompilerState {
    fn new() -> CompilerState {
        CompilerState {
            operator_valency: None,
            operator: None,
            section: SectionMode::Config,
            output_buffer: Vec::new(),
            labels: HashMap::new(),
        }
    }
}

pub fn compile(input: &str) -> Vec<u8> {
    let mut state: CompilerState = CompilerState::new();

    for (linenum, line) in input.lines().enumerate() {
        for word in line.replace(";", " ;").split_whitespace() {
            if word.to_string().pop().unwrap() == ':' {
                state.labels.insert(word.to_string(), code_buf_len(&state.output_buffer));
                continue;
            }

            if state.operator_valency == None {
                match Ops::from(word) {
                    Ops::Unknown => {
                        panic!("Unknown Operation on Line {}: {}", linenum, word);
                    },
                    op => {
                        state.operator_valency = Some(op.valency());
                        state.operator = Some(op);
                    }
                }
            } else if state.operator_valency.unwrap() > 0 {
                match state.operator {
                    Some(Ops::Section) => {
                        let sm = SectionMode::from(word);
                        if sm == SectionMode::Unknown {
                            panic!("Unknown Section on Line {}: {}", linenum, word);
                        }
                        state.section = sm;
                        state.operator_valency = Some(state.operator_valency.unwrap() - 1);
                        state.output_buffer.extend_from_slice(&slice_to_code(&instruction::section(sm)));
                    },
                    _ => {
                    }
                }
            } else {
                if word == ";" {
                    state.operator_valency = None;
                    state.operator = None;
                }
            }
        }
    }
    unify_labels(&state.output_buffer, state.labels)
}