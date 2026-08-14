use std::env; //permite que el programa lea cualquier argumento de línea de comandos.
use std::process; //permite que el programa finalice con un código de salida.
use minigrep::Config; //importa la estructura Config desde el archivo lib.rs.

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


