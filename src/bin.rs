use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mktoc")]
struct Cli {
    #[structopt()]
    file: String,

    #[structopt(long, short)]
    write: bool,
}

fn handle_write(new_toc: String) {
    let opts = Cli::from_args();
    if opts.write {
        let res_write = std::fs::write(opts.file, new_toc.as_bytes());
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
    let opts = Cli::from_args();
    let res = mktoc::make_toc(opts.file);

    match res {
        Ok(new_toc) => {
            handle_write(new_toc);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
