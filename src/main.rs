use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli_request: Request = parse_request(&args);

    println!("Searching for {query} in {file_path} ...\n...\n...");

    let file_contents: String = fs::read_to_string(file_path)
                                .expect("Failed to read the file.");

    println!("Successfully read the file:\n{file_contents}")



}
struct Request {
    query: String,
    file_path: String
}
fn parse_request(args: &[String]) -> Request{
    let query: String = args[1].clone();
    let file_path: String = args[2].clone();
    Request{query, file_path}
}