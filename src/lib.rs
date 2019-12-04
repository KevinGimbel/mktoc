use std::fs::File;
use std::io::prelude::*;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mktoc")]
struct Cli {
    #[structopt()]
    file: String,

    #[structopt(long, short)]
    write: bool,
}

const COMMENT_BEGIN: &str = "<!-- BEGIN mktoc -->";
const COMMENT_END: &str = "<!-- END mktoc -->";

/// reads a file into a mutable string
fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}
/// parses a string and extracts all headlines to build a table of contents
///
/// Uses basic regex "((#{1,6}\s))((.*))" to parse headings. Right now this produces errors because it also matches
/// comments in code blocks and if the headline contains images those will be matched, too.
pub fn generate_toc(original_content: String, min_depth: i32, max_depth: i32) -> String {
    let mut already_found_code_open = false;
    let mut code_block_found = false;
    let mut new_toc = String::from(COMMENT_BEGIN);
    let re = regex::Regex::new(r"((#{1,6}\s))((.*))").unwrap();
    for line in original_content.lines() {

        if line.starts_with("```") {
            code_block_found = true;
        }

        if !code_block_found && !already_found_code_open {
            if line.starts_with("#") {
                let caps = re.captures(line).unwrap();
                let level: i32 = (caps.get(2).unwrap().as_str().chars().count() - 1) as i32;
                if level < min_depth {
                    continue;
                }

                if level > max_depth {
                    continue;
                }


                let text = caps.get(3).unwrap().as_str();
                let link = text.replace(" ", "-").to_ascii_lowercase();
                let spaces = match level {
                    3 => String::from("  "),
                    4 => String::from("    "),
                    5 => String::from("      "),
                    6 => String::from("        "),
                    _ => String::from(""),
                };
                new_toc = format!(
                    "{old}\n{spaces}- [{text}](#{link})",
                    old = new_toc.as_str(),
                    spaces = spaces,
                    text = text,
                    link = link
                );
            }
        }

        if code_block_found && already_found_code_open {
            code_block_found = false;
            already_found_code_open = false;
        }

        if line.starts_with("```") {
            already_found_code_open = true;
        }

    }

    new_toc = format!("{}\n{}", new_toc, COMMENT_END);

    new_toc
}

/// takes a file path as `String` and returns a table of contents for the file
pub fn make_toc(file_path_in: String, min_depth: i32, max_depth: i32) -> Result<String, ::std::io::Error> {
    let content = read_file(file_path_in)?;
    let new_toc = generate_toc(content.to_owned(), min_depth, max_depth);
    let re_toc = regex::Regex::new(r"(?ms)^(<!-- BEGIN mktoc).*(END mktoc -->)").unwrap();
    let res: String = re_toc
        .replace_all(content.as_str(), new_toc.as_str())
        .into_owned();

    Ok(res)
}
