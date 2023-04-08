mod app;
mod cli;
mod exit_code;

use app::App;
use exit_code::ExitCode;
use exit_code::ExitCode::GeneralError;

use cli::Opt;
use structopt::StructOpt;

fn run() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let mut app = App::new();

    app.enter_refs_view(&opt.path)
}

fn main() {
    let result = run();

    match result {
        Ok(exit_code) => {
            exit_code.exit();
        }
        Err(err) => {
            eprintln!("[tig error]: {:#}", err);
            GeneralError.exit();
        }
    }
}
