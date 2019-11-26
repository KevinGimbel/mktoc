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

fn read_file(file_path: String) -> Result<String, ::std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn generate_toc(original_content: String) -> String {
    // @TODO: This RegEx creates weird outputs if the headline contains additional markdown, like images
    let re = regex::Regex::new(r"((#{1,6}\s))((.*))").unwrap();
    let mut new_toc = String::from(COMMENT_BEGIN);
    // let caps = re.captures(content.as_str());
    for caps in re.captures_iter(original_content.as_str()) {
        let level: usize = caps.get(2).unwrap().as_str().chars().count() - 1;
        let text = caps.get(3).unwrap().as_str();
        // @TODO: Use real URL encoding
        let link = text.replace(" ", "-").to_ascii_lowercase();
        // let spaces = " ".repeat(level -1);
        let spaces = match level {
            3 => String::from("  "),
            4 => String::from("    "),
            5 => String::from("      "),
            6 => String::from("        "),
            _ => String::from(""),
        };

        new_toc = format!(
            "{old}\n{spaces}- [{text}](#{link})",
            old = new_toc,
            spaces = spaces,
            text = text,
            link = link
        );
    }

    new_toc = format!("{}\n{}", new_toc, COMMENT_END);

    new_toc
}

pub fn make_toc(file_path_in: String) -> Result<String, ::std::io::Error> {
    let content = read_file(file_path_in)?;
    let new_toc = generate_toc(content.to_owned());
    let re_toc = regex::Regex::new(r"(?ms)^(<!-- BEGIN mktoc).*(END mktoc -->)").unwrap();
    let res: String = re_toc
        .replace_all(content.as_str(), new_toc.as_str())
        .into_owned();

    Ok(res)
}
