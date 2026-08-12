use std::env; //permite que el programa lea cualquier argumento de línea de comandos.

fn main() {
    let args: Vec<String> = env::args().collect();

    let query: &String = &args[1];
    let file_path: &String = &args[2];
    
    println!("Searching for {query}");
    println!("In file {file_path}");
}
