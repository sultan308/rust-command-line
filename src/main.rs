use std::env;
use std::process;

use minigrep::Request;
use minigrep::handle;
fn main() {
    let args = env::args();
    let cli_request: Request = Request::build(args).unwrap_or_else(|err|{
        eprintln!("Failed to parse args: {}",err);
        process::exit(1)
    });
    println!("Searching for {} in {} ...\n...\n...", &cli_request.query, &cli_request.file_path);
    if let Err(e) = handle(cli_request) {
        eprintln!("Encountered the following error while fulfilling your request:\n{}", e);
        process::exit(1)
    }
}
