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

impl Config {
    // ensure_min_max sets the max_depth and min_depth values to 
    // their defaults in case they were configured wrong.
    fn ensure_min_max(&mut self) {
        if (self.max_depth > 6) || (self.max_depth < 1)  {
             self.max_depth = default_max_depth();
             eprintln!("WARNING: max_depthout of boound. Default value '{}' used.", self.max_depth);
        }
        
        if (self.min_depth < 1) || (self.min_depth > 6)  {
            self.min_depth = default_min_depth();
            eprintln!("WARNING: min_depthout of boound. Default value '{}' used.", self.min_depth);
        }
    }
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
    let mut config: Config = match serde_json::from_str(json_str) {
        Ok(config) => config,
        Err(_e) => { Config::default() }
    };

    // ensures the min_depth and max_depth are within scope
    config.ensure_min_max();

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
    fn test_text_to_url() {
        struct TestCase<'a> {
            name: &'a str,
            input: &'a str,
            expected: String,
        }

        let tests = vec![
            TestCase{
                name: "Case 01: My heading",
                input: "My heading",
                expected: String::from("my-heading")
            },TestCase{
                name: "Case 02: My Heading with `code`",
                input: "My Heading with `code`",
                expected: String::from("my-heading-with-code")
            },TestCase{
                name: "Case 03: Emoji 💃",
                input: "Emoji 💃",
                expected: String::from("emoji-💃")
            },TestCase{
                name: "Case 03: Number sign",
                input: "Why #5 is nice",
                expected: String::from("why-5-is-nice")
            },TestCase{
                name: "Case 04: Special signs",
                input: "Reasons to say !@;#\\!! out loud",
                expected: String::from("reasons-to-say-@-out-loud")
            },TestCase{
                name: "Case 05: Empty",
                input: "",
                expected: String::from("")
            }
        ];

        for test in tests {
            dbg!(test.name);
            assert_eq!(text_to_url(test.input), test.expected);
        }
    }

    #[test]
    fn test_parse_json_config_and_begin_comment() {
        struct TestCase<'a> {
            name: &'a str,
            input: &'a str,
            expected: Config,
        }

        let tests = vec![
        TestCase{
            name: "only min_depth set",
            input: "<!-- BEGIN mktoc {\"min_depth\":3} -->",
            expected: Config{
                min_depth: 3,
                max_depth: 6
            }
        },
        TestCase{
            name: "min_depth and max_depth set",
            input: "<!-- BEGIN mktoc {\"min_depth\":3, \"max_depth\":4} -->",
            expected: Config{
                min_depth: 3,
                max_depth: 4
            }
        },
        TestCase{
            name: "only max_depth set",
            input: "<!-- BEGIN mktoc {\"max_depth\":4} -->",
            expected: Config{
                min_depth: 1,
                max_depth: 4
            }
        },
        TestCase{
            name: "no json config, returns default values",
            input: "<!-- BEGIN mktoc -->",
            expected: Config{
                min_depth: 1,
                max_depth: 6
            }
        },
        TestCase{
            name: "invalid max_depth set results in default max_depth being used",
            input: "<!-- BEGIN mktoc {\"max_depth\":10} -->",
            expected: Config{
                min_depth: 1,
                max_depth: 6
            }
        },
        ];

        for test in tests {
            // logs the name of the test in case it fails.
            dbg!(test.name);
            let (cnf, comment) = parse_json_config_and_begin_comment(&test.input);
            assert_eq!(cnf.max_depth, test.expected.max_depth);
            assert_eq!(cnf.min_depth, test.expected.min_depth);
            assert_eq!(comment, test.input.to_string())
        }

    }
}