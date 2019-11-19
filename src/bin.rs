use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mktoc")]
struct Cli {
    #[structopt()]
    file: String,

    #[structopt(long, short)]
    write: bool,
}

fn main() -> std::io::Result<()> {
    let opts = Cli::from_args();
    let file = &opts.file.to_owned();
    let res = mktoc::make_toc(file.to_string());
    if opts.write {
        std::fs::write(file, res.as_bytes())?;
    } else {
        println!("{}", res);
    }

    Ok(())
}
