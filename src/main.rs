use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let mut f = File::open(filename).expect("Arquivo não encontrado");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Problema na leitura do arquivo");

    println!("With text:\n{}", contents);
}