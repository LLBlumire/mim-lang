#[macro_use]
extern crate clap;

extern crate byteorder;
extern crate mim_lang_core as core;

mod asm;

use clap::App;
use clap::Arg;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let matches = App::new("Mim Compiler")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Compiler Mim and MimASM")
        .arg(
            Arg::with_name("asm")
                .short("a")
                .long("asm")
                .value_name("FILE")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("out")
            .short("o")
            .long("out")
            .value_name("FILE")
            .takes_value(true)
            .required(true)
        )
        .get_matches();
    
    let out_path = matches.value_of("out").unwrap();
    let mut out_file = File::create(out_path).unwrap();

    for asm_path in matches.values_of("asm").unwrap() {
        let mut asm_file = File::open(&asm_path).unwrap();
        let mut buf = String::new();
        asm_file.read_to_string(&mut buf).unwrap();
        out_file.write(&asm::compile(&buf)).unwrap();
    }
}
