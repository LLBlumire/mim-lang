//! The Mim Compiler
//! Facilitates the compilation of MimASM.

#[macro_use]
extern crate clap;

extern crate byteorder;

use clap::Arg;
use clap::App;
use std::fs::File;
use std::io::Read;
use std::io::Write;

mod asm;

fn main() {
    let matches = App::new("Mim Compiler")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("assembler")
                .short("a")
                .long("assembler")
                .value_name("FILE")
                .help("A MimASM file to compile.")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("The file to output too.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let output_file = matches.value_of("output").unwrap();

    let mut output: Vec<u8> = Vec::new();
    for asm_file in matches.values_of("assembler").into_iter().flat_map(|n| n) {
        let file_content = {
            if let Ok(mut file_handle) = File::open(asm_file) {
                let mut buf = String::new();
                if let Ok(_) = file_handle.read_to_string(&mut buf) {
                } else {
                    println!("Unable to Read File: `{}`", asm_file);
                }
                buf
            } else {
                println!("Unknown Input Assembler: `{}`", asm_file);
                continue;
            }
        };
        println!("Building Input Assembler: `{}`", asm_file);
        output.append(&mut asm::compile(&file_content));
    }

    if let Ok(mut file_handle) = File::create(output_file) {
        if let Ok(_) = file_handle.write_all(&output) {
            println!("Writing to File: `{}`", output_file);
        } else {
            println!("Failed to Write: `{}`", output_file);
        }
    } else {
        println!("Failed to Create: `{}`", output_file);
    }
}
