use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli_request: Request = Request::new(&args);

    println!("Searching for {} in {} ...\n...\n...", &cli_request.query, &cli_request.file_path);

    let file_contents: String = fs::read_to_string(&cli_request.file_path)
                                .expect("Failed to read the file.");

    println!("Successfully read the file:\n{file_contents}")



}
struct Request {
    query: String,
    file_path: String
}
impl Request {
    fn new(args: &[String]) -> Request {
        if args.len() < 3 {
            panic!("not enough args given! At least 3 is required.")
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        Request { query, file_path}
    }
}