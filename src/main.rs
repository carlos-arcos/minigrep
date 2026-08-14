use std::env; //permite que el programa lea cualquier argumento de línea de comandos.
use std::fs; //permite que el programa lea archivos del sistema de archivos.

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let file_path: &String = &args[2];
    
    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("File contents:\n{contents}");
}
