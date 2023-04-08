use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tig")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    path: PathBuf,
}
