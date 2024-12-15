use std::error::Error;
use std::fs;

pub struct Request {
    pub query: String,
    pub file_path: String
}
impl Request {
    pub fn build(args: &[String]) -> Result<Request, &'static str> {
        if args.len() < 3 {
            return Err("not enough args given! At least 3 is required.")
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        Ok(Request { query, file_path})
    }
}

pub fn handle(request: Request) -> Result<(),Box<dyn Error>>{
    let file_contents: String = fs::read_to_string(&request.file_path)?;
    println!("Successfully read the file:\n{file_contents}");
    Ok(())
}