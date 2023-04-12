use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

const COMMENT_BEGIN: &str = "<!-- BEGIN mktoc -->";
const COMMENT_END: &str = "<!-- END mktoc -->";

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    #[serde(default = "default_min_depth")]
    min_depth: i32,
    #[serde(default = "default_max_depth")]
    max_depth: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self { min_depth: default_min_depth(), max_depth: default_max_depth() }
    }
}

fn default_min_depth() -> i32 {
    1
}
fn default_max_depth() -> i32 {
    6
}

/// reads a file into a mutable string
fn read_file<P>(file_path: P) -> Result<String, ::std::io::Error>
    where P: AsRef<Path>
{
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn text_to_url(text: &str) -> String {
    text.trim()
        .replace(" ", "-")
        .replace("(", "")
        .replace(")", "")
        .replace("`", "")
        .replace("´", "")
        .replace("'", "")
        .replace("\"", "")
        .replace("[", "")
        .replace("]", "")
        .replace("{", "")
        .replace("}", "")
        .replace("?", "")
        .replace("¿", "")
        .replace("!", "")
        .replace("¡", "")
        .replace(".", "")
        .replace(",", "")
        .replace("\\", "")
        .replace("/", "")
        .replace(":", "")
        .replace(";", "")
        .replace("§", "")
        .replace("$", "")
        .replace("%", "")
        .replace("&", "")
        .replace("=", "")
        .replace("^", "")
        .replace("°", "")
        .replace("#", "")
        .replace("+", "")
        .replace("*", "")
        .replace("<", "")
        .replace(">", "")
        .to_ascii_lowercase()
}

/// parses a string and extracts all headlines to build a table of contents
///
/// Uses a basic regex "((#{1,6}\s))((.*))" to parse headings out of the
pub fn generate_toc(original_content: String, min_depth: i32, max_depth: i32, start_comment: String) -> String {
    let mut already_found_code_open = false;
    let mut code_block_found = false;
    let mut new_toc = start_comment;
    let re = regex::Regex::new(r"((#{1,6}\s))((.*))").unwrap();
    for line in original_content.lines() {
        let line_s: String = line.chars().take(3).collect();
        if line_s == "```".to_owned() {
            code_block_found = true;
        }

        if !code_block_found && !already_found_code_open {
            if line.starts_with("#") {
                // Check if the regex matches, if it doesn't continue skip (continue) the loop.
                let caps = match re.captures(line) {
                    Some(matched) => matched,
                    None => { continue; }
                };
                
                let level: i32 = (caps.get(2).unwrap().as_str().chars().count() - 1) as i32;
                
                if level < min_depth {
                    continue;
                }

                if level > max_depth {
                    continue;
                }

                let text = caps.get(3).unwrap().as_str();
                let link = text_to_url(text);
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

fn parse_json_config_and_begin_comment<'t>(text: &'t str) -> (Config, String) {
    let mut start_comment = COMMENT_BEGIN.to_string();
    let re = Regex::new(r"<!--\s*BEGIN mktoc\s*(?P<json>\{.*\})\s*-->").unwrap();
    let json_str = match re.captures(&text) {
        Some(captures) => captures.name("json").unwrap().as_str(),
        None => { "" }
    };

    // if a json config is found it will be injected into the start comment.
    if !json_str.is_empty() {
        start_comment = format!("<!-- BEGIN mktoc {} -->", json_str);
    }

    // parse the JSON string as a Config struct
    let config: Config = match serde_json::from_str(json_str) {
        Ok(config) => config,
        Err(_e) => { Config::default() }
    };

    return (config, start_comment)
}

/// takes a file path as `String` and returns a table of contents for the file
pub fn make_toc<P>(
    file_path_in: P,
    min_depth: i32,
    max_depth: i32,
)
    -> Result<String, ::std::io::Error>
    where P: AsRef<Path>
{
    let content = read_file(file_path_in)?;
    // extract the JSON string from the comment
    let (config, start_comment) = parse_json_config_and_begin_comment(&content);

    // Read min and max depth values, config from the file itself takes 
    // priority over CLI args or environment values.
    let min_depth_value = match Some(config.min_depth) {
        Some(_value) => config.min_depth,
        None => min_depth
    };

    let max_depth_value = match Some(config.max_depth) {
        Some(_value) => config.max_depth,
        None => max_depth
    };
    
    let new_toc = generate_toc(content.to_owned(), min_depth_value, max_depth_value, start_comment);
    
    let re_toc =
        Regex::new(r"(?ms)^(<!--\s*BEGIN mktoc\s*(?P<json>\{.*\})\s*-->)(.*?)(<!-- END mktoc -->)").unwrap();
    let res: String = re_toc
        .replacen(content.as_str(), 1, new_toc.as_str())
        .into_owned();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_config_and_begin_comment() {
        struct TestCase<'a> {
            input: &'a str,
            expected: Config,
        }

        let tests = vec![TestCase{
            input: "<!-- BEGIN mktoc {\"min_depth\":3} -->",
            expected: Config{
                min_depth: 3,
                max_depth: 6
            }
        }];

        for test in tests {
            let (cnf, comment) = parse_json_config_and_begin_comment(&test.input);
            assert_eq!(cnf.max_depth, test.expected.max_depth);
            assert_eq!(cnf.min_depth, test.expected.min_depth);
            assert_eq!(comment, test.input.to_string())
        }

    }
}