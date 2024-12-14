use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path ) = parse_args(&args);

    println!("Searching for {query} in {file_path} ...\n...\n...");

    let file_contents: String = fs::read_to_string(file_path)
                                .expect("Failed to read the file.");

    println!("Successfully read the file:\n{file_contents}")



}

fn parse_args(args: &[String]) -> (&String, &String){
    let query: &String = &args[1];
    let file_path: &String = &args[2];

    (query, file_path)
}