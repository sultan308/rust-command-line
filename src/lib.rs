use std::error::Error;
use std::fs;
use std::env;
pub struct Request {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}
impl Request {
    pub fn build(args: &[String]) -> Result<Request, &'static str> {
        if args.len() < 3 {
            return Err("not enough args given! At least 3 is required.")
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        Ok(Request { query, file_path, ignore_case})
    }
}

pub fn handle(request: Request) -> Result<(),Box<dyn Error>>{
    let file_contents: String = fs::read_to_string(&request.file_path)?;
    println!("Successfully read the file:\n");
    for line_found in find_lines(&request.query, &file_contents){
        println!("{line_found}")
    }
    Ok(())
}

pub fn find_lines<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let mut lines_found:Vec<&str> = Vec::new();
    for line in content.lines(){
        if line.contains(query){
            lines_found.push(line);
        }
    }
    lines_found
}
pub fn case_insensitive_find_lines<'a>(query: &str, content: &'a str) -> Vec<&'a str>{
    let mut lines_found:Vec<&str> = Vec::new();
    for line in content.lines(){
        if line.to_lowercase().contains(&query.to_lowercase()){
            lines_found.push(line);
        }
    }
    lines_found
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn case_sensitive_find_lines_one_line(){
        let query: String = String::from("Or");

        let content = "\
Or the blue parakeet does not return
from the little grave in the fern garden
though one may wake in the morning
thinking mother's call is the bird.";

        assert_eq!(
            vec!["Or the blue parakeet does not return"],
            find_lines(&query, content)
        );
    }
    #[test]
    fn case_sensitive_find_lines_two_lines(){
        let query: String = String::from("in ");

        let content = "\
Or the blue parakeet does not return
from the little grave in the fern garden
though one may wake in the morning
In thinking mother's call is the bird.";

        assert_eq!(
            vec!["from the little grave in the fern garden",
            "though one may wake in the morning"],
            find_lines(&query, content)
        );
    }

    #[test]
    fn case_insensitive_find_lines_three_line(){
        let query: String = String::from("Or");

        let content = "\
Or the blue parakeet does not return
from the little grave in the fern garden
though one may wake in the morning
thinking mother's call is the bird.
oR test";

        assert_eq!(
            vec!["Or the blue parakeet does not return",
                 "though one may wake in the morning",
                 "oR test"],
            case_insensitive_find_lines(&query, content)
        );
    }
}