use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cli_request: Request = Request::build(&args).unwrap_or_else(|err|{
        println!("Failed to parse args: {}",err);
        process::exit(1)
    });

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
    fn build(args: &[String]) -> Result<Request, &'static str> {
        if args.len() < 3 {
            return Err("not enough args given! At least 3 is required.")
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        Ok(Request { query, file_path})
    }
}