use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = "README.md")]
    file: String,

    /// If set will output to stdout instead of replacing content in file
    #[arg(long, short)]
    stdout: bool,

    /// Minimum heading level
    #[arg(long, short = 'm', default_value_t = 1, env = "MKTOC_MIN_DEPTH")]
    min_depth: i32,

    /// Maximum heading level
    #[arg(long, short = 'M', default_value_t = 6, env = "MKTOC_MAX_DEPTH")]
    max_depth: i32,
}

fn handle_write(args: Args, new_toc: String) {
    if !args.stdout {
        let res_write = std::fs::write(&args.file, new_toc.as_bytes());
        match res_write {
            Ok(_r) => {
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Failed to write file. Error kind: {:?}", e.kind());
                std::process::exit(1);
            }
        }
    } else {
        println!("{}", new_toc);
    }
}

fn main() {
    let args = Args::parse();
    let res = mktoc::make_toc(&args.file, args.min_depth, args.max_depth);

    match res {
        Ok(new_toc) => {
            handle_write(args, new_toc);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
