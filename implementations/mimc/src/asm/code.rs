use byteorder::BE;
use byteorder::WriteBytesExt;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum Code {
    Entry(u8),
    Await(String),
}

pub fn slice_to_code(buffer: &[u8]) -> Vec<Code> {
    buffer.into_iter().cloned().map(|n| Code::Entry(n)).collect()
}

pub fn code_buf_len(buffer: &[Code]) -> u64 {
    let mut acc = 0;
    for entry in buffer {
        match entry {
            &Code::Entry(_) => acc += 1,
            &Code::Await(_) => acc += 8
        }
    }
    acc
}

pub fn unify_labels(buffer: &[Code], labels: HashMap<String, u64>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for entry in buffer {
        match entry {
            &Code::Entry(n) => output.push(n),
            &Code::Await(ref s) => {
                if let Some(v) = labels.get(s) {
                    output.write_u64::<BE>(*v).unwrap();
                } else {
                    panic!("Unknown Label {}", s);
                }
            }
        }
    }
    output
}