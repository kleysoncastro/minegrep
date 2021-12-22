extern crate minegrep;

use std::env;
use std::process;

use minegrep::Config;
fn main() {

    let args: Vec<String> = env::args().collect();

    // verifica se todos os argumentos necessarios foram passados 
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problema nos argumentos passados: {}", err);
        process::exit(1);
    });
    
     println!("Searching for {}", config.query);
    println!("In file {}", config.filename); 


    // Ferifica se a erro nos parametros
    if let Err(e) = minegrep::run(config) {
        println!("Erro na execução: {}", e);
        process::exit(1);
    }

}

