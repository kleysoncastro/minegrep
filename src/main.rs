use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
    
    let mut f: File = File::open(filename).expect("Arquivo não encontrado");

    let mut contents = String::new();

    println!("Arquivo lido: {}", filename);


    f.read_to_string(&mut &mut contents)
    .expect("Problema pra ler o arquivo");

        
        println!("##### Conteudo: \n\n{}", contents);

}

fn parse_config(args: & [String]) -> (&str, &str) {

    let query: &String = &args[1];
    let filename: &String = & args[2];

    (query, filename)
}