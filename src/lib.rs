use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

const COMMENT_BEGIN: &str = "<!-- BEGIN mktoc -->";
const COMMENT_END: &str = "<!-- END mktoc -->";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "default_min_depth")]
    pub min_depth: i32,
    #[serde(default = "default_max_depth")]
    pub max_depth: i32,
    #[serde(default)]
    pub wrap_in_details: bool, 
    #[serde(default)]
    pub start_comment: String,
}

impl Config {
    // ensure_min_max sets the max_depth and min_depth values to 
    // their defaults in case they were configured wrong.
    fn ensure_min_max(&mut self) {
        if (self.max_depth > 6) || (self.max_depth < 1)  {
             self.max_depth = default_max_depth();
             eprintln!("WARNING: max_depth out of bound. Default value '{}' used.", self.max_depth);
        }
        
        if (self.min_depth < 1) || (self.min_depth > 6)  {
            self.min_depth = default_min_depth();
            eprintln!("WARNING: min_depth out of bound. Default value '{}' used.", self.min_depth);
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self { min_depth: default_min_depth(), max_depth: default_max_depth(), wrap_in_details: false, start_comment: COMMENT_BEGIN.to_string()}
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

fn strip_markdown_links(text: &str) -> String {
    let pattern =  r"(?P<before>[^\[]*)(\[(?P<text>[^\]]+)\]\((?P<link>[^)]+)\))(?P<after>[^\[]*)";
    let re_link = regex::Regex::new(pattern).unwrap();
    let txt = re_link.replace_all(text, "$before$text$after");

    txt.to_string()
}

fn text_to_url(text: &str) -> String {
    let txt = strip_markdown_links(text);
    txt.trim()
        .replace(' ', "-")
        .replace(['(', ')', '`', '´', '\'', '"', '[', ']', '{', '}', '?', '¿', '!', '¡', '.', ',', '\\', '/', ':', ';', '§', '$', '%', '&', '=', '^', '°', '#', '+', '*', '<', '>'], "")
        .to_ascii_lowercase()
}

/// parses a string and extracts all headlines to build a table of contents
///
/// Uses a basic regex "((#{1,6}\s))((.*))" to parse headings out of the
pub fn generate_toc(original_content: String, config: Config) -> String {
    let mut already_found_code_open = false;
    let mut code_block_found = false;
    let mut new_toc = String::from("");
    let re = regex::Regex::new(r"((#{1,6}\s))((.*))").unwrap();
    for line in original_content.lines() {
        let line_s: String = line.chars().take(3).collect();
        if line_s == *"```" {
            code_block_found = true;
        }

        if !code_block_found && !already_found_code_open && line.starts_with('#') {
                // Check if the regex matches, if it doesn't continue skip (continue) the loop.
                let caps = match re.captures(line) {
                    Some(matched) => matched,
                    None => { continue; }
                };
                
                let level: i32 = (caps.get(2).unwrap().as_str().chars().count() - 1) as i32;
                
                if level < config.min_depth {
                    continue;
                }

                if level > config.max_depth {
                    continue;
                }

                let text = strip_markdown_links(caps.get(3).unwrap().as_str());
                let link = text_to_url(text.as_str());
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

        if code_block_found && already_found_code_open {
            code_block_found = false;
            already_found_code_open = false;
        }

        if line.starts_with("```") {
            already_found_code_open = true;
        }
    }

    if config.wrap_in_details {
        let new_toc_content = cleanup_wrapped_toc(new_toc);
        new_toc = format!("{}\n<details><summary>Table of Contents</summary>\n{}\n\n</details>\n{}\n",config.start_comment, new_toc_content, COMMENT_END);
    } else {
        new_toc = format!("{}\n{}\n{}", config.start_comment, new_toc, COMMENT_END);
    }


    new_toc
}

fn cleanup_wrapped_toc(input: String) -> String {
    // starting with an indention of 4 GitHub will render code. So we strip away all lines
    // staring with 4 spaces.
    let re = Regex::new(r"(?m)^ {4}").unwrap();
    let new_content = re.replace_all(&input, "").to_string();

    new_content
}

fn parse_json_config(text: &str) -> (Config, bool) {
    let mut json_config_found = false;
    let mut start_comment = COMMENT_BEGIN.to_string();
    let re = Regex::new(r"<!--\s*BEGIN mktoc\s*(?P<json>\{.*\})\s*-->").unwrap();
    let json_str = match re.captures(text) {
        Some(captures) => captures.name("json").unwrap().as_str(),
        None => { "" }
    };

    // if a json config is found it will be injected into the start comment.
    if !json_str.is_empty() {
        start_comment = format!("<!-- BEGIN mktoc {} -->", json_str);
        json_config_found = true;
    }

    // parse the JSON string as a Config struct
    let mut config: Config = match serde_json::from_str(json_str) {
        Ok(config) => config,
        Err(_e) => { 
            json_config_found = false; 
            Config::default()
        }
    };

    // ensures the min_depth and max_depth are within scope
    config.ensure_min_max();
    // assign start comment
    config.start_comment = start_comment;
    
    (config, json_config_found)
}

fn parse_json_config_or_use_provided(content: &str, cnf: Config) -> Config {
    let (json_config, json_config_found) = parse_json_config(content);
    
    // if a json config was found it takes priority over any other config
    if json_config_found {
        return json_config;
    }
    
    cnf
}

/// takes a file path as `String` and returns a table of contents for the file
pub fn make_toc<P>(
    file_path_in: P,
    cnf: Config,
)
    -> Result<String, ::std::io::Error>
    where P: AsRef<Path>
{
    let content = read_file(file_path_in)?;
    let config = parse_json_config_or_use_provided(&content, cnf);
    
    // create new ToC
    let new_toc = generate_toc(content.to_owned(), config);

    // get the ToC position and replace it with the new ToC
    let re_toc =
        Regex::new(r"(?ms)^(<!-- BEGIN mktoc(.*?)-->)(.*?)(<!-- END mktoc -->)").unwrap();
    let res: String = re_toc
        .replace(content.as_str(), new_toc.as_str())
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
    fn test_parse_json_config() {
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
                    ..Default::default()
                }
            },
            TestCase{
                name: "min_depth and max_depth set",
                input: "<!-- BEGIN mktoc {\"min_depth\":3, \"max_depth\":4} -->",
                expected: Config{
                    min_depth: 3,
                    max_depth: 4,
                    ..Default::default()
                }
            },
            TestCase{
                name: "only max_depth set",
                input: "<!-- BEGIN mktoc {\"max_depth\":4} -->",
                expected: Config{
                    max_depth: 4,
                    ..Default::default()
                }
            },
            TestCase{
                name: "no json config, returns default values",
                input: "<!-- BEGIN mktoc -->",
                expected: Config::default()
            },
            TestCase{
                name: "invalid max_depth set results in default max_depth being used",
                input: "<!-- BEGIN mktoc {\"max_depth\":10} -->",
                expected: Config{
                    max_depth: 6,
                    ..Default::default()
                }
            },
        ];

        for test in tests {
            // logs the name of the test in case it fails.
            dbg!(test.name);
            let (cnf, _json_config_found) = parse_json_config(&test.input);
            assert_eq!(cnf.max_depth, test.expected.max_depth);
            assert_eq!(cnf.min_depth, test.expected.min_depth);
            assert_eq!(cnf.start_comment, test.input.to_string());
        }
    }

    #[test]
    fn test_strip_markdown_links() {
        struct TestCase {
            input: &'static str,
            expected: String,
        }
        let test_cases = [
            TestCase {
                input: "This is a [link](https://example.com) in a Markdown text.",
                expected: String::from("This is a link in a Markdown text."),
            },
            TestCase {
                input: "This is a [link](https://example.com) and [another one](https://example.org) in a Markdown text.",
                expected: String::from("This is a link and another one in a Markdown text."),
            },
            TestCase {
                input: "This is a text without any links.",
                expected: String::from("This is a text without any links."),
            },
            TestCase {
                input: "",
                expected: String::from(""),
            },
            TestCase {
                input: "This is a [link](https://example.com) with some text after it.",
                expected: String::from("This is a link with some text after it."),
            },
        ];
    
        for test_case in &test_cases {
            assert_eq!(strip_markdown_links(test_case.input), test_case.expected);
        }
    }
}