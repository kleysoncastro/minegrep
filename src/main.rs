use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args);
    
    let mut f: File = File::open(&config.filename).expect("Arquivo não encontrado");

    let mut contents: String = String::new();



    f.read_to_string(&mut &mut contents)
    .expect("Problema pra ler o arquivo");

        
        println!("##### Conteudo: \n\n{}", contents);

}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query: String = args[1].clone();
        let filename: String = args[2].clone();

        Config {query, filename}
    }
}
